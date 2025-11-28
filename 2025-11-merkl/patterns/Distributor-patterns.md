## Verified Patterns Found: 21

## Verified Patterns Found in following Categories:

- AccessControlOrAuthByPass
- TimelockEdgeCase
- UnsafeRecipient
- InitOrderOrUnintialized
- AccountingInvariantViolation
- FeeOnTransferAssumption
- GriefableCallbacks
- FeeAccountingDrift
- Reentrancy
- StandardViolation
- ExternalCallAfterStateChange
- ERC20DecimalsMismatch
- UncheckedLowLevelCallResults



## Summary of Patterns

Silent swallowing of onClaim hook failure leads to lost user funds

Excessive Guardian privileges allow draining user rewards

Cross-chain Merkle proof replay risk

Proxy Initialization Race Condition

Incompatibility with Fee-on-Transfer tokens breaks dispute resolution

Dispute resolution broken for fee-on-transfer dispute tokens

Fee-on-Transfer Token Support Breaks Dispute Mechanism

Integration Risk: onClaim hook receives cumulative amount

Reentrancy in disputeTree leads to fund loss

Dispute resolution DoS via fee-on-transfer tokens

Dispute mechanism DoS via Fee-on-Transfer tokens

Governor updateTree call promotes pending/disputed tree to active status

Dispute mechanism broken by Fee-on-Transfer tokens

Stale disputeAmount after disputeToken update

Dispute mechanism permanent lockup with Fee-on-Transfer tokens

Swallowed revert in claim hook breaks atomicity

Bypass of operator whitelist via `tx.origin` check

Incompatibility with Fee-on-Transfer reward tokens

Griefable Callbacks in Reward Claiming

Griefing via return bomb in onClaim callback

Griefable claim recipient can DoS batch claiming

## Patterns



 ### Issue Type: ExternalCallAfterStateChange

 ### Relevant Function/Location: Distributor._claim

 ### Title
Silent swallowing of onClaim hook failure leads to lost user funds
 ### Description/Code Snippet
In the `_claim` function, the contract transfers tokens to the recipient and subsequently calls `IClaimRecipient(recipient).onClaim` within a `try/catch` block. The `catch` block is empty, causing any revert in the hook (e.g., due to gas limits, temporary logic failures, or paused states) to be silently ignored. Crucially, the token transfer occurs *before* this call and is not rolled back. If the recipient is a smart contract (such as a vault or smart wallet) that relies on the `onClaim` callback to update internal accounting (e.g., crediting a user's deposit balance), the state becomes inconsistent: the vault receives the tokens, but the user is never credited, effectively causing a loss of funds.
 ### Static Signals
no try/catch or rollback on failure, bookkeeping updated; external call at end
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: Distributor.toggleOperator, toggleMainOperatorStatus

 ### Title
Excessive Guardian privileges allow draining user rewards
 ### Description/Code Snippet
The `toggleOperator` and `toggleMainOperatorStatus` functions allow the `Guardian` role (described as having limited privileges) to authorize operators for any user or globally. A compromised or malicious Guardian can authorize themselves as a main operator for a token and drain all user rewards via `claim`.
 ### Static Signals
onlyGuardian modifier on sensitive state, operator approval bypass
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: Distributor._verifyProof

 ### Title
Cross-chain Merkle proof replay risk
 ### Description/Code Snippet
The Merkle leaf construction `keccak256(abi.encode(user, token, amount))` does not include the chain ID or contract address. If the same Merkle root is reused across different chains (e.g. by operational error or design), a valid proof from one chain can be replayed on another, potentially allowing double claims if the user has a balance on both.
 ### Static Signals
keccak256(abi.encode(...)) missing chainId
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: InitOrderOrUnintialized

 ### Relevant Function/Location: Distributor.initialize

 ### Title
Proxy Initialization Race Condition
 ### Description/Code Snippet
The `Distributor` contract is designed to be used with a proxy, and its `initialize` function is external. The provided deployment script (`Distributor.s.sol`) deploys the `ERC1967Proxy` and then calls `initialize` in a separate transaction (or separate call within the script). This non-atomic initialization sequence allows an attacker to front-run the `initialize` call on the deployed proxy, setting the `accessControlManager` to a malicious address. The attacker would then gain Governor privileges, allowing them to recover tokens or control the contract.
 ### Static Signals
initialize callable after deployment, missing atomic initialization in proxy constructor
 ### Assets at Risk
contract control, pre-funded rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Distributor.disputeTree, resolveDispute

 ### Title
Incompatibility with Fee-on-Transfer tokens breaks dispute resolution
 ### Description/Code Snippet
The `disputeTree` function assumes that `safeTransferFrom(msg.sender, address(this), disputeAmount)` results in the contract receiving exactly `disputeAmount`. If the `disputeToken` charges a fee on transfer, the contract receives less. Subsequently, `resolveDispute` attempts to transfer the full `disputeAmount` back to the disputer (if valid) or governor (if invalid). This transfer will revert due to insufficient balance (if the contract has no excess funds), causing the dispute resolution to be permanently blocked.
 ### Static Signals
assumes transferFrom(amount) credits exactly amount, accounting based on transfer parameter, not actual balance change, no balanceBefore/After check
 ### Assets at Risk
dispute collateral
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Distributor.resolveDispute

 ### Title
Dispute resolution broken for fee-on-transfer dispute tokens
 ### Description/Code Snippet
The `disputeTree` function transfers `disputeAmount` from the user to the contract. If `disputeToken` is a fee-on-transfer token, the contract receives less than `disputeAmount`. The `resolveDispute` function attempts to transfer the full `disputeAmount` back to the `disputer` (if valid) or the `governor` (if invalid). This transfer will revert due to insufficient balance, causing a Denial of Service on the dispute resolution mechanism and locking the contract in a disputed state.
 ### Static Signals
safeTransfer(disputer, disputeAmount), disputeAmount transfer assumed 1:1
 ### Assets at Risk
disputeToken
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeAccountingDrift

 ### Relevant Function/Location: Distributor.disputeTree

 ### Title
Fee-on-Transfer Token Support Breaks Dispute Mechanism
 ### Description/Code Snippet
The `disputeTree` function transfers `disputeAmount` of `disputeToken` from the disputer to the contract using `safeTransferFrom`, but does not account for potential deflationary fees (fee-on-transfer tokens). If a fee is taken, the contract receives less than `disputeAmount`. However, `resolveDispute` attempts to refund the full `disputeAmount` to the disputer (if valid) or transfer it to the governor (if invalid) using `safeTransfer`. This call will revert due to insufficient balance, permanently locking the dispute in an unresolved state (`disputer != 0`), effectively DoS-ing the tree update mechanism.
 ### Static Signals
safeTransferFrom without balance check, subsequent safeTransfer of same amount
 ### Assets at Risk
governance/dispute mechanism
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: Distributor._claim

 ### Title
Integration Risk: onClaim hook receives cumulative amount
 ### Description/Code Snippet
The `_claim` function passes the `amount` (cumulative total from the Merkle tree) to the `onClaim` hook, rather than the `toSend` (incremental) amount actually transferred in the current transaction. Many standard DeFi integrations (vaults, wrappers) expect hooks to report the delta/received amount. If a recipient contract uses the `amount` parameter to update its accounting, it will incorrectly credit the user with the cumulative total every time they claim a small increment, leading to double-counting and potential insolvency of the recipient.
 ### Static Signals
accounting based on transfer parameter, not actual balance change, confusing parameter naming
 ### Assets at Risk
Recipient funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: Reentrancy

 ### Relevant Function/Location: Distributor.disputeTree

 ### Title
Reentrancy in disputeTree leads to fund loss
 ### Description/Code Snippet
The `disputeTree` function violates the Checks-Effects-Interactions pattern by updating the `disputer` state variable *after* the external `safeTransferFrom` call. If `disputeToken` is an ERC777 token or similar with transfer hooks, an attacker can re-enter `disputeTree`, paying the dispute bond multiple times while overwriting the `disputer` address. Upon resolution, only one bond amount is refunded.
 ### Static Signals
state update after external call, missing nonReentrant
 ### Assets at Risk
disputeToken
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Distributor.resolveDispute

 ### Title
Dispute resolution DoS via fee-on-transfer tokens
 ### Description/Code Snippet
The `disputeTree` function pulls `disputeAmount` from the disputer using `safeTransferFrom`, but does not verify the actual amount received. If `disputeToken` is a fee-on-transfer token, the contract receives less than `disputeAmount`. The `resolveDispute` function subsequently attempts to transfer the full `disputeAmount` back (either to the disputer or governor). This transfer will revert due to insufficient balance, permanently locking the contract in a disputed state and freezing the Merkle tree.
 ### Static Signals
safeTransferFrom without balance check, safeTransfer of full amount
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Distributor.disputeTree, resolveDispute

 ### Title
Dispute mechanism DoS via Fee-on-Transfer tokens
 ### Description/Code Snippet
The `disputeTree` function transfers `disputeAmount` from the user but does not check the actual amount received. If `disputeToken` applies a transfer fee, the contract receives less than `disputeAmount`. `resolveDispute(true)` subsequently attempts to transfer the full `disputeAmount` back to the disputer, which will revert due to insufficient balance. This permanently locks the contract in a disputed state, preventing `updateTree`.
 ### Static Signals
safeTransferFrom without balance check, safeTransfer of fixed amount
 ### Assets at Risk
disputeToken, protocol liveness
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: TimelockEdgeCase

 ### Relevant Function/Location: Distributor.updateTree

 ### Title
Governor updateTree call promotes pending/disputed tree to active status
 ### Description/Code Snippet
The `updateTree` function updates `lastTree = tree` before installing the new tree. `lastTree` is intended to be the safe fallback while the current `tree` is in its dispute period. However, if `updateTree` is called (by a Governor) while the current `tree` is still pending or disputed, that unverified/bad tree is moved to `lastTree`. Since `getMerkleRoot` returns `lastTree` during the new dispute period, users are immediately exposed to the unverified tree, bypassing its dispute window.
 ### Static Signals
lastTree = tree, getMerkleRoot returns lastTree when in dispute
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Distributor.disputeTree

 ### Title
Dispute mechanism broken by Fee-on-Transfer tokens
 ### Description/Code Snippet
The `disputeTree` function transfers `disputeAmount` from the disputer to the contract using `safeTransferFrom`. This assumes the contract receives the exact `disputeAmount`. However, if the `disputeToken` is a fee-on-transfer token, the contract receives less than the amount recorded. The `resolveDispute` function subsequently attempts to transfer the full `disputeAmount` back to the disputer (if valid) or the governor (if invalid). This transfer will revert due to insufficient balance, permanently locking the dispute resolution mechanism and the staked funds.
 ### Static Signals
safeTransferFrom(msg.sender, address(this), disputeAmount), safeTransfer(disputer, disputeAmount), no balance check before/after transfer
 ### Assets at Risk
disputeToken
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ERC20DecimalsMismatch

 ### Relevant Function/Location: Distributor.setDisputeToken

 ### Title
Stale disputeAmount after disputeToken update
 ### Description/Code Snippet
The `setDisputeToken` function allows the governor to change the token used for disputes but does not reset or validate the existing `disputeAmount`. If the new token has different decimals (e.g., switching from 18-decimal DAI to 6-decimal USDC), the required dispute stake will be incorrect by orders of magnitude (e.g., 10^12 times too high or too low), potentially causing a DoS of the dispute mechanism or allowing cheap spam disputes.
 ### Static Signals
mixes token amounts with 18-decimal math unscaled, token/asset address changes without accounting migration
 ### Assets at Risk
disputeToken
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Distributor.disputeTree

 ### Title
Dispute mechanism permanent lockup with Fee-on-Transfer tokens
 ### Description/Code Snippet
The `disputeTree` function requires the caller to transfer `disputeAmount` of `disputeToken` to the contract via `safeTransferFrom`. It does not check the actual amount received. If `disputeToken` is a fee-on-transfer token, the contract receives less than `disputeAmount`. The `resolveDispute` function later attempts to transfer the full `disputeAmount` back to the disputer (if valid) or the governor (if invalid). This transfer will revert due to insufficient balance. Because `resolveDispute` is the only standard mechanism to clear the `disputer` flag and unfreeze the Merkle tree updates, the contract's update mechanism becomes permanently bricked until governance manually intervenes to fund the deficit.
 ### Static Signals
accounting state not updated when underlying asset is swapped/upgraded, balance tracking references different token than actual holdings
 ### Assets at Risk
protocol availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UncheckedLowLevelCallResults

 ### Relevant Function/Location: Distributor._claim

 ### Title
Swallowed revert in claim hook breaks atomicity
 ### Description/Code Snippet
In the `_claim` function, the contract transfers tokens to a recipient and then attempts to call the `onClaim` hook on that recipient. This call is wrapped in a `try/catch` block with an empty `catch` clause. If the recipient is a smart contract that reverts inside `onClaim` (e.g., due to a paused state, internal error, or slippage check), the revert is ignored, and the token transfer persists. This breaks atomicity for contracts that rely on the hook to account for received funds (e.g., vaults crediting shares), potentially leading to loss of user funds or state inconsistencies.
 ### Static Signals
try IClaimRecipient(recipient).onClaim, catch {}, swallowed revert
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: Distributor._claim

 ### Title
Bypass of operator whitelist via `tx.origin` check
 ### Description/Code Snippet
The `_claim` function includes an authorization check `msg.sender != user && tx.origin != user`. This logic allows any caller to bypass the `operators` whitelist if `tx.origin == user`. While the contract forces the recipient to be the user in this scenario, this allows unauthorized actors (e.g., phishing contracts or unsolicited wrappers) to force the execution of a claim for a user who has not approved them, bypassing the intended access control model for claim initiation.
 ### Static Signals
no onlyOwner/hasRole, tx.origin used for auth
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Distributor._claim

 ### Title
Incompatibility with Fee-on-Transfer reward tokens
 ### Description/Code Snippet
The `_claim` function calculates the `toSend` amount based on the Merkle proof and immediately updates the `claimed` mapping with this full amount. It then executes `IERC20(token).safeTransfer(recipient, toSend)`. The contract does not verify the actual amount received by the recipient. If the reward token implements fees on transfer, the recipient receives less than the amount recorded in `claimed`. This discrepancy permanently desynchronizes the on-chain accounting from the actual transferred value and can break accounting in recipient contracts that rely on the `amount` parameter passed to `onClaim`.
 ### Static Signals
uses input amount instead of post-transfer delta, no balanceBefore/After check
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: Distributor._claim

 ### Title
Griefable Callbacks in Reward Claiming
 ### Description/Code Snippet
The `_claim` function uses a `try/catch` block around the `IClaimRecipient(recipient).onClaim` call with an empty `catch` clause. This swallows any reverts from the recipient's hook. If a user sets a smart contract as a recipient (e.g. a vault or payment splitter) that relies on the `onClaim` hook for internal accounting (like issuing shares or crediting deposits), a failure in the hook (due to gas limits, logic checks, or pausing) will be ignored. The Distributor will still transfer the tokens, but the recipient contract will not be aware of the deposit, leading to a loss of user funds or state inconsistency.
 ### Static Signals
try IClaimRecipient(recipient).onClaim(...) ... catch {}
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: Distributor._claim

 ### Title
Griefing via return bomb in onClaim callback
 ### Description/Code Snippet
The `_claim` function executes a low-level call to `IClaimRecipient(recipient).onClaim` inside a `try` block. Solidity's `try/catch` mechanism attempts to copy the return data to memory. A malicious recipient can return an excessively large payload ('return bomb'), causing the transaction to consume all available gas due to memory expansion costs. This allows a malicious user or recipient to DoS batch claiming operations (where an operator claims for multiple users in one transaction), as the failure of one claim will revert the entire batch.
 ### Static Signals
callback gas not limited (forwarding all gas), return data not bounded (return bomb vulnerability)
 ### Assets at Risk
gas
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: Distributor._claim

 ### Title
Griefable claim recipient can DoS batch claiming
 ### Description/Code Snippet
The `_claim` function is designed to handle batch claims and uses a `try/catch` block to prevent a single recipient's failure from reverting the entire transaction. However, inside the `try` block, the code explicitly checks `if (callbackSuccess != CALLBACK_SUCCESS) revert Errors.InvalidReturnMessage();`. A malicious user can set a recipient contract that returns a success status but an invalid return value (e.g., `bytes32(0)`). This triggers the explicit revert, causing the entire batch transaction to fail. This allows a single malicious user to grief operators or other users attempting to batch-claim rewards.
 ### Static Signals
callback success required for core flow to proceed, external call in loop without failure isolation, revert on specific return value despite try/catch wrapper
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

