
## Issue => Proof Steps

Follow these steps, please

Honest Assessment:
1. is the bug IN SCOPE per client scope? see covenant-scope.md 
if its in scope then...
2. honest assessment: is bug legit? does protocol have safeguards against it? carefuly trace code and read natspec + inline comments.
3. is 'Derived From' pattern/invariant assumption correct? 
4. is this by design?
Check ALL:
- NatSpec (@dev, @notice, @custom)
- Inline comments (`// NOTE:`, `// IMPORTANT:`)
- megapot-docs.md & megapot-scope.md (Known Limitations, Design Decisions)
- Function placement (Emergency/Admin sections)
- Function naming (emergencyPause, adminOnly)

**CRITICAL: Documentation ≠ Not a Vulnerability**
- Documented behavior can STILL be a valid finding if it creates:
  • Economic risk/loss for users (e.g., liquidators, LPs, depositors)
  • Incentive misalignment that harms protocol health
  • Unfair value extraction or MEV opportunities
  • Lack of user protection (missing slippage, deadlines, bounds)
- Examples of VALID findings despite being "by design":
  • Liquidations without minOut → liquidator loss risk (Medium)
  • Auctions without price floors → value extraction (Medium)
  • Withdrawals without deadlines → MEV/sandwich risk (Medium)
  • Fee mechanisms that systematically favor one party (Low/Medium)
- Only mark Invalid if behavior is BOTH documented AND has no security/economic impact

5. Configuration check: 
**Does finding rely on constants (time, thresholds, buffers)?**

If YES, check:
- `// for testnet`, `// for testing`, `// temporary` comments
- Suspiciously small values (WEEK=1800 vs 604800, BUFFER=300 vs 3600)
- Commented-out production values
- Re-assess with production config

6. how likely is this attack? For Medium or High severity attack cannot be unliklely, unless its HIGH impact.
7. is Impact accurately stated?

**WHEN IN DOUBT, LEAN TOWARD VALID**
- If a finding shows realistic user loss, mark Valid even if documented
- If a finding matches historical C4 Medium patterns, mark Valid
- If a finding shows missing standard protections, mark Valid
- Only mark Invalid if you're VERY confident it's a non-issue
- Mark "in doubt" findings as SomeWhatConfident (not VeryConfident or Confident)
- Remember: False negatives (missing real bugs) are worse than false positives

**Please go through verify-checklist.md to fully confirm finding is valid.**

1. is the bug IN SCOPE per client scope? see covenant-scope.md 
if its in scope then...
2. honest assessment: is bug legit? does protocol have safeguards against it? carefuly trace code. 
3. is this by design? (look at natspec comments and megapot-docs.md)
   **NOTE:** Documented behavior can still be a vulnerability if it creates economic risk or harms users
4. how likely is this attack? For Medium or High severity attack cannot be unliklely, unless its HIGH impact.
5. is Impact accurately stated?
6. is the severity level accurately stated? where severity levels are:  High | Medium | Low | informational ?
Would it likely receive **≥ Medium severity** in a Code4rena contest

# Code4rena Severity Classifications

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


NEXT

4. 
ADD PoC test inside of test/c4-poc/PoCLaunchpad.t.sol or test/c4-poc/PoCPerps.t.sol depending on which is has mock assests and contracts we need to interact with for specific PoC test.

## Creating a PoC Guide

The project is composed of two core systems; the perpetual CLOB system, and the Launchpad system. Within the codebase, we have introduced two test files (PoCPerps.t.sol & PoCLaunchpad.t.sol) under the test/c4-poc folder that sets up each system with mock implementations to allow PoCs to be constructed in a straightforward manner.

Depending on where the vulnerability lies, Wardens should utilize the correct PoC file alongside the relevant storage entries (i.e. the launchpad in case a launchpad vulnerability is demonstrated etc.).

For a submission to be considered valid, the test case should execute successfully via the following command:

forge test --match-test submissionValidity
PoCs meant to demonstrate a reverting transaction must utilize the special expect utility functions forge exposes. Failure to do so may result in an invalidation of the submission.

NOTE: do NOT alter the TestBase files.  make all changes in either PoCLuanchpad.t.sol or PoCPerps.t.sol , PoC must be contained in submissionValidity function.

**REQUIREMENTS**: PoC must use state and contracts deployed in Base setup() that are inherited.


If PoC tests confirmes issue then...

5. ->
Please write up full Poc report following poc-proof/readme.md guidelines. 
Please include FULL runnable PoC we created in this step , as C4 judges will run it.
Also include full OUTPUT for test 


------------------------------------------------------------------
## REPORT VERIFICATION

1. honest assessment: does this bug report look correct? 
Does foundry test look right? is it correctly setup to demonstrate bug exists?
Do the foundry test results look right?  Does it prove bug is real?

2. is the severity level accurately stated? where severity levels are: Critical | High | Medium | Low | informational ?
Would it likely receive **≥ Medium severity** in a Code4rena contest
this severity scale is for a competitive audit. Refer to the below:

### Code4rena severity rubric

| Severity       | Typical Impact 
| -------------- ---------------- |
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

| **Gas / Info** | **Non-payable noise**:  
- Gas optimizations  
- NatSpec, comments, documentation errors  


3. what would this security bug (detailed in below report)  result in if exploited? list all that apply: 

- Direct theft of any user funds, whether at-rest or in-motion, other than unclaimed yield
- Permanent freezing of funds
- Protocol insolvency
- Theft of unclaimed yield
- Temporary freezing of funds for at least 24 hours
- Theft of gas
- Smart contract unable to operate due to lack of token funds
- Contract fails to deliver promised returns, but doesn't lose value
- Unbounded gas consumption
- Temporary freezing of funds for at least 1 hour

## CONTEXT 

<ADD SOLIDITY IR + STORAGE>

## REPORT CODE VERIFICATION

Please review the security bug report below.

## REPORT


## END OF REPORT

Does this security bug report contain the below test contract?
Are they identical?

## TEST CONTRACT 

