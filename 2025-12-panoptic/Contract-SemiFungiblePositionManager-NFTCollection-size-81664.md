
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.24;
// Interfaces
import {IERC20Partial} from "@tokens/interfaces/IERC20Partial.sol";
import {IPoolManager} from "v4-core/interfaces/IPoolManager.sol";
// Inherited implementations
import {ERC1155} from "@tokens/ERC1155Minimal.sol";
import {Multicall} from "@base/Multicall.sol";
import {TransientReentrancyGuard} from "solmate/src/utils/TransientReentrancyGuard.sol";
// Libraries
import {Constants} from "@libraries/Constants.sol";
import {EfficientHash} from "@libraries/EfficientHash.sol";
import {Errors} from "@libraries/Errors.sol";
import {Math} from "@libraries/Math.sol";
import {PanopticMath} from "@libraries/PanopticMath.sol";
import {V4StateReader} from "@libraries/V4StateReader.sol";
// Custom types
import {LeftRightUnsigned, LeftRightSigned, LeftRightLibrary} from "@types/LeftRight.sol";
import {LiquidityChunk} from "@types/LiquidityChunk.sol";
import {PoolData, PoolDataLibrary} from "@types/PoolData.sol";
import {TokenId} from "@types/TokenId.sol";
import {BalanceDelta} from "v4-core/types/BalanceDelta.sol";
import {Currency} from "v4-core/types/Currency.sol";
import {PoolId} from "v4-core/types/PoolId.sol";
import {PoolKey} from "v4-core/types/PoolKey.sol";

//                                                                        ..........
//                       ,.                                   .,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,.                                    ,,
//                    ,,,,,,,                           ,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,                            ,,,,,,
//                  .,,,,,,,,,,.                   ,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,                     ,,,,,,,,,,,
//                .,,,,,,,,,,,,,,,             ,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,.              ,,,,,,,,,,,,,,,
//               ,,,,,,,,,,,,,,.            ,,,,,,,,,,,,,,,,,,,,,,,,,,,                ,,,,,,,,,,,,,,,,,,,,,,,,,,.             ,,,,,,,,,,,,,,,
//             ,,,,,,,,,,,,,,,           ,,,,,,,,,,,,,,,,,,,,,,                                ,,,,,,,,,,,,,,,,,,,,,,            ,,,,,,,,,,,,,,,
//            ,,,,,,,,,,,,,.           ,,,,,,,,,,,,,,,,,,                                           .,,,,,,,,,,,,,,,,,,            ,,,,,,,,,,,,,,
//          ,,,,,,,,,,,,,,          ,,,,,,,,,,,,,,,,,.                                                  ,,,,,,,,,,,,,,,,,           .,,,,,,,,,,,,,
//         ,,,,,,,,,,,,,.         .,,,,,,,,,,,,,,,.                                                        ,,,,,,,,,,,,,,,,           ,,,,,,,,,,,,,.
//        ,,,,,,,,,,,,,          ,,,,,,,,,,,,,,,                                                              ,,,,,,,,,,,,,,,           ,,,,,,,,,,,,,
//       ,,,,,,,,,,,,,         ,,,,,,,,,,,,,,.                                                                  ,,,,,,,,,,,,,,           ,,,,,,,,,,,,,
//      ,,,,,,,,,,,,,         ,,,,,,,,,,,,,,                                                                      ,,,,,,,,,,,,,,          ,,,,,,,,,,,,,
//     ,,,,,,,,,,,,,         ,,,,,,,,,,,,,                                                                         ,,,,,,,,,,,,,,          ,,,,,,,,,,,,.
//    .,,,,,,,,,,,,        .,,,,,,,,,,,,,                                                                            ,,,,,,,,,,,,,          ,,,,,,,,,,,,
//    ,,,,,,,,,,,,         ,,,,,,,,,,,,                                                                               ,,,,,,,,,,,,,         .,,,,,,,,,,,,
//   ,,,,,,,,,,,,         ,,,,,,,,,,,,                                                                                 ,,,,,,,,,,,,.         ,,,,,,,,,,,,
//   ,,,,,,,,,,,,        ,,,,,,,,,,,,.                █████████  ███████████ ███████████  ██████   ██████               ,,,,,,,,,,,,          ,,,,,,,,,,,,
//  .,,,,,,,,,,,,        ,,,,,,,,,,,,                ███░░░░░███░░███░░░░░░█░░███░░░░░███░░██████ ██████                .,,,,,,,,,,,,         ,,,,,,,,,,,,
//  ,,,,,,,,,,,,        ,,,,,,,,,,,,                ░███    ░░░  ░███   █ ░  ░███    ░███ ░███░█████░███                 ,,,,,,,,,,,,         ,,,,,,,,,,,,.
//  ,,,,,,,,,,,,        ,,,,,,,,,,,,                ░░█████████  ░███████    ░██████████  ░███░░███ ░███                 .,,,,,,,,,,,          ,,,,,,,,,,,.
//  ,,,,,,,,,,,,        ,,,,,,,,,,,,                 ░░░░░░░░███ ░███░░░█    ░███░░░░░░   ░███ ░░░  ░███                  ,,,,,,,,,,,.         ,,,,,,,,,,,,
//  ,,,,,,,,,,,,        ,,,,,,,,,,,,                 ███    ░███ ░███  ░     ░███         ░███      ░███                  ,,,,,,,,,,,,         ,,,,,,,,,,,,
//  ,,,,,,,,,,,,        ,,,,,,,,,,,,                ░░█████████  █████       █████        █████     █████                 ,,,,,,,,,,,          ,,,,,,,,,,,,
//  ,,,,,,,,,,,,        ,,,,,,,,,,,,                 ░░░░░░░░░  ░░░░░       ░░░░░        ░░░░░     ░░░░░                 ,,,,,,,,,,,,          ,,,,,,,,,,,.
//  ,,,,,,,,,,,,        .,,,,,,,,,,,.                                                                                    ,,,,,,,,,,,,         ,,,,,,,,,,,,
//  .,,,,,,,,,,,,        ,,,,,,,,,,,,                                                                                   .,,,,,,,,,,,,         ,,,,,,,,,,,,
//   ,,,,,,,,,,,,        ,,,,,,,,,,,,,                                                                                  ,,,,,,,,,,,,          ,,,,,,,,,,,,
//   ,,,,,,,,,,,,.        ,,,,,,,,,,,,.                                                                                ,,,,,,,,,,,,.         ,,,,,,,,,,,,
//    ,,,,,,,,,,,,         ,,,,,,,,,,,,,                                                                              ,,,,,,,,,,,,,         .,,,,,,,,,,,,
//     ,,,,,,,,,,,,         ,,,,,,,,,,,,,                                                                            ,,,,,,,,,,,,,         .,,,,,,,,,,,,
//     .,,,,,,,,,,,,         ,,,,,,,,,,,,,                                                                         ,,,,,,,,,,,,,.          ,,,,,,,,,,,,
//      ,,,,,,,,,,,,,         ,,,,,,,,,,,,,,                                                                     .,,,,,,,,,,,,,.          ,,,,,,,,,,,,
//       ,,,,,,,,,,,,,         .,,,,,,,,,,,,,,                                                                 .,,,,,,,,,,,,,,          .,,,,,,,,,,,,
//        ,,,,,,,,,,,,,          ,,,,,,,,,,,,,,,                                                             ,,,,,,,,,,,,,,,.          ,,,,,,,,,,,,,.
//         ,,,,,,,,,,,,,,          ,,,,,,,,,,,,,,,,                                                       .,,,,,,,,,,,,,,,,           ,,,,,,,,,,,,,
//          .,,,,,,,,,,,,,           ,,,,,,,,,,,,,,,,,                                                 .,,,,,,,,,,,,,,,,,           ,,,,,,,,,,,,,,
//            ,,,,,,,,,,,,,,           ,,,,,,,,,,,,,,,,,,,.                                        ,,,,,,,,,,,,,,,,,,,.            ,,,,,,,,,,,,,,
//             ,,,,,,,,,,,,,,,            ,,,,,,,,,,,,,,,,,,,,,,                             .,,,,,,,,,,,,,,,,,,,,,,             ,,,,,,,,,,,,,,
//               ,,,,,,,,,,,,,,,            .,,,,,,,,,,,,,,,,,,,,,,,,,,,,,.        ,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,             .,,,,,,,,,,,,,,.
//                 ,,,,,,,,,,,,,,.              ,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,               ,,,,,,,,,,,,,,,
//                   ,,,,,,,,,,                     ,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,                     .,,,,,,,,,,
//                     ,,,,,.                            ,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,                             ,,,,,,
//                       ,                                     ..,,,,,,,,,,,,,,,,,,,,,,,,,,,,.

/// @author Axicon Labs Limited
/// @title Semi-Fungible Position Manager (ERC1155) - a gas-efficient Uniswap V4 position manager.
/// @notice Wraps Uniswap V4 positions with up to 4 legs behind an ERC1155 token.
contract SemiFungiblePositionManager is ERC1155, Multicall, TransientReentrancyGuard {
    /*//////////////////////////////////////////////////////////////
                                 EVENTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Emitted when a Uniswap V4 pool is initialized in the SFPM.
    /// @param idV4 The Uniswap V4 pool identifier (hash of `poolKey`)
    /// @param poolId The SFPM's pool identifier for the pool, including the 16-bit tick spacing and 48-bit pool pattern
    /// @param minEnforcedTick The initial minimum enforced tick for the pool
    /// @param maxEnforcedTick The initial maximum enforced tick for the pool
    event PoolInitialized(
        PoolId indexed idV4,
        uint64 poolId,
        int24 minEnforcedTick,
        int24 maxEnforcedTick
    );

    /// @notice Emitted when the enforced tick range is expanded for a given Uniswap `idV4`.
    /// @dev Will be emitted on any `expandEnforcedTickRange` call, even if the enforced ticks are not actually changed.
    /// @param idV4 The Uniswap V4 pool identifier (hash of `poolKey`)
    /// @param minEnforcedTick The new minimum enforced tick for the pool
    /// @param maxEnforcedTick The new maximum enforced tick for the pool
    event EnforcedTicksUpdated(PoolId indexed idV4, int24 minEnforcedTick, int24 maxEnforcedTick);

    /// @notice Emitted when a position is destroyed/burned.
    /// @param recipient The address of the user who burned the position
    /// @param tokenId The tokenId of the burned position
    /// @param positionSize The number of contracts burnt, expressed in terms of the asset
    event TokenizedPositionBurnt(
        address indexed recipient,
        TokenId indexed tokenId,
        uint128 positionSize
    );

    /// @notice Emitted when a position is created/minted.
    /// @param caller The address of the user who minted the position
    /// @param tokenId The tokenId of the minted position
    /// @param positionSize The number of contracts minted, expressed in terms of the asset
    event TokenizedPositionMinted(
        address indexed caller,
        TokenId indexed tokenId,
        uint128 positionSize
    );

    /*//////////////////////////////////////////////////////////////
                                 TYPES
    //////////////////////////////////////////////////////////////*/

    using Math for uint256;
    using Math for int256;

    /*//////////////////////////////////////////////////////////////
                            IMMUTABLES
    //////////////////////////////////////////////////////////////*/

    /// @notice Flag used to indicate a regular position mint.
    bool internal constant MINT = false;

    /// @notice Flag used to indicate that a position burn (with a burnTokenId) is occurring.
    bool internal constant BURN = true;

    /// @notice The canonical Uniswap V4 Pool Manager address.
    IPoolManager internal immutable POOL_MANAGER_V4;

    /// @notice The approximate minimum amount of tokens it should require to fill `maxLiquidityPerTick` at the minimum and maximum enforced ticks.
    uint256 internal immutable MIN_ENFORCED_TICKFILL_COST;

    /// @notice The approximate minimum amount of tokens it should require to fill `maxLiquidityPerTick` at the minimum and maximum enforced ticks for native-token pools.
    uint256 internal immutable NATIVE_ENFORCED_TICKFILL_COST;

    /// @notice The multiplier, in basis points, to apply to the token supply and set as the minimum enforced tick fill cost if greater than `MIN_ENFORCED_TICKFILL_COST`.
    uint256 internal immutable SUPPLY_MULTIPLIER_TICKFILL;

    /*//////////////////////////////////////////////////////////////
                            STORAGE
    //////////////////////////////////////////////////////////////*/

    /// @notice Retrieve the SFPM PoolIdData struct associated with a given Uniswap V4 poolId.
    mapping(PoolId idV4 => mapping(uint256 vegoid => PoolData poolData)) internal s_V4toSFPMIdData;

    /// @notice Retrieve the Uniswap V4 pool key corresponding to a given poolId.
    mapping(uint64 poolId => PoolKey key) internal s_poolIdToKey;

    /*
        We're tracking the amount of net and removed liquidity for the specific region:

             net amount
           received minted
          ▲ for isLong=0     amount
          │                 moved out      actual amount
          │  ┌────┐-T      due isLong=1   in the UniswapV4 Pool
          │  │    │          mints
          │  │    │                        ┌────┐-(T-R)
          │  │    │         ┌────┐-R       │    │
          │  │    │         │    │         │    │
          └──┴────┴─────────┴────┴─────────┴────┴──────►
             total=T       removed=R      net=(T-R)


     *       removed liquidity r          net liquidity N=(T-R)
     * |<------- 128 bits ------->|<------- 128 bits ------->|
     * |<---------------------- 256 bits ------------------->|
     */

    /// @notice Retrieve the current liquidity state in a chunk for a given user.
    /// @dev `removedAndNetLiquidity` is a LeftRight. The right slot represents the liquidity currently sold (added) in the AMM owned by the user and
    // the left slot represents the amount of liquidity currently bought (removed) that has been removed from the AMM - the user owes it to a seller.
    // The reason why it is called "removedLiquidity" is because long options are created by removed liquidity - ie. short selling LP positions.
    mapping(bytes32 positionKey => LeftRightUnsigned removedAndNetLiquidity)
        internal s_accountLiquidity;

    /*
        Any liquidity that has been deposited in the AMM using the SFPM will collect fees over
        time, we call this the gross premia. If that liquidity has been removed, we also need to
        keep track of the amount of fees that *would have been collected*, we call this the owed
        premia. The gross and owed premia are tracked per unit of liquidity by the
        s_accountPremiumGross and s_accountPremiumOwed accumulators.

        Here is how we can use the accumulators to compute the Gross, Net, and Owed fees collected
        by any position.

        Let`s say Charlie the smart contract deposited T into the AMM and later removed R from that
        same tick using a tokenId with a isLong=1 parameter. Because the netLiquidity is only (T-R),
        the AMM will collect fees equal to:

              net_feesCollectedX128 = feeGrowthX128 * (T - R)
                                    = feeGrowthX128 * N

        where N = netLiquidity = T-R. Had that liquidity never been removed, we want the gross
        premia to be given by:

              gross_feesCollectedX128 = feeGrowthX128 * T

        So we must keep track of fees for the removed liquidity R so that the long premia exactly
        compensates for the fees that would have been collected from the initial liquidity.

        In addition to tracking, we also want to track those fees plus a small spread. Specifically,
        we want:

              gross_feesCollectedX128 = net_feesCollectedX128 + owed_feesCollectedX128

       where

              owed_feesCollectedX128 = feeGrowthX128 * R * (1 + spread)                      (Eqn 1)

        A very opinionated definition for the spread is:

              spread = ν*(liquidity removed from that strike)/(netLiquidity remaining at that strike)
                     = ν*R/N

        For an arbitrary parameter 0 <= ν <= 1 (ν = 1/VEGOID). This way, the gross_feesCollectedX128 will be given by:

              gross_feesCollectedX128 = feeGrowthX128 * N + feeGrowthX128*R*(1 + ν*R/N)
                                      = feeGrowthX128 * T + feesGrowthX128*ν*R^2/N
                                      = feeGrowthX128 * T * (1 + ν*R^2/(N*T))                (Eqn 2)

        The s_accountPremiumOwed accumulator tracks the feeGrowthX128 * R * (1 + spread) term
        per unit of removed liquidity R every time the position touched:

              s_accountPremiumOwed += feeGrowthX128 * R * (1 + ν*R/N) / R
                                   += feeGrowthX128 * (T - R + ν*R)/N
                                   += feeGrowthX128 * T/N * (1 - R/T + ν*R/T)

        Note that the value of feeGrowthX128 can be extracted from the amount of fees collected by
        the smart contract since the amount of feesCollected is related to feeGrowthX128 according
        to:

             feesCollected = feesGrowthX128 * (T-R)

        So that we get:

             feesGrowthX128 = feesCollected/N

        And the accumulator is computed from the amount of collected fees according to:

             s_accountPremiumOwed += feesCollected * T/N^2 * (1 - R/T + ν*R/T)          (Eqn 3)

        So, the amount of owed premia for a position of size r minted at time t1 and burnt at
        time t2 is:

             owedPremia(t1, t2) = (s_accountPremiumOwed_t2-s_accountPremiumOwed_t1) * r
                                = ∆feesGrowthX128 * r * T/N * (1 - R/T + ν*R/T)
                                = ∆feesGrowthX128 * r * (T - R + ν*R)/N
                                = ∆feesGrowthX128 * r * (N + ν*R)/N
                                = ∆feesGrowthX128 * r * (1 + ν*R/N)             (same as Eqn 1)

        This way, the amount of premia owed for a position will match Eqn 1 exactly.

        Similarly, the amount of gross fees for the total liquidity is tracked in a similar manner
        by the s_accountPremiumGross accumulator.

        However, since we require that Eqn 2 holds up-- ie. the gross fees collected should be equal
        to the net fees collected plus the ower fees plus the small spread, the expression for the
        s_accountPremiumGross accumulator has to be given by (you`ll see why in a minute):

            s_accountPremiumGross += feesCollected * T/N^2 * (1 - R/T + ν*R^2/T^2)       (Eqn 4)

        This expression can be used to calculate the fees collected by a position of size t between times
        t1 and t2 according to:

            grossPremia(t1, t2) = ∆(s_accountPremiumGross) * t
                                = ∆feeGrowthX128 * t * T/N * (1 - R/T + ν*R^2/T^2)
                                = ∆feeGrowthX128 * t * (T - R + ν*R^2/T) / N
                                = ∆feeGrowthX128 * t * (N + ν*R^2/T) / N
                                = ∆feeGrowthX128 * t * (1  + ν*R^2/(N*T))   (same as Eqn 2)

        where the last expression matches Eqn 2 exactly.

        In summary, the s_accountPremium accumulators allow smart contracts that need to handle
        long+short liquidity to guarantee that liquidity deposited always receives the correct
        premia, whether that liquidity has been removed from the AMM or not.

        Note that the expression for the spread is extremely opinionated, and may not fit the
        specific risk management profile of every smart contract. And simply setting the ν parameter
        to zero would get rid of the "spread logic".
    */

    /// @notice Per-liquidity accumulator for the premium owed by buyers on a given chunk, tokenType and account.
    mapping(bytes32 positionKey => LeftRightUnsigned accountPremium) private s_accountPremiumOwed;

    /// @notice Per-liquidity accumulator for the premium earned by sellers on a given chunk, tokenType and account.
    mapping(bytes32 positionKey => LeftRightUnsigned accountPremium) private s_accountPremiumGross;

    /*//////////////////////////////////////////////////////////////
                             INITIALIZATION
    //////////////////////////////////////////////////////////////*/

    /// @notice Set the canonical Uniswap V4 pool manager address and tick fill parameters.
    /// @param poolManager The canonical Uniswap V4 pool manager address
    /// @param _minEnforcedTickFillCost The minimum amount of tokens it should require to fill `maxLiquidityPerTick` at the minimum and maximum enforced ticks
    /// @param _nativeEnforcedTickFillCost The minimum amount of tokens it should require to fill `maxLiquidityPerTick` at the minimum and maximum enforced ticks for native-token pools
    /// @param _supplyMultiplierTickFill The multiplier, in basis points, to apply to the token supply and set as the minimum enforced tick fill cost if greater than `MIN_ENFORCED_TICKFILL_COST`
    constructor(
        IPoolManager poolManager,
        uint256 _minEnforcedTickFillCost,
        uint256 _nativeEnforcedTickFillCost,
        uint256 _supplyMultiplierTickFill
    ) {
        POOL_MANAGER_V4 = poolManager;
        MIN_ENFORCED_TICKFILL_COST = _minEnforcedTickFillCost;
        NATIVE_ENFORCED_TICKFILL_COST = _nativeEnforcedTickFillCost;
        SUPPLY_MULTIPLIER_TICKFILL = _supplyMultiplierTickFill;
    }

    /// @notice Initialize a Uniswap V4 pool in the SFPM.
    /// @dev Revert if already initialized.
    /// @param key An identifying key for a Uniswap V4 pool
    function initializeAMMPool(
        PoolKey calldata key,
        uint8 vegoid
    ) external returns (uint64 poolId) {
        PoolId idV4 = key.toId();

        if (V4StateReader.getSqrtPriceX96(POOL_MANAGER_V4, idV4) == 0)
            revert Errors.PoolNotInitialized();

        // return if the pool has already been initialized in SFPM
        // pools can be initialized from the Panoptic Factory or by calling initializeAMMPool directly, so reverting
        // could prevent a PanopticPool from being deployed on a previously initialized but otherwise valid pool
        if (s_V4toSFPMIdData[idV4][vegoid].initialized())
            return s_V4toSFPMIdData[idV4][vegoid].poolId();

        // The base poolId is composed as follows:
        // [tickSpacing][pool pattern]
        // [16 bit tickSpacing][most significant 48 bits of the V4 poolId]
        poolId = _getPoolId(idV4, key.tickSpacing, vegoid);

        // There are 1,099,511,627,776 possible pool patterns.
        // A modern GPU can generate a collision in such a space relatively quickly,
        // so if a collision is detected increment the pool pattern until a unique poolId is found
        while (s_poolIdToKey[poolId].tickSpacing != 0) {
            poolId = PanopticMath.incrementPoolPattern(poolId);
        }

        uint128 maxLiquidityPerTick = Math.getMaxLiquidityPerTick(key.tickSpacing);

        int24 minEnforcedTick;
        int24 maxEnforcedTick;
        unchecked {
            minEnforcedTick = int24(
                -Math.getApproxTickWithMaxAmount(
                    Math.max(
                        MIN_ENFORCED_TICKFILL_COST,
                        (IERC20Partial(Currency.unwrap(key.currency1)).totalSupply() *
                            SUPPLY_MULTIPLIER_TICKFILL) / 10_000
                    ),
                    key.tickSpacing,
                    maxLiquidityPerTick
                )
            );
            maxEnforcedTick = int24(
                Math.getApproxTickWithMaxAmount(
                    Currency.unwrap(key.currency0) == address(0)
                        ? NATIVE_ENFORCED_TICKFILL_COST
                        : Math.max(
                            MIN_ENFORCED_TICKFILL_COST,
                            (IERC20Partial(Currency.unwrap(key.currency0)).totalSupply() *
                                SUPPLY_MULTIPLIER_TICKFILL) / 10_000
                        ),
                    key.tickSpacing,
                    maxLiquidityPerTick
                )
            );
        }

        s_V4toSFPMIdData[idV4][vegoid] = PoolDataLibrary.storePoolData(
            maxLiquidityPerTick,
            poolId,
            minEnforcedTick,
            maxEnforcedTick,
            true
        );

        s_poolIdToKey[poolId] = key;

        emit PoolInitialized(idV4, poolId, minEnforcedTick, maxEnforcedTick);
    }

    /// @notice Given a 256-bit Uniswap V4 pool ID (hash) and the corresponding `tickSpacing`, return its 64-bit ID as used in the `TokenId` of Panoptic.
    // Example:
    //      [16-bit tickSpacing][8-bits vegoid][last 40 bits of Uniswap V4 pool ID] = poolId
    //      e.g.:
    //         idV4        = 0x9c33e1937fe23c3ff82d7725f2bb5af696db1c89a9b8cae141cb0e986847638a
    //         vegoid      = 42 (0x2a)
    //         tickSpacing = 60 (0x3c)
    //      the returned id is then:
    //         poolPattern = 0x000000986847638a
    //         vegoid      = 0x00002a0000000000
    //         tickSpacing = 0x003c000000000000    +
    //         --------------------------------------------
    //         poolId      = 0x0032a986847638a
    /// @param idV4 The 256-bit Uniswap V4 pool ID
    /// @param tickSpacing The tick spacing of the Uniswap V4 pool identified by `idV4`
    /// @param vegoid The vegoid of the SFPM, must be 8 bits
    /// @return A fingerprint representing the Uniswap V4 pool
    function _getPoolId(
        PoolId idV4,
        int24 tickSpacing,
        uint256 vegoid
    ) internal pure returns (uint64) {
        unchecked {
            return
                uint40(uint256(PoolId.unwrap(idV4))) +
                (uint64(uint8(vegoid)) << 40) +
                (uint64(uint24(tickSpacing)) << 48);
        }
    }

    /// @notice Recomputes and decreases `minEnforcedTick` and/or increases `maxEnforcedTick` for a given V4 pool `key` if certain conditions are met.
    /// @dev This function will only have an effect if both conditions are met:
    /// - The token supply for one of the (non-native) tokens was greater than MIN_ENFORCED_TICKFILL_COST at the last `initializeAMMPool` or `expandEnforcedTickRangeForPool` call for `poolId`
    /// - The token supply for one of the tokens meeting the first condition has *decreased* significantly since the last call
    /// @dev This function *cannot* decrease the absolute value of either enforced tick, i.e., it can only widen the range of possible ticks.
    /// @dev The purpose of this function is to prevent pools created while a large amount of one of the tokens was flash-minted from being stuck in a narrow tick range.
    /// @param poolId The poolId on which to expand the enforced tick range
    function expandEnforcedTickRange(uint64 poolId) external {
        PoolKey memory key = s_poolIdToKey[poolId];
        PoolId idV4 = key.toId();

        uint256 vegoid = uint8(poolId >> 40);
        PoolData dataOld = s_V4toSFPMIdData[idV4][vegoid];

        if (!dataOld.initialized()) revert Errors.PoolNotInitialized();

        // tick spacing is stored in the highest 16 bits of the poolId
        int24 tickSpacing = int24(uint24(dataOld.poolId() >> 48));

        uint128 maxLiquidityPerTick = dataOld.maxLiquidityPerTick();

        int24 minEnforcedTick;
        int24 maxEnforcedTick;
        unchecked {
            minEnforcedTick = int24(
                Math.min(
                    dataOld.minEnforcedTick(),
                    -Math.getApproxTickWithMaxAmount(
                        Math.max(
                            MIN_ENFORCED_TICKFILL_COST,
                            (IERC20Partial(Currency.unwrap(key.currency1)).totalSupply() *
                                SUPPLY_MULTIPLIER_TICKFILL) / 10_000
                        ),
                        tickSpacing,
                        maxLiquidityPerTick
                    )
                )
            );
            maxEnforcedTick = int24(
                Math.max(
                    dataOld.maxEnforcedTick(),
                    Math.getApproxTickWithMaxAmount(
                        Currency.unwrap(key.currency0) == address(0)
                            ? NATIVE_ENFORCED_TICKFILL_COST
                            : Math.max(
                                MIN_ENFORCED_TICKFILL_COST,
                                (IERC20Partial(Currency.unwrap(key.currency0)).totalSupply() *
                                    SUPPLY_MULTIPLIER_TICKFILL) / 10_000
                            ),
                        tickSpacing,
                        maxLiquidityPerTick
                    )
                )
            );
        }

        s_V4toSFPMIdData[idV4][vegoid] = PoolDataLibrary.storePoolData(
            maxLiquidityPerTick,
            dataOld.poolId(),
            minEnforcedTick,
            maxEnforcedTick,
            dataOld.initialized()
        );

        emit EnforcedTicksUpdated(idV4, minEnforcedTick, maxEnforcedTick);
    }

    /*//////////////////////////////////////////////////////////////
                        UNISWAP V4 LOCK CALLBACK                           
    //////////////////////////////////////////////////////////////*/

    /// @notice Executes the corresponding operations and state updates required to mint `tokenId` of `positionSize` in `key`
    /// @param key The Uniswap V4 pool key in which to mint `tokenId`
    /// @param tickLimitLow The lower bound of an acceptable open interval for the ending price
    /// @param tickLimitHigh The upper bound of an acceptable open interval for the ending price
    /// @param positionSize The number of contracts minted, expressed in terms of the asset
    /// @param tokenId The tokenId of the minted position, which encodes information about up to 4 legs
    /// @param isBurn Flag indicating if the position is being burnt
    /// @return An array of LeftRight encoded words containing the amount of currency0 and currency1 collected as fees for each leg
    /// @return The net amount of currency0 and currency1 moved to/from the Uniswap V4 pool
    function _unlockAndCreatePositionInAMM(
        PoolKey memory key,
        int24 tickLimitLow,
        int24 tickLimitHigh,
        uint128 positionSize,
        TokenId tokenId,
        bool isBurn
    ) internal returns (LeftRightUnsigned[4] memory, LeftRightSigned, int24) {
        return
            abi.decode(
                POOL_MANAGER_V4.unlock(
                    abi.encode(
                        msg.sender,
                        key,
                        tickLimitLow,
                        tickLimitHigh,
                        positionSize,
                        tokenId,
                        isBurn
                    )
                ),
                (LeftRightUnsigned[4], LeftRightSigned, int24)
            );
    }

    /// @notice Uniswap V4 unlock callback implementation.
    /// @dev Parameters are `(address account, PoolKey key, int24 tickLimitLow, int24 tickLimitHigh, uint128 positionSize, TokenId tokenId, bool isBurn)`.
    /// @dev Executes the corresponding operations and state updates required to mint `tokenId` of `positionSize` in `key`
    /// @dev (shorts/longs are reversed before calling this function at burn)
    /// @param data The encoded data containing the input parameters
    /// @return `(LeftRightUnsigned[4] collectedByLeg, LeftRightSigned totalMoved)` An array of LeftRight encoded words containing the amount of currency0 and currency1 collected as fees for each leg and the net amount of currency0 and currency1 moved to/from the Uniswap V4 pool
    function unlockCallback(bytes calldata data) external returns (bytes memory) {
        if (msg.sender != address(POOL_MANAGER_V4)) revert Errors.UnauthorizedUniswapCallback();
        (
            address account,
            PoolKey memory key,
            int24 tickLimitLow,
            int24 tickLimitHigh,
            uint128 positionSize,
            TokenId tokenId,
            bool isBurn
        ) = abi.decode(data, (address, PoolKey, int24, int24, uint128, TokenId, bool));
        bool invertedLimits = tickLimitLow > tickLimitHigh;
        (
            LeftRightUnsigned[4] memory collectedByLeg,
            LeftRightSigned totalMoved
        ) = _createPositionInAMM(account, key, invertedLimits, positionSize, tokenId, isBurn);

        // Get the current tick of the Uniswap pool, check slippage
        int24 currentTick = getCurrentTick(abi.encode(key));
        if (invertedLimits) (tickLimitLow, tickLimitHigh) = (tickLimitHigh, tickLimitLow);

        if ((currentTick >= tickLimitHigh) || (currentTick <= tickLimitLow))
            revert Errors.PriceBoundFail(currentTick);
        return abi.encode(collectedByLeg, totalMoved, currentTick);
    }

    /*//////////////////////////////////////////////////////////////
                       PUBLIC MINT/BURN FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Burn a new position containing up to 4 legs wrapped in a ERC1155 token.
    /// @dev Auto-collect all accumulated fees.
    /// @param poolKey The Uniswap V4 pool key in which to burn `tokenId`
    /// @param tokenId The tokenId of the minted position, which encodes information about up to 4 legs
    /// @param positionSize The number of contracts minted, expressed in terms of the asset
    /// @param tickLimitLow The lower bound of an acceptable open interval for the ending price
    /// @param tickLimitHigh The upper bound of an acceptable open interval for the ending price
    /// @return An array of LeftRight encoded words containing the amount of currency0 and currency1 collected as fees for each leg
    /// @return The net amount of currency0 and currency1 moved to/from the Uniswap V4 pool
    function burnTokenizedPosition(
        bytes calldata poolKey,
        TokenId tokenId,
        uint128 positionSize,
        int24 tickLimitLow,
        int24 tickLimitHigh
    ) external nonReentrant returns (LeftRightUnsigned[4] memory, LeftRightSigned, int24) {
        _burn(msg.sender, TokenId.unwrap(tokenId), positionSize);

        emit TokenizedPositionBurnt(msg.sender, tokenId, positionSize);
        PoolKey memory _key = abi.decode(poolKey, (PoolKey));
        return
            _unlockAndCreatePositionInAMM(
                _key,
                tickLimitLow,
                tickLimitHigh,
                positionSize,
                tokenId.flipToBurnToken(),
                BURN
            );
    }

    /// @notice Create a new position `tokenId` containing up to 4 legs.
    /// @param poolKey The Uniswap V4 pool key in which to mint `tokenId`
    /// @param tokenId The tokenId of the minted position, which encodes information for up to 4 legs
    /// @param positionSize The number of contracts minted, expressed in terms of the asset
    /// @param tickLimitLow The lower bound of an acceptable open interval for the ending price
    /// @param tickLimitHigh The upper bound of an acceptable open interval for the ending price
    /// @return An array of LeftRight encoded words containing the amount of currency0 and currency1 collected as fees for each leg
    /// @return The net amount of currency0 and currency1 moved to/from the Uniswap V4 pool
    function mintTokenizedPosition(
        bytes calldata poolKey,
        TokenId tokenId,
        uint128 positionSize,
        int24 tickLimitLow,
        int24 tickLimitHigh
    ) external nonReentrant returns (LeftRightUnsigned[4] memory, LeftRightSigned, int24) {
        _mint(msg.sender, TokenId.unwrap(tokenId), positionSize);

        emit TokenizedPositionMinted(msg.sender, tokenId, positionSize);

        // verify that the tokenId is correctly formatted and conforms to all enforced constraints
        tokenId.validate();
        PoolKey memory _key = abi.decode(poolKey, (PoolKey));

        return
            _unlockAndCreatePositionInAMM(
                _key,
                tickLimitLow,
                tickLimitHigh,
                positionSize,
                tokenId,
                MINT
            );
    }

    /*//////////////////////////////////////////////////////////////
                     TRANSFER HOOK IMPLEMENTATIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice All ERC1155 transfers are disabled.
    function safeTransferFrom(
        address,
        address,
        uint256,
        uint256,
        bytes calldata
    ) public pure override {
        revert();
    }

    /// @notice All ERC1155 transfers are disabled.
    function safeBatchTransferFrom(
        address,
        address,
        uint256[] calldata,
        uint256[] calldata,
        bytes calldata
    ) public pure override {
        revert();
    }

    /*//////////////////////////////////////////////////////////////
              AMM INTERACTION AND POSITION UPDATE HELPERS
    //////////////////////////////////////////////////////////////*/

    /// @notice Called to perform an ITM swap in the Uniswap pool to resolve any non-tokenType token deltas.
    /// @dev When a position is minted or burnt in-the-money (ITM) we are *not* 100% currency0 or 100% currency1: we have a mix of both tokens.
    /// @dev The swapping for ITM options is needed because only one of the tokens are "borrowed" by a user to create the position.
    // This is an ITM situation below (price within the range of the chunk):
    //
    //        AMM       strike
    //     liquidity   price tick
    //        ▲           │
    //        │       ┌───▼───┐
    //        │       │       │liquidity chunk
    //        │ ┌─────┴─▲─────┴─────┐
    //        │ │       │           │
    //        │ │       :           │
    //        │ │       :           │
    //        │ │       :           │
    //        └─┴───────▲───────────┴─► price
    //                  │
    //            current price
    //             in-the-money: mix of tokens 0 and 1 within the chunk
    //
    //   If we take currency0 as an example, we deploy it to the AMM pool and *then* swap to get the right mix of currency0 and currency1
    //   to be correctly in the money at that strike.
    //   It that position is burnt, then we remove a mix of the two tokens and swap one of them so that the user receives only one.
    /// @param key The Uniswap V4 pool key in which to perform the swap
    /// @param itmAmounts How much to swap (i.e. how many tokens are ITM)
    /// @param asset The asset of the first leg of the tokenId (determines which token to swap into)
    /// @return The token deltas swapped in the AMM
    function swapInAMM(
        PoolKey memory key,
        LeftRightSigned itmAmounts,
        uint256 asset
    ) internal returns (LeftRightSigned) {
        unchecked {
            bool zeroForOne; // The direction of the swap, true for currency0 to currency1, false for currency1 to currency0
            int256 swapAmount; // The amount of currency0 or currency1 to swap

            // unpack the in-the-money amounts
            int128 itm0 = itmAmounts.rightSlot();
            int128 itm1 = itmAmounts.leftSlot();

            // NOTE: upstream users of this function such as the Panoptic Pool should ensure users always compensate for the ITM amount delta
            // the netting swap is not perfectly accurate, and it is possible for swaps to run out of liquidity, so we do not want to rely on it
            // this is simply a convenience feature, and should be treated as such
            if ((itm0 != 0) && (itm1 != 0)) {
                // ensure the tokens are swapped from the correct asset.
                if (asset == 0) {
                    zeroForOne = itm0 < 0;
                    swapAmount = itm0;
                } else {
                    zeroForOne = itm1 > 0;
                    swapAmount = itm1;
                }
            } else if (itm0 != 0) {
                zeroForOne = itm0 < 0;
                swapAmount = itm0;
            } else {
                zeroForOne = itm1 > 0;
                swapAmount = itm1;
            }

            // NOTE: can occur if itm0 and itm1 have the same value
            // in that case, swapping would be pointless so skip
            if (swapAmount == 0) return LeftRightSigned.wrap(0);

            BalanceDelta swapDelta = POOL_MANAGER_V4.swap(
                key,
                IPoolManager.SwapParams(
                    zeroForOne,
                    swapAmount,
                    zeroForOne
                        ? Constants.MIN_POOL_SQRT_RATIO + 1
                        : Constants.MAX_POOL_SQRT_RATIO - 1
                ),
                ""
            );
            // return token deltas
            return
                LeftRightSigned.wrap(0).addToRightSlot(-swapDelta.amount0()).addToLeftSlot(
                    -swapDelta.amount1()
                );
        }
    }

    /// @notice Create the position in the AMM defined by `tokenId`.
    /// @dev Loops over each leg in the tokenId and calls _createLegInAMM for each, which does the mint/burn in the AMM.
    /// @param account The address of the user creating the position
    /// @param key The Uniswap V4 pool key in which to create the position
    /// @param invertedLimits Whether the inputted lower limit > upper limit
    /// @param positionSize The size of the option position
    /// @param tokenId The option position
    /// @param isBurn Whether a position is being minted (false) or burned (true)
    /// @return collectedByLeg An array of LeftRight encoded words containing the amount of currency0 and currency1 collected as fees for each leg
    /// @return totalMoved The net amount of funds moved to/from Uniswap
    function _createPositionInAMM(
        address account,
        PoolKey memory key,
        bool invertedLimits,
        uint128 positionSize,
        TokenId tokenId,
        bool isBurn
    ) internal returns (LeftRightUnsigned[4] memory collectedByLeg, LeftRightSigned totalMoved) {
        // upper bound on amount of tokens contained across all legs of the position at any given tick
        uint256 amount0;
        uint256 amount1;

        LeftRightSigned itmAmounts;
        LeftRightUnsigned totalCollected;

        {
            PoolData poolData = s_V4toSFPMIdData[key.toId()][tokenId.vegoid()];
            if (poolData.poolId() != tokenId.poolId() || !poolData.initialized())
                revert Errors.WrongUniswapPool();

            for (uint256 leg = 0; leg < tokenId.countLegs(); ) {
                if (tokenId.width(leg) == 0) {
                    uint256 isLong = tokenId.isLong(leg);
                    LeftRightUnsigned amountsMoved = PanopticMath.getAmountsMoved(
                        tokenId,
                        positionSize,
                        leg,
                        true
                    );
                    int128 signMultiplier = isLong == 0 ? int128(-1) : int128(1);

                    {
                        uint256 tokenType = tokenId.tokenType(leg);
                        int128 itm0 = tokenType == 1
                            ? int128(0)
                            : signMultiplier * int128(amountsMoved.rightSlot());

                        int128 itm1 = tokenType == 0
                            ? int128(0)
                            : signMultiplier * int128(amountsMoved.leftSlot());

                        itmAmounts = itmAmounts.addToRightSlot(itm0).addToLeftSlot(itm1);
                    }
                } else {
                    address _account = account;
                    PoolKey memory _key = key;
                    LiquidityChunk liquidityChunk;
                    {
                        uint128 _positionSize = positionSize;
                        liquidityChunk = PanopticMath.getLiquidityChunk(
                            tokenId,
                            leg,
                            _positionSize
                        );
                    }
                    // validate tick range for newly minted positions
                    if (!isBurn) {
                        int24 tickSpacing = tokenId.tickSpacing();
                        int24 tickLower = liquidityChunk.tickLower();
                        int24 tickUpper = liquidityChunk.tickUpper();

                        if (
                            tickLower % tickSpacing != 0 ||
                            tickUpper % tickSpacing != 0 ||
                            tickLower < poolData.minEnforcedTick() ||
                            tickUpper > poolData.maxEnforcedTick()
                        ) revert Errors.InvalidTickBound();
                    }
                    unchecked {
                        // increment accumulators of the upper bound on tokens contained across all legs of the position at any given tick
                        amount0 += Math.getAmount0ForLiquidity(liquidityChunk);

                        amount1 += Math.getAmount1ForLiquidity(liquidityChunk);
                    }

                    LeftRightSigned movedLeg;
                    TokenId _tokenId = tokenId;
                    bool _isBurn = isBurn;
                    (movedLeg, collectedByLeg[leg]) = _createLegInAMM(
                        _account,
                        _key,
                        _tokenId,
                        leg,
                        liquidityChunk,
                        _isBurn,
                        _tokenId.vegoid()
                    );

                    totalMoved = totalMoved.add(movedLeg);
                    totalCollected = totalCollected.add(collectedByLeg[leg]);

                    // if tokenType is 1, and we transacted some currency0: then this leg is ITM
                    // if tokenType is 0, and we transacted some currency1: then this leg is ITM
                    itmAmounts = itmAmounts.add(
                        _tokenId.tokenType(leg) == 0
                            ? LeftRightSigned.wrap(0).addToLeftSlot(movedLeg.leftSlot())
                            : LeftRightSigned.wrap(0).addToRightSlot(movedLeg.rightSlot())
                    );
                }
                unchecked {
                    ++leg;
                }
            }
        }

        // Ensure upper bound on amount of tokens contained across all legs of the position on any given tick does not exceed a maximum of (2**127-1).
        // This is the maximum value of the `int128` type we frequently use to hold token amounts, so a given position's size should be guaranteed to
        // fit within that limit at all times.
        if (amount0 > uint128(type(int128).max - 4) || amount1 > uint128(type(int128).max - 4))
            revert Errors.PositionTooLarge();

        if (invertedLimits) {
            // if the in-the-money amount is not zero (i.e. positions were minted ITM) and the user did provide tick limits LOW > HIGH, then swap necessary amounts
            if ((LeftRightSigned.unwrap(itmAmounts) != 0)) {
                totalMoved = swapInAMM(key, itmAmounts, tokenId.asset(0)).add(totalMoved);
            }
        }

        {
            LeftRightSigned cumulativeDelta = totalMoved.sub(totalCollected);

            if (cumulativeDelta.rightSlot() > 0) {
                POOL_MANAGER_V4.burn(
                    account,
                    uint160(Currency.unwrap(key.currency0)),
                    uint128(cumulativeDelta.rightSlot())
                );
            } else if (cumulativeDelta.rightSlot() < 0) {
                POOL_MANAGER_V4.mint(
                    account,
                    uint160(Currency.unwrap(key.currency0)),
                    uint128(-cumulativeDelta.rightSlot())
                );
            }

            if (cumulativeDelta.leftSlot() > 0) {
                POOL_MANAGER_V4.burn(
                    account,
                    uint160(Currency.unwrap(key.currency1)),
                    uint128(cumulativeDelta.leftSlot())
                );
            } else if (cumulativeDelta.leftSlot() < 0) {
                POOL_MANAGER_V4.mint(
                    account,
                    uint160(Currency.unwrap(key.currency1)),
                    uint128(-cumulativeDelta.leftSlot())
                );
            }
        }
    }

    /// @notice Create the position in the AMM for a specific leg in the tokenId.
    /// @dev For the leg specified by the _leg input:
    /// @dev  - mints any new liquidity in the AMM needed (via _mintLiquidity)
    /// @dev  - burns any new liquidity in the AMM needed (via _burnLiquidity)
    /// @dev  - tracks all amounts minted and burned
    /// @dev To burn a position, the opposing position is "created" through this function,
    /// but we need to pass in a flag to indicate that so the removedLiquidity is updated.
    /// @param account The address of the user creating the position
    /// @param key The Uniswap V4 pool key in which to create the position
    /// @param tokenId The option position
    /// @param leg The leg index that needs to be modified
    /// @param liquidityChunk The liquidity chunk in Uniswap represented by the leg
    /// @param isBurn Whether a position is being burned (true) or minted (false)
    /// @return moved The net amount of funds moved to/from Uniswap
    /// @return collectedSingleLeg LeftRight encoded words containing the amount of currency0 and currency1 collected as fees
    function _createLegInAMM(
        address account,
        PoolKey memory key,
        TokenId tokenId,
        uint256 leg,
        LiquidityChunk liquidityChunk,
        bool isBurn,
        uint256 vegoid
    ) internal returns (LeftRightSigned moved, LeftRightUnsigned collectedSingleLeg) {
        // unique key to identify the liquidity chunk in this Uniswap pool
        bytes32 positionKey = EfficientHash.efficientKeccak256(
            abi.encodePacked(
                key.toId(),
                account,
                tokenId.tokenType(leg),
                liquidityChunk.tickLower(),
                liquidityChunk.tickUpper()
            )
        );

        // update our internal bookkeeping of how much liquidity we have deployed in the AMM
        // for example: if this leg is short, we add liquidity to the amm, make sure to add that to our tracking
        uint128 updatedLiquidity;
        uint256 isLong = tokenId.isLong(leg);
        LeftRightUnsigned currentLiquidity = s_accountLiquidity[positionKey];
        {
            // s_accountLiquidity is a LeftRight. The right slot represents the liquidity currently sold (added) in the AMM owned by the user
            // the left slot represents the amount of liquidity currently bought (removed) that has been removed from the AMM - the user owes it to a seller
            // the reason why it is called "removedLiquidity" is because long options are created by removing - ie. short selling LP positions
            uint128 startingLiquidity = currentLiquidity.rightSlot();
            uint128 removedLiquidity = currentLiquidity.leftSlot();
            uint128 chunkLiquidity = liquidityChunk.liquidity();

            // 0-liquidity interactions are asymmetrical in Uniswap (burning 0 liquidity is permitted and functions as a poke, but minting is prohibited)
            // thus, we prohibit all 0-liquidity chunks to prevent users from creating positions that cannot be closed
            if (chunkLiquidity == 0) revert Errors.ChunkHasZeroLiquidity();

            if (isLong == 0) {
                // selling/short: so move from account *to* uniswap
                // we're minting more liquidity in uniswap: so add the incoming liquidity chunk to the existing liquidity chunk
                updatedLiquidity = startingLiquidity + chunkLiquidity;

                /// @dev If the isLong flag is 0=short but the position was burnt, then this is closing a long position
                /// @dev so the amount of removed liquidity should decrease.
                if (isBurn) {
                    removedLiquidity -= chunkLiquidity;
                }
            } else {
                // the _leg is long (buying: moving *from* uniswap to account)
                // so we seek to move the incoming liquidity chunk *out* of uniswap - but was there sufficient liquidity sitting in uniswap
                // in the first place?
                if (startingLiquidity < chunkLiquidity) {
                    // the amount we want to move (liquidityChunk.legLiquidity()) out of uniswap is greater than
                    // what the account that owns the liquidity in uniswap has (startingLiquidity)
                    // we must ensure that an account can only move its own liquidity out of uniswap
                    // so we revert in this case
                    revert Errors.NotEnoughLiquidityInChunk();
                } else {
                    // startingLiquidity is >= chunkLiquidity, so no possible underflow
                    unchecked {
                        // we want to move less than what already sits in uniswap, no problem:
                        updatedLiquidity = startingLiquidity - chunkLiquidity;
                    }
                }

                /// @dev If the isLong flag is 1=long and the position is minted, then this is opening a long position
                /// @dev so the amount of removed liquidity should increase.
                if (!isBurn) {
                    removedLiquidity += chunkLiquidity;
                }
            }

            // update the starting liquidity for this position for next time around
            s_accountLiquidity[positionKey] = LeftRightUnsigned
                .wrap(updatedLiquidity)
                .addToLeftSlot(removedLiquidity);
        }

        // track how much liquidity we need to collect from uniswap
        // add the fees that accumulated in uniswap within the liquidityChunk:

        /* if the position is NOT long (selling a put or a call), then _mintLiquidity to move liquidity
            from the msg.sender to the Uniswap V4 pool:
            Selling(isLong=0): Mint chunk of liquidity in Uniswap (defined by upper tick, lower tick, and amount)
                   ┌─────────────────────────────────┐
            ▲     ┌▼┐ liquidityChunk                 │
            │  ┌──┴─┴──┐                         ┌───┴──┐
            │  │       │                         │      │
            └──┴───────┴──►                      └──────┘
                Uniswap V4                      account

            else: the position is long (buying a put or a call), then _burnLiquidity to remove liquidity from Uniswap V4
            Buying(isLong=1): Burn in Uniswap
                   ┌─────────────────┐
            ▲     ┌┼┐                │
            │  ┌──┴─┴──┐         ┌───▼──┐
            │  │       │         │      │
            └──┴───────┴──►      └──────┘
                Uniswap V4       account
        */

        LiquidityChunk _liquidityChunk = liquidityChunk;

        PoolKey memory _key = key;

        (BalanceDelta delta, BalanceDelta feesAccrued) = POOL_MANAGER_V4.modifyLiquidity(
            _key,
            IPoolManager.ModifyLiquidityParams(
                _liquidityChunk.tickLower(),
                _liquidityChunk.tickUpper(),
                isLong == 0
                    ? int256(uint256(_liquidityChunk.liquidity()))
                    : -int256(uint256(_liquidityChunk.liquidity())),
                positionKey
            ),
            ""
        );

        unchecked {
            moved = LeftRightSigned
                .wrap(0)
                .addToRightSlot(feesAccrued.amount0() - delta.amount0())
                .addToLeftSlot(feesAccrued.amount1() - delta.amount1());
        }

        // (premium can only be collected if liquidity existed in the chunk prior to this mint)
        if (currentLiquidity.rightSlot() > 0) {
            collectedSingleLeg = LeftRightUnsigned
                .wrap(0)
                .addToRightSlot(uint128(feesAccrued.amount0()))
                .addToLeftSlot(uint128(feesAccrued.amount1()));

            _updateStoredPremia(positionKey, currentLiquidity, collectedSingleLeg, vegoid);
        }
    }

    /// @notice Updates the premium accumulators for a chunk with the latest collected tokens.
    /// @param positionKey A key representing a liquidity chunk/range in Uniswap
    /// @param currentLiquidity The total amount of liquidity in the AMM for the specified chunk
    /// @param collectedAmounts The amount of tokens (currency0 and currency1) collected from Uniswap
    function _updateStoredPremia(
        bytes32 positionKey,
        LeftRightUnsigned currentLiquidity,
        LeftRightUnsigned collectedAmounts,
        uint256 vegoid
    ) private {
        (
            LeftRightUnsigned deltaPremiumOwed,
            LeftRightUnsigned deltaPremiumGross
        ) = _getPremiaDeltas(currentLiquidity, collectedAmounts, vegoid);

        // add deltas to accumulators and freeze both accumulators (for a token) if one of them overflows
        // (i.e if only currency0 (right slot) of the owed premium overflows, then stop accumulating  both currency0 owed premium and currency0 gross premium for the chunk)
        // this prevents situations where the owed premium gets out of sync with the gross premium due to one of them overflowing
        (s_accountPremiumOwed[positionKey], s_accountPremiumGross[positionKey]) = LeftRightLibrary
            .addCapped(
                s_accountPremiumOwed[positionKey],
                deltaPremiumOwed,
                s_accountPremiumGross[positionKey],
                deltaPremiumGross
            );
    }

    /// @notice Compute deltas for Owed/Gross premium given quantities of tokens collected from Uniswap.
    /// @dev Returned accumulators are capped at the max value (`2^128 - 1`) for each token if they overflow.
    /// @param currentLiquidity NetLiquidity (right) and removedLiquidity (left) at the start of the transaction
    /// @param collectedAmounts Total amount of tokens (currency0 and currency1) collected from Uniswap
    /// @return deltaPremiumOwed The extra premium (per liquidity X64) to be added to the owed accumulator for currency0 (right) and currency1 (left)
    /// @return deltaPremiumGross The extra premium (per liquidity X64) to be added to the gross accumulator for currency0 (right) and currency1 (left)
    function _getPremiaDeltas(
        LeftRightUnsigned currentLiquidity,
        LeftRightUnsigned collectedAmounts,
        uint256 vegoid
    )
        private
        pure
        returns (LeftRightUnsigned deltaPremiumOwed, LeftRightUnsigned deltaPremiumGross)
    {
        // extract liquidity values
        uint256 removedLiquidity = currentLiquidity.leftSlot();
        uint256 netLiquidity = currentLiquidity.rightSlot();

        // premia spread equations are graphed and documented here: https://www.desmos.com/calculator/mdeqob2m04
        // explains how we get from the premium per liquidity (calculated here) to the total premia collected and the multiplier
        // as well as how the value of VEGOID affects the premia
        // note that the "base" premium is just a common factor shared between the owed (long) and gross (short)
        // premia, and is only separated to simplify the calculation
        // (the graphed equations include this factor without separating it)
        unchecked {
            uint256 totalLiquidity = netLiquidity + removedLiquidity;

            uint256 premium0X64_base;
            uint256 premium1X64_base;

            {
                uint128 collected0 = collectedAmounts.rightSlot();
                uint128 collected1 = collectedAmounts.leftSlot();

                // compute the base premium as collected * total / net^2 (from Eqn 3)
                premium0X64_base = Math.mulDiv(
                    collected0,
                    totalLiquidity * 2 ** 64,
                    netLiquidity ** 2
                );
                premium1X64_base = Math.mulDiv(
                    collected1,
                    totalLiquidity * 2 ** 64,
                    netLiquidity ** 2
                );
            }

            {
                uint128 premium0X64_owed;
                uint128 premium1X64_owed;
                {
                    // compute the owed premium (from Eqn 3)
                    uint256 numerator = netLiquidity + (removedLiquidity / vegoid);

                    premium0X64_owed = Math
                        .mulDiv(premium0X64_base, numerator, totalLiquidity)
                        .toUint128Capped();
                    premium1X64_owed = Math
                        .mulDiv(premium1X64_base, numerator, totalLiquidity)
                        .toUint128Capped();

                    deltaPremiumOwed = LeftRightUnsigned.wrap(premium0X64_owed).addToLeftSlot(
                        premium1X64_owed
                    );
                }
            }

            {
                uint128 premium0X64_gross;
                uint128 premium1X64_gross;
                {
                    // compute the gross premium (from Eqn 4)
                    uint256 numerator = totalLiquidity ** 2 -
                        totalLiquidity *
                        removedLiquidity +
                        ((removedLiquidity ** 2) / vegoid);

                    premium0X64_gross = Math
                        .mulDiv(premium0X64_base, numerator, totalLiquidity ** 2)
                        .toUint128Capped();
                    premium1X64_gross = Math
                        .mulDiv(premium1X64_base, numerator, totalLiquidity ** 2)
                        .toUint128Capped();

                    deltaPremiumGross = LeftRightUnsigned.wrap(premium0X64_gross).addToLeftSlot(
                        premium1X64_gross
                    );
                }
            }
        }
    }

    /*//////////////////////////////////////////////////////////////
                                QUERIES
    //////////////////////////////////////////////////////////////*/

    /// @notice Return the liquidity associated with a given liquidity chunk/tokenType for a user on a Uniswap pool.
    /// @param poolKey the poolKey of the UniswapV4 pool
    /// @param owner The address of the account that is queried
    /// @param tokenType The tokenType of the position
    /// @param tickLower The lower end of the tick range for the position
    /// @param tickUpper The upper end of the tick range for the position
    /// @return accountLiquidities The amount of liquidity that held in and removed from Uniswap for that chunk (netLiquidity:removedLiquidity -> rightSlot:leftSlot)
    function getAccountLiquidity(
        bytes calldata poolKey,
        address owner,
        uint256 tokenType,
        int24 tickLower,
        int24 tickUpper
    ) external view returns (LeftRightUnsigned accountLiquidities) {
        PoolKey memory key = abi.decode(poolKey, (PoolKey));

        // Extract the account liquidity for a given Uniswap pool, owner, token type, and ticks
        // tokenType input here is the asset of the positions minted, this avoids put liquidity to be used for call, and vice-versa
        accountLiquidities = s_accountLiquidity[
            EfficientHash.efficientKeccak256(
                abi.encodePacked(key.toId(), owner, tokenType, tickLower, tickUpper)
            )
        ];
    }

    /// @notice Return the premium associated with a given position, where premium is an accumulator of feeGrowth for the touched position.
    /// @dev If an atTick parameter is provided that is different from `type(int24).max`, then it will update the premium up to the current
    /// block at the provided atTick value. We do this because this may be called immediately after the Uniswap V4 pool has been touched,
    /// so no need to read the feeGrowths from the Uniswap V4 pool.
    /// @param poolKey the poolKey of the UniswapV4 pool
    /// @param owner The address of the account that is queried
    /// @param tokenType The tokenType of the position
    /// @param tickLower The lower end of the tick range for the position
    /// @param tickUpper The upper end of the tick range for the position
    /// @param atTick The current tick. Set `atTick < (type(int24).max = 8388608)` to get latest premium up to the current block
    /// @param isLong Whether the position is long (=1) or short (=0)
    /// @return The amount of premium (per liquidity X64) for currency0 = `sum(feeGrowthLast0X128)` over every block where the position has been touched
    /// @return The amount of premium (per liquidity X64) for currency1 = `sum(feeGrowthLast0X128)` over every block where the position has been touched
    function getAccountPremium(
        bytes calldata poolKey,
        address owner,
        uint256 tokenType,
        int24 tickLower,
        int24 tickUpper,
        int24 atTick,
        uint256 isLong,
        uint256 vegoid
    ) external view returns (uint128, uint128) {
        PoolKey memory key = abi.decode(poolKey, (PoolKey));
        bytes32 positionKey = EfficientHash.efficientKeccak256(
            abi.encodePacked(key.toId(), owner, tokenType, tickLower, tickUpper)
        );

        LeftRightUnsigned acctPremia;

        LeftRightUnsigned accountLiquidities = s_accountLiquidity[positionKey];
        uint128 netLiquidity = accountLiquidities.rightSlot();

        // Compute the premium up to the current block (ie. after last touch until now). Do not proceed if `atTick == (type(int24).max = 8388608)`
        if (atTick < type(int24).max && netLiquidity != 0) {
            // unique key to identify the liquidity chunk in this Uniswap pool
            LeftRightUnsigned amountToCollect;
            {
                PoolId _idV4 = key.toId();
                int24 _tickLower = tickLower;
                int24 _tickUpper = tickUpper;
                int24 _atTick = atTick;
                bytes32 _positionKey = positionKey;

                (uint256 feeGrowthInside0X128, uint256 feeGrowthInside1X128) = V4StateReader
                    .getFeeGrowthInside(POOL_MANAGER_V4, _idV4, _atTick, _tickLower, _tickUpper);

                (uint256 feeGrowthInside0LastX128, uint256 feeGrowthInside1LastX128) = V4StateReader
                    .getFeeGrowthInsideLast(
                        POOL_MANAGER_V4,
                        _idV4,
                        keccak256(
                            abi.encodePacked(address(this), _tickLower, _tickUpper, _positionKey)
                        )
                    );

                unchecked {
                    amountToCollect = LeftRightUnsigned
                        .wrap(
                            uint128(
                                Math.mulDiv128(
                                    feeGrowthInside0X128 - feeGrowthInside0LastX128,
                                    netLiquidity
                                )
                            )
                        )
                        .addToLeftSlot(
                            uint128(
                                Math.mulDiv128(
                                    feeGrowthInside1X128 - feeGrowthInside1LastX128,
                                    netLiquidity
                                )
                            )
                        );
                }
            }

            (LeftRightUnsigned premiumOwed, LeftRightUnsigned premiumGross) = _getPremiaDeltas(
                accountLiquidities,
                amountToCollect,
                vegoid
            );

            // add deltas to accumulators and freeze both accumulators (for a token) if one of them overflows
            // (i.e if only currency0 (right slot) of the owed premium overflows, then stop accumulating both currency0 owed premium and currency0 gross premium for the chunk)
            // this prevents situations where the owed premium gets out of sync with the gross premium due to one of them overflowing
            (premiumOwed, premiumGross) = LeftRightLibrary.addCapped(
                s_accountPremiumOwed[positionKey],
                premiumOwed,
                s_accountPremiumGross[positionKey],
                premiumGross
            );

            acctPremia = isLong == 1 ? premiumOwed : premiumGross;
        } else {
            // Extract the account liquidity for a given Uniswap pool, owner, token type, and ticks
            acctPremia = isLong == 1
                ? s_accountPremiumOwed[positionKey]
                : s_accountPremiumGross[positionKey];
        }
        return (acctPremia.rightSlot(), acctPremia.leftSlot());
    }

    /// @notice Returns the Uniswap V4 poolkey  for a given `poolId`.
    /// @param poolId The unique pool identifier for a Uni V4 pool in the SFPM
    /// @return The Uniswap V4 pool key corresponding to `poolId`
    function getUniswapV4PoolKeyFromId(uint64 poolId) external view returns (PoolKey memory) {
        return s_poolIdToKey[poolId];
    }

    /// @notice Returns the current enforced tick limits for a given idV4 `poolId`.
    /// @param poolId The unique pool identifier for a Uniswap V4 pool
    /// @return The minimum enforced tick for chunks created in the pool corresponding to `poolId`
    /// @return The maximum enforced tick for chunks created in the pool corresponding to `poolId`
    function getEnforcedTickLimits(uint64 poolId) external view returns (int24, int24) {
        PoolKey memory poolKey = s_poolIdToKey[poolId];
        uint256 vegoid = uint8(poolId >> 40);
        PoolData poolData = s_V4toSFPMIdData[poolKey.toId()][vegoid];
        return (poolData.minEnforcedTick(), poolData.maxEnforcedTick());
    }

    /// @notice Returns the `poolId` for a given Uniswap pool.
    /// @param id The PoolId of the Uniswap V4 Pool
    /// @return poolId The unique pool identifier corresponding to a idV4
    function getPoolId(bytes memory id, uint8 vegoid) external view returns (uint64) {
        PoolId idV4 = abi.decode(id, (PoolId));
        return s_V4toSFPMIdData[idV4][vegoid].poolId();
    }

    /// @notice Returns the current tick of a given Uniswap V4 pool
    /// @param poolKey the poolKey of the UniswapV4 pool
    /// @return currentTick The current tick of the Uniswap pool
    function getCurrentTick(bytes memory poolKey) public view returns (int24 currentTick) {
        PoolKey memory key = abi.decode(poolKey, (PoolKey));
        currentTick = V4StateReader.getTick(POOL_MANAGER_V4, key.toId());
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

// Uniswap V4 interfaces
import {IPoolManager} from "v4-core/interfaces/IPoolManager.sol";
// Uniswap V4 libraries
import {StateLibrary} from "v4-core/libraries/StateLibrary.sol";
// Uniswap V4 types
import {PoolId} from "v4-core/types/PoolId.sol";

/// @notice A library to retrieve state information from Uniswap V4 pools via `extsload`.
/// @author Axicon Labs Limited, credit to Uniswap Labs under MIT License
library V4StateReader {
    /// @notice Retrieves the current `sqrtPriceX96` from a Uniswap V4 pool.
    /// @param manager The Uniswap V4 pool manager contract
    /// @param poolId The pool ID of the Uniswap V4 pool
    /// @return sqrtPriceX96 The current `sqrtPriceX96` of the Uniswap V4 pool
    function getSqrtPriceX96(
        IPoolManager manager,
        PoolId poolId
    ) internal view returns (uint160 sqrtPriceX96) {
        bytes32 stateSlot = StateLibrary._getPoolStateSlot(poolId);
        bytes32 data = manager.extsload(stateSlot);

        assembly ("memory-safe") {
            sqrtPriceX96 := and(data, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF)
        }
    }

    /// @notice Retrieves the current tick from a Uniswap V4 pool.
    /// @param manager The Uniswap V4 pool manager contract
    /// @param poolId The pool ID of the Uniswap V4 pool
    /// @return tick The current tick of the Uniswap V4 pool
    function getTick(IPoolManager manager, PoolId poolId) internal view returns (int24 tick) {
        bytes32 stateSlot = StateLibrary._getPoolStateSlot(poolId);
        bytes32 data = manager.extsload(stateSlot);

        assembly ("memory-safe") {
            tick := signextend(2, shr(160, data))
        }
    }

    /// @notice Calculates the fee growth that has occurred (per unit of liquidity) in the AMM/Uniswap for an
    /// option position's tick range.
    /// @param manager The Uniswap V4 pool manager contract
    /// @param idV4 The pool ID of the Uniswap V4 pool
    /// @param currentTick The current price tick in the AMM
    /// @param tickLower The lower tick of the option position leg (a liquidity chunk)
    /// @param tickUpper The upper tick of the option position leg (a liquidity chunk)
    /// @return feeGrowthInside0X128 The fee growth in the AMM for currency0
    /// @return feeGrowthInside1X128 The fee growth in the AMM for currency1
    function getFeeGrowthInside(
        IPoolManager manager,
        PoolId idV4,
        int24 currentTick,
        int24 tickLower,
        int24 tickUpper
    ) internal view returns (uint256 feeGrowthInside0X128, uint256 feeGrowthInside1X128) {
        // Get feesGrowths from the option position's lower+upper ticks
        // lowerOut0: For currency0: fee growth per unit of liquidity on the _other_ side of tickLower (relative to currentTick)
        // only has relative meaning, not absolute — the value depends on when the tick is initialized
        // (...)
        // upperOut1: For currency1: fee growth on the _other_ side of tickUpper (again: relative to currentTick)
        // the point is: the range covered by lowerOut0 changes depending on where currentTick is.
        (uint256 lowerOut0, uint256 lowerOut1) = StateLibrary.getTickFeeGrowthOutside(
            manager,
            idV4,
            tickLower
        );
        (uint256 upperOut0, uint256 upperOut1) = StateLibrary.getTickFeeGrowthOutside(
            manager,
            idV4,
            tickUpper
        );

        // compute the effective feeGrowth, depending on whether price is above/below/within range
        unchecked {
            if (currentTick < tickLower) {
                /**
                  Diagrams shown for currency0, and applies for currency1 the same
                  L = lowerTick, U = upperTick

                    liquidity         lowerOut0 (all fees collected in this price tick range for currency0)
                        ▲            ◄──────────────^v───► (to MAX_TICK)
                        │
                        │                      upperOut0
                        │                     ◄─────^v───►
                        │           ┌────────┐
                        │           │ chunk  │
                        │           │        │
                        └─────▲─────┴────────┴────────► price tick
                              │     L        U
                              │
                           current
                            tick
                */
                feeGrowthInside0X128 = lowerOut0 - upperOut0; // fee growth inside the chunk
                feeGrowthInside1X128 = lowerOut1 - upperOut1;
            } else if (currentTick >= tickUpper) {
                /**
                    liquidity
                        ▲           upperOut0
                        │◄─^v─────────────────────►
                        │
                        │     lowerOut0  ┌────────┐
                        │◄─^v───────────►│ chunk  │
                        │                │        │
                        └────────────────┴────────┴─▲─────► price tick
                                         L        U │
                                                    │
                                                 current
                                                  tick
                 */
                feeGrowthInside0X128 = upperOut0 - lowerOut0;
                feeGrowthInside1X128 = upperOut1 - lowerOut1;
            } else {
                /**
                  current AMM tick is within the option position range (within the chunk)

                     liquidity
                        ▲        feeGrowthGlobal0X128 = global fee growth
                        │                             = (all fees collected for the entire price range for currency0)
                        │
                        │
                        │     lowerOut0  ┌──────────────┐ upperOut0
                        │◄─^v───────────►│              │◄─────^v───►
                        │                │     chunk    │
                        │                │              │
                        └────────────────┴───────▲──────┴─────► price tick
                                         L       │      U
                                                 │
                                              current
                                               tick
                */

                (uint256 feeGrowthGlobal0X128, uint256 feeGrowthGlobal1X128) = StateLibrary
                    .getFeeGrowthGlobals(manager, idV4);
                feeGrowthInside0X128 = feeGrowthGlobal0X128 - lowerOut0 - upperOut0;
                feeGrowthInside1X128 = feeGrowthGlobal1X128 - lowerOut1 - upperOut1;
            }
        }
    }

    /// @notice Retrieves the last stored `feeGrowthInsideLast` values for a unique Uniswap V4 position.
    /// @dev Corresponds to pools[poolId].positions[positionId] in `manager`.
    /// @param manager The Uniswap V4 pool manager contract
    /// @param poolId The ID of the Uniswap V4 pool
    /// @param positionId The ID of the position, which is a hash of the owner, tickLower, tickUpper, and salt
    /// @return feeGrowthInside0LastX128 The fee growth inside the position for currency0
    /// @return feeGrowthInside1LastX128 The fee growth inside the position for currency1
    function getFeeGrowthInsideLast(
        IPoolManager manager,
        PoolId poolId,
        bytes32 positionId
    ) internal view returns (uint256 feeGrowthInside0LastX128, uint256 feeGrowthInside1LastX128) {
        bytes32 slot = StateLibrary._getPositionInfoSlot(poolId, positionId);

        // read all 3 words of the Position.State struct
        bytes32[] memory data = manager.extsload(slot, 3);

        assembly ("memory-safe") {
            feeGrowthInside0LastX128 := mload(add(data, 64))
            feeGrowthInside1LastX128 := mload(add(data, 96))
        }
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.24;

// Libraries
import {Errors} from "@libraries/Errors.sol";
import {Math} from "@libraries/Math.sol";

type LeftRightUnsigned is uint256;
using LeftRightLibrary for LeftRightUnsigned global;

type LeftRightSigned is int256;
using LeftRightLibrary for LeftRightSigned global;

/// @title Pack two separate data (each of 128bit) into a single 256-bit slot; 256bit-to-128bit packing methods.
/// @author Axicon Labs Limited
/// @notice Simple data type that divides a 256-bit word into two 128-bit slots.
library LeftRightLibrary {
    using Math for uint256;

    /// @notice AND bitmask to isolate the left half of a uint256.
    uint256 internal constant LEFT_HALF_BIT_MASK =
        0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF00000000000000000000000000000000;

    /// @notice AND bitmask to isolate the left half of an int256.
    int256 internal constant LEFT_HALF_BIT_MASK_INT =
        int256(uint256(0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF00000000000000000000000000000000));

    /// @notice AND bitmask to isolate the right half of an int256.
    int256 internal constant RIGHT_HALF_BIT_MASK = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;

    /*//////////////////////////////////////////////////////////////
                               RIGHT SLOT
    //////////////////////////////////////////////////////////////*/

    /// @notice Get the "right" slot from a bit pattern.
    /// @param self The 256 bit value to extract the right half from
    /// @return The right half of `self`
    function rightSlot(LeftRightUnsigned self) internal pure returns (uint128) {
        return uint128(LeftRightUnsigned.unwrap(self));
    }

    /// @notice Get the "right" slot from a bit pattern.
    /// @param self The 256 bit value to extract the right half from
    /// @return The right half of `self`
    function rightSlot(LeftRightSigned self) internal pure returns (int128) {
        return int128(LeftRightSigned.unwrap(self));
    }

    // All addToRightSlot functions add bits to the right slot without clearing it first
    // Typically, the slot is already clear when writing to it, but if it is not, the bits will be added to the existing bits
    // Therefore, the assumption must not be made that the bits will be cleared while using these helpers
    // Note that the values *within* the slots are allowed to overflow, but overflows are contained and will not leak into the other slot

    /// @notice Add to the "right" slot in a 256-bit pattern.
    /// @param self The 256-bit pattern to be written to
    /// @param right The value to be added to the right slot
    /// @return `self` with `right` added (not overwritten, but added) to the value in its right 128 bits
    function addToRightSlot(
        LeftRightUnsigned self,
        uint128 right
    ) internal pure returns (LeftRightUnsigned) {
        unchecked {
            // prevent the right slot from leaking into the left one in the case of an overflow
            // ff + 1 = (1)00, but we want just ff + 1 = 00
            return
                LeftRightUnsigned.wrap(
                    (LeftRightUnsigned.unwrap(self) & LEFT_HALF_BIT_MASK) +
                        uint256(uint128(LeftRightUnsigned.unwrap(self)) + right)
                );
        }
    }

    /// @notice Add to the "right" slot in a 256-bit pattern.
    /// @param self The 256-bit pattern to be written to
    /// @param right The value to be added to the right slot
    /// @return `self` with `right` added (not overwritten, but added) to the value in its right 128 bits
    function addToRightSlot(
        LeftRightSigned self,
        int128 right
    ) internal pure returns (LeftRightSigned) {
        // bit mask needed in case rightHalfBitPattern < 0 due to 2's complement
        unchecked {
            // prevent the right slot from leaking into the left one in the case of a positive sign change
            // ff + 1 = (1)00, but we want just ff + 1 = 00
            return
                LeftRightSigned.wrap(
                    (LeftRightSigned.unwrap(self) & LEFT_HALF_BIT_MASK_INT) +
                        (int256(int128(LeftRightSigned.unwrap(self)) + right) & RIGHT_HALF_BIT_MASK)
                );
        }
    }

    /*//////////////////////////////////////////////////////////////
                               LEFT SLOT
    //////////////////////////////////////////////////////////////*/

    /// @notice Get the "left" slot from a bit pattern.
    /// @param self The 256 bit value to extract the left half from
    /// @return The left half of `self`
    function leftSlot(LeftRightUnsigned self) internal pure returns (uint128) {
        return uint128(LeftRightUnsigned.unwrap(self) >> 128);
    }

    /// @notice Get the "left" slot from a bit pattern.
    /// @param self The 256 bit value to extract the left half from
    /// @return The left half of `self`
    function leftSlot(LeftRightSigned self) internal pure returns (int128) {
        return int128(LeftRightSigned.unwrap(self) >> 128);
    }

    /// All addToLeftSlot functions add bits to the left slot without clearing it first
    // Typically, the slot is already clear when writing to it, but if it is not, the bits will be added to the existing bits
    // Therefore, the assumption must not be made that the bits will be cleared while using these helpers
    // Note that the values *within* the slots are allowed to overflow, but overflows are contained and will not leak into the other slot

    /// @notice Add to the "left" slot in a 256-bit pattern.
    /// @param self The 256-bit pattern to be written to
    /// @param left The value to be added to the left slot
    /// @return `self` with `left` added (not overwritten, but added) to the value in its left 128 bits
    function addToLeftSlot(
        LeftRightUnsigned self,
        uint128 left
    ) internal pure returns (LeftRightUnsigned) {
        unchecked {
            return LeftRightUnsigned.wrap(LeftRightUnsigned.unwrap(self) + (uint256(left) << 128));
        }
    }

    /// @notice Add to the "left" slot in a 256-bit pattern.
    /// @param self The 256-bit pattern to be written to
    /// @param left The value to be added to the left slot
    /// @return `self` with `left` added (not overwritten, but added) to the value in its left 128 bits
    function addToLeftSlot(
        LeftRightSigned self,
        int128 left
    ) internal pure returns (LeftRightSigned) {
        unchecked {
            return LeftRightSigned.wrap(LeftRightSigned.unwrap(self) + (int256(left) << 128));
        }
    }

    /*//////////////////////////////////////////////////////////////
                             MATH FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Add two LeftRight-encoded words; revert on overflow or underflow.
    /// @param x The augend
    /// @param y The addend
    /// @return z The sum `x + y`
    function add(
        LeftRightUnsigned x,
        LeftRightUnsigned y
    ) internal pure returns (LeftRightUnsigned z) {
        unchecked {
            // adding leftRight packed uint128's is same as just adding the values explicitly
            // given that we check for overflows of the left and right values
            z = LeftRightUnsigned.wrap(LeftRightUnsigned.unwrap(x) + LeftRightUnsigned.unwrap(y));

            // on overflow z will be less than either x or y
            // type cast z to uint128 to isolate the right slot and if it's lower than a value it's comprised of (x)
            // then an overflow has occurred
            if (
                LeftRightUnsigned.unwrap(z) < LeftRightUnsigned.unwrap(x) ||
                (uint128(LeftRightUnsigned.unwrap(z)) < uint128(LeftRightUnsigned.unwrap(x)))
            ) revert Errors.UnderOverFlow();
        }
    }

    /// @notice Subtract two LeftRight-encoded words; revert on overflow or underflow.
    /// @param x The minuend
    /// @param y The subtrahend
    /// @return z The difference `x - y`
    function sub(
        LeftRightUnsigned x,
        LeftRightUnsigned y
    ) internal pure returns (LeftRightUnsigned z) {
        unchecked {
            // subtracting leftRight packed uint128's is same as just subtracting the values explicitly
            // given that we check for underflows of the left and right values
            z = LeftRightUnsigned.wrap(LeftRightUnsigned.unwrap(x) - LeftRightUnsigned.unwrap(y));

            // on underflow z will be greater than either x or y
            // type cast z to uint128 to isolate the right slot and if it's higher than a value that was subtracted from (x)
            // then an underflow has occurred
            if (
                LeftRightUnsigned.unwrap(z) > LeftRightUnsigned.unwrap(x) ||
                (uint128(LeftRightUnsigned.unwrap(z)) > uint128(LeftRightUnsigned.unwrap(x)))
            ) revert Errors.UnderOverFlow();
        }
    }

    /// @notice Add two LeftRight-encoded words; revert on overflow or underflow.
    /// @param x The augend
    /// @param y The addend
    /// @return z The sum `x + y`
    function add(LeftRightUnsigned x, LeftRightSigned y) internal pure returns (LeftRightSigned z) {
        unchecked {
            int256 left = int256(uint256(x.leftSlot())) + y.leftSlot();
            int128 left128 = int128(left);

            if (left128 != left) revert Errors.UnderOverFlow();

            int256 right = int256(uint256(x.rightSlot())) + y.rightSlot();
            int128 right128 = int128(right);

            if (right128 != right) revert Errors.UnderOverFlow();

            return z.addToRightSlot(right128).addToLeftSlot(left128);
        }
    }

    /// @notice Add two LeftRight-encoded words; revert on overflow or underflow.
    /// @param x The augend
    /// @param y The addend
    /// @return z The sum `x + y`
    function add(LeftRightSigned x, LeftRightSigned y) internal pure returns (LeftRightSigned z) {
        unchecked {
            int256 left256 = int256(x.leftSlot()) + y.leftSlot();
            int128 left128 = int128(left256);

            int256 right256 = int256(x.rightSlot()) + y.rightSlot();
            int128 right128 = int128(right256);

            if (left128 != left256 || right128 != right256) revert Errors.UnderOverFlow();

            return z.addToRightSlot(right128).addToLeftSlot(left128);
        }
    }

    /// @notice Subtract two LeftRight-encoded words; revert on overflow or underflow.
    /// @param x The minuend
    /// @param y The subtrahend
    /// @return z The difference `x - y`
    function sub(LeftRightSigned x, LeftRightSigned y) internal pure returns (LeftRightSigned z) {
        unchecked {
            int256 left256 = int256(x.leftSlot()) - y.leftSlot();
            int128 left128 = int128(left256);

            int256 right256 = int256(x.rightSlot()) - y.rightSlot();
            int128 right128 = int128(right256);

            if (left128 != left256 || right128 != right256) revert Errors.UnderOverFlow();

            return z.addToRightSlot(right128).addToLeftSlot(left128);
        }
    }

    /// @notice Subtract two LeftRight-encoded words; revert on overflow or underflow.
    /// @param x The minuend
    /// @param y The subtrahend
    /// @return z The difference `x - y`
    function sub(LeftRightSigned x, LeftRightUnsigned y) internal pure returns (LeftRightSigned z) {
        unchecked {
            int256 left256 = int256(x.leftSlot()) - int256(uint256(y.leftSlot()));
            int128 left128 = int128(left256);

            int256 right256 = int256(x.rightSlot()) - int256(uint256(y.rightSlot()));
            int128 right128 = int128(right256);

            if (left128 != left256 || right128 != right256) revert Errors.UnderOverFlow();

            return z.addToRightSlot(right128).addToLeftSlot(left128);
        }
    }

    /// @notice Subtract two LeftRight-encoded words; revert on overflow or underflow.
    /// @notice For each slot, rectify difference `x - y` to 0 if negative.
    /// @param x The minuend
    /// @param y The subtrahend
    /// @return z The difference `x - y`
    function subRect(
        LeftRightSigned x,
        LeftRightSigned y
    ) internal pure returns (LeftRightUnsigned z) {
        unchecked {
            int256 left256 = int256(x.leftSlot()) - y.leftSlot();
            int128 left128 = int128(left256);

            int256 right256 = int256(x.rightSlot()) - y.rightSlot();
            int128 right128 = int128(right256);

            if (left128 != left256 || right128 != right256) revert Errors.UnderOverFlow();

            return
                z.addToRightSlot(uint128(uint256((Math.max(right128, 0))))).addToLeftSlot(
                    uint128(uint256((Math.max(left128, 0))))
                );
        }
    }

    /// @notice Adds two sets of LeftRight-encoded words, freezing both right slots if either overflows, and vice versa.
    /// @dev Used for linked accumulators, so if the accumulator for one side overflows for a token, both cease to accumulate.
    /// @param x The first augend
    /// @param dx The addend for `x`
    /// @param y The second augend
    /// @param dy The addend for `y`
    /// @return The sum `x + dx`
    /// @return The sum `y + dy`
    function addCapped(
        LeftRightUnsigned x,
        LeftRightUnsigned dx,
        LeftRightUnsigned y,
        LeftRightUnsigned dy
    ) internal pure returns (LeftRightUnsigned, LeftRightUnsigned) {
        uint128 z_xR = (uint256(x.rightSlot()) + dx.rightSlot()).toUint128Capped();
        uint128 z_xL = (uint256(x.leftSlot()) + dx.leftSlot()).toUint128Capped();
        uint128 z_yR = (uint256(y.rightSlot()) + dy.rightSlot()).toUint128Capped();
        uint128 z_yL = (uint256(y.leftSlot()) + dy.leftSlot()).toUint128Capped();

        bool r_Enabled = !(z_xR == type(uint128).max || z_yR == type(uint128).max);
        bool l_Enabled = !(z_xL == type(uint128).max || z_yL == type(uint128).max);

        return (
            LeftRightUnsigned.wrap(r_Enabled ? z_xR : x.rightSlot()).addToLeftSlot(
                l_Enabled ? z_xL : x.leftSlot()
            ),
            LeftRightUnsigned.wrap(r_Enabled ? z_yR : y.rightSlot()).addToLeftSlot(
                l_Enabled ? z_yL : y.leftSlot()
            )
        );
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.24;

type LiquidityChunk is uint256;
using LiquidityChunkLibrary for LiquidityChunk global;

/// @title A Panoptic Liquidity Chunk. Tracks Tick Range and Liquidity Information for a "chunk." Used to track movement of chunks.
/// @author Axicon Labs Limited
///
/// @notice A liquidity chunk is an amount of `liquidity` deployed between two ticks: `tickLower` and `tickUpper`
/// into a concentrated liquidity AMM.
//
//                liquidity
//                    ▲      liquidity chunk
//                    │        │
//                    │    ┌───▼────┐   ▲
//                    │    │        │   │ liquidity/size
//      Other AMM     │  ┌─┴────────┴─┐ ▼ of chunk
//      liquidity  ───┼──┼─►          │
//                    │  │            │
//                    └──┴─▲────────▲─┴──► price ticks
//                         │        │
//                         │        │
//                    tickLower     │
//                              tickUpper
//
// PACKING RULES FOR A LIQUIDITYCHUNK:
// =================================================================================================
//  From the LSB to the MSB:
// (1) Liquidity        128bits  : The liquidity within the chunk (uint128).
// ( ) (Zero-bits)       80bits  : Zero-bits to match a total uint256.
// (2) tick Upper        24bits  : The upper tick of the chunk (int24).
// (3) tick Lower        24bits  : The lower tick of the chunk (int24).
// Total                256bits  : Total bits used by a chunk.
// ===============================================================================================
//
// The bit pattern is therefore:
//
//           (3)             (2)             ( )                (1)
//    <-- 24 bits -->  <-- 24 bits -->  <-- 80 bits -->   <-- 128 bits -->
//        tickLower       tickUpper         Zeros             Liquidity
//
//        <--- most significant bit        least significant bit --->
//
library LiquidityChunkLibrary {
    /// @notice AND mask to strip the `tickLower` value from a packed LiquidityChunk.
    uint256 internal constant CLEAR_TL_MASK =
        0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;

    /// @notice AND mask to strip the `tickUpper` value from a packed LiquidityChunk.
    uint256 internal constant CLEAR_TU_MASK =
        0xFFFFFF000000FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;

    /*//////////////////////////////////////////////////////////////
                                ENCODING
    //////////////////////////////////////////////////////////////*/

    /// @notice Create a new `LiquidityChunk` given by its bounding ticks and its liquidity.
    /// @param _tickLower The lower tick of the chunk
    /// @param _tickUpper The upper tick of the chunk
    /// @param amount The amount of liquidity to add to the chunk
    /// @return The new chunk with the given liquidity and tick range
    function createChunk(
        int24 _tickLower,
        int24 _tickUpper,
        uint128 amount
    ) internal pure returns (LiquidityChunk) {
        unchecked {
            // casting to 'uint256' is safe because _tickUpper/_tickLower is always < 2**24
            // forge-lint: disable-next-line(unsafe-typecast)
            return
                LiquidityChunk.wrap(
                    // casting to 'uint24' is safe because _tickLower/_tickUpper is always < 2**24
                    // forge-lint: disable-next-line(unsafe-typecast)
                    (uint256(uint24(_tickLower)) << 232) +
                        (uint256(uint24(_tickUpper)) << 208) +
                        uint256(amount)
                );
        }
    }

    /// @notice Add liquidity to `self`.
    /// @param self The LiquidityChunk to add liquidity to
    /// @param amount The amount of liquidity to add to `self`
    /// @return `self` with added liquidity `amount`
    function addLiquidity(
        LiquidityChunk self,
        uint128 amount
    ) internal pure returns (LiquidityChunk) {
        unchecked {
            return LiquidityChunk.wrap(LiquidityChunk.unwrap(self) + amount);
        }
    }

    /// @notice Add the lower tick to `self`.
    /// @param self The LiquidityChunk to add the lower tick to
    /// @param _tickLower The lower tick to add to `self`
    /// @return `self` with added lower tick `_tickLower`
    function addTickLower(
        LiquidityChunk self,
        int24 _tickLower
    ) internal pure returns (LiquidityChunk) {
        unchecked {
            return
                LiquidityChunk.wrap(
                    LiquidityChunk.unwrap(self) + (uint256(uint24(_tickLower)) << 232)
                );
        }
    }

    /// @notice Add the upper tick to `self`.
    /// @param self The LiquidityChunk to add the upper tick to
    /// @param _tickUpper The upper tick to add to `self`
    /// @return `self` with added upper tick `_tickUpper`
    function addTickUpper(
        LiquidityChunk self,
        int24 _tickUpper
    ) internal pure returns (LiquidityChunk) {
        unchecked {
            return
                LiquidityChunk.wrap(
                    LiquidityChunk.unwrap(self) + ((uint256(uint24(_tickUpper))) << 208)
                );
        }
    }

    /// @notice Overwrites the lower tick on `self`.
    /// @param self The LiquidityChunk to overwrite the lower tick on
    /// @param _tickLower The lower tick to overwrite `self` with
    /// @return `self` with `_tickLower` as the new lower tick
    function updateTickLower(
        LiquidityChunk self,
        int24 _tickLower
    ) internal pure returns (LiquidityChunk) {
        unchecked {
            return
                LiquidityChunk.wrap(LiquidityChunk.unwrap(self) & CLEAR_TL_MASK).addTickLower(
                    _tickLower
                );
        }
    }

    /// @notice Overwrites the upper tick on `self`.
    /// @param self The LiquidityChunk to overwrite the upper tick on
    /// @param _tickUpper The upper tick to overwrite `self` with
    /// @return `self` with `_tickUpper` as the new upper tick
    function updateTickUpper(
        LiquidityChunk self,
        int24 _tickUpper
    ) internal pure returns (LiquidityChunk) {
        unchecked {
            return
                LiquidityChunk.wrap(LiquidityChunk.unwrap(self) & CLEAR_TU_MASK).addTickUpper(
                    _tickUpper
                );
        }
    }

    /*//////////////////////////////////////////////////////////////
                                DECODING
    //////////////////////////////////////////////////////////////*/

    /// @notice Get the lower tick of `self`.
    /// @param self The LiquidityChunk to get the lower tick from
    /// @return The lower tick of `self`
    function tickLower(LiquidityChunk self) internal pure returns (int24) {
        unchecked {
            return int24(int256(LiquidityChunk.unwrap(self) >> 232));
        }
    }

    /// @notice Get the upper tick of `self`.
    /// @param self The LiquidityChunk to get the upper tick from
    /// @return The upper tick of `self`
    function tickUpper(LiquidityChunk self) internal pure returns (int24) {
        unchecked {
            return int24(int256(LiquidityChunk.unwrap(self) >> 208));
        }
    }

    /// @notice Get the amount of liquidity/size of `self`.
    /// @param self The LiquidityChunk to get the liquidity from
    /// @return The liquidity of `self`
    function liquidity(LiquidityChunk self) internal pure returns (uint128) {
        unchecked {
            return uint128(LiquidityChunk.unwrap(self));
        }
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.24;

// Interfaces
import {IUniswapV3Factory} from "univ3-core/interfaces/IUniswapV3Factory.sol";
// Libraries
import {Errors} from "@libraries/Errors.sol";

/// @title Library for verifying and decoding Uniswap callbacks.
/// @author Axicon Labs Limited
/// @notice This library provides functions to verify that a callback came from a canonical Uniswap V3 pool with a claimed set of features.
library CallbackLib {
    /// @notice Type defining characteristics of a Uniswap V3 pool.
    /// @param token0 The address of `token0` for the Uniswap pool
    /// @param token1 The address of `token1` for the Uniswap pool
    /// @param fee The fee tier of the Uniswap pool (in hundredths of basis points)
    struct PoolFeatures {
        address token0;
        address token1;
        uint24 fee;
    }

    /// @notice Type for data sent by pool in mint/swap callbacks used to validate the pool and send back requisite tokens.
    /// @param poolFeatures The features of the pool that sent the callback (used to validate that the pool is canonical)
    /// @param payer The address from which the requested tokens should be transferred
    struct CallbackData {
        PoolFeatures poolFeatures;
        address payer;
    }

    /// @notice Verifies that a callback came from the canonical Uniswap pool with a claimed set of features.
    /// @param sender The address initiating the callback and claiming to be a Uniswap pool
    /// @param factory The address of the canonical Uniswap V3 factory
    /// @param features The features `sender` claims to contain (tokens and fee)
    function validateCallback(
        address sender,
        IUniswapV3Factory factory,
        PoolFeatures memory features
    ) internal view {
        // Call getPool on the factory to verify that the sender corresponds to the canonical pool with the claimed features
        if (factory.getPool(features.token0, features.token1, features.fee) != sender)
            revert Errors.InvalidUniswapCallback();
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.24;

// Libraries
import {Constants} from "@libraries/Constants.sol";
import {Errors} from "@libraries/Errors.sol";
import {PanopticMath} from "@libraries/PanopticMath.sol";

type TokenId is uint256;
using TokenIdLibrary for TokenId global;

/// @title Panoptic's tokenId: the fundamental options position.
/// @author Axicon Labs Limited
/// @notice This is the token ID used in the ERC1155 representation of the option position in the SFPM.
/// @notice The SFPM "overloads" the ERC1155 `id` by storing all option information in said `id`.
/// @notice Contains methods for packing and unpacking a Panoptic options position into a uint256 bit pattern.
// PACKING RULES FOR A TOKENID:
// this is how the token Id is packed into its bit-constituents containing position information.
// the following is a diagram to be read top-down in a little endian format
// (so (1) below occupies the first 64 least significant bits, e.g.):
// From the LSB to the MSB:
// ===== 1 time (same for all legs) ==============================================================
//      Property         Size      Offset      Comment
// (0) univ3pool        40bits     0bits      : first 5 bytes representing the Uniswap pool  (first 40 bits; little-endian), plus an incrementing number in the event of a collision
// (1) vegoid           8bits      40bits     : vegoid for the sfpm pool
// (2) tickSpacing      16bits     48bits     : tickSpacing for the univ3pool. Up to 16 bits
// ===== 4 times (one for each leg) ==============================================================
// (3) asset             1bit      0bits      : Specifies the asset (0: token0, 1: token1)
// (4) optionRatio       7bits     1bits      : number of contracts per leg
// (5) isLong            1bit      8bits      : long==1 means liquidity is removed, long==0 -> liquidity is added
// (6) tokenType         1bit      9bits      : put/call: which token is moved when deployed (0 -> token0, 1 -> token1)
// (7) riskPartner       2bits     10bits     : normally its own index. Partner in defined risk position otherwise
// (8) strike           24bits     12bits     : strike price; defined as (tickUpper + tickLower) / 2
// (9) width            12bits     36bits     : width; defined as (tickUpper - tickLower) / tickSpacing
// Total                48bits                : Each leg takes up this many bits
// ===============================================================================================
//
// The bit pattern is therefore, in general:
//
//                        (strike price tick of the 3rd leg)
//                            |             (width of the 2nd leg)
//                            |                   |
// (9)(8)(7)(6)(5)(4)(3)  (9)(8)(7)(6)(5)(4)(3)  (9)(8)(7)(6)(5)(4)(3)   (9)(8)(7)(6)(5)(4)(3)       (2)          (1)           (0)
//  <---- 48 bits ---->    <---- 48 bits ---->    <---- 48 bits ---->     <---- 48 bits ---->   <- 16 bits -> <- 8 bits ->  <- 40 bits ->
//         Leg 4                  Leg 3                  Leg 2                   Leg 1           tickSpacing   vegoid    Uniswap Pool Pattern
//
//  <--- most significant bit                                                                             least significant bit --->
//
// Some rules of how legs behave (we enforce these in a `validate()` function):
//   - a leg is inactive if it's not part of the position. Technically it means that all bits are zero.
//   - a leg is active if it has an optionRatio > 0 since this must always be set for an active leg.
//   - if a leg is active (e.g. leg 1) there can be no gaps in other legs meaning: if leg 1 is active then leg 3 cannot be active if leg 2 is inactive.
//
// Examples:
//  We can think of the bit pattern as an array starting at bit index 0 going to bit index 255 (so 256 total bits)
//  We also refer to the legs via their index, so leg number 2 has leg index 1 (legIndex) (counting from zero), and in general leg number N has leg index N-1.
//  - the underlying strike price of the 2nd leg (leg index = 1) in this option position starts at bit index  (64 + 12 + 48 * (leg index=1))=123
//  - the tokenType of the 4th leg in this option position starts at bit index 64+9+48*3=217
//  - the Uniswap V3 pool id starts at bit index 0 and ends at bit index 63 (and thus takes up 64 bits).
//  - the width of the 3rd leg in this option position starts at bit index 64+36+48*2=196
library TokenIdLibrary {
    /// @notice AND mask to extract all `isLong` bits for each leg from a TokenId.
    uint256 internal constant LONG_MASK =
        0x100_000000000100_000000000100_000000000100_0000000000000000;

    /// @notice AND mask to clear `poolId` from a TokenId.
    uint256 internal constant CLEAR_POOLID_MASK =
        0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF_0000000000000000;

    /// @notice AND mask to clear all bits except for the option ratios of the legs.
    uint256 internal constant OPTION_RATIO_MASK =
        0x0000000000FE_0000000000FE_0000000000FE_0000000000FE_0000000000000000;

    /// @notice AND mask to clear all bits except for the components of the chunk key (strike, width, tokenType) for each leg.
    uint256 internal constant CHUNK_MASK =
        0xFFFFFFFFF200_FFFFFFFFF200_FFFFFFFFF200_FFFFFFFFF200_0000000000000000;

    /// @notice AND mask to cut a sign-extended int256 back to an int24.
    int256 internal constant BITMASK_INT24 = 0xFFFFFF;

    /*//////////////////////////////////////////////////////////////
                                DECODING
    //////////////////////////////////////////////////////////////*/

    /// @notice The full poolId (Uniswap pool identifier + pool pattern) of this option position.
    /// @param self The TokenId to extract `poolId` from
    /// @return The `poolId` (Panoptic's pool fingerprint, contains the whole 64 bit sequence with the tickSpacing) of the Uniswap V3 pool
    function poolId(TokenId self) internal pure returns (uint64) {
        unchecked {
            return uint64(TokenId.unwrap(self));
        }
    }

    /// @notice The vegoid of this option position.
    /// @param self The TokenId to extract `vegoid` from
    /// @return The `vegoid` of the Uniswap V3 pool
    function vegoid(TokenId self) internal pure returns (uint8) {
        unchecked {
            return uint8((TokenId.unwrap(self) >> 40) % 2 ** 8);
        }
    }

    /// @notice The tickSpacing of this option position.
    /// @param self The TokenId to extract `tickSpacing` from
    /// @return The `tickSpacing` of the Uniswap V3 pool
    function tickSpacing(TokenId self) internal pure returns (int24) {
        unchecked {
            return int24(uint24((TokenId.unwrap(self) >> 48) % 2 ** 16));
        }
    }

    /// @notice Get the asset basis for this TokenId.
    /// @dev Which token is the asset - can be token0 (return 0) or token1 (return 1).
    /// @param self The TokenId to extract `asset` from
    /// @param legIndex The leg index of this position (in {0,1,2,3}) to extract `asset` from
    /// @return 0 if asset is token0, 1 if asset is token1
    function asset(TokenId self, uint256 legIndex) internal pure returns (uint256) {
        unchecked {
            return uint256((TokenId.unwrap(self) >> (64 + legIndex * 48)) % 2);
        }
    }

    /// @notice Get the number of contracts multiplier for leg `legIndex`.
    /// @param self The TokenId to extract `optionRatio` at `legIndex` from
    /// @param legIndex The leg index of this position (in {0,1,2,3})
    /// @return The number of contracts multiplier for leg `legIndex`
    function optionRatio(TokenId self, uint256 legIndex) internal pure returns (uint256) {
        unchecked {
            return uint256((TokenId.unwrap(self) >> (64 + legIndex * 48 + 1)) % 128);
        }
    }

    /// @notice Return 1 if the nth leg (leg index `legIndex`) is a long position.
    /// @param self The TokenId to extract `isLong` at `legIndex` from
    /// @param legIndex The leg index of this position (in {0,1,2,3})
    /// @return 1 if long; 0 if not long
    function isLong(TokenId self, uint256 legIndex) internal pure returns (uint256) {
        unchecked {
            return uint256((TokenId.unwrap(self) >> (64 + legIndex * 48 + 8)) % 2);
        }
    }

    /// @notice Get the type of token moved for a given leg (implies a call or put). Either Token0 or Token1.
    /// @param self The TokenId to extract `tokenType` at `legIndex` from
    /// @param legIndex The leg index of this position (in {0,1,2,3})
    /// @return 1 if the token moved is token1 or 0 if the token moved is token0
    function tokenType(TokenId self, uint256 legIndex) internal pure returns (uint256) {
        unchecked {
            return uint256((TokenId.unwrap(self) >> (64 + legIndex * 48 + 9)) % 2);
        }
    }

    /// @notice Get the associated risk partner of the leg index (generally another leg index in the position if enabled or the same leg index if no partner).
    /// @param self The TokenId to extract `riskPartner` at `legIndex` from
    /// @param legIndex The leg index of this position (in {0,1,2,3})
    /// @return The leg index of `legIndex`'s risk partner
    function riskPartner(TokenId self, uint256 legIndex) internal pure returns (uint256) {
        unchecked {
            return uint256((TokenId.unwrap(self) >> (64 + legIndex * 48 + 10)) % 4);
        }
    }

    /// @notice Get the strike price tick of the nth leg (with index `legIndex`).
    /// @param self The TokenId to extract `strike` at `legIndex` from
    /// @param legIndex The leg index of this position (in {0,1,2,3})
    /// @return The strike price tick of the leg
    function strike(TokenId self, uint256 legIndex) internal pure returns (int24) {
        unchecked {
            return int24(int256(TokenId.unwrap(self) >> (64 + legIndex * 48 + 12)));
        }
    }

    /// @notice Get the width (distance between upper and lower ticks) of the nth leg (index `legIndex`).
    /// @dev The width is always positive; it is returned as an int24 for internal consistency with strike operations.
    /// @param self The TokenId to extract `width` at `legIndex` from
    /// @param legIndex The leg index of this position (in {0,1,2,3})
    /// @return The width of the position
    function width(TokenId self, uint256 legIndex) internal pure returns (int24) {
        unchecked {
            return int24(int256((TokenId.unwrap(self) >> (64 + legIndex * 48 + 36)) % 4096));
        } // "% 4096" = take last (2 ** 12 = 4096) 12 bits
    }

    /*//////////////////////////////////////////////////////////////
                                ENCODING
    //////////////////////////////////////////////////////////////*/

    /// @notice Add the Uniswap pool identifier corresponding to this option position (contains the entropy and tickSpacing).
    /// @param self The TokenId to add `_poolId` to
    /// @param _poolId The PoolID to add to `self`
    /// @return `self` with `_poolId` added to the PoolID slot
    function addPoolId(TokenId self, uint64 _poolId) internal pure returns (TokenId) {
        unchecked {
            return TokenId.wrap(TokenId.unwrap(self) + _poolId);
        }
    }

    /// @notice Add the `tickSpacing` to the PoolID for `self`.
    /// @param self The TokenId to add `_tickSpacing` to
    /// @param _tickSpacing The tickSpacing to add to `self`
    /// @return `self` with `_tickSpacing` added to the TickSpacing slot in the PoolID
    function addTickSpacing(TokenId self, int24 _tickSpacing) internal pure returns (TokenId) {
        unchecked {
            return TokenId.wrap(TokenId.unwrap(self) + (uint256(uint24(_tickSpacing)) << 48));
        }
    }

    /// @notice Add the asset basis for this position.
    /// @param self The TokenId to add `_asset` to
    /// @param _asset The asset to add to the Asset slot in `self` for `legIndex`
    /// @param legIndex The leg index of this position (in {0,1,2,3})
    /// @return `self` with `_asset` added to the Asset slot
    function addAsset(
        TokenId self,
        uint256 _asset,
        uint256 legIndex
    ) internal pure returns (TokenId) {
        unchecked {
            return
                TokenId.wrap(TokenId.unwrap(self) + (uint256(_asset % 2) << (64 + legIndex * 48)));
        }
    }

    /// @notice Add the number of contracts multiplier to leg index `legIndex`.
    /// @param self The TokenId to add `_optionRatio` to
    /// @param _optionRatio The number of contracts multiplier to add to the OptionRatio slot in `self` for LegIndex
    /// @param legIndex The leg index of the position (in {0,1,2,3})
    /// @return `self` with `_optionRatio` added to the OptionRatio slot for `legIndex`
    function addOptionRatio(
        TokenId self,
        uint256 _optionRatio,
        uint256 legIndex
    ) internal pure returns (TokenId) {
        unchecked {
            return
                TokenId.wrap(
                    TokenId.unwrap(self) + (uint256(_optionRatio % 128) << (64 + legIndex * 48 + 1))
                );
        }
    }

    /// @notice Add "isLong" parameter indicating whether a leg is long (isLong=1) or short (isLong=0).
    /// @param self The TokenId to add `_isLong` to
    /// @param _isLong The isLong parameter to add to the IsLong slot in `self` for `legIndex`
    /// @param legIndex The leg index of this position (in {0,1,2,3})
    /// @return `self` with `_isLong` added to the IsLong slot for `legIndex`
    function addIsLong(
        TokenId self,
        uint256 _isLong,
        uint256 legIndex
    ) internal pure returns (TokenId) {
        unchecked {
            return TokenId.wrap(TokenId.unwrap(self) + ((_isLong % 2) << (64 + legIndex * 48 + 8)));
        }
    }

    /// @notice Add the type of token moved for a given leg (implies a call or put). Either Token0 or Token1.
    /// @param self The TokenId to add `_tokenType` to
    /// @param _tokenType The tokenType to add to the TokenType slot in `self` for `legIndex`
    /// @param legIndex The leg index of this position (in {0,1,2,3})
    /// @return `self` with `_tokenType` added to the TokenType slot for `legIndex`
    function addTokenType(
        TokenId self,
        uint256 _tokenType,
        uint256 legIndex
    ) internal pure returns (TokenId) {
        unchecked {
            return
                TokenId.wrap(
                    TokenId.unwrap(self) + (uint256(_tokenType % 2) << (64 + legIndex * 48 + 9))
                );
        }
    }

    /// @notice Add the associated risk partner of the leg index.
    /// @param self The TokenId to add `_riskPartner` to
    /// @param _riskPartner The riskPartner to add to the RiskPartner slot in `self` for `legIndex`
    /// @param legIndex The leg index of this position (in {0,1,2,3})
    /// @return `self` with `_riskPartner` added to the RiskPartner slot for `legIndex`
    function addRiskPartner(
        TokenId self,
        uint256 _riskPartner,
        uint256 legIndex
    ) internal pure returns (TokenId) {
        unchecked {
            return
                TokenId.wrap(
                    TokenId.unwrap(self) + (uint256(_riskPartner % 4) << (64 + legIndex * 48 + 10))
                );
        }
    }

    /// @notice Add the strike price tick of the nth leg (index `legIndex`).
    /// @param self The TokenId to add `_strike` to
    /// @param _strike The strike price tick to add to the Strike slot in `self` for `legIndex`
    /// @param legIndex The leg index of this position (in {0,1,2,3})
    /// @return `self` with `_strike` added to the Strike slot for `legIndex`
    function addStrike(
        TokenId self,
        int24 _strike,
        uint256 legIndex
    ) internal pure returns (TokenId) {
        unchecked {
            return
                TokenId.wrap(
                    TokenId.unwrap(self) +
                        uint256((int256(_strike) & BITMASK_INT24) << (64 + legIndex * 48 + 12))
                );
        }
    }

    /// @notice Add the width of the nth leg (index `legIndex`).
    /// @param self The TokenId to add `_width` to
    /// @param _width The width to add to the Width slot in `self` for `legIndex`
    /// @param legIndex The leg index of this position (in {0,1,2,3})
    /// @return `self` with `_width` added to the Width slot for `legIndex`
    function addWidth(
        TokenId self,
        int24 _width,
        uint256 legIndex
    ) internal pure returns (TokenId) {
        // % 4096 -> take 12 bits from the incoming 24 bits (there's no uint12)
        unchecked {
            return
                TokenId.wrap(
                    TokenId.unwrap(self) +
                        (uint256(uint24(_width) % 4096) << (64 + legIndex * 48 + 36))
                );
        }
    }

    /// @notice Add a leg to a TokenId.
    /// @param self The tokenId in the SFPM representing an option position
    /// @param legIndex The leg index of this position (in {0,1,2,3}) to add
    /// @param _optionRatio The relative size of the leg
    /// @param _asset The asset of the leg
    /// @param _isLong Whether the leg is long
    /// @param _tokenType The type of token moved for the leg
    /// @param _riskPartner The associated risk partner of the leg
    /// @param _strike The strike price tick of the leg
    /// @param _width The width of the leg
    /// @return tokenId The tokenId with the leg added
    function addLeg(
        TokenId self,
        uint256 legIndex,
        uint256 _optionRatio,
        uint256 _asset,
        uint256 _isLong,
        uint256 _tokenType,
        uint256 _riskPartner,
        int24 _strike,
        int24 _width
    ) internal pure returns (TokenId tokenId) {
        tokenId = addOptionRatio(self, _optionRatio, legIndex);
        tokenId = addAsset(tokenId, _asset, legIndex);
        tokenId = addIsLong(tokenId, _isLong, legIndex);
        tokenId = addTokenType(tokenId, _tokenType, legIndex);
        tokenId = addRiskPartner(tokenId, _riskPartner, legIndex);
        tokenId = addStrike(tokenId, _strike, legIndex);
        tokenId = addWidth(tokenId, _width, legIndex);
    }

    /*//////////////////////////////////////////////////////////////
                                HELPERS
    //////////////////////////////////////////////////////////////*/

    /// @notice Flip all the `isLong` positions in the legs in the `tokenId` option position.
    /// @param self The TokenId to flip isLong for on all active legs
    /// @return tokenId `self` with all `isLong` bits flipped
    function flipToBurnToken(TokenId self) internal pure returns (TokenId) {
        unchecked {
            // NOTE: This is a hack to avoid blowing up the contract size.
            // We need to ensure that only active legs are flipped
            // In order to achieve this, we shift our long bit mask to the right by (4-# active legs)
            // i.e the whole mask is used to flip all legs with 4 legs, but only the first leg is flipped with 1 leg so we shift by 3 legs
            // We also clear the poolId area of the mask to ensure the bits that are shifted right into the area don't flip and cause issues
            return
                TokenId.wrap(
                    TokenId.unwrap(self) ^
                        ((LONG_MASK >> (48 * (4 - self.countLegs()))) & CLEAR_POOLID_MASK)
                );
        }
    }

    /// @notice Count the number of legs (out of a maximum of 4) that are long positions.
    /// @param self The TokenId to count longs for
    /// @return The number of long positions in `self` (in the range {0,...,4})
    function countLongs(TokenId self) internal pure returns (uint256) {
        unchecked {
            return self.isLong(0) + self.isLong(1) + self.isLong(2) + self.isLong(3);
        }
    }

    /// @notice Get the option position's nth leg's (index `legIndex`) tick ranges (lower, upper).
    /// @param self The TokenId to extract the tick range from
    /// @param legIndex The leg index of the position (in {0,1,2,3})
    /// @return legLowerTick The lower tick of the leg/liquidity chunk
    /// @return legUpperTick The upper tick of the leg/liquidity chunk
    function asTicks(
        TokenId self,
        uint256 legIndex
    ) internal pure returns (int24 legLowerTick, int24 legUpperTick) {
        (legLowerTick, legUpperTick) = PanopticMath.getTicks(
            self.strike(legIndex),
            self.width(legIndex),
            self.tickSpacing()
        );
    }

    /// @notice Return the number of active legs in the option position.
    /// @dev ASSUMPTION: For any leg, the option ratio is always > 0 (the leg always has a number of contracts associated with it).
    /// @param self The TokenId to count active legs for
    /// @return numLegs The number of active legs in `self` (in the range {0,...,4})
    function countLegs(TokenId self) internal pure returns (uint256 numLegs) {
        // Strip all bits except for the option ratios
        uint256 optionRatios = (TokenId.unwrap(self) & OPTION_RATIO_MASK) >> 64;

        unchecked {
            // forge-lint: disable-next-line(incorrect-shift)
            while (optionRatios >= (1 << (48 * numLegs))) {
                ++numLegs;
            }
        }
    }

    /// @notice Clear a leg in an option position at `legIndex`.
    /// @dev NOTE: it's important that the caller fills in the leg details after.
    //  - optionRatio is zeroed
    //  - asset is zeroed
    //  - width is zeroed
    //  - strike is zeroed
    //  - tokenType is zeroed
    //  - isLong is zeroed
    //  - riskPartner is zeroed
    /// @param self The TokenId to clear the leg from
    /// @param legIndex The leg index to reset, in {0,1,2,3}
    /// @return `self` with the `legIndex`th leg zeroed
    function clearLeg(TokenId self, uint256 legIndex) internal pure returns (TokenId) {
        if (legIndex == 0)
            return
                TokenId.wrap(
                    TokenId.unwrap(self) &
                        0xFFFFFFFFFFFF_FFFFFFFFFFFF_FFFFFFFFFFFF_000000000000_FFFFFFFFFFFFFFFF
                );
        if (legIndex == 1)
            return
                TokenId.wrap(
                    TokenId.unwrap(self) &
                        0xFFFFFFFFFFFF_FFFFFFFFFFFF_000000000000_FFFFFFFFFFFF_FFFFFFFFFFFFFFFF
                );
        if (legIndex == 2)
            return
                TokenId.wrap(
                    TokenId.unwrap(self) &
                        0xFFFFFFFFFFFF_000000000000_FFFFFFFFFFFF_FFFFFFFFFFFF_FFFFFFFFFFFFFFFF
                );
        if (legIndex == 3)
            return
                TokenId.wrap(
                    TokenId.unwrap(self) &
                        0x000000000000_FFFFFFFFFFFF_FFFFFFFFFFFF_FFFFFFFFFFFF_FFFFFFFFFFFFFFFF
                );

        return self;
    }

    /*//////////////////////////////////////////////////////////////
                               VALIDATION
    //////////////////////////////////////////////////////////////*/

    /// @notice Checks if a TokenId is valid and reverts with an error reflecting the incorrect parameter for invalid positions.
    /// @param self The TokenId to validate
    function validate(TokenId self) internal pure {
        if (self.optionRatio(0) == 0) revert Errors.InvalidTokenIdParameter(1);

        // loop through the 4 (possible) legs in the tokenId `self`
        unchecked {
            // extract strike, width, and tokenType
            uint256 chunkData = (TokenId.unwrap(self) & CHUNK_MASK) >> 64;
            for (uint256 i = 0; i < 4; ++i) {
                if (self.optionRatio(i) == 0) {
                    // final leg in this position identified;
                    // make sure any leg above this are zero as well
                    // (we don't allow gaps eg having legs 1 and 4 active without 2 and 3 is not allowed)
                    if ((TokenId.unwrap(self) >> (64 + 48 * i)) != 0)
                        revert Errors.InvalidTokenIdParameter(1);

                    break; // we are done iterating over potential legs
                }

                // prevent legs touching the same chunks - all chunks in the position must be discrete
                uint256 numLegs = self.countLegs();
                for (uint256 j = i + 1; j < numLegs; ++j) {
                    if (uint48(chunkData >> (48 * i)) == uint48(chunkData >> (48 * j))) {
                        revert Errors.InvalidTokenIdParameter(6);
                    }
                }

                // Strike cannot be MIN_TICK or MAX_TICK
                if (
                    (self.strike(i) == Constants.MIN_POOL_TICK) ||
                    (self.strike(i) == Constants.MAX_POOL_TICK)
                ) revert Errors.InvalidTokenIdParameter(4);

                // In the following, we check whether the risk partner of this leg is itself
                // or another leg in this position.
                uint256 riskPartnerIndex = self.riskPartner(i);
                if (riskPartnerIndex != i) {
                    // Ensures that risk partners are mutual
                    if (self.riskPartner(riskPartnerIndex) != i)
                        revert Errors.InvalidTokenIdParameter(3);
                }
            }
        }
    }

    /// @notice Check whether a position `self` contains at least one exercisable long leg.
    /// @dev A leg is considered exercisable if it is:
    ///      - long (isLong == 1), and
    ///      - not a loan/credit leg (width != 0).
    /// @dev This function does NOT check moneyness or price ranges.
    /// @return hasExercisableLong Returns 1 if such a leg exists, 0 otherwise.
    function validateIsExercisable(TokenId self) internal pure returns (uint256) {
        unchecked {
            uint256 numLegs = self.countLegs();
            for (uint256 i = 0; i < numLegs; ++i) {
                if (self.isLong(i) == 1 && self.width(i) != 0) return 1; // validated
            }
        }

        // Fail if position has no legs that is far-out-of-the-money
        return 0;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {PoolKey} from "./PoolKey.sol";

type PoolId is bytes32;

/// @notice Library for computing the ID of a pool
library PoolIdLibrary {
    /// @notice Returns value equal to keccak256(abi.encode(poolKey))
    function toId(PoolKey memory poolKey) internal pure returns (PoolId poolId) {
        assembly ("memory-safe") {
            // 0xa0 represents the total size of the poolKey struct (5 slots of 32 bytes)
            poolId := keccak256(poolKey, 0xa0)
        }
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity >=0.5.0;

/// @title The interface for the Uniswap V3 Factory
/// @notice The Uniswap V3 Factory facilitates creation of Uniswap V3 pools and control over the protocol fees
interface IUniswapV3Factory {
    /// @notice Emitted when the owner of the factory is changed
    /// @param oldOwner The owner before the owner was changed
    /// @param newOwner The owner after the owner was changed
    event OwnerChanged(address indexed oldOwner, address indexed newOwner);

    /// @notice Emitted when a pool is created
    /// @param token0 The first token of the pool by address sort order
    /// @param token1 The second token of the pool by address sort order
    /// @param fee The fee collected upon every swap in the pool, denominated in hundredths of a bip
    /// @param tickSpacing The minimum number of ticks between initialized ticks
    /// @param pool The address of the created pool
    event PoolCreated(
        address indexed token0,
        address indexed token1,
        uint24 indexed fee,
        int24 tickSpacing,
        address pool
    );

    /// @notice Emitted when a new fee amount is enabled for pool creation via the factory
    /// @param fee The enabled fee, denominated in hundredths of a bip
    /// @param tickSpacing The minimum number of ticks between initialized ticks for pools created with the given fee
    event FeeAmountEnabled(uint24 indexed fee, int24 indexed tickSpacing);

    /// @notice Returns the current owner of the factory
    /// @dev Can be changed by the current owner via setOwner
    /// @return The address of the factory owner
    function owner() external view returns (address);

    /// @notice Returns the tick spacing for a given fee amount, if enabled, or 0 if not enabled
    /// @dev A fee amount can never be removed, so this value should be hard coded or cached in the calling context
    /// @param fee The enabled fee, denominated in hundredths of a bip. Returns 0 in case of unenabled fee
    /// @return The tick spacing
    function feeAmountTickSpacing(uint24 fee) external view returns (int24);

    /// @notice Returns the pool address for a given pair of tokens and a fee, or address 0 if it does not exist
    /// @dev tokenA and tokenB may be passed in either token0/token1 or token1/token0 order
    /// @param tokenA The contract address of either token0 or token1
    /// @param tokenB The contract address of the other token
    /// @param fee The fee collected upon every swap in the pool, denominated in hundredths of a bip
    /// @return pool The pool address
    function getPool(
        address tokenA,
        address tokenB,
        uint24 fee
    ) external view returns (address pool);

    /// @notice Creates a pool for the given two tokens and fee
    /// @param tokenA One of the two tokens in the desired pool
    /// @param tokenB The other of the two tokens in the desired pool
    /// @param fee The desired fee for the pool
    /// @dev tokenA and tokenB may be passed in either order: token0/token1 or token1/token0. tickSpacing is retrieved
    /// from the fee. The call will revert if the pool already exists, the fee is invalid, or the token arguments
    /// are invalid.
    /// @return pool The address of the newly created pool
    function createPool(
        address tokenA,
        address tokenB,
        uint24 fee
    ) external returns (address pool);

    /// @notice Updates the owner of the factory
    /// @dev Must be called by the current owner
    /// @param _owner The new owner of the factory
    function setOwner(address _owner) external;

    /// @notice Enables a fee amount with the given tickSpacing
    /// @dev Fee amounts may never be removed once enabled
    /// @param fee The fee amount to enable, denominated in hundredths of a bip (i.e. 1e-6)
    /// @param tickSpacing The spacing between ticks to be enforced for all pools created with the given fee amount
    function enableFeeAmount(uint24 fee, int24 tickSpacing) external;
}

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

/// @title Efficient Keccak256 Library
/// @notice Provides gas-efficient keccak256 hashing using inline assembly
library EfficientHash {
    /// @notice Efficiently compute keccak256 hash for position key (address, address, uint256, int24, int24)
    /// @param univ3pool The Uniswap V3 pool address (20 bytes)
    /// @param owner The owner address (20 bytes)
    /// @param tokenType The token type (32 bytes)
    /// @param tickLower The lower tick (3 bytes when packed)
    /// @param tickUpper The upper tick (3 bytes when packed)
    /// @return hash The keccak256 hash of the packed data
    function efficientKeccak256(
        address univ3pool,
        address owner,
        uint256 tokenType,
        int24 tickLower,
        int24 tickUpper
    ) internal pure returns (bytes32 hash) {
        assembly {
            let freeMemPtr := mload(0x40)
            // Pack: 20 + 20 + 32 + 3 + 3 = 78 bytes (0x4e)
            mstore(freeMemPtr, shl(96, univ3pool)) // address at byte 0
            mstore(add(freeMemPtr, 0x14), shl(96, owner)) // address at byte 20
            mstore(add(freeMemPtr, 0x28), tokenType) // uint256 at byte 40
            mstore(add(freeMemPtr, 0x48), shl(232, and(tickLower, 0xFFFFFF))) // int24 at byte 72
            mstore(add(freeMemPtr, 0x4b), shl(232, and(tickUpper, 0xFFFFFF))) // int24 at byte 75

            hash := keccak256(freeMemPtr, 0x4e)
        }
    }

    /// @notice Efficiently compute keccak256 hash for chunk key (int24, int24, uint256)
    /// @param strike The strike tick (3 bytes when packed)
    /// @param width The width (3 bytes when packed)
    /// @param tokenType The token type (32 bytes)
    /// @return hash The keccak256 hash of the packed data
    function efficientKeccak256(
        int24 strike,
        int24 width,
        uint256 tokenType
    ) internal pure returns (bytes32 hash) {
        assembly {
            let freeMemPtr := mload(0x40)
            // Pack: 3 + 3 + 32 = 38 bytes (0x26)
            mstore(freeMemPtr, shl(232, and(strike, 0xFFFFFF))) // int24 at byte 0
            mstore(add(freeMemPtr, 0x03), shl(232, and(width, 0xFFFFFF))) // int24 at byte 3
            mstore(add(freeMemPtr, 0x06), tokenType) // uint256 at byte 6

            hash := keccak256(freeMemPtr, 0x26)
        }
    }

    /// @notice Efficiently compute keccak256 hash for a uint256 array
    /// @param data The uint256 array to hash
    /// @return hash The keccak256 hash of the packed data
    function efficientKeccak256(uint256[] memory data) internal pure returns (bytes32 hash) {
        assembly {
            // data layout in memory: [length][item0][item1]...
            // Skip the length field (32 bytes) and hash the rest
            let dataLength := mload(data)
            let dataStart := add(data, 0x20)
            let bytesToHash := mul(dataLength, 0x20)

            hash := keccak256(dataStart, bytesToHash)
        }
    }

    /// @notice Efficiently compute keccak256 hash for bytes memory
    /// @param data The bytes to hash
    /// @return hash The keccak256 hash of the data
    function efficientKeccak256(bytes memory data) internal pure returns (bytes32 hash) {
        assembly {
            // bytes layout in memory: [length][data...]
            let dataLength := mload(data)
            let dataStart := add(data, 0x20)

            hash := keccak256(dataStart, dataLength)
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {SafeCast} from "../libraries/SafeCast.sol";

/// @dev Two `int128` values packed into a single `int256` where the upper 128 bits represent the amount0
/// and the lower 128 bits represent the amount1.
type BalanceDelta is int256;

using {add as +, sub as -, eq as ==, neq as !=} for BalanceDelta global;
using BalanceDeltaLibrary for BalanceDelta global;
using SafeCast for int256;

function toBalanceDelta(int128 _amount0, int128 _amount1) pure returns (BalanceDelta balanceDelta) {
    assembly ("memory-safe") {
        balanceDelta := or(shl(128, _amount0), and(sub(shl(128, 1), 1), _amount1))
    }
}

function add(BalanceDelta a, BalanceDelta b) pure returns (BalanceDelta) {
    int256 res0;
    int256 res1;
    assembly ("memory-safe") {
        let a0 := sar(128, a)
        let a1 := signextend(15, a)
        let b0 := sar(128, b)
        let b1 := signextend(15, b)
        res0 := add(a0, b0)
        res1 := add(a1, b1)
    }
    return toBalanceDelta(res0.toInt128(), res1.toInt128());
}

function sub(BalanceDelta a, BalanceDelta b) pure returns (BalanceDelta) {
    int256 res0;
    int256 res1;
    assembly ("memory-safe") {
        let a0 := sar(128, a)
        let a1 := signextend(15, a)
        let b0 := sar(128, b)
        let b1 := signextend(15, b)
        res0 := sub(a0, b0)
        res1 := sub(a1, b1)
    }
    return toBalanceDelta(res0.toInt128(), res1.toInt128());
}

function eq(BalanceDelta a, BalanceDelta b) pure returns (bool) {
    return BalanceDelta.unwrap(a) == BalanceDelta.unwrap(b);
}

function neq(BalanceDelta a, BalanceDelta b) pure returns (bool) {
    return BalanceDelta.unwrap(a) != BalanceDelta.unwrap(b);
}

/// @notice Library for getting the amount0 and amount1 deltas from the BalanceDelta type
library BalanceDeltaLibrary {
    /// @notice A BalanceDelta of 0
    BalanceDelta public constant ZERO_DELTA = BalanceDelta.wrap(0);

    function amount0(BalanceDelta balanceDelta) internal pure returns (int128 _amount0) {
        assembly ("memory-safe") {
            _amount0 := sar(128, balanceDelta)
        }
    }

    function amount1(BalanceDelta balanceDelta) internal pure returns (int128 _amount1) {
        assembly ("memory-safe") {
            _amount1 := signextend(15, balanceDelta)
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {Currency} from "../types/Currency.sol";
import {PoolKey} from "../types/PoolKey.sol";
import {IHooks} from "./IHooks.sol";
import {IERC6909Claims} from "./external/IERC6909Claims.sol";
import {IProtocolFees} from "./IProtocolFees.sol";
import {BalanceDelta} from "../types/BalanceDelta.sol";
import {PoolId} from "../types/PoolId.sol";
import {IExtsload} from "./IExtsload.sol";
import {IExttload} from "./IExttload.sol";

/// @notice Interface for the PoolManager
interface IPoolManager is IProtocolFees, IERC6909Claims, IExtsload, IExttload {
    /// @notice Thrown when a currency is not netted out after the contract is unlocked
    error CurrencyNotSettled();

    /// @notice Thrown when trying to interact with a non-initialized pool
    error PoolNotInitialized();

    /// @notice Thrown when unlock is called, but the contract is already unlocked
    error AlreadyUnlocked();

    /// @notice Thrown when a function is called that requires the contract to be unlocked, but it is not
    error ManagerLocked();

    /// @notice Pools are limited to type(int16).max tickSpacing in #initialize, to prevent overflow
    error TickSpacingTooLarge(int24 tickSpacing);

    /// @notice Pools must have a positive non-zero tickSpacing passed to #initialize
    error TickSpacingTooSmall(int24 tickSpacing);

    /// @notice PoolKey must have currencies where address(currency0) < address(currency1)
    error CurrenciesOutOfOrderOrEqual(address currency0, address currency1);

    /// @notice Thrown when a call to updateDynamicLPFee is made by an address that is not the hook,
    /// or on a pool that does not have a dynamic swap fee.
    error UnauthorizedDynamicLPFeeUpdate();

    /// @notice Thrown when trying to swap amount of 0
    error SwapAmountCannotBeZero();

    ///@notice Thrown when native currency is passed to a non native settlement
    error NonzeroNativeValue();

    /// @notice Thrown when `clear` is called with an amount that is not exactly equal to the open currency delta.
    error MustClearExactPositiveDelta();

    /// @notice Emitted when a new pool is initialized
    /// @param id The abi encoded hash of the pool key struct for the new pool
    /// @param currency0 The first currency of the pool by address sort order
    /// @param currency1 The second currency of the pool by address sort order
    /// @param fee The fee collected upon every swap in the pool, denominated in hundredths of a bip
    /// @param tickSpacing The minimum number of ticks between initialized ticks
    /// @param hooks The hooks contract address for the pool, or address(0) if none
    /// @param sqrtPriceX96 The price of the pool on initialization
    /// @param tick The initial tick of the pool corresponding to the initialized price
    event Initialize(
        PoolId indexed id,
        Currency indexed currency0,
        Currency indexed currency1,
        uint24 fee,
        int24 tickSpacing,
        IHooks hooks,
        uint160 sqrtPriceX96,
        int24 tick
    );

    /// @notice Emitted when a liquidity position is modified
    /// @param id The abi encoded hash of the pool key struct for the pool that was modified
    /// @param sender The address that modified the pool
    /// @param tickLower The lower tick of the position
    /// @param tickUpper The upper tick of the position
    /// @param liquidityDelta The amount of liquidity that was added or removed
    /// @param salt The extra data to make positions unique
    event ModifyLiquidity(
        PoolId indexed id, address indexed sender, int24 tickLower, int24 tickUpper, int256 liquidityDelta, bytes32 salt
    );

    /// @notice Emitted for swaps between currency0 and currency1
    /// @param id The abi encoded hash of the pool key struct for the pool that was modified
    /// @param sender The address that initiated the swap call, and that received the callback
    /// @param amount0 The delta of the currency0 balance of the pool
    /// @param amount1 The delta of the currency1 balance of the pool
    /// @param sqrtPriceX96 The sqrt(price) of the pool after the swap, as a Q64.96
    /// @param liquidity The liquidity of the pool after the swap
    /// @param tick The log base 1.0001 of the price of the pool after the swap
    /// @param fee The swap fee in hundredths of a bip
    event Swap(
        PoolId indexed id,
        address indexed sender,
        int128 amount0,
        int128 amount1,
        uint160 sqrtPriceX96,
        uint128 liquidity,
        int24 tick,
        uint24 fee
    );

    /// @notice Emitted for donations
    /// @param id The abi encoded hash of the pool key struct for the pool that was donated to
    /// @param sender The address that initiated the donate call
    /// @param amount0 The amount donated in currency0
    /// @param amount1 The amount donated in currency1
    event Donate(PoolId indexed id, address indexed sender, uint256 amount0, uint256 amount1);

    /// @notice All interactions on the contract that account deltas require unlocking. A caller that calls `unlock` must implement
    /// `IUnlockCallback(msg.sender).unlockCallback(data)`, where they interact with the remaining functions on this contract.
    /// @dev The only functions callable without an unlocking are `initialize` and `updateDynamicLPFee`
    /// @param data Any data to pass to the callback, via `IUnlockCallback(msg.sender).unlockCallback(data)`
    /// @return The data returned by the call to `IUnlockCallback(msg.sender).unlockCallback(data)`
    function unlock(bytes calldata data) external returns (bytes memory);

    /// @notice Initialize the state for a given pool ID
    /// @dev A swap fee totaling MAX_SWAP_FEE (100%) makes exact output swaps impossible since the input is entirely consumed by the fee
    /// @param key The pool key for the pool to initialize
    /// @param sqrtPriceX96 The initial square root price
    /// @return tick The initial tick of the pool
    function initialize(PoolKey memory key, uint160 sqrtPriceX96) external returns (int24 tick);

    struct ModifyLiquidityParams {
        // the lower and upper tick of the position
        int24 tickLower;
        int24 tickUpper;
        // how to modify the liquidity
        int256 liquidityDelta;
        // a value to set if you want unique liquidity positions at the same range
        bytes32 salt;
    }

    /// @notice Modify the liquidity for the given pool
    /// @dev Poke by calling with a zero liquidityDelta
    /// @param key The pool to modify liquidity in
    /// @param params The parameters for modifying the liquidity
    /// @param hookData The data to pass through to the add/removeLiquidity hooks
    /// @return callerDelta The balance delta of the caller of modifyLiquidity. This is the total of both principal, fee deltas, and hook deltas if applicable
    /// @return feesAccrued The balance delta of the fees generated in the liquidity range. Returned for informational purposes
    /// @dev Note that feesAccrued can be artificially inflated by a malicious actor and integrators should be careful using the value
    /// For pools with a single liquidity position, actors can donate to themselves to inflate feeGrowthGlobal (and consequently feesAccrued)
    /// atomically donating and collecting fees in the same unlockCallback may make the inflated value more extreme
    function modifyLiquidity(PoolKey memory key, ModifyLiquidityParams memory params, bytes calldata hookData)
        external
        returns (BalanceDelta callerDelta, BalanceDelta feesAccrued);

    struct SwapParams {
        /// Whether to swap token0 for token1 or vice versa
        bool zeroForOne;
        /// The desired input amount if negative (exactIn), or the desired output amount if positive (exactOut)
        int256 amountSpecified;
        /// The sqrt price at which, if reached, the swap will stop executing
        uint160 sqrtPriceLimitX96;
    }

    /// @notice Swap against the given pool
    /// @param key The pool to swap in
    /// @param params The parameters for swapping
    /// @param hookData The data to pass through to the swap hooks
    /// @return swapDelta The balance delta of the address swapping
    /// @dev Swapping on low liquidity pools may cause unexpected swap amounts when liquidity available is less than amountSpecified.
    /// Additionally note that if interacting with hooks that have the BEFORE_SWAP_RETURNS_DELTA_FLAG or AFTER_SWAP_RETURNS_DELTA_FLAG
    /// the hook may alter the swap input/output. Integrators should perform checks on the returned swapDelta.
    function swap(PoolKey memory key, SwapParams memory params, bytes calldata hookData)
        external
        returns (BalanceDelta swapDelta);

    /// @notice Donate the given currency amounts to the in-range liquidity providers of a pool
    /// @dev Calls to donate can be frontrun adding just-in-time liquidity, with the aim of receiving a portion donated funds.
    /// Donors should keep this in mind when designing donation mechanisms.
    /// @dev This function donates to in-range LPs at slot0.tick. In certain edge-cases of the swap algorithm, the `sqrtPrice` of
    /// a pool can be at the lower boundary of tick `n`, but the `slot0.tick` of the pool is already `n - 1`. In this case a call to
    /// `donate` would donate to tick `n - 1` (slot0.tick) not tick `n` (getTickAtSqrtPrice(slot0.sqrtPriceX96)).
    /// Read the comments in `Pool.swap()` for more information about this.
    /// @param key The key of the pool to donate to
    /// @param amount0 The amount of currency0 to donate
    /// @param amount1 The amount of currency1 to donate
    /// @param hookData The data to pass through to the donate hooks
    /// @return BalanceDelta The delta of the caller after the donate
    function donate(PoolKey memory key, uint256 amount0, uint256 amount1, bytes calldata hookData)
        external
        returns (BalanceDelta);

    /// @notice Writes the current ERC20 balance of the specified currency to transient storage
    /// This is used to checkpoint balances for the manager and derive deltas for the caller.
    /// @dev This MUST be called before any ERC20 tokens are sent into the contract, but can be skipped
    /// for native tokens because the amount to settle is determined by the sent value.
    /// However, if an ERC20 token has been synced and not settled, and the caller instead wants to settle
    /// native funds, this function can be called with the native currency to then be able to settle the native currency
    function sync(Currency currency) external;

    /// @notice Called by the user to net out some value owed to the user
    /// @dev Will revert if the requested amount is not available, consider using `mint` instead
    /// @dev Can also be used as a mechanism for free flash loans
    /// @param currency The currency to withdraw from the pool manager
    /// @param to The address to withdraw to
    /// @param amount The amount of currency to withdraw
    function take(Currency currency, address to, uint256 amount) external;

    /// @notice Called by the user to pay what is owed
    /// @return paid The amount of currency settled
    function settle() external payable returns (uint256 paid);

    /// @notice Called by the user to pay on behalf of another address
    /// @param recipient The address to credit for the payment
    /// @return paid The amount of currency settled
    function settleFor(address recipient) external payable returns (uint256 paid);

    /// @notice WARNING - Any currency that is cleared, will be non-retrievable, and locked in the contract permanently.
    /// A call to clear will zero out a positive balance WITHOUT a corresponding transfer.
    /// @dev This could be used to clear a balance that is considered dust.
    /// Additionally, the amount must be the exact positive balance. This is to enforce that the caller is aware of the amount being cleared.
    function clear(Currency currency, uint256 amount) external;

    /// @notice Called by the user to move value into ERC6909 balance
    /// @param to The address to mint the tokens to
    /// @param id The currency address to mint to ERC6909s, as a uint256
    /// @param amount The amount of currency to mint
    /// @dev The id is converted to a uint160 to correspond to a currency address
    /// If the upper 12 bytes are not 0, they will be 0-ed out
    function mint(address to, uint256 id, uint256 amount) external;

    /// @notice Called by the user to move value from ERC6909 balance
    /// @param from The address to burn the tokens from
    /// @param id The currency address to burn from ERC6909s, as a uint256
    /// @param amount The amount of currency to burn
    /// @dev The id is converted to a uint160 to correspond to a currency address
    /// If the upper 12 bytes are not 0, they will be 0-ed out
    function burn(address from, uint256 id, uint256 amount) external;

    /// @notice Updates the pools lp fees for the a pool that has enabled dynamic lp fees.
    /// @dev A swap fee totaling MAX_SWAP_FEE (100%) makes exact output swaps impossible since the input is entirely consumed by the fee
    /// @param key The key of the pool to update dynamic LP fees for
    /// @param newDynamicLPFee The new dynamic pool LP fee
    function updateDynamicLPFee(PoolKey memory key, uint24 newDynamicLPFee) external;
}

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.24;

/// @title Multicall
/// @notice Enables calling multiple methods in a single call to the contract.
/// @dev Helpful for performing batch operations such as an "emergency exit", or simply creating advanced positions.
/// @author Axicon Labs Limited
abstract contract Multicall {
    /// @notice Performs multiple calls on the inheritor in a single transaction, and returns the data from each call.
    /// @param data The calldata for each call
    /// @return results The data returned by each call
    function multicall(bytes[] calldata data) public payable returns (bytes[] memory results) {
        results = new bytes[](data.length);
        for (uint256 i = 0; i < data.length; ) {
            (bool success, bytes memory result) = address(this).delegatecall(data[i]);

            if (!success) {
                // Bubble up the revert reason
                // The bytes type is ABI encoded as a length-prefixed byte array
                // So we simply need to add 32 to the pointer to get the start of the data
                // And then revert with the size loaded from the first 32 bytes
                // Other solutions will do work to differentiate the revert reasons and provide parenthetical information
                // However, we have chosen to simply replicate the the normal behavior of the call
                // NOTE: memory-safe because it reads from memory already allocated by solidity (the bytes memory result)
                assembly ("memory-safe") {
                    revert(add(result, 32), mload(result))
                }
            }

            results[i] = result;

            unchecked {
                ++i;
            }
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.8.24;

/// @notice Gas optimized reentrancy protection for smart contracts. Leverages Cancun transient storage.
/// @author Solmate (https://github.com/transmissions11/solmate/blob/main/src/utils/TransientReentrancyGuard.sol)
/// @author Modified from Soledge (https://github.com/Vectorized/soledge/blob/main/src/utils/ReentrancyGuard.sol)
abstract contract TransientReentrancyGuard {
    /// Warning: Be careful to avoid collisions with this hand picked slot!
    uint256 private constant REENTRANCY_GUARD_SLOT = 0x1FACE81BADDEADBEEF;


    modifier nonReentrant() virtual {
        bool noReentrancy;

        /// @solidity memory-safe-assembly
        assembly {
            noReentrancy := iszero(tload(REENTRANCY_GUARD_SLOT))

            // Any non-zero value would work, but
            // ADDRESS is cheap and certainly not 0.
            // Wastes a bit of gas doing this before
            // require in the revert path, but we're
            // only optimizing for the happy path here.
            tstore(REENTRANCY_GUARD_SLOT, address())
        }

        require(noReentrancy, "REENTRANCY");

        _;

        /// @solidity memory-safe-assembly
        assembly {
            // Need to set back to zero, as transient
            // storage is only cleared at the end of the
            // tx, not the end of the outermost call frame.
            tstore(REENTRANCY_GUARD_SLOT, 0)
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {CustomRevert} from "./CustomRevert.sol";

/// @title Safe casting methods
/// @notice Contains methods for safely casting between types
library SafeCast {
    using CustomRevert for bytes4;

    error SafeCastOverflow();

    /// @notice Cast a uint256 to a uint160, revert on overflow
    /// @param x The uint256 to be downcasted
    /// @return y The downcasted integer, now type uint160
    function toUint160(uint256 x) internal pure returns (uint160 y) {
        y = uint160(x);
        if (y != x) SafeCastOverflow.selector.revertWith();
    }

    /// @notice Cast a uint256 to a uint128, revert on overflow
    /// @param x The uint256 to be downcasted
    /// @return y The downcasted integer, now type uint128
    function toUint128(uint256 x) internal pure returns (uint128 y) {
        y = uint128(x);
        if (x != y) SafeCastOverflow.selector.revertWith();
    }

    /// @notice Cast a int128 to a uint128, revert on overflow or underflow
    /// @param x The int128 to be casted
    /// @return y The casted integer, now type uint128
    function toUint128(int128 x) internal pure returns (uint128 y) {
        if (x < 0) SafeCastOverflow.selector.revertWith();
        y = uint128(x);
    }

    /// @notice Cast a int256 to a int128, revert on overflow or underflow
    /// @param x The int256 to be downcasted
    /// @return y The downcasted integer, now type int128
    function toInt128(int256 x) internal pure returns (int128 y) {
        y = int128(x);
        if (y != x) SafeCastOverflow.selector.revertWith();
    }

    /// @notice Cast a uint256 to a int256, revert on overflow
    /// @param x The uint256 to be casted
    /// @return y The casted integer, now type int256
    function toInt256(uint256 x) internal pure returns (int256 y) {
        y = int256(x);
        if (y < 0) SafeCastOverflow.selector.revertWith();
    }

    /// @notice Cast a uint256 to a int128, revert on overflow
    /// @param x The uint256 to be downcasted
    /// @return The downcasted integer, now type int128
    function toInt128(uint256 x) internal pure returns (int128) {
        if (x >= 1 << 127) SafeCastOverflow.selector.revertWith();
        return int128(int256(x));
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/// @title Library for reverting with custom errors efficiently
/// @notice Contains functions for reverting with custom errors with different argument types efficiently
/// @dev To use this library, declare `using CustomRevert for bytes4;` and replace `revert CustomError()` with
/// `CustomError.selector.revertWith()`
/// @dev The functions may tamper with the free memory pointer but it is fine since the call context is exited immediately
library CustomRevert {
    /// @dev ERC-7751 error for wrapping bubbled up reverts
    error WrappedError(address target, bytes4 selector, bytes reason, bytes details);

    /// @dev Reverts with the selector of a custom error in the scratch space
    function revertWith(bytes4 selector) internal pure {
        assembly ("memory-safe") {
            mstore(0, selector)
            revert(0, 0x04)
        }
    }

    /// @dev Reverts with a custom error with an address argument in the scratch space
    function revertWith(bytes4 selector, address addr) internal pure {
        assembly ("memory-safe") {
            mstore(0, selector)
            mstore(0x04, and(addr, 0xffffffffffffffffffffffffffffffffffffffff))
            revert(0, 0x24)
        }
    }

    /// @dev Reverts with a custom error with an int24 argument in the scratch space
    function revertWith(bytes4 selector, int24 value) internal pure {
        assembly ("memory-safe") {
            mstore(0, selector)
            mstore(0x04, signextend(2, value))
            revert(0, 0x24)
        }
    }

    /// @dev Reverts with a custom error with a uint160 argument in the scratch space
    function revertWith(bytes4 selector, uint160 value) internal pure {
        assembly ("memory-safe") {
            mstore(0, selector)
            mstore(0x04, and(value, 0xffffffffffffffffffffffffffffffffffffffff))
            revert(0, 0x24)
        }
    }

    /// @dev Reverts with a custom error with two int24 arguments
    function revertWith(bytes4 selector, int24 value1, int24 value2) internal pure {
        assembly ("memory-safe") {
            let fmp := mload(0x40)
            mstore(fmp, selector)
            mstore(add(fmp, 0x04), signextend(2, value1))
            mstore(add(fmp, 0x24), signextend(2, value2))
            revert(fmp, 0x44)
        }
    }

    /// @dev Reverts with a custom error with two uint160 arguments
    function revertWith(bytes4 selector, uint160 value1, uint160 value2) internal pure {
        assembly ("memory-safe") {
            let fmp := mload(0x40)
            mstore(fmp, selector)
            mstore(add(fmp, 0x04), and(value1, 0xffffffffffffffffffffffffffffffffffffffff))
            mstore(add(fmp, 0x24), and(value2, 0xffffffffffffffffffffffffffffffffffffffff))
            revert(fmp, 0x44)
        }
    }

    /// @dev Reverts with a custom error with two address arguments
    function revertWith(bytes4 selector, address value1, address value2) internal pure {
        assembly ("memory-safe") {
            let fmp := mload(0x40)
            mstore(fmp, selector)
            mstore(add(fmp, 0x04), and(value1, 0xffffffffffffffffffffffffffffffffffffffff))
            mstore(add(fmp, 0x24), and(value2, 0xffffffffffffffffffffffffffffffffffffffff))
            revert(fmp, 0x44)
        }
    }

    /// @notice bubble up the revert message returned by a call and revert with a wrapped ERC-7751 error
    /// @dev this method can be vulnerable to revert data bombs
    function bubbleUpAndRevertWith(
        address revertingContract,
        bytes4 revertingFunctionSelector,
        bytes4 additionalContext
    ) internal pure {
        bytes4 wrappedErrorSelector = WrappedError.selector;
        assembly ("memory-safe") {
            // Ensure the size of the revert data is a multiple of 32 bytes
            let encodedDataSize := mul(div(add(returndatasize(), 31), 32), 32)

            let fmp := mload(0x40)

            // Encode wrapped error selector, address, function selector, offset, additional context, size, revert reason
            mstore(fmp, wrappedErrorSelector)
            mstore(add(fmp, 0x04), and(revertingContract, 0xffffffffffffffffffffffffffffffffffffffff))
            mstore(
                add(fmp, 0x24),
                and(revertingFunctionSelector, 0xffffffff00000000000000000000000000000000000000000000000000000000)
            )
            // offset revert reason
            mstore(add(fmp, 0x44), 0x80)
            // offset additional context
            mstore(add(fmp, 0x64), add(0xa0, encodedDataSize))
            // size revert reason
            mstore(add(fmp, 0x84), returndatasize())
            // revert reason
            returndatacopy(add(fmp, 0xa4), 0, returndatasize())
            // size additional context
            mstore(add(fmp, add(0xa4, encodedDataSize)), 0x04)
            // additional context
            mstore(
                add(fmp, add(0xc4, encodedDataSize)),
                and(additionalContext, 0xffffffff00000000000000000000000000000000000000000000000000000000)
            )
            revert(fmp, add(0xe4, encodedDataSize))
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/// @title Minimal ERC20 interface for Uniswap
/// @notice Contains a subset of the full ERC20 interface that is used in Uniswap V3
interface IERC20Minimal {
    /// @notice Returns an account's balance in the token
    /// @param account The account for which to look up the number of tokens it has, i.e. its balance
    /// @return The number of tokens held by the account
    function balanceOf(address account) external view returns (uint256);

    /// @notice Transfers the amount of token from the `msg.sender` to the recipient
    /// @param recipient The account that will receive the amount transferred
    /// @param amount The number of tokens to send from the sender to the recipient
    /// @return Returns true for a successful transfer, false for an unsuccessful transfer
    function transfer(address recipient, uint256 amount) external returns (bool);

    /// @notice Returns the current allowance given to a spender by an owner
    /// @param owner The account of the token owner
    /// @param spender The account of the token spender
    /// @return The current allowance granted by `owner` to `spender`
    function allowance(address owner, address spender) external view returns (uint256);

    /// @notice Sets the allowance of a spender from the `msg.sender` to the value `amount`
    /// @param spender The account which will be allowed to spend a given amount of the owners tokens
    /// @param amount The amount of tokens allowed to be used by `spender`
    /// @return Returns true for a successful approval, false for unsuccessful
    function approve(address spender, uint256 amount) external returns (bool);

    /// @notice Transfers `amount` tokens from `sender` to `recipient` up to the allowance given to the `msg.sender`
    /// @param sender The account from which the transfer will be initiated
    /// @param recipient The recipient of the transfer
    /// @param amount The amount of the transfer
    /// @return Returns true for a successful transfer, false for unsuccessful
    function transferFrom(address sender, address recipient, uint256 amount) external returns (bool);

    /// @notice Event emitted when tokens are transferred from one address to another, either via `#transfer` or `#transferFrom`.
    /// @param from The account from which the tokens were sent, i.e. the balance decreased
    /// @param to The account to which the tokens were sent, i.e. the balance increased
    /// @param value The amount of tokens that were transferred
    event Transfer(address indexed from, address indexed to, uint256 value);

    /// @notice Event emitted when the approval amount for the spender of a given owner's tokens changes.
    /// @param owner The account that approved spending of its tokens
    /// @param spender The account for which the spending allowance was modified
    /// @param value The new allowance from the owner to the spender
    event Approval(address indexed owner, address indexed spender, uint256 value);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {PoolKey} from "../types/PoolKey.sol";
import {BalanceDelta} from "../types/BalanceDelta.sol";
import {IPoolManager} from "./IPoolManager.sol";
import {BeforeSwapDelta} from "../types/BeforeSwapDelta.sol";

/// @notice V4 decides whether to invoke specific hooks by inspecting the least significant bits
/// of the address that the hooks contract is deployed to.
/// For example, a hooks contract deployed to address: 0x0000000000000000000000000000000000002400
/// has the lowest bits '10 0100 0000 0000' which would cause the 'before initialize' and 'after add liquidity' hooks to be used.
/// See the Hooks library for the full spec.
/// @dev Should only be callable by the v4 PoolManager.
interface IHooks {
    /// @notice The hook called before the state of a pool is initialized
    /// @param sender The initial msg.sender for the initialize call
    /// @param key The key for the pool being initialized
    /// @param sqrtPriceX96 The sqrt(price) of the pool as a Q64.96
    /// @return bytes4 The function selector for the hook
    function beforeInitialize(address sender, PoolKey calldata key, uint160 sqrtPriceX96) external returns (bytes4);

    /// @notice The hook called after the state of a pool is initialized
    /// @param sender The initial msg.sender for the initialize call
    /// @param key The key for the pool being initialized
    /// @param sqrtPriceX96 The sqrt(price) of the pool as a Q64.96
    /// @param tick The current tick after the state of a pool is initialized
    /// @return bytes4 The function selector for the hook
    function afterInitialize(address sender, PoolKey calldata key, uint160 sqrtPriceX96, int24 tick)
        external
        returns (bytes4);

    /// @notice The hook called before liquidity is added
    /// @param sender The initial msg.sender for the add liquidity call
    /// @param key The key for the pool
    /// @param params The parameters for adding liquidity
    /// @param hookData Arbitrary data handed into the PoolManager by the liquidity provider to be passed on to the hook
    /// @return bytes4 The function selector for the hook
    function beforeAddLiquidity(
        address sender,
        PoolKey calldata key,
        IPoolManager.ModifyLiquidityParams calldata params,
        bytes calldata hookData
    ) external returns (bytes4);

    /// @notice The hook called after liquidity is added
    /// @param sender The initial msg.sender for the add liquidity call
    /// @param key The key for the pool
    /// @param params The parameters for adding liquidity
    /// @param delta The caller's balance delta after adding liquidity; the sum of principal delta, fees accrued, and hook delta
    /// @param feesAccrued The fees accrued since the last time fees were collected from this position
    /// @param hookData Arbitrary data handed into the PoolManager by the liquidity provider to be passed on to the hook
    /// @return bytes4 The function selector for the hook
    /// @return BalanceDelta The hook's delta in token0 and token1. Positive: the hook is owed/took currency, negative: the hook owes/sent currency
    function afterAddLiquidity(
        address sender,
        PoolKey calldata key,
        IPoolManager.ModifyLiquidityParams calldata params,
        BalanceDelta delta,
        BalanceDelta feesAccrued,
        bytes calldata hookData
    ) external returns (bytes4, BalanceDelta);

    /// @notice The hook called before liquidity is removed
    /// @param sender The initial msg.sender for the remove liquidity call
    /// @param key The key for the pool
    /// @param params The parameters for removing liquidity
    /// @param hookData Arbitrary data handed into the PoolManager by the liquidity provider to be be passed on to the hook
    /// @return bytes4 The function selector for the hook
    function beforeRemoveLiquidity(
        address sender,
        PoolKey calldata key,
        IPoolManager.ModifyLiquidityParams calldata params,
        bytes calldata hookData
    ) external returns (bytes4);

    /// @notice The hook called after liquidity is removed
    /// @param sender The initial msg.sender for the remove liquidity call
    /// @param key The key for the pool
    /// @param params The parameters for removing liquidity
    /// @param delta The caller's balance delta after removing liquidity; the sum of principal delta, fees accrued, and hook delta
    /// @param feesAccrued The fees accrued since the last time fees were collected from this position
    /// @param hookData Arbitrary data handed into the PoolManager by the liquidity provider to be be passed on to the hook
    /// @return bytes4 The function selector for the hook
    /// @return BalanceDelta The hook's delta in token0 and token1. Positive: the hook is owed/took currency, negative: the hook owes/sent currency
    function afterRemoveLiquidity(
        address sender,
        PoolKey calldata key,
        IPoolManager.ModifyLiquidityParams calldata params,
        BalanceDelta delta,
        BalanceDelta feesAccrued,
        bytes calldata hookData
    ) external returns (bytes4, BalanceDelta);

    /// @notice The hook called before a swap
    /// @param sender The initial msg.sender for the swap call
    /// @param key The key for the pool
    /// @param params The parameters for the swap
    /// @param hookData Arbitrary data handed into the PoolManager by the swapper to be be passed on to the hook
    /// @return bytes4 The function selector for the hook
    /// @return BeforeSwapDelta The hook's delta in specified and unspecified currencies. Positive: the hook is owed/took currency, negative: the hook owes/sent currency
    /// @return uint24 Optionally override the lp fee, only used if three conditions are met: 1. the Pool has a dynamic fee, 2. the value's 2nd highest bit is set (23rd bit, 0x400000), and 3. the value is less than or equal to the maximum fee (1 million)
    function beforeSwap(
        address sender,
        PoolKey calldata key,
        IPoolManager.SwapParams calldata params,
        bytes calldata hookData
    ) external returns (bytes4, BeforeSwapDelta, uint24);

    /// @notice The hook called after a swap
    /// @param sender The initial msg.sender for the swap call
    /// @param key The key for the pool
    /// @param params The parameters for the swap
    /// @param delta The amount owed to the caller (positive) or owed to the pool (negative)
    /// @param hookData Arbitrary data handed into the PoolManager by the swapper to be be passed on to the hook
    /// @return bytes4 The function selector for the hook
    /// @return int128 The hook's delta in unspecified currency. Positive: the hook is owed/took currency, negative: the hook owes/sent currency
    function afterSwap(
        address sender,
        PoolKey calldata key,
        IPoolManager.SwapParams calldata params,
        BalanceDelta delta,
        bytes calldata hookData
    ) external returns (bytes4, int128);

    /// @notice The hook called before donate
    /// @param sender The initial msg.sender for the donate call
    /// @param key The key for the pool
    /// @param amount0 The amount of token0 being donated
    /// @param amount1 The amount of token1 being donated
    /// @param hookData Arbitrary data handed into the PoolManager by the donor to be be passed on to the hook
    /// @return bytes4 The function selector for the hook
    function beforeDonate(
        address sender,
        PoolKey calldata key,
        uint256 amount0,
        uint256 amount1,
        bytes calldata hookData
    ) external returns (bytes4);

    /// @notice The hook called after donate
    /// @param sender The initial msg.sender for the donate call
    /// @param key The key for the pool
    /// @param amount0 The amount of token0 being donated
    /// @param amount1 The amount of token1 being donated
    /// @param hookData Arbitrary data handed into the PoolManager by the donor to be be passed on to the hook
    /// @return bytes4 The function selector for the hook
    function afterDonate(
        address sender,
        PoolKey calldata key,
        uint256 amount0,
        uint256 amount1,
        bytes calldata hookData
    ) external returns (bytes4);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/// @notice Interface for functions to access any storage slot in a contract
interface IExtsload {
    /// @notice Called by external contracts to access granular pool state
    /// @param slot Key of slot to sload
    /// @return value The value of the slot as bytes32
    function extsload(bytes32 slot) external view returns (bytes32 value);

    /// @notice Called by external contracts to access granular pool state
    /// @param startSlot Key of slot to start sloading from
    /// @param nSlots Number of slots to load into return value
    /// @return values List of loaded values.
    function extsload(bytes32 startSlot, uint256 nSlots) external view returns (bytes32[] memory values);

    /// @notice Called by external contracts to access sparse pool state
    /// @param slots List of slots to SLOAD from.
    /// @return values List of loaded values.
    function extsload(bytes32[] calldata slots) external view returns (bytes32[] memory values);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Currency} from "./Currency.sol";
import {IHooks} from "../interfaces/IHooks.sol";
import {PoolIdLibrary} from "./PoolId.sol";

using PoolIdLibrary for PoolKey global;

/// @notice Returns the key for identifying a pool
struct PoolKey {
    /// @notice The lower currency of the pool, sorted numerically
    Currency currency0;
    /// @notice The higher currency of the pool, sorted numerically
    Currency currency1;
    /// @notice The pool LP fee, capped at 1_000_000. If the highest bit is 1, the pool has a dynamic fee and must be exactly equal to 0x800000
    uint24 fee;
    /// @notice Ticks that involve positions must be a multiple of tick spacing
    int24 tickSpacing;
    /// @notice The hooks of the pool
    IHooks hooks;
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {PoolKey} from "../types/PoolKey.sol";
import {BalanceDelta} from "../types/BalanceDelta.sol";
import {IPoolManager} from "./IPoolManager.sol";
import {BeforeSwapDelta} from "../types/BeforeSwapDelta.sol";

/// @notice V4 decides whether to invoke specific hooks by inspecting the least significant bits
/// of the address that the hooks contract is deployed to.
/// For example, a hooks contract deployed to address: 0x0000000000000000000000000000000000002400
/// has the lowest bits '10 0100 0000 0000' which would cause the 'before initialize' and 'after add liquidity' hooks to be used.
/// See the Hooks library for the full spec.
/// @dev Should only be callable by the v4 PoolManager.
interface IHooks {
    /// @notice The hook called before the state of a pool is initialized
    /// @param sender The initial msg.sender for the initialize call
    /// @param key The key for the pool being initialized
    /// @param sqrtPriceX96 The sqrt(price) of the pool as a Q64.96
    /// @return bytes4 The function selector for the hook
    function beforeInitialize(address sender, PoolKey calldata key, uint160 sqrtPriceX96) external returns (bytes4);

    /// @notice The hook called after the state of a pool is initialized
    /// @param sender The initial msg.sender for the initialize call
    /// @param key The key for the pool being initialized
    /// @param sqrtPriceX96 The sqrt(price) of the pool as a Q64.96
    /// @param tick The current tick after the state of a pool is initialized
    /// @return bytes4 The function selector for the hook
    function afterInitialize(address sender, PoolKey calldata key, uint160 sqrtPriceX96, int24 tick)
        external
        returns (bytes4);

    /// @notice The hook called before liquidity is added
    /// @param sender The initial msg.sender for the add liquidity call
    /// @param key The key for the pool
    /// @param params The parameters for adding liquidity
    /// @param hookData Arbitrary data handed into the PoolManager by the liquidity provider to be passed on to the hook
    /// @return bytes4 The function selector for the hook
    function beforeAddLiquidity(
        address sender,
        PoolKey calldata key,
        IPoolManager.ModifyLiquidityParams calldata params,
        bytes calldata hookData
    ) external returns (bytes4);

    /// @notice The hook called after liquidity is added
    /// @param sender The initial msg.sender for the add liquidity call
    /// @param key The key for the pool
    /// @param params The parameters for adding liquidity
    /// @param delta The caller's balance delta after adding liquidity; the sum of principal delta, fees accrued, and hook delta
    /// @param feesAccrued The fees accrued since the last time fees were collected from this position
    /// @param hookData Arbitrary data handed into the PoolManager by the liquidity provider to be passed on to the hook
    /// @return bytes4 The function selector for the hook
    /// @return BalanceDelta The hook's delta in token0 and token1. Positive: the hook is owed/took currency, negative: the hook owes/sent currency
    function afterAddLiquidity(
        address sender,
        PoolKey calldata key,
        IPoolManager.ModifyLiquidityParams calldata params,
        BalanceDelta delta,
        BalanceDelta feesAccrued,
        bytes calldata hookData
    ) external returns (bytes4, BalanceDelta);

    /// @notice The hook called before liquidity is removed
    /// @param sender The initial msg.sender for the remove liquidity call
    /// @param key The key for the pool
    /// @param params The parameters for removing liquidity
    /// @param hookData Arbitrary data handed into the PoolManager by the liquidity provider to be be passed on to the hook
    /// @return bytes4 The function selector for the hook
    function beforeRemoveLiquidity(
        address sender,
        PoolKey calldata key,
        IPoolManager.ModifyLiquidityParams calldata params,
        bytes calldata hookData
    ) external returns (bytes4);

    /// @notice The hook called after liquidity is removed
    /// @param sender The initial msg.sender for the remove liquidity call
    /// @param key The key for the pool
    /// @param params The parameters for removing liquidity
    /// @param delta The caller's balance delta after removing liquidity; the sum of principal delta, fees accrued, and hook delta
    /// @param feesAccrued The fees accrued since the last time fees were collected from this position
    /// @param hookData Arbitrary data handed into the PoolManager by the liquidity provider to be be passed on to the hook
    /// @return bytes4 The function selector for the hook
    /// @return BalanceDelta The hook's delta in token0 and token1. Positive: the hook is owed/took currency, negative: the hook owes/sent currency
    function afterRemoveLiquidity(
        address sender,
        PoolKey calldata key,
        IPoolManager.ModifyLiquidityParams calldata params,
        BalanceDelta delta,
        BalanceDelta feesAccrued,
        bytes calldata hookData
    ) external returns (bytes4, BalanceDelta);

    /// @notice The hook called before a swap
    /// @param sender The initial msg.sender for the swap call
    /// @param key The key for the pool
    /// @param params The parameters for the swap
    /// @param hookData Arbitrary data handed into the PoolManager by the swapper to be be passed on to the hook
    /// @return bytes4 The function selector for the hook
    /// @return BeforeSwapDelta The hook's delta in specified and unspecified currencies. Positive: the hook is owed/took currency, negative: the hook owes/sent currency
    /// @return uint24 Optionally override the lp fee, only used if three conditions are met: 1. the Pool has a dynamic fee, 2. the value's 2nd highest bit is set (23rd bit, 0x400000), and 3. the value is less than or equal to the maximum fee (1 million)
    function beforeSwap(
        address sender,
        PoolKey calldata key,
        IPoolManager.SwapParams calldata params,
        bytes calldata hookData
    ) external returns (bytes4, BeforeSwapDelta, uint24);

    /// @notice The hook called after a swap
    /// @param sender The initial msg.sender for the swap call
    /// @param key The key for the pool
    /// @param params The parameters for the swap
    /// @param delta The amount owed to the caller (positive) or owed to the pool (negative)
    /// @param hookData Arbitrary data handed into the PoolManager by the swapper to be be passed on to the hook
    /// @return bytes4 The function selector for the hook
    /// @return int128 The hook's delta in unspecified currency. Positive: the hook is owed/took currency, negative: the hook owes/sent currency
    function afterSwap(
        address sender,
        PoolKey calldata key,
        IPoolManager.SwapParams calldata params,
        BalanceDelta delta,
        bytes calldata hookData
    ) external returns (bytes4, int128);

    /// @notice The hook called before donate
    /// @param sender The initial msg.sender for the donate call
    /// @param key The key for the pool
    /// @param amount0 The amount of token0 being donated
    /// @param amount1 The amount of token1 being donated
    /// @param hookData Arbitrary data handed into the PoolManager by the donor to be be passed on to the hook
    /// @return bytes4 The function selector for the hook
    function beforeDonate(
        address sender,
        PoolKey calldata key,
        uint256 amount0,
        uint256 amount1,
        bytes calldata hookData
    ) external returns (bytes4);

    /// @notice The hook called after donate
    /// @param sender The initial msg.sender for the donate call
    /// @param key The key for the pool
    /// @param amount0 The amount of token0 being donated
    /// @param amount1 The amount of token1 being donated
    /// @param hookData Arbitrary data handed into the PoolManager by the donor to be be passed on to the hook
    /// @return bytes4 The function selector for the hook
    function afterDonate(
        address sender,
        PoolKey calldata key,
        uint256 amount0,
        uint256 amount1,
        bytes calldata hookData
    ) external returns (bytes4);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {IERC20Minimal} from "../interfaces/external/IERC20Minimal.sol";
import {CustomRevert} from "../libraries/CustomRevert.sol";

type Currency is address;

using {greaterThan as >, lessThan as <, greaterThanOrEqualTo as >=, equals as ==} for Currency global;
using CurrencyLibrary for Currency global;

function equals(Currency currency, Currency other) pure returns (bool) {
    return Currency.unwrap(currency) == Currency.unwrap(other);
}

function greaterThan(Currency currency, Currency other) pure returns (bool) {
    return Currency.unwrap(currency) > Currency.unwrap(other);
}

function lessThan(Currency currency, Currency other) pure returns (bool) {
    return Currency.unwrap(currency) < Currency.unwrap(other);
}

function greaterThanOrEqualTo(Currency currency, Currency other) pure returns (bool) {
    return Currency.unwrap(currency) >= Currency.unwrap(other);
}

/// @title CurrencyLibrary
/// @dev This library allows for transferring and holding native tokens and ERC20 tokens
library CurrencyLibrary {
    /// @notice Additional context for ERC-7751 wrapped error when a native transfer fails
    error NativeTransferFailed();

    /// @notice Additional context for ERC-7751 wrapped error when an ERC20 transfer fails
    error ERC20TransferFailed();

    /// @notice A constant to represent the native currency
    Currency public constant ADDRESS_ZERO = Currency.wrap(address(0));

    function transfer(Currency currency, address to, uint256 amount) internal {
        // altered from https://github.com/transmissions11/solmate/blob/44a9963d4c78111f77caa0e65d677b8b46d6f2e6/src/utils/SafeTransferLib.sol
        // modified custom error selectors

        bool success;
        if (currency.isAddressZero()) {
            assembly ("memory-safe") {
                // Transfer the ETH and revert if it fails.
                success := call(gas(), to, amount, 0, 0, 0, 0)
            }
            // revert with NativeTransferFailed, containing the bubbled up error as an argument
            if (!success) {
                CustomRevert.bubbleUpAndRevertWith(to, bytes4(0), NativeTransferFailed.selector);
            }
        } else {
            assembly ("memory-safe") {
                // Get a pointer to some free memory.
                let fmp := mload(0x40)

                // Write the abi-encoded calldata into memory, beginning with the function selector.
                mstore(fmp, 0xa9059cbb00000000000000000000000000000000000000000000000000000000)
                mstore(add(fmp, 4), and(to, 0xffffffffffffffffffffffffffffffffffffffff)) // Append and mask the "to" argument.
                mstore(add(fmp, 36), amount) // Append the "amount" argument. Masking not required as it's a full 32 byte type.

                success :=
                    and(
                        // Set success to whether the call reverted, if not we check it either
                        // returned exactly 1 (can't just be non-zero data), or had no return data.
                        or(and(eq(mload(0), 1), gt(returndatasize(), 31)), iszero(returndatasize())),
                        // We use 68 because the length of our calldata totals up like so: 4 + 32 * 2.
                        // We use 0 and 32 to copy up to 32 bytes of return data into the scratch space.
                        // Counterintuitively, this call must be positioned second to the or() call in the
                        // surrounding and() call or else returndatasize() will be zero during the computation.
                        call(gas(), currency, 0, fmp, 68, 0, 32)
                    )

                // Now clean the memory we used
                mstore(fmp, 0) // 4 byte `selector` and 28 bytes of `to` were stored here
                mstore(add(fmp, 0x20), 0) // 4 bytes of `to` and 28 bytes of `amount` were stored here
                mstore(add(fmp, 0x40), 0) // 4 bytes of `amount` were stored here
            }
            // revert with ERC20TransferFailed, containing the bubbled up error as an argument
            if (!success) {
                CustomRevert.bubbleUpAndRevertWith(
                    Currency.unwrap(currency), IERC20Minimal.transfer.selector, ERC20TransferFailed.selector
                );
            }
        }
    }

    function balanceOfSelf(Currency currency) internal view returns (uint256) {
        if (currency.isAddressZero()) {
            return address(this).balance;
        } else {
            return IERC20Minimal(Currency.unwrap(currency)).balanceOf(address(this));
        }
    }

    function balanceOf(Currency currency, address owner) internal view returns (uint256) {
        if (currency.isAddressZero()) {
            return owner.balance;
        } else {
            return IERC20Minimal(Currency.unwrap(currency)).balanceOf(owner);
        }
    }

    function isAddressZero(Currency currency) internal pure returns (bool) {
        return Currency.unwrap(currency) == Currency.unwrap(ADDRESS_ZERO);
    }

    function toId(Currency currency) internal pure returns (uint256) {
        return uint160(Currency.unwrap(currency));
    }

    // If the upper 12 bytes are non-zero, they will be zero-ed out
    // Therefore, fromId() and toId() are not inverses of each other
    function fromId(uint256 id) internal pure returns (Currency) {
        return Currency.wrap(address(uint160(id)));
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {PoolKey} from "../types/PoolKey.sol";
import {BalanceDelta} from "../types/BalanceDelta.sol";
import {IPoolManager} from "./IPoolManager.sol";
import {BeforeSwapDelta} from "../types/BeforeSwapDelta.sol";

/// @notice V4 decides whether to invoke specific hooks by inspecting the least significant bits
/// of the address that the hooks contract is deployed to.
/// For example, a hooks contract deployed to address: 0x0000000000000000000000000000000000002400
/// has the lowest bits '10 0100 0000 0000' which would cause the 'before initialize' and 'after add liquidity' hooks to be used.
/// See the Hooks library for the full spec.
/// @dev Should only be callable by the v4 PoolManager.
interface IHooks {
    /// @notice The hook called before the state of a pool is initialized
    /// @param sender The initial msg.sender for the initialize call
    /// @param key The key for the pool being initialized
    /// @param sqrtPriceX96 The sqrt(price) of the pool as a Q64.96
    /// @return bytes4 The function selector for the hook
    function beforeInitialize(address sender, PoolKey calldata key, uint160 sqrtPriceX96) external returns (bytes4);

    /// @notice The hook called after the state of a pool is initialized
    /// @param sender The initial msg.sender for the initialize call
    /// @param key The key for the pool being initialized
    /// @param sqrtPriceX96 The sqrt(price) of the pool as a Q64.96
    /// @param tick The current tick after the state of a pool is initialized
    /// @return bytes4 The function selector for the hook
    function afterInitialize(address sender, PoolKey calldata key, uint160 sqrtPriceX96, int24 tick)
        external
        returns (bytes4);

    /// @notice The hook called before liquidity is added
    /// @param sender The initial msg.sender for the add liquidity call
    /// @param key The key for the pool
    /// @param params The parameters for adding liquidity
    /// @param hookData Arbitrary data handed into the PoolManager by the liquidity provider to be passed on to the hook
    /// @return bytes4 The function selector for the hook
    function beforeAddLiquidity(
        address sender,
        PoolKey calldata key,
        IPoolManager.ModifyLiquidityParams calldata params,
        bytes calldata hookData
    ) external returns (bytes4);

    /// @notice The hook called after liquidity is added
    /// @param sender The initial msg.sender for the add liquidity call
    /// @param key The key for the pool
    /// @param params The parameters for adding liquidity
    /// @param delta The caller's balance delta after adding liquidity; the sum of principal delta, fees accrued, and hook delta
    /// @param feesAccrued The fees accrued since the last time fees were collected from this position
    /// @param hookData Arbitrary data handed into the PoolManager by the liquidity provider to be passed on to the hook
    /// @return bytes4 The function selector for the hook
    /// @return BalanceDelta The hook's delta in token0 and token1. Positive: the hook is owed/took currency, negative: the hook owes/sent currency
    function afterAddLiquidity(
        address sender,
        PoolKey calldata key,
        IPoolManager.ModifyLiquidityParams calldata params,
        BalanceDelta delta,
        BalanceDelta feesAccrued,
        bytes calldata hookData
    ) external returns (bytes4, BalanceDelta);

    /// @notice The hook called before liquidity is removed
    /// @param sender The initial msg.sender for the remove liquidity call
    /// @param key The key for the pool
    /// @param params The parameters for removing liquidity
    /// @param hookData Arbitrary data handed into the PoolManager by the liquidity provider to be be passed on to the hook
    /// @return bytes4 The function selector for the hook
    function beforeRemoveLiquidity(
        address sender,
        PoolKey calldata key,
        IPoolManager.ModifyLiquidityParams calldata params,
        bytes calldata hookData
    ) external returns (bytes4);

    /// @notice The hook called after liquidity is removed
    /// @param sender The initial msg.sender for the remove liquidity call
    /// @param key The key for the pool
    /// @param params The parameters for removing liquidity
    /// @param delta The caller's balance delta after removing liquidity; the sum of principal delta, fees accrued, and hook delta
    /// @param feesAccrued The fees accrued since the last time fees were collected from this position
    /// @param hookData Arbitrary data handed into the PoolManager by the liquidity provider to be be passed on to the hook
    /// @return bytes4 The function selector for the hook
    /// @return BalanceDelta The hook's delta in token0 and token1. Positive: the hook is owed/took currency, negative: the hook owes/sent currency
    function afterRemoveLiquidity(
        address sender,
        PoolKey calldata key,
        IPoolManager.ModifyLiquidityParams calldata params,
        BalanceDelta delta,
        BalanceDelta feesAccrued,
        bytes calldata hookData
    ) external returns (bytes4, BalanceDelta);

    /// @notice The hook called before a swap
    /// @param sender The initial msg.sender for the swap call
    /// @param key The key for the pool
    /// @param params The parameters for the swap
    /// @param hookData Arbitrary data handed into the PoolManager by the swapper to be be passed on to the hook
    /// @return bytes4 The function selector for the hook
    /// @return BeforeSwapDelta The hook's delta in specified and unspecified currencies. Positive: the hook is owed/took currency, negative: the hook owes/sent currency
    /// @return uint24 Optionally override the lp fee, only used if three conditions are met: 1. the Pool has a dynamic fee, 2. the value's 2nd highest bit is set (23rd bit, 0x400000), and 3. the value is less than or equal to the maximum fee (1 million)
    function beforeSwap(
        address sender,
        PoolKey calldata key,
        IPoolManager.SwapParams calldata params,
        bytes calldata hookData
    ) external returns (bytes4, BeforeSwapDelta, uint24);

    /// @notice The hook called after a swap
    /// @param sender The initial msg.sender for the swap call
    /// @param key The key for the pool
    /// @param params The parameters for the swap
    /// @param delta The amount owed to the caller (positive) or owed to the pool (negative)
    /// @param hookData Arbitrary data handed into the PoolManager by the swapper to be be passed on to the hook
    /// @return bytes4 The function selector for the hook
    /// @return int128 The hook's delta in unspecified currency. Positive: the hook is owed/took currency, negative: the hook owes/sent currency
    function afterSwap(
        address sender,
        PoolKey calldata key,
        IPoolManager.SwapParams calldata params,
        BalanceDelta delta,
        bytes calldata hookData
    ) external returns (bytes4, int128);

    /// @notice The hook called before donate
    /// @param sender The initial msg.sender for the donate call
    /// @param key The key for the pool
    /// @param amount0 The amount of token0 being donated
    /// @param amount1 The amount of token1 being donated
    /// @param hookData Arbitrary data handed into the PoolManager by the donor to be be passed on to the hook
    /// @return bytes4 The function selector for the hook
    function beforeDonate(
        address sender,
        PoolKey calldata key,
        uint256 amount0,
        uint256 amount1,
        bytes calldata hookData
    ) external returns (bytes4);

    /// @notice The hook called after donate
    /// @param sender The initial msg.sender for the donate call
    /// @param key The key for the pool
    /// @param amount0 The amount of token0 being donated
    /// @param amount1 The amount of token1 being donated
    /// @param hookData Arbitrary data handed into the PoolManager by the donor to be be passed on to the hook
    /// @return bytes4 The function selector for the hook
    function afterDonate(
        address sender,
        PoolKey calldata key,
        uint256 amount0,
        uint256 amount1,
        bytes calldata hookData
    ) external returns (bytes4);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {PoolKey} from "./PoolKey.sol";

type PoolId is bytes32;

/// @notice Library for computing the ID of a pool
library PoolIdLibrary {
    /// @notice Returns value equal to keccak256(abi.encode(poolKey))
    function toId(PoolKey memory poolKey) internal pure returns (PoolId poolId) {
        assembly ("memory-safe") {
            // 0xa0 represents the total size of the poolKey struct (5 slots of 32 bytes)
            poolId := keccak256(poolKey, 0xa0)
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Currency} from "./Currency.sol";
import {IHooks} from "../interfaces/IHooks.sol";
import {PoolIdLibrary} from "./PoolId.sol";

using PoolIdLibrary for PoolKey global;

/// @notice Returns the key for identifying a pool
struct PoolKey {
    /// @notice The lower currency of the pool, sorted numerically
    Currency currency0;
    /// @notice The higher currency of the pool, sorted numerically
    Currency currency1;
    /// @notice The pool LP fee, capped at 1_000_000. If the highest bit is 1, the pool has a dynamic fee and must be exactly equal to 0x800000
    uint24 fee;
    /// @notice Ticks that involve positions must be a multiple of tick spacing
    int24 tickSpacing;
    /// @notice The hooks of the pool
    IHooks hooks;
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

// Return type of the beforeSwap hook.
// Upper 128 bits is the delta in specified tokens. Lower 128 bits is delta in unspecified tokens (to match the afterSwap hook)
type BeforeSwapDelta is int256;

// Creates a BeforeSwapDelta from specified and unspecified
function toBeforeSwapDelta(int128 deltaSpecified, int128 deltaUnspecified)
    pure
    returns (BeforeSwapDelta beforeSwapDelta)
{
    assembly ("memory-safe") {
        beforeSwapDelta := or(shl(128, deltaSpecified), and(sub(shl(128, 1), 1), deltaUnspecified))
    }
}

/// @notice Library for getting the specified and unspecified deltas from the BeforeSwapDelta type
library BeforeSwapDeltaLibrary {
    /// @notice A BeforeSwapDelta of 0
    BeforeSwapDelta public constant ZERO_DELTA = BeforeSwapDelta.wrap(0);

    /// extracts int128 from the upper 128 bits of the BeforeSwapDelta
    /// returned by beforeSwap
    function getSpecifiedDelta(BeforeSwapDelta delta) internal pure returns (int128 deltaSpecified) {
        assembly ("memory-safe") {
            deltaSpecified := sar(128, delta)
        }
    }

    /// extracts int128 from the lower 128 bits of the BeforeSwapDelta
    /// returned by beforeSwap and afterSwap
    function getUnspecifiedDelta(BeforeSwapDelta delta) internal pure returns (int128 deltaUnspecified) {
        assembly ("memory-safe") {
            deltaUnspecified := signextend(15, delta)
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Currency} from "../types/Currency.sol";
import {PoolId} from "../types/PoolId.sol";
import {PoolKey} from "../types/PoolKey.sol";

/// @notice Interface for all protocol-fee related functions in the pool manager
interface IProtocolFees {
    /// @notice Thrown when protocol fee is set too high
    error ProtocolFeeTooLarge(uint24 fee);

    /// @notice Thrown when collectProtocolFees or setProtocolFee is not called by the controller.
    error InvalidCaller();

    /// @notice Thrown when collectProtocolFees is attempted on a token that is synced.
    error ProtocolFeeCurrencySynced();

    /// @notice Emitted when the protocol fee controller address is updated in setProtocolFeeController.
    event ProtocolFeeControllerUpdated(address indexed protocolFeeController);

    /// @notice Emitted when the protocol fee is updated for a pool.
    event ProtocolFeeUpdated(PoolId indexed id, uint24 protocolFee);

    /// @notice Given a currency address, returns the protocol fees accrued in that currency
    /// @param currency The currency to check
    /// @return amount The amount of protocol fees accrued in the currency
    function protocolFeesAccrued(Currency currency) external view returns (uint256 amount);

    /// @notice Sets the protocol fee for the given pool
    /// @param key The key of the pool to set a protocol fee for
    /// @param newProtocolFee The fee to set
    function setProtocolFee(PoolKey memory key, uint24 newProtocolFee) external;

    /// @notice Sets the protocol fee controller
    /// @param controller The new protocol fee controller
    function setProtocolFeeController(address controller) external;

    /// @notice Collects the protocol fees for a given recipient and currency, returning the amount collected
    /// @dev This will revert if the contract is unlocked
    /// @param recipient The address to receive the protocol fees
    /// @param currency The currency to withdraw
    /// @param amount The amount of currency to withdraw
    /// @return amountCollected The amount of currency successfully withdrawn
    function collectProtocolFees(address recipient, Currency currency, uint256 amount)
        external
        returns (uint256 amountCollected);

    /// @notice Returns the current protocol fee controller address
    /// @return address The current protocol fee controller address
    function protocolFeeController() external view returns (address);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {SafeCast} from "../libraries/SafeCast.sol";

/// @dev Two `int128` values packed into a single `int256` where the upper 128 bits represent the amount0
/// and the lower 128 bits represent the amount1.
type BalanceDelta is int256;

using {add as +, sub as -, eq as ==, neq as !=} for BalanceDelta global;
using BalanceDeltaLibrary for BalanceDelta global;
using SafeCast for int256;

function toBalanceDelta(int128 _amount0, int128 _amount1) pure returns (BalanceDelta balanceDelta) {
    assembly ("memory-safe") {
        balanceDelta := or(shl(128, _amount0), and(sub(shl(128, 1), 1), _amount1))
    }
}

function add(BalanceDelta a, BalanceDelta b) pure returns (BalanceDelta) {
    int256 res0;
    int256 res1;
    assembly ("memory-safe") {
        let a0 := sar(128, a)
        let a1 := signextend(15, a)
        let b0 := sar(128, b)
        let b1 := signextend(15, b)
        res0 := add(a0, b0)
        res1 := add(a1, b1)
    }
    return toBalanceDelta(res0.toInt128(), res1.toInt128());
}

function sub(BalanceDelta a, BalanceDelta b) pure returns (BalanceDelta) {
    int256 res0;
    int256 res1;
    assembly ("memory-safe") {
        let a0 := sar(128, a)
        let a1 := signextend(15, a)
        let b0 := sar(128, b)
        let b1 := signextend(15, b)
        res0 := sub(a0, b0)
        res1 := sub(a1, b1)
    }
    return toBalanceDelta(res0.toInt128(), res1.toInt128());
}

function eq(BalanceDelta a, BalanceDelta b) pure returns (bool) {
    return BalanceDelta.unwrap(a) == BalanceDelta.unwrap(b);
}

function neq(BalanceDelta a, BalanceDelta b) pure returns (bool) {
    return BalanceDelta.unwrap(a) != BalanceDelta.unwrap(b);
}

/// @notice Library for getting the amount0 and amount1 deltas from the BalanceDelta type
library BalanceDeltaLibrary {
    /// @notice A BalanceDelta of 0
    BalanceDelta public constant ZERO_DELTA = BalanceDelta.wrap(0);

    function amount0(BalanceDelta balanceDelta) internal pure returns (int128 _amount0) {
        assembly ("memory-safe") {
            _amount0 := sar(128, balanceDelta)
        }
    }

    function amount1(BalanceDelta balanceDelta) internal pure returns (int128 _amount1) {
        assembly ("memory-safe") {
            _amount1 := signextend(15, balanceDelta)
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/// @title Library for reverting with custom errors efficiently
/// @notice Contains functions for reverting with custom errors with different argument types efficiently
/// @dev To use this library, declare `using CustomRevert for bytes4;` and replace `revert CustomError()` with
/// `CustomError.selector.revertWith()`
/// @dev The functions may tamper with the free memory pointer but it is fine since the call context is exited immediately
library CustomRevert {
    /// @dev ERC-7751 error for wrapping bubbled up reverts
    error WrappedError(address target, bytes4 selector, bytes reason, bytes details);

    /// @dev Reverts with the selector of a custom error in the scratch space
    function revertWith(bytes4 selector) internal pure {
        assembly ("memory-safe") {
            mstore(0, selector)
            revert(0, 0x04)
        }
    }

    /// @dev Reverts with a custom error with an address argument in the scratch space
    function revertWith(bytes4 selector, address addr) internal pure {
        assembly ("memory-safe") {
            mstore(0, selector)
            mstore(0x04, and(addr, 0xffffffffffffffffffffffffffffffffffffffff))
            revert(0, 0x24)
        }
    }

    /// @dev Reverts with a custom error with an int24 argument in the scratch space
    function revertWith(bytes4 selector, int24 value) internal pure {
        assembly ("memory-safe") {
            mstore(0, selector)
            mstore(0x04, signextend(2, value))
            revert(0, 0x24)
        }
    }

    /// @dev Reverts with a custom error with a uint160 argument in the scratch space
    function revertWith(bytes4 selector, uint160 value) internal pure {
        assembly ("memory-safe") {
            mstore(0, selector)
            mstore(0x04, and(value, 0xffffffffffffffffffffffffffffffffffffffff))
            revert(0, 0x24)
        }
    }

    /// @dev Reverts with a custom error with two int24 arguments
    function revertWith(bytes4 selector, int24 value1, int24 value2) internal pure {
        assembly ("memory-safe") {
            let fmp := mload(0x40)
            mstore(fmp, selector)
            mstore(add(fmp, 0x04), signextend(2, value1))
            mstore(add(fmp, 0x24), signextend(2, value2))
            revert(fmp, 0x44)
        }
    }

    /// @dev Reverts with a custom error with two uint160 arguments
    function revertWith(bytes4 selector, uint160 value1, uint160 value2) internal pure {
        assembly ("memory-safe") {
            let fmp := mload(0x40)
            mstore(fmp, selector)
            mstore(add(fmp, 0x04), and(value1, 0xffffffffffffffffffffffffffffffffffffffff))
            mstore(add(fmp, 0x24), and(value2, 0xffffffffffffffffffffffffffffffffffffffff))
            revert(fmp, 0x44)
        }
    }

    /// @dev Reverts with a custom error with two address arguments
    function revertWith(bytes4 selector, address value1, address value2) internal pure {
        assembly ("memory-safe") {
            let fmp := mload(0x40)
            mstore(fmp, selector)
            mstore(add(fmp, 0x04), and(value1, 0xffffffffffffffffffffffffffffffffffffffff))
            mstore(add(fmp, 0x24), and(value2, 0xffffffffffffffffffffffffffffffffffffffff))
            revert(fmp, 0x44)
        }
    }

    /// @notice bubble up the revert message returned by a call and revert with a wrapped ERC-7751 error
    /// @dev this method can be vulnerable to revert data bombs
    function bubbleUpAndRevertWith(
        address revertingContract,
        bytes4 revertingFunctionSelector,
        bytes4 additionalContext
    ) internal pure {
        bytes4 wrappedErrorSelector = WrappedError.selector;
        assembly ("memory-safe") {
            // Ensure the size of the revert data is a multiple of 32 bytes
            let encodedDataSize := mul(div(add(returndatasize(), 31), 32), 32)

            let fmp := mload(0x40)

            // Encode wrapped error selector, address, function selector, offset, additional context, size, revert reason
            mstore(fmp, wrappedErrorSelector)
            mstore(add(fmp, 0x04), and(revertingContract, 0xffffffffffffffffffffffffffffffffffffffff))
            mstore(
                add(fmp, 0x24),
                and(revertingFunctionSelector, 0xffffffff00000000000000000000000000000000000000000000000000000000)
            )
            // offset revert reason
            mstore(add(fmp, 0x44), 0x80)
            // offset additional context
            mstore(add(fmp, 0x64), add(0xa0, encodedDataSize))
            // size revert reason
            mstore(add(fmp, 0x84), returndatasize())
            // revert reason
            returndatacopy(add(fmp, 0xa4), 0, returndatasize())
            // size additional context
            mstore(add(fmp, add(0xa4, encodedDataSize)), 0x04)
            // additional context
            mstore(
                add(fmp, add(0xc4, encodedDataSize)),
                and(additionalContext, 0xffffffff00000000000000000000000000000000000000000000000000000000)
            )
            revert(fmp, add(0xe4, encodedDataSize))
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {IERC20Minimal} from "../interfaces/external/IERC20Minimal.sol";
import {CustomRevert} from "../libraries/CustomRevert.sol";

type Currency is address;

using {greaterThan as >, lessThan as <, greaterThanOrEqualTo as >=, equals as ==} for Currency global;
using CurrencyLibrary for Currency global;

function equals(Currency currency, Currency other) pure returns (bool) {
    return Currency.unwrap(currency) == Currency.unwrap(other);
}

function greaterThan(Currency currency, Currency other) pure returns (bool) {
    return Currency.unwrap(currency) > Currency.unwrap(other);
}

function lessThan(Currency currency, Currency other) pure returns (bool) {
    return Currency.unwrap(currency) < Currency.unwrap(other);
}

function greaterThanOrEqualTo(Currency currency, Currency other) pure returns (bool) {
    return Currency.unwrap(currency) >= Currency.unwrap(other);
}

/// @title CurrencyLibrary
/// @dev This library allows for transferring and holding native tokens and ERC20 tokens
library CurrencyLibrary {
    /// @notice Additional context for ERC-7751 wrapped error when a native transfer fails
    error NativeTransferFailed();

    /// @notice Additional context for ERC-7751 wrapped error when an ERC20 transfer fails
    error ERC20TransferFailed();

    /// @notice A constant to represent the native currency
    Currency public constant ADDRESS_ZERO = Currency.wrap(address(0));

    function transfer(Currency currency, address to, uint256 amount) internal {
        // altered from https://github.com/transmissions11/solmate/blob/44a9963d4c78111f77caa0e65d677b8b46d6f2e6/src/utils/SafeTransferLib.sol
        // modified custom error selectors

        bool success;
        if (currency.isAddressZero()) {
            assembly ("memory-safe") {
                // Transfer the ETH and revert if it fails.
                success := call(gas(), to, amount, 0, 0, 0, 0)
            }
            // revert with NativeTransferFailed, containing the bubbled up error as an argument
            if (!success) {
                CustomRevert.bubbleUpAndRevertWith(to, bytes4(0), NativeTransferFailed.selector);
            }
        } else {
            assembly ("memory-safe") {
                // Get a pointer to some free memory.
                let fmp := mload(0x40)

                // Write the abi-encoded calldata into memory, beginning with the function selector.
                mstore(fmp, 0xa9059cbb00000000000000000000000000000000000000000000000000000000)
                mstore(add(fmp, 4), and(to, 0xffffffffffffffffffffffffffffffffffffffff)) // Append and mask the "to" argument.
                mstore(add(fmp, 36), amount) // Append the "amount" argument. Masking not required as it's a full 32 byte type.

                success :=
                    and(
                        // Set success to whether the call reverted, if not we check it either
                        // returned exactly 1 (can't just be non-zero data), or had no return data.
                        or(and(eq(mload(0), 1), gt(returndatasize(), 31)), iszero(returndatasize())),
                        // We use 68 because the length of our calldata totals up like so: 4 + 32 * 2.
                        // We use 0 and 32 to copy up to 32 bytes of return data into the scratch space.
                        // Counterintuitively, this call must be positioned second to the or() call in the
                        // surrounding and() call or else returndatasize() will be zero during the computation.
                        call(gas(), currency, 0, fmp, 68, 0, 32)
                    )

                // Now clean the memory we used
                mstore(fmp, 0) // 4 byte `selector` and 28 bytes of `to` were stored here
                mstore(add(fmp, 0x20), 0) // 4 bytes of `to` and 28 bytes of `amount` were stored here
                mstore(add(fmp, 0x40), 0) // 4 bytes of `amount` were stored here
            }
            // revert with ERC20TransferFailed, containing the bubbled up error as an argument
            if (!success) {
                CustomRevert.bubbleUpAndRevertWith(
                    Currency.unwrap(currency), IERC20Minimal.transfer.selector, ERC20TransferFailed.selector
                );
            }
        }
    }

    function balanceOfSelf(Currency currency) internal view returns (uint256) {
        if (currency.isAddressZero()) {
            return address(this).balance;
        } else {
            return IERC20Minimal(Currency.unwrap(currency)).balanceOf(address(this));
        }
    }

    function balanceOf(Currency currency, address owner) internal view returns (uint256) {
        if (currency.isAddressZero()) {
            return owner.balance;
        } else {
            return IERC20Minimal(Currency.unwrap(currency)).balanceOf(owner);
        }
    }

    function isAddressZero(Currency currency) internal pure returns (bool) {
        return Currency.unwrap(currency) == Currency.unwrap(ADDRESS_ZERO);
    }

    function toId(Currency currency) internal pure returns (uint256) {
        return uint160(Currency.unwrap(currency));
    }

    // If the upper 12 bytes are non-zero, they will be zero-ed out
    // Therefore, fromId() and toId() are not inverses of each other
    function fromId(uint256 id) internal pure returns (Currency) {
        return Currency.wrap(address(uint160(id)));
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/// @notice Interface for claims over a contract balance, wrapped as a ERC6909
interface IERC6909Claims {
    /*//////////////////////////////////////////////////////////////
                                 EVENTS
    //////////////////////////////////////////////////////////////*/

    event OperatorSet(address indexed owner, address indexed operator, bool approved);

    event Approval(address indexed owner, address indexed spender, uint256 indexed id, uint256 amount);

    event Transfer(address caller, address indexed from, address indexed to, uint256 indexed id, uint256 amount);

    /*//////////////////////////////////////////////////////////////
                                 FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Owner balance of an id.
    /// @param owner The address of the owner.
    /// @param id The id of the token.
    /// @return amount The balance of the token.
    function balanceOf(address owner, uint256 id) external view returns (uint256 amount);

    /// @notice Spender allowance of an id.
    /// @param owner The address of the owner.
    /// @param spender The address of the spender.
    /// @param id The id of the token.
    /// @return amount The allowance of the token.
    function allowance(address owner, address spender, uint256 id) external view returns (uint256 amount);

    /// @notice Checks if a spender is approved by an owner as an operator
    /// @param owner The address of the owner.
    /// @param spender The address of the spender.
    /// @return approved The approval status.
    function isOperator(address owner, address spender) external view returns (bool approved);

    /// @notice Transfers an amount of an id from the caller to a receiver.
    /// @param receiver The address of the receiver.
    /// @param id The id of the token.
    /// @param amount The amount of the token.
    /// @return bool True, always, unless the function reverts
    function transfer(address receiver, uint256 id, uint256 amount) external returns (bool);

    /// @notice Transfers an amount of an id from a sender to a receiver.
    /// @param sender The address of the sender.
    /// @param receiver The address of the receiver.
    /// @param id The id of the token.
    /// @param amount The amount of the token.
    /// @return bool True, always, unless the function reverts
    function transferFrom(address sender, address receiver, uint256 id, uint256 amount) external returns (bool);

    /// @notice Approves an amount of an id to a spender.
    /// @param spender The address of the spender.
    /// @param id The id of the token.
    /// @param amount The amount of the token.
    /// @return bool True, always
    function approve(address spender, uint256 id, uint256 amount) external returns (bool);

    /// @notice Sets or removes an operator for the caller.
    /// @param operator The address of the operator.
    /// @param approved The approval status.
    /// @return bool True, always
    function setOperator(address operator, bool approved) external returns (bool);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {CustomRevert} from "./CustomRevert.sol";

/// @title Safe casting methods
/// @notice Contains methods for safely casting between types
library SafeCast {
    using CustomRevert for bytes4;

    error SafeCastOverflow();

    /// @notice Cast a uint256 to a uint160, revert on overflow
    /// @param x The uint256 to be downcasted
    /// @return y The downcasted integer, now type uint160
    function toUint160(uint256 x) internal pure returns (uint160 y) {
        y = uint160(x);
        if (y != x) SafeCastOverflow.selector.revertWith();
    }

    /// @notice Cast a uint256 to a uint128, revert on overflow
    /// @param x The uint256 to be downcasted
    /// @return y The downcasted integer, now type uint128
    function toUint128(uint256 x) internal pure returns (uint128 y) {
        y = uint128(x);
        if (x != y) SafeCastOverflow.selector.revertWith();
    }

    /// @notice Cast a int128 to a uint128, revert on overflow or underflow
    /// @param x The int128 to be casted
    /// @return y The casted integer, now type uint128
    function toUint128(int128 x) internal pure returns (uint128 y) {
        if (x < 0) SafeCastOverflow.selector.revertWith();
        y = uint128(x);
    }

    /// @notice Cast a int256 to a int128, revert on overflow or underflow
    /// @param x The int256 to be downcasted
    /// @return y The downcasted integer, now type int128
    function toInt128(int256 x) internal pure returns (int128 y) {
        y = int128(x);
        if (y != x) SafeCastOverflow.selector.revertWith();
    }

    /// @notice Cast a uint256 to a int256, revert on overflow
    /// @param x The uint256 to be casted
    /// @return y The casted integer, now type int256
    function toInt256(uint256 x) internal pure returns (int256 y) {
        y = int256(x);
        if (y < 0) SafeCastOverflow.selector.revertWith();
    }

    /// @notice Cast a uint256 to a int128, revert on overflow
    /// @param x The uint256 to be downcasted
    /// @return The downcasted integer, now type int128
    function toInt128(uint256 x) internal pure returns (int128) {
        if (x >= 1 << 127) SafeCastOverflow.selector.revertWith();
        return int128(int256(x));
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/// @notice Interface for functions to access any transient storage slot in a contract
interface IExttload {
    /// @notice Called by external contracts to access transient storage of the contract
    /// @param slot Key of slot to tload
    /// @return value The value of the slot as bytes32
    function exttload(bytes32 slot) external view returns (bytes32 value);

    /// @notice Called by external contracts to access sparse transient pool state
    /// @param slots List of slots to tload
    /// @return values List of loaded values
    function exttload(bytes32[] calldata slots) external view returns (bytes32[] memory values);
}


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS

## SUPPORTING CONTEXT: EXTERNAL LIBRARIES
// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.24;

// OpenZeppelin libraries
import {ERC1155Holder} from "@openzeppelin/contracts/token/ERC1155/utils/ERC1155Holder.sol";

/// @title Minimalist ERC1155 implementation without metadata.
/// @author Axicon Labs Limited
/// @author Modified from Solmate (https://github.com/transmissions11/solmate/blob/v7/src/tokens/ERC1155.sol)
/// @dev Not compliant to the letter, does not include any metadata functionality.
abstract contract ERC1155 {
    /*//////////////////////////////////////////////////////////////
                                 EVENTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Emitted when only a single token is transferred.
    /// @param operator The user who initiated the transfer
    /// @param from The user who sent the tokens
    /// @param to The user who received the tokens
    /// @param id The ERC1155 token id
    /// @param amount The amount of tokens transferred
    event TransferSingle(
        address indexed operator,
        address indexed from,
        address indexed to,
        uint256 id,
        uint256 amount
    );

    /// @notice Emitted when multiple tokens are transferred from one user to another.
    /// @param operator The user who initiated the transfer
    /// @param from The user who sent the tokens
    /// @param to The user who received the tokens
    /// @param ids The ERC1155 token ids
    /// @param amounts The amounts of tokens transferred
    event TransferBatch(
        address indexed operator,
        address indexed from,
        address indexed to,
        uint256[] ids,
        uint256[] amounts
    );

    /// @notice Emitted when the approval status of an operator to transfer all tokens on behalf of a user is modified.
    /// @param owner The user who approved or disapproved `operator` to transfer their tokens
    /// @param operator The user who was approved or disapproved to transfer all tokens on behalf of `owner`
    /// @param approved Whether `operator` is approved or disapproved to transfer all tokens on behalf of `owner`
    event ApprovalForAll(address indexed owner, address indexed operator, bool approved);

    /*//////////////////////////////////////////////////////////////
                                 ERRORS
    //////////////////////////////////////////////////////////////*/

    /// @notice Emitted when a user attempts to transfer tokens they do not own nor are approved to transfer.
    error NotAuthorized();

    /// @notice Emitted when an attempt is made to initiate a transfer to a contract recipient that fails to signal support for ERC1155.
    error UnsafeRecipient();

    /*//////////////////////////////////////////////////////////////
                             ERC1155 STORAGE
    //////////////////////////////////////////////////////////////*/

    /// @notice Token balances for each user.
    /// @dev Indexed by user, then by token id.
    mapping(address account => mapping(uint256 tokenId => uint256 balance)) public balanceOf;

    /// @notice Approved addresses for each user.
    /// @dev Indexed by user, then by operator.
    /// @dev Operator is approved to transfer all tokens on behalf of user.
    mapping(address owner => mapping(address operator => bool approvedForAll))
        public isApprovedForAll;

    /*//////////////////////////////////////////////////////////////
                              ERC1155 LOGIC
    //////////////////////////////////////////////////////////////*/

    /// @notice Approve or revoke approval for `operator` to transfer all tokens on behalf of the caller.
    /// @param operator The address to approve or revoke approval for
    /// @param approved True to approve, false to revoke approval
    function setApprovalForAll(address operator, bool approved) public {
        isApprovedForAll[msg.sender][operator] = approved;

        emit ApprovalForAll(msg.sender, operator, approved);
    }

    /// @notice Transfer a single token from one user to another.
    /// @dev Supports approved token transfers.
    /// @param from The user to transfer tokens from
    /// @param to The user to transfer tokens to
    /// @param id The ERC1155 token id to transfer
    /// @param amount The amount of tokens to transfer
    /// @param data Optional data to include in the `onERC1155Received` hook
    function safeTransferFrom(
        address from,
        address to,
        uint256 id,
        uint256 amount,
        bytes calldata data
    ) public virtual {
        if (!(msg.sender == from || isApprovedForAll[from][msg.sender])) revert NotAuthorized();

        balanceOf[from][id] -= amount;

        // balance will never overflow
        unchecked {
            balanceOf[to][id] += amount;
        }

        emit TransferSingle(msg.sender, from, to, id, amount);

        if (to.code.length != 0) {
            if (
                ERC1155Holder(to).onERC1155Received(msg.sender, from, id, amount, data) !=
                ERC1155Holder.onERC1155Received.selector
            ) {
                revert UnsafeRecipient();
            }
        }
    }

    /// @notice Transfer multiple tokens from one user to another.
    /// @dev Supports approved token transfers.
    /// @dev `ids` and `amounts` must be of equal length.
    /// @param from The user to transfer tokens from
    /// @param to The user to transfer tokens to
    /// @param ids The ERC1155 token ids to transfer
    /// @param amounts The amounts of tokens to transfer
    /// @param data Optional data to include in the `onERC1155Received` hook
    function safeBatchTransferFrom(
        address from,
        address to,
        uint256[] calldata ids,
        uint256[] calldata amounts,
        bytes calldata data
    ) public virtual {
        if (!(msg.sender == from || isApprovedForAll[from][msg.sender])) revert NotAuthorized();

        // Storing these outside the loop saves ~15 gas per iteration.
        uint256 id;
        uint256 amount;

        for (uint256 i = 0; i < ids.length; ) {
            id = ids[i];
            amount = amounts[i];

            balanceOf[from][id] -= amount;

            // balance will never overflow
            unchecked {
                balanceOf[to][id] += amount;
            }

            // An array can't have a total length
            // larger than the max uint256 value.
            unchecked {
                ++i;
            }
        }

        emit TransferBatch(msg.sender, from, to, ids, amounts);

        if (to.code.length != 0) {
            if (
                ERC1155Holder(to).onERC1155BatchReceived(msg.sender, from, ids, amounts, data) !=
                ERC1155Holder.onERC1155BatchReceived.selector
            ) {
                revert UnsafeRecipient();
            }
        }
    }

    /// @notice Query balances for multiple users and tokens at once.
    /// @dev `owners` and `ids` should be of equal length.
    /// @param owners The list of users to query balances for
    /// @param ids The list of ERC1155 token ids to query
    /// @return balances The balances for each owner-id pair in the same order as the input arrays
    function balanceOfBatch(
        address[] calldata owners,
        uint256[] calldata ids
    ) public view returns (uint256[] memory balances) {
        balances = new uint256[](owners.length);

        // Unchecked because the only math done is incrementing
        // the array index counter which cannot possibly overflow.
        unchecked {
            for (uint256 i = 0; i < owners.length; ++i) {
                balances[i] = balanceOf[owners[i]][ids[i]];
            }
        }
    }

    /*//////////////////////////////////////////////////////////////
                              ERC165 LOGIC
    //////////////////////////////////////////////////////////////*/

    /// @notice Signal support for ERC165 and ERC1155.
    /// @param interfaceId The interface to check for support
    /// @return Whether the interface is supported
    function supportsInterface(bytes4 interfaceId) public pure returns (bool) {
        return
            interfaceId == 0x01ffc9a7 || // ERC165 Interface ID for ERC165
            interfaceId == 0xd9b67a26; // ERC165 Interface ID for ERC1155
    }

    /*//////////////////////////////////////////////////////////////
                        INTERNAL MINT/BURN LOGIC
    //////////////////////////////////////////////////////////////*/

    /// @notice Internal utility to mint tokens to a user's account.
    /// @param to The user to mint tokens to
    /// @param id The ERC1155 token id to mint
    /// @param amount The amount of tokens to mint
    function _mint(address to, uint256 id, uint256 amount) internal {
        // balance will never overflow
        unchecked {
            balanceOf[to][id] += amount;
        }

        emit TransferSingle(msg.sender, address(0), to, id, amount);

        if (to.code.length != 0) {
            if (
                ERC1155Holder(to).onERC1155Received(msg.sender, address(0), id, amount, "") !=
                ERC1155Holder.onERC1155Received.selector
            ) {
                revert UnsafeRecipient();
            }
        }
    }

    /// @notice Internal utility to burn tokens from a user's account.
    /// @param from The user to burn tokens from
    /// @param id The ERC1155 token id to burn
    /// @param amount The amount of tokens to burn
    function _burn(address from, uint256 id, uint256 amount) internal {
        balanceOf[from][id] -= amount;

        emit TransferSingle(msg.sender, from, address(0), id, amount);
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.24;

/// @title Custom Errors library.
/// @author Axicon Labs Limited
/// @notice Contains all custom error messages used in Panoptic.
library Errors {
    /// @notice PanopticPool: The account is not solvent enough to perform the desired action
    error AccountInsolvent(uint256 solvent, uint256 numberOfTicks);

    /// @notice Casting error
    /// @dev e.g. uint128(uint256(a)) fails
    error CastingError();

    /// @notice CollateralTracker: Attempted to withdraw/redeem less than a single asset
    error BelowMinimumRedemption();

    /// @notice SFPM: Mints/burns of zero-liquidity chunks in Uniswap are not supported
    error ChunkHasZeroLiquidity();

    /// @notice CollateralTracker: Collateral token has already been initialized
    error CollateralTokenAlreadyInitialized();

    /// @notice CollateralTracker: The amount of shares (or assets) deposited is larger than the maximum permitted
    error DepositTooLarge();

    /// @notice PanopticPool: The list of provided TokenIds has a duplicate entry
    error DuplicateTokenId();

    /// @notice PanopticPool: The effective liquidity (X32) is greater than min(`MAX_SPREAD`, `USER_PROVIDED_THRESHOLD`) during a long mint or short burn
    /// @dev Effective liquidity measures how much new liquidity is minted relative to how much is already in the pool
    error EffectiveLiquidityAboveThreshold();

    /// @notice CollateralTracker: Attempted to withdraw/redeem more than available liquidity, owned shares, or open positions would allow for
    error ExceedsMaximumRedemption();

    /// @notice PanopticPool: The provided list of option positions is incorrect or invalid
    error InputListFail();

    /// @notice Tick is not between `MIN_TICK` and `MAX_TICK`
    error InvalidTick();

    /// @notice Liquidity in a chunk is above 2**128
    error LiquidityTooHigh();

    /// @notice CollateralTracker: There is not enough available liquidity to fulfill a credit in the PanopticPool
    error InsufficientCreditLiquidity();

    /// @notice RiskEngine: invalid builder code
    error InvalidBuilderCode();

    /// @notice The TokenId provided by the user is malformed or invalid
    /// @param parameterType poolId=0, ratio=1, tokenType=2, risk_partner=3, strike=4, width=5, two identical strike/width/tokenType chunks=6
    error InvalidTokenIdParameter(uint256 parameterType);

    /// @notice A mint or swap callback was attempted from an address that did not match the canonical Uniswap V3 pool with the claimed features
    error InvalidUniswapCallback();

    /// @notice RiskEngine: There is a mismatch between the length of the positionIdList and positionBalanceArray
    error LengthMismatch();

    /// @notice PanopticPool: The Net Liquidity is zero due to small positions and cannot be used to compute the liquiditySpread
    error NetLiquidityZero();

    /// @notice PanopticPool: None of the legs in a position are force-exercisable (they are all either short or ATM long)
    error NoLegsExercisable();

    /// @notice PanopticPool: The leg is not long, so premium cannot be settled through `settleLongPremium`
    error NotALongLeg();

    /// @notice builderWallet: can only be called by the Builder
    error NotBuilder();

    /// @notice PanopticPool: There is not enough available liquidity in the chunk for one of the long legs to be created (or for one of the short legs to be closed)
    error NotEnoughLiquidityInChunk();

    /// @notice CollateralTracker: The user does not own enough assets to open/close a position
    error NotEnoughTokens(address tokenAddress, uint256 assetsRequested, uint256 assetBalance);

    /// @notice RiskEngine: can only be called by the guardian
    error NotGuardian();

    /// @notice PanopticPool: Position is still solvent and cannot be liquidated
    error NotMarginCalled();

    /// @notice CollateralTracker: The caller for a permissioned function is not the Panoptic Pool
    error NotPanopticPool();

    /// @notice Uniswap pool has already been initialized in the SFPM or created in the factory
    error PoolAlreadyInitialized();

    /// @notice The Uniswap Pool has not been created, so it cannot be used in the SFPM or have a PanopticPool created for it by the factory
    error PoolNotInitialized();

    /// @notice CollateralTracker: The user has open/active option positions, so they cannot transfer collateral shares
    error PositionCountNotZero();

    /// @notice PanopticPool: A position with the given token ID is not owned by the user and has positionSize=0
    error PositionNotOwned();

    /// @notice SFPM: The maximum token deltas (excluding swaps) for a position exceed (2^127 - 5) at some valid price
    error PositionTooLarge();

    /// @notice The current tick in the pool (post-ITM-swap) has fallen outside a user-defined open interval slippage range
    error PriceBoundFail(int24 currentTick);

    /// @notice The Price impact of that trade is too large
    error PriceImpactTooLarge();

    /// @notice An oracle price is too far away from another oracle price or the current tick
    /// @dev This is a safeguard against price manipulation during option mints, burns, liquidations, force exercises, and premium settlements
    error StaleOracle();

    /// @notice PanopticPool: The position being minted would increase the total amount of legs open for the account above the maximum
    error TooManyLegsOpen();

    /// @notice ERC20 or SFPM (ERC1155) token transfer did not complete successfully
    error TransferFailed(address token, address from, uint256 amount, uint256 balance);

    /// @notice The tick range given by the strike price and width is invalid
    /// because the upper and lower ticks are not initializable multiples of `tickSpacing`
    /// or one of the ticks exceeds the `MIN_TICK` or `MAX_TICK` bounds
    error InvalidTickBound();

    /// @notice An unlock callback was attempted from an address other than the canonical Uniswap V4 pool manager
    error UnauthorizedUniswapCallback();

    /// @notice An operation in a library has failed due to an underflow or overflow
    error UnderOverFlow();

    /// @notice PanopticPool: The supplied poolId does not match the poolId for that Uniswap Pool
    error WrongPoolId();

    /// @notice SFPM: The poolId's don't match
    error WrongUniswapPool();

    /// @notice PanopticFactory: the zero address was supplied as a parameter
    error ZeroAddress();

    /// @notice CollateralTracker: Mints/burns of a position returns no collateral requirement
    error ZeroCollateralRequirement();

    /// @notice PanopticMath: The supplied tokenId has no valid legs
    error TokenIdHasZeroLegs();
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.24;
// Interfaces
import {IERC20Metadata} from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import {IUniswapV3Pool} from "univ3-core/interfaces/IUniswapV3Pool.sol";
// Libraries
import {Constants} from "@libraries/Constants.sol";
import {Math} from "@libraries/Math.sol";
import {EfficientHash} from "@libraries/EfficientHash.sol";
import {Errors} from "@libraries/Errors.sol";
// OpenZeppelin libraries
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
// Custom types
import {LeftRightUnsigned, LeftRightSigned} from "@types/LeftRight.sol";
import {LiquidityChunk} from "@types/LiquidityChunk.sol";
import {TokenId} from "@types/TokenId.sol";
import {RiskParameters} from "@types/RiskParameters.sol";

/// @title Compute general math quantities relevant to Panoptic and AMM pool management.
/// @notice Contains Panoptic-specific helpers and math functions.
/// @author Axicon Labs Limited
library PanopticMath {
    using Math for uint256;

    /// @notice This is equivalent to `type(uint256).max` — used in assembly blocks as a replacement.
    uint256 internal constant MAX_UINT256 = 2 ** 256 - 1;

    /// @notice Masks 16-bit tickSpacing and 8 bits of vegoid out of 64-bit `[16-bit tickspacing][8-bit vegoid][40-bit poolPattern]` format poolId.
    uint64 internal constant TICKSPACING_VEGOID_MASK = 0xFFFFFF0000000000;

    uint256 internal constant PRIME_MODULUS_248 =
        0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff13;

    uint256 internal constant PRIME_MODULUS_124_0 = 0xfffffffffffffffffffffffffffffc5; // 2**124 - 59
    uint256 internal constant PRIME_MODULUS_124_1 = 0xffffffffffffffffffffffffffffd99; // 2**124 - 615

    // Mask for isolating a 124-bit lane
    uint256 internal constant LANE_MASK_124 = 0xfffffffffffffffffffffffffffffff;

    uint256 internal constant UPPER_120BITS_MASK =
        0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF00000000000000000000000000000000;

    uint256 internal constant BITMASK_UINT88 = 0xFFFFFFFFFFFFFFFFFFFFFF;
    uint256 internal constant BITMASK_UINT22 = 0x3FFFFF;

    /*//////////////////////////////////////////////////////////////
                              UTILITIES
    //////////////////////////////////////////////////////////////*/

    /// @notice Increments the pool pattern (first 48 bits) of a poolId by 1.
    /// @param poolId The 64-bit pool ID
    /// @return The provided `poolId` with its pool pattern slot incremented by 1
    function incrementPoolPattern(uint64 poolId) internal pure returns (uint64) {
        unchecked {
            return (poolId & TICKSPACING_VEGOID_MASK) + (uint40(poolId + 1));
        }
    }

    /// @notice Get the number of leading hex characters in an address.
    //     0x0000bababaab...     0xababababab...
    //          ▲                 ▲
    //          │                 │
    //     4 leading hex      0 leading hex
    //    character zeros    character zeros
    //
    /// @param addr The address to get the number of leading zero hex characters for
    /// @return The number of leading zero hex characters in the address
    function numberOfLeadingHexZeros(address addr) external pure returns (uint256) {
        unchecked {
            return addr == address(0) ? 40 : 39 - Math.mostSignificantNibble(uint160(addr));
        }
    }

    /// @notice Returns ERC20 symbol of `token`.
    /// @param token The address of the token to get the symbol of
    /// @return The symbol of `token` or "???" if not supported
    function safeERC20Symbol(address token) external view returns (string memory) {
        // not guaranteed that token supports metadata extension
        // so we need to let call fail and return placeholder if not
        try IERC20Metadata(token).symbol() returns (string memory symbol) {
            return symbol;
        } catch {
            return "???";
        }
    }

    /// @notice Converts `fee` to a string with "bps" appended.
    /// @dev The lowest supported value of `fee` is 1 (`="0.01bps"`).
    /// @param fee The fee to convert to a string (in hundredths of basis points)
    /// @return Stringified version of `fee` with "bps" appended
    function uniswapFeeToString(uint24 fee) internal pure returns (string memory) {
        return
            string.concat(
                Strings.toString(fee / 100),
                fee % 100 == 0
                    ? ""
                    : string.concat(
                        ".",
                        Strings.toString((fee / 10) % 10),
                        Strings.toString(fee % 10)
                    ),
                "bps"
            );
    }

    /// @notice Update an existing account's "positions hash" with a new `tokenId`.
    /// @notice The positions hash contains a fingerprint of all open positions created by an account/user and a count of the legs across those positions.
    /// @dev The "fingerprint" portion of the hash is given by XORing the hashed `tokenId` of each position the user has open together.
    /// @param existingHash The existing position hash representing a list of positions and the count of the legs across those positions
    /// @param tokenId The new position to modify the existing hash with: `existingHash = uint248(existingHash) ^ uint248(hashOf(tokenId))`
    /// @param addFlag Whether to mint (add) the tokenId to the count of positions or burn (subtract) it from the count `(existingHash >> 248) +/- tokenId.countLegs()`
    /// @return newHash The updated position hash with the new tokenId XORed in and the leg count incremented/decremented
    function updatePositionsHash(
        uint256 existingHash,
        TokenId tokenId,
        bool addFlag
    ) internal pure returns (uint256) {
        // update hash by using the homomorphicHash method
        uint256 updatedHash = homomorphicHash(existingHash, TokenId.unwrap(tokenId), addFlag);

        // increment the upper 8 bits (leg counter) if addFlag=true, decrement otherwise
        uint8 numberOfLegs = uint8(tokenId.countLegs());
        if (numberOfLegs == 0) revert Errors.TokenIdHasZeroLegs();

        // unchecked, so reverts if overflow
        uint256 newLegCount = addFlag
            ? uint8(existingHash >> 248) + numberOfLegs
            : uint8(existingHash >> 248) - numberOfLegs;

        unchecked {
            return uint256(updatedHash) + (newLegCount << 248);
        }
    }

    /// @notice Computes a homomorphic hash by adding or subtracting an item from an existing hash
    /// @dev Uses XOR-based homomorphic hashing (XHASH). The hash of the item is XORed with the
    ///      existing hash. Since XOR is its own inverse (A ⊕ B ⊕ B = A), both addition and
    ///      subtraction operations use the same XOR operation. This ensures the operation is
    ///      reversible and order-independent for the same set of items.
    ///      OR
    ///      Uses additive homomorphic hashing (AdHash) over a 248-bit prime field. The hash of the item
    ///      is either added to or subtracted from the existing hash using modular arithmetic.
    ///      Subtraction is implemented as addition of the modular inverse: hash + (PRIME - itemHash) mod PRIME.
    ///      This ensures the operation is reversible and order-independent for the same set of items.
    ///      OR
    ///      Uses LtHash (Lattice-based Hash) with k=2 lanes for improved collision resistance.
    ///      The 248-bit hash space is divided into two 124-bit lanes, each operating under
    ///      modular arithmetic with a 124-bit prime. The item hash is split into two 124-bit
    ///      chunks and each chunk is added/subtracted from its corresponding lane independently.
    ///      Subtraction is implemented as addition of the modular inverse: lane + (PRIME - chunk) mod PRIME.
    ///      This parallel lane approach provides better security properties than single-lane hashing
    ///      while maintaining homomorphic properties (order-independence and reversibility).
    /// @param hash The existing hash value (only lower 248 bits are used)
    /// @param item The item to be hashed and added/subtracted (typically a TokenId cast to uint256)
    /// @param addFlag True to add the item to the hash, false to subtract it
    /// @return The updated homomorphic hash as a uint256 (but only lower 248 bits contain the hash)
    function homomorphicHash(
        uint256 hash,
        uint256 item,
        bool addFlag
    ) internal pure returns (uint256) {
        /*
        {
            // XHASH
            return
                uint248(hash) ^
                (uint248(uint256(EfficientHash.efficientKeccak256(abi.encode(item)))));
        }
        {
            // AdHash
            uint256 itemHash = uint256(EfficientHash.efficientKeccak256(abi.encode(item)));
            return
                addFlag
                    ? addmod(uint248(hash), uint248(itemHash), PRIME_MODULUS_248)
                    : addmod(
                        uint248(hash),
                        PRIME_MODULUS_248 - (itemHash % PRIME_MODULUS_248),
                        PRIME_MODULUS_248
                    );
        }
        */
        unchecked {
            // LtHash, k=2
            uint256 itemHash = uint256(EfficientHash.efficientKeccak256(abi.encode(item)));

            // Pre-calculate the 124-bit chunks for the item to be added/removed
            uint256 item_h0 = itemHash & LANE_MASK_124;
            uint256 item_h1 = (itemHash >> 124) & LANE_MASK_124;

            uint256 lane0 = hash & LANE_MASK_124;
            uint256 newItem_h0 = addFlag
                ? item_h0
                : PRIME_MODULUS_124_0 - (item_h0 % PRIME_MODULUS_124_0);
            uint256 hash0 = addmod(lane0, newItem_h0, PRIME_MODULUS_124_0);

            uint256 lane1 = (hash >> 124) & LANE_MASK_124;
            uint256 newItem_h1 = addFlag
                ? item_h1
                : PRIME_MODULUS_124_1 - (item_h1 % PRIME_MODULUS_124_1);
            uint256 hash1 = addmod(lane1, newItem_h1, PRIME_MODULUS_124_1);

            return hash0 + (hash1 << 124);
        }
    }

    /// @notice Checks if an array of TokenIds contains any duplicate values
    /// @dev Uses assembly for gas optimization. Performs O(n²) comparison by checking each element
    ///      against all subsequent elements. Returns false immediately upon finding the first duplicate.
    ///      Arrays with 0 or 1 elements are considered to have no duplicates.
    /// @param arr The array of TokenIds to check for duplicates
    /// @return True if the array contains no duplicate TokenIds, false if duplicates are found
    function hasNoDuplicateTokenIds(TokenId[] calldata arr) external pure returns (bool) {
        assembly {
            let len := arr.length
            let offset := arr.offset

            // Early return for 0 or 1 elements
            if lt(len, 2) {
                mstore(0x00, 1)
                return(0x00, 0x20)
            }

            // Check for duplicates
            for {
                let i := 0
            } lt(i, len) {
                i := add(i, 1)
            } {
                let val := calldataload(add(offset, mul(i, 0x20)))
                for {
                    let j := add(i, 1)
                } lt(j, len) {
                    j := add(j, 1)
                } {
                    if eq(val, calldataload(add(offset, mul(j, 0x20)))) {
                        mstore(0x00, 0)
                        return(0x00, 0x20)
                    }
                }
            }

            mstore(0x00, 1)
            return(0x00, 0x20)
        }
    }

    /*//////////////////////////////////////////////////////////////
                          ORACLE CALCULATIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Returns the median of the last `cardinality` average prices over `period` observations from `univ3pool`.
    /// @dev Used when we need a manipulation-resistant TWAP price.
    /// @dev Uniswap observations snapshot the closing price of the last block before the first interaction of a given block.
    /// @dev The maximum frequency of observations is 1 per block, but there is no guarantee that the pool will be observed at every block.
    /// @dev Each period has a minimum length of `blocktime * period`, but may be longer if the Uniswap pool is relatively inactive.
    /// @dev The final price used in the array (of length `cardinality`) is the average of `cardinality` observations spaced by `period` (which is itself a number of observations).
    /// @dev Thus, the minimum total time window is `cardinality * period * blocktime`.
    /// @param univ3pool The Uniswap pool to get the median observation from
    /// @param observationIndex The index of the last observation in the pool
    /// @param observationCardinality The number of observations in the pool
    /// @param cardinality The number of `periods` to in the median price array, should be odd
    /// @param period The number of observations to average to compute one entry in the median price array
    /// @return The median of `cardinality` observations spaced by `period` in the Uniswap pool
    /// @return The latest observation in the Uniswap pool
    function computeMedianObservedPrice(
        IUniswapV3Pool univ3pool,
        uint256 observationIndex,
        uint256 observationCardinality,
        uint256 cardinality,
        uint256 period
    ) internal view returns (int24, int24) {
        unchecked {
            int256[] memory tickCumulatives = new int256[](cardinality + 1);

            uint256[] memory timestamps = new uint256[](cardinality + 1);
            // get the last "cardinality" timestamps/tickCumulatives (if observationIndex < cardinality, the index will wrap back from observationCardinality)
            for (uint256 i = 0; i < cardinality + 1; ++i) {
                (timestamps[i], tickCumulatives[i], , ) = univ3pool.observations(
                    uint256(
                        (int256(observationIndex) - int256(i * period)) +
                            int256(observationCardinality)
                    ) % observationCardinality
                );
            }

            int256[] memory ticks = new int256[](cardinality);
            // use cardinality periods given by cardinality + 1 accumulator observations to compute the last cardinality observed ticks spaced by period
            for (uint256 i = 0; i < cardinality; ++i) {
                ticks[i] =
                    (tickCumulatives[i] - tickCumulatives[i + 1]) /
                    int256(timestamps[i] - timestamps[i + 1]);
            }

            // the `ticks` array descends from the most recent Uniswap observation prior to the sort
            int24 latestTick = int24(ticks[0]);

            // get the median of the `ticks` array (assuming `cardinality` is odd)
            return (int24(Math.sort(ticks)[cardinality / 2]), latestTick);
        }
    }

    /// @notice Computes the TWAP of a Uniswap V3 pool using data from its oracle.
    /// @dev Note that our definition of TWAP differs from a typical mean of prices over a time window.
    /// @dev We instead observe the average price over a series of time intervals, and define the TWAP as the median of those averages.
    /// @param univ3pool The Uniswap pool from which to compute the TWAP
    /// @param twapWindow The time window to compute the TWAP over
    /// @return The final calculated TWAP tick
    function twapFilter(IUniswapV3Pool univ3pool, uint32 twapWindow) external view returns (int24) {
        uint32[] memory secondsAgos = new uint32[](20);

        int256[] memory twapMeasurement = new int256[](19);

        unchecked {
            // construct the time slots
            for (uint256 i = 0; i < 20; ++i) {
                secondsAgos[i] = uint32(((i + 1) * twapWindow) / 20);
            }

            // observe the tickCumulative at the 20 pre-defined time slots
            (int56[] memory tickCumulatives, ) = univ3pool.observe(secondsAgos);

            // compute the average tick per 30s window
            for (uint256 i = 0; i < 19; ++i) {
                twapMeasurement[i] = int24(
                    (tickCumulatives[i] - tickCumulatives[i + 1]) / int56(uint56(twapWindow / 20))
                );
            }

            // sort the tick measurements
            int256[] memory sortedTicks = Math.sort(twapMeasurement);

            // Get the median value
            return int24(sortedTicks[9]);
        }
    }

    /*//////////////////////////////////////////////////////////////
                          LIQUIDITY CHUNK MATH
    //////////////////////////////////////////////////////////////*/

    /// @notice For a given option position (`tokenId`), leg index within that position (`legIndex`), and `positionSize` get the tick range spanned and its
    /// liquidity (share ownership) in the Uniswap V3 pool; this is a liquidity chunk.
    //          Liquidity chunk  (defined by tick upper, tick lower, and its size/amount: the liquidity)
    //   liquidity    │
    //         ▲      │
    //         │     ┌▼┐
    //         │  ┌──┴─┴──┐
    //         │  │       │
    //         │  │       │
    //         └──┴───────┴────► price
    //         Uniswap V3 Pool
    /// @param tokenId The option position id
    /// @param legIndex The leg index of the option position, can be {0,1,2,3}
    /// @param positionSize The number of contracts held by this leg
    /// @return A LiquidityChunk with `tickLower`, `tickUpper`, and `liquidity`
    function getLiquidityChunk(
        TokenId tokenId,
        uint256 legIndex,
        uint128 positionSize
    ) internal pure returns (LiquidityChunk) {
        // get the tick range for this leg
        (int24 tickLower, int24 tickUpper) = tokenId.asTicks(legIndex);

        // Get the amount of liquidity owned by this leg in the Uniswap V3 pool in the above tick range
        // Background:
        //
        //  In Uniswap V3, the amount of liquidity received for a given amount of token0 when the price is
        //  not in range is given by:
        //     Liquidity = amount0 * (sqrt(upper) * sqrt(lower)) / (sqrt(upper) - sqrt(lower))
        //  For token1, it is given by:
        //     Liquidity = amount1 / (sqrt(upper) - sqrt(lower))
        //
        //  However, in Panoptic, each position has a asset parameter. The asset is the "basis" of the position.
        //  In TradFi, the asset is always cash and selling a $1000 put requires the user to lock $1000, and selling
        //  a call requires the user to lock 1 unit of asset.
        //
        //  Because Uniswap V3 chooses token0 and token1 from the alphanumeric order, there is no consistency as to whether token0 is
        //  stablecoin, ETH, or an ERC20. Some pools may want ETH to be the asset (e.g. ETH-DAI) and some may wish the stablecoin to
        //  be the asset (e.g. DAI-ETH) so that K asset is moved for puts and 1 asset is moved for calls.
        //  But since the convention is to force the order always we have no say in this.
        //
        //  To solve this, we encode the asset value in tokenId. This parameter specifies which of token0 or token1 is the
        //  asset, such that:
        //     when asset=0, then amount0 moved at strike K =1.0001**currentTick is 1, amount1 moved to strike K is K
        //     when asset=1, then amount1 moved at strike K =1.0001**currentTick is K, amount0 moved to strike K is 1/K
        //
        //  The following function takes this into account when computing the liquidity of the leg and switches between
        //  the definition for getLiquidityForAmount0 or getLiquidityForAmount1 when relevant.

        uint256 amount = positionSize * tokenId.optionRatio(legIndex);
        if (tokenId.asset(legIndex) == 0) {
            return Math.getLiquidityForAmount0(tickLower, tickUpper, amount);
        } else {
            return Math.getLiquidityForAmount1(tickLower, tickUpper, amount);
        }
    }

    /// @notice Extract the tick range specified by `strike` and `width` for the given `tickSpacing`.
    /// @param strike The strike price of the option
    /// @param width The width of the option
    /// @param tickSpacing The tick spacing of the underlying Uniswap V3 pool
    /// @return The lower tick of the liquidity chunk
    /// @return The upper tick of the liquidity chunk
    function getTicks(
        int24 strike,
        int24 width,
        int24 tickSpacing
    ) internal pure returns (int24, int24) {
        (int24 rangeDown, int24 rangeUp) = PanopticMath.getRangesFromStrike(width, tickSpacing);

        unchecked {
            return (strike - rangeDown, strike + rangeUp);
        }
    }

    /// @notice Returns the distances of the upper and lower ticks from the strike for a position with the given width and tickSpacing.
    /// @dev Given `r = (width * tickSpacing) / 2`, `tickLower = strike - floor(r)` and `tickUpper = strike + ceil(r)`.
    /// @param width The width of the leg
    /// @param tickSpacing The tick spacing of the underlying pool
    /// @return The distance of the lower tick from the strike
    /// @return The distance of the upper tick from the strike
    function getRangesFromStrike(
        int24 width,
        int24 tickSpacing
    ) internal pure returns (int24, int24) {
        return (
            (width * tickSpacing) / 2,
            int24(int256(Math.unsafeDivRoundingUp(uint24(width) * uint24(tickSpacing), 2)))
        );
    }

    /// @notice Computes the chunk key for a given leg of a position.
    /// @dev The chunk key uniquely identifies a liquidity chunk by its strike, width, and token type.
    /// @param tokenId The option position
    /// @param leg The leg index within the position
    /// @return chunkKey The keccak256 hash identifying this chunk
    function getChunkKey(TokenId tokenId, uint256 leg) internal pure returns (bytes32 chunkKey) {
        chunkKey = EfficientHash.efficientKeccak256(
            abi.encodePacked(tokenId.strike(leg), tokenId.width(leg), tokenId.tokenType(leg))
        );
    }

    /*//////////////////////////////////////////////////////////////
                         TOKEN CONVERSION LOGIC
    //////////////////////////////////////////////////////////////*/

    /// @notice Compute the amount of notional value underlying an option position.
    /// @param tokenId The option position id
    /// @param positionSize The number of contracts of the option
    /// @param opening Whether you need the token0s and token1s moved while opening the position, or while closing
    /// @return longAmounts Left-right packed word where rightSlot = token0 and leftSlot = token1 held against borrowed Uniswap liquidity for long legs
    /// @return shortAmounts Left-right packed word where where rightSlot = token0 and leftSlot = token1 borrowed to create short legs
    function computeExercisedAmounts(
        TokenId tokenId,
        uint128 positionSize,
        bool opening
    ) internal pure returns (LeftRightSigned longAmounts, LeftRightSigned shortAmounts) {
        uint256 numLegs = tokenId.countLegs();
        for (uint256 leg = 0; leg < numLegs; ) {
            (LeftRightSigned longs, LeftRightSigned shorts) = calculateIOAmounts(
                tokenId,
                positionSize,
                leg,
                opening
            );

            longAmounts = longAmounts.add(longs);
            shortAmounts = shortAmounts.add(shorts);
            unchecked {
                ++leg;
            }
        }
    }

    /// @notice Convert an amount of token0 into an amount of token1 given the sqrtPriceX96 in a Uniswap pool defined as `sqrt(1/0)*2^96`.
    /// @dev Uses reduced precision after tick 443636 in order to accommodate the full range of ticks
    /// @param amount The amount of token0 to convert into token1
    /// @param sqrtPriceX96 The square root of the price at which to convert `amount` of token0 into token1
    /// @return The converted `amount` of token0 represented in terms of token1
    function convert0to1(uint256 amount, uint160 sqrtPriceX96) internal pure returns (uint256) {
        unchecked {
            // the tick 443636 is the maximum price where (price) * 2**192 fits into a uint256 (< 2**256-1)
            // above that tick, we are forced to reduce the amount of decimals in the final price by 2**64 to 2**128
            if (sqrtPriceX96 < type(uint128).max) {
                return Math.mulDiv192(amount, uint256(sqrtPriceX96) ** 2);
            } else {
                return Math.mulDiv128(amount, Math.mulDiv64(sqrtPriceX96, sqrtPriceX96));
            }
        }
    }

    /// @notice Convert an amount of token0 into an amount of token1 given the sqrtPriceX96 in a Uniswap pool defined as `sqrt(1/0)*2^96`.
    /// @dev Uses reduced precision after tick 443636 in order to accommodate the full range of ticks
    /// @param amount The amount of token0 to convert into token1
    /// @param sqrtPriceX96 The square root of the price at which to convert `amount` of token0 into token1
    /// @return The converted `amount` of token0 represented in terms of token1
    function convert0to1RoundingUp(
        uint256 amount,
        uint160 sqrtPriceX96
    ) internal pure returns (uint256) {
        unchecked {
            // the tick 443636 is the maximum price where (price) * 2**192 fits into a uint256 (< 2**256-1)
            // above that tick, we are forced to reduce the amount of decimals in the final price by 2**64 to 2**128
            if (sqrtPriceX96 < type(uint128).max) {
                return Math.mulDiv192RoundingUp(amount, uint256(sqrtPriceX96) ** 2);
            } else {
                return Math.mulDiv128RoundingUp(amount, Math.mulDiv64(sqrtPriceX96, sqrtPriceX96));
            }
        }
    }

    /// @notice Convert an amount of token1 into an amount of token0 given the sqrtPriceX96 in a Uniswap pool defined as `sqrt(1/0)*2^96`.
    /// @dev Uses reduced precision after tick 443636 in order to accommodate the full range of ticks.
    /// @param amount The amount of token1 to convert into token0
    /// @param sqrtPriceX96 The square root of the price at which to convert `amount` of token1 into token0
    /// @return The converted `amount` of token1 represented in terms of token0
    function convert1to0(uint256 amount, uint160 sqrtPriceX96) internal pure returns (uint256) {
        unchecked {
            // the tick 443636 is the maximum price where (price) * 2**192 fits into a uint256 (< 2**256-1)
            // above that tick, we are forced to reduce the amount of decimals in the final price by 2**64 to 2**128
            if (sqrtPriceX96 < type(uint128).max) {
                return Math.mulDiv(amount, 2 ** 192, uint256(sqrtPriceX96) ** 2);
            } else {
                return Math.mulDiv(amount, 2 ** 128, Math.mulDiv64(sqrtPriceX96, sqrtPriceX96));
            }
        }
    }

    /// @notice Convert an amount of token1 into an amount of token0 given the sqrtPriceX96 in a Uniswap pool defined as `sqrt(1/0)*2^96`.
    /// @dev Uses reduced precision after tick 443636 in order to accommodate the full range of ticks.
    /// @param amount The amount of token1 to convert into token0
    /// @param sqrtPriceX96 The square root of the price at which to convert `amount` of token1 into token0
    /// @return The converted `amount` of token1 represented in terms of token0
    function convert1to0RoundingUp(
        uint256 amount,
        uint160 sqrtPriceX96
    ) internal pure returns (uint256) {
        unchecked {
            // the tick 443636 is the maximum price where (price) * 2**192 fits into a uint256 (< 2**256-1)
            // above that tick, we are forced to reduce the amount of decimals in the final price by 2**64 to 2**128
            if (sqrtPriceX96 < type(uint128).max) {
                return Math.mulDivRoundingUp(amount, 2 ** 192, uint256(sqrtPriceX96) ** 2);
            } else {
                return
                    Math.mulDivRoundingUp(
                        amount,
                        2 ** 128,
                        Math.mulDiv64(sqrtPriceX96, sqrtPriceX96)
                    );
            }
        }
    }

    /// @notice Convert an amount of token0 into an amount of token1 given the sqrtPriceX96 in a Uniswap pool defined as `sqrt(1/0)*2^96`.
    /// @dev Uses reduced precision after tick 443636 in order to accommodate the full range of ticks.
    /// @param amount The amount of token0 to convert into token1
    /// @param sqrtPriceX96 The square root of the price at which to convert `amount` of token0 into token1
    /// @return The converted `amount` of token0 represented in terms of token1
    function convert0to1(int256 amount, uint160 sqrtPriceX96) internal pure returns (int256) {
        unchecked {
            // the tick 443636 is the maximum price where (price) * 2**192 fits into a uint256 (< 2**256-1)
            // above that tick, we are forced to reduce the amount of decimals in the final price by 2**64 to 2**128
            if (sqrtPriceX96 < type(uint128).max) {
                int256 absResult = Math
                    .mulDiv192(Math.absUint(amount), uint256(sqrtPriceX96) ** 2)
                    .toInt256();
                return amount < 0 ? -absResult : absResult;
            } else {
                int256 absResult = Math
                    .mulDiv128(Math.absUint(amount), Math.mulDiv64(sqrtPriceX96, sqrtPriceX96))
                    .toInt256();
                return amount < 0 ? -absResult : absResult;
            }
        }
    }

    /// @notice Convert an amount of token0 into an amount of token1 given the sqrtPriceX96 in a Uniswap pool defined as `sqrt(1/0)*2^96`.
    /// @dev Uses reduced precision after tick 443636 in order to accommodate the full range of ticks.
    /// @param amount The amount of token0 to convert into token1
    /// @param sqrtPriceX96 The square root of the price at which to convert `amount` of token0 into token1
    /// @return The converted `amount` of token0 represented in terms of token1
    function convert0to1RoundingUp(
        int256 amount,
        uint160 sqrtPriceX96
    ) internal pure returns (int256) {
        unchecked {
            // the tick 443636 is the maximum price where (price) * 2**192 fits into a uint256 (< 2**256-1)
            // above that tick, we are forced to reduce the amount of decimals in the final price by 2**64 to 2**128
            if (sqrtPriceX96 < type(uint128).max) {
                int256 absResult = Math
                    .mulDiv192RoundingUp(Math.absUint(amount), uint256(sqrtPriceX96) ** 2)
                    .toInt256();
                return amount < 0 ? -absResult : absResult;
            } else {
                int256 absResult = Math
                    .mulDiv128RoundingUp(
                        Math.absUint(amount),
                        Math.mulDiv64(sqrtPriceX96, sqrtPriceX96)
                    )
                    .toInt256();
                return amount < 0 ? -absResult : absResult;
            }
        }
    }

    /// @notice Convert an amount of token1 into an amount of token0 given the sqrtPriceX96 in a Uniswap pool defined as `sqrt(1/0)*2^96`.
    /// @dev Uses reduced precision after tick 443636 in order to accommodate the full range of ticks.
    /// @param amount The amount of token1 to convert into token0
    /// @param sqrtPriceX96 The square root of the price at which to convert `amount` of token1 into token0
    /// @return The converted `amount` of token1 represented in terms of token0
    function convert1to0(int256 amount, uint160 sqrtPriceX96) internal pure returns (int256) {
        unchecked {
            // the tick 443636 is the maximum price where (price) * 2**192 fits into a uint256 (< 2**256-1)
            // above that tick, we are forced to reduce the amount of decimals in the final price by 2**64 to 2**128
            if (sqrtPriceX96 < type(uint128).max) {
                int256 absResult = Math
                    .mulDiv(Math.absUint(amount), 2 ** 192, uint256(sqrtPriceX96) ** 2)
                    .toInt256();
                return amount < 0 ? -absResult : absResult;
            } else {
                int256 absResult = Math
                    .mulDiv(
                        Math.absUint(amount),
                        2 ** 128,
                        Math.mulDiv64(sqrtPriceX96, sqrtPriceX96)
                    )
                    .toInt256();
                return amount < 0 ? -absResult : absResult;
            }
        }
    }

    /// @notice Convert an amount of token1 into an amount of token0 given the sqrtPriceX96 in a Uniswap pool defined as `sqrt(1/0)*2^96`.
    /// @dev Uses reduced precision after tick 443636 in order to accommodate the full range of ticks.
    /// @param amount The amount of token1 to convert into token0
    /// @param sqrtPriceX96 The square root of the price at which to convert `amount` of token1 into token0
    /// @return The converted `amount` of token1 represented in terms of token0
    function convert1to0RoundingUp(
        int256 amount,
        uint160 sqrtPriceX96
    ) internal pure returns (int256) {
        unchecked {
            // the tick 443636 is the maximum price where (price) * 2**192 fits into a uint256 (< 2**256-1)
            // above that tick, we are forced to reduce the amount of decimals in the final price by 2**64 to 2**128
            if (sqrtPriceX96 < type(uint128).max) {
                int256 absResult = Math
                    .mulDivRoundingUp(Math.absUint(amount), 2 ** 192, uint256(sqrtPriceX96) ** 2)
                    .toInt256();
                return amount < 0 ? -absResult : absResult;
            } else {
                int256 absResult = Math
                    .mulDivRoundingUp(
                        Math.absUint(amount),
                        2 ** 128,
                        Math.mulDiv64(sqrtPriceX96, sqrtPriceX96)
                    )
                    .toInt256();
                return amount < 0 ? -absResult : absResult;
            }
        }
    }

    /// @notice Get a single collateral balance and requirement in terms of the lowest-priced token for a given set of (token0/token1) collateral balances and requirements.
    /// @param tokenData0 LeftRight encoded word with balance of token0 in the right slot, and required balance in left slot
    /// @param tokenData1 LeftRight encoded word with balance of token1 in the right slot, and required balance in left slot
    /// @param sqrtPriceX96 The price at which to compute the collateral value and requirements
    /// @return The combined collateral balance of `tokenData0` and `tokenData1` in terms of (token0 if `price(token1/token0) < 1` and vice versa)
    /// @return The combined required collateral threshold of `tokenData0` and `tokenData1` in terms of (token0 if `price(token1/token0) < 1` and vice versa)
    function getCrossBalances(
        LeftRightUnsigned tokenData0,
        LeftRightUnsigned tokenData1,
        uint160 sqrtPriceX96
    ) internal pure returns (uint256, uint256) {
        // convert values to the highest precision (lowest price) of the two tokens (token0 if price token1/token0 < 1 and vice versa)
        if (sqrtPriceX96 < Constants.FP96) {
            return (
                tokenData0.rightSlot() +
                    PanopticMath.convert1to0(tokenData1.rightSlot(), sqrtPriceX96),
                tokenData0.leftSlot() +
                    PanopticMath.convert1to0RoundingUp(tokenData1.leftSlot(), sqrtPriceX96)
            );
        }

        return (
            PanopticMath.convert0to1(tokenData0.rightSlot(), sqrtPriceX96) + tokenData1.rightSlot(),
            PanopticMath.convert0to1RoundingUp(tokenData0.leftSlot(), sqrtPriceX96) +
                tokenData1.leftSlot()
        );
    }

    /// @notice Compute the notional value (for `tokenType = 0` and `tokenType = 1`) represented by a given leg in an option position.
    /// @param tokenId The option position identifier
    /// @param positionSize The number of option contracts held in this position (each contract can control multiple tokens)
    /// @param legIndex The leg index of the option contract, can be {0,1,2,3}
    /// @param opening Whether this position is being opened or closed
    /// @return A LeftRight encoded variable containing the amount0 and the amount1 value controlled by this option position's leg
    function getAmountsMoved(
        TokenId tokenId,
        uint128 positionSize,
        uint256 legIndex,
        bool opening
    ) internal pure returns (LeftRightUnsigned) {
        uint128 amount0;
        uint128 amount1;

        bool hasWidth = tokenId.width(legIndex) != 0;
        // if the width is zero, add 1 to the width to allow liquidity amounts to be computes
        /// @dev this is just for accounting purposes, the actual tokenId will remain with a width = 0
        if (!hasWidth) {
            tokenId = tokenId.addWidth(2, legIndex);
        }

        LiquidityChunk liquidityChunk = getLiquidityChunk(tokenId, legIndex, positionSize);

        // Shorts round UP to ensure user pays enough (conservative for protocol)
        // Longs round DOWN to ensure user receives correct amount (conservative for protocol)
        if (
            (tokenId.isLong(legIndex) == 0 && opening) ||
            (tokenId.isLong(legIndex) != 0 && !opening) ||
            !hasWidth
        ) {
            amount0 = uint128(Math.getAmount0ForLiquidityUp(liquidityChunk));
            amount1 = uint128(Math.getAmount1ForLiquidityUp(liquidityChunk));
        } else {
            amount0 = uint128(Math.getAmount0ForLiquidity(liquidityChunk));
            amount1 = uint128(Math.getAmount1ForLiquidity(liquidityChunk));
        }
        return LeftRightUnsigned.wrap(amount0).addToLeftSlot(amount1);
    }

    /// @notice Compute the amount of funds that are moved to or removed from the Panoptic Pool when `tokenId` is created.
    /// @param tokenId The option position identifier
    /// @param positionSize The number of positions minted
    /// @param legIndex The leg index minted in this position, can be {0,1,2,3}
    /// @param opening Whether this position is being opened or closed
    /// @return longs A LeftRight-packed word containing the total amount of long positions
    /// @return shorts A LeftRight-packed word containing the amount of short positions
    function calculateIOAmounts(
        TokenId tokenId,
        uint128 positionSize,
        uint256 legIndex,
        bool opening
    ) internal pure returns (LeftRightSigned longs, LeftRightSigned shorts) {
        LeftRightUnsigned amountsMoved = getAmountsMoved(tokenId, positionSize, legIndex, opening);

        bool isShort = tokenId.isLong(legIndex) == 0;

        if (tokenId.tokenType(legIndex) == 0) {
            if (isShort) {
                // if option is short, increment shorts by contracts
                shorts = LeftRightSigned.wrap(0).addToRightSlot(
                    Math.toInt128(amountsMoved.rightSlot())
                );
            } else {
                // is option is long, increment longs by contracts
                longs = LeftRightSigned.wrap(0).addToRightSlot(
                    Math.toInt128(amountsMoved.rightSlot())
                );
            }
        } else {
            if (isShort) {
                // if option is short, increment shorts by notional
                shorts = LeftRightSigned.wrap(0).addToLeftSlot(
                    Math.toInt128(amountsMoved.leftSlot())
                );
            } else {
                // if option is long, increment longs by notional
                longs = LeftRightSigned.wrap(0).addToLeftSlot(
                    Math.toInt128(amountsMoved.leftSlot())
                );
            }
        }
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.24;
// Libraries
import {Errors} from "@libraries/Errors.sol";
import {Constants} from "@libraries/Constants.sol";
import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";
// Custom types
import {LiquidityChunk, LiquidityChunkLibrary} from "@types/LiquidityChunk.sol";

/// @title Core math library.
/// @author Axicon Labs Limited
/// @notice Contains general math helpers and functions
library Math {
    /// @notice This is equivalent to `type(uint256).max` — used in assembly blocks as a replacement.
    uint256 internal constant MAX_UINT256 = 2 ** 256 - 1;

    uint256 constant WAD = 1e18;
    int256 constant WAD_INT = int256(1e18);

    /// @dev ln(2).
    int256 internal constant LN_2_INT = 0.693147180559945309 ether;

    /// @dev ln(1e-18).
    int256 internal constant LN_WEI_INT = -41.446531673892822312 ether;

    /// @dev Above this bound, `wExp` is clipped to avoid overflowing when multiplied with 1 ether.
    /// @dev This upper bound corresponds to: ln(type(int256).max / 1e36) (scaled by WAD, floored).
    int256 internal constant WEXP_UPPER_BOUND = 93.859467695000404319 ether;

    /// @dev The value of wExp(`WEXP_UPPER_BOUND`).
    int256 internal constant WEXP_UPPER_VALUE =
        57716089161558943949701069502944508345128.422502756744429568 ether;

    /*//////////////////////////////////////////////////////////////
                          GENERAL MATH HELPERS
    //////////////////////////////////////////////////////////////*/

    /// @notice Compute the min of the incoming int24s `a` and `b`.
    /// @param a The first number
    /// @param b The second number
    /// @return The min of `a` and `b`: min(a, b), e.g.: min(4, 1) = 1
    function min24(int24 a, int24 b) internal pure returns (int24) {
        return a < b ? a : b;
    }

    /// @notice Compute the max of the incoming int24s `a` and `b`.
    /// @param a The first number
    /// @param b The second number
    /// @return The max of `a` and `b`: max(a, b), e.g.: max(4, 1) = 4
    function max24(int24 a, int24 b) internal pure returns (int24) {
        return a > b ? a : b;
    }

    /// @notice Compute the min of the incoming `a` and `b`.
    /// @param a The first number
    /// @param b The second number
    /// @return The min of `a` and `b`: min(a, b), e.g.: min(4, 1) = 1
    function min(uint256 a, uint256 b) internal pure returns (uint256) {
        return a < b ? a : b;
    }

    /// @notice Compute the min of the incoming `a` and `b`.
    /// @param a The first number
    /// @param b The second number
    /// @return The min of `a` and `b`: min(a, b), e.g.: min(4, 1) = 1
    function min(int256 a, int256 b) internal pure returns (int256) {
        return a < b ? a : b;
    }

    /// @notice Compute the max of the incoming `a` and `b`.
    /// @param a The first number
    /// @param b The second number
    /// @return The max of `a` and `b`: max(a, b), e.g.: max(4, 1) = 4
    function max(uint256 a, uint256 b) internal pure returns (uint256) {
        return a > b ? a : b;
    }

    /// @notice Compute the max of the incoming `a` and `b`.
    /// @param a The first number
    /// @param b The second number
    /// @return The max of `a` and `b`: max(a, b), e.g.: max(4, 1) = 4
    function max(int256 a, int256 b) internal pure returns (int256) {
        return a > b ? a : b;
    }

    /// @notice Compute the absolute value of an integer.
    /// @param x The incoming *signed* integer to take the absolute value of
    /// @dev Does not support `type(int256).min` and will revert (`type(int256).max = abs(type(int256).min) - 1`).
    /// @return The absolute value of `x`, e.g. abs(-4) = 4
    function abs(int256 x) internal pure returns (int256) {
        return x > 0 ? x : -x;
    }

    /// @notice Compute the absolute value of an integer.
    /// @param x The incoming *signed* integer to take the absolute value of
    /// @dev Supports `type(int256).min` because the corresponding value can fit in a uint (unlike `type(int256).max`).
    /// @return The absolute value of `x`, e.g. abs(-4) = 4
    function absUint(int256 x) internal pure returns (uint256) {
        unchecked {
            return x > 0 ? uint256(x) : uint256(-x);
        }
    }

    /// @notice Returns the index of the most significant nibble of the 160-bit number,
    /// where the least significant nibble is at index 0 and the most significant nibble is at index 39.
    /// @param x The value for which to compute the most significant nibble
    /// @return r The index of the most significant nibble (default: 0)
    function mostSignificantNibble(uint160 x) internal pure returns (uint256 r) {
        unchecked {
            if (x >= 0x100000000000000000000000000000000) {
                x >>= 128;
                r += 32;
            }
            if (x >= 0x10000000000000000) {
                x >>= 64;
                r += 16;
            }
            if (x >= 0x100000000) {
                x >>= 32;
                r += 8;
            }
            if (x >= 0x10000) {
                x >>= 16;
                r += 4;
            }
            if (x >= 0x100) {
                x >>= 8;
                r += 2;
            }
            if (x >= 0x10) {
                r += 1;
            }
        }
    }

    /*//////////////////////////////////////////////////////////////
                               TICK MATH
    //////////////////////////////////////////////////////////////*/

    /// @notice Computes a tick that will require approximately `amount` of token0 to create a `tickSpacing`-wide position with `maxLiquidityPerTick` at `tickUpper = tick` in Uniswap.
    /// @dev This function can have a maximum of two ticks of error from one of the ticks with `amount(tickRes + 2) < amount < amount(tickRes - 2)`.
    /// @dev `tickSpacing` is assumed to be within the range (0, 32768)
    /// @dev `maxLiquidityPerTick` for `s=tickSpacing` should be defined by `(2^128 - 1) / ((887272/s) - (-887272/s) + 1)`
    /// @param amount The desired amount of token0 required to fill the returned tick
    /// @param tickSpacing The spacing between initializable ticks in the Uniswap pool
    /// @param maxLiquidityPerTick The maximum liquidity that can reference any given tick in the Uniswap pool
    /// @return A tick that will require approximately `amount` of token0 to create a `tickSpacing`-wide position with `maxLiquidityPerTick` at `tickUpper = tick`
    function getApproxTickWithMaxAmount(
        uint256 amount,
        int24 tickSpacing,
        uint256 maxLiquidityPerTick
    ) internal pure returns (int24) {
        unchecked {
            // abs(max_error) ≈ 2^-13 * log₂(√1.0001)⁻¹ ≈ -1.70234
            return
                int24(
                    int256(
                        Math.log_Sqrt1p0001MantissaRect(
                            Math.mulDivCapped(
                                amount,
                                2 ** 224,
                                (maxLiquidityPerTick *
                                    (Math.getSqrtRatioAtTick(tickSpacing) - 2 ** 96)),
                                128
                            ),
                            13
                        )
                    )
                );
        }
    }

    /// @notice Computes the maximum liquidity that is allowed to reference any given tick in a Uniswap V3 pool with `tickSpacing`.
    /// @param tickSpacing The spacing between initializable ticks in the Uniswap V3 pool
    /// @return The maximum liquidity that can reference any given tick in the Uniswap V3 pool
    function getMaxLiquidityPerTick(int24 tickSpacing) internal pure returns (uint128) {
        unchecked {
            // forge-lint: disable-next-line(divide-before-multiply)
            return type(uint128).max / uint24((Constants.MAX_POOL_TICK / tickSpacing) * 2 + 1);
        }
    }

    /// @notice Calculates `1.0001^(tick/2)` as an X96 number.
    /// @dev Will revert if `abs(tick) > 887272`.
    /// @param tick Value of the tick for which `sqrt(1.0001^tick)` is calculated
    /// @return A Q64.96 number representing the sqrt price at the provided tick
    function getSqrtRatioAtTick(int24 tick) internal pure returns (uint160) {
        unchecked {
            uint256 absTick = tick < 0 ? uint256(-int256(tick)) : uint256(int256(tick));
            if (absTick > uint256(int256(Constants.MAX_POOL_TICK))) revert Errors.InvalidTick();

            // sqrt(1.0001^(-absTick)) = ∏ sqrt(1.0001^(-bit_i))
            // ex: absTick = 100 = binary 1100100, so sqrt(1.0001^-100) = sqrt(1.0001^-64) * sqrt(1.0001^-32) * sqrt(1.0001^-4)
            // constants are 2^128/(sqrt(1.0001)^bit_i) rounded half-up

            // if the first bit is 0, initialize sqrtR to 1 (2^128)
            uint256 sqrtR = absTick & 0x1 != 0
                ? 0xfffcb933bd6fad37aa2d162d1a594001
                : 0x100000000000000000000000000000000;

            if (absTick & 0x2 != 0) sqrtR = (sqrtR * 0xfff97272373d413259a46990580e213a) >> 128;

            if (absTick & 0x4 != 0) sqrtR = (sqrtR * 0xfff2e50f5f656932ef12357cf3c7fdcc) >> 128;

            if (absTick & 0x8 != 0) sqrtR = (sqrtR * 0xffe5caca7e10e4e61c3624eaa0941cd0) >> 128;

            if (absTick & 0x10 != 0) sqrtR = (sqrtR * 0xffcb9843d60f6159c9db58835c926644) >> 128;

            if (absTick & 0x20 != 0) sqrtR = (sqrtR * 0xff973b41fa98c081472e6896dfb254c0) >> 128;

            if (absTick & 0x40 != 0) sqrtR = (sqrtR * 0xff2ea16466c96a3843ec78b326b52861) >> 128;

            if (absTick & 0x80 != 0) sqrtR = (sqrtR * 0xfe5dee046a99a2a811c461f1969c3053) >> 128;

            if (absTick & 0x100 != 0) sqrtR = (sqrtR * 0xfcbe86c7900a88aedcffc83b479aa3a4) >> 128;

            if (absTick & 0x200 != 0) sqrtR = (sqrtR * 0xf987a7253ac413176f2b074cf7815e54) >> 128;

            if (absTick & 0x400 != 0) sqrtR = (sqrtR * 0xf3392b0822b70005940c7a398e4b70f3) >> 128;

            if (absTick & 0x800 != 0) sqrtR = (sqrtR * 0xe7159475a2c29b7443b29c7fa6e889d9) >> 128;

            if (absTick & 0x1000 != 0) sqrtR = (sqrtR * 0xd097f3bdfd2022b8845ad8f792aa5825) >> 128;

            if (absTick & 0x2000 != 0) sqrtR = (sqrtR * 0xa9f746462d870fdf8a65dc1f90e061e5) >> 128;

            if (absTick & 0x4000 != 0) sqrtR = (sqrtR * 0x70d869a156d2a1b890bb3df62baf32f7) >> 128;

            if (absTick & 0x8000 != 0) sqrtR = (sqrtR * 0x31be135f97d08fd981231505542fcfa6) >> 128;

            if (absTick & 0x10000 != 0) sqrtR = (sqrtR * 0x9aa508b5b7a84e1c677de54f3e99bc9) >> 128;

            if (absTick & 0x20000 != 0) sqrtR = (sqrtR * 0x5d6af8dedb81196699c329225ee604) >> 128;

            if (absTick & 0x40000 != 0) sqrtR = (sqrtR * 0x2216e584f5fa1ea926041bedfe98) >> 128;

            if (absTick & 0x80000 != 0) sqrtR = (sqrtR * 0x48a170391f7dc42444e8fa2) >> 128;

            // 2^128 * sqrt(1.0001^x) = 2^128 / sqrt(1.0001^-x)
            if (tick > 0) sqrtR = type(uint256).max / sqrtR;

            // Downcast + rounding up to keep is consistent with Uniswap's
            return uint160((sqrtR >> 32) + (sqrtR % (1 << 32) == 0 ? 0 : 1));
        }
    }

    /// @notice Approximates the absolute value of log base `sqrt(1.0001)` for a number in (0, 1) (`argX128/2^128`) with `precision` bits of precision.
    /// @param argX128 The Q128.128 fixed-point number in the range (0, 1) to calculate the log of
    /// @param precision The bits of precision with which to compute the result, max 63 (`err <≈ 2^-precision * log₂(√1.0001)⁻¹`)
    /// @return The absolute value of log with base `sqrt(1.0001)` for `argX128/2^128`
    function log_Sqrt1p0001MantissaRect(
        uint256 argX128,
        uint256 precision
    ) internal pure returns (uint256) {
        unchecked {
            // =[log₂(x)] =MSB(x)
            uint256 log2_res = FixedPointMathLib.log2(argX128);

            // Normalize argX128 to [1, 2)
            // x_normal = x / 2^[log₂(x)]
            // = 1.a₁a₂a₃... = 2^(0.b₁b₂b₃...)
            // log₂(x_normal) = log₂(x / 2^⌊log₂(x)⌋)
            // log₂(x_normal) = log₂(x) - log₂(2^⌊log₂(x)⌋)
            // log₂(x_normal) = log₂(x) - ⌊log₂(x)⌋
            // log₂(x) = log₂(x_normal) + ⌊log₂(x)⌋
            argX128 <<= (127 - log2_res);

            // =[log₂(x)] * 2^64
            log2_res = (128 - log2_res) << 64;

            // log₂(x_normal) = 0.b₁b₂b₃...
            // x_normal = (1.a₁a₂a₃...) = 2^(0.b₁b₂b₃...)
            // x_normal² = (1.a₁a₂a₃...)² = (2^(0.b₁b₂b₃...))²
            // = 2^(0.b₁b₂b₃... * 2)
            // = 2^(b₁ + 0.b₂b₃...)
            // if bᵢ = 1, renormalize x_normal² to [1, 2):
            // 2^(b₁ + 0.b₂b₃...) / 2^b₁ = 2^((b₁ - 1).b₂b₃...)
            // = 2^(0.b₂b₃...)
            // error = [0, 2⁻ⁿ)
            uint256 iterBound = 63 - precision;
            for (uint256 i = 63; i > iterBound; i--) {
                argX128 = (argX128 ** 2) >> 127;
                uint256 bit = argX128 >> 128;
                log2_res -= bit << i;
                argX128 >>= bit;
            }

            // log₍√₁.₀₀₀₁₎(x) = log₂(x) / log₂(√1.0001)
            // 2^64 / log₂(√1.0001) ≈ 255738959000112593413423
            return (log2_res * 255738959000112593413423) / 2 ** 128;
        }
    }

    /*//////////////////////////////////////////////////////////////
                           LIQUIDITY AMOUNTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Calculates the amount of token0 received for a given LiquidityChunk.
    /// @param liquidityChunk A specification for a liquidity chunk in Uniswap containing `liquidity`, `tickLower`, and `tickUpper`
    /// @return The amount of token0 represented by `liquidityChunk` when `currentTick < tickLower`
    function getAmount0ForLiquidityUp(
        LiquidityChunk liquidityChunk
    ) internal pure returns (uint256) {
        uint160 lowPriceX96 = getSqrtRatioAtTick(liquidityChunk.tickLower());
        uint160 highPriceX96 = getSqrtRatioAtTick(liquidityChunk.tickUpper());
        unchecked {
            return
                mulDivRoundingUp(
                    mulDivRoundingUp(
                        uint256(liquidityChunk.liquidity()) << 96,
                        highPriceX96 - lowPriceX96,
                        highPriceX96
                    ),
                    1,
                    lowPriceX96
                );
        }
    }

    /// @notice Calculates the amount of token1 received for a given LiquidityChunk.
    /// @param liquidityChunk A specification for a liquidity chunk in Uniswap containing `liquidity`, `tickLower`, and `tickUpper`
    /// @return The amount of token1 represented by `liquidityChunk` when `currentTick > tickUpper`
    function getAmount1ForLiquidityUp(
        LiquidityChunk liquidityChunk
    ) internal pure returns (uint256) {
        uint160 lowPriceX96 = getSqrtRatioAtTick(liquidityChunk.tickLower());
        uint160 highPriceX96 = getSqrtRatioAtTick(liquidityChunk.tickUpper());

        unchecked {
            return mulDiv96RoundingUp(liquidityChunk.liquidity(), highPriceX96 - lowPriceX96);
        }
    }

    /// @notice Calculates the amount of token0 received for a given LiquidityChunk.
    /// @param liquidityChunk A specification for a liquidity chunk in Uniswap containing `liquidity`, `tickLower`, and `tickUpper`
    /// @return The amount of token0 represented by `liquidityChunk` when `currentTick < tickLower`
    function getAmount0ForLiquidity(LiquidityChunk liquidityChunk) internal pure returns (uint256) {
        uint160 lowPriceX96 = getSqrtRatioAtTick(liquidityChunk.tickLower());
        uint160 highPriceX96 = getSqrtRatioAtTick(liquidityChunk.tickUpper());
        unchecked {
            return
                mulDiv(
                    uint256(liquidityChunk.liquidity()) << 96,
                    highPriceX96 - lowPriceX96,
                    highPriceX96
                ) / lowPriceX96;
        }
    }

    /// @notice Calculates the amount of token1 received for a given LiquidityChunk.
    /// @param liquidityChunk A specification for a liquidity chunk in Uniswap containing `liquidity`, `tickLower`, and `tickUpper`
    /// @return The amount of token1 represented by `liquidityChunk` when `currentTick > tickUpper`
    function getAmount1ForLiquidity(LiquidityChunk liquidityChunk) internal pure returns (uint256) {
        uint160 lowPriceX96 = getSqrtRatioAtTick(liquidityChunk.tickLower());
        uint160 highPriceX96 = getSqrtRatioAtTick(liquidityChunk.tickUpper());

        unchecked {
            return mulDiv96(liquidityChunk.liquidity(), highPriceX96 - lowPriceX96);
        }
    }

    /// @notice Calculates the amount of token0 and token1 received for a given LiquidityChunk at the provided `currentTick`.
    /// @param currentTick The tick at which to evaluate `liquidityChunk`
    /// @param liquidityChunk A specification for a liquidity chunk in Uniswap containing `liquidity`, `tickLower`, and `tickUpper`
    /// @return amount0 The amount of token0 represented by `liquidityChunk` at `currentTick`
    /// @return amount1 The amount of token1 represented by `liquidityChunk` at `currentTick`
    function getAmountsForLiquidity(
        int24 currentTick,
        LiquidityChunk liquidityChunk
    ) internal pure returns (uint256 amount0, uint256 amount1) {
        if (currentTick <= liquidityChunk.tickLower()) {
            amount0 = getAmount0ForLiquidity(liquidityChunk);
        } else if (currentTick >= liquidityChunk.tickUpper()) {
            amount1 = getAmount1ForLiquidity(liquidityChunk);
        } else {
            amount0 = getAmount0ForLiquidity(liquidityChunk.updateTickLower(currentTick));
            amount1 = getAmount1ForLiquidity(liquidityChunk.updateTickUpper(currentTick));
        }
    }

    /// @notice Returns a LiquidityChunk at the provided tick range with `liquidity` corresponding to `amount0`.
    /// @param tickLower The lower tick of the chunk
    /// @param tickUpper The upper tick of the chunk
    /// @param amount0 The amount of token0
    /// @return A LiquidityChunk with `tickLower`, `tickUpper`, and the calculated amount of liquidity for `amount0`
    function getLiquidityForAmount0(
        int24 tickLower,
        int24 tickUpper,
        uint256 amount0
    ) internal pure returns (LiquidityChunk) {
        unchecked {
            uint160 lowPriceX96 = getSqrtRatioAtTick(tickLower);
            uint160 highPriceX96 = getSqrtRatioAtTick(tickUpper);

            uint256 liquidity = mulDiv(
                amount0,
                mulDiv96(highPriceX96, lowPriceX96),
                highPriceX96 - lowPriceX96
            );

            // This check guarantees the following uint128 cast is safe.
            if (liquidity > type(uint128).max) revert Errors.LiquidityTooHigh();

            // casting to 'uint128' is safe because of the liquidity > type(uint128).max check above
            // forge-lint: disable-next-line(unsafe-typecast)
            return LiquidityChunkLibrary.createChunk(tickLower, tickUpper, uint128(liquidity));
        }
    }

    /// @notice Returns a LiquidityChunk at the provided tick range with `liquidity` corresponding to `amount1`.
    /// @param tickLower The lower tick of the chunk
    /// @param tickUpper The upper tick of the chunk
    /// @param amount1 The amount of token1
    /// @return A LiquidityChunk with `tickLower`, `tickUpper`, and the calculated amount of liquidity for `amount1`
    function getLiquidityForAmount1(
        int24 tickLower,
        int24 tickUpper,
        uint256 amount1
    ) internal pure returns (LiquidityChunk) {
        unchecked {
            uint160 lowPriceX96 = getSqrtRatioAtTick(tickLower);
            uint160 highPriceX96 = getSqrtRatioAtTick(tickUpper);

            uint256 liquidity = mulDiv(amount1, Constants.FP96, highPriceX96 - lowPriceX96);

            // This check guarantees the following uint128 cast is safe.
            if (liquidity > type(uint128).max) revert Errors.LiquidityTooHigh();

            return LiquidityChunkLibrary.createChunk(tickLower, tickUpper, uint128(liquidity));
        }
    }

    /*//////////////////////////////////////////////////////////////
                                CASTING
    //////////////////////////////////////////////////////////////*/

    /// @notice Downcast uint256 to uint128. Revert on overflow or underflow.
    /// @param toDowncast The uint256 to be downcasted
    /// @return downcastedInt `toDowncast` downcasted to uint128
    function toUint128(uint256 toDowncast) internal pure returns (uint128 downcastedInt) {
        if ((downcastedInt = uint128(toDowncast)) != toDowncast) revert Errors.CastingError();
    }

    /// @notice Downcast uint256 to uint128, but cap at type(uint128).max on overflow.
    /// @param toDowncast The uint256 to be downcasted
    /// @return downcastedInt `toDowncast` downcasted to uint128
    function toUint128Capped(uint256 toDowncast) internal pure returns (uint128 downcastedInt) {
        if ((downcastedInt = uint128(toDowncast)) != toDowncast) {
            downcastedInt = type(uint128).max;
        }
    }

    /// @notice Downcast uint128 to int128.
    /// @param toCast The uint256 to be downcasted
    /// @return downcastedInt `toDowncast` downcasted to int128
    function toInt128(uint128 toCast) internal pure returns (int128 downcastedInt) {
        if ((downcastedInt = int128(toCast)) < 0) revert Errors.CastingError();
    }

    /// @notice Cast an int256 to an int128, revert on overflow or underflow.
    /// @param toCast The int256 to be downcasted
    /// @return downcastedInt `toCast` downcasted to int128
    function toInt128(int256 toCast) internal pure returns (int128 downcastedInt) {
        if (!((downcastedInt = int128(toCast)) == toCast)) revert Errors.CastingError();
    }

    /// @notice Cast a uint256 to an int256, revert on overflow.
    /// @param toCast The uint256 to be downcasted
    /// @return `toCast` downcasted to int256
    function toInt256(uint256 toCast) internal pure returns (int256) {
        if (toCast > uint256(type(int256).max)) revert Errors.CastingError();
        return int256(toCast);
    }

    /*//////////////////////////////////////////////////////////////
                                 MULDIV
    //////////////////////////////////////////////////////////////*/

    /// @notice Calculates `floor(a×b÷denominator)` with full precision. Throws if result overflows a uint256 or `denominator == 0`.
    /// @param a The multiplicand
    /// @param b The multiplier
    /// @param denominator The divisor
    /// @return result The 256-bit result
    /// @dev Credit to Remco Bloemen under MIT license https://xn--2-umb.com/21/muldiv for this and all following `mulDiv` functions.
    function mulDiv(
        uint256 a,
        uint256 b,
        uint256 denominator
    ) internal pure returns (uint256 result) {
        unchecked {
            // 512-bit multiply [prod1 prod0] = a * b
            // Compute the product mod 2**256 and mod 2**256 - 1
            // then use the Chinese Remainder Theorem to reconstruct
            // the 512 bit result. The result is stored in two 256
            // variables such that product = prod1 * 2**256 + prod0
            uint256 prod0; // Least significant 256 bits of the product
            uint256 prod1; // Most significant 256 bits of the product
            assembly ("memory-safe") {
                let mm := mulmod(a, b, not(0))
                prod0 := mul(a, b)
                prod1 := sub(sub(mm, prod0), lt(mm, prod0))
            }

            // Handle non-overflow cases, 256 by 256 division
            if (prod1 == 0) {
                require(denominator > 0);
                assembly ("memory-safe") {
                    result := div(prod0, denominator)
                }
                return result;
            }

            // Make sure the result is less than 2**256.
            // Also prevents denominator == 0
            require(denominator > prod1);

            ///////////////////////////////////////////////
            // 512 by 256 division.
            ///////////////////////////////////////////////

            // Make division exact by subtracting the remainder from [prod1 prod0]
            // Compute remainder using mulmod
            uint256 remainder;
            assembly ("memory-safe") {
                remainder := mulmod(a, b, denominator)
            }
            // Subtract 256 bit number from 512 bit number
            assembly ("memory-safe") {
                prod1 := sub(prod1, gt(remainder, prod0))
                prod0 := sub(prod0, remainder)
            }

            // Factor powers of two out of denominator
            // Compute largest power of two divisor of denominator.
            // Always >= 1.
            uint256 twos = (0 - denominator) & denominator;
            // Divide denominator by power of two
            assembly ("memory-safe") {
                denominator := div(denominator, twos)
            }

            // Divide [prod1 prod0] by the factors of two
            assembly ("memory-safe") {
                prod0 := div(prod0, twos)
            }
            // Shift in bits from prod1 into prod0. For this we need
            // to flip `twos` such that it is 2**256 / twos.
            // If twos is zero, then it becomes one
            assembly ("memory-safe") {
                twos := add(div(sub(0, twos), twos), 1)
            }
            prod0 |= prod1 * twos;

            // Invert denominator mod 2**256
            // Now that denominator is an odd number, it has an inverse
            // modulo 2**256 such that denominator * inv = 1 mod 2**256.
            // Compute the inverse by starting with a seed that is correct
            // correct for four bits. That is, denominator * inv = 1 mod 2**4
            uint256 inv = (3 * denominator) ^ 2;
            // Now use Newton-Raphson iteration to improve the precision.
            // Thanks to Hensel's lifting lemma, this also works in modular
            // arithmetic, doubling the correct bits in each step.
            inv *= 2 - denominator * inv; // inverse mod 2**8
            inv *= 2 - denominator * inv; // inverse mod 2**16
            inv *= 2 - denominator * inv; // inverse mod 2**32
            inv *= 2 - denominator * inv; // inverse mod 2**64
            inv *= 2 - denominator * inv; // inverse mod 2**128
            inv *= 2 - denominator * inv; // inverse mod 2**256

            // Because the division is now exact we can divide by multiplying
            // with the modular inverse of denominator. This will give us the
            // correct result modulo 2**256. Since the preconditions guarantee
            // that the outcome is less than 2**256, this is the final result.
            // We don't need to compute the high bits of the result and prod1
            // is no longer required.
            result = prod0 * inv;
        }
    }

    /// @notice Calculates `min(floor(a×b÷denominator), 2^256-1)` with full precision.
    /// @param a The multiplicand
    /// @param b The multiplier
    /// @param denominator The divisor
    /// @return result The 256-bit result
    function mulDivCapped(
        uint256 a,
        uint256 b,
        uint256 denominator
    ) internal pure returns (uint256 result) {
        unchecked {
            // 512-bit multiply [prod1 prod0] = a * b
            // Compute the product mod 2**256 and mod 2**256 - 1
            // then use the Chinese Remainder Theorem to reconstruct
            // the 512 bit result. The result is stored in two 256
            // variables such that product = prod1 * 2**256 + prod0
            uint256 prod0; // Least significant 256 bits of the product
            uint256 prod1; // Most significant 256 bits of the product
            assembly ("memory-safe") {
                let mm := mulmod(a, b, not(0))
                prod0 := mul(a, b)
                prod1 := sub(sub(mm, prod0), lt(mm, prod0))
            }

            // Handle non-overflow cases, 256 by 256 division
            if (prod1 == 0) {
                require(denominator > 0);
                assembly ("memory-safe") {
                    result := div(prod0, denominator)
                }
                return result;
            }

            if (denominator <= prod1) return type(uint256).max;

            ///////////////////////////////////////////////
            // 512 by 256 division.
            ///////////////////////////////////////////////

            // Make division exact by subtracting the remainder from [prod1 prod0]
            // Compute remainder using mulmod
            uint256 remainder;
            assembly ("memory-safe") {
                remainder := mulmod(a, b, denominator)
            }
            // Subtract 256 bit number from 512 bit number
            assembly ("memory-safe") {
                prod1 := sub(prod1, gt(remainder, prod0))
                prod0 := sub(prod0, remainder)
            }

            // Factor powers of two out of denominator
            // Compute largest power of two divisor of denominator.
            // Always >= 1.
            uint256 twos = (0 - denominator) & denominator;
            // Divide denominator by power of two
            assembly ("memory-safe") {
                denominator := div(denominator, twos)
            }

            // Divide [prod1 prod0] by the factors of two
            assembly ("memory-safe") {
                prod0 := div(prod0, twos)
            }
            // Shift in bits from prod1 into prod0. For this we need
            // to flip `twos` such that it is 2**256 / twos.
            // If twos is zero, then it becomes one
            assembly ("memory-safe") {
                twos := add(div(sub(0, twos), twos), 1)
            }
            prod0 |= prod1 * twos;

            // Invert denominator mod 2**256
            // Now that denominator is an odd number, it has an inverse
            // modulo 2**256 such that denominator * inv = 1 mod 2**256.
            // Compute the inverse by starting with a seed that is correct
            // correct for four bits. That is, denominator * inv = 1 mod 2**4
            uint256 inv = (3 * denominator) ^ 2;
            // Now use Newton-Raphson iteration to improve the precision.
            // Thanks to Hensel's lifting lemma, this also works in modular
            // arithmetic, doubling the correct bits in each step.
            inv *= 2 - denominator * inv; // inverse mod 2**8
            inv *= 2 - denominator * inv; // inverse mod 2**16
            inv *= 2 - denominator * inv; // inverse mod 2**32
            inv *= 2 - denominator * inv; // inverse mod 2**64
            inv *= 2 - denominator * inv; // inverse mod 2**128
            inv *= 2 - denominator * inv; // inverse mod 2**256

            // Because the division is now exact we can divide by multiplying
            // with the modular inverse of denominator. This will give us the
            // correct result modulo 2**256. Since the preconditions guarantee
            // that the outcome is less than 2**256, this is the final result.
            // We don't need to compute the high bits of the result and prod1
            // is no longer required.
            result = prod0 * inv;
        }
    }

    /// @notice Calculates `min(floor(a×b÷denominator), 2^power-1)` with full precision.
    /// @param a The multiplicand
    /// @param b The multiplier
    /// @param denominator The divisor
    /// @param power The upper bound of the open interval representing the range of this function, given by `2^power`
    /// @return result The 256-bit result
    function mulDivCapped(
        uint256 a,
        uint256 b,
        uint256 denominator,
        uint256 power
    ) internal pure returns (uint256 result) {
        unchecked {
            // 512-bit multiply [prod1 prod0] = a * b
            // Compute the product mod 2**256 and mod 2**256 - 1
            // then use the Chinese Remainder Theorem to reconstruct
            // the 512 bit result. The result is stored in two 256
            // variables such that product = prod1 * 2**256 + prod0
            uint256 prod0; // Least significant 256 bits of the product
            uint256 prod1; // Most significant 256 bits of the product
            assembly ("memory-safe") {
                let mm := mulmod(a, b, not(0))
                prod0 := mul(a, b)
                prod1 := sub(sub(mm, prod0), lt(mm, prod0))
            }
            // Handle non-overflow cases, 256 by 256 division
            if (prod1 == 0) {
                require(denominator > 0);
                assembly ("memory-safe") {
                    result := div(prod0, denominator)
                }
                return Math.min(result, 2 ** power - 1);
            }

            if (denominator >> (256 - power) <= prod1) return 2 ** power - 1;

            ///////////////////////////////////////////////
            // 512 by 256 division.
            ///////////////////////////////////////////////

            // Make division exact by subtracting the remainder from [prod1 prod0]
            // Compute remainder using mulmod
            uint256 remainder;
            assembly ("memory-safe") {
                remainder := mulmod(a, b, denominator)
            }
            // Subtract 256 bit number from 512 bit number
            assembly ("memory-safe") {
                prod1 := sub(prod1, gt(remainder, prod0))
                prod0 := sub(prod0, remainder)
            }

            // Factor powers of two out of denominator
            // Compute largest power of two divisor of denominator.
            // Always >= 1.
            uint256 twos = (0 - denominator) & denominator;
            // Divide denominator by power of two
            assembly ("memory-safe") {
                denominator := div(denominator, twos)
            }

            // Divide [prod1 prod0] by the factors of two
            assembly ("memory-safe") {
                prod0 := div(prod0, twos)
            }
            // Shift in bits from prod1 into prod0. For this we need
            // to flip `twos` such that it is 2**256 / twos.
            // If twos is zero, then it becomes one
            assembly ("memory-safe") {
                twos := add(div(sub(0, twos), twos), 1)
            }
            prod0 |= prod1 * twos;

            // Invert denominator mod 2**256
            // Now that denominator is an odd number, it has an inverse
            // modulo 2**256 such that denominator * inv = 1 mod 2**256.
            // Compute the inverse by starting with a seed that is correct
            // correct for four bits. That is, denominator * inv = 1 mod 2**4
            uint256 inv = (3 * denominator) ^ 2;
            // Now use Newton-Raphson iteration to improve the precision.
            // Thanks to Hensel's lifting lemma, this also works in modular
            // arithmetic, doubling the correct bits in each step.
            inv *= 2 - denominator * inv; // inverse mod 2**8
            inv *= 2 - denominator * inv; // inverse mod 2**16
            inv *= 2 - denominator * inv; // inverse mod 2**32
            inv *= 2 - denominator * inv; // inverse mod 2**64
            inv *= 2 - denominator * inv; // inverse mod 2**128
            inv *= 2 - denominator * inv; // inverse mod 2**256

            // Because the division is now exact we can divide by multiplying
            // with the modular inverse of denominator. This will give us the
            // correct result modulo 2**256. Since the preconditions guarantee
            // that the outcome is less than 2**256, this is the final result.
            // We don't need to compute the high bits of the result and prod1
            // is no longer required.
            result = prod0 * inv;
        }
    }

    /// @notice Calculates `ceil(a×b÷denominator)` with full precision. Throws if result overflows a uint256 or `denominator == 0`.
    /// @param a The multiplicand
    /// @param b The multiplier
    /// @param denominator The divisor
    /// @return result The 256-bit result
    function mulDivRoundingUp(
        uint256 a,
        uint256 b,
        uint256 denominator
    ) internal pure returns (uint256 result) {
        unchecked {
            result = mulDiv(a, b, denominator);
            if (mulmod(a, b, denominator) > 0) {
                require(result < type(uint256).max);
                result++;
            }
        }
    }

    /// @notice Calculates `floor(a×b÷2^64)` with full precision. Throws if result overflows a uint256.
    /// @param a The multiplicand
    /// @param b The multiplier
    /// @return The 256-bit result
    function mulDiv64(uint256 a, uint256 b) internal pure returns (uint256) {
        unchecked {
            // 512-bit multiply [prod1 prod0] = a * b
            // Compute the product mod 2**256 and mod 2**256 - 1
            // then use the Chinese Remainder Theorem to reconstruct
            // the 512 bit result. The result is stored in two 256
            // variables such that product = prod1 * 2**256 + prod0
            uint256 prod0; // Least significant 256 bits of the product
            uint256 prod1; // Most significant 256 bits of the product
            assembly ("memory-safe") {
                let mm := mulmod(a, b, not(0))
                prod0 := mul(a, b)
                prod1 := sub(sub(mm, prod0), lt(mm, prod0))
            }

            // Handle non-overflow cases, 256 by 256 division
            if (prod1 == 0) {
                uint256 res;
                assembly ("memory-safe") {
                    // Right shift by n is equivalent and 2 gas cheaper than division by 2^n
                    res := shr(64, prod0)
                }
                return res;
            }

            // Make sure the result is less than 2**256.
            require(2 ** 64 > prod1);

            ///////////////////////////////////////////////
            // 512 by 256 division.
            ///////////////////////////////////////////////

            // Make division exact by subtracting the remainder from [prod1 prod0]
            // Compute remainder using mulmod
            uint256 remainder;
            assembly ("memory-safe") {
                remainder := mulmod(a, b, 0x10000000000000000)
            }
            // Subtract 256 bit number from 512 bit number
            assembly ("memory-safe") {
                prod1 := sub(prod1, gt(remainder, prod0))
                prod0 := sub(prod0, remainder)
            }

            // Divide [prod1 prod0] by the factors of two (note that this is just 2**96 since the denominator is a power of 2 itself)
            assembly ("memory-safe") {
                // Right shift by n is equivalent and 2 gas cheaper than division by 2^n
                prod0 := shr(64, prod0)
            }
            // Shift in bits from prod1 into prod0. For this we need
            // to flip `twos` such that it is 2**256 / twos.
            // If twos is zero, then it becomes one
            // Note that this is just 2**192 since 2**256 over the fixed denominator (2**64) equals 2**192
            prod0 |= prod1 * 2 ** 192;

            return prod0;
        }
    }

    /// @notice Calculates `floor(a×b÷2^96)` with full precision. Throws if result overflows a uint256.
    /// @param a The multiplicand
    /// @param b The multiplier
    /// @return The 256-bit result
    function mulDiv96(uint256 a, uint256 b) internal pure returns (uint256) {
        unchecked {
            // 512-bit multiply [prod1 prod0] = a * b
            // Compute the product mod 2**256 and mod 2**256 - 1
            // then use the Chinese Remainder Theorem to reconstruct
            // the 512 bit result. The result is stored in two 256
            // variables such that product = prod1 * 2**256 + prod0
            uint256 prod0; // Least significant 256 bits of the product
            uint256 prod1; // Most significant 256 bits of the product
            assembly ("memory-safe") {
                let mm := mulmod(a, b, not(0))
                prod0 := mul(a, b)
                prod1 := sub(sub(mm, prod0), lt(mm, prod0))
            }

            // Handle non-overflow cases, 256 by 256 division
            if (prod1 == 0) {
                uint256 res;
                assembly ("memory-safe") {
                    // Right shift by n is equivalent and 2 gas cheaper than division by 2^n
                    res := shr(96, prod0)
                }
                return res;
            }

            // Make sure the result is less than 2**256.
            require(2 ** 96 > prod1);

            ///////////////////////////////////////////////
            // 512 by 256 division.
            ///////////////////////////////////////////////

            // Make division exact by subtracting the remainder from [prod1 prod0]
            // Compute remainder using mulmod
            uint256 remainder;
            assembly ("memory-safe") {
                remainder := mulmod(a, b, 0x1000000000000000000000000)
            }
            // Subtract 256 bit number from 512 bit number
            assembly ("memory-safe") {
                prod1 := sub(prod1, gt(remainder, prod0))
                prod0 := sub(prod0, remainder)
            }

            // Divide [prod1 prod0] by the factors of two (note that this is just 2**96 since the denominator is a power of 2 itself)
            assembly ("memory-safe") {
                // Right shift by n is equivalent and 2 gas cheaper than division by 2^n
                prod0 := shr(96, prod0)
            }
            // Shift in bits from prod1 into prod0. For this we need
            // to flip `twos` such that it is 2**256 / twos.
            // If twos is zero, then it becomes one
            // Note that this is just 2**160 since 2**256 over the fixed denominator (2**96) equals 2**160
            prod0 |= prod1 * 2 ** 160;

            return prod0;
        }
    }

    /// @notice Calculates `ceil(a×b÷2^96)` with full precision. Throws if result overflows a uint256.
    /// @param a The multiplicand
    /// @param b The multiplier
    /// @return result The 256-bit result
    function mulDiv96RoundingUp(uint256 a, uint256 b) internal pure returns (uint256 result) {
        unchecked {
            result = mulDiv96(a, b);
            if (mulmod(a, b, 2 ** 96) > 0) {
                require(result < type(uint256).max);
                result++;
            }
        }
    }

    /// @notice Calculates `floor(a×b÷2^128)` with full precision. Throws if result overflows a uint256.
    /// @param a The multiplicand
    /// @param b The multiplier
    /// @return The 256-bit result
    function mulDiv128(uint256 a, uint256 b) internal pure returns (uint256) {
        unchecked {
            // 512-bit multiply [prod1 prod0] = a * b
            // Compute the product mod 2**256 and mod 2**256 - 1
            // then use the Chinese Remainder Theorem to reconstruct
            // the 512 bit result. The result is stored in two 256
            // variables such that product = prod1 * 2**256 + prod0
            uint256 prod0; // Least significant 256 bits of the product
            uint256 prod1; // Most significant 256 bits of the product
            assembly ("memory-safe") {
                let mm := mulmod(a, b, not(0))
                prod0 := mul(a, b)
                prod1 := sub(sub(mm, prod0), lt(mm, prod0))
            }

            // Handle non-overflow cases, 256 by 256 division
            if (prod1 == 0) {
                uint256 res;
                assembly ("memory-safe") {
                    // Right shift by n is equivalent and 2 gas cheaper than division by 2^n
                    res := shr(128, prod0)
                }
                return res;
            }

            // Make sure the result is less than 2**256.
            require(2 ** 128 > prod1);

            ///////////////////////////////////////////////
            // 512 by 256 division.
            ///////////////////////////////////////////////

            // Make division exact by subtracting the remainder from [prod1 prod0]
            // Compute remainder using mulmod
            uint256 remainder;
            assembly ("memory-safe") {
                remainder := mulmod(a, b, 0x100000000000000000000000000000000)
            }
            // Subtract 256 bit number from 512 bit number
            assembly ("memory-safe") {
                prod1 := sub(prod1, gt(remainder, prod0))
                prod0 := sub(prod0, remainder)
            }

            // Divide [prod1 prod0] by the factors of two (note that this is just 2**128 since the denominator is a power of 2 itself)
            assembly ("memory-safe") {
                // Right shift by n is equivalent and 2 gas cheaper than division by 2^n
                prod0 := shr(128, prod0)
            }
            // Shift in bits from prod1 into prod0. For this we need
            // to flip `twos` such that it is 2**256 / twos.
            // If twos is zero, then it becomes one
            // Note that this is just 2**160 since 2**256 over the fixed denominator (2**128) equals 2**128
            prod0 |= prod1 * 2 ** 128;

            return prod0;
        }
    }

    /// @notice Calculates `ceil(a×b÷2^128)` with full precision. Throws if result overflows a uint256.
    /// @param a The multiplicand
    /// @param b The multiplier
    /// @return result The 256-bit result
    function mulDiv128RoundingUp(uint256 a, uint256 b) internal pure returns (uint256 result) {
        unchecked {
            result = mulDiv128(a, b);
            if (mulmod(a, b, 2 ** 128) > 0) {
                require(result < type(uint256).max);
                result++;
            }
        }
    }

    /// @notice Calculates `floor(a×b÷2^192)` with full precision. Throws if result overflows a uint256.
    /// @param a The multiplicand
    /// @param b The multiplier
    /// @return The 256-bit result
    function mulDiv192(uint256 a, uint256 b) internal pure returns (uint256) {
        unchecked {
            // 512-bit multiply [prod1 prod0] = a * b
            // Compute the product mod 2**256 and mod 2**256 - 1
            // then use the Chinese Remainder Theorem to reconstruct
            // the 512 bit result. The result is stored in two 256
            // variables such that product = prod1 * 2**256 + prod0
            uint256 prod0; // Least significant 256 bits of the product
            uint256 prod1; // Most significant 256 bits of the product
            assembly ("memory-safe") {
                let mm := mulmod(a, b, not(0))
                prod0 := mul(a, b)
                prod1 := sub(sub(mm, prod0), lt(mm, prod0))
            }

            // Handle non-overflow cases, 256 by 256 division
            if (prod1 == 0) {
                uint256 res;
                assembly ("memory-safe") {
                    // Right shift by n is equivalent and 2 gas cheaper than division by 2^n
                    res := shr(192, prod0)
                }
                return res;
            }

            // Make sure the result is less than 2**256.
            require(2 ** 192 > prod1);

            ///////////////////////////////////////////////
            // 512 by 256 division.
            ///////////////////////////////////////////////

            // Make division exact by subtracting the remainder from [prod1 prod0]
            // Compute remainder using mulmod
            uint256 remainder;
            assembly ("memory-safe") {
                remainder := mulmod(a, b, 0x1000000000000000000000000000000000000000000000000)
            }
            // Subtract 256 bit number from 512 bit number
            assembly ("memory-safe") {
                prod1 := sub(prod1, gt(remainder, prod0))
                prod0 := sub(prod0, remainder)
            }

            // Divide [prod1 prod0] by the factors of two (note that this is just 2**96 since the denominator is a power of 2 itself)
            assembly ("memory-safe") {
                // Right shift by n is equivalent and 2 gas cheaper than division by 2^n
                prod0 := shr(192, prod0)
            }
            // Shift in bits from prod1 into prod0. For this we need
            // to flip `twos` such that it is 2**256 / twos.
            // If twos is zero, then it becomes one
            // Note that this is just 2**64 since 2**256 over the fixed denominator (2**192) equals 2**64
            prod0 |= prod1 * 2 ** 64;

            return prod0;
        }
    }

    /// @notice Calculates `ceil(a×b÷2^192)` with full precision.
    /// @param a The multiplicand
    /// @param b The multiplier
    /// @return result The 256-bit result
    function mulDiv192RoundingUp(uint256 a, uint256 b) internal pure returns (uint256 result) {
        unchecked {
            result = mulDiv192(a, b);
            if (mulmod(a, b, 2 ** 192) > 0) {
                require(result < type(uint256).max);
                result++;
            }
        }
    }

    /// @notice Calculates `floor(a×b÷10^18)` with full precision. Throws if result overflows a uint256.
    /// @param a The multiplicand
    /// @param b The multiplier
    /// @return The 256-bit result
    /// @dev Optimized version of mulDiv for WAD (10^18) denominator
    function mulDivWad(uint256 a, uint256 b) internal pure returns (uint256) {
        unchecked {
            // 512-bit multiply [prod1 prod0] = a * b
            // Compute the product mod 2**256 and mod 2**256 - 1
            // then use the Chinese Remainder Theorem to reconstruct
            // the 512 bit result. The result is stored in two 256
            // variables such that product = prod1 * 2**256 + prod0
            uint256 prod0; // Least significant 256 bits of the product
            uint256 prod1; // Most significant 256 bits of the product
            assembly ("memory-safe") {
                let mm := mulmod(a, b, not(0))
                prod0 := mul(a, b)
                prod1 := sub(sub(mm, prod0), lt(mm, prod0))
            }

            // Handle non-overflow cases, 256 by 256 division
            if (prod1 == 0) {
                uint256 res;
                assembly ("memory-safe") {
                    res := div(prod0, 1000000000000000000)
                }
                return res;
            }

            // Make sure the result is less than 2**256.
            require(1000000000000000000 > prod1);

            ///////////////////////////////////////////////
            // 512 by 256 division.
            ///////////////////////////////////////////////

            // Make division exact by subtracting the remainder from [prod1 prod0]
            // Compute remainder using mulmod
            uint256 remainder;
            assembly ("memory-safe") {
                remainder := mulmod(a, b, 1000000000000000000)
            }
            // Subtract 256 bit number from 512 bit number
            assembly ("memory-safe") {
                prod1 := sub(prod1, gt(remainder, prod0))
                prod0 := sub(prod0, remainder)
            }

            // Since 10^18 = 2^18 * 5^18, we need to handle both factors
            // First divide by 2^18
            assembly ("memory-safe") {
                // Right shift by 18 is equivalent to division by 2^18
                prod0 := shr(18, prod0)
            }

            // Shift in bits from prod1 into prod0
            // We need 2^256 / 2^18 = 2^238
            prod0 |= prod1 << 238;

            // Now divide by 5^18 using modular inverse
            // Precomputed modular inverse of 5^18 modulo 2^256
            // This means: (5^18 * inv) mod 2^256 = 1
            uint256 inv = 0xaccb18165bd6fe31ae1cf318dc5b51eee0e1ba569b88cd74c1773b91fac10669;

            return prod0 * inv;
        }
    }

    /// @notice Calculates `ceil(a×b÷10^18)` with full precision.
    /// @param a The multiplicand
    /// @param b The multiplier
    /// @return result The 256-bit result
    function mulDivWadRoundingUp(uint256 a, uint256 b) internal pure returns (uint256 result) {
        unchecked {
            result = mulDivWad(a, b);
            if (mulmod(a, b, 10 ** 18) > 0) {
                require(result < type(uint256).max);
                result++;
            }
        }
    }

    /// @notice Calculates `ceil(a÷b)`, returning 0 if `b == 0`.
    /// @param a The numerator
    /// @param b The denominator
    /// @return result The 256-bit result
    function unsafeDivRoundingUp(uint256 a, uint256 b) internal pure returns (uint256 result) {
        assembly ("memory-safe") {
            result := add(div(a, b), gt(mod(a, b), 0))
        }
    }

    /*//////////////////////////////////////////////////////////////
                                SORTING
    //////////////////////////////////////////////////////////////*/

    /// @notice QuickSort is a sorting algorithm that employs the Divide and Conquer strategy. It selects a pivot element and arranges the given array around
    /// this pivot by correctly positioning it within the sorted array.
    /// @param arr The elements that must be sorted
    /// @param left The starting index
    /// @param right The ending index
    function quickSort(int256[] memory arr, int256 left, int256 right) internal pure {
        unchecked {
            int256 i = left;
            int256 j = right;
            if (i == j) return;
            int256 pivot = arr[uint256(left + (right - left) / 2)];
            while (i < j) {
                while (arr[uint256(i)] < pivot) i++;
                while (pivot < arr[uint256(j)]) j--;
                if (i <= j) {
                    (arr[uint256(i)], arr[uint256(j)]) = (arr[uint256(j)], arr[uint256(i)]);
                    i++;
                    j--;
                }
            }
            if (left < j) quickSort(arr, left, j);
            if (i < right) quickSort(arr, i, right);
        }
    }

    /// @notice Calls `quickSort` with default starting index of 0 and ending index of the last element in the array.
    /// @param data The elements that must be sorted
    /// @return The sorted array
    function sort(int256[] memory data) internal pure returns (int256[] memory) {
        unchecked {
            quickSort(data, int256(0), int256(data.length - 1));
        }
        return data;
    }

    /*//////////////////////////////////////////////////////////////
                         EXPONENTIAL MATH
    //////////////////////////////////////////////////////////////*/

    /// @dev Returns the sum of the first three non-zero terms of a Taylor expansion of e^(nx) - 1, to approximate a
    /// continuous compound interest rate. Source: https://github.com/morpho-org/morpho-blue/blob/main/src/libraries/MathLib.sol
    function wTaylorCompounded(uint256 x, uint256 n) internal pure returns (uint256) {
        uint256 firstTerm = x * n;
        uint256 secondTerm = mulDiv(firstTerm, firstTerm, 2 * WAD);
        uint256 thirdTerm = mulDiv(secondTerm, firstTerm, 3 * WAD);

        return firstTerm + secondTerm + thirdTerm;
    }

    /// @dev Returns the sum of the first three non-zero terms of a Taylor expansion of e^(nx), to approximate a
    /// continuous compound interest rate for a custom scale s. Source: https://github.com/morpho-org/morpho-blue/blob/main/src/libraries/MathLib.sol
    function sTaylorCompounded(uint256 x, uint256 s) internal pure returns (uint256) {
        uint256 zerothTerm = s;
        uint256 firstTerm = x;
        uint256 secondTerm = mulDiv(firstTerm, firstTerm, 2 * s);
        uint256 thirdTerm = mulDiv(secondTerm, firstTerm, 3 * s);
        uint256 fourthTerm = mulDiv(thirdTerm, firstTerm, 4 * s);

        return zerothTerm + firstTerm + secondTerm + thirdTerm + fourthTerm;
    }

    /// @dev Returns the multiplication of `x` by `y` (in WAD) rounded towards 0.
    function wMulToZero(int256 x, int256 y) internal pure returns (int256) {
        return (x * y) / WAD_INT;
    }

    /// @dev Returns the division of `x` by `y` (in WAD) rounded towards 0.
    function wDivToZero(int256 x, int256 y) internal pure returns (int256) {
        return (x * WAD_INT) / y;
    }

    /// @dev Bounds `x` between `low` and `high`.
    /// @dev Assumes that `low` <= `high`. If it is not the case it returns `low`.
    function bound(int256 x, int256 low, int256 high) internal pure returns (int256 z) {
        assembly {
            // z = min(x, high).
            z := xor(x, mul(xor(x, high), slt(high, x)))
            // z = max(z, low).
            z := xor(z, mul(xor(z, low), sgt(low, z)))
        }
    }

    /// @dev Returns an approximation of exp.
    function wExp(int256 x) internal pure returns (int256) {
        unchecked {
            // If x < ln(1e-18) then exp(x) < 1e-18 so it is rounded to zero.
            if (x < LN_WEI_INT) return 0;
            // `wExp` is clipped to avoid overflowing when multiplied with 1 ether.
            if (x >= WEXP_UPPER_BOUND) return WEXP_UPPER_VALUE;

            // Decompose x as x = q * ln(2) + r with q an integer and -ln(2)/2 <= r <= ln(2)/2.
            // q = x / ln(2) rounded half toward zero.
            int256 roundingAdjustment = (x < 0) ? -(LN_2_INT / 2) : (LN_2_INT / 2);
            // Safe unchecked because x is bounded.
            int256 q = (x + roundingAdjustment) / LN_2_INT;
            // Safe unchecked because |q * ln(2) - x| <= ln(2)/2.
            int256 r = x - q * LN_2_INT;

            // Compute e^r with a 2nd-order Taylor polynomial.
            // Safe unchecked because |r| < 1e18.
            int256 expR = WAD_INT + r + (r * r) / WAD_INT / 2;

            // Return e^x = 2^q * e^r.
            if (q >= 0) return expR << uint256(q);
            else return expR >> uint256(-q);
        }
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.24;

/// @title Partial definition of the ERC20 interface as defined in the EIP
/// @dev Does not include return values as certain tokens such as USDT fail to implement them.
/// @dev Since the return value is not expected, this interface works with both compliant and non-compliant tokens.
/// @dev Clients are recommended to consume and handle the return of negative success values.
/// @dev However, we cannot productively handle a failed approval and such a situation would surely cause a revert later in execution.
/// @dev In addition, no notable instances exist of tokens that both i) contain a failure case for `approve` and ii) return `false` instead of reverting.
/// @author Axicon Labs Limited
interface IERC20Partial {
    /// @notice Returns the amount of tokens owned by `account`.
    /// @dev This function is unchanged from the EIP.
    /// @param account The address to query the balance of
    /// @return The amount of tokens owned by `account`
    function balanceOf(address account) external view returns (uint256);

    /// @notice Sets `amount` as the allowance of `spender` over the caller's tokens.
    /// @dev While this function is specified to return a boolean value in the EIP, this interface does not expect one.
    /// @param spender The address which will spend the funds
    /// @param amount The amount of tokens allowed to be spent
    function approve(address spender, uint256 amount) external;

    /// @notice Transfers tokens from the caller to another user.
    /// @param to The user to transfer tokens to
    /// @param amount The amount of tokens to transfer
    function transfer(address to, uint256 amount) external;

    /// @notice Returns the amount of tokens in existence.
    function totalSupply() external view returns (uint256);
}

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.24;

type PoolData is uint256;
using PoolDataLibrary for PoolData global;

/// @title A Panoptic Pool Data. Tracks the Uniswap Pool, the minEnforcedTick, and the maxEnforcedTick
/// @author Axicon Labs Limited
//
//
// PACKING RULES FOR A POOLDATA:
// =================================================================================================
//  From the LSB to the MSB:
// (1) maxLiquidityPerTick  128bits : The max liquidity per tick
// (2) poolId               64bits  : The poolId
// (3) minEnforcedTick      24bits  : The current minimum enforced tick for the pool in the SFPM (int24).
// (4) maxEnforcedTick      24bits  : The current maximum enforced tick for the pool in the SFPM (int24).
// (5) initialized          1bit    : Whether the pool has been initialized
// Total                    241bits : Total bits used by a PoolData.
// ===============================================================================================
//
// The bit pattern is therefore:
//
//        (5)             (4)                   (3)             (2)                    (1)
//    <-- 1 bit --><---- 24 bits ----> <---- 24 bits ----> <---- 64 bits ----> <---- 128 bits ---->
//     initialized    maxEnforcedTick    minEnforcedTick         poolId           maxLiquidityPerTick
//
//    <--- most significant bit                                            least significant bit --->
//
library PoolDataLibrary {
    /*//////////////////////////////////////////////////////////////
                                ENCODING
    //////////////////////////////////////////////////////////////*/

    /// @notice Create a new `PoolData` given by UniswapV3Pool, min/maxEnforcedTick.
    /// @param _maxLiquidityPerTick The max liquidity per tick
    /// @param _poolId The poolId of the Uniswap pool
    /// @param _minEnforcedTick The current minimum enforced tick for the pool in the SFPM
    /// @param _maxEnforcedTick The current maximum enforced tick for the pool in the SFPM
    /// @return The new PoolData with the given IUniswapV3Pool and min/maxEnforcedTick
    function storePoolData(
        uint128 _maxLiquidityPerTick,
        uint64 _poolId,
        int24 _minEnforcedTick,
        int24 _maxEnforcedTick,
        bool _initialized
    ) internal pure returns (PoolData) {
        unchecked {
            return
                PoolData.wrap(
                    _maxLiquidityPerTick +
                        (uint256(_poolId) << 128) +
                        (uint256(uint24(_minEnforcedTick)) << 192) +
                        (uint256(uint24(_maxEnforcedTick)) << 216) +
                        (uint256(_initialized ? 1 : 0) << 240)
                );
        }
    }

    /*//////////////////////////////////////////////////////////////
                                DECODING
    //////////////////////////////////////////////////////////////*/

    /// @notice Get the maxLiquidityPerTick of `self`.
    /// @param self The PoolData to retrieve the max liquidity from
    /// @return The maxLiquidityPerTick of `self`
    function maxLiquidityPerTick(PoolData self) internal pure returns (uint128) {
        unchecked {
            return uint128(PoolData.unwrap(self));
        }
    }

    /// @notice Get the poolId of `self`.
    /// @param self The PoolData to retrieve the poolId from
    /// @return The poolId of `self`
    function poolId(PoolData self) internal pure returns (uint64) {
        unchecked {
            return uint64(PoolData.unwrap(self) >> 128);
        }
    }

    /// @notice Get the min enforced tick of `self`.
    /// @param self The PoolData to retrieve the min enforced tick from
    /// @return The min enforced tick of `self`
    function minEnforcedTick(PoolData self) internal pure returns (int24) {
        unchecked {
            return int24(uint24(PoolData.unwrap(self) >> 192));
        }
    }

    /// @notice Get the max enforced tick of `self`.
    /// @param self The PoolData to retrieve the max enforced tick from
    /// @return The max enforced tick of `self`
    function maxEnforcedTick(PoolData self) internal pure returns (int24) {
        unchecked {
            return int24(uint24(PoolData.unwrap(self) >> 216));
        }
    }

    /// @notice Get the initialized bool of `self`.
    /// @param self The PoolData to retrieve initialized flag from
    /// @return The initialized flag of `self`
    function initialized(PoolData self) internal pure returns (bool) {
        unchecked {
            return ((PoolData.unwrap(self) >> 240) % 2) > 0;
        }
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.24;

/// @title Library of Constants used in Panoptic.
/// @author Axicon Labs Limited
/// @notice This library provides constants used in Panoptic.
library Constants {
    /// @notice Fixed point multiplier: 2**96
    uint256 internal constant FP96 = 0x1000000000000000000000000;

    /// @notice Minimum possible price tick in a Uniswap V3 pool
    int24 internal constant MIN_POOL_TICK = -887272;

    /// @notice Maximum possible price tick in a Uniswap V3 pool
    int24 internal constant MAX_POOL_TICK = 887272;

    /// @notice Minimum possible sqrtPriceX96 in a Uniswap V3 pool
    uint160 internal constant MIN_POOL_SQRT_RATIO = 4295128739;

    /// @notice Maximum possible sqrtPriceX96 in a Uniswap V3 pool
    uint160 internal constant MAX_POOL_SQRT_RATIO =
        1461446703485210103287273052203988822378723970342;

    /// @notice The maximum amount of change, in ticks, permitted before TICK_OFFSET is updated.
    int24 internal constant MAX_RESIDUAL_THRESHOLD = 1024;
}


END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

// Foundry
import "forge-std/Script.sol";
import {PanopticFactory} from "@contracts/PanopticFactory.sol";
import {CollateralTracker} from "@contracts/CollateralTracker.sol";
import {RiskEngine} from "@contracts/RiskEngine.sol";
import {PanopticPool} from "@contracts/PanopticPool.sol";
import {ISemiFungiblePositionManager} from "@contracts/interfaces/ISemiFungiblePositionManager.sol";
import {SemiFungiblePositionManager} from "@contracts/SemiFungiblePositionManager.sol";
import {IUniswapV3Factory} from "univ3-core/interfaces/IUniswapV3Factory.sol";
import {Pointer, PointerLibrary} from "@types/Pointer.sol";
import {PanopticHelper} from "@test_periphery/PanopticHelper.sol";

contract DeployProtocol is Script {
    struct PointerInfo {
        uint256 codeIndex;
        uint256 end;
        uint256 start;
    }

    function run() public {
        uint256 DEPLOYER_PRIVATE_KEY = vm.envUint("DEPLOYER_PRIVATE_KEY");

        // 0x0227628f3f023bb0b980b67d528571c95c6dac1c: sepolia
        IUniswapV3Factory uniFactory = IUniswapV3Factory(vm.envAddress("UNIV3_FACTORY"));

        vm.startBroadcast(DEPLOYER_PRIVATE_KEY);

        string memory metadata = vm.readFile("./metadata/out/MetadataPackage.json");

        bytes[] memory bytecodes = vm.parseJsonBytesArray(metadata, ".bytecodes");
        address[] memory pointerAddresses = new address[](bytecodes.length);

        for (uint256 i = 0; i < bytecodes.length; i++) {
            bytes memory code = bytecodes[i];
            address pointer;
            // deploy code and store pointer
            assembly {
                pointer := create(0, add(code, 0x20), mload(code))
                if iszero(extcodesize(pointer)) {
                    revert(0, 0)
                }
            }
            pointerAddresses[i] = pointer;
        }

        PointerInfo[][] memory pointerInfo = abi.decode(
            vm.parseJson(metadata, ".pointers"),
            (PointerInfo[][])
        );
        Pointer[][] memory pointers = new Pointer[][](pointerInfo.length);

        for (uint256 i = 0; i < pointerInfo.length; i++) {
            pointers[i] = new Pointer[](pointerInfo[i].length);
            for (uint256 j = 0; j < pointerInfo[i].length; j++) {
                pointers[i][j] = PointerLibrary.createPointer(
                    pointerAddresses[pointerInfo[i][j].codeIndex],
                    uint48(pointerInfo[i][j].start),
                    uint48(pointerInfo[i][j].end)
                );
            }
        }

        string[] memory propsStr = vm.parseJsonStringArray(metadata, ".properties");
        bytes32[] memory props = new bytes32[](propsStr.length);
        for (uint256 i = 0; i < propsStr.length; i++) {
            props[i] = bytes32(bytes(propsStr[i]));
        }

        string[][] memory indicesStr = abi.decode(vm.parseJson(metadata, ".indices"), (string[][]));
        uint256[][] memory indices = new uint256[][](indicesStr.length);
        for (uint256 i = 0; i < indicesStr.length; i++) {
            indices[i] = new uint256[](indicesStr[i].length);
            for (uint256 j = 0; j < indicesStr[i].length; j++) {
                indices[i][j] = vm.parseUint(indicesStr[i][j]);
            }
        }

        SemiFungiblePositionManager sfpm = new SemiFungiblePositionManager(uniFactory, 10 ** 13, 0);

        // risk engine MED
        new RiskEngine(
            10_000_000,
            10_000_000,
            address(0), // add guardian
            address(0) // add builderFactory
        );
        /*
        // risk engine LOW
        new RiskEngine(500_000, 250_000, 128, 5_000_000, 9_000_000);
        // risk engine HIGH
        new RiskEngine(4_500_000, 2_250_000, 128, 5_000_000, 9_000_000);
        */
        new PanopticFactory(
            sfpm,
            uniFactory,
            address(new PanopticPool(ISemiFungiblePositionManager(address(sfpm)))),
            address(new CollateralTracker(10)),
            props,
            indices,
            pointers
        );

        new PanopticHelper(ISemiFungiblePositionManager(address(sfpm)));

        // factory.tokenURI(0x00c34C41289e6c433723542BB1Eba79c6919504EDD);
        vm.stopBroadcast();
    }
}

