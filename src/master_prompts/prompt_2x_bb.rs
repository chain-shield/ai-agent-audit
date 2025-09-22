pub const PROMPT_2X_BB: &str = r#"
Please Analyse the *entire* Solidity source below for 
*each category* of the security vulnerabilities listed below:

CATEGORIES  
1.  tx_origin                     // auth that trusts `tx.origin`  
2.  array_limits                  // OOB reads/writes, dynamic-array gas bombs  
3.  pragma                        // floating pragma, outdated compiler bugs  
4.  inheritance                   // bad overrides, diamond ambiguity  
5.  integer_math                  // rounding, precision div-by-zero  
6.  confidential_data             // private info leak via events / public vars  
7.  default_visibility            // funcs / vars defaulting to `public`  
8.  pausable_emergency_stop       // missing pause guards or bypasses  
9.  timestamp_dependent_logic     // miner-controlled `block.timestamp` / `number`  
10. unchecked_return              // ignoring `call`, ERC-20 `transfer` boolean  
11. event_consistency             // critical state changes not emitted / mis-ordered  
12. short_address                 // calldata truncation on L1/L2 bridges  
13. gas_grief_block_limit         // user-scaling loops, heavy SSTORE in hot paths  

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
- **ANALYZE ALL 13 CATEGORIES** - No exceptions
- **Be thorough** - Don't rush through categories
- **Be precise** - Use exact classification criteria
- **Scope** - if scope is provided below, then only report vulnerability that are in scope
- **Think like an attacker** - Consider how each vulnerability could be exploited
- **Provide only the JSON** - No additional commentary in final output
"#;
