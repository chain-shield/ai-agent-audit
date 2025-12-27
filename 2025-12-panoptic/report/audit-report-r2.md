# 2025 12 panoptic - Findings Report
## Commit hash: 4ef958cbe0a136fac56138f55a64706d1db037de

##Findings by Status


Finding Status: Valid


[M-1]. Collateral Requirement Bypass via Flash Utilization Manipulation
**Derived From** : Large liquidity provider
Finding Status: Valid
Privilege: Permissionless


[H-2]. Collateral Requirement Bypass via Flash Utilization Manipulation
**Derived From** : Collateral Requirement Bypass via Flash Utilization Manipulation
Finding Status: Valid
Privilege: Permissionless


[M-3]. Protocol Commission Skimming via Builder Codes due to Unburnt Remainder
**Derived From** : PanopticFactory deploying clone immutable args
Finding Status: Valid
Privilege: Permissionless


[H-4]. BuilderWallet Hijacking via Unprotected Init allowing fee theft
**Derived From** : Anyone calling BuilderWallet.init
Finding Status: Valid
Privilege: Permissionless


[M-5]. Collateral Requirement Bypass via Flash Utilization Manipulation
**Derived From** : Large liquidity provider: Collateral Requirement Bypass via Flash Utilization Manipulation
Finding Status: Valid
Privilege: Permissionless


[H-6]. Theft of stuck ETH via Multicall msg.value reuse in Native V4 Pools
**Derived From** : Multicall batch composer
Finding Status: Valid
Privilege: Permissionless


[H-7]. BuilderWallet Hijacking via Unprotected Init Function
**Derived From** : Anyone calling BuilderWallet.init
Finding Status: Valid
Privilege: Permissionless


[M-8]. Collateral Requirement Bypass via Flash Utilization Manipulation
**Derived From** : Large liquidity provider: Collateral Requirement Bypass via Flash Utilization Manipulation
Finding Status: Valid
Privilege: Permissionless


[M-9]. Liquidation DoS via atomic spot price manipulation
**Derived From** : Liquidation bot
Finding Status: Valid
Privilege: Permissionless


[M-10]. Liquidation DoS via Atomic Spot Price Manipulation
**Derived From** : Liquidation bot: Liquidation DoS via atomic spot price manipulation
Finding Status: Valid
Privilege: Permissionless


[H-11]. Interest Debt Evasion via Insolvency on Position Closure
**Derived From** : Any user calling dispatch()
Finding Status: Valid
Privilege: Permissionless


[M-12]. Interest Rate Manipulation via Atomic Utilization Spiking
**Derived From** : Mempool MEV searcher: Flash Interest Rate Manipulation via Atomic Utilization Spiking
Finding Status: Valid
Privilege: Permissionless


[H-13]. BuilderWallet Hijacking via Unprotected Init
**Derived From** : Anyone calling BuilderWallet.init
Finding Status: Valid
Privilege: Permissionless


[H-14]. Interest Debt Evasion via Insolvency on Position Closure
**Derived From** : Any user calling dispatch()
Finding Status: Valid
Privilege: Permissionless


[H-15]. Cross-Vegoid Namespace Collision Corrupts Accounting
**Derived From** : Permissionless Uniswap pool initializer in SFPM
Finding Status: Valid
Privilege: Permissionless


[H-16]. Theft of stuck ETH via Multicall msg.value reuse in Native V4 Pools
**Derived From** : Multicall batch composer
Finding Status: Valid
Privilege: Permissionless


[H-17]. Interest Debt Evasion via Insolvency on Position Closure
**Derived From** : Any user calling dispatch()
Finding Status: Valid
Privilege: Permissionless


[H-18]. Flash Interest Rate Manipulation via Atomic Utilization Spiking
**Derived From** : Mempool MEV searcher
Finding Status: Valid
Privilege: Permissionless


[M-19]. Liquidation DoS via Spot Price Manipulation
**Derived From** : Underlying pool trader moving tick
Finding Status: Valid
Privilege: Permissionless


[H-20]. Protocol Commission Skimming via Builder Codes deprives PLPs of revenue
**Derived From** : PanopticFactory deploying clone immutable args
Finding Status: Valid
Privilege: Permissionless


[M-21]. Protocol fee leakage and PLP yield loss due to incorrect fee splitting in CollateralTracker
**Derived From** : Invariant Type: Balance - total_commission_paid == expected_commission_fee
Finding Status: Valid
Privilege: Permissionless


[M-22]. Liquidation DoS due to arithmetic underflow in RiskEngine.getLiquidationBonus for solvent-by-value but insolvent-by-rule accounts
**Derived From** : Invariant Type: Arithmetic; Predicate: liquidationBonus <= collateralBalance
Finding Status: Valid
Privilege: Permissionless


[H-23]. Force Exercise Fee Inversion via Tick Manipulation
**Derived From** : Third-party operator calling dispatchFrom()
Finding Status: Valid
Privilege: Permissionless


[H-24]. Force Exercise Fee Evasion via Oracle Manipulation
**Derived From** : Force exercisor targeting stuck longs
Finding Status: Valid
Privilege: Permissionless


[H-25]. Force Exercise Fee Inversion via Tick Manipulation
**Derived From** : Force exercisor targeting stuck longs
Finding Status: Valid
Privilege: Permissionless


[M-26]. Force Exercise Fee Evasion via Oracle Manipulation
**Derived From** : Force exercisor targeting stuck longs
Finding Status: Valid
Privilege: Permissionless



Finding Status: InvalidOutOfScope


[H-27]. Unincentivized Liquidation and Seller Loss due to Insolvency Masking in RiskEngine
**Derived From** : Large position holder near insolvency
Finding Status: InvalidOutOfScope
Privilege: Permissionless


[H-28]. Insufficient Premium Haircut due to Insolvency Masking in RiskEngine
**Derived From** : Large position holder near insolvency
Finding Status: InvalidOutOfScope
Privilege: Permissionless


[H-29]. Free Interest Accrual via Delegation Exploitation
**Derived From** : CollateralTracker vault contract
Finding Status: InvalidOutOfScope
Privilege: Permissionless


[H-30]. Liquidation bonus evaluates to zero for deeply insolvent accounts due to masked collateral balance
**Derived From** : liquidationBonusUsesGrossCollateral == true
Finding Status: InvalidOutOfScope
Privilege: Permissionless


[H-31]. Free Interest Accrual via Self-Settlement and Delegation Mechanism Abuse
**Derived From** : CollateralTracker vault contract: Free Interest Accrual via Delegation exploitation
Finding Status: InvalidOutOfScope
Privilege: Permissionless


[H-32]. Permanent supply inflation and share value dilution due to accounting error in insolvent account delegation
**Derived From** : CollateralTracker Asset Accounting
Finding Status: InvalidOutOfScope
Privilege: Permissionless


[H-33]. Free Interest Accrual via Delegation exploitation
**Derived From** : CollateralTracker vault contract
Finding Status: InvalidOutOfScope
Privilege: Permissionless


[H-34]. Protocol Value Leakage via Phantom Share Interest Burn in CollateralTracker
**Derived From** : CollateralTracker0 vault contract
Finding Status: InvalidOutOfScope
Privilege: Permissionless


[H-35]. DoS of liquidations for insolvent accounts due to zeroed liquidation bonus
**Derived From** : RiskEngine Liquidation Bonuses
Finding Status: InvalidOutOfScope
Privilege: Permissionless


[H-36]. Ghost Shares created in CollateralTracker due to incorrect revoke logic during insolvency
**Derived From** : Balance Invariant: totalSupply() decreases by the amount of real shares consumed for interest during delegation
Finding Status: InvalidOutOfScope
Privilege: Permissionless


[H-37]. Protocol Value Leakage via Phantom Share Interest Burn
**Derived From** : CollateralTracker0 vault contract
Finding Status: InvalidOutOfScope
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood + InvalidNotExploitable


[M-38]. Permanent DoS of Pool Initialization via Collision Pre-computation
**Derived From** : PoolId collision griefer via many pool creations
Finding Status: LowSeverityDueToRareLikelihood + InvalidNotExploitable
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood + InvalidByDesign


[M-39]. Interest Rate Stagnation in Inactive Pools
**Derived From** : Borrower
Finding Status: LowSeverityDueToRareLikelihood + InvalidByDesign
Privilege: Permissionless


[M-40]. Interest Rate Model Stagnation allows cheap borrowing on inactive pools
**Derived From** : Borrower: Interest Rate Stagnation in Inactive Pools
Finding Status: LowSeverityDueToRareLikelihood + InvalidByDesign
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood


[M-41]. Liquidation Denial of Service due to _internalSupply underflow
**Derived From** : Invariant Type: Balance - amount <= _internalSupply
Finding Status: LowSeverityDueToRareLikelihood
Privilege: Permissionless


[M-42]. Liquidation DoS due to arithmetic underflow when debt exceeds share supply
**Derived From** : Invariant Type: Balance - CollateralTracker._burn
Finding Status: LowSeverityDueToRareLikelihood
Privilege: Permissionless


[M-43]. Permanent Denial of Service in Liquidation via Internal Supply Underflow
**Derived From** : Liquidator triggering settleLiquidation
Finding Status: LowSeverityDueToRareLikelihood
Privilege: Permissionless


[M-44]. Permanent Denial of Service in Liquidation via Internal Supply Underflow
**Derived From** : Liquidator triggering settleLiquidation
Finding Status: LowSeverityDueToRareLikelihood
Privilege: Permissionless



Finding Status: LowSeverityDueToRareLikelihood + InvalidERC20EdgeCase


[H-45]. Delegated Share Reentrancy Attack in Liquidation
**Derived From** : ERC4626 withdrawer or redeemer
Finding Status: LowSeverityDueToRareLikelihood + InvalidERC20EdgeCase
Privilege: Permissionless


### Number of Findings
- C: 0
- H: 27
- M: 18
- L: 0
- I: 0

##Findings by Status


Finding Status: Valid
## [M-1]. Collateral Requirement Bypass via Flash Utilization Manipulation

## id: M0iHMvdyWRaSQXGQXqyI9

## Derived From Pattern/Invariant
Large liquidity provider

## Exploit Type
FlashLoanEconomicManipulation

## Location
RiskEngine._getGlobalUtilization

## Finding Status: Valid
### Finding Status Justification: The transient-slot 'max utilization in tx' protection cannot capture the pre-transaction utilization, so a flash deposit at tx start can lower the first recorded utilization and allow minting with an artificially low utilization snapshot. This is achievable in one transaction and can materially reduce required collateral for a lasting position, so it is exploitable and not low impact/likelihood.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `RiskEngine` calculates collateral requirements based on the utilization *stored at the time of minting* in `PositionBalance`. An attacker can flash-deposit liquidity to lower utilization to near 0%, mint a large position with minimal collateral requirements (e.g., 20%), and then immediately withdraw the liquidity. The position retains the low collateral requirement despite the pool subsequently having high utilization (high risk).

## Impact
Protocol is exposed to undercollateralized high-risk positions; potential for bad debt if market moves.

## Command to Run Test


## Proof of Concept
1. Attacker flash loans funds and deposits into CollateralTracker (Utilization -> 0%).
2. Attacker mints large short option position (Requirement = 20%).
3. Attacker withdraws deposit (Utilization -> High).
4. Position remains active with 20% requirement despite high risk environment.

## Proof of Code
function testCollateralBypass() public {
    // Flash deposit, mint, withdraw scenario
}

## Suggested Mitigation
Require collateral checks to use the greater of `stored utilization` and `current utilization`, or re-evaluate utilization during solvency checks.


## [H-2]. Collateral Requirement Bypass via Flash Utilization Manipulation

## id: pyjZktrG_IrQ8nlPLIZQr

## Derived From Pattern/Invariant
Collateral Requirement Bypass via Flash Utilization Manipulation

## Exploit Type
FlashLoanEconomicManipulation

## Location
CollateralTracker._poolUtilizationWad

## Finding Status: Valid
### Finding Status Justification: Same core issue: transient tracking can't see pre-tx utilization, so a flash deposit can set a low utilization baseline and allow minting with a low utilization snapshot that persists. This is feasible and can materially under-collateralize positions relative to post-tx utilization.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `RiskEngine` calculates collateral requirements based on the pool utilization stored in `PositionBalance` at the time of minting. The `CollateralTracker` employs transient storage to track the maximum utilization during a transaction to prevent manipulation, but it initializes this tracker with the utilization *after* the first interaction. An attacker can flash-deposit a large amount of assets (lowering utilization), mint a large option position (locking in the low utilization and thus a low collateral requirement), and then withdraw the assets in the same transaction. The position retains the low collateral requirement indefinitely, bypassing risk controls.

## Impact
Attacker creates highly leveraged, undercollateralized positions, exposing the protocol to bad debt.

## Command to Run Test


## Proof of Concept
1. Attacker observes high utilization (e.g., 80%).
2. Attacker flash-deposits huge liquidity. Utilization drops to 1%.
3. `deposit` calls `_accrueInterest` -> `_poolUtilizationWad`, initializing transient storage with 1%.
4. Attacker mints options. `PositionBalance` records 1% utilization (min collateral ratio).
5. Attacker withdraws deposit.
6. Attacker holds a risky position with minimal collateral.

## Proof of Code
function testFlashUtilization() public {
    // 1. Flash deposit
    collateralTracker.deposit(1000 ether, address(this));
    // 2. Mint option (locks low util)
    panopticPool.dispatch(mintParams...);
    // 3. Withdraw
    collateralTracker.withdraw(1000 ether, ...);
    // 4. Assert position has low collateral requirement despite high pool utilization
    (,,,, int256 util0,,) = panopticPool.positionData(address(this), tokenId);
    assertLt(util0, 100); // 1%
}

## Suggested Mitigation
Ensure `_poolUtilization` considers the utilization *before* any deposits in the transaction, or disallow minting if utilization has changed significantly within the block.


## [M-3]. Protocol Commission Skimming via Builder Codes due to Unburnt Remainder

## id: A5Ly18NIBDexm9Fq8MMLT

## Derived From Pattern/Invariant
PanopticFactory deploying clone immutable args

## Exploit Type
IncentiveMisalignmentOrGameTheory

## Location
CollateralTracker.sol.settleMint

## Finding Status: Valid
### Finding Status Justification: CollateralTracker.settleMint() computes sharesToBurn from commissionFee, but when feeRecipient!=0 it transfers only (sharesToBurn*protocolSplit/DECIMALS) + (sharesToBurn*builderSplit/DECIMALS). With PROTOCOL_SPLIT=6500 and BUILDER_SPLIT=2500 (DECIMALS=10_000), 10% of sharesToBurn is neither transferred nor burned, leaving the minter effectively paying less commission than intended. No other code path burns/transfers the remainder. This is directly exploitable by using any valid builder code. Impact is systemic fee leakage (reduced fee collection / reduced PLP benefit compared to the no-builder path), but not a direct vault drain, so Medium.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `CollateralTracker.settleMint`, commission fees are split between the protocol and a builder if a builder code is present. The logic calculates `sharesToBurn` based on the total commission, then transfers `shares * protocolSplit` and `shares * builderSplit`. If the sum of splits is less than 100% (e.g. 65% + 25% = 90%), the remaining shares (10%) are neither transferred nor burned, effectively remaining in the user's wallet as a discount.

## Impact
Loss of protocol revenue and yield for PLPs (since unburned shares do not increase share price).

## Command to Run Test


## Proof of Concept
1. Deploy with Protocol Split 65%, Builder Split 25%. 2. User mints option with builder code. 3. `settleMint` calculates commission of 100 shares. 4. Transfers 65 to protocol, 25 to builder. 5. Remaining 10 shares are kept by user.

## Proof of Code
function testCommissionSkim() public { ... }

## Suggested Mitigation
Ensure the remaining shares (100% - splits) are burned or transferred to the protocol.


## [H-4]. BuilderWallet Hijacking via Unprotected Init allowing fee theft

## id: JuGZo1dQlKIE61xuY5efd

## Derived From Pattern/Invariant
Anyone calling BuilderWallet.init

## Exploit Type
AccessControl

## Location
RiskEngine.sol.BuilderWallet.init

## Finding Status: Valid
### Finding Status Justification: Same BuilderWallet.init issue: any caller can overwrite builderAdmin post-deploy and sweep ERC20 balances; not a design choice and not excluded by scope.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `BuilderWallet` contract (defined in `RiskEngine.sol`) has an `init` function that sets the `builderAdmin` address. This function is `external` and lacks any access control or a check to see if the wallet has already been initialized. While the `BuilderFactory` calls `init` immediately after deployment, the function remains open. An attacker can call `init` on any deployed `BuilderWallet` to overwrite the `builderAdmin` with their own address. Subsequently, the attacker can call `sweep` to drain all fees accumulated in the wallet.

## Impact
Complete theft of all protocol fees allocated to builders/partners.

## Command to Run Test


## Proof of Concept
1. Monitoring the chain, an attacker identifies a `BuilderWallet` deployed by `BuilderFactory`. 2. The attacker calls `BuilderWallet.init(attacker_address)` on the target wallet. 3. `builderAdmin` is updated to the attacker's address. 4. The attacker calls `sweep(token, attacker_address)` to withdraw all funds.

## Proof of Code
contract BuilderHijackTest is Test {
    function testHijack() public {
        // Setup
        RiskEngine re = new RiskEngine(0,0,address(0),address(0));
        BuilderFactory bf = new BuilderFactory(address(this));
        address wallet = bf.deployBuilder(1, address(this));
        
        // Simulate fees accruing
        MockERC20 token = new MockERC20();
        token.mint(wallet, 1000 ether);

        // Attack
        address attacker = address(0xBAD);
        vm.prank(attacker);
        BuilderWallet(wallet).init(attacker);

        // Steal
        vm.prank(attacker);
        BuilderWallet(wallet).sweep(address(token), attacker);
        assertEq(token.balanceOf(attacker), 1000 ether);
    }
}

## Suggested Mitigation
Add a boolean flag `initialized` to `BuilderWallet` and require `!initialized` in the `init` function, or add `require(builderAdmin == address(0))`.


## [M-5]. Collateral Requirement Bypass via Flash Utilization Manipulation

## id: a6BQmsI_n-d3mdzZYrNHE

## Derived From Pattern/Invariant
Large liquidity provider: Collateral Requirement Bypass via Flash Utilization Manipulation

## Exploit Type
FlashLoanEconomicManipulation

## Location
CollateralTracker._poolUtilization

## Finding Status: Valid
### Finding Status Justification: Same core issue: the in-tx max tracking cannot defend against starting a transaction by lowering utilization from a high pre-tx value, so mint-time utilization snapshots can be manipulated and persist beyond the tx.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `CollateralTracker` uses a transient storage mechanism (`UTILIZATION_TRANSIENT_SLOT`) to track the maximum utilization within a transaction to prevent manipulation. However, if a transaction *starts* with a large deposit, the utilization drops immediately. Subsequent `mint` actions in the same transaction will read this low utilization and lock in a low collateral requirement (e.g., 20% instead of 100%). If the attacker withdraws the deposit at the end of the transaction, the utilization spikes back up, but the position retains the low collateral requirement.

## Impact
Users can create highly leveraged positions in saturated pools where 100% collateralization should be enforced, increasing protocol risk.

## Command to Run Test


## Proof of Concept
1. Pool utilization is 95% (Requires 100% collateral).
2. Attacker Flash Deposits large amount. Utilization drops to 10%.
3. Attacker calls `dispatch(mint)`. `_poolUtilization` reads 10%. Position minted with 20% collateral req.
4. Attacker withdraws deposit. Utilization returns to 95%.
5. Attacker holds a risky position with insufficient collateral relative to pool risk.

## Proof of Code
function testCollateralBypass() public {
    // Flash deposit
    collateralTracker.deposit(hugeAmount, attacker);
    // Mint options
    panopticPool.dispatch(mintParams);
    // Withdraw
    collateralTracker.withdraw(hugeAmount, attacker, attacker);
    // Check position collateral requirement is low despite high util
}

## Suggested Mitigation
Ensure `_poolUtilization` captures the utilization state *before* any deposits in the transaction, or enforce re-evaluation of collateral requirements upon withdrawal if utilization increases significantly.


## [H-6]. Theft of stuck ETH via Multicall msg.value reuse in Native V4 Pools

## id: m9UBk0JjMp7Rrbfu25pmc

## Derived From Pattern/Invariant
Multicall batch composer

## Exploit Type
ReplayAttack

## Location
CollateralTracker.deposit

## Finding Status: Valid
### Finding Status Justification: Although this entry cites `CollateralTracker.deposit` as the location, the exploit requires `Multicall`’s `delegatecall` behavior: `msg.value` remains constant across subcalls. Each `deposit` on a native v4 pool passes the same `msg.value` into `_settleCurrencyDelta` and `unlockCallback` attempts to settle `assets` ETH from the CollateralTracker balance. After the first deposit uses the attacker-funded ETH, subsequent deposits can draw from any ETH already held by the contract, letting the attacker mint extra shares and effectively steal stuck ETH.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `CollateralTracker` inherits `Multicall` and allows batching of calls. The `deposit` function is `payable` and uses `msg.value` to settle ETH in Native V4 pools. In a multicall sequence, `msg.value` persists across delegatecalls. If an attacker sends 1 ETH and calls `deposit(1 ETH)` twice, the `msg.value` check/usage applies to both. The first call consumes the legitimate 1 ETH. The second call attempts to use `msg.value` (1 ETH) again. If the `CollateralTracker` holds any excess/stuck ETH (e.g., from `selfdestruct` or mistakenly sent funds), the second `deposit` will successfully consume that stuck ETH, crediting the attacker with 2 ETH deposits for the cost of 1 ETH.

## Impact
Theft of all stuck ETH in the contract, or draining of other users' pending ETH deposits.

## Command to Run Test


## Proof of Concept
1. Identify a `CollateralTracker` with stuck ETH (or force-feed it). 2. Construct a `multicall` transaction with two `deposit(1 ETH)` calls. 3. Attach 1 ETH to the transaction. 4. Execute. The first deposit consumes the sent ETH. The second deposit consumes the stuck ETH. 5. Attacker receives shares for 2 ETH.

## Proof of Code
function testMulticallValueReuse() public { 
 // Seed contract with stuck ETH 
 vm.deal(address(collateralTracker), 1 ether); 
 bytes[] memory calls = new bytes[](2); 
 calls[0] = abi.encodeWithSelector(CollateralTracker.deposit.selector, 1 ether, attacker); 
 calls[1] = abi.encodeWithSelector(CollateralTracker.deposit.selector, 1 ether, attacker); 
 collateralTracker.multicall{value: 1 ether}(calls); 
 assertEq(collateralTracker.balanceOf(attacker), collateralTracker.convertToShares(2 ether)); 
 }

## Suggested Mitigation
Override `multicall` to prevent `msg.value` reuse, or ensure `deposit` explicitly checks that it consumes only its allocated portion of `msg.value` (non-trivial in Multicall). Recommended: Do not allow `payable` multicall with loopable `deposit` that relies on `msg.value`.


## [H-7]. BuilderWallet Hijacking via Unprotected Init Function

## id: AwDui_9ZQXE_j1-Hm-dFD

## Derived From Pattern/Invariant
Anyone calling BuilderWallet.init

## Exploit Type
AccessControl

## Location
RiskEngine.sol.init

## Finding Status: Valid
### Finding Status Justification: BuilderWallet.init is external and re-initializable with no access control or one-time guard; this enables permissionless takeover and fee theft and is within scope (RiskEngine.sol).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `BuilderWallet` contract has a public `init` function that sets the `builderAdmin` address. This function lacks any access control or a check to see if the contract has already been initialized. Although `BuilderFactory` calls `init` immediately after deployment, the `BuilderWallet` is a separate contract. An attacker can monitor the mempool for `BuilderFactory.deployBuilder` transactions or scan for deployed wallets and call `init` again (or front-run/back-run) to overwrite the `builderAdmin`. Once the admin is overwritten, the attacker can call `sweep` to drain any fees accrued in the wallet.

## Impact
Theft of all protocol fees destined for the builder wallet.

## Command to Run Test


## Proof of Concept
1. `BuilderFactory` deploys `BuilderWallet` using CREATE2 and calls `init(honestAdmin)`. 2. Attacker observes the new wallet address. 3. Attacker calls `BuilderWallet.init(attacker)` on the wallet. 4. `builderAdmin` is updated to `attacker`. 5. Attacker calls `sweep` to transfer tokens to themselves.

## Proof of Code
contract BuilderWalletTest is Test { BuilderFactory factory; BuilderWallet wallet; function setUp() public { factory = new BuilderFactory(address(this)); address walletAddr = factory.deployBuilder(1, address(this)); wallet = BuilderWallet(walletAddr); } function testHijack() public { address attacker = address(0xBAD); vm.prank(attacker); wallet.init(attacker); assertEq(wallet.builderAdmin(), attacker); } }

## Suggested Mitigation
Add an `initialized` flag to the `BuilderWallet` and revert in `init` if it is already set, or ensure `builderAdmin` can only be set once.


## [M-8]. Collateral Requirement Bypass via Flash Utilization Manipulation

## id: SzdfTpoxyUahDm2dt_iEh

## Derived From Pattern/Invariant
Large liquidity provider: Collateral Requirement Bypass via Flash Utilization Manipulation

## Exploit Type
FlashLoanEconomicManipulation

## Location
PanopticPool.sol.dispatch

## Finding Status: Valid
### Finding Status Justification: Same utilization-snapshot issue: a large deposit can lower utilization at the time the snapshot is taken, and because requirements key off mint-time utilization, the position can remain under-collateralized after utilization returns high. The transient-slot approach does not eliminate this if the tx begins after utilization has already been lowered.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Users can lower their collateral requirement for a position by manipulating the pool utilization within the same transaction. By flash-depositing a large amount of assets, the utilization drops. The user then mints a position, which snapshots this low utilization (and thus low collateral requirement) in `PositionBalance`. The user then withdraws the deposit. The position retains the low collateral requirement indefinitely, exposing the protocol to under-collateralized risk.

## Impact
Protocol exposure to high-risk positions with insufficient collateral, leading to potential bad debt.

## Command to Run Test


## Proof of Concept
1. Current Util: 80%. Collateral Req: High.
2. Attacker Flash Deposits. Util -> 10%.
3. Attacker Mints Short Put. Snapshots 10% Util (Low Req).
4. Attacker Withdraws Deposit. Util -> 80%.
5. Attacker maintains position with 10% Util requirement.

## Proof of Code
function testFlashUtil() public {
    // Check high req
    tracker.deposit(hugeAmount);
    pool.dispatch(mint);
    tracker.withdraw(hugeAmount);
    // Assert position stored low utilization
}

## Suggested Mitigation
Prevent deposits and mints in the same block, or use a time-weighted average for utilization in `settleMint`, or check utilization at the end of the transaction for the snapshot.


## [M-9]. Liquidation DoS via atomic spot price manipulation

## id: R3ElhwCj4UaWk2zH3x_0c

## Derived From Pattern/Invariant
Liquidation bot

## Exploit Type
Dos

## Location
PanopticPool._checkSolvencyAtTicks

## Finding Status: Valid
### Finding Status Justification: The TWAP deviation bound (513 ticks) limits manipulation magnitude but does not prevent the DoS: for near-threshold accounts a within-band spot move can make the account solvent at one tick, causing NotMarginCalled and blocking liquidation.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The liquidation logic in `PanopticPool` requires an account to be insolvent at *all* checked ticks (Spot, TWAP, Median, Latest). If `solvent > 0`, liquidation reverts. An attacker (or the insolvent user) can manipulate the Spot price (`currentTick`) within the allowed 5% deviation to make the account appear solvent at that single tick, thereby blocking the liquidation even if the account is insolvent at all other oracle ticks.

## Impact
Insolvent accounts cannot be liquidated, leading to accrual of bad debt.

## Command to Run Test


## Proof of Concept
1. Account is insolvent at TWAP and Median.
2. Attacker swaps in Uniswap to move `currentTick` to a price where the account is barely solvent.
3. Liquidator calls `dispatchFrom`.
4. `_checkSolvencyAtTicks` finds account solvent at `currentTick`. `solvent` becomes 1.
5. Transaction reverts with `NotMarginCalled`.

## Proof of Code
function testLiquidationDoS() public {
    // Manipulate spot price to block liquidation
}

## Suggested Mitigation
Relax the liquidation condition; allow liquidation if the account is insolvent at the reliable Oracle tick (TWAP/Median), regardless of the manipulatable Spot tick.


## [M-10]. Liquidation DoS via Atomic Spot Price Manipulation

## id: 6XnuVGqBOkPeN4ATVbjKr

## Derived From Pattern/Invariant
Liquidation bot: Liquidation DoS via atomic spot price manipulation

## Exploit Type
Dos

## Location
PanopticPool.sol.dispatchFrom

## Finding Status: Valid
### Finding Status Justification: The TWAP-deviation check is only a partial mitigation; within the allowed band, a solvent-at-one-tick outcome can still be induced for marginal accounts, reverting liquidation.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `dispatchFrom` function requires an account to be insolvent at *all* four oracle ticks (Spot, TWAP, Latest, Current) to proceed with liquidation. If an account is solvent at any one of these ticks, liquidation reverts. An attacker can front-run a liquidation transaction by flash-swapping the pool to a price where the account appears solvent at the `currentTick` (instant spot). This blocks the liquidation even if the account is deeply insolvent according to the TWAP and other robust metrics.

## Impact
Inability to liquidate insolvent accounts, leading to bad debt accumulation.

## Command to Run Test


## Proof of Concept
1. Account is insolvent at TWAP.
2. Liquidator calls `dispatchFrom`.
3. Attacker front-runs: Swaps Uniswap pool to move `currentTick` to a favorable price.
4. `_checkSolvencyAtTicks` finds account solvent at `currentTick`.
5. Transaction reverts with `NotMarginCalled`.

## Proof of Code
function testLiquidationDoS() public {
    // Make account insolvent
    // Manipulate spot price to solvent range
    vm.expectRevert(Errors.NotMarginCalled.selector);
    pool.dispatchFrom(..., liquidate);
}

## Suggested Mitigation
Allow liquidation if the account is insolvent at the robust TWAP/Median ticks, ignoring the volatile `currentTick` if it contradicts the others.


## [H-11]. Interest Debt Evasion via Insolvency on Position Closure

## id: roQO248tRJ5rsTlE8w4tc

## Derived From Pattern/Invariant
Any user calling dispatch()

## Exploit Type
AccountingInvariantViolation

## Location
CollateralTracker._updateBalancesAndSettle

## Finding Status: Valid
### Finding Status Justification: Same root as 82ummALuMB6TMmDlcGaDM: _accrueInterest can partially burn shares without updating userBorrowIndex (insolvent path), then _updateBalancesAndSettle adjusts netBorrows (principal) down to 0 when the position is closed. Since owed interest is computed from principal and index delta, zeroing principal makes any unpaid interest unrecoverable. No separate accumulator or deduction-from-proceeds exists. This can create protocol bad debt/lost lender yield.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
A user can evade paying accrued interest by closing their position while 'interest-insolvent'. When `CollateralTracker.settleBurn` triggers `_accrueInterest`, if the user lacks sufficient shares to pay the full interest, their entire balance is burned, but their `userBorrowIndex` is NOT updated (to preserve the debt record). However, `_updateBalancesAndSettle` subsequently updates `s_interestState` by adding the closed position's `netBorrows` (as a negative delta) to the left slot, effectively zeroing out the user's `netBorrows` principal. Since the principal is now zero, the past unpaid interest (which is calculated as `principal * (currIndex - oldIndex)`) becomes unrecoverable and is effectively erased.

## Impact
Users can evade interest payments, causing loss of yield for the protocol/LPs.

## Command to Run Test


## Proof of Concept
1. User opens a short position (borrows assets).
2. Time passes; interest accrues such that `interest > userShares`.
3. User calls `dispatch()` to close the position.
4. `_accrueInterest` burns user's shares but keeps `userBorrowIndex` old.
5. `_updateBalancesAndSettle` zeros out `netBorrows`.
6. User withdraws principal collateral (via AMM swap proceeds or remaining collateral logic) minus only the principal debt, evading the interest debt.

## Proof of Code
function testInterestEvasion() public {
    // Setup: Borrower opens position
    // Fast forward time to accrue high interest
    // Borrower withdraws all collateral shares (or has 0 balance)
    // Borrower closes position
    // Assert borrow index not updated but debt principal zeroed
}

## Suggested Mitigation
In `_accrueInterest`, when the user is insolvent, the unpaid interest should be stored in a separate accumulator or deducted from the settlement proceeds (tokenToPay) in `_updateBalancesAndSettle`.


## [M-12]. Interest Rate Manipulation via Atomic Utilization Spiking

## id: mmdg122TKDNdXKN-hnh8X

## Derived From Pattern/Invariant
Mempool MEV searcher: Flash Interest Rate Manipulation via Atomic Utilization Spiking

## Exploit Type
FlashLoanEconomicManipulation

## Location
CollateralTracker._updateInterestRate

## Finding Status: Valid
### Finding Status Justification: CollateralTracker._accrueInterest() computes interest for the entire elapsed period since last update (`deltaTime`) using `interestRateSnapshot` produced by `_updateInterestRate()`. `_updateInterestRate()` in turn uses `_poolUtilizationWad()` which intentionally snapshots the MAX utilization observed in the current transaction via transient storage. Therefore, after a long idle period, an attacker can first increase `s_assetsInAMM` (e.g., via a large mint/borrow action), then call `accrueInterest()` in the same transaction: the global borrow index and `unrealizedInterest` are updated as if the whole past interval experienced the spiked utilization. This globally increases borrowers’ debt and can trigger liquidations or value transfer to lenders. There is no mechanism to use the “previous-period” utilization for the past interval.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `CollateralTracker` latches the *maximum* pool utilization observed in a transaction into transient storage via `_poolUtilizationWad`. When `accrueInterest` is called, it calculates the interest rate for the *past* time interval using this transient maximum. 

An attacker can mint a massive position (spiking utilization to 100%) and call `accrueInterest` in the same transaction. The protocol will apply the 100% utilization interest rate to the entire time elapsed since the last interaction (e.g., 24 hours), drastically inflating the interest owed by all borrowers. The attacker can then burn their position, reverting utilization, but the inflated debt remains.

## Impact
Griefing of borrowers (massive interest debt) or theft of yield by lenders.

## Command to Run Test


## Proof of Concept
1. Pool has been inactive for 24h.
2. Attacker mints large option to spike utilization to 100%.
3. Attacker calls `accrueInterest`.
4. `_updateInterestRate` reads 100% utilization.
5. `_calculateCurrentInterestState` applies the high rate to the past 24h.
6. Borrow index jumps.
7. Attacker burns position.

## Proof of Code
function testFlashRateManipulation() public { ... }

## Suggested Mitigation
Do not update the interest rate for the *past* interval using the *current/transient* utilization. Use the utilization stored from the *previous* interaction.


## [H-13]. BuilderWallet Hijacking via Unprotected Init

## id: HA7pUwHgvBXLo88oZsfWk

## Derived From Pattern/Invariant
Anyone calling BuilderWallet.init

## Exploit Type
AccessControl

## Location
BuilderWallet.init

## Finding Status: Valid
### Finding Status Justification: BuilderWallet.init is unguarded and can be called repeatedly by anyone to seize builderAdmin and sweep funds; this is an in-scope access control vulnerability.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `BuilderWallet.init` function is `external` and lacks any access control or check to see if the wallet is already initialized. An attacker can call `init` on a deployed wallet to overwrite the `builderAdmin` address with their own, then call `sweep` to drain accumulated fees.

## Impact
Theft of all protocol fees allocated to the builder.

## Command to Run Test


## Proof of Concept
1. Factory deploys wallet. 2. Attacker calls `init(attacker)`. 3. Attacker calls `sweep`.

## Proof of Code
function testWalletHijack() public { ... }

## Suggested Mitigation
Add a check `if (builderAdmin != address(0)) revert;` in `init`.


## [H-14]. Interest Debt Evasion via Insolvency on Position Closure

## id: 82ummALuMB6TMmDlcGaDM

## Derived From Pattern/Invariant
Any user calling dispatch()

## Exploit Type
AccountingInvariantViolation

## Location
CollateralTracker.sol.settleBurn

## Finding Status: Valid
### Finding Status Justification: Code path exists: settleBurn() -> _updateBalancesAndSettle() calls _accrueInterest(owner,false). In _accrueInterest, if interest shares > userBalance and !isDeposit, it burns the entire balance but explicitly keeps userBorrowIndex at the old value. Immediately after, _updateBalancesAndSettle mutates s_interestState[owner] by addToLeftSlot(netBorrowsDelta), which can reduce netBorrows to 0 when closing the position. Since future interest is computed as principal * (currIndex-userIndex)/userIndex, zeroing principal makes the previously-unpaid interest unrecoverable (erased). There is no mechanism to carry forward “unpaid interest” into a separate debt bucket or to deduct it from settlement proceeds. This can cause real protocol/LP yield loss (bad debt).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
When a user closes a position (`settleBurn`), `_accrueInterest` is called. If the user is insolvent on interest (shares < interest owed), it burns all shares but does *not* update the `userBorrowIndex`. The settlement logic then calculates `tokenToPay` (option proceeds) but fails to deduct the remaining unpaid interest. The user's `netBorrows` is reset (e.g. to 0), erasing the debt record associated with the old index, effectively allowing the user to exit with full option proceeds while defaulting on interest.

## Impact
Protocol bad debt; users can evade accrued interest payments.

## Command to Run Test


## Proof of Concept
1. User accrues significant interest debt > collateral shares. 2. User closes profitable position via `dispatch`. 3. `_accrueInterest` burns shares but leaves index stale. 4. `settleBurn` pays out full option value (`tokenToPay`) without deducting the unpaid interest deficit. 5. User withdraws proceeds; debt is erased.

## Proof of Code
function testInterestEvasion() public { ... }

## Suggested Mitigation
Deduct any remaining `interestOwed` from `tokenToPay` in `_updateBalancesAndSettle` if the user was unable to pay via shares.


## [H-15]. Cross-Vegoid Namespace Collision Corrupts Accounting

## id: -813SuoDE7rXH9-lPr5EX

## Derived From Pattern/Invariant
Permissionless Uniswap pool initializer in SFPM

## Exploit Type
AccountingInvariantViolation

## Location
SemiFungiblePositionManager._createLegInAMM

## Finding Status: Valid
### Finding Status Justification: The collision is real: positionKey omits vegoid while pool initialization/state is namespaced by (idV4, vegoid). No guard prevents initializing multiple vegoid values for same idV4, so the same (idV4, account, tokenType, ticks) shares liquidity/premium storage while premium math uses the caller's vegoid, enabling inconsistent accounting/corruption.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `SemiFungiblePositionManager` allows initializing the same Uniswap V4 pool multiple times with different `vegoid` parameters, creating distinct `poolId`s in the SFPM. However, the `positionKey` used to track `s_accountLiquidity` and `s_accountPremium` in `_createLegInAMM` is derived solely from the Uniswap V4 pool key hash (`key.toId()`), the user address, and token parameters, ignoring the `vegoid`. 

This means that positions in two different SFPM pools (e.g., `vegoid=1` and `vegoid=2`) that share the same underlying Uniswap pool will read and write to the same storage slots. This causes liquidity and premium accumulators to cross-contaminate. Since `_getPremiaDeltas` uses the `vegoid` of the current call for its math (`removedLiquidity / vegoid`), mixing liquidity from different vegoid pools corrupts the premium calculations, leading to massive accounting errors, potential theft, or insolvency.

## Impact
Critical corruption of internal accounting. Users interacting with one pool inadvertently modify the state of another, leading to incorrect debt/credit calculations and potential loss of funds.

## Command to Run Test


## Proof of Concept
1. Attacker calls `initializeAMMPool` for a V4 pool with `vegoid=1`.
2. Attacker calls `initializeAMMPool` for the same V4 pool with `vegoid=2`.
3. Attacker mints a position in the `vegoid=1` pool. `s_accountLiquidity` is updated for the computed `positionKey`.
4. Attacker mints a position in the `vegoid=2` pool. The logic derives the *same* `positionKey` and adds to the existing liquidity.
5. When premiums are updated, the math mixes `vegoid=1` and `vegoid=2` parameters on the aggregated liquidity, resulting in nonsensical values.

## Proof of Code
function testCollision() public { address alice = address(0x1); vm.startPrank(alice); PoolKey memory key = ...; sfpm.initializeAMMPool(key, 1); sfpm.initializeAMMPool(key, 2); sfpm.mintTokenizedPosition(sfpm.getPoolId(key, 1), ...); sfpm.mintTokenizedPosition(sfpm.getPoolId(key, 2), ...); // Assert s_accountLiquidity is shared }

## Suggested Mitigation
Include `vegoid` in the `abi.encodePacked` arguments when generating `positionKey` in `_createLegInAMM`, `getAccountLiquidity`, and `getAccountPremium`.


## [H-16]. Theft of stuck ETH via Multicall msg.value reuse in Native V4 Pools

## id: GWJ3A0ocnolqpww2KIdQz

## Derived From Pattern/Invariant
Multicall batch composer

## Exploit Type
ReplayAttack

## Location
CollateralTracker.multicall

## Finding Status: Valid
### Finding Status Justification: CollateralTracker inherits Multicall, which executes each subcall via `delegatecall`, preserving the same `msg.value` across all internal calls. For Uniswap v4 native pools, `deposit()` calls `_settleCurrencyDelta(..., int256(assets))`, which passes `msg.value` into `poolManager.unlock` and then `unlockCallback` settles `uint256(delta)` ETH from the CollateralTracker’s balance. Because `msg.value` is not “consumed per subcall”, a multicall can invoke multiple `deposit(assets)` while only sending ETH once; subsequent deposits can be funded from any pre-existing/stuck ETH in the CollateralTracker. There is no per-subcall accounting of `msg.value`, so surplus ETH held by the contract can be stolen and turned into credited shares.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `CollateralTracker` inherits `Multicall` and allows batching calls via `delegatecall`. The `deposit` function is `payable` and uses `msg.value` to settle native ETH transfers with the `PoolManager`. Since `msg.value` persists across all `delegatecall` executions in a `multicall` transaction, an attacker can batch multiple `deposit` calls while sending ETH only once. Each `deposit` call will trigger a settlement of `msg.value` amount from the `CollateralTracker` to the `PoolManager`. If the `CollateralTracker` holds any surplus or stuck ETH (e.g., from direct transfers or self-destructs), the attacker can credit themselves with multiple deposits while only paying for one, effectively stealing the contract's ETH balance.

## Impact
Direct theft of ETH held by the contract.

## Command to Run Test


## Proof of Concept
1. Attacker identifies a `CollateralTracker` (V4 native ETH) with 1 ETH of stuck funds.
2. Attacker constructs a `multicall` with two payloads: `deposit(1 ETH)` and `deposit(1 ETH)`.
3. Attacker executes the transaction sending exactly 1 ETH.
4. First `deposit` uses the 1 ETH `msg.value` to settle with PoolManager. `CollateralTracker` pays 1 ETH.
5. Second `deposit` reuses the 1 ETH `msg.value` context. `CollateralTracker` pays another 1 ETH to PoolManager (using the stuck funds).
6. Attacker is credited with 2 ETH of shares but paid only 1 ETH.

## Proof of Code
function testExploitMulticall() public {
    // Setup: Force 1 ETH into CollateralTracker
    address attacker = address(0xBAD);
    vm.deal(address(collateralTracker), 1 ether);
    vm.deal(attacker, 1 ether);

    vm.startPrank(attacker);
    bytes[] memory calls = new bytes[](2);
    calls[0] = abi.encodeWithSelector(CollateralTracker.deposit.selector, 1 ether, attacker);
    calls[1] = abi.encodeWithSelector(CollateralTracker.deposit.selector, 1 ether, attacker);

    // Call multicall with 1 ETH
    CollateralTracker(collateralTracker).multicall{value: 1 ether}(calls);

    // Attacker should have 2 ETH worth of shares
    assertEq(collateralTracker.balanceOf(attacker), collateralTracker.convertToShares(2 ether));
}

## Suggested Mitigation
Override `multicall` to disallow `msg.value` preservation or ensure `deposit` checks that `msg.value` is not reused (e.g., by tracking used value or disallowing payable multicalls).


## [H-17]. Interest Debt Evasion via Insolvency on Position Closure

## id: IN-LPzXn_ozSWnKmtTfcI

## Derived From Pattern/Invariant
Any user calling dispatch()

## Exploit Type
AccountingInvariantViolation

## Location
CollateralTracker._accrueInterest

## Finding Status: Valid
### Finding Status Justification: Same core issue as 82ummALuMB6TMmDlcGaDM/roQO248tRJ5rsTlE8w4tc: CollateralTracker._accrueInterest’s insolvent path keeps userBorrowIndex unchanged, then position settlement can reduce netBorrows to 0, erasing the portion of accrued interest that could not be paid from shares. No separate “unpaid interest” tracking exists, and there is no deduction of remaining interest from settlement proceeds in _updateBalancesAndSettle.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `CollateralTracker._accrueInterest`, if a user owes more interest than their share balance, the function burns all shares but *does not* update the `userBorrowIndex`. However, when closing a position via `settleBurn` -> `_updateBalancesAndSettle`, the user's `netBorrows` is updated (potentially to 0). This effectively wipes the accrued interest debt that exceeded the user's balance, as future interest calculations will be based on the new `netBorrows` (0) regardless of the old index.

## Impact
Users can evade interest payments that exceed their collateral balance upon position closure.

## Command to Run Test


## Proof of Concept
1. User opens short position (borrows).
2. Accrues huge interest > collateral balance.
3. User calls `dispatch` to burn/close position.
4. `_accrueInterest` burns user's entire balance (partial pay). Index NOT updated.
5. `_updateBalancesAndSettle` sets user's `netBorrows` to 0.
6. User is now debt free; the unpaid interest is permanently erased.

## Proof of Code
function testInterestEvasion() public {
  // Accrue interest > balance
  vm.warp(block.timestamp + 365 days);
  pool.dispatch(burn);
  // Assert userBorrowIndex is old but netBorrows is 0
}

## Suggested Mitigation
When settling a position (`netBorrows` changes), any outstanding interest from the *old* principal must be accounted for or deducted from the settlement proceeds even if the user was momentarily insolvent.


## [H-18]. Flash Interest Rate Manipulation via Atomic Utilization Spiking

## id: exzWjocLvnf_P3y1Iupns

## Derived From Pattern/Invariant
Mempool MEV searcher

## Exploit Type
FlashLoanEconomicManipulation

## Location
CollateralTracker._accrueInterest

## Finding Status: Valid
### Finding Status Justification: Same root cause as the utilization-spike finding: `_accrueInterest()` applies a rate snapshot (derived from the transaction’s max utilization) to the entire past `deltaTime` since last interaction. Because `_poolUtilizationWad()` stores the maximum utilization within the transaction, an attacker can temporarily raise utilization and then force an interest accrual that retroactively charges all borrowers at that elevated rate for the full elapsed period. This globally increases borrowIndex and borrowers’ interest owed, which is economically exploitable (griefing and/or extracting value as a lender) after periods of inactivity.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
An attacker can manipulate the interest rate for the entire elapsed period by spiking pool utilization in the same transaction that interest is accrued. The `CollateralTracker` uses transient storage to track the maximum utilization within a transaction. The `_accrueInterest` function calculates interest for the *past* duration using the *current* rate derived from this transient maximum. By minting a large position (spiking utilization to 100%) and then calling `accrueInterest` (or triggering it via another action) within the same transaction, the attacker forces the protocol to calculate interest for the previous time interval (e.g., 24 hours) at the spiked 100% utilization rate, regardless of actual historical utilization. This inflicts massive unfair interest debt on borrowers.

## Impact
Borrowers incur massive unexpected debt; Lenders (or attacker holding shares) unfairly extract value.

## Command to Run Test


## Proof of Concept
1. Wait for significant time to pass without interaction (e.g., 24 hours). 2. Flash loan assets. 3. Mint a massive short option position via `PanopticPool`, driving `CollateralTracker` utilization to 100%. 4. Call `CollateralTracker.accrueInterest()`. The protocol reads the 100% utilization from transient storage and applies the corresponding max interest rate to the entire 24-hour period. 5. Burn the option position and repay flash loan. 6. Profit from share price increase or grief borrowers.

## Proof of Code
function testFlashRateManipulation() public { 
 vm.warp(block.timestamp + 1 days); 
 // Assume setup with borrowers 
 vm.startPrank(attacker); 
 // Mint huge position 
 panopticPool.dispatch(...); 
 // Utilization is now 100% 
 // Accrue interest 
 collateralTracker.accrueInterest(); 
 // Borrow index increased as if utilization was 100% for 1 day 
 panopticPool.dispatch(...); // Burn 
 vm.stopPrank(); 
 }

## Suggested Mitigation
Do not use the transient maximum utilization for interest accrual calculation of the *past* period. Interest accrual should use the utilization state *before* the current transaction's changes, or the rate should only apply to future time.


## [M-19]. Liquidation DoS via Spot Price Manipulation

## id: oFF6p9zZhhOMSp_qc7KnU

## Derived From Pattern/Invariant
Underlying pool trader moving tick

## Exploit Type
Dos

## Location
PanopticPool.dispatchFrom

## Finding Status: Valid
### Finding Status Justification: The twap-vs-current tick bound only limits the manipulation magnitude; for marginally insolvent accounts that become solvent within ~513 ticks, including currentTick in the 'must be insolvent at all ticks' condition can still be used to revert liquidations.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Liquidation in `PanopticPool.dispatchFrom` requires the target to be insolvent at *all* checked ticks (`spot`, `median`, `latest`, `current`). An insolvent user can block liquidation by manipulating the `currentTick` (spot price) using a flash loan/swap to make themselves appear solvent at that specific tick. Since `solvent == 0` (insolvent everywhere) is required for liquidation, a single solvent tick causes the transaction to revert with `NotMarginCalled`.

## Impact
Insolvent accounts cannot be liquidated, leading to bad debt accumulation.

## Command to Run Test


## Proof of Concept
1. User is insolvent at Oracle tick.
2. Liquidator calls `dispatchFrom`.
3. User (or MEV bot) front-runs with a swap to move `currentTick` such that `isAccountSolvent` returns true.
4. Liquidation transaction reverts.

## Proof of Code
function testLiquidationDoS() public {
  // make user insolvent
  // manipulate pool price
  vm.expectRevert(Errors.NotMarginCalled.selector);
  pool.dispatchFrom(..., liquidate, ...);
}

## Suggested Mitigation
Do not include `currentTick` in the 'all-must-be-insolvent' check, or use a majority vote system.


## [H-20]. Protocol Commission Skimming via Builder Codes deprives PLPs of revenue

## id: 1FeG0DJSb4QC8CY6Zsuwb

## Derived From Pattern/Invariant
PanopticFactory deploying clone immutable args

## Exploit Type
IncentiveMisalignmentOrGameTheory

## Location
CollateralTracker.settleMint

## Finding Status: Valid
### Finding Status Justification: Same core bug as A5Ly18NIBDexm9Fq8MMLT: when a builder feeRecipient is present, only protocolSplit and builderSplit portions of sharesToBurn are transferred; the remainder (DECIMALS - protocolSplit - builderSplit) is not burned/transferred, effectively discounting the commission. Additionally, the builder-code branch does not burn shares (unlike the no-builder branch), so PLPs do not receive the same share-price appreciation mechanism. This is directly exploitable by using a builder code and results in systematic fee leakage (medium impact).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `CollateralTracker.settleMint`, if a `feeRecipient` (builder) is present, the protocol splits the commission: 65% to RiskEngine (Protocol), 25% to Builder. The remaining 10% is neither burned nor transferred, effectively staying with the user. Crucially, Passive Liquidity Providers (PLPs) benefit only when shares are burned (appreciating share price). When a builder code is used, 0% is burned. This allows users to self-refer to get a 10% discount while PLPs lose 100% of their expected commission revenue.

## Impact
PLPs lose 100% of commission revenue on transactions with builder codes; users incentivized to use codes to pay less.

## Command to Run Test


## Proof of Concept
1. User mints option with a builder code (can be their own secondary address).
2. `settleMint` calculates commission.
3. Transfers `PROTOCOL_SPLIT` (65%) to RiskEngine and `BUILDER_SPLIT` (25%) to builder.
4. No burn occurs.
5. User pays 90% of fee. PLPs receive 0 benefit.

## Proof of Code
function testCommissionSkimming() public {
  // Setup builder code
  uint256 builderCode = 1;
  // Mint with code
  vm.prank(user);
  pool.dispatch(..., builderCode);
  // Check PLP share price (totalAssets/totalSupply) - unchanged (no burn)
  // Check User balance - paid 90% of calculated comms
}

## Suggested Mitigation
Burn the remaining shares or transfer the protocol split to the CollateralTracker itself to benefit PLPs.


## [M-21]. Protocol fee leakage and PLP yield loss due to incorrect fee splitting in CollateralTracker

## id: 2n_F2bxOIvOEwjZUaUN0k

## Derived From Pattern/Invariant
Invariant Type: Balance - total_commission_paid == expected_commission_fee

## Exploit Type
AccountingInvariantViolation

## Location
CollateralTracker.settleMint

## Finding Status: Valid
### Finding Status Justification: In `CollateralTracker.settleMint` (and similarly in `settleBurn`), when `riskParameters.feeRecipient() != 0`, the code transfers `(sharesToBurn * protocolSplit)/DECIMALS` and `(sharesToBurn * builderSplit)/DECIMALS`, but never transfers/burns the remainder `sharesToBurn - protocolShares - builderShares`. With the provided constants `PROTOCOL_SPLIT=6500` and `BUILDER_SPLIT=2500` (sum=9000), ~10% of the intended commission is not collected, letting option owners systematically underpay by using a builder code. This is a direct, repeatable accounting/fee-collection bug. (Additionally, the emitted `CommissionPaid` builder field appears to use `protocolSplit` twice, indicating event/accounting inconsistencies.)
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `CollateralTracker.settleMint` and `settleBurn`, when a builder (`feeRecipient`) is configured, the protocol splits the calculated commission fee (`sharesToBurn`) between the protocol and the builder. The split ratios are defined in `RiskEngine` as `PROTOCOL_SPLIT = 6500` (65%) and `BUILDER_SPLIT = 2500` (25%). These sum to 90%. The code transfers these portions from the user using `_transferFrom`. However, the remaining 10% of the calculated fee is neither transferred nor burned, effectively staying in the user's balance. Additionally, unlike the no-builder case where fees are burned (benefiting all PLPs via share appreciation), the builder case transfers shares without burning, resulting in 0% yield for PLPs on these transactions.

## Impact
The protocol fails to collect 10% of the intended fees, users are undercharged, and PLPs lose 100% of the yield from commissions when a builder is involved.

## Command to Run Test


## Proof of Concept
1. Configure a pool with a builder (`feeRecipient != 0`).
2. User mints an option calling `settleMint`.
3. `sharesToBurn` is calculated as 100 shares.
4. `PROTOCOL_SPLIT` (65) is transferred to riskEngine.
5. `BUILDER_SPLIT` (25) is transferred to builder.
6. Total 90 shares transferred. User retains 10 shares of the fee. 0 shares burned.

## Proof of Code
function testFeeLeakage() public {
    // Setup pool with builder
    // Mint option
    // Assert user balance decreased by only 90% of fee
    // Assert totalSupply did not decrease (no burn)
}

## Suggested Mitigation
Update `RiskParameters` splits to sum to 10000 (DECIMALS), or ensure the remainder is burned for PLPs or transferred to a default recipient.


## [M-22]. Liquidation DoS due to arithmetic underflow in RiskEngine.getLiquidationBonus for solvent-by-value but insolvent-by-rule accounts

## id: Hh59flTsI76ZLFdMa3pme

## Derived From Pattern/Invariant
Invariant Type: Arithmetic; Predicate: liquidationBonus <= collateralBalance

## Exploit Type
IntegerMath

## Location
RiskEngine.getLiquidationBonus

## Finding Status: Valid
### Finding Status Justification: getLiquidationBonus does `thresholdCross - balanceCross` with checked math; if `balanceCross > thresholdCross` this reverts and can brick liquidation. This state is plausible when cross-margining is effectively disabled (crossBufferRatio=0) and insolvency is determined per-token despite cross-value surplus. There is no saturating-sub or conditional guard, and the impact can be liquidation DoS during stressed utilization.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `RiskEngine.getLiquidationBonus`, the protocol calculates `bonusCross` as `Math.min(balanceCross / 2, thresholdCross - balanceCross)`. This calculation assumes that `thresholdCross` (the maintenance margin requirement) is strictly greater than `balanceCross` (the user's collateral balance) whenever liquidation occurs.

However, a user can be deemed insolvent by `isAccountSolvent` even if `balanceCross >= thresholdCross` (i.e., total collateral value exceeds requirements) if the pool utilization is saturated (>90%). In this state, `_crossBufferRatio` drops to 0, disabling cross-collateralization. A user with a surplus in one token and a deficit in another is marked insolvent despite having positive net equity.

When such a user is liquidated, `getLiquidationBonus` is called. Since `balanceCross >= thresholdCross`, the subtraction `thresholdCross - balanceCross` underflows and reverts (Solidity 0.8+ default checked math), causing the liquidation transaction to fail. This prevents the protocol from closing risky positions in high-utilization states.

## Impact
Insolvent users (by protocol rules) cannot be liquidated during high utilization periods if their total collateral value exceeds the requirement. This prevents the protocol from mitigating risk and can lead to the accumulation of bad debt if market prices move further against the user.

## Command to Run Test


## Proof of Concept
1. Deploy a Panoptic Pool and saturate utilization (>90%) by borrowing most of Token A.
2. User deposits 2000 units of Token B.
3. User sells a Put option that requires 1000 units of Token A as collateral (represented as borrowed Token A).
4. User holds 2000 Token B, Owes 1000 Token A. Assuming 1:1 price, `balanceCross` (2000) > `thresholdCross` (1000).
5. Due to saturation, `_crossBufferRatio` is 0. Solvency check `balanceA (0) >= reqA (1000)` fails. User is insolvent.
6. Liquidator calls `liquidate`.
7. `RiskEngine.getLiquidationBonus` executes. `thresholdCross (1000) - balanceCross (2000)` underflows and reverts.
8. Liquidation is impossible.

## Proof of Code
function testLiquidationDoS() public {
    // Setup pool and saturate utilization to >90%
    // ... setup code ...
    
    // Create victim with imbalanced but net-positive portfolio
    // Victim collateral: 2000 Token B
    // Victim liability: 1000 Token A
    // Total Value: +1000 equivalent
    
    // Assert victim is insolvent due to utilization cap
    assertFalse(panopticPool.isAccountSolvent(victim));
    
    // Attempt liquidation
    vm.prank(liquidator);
    vm.expectRevert(); // Underflow / Panic(0x11)
    panopticPool.liquidate(victim, positions);
}

## Suggested Mitigation
In `RiskEngine.getLiquidationBonus`, use a saturating subtraction or check if `balanceCross > thresholdCross`. If the user has sufficient total collateral (`balanceCross >= thresholdCross`), `bonusCross` should effectively be zero (or a minimal fee), and the protocol should proceed to use the user's collateral to cover the deficit without paying an additional insolvency bonus.


## [H-23]. Force Exercise Fee Inversion via Tick Manipulation

## id: hJ3xch2y8-G7cBRLcq0J_

## Derived From Pattern/Invariant
Third-party operator calling dispatchFrom()

## Exploit Type
FlashLoanEconomicManipulation

## Location
RiskEngine.sol.exerciseCost

## Finding Status: Valid
### Finding Status Justification: RiskEngine.exerciseCost() starts from a negative base fee (cost) but then subtracts (currentValue-oracleValue) for each long leg, which can add a positive amount if currentValue < oracleValue. There is no clamp ensuring the resulting fee remains <= 0. PanopticPool._forceExercise then passes the resulting signed fees into getRefundAmounts and finally CollateralTracker.refund(account, exercisor, assetsDelta). If fees become positive, refund() transfers shares from the exercisee (victim) to the exercisor, inverting the intended payment direction. The only bound is abs(currentTick-twapTick) <= tickDeltaLiquidation in dispatchFrom, but that does not guarantee sign safety. This is directly exploitable and can result in theft.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The force exercise fee calculation in `RiskEngine.exerciseCost` subtracts the price delta (`currentValue - oracleValue`) from the base fee. Since the base fee is negative (a cost to the exercisor), an attacker can manipulate the spot price (`currentTick`) such that `currentValue` is significantly lower than `oracleValue`. This makes the fee positive. `CollateralTracker.refund` then transfers this positive amount from the victim to the exercisor, allowing the attacker to steal funds from the victim.

## Impact
Direct theft of user collateral during force exercise.

## Command to Run Test


## Proof of Concept
1. Attacker targets a long position. 2. Attacker flash-manipulates pool price so leg value drops below oracle value. 3. Attacker calls `forceExercise`. 4. `exerciseCost` returns positive value. 5. `refund` transfers funds from victim to attacker.

## Proof of Code
function testForceExerciseInversion() public { ... }

## Suggested Mitigation
Ensure `exerciseFees` cannot be positive (capped at 0 or strictly a cost) or use `abs` logic correctly to prevent inversion.


## [H-24]. Force Exercise Fee Evasion via Oracle Manipulation

## id: K7jfAeqSO-ynETSHeYANZ

## Derived From Pattern/Invariant
Force exercisor targeting stuck longs

## Exploit Type
Oracle

## Location
RiskEngine.sol.exerciseCost

## Finding Status: Valid
### Finding Status Justification: The abs(currentTick - twapTick) <= tickDeltaLiquidation bound does not prevent shifting a near-boundary long leg just out of range; it only limits how far price can be moved, so fee-regime flipping can still be feasible for near-the-money positions.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
An attacker can manipulate the `currentTick` to make an In-The-Money (ITM) leg appear Out-Of-The-Money (OTM) relative to the strike. `RiskEngine.exerciseCost` determines the fee based on whether the leg is in-range using `currentTick`. If manipulated OTM, the fee drops to `ONE_BPS` instead of the higher `FORCE_EXERCISE_COST`, allowing the attacker to force-close the victim's position cheaply.

## Impact
Victim is forced out of position without receiving fair compensation.

## Command to Run Test


## Proof of Concept
1. Attacker manipulates price OTM. 2. Calls `forceExercise`. 3. Pays tiny fee.

## Proof of Code
function testFeeEvasion() public { ... }

## Suggested Mitigation
Use `oracleTick` or a combination of checks to determine moneyness for fee calculation.


## [H-25]. Force Exercise Fee Inversion via Tick Manipulation

## id: 0O2-bAfDpB-KERzXpGikE

## Derived From Pattern/Invariant
Force exercisor targeting stuck longs

## Exploit Type
FlashLoanEconomicManipulation

## Location
RiskEngine.exerciseCost

## Finding Status: Valid
### Finding Status Justification: Duplicate of hJ3xch2y8-G7cBRLcq0J_. RiskEngine.exerciseCost can become positive due to subtracting (current-oracle) value deltas, and PanopticPool then settles via CollateralTracker.refund in a way that makes positive values transfer from victim to exercisor.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `exerciseCost` formula subtracts the difference between current value and oracle value from the base fee: `fee = base - (current - oracle)`. If an attacker manipulates the price such that `current << oracle`, the term `-(negative_diff)` adds a large positive value to the fee. In `PanopticPool._forceExercise`, a positive fee triggers a refund *from* the exercisee (victim) *to* the exercisor (attacker). An attacker can steal funds from a victim by force-exercising them at a manipulated price.

## Impact
Theft of user collateral by force-exercising at manipulated prices.

## Command to Run Test


## Proof of Concept
1. Attacker manipulates price to crash option value (`current` ~ 0).
2. Oracle value is still high.
3. `exerciseCost` becomes positive.
4. Attacker calls `forceExercise`.
5. Victim pays Attacker the positive fee.

## Proof of Code
function testFeeInversion() public {
  // manipulate price
  int256 fee = riskEngine.exerciseCost(...);
  assertGt(fee, 0);
}

## Suggested Mitigation
Cap the exercise fee logic or ensure the dynamic component cannot invert the fee direction (payment flow).


## [M-26]. Force Exercise Fee Evasion via Oracle Manipulation

## id: jmzdpSeozlogOnIfTwSS-

## Derived From Pattern/Invariant
Force exercisor targeting stuck longs

## Exploit Type
FlashLoanEconomicManipulation

## Location
RiskEngine.exerciseCost

## Finding Status: Valid
### Finding Status Justification: As with K7jfAeqSO-ynETSHeYANZ, the twap-vs-current bound is not sufficient to prevent pushing borderline legs out of range within the allowed tick band, so it is not a complete safeguard.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The fee for force exercising a position is determined by `RiskEngine.exerciseCost`, which checks if the position legs are in-range using `currentTick`. If the legs are out-of-range, the fee drops to 1 basis point. An attacker can manipulate `currentTick` to be just outside the position's range (but within the 5% oracle deviation limit) to force exercise a position cheaply, bypassing the intended ~1% fee for in-range/near-range positions.

## Impact
Loss of value for position holders; attackers can close others' positions without paying the fair compensation fee.

## Command to Run Test


## Proof of Concept
1. Position is Near-The-Money. Fee should be high. 
2. Attacker moves `currentTick` slightly OTM. 
3. `exerciseCost` calculates 1bps fee. 
4. Attacker exercises cheaply.

## Proof of Code
function testFeeEvasion() public { // Move tick 
 int256 fee = riskEngine.exerciseCost(...); 
 assertEq(fee, 1 bps); }

## Suggested Mitigation
Use the Oracle tick (TWAP) or a composite of Oracle/Spot to determine if a position is in-range for fee calculations.





Finding Status: InvalidOutOfScope
## [H-27]. Unincentivized Liquidation and Seller Loss due to Insolvency Masking in RiskEngine

## id: mkv5-p-4_F2SSfI9ha4Tz

## Derived From Pattern/Invariant
Large position holder near insolvency

## Exploit Type
IncentiveMisalignmentOrGameTheory

## Location
RiskEngine.sol._getMargin

## Finding Status: InvalidOutOfScope
### Finding Status Justification: RiskEngine._getMargin() explicitly masks the “interest > balance” case by setting interest=balance and balance=0 (see the two if-blocks after ct.assetsAndInterest(user)). This can materially change downstream liquidation math because the returned tokenData*.rightSlot becomes 0 even when the user has gross collateral, and only the capped interest is added into requirements.

That said, the contest scope explicitly lists Nethermind pre-contest issues about insolvency masking / bonus logic as OUT OF SCOPE (“Masking Insolvency Magnitude”, “Broken Bonus Calculations”). This reported behavior matches that out-of-scope category.

No code-level safeguard prevents this masking; it is the implemented behavior today and can affect liquidation incentives and haircut accounting, but it is excluded by scope.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `RiskEngine._getMargin`, if a user's interest debt exceeds their collateral balance (`interest > balance`), the function sets `interest = balance` and `balance = 0` to prevent underflow. This `tokenData` with 0 balance is passed to `getLiquidationBonus`. Consequently, `getLiquidationBonus` calculates the bonus based on a 0 balance, resulting in a 0 bonus for the liquidator. Furthermore, because the balance is reported as 0, the calculated protocol loss (`collateralRemaining`) is exaggerated (equal to the full debt paid). `haircutPremia` then claws back this exaggerated amount from sellers. The insolvent user retains their residual assets in `CollateralTracker` (since `settleLiquidation` burns shares based on the 0 bonus calculation but doesn't seize the remaining balance), while sellers are forced to cover the user's retained assets.

## Impact
Liquidators receive 0 rewards for liquidating deeply insolvent accounts, leading to bad debt accumulation. Option sellers suffer excess haircuts covering funds that the insolvent user keeps.

## Command to Run Test


## Proof of Concept
1. User has 10 USDC collateral but owes 100 USDC interest. 2. Liquidator calls `liquidate`. 3. `_getMargin` caps interest at 10 and sets reported balance to 0. 4. `getLiquidationBonus` sees 0 balance, returns 0 bonus. 5. `haircutPremia` sees loss of 100 (assuming full closure cost), haircuts sellers for 100. 6. `settleLiquidation` is called with 0 bonus. 7. User keeps their 10 USDC (as shares). Sellers pay 100. Liquidator pays gas for 0 reward.

## Proof of Code
function testLiquidationInsolvency() public {
    // Mock setup where interest > balance
    // call RiskEngine.getLiquidationBonus with tokenData having 0 balance
    // assert bonus is 0
}

## Suggested Mitigation
In `_getMargin`, allow returning negative solvency or separate the reporting of `balance` and `interest` without capping/zeroing, so `getLiquidationBonus` can see the actual remaining collateral and award it to the liquidator.


## [H-28]. Insufficient Premium Haircut due to Insolvency Masking in RiskEngine

## id: 7-JaU2-wPfV1tfktTWBaM

## Derived From Pattern/Invariant
Large position holder near insolvency

## Exploit Type
AccountingInvariantViolation

## Location
RiskEngine._getMargin

## Finding Status: InvalidOutOfScope
### Finding Status Justification: The masking logic in RiskEngine._getMargin (interest>balance => interest=balance, balance=0) exists and can cause the engine to under-represent the true deficit (interest beyond balance is not represented). Liquidation haircut logic relies on the computed collateralRemaining / protocol loss path from getLiquidationBonus and then haircutPremia, so masking can affect how much is clawed back.

However, these insolvency masking effects on protocol loss magnitude / haircut and bonus computations are explicitly listed as OUT OF SCOPE in the provided Nethermind pre-contest out-of-scope section.

No safeguard in the current code reports or carries forward the full deficit beyond the capped interest.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `RiskEngine._getMargin`, if a user's interest debt exceeds their collateral balance, the interest requirement is capped to the balance and the balance is reported as zero. This logic effectively masks the magnitude of the insolvency. When `getLiquidationBonus` is called during liquidation, it uses this zeroed balance to calculate the protocol loss (collateral remaining). Consequently, the calculated loss is understated (it ignores the unpaid interest bad debt). `haircutPremia` uses this understated loss to determine how much premium to claw back from sellers. As a result, the protocol fails to haircut enough premium to cover the total bad debt (unpaid interest + liquidation deficit), leading to permanent protocol loss.

## Impact
Protocol bad debt is not fully covered by premium haircuts, leading to LP value loss.

## Command to Run Test


## Proof of Concept
1. User has 20 assets and owes 100 interest. 2. `_getMargin` sets req=20, bal=0. 3. Liquidator calls `liquidate`. 4. `getLiquidationBonus` sees balance 0. 5. Liquidator burns positions costing 10. `netPaid`=10. 6. `collateralRemaining` = -10. 7. `haircutPremia` claws back 10 from sellers. 8. The 80 unpaid interest is never recovered/haircut.

## Proof of Code
function testMasking() public { ... }

## Suggested Mitigation
Return the true negative balance or deficit in `_getMargin` and account for total debt in `haircutPremia` logic.


## [H-29]. Free Interest Accrual via Delegation Exploitation

## id: Vx4GLAfrsqayzNbAv_2aL

## Derived From Pattern/Invariant
CollateralTracker vault contract

## Exploit Type
IncentiveMisalignmentOrGameTheory

## Location
CollateralTracker.delegate/revoke

## Finding Status: InvalidOutOfScope
### Finding Status Justification: CollateralTracker.delegate/revoke behavior still allows temporary balance inflation to near `type(uint248).max` without increasing `_internalSupply`, while `_accrueInterest` can burn shares during that window and update the user borrow index. revoke can then restore `_internalSupply` when `balance < type(uint248).max`, undoing burns that happened while delegated. This is materially the same class of issue as the publicly listed Nethermind out-of-scope item “Orphan Shares in Delegate/Revoke”, so it is out of scope even if it results in interest not being economically paid by the user.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
A user can wipe their accrued interest obligation without paying assets by exploiting the `delegate` / `revoke` mechanism. When a user interacts with the pool (e.g., via `settlePremium`), the `PanopticPool` calls `delegate(user)`, inflating their share balance to `type(uint248).max`. 

Subsequently, `_accrueInterest` is triggered (e.g., via `settleBurn`). It calculates interest owed and burns shares from the user. Since the user has a massive delegated balance, the burn succeeds even if the user has no real collateral. `_accrueInterest` then updates the user's `borrowIndex`, marking the interest as paid. 

Finally, `revoke(user)` is called. It observes `balance < max` (due to the interest burn), sets the user's balance to 0 (or real balance), and increments `_internalSupply` by the burned amount to restore total supply. The net result is that the user's debt index is updated (interest paid) and the total supply is restored, but no real assets were paid and no real shares were permanently burned. The `unrealizedGlobalInterest` is reduced, socializing the loss to all lenders.

## Impact
Users can perpetually avoid paying interest on borrowed liquidity, effectively stealing yield from Panoptic Liquidity Providers (PLPs).

## Command to Run Test


## Proof of Concept
1. User A has open positions and 0 collateral (or withdraws collateral).
2. Interest accrues over time.
3. User A calls `PanopticPool.dispatch(settlePremium)` on themselves.
4. `delegate(User A)` sets balance to ~MAX.
5. `settleBurn` -> `_accrueInterest` burns phantom shares to pay interest. User's `borrowIndex` is updated.
6. `revoke(User A)` restores `_internalSupply` and resets user balance.
7. User A owes 0 interest and paid 0 assets.

## Proof of Code
function test_freeInterest() public {
    // Setup user with debt
    // Advance time
    uint256 interestOwed = collateralTracker.owedInterest(user);
    assertGt(interestOwed, 0);
    // User calls settlePremium
    vm.prank(user);
    panopticPool.dispatch(..., settlePremiumArgs, ...);
    // Verify interest wiped
    assertEq(collateralTracker.owedInterest(user), 0);
    // Verify user balance unchanged/zero
}

## Suggested Mitigation
In `revoke`, track the amount of phantom shares burned for interest separately or revert/require payment if `balance < max` due to interest accrual. Alternatively, ensure `_accrueInterest` cannot burn delegated phantom shares.


## [H-30]. Liquidation bonus evaluates to zero for deeply insolvent accounts due to masked collateral balance

## id: jCtSolPrKi9fe2fWD_T72

## Derived From Pattern/Invariant
liquidationBonusUsesGrossCollateral == true

## Exploit Type
AccountingInvariantViolation

## Location
RiskEngine.sol.getLiquidationBonus

## Finding Status: InvalidOutOfScope
### Finding Status Justification: RiskEngine._getMargin() sets balance=0 when interest>balance and only adds the capped interest into requirements. PanopticPool._liquidate calls riskEngine.getMargin(...) to produce tokenData0/tokenData1, and then RiskEngine.getLiquidationBonus uses tokenData*.rightSlot() (after subtracting shortPremium) as the basis for cross-balance and bonus calculation. If rightSlot is 0, the computed bonusCross can be 0, yielding a 0 liquidation bonus in extreme cases.

This matches the report’s core mechanic.

However, this class of insolvency masking and its effect on bonus calculations is explicitly marked OUT OF SCOPE in the provided Nethermind pre-contest out-of-scope list (masking insolvency magnitude / broken bonus calculations).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `RiskEngine.getLiquidationBonus` function relies on `getMargin` to determine the user's collateral balance. In `_getMargin`, if the user's accrued interest exceeds their deposited balance (`interest > balance`), the function caps the reported interest at the balance amount and sets the reported balance to zero. Consequently, when `getLiquidationBonus` is called for such a deeply insolvent user, it sees a collateral balance of zero. This results in a calculated liquidation bonus of zero, even if the user still has significant gross collateral that should be seized to pay the liquidator. Liquidators receive no compensation for gas or effort, leading to a denial of service for liquidations of deeply insolvent accounts.

## Impact
Deeply insolvent accounts (where interest debt exceeds assets) cannot be liquidated because the incentive mechanism breaks (bonus = 0). This causes the protocol to hold onto bad debt/insolvent positions indefinitely instead of clearing them, and the remaining collateral in the account is effectively locked/stuck until it is slowly consumed by interest accrual.

## Command to Run Test


## Proof of Concept
1. Alice deposits 100 USDC.
2. Alice opens a position and accumulates 150 USDC in interest debt over time (Deep Insolvency).
3. Bob tries to liquidate Alice.
4. `RiskEngine` calls `_getMargin`. Since 150 > 100, it sets `interest=100`, `balance=0`.
5. `RiskEngine` calculates bonus based on `balance=0`. Bonus evaluates to 0.
6. `PanopticPool` settles liquidation with 0 bonus.
7. Bob pays gas for the transaction but receives 0 USDC reward. Bob loses money and has no incentive to liquidate.

## Proof of Code
function test_ZeroBonusDeepInsolvency() public {
    // Setup deeply insolvent user
    // ...
    (LeftRightSigned bonus, ) = riskEngine.getLiquidationBonus(...);
    // Bonus should be > 0 given there is collateral, but it is 0
    assertEq(bonus.rightSlot(), 0);
}

## Suggested Mitigation
In `RiskEngine.getLiquidationBonus`, ensure that the bonus is calculated based on the gross collateral balance (before interest deduction) or ensure a minimum liquidation fee is always paid from the remaining collateral, regardless of the `interest > balance` state.


## [H-31]. Free Interest Accrual via Self-Settlement and Delegation Mechanism Abuse

## id: Xyvl37G9ODpa53LuElv2n

## Derived From Pattern/Invariant
CollateralTracker vault contract: Free Interest Accrual via Delegation exploitation

## Exploit Type
IncentiveMisalignmentOrGameTheory

## Location
CollateralTracker.sol.delegate / revoke

## Finding Status: InvalidOutOfScope
### Finding Status Justification: CollateralTracker.delegate/revoke still enables “phantom share” flows: delegate inflates `balanceOf[delegatee]` near `type(uint248).max` without increasing `_internalSupply`; during the delegated period, `_accrueInterest` can `_burn` shares from that inflated balance (success path) and update the user borrow index as if interest was paid. Then revoke can restore `_internalSupply` by `type(uint248).max - balance` when `balance < type(uint248).max`, effectively undoing burns that occurred while delegated. This family of delegate/revoke share-invariant/interest-payment behaviors is explicitly listed as “Additional Findings from Nethermind pre-contest - ALL OUT OF SCOPE” (Orphan Shares in Delegate/Revoke), so it is out of scope even if exploitable.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `delegate()` function allows the `PanopticPool` to artificially inflate a user's share balance to `type(uint248).max`. When `_accrueInterest` is triggered subsequently (e.g., during `settleBurn`), if the user owes interest, the system burns shares. Since the user has an infinite delegated balance, the burn succeeds (Solvent path). However, `revoke()` later restores the supply by calculating `MAX - currentBalance`. This effectively allows the user to pay off real interest debt using the temporary phantom shares. A user can exploit this by approving themselves and calling `dispatchFrom` (settle mode) on their own position, triggering `_settlePremium` -> `delegate` -> `_accrueInterest` -> `revoke`, wiping their interest debt at no cost.

## Impact
Users can evade interest payments, leading to yield theft from LPs and protocol insolvency.

## Command to Run Test


## Proof of Concept
1. User accrues significant interest debt.
2. User approves themselves as an operator.
3. User calls `dispatchFrom(self, self, positions, positions)` (triggering `_settlePremium`).
4. `PanopticPool` calls `delegate(user)`. Balance becomes ~MAX.
5. `_settlePremium` calls `_settleOptions` -> `settleBurn` -> `_accrueInterest`.
6. Interest is paid by burning phantom shares. User index is updated to current.
7. `revoke(user)` resets balance and adjusts supply. Interest debt is gone.

## Proof of Code
function testFreeInterest() public {
    // Advance time to accrue interest
    vm.warp(block.timestamp + 365 days);
    uint256 debtBefore = tracker.owedInterest(user);
    vm.prank(user);
    pool.dispatchFrom(empty, user, pos, pos, params);
    uint256 debtAfter = tracker.owedInterest(user);
    assertGt(debtBefore, 0);
    assertEq(debtAfter, 0);
}

## Suggested Mitigation
In `revoke`, track the amount of phantom shares burned (interest paid) and require the user to repay that amount in real shares or assets before resetting the balance, or disallow `delegate` usage for self-settlement flows.


## [H-32]. Permanent supply inflation and share value dilution due to accounting error in insolvent account delegation

## id: fymuEyrsFZg3fqxs0Mds2

## Derived From Pattern/Invariant
CollateralTracker Asset Accounting

## Exploit Type
AccountingInvariantViolation

## Location
CollateralTracker.delegate

## Finding Status: InvalidOutOfScope
### Finding Status Justification: CollateralTracker.delegate/revoke can create supply/balance inconsistencies by inflating balances without increasing `_internalSupply`, allowing burns during delegation, and then restoring `_internalSupply` during revoke. This aligns with the publicly disclosed Nethermind pre-contest out-of-scope issue “Orphan Shares in Delegate/Revoke” describing orphaned shares / totalSupply mismatch. As it is explicitly listed as out of scope, it should not be treated as a valid contest vulnerability despite the behavior existing in code.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
When a user is insolvent (owed interest > balance), `CollateralTracker.delegate` sets their balance to `type(uint248).max` by adding `type(uint248).max - balance`. This effectively hides the user's real balance `X` without removing it from `_internalSupply`. During liquidation, `_accrueInterest` burns shares `Y` from this inflated balance, decreasing `_internalSupply` by `Y`. Finally, `revoke` observes the balance drop and restores `Y` to `_internalSupply`. The net change to `_internalSupply` is 0. However, the user's original `X` shares (which were effectively consumed to pay debt) remain in `_internalSupply` but are no longer in any user's `balanceOf`. This results in `totalSupply > sum(balances)`, permanently inflating supply and diluting the value of all other shares.

## Impact
Permanent inflation of share supply leading to value dilution for all liquidity providers.

## Command to Run Test


## Proof of Concept
1. User has 100 shares. 2. User becomes insolvent (owes 200 interest). 3. `liquidate` calls `delegate`. 4. `delegate` sees insolvency, adds `MAX - 100`. Balance becomes `MAX`. `_internalSupply` unchanged. 5. `_accrueInterest` burns 200 shares. Balance `MAX - 200`. `_internalSupply` decreases by 200. 6. `revoke` sees balance `MAX - 200`. Adds 200 to `_internalSupply`. 7. Final `_internalSupply` change is 0. User balance is 0. The original 100 shares are now 'orphan' shares in the supply.

## Proof of Code
function testOrphanShares() public {
    // Setup insolvent user
    // ... 
    uint256 startSupply = ct.totalSupply();
    ct.delegate(user);
    // simulate interest accrual burning shares
    vm.prank(address(pool));
    ct.transfer(address(0), 200);
    ct.revoke(user);
    assertGt(ct.totalSupply(), startSupply - 100); // Supply didn't decrease by user's collateral
}

## Suggested Mitigation
In `delegate`, if balance is consumed, explicitly burn the consumed amount from `_internalSupply` or track it to be burned during revoke.


## [H-33]. Free Interest Accrual via Delegation exploitation

## id: O5rqZgAuhxSOZG_vFHhd1

## Derived From Pattern/Invariant
CollateralTracker vault contract

## Exploit Type
IncentiveMisalignmentOrGameTheory

## Location
CollateralTracker.delegate

## Finding Status: InvalidOutOfScope
### Finding Status Justification: The delegate/revoke mechanism can allow interest settlement burns to be economically neutralized: `_accrueInterest` burns shares while the user has an inflated delegated balance, then revoke can restore `_internalSupply` for the delta when `balance < type(uint248).max`. This undermines the intended transfer of value from borrowers to lenders. However, delegate/revoke orphan-share and related accounting effects are explicitly disclosed as out of scope in the “Additional Findings from Nethermind pre-contest” section.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `delegate` mechanism in `CollateralTracker` allows a user to temporarily hold infinite shares. If `_accrueInterest` is triggered during delegation (e.g., via `_settlePremium` or `_forceExercise`), it burns shares from this infinite balance to pay interest debt. The `revoke` function then restores `_internalSupply` based on the difference, effectively undoing the burn. The user's `borrowIndex` is updated as if they paid, but no real assets or shares were consumed.

## Impact
Users can wipe their accrued interest debt to zero without paying, effectively stealing yield from liquidity providers.

## Command to Run Test


## Proof of Concept
1. User accrues significant interest debt.
2. User calls `dispatchFrom` to `settlePremium` on themselves (or is force exercised).
3. `_settlePremium` calls `delegate(user)`.
4. `_settleOptions` -> `settleBurn` -> `_accrueInterest` is triggered.
5. `_accrueInterest` burns phantom shares and updates `userBorrowIndex` (marking interest paid).
6. `revoke` restores the burned amount to `_internalSupply`.
7. User debt is gone, protocol gained nothing.

## Proof of Code
function testFreeInterest() public {
    // Accrue interest, delegate, accrue again, revoke
}

## Suggested Mitigation
In `CollateralTracker`, ensure that `_accrueInterest` burns real shares first or track interest payments separately during delegation and require actual settlement before revoking.


## [H-34]. Protocol Value Leakage via Phantom Share Interest Burn in CollateralTracker

## id: ZU4fL9EC-8x6nvMntD6Bk

## Derived From Pattern/Invariant
CollateralTracker0 vault contract

## Exploit Type
AccountingInvariantViolation

## Location
CollateralTracker.sol.revoke

## Finding Status: InvalidOutOfScope
### Finding Status Justification: CollateralTracker.delegate() inflates balanceOf by ~type(uint248).max (minus balanceConsumedByInterest), and revoke() restores _internalSupply if phantom shares were consumed. This pattern can “undo” burns performed during delegation (including interest burns), causing accounting/supply inconsistencies consistent with the described effect. However, this entire delegate/revoke orphan/ghost share behavior is explicitly listed as an out-of-scope pre-contest known issue (“Orphan Shares in Delegate/Revoke”). Therefore, even if the bug exists and is exploitable in principle, it must be marked out of scope for this audit.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Users can evade interest payments by delegating phantom shares to themselves. When interest accrues, `_accrueInterest` burns shares (including phantom ones) from the user's balance and reduces `_internalSupply`. However, when `revoke` is called, the logic incorrectly restores the full amount of phantom shares to `_internalSupply` even if some were burned to pay interest. This effectively un-burns the interest payment, reversing the value transfer to lenders and diluting the share price.

## Impact
Lenders lose yield as interest payments made with phantom shares are effectively reverted upon revocation.

## Command to Run Test


## Proof of Concept
1. User has 100 shares. 2. User calls `delegate(self)`. Balance becomes ~2^248 + 100. 3. Interest accrues; `_accrueInterest` burns 50 shares. `_internalSupply` decreases. 4. User calls `revoke(self)`. Logic sees `balance < max` and adds `max - balance` back to `_internalSupply`. 5. `_internalSupply` increases by the amount burned, negating the interest payment's value accretion to other lenders.

## Proof of Code
function testInterestEvasion() public { ... }

## Suggested Mitigation
In `revoke`, do not restore `_internalSupply` for the portion of phantom shares that were consumed by interest burns.


## [H-35]. DoS of liquidations for insolvent accounts due to zeroed liquidation bonus

## id: 9qNoSQmGuyPkSitPo6agE

## Derived From Pattern/Invariant
RiskEngine Liquidation Bonuses

## Exploit Type
AccountingInvariantViolation

## Location
RiskEngine.getLiquidationBonus

## Finding Status: InvalidOutOfScope
### Finding Status Justification: The behavior stems from the interest-capping/deficit-masking logic, but calling it 'by design therefore not a vulnerability' is not inherently correct; it is an acknowledged tradeoff/bug class. It is, however, explicitly listed as out-of-scope (Masking Insolvency Magnitude / Broken Bonus Calculations).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `RiskEngine.getMargin`, if a user owes more interest than their balance, the interest is capped at the balance and the balance is set to 0. This zeroed balance is then passed to `getLiquidationBonus`. The liquidation bonus calculation depends on this balance (`balanceCross`). Since it is 0, the calculated bonus is 0. Liquidators are expected to receive a bonus to cover gas and provide profit. When liquidating insolvent users (`interest > balance`), liquidators receive 0 bonus but still incur gas costs and potentially swap fees. Rational liquidators will not liquidate these accounts, leading to bad debt accumulation.

## Impact
Insolvent accounts cannot be liquidated profitably, leading to protocol bad debt.

## Command to Run Test


## Proof of Concept
1. User has 10 tokens, owes 20 interest. 2. `getMargin` sets user balance to 0, interest to 10. 3. `isAccountSolvent` returns false. 4. `getLiquidationBonus` is called with balance 0. 5. It returns 0 bonus. 6. `PanopticPool._liquidate` pays 0 to liquidator. 7. Liquidator loses gas fees.

## Proof of Code
function testLiquidationBonusZero() public {
    // Setup insolvent user
    // ...
    (LeftRightSigned bonus, ) = riskEngine.getLiquidationBonus(tokenDataZeroBalance, ...);
    assertEq(bonus.rightSlot(), 0);
}

## Suggested Mitigation
In `getLiquidationBonus`, ensure that the bonus calculation accounts for the total collateral available before interest deduction, or ensure the protocol covers the liquidator's incentive even if the user balance is zero.


## [H-36]. Ghost Shares created in CollateralTracker due to incorrect revoke logic during insolvency

## id: CsgBpwGlMPadEKDayYXgs

## Derived From Pattern/Invariant
Balance Invariant: totalSupply() decreases by the amount of real shares consumed for interest during delegation

## Exploit Type
AccountingInvariantViolation

## Location
CollateralTracker.revoke

## Finding Status: InvalidOutOfScope
### Finding Status Justification: The described delegate/revoke + interest burn interaction is consistent with the existing CollateralTracker.delegate() / revoke() logic that can restore _internalSupply when phantom shares were consumed. This matches the explicit pre-contest “Orphan Shares in Delegate/Revoke” known issue included in the provided out-of-scope list. Therefore, despite the bug being real and potentially high impact, it is out of scope for this audit per the competition rules.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
When a user with `interest > balance` (insolvent) is delegated (e.g., during liquidation), `delegate()` calculates `balanceConsumedByInterest` to be their full balance `B`, effectively setting the delegation baseline to consume real shares first. Inside `delegate`, the balance is set to `type(uint248).max` (Phantom Supply `P`), but the addition is `P - B`. When `_accrueInterest` runs, it burns `shares` (equal to interest `I`, where `I > B`) from this inflated balance. `revoke()` then detects the balance decrease and restores `_internalSupply` by `shares` (since `P - (P - I) = I`). 

However, the user effectively paid `I` using `B` real shares and `I - B` phantom shares. The correct behavior should be to reduce `totalSupply` by `B` (the real shares consumed). The current logic restores the full `shares` amount to `totalSupply`, resulting in `totalSupply` remaining constant while the user's balance is wiped to 0. This creates `B` 'Ghost Shares'—shares that exist in `totalSupply` but are not owned by any account—permanently diluting the share price (`totalAssets/totalSupply`) and causing loss to all other Panoptic Liquidity Providers (PLPs).

## Impact
Permanent inflation of `totalSupply` relative to actual balances (`totalSupply > sum(balanceOf)`). This dilutes the share value for all PLPs by the amount of the insolvent user's collateral, failing to capture the value of the liquidated collateral to cover bad debt.

## Command to Run Test


## Proof of Concept
1. Alice deposits collateral and receives `B` shares.
2. Market conditions change such that Alice owes interest `I > B`.
3. A liquidator triggers liquidation (or force exercise) on Alice.
4. `PanopticPool` calls `delegate(Alice)` on the CollateralTracker.
   - `delegate` sees `I > B` (via `previewWithdraw(_owedInterest)`), so it sets `balanceConsumedByInterest = B`.
   - It adds `P - B` to Alice's balance. New Balance = `P`.
5. `PanopticPool` settlement logic calls `_accrueInterest(Alice)`.
   - Interest owed is `I`.
   - Since `Balance (P) > I`, it burns `I` shares. `totalSupply` decreases by `I`. Alice's balance becomes `P - I`.
6. `PanopticPool` calls `revoke(Alice)`.
   - `revoke` sees `balance (P - I) < P`. It calculates restoration amount `P - (P - I) = I`.
   - It adds `I` back to `_internalSupply`.
   - It sets Alice's balance to 0.
7. **Result:** Net change to `totalSupply` is `-I + I = 0`. Net change to sum of balances is `-B`. The `B` shares that were burned from Alice are effectively restored to the supply without an owner.

## Proof of Code
/** 
 * Add this to a test file (e.g. CollateralTrackerTest.t.sol) inheriting from proper test setup 
 */
function test_GhostSharesExploit() public {
    // Setup: Create a user with balance B
    uint256 B = 100 ether;
    address alice = address(0x1);
    
    vm.prank(address(collateralTracker)); // Mint via internal/harness or deposit
    collateralTrackerMock.mint(alice, B);

    // Simulate conditions where Interest (I) > B
    // For test simplicity, we mock _owedInterest or manipulate state directly
    uint256 I = 150 ether;
    collateralTrackerMock.setOwedInterest(alice, I);

    // Pre-state checks
    uint256 startSupply = collateralTrackerMock.totalSupply();
    assertEq(collateralTrackerMock.balanceOf(alice), B);

    // 1. Delegate
    vm.prank(address(panopticPool));
    collateralTrackerMock.delegate(alice);
    // Balance is now P (type(uint248).max)
    assertEq(collateralTrackerMock.balanceOf(alice), type(uint248).max);

    // 2. Accrue Interest (Burns I)
    vm.prank(address(panopticPool)); // masquerade as pool for permissioned calls if needed
    collateralTrackerMock.accrueInterest(alice);
    // Supply decreased by I
    assertEq(collateralTrackerMock.totalSupply(), startSupply - I);
    
    // 3. Revoke
    vm.prank(address(panopticPool));
    collateralTrackerMock.revoke(alice);

    // Post-state analysis
    uint256 endSupply = collateralTrackerMock.totalSupply();
    uint256 endBalance = collateralTrackerMock.balanceOf(alice);

    // Alice is wiped out
    assertEq(endBalance, 0);

    // INVARIANT VIOLATION:
    // Expected: Supply should decrease by B (the real collateral consumed)
    // Actual: Supply returns to startSupply (net change 0)
    // Ghost Shares = B
    assertEq(endSupply, startSupply);
    assertEq(endSupply, collateralTrackerMock.realTotalSupply() + B); // assuming realTotalSupply tracks sum of balances
}

## Suggested Mitigation
Modify `delegate` to always add `type(uint248).max` regardless of solvency, rather than subtracting `balanceConsumedByInterest`. This ensures `revoke` logic correctly calculates the net change. 

Change line in `delegate`:
```solidity
// Remove balanceConsumedByInterest logic
balanceOf[delegatee] += type(uint248).max;
```
With this change:
- Delegate: Bal = `B + P`.
- Accrue (Burn `I`): Bal = `B + P - I`.
- Revoke: Restore `P - (B + P - I) = I - B`.
- Net Supply Change: `-I + (I - B) = -B`. Correct.


## [H-37]. Protocol Value Leakage via Phantom Share Interest Burn

## id: 69pFANkblvkmA5AYAK9xP

## Derived From Pattern/Invariant
CollateralTracker0 vault contract

## Exploit Type
AccountingInvariantViolation

## Location
CollateralTracker.revoke

## Finding Status: InvalidOutOfScope
### Finding Status Justification: Same delegate/revoke phantom-share supply restoration behavior as ZU4fL9EC-8x6nvMntD6Bk/CsgBpwGlMPadEKDayYXgs, and explicitly listed as an out-of-scope pre-contest known issue (“Orphan Shares in Delegate/Revoke”).
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
When a user delegates collateral (Phantom Shares), their balance is artificially inflated. If `_accrueInterest` is triggered, it burns shares from this inflated balance. Since `balance > shares_to_burn`, it is treated as a full payment and the borrow index is updated. However, the `revoke` function restores the *full* amount of phantom shares to `_internalSupply` (`type(uint248).max - balance`), effectively 'un-burning' the shares used for interest. The protocol considers the interest paid, but the total supply is restored, diluting PLPs.

## Impact
Interest payments made during delegation are effectively reverted upon revocation, causing loss of yield for PLPs.

## Command to Run Test


## Proof of Concept
1. User delegates shares.
2. Accrues interest. `_accrueInterest` burns X shares.
3. User calls `revoke`.
4. `revoke` logic adds X back to `_internalSupply` because `balance` was reduced by X.
5. Result: User debt settled, but share supply not reduced.

## Proof of Code
function testPhantomBurn() public {
  collateralTracker.delegate(user);
  // Accrue interest
  collateralTracker.revoke(user);
  // Check totalSupply is same as start, but user index updated
}

## Suggested Mitigation
Track the amount of 'real' shares burned during delegation and do not restore them in `revoke`.





Finding Status: LowSeverityDueToRareLikelihood + InvalidNotExploitable
## [M-38]. Permanent DoS of Pool Initialization via Collision Pre-computation

## id: XR33Y8Rj_swgGor-AIbNT

## Derived From Pattern/Invariant
PoolId collision griefer via many pool creations

## Exploit Type
Dos

## Location
SemiFungiblePositionManager.initializeAMMPool

## Finding Status: LowSeverityDueToRareLikelihood + InvalidNotExploitable
### Finding Status Justification: The unbounded loop is a current code path; what is speculative is feasibility of grinding many consecutive 40-bit suffix collisions, not the existence of the condition itself. Thus labeling it as only a future/post-upgrade state is inaccurate.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `initializeAMMPool` function handles `poolId` collisions (where the bottom 40 bits match) by incrementing the pool pattern in a loop. 

```solidity
while (s_poolIdToKey[poolId].tickSpacing != 0) {
    poolId = PanopticMath.incrementPoolPattern(poolId);
}
```

An attacker can pre-calculate a sequence of Uniswap PoolKeys that hash to a specific 40-bit suffix sequence (`S`, `S+1`, `S+2`...). By initializing these pools, the attacker occupies the collision resolution path for a victim's target pool. When the victim attempts to initialize their pool, the transaction runs out of gas looping through the attacker's occupied slots.

## Impact
Permanent Denial of Service for specific high-value pools, preventing them from being used in the protocol.

## Command to Run Test


## Proof of Concept
1. Attacker identifies the hash suffix `S` for the Victim's target pool.
2. Attacker finds inputs that map to `S`, `S+1`, `S+2`, ... `S+N`.
3. Attacker calls `initializeAMMPool` for all of them.
4. Victim calls `initializeAMMPool`.
5. Victim's transaction loops `N` times and reverts (OOG).

## Proof of Code
function testPoolIdDoS() public {
    // Simulate occupied slots
    // Assert OOG on next init
}

## Suggested Mitigation
Limit the number of collision resolution attempts (e.g., to 10) and revert if exceeded, or increase the entropy of the poolId to make collisions infeasible.





Finding Status: LowSeverityDueToRareLikelihood + InvalidByDesign
## [M-39]. Interest Rate Stagnation in Inactive Pools

## id: qu5MW5Womio64VA8_Hqso

## Derived From Pattern/Invariant
Borrower

## Exploit Type
IncentiveMisalignmentOrGameTheory

## Location
RiskEngine.updateInterestRate

## Finding Status: LowSeverityDueToRareLikelihood + InvalidByDesign
### Finding Status Justification: The capped-elapsed behavior is real (rateAtTarget update ignores long inactivity beyond 4096s), so the phenomenon exists. The cap is not a safeguard against the described undercharging; it is the cause. Borrowers can strategically benefit by keeping an otherwise-high-util pool inactive and realizing a lower adjusted rate later. The potential interest undercharge over long inactivity can be large, so impact is not inherently low.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `RiskEngine` updates the adaptive interest rate based on time elapsed since the last interaction. However, `IRM_MAX_ELAPSED_TIME` caps this elapsed time to ~1.1 hours (4096 seconds). 

If a pool has no interactions for a long period (e.g., a month), the interest rate adjustment mechanism only accounts for 1 hour of drift. If utilization was high (e.g., 100%) during this entire period, the interest rate should have increased significantly to curb demand. Due to the cap, the rate stays artificially low, allowing borrowers to pay minimal interest in illiquid/inactive pools.

## Impact
LPs earn insufficient yield in high-utilization inactive pools; Borrowers exploit stagnant low rates.

## Command to Run Test


## Proof of Concept
1. Create pool, borrow to 100% utilization.
2. Warp time 30 days.
3. Trigger interest update.
4. Rate increases only by a small increment corresponding to 1 hour of high utilization, instead of 30 days.

## Proof of Code
function test_irmStagnation() public {
    // maximize utilization
    // warp(30 days)
    // updateInterestRate
    // assert rate is much lower than expected PID growth
}

## Suggested Mitigation
Remove or significantly increase `IRM_MAX_ELAPSED_TIME`, or calculate the PID adjustment using the full elapsed time.


## [M-40]. Interest Rate Model Stagnation allows cheap borrowing on inactive pools

## id: 0_gRkuZ4erEZ3nrybert6

## Derived From Pattern/Invariant
Borrower: Interest Rate Stagnation in Inactive Pools

## Exploit Type
IncentiveMisalignmentOrGameTheory

## Location
RiskEngine._borrowRate

## Finding Status: LowSeverityDueToRareLikelihood + InvalidByDesign
### Finding Status Justification: The capped-elapsed update for rateAtTarget is real, so the described stagnation can occur. The cap is not a safeguard against cheap borrowing; it bounds rateAtTarget changes and can under-react after long inactivity. Borrowers can benefit by minimizing interactions. The economic impact can be significant for long inactivity periods with high utilization.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
The `RiskEngine` calculates interest rate updates using `_borrowRate`, which caps the `elapsed` time at `IRM_MAX_ELAPSED_TIME` (4096 seconds ~ 1.1 hours). If a pool remains inactive (no state-changing interactions) for a long period (e.g., 30 days) while having high utilization, the adaptive rate mechanism (`rateAtTarget`) will only advance by ~1 hour worth of adjustment instead of the full duration. This prevents the interest rate from spiking to punish long-term high utilization, allowing borrowers to pay significantly less interest than intended.

## Impact
Loss of yield for LPs; Borrowers can maintain high utilization positions for long periods without facing the intended exponential rate increase.

## Command to Run Test


## Proof of Concept
1. Attacker borrows heavily from a pool, pushing utilization above target.
2. Attacker waits for 30 days without interacting with the pool (assuming low activity).
3. Attacker calls `accrueInterest` or modifies position.
4. `_borrowRate` calculates `elapsed = min(30 days, 4096s)`.
5. The `rateAtTarget` increases only slightly (as if 1 hour passed).
6. Attacker pays interest based on this stagnated rate for the full 30 days.

## Proof of Code
function testInterestStagnation() public {
    // maximize utilization
    // warp 30 days
    vm.warp(block.timestamp + 30 days);
    // interact
    collateralTracker.accrueInterest();
    // check rateAtTarget is much lower than expected for 30 days of high util
}

## Suggested Mitigation
Remove the `IRM_MAX_ELAPSED_TIME` cap or implement a mechanism to calculate updates iteratively/analytically for long durations.





Finding Status: LowSeverityDueToRareLikelihood
## [M-41]. Liquidation Denial of Service due to _internalSupply underflow

## id: IeLenOQ-9VvwXvLZ4hYVG

## Derived From Pattern/Invariant
Invariant Type: Balance - amount <= _internalSupply

## Exploit Type
AccountingInvariantViolation

## Location
CollateralTracker._burn

## Finding Status: LowSeverityDueToRareLikelihood
### Finding Status Justification: ERC20Minimal._burn() (used by CollateralTracker) decrements `_internalSupply` with checked arithmetic. During liquidation the PanopticPool calls `CollateralTracker.delegate(liquidatee)` which inflates `balanceOf[liquidatee]` without increasing `_internalSupply`. Subsequent settlement steps can `_burn` large share amounts from this delegated balance. If a burn amount exceeds the current `_internalSupply` (possible under extreme bad-debt / very large share burn requirements), `_internalSupply -= amount` reverts, bricking the liquidation path before any later `revoke`/restoration logic can run. While reaching `amount > _internalSupply` likely requires extreme conditions, the revert is a hard DoS on liquidation when it occurs.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
During the liquidation of a deeply insolvent account where the loss (debt) exceeds the pool's `_internalSupply` (total assets deposited by LPs), the `_burn` function reverts. The liquidation process delegates phantom shares to the liquidatee to cover the burn, but `_burn` decrements the `_internalSupply` variable. If the amount to burn (the loss) is greater than `_internalSupply`, the checked arithmetic reverts. This prevents the liquidation of significant bad debt, potentially bricking the pool or leaving LPs with unrecognized losses.

## Impact
Liquidations fail for large insolvent positions, causing bad debt to accumulate and potentially freezing pool operations for affected accounts.

## Command to Run Test


## Proof of Concept
1. Create a pool with small LP liquidity (`_internalSupply` = 100).
2. User A opens a highly leveraged short position.
3. Market moves significantly; User A's loss becomes 200.
4. Liquidator calls `liquidate`.
5. `settleBurn` attempts to burn 200 shares from User A (using delegated phantom shares).
6. `_burn` tries `_internalSupply -= 200`.
7. Revert due to underflow (100 - 200).

## Proof of Code
function testLiquidationDoS() public {
    // Setup small pool
    // Create huge debt position
    // Attempt liquidation
    // Expect revert
}

## Suggested Mitigation
Modify `_burn` to allow `_internalSupply` to temporarily underflow (using a signed integer or separate tracking) during the delegate/revoke context, or cap the burn amount to available supply and socialize the remaining loss.


## [M-42]. Liquidation DoS due to arithmetic underflow when debt exceeds share supply

## id: GKSmJGEZc1BRpSujfaB81

## Derived From Pattern/Invariant
Invariant Type: Balance - CollateralTracker._burn

## Exploit Type
IntegerOverflow

## Location
CollateralTracker.settleBurn

## Finding Status: LowSeverityDueToRareLikelihood
### Finding Status Justification: In liquidation, `delegate()` inflates the liquidatee’s `balanceOf` without increasing `_internalSupply`. `settleBurn` ultimately burns shares (via `_burn`) to settle token deltas. ERC20Minimal._burn uses checked arithmetic on `_internalSupply -= amount`. If the burn amount (computed from the liquidation/close costs) exceeds `_internalSupply`, the burn reverts and liquidation cannot complete. No code path in `settleBurn`, `_updateBalancesAndSettle`, or `_burn` detects and handles `amount > _internalSupply`. This can permanently prevent liquidation under extreme loss scenarios.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
During the liquidation of a deeply insolvent account, `settleBurn` determines the amount of debt (tokens owed) and converts it to shares (`sharesToBurn`). It then calls `_burn` to destroy these shares. `_burn` decrements `_internalSupply`. Since `delegate` (called at the start of liquidation) increases the user's `balanceOf` (virtual shares) but does *not* increase `_internalSupply`, the `_internalSupply` remains equal to the real share supply. If the insolvent user's debt (in shares) exceeds the total real share supply (`_internalSupply`)—which can happen if the pool assets are low relative to the loss—the subtraction `_internalSupply -= amount` will underflow and revert. This prevents the liquidation from succeeding, leaving the bad debt in the system.

## Impact
Insolvency cannot be resolved for positions where the loss exceeds the pool's equity, leading to a permanent bad debt state and potential loss of funds for LPs as the position cannot be closed.

## Command to Run Test


## Proof of Concept
1. A pool exists with 1000 assets and corresponding `_internalSupply`. 2. A user holds a position that goes deeply underwater (loss > 1000 assets). 3. A liquidator calls `liquidate`. 4. The protocol calls `delegate`, giving the user virtual shares. 5. `settleBurn` is called; it calculates `sharesToBurn` corresponding to the >1000 asset loss. 6. `_burn` attempts `_internalSupply -= sharesToBurn`. 7. Revert due to underflow. Liquidation fails.

## Proof of Code
function testLiquidationDoS() public { 
    // Setup small pool 
    // Setup huge debt position 
    vm.expectRevert(stdError.arithmeticError); 
    panopticPool.dispatchFrom(liquidator, insolventUser, ...); 
}

## Suggested Mitigation
In `settleBurn` (or `_burn`), detect if `sharesToBurn > _internalSupply`. If so, burn all of `_internalSupply` (setting it to 0 or 1) and account for the remaining deficit as protocol loss to be handled in `settleLiquidation`.


## [M-43]. Permanent Denial of Service in Liquidation via Internal Supply Underflow

## id: 3AgeXt4y1mFP7MaXX2H5P

## Derived From Pattern/Invariant
Liquidator triggering settleLiquidation

## Exploit Type
EmergencyModeStateStuck

## Location
CollateralTracker.settleLiquidation

## Finding Status: LowSeverityDueToRareLikelihood
### Finding Status Justification: This is another manifestation of the same core issue: liquidation relies on delegated balances to allow burning/settlement, but `_burn` always decrements `_internalSupply`. If any burn amount during liquidation (interest/settlement/premium haircut) exceeds `_internalSupply`, the liquidation reverts and cannot complete. There is no compensating logic before the burn to cap the amount or to account for excess as protocol loss. While the condition is likely rare, it is a protocol-level DoS when triggered.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
Liquidation requires burning shares from the liquidatee to settle interest/premium debts. This burn is executed via `_burn`, which decrements `_internalSupply`. If the `_internalSupply` (real shares) is very low (e.g., due to PLP withdrawals) but the liquidatee has a large position (leveraged via phantom shares), the amount of shares required to be burned for debt settlement can exceed `_internalSupply`. `_burn` will revert on underflow, making liquidation impossible and leaving the protocol with bad debt.

## Impact
Inability to liquidate insolvent accounts; accumulation of bad debt.

## Command to Run Test


## Proof of Concept
1. LPs withdraw most funds; `_internalSupply` is low (e.g. 100). 2. User has open position, accumulates 200 shares worth of interest debt. 3. User becomes insolvent. 4. Liquidator calls `dispatchFrom` to liquidate. 5. Execution reaches `accrueInterest` -> `_burn`. 6. `_burn` tries `_internalSupply -= 200`. Reverts. 7. Liquidation fails.

## Proof of Code
function testLiquidationDoS() public { 
 // Drain pool 
 // Accumulate debt > supply 
 vm.expectRevert(); 
 panopticPool.dispatchFrom(...); // Liquidate 
 }

## Suggested Mitigation
Allow `_internalSupply` to underflow (handle as negative/debt) or cap the burn amount to available supply and socialize the remaining loss.


## [M-44]. Permanent Denial of Service in Liquidation via Internal Supply Underflow

## id: MVU-qy8uW6yKJHKNJLwPG

## Derived From Pattern/Invariant
Liquidator triggering settleLiquidation

## Exploit Type
Dos

## Location
CollateralTracker._accrueInterest

## Finding Status: LowSeverityDueToRareLikelihood
### Finding Status Justification: Liquidation flow delegates virtual shares (no `_internalSupply` increase) and then accrues/burns shares to settle interest via `_accrueInterest` (and other settlement burns). Any burn during delegation still decrements `_internalSupply` in ERC20Minimal._burn(). If the needed burn amount exceeds `_internalSupply`, liquidation reverts, preventing resolution of the insolvent account. There is no guard/cap in `_burn` or liquidation settlement to handle such a condition gracefully (e.g., socializing remaining loss). Preconditions are likely extreme, but when met this is a hard liquidation DoS.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
In `CollateralTracker`, `_internalSupply` tracks the supply of shares corresponding to real assets. During liquidation, the liquidatee is delegated virtual shares (which do not increase `_internalSupply`). If the liquidatee owes significant interest, `_accrueInterest` attempts to burn shares from the liquidatee. The `_burn` function decreases `_internalSupply`. If the interest owed (in shares) is greater than the current `_internalSupply` (which can happen if the pool has low liquidity/utilization but high accumulated debt, or simply if `interest > 10^6` virtual shares when empty), the subtraction underflows and reverts. This bricks the liquidation process, leaving the protocol stuck with the bad debt.

## Impact
Inability to liquidate insolvent accounts, leading to bad debt accumulation.

## Command to Run Test


## Proof of Concept
1. Pool has low `_internalSupply` (e.g. LPs withdrew).
2. Insolvent user has interest debt > `_internalSupply`.
3. Liquidator calls `dispatchFrom` (liquidate).
4. Calls `CollateralTracker.delegate` (balance increases, internalSupply same).
5. Calls `CollateralTracker.settleBurn` -> `_accrueInterest`.
6. `_accrueInterest` tries to burn `interest` shares.
7. `_burn` reverts due to `_internalSupply` underflow.

## Proof of Code
function testLiquidationDoS() public { 
 // Drain pool to low internalSupply 
 // Create insolvent user with high interest 
 // Attempt liquidation -> Revert 
}

## Suggested Mitigation
Modify `_burn` or `_accrueInterest` to handle virtual/delegated shares differently, ensuring they do not reduce `_internalSupply` below zero or track them separately.





Finding Status: LowSeverityDueToRareLikelihood + InvalidERC20EdgeCase
## [H-45]. Delegated Share Reentrancy Attack in Liquidation

## id: THMx6jL3XMJuMIetJvTZD

## Derived From Pattern/Invariant
ERC4626 withdrawer or redeemer

## Exploit Type
Reentrancy

## Location
CollateralTracker.settleLiquidation

## Finding Status: LowSeverityDueToRareLikelihood + InvalidERC20EdgeCase
### Finding Status Justification: In `CollateralTracker.settleLiquidation` (bonus < 0, V3 path), the contract performs an external `SafeTransferLib.safeTransferFrom` call before it removes the liquidatee’s delegated `type(uint248).max` balance. There is no reentrancy guard on ERC4626 entrypoints (`redeem/withdraw/mint/deposit`). If the underlying token supports reentrant callbacks (e.g., ERC777-style hooks) and the liquidatee is also the liquidator (self-liquidation via `dispatchFrom` with `account == msg.sender`), the liquidatee contract can reenter during `transferFrom` while still holding delegated shares and attempt to `redeem` up to `s_depositedAssets-1`. This depends on non-standard callback behavior; standard ERC20s do not reenter.
### Finding Complexity: 0
### PoC Test Status: ErrorRunningTests
## Minimim Privilege Required:Permissionless


## Description
During `settleLiquidation`, the `CollateralTracker` transfers tokens from the liquidator to cover protocol loss if the bonus is negative. This transfer uses `SafeTransferFrom`, which can trigger a reentrancy callback (e.g., ERC777 tokens or malicious contract). At this point in execution, the liquidatee's share balance has been artificially inflated via `delegate` (to ~uint248.max) to facilitate settlement, and their options have been closed (0 legs). An attacker can reenter `redeem` on the liquidatee's account, successfully burning the delegated phantom shares to drain the `CollateralTracker`'s assets before `revoke` is called.

## Impact
Complete drainage of `CollateralTracker` assets.

## Command to Run Test


## Proof of Concept
1. Attacker sets up a liquidatable position. 2. Attacker triggers liquidation (or waits for one) using a malicious token/hook. 3. `PanopticPool` calls `delegate`, inflating liquidatee balance. 4. `settleLiquidation` is called. 5. If `bonus < 0`, `safeTransferFrom` is called on Attacker. 6. Attacker reenters `CollateralTracker.redeem` for the liquidatee. 7. Since balance is high and legs are 0, `redeem` succeeds, sending all vault assets to attacker. 8. Recursion unwinds, `revoke` clears the phantom balance, but assets are gone.

## Proof of Code
function testReentrancyLiquidation() public { 
 // Setup liquidatable state 
 // Hook into transfer 
 // Call redeem in hook 
 // Assert assets stolen 
 }

## Suggested Mitigation
Add a `nonReentrant` modifier to `redeem`, `withdraw`, `mint`, `deposit`, and `settleLiquidation`.



