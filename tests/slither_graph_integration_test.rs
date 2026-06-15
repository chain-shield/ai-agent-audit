use ai_agent_audit::{
    build_brain::slither_ffi::build_slither_args, config::AuditType,
    prepare_code::git_clone::RepoPaths,
};
use std::path::{Path, PathBuf};
use std::process::Command;

fn graph_workspace_root() -> Option<PathBuf> {
    std::env::var_os("AI_AGENT_AUDIT_GRAPH_WORKSPACE")
        .map(PathBuf::from)
        .filter(|path| path.exists())
}

fn graph_repo(root: &Path) -> RepoPaths {
    RepoPaths {
        github_url: "https://github.com/graphprotocol/contracts, https://github.com/graphprotocol/token-distribution".to_string(),
        project_id: "thegraph-integration".to_string(),
        root: root.to_path_buf(),
        sol_files: vec![],
        test_files: vec![],
        script_files: vec![],
        config_files: vec![],
        lib_config_files: vec![],
        source_code_folders: vec![],
        docs: vec![],
        repo_name: "thegraph".to_string(),
        audit_scope: None,
        excluded_folders: None,
        scoped_files: None,
        monorepo_folders: None,
        commit_hash: "52b5356d3efea1508396b7f46a18854fb7ae9112".to_string(),
        audit_type: AuditType::ImmunefiBugBounty,
    }
}

#[test]
#[ignore = "requires the prepared The Graph polyrepo workspace and Slither"]
fn graph_hardhat_artifact_layouts_run_slither_printers() {
    let Some(root) = graph_workspace_root() else {
        eprintln!("Skipping: AI_AGENT_AUDIT_GRAPH_WORKSPACE/default Graph workspace not found");
        return;
    };
    let repo = graph_repo(&root);

    let packages = [
        ("graphprotocol-contracts/packages/horizon", "build"),
        (
            "graphprotocol-contracts/packages/subgraph-service",
            "build/contracts",
        ),
        ("graphprotocol-token-distribution", "build/artifacts"),
    ];

    for (relative_package, expected_artifacts_dir) in packages {
        let package = root.join(relative_package);
        assert!(
            package.exists(),
            "missing Graph package fixture: {}",
            package.display()
        );

        for (printer, json_output) in [("call-graph", true), ("slithir-ssa", false)] {
            let args = build_slither_args(
                &repo,
                Some(printer),
                Some(package.clone()),
                json_output,
                true,
            );
            let joined = args.join(" ");
            assert!(
                joined.contains(&format!(
                    "--hardhat-artifacts-directory {expected_artifacts_dir}"
                )),
                "wrong Slither artifacts dir for {relative_package}: {joined}"
            );

            let output = Command::new("slither")
                .current_dir(&root)
                .args(&args)
                .output()
                .expect("failed to run slither");

            assert!(
                output.status.success(),
                "Slither {printer} failed for {relative_package}: {}",
                String::from_utf8_lossy(&output.stderr)
            );

            if json_output {
                assert!(
                    !output.stdout.is_empty(),
                    "Slither produced empty JSON for {relative_package}"
                );
                let json: serde_json::Value =
                    serde_json::from_slice(&output.stdout).expect("Slither output should be JSON");
                assert_eq!(
                    json.get("success").and_then(|value| value.as_bool()),
                    Some(true)
                );
            } else {
                let text = if output.stdout.is_empty() {
                    &output.stderr
                } else {
                    &output.stdout
                };
                assert!(
                    !text.is_empty(),
                    "Slither {printer} produced empty output for {relative_package}"
                );
            }
        }
    }
}
