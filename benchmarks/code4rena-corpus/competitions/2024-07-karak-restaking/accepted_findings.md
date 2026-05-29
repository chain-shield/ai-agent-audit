# Accepted H/M Findings: Karak Restaking

# [H-01] Slashing NativeVault will lead to locked ETH for the users

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-karak-restaking
- **Source snapshot:** competitions/2024-07-karak-restaking/final_report.html

NativeVault will lead to locked ETH for the users Submitted by zanderbyte, also found by anonymousjoe, lanrebayode77, sl1, and KupiaSec

- https://github.com/code-423n4/2024-07-karak/blob/f5e52fdcb4c20c4318d532a9f08f7876e9afb321/src/NativeVault.sol#L238
- https://github.com/code-423n4/2024-07-karak/blob/f5e52fdcb4c20c4318d532a9f08f7876e9afb321/src/NativeVault.sol#L277
- https://github.com/code-423n4/2024-07-karak/blob/f5e52fdcb4c20c4318d532a9f08f7876e9afb321/src/NativeVault.sol#L348
- https://github.com/code-423n4/2024-07-karak/blob/f5e52fdcb4c20c4318d532a9f08f7876e9afb321/src/NativeVault.sol#L512
The Karak protocol includes a slashing mechanism that allows the contract owner to penalize stakers by reducing their staked assets in the event of malicious activity by the operator. If a user with a staked balance of 32 ETH is subject to a 3 ETH slashing, they should ideally be able to withdraw the remaining 29 ETH. However, due to a flaw in the implementation, when the user attempts to fully withdraw their ETH, they are only able to withdraw less than the actual remaining amount, with some ETH becoming permanently locked in the protocol.

Specifically, if all share tokens are burned during the withdrawal process, the user cannot access the remaining locked ETH. This results in users receiving fewer ETH than they are entitled to, with the excess ETH becoming inaccessible and permanently locked within the protocol.

## Recommended Mitigation Steps

Due to the complexity and the current limitations in testing the implementation of the slashing mechanism, a precise fix is difficult to pinpoint. In the example provided above, the problem occurs at step 5 where _decreaseBalance is called again reducing the totalAssets by another 3e18 (the slashed amount). If totalAssets are not decreased again and stays 29e18 in the next step, withdrawableWei(msg.sender) would correctly return the minimum between 28999999999999999999 and 29000000000000000000.

However, to address the root cause of the issue, a better mechanism for handling slashing should be implemented and tested.

karan-andalusia (Karak) confirmed via duplicate issue #29 MiloTruck (judge) commented:

Great find!

The crux of this issue is that nodeBalanceWei is calculated after _transferToSlashStore() is called, so node.nodeAddress.balance will have already decreased before the unattributed balance can be added to totalRestakedETH.

This causes a loss of funds for the node owner as he does not receive shares for the unattributed balance that was lost, as such, I believe high severity is appropriate.

Karak mitigated:

This mitigation only burns the ETH that has already been credited to the user consequently avoiding this scenario.

Status:

Mitigation confirmed. Full details in reports from 0xCiphky, KupiaSec and sl1.

# [H-02] The operator can create a NativeVault that can be silently unslashable

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-karak-restaking
- **Source snapshot:** competitions/2024-07-karak-restaking/final_report.html

NativeVault that can be silently unslashable Submitted by Fulum, also found by lanrebayode77, Takarez ( 1, 2 ), ayden ( 1, 2 ), trachev ( 1, 2 ), 20centclub, sl1, givn, and 0xCiphky An operator can create a NativeVault which always reverts when slashAssets() is called and make the vault unslashable.

## Recommended Mitigation Steps

You can add a verified slashingHandler for ETH on the Core contract and verify the operator use the correct slashStore params on the initialize() function in the NativeVault, or on the deployVaults() for NativeVault.

dewpe (Karak) disputed and commented:

We think it should be re-classified to med because it’s ultimately the DSS’ responsibility to figure out which operators and vaults to take in via the registrationHook.selector.

MiloTruck (judge) commented:

The crux of this issue and its duplicates is that for native vaults, vaultConfig.extraData has no input validation in when calling deployVaults(). This means the operator can set manager, slashStore and nodeImplementation to anything.

This issue has demonstrated how an operator can create an unslashable vault by creating it with slashStore as a different address than the whitelisted slashing handler for ETH. Slashing is core functionality of the protocol and being unable to do so would threaten the integrity of operators (eg. operator can act malicious, causing loss of funds, at no risk).

- https://github.com/code-423n4/2024-07-karak-findings/issues/85
describes how the setting the manager will allow the operator to call restricted functions.

As such, I believe high severity is appropriate.

We think it should be re-classified to med because it’s ultimately the DSS’ responsibility to figure out which operators and vaults to take in via the registrationHook.selector.

Given that the DSS implementation isn’t in-scope and wardens did not know what registrationHook could/couldn’t do during the audit, I don’t think it is fair to downgrade the issue based on this.

Karak mitigated:

This mitigation removes the SlashStore altogether and the NativeVault itself burns the slashed ETH.

Status:

Unmitigated. Full details in reports from sl1 and 0xCiphky, and also included in the

# [H-03] A DoS on snapshots due to a rounding error in calculations

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-karak-restaking
- **Source snapshot:** competitions/2024-07-karak-restaking/final_report.html

DoS on snapshots due to a rounding error in calculations Submitted by KupiaSec Creating a snapshot may be impossible due to a rounding error in the calculations.

## Recommended Mitigation Steps

The _transferToSlashStore() function should be fixed as follows:

- uint256 slashedAssets = node.totalRestakedETH - convertToAssets(balanceOf(nodeOwner)); + uint256 slashedAssets; + if(node.totalRestakedETH > convertToAssets(balanceOf(nodeOwner))) { + slashedAssets = node.totalRestakedETH - convertToAssets(balanceOf(nodeOwner)); + } MiloTruck (judge) commented:

Mitigation confirmed. Full details in reports from sl1, KupiaSec and 0xCiphky.

# [H-04] Violation of Invariant Allowing DSSs to Slash Unregistered Operators

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Finding ID:** H-04
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-karak-restaking
- **Source snapshot:** competitions/2024-07-karak-restaking/final_report.html

Submitted by 0xCiphky, also found by 20centclub and KupiaSec Operators select DSSs to allocate their funds to, and for each DSS, they can choose a list of vaults to stake. The selected vaults are fully staked to the chosen DSS.

To unstake, the operator must adhere to a MIN_STAKE_UPDATE_DELAY of 9 days, which prevents front-running a slashing event. An operator can fully unregister from a DSS once all the vaults are unstaked. The DSS has the ability to slash any malicious behaviour that occurred before the withdrawal initiation for up to 7 days.

The problem is, the current implementation breaks one of the main invariants (confirmed by sponsors).

Only DSSs an operator is registered with can slash said operator Consider the following scenario:

Operator calls the requestUpdateVaultStakeInDSS function to unstake Vault 1 from DSS, initiating the 9-day MIN_STAKE_UPDATE_DELAY.

After 8 days, the DSS calls the requestSlashing function on Operator’s Vault 1, starting the 2-day SLASHING_VETO_WINDOW.

After 1 day, the operator calls the finalizeUnstaking function to finalize unstaking Vault 1 from DSS and the unregisterOperatorFromDSS function to unregister from DSS.

After another day, the DSS (or anyone) calls the finalizeSlashing function to finalize the slash on the operator, who is no longer registered with the DSS.

## Impact

The current implementation allows a DSS to finalize a slash on an operator even after the operator has unregistered from the DSS. This breaks the intended invariant that only DSSs an operator is registered with can slash that operator.

Additionally, new users depositing into the DSS may be unaware of the pending slash because the operator is no longer registered with the DSS, leading to a potential loss on deposits.

Tools Used Manual analysis Foundry Recommendation Consider implementing a check to prevent operators from unregistering from a DSS that has pending slashing requests on them. This will ensure that all slashing actions are finalized before an operator can fully unregister, thereby maintaining the intended invariant/design.

MiloTruck (judge) commented:

This issue demonstrates how if requestSlashing() was called in the last 2 days of a vault unstake request (ie. during SLASHING_VETO_WINDOW ), finalizeSlashing() can be called on a vault even after its operator has unregistered the DSS with unregisterOperatorFromDSS().

Given that this breaks one of the main invariants stated in the README, I believe high severity is appropriate.

devdks25 (Karak) commented:

Fixed.

MiloTruck (judge) commented:

There have been comments that this should be downgraded as the veto committee has the ability to stop it, but I disagree. Following this reasoning, all issues related to slashing can be downgraded to QA since the veto committee can simply stop all slashings-related bugs.

It’s not clear when the veto committee will overturn a slashing and for what reasons, as such, we cannot assume this will be stopped by the committee. I think a fair assumption to make is the committee will overturn any slashings that are outright malicious, which isn’t the case here.

This remains as high.

Karak mitigated:

This mitigation validates the operator, vaults status in the finalizing slashing.

Status:

Mitigation confirmed. Full details in reports from sl1 and 0xCiphky.

Medium Risk Findings (5)

# [M-01] Changing the slashingHandler for NativeVaults will DoS slashing

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-karak-restaking
- **Source snapshot:** competitions/2024-07-karak-restaking/final_report.html

slashingHandler for NativeVaults will DoS slashing Submitted by givn, also found by sl1 Relevant code:

NativeVault::slashAssets Description Core ’s state contains a mapping between assets and slashingHandlers - assetSlashingHandlers. Assets that are assigned a slashing handler are considered whitelisted and vaults containing this asset can be deployed.

When a slash is performed on a vault, the following function is called:

function slashAssets ( uint256 slashPercentageWad, address slashingHandler ) external returns ( uint256 transferAmount ); Inside NativeVault ’s implementation of the function, the following check is performed:

if ( slashingHandler != self.

slashStore ) revert NotSlashStore (); This check can be problematic in the case where some number of native vaults have been deployed and the assetSlashingHandlers mapping gets updated with a new slashHandler. Then, the slashAssets function will always revert.

Root Cause Unsynchronized state update causes DoS of slashing functionality inside NativeVault.

## Impact

If the slashingHandler for restaking gets changed at some point it will cause DoS of the slashing functionality for NativeVaults. It will be blocked until contracts get updated with new implementation or old slashing handler is restored.

The issue could be exacerbated if some new NativeVaults get deployed with the new slash handler. Then, reverting to the old slash handler will break the newly deployed NativeVaults, making update implementation the only option to fix this issue. But this is also troublesome because only normal Vaults can have they’re implementation updated for all deployments with one call.

NativeVaults would have to be updated one by one.

# [M-02] A snapshot may face a permanent DoS if both a slashing event occurs in the NativeVault and the staker’s validator is penalized

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-karak-restaking
- **Source snapshot:** competitions/2024-07-karak-restaking/final_report.html

NativeVault and the staker’s validator is penalized Submitted by KupiaSec

- https://github.com/code-423n4/2024-07-karak/blob/main/src/NativeVault.sol#L126-L162
- https://github.com/code-423n4/2024-07-karak/blob/main/src/NativeVault.sol#L476-L495
- https://github.com/code-423n4/2024-07-karak/blob/main/src/NativeVault.sol#L517-L525
- https://github.com/code-423n4/2024-07-karak/blob/main/src/NativeVault.sol#L507-L515

## Impact

The staker can still receive rewards even if their validators are empty.

## Recommended Mitigation Steps

The _decreaseBalance() function should be enhanced to properly handle cases where the burning amount exceeds the existing share amount.

For example:

function _decreaseBalance(address _of, uint256 assets) internal { NativeVaultLib.Storage storage self = _state(); - uint256 shares = convertToShares(assets); + uint256 shares = Math.min(convertToShares(assets), balanceOf(nodeOwner)); _beforeWithdraw(assets, shares); _burn(_of, shares); self.totalAssets -= assets; self.ownerToNode[_of].totalRestakedETH -= assets; emit DecreasedBalance(self.ownerToNode[_of].totalRestakedETH); } MiloTruck (judge) decreased severity to Medium and commented:

Subsequently, Alice’s validator loses all its funds (this can happen within the 7-day snapshot period, even within just 1 hour). A snapshot should then be taken to reduce Alice’s assets by 32 ETH A validator getting slashed for its entire effective balance is only possible due to the correlation penalty, which is extremely unlikely.

Historically, most slashing penalties have been around 1 ETH.

The following conditions must be met for this issue to occur:

Karak slashes an operator for X amount, followed by beacon chain slashing for Y amount.

X +Y must exceed the node owner’s total restaked balance.

I believe this is extremely unlikely to occur, and as such, medium severity is appropriate.

MiloTruck (judge) commented:

According to the C4 severity categorization, the criteria for medium severity is:

2 — Med: Assets not at direct risk, but the function of the protocol or its availability could be impacted, or leak value with a hypothetical attack path with stated assumptions, but external requirements.

As stated in my initial comment, this requires both Karak and beacon chain slashing to occur. The upper limit of how much the DSS can slash an account for is 100%:

uint256 public constant HUNDRED_PERCENT_WAD = 100e18; uint256 public constant MAX_SLASHING_PERCENT_WAD = HUNDRED_PERCENT_WAD; Therefore, it is possible for the sum of Karak and beacon chain slashing to exceed 100%, however unlikely. I view this as “with stated assumptions, but external requirements”.

Regarding impact, validateSnapshotProofs() and validateExpiredSnapshot() will permanently be DOSed for the user - they can’t stake into the vault in the future even if they would like to. This fulfills “the function of the protocol or its availability could be impacted”.

As such, this remains as medium.

karan-andalusia (Karak) confirmed Karak mitigated:

This mitigation accounts for the decrease in balance of the users shares before burning.

Status:

Mitigation confirmed. Full details in reports from 0xCiphky, sl1 ( 1, 2 ) and KupiaSec.

# [M-03] When malicious behavior occurs and DSS requests slashing against vault during 2 day period after SLASHING_WINDOW of 7 days is passed after staker initiates a withdrawal, token amount to be slashed is calculated to be higher than what it should be

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-karak-restaking
- **Source snapshot:** competitions/2024-07-karak-restaking/final_report.html

SLASHING_WINDOW of 7 days is passed after staker initiates a withdrawal, token amount to be slashed is calculated to be higher than what it should be Submitted by rbserver, also found by MrValioBg and Shaheen

## Impact

According to

- https://github.com/code-423n4/2024-07-karak?tab=readme-ov-file#stakers, Stakers can initiate a withdrawal, subject to a ``MIN_WITHDRAW_DELAY`` of 9 days, and DSS can slash any malicious behavior occurring before the withdrawal initiation for up to 7 days. Since MIN_WITHDRAW_DELAY equals SLASHING_WINDOW + SLASHING_VETO_WINDOW, the staker’s withdrawal should be safe without being associated with any malicious behavior and hence not slashable when the DSS does not request any slashing against the corresponding vault within the SLASHING_WINDOW of 7 days after such withdrawal is initiated.

- https://github.com/code-423n4/2024-07-karak/blob/d19a4de35bcaf31ccec8bccd36e2d26594d05aad/src/interfaces/Constants.sol#L13-L16
uint256 public constant SLASHING_WINDOW = 7 days; uint256 public constant SLASHING_VETO_WINDOW = 2 days; uint256 public constant MIN_STAKE_UPDATE_DELAY = SLASHING_WINDOW + SLASHING_VETO_WINDOW; uint256 public constant MIN_WITHDRAWAL_DELAY = SLASHING_WINDOW + SLASHING_VETO_WINDOW; During the 2 day period after the SLASHING_WINDOW of 7 days is passed after the staker’s withdrawal is initiated, such as when just couple minutes are passed after such SLASHING_WINDOW is reached, the staker’s withdrawal cannot be finished but a malicious behavior can occur and the DSS can make a slashing request against the corresponding vault; in this situation, the staker’s withdrawal mentioned previously should not be subject to such slashing though. At that time, when the DSS calls the

Core.requestSlashing function, which further calls the SlasherLib.requestSlashing and SlasherLib.fetchEarmarkedStakes functions, the token amount to be slashed is calculated as the DSS’s allowed slashing percentage of the vault’s totalAssets(), which includes the token amount corresponding to the staker’s withdrawal. Because the staker’s withdrawal should not be slashable, the token amount to be slashed actually should be the DSS’s allowed slashing percentage of a value that equals the vault’s totalAssets() minus the token amount corresponding to the staker’s withdrawal. Thus, the DSS can unfairly slash more underlying token amount from the vault than it should be allowed.

- https://github.com/code-423n4/2024-07-karak/blob/d19a4de35bcaf31ccec8bccd36e2d26594d05aad/src/entities/SlasherLib.sol#L94-L124
function requestSlashing ( CoreLib.Storage storage self, IDSS dss, SlashRequest memory slashingMetadata, uint256 nonce ) external returns ( QueuedSlashing memory queuedSlashing ) { validateRequestSlashingParams ( self, slashingMetadata, dss ); uint256 [] memory earmarkedStakes = fetchEarmarkedStakes ( slashingMetadata ); queuedSlashing = QueuedSlashing ({ dss:

dss, timestamp:

uint96 ( block.

timestamp ), operator:

slashingMetadata.

operator, vaults:

slashingMetadata.

vaults, earmarkedStakes:

earmarkedStakes, nonce:

nonce }); self.

slashingRequests [ calculateRoot ( queuedSlashing )] = true;...

}

- https://github.com/code-423n4/2024-07-karak/blob/d19a4de35bcaf31ccec8bccd36e2d26594d05aad/src/entities/SlasherLib.sol#L79-L92
function fetchEarmarkedStakes ( SlashRequest memory slashingMetadata ) internal view returns ( uint256 [] memory earmarkedStakes ) { earmarkedStakes = new uint256 []( slashingMetadata.

vaults.

length ); for ( uint256 i = 0; i < slashingMetadata.

vaults.

length; ++ i ) { earmarkedStakes [ i ] = Math.

mulDiv ( slashingMetadata.

slashPercentagesWad [ i ], IKarakBaseVault ( slashingMetadata.

vaults [ i ]).

totalAssets (), Constants.

MAX_SLASHING_PERCENT_WAD ); } Moreover, when the MIN_WITHDRAW_DELAY of 9 days is passed after the staker’s withdrawal is initiated, the staker can call the Vault.finishRedeem function for finishing his withdrawal request. Since SLASHING_VETO_WINDOW is 2 days, the DSS can call the Core.finalizeSlashing function for finalizing its slashing request just after the MIN_WITHDRAW_DELAY of 9 days is passed after the initiation of the staker’s withdrawal if the DSS’s slashing request was made when just couple minutes were passed after the SLASHING_WINDOW for such withdrawal of the staker was reached. Because both the staker’s Vault.finishRedeem transaction and the DSS’s Core.finalizeSlashing transaction are sent at the similar time, a malicious miner can place and execute the DSS’s

Core.finalizeSlashing transaction before the staker’s Vault.finishRedeem transaction. In this case, the staker’s withdrawal can be unfairly slashed by the DSS even though it should not be slashed.

- https://github.com/code-423n4/2024-07-karak/blob/53eb78ebda718d752023db4faff4ab1567327db4/src/Vault.sol#L157-L188
function finishRedeem ( bytes32 withdrawalKey ) external nonReentrant whenFunctionNotPaused (Constants.PAUSE_VAULT_FINISH_REDEEM) { ( VaultLib.

State storage state, VaultLib.

Config storage config ) = _storage (); WithdrawLib.

QueuedWithdrawal memory startedWithdrawal = state.

validateQueuedWithdrawal ( withdrawalKey );...

}

- https://github.com/code-423n4/2024-07-karak/blob/d19a4de35bcaf31ccec8bccd36e2d26594d05aad/src/entities/VaultLib.sol#L24-L38
function validateQueuedWithdrawal ( State storage self, bytes32 withdrawalKey ) internal view returns (WithdrawLib.QueuedWithdrawal memory qdWithdrawal ) { qdWithdrawal = self.

withdrawalMap [ withdrawalKey ];...

if ( qdWithdrawal.

start + Constants.

MIN_WITHDRAWAL_DELAY > block.

timestamp ) { revert MinWithdrawDelayNotPassed (); }

- https://github.com/code-423n4/2024-07-karak/blob/53eb78ebda718d752023db4faff4ab1567327db4/src/Core.sol#L248-L256
function finalizeSlashing (SlasherLib.QueuedSlashing memory queuedSlashing ) external nonReentrant whenFunctionNotPaused (Constants.PAUSE_CORE_FINALIZE_SLASHING) { _self ().

finalizeSlashing ( queuedSlashing );...

}

- https://github.com/code-423n4/2024-07-karak/blob/d19a4de35bcaf31ccec8bccd36e2d26594d05aad/src/entities/SlasherLib.sol#L126-L151
function finalizeSlashing (CoreLib.Storage storage self, QueuedSlashing memory queuedSlashing ) external {...

if ( queuedSlashing.

timestamp + Constants.

SLASHING_VETO_WINDOW > block.

timestamp ) { revert MinSlashingDelayNotPassed (); }...

}

## Recommended Mitigation Steps

One way to mitigate this issue is to update the Vault.finishRedeem function to allow the staker’s withdrawal to be finished after the SLASHING_WINDOW of 7 days is passed after such withdrawal is initiated if the DSS does not request any slashing against the corresponding vault within such SLASHING_WINDOW and only enforce the MIN_WITHDRAW_DELAY of 9 days on the withdrawal if the DSS has requested to slash the corresponding vault within such SLASHING_WINDOW.

devdks25 (Karak) commented:

Ideally the protocol doesn’t provide any financial advantage (to any individual) for slashing any operator. So if a staker provides a quite low priority gas fees then it might stay in the mempool for long enough to be slashed for activity which it wasn’t responsible for. So imo the staker’s should provide enough priority gas fees for finishRedeem to prevent the aforementioned. cc: @dewpe dewpe (Karak) commented:

Would re-rate to a non-issue.

From a technical standpoint, the user can withdraw on the exact second that the 9th day hits but they could delay it forever if they really wanted. It’s ultimately up to them to finish it in a timely manner. On the frontend we can state “hey a slashing has started so withdraw your funds right when the 9th day hits”. In normal operations, a DSS would be written so that it slashes users from only the active operator set and that code can be reviewed by stakers and operators before depositing and delegating. If you let them out at 7 days if there isn’t a slashing request then you may run into some timing games if there is a slashing request occurs then.

From a philosophical standpoint, DSS contracts would be audited and agreeing to allocate assets to them or to an operator that allocates to them means you agree to whatever slashing conditions the DSS implements. In theory, the DSS can slash you for 100% right when you register if it wanted.

devdks25 (Karak) disputed MiloTruck (judge) decreased severity to Medium and commented:

Agree that the second scenario is unrealistic - it is entirely the staker’s responsibility to ensure they withdraw their funds on time. If the staker calls finishRedeem() with a priority fee so low that his transaction remains in the mempool for an extended period of time, it is considered a user mistake.

However, the warden does make a valid point that calculating the amount of assets to slash based on totalAssets() will include assets queued for withdrawal.

@dewpe Isn’t it an issue if the DSS ends up slashing a higher percentage of the remaining assets? For example:

Vault has 100 tokens.

User requests a withdrawal of 50 tokens.

DSS calls requestSlashing() to slash 50% of assets, which is calculated as 50 tokens.

User withdraws 50 tokens.

finalizeSlashing() slashes the remaining 50 tokens, leaving 0 tokens in the vault.

Shouldn’t the calculation in requestSlashing() exclude assets in pending withdrawals that have passed SLASHING_WINDOW ? Although in practice this isn’t easy to implement.

devdks25 (Karak) commented:

@MiloTruck, the example seems apt and to mitigate it we are thinking of computing earmarkedStakes during finalizeSlashing, this way all the stakers that are staked will only be slashed.

devdks25 (Karak) commented:

Fixed.

Karak mitigated:

This mitigation computes the slashing amount in finalize slashing.

Status:

Mitigation confirmed. Full details in reports from 0xCiphky, KupiaSec and sl1.

# [M-04] Delayed slashing window and lack of transparency for pending slashes could lead to loss of funds

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-karak-restaking
- **Source snapshot:** competitions/2024-07-karak-restaking/final_report.html

Submitted by 0xCiphky, also found by Takarez, lanrebayode77, and KupiaSec Stakers can provide liquidity by choosing an operator and vault to deposit their assets with. In return, they receive shares based on their deposited amount relative to the total assets in the vault.

DSSs have the right to slash vaults if it feels that an operator has failed to perform its tasks adequately. DSSs are subjected to a delay of 2 days (represented by VETO_WINDOW ) before a slashing can be finalized. This allows the slashing committee to veto a slashing event if it feels that the slashing was unfair.

The handleSlashing function transfers the slashed assets from the vault, effectively reducing the total assets in the vault, which slashes each shareholder relative to the number of shares they own.

function handleSlashing ( IERC20 token, uint256 amount ) external { if ( amount == 0 ) revert ZeroAmount (); if (!

_config ().

supportedAssets [ token ]) revert UnsupportedAsset (); SafeTransferLib.

safeTransferFrom ( address ( token ), msg.

sender, address ( this ), amount ); // Below is where custom logic for each asset lives SafeTransferLib.

safeTransfer ( address ( token ), address ( 0 ), amount ); } The problem with the current system is that the two-day veto window introduces a time gap between when a slash is confirmed and when it is finalized. The slash request records the amount that should be slashed depending on the slash percentage and total assets in the fetchEarmarkedStakes function. Therefore, it is fair to assume that the shareholders at that timestamp should be slashed. However, currently, users who deposit between the slash request and when it is executed will also be slashed and lose value.

Furthermore, the protocol currently lacks any getter methods to warn users of pending slashes for vaults, meaning users could unknowingly deposit into such vaults and lose value in a short period.

In the worst-case scenario, if a vault is fully slashed and the total assets become zero, since the total supply will still be non-zero and previous users will still own shares, any new deposits will instantly lose value, and some funds will be incorrectly allocated to previous users.

## Impact

The current implementation can lead to unfair slashing of users who deposit between the slashing request and its finalization. Users unknowingly depositing into vaults with pending slashes will lose value. With no getter methods available, there is no way for users to identify such vaults and avoid potential losses.

Recommendation Implement getter methods to allow users to check for pending slashes on vaults before making deposits.

If a vault is fully slashed, consider disabling deposits to avoid users from losing value. If disabling deposits is not feasible, make sure users are warned of a slashing event or that the vault is empty with shares still in it, which could cause losses.

devdks25 (Karak) commented:

We plan on running indexers for operator related metrics.

If a vault is fully slashed, consider disabling deposits to avoid users from losing value. If disabling deposits is not feasible, make sure users are warned of a slashing event or that the vault is empty with shares still in it, which could cause losses.

Ideally no staker should deposit in such vault, but still the vault’s deposit can be paused.

cc: @dewpe dewpe (Karak) commented:

Would reclassify to a non-issue.

This is more of a frontend and indexer issue instead of a contract/protocol issue. Operators are already performing due diligence prior to delegating to an DSS. Part of their DD would be if there is any pending slashings. The frontend would make it obvious that there are pending slashings that could affect a users share price.

We could add helper functions on the querier to help protocols building on top have the same observability.

dewpe (Karak) disputed MiloTruck (judge) commented:

The warden has demonstrated how stakers that deposit while a slash is ongoing will lose funds.

Although it is technically the responsibility of stakers to ensure the vault’s current state does not cause them to lose funds when depositing, it isn’t apparent to the regular user that an ongoing slash will cause a loss of funds for new deposits. I also believe it isn’t documented anywhere that users must check if a slash is ongoing before depositing into vaults prior to the audit.

As such, I am inclined to award this as medium severity.

devdks25 (Karak) commented:

To mitigate this the vault deposits will be paused by the core during a queued slashing event and will be unpaused post finalizeSlashing/cancelSlashing.

Additionally a deposit_OVERRIDE_SLASH_PROTECTION method will be exposed to prevent griefing.

Karak mitigated:

This mitigation exposes a getter to determine if a vault’s queued for slashing.

Status:

Mitigation confirmed. Full details in reports from 0xCiphky, KupiaSec and sl1.

# [M-05] Slashings will always fail in some cases

- **Contest:** Karak Restaking
- **Slug:** 2024-07-karak-restaking
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-karak-restaking
- **Source snapshot:** competitions/2024-07-karak-restaking/final_report.html

Submitted by 0xCiphky The requestSlashing function allows a slashing to be requested for a given operator’s deployed vaults staked to the DSS. The slashing request must pass the SLASHING_VETO_WINDOW (2 days) before it can be confirmed, allowing the veto committee to cancel any unfair queued slashing.

This time gap can create situations where the requested slashed amount is no longer possible, as the contract might have had previous withdrawals or been slashed by other DSS’s in that time, reducing its overall balance. The slashAssets function in the vault contract handles this by taking the minimum of the requested slashed amount and the contract balance.

function slashAssets ( uint256 totalAssetsToSlash, address slashingHandler ) external onlyCore returns ( uint256 transferAmount ) { transferAmount = Math.

min ( totalAssets (), totalAssetsToSlash ); // Approve to the handler and then call the handler which will draw the funds SafeTransferLib.

safeApproveWithRetry ( asset (), slashingHandler, transferAmount ); ISlashingHandler ( slashingHandler ).

handleSlashing ( IERC20 ( asset ()), transferAmount ); emit Slashed ( transferAmount ); } However, if the total assets in the contract are zero, the transferAmount will be zero. When this zero value is passed to the handleSlashing function, it will revert due to a check that ensures the amount is not zero.

Since DSS can slash 100% of a vault and vaults can be staked to multiple DSS, it is possible that a vault could be slashed before this slashing request, leaving its total assets as zero.

Another scenario is if there were pending withdrawals that were completed before the slashing, resulting in the total assets being zero after the withdrawals.

function handleSlashing ( IERC20 token, uint256 amount ) external { if ( amount == 0 ) revert ZeroAmount (); if (!

_config ().

supportedAssets [ token ]) revert UnsupportedAsset (); SafeTransferLib.

safeTransferFrom ( address ( token ), msg.

sender, address ( this ), amount ); // Below is where custom logic for each asset lives SafeTransferLib.

safeTransfer ( address ( token ), address ( 0 ), amount ); } As a result, if this slashing reverts and since a slash request can include multiple slashes for different vaults, the entire transaction will revert, blocking other vault slashes as well.

Consider the following scenario:

A DSS requests a slash for all 32 vaults of one of its registered operators.

One of these vaults has its total assets reduced to zero, either due to another DSS slash or because pending withdrawals were completed.

After the two-day veto period, the finalizeSlashing function is called but fails because the slash function of the vault with zero assets reverts.

As a result, the other 31 vaults cannot be slashed either.

Additional Complications:

The operator must create another slash request excluding the problematic vault and go through the process again, including another two-day veto period.

This could affect the slash amount if other users complete withdrawals during this time.

The pending request must also be canceled by the veto; otherwise, if the zero-asset vault becomes non-zero, it could lead to an incorrect double slashing as anyone can finalize a slash request.

## Impact

If the slashAssets function encounters this scenario while attempting to slash assets, it will cause the entire transaction to revert. This will block other slashing requests within the same transaction and lead to additional logical issues as described above.

Mitigation confirmed. Full details in reports from 0xCiphky, KupiaSec and sl1.
