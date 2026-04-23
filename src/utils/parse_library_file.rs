use crate::enumerator::libraries::{LibFn, ParsedLibrary};
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibCall {
    pub library: String,       // e.g. "Transfers"
    pub canonical_sig: String, // e.g. "transferNAT(address,uint256)"
}

/// Parse a single SSA line. If it contains a LIBRARY_CALL, return (library, "name(types)").
pub fn parse_libcall_line(line: &str) -> Option<LibCall> {
    static RE: Lazy<Regex> = Lazy::new(|| {
        // Examples handled:
        // TMP_X = LIBRARY_CALL, dest:Transfers, function:Transfers.transferNAT(address,uint256), arguments:[...]
        // TMP_X = LIBRARY_CALL, dest:Lib, function:Lib.do(bytes32[],uint256), arguments:[...]
        Regex::new(
            r#"LIBRARY_CALL\s*,\s*dest:\s*([A-Za-z_][A-Za-z0-9_]*)\s*,\s*function:\s*([A-Za-z_][A-Za-z0-9_]*)\.([A-Za-z_][A-Za-z0-9_]*)\(([^)]*)\)"#
        ).unwrap()
    });

    let caps = RE.captures(line)?;
    let dest = caps.get(1)?.as_str();
    let _ = caps.get(2)?.as_str(); // library in the 'function:' field
    let fn_name = caps.get(3)?.as_str();
    let params = caps.get(4).map(|m| m.as_str()).unwrap_or("");

    // Prefer the library from `dest:`; if it mismatches the `function:` prefix, we still trust `dest`.
    let library = dest.to_string();
    let canonical_sig = format!("{}({})", fn_name, params);

    Some(LibCall {
        library,
        canonical_sig,
    })
}

/// Parse many lines and collect all LIBRARY_CALLs.
pub fn collect_library_calls(ir_text: &str) -> Vec<LibCall> {
    ir_text.lines().filter_map(parse_libcall_line).collect()
}

static FUNCTION_SIGNATURE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"function\s+([A-Za-z_]\w*)\s*\(([^)]*)\)").unwrap());

/* --------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_basic_libcall() {
        let line = "TMP_8735(bool) = LIBRARY_CALL, dest:Transfers, function:Transfers.transferNAT(address,uint256), arguments:['REF_5435', '_amount_1']";
        let got = parse_libcall_line(line).unwrap();
        assert_eq!(got.library, "Transfers");
        assert_eq!(got.canonical_sig, "transferNAT(address,uint256)");
    }

    #[test]
    fn ignores_non_libcall() {
        let line = "Modifier ReentrancyGuard.nonReentrant())";
        assert!(parse_libcall_line(line).is_none());
    }

    #[test]
    fn collects_multiple_libcalls() {
        let text = r#"
            TMP = LIBRARY_CALL, dest:LibA, function:LibA.foo(address), arguments:['x']
            Modifier ReentrancyGuard.nonReentrant())
            TMP2 = LIBRARY_CALL, dest:LibB, function:LibB.bar(bytes32[],uint256), arguments:['a','b']
        "#;
        let calls = collect_libcalls(text);
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0].library, "LibA");
        assert_eq!(calls[0].canonical_sig, "foo(address)");
        assert_eq!(calls[1].library, "LibB");
        assert_eq!(calls[1].canonical_sig, "bar(bytes32[],uint256)");
    }
}
-------------------------- */
pub fn parse_library_text(src: &str) -> Option<ParsedLibrary> {
    // Find `library <Name> {`
    let re_lib = Regex::new(r"(?m)^\s*library\s+([A-Za-z_]\w*)\s*\{").unwrap();
    let m = re_lib.captures(src)?;
    let lib_name = m.get(1).unwrap().as_str().to_string();
    let brace_pos = m.get(0).unwrap().end() - 1; // position of '{'

    let (block_start, block_end) = find_block_bounds(src, brace_pos)?;
    let block = &src[block_start..block_end];

    let fns = extract_functions(block);
    Some(ParsedLibrary {
        name: lib_name,
        functions: fns,
    })
}

/// Extract each `function ... { ... }` from the library block.
/// For each function, build canonical signature "name(type1,type2)" and the full body text.
fn extract_functions(block: &str) -> Vec<LibFn> {
    let bytes = block.as_bytes();
    let mut i = 0usize;
    let mut out = Vec::new();

    while i < bytes.len() {
        i = skip_ws_comments_strings(block, i);
        if i >= bytes.len() {
            break;
        }

        if starts_with_kw(block, i, "function") && is_word_boundary(block, i + "function".len()) {
            let fn_start = i;
            i += "function".len();

            // Header: from after "function" up to '{' or ';' at depth 0 (paren depth tracked)
            i = skip_ws_comments_strings(block, i);
            let mut j = i;
            let mut paren = 0i32;
            let mut header_end: Option<usize> = None;
            while j < bytes.len() {
                j = skip_ws_comments_strings(block, j);
                if j >= bytes.len() {
                    break;
                }
                match bytes[j] {
                    b'(' => {
                        paren += 1;
                        j += 1;
                    }
                    b')' => {
                        paren -= 1;
                        j += 1;
                    }
                    b'{' if paren == 0 => {
                        header_end = Some(j);
                        break;
                    }
                    b';' if paren == 0 => {
                        header_end = Some(j);
                        break;
                    } // no body
                    _ => {
                        j += 1;
                    }
                }
            }
            let h_end = match header_end {
                Some(x) => x,
                None => break,
            };
            let header = block[fn_start..h_end].to_string();

            // Name + params
            if let Some(c) = FUNCTION_SIGNATURE_RE.captures(&header) {
                let fn_name = c.get(1).unwrap().as_str();
                let params_raw = c.get(2).map(|m| m.as_str()).unwrap_or("");
                let param_types = canonicalize_param_list(params_raw);
                let canonical_sig = format!("{}({})", fn_name, param_types);

                // Has body or not?
                let next = skip_ws_comments_strings(block, h_end);
                if next < bytes.len() && bytes[next] == b'{' {
                    if let Some((_b_start, b_end)) = find_block_bounds(block, next) {
                        // full body includes declaration + braces content
                        let full = block[fn_start..b_end].to_string();
                        out.push(LibFn {
                            canonical_sig,
                            full_body: full,
                        });
                        i = b_end + 1;
                        continue;
                    } else {
                        break;
                    }
                } else {
                    // function without body -> skip (you said you want full body with braces)
                    i = next + 1;
                    continue;
                }
            } else {
                // Fallback: skip until next char
                i = h_end + 1;
                continue;
            }
        } else {
            i += 1;
        }
    }

    out
}

/// Turn a Solidity parameter list into canonical types: "address,uint256,bytes32[]"
fn canonicalize_param_list(params: &str) -> String {
    // Split on top-level commas (no nested tuples support here by design).
    // For typical library params this is sufficient.
    let parts = split_top_level_commas(params);
    let mut types = Vec::with_capacity(parts.len());
    for p in parts {
        let ty = canonicalize_one_param(p);
        if !ty.is_empty() {
            types.push(ty);
        }
    }
    types.join(",")
}

/// Heuristic: remove storage/location keywords and parameter name; keep the type (with array suffixes).
fn canonicalize_one_param(param: &str) -> String {
    let s = param.trim().to_string();
    if s.is_empty() {
        return s;
    }

    // Remove storage/location/type modifiers that shouldn't be in the canonical signature
    // (order-insensitive; do simple whitespace-based removal)
    const STRIP_WORDS: &[&str] = &[
        "memory",
        "calldata",
        "storage",
        "payable",
        "view",
        "pure",
        "internal",
        "external",
        "virtual",
        "override",
        "indexed",
        "immutable",
    ];
    let mut tokens: Vec<String> = s.split_whitespace().map(|t| t.to_string()).collect();
    tokens.retain(|t| !STRIP_WORDS.contains(&t.as_str()));

    if tokens.is_empty() {
        return String::new();
    }

    // Rejoin to handle array suffixes like "bytes32[][3]"
    let mut joined = tokens.join(" ");

    // If it looks like "<type> <name>", drop the last token as a name.
    // Heuristic: if there's a space and the last token is a plain identifier (no []), treat it as a name.
    if let Some(idx) = joined.rfind(' ') {
        let last = joined[idx + 1..].trim();
        let before = joined[..idx].trim();
        if is_identifier(last) && !last.ends_with(']') && !before.is_empty() {
            joined = before.to_string();
        }
    }

    // Remove spaces inside the type like "uint256 [ ]" -> "uint256[]"
    joined.retain(|c| c != ' ');

    joined
}

fn split_top_level_commas(s: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0usize;
    let mut i = 0usize;
    let b = s.as_bytes();
    let mut depth = 0i32;

    while i < b.len() {
        match b[i] {
            b'(' => {
                depth += 1;
                i += 1;
            }
            b')' => {
                depth -= 1;
                i += 1;
            }
            b',' if depth == 0 => {
                parts.push(s[start..i].trim());
                i += 1;
                start = i;
            }
            b'"' | b'\'' => {
                // skip string literal
                let quote = b[i];
                i += 1;
                while i < b.len() {
                    if b[i] == b'\\' {
                        i += 2;
                        continue;
                    }
                    if b[i] == quote {
                        i += 1;
                        break;
                    }
                    i += 1;
                }
            }
            _ => {
                i += 1;
            }
        }
    }
    if start < s.len() {
        parts.push(s[start..].trim());
    }
    parts
}

fn is_identifier(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c == '_' || c.is_ascii_alphabetic() => (),
        _ => return false,
    }
    chars.all(|c| c == '_' || c.is_ascii_alphanumeric())
}

/// Find matching `{ ... }` bounds starting at the position of `{`.
/// Returns (start_index, end_index_exclusive_of_closing_brace).
fn find_block_bounds(text: &str, open_brace_pos: usize) -> Option<(usize, usize)> {
    let b = text.as_bytes();
    if *b.get(open_brace_pos)? != b'{' {
        return None;
    }
    let mut i = open_brace_pos;
    let mut depth = 0i32;

    while i < b.len() {
        i = skip_ws_comments_strings(text, i);
        if i >= b.len() {
            break;
        }
        match b[i] {
            b'{' => {
                depth += 1;
                i += 1;
            }
            b'}' => {
                depth -= 1;
                i += 1;
                if depth == 0 {
                    return Some((open_brace_pos, i));
                }
            }
            _ => {
                i += 1;
            }
        }
    }
    None
}

/// Advance index past whitespace/comments/strings. Returns new index (may be equal to `i`).
fn skip_ws_comments_strings(s: &str, mut i: usize) -> usize {
    let b = s.as_bytes();
    let n = b.len();

    loop {
        // whitespace
        while i < n && (b[i] as char).is_whitespace() {
            i += 1;
        }
        if i >= n {
            return i;
        }

        // line comment //
        if i + 1 < n && b[i] == b'/' && b[i + 1] == b'/' {
            i += 2;
            while i < n && b[i] != b'\n' {
                i += 1;
            }
            continue;
        }

        // block comment /* ... */
        if i + 1 < n && b[i] == b'/' && b[i + 1] == b'*' {
            i += 2;
            while i + 1 < n && !(b[i] == b'*' && b[i + 1] == b'/') {
                i += 1;
            }
            i = (i + 2).min(n);
            continue;
        }

        // string literal "..."
        if i < n && b[i] == b'"' {
            i += 1;
            while i < n {
                if b[i] == b'\\' {
                    i += 2;
                    continue;
                }
                if b[i] == b'"' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            continue;
        }

        // string literal '...'
        if i < n && b[i] == b'\'' {
            i += 1;
            while i < n {
                if b[i] == b'\\' {
                    i += 2;
                    continue;
                }
                if b[i] == b'\'' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            continue;
        }

        break;
    }
    i
}

fn starts_with_kw(text: &str, pos: usize, kw: &str) -> bool {
    text.as_bytes()
        .get(pos..pos + kw.len())
        .map(|s| s == kw.as_bytes())
        .unwrap_or(false)
}

fn is_word_boundary(text: &str, pos: usize) -> bool {
    match text.as_bytes().get(pos) {
        None => true,
        Some(&ch) => !((ch as char).is_ascii_alphanumeric() || ch == b'_'),
    }
}

/* ---------------------------
Example usage:

let src = r#"
library Transfers {
    using X for uint256;

    function transferNAT(address to, uint256 amt) internal returns (bool) {
        if (amt == 0) { return false; }
        return true;
    }

    function helper(uint a) internal pure returns (uint) { return a + 1; }
}
"#;
let parsed = parse_library_text(src).unwrap();
assert_eq!(parsed.name, "Transfers");
assert_eq!(parsed.functions[0].canonical_sig, "transferNAT(address,uint256)");
assert!(parsed.functions[0].full_body.starts_with("function transferNAT("));
--------------------------- */

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    use crate::test_support::solidity_mocks::{CONVERSION_SOL, TRANSFERS_SOL};

    const _CONVERSION_SOL_INLINE: &str = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {IPriceReader} from "../../ftso/interfaces/IPriceReader.sol";
import {SafePct} from "../../utils/library/SafePct.sol";
import {AssetManagerState} from "./data/AssetManagerState.sol";
import {Globals} from "./Globals.sol";
import {CollateralTypeInt} from "./data/CollateralTypeInt.sol";
import {AssetManagerSettings} from "../../userInterfaces/data/AssetManagerSettings.sol";


library Conversion {
    using SafePct for uint256;

    uint256 internal constant AMG_TOKEN_WEI_PRICE_SCALE_EXP = 9;
    uint256 internal constant AMG_TOKEN_WEI_PRICE_SCALE = 10 ** AMG_TOKEN_WEI_PRICE_SCALE_EXP;
    uint256 internal constant NAT_WEI = 1e18;
    uint256 internal constant GWEI = 1e9;

    function currentAmgPriceInTokenWei(
        uint256 _tokenType
    )
        internal view
        returns (uint256 _price)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        (_price,,) = currentAmgPriceInTokenWeiWithTs(state.collateralTokens[_tokenType], false);
    }

    function currentAmgPriceInTokenWei(
        CollateralTypeInt.Data storage _token
    )
        internal view
        returns (uint256 _price)
    {
        (_price,,) = currentAmgPriceInTokenWeiWithTs(_token, false);
    }

    function currentAmgPriceInTokenWeiWithTrusted(
        CollateralTypeInt.Data storage _token
    )
        internal view
        returns (uint256 _ftsoPrice, uint256 _trustedPrice)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        (uint256 ftsoPrice, uint256 assetTimestamp, uint256 tokenTimestamp) =
            currentAmgPriceInTokenWeiWithTs(_token, false);
        (uint256 trustedPrice, uint256 assetTimestampTrusted, uint256 tokenTimestampTrusted) =
            currentAmgPriceInTokenWeiWithTs(_token, true);
        bool trustedPriceFresh = tokenTimestampTrusted + settings.maxTrustedPriceAgeSeconds >= tokenTimestamp
                && assetTimestampTrusted + settings.maxTrustedPriceAgeSeconds >= assetTimestamp;
        _ftsoPrice = ftsoPrice;
        _trustedPrice = trustedPriceFresh ? trustedPrice : ftsoPrice;
    }

    function convertAmgToUBA(
        uint64 _valueAMG
    )
        internal view
        returns (uint256)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        return uint256(_valueAMG) * settings.assetMintingGranularityUBA;
    }

    function convertUBAToAmg(
        uint256 _valueUBA
    )
        internal view
        returns (uint64)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        return SafeCast.toUint64(_valueUBA / settings.assetMintingGranularityUBA);
    }

    function roundUBAToAmg(
        uint256 _valueUBA
    )
        internal view
        returns (uint256)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        return _valueUBA - (_valueUBA % settings.assetMintingGranularityUBA);
    }

    function convertLotsToAMG(
        uint256 _lots
    )
        internal view
        returns (uint64)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        return SafeCast.toUint64(_lots * settings.lotSizeAMG);
    }

    function convertLotsToUBA(
        uint256 _lots
    )
        internal view
        returns (uint256)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        return _lots * settings.lotSizeAMG * settings.assetMintingGranularityUBA;
    }

    function convert(
        uint256 _amount,
        CollateralTypeInt.Data storage _fromToken,
        CollateralTypeInt.Data storage _toToken
    )
        internal view
        returns (uint256)
    {
        uint256 priceMul = currentAmgPriceInTokenWei(_toToken);
        uint256 priceDiv = currentAmgPriceInTokenWei(_fromToken);
        return _amount.mulDiv(priceMul, priceDiv);
    }

    function convertFromUSD5(
        uint256 _amountUSD5,
        CollateralTypeInt.Data storage _token
    )
        internal view
        returns (uint256)
    {
        if (bytes(_token.tokenFtsoSymbol).length == 0) {
            return _amountUSD5;
        }
        (uint256 tokenPrice,, uint256 tokenFtsoDec) = readFtsoPrice(_token.tokenFtsoSymbol, false);
        uint256 expPlus = _token.decimals + tokenFtsoDec - 5;
        return _amountUSD5.mulDiv(10 ** expPlus, tokenPrice);
    }

    function currentAmgPriceInTokenWeiWithTs(
        CollateralTypeInt.Data storage _token,
        bool _fromTrustedProviders
    )
        internal view
        returns (uint256 /*_price*/, uint256 /*_assetTimestamp*/, uint256 /*_tokenTimestamp*/)
    {
        (uint256 assetPrice, uint256 assetTs, uint256 assetFtsoDec) =
            readFtsoPrice(_token.assetFtsoSymbol, _fromTrustedProviders);
        if (_token.directPricePair) {
            uint256 price = calcAmgToTokenWeiPrice(_token.decimals, 1, 0, assetPrice, assetFtsoDec);
            return (price, assetTs, assetTs);
        } else {
            (uint256 tokenPrice, uint256 tokenTs, uint256 tokenFtsoDec) =
                readFtsoPrice(_token.tokenFtsoSymbol, _fromTrustedProviders);
            uint256 price =
                calcAmgToTokenWeiPrice(_token.decimals, tokenPrice, tokenFtsoDec, assetPrice, assetFtsoDec);
            return (price, assetTs, tokenTs);
        }
    }

    function readFtsoPrice(string memory _symbol, bool _fromTrustedProviders)
        internal view
        returns (uint256, uint256, uint256)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        IPriceReader priceReader = IPriceReader(settings.priceReader);
        if (_fromTrustedProviders) {
            return priceReader.getPriceFromTrustedProviders(_symbol);
        } else {
            return priceReader.getPrice(_symbol);
        }
    }

    function calcAmgToTokenWeiPrice(
        uint256 _tokenDecimals,
        uint256 _tokenPrice,
        uint256 _tokenFtsoDecimals,
        uint256 _assetPrice,
        uint256 _assetFtsoDecimals
    )
        internal view
        returns (uint256)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        uint256 expPlus = _tokenDecimals + _tokenFtsoDecimals + AMG_TOKEN_WEI_PRICE_SCALE_EXP;
        uint256 expMinus = settings.assetMintingDecimals + _assetFtsoDecimals;
        assert(expPlus >= expMinus);
        return _assetPrice.mulDiv(10 ** (expPlus - expMinus), _tokenPrice);
    }

    function convertAmgToTokenWei(uint256 _valueAMG, uint256 _amgToTokenWeiPrice) internal pure returns (uint256) {
        return _valueAMG.mulDiv(_amgToTokenWeiPrice, AMG_TOKEN_WEI_PRICE_SCALE);
    }

    function convertTokenWeiToAMG(uint256 _valueNATWei, uint256 _amgToTokenWeiPrice) internal pure returns (uint256) {
        return _valueNATWei.mulDiv(AMG_TOKEN_WEI_PRICE_SCALE, _amgToTokenWeiPrice);
    }
}
"#;

    const _TRANSFERS_SOL_INLINE: &str = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IWNat} from "../../flareSmartContracts/interfaces/IWNat.sol";
import {Reentrancy} from "../../openzeppelin/library/Reentrancy.sol";


library Transfers {
    uint256 internal constant TRANSFER_GAS_ALLOWANCE = 100_000;

    error TransferFailed();

    modifier requireReentrancyGuard {
        Reentrancy.requireReentrancyGuard();
        _;
    }

    function transferNAT(address payable _recipient, uint256 _amount)
        internal
        requireReentrancyGuard
    {
        if (_amount > 0) {
            (bool success, ) = _recipient.call{value: _amount, gas: TRANSFER_GAS_ALLOWANCE}("");
            require(success, TransferFailed());
        }
    }

    function depositWNat(IWNat _wNat, address _recipient, uint256 _amount)
        internal
    {
        if (_amount > 0) {
            _wNat.depositTo{value: _amount}(_recipient);
        }
    }
}
"#;

    #[test]
    fn parses_conversion_library_signatures() {
        let parsed = parse_library_text(CONVERSION_SOL).expect("parse Conversion");
        println!("[test] Conversion library name: {}", parsed.name);
        println!(
            "[test] Conversion functions parsed: {}",
            parsed.functions.len()
        );
        assert_eq!(parsed.name, "Conversion");
        let sigs: HashSet<&str> = parsed
            .functions
            .iter()
            .map(|f| f.canonical_sig.as_str())
            .collect();
        println!("[test] Conversion sigs: {:?}", sigs);

        assert!(sigs.contains("currentAmgPriceInTokenWei(uint256)"));
        assert!(sigs.contains("currentAmgPriceInTokenWei(CollateralTypeInt.Data)"));
        assert!(sigs.contains("convert(uint256,CollateralTypeInt.Data,CollateralTypeInt.Data)"));
        assert!(sigs.contains("convertFromUSD5(uint256,CollateralTypeInt.Data)"));
        assert!(sigs.contains("calcAmgToTokenWeiPrice(uint256,uint256,uint256,uint256,uint256)"));
        assert!(sigs.contains("convertAmgToTokenWei(uint256,uint256)"));
        assert!(sigs.contains("convertTokenWeiToAMG(uint256,uint256)"));
        assert!(sigs.contains("readFtsoPrice(string,bool)"));

        // All parsed functions should start with 'function ' and contain the closing brace
        for f in &parsed.functions {
            assert!(f.full_body.starts_with("function "));
            assert!(f.full_body.ends_with('}'));

            // Check header → canonical signature round-trip
            let caps = FUNCTION_SIGNATURE_RE
                .captures(&f.full_body)
                .expect("header match");
            let name = caps.get(1).unwrap().as_str();
            let params_raw = caps.get(2).map(|m| m.as_str()).unwrap_or("");
            let canon_params = canonicalize_param_list(params_raw);
            let recomposed = format!("{}({})", name, canon_params);
            assert_eq!(
                recomposed, f.canonical_sig,
                "canonical sig mismatch for {}",
                name
            );

            // Ensure braces match and the closing brace is at the very end
            let open_idx = f.full_body.find('{').expect("open brace");
            let (_start, end_idx) =
                find_block_bounds(&f.full_body, open_idx).expect("block bounds");
            assert_eq!(
                end_idx,
                f.full_body.len(),
                "block should end at full_body end"
            );

            // Sanity: brace counts
            let opens = f.full_body.chars().filter(|&c| c == '{').count();
            let closes = f.full_body.chars().filter(|&c| c == '}').count();
            assert_eq!(opens, closes, "unbalanced braces in {}", name);
        }

        // Function-specific body content checks for Transfers
        for f in &parsed.functions {
            match f.canonical_sig.as_str() {
                "transferNAT(address,uint256)" => {
                    assert!(f.full_body.contains("_recipient.call"));
                    assert!(f.full_body.contains("TRANSFER_GAS_ALLOWANCE"));
                }
                "depositWNat(IWNat,address,uint256)" => {
                    assert!(f.full_body.contains("_wNat.depositTo"));
                }
                _ => {}
            }
        }
    }

    #[test]
    fn parses_transfers_library_signatures() {
        let parsed = parse_library_text(TRANSFERS_SOL).expect("parse Transfers");
        println!("[test] Transfers library name: {}", parsed.name);
        println!(
            "[test] Transfers functions parsed: {}",
            parsed.functions.len()
        );
        assert_eq!(parsed.name, "Transfers");
        let sigs: HashSet<&str> = parsed
            .functions
            .iter()
            .map(|f| f.canonical_sig.as_str())
            .collect();
        println!("[test] Transfers sigs: {:?}", sigs);

        // payable should be stripped from canonical param types
        assert!(sigs.contains("transferNAT(address,uint256)"));
        assert!(sigs.contains("depositWNat(IWNat,address,uint256)"));

        for f in &parsed.functions {
            assert!(f.full_body.starts_with("function "));
            assert!(f.full_body.ends_with('}'));

            // Round-trip canonical sig check
            let caps = FUNCTION_SIGNATURE_RE
                .captures(&f.full_body)
                .expect("header match");
            let name = caps.get(1).unwrap().as_str();
            let params_raw = caps.get(2).map(|m| m.as_str()).unwrap_or("");
            let canon_params = canonicalize_param_list(params_raw);
            let recomposed = format!("{}({})", name, canon_params);
            assert_eq!(
                recomposed, f.canonical_sig,
                "canonical sig mismatch for {}",
                name
            );

            // Ensure braces match and the closing brace is at the very end
            let open_idx = f.full_body.find('{').expect("open brace");
            let (_start, end_idx) =
                find_block_bounds(&f.full_body, open_idx).expect("block bounds");
            assert_eq!(
                end_idx,
                f.full_body.len(),
                "block should end at full_body end"
            );

            // Sanity: brace counts
            let opens = f.full_body.chars().filter(|&c| c == '{').count();
            let closes = f.full_body.chars().filter(|&c| c == '}').count();
            assert_eq!(opens, closes, "unbalanced braces in {}", name);
        }
    }

    #[test]
    fn parses_library_call_lines() {
        let line = "TMP_8735(bool) = LIBRARY_CALL, dest:Transfers, function:Transfers.transferNAT(address,uint256), arguments:['REF_5435', '_amount_1']";
        let got = parse_libcall_line(line).expect("libcall");
        println!("[test] Parsed libcall: {:?}", got);
        assert_eq!(got.library, "Transfers");
        assert_eq!(got.canonical_sig, "transferNAT(address,uint256)");

        let line2 = "TMP_X = LIBRARY_CALL, dest:Lib, function:Lib.do(bytes32[],uint256), arguments:['a','b']";
        let got2 = parse_libcall_line(line2).expect("libcall2");
        println!("[test] Parsed libcall2: {:?}", got2);
        assert_eq!(got2.library, "Lib");
        assert_eq!(got2.canonical_sig, "do(bytes32[],uint256)");
    }
}
