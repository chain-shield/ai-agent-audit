/// Repository preparation and Docker-based building.
///
/// This module handles secure repository cloning in Docker containers,
/// auto-detection of build systems (Foundry/Hardhat), and file filtering
/// for smart contract analysis.
use anyhow::{Context, Result};
use glob::glob;
use ignore::gitignore::GitignoreBuilder;
use log::info;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::Command;
use std::{
    fs,
    fs::File,
    io::{self, BufRead},
    path::Path,
};
use walkdir::WalkDir;

use crate::cli_args::parse::Cli;
use crate::config::{AuditType, audit_config};
use crate::utils::check_folder_name::{
    contains_build_config, is_library_package_json, is_monorepo_config_file, is_root_config_file,
    is_script_file, is_test_file,
};
use crate::utils::file_security::validate_repo_url;
use crate::utils::remapping::parse_and_store_remappings;

/// Build flags for forge compilation
#[derive(Debug, Clone, Copy)]
pub enum BuildFlags {
    /// Standard forge build
    Standard,
    /// Forge build with --via-ir --build-info flags
    ViaIr,
}

#[derive(Debug, Clone, Default)]
pub struct PocConfig {
    pub instructions: String,
    pub test_folder: PathBuf,
    pub template: String,
}

/// Contains paths to the repository root and relevant files.
/// This struct organizes the paths to Solidity files and documentation
/// that will be processed for analysis.
#[derive(Debug, Clone)]
pub struct RepoPaths {
    pub github_url: String,
    /// uniquely indentifies this project by protocol name and commit hash
    pub project_id: String,
    /// Path to the repository root directory
    pub root: PathBuf,
    /// Paths to all Solidity (.sol) files in the repository
    pub sol_files: Vec<PathBuf>, // includes test and script files
    pub test_files: Vec<PathBuf>,
    pub script_files: Vec<PathBuf>,
    pub config_files: Vec<PathBuf>,     // NOT in sol_files
    pub lib_config_files: Vec<PathBuf>, // config files (package.json) in lib folder
    pub source_code_folders: Vec<PathBuf>,
    /// Paths to documentation files (README.md, etc.)
    pub docs: Vec<PathBuf>,
    /// e.g. `"my-cool-repo"`
    pub repo_name: String,
    /// audit scope file
    pub audit_scope: Option<PathBuf>,
    /// folder exclude from scope
    pub excluded_folders: Option<Vec<PathBuf>>,
    pub scoped_files: Option<PathBuf>,
    pub monorepo_folders: Option<PathBuf>,
    /// full 40-char SHA, e.g. `"1a2b3c4d5e6f7g8h9i0j1k2l3m4n5o6p7q8r9s0t"`
    pub commit_hash: String,
    /// audit type
    pub audit_type: AuditType,
    /// instructions, template, and folder location for PoCs
    pub poc: PocConfig,
}

/// Clones a repository and builds it in a secure Docker environment.
///
/// This function performs the complete repository preparation workflow:
/// 1. Creates a Docker volume for isolated analysis
/// 2. Clones the repository using Trail of Bits security toolbox
/// 3. Auto-detects and builds with Foundry or Hardhat
/// 4. Filters and organizes Solidity files and documentation
/// 5. Extracts commit hash for unique identification
///
/// # Arguments
/// * `url` - Git repository URL to clone and analyze
/// * `subfolder` - Optional subfolder name to analyze within the repository
/// * `build_flags` - Build flags for forge compilation
///
/// # Returns
/// * `RepoPaths` - Organized repository paths and metadata
///
/// # Security
/// All operations are performed in isolated Docker containers to prevent
/// malicious code execution on the host system.
pub fn clone_and_filter_git_repo(
    // url: &str,
    // subfolder: Option<&str>,
    // build_flags: BuildFlags,
    cli: &Cli,
) -> Result<RepoPaths> {
    let repo_url = cli.get_repo();

    // 🔐 Validate the repository URL for safety
    validate_repo_url(repo_url)?;

    // 1. Read HEAD and get the first 6 chars of the commit SHA
    let commit_hash = get_commit_hash(repo_url)?;

    // 2. Extract & sanitize the repo name
    let base_github_url = repo_url.trim_end_matches(".git");

    let mut repo_name = base_github_url
        .rsplit('/')
        .next()
        .unwrap_or("repo")
        .to_string();

    let github_url = format!("{}/blob/{}", base_github_url, commit_hash);

    // Append subfolder to repo_name if specified
    if let Some(sf) = &cli.subfolder {
        repo_name = format!("{}/{}", repo_name, sf);
    }
    info!("repo_name ==> {}", repo_name);

    // 5. git clone, install, and build in secure docker container
    // 6. define project id
    // returns dierctory where files are located
    let project_id = format!("{}-{}", repo_name.replace("/", "-"), &commit_hash[..6]);
    let root = clone_and_build_repo(cli, &repo_name, &project_id)?;

    // 6. Build .gitignore matcher
    let mut ign = GitignoreBuilder::new(&root);
    ign.add_line(None, "dist")?;
    ign.add_line(None, "out")?;
    let ign = ign.build()?;

    // Determine the search root - if subfolder is specified, search within that subdirectory
    let search_root = root.join(&repo_name);
    info!("search_root => {}", search_root.display());

    let source_code_folders = cli
        .code_folders
        .iter()
        .map(|f| search_root.join(f))
        .collect::<Vec<PathBuf>>();
    info!("source_code_folders => {:?}", source_code_folders);

    // Validate that the search root exists
    if !search_root.exists() {
        anyhow::bail!(
            "Specified path '{}' does not exist in the repository",
            repo_name
        );
    }

    // create excluded folders
    let excluded_folders = if let Some(folders) = &cli.exclude_folders {
        let folder_paths: Vec<PathBuf> = folders
            .iter()
            .map(|f| search_root.join(f).to_path_buf())
            .filter(|p| p.exists())
            .collect();
        Some(folder_paths)
    } else {
        None
    };

    // check if custom doc folder set it up
    let mut docs = Vec::new();

    let has_custom_docs = match &cli.custom_doc {
        Some(doc) => {
            let doc_path = Path::new(doc).to_path_buf();
            if doc_path.exists() {
                docs.push(doc_path);
                true
            } else {
                panic!("invalid custom docs");
            }
        }
        None => false,
    };

    let has_doc_folder = match &cli.doc_folder {
        Some(doc) => {
            let doc_path = search_root.join(doc).to_string_lossy().to_string();

            info!("doc path => {}", doc_path);

            for e in glob(&format!("{}/*.md", doc_path)).expect("invalid doc folder") {
                if let Ok(path) = e {
                    docs.push(path)
                }
            }

            true
        }
        None => false,
    };

    let monorepo_folders = match &cli.monorepo_folders {
        Some(repos) => Some(Path::new(repos).to_path_buf()),
        None => None,
    };

    // Initialize vectors to store file paths
    let mut sol_files = Vec::new();
    let mut test_files = Vec::new();
    let mut script_files = Vec::new();

    let mut config_files = Vec::new();
    let mut lib_config_files = Vec::new();

    for entry in WalkDir::new(&search_root)
        .into_iter()
        .filter_entry(|e| {
            let path = e.path();
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default()
                .to_ascii_lowercase();
            // Skip out/, cache/, and .git
            !(name == "out" || name == "cache" || name == ".git")
        })
        .filter_map(Result::ok)
    {
        let path = entry.path();

        if entry.file_type().is_dir() {
            continue;
        }
        // skip if gitignore or symlink
        if ign.matched(path, false).is_ignore()
            || fs::symlink_metadata(path)?.file_type().is_symlink()
        {
            continue;
        }

        // Parse remappings.txt and store in global cache
        if (path.file_name() == Some(OsStr::new("remapping.txt"))
            || path.file_name() == Some(OsStr::new("remappings.txt")))
            && path.parent() == Some(&search_root)
        {
            info!("Found remapping file: {}", path.display());
            match parse_and_store_remappings(path, &project_id) {
                Ok(count) => {
                    info!(
                        "Successfully parsed {} remappings from {}",
                        count,
                        path.display()
                    );
                }
                Err(e) => {
                    log::warn!("Failed to parse remappings from {}: {}", path.display(), e);
                }
            }
        }

        //only get md docs from root folder /*.md
        match path.extension().and_then(|e| e.to_str()) {
            Some("sol") => {
                if is_test_file(path, search_root.as_path()) {
                    test_files.push(path.to_path_buf());
                }
                if is_script_file(path) {
                    script_files.push(path.to_path_buf());
                }

                sol_files.push(path.to_path_buf())
            }
            Some("md")
                if path.parent().map_or(false, |p| p == search_root)
                    && !has_doc_folder
                    && !has_custom_docs =>
            {
                docs.push(path.to_path_buf())
            }
            Some(_) => {
                if is_library_package_json(path, search_root.as_path()) {
                    lib_config_files.push(path.to_path_buf())
                } else if is_root_config_file(path, search_root.as_path())
                    || is_monorepo_config_file(path, search_root.as_path(), &monorepo_folders)?
                {
                    config_files.push(path.to_path_buf())
                }
            }
            _ => {}
        }
    }

    let audit_scope = match &cli.audit_scope {
        Some(scope) => {
            let audit_scope_file = Path::new(scope).to_path_buf();

            if !audit_scope_file.exists() {
                panic!("invalid audit scope file md - does not exist");
            }

            Some(audit_scope_file)
        }
        None => None,
    };

    let scoped_files = match &cli.scoped_files {
        Some(scope) => {
            let scope_file = Path::new(scope).to_path_buf();

            if !scope_file.exists() {
                panic!("invalid scope files txt - does not exist");
            }

            Some(scope_file)
        }
        None => None,
    };

    let poc_instructions_content = match &cli.poc_instructions {
        Some(instructions_poc) => {
            let poc_instructions_file = Path::new(&instructions_poc).to_path_buf();
            if poc_instructions_file.exists() {
                fs::read_to_string(poc_instructions_file)?
            } else {
                panic!("poc instructions file does not exist!");
            }
        }
        None => String::new(),
    };
    info!("PoC instructions size: {}", poc_instructions_content.len());

    let poc_template_content = match &cli.poc_template {
        Some(template_poc) => {
            let poc_template_file = Path::new(&template_poc).to_path_buf();
            if poc_template_file.exists() {
                fs::read_to_string(poc_template_file)?
            } else {
                panic!("poc template file does not exist!");
            }
        }
        None => String::new(),
    };
    info!("PoC template size: {}", poc_template_content.len());

    let test_folder = match &cli.test_folder {
        Some(folder) => {
            let folder = search_root.join(folder);

            if !folder.exists() {
                panic!("invalid test folder - does not exist");
            } else if poc_instructions_content.is_empty() {
                panic!("valid test folder - however PoC instructions missing!");
            }
            folder
        }
        None => {
            let folder = search_root.join("test");

            if !folder.exists() {
                log::warn!("invalid test folder - does not exist: {}", folder.display());
            } else if poc_instructions_content.is_empty() {
                panic!("valid test folder - however PoC instructions missing!");
            }
            folder
        }
    };

    // Return the collected paths
    Ok(RepoPaths {
        github_url,
        project_id,
        root,
        sol_files,
        test_files,
        script_files,
        config_files,
        lib_config_files,
        source_code_folders,
        docs,
        poc: PocConfig {
            instructions: poc_instructions_content,
            template: poc_template_content,
            test_folder,
        },
        repo_name,
        audit_scope,
        audit_type: cli.audit_type.clone(),
        excluded_folders,
        scoped_files,
        monorepo_folders,
        commit_hash,
    })
}

/// Adds GitHub authentication token to URL if GITHUB_TOKEN env var is set.
/// This enables cloning private repositories.
fn add_github_auth(repo_url: &str) -> String {
    if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        if repo_url.starts_with("https://github.com/") {
            return repo_url.replace(
                "https://github.com/",
                &format!("https://{}@github.com/", token),
            );
        }
    }

    repo_url.to_string()
}

pub fn clone_and_build_repo(cli: &Cli, repo_name: &str, project_id: &str) -> Result<PathBuf> {
    let docker_volume = format!("{}/{}", audit_config().docker_volume, project_id);
    let docker_path = PathBuf::from(&docker_volume);

    // Derived paths
    let repo_root = repo_name.split('/').next().unwrap_or(repo_name);
    let workspace_root = docker_path.join(repo_root);
    // Test override: allow tests to provide a local workspace path to bypass Docker
    if let Ok(local_ws) = std::env::var("AIAUDIT_TEST_LOCAL_WORKSPACE") {
        log::info!("Using test local workspace override at {}", local_ws);
        return Ok(PathBuf::from(local_ws));
    }

    let build_stamp = docker_path.join(".chainshield_build_ok");

    // Check if build artifacts exist (foundry uses 'out', hardhat uses 'artifacts')
    let has_build_artifacts = workspace_root.join("out").exists()
        || workspace_root.join("artifacts").exists()
        || workspace_root.join("build").exists(); // Some projects use 'build'

    // Determine if we should reuse the existing workspace
    let should_reuse = docker_path.exists()
        && build_stamp.exists()
        && workspace_root.exists()
        && has_build_artifacts
        && !cli.force_rebuild;

    if should_reuse {
        log::info!(
            "✅ Reusing existing workspace (build artifacts found): {}",
            docker_path.display()
        );
        return Ok(PathBuf::from(docker_volume));
    }

    // Log the reason for rebuild
    if docker_path.exists() {
        let mut reasons = Vec::new();
        if cli.force_rebuild {
            reasons.push("--force-rebuild flag set");
        }
        if !build_stamp.exists() {
            reasons.push("build stamp missing");
        }
        if !has_build_artifacts {
            reasons.push("build artifacts (out/artifacts/build) not found");
        }

        log::warn!(
            "🔄 Rebuilding workspace at {} (reason: {})",
            docker_path.display(),
            reasons.join(", ")
        );

        fs::remove_dir_all(&docker_path).with_context(|| {
            format!(
                "Failed to remove existing docker volume {}",
                docker_path.display()
            )
        })?;
    } else {
        log::info!("📦 Creating new workspace at {}", docker_path.display());
    }
    fs::create_dir_all(&docker_path)?;

    // Shallow clone for speed and security
    log::info!("git cloning repo...");

    let build_command = cli.generate_build_command();
    let repo_url = add_github_auth(cli.get_repo());

    // Install build tools if using custom builder (needed for native node modules)
    let setup_build_tools = if matches!(cli.builder, crate::cli_args::parse::BuilderType::Custom) {
        "apt-get update -qq && apt-get install -y -qq build-essential python3 > /dev/null 2>&1 && "
    } else {
        ""
    };

    let clone_and_build_command = format!(
        "{setup_build_tools}\
     git clone --depth=1 {repo_url} {repo_root} && \
     cd {repo_name} && \
     git config --global url.\"https://github.com/\".insteadOf \"ssh://git@github.com/\" && \
     git config --global url.\"https://github.com/\".insteadOf \"git@github.com:\" && \
     git config --global url.\"https://\".insteadOf \"ssh://\" && \
     export PNPM_HOME=/workspace/.pnpm && \
     export PATH=$PNPM_HOME:$PATH && \
     {build_command}"
    );

    let status = Command::new("docker")
        .args([
            "run",
            "--rm",
            "--user",
            "root",
            "-v",
            &format!("{}:/workspace", docker_volume),
            "-w",
            "/workspace",
            "trailofbits/eth-security-toolbox:nightly",
            "bash",
            "-c",
            &clone_and_build_command,
        ])
        .status()
        .context("Failed to clone and build repository in Docker")?;

    if !status.success() {
        anyhow::bail!("Clone and Build failed in Docker");
    }

    // On success, stamp the workspace for reuse
    fs::write(
        &build_stamp,
        format!(
            "project_id={}\nrepo={}\ncommit={}\n",
            project_id,
            cli.get_repo(),
            &get_commit_hash(cli.get_repo())?[..6]
        ),
    )?;

    Ok(PathBuf::from(docker_volume))
}

fn get_commit_hash(repo_url: &str) -> Result<String> {
    let auth_url = add_github_auth(repo_url);

    let output = Command::new("git")
        .args(["ls-remote", &auth_url, "HEAD"])
        .output()
        .context("Failed to run git ls-remote")?;

    if !output.status.success() {
        anyhow::bail!(
            "git ls-remote failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let stdout = String::from_utf8(output.stdout)?;
    let commit_hash = stdout
        .split_whitespace()
        .next()
        .context("Unexpected ls-remote output format")?
        .to_string();

    Ok(commit_hash)
}

impl AsRef<RepoPaths> for RepoPaths {
    fn as_ref(&self) -> &RepoPaths {
        self
    }
}

impl RepoPaths {
    /// Generates a unique identifier for the repository using name and short commit hash.
    /// Used for creating unique vector database collections and cache keys.
    pub fn unique_repo_hash(&self) -> String {
        format!(
            "{}-{}",
            self.repo_name.replace("/", "-"),
            &self.commit_hash[..6]
        )
    }
    /// root folder of protocol that contains foundery.toml etc
    pub fn get_protocol_root(&self) -> PathBuf {
        self.root.join(&self.repo_name)
    }

    /// Determine which directories Slither should be executed against.
    ///
    /// Slither needs to be run at a directory that contains a build configuration
    /// (e.g., `hardhat.config.*` or `foundry.toml`). For monorepos, each package
    /// often has its own config and must be analyzed separately.
    ///
    /// Priority:
    /// 1. If `monorepo_folders` is set, use those explicit folders.
    /// 2. Otherwise, use `source_code_folders` entries that *also* contain a build config.
    /// 3. Otherwise, fall back to the protocol root if it contains a build config.
    /// 4. Otherwise, scan first-level subdirectories of the protocol root for build configs.
    pub fn slither_roots(&self) -> Result<Vec<PathBuf>> {
        // 1) explicit monorepo folders file
        let explicit = self.extract_monorepo_folders()?;
        if !explicit.is_empty() {
            return Ok(explicit);
        }

        let protocol_root = self.get_protocol_root();
        let mut roots: Vec<PathBuf> = Vec::new();

        // 2) treat code_folders/source_code_folders as potential package roots (monorepo-friendly)
        for folder in &self.source_code_folders {
            if folder.exists() && contains_build_config(&folder.clone()) {
                roots.push(folder.clone());
            }
        }

        // 3) single-project fallback: root config
        if roots.is_empty() && protocol_root.exists() && contains_build_config(&protocol_root) {
            roots.push(protocol_root.clone());
        }

        // 4) last resort: check immediate subdirectories (cheap; avoids deep walking)
        if roots.is_empty() && protocol_root.exists() {
            if let Ok(entries) = std::fs::read_dir(&protocol_root) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() && contains_build_config(&path) {
                        roots.push(path);
                    }
                }
            }
        }

        roots.sort();
        roots.dedup();
        Ok(roots)
    }

    /// Generic helper to safely read a file, skipping symlinks and empty files.
    fn read_file_content(&self, file: &Path) -> Result<Option<(String, String)>> {
        if fs::symlink_metadata(file)?.file_type().is_symlink() {
            return Ok(None);
        }

        let protocol_folder = self.root.join(&self.repo_name);

        let filename = if file.starts_with(&self.root) {
            file.strip_prefix(&protocol_folder)?
                .to_string_lossy()
                .to_string()
        } else {
            file.file_name() // Option<&OsStr>
                .and_then(|f| Some(f.to_string_lossy().to_string())) // Option<&str>
                .unwrap_or_else(|| file.to_string_lossy().to_string()) // fallback
        };

        let content = match fs::read_to_string(file) {
            Ok(c) => c,
            Err(e) => {
                log::warn!("Could not read {}: {}", file.display(), e);
                return Ok(None);
            }
        };

        if content.trim().is_empty() {
            return Ok(None);
        }

        Ok(Some((filename, content)))
    }

    pub fn extract_content_from_scope_file(&self) -> Result<String> {
        let Some(scope_file) = &self.audit_scope else {
            return Ok(String::new());
        };

        if let Some((filename, content)) = self.read_file_content(scope_file)? {
            info!("extracting audit scope from {}", filename);
            Ok(content)
        } else {
            Ok(String::new())
        }
    }

    pub fn extract_monorepo_folders(&self) -> Result<Vec<PathBuf>> {
        let Some(monorepos) = &self.monorepo_folders else {
            return Ok(Vec::new());
        };

        let search_root = self.root.join(&self.repo_name);
        extract_list_of_files(monorepos, &search_root)
    }

    pub fn extract_scoped_files(&self) -> Result<Vec<PathBuf>> {
        let Some(scoped_files) = &self.scoped_files else {
            return Ok(Vec::new());
        };

        let search_root = self.root.join(&self.repo_name);
        extract_list_of_files(scoped_files, &search_root)
    }

    pub fn extract_content_from_docs(&self) -> Result<String> {
        let mut docs = String::new();

        for doc in &self.docs {
            if let Some((filename, content)) = self.read_file_content(doc)? {
                // info!("extracting content from {} doc file", filename);
                docs.push_str(&format!("### {}\n\n{}\n\n", filename, content));
            }
        }

        Ok(docs)
    }

    pub fn extract_lib_config_headers(&self) -> Result<String> {
        let mut config_headers = String::new();

        for config_file in &self.lib_config_files {
            if let Some((file, content)) = self.read_file_content(config_file)? {
                let first_eight_lines = content.lines().take(8).collect::<Vec<_>>().join("\n");
                config_headers.push_str(&format!("### {}\n\n{}\n\n", file, first_eight_lines));
            }
        }

        Ok(config_headers)
    }

    pub fn extract_content_from_config_files(&self) -> Result<String> {
        let mut config_content = String::new();

        for config in &self.config_files {
            if let Some((file, content)) = self.read_file_content(config)? {
                config_content.push_str(&format!("### {}\n\n{}\n\n", file, content));
            }
        }

        Ok(config_content)
    }
}

// read a files that contains a list of files (with relative path) and return array with full
// path for each file
pub fn extract_list_of_files(files: &PathBuf, root_folder: &PathBuf) -> Result<Vec<PathBuf>> {
    let file = File::open(files)?;
    let reader = io::BufReader::new(file);

    let paths: Vec<PathBuf> = reader
        .lines()
        .filter_map(|line| line.ok()) // drop I/O errors
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty()) // skip blank lines
        .map(|p| {
            let path = p.strip_prefix("./").unwrap_or(&p);
            root_folder.join(path)
        }) // turn String into PathBuf
        .collect();

    Ok(paths)
}
