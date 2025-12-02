## Verified Patterns Found: 24

## Verified Patterns Found in following Categories:

- MaturityorGatingByPass
- UnboundedLoops
- InitOrderOrUnintialized
- TimelockEdgeCase
- AccountingInvariantViolation
- StateGrowthOrStorageBloat
- ConfigFootgun
- UpgradeAuthBypass
- AccessControlOrAuthByPass
- FeeOnTransferAssumption
- SlippageMissingOrInsufficient
- StorageCollisionOrSelectorClash



## Summary of Patterns

User Fee Rebate DoS

Campaign Duration Override Bypasses Minimum Reward Rate Invariant

recoverFees sweeps user deposits along with protocol fees

Fee-on-Transfer tokens cause Distributor insolvency

Immediate reward reallocation violates 1-year recovery term

Missing Storage Gap in UUPSHelper Base Contract

Unbounded Growth of Reward Tokens Array

Missing slippage protection for campaign creation fees

Campaign rewards reallocation allows premature clawback

Fee-On-Transfer Token Incompatibility Leading to Accounting Insolvency

Reward token list corruption via duplicate entries

Fee-on-Transfer tokens lead to Distributor insolvency

Campaign overrides ignored by reallocation logic allowing premature reward clawback

Initialization Front-running

Initialization Chain Missing Parent Initializers

Governor can bypass token transfer in increaseTokenBalance

Unbounded state growth in rewardTokens array

Unprotected UUPS Upgrade in Uninitialized State

Accounting discrepancy with fee-on-transfer tokens enables user fund theft

Governor can sweep all user deposits via recoverFees

Fee rebate bypass via operator delegation

Creator Can Seize Rewards Before Users Can Claim

Protocol fee recovery sweeps user pre-deposits due to commingled accounting

Insolvency risk with Fee-on-Transfer tokens due to naive balance updates

## Patterns



 ### Issue Type: ConfigFootgun

 ### Relevant Function/Location: DistributionCreator.setUserFeeRebate

 ### Title
User Fee Rebate DoS
 ### Description/Code Snippet
The `setUserFeeRebate` function allows setting a rebate value greater than `BASE_9` (100%). In `_createCampaign`, the fee calculation `(BASE_9 - feeRebate[msg.sender])` will underflow and revert if the rebate exceeds `BASE_9`. This allows a governor to accidentally or maliciously permanently disable campaign creation for a specific user.
 ### Static Signals
no check for rebate <= BASE_9, subtraction in _computeFees
 ### Assets at Risk
availability of service
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: MaturityorGatingByPass

 ### Relevant Function/Location: DistributionCreator.overrideCampaign

 ### Title
Campaign Duration Override Bypasses Minimum Reward Rate Invariant
 ### Description/Code Snippet
The `_createCampaign` function enforces a minimum reward rate invariant: `(amount * HOUR) / duration >= rewardTokenMinAmount`. However, the `overrideCampaign` function allows the campaign creator to modify the `duration` without re-verifying this invariant. An attacker can create a valid short-duration campaign to pass the check, and then immediately override it to a very long duration (e.g., 100 years), effectively bypassing the minimum reward rate policy set by governance to prevent dust/spam campaigns.
 ### Static Signals
assumes invariant without verifying, parameter update missing validation checks present in creation
 ### Assets at Risk
protocol utility (spam protection)
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: ConfigFootgun

 ### Relevant Function/Location: DistributionCreator.recoverFees

 ### Title
recoverFees sweeps user deposits along with protocol fees
 ### Description/Code Snippet
The `recoverFees` function transfers the entire `balanceOf(address(this))` for specified tokens to a recipient. However, the contract stores both protocol fees (accumulated when `feeRecipient` is zero) and user pre-deposits (via `increaseTokenBalance`) in the same balance. Calling `recoverFees` to collect fees will inevitably sweep and steal all user deposits held in the contract, causing a loss of funds for campaign creators.
 ### Static Signals
tokens[i].balanceOf(address(this)), safeTransfer(to, amount)
 ### Assets at Risk
user deposits, campaign funds
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: DistributionCreator._pullTokens

 ### Title
Fee-on-Transfer tokens cause Distributor insolvency
 ### Description/Code Snippet
The `_pullTokens` function (called by `createCampaign`) transfers `campaignAmountMinusFees` from the creator to the distributor using `safeTransferFrom` but does not verify the actual amount received by the destination. If the whitelisted reward token has a fee-on-transfer mechanism, the `Distributor` contract receives less than the recorded `campaignAmount`. Since the off-chain engine and `Distributor` accounting rely on the full amount being present, this discrepancy leads to insolvency in the `Distributor`, causing the last users' claims to revert due to insufficient balance.
 ### Static Signals
uses input amount instead of post-transfer delta, assumes transferFrom(amount) credits exactly amount
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: TimelockEdgeCase

 ### Relevant Function/Location: DistributionCreator.reallocateCampaignRewards

 ### Title
Immediate reward reallocation violates 1-year recovery term
 ### Description/Code Snippet
The `reallocateCampaignRewards` function allows a campaign creator to reallocate (claw back) unclaimed rewards immediately after the campaign ends (`block.timestamp < ... + duration` revert). However, the protocol's Terms and Conditions (stored in the `message` variable and accepted by users) state that rewards can only be recovered if unclaimed for more than 1 year. This implementation allows creators to bypass the agreed-upon grace period and seize users' earned but unclaimed rewards instantly.
 ### Static Signals
timestamp check allows immediate action after duration, spec (doc) vs implementation mismatch
 ### Assets at Risk
unclaimed user rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StorageCollisionOrSelectorClash

 ### Relevant Function/Location: UUPSHelper.N/A

 ### Title
Missing Storage Gap in UUPSHelper Base Contract
 ### Description/Code Snippet
The `DistributionCreator` contract inherits from `UUPSHelper`, which is an abstract contract that inherits from `UUPSUpgradeable`. While `UUPSUpgradeable` likely contains storage gaps, `UUPSHelper` itself does not define a `__gap`. If `UUPSHelper` is ever modified to include state variables in a future update, it will cause a storage collision with `DistributionCreator`'s state variables (starting with `accessControlManager`), corrupting contract storage upon upgrade.
 ### Static Signals
base contract missing __gap, upgradeable inheritance chain
 ### Assets at Risk
contract state, access control
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: UnboundedLoops

 ### Relevant Function/Location: DistributionCreator.setRewardTokenMinAmounts

 ### Title
Unbounded Growth of Reward Tokens Array
 ### Description/Code Snippet
The `setRewardTokenMinAmounts` function adds tokens to the `rewardTokens` array whenever a token's minimum amount is set to a non-zero value. If a token is removed (amount set to 0) and then re-added, the code explicitly skips the duplicate check and pushes the token again. This allows the `rewardTokens` array to grow unbounded with duplicate entries. The `getValidRewardTokens` function iterates over this entire array (if called without pagination limits), potentially leading to a Denial of Service (DoS) due to gas limits for off-chain integrations or other contracts relying on this view.
 ### Static Signals
array push without unique check, iteration count grows with contract state
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: DistributionCreator.createCampaign

 ### Title
Missing slippage protection for campaign creation fees
 ### Description/Code Snippet
The `createCampaign` function calculates protocol fees dynamically based on `defaultFees` or `campaignSpecificFees`. The user passes a gross `amount`, and the net distribution amount is derived by subtracting fees. There is no parameter for the user to specify a minimum net distribution amount or a maximum fee. If the governance updates the fee parameters (e.g. via `setFees`) between the user's transaction submission and execution, the user may suffer significant slippage, funding a campaign with much less capital than intended.
 ### Static Signals
payout calculated at execution time without minimum bound, admin-settable fees affect user value immediately
 ### Assets at Risk
user campaign budget
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: DistributionCreator.reallocateCampaignRewards

 ### Title
Campaign rewards reallocation allows premature clawback
 ### Description/Code Snippet
The `reallocateCampaignRewards` function allows campaign creators to reallocate unclaimed rewards immediately after the campaign `duration` ends (`block.timestamp < start + duration`). This contradicts the protocol's terms (set in `setMessage` in deployment scripts) which state a 1-year grace period. This implementation allows creators to 'rug' rewards from users who haven't claimed immediately, violating the intended safety invariant.
 ### Static Signals
block.timestamp < _campaign.startTimestamp + _campaign.duration
 ### Assets at Risk
user rewards
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: DistributionCreator.increaseTokenBalance, _pullTokens

 ### Title
Fee-On-Transfer Token Incompatibility Leading to Accounting Insolvency
 ### Description/Code Snippet
The contract supports arbitrary whitelisted ERC20 tokens but fails to handle fee-on-transfer tokens correctly in two critical flows. 
1. In `increaseTokenBalance`, the contract credits the user's internal balance with the input `amount` while the contract actually receives `amount - fee`. 
2. In `_pullTokens` (direct transfer path), the contract transfers `campaignAmountMinusFees` to the Distributor via `safeTransferFrom`. The Distributor receives less than the specified amount, but the campaign is initialized with the full amount. 
This discrepancy causes insolvency in the internal vault (allowing some users to withdraw/spend tokens belonging to others) and in the Distributor (causing the last claimers of a campaign to fail due to insufficient funds).
 ### Static Signals
uses input amount instead of post-transfer delta, assumes transferFrom(amount) credits exactly amount
 ### Assets at Risk
pre-deposited user funds, Distributor reward balance
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: DistributionCreator.setRewardTokenMinAmounts

 ### Title
Reward token list corruption via duplicate entries
 ### Description/Code Snippet
The `setRewardTokenMinAmounts` function adds a token to the `rewardTokens` array if `amount != 0` and the previous minimum amount was 0. However, it does not remove the token from the array when `amount` is set to 0. If a token is disabled (amount set to 0) and then re-enabled (amount set > 0), it is pushed to the `rewardTokens` array a second time. This causes duplicates in the `getValidRewardTokens` view function, potentially breaking off-chain integrations or iterating loops.
 ### Static Signals
rewardTokens.push(tokens[i]), rewardTokenMinAmounts[tokens[i]] == 0
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: DistributionCreator._pullTokens

 ### Title
Fee-on-Transfer tokens lead to Distributor insolvency
 ### Description/Code Snippet
In `_pullTokens`, the contract transfers `campaignAmountMinusFees` to the `Distributor` using `safeTransferFrom`. It does not verify the actual amount received by the `Distributor`. If a Fee-on-Transfer token is used, the `Distributor` receives less than `campaignAmountMinusFees`. However, the campaign parameters (used by the off-chain engine to calculate rewards) assume the full amount is available. This leads to an accounting invariant violation where the Distributor holds fewer tokens than the sum of user claims, causing insolvency for the last claimers.
 ### Static Signals
safeTransferFrom without balance check, accounting assumes transfer success implies full amount
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: MaturityorGatingByPass

 ### Relevant Function/Location: DistributionCreator.reallocateCampaignRewards

 ### Title
Campaign overrides ignored by reallocation logic allowing premature reward clawback
 ### Description/Code Snippet
The `reallocateCampaignRewards` function checks if a campaign has ended using the original campaign parameters (`_campaign.duration`) retrieved from `campaignList`. It ignores any campaign overrides set via `overrideCampaign` (stored in `campaignOverrides`) which are intended to modify campaign parameters such as duration. If a campaign is extended via an override, the reallocation check fails to account for the new end date, allowing the creator to reallocate/claw back rewards based on the original shorter duration while the campaign is ostensibly still active.
 ### Static Signals
vesting/maturity check missing or incomplete, branches on original state ignoring overrides
 ### Assets at Risk
unclaimed rewards
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: InitOrderOrUnintialized

 ### Relevant Function/Location: DistributionCreator.initialize

 ### Title
Initialization Front-running
 ### Description/Code Snippet
The `initialize` function is public and permissionless. The deployment script deploys the proxy and initializes it in separate transactions. An attacker can front-run the initialization transaction to set the `accessControlManager` to a malicious contract and the `distributor` to a controlled address, effectively taking control of the contract deployment.
 ### Static Signals
initialize function is external, AccessControlManager set in initialize
 ### Assets at Risk
protocol control, future deposits
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: InitOrderOrUnintialized

 ### Relevant Function/Location: DistributionCreator.initialize

 ### Title
Initialization Chain Missing Parent Initializers
 ### Description/Code Snippet
The `initialize` function in `DistributionCreator` sets state variables but fails to call `__ReentrancyGuard_init` or `__UUPSUpgradeable_init`. While `ReentrancyGuardUpgradeable` currently defaults to a working state even if uninitialized (0 != _ENTERED), skipping parent initializers is a pattern that can lead to uninitialized critical state if upstream libraries change or if variable defaults differ from initialized values.
 ### Static Signals
missing __ReentrancyGuard_init, missing __UUPSUpgradeable_init
 ### Assets at Risk
reentrancy protection state
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: DistributionCreator.increaseTokenBalance

 ### Title
Governor can bypass token transfer in increaseTokenBalance
 ### Description/Code Snippet
The `increaseTokenBalance` function explicitly skips the `safeTransferFrom` call if `msg.sender` is the Governor, allowing them to increase `creatorBalance` without providing underlying tokens. This breaks the accounting invariant where `creatorBalance` is backed 1:1 by held tokens. A malicious or compromised Governor can mint fake balances and then use `decreaseTokenBalance` to drain real tokens deposited by other users.
 ### Static Signals
if (!accessControlManager.isGovernor(msg.sender)), safeTransferFrom
 ### Assets at Risk
user deposits
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: StateGrowthOrStorageBloat

 ### Relevant Function/Location: DistributionCreator.setRewardTokenMinAmounts

 ### Title
Unbounded state growth in rewardTokens array
 ### Description/Code Snippet
The `setRewardTokenMinAmounts` function appends tokens to the `rewardTokens` array whenever a token's minimum amount is updated from 0 to a non-zero value. The logic `if (amount != 0 && rewardTokenMinAmounts[tokens[i]] == 0)` only checks the *current* stored minimum amount, not whether the token already exists in the `rewardTokens` list. If a token is disabled (amount set to 0) and later re-enabled, a duplicate entry is pushed to the array. Since the array is never pruned, frequent toggling leads to unbounded growth, bloating storage and increasing gas costs for the `getValidRewardTokens` view function.
 ### Static Signals
append-only arrays with no pruning, mapping enumerations via arrays
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: UpgradeAuthBypass

 ### Relevant Function/Location: DistributionCreator._authorizeUpgrade

 ### Title
Unprotected UUPS Upgrade in Uninitialized State
 ### Description/Code Snippet
The `onlyGovernorUpgrader` modifier in `UUPSHelper` bypasses the authorization check if the `accessControlManager` address is `address(0)` (uninitialized). Since `DistributionCreator` initializes `accessControlManager` in the `initialize` function, and the deployment script separates proxy creation and initialization into two transactions, an attacker can front-run the initialization to call `upgradeTo` (which uses `_authorizeUpgrade`), bypassing the check and taking control of the proxy implementation.
 ### Static Signals
public upgradeTo, conditional auth check on address(0)
 ### Assets at Risk
protocol control, user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: DistributionCreator.increaseTokenBalance

 ### Title
Accounting discrepancy with fee-on-transfer tokens enables user fund theft
 ### Description/Code Snippet
The `increaseTokenBalance` function updates the internal `creatorBalance` with the full input `amount`, while the contract actually receives `amount` minus transfer fees if a fee-on-transfer token is used. This inflates the user's internal balance relative to the contract's actual held assets. A malicious user can exploit this by depositing FOT tokens and immediately creating a campaign that transfers out the credited amount (effectively using other users' tokens or non-FOT tokens), leaving the contract insolvent and unable to fulfill withdrawals for other legitimate depositors.
 ### Static Signals
uses input amount instead of post-transfer delta, no balanceBefore/After check
 ### Assets at Risk
user deposits
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: DistributionCreator.recoverFees

 ### Title
Governor can sweep all user deposits via recoverFees
 ### Description/Code Snippet
The `recoverFees` function allows the Governor to withdraw `tokens[i].balanceOf(address(this))`. Since the `DistributionCreator` contract holds all user pre-deposits (`creatorBalance`) in the same address, this function does not distinguish between accumulated protocol fees and user funds. Calling this function to collect fees will effectively rug pull all user deposits held in the contract.
 ### Static Signals
balanceOf(address(this)) transferred, contract holds user deposits, no separation of fees vs deposits
 ### Assets at Risk
user deposits, creatorBalance
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: DistributionCreator._computeFees

 ### Title
Fee rebate bypass via operator delegation
 ### Description/Code Snippet
The `_computeFees` function calculates fee rebates based on `feeRebate[msg.sender]`. Since `createCampaign` allows a campaign to be created by an authorized operator (where `msg.sender` is the operator and `newCampaign.creator` is the payer), a creator with 0% rebate can delegate campaign creation to an operator address with a high rebate. This allows the creator to bypass their assigned fee schedule.
 ### Static Signals
feeRebate[msg.sender], msg.sender != creator
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: DistributionCreator.reallocateCampaignRewards

 ### Title
Creator Can Seize Rewards Before Users Can Claim
 ### Description/Code Snippet
The `reallocateCampaignRewards` function allows a campaign creator to redirect unclaimed rewards to a new address immediately after the campaign ends (`block.timestamp >= start + duration`). However, due to the off-chain Merkl Engine's computation delay (approx. 2 hours) and the subsequent on-chain dispute period (1-2 hours), the rewards for the final epoch are not claimable by users until several hours after the campaign's timestamp expiry. This creates a guaranteed window where a malicious creator can 'reallocate' (steal) the final epoch's rewards before eligible users even have the opportunity to claim them.
 ### Static Signals
block.timestamp < _campaign.startTimestamp + _campaign.duration
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: DistributionCreator.recoverFees

 ### Title
Protocol fee recovery sweeps user pre-deposits due to commingled accounting
 ### Description/Code Snippet
The `recoverFees` function allows the governor to withdraw accumulated protocol fees by transferring the entire `balanceOf(address(this))` of specified tokens. However, the contract also holds user pre-deposits in the same address, tracked via the `creatorBalance` mapping. Since there is no segregation between accumulated fees and user deposits, executing `recoverFees` will inevitably confiscate all user funds held in the contract, violating the invariant that user deposits should remain available for campaign creation.
 ### Static Signals
tokens[i].safeTransfer(to, tokens[i].balanceOf(address(this))), creatorBalance[user][rewardToken]
 ### Assets at Risk
creatorBalance
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: DistributionCreator.increaseTokenBalance

 ### Title
Insolvency risk with Fee-on-Transfer tokens due to naive balance updates
 ### Description/Code Snippet
The `increaseTokenBalance` function updates `creatorBalance` based on the input `amount` parameter rather than the actual amount received by the contract. If a Fee-on-Transfer (FOT) token is whitelisted, the contract receives less than the recorded `creatorBalance`. This causes `sum(creatorBalance) > balanceOf(address(this))`, leading to insolvency where the last users to attempt withdrawal or campaign creation will fail.
 ### Static Signals
_updateBalance(user, rewardToken, creatorBalance[...] + amount), safeTransferFrom(msg.sender, address(this), amount)
 ### Assets at Risk
creatorBalance
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

