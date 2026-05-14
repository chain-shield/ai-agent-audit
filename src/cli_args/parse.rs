use core::fmt;
use std::{collections::BTreeMap, fs};

use crate::config::{AuditType, audit_config};
use clap::{Parser, ValueEnum};
use serde::Deserialize;

/// Default audit type for serde deserialization
fn default_audit_type() -> AuditType {
    AuditType::Code4rena
}

/// Default code folders for serde deserialization
fn default_code_folders() -> Vec<String> {
    vec!["src".to_string()]
}

/// Default builder type for serde deserialization
fn default_builder() -> BuilderType {
    BuilderType::Auto
}

/// Default via_ir flag for serde deserialization
fn default_via_ir() -> bool {
    false
}

/// Default force_rebuild flag for serde deserialization
fn default_force_rebuild() -> bool {
    false
}

fn default_context_files() -> Vec<String> {
    vec!["README.md".to_string()]
}

fn default_context_output_dir() -> String {
    "audit-docs".to_string()
}

fn default_force_regenerate() -> bool {
    true
}

fn default_context_token_limit() -> usize {
    5_000
}

fn default_poc_allow_fork() -> bool {
    true
}

fn default_poc_prefer_fork() -> bool {
    true
}

fn default_poc_rpc_env() -> BTreeMap<String, String> {
    BTreeMap::from([
        (
            "ethereum-mainnet".to_string(),
            "MAINNET_RPC_URL".to_string(),
        ),
        (
            "ethereum-sepolia".to_string(),
            "SEPOLIA_RPC_URL".to_string(),
        ),
        (
            "arbitrum-mainnet".to_string(),
            "ARBITRUM_RPC_URL".to_string(),
        ),
        (
            "arbitrum-sepolia".to_string(),
            "ARBITRUM_SEPOLIA_RPC_URL".to_string(),
        ),
        (
            "optimism-mainnet".to_string(),
            "OPTIMISM_RPC_URL".to_string(),
        ),
        (
            "optimism-sepolia".to_string(),
            "OPTIMISM_SEPOLIA_RPC_URL".to_string(),
        ),
        ("base-mainnet".to_string(), "BASE_RPC_URL".to_string()),
        (
            "base-sepolia".to_string(),
            "BASE_SEPOLIA_RPC_URL".to_string(),
        ),
        ("polygon-mainnet".to_string(), "POLYGON_RPC_URL".to_string()),
        (
            "polygon-amoy".to_string(),
            "POLYGON_AMOY_RPC_URL".to_string(),
        ),
        ("zksync-mainnet".to_string(), "ZKSYNC_RPC_URL".to_string()),
        (
            "zksync-sepolia".to_string(),
            "ZKSYNC_SEPOLIA_RPC_URL".to_string(),
        ),
        ("bnb-mainnet".to_string(), "BNB_RPC_URL".to_string()),
        ("bsc-mainnet".to_string(), "BSC_RPC_URL".to_string()),
        ("bnb-testnet".to_string(), "BNB_TESTNET_RPC_URL".to_string()),
        (
            "avalanche-mainnet".to_string(),
            "AVALANCHE_RPC_URL".to_string(),
        ),
        (
            "avalanche-fuji".to_string(),
            "AVALANCHE_FUJI_RPC_URL".to_string(),
        ),
        ("linea-mainnet".to_string(), "LINEA_RPC_URL".to_string()),
        (
            "linea-sepolia".to_string(),
            "LINEA_SEPOLIA_RPC_URL".to_string(),
        ),
        ("scroll-mainnet".to_string(), "SCROLL_RPC_URL".to_string()),
        (
            "scroll-sepolia".to_string(),
            "SCROLL_SEPOLIA_RPC_URL".to_string(),
        ),
        ("mantle-mainnet".to_string(), "MANTLE_RPC_URL".to_string()),
        (
            "mantle-sepolia".to_string(),
            "MANTLE_SEPOLIA_RPC_URL".to_string(),
        ),
        ("blast-mainnet".to_string(), "BLAST_RPC_URL".to_string()),
        (
            "blast-sepolia".to_string(),
            "BLAST_SEPOLIA_RPC_URL".to_string(),
        ),
        ("gnosis-mainnet".to_string(), "GNOSIS_RPC_URL".to_string()),
        (
            "gnosis-chiado".to_string(),
            "GNOSIS_CHIADO_RPC_URL".to_string(),
        ),
        ("celo-mainnet".to_string(), "CELO_RPC_URL".to_string()),
        (
            "celo-alfajores".to_string(),
            "CELO_ALFAJORES_RPC_URL".to_string(),
        ),
        (
            "unichain-mainnet".to_string(),
            "UNICHAIN_RPC_URL".to_string(),
        ),
        (
            "unichain-sepolia".to_string(),
            "UNICHAIN_SEPOLIA_RPC_URL".to_string(),
        ),
        ("sonic-mainnet".to_string(), "SONIC_RPC_URL".to_string()),
        (
            "sonic-testnet".to_string(),
            "SONIC_TESTNET_RPC_URL".to_string(),
        ),
        (
            "berachain-mainnet".to_string(),
            "BERACHAIN_RPC_URL".to_string(),
        ),
        (
            "berachain-bepolia".to_string(),
            "BERACHAIN_BEPOLIA_RPC_URL".to_string(),
        ),
        (
            "hyperliquid-mainnet".to_string(),
            "HYPERLIQUID_RPC_URL".to_string(),
        ),
        (
            "hyperliquid-testnet".to_string(),
            "HYPERLIQUID_TESTNET_RPC_URL".to_string(),
        ),
    ])
}

fn default_v12_source() -> V12Source {
    V12Source::Auto
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum V12Source {
    Auto,
    Disabled,
    Url(String),
}

impl Default for V12Source {
    fn default() -> Self {
        default_v12_source()
    }
}

impl<'de> serde::Deserialize<'de> for V12Source {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = V12Source;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(r#""auto", false, "disabled", or a V12 URL"#)
            }

            fn visit_bool<E>(self, value: bool) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(if value {
                    V12Source::Auto
                } else {
                    V12Source::Disabled
                })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let trimmed = value.trim();
                if trimmed.eq_ignore_ascii_case("auto") {
                    Ok(V12Source::Auto)
                } else if trimmed.eq_ignore_ascii_case("disabled")
                    || trimmed.eq_ignore_ascii_case("none")
                    || trimmed.eq_ignore_ascii_case("false")
                {
                    Ok(V12Source::Disabled)
                } else {
                    Ok(V12Source::Url(trimmed.to_string()))
                }
            }

            fn visit_string<E>(self, value: String) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_str(&value)
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContextConfig {
    #[serde(default = "default_context_files")]
    pub files: Vec<String>,
    #[serde(default)]
    pub urls: Vec<String>,
    #[serde(default = "default_v12_source")]
    pub v12_url: V12Source,
    #[serde(default = "default_context_output_dir")]
    pub output_dir: String,
    #[serde(default = "default_force_regenerate")]
    pub force_regenerate: bool,
    #[serde(default = "default_context_token_limit")]
    pub max_tokens_per_file: usize,
}

impl Default for ContextConfig {
    fn default() -> Self {
        Self {
            files: default_context_files(),
            urls: Vec::new(),
            v12_url: default_v12_source(),
            output_dir: default_context_output_dir(),
            force_regenerate: default_force_regenerate(),
            max_tokens_per_file: default_context_token_limit(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PocConfig {
    #[serde(default = "default_poc_allow_fork")]
    pub allow_fork: bool,
    #[serde(default = "default_poc_prefer_fork")]
    pub prefer_fork: bool,
    #[serde(default = "default_poc_rpc_env")]
    pub rpc_env: BTreeMap<String, String>,
}

impl Default for PocConfig {
    fn default() -> Self {
        Self {
            allow_fork: default_poc_allow_fork(),
            prefer_fork: default_poc_prefer_fork(),
            rpc_env: default_poc_rpc_env(),
        }
    }
}

impl PocConfig {
    pub fn rpc_env_var_for(&self, network: &str) -> Option<String> {
        self.rpc_env
            .get(network)
            .cloned()
            .or_else(|| default_poc_rpc_env().get(network).cloned())
    }
}

#[derive(Debug, Clone, Default)]
pub struct ResolvedRepoConfig {
    pub repo_url: String,
    pub branch: Option<String>,
    pub tree_paths: Vec<String>,
}

#[derive(Debug, Clone, ValueEnum, Deserialize, strum_macros::EnumString)]
#[clap(rename_all = "kebab-case")]
pub enum BuilderType {
    Foundry,
    Hardhat,
    HardhatYarn,
    Custom,
    Auto,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Deserialize)]
#[clap(rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum ValidationSupervisionMode {
    Off,
    Gui,
}

#[derive(Parser, Clone, Debug, Deserialize)]
#[command(author, version, about)]
pub struct Cli {
    /// Path to config file (optional)
    #[arg(long)]
    config: Option<String>,

    /// git repo for security audit (optional if provided in config file)
    #[arg(value_parser = validate_repo_url)]
    pub repo: Option<String>,

    /// Immunefi bug bounty URL used to derive repo, docs, and scope.
    #[arg(long)]
    #[serde(default)]
    pub immunefi_bounty: Option<String>,

    /// Code4rena bug bounty URL used to derive repo, docs, and scope.
    #[arg(long)]
    #[serde(default)]
    pub code4rena_bounty: Option<String>,

    /// Optional git branch/ref derived from external bounty metadata.
    #[arg(skip)]
    #[serde(default)]
    pub repo_branch: Option<String>,

    /// Optional tree paths derived from external bounty metadata.
    #[arg(skip)]
    #[serde(default)]
    pub repo_tree_paths: Vec<String>,

    /// Internal repo set derived from Immunefi metadata for polyrepo bounties.
    #[arg(skip)]
    #[serde(skip)]
    pub resolved_repos: Vec<ResolvedRepoConfig>,

    /// Optional subfolder pointing to project root (if not root folder of git clone)
    #[arg(long)]
    pub subfolder: Option<String>,

    /// Optional folders pointing to where main source is located (if not src/), defaults to src
    #[arg(long, default_value = "src", value_delimiter = ',')]
    #[serde(default = "default_code_folders")]
    pub code_folders: Vec<String>,

    /// Optional audit scope doc (markdown file please), should be in current directory
    #[arg(long)]
    pub audit_scope: Option<String>,

    /// Optional doc folders, specify where to find documention if not in src/*.md
    #[arg(long)]
    pub doc_folder: Option<String>,

    /// Optional add if monorepo, text file that contains all folder of monorepo packages (list of
    /// files)
    #[arg(long)]
    pub monorepo_folders: Option<String>,

    /// Optional custom doc to replace docs (md file) in /src folder, *should be md filename in current directory*
    #[arg(long)]
    pub custom_doc: Option<String>,

    // Optional exclude folders from scope
    #[arg(long, value_delimiter = ',')]
    pub exclude_folders: Option<Vec<String>>,

    /// Optional provide file with list of files (relate links) that are in scope - works well with
    /// code4rena
    #[arg(long)]
    pub scoped_files: Option<String>,

    /// Audit type (Code4rena, Code4renaBounty, ImmunefiBugBounty, Sherlock, Cantina, Client)
    #[arg(long, default_value_t = AuditType::Code4rena)]
    #[serde(default = "default_audit_type")]
    pub audit_type: AuditType,

    /// Builder type (foundry, hardhat, custom, auto)
    #[arg(long, default_value_t = BuilderType::Auto)]
    #[serde(default = "default_builder")]
    pub builder: BuilderType,

    /// if want to use --via-ir flag with foundry
    #[arg(long, help = "Use forge --via-ir")]
    #[serde(default = "default_via_ir")]
    pub via_ir: bool,

    /// Force re-clone and rebuild even if a cached workspace exists
    #[arg(
        long,
        help = "Force re-clone and rebuild even if a cached workspace exists"
    )]
    #[serde(default = "default_force_rebuild")]
    pub force_rebuild: bool,

    /// Custom build command for `custom` builder
    #[arg(long)]
    pub build_cmd: Option<String>,

    /// Control Codex GUI-supervised validation job emission after report export.
    /// Defaults to GUI unless explicitly set to off.
    #[arg(long, value_enum)]
    #[serde(default)]
    pub validation_supervision: Option<ValidationSupervisionMode>,

    /// Optional generated audit context configuration. YAML-only for now.
    #[arg(skip)]
    #[serde(default)]
    pub context: Option<ContextConfig>,

    /// Optional PoC runtime configuration. YAML-only for now.
    #[arg(skip)]
    #[serde(default)]
    pub poc: PocConfig,
}

fn validate_repo_url(s: &str) -> std::result::Result<String, String> {
    // Validate URL format and length before processing
    if s.len() > audit_config().max_repo_url_length {
        return Err(format!(
            "Repository URL is too long (max {} characters)",
            audit_config().max_repo_url_length
        ));
    }

    if !s.starts_with("https://") && !s.starts_with("http://") {
        return Err("Only HTTP/HTTPS repository URLs are supported".to_string());
    }
    Ok(s.to_string())
}

impl Cli {
    pub fn parse_args() -> anyhow::Result<Self> {
        let cli = Cli::parse();

        // Load yaml if present (--config config.yaml)
        let mut config_cli = cli.clone();

        if let Some(config_yaml) = &cli.config {
            let yaml_str = fs::read_to_string(config_yaml)?;
            let config_values: Cli = serde_yaml::from_str(&yaml_str)?;

            // Merge: CLI args override YAML values (except repo which comes from YAML if not provided)
            if cli.repo.is_none() && config_values.repo.is_some() {
                // Use repo from YAML
                config_cli.repo = config_values.repo.clone();
            }
            if config_cli.immunefi_bounty.is_none() && config_values.immunefi_bounty.is_some() {
                config_cli.immunefi_bounty = config_values.immunefi_bounty;
            }
            if config_cli.code4rena_bounty.is_none() && config_values.code4rena_bounty.is_some() {
                config_cli.code4rena_bounty = config_values.code4rena_bounty;
            }
            if config_values.repo_branch.is_some() {
                config_cli.repo_branch = config_values.repo_branch;
            }
            if !config_values.repo_tree_paths.is_empty() {
                config_cli.repo_tree_paths = config_values.repo_tree_paths;
            }

            // Override other fields from YAML if they were specified
            if config_values.subfolder.is_some() {
                config_cli.subfolder = config_values.subfolder;
            }
            if !config_values.code_folders.is_empty() && config_values.code_folders != vec!["src"] {
                config_cli.code_folders = config_values.code_folders;
            }
            if config_values.audit_scope.is_some() {
                config_cli.audit_scope = config_values.audit_scope;
            }
            if config_values.doc_folder.is_some() {
                config_cli.doc_folder = config_values.doc_folder;
            }
            if config_values.monorepo_folders.is_some() {
                config_cli.monorepo_folders = config_values.monorepo_folders;
            }
            if config_values.custom_doc.is_some() {
                config_cli.custom_doc = config_values.custom_doc;
            }
            if config_values.exclude_folders.is_some() {
                config_cli.exclude_folders = config_values.exclude_folders;
            }
            if config_values.scoped_files.is_some() {
                config_cli.scoped_files = config_values.scoped_files;
            }
            if config_values.build_cmd.is_some() {
                config_cli.build_cmd = config_values.build_cmd;
            }
            if config_cli.validation_supervision.is_none()
                && config_values.validation_supervision.is_some()
            {
                config_cli.validation_supervision = config_values.validation_supervision;
            }
            if config_values.context.is_some() {
                config_cli.context = config_values.context;
            }
            config_cli.poc = config_values.poc;

            // Always use YAML values for these fields if present
            config_cli.audit_type = config_values.audit_type;
            config_cli.builder = config_values.builder;
            config_cli.via_ir = config_values.via_ir;
            config_cli.force_rebuild = config_values.force_rebuild;
        }

        if matches!(config_cli.audit_type, AuditType::ImmunefiBugBounty)
            && config_cli.immunefi_bounty.is_none()
        {
            anyhow::bail!("immunefi_bounty must be provided when audit_type is ImmunefiBugBounty");
        }

        if matches!(config_cli.audit_type, AuditType::Code4renaBounty)
            && config_cli.repo.is_none()
            && config_cli.code4rena_bounty.is_none()
        {
            anyhow::bail!(
                "code4rena_bounty or repo must be provided when audit_type is Code4renaBounty"
            );
        }

        // Validate that repo is provided either via CLI/YAML or derivable from a supported bounty.
        if config_cli.repo.is_none()
            && !matches!(
                config_cli.audit_type,
                AuditType::ImmunefiBugBounty | AuditType::Code4renaBounty
            )
        {
            anyhow::bail!(
                "Repository URL must be provided either via CLI argument or in config file"
            );
        }

        Ok(config_cli)
    }

    /// Get the repository URL (guaranteed to be Some after parse_args validation)
    pub fn get_repo(&self) -> &str {
        self.repo
            .as_ref()
            .expect("repo should be validated in parse_args")
    }

    pub fn validation_supervision_mode(&self) -> ValidationSupervisionMode {
        self.validation_supervision
            .or_else(|| {
                std::env::var("AI_AGENT_AUDIT_VALIDATION_SUPERVISION")
                    .ok()
                    .and_then(|value| match value.trim().to_ascii_lowercase().as_str() {
                        "gui" => Some(ValidationSupervisionMode::Gui),
                        "off" | "" => Some(ValidationSupervisionMode::Off),
                        _ => None,
                    })
            })
            .unwrap_or(ValidationSupervisionMode::Gui)
    }

    pub fn generate_build_command(&self) -> String {
        let copy_env = "[ -f .env.example ] && cp .env.example .env || true";
        let base_forge = "forge build --build-info --skip test --skip script";
        let npm_install = "[ -d node_modules ] || if [ -f package-lock.json ] || [ -f npm-shrinkwrap.json ]; then npm ci --ignore-scripts; else npm install --ignore-scripts; fi";
        let yarn_install = "[ -d node_modules ] || (yarn install --frozen-lockfile --ignore-scripts || yarn install --immutable --mode=skip-builds)";
        let pnpm_install = "[ -d node_modules ] || pnpm install --frozen-lockfile --ignore-scripts";
        let bun_install = "[ -d node_modules ] || bun install --frozen-lockfile --ignore-scripts";

        let forge_build_cmd = if self.via_ir {
            format!("{base_forge} --via-ir")
        } else {
            base_forge.to_string()
        };

        match self.builder {
            BuilderType::Hardhat => {
                format!("{copy_env}; {npm_install}; npx --no-install hardhat compile")
            }
            BuilderType::HardhatYarn => {
                format!("{copy_env}; {yarn_install}; yarn hardhat compile")
            }
            BuilderType::Custom => self
                .build_cmd
                .clone()
                .expect("--build-cmd required with --builder custom"),
            BuilderType::Auto | BuilderType::Foundry => {
                // Auto-detect in the local workspace based on project files.
                format!(
                    "if [ -f foundry.toml ]; then {forge_build_cmd}; \
             elif [ -f hardhat.config.js ] || [ -f hardhat.config.ts ] || [ -f hardhat.config.cjs ]; then \
             {copy_env}; \
             if [ -f bun.lock ] || [ -f bun.lockb ]; then {bun_install}; bun run hardhat compile; \
             elif [ -f yarn.lock ]; then {yarn_install}; yarn hardhat compile; \
             elif [ -f pnpm-lock.yaml ]; then {pnpm_install}; pnpm hardhat compile; \
             elif [ -f package.json ]; then {npm_install}; npx --no-install hardhat compile; \
             else npx --no-install hardhat compile; fi; \
             else echo 'No build system detected'; exit 1; fi"
                )
            }
        }
    }
}

impl fmt::Display for BuilderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // keep the same kebab‑case you exposed on the CLI
        let s = match self {
            BuilderType::Foundry => "foundry",
            BuilderType::Hardhat => "hardhat",
            BuilderType::HardhatYarn => "hardhat-yarn",
            BuilderType::Custom => "custom",
            BuilderType::Auto => "auto",
        };
        f.write_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_config_defaults_regenerate_with_5000_token_limit() {
        let config = ContextConfig::default();
        assert_eq!(config.files, vec!["README.md"]);
        assert_eq!(config.output_dir, "audit-docs");
        assert!(config.force_regenerate);
        assert_eq!(config.max_tokens_per_file, 5_000);
        assert_eq!(config.v12_url, V12Source::Auto);
    }

    #[test]
    fn v12_source_deserializes_common_forms() {
        assert_eq!(
            serde_yaml::from_str::<V12Source>("auto").unwrap(),
            V12Source::Auto
        );
        assert_eq!(
            serde_yaml::from_str::<V12Source>("false").unwrap(),
            V12Source::Disabled
        );
        assert_eq!(
            serde_yaml::from_str::<V12Source>("https://v12.sh/runs/1/public").unwrap(),
            V12Source::Url("https://v12.sh/runs/1/public".to_string())
        );
    }

    #[test]
    fn generated_build_commands_install_js_deps_without_lifecycle_scripts() {
        let cli: Cli = serde_yaml::from_str(
            r#"
repo: "https://github.com/example/protocol.git"
builder: "Auto"
"#,
        )
        .unwrap();
        let command = cli.generate_build_command();
        assert!(!command.contains("forge install"));
        assert!(command.contains("forge build"));
        assert!(command.contains("bun install --frozen-lockfile --ignore-scripts"));
        assert!(command.contains("yarn install --frozen-lockfile --ignore-scripts"));
        assert!(command.contains("pnpm install --frozen-lockfile --ignore-scripts"));
        assert!(command.contains("npm ci --ignore-scripts"));
        assert!(command.contains("npm install --ignore-scripts"));
        assert!(command.contains("bun run hardhat compile"));

        let cli: Cli = serde_yaml::from_str(
            r#"
repo: "https://github.com/example/protocol.git"
builder: "HardhatYarn"
"#,
        )
        .unwrap();
        let command = cli.generate_build_command();
        assert!(command.contains("yarn install --frozen-lockfile --ignore-scripts"));
        assert!(command.contains("yarn hardhat compile"));
    }

    #[test]
    fn immunefi_bounty_config_can_derive_repo_later() {
        let cli: Cli = serde_yaml::from_str(
            r#"
audit_type: "ImmunefiBugBounty"
immunefi_bounty: "https://immunefi.com/bug-bounty/ssvnetwork/information/"
"#,
        )
        .unwrap();

        assert_eq!(cli.audit_type, AuditType::ImmunefiBugBounty);
        assert!(cli.repo.is_none());
        assert_eq!(
            cli.immunefi_bounty.as_deref(),
            Some("https://immunefi.com/bug-bounty/ssvnetwork/information/")
        );
    }

    #[test]
    fn immunefi_bounty_field_is_accepted() {
        let cli: Cli = serde_yaml::from_str(
            r#"
audit_type: "ImmunefiBugBounty"
immunefi_bounty: "https://immunefi.com/bug-bounty/ssvnetwork/information/"
"#,
        )
        .unwrap();

        assert_eq!(
            cli.immunefi_bounty.as_deref(),
            Some("https://immunefi.com/bug-bounty/ssvnetwork/information/")
        );
    }

    #[test]
    fn code4rena_bounty_config_can_derive_repo_later() {
        let cli: Cli = serde_yaml::from_str(
            r#"
audit_type: "Code4renaBounty"
code4rena_bounty: "https://code4rena.com/bounties/moonwell"
"#,
        )
        .unwrap();

        assert_eq!(cli.audit_type, AuditType::Code4renaBounty);
        assert!(cli.repo.is_none());
        assert_eq!(
            cli.code4rena_bounty.as_deref(),
            Some("https://code4rena.com/bounties/moonwell")
        );
    }

    #[test]
    fn poc_config_accepts_rpc_env_overrides_with_default_fallbacks() {
        let cli: Cli = serde_yaml::from_str(
            r#"
repo: "https://github.com/example/protocol.git"
poc:
  rpc_env:
    ethereum-mainnet: "CUSTOM_MAINNET_RPC_URL"
"#,
        )
        .unwrap();

        assert!(cli.poc.allow_fork);
        assert!(cli.poc.prefer_fork);
        assert_eq!(
            cli.poc.rpc_env_var_for("ethereum-mainnet").as_deref(),
            Some("CUSTOM_MAINNET_RPC_URL")
        );
        assert_eq!(
            cli.poc.rpc_env_var_for("base-mainnet").as_deref(),
            Some("BASE_RPC_URL")
        );
    }
}
