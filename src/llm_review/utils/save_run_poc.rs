use std::{
    fs,
    process::{Command, Stdio},
};

use anyhow::Result;
use regex::Regex;

use crate::{
    llm_review::phases::add_poc_findings::{PocStatus, PocTest},
    prepare_code::git_clone::RepoPaths,
    utils::runtime_deps::{RuntimeDependency, ensure_runtime_dependencies},
};

pub fn save_and_run_poc_test(poc_test: &mut PocTest, repo: &RepoPaths) -> Result<()> {
    // save file
    log::info!(
        "💾 Saving PoC test to: {}",
        poc_test.poc_test_file.display()
    );
    fs::write(
        poc_test.poc_test_file.clone(),
        poc_test.poc_test_code.clone(),
    )?;

    // run test
    log::info!(
        "🧪 Running PoC test with command: {}",
        poc_test.poc_test_command
    );

    let mut deps = vec![RuntimeDependency::Shell];
    if poc_test.poc_test_command.contains("forge") {
        deps.push(RuntimeDependency::Forge);
    }
    if poc_test.poc_test_command.contains("npm") {
        deps.extend([RuntimeDependency::Node, RuntimeDependency::Npm]);
    }
    if poc_test.poc_test_command.contains("yarn") {
        deps.extend([RuntimeDependency::Node, RuntimeDependency::Yarn]);
    }
    if poc_test.poc_test_command.contains("pnpm") {
        deps.extend([RuntimeDependency::Node, RuntimeDependency::Pnpm]);
    }
    ensure_runtime_dependencies("PoC test execution", &deps)?;

    let work_dir = repo.get_protocol_root();
    let out = Command::new("sh")
        .args(["-lc", &poc_test.poc_test_command])
        .current_dir(&work_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    let exit_code = out.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();

    // Log test output
    log::info!("📊 PoC test exit code: {}", exit_code);
    if !stdout.is_empty() {
        log::info!("📝 PoC test stdout:\n{}", stdout);
    }
    if !stderr.is_empty() {
        log::warn!("⚠️  PoC test stderr:\n{}", stderr);
    }

    // Always combine stdout and stderr for complete error context
    // This ensures the LLM sees compilation errors (stderr) along with test results (stdout)
    let parse_target = format!("{}\n{}", stdout, stderr);
    poc_test.poc_test_output = parse_target.clone();
    let parsed = parse_forge_result(&parse_target);

    // Classify status:
    // - If the test runner returned nonzero, prefer "test_error" unless we clearly see failing tests.
    // - If parsed says failed > 0 => failing_tests
    // - If parsed says 0/0 => treat as test_error (no tests found / pattern mismatch).
    // - Else passed.
    match parsed {
        Some((pass, fail, _)) => {
            if fail > 0 {
                poc_test.poc_test_status = PocStatus::FailingTests;
            } else if pass == 0 && fail == 0 {
                // Sometimes forge runs nothing; treat as error to force attention.
                poc_test.poc_test_status = PocStatus::ErrorRunningTests;
            } else if pass > 0 {
                poc_test.poc_test_status = PocStatus::AllTestPass;
            } else {
                poc_test.poc_test_status = PocStatus::ErrorRunningTests;
            }
        }
        None => {
            // Fallback heuristic: if "failed" appears and not "0 failed", call it failing_tests.
            let s = strip_ansi(&(stdout.clone() + "\n" + &stderr));
            let failed_word = Regex::new(r"(?i)\bfailed\b").unwrap().is_match(&s);
            let zero_failed = Regex::new(r"(?i)\b0\s+failed\b").unwrap().is_match(&s);
            let has_failing_tests = failed_word && !zero_failed;

            if has_failing_tests {
                poc_test.poc_test_status = PocStatus::FailingTests;
            } else {
                poc_test.poc_test_status = PocStatus::ErrorRunningTests;
            }
        }
    }
    Ok(())
}

/// Parse typical Foundry summaries like:
/// - "Test result: ok. 4 passed; 0 failed; 0 skipped"
/// - "Suite result: FAILED. 12 passed; 1 failed; 0 skipped"
/// - Also handles cases where "; 0 skipped" is omitted.
fn parse_forge_result(out: &str) -> Option<(u64, u64, Option<u64>)> {
    let s = strip_ansi(out);

    // Most common summary line (Test|Suite) ... X passed; Y failed; Z skipped
    // Make "skipped" optional.
    let re = Regex::new(
        r"(?mi)(?:Test|Suite)\s+result:\s*(?:ok|FAILED)[^\n]*?(\d+)\s+passed;[^\n]*?(\d+)\s+failed(?:;[^\n]*?(\d+)\s+skipped)?"
    ).ok()?;

    if let Some(caps) = re.captures(&s) {
        let passed = caps.get(1)?.as_str().parse().ok()?;
        let failed = caps.get(2)?.as_str().parse().ok()?;
        let skipped = caps.get(3).and_then(|m| m.as_str().parse().ok());
        return Some((passed, failed, skipped));
    }

    // Fallback #1: forge sometimes prints "X passed; Y failed" on a single line without the prefix
    let re2 = Regex::new(r"(?mi)(\d+)\s+passed;\s+(\d+)\s+failed(?:;\s+(\d+)\s+skipped)?").ok()?;
    if let Some(caps) = re2.captures(&s) {
        let passed = caps.get(1)?.as_str().parse().ok()?;
        let failed = caps.get(2)?.as_str().parse().ok()?;
        let skipped = caps.get(3).and_then(|m| m.as_str().parse().ok());
        return Some((passed, failed, skipped));
    }

    None
}

/// Strip ANSI escape codes to make regex parsing reliable.
fn strip_ansi(s: &str) -> String {
    // \x1b[ ... m
    let re = Regex::new(r"\x1B\[[0-9;]*m").unwrap();
    re.replace_all(s, "").into_owned()
}
