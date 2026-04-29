use core::fmt;
use std::fs;

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

#[derive(Debug, Clone, ValueEnum, Deserialize, strum_macros::EnumString)]
#[clap(rename_all = "kebab-case")]
pub enum BuilderType {
    Foundry,
    Hardhat,
    HardhatYarn,
    Custom,
    Auto,
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

    /// Optional instructions for writing PoC plus command to run PoC
    #[arg(long)]
    pub poc_instructions: Option<String>,

    /// Optional provide poc template (and inherited contracts)  for creating PoC tests
    #[arg(long)]
    pub poc_template: Option<String>,

    /// Optional provide test folder where poc template should be saved (relative path)
    #[arg(long)]
    pub test_folder: Option<String>,

    /// Audit type (Code4rena, Sherlock, Cantina, Client)
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
            if config_values.poc_instructions.is_some() {
                config_cli.poc_instructions = config_values.poc_instructions;
            }
            if config_values.poc_template.is_some() {
                config_cli.poc_template = config_values.poc_template;
            }
            if config_values.test_folder.is_some() {
                config_cli.test_folder = config_values.test_folder;
            }
            if config_values.build_cmd.is_some() {
                config_cli.build_cmd = config_values.build_cmd;
            }

            // Always use YAML values for these fields if present
            config_cli.audit_type = config_values.audit_type;
            config_cli.builder = config_values.builder;
            config_cli.via_ir = config_values.via_ir;
            config_cli.force_rebuild = config_values.force_rebuild;
        }

        // Validate that repo is provided either via CLI or YAML
        if config_cli.repo.is_none() {
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

    pub fn generate_build_command(&self) -> String {
        let base_forge = "forge install && forge build --build-info --skip test --skip script";

        let forge_build_cmd = if self.via_ir {
            format!("{base_forge} --via-ir")
        } else {
            base_forge.to_string()
        };

        match self.builder {
            BuilderType::Hardhat => {
                "[ -f .env.example ] && cp .env.example .env; npm install hardhat --legacy-peer-deps && npm install --legacy-peer-deps && npx hardhat compile".to_string()
            }
            BuilderType::HardhatYarn => {
                "[ -f .env.example ] && cp .env.example .env; yarn install && yarn hardhat compile".to_string()
            }
            BuilderType::Custom => self
                .build_cmd
                .clone()
                .expect("--build-cmd required with --builder custom"),
            BuilderType::Auto | BuilderType::Foundry => {
                // Auto-detect in the local workspace based on project files.
                format!(
                    "if [ -f foundry.toml ]; then {forge_build_cmd}; \
             elif [ -f hardhat.config.js ] || [ -f hardhat.config.ts ]; then \
             if [ -f yarn.lock ]; then [ -f .env.example ] && cp .env.example .env; yarn install && yarn hardhat compile; \
             elif [ -f pnpm-lock.yaml ]; then [ -f .env.example ] && cp .env.example .env; pnpm install && pnpm hardhat compile; \
             else [ -f .env.example ] && cp .env.example .env; npm install hardhat --legacy-peer-deps && npm install --legacy-peer-deps && npx hardhat compile; fi; \
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
