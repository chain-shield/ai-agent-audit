/// Master security analysis prompt covering all vulnerability categories.
///
/// This comprehensive prompt instructs AI agents to analyze smart contracts
/// across 19 different security vulnerability categories, providing systematic
/// coverage of common and advanced attack vectors.

pub const MASTER_SECURITY_PROMPT: &str = r#"
Please Analyse the *entire* Solidity source below for 
*each category* of the security vulnerabilities listed below:

CATEGORIES  
1.  access_control                // missing / mis-scoped auth, ownership loss  
2.  array_limits                  // OOB reads/writes, dynamic-array gas bombs  
3.  confidential_data             // private info leak via events / public vars  
4.  default_visibility            // funcs / vars defaulting to `public`  
5.  dos                           // gas exhaustion, revert griefing, block gas limit  
6.  inheritance                   // bad overrides, diamond ambiguity  
7.  integer_math                  // rounding, precision div-by-zero  
8.  oracle                        // price-feed spoofing, stale data, missing sanity checks  
9.  pragma                        // floating pragma, outdated compiler bugs  
10. randomness                    // predictable entropy, miner influence  
11. reentrancy                    // state update after external call, cross-function  
12. replay_attack                 // sig replay, chain-ID mix-ups  
13. self_destruct                 // griefing / forced-ETH via `selfdestruct`  
14. short_address                 // calldata truncation on L1/L2 bridges  
15. storage_layout                // slot collisions, struct packing, uninitialized_storage  
16. tx_origin                     // auth that trusts `tx.origin`  
17. unchecked_return              // ignoring `call`, ERC-20 `transfer` boolean  
18. unexpected_eth                // Ether stuck / overly strict balance checks  
19. zero_code                     // constructor-phase contract bypasses  
20. frontrun_mev                  // front-run / sandwich / back-run / latency arbitrage vectors  
21. upgradeability_initializer_safety // proxy init gaps, `initializer()` abuse  
22. pausable_emergency_stop       // missing pause guards or bypasses  
23. timestamp_dependent_logic     // miner-controlled `block.timestamp` / `number`  
24. flash_loan_economic_manipulation // state checked & used within same tx  
25. delegatecall_low_level_ops    // unsafe `delegatecall`, inline assembly scribbles  
26. signature_malleability        // EIP-2 `s` checks, EIP-712 domain separation  
27. event_consistency             // critical state changes not emitted / mis-ordered  
28. gas_grief_block_limit         // user-scaling loops, heavy SSTORE in hot paths  
29. integer_overflow              // overflow / underflow, div-by-zero  

## 🔍 ANALYSIS REQUIREMENTS

### DEPTH OF ANALYSIS
- **Read every line** of the contract code
- **Consider edge cases** and attack vectors for each category
- **Look for subtle vulnerabilities** that may not be immediately obvious
- **Consider interactions** between different parts of the contract

### CLASSIFICATION CRITERIA
For **each category** decide one of:
  • VIOLATION – bug exists in this contract
  • SAFE      – relevant but properly handled
  • N/A       – category not applicable to this code

### REASONING PROCESS
Before providing your final JSON output, you must:
1. **Silently analyze each category** in order (1-20)
2. **Consider all relevant code sections** for each category
3. **Make evidence-based classifications** 
4. **Double-check** that no category was skipped

## ⚠️ CRITICAL REMINDERS
- **ANALYZE ALL 20 CATEGORIES** - No exceptions
- **Be thorough** - Don't rush through categories
- **Be precise** - Use exact classification criteria
- **Think like an attacker** - Consider how each vulnerability could be exploited
- **Provide only the JSON** - No additional commentary in final output
"#;
