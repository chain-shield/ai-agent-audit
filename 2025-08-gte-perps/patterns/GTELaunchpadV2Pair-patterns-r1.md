## Verified Patterns Found: 9

## Verified Patterns Found in following Categories:

- ReserveOrPriceDesync
- AccountingInvariantViolation
- BeaconOrFactoryAuthorityDrift
- StandardViolation
- PermitMisuse
- PermitFrontRun
- AccessControlOrAuthByPass
- GriefableCallbacks



## Summary of Patterns

Launchpad fee distribution assumes Distributor pulls tokens, enabling reserve/balance desync and fee theft via skim

Launchpad fee distribution callback can brick AMM pair if Distributor reverts

Launchpad fee tokens can plausibly be stolen via skim() depending on Distributor implementation

Permissionless factory.createPair lets anyone block launchpad-wired pair creation

Launchpad fee distribution callback can brick AMM pair (griefable external dependency)

Deterministic GTELaunchpadV2Pair deployment can be front‑run and permanently block launchpad-enabled pair

LP token permit can be front-run due to weak signature validation (no low-s / v checks)

Permit signature malleability due to missing v/s validation in LP token

Permissionless factory allows front‑running of launchpad pair creation, breaking launchpad fee wiring

## Patterns



 ### Issue Type: ReserveOrPriceDesync

 ### Relevant Function/Location: GTELaunchpadV2Pair.skim

 ### Title
Launchpad fee distribution assumes Distributor pulls tokens, enabling reserve/balance desync and fee theft via skim
 ### Description/Code Snippet
In `GTELaunchpadV2Pair`, the launchpad fee accounting assumes that the external `Distributor` contract will always `transferFrom` the exact `fee0`/`fee1` amounts after the pair sets an allowance. Reserves are then computed under that assumption, which can desync internal reserves from the actual token balances if the Distributor implementation ever deviates (e.g., upgrade bug, partial pull, or no pull).

Key code paths:

1. Fee accrual and distribution are wired through `_update`:

```solidity
function _update(
    uint256 balance0,
    uint256 balance1,
    uint112 _reserve0,
    uint112 _reserve1,
    uint112 newLaunchpadFee0,
    uint112 newLaunchpadFee1
) private {
    ...
    uint112 totalLaunchpadFee0 = accruedLaunchpadFee0 + newLaunchpadFee0;
    uint112 totalLaunchpadFee1 = accruedLaunchpadFee1 + newLaunchpadFee1;
    ...
    if (timeElapsed > 0 && _reserve0 != 0 && _reserve1 != 0) {
        ...
        if (launchpadFeeDistributor > address(0)) {
            if (totalLaunchpadFee0 | totalLaunchpadFee1 > 0) {
                delete accruedLaunchpadFee0;
                delete accruedLaunchpadFee1;
                _distributeLaunchpadFees(totalLaunchpadFee0, totalLaunchpadFee1);
            }
        }
    } else if (launchpadFeeDistributor > address(0) && newLaunchpadFee0 | newLaunchpadFee1 > 0) {
        accruedLaunchpadFee0 = totalLaunchpadFee0;
        accruedLaunchpadFee1 = totalLaunchpadFee1;
        emit LaunchpadFeesAccrued(newLaunchpadFee0, newLaunchpadFee1);
    }

    // Balances contain both accrued and new launchpad fees earned this tx
    // as balance is called before any fee distributions, so subtract the total
    reserve0 = _reserve0 = uint112(balance0) - totalLaunchpadFee0;
    reserve1 = _reserve1 = uint112(balance1) - totalLaunchpadFee1;
    ...
}
```

2. Fee distribution itself only **approves** and calls `Distributor.addRewards`, but does not verify that tokens were actually pulled:

```solidity
function _distributeLaunchpadFees(uint112 fee0, uint112 fee1) internal {
    if ((fee0 | fee1) > 0) {
        address _token0 = token0;
        address _token1 = token1;
        address distributor = launchpadFeeDistributor;

        // Since only pairs created by the launchpad can accrue fee tracking, the tokens are trusted
        if (fee0 > 0) _safeApprove(_token0, distributor, uint256(fee0));
        if (fee1 > 0) _safeApprove(_token1, distributor, uint256(fee1));

        IDistributor(distributor).addRewards(_token0, _token1, uint128(fee0), uint128(fee1));

        emit LaunchpadFeesCollected(fee0, fee1);
    }
}
```

3. `skim()` later treats `reserve + accruedLaunchpadFee` as the reference for protocol-owned balances and transfers any excess to an arbitrary `to`:

```solidity
function skim(address to) external lock {
    address _token0 = token0;
    address _token1 = token1;
    _safeTransfer(
        _token0,
        to,
        IERC20(_token0).balanceOf(address(this)).sub(reserve0 + accruedLaunchpadFee0)
    );
    _safeTransfer(
        _token1,
        to,
        IERC20(_token1).balanceOf(address(this)).sub(reserve1 + accruedLaunchpadFee1)
    );
}
```

**How this can desync reserves and enable theft**:

- `_update` computes `reserve0/1` by subtracting `totalLaunchpadFee*` from the current balances, under the assumption that these fees are *about to be* moved out to the Distributor.
- `_distributeLaunchpadFees` merely sets allowances and calls `addRewards`; if that external call *does not* pull tokens (e.g., due to a future implementation bug, a change in semantics, or a partial pull), the actual `balanceOf(this)` for one or both tokens will remain **higher** than `reserve + accruedLaunchpadFee`.
- At this point, any user can call `skim(to)` and receive `balance - (reserve + accruedLaunchpadFee)` for each token. That difference will include some or all of the "stuck" launchpad fee tokens which were never actually transferred out to the rewards contract.

This creates a **ReserveOrPriceDesync** pattern:

- Internal accounting (`reserve0/1` plus `accruedLaunchpadFee`) assumes some tokens have left the contract, while the actual ERC20 balances still include them.
- The protocol exposes a public function (`skim`) that transfers any "extra" tokens to an arbitrary address, allowing a permissionless caller to extract those mis-accounted fees.

While the project comments state that "tokens are trusted", the problematic assumption is about the *behavior* of `launchpadFeeDistributor.addRewards`, not token correctness. Any future upgrade or deviation in the Distributor that fails to fully pull the approved amount will make these untransferred launchpad fees stealable through `skim`.

This is contract-composition-specific and does not rely on fee-on-transfer tokens or malicious admins in the pair itself; it requires only that the Distributor behaves in a way that is inconsistent with the implicit invariant in `GTELaunchpadV2Pair` ("approved == transferred").
 ### Static Signals
reserves computed as balance - assumed fees, external addRewards called without verifying transfer, skim transfers balance - (reserve + accruedFees) to arbitrary address, no invariant check that distributor pulled approved amount
 ### Assets at Risk
launchpad fee rewards, AMM pool token balances
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: GTELaunchpadV2Pair._distributeLaunchpadFees

 ### Title
Launchpad fee distribution callback can brick AMM pair if Distributor reverts
 ### Description/Code Snippet
In `GTELaunchpadV2Pair`, launchpad swap fees are periodically forwarded to an external `Distributor` contract via `_distributeLaunchpadFees`, which is called inside `_update`:

```solidity
function _update(
    uint256 balance0,
    uint256 balance1,
    uint112 _reserve0,
    uint112 _reserve1,
    uint112 newLaunchpadFee0,
    uint112 newLaunchpadFee1
) private {
    ...
    if (timeElapsed > 0 && _reserve0 != 0 && _reserve1 != 0) {
        ...
        if (launchpadFeeDistributor > address(0)) {
            if (totalLaunchpadFee0 | totalLaunchpadFee1 > 0) {
                delete accruedLaunchpadFee0;
                delete accruedLaunchpadFee1;
                _distributeLaunchpadFees(totalLaunchpadFee0, totalLaunchpadFee1);
            }
        }
    } else if (launchpadFeeDistributor > address(0) && newLaunchpadFee0 | newLaunchpadFee1 > 0) {
        accruedLaunchpadFee0 = totalLaunchpadFee0;
        accruedLaunchpadFee1 = totalLaunchpadFee1;
        emit LaunchpadFeesAccrued(newLaunchpadFee0, newLaunchpadFee1);
    }
    ...
}

function _distributeLaunchpadFees(uint112 fee0, uint112 fee1) internal {
    if ((fee0 | fee1) > 0) {
        address _token0 = token0;
        address _token1 = token1;
        address distributor = launchpadFeeDistributor;

        if (fee0 > 0) _safeApprove(_token0, distributor, uint256(fee0));
        if (fee1 > 0) _safeApprove(_token1, distributor, uint256(fee1));

        IDistributor(distributor).addRewards(_token0, _token1, uint128(fee0), uint128(fee1));

        emit LaunchpadFeesCollected(fee0, fee1);
    }
}
```

`_update` is called from core functions `swap`, `mint`, `burn`, `sync`, and `endRewardsAccrual`. When `timeElapsed > 0` and there are collected fees, `_distributeLaunchpadFees` is invoked *unconditionally* inside the state update flow, with no `try/catch`, no gas limiting, and no fallback path.

This means that if the external `Distributor.addRewards` call ever reverts (due to a logic bug, misconfiguration, or state limitation in the Distributor), then any call path that reaches `_distributeLaunchpadFees` will revert as well. For pairs with a non-zero `launchpadFeeDistributor`, this can:

- Make `swap` revert in all future blocks where fee distribution is triggered, effectively freezing trading on that pair.
- Potentially block `mint` / `burn` or `sync` calls in those blocks, preventing LPs from adjusting liquidity or re-synchronizing reserves.

Because any user executing a swap that crosses a block boundary (i.e., `timeElapsed > 0`) can trigger the failing callback, a single faulty Distributor deployment or configuration can cause a persistent DoS on the AMM pair. There is no mechanism to bypass the callback on failure, and the pair itself has no way to recover (e.g., by disabling `launchpadFeeDistributor`) without an upgrade or out-of-band intervention.

This matches the **GriefableCallbacks** pattern: a core protocol operation (`swap`) depends on an unguarded call to an external module (`Distributor`) whose failure can brick the pair for all users.

 ### Static Signals
external call to Distributor.addRewards() inside core AMM update, no try/catch around callback, callback failure causes swap/mint/burn/sync to revert, launchpadFeeDistributor is an arbitrary external contract address
 ### Assets at Risk
liquidityPool, tradingContinuity, launchpadRewardFlow
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: GTELaunchpadV2Pair.skim

 ### Title
Launchpad fee tokens can plausibly be stolen via skim() depending on Distributor implementation
 ### Description/Code Snippet
GTELaunchpadV2Pair introduces an extra accounting layer for launchpad fees on top of the standard Uniswap V2 reserve model. The intent is that a portion of swap fees are periodically sent to `launchpadFeeDistributor` while reserves only track "user" liquidity.

Key logic:
- In `swap()`, after computing amounts in/out, launchpad fees are computed and passed into `_update`:

  - `swap()`:
    - `(uint112 launchpadFee0, uint112 launchpadFee1) = launchpadFeeDistributor > address(0) && rewardsPoolActive > 0 ? _getLaunchpadFees(amount0In, amount1In) : (uint112(0), uint112(0));`
    - `_update(balance0, balance1, _reserve0, _reserve1, launchpadFee0, launchpadFee1);`

- `_update()`:

  - Computes new total fees:
    - `uint112 totalLaunchpadFee0 = accruedLaunchpadFee0 + newLaunchpadFee0;`
    - `uint112 totalLaunchpadFee1 = accruedLaunchpadFee1 + newLaunchpadFee1;`

  - If a new block has elapsed and `launchpadFeeDistributor` is set, it immediately triggers distribution:
    - `if (launchpadFeeDistributor > address(0)) {`
      - `if (totalLaunchpadFee0 | totalLaunchpadFee1 > 0) {`
        - `delete accruedLaunchpadFee0;`
        - `delete accruedLaunchpadFee1;`
        - `_distributeLaunchpadFees(totalLaunchpadFee0, totalLaunchpadFee1);`

  - Then reserves are updated by subtracting total fees from the balances **that were observed before the distribution call**:
    - `reserve0 = _reserve0 = uint112(balance0) - totalLaunchpadFee0;`
    - `reserve1 = _reserve1 = uint112(balance1) - totalLaunchpadFee1;`

- `_distributeLaunchpadFees()` only sets allowances and calls `IDistributor.addRewards`:

  - `if ((fee0 | fee1) > 0) {`
    - `_safeApprove(_token0, distributor, uint256(fee0));`
    - `_safeApprove(_token1, distributor, uint256(fee1));`
    - `IDistributor(distributor).addRewards(_token0, _token1, uint128(fee0), uint128(fee1));`

There is no explicit transfer of `fee0`/`fee1` tokens in the pair contract; it relies on `Distributor.addRewards` to pull tokens via `transferFrom`. This creates a subtle but critical dependency: **if** `Distributor.addRewards` does not immediately transfer out the tokens but instead just records the amounts (or fails to pull them for any reason), then after `_update`:

- `reserve0` and `reserve1` will have been decreased by `totalLaunchpadFee0` / `totalLaunchpadFee1` (fees are excluded from reserves),
- `accruedLaunchpadFee0` and `accruedLaunchpadFee1` are zeroed,
- the actual ERC20 balances may still include those `totalLaunchpadFee*` tokens because no transfer has occurred.

At that point, anyone can call `skim()`:

- `function skim(address to) external lock {`
  - `_safeTransfer(_token0, to, IERC20(_token0).balanceOf(address(this)).sub(reserve0 + accruedLaunchpadFee0));`
  - `_safeTransfer(_token1, to, IERC20(_token1).balanceOf(address(this)).sub(reserve1 + accruedLaunchpadFee1));`

Given `accruedLaunchpadFee* == 0` and `reserve*` already excludes `totalLaunchpadFee*`, `balance - (reserve + accrued)` equals exactly the unaccounted extra tokens (including any undisbursed fees). Under the above condition (fees not yet transferred out), this makes the launchpad fee tokens that should belong to the rewards system fully skim-able by any address.

This is a realistic pattern because the pair contract does not enforce that `addRewards` must pull tokens; it only sets approvals and calls a function whose implementation is external to this contract. Any change/bug in `Distributor` that defers or omits the pull will silently turn launchpad fees into "donations" that are freely withdrawable via `skim()`.

This manifests as an **accounting invariant violation**:
- Internal state assumes launchpad fees are no longer part of reserves and are conceptually owed to the distributor.
- But unless the external call actually moves the tokens, on-chain balances remain higher than reserves, and the canonical `skim()` function treats that imbalance as excess tokens that can be withdrawn by anyone.

Even if the current Distributor implementation does pull tokens immediately, this coupling between fee accounting and an external contract’s transfer semantics is brittle; any future change to `addRewards` (e.g., moving transfer to a separate function, or a bug that prevents pulling for a subset of calls) instantly creates an exploitable gap where launchpad fees can be stolen via `skim()`.
 ### Static Signals
skim() is permissionless and transfers contract balance minus (reserve + accrued), launchpad fees excluded from reserves in _update before verifying tokens actually moved, external fee distribution relies on addRewards() semantics and transferFrom outside this contract, accruedLaunchpadFee0/1 deleted before potential failure or omission of the actual token transfer
 ### Assets at Risk
launchpad fee rewards, protocol fee revenue, excess token balances in the pair
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: GTELaunchpadV2PairFactory.createPair

 ### Title
Permissionless factory.createPair lets anyone block launchpad-wired pair creation
 ### Description/Code Snippet
GTELaunchpadV2PairFactory.createPair is fully permissionless but encodes special launchpad-specific wiring (launchpadLp and launchpadFeeDistributor) only when called by the trusted launchpad address. The mapping key used to store and guard uniqueness of pairs, however, only depends on (token0, token1), not on the launchpad-specific parameters.

Relevant code:

```solidity
contract GTELaunchpadV2PairFactory is IUniswapV2Factory {
    address immutable launchpad;
    address immutable launchpadLp;
    address immutable launchpadFeeDistributor;

    mapping(address => mapping(address => address)) public getPair;

    function createPair(address tokenA, address tokenB) external returns (address pair) {
        if (tokenA == tokenB) revert("UniswapV2: IDENTICAL_ADDRESSES");
        (address token0, address token1) = tokenA < tokenB ? (tokenA, tokenB) : (tokenB, tokenA);
        if (token0 == address(0)) revert("UniswapV2: ZERO_ADDRESS");
        if (getPair[token0][token1] != address(0)) revert("UniswapV2: PAIR_EXISTS"); // single check is sufficient
        bytes memory bytecode = type(GTELaunchpadV2Pair).creationCode;

        (address _launchpadLp, address _launchpadFeeDistributor) =
            msg.sender == launchpad ? (launchpadLp, launchpadFeeDistributor) : (address(0), address(0));

        bytes32 salt = keccak256(abi.encodePacked(token0, token1, _launchpadLp, _launchpadFeeDistributor));
        assembly {
            pair := create2(0, add(bytecode, 32), mload(bytecode), salt)
        }
        IUniswapV2Pair(pair).initialize(token0, token1, _launchpadLp, _launchpadFeeDistributor);
        getPair[token0][token1] = pair;
        getPair[token1][token0] = pair; // populate mapping in the reverse direction
        allPairs.push(pair);
        emit PairCreated(token0, token1, pair, allPairs.length);
    }
}
```

Behavior:
- If `msg.sender == launchpad`, the pair is initialized with non-zero `launchpadLp` and `launchpadFeeDistributor`, allowing fee siphoning to the Distributor as intended.
- If `msg.sender != launchpad`, `_launchpadLp` and `_launchpadFeeDistributor` are set to `address(0)`, and the pair is initialized **without** launchpad fee wiring.
- The uniqueness check and `getPair` mapping key only consider `(token0, token1)`, not the launchpad parameters.

This opens a realistic attack/abuse pattern:
1. Before the official Launchpad graduation for a token, any arbitrary user calls `createPair(tokenA, tokenB)` on the factory.
2. Since `msg.sender != launchpad`, the new pair has `launchpadLp = 0` and `launchpadFeeDistributor = 0`, and thus will never accrue or distribute launchpad rewards.
3. `getPair[token0][token1]` is now permanently set to this "non-launchpad" pair.
4. When the real launchpad later tries to graduate and create its intended `GTELaunchpadV2Pair` with wired Distributor logic, `createPair` reverts with `PAIR_EXISTS` for the same `(token0, token1)` pair, so it can never create the canonical rewards-enabled pair.

Downstream components that trust `factory.getPair` for routing and rewards assumptions may then:
- Route user flow through a pair that never accrues launchpad LP rewards.
- Fail to ever set up the launchpad fee accrual path for that asset pair, effectively disabling part of the launchpad’s economic design.

Because this behavior arises purely from the permissionless nature of `createPair` combined with the special conditional wiring on `msg.sender == launchpad`, it matches an access-control / auth-bypass pattern on a **sensitive factory function** that configures where protocol fees and rewards flow.
 ### Static Signals
createPair() is external and permissionless, launchpadLp and launchpadFeeDistributor set only when msg.sender == launchpad, getPair[token0][token1] uniqueness check ignores launchpad-specific parameters, PAIR_EXISTS prevents later launchpad-owned pair creation
 ### Assets at Risk
launchpad LP fee share, rewards distributor flows, launchpad’s ability to deploy canonical pair for a token
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: GTELaunchpadV2Pair._update

 ### Title
Launchpad fee distribution callback can brick AMM pair (griefable external dependency)
 ### Description/Code Snippet
The `GTELaunchpadV2Pair` contract tightly couples core AMM operations with an external `IDistributor` callback via `_distributeLaunchpadFees`, without any failure isolation (no `try/catch`, no bypass on failure, and forwards essentially all gas).

Relevant code path:

```solidity
function _update(
    uint256 balance0,
    uint256 balance1,
    uint112 _reserve0,
    uint112 _reserve1,
    uint112 newLaunchpadFee0,
    uint112 newLaunchpadFee1
) private {
    ...
    uint112 totalLaunchpadFee0 = accruedLaunchpadFee0 + newLaunchpadFee0;
    uint112 totalLaunchpadFee1 = accruedLaunchpadFee1 + newLaunchpadFee1;

    uint32 blockTimestamp = uint32(block.timestamp % 2 ** 32);
    uint32 timeElapsed = blockTimestamp - blockTimestampLast; // overflow is desired
    if (timeElapsed > 0 && _reserve0 != 0 && _reserve1 != 0) {
        ...
        if (launchpadFeeDistributor > address(0)) {
            if (totalLaunchpadFee0 | totalLaunchpadFee1 > 0) {
                delete accruedLaunchpadFee0;
                delete accruedLaunchpadFee1;
                _distributeLaunchpadFees(totalLaunchpadFee0, totalLaunchpadFee1);
            }
        }
    } else if (launchpadFeeDistributor > address(0) && newLaunchpadFee0 | newLaunchpadFee1 > 0) {
        accruedLaunchpadFee0 = totalLaunchpadFee0;
        accruedLaunchpadFee1 = totalLaunchpadFee1;
        emit LaunchpadFeesAccrued(newLaunchpadFee0, newLaunchpadFee1);
    }
    ...
}

function _distributeLaunchpadFees(uint112 fee0, uint112 fee1) internal {
    if ((fee0 | fee1) > 0) {
        address _token0 = token0;
        address _token1 = token1;
        address distributor = launchpadFeeDistributor;

        // Since only pairs created by the launchpad can accrue fee tracking, the tokens are trusted
        if (fee0 > 0) _safeApprove(_token0, distributor, uint256(fee0));
        if (fee1 > 0) _safeApprove(_token1, distributor, uint256(fee1));

        IDistributor(distributor).addRewards(_token0, _token1, uint128(fee0), uint128(fee1));

        emit LaunchpadFeesCollected(fee0, fee1);
    }
}
```

`_update` is invoked from all core AMM entrypoints (`swap`, `mint`, `burn`, `sync`, and via `endRewardsAccrual`). When time has elapsed and there are accumulated launchpad fees, `_update` *must* call `_distributeLaunchpadFees`, which in turn calls the external `IDistributor.addRewards` contract and requires it to succeed.

There is no `try/catch`, no gas cap, and no alternative path if the distributor misbehaves. Any of the following in `IDistributor` (or downstream calls it makes) will brick the pair’s core functionality for as long as `launchpadFeeDistributor` remains non-zero:

- Reverts unconditionally or under some state condition.
- Reverts only for certain token amounts (e.g. after fees cross a threshold).
- Consumes excessive gas (e.g. iterating over an unbounded list), causing swaps to run out of gas.

Because `_update` is called in the critical path of `swap`, `mint`, and `burn`, any revert inside `IDistributor.addRewards` causes these user-facing operations to revert as well. Users cannot opt-out of the callback, meaning a buggy or griefable distributor implementation (or a bad upgrade, or misconfiguration pointing to a wrong address) can:

- **DoS the AMM pair completely**: swaps, liquidity adds/removals, and `sync` will revert whenever time has elapsed and `totalLaunchpadFee0|totalLaunchpadFee1 > 0`.
- **Lock user liquidity**: LPs may be unable to burn LP tokens to withdraw assets if `burn()` reverts due to fee distribution.

This matches the `GriefableCallbacks` pattern: an external, untrusted (or at least fallible) callback is mandatory for core flow to succeed, and there is no failure isolation.

A more robust design would:
- Wrap the distributor call in `try/catch` and, on failure, keep `accruedLaunchpadFee*` in storage without distributing; or
- Allow an emergency switch to bypass distribution while preserving AMM usability; or
- At minimum, ensure that a misconfigured or bricked distributor cannot permanently prevent swaps and liquidity operations.
 ### Static Signals
external call to IDistributor.addRewards without try/catch, callback success required for swaps/mints/burns to complete, no gas limit on external call, external call inside core pricing/state update logic
 ### Assets at Risk
AMM liquidity, user LP positions, launchpad rewards distribution, trading availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: GTELaunchpadV2PairFactory.createPair

 ### Title
Deterministic GTELaunchpadV2Pair deployment can be front‑run and permanently block launchpad-enabled pair
 ### Description/Code Snippet
GTELaunchpadV2PairFactory.createPair() is permissionless and uses CREATE2 with a salt that depends on (token0, token1, _launchpadLp, _launchpadFeeDistributor):

- If msg.sender == launchpad, it passes (launchpadLp, launchpadFeeDistributor) into the salt and initializes the pair with these non-zero addresses.
- For everyone else, it passes (address(0), address(0)) into the salt and initializes the pair with zero launchpadLp/launchpadFeeDistributor.

```solidity
function createPair(address tokenA, address tokenB) external returns (address pair) {
    ...
    (address _launchpadLp, address _launchpadFeeDistributor) =
        msg.sender == launchpad ? (launchpadLp, launchpadFeeDistributor) : (address(0), address(0));

    bytes32 salt = keccak256(abi.encodePacked(token0, token1, _launchpadLp, _launchpadFeeDistributor));
    assembly {
        pair := create2(0, add(bytecode, 32), mload(bytecode), salt)
    }
    IUniswapV2Pair(pair).initialize(token0, token1, _launchpadLp, _launchpadFeeDistributor);
    getPair[token0][token1] = pair;
    getPair[token1][token0] = pair;
    allPairs.push(pair);
}
```

The mapping getPair[token0][token1] tracks only by (token0, token1), **ignoring** the launchpad-specific parameters used in the CREATE2 salt. That means:

1. An attacker can front‑run the Launchpad by calling createPair(token0, token1) **before** the Launchpad does, with msg.sender != launchpad.
   - This deploys a pair via CREATE2 with salt keccak(token0, token1, 0, 0), and initializes it with launchpadLp = 0 and launchpadFeeDistributor = 0.
   - getPair[token0][token1] is set to this attacker-created pair.
2. Later, when the Launchpad tries to graduate the bonding curve and call factory.createPair(token0, token1) as launchpad, it hits:

```solidity
if (getPair[token0][token1] != address(0)) revert("UniswapV2: PAIR_EXISTS");
```

and reverts, because getPair already points to the zero-launchpad pair. The Launchpad can no longer create its own rewards-enabled pair for that token pair.

This matches the "deterministic deployment DoS via front-run pre-creation" pattern: the factory uses deterministic CREATE2 but **reverts on collisions** instead of returning the existing address, and the key used for collision checks (getPair[token0][token1]) does not fully match the salt key space (token0, token1, launchpadLp, launchpadFeeDistributor). A single permissionless front‑run transaction can permanently block the protocol from deploying the intended launchpad-enabled pair, disabling fee accrual to the Distributor and potentially breaking the graduation flow.

While user funds in the pair are not directly stolen, the launchpad invariants and rewards design are violated, and the launch process for a given token pair can be griefed/DoSed by any external account.
 ### Static Signals
permissionless factory.createPair, CREATE2 deterministic deployment, salt includes launchpad-specific params not reflected in getPair key, factory reverts on PAIR_EXISTS instead of returning existing address
 ### Assets at Risk
launchpad rewards, AMM launch/graduation functionality
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PermitFrontRun

 ### Relevant Function/Location: UniswapV2ERC20.permit

 ### Title
LP token permit can be front-run due to weak signature validation (no low-s / v checks)
 ### Description/Code Snippet
The same `permit` implementation for the LP token also matches the `PermitFrontRun` pattern: it accepts signatures without enforcing low‑s or canonical v values. While nonces are incremented as part of the signed data (preventing trivial exact replay), accepting multiple encodings of logically equivalent signatures makes it easier for an attacker/relayer to front‑run or race the intended transaction in ecosystems where wallets or middleware assume strict EIP‑2/EIP‑712 behavior.

Code (same as above):

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
            keccak256(abi.encode(PERMIT_TYPEHASH, owner, spender, value, nonces[owner]++, deadline))
        )
    );
    address recoveredAddress = ecrecover(digest, v, r, s);
    require(recoveredAddress != address(0) && recoveredAddress == owner, "UniswapV2: INVALID_SIGNATURE");
    _approve(owner, spender, value);
}
```

Static signals:
- No low‑s check on `s`.
- `v` is not constrained to 27/28.
- Direct `ecrecover` usage.

Scenario sketch:
- A user signs a permit off-chain using a wallet that enforces canonical (low‑s) signatures.
- An attacker, having access to the raw signature material (e.g., via a compromised relayer or shared channel), derives a malleated version that still recovers to the same address but uses a non‑canonical `s` or unconventional `v` value.
- The attacker front‑runs the victim’s intended permit usage in the same block (for example, combining `permit` + some state‑changing call via a router or dApp) with the malleated signature; the contract accepts it.
- Depending on the integration, this can cause the victim’s intended follow‑up transaction (which assumes a particular state around allowances or nonces) to fail or behave unexpectedly, while the attacker already exploited the approval.

Although the nonce-in-digest design mitigates straightforward replay, the lack of strict signature form remains a recognized security hardening requirement for permit-style flows, especially as they are often combined with other operations in a single meta-tx or batched call.
 ### Static Signals
no s-value malleability guard, v not enforced to 27/28, direct ecrecover, permit and allowance used for follow-up operations
 ### Assets at Risk
LP token holders, user allowances / approvals
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: PermitMisuse

 ### Relevant Function/Location: UniswapV2ERC20.permit

 ### Title
Permit signature malleability due to missing v/s validation in LP token
 ### Description/Code Snippet
The UniswapV2-style LP token used by GTELaunchpadV2Pair implements EIP-2612-style permit but does not enforce standard anti-malleability checks on the signature parameters `v` and `s`.

In `UniswapV2ERC20.permit`:

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
            keccak256(abi.encode(PERMIT_TYPEHASH, owner, spender, value, nonces[owner]++, deadline))
        )
    );
    address recoveredAddress = ecrecover(digest, v, r, s);
    require(recoveredAddress != address(0) && recoveredAddress == owner, "UniswapV2: INVALID_SIGNATURE");
    _approve(owner, spender, value);
}
```

Issues:
- There is **no check that `s <= secp256k1n/2`** (the low-s rule).
- There is **no check that `v` is 27 or 28** (or normalized from 0/1).

This violates common EIP-2612 / EIP-712 recommendations and allows **signature malleability**: for any valid `(v, r, s)` there exists another valid signature `(v', r, n - s)` for the same digest that an attacker can compute without knowing the private key. While the implementation does correctly consume and increment a nonce, meaning classic replay of the *same* permit in this contract is prevented, the malleability still:
- Breaks assumptions for off-chain signers and integrators that expect unique, non-malleable signatures.
- May enable subtle logic bugs in integrators relying on signature uniqueness or cross-contract replay protection (if they re-use the same digest across domains) since the contract accepts both low-s and high-s variants.

Given this LP token is intended to be integrated widely (AMMs, routers, aggregators) and exposes a public `permit`, the lack of standard signature normalization is a realistic security concern and matches the Code4rena `PermitMisuse` pattern.
 ### Static Signals
ecrecover used without enforcing s <= secp256k1n/2, ecrecover used without validating v in {27,28} or normalized, EIP-2612 style permit exposed on widely used LP token
 ### Assets at Risk
LP token allowances, downstream integrations relying on non-malleable permits
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: BeaconOrFactoryAuthorityDrift

 ### Relevant Function/Location: GTELaunchpadV2PairFactory.createPair

 ### Title
Permissionless factory allows front‑running of launchpad pair creation, breaking launchpad fee wiring
 ### Description/Code Snippet
The `GTELaunchpadV2PairFactory` is a generic Uniswap V2–style factory with a twist: only when `createPair` is called by the `launchpad` address does the created pair get wired with non‑zero `launchpadLp` and `launchpadFeeDistributor`, which are essential for the launchpad rewards mechanics.

Relevant code:

```solidity
contract GTELaunchpadV2PairFactory is IUniswapV2Factory {
    address immutable launchpad;
    address immutable launchpadLp;
    address immutable launchpadFeeDistributor;

    mapping(address => mapping(address => address)) public getPair;

    function createPair(address tokenA, address tokenB) external returns (address pair) {
        if (tokenA == tokenB) revert("UniswapV2: IDENTICAL_ADDRESSES");
        (address token0, address token1) = tokenA < tokenB ? (tokenA, tokenB) : (tokenB, tokenA);
        if (token0 == address(0)) revert("UniswapV2: ZERO_ADDRESS");
        if (getPair[token0][token1] != address(0)) revert("UniswapV2: PAIR_EXISTS");
        bytes memory bytecode = type(GTELaunchpadV2Pair).creationCode;

        (address _launchpadLp, address _launchpadFeeDistributor) =
            msg.sender == launchpad ? (launchpadLp, launchpadFeeDistributor) : (address(0), address(0));

        bytes32 salt = keccak256(abi.encodePacked(token0, token1, _launchpadLp, _launchpadFeeDistributor));
        assembly {
            pair := create2(0, add(bytecode, 32), mload(bytecode), salt)
        }
        IUniswapV2Pair(pair).initialize(token0, token1, _launchpadLp, _launchpadFeeDistributor);
        getPair[token0][token1] = pair;
        getPair[token1][token0] = pair;
        emit PairCreated(token0, token1, pair, allPairs.length);
    }
}
```

The launchpad‑aware pair logic in `GTELaunchpadV2Pair` relies on the `launchpadLp` and `launchpadFeeDistributor` having been set at initialization time by the factory:

```solidity
function initialize(address _token0, address _token1, address _launchpadLp, address _launchpadFeeDistributor)
    external
{
    if (msg.sender != factory) revert("UniswapV2: FORBIDDEN");
    token0 = _token0;
    token1 = _token1;
    launchpadLp = _launchpadLp;
    launchpadFeeDistributor = _launchpadFeeDistributor;
    rewardsPoolActive = 1;
}

function _distributeLaunchpadFees(uint112 fee0, uint112 fee1) internal {
    if ((fee0 | fee1) > 0) {
        address distributor = launchpadFeeDistributor;
        IDistributor(distributor).addRewards(_token0, _token1, uint128(fee0), uint128(fee1));
    }
}
```

**Pattern manifestation**

- `createPair` is **fully permissionless**; there is no restriction that only the `launchpad` contract can create a pair for a given `(launchToken, quoteToken)` pair.
- However, the mapping `getPair[token0][token1]` enforces a **single canonical pair** per ordered token pair:

```solidity
if (getPair[token0][token1] != address(0)) revert("UniswapV2: PAIR_EXISTS");
```

- When a non‑launchpad caller first creates the pair, `msg.sender != launchpad`, so `_launchpadLp` and `_launchpadFeeDistributor` are set to `address(0)` and encoded into the CREATE2 salt. The deployed `GTELaunchpadV2Pair` is initialized with both launchpad‑specific addresses as zero.
- Later, when the real `Launchpad` contract tries to "graduate" a bonding‑curve token and call `factory.createPair(launchToken, quoteToken)` expecting its rewards wiring, the call will revert with `"UniswapV2: PAIR_EXISTS"` because `getPair[launchToken][quoteToken]` was already populated by the attacker’s earlier, generic pair.
- Due to the mapping check, the factory **cannot** create a second, correctly wired launchpad pair for the same token pair, even though the CREATE2 salt would differ (because it would now include non‑zero `_launchpadLp`/`_launchpadFeeDistributor`). The registry enforces uniqueness only on `(token0, token1)`.

This produces an *authority drift* from the perspective of the launchpad: a permissionless user can permanently occupy the canonical pair slot for a launchpad token/quote pair and force the system to use a pair that:

- has `launchpadLp == address(0)` and `launchpadFeeDistributor == address(0)`,
- will never accrue or route launchpad fee share to the `Distributor`,
- may break invariants assumed by the off‑chain and on‑chain launchpad logic (docs say: "as soon as the bonding-curve finishes, liquidity is parked in a customised `GTELaunchpadV2Pair` whose fees feed the launchpad’s `Distributor`" — this invariant can be violated).

**Realistic exploit scenario**

1. A new token is launched via `Launchpad`; its ERC‑20 address `launchToken` and `quoteToken` (e.g. USDC) are public.
2. Before the bonding curve completes and the Launchpad calls `createPair(launchToken, quoteToken)`, an attacker sends a transaction:
   ```solidity
   factory.createPair(launchToken, quoteToken); // from EOAs or any non-launchpad contract
   ```
   This succeeds and stores `getPair[launchToken][quoteToken] = pairAttack`, with zeroed launchpad fee addresses.
3. Later, when Launchpad graduation logic runs and attempts to create the official launchpad pair, that call reverts with `PAIR_EXISTS`.
4. Launchpad code (and/or frontends) must then either:
   - fail the graduation flow (potentially locking user funds or blocking launch), or
   - use the attacker‑created pair as "the" AMM for the token, losing the launchpad fee share and rewards distribution functionality.

This is not merely a governance risk: a **permissionless actor** can unilaterally and irrevocably prevent the launchpad from ever deploying its special-fee pair for that token pair via this factory, violating documented assumptions about automatic pool creation and fee routing.

This matches the **BeaconOrFactoryAuthorityDrift** pattern because:

- The factory is the authority on what the canonical pair is for each token pair.
- That authority over the `(token0, token1) → pair` binding and the special launchpad wiring can be **preempted** by any user before the intended privileged contract (Launchpad) acts.
- Once preempted, the factory’s registry prevents the Launchpad from later asserting its intended implementation/fee wiring for that asset pair.

Mitigations could include:

- Restricting `createPair` calls for *launchpad‑controlled tokens* (e.g., while still allowing unrestricted pairs for arbitrary non‑launchpad tokens), or
- Allowing multiple pairs per token pair keyed by `(token0, token1, launchpadFlag)` in the registry, or
- Deferring publication in `getPair` until after a launchpad‑specific check is satisfied for launchpad assets.

 ### Static Signals
createPair is external and fully permissionless, factory encodes launchpad wiring based on msg.sender == launchpad, getPair[token0][token1] is single-valued and blocks later pairs, launchpadLp and launchpadFeeDistributor set to zero for non-launchpad callers
 ### Assets at Risk
launchpad fee revenue, Distributor rewards pool, launchpad graduation flow / ability to deploy intended AMM pair
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

