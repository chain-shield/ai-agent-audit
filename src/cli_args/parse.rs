use core::fmt;

use crate::config::audit_config;
use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
#[clap(rename_all = "kebab-case")]
pub enum BuilderType {
    Foundry,
    Hardhat,
    Custom,
    Auto,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    /// git repo for security audit
    #[arg(value_parser = validate_repo_url)]
    pub repo: String,

    /// Optional subfolder pointing to code
    #[arg(long)]
    pub subfolder: Option<String>,

    /// Optional audit scope doc (markdown please)
    #[arg(long)]
    pub audit_scope: Option<String>,

    /// Optional doc folders, specify where to find documention if not in src/*.md
    #[arg(long)]
    pub doc_folder: Option<String>,

    /// Optional custom doc to replace docs in /src folder, should be md file in current directory
    #[arg(long)]
    pub custom_doc: Option<String>,

    // Optional exclude folders from scope
    #[arg(long, value_delimiter = ',')]
    pub exclude_folders: Option<Vec<String>>,

    /// Builder type (foundry, hardhat, custom, auto)
    #[arg(long, default_value_t = BuilderType::Auto)]
    pub builder: BuilderType,

    /// if want to use --via-ir flag with foundry
    #[arg(long, help = "Use forge --via-ir")]
    pub via_ir: bool,

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
    pub fn generate_build_command(&self) -> String {
        let base_forge = "forge install && forge build --build-info --skip test --skip script";

        let forge_build_cmd = if self.via_ir {
            format!("{base_forge} --via-ir")
        } else {
            base_forge.to_string()
        };

        match self.builder {
            BuilderType::Hardhat => {
                "npm install hardhat && npm install && npx hardhat compile".to_string()
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
             npm install hardhat && npm install && npx hardhat compile; \
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
            BuilderType::Custom => "custom",
            BuilderType::Auto => "auto",
        };
        f.write_str(s)
    }
}
