pub const PROMPT_2X_A: &str = r#"
Please Analyse the *entire* Solidity source below for 
*each category* of the security vulnerabilities listed below:

CATEGORIES  
1.  access_control                // missing / mis-scoped auth, ownership loss  
2.  array_limits                  // OOB reads/writes, dynamic-array gas bombs  
3.  dos                           // gas exhaustion, revert griefing, block gas limit  
4.  inheritance                   // bad overrides, diamond ambiguity  
5.  integer_math                  // rounding, precision div-by-zero  
6.  integer_overflow              // overflow / underflow, div-by-zero  
7.  frontrun_mev                  // front-run / sandwich / back-run / latency arbitrage vectors  
8.  oracle                        // price-feed spoofing, stale data, missing sanity checks  
9.  randomness                    // predictable entropy, miner influence  
10. reentrancy                    // state update after external call, cross-function  
11. unchecked_return              // ignoring `call`, ERC-20 `transfer` boolean  
12. unexpected_eth                // Ether stuck / overly strict balance checks  
13. storage_layout                // slot collisions, struct packing, uninitialized_storage  
14. self_destruct                 // griefing / forced-ETH via `selfdestruct`  

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
