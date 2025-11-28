## Verified Patterns Found: 26

## Verified Patterns Found in following Categories:

- FeeOnTransferAssumption
- StaleOracleAcceptance
- GriefableCallbacks
- ERC20DecimalsMismatch
- AccountingInvariantViolation
- AccessControlOrAuthByPass
- OracleUsingDEXorTWAP
- ReserveOrPriceDesync
- FlashLoanEconomicManipulation
- StandardViolation
- SlippageMissingOrInsufficient



## Summary of Patterns

Unchecked Fee-On-Transfer Collateral Breaks Accounting

Ownership renunciation can permanently lock custodian configuration

Lack of Oracle Staleness or Liveness Checks in Issuer

SlippageMissingOrInsufficient (Unbounded Delayed Redemption)

Implicit 18-Decimal Assumption on Collateral Token

SlippageMissingOrInsufficient in Redemption Service Level

ReserveOrPriceDesync in rebalanceFunds via AUM dependency

Yield distribution DoS via malicious recipient

FeeOnTransferAssumption in processTransfer and rebalanceFunds

Whitelist Bypass via mintFor

StaleOracleAcceptance in iTryIssuer

Flash Loan Manipulation of AUM triggers Liquidity Drain in FastAccessVault

Admin can drain vault collateral via rescueToken

FastAccessVault.rescueToken desyncs issuer accounting causing unbacked yield minting

Missing Oracle Staleness Checks

ERC20DecimalsMismatch in Minting Logic

Unsafe ERC20 transfer calls exclude non-compliant tokens

Missing UUPS Implementation in iTry Token

Acceptance of Stale NAV Prices

Inconsistent SafeERC20 Usage in FastAccessVault

Public rebalanceFunds allows griefing of instant redemptions

Missing Slippage Protection in Fast Redemption

iTryIssuer assumes 1:1 collateral transfer, inflating AUM with fee-on-transfer tokens

ERC-4626 Compliance Violation in StakediTryV2

Unsafe ERC20 Transfer usage despite SafeERC20 import

Protocol insolvency risk with Fee-on-Transfer collateral

## Patterns



 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: iTryIssuer._transferIntoVault

 ### Title
Unchecked Fee-On-Transfer Collateral Breaks Accounting
 ### Description/Code Snippet
The `iTryIssuer` contract accounts for collateral using the input `dlfAmount` but transfers it using `transferFrom`. If the collateral token implements a transfer fee, the `FastAccessVault` receives less than `dlfAmount`, while `_totalDLFUnderCustody` is incremented by the full `dlfAmount`. This permanently decouples the internal accounting from the actual backing assets, potentially leading to insolvency or withdrawal failures.
 ### Static Signals
balance tracking references different token than actual holdings, collateral swapped but totalCollateral unchanged
 ### Assets at Risk
collateralToken
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: FastAccessVault.renounceOwnership

 ### Title
Ownership renunciation can permanently lock custodian configuration
 ### Description/Code Snippet
While the protocol uses `SingleAdminAccessControl` (which prevents role renunciation) for the `iTryIssuer`, the `FastAccessVault` inherits standard OpenZeppelin `Ownable` without overriding `renounceOwnership`. If the admin accidentally calls `renounceOwnership`, the `custodian` address and buffer parameters become immutable. If the set custodian wallet is later compromised or needs rotation, the vault's `rebalanceFunds` function will continue to sweep excess funds to the compromised address indefinitely.
 ### Static Signals
inherits Ownable, no renounceOwnership override
 ### Assets at Risk
excess vault buffer
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: OracleUsingDEXorTWAP

 ### Relevant Function/Location: iTryIssuer.mintFor/redeemFor

 ### Title
Lack of Oracle Staleness or Liveness Checks in Issuer
 ### Description/Code Snippet
The `iTryIssuer` contract depends entirely on `oracle.price()` for critical minting and redemption pricing. The `IOracle` interface and the consumer logic in `iTryIssuer` only retrieve a `uint256` price, with no accompanying timestamp or round data. This makes it impossible for the issuer to verify the freshness of the data. If the underlying oracle stops updating or becomes stale (and does not internally revert), the protocol will continue to mint/redeem at stale prices.
 ### Static Signals
no updatedAt/answeredInRound checks, single spot read
 ### Assets at Risk
protocol collateral
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: iTryIssuer.redeemFor

 ### Title
SlippageMissingOrInsufficient (Unbounded Delayed Redemption)
 ### Description/Code Snippet
In `redeemFor`, if the `FastAccessVault` lacks sufficient liquidity, the function automatically falls back to `_redeemFromCustodian`. This burns the user's iTRY immediately but processes the DLF payout asynchronously via a custodian event (`CustodianTransferRequested`) with no on-chain time bound for settlement. Users calling `redeemITRY` expecting instant liquidity (slippage protection on time/availability) are forced into an illiquid 'IOU' state without opt-in, exposing them to indefinite counterparty latency and opportunity cost.
 ### Static Signals
payout calculated at execution time without minimum bound, value transfers executed without time bounds
 ### Assets at Risk
DLF collateral
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: ERC20DecimalsMismatch

 ### Relevant Function/Location: iTryIssuer.previewMint

 ### Title
Implicit 18-Decimal Assumption on Collateral Token
 ### Description/Code Snippet
The `iTryIssuer` minting and redemption logic calculates `iTRYAmount = netDlfAmount * navPrice / 1e18`. This formula strictly implies that the collateral token (DLF) has 18 decimals (matching the iTRY token). If the underlying DLF token has different decimals (e.g., 6), the minted iTRY amount will be incorrect by orders of magnitude (e.g., undervalued by 1e12), leading to massive user loss or protocol inflation.
 ### Static Signals
mixes token amounts with 18-decimal math unscaled
 ### Assets at Risk
users' funds, collateralToken
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: FastAccessVault.sol.rebalanceFunds

 ### Title
SlippageMissingOrInsufficient in Redemption Service Level
 ### Description/Code Snippet
The protocol allows `redeemITRY` users to be serviced instantly via the `FastAccessVault` or delayed via the Custodian. However, `rebalanceFunds` in `FastAccessVault` is permissionless and can be triggered (e.g., by MEV bots or griefers) to flush 'excess' liquidity to the Custodian immediately before a user's redemption transaction. The `redeemITRY` flow lacks a 'deadline' or 'minServiceLevel' parameter (e.g., 'revert if not from buffer'), forcing users into the delayed Custodian path without their consent, exposing them to prolonged market volatility/slippage off-chain.
 ### Static Signals
deadline omitted, payout calculated at execution time without minimum bound
 ### Assets at Risk
_vaultToken
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ReserveOrPriceDesync

 ### Relevant Function/Location: FastAccessVault.sol.rebalanceFunds

 ### Title
ReserveOrPriceDesync in rebalanceFunds via AUM dependency
 ### Description/Code Snippet
The `rebalanceFunds` function relies on `_issuerContract.getCollateralUnderCustody()` to calculate the `targetBalance`. The Issuer tracks this value via internal accounting (`_totalDLFUnderCustody`) which is never synced with the actual token balances in the Vault or Custodian. If the Vault receives a donation, incurs a fee, or suffers a loss, the `balanceOf(this)` will diverge from the Issuer's tracked state. `rebalanceFunds` then uses this desynced state to determine if it should send 'excess' funds to the custodian or request a top-up, potentially mismanaging liquidity based on a stale or incorrect invariant.
 ### Static Signals
assumes invariant without verifying, no sync() after taxed transfers
 ### Assets at Risk
_vaultToken
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: YieldForwarder.processNewYield

 ### Title
Yield distribution DoS via malicious recipient
 ### Description/Code Snippet
The `iTryIssuer.processAccumulatedYield` function relies on `YieldForwarder.processNewYield`, which performs a direct token transfer to `yieldRecipient`. If `yieldRecipient` is a contract that reverts on receipt (e.g. lacks `onTokenReceived` support or contains reverting logic), the entire yield distribution process reverts. This blocks the protocol from accounting for yield and adjusting the iTRY supply, leading to state desynchronization.
 ### Static Signals
external call without try/catch, transfer to arbitrary address
 ### Assets at Risk
yield
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: FastAccessVault.sol.processTransfer

 ### Title
FeeOnTransferAssumption in processTransfer and rebalanceFunds
 ### Description/Code Snippet
FastAccessVault assumes standard ERC20 behavior where `transfer(amount)` results in the recipient receiving exactly `amount`. In `processTransfer`, if the `_vaultToken` (DLF) has transfer fees or is deflationary, the `_receiver` will receive less than `_amount`, but the `TransferProcessed` event and the calling Issuer's accounting (which burns iTRY 1:1) will record the full `_amount`. Similarly, `rebalanceFunds` calculates `excess` based on `balanceOf` and transfers it to the custodian; if fees apply, the custodian receives less, creating an accounting discrepancy.
 ### Static Signals
accounting based on transfer parameter, uses input amount instead of post-transfer delta, assumes transferFrom(amount) credits exactly amount
 ### Assets at Risk
_vaultToken
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: iTryIssuer.mintFor

 ### Title
Whitelist Bypass via mintFor
 ### Description/Code Snippet
The `iTryIssuer.mintFor` function allows a whitelisted user to mint tokens to a non-whitelisted recipient. The `iTry` token's `_beforeTokenTransfer` hook permits the Minter role to bypass destination whitelist checks, effectively allowing unauthorized users to hold tokens.
 ### Static Signals
role check on msg.sender, no role check on recipient
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: StaleOracleAcceptance

 ### Relevant Function/Location: iTryIssuer.processAccumulatedYield

 ### Title
StaleOracleAcceptance in iTryIssuer
 ### Description/Code Snippet
The iTryIssuer contract retrieves the NAV price via `oracle.price()` in `processAccumulatedYield`, `previewMint`, and `previewRedeem` without performing any freshness checks (e.g., `updatedAt`, `answeredInRound`, or timestamp validation). If the underlying oracle (Redstone or otherwise) provides stale data or stops updating, the system will continue to mint/redeem and distribute yield based on outdated prices. This allows arbitrageurs to exploit price lag or prevents the protocol from reacting to NAV drops, potentially leading to under-collateralization or incorrect yield distribution.
 ### Static Signals
no updatedAt/answeredInRound checks on Chainlink, accepts price older than reasonable threshold
 ### Assets at Risk
collateralToken, iTryToken
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: FlashLoanEconomicManipulation

 ### Relevant Function/Location: FastAccessVault.rebalanceFunds()

 ### Title
Flash Loan Manipulation of AUM triggers Liquidity Drain in FastAccessVault
 ### Description/Code Snippet
The `FastAccessVault.rebalanceFunds()` function calculates the target buffer based on the current AUM (`_issuerContract.getCollateralUnderCustody()`). An attacker can flash-borrow DLF (or iTRY), redeem a large amount to temporarily depress the AUM and the corresponding `_totalDLFUnderCustody` accounting variable, and then call `rebalanceFunds()`. The vault will perceive its current balance as 'excess' relative to the temporarily lowered target and transfer the liquidity to the off-chain custodian. This creates a Denial of Service for legitimate users attempting instant redemptions, forcing them into the slower custodian process.
 ### Static Signals
branches on pool.balanceOf(), rebalance logic depends on manipulable external metric
 ### Assets at Risk
liquidity buffer
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: FastAccessVault.rescueToken

 ### Title
Admin can drain vault collateral via rescueToken
 ### Description/Code Snippet
The `rescueToken` function allows the owner to transfer any ERC20 token, including the `_vaultToken` (DLF) which is the core collateral. There is no check to prevent the `_vaultToken` from being rescued. A compromised or malicious admin key can drain the entire liquidity buffer, breaking the solvency invariant and causing DoS on redemptions.
 ### Static Signals
IERC20(token).safeTransfer, no check for token != _vaultToken
 ### Assets at Risk
_vaultToken
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: ReserveOrPriceDesync

 ### Relevant Function/Location: FastAccessVault.rescueToken

 ### Title
FastAccessVault.rescueToken desyncs issuer accounting causing unbacked yield minting
 ### Description/Code Snippet
The `FastAccessVault` is designed to hold the `_vaultToken` (DLF) as collateral for the `iTryIssuer`. The `iTryIssuer` tracks the total system collateral in `_totalDLFUnderCustody` and uses this value to calculate and mint protocol yield via `processAccumulatedYield`. However, `FastAccessVault.rescueToken` allows the owner to withdraw *any* token, including the `_vaultToken`, without notifying the `iTryIssuer`. If the owner rescues DLF tokens (e.g., to move to cold storage or due to operational error), `_totalDLFUnderCustody` in the Issuer will remain higher than the actual held assets. Consequently, `processAccumulatedYield` will calculate yield based on phantom collateral, minting unbacked iTRY tokens.
 ### Static Signals
assumes invariant without verifying, no sync() after taxed transfers
 ### Assets at Risk
iTRY protocol yield, DLF collateral
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: OracleUsingDEXorTWAP

 ### Relevant Function/Location: iTryIssuer.mintITRY

 ### Title
Missing Oracle Staleness Checks
 ### Description/Code Snippet
`iTryIssuer` relies on `oracle.price()` for minting and redemption without validating data freshness (heartbeat/timestamp). If the off-chain NAV feed is delayed or stale, users can arbitrage the difference between the stale on-chain price and the real-world value.
 ### Static Signals
single spot read, no updatedAt check
 ### Assets at Risk
collateral
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ERC20DecimalsMismatch

 ### Relevant Function/Location: iTryIssuer.mintFor

 ### Title
ERC20DecimalsMismatch in Minting Logic
 ### Description/Code Snippet
The `mintFor` function calculates the iTRY amount using `iTRYAmount = netDlfAmount * navPrice / 1e18`. This formula assumes the `collateralToken` (DLF) has 18 decimals (matching the `1e18` divisor and standard iTRY decimals). If the DLF token has fewer decimals (e.g., 6), the minted iTRY amount will be underscaled by orders of magnitude (e.g., minting dust instead of full value), resulting in immediate value loss for the user. The `FastAccessVault` also relies on consistent decimals for rebalancing calculations.
 ### Static Signals
mixes token amounts with 18-decimal math unscaled
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: FastAccessVault.processTransfer, rebalanceFunds

 ### Title
Unsafe ERC20 transfer calls exclude non-compliant tokens
 ### Description/Code Snippet
The `FastAccessVault` imports and attaches `SafeERC20` for `IERC20`, and correctly uses `safeTransfer` in `rescueToken`. However, in the critical functions `processTransfer` and `rebalanceFunds`, it explicitly calls the raw `_vaultToken.transfer(...)` method and manually checks the boolean return value. This pattern causes transactions to revert for non-compliant ERC20 tokens (like USDT) that do not return a boolean, breaking the vault's core functionality for such assets.
 ### Static Signals
!_vaultToken.transfer, using SafeERC20 but calling .transfer
 ### Assets at Risk
Availability of funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: iTry.N/A

 ### Title
Missing UUPS Implementation in iTry Token
 ### Description/Code Snippet
Documentation specifies the iTRY token is UUPS-upgradeable, but the `iTry` contract does not inherit `UUPSUpgradeable` nor implement `_authorizeUpgrade`. This mismatch prevents valid UUPS upgrades and would brick the proxy if deployed as UUPS.
 ### Static Signals
missing _authorizeUpgrade, missing UUPSUpgradeable inheritance
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: StaleOracleAcceptance

 ### Relevant Function/Location: iTryIssuer.mintFor

 ### Title
Acceptance of Stale NAV Prices
 ### Description/Code Snippet
The `iTryIssuer` calls `oracle.price()` to determine the exchange rate for minting and redemption but performs no validation on the freshness of the returned data (e.g., no `updatedAt` check or grace period enforcement). If the oracle feed becomes stale or the heartbeat is missed, users can exploit the outdated price to arbitrage the protocol against the real-world NAV.
 ### Static Signals
accepts price older than reasonable threshold, no grace period after oracle update before using price
 ### Assets at Risk
protocol collateral
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: FastAccessVault.processTransfer(address,uint256)

 ### Title
Inconsistent SafeERC20 Usage in FastAccessVault
 ### Description/Code Snippet
The `FastAccessVault` contract imports and attaches `SafeERC20` for `IERC20`, but explicitly bypasses it in `processTransfer` and `rebalanceFunds` by using `_vaultToken.transfer(...)` with a manual boolean check. This violates the `SafeERC20` pattern and causes the contract to revert for non-compliant ERC20 tokens (like USDT on mainnet) that do not return a boolean value, potentially locking functionality if such a token is used as the `_vaultToken`.
 ### Static Signals
transfer/transferFrom missing return bool handling, SafeERC20 imported but unused
 ### Assets at Risk
vault funds
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: FastAccessVault.rebalanceFunds

 ### Title
Public rebalanceFunds allows griefing of instant redemptions
 ### Description/Code Snippet
The `rebalanceFunds` function is permissionless and moves 'excess' tokens (balance > target) to the custodian. An attacker can monitor the mempool for large `redeemITRY` transactions that would succeed due to available vault liquidity, front-run them with `rebalanceFunds` to sweep the excess liquidity to the custodian, and force the victim's redemption to fail the `processTransfer` check (falling back to the delayed custodian path).
 ### Static Signals
public visibility, transfer(custodian, excess), no access control
 ### Assets at Risk
user redemption latency, service quality
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: StakediTryFastRedeem.fastRedeem

 ### Title
Missing Slippage Protection in Fast Redemption
 ### Description/Code Snippet
The `StakediTryFastRedeem` contract's `fastRedeem` function allows users to redeem shares for assets instantly but lacks a `minAssetsOut` parameter. Unlike the `iTryIssuer` redemption functions which enforce slippage bounds, `fastRedeem` exposes users to sandwich attacks or share price manipulation/volatility where they could receive significantly fewer assets than expected.
 ### Static Signals
payout calculated at execution time without minimum bound, no minAmountOut parameter in liquidation/redemption
 ### Assets at Risk
users' staked assets
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: iTryIssuer._transferIntoVault

 ### Title
iTryIssuer assumes 1:1 collateral transfer, inflating AUM with fee-on-transfer tokens
 ### Description/Code Snippet
In `iTryIssuer.mintFor`, the contract calls `_transferIntoVault`, which increments `_totalDLFUnderCustody` by the input `dlfAmount` and then executes `collateralToken.transferFrom`. If the `collateralToken` (DLF) implements transfer fees (common in RWA/fund tokens), the `FastAccessVault` receives less than `dlfAmount`. The Issuer's tracking variable `_totalDLFUnderCustody` will permanently exceed the actual collateral balance. This inflated value is used in `processAccumulatedYield` to determine NAV growth, leading to the minting of yield against non-existent collateral.
 ### Static Signals
uses input amount instead of post-transfer delta, accounting based on transfer parameter, not actual balance change
 ### Assets at Risk
iTRY protocol yield
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: StakediTryV2.maxWithdraw

 ### Title
ERC-4626 Compliance Violation in StakediTryV2
 ### Description/Code Snippet
StakediTryV2 disables `withdraw` and `redeem` when the cooldown is active but fails to override `maxWithdraw` and `maxRedeem` to return 0. This violates EIP-4626 invariants, causing integrations (e.g., routers) to revert unexpectedly when relying on `maxWithdraw`.
 ### Static Signals
withdraw reverts conditional, maxWithdraw returns > 0
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: FastAccessVault.processTransfer

 ### Title
Unsafe ERC20 Transfer usage despite SafeERC20 import
 ### Description/Code Snippet
The contract imports `SafeERC20` and applies it to `IERC20`, but uses the raw `transfer()` function in `processTransfer` and `rebalanceFunds` instead of `safeTransfer()`. This will cause the transaction to revert if the `_vaultToken` (DLF) does not return a boolean value (e.g. USDT) or if it returns false without reverting (though the boolean check handles false, the void return is the main risk).
 ### Static Signals
_vaultToken.transfer(_receiver, _amount), using SafeERC20 for IERC20
 ### Assets at Risk
_vaultToken
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: FastAccessVault.rebalanceFunds

 ### Title
Protocol insolvency risk with Fee-on-Transfer collateral
 ### Description/Code Snippet
The `rebalanceFunds` logic calculates the `targetBufferBalance` based on the issuer's AUM (`getCollateralUnderCustody`). The issuer updates AUM based on the `amount` parameter of transfers, not the actual received balance. If the `_vaultToken` (DLF) has transfer fees, the Vault's actual balance will be lower than the Accounting AUM implies. This causes `rebalanceFunds` to calculate an inflated target, potentially preventing excess liquidity from being released or constantly requesting unnecessary top-ups.
 ### Static Signals
_calculateTargetBufferBalance, uses external accounting instead of balance delta
 ### Assets at Risk
liquidity buffer efficiency
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

