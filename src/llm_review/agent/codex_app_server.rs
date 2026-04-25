use crate::config::MAX_CODEX_TURNS_PER_SESSION;
use anyhow::{Context, Result, anyhow, bail};
use schemars::{JsonSchema, schema_for};
use serde_json::{Value, json};
use std::env;
use std::io::{BufRead, BufReader, Write};
#[cfg(unix)]
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::sync::{Condvar, Mutex, OnceLock};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const MAX_PROMPT_ATTEMPTS: usize = 10;
const INITIAL_RETRY_DELAY_MS: u64 = 1_000;
const RATE_LIMIT_FALLBACK_WAIT_SECS: u64 = 5 * 60 * 60;
const CODEX_BIN_ENV_VARS: &[&str] = &["AI_AGENT_AUDIT_CODEX_BIN", "CODEX_BIN"];
// Keep the default pool conservative: each pooled Codex app-server session is a
// separate long-lived subprocess, so large pools multiply memory usage fast when
// users run multiple audits in parallel.
const DEFAULT_CODEX_SESSION_POOL_SIZE: usize = 2;
const CODEX_SESSION_POOL_SIZE_ENV: &str = "AI_AGENT_AUDIT_CODEX_SESSION_POOL_SIZE";
const CODEX_REQUEST_TIMEOUT_SECS: u64 = 30;
const CODEX_TURN_EVENT_TIMEOUT_SECS: u64 = 10 * 60;
const CODEX_SHUTDOWN_POLL_ATTEMPTS: usize = 10;
const CODEX_SHUTDOWN_POLL_INTERVAL_MS: u64 = 50;
#[cfg(target_os = "macos")]
const PLATFORM_CODEX_FALLBACKS: &[&str] = &["/Applications/Codex.app/Contents/Resources/codex"];
#[cfg(not(target_os = "macos"))]
const PLATFORM_CODEX_FALLBACKS: &[&str] = &[];

static CODEX_SESSION_POOL: OnceLock<CodexSessionPool> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct CodexAgentConfig {
    pub model: String,
    pub preamble: String,
    pub context: Option<String>,
    pub reasoning_effort: Option<String>,
    pub service_tier: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatGptAccount {
    pub email: String,
    pub plan_type: String,
}

impl CodexAgentConfig {
    pub fn prompt_blocking(&self, prompt: &str) -> Result<String> {
        self.prompt_with_schema(prompt, None)
    }

    pub fn prompt_structured_blocking<T>(&self, prompt: &str) -> Result<String>
    where
        T: JsonSchema,
    {
        let mut schema = serde_json::to_value(schema_for!(T))
            .context("failed to serialize structured Codex output schema")?;
        sanitize_schema_for_codex(&mut schema);
        self.prompt_with_schema(prompt, Some(schema))
    }

    fn prompt_with_schema(&self, prompt: &str, output_schema: Option<Value>) -> Result<String> {
        let mut retry_delay_ms = INITIAL_RETRY_DELAY_MS;

        for attempt in 1..=MAX_PROMPT_ATTEMPTS {
            match self.prompt_once(prompt, output_schema.clone()) {
                Ok(response) => return Ok(response),
                Err(PromptFailure::RateLimited { message, reset_at }) => {
                    let wait_secs = compute_rate_limit_wait_secs(reset_at);
                    log::warn!(
                        "Codex rate limited (attempt {attempt}/{MAX_PROMPT_ATTEMPTS}): {message}. Waiting {}s before retrying.",
                        wait_secs
                    );
                    thread::sleep(Duration::from_secs(wait_secs));
                }
                Err(PromptFailure::Retryable(message)) => {
                    if attempt == MAX_PROMPT_ATTEMPTS {
                        bail!("Codex prompt failed after retries: {message}");
                    }
                    log::warn!(
                        "Codex prompt retryable failure (attempt {attempt}/{MAX_PROMPT_ATTEMPTS}): {message}. Waiting {}ms before retrying.",
                        retry_delay_ms
                    );
                    thread::sleep(Duration::from_millis(retry_delay_ms));
                    retry_delay_ms = (retry_delay_ms * 2).min(60_000);
                }
                Err(PromptFailure::Fatal(err)) => return Err(err),
            }
        }

        bail!("Codex prompt exhausted retries without success")
    }

    fn prompt_once(&self, prompt: &str, output_schema: Option<Value>) -> PromptAttemptResult {
        let mut session = codex_session_pool()
            .acquire()
            .map_err(PromptFailure::Fatal)?;

        if let Err(err) = session.session_mut().ensure_chatgpt_auth_cached() {
            session.mark_broken();
            return Err(PromptFailure::Fatal(err));
        }

        let result = session.session_mut().run_turn(self, prompt, output_schema);
        if result.is_err() {
            session.mark_broken();
        } else {
            session.mark_broken_if_not_running();
            session.mark_broken_if_turn_limit_reached();
        }
        result
    }
}

#[derive(Debug)]
enum PromptFailure {
    RateLimited {
        message: String,
        reset_at: Option<u64>,
    },
    Retryable(String),
    Fatal(anyhow::Error),
}

type PromptAttemptResult = std::result::Result<String, PromptFailure>;

pub fn ensure_chatgpt_auth() -> Result<()> {
    let mut session = codex_session_pool().acquire()?;
    session.session_mut().ensure_chatgpt_auth_cached()
}

pub fn cached_chatgpt_account() -> Result<Option<ChatGptAccount>> {
    let mut session = codex_session_pool().acquire()?;
    session.session_mut().cached_chatgpt_account()
}

fn codex_session_pool() -> &'static CodexSessionPool {
    CODEX_SESSION_POOL.get_or_init(|| {
        let max_sessions = resolve_codex_session_pool_size();
        log::info!(
            "Initializing Codex app-server session pool with capacity {}",
            max_sessions
        );
        CodexSessionPool::new(max_sessions)
    })
}

fn resolve_codex_session_pool_size() -> usize {
    env::var(CODEX_SESSION_POOL_SIZE_ENV)
        .ok()
        .and_then(|value| {
            let trimmed = value.trim();
            match trimmed.parse::<usize>() {
                Ok(parsed) if parsed > 0 => Some(parsed),
                _ => {
                    log::warn!(
                        "Ignoring {}='{}' because it is not a positive integer",
                        CODEX_SESSION_POOL_SIZE_ENV,
                        trimmed
                    );
                    None
                }
            }
        })
        .unwrap_or(DEFAULT_CODEX_SESSION_POOL_SIZE)
}

struct CodexSessionPool {
    state: Mutex<CodexSessionPoolState>,
    available: Condvar,
    max_sessions: usize,
}

struct CodexSessionPoolState {
    idle: Vec<CodexSession>,
    total_sessions: usize,
}

impl CodexSessionPool {
    fn new(max_sessions: usize) -> Self {
        Self {
            state: Mutex::new(CodexSessionPoolState {
                idle: Vec::new(),
                total_sessions: 0,
            }),
            available: Condvar::new(),
            max_sessions,
        }
    }

    fn acquire(&'static self) -> Result<PooledCodexSession> {
        loop {
            let mut state = self
                .state
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());

            while let Some(mut session) = state.idle.pop() {
                if session.is_running() {
                    return Ok(PooledCodexSession::new(self, session));
                }
                state.total_sessions = state.total_sessions.saturating_sub(1);
            }

            if state.total_sessions < self.max_sessions {
                state.total_sessions += 1;
                drop(state);

                match CodexSession::start_initialized() {
                    Ok(session) => return Ok(PooledCodexSession::new(self, session)),
                    Err(err) => {
                        self.release_failed_slot();
                        return Err(err);
                    }
                }
            }

            drop(
                self.available
                    .wait(state)
                    .unwrap_or_else(|poisoned| poisoned.into_inner()),
            );
        }
    }

    fn return_session(&self, session: CodexSession) {
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        state.idle.push(session);
        self.available.notify_one();
    }

    fn discard_session(&self) {
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        state.total_sessions = state.total_sessions.saturating_sub(1);
        self.available.notify_one();
    }

    fn release_failed_slot(&self) {
        self.discard_session();
    }
}

struct PooledCodexSession {
    pool: &'static CodexSessionPool,
    session: Option<CodexSession>,
    broken: bool,
}

impl PooledCodexSession {
    fn new(pool: &'static CodexSessionPool, session: CodexSession) -> Self {
        Self {
            pool,
            session: Some(session),
            broken: false,
        }
    }

    fn session_mut(&mut self) -> &mut CodexSession {
        self.session
            .as_mut()
            .expect("pooled Codex session should always be present while checked out")
    }

    fn mark_broken_if_not_running(&mut self) {
        if let Some(session) = self.session.as_mut()
            && !session.is_running()
        {
            self.broken = true;
        }
    }

    fn mark_broken_if_turn_limit_reached(&mut self) {
        if let Some(session) = self.session.as_ref()
            && session.should_recycle()
        {
            log::info!(
                "Recycling Codex app-server session after {} completed turns",
                session.completed_turns
            );
            self.broken = true;
        }
    }

    fn mark_broken(&mut self) {
        self.broken = true;
    }
}

impl Drop for PooledCodexSession {
    fn drop(&mut self) {
        let Some(mut session) = self.session.take() else {
            return;
        };

        if self.broken || !session.is_running() {
            self.pool.discard_session();
            return;
        }

        self.pool.return_session(session);
    }
}

struct CodexSession {
    child: Child,
    stdin: ChildStdin,
    message_rx: Receiver<Result<Value>>,
    next_id: u64,
    auth_verified: bool,
    completed_turns: usize,
}

impl CodexSession {
    fn start() -> Result<Self> {
        let codex_cli = resolve_codex_cli_path()?;
        let mut command = Command::new(&codex_cli);
        command
            .args(["app-server", "--listen", "stdio://"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());
        configure_codex_process_group(&mut command);

        let mut child = command.spawn().with_context(|| {
            format!(
                "failed to start Codex app-server using `{}`",
                codex_cli.display()
            )
        })?;

        let stdin = child
            .stdin
            .take()
            .context("failed to capture Codex app-server stdin")?;
        let stdout = child
            .stdout
            .take()
            .context("failed to capture Codex app-server stdout")?;
        let message_rx = spawn_codex_reader_thread(stdout);

        Ok(Self {
            child,
            stdin,
            message_rx,
            next_id: 1,
            auth_verified: false,
            completed_turns: 0,
        })
    }

    fn start_initialized() -> Result<Self> {
        let mut session = Self::start()?;
        session.initialize()?;
        Ok(session)
    }

    fn initialize(&mut self) -> Result<()> {
        let _ = self.request(
            "initialize",
            Some(json!({
                "clientInfo": {
                    "name": "ai-agent-audit",
                    "title": "AI Agent Audit",
                    "version": env!("CARGO_PKG_VERSION"),
                }
            })),
        )?;
        self.notify("initialized", Some(json!({})))?;
        Ok(())
    }

    fn ensure_chatgpt_auth_cached(&mut self) -> Result<()> {
        if self.auth_verified {
            return Ok(());
        }

        self.ensure_chatgpt_auth()?;
        self.auth_verified = true;
        Ok(())
    }

    fn ensure_chatgpt_auth(&mut self) -> Result<()> {
        let account = self.read_account(true)?;
        let account_type = account
            .as_ref()
            .and_then(|acct| acct.get("type"))
            .and_then(Value::as_str);

        if account_type == Some("chatgpt") {
            self.auth_verified = true;
            return Ok(());
        }

        if account_type == Some("apiKey") {
            let _ = self.request("account/logout", None)?;
        }

        let login = self.request(
            "account/login/start",
            Some(json!({ "type": "chatgptDeviceCode" })),
        )?;

        let login_id = login
            .get("loginId")
            .and_then(Value::as_str)
            .context("Codex login response missing loginId")?
            .to_string();
        let verification_url = login
            .get("verificationUrl")
            .and_then(Value::as_str)
            .context("Codex login response missing verificationUrl")?;
        let user_code = login
            .get("userCode")
            .and_then(Value::as_str)
            .context("Codex login response missing userCode")?;

        eprintln!("\nChatGPT sign-in required for OpenAI access.");
        eprintln!("Open: {}", verification_url);
        eprintln!("Code: {}\n", user_code);
        open_browser_best_effort(verification_url);

        loop {
            let message = self.read_message()?;
            let Some(method) = message.get("method").and_then(Value::as_str) else {
                continue;
            };

            if method != "account/login/completed" {
                continue;
            }

            let params = message.get("params").cloned().unwrap_or_else(|| json!({}));
            let completed_login_id = params.get("loginId").and_then(Value::as_str);
            if completed_login_id != Some(login_id.as_str()) {
                continue;
            }

            if params
                .get("success")
                .and_then(Value::as_bool)
                .unwrap_or(false)
            {
                break;
            }

            let error = params
                .get("error")
                .and_then(Value::as_str)
                .unwrap_or("unknown login failure");
            bail!("ChatGPT sign-in failed: {error}");
        }

        let account = self.read_account(true)?;
        let account_type = account
            .as_ref()
            .and_then(|acct| acct.get("type"))
            .and_then(Value::as_str);

        if account_type != Some("chatgpt") {
            bail!("Codex did not return a ChatGPT-authenticated account after login");
        }

        self.auth_verified = true;
        Ok(())
    }

    fn cached_chatgpt_account(&mut self) -> Result<Option<ChatGptAccount>> {
        let Some(account) = self.read_account(false)? else {
            return Ok(None);
        };

        if account.get("type").and_then(Value::as_str) != Some("chatgpt") {
            return Ok(None);
        }

        self.auth_verified = true;

        Ok(Some(ChatGptAccount {
            email: account
                .get("email")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string(),
            plan_type: account
                .get("planType")
                .and_then(Value::as_str)
                .unwrap_or("unknown")
                .to_string(),
        }))
    }

    fn run_turn(
        &mut self,
        config: &CodexAgentConfig,
        prompt: &str,
        output_schema: Option<Value>,
    ) -> PromptAttemptResult {
        let developer_instructions = format!(
            "{}\n\nDo not browse the web, run shell commands, inspect local files, or call tools. Answer only from the prompt content and return the requested final answer.",
            config.preamble
        );

        let thread_params = build_thread_start_params(config, &developer_instructions);

        let thread_result = self.request("thread/start", Some(thread_params));
        let thread_id = match thread_result {
            Ok(result) => result
                .get("thread")
                .and_then(|thread| thread.get("id"))
                .and_then(Value::as_str)
                .map(str::to_owned)
                .ok_or_else(|| anyhow!("Codex thread/start missing thread id")),
            Err(err) => Err(err),
        }
        .map_err(classify_error)?;

        let input_text = compose_input(prompt, config.context.as_deref());

        let mut turn_params = json!({
            "threadId": thread_id,
            "input": [
                {
                    "type": "text",
                    "text": input_text,
                }
            ],
            "effort": config.reasoning_effort,
            "approvalPolicy": "never",
        });

        if let Some(schema) = output_schema {
            turn_params["outputSchema"] = schema;
        }

        let turn_result = self.request("turn/start", Some(turn_params));
        let turn = turn_result.map_err(classify_error)?;
        let turn_id = turn
            .get("turn")
            .and_then(|turn| turn.get("id"))
            .and_then(Value::as_str)
            .map(str::to_owned)
            .ok_or_else(|| anyhow!("Codex turn/start missing turn id"))
            .map_err(PromptFailure::Fatal)?;

        let mut final_text: Option<String> = None;

        loop {
            let message = match self
                .read_message_with_timeout(Duration::from_secs(CODEX_TURN_EVENT_TIMEOUT_SECS))
            {
                Ok(message) => message,
                Err(err) => return Err(classify_error(err)),
            };

            let Some(method) = message.get("method").and_then(Value::as_str) else {
                continue;
            };

            match method {
                "item/completed" => {
                    let params = message.get("params").cloned().unwrap_or_else(|| json!({}));
                    let matches_turn =
                        params.get("turnId").and_then(Value::as_str) == Some(turn_id.as_str());
                    if !matches_turn {
                        continue;
                    }

                    let Some(item) = params.get("item") else {
                        continue;
                    };
                    if item.get("type").and_then(Value::as_str) != Some("agentMessage") {
                        continue;
                    }

                    let text = item
                        .get("text")
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .trim()
                        .to_string();
                    if text.is_empty() {
                        continue;
                    }

                    let phase = item.get("phase").and_then(Value::as_str);
                    if phase == Some("final_answer") || phase.is_none() {
                        final_text = Some(text);
                    }
                }
                "raw_response_item/completed" => {
                    let params = message.get("params").cloned().unwrap_or_else(|| json!({}));
                    let matches_turn =
                        params.get("turnId").and_then(Value::as_str) == Some(turn_id.as_str());
                    if !matches_turn {
                        continue;
                    }

                    let Some(item) = params.get("item") else {
                        continue;
                    };
                    if item.get("type").and_then(Value::as_str) != Some("message") {
                        continue;
                    }

                    let phase = item.get("phase").and_then(Value::as_str);
                    if phase != Some("final_answer") && phase.is_some() {
                        continue;
                    }

                    let content = item
                        .get("content")
                        .and_then(Value::as_array)
                        .cloned()
                        .unwrap_or_default();
                    let text = content
                        .iter()
                        .filter(|part| {
                            part.get("type").and_then(Value::as_str) == Some("output_text")
                        })
                        .filter_map(|part| part.get("text").and_then(Value::as_str))
                        .collect::<Vec<_>>()
                        .join("");

                    if !text.trim().is_empty() {
                        final_text = Some(text.trim().to_string());
                    }
                }
                "turn/completed" => {
                    let params = message.get("params").cloned().unwrap_or_else(|| json!({}));
                    let Some(turn) = params.get("turn") else {
                        continue;
                    };
                    if turn.get("id").and_then(Value::as_str) != Some(turn_id.as_str()) {
                        continue;
                    }

                    let status = turn.get("status").and_then(Value::as_str).unwrap_or("");
                    return match status {
                        "completed" | "interrupted" => {
                            let output = final_text
                                .filter(|text| !text.trim().is_empty())
                                .ok_or_else(|| {
                                    PromptFailure::Fatal(anyhow!(
                                        "Codex completed without final text output"
                                    ))
                                })?;
                            self.completed_turns = self.completed_turns.saturating_add(1);
                            Ok(output)
                        }
                        "failed" => {
                            let error = turn.get("error").cloned().unwrap_or_else(|| json!({}));
                            Err(classify_turn_error(error, self))
                        }
                        other => Err(PromptFailure::Retryable(format!(
                            "unexpected Codex turn status: {other}"
                        ))),
                    };
                }
                _ => {}
            }
        }
    }

    fn request(&mut self, method: &str, params: Option<Value>) -> Result<Value> {
        let id = self.next_id;
        self.next_id += 1;

        let mut message = json!({
            "id": id,
            "method": method,
        });

        if let Some(params) = params {
            message["params"] = params;
        }

        self.write_message(&message)?;

        loop {
            let response =
                self.read_message_with_timeout(Duration::from_secs(CODEX_REQUEST_TIMEOUT_SECS))?;
            let Some(response_id) = response.get("id").and_then(Value::as_u64) else {
                continue;
            };

            if response_id != id {
                continue;
            }

            if let Some(error) = response.get("error") {
                let message = error
                    .get("message")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown Codex JSON-RPC error");
                let code = error
                    .get("code")
                    .and_then(Value::as_i64)
                    .map(|code| format!(" (code {code})"))
                    .unwrap_or_default();
                bail!("Codex request `{method}` failed{code}: {message}");
            }

            return Ok(response.get("result").cloned().unwrap_or_else(|| json!({})));
        }
    }

    fn notify(&mut self, method: &str, params: Option<Value>) -> Result<()> {
        let mut message = json!({ "method": method });
        if let Some(params) = params {
            message["params"] = params;
        }
        self.write_message(&message)
    }

    fn write_message(&mut self, message: &Value) -> Result<()> {
        serde_json::to_writer(&mut self.stdin, message)
            .context("failed to write JSON-RPC request to Codex")?;
        self.stdin.write_all(b"\n")?;
        self.stdin.flush()?;
        Ok(())
    }

    fn read_message(&mut self) -> Result<Value> {
        recv_session_message(&self.message_rx, None)
    }

    fn read_message_with_timeout(&mut self, timeout: Duration) -> Result<Value> {
        recv_session_message(&self.message_rx, Some(timeout))
    }

    fn get_rate_limit_reset_at(&mut self) -> Option<u64> {
        let response = self.request("account/rateLimits/read", None).ok()?;
        extract_reset_at(&response)
    }

    fn read_account(&mut self, refresh_token: bool) -> Result<Option<Value>> {
        let account = self.request(
            "account/read",
            Some(json!({ "refreshToken": refresh_token })),
        )?;
        Ok(account.get("account").cloned())
    }

    fn is_running(&mut self) -> bool {
        self.child.try_wait().ok().flatten().is_none()
    }

    fn should_recycle(&self) -> bool {
        self.completed_turns >= MAX_CODEX_TURNS_PER_SESSION
    }
}

fn spawn_codex_reader_thread(stdout: ChildStdout) -> Receiver<Result<Value>> {
    let (message_tx, message_rx) = mpsc::channel::<Result<Value>>();

    thread::spawn(move || {
        let mut stdout = BufReader::new(stdout);
        let mut line = String::new();

        loop {
            line.clear();
            let bytes = stdout
                .read_line(&mut line)
                .context("failed to read Codex app-server output");

            let bytes = match bytes {
                Ok(bytes) => bytes,
                Err(err) => {
                    let _ = message_tx.send(Err(err));
                    return;
                }
            };

            if bytes == 0 {
                let _ = message_tx.send(Err(anyhow!(
                    "Codex app-server closed the stdio transport unexpectedly"
                )));
                return;
            }

            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            let parsed = serde_json::from_str(trimmed)
                .with_context(|| format!("failed to parse Codex JSON-RPC payload: {trimmed}"));
            if message_tx.send(parsed).is_err() {
                return;
            }
        }
    });

    message_rx
}

fn recv_session_message(
    message_rx: &Receiver<Result<Value>>,
    timeout: Option<Duration>,
) -> Result<Value> {
    match timeout {
        Some(timeout) => message_rx.recv_timeout(timeout).map_err(|err| match err {
            RecvTimeoutError::Timeout => anyhow!(
                "timed out waiting {}s for Codex app-server message",
                timeout.as_secs()
            ),
            RecvTimeoutError::Disconnected => {
                anyhow!("Codex app-server message channel closed unexpectedly")
            }
        })?,
        None => message_rx
            .recv()
            .map_err(|_| anyhow!("Codex app-server message channel closed unexpectedly"))?,
    }
}

fn resolve_codex_cli_path() -> Result<PathBuf> {
    for env_var in CODEX_BIN_ENV_VARS {
        if let Some(path) = env::var_os(env_var).map(PathBuf::from) {
            if is_runnable_file(&path) {
                return Ok(path);
            }

            bail!(
                "{} points to `{}`, but that file does not exist or is not executable",
                env_var,
                path.display()
            );
        }
    }

    if let Some(path) = find_codex_on_path() {
        return Ok(path);
    }

    for fallback in PLATFORM_CODEX_FALLBACKS {
        let path = PathBuf::from(fallback);
        if is_runnable_file(&path) {
            return Ok(path);
        }
    }

    let fallback_list = if PLATFORM_CODEX_FALLBACKS.is_empty() {
        "none".to_string()
    } else {
        PLATFORM_CODEX_FALLBACKS.join(", ")
    };

    bail!(
        "could not locate the Codex CLI binary; set AI_AGENT_AUDIT_CODEX_BIN to an absolute `codex` path, ensure `codex` is on PATH, or install Codex. Checked PATH and platform fallbacks: {fallback_list}"
    );
}

fn find_codex_on_path() -> Option<PathBuf> {
    let executable = if cfg!(windows) { "codex.exe" } else { "codex" };
    let path_env = env::var_os("PATH")?;

    env::split_paths(&path_env)
        .map(|dir| dir.join(executable))
        .find(|candidate| is_runnable_file(candidate))
}

fn is_runnable_file(path: &Path) -> bool {
    path.is_file()
}

fn configure_codex_process_group(command: &mut Command) {
    #[cfg(unix)]
    unsafe {
        // Give each pooled app-server its own process group so we can tear down
        // any MCP/browser helper descendants when the session is recycled.
        command.pre_exec(|| {
            if libc::setpgid(0, 0) != 0 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
}

fn terminate_codex_process_tree(child: &mut Child) {
    #[cfg(unix)]
    if let Some(process_group_id) = i32::try_from(child.id()).ok() {
        if process_group_is_alive(process_group_id) {
            send_process_group_signal(process_group_id, libc::SIGTERM);
            if !wait_for_process_group_exit(process_group_id, child) {
                send_process_group_signal(process_group_id, libc::SIGKILL);
                let _ = wait_for_process_group_exit(process_group_id, child);
            }
        }
    }

    if child.try_wait().ok().flatten().is_none() {
        let _ = child.kill();
    }
    let _ = child.wait();
}

#[cfg(unix)]
fn wait_for_process_group_exit(process_group_id: i32, child: &mut Child) -> bool {
    for _ in 0..CODEX_SHUTDOWN_POLL_ATTEMPTS {
        let _ = child.try_wait();
        if !process_group_is_alive(process_group_id) {
            return true;
        }
        thread::sleep(Duration::from_millis(CODEX_SHUTDOWN_POLL_INTERVAL_MS));
    }

    let _ = child.try_wait();
    !process_group_is_alive(process_group_id)
}

#[cfg(unix)]
fn process_group_is_alive(process_group_id: i32) -> bool {
    let result = unsafe { libc::killpg(process_group_id, 0) };
    if result == 0 {
        return true;
    }

    let err = std::io::Error::last_os_error();
    match err.raw_os_error() {
        Some(libc::ESRCH) => false,
        Some(libc::EPERM) => true,
        _ => {
            log::warn!(
                "Failed to probe Codex process group {}: {}",
                process_group_id,
                err
            );
            true
        }
    }
}

#[cfg(unix)]
fn send_process_group_signal(process_group_id: i32, signal: i32) {
    let result = unsafe { libc::killpg(process_group_id, signal) };
    if result == 0 {
        return;
    }

    let err = std::io::Error::last_os_error();
    if err.raw_os_error() != Some(libc::ESRCH) {
        log::warn!(
            "Failed to signal Codex process group {} with signal {}: {}",
            process_group_id,
            signal,
            err
        );
    }
}

impl Drop for CodexSession {
    fn drop(&mut self) {
        terminate_codex_process_tree(&mut self.child);
    }
}

fn compose_input(prompt: &str, context: Option<&str>) -> String {
    match context {
        Some(context) if !context.trim().is_empty() => {
            format!("## Context\n\n{context}\n\n## Task\n\n{prompt}")
        }
        _ => prompt.to_string(),
    }
}

fn build_thread_start_params(config: &CodexAgentConfig, developer_instructions: &str) -> Value {
    json!({
        "model": config.model,
        "ephemeral": true,
        "serviceName": "ai-agent-audit",
        "developerInstructions": developer_instructions,
        "serviceTier": config.service_tier,
    })
}

fn classify_error(err: anyhow::Error) -> PromptFailure {
    let message = err.to_string();
    if is_rate_limit_message(&message) {
        return PromptFailure::RateLimited {
            message,
            reset_at: None,
        };
    }

    if is_retryable_message(&message) {
        return PromptFailure::Retryable(message);
    }

    PromptFailure::Fatal(err)
}

fn classify_turn_error(error: Value, session: &mut CodexSession) -> PromptFailure {
    let message = error
        .get("message")
        .and_then(Value::as_str)
        .unwrap_or("Codex turn failed")
        .to_string();

    let codex_error = error
        .get("codexErrorInfo")
        .and_then(|value| {
            if value.is_string() {
                value.as_str().map(str::to_owned)
            } else if value.is_object() {
                serde_json::to_string(value).ok()
            } else {
                None
            }
        })
        .unwrap_or_default();

    if codex_error.contains("usageLimitExceeded") || is_rate_limit_message(&message) {
        return PromptFailure::RateLimited {
            message,
            reset_at: session.get_rate_limit_reset_at(),
        };
    }

    if is_auth_message(&message) || codex_error.to_ascii_lowercase().contains("auth") {
        session.auth_verified = false;
        return PromptFailure::Retryable(message);
    }

    if codex_error.contains("serverOverloaded")
        || codex_error.contains("internalServerError")
        || codex_error.contains("responseStreamDisconnected")
        || codex_error.contains("responseStreamConnectionFailed")
        || is_retryable_message(&message)
    {
        return PromptFailure::Retryable(message);
    }

    PromptFailure::Fatal(anyhow!("Codex turn failed: {message}"))
}

fn is_rate_limit_message(message: &str) -> bool {
    let message = message.to_ascii_lowercase();
    message.contains("rate limit")
        || message.contains("usage limit")
        || message.contains("credits depleted")
        || message.contains("too many requests")
}

fn is_retryable_message(message: &str) -> bool {
    let message = message.to_ascii_lowercase();
    message.contains("server overloaded")
        || message.contains("server busy")
        || message.contains("internal server error")
        || message.contains("retry later")
        || message.contains("temporarily unavailable")
        || message.contains("connection failed")
        || message.contains("stream disconnected")
        || message.contains("timed out")
}

fn is_auth_message(message: &str) -> bool {
    let message = message.to_ascii_lowercase();
    message.contains("unauthorized")
        || message.contains("authentication")
        || message.contains("login required")
        || message.contains("sign in")
        || message.contains("token expired")
        || message.contains("not authenticated")
}

fn extract_reset_at(response: &Value) -> Option<u64> {
    let mut reset_candidates = Vec::new();

    if let Some(snapshot) = response.get("rateLimits") {
        collect_snapshot_reset(snapshot, &mut reset_candidates);
    }

    if let Some(map) = response
        .get("rateLimitsByLimitId")
        .and_then(Value::as_object)
    {
        for snapshot in map.values() {
            collect_snapshot_reset(snapshot, &mut reset_candidates);
        }
    }

    reset_candidates.into_iter().max()
}

fn sanitize_schema_for_codex(schema: &mut Value) {
    match schema {
        Value::Object(map) => {
            map.remove("$schema");
            map.remove("default");

            if let Some(one_of) = map.remove("oneOf") {
                match map.get_mut("anyOf") {
                    Some(Value::Array(existing)) => {
                        if let Value::Array(mut values) = one_of {
                            existing.append(&mut values);
                        } else {
                            existing.push(one_of);
                        }
                    }
                    _ => {
                        map.insert("anyOf".to_string(), one_of);
                    }
                }
            }

            if let Some(ref_value) = map.get("$ref").cloned() {
                map.clear();
                map.insert("$ref".to_string(), ref_value);
                return;
            }

            let is_object_schema = map
                .get("type")
                .and_then(Value::as_str)
                .is_some_and(|ty| ty == "object")
                || map.contains_key("properties");

            if is_object_schema {
                if !map.contains_key("additionalProperties") {
                    map.insert("additionalProperties".to_string(), Value::Bool(false));
                }

                let existing_required = map
                    .get("required")
                    .and_then(Value::as_array)
                    .map(|required| {
                        required
                            .iter()
                            .filter_map(Value::as_str)
                            .map(str::to_owned)
                            .collect::<std::collections::HashSet<_>>()
                    })
                    .unwrap_or_default();

                if let Some(Value::Object(properties)) = map.get_mut("properties") {
                    let property_names = properties.keys().cloned().collect::<Vec<_>>();

                    for (property_name, property_schema) in properties.iter_mut() {
                        if !existing_required.contains(property_name) {
                            make_schema_nullable(property_schema);
                        }
                        sanitize_schema_for_codex(property_schema);
                    }

                    map.insert(
                        "required".to_string(),
                        Value::Array(property_names.into_iter().map(Value::String).collect()),
                    );
                } else if !map.contains_key("required") {
                    map.insert("required".to_string(), Value::Array(Vec::new()));
                }
            }

            for key in ["properties", "$defs", "definitions", "patternProperties"] {
                if let Some(Value::Object(children)) = map.get_mut(key) {
                    for child in children.values_mut() {
                        sanitize_schema_for_codex(child);
                    }
                }
            }

            for key in ["items", "additionalProperties", "contains"] {
                if let Some(value) = map.get_mut(key) {
                    sanitize_schema_for_codex(value);
                }
            }

            for key in ["anyOf", "allOf", "oneOf", "prefixItems"] {
                if let Some(Value::Array(values)) = map.get_mut(key) {
                    for value in values {
                        sanitize_schema_for_codex(value);
                    }
                }
            }
        }
        Value::Array(values) => {
            for value in values {
                sanitize_schema_for_codex(value);
            }
        }
        _ => {}
    }
}

fn make_schema_nullable(schema: &mut Value) {
    match schema {
        Value::Object(map) => {
            if let Some(value) = map.get_mut("type") {
                match value {
                    Value::String(ty) => {
                        if ty != "null" {
                            *value = Value::Array(vec![
                                Value::String(ty.clone()),
                                Value::String("null".to_string()),
                            ]);
                        }
                    }
                    Value::Array(types) => {
                        let has_null = types.iter().any(|ty| ty.as_str() == Some("null"));
                        if !has_null {
                            types.push(Value::String("null".to_string()));
                        }
                    }
                    _ => {}
                }
                return;
            }

            for union_key in ["anyOf", "oneOf"] {
                if let Some(Value::Array(variants)) = map.get_mut(union_key) {
                    let has_null = variants.iter().any(|variant| {
                        variant
                            .get("type")
                            .and_then(Value::as_str)
                            .is_some_and(|ty| ty == "null")
                    });
                    if !has_null {
                        variants.push(json!({ "type": "null" }));
                    }
                    return;
                }
            }
        }
        _ => {}
    }

    let original = schema.clone();
    *schema = json!({
        "anyOf": [
            original,
            { "type": "null" }
        ]
    });
}

fn collect_snapshot_reset(snapshot: &Value, reset_candidates: &mut Vec<u64>) {
    let primary = snapshot
        .get("primary")
        .cloned()
        .unwrap_or_else(|| json!({}));
    let secondary = snapshot
        .get("secondary")
        .cloned()
        .unwrap_or_else(|| json!({}));

    for window in [primary, secondary] {
        let used_percent = window
            .get("usedPercent")
            .and_then(Value::as_u64)
            .unwrap_or_default();
        let reset_at = window.get("resetsAt").and_then(Value::as_u64);
        if used_percent >= 100 {
            if let Some(reset_at) = reset_at {
                reset_candidates.push(reset_at);
            }
        }
    }

    if snapshot.get("rateLimitReachedType").is_some() {
        for field in ["primary", "secondary"] {
            if let Some(reset_at) = snapshot
                .get(field)
                .and_then(|window| window.get("resetsAt"))
                .and_then(Value::as_u64)
            {
                reset_candidates.push(reset_at);
            }
        }
    }
}

fn compute_rate_limit_wait_secs(reset_at: Option<u64>) -> u64 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    match reset_at {
        Some(reset_at) if reset_at > now => (reset_at - now) + 5,
        _ => RATE_LIMIT_FALLBACK_WAIT_SECS,
    }
}

fn open_browser_best_effort(url: &str) {
    #[cfg(target_os = "macos")]
    let command = ("open", vec![url]);
    #[cfg(target_os = "linux")]
    let command = ("xdg-open", vec![url]);
    #[cfg(target_os = "windows")]
    let command = ("cmd", vec!["/C", "start", url]);

    #[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
    {
        let _ = Command::new(command.0).args(command.1).spawn();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
    enum NestedRole {
        User,
        Admin,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
    struct NestedItem {
        id: Option<String>,
        name: String,
        #[schemars(description = "Actor role")]
        role: NestedRole,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
    struct NestedCollection {
        summary: Option<String>,
        items: Vec<NestedItem>,
    }

    #[test]
    fn test_sanitize_schema_for_codex_requires_optional_fields_and_marks_them_nullable() {
        let mut schema =
            serde_json::to_value(schema_for!(NestedCollection)).expect("schema should serialize");

        sanitize_schema_for_codex(&mut schema);

        let root = schema
            .as_object()
            .expect("sanitized schema should remain a JSON object");
        let root_required = root
            .get("required")
            .and_then(Value::as_array)
            .expect("root schema should define required fields");

        assert!(
            root_required
                .iter()
                .any(|value| value.as_str() == Some("summary")),
            "root optional fields should be promoted into required"
        );
        assert!(
            root_required
                .iter()
                .any(|value| value.as_str() == Some("items")),
            "root required should include every property"
        );

        let summary_schema = root
            .get("properties")
            .and_then(Value::as_object)
            .and_then(|properties| properties.get("summary"))
            .expect("root summary property should exist");
        assert!(
            schema_allows_null(summary_schema),
            "root optional field should become nullable"
        );

        let nested_item_schema = find_object_schema_with_property(&schema, "id")
            .expect("nested item object schema should be discoverable");
        let nested_required = nested_item_schema
            .get("required")
            .and_then(Value::as_array)
            .expect("nested object schema should define required fields");

        assert!(
            nested_required
                .iter()
                .any(|value| value.as_str() == Some("id")),
            "nested optional fields should be promoted into required"
        );
        assert!(
            nested_required
                .iter()
                .any(|value| value.as_str() == Some("name")),
            "nested required should include every property"
        );
        assert_eq!(
            nested_item_schema.get("additionalProperties"),
            Some(&Value::Bool(false)),
            "Codex object schemas should disallow extra properties"
        );

        let id_schema = nested_item_schema
            .get("properties")
            .and_then(Value::as_object)
            .and_then(|properties| properties.get("id"))
            .expect("nested id property should exist");
        assert!(
            schema_allows_null(id_schema),
            "nested optional field should become nullable"
        );
        let role_schema = nested_item_schema
            .get("properties")
            .and_then(Value::as_object)
            .and_then(|properties| properties.get("role"))
            .expect("nested role property should exist");
        assert!(
            !schema_contains_key(role_schema, "oneOf"),
            "enum property schemas should not retain oneOf"
        );
        assert!(
            !ref_schema_has_sibling_keywords(role_schema),
            "$ref schemas should not keep sibling keywords like description"
        );
        assert!(
            !schema_contains_key(&schema, "oneOf"),
            "sanitized Codex schemas should not contain oneOf"
        );
    }

    #[test]
    fn test_build_thread_start_params_keeps_each_turn_ephemeral() {
        let config = CodexAgentConfig {
            model: "gpt-5.4".to_string(),
            preamble: "Test".to_string(),
            context: None,
            reasoning_effort: Some("xhigh".to_string()),
            service_tier: None,
        };

        let params = build_thread_start_params(&config, "Developer instructions");

        assert_eq!(
            params.get("ephemeral").and_then(Value::as_bool),
            Some(true),
            "Codex turns should always use ephemeral threads so prior call context is cleared"
        );
        assert_eq!(
            params.get("serviceName").and_then(Value::as_str),
            Some("ai-agent-audit")
        );
    }

    #[test]
    fn test_recv_session_message_timeout_is_retryable() {
        let (_message_tx, message_rx) = mpsc::channel::<Result<Value>>();
        let err = recv_session_message(&message_rx, Some(Duration::from_millis(5)))
            .expect_err("missing messages should time out");

        assert!(
            err.to_string().contains("timed out waiting"),
            "timeout error should explain that the Codex stream went silent"
        );
        assert!(
            matches!(classify_error(err), PromptFailure::Retryable(_)),
            "silent-turn timeouts should be treated as retryable"
        );
    }

    fn find_object_schema_with_property<'a>(
        schema: &'a Value,
        property_name: &str,
    ) -> Option<&'a serde_json::Map<String, Value>> {
        match schema {
            Value::Object(map) => {
                if map
                    .get("properties")
                    .and_then(Value::as_object)
                    .is_some_and(|properties| properties.contains_key(property_name))
                {
                    return Some(map);
                }

                for value in map.values() {
                    if let Some(found) = find_object_schema_with_property(value, property_name) {
                        return Some(found);
                    }
                }

                None
            }
            Value::Array(values) => values
                .iter()
                .find_map(|value| find_object_schema_with_property(value, property_name)),
            _ => None,
        }
    }

    fn schema_allows_null(schema: &Value) -> bool {
        match schema {
            Value::Object(map) => {
                if map
                    .get("type")
                    .and_then(Value::as_str)
                    .is_some_and(|ty| ty == "null")
                {
                    return true;
                }

                if map
                    .get("type")
                    .and_then(Value::as_array)
                    .is_some_and(|types| types.iter().any(|ty| ty.as_str() == Some("null")))
                {
                    return true;
                }

                for key in ["anyOf", "oneOf"] {
                    if map
                        .get(key)
                        .and_then(Value::as_array)
                        .is_some_and(|variants| variants.iter().any(schema_allows_null))
                    {
                        return true;
                    }
                }

                false
            }
            _ => false,
        }
    }

    fn schema_contains_key(schema: &Value, key: &str) -> bool {
        match schema {
            Value::Object(map) => {
                if map.contains_key(key) {
                    return true;
                }
                map.values().any(|value| schema_contains_key(value, key))
            }
            Value::Array(values) => values.iter().any(|value| schema_contains_key(value, key)),
            _ => false,
        }
    }

    fn ref_schema_has_sibling_keywords(schema: &Value) -> bool {
        match schema {
            Value::Object(map) => {
                if map.contains_key("$ref") && map.len() > 1 {
                    return true;
                }
                map.values().any(ref_schema_has_sibling_keywords)
            }
            Value::Array(values) => values.iter().any(ref_schema_has_sibling_keywords),
            _ => false,
        }
    }
}
