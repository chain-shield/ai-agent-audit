## Verified Patterns Found: 5

## Verified Patterns Found in following Categories:

- PermitMisuse
- PermitOrSignatureReplay
- ReserveOrPriceDesync
- AccountingInvariantViolation



## Summary of Patterns

Launchpad fee accounting lets LPs steal from pool via mint/burn desync

RewardsTracker uses uint96 rewardDebt, risking overflow and broken rewards invariants

LP token permit lacks s-range and v validation, allowing malleable signatures

Static DOMAIN_SEPARATOR in LP token permit allows cross-chain signature replay

LP token permit lacks s-value and v-range checks (signature malleability)

## Patterns



 ### Issue Type: ReserveOrPriceDesync

 ### Relevant Function/Location: GTELaunchpadV2Pair.mint

 ### Title
Launchpad fee accounting lets LPs steal from pool via mint/burn desync
 ### Description/Code Snippet
GTELaunchpadV2Pair introduces custom accounting for launchpad fees via `accruedLaunchpadFee0/1` that are subtracted from balances when updating reserves, but `mint` and `burn` use inconsistent bases for liquidity and payout calculations. This can let a liquidity provider capture part of the launchpad fee balance (meant for the Distributor) and then still have that same fee amount later sent to the Distributor, effectively draining reserves from other LPs.

Key pieces:

- Reserves are stored *net* of launchpad fees in `_update`:
```solidity
function _update(
    uint256 balance0,
    uint256 balance1,
    uint112 _reserve0,
    uint112 _reserve1,
    uint112 newLaunchpadFee0,
    uint112 newLaunchpadFee1
) private {
    // ...
    uint112 totalLaunchpadFee0 = accruedLaunchpadFee0 + newLaunchpadFee0;
    uint112 totalLaunchpadFee1 = accruedLaunchpadFee1 + newLaunchpadFee1;
    // ...
    reserve0 = _reserve0 = uint112(balance0) - totalLaunchpadFee0;
    reserve1 = _reserve1 = uint112(balance1) - totalLaunchpadFee1;
}
```
`reserve{0,1}` represent AMM reserves **excluding** all accumulated launchpad fees (`accruedLaunchpadFee{0,1}`), while the actual token balances held by the contract are:

`balance{0,1} = reserve{0,1} + accruedLaunchpadFee{0,1}` (ignoring donations).

- `swap` accrues launchpad fees only when `rewardsPoolActive > 0` and `launchpadFeeDistributor != 0`:
```solidity
(uint112 launchpadFee0, uint112 launchpadFee1) =
    launchpadFeeDistributor > address(0) && rewardsPoolActive > 0
        ? _getLaunchpadFees(amount0In, amount1In)
        : (uint112(0), uint112(0));

_update(balance0, balance1, _reserve0, _reserve1, launchpadFee0, launchpadFee1);
```
On the **second+ swap in a block**, `_update` runs with `timeElapsed == 0`, so it *accrues* the new launchpad fee into `accruedLaunchpadFee{0,1}` and sets reserves net of it. Those accrued fees remain in the contract balance but are not yet sent to the `Distributor`.

- `mint` uses the raw balance–reserve difference as the deposited amounts, without excluding accrued launchpad fees:
```solidity
function mint(address to) external lock returns (uint256 liquidity) {
    (uint112 _reserve0, uint112 _reserve1,) = getReserves();
    uint256 balance0 = IERC20(token0).balanceOf(address(this));
    uint256 balance1 = IERC20(token1).balanceOf(address(this));
    uint256 amount0 = balance0.sub(_reserve0);
    uint256 amount1 = balance1.sub(_reserve1);
    // ...
    liquidity = Math.min(amount0.mul(_totalSupply) / _reserve0,
                         amount1.mul(_totalSupply) / _reserve1);
    // ...
    _mint(to, liquidity);
    _update(balance0, balance1, _reserve0, _reserve1, uint112(0), uint112(0));
}
```
If there is a non-zero `accruedLaunchpadFee0` or `accruedLaunchpadFee1` (from previous swaps **in the same block**), then

`amountX = balanceX - _reserveX = user_depositX + accruedLaunchpadFeeX`.

So `mint` counts already-accrued launchpad fees as if they were part of the user's deposit.

- `burn` pays out based on the full current balances (including accrued fees), not the reserves:
```solidity
function burn(address to) external lock returns (uint256 amount0, uint256 amount1) {
    (uint112 _reserve0, uint112 _reserve1,) = getReserves();
    address _token0 = token0;
    address _token1 = token1;
    uint256 balance0 = IERC20(_token0).balanceOf(address(this));
    uint256 balance1 = IERC20(_token1).balanceOf(address(this));
    uint256 liquidity = balanceOf[address(this)];

    // Payout based on balances, not reserves
    amount0 = liquidity.mul(balance0) / _totalSupply;
    amount1 = liquidity.mul(balance1) / _totalSupply;
    // ...
    _safeTransfer(_token0, to, amount0);
    _safeTransfer(_token1, to, amount1);
    balance0 = IERC20(_token0).balanceOf(address(this));
    balance1 = IERC20(_token1).balanceOf(address(this));

    _update(balance0, balance1, _reserve0, _reserve1, uint112(0), uint112(0));
}
```
Later, when `_update` is called in a **later block** (where `timeElapsed > 0`), it will distribute the full `accruedLaunchpadFee{0,1}` to the `Distributor`:
```solidity
if (timeElapsed > 0 && _reserve0 != 0 && _reserve1 != 0) {
    // ...
    if (launchpadFeeDistributor > address(0)) {
        if (totalLaunchpadFee0 | totalLaunchpadFee1 > 0) {
            delete accruedLaunchpadFee0;
            delete accruedLaunchpadFee1;
            _distributeLaunchpadFees(totalLaunchpadFee0, totalLaunchpadFee1);
        }
    }
}
```

**Exploit pattern (high-level):**

1. Ensure an `_update` has run earlier in the block (e.g. via `sync()`), so the next `_update` in the same block has `timeElapsed == 0`.
2. Perform a `swap` so that `_getLaunchpadFees` returns a non-zero fee (e.g., fees accrued on token0). Because `timeElapsed == 0`, this fee is stored in `accruedLaunchpadFee0`, and reserves are set net of it, but the fee tokens remain in the pair's `balanceOf`.
3. In the **same block**, call `mint` depositing only on the *other* side (e.g., deposit only token1), so that `amount0 = balance0 - _reserve0` is effectively equal to the accrued fee on token0, and `amount1` is the user's real deposit. Because `amount0` includes the fee, the LP `liquidity` minted is boosted even though the user never deposited token0.
4. In a later block, transfer LP tokens back to the pair and call `burn`. The payout uses the full `balance0/balance1` (which include the accrued fee), so the attacker redeems a share of both reserves and the launchpad fee.
5. When `_update` runs in this later block, `timeElapsed > 0` and `totalLaunchpadFee0` is still the original accrued fee, so `_distributeLaunchpadFees` transfers the full fee amount to the `Distributor`, *again*, now funded out of the remaining reserves.

Net result: the attacker has received a portion of the launchpad fee once via `burn`, and that same fee is later sent to the Distributor, effectively debiting the pool reserves twice. This breaks the intended invariant that `accruedLaunchpadFee{0,1}` always corresponds 1:1 to extra tokens sitting on top of the reserves and causes a desync between how the AMM math thinks about reserves and who actually owns which tokens.

Economically, this produces a drain from other LPs (and/or the launchpad LP) into the attacker, triggered purely via permissionless `swap`, `mint`, and `burn` calls, and depends only on normal same-block fee accrual behavior, not on any privileged action.

 ### Static Signals
Custom fee buckets accruedLaunchpadFee0/1 tracked separately from reserves, _update sets reserves as balance - totalLaunchpadFee, mint calculates deposited amounts as balance - reserve without excluding accrued fees, burn distributes based on full balances, not reserves, fee distribution delayed until timeElapsed > 0, allowing same-block accrued fees to be counted as deposits
 ### Assets at Risk
LP liquidity in GTELaunchpadV2Pair, launchpad LP share, overall pool reserves
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: RewardsTrackerLib.claim

 ### Title
RewardsTracker uses uint96 rewardDebt, risking overflow and broken rewards invariants
 ### Description/Code Snippet
The rewards distribution logic in `RewardsTrackerLib` stores per-user reward debt in `uint96` but computes accumulated rewards in full `uint256` precision, creating a potential overflow/truncation point that can break the accounting invariant between `totalPendingRewards` and per-user entitlements.

Relevant code (struct and methods):

```solidity
struct UserRewardData {
    uint96 shares;           // User's current share count
    uint96 baseRewardDebt;   // Used to calculate base token rewards owed
    uint96 quoteRewardDebt;  // Used to calculate quote token rewards owed
}

uint128 public constant PRECISION_FACTOR = 1e12;

function totalAccRewards(uint256 shares, uint256 accRewardsPerShare)
    internal
    pure
    returns (uint256)
{
    return (shares * accRewardsPerShare) / PRECISION_FACTOR;
}

function claim(RewardPoolData storage self, address user)
    internal
    returns (uint256 baseAmount, uint256 quoteAmount)
{
    UserRewardData storage userData = self.userRewards[user];
    uint256 shares = uint256(userData.shares);
    if (shares == 0) revert ZeroShareClaim();

    (uint256 accBaseRewardsPerShare, uint256 accQuoteRewardsPerShare) = self.update();

    uint256 totalAccBaseRewards = totalAccRewards(shares, accBaseRewardsPerShare);
    uint256 totalAccQuoteRewards = totalAccRewards(shares, accQuoteRewardsPerShare);

    baseAmount = totalAccBaseRewards - uint128(userData.baseRewardDebt);
    quoteAmount = totalAccQuoteRewards - uint128(userData.quoteRewardDebt);

    // DOWNCAST to uint96 without bounds check
    userData.baseRewardDebt = uint96(totalAccBaseRewards);
    userData.quoteRewardDebt = uint96(totalAccQuoteRewards);
}
```

The accumulators `accBaseRewardPerShare` / `accQuoteRewardPerShare` and the result of `totalAccRewards` are `uint256`, with no explicit cap other than what token amounts and reward history allow. These values are then cast to `uint96` for storage in `baseRewardDebt` / `quoteRewardDebt` **without any bounds check**. In Solidity 0.8, narrowing casts silently truncate higher bits (modulo 2^96).

If the product `shares * accRewardsPerShare / PRECISION_FACTOR` for a user ever exceeds `2^96 - 1` (≈7.9e28), then:
- `userData.baseRewardDebt` will be stored as the lower 96 bits of `totalAccBaseRewards` instead of the full value.
- On subsequent `claim` / `stake` / `unstake`, pending rewards are computed as:
  ```solidity
  baseAmount = totalAccRewards(shares, accBaseRewardsPerShare) - userData.baseRewardDebt;
  ```
  Because `userData.baseRewardDebt` is truncated, `baseAmount` is artificially inflated by a multiple of `2^96` tokens compared to the true intended amount.

This breaks the accounting invariant that:

- "Sum of all users' `pending rewards` <= `Distributor.totalPendingRewards[asset]` <= actual token balance".

Downstream in `Distributor._distributeAssets`:

```solidity
function _distributeAssets(address base, uint256 baseAmount, address quote, uint256 quoteAmount) internal {
    if (baseAmount > 0) {
        _decreaseTotalPending(base, baseAmount);
        base.safeTransfer(msg.sender, baseAmount);
    }
    ...
}

function _decreaseTotalPending(address asset, uint256 amount) internal {
    uint256 currTotal = totalPendingRewards[asset];
    if (currTotal < amount) revert ClaimAmountExceedsTotalPendingRewards();
    unchecked { totalPendingRewards[asset] -= amount; }
}
```

Effects of the truncation bug:
- A user whose `baseRewardDebt` overflowed can observe a very large `baseAmount` from `claimRewards`.
- If `amount <= totalPendingRewards[asset]`, they can **drain more rewards than their fair share**, stealing rewards from other stakers.
- If `amount > totalPendingRewards[asset]`, **all `claimRewards` / `increaseStake` / `decreaseStake` paths that try to realize rewards for that asset will revert with `ClaimAmountExceedsTotalPendingRewards`**, effectively bricking further reward distribution for that pool.

Whether the overflow is reachable depends on real-world reward magnitudes and durations, but the type choice (`uint96` for per-user debt vs `uint256` for accumulators) plus unchecked downcast is a clear instance of the `AccountingInvariantViolation` pattern affecting the correctness and safety of reward accounting.
 ### Static Signals
reward debt stored as uint96 while accumulators are uint256, downcast from uint256 to uint96 without bounds check, pending rewards derived from truncated debt can exceed total pending, potential break of emitted == claimed + unclaimed style invariant
 ### Assets at Risk
rewards
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PermitMisuse

 ### Relevant Function/Location: GTELaunchpadV2Pair.permit

 ### Title
LP token permit lacks s-range and v validation, allowing malleable signatures
 ### Description/Code Snippet
The `permit` implementation on the LP token (via `UniswapV2ERC20`) uses raw `ecrecover` and only checks that the recovered address equals `owner`, but does not enforce the EIP-2 requirement that `s` lies in the lower half-order of the secp256k1 curve and does not restrict `v` to {27, 28} beyond what `ecrecover` implicitly accepts.

```solidity
function permit(
    address owner,
    address spender,
    uint256 value,
    uint256 deadline,
    uint8 v,
    bytes32 r,
    bytes32 s
) external {
    require(deadline >= block.timestamp, "UniswapV2: EXPIRED");
    bytes32 digest = keccak256(
        abi.encodePacked(
            "\x19\x01",
            DOMAIN_SEPARATOR,
            keccak256(
                abi.encode(
                    PERMIT_TYPEHASH,
                    owner,
                    spender,
                    value,
                    nonces[owner]++,
                    deadline
                )
            )
        )
    );
    address recoveredAddress = ecrecover(digest, v, r, s);
    require(recoveredAddress != address(0) && recoveredAddress == owner, "UniswapV2: INVALID_SIGNATURE");
    _approve(owner, spender, value);
}
```

Static signals:
- No check that `uint256(s) <= secp256k1n/2` (no EIP-2 low-s enforcement).
- No explicit check that `v` is 27 or 28 (relies solely on `ecrecover` behavior).

This means each logical signature can have multiple valid `(v, r, s)` encodings (high-s / low-s malleability). While the nonce logic prevents simple *on-chain* replay with the same digest, signature malleability can still cause:
- Off-chain signers or wallets that enforce low-s signatures to be bypassed by alternative high-s encodings the contract accepts.
- Integrations that expect a canonical unique signature representation to be broken, complicating replay protection or signature tracking across systems.

This matches the Top Code4rena `PermitMisuse` pattern (accepting malleable signatures through raw `ecrecover` without s-range and v validation). It weakens the cryptographic robustness of `permit` and can enable subtle cross-system replay/validation inconsistencies even if a direct theft path may require additional conditions.
 ### Static Signals
uses ecrecover directly, no check that s <= secp256k1n/2, no explicit v == 27 || v == 28 check, accepts potentially malleable signatures in permit()
 ### Assets at Risk
LP token approvals / allowances, integrations relying on canonical EIP-2612 signatures
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PermitOrSignatureReplay

 ### Relevant Function/Location: GTELaunchpadV2Pair.permit

 ### Title
Static DOMAIN_SEPARATOR in LP token permit allows cross-chain signature replay
 ### Description/Code Snippet
The LP token `permit` implementation (inherited from `UniswapV2ERC20`) precomputes a fixed `DOMAIN_SEPARATOR` in the constructor using the chainId at deployment, and never updates it.

```solidity
contract UniswapV2ERC20 is IUniswapV2ERC20 {
    bytes32 public DOMAIN_SEPARATOR;
    // ...
    constructor() {
        uint256 chainId;
        assembly {
            chainId := chainid()
        }
        DOMAIN_SEPARATOR = keccak256(
            abi.encode(
                keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"),
                keccak256(bytes(name)),
                keccak256(bytes("1")),
                chainId,
                address(this)
            )
        );
    }

    function permit(
        address owner,
        address spender,
        uint256 value,
        uint256 deadline,
        uint8 v,
        bytes32 r,
        bytes32 s
    ) external {
        require(deadline >= block.timestamp, "UniswapV2: EXPIRED");
        bytes32 digest = keccak256(
            abi.encodePacked(
                "\x19\x01",
                DOMAIN_SEPARATOR,
                keccak256(
                    abi.encode(
                        PERMIT_TYPEHASH,
                        owner,
                        spender,
                        value,
                        nonces[owner]++,
                        deadline
                    )
                )
            )
        );
        address recoveredAddress = ecrecover(digest, v, r, s);
        require(recoveredAddress != address(0) && recoveredAddress == owner, "UniswapV2: INVALID_SIGNATURE");
        _approve(owner, spender, value);
    }
}
```

Because `DOMAIN_SEPARATOR` is stored once and never recomputed, if the chain undergoes a fork or chainId change where the contract storage is cloned but the environment's chainId differs, the domain separator on both chains remains identical. Combined with per-owner nonces that start from the same value on both chains at fork time, a valid `permit` signature used on one chain can be replayed on the forked chain, granting the same allowance there without the user’s awareness.

This matches the Top Code4rena `PermitOrSignatureReplay` pattern: the domain separator is effectively not bound to the *runtime* chain, so signatures are not chain-unique across forks. An attacker who controls or intercepts a user’s signed `permit` on chain A can replay it on a forked chain B where the LP token still has value, obtaining unauthorized allowances and then transferring the user’s LP tokens (which correspond to a share of underlying pool assets).
 ### Static Signals
DOMAIN_SEPARATOR computed once in constructor, DOMAIN_SEPARATOR uses chainId at deployment only, permit() uses stored DOMAIN_SEPARATOR without recomputation, no dynamic chainId binding or fork-detection in permit()
 ### Assets at Risk
user LP token balances, LP token allowances, underlying pool assets claimable via unauthorized burns
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PermitMisuse

 ### Relevant Function/Location: UniswapV2ERC20.permit

 ### Title
LP token permit lacks s-value and v-range checks (signature malleability)
 ### Description/Code Snippet
The UniswapV2-style LP token implementation `UniswapV2ERC20` exposes an EIP-2612 `permit` without enforcing standard ECDSA malleability constraints on the signature.

Code (simplified):

```solidity
function permit(
    address owner,
    address spender,
    uint256 value,
    uint256 deadline,
    uint8 v,
    bytes32 r,
    bytes32 s
) external {
    require(deadline >= block.timestamp, "UniswapV2: EXPIRED");
    bytes32 digest = keccak256(
        abi.encodePacked(
            "\x19\x01",
            DOMAIN_SEPARATOR,
            keccak256(
                abi.encode(
                    PERMIT_TYPEHASH,
                    owner,
                    spender,
                    value,
                    nonces[owner]++,
                    deadline
                )
            )
        )
    );
    address recoveredAddress = ecrecover(digest, v, r, s);
    require(
        recoveredAddress != address(0) && recoveredAddress == owner,
        "UniswapV2: INVALID_SIGNATURE"
    );
    _approve(owner, spender, value);
}
```

Issues:
- `ecrecover` is used directly without requiring `s <= secp256k1n/2` (no low-s canonicality).
- `v` is not constrained to {27, 28} (or normalized from {0,1}).

This allows **signature malleability**: from any valid `(v, r, s)` an attacker can derive another valid `(v', r, n-s)` pair that recovers the same `owner`. While nonces prevent simple replay for the *same* digest, malleability can still:
- Complicate off-chain tracking and signature reuse prevention.
- Enable front-running races in integrated flows (router uses `permit` + action) where different representations of the same logical permit are submitted by different parties, increasing attack surface for MEV bots to manipulate which spender ultimately gets the allowance first in complex integrations.

Even if severity might be limited here, this matches the `PermitMisuse` pattern: permit implemented without modern anti-malleability guards.
 ### Static Signals
ecrecover used directly, no s <= secp256k1n/2 check, v not restricted to 27/28
 ### Assets at Risk
LP token approvals, user LP positions routed via permit
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

