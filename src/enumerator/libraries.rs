use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use crate::utils::parse_library_file::LibCall;

#[derive(Debug, Clone)]
pub struct LibFn {
    /// e.g. "transferNAT(address,uint256)"
    pub canonical_sig: String,
    /// Full text from `function ... {` up to the matching `}` (inclusive)
    pub full_body: String,
}

#[derive(Debug, Clone)]
pub struct ParsedLibrary {
    pub name: String,
    pub functions: Vec<LibFn>,
}

/// Global metadata context shared across all AI agents
static LIBRARY_FN_TO_CODE: Lazy<Arc<Mutex<HashMap<String, String>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::<String, String>::new())));

pub async fn generate_library_to_code_mapping(library_fns: &[ParsedLibrary]) -> anyhow::Result<()> {
    let map = Arc::clone(&LIBRARY_FN_TO_CODE);
    let mut library_fn_map = map.lock().await;

    for lib in library_fns {
        for func in &lib.functions {
            let key = format!("{}.{}", lib.name, func.canonical_sig);
            library_fn_map.insert(key, func.full_body.clone());
        }
    }

    Ok(())
}

pub async fn get_library_fn_code(lib: &str, fn_sig: &str) -> Option<String> {
    let map = Arc::clone(&LIBRARY_FN_TO_CODE);
    let library_fn_map = map.lock().await;
    let key = format!("{}.{}", lib, fn_sig);

    library_fn_map.get(&key).cloned()
}

pub async fn get_library_code_for_library_calls(
    library_calls: &[LibCall],
) -> HashMap<String, String> {
    let mut lib_fn_to_code_map = HashMap::<String, String>::new();

    for library_call in library_calls {
        let key = format!("{}.{}", library_call.library, library_call.canonical_sig);
        if let Some(code) =
            get_library_fn_code(&library_call.library, &library_call.canonical_sig).await
        {
            lib_fn_to_code_map.insert(key, code);
        };
    }
    lib_fn_to_code_map
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enumerator::libraries::{
        generate_library_to_code_mapping, get_library_code_for_library_calls,
    };
    use crate::test_support::solidity_mocks::{CONVERSION_SOL, TRANSFERS_SOL};
    use crate::utils::parse_library_file::{LibCall, parse_library_text};
    use std::collections::HashSet;

    fn parse_mocks() -> Vec<ParsedLibrary> {
        let mut libs = Vec::new();
        let transfers = parse_library_text(TRANSFERS_SOL).expect("parse Transfers");
        let conversion = parse_library_text(CONVERSION_SOL).expect("parse Conversion");
        assert_eq!(transfers.name, "Transfers");
        assert_eq!(conversion.name, "Conversion");
        assert_eq!(transfers.functions.len(), 2);
        assert_eq!(conversion.functions.len(), 15);
        libs.push(transfers);
        libs.push(conversion);
        libs
    }

    #[tokio::test(flavor = "current_thread")]
    async fn mapping_from_parser_covers_all_sigs() {
        // Arrange: parse full mocks and seed the global mapping
        let libs = parse_mocks();

        // Build expected signature sets
        let expected_transfers: HashSet<&str> = [
            "transferNAT(address,uint256)",
            "depositWNat(IWNat,address,uint256)",
        ]
        .into_iter()
        .collect();
        let expected_conversion: HashSet<&str> = [
            "convertFromUSD5(uint256,CollateralTypeInt.Data)",
            "convertUBAToAmg(uint256)",
            "calcAmgToTokenWeiPrice(uint256,uint256,uint256,uint256,uint256)",
            "convertTokenWeiToAMG(uint256,uint256)",
            "convertLotsToUBA(uint256)",
            "readFtsoPrice(string,bool)",
            "convertAmgToTokenWei(uint256,uint256)",
            "currentAmgPriceInTokenWei(CollateralTypeInt.Data)",
            "convertLotsToAMG(uint256)",
            "convertAmgToUBA(uint64)",
            "currentAmgPriceInTokenWeiWithTrusted(CollateralTypeInt.Data)",
            "convert(uint256,CollateralTypeInt.Data,CollateralTypeInt.Data)",
            "roundUBAToAmg(uint256)",
            "currentAmgPriceInTokenWeiWithTs(CollateralTypeInt.Data,bool)",
            "currentAmgPriceInTokenWei(uint256)",
        ]
        .into_iter()
        .collect();

        let transfers_sigs: HashSet<String> = libs[0]
            .functions
            .iter()
            .map(|f| f.canonical_sig.clone())
            .collect();
        let conversion_sigs: HashSet<String> = libs[1]
            .functions
            .iter()
            .map(|f| f.canonical_sig.clone())
            .collect();

        assert_eq!(transfers_sigs.len(), expected_transfers.len());
        assert_eq!(conversion_sigs.len(), expected_conversion.len());
        assert_eq!(
            transfers_sigs,
            expected_transfers
                .into_iter()
                .map(|s| s.to_string())
                .collect()
        );
        assert_eq!(
            conversion_sigs,
            expected_conversion
                .into_iter()
                .map(|s| s.to_string())
                .collect()
        );

        // Seed mapping
        generate_library_to_code_mapping(&libs).await.unwrap();

        // Build calls for all functions
        let mut calls: Vec<LibCall> = Vec::new();
        for lib in &libs {
            for f in &lib.functions {
                calls.push(LibCall {
                    library: lib.name.clone(),
                    canonical_sig: f.canonical_sig.clone(),
                });
            }
        }

        // Act: retrieve mapping for all calls at once
        let map = get_library_code_for_library_calls(&calls).await;

        // Assert: all keys present and code looks correct
        assert_eq!(map.len(), calls.len());
        let mut missing: Vec<String> = Vec::new();
        for c in &calls {
            let key = format!("{}.{}", c.library, c.canonical_sig);
            match map.get(&key) {
                Some(code) => {
                    // Validate function header appears
                    let name = c.canonical_sig.split('(').next().unwrap_or("");
                    assert!(code.contains("function "));
                    assert!(code.contains(&format!("{}(", name)));
                }
                None => missing.push(key),
            }
        }
        assert!(missing.is_empty(), "missing entries: {:?}", missing);
    }
}
