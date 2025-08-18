pub const CODE4RENA_SEVERITY_RUBRIC: &str = r#"

| Severity       | Typical Impact 
| -------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **High**       | **Direct, permissionless monetary or control loss**:  
- Permanent theft/drain of funds (vault, pool, treasury)  
- Inflation/mint exploits (share inflation, reward overclaim, unbounded mint)  
- Permanent freezing/bricking of funds (withdrawals impossible)  
- Oracle/price manipulation enabling profitable trades or draining reserves  
- Arbitrary code execution / delegatecall takeover  
- Governance capture or voting-power theft                                                                                                                                                                                                                                             |

| **Medium**     | **Convincing, repeatable permissionless exploit that needs admin intervention to fix**:  
- Temporary DoS of core user flows (deposits/withdrawals paused or blocked)  
- Reward distortion (attacker extracts unearned share of yield/fees, other users underpaid)  
- Oracle/math skew that misprices swaps, collateral, or rewards with $$ impact  
- Accounting errors that cause balance mismatches, temporary fund lockups, or reversible asset misallocation  
- Token assumption break (fee-on-transfer/rebase/decimals) causing funds stuck, withdrawals underpaid, or vault shares inflated  
- Unbounded gas growth (looping through storage arrays/mappings) that blocks function execution until admin fixes state                                                                                                                  |

| **Low**        | **Griefing / minor safety issues**:  
- Edge-case DoS (requires attacker to burn gas, little systemic impact)  
- Mild precision drift (rounding pennies, no extractable gain)  
- Best-practice deviations (reentrancy guard missing but no impact, unchecked SafeERC20 return that only causes revert)                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |

| **Gas / Info** | **Non-payable noise**:  
- Gas optimizations  
- NatSpec, comments, documentation errors  
- Style/clarity issues                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
"#;

pub const SHERLOCK_SEVERITY_RUBRIC: &str = r#"

| Severity  | Typical impact                                                                                          |
|-----------|----------------------------------------------------------------------------------------------------------|
| High      | Direct loss of funds exceeding 1% and $10 of principal, yield, or protocol fees; permanent governance takeover; straightforward exploitable attack path |
| Medium    | Loss of funds exceeding 0.01% and $10 under specific conditions or constraints (e.g., requiring particular market conditions, user interactions, or protocol states; limited by time windows, admin actions, or partial exploitability); DoS locking funds over a week or disrupting time-sensitive functions; replayable attacks with escalating impact |
| Low       | Negligible impact on funds or functionality; gas optimizations, style issues, documentation gaps, or invalid categories (e.g., zero-address checks, front-running without irreversible damage) |

"#;

pub const DEFAULT_SEVERITY_RUBRIC: &str = r#"

| Severity     | Typical impact examples                                                                                                                                                                                                            | What it signals to the team               |
| ------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------- |
| **Critical** | - Direct theft of any funds - Permanent, **total** loss or control of all user or protocol funds - Arbitrary code execution                                                                                                  | “Drop everything—patch immediately.”      |
| **High**     | - **Permanent freezing** or bricking of user or protocol funds (can’t be reversed without privileged migration) - Loss of governance control - Logic that lets an attacker mint/ burn / drain but under specific constraints | “Must fix before next release / upgrade.” |
| **Medium**   | - Temporary loss (funds stuck until admin action) - Convincing grief / DoS that makes the protocol unusable - Oracle or math bugs that skew accounting but don’t directly drain value                                        | “Important, schedule a patch.”            |
| **Low**      | - Minor economic grief (extra gas, incorrect event data) - Edge-case DoS that requires unusual conditions - Best-practice deviations with limited real-world impact                                                          | “Fix in regular development cycle.”       |
| **Insight**  | Code cleanliness, documentation issues, minor style or test suggestions                                                                                                                                                            | “Nice-to-have, no security impact.”       |


"#;
