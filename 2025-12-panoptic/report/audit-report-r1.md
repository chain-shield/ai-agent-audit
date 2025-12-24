# 2025 12 panoptic - Findings Report
## Commit hash: a4361d6d8dc6420c09187d80ea1a7ce851d1ca36

##Findings by Status


Finding Status: Valid


[M-1]. Yield leakage and suppressed borrowing costs due to capped interest rate adaptation interval in RiskEngine
**Derived From** : UnincentivizedMaintenanceOrKeeperlessProgress
Finding Status: Valid
Privilege: Permissionless


[H-2]. Unauthorized takeover of BuilderWallet via unprotected init() function
**Derived From** : AccessControlOrAuthByPass
Finding Status: Valid
Privilege: Permissionless


[H-3]. Permanent Yield Loss for PLPs due to `s_creditedShares` Accounting Drift in `CollateralTracker`
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless


[H-4]. DoS on functions utilizing builder codes due to insufficient bit allocation for feeRecipient address in RiskParameters
**Derived From** : Storage Layout
Finding Status: Valid
Privilege: Permissionless


[H-5]. Protocol insolvency via phantom share transfer in forceExercise
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless


[H-6]. Protocol Loss Exacerbation due to Inverse Haircut Logic in Liquidations
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless


[H-7]. Liquidation with protocol loss mints shares to insolvent liquidatee instead of liquidator
**Derived From** : Access Control and Logic Flaw in Liquidation Settlement
Finding Status: Valid
Privilege: Permissionless


[H-8]. Safe Mode Bypass in PanopticPool.validateCollateralWithdrawable Allows Insolvency Escape
**Derived From** : AuthByPass
Finding Status: Valid
Privilege: Permissionless


[H-9]. Stale Oracle Data used in Solvency Checks allows Undercollateralized Positions
**Derived From** : OracleUsingDEXorTWAP
Finding Status: Valid
Privilege: Permissionless


[M-10]. Force Exercise Fee Avoidance via Spot Price Manipulation
**Derived From** : FlashLoanEconomicManipulation
Finding Status: Valid
Privilege: Permissionless


[M-11]. Premium accounting corruption due to `s_accountLiquidity` key collision across `vegoid`s
**Derived From** : Storage Collision
Finding Status: Valid
Privilege: Permissionless


[H-12]. Donation Attack via Liquidity Manipulation in SFPM Amplifies Premium Debt
**Derived From** : FlashLoanEconomicManipulation
Finding Status: Valid
Privilege: Permissionless


[H-13]. Loss of Protocol Fees due to Incorrect Capping Logic in LeftRightLibrary
**Derived From** : IntegerOverflow
Finding Status: Valid
Privilege: Permissionless


[H-14]. Interest Debt Wipe via Force Exercise Delegation Mechanism
**Derived From** : IncentiveMisalignmentOrGameTheory
Finding Status: Valid
Privilege: Permissionless


[H-15]. Protocol Insolvency via Invalid Risk Partner Chaining due to Missing TokenId Validation
**Derived From** : AccessControlOrAuthByPass
Finding Status: Valid
Privilege: Permissionless


[M-16]. DoS on partial position closing during Safe Mode due to strict solvency checks
**Derived From** : EmergencyModeStateStuck
Finding Status: Valid
Privilege: Permissionless


[H-17]. DoS of Liquidation due to `MAX_SPREAD` check in `PanopticPool` bricks bad debt clearance
**Derived From** : DoS
Finding Status: Valid
Privilege: Permissionless


[H-18]. TokenId.validate allows active legs to partner with inactive legs, bypassing collateral requirements
**Derived From** : Input Validation
Finding Status: Valid
Privilege: Permissionless


[M-19]. Accounting Corruption due to Shared Liquidity Storage across Vegoids
**Derived From** : StorageLayout
Finding Status: Valid
Privilege: Permissionless


[H-20]. Accounting collision in `SemiFungiblePositionManager` allows bypassing fee spread tiers via `positionKey` conflict
**Derived From** : ExternalProtocolKeyCollision
Finding Status: Valid
Privilege: Permissionless


[H-21]. Yield Theft via Premium Accumulator Overflow from Dust Liquidity Manipulation
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless


[H-22]. Premia accumulator freeze in SemiFungiblePositionManager enables free option positions and griefs liquidity providers
**Derived From** : IntegerOverflow
Finding Status: Valid
Privilege: Permissionless


[M-23]. Permanent Denial of Service in SFPM Pools via Zero Vegoid Initialization
**Derived From** : Dos
Finding Status: Valid
Privilege: Permissionless


[H-24]. DoS of Liquidation via Max Spread Check in PanopticPool._burnOptions
**Derived From** : Dos
Finding Status: Valid
Privilege: Permissionless


[H-25]. Solvent users can evade interest payments via Force Exercise or Settle Premium due to incorrect delegation ordering
**Derived From** : Call Ordering / Checks-Effects-Interactions
Finding Status: Valid
Privilege: Permissionless


[M-26]. PLPs receive zero commission yield when Builder Code is used
**Derived From** : FeeAccountingDrift
Finding Status: Valid
Privilege: Permissionless


[H-27]. Permanent share price inflation due to unrecoverable bad debt in CollateralTracker
**Derived From** : AccountingInvariantViolation
Finding Status: Valid
Privilege: Permissionless


[H-28]. Theft of funds via Force Exercise fee manipulation using Flash Loans
**Derived From** : FlashLoanEconomicManipulation
Finding Status: Valid
Privilege: Permissionless


[M-29]. Incentive misalignment in `RiskEngine.exerciseCost` rewards price manipulation against forced exercise targets
**Derived From** : IncentiveMisalignmentOrGameTheory
Finding Status: Valid
Privilege: Permissionless


[H-30]. Flash Loan Price Manipulation allows bypassing Force Exercise Fees in RiskEngine
**Derived From** : FlashLoanEconomicManipulation
Finding Status: Valid
Privilege: Permissionless



Finding Status: LowSeverityDueToLowImpact


[H-31]. Broken ITM Swap Logic in SFPM Swaps Asset Tokens to Non-Asset Tokens
**Derived From** : IncentiveMisalignmentOrGameTheory
Finding Status: LowSeverityDueToLowImpact
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood + InvalidERC20EdgeCase


[H-32]. Read-Only Reentrancy in CollateralTracker.deposit via ERC777 hooks allows theft of vault value
**Derived From** : ERC777HookReentrancy
Finding Status: LowSeverityDueToRareLikelihood + InvalidERC20EdgeCase
Privilege: Permissionless


[H-33]. Infinite Share Minting and Vault Drainage via Self-Liquidation Reentrancy in CollateralTracker
**Derived From** : ERC777HookReentrancy
Finding Status: LowSeverityDueToRareLikelihood + InvalidERC20EdgeCase
Privilege: Permissionless


[M-34]. Storage corruption in initializeAMMPool via reentrancy leads to poolId collision and DoS of tick expansion
**Derived From** : StorageCollisionOrSelectorClash
Finding Status: LowSeverityDueToRareLikelihood + InvalidERC20EdgeCase
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood


[H-35]. Theft of stuck ETH in PanopticPool via Multicall msg.value reuse
**Derived From** : Unexpected Ether
Finding Status: LowSeverityDueToRareLikelihood
Privilege: Permissionless


[H-36]. Silent overflow of unrealizedInterest in MarketStateLibrary causes permanent loss of protocol assets
**Derived From** : AccountingInvariantViolation
Finding Status: LowSeverityDueToRareLikelihood
Privilege: Permissionless


[H-37]. DoS of Liquidation and Force Exercise due to `_internalSupply` Underflow in `CollateralTracker._burn`
**Derived From** : AccountingInvariantViolation
Finding Status: LowSeverityDueToRareLikelihood
Privilege: Permissionless



Finding Status: InvalidERC20EdgeCase


[H-38]. Reentrancy in settleLiquidation allows theft via phantom share withdrawal
**Derived From** : ERC777HookReentrancy
Finding Status: InvalidERC20EdgeCase
Privilege: Permissionless



Finding Status: InvalidOutOfScope


[H-39]. Permanent Accounting Invariant Violation and Bad Debt Accumulation due to Insolvency Masking in RiskEngine
**Derived From** : AccountingInvariantViolation
Finding Status: InvalidOutOfScope
Privilege: Permissionless


[H-40]. LP Dilution via Phantom Share Interest Payment Exploit
**Derived From** : AccountingInvariantViolation
Finding Status: InvalidOutOfScope
Privilege: Permissionless


[M-41]. Permanent share supply inflation due to incorrect phantom share tracking in delegate/revoke flow
**Derived From** : AccountingInvariantViolation
Finding Status: InvalidOutOfScope
Privilege: Permissionless


[H-42]. Interest Evasion via self-Force-Exercise and Revoke
**Derived From** : Phantom shares from delegation allow evading interest payments
Finding Status: InvalidOutOfScope
Privilege: Permissionless


[M-43]. Liquidation DoS due to priority of interest deduction over liquidation bonus
**Derived From** : Incentive / Game Theory (Griefing DoS)
Finding Status: InvalidOutOfScope
Privilege: Permissionless


[H-44]. Solvency Check Bypass via Insolvency Masking in Margin Calculation
**Derived From** : AccountingInvariantViolation
Finding Status: InvalidOutOfScope
Privilege: Permissionless



Finding Status: InvalidGovernanceRisk


[M-45]. Initialization Revert for Native Pools due to Logarithm of Zero
**Derived From** : UnexpectedEth
Finding Status: InvalidGovernanceRisk
Privilege: Permissionless



Finding Status: InvalidSafeGuardInPlace


[H-46]. Forced Exercise Fee Manipulation via Flash Loan allowing cheap destruction of OTM/ITM positions
**Derived From** : FlashLoanEconomicManipulation
Finding Status: InvalidSafeGuardInPlace
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood + InvalidNotExploitable


[M-47]. Loss of yield and accounting corruption due to `unrealizedInterest` overflow in `MarketState` for high-supply tokens
**Derived From** : Integer Overflow
Finding Status: LowSeverityDueToRareLikelihood + InvalidNotExploitable
Privilege: Permissionless


[M-48]. Protocol bricking due to borrowIndex overflow corrupting MarketState storage
**Derived From** : IntegerOverflow
Finding Status: LowSeverityDueToRareLikelihood + InvalidNotExploitable
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 34
- M: 14
- L: 0
- I: 0

##Findings by Status


Finding Status: Valid
## [M-1]. Yield leakage and suppressed borrowing costs due to capped interest rate adaptation interval in RiskEngine

## id: XeM7pDZlz8IoQpJcVfhWT

## Derived From Pattern/Invariant
UnincentivizedMaintenanceOrKeeperlessProgress

## Exploit Type
IncentiveMisalignmentOrGameTheory

## Location
RiskEngine._borrowRate

## Finding Status: Valid
### Finding Status Justification: The cap in RiskEngine._borrowRate (IRM_MAX_ELAPSED_TIME) combined with CollateralTracker applying the returned avgRate to the full deltaTime creates a real under-accrual vs continuous/stepwise adaptation; there is no compensating mechanism that iterates updates over long inactivity. Inactivity >4096s is common in less-active pools, so it is not truly low-likelihood, and borrowers can benefit from periods where no one triggers accrual.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The Adaptive Interest Rate Model (IRM) in `RiskEngine.sol` employs a `PID` controller to adjust the `rateAtTarget` based on utilization error. However, the `_borrowRate` function caps the time elapsed for rate adaptation to `IRM_MAX_ELAPSED_TIME` (4096 seconds, ~1.1 hours). 

When `CollateralTracker._accrueInterest` is called after a long period of inactivity (e.g., 24 hours) with high utilization:
1. `_updateInterestRate` calls `RiskEngine.updateInterestRate`, which calculates the new `rateAtTarget` assuming only ~1 hour has passed. The rate increases much less than it should given the sustained high utilization.
2. It returns an `avgRate` representing the average rate during that capped 1-hour interval.
3. `_accrueInterest` then applies this suppressed `avgRate` to the *entire* `deltaTime` (24 hours) via `_calculateCurrentInterestState`.

This results in borrowers paying significantly less interest than the model intends for sustained high utilization periods. It creates an incentive for large borrowers to ensure pool inactivity (e.g., by not interacting) to keep rates artificially low, causing yield leakage for Panoptic Liquidity Providers (PLPs).

## Impact
Lenders (PLPs) suffer a loss of yield as interest rates fail to adapt to market conditions during periods of low activity. Borrowers can exploit this to maintain cheap leverage.

## Command to Run Test


## Proof of Concept
1. Assume `rateAtTarget` is 5% and utilization is 100% (Target is 66%). The rate should climb rapidly to `MAX_RATE_AT_TARGET`.
2. Attacker (borrower) holds a large short position and ensures no interactions occur with the `CollateralTracker` for 24 hours.
3. After 24 hours, a transaction triggers `_accrueInterest`.
4. `_borrowRate` calculates `elapsed = min(24h, 1.1h) = 1.1h`.
5. `rateAtTarget` adapts slightly (e.g., to 5.5%). `avgRate` returned is ~5.25%.
6. Interest is calculated as `5.25% * 24 hours`.
7. If the cap were not present, `rateAtTarget` might have reached 50%+, and the average rate for the day would be much higher (e.g., 25%).
8. The borrower saves a massive amount of interest.

## Proof of Code
function test_IRMCapLeakage() public {
    // Setup: Mint options to reach 90% utilization
    // ... (setup code) ...
    
    // Snapshot state
    uint256 prevAssets = ct.totalAssets();
    
    // Warp 24 hours
    vm.warp(block.timestamp + 1 days);
    
    // Trigger interest accrual
    ct.accrueInterest();
    
    // Calculate accrued interest
    uint256 accrued = ct.totalAssets() - prevAssets;
    
    // Expectation: If rate adapted for 24h, accrued should be High.
    // Reality: Accrued is Low because adaptation stopped after 1h.
    // assertLt(accrued, expectedFairValue);
}

## Suggested Mitigation
Remove or significantly increase `IRM_MAX_ELAPSED_TIME` to allow the interest rate to adapt correctly over long periods of inactivity, or implement an iterative calculation that updates the rate in steps if `elapsed` exceeds the limit.


## [H-2]. Unauthorized takeover of BuilderWallet via unprotected init() function

## id: AUfKm7u2kTdoY2e7FLKm5

## Derived From Pattern/Invariant
AccessControlOrAuthByPass

## Exploit Type
AccessControl

## Location
BuilderWallet.init

## Finding Status: Valid
### Finding Status Justification: BuilderWallet.init is externally callable with no onlyFactory restriction and no single-initialization guard, so overwriting builderAdmin is possible. sweep() is gated only by builderAdmin, so takeover directly enables token theft. This is exploitable and high impact; there is no effective safeguard and it is not a reasonable intended design.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `BuilderWallet` contract, defined within `RiskEngine.sol`, contains an `init` function that sets the `builderAdmin` address. This function is declared as `external` and lacks any access control modifiers or checks to ensure it hasn't been called previously. 

 Vulnerable snippet in `RiskEngine.sol` (BuilderWallet): 
 ```solidity
 function init(address _builderAdmin) external {
     builderAdmin = _builderAdmin;
 }
 ``` 

 While the `BuilderFactory` calls this function immediately after deployment in `deployBuilder`, the function remains open. An attacker can call `init` on any existing `BuilderWallet` instance to overwrite the `builderAdmin` with their own address. Once they are the admin, they can call `sweep` to drain any tokens collected by the wallet.

## Impact
An attacker can hijack any BuilderWallet and steal all accumulated referral fees/commissions.

## Command to Run Test


## Proof of Concept
1. Honest builder deploys a wallet via `BuilderFactory.deployBuilder(code, admin)`. The wallet is created and `builderAdmin` is set to `admin`. 
2. Attacker observes the deployed wallet address. 
3. Attacker calls `BuilderWallet(wallet).init(attackerAddress)`. 
4. The `builderAdmin` is updated to `attackerAddress`. 
5. Attacker calls `sweep(token, attackerAddress)` to drain funds.

## Proof of Code
contract BuilderWalletExploitTest is Test {
    BuilderWallet wallet;
    address factory = address(0x123);
    address honestAdmin = address(0xABC);
    address attacker = address(0xBAD);

    function setUp() public {
        // Deploy wallet as factory would
        wallet = new BuilderWallet(factory);
        wallet.init(honestAdmin);
    }

    function testHijack() public {
        assertEq(wallet.builderAdmin(), honestAdmin);
        
        vm.prank(attacker);
        wallet.init(attacker);
        
        assertEq(wallet.builderAdmin(), attacker);
    }
}

## Suggested Mitigation
Add a check in `init` to ensure it can only be called once, or restrict caller to the `FACTORY`. 
 Example fix: 
 `require(builderAdmin == address(0), "Already initialized");` 
 or 
 `require(msg.sender == FACTORY, "Not factory");`


## [H-3]. Permanent Yield Loss for PLPs due to `s_creditedShares` Accounting Drift in `CollateralTracker`

## id: CDTu5j2FEfqIwoEIPSa-E

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
CollateralTracker._updateBalancesAndSettle

## Finding Status: Valid
### Finding Status Justification: Because s_creditedShares is adjusted using the current exchange rate at burn (and mint uses rounding-up), changes in totalAssets/totalSupply over time can leave residual creditedShares after a long is closed; there is no per-position credited-share tracking or other reconciliation to guarantee full reversal. This is an accounting drift that can persist and accumulate under typical share-price changes, diluting real shareholders.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `CollateralTracker` contract maintains a global variable `s_creditedShares` to track the amount of virtual shares minted to back long option positions (when `longAmount > 0`). When a long position is created, `creditDelta` shares are added to `s_creditedShares` based on the current share price (`totalSupply / totalAssets`). When the position is closed (burned), `creditDelta` shares are removed, recalculated using the *new* current share price. 

Because the share price naturally increases over time due to interest accrual (`totalAssets` grows while `totalSupply` is constant for interest), the number of shares corresponding to the same fixed asset amount decreases. Consequently, the amount of shares removed (`creditDelta_burn`) is strictly less than the amount originally added (`creditDelta_mint`).

This results in a residual positive balance in `s_creditedShares` even after all long positions are closed. Since `s_creditedShares` is part of the `totalSupply()` calculation, this permanently inflates the total supply. The inflated supply dilutes the value of real shares held by Panoptic Liquidity Providers (PLPs), causing a permanent leak of assets/yield. These assets remain locked in the protocol as they are theoretically owned by the 'phantom' shares that no one holds.

## Impact
Panoptic Liquidity Providers (PLPs) suffer a permanent loss of yield and principal over time as `s_creditedShares` accumulates 'dust' from every long option cycle, permanently diluting the share price.

## Command to Run Test


## Proof of Concept
1. Alice deposits 1e18 assets into `CollateralTracker`. `totalSupply` = 1e18 + 1e6 (internal). Share Price = 1.
2. Bob mints a long option worth 1e18 assets. `creditDelta_mint` = 1e18 shares (approx). `s_creditedShares` += 1e18.
3. Time passes, interest accrues in the pool (e.g. from other borrowers). `totalAssets` grows to 1.1e18 (per original share). Share Price becomes 1.1.
4. Bob burns his long option (worth 1e18 assets). `creditDelta_burn` = 1e18 / 1.1 = ~0.909e18 shares.
5. `s_creditedShares` decreases by 0.909e18. Residual `s_creditedShares` = 0.091e18.
6. Even though Bob's position is closed, `totalSupply` remains inflated by 0.091e18 ghost shares.
7. Alice tries to withdraw. She owns 1e18 shares. Total Supply is ~1.091e18. She gets `1e18 / 1.091e18 * TotalAssets`. She receives less than her fair share of the assets (principal + interest). The difference is locked in the protocol.

## Proof of Code
        // Insert in CollateralTracker.t.sol or similar
        function test_CreditedSharesDrift() public {
            // 1. Setup: Deposit initial liquidity
            vm.startPrank(alice);
            token.approve(address(collateralTracker), 100 ether);
            collateralTracker.deposit(100 ether, alice);
            vm.stopPrank();

            // 2. Bob mints a long option (requires another user to mint short first to provide liquidity, simplified here)
            // Assume Bob mints a position that results in 10 ether worth of long value
            // This increases s_creditedShares by ~10 ether worth of shares
            
            // 3. Simulate Interest Accrual / Share Price Increase
            // Manually increase totalAssets via donation or mock interest
            vm.prank(address(collateralTracker));
            token.transfer(address(collateralTracker), 10 ether); // 10% growth

            // 4. Bob burns the position
            // The burn logic calculates shares based on NEW price (1.1x)
            // Shares removed = 10 ether / 1.1 = 9.09 shares
            // Original added = 10 shares
            // Drift = 0.91 shares remaining in s_creditedShares
            
            // 5. Assert totalSupply is higher than expected
            // assertGt(collateralTracker.totalSupply(), expectedSupply);
        }

## Suggested Mitigation
Track the exact number of shares credited for each long position leg at the time of minting (e.g., in `PositionBalance` or a separate mapping) and remove that exact amount upon burning, instead of recalculating the share amount based on the current exchange rate.


## [H-4]. DoS on functions utilizing builder codes due to insufficient bit allocation for feeRecipient address in RiskParameters

## id: cqu6lWNe63sXbZ5f_1btb

## Derived From Pattern/Invariant
Storage Layout

## Exploit Type
StorageLayout

## Location
RiskEngine.getRiskParameters

## Finding Status: Valid
### Finding Status Justification: The code path is present and will revert for almost all builder codes. `RiskEngine.getRiskParameters` computes `feeRecipient = uint256(uint160(_computeBuilderWallet(builderCode))).toUint128();`. `Math.toUint128` reverts if the value does not fit in 128 bits. Since EVM addresses are 160 bits, a random address will exceed `type(uint128).max` with probability 1 - 2^-32 (~99.99999998%). Therefore, any call to `PanopticPool.dispatch` with nonzero `builderCode` will almost always revert in `getRiskParameters`, breaking the builder/referral feature. There is no alternate path or truncation; this is a hard DoS for builder-code usage, independent of token behavior and without requiring special on-chain state.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `RiskParameters` struct packs the `feeRecipient` address into 128 bits to fit within a single 256-bit storage slot. However, Ethereum addresses are 160 bits (20 bytes). In `RiskEngine.getRiskParameters`, the `feeRecipient` address (derived from `_computeBuilderWallet`) is downcasted to `uint128` using `toUint128()`. Since `toUint128()` reverts on overflow and the vast majority of addresses are greater than `2^128 - 1`, this function will revert for almost all valid builder wallets. As `getRiskParameters` is called by `PanopticPool.dispatch` (via `getRiskParameters`), any user interaction that provides a non-zero `builderCode` will likely revert, rendering the builder referral feature unusable and causing a Denial of Service for those transactions.

## Impact
The builder referral system is functionally broken. Any attempt to use a builder code will cause transactions to revert, preventing users from minting options with referrals and preventing builders from earning fees.

## Command to Run Test


## Proof of Concept
1. Deploy the protocol.
2. User calls `PanopticPool.dispatch` with a valid `builderCode` (non-zero).
3. `PanopticPool` calls `RiskEngine.getRiskParameters(..., builderCode)`.
4. `RiskEngine` calculates `feeRecipient` address using `_computeBuilderWallet`.
5. The address is cast to `uint128`. If the address > `type(uint128).max` (probability ~100%), the transaction reverts with `CastingError`.
6. The user's transaction fails.

## Proof of Code
function test_RiskParametersRevertWithBuilder() public {
    // Mock address > uint128.max
    address mockBuilder = address(0x100000000000000000000000000000001);
    // This mimics the logic in RiskEngine.getRiskParameters
    vm.expectRevert(); // Expect revert due to overflow
    uint128 recipient = uint128(uint160(mockBuilder));
    // Using the SafeCast library or equivalent check present in PanopticMath
    if (uint160(mockBuilder) > type(uint128).max) revert Errors.CastingError();
}

## Suggested Mitigation
Increase the size allocated for `feeRecipient` in the `RiskParameters` struct to 160 bits. Since `RiskParameters` is already tightly packed (256 bits), this will require either using a second storage slot (e.g., `RiskParameters2`) or reducing the precision/size of other parameters if possible (though unlikely given the current packing density). Alternatively, store the `feeRecipient` address separately or return it as a separate return value from `RiskEngine`.


## [H-5]. Protocol insolvency via phantom share transfer in forceExercise

## id: Fzr9atqI8Kp63kqk0L45y

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
CollateralTracker.refund

## Finding Status: Valid
### Finding Status Justification: No downgrade reasons were provided. The sequence delegate(payor) -> refund(payor, exercisor, +assets) via internal _transferFrom -> revoke(payor) can inflate _internalSupply and make transferred phantom shares redeemable, i.e., a plausible direct loss vector if a positive refund is attainable while the payor lacks sufficient real shares.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `delegate` mechanism in `CollateralTracker` mints phantom (virtual) shares to an account to allow operations like burning options even if the user is transiently insolvent. The `revoke` mechanism assumes any phantom shares consumed were burned (destroying supply). However, in `PanopticPool._forceExercise`, the `refund` function is called between `delegate` and `revoke`. `refund` uses `transferFrom`. If `riskEngine.getRefundAmounts` determines that the liquidatee (`account`) owes funds to the exercisor (`msg.sender`)—which can happen if `exerciseFees` becomes positive due to oracle/spot divergence or cross-collateralization logic—the `refund` function will transfer shares from `account` to `msg.sender`. Since `account` holds phantom shares, these phantom shares are transferred to `msg.sender`. 

When `revoke(account)` is called, it detects the missing shares from `account` and increases `_internalSupply` to account for the "consumption", effectively verifying the phantom shares as real. `msg.sender` now holds valid shares that are backed by the protocol's assets, despite no assets being deposited for them. This dilutes existing LPs and allows theft of protocol funds.

## Impact
Direct theft of protocol funds by minting unbacked shares via forced exercise mechanism.

## Command to Run Test


## Proof of Concept
1. Attacker creates two accounts: Victim and Attacker.
2. Victim opens a position that becomes long.
3. Attacker waits for a market condition where `exerciseCost` calculation results in a positive fee (Victim pays Attacker) due to slippage `currentValue` vs `oracleValue` (valid within `MAX_TICKS_DELTA`), or uses the cross-collateral swap logic in `getRefundAmounts` to force a payment from Victim.
4. Victim withdraws all collateral (or is insolvent).
5. Attacker calls `forceExercise` on Victim.
6. `ct0.delegate(Victim)` gives Victim phantom shares.
7. `getRefundAmounts` calculates Victim owes Attacker.
8. `ct0.refund(Victim, Attacker, amount)` transfers phantom shares to Attacker.
9. `ct0.revoke(Victim)` sees Victim's balance is 0 (or low), assumes shares were burned, and increases `_internalSupply`.
10. Attacker calls `withdraw` to convert the stolen shares into assets, draining the pool.

## Proof of Code
function testExploitPhantomShares() public {
    // Setup: User A (victim) and User B (attacker)
    // A has a position. A removes all collateral.
    // B calls forceExercise.
    // Mock a scenario where refund moves funds A -> B.
    // Assert B's balance increases and is withdrawable.
}

## Suggested Mitigation
Modify `CollateralTracker.refund` (and any transfer function used during delegation) to disallow transferring phantom shares. Specifically, ensure that `balanceOf[from] - amount >= type(uint248).max` if `from` is currently delegated. Alternatively, revert `forceExercise` if the user cannot pay the fee with real collateral.


## [H-6]. Protocol Loss Exacerbation due to Inverse Haircut Logic in Liquidations

## id: oi23kukBTfjTB5ZgOjSUB

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
InteractionHelper.settleAmounts

## Finding Status: Valid
### Finding Status Justification: If the described sign mismatch holds (positive haircut passed as realizedPremium), it can mint shares to the liquidatee during protocol-loss liquidations whenever haircutTotal != 0, worsening loss and diluting LPs. That is neither low impact nor particularly rare.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
When a liquidation results in protocol loss (collateral < required payout), the `RiskEngine` calculates a 'haircut' to claw back premiums paid to the liquidatee (option seller). The calculated `haircutTotal` represents the amount of premium that should be revoked from the liquidatee to cover the deficit.

However, in `InteractionHelper.settleAmounts`, this `haircutTotal` (a positive value) is passed to `CollateralTracker.settleBurn` as the `realizedPremium` argument. In `CollateralTracker.settleBurn`, the logic `tokenToPay = ... - realizedPremium` is used. A positive `realizedPremium` reduces `tokenToPay` (making it negative if it was 0), which triggers `_mint` instead of `_burn`.

Code Path:
1. `RiskEngine.haircutPremia` calculates `haircutTotal` (positive magnitude of clawback).
2. `InteractionHelper.settleAmounts` calls `ct.settleBurn(..., int128(haircutTotal), ...)`.
3. `CollateralTracker.settleBurn` calculates `tokenToPay = -haircutTotal`.
4. `tokenToPay < 0` triggers `_mint(liquidatee, haircutTotal)`.

Result: Instead of burning shares to recover funds from the insolvent user, the protocol mints *more* shares to them, increasing the protocol loss and diluting other depositors.

## Impact
Protocol loss is amplified during liquidations involving premium haircuts. Insolvent users receive additional tokens instead of having their assets clawed back, leading to direct loss of funds for the protocol and liquidity providers.

## Command to Run Test


## Proof of Concept
1. Create a scenario where a user (Liquidatee) is insolvent and has caused protocol loss, but has received Long Premium.
2. Liquidator calls `liquidate`.
3. `RiskEngine` calculates a `haircutTotal` of 100 tokens.
4. `settleAmounts` is called with `haircutTotal = 100`.
5. `CollateralTracker.settleBurn` is called with `realizedPremium = 100`.
6. `tokenToPay` becomes -100.
7. `CollateralTracker` mints 100 shares to the Liquidatee.

## Proof of Code
    function test_Haircut_Logic_Failure() public {
        // Mock setup where haircut is triggered
        // Assert that Liquidatee balance INCREASES after haircut application
        // instead of DECREASING.
        uint256 preBalance = tracker.balanceOf(liquidatee);
        // Trigger liquidation logic...
        uint256 postBalance = tracker.balanceOf(liquidatee);
        assertGt(postBalance, preBalance, 'Liquidatee received shares instead of burning');
    }

## Suggested Mitigation
In `InteractionHelper.settleAmounts`, pass the negated value of `haircutTotal` to `settleBurn`. Change `int128(haircutTotal.rightSlot())` to `-int128(haircutTotal.rightSlot())`. This ensures `tokenToPay` becomes positive, triggering the `_burn` path in `CollateralTracker`.


## [H-7]. Liquidation with protocol loss mints shares to insolvent liquidatee instead of liquidator

## id: qTHIeshUYkqc5osgehjoR

## Derived From Pattern/Invariant
Access Control and Logic Flaw in Liquidation Settlement

## Exploit Type
AccountingInvariantViolation

## Location
CollateralTracker.settleLiquidation

## Finding Status: Valid
### Finding Status Justification: As written, settleLiquidation(bonus<0) mints shares to the liquidatee even though the liquidator pays underlying, breaking liquidation incentives and potentially worsening protocol outcomes. This is not low impact and is reachable whenever liquidations with bonus<0 occur.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `CollateralTracker.settleLiquidation`, when a liquidation results in protocol loss (`bonus < 0`), the liquidator is required to pay tokens (`bonusAbs`) to the protocol to cover the deficit. The intended behavior (per documentation and economic logic) is to mint shares to the liquidator as compensation for these provided tokens. However, the code incorrectly calls `_mint(liquidatee, convertToShares(bonusAbs))`. 

Furthermore, immediately after minting, the function executes logic to wipe the `liquidatee`'s balance (to handle virtual share revocation). This sequence effectively destroys the newly minted shares by overwriting the `balanceOf[liquidatee]` without decrementing the `_internalSupply` or `totalSupply` for the minted amount. 

As a result:
1. The liquidator pays tokens but receives nothing, guaranteeing a loss.
2. The `totalSupply` increases while the `sum(balances)` does not, permanently breaking the accounting invariant `totalSupply == sum(balances)`.
3. Liquidators are disincentivized from performing liquidations that result in protocol loss, leading to bad debt accumulation.

## Impact
Liquidators lose funds when covering protocol loss, leading to a denial of service for liquidations (rational actors will not liquidate). Additionally, the token's total supply invariant is permanently broken.

## Command to Run Test


## Proof of Concept
1. A user becomes insolvent with a position that causes protocol loss (collateral < required premium/payout).
2. A liquidator calls `PanopticPool.liquidate`.
3. `RiskEngine` calculates a negative bonus (liquidator must pay to cover loss).
4. `PanopticPool` calls `CollateralTracker.settleLiquidation(..., bonus < 0)`.
5. `CollateralTracker` transfers tokens from liquidator to `PanopticPool`.
6. `CollateralTracker` mints shares to `liquidatee` (the insolvent user).
7. `CollateralTracker` overwrites `liquidatee`'s balance to handle virtual share cleanup, effectively deleting the just-minted shares.
8. Liquidator has paid funds and received zero shares.

## Proof of Code
function testLiquidationMintBug() public {
    // Mock setup simulating CollateralTracker context
    // Assume liquidator pays 1000 assets, shares exchange rate 1:1
    int256 bonus = -1000;
    uint256 bonusAbs = 1000;
    
    // Simulate settleLiquidation logic
    uint256 preLiquidatorBalance = balanceOf(liquidator);
    uint256 preLiquidateeBalance = balanceOf(liquidatee);
    uint256 preSupply = totalSupply;
    
    // Implementation logic replication:
    // _mint(liquidatee, bonusAbs);
    balanceOf[liquidatee] += bonusAbs;
    totalSupply += bonusAbs;
    
    // Wipe liquidatee balance logic
    balanceOf[liquidatee] = 0; 
    
    // Assertions
    assertEq(balanceOf(liquidator), preLiquidatorBalance); // Liquidator got nothing
    assertEq(totalSupply, preSupply + bonusAbs); // Supply increased
    // Sum of balances mismatch: totalSupply > sum(balances)
}

## Suggested Mitigation
Change the mint target from `liquidatee` to `liquidator` inside the `if (bonus < 0)` block: `_mint(liquidator, convertToShares(bonusAbs));`.


## [H-8]. Safe Mode Bypass in PanopticPool.validateCollateralWithdrawable Allows Insolvency Escape

## id: yt5I5Sgpb93gdC-62mdDE

## Derived From Pattern/Invariant
AuthByPass

## Exploit Type
AuthByPass

## Location
PanopticPool.validateCollateralWithdrawable

## Finding Status: Valid
### Finding Status Justification: The bug does exist: validateCollateralWithdrawable hardcodes safeMode=0 when calling _validateSolvency, so safe-mode’s stricter collateralization path in _checkSolvencyAtTicks is bypassed for withdrawals-with-positions. There is no compensating safeguard in CollateralTracker.withdraw (it relies on this hook). This is not clearly justified as intended behavior, and it is exploitable by withdrawing collateral during an active safe mode that would otherwise disallow the withdrawal, increasing bad-debt risk.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `validateCollateralWithdrawable` function in `PanopticPool.sol` is used to verify if a user's account remains solvent after a collateral withdrawal. It retrieves the current `riskParameters` using `getRiskParameters(0)`, which includes the current `safeMode` status (triggered by oracle volatility or guardian lock). However, when calling the internal `_validateSolvency` function, it explicitly passes `0` as the `safeMode` argument instead of `riskParameters.safeMode()`. 

In `_validateSolvency`, if `safeMode > 0`, the system forces 100% collateralization (no cross-margining, fully covered positions). By passing `0`, the withdrawal check runs under normal solvency rules even if the protocol is in Safe Mode. This allows users to withdraw collateral and potentially leave their account under-collateralized relative to the required Safe Mode standards during high volatility events, undermining the protocol's safety mechanism.

## Impact
Users can withdraw collateral down to normal margin limits during Safe Mode, bypassing stricter solvency requirements intended to protect the protocol during high volatility or oracle instability. This increases the risk of bad debt.

## Command to Run Test


## Proof of Concept
1. The protocol enters Safe Mode (e.g., due to high oracle deviation or Guardian action). Collateral requirements effectively increase to 100% (fully covered).
2. A user has a portfolio that is solvent under normal rules but insolvent under Safe Mode rules (e.g., relying on cross-collateralization).
3. The user calls `withdraw` on the `CollateralTracker`, which calls `PanopticPool.validateCollateralWithdrawable`.
4. `validateCollateralWithdrawable` calls `_validateSolvency` with hardcoded `safeMode=0`.
5. The solvency check passes using relaxed normal rules.
6. The withdrawal succeeds, leaving the user with less collateral than required by the active Safe Mode, potentially leading to insolvency if prices move further.

## Proof of Code
function testSafeModeBypass() public {
    // Setup: Create a position that is solvent normally but insolvent in Safe Mode
    // ... (Assume setup similar to standard tests)
    
    // 1. Trigger Safe Mode (mocking oracle deviation or guardian lock)
    vm.prank(guardian);
    riskEngine.lockPool(panopticPool);
    assertEq(panopticPool.isSafeMode(), 3);

    // 2. User attempts to withdraw collateral
    // Under Safe Mode, this should fail if they are near the limit, but it succeeds
    uint256 withdrawAmount = 1 ether;
    vm.prank(user);
    // Expectation: Should revert if safeMode was enforced
    collateralTracker0.withdraw(withdrawAmount, user, user);
    // Reality: Succeeds because safeMode is passed as 0
}

## Suggested Mitigation
Pass the actual safe mode status to `_validateSolvency`.

```diff
function validateCollateralWithdrawable(
    address user,
    TokenId[] calldata positionIdList,
    bool usePremiaAsCollateral
) external view {
    (RiskParameters riskParameters, ) = getRiskParameters(0);
    _validateSolvency(
        user,
        positionIdList,
        riskParameters.bpDecreaseBuffer(),
        usePremiaAsCollateral,
-       0
+       riskParameters.safeMode()
    );
}
```


## [H-9]. Stale Oracle Data used in Solvency Checks allows Undercollateralized Positions

## id: Uyizca5RhzgGDGi88uzGc

## Derived From Pattern/Invariant
OracleUsingDEXorTWAP

## Exploit Type
Oracle

## Location
RiskEngine.getSolvencyTicks

## Finding Status: Valid
### Finding Status Justification: This is not merely 'stale EMA over minutes' speculation; it is a same-transaction logic issue where updated oracle data is computed but not used for the solvency check, potentially letting solvency pass at an outdated conversion tick. That is not inherently low impact.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `RiskEngine.getSolvencyTicks` function retrieves oracle ticks from the `_oraclePack` via `OraclePack.getOracleTicks`. However, `OraclePack.getOracleTicks` returns the `spotEMATick` (and others) from the *input* packed data (`self.spotEMA()`) *before* computing any potential updates in `computeInternalMedian`. 

Consequently, even if `computeInternalMedian` determines that an update is necessary due to time elapsed and calculates a new `oraclePack`, the ticks returned to the Risk Engine correspond to the *previous* state (which could be hours old). The `PanopticPool` solvency check (`isAccountSolvent`) relies on these stale ticks. 

While the protocol has a 'Safe Mode' that triggers if the current spot price deviates significantly from the (stale) EMA, Safe Mode only increases collateral ratios to 100%. It does not correct the base price used for currency conversion in cross-collateral checks (e.g. converting Token A collateral value to Token B requirement).

## Impact
Users can exploit stale prices to open undercollateralized positions, particularly involving cross-collateral (e.g., using Token A collateral for Token B requirement) when the real exchange rate differs from the stale oracle rate. This leads to immediate bad debt and protocol loss.

## Command to Run Test


## Proof of Concept
1. Assume ETH/USDC pool. Oracle Price = $2000 (updated 2 hours ago). Real Price = $4000.
2. User deposits 2000 USDC. Stale Oracle values this as 1 ETH buying power.
3. User calls `mintTokenizedPosition` to write a Short Call on ETH (Liability ~1 ETH).
4. `PanopticPool` calls `_validateSolvency` -> `RiskEngine.getSolvencyTicks`.
5. `getSolvencyTicks` returns the stale $2000 tick.
6. `RiskEngine.isSafeMode` detects deviation ($2000 vs $4000) and enables Safe Mode (100% collateral ratio).
7. Solvency check runs at $2000 tick:
   - Collateral: 2000 USDC -> converts to 1 ETH at $2000/ETH.
   - Requirement: 1 ETH (100% of notional).
   - 1 ETH >= 1 ETH. Solvency Check PASSES.
8. In reality, at $4000/ETH:
   - Collateral: 2000 USDC -> 0.5 ETH.
   - Liability: 1 ETH.
   - User is insolvent by 0.5 ETH.
9. User has successfully created a bad debt position.

## Proof of Code
function testStaleOracleSolvency() public {
    // Setup pool with ETH/USDC, init price 2000
    // Advance time 2 hours
    // Move pool price to 4000 (don't poke oracle)
    
    // User deposits 2000 USDC
    // User mints Short Call ETH
    // Assert Mint success (should fail if oracle was fresh)
    // Assert User is instantly liquidatable/insolvent
}

## Suggested Mitigation
In `OraclePack.sol`, update `getOracleTicks` to return the updated EMAs if an update occurs. Alternatively, ensure `spotEMATick` is derived from the `_updatedOraclePack` if it is not null, otherwise from `self`.


## [M-10]. Force Exercise Fee Avoidance via Spot Price Manipulation

## id: Hs4D2HoPX48S4VK1Xp7Ka

## Derived From Pattern/Invariant
FlashLoanEconomicManipulation

## Exploit Type
FlashLoanEconomicManipulation

## Location
RiskEngine.sol.exerciseCost

## Finding Status: Valid
### Finding Status Justification: A boundary flip (in-range vs out-of-range) changes the base fee from ~1% to 1bp, which is material. The allowed TWAP deviation still permits borderline manipulation, so it is not inherently low likelihood.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `RiskEngine.exerciseCost` function calculates the fee required to force-exercise a position. A critical component of this calculation is determining whether the position is 'in-range' or 'out-of-range'. If the position has any legs in-range (`hasLegsInRange = true`), the base fee is set to `FORCE_EXERCISE_COST` (~1% of notional). If strictly out-of-range, the fee is `ONE_BPS` (0.01%). 

The vulnerability is that `hasLegsInRange` is determined solely using `currentTick` (the instantaneous spot price). While `PanopticPool` enforces that `currentTick` is within a ~5% deviation of the `twapTick` (oracle price) via `MAX_TWAP_DELTA_LIQUIDATION`, an attacker can still manipulate the `currentTick` within this allowed range to push a borderline ITM position into OTM territory. By doing so, the attacker (seller) can force-exercise the position paying only 1 basis point instead of the intended 1% surcharge, effectively stealing the fee revenue due to the option buyer.

## Impact
Option sellers can avoid paying the 1% force-exercise fee on positions that are validly in-range according to the oracle but manipulable to be out-of-range in the spot market. This results in a loss of yield/fees for option buyers.

## Command to Run Test


## Proof of Concept
1. Alice (seller) has a short put position with strike K. The oracle price (TWAP) is slightly below K (ITM), and the spot price is also below K.
2. Alice wants to force-exercise the position to free up liquidity but wants to avoid the 1% `FORCE_EXERCISE_COST`.
3. Alice performs a swap in the underlying Uniswap pool to push the `currentTick` above K + width (OTM), but keeps the deviation within the `MAX_TWAP_DELTA_LIQUIDATION` (513 ticks) limit relative to TWAP.
4. Alice calls `dispatch` -> `forceExercise`.
5. `RiskEngine.exerciseCost` uses the manipulated `currentTick` to check `hasLegsInRange`. It evaluates to `false`.
6. The fee is calculated as `ONE_BPS` instead of `FORCE_EXERCISE_COST`.
7. Alice burns the position cheaply and arbitrages the price back, saving ~1% of the position notional value.

## Proof of Code
function test_ForceExerciseFeeAvoidance() public {
    // Setup: User mints a short put close to ATM
    uint256 positionSize = 10 ether;
    int24 width = 10;
    int24 strike = currentTick - 5; // Slightly ITM
    TokenId tokenId = TokenIdLibrary.addLeg(0, 0, 1, 1, 0, 0, 0, strike, width);
    
    // Mint position...
    
    // Check fee before manipulation (should be ~1%)
    int256 feeBefore = riskEngine.exerciseCost(currentTick, currentTick, tokenId, balance).rightSlot();
    
    // Manipulate spot price to be OTM (e.g. strike + width + 1) but within TWAP tolerance
    int24 manipulatedTick = strike + width + 1;
    // Ensure abs(manipulated - oracle) < 513
    
    // Check fee after manipulation (should be ~0.01%)
    int256 feeAfter = riskEngine.exerciseCost(manipulatedTick, currentTick, tokenId, balance).rightSlot();
    
    assertGt(abs(feeBefore), abs(feeAfter) * 50); // Fee reduced by huge factor
}

## Suggested Mitigation
Modify `RiskEngine.exerciseCost` to use the `oracleTick` (or a check against both `oracleTick` and `currentTick`) when determining `hasLegsInRange`. This ensures that fees are based on the robust market price rather than the manipulable spot price.


## [M-11]. Premium accounting corruption due to `s_accountLiquidity` key collision across `vegoid`s

## id: 6BgatfKKgdrrhCQRoKRlI

## Derived From Pattern/Invariant
Storage Collision

## Exploit Type
StorageLayout

## Location
SemiFungiblePositionManager.s_accountLiquidity

## Finding Status: Valid
### Finding Status Justification: Code path exists: `_createLegInAMM` derives `positionKey = keccak256(abi.encodePacked(key.toId(), account, tokenType, tickLower, tickUpper))` and uses it for `s_accountLiquidity`, `s_accountPremiumOwed`, and `s_accountPremiumGross`. `initializeAMMPool` clearly supports multiple SFPM pool configs per same Uniswap V4 pool via `s_V4toSFPMIdData[idV4][vegoid]`, and `tokenId.vegoid()` is used elsewhere, but `vegoid` is omitted from the `positionKey`. Therefore positions across different `vegoid`s but same pool/ticks collide in the same storage slot, while `_updateStoredPremia/_getPremiaDeltas` apply the `vegoid` passed in the current transaction, mixing state across configs. No guard prevents interacting with multiple `vegoid`s for the same underlying pool. This is exploitable by intentionally opening/touching positions across vegoids and results in persistent mis-accounting of owed/gross premia for that chunk. Impact is medium (accounting corruption/incorrect premium transfers), with realistic conditions if multiple vegoids are used.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `SemiFungiblePositionManager` tracks user liquidity and premium accumulators in `s_accountLiquidity`, `s_accountPremiumOwed`, and `s_accountPremiumGross` mappings. These mappings are keyed by `positionKey`, which is derived from `key.toId()`, `account`, `tokenType`, `tickLower`, and `tickUpper`. However, `initializeAMMPool` allows creating multiple Panoptic pools on the same Uniswap V4 pool (`key.toId()`) distinguished by different `vegoid`s. Since `vegoid` is NOT included in the `positionKey`, multiple pools with different `vegoid`s (and thus different premium spread parameters) share the same storage slots for a given user. If a user interacts with multiple `vegoid`s for the same underlying pool, the premium accumulators will be corrupted as they are updated using spread factors from the current transaction's `vegoid` mixed with state from others.

## Impact
Permanent corruption of premium accounting for users or protocols utilizing multiple `vegoid` configurations for the same underlying pool, leading to incorrect owed/gross premium calculations.

## Command to Run Test


## Proof of Concept
1. User initializes a pool with `vegoid=1`. 
2. User initializes the same Uniswap pool with `vegoid=2`.
3. User mints a Short position in `vegoid=1`. `s_accountLiquidity` and premia accumulators are updated.
4. User mints a Short position in `vegoid=2` (same ticks).
5. The contract derives the same `positionKey` and updates the SAME accumulator slots, but uses the spread logic for `vegoid=2`.
6. The resulting premium accounting is a meaningless mix of both spread configurations.

## Proof of Code
function testVegoidCollision() public { ... }

## Suggested Mitigation
Include `vegoid` in the `positionKey` generation within `EfficientHash.efficientKeccak256` or the `_createLegInAMM` function logic.


## [H-12]. Donation Attack via Liquidity Manipulation in SFPM Amplifies Premium Debt

## id: ZMyQQW3XBi-xMdryMKiJu

## Derived From Pattern/Invariant
FlashLoanEconomicManipulation

## Exploit Type
FlashLoanEconomicManipulation

## Location
SemiFungiblePositionManager._updateStoredPremia

## Finding Status: Valid
### Finding Status Justification: SFPM treats `feesAccrued` from Uniswap V4 `modifyLiquidity` as the collected premium base (`collectedSingleLeg = feesAccrued`). Uniswap V4 explicitly warns `feesAccrued` can be artificially inflated (e.g., via donations). Because `_getPremiaDeltas` amplifies `collected` by `totalLiquidity / netLiquidity^2` (and then applies additional spread terms), an attacker who can drive `netLiquidity` very low can make a donation create disproportionately large premia deltas. This can translate into large premium credits to shorts and large debts to longs beyond the donated amount, effectively extracting value from long holders’ collateral (or forcing liquidations/protocol loss). There is no cap distinguishing trading fees from donations, and no minimum net-liquidity floor to prevent amplification.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `SemiFungiblePositionManager` (SFPM) calculates option premium updates based on the fees collected from Uniswap V4. The formula used in `_updateStoredPremia` is derived as `collectedAmount * totalLiquidity / netLiquidity^2`. In the context of the Panoptic protocol, `netLiquidity` represents the liquidity actually present in the AMM, while `totalLiquidity` includes both present and removed (long) liquidity.

An attacker can exploit this by manipulating `netLiquidity` to be extremely small (e.g., 1 wei) by minting Long positions until the pool is nearly fully utilized. Then, the attacker (or an accomplice) performs a `donate` call to the Uniswap V4 pool. Uniswap V4 includes donations in the `feesAccrued` returned by `modifyLiquidity`. 

When `modifyLiquidity` is triggered, the `collectedAmount` (which includes the donation) is multiplied by `totalLiquidity / netLiquidity^2`. Since `netLiquidity` is tiny, this multiplier is massive. This results in an exorbitant increase in `s_accountPremiumOwed` (debt for Long holders) and `s_accountPremiumGross` (credit for Short holders).

If the attacker holds a Short position (or `PanopticPool` shares), they receive a massive credit. Users holding Long positions (who share the same `positionKey` under the `PanopticPool` account) incur a massive debt, effectively draining their collateral or causing protocol insolvency.

## Impact
Theft of user funds (Long holders) or creation of massive protocol bad debt, leading to insolvency.

## Command to Run Test


## Proof of Concept
1. Attacker observes a chunk where `totalLiquidity` is substantial.
2. Attacker mints enough Long positions (removes liquidity) to reduce `netLiquidity` to 1 wei.
3. Attacker mints a Short position (adds liquidity) or ensures they hold shares in the `PanopticPool`.
4. Attacker calls `POOL_MANAGER_V4.donate(...)` with a significant amount (e.g., 1 ETH).
5. Attacker calls `mintTokenizedPosition` with a minimal amount to trigger `_createLegInAMM` -> `modifyLiquidity`.
6. SFPM calculates premium delta: `1 ETH * T / 1^2`. The `s_accountPremiumGross` accumulator jumps by `T * 1 ETH`.
7. Attacker's Short position is credited with `T * 1 ETH` per unit (roughly). Attacker exits with massive profit drawn from Long holders' debt.

## Proof of Code
contract ExploitTest is Test {
    SemiFungiblePositionManager sfpm;
    IPoolManager poolManager;
    PoolKey key;
    
    function testDonationAttack() public {
        // Setup SFPM and Pool
        // 1. Mint Short Position (Add Liquidity T=1000)
        // 2. Mint Long Position (Remove Liquidity R=999) -> Net N=1
        // 3. Donate 1 ETH to V4 Pool
        // 4. Touch SFPM (e.g. mint small)
        // 5. Check s_accountPremiumGross accumulator
        // Assert accumulator has increased by ~ 1 ETH * 1000
    }
}

## Suggested Mitigation
Enforce a minimum `netLiquidity` buffer in SFPM (similar to Uniswap V2's minimum liquidity lock) to prevent `N` from becoming small enough to cause overflow/amplification. Alternatively, track `feeGrowthGlobal` snapshots to distinguish trading fees from donations or cap the maximum premium update per tick.


## [H-13]. Loss of Protocol Fees due to Incorrect Capping Logic in LeftRightLibrary

## id: n-_xgKymo2vqxdx8OCqAK

## Derived From Pattern/Invariant
IntegerOverflow

## Exploit Type
IntegerOverflow

## Location
SemiFungiblePositionManager._updateStoredPremia

## Finding Status: Valid
### Finding Status Justification: `LeftRightLibrary.addCapped` is documented/used as if it “caps” accumulator growth, but when a side hits the cap it returns the *previous* value (`x.rightSlot()` / `y.rightSlot()`) rather than returning `type(uint128).max`. This creates a freeze-at-old-value behavior, discarding the overflowing update instead of saturating. That means sufficiently large premia updates are not recorded at all, and repeated large updates can prevent any further accumulation, leading to systematic under-accounting of fees/premia. This is exploitable in conjunction with small-net-liquidity amplification: an attacker can intentionally cause the first large update to overflow, freezing the accumulator near zero and reducing future premium obligations.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `LeftRightLibrary.addCapped` function is used to update the premium accumulators (`s_accountPremiumOwed` and `s_accountPremiumGross`) while preventing overflows. The logic intends to 'cap' the accumulators at `type(uint128).max` if an addition would cause an overflow, and if one token accumulator overflows, it stops updating both to maintain synchronization. However, the implementation has a logic error: when an overflow is detected (`r_Enabled` or `l_Enabled` becomes false), the function returns the *original* value (`x.rightSlot()`) instead of the capped value (`type(uint128).max`). 

This means that when a fee accrual event is large enough to trigger an overflow (which is exacerbated by the `2^64` scaling factor and the `TotalLiquidity / NetLiquidity` multiplier in `_getPremiaDeltas`), the update is effectively discarded. The accumulator freezes at the previous value, failing to account for the accrued fees. A user can exploit this by creating a highly leveraged position (high `Total/Net` liquidity ratio) that triggers an immediate overflow on the first fee update, effectively freezing their premium obligation at zero or a very low value.

## Impact
High loss of yield for the protocol and liquidity providers. Attackers can hold large, leveraged positions without paying the required premiums/fees.

## Command to Run Test


## Proof of Concept
1. Attacker calls `mintTokenizedPosition` to mint a large Short position (adds liquidity `T`, e.g., 1000 units). Net Liquidity `N=1000`.
2. Attacker calls `mintTokenizedPosition` to mint a large Long position (removes liquidity `R`, e.g., 999 units) in the same chunk. Net Liquidity `N` becomes 1. Total Liquidity `Total` remains ~2000.
3. The multiplier `Total/N` becomes ~2000.
4. Time passes and fees accumulate in the Uniswap pool.
5. Attacker triggers a premium update (e.g., via a dust mint or burn). `_getPremiaDeltas` calculates `deltaPremiumOwed`. Due to the `2^64` scaling and the large `Total/N` multiplier, `deltaPremiumOwed` exceeds `type(uint128).max` (or causes `current + delta` to overflow).
6. `_updateStoredPremia` calls `LeftRightLibrary.addCapped`.
7. Inside `addCapped`, `z_xR` is calculated as `type(uint128).max` (capped). `r_Enabled` becomes `false`.
8. `addCapped` returns `x.rightSlot()` (the old value) instead of `z_xR`.
9. `s_accountPremiumOwed` is not updated.
10. Attacker burns their positions. `getAccountPremium` returns the stale/frozen accumulator value.
11. The calculated premium owed `(current - last) * size` is zero (or negligible). The attacker pays no fees.

## Proof of Code
import "forge-std/Test.sol";
import {LeftRightLibrary, LeftRightUnsigned} from "contracts/types/LeftRight.sol";

contract AddCappedTest is Test {
    using LeftRightLibrary for LeftRightUnsigned;

    function testAddCappedFreezesOnOverflow() public {
        // Simulate an accumulator near max
        uint128 currentVal = type(uint128).max - 100;
        LeftRightUnsigned current = LeftRightUnsigned.wrap(currentVal); // Right slot has val, Left is 0
        
        // Simulate a large delta that causes overflow
        // With Panoptic scaling, this is easy to reach with high leverage
        uint128 deltaVal = 200;
        LeftRightUnsigned delta = LeftRightUnsigned.wrap(deltaVal);

        // Perform addCapped
        (LeftRightUnsigned result, ) = LeftRightLibrary.addCapped(current, delta, current, delta);

        // Expected behavior for "Cap": result should be type(uint128).max
        // Actual behavior: result is currentVal (freeze)
        uint128 resultRight = result.rightSlot();
        
        console.log("Current:", currentVal);
        console.log("Delta:", deltaVal);
        console.log("Result:", resultRight);
        console.log("Max:", type(uint128).max);

        // Assert that it incorrectly returns the old value instead of capping
        assertEq(resultRight, currentVal, "Accumulator failed to freeze at old value (Exploit failed)");
        assertTrue(resultRight < type(uint128).max, "Accumulator was capped correctly (Exploit fixed)");
    }
}

## Suggested Mitigation
Modify `LeftRightLibrary.addCapped` to return the capped value (`z_xR` / `z_xL`) when the enabled flag is false, or explicitly handle the overflow state to ensure fees are not lost. 

```solidity
return (
    LeftRightUnsigned.wrap(r_Enabled ? z_xR : type(uint128).max)
        .addToLeftSlot(l_Enabled ? z_xL : type(uint128).max),
    // ... handle y similarly
);
```


## [H-14]. Interest Debt Wipe via Force Exercise Delegation Mechanism

## id: FIGoD-iLIgmR6ei1e15Ky

## Derived From Pattern/Invariant
IncentiveMisalignmentOrGameTheory

## Exploit Type
AccountingInvariantViolation

## Location
CollateralTracker.delegate

## Finding Status: Valid
### Finding Status Justification: This falls into the (explicitly out-of-scope) delegate/revoke accounting family (notably 'Orphan Shares in Delegate/Revoke' and related interest/index-update quirks). If it occurs, it is not low impact because it can socialize unpaid interest onto LPs.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `CollateralTracker` allows a mechanism where `PanopticPool` delegates `type(uint248).max` virtual shares to a user during force exercises and liquidations to ensure settlement mechanics succeed. However, this interaction creates a vulnerability when combined with `_accrueInterest`. 

When `delegate` is called, the user's balance is artificially inflated to `~MAX`. If the user has accrued interest, `_accrueInterest` is triggered (via `settleBurn`). Because the user's balance appears extremely large, the interest accrual logic treats the user as solvent (`shares > userBalance` is false), burns the interest amount from the virtual balance, and **updates the user's borrow index**, effectively marking the debt as paid. 

Subsequently, `revoke` is called to remove the virtual shares. The logic in `revoke` restores `_internalSupply` by adding back the amount of shares that were burned (`MAX - balance`), neutralizing the supply impact. 

The net result is that the user's interest debt is wiped out (index updated) without any real assets being deducted from their account (as only virtual shares were burned and then the supply impact was reversed). An attacker can exploit this by force-exercising a small, out-of-the-money position on their own account to erase significant accrued interest obligations.

## Impact
Protocol insolvency and theft of yield. Users can erase their interest debts without paying, causing loss of funds for Panoptic Liquidity Providers.

## Command to Run Test


## Proof of Concept
1. Attacker opens a position and accumulates significant interest debt over time.
2. Attacker ensures they hold a force-exercisable position (e.g., a small OTM long leg).
3. Attacker calls `PanopticPool.forceExercise` (via `dispatch`) on their own position using a second account.
4. `PanopticPool` calls `CollateralTracker.delegate(attacker)`, inflating attacker's balance to `MAX`.
5. `PanopticPool` calls `_burnOptions`, which calls `CollateralTracker.settleBurn`, triggering `_accrueInterest`.
6. `_accrueInterest` sees the inflated balance, burns the interest amount from the virtual shares, and updates the attacker's `userBorrowIndex` to the current index (erasing the debt).
7. `PanopticPool` calls `CollateralTracker.revoke(attacker)`. The logic detects phantom share consumption and restores `_internalSupply`.
8. Attacker's real collateral remains untouched, but their interest debt is wiped.

## Proof of Code
function testExploitInterestWipe() public {
    // Setup: User borrows and accrues interest
    vm.startPrank(user);
    // ... mint position ...
    vm.warp(block.timestamp + 365 days); // Large interest accrues
    uint256 interestOwed = collateralTracker.owedInterest(user);
    assertGt(interestOwed, 0);
    
    // Exploit: Force Exercise self
    // Create a scenario where force exercise is valid (e.g. price move)
    vm.stopPrank();
    vm.prank(liquidator);
    panopticPool.forceExercise(user, tokenId, ...);
    
    // Check: Interest wiped but not paid
    (uint256 assets, uint256 newInterest) = collateralTracker.assetsAndInterest(user);
    assertEq(newInterest, 0, "Interest should be wiped");
    // Check that user didn't pay real assets (balance should be ~same minus exercise fee)
}

## Suggested Mitigation
In `CollateralTracker.delegate`, record the user's pre-delegation balance and interest state. In `revoke`, or by using a specialized modifier/flag during delegation, ensure that `_accrueInterest` does not update the `userBorrowIndex` if the payment was made using delegated virtual shares. Alternatively, restrict `forceExercise` to only operate if the user has sufficient *real* collateral to cover interest, or ensure the interest deducted from virtual shares is realized as a debt against the user's real collateral.


## [H-15]. Protocol Insolvency via Invalid Risk Partner Chaining due to Missing TokenId Validation

## id: YN17tk9KVD12bU3htte6o

## Derived From Pattern/Invariant
AccessControlOrAuthByPass

## Exploit Type
AuthByPass

## Location
PanopticPool.dispatch

## Finding Status: Valid
### Finding Status Justification: TokenIdLibrary.validate() enforces mutual riskPartner relationships, but PanopticPool.dispatch’s mint path only checks poolId and never calls TokenId.validate before using RiskEngine collateral logic. RiskEngine._getRequiredCollateralSingleLegPartner relies on `index < partnerIndex ? ... : 0` to avoid double-counting spreads, which can be abused if riskPartner links are non-mutual (e.g., chains/cycles). Without validation, an attacker can craft a TokenId that causes RiskEngine to treat an actually unhedged exposure as multiple “defined-risk” relationships and understate collateral. This can allow minting undercollateralized positions leading to insolvency and potential protocol loss.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `TokenId` library includes a `validate` function intended to ensure that option positions are well-formed, specifically that risk partners are mutual (if leg A partners with leg B, leg B must partner with leg A). However, this `validate` function is never called within `PanopticPool.sol` or the `SFPM`. 

Because the `RiskEngine` calculates collateral requirements by iterating through legs and assuming standard partner configurations, a malicious user can construct a `TokenId` with a circular or chained risk partner configuration (e.g., Leg 0 -> Leg 1 -> Leg 2 -> Leg 0). 

The `_getRequiredCollateralSingleLegPartner` function in `RiskEngine.sol` contains logic to deduplicate spread calculations: `return index < partnerIndex ? _computeSpread(...) : 0;`. 

In a chain like 0->1->2->0:
- Leg 0 (partners with 1): `0 < 1`, computes Spread(0,1).
- Leg 1 (partners with 2): `1 < 2`, computes Spread(1,2).
- Leg 2 (partners with 0): `2 > 0`, returns 0.

By carefully selecting the legs (e.g., Leg 0 = Short Call, Leg 1 = Long Call, Leg 2 = Short Call, all at same strike), the user can trick the system into calculating the collateral as two fully hedged spreads (Spread(0,1) ~ 0 risk, Spread(1,2) ~ 0 risk), while the actual position contains a naked short call (Leg 2) which has infinite risk. This allows minting naked positions with near-zero collateral.

## Impact
A malicious user can mint highly risky, under-collateralized positions (e.g., naked shorts with zero collateral). If the market moves against these positions, the user goes insolvent immediately, leaving the protocol with bad debt and causing loss of funds for PLPs.

## Command to Run Test


## Proof of Concept
1. Attacker constructs a `TokenId` manually with 3 active legs:
   - Leg 0: Short Call at strike K. `riskPartner` index set to 1.
   - Leg 1: Long Call at strike K. `riskPartner` index set to 2.
   - Leg 2: Short Call at strike K. `riskPartner` index set to 0.
2. Attacker calls `PanopticPool.dispatch` to mint this position.
3. `_validateSolvency` calls `RiskEngine.getMargin` -> `_getTotalRequiredCollateral`.
4. `RiskEngine` iterates legs:
   - Leg 0: Calculates spread collateral with Leg 1 (Short K + Long K). Result is ~0.
   - Leg 1: Calculates spread collateral with Leg 2 (Long K + Short K). Result is ~0.
   - Leg 2: Since `index (2) > partner (0)`, returns 0.
5. Total collateral requirement is ~0.
6. The actual position is Net Short 1 Call (Short + Long + Short). This requires significant collateral.
7. Attacker successfully mints a naked short position for free.

## Proof of Code
/* 
In a Foundry test file (e.g. RiskPartnerExploit.t.sol): 
*/

function testRiskPartnerExploit() public {
    // Setup: Assume standard deployment
    // Construct malicious TokenId
    TokenId maliciousTokenId = TokenId.wrap(0);
    maliciousTokenId = maliciousTokenId.addPoolId(poolId);
    // Add legs to create chain 0->1->2->0
    // Leg 0: Short Call, Ratio 1, Partner 1
    maliciousTokenId = maliciousTokenId.addLeg(0, 1, 0, 0, 0, 1, strike, width);
    // Leg 1: Long Call, Ratio 1, Partner 2
    maliciousTokenId = maliciousTokenId.addLeg(1, 1, 0, 1, 0, 2, strike, width);
    // Leg 2: Short Call, Ratio 1, Partner 0
    maliciousTokenId = maliciousTokenId.addLeg(2, 1, 0, 0, 0, 0, strike, width);
    
    // Verify Validation would fail
    vm.expectRevert();
    TokenIdLibrary.validate(maliciousTokenId);
    
    // Minting succeeds (simulated call to RiskEngine solvency check)
    bool isSolvent = riskEngine.isAccountSolvent(..., [maliciousTokenId], ...);
    assertTrue(isSolvent, "Account should be solvent with 0 collateral despite having naked short");
}

## Suggested Mitigation
Call `TokenId.validate(tokenId)` inside `PanopticPool._validatePositionList` or at the beginning of `dispatch` loop to ensure all minted positions conform to the expected structure (valid ratios, no gaps, mutual risk partners).


## [M-16]. DoS on partial position closing during Safe Mode due to strict solvency checks

## id: -3nRIujFbTLojMcjByT9-

## Derived From Pattern/Invariant
EmergencyModeStateStuck

## Exploit Type
EmergencyModeStateStuck

## Location
PanopticPool.dispatch

## Finding Status: Valid
### Finding Status Justification: In PanopticPool.dispatch, after executing burns, it always calls _validateSolvency(user, finalPositionIdList, ..., safeMode). In _checkSolvencyAtTicks, if safeMode>0 it overrides the positionBalanceArray utilizations to max (effectively enforcing fully covered/most conservative requirements). If safe mode activates while a user is leveraged, they can become insolvent under the stricter rules; any attempt to partially burn that improves risk but does not reach full solvency will still revert at the end-of-tx solvency check, preventing incremental deleveraging. This is a practical DoS/“stuck positions” condition during volatility and can force liquidation or require all-at-once unwinds.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
When the protocol enters Safe Mode (e.g., due to oracle volatility), `PanopticPool` enforces 100% collateralization (no leverage) for all positions via `riskEngine.getRiskParameters` and `_validateSolvency`. `PanopticPool.dispatch` calls `_validateSolvency` at the end of every interaction, requiring the user to be solvent *after* the transaction.

If a user is leveraged (e.g., 5x) when Safe Mode activates, they become immediately insolvent. To de-risk, they attempt to burn (close) some positions. However, unless they close *enough* positions to reach 1:1 collateralization (or close *all* positions), `_validateSolvency` will revert because they remain insolvent by the strict Safe Mode standard. This creates an 'all-or-nothing' exit requirement. Users who cannot afford the gas or liquidity to close their entire portfolio in a single transaction are trapped and cannot de-risk, eventually leading to liquidation.

## Impact
Leveraged users are prevented from partially unwinding positions during market turbulence (Safe Mode), forcing them to be liquidated or requiring massive capital/gas to exit in one go.

## Command to Run Test


## Proof of Concept
1. User mints positions with 5x leverage.
2. Market volatility triggers Safe Mode (oracle deviation).
3. Collateral requirement jumps to 100%.
4. User tries to burn 50% of their positions to reduce risk.
5. `dispatch` executes the burn.
6. `dispatch` calls `_validateSolvency`.
7. User is still 5x leveraged on remaining positions (insolvent under 100% rule).
8. Transaction reverts.
9. User cannot exit.

## Proof of Code
function testSafeModeLock() public {
    // Enter safe mode via mock or oracle manip
    // Try to burn 1 leg of a multi-leg portfolio
    // Assert revert AccountInsolvent
}

## Suggested Mitigation
Modify `_validateSolvency` or `dispatch` to allow transactions that *improve* the user's solvency ratio (reduce risk) even if they remain insolvent, particularly when in Safe Mode. Alternatively, allow `burn` operations to bypass the strict Safe Mode solvency check if the resulting leverage/risk is lower than before.


## [H-17]. DoS of Liquidation due to `MAX_SPREAD` check in `PanopticPool` bricks bad debt clearance

## id: Hnj5gU5D1dLRiHoa-LZE1

## Derived From Pattern/Invariant
DoS

## Exploit Type
Dos

## Location
PanopticPool._checkLiquiditySpread

## Finding Status: Valid
### Finding Status Justification: PanopticPool._updateSettlementPostBurn enforces _checkLiquiditySpread(..., riskParameters.maxSpread()) during burns of short legs, and liquidation uses the same burn path (_liquidate -> _burnAllOptionsFrom -> _burnOptions -> _updateSettlementPostBurn). If burning an insolvent account’s short liquidity causes netLiquidity to become 0 (Errors.NetLiquidityZero) or removedLiquidity/netLiquidity to exceed MAX_SPREAD, the burn (and thus the entire liquidation) reverts. This can block liquidation/bad-debt clearance for edge cases (small liquidity, high removed liquidity, or positions that dominate netLiquidity). No liquidation-specific bypass exists.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `PanopticPool` contract enforces a `MAX_SPREAD` check (currently 90%) during option burns to prevent manipulating the pool's utilization ratio. This check is performed in `_checkLiquiditySpread` called via `_updateSettlementPostBurn`. However, this check is also enforced during liquidations (`_liquidate` -> `_burnAllOptionsFrom` -> `_burnOptions`).

If a liquidatee holds a large short position such that burning it would reduce the pool's `netLiquidity` significantly (causing the `removedLiquidity / netLiquidity` ratio to exceed 90% or become infinite if net -> 0), the transaction will revert with `Errors.EffectiveLiquidityAboveThreshold` or `Errors.NetLiquidityZero`. 

This creates a scenario where a large insolvent position cannot be liquidated because closing it would leave the pool in an 'invalid' state according to spread limits. Consequently, the protocol is forced to hold bad debt, and the insolvent position continues to pose a risk to Panoptic Liquidity Providers (PLPs) without a mechanism to close it.

## Impact
Insolvent positions cannot be liquidated, leading to accumulation of bad debt and loss of funds for liquidity providers.

## Command to Run Test


## Proof of Concept
1. Setup a Panoptic Pool with low liquidity.
2. Alice (Attacker) mints a large short option position, providing the majority of `netLiquidity` for a specific chunk.
3. Bob buys a large amount of options in that chunk, increasing `removedLiquidity`. The spread `removed / net` is high but valid (e.g., 80%).
4. Alice becomes insolvent due to price movement.
5. Liquidator attempts to call `dispatchFrom` to liquidate Alice.
6. The liquidation process attempts to burn Alice's short position.
7. Burning Alice's position reduces `netLiquidity` significantly (e.g., to near zero), while `removedLiquidity` remains constant (held by Bob).
8. The new spread `removed / (net - aliceLiquidity)` exceeds `MAX_SPREAD` (90%).
9. `_checkLiquiditySpread` reverts.
10. Alice cannot be liquidated.

## Proof of Code
function testLiquidationDoS() public {
    // 1. Setup pool and liquidity
    // 2. Mint Short (Alice)
    // 3. Mint Long (Bob) such that spread is high
    // 4. Move price to make Alice insolvent
    // 5. Prank Liquidator -> call dispatchFrom(liquidate Alice)
    // 6. Assert Revert(EffectiveLiquidityAboveThreshold)
}

## Suggested Mitigation
Modify `_updateSettlementPostBurn` or `_burnOptions` to skip the `_checkLiquiditySpread` validation when the caller is a liquidator (or during any liquidation context). Liquidation should prioritize closing insolvent positions over maintaining pool spread targets.


## [H-18]. TokenId.validate allows active legs to partner with inactive legs, bypassing collateral requirements

## id: 3C6cgxiFK4WsuhU8lcdB1

## Derived From Pattern/Invariant
Input Validation

## Exploit Type
AuthByPass

## Location
TokenId.sol.validate

## Finding Status: Valid
### Finding Status Justification: Code path exists in `TokenIdLibrary.validate`. It enforces mutuality for risk partners but never checks that the partner leg is active (`optionRatio(partner) > 0`). Because `countLegs()` is driven by optionRatio bits, a position can have `countLegs()==1` while `riskPartner(0)==1`. Validation then checks `riskPartner(1)` (all-zero leg => partner=0) and passes mutuality (`== i`). The loop then breaks at `i=1` due to `optionRatio(1)==0`, without rejecting the partner reference. This allows minting tokenIds with “phantom” partners. If RiskEngine’s spread/defined-risk logic assumes partnered legs are real, collateral requirements can be understated, enabling undercollateralized shorts (potential bad debt). There is no mitigation in SFPM (it calls `tokenId.validate()` and accepts it).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `TokenId.validate` function in `TokenId.sol` enforces that risk partners are mutual (if leg A points to leg B, leg B must point to leg A). However, it fails to verify that the partner leg is actually *active* (i.e., has an `optionRatio > 0`). 

In the `validate` loop, iteration stops as soon as an inactive leg is encountered. If Leg 0 is active and designates Leg 1 as its risk partner, and Leg 1 is inactive (all zeros), the check passes because Leg 1's zeroed `riskPartner` bits implicitly point to Leg 0 (index 0). 

This allows an attacker to mint a position where a risky leg (e.g., Short Call) is partnered with a non-existent leg. The RiskEngine, seeing a defined risk partner, may treat the position as a risk-defined spread (e.g., Short Call + Long Call) rather than a naked position. This drastically underestimates the required collateral, allowing the attacker to open highly leveraged positions that can cause bad debt.

## Impact
Undercollateralization leading to protocol insolvency. An attacker can open naked short positions with the collateral requirements of a spread.

## Command to Run Test


## Proof of Concept
1. Attacker constructs a `TokenId` with Leg 0 active (e.g., Short Call at strike K) and Leg 1 inactive (all zeros).
2. Attacker sets Leg 0's `riskPartner` bits to `1`.
3. Leg 1 is all zeros, so its `riskPartner` bits are `0`.
4. Attacker calls `mintTokenizedPosition`.
5. `TokenId.validate` checks Leg 0: `riskPartner(0)` is 1. `riskPartner(1)` is 0. Mutuality check `0 == 0` passes.
6. `TokenId.validate` checks Leg 1: `optionRatio` is 0. Loop breaks.
7. The malformed `TokenId` is minted.
8. RiskEngine calculates collateral assuming Leg 0 is hedged by Leg 1 (which effectively has strike 0/width 0), potentially resulting in significantly lower collateral than required.

## Proof of Code
function test_InvalidRiskPartnerValidation() public {
    // Construct a TokenId with Leg 0 active, Leg 1 inactive
    // Leg 0: ratio=1, asset=0, isLong=0 (short), tokenType=1 (call), partner=1, strike=1000
    TokenId tokenId = TokenIdLibrary.addLeg(TokenId.wrap(0), 0, 1, 0, 0, 1, 1, 1000, 10);
    
    // Leg 1 remains 0 (inactive)
    // Validate should revert but passes
    TokenIdLibrary.validate(tokenId);
}

## Suggested Mitigation
In `TokenId.validate`, when checking `riskPartnerIndex`, ensure that the partner leg is active by verifying `self.optionRatio(riskPartnerIndex) > 0`.


## [M-19]. Accounting Corruption due to Shared Liquidity Storage across Vegoids

## id: 93J1R52mNQpCa-CpND8fj

## Derived From Pattern/Invariant
StorageLayout

## Exploit Type
StorageLayout

## Location
SemiFungiblePositionManager._createLegInAMM

## Finding Status: Valid
### Finding Status Justification: This is the same root cause as the other vegoid-collision report: `s_V4toSFPMIdData` is scoped by `vegoid`, but `positionKey` (and therefore `s_accountLiquidity` and both premium accumulators) omits `vegoid`. Consequently, liquidity/removal state is aggregated across vegoids for the same pool/ticks/tokenType/account, while premium deltas are computed using the `vegoid` provided to `_getPremiaDeltas` on the current call. This mixes fee spread configurations in the same accounting buckets and produces incorrect owed/gross premium values. No existing safeguard separates accounting per vegoid.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `SemiFungiblePositionManager` allows creating multiple internal pools for the same underlying Uniswap V4 pool by using different `vegoid` parameters. The mapping `s_V4toSFPMIdData` correctly scopes configuration data by `vegoid`. However, the `s_accountLiquidity` and premium accumulator mappings use a `positionKey` derived from `EfficientHash.efficientKeccak256(key.toId(), account, ...)` which relies only on the Uniswap V4 Pool ID (`key.toId()`) and user address, omitting the `vegoid`.

This means that if a user opens positions in two different SFPM pools (e.g., `vegoid=1` and `vegoid=2`) that wrap the same V4 pool, their liquidity and fee accumulators will collide and sum up in the same storage slot. Since `_getPremiaDeltas` uses the `vegoid` parameter to calculate spread (`removedLiquidity / vegoid`), mixing liquidity from different vegoid pools results in corrupted premium calculations. For example, liquidity added in `vegoid=1` will affect the math for `vegoid=2` transactions, leading to incorrect fee distribution.

## Impact
Corrupted accounting for users interacting with multiple SFPM pools (vegoids) for the same underlying V4 pool, leading to loss of funds or incorrect premium payments.

## Command to Run Test


## Proof of Concept
1. Initialize SFPM pool A with `vegoid=1` for V4 Pool X.
2. Initialize SFPM pool B with `vegoid=2` for V4 Pool X.
3. User mints 100 liquidity in Pool A. `s_accountLiquidity` for the chunk increases by 100.
4. User mints 100 liquidity in Pool B. `s_accountLiquidity` uses the same key, so it increases to 200.
5. When updating premia for Pool B, `_getPremiaDeltas` uses `vegoid=2` but applies it to the total liquidity of 200 (100 of which should be subject to `vegoid=1` math).
6. The calculated premia delta is incorrect because it applies the wrong spread parameter to half the liquidity.

## Proof of Code
function testVegoidCollision() public {
    // Initialize vegoid=1 and vegoid=2 for same pool
    // Mint in vegoid=1
    // Mint in vegoid=2
    // Assert liquidity in vegoid=2 reflects sum of both
}

## Suggested Mitigation
Include `vegoid` in the `positionKey` derivation in `_createLegInAMM`, or ensure `key.toId()` is combined with `vegoid` when generating the storage key for account liquidity and premium accumulators.


## [H-20]. Accounting collision in `SemiFungiblePositionManager` allows bypassing fee spread tiers via `positionKey` conflict

## id: mgjWTXekEDrYyislebIeb

## Derived From Pattern/Invariant
ExternalProtocolKeyCollision

## Exploit Type
ExternalProtocolKeyCollision

## Location
SemiFungiblePositionManager._createLegInAMM

## Finding Status: Valid
### Finding Status Justification: Because `positionKey` omits `vegoid`, the same accumulator slot is updated regardless of which `vegoid`-scoped pool config is being acted upon. `_updateStoredPremia` uses the `vegoid` argument to compute spread terms in `_getPremiaDeltas` (division by `vegoid` appears in both owed and gross numerators). Therefore, “touching” the shared positionKey via a different vegoid causes premium growth to be computed under the wrong spread tier and impacts the eventual premium settlement for positions that should be under a different tier. This can be used to underpay premiums relative to the intended configuration (fee tier bypass), which is direct economic harm to counterparties/protocol.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `SemiFungiblePositionManager` (SFPM) allows creating multiple 'pools' for the same underlying Uniswap V4 pool, differentiated by a `vegoid` parameter (which determines the spread multiplier `v`). However, the `positionKey` used to track per-user liquidity and accrued premia in `s_accountLiquidity`, `s_accountPremiumOwed`, and `s_accountPremiumGross` is derived only from the Uniswap V4 Pool ID, user address, token type, and ticks. It critically omits the `vegoid`. 

```solidity
// contracts/SemiFungiblePositionManager.sol
function _createLegInAMM(..., uint256 vegoid) internal ... {
    // ...
    bytes32 positionKey = EfficientHash.efficientKeccak256(
        abi.encodePacked(
            key.toId(),
            account,
            tokenId.tokenType(leg),
            liquidityChunk.tickLower(),
            liquidityChunk.tickUpper()
        )
    );
    // ...
    _updateStoredPremia(positionKey, ..., vegoid);
}
```

This causes positions in different SFPM pools (different `vegoid`s) but the same V4 pool to share the same storage slot and accounting state. Since `_updateStoredPremia` calculates premium deltas using the `vegoid` passed in the arguments (from the token initiating the transaction), a user can manipulate the shared accumulator. By interacting with the position using a token from a low-spread pool (high `vegoid`), the shared accumulator updates using the low spread. This allows a user holding a position in a high-spread pool (low `vegoid`) to effectively pay the lower spread, bypassing the protocol's risk/fee tiers.

## Impact
Users can bypass high fee/spread tiers, causing loss of yield for the protocol and liquidity providers, and corrupting accounting invariants across pools.

## Command to Run Test


## Proof of Concept
1. Attacker identifies a target Uniswap V4 pool.
2. Attacker calls `initializeAMMPool` with `vegoid=1` (High Spread Pool A) and `vegoid=255` (Low Spread Pool B).
3. Attacker mints a large Short position in Pool A. This position implies a high owed premium spread.
4. Attacker mints a dust Short position in Pool B. This shares the same `positionKey` as the Pool A position.
5. As trading occurs in the V4 pool, fees accrue.
6. Attacker periodically calls `mint` or `burn` (even 0 amounts if possible, or dust) on the Pool B position.
7. `_createLegInAMM` is called with `vegoid=255`. The shared accumulator `s_accountPremiumOwed[positionKey]` is updated using the spread formula for `vegoid=255` (lower spread).
8. When the attacker eventually closes the Pool A position, the premium owed is calculated based on the accumulated value, which grew at the Pool B (low spread) rate.
9. Attacker successfully avoided paying the higher spread associated with Pool A.

## Proof of Code
import "forge-std/Test.sol";
import {SemiFungiblePositionManager} from "contracts/SemiFungiblePositionManager.sol";
import {TokenId} from "contracts/types/TokenId.sol";
import {PoolKey} from "v4-core/types/PoolKey.sol";
import {Currency} from "v4-core/types/Currency.sol";
import {IPoolManager} from "v4-core/interfaces/IPoolManager.sol";

contract KeyCollisionTest is Test {
    SemiFungiblePositionManager sfpm;
    IPoolManager poolManager;
    PoolKey poolKey;

    function setUp() public {
        // Mock setup or deploy contracts
        // Assume sfpm and poolManager are deployed and linked
    }

    function testKeyCollision() public {
        // 1. Initialize two pools with different vegoids
        uint8 vegoidHigh = 1;
        uint8 vegoidLow = 255;
        
        // 2. Mint position in High Spread pool
        // 3. Mint dust in Low Spread pool
        // 4. Simulate fee accrual in V4
        // 5. Touch Low Spread position
        // 6. Verify High Spread position owed premium is lower than expected
    }
}

## Suggested Mitigation
Include `vegoid` in the `positionKey` derivation within `_createLegInAMM` to ensure state isolation between different SFPM pools wrapping the same V4 pool:

```solidity
bytes32 positionKey = EfficientHash.efficientKeccak256(
    abi.encodePacked(
        key.toId(),
        account,
        tokenId.tokenType(leg),
        liquidityChunk.tickLower(),
        liquidityChunk.tickUpper(),
        vegoid // Add this
    )
);
```


## [H-21]. Yield Theft via Premium Accumulator Overflow from Dust Liquidity Manipulation

## id: ufzMID5PLE4kX5dUVT4z1

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
SemiFungiblePositionManager._getPremiaDeltas

## Finding Status: Valid
### Finding Status Justification: `_getPremiaDeltas` divides by `netLiquidity**2` and multiplies by `totalLiquidity * 2**64`. There is no minimum bound on `netLiquidity` (only `chunkLiquidity != 0`). If utilization is driven so `netLiquidity` becomes very small while `removedLiquidity` remains large, premium-per-liquidity updates can become enormous. Then `toUint128Capped()` produces `type(uint128).max` deltas, and `LeftRightLibrary.addCapped` freezes (returns the prior accumulator value) once an overflow/saturation is detected. Freezing prevents further premium growth, which can allow subsequent long positions (or existing longs after their snapshot) to see reduced/zero owed-premium deltas while sellers stop accruing yield. This is an economic/accounting invariant violation with realistic manipulation paths via long/short balancing to minimize net liquidity.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `SemiFungiblePositionManager` calculates option premia using `s_accountPremiumOwed` and `s_accountPremiumGross` accumulators. The increment for these accumulators is derived in `_getPremiaDeltas` using the formula: `premium_base = collected_fees * total_liquidity * 2^64 / net_liquidity^2`. 

There is no minimum check for `netLiquidity`. An attacker can manipulate `netLiquidity` to be extremely small (e.g., 1 wei) by creating a Short position of size `X` and a Long position of size `X-1`. When `netLiquidity` is 1, the scaling factor `2^64 / 1^2` is enormous (`2^64`). Even a small amount of collected fees (e.g., 1e6 USDC) results in a delta of `~1e6 * 2^64` per liquidity unit. When multiplied by total liquidity, this quickly overflows the `uint128` accumulator limit.

The `LeftRightLibrary.addCapped` function detects this overflow and, instead of wrapping, returns the *original* accumulator value (effectively freezing it). Once frozen, the `owed` premium for Long positions (which is calculated as `(currentAcc - lastAcc) * positionSize`) becomes zero for any subsequent period, regardless of the fees actually collected by the pool. An attacker can thus hold large Long positions for free, stealing yield from PLPs/Sellers.

## Impact
Attackers can hold long option positions without paying premiums, effectively stealing yield from liquidity providers and option sellers.

## Command to Run Test


## Proof of Concept
1. Attacker calls `mintTokenizedPosition` to create a Short position of 2 wei and a Long position of 1 wei in the same chunk. `netLiquidity` = 1 wei.
2. Attacker performs swaps in the underlying Uniswap V4 pool to generate fees.
3. The fees collected trigger `_updateStoredPremia`. With `netLiquidity=1`, the premium accumulator delta is massive (proportional to `fees * 2^64`).
4. Repeat step 2 until the `s_accountPremiumOwed` accumulator hits `type(uint128).max` and freezes (updates are discarded by `addCapped`).
5. Attacker mints a large Long position in the same chunk.
6. Time passes and fees accrue in the pool.
7. Attacker burns the Long position. The premium owed is calculated as `(currentAcc - initialAcc) * size`. Since `currentAcc` is frozen at the same value as `initialAcc` (or close to it), the premium owed is 0.
8. Attacker profited from a free option position.

## Proof of Code
function testPremiaOverflow() public {
    // Assume standard setup
    // Mint Short 2 wei, Long 1 wei -> Net 1 wei
    // Swap to generate fees
    // Check accumulator is capped
    // Mint large Long
    // Swap more
    // Burn Long, check premium owed is 0
}

## Suggested Mitigation
Enforce a minimum `netLiquidity` threshold (e.g., 1e6 wei) in `_getPremiaDeltas` or `_createLegInAMM` to prevent the denominator from being too small, or cap the calculated `premium_base` to a reasonable maximum per update to prevent rapid overflow.


## [H-22]. Premia accumulator freeze in SemiFungiblePositionManager enables free option positions and griefs liquidity providers

## id: UxWEVqTX7kiN6oIESVdQF

## Derived From Pattern/Invariant
IntegerOverflow

## Exploit Type
IntegerOverflow

## Location
SemiFungiblePositionManager._getPremiaDeltas

## Finding Status: Valid
### Finding Status Justification: The freeze behavior is real: `LeftRightLibrary.addCapped` computes capped sums, then if either side hits `type(uint128).max` it disables further accumulation for that token and returns the *old* slot value (freezing). With small `netLiquidity`, `_getPremiaDeltas` can generate capped deltas frequently, causing a permanent stop in accumulator growth for that chunk/token side. Since premia owed/gross are derived from accumulator deltas between snapshots, freezing can make deltas effectively zero over time, which undercharges longs and prevents shorts/PLPs from earning expected premia. No other guard enforces a minimum netLiquidity or caps the per-update premia to avoid hitting the freeze condition.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `SemiFungiblePositionManager.sol`, the `_getPremiaDeltas` function calculates the amount of premium (fees) to be distributed to long (owed) and short (gross) position holders. The calculation relies on `netLiquidity` (the amount of liquidity physically present in the AMM) as a divisor. Specifically, the base premium per unit of liquidity is calculated as `collected * total * 2^64 / net^2`. 

If `netLiquidity` is reduced to a very small non-zero value (e.g., 1 wei) by minting Long positions, the term `net^2` becomes 1, causing the calculated premium delta to be extremely large (proportional to `totalLiquidity * 2^64`). This value easily exceeds `type(uint128).max`.

The `_updateStoredPremia` function uses `LeftRightLibrary.addCapped` to add this delta to the `s_accountPremiumOwed` and `s_accountPremiumGross` accumulators. The `addCapped` function is designed to 'freeze' the accumulator if an overflow occurs (returning the original value instead of the new capped value). 

Once frozen, the accumulators stop increasing. This has two critical consequences:
1. Option buyers (Longs) stop accruing debt (premium owed), effectively enjoying 'free' options.
2. Option sellers (Shorts/LPs) stop earning yield (premium gross), suffering a loss of funds.

## Impact
High. Option sellers lose yield permanently on affected chunks; option buyers can maintain positions without paying premiums, leading to protocol value leakage and unfairness.

## Command to Run Test


## Proof of Concept
1. Attacker identifies a liquidity chunk with significant total liquidity (e.g., 10 ETH) and activity.
2. Attacker mints a large Long position (removes liquidity) in that chunk, sized such that the remaining `netLiquidity` in the AMM is exactly 1 wei.
3. Attacker performs a swap in the Uniswap V4 pool across that tick range to generate a non-zero amount of fees (`collectedAmounts > 0`).
4. Attacker performs any action on that chunk (e.g., mints a tiny position) to trigger `_createLegInAMM` -> `_updateStoredPremia`.
5. `_getPremiaDeltas` calculates a massive delta due to division by `1^2`.
6. `addCapped` detects the overflow and freezes `s_accountPremiumOwed` and `s_accountPremiumGross` at their current values.
7. Future fee collections trigger the same overflow, keeping the accumulators frozen.
8. The Attacker (holding the Long position) no longer accumulates premium debt.

## Proof of Code
import "forge-std/Test.sol";
import {SemiFungiblePositionManager} from "contracts/SemiFungiblePositionManager.sol";
import {IPoolManager} from "v4-core/interfaces/IPoolManager.sol";
import {PoolKey} from "v4-core/types/PoolKey.sol";
import {Currency} from "v4-core/types/Currency.sol";
import {PoolId, PoolIdLibrary} from "v4-core/types/PoolId.sol";
import {TokenId} from "contracts/types/TokenId.sol";
import {LeftRightUnsigned} from "contracts/types/LeftRight.sol";
import {BalanceDelta} from "v4-core/types/BalanceDelta.sol";

contract MockPoolManager {
    function unlock(bytes calldata data) external returns (bytes memory) {
        (bool success, bytes memory returnData) = msg.sender.call(abi.encodeWithSignature("unlockCallback(bytes)", data));
        require(success, "Callback failed");
        return returnData;
    }
    
    // Mock modifyLiquidity to return fees
    function modifyLiquidity(PoolKey memory, IPoolManager.ModifyLiquidityParams memory, bytes calldata) external pure returns (BalanceDelta, BalanceDelta) {
        // Return 0 delta but non-zero fees to trigger updateStoredPremia
        // Using small fee amount (1000) which is enough to overflow when netLiquidity is 1
        return (BalanceDelta.wrap(0), BalanceDelta.wrap(1000)); 
    }
    
    function burn(address, uint256, uint256) external {}
    function mint(address, uint256, uint256) external {}
    function swap(PoolKey memory, IPoolManager.SwapParams memory, bytes calldata) external pure returns (BalanceDelta) { return BalanceDelta.wrap(0); }
    function extsload(bytes32) external pure returns (bytes32) { return bytes32(0); }
}

contract SFPMFreezeTest is Test {
    SemiFungiblePositionManager sfpm;
    MockPoolManager pm;
    
    function setUp() public {
        pm = new MockPoolManager();
        sfpm = new SemiFungiblePositionManager(IPoolManager(address(pm)), 100, 100, 100);
    }
    
    function testAccumulatorFreeze() public {
        PoolKey memory key = PoolKey(Currency.wrap(address(0x1)), Currency.wrap(address(0x2)), 3000, 60,  IHooks(address(0)));
        
        // Initialize pool
        // We need to bypass the check in initializeAMMPool for V4StateReader.getSqrtPriceX96
        // For this PoC unit test, we assume the pool is initialized or mock the call, 
        // but simpler is to test the logic directly or through a harness. 
        // Here we simulate the state that causes the issue: 
        // netLiquidity = 1, totalLiquidity = large, collected = small.
        
        uint128 collected0 = 1000;
        uint256 netLiquidity = 1;
        uint256 removedLiquidity = 1e18; // 1 ETH worth
        uint256 totalLiquidity = netLiquidity + removedLiquidity;
        
        // Math from _getPremiaDeltas
        // premium0X64_base = collected0 * total * 2^64 / net^2
        uint256 base = uint256(collected0) * totalLiquidity * (2**64) / (netLiquidity**2);
        
        // base is approx 1000 * 1e18 * 1.8e19 = 1.8e40
        // uint128 max is 3.4e38
        assertTrue(base > type(uint128).max, "Base should overflow uint128");
        
        // premium0X64_owed = base * numerator / total
        // numerator ~= total (approx)
        // owed ~= base
        uint256 vegoid = 1;
        uint256 numerator = netLiquidity + (removedLiquidity / vegoid);
        uint256 owed = base * numerator / totalLiquidity;
        
        assertTrue(owed > type(uint128).max, "Owed should overflow uint128");
        
        // This overflow causes addCapped to freeze the accumulator
    }
}

## Suggested Mitigation
Enforce a minimum `netLiquidity` (e.g., 1e6 wei) in `_createLegInAMM` to prevent it from dropping to a level that causes `net^2` to be small enough to trigger overflows in the fee calculation logic. Alternatively, use higher precision accumulators or check for overflow before capping.


## [M-23]. Permanent Denial of Service in SFPM Pools via Zero Vegoid Initialization

## id: UMpS-823DkCVHYyhHqHNu

## Derived From Pattern/Invariant
Dos

## Exploit Type
Dos

## Location
SemiFungiblePositionManager.initializeAMMPool

## Finding Status: Valid
### Finding Status Justification: `initializeAMMPool` is permissionless and accepts `vegoid=0` from any caller; the resulting division-by-zero reverts can lock users into uncloseable positions in that vegoid. This is not limited to (or caused by) a privileged/admin actor, so it is not purely a governance-risk issue.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `initializeAMMPool` function in `SemiFungiblePositionManager` allows any user to initialize a Uniswap V4 pool within the SFPM system. It accepts a `vegoid` parameter (uint8) which is part of the `poolId` generation but also acts as a divisor in premium calculations. The function does not validate that `vegoid` is non-zero. If a pool is initialized with `vegoid=0`, the function succeeds, but subsequent interactions (mint/burn) will invariably fail due to a division-by-zero error in `_getPremiaDeltas` (`removedLiquidity / vegoid`). Since `initializeAMMPool` enforces that a pool (defined by `idV4` + `vegoid`) can only be initialized once, an attacker can intentionally initialize pools with `vegoid=0`, rendering them permanently unusable for that specific parameter set. If specific `vegoid` values are expected by the protocol or frontend, this creates a griefing vector.

## Impact
Permanent DoS of specific pool configurations. Users or the protocol attempting to use a pool initialized with `vegoid=0` will face reverts on all state-changing operations.

## Command to Run Test


## Proof of Concept
1. Attacker calls `initializeAMMPool(poolKey, 0)`. The transaction succeeds, creating a `poolId` with `vegoid=0` and marking it as initialized.
2. Victim (or Protocol) attempts to use this pool (e.g., via `mintTokenizedPosition` using the returned `poolId`).
3. The transaction calls `_createPositionInAMM` -> `_createLegInAMM` -> `_updateStoredPremia` -> `_getPremiaDeltas`.
4. `_getPremiaDeltas` executes `uint256 numerator = netLiquidity + (removedLiquidity / vegoid);`.
5. EVM reverts due to division by zero.
6. The pool is permanently broken.

## Proof of Code
import "forge-std/Test.sol";
import {SemiFungiblePositionManager} from "contracts/SemiFungiblePositionManager.sol";
// Mock imports...

contract VegoidDosTest is Test {
    SemiFungiblePositionManager sfpm;
    // Setup sfpm...

    function testVegoidZeroDos() public {
        // Mock PoolKey
        // PoolKey memory key = ...;
        
        // Initialize with vegoid = 0
        // uint64 poolId = sfpm.initializeAMMPool(key, 0);
        
        // Attempt to mint (requires mocking the V4 interaction)
        // vm.expectRevert(); // Division by zero
        // sfpm.mintTokenizedPosition(... poolId ...);
    }
}

## Suggested Mitigation
Add a check in `initializeAMMPool` to ensure `vegoid > 0`.

```solidity
if (vegoid == 0) revert Errors.InvalidTokenIdParameter(1);
```


## [H-24]. DoS of Liquidation via Max Spread Check in PanopticPool._burnOptions

## id: IMl40VqYjPEMJWdlNPVIR

## Derived From Pattern/Invariant
Dos

## Exploit Type
Dos

## Location
PanopticPool._burnOptions

## Finding Status: Valid
### Finding Status Justification: No downgrade reasons were provided. On the merits, the code path appears live: liquidation burns go through _updateSettlementPostBurn which still applies _checkLiquiditySpread for shorts, and can revert under high utilization. This is a concrete liquidation-DoS/liveness risk, not merely speculative, and not protected by a liquidation bypass.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `_burnOptions` function in `PanopticPool` enforces the `_checkLiquiditySpread` validation whenever a short option position is burned (closed). This check ensures that the ratio of removed liquidity (by long options) to net liquidity (available in AMM) does not exceed `MAX_SPREAD` (90%). When a short position is burned, `netLiquidity` decreases, which increases the spread ratio. If the pool is highly utilized (near the 90% limit), reducing `netLiquidity` further will cause the ratio to exceed the limit, triggering a revert via `Errors.EffectiveLiquidityAboveThreshold()`. 

Crucially, this check is applied even during liquidations (via `_liquidate` -> `_burnAllOptionsFrom` -> `_burnOptions`). An attacker can front-run a liquidation transaction by buying options (removing liquidity) to spike the spread ratio, or the natural market state could reach high utilization. In this state, the liquidator cannot close the insolvent user's short positions because the transaction reverts, preventing liquidation and causing the protocol to accumulate bad debt.

## Impact
Liquidations are blocked for highly utilized chunks, leading to the accumulation of bad debt and potential protocol insolvency.

## Command to Run Test


## Proof of Concept
1. Assume a chunk has 100 Net Liquidity (sellers) and 89 Removed Liquidity (buyers). Ratio = 89/100 = 0.89 (89%). Limit is 0.90.
2. User Alice is insolvent and holds a short position of 10 liquidity in this chunk.
3. Liquidator attempts to liquidate Alice. The process calls `_burnOptions` to close Alice's short position.
4. Burning 10 liquidity reduces Net Liquidity to 90. Removed Liquidity remains 89.
5. New Ratio = 89/90 = 0.988 (98.8%).
6. 0.988 > 0.90 (MAX_SPREAD). The call to `_checkLiquiditySpread` reverts.
7. Liquidation fails. Alice's position remains open, continuing to accrue debt.

## Proof of Code
function testLiquidationDoS() public {
    // Setup: High utilization scenario
    // Assume Alice has short position, Bob has long position
    // Utilization is near 90%
    // Alice becomes insolvent
    // Liquidator calls dispatchFrom to liquidate Alice
    // Expect Revert due to EffectiveLiquidityAboveThreshold
}

## Suggested Mitigation
Modify `_burnOptions` to accept a flag indicating if the call is part of a liquidation, and bypass the `_checkLiquiditySpread` validation in that case. Alternatively, allow `_checkLiquiditySpread` to be skipped if `msg.sender` is the PanopticPool executing a liquidation flow.


## [H-25]. Solvent users can evade interest payments via Force Exercise or Settle Premium due to incorrect delegation ordering

## id: 48oVSKlCjuVYK5u5D_p9M

## Derived From Pattern/Invariant
Call Ordering / Checks-Effects-Interactions

## Exploit Type
CallOrderingOrCEI

## Location
PanopticPool._forceExercise

## Finding Status: Valid
### Finding Status Justification: This is within the explicitly out-of-scope delegate/revoke/interest-index interaction family. If it can be triggered, it is not low impact because it affects interest realization and can socialize losses.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `PanopticPool` contract's `_forceExercise` and `_settlePremium` functions call `CollateralTracker.delegate(owner)` before ensuring that the user's accrued interest is settled against their real balance. 

When `delegate` is called, the user's share balance is inflated to `type(uint248).max` (minus existing debt logic) using phantom shares. Subsequently, the transaction flow calls `_settleOptions` -> `CollateralTracker.settleBurn` -> `_accrueInterest`.

Inside `_accrueInterest`, the contract checks if the user has enough shares to pay the interest: `if (shares > userBalance)`. Since the balance is now inflated (Phantom), this check passes even if the user has 0 real shares. The interest is 'paid' by burning phantom shares, and the user's borrow index is updated, effectively marking the debt as cleared.

Finally, `revoke` is called, which detects that phantom shares were burned and restores the `_internalSupply` to compensate. The result is that the user's interest debt is erased from the system without burning any real collateral or assets, effectively socializing the debt onto other liquidity providers, even if the user was technically solvent (e.g., holding valuable option positions that could have covered the debt).

## Impact
Users can wipe out their accrued interest debt without paying, leading to loss of yield for liquidity providers and protocol insolvency.

## Command to Run Test


## Proof of Concept
1. Attacker opens a Short Put position (borrower) and a Long Call position (exercisable).
2. Attacker withdraws all collateral shares (if allowed by solvency of the Long Call), leaving 0 real shares.
3. Time passes, and significant interest accrues on the Short Put.
4. Attacker calls `forceExercise` on their own Long Call.
5. `PanopticPool` calls `delegate(attacker)`, setting their balance to `MAX`.
6. `PanopticPool` burns the Long Call. `settleBurn` calls `accrueInterest`.
7. `accrueInterest` burns phantom shares to pay the debt and resets the attacker's interest index.
8. `revoke(attacker)` restores the supply.
9. Attacker's interest debt is gone; they paid nothing.

## Proof of Code
contract InterestEvasionTest is Test, PanopticHelper {
    function testInterestEvasion() public {
        // Setup: Create pool, add liquidity
        (PanopticPool pool, CollateralTracker ct0, CollateralTracker ct1) = createPool();
        uint256 amount = 1000 ether;
        ct0.deposit(amount, alice);
        ct1.deposit(amount, alice);
        
        // Alice mints a Short Put (borrower) and a Long Call (asset)
        vm.startPrank(alice);
        // TokenId: leg0=Short Put, leg1=Long Call. 
        // Simplified for brevity: mint positions such that Alice borrows but stays solvent via Long option value
        // ... (Minting logic) ...
        
        // Withdraw collateral to 0 (simulate liquidity crunch but solvent via option value)
        ct0.withdraw(ct0.balanceOf(alice), alice, alice);
        vm.stopPrank();

        // Time travel to accrue interest
        vm.warp(block.timestamp + 30 days);

        // Verify Alice owes interest
        uint256 interestOwed = ct0.owedInterest(alice);
        assertGt(interestOwed, 0);

        // Alice Force Exercises herself (or SettlePremium)
        vm.prank(alice);
        pool.dispatch(forceExerciseArgs);

        // Check debt is wiped
        uint256 interestAfter = ct0.owedInterest(alice);
        assertEq(interestAfter, 0);
        // Check Alice paid 0 real assets (balance was 0)
    }
}

## Suggested Mitigation
In `PanopticPool.sol`, inside `_forceExercise` and `_settlePremium` (and any other flow using delegation for solvent users), call `CollateralTracker.accrueInterest(account)` for both tokens *before* calling `CollateralTracker.delegate(account)`. This ensures interest is settled against the user's real balance (potentially triggering the insolvency logic that preserves the debt index) before phantom shares cloak the lack of funds.


## [M-26]. PLPs receive zero commission yield when Builder Code is used

## id: cJmMgdshXv5MI5e6s9HBB

## Derived From Pattern/Invariant
FeeAccountingDrift

## Exploit Type
AccountingInvariantViolation

## Location
CollateralTracker.settleMint

## Finding Status: Valid
### Finding Status Justification: Not low impact: this is a systematic fee-accounting/distribution bug that leaks ~10% of the commission amount on every mint/burn with a builder code (i.e., recurring value loss/discount). Over meaningful volume, aggregate loss can be material.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `CollateralTracker.settleMint` and `settleBurn`, the protocol collects commission fees from users. If a `builderCode` (feeRecipient) is provided, the code splits the fee between the protocol (`PROTOCOL_SPLIT` = 65%) and the builder (`BUILDER_SPLIT` = 25%). However, the remaining 10% of the fee (implicitly allocated to Panoptic Liquidity Providers or PLPs) is neither burned nor transferred. Unlike the case where `feeRecipient == 0` (where 100% is burned, benefiting PLPs), the `else` block only performs `_transferFrom` for the protocol and builder portions. The remaining shares are left in the user's balance. This results in PLPs receiving zero yield from commissions when a builder code is used, and the user effectively receiving a 10% fee discount.

## Impact
Panoptic Liquidity Providers (PLPs) permanently lose their share of commission yield (10% of total fees) whenever a transaction uses a builder referral code.

## Command to Run Test


## Proof of Concept
1. User calls `dispatch` to mint an option with a `builderCode`. 2. `PanopticPool` calls `CollateralTracker.settleMint`. 3. `settleMint` calculates `commissionFee` and `sharesToBurn`. 4. Since `feeRecipient != 0`, it enters the `else` block. 5. It transfers 65% of shares to RiskEngine and 25% to Builder. 6. It emits `CommissionPaid` event. 7. The remaining 10% of shares are NOT burned and remain with the user. PLPs receive no benefit.

## Proof of Code
function test_PLPFeeBypass() public {
    // Setup user, builder, and collateral
    address user = address(0x1);
    address builder = address(0x2);
    uint256 builderCode = 1;
    // ... setup logic ...
    // Mint with builder code
    vm.prank(user);
    pool.dispatch(..., builderCode);
    // Check PLP share price or totalAssets - unchanged/lower than expected
    // Verify user balance - higher than expected (10% fee saved)
}

## Suggested Mitigation
In the `else` block of `settleMint` and `settleBurn`, add a `_burn(optionOwner, sharesToBurn - protocolShares - builderShares)` call to burn the remaining portion of the fee for PLPs.


## [H-27]. Permanent share price inflation due to unrecoverable bad debt in CollateralTracker

## id: Gn8kgv-XXxB9yEz6jQ9uI

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
CollateralTracker._accrueInterest

## Finding Status: Valid
### Finding Status Justification: The scenario (partial interest payment without clearing the corresponding global unrealizedInterest, followed by netBorrows dropping to 0) is plausible in normal flows; it does not require exceptionally rare conditions, and can occur whenever an underfunded borrower closes/settles in a way that zeroes netBorrows while leaving some previously accrued interest unpaid.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `CollateralTracker.sol`, the `_accrueInterest` function handles interest payments from borrowers. If a borrower is insolvent (interest owed > share balance), the function burns their entire balance to pay as much as possible, but the remaining unpaid interest is not cleared from the global `s_marketState.unrealizedInterest` accumulator. 

When the insolvent user is subsequently liquidated, their positions are closed and `netBorrows` drops to zero, stopping future interest accrual. However, the previously accrued but unpaid interest remains in `unrealizedInterest` forever. 

Since `totalAssets()` includes `unrealizedInterest` (treating it as a receivable asset), the vault overstates its total assets by the amount of this bad debt. This permanently inflates the share price (`totalAssets` / `totalSupply`), allowing users to withdraw more assets than they are entitled to, effectively draining the vault and socializing the bad debt onto the last remaining depositors who will face a deficit.

## Impact
Protocol insolvency and loss of funds for depositors as the share price becomes permanently decoupled from real assets.

## Command to Run Test


## Proof of Concept
1. Attacker (or victim) opens a large short position.
2. Interest accrues until the user becomes insolvent (`interestOwed > balance`).
3. User is liquidated: `dispatch` calls `_burnAllOptionsFrom`, which calls `settleBurn`, which calls `_accrueInterest`.
4. `_accrueInterest` burns the user's remaining shares but leaves the unpaid portion of interest in `s_marketState.unrealizedInterest`.
5. `_burnAllOptionsFrom` closes positions, setting `netBorrows` to 0.
6. The user is now gone (0 shares, 0 positions), but `unrealizedInterest` still holds the unpaid debt.
7. `totalAssets()` is now `realAssets + badDebt`.
8. Other users withdrawing via `withdraw/redeem` receive payouts based on the inflated `totalAssets`, extracting value from the protocol and leaving a hole for the last users.

## Proof of Code
    function testBadDebtInflatesSharePrice() public {
        // Setup: Bob deposits and mints short options
        vm.startPrank(bob);
        collateralTracker.deposit(1000 ether, bob);
        // ... mint short option ...
        vm.stopPrank();

        // Advance time to accrue massive interest (insolvency)
        vm.warp(block.timestamp + 365 days);

        // Bob is insolvent. Alice liquidates Bob.
        vm.startPrank(alice);
        panopticPool.dispatchFrom(..., bob, ...);
        vm.stopPrank();

        // Assert bad debt remains in unrealizedInterest
        uint256 unrealized = collateralTracker.unrealizedGlobalInterest();
        assertGt(unrealized, 0);
        
        // Assert share price is inflated (Assets > Supply despite 1:1 initial)
        assertGt(collateralTracker.convertToAssets(1 ether), 1 ether);
    }

## Suggested Mitigation
Implement a bad debt socialization mechanism. When `_accrueInterest` detects insolvency (shares > userBalance), the unpaid portion of the interest should be explicitly deducted from `_unrealizedGlobalInterest` to write off the asset that will never be collected. Alternatively, handle bad debt write-off during the liquidation process.


## [H-28]. Theft of funds via Force Exercise fee manipulation using Flash Loans

## id: v3D99ywH7Ue3dnN7uA7U1

## Derived From Pattern/Invariant
FlashLoanEconomicManipulation

## Exploit Type
FlashLoanEconomicManipulation

## Location
RiskEngine.exerciseCost

## Finding Status: Valid
### Finding Status Justification: Given exerciseCost's sign structure (subtracting current-oracle value deltas from an initially negative fee), it is plausible for bounded spot-vs-oracle discrepancies to flip a fee component positive, turning it into a transfer from victim to exercisor. The TWAP-delta bound limits magnitude but does not inherently prevent sign-flips, so it is not clearly non-exploitable nor clearly rare.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `exerciseCost` function in `RiskEngine.sol` calculates the fee paid by a force-exercisor to the exercisee (victim). This calculation includes an adjustment term intended to account for the discrepancy between the Spot price (used for burning the position) and the Oracle price (fair value). 

The code subtracts the difference `(currentValue - oracleValue)` from the base fee: `exerciseFees = exerciseFees.sub(LeftRightSigned...addToRightSlot(currentValue - oracleValue)...)`. 

Since `exerciseFees` starts as a negative value (representing a payment *to* the victim), subtracting this difference creates a vulnerability. If an attacker manipulates the Spot price via a flash loan such that `currentValue` is significantly lower than `oracleValue` (e.g., pushing a Long position OTM), the term `(currentValue - oracleValue)` becomes negative. Subtracting a negative value is equivalent to adding a positive value, which increases `exerciseFees`. 

If the manipulation is large enough (within the 5% buffer allowed by `MAX_TWAP_DELTA_LIQUIDATION`), `exerciseFees` can flip from negative (payment to victim) to positive (payment *from* victim). In `CollateralTracker.getRefundAmounts` (called by `_forceExercise`), a positive fee triggers a transfer of assets from the `payor` (victim) to the `msg.sender` (attacker). This allows an attacker to force-close a victim's position at a manipulated unfavorable price AND force the victim to pay the attacker for the 'privilege', effectively stealing the intrinsic value difference.

## Impact
Direct theft of user collateral. An attacker can drain funds from users holding long positions by manipulating the spot price and force-exercising them.

## Command to Run Test


## Proof of Concept
1. Victim holds a large Long position (e.g., 100 ETH Calls) that is In-The-Money (ITM) according to the TWAP Oracle.
2. Attacker flash loans a large amount of tokens and swaps in the Uniswap pool to push the Spot price down, making the position Out-Of-The-Money (OTM) or significantly less valuable at Spot compared to Oracle. The price move is kept within `MAX_TWAP_DELTA_LIQUIDATION` (approx 5%).
3. Attacker calls `PanopticPool.forceExercise` on the victim's position.
4. `RiskEngine.exerciseCost` calculates the fee. The base fee is small (1bp) because the position is OTM at Spot. The adjustment term `currentValue - oracleValue` is a large negative number (e.g., -5 ETH).
5. `exerciseFees` = -0.01 ETH - (-5 ETH) = +4.99 ETH.
6. `PanopticPool` calls `ct.refund(victim, attacker, 4.99 ETH)`. This transfers 4.99 ETH from the Victim's collateral to the Attacker.
7. Victim's position is burned at the manipulated Spot price (worth near 0).
8. Attacker swaps back to repay flash loan and profits ~4.99 ETH minus fees.

## Proof of Code
function testForceExerciseTheft() public {
    // Setup: User mints a long position
    vm.startPrank(user);
    collateralTracker0.deposit(10 ether, user);
    // Mint ITM Long Position...
    vm.stopPrank();

    // Attack
    vm.startPrank(attacker);
    // 1. Manipulate price (simulate flash loan swap)
    // Move tick down by ~500 ticks (approx 5%)
    int24 currentTick = sfpm.getCurrentTick(poolKey);
    poolManager.swap(poolKey, ...); // Push price down

    // 2. Force Exercise
    // Check balances before
    uint256 attackerBalBefore = collateralTracker0.balanceOf(attacker);
    
    panopticPool.dispatchFrom(..., attacker, [tokenId], ...);
    
    // 3. Check profit
    uint256 attackerBalAfter = collateralTracker0.balanceOf(attacker);
    assertGt(attackerBalAfter, attackerBalBefore, "Attacker should profit from inverted fee logic");
}

## Suggested Mitigation
Invert the sign of the value adjustment in `RiskEngine.exerciseCost`. The intent is to compensate the victim if they are forced out at a bad Spot price. Therefore, if `currentValue < oracleValue`, the payment to the victim (negative fee) should increase magnitude (become more negative). 

Change:
`exerciseFees = exerciseFees.sub(delta)`
To:
`exerciseFees = exerciseFees.add(delta)` 
(or equivalent logic ensuring the exercisor pays the difference).


## [M-29]. Incentive misalignment in `RiskEngine.exerciseCost` rewards price manipulation against forced exercise targets

## id: JIBvmcc-titE8A0e3crwS

## Derived From Pattern/Invariant
IncentiveMisalignmentOrGameTheory

## Exploit Type
IncentiveMisalignmentOrGameTheory

## Location
RiskEngine.exerciseCost

## Finding Status: Valid
### Finding Status Justification: RiskEngine.exerciseCost subtracts `(currentValue - oracleValue)` from exerciseFees for each long leg. When the current price is manipulated to be worse for the long holder (e.g., currentValue < oracleValue), (currentValue-oracleValue) becomes negative and subtracting it increases exerciseFees (toward zero), reducing the magnitude of the negative fee ultimately paid by the exercisor. This is opposite the intuitive compensation goal described in the comments (that unfavorable execution should be compensated). While dispatchFrom enforces a TWAP/currentTick proximity bound, it still allows bounded, atomic spot manipulation within that range, making the mispricing economically exploitable.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `RiskEngine.exerciseCost` function calculates the fee an exercisor must pay to force-exercise a long position. The formula includes a slippage adjustment: `exerciseFees = exerciseFees.sub(currentValue - oracleValue)`. 

If the current spot price (`currentValue`) is manipulated to be worse for the option holder than the oracle price (`oracleValue`), the term `(currentValue - oracleValue)` becomes negative. Subtracting a negative term adds a positive value to `exerciseFees`. Since `exerciseFees` is initially negative (representing a payment *from* exercisor *to* holder), adding a positive value makes it *less negative* (closer to zero). 

This means the exercisor pays *less* fee if they manipulate the price to execute the forced burn at a bad rate for the holder. This creates a perverse incentive for the exercisor (who might also be the counterparty seller) to sandwich the force-exercise transaction: crash the price, force exercise (holder gets bad payout + low fee), then recover price. The holder suffers a double loss (bad execution + reduced compensation).

## Impact
Exercisors are economically incentivized to manipulate the spot price against option holders during force exercises, leading to reduced compensation fees and worse execution prices for victims.

## Command to Run Test


## Proof of Concept
1. Attacker identifies a target user holding a Long Call position that is near OTM.
2. Attacker uses a flash loan to swap in the Uniswap pool, pushing the price down significantly (but within the 5% TWAP delta limit enforced by `dispatchFrom`).
3. This makes the position deeply OTM (triggering the cheaper 1bps base fee if applicable, or just affecting the delta term).
4. Attacker calls `dispatchFrom` to force exercise the victim's position.
5. Inside `exerciseCost`: `currentValue` (at manipulated low price) is much lower than `oracleValue`.
6. The fee calculation reduces the payout to the victim because of the price deviation.
7. The victim's position is burnt at the manipulated low price (SFPM interaction).
8. Attacker swaps back to close the flash loan.
9. Attacker benefits from paying a reduced fee and potentially clearing a liability cheaply.

## Proof of Code
function testForceExerciseIncentive() public {
    // Setup position
    // Manipulate price down
    int256 feeManipulated = riskEngine.exerciseCost(manipulatedTick, oracleTick, ...);
    // Reset price
    int256 feeNormal = riskEngine.exerciseCost(oracleTick, oracleTick, ...);
    // Assert feeManipulated (magnitude) < feeNormal (magnitude)
    // i.e., exercisor pays less
}

## Suggested Mitigation
The slippage adjustment term in `exerciseCost` should be inverted or removed. If the execution price is worse than the oracle price for the holder, the exercisor should be required to pay *more* to compensate, not less.


## [H-30]. Flash Loan Price Manipulation allows bypassing Force Exercise Fees in RiskEngine

## id: am8VvV5wXNXGSf8fElFo_

## Derived From Pattern/Invariant
FlashLoanEconomicManipulation

## Exploit Type
FlashLoanEconomicManipulation

## Location
RiskEngine.exerciseCost

## Finding Status: Valid
### Finding Status Justification: exerciseCost uses currentTick (spot) for the in-range vs out-of-range tier switch, and forceExercise/dispatchFrom does not enforce a TWAP-vs-spot bound like liquidations do. This makes the tier selection atomically manipulable; the (currentValue-oracleValue) adjustment does not prevent boundary-flip savings in general, it only offsets some value differences.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `RiskEngine.exerciseCost` function determines the fee tier for force-exercising a position based on whether the position is 'in range' relative to the *current* spot price (`currentTick`). If the position is in-range, the fee is high (approx. 1%); if out-of-range (OTM), the fee is low (1 bps). An attacker can use a flash loan to manipulate the Uniswap pool price (`currentTick`) to be just outside the range of a victim's long position. This triggers the low fee tier. The attacker then force-exercises the victim's position, burning it for a negligible fee, and repays the flash loan. This allows the attacker (the option seller) to reclaim collateral/liquidity cheaply while griefing the victim who loses their position without fair compensation.

## Impact
Attackers can close short positions against victims at a fraction of the intended cost, causing loss of option value for the victim.

## Command to Run Test


## Proof of Concept
1. Victim holds a long option position with range [L, U]. Current price P is inside [L, U]. 2. Attacker flash loans funds and swaps in the Uniswap pool to move price P' to U + 1 (OTM). 3. Attacker calls `PanopticPool.dispatchFrom` to force-exercise the victim. 4. `RiskEngine.exerciseCost` checks range using P', sees OTM, and assesses 1bps fee. 5. Victim position is burned. 6. Attacker swaps back to P and repays loan. 7. Attacker effectively closed the position avoiding the ~1% fee.

## Proof of Code
function testFlashLoanExercise() public { 
    // Setup position ITM 
    // Flash loan / swap to move tick OTM 
    vm.startPrank(attacker); 
    pool.swap(..., targetTick); 
    // Force exercise 
    panopticPool.dispatchFrom(..., victim, ...); 
    // Verify fee paid was minimal (1bps) 
}

## Suggested Mitigation
Use the oracle tick (e.g., `oracleTick` passed to the function, which is a median/EMA) instead of `currentTick` (spot price) to determine whether the legs are in-range for the fee tier switch. This prevents spot price manipulation from affecting the fee logic.





Finding Status: LowSeverityDueToLowImpact
## [H-31]. Broken ITM Swap Logic in SFPM Swaps Asset Tokens to Non-Asset Tokens

## id: 19on_1jKlQUkq9IGM_seC

## Derived From Pattern/Invariant
IncentiveMisalignmentOrGameTheory

## Exploit Type
Custom

## Location
SemiFungiblePositionManager.swapInAMM

## Finding Status: LowSeverityDueToLowImpact
### Finding Status Justification: `SemiFungiblePositionManager.swapInAMM` chooses swap direction/amount based on `asset`: if `asset==0`, it swaps based on `itm0`; else it swaps based on `itm1` (`zeroForOne = itm1 > 0; swapAmount = itm1`). Given SFPM’s sign convention (positive deltas = user pays into AMM, negative = user receives), this logic can select the “asset” side rather than swapping non-asset exposure into the asset, producing the inverted result described. However, the impact is limited: the ITM netting swap only runs when the user intentionally supplies inverted tick limits (`invertedLimits`), and PanopticPool’s forced actions use normal limits (so they do not hit this path). Users can avoid the feature entirely. Thus it is a real logic bug but primarily a UX/value-leak via unnecessary/wrong-direction swaps rather than a protocol-drain vector.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `SemiFungiblePositionManager.swapInAMM` function is intended to net out token deltas so that users can transact primarily in the asset token (e.g., Token1 for a Put). However, the logic consistently identifies the *Asset* token delta and swaps it into the *Non-Asset* token. 

For example, if `asset=1` (Token1) and the user burns a position receiving `itm1` (Token1) and `itm0` (Token0), the logic checks `itm1`, sets `swapAmount = itm1`, and `zeroForOne = false` (Token1 -> Token0). This sells the Asset (Token1) to buy the Non-Asset (Token0), leaving the user with exposure to the wrong token. The logic should instead identify the Non-Asset delta and swap it into the Asset.

## Impact
Users burning In-The-Money (ITM) positions using the netting feature will unknowingly sell their asset tokens and acquire non-asset tokens. This exposes them to unintended price risk (FX risk) and fails to deliver the promised netting functionality. In `asset=1` pools, burning a deep ITM put (returning mostly Token0 and some Token1) will result in selling the Token1 for *more* Token0, leaving the user with 100% Token0 instead of the expected Token1.

## Command to Run Test


## Proof of Concept
1. Consider a Pool with Token0 (ETH) and Token1 (USDC). `asset=1` (USDC).
2. User burns an ITM Put. They receive `100 ETH` (`itm0 = -100`) and `1000 USDC` (`itm1 = -1000`). (Note: SFPM sign convention is Negative for User Receive).
3. `swapInAMM` is called with `asset=1`.
4. Logic hits `else` (`asset==1`).
5. `zeroForOne = itm1 > 0`. `itm1` is negative, so `false`. (Swap 1 -> 0).
6. `swapAmount = itm1` (-1000). Uniswap Exact Input = 1000.
7. Contract calls Uniswap: Sell 1000 USDC for ETH.
8. User ends up with `100 + X` ETH and `0` USDC.
9. User expected to net out ETH into USDC (the asset). Instead, they dumped their USDC for ETH.

## Proof of Code
function testBrokenSwap() public {
    // Setup pool and position
    // ... (Requires extensive setup of Pool/SFPM)
    // Mocking the logic:
    int128 itm0 = -100; // Receive 100 Token0
    int128 itm1 = -1000; // Receive 1000 Token1
    uint256 asset = 1; // Asset is Token1
    
    // Current Logic
    bool zeroForOne = itm1 > 0; // false (1->0)
    int256 swapAmount = itm1;
    
    // Assertion: Logic sells Token1 (Asset) for Token0
    assertEq(zeroForOne, false); // 1 -> 0
    // Correct Logic should swap itm0 (Token0) to Token1
    // Expectation failed.
}

## Suggested Mitigation
Rewrite `swapInAMM` to identify the *Non-Asset* delta and swap it to the *Asset*. 
If `asset == 1`: swap `itm0` to `token1` (if `itm0 != 0`). `zeroForOne = itm0 < 0` (sell itm0). `swapAmount = itm0`.
If `asset == 0`: swap `itm1` to `token0` (if `itm1 != 0`). `zeroForOne = itm1 > 0` (sell itm1). `swapAmount = itm1`.





Finding Status: LowSeverityDueToRareLikelihood + InvalidERC20EdgeCase
## [H-32]. Read-Only Reentrancy in CollateralTracker.deposit via ERC777 hooks allows theft of vault value

## id: vaG3qfiAHl9X3IVxhzBUZ

## Derived From Pattern/Invariant
ERC777HookReentrancy

## Exploit Type
Reentrancy

## Location
CollateralTracker.deposit

## Finding Status: LowSeverityDueToRareLikelihood + InvalidERC20EdgeCase
### Finding Status Justification: CollateralTracker.deposit computes shares = previewDeposit(assets) before executing the external SafeTransferLib.safeTransferFrom(underlyingToken, msg.sender, panopticPool, assets). With an ERC777-like token, msg.sender’s hook can reenter CollateralTracker and call donate (burning existing shares), reducing totalSupply without yet increasing s_depositedAssets for the pending deposit (effects occur after transfer). When deposit resumes, it mints the precomputed shares, which now represent a larger fraction of the pool, creating value extraction from other holders. There is no nonReentrant guard or post-transfer share recomputation. This relies on hook-enabled tokens.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `deposit` function in `CollateralTracker.sol` calculates the amount of shares to mint based on the `previewDeposit` result (which uses the current `totalAssets` and `totalSupply`) *before* transferring the assets from the user. It then calls `safeTransferFrom` to pull the assets. If the underlying token is an ERC777 token (or supports transfer hooks), the sender receives control flow via `tokensToSend`. Inside this hook, the attacker can call `donate` to burn their existing shares, which decreases `totalSupply` without changing `totalAssets`, effectively artificially inflating the share price. When the hook returns, `deposit` resumes and mints the originally calculated number of shares. Since the share price has increased, the shares minted are worth more than the assets deposited, allowing the attacker to steal value from other liquidity providers.

## Impact
An attacker can steal funds from the CollateralTracker vault by manipulating the share price mid-deposit, draining value from other users.

## Command to Run Test


## Proof of Concept
1. Attacker holds some shares in `CollateralTracker`. 2. Attacker calls `deposit(amount)`. 3. `previewDeposit` calculates `shares` based on current price (e.g. 1:1). 4. `safeTransferFrom` triggers attacker's hook. 5. Inside hook, attacker calls `donate(existingShares)`. 6. `donate` burns shares, reducing supply but keeping assets constant. Share price doubles (e.g. 2:1). 7. Hook returns. 8. `deposit` mints the pre-calculated `shares`. 9. Attacker receives shares valued at 2:1 but paid 1:1, profiting instantly.

## Proof of Code
function testReentrancyExploit() public { 
    // Setup: deploy vault with ERC777 underlying, deposit initial liquidity 
    vm.startPrank(attacker); 
    // 1. Calculate expected shares 
    uint256 depositAmount = 100 ether; 
    // 2. Call deposit, hook triggers donate 
    vault.deposit(depositAmount, attacker); 
    // 3. Check profit 
    assertGt(vault.convertToAssets(vault.balanceOf(attacker)), depositAmount); 
}

## Suggested Mitigation
Add a `nonReentrant` modifier to the `deposit`, `mint`, `withdraw`, and `redeem` functions in `CollateralTracker.sol`. Alternatively, implement checks-effects-interactions by updating accounting before external calls, though strictly applied CEI is difficult with transfer-from logic.


## [H-33]. Infinite Share Minting and Vault Drainage via Self-Liquidation Reentrancy in CollateralTracker

## id: sKlQH_P6qQv9GyHGXV0f-

## Derived From Pattern/Invariant
ERC777HookReentrancy

## Exploit Type
Reentrancy

## Location
CollateralTracker.sol.transfer

## Finding Status: LowSeverityDueToRareLikelihood + InvalidERC20EdgeCase
### Finding Status Justification: During PanopticPool._liquidate, CollateralTracker.delegate(liquidatee) grants ~2^248 phantom balance (not added to totalSupply). Later CollateralTracker.settleLiquidation(bonus<0) performs SafeTransferLib.safeTransferFrom(underlyingToken, liquidator, msg.sender, bonusAbs) before removing phantom shares. If the underlying token has ERC777-style hooks, a self-liquidating liquidator can reenter CollateralTracker.transfer while numberOfLegs(liquidatee)==0 (post _burnAllOptionsFrom) and transfer phantom balance to another account. settleLiquidation then observes type(uint248).max > liquidateeBalance and increases _internalSupply by (type(uint248).max - liquidateeBalance), effectively making the transferred phantom balance redeemable supply. This can allow draining vault assets. The issue relies on hook-enabled tokens (non-standard ERC20 behavior) and a reachable bonus<0 scenario, making likelihood rare but impact catastrophic.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
A reentrancy vulnerability exists in `CollateralTracker.sol` combined with the liquidation flow in `PanopticPool.sol`. During liquidation (`dispatchFrom` -> `_liquidate`), the protocol temporarily delegates `type(uint248).max` phantom shares to the liquidatee to facilitate settlement. The `_liquidate` function subsequently burns all options (resetting the user's `numberOfLegs` to 0) and calls `settleLiquidation`. If the liquidation bonus is negative (which occurs if the liquidatee has significant protocol loss covered by haircutting their long premiums), `settleLiquidation` initiates a `safeTransferFrom` to pull tokens from the liquidator (who can be the liquidatee themselves in a self-liquidation). If the underlying token is an ERC777 or has transfer hooks, the liquidator can reenter `CollateralTracker.transfer`. Since `numberOfLegs` is 0 and the user holds `type(uint248).max` delegated shares, they can transfer these phantom shares to a secondary account. Finally, `revoke` executes, sees a zero/low balance for the liquidatee, and incorrectly inflates `_internalSupply` to compensate for the missing shares. The secondary account is left holding valid phantom shares that can be redeemed to drain the entire vault.

## Impact
Complete theft of all assets deposited in the CollateralTracker vault.

## Command to Run Test


## Proof of Concept
1. Attacker creates a portfolio with a large Short position (causing insolvency) and a Long position (paid premium) to ensure the liquidation calculation results in a negative bonus (due to `haircutPremia` clawbacks). 2. Attacker calls `dispatchFrom` to liquidate themselves. 3. `CollateralTracker.delegate` gives Attacker 2^248 shares. 4. `_burnAllOptionsFrom` closes positions; `numberOfLegs` becomes 0. 5. `settleLiquidation` sees negative bonus and calls `safeTransferFrom` on Attacker. 6. Attacker's ERC777 `tokensToSend` hook triggers. 7. Attacker calls `CollateralTracker.transfer(Attacker2, 2^248)` inside the hook. 8. `revoke` executes, sees Attacker's balance is 0, and inflates `_internalSupply`. 9. Attacker2 redeems the shares for all vault assets.

## Proof of Code
function testExploit() public { /* Mock ERC777 and setup insolvent position */ vm.prank(attacker); panopticPool.dispatchFrom(..., attacker, ...); /* Inside hook: collateralTracker.transfer(receiver, type(uint248).max); */ }

## Suggested Mitigation
In `CollateralTracker.transfer` and `transferFrom`, add a check to ensure `balanceOf[msg.sender]` is less than `type(uint248).max`, preventing the transfer of delegated phantom shares.


## [M-34]. Storage corruption in initializeAMMPool via reentrancy leads to poolId collision and DoS of tick expansion

## id: Nc-NCDHzX9RAxy_9lXfWt

## Derived From Pattern/Invariant
StorageCollisionOrSelectorClash

## Exploit Type
Reentrancy

## Location
SemiFungiblePositionManager.initializeAMMPool

## Finding Status: LowSeverityDueToRareLikelihood + InvalidERC20EdgeCase
### Finding Status Justification: `initializeAMMPool` is not `nonReentrant` and makes external calls to `IERC20Partial(...).totalSupply()` before writing `s_V4toSFPMIdData` and `s_poolIdToKey`. The poolId collision resolution checks `s_poolIdToKey[poolId].tickSpacing != 0` first; a reentrant call during `totalSupply()` can initialize another pool using the same `poolId` (given a feasible 40-bit collision), then the outer call resumes and overwrites `s_poolIdToKey[poolId]`. This can desynchronize `s_V4toSFPMIdData[idV4][vegoid]` vs `s_poolIdToKey[poolId]`, breaking lookups and causing downstream DoS/misbehavior (e.g., `expandEnforcedTickRange` using the wrong key). Exploit requires a reentrant/non-standard token and a crafted poolId collision, so likelihood is rare but the code path is real.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `initializeAMMPool` function assigns a unique `poolId` to a Uniswap V4 pool by hashing its key and handling collisions via `incrementPoolPattern`. The collision check (`s_poolIdToKey[poolId].tickSpacing != 0`) and the state updates (`s_V4toSFPMIdData` and `s_poolIdToKey`) are separated by external calls to `IERC20Partial(currency).totalSupply()` within the `minEnforcedTick` calculation.

An attacker can exploit this by using a malicious token with a reentrant `totalSupply()` function. By creating a malicious pool that collides (in the 40-bit hash prefix) with a target victim pool, the attacker can: 1. Call `initializeAMMPool` for the malicious pool. 2. Pass the collision check (slot empty). 3. Reenter in `totalSupply` and initialize the victim pool. The victim pool claims the `poolId` P. 4. Resume the first call and overwrite `s_poolIdToKey[P]` with the malicious pool's key.

Consequently, `s_V4toSFPMIdData` for the victim pool points to `poolId` P, but `s_poolIdToKey[P]` points to the malicious pool. This breaks `expandEnforcedTickRange` for the victim pool (which relies on `s_poolIdToKey` to fetch the key), effectively permanently freezing its enforced tick range and causing DoS for users attempting to mint positions outside the initial range.

## Impact
The victim pool's tick range configuration becomes frozen (DoS on `expandEnforcedTickRange`), preventing it from adapting to token supply changes. Additionally, `getUniswapV4PoolKeyFromId` returns incorrect data for the victim pool's ID.

## Command to Run Test


## Proof of Concept
1. Attacker identifies a victim pool Key V. Calculates its 40-bit hash prefix H.
2. Attacker generates a Malicious Key M (using a custom token) that also hashes to H in the 40-bit space.
3. Attacker calls `initializeAMMPool(M)`.
4. `_getPoolId(M)` calculates ID P. The loop checks `s_poolIdToKey[P]`, which is empty.
5. The code proceeds to calculate `minEnforcedTick`, calling `M.currency1.totalSupply()`.
6. Inside `totalSupply()`, the attacker calls `initializeAMMPool(V)`.
7. `_getPoolId(V)` calculates ID P. The loop checks `s_poolIdToKey[P]`, still empty. `V` is initialized: `s_V4toSFPMIdData[V] = P`, `s_poolIdToKey[P] = V`.
8. The inner call returns.
9. The outer call resumes. It writes `s_V4toSFPMIdData[M] = P` and overwrites `s_poolIdToKey[P] = M`.
10. `V` remains mapped to P, but P resolves to `M`. `expandEnforcedTickRange(P)` now operates on `M`, leaving `V` frozen.

## Proof of Code
/* 
// To run this test, place it in a file (e.g., SFPMReentrancy.t.sol) in test/foundry/ and run `forge test`.
// Note: Requires mocks for IPoolManager and setup similar to DeployProtocol script.
*/

contract ReentrantToken {
    SemiFungiblePositionManager sfpm;
    PoolKey victimKey;
    bool entered;

    constructor(SemiFungiblePositionManager _sfpm, PoolKey memory _victim) {
        sfpm = _sfpm;
        victimKey = _victim;
    }

    function totalSupply() external view returns (uint256) {
        // Need to cast to non-view to perform reentrancy in this PoC context
        // In reality, one would use a non-view function if interface allowed or just use the view call if possible to trigger side-effect in older sol or via complex setup.
        // For PoC simplicity, we assume the interface call allows re-entry.
        if (!entered) {
            // Hack to toggle flag in view function for PoC logic simulation
            // In real attack, this would be a standard reentrancy
             try sfpm.initializeAMMPool(victimKey, 0) {} catch {}
        }
        return 1000 ether;
    }
}
// Note: Full compilable test requires mocking V4 environment which is extensive.
// The logic above demonstrates the callback path.

## Suggested Mitigation
Add the `nonReentrant` modifier to the `initializeAMMPool` function to prevent reentrant calls during the initialization process.





Finding Status: LowSeverityDueToRareLikelihood
## [H-35]. Theft of stuck ETH in PanopticPool via Multicall msg.value reuse

## id: G_VN1cAgFDC5AIsDc7RUn

## Derived From Pattern/Invariant
Unexpected Ether

## Exploit Type
UnexpectedEth

## Location
PanopticPool.dispatchFrom

## Finding Status: LowSeverityDueToRareLikelihood
### Finding Status Justification: Multicall uses delegatecall, preserving the original msg.value for each internal call. PanopticPool.dispatchFrom is payable and, in the liquidation path, forwards msg.value to CollateralTracker.settleLiquidation{value: msg.value}. In CollateralTracker.settleLiquidation when bonus >= 0 it refunds msg.value back to liquidator. If PanopticPool already holds ETH (e.g., via accidental value to payable functions where value is not forwarded/used, multicall value, or forced ETH), a multicall batching multiple dispatchFrom liquidations can forward msg.value multiple times, refunding each time, draining the pre-existing ETH balance. Exploit requires PanopticPool to already have enough ETH to cover repeated forwards, so likelihood is rare; impact limited to stuck ETH but can be meaningful.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `PanopticPool` contract inherits `Multicall`, which allows batching multiple calls in a single transaction. `PanopticPool` also contains the `dispatchFrom` function, which is `payable` and forwards `msg.value` to the `CollateralTracker` via `ct0.settleLiquidation{value: msg.value}(...)`. 

When `Multicall.multicall` is called with a value (e.g., 1 ETH), the `msg.value` for the transaction is set to 1 ETH. Inside the `multicall` loop, `delegatecall` preserves this `msg.value`. If an attacker batches multiple `dispatchFrom` calls in one transaction, each call will attempt to transfer `msg.value` (1 ETH) from `PanopticPool`'s balance to the `CollateralTracker`. 

In `CollateralTracker.settleLiquidation`, if `bonus > 0` (which implies the protocol pays the liquidator/caller), the function refunds `msg.value` to the caller (`liquidator`). 

Therefore, an attacker can send 1 ETH, execute `dispatchFrom` twice (triggering `bonus > 0` path), and receive 2 ETH in refunds (1 ETH from their own send, and 1 ETH stolen from `PanopticPool`'s balance). This drains any ETH held by the `PanopticPool` contract (e.g., stuck funds or accumulated dust).

## Impact
Theft of any ETH held by the PanopticPool contract.

## Command to Run Test


## Proof of Concept
1. Assume `PanopticPool` has 1 ETH balance (e.g., from a previous user accidentally calling `dispatchFrom` with value that wasn't consumed, or `selfdestruct`).
2. Attacker calls `PanopticPool.multicall` with 1 ETH.
3. The batch contains two calls to `dispatchFrom` targeting an account that results in a `bonus > 0` liquidation (or simply a force exercise/settlement where the logic flows to refund).
4. First `dispatchFrom`: Sends 1 ETH (msg.value) to `CollateralTracker`. `CollateralTracker` refunds 1 ETH to Attacker.
5. Second `dispatchFrom`: Sends 1 ETH (msg.value) from `PanopticPool`'s balance to `CollateralTracker`. `CollateralTracker` refunds 1 ETH to Attacker.
6. Attacker spent 1 ETH, received 2 ETH. `PanopticPool` lost 1 ETH.

## Proof of Code
function test_multicall_value_reuse() public {
    // Setup: Simulate stuck ETH in PanopticPool
    vm.deal(address(panopticPool), 1 ether);
    
    // Create a scenario where settleLiquidation refunds msg.value
    // This typically happens when bonus > 0. We can mock or setup a state.
    // For PoC simplicity, we assume we can trigger dispatchFrom such that ct.settleLiquidation is called.
    
    bytes[] memory calls = new bytes[](2);
    bytes memory callData = abi.encodeWithSelector(PanopticPool.dispatchFrom.selector, ...);
    calls[0] = callData;
    calls[1] = callData;
    
    uint256 balBefore = address(attacker).balance;
    panopticPool.multicall{value: 1 ether}(calls);
    uint256 balAfter = address(attacker).balance;
    
    // Attacker gains 1 ether (stolen from pool)
    assertEq(balAfter - balBefore, 1 ether);
}

## Suggested Mitigation
Override `multicall` in `PanopticPool` to disallow `msg.value` if multiple calls are present, or ensure `dispatchFrom` does not blindly pass `msg.value` in a loop. Alternatively, ensure `PanopticPool` never holds ETH by sweeping it, though the multicall loop issue remains for the duration of the tx.


## [H-36]. Silent overflow of unrealizedInterest in MarketStateLibrary causes permanent loss of protocol assets

## id: kiedKeYrHatGuHEUHXxHH

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
MarketStateLibrary.updateUnrealizedInterest

## Finding Status: LowSeverityDueToRareLikelihood
### Finding Status Justification: MarketState.unrealizedInterest is stored in 106 bits and MarketStateLibrary.updateUnrealizedInterest masks newInterest with (2^106-1) via `and(newInterest, max106)`, producing silent wrap/truncation. CollateralTracker.totalAssets() directly includes s_marketState.unrealizedInterest(); if it wraps downward, totalAssets() drops sharply and share price can drop, harming LPs. There is no require/cap to prevent the 106-bit overflow. While reaching 2^106 requires very large notional/long time, the root cause exists now and is not prevented by deposit limits (only per-deposit is capped; total can still grow) or by the uint128 accumulator (which allows far larger values than 2^106).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `MarketState` struct packs `unrealizedInterest` into 106 bits. The `MarketStateLibrary.updateUnrealizedInterest` function updates this value using assembly with a bitmask (`and(newInterest, max106)`). If the accumulated unrealized interest exceeds `2^106 - 1` (approximately 8.1e31), the value silently wraps around (modulo behavior) due to the masking. 

This wrap-around causes the `unrealizedInterest` recorded in the `CollateralTracker` to drop significantly (by multiples of 2^106). Since `unrealizedInterest` is a component of `totalAssets()`, this triggers an immediate and sharp decrease in the calculated `totalAssets()`, causing a drop in the share price. Existing liquidity providers suffer an immediate loss of value proportional to the overflow amount. 

For high-supply tokens (like SHIB, PEPE, or tokens with 18+ decimals) or pools with high utilization and long durations, reaching 8e31 in interest is practically achievable (e.g., 10% interest on a 1e32 pool supply).

## Impact
Permanent loss of funds for liquidity providers due to share price collapse when interest accumulator overflows.

## Command to Run Test


## Proof of Concept
1. Deploy a CollateralTracker for a high-supply token (e.g., 18 decimals, total supply > 1e32).
2. LPs deposit a large amount of liquidity (e.g., 5e31 assets).
3. Generate activity such that `unrealizedInterest` grows (high utilization borrow).
4. Wait or simulate time passage until `unrealizedInterest` + `newInterest` > 2^106 (approx 8.11e31).
5. Trigger `accrueInterest()`.
6. `updateUnrealizedInterest` is called. The new value exceeds 106 bits.
7. The assembly mask `and(newInterest, max106)` truncates the high bits, effectively resetting the interest accumulator to a small value.
8. `CollateralTracker.totalAssets()` decreases by ~8.11e31.
9. Share price drops instantly. LPs withdrawing now receive significantly less assets than they are entitled to.

## Proof of Code
    function testUnrealizedInterestOverflow() public {
        // Simulate MarketStateLibrary internal logic
        uint128 overflowInterest = uint128(2**106) + 100;
        
        // Store initial state (0 interest)
        MarketState state = MarketStateLibrary.storeMarketState(0, 0, 0, 0);
        
        // Update with overflow value
        state = MarketStateLibrary.updateUnrealizedInterest(state, overflowInterest);
        
        // Retrieve value
        uint128 storedInterest = state.unrealizedInterest();
        
        // Assert that the stored value is wrapped (100) instead of the actual value
        // This proves the silent overflow mechanism
        assertEq(storedInterest, 100);
        assertEq(storedInterest < overflowInterest, true);
    }

## Suggested Mitigation
In `MarketStateLibrary.updateUnrealizedInterest`, remove the silent masking behavior and instead `require(newInterest <= max106, "Interest Overflow");` or cap the interest at `max106` to prevent the catastrophic drop in `totalAssets`.


## [H-37]. DoS of Liquidation and Force Exercise due to `_internalSupply` Underflow in `CollateralTracker._burn`

## id: RS0vx9hNNDX2mmTJ-8rY3

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
Dos

## Location
CollateralTracker._burn

## Finding Status: LowSeverityDueToRareLikelihood
### Finding Status Justification: CollateralTracker.delegate inflates a user’s balance without increasing `_internalSupply`. During liquidation/force exercise/settlePremium, the protocol then performs burns (via settleBurn or _accrueInterest) against this inflated balance. ERC20Minimal._burn always decrements `_internalSupply` and will revert on underflow. Therefore, if the burn amount computed in shares exceeds `_internalSupply` (e.g., large settlement relative to real supply, or supply skew from creditedShares/rounding), liquidation/force exercise can revert and become impossible for that case. There is no explicit guard in delegate/settleBurn to ensure `_internalSupply` can support the burn amount, so this can brick critical bad-debt management in extreme conditions.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `CollateralTracker._burn` function strictly decrements `_internalSupply` whenever shares are burned. The `delegate` function, used during liquidations and force exercises to provide virtual shares to the liquidatee (to ensure solvency during settlement), increases the user's `balanceOf` but does explicitly *not* increase `_internalSupply`. 

When `_burn` is subsequently called (e.g., via `settleBurn` in `_burnOptions`), it attempts to burn shares corresponding to the user's debt. If the amount of shares to burn exceeds the current `_internalSupply` (which represents only the *real* assets in the pool), the subtraction `_internalSupply -= amount` will underflow and revert. 

This scenario is reachable if the debt being settled is large relative to the pool's current real supply (e.g., in a small pool, or if the user has a large position relative to others). This revert blocks the `liquidate` and `forceExercise` functions, effectively bricking the protocol's bad debt management mechanisms.

## Impact
Liquidation and force exercise functionality is permanently blocked for positions where the debt exceeds the total real share supply, leading to accumulation of bad debt and potential protocol insolvency.

## Command to Run Test


## Proof of Concept
1. Setup a `CollateralTracker` with a small `_internalSupply` (e.g., 1 wei of shares). 
2. Attacker creates a large debt position (possible via flash loans or accumulated fees) such that the shares equivalent of the debt > 1. 
3. User becomes insolvent or eligible for force exercise. 
4. Liquidator calls `PanopticPool.liquidate()`. 
5. `liquidate` calls `CollateralTracker.delegate(liquidatee)`, giving them virtual shares. 
6. `liquidate` calls `_burnOptions`, which calls `CollateralTracker.settleBurn`. 
7. `settleBurn` calls `_burn(liquidatee, debtShares)`. 
8. `_burn` attempts `_internalSupply -= debtShares`. Since `debtShares > _internalSupply`, this reverts. 
9. Liquidation fails.

## Proof of Code
function test_LiquidationDoS_Underflow() public {
    // Setup: Create a small pool supply
    // ... (Assume pool has 100 shares)
    // User creates a position that incurs a debt of 200 shares worth of assets
    // Liquidator tries to liquidate
    vm.expectRevert(stdError.arithmeticError);
    pool.liquidate(user, ...);
}

## Suggested Mitigation
Modify `CollateralTracker._burn` to handle virtual shares or `_internalSupply` updates differently. For example, `delegate` could track the amount of virtual supply added, or `_burn` could be modified to not decrement `_internalSupply` if the burn amount exceeds it (implying virtual shares are being burned) or check against a `virtualSupply` variable.





Finding Status: InvalidERC20EdgeCase
## [H-38]. Reentrancy in settleLiquidation allows theft via phantom share withdrawal

## id: eTHeH3P7VwWcExKyfPtXw

## Derived From Pattern/Invariant
ERC777HookReentrancy

## Exploit Type
Reentrancy

## Location
CollateralTracker.settleLiquidation

## Finding Status: InvalidERC20EdgeCase
### Finding Status Justification: Code path exists: PanopticPool._liquidate delegates phantom shares via CollateralTracker.delegate(liquidatee), then calls CollateralTracker.settleLiquidation. In settleLiquidation, the bonus<0 branch performs an external SafeTransferLib.safeTransferFrom(underlyingToken, liquidator, msg.sender, bonusAbs) before revoking phantom shares (the revoke-like logic at the end). With a hook-capable token (e.g., ERC777-style), the liquidator can reenter CollateralTracker.withdraw during transferFrom. Since delegation inflated balanceOf, maxWithdraw() becomes bounded only by s_depositedAssets-1, enabling withdrawal of real assets while burning “phantom” shares. After reentrancy, settleLiquidation’s phantom-revocation logic restores _internalSupply by adding (type(uint248).max - liquidateeBalance), effectively undoing the burn while assets have already left, diluting LPs and enabling draining. There is no nonReentrant guard and no CEI ordering that revokes before external calls.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
During liquidation (`PanopticPool._liquidate`), the protocol temporarily delegates 'phantom' shares to the liquidatee (via `CollateralTracker.delegate`) to facilitate settlement. The function then calls `CollateralTracker.settleLiquidation`. If `bonus < 0` (protocol loss mitigation), this function transfers tokens from the liquidator using `SafeTransferLib.safeTransferFrom`. If the underlying token has transfer hooks (e.g., ERC777), the liquidator can reenter `CollateralTracker.withdraw`. Since the liquidatee (who can be the liquidator) holds phantom shares, the withdrawal succeeds, burning the phantom shares and paying out real assets. Upon return to `settleLiquidation`, the logic detects the missing shares and incorrectly inflates `_internalSupply` to compensate, breaking the share price invariant and enabling massive theft.

## Impact
Direct theft of vault assets and permanent bricking of the pool via supply inflation.

## Command to Run Test


## Proof of Concept
1. Attacker opens a position using an ERC777-compatible token (or one with callbacks) in a way that creates a deficit in that token and surplus in the other (cross-collateral).
2. Attacker renders their account insolvent.
3. Attacker calls `PanopticPool.dispatchFrom` to liquidate themselves (or via a second account).
4. In `_liquidate`, `CollateralTracker.delegate` gives the attacker phantom shares.
5. `settleLiquidation` is called. The `bonus < 0` branch executes `safeTransferFrom` to collect tokens from the attacker.
6. The token hook triggers. Attacker calls `CollateralTracker.withdraw`.
7. `withdraw` burns the phantom shares and sends real assets to the attacker.
8. The hook returns. `settleLiquidation` proceeds to the 'revoke' logic. It sees the user balance is zero (less than the phantom amount) and adds the difference to `_internalSupply`, massively inflating it.

## Proof of Code
function testExploitPhantomShares() public {
    // Setup malicious token and pool
    // ... (setup code) ...
    // Trigger liquidation with reentrancy hook
    vm.prank(attacker);
    panopticPool.dispatchFrom(..., attacker, ...);
    // Assert assets stolen and supply inflated
}

## Suggested Mitigation
Add `nonReentrant` modifier to `CollateralTracker.settleLiquidation` and `withdraw`, or ensure `revoke` logic runs before any external calls in `settleLiquidation`.





Finding Status: InvalidOutOfScope
## [H-39]. Permanent Accounting Invariant Violation and Bad Debt Accumulation due to Insolvency Masking in RiskEngine

## id: Ute6xuqi4rNR8SWXkIsT-

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
RiskEngine.getMargin

## Finding Status: InvalidOutOfScope
### Finding Status Justification: The behavior is explicitly flagged as a known pre-contest issue and thus out of scope, but it is not "by design" in the sense of an intended/desired invariant; it is a known accounting limitation/bug accepted for this contest.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `RiskEngine.getMargin` function artificially caps the interest requirement at the user's available balance when the user is insolvent (interest owed > balance). This logic masks the true magnitude of the deficit. Consequently, when `PanopticPool._liquidate` calls `RiskEngine.getLiquidationBonus`, the protocol loss is calculated based on this masked deficit (0 or small) rather than the actual bad debt. Furthermore, the `CollateralTracker` does not write off the unpaid interest from the global `s_marketState.unrealizedInterest` accumulator during liquidation. 

Snippet from `RiskEngine.sol`:
```solidity
if (interest0 > balance0) {
    interest0 = balance0; // Cap interest
    balance0 = 0; // Zero balance
} ...
tokensRequired = tokensRequired.addToRightSlot(uint128(interest0));
```
Because `unrealizedInterest` is a component of `totalAssets()`, the protocol continues to report assets that simply do not exist (the unpaid interest from the liquidated user). This inflates the share price, allowing early withdrawers to exit with more value than they should, leaving the last LPs to bear the entire loss (insolvency/bank run).

## Impact
The `totalAssets` of the `CollateralTracker` becomes permanently inflated by the amount of unpaid interest from liquidated users. This leads to an incorrect share price, causing a loss of funds for remaining liquidity providers and potentially breaking the vault's solvency.

## Command to Run Test


## Proof of Concept
1. Attacker (or victim) deposits a small amount of collateral into `CollateralTracker`. 
2. They open a large short position that accrues significant interest over time (or utilization is manipulated to spike interest). 
3. The interest owed grows larger than their collateral balance. 
4. A liquidator calls `dispatchFrom` to liquidate the account. 
5. `RiskEngine.getMargin` caps the interest requirement to the user's balance, reporting a net deficit of only the collateral amount (effectively zeroing out the excess debt). 
6. `PanopticPool` liquidates the user, seizing their collateral. 
7. The `CollateralTracker` does not reduce `unrealizedGlobalInterest` by the unpaid portion of the debt. 
8. The unpaid debt remains in `totalAssets()`, keeping the share price artificially high. 
9. The attacker (using a separate LP account) withdraws their liquidity at the inflated price, stealing value from other LPs.

## Proof of Code
/* 
Add this test to a Foundry test file (e.g. BadDebt.t.sol) inheriting from Panoptic test setup 
*/
function testBadDebtAccumulation() public {
    // Setup: User1 and User2 deposit
    vm.startPrank(user1);
    collateralTracker.deposit(100 ether, user1);
    vm.stopPrank();
    
    vm.startPrank(user2);
    collateralTracker.deposit(100 ether, user2);
    vm.stopPrank();

    // User1 opens a position that will accrue interest
    // ... (omitted: setup short position for user1) ...
    
    // Fast forward time / manipulate utilization to accrue huge interest
    // Assume user1 owes 150 ether interest, has 100 ether balance
    // Deficit is 50 ether
    
    // Liquidate User1
    vm.prank(liquidator);
    panopticPool.dispatchFrom(..., user1, ...);

    // Check totalAssets
    uint256 totalAssets = collateralTracker.totalAssets();
    uint256 realAssets = token.balanceOf(address(panopticPool)); // simplified
    
    // totalAssets should equal realAssets, but it will be higher by ~50 ether
    assertGt(totalAssets, realAssets + 1 ether);
}

## Suggested Mitigation
1. Modify `RiskEngine.getMargin` to report the full interest requirement even if it exceeds the balance, ensuring the true deficit is visible. 
2. Update `CollateralTracker` to include a mechanism (callable during liquidation) that writes off bad debt by reducing `s_marketState.unrealizedInterest` when a user is fully liquidated and their collateral is insufficient to cover the accrued interest.


## [H-40]. LP Dilution via Phantom Share Interest Payment Exploit

## id: iGe_mjh7bVJ74dthsFW2N

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
CollateralTracker.delegate/revoke

## Finding Status: InvalidOutOfScope
### Finding Status Justification: The described mechanism (delegate inflates balance, _accrueInterest burns shares, revoke restores _internalSupply when phantom shares were consumed) matches the actual CollateralTracker.delegate/revoke and _accrueInterest behavior. However, this family of issues is explicitly listed as OUT OF SCOPE in the prompt’s “Additional Findings from Nethermind pre-contest” (Orphan Shares in Delegate/Revoke and related interest/solvency masking items). As such, despite being a real accounting problem in this codebase, it is excluded from consideration for this audit scope.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Users can wipe out their interest debt at the expense of other Liquidity Providers (LPs) by leveraging the `delegate` and `revoke` mechanism in `CollateralTracker`. When a user owes more interest than their share balance, the `delegate` function increases their balance to `type(uint248).max` (phantom shares). Subsequently, `_accrueInterest` burns the required interest amount from this inflated balance, effectively marking the debt as paid in `s_interestState`. However, `revoke` restores the `_internalSupply` by adding back the burned phantom amount. This sequence results in the protocol 'printing' shares to pay the user's interest debt, while the user only loses their real share balance (which is less than the debt). The net result is that the global `totalSupply` is restored (as if no interest was paid), but the global `unrealizedInterest` (debt) is reduced, causing a drop in `totalAssets` relative to `totalSupply` and thus diluting the share price for all other LPs.

## Impact
Users can erase large interest debts for free (minus their small share balance), causing direct loss of value to all other Panoptic Liquidity Providers.

## Command to Run Test


## Proof of Concept
1. Attacker accumulates a large interest debt (e.g., 100 shares) but maintains a low share balance (e.g., 2 shares) while remaining solvent via Long Option value.
2. Attacker triggers a flow that calls `delegate` (e.g., via `settlePremium` or `liquidate`).
3. `delegate` sets Attacker's balance to `Max`.
4. `_accrueInterest` burns 100 shares from Attacker's phantom balance and updates their borrow index (debt paid).
5. `revoke` observes `Max > Balance` (since `Balance = Max - 100`) and restores `100` shares to `_internalSupply`.
6. Result: Attacker's debt of 100 is wiped. Attacker lost only 2 real shares. Protocol supply is restored, but `unrealizedInterest` asset is removed. Share price drops.

## Proof of Code
function testExploitPhantomInterest() public {
    // Setup user with debt > balance
    // ... (omitted setup for brevity)
    // Assert debt paid but supply restored
}

## Suggested Mitigation
In `revoke`, track the amount of *real* shares that should have been burned (min(interest, realBalance)) and ensure `_internalSupply` is reduced by the amount of interest effectively forgiven or track the forgiven debt to handle it correctly (e.g., create a bad debt deficit).


## [M-41]. Permanent share supply inflation due to incorrect phantom share tracking in delegate/revoke flow

## id: Ifwr3wIv53sJg8LbvuYLe

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
CollateralTracker.sol.revoke

## Finding Status: InvalidOutOfScope
### Finding Status Justification: Code path exists in CollateralTracker.delegate/revoke plus CollateralTracker._accrueInterest. delegate() adds type(uint248).max minus balanceConsumedByInterest, but _accrueInterest burns shares based on the *post-delegation* balance, so it can burn phantom shares when interestShares > the user’s real shares. revoke() then restores _internalSupply by (type(uint248).max - balance), netting out the burn and leaving totalSupply higher than it should be relative to real-burn expectation, matching the described orphan/phantom-share accounting drift. This is materially the same issue as the explicitly listed Nethermind pre-contest out-of-scope item “Orphan Shares in Delegate/Revoke”, so it is out of scope here, even though the bug is real and triggerable in today’s code.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `delegate` function in `CollateralTracker.sol` inflates a user's balance to `type(uint248).max` to allow settlement of positions. However, it reduces the delegation amount by `balanceConsumedByInterest` (min(interest, balance)) to prevent virtual shares from paying interest. If a user is insolvent (interest > balance), `delegate` adds `max - balance` to their account. Subsequently, `_accrueInterest` burns the user's entire balance (real shares) to pay debt. Finally, `revoke` calculates the phantom shares to restore to `_internalSupply` as `max - balance`. Since the balance is 0 (due to interest burn), `revoke` adds `max` to `_internalSupply`. The net result is that the user's real shares were burned, but `_internalSupply` was increased by the amount of those real shares (plus the phantom amount), inflating the total supply relative to the actual assets/balances. This permanently disconnects `totalSupply` from `sum(balances)`, diluting all share holders.

## Impact
Permanent inflation of `totalSupply` leading to share price dilution/deflation. Effectively destroys value for all Liquidity Providers.

## Command to Run Test


## Proof of Concept
1. User has balance 10. User owes 20 interest.
2. `delegate(user)` is triggered (e.g. via forceExercise on insolvent user). 
3. `delegate` calculates `balanceConsumed` = 10. Adds `max - 10` to user. User balance = `max`.
4. `_accrueInterest` burns 20 shares (10 real + 10 virtual). User balance = `max - 20`.
5. `revoke(user)` called. Checks `max > balance`.
6. `_internalSupply += max - (max - 20) = 20`.
7. Net Change: User balance -10 (correct). `totalSupply` change: -20 (burn) + 20 (restore) = 0.
8. Expected `totalSupply` change: -10. Actual: 0. Supply is inflated by 10.

## Proof of Code
        // Insert in a test file importing CollateralTracker
        function test_InflationAttack() public {
            // Setup: User has 10 shares, Owes 20 interest
            // Mock state where user is insolvent
            
            // Simulate delegate
            uint256 max = type(uint248).max;
            uint256 balance = 10;
            uint256 interest = 20;
            uint256 consumed = 10;
            
            // delegate()
            uint256 delegatedBalance = balance + (max - consumed);
            assertEq(delegatedBalance, max);
            
            // accrueInterest()
            uint256 burned = interest;
            uint256 finalBalance = delegatedBalance - burned;
            
            // revoke()
            uint256 restore = max - finalBalance;
            
            // Net Supply Change = -burned + restore
            int256 netSupply = -int256(burned) + int256(restore);
            
            // We expect netSupply to be -10 (the user's real balance burned)
            // But it is 0
            assertEq(netSupply, 0);
        }

## Suggested Mitigation
In `delegate`, simply add `type(uint248).max` without subtracting `balanceConsumedByInterest`. Or, update `revoke` logic to track exactly how much was added during `delegate`.


## [H-42]. Interest Evasion via self-Force-Exercise and Revoke

## id: BleGuMzC6BP0eAgAJLa2p

## Derived From Pattern/Invariant
Phantom shares from delegation allow evading interest payments

## Exploit Type
AccountingInvariantViolation

## Location
CollateralTracker.revoke

## Finding Status: InvalidOutOfScope
### Finding Status Justification: The code path exists: PanopticPool._forceExercise calls ct0/ct1.delegate(account) before _burnOptions; CollateralTracker.settleBurn -> _updateBalancesAndSettle calls _accrueInterest(optionOwner), which can burn shares while the account has delegated phantom balance. CollateralTracker.revoke then restores _internalSupply when balance < type(uint248).max, effectively undoing phantom-share burns. This matches the known “delegate/revoke creates orphan shares / phantom-share interest payment” behavior documented as out-of-scope in the prompt’s Nethermind pre-contest list (Orphan Shares in Delegate/Revoke / related interest handling). Thus, although the behavior is real and exploitable in principle, it is explicitly out of scope for this audit.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Users can evade paying accrued interest by triggering a force exercise (or having a sybil do it) on their own positions. When `_forceExercise` is called, `CollateralTracker.delegate` increases the user's balance by `type(uint248).max` (phantom shares). Subsequently, `settleBurn` calls `_accrueInterest`, which burns shares from the user to pay interest. Because the user has a massive phantom balance, the burn succeeds, and the user's borrow index is updated to mark the interest as paid. Finally, `revoke` is called. `revoke` detects that the balance is less than `type(uint248).max` (due to the interest burn) and adds the difference to `_internalSupply` to restore the total supply invariant. This effectively undoes the supply reduction from the interest payment. The result is that the user's debt is cleared (index updated) but the protocol/LPs receive no value (supply restored, no assets added), constituting theft of yield.

## Impact
Users can wipe their interest debt without paying, leading to a loss of yield for Panoptic Liquidity Providers.

## Command to Run Test


## Proof of Concept
1. User has a position and owes significant interest. 2. User (or sybil) calls `dispatchFrom` to force exercise the position. 3. `CollateralTracker.delegate` gives user `MAX` shares. 4. `settleBurn` -> `_accrueInterest` burns `X` shares for interest. User balance becomes `MAX - X`. User index updated. 5. `CollateralTracker.revoke` sees balance `MAX - X`. It sets balance to 0 and adds `X` to `_internalSupply`. 6. `totalSupply` increases by `X`, cancelling the previous burn. 7. Net result: User debt cleared, Share price (Assets/Supply) unchanged (interest not paid).

## Proof of Code
function test_InterestEvasion() public {
    // Setup position and accrue interest
    // ...
    uint256 preSupply = ct.totalSupply();
    uint256 preAssets = ct.totalAssets();
    
    // Force exercise self
    panopticPool.dispatchFrom(user, user, ...);
    
    // Assert borrow index updated but share price/supply indicates no payment
    assertEq(ct.totalSupply(), preSupply);
    assertEq(ct.totalAssets(), preAssets);
    // But user debt is gone
    assertEq(ct.owedInterest(user), 0);
}

## Suggested Mitigation
In `revoke`, do not restore `_internalSupply` for shares burned by `_accrueInterest`. Alternatively, ensure `_accrueInterest` does not run during the delegation period, or track the specific amount delegated and only revoke that exact amount.


## [M-43]. Liquidation DoS due to priority of interest deduction over liquidation bonus

## id: 2CRE8RDF1oU6io-ysv5ZI

## Derived From Pattern/Invariant
Incentive / Game Theory (Griefing DoS)

## Exploit Type
Dos

## Location
RiskEngine.getLiquidationBonus

## Finding Status: InvalidOutOfScope
### Finding Status Justification: While explicitly listed as out-of-scope (masking insolvency magnitude / broken bonus calculations), the impact is not necessarily low: it can materially reduce/zero liquidator incentives for interest-heavy insolvent accounts, risking liquidation inaction.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `RiskEngine.sol`, the `getLiquidationBonus` function calculates the liquidator's reward using the user's available collateral balance. This balance is derived from `_getMargin`, which explicitly caps the interest owed at the user's total balance and sets the available balance to zero if the user owes more interest than they possess (`if (interest0 > balance0) { interest0 = balance0; balance0 = 0; }`). 

Consequently, if an insolvent account's debt is primarily composed of accrued interest such that `interest > assets`, `getLiquidationBonus` sees an available balance of 0 and calculates a bonus of 0. 

A liquidator calling `liquidate` in `PanopticPool` would incur gas costs (potentially high due to multiple AMM calls to close positions) but receive 0 reward. Rational actors will therefore decline to liquidate these accounts. As a result, these insolvent positions remain open, accumulating further bad debt and protocol risk indefinitely.

## Impact
Insolvent accounts with high interest debt are not liquidated, leading to accumulation of bad debt and potential protocol insolvency as positions remain open.

## Command to Run Test


## Proof of Concept
1. User opens a short option position.
2. Time passes, and the position accumulates significant interest debt such that `interestOwed > userCollateralBalance`.
3. The user is insolvent via `isAccountSolvent`.
4. A liquidator attempts to calculate profitability of liquidation.
5. `RiskEngine.getMargin` sets `availableBalance` to 0 because all collateral is theoretically consumed by interest.
6. `RiskEngine.getLiquidationBonus` computes bonus based on 0 balance, returning 0.
7. Liquidator observes 0 reward for a gas-intensive operation and chooses not to execute `liquidate`.
8. The bad debt remains on the protocol.

## Proof of Code
function testLiquidationDoS() public {
    // Setup: Create short position
    // ... (mint option) ...
    
    // Simulate massive time passing to accrue interest > collateral
    vm.warp(block.timestamp + 3650 days);
    
    // Verify insolvency
    bool solvent = riskEngine.isAccountSolvent(...);
    assertFalse(solvent);
    
    // Check liquidation bonus
    (LeftRightSigned bonus, ) = riskEngine.getLiquidationBonus(...);
    
    // Assert bonus is zero despite collateral existing (it's just reserved for interest)
    assertEq(bonus.rightSlot(), 0);
    assertEq(bonus.leftSlot(), 0);
}

## Suggested Mitigation
Adjust the liquidation logic to allow a minimum portion of the remaining collateral to be used as a liquidation bonus, even if it cuts into the interest payment owed to the protocol. This ensures liquidators are incentivized to close bad debt positions.


## [H-44]. Solvency Check Bypass via Insolvency Masking in Margin Calculation

## id: IZJfAErUENAMWLFxEr7yT

## Derived From Pattern/Invariant
AccountingInvariantViolation

## Exploit Type
AccountingInvariantViolation

## Location
RiskEngine._getMargin

## Finding Status: InvalidOutOfScope
### Finding Status Justification: This is explicitly out-of-scope per the pre-contest 'Masking Insolvency Magnitude' note, but the impact characterization as low is not correct: understatement of deficits can allow materially undercollateralized behavior (cross-collateral solvency passing).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `RiskEngine._getMargin` function calculates the collateral status of an account. It retrieves the user's assets and accrued interest from `CollateralTracker`. The logic includes a conditional check: if `interest > balance`, it caps the interest deduction at `balance` and sets `balance` to 0. 

```solidity
if (interest0 > balance0) {
    interest0 = balance0; // Cap interest
    balance0 = 0; // Zero balance
} ...
```

It then adds this capped `interest0` to the `tokensRequired`. This effectively hides the magnitude of the insolvency. For example, if a user owes 100 in interest but has only 10 in balance, the system calculates the deficit as 10 (req) - 0 (bal) = 10, instead of the actual 90. 

This becomes critical in `isAccountSolvent`, which uses cross-collateralization. A deficit in one token is compared against the surplus in another. Because the deficit is significantly understated due to masking, a small surplus in the second token can incorrectly make the account appear solvent. This allows insolvent users to withdraw collateral or open new positions, increasing bad debt.

## Impact
Protocol bad debt. Insolvent users can bypass liquidation checks, withdraw remaining collateral from other tokens, or open new positions while deeply under-collateralized.

## Command to Run Test


## Proof of Concept
1. User deposits 10 USDC and 100 DAI (assume 1 DAI = 0.5 USDC). Total Value: $60.
2. User opens positions and accrues 100 USDC in interest debt. Real Net Value: 10 USDC - 100 Debt + 100 DAI = -40 USD (Insolvent).
3. `_getMargin` for USDC sees `balance` (10) < `interest` (100). Sets `req=10`, `bal=0`.
4. `isAccountSolvent` checks USDC solvency: `bal (0) + convert(DAI_Surplus) >= req (10)`.
5. DAI Surplus is 100 DAI ($50). Converted to USDC = 100 units (approx).
6. `0 + 100 >= 10`. Evaluates to TRUE.
7. User is deemed solvent despite being $40 underwater.
8. User calls `withdraw` to remove DAI, leaving the protocol with the USDC bad debt.

## Proof of Code
function testMaskingInsolvency() public {
    // Setup account with small balance token0, large surplus token1, huge debt token0
    // ...
    bool isSolvent = riskEngine.isAccountSolvent(...);
    assertTrue(isSolvent, "Should appear solvent due to masking");
    // Assert real health is negative
}

## Suggested Mitigation
In `RiskEngine._getMargin`, accurately report the full interest requirement even if it exceeds the user's balance. Do not cap `interest0` at `balance0`. Instead, if `interest > balance`, set `balance` to 0 and add the *full* `interest` amount to `tokensRequired`. This ensures the deficit is correctly reflected in solvency calculations.





Finding Status: InvalidGovernanceRisk
## [M-45]. Initialization Revert for Native Pools due to Logarithm of Zero

## id: 28A4OIIlYbn04qJ0YGfTn

## Derived From Pattern/Invariant
UnexpectedEth

## Exploit Type
StandardViolation

## Location
SemiFungiblePositionManager.initializeAMMPool

## Finding Status: InvalidGovernanceRisk
### Finding Status Justification: In `initializeAMMPool`, if `currency0` is native (`address(0)`), the code passes `NATIVE_ENFORCED_TICKFILL_COST` directly into `Math.getApproxTickWithMaxAmount`. If this immutable is set to `0`, then `getApproxTickWithMaxAmount(0, ...)` passes `argX128=0` into `Math.log_Sqrt1p0001MantissaRect`, which calls `FixedPointMathLib.log2(0)` and reverts. This makes native pools un-initializable (hard DoS) under that deployment configuration. The issue is primarily a configuration/governance risk because the constructor argument can be chosen non-zero; however, the provided deployment snippet sets it to 0, making it a practical present-day bug for that deployment.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `initializeAMMPool` function determines the `min/maxEnforcedTick` by calling `Math.getApproxTickWithMaxAmount`. For native token pools (`Currency.unwrap(key.currency0) == address(0)`), it uses `NATIVE_ENFORCED_TICKFILL_COST` as the amount. 

In the provided deployment configuration, `NATIVE_ENFORCED_TICKFILL_COST` is set to `0`. Passing `0` as the `amount` to `Math.getApproxTickWithMaxAmount` triggers a call to `Math.log_Sqrt1p0001MantissaRect(0, ...)` which eventually calls `FixedPointMathLib.log2(0)`. In the Solady library used, `log2(0)` reverts (is undefined). 

This causes `initializeAMMPool` to always revert for any pool involving the native asset (ETH), permanently preventing their creation and usage.

## Impact
Medium. Complete Denial of Service for native token pools (ETH pairs) preventing them from ever being initialized.

## Command to Run Test


## Proof of Concept
1. Deploy `SemiFungiblePositionManager` with `NATIVE_ENFORCED_TICKFILL_COST = 0`.
2. Attempt to call `initializeAMMPool` with a `PoolKey` where `currency0` is `address(0)` (Native).
3. The logic selects `NATIVE_ENFORCED_TICKFILL_COST` (0) as the amount.
4. `Math.getApproxTickWithMaxAmount(0, ...)` is called.
5. Internal math calls `log2(0)`, which reverts.
6. Initialization fails.

## Proof of Code
function test_NativePool_Revert() public {
    SemiFungiblePositionManager sfpm = new SemiFungiblePositionManager(manager, 10**13, 0, 0);
    PoolKey memory key = PoolKey(Currency.wrap(address(0)), currency1, 3000, 60, IHooks(address(0)));
    vm.expectRevert();
    sfpm.initializeAMMPool(key, 0);
}

## Suggested Mitigation
Ensure `NATIVE_ENFORCED_TICKFILL_COST` is set to a non-zero value in the constructor or add a check to handle 0 by setting a default minimum.





Finding Status: InvalidSafeGuardInPlace
## [H-46]. Forced Exercise Fee Manipulation via Flash Loan allowing cheap destruction of OTM/ITM positions

## id: bsizQ38XBVs-DfmzY8R90

## Derived From Pattern/Invariant
FlashLoanEconomicManipulation

## Exploit Type
FlashLoanEconomicManipulation

## Location
PanopticPool._forceExercise

## Finding Status: InvalidSafeGuardInPlace
### Finding Status Justification: RiskEngine.exerciseCost uses currentTick (spot) to set hasLegsInRange (thus choosing FORCE_EXERCISE_COST vs ONE_BPS), and PanopticPool._forceExercise passes the currentTick from SFPM as the first argument. PanopticPool.dispatchFrom enforces a manipulation bound (abs(currentTick - twapTick) <= tickDeltaLiquidation, 513 ticks), which is a partial safeguard, but it still allows an attacker to move the spot within that band in a single transaction and potentially flip near-boundary legs from in-range to out-of-range, materially reducing the paid fee. This is an economic/griefing vector rather than direct protocol asset theft, hence Medium impact.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `PanopticPool._forceExercise` function allows a user to force another user to exercise their position. The fee paid by the exercisor to the victim is calculated in `RiskEngine.exerciseCost`. This calculation relies on `currentTick` (the spot price) to determine if the position's legs are 'in-range' or 'out-of-range'. 

If legs are in-range, the fee is high (`FORCE_EXERCISE_COST` ~10%). If out-of-range, the fee is low (`ONE_BPS` ~0.01%). 

An attacker can use a flash loan to manipulate the Uniswap pool price (`currentTick`) such that a victim's In-The-Money (ITM) or Near-The-Money position appears Out-Of-The-Money (OTM) / out-of-range. This drops the forced exercise fee from ~10% to ~0.01%.

Simultaneously, `_forceExercise` calls `_burnOptions`, which settles the position at the `currentTick`. Since the price is manipulated to be OTM, the intrinsic value returned to the victim is minimized (likely zero for options, returning only collateral). 

The attacker thus forces the victim to close their position at a highly unfavorable price while paying a negligible fee, effectively destroying the value of the victim's position for cheap. Although `dispatchFrom` has a check that `currentTick` is within ~5% of `twapTick`, a 5% deviation is often sufficient to push Near-The-Money positions out of range.

## Impact
An attacker can destroy the value of other users' positions (griefing) or manipulate open interest cheaply. Victims lose the time value and potentially the intrinsic value of their options.

## Command to Run Test


## Proof of Concept
1. Bob holds a Long Call position that is slightly In-The-Money (ITM) or At-The-Money (ATM). 
2. Alice (attacker) observes this. 
3. Alice takes a flash loan and swaps in the Uniswap pool to push the price down by 4-5% (within the `MAX_TWAP_DELTA_LIQUIDATION` limit of 513 ticks), making Bob's position appear Out-Of-The-Money (OTM). 
4. Alice calls `PanopticPool.dispatchFrom` with the intent to force exercise Bob's position. 
5. `PanopticPool` calls `RiskEngine.exerciseCost(currentTick, ...)` using the manipulated spot price. 
6. `RiskEngine` calculates `hasLegsInRange` as `false` because the price is now outside the leg's range. The fee is set to `ONE_BPS` (0.01%). 
7. `PanopticPool` executes the burn. `SFPM` burns the position at the manipulated OTM price, returning minimal value to Bob. 
8. Alice pays the small fee. Bob's position is closed at a loss. 
9. Alice reverses the flash loan swap.

## Proof of Code
function testForceExerciseFeeManipulation() public {
    // Setup: User has a Long Call position near the money
    uint256 positionSize = 10 ether;
    TokenId tokenId = mintCallOption(bob, positionSize, currentTick);

    // Verify fee is high initially (ITM/ATM)
    int256 feeNormal = riskEngine.exerciseCost(currentTick, twapTick, tokenId, balance);
    assertGt(abs(feeNormal), 1e16); // Expect ~10% fee

    // Attack: Manipulate price OTM (but within 5% TWAP check)
    vm.startPrank(attacker);
    int24 manipulatedTick = currentTick - 500; // ~5% move
    mockUniswapPool.setTick(manipulatedTick);

    // Verify fee is now low
    int256 feeManipulated = riskEngine.exerciseCost(manipulatedTick, twapTick, tokenId, balance);
    assertLt(abs(feeManipulated), 1e15); // Expect ~0.01% fee

    // Execute Force Exercise
    panopticPool.dispatchFrom(..., bob, ...);
    
    // Bob's position is gone, he received almost no value and tiny fee
    vm.stopPrank();
}

## Suggested Mitigation
In `RiskEngine.exerciseCost`, use `oracleTick` (TWAP) instead of `currentTick` to determine if `hasLegsInRange` and to calculate the base fee tier. This prevents attackers from manipulating the spot price to lower the exercise fee.





Finding Status: LowSeverityDueToRareLikelihood + InvalidNotExploitable
## [M-47]. Loss of yield and accounting corruption due to `unrealizedInterest` overflow in `MarketState` for high-supply tokens

## id: 6DJOY2PY36aFKlPZLV-aq

## Derived From Pattern/Invariant
Integer Overflow

## Exploit Type
IntegerOverflow

## Location
CollateralTracker._accrueInterest

## Finding Status: LowSeverityDueToRareLikelihood + InvalidNotExploitable
### Finding Status Justification: `MarketState` packs `unrealizedInterest` into 106 bits (bits 150-255). `MarketStateLibrary.updateUnrealizedInterest` masks inputs to 106 bits, and `storeMarketState` does not enforce a bound; if `_unrealizedInterest` exceeds 2^106-1 (but still fits in the uint128 accumulator used in `_calculateCurrentInterestState`), the high bits will be truncated when shifted into the packed slot, silently corrupting the stored value. This can make `totalAssets()` drop (since it includes `s_marketState.unrealizedInterest()`), breaking the “share price non-decreasing” accounting expectation. Practically, reaching 2^106 requires extreme scale/time (deposit cap is ~2^104), so exploitation is unlikely and more a long-horizon safety/robustness issue than an attacker-driven drain.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `MarketState` struct in `CollateralTracker.sol` packs `unrealizedInterest` into 106 bits. This accumulator tracks the total interest accrued by borrowers but not yet distributed/paid. 2^106 is approximately 8.1e31. For tokens with high total supplies (e.g., SHIB, PEPE, BONK, LUNC) or even standard tokens with high decimals and large pool usage, the accumulated interest can exceed this limit. Since `MarketStateLibrary.storeMarketState` uses bit-shifting without overflow checks for the packed fields, and `MarketStateLibrary.updateUnrealizedInterest` explicitly masks the input to 106 bits, an overflow will result in a silent wrap-around (truncation) of the `unrealizedInterest` value.

When this occurs, `totalAssets()` (which includes `unrealizedInterest`) will abruptly decrease, causing the share price (`totalAssets`/`totalSupply`) to drop. This results in a direct loss of value for all passive Panoptic Liquidity Providers (PLPs) holding shares in the `CollateralTracker`. Additionally, it breaks the invariant that `totalAssets` must accurately reflect the system's solvency.

## Impact
Accounting corruption leading to loss of accrued interest yield for LPs and a decrease in share price.

## Command to Run Test


## Proof of Concept
1. Deploy a CollateralTracker for a high-supply token (e.g., 1e30 supply).
2. Users deposit large amounts, and borrowers utilize significant liquidity (`s_assetsInAMM` is high).
3. High utilization triggers a high interest rate via the Adaptive Interest Rate Model.
4. Over time (or quickly if amounts are large), `_accrueInterest` calculates new interest that pushes `unrealizedGlobalInterest` beyond 2^106 - 1.
5. `MarketStateLibrary.storeMarketState` is called, causing the higher bits of `unrealizedInterest` to be discarded or overwrite adjacent fields.
6. `totalAssets()` returns a value lower than actual, causing the share price to drop. LPs withdrawing at this point suffer a loss.

## Proof of Code
function test_UnrealizedInterestOverflow() public {
    // Setup MarketState with near-max unrealized interest
    uint128 highInterest = uint128(2**106 - 1);
    MarketState state = MarketStateLibrary.storeMarketState(1e18, 0, 0, highInterest);
    
    // Simulate adding small interest
    uint128 newInterest = 100;
    uint128 overflowed = highInterest + newInterest;
    
    // Update state
    state = state.updateUnrealizedInterest(overflowed);
    
    // Check value
    uint128 stored = state.unrealizedInterest();
    
    // stored will be (overflowed & mask), much smaller than overflowed
    assertLt(stored, highInterest);
}

## Suggested Mitigation
Increase the size of the `unrealizedInterest` field in `MarketState` by reducing the precision or size of other fields (e.g., `borrowIndex` or `rateAtTarget`), or use a separate storage slot for `unrealizedInterest` to allow it to utilize the full `uint256` or `uint128` range.


## [M-48]. Protocol bricking due to borrowIndex overflow corrupting MarketState storage

## id: 5Ru0EnF1zHLPDpHKqCbDt

## Derived From Pattern/Invariant
IntegerOverflow

## Exploit Type
IntegerOverflow

## Location
CollateralTracker._accrueInterest

## Finding Status: LowSeverityDueToRareLikelihood + InvalidNotExploitable
### Finding Status Justification: This is not dependent on a hypothetical upgrade or future code path: MarketStateLibrary.storeMarketState currently does not mask _borrowIndex to 80 bits, so state corruption is a real long-horizon risk in the current code. The main mitigating factor is time/scale (i.e., low likelihood), not ‘future speculation’ in the sense of requiring a future code change.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `MarketState` struct tightly packs `borrowIndex` (uint80) and `marketEpoch` (uint32) into a single `uint256` slot. The `CollateralTracker._accrueInterest` function calculates `currentBorrowIndex` as a `uint128` allowing it to grow beyond 80 bits. When `MarketStateLibrary.storeMarketState` is called to update storage, it adds the `_borrowIndex` to the packed value without masking or checking if it fits in 80 bits. 

If `borrowIndex` exceeds `type(uint80).max`, the overflow spills into the adjacent `marketEpoch` bits. This corrupts the timestamp tracking (adding 1 to the spillover adds 4 seconds to the epoch), which will cause `_calculateCurrentInterestState` to compute incorrect time deltas (potentially underflowing `uint32` subtraction or computing huge intervals) in subsequent calls. This results in massive incorrect interest accrual, rendering the protocol insolvent or unusable.

While `borrowIndex` starts at `1e18`, high utilization rates (up to 800% APY) allowed by the RiskEngine could cause this overflow within ~6-7 years.

## Impact
Complete protocol failure due to corrupted state and interest calculations.

## Command to Run Test


## Proof of Concept
1. `borrowIndex` grows over time via interest accrual. 
2. Once `borrowIndex` > `2^80 - 1` (approx `1.2e24`), `_accrueInterest` calls `storeMarketState`.
3. The addition in `storeMarketState` carries the overflow bit into `marketEpoch`.
4. `marketEpoch` (last updated timestamp) is now corrupted (effectively pushed into the future).
5. Next interaction calls `_calculateCurrentInterestState`. `deltaTime = currentEpoch - storedEpoch` underflows (since stored is future) or is garbage.
6. Interest calculation uses huge/wrong `deltaTime`, setting `borrowIndex` to max or confusing accounting.

## Proof of Code
function testBorrowIndexOverflow() public {
    // Setup MarketState with max uint80 borrowIndex
    uint256 max80 = type(uint80).max;
    // Simulate calculation returning slightly more
    uint128 newIndex = uint128(max80) + 1;
    uint32 epoch = 100;
    
    // Store
    MarketState state = MarketStateLibrary.storeMarketState(newIndex, epoch, 0, 0);
    
    // Decode
    uint80 storedIndex = state.borrowIndex();
    uint32 storedEpoch = state.marketEpoch();
    
    // Assertions
    assertEq(storedIndex, 0); // Rolled over in lower bits
    assertEq(storedEpoch, epoch + 1); // Corrupted epoch
}

## Suggested Mitigation
In `MarketStateLibrary.storeMarketState`, allow `_borrowIndex` to be larger but mask it to 80 bits, or revert if `_borrowIndex > type(uint80).max`. Alternatively, in `CollateralTracker`, ensure `currentBorrowIndex` is capped at `type(uint80).max`.



