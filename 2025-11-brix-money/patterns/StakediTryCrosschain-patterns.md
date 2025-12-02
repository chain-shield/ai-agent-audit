## Verified Patterns Found: 41

## Verified Patterns Found in following Categories:

- AccountingInvariantViolation
- ReserveOrPriceDesync
- Reentrancy
- StorageCollisionOrSelectorClash
- StaleOracleAcceptance
- InitOrderOrUnintialized
- GovernanceDelegationFlaw
- ERC4626SharePriceMismatch
- MaturityorGatingByPass
- ERC20DecimalsMismatch
- FeeOnTransferAssumption
- SlippageMissingOrInsufficient
- OracleUsingDEXorTWAP
- PricePrecisionOrRoundingError
- TimelockEdgeCase
- ExternalCallAfterStateChange
- AccessControlOrAuthByPass
- UpgradeAuthBypass
- GriefableCallbacks
- StandardViolation



## Summary of Patterns

Cross-chain Unstake Lockup when Cooldown Disabled

Cooldown Extension Griefing via Cross-Chain Composition

Custodian Address Desynchronization between Issuer and Vault

Missing UUPS Implementation prevents iTRY Token Upgrades

Forced cooldown reset via Composer enables indefinite fund lockup

Oracle Spec/Implementation Mismatch leading to Accounting Violation

Cooldown Reset Griefing via Cross-chain Composer

Inconsistent Blacklist Management Roles in Spoke OFTs

Unsafe ERC20 Transfer Implementation

Storage Collision Risk in Upgradeable Contracts

ERC4626 Compliance Violation in MaxWithdraw

Fast redemption lacks slippage protection against fee changes

Reentrancy and CEI violation in cross-chain cooldown initiation

Missing staleness checks on Oracle price feed

Unsafe Token Transfer in Yield Forwarder

iTry token contract missing UUPS upgrade authorization

Unsafe decimal assumption in collateral conversion

Hardcoded 18-decimal assumption for collateral token leads to value loss or inflation

Missing slippage protection (deadline) in mint and redeem operations

Storage Gap Missing in Upgradeable Parent

UUPS Upgradeability Implementation Mismatch

Stale Oracle Price Acceptance

External Call to Yield Receiver After State Change Without Reentrancy Guard

Redstone Oracle integration fails to propagate payload, breaking price updates

Uninitialized Parent Contract State

Potential ERC20 Decimals Mismatch in Minting Logic

ERC4626 maxRedeem Violates Specification

Issuer accounting incompatible with fee-on-transfer collateral

Lack of staleness validation for NAV Oracle

Oracle Price Staleness and Validation Missing

Potential rounding bias in Fast Redeem fee accounting

Cross-Chain DoS via Blacklisted Recipient in iTryTokenOFT

mintFor allows bypassing whitelist holding restrictions

Unbounded yield minting due to missing slippage/cap on Oracle price

Decimal mismatch handling in minting logic

Yield distribution relies on internal accounting disjoint from actual vault balance

Accounting Invariant Violation in Yield Calculation

Accounting Invariant Violation via Yield Minting

Missing slippage protection in fast redemption

ERC4626 Compliance Violation in Withdrawal Logic

Fee-on-Transfer Token Support Missing in Collateral Handling

## Patterns



 ### Issue Type: TimelockEdgeCase

 ### Relevant Function/Location: StakediTryCrosschain.unstakeThroughComposer

 ### Title
Cross-chain Unstake Lockup when Cooldown Disabled
 ### Description/Code Snippet
In `StakediTryCrosschain`, the `unstakeThroughComposer` function checks `block.timestamp >= userCooldown.cooldownEnd` but fails to check if `cooldownDuration == 0`. In `StakediTryV2`, the local `unstake` function allows immediate withdrawal if `cooldownDuration == 0`. This inconsistency means that if governance disables the cooldown (emergency unlock), cross-chain users remain locked out until their original time passes, while local users can exit immediately.
 ### Static Signals
missing cooldownDuration == 0 check, inconsistent condition vs StakediTryV2.unstake
 ### Assets at Risk
User stake (wiTRY)
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: StakediTryCrosschain._startComposerCooldown

 ### Title
Cooldown Extension Griefing via Cross-Chain Composition
 ### Description/Code Snippet
In `StakediTryCrosschain`, `_startComposerCooldown` unconditionally overwrites `cooldowns[redeemer].cooldownEnd` with `block.timestamp + cooldownDuration`. If the `COMPOSER_ROLE` (LayerZero adapter) allows arbitrary users to trigger cross-chain deposits to a target `redeemer`, an attacker can repeatedly send dust amounts to a victim, perpetually resetting their cooldown timer and locking their funds indefinitely.
 ### Static Signals
cooldownEnd = block.timestamp + duration, overwrites existing state, triggered by composer
 ### Assets at Risk
user staked assets
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GovernanceDelegationFlaw

 ### Relevant Function/Location: FastAccessVault.setCustodian

 ### Title
Custodian Address Desynchronization between Issuer and Vault
 ### Description/Code Snippet
The `iTryIssuer` and `FastAccessVault` contracts both store the `custodian` address but manage it via different roles (`_INTEGRATION_MANAGER_ROLE` vs `onlyOwner`) and separate setters. If these addresses get out of sync, `FastAccessVault.rebalanceFunds` will send excess collateral to the wrong (old) custodian, while the Issuer expects the new custodian to handle it, leading to operational failure and potential fund loss.
 ### Static Signals
custodian stored in multiple contracts, setters protected by different roles
 ### Assets at Risk
Collateral (DLF)
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: UpgradeAuthBypass

 ### Relevant Function/Location: iTry.N/A

 ### Title
Missing UUPS Implementation prevents iTRY Token Upgrades
 ### Description/Code Snippet
The iTRY token documentation states it is 'UUPS-upgradeable', but the `iTry` contract does not inherit from `UUPSUpgradeable` nor does it implement the `_authorizeUpgrade` function. If deployed behind a UUPS proxy (as intended), calls to `upgradeTo` will fail because the implementation contract does not expose the necessary upgrade interface, rendering the token immutable and breaking the upgrade path.
 ### Static Signals
contract iTry is ... ERC20BurnableUpgradeable, missing UUPSUpgradeable inheritance, missing _authorizeUpgrade
 ### Assets at Risk
Upgrade capability
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: StakediTryCrosschain.cooldownSharesByComposer

 ### Title
Forced cooldown reset via Composer enables indefinite fund lockup
 ### Description/Code Snippet
The `cooldownSharesByComposer` function unconditionally updates `cooldowns[redeemer].cooldownEnd` to `block.timestamp + cooldownDuration` whenever called. An attacker can send negligible amounts of assets/shares cross-chain via the Composer to a target user who already has a cooldown pending. This action resets the user's cooldown timer, and if repeated just before the cooldown expires, can lock the user's funds indefinitely.
 ### Static Signals
cooldownEnd = block.timestamp + cooldownDuration, input address redeemer, no check for existing cooldown
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: iTryIssuer.mintFor

 ### Title
Oracle Spec/Implementation Mismatch leading to Accounting Violation
 ### Description/Code Snippet
The `IOracle` interface documentation specifies that `price()` returns 'the price of 1 unit of iTRY quoted in 1 unit of collateral token' (iTRY/DLF rate). However, the `iTryIssuer` implementation uses `price()` as if it returns the value of 1 unit of Collateral in iTRY (DLF/iTRY rate). If the Oracle adheres to its spec, the Issuer uses the inverse price, breaking the 1:1 backing invariant and causing incorrect minting/redemption values.
 ### Static Signals
balance tracking references different token than actual holdings
 ### Assets at Risk
collateralToken
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: MaturityorGatingByPass

 ### Relevant Function/Location: StakediTryCrosschain._startComposerCooldown

 ### Title
Cooldown Reset Griefing via Cross-chain Composer
 ### Description/Code Snippet
In `StakediTryCrosschain`, the `_startComposerCooldown` function unconditionally resets a user's `cooldownEnd` timestamp to `block.timestamp + duration`. An attacker can perpetually block a victim from unstaking by triggering small, repeated cross-chain redemption requests (via the Composer) to the victim's address, forcing the cooldown timer to reset repeatedly.
 ### Static Signals
cooldown period bypassable via reentrancy or state manipulation, challenge mechanism fails due to state confusion
 ### Assets at Risk
wiTRY
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: iTryTokenOFT.addBlacklistAddress

 ### Title
Inconsistent Blacklist Management Roles in Spoke OFTs
 ### Description/Code Snippet
The `iTryTokenOFT` contract on spoke chains relies on `onlyOwner` for blacklisting, whereas the Hub `iTry` contract uses a dedicated `BLACKLIST_MANAGER_ROLE` and the sibling `wiTryOFT` uses a `blackLister` delegate. This inconsistency prevents the protocol from delegating iTRY blacklist operations on spoke chains to the designated manager, forcing usage of the supreme admin key for routine compliance.
 ### Static Signals
onlyOwner instead of specific role, inconsistent vs wiTryOFT/iTry
 ### Assets at Risk
Operational security
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: FastAccessVault.processTransfer

 ### Title
Unsafe ERC20 Transfer Implementation
 ### Description/Code Snippet
The `FastAccessVault` and `YieldForwarder` contracts explicitly check the boolean return value of `transfer` and `transferFrom` (e.g., `if (!token.transfer(...)) revert`). This causes transactions to revert for non-compliant ERC20 tokens (like USDT) that do not return a boolean value, resulting in a denial of service for core protocol functions if such tokens are used as collateral or yield.
 ### Static Signals
transfer/transferFrom missing return bool, transfer() doesn't return bool
 ### Assets at Risk
DLF, Yield Tokens
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: StorageCollisionOrSelectorClash

 ### Relevant Function/Location: iTry.N/A

 ### Title
Storage Collision Risk in Upgradeable Contracts
 ### Description/Code Snippet
The upgradeable contracts `iTry` and `SingleAdminAccessControlUpgradeable` do not define a `__gap` storage variable. `iTry` defines its own state (`transferState`) after inheriting `SingleAdminAccessControlUpgradeable`. If `SingleAdminAccessControlUpgradeable` (or its base contracts) is upgraded in the future to include new state variables, it will overwrite the storage slots used by `iTry`, resulting in state corruption.
 ### Static Signals
Upgradeable inheritance, Missing __gap
 ### Assets at Risk
contract storage
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: StakediTryV2.maxWithdraw

 ### Title
ERC4626 Compliance Violation in MaxWithdraw
 ### Description/Code Snippet
In `StakediTryV2`, the `withdraw` and `redeem` functions revert when `cooldownDuration > 0` due to the `ensureCooldownOff` modifier. However, `maxWithdraw` and `maxRedeem` are not overridden to return 0 in this state. This violates the ERC4626 invariant that `maxWithdraw` must return the amount that can be withdrawn in the current block, potentially causing DoS in integrations that rely on `maxWithdraw` for checks.
 ### Static Signals
withdraw reverts, maxWithdraw returns > 0
 ### Assets at Risk
integrator funds, availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: StakediTryFastRedeem.fastRedeem, fastWithdraw

 ### Title
Fast redemption lacks slippage protection against fee changes
 ### Description/Code Snippet
The `StakediTryFastRedeem` contract allows users to bypass the cooldown by paying a fee (`fastRedeemFeeInBPS`), which can be set up to 20% by the admin. The `fastRedeem` and `fastWithdraw` functions do not accept a `minAssetsOut` or `maxSharesIn` parameter. If the fee is updated (front-run) or if the user is unaware of a recent fee hike, they may receive significantly fewer assets than expected.
 ### Static Signals
payout calculated at execution time without minimum bound, fee variable used in payout
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: Reentrancy

 ### Relevant Function/Location: StakediTryCrosschain._startComposerCooldown

 ### Title
Reentrancy and CEI violation in cross-chain cooldown initiation
 ### Description/Code Snippet
In `StakediTryCrosschain._startComposerCooldown`, the contract calls `_withdraw` (which interacts externally by transferring assets) *before* updating the `cooldowns` state mapping. While `nonReentrant` protects against direct reentrancy, this violation of the Checks-Effects-Interactions pattern creates a risk if the asset transfer triggers any hooks or if future upgrades remove the guard.
 ### Static Signals
untrusted call before all updates, state change after external call
 ### Assets at Risk
vault liquidity
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: OracleUsingDEXorTWAP

 ### Relevant Function/Location: iTryIssuer.mintITRY

 ### Title
Missing staleness checks on Oracle price feed
 ### Description/Code Snippet
The `iTryIssuer` contract retrieves the NAV price via `oracle.price()` in `mintITRY`, `redeemITRY`, and `processAccumulatedYield` without performing any checks on the data's freshness or timestamp. If the connected oracle (e.g., Chainlink or Redstone adapter) provides stale data, the protocol is vulnerable to arbitrage and incorrect yield distribution.
 ### Static Signals
no updatedAt/answeredInRound checks on Chainlink, single spot read
 ### Assets at Risk
collateral, treasury
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: YieldForwarder.processNewYield

 ### Title
Unsafe Token Transfer in Yield Forwarder
 ### Description/Code Snippet
The `YieldForwarder` contract uses the standard `IERC20.transfer` method which expects a boolean return value. If the immutable `yieldToken` is deployed as a non-standard ERC20 (e.g., USDT) that does not return a boolean, this function will revert, permanently blocking the `processAccumulatedYield` flow in the `iTryIssuer`.
 ### Static Signals
transfer/transferFrom missing return bool
 ### Assets at Risk
yield rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: iTry.N/A

 ### Title
iTry token contract missing UUPS upgrade authorization
 ### Description/Code Snippet
The `iTry` contract is described as UUPS-upgradeable in documentation and initialized as an upgradeable contract, but it fails to inherit `UUPSUpgradeable` or implement the required `_authorizeUpgrade` function. This omission renders the contract non-upgradeable if deployed behind a UUPS proxy, violating the intended design.
 ### Static Signals
missing _authorizeUpgrade, upgradeable dependencies without UUPS inheritance
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: ERC20DecimalsMismatch

 ### Relevant Function/Location: iTryIssuer.mintFor

 ### Title
Unsafe decimal assumption in collateral conversion
 ### Description/Code Snippet
The `mintFor` and `redeemFor` functions calculate token amounts using `navPrice / 1e18` (mint) and `* 1e18 / navPrice` (redeem). This logic hardcodes an assumption that the `collateralToken` (DLF) has 18 decimals. If the DLF token uses a different precision (e.g., 6 decimals, common for RWA/stablecoins), the calculated amounts will be incorrect by orders of magnitude (e.g., factor of 10^12), leading to either massive loss of user funds or unbacked minting of iTRY.
 ### Static Signals
mixes token amounts with 18-decimal math unscaled
 ### Assets at Risk
collateralToken, iTryToken
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: PricePrecisionOrRoundingError

 ### Relevant Function/Location: iTryIssuer.previewMint, previewRedeem, mintFor, redeemFor

 ### Title
Hardcoded 18-decimal assumption for collateral token leads to value loss or inflation
 ### Description/Code Snippet
The `iTryIssuer` calculations for minting (`_mint`) and redeeming (`previewRedeem`) assume the collateral token (DLF) has 18 decimals, matching the iTRY token. The formula `netDlfAmount * navPrice / 1e18` scales strictly by 1e18. If the DLF token uses 6 decimals (common for stablecoins/funds), users will mint only a tiny fraction (1e-12) of the intended iTRY value. If DLF uses >18 decimals, users mint vastly inflated amounts. There is no check for `collateralToken.decimals()` in the constructor.
 ### Static Signals
scaling by 1e18, no decimals() check
 ### Assets at Risk
user funds, protocol solvency
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: iTryIssuer.mintITRY, redeemITRY

 ### Title
Missing slippage protection (deadline) in mint and redeem operations
 ### Description/Code Snippet
`mintITRY` and `redeemITRY` functions accept a `minAmountOut` parameter but lack a `deadline` timestamp parameter. This allows miners or MEV bots to hold signed transactions in the mempool and execute them at a later time when the NAV price has moved unfavorably for the user (but within the `minAmountOut` bounds) or to grief the user by executing stale transactions.
 ### Static Signals
deadline omitted, external call
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StorageCollisionOrSelectorClash

 ### Relevant Function/Location: SingleAdminAccessControlUpgradeable.N/A

 ### Title
Storage Gap Missing in Upgradeable Parent
 ### Description/Code Snippet
The `SingleAdminAccessControlUpgradeable` abstract contract inherits `AccessControlUpgradeable` but does not define a `__gap` variable. The `iTry` contract inherits this. Future upgrades to `SingleAdminAccessControlUpgradeable` (e.g., adding state variables) would shift the storage layout of `iTry` (specifically `transferState`), causing storage collisions and state corruption.
 ### Static Signals
upgradeable contract, missing __gap
 ### Assets at Risk
contract storage, admin privileges
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: iTry._authorizeUpgrade

 ### Title
UUPS Upgradeability Implementation Mismatch
 ### Description/Code Snippet
The `iTry` token is documented as UUPS-upgradeable and inherits `ERC20PermitUpgradeable`, but it fails to inherit `UUPSUpgradeable` or implement the critical `_authorizeUpgrade` function. If deployed as a UUPS proxy, the contract will either fail to initialize properly or be permanently non-upgradeable (bricked upgrade path), violating the intended standard and governance control.
 ### Static Signals
upgradeable import, missing _authorizeUpgrade, missing UUPSUpgradeable inheritance
 ### Assets at Risk
protocol governance
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: StaleOracleAcceptance

 ### Relevant Function/Location: iTryIssuer.processAccumulatedYield

 ### Title
Stale Oracle Price Acceptance
 ### Description/Code Snippet
The `iTryIssuer` contract retrieves the NAV price via `oracle.price()` in `mintITRY`, `redeemITRY`, and `processAccumulatedYield` without verifying the data's freshness (timestamp or round ID). If the oracle feed becomes stale (e.g., due to network congestion or sequencer downtime), the protocol may execute trades or mint yield based on outdated prices, potentially leading to arbitrage opportunities or insolvency.
 ### Static Signals
no updatedAt/answeredInRound checks, accepts price older than reasonable threshold
 ### Assets at Risk
treasury, collateral (DLF)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ExternalCallAfterStateChange

 ### Relevant Function/Location: iTryIssuer.processAccumulatedYield

 ### Title
External Call to Yield Receiver After State Change Without Reentrancy Guard
 ### Description/Code Snippet
In `processAccumulatedYield`, the contract updates `_totalIssuedITry` (via `_mint`) and then calls `yieldReceiver.processNewYield`. This function is not marked `nonReentrant`. If the `yieldReceiver` (configured by admin) is malicious or compromised, it could re-enter the system. While the specific impact is limited by the `_mint` occurring first, it violates the Checks-Effects-Interactions pattern in a core economic function.
 ### Static Signals
external call at end, no try/catch or rollback on failure
 ### Assets at Risk
iTRY
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: ReserveOrPriceDesync

 ### Relevant Function/Location: iTryIssuer.mintFor, redeemFor, processAccumulatedYield

 ### Title
Redstone Oracle integration fails to propagate payload, breaking price updates
 ### Description/Code Snippet
The `iTryIssuer` relies on an `oracle` (intended to be Redstone) to fetch NAV prices. Redstone's On-Demand (Pull) model requires the transaction calldata to contain signed price updates, which the consumer contract must extract using `getOracleNumericValueFromTxMsg`. However, `iTryIssuer` calls `oracle.price()` externally without forwarding the calldata or inheriting the Redstone consumer logic itself. As a result, the `oracle` contract sees empty calldata (only the function selector) and cannot validate or update the price, causing denial of service or usage of stale prices.
 ### Static Signals
oracle.price() call, missing getOracleNumericValueFromTxMsg usage, missing calldata forwarding
 ### Assets at Risk
protocol solvency, user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: InitOrderOrUnintialized

 ### Relevant Function/Location: iTry.initialize

 ### Title
Uninitialized Parent Contract State
 ### Description/Code Snippet
The `iTry` contract's `initialize` function calls `__ERC20_init` and others but fails to call `__AccessControl_init` (or verify `SingleAdminAccessControl` initialization). This may leave internal `AccessControl` state (like `_roles` or ERC165 registration) uninitialized, potentially breaking role checks or interface support inquiries.
 ### Static Signals
missing __AccessControl_init, initializer modifier
 ### Assets at Risk
access control integrity
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: ERC20DecimalsMismatch

 ### Relevant Function/Location: iTryIssuer.previewMint

 ### Title
Potential ERC20 Decimals Mismatch in Minting Logic
 ### Description/Code Snippet
In `iTryIssuer`, the `previewMint` and `mintITRY` functions calculate the mint amount using `dlfAmount * navPrice / 1e18`, implicitly assuming the collateral token (DLF) has 18 decimals to match the 1e18 scaling of `navPrice`. If the DLF token uses fewer decimals (e.g., 6), the resulting iTRY amount will be drastically undervalued (scaled down by 1e12), causing significant loss of user funds. The constructor does not validate `collateralToken.decimals()`.
 ### Static Signals
mixes token amounts with 18-decimal math unscaled
 ### Assets at Risk
user funds (DLF)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: StakediTryV2.maxRedeem

 ### Title
ERC4626 maxRedeem Violates Specification
 ### Description/Code Snippet
In `StakediTryV2`, the `redeem` and `withdraw` functions revert when the cooldown is active, but the `maxRedeem` and `maxWithdraw` functions continue to return the user's full balance. This violates the ERC4626 specification which requires `max` functions to return 0 if the operation would revert, breaking compatibility with standard integrators.
 ### Static Signals
incorrect return values/events per standard spec, standard-required function missing or has wrong signature
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: iTryIssuer.mintFor

 ### Title
Issuer accounting incompatible with fee-on-transfer collateral
 ### Description/Code Snippet
The `mintFor` function in `iTryIssuer` calls `_transferIntoVault`, which executes `collateralToken.transferFrom` and increments `_totalDLFUnderCustody` by the input `dlfAmount`. If the collateral token (`DLF`) implements a fee-on-transfer mechanism, the Vault receives less than `dlfAmount`, but the Issuer's accounting records the full amount. This creates a solvency gap where `_totalDLFUnderCustody` (liabilities) exceeds the actual assets held by the system.
 ### Static Signals
uses input amount instead of post-transfer delta, assumes transferFrom(amount) credits exactly amount
 ### Assets at Risk
collateral reserves
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StaleOracleAcceptance

 ### Relevant Function/Location: iTryIssuer.mintFor

 ### Title
Lack of staleness validation for NAV Oracle
 ### Description/Code Snippet
The `iTryIssuer` calls `oracle.price()` without performing any validation on the returned data (e.g., `updatedAt`, `answeredInRound`, or custom timestamp checks). It relies entirely on the external Oracle contract to revert if data is stale. If the configured Oracle adapter returns a stale or zero price without reverting (a common configuration risk), the system will accept invalid prices for minting and redemption.
 ### Static Signals
no updatedAt/answeredInRound checks, accepts price older than reasonable threshold
 ### Assets at Risk
iTryToken
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: OracleUsingDEXorTWAP

 ### Relevant Function/Location: iTryIssuer.previewMint, mintFor, previewRedeem, redeemFor, processAccumulatedYield

 ### Title
Oracle Price Staleness and Validation Missing
 ### Description/Code Snippet
The `iTryIssuer` contract uses `oracle.price()` to determine the NAV for minting and redemption but fails to validate the freshness of the returned price (e.g., checking `updatedAt` or `roundId` for Chainlink/Redstone feeds). While it checks for a zero price, it accepts stale prices, which could allow arbitrage or value extraction if the oracle updates are delayed or the feed freezes.
 ### Static Signals
oracle.price(), no stale check, no heartbeat check
 ### Assets at Risk
collateralToken, iTryToken
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ERC4626SharePriceMismatch

 ### Relevant Function/Location: StakediTryFastRedeem._redeemWithFee

 ### Title
Potential rounding bias in Fast Redeem fee accounting
 ### Description/Code Snippet
In `StakediTryFastRedeem`, the function `_redeemWithFee` calculates `feeShares` by converting `feeAssets` to shares (rounding up) and then subtracting this from `shares` to get `netShares`. Due to non-linear rounding, `netShares` may be artificially reduced relative to `netAssets`. While `_withdraw` does not revert on this mismatch, it results in the user burning a specific amount of shares for a slightly better-than-market asset rate (at the expense of the treasury's fee rate accuracy) or vice versa depending on the share price, creating a systematic precision leak.
 ### Static Signals
rounding bias always favors caller, divide before multiply in convertToShares/assets
 ### Assets at Risk
treasury
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: iTryTokenOFT._beforeTokenTransfer

 ### Title
Cross-Chain DoS via Blacklisted Recipient in iTryTokenOFT
 ### Description/Code Snippet
In `iTryTokenOFT`, incoming LayerZero messages trigger `_credit` -> `_mint` -> `_beforeTokenTransfer`. If the recipient is blacklisted, `_beforeTokenTransfer` reverts, causing the LayerZero message to fail and potentially blocking the message channel (DoS). Unlike `wiTryOFT`, which gracefully redirects such funds to the owner, `iTryTokenOFT` lacks this fallback mechanism.
 ### Static Signals
revert OperationNotAllowed(), no redirect logic in _credit
 ### Assets at Risk
Cross-chain messaging channel
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: iTryIssuer.mintFor

 ### Title
mintFor allows bypassing whitelist holding restrictions
 ### Description/Code Snippet
The `iTryIssuer.mintFor` function checks that the *caller* is whitelisted but allows minting tokens to a *non-whitelisted* recipient. The `iTry` token contract's `_beforeTokenTransfer` hook permits minting to non-whitelisted addresses (in `WHITELIST_ENABLED` state) but blocks subsequent transfers or redemptions by them. This leads to tokens being permanently stuck in non-whitelisted accounts.
 ### Static Signals
role check skipped for recipient, transfer restriction mismatch
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: iTryIssuer.processAccumulatedYield

 ### Title
Unbounded yield minting due to missing slippage/cap on Oracle price
 ### Description/Code Snippet
The `processAccumulatedYield` function calculates `newYield` based purely on `oracle.price()` and `_totalDLFUnderCustody` without any maximum bound or slippage protection. If the Oracle reports an erroneously high price (e.g., due to manipulation or malfunction), the contract will mint an unlimited amount of iTRY tokens, diluting the supply and breaking the 1:1 backing invariant.
 ### Static Signals
oracle.price() used to calculate mint amount, no maxAmount check, no slippage parameter
 ### Assets at Risk
iTRY token supply
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: PricePrecisionOrRoundingError

 ### Relevant Function/Location: iTryIssuer.previewMint

 ### Title
Decimal mismatch handling in minting logic
 ### Description/Code Snippet
The `previewMint` and `previewRedeem` functions calculate amounts using `amount * navPrice / 1e18`. This formula inherently assumes that the `collateralToken` has 18 decimals (matching `iTryToken` and the 1e18 scaling factor). If the collateral token has fewer decimals (e.g., 6, common for stablecoins/RWAs), a user depositing `1e6` (1 unit) will be credited `1e6` iTRY (1e-12 units), resulting in a massive loss of value due to missing decimal normalization.
 ### Static Signals
mix 6/8/18 decimals without normalization
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ReserveOrPriceDesync

 ### Relevant Function/Location: iTryIssuer.processAccumulatedYield

 ### Title
Yield distribution relies on internal accounting disjoint from actual vault balance
 ### Description/Code Snippet
The `processAccumulatedYield` function calculates distributable yield based on `_totalDLFUnderCustody * navPrice`. However, `_totalDLFUnderCustody` is an internal accounting variable updated only during mint/redeem. If the underlying asset balance changes externally (e.g., via rebase, custodian fees, or direct transfers/donations to the vault/custodian), the internal state desynchronizes from reality. This can lead to minting yield based on non-existent assets or failing to distribute valid yield.
 ### Static Signals
assumes invariant without verifying, accounting based on transfer parameter, not actual balance change
 ### Assets at Risk
protocol equity, iTry backing
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: iTryIssuer.processAccumulatedYield

 ### Title
Accounting Invariant Violation in Yield Calculation
 ### Description/Code Snippet
The `iTry` token contract inherits `ERC20BurnableUpgradeable`, allowing any user to burn their tokens via `burn()`. However, the `iTryIssuer` contract tracks the total liability in a local variable `_totalIssuedITry` which is only updated during calls to `redeemITRY`. If a user burns `iTry` directly, `_totalIssuedITry` will not decrease, causing a permanent desynchronization between the actual token supply and the issuer's liability tracking. Consequently, `processAccumulatedYield` will calculate yield based on an inflated liability figure, significantly underestimating (or eliminating) the yield distributed to `wiTRY` holders.
 ### Static Signals
_totalIssuedITry vs totalSupply mismatch, ERC20Burnable inheritance without hook
 ### Assets at Risk
yield, rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: iTryIssuer.processAccumulatedYield

 ### Title
Accounting Invariant Violation via Yield Minting
 ### Description/Code Snippet
The `processAccumulatedYield` function in `iTryIssuer` mints new iTRY liabilities based on the formula `(TotalCollateral * CurrentNAV) - TotalIssued`. If the NAV price spikes (or is manipulated) and then subsequently drops, the `TotalIssued` (liabilities) will remain elevated while the collateral value decreases. This action irreversibly creates unbacked iTRY, rendering the system insolvent (Liabilities > Assets) with no mechanism to burn the excess issuance to restore the peg.
 ### Static Signals
totalSupply != sum(balances), accounting state not updated when underlying asset is swapped/upgraded
 ### Assets at Risk
protocol solvency, iTRY holders
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: StakediTryFastRedeem.fastRedeem

 ### Title
Missing slippage protection in fast redemption
 ### Description/Code Snippet
The `fastRedeem` and `fastWithdraw` functions allow users to exit the staking vault immediately by paying a fee. However, these functions lack a `minAssetsOut` or `maxSharesIn` parameter. If the administrator updates the `fastRedeemFeeInBPS` (up to 20%) or if the underlying share value drops (e.g., due to negative yield/slashing) between transaction submission and execution, the user will receive fewer assets than expected without a revert.
 ### Static Signals
amountOutMin=0 or missing, payout calculated at execution time without minimum bound
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: StakediTryV2.withdraw

 ### Title
ERC4626 Compliance Violation in Withdrawal Logic
 ### Description/Code Snippet
The `StakediTryV2` contract overrides `withdraw` and `redeem` to revert when `cooldownDuration > 0` via the `ensureCooldownOff` modifier. This violates the ERC4626 standard which implies these functions should effect a withdrawal or follow standard failure modes (e.g. maxWithdraw returning 0). This deviation can cause denial of service for standard-compliant aggregators or smart contracts attempting to interact with the vault.
 ### Static Signals
standard-required function missing or has wrong signature, preview/view functions modify state
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: iTryIssuer._transferIntoVault

 ### Title
Fee-on-Transfer Token Support Missing in Collateral Handling
 ### Description/Code Snippet
The `_transferIntoVault` function in `iTryIssuer` updates the internal accounting variable `_totalDLFUnderCustody` with the full `dlfAmount` specified by the user, but `collateralToken.transferFrom` may transfer a smaller amount if the token implements fee-on-transfer logic. This creates a discrepancy where the protocol's accounting of assets exceeds the actual balance in the `FastAccessVault`, leading to inflated yield calculations and potential insolvency.
 ### Static Signals
accounting based on transfer parameter, not actual balance change, assumes transferFrom(amount) credits exactly amount
 ### Assets at Risk
collateralToken
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

