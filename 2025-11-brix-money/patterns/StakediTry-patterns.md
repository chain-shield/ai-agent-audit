## Verified Patterns Found: 25

## Verified Patterns Found in following Categories:

- TimelockEdgeCase
- ReserveOrPriceDesync
- AccessControlOrAuthByPass
- PricePrecisionOrRoundingError
- ERC20DecimalsMismatch
- UnsafeRecipient
- GriefableCallbacks
- SlippageMissingOrInsufficient
- StaleOracleAcceptance
- FeeOnTransferAssumption
- AccountingInvariantViolation
- UnsafeAssembyTypeCasts
- StandardViolation
- UpgradeAuthBypass



## Summary of Patterns

Strict vesting check in transferInRewards causes yield distribution denial-of-service

Accounting invariant broken by external token burns

Blacklisted Users' Funds Permanently Locked in Silo

UUPS Upgradeability Implementation Mismatch

Accounting desync in FastAccessVault if Collateral Token rebases

Soft Restricted Staker Role bypass via transfer

Cross-chain Cooldown Reset Griefing

Lack of asset rescue mechanism in iTrySilo risks permanent fund loss

Missing Slippage Protection in Staking and Fast Redemption

Hardcoded 18-decimal assumption for Oracle price feed

ERC20DecimalsMismatch in iTryIssuer Pricing Logic

StaleOracleAcceptance in iTryIssuer

Unsafe assembly type casting in cooldown accounting

Assets in cooldown become permanently stuck if user is blacklisted

Accounting Desync on FastAccessVault Rebalance

Accounting Invariant Violation via FastAccessVault Rescue

Hardcoded decimal assumption in minting logic

Oracle price accepted without freshness validation

Fee-On-Transfer Token Incompatibility in Vesting Logic

ERC4626 Standard Violation: Reverting withdraw/redeem

Bypass of Full Restricted Role in unstake()

ERC4626 Standard Violation: maxWithdraw/maxRedeem return non-zero when disabled

Slippage Protection Missing in Vault Operations

Missing Slippage Protection in Cooldown and Fast Redeem Functions

Assumption of 1:1 transfer in iTryIssuer leads to undercollateralization

## Patterns



 ### Issue Type: TimelockEdgeCase

 ### Relevant Function/Location: StakediTry.transferInRewards

 ### Title
Strict vesting check in transferInRewards causes yield distribution denial-of-service
 ### Description/Code Snippet
The `transferInRewards` function in `StakediTry` explicitly reverts with `StillVesting()` if `getUnvestedAmount() > 0`. This enforces a rigid sequential vesting schedule where the Rewarder cannot top up or queue new rewards until the previous period has completely finished, leading to potential yield gaps ('cliffs') and operational availability issues.
 ### Static Signals
revert if getUnvestedAmount() > 0, sequential vesting enforcement
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: iTryIssuer.processAccumulatedYield

 ### Title
Accounting invariant broken by external token burns
 ### Description/Code Snippet
The `iTryIssuer` contract tracks total issued iTRY via the `_totalIssuedITry` state variable to calculate yield and backing. However, the `iTry` token inherits `ERC20BurnableUpgradeable`, allowing any user to burn tokens publicly. These burns reduce `iTry.totalSupply()` but do not update `_totalIssuedITry` in the issuer. This desynchronization causes `processAccumulatedYield` to underestimate the system's solvency, potentially failing to distribute valid yield or halting operations due to perceived undercollateralization.
 ### Static Signals
totalSupply != sum(balances), accounting state not updated when underlying asset is swapped/upgraded
 ### Assets at Risk
yield, rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: StakediTryV2.unstake

 ### Title
Blacklisted Users' Funds Permanently Locked in Silo
 ### Description/Code Snippet
In `StakediTryV2`, the `unstake` function calls `silo.withdraw`, which attempts to transfer assets to the user. If the user is blacklisted during the cooldown period, `iTry.transfer` will revert, causing `unstake` to fail. Since `iTrySilo` is immutable and lacks a rescue function, and `StakediTryV2.rescueTokens` cannot access Silo funds, the assets are permanently locked. The admin's `redistributeLockedAmount` function only works on token balances, not on Silo claims.
 ### Static Signals
no try/catch around external hook, withdrawal blocked because recipient's receive() reverts, no bypass on callback failure
 ### Assets at Risk
silo assets, user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UpgradeAuthBypass

 ### Relevant Function/Location: iTry.N/A

 ### Title
UUPS Upgradeability Implementation Mismatch
 ### Description/Code Snippet
Documentation states `iTry` token is UUPS-upgradeable. The contract imports `ERC20PermitUpgradeable` and other upgradeable/initializable modules but fails to inherit `UUPSUpgradeable` or implement the required `_authorizeUpgrade` function. If deployed behind a UUPS proxy (ERC1967Proxy), the proxy will be unable to call the upgrade logic on the implementation, rendering the contract permanently non-upgradeable and potentially bricking governance control over the token logic.
 ### Static Signals
Initializable used without UUPSUpgradeable, missing _authorizeUpgrade
 ### Assets at Risk
governance control
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: ReserveOrPriceDesync

 ### Relevant Function/Location: FastAccessVault.rebalanceFunds

 ### Title
Accounting desync in FastAccessVault if Collateral Token rebases
 ### Description/Code Snippet
The `FastAccessVault` and `iTryIssuer` track `_totalDLFUnderCustody` as a simple sum of deposits minus redemptions. If the underlying `collateralToken` (DLF) is a rebasing token (positive rebase), the actual balance in `FastAccessVault` will grow larger than `_totalDLFUnderCustody`. The `rebalanceFunds` function in `FastAccessVault` uses `balanceOf(address(this))` to check against a target, potentially sweeping the 'invisible' rebase yield to the custodian as 'excess funds', while `iTryIssuer` calculates yield based solely on NAV price changes. This leads to a loss of yield for stakers or accounting inconsistencies.
 ### Static Signals
balanceOf(this) used for logic, internal accounting variable
 ### Assets at Risk
DLF Yield
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: StakediTry._beforeTokenTransfer

 ### Title
Soft Restricted Staker Role bypass via transfer
 ### Description/Code Snippet
The `StakediTry` contract implements a `SOFT_RESTRICTED_STAKER_ROLE` intended to prevent an address from staking (minting/depositing). This is enforced in `_deposit`. However, `_beforeTokenTransfer` only checks the `FULL_RESTRICTED_STAKER_ROLE`. Consequently, a 'soft restricted' user can still acquire `wiTRY` shares via a standard ERC20 transfer from another user, effectively bypassing the restriction on holding or entering the staking position.
 ### Static Signals
role check in deposit, missing role check in transfer
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: StakediTryCrosschain.cooldownSharesByComposer

 ### Title
Cross-chain Cooldown Reset Griefing
 ### Description/Code Snippet
In `StakediTryCrosschain`, the `cooldownSharesByComposer` and `cooldownAssetsByComposer` functions allow a Composer to add assets to a user's cooldown. The logic `cooldowns[redeemer].cooldownEnd = block.timestamp + cooldownDuration` unconditionally resets the cooldown timer for the user's *entire* pending balance. An attacker can exploit this by bridging a negligible amount (dust) to a victim's address on the Hub chain, perpetually resetting their cooldown timer to the full duration and locking their funds indefinitely.
 ### Static Signals
cooldownEnd overwritten, no check for significant deposit, external input controls timer
 ### Assets at Risk
user funds (locked)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: iTrySilo.N/A

 ### Title
Lack of asset rescue mechanism in iTrySilo risks permanent fund loss
 ### Description/Code Snippet
The `iTrySilo` contract, which holds assets during the cooldown period, does not implement a `rescueTokens` function for the underlying asset, nor does it allow the `StakediTryV2` vault to withdraw arbitrary amounts (only strictly tracked `underlyingAmount`). If assets are accidentally sent directly to the Silo or if accounting invariants drift, these funds are permanently locked with no recovery path.
 ### Static Signals
no rescueTokens function, onlyStakingVault restricts withdrawals
 ### Assets at Risk
collateral
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: StakediTryV2.cooldownAssets, cooldownShares

 ### Title
Missing Slippage Protection in Staking and Fast Redemption
 ### Description/Code Snippet
The `StakediTryV2` and `StakediTryFastRedeem` contracts introduce custom state-changing functions `cooldownAssets`, `cooldownShares`, `fastRedeem`, and `fastWithdraw` that exchange assets for shares (or vice versa) using the current exchange rate. Unlike standard ERC4626 `deposit/redeem` or the `iTryIssuer` functions, these functions lack `minAssetsOut` or `maxSharesIn` parameters. This omission exposes users to unlimited slippage if the exchange rate changes unfavorably (e.g., due to large yield distribution, slashing, or sandwich attacks) between transaction submission and execution.
 ### Static Signals
no minAmountOut parameter, previewWithdraw result used directly, previewRedeem result used directly
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PricePrecisionOrRoundingError

 ### Relevant Function/Location: iTryIssuer.mintFor, redeemFor

 ### Title
Hardcoded 18-decimal assumption for Oracle price feed
 ### Description/Code Snippet
The `iTryIssuer` functions `mintFor` and `redeemFor` calculate token amounts using `navPrice / 1e18` (e.g. `iTRYAmount = netDlfAmount * navPrice / 1e18`). This logic hardcodes an assumption that the `oracle.price()` returns a value with 18 decimals. If the integrated Oracle (Redstone/Chainlink) returns 8 decimals (standard for many USD feeds), the resulting amounts will be underscaled by a factor of 10^10, leading to severe loss of funds or incorrect minting amounts.
 ### Static Signals
division by 1e18, mix decimals without normalization
 ### Assets at Risk
user funds, minted tokens
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ERC20DecimalsMismatch

 ### Relevant Function/Location: iTryIssuer.mintFor, previewMint, previewRedeem

 ### Title
ERC20DecimalsMismatch in iTryIssuer Pricing Logic
 ### Description/Code Snippet
The `iTryIssuer` calculates `iTRYAmount = netDlfAmount * navPrice / 1e18`. This formula hardcodes a division by `1e18`, implicitly assuming that `navPrice` is scaled to 18 decimals and that the input amounts align with this scale. If the underlying Oracle uses 8 decimals (standard for many USD feeds) or if the DLF token does not use 18 decimals, this calculation will result in significant value distortion (underflow or overflow).
 ### Static Signals
mixes token amounts with 18-decimal math unscaled, uses oracle price with different base decimals
 ### Assets at Risk
protocol funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StaleOracleAcceptance

 ### Relevant Function/Location: iTryIssuer.mintFor, redeemFor, processAccumulatedYield

 ### Title
StaleOracleAcceptance in iTryIssuer
 ### Description/Code Snippet
The `iTryIssuer` contract uses `oracle.price()` to determine the exchange rate for minting and redeeming iTRY without performing any validation on the returned data's freshness (e.g., `updatedAt`, `answeredInRound` for Chainlink/Redstone). If the oracle feed becomes stale or stops updating, users can mint or redeem tokens at incorrect prices.
 ### Static Signals
price fetched at execution without user-specified floor, no updatedAt/answeredInRound checks, accepts price older than reasonable threshold
 ### Assets at Risk
protocol collateral (DLF), iTRY stability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeAssembyTypeCasts

 ### Relevant Function/Location: StakediTryV2.cooldownAssets

 ### Title
Unsafe assembly type casting in cooldown accounting
 ### Description/Code Snippet
In `cooldownAssets` and `cooldownShares`, the `assets` amount (uint256) is cast to `uint152` when adding to `cooldowns[msg.sender].underlyingAmount`. This cast is performed without a prior check that `assets` fits within 152 bits. While 2^152 is a very large number, if the asset supply or decimals allowed for such a value, the silent truncation would lead to severe loss of user funds stored in the cooldown state.
 ### Static Signals
downcasts without range checks, uint152(assets)
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: StakediTryV2.unstake

 ### Title
Assets in cooldown become permanently stuck if user is blacklisted
 ### Description/Code Snippet
The protocol allows blacklisting users via `iTry` token roles or `StakediTry` roles. If a user initiates a cooldown (burning shares and moving assets to the `iTrySilo`) and is subsequently blacklisted before calling `unstake`, their assets become permanently stuck. The `unstake` function calls `silo.withdraw`, which triggers an `iTry.transfer` to the user. This transfer will revert due to the blacklist hook in `iTry`. Unlike `iTry` tokens or `wiTry` shares held in a user's wallet, which can be seized/redistributed by the admin using `redistributeLockedAmount`, there is no mechanism to seize or redirect the pending cooldown assets held in the `iTrySilo`. Admin cannot use `rescueTokens` on `StakediTry` to recover them as the asset is protected.
 ### Static Signals
transfer reverts on blacklist, no admin override for silo, redistributeLockedAmount only burns shares
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: FastAccessVault.rebalanceFunds

 ### Title
Accounting Desync on FastAccessVault Rebalance
 ### Description/Code Snippet
The `FastAccessVault.rebalanceFunds` function transfers excess DLF (e.g., from direct donations or mistaken transfers) to the custodian. However, it does not notify the `iTryIssuer` to increment `_totalDLFUnderCustody`. Since `iTryIssuer` relies on `_totalDLFUnderCustody` to calculate yield, these extra funds are effectively ignored by the protocol's accounting. The yield generated by these assets will never be minted or distributed to stakers, resulting in a permanent loss of yield value for the protocol.
 ### Static Signals
external transfer of core asset without updating accounting state, balance tracking references different token than actual holdings
 ### Assets at Risk
protocol yield
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: FastAccessVault.rescueToken

 ### Title
Accounting Invariant Violation via FastAccessVault Rescue
 ### Description/Code Snippet
The `FastAccessVault.rescueToken` function allows the owner to withdraw any token, including the collateral `_vaultToken` (DLF), without restriction. However, the `iTryIssuer` contract tracks the system's total collateral in `_totalDLFUnderCustody` and uses it to calculate yield/minting. If the owner rescues DLF from the vault, `_totalDLFUnderCustody` is not updated, causing the Issuer to overestimate backing collateral. This desynchronization leads to `processAccumulatedYield` minting unbacked iTRY yield based on collateral that is no longer in the system.
 ### Static Signals
rescueToken checks address(0) but not asset address, state variable tracking external balance not updated
 ### Assets at Risk
iTRY protocol solvency
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: ERC20DecimalsMismatch

 ### Relevant Function/Location: iTryIssuer.mintFor

 ### Title
Hardcoded decimal assumption in minting logic
 ### Description/Code Snippet
The `iTryIssuer.mintFor` function calculates `iTRYAmount` using the formula `netDlfAmount * navPrice / 1e18`, which implicitly assumes the collateral token (DLF) has 18 decimals (matching iTRY). If the collateral token has fewer decimals (e.g., 6), the minted amount will be drastically undervalued (orders of magnitude loss for the user). If it has more, it will be overvalued.
 ### Static Signals
mixes token amounts with 18-decimal math unscaled
 ### Assets at Risk
user funds, collateral
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StaleOracleAcceptance

 ### Relevant Function/Location: iTryIssuer.mintFor

 ### Title
Oracle price accepted without freshness validation
 ### Description/Code Snippet
The `iTryIssuer` contract calls `oracle.price()` in `mintFor`, `redeemFor`, and `processAccumulatedYield` without validating the data's timestamp or round ID. If the oracle returns stale data (e.g., due to a paused feed or lack of updates), the system may process mints and redemptions at incorrect prices, leading to arbitrage or insolvency.
 ### Static Signals
no updatedAt/answeredInRound checks on Chainlink, accepts price older than reasonable threshold
 ### Assets at Risk
collateral, treasury
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: StakediTry.transferInRewards

 ### Title
Fee-On-Transfer Token Incompatibility in Vesting Logic
 ### Description/Code Snippet
The `transferInRewards` function in `StakediTry` updates the `vestingAmount` state variable using the raw input `amount` and subsequently pulls tokens via `safeTransferFrom`. If the underlying `iTry` asset (which is upgradeable) implements or is upgraded to include transfer fees, the actual balance increase will be less than `amount`. The `totalAssets()` calculation relies on `balanceOf(address(this)) - getUnvestedAmount()`. Since `getUnvestedAmount()` is derived from the full `amount`, it may exceed the contract's balance (if balance increase < `amount`), causing `totalAssets()` to underflow and revert. This would permanently break all vault functionality (deposits, withdrawals, pricing).
 ### Static Signals
_updateVestingAmount(amount), safeTransferFrom(..., amount), totalAssets() = balance - unvested
 ### Assets at Risk
vault functionality
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: StakediTryV2.withdraw

 ### Title
ERC4626 Standard Violation: Reverting withdraw/redeem
 ### Description/Code Snippet
The `StakediTryV2` contract conditionally disables `withdraw` and `redeem` using the `ensureCooldownOff` modifier when `cooldownDuration > 0`. However, the inherited `maxWithdraw` and `maxRedeem` functions (from ERC4626) continue to return the user's full balance/limit. This violates the ERC4626 invariant that `withdraw(maxWithdraw(...))` should not revert. This behavior breaks compatibility with standard vault integrators, routers, and solvency checkers.
 ### Static Signals
modifier ensures toggle is off, maxWithdraw returns non-zero while withdraw reverts
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: StakediTryV2.unstake

 ### Title
Bypass of Full Restricted Role in unstake()
 ### Description/Code Snippet
The `unstake` function in `StakediTryV2` allows users to withdraw assets from the Silo after their cooldown period. However, unlike `withdraw`, `redeem`, and `cooldownAssets`, this function does NOT check if the user has the `FULL_RESTRICTED_STAKER_ROLE`. This means a user who has been restricted (intended to have their funds frozen) can still retrieve their underlying assets if they had previously entered the cooldown queue, bypassing the compliance restriction.
 ### Static Signals
missing modifier, role check missing, cooldowns[msg.sender] read without role validation
 ### Assets at Risk
collateral/underlying assets
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: StakediTryV2.maxWithdraw

 ### Title
ERC4626 Standard Violation: maxWithdraw/maxRedeem return non-zero when disabled
 ### Description/Code Snippet
In `StakediTryV2`, the `withdraw` and `redeem` functions are modified to revert when `cooldownDuration > 0`. However, the contract does not override `maxWithdraw` or `maxRedeem` to reflect this limitation. According to EIP-4626, `maxWithdraw` and `maxRedeem` MUST return 0 if the corresponding operation would revert due to global limits (like the cooldown mode). Integrators relying on the standard will see a positive max withdrawal amount but will face reverts when attempting execution, leading to Denial of Service in dependent systems.
 ### Static Signals
withdraw reverts based on state, maxWithdraw not overridden
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: StakediTryV2.cooldownAssets, cooldownShares, fastRedeem, fastWithdraw

 ### Title
Slippage Protection Missing in Vault Operations
 ### Description/Code Snippet
The `cooldownAssets`, `cooldownShares`, `fastRedeem`, and `fastWithdraw` functions in `StakediTryV2` and `StakediTryFastRedeem` execute share-to-asset conversions at the current exchange rate without accepting user-defined minimum output (`minAssets`) or maximum input (`maxShares`) parameters. Unlike the `iTryIssuer` contract which enforces slippage bounds, the vault exposes users to sandwich attacks or unfavorable rate changes in the event of price fluctuations or large pool state changes (e.g. donations).
 ### Static Signals
amountOutMin=0 or missing, payout calculated at execution time without minimum bound
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: StakediTryV2.cooldownAssets

 ### Title
Missing Slippage Protection in Cooldown and Fast Redeem Functions
 ### Description/Code Snippet
The `cooldownAssets`, `cooldownShares`, `fastRedeem`, and `fastWithdraw` functions (and their cross-chain composer equivalents) execute conversions between shares and assets using the current exchange rate without accepting user-defined minimum output (`minAssets`) or maximum input (`maxShares`) parameters. Unlike standard ERC4626 functions (`withdraw`/`redeem` which are overridden/disabled here or allow slippage args in standard implementations), these custom entry points force execution at the spot rate. Users are exposed to value loss if the share price fluctuates unfavorably (e.g., due to market movements or front-running) between transaction submission and execution.
 ### Static Signals
shares = previewWithdraw(assets), assets = previewRedeem(shares), no minAssets param, no maxShares param
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: iTryIssuer._transferIntoVault

 ### Title
Assumption of 1:1 transfer in iTryIssuer leads to undercollateralization
 ### Description/Code Snippet
In `iTryIssuer.mintFor`, the contract calculates the `iTRYAmount` to mint based on the input `dlfAmount` (minus fees), assuming that `collateralToken.transferFrom` successfully delivers exactly `dlfAmount` to the `liquidityVault`. If the `collateralToken` (DLF) implements a fee-on-transfer mechanism or is a rebasing token where the transfer amount differs from the received amount, the `FastAccessVault` will receive fewer tokens than `iTryIssuer` accounts for. This results in the minting of unbacked iTRY, breaking the 1:1 backing invariant.
 ### Static Signals
transferFrom(from, to, amount), no balance check after transfer
 ### Assets at Risk
iTRY, DLF
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole

