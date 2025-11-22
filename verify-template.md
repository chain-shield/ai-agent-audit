
## Issue => Proof Steps

Follow these steps, please

Honest Assessment:
1. is the bug IN SCOPE per client scope? see covenant-scope.md 
2. make sure this bug is not already covered in V12 findings: 

if its in scope then...
2. honest assessment: is bug legit? 
Go through checklist in verify-checklist.md, list each step and why it passes or fails.


2. is the severity level accurately stated? where severity levels are:  High | Medium | Low | informational ?
Would it likely receive **≥ Medium severity** in a Code4rena contest



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

