
## *MAIN TARGET CONTRACT* TO REVIEW

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

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
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


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS

## SUPPORTING CONTEXT: EXTERNAL LIBRARIES
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

