pub const PROMPT_2X_AA: &str = r#"
Please Analyse the *entire* Solidity source below for 
*each category* of the security vulnerabilities listed below:

CATEGORIES  
1.  access_control                // missing / mis-scoped auth, ownership loss  
2.  dos                           // gas exhaustion, revert griefing, block gas limit  
3.  integer_overflow              // overflow / underflow, div-by-zero  
4.  signature_malleability        // EIP-2 `s` checks, EIP-712 domain separation  
5.  unexpected_eth                // Ether stuck / overly strict balance checks  
6.  storage_layout                // slot collisions, struct packing, uninitialized_storage  
7.  frontrun_mev                  // front-run / sandwich / back-run / latency arbitrage vectors  
    ### 7-A  Quick front-running / TOD checklist
    - Public functions: can caller profit by seeing a tx in mempool and racing it?  
    - Sequencing deps: does fn A write state that fn B reads in the *same* block?  
    - Value-transfer timing: funds sent immediately after a calc the attacker can influence?  
    - Deterministic selection: winner/outcome based on current on-chain state?  
    - Mitigations present? (pull payments, commit-reveal, VRF, time-locks, ACL)  
8.  oracle                        // price-feed spoofing, stale data, missing sanity checks  
9.  randomness                    // predictable entropy, miner influence  
10. reentrancy                    // state update after external call, cross-function  
11. delegatecall_low_level_ops    // unsafe `delegatecall`, inline assembly scribbles  
12. replay_attack                 // sig replay, chain-ID mix-ups  
13. upgradeability_initializer_safety // proxy init gaps, `initializer()` abuse  
14. self_destruct                 // griefing / forced-ETH via `selfdestruct`  
15. zero_code                     // constructor-phase contract bypasses  
16. flash_loan_economic_manipulation // state checked & used within same tx  

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
- **ANALYZE ALL 16 CATEGORIES** - No exceptions
- **Be thorough** - Don't rush through categories
- **Be precise** - Use exact classification criteria
- **Scope** - if scope is provided below, then only report vulnerability that are in scope
- **Think like an attacker** - Consider how each vulnerability could be exploited
- **Provide only the JSON** - No additional commentary in final output

"#;
