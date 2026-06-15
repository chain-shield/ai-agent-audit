use ai_agent_audit::{
    build_brain::{
        callgraph::extract_dot_blobs, enrichment::build_semantics_db_from_call_graph,
        slither_ffi::run_printer_json,
    },
    config::AuditType,
    prepare_code::git_clone::RepoPaths,
};
use rusqlite::Connection;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

fn ssv_repo_root() -> PathBuf {
    std::env::var_os("SSV_INTEGRATION_ROOT")
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var_os("HOME")
                .map(PathBuf::from)
                .map(|home| home.join("Desktop/Audit/ssv-network-9bb7b2"))
        })
        .unwrap_or_else(|| PathBuf::from("ssv-network-9bb7b2"))
}

fn slither_available() -> bool {
    Command::new("slither")
        .arg("--version")
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false)
}

fn ssv_repo_paths(root: &Path) -> RepoPaths {
    RepoPaths {
        github_url:
            "https://github.com/ssvlabs/ssv-network/blob/9bb7b21d4432f34f623bed3e0bb3fa77f1e5d2b9"
                .to_string(),
        project_id: "ssv-network-9bb7b2".to_string(),
        root: root.to_path_buf(),
        sol_files: vec![],
        test_files: vec![],
        script_files: vec![],
        config_files: vec![],
        lib_config_files: vec![],
        source_code_folders: vec![root.join("ssv-network/contracts")],
        docs: vec![],
        repo_name: "ssv-network".to_string(),
        audit_scope: None,
        excluded_folders: None,
        scoped_files: None,
        monorepo_folders: None,
        commit_hash: "9bb7b21d4432f34f623bed3e0bb3fa77f1e5d2b9".to_string(),
        audit_type: AuditType::ImmunefiBugBounty,
    }
}

#[tokio::test]
async fn ssv_mixed_hardhat_foundry_call_graph_uses_foundry_fallback() -> anyhow::Result<()> {
    if !slither_available() {
        eprintln!("skipping SSV Slither integration test: slither is not on PATH");
        return Ok(());
    }

    let root = ssv_repo_root();
    let protocol_root = root.join("ssv-network");
    if !protocol_root.join("foundry.toml").exists() || !protocol_root.join("out").exists() {
        eprintln!(
            "skipping SSV Slither integration test: expected built SSV repo at {}",
            protocol_root.display()
        );
        return Ok(());
    }

    let repo = ssv_repo_paths(&root);
    let json = run_printer_json(&repo, "call-graph", None).await?;
    let blobs = extract_dot_blobs(&json)?;

    assert!(
        blobs.iter().any(|blob| blob.contains("SSVNetwork")),
        "SSVNetwork call graph should be present"
    );
    assert!(
        blobs.iter().any(|blob| blob.contains("SSVNetworkViews")),
        "SSVNetworkViews call graph should be present"
    );

    let semantic_db = build_semantics_db_from_call_graph(repo.clone()).await?;
    let conn = Connection::open(semantic_db)?;
    let ssv_network_functions: i64 = conn.query_row(
        "SELECT COUNT(*) FROM functions WHERE project_id = ?1 AND contract = 'SSVNetwork'",
        [&repo.project_id],
        |row| row.get(0),
    )?;
    assert!(
        ssv_network_functions > 0,
        "semantic DB should include SSVNetwork functions"
    );

    Ok(())
}
