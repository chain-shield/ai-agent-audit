pub const CODE4RENA_SEVERITY_RUBRIC: &str = r#"

# Severity Classifications

## Estimating Risk

**Assets** = funds, NFTs, data, authorization, or private/confidential information.

* **High (3):** Assets can be directly or indirectly stolen, lost, or compromised (with a valid, realistic attack path).
* **Medium (2):** Assets not directly at risk, but protocol function, availability, or value could be impacted. Requires assumptions or external conditions for exploitation.
* **QA (Low):** Includes:

  * Low-risk issues (no asset risk, state handling, spec mismatch, comments).
  * Governance/Centralization risks (admin privileges, trust assumptions).
  * Non-critical issues (style, clarity, syntax, versioning, monitoring/events).

### Loss of Assets

* **Dust amounts** (rounding errors, marginal fee variations) → QA/Low.
* **Real amounts** → Severity depends on conditions and likelihood.

### Loss of Yield

* **Matured yield** loss = High (same as capital).
* **Dust yield** loss = QA/Low.
* **Unmatured yield/in-motion yield** = capped at Medium.

## Centralization Risks

* Assume assigned roles are trustworthy and act in the protocol’s best interest.
* Reckless admin mistakes = invalid.
* Direct misuse of privileges = QA.
* Bugs only reachable via admin misuse = QA.
* Privilege escalation = judged by likelihood and impact (up to Medium).
* Vulnerabilities in privileged functions under reasonable use = up to Medium.

## Unsupported / Non-Standard Tokens

* Non-standard ERC-20 or fee-on-transfer tokens = **out of scope** unless explicitly supported in docs.
* Exception: **USDT** (in-scope despite non-standard behavior).
* Judges should invalidate non-compliant findings.
* Definition of ERC-20 = [Ethereum docs](https://ethereum.org/en/developers/docs/standards/tokens/erc-20/).

## View Functions

* Findings about unused `view` functions = Low (QA) at best.

## Out-of-Scope (OOS) Libraries

* Root cause in OOS contract = OOS.
* Incorrect use of OOS functionality in in-scope contract = valid, in-scope.
* Judge discretion applies for edge cases.

## User Mistakes

* Issues requiring careless user input = QA at best, may be invalid.
* Non-privileged users expected to preview transactions.
* Phishing and bad user hygiene fall under this rule.

## Speculation on Future Code

* Issues not exploitable within current scope = speculative.
* Only valid if root cause exists in current code.
* Wardens may argue likelihood of future code changes making bug manifest.
* Judges may assign severity based on likelihood and impact.
* Integrations: assume competent third-party integrator with due diligence.

## Event-Related Impacts

* Faulty events assessed by broader functional impact:

  * Used in bridging/proofs = severity based on function affected.
  * Non-compliance with EIPs = based on impact.
  * Cosmetic/readability issues = Low.
* Front-end display/readability bugs = capped at Low.

## Other Specific Rules

* **Approve race condition:**

  * Approve/safeApprove front-run = **not a valid vulnerability**.
  * Approve/safeApprove = **not deprecated**.
  * `increaseAllowance` / `decreaseAllowance` = deprecated, but usage is not a finding.

"#;

pub const CODE4RENA_SEVERITY_RUBRIC_OLD: &str = r#"

 Severity       | Typical Impact 
| -------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **High**       | **Direct, permissionless monetary or control loss**:  
- Permanent theft/drain of funds (vault, pool, treasury)  
- Inflation/mint exploits (share inflation, reward overclaim with immediate cashout, unbounded mint)  
- **Reward redirection/distortion with immediate extractable value** (e.g., unauthorized validator/score/delegation that increases attacker’s payouts or diverts others’ rewards; governance/voting-power hijack that changes payout policy)  
- Permanent freezing/bricking of funds (withdrawals impossible)  
- Oracle/price manipulation enabling profitable trades or draining reserves  
- Arbitrary code execution / delegatecall takeover  
- Governance capture or voting-power theft  
- Critical ERC standard violation that enables theft or loss of redeemability (e.g., burn bypass that desynchronizes balances vs. totalSupply in a way that lets attacker cash out) |
    
| **Medium**     | **Convincing, repeatable permissionless exploit that needs admin intervention to fix**:  
- Temporary DoS of core user flows (deposits/withdrawals paused or blocked)  
- **Reward distortion that is bounded/temporary or needs admin repair** (e.g., mis-weighted rewards that don’t allow immediate cashout or are limited to a small window until config is fixed)  
- Oracle/math skew that misprices swaps, collateral, or rewards with $$ impact but not a direct drain  
- Accounting errors causing balance mismatches, temporary fund lockups, or reversible asset misallocation  
- Token assumption breaks (fee-on-transfer/rebase/decimals) causing stuck funds or under/overpayment but fixable by admin/state repair  
- Unbounded gas growth that blocks execution until admin cleanup |

| **Low**        | **Griefing / minor safety issues**:  
- Edge-case DoS (requires attacker to burn gas, little systemic impact)  
- Mild precision drift (rounding pennies, no extractable gain)  
- Best-practice deviations (reentrancy guard missing but no impact, unchecked SafeERC20 return that only causes revert)                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
- User mistake: avoidable self-loss with no attacker profit or protocol risk (e.g., sending ETH to non-productive path with no refund), mitigable via pre-checks or documented constraints.

| **Gas / Info** | **Non-payable noise**:  
- Gas optimizations  
- NatSpec, comments, documentation errors  
- Style/clarity issues                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
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
