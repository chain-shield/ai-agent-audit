## Verified Patterns Found: 20

## Verified Patterns Found in following Categories:

- BeaconOrFactoryAuthorityDrift
- TWAPWindowPinningOrLowLiquidity
- StandardViolation
- AccountingInvariantViolation
- SlippageMissingOrInsufficient
- UnboundedLoops
- ConfigFootgun
- AccessControlOrAuthByPass
- Reentrancy
- FeeOnTransferAssumption



## Summary of Patterns

Missing slippage protection in Positions withdrawal

TokenWrapper fails to update user balances or transfer tokens leading to fund loss and accounting mismatch

TokenWrapper totalSupply invariant violation on burn

TokenWrapper assumes optional ERC20 metadata functions exist

Reentrancy in FlashAccountant via Token Callbacks

FlashAccountant allows arbitrary debt self-reporting via updateDebt

Missing Output Amount Check in Router Swap Overload

Incentives FOT Token Accounting Mismatch

Incentives contract assumes 1:1 transfer for Fee-on-Transfer tokens

TokenWrapper time-lock check applied to wrapping instead of unwrapping

RevenueBuybacks DoS with Fee-On-Transfer tokens

TokenWrapper emits misleading Transfer events for mint/burn

Insolvency with negative rebasing tokens in TokenWrapper

Positions withdrawal lacks minimum output amounts (slippage protection)

Factory enables deployment of deceptive TokenWrappers facilitating phishing

TokenWrapperFactory lacks idempotent deployment

Unbounded loop in TWAMM virtual order execution causes DoS

Oracle extension enforces zero-fee pools enabling cheap manipulation

ConfigFootgun: Broken metadata for Native Token wrappers

ConfigFootgun: Unbounded unlockTime causes metadata reverts

## Patterns



 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Positions.withdraw

 ### Title
Missing slippage protection in Positions withdrawal
 ### Description/Code Snippet
The `withdraw` function in `BasePositions.sol` (inherited by `Positions.sol`) allows liquidity providers to burn liquidity and receive underlying assets. However, the function signature lacks `amount0Min` and `amount1Min` parameters to enforce minimum output amounts. This exposes users to sandwich attacks where an attacker can manipulate the pool price/tick immediately before the withdrawal, causing the user to receive an unfavorable ratio of assets and realize impermanent loss at a manipulated price.
 ### Static Signals
payout calculated at execution time without minimum bound, no minAmountOut parameter in liquidation/redemption
 ### Assets at Risk
liquidity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: TokenWrapper.handleForwardData

 ### Title
TokenWrapper fails to update user balances or transfer tokens leading to fund loss and accounting mismatch
 ### Description/Code Snippet
In `TokenWrapper.sol`, the `handleForwardData` function correctly updates the `CORE` saved balances and debt to reflect wrapping/unwrapping actions, but it completely fails to update the internal `_balanceOf` mapping for the user (mint/burn) and fails to transfer the underlying tokens to the user during unwraps. Specifically, when wrapping (amount > 0), the wrapper's Core balance increases, but the user receives no wrapper tokens (`_balanceOf` is not incremented). When unwrapping (amount < 0), the wrapper's Core balance decreases, but the user is not sent any underlying tokens and their wrapper balance is not decremented. This breaks the accounting invariant `totalSupply == sum(balances)` and results in total loss of funds for users attempting to wrap tokens.
 ### Static Signals
balance tracking references different token than actual holdings, accounting state not updated when underlying asset is swapped/upgraded
 ### Assets at Risk
user funds, underlying token collateral
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: TokenWrapper.transfer

 ### Title
TokenWrapper totalSupply invariant violation on burn
 ### Description/Code Snippet
The `TokenWrapper` contract allows users to transfer tokens to `address(0)` or `address(CORE)`. Transferring to `address(0)` reduces the sender's `_balanceOf` without updating the `totalSupply` (which is tracked in `Core.savedBalances`). Transferring to `address(CORE)` updates a transient `coreBalance` which is wiped at the end of the transaction, effectively burning the tokens while `totalSupply` remains unchanged. Both actions cause `totalSupply` to permanently exceed the sum of all accessible balances, violating the standard ERC20 invariant `totalSupply == sum(balances)`.
 ### Static Signals
totalSupply != sum(balances)
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: TokenWrapper.decimals

 ### Title
TokenWrapper assumes optional ERC20 metadata functions exist
 ### Description/Code Snippet
The `TokenWrapper` contract implements `decimals()`, `name()`, and `symbol()` by directly calling `UNDERLYING_TOKEN.decimals()`, `name()`, and `symbol()`. These functions are optional in the ERC-20 specification. If the underlying token does not implement them (or implements them with a different return type/signature), calls to the wrapper's metadata functions will revert. This breaks integration for strictly compliant but minimal ERC-20 tokens.
 ### Static Signals
UNDERLYING_TOKEN.decimals(), UNDERLYING_TOKEN.name(), UNDERLYING_TOKEN.symbol()
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: Reentrancy

 ### Relevant Function/Location: FlashAccountant.startPayments

 ### Title
Reentrancy in FlashAccountant via Token Callbacks
 ### Description/Code Snippet
The `FlashAccountant` uses `startPayments` and `completePayments` to calculate debt updates based on balance changes. `startPayments` snapshots the balance into transient storage using a key derived from the token address. If a token (e.g., ERC-777) allows reentrancy during transfer, a nested call to `startPayments` for the same token will overwrite the snapshot slot. Upon return, the inner `completePayments` clears the slot, causing the outer `completePayments` to read a zero snapshot, resulting in zero credit for the user's payment. This leads to loss of funds for the user.
 ### Static Signals
tstore, call, tload, balanceOf
 ### Assets at Risk
User funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: FlashAccountant.updateDebt

 ### Title
FlashAccountant allows arbitrary debt self-reporting via updateDebt
 ### Description/Code Snippet
The `FlashAccountant.updateDebt` function allows any caller to modify the debt tracking for `(currentLocker, msg.sender)`. While this is intended for tokens like `TokenWrapper` to report debts incurred by the locker, it effectively allows any contract to inject arbitrary debt entries into the accountant for themselves. A malicious contract could call `updateDebt` to register a debt, forcing the locker to pay it (transfer tokens to the accountant) to unlock, or register a negative debt (credit) to drain any surplus of that specific malicious token from the accountant. While limited to the malicious token itself, this behavior relies on the assumption that only trusted tokens call this function.
 ### Static Signals
msg.data.length != 20, _accountDebt(id, msg.sender, delta)
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: Router.swap

 ### Title
Missing Output Amount Check in Router Swap Overload
 ### Description/Code Snippet
In `Router.sol`, the `swap` overload `swap(PoolKey memory poolKey, bool isToken1, int128 amount, SqrtRatio sqrtRatioLimit, uint256 skipAhead)` calls the internal implementation with `calculatedAmountThreshold` set to `type(int256).min`. This explicitly disables the minimum output amount check (slippage protection) for the swap. If a user utilizes this function relying solely on `sqrtRatioLimit` for protection, and incorrectly sets that limit (e.g., to min/max square root ratio), they will be vulnerable to unlimited slippage and sandwich attacks.
 ### Static Signals
type(int256).min, calculatedAmountThreshold
 ### Assets at Risk
User funds being swapped
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: Incentives.fund

 ### Title
Incentives FOT Token Accounting Mismatch
 ### Description/Code Snippet
The `Incentives.fund` function updates the `funded` state based on the `minimum` amount requested, but uses `safeTransferFrom` without verifying the actual amount received. If a Fee-on-Transfer (FOT) token is used, the contract receives less than `fundedAmount`. The internal accounting `dropState.funded` will reflect the higher amount, leading to insolvency when the last users attempt to claim their share.
 ### Static Signals
safeTransferFrom, state update before transfer, no balance check
 ### Assets at Risk
Incentives contract balance
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: Incentives.fund

 ### Title
Incentives contract assumes 1:1 transfer for Fee-on-Transfer tokens
 ### Description/Code Snippet
The `fund` function in `Incentives.sol` calculates the required funding amount (`fundedAmount`) and transfers it using `SafeTransferLib.safeTransferFrom`, but updates the internal `dropState.funded` accounting based on the `minimum` parameter rather than the actual balance increase. If a Fee-on-Transfer token is used, the contract will track more funds than it actually received. This leads to insolvency where the last users attempting to `claim` will face reverts due to insufficient contract balance.
 ### Static Signals
uses input amount instead of post-transfer delta, accounting based on transfer parameter
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: TokenWrapper.handleForwardData

 ### Title
TokenWrapper time-lock check applied to wrapping instead of unwrapping
 ### Description/Code Snippet
In `TokenWrapper.sol`, the `handleForwardData` function handles both wrapping and unwrapping operations forwarded from Core. The comments state that a positive amount indicates wrapping and a negative amount indicates unwrapping. However, the logic analyzing the `amount` applies the `TooEarly` time-lock check to the negative amount path (wrapping/unwrapping ambiguity notwithstanding). 

Based on the debt accounting in `Core.updateSavedBalances` (where a positive delta increases the user's saved balance, effectively an unwrap operation where the user receives tokens) and `Core.updateDebt` (where a negative debt update decreases the user's debt, effectively a payment), the `amount > 0` path executes an Unwrap (User gets Underlying, Pays Wrapper). 

Crucially, this `amount > 0` path has **no timestamp check**. The `amount < 0` path (Wrap) has the check `if (block.timestamp < UNLOCK_TIME)`. This reverses the intended logic: users can unwrap `gEKUBO` immediately after creation, defeating the vesting mechanism, while wrapping is restricted until the unlock time.
 ### Static Signals
if (amount < 0) { if (block.timestamp < UNLOCK_TIME) revert TooEarly(); }, missing check on amount > 0
 ### Assets at Risk
Underlying tokens locked in TokenWrapper
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: RevenueBuybacks.roll

 ### Title
RevenueBuybacks DoS with Fee-On-Transfer tokens
 ### Description/Code Snippet
The `RevenueBuybacks.roll` function assumes standard ERC20 behavior where the transferred amount equals the received amount. It reads the full `balanceOf(token)` and calls `ORDERS.increaseSellAmount` with this value. `Orders` initiates a transfer via `FlashAccountant`. If the revenue token imposes a fee-on-transfer, the `FlashAccountant` receives less than the requested amount. Since `TWAMM` (via `Orders`) registers a debt for the full `amount` but the Accountant only receives `amount - fee`, the invariant `DebtsNotZeroed` will be violated, causing the transaction to revert. This permanently breaks the automated buyback mechanism for any fee-on-transfer revenue tokens.
 ### Static Signals
balanceOf(token), increaseSellAmount, no balance delta check
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: TokenWrapper.transfer

 ### Title
TokenWrapper emits misleading Transfer events for mint/burn
 ### Description/Code Snippet
The `TokenWrapper` contract mints and burns wrapper tokens via the `transfer` function when interacting with the Core/Accountant. When minting (wrapping), it emits `Transfer(CORE, user, amount)` because the Accountant calls `transfer(to, amount)`. When burning (unwrapping), it emits `Transfer(user, CORE, amount)` because the Accountant calls `transfer(accountant, amount)`. Since `CORE` is a deployed contract address and not `address(0)`, these events interpret `CORE` as a token holder rather than signifying mint/burn operations. This violates standard ERC20 event conventions for minting/burning (which use `address(0)`) and will confuse off-chain indexers, analytics dashboards, and wallets, leading to incorrect supply and balance tracking.
 ### Static Signals
emit Transfer(msg.sender, ...), msg.sender check against CORE
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: TokenWrapper.handleForwardData

 ### Title
Insolvency with negative rebasing tokens in TokenWrapper
 ### Description/Code Snippet
The `TokenWrapper` contract maintains accounting of underlying assets using `Core.updateSavedBalances` based strictly on the input `amount` passed during wrapping/unwrapping. It does not account for balance changes due to rebasing (elastic supply) or deflationary mechanisms of the underlying token. If the underlying token has a negative rebase, `Core`'s actual balance of the token will decrease, but the `TokenWrapper`'s `savedBalances` will remain unchanged. This discrepancy leads to insolvency where the last users to attempt an unwrap will fail because `Core` holds insufficient tokens to fulfill the stored `savedBalance`. Additionally, positive rebases are not captured, causing yield loss for wrapper holders.
 ### Static Signals
accounting based on transfer parameter, not actual balance change, no handling for rebasing tokens
 ### Assets at Risk
underlyingToken
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: BasePositions.withdraw

 ### Title
Positions withdrawal lacks minimum output amounts (slippage protection)
 ### Description/Code Snippet
The `withdraw` function in `BasePositions.sol` (inherited by `Positions.sol`) allows users to burn a specific amount of `liquidity` to receive `token0` and `token1`. However, the function does not accept `minAmount0` or `minAmount1` parameters to enforce slippage protection. The amounts returned are determined by `CORE.updatePosition` based on the current pool price (sqrtRatio). If the pool price is manipulated or experiences high volatility before the transaction executes, the user may receive significantly fewer tokens than expected or an undesirable ratio of tokens, directly exposing them to MEV/sandwich attacks.
 ### Static Signals
withdraw() converts shares to assets at current rate without minimum, payout calculated at execution time without minimum bound
 ### Assets at Risk
user liquidity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: BeaconOrFactoryAuthorityDrift

 ### Relevant Function/Location: TokenWrapperFactory.deployWrapper

 ### Title
Factory enables deployment of deceptive TokenWrappers facilitating phishing
 ### Description/Code Snippet
The `deployWrapper` function allows any user to deploy a `TokenWrapper` for any `underlyingToken` address without validation. The deployed wrapper dynamically mirrors the name and symbol of the underlying token (delegating calls to it). This allows malicious actors to create wrappers for spoofed tokens that appear identical to legitimate ones in metadata. Since the `TokenWrapperFactory` is a trusted protocol contract and emits a `TokenWrapperDeployed` event, users and indexers might mistakenly trust these deceptive wrappers, leading to phishing attacks.
 ### Static Signals
factory sets implementation/strategy from user input, critical asset/token address mutable by external actor
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: TokenWrapperFactory.deployWrapper

 ### Title
TokenWrapperFactory lacks idempotent deployment
 ### Description/Code Snippet
The `deployWrapper` function uses `CREATE2` with a deterministic salt to deploy `TokenWrapper` contracts. It blindly attempts to deploy using `new TokenWrapper{salt: salt}(...)` without checking if the contract already exists. If a wrapper with the same parameters has already been deployed, the transaction will revert due to address collision instead of returning the existing address. This is a violation of the robust factory pattern standard, potentially causing DoS for integrations that attempt to deploy wrappers deterministically without pre-checks.
 ### Static Signals
new TokenWrapper{salt: salt}, no extcodesize check
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnboundedLoops

 ### Relevant Function/Location: TWAMM._executeVirtualOrdersFromWithinLock

 ### Title
Unbounded loop in TWAMM virtual order execution causes DoS
 ### Description/Code Snippet
The `_executeVirtualOrdersFromWithinLock` function iterates through time intervals from the last execution time to the current `block.timestamp` using a `while` loop. If a pool has not been interacted with for a long period, and the time intervals (bins) are densely populated with orders (e.g., every 30-minute bin has an active order), the number of iterations can become very large. Since each iteration involves computationally expensive operations like `CORE.swap`, the gas cost can easily exceed the block gas limit, permanently bricking the pool and freezing all assets within it.
 ### Static Signals
while (time != block.timestamp), call inside loop, state update inside loop
 ### Assets at Risk
liquidity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: TWAPWindowPinningOrLowLiquidity

 ### Relevant Function/Location: Oracle.beforeInitializePool

 ### Title
Oracle extension enforces zero-fee pools enabling cheap manipulation
 ### Description/Code Snippet
The `Oracle` extension's `beforeInitializePool` function strictly enforces that the pool fee must be zero (`key.config.fee() == 0`). While this might be intended to create efficient oracle pools, it removes the economic barrier (swap fees) that typically protects TWAP oracles from manipulation. An attacker can manipulate the pool price, force a snapshot update, and arbitrage the price back with near-zero cost (only gas), rendering the oracle data unsafe for downstream usage.
 ### Static Signals
fee enforcement == 0, oracle relies on low-cost manipulation environment
 ### Assets at Risk
oracle-dependent protocols
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: ConfigFootgun

 ### Relevant Function/Location: TokenWrapperFactory.deployWrapper

 ### Title
ConfigFootgun: Broken metadata for Native Token wrappers
 ### Description/Code Snippet
The `TokenWrapperFactory` allows `deployWrapper` to be called with `underlyingToken` set to `address(0)`, which corresponds to `NATIVE_TOKEN_ADDRESS` (ETH) in the Ekubo protocol. While the core wrapping logic supports `address(0)` correctly, the deployed `TokenWrapper` delegates `name()`, `symbol()`, and `decimals()` calls to the underlying token. Calling these functions on `address(0)` causes a revert (due to missing code/empty return data). This results in a wrapper that functions for swapping but is incompatible with most wallets, explorers, and UIs due to broken metadata views.
 ### Static Signals
no zero-address/known-allowlist checks, owner can set arbitrary token
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ConfigFootgun

 ### Relevant Function/Location: TokenWrapperFactory.deployWrapper

 ### Title
ConfigFootgun: Unbounded unlockTime causes metadata reverts
 ### Description/Code Snippet
The `TokenWrapper` constructor accepts any `uint256` for `unlockTime`. The `name()` function converts this timestamp to a human-readable date string using `toDate()`. If a user or factory caller sets an extremely large `unlockTime` (e.g., `type(uint256).max`), the date conversion library may overflow or produce invalid strings, causing the `name()` function to revert. This effectively bricks the token's metadata view.
 ### Static Signals
owner can set arbitrary unlockTime, no upper bound check
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

