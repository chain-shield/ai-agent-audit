
## PROTOCOL OVERVIEW:

### GTE Protocol – High-performance Launchpad & Perps CLOB

GTE unifies token launches, AMM liquidity, and a central-limit order-book (CLOB) for spot + perpetuals on the MegaETH L2.

1. **Launchpad**
   • Deploys a new ERC20 *LaunchToken*, selling 80 % via a **SimpleBondingCurve**. Quote payments stay in Launchpad; bought tokens are locked as *bonding shares* until `unlock()`.  
   • When curve supply empties, Launchpad auto-creates a Uniswap-V2 pair through **GTELaunchpadV2PairFactory**, seeds it with the remaining 20 % supply, and locks LP tokens in **LaunchpadLPVault**.  
   • Swap fees (0.1 %) stream to **Distributor**, which also pays staking rewards tracked by *RewardsTracker*.

2. **Perpetual CLOB**
   • Users deposit USDC into the ERC-4626 **GTL** vault; shares represent pro-rata claim.  
   • **PerpManager** holds vault USDC as collateral, lets traders (or whitelisted operators) post margin, set leverage, and manage orders on-chain.  
   • Risk is controlled by **LiquidatorPanel** (forced closes) and **InsuranceFund**; admins can pause via **AdminPanel**.  
   • Withdrawals are request-queued and batch-executed by vault admins, preventing griefing while keeping the system solvent.

3. **Security & Trust**
   • Owner roles grant admin/operator rights; withdrawal admin can stall exits, and Launchpad owner can change curve/fees.  
   • No contract can arbitrary seize user balances; all funds move through explicit, role-checked paths.


## SUMMARY OF FILE: 2025-08-gte-perps/contracts/perps/GTL.sol
### GTL.sol (GTL)
Purpose: ERC-4626 USDC vault for GTE Perps. Users deposit USDC, receive GTL shares; PerpManager uses vault funds as collateral across sub-accounts. Withdrawals are queued by users and batch-processed by an Admin who burns shares and transfers USDC. Owner appoints Admins and authorises operators.
Trust model: Admin (role) controls withdrawal execution; PerpManager trades collateral; Owner controls Admins—users must trust both.

Storage
• usdc – asset token
• perpManager – PerpManager addr
• ADMIN_ROLE – role id
• _subaccounts – uint256 set of sub-accts
• _withdrawalQueue – queued ids
• _withdrawalCounter – incrementing id
• _queuedWithdrawal – id→Withdrawal
• _queuedShares – acc→shares

External/Public Functions
1. initialize(address _owner) external •nonpayable •initializer
   /// Set owner & infinite approve PerpManager
2. queueWithdrawal(uint256 shares) external returns(uint256) •nonpayable
   /// User requests withdrawal; returns id
3. cancelWithdrawal(uint256 id) external •nonpayable
   /// Caller cancels own queued withdrawal
4. processWithdrawals(uint256 num) external •nonpayable •onlyAdmin
   /// Admin executes first `num` queued withdrawals
5. grantAdminRole(address) external •nonpayable •onlyOwner
   /// Owner grants ADMIN_ROLE
6. revokeAdminRole(address) external •nonpayable •onlyOwner
   /// Owner revokes ADMIN_ROLE
7. approveOperator(address) external •nonpayable •onlyOwner
   /// Owner whitelists operator on PerpManager
8. disapproveOperator(address) external •nonpayable •onlyOwner
   /// Owner removes operator approval
9. addSubaccount(uint256) external •nonpayable •onlyPerpManager
   /// PerpManager registers sub-account
10. removeSubaccount(uint256) external •nonpayable •onlyPerpManager
    /// PerpManager deregisters sub-account
11. totalAssets() public view returns(uint256)
    /// Full vault + trading balances
12. totalAccountValue() public view returns(uint256)
    /// Sum positive PnL over sub-accounts
13. orderbookCollateral() public view returns(uint256)
    /// Collateral locked on orderbook
14. freeCollateralBalance() public view returns(uint256)
    /// Idle collateral at PerpManager
15. getSubaccounts() external view returns(uint256[])
    /// All linked sub-accounts
16. getQueuedWithdrawal(uint256) external view returns(Withdrawal)
    /// Details of id
17. getQueuedShares(address) external view returns(uint256)
    /// Shares queued by user
18. getWithdrawalQueue() external view returns(uint256[])
    /// Pending withdrawal ids
19. hasAdminRole(address) external view returns(bool)
    /// Check ADMIN_ROLE
20. previewWithdraw/maxWithdraw/maxRedeem pure overrides return 0 (disabled)


## SUMMARY OF FILE: 2025-08-gte-perps/contracts/perps/PerpManager.sol
### PerpManager (≤100 words)
Orchestrates collateral, margin, leverage and order-flow for GTE perpetual CLOB.  Users (or whitelisted operators) manage USDC collateral, adjust margin/leverage and place/amend/cancel spot, standard or backstop limit orders.  Funds live in CollateralManager; PerpManager cannot custody or withdraw except via user/role-gated calls.  AdminPanel can pause/param-tune; LiquidatorPanel handles liquidations; no admin access to user balances.

#### Storage
• accountManager – IAccountManager immutable – spot↔perp bridge

#### External/Public Functions
1. deposit(address,uint256) external onlySenderOrOperator nonpayable  — Deposit free collateral.
2. withdraw(address,uint256) external onlySenderOrOperator nonpayable — Withdraw free collateral.
3. depositTo(address,uint256) external nonpayable — Deposit collateral into another account.
4. depositFromSpot(address,uint256) external onlySenderOrOperator nonpayable — Move funds from Spot.
5. withdrawToSpot(address,uint256) external nonpayable — Called by AccountManager to send funds back.
6. addMargin(address,uint256,uint256) external onlySenderOrOperator nonpayable — Increase position margin.
7. removeMargin(address,uint256,uint256) external onlySenderOrOperator nonpayable — Decrease position margin.
8. setPositionLeverage(bytes32,address,uint256,uint256) external onlySenderOrOperator nonpayable — Change pos leverage.
9. placeOrder(address,PlaceOrderArgs) external onlySenderOrOperator nonpayable — Place std order.
10. postLimitOrderBackstop(address,PlaceOrderArgs) external onlySenderOrOperator nonpayable — Post backstop order.
11. amendLimitOrder(address,AmendLimitOrderArgs) external onlySenderOrOperator nonpayable — Modify std order.
12. cancelLimitOrders(bytes32,address,uint256,uint256[]) external onlySenderOrOperator nonpayable — Cancel std orders.
13. amendLimitOrderBackstop(address,AmendLimitOrderArgs) external onlySenderOrOperator nonpayable — Modify backstop order.
14. cancelLimitOrdersBackstop(bytes32,address,uint256,uint256[]) external onlySenderOrOperator nonpayable — Cancel backstop orders.
15. cancelConditionalOrders(address,uint256[]) external onlySenderOrOperator nonpayable — Void pending conditional orders.


## SUMMARY OF FILE: 2025-08-gte-perps/contracts/perps/modules/LiquidatorPanel.sol
### LiquidatorPanel (admin‐only liquidation module)

Purpose: Manages forced position closures to keep perp markets solvent. Holds no funds; operates on ClearingHouse storage. Only addresses with ADMIN, LIQUIDATOR or BACKSTOP_LIQUIDATOR roles may call actions.

Storage Vars: — (delegated to StorageLib)

External / Public Functions

1. `function liquidate(bytes32 asset, address acct, uint256 sub)` external onlyLiquidator onlyActiveProtocol
   ▸ Performs standard liquidation on "standard" book.

2. `function backstopLiquidate(bytes32 asset, address acct, uint256 sub)` external onlyBackstopLiquidator onlyActiveProtocol
   ▸ Executes backstop liquidation; shares fee with backstop liquidators.

3. `function deleverage(bytes32 asset, DeleveragePair[] pairs)` external onlyLiquidator onlyActiveProtocol
   ▸ Auto-deleverages underwater maker vs solvent taker at bankruptcy price.

4. `function delistClose(bytes32 asset, Account[] accts)` external onlyLiquidator onlyActiveProtocol
   ▸ Forces full close for accounts after market is delisted.



## SUMMARY OF FILE: 2025-08-gte-perps/contracts/launchpad/BondingCurves/SimpleBondingCurve.sol
### SimpleBondingCurve
Purpose: Implements a constant-product style bonding curve for tokens created via Launchpad. Curve shape set by immutable virtual reserves; Launchpad alone performs buys/sells, while Launchpad owner may (deprecated) overwrite reserves. Contract never receives tokens itself—funds remain in Launchpad, so trust rests on Launchpad owner.

Storage:
• VIRTUAL_BASE – virtual base reserve
• VIRTUAL_QUOTE – virtual quote reserve
• reserves – token→{quote,base} live reserves
• supply – token→{total,bonding} supplies
• launchpad – Launchpad contract address

External/Public Functions:
1. init(bytes data) external onlyLaunchpad – Set virtual reserves once.
2. initializeCurve(address token,uint256 total,uint256 bonding) external onlyLaunchpad – Register new token & reserves.
3. setReserves(address token,uint256 quote,uint256 base) external onlyLaunchpadOwner – Manually set live reserves (deprecated).
4. setVirtualReserves(uint256 base,uint256 quote) external onlyLaunchpadOwner – Reset virtual reserves (deprecated).
5. buy(address token,uint256 baseAmt) external onlyLaunchpad returns(uint256 quoteAmt) – Quote needed quote & update reserves.
6. sell(address token,uint256 baseAmt) external onlyLaunchpad returns(uint256 quoteAmt) – Return quote for base & update reserves.
7. bondingSupply(address) external view returns(uint256) – Remaining bondable supply.
8. totalSupply(address) external view returns(uint256) – Total minted supply record.
9. baseSoldFromCurve(address) external view returns(uint256) – Cumulative base sold.
10. quoteBoughtByCurve(address) external view returns(uint256) – Cumulative quote collected.
11. getReserves(address) external view returns(uint256 quote,uint256 base) – Current reserves.
12. quoteBaseForQuote(address,uint256 quote,bool isBuy) external view returns(uint256 base) – Price calc helper.
13. quoteQuoteForBase(address,uint256 base,bool isBuy) external view returns(uint256 quote) – Price calc helper.
14. supportsInterface(bytes4) external pure returns(bool) – ERC-165 support.


## SUMMARY OF FILE: 2025-08-gte-perps/contracts/launchpad/Distributor.sol
### Distributor (100 words)
Distributes bonding-curve/launchpad incentive rewards. Holds user/third-party reward tokens; only Launchpad moves stakes, while owner/ADMIN may skim accidental donations. Trust: users’ pending rewards custodied here; admin can withdraw only excess. Key entrypoints cover pool creation, reward funding, stake delta from Launchpad, and user claiming.

Storage
• ADMIN_ROLE – role id
• initialized (bool) – one-time init flag
• launchpad (address) – Launchpad contract
• totalPendingRewards mapping(asset=>uint) – undistributed user rewards

External/Public Functions
• initialize(address) public onlyOwner – set launchpad, lock.
  "Set launchpad once after deploy."
• skimExcessRewards(asset,uint) external onlyOwnerOrRoles – nonpayable
  "Withdraw tokens above pending totals."
• getRewardsPoolData(asset) external view – pure/view
  "Return pool metadata snapshot."
• getUserData(asset,acct) external view – view
  "Return user stake & rewards for pool."
• getUserDataForTokens(assets[],acct) external view
  "Batch version of getUserData."  
• getPendingRewards(asset,acct) external view
  "Compute claimable base & quote."
• endRewards(pair) external onlyLaunchpad – nonpayable
  "Stop reward accrual for pair."
• createRewardsPair(asset,quote) external onlyLaunchpad – nonpayable
  "Initialize new reward pool."
• addRewards(tok0,tok1,amt0,amt1) external nonpayable
  "Fund pool with incentive tokens."
• increaseStake(asset,acct,shares) external onlyLaunchpad – returns(uint,uint)
  "Update stake up; distribute owed."
• decreaseStake(asset,acct,shares) external onlyLaunchpad – returns(uint,uint)
  "Update stake down; distribute owed."
• claimRewards(asset) external returns(uint,uint)
  "User withdraws pending rewards."


## SUMMARY OF FILE: 2025-08-gte-perps/contracts/launchpad/LaunchToken.sol
### `LaunchToken` (≤100 words)
ERC20 minted/owned by Launchpad.  Transfers are disabled while bonding; only Launchpad may move tokens to buyers or GTE router.  Each buyer’s `bondingShare` tracks pre-bonding stake used for launchpad reward accounting.  `unlock()` lifts the transfer lock; rewards end automatically when all shares burned before unlock.  Launchpad can mint once; total supply capped at `uint96.max`.  Trust model: Launchpad is sole admin, controls minting and unlock; no other privileged roles.  No direct user funds held besides standard ERC20 balances.

Storage
• ABI_VERSION – abi version
• launchpad – launchpad addr
• gteRouter – router addr
• _name, _symbol – token metadata
• _mediaURI – image/metadata URI
• unlocked – transfers flag
• eventNonce – incremental id for events
• totalFeeShare – sum of bonding shares
• bondingShare mapping(addr⇒uint) – user stakes

Public / External Functions
1. `function name() public view returns (string)`  – ERC20 name.
2. `function symbol() public view returns (string)` – ERC20 symbol.
3. `function mediaURI() public view returns (string)` – Returns media URI.
4. `function unlock() external onlyLaunchpad` – Enable unrestricted transfers.
5. `function mint(uint256 amount) external onlyLaunchpad` – Mint tokens to launchpad; checks cap.


## SUMMARY OF FILE: 2025-08-gte-perps/contracts/launchpad/Launchpad.sol
### Launchpad.sol

Purpose & Trust (≤100w):
Permissionless token launch system: deploys new LaunchToken, sells 80% supply on bonding curve, auto-creates Uniswap-V2 pair & locks 20% LP in LaunchpadLPVault when curve supply exhausted. Holds user quote funds until graduation. Owner can update curve, quote token, fees. Users trust contract to hold quotes & execute swaps; admin only pulls fees & config.

Storage:
• gteRouter – router hub
• operator – OperatorPanel contract
• distributor – fee distributor
• uniV2Router – Uniswap v2 router
• uniV2Factory – factory
• launchpadLPVault – vault for locked LP
• currentQuoteAsset – ERC20 used as quote
• currentBondingCurve – active curve impl
• _launches – token→LaunchData
• launchFee – ETH fee to create token
• uniV2InitCodeHash – factory init hash

External/Public Functions:
1. launches(addr) public view – "Return LaunchData for token"
2. baseSoldFromCurve(addr) public view – "Total base sold so far"
3. quoteBoughtByCurve(addr) public view – "Quote accrued by curve"
4. quoteBaseForQuote(addr,uint,bool) public view – "Base ⇄ Quote quote"
5. quoteQuoteForBase(addr,uint,bool) public view – "Quote ⇄ Base quote"
6. eventNonce() external view – "Global seq id"
7. initialize(owner,quote,curve,vault,data) external – initializer – "Proxy init"
8. launch(name,symbol,uri) external payable nonReentrant – "Deploy & register new token"
9. buy(buyData) external nonReentrant onlyBondingActive onlySenderOrOperator – "Buy base from curve"
10. sell(acc,token,recp,amt,minOut) external nonReentrant onlyBondingActive onlySenderOrOperator – "Sell base to curve"
11. updateBondingCurve(addr) external onlyOwner – "Set new curve impl"
12. updateQuoteAsset(addr) external onlyOwner – "Set new quote ERC20"
13. updateInitCodeHash(bytes) external onlyOwner – "Set pair init hash"
14. pullFees() external onlyOwner – "Withdraw collected ETH fees"
15. updateLaunchFee(uint) external onlyOwner – "Set launch ETH fee"
16. updateLaunchpadLPVault(addr) external onlyOwner – "Set vault"
17. increaseStake(acc,shares) external onlyLaunchAsset – "Inc user stake for rewards"
18. decreaseStake(acc,shares) external onlyLaunchAsset – "Dec user stake"
19. endRewards() external onlyLaunchAsset – "Close rewards, push to distributor"


## SUMMARY OF FILE: 2025-08-gte-perps/contracts/launchpad/LaunchpadLPVault.sol
### LaunchpadLPVault
Purpose: Holds LP tokens generated by launchpad sales. Admin-custodied; owner (set at init) controls vault, launchpad addr stored for future use. No user interaction; strictly an asset container.

Storage
• launchpad – launchpad contract address
• ABI_VERSION – impl ABI version constant

External / Public Functions
1. `function initialize(address launchpad_, address initialOwner) external initializer nonpayable`
   – Set launchpad address and assign contract owner. (<=100 char natspec)
2. `fallback() external nonpayable`
   – Reverts; blocks unexpected calls. (<=100 char natspec)


## SUMMARY OF FILE: 2025-08-gte-perps/contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol
### GTELaunchpadV2Pair (<=100 words)
Uniswap-V2 LP token with extra logic to share 0.1 % swap fee (1/1000) with a Launchpad rewards pool. When the pool is linked to a Launchpad, a proportional share of each swap fee is accumulated and periodically forwarded to a Distributor. Admin (launchpadFeeDistributor) can permanently stop further fee accrual. Otherwise, behaviour mirrors classic UniswapV2Pair (mint, burn, swap, skim, sync).

Storage
• launchpadLp – lp holder for launchpad
• launchpadFeeDistributor – contract that receives fees
• factory – pair factory
• token0 / token1
• reserve0 / reserve1
• blockTimestampLast
• price0CumulativeLast / price1CumulativeLast
• kLast – last √k snapshot
• accruedLaunchpadFee0 / 1 – pending fees
• rewardsPoolActive – 1 if accruing
• unlocked – re-entrancy flag

External / Public functions
1. constructor() public – sets factory.  @dev Init factory address
2. initialize(address,address,address,address) external nonpayable – called by factory once.  @dev Set tokens & LP params
3. getReserves() public view – returns reserves & timestamp.  @dev Uniswap getter
4. getAccruedLaunchpadFees() public view – returns pending fees.  @dev Launchpad fee getter
5. endRewardsAccrual() external nonpayable – only distributor; stop & clear rewards.  @dev Halts fee accrual
6. mint(address to) external lock nonpayable – provide liquidity & mint LP.  @dev Mint LP tokens
7. burn(address to) external lock nonpayable – burn LP, redeem tokens.  @dev Remove liquidity
8. swap(uint amount0Out,uint amount1Out,address to,bytes data) external lock nonpayable – token swap.  @dev Swap with fee routing
9. skim(address to) external lock nonpayable – send excess tokens to addr.  @dev Force balances = reserves
10. sync() external lock nonpayable – update reserves from balances.  @dev Force reserve sync


## SUMMARY OF FILE: 2025-08-gte-perps/contracts/launchpad/uniswap/GTELaunchpadV2PairFactory.sol
### GTELaunchpadV2PairFactory
Permissionless Uniswap-V2 style pair factory; anyone can deploy a GTELaunchpadV2Pair. When the caller equals `launchpad`, the pair is initialized with special `launchpadLp` and `launchpadFeeDistributor` addresses, routing LP tokens & fees to the launchpad vault. Admin power is limited to `feeToSetter`, who can change `feeTo` and hand over the role.

Storage
* launchpad – launchpad contract
* launchpadLp – LP vault for launchpad
* launchpadFeeDistributor – fee recipient
* feeTo – address collecting pair fees
* feeToSetter – admin permitted to set `feeTo`
* getPair – token0⇒token1⇒pair mapping
* allPairs – list of created pairs

External / public functions
* `function allPairsLength() external view returns (uint256)`  
  /// returns current pairs count
* `function createPair(address tokenA,address tokenB) external returns (address)`  
  /// deploys new pair via CREATE2; reverts on duplicates
* `function setFeeTo(address _feeTo) external`  
  /// feeToSetter sets fee recipient
* `function setFeeToSetter(address _feeToSetter) external`  
  /// transfers feeToSetter role


## Main List of Files in Project

contracts/launchpad/BondingCurve.sol
contracts/launchpad/BondingCurves/IBondingCurveMinimal.sol
contracts/launchpad/BondingCurves/SimpleBondingCurve.sol
contracts/launchpad/Distributor.sol
contracts/launchpad/LaunchToken.sol
contracts/launchpad/Launchpad.sol
contracts/launchpad/LaunchpadLPVault.sol
contracts/launchpad/libraries/RewardsTracker.sol
contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol
contracts/launchpad/uniswap/GTELaunchpadV2PairFactory.sol
contracts/launchpad/uniswap/interfaces/IGTELaunchpadV2Pair.sol
contracts/launchpad/uniswap/interfaces/IUniswapV2Router01.sol
contracts/perps/GTL.sol
contracts/perps/PerpManager.sol
contracts/perps/modules/AdminPanel.sol
contracts/perps/modules/LiquidatorPanel.sol
contracts/perps/modules/ViewPort.sol
contracts/perps/types/BackstopLiquidatorDataLib.sol
contracts/perps/types/Book.sol
contracts/perps/types/CLOBLib.sol
contracts/perps/types/ClearingHouse.sol
contracts/perps/types/CollateralManager.sol
contracts/perps/types/Constants.sol
contracts/perps/types/Enums.sol
contracts/perps/types/FeeManager.sol
contracts/perps/types/FundingRateEngine.sol
contracts/perps/types/InsuranceFund.sol
contracts/perps/types/Market.sol
contracts/perps/types/Order.sol
contracts/perps/types/PackedFeeRatesLib.sol
contracts/perps/types/Position.sol
contracts/perps/types/PriceHistory.sol
contracts/perps/types/StorageLib.sol
contracts/perps/types/Structs.sol


 ## DOCUMENTATION: 

 ### gte-docs.md

# Overview

## Launchpad

Permissionless token launcher and project token launchpad.

### Permissionless Token Launcher

The GTE token launcher is a permissionless system that allows anyone to boostrap liquidity to launch a token on GTE. Launches are fair, meaning that no tokens will be available for purchase by the team beforehand. The flow of launching a new long-tail asset is as follows:

- 80% of the token supply will be traded on a bonding curve, and when a token hits the bonding price, a liquidity pool will automatically be deployed on the GTE AMM seeded with 20% of the supply reserved from the bonding curve.
- After a launched token bonds and gets its own liquidity pool, the token will be immediately tradeable in the DEX aggregator frontend.
- After a token reaches sufficient maturity and market depth, it will be automatically added to the GTE CLOB platform.

### Project Token Launchpad

The GTE Token Launchpad addresses the growing skepticism around CEX listings, which are often expensive and lack transparency in price discovery. Unlike CEXs, GTE partners with projects on MegaETH to launch tokens onchain through our token launchpad and across our trading venues.

The launchpad facilitates the creation of fully onchain token vaults, enabling token sales to the GTE community. Upon a sale, tokens are locked in a stake vault. Users who hold their staked tokens for longer periods receive more tokens at the time of vault unlock. Additionally, GTE receives a portion of the initial supply dedicated to the launchpad.

This process is conducted in a fully compliant manner, with partnerships in place to provide necessary KYC, ensuring protection for both GTE and its users.

## Perps CLOB

GTE onchain Central-Limit Order Book

### What is a CLOB?

The order book is an exchange design that resembles traditional finance. For any given asset pair, an order book maintains a bid and ask side – each one being a list of buy and sell orders, respectively. Each order is placed at a different price level, called a limit, and has an order size, which represents the amount of the trade asset that the order wants to buy or sell. Order books use an algorithmic matching engine to match up buy and sell orders, settling the funds of orders that fulfill each other. Most order books use “price-time priority” for their matching engines, meaning that the highest buy offers and lowest sell offers are settled first, followed by the chronological sequence of orders placed at that limit price.

### Perps
GTE leverages its high-performance infrastructure to offer Central Limit Order Books for both major market types, with perpetual futures being the focus of this contest:

- Perpetual Futures CLOB: For trading derivatives contracts that mimic spot prices without an expiry date, allowing for leverage and hedging strategies.


# GTE Protocol Overview (Security-Oriented Summary)

## Core Components

* **Token Launchpad & Launcher**
  Fully permissionless — enables fair liquidity bootstrapping via bonding curves, automatically spawning an AMM pool and enabling immediate trading.
* **Classic AMM**
  Facilitates price discovery for new and niche tokens.
* **Spot & Perpetual CLOB (on-chain order book)**
  Runs centralized exchange–style matching on-chain using a crankless design — meaning orders are matched seamlessly with high frequency.
* **Best-Price Aggregator**
  Routes trades across AMM, CLOB, and potentially other MegaETH venues to ensure optimal pricing.

All of this is brought together under one roof—launch, price discovery, live trading, perpetuals—in a streamlined, low-friction interface.

## MegaETH: The Foundation

* **EVM-compatible L2** with a real-time sequencer, enabling parallel execution, and integrated with EigenDA for robust data availability.
* Capable of **100,000 TPS** and **single-digit millisecond latency** — setting the stage for on-chain order books that match CEX speeds.
* **Low gas cost** facilitates frequent order cancellation and resubmission without penalty — ideal for market maker strategies and tight spread regimes.
* **Price-time priority matching** mirrors traditional trading fairness models, encouraging pro traders.
* Fully composable within the Ethereum DeFi landscape, thanks to EVM compatibility.

## **MAIN INVARIANTS**
## ************************************************************************
We define the PerpManager's USDC balance as:

$$
\sum^{total\_users}_{i=0}{user\_free\_collateral\_balance[i]} + \sum^{total\_users}_{i=0}{user\_margin\_balance[i]} + insurance\_fund\_balance
$$
## ************************************************************************

## **AREAS OF CONCERN ==> PAY EXTRA ATTENTION TO BELOW TO WHEN BUG HUNTING**

### Economical Vulnerabilities

Economical attacks (e.g. engaging large amounts of tokens to break the platform or profit from it) are considered a valid attack vector; we encourage Wardens to look out for ways to generate "bad debt", e.g. negative equity, in a way that would result in net-positive gains for the attacker, or make the platform illiquid. ADL (Auto-De-Leverage) abuses that result in monetary gains for the attacker as well as any attack that would result in loss of funds for other users, themselves or the Platform's funds, making it illiquid, are of particular interest to us. To note, *fund loss of oneself must result from an inadvertent action to be considered a valid vulnerability and must not arise from deliberate misuse of the platform*.

### Orderbook Denial-of-Service

The Orderbook should be able to process orders at all times; we invite the wardens to look for attacks that would result in a Denial Of Service (e.g. placing an order that cannot be cleared by the ClearingHouse), or that bypasses the limit of orders a user can place in one transaction.


This should always be equal to or less than the PerpManager's USDC Token balance.





 ## CONFIG FILES: 

 ### foundry.toml

[profile.default]
evm_version = 'cancun'
libs = ["lib"]
optimizer = true
optimizer-runs = 200
out = "out"
solc_version = '0.8.27'
src = "contracts"
test = 'test'
via_ir = false


[rpc_endpoints]
testnet = "${RPC_TESTNET}"


[fmt]
single_line_statement_blocks="single"
multiline_func_header="attributes_first"
contract_new_lines=false
sort_imports=false
override_spacing=true
line_length=120
tab_width=4
int_types="long"
quote_style="double"
number_underscore="thousands"
hex_underscore="remove"
wrap_comments=false
ignore=["script/", "test/"]


### remappings.txt

@openzeppelin=lib/openzeppelin-contracts/contracts
@openzeppelin-contracts-upgradeable=lib/openzeppelin-contracts-upgradeable/contracts
@solady=lib/solady/src/
@permit2=lib/permit2/src/
forge-std=lib/forge-std/src/
@gte-univ2-core=lib/gte-univ2/src/core/
@gte-univ2-periphery=lib/gte-univ2/src/periphery/

