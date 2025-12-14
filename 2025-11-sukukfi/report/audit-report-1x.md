# 2025 11 sukukfi - Findings Report
## Commit hash: 18fe2578cf1c6203dac7ff21513533010f3dda3e

##Findings by Status


Finding Status: Valid


[M-1]. Vault Unregistration Griefing via Dust Donation
**Derived From** : ForcedAssetVsStrictEquality
Finding Status: Valid
Privilege: Permissionless


[M-2]. Slippage Missing in Investment Operations
**Derived From** : SlippageMissingOrInsufficient
Finding Status: Valid
Privilege: RequiresRole


[M-3]. Rounding direction in withdraw allows free asset withdrawals
**Derived From** : PricePrecision
Finding Status: Valid
Privilege: RequiresRole



Finding Status: LowSeverityDueToRareLikelihood + InvalidSafeGuardInPlace + InvalidOutOfScope + InvalidNotExploitable + InvalidGovernanceRisk + InvalidByDesign


[M-4]. Phantom yield in rBalance causes Investment Layer insolvency
**Derived From** : AccountingInvariantViolation
Finding Status: LowSeverityDueToRareLikelihood + InvalidSafeGuardInPlace + InvalidOutOfScope + InvalidNotExploitable + InvalidGovernanceRisk + InvalidByDesign
Privilege: RequiresRole



Finding Status: LowSeverityDueToLowImpact


[M-5]. Double Allowance Consumption in WERC7575ShareToken.transferFrom
**Derived From** : AccountingInvariantViolation
Finding Status: LowSeverityDueToLowImpact
Privilege: Permissionless



Finding Status: InvalidByDesign


[H-6]. Protocol Insolvency Risk due to Multi-Asset Peg Dependency
**Derived From** : Multi-Asset Reserve Desync Arbitrage
Finding Status: InvalidByDesign
Privilege: Permissionless


[H-7]. Multi-Asset Reserve Desync Arbitrage
**Derived From** : Oracle
Finding Status: InvalidByDesign
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood + InvalidNotExploitable + InvalidGovernanceRisk + InvalidFutureSpeculation + InvalidERC20EdgeCase


[H-8]. Permanent DoS via Broken Vault Dependency in ShareTokenUpgradeable
**Derived From** : Dos
Finding Status: LowSeverityDueToRareLikelihood + InvalidNotExploitable + InvalidGovernanceRisk + InvalidFutureSpeculation + InvalidERC20EdgeCase
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood + LowSeverityDueToLowImpact + InvalidNotExploitable + InvalidGovernanceRisk


[M-9]. Unsafe UUPS Implementation Bypassing Verification
**Derived From** : StandardViolation
Finding Status: LowSeverityDueToRareLikelihood + LowSeverityDueToLowImpact + InvalidNotExploitable + InvalidGovernanceRisk
Privilege: RequiresAdminRole



Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk


[M-10]. TotalSupply Invariant Violation via Unsafe Recipient in Batch Transfers
**Derived From** : AccountingInvariantViolation
Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
Privilege: RequiresRole



Finding Status: LowSeverityDueToRareLikelihood + LowSeverityDueToLowImpact + InvalidSafeGuardInPlace + InvalidNotExploitable + InvalidFutureSpeculation + InvalidByDesign + InvalidBugDoesNotExist


[M-11]. Share Price Manipulation via Donation
**Derived From** : FlashLoanEconomicManipulation
Finding Status: LowSeverityDueToRareLikelihood + LowSeverityDueToLowImpact + InvalidSafeGuardInPlace + InvalidNotExploitable + InvalidFutureSpeculation + InvalidByDesign + InvalidBugDoesNotExist
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood + LowSeverityDueToLowImpact + InvalidSafeGuardInPlace + InvalidNotExploitable + InvalidFutureSpeculation + InvalidERC20EdgeCase + InvalidBugDoesNotExist


[M-12]. Read-Only Reentrancy in `totalAssets` via `requestDeposit`
**Derived From** : Reentrancy
Finding Status: LowSeverityDueToRareLikelihood + LowSeverityDueToLowImpact + InvalidSafeGuardInPlace + InvalidNotExploitable + InvalidFutureSpeculation + InvalidERC20EdgeCase + InvalidBugDoesNotExist
Privilege: Permissionless



Finding Status: LowSeverityDueToLowImpact + InvalidSafeGuardInPlace + InvalidNotExploitable + InvalidByDesign + InvalidBugDoesNotExist


[M-13]. Missing Slippage Protection in Async Request Functions
**Derived From** : SlippageMissingOrInsufficient
Finding Status: LowSeverityDueToLowImpact + InvalidSafeGuardInPlace + InvalidNotExploitable + InvalidByDesign + InvalidBugDoesNotExist
Privilege: Permissionless


[M-14]. Share Price Manipulation via Donation and Spot Balance Dependency
**Derived From** : Share Price Manipulation via Donation (Spot Balance Dependency)
Finding Status: LowSeverityDueToLowImpact + InvalidSafeGuardInPlace + InvalidNotExploitable + InvalidByDesign + InvalidBugDoesNotExist
Privilege: Permissionless



Finding Status: InvalidGovernanceRisk


[H-15]. Share Price Inflation via Insolvency Clamping in totalAssets
**Derived From** : AccountingInvariantViolation
Finding Status: InvalidGovernanceRisk
Privilege: RequiresAdminRole


### Number of Findings
- C: 0
- H: 4
- M: 11
- L: 0
- I: 0

##Findings by Status


Finding Status: Valid
## [M-1]. Vault Unregistration Griefing via Dust Donation

## id: imHkmZZL7WZjT9_jgXAnL

## Derived From Pattern/Invariant
ForcedAssetVsStrictEquality

## Exploit Type
ForcedAssetVsStrictEquality

## Location
ShareTokenUpgradeable.unregisterVault

## Finding Status: Valid
### Finding Status Justification: "Low impact" is false: any account can permanently block unregisterVault by donating 1 unit to the vault (strict balance==0 check). This can prevent vault rotation/deprecation and can exhaust MAX_VAULTS_PER_SHARE_TOKEN capacity over time, impacting protocol operability (availability/management), not merely cosmetic behavior.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `ShareTokenUpgradeable.unregisterVault` function strictly requires `IERC20(asset).balanceOf(vaultAddress) == 0`. An attacker can send a minimal amount (1 wei) of the asset to the vault address directly. This causes the strict equality check to fail, preventing the owner from unregistering the vault. While the Investment Manager can theoretically sweep assets via `investAssets`, this requires the vault to be active and the Investment Vault to accept dust. If the dust cannot be cleared perfectly, the vault cannot be unregistered, permanently occupying one of the limited slots (`MAX_VAULTS_PER_SHARE_TOKEN` = 10).

## Impact
Permanent Denial of Service for vault management; inability to remove deprecated or broken vaults.

## Command to Run Test


## Proof of Concept
1. Admin prepares to unregister a vault. 2. Attacker sends 1 wei of asset to the vault. 3. Admin calls `unregisterVault`. 4. The call reverts because `balanceOf(vault) != 0`.

## Proof of Code
function testUnregisterGrief() public { ... }

## Suggested Mitigation
Allow unregistration if `balanceOf` is non-zero but `isActive` is false, or provide a method to sweep remaining dust to a treasury before unregistration check.





Finding Status: Unknown
## [M-2]. Slippage Missing in Investment Operations

## id: fLcsNrPYe9KI_7roeFb-p

## Derived From Pattern/Invariant
SlippageMissingOrInsufficient

## Exploit Type
SlippageMissingOrInsufficient

## Location
ERC7575VaultUpgradeable.investAssets

## Finding Status: 
### Finding Status Justification: ERC7575VaultUpgradeable.investAssets approves and calls IERC7575(investmentVault).deposit(amount, shareToken) with no min-out constraint; withdrawFromInvestment uses previewWithdraw and redeems shares, then accepts whatever assets are returned (only measuring balance diff). There is no on-chain slippage bound, deadline, or TWAP. If the external investment vault has a manipulable exchange rate (common ERC4626-style share pricing), a third party can MEV-sandwich the manager’s public tx and force a worse rate, permanently losing value for users. No existing guard mitigates this (nonReentrant is unrelated). This is a real code-level missing check, not just a spec deviation.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `investAssets` and `withdrawFromInvestment` functions in `ERC7575VaultUpgradeable.sol` interact with external investment vaults (`IERC7575`) without any user-specified or manager-specified slippage protection (minimum shares or minimum assets). If the external vault has a variable exchange rate (e.g., standard ERC-4626) and is manipulated (e.g., via sandwich attack or flash loan) or experiences high volatility, the protocol may receive significantly fewer shares or assets than expected, resulting in loss of value for the vault users.

## Impact
Loss of vault assets due to unfavorable exchange rates or MEV attacks.

## Command to Run Test


## Proof of Concept
1. Manager calls `investAssets(1000)`.
2. MEV bot sandwiches tx, inflating share price of investment vault.
3. Vault receives very few shares for 1000 assets.
4. Assets lost.

## Proof of Code
function testMissingSlippage() public { ... }

## Suggested Mitigation
Add `minShares` and `minAssets` parameters to `investAssets` and `withdrawFromInvestment` functions.


## [M-3]. Rounding direction in withdraw allows free asset withdrawals

## id: A4s-q5sAoswR2c5ohRxMM

## Derived From Pattern/Invariant
PricePrecision

## Exploit Type
PricePrecision

## Location
ERC7575VaultUpgradeable.withdraw

## Finding Status: 
### Finding Status Justification: ERC7575VaultUpgradeable.withdraw computes shares = assets.mulDiv(availableShares, availableAssets, Floor). It then always transfers assets out, but burns shares only if (shares > 0). If availableAssets > availableShares (i.e., assets/share > 1 in the stored claimable ratio), a caller can choose small assets such that shares rounds to 0, repeatedly draining claimableRedeemAssets while burning zero shares. State updates subtract assets but not shares, and transfer executes unconditionally. This is a concrete, permissionless exploit once a controller has claimable redemption assets and the ratio allows floor-to-zero. No existing guard prevents it (nonReentrant doesn’t address the rounding/logic error).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
In `ERC7575VaultUpgradeable.withdraw`, the function calculates the shares to burn using `Math.Rounding.Floor`: `shares = assets.mulDiv(availableShares, availableAssets, Math.Rounding.Floor)`. If `availableAssets > availableShares` (share price > 1 asset), a user can withdraw small amounts of assets such that `assets * availableShares < availableAssets`, resulting in `shares = 0`. The code executes `if (shares > 0) { ... burn ... }` (implicit or explicit check) but unconditionally transfers assets `SafeTokenTransfers.safeTransfer($.asset, receiver, assets)`. This allows users to drain assets without burning any shares.

## Impact
Users can drain all claimable assets from the vault without burning shares.

## Command to Run Test


## Proof of Concept
1. User has claimable redeem assets. Price is 2 assets per share.
2. User calls `withdraw(1, receiver, controller)`.
3. `shares = 1 * 1 / 2 = 0`.
4. Assets transferred, 0 shares burned.
5. Repeat until drained.

## Proof of Code
function testFreeWithdrawal() public { ... }

## Suggested Mitigation
Use `Math.Rounding.Ceil` for share calculation in `withdraw` or require `shares > 0`.





Finding Status: LowSeverityDueToRareLikelihood + InvalidSafeGuardInPlace + InvalidOutOfScope + InvalidNotExploitable + InvalidGovernanceRisk + InvalidByDesign
## [M-4]. Phantom yield in rBalance causes Investment Layer insolvency

## id: YTL2l1vPee1ra9hg7N1vJ

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
WERC7575ShareToken.adjustrBalance

## Finding Status: LowSeverityDueToRareLikelihood + InvalidSafeGuardInPlace + InvalidOutOfScope + InvalidNotExploitable + InvalidGovernanceRisk + InvalidByDesign
### Finding Status Justification: "Finding does not exist" is false because, mechanically, counting rBalance as assets can inflate conversion rates without immediate on-chain redeemability. "Low impact" is false because if privileged accounting diverges from real backing (even within caps), it can materially misprice shares and create insolvency/failed redemptions; it’s primarily mitigated only by the trusted-role/process assumption and the documented design intent.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `WERC7575ShareToken` implements an `adjustrBalance` function intended to inject yield/profit into the system by increasing `_rBalances`. The Investment Layer (`ShareTokenUpgradeable`) counts these `rBalance` increases as assets (`_calculateInvestmentAssets`). However, `adjustrBalance` does not mint or transfer actual liquid tokens to `_balances`. Since `WERC7575Vault` redemption burns from `_balances` (standard ERC20 behavior), the Investment Layer cannot redeem this phantom yield. When investors try to exit, the Investment Layer will be insolvent as it holds 'value' in `rBalance` that cannot be converted to underlying assets via redemption.

## Impact
Insolvency of the investment layer; investors cannot withdraw quoted profits.

## Command to Run Test


## Proof of Concept
1. Admin calls `adjustrBalance(shareToken, ..., profit=100)`.
2. `rBalance` increases by 100. Share price increases.
3. User redeems shares worth 100 extra.
4. `ShareToken` calls `vault.redeem`.
5. Vault burns shares but has no extra assets to pay out (reverts on transfer).

## Proof of Code
function testPhantomYield() public { ... }

## Suggested Mitigation
Ensure `adjustrBalance` is accompanied by a real asset deposit to the backing vault, or mint tokens to match the `rBalance` increase.





Finding Status: LowSeverityDueToLowImpact
## [M-5]. Double Allowance Consumption in WERC7575ShareToken.transferFrom

## id: s3HDnrxgq6pz1mX8aqnn-

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
WERC7575ShareToken.transferFrom

## Finding Status: LowSeverityDueToLowImpact
### Finding Status Justification: "User error" is false: the double-decrement is caused by the contract’s override calling _spendAllowance(from, from, value) and then OZ ERC20.transferFrom calling _spendAllowance(from, msg.sender, value) again when msg.sender==from. Users can trigger this via common self-transferFrom patterns (routers/smart wallets), so it’s a contract behavior issue, even if impact is mostly UX.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `transferFrom` function in `WERC7575ShareToken` explicitly calls `_spendAllowance(from, from, value)` to enforce self-allowance (permit), and then calls `super.transferFrom(from, to, value)`. The parent OpenZeppelin `ERC20.transferFrom` implementation calls `_spendAllowance(from, msg.sender, value)`. When a user calls `transferFrom` on themselves (`msg.sender == from`), `allowance[from][from]` is deducted twice: once in the override and once in the parent function. This creates an accounting invariant violation where a self-transfer costs double the allowance amount.

## Impact
Users consume double the required allowance, leading to failed transactions or locked funds.

## Command to Run Test


## Proof of Concept
1. User permits 100 tokens to self.
2. User calls `transferFrom(self, other, 50)`.
3. Allowance decreases by 50 (override) + 50 (parent) = 100.
4. User has 0 allowance left despite transferring only 50.

## Proof of Code
function testDoubleAllowance() public { ... }

## Suggested Mitigation
Do not call `_spendAllowance(from, from, value)` if `msg.sender == from`, or rely on parent check for caller allowance and only enforce self-allowance if `msg.sender != from` (but standard requires self-allowance always; likely remove `super` call and implement transfer logic manually).





Finding Status: InvalidByDesign
## [H-6]. Protocol Insolvency Risk due to Multi-Asset Peg Dependency

## id: Nk1-yxDAq6ElC4xWhyui6

## Derived From Pattern/Invariant
Multi-Asset Reserve Desync Arbitrage

## Exploit Type
FlashLoanEconomicManipulation

## Location
ERC7575VaultUpgradeable.requestRedeem

## Finding Status: InvalidByDesign
### Finding Status Justification: "Finding does not exist" is false because the cross-vault depeg arbitrage is feasible given the code prices all assets only by decimal normalization (no oracle). However, it is true this is an explicit design choice (no on-chain market pricing), so it’s an economic/design risk rather than a violated code invariant.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The system allows multiple vaults with chemically different underlying assets (e.g., USDC, DAI) to mint the same fungible `ShareToken` at a 1:1 normalized ratio. The protocol lacks an oracle to verify real-time market rates and assumes a hard peg. If one underlying asset depegs (e.g., USDC drops to $0.90), users can deposit the depegged asset to mint shares at face value and immediately redeem them for the stronger asset (e.g., DAI) from another vault. This arbitrage drains the valuable reserves and leaves the protocol backing the share token with devalued assets.

## Impact
Protocol insolvency; loss of funds for passive liquidity providers.

## Command to Run Test


## Proof of Concept
1. USDC depegs to 0.9 DAI. 2. Attacker buys 1M USDC for 900k DAI. 3. Attacker deposits 1M USDC, gets 1M Shares. 4. Attacker redeems 1M Shares for 1M DAI. 5. Profit 100k DAI. Protocol lost 1M DAI and holds 1M devalued USDC.

## Proof of Code
function test_PegArbitrage() public { ... }

## Suggested Mitigation
Implement Chainlink or similar oracles to verify asset prices during deposits/redemptions and adjust the exchange rate or pause operations if a depeg is detected.


## [H-7]. Multi-Asset Reserve Desync Arbitrage

## id: qxElB9_9XJeZxQGLAGon4

## Derived From Pattern/Invariant
Oracle

## Exploit Type
Oracle

## Location
ShareTokenUpgradeable.convertNormalizedAssetsToShares

## Finding Status: InvalidByDesign
### Finding Status Justification: "Finding does not exist" is false for the same reason as #0: the arbitrage is economically real because conversion ignores market price. It is nevertheless "by design" in that the implemented invariant is only decimal normalization, not $-value parity.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The system allows multiple vaults with different underlying assets (e.g., USDC, DAI) to mint the same fungible `ShareToken`, assuming a fixed 1:1 exchange rate (after decimal normalization). The protocol lacks an oracle to verify real-time market prices. If one underlying asset depegs (e.g., USDC drops to $0.90), the invariant `1 Share = 1 Normalized Unit` breaks. Users can deposit the depegged asset to mint shares at face value and redeem them for a pegged asset (e.g., DAI) from another vault, effectively draining the valuable reserves and diluting other share holders.

## Impact
Drain of vault reserves during asset depeg events.

## Command to Run Test


## Proof of Concept
1. USDC depegs to $0.90. DAI is $1.00.
2. Attacker buys 1000 USDC for $900.
3. Attacker requests deposit 1000 USDC. Manager fulfills (1:1).
4. Attacker gets 1000 Shares.
5. Attacker requests redeem 1000 Shares from DAI vault.
6. Attacker gets 1000 DAI ($1000).
7. Profit $100.

## Proof of Code
function testDepegArbitrage() public { ... }

## Suggested Mitigation
Integrate chainlink oracles to value assets during deposits/withdrawals or restrict redemption to the same asset used for deposit.





Finding Status: LowSeverityDueToRareLikelihood + InvalidNotExploitable + InvalidGovernanceRisk + InvalidFutureSpeculation + InvalidERC20EdgeCase
## [H-8]. Permanent DoS via Broken Vault Dependency in ShareTokenUpgradeable

## id: 20RJ2TZeejCPez5-FHPcw

## Derived From Pattern/Invariant
Dos

## Exploit Type
Dos

## Location
ShareTokenUpgradeable.getCirculatingSupplyAndAssets

## Finding Status: LowSeverityDueToRareLikelihood + InvalidNotExploitable + InvalidGovernanceRisk + InvalidFutureSpeculation + InvalidERC20EdgeCase
### Finding Status Justification: ShareTokenUpgradeable.getCirculatingSupplyAndAssets loops all registered vaults and makes external calls to each vault’s getClaimableSharesAndNormalizedAssets(), with no try/catch. If any one vault’s call reverts, conversions (convertNormalizedAssetsToShares / convertSharesToNormalizedAssets) revert and can block fulfillDeposit/fulfillRedeem flows system-wide. Additionally, unregisterVault ends with a direct IERC20(asset).balanceOf(vaultAddress) call outside the earlier try/catch; if balanceOf itself reverts, the vault cannot be removed. However, the scenario requires a non-standard/malicious asset (or a governance-chosen integration) where balanceOf/relevant view paths revert; standard ERC20s do not typically do this. Hence it is mainly a governance/integration risk and somewhat speculative.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `ShareTokenUpgradeable.getCirculatingSupplyAndAssets` function iterates over all registered vaults to aggregate assets. It calls `vault.getClaimableSharesAndNormalizedAssets()`, which calls `vault.totalAssets()`, which calls `IERC20(asset).balanceOf(vault)`. If a single underlying asset reverts on `balanceOf` (e.g. paused token, malicious upgrade, or hacked adapter), the entire ShareToken accounting reverts. This bricks `convertNormalizedAssetsToShares` and `fulfillDeposit` for ALL vaults. Crucially, `unregisterVault` also performs the same `totalAssets()` check to verify the vault is empty, so the Owner cannot remove the broken vault to restore system functionality. The protocol becomes permanently frozen.

## Impact
Permanent system freeze; deposits and rate calculations fail for all vaults.

## Command to Run Test


## Proof of Concept
1. Register a vault with a Pausable token (e.g. USDC).
2. Token admin pauses USDC.
3. `balanceOf` reverts.
4. `getCirculatingSupplyAndAssets` reverts.
5. `unregisterVault` reverts (cannot remove).
6. System bricked.

## Proof of Code
function testDosWithBrokenVault() public { ... }

## Suggested Mitigation
Wrap external calls in try-catch blocks within the loop and allow unregistration without querying the broken vault (e.g. emergency force remove).





Finding Status: LowSeverityDueToRareLikelihood + LowSeverityDueToLowImpact + InvalidNotExploitable + InvalidGovernanceRisk
## [M-9]. Unsafe UUPS Implementation Bypassing Verification

## id: -WiKI3ZmbmT7tSLTwX2vb

## Derived From Pattern/Invariant
StandardViolation

## Exploit Type
StandardViolation

## Location
ShareTokenUpgradeable.upgradeTo

## Finding Status: LowSeverityDueToRareLikelihood + LowSeverityDueToLowImpact + InvalidNotExploitable + InvalidGovernanceRisk
### Finding Status Justification: Both ERC7575VaultUpgradeable and ShareTokenUpgradeable expose upgradeTo/upgradeToAndCall that directly call ERC1967Utils.upgradeToAndCall without the standard UUPSUpgradeable proxiableUUID/implementation compatibility checks. This makes it possible for the owner to upgrade the proxy to an implementation that lacks future upgrade functions, effectively bricking upgrades. This is a real missing safety mechanism in the current code, but it is not permissionless: onlyOwner can trigger it, and it manifests primarily as an “admin can brick themselves” governance/operational risk (which contest rules generally treat as low severity under trusted-admin assumptions).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
Both `ShareTokenUpgradeable` and `ERC7575VaultUpgradeable` inherit from `UUPSUpgradeable` but implement a custom `upgradeTo` function that calls `ERC1967Utils.upgradeToAndCall` directly. This bypasses the standard UUPS safety mechanism (normally enforced via `_authorizeUpgrade` and `UUPSUpgradeable`'s version of `upgradeTo`) which verifies that the new implementation contract supports UUPS upgrades (`proxiableUUID` check). Upgrading to a logic contract that lacks this UUID or the upgrade function will permanently brick the proxy, preventing future upgrades.

## Impact
Permanent bricking of the proxy contract if an invalid implementation is set.

## Command to Run Test


## Proof of Concept
1. Owner calls `upgradeTo(invalidImpl)`.
2. `invalidImpl` has no upgrade function.
3. Proxy delegates to `invalidImpl`.
4. Proxy can no longer be upgraded.

## Proof of Code
function testUnsafeUpgrade() public { ... }

## Suggested Mitigation
Remove the custom `upgradeTo` function and instead override `_authorizeUpgrade` to restrict access, allowing the standard `UUPSUpgradeable` implementation to handle verification.





Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
## [M-10]. TotalSupply Invariant Violation via Unsafe Recipient in Batch Transfers

## id: bF1-74HN_kohLgzJ0z81s

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
WERC7575ShareToken.batchTransfers

## Finding Status: LowSeverityDueToRareLikelihood + InvalidGovernanceRisk
### Finding Status Justification: "Low impact" is false: if the validator misuses/gets compromised, including address(0) can desync sum(balances) vs totalSupply and can effectively create/move phantom balances, which is high-impact even if it is gated behind a trusted role.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresRole


## Description
The `WERC7575ShareToken.batchTransfers` function directly modifies `_balances` based on input arrays without checking for `address(0)` or updating `_totalSupply` (it bypasses the `_update` override). If the validator passes `address(0)` as a creditor, `_balances[address(0)]` increases but `_totalSupply` remains unchanged. If `address(0)` is a debtor, it decreases. This breaks the ERC20 invariant `totalSupply == sum(balances)` and allows phantom minting/burning if `address(0)` is used.

## Impact
Corruption of token accounting; potential for infinite minting if address(0) is used as source.

## Command to Run Test


## Proof of Concept
1. Validator calls `batchTransfers` with `creditor = address(0)`. 2. `_balances[0]` increases. 3. `totalSupply` is unchanged. 4. Validator calls `batchTransfers` with `debtor = address(0)` (if balance exists). 5. Phantom tokens moved to real user.

## Proof of Code
function testZeroAddressBatch() public { ... }

## Suggested Mitigation
Add `if (debtor == address(0) || creditor == address(0)) revert` checks in `batchTransfers` and `consolidateTransfers`.





Finding Status: LowSeverityDueToRareLikelihood + LowSeverityDueToLowImpact + InvalidSafeGuardInPlace + InvalidNotExploitable + InvalidFutureSpeculation + InvalidByDesign + InvalidBugDoesNotExist
## [M-11]. Share Price Manipulation via Donation

## id: 4M0iFMh5iFqgU_QvCgp9y

## Derived From Pattern/Invariant
FlashLoanEconomicManipulation

## Exploit Type
FlashLoanEconomicManipulation

## Location
ShareTokenUpgradeable.convertNormalizedAssetsToShares

## Finding Status: LowSeverityDueToRareLikelihood + LowSeverityDueToLowImpact + InvalidSafeGuardInPlace + InvalidNotExploitable + InvalidFutureSpeculation + InvalidByDesign + InvalidBugDoesNotExist
### Finding Status Justification: ShareTokenUpgradeable._calculateInvestmentAssets intentionally uses IERC20(investmentShareToken).balanceOf(this) (+ optional rBalanceOf) as the invested-assets component. Donating investmentShareToken to the ShareToken contract will increase totalNormalizedAssets and thus change the conversion rate, but this is the standard ERC4626 ‘donation increases assets’ behavior: the donor cannot extract value without already owning shares (and even then it is generally value-neutral, aside from rounding). The code also includes VIRTUAL_SHARES/VIRTUAL_ASSETS, which mitigates extreme inflation effects for small supply. The claimed ‘dust shares’ grief is not supported as a strong issue: fulfillDeposit reverts on zero shares and minimum deposits are large; there is no clear profit path.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `ShareTokenUpgradeable` calculates the global share price based on `totalNormalizedAssets`, which includes the balance of `investmentShareToken` held by the contract. An attacker can donate `investmentShareToken` assets directly to the `ShareTokenUpgradeable` contract to artificially inflate `totalNormalizedAssets` and thus the share price (`assets/supply`). While a self-donation/redeem cycle is typically zero-sum for the attacker, this manipulation can be used to grief other users (e.g., inflating price right before a `fulfillDeposit` executes, causing the user to receive dust shares) or exploit external systems relying on the share price.

## Impact
Griefing of user deposits; potential manipulation of external protocols.

## Command to Run Test


## Proof of Concept
1. Attacker observes user deposit request.
2. Before fulfillment, attacker donates large amount of investment tokens to ShareToken.
3. `totalNormalizedAssets` spikes. Share price spikes.
4. `fulfillDeposit` executes. User gets negligible shares for their assets.

## Proof of Code
function testDonationAttack() public { ... }

## Suggested Mitigation
Use an internal accounting variable for `investedAssets` instead of `balanceOf(this)`.





Finding Status: LowSeverityDueToRareLikelihood + LowSeverityDueToLowImpact + InvalidSafeGuardInPlace + InvalidNotExploitable + InvalidFutureSpeculation + InvalidERC20EdgeCase + InvalidBugDoesNotExist
## [M-12]. Read-Only Reentrancy in `totalAssets` via `requestDeposit`

## id: UXdV2xihBeO3ixFMeJE42

## Derived From Pattern/Invariant
Reentrancy

## Exploit Type
Reentrancy

## Location
ERC7575VaultUpgradeable.requestDeposit

## Finding Status: LowSeverityDueToRareLikelihood + LowSeverityDueToLowImpact + InvalidSafeGuardInPlace + InvalidNotExploitable + InvalidFutureSpeculation + InvalidERC20EdgeCase + InvalidBugDoesNotExist
### Finding Status Justification: requestDeposit does transfer assets before incrementing pendingDepositAssets, but it is protected by nonReentrant, which blocks reentrant state-changing calls back into requestDeposit/fulfill/claim flows. The only remaining window is a read-only observation where a hook-enabled token (e.g., ERC777) calls out to an attacker contract that queries totalAssets mid-transfer. That requires a non-standard underlying with transfer hooks and an external protocol that trusts this transient view during the same call stack. No direct on-chain theft or state corruption is shown against this protocol itself. Given the protocol’s intended standard ERC20 stablecoin assets and the reentrancy guard, the reported impact is largely speculative/integration-dependent.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `ERC7575VaultUpgradeable.requestDeposit`, assets are transferred from the user using `safeTransferFrom` *before* the `pendingDepositAssets` state is updated. If the underlying asset supports transfer hooks (e.g., ERC-777 or ERC-677), an attacker can trigger code execution after the balance update but before the state update. During this window, `totalAssets()` (calculated as `balanceOf(this) - reservedAssets`) will be artificially inflated because the balance has increased but `reservedAssets` (which includes pending deposits) has not yet updated. This manipulates the share price for any external protocol reading it.

## Impact
Manipulation of share price allowing theft of funds in integrated protocols.

## Command to Run Test


## Proof of Concept
1. Attacker contracts calls `requestDeposit` with ERC777 token.
2. `tokensReceived` hook fires.
3. Inside hook, `totalAssets` is checked. It is inflated.
4. Attacker performs arbitrage against the inflated price.

## Proof of Code
function testReadOnlyReentrancy() public { ... }

## Suggested Mitigation
Update state variables (increment pending assets) before transferring tokens (Checks-Effects-Interactions).





Finding Status: LowSeverityDueToLowImpact + InvalidSafeGuardInPlace + InvalidNotExploitable + InvalidByDesign + InvalidBugDoesNotExist
## [M-13]. Missing Slippage Protection in Async Request Functions

## id: FXdAc2Ybka7dEXgtd_7CQ

## Derived From Pattern/Invariant
SlippageMissingOrInsufficient

## Exploit Type
SlippageMissingOrInsufficient

## Location
ERC7575VaultUpgradeable.requestDeposit

## Finding Status: LowSeverityDueToLowImpact + InvalidSafeGuardInPlace + InvalidNotExploitable + InvalidByDesign + InvalidBugDoesNotExist
### Finding Status Justification: requestDeposit/requestRedeem intentionally initiate an async flow where the exchange rate is determined at fulfill time. The implementation includes user-facing cancellation for pending requests (ERC7887) which serves as the main protection against unfavorable fills over time. ERC7540 itself does not require minShares/minAssets parameters at request time. Therefore the absence of explicit slippage parameters is a design/UX choice consistent with the async model (users either accept fulfill timing/rate or cancel while pending), not a broken code path or bypassed invariant.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `requestDeposit` and `requestRedeem` functions in `ERC7575VaultUpgradeable` initiate an asynchronous flow where users lock assets/shares, but the exchange rate is determined later at fulfillment time. These functions lack `minShares` or `minAssets` parameters. If the share price moves unfavorably (due to market volatility or manipulation) between request and fulfillment, users are forced to accept the unfavorable rate without recourse. This violates the safety expectations for async vaults where time delays introduce price risk.

## Impact
Users may receive significantly fewer shares or assets than expected due to price changes.

## Command to Run Test


## Proof of Concept
1. User calls `requestDeposit` expecting 1:1 rate. 2. Share price doubles due to oracle update or manipulation. 3. Manager calls `fulfillDeposit`. 4. User receives 50% of expected shares.

## Proof of Code
function testSlippage() public { ... }

## Suggested Mitigation
Add `minShares` to `requestDeposit` and `minAssets` to `requestRedeem`. Store these in the Request struct and validate during fulfillment or allow user cancellation if limits not met.


## [M-14]. Share Price Manipulation via Donation and Spot Balance Dependency

## id: 6krXIVjxnqT98_6NkbMVE

## Derived From Pattern/Invariant
Share Price Manipulation via Donation (Spot Balance Dependency)

## Exploit Type
FlashLoanEconomicManipulation

## Location
ERC7575VaultUpgradeable.totalAssets

## Finding Status: LowSeverityDueToLowImpact + InvalidSafeGuardInPlace + InvalidNotExploitable + InvalidByDesign + InvalidBugDoesNotExist
### Finding Status Justification: "Future speculation" is false: spot-balance donation effects (balanceOf-based totalAssets) are an immediate, present-state property. The better characterization is that it’s an accepted/by-design ERC4626-style property and generally not profitably exploitable, rather than something that only appears in a future/post-upgrade state.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `totalAssets` function in `ERC7575VaultUpgradeable` relies directly on `IERC20(asset).balanceOf(address(this))`. This allows an attacker to manipulate the share price by donating assets to the vault to inflate `totalAssets`. While `ShareToken` uses virtual offsets to mitigate inflation attacks, the direct spot balance dependency still allows manipulation of the exchange rate used in `fulfillDeposit` and `fulfillRedeem`. An attacker can sandwich an Investment Manager's fulfillment transaction with a donation to force fulfillment at an unfavorable rate for the user.

## Impact
Griefing of user deposits/redemptions; financial loss for users if manager fulfills at manipulated rates.

## Command to Run Test


## Proof of Concept
1. User requests deposit. 2. Attacker sees Manager's fulfill tx. 3. Attacker front-runs with large donation to Vault. 4. `totalAssets` spikes. 5. Manager tx executes: User gets fewer shares than expected. 6. Attacker cannot easily reclaim donation, but can grief or exploit if they hold a large share position.

## Proof of Code
function test_SpotManipulation() public { ... }

## Suggested Mitigation
Use an internal accounting variable for total assets that is only updated via valid deposit/mint/invest functions, ignoring direct transfers (donations).





Finding Status: InvalidGovernanceRisk
## [H-15]. Share Price Inflation via Insolvency Clamping in totalAssets

## id: 6iphefa32ktuPNs2IFBhJ

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
ERC7575VaultUpgradeable.totalAssets

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: ERC7575VaultUpgradeable.totalAssets returns max(balance - reservedAssets, 0), where reservedAssets includes totalClaimableRedeemAssets. ShareTokenUpgradeable.getCirculatingSupplyAndAssets excludes claimable redeem shares from circulatingSupply, but totalNormalizedAssets still includes invested assets held at ShareTokenUpgradeable level. If a vault has invested away its liquid assets and fulfillRedeem is called (creating claimableRedeemAssets) before withdrawing liquidity back, reservedAssets can exceed balance so totalAssets clamps to 0, while invested assets remain fully counted. This combination can inflate the computed price for subsequent conversions (reduced circulating supply without correspondingly reducing counted assets), leading to mispricing and potentially over-allocation of claimable assets. It is reachable in normal operations (manager-controlled) and can create insolvency/misaccounting risk if fulfill sequencing is not perfectly managed.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:RequiresAdminRole


## Description
In `ERC7575VaultUpgradeable.totalAssets()`, the function returns `balance > reservedAssets ? balance - reservedAssets : 0`. `reservedAssets` includes `totalClaimableRedeemAssets`. If the Investment Manager invests all liquid assets (so `balance` becomes 0) and then fulfills a redemption request (converting pending shares to `claimableRedeemAssets`), `reservedAssets` will exceed `balance`. The function will return 0 due to clamping. However, `ShareTokenUpgradeable` calculates the global share price by summing `totalAssets()` from all vaults (0) and adding `investedAssets` (full amount) while subtracting the redeemed shares from `circulatingSupply`. This effectively counts the assets twice (once in `investedAssets` and implicitly again by not deducting the liability from `totalAssets`), causing `totalNormalizedAssets` to be overstated relative to `circulatingSupply`, leading to an artificial spike in share price.

## Impact
Artificial inflation of share price causing loss of value for remaining shareholders or unfair redemption rates.

## Command to Run Test


## Proof of Concept
1. Vault has 100 USDC. User A has 100 Shares (Supply=100). 
2. Manager invests 100 USDC. `balance`=0, `invested`=100.
3. User A requests redeem 100 Shares. Manager fulfills. `claimableRedeemAssets`=100.
4. `reserved`=100. `totalAssets` = 0 (clamped). `ShareToken.assets` = 100 (invested). `circulatingSupply` = 0 (100 - 100).
5. Scenario with 2 users: A redeems 50. `reserved`=50. `circulating`=50. `assets`=100. Price = 2.0 (Double!).

## Proof of Code
function testSharePriceInflation() public { ... }

## Suggested Mitigation
Ensure `totalAssets` allows negative accounting or `ShareToken` subtracts reserved assets from `investedAssets`.



