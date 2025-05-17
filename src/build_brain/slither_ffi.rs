use anyhow::{anyhow, Result};
use log::{debug, info};
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
        .args(&[
            ".",
            "--foundry-ignore-compile",
            "--foundry-out-directory",
            "out",
            "--print",
            printer,
            "--disable-color",
        ])
        .stdout(Stdio::piped()) // capture printer text
        .stderr(Stdio::piped()) // capture banner & errors (nothing hits tty)
        .output()?;

    // ❶  use whichever stream is non-empty
    let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
    if text.trim().is_empty() {
        text = String::from_utf8_lossy(&output.stderr).into_owned();
    }

    if text.trim().is_empty() {
        return Err(anyhow!("Slither ran but produced no `{}` output", printer));
    }

    Ok(text)
}

/// Parse the `slithir-ssa` printer output -> Vec<SlithIRFn>
pub fn parse_slithir(text: &str) -> Vec<SlithIRFn> {
    let mut current_contract = String::new();
    let mut current_fn = String::new();
    let mut buf = String::new();
    let mut out = Vec::new();

    info!("text in parse_slithir {}", text.len());
    for line in text.lines() {
        // info!("LINE => {}", line);
        if line.starts_with("Contract ") {
            current_contract = line["Contract ".len()..].trim_end_matches(':').to_owned();
        } else if line.starts_with("\tFunction ") {
            // flush previous
            if !current_fn.is_empty() {
                let ir_content = replace_special_character(&buf);
                out.push(SlithIRFn {
                    contract: current_contract.clone(),
                    function: current_fn.clone(),
                    ir: ir_content,
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
    if !current_fn.is_empty() && !buf.trim().is_empty() && buf.trim().len() > 10 {
        let ir_content = replace_special_character(&buf);
        out.push(SlithIRFn {
            contract: current_contract,
            function: current_fn,
            ir: ir_content,
        });
    } else if !current_fn.is_empty() {
        info!(
            "Skipping empty or small IR for {}::{}",
            current_contract, current_fn
        );
    }
    info!("functions => {:#?}", out.len());
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
            if cols.len() >= 3 && cols[1] != "Name" && !cols[1].is_empty() && !cols[2].is_empty() {
                vars.push(StorageVar {
                    contract: current_contract.clone(),
                    name: cols[1].to_owned(),
                    r#type: cols[2].to_owned(),
                });
            } else {
                debug!(
                    "Skipping invalid storage var in {}: {:?}",
                    current_contract, cols
                );
            }
        }
    }
    info!("storage => {:#?}", vars.len());
    vars
}

pub fn replace_special_character(text: &str) -> String {
    let cleaned_text = text.trim().replace("ϕ", "phi");

    cleaned_text
}
/// Public façade: run both printers, return parsed artefacts
pub fn dump_ir_and_storage(repo_root: &Path) -> Result<(Vec<SlithIRFn>, Vec<StorageVar>)> {
    let ir_raw = run_printer(repo_root, "slithir-ssa")?;
    let storage_raw = run_printer(repo_root, "variable-order")?;
    info!("storage raw => {}", storage_raw.len());
    Ok((parse_slithir(&ir_raw), parse_storage(&storage_raw)))
}

pub fn dump_chunks_to_dir(repo_root: &Path, dir: &Path) -> Result<Vec<PathBuf>> {
    // 1 . gather IR + storage  (re-use existing function)
    info!("get ir and storage chuncks");
    let (ir_vec, storage_vec) = dump_ir_and_storage(repo_root)?;
    info!("storage vec => {:?}", storage_vec.len());

    // 2 . serialise each artefact → one text file
    let mut out_paths = Vec::new();

    info!("convert fn ir to txt files");
    for fn_ir in &ir_vec {
        let meta = format!("IR:{}::{}", fn_ir.contract, fn_ir.function);
        // info!("fn ir meta => {}", meta);

        let p = dir.join(meta.replace("::", "_") + ".txt");
        fs::write(&p, &fn_ir.ir)?;
        out_paths.push(p);
    }

    info!("convert storage vars to txt files");
    for var in &storage_vec {
        let meta = format!("STORAGE:{}::{}", var.contract, var.name);
        // info!("storage meta => {}", meta);
        let body = format!("{} {}", var.name, var.r#type);
        let p = dir.join(meta.replace("::", "_") + ".txt");
        fs::write(&p, body)?;
        out_paths.push(p);
    }

    info!("fn ir and storage var files => {:?}", out_paths.len());

    Ok(out_paths)
}
