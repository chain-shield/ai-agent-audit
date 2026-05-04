/// Repository preparation and native building.
///
/// This module handles repository cloning, auto-detection of build systems
/// (Foundry/Hardhat), and file filtering for smart contract analysis.
use anyhow::{Context, Result};
use glob::glob;
use ignore::gitignore::GitignoreBuilder;
use log::{info, warn};
use regex::Regex;
use std::collections::BTreeSet;
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

use crate::cli_args::parse::{BuilderType, Cli, ResolvedRepoConfig};
use crate::config::{AuditType, audit_config};
use crate::prepare_code::audit_context::{
    GeneratedAuditContext, generate_audit_context, should_generate_context,
};
use crate::prepare_code::immunefi::{ResolvedGitCodebase, fetch_immunefi_bounty, same_github_repo};
use crate::utils::check_folder_name::{
    contains_build_config, is_library_package_json, is_monorepo_config_file, is_root_config_file,
    is_script_file, is_test_file,
};
use crate::utils::file_security::validate_repo_url;
use crate::utils::remapping::parse_and_store_remappings;
use crate::utils::runtime_deps::{RuntimeDependency, ensure_runtime_dependencies};

/// Build flags for forge compilation
#[derive(Debug, Clone, Copy)]
pub enum BuildFlags {
    /// Standard forge build
    Standard,
    /// Forge build with --via-ir --build-info flags
    ViaIr,
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
}

/// Clones a repository and builds it in the configured local workspace.
///
/// This function performs the complete repository preparation workflow:
/// 1. Creates/reuses a local workspace under `~/Desktop/Audit` by default
/// 2. Clones the repository
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
pub async fn clone_and_filter_git_repo(
    // url: &str,
    // subfolder: Option<&str>,
    // build_flags: BuildFlags,
    cli: &Cli,
) -> Result<RepoPaths> {
    let mut effective_cli = cli.clone();
    if matches!(effective_cli.audit_type, AuditType::ImmunefiBugBounty) {
        configure_immunefi_bounty_cli(&mut effective_cli).await?;
    }
    let cli = &effective_cli;
    if matches!(cli.audit_type, AuditType::ImmunefiBugBounty) && cli.resolved_repos.len() > 1 {
        return clone_and_filter_immunefi_polyrepo(cli).await;
    }
    let repo_url = cli.get_repo();

    // 🔐 Validate the repository URL for safety
    validate_repo_url(repo_url)?;

    // 1. Read HEAD and get the first 6 chars of the commit SHA
    let commit_hash = get_commit_hash(repo_url, cli.repo_branch.as_deref())?;

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

    // 5. git clone, install, and build in the local workspace
    // 6. define project id
    // returns dierctory where files are located
    let project_id = format!("{}-{}", repo_name.replace("/", "-"), &commit_hash[..6]);
    let root = clone_and_build_repo(cli, &repo_name, &project_id)?;

    // Determine the search root - if subfolder is specified, search within that subdirectory
    let search_root = root.join(&repo_name);
    info!("search_root => {}", search_root.display());

    // Validate that the search root exists
    if !search_root.exists() {
        anyhow::bail!(
            "Specified path '{}' does not exist in the repository",
            repo_name
        );
    }

    collect_repo_paths_after_clone(
        cli,
        root,
        search_root,
        repo_name,
        project_id,
        github_url,
        commit_hash,
    )
    .await
}

async fn collect_repo_paths_after_clone(
    cli: &Cli,
    root: PathBuf,
    search_root: PathBuf,
    repo_name: String,
    project_id: String,
    github_url: String,
    commit_hash: String,
) -> Result<RepoPaths> {
    let mut ign = GitignoreBuilder::new(&root);
    ign.add_line(None, "dist")?;
    ign.add_line(None, "out")?;
    let ign = ign.build()?;

    let generated_context = if should_generate_context(cli) {
        Some(generate_audit_context(cli, &root, &search_root, &repo_name, &commit_hash).await?)
    } else {
        None
    };

    if let Some(context) = &generated_context {
        info!(
            "generated audit context: scope_txt={}, scope_md={}, docs_md={}",
            context.scope_txt.display(),
            context.scope_md.display(),
            context.docs_md.display()
        );
    }

    let effective_code_folders =
        infer_effective_code_folders(cli, &search_root, generated_context.as_ref())?;
    let source_code_folders = effective_code_folders
        .iter()
        .map(|f| search_root.join(f))
        .collect::<Vec<PathBuf>>();
    info!("source_code_folders => {:?}", source_code_folders);

    let effective_custom_doc = generated_context
        .as_ref()
        .map(|context| context.docs_md.to_string_lossy().to_string())
        .or_else(|| cli.custom_doc.clone());
    let effective_audit_scope = generated_context
        .as_ref()
        .map(|context| context.scope_md.to_string_lossy().to_string())
        .or_else(|| cli.audit_scope.clone());
    let effective_scoped_files = generated_context
        .as_ref()
        .map(|context| context.scope_txt.to_string_lossy().to_string())
        .or_else(|| cli.scoped_files.clone());

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

    let mut docs = Vec::new();

    let has_custom_docs = match &effective_custom_doc {
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

            for path in glob(&format!("{}/*.md", doc_path))
                .expect("invalid doc folder")
                .flatten()
            {
                docs.push(path)
            }

            true
        }
        None => false,
    };

    if let Some(context) = &generated_context {
        for doc in &context.extra_docs {
            if doc.exists() {
                docs.push(doc.clone());
            }
        }
    }

    let monorepo_folders = infer_effective_monorepo_folders(
        cli,
        &search_root,
        generated_context.as_ref(),
        &repo_name,
    )?;

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
            !(name == "out" || name == "cache" || name == ".git")
        })
        .filter_map(Result::ok)
    {
        let path = entry.path();

        if entry.file_type().is_dir() {
            continue;
        }
        if ign.matched(path, false).is_ignore()
            || fs::symlink_metadata(path)?.file_type().is_symlink()
        {
            continue;
        }

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
                if path.parent().is_some_and(|p| p == search_root)
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

    let audit_scope = match &effective_audit_scope {
        Some(scope) => {
            let audit_scope_file = Path::new(scope).to_path_buf();

            if !audit_scope_file.exists() {
                panic!("invalid audit scope file md - does not exist");
            }

            Some(audit_scope_file)
        }
        None => None,
    };

    let scoped_files = match &effective_scoped_files {
        Some(scope) => {
            let scope_file = Path::new(scope).to_path_buf();

            if !scope_file.exists() {
                panic!("invalid scope files txt - does not exist");
            }

            Some(scope_file)
        }
        None => None,
    };

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
        repo_name,
        audit_scope,
        audit_type: cli.audit_type.clone(),
        excluded_folders,
        scoped_files,
        monorepo_folders,
        commit_hash,
    })
}

async fn clone_and_filter_immunefi_polyrepo(cli: &Cli) -> Result<RepoPaths> {
    ensure_runtime_dependencies(
        "polyrepo cloning",
        &[RuntimeDependency::Git, RuntimeDependency::Shell],
    )?;
    let slug = cli
        .immunefi_bounty
        .as_deref()
        .and_then(immunefi_slug_from_url)
        .unwrap_or("immunefi-polyrepo")
        .to_string();
    let commits = cli
        .resolved_repos
        .iter()
        .map(|repo| get_commit_hash(&repo.repo_url, repo.branch.as_deref()))
        .collect::<Result<Vec<_>>>()?;
    let fingerprint = polyrepo_workspace_fingerprint(&cli.resolved_repos, &commits);
    let commit_hash = format!("{}-{}", fingerprint, commits.join("-"));
    let project_id = format!("{}-{}", sanitize_artifact_name(&slug), fingerprint);
    let root = clone_and_build_polyrepo_workspace(cli, &project_id, &commits)?;
    let search_root = root.clone();
    let github_url = cli
        .resolved_repos
        .iter()
        .map(|repo| {
            let commit = commits
                .get(
                    cli.resolved_repos
                        .iter()
                        .position(|candidate| candidate.repo_url == repo.repo_url)
                        .unwrap_or(0),
                )
                .map(|hash| hash.as_str())
                .unwrap_or("HEAD");
            format!("{}/blob/{}", repo.repo_url.trim_end_matches(".git"), commit)
        })
        .collect::<Vec<_>>()
        .join(", ");

    collect_repo_paths_after_clone(
        cli,
        root,
        search_root,
        slug,
        project_id,
        github_url,
        commit_hash,
    )
    .await
}

/// Adds GitHub authentication token to URL if GITHUB_TOKEN env var is set.
/// This enables cloning private repositories.
fn add_github_auth(repo_url: &str) -> String {
    if let Ok(token) = std::env::var("GITHUB_TOKEN")
        && repo_url.starts_with("https://github.com/")
    {
        return repo_url.replace(
            "https://github.com/",
            &format!("https://{}@github.com/", token),
        );
    }

    repo_url.to_string()
}

async fn configure_immunefi_bounty_cli(cli: &mut Cli) -> Result<()> {
    let bounty_url = cli
        .immunefi_bounty
        .as_deref()
        .context("immunefi_bounty is required for AuditType::ImmunefiBugBounty")?;
    let bounty = fetch_immunefi_bounty(bounty_url)
        .await
        .with_context(|| format!("Failed to fetch Immunefi bounty metadata from {bounty_url}"))?;
    let mut codebases = match bounty.resolved_git_codebases() {
        Ok(codebases) => codebases,
        Err(err) if cli.repo.is_some() => {
            warn!(
                "Could not derive a unique Immunefi smart-contract repo for `{}`; using manually configured repo. Derivation error: {err:#}",
                bounty.project
            );
            Vec::new()
        }
        Err(err) => return Err(err),
    };
    for codebase in &mut codebases {
        resolve_github_tree_refs(codebase)?;
    }

    if let Some(configured_repo) = cli.repo.as_deref() {
        let matched_codebase = codebases
            .iter()
            .find(|codebase| same_github_repo(configured_repo, &codebase.repo_url))
            .cloned();
        if !codebases.is_empty() && matched_codebase.is_none() {
            anyhow::bail!(
                "Configured repo `{}` does not match any Immunefi Smart Contract GitHub repo for `{}`: {}. Refusing to audit the wrong repository.",
                configured_repo,
                bounty.project,
                codebases
                    .iter()
                    .map(|codebase| codebase.repo_url.clone())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        if let Some(codebase) = matched_codebase {
            apply_immunefi_resolved_repo(cli, &codebase, &bounty.project)?;
            apply_immunefi_codebase_location_hints(cli, &codebase);
        }
        return Ok(());
    }

    if codebases.is_empty() {
        anyhow::bail!(
            "Could not derive a cloneable Immunefi Smart Contract repo for `{}`. Specify `repo` manually.",
            bounty.project
        );
    }

    cli.resolved_repos = codebases
        .iter()
        .map(|codebase| ResolvedRepoConfig {
            repo_url: codebase.repo_url.clone(),
            branch: codebase.branch.clone(),
            tree_paths: codebase.tree_paths.clone(),
        })
        .collect();

    if codebases.len() > 1 {
        info!(
            "Derived {} repositories from Immunefi bounty `{}`: {}",
            codebases.len(),
            bounty.project,
            codebases
                .iter()
                .map(|codebase| codebase.repo_url.clone())
                .collect::<Vec<_>>()
                .join(", ")
        );
        cli.repo = Some(codebases[0].repo_url.clone());
        return Ok(());
    }

    let codebase = codebases.remove(0);
    apply_immunefi_resolved_repo(cli, &codebase, &bounty.project)?;
    apply_immunefi_codebase_location_hints(cli, &codebase);

    Ok(())
}

fn apply_immunefi_resolved_repo(
    cli: &mut Cli,
    codebase: &ResolvedGitCodebase,
    project: &str,
) -> Result<()> {
    if let Some(configured_repo) = cli.repo.as_deref() {
        if !same_github_repo(configured_repo, &codebase.repo_url) {
            anyhow::bail!(
                "Configured repo `{}` does not match Immunefi Resources GitHub Codebase `{}` for `{}`. Refusing to audit the wrong repository.",
                configured_repo,
                codebase.repo_url,
                project
            );
        }
    } else {
        info!(
            "Derived repository from Immunefi bounty `{}`: {}",
            project, codebase.repo_url
        );
    }
    cli.repo = Some(codebase.repo_url.clone());

    Ok(())
}

fn apply_immunefi_codebase_location_hints(cli: &mut Cli, codebase: &ResolvedGitCodebase) {
    if cli.repo_branch.is_none()
        && let Some(branch) = &codebase.branch
    {
        info!("Derived repository branch from Immunefi codebase URL: {branch}");
        cli.repo_branch = Some(branch.clone());
    }

    if cli.repo_tree_paths.is_empty() && !codebase.tree_paths.is_empty() {
        info!(
            "Derived repository tree path hints from Immunefi codebase URL: {:?}",
            codebase.tree_paths
        );
        cli.repo_tree_paths = codebase.tree_paths.clone();
    }
}

fn resolve_github_tree_refs(codebase: &mut ResolvedGitCodebase) -> Result<()> {
    if codebase.raw_tree_refs.is_empty() {
        return Ok(());
    }

    let remote_refs = list_remote_tree_refs(&codebase.repo_url)?;
    let mut branches = BTreeSet::new();
    let mut tree_paths = Vec::new();

    for raw_tree_ref in &codebase.raw_tree_refs {
        let segments = raw_tree_ref
            .split('/')
            .filter(|segment| !segment.is_empty())
            .collect::<Vec<_>>();
        let Some((git_ref, tree_path)) = split_github_tree_ref(&segments, &remote_refs) else {
            anyhow::bail!(
                "Could not resolve GitHub tree URL ref `{}` for `{}` against remote branches/tags",
                raw_tree_ref,
                codebase.repo_url
            );
        };
        branches.insert(git_ref);
        if let Some(tree_path) = tree_path {
            tree_paths.push(tree_path);
        }
    }

    if branches.len() > 1 {
        anyhow::bail!(
            "Immunefi bounty references one repo with multiple tree refs: {}. Configure this bounty manually for now.",
            branches.into_iter().collect::<Vec<_>>().join(", ")
        );
    }

    codebase.branch = branches.into_iter().next();
    tree_paths.sort();
    tree_paths.dedup();
    codebase.tree_paths = tree_paths;
    Ok(())
}

fn list_remote_tree_refs(repo_url: &str) -> Result<BTreeSet<String>> {
    let auth_url = add_github_auth(repo_url);
    let output = Command::new("git")
        .args(["ls-remote", "--heads", "--tags", &auth_url])
        .output()
        .context("Failed to run git ls-remote for Immunefi tree refs")?;

    if !output.status.success() {
        anyhow::bail!(
            "git ls-remote failed while resolving Immunefi tree refs: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let stdout = String::from_utf8(output.stdout)?;
    Ok(stdout
        .lines()
        .filter_map(|line| line.split_whitespace().nth(1))
        .filter(|reference| !reference.ends_with("^{}"))
        .filter_map(|reference| {
            reference
                .strip_prefix("refs/heads/")
                .or_else(|| reference.strip_prefix("refs/tags/"))
        })
        .map(str::to_string)
        .collect())
}

fn split_github_tree_ref(
    segments: &[&str],
    remote_refs: &BTreeSet<String>,
) -> Option<(String, Option<String>)> {
    for end in (1..=segments.len()).rev() {
        let candidate = segments[..end].join("/");
        if is_full_git_sha(&candidate) || remote_refs.contains(&candidate) {
            let tree_path = if end < segments.len() {
                Some(segments[end..].join("/"))
            } else {
                None
            };
            return Some((candidate, tree_path));
        }
    }
    None
}

fn infer_effective_code_folders(
    cli: &Cli,
    search_root: &Path,
    generated_context: Option<&GeneratedAuditContext>,
) -> Result<Vec<String>> {
    if !matches!(cli.audit_type, AuditType::ImmunefiBugBounty) {
        return Ok(cli.code_folders.clone());
    }
    let should_infer_from_scope = cli.code_folders.as_slice() == ["src"];
    if !should_infer_from_scope {
        return Ok(cli.code_folders.clone());
    }

    let tree_path_folders = immunefi_tree_code_folders(cli, search_root);
    let Some(context) = generated_context else {
        if tree_path_folders.is_empty() {
            return Ok(cli.code_folders.clone());
        }
        info!(
            "Using Immunefi tree path hints as code_folders: {:?}",
            tree_path_folders
        );
        return Ok(tree_path_folders);
    };
    let inferred = infer_code_folders_from_scope_txt(&context.scope_txt, search_root)?;
    if inferred.is_empty() {
        if tree_path_folders.is_empty() {
            warn!(
                "Could not infer Immunefi code_folders from generated scope {}; keeping default {:?}",
                context.scope_txt.display(),
                cli.code_folders
            );
            Ok(cli.code_folders.clone())
        } else {
            info!(
                "Could not infer Immunefi code_folders from generated scope {}; using tree path hints {:?}",
                context.scope_txt.display(),
                tree_path_folders
            );
            Ok(tree_path_folders)
        }
    } else {
        info!("Inferred Immunefi code_folders from scope: {:?}", inferred);
        Ok(inferred)
    }
}

fn immunefi_tree_code_folders(cli: &Cli, search_root: &Path) -> Vec<String> {
    if !matches!(cli.audit_type, AuditType::ImmunefiBugBounty)
        || cli.code_folders != vec!["src".to_string()]
    {
        return Vec::new();
    }

    let mut folders = cli
        .repo_tree_paths
        .iter()
        .filter_map(|path| normalize_immunefi_tree_path(path))
        .filter(|path| search_root.join(path).exists())
        .collect::<Vec<_>>();
    folders.sort_by_key(|folder| (folder.matches('/').count(), folder.len(), folder.clone()));
    folders.dedup();
    folders
}

fn normalize_immunefi_tree_path(path: &str) -> Option<String> {
    let normalized = path
        .trim()
        .trim_matches('/')
        .trim_start_matches("./")
        .trim_end_matches('/')
        .to_string();
    if normalized.is_empty() {
        None
    } else {
        Some(normalized)
    }
}

fn infer_code_folders_from_scope_txt(scope_txt: &Path, search_root: &Path) -> Result<Vec<String>> {
    let raw = fs::read_to_string(scope_txt).with_context(|| {
        format!(
            "Failed to read generated scope file {}",
            scope_txt.display()
        )
    })?;
    let mut folders = Vec::new();
    for line in raw.lines().map(str::trim).filter(|line| !line.is_empty()) {
        let relative = line.trim_start_matches("./").trim_start_matches('/');
        if relative.split('/').any(|part| {
            matches!(
                part,
                "node_modules" | ".git" | "out" | "cache" | "artifacts"
            )
        }) {
            continue;
        }
        let parts = relative.split('/').collect::<Vec<_>>();
        if parts.len() <= 1 {
            continue;
        }
        let folder = parts
            .iter()
            .position(|part| *part == "contracts" || *part == "src")
            .map(|index| parts[..=index].join("/"))
            .unwrap_or_else(|| parts[..parts.len() - 1].join("/"));
        if !folder.is_empty() && search_root.join(&folder).exists() {
            folders.push(folder);
        }
    }

    folders.sort_by_key(|folder| (folder.matches('/').count(), folder.len(), folder.clone()));
    folders.dedup();
    Ok(folders)
}

fn infer_effective_monorepo_folders(
    cli: &Cli,
    search_root: &Path,
    generated_context: Option<&GeneratedAuditContext>,
    repo_name: &str,
) -> Result<Option<PathBuf>> {
    if cli.monorepo_folders.is_some() || !matches!(cli.audit_type, AuditType::ImmunefiBugBounty) {
        return Ok(cli
            .monorepo_folders
            .as_ref()
            .map(|repos| Path::new(repos).to_path_buf()));
    }

    let Some(context) = generated_context else {
        return Ok(None);
    };
    let roots = infer_monorepo_roots_from_scope_txt(&context.scope_txt, search_root)?;
    if roots.len() <= 1 {
        return Ok(None);
    }

    let monorepo_path = context.output_dir.join(format!(
        "{}-monorepos.txt",
        sanitize_artifact_name(repo_name)
    ));
    let mut content = roots
        .iter()
        .map(|root| format!("./{}/", root.trim_matches('/')))
        .collect::<Vec<_>>()
        .join("\n");
    content.push('\n');
    fs::write(&monorepo_path, content)
        .with_context(|| format!("Failed to write {}", monorepo_path.display()))?;
    info!(
        "Generated Immunefi monorepo folders file from scope/build roots: {}",
        monorepo_path.display()
    );
    Ok(Some(monorepo_path))
}

fn infer_monorepo_roots_from_scope_txt(
    scope_txt: &Path,
    search_root: &Path,
) -> Result<Vec<String>> {
    let raw = fs::read_to_string(scope_txt).with_context(|| {
        format!(
            "Failed to read generated scope file {}",
            scope_txt.display()
        )
    })?;
    let mut roots = Vec::new();
    for line in raw.lines().map(str::trim).filter(|line| !line.is_empty()) {
        let relative = line.trim_start_matches("./").trim_start_matches('/');
        let mut current = search_root.join(relative).parent().map(Path::to_path_buf);
        while let Some(path) = current {
            if path == search_root {
                break;
            }
            if contains_build_config(&path) {
                let rel = path
                    .strip_prefix(search_root)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .replace('\\', "/");
                roots.push(rel);
                break;
            }
            current = path.parent().map(Path::to_path_buf);
        }
    }

    roots.sort();
    roots.dedup();
    Ok(roots)
}

fn sanitize_artifact_name(raw: &str) -> String {
    raw.chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

fn repo_slug_from_url(repo_url: &str) -> String {
    let clean = repo_url
        .trim()
        .trim_end_matches('/')
        .trim_end_matches(".git")
        .split('?')
        .next()
        .unwrap_or(repo_url)
        .split('#')
        .next()
        .unwrap_or(repo_url);
    if let Some(after_host) = clean.split("github.com/").nth(1) {
        let parts = after_host
            .split('/')
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>();
        if parts.len() >= 2 {
            return sanitize_artifact_name(&format!("{}-{}", parts[0], parts[1]));
        }
    }
    sanitize_artifact_name(clean.rsplit('/').next().unwrap_or("repo"))
}

fn immunefi_slug_from_url(url: &str) -> Option<&str> {
    let after = url.split("/bug-bounty/").nth(1)?;
    after
        .split(['/', '#', '?'])
        .find(|segment| !segment.is_empty())
}

fn has_build_artifacts(build_root: &Path) -> bool {
    build_root.join("out").exists()
        || build_root.join("artifacts").exists()
        || build_root.join("build").exists()
}

fn infer_build_root(cli: &Cli, cloned_repo_root: &Path, default_build_root: &Path) -> PathBuf {
    if cli.subfolder.is_some()
        || !matches!(cli.audit_type, AuditType::ImmunefiBugBounty)
        || cli.repo_tree_paths.len() != 1
    {
        return default_build_root.to_path_buf();
    }

    let Some(tree_path) = normalize_immunefi_tree_path(&cli.repo_tree_paths[0]) else {
        return default_build_root.to_path_buf();
    };
    let tree_build_root = cloned_repo_root.join(tree_path);
    if tree_build_root.exists()
        && contains_build_config(&tree_build_root)
        && !contains_build_config(cloned_repo_root)
    {
        info!(
            "Using Immunefi tree path as build root because the repository root has no build config: {}",
            tree_build_root.display()
        );
        tree_build_root
    } else {
        default_build_root.to_path_buf()
    }
}

fn clone_pinned_commit(
    workspace_path: &Path,
    repo_root: &str,
    repo_url: &str,
    commit: &str,
) -> Result<()> {
    let target = workspace_path.join(repo_root);
    fs::create_dir_all(&target)
        .with_context(|| format!("Failed to create clone target {}", target.display()))?;

    run_git_command(
        Command::new("git")
            .arg("init")
            .arg(".")
            .current_dir(&target),
        "git init",
    )?;
    run_git_command(
        Command::new("git")
            .args(["remote", "add", "origin", repo_url])
            .current_dir(&target),
        "git remote add origin",
    )?;
    run_git_command(
        Command::new("git")
            .args(["fetch", "--depth=1", "origin", commit])
            .current_dir(&target),
        "git fetch pinned commit",
    )?;
    run_git_command(
        Command::new("git")
            .args(["checkout", "--detach", "FETCH_HEAD"])
            .current_dir(&target),
        "git checkout pinned commit",
    )?;

    Ok(())
}

fn run_git_command(command: &mut Command, description: &str) -> Result<()> {
    let output = command
        .output()
        .with_context(|| format!("Failed to run {description}"))?;
    if !output.status.success() {
        anyhow::bail!(
            "{description} failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

pub fn clone_and_build_repo(cli: &Cli, repo_name: &str, project_id: &str) -> Result<PathBuf> {
    ensure_runtime_dependencies(
        "repository cloning",
        &[RuntimeDependency::Git, RuntimeDependency::Shell],
    )?;

    let workspace_path = audit_config().workspace_root_path().join(project_id);

    // Derived paths
    let repo_root = repo_name.split('/').next().unwrap_or(repo_name);
    let cloned_repo_root = workspace_path.join(repo_root);
    // Test override: allow tests to provide a local workspace path.
    if let Ok(local_ws) = std::env::var("AIAUDIT_TEST_LOCAL_WORKSPACE") {
        log::info!("Using test local workspace override at {}", local_ws);
        return Ok(PathBuf::from(local_ws));
    }

    let build_stamp = workspace_path.join(".chainshield_build_ok");

    let default_build_root = workspace_path.join(repo_name);
    let cached_build_roots = single_repo_build_roots(cli, &cloned_repo_root, &default_build_root);

    // Check if build artifacts exist (foundry uses 'out', hardhat uses 'artifacts')
    let has_build_artifacts = !cached_build_roots.is_empty()
        && cached_build_roots
            .iter()
            .all(|root| has_build_artifacts(root));
    let custom_build = matches!(cli.builder, BuilderType::Custom);

    // Determine if we should reuse the existing workspace
    let should_reuse = workspace_path.exists()
        && build_stamp.exists()
        && cloned_repo_root.exists()
        && (has_build_artifacts || custom_build)
        && !cli.force_rebuild;

    if should_reuse {
        log::info!(
            "✅ Reusing existing workspace ({}): {}",
            if has_build_artifacts {
                "build artifacts found"
            } else {
                "custom build stamp found"
            },
            workspace_path.display(),
        );
        return Ok(workspace_path);
    }

    // Log the reason for rebuild
    if workspace_path.exists() {
        let mut reasons = Vec::new();
        if cli.force_rebuild {
            reasons.push("--force-rebuild flag set");
        }
        if !build_stamp.exists() {
            reasons.push("build stamp missing");
        }
        if !has_build_artifacts && !custom_build {
            reasons.push("build artifacts (out/artifacts/build) not found");
        }

        log::warn!(
            "🔄 Rebuilding workspace at {} (reason: {})",
            workspace_path.display(),
            reasons.join(", ")
        );

        fs::remove_dir_all(&workspace_path).with_context(|| {
            format!(
                "Failed to remove existing local workspace {}",
                workspace_path.display()
            )
        })?;
    } else {
        log::info!("📦 Creating new workspace at {}", workspace_path.display());
    }
    fs::create_dir_all(&workspace_path)?;

    // Shallow clone for speed and security
    log::info!("git cloning repo...");

    let repo_url = add_github_auth(cli.get_repo());

    if let Some(commit) = cli
        .repo_branch
        .as_deref()
        .filter(|branch| is_full_git_sha(branch))
    {
        clone_pinned_commit(&workspace_path, repo_root, &repo_url, commit)?;
    } else {
        let mut clone_command = Command::new("git");
        clone_command.arg("clone").arg("--depth=1");
        if let Some(branch) = &cli.repo_branch {
            clone_command.arg("--branch").arg(branch);
        }
        let clone_output = clone_command
            .arg(&repo_url)
            .arg(repo_root)
            .current_dir(&workspace_path)
            .output()
            .context("Failed to run git clone")?;

        if !clone_output.status.success() {
            anyhow::bail!(
                "git clone failed for {}\nstdout:\n{}\nstderr:\n{}",
                cli.get_repo(),
                String::from_utf8_lossy(&clone_output.stdout),
                String::from_utf8_lossy(&clone_output.stderr)
            );
        }
    }

    configure_git_url_rewrites(&cloned_repo_root)?;

    let build_roots = single_repo_build_roots(cli, &cloned_repo_root, &default_build_root);
    if build_roots.is_empty() && !matches!(cli.builder, BuilderType::Custom) {
        anyhow::bail!(
            "No build system detected for '{}'. Specify `builder: Custom` with `build_cmd`, or provide the correct repo/tree path.",
            cli.get_repo()
        );
    }
    for build_root in &build_roots {
        if !build_root.exists() {
            anyhow::bail!(
                "Build root '{}' does not exist after cloning '{}'. Check the configured subfolder.",
                build_root.display(),
                cli.get_repo()
            );
        }
    }

    for build_root in &build_roots {
        ensure_build_runtime_dependencies(cli, build_root)?;
        run_build_command(cli, &workspace_path, build_root)?;
    }

    if build_roots.is_empty() {
        anyhow::bail!(
            "No build roots were selected for '{}'. Specify `builder: Custom` with `build_cmd`, or provide a repo/tree path with a supported build config.",
            cli.get_repo()
        );
    }

    // On success, stamp the workspace for reuse
    fs::write(
        &build_stamp,
        format!(
            "project_id={}\nrepo={}\ncommit={}\n",
            project_id,
            cli.get_repo(),
            &get_commit_hash(cli.get_repo(), cli.repo_branch.as_deref())?[..6]
        ),
    )?;

    Ok(workspace_path)
}

fn single_repo_build_roots(
    cli: &Cli,
    cloned_repo_root: &Path,
    default_build_root: &Path,
) -> Vec<PathBuf> {
    if cli.subfolder.is_some() || !matches!(cli.audit_type, AuditType::ImmunefiBugBounty) {
        return vec![default_build_root.to_path_buf()];
    }

    if cli.repo_tree_paths.len() <= 1 {
        return vec![infer_build_root(cli, cloned_repo_root, default_build_root)];
    }

    let mut roots = Vec::new();
    for tree_path in &cli.repo_tree_paths {
        if let Some(path) = normalize_immunefi_tree_path(tree_path) {
            let candidate = cloned_repo_root.join(path);
            if candidate.exists() && contains_build_config(&candidate) {
                roots.push(candidate);
            }
        }
    }
    roots.sort();
    roots.dedup();

    if matches!(cli.builder, BuilderType::Custom) {
        if roots.is_empty() {
            roots.push(default_build_root.to_path_buf());
        }
        return roots;
    }

    if !roots.is_empty() {
        return roots;
    }
    if contains_build_config(cloned_repo_root) {
        return vec![cloned_repo_root.to_path_buf()];
    }
    Vec::new()
}

fn clone_and_build_polyrepo_workspace(
    cli: &Cli,
    project_id: &str,
    commits: &[String],
) -> Result<PathBuf> {
    let workspace_path = audit_config().workspace_root_path().join(project_id);
    let build_stamp = workspace_path.join(".chainshield_build_ok");
    let expected_stamp = polyrepo_workspace_stamp(&cli.resolved_repos, commits);

    let should_reuse = workspace_path.exists()
        && build_stamp.exists()
        && fs::read_to_string(&build_stamp)
            .map(|stamp| stamp == expected_stamp)
            .unwrap_or(false)
        && !cli.force_rebuild
        && cli.resolved_repos.iter().all(|repo| {
            workspace_path
                .join(repo_slug_from_url(&repo.repo_url))
                .exists()
        })
        && polyrepo_has_reusable_build_outputs(cli, &workspace_path);

    if should_reuse {
        log::info!(
            "✅ Reusing existing Immunefi polyrepo workspace: {}",
            workspace_path.display()
        );
        return Ok(workspace_path);
    }

    if workspace_path.exists() {
        log::warn!(
            "🔄 Rebuilding Immunefi polyrepo workspace at {}",
            workspace_path.display()
        );
        fs::remove_dir_all(&workspace_path).with_context(|| {
            format!(
                "Failed to remove existing local workspace {}",
                workspace_path.display()
            )
        })?;
    } else {
        log::info!(
            "📦 Creating Immunefi polyrepo workspace at {}",
            workspace_path.display()
        );
    }
    fs::create_dir_all(&workspace_path)?;

    for repo in &cli.resolved_repos {
        let repo_root = repo_slug_from_url(&repo.repo_url);
        let target = workspace_path.join(&repo_root);
        let repo_url = add_github_auth(&repo.repo_url);
        log::info!("git cloning polyrepo member {}...", repo.repo_url);
        if let Some(commit) = repo
            .branch
            .as_deref()
            .filter(|branch| is_full_git_sha(branch))
        {
            clone_pinned_commit(&workspace_path, &repo_root, &repo_url, commit)?;
        } else {
            let mut clone_command = Command::new("git");
            clone_command.arg("clone").arg("--depth=1");
            if let Some(branch) = &repo.branch {
                clone_command.arg("--branch").arg(branch);
            }
            let clone_output = clone_command
                .arg(&repo_url)
                .arg(&repo_root)
                .current_dir(&workspace_path)
                .output()
                .context("Failed to run git clone for Immunefi polyrepo member")?;

            if !clone_output.status.success() {
                anyhow::bail!(
                    "git clone failed for {}\nstdout:\n{}\nstderr:\n{}",
                    repo.repo_url,
                    String::from_utf8_lossy(&clone_output.stdout),
                    String::from_utf8_lossy(&clone_output.stderr)
                );
            }
        }

        configure_git_url_rewrites(&target)?;

        let build_roots = polyrepo_build_roots(cli, repo, &target);
        if build_roots.is_empty() && !matches!(cli.builder, BuilderType::Custom) {
            anyhow::bail!(
                "No build system detected for Immunefi polyrepo member `{}`. Specify `builder: Custom` with `build_cmd`, or provide the correct repo/tree path.",
                repo.repo_url
            );
        }
        for build_root in build_roots {
            ensure_build_runtime_dependencies(cli, &build_root)?;
            run_build_command(cli, &workspace_path, &build_root)?;
        }
    }

    fs::write(&build_stamp, expected_stamp)?;
    Ok(workspace_path)
}

fn polyrepo_workspace_stamp(repos: &[ResolvedRepoConfig], commits: &[String]) -> String {
    let mut lines = repos
        .iter()
        .zip(commits.iter())
        .map(|(repo, commit)| {
            let mut tree_paths = repo.tree_paths.clone();
            tree_paths.sort();
            format!(
                "repo={} branch={} tree_paths={} commit={}",
                repo.repo_url,
                repo.branch.as_deref().unwrap_or(""),
                tree_paths.join(","),
                commit
            )
        })
        .collect::<Vec<_>>();
    lines.sort();
    format!("{}\n", lines.join("\n"))
}

fn polyrepo_workspace_fingerprint(repos: &[ResolvedRepoConfig], commits: &[String]) -> String {
    stable_short_hash(&polyrepo_workspace_stamp(repos, commits))
}

fn polyrepo_has_reusable_build_outputs(cli: &Cli, workspace_path: &Path) -> bool {
    if matches!(cli.builder, BuilderType::Custom) {
        return true;
    }
    cli.resolved_repos.iter().all(|repo| {
        let repo_root = workspace_path.join(repo_slug_from_url(&repo.repo_url));
        let build_roots = polyrepo_build_roots(cli, repo, &repo_root);
        !build_roots.is_empty() && build_roots.iter().all(|root| has_build_artifacts(root))
    })
}

fn stable_short_hash(input: &str) -> String {
    // FNV-1a 64-bit: deterministic across platforms and plenty for cache keys.
    let mut hash = 0xcbf29ce484222325u64;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")[..12].to_string()
}

fn polyrepo_build_roots(cli: &Cli, repo: &ResolvedRepoConfig, repo_root: &Path) -> Vec<PathBuf> {
    let mut roots = Vec::new();
    for tree_path in &repo.tree_paths {
        if let Some(path) = normalize_immunefi_tree_path(tree_path) {
            let candidate = repo_root.join(path);
            if candidate.exists() && contains_build_config(&candidate) {
                roots.push(candidate);
            }
        }
    }
    if roots.is_empty() {
        roots.push(repo_root.to_path_buf());
    }
    roots.sort();
    roots.dedup();

    if matches!(cli.builder, BuilderType::Custom) {
        return roots;
    }

    let configured_roots = roots
        .into_iter()
        .filter(|root| contains_build_config(root))
        .collect::<Vec<_>>();
    if !configured_roots.is_empty() {
        return configured_roots;
    }
    if contains_build_config(repo_root) {
        return vec![repo_root.to_path_buf()];
    }
    Vec::new()
}

fn run_build_command(cli: &Cli, workspace_path: &Path, build_root: &Path) -> Result<()> {
    let build_command = cli.generate_build_command();
    let pnpm_home = workspace_path.join(".pnpm");
    let path = std::env::var_os("PATH")
        .and_then(|existing_path| {
            std::env::join_paths(
                std::iter::once(pnpm_home.clone()).chain(std::env::split_paths(&existing_path)),
            )
            .ok()
        })
        .unwrap_or_else(|| pnpm_home.clone().into_os_string());

    let build_output = Command::new("sh")
        .args(["-lc", &build_command])
        .current_dir(build_root)
        .env("PNPM_HOME", &pnpm_home)
        .env("PATH", path)
        .output()
        .with_context(|| format!("Failed to run build command from {}", build_root.display()))?;

    if !build_output.status.success() {
        anyhow::bail!(
            "Repository build failed from '{}'.\n\nCommand:\n{}\n\nstdout:\n{}\nstderr:\n{}",
            build_root.display(),
            build_command,
            String::from_utf8_lossy(&build_output.stdout),
            String::from_utf8_lossy(&build_output.stderr)
        );
    }
    Ok(())
}

fn get_commit_hash(repo_url: &str, branch: Option<&str>) -> Result<String> {
    if let Some(branch) = branch
        && is_full_git_sha(branch)
    {
        return Ok(branch.to_string());
    }

    let auth_url = add_github_auth(repo_url);

    let patterns = if let Some(branch) = branch {
        vec![
            format!("refs/heads/{branch}"),
            format!("refs/tags/{branch}"),
            branch.to_string(),
        ]
    } else {
        vec!["HEAD".to_string()]
    };

    for git_ref in patterns {
        let output = Command::new("git")
            .args(["ls-remote", &auth_url, &git_ref])
            .output()
            .context("Failed to run git ls-remote")?;

        if !output.status.success() {
            anyhow::bail!(
                "git ls-remote failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        let stdout = String::from_utf8(output.stdout)?;
        if let Some(commit_hash) = first_ls_remote_hash(&stdout) {
            return Ok(commit_hash);
        }
    }

    anyhow::bail!(
        "Unexpected ls-remote output format for ref `{}`",
        branch.unwrap_or("HEAD")
    )
}

fn first_ls_remote_hash(stdout: &str) -> Option<String> {
    stdout
        .lines()
        .find(|line| !line.trim().is_empty() && !line.trim_end().ends_with("^{}"))
        .and_then(|line| line.split_whitespace().next())
        .map(str::to_string)
        .or_else(|| {
            stdout
                .lines()
                .find(|line| line.trim_end().ends_with("^{}"))
                .and_then(|line| line.split_whitespace().next())
                .map(str::to_string)
        })
}

fn is_full_git_sha(value: &str) -> bool {
    value.len() == 40 && value.chars().all(|ch| ch.is_ascii_hexdigit())
}

fn configure_git_url_rewrites(repo_root: &Path) -> Result<()> {
    let rewrites = [
        ("https://github.com/", "ssh://git@github.com/"),
        ("https://github.com/", "git@github.com:"),
        ("https://", "ssh://"),
    ];

    for (replacement, instead_of) in rewrites {
        let key = format!("url.{}.insteadOf", replacement);
        let output = Command::new("git")
            .args(["config", &key, instead_of])
            .current_dir(repo_root)
            .output()
            .with_context(|| {
                format!(
                    "Failed to configure git URL rewrite in {}",
                    repo_root.display()
                )
            })?;

        if !output.status.success() {
            anyhow::bail!(
                "Failed to configure git URL rewrite in '{}'.\nstdout:\n{}\nstderr:\n{}",
                repo_root.display(),
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    Ok(())
}

fn ensure_build_runtime_dependencies(cli: &Cli, build_root: &Path) -> Result<()> {
    let mut deps = vec![RuntimeDependency::Shell];

    match cli.builder {
        BuilderType::Foundry => {
            deps.extend([RuntimeDependency::Git, RuntimeDependency::Forge]);
        }
        BuilderType::Hardhat => {
            deps.extend([
                RuntimeDependency::Node,
                RuntimeDependency::Npm,
                RuntimeDependency::Npx,
            ]);
        }
        BuilderType::HardhatYarn => {
            deps.extend([RuntimeDependency::Node, RuntimeDependency::Yarn]);
        }
        BuilderType::Custom => {
            if let Some(command) = &cli.build_cmd {
                deps.extend(infer_custom_build_dependencies(command));
            }
        }
        BuilderType::Auto => {
            if build_root.join("foundry.toml").exists() || build_root.join("forge.toml").exists() {
                deps.extend([RuntimeDependency::Git, RuntimeDependency::Forge]);
            } else if build_root.join("hardhat.config.js").exists()
                || build_root.join("hardhat.config.ts").exists()
                || build_root.join("hardhat.config.cjs").exists()
            {
                deps.push(RuntimeDependency::Node);
                if build_root.join("yarn.lock").exists() {
                    deps.push(RuntimeDependency::Yarn);
                } else if build_root.join("pnpm-lock.yaml").exists() {
                    deps.push(RuntimeDependency::Pnpm);
                } else {
                    deps.extend([RuntimeDependency::Npm, RuntimeDependency::Npx]);
                }
            }
        }
    }

    ensure_runtime_dependencies("repository build", &deps)
}

fn infer_custom_build_dependencies(command: &str) -> Vec<RuntimeDependency> {
    let mut deps = Vec::new();

    if shell_command_mentions(command, "git") {
        deps.push(RuntimeDependency::Git);
    }
    if shell_command_mentions(command, "forge") {
        deps.push(RuntimeDependency::Forge);
    }
    if shell_command_mentions(command, "slither") {
        deps.push(RuntimeDependency::Slither);
    }
    if shell_command_mentions(command, "npm") {
        deps.extend([RuntimeDependency::Node, RuntimeDependency::Npm]);
    }
    if shell_command_mentions(command, "npx") {
        deps.extend([RuntimeDependency::Node, RuntimeDependency::Npx]);
    }
    if shell_command_mentions(command, "yarn") {
        deps.extend([RuntimeDependency::Node, RuntimeDependency::Yarn]);
    }
    if shell_command_mentions(command, "pnpm") {
        deps.extend([RuntimeDependency::Node, RuntimeDependency::Pnpm]);
    }
    if shell_command_mentions(command, "hardhat") && !deps.contains(&RuntimeDependency::Node) {
        deps.push(RuntimeDependency::Node);
    }

    deps
}

fn shell_command_mentions(command: &str, name: &str) -> bool {
    let pattern = format!(
        r"(^|[^A-Za-z0-9_./-]){}([^A-Za-z0-9_.-]|$)",
        regex::escape(name)
    );
    Regex::new(&pattern)
        .map(|regex| regex.is_match(command))
        .unwrap_or(false)
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
        let candidate = self.root.join(&self.repo_name);
        if candidate.exists() {
            candidate
        } else {
            self.root.clone()
        }
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
        if roots.is_empty()
            && protocol_root.exists()
            && let Ok(entries) = std::fs::read_dir(&protocol_root)
        {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() && contains_build_config(&path) {
                    roots.push(path);
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

        let protocol_folder = self.get_protocol_root();

        let filename = if file.starts_with(&self.root) {
            file.strip_prefix(&protocol_folder)?
                .to_string_lossy()
                .to_string()
        } else {
            file.file_name()
                .map(|f| f.to_string_lossy().to_string())
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

        let search_root = self.get_protocol_root();
        extract_list_of_files(monorepos, &search_root)
    }

    pub fn extract_scoped_files(&self) -> Result<Vec<PathBuf>> {
        let Some(scoped_files) = &self.scoped_files else {
            return Ok(Vec::new());
        };

        let search_root = self.get_protocol_root();
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
pub fn extract_list_of_files(files: &Path, root_folder: &Path) -> Result<Vec<PathBuf>> {
    let file = File::open(files)?;
    let reader = io::BufReader::new(file);

    let paths: Vec<PathBuf> = reader
        .lines()
        .map_while(Result::ok) // drop I/O errors
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty()) // skip blank lines
        .map(|p| {
            let path = p.strip_prefix("./").unwrap_or(&p);
            root_folder.join(path)
        }) // turn String into PathBuf
        .collect();

    Ok(paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_build_dependency_inference_detects_package_managers() {
        let deps = infer_custom_build_dependencies(
            "git submodule update --init && cd contracts && yarn hardhat compile && npx hardhat compile",
        );

        assert!(deps.contains(&RuntimeDependency::Git));
        assert!(deps.contains(&RuntimeDependency::Node));
        assert!(deps.contains(&RuntimeDependency::Yarn));
        assert!(deps.contains(&RuntimeDependency::Npx));
    }

    #[test]
    fn custom_build_dependency_inference_keeps_submodule_only_lightweight() {
        let deps = infer_custom_build_dependencies("git submodule update --init --recursive");

        assert_eq!(deps, vec![RuntimeDependency::Git]);
    }

    #[test]
    fn immunefi_code_folders_infer_contracts_root_from_scope() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("contracts/core")).unwrap();
        let scope = tmp.path().join("scope.txt");
        fs::write(
            &scope,
            "./contracts/SSVNetwork.sol\n./contracts/core/SSVNetworkViews.sol\n",
        )
        .unwrap();

        let folders = infer_code_folders_from_scope_txt(&scope, tmp.path()).unwrap();

        assert_eq!(folders, vec!["contracts"]);
    }

    #[test]
    fn immunefi_tree_paths_are_source_hints_not_subfolders() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("contracts")).unwrap();
        let cli: Cli = serde_yaml::from_str(
            r#"
repo: "https://github.com/org/repo"
audit_type: "ImmunefiBugBounty"
immunefi_bounty: "https://immunefi.com/bug-bounty/example/information/"
repo_tree_paths:
  - "contracts"
"#,
        )
        .unwrap();

        let folders = infer_effective_code_folders(&cli, tmp.path(), None).unwrap();

        assert_eq!(cli.subfolder, None);
        assert_eq!(folders, vec!["contracts"]);
    }

    #[test]
    fn immunefi_matching_tree_repo_is_normalized_to_clone_url() {
        let mut cli: Cli = serde_yaml::from_str(
            r#"
repo: "https://github.com/org/repo/tree/main/contracts"
audit_type: "ImmunefiBugBounty"
immunefi_bounty: "https://immunefi.com/bug-bounty/example/information/"
"#,
        )
        .unwrap();
        let codebase = ResolvedGitCodebase {
            repo_url: "https://github.com/org/repo".to_string(),
            branch: Some("main".to_string()),
            tree_paths: vec!["contracts".to_string()],
            raw_tree_refs: vec!["main/contracts".to_string()],
        };

        apply_immunefi_resolved_repo(&mut cli, &codebase, "Example").unwrap();

        assert_eq!(cli.repo.as_deref(), Some("https://github.com/org/repo"));
    }

    #[test]
    fn manual_immunefi_repo_inherits_resolved_tree_branch_and_paths() {
        let mut cli: Cli = serde_yaml::from_str(
            r#"
repo: "https://github.com/org/repo"
audit_type: "ImmunefiBugBounty"
immunefi_bounty: "https://immunefi.com/bug-bounty/example/information/"
"#,
        )
        .unwrap();
        let codebase = ResolvedGitCodebase {
            repo_url: "https://github.com/org/repo".to_string(),
            branch: Some("release/v2".to_string()),
            tree_paths: vec!["packages/a".to_string(), "packages/b".to_string()],
            raw_tree_refs: vec![
                "release/v2/packages/a".to_string(),
                "release/v2/packages/b".to_string(),
            ],
        };

        apply_immunefi_resolved_repo(&mut cli, &codebase, "Example").unwrap();
        apply_immunefi_codebase_location_hints(&mut cli, &codebase);

        assert_eq!(cli.repo.as_deref(), Some("https://github.com/org/repo"));
        assert_eq!(cli.repo_branch.as_deref(), Some("release/v2"));
        assert_eq!(
            cli.repo_tree_paths,
            vec!["packages/a".to_string(), "packages/b".to_string()]
        );
    }

    #[test]
    fn immunefi_build_root_uses_tree_path_only_when_root_has_no_config() {
        let tmp = tempfile::tempdir().unwrap();
        let repo_root = tmp.path().join("repo");
        let package_root = repo_root.join("packages/protocol");
        fs::create_dir_all(&package_root).unwrap();
        fs::write(package_root.join("foundry.toml"), "[profile.default]\n").unwrap();
        let cli: Cli = serde_yaml::from_str(
            r#"
repo: "https://github.com/org/repo"
audit_type: "ImmunefiBugBounty"
immunefi_bounty: "https://immunefi.com/bug-bounty/example/information/"
repo_tree_paths:
  - "packages/protocol"
"#,
        )
        .unwrap();

        let build_root = infer_build_root(&cli, &repo_root, &repo_root);

        assert_eq!(build_root, package_root);
    }

    #[test]
    fn polyrepo_build_roots_do_not_append_unconfigured_repo_root_after_tree_root() {
        let tmp = tempfile::tempdir().unwrap();
        let repo_root = tmp.path().join("repo");
        let package_root = repo_root.join("packages/protocol");
        fs::create_dir_all(&package_root).unwrap();
        fs::write(package_root.join("foundry.toml"), "[profile.default]\n").unwrap();
        let cli: Cli = serde_yaml::from_str(
            r#"
repo: "https://github.com/org/repo"
audit_type: "ImmunefiBugBounty"
immunefi_bounty: "https://immunefi.com/bug-bounty/example/information/"
"#,
        )
        .unwrap();
        let repo = ResolvedRepoConfig {
            repo_url: "https://github.com/org/repo".to_string(),
            branch: Some("main".to_string()),
            tree_paths: vec!["packages/protocol".to_string()],
        };

        let roots = polyrepo_build_roots(&cli, &repo, &repo_root);

        assert_eq!(roots, vec![package_root]);
    }

    #[test]
    fn single_repo_build_roots_include_multiple_immunefi_tree_roots() {
        let tmp = tempfile::tempdir().unwrap();
        let repo_root = tmp.path().join("repo");
        let package_a = repo_root.join("packages/a");
        let package_b = repo_root.join("packages/b");
        fs::create_dir_all(&package_a).unwrap();
        fs::create_dir_all(&package_b).unwrap();
        fs::write(package_a.join("foundry.toml"), "[profile.default]\n").unwrap();
        fs::write(
            package_b.join("hardhat.config.js"),
            "module.exports = {};\n",
        )
        .unwrap();
        let cli: Cli = serde_yaml::from_str(
            r#"
repo: "https://github.com/org/repo"
audit_type: "ImmunefiBugBounty"
immunefi_bounty: "https://immunefi.com/bug-bounty/example/information/"
repo_tree_paths:
  - "packages/a"
  - "packages/b"
"#,
        )
        .unwrap();

        let roots = single_repo_build_roots(&cli, &repo_root, &repo_root);

        assert_eq!(roots, vec![package_a, package_b]);
    }

    #[test]
    fn tree_ref_resolution_prefers_longest_branch_match() {
        let refs = BTreeSet::from(["release".to_string(), "release/v1".to_string()]);
        let segments = ["release", "v1", "contracts"];

        let (git_ref, tree_path) = split_github_tree_ref(&segments, &refs).unwrap();

        assert_eq!(git_ref, "release/v1");
        assert_eq!(tree_path.as_deref(), Some("contracts"));
    }

    #[test]
    fn commit_hash_accepts_pinned_sha_without_remote_lookup() {
        let sha = "0123456789abcdef0123456789abcdef01234567";

        let commit = get_commit_hash("https://github.com/org/repo", Some(sha)).unwrap();

        assert_eq!(commit, sha);
    }

    #[test]
    fn immunefi_monorepo_roots_infer_multiple_build_roots() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("ve33/contracts")).unwrap();
        fs::create_dir_all(tmp.path().join("cl/contracts/core")).unwrap();
        fs::write(tmp.path().join("ve33/foundry.toml"), "[profile.default]\n").unwrap();
        fs::write(tmp.path().join("cl/foundry.toml"), "[profile.default]\n").unwrap();
        let scope = tmp.path().join("scope.txt");
        fs::write(
            &scope,
            "./ve33/contracts/Voter.sol\n./cl/contracts/core/CLPool.sol\n",
        )
        .unwrap();

        let roots = infer_monorepo_roots_from_scope_txt(&scope, tmp.path()).unwrap();

        assert_eq!(roots, vec!["cl", "ve33"]);
    }

    #[test]
    fn polyrepo_repo_paths_use_workspace_root_when_slug_folder_is_absent() {
        let tmp = tempfile::tempdir().unwrap();
        let member_contract = tmp.path().join("ens-contracts/contracts/ENSRegistry.sol");
        fs::create_dir_all(member_contract.parent().unwrap()).unwrap();
        fs::write(&member_contract, "contract ENSRegistry {}\n").unwrap();
        let monorepos = tmp.path().join("monorepos.txt");
        fs::write(&monorepos, "./ens-contracts\n").unwrap();
        let scoped_files = tmp.path().join("scope.txt");
        fs::write(&scoped_files, "./ens-contracts/contracts/ENSRegistry.sol\n").unwrap();

        let repo = RepoPaths {
            github_url: "https://github.com/ensdomains/ens-contracts".to_string(),
            project_id: "ens-abc123".to_string(),
            root: tmp.path().to_path_buf(),
            sol_files: vec![member_contract.clone()],
            test_files: vec![],
            script_files: vec![],
            config_files: vec![],
            lib_config_files: vec![],
            source_code_folders: vec![tmp.path().join("ens-contracts/contracts")],
            docs: vec![],
            repo_name: "ens".to_string(),
            audit_scope: None,
            excluded_folders: None,
            scoped_files: Some(scoped_files),
            monorepo_folders: Some(monorepos),
            commit_hash: "abcdef1234567890abcdef1234567890abcdef12".to_string(),
            audit_type: AuditType::ImmunefiBugBounty,
        };

        assert_eq!(repo.get_protocol_root(), tmp.path());
        assert_eq!(
            repo.extract_scoped_files().unwrap(),
            vec![member_contract.clone()]
        );
        assert_eq!(
            repo.extract_monorepo_folders().unwrap(),
            vec![tmp.path().join("ens-contracts")]
        );
    }

    #[test]
    fn polyrepo_workspace_fingerprint_changes_when_non_first_repo_changes() {
        let repos = vec![
            ResolvedRepoConfig {
                repo_url: "https://github.com/example/first".to_string(),
                branch: Some("main".to_string()),
                tree_paths: vec![],
            },
            ResolvedRepoConfig {
                repo_url: "https://github.com/example/second".to_string(),
                branch: Some("main".to_string()),
                tree_paths: vec!["packages/contracts".to_string()],
            },
        ];
        let original = vec![
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".to_string(),
        ];
        let changed_second = vec![
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
            "cccccccccccccccccccccccccccccccccccccccc".to_string(),
        ];

        assert_ne!(
            polyrepo_workspace_fingerprint(&repos, &original),
            polyrepo_workspace_fingerprint(&repos, &changed_second)
        );
        assert_ne!(
            polyrepo_workspace_stamp(&repos, &original),
            polyrepo_workspace_stamp(&repos, &changed_second)
        );
    }

    #[test]
    fn polyrepo_unique_repo_hash_changes_when_non_first_repo_changes() {
        let repos = vec![
            ResolvedRepoConfig {
                repo_url: "https://github.com/example/first".to_string(),
                branch: Some("main".to_string()),
                tree_paths: vec![],
            },
            ResolvedRepoConfig {
                repo_url: "https://github.com/example/second".to_string(),
                branch: Some("main".to_string()),
                tree_paths: vec!["packages/contracts".to_string()],
            },
        ];
        let original = vec![
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".to_string(),
        ];
        let changed_second = vec![
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
            "cccccccccccccccccccccccccccccccccccccccc".to_string(),
        ];
        let repo_with_commits = |commits: &[String]| RepoPaths {
            github_url: "https://github.com/example/first, https://github.com/example/second"
                .to_string(),
            project_id: "example-polyrepo".to_string(),
            root: PathBuf::from("/tmp"),
            sol_files: vec![],
            test_files: vec![],
            script_files: vec![],
            config_files: vec![],
            lib_config_files: vec![],
            source_code_folders: vec![],
            docs: vec![],
            repo_name: "example".to_string(),
            audit_scope: None,
            excluded_folders: None,
            scoped_files: None,
            monorepo_folders: None,
            commit_hash: format!(
                "{}-{}",
                polyrepo_workspace_fingerprint(&repos, commits),
                commits.join("-")
            ),
            audit_type: AuditType::ImmunefiBugBounty,
        };

        assert_ne!(
            repo_with_commits(&original).unique_repo_hash(),
            repo_with_commits(&changed_second).unique_repo_hash()
        );
    }

    #[test]
    fn polyrepo_reuse_requires_build_artifacts_for_auto_builds() {
        let tmp = tempfile::tempdir().unwrap();
        let repo_root = tmp.path().join("org-repo");
        let package_root = repo_root.join("packages/protocol");
        fs::create_dir_all(&package_root).unwrap();
        fs::write(package_root.join("foundry.toml"), "[profile.default]\n").unwrap();
        let mut cli: Cli = serde_yaml::from_str(
            r#"
repo: "https://github.com/org/repo"
audit_type: "ImmunefiBugBounty"
immunefi_bounty: "https://immunefi.com/bug-bounty/example/information/"
"#,
        )
        .unwrap();
        cli.resolved_repos = vec![ResolvedRepoConfig {
            repo_url: "https://github.com/org/repo".to_string(),
            branch: Some("main".to_string()),
            tree_paths: vec!["packages/protocol".to_string()],
        }];

        assert!(!polyrepo_has_reusable_build_outputs(&cli, tmp.path()));

        fs::create_dir_all(package_root.join("out")).unwrap();

        assert!(polyrepo_has_reusable_build_outputs(&cli, tmp.path()));
    }

    #[test]
    fn polyrepo_repo_slugs_are_owner_qualified() {
        assert_eq!(
            repo_slug_from_url("https://github.com/alpha/contracts"),
            "alpha-contracts"
        );
        assert_eq!(
            repo_slug_from_url("https://github.com/beta/contracts.git"),
            "beta-contracts"
        );
    }
}
