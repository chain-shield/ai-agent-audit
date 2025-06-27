pub const PROMPT_3X_C: &str = r#"
Please Analyse the *entire* Solidity source below for 
*each category* of the security vulnerabilities listed below:

CATEGORIES  
1.  unchecked_return              // ignoring `call`, ERC-20 `transfer` boolean  
2.  storage_layout                // slot collisions, struct packing, uninitialized_storage  
3.  inheritance                   // bad overrides, diamond ambiguity  
4.  self_destruct                 // griefing / forced-ETH via `selfdestruct`  
5.  integer_math                  // rounding, precision div-by-zero  
6.  tx_origin                     // auth that trusts `tx.origin`  
7.  zero_code                     // constructor-phase contract bypasses  
8.  pragma                        // floating pragma, outdated compiler bugs  
9.  confidential_data             // private info leak via events / public vars  
10. short_address                 // calldata truncation on L1/L2 bridges  

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
