use super::slither_ffi::{ContractSummary, SlithIRFn, StorageVar};
use log::{debug, info};
pub fn parse_slither(text: &str) -> Vec<String> {
    let mut current_issue = String::new();
    let mut issues = Vec::<String>::new();

    for line in text.lines() {
        // parse each detected issue
        if line.contains("Detectors") && line.ends_with(':') {
            if !current_issue.is_empty() {
                issues.push(current_issue.to_string());
                current_issue = line.to_string();
            }
        } else {
            current_issue.push_str(line);
        }
    }

    // add last issues
    if !current_issue.is_empty() {
        issues.push(current_issue.to_string());
    }

    // info!("issues => {:#?}", issues);
    issues
}

/// Parses the output of the Slither 'slithir-ssa' printer into a vector of SlithIRFn structs.
///
/// This function processes the text output from Slither's slithir-ssa printer,
/// which contains the SlithIR representation of functions in Solidity contracts.
/// It extracts the contract name, function name, and IR content for each function.
///
/// @param text - The raw text output from the slithir-ssa printer
/// @return Vector of SlithIRFn structs containing the parsed data
pub fn parse_slithir_ssa(text: &str) -> Vec<SlithIRFn> {
    let mut current_contract = String::new();
    let mut current_fn = String::new();
    let mut buf = String::new();
    let mut out = Vec::new();

    // info!("text in parse_slithir {}", text.len());
    for line in text.lines() {
        // Parse contract lines (format: "Contract ContractName:")
        if line.starts_with("Contract ") {
            current_contract = line["Contract ".len()..].trim_end_matches(':').to_owned();
        }
        // Parse function lines (format: "\tFunction functionName:")
        else if line.starts_with("\tFunction ") {
            // Flush previous function data if we have any
            if !current_fn.is_empty() {
                let ir_content = replace_special_character(&buf);
                let ir_content_cleaned =
                    ir_content.replace("IRs:\n", "").replace("Expression:", "");
                out.push(SlithIRFn {
                    contract: current_contract.clone(),
                    function: current_fn.clone(),
                    ir: ir_content_cleaned,
                });
            }
            // Extract new function name and reset buffer
            current_fn = line.trim()[9..].trim_end_matches(':').to_owned();
            buf.clear();
        }
        // Parse IR lines (format: "\t\t<ir content>")
        else if line.starts_with("\t\t") {
            buf.push_str(line.trim_start());
            buf.push('\n');
        }
    }

    // Flush the last function after processing all lines
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

    // info!("functions => {:#?}", out);
    out
}

pub fn parse_slithir_contract_summary(text: &str) -> Vec<ContractSummary> {
    let mut current_name = String::new();
    let mut current_body = String::new();
    let mut out = Vec::<ContractSummary>::new();

    for line in text.lines() {
        // ── new contract header ─────────────────────────────────
        if let Some(rest) = line.strip_prefix("+ Contract ") {
            // flush the previous one (if any)
            if !current_name.is_empty() {
                out.push(ContractSummary {
                    contract: current_name.clone(),
                    content: current_body.trim_end().to_string(),
                });
                current_body.clear();
            }

            // take “IERC20 (Most derived contract)” → “IERC20”
            current_name = rest
                .split_whitespace() // split at first space
                .next()
                .unwrap_or_default()
                .to_string();
            continue;
        }

        // ── body line (ignore leading INFO: lines) ─────────────
        if current_name.is_empty() {
            continue; // still in preamble; skip
        }

        if !line.trim().is_empty() {
            current_body.push_str(line.trim_start());
            current_body.push('\n');
        }
        // we do NOT rely on blank line to flush; handled by header or final push
    }

    // push the last contract
    if !current_name.is_empty() {
        out.push(ContractSummary {
            contract: current_name,
            content: current_body.trim_end().to_string(),
        });
    }
    out
}

/// Parses the output of the Slither 'variable-order' printer into a vector of StorageVar structs.
///
/// This function processes the text output from Slither's variable-order printer,
/// which contains information about storage variables in Solidity contracts.
/// It extracts the contract name, variable name, and variable type for each storage variable.
///
/// @param text - The raw text output from the variable-order printer
/// @return Vector of StorageVar structs containing the parsed data
pub fn parse_storage(text: &str) -> Vec<StorageVar> {
    let mut current_contract = String::new();
    let mut vars = Vec::new();

    for line in text.lines() {
        // Parse contract lines (format: "Contract ContractName:")
        if line.starts_with("Contract") && line.ends_with(':') {
            current_contract = line["Contract".len()..]
                .trim_end_matches(':')
                .trim()
                .to_owned();
        }
        // Parse variable lines (format: "| <index> | <name> | <type> | <...> |")
        else if line.starts_with('|') && line.contains('|') {
            let cols: Vec<_> = line.split('|').map(|c| c.trim()).collect();
            // Check if this is a valid variable line (has enough columns and not a header)
            if cols.len() >= 3 && cols[1] != "Name" && !cols[1].is_empty() && !cols[2].is_empty() {
                // cols[1] is contract_name.function_name.  need to parse
                let (contract, function) = split_str_by_period(cols[1]).unwrap();
                vars.push(StorageVar {
                    contract,
                    name: function,
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

    // info!("storage => {:#?}", vars.len());
    vars
}
// splits "first.last" => ("first","last")
fn split_str_by_period(input: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = input.split('.').collect();
    if parts.len() == 2 {
        Some((parts[0].to_string(), parts[1].to_string()))
    } else {
        None // Invalid format
    }
}
/// Replces special characters in the SlithIR text with their ASCII equivalents.
///
/// This function replaces the Greek letter phi (ϕ) with the ASCII string "phi"
/// to ensure the text can be properly processed and displayed.
///
/// @param text - The text containing special characters
/// @return String with special characters replaced
pub fn replace_special_character(text: &str) -> String {
    // Replace the Greek letter phi (ϕ) with "phi"
    let cleaned_text = text.trim().replace("ϕ", "phi");

    cleaned_text
}
