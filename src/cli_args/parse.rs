use core::fmt;
use std::fs;

use crate::config::{audit_config, AuditType};
use clap::{Parser, ValueEnum};
use serde::Deserialize;

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

    /// git repo for security audit
    #[arg(value_parser = validate_repo_url)]
    pub repo: String,

    /// Optional subfolder pointing to project root (if not root folder of git clone)
    #[arg(long)]
    pub subfolder: Option<String>,

    /// Optional folders pointing to where main source is located (if not src/), defaults to src
    #[arg(long, default_value = "src", value_delimiter = ',')]
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

    /// Builder type (foundry, hardhat, custom, auto)
    #[arg(long, default_value_t = AuditType::Code4rena)]
    pub audit_type: AuditType,

    /// Builder type (foundry, hardhat, custom, auto)
    #[arg(long, default_value_t = BuilderType::Auto)]
    pub builder: BuilderType,

    /// if want to use --via-ir flag with foundry
    #[arg(long, help = "Use forge --via-ir")]
    pub via_ir: bool,

    /// Force re-clone and rebuild even if a cached workspace exists
    #[arg(
        long,
        help = "Force re-clone and rebuild even if a cached workspace exists"
    )]
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

        if let Some(config_ymal) = &cli.config {
            let yaml_str = fs::read_to_string(config_ymal)?;
            let config_values: Cli = serde_yaml::from_str(&yaml_str)?;

            // override command line args with values from yaml
            config_cli = config_values;
        }

        Ok(config_cli)
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
                // Auto-detect inside Docker based on files
                format!(
                    "if [ -f foundry.toml ]; then {forge_build_cmd}; \
             elif [ -f hardhat.config.js ] || [ -f hardhat.config.ts ]; then \
             if [ -f yarn.lock ]; then [ -f .env.example ] && cp .env.example .env; yarn install && yarn hardhat compile; \
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
