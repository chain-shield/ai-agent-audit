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

    /// Optional generated audit context configuration. YAML-only for now.
    #[arg(skip)]
    #[serde(default)]
    pub context: Option<ContextConfig>,
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
            if config_values.context.is_some() {
                config_cli.context = config_values.context;
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
        let copy_env = "[ -f .env.example ] && cp .env.example .env || true";
        let base_forge = "forge build --build-info --skip test --skip script";

        let forge_build_cmd = if self.via_ir {
            format!("{base_forge} --via-ir")
        } else {
            base_forge.to_string()
        };

        match self.builder {
            BuilderType::Hardhat => {
                format!("{copy_env}; npx --no-install hardhat compile")
            }
            BuilderType::HardhatYarn => {
                format!("{copy_env}; yarn hardhat compile")
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
             {copy_env}; \
             if [ -f yarn.lock ]; then yarn hardhat compile; \
             elif [ -f pnpm-lock.yaml ]; then pnpm hardhat compile; \
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
    fn generated_build_commands_do_not_install_packages() {
        let cli: Cli = serde_yaml::from_str(
            r#"
repo: "https://github.com/example/protocol.git"
builder: "Auto"
"#,
        )
        .unwrap();
        let command = cli.generate_build_command();
        for forbidden in [
            "forge install",
            "npm install",
            "yarn install",
            "pnpm install",
        ] {
            assert!(
                !command.contains(forbidden),
                "generated command unexpectedly contains `{forbidden}`: {command}"
            );
        }
        assert!(command.contains("forge build"));
        assert!(command.contains("npx --no-install hardhat compile"));

        let cli: Cli = serde_yaml::from_str(
            r#"
repo: "https://github.com/example/protocol.git"
builder: "HardhatYarn"
"#,
        )
        .unwrap();
        let command = cli.generate_build_command();
        assert!(!command.contains("yarn install"));
        assert!(command.contains("yarn hardhat compile"));
    }
}
