
## *MAIN TARGET CONTRACT* TO REVIEW

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

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
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
// OpenZeppelin Contracts (last updated v4.8.0) (utils/Strings.sol)

pragma solidity ^0.8.0;

import "./math/Math.sol";

/**
 * @dev String operations.
 */
library Strings {
    bytes16 private constant _SYMBOLS = "0123456789abcdef";
    uint8 private constant _ADDRESS_LENGTH = 20;

    /**
     * @dev Converts a `uint256` to its ASCII `string` decimal representation.
     */
    function toString(uint256 value) internal pure returns (string memory) {
        unchecked {
            uint256 length = Math.log10(value) + 1;
            string memory buffer = new string(length);
            uint256 ptr;
            /// @solidity memory-safe-assembly
            assembly {
                ptr := add(buffer, add(32, length))
            }
            while (true) {
                ptr--;
                /// @solidity memory-safe-assembly
                assembly {
                    mstore8(ptr, byte(mod(value, 10), _SYMBOLS))
                }
                value /= 10;
                if (value == 0) break;
            }
            return buffer;
        }
    }

    /**
     * @dev Converts a `uint256` to its ASCII `string` hexadecimal representation.
     */
    function toHexString(uint256 value) internal pure returns (string memory) {
        unchecked {
            return toHexString(value, Math.log256(value) + 1);
        }
    }

    /**
     * @dev Converts a `uint256` to its ASCII `string` hexadecimal representation with fixed length.
     */
    function toHexString(uint256 value, uint256 length) internal pure returns (string memory) {
        bytes memory buffer = new bytes(2 * length + 2);
        buffer[0] = "0";
        buffer[1] = "x";
        for (uint256 i = 2 * length + 1; i > 1; --i) {
            buffer[i] = _SYMBOLS[value & 0xf];
            value >>= 4;
        }
        require(value == 0, "Strings: hex length insufficient");
        return string(buffer);
    }

    /**
     * @dev Converts an `address` with fixed length of 20 bytes to its not checksummed ASCII `string` hexadecimal representation.
     */
    function toHexString(address addr) internal pure returns (string memory) {
        return toHexString(uint256(uint160(addr)), _ADDRESS_LENGTH);
    }
}


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS

## SUPPORTING CONTEXT: EXTERNAL LIBRARIES
// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.24;

type RiskParameters is uint256;
using RiskParametersLibrary for RiskParameters global;

/// @title A Panoptic Risk Parameters. Tracks the data outputted from the RiskEngine, like the safeMode, commission fees, (etc).
/// @author Axicon Labs Limited
//
//
// PACKING RULES FOR A RISKPARAMETERS:
// =================================================================================================
//  From the LSB to the MSB:
// (1) safeMode             4 bits  : The safeMode state
// (2) notionalFee          14 bits : The fee to be charged on notional at mint
// (3) premiumFee           14 bits : The fee to be charged on the premium at burn
// (4) protocolSplit        14 bits : The part of the fee that goes to the protocol w/ buildercodes
// (5) builderSplit         14 bits : The part of the fee that goes to the builder w/ buildercodes
// (6) tickDeltaLiquidation 13 bits : The MAX_TWAP_DELTA_LIQUIDATION. Tick deviation = 1.0001**(2**13) = +/- 126%
// (7) maxSpread            22 bits : The MAX_SPREAD, in bps. Max fraction removed = 2**22/(2**22 + 10_000) = 99.76%
// (8) bpDecreaseBuffer     26 bits : The BP_DECREASE_BUFFER, in millitick
// (9) maxLegs              7 bits  : The MAX_OPEN_LEGS (constrained to be <128)
// (9) feeRecipient         128bits : The recipient of the commission fee split
// Total                    256bits  : Total bits used by a RiskParameters.
// ===============================================================================================
//
// The bit pattern is therefore:
//
//          (9)              (8)          (7)              (6)             (5)            (4)          (3)             (2)              (1)
//    <-- 128 bits --><-- 7 bits --><-- 26 bits --><-- 22 bits --><-- 13 bits --><-- 14 bits --><-- 14 bits --> <-- 14 bits --> <-- 14 bits --> <-- 4 bits -->
//        feeRecipient   maxLegs      bpDecrease      maxSpread      tickDelta    builderSplit   protocolSplit    premiumFee    notionalFee         safeMode
//
//    <--- most significant bit                                                                  least significant bit --->
//
library RiskParametersLibrary {
    /*//////////////////////////////////////////////////////////////
                                ENCODING
    //////////////////////////////////////////////////////////////*/

    /// @notice Create a new `RiskParameters` object.
    /// @param _safeMode The safe mode state (uint6)
    /// @param _notionalFee The commission fee (uint14)
    /// @param _premiumFee The commission fee (uint14)
    /// @param _protocolSplit The part of the fee that goes to the protocol w/ buildercodes (uint14)
    /// @param _builderSplit The part of the fee that goes to the builder w/ buildercodes (uint14)
    /// @param _tickDeltaLiquidation The MAX_TWAP_DELTA_LIQUIDATION (uint16)
    /// @param _maxSpread The MAX_SPREAD, in bps (uint24)
    /// @param _bpDecreaseBuffer The BP_DECREASE_BUFFER, in millitick (uint26)
    /// @param _maxLegs The maximum allowed number of legs across all open positions for a user
    /// @param _feeRecipient The recipient of the commission fee split (uint128)
    /// @return result The new RiskParameters object
    function storeRiskParameters(
        uint256 _safeMode,
        uint256 _notionalFee,
        uint256 _premiumFee,
        uint256 _protocolSplit,
        uint256 _builderSplit,
        uint256 _tickDeltaLiquidation,
        uint256 _maxSpread,
        uint256 _bpDecreaseBuffer,
        uint256 _maxLegs,
        uint256 _feeRecipient
    ) internal pure returns (RiskParameters result) {
        assembly {
            result := add(
                add(
                    add(
                        add(_safeMode, shl(4, _notionalFee)),
                        add(shl(18, _premiumFee), shl(32, _protocolSplit))
                    ),
                    add(shl(46, _builderSplit), shl(60, _tickDeltaLiquidation))
                ),
                add(
                    add(shl(73, _maxSpread), add(shl(95, _bpDecreaseBuffer), shl(121, _maxLegs))),
                    shl(128, _feeRecipient)
                )
            )
        }
    }

    /*//////////////////////////////////////////////////////////////
                                DECODING
    //////////////////////////////////////////////////////////////*/

    /// @notice Get the safeMode state of `self`.
    /// @param self The RiskParameters to retrieve the safeMode state from
    /// @return result The safeMode of `self`
    function safeMode(RiskParameters self) internal pure returns (uint8 result) {
        assembly {
            result := and(self, 0xF)
        }
    }

    /// @notice Get the notionalFee of `self`.
    /// @param self The RiskParameters to retrieve the notionalFee from
    /// @return result The notionalFee of `self`
    function notionalFee(RiskParameters self) internal pure returns (uint16 result) {
        assembly {
            result := and(shr(4, self), 0x3FFF)
        }
    }

    /// @notice Get the premiumFee of `self`.
    /// @param self The RiskParameters to retrieve the premiumFee from
    /// @return result The premiumFee of `self`
    function premiumFee(RiskParameters self) internal pure returns (uint16 result) {
        assembly {
            result := and(shr(18, self), 0x3FFF)
        }
    }

    /// @notice Get the protocolSplit of `self`.
    /// @param self The RiskParameters to retrieve the protocolSplit from
    /// @return result The protocolSplit of `self`
    function protocolSplit(RiskParameters self) internal pure returns (uint16 result) {
        assembly {
            result := and(shr(32, self), 0x3FFF)
        }
    }

    /// @notice Get the builderSplit of `self`.
    /// @param self The RiskParameters to retrieve the builderSplit from
    /// @return result The builderSplit of `self`
    function builderSplit(RiskParameters self) internal pure returns (uint16 result) {
        assembly {
            result := and(shr(46, self), 0x3FFF)
        }
    }

    /// @notice Get the tickDeltaLiquidation of `self`.
    /// @param self The RiskParameters to retrieve the tickDeltaLiquidation from
    /// @return result The tickDeltaLiquidation of `self`
    function tickDeltaLiquidation(RiskParameters self) internal pure returns (uint16 result) {
        assembly {
            result := and(shr(60, self), 0x1FFF)
        }
    }

    /// @notice Get the maxSpread of `self`.
    /// @param self The RiskParameters to retrieve the maxSpread from
    /// @return result The maxSpread of `self`
    function maxSpread(RiskParameters self) internal pure returns (uint24 result) {
        assembly {
            result := and(shr(73, self), 0x3FFFFF)
        }
    }

    /// @notice Get the bpDecreaseBuffer of `self`.
    /// @param self The RiskParameters to retrieve the bpDecreaseBuffer from
    /// @return result The bpDecreaseBuffer of `self`
    function bpDecreaseBuffer(RiskParameters self) internal pure returns (uint32 result) {
        assembly {
            result := and(shr(95, self), 0x3FFFFFF)
        }
    }

    /// @notice Get the maxLegs of `self`.
    /// @param self The RiskParameters to retrieve the maxLegs from
    /// @return result The maxLegs of `self`
    function maxLegs(RiskParameters self) internal pure returns (uint8 result) {
        assembly {
            result := and(shr(121, self), 0x7F)
        }
    }

    /// @notice Get the feeRecipient of `self`.
    /// @param self The RiskParameters to retrieve the feeRecipient from
    /// @return result The feeRecipient of `self`
    function feeRecipient(RiskParameters self) internal pure returns (uint128 result) {
        assembly {
            result := shr(128, self)
        }
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


END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

