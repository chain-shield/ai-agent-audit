
## Issue => Proof Steps

For this security vulnerability posted above follow these steps, please

1. is the bug IN SCOPE ?
Privileged Roles are protected so exploit MUST be permissionless.  Files is scope are defined in scope.txt file, make sure the exploit occurs in one of these files.
also note : 
The FAssets system is able to support wrapped tokens for XRP, BTC and DOGE. However, the initial deployment will only have XRP (FXRP) enabled and that will be the sole scope of this audit competition. Any attacks related to FBTC, FDOGE, or UTXO-based logic in general, are out of scope.

if its in scope then...
2. honest assessment: is bug legit? does protocol have safeguards against it? is it realistic for an attacker to exploit?
is 'Derived From' pattern/invariant assumption correct? 
If yes to ALL of the above then...

3. is the severity level accurately stated? where severity levels are:  High | Medium | Low | informational ?
Would it likely receive **≥ Medium severity** in a Code4rena contest

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

ALSO, carefully consider how likely is this attack to happen? (if highly unlikely its a LOW)


NEXT

now create a PoC test in hardhat for this inside of test/PoC-mint-during-pause.ts. Please utltiize any mocks nad configurations this ts file has alerady provided (required by C4)

4. 

For this finding create a PoC test in hardhat for this inside of  test/PoC-zero-price-liquidation.ts.  Please utltiize any mocks and configurations this ts file has already provided (required by C4). . Tests must be written in hardhat.

keep updating and rerunning test until is passes (while maintaining rigirous PoC for issue). use nvm to switch to node 20 to run

5. ->
## WRITE REPORT
please write up professional C4 submission ready report that includes: title, severity, github link to root cause (with line numbers) , Vulnerability details that include Finding description, proof of concept, impact, and Mitigation steps.

In Vulnerability Details please add relevant code snippets to corroborate vulnerability. (make sure code snippets do NOT have <augment_code..> tag wrapping them.)


------------------------------------------------------------------
## DEDUP

Do the these findings have the same root cause? If not, do any 2 of the finding have same root cause?
Should these be reported in same report for C4? (if have same root cause?)

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

