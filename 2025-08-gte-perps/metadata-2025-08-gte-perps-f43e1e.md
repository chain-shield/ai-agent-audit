
## PROTOCOL OVERVIEW:

# GTE Protocol – High-Speed On-Chain Exchange Suite

GTE is a family of Solidity contracts that together deliver a **full-stack, CEX-style trading venue on Ethereum-compatible chains**.  The system is split into three logical pillars—Launchpad, Spot Exchange and Perpetual Futures—tied together by shared utilities (router, operator hub, vaults).

---

## 1. Launchpad & Token Bootstrap

### Goal
Enable anyone to create a new ERC-20 and *fair-launch* it without trusted seed rounds.

### Flow
1. **Launch** – `Launchpad.launch()` deploys a `LaunchToken` (1 B supply).
2. **Bonding Curve Sale** – `SimpleBondingCurve` sells 80 % of supply against an existing quote asset; pricing is automated via `x*y=k` math + configurable virtual reserves.
3. **Auto-LP Mint** – Once bonding supply sells out, the contract:
   * Creates a Uniswap-V2 pair through `GTELaunchpadV2PairFactory`.
   * Seeds it with the remaining 20 % of tokens plus the collected quote asset.
   * Sends resulting LP tokens to `LaunchpadLPVault` for custody.
4. **Rewards** – `Distributor` streams incentive tokens to stakers during the bonding phase; share accounting happens inside `LaunchToken`.
5. **User Interactions** – All buy/sell/claim operations can go through `GTERouter` for UX simplicity;
   Launchpad never holds user funds longer than the current tx.

**Security/Trust** – No upgrade paths inside core launch contracts; owner can only tweak fees & config, never seize tokens.  LP custody is immutably limited to the Launchpad/Vault pair.

---

## 2. Spot Exchange (CLOB)

### Components
* `CLOBManager` – deploys & admin-configures individual order books (tick size, lot, etc.).
* `CLOB` – price-time-priority limit-order book; matches orders but never escrows assets.
* `AccountManager` – central ERC-20 ledger that actually holds user balances, settles fills, and manages fee tiers.

### Order Lifecycle
1. Trader deposits collateral via `AccountManager.deposit()` (or router helper).
2. Trader (or an approved operator) calls `CLOB.placeOrder()` to create bid/ask.
3. Matching logic inside the book sends settlement hooks to `AccountManager.settleIncomingOrder()`, which moves balances and invoices maker/taker fees.
4. Withdrawals are initiated via `AccountManager.withdraw()`.

**Admin Scope** – Managers can tune market parameters and sweep protocol fees, but cannot move user balances directly.

---

## 3. Perpetual Futures

### Architecture
* **Collateral** – Same USDC sits inside `CollateralManager` (part of a shared storage library).
* **PerpManager** – user-facing façade for collateral moves, margin, leverage updates and order placement.  Relies on external matching engine modules.
* **LiquidatorPanel** – executes forced closures when accounts breach maintenance margin.
* **AdminPanel / ViewPort et al.** – pause protocol, tweak risk params, expose read-only data.
* **GTL Vault** – ERC-4626 vault pooling USDC so liquidity providers can earn funding fees. Withdrawals are queued and later processed by Admins.

### Interaction Example
1. User deposits USDC into `PerpManager.deposit()` (or bridges from spot with `depositFromSpot`).
2. Opens a position by posting orders; `PerpManager.placeOrder()` validates role and forwards to matching engine.
3. Funding payments, PnL and liquidation flows interact exclusively with the shared storage library—PerpManager itself stays stateless apart from immutable pointers.

**Risk Controls** – Liquidations, ADL and delist actions are gated behind dedicated roles (`LIQUIDATOR`, `BACKSTOP_LIQUIDATOR`).  Insurance fund accounting is handled in libraries, not the panels.

---

## 4. Shared Utilities

### OperatorHub
Aggregates role-granting for both spot (`AccountManager`) and perps (`PerpManager`).  Users call `approveOperator…()` to delegate fine-grained bit-mapped roles; no owner functions exist.

### GTERouter
UX front end that chains deposits, CLOB actions, bonding-curve swaps and AMM trades in a single tx.  Holds no persistent state; core addresses are immutable constructor params.

### Uniswap-V2 Fork
`GTELaunchpadV2Pair` adds a **0.1 % launchpad fee siphon** that streams to `launchpadLp` until `launchpadFeeDistributor` turns it off.

---

## 5. Contract Relationships (Bird’s-Eye)
```
User ─┐
      │  (1) Router / direct → AccountManager ↔ Spot CLOBs
      │
      │  (2) Router / direct → Launchpad → BondingCurve → Uniswap Pair
      │                                         │
      │                                         └→ LaunchpadLPVault (LP custody)
      │
      │  (3) Router / direct → PerpManager ↔ LiquidatorPanel, etc.
      │                               │
      │                               └→ GTL (liquidity pool vault)
      │
OperatorHub  ⟷  AccountManager & PerpManager (role delegation)
```

---

## 6. Security & Trust Assumptions
1. **No implicit custodianship** – All user funds reside in three vaults only: `AccountManager`, `CollateralManager` and `GTL`.  Other contracts merely forward calls.
2. **Role-Based Access** – Every state change checks `msg.sender` against bit-mapped roles.  `OperatorHub` lets users self-manage delegation.
3. **Immutable Critical Pointers** – Factories, routers and managers store module addresses as `immutable` to prevent rug pulls.
4. **Upgradeable Market Implementations** – Individual CLOB markets live behind a Beacon; the beacon itself is owned by protocol governance, not the manager contract.
5. **Withdrawal Safety** – GTL withdrawals are two-step (queue + admin process) to prevent bank-run style drains during volatile periods.

---

## 7. Key Invariants for Auditors
* `AccountManager` token balances + protocol fees == actual ERC-20 balance of the contract.
* Perp system invariant (see docs): sum(free + margin + insurance) ≤ USDC held by PerpManager collateral vault.
* `totalPendingRewards` in `Distributor` ≤ contract token balance.
* `GTL.totalAssets()` must include on-chain USDC + collateral posted to sub-accounts.

---

## 8. Upgrade & Admin Summary
* **Launchpad** – Owner can adjust launch fee, bonding curve address, LP vault, Factory `initCodeHash`, but never seize user tokens.
* **Spot & Perps** – Owners can set fee tiers, risk params, pause, liquidate, or collect protocol fees; they cannot withdraw user balances.
* **GTL** – Owner manages Admins and operator approvals; Admins execute queued withdrawals.

---

## 9. Why It Matters
GTE combines CEX-grade latency (via MegaETH L2) with DeFi composability.  By integrating permissionless token launches, spot trading and perpetual futures into one contract suite—with all balances custodial on-chain—it aims to deliver a *trust-minimized, high-performance alternative to centralized exchanges* while still enabling traditional market-maker workflows like rapid order cancel/replace.



## SUMMARY OF FILE: 2025-08-gte-perps/contracts/utils/OperatorHub.sol
### OperatorHub.sol — 1 contract (≈120 LOC)
Permissionless utility contract that **aggregates operator-approval calls** for both Spot (AccountManager) and Perps (PerpManager) subsystems.  It never touches user funds; it simply forwards role-management requests to the two underlying modules, acting as a single UX entry-point.  No owner/admin functions exist—every state-changing call is authenticated by `msg.sender`, so users retain full control over who can act on their behalf.

#### Storage Variables
| Name | Type | Description (≤50 chars) |
| ---- | ---- | ------------------------- |
| perpManager | IOperatorPanel | Perps operator/role registry |
| accountManager | IOperatorPanel | Spot operator/role registry |

#### Public/External Functions
1. `constructor(IViewPort perpMgr, IAccountManager acctMgr)` – external, non-payable.  Caches immutable addresses of Perps & Spot operator panels. *(Sets dependencies at deployment; no special privileges.)*

2. `getRoleApprovalsSpot(address account, address operator) view returns (uint256)` – external, view, pure-forward. *Fetch current role bitmap an operator has on a spot account.*

3. `getRoleApprovalsPerps(address account, address operator) view returns (uint256)` – external, view. *Same as above for perps.*

4. `approveOperatorSpot(address operator, uint256 roles)` – external, non-payable. *Caller whitelists `operator` for given role bits on Spot panel.*

5. `approveOperatorPerps(address operator, uint256 roles)` – external, non-payable. *Caller whitelists `operator` for Perps panel.*

6. `disapproveOperatorSpot(address operator, uint256 roles)` – external, non-payable. *Caller removes specific role bits from operator on Spot panel.*

7. `disapproveOperatorPerps(address operator, uint256 roles)` – external, non-payable. *Caller removes role bits on Perps panel.*



## SUMMARY OF FILE: 2025-08-gte-perps/contracts/account-manager/AccountManager.sol
### AccountManager (contracts/.../AccountManager.sol)

**Purpose & Trust Model (≈205 words)**  
On-chain balance ledger for GTE spot exchange. Holds user ERC-20 collateral, routes transfers between Spot, Router and Perps, and performs post-trade settlement for CLOB markets with automatic maker/taker fee accounting. 80-character security: funds are escrowed in the contract; only:  
• Users / approved operators can deposit & withdraw.  
• `CLOBManager` may register markets, set fee tiers and touch balances for order amendments.  
• Registered markets call `settleIncomingOrder`/credit/debit to move balances atomically after each fill.  
• Owner or `FEE_COLLECTOR` role can sweep accrued protocol fees.  
All state changing calls guarded by role modifiers so no single privileged actor can steal user funds, yet admin can upgrade fee tiers & collect fees.  

---
#### Storage
| Name | Type | Description (≤50 chars) |
| ---- | ---- | ----------------------- |
| `isMarket` | mapping(address⇒bool) | whitelist of spot markets |
| `accountTokenBalances` | mapping(addr⇒mapping(addr⇒uint)) | user ⇄ token balances |
| `FEE_COLLECTOR` | uint256 constant | role id for fee sweeper |
| `gteRouter` | address immutable | global router bypass |
| `clobManager` | address immutable | settlement authority |
| `spotMakerFeeRates` | PackedFeeRates immutable | tiered maker bps |
| `spotTakerFeeRates` | PackedFeeRates immutable | tiered taker bps |
| `perpManager` | IPerpManager immutable | perps collateral mgr |

---
#### External & Public Functions
`constructor(address _router,address _clob,address _hub,uint16[] maker,uint16[] taker,address _perp)` (public) – sets immutables & disables init.  
`initialize(address _owner)` external initializer – one-time owner setup.  
`getAccountBalance(address, address)` external view – return user token amount.  
`getEventNonce()` external view – monotonic event id.  
`getTotalFees(address)` external view – total collected fees.  
`getUnclaimedFees(address)` external view – unclaimed fees.  
`getFeeTier(address)` external view – user’s fee tier enum.  
`getSpotTakerFeeRateForTier(FeeTiers)` external view – taker bps.  
`getSpotMakerFeeRateForTier(FeeTiers)` external view – maker bps.  
`deposit(address acc,address tok,uint)` external nonpayable onlySenderOrOperator(SPOT_DEPOSIT) – pull from caller, credit acc.  
`depositTo(address acc,address tok,uint)` external nonpayable – pull from msg.sender, credit acc.  
`depositFromPerps(address,uint)` external onlySenderOrOperator(PERP_TO_SPOT_DEPOSIT) – withdraw from Perps then credit.  
`depositFromRouter(address,address,uint)` external onlyGTERouter – router funded deposit.  
`withdraw(address acc,address tok,uint)` external onlySenderOrOperator(SPOT_WITHDRAW) – debit & send to acc.  
`withdrawToPerps(address,uint)` external – PerpManager only; moves funds to perps.  
`withdrawToRouter(address,address,uint)` external onlyGTERouter – debit & send to router.  
`registerMarket(address)` external onlyCLOBManager – whitelist market.  
`collectFees(address tok,address dst)` external onlyOwnerOrRoles(FEE_COLLECTOR) – sweep accrued fees.  
`setSpotAccountFeeTier(address,FeeTiers)` external onlyCLOBManager – set one tier.  
`setSpotAccountFeeTiers(address[],FeeTiers[])` external onlyCLOBManager – batch tier set.  
`settleIncomingOrder(SettleParams)` external onlyMarket – core settlement, fee accrual.  
`creditAccount(address,address,uint)` external onlyMarket – credit with event.  
`creditAccountNoEvent(address,address,uint)` external onlyMarket – silent credit.  
`debitAccount(address,address,uint)` external onlyMarket – debit with event.

Each function emits events or reverts on insufficient balance, ensuring accounting integrity.



## SUMMARY OF FILE: 2025-08-gte-perps/contracts/launchpad/uniswap/GTELaunchpadV2PairFactory.sol
### `GTELaunchpadV2PairFactory`
Permissionless factory that mints Uniswap-V2-style pairs, with special logic to embed Launchpad-specific LP & fee-distributor addresses when the call originates from the Launchpad contract. Funds are non-custodial; the factory only deploys pair contracts. Admin ( `feeToSetter`) can redirect protocol fees but holds no user funds.

Purpose / Trust Model
* Deploys GTELaunchpadV2Pair instances via CREATE2.
* If `msg.sender == launchpad`, the created pair records `launchpadLp` & `launchpadFeeDistributor`; otherwise those fields are zero.
* Only `feeToSetter` can change `feeTo` or transfer `feeToSetter` role.

Storage Variables
* `launchpad` (immutable) – Launchpad contract address.
* `launchpadLp` (immutable) – Launchpad LP vault address.
* `launchpadFeeDistributor` (immutable) – Launchpad fee distro addr.
* `feeTo` – address collecting LP protocol fees.
* `feeToSetter` – admin able to set `feeTo` and role.
* `getPair` – mapping[token0][token1] → pair addr.
* `allPairs` – dynamic array of pair addresses.

Key Functions
1. `function allPairsLength() external view returns(uint256);`
   • @notice Returns total pairs deployed.
2. `function createPair(address tokenA,address tokenB) external returns(address pair);`
   • @notice Deploys a new pair via CREATE2; reverts on duplicates or invalid args. Embeds launchpad LP params when called by Launchpad.
3. `function setFeeTo(address _feeTo) external;`  (only feeToSetter)
   • @notice Updates the protocol-fee recipient.
4. `function setFeeToSetter(address _feeToSetter) external;` (only feeToSetter)
   • @notice Transfers admin rights to new address.

Each state-changing function is `nonpayable`; `allPairsLength` is `view`. Reverts mirror standard UniswapV2Factory errors.


## SUMMARY OF FILE: 2025-08-gte-perps/contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol
## GTELaunchpadV2Pair (`contracts/launchpad/uniswap/GTELaunchpadV2Pair.sol`)

Permission-less AMM pair used by GTE Launchpad. 100 % Uniswap-V2 compatible but adds launchpad-specific fee harvesting so that the launchpad’s LP wallet earns 0.1 % of each swap until the rewards pool is manually shut off by `launchpadFeeDistributor`. Users deposit both tokens, receive LP shares, and can trade exactly as on Uni-V2. Trust model: users’ funds are non-custodial; factory owner can only receive standard UNI fees. `launchpadFeeDistributor` can halt & claim outstanding launchpad fees.

### Major External Entrypoints
• `initialize(address t0,address t1,address lp,address dist)` – one-time factory call.
• `mint(address to)` – add liquidity.
• `burn(address to)` – remove liquidity.
• `swap(uint256 out0,uint256 out1,address to,bytes data)` – core swap.
• `endRewardsAccrual()` – distributor stops fee accrual, claims leftovers.
• `skim(address to)` – remove accidental tokens.
• `sync()` – force reserves update.

### Storage Variables
- launchpadLp – LP wallet holding rewards
- launchpadFeeDistributor – contract that pulls fees
- factory – deploying factory
- token0 / token1 – pair assets
- reserve0 / reserve1 – last stored reserves
- blockTimestampLast – last update block time
- price0CumulativeLast / price1CumulativeLast – TWAP sums
- kLast – last k for fee-on logic
- accruedLaunchpadFee0/1 – unclaimed launchpad fees
- rewardsPoolActive – 1 if fee accrual active
- unlocked – re-entrancy flag

### Function Reference (≤50 word NatSpec each)
`getReserves() public view` – Return stored reserves & timestamp.
`getAccruedLaunchpadFees() public view` – Return unpaid launchpad fees.
`initialize(...) external` – Factory sets tokens and fee roles.
`endRewardsAccrual() external` – Distributor ends & zeroes fee accrual.
`mint(address) external lock` – Deposit both tokens, mint LP shares.
`burn(address) external lock` – Burn LP shares, withdraw tokens.
`swap(uint256,uint256,address,bytes) external lock` – Perform constant-product swap; accrues fees.
`skim(address) external lock` – Transfer excess tokens to `to`.
`sync() external lock` – Update reserves to actual balances.



## SUMMARY OF FILE: 2025-08-gte-perps/contracts/launchpad/LaunchToken.sol
### Contract: LaunchToken (ERC20 extension)

Purpose & Trust Model (≤250 words)
LaunchToken represents the temporary, bonding-phase token minted by `Launchpad` when a project is onboarded.  It is **custodied by the Launchpad contract** until transfers are unlocked.  While `unlocked` is `false`, trading is heavily restricted: only the Launchpad can send or receive tokens (router may receive) and every outgoing transfer adjusts the sender’s “bonding share”.  These shares drive reward distribution in the Launchpad vault.  After the Launchpad calls `unlock()`, tokens become fully transferrable and the bonding-share tracking logic is disabled.  No user funds are held directly; the only privileged address is `launchpad` (immutable) which can mint once, unlock transfers, and be notified of staking changes.

Storage Variables (name →  <50 char description)
* ABI_VERSION – impl version for indexers
* launchpad – immutable authority
* gteRouter – AMM router allowed pre-unlock
* _name – ERC20 name string
* _symbol – ERC20 symbol string
* _mediaURI – off-chain asset uri
* unlocked – global transfer flag
* eventNonce – ever-incrementing event counter
* totalFeeShare – total pre-bonding stake
* bondingShare – user ⇒ stake amount

Major Entrypoints
1. **unlock() external onlyLaunchpad** – enables free transfers. Emits `TransfersUnlocked`.
2. **mint(uint256) external onlyLaunchpad** – one-time mint to Launchpad. Checks `totalSupply ≤ 2^96-1`.
3. View helpers: `name()`, `symbol()`, `mediaURI()`.

Key Internal Hooks
* **_beforeTokenTransfer** – enforces bonding rules and updates shares.
* **_increaseFeeShares / _decreaseFeeShares** – sync staking with `ILaunchpad`.

Function Interface Summary (≤50 words each)
• unlock() external onlyLaunchpad nonpayable – Flip `unlocked` true; emit event.
• mint(uint256 amt) external onlyLaunchpad nonpayable – Mint `amt` to Launchpad; revert if supply > 2^96-1.
• name() public view returns(string) – ERC20 override.
• symbol() public view returns(string) – ERC20 override.
• mediaURI() public view returns(string) – Returns token artwork URI.


## SUMMARY OF FILE: 2025-08-gte-perps/contracts/launchpad/BondingCurves/SimpleBondingCurve.sol
### SimpleBondingCurve
Permission-less pricing module backing GTE Launchpad token sales.  It exposes a constant-product style bonding curve, parameterised by configurable virtual reserves that shape the initial price slope.  The contract is stateless with respect to real tokens— it only stores numerical reserves; all ERC-20 transfers are executed by the Launchpad, which is the sole caller of trade and setup methods.  Users therefore trust Launchpad; this contract merely produces deterministic quotes.  Launchpad owner can set virtual reserves once at deployment; legacy setters remain but are marked *deprecated*.  No other admin keys exist.

Key flow
1. `init()` – Launchpad passes `(virtualBase, virtualQuote)` once.
2. `initializeCurve()` – executed per token to seed on-chain supply & virtual reserves.
3. `buy()` / `sell()` – Launchpad converts between quote & base amounts, updating per-token reserves.

### Storage Variables
- `uint256 VIRTUAL_BASE` – global virtual base reserve
- `uint256 VIRTUAL_QUOTE` – global virtual quote reserve
- `mapping(address=>Reserves) reserves` – per-token live reserves
- `mapping(address=>Supply) supply` – per-token total & bonding supply
- `address launchpad` – immutable Launchpad contract

### Public / External Functions
| Function | Visibility / Mutability / Modifiers | ≤50-word NatSpec |
|---|---|---|
| `init(bytes data)` | external, nonpayable, onlyLaunchpad | One-time global init; decodes and stores virtual reserves controlling the curve’s initial price. |
| `initializeCurve(address token, uint256 total, uint256 bonding)` | external, nonpayable, onlyLaunchpad | Per token setup; seeds reserve values and supply tracking, emits NewTokenLaunched. |
| `buy(address token, uint256 base)` | external, nonpayable, onlyLaunchpad | Consumes base tokens from curve, returns required quote amount; updates reserves via x*y=k math. |
| `sell(address token, uint256 base)` | external, nonpayable, onlyLaunchpad | Mints base back to curve, returns quote to seller; mirror of `buy`. |
| `setReserves(address, uint256, uint256)` | external, nonpayable, onlyLaunchpadOwner | Legacy manual override for reserves; discouraged in production. |
| `setVirtualReserves(uint256, uint256)` | external, nonpayable, onlyLaunchpadOwner | Legacy ability to change global virtual reserves; reverts on zero values. |
| View helpers (`bondingSupply`, `totalSupply`, `baseSoldFromCurve`, `quoteBoughtByCurve`, `getReserves`, `quoteBaseForQuote`, `quoteQuoteForBase`, `supportsInterface`) | external/view | Read-only utilities for Launchpad/frontend to inspect curve state or price quotes. |


## SUMMARY OF FILE: 2025-08-gte-perps/contracts/launchpad/LaunchpadLPVault.sol
### contracts/launchpad/LaunchpadLPVault.sol  
Permission-controlled vault meant to custody the liquidity-pool (LP) tokens that are minted at the end of a Launchpad sale.  The Launchpad contract (stored in `launchpad`) will later be able to pull the LP tokens and seed secondary-market liquidity.  No user funds are deposited directly; only the Launchpad and the owner (via OZ Ownable2Step) have control.  Trust model: users rely on the Launchpad logic and on the owner’s upgrade/admin rights—this vault itself is intentionally minimal and non-upgradable (impl is upgradeable but all external calls are disabled).

Major entry-points
1. `initialize(address launchpad_, address initialOwner) external initializer` — one-time setup setting the Launchpad that may interact with the vault and assigning ownership.  
2. Fallback — reverts on **all** other calls, guaranteeing that no ERC20/ETH can be mistakenly sent or drained through arbitrary function calls.

Storage variables  
- `address launchpad` – Launchpad contract allowed to manage vault.  
- `uint256 constant ABI_VERSION` – impl ABI version for indexers.

Function list
```
function initialize(address launchpad_, address initialOwner)
    external initializer
    onlyInitializing
    nonpayable
```
NatSpec (≤50w): *Initialises vault with Launchpad address and transfers owner rights to `initialOwner`. Must be called once immediately after deployment.*

```
fallback() external nonpayable
```
NatSpec: *Catch-all to reject any unknown function or ETH transfer; always reverts with `FallbackRevert()`.*

Because every other op is blocked, assets can only be moved via privileged Launchpad logic (not shown).


## SUMMARY OF FILE: 2025-08-gte-perps/contracts/launchpad/Distributor.sol
### Distributor

**Summary (≤250 words)**  
`Distributor` escrows and streams incentive rewards to stakers of a Launchpad bonding pair.  A once-set `launchpad` contract is the sole caller that mutates staking state, while token holders and admins can inject or skim reward tokens.  The contract tracks per-asset `totalPendingRewards` so that accidental transfers can be skimmed without touching users’ unclaimed amounts.  Owner/`ADMIN_ROLE` may withdraw excess tokens; the Launchpad may create pools, update stakes, or end reward accrual.  All accounting is delegated to `RewardsTracker` libs, keeping this contract light and focused on custody and transfer of funds.  Users interact trustlessly via `claimRewards`, receiving their share of base/quote incentives.

**Storage**
| Name | Type | Description |
| --- | --- | --- |
| ADMIN_ROLE | uint256 | Constant bitmask for admin role |
| initialized | bool | One-time init flag |
| launchpad | address | Authorized Launchpad caller |
| totalPendingRewards | mapping(address⇒uint256) | Unclaimed rewards per asset |

**Public / External Functions**
- `constructor()` public – sets initial owner.  
- `initialize(address _launchpad)` external onlyOwner – one-time Launchpad setter.  
- `skimExcessRewards(address asset,uint256 amt)` external onlyOwnerOrRoles – withdraw donations above pending tally.  
- `getRewardsPoolData(address)` external view – return full pool snapshot.  
- `getUserData(address asset,address user)` external view – user’s reward meta.  
- `getUserDataForTokens(address[] assets,address user)` external view – batched variant.  
- `getPendingRewards(address asset,address user)` external view – pending base & quote.  
- `endRewards(IGTELaunchpadV2Pair)` external onlyLaunchpad – stop accrual on pair.  
- `createRewardsPair(address launch,address quote)` external onlyLaunchpad – deploy new pool.  
- `addRewards(address t0,address t1,uint128 a0,uint128 a1)` external – inject incentives, auto-detect orientation.  
- `increaseStake(address launch,address acct,uint96 shares)` external onlyLaunchpad – bump stake & auto-payout accrued.  
- `decreaseStake(address launch,address acct,uint96 shares)` external onlyLaunchpad – reduce stake & payout.  
- `claimRewards(address launch)` external – claim owed rewards.

Each mutating function enforces safety checks (pool existence, share >0, overflow, etc.) and calls internal `_increase/ _decreaseTotalPending` to maintain invariant that contract balance ≥ pending rewards.  All token transfers use `SafeTransferLib`.



## SUMMARY OF FILE: 2025-08-gte-perps/contracts/launchpad/Launchpad.sol
### Launchpad.sol  
Permissionless factory that lets any user mint a new ERC-20 (LaunchToken) whose initial distribution & price discovery happens on a bonding-curve. 80 % of supply is sold by the curve; once exhausted the contract auto-creates a UniV2 pair, seeds it with the remaining 20 % + accumulated quote, and hands the LP over to LaunchpadLPVault. Contract never escrows user funds except during a swap; owner can only tweak config & withdraw launch fees.  

Major entrypoints  
• initialize(owner, quote, curve, lpVault, curveInit) ‑ external ‑ initializer  
• launch(name,symbol,mediaURI) ‑ external payable nonReentrant → token  
• buy(BuyData) ‑ external nonReentrant onlyBondingActive onlySenderOrOperator → (base,quote)  
• sell(account,token,recipient,amountInBase,minQuote) ‑ external nonReentrant onlyBondingActive onlySenderOrOperator → (base,quote)  
• increaseStake(account,shares) / decreaseStake… / endRewards() ‑ external onlyLaunchAsset  
• updateBondingCurve(addr) / updateQuoteAsset(addr) / updateInitCodeHash(bytes) / updateLaunchFee(uint) / updateLaunchpadLPVault(addr) ‑ external onlyOwner  
• pullFees() ‑ external onlyOwner  
View helpers: launches, baseSoldFromCurve, quoteBoughtByCurve, quoteBaseForQuote, quoteQuoteForBase, eventNonce.  

Storage variables  
ABI_VERSION – uint256 – impl version  
TOTAL_SUPPLY – uint256 – 1B tokens  
BONDING_SUPPLY – uint256 – 800M curve  
gteRouter – address – main router  
operator – IOperatorPanel – operator ACL  
distributor – IDistributor – fee splitter  
uniV2Router – IUniswapV2RouterMinimal  
uniV2Factory – IUniswapV2FactoryMinimal  
launchpadLPVault – LaunchpadLPVault – holds initial LP  
currentQuoteAsset – LaunchToken – quote currency  
currentBondingCurve – IBondingCurveMinimal  
_launches – mapping(address⇒LaunchData) – per token info  
launchFee – uint256 – ETH fee to create token  
uniV2InitCodeHash – bytes – factory salt  

Each function emits granular events (LaunchpadDeployed, TokenLaunched, Swap, BondingLocked…) enabling off-chain indexing and upgrade-safe tracking.


## SUMMARY OF FILE: 2025-08-gte-perps/contracts/perps/GTL.sol
### GTL (GTE Liquidity Pool)

**Purpose & Trust Model (≤250 words)**
GTL is an ERC-4626–compatible vault that pools USDC to provide collateral liquidity for GTE Perpetuals.  Users deposit USDC and receive GTL shares.  Withdrawals are *queued* rather than instant: users request a withdrawal (`queueWithdrawal`) and an Admin later batches and fulfils them (`processWithdrawals`).  The vault’s assets are not just its on-chain USDC balance; it also counts collateral posted to the on-chain order-book (`orderbookCollateral`), free collateral sitting inside the PerpManager, and positive account value across all linked Perp sub-accounts.  Only the designated `perpManager` can add/remove trading sub-accounts.  An Owner can grant/revoke `ADMIN_ROLE`; Admins may process withdrawals but cannot sweep funds.  Funds are therefore held in a semi-custodial model: users must trust Admins to process their queued withdrawals fairly and the PerpManager to preserve collateral balances.

Major entry points: deposits/withdraws via inherited ERC4626, `queueWithdrawal`, `cancelWithdrawal`, `processWithdrawals`, role-management, PerpManager hooks and various getters.

---
### Storage Variables
- `uint256 public constant ABI_VERSION` – implementation ABI id
- `address public immutable usdc` – underlying asset token
- `address public immutable perpManager` – PerpManager contract
- `uint256 public constant ADMIN_ROLE` – bitmask for admin
- `EnumerableSetLib.Uint256Set _subaccounts` – active perp sub-accounts
- `uint256[] _withdrawalQueue` – pending withdrawal ids
- `uint256 _withdrawalCounter` – running id index
- `mapping(uint256⇒Withdrawal) _queuedWithdrawal` – id ⇒ request data
- `mapping(address⇒uint256) _queuedShares` – shares awaiting withdrawal

---
### External / Public Functions
```
constructor(address usdc_, address perpMgr_)               
initialize(address owner_) external                        
name() public view override                                
symbol() public view override                              
asset() public view override                               
queueWithdrawal(uint256 shares) external returns (uint256) 
cancelWithdrawal(uint256 id) external                      
processWithdrawals(uint256 num) external onlyAdmin         
grantAdminRole(address acct) external onlyOwner            
revokeAdminRole(address acct) external onlyOwner           
approveOperator(address op) external onlyOwner             
disapproveOperator(address op) external onlyOwner          
addSubaccount(uint256 sub) external onlyPerpManager        
removeSubaccount(uint256 sub) external onlyPerpManager     
totalAssets() public view override returns (uint256)       
totalAccountValue() public view returns (uint256)          
orderbookCollateral() public view returns (uint256)        
freeCollateralBalance() public view returns (uint256)      
getSubaccounts() external view returns (uint256[])         
getQueuedWithdrawal(uint256 id) external view              
getQueuedShares(address acct) external view                
getWithdrawalQueue() external view returns (uint256[])     
hasAdminRole(address acct) external view returns (bool)    
previewWithdraw(uint256) public pure override              
maxWithdraw(address) public pure override                  
maxRedeem(address) public pure override                    
```

---
### Function NatSpec (≤50 words each)
- **constructor** — Set immutable USDC & PerpManager addresses; disables initialize.
- **initialize** — One-time call approving PerpManager to spend max USDC and sets contract owner.
- **name/symbol/asset** — Metadata overrides for ERC-4626.
- **queueWithdrawal** — User requests withdrawal of `shares`; records request and emits event.
- **cancelWithdrawal** — Sender cancels own queued withdrawal; updates accounting.
- **processWithdrawals** — Admin batch-executes first `num` queued withdrawals, burns shares, transfers USDC.
- **grantAdminRole / revokeAdminRole** — Owner manages ADMIN_ROLE addresses.
- **approveOperator / disapproveOperator** — Owner syncs admin operator status inside PerpManager.
- **addSubaccount / removeSubaccount** — PerpManager registers or drops a linked trading sub-account.
- **totalAssets / totalAccountValue / orderbookCollateral / freeCollateralBalance** — Aggregate vault asset valuation helpers.
- **Getter group** — Expose queue, shares, subaccounts and role status.
- **previewWithdraw / maxWithdraw / maxRedeem** — Disabled 4626 helpers, always return 0.



## SUMMARY OF FILE: 2025-08-gte-perps/contracts/perps/PerpManager.sol
### PerpManager (<=250 words)
Permissioned façade over the clearing-house & collateral subsystems for GTE’s Perpetual-Futures CLOB.  Users (or whitelisted operators) move USDC between free collateral, margin, spot, and order-book escrows, as well as place/amend/cancel standard or “backstop” limit orders.  Risk checks, funding-payment realisation, and liquidation toggles live in the inherited panels; PerpManager only orchestrates calls and enforces role-based access.

Trust model
• Users custody collateral in CollateralManager; PerpManager only moves it under explicit calls or operator delegation.
• AdminPanel can pause protocol; LiquidatorPanel can liquidate; OperatorHub grants delegated roles.
• No owner funds at risk inside PerpManager itself.

Storage variables
- accountManager (IAccountManager)  : immutable ptr to spot account bridge.
(Additional state sits in inherited contracts & external libraries.)

Major external entrypoints
1. deposit(account, amount) external – add free collateral from caller. non-payable
2. withdraw(account, amount) external – remove free collateral to caller.
3. depositTo(account, amount) external – 3rd-party top-up.
4. depositFromSpot(account, amount) external – pull USDC from spot AccountManager.
5. withdrawToSpot(account, amount) external – bridge back to spot (AccountManager only).
6. addMargin(account, sub, amount) external – move free collateral → position margin.
7. removeMargin(account, sub, amount) external – redeem margin → free collateral.
8. setPositionLeverage(asset, account, sub, leverage) external returns(int) – change leverage, adjusts collateral.
9. placeOrder(account, args) external returns(PlaceOrderResult) – market / limit / IOC on standard book.
10. postLimitOrderBackstop(account, args) external – MOC-only orders on backstop book.
11. amendLimitOrder(account, args) external returns(int) – modify price/qty, std book.
12. cancelLimitOrders(asset, account, sub, ids[]) external returns(uint) – std book refund.
13. amendLimitOrderBackstop(...)
14. cancelLimitOrdersBackstop(...)
15. cancelConditionalOrders(account, nonces[]) external – revoke queued triggers.

Internal helpers
- _getCollateral(baseAmt, price, leverage) private pure → collateral

Function signatures, vis./mutability/modifiers & ≤50-word NatSpec
```solidity
function deposit(address account,uint256 amount) external nonpayable onlySenderOrOperator; /// Transfer USDC from msg.sender to account’s free collateral balance.
function withdraw(address account,uint256 amount) external nonpayable onlySenderOrOperator; /// Burn free collateral and send USDC to caller, post-risk checks.
function depositTo(address account,uint256 amount) external nonpayable; /// Third party tops up `account` free collateral.
function depositFromSpot(address account,uint256 amount) external nonpayable onlySenderOrOperator; /// Pull USDC from AccountManager spot vault then credit free collateral.
function withdrawToSpot(address account,uint256 amount) external nonpayable; /// AccountManager callback that transfers margin back to spot vault.
function addMargin(address account,uint256 sub,uint256 amount) external nonpayable onlySenderOrOperator; /// Convert free collateral into position margin; realises funding.
function removeMargin(address account,uint256 sub,uint256 amount) external nonpayable onlySenderOrOperator; /// Withdraw margin after risk checks; realises funding.
function setPositionLeverage(bytes32 asset,address account,uint256 sub,uint256 leverage) external nonpayable onlySenderOrOperator returns(int256); /// Adjust leverage & collateral for open/queued orders.
function placeOrder(address account,PlaceOrderArgs calldata args) external nonpayable onlySenderOrOperator returns(PlaceOrderResult); /// Submit order to matching engine (standard book).
function postLimitOrderBackstop(address account,PlaceOrderArgs calldata args) external nonpayable onlySenderOrOperator returns(PlaceOrderResult); /// Post MOC backstop order.
function amendLimitOrder(address account,AmendLimitOrderArgs calldata args) external nonpayable onlySenderOrOperator returns(int256); /// Modify existing standard limit order.
function cancelLimitOrders(bytes32 asset,address account,uint256 sub,uint256[] calldata ids) external nonpayable onlySenderOrOperator returns(uint256); /// Cancel selected limit orders and refund unused margin.
function amendLimitOrderBackstop(address account,AmendLimitOrderArgs calldata args) external nonpayable onlySenderOrOperator returns(int256); /// Amend backstop limit order.
function cancelLimitOrdersBackstop(bytes32 asset,address account,uint256 sub,uint256[] calldata ids) external nonpayable onlySenderOrOperator returns(uint256); /// Cancel backstop orders and refund.
function cancelConditionalOrders(address account,uint256[] calldata nonces) external nonpayable onlySenderOrOperator; /// Invalidate off-chain conditional order signatures.
```


## SUMMARY OF FILE: 2025-08-gte-perps/contracts/perps/modules/LiquidatorPanel.sol
### LiquidatorPanel.sol (≈240 loc)
Permissioned module that executes all forced position reductions on the perpetual CLOB.  It interacts with global singleton storage via `StorageLib` and therefore holds **no contract-level storage** of its own.  A caller must own `ADMIN_ROLE` or the respective **LIQUIDATOR / BACKSTOP_LIQUIDATOR** role; additionally the global `ClearingHouse.active` flag must be true.  All user funds remain in the shared `CollateralManager`/`InsuranceFund`; this contract only orchestrates state transitions and fee accounting.

Main entrypoints
1. `liquidate(bytes32 asset, address account, uint256 sub)` – standard maintenance-margin liquidation.
2. `backstopLiquidate(bytes32 asset, address, uint256)` – secondary auction after failed normal liquidation; distributes prorated margin to back-stop keepers.
3. `deleverage(bytes32 asset, DeleveragePair[] pairs)` – ADL that matches an underwater maker with solvent taker at maker bankruptcy price.
4. `delistClose(bytes32 asset, Account[] accounts)` – forcibly closes every open position once a market is flagged `DELISTED`.

Storage variables
(none)

Function interface & short NatSpec
- `liquidate(asset, account, subaccount) external onlyLiquidator onlyActiveProtocol`  
  Performs full position close via orderbook, settles PnL, pays/claims insurance.  Emits `Liquidation`.
- `backstopLiquidate(asset, account, subaccount) external onlyBackstopLiquidator onlyActiveProtocol`  
  Same as `liquidate` but uses back-stop book; distributes prorated margin to winning keepers.
- `deleverage(asset, pairs) external onlyLiquidator onlyActiveProtocol`  
  Auto-deleveraging loop; iterates through maker/taker pairs and partially closes both sides at maker bankruptcy price.
- `delistClose(asset, accounts) external onlyLiquidator onlyActiveProtocol`  
  Closes remaining positions for delisted market at last mark price.

Internal helpers: `_liquidate`, `_deleveragePair`, `_deleverage`, `_delistClose`, `_settleBackstopLiquidation`, etc.  These encapsulate bookkeeping, validation and event emission but hold no persistent state.


## SUMMARY OF FILE: 2025-08-gte-perps/contracts/clob/CLOB.sol
### CLOB.sol – Spot Central-Limit Order Book

**Purpose & Trust Model (≤250 words)**  
Permissionless on-chain spot order book allowing any user (or approved operator) to place, amend and cancel limit/market orders for a base/quote pair. Users deposit tokens into `AccountManager`; the CLOB only keeps in-contract accounting in a `Book` struct. Matching, fee computation and balance transfers are delegated to `AccountManager` / `CLOBManager` ensuring no direct custody inside the CLOB itself. Admin (factory) can tune tick/lot sizes and clean up expired orders but cannot touch user funds. Max orders per side is capped; least-competitive orders are automatically popped.

**Storage Variables**  
• ABI_VERSION (uint256) – version for indexers  
• gteRouter (address) – trusted router bypassing op-check  
• operator (IOperatorPanel) – RBAC registry  
• factory (ICLOBManager) – parent that created book  
• accountManager (IAccountManager) – token vault + settle  
• maxNumOrdersPerSide (uint256) – hard cap per side  
(Per-market state is kept in an unstructured `Book` slot via `CLOBStorageLib`).

**External / Public Functions**  
• constructor(address,address,address,uint256) external – Sets immutables. Non-payable.  
• initialize(MarketConfig,MarketSettings,address) external initializer – one-time setup.  
• getBaseToken() external view – returns base ERC20.  
• getQuoteToken() external view – returns quote ERC20.  
• getBaseTokenAmount(uint256 price,uint256 quote) external view – quote→base converter.  
• getQuoteTokenAmount(uint256 price,uint256 base) external view – base→quote converter.  
• getMarketConfig() external view – full pair config.  
• getMarketSettings() external view – mutable parameters.  
• getTickSize() / getLotSizeInBase() external view – returns granularities.  
• getOpenInterest() external view – aggregated maker OI.  
• getOrder(uint256) external view – order struct by id.  
• getTOB() external view – best bid / ask.  
• getLimit(uint256,Side) external view – returns price bucket.  
• getNumBids() / getNumAsks() external view – book depth.  
• getNextOrders(uint256,uint256) external view – sequential pagination.  
• getNextBiggestPrice(uint256,Side) / getNextSmallestPrice(...) external view – adjacent price discovery.  
• getNextOrderId() external view – nonce preview.  
• getEventNonce() external view – global event counter.  
• getOrdersPaginated(uint256,Side,uint256) external view – TOB pagination.  
• getOrdersPaginated(OrderId,uint256) external view – id-based pagination.  
• getBaseQuanta() external view – lot-sized unit.  
• setMaxLimitsPerTx(uint8) external onlyManager – adjust per-tx caps. Non-payable.  
• setTickSize(uint256) external onlyManager – update tick.  
• setMinLimitOrderAmountInBase(uint256) external onlyManager – size floor.  
• setLotSizeInBase(uint256) external onlyManager – standardize size.  
• adminCancelExpiredOrders(OrderId[],Side) external onlyManager – bulk expiry clear.  
• placeOrder(address,PlaceOrderArgs) external onlySenderOrOperator – create market/limit order; returns PlaceOrderResult.  
• amend(address,AmendArgs) external onlySenderOrOperator – modify price/size/TIF.  
• cancel(address,CancelArgs) external onlySenderOrOperator – cancel orders.

Each mutative method emits detailed events (`OrderProcessed`, `OrderCanceled`, etc.) to aid off-chain matching engines.



## SUMMARY OF FILE: 2025-08-gte-perps/contracts/clob/CLOBManager.sol
### CLOBManager (250-word summary)
Permissioned factory & admin contract that governs spot Central-Limit Order Book (CLOB) markets.  It deploys new markets as BeaconProxy clones, registers them in an external AccountManager, and exposes admin endpoints to tune market parameters and manage fee/limit whitelists.  Users never deposit tokens here; funds stay in individual CLOB markets.  Trust model: owner or addresses with specific bit-flag roles (MARKET_MANAGER, FEE_TIER_SETTER, MAX_LIMIT_WHITELISTER, EXPIRED_ORDER_CLEARER) can mutate state; everyone else can only query.  Contract is non-upgradeable but each market is upgrade-ready through the immutable beacon.

#### Storage
1. beacon – address – Beacon for CLOB implementations.
2. accountManager – IAccountManager – external registry.
3. isCLOB – mapping(address⇒bool) – true if market produced by this manager.
4. clob – mapping(bytes32⇒address) – tokenPairHash → market address.
5. maxLimitWhitelist – mapping(address⇒bool) – exempt from per-tx limits.
6. ABI_VERSION/role bitmasks – uint256 constants.

#### Major External Functions
• getMarketAddress(tokenA,tokenB) external view – returns market.
• isMarket(market) external view – validity check.
• getMaxLimitExempt(account) external view – whitelist status.
• getEventNonce() external view – global event counter.
• createMarket(base,quote,settings) external onlyOwnerOrRoles – deploys BeaconProxy, stores, emits MarketCreated.
• setTickSize(market,size) external onlyOwnerOrRoles – admin adjust.
• setLotSizeInBase(market,size) external onlyOwnerOrRoles – admin adjust.
• setMinLimitOrderAmountInBase(market,amt) external onlyOwnerOrRoles – admin adjust.
• adminCancelExpiredOrders(market,ids,side) external onlyOwnerOrRoles – force cancel.
• setAccountFeeTiers(accounts,tiers) external onlyOwnerOrRoles – batch fee tier set.
• setMaxLimitsExempt(accounts,toggles) external onlyOwnerOrRoles – toggle whitelist.
• setMaxLimitsPerTx(market,newMax) external onlyOwnerOrRoles – update per-tx limit.

---
#### Function Interfaces & DevDocs (≤50 words each)
```solidity
constructor(address beacon,address accountMgr) // Sets immutables, disables initializers.
initialize(address owner) external initializer // One-shot owner setup for proxy deployment.
getMarketAddress(address,address) external view returns(address) // Fetch market by unordered pair.
isMarket(address) external view returns(bool) // True if produced by this contract.
getMaxLimitExempt(address) external view returns(bool) // Check whitelist bypass.
getEventNonce() external view returns(uint256) // Global monotonic event id.
createMarket(address,address,SettingsParams) external onlyOwnerOrRoles returns(address) // Deploys CLOB via BeaconProxy after validation.
setTickSize(ICLOB,uint256) external onlyOwnerOrRoles // Adjust price tick granularity.
setLotSizeInBase(ICLOB,uint256) external onlyOwnerOrRoles // Adjust lot size.
setMinLimitOrderAmountInBase(ICLOB,uint256) external onlyOwnerOrRoles // Adjust min order size.
adminCancelExpiredOrders(ICLOB,OrderId[],Side) external onlyOwnerOrRoles // Bulk cancel aged orders.
setAccountFeeTiers(address[],FeeTiers[]) external onlyOwnerOrRoles // Batch fee tier config.
setMaxLimitsExempt(address[],bool[]) external onlyOwnerOrRoles // Toggle max-limit whitelist.
setMaxLimitsPerTx(ICLOB,uint8) external onlyOwnerOrRoles // Adjust limits per transaction.
```


## SUMMARY OF FILE: 2025-08-gte-perps/contracts/router/GTERouter.sol
### GTERouter.sol – Summary (≤ 250 words)
Permission-less front-end router that aggregates AccountManager, CLOB orderbooks, Launchpad bonding curve, Permit2 and Uniswap-V2 swaps.  It never custody funds longer than one transaction: tokens withdrawn from the user are either (a) re-deposited into AccountManager under the caller, (b) forwarded to a CLOB fill, (c) swapped via AMM, or (d) sold/bought through Launchpad.  No owner/admin functions exist; core addresses are immutable constructor params, so users only trust the external contracts it calls.

---
#### Storage variables
• `uint256 ABI_VERSION` – hard-coded ABI tag  
• `WETH weth` – canonical WETH instance  
• `ILaunchpad launchpad` – bonding-curve Launchpad  
• `IAccountManager acctManager` – spot margin account system  
• `ICLOBManager clobAdminPanel` – registry that verifies CLOB markets  
• `IAllowanceTransfer permit2` – Uniswap Permit2 helper  
• `IUniswapV2Router01 uniV2Router` – AMM router

---
#### External / public entry-points
1. `spotDeposit(address token,uint256 amount,bool fromRouter) external` – non-payable.  Deposits ERC-20 into AccountManager; can pull from user or expect prior transfer.
2. `spotDepositPermit2(address token,uint160 amount,PermitSingle permit,bytes sig) external` – non-payable.  Uses Permit2 to transfer + deposit in a single tx.
3. `wrapSpotDeposit() external payable` – payable.  Wraps ETH to WETH then deposits.
4. `spotWithdraw(address token,uint256 amount) external` – non-payable.  Withdraws from AccountManager to caller.
5. `clobCancel(ICLOB clob,CancelArgs args) external` – non-payable, `isMarket`. Cancels user order; returns refunded balances.
6. `clobAmend(ICLOB clob,AmendArgs args) external` – non-payable, `isMarket`. Amend limit order sizes/prices.
7. `clobPlaceOrder(ICLOB clob,PlaceOrderArgs args) external` – non-payable, `isMarket`. Places order on CLOB.
8. `launchpadSell(address token,uint256 inBase,uint256 minQuote) external nonReentrant` – Sells launch token via bonding curve.
9. `launchpadBuy(address token,uint256 outBase,address quote,uint256 maxQuote) external nonReentrant` – Buys launch token.
10. `executeRoute(address tokenIn,uint256 amountIn,uint256 minOut,uint256 deadline,bytes[] hops) external nonReentrant` – Aggregator that chains arbitrary hops (CLOB fill & UniV2 swap) with slippage/timeout guard.

Every function uses msg.sender as the beneficiary; no privileged actors.



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

