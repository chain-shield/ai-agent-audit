## Verified Patterns Found: 17

## Verified Patterns Found in following Categories:

- AccountingInvariantViolation
- CrossChainMessageSpoofing
- FeeOnTransferAssumption
- StandardViolation
- FeeAccountingDrift
- GriefableCallbacks
- AccessControlOrAuthByPass
- SlippageMissingOrInsufficient
- FinalityOrReplayAcrossDomains
- UncheckedLowLevelCallResults



## Summary of Patterns

Premature unstake requests can block ordered LayerZero channels

Checks-Effects-Interactions Violation in Cooldown Initiation

Spoofing of `composeFrom` in `handleCompose` allows Cooldown Griefing

Excess native fees permanently locked in wiTryVaultComposer

Strict slippage check in cross-chain operations causes DoS for dust amounts

Slippage Check Ignored in Cross-Chain Cooldown Initiation

Griefable Cross-chain Unstake via Rigid Fee Forwarding

Failed compose actions may permanently lock funds due to expensive refund logic

Fee-on-Transfer Token Incompatibility in VaultComposerSync

Trapped excess fees in handleCompose

Slippage protection bypass in fastRedeem allows loss of user funds

Cross-chain operations revert due to strict slippage check on dust

Griefing/DoS of Cooldowns via Authenticated Data Spoofing

Fee-on-transfer tokens break deposit flow

Unrecoverable Unstake Requests due to strict fee check in `_handleUnstake`

User-provided `extraOptions` ignored in `_handleUnstake` prevents gas adjustment

Missing slippage check in `_fastRedeem` exposes users to value loss

## Patterns



 ### Issue Type: FinalityOrReplayAcrossDomains

 ### Relevant Function/Location: wiTryVaultComposer._handleUnstake

 ### Title
Premature unstake requests can block ordered LayerZero channels
 ### Description/Code Snippet
The `_handleUnstake` function calls `vault.unstakeThroughComposer`, which reverts if the user's cooldown is incomplete or assets are zero. In LayerZero V2, if the OApp is configured for ordered execution (a common default to ensure nonce sequencing), a single reverting message blocks the delivery channel for all subsequent messages from that source chain. An attacker can intentionally send a premature unstake request (valid on source) that reverts on destination, creating a Denial-of-Service for the bridge path.
 ### Static Signals
revert InvalidCooldown(), no try/catch
 ### Assets at Risk
bridge availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: StakediTryCrosschain.cooldownSharesByComposer

 ### Title
Checks-Effects-Interactions Violation in Cooldown Initiation
 ### Description/Code Snippet
The `cooldownSharesByComposer` function in `StakediTryCrosschain` calls `_withdraw` (which transfers assets to the silo) *before* updating the user's cooldown state (`cooldowns[redeemer]`). While the current asset (iTRY) is a standard ERC20, if it were upgraded to have transfer hooks (e.g., ERC777/ERC1363), this ordering would allow reentrancy, potentially violating the CEI pattern. The function lacks a `nonReentrant` modifier, unlike the parent `withdraw` function.
 ### Static Signals
state update after external call, missing nonReentrant modifier
 ### Assets at Risk
Vault liquidity
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: CrossChainMessageSpoofing

 ### Relevant Function/Location: wiTryVaultComposer.handleCompose

 ### Title
Spoofing of `composeFrom` in `handleCompose` allows Cooldown Griefing
 ### Description/Code Snippet
The `wiTryVaultComposer` contract relies on `handleCompose` to execute sensitive cross-chain actions like `INITIATE_COOLDOWN`. It determines the user identity (`redeemer`) using the `_composeFrom` argument, which `VaultComposerSync` extracts from the composed message payload (`_composeMsg`).

However, in the standard `OFTCore` implementation, the compose payload is constructed directly from the user-supplied `SendParam.composeMsg` without injecting the authenticated sender address (`_origin.sender`). As `OFTComposeMsgCodec.encode` in `_lzReceive` merely appends this user payload to the header, and `VaultComposerSync` blindly reads the bytes at the `COMPOSE_FROM` offset from this payload, a malicious user can craft a `composeMsg` that places a victim's address in the `composeFrom` slot.

By spoofing `composeFrom` as a victim's address and bridging a negligible amount of shares, an attacker can trigger `_initiateCooldown` for the victim. This resets the victim's cooldown timer (e.g., to 90 days), preventing them from withdrawing their funds indefinitely (Denial of Service/Griefing).
 ### Static Signals
_composeFrom derived from user payload, handleCompose trusts _composeFrom
 ### Assets at Risk
user funds (via permanent lock/DoS)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: wiTryVaultComposer._fastRedeem

 ### Title
Excess native fees permanently locked in wiTryVaultComposer
 ### Description/Code Snippet
In `wiTryVaultComposer`, the `_fastRedeem` function (and the inherited `_depositAndSend`) invokes `_send`, which calls `IOFT.send`. The `_refundAddress` parameter passed to `_send` is hardcoded to `address(this)`. When LayerZero executes the cross-chain transaction, any excess `msg.value` (native gas) provided for fees is refunded to `_refundAddress`. Since `address(this)` is the contract itself and it lacks a permissionless function for users to withdraw ETH, these funds are permanently locked/stuck in the contract, accessible only to the Admin via `rescueToken`. This violates the standard pattern where refunds should return to the `msg.sender` or the originating user.
 ### Static Signals
_send(..., address(this)), address(this) as refund address
 ### Assets at Risk
native currency (ETH/GAS)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: wiTryVaultComposer._handleUnstake

 ### Title
Strict slippage check in cross-chain operations causes DoS for dust amounts
 ### Description/Code Snippet
In `wiTryVaultComposer`, the functions `_fastRedeem` and `_handleUnstake` retrieve an asset amount from the vault and attempt to send it cross-chain via the `ASSET_OFT` (iTRY) adapter. Both functions enforce strict equality for slippage protection by setting `_sendParam.minAmountLD = assets`. 

However, the LayerZero `OFTCore` logic (inherited by the iTRY adapter) typically uses a shared decimal configuration (default 6) that is lower than the ERC20 token's decimals (18). The `_debit` operation in the OFT adapter truncates any dust (amounts smaller than the shared decimal precision, e.g., < 1e12 wei) before sending. 

Because `_fastRedeem` and `_handleUnstake` require the amount received by the OFT logic (`amountReceivedLD`) to be greater than or equal to the exact vault amount (`minAmountLD`), any transaction involving an amount with dust will revert with `SlippageExceeded`. This permanently bricks the unstaking and fast redeem flows for users with fractional balances.
 ### Static Signals
_sendParam.minAmountLD = assets, revert SlippageExceeded
 ### Assets at Risk
User staked funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: wiTryVaultComposer._initiateCooldown

 ### Title
Slippage Check Ignored in Cross-Chain Cooldown Initiation
 ### Description/Code Snippet
In `wiTryVaultComposer.handleCompose`, the `INITIATE_COOLDOWN` path decodes a `SendParam` struct which includes a `minAmountLD` (minimum assets to receive/lock). However, the internal `_initiateCooldown` function completely ignores this parameter. It calls `cooldownSharesByComposer` which locks assets based on the current share price. If the share price decreases significantly or is manipulated before the transaction executes on the hub chain, the user may lock far fewer assets than their specified minimum tolerance without the transaction reverting.
 ### Static Signals
minAmountLD ignored, no slippage assert
 ### Assets at Risk
user assets
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: wiTryVaultComposer._handleUnstake

 ### Title
Griefable Cross-chain Unstake via Rigid Fee Forwarding
 ### Description/Code Snippet
The `_handleUnstake` function in `wiTryVaultComposer` calls `_send` to return assets to the spoke chain. `_send` relies on `msg.value` (the `returnTripAllocation` passed from the source chain) to pay the LayerZero fee. If gas prices on the hub chain increase between the request and execution, `msg.value` will be insufficient, causing `_send` (and thus `_handleUnstake`) to revert. This leaves the user's assets unstaked but stuck in the Composer/Vault on the hub chain, creating a denial-of-service where the user cannot retrieve their funds without complex manual intervention.
 ### Static Signals
_send(..., msg.value, ...), no try/catch around external hook, callback success required for core flow to proceed
 ### Assets at Risk
User unstaked assets
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UncheckedLowLevelCallResults

 ### Relevant Function/Location: VaultComposerSync.lzCompose

 ### Title
Failed compose actions may permanently lock funds due to expensive refund logic
 ### Description/Code Snippet
In `VaultComposerSync.lzCompose`, if `handleCompose` reverts, the contract attempts to `_refund` by sending an OFT message back to the source. OFT sends are gas and value intensive. If `handleCompose` consumes most of the gas before reverting (e.g. Out Of Gas error), or if the provided `msg.value` is insufficient for the refund leg (which may be more expensive than the forward leg), `_refund` will revert. This causes the entire LayerZero delivery transaction to fail, preventing the refund and permanently locking the user's funds in the Composer contract unless the message can be retried with higher parameters.
 ### Static Signals
catch, _refund, IOFT.send
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: VaultComposerSync._depositAndSend

 ### Title
Fee-on-Transfer Token Incompatibility in VaultComposerSync
 ### Description/Code Snippet
The `_depositAndSend` function in `VaultComposerSync` assumes that the amount of assets transferred to it via `safeTransferFrom` equals the amount received. It subsequently calls `VAULT.deposit` with the original `_assetAmount`. If the underlying `ASSET_ERC20` (iTRY) has a transfer fee (now or in a future upgrade), the contract will receive less than `_assetAmount`, causing `VAULT.deposit` to revert due to insufficient balance, effectively breaking deposits.
 ### Static Signals
safeTransferFrom(msg.sender, address(this), _assetAmount), VAULT.deposit(_assetAmount, address(this)), uses input amount instead of post-transfer delta
 ### Assets at Risk
User deposits
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeAccountingDrift

 ### Relevant Function/Location: wiTryVaultComposer.handleCompose

 ### Title
Trapped excess fees in handleCompose
 ### Description/Code Snippet
The `handleCompose` function calls `_depositAndSend` and `_fastRedeem` passing `address(this)` as the `_refundAddress` for the LayerZero cross-chain operation. Any excess `msg.value` (native fee refunds) returned by the IOFT `send` operation will be sent to the contract address instead of the user or the `composeFrom` address. These funds accumulate in the contract and cannot be retrieved by the user.
 ### Static Signals
address(this) as refund address, excess msg.value not returned to sender
 ### Assets at Risk
user native ETH
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: wiTryVaultComposer._fastRedeem

 ### Title
Slippage protection bypass in fastRedeem allows loss of user funds
 ### Description/Code Snippet
In `wiTryVaultComposer._fastRedeem`, the `_sendParam.minAmountLD` provided by the user (decoded from `_composeMsg` and intended to protect against redemption slippage) is strictly overwritten with the `assets` amount returned from the vault redemption. This forces the cross-chain send to accept whatever amount the vault returns, even if it is significantly lower than the user's expectation (e.g. due to variable fees or NAV changes), effectively disabling slippage protection for the vault redemption step.
 ### Static Signals
_sendParam.minAmountLD = assets, _sendParam.amountLD = assets
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: wiTryVaultComposer._handleUnstake

 ### Title
Cross-chain operations revert due to strict slippage check on dust
 ### Description/Code Snippet
In `wiTryVaultComposer`, both `_handleUnstake` and `_fastRedeem` explicitly set `_sendParam.minAmountLD` equal to the exact `assets` amount returned from the vault. Since `iTRY` has 18 decimals and LayerZero OFTs typically use 6 shared decimals, any asset amount containing 'dust' (fractions that are lost during Local->Shared->Local conversion) will cause `_send` to revert with `SlippageExceeded` (because `amountReceivedLD < minAmountLD`). This effectively causes a Denial of Service for unstaking and fast redemptions for almost all users, as calculated redemption amounts will rarely align perfectly with shared decimal precision.
 ### Static Signals
_sendParam.minAmountLD = assets, _sendParam.amountLD = assets
 ### Assets at Risk
user assets (stuck in silo or composer)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: wiTryVaultComposer.handleCompose

 ### Title
Griefing/DoS of Cooldowns via Authenticated Data Spoofing
 ### Description/Code Snippet
The `wiTryVaultComposer` relies on `_message.composeFrom()` in `lzCompose` -> `handleCompose` to identify the user initiating a cooldown. However, when using the standard OFT `send()` function with composition, the `composeMsg` (and thus the bytes extracted as `composeFrom`) is fully controlled by the caller (user) on the source chain. An attacker can send a negligible amount of shares (e.g., 1 wei) and set `composeFrom` to a victim's address. This triggers `_initiateCooldown` -> `VAULT.cooldownSharesByComposer`, which overwrites the victim's `cooldownEnd` timestamp to `block.timestamp + duration`. Repeated attacks can permanently lock the victim's funds by continually extending the cooldown period.
 ### Static Signals
_message.composeFrom() used as authority, cooldowns[redeemer].cooldownEnd = ... (overwrite)
 ### Assets at Risk
User staked funds (indefinite lock)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: VaultComposerSync._deposit

 ### Title
Fee-on-transfer tokens break deposit flow
 ### Description/Code Snippet
The `VaultComposerSync` logic (inherited by `wiTryVaultComposer`) assumes that the amount specified in the LayerZero message matches the balance credited to the contract. If the asset token (iTRY) implements transfer fees (as suggested by the protocol's configurable fee capabilities), the contract receives less than the specified `_amount`. Consequently, the subsequent `VAULT.deposit(_amount)` call will revert due to insufficient balance, causing the cross-chain deposit to fail and potentially locking the funds if the refund mechanism also encounters balance issues.
 ### Static Signals
accounting based on transfer parameter, not actual balance change, assumes transferFrom(amount) credits exactly amount
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: wiTryVaultComposer._handleUnstake

 ### Title
Unrecoverable Unstake Requests due to strict fee check in `_handleUnstake`
 ### Description/Code Snippet
The `_handleUnstake` function in `wiTryVaultComposer` initiates a cross-chain send using `_send` which calls `IOFT.send{value: msg.value}`. The `OFTCore` implementation strictly requires `msg.value` to exactly match the calculated native fee. Since `msg.value` delivered to `lzReceive` is fixed by the source chain options (returnTripAllocation), any increase in required fees (e.g., gas spikes) between the source transaction and Hub execution causes `IOFT.send` to revert. Because `msg.value` cannot be increased on a LayerZero retry, the unstake request becomes permanently stuck, and the user's fee is lost.
 ### Static Signals
IOFT(_oft).send{value: msg.value}, strict fee equality check in OFTCore
 ### Assets at Risk
native ETH (fees)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: wiTryVaultComposer._handleUnstake

 ### Title
User-provided `extraOptions` ignored in `_handleUnstake` prevents gas adjustment
 ### Description/Code Snippet
The `_handleUnstake` function decodes an `UnstakeMessage` which contains an `extraOptions` field, intended to configure LayerZero execution options (e.g., gas limit) for the return message to the spoke chain. However, the function ignores this field and instead constructs `SendParam` using `OptionsBuilder.newOptions()` (empty options). If the target chain requires specific enforced options that are not configured by default in the Adapter, or if a user needs to supply extra gas for a complex receiver, the return message will fail (out of gas) on the destination, with no way for the user to influence the parameters.
 ### Static Signals
OptionsBuilder.newOptions() used instead of decoded options, abi.decode(..., UnstakeMessage) but field unused
 ### Assets at Risk
cross-chain message reliability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: wiTryVaultComposer._fastRedeem

 ### Title
Missing slippage check in `_fastRedeem` exposes users to value loss
 ### Description/Code Snippet
In `_fastRedeem`, the function decodes a `SendParam` struct from the user's composed message, which includes a `minAmountLD` intended to enforce a minimum output amount (slippage protection) for the entire operation. However, the function fails to check if the `assets` obtained from `VAULT.fastRedeemThroughComposer` satisfy this minimum. Instead, it overwrites `_sendParam.minAmountLD` with the actual redeemed `assets` amount before initiating the cross-chain send. This completely bypasses the user's slippage protection for the share-to-asset redemption step (which incurs a variable fee) and exposes them to receiving fewer assets than expected.
 ### Static Signals
_sendParam.minAmountLD = assets
 ### Assets at Risk
User funds (assets)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

