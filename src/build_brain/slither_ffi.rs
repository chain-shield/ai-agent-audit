use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// One SlithIR function entry (simplified)
#[derive(Debug, Serialize, Deserialize)]
pub struct SlithIRFn {
    pub contract: String,
    pub function: String,
    pub ir: String,
}

/// Single storage slot description
#[derive(Debug, Serialize, Deserialize)]
pub struct StorageVar {
    pub contract: String,
    pub name: String,
    pub r#type: String,
}

/// Run *one* printer, capture stdout, return raw text
fn run_printer(repo_root: &Path, printer: &str) -> Result<String> {
    let output = Command::new("slither")
        .current_dir(repo_root)
        .args([".", "--print", printer, "--quiet"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("Slither printer `{printer}` failed"));
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// Parse the `slithir-ssa` printer output -> Vec<SlithIRFn>
pub fn parse_slithir(text: &str) -> Vec<SlithIRFn> {
    let mut current_contract = String::new();
    let mut current_fn = String::new();
    let mut buf = String::new();
    let mut out = Vec::new();

    for line in text.lines() {
        if line.starts_with("Contract ") {
            current_contract = line["Contract ".len()..].trim_end_matches(':').to_owned();
        } else if line.starts_with("\tFunction ") {
            // flush previous
            if !current_fn.is_empty() {
                out.push(SlithIRFn {
                    contract: current_contract.clone(),
                    function: current_fn.clone(),
                    ir: buf.trim().to_owned(),
                });
            }
            current_fn = line.trim()[9..].trim_end_matches(':').to_owned();
            buf.clear();
        } else if line.starts_with("\t\t") {
            buf.push_str(line.trim_start());
            buf.push('\n');
        }
    }
    // flush last
    if !current_fn.is_empty() {
        out.push(SlithIRFn {
            contract: current_contract,
            function: current_fn,
            ir: buf.trim().to_owned(),
        });
    }
    out
}

/// Parse `variable-order` printer output -> Vec<StorageVar>
pub fn parse_storage(text: &str) -> Vec<StorageVar> {
    let mut current_contract = String::new();
    let mut vars = Vec::new();
    for line in text.lines() {
        if line.starts_with("Contract") && line.ends_with(':') {
            current_contract = line["Contract".len()..]
                .trim_end_matches(':')
                .trim()
                .to_owned();
        } else if line.starts_with('|') && line.contains('|') {
            let cols: Vec<_> = line.split('|').map(|c| c.trim()).collect();
            if cols.len() >= 3 && cols[1] != "Name" {
                vars.push(StorageVar {
                    contract: current_contract.clone(),
                    name: cols[1].to_owned(),
                    r#type: cols[2].to_owned(),
                });
            }
        }
    }
    vars
}

/// Public façade: run both printers, return parsed artefacts
pub fn dump_ir_and_storage(repo_root: &Path) -> Result<(Vec<SlithIRFn>, Vec<StorageVar>)> {
    let ir_raw = run_printer(repo_root, "slithir-ssa")?;
    let storage_raw = run_printer(repo_root, "variable-order")?;

    Ok((parse_slithir(&ir_raw), parse_storage(&storage_raw)))
}

pub fn dump_chunks_to_dir(repo_root: &Path, dir: &Path) -> Result<Vec<PathBuf>> {
    // 1 . gather IR + storage  (re-use existing function)
    let (ir_vec, storage_vec) = dump_ir_and_storage(repo_root)?;

    // 2 . serialise each artefact → one text file
    let mut out_paths = Vec::new();

    for fn_ir in &ir_vec {
        let meta = format!("IR:{}::{}", fn_ir.contract, fn_ir.function);
        let p = dir.join(meta.replace("::", "_") + ".txt");
        fs::write(&p, &fn_ir.ir)?;
        out_paths.push(p);
    }
    for var in &storage_vec {
        let meta = format!("STORAGE:{}::{}", var.contract, var.name);
        let body = format!("{} {}", var.name, var.r#type);
        let p = dir.join(meta.replace("::", "_") + ".txt");
        fs::write(&p, body)?;
        out_paths.push(p);
    }

    Ok(out_paths)
}
