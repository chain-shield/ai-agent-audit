
## Issue => Proof Steps

For this security vulnerability posted above follow these steps, please

1. is the bug IN SCOPE ? (carefully review indexfun-scope.md and indexfun-scope.txt)

if its in scope then...
2. honest assessment: is bug legit? does protocol have safeguards against it? is it realistic for an attacker to exploit? does this exploit work under normal market conditions?
are there any external dependencies that cannot be seen and analyzed (creating uncertainly about validity of finding)?
is 'Derived From' pattern/invariant assumption correct? 
If yes to ALL of the above then...

3. is the severity level accurately stated? where severity levels are:  High | Medium | Low | informational ?
Would it likely receive **≥ Medium severity** in a Sherlock contest

# Criteria for Issue Validity

### **II. Criteria for Issue Severity:**

### III. Sherlock's standards:

1. **Subjectivity:** Despite these guidelines, we must understand that because of the complexity & subjective nature of smart contract security, there may be issues that are judged beyond the purview of this guide. However, for the vast majority of cases, this guide should suffice. Sherlock's chosen judges continue to have the last word on considering any issue as valid or not.
2. **Hierarchy of truth:** If the protocol team provides no specific information, the default guidelines always apply.

   If the protocol team includes specific information in the README or CODE COMMENTS, that information may be used as context during the judging process. In cases where there is a contradiction between the README and CODE COMMENTS, the README should be considered the primary source of truth.

   The judge can decide that CODE COMMENTS are outdated. In that case, the default guidelines apply.

   > Example: The code comments state that "a swap can never fail" even though the code is built to support failing swaps.

   The protocol team can use the README (and only the README) to define the protocol's invariants/properties. Specifically, only the following question can be used for that:

   > What properties/invariants do you want to hold even if breaking them has a low/unknown impact?

   Issues that break the invariants from the above question, irrespective of whether the impact is low/unknown, could be assigned Medium severity if it doesn't conflict with common sense. High severity will be applied only if the issue falls into the High severity category in the judging guidelines.

   > Example: The README states "Admin can only call XYZ function once" but the code allows the Admin to call XYZ function twice; this is a valid Medium

   > Example: The README states "Variable X must always match USDC amount in the contract" and a user can donate USDC to break that invariant without causing issues to the protocol; this is an invalid issue

   The Sherlock Judge can use public statements up until 24h before the contest ends to override the language in the chosen source of truth.

   If guidelines are updated, the new guidelines apply only to contests that start after the date of change. Please check [criteria-changelog](https://docs.sherlock.xyz/audits/judging/guidelines/criteria-changelog "mention") for information on the latest changes in the judging guidelines.

   **Historical decisions are not considered sources of truth.**
3. **Could Denial-of-Service (DOS), griefing, or locking of contracts count as Medium (or High) severity issue?** To judge the severity we use two separate criteria:

   1. The issue causes funds to be locked for more than a week.
   2. The issue impacts the availability of time-sensitive functions (cutoff functions are not considered time-sensitive).

   If at least one of these is describing the case, the issue can be Medium. If both apply, the issue can be considered High severity. Additional constraints related to the issue may decrease its severity accordingly.

   If a single occurrence of the attack results in a denial of service (DOS) for less than a week, the issue should be evaluated based on a **single occurrence** of the attack, even if it can be repeated indefinitely. It qualifies as a medium-level issue only if it disrupts a clearly time-sensitive function.

   > Note: if the single occurrence of the attack is relatively long (e.g. >2 days), and it takes only 2-3 iterations to cause a 7-day DOS, it may be considered a valid finding.
4. **(External) Admin trust assumptions**:\
   If a protocol defines restrictions on the owner/admin, issues involving attacks that bypass these restrictions may be considered valid. These restrictions must be explicitly stated and will be assessed case by case. Admin functions are generally assumed to be used correctly and not harm users/the protocol.

   Note: if the (external) admin will unknowingly cause issues, it can be considered a valid issue.\
   Note: the internal protocol roles are trusted by default. They can be considered untrusted (i.e. act maliciously) only if it's, specifically, claimed to be untrusted in the contest README OR the user can get the role without admin/owner permission (e.g. paying a specific fee).

   > Example: Admin sets fee to 200%. The issue "Admin can break deposit by setting fee to a 100%+" is invalid as it's common sense that fees can not be more than 100% on a deposit.

   > Example: Admin sets fee to 20%. This will cause liquidations to fail in case the utilization ratio is below 10%, this can be Medium as the admin is not aware of the consequences of his action.
5. **Contract Scope:**
   1. If a contract is in contest Scope, then all its parent contracts are included by default.
   2. In case the vulnerability exists in a library and an in-scope contract uses it and is affected by this bug this is a valid issue.
   3. If there is a vulnerability in a contract from the contest repository but is not included in the scope then issues related to it cannot be considered valid.
6. **Design decisions** are not valid issues. Even if the design is suboptimal, but doesn't imply any loss of funds, these issues are considered informational.

### IV. How to identify a high issue:

1. Direct loss of funds without (extensive) limitations of external conditions. The loss of the affected party must be significant.

**Guidelines for Significant Loss:**

* Users lose more than 1% *and* more than $10 of their principal.
* Users lose more than 1% *and* more than $10 of their yield.
* The protocol loses more than 1% *and* more than $10 of the fees.

### V. How to identify a medium issue:

1. Causes a loss of funds but requires certain external conditions or specific states, or a loss is highly constrained. The loss must be relevant to the affected party.
2. Breaks **core** contract functionality, rendering the contract useless or leading to loss of funds that's relevant to the affected party.

> Note: If a single attack can cause a 0.01% loss but can be replayed indefinitely, it will be considered a 100% loss and can be medium or high, depending on the constraints.

**Guidelines for Relevant Loss:**

* Users lose more than 0.01% *and* more than $10 of their principal.
* Users lose more than 0.01% *and* more than $10 of their yield.
* The protocol loses more than 0.01% *and* more than $10 of the fees.

**Note**
Likelihood is not considered when identifying the severity and the validity of the report.

### VI. Recommendations:

1. A PoC (Proof of Concept) is recommended for issues in any of the following categories to avoid placing the burden of proof on the judge:

* Non-obvious issues with complex vulnerabilities or attack paths
* Issues with non-trivial input constraints, to demonstrate the attack is feasible despite them
* Issues involving precision loss
* Reentrancy attacks
* Attacks related to gas consumption or reverting message calls

Additionally, Watsons are strongly encouraged to specify all conditions required to trigger the issue and to clarify scenarios where these constraints may apply.

If the original report does not include a Proof of Concept (PoC), it will be considered invalid if the issue cannot be clearly understood without one.

2. Issues that depend on front-running will get their severity downgraded if the deployment chain has a private mempool. High will be Medium, and Medium will be Invalid.

> For example, if we have a standard front-running slippage-related issue on DEX, which deserves High severity under normal circumstances (e.g. front-running on Eth Mainnet), should be viewed as Medium on chains with private mempool (e.g. Optimism). Also, the reports have to explain how the issue can happen with unintentional front-running, e.g. saying "the attacker will monitor the mempool looking for the victim's transaction" is invalid.

### VII. List of Issue categories that are NOT VALID:

1. **Gas optimizations:** The user/protocol ends up paying a little extra gas because of this issue.
2. **Incorrect Event values:** Incorrectly calculated/wrong values in emitted events are not considered valid medium or high.
3. **Zero address checks:** Check to make sure input values are not zero addresses.
4. **User input validation:** User input validation to prevent user mistakes is not considered a valid issue. However, if a user input could result in a major protocol malfunction or significant loss of funds for other parties (protocol or other users) it could be a valid high. [Example(Valid)](https://github.com/sherlock-audit/2022-10-illuminate-judging/issues/47)
5. **Admin Input/call validation:**
   1. Admin could have an incorrect call order. Example: If an Admin forgets to `setWithdrawAddress()` before calling `withdrawAll()` This is not a valid issue.
   2. An admin action can break certain assumptions about the functioning of the code. Example: Pausing a collateral causes some users to be unfairly liquidated or any other action causing loss of funds. This is not considered a valid issue.
6. **Contract / Admin Address Blacklisting / Freezing:** If a protocol's smart contracts or admin addresses get added to a "blacklist" and the functionality of the protocol is affected by this blacklist, this is not considered a valid issue.\
   However, there could be cases where an attacker would use a blacklisted address to cause harm to a protocol functioning. [Example(Valid)](https://github.com/sherlock-audit/2022-11-opyn-judging/issues/219)
7. **Front-running initializers:** Front-running initializers where there is no irreversible damage or loss of funds & the protocol could just redeploy and initialize again is not a valid issue.
8. **User experience issues:** Issues causing only minor inconvenience without fund loss, such as temporarily inaccessible funds recoverable by the admin, are not valid.
9. **User Blacklist:** User getting blacklisted by a token/contract causing harm only to themselves is **not** a valid medium/high.
10. Issues assuming future opcode gas repricing are not considered to be of Medium/High severity.\
    **Use of call vs transfer** will be considered as a protocol design choice if there is no good reason why the call may consume more than 2300 gas without opcode repricings.
11. **Accidental direct token transfers**: users accidentally or intentionally directly transferring any tokens (native, ERC20, etc.) into the in-scope contracts, while it is not a part of the expected protocol operations, and the contract doesn't have a way to retrieve them, is **not** valid as a medium/high issue and is considered a user mistake, if the user hurts themselves only. However, if it leads to hurting the protocol and/or other users, it may be considered a valid issue.
12. **Loss of airdrops** or any other rewards that are not part of the original protocol design is not considered a valid high/medium. [Example](https://github.com/sherlock-audit/2023-02-openq-judging/issues/323)
13. **Use of Storage gaps:** Simple contracts with one of the parent contract not implementing storage gaps are considered low/informational.\
    **Exception**: However, if the protocol design has a highly complex and branched set of contract inheritance with storage gaps inconsistently applied throughout and the submission clearly describes the necessity of storage gaps it can be considered a valid medium. [Example](https://github.com/sherlock-audit/2022-09-notional-judging/issues/64)
14. **Incorrect values in View functions** are by default considered **low**.\
    **Exception**: In case any of these incorrect values returned by the view functions are used as a part of a larger function which would result in loss of funds then it would be a valid **medium/high** depending on the impact.
15. **Stale prices and Chainlink round completeness** Recommendations to implement round completeness or stale price checks for any oracle are invalid.

    > Exception: the recommendation to implement stale price checks **may** be valid. For [example](https://github.com/sherlock-audit/2024-12-mach-finance-judging/issues/41), the protocol may be using Pyth pull-based oracle, which requires requesting the price before using it. Hence, if we don't request the price firstly, or check it for staleness, then we can end up using very old price (e.g. from 1 hour/day ago).
16. Issues from the previous contests with `wont fix` labels (if it's an update contest) **and** issues from previous audits (linked in the contest README) marked as acknowledged (not fixed) are not considered valid.
17. **Chain re-org** and **network liveness** issues are not valid.
18. **ERC721 unsafe mint:** Issues where users cannot safemint ERC721 tokens due to unsupported implementation are not valid.\
    Example: <https://github.com/sherlock-audit/2023-03-teller-judging/issues/8>
19. **Future issues:** Issues that result out of a future integration/implementation that was not mentioned in the docs/README or because of a future change in the code (as a fix to another issue) are **not** valid issues.
20. **Non-Standard tokens:** Issues related to tokens with non-standard behaviors, such as [weird-tokens](https://github.com/d-xo/weird-erc20) are not considered valid by default unless these tokens are explicitly mentioned in the README. Tokens with decimals between 6 and 18 are not considered weird.
21. Using Solidity versions that support **EVM opcodes that don't work** on networks on which the protocol is deployed is not a valid issue beacause one can manage compilation flags to compile for past EVM versions on newer Solidity versions.
22. **Sequencers** are assumed to operate reliably without misbehavior or downtime. Vulnerabilities or attacks that rely on sequencers going offline or malfunctioning are invalid.



NEXT

now create a PoC test in hardhat for this inside of test/PoC-mint-during-pause.ts. Please utltiize any mocks nad configurations this ts file has alerady provided (required by C4)

4. 

For this finding create a PoC test in hardhat for this inside of test/PoC-mint-during-pause.ts. Please utltiize any mocks and configurations this ts file has already provided (required by C4). . Tests must be written in hardhat.

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

