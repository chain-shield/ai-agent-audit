/// This module handles the enrichment of smart contract data using Slither analysis.
/// It provides functionality to extract intermediate representation (IR) and storage information
/// from Solidity contracts, and to build Forge projects.
use super::slither_ffi::{dump_ir_and_storage, SlithIRFn, StorageVar};
use anyhow::Result;
use std::path::Path;
use std::process::Command;

/// Contains the enriched data extracted from Solidity contracts.
/// This includes the intermediate representation (IR) of functions and storage variable information.
pub struct Enriched {
    /// Vector of SlithIR function representations
    pub ir: Vec<SlithIRFn>,
    /// Vector of storage variable information
    pub storage: Vec<StorageVar>,
}

/// Extracts IR and storage information from Solidity contracts using Slither.
///
/// This function uses the Slither static analysis framework to analyze Solidity contracts
/// and extract their intermediate representation and storage information.
///
/// @param repo_root - Path to the repository root containing Solidity contracts
/// @return Result containing the enriched data
pub fn enrich_with_slither(repo_root: &Path) -> Result<Enriched> {
    let (ir, storage) = dump_ir_and_storage(repo_root)?;
    Ok(Enriched { ir, storage })
}

/// Builds a Forge project by running `forge install` and `forge build`.
///
/// This function executes the necessary Forge commands to install dependencies
/// and build the Solidity contracts in the repository.
///
/// @param repo_root - Path to the repository root containing the Forge project
/// @return Result indicating success or failure
pub fn forge_build(repo_root: &Path) -> Result<()> {
    // Step 1: Run `forge install` to pull in remappings
    let install_status = Command::new("forge")
        .current_dir(repo_root)
        .arg("install")
        .status()?;
    anyhow::ensure!(install_status.success(), "`forge install` failed");

    // Step 2: Now build
    let build_status = Command::new("forge")
        .current_dir(repo_root)
        .args(["-q", "build", "--build-info"])
        .status()?;
    anyhow::ensure!(build_status.success(), "`forge build` failed");
    Ok(())
}

// After building, you can read ./out/*.json artefacts for ABIs, bytecode sizes, etc.
