use ai_agent_audit::{
    build_brain::slither_ffi::build_slither_args, config::AuditType,
    prepare_code::git_clone::RepoPaths,
};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn make_repo_layout(root: &Path, repo_name: &str, files: &[&str], dirs: &[&str]) -> RepoPaths {
    let repo_root = root.join(repo_name);
    fs::create_dir_all(&repo_root).unwrap();
    for d in dirs {
        fs::create_dir_all(repo_root.join(d)).unwrap();
    }
    for f in files {
        let p = repo_root.join(f);
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        let mut file = File::create(p).unwrap();
        let _ = file.write_all(b"\n");
    }
    RepoPaths {
        project_id: format!("{}-TEST", repo_name),
        root: root.to_path_buf(),
        sol_files: vec![],
        monorepo_folders: None,
        test_files: vec![],
        script_files: vec![],
        config_files: vec![],
        lib_config_files: vec![],
        source_code_folders: vec![repo_root.join("contracts")],
        docs: vec![],
        repo_name: repo_name.to_string(),
        audit_scope: None,
        excluded_folders: None,
        scoped_files: None,
        commit_hash: "deadbeefdeadbeefdeadbeefdeadbeefdeadbeef".to_string(),
        audit_type: AuditType::Code4rena,
        poc: ai_agent_audit::prepare_code::git_clone::PocConfig::default(),
    }
}

#[test]
fn hardhat_repo_pref_over_foundry_yarn() {
    let tmp = std::env::temp_dir().join("slither_ffi_detect_hardhat_int");
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(&tmp).unwrap();
    let repo = make_repo_layout(
        &tmp,
        "2025-08-flare",
        &[
            "package.json",
            "yarn.lock",
            "hardhat.config.ts",
            "foundry.toml",
        ],
        &[],
    );
    let args = build_slither_args(&repo, Some("slithir-ssa"), None, false);
    let joined = args.join(" ");
    assert!(joined.contains("--compile-force-framework hardhat"));
    assert!(joined.contains("--hardhat-ignore-compile"));
    assert!(joined.contains("--hardhat-artifacts-directory artifacts"));
    assert!(!joined.contains("--foundry-ignore-compile"));
}

#[test]
fn foundry_yarn_without_hardhat_config() {
    let tmp = std::env::temp_dir().join("slither_ffi_detect_foundryyarn_int");
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(&tmp).unwrap();
    let repo = make_repo_layout(
        &tmp,
        "my-repo",
        &["package.json", "yarn.lock", "foundry.toml"],
        &[],
    );
    let args = build_slither_args(&repo, None, None, false);
    let joined = args.join(" ");
    assert!(!joined.contains("--foundry-ignore-compile"));
    assert!(!joined.contains("--hardhat-ignore-compile"));
}

#[test]
fn hardhat_monorepo_artifacts_dir() {
    let tmp = std::env::temp_dir().join("slither_ffi_hardhat_monorepo_int");
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(&tmp).unwrap();
    let repo = make_repo_layout(
        &tmp,
        "mono",
        &["package.json", "yarn.lock", "hardhat.config.ts"],
        &["packages/hardhat/artifacts"],
    );
    let args = build_slither_args(&repo, Some("slithir-ssa"), None, false);
    let joined = args.join(" ");
    assert!(joined.contains("--compile-force-framework hardhat"));
    assert!(joined.contains("--hardhat-ignore-compile"));
    assert!(joined.contains("--hardhat-artifacts-directory packages/hardhat/artifacts"));
}
