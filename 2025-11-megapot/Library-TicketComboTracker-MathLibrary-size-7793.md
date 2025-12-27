
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import { Combinations } from "./Combinations.sol";
import { LibBit } from "solady/src/utils/LibBit.sol";

/**
 * @title TicketComboTracker
 * @notice Library for tracking jackpot ticket combinations and calculating win distributions efficiently
 * @dev Implements scalable settlement calculations using bit vectors and inclusion-exclusion principle:
 *      - Stores ticket combinations as bit vectors for efficient subset operations
 *      - Tracks both unique and duplicate ticket counts per combination subset
 *      - Uses inclusion-exclusion principle to avoid double-counting when calculating payouts
 *      - Enables O(1) duplicate detection and efficient tier-based payout calculations
 *      - Supports configurable normal ball ranges and bonusball values
 *      - Optimized for gas efficiency in high-volume jackpot scenarios
 */
library TicketComboTracker {
    struct ComboCount {
        uint128 count;
        uint128 dupCount;
    }

    struct Tracker {
        uint8 normalMax;
        uint8 bonusballMax;
        uint8 normalTiers;
        mapping(uint8 => mapping(uint256 => ComboCount)) comboCounts;
        mapping(uint8 => ComboCount) bonusballTicketCounts;
    }

    /**
     * @notice Initializes a combo tracker with jackpot configuration parameters
     * @dev Sets up the tracker with ball ranges and tier configuration for efficient combo tracking.
     *      Must be called before using any other tracker functions.
     * @param tracker Storage reference to the tracker being initialized
     * @param _normalMax Maximum value for normal balls (1 to this value)
     * @param _bonusballMax Maximum value for bonusball (1 to this value)
     * @param _normalTiers Number of normal balls per ticket (typically 5)
     * @custom:effects
     * - Configures tracker parameters for combo calculations
     * - Prepares tracker for ticket insertion and counting operations
     * @custom:security
     * - No validation as this is internal initialization
     * - Caller responsible for providing valid parameters
     */
    function init(
        Tracker storage tracker,
        uint8 _normalMax,
        uint8 _bonusballMax,
        uint8 _normalTiers
    ) internal {
        tracker.normalMax = _normalMax;
        tracker.bonusballMax = _bonusballMax;
        tracker.normalTiers = _normalTiers;
    }

    /**
     * @notice Converts an array of normal ball numbers to a bit vector representation
     * @dev Creates a bit vector where each bit position represents a ball number.
     *      Validates no duplicates and all numbers are within valid range.
     * @param _set Array of ball numbers to convert
     * @param _maxNormalBall Maximum valid ball number
     * @return Bit vector representation where bit N is set if ball N is selected
     * @custom:requirements
     * - Set must not be empty
     * - All numbers must be > 0 and <= _maxNormalBall
     * - No duplicate numbers allowed
     * @custom:security
     * - Validates range and uniqueness to prevent invalid combinations
     * - Uses bit operations for efficient duplicate detection
     */
    function toNormalsBitVector(
        uint8[] memory _set,
        uint256 _maxNormalBall
    )
        internal
        pure
        returns (uint256)
    {
        require(_set.length != 0, "Invalid set length");
        uint256 bitVector = 0;
        for (uint256 i; i < _set.length; ++i) {
            require(_set[i] <= _maxNormalBall && _set[i] > 0, "Invalid set selection");
            require((bitVector & (1 << _set[i])) == 0, "Duplicate number in set");
            bitVector |= 1 << _set[i];
        }
        return bitVector;
    }

    /**
     * @notice Inserts a ticket combination into the tracker and updates subset counts
     * @dev Converts ticket to bit vector, generates all subsets, and updates counts.
     *      Distinguishes between first purchase of a ticket (unique) and duplicates for
     *      payout calculations. 
     * @param _tracker Storage reference to the tracker
     * @param _normalBalls Array of normal ball numbers
     * @param _bonusball Bonusball number
     * @return ticketNumbers Bit vector representation of the complete ticket
     * @return isDup True if this exact combination was already inserted
     * @custom:requirements
     * - Normal balls array length must match tracker.normalTiers
     * - All ball numbers must be valid per tracker configuration
     * @custom:effects
     * - Updates counts for all subset combinations of the ticket
     * - Increments either unique or duplicate counts based on prior existence
     * - Tracks bonusball-specific subset counts for tier calculations
     * @custom:security
     * - Validates ticket format matches tracker configuration
     * - Prevents invalid combinations through bit vector validation
     */
    function insert(
        Tracker storage _tracker,
        uint8[] memory _normalBalls,
        uint8 _bonusball
    )
        internal
        returns (uint256 ticketNumbers, bool isDup)
    {
        require(_normalBalls.length == _tracker.normalTiers, "Invalid pick length");
        uint256 set = toNormalsBitVector(_normalBalls, _tracker.normalMax);
        // Iterate over all tier combos and store the combo counts
        isDup = _tracker.comboCounts[_bonusball][set].count > 0;
        for (uint8 i = 1; i <= _tracker.normalTiers; i++) {
            uint256[] memory subsets = Combinations.generateSubsets(set, i);
            for (uint256 j = 0; j < subsets.length; j++) {
                if (isDup) {
                    _tracker.comboCounts[_bonusball][subsets[j]].dupCount++;
                } else {
                    _tracker.comboCounts[_bonusball][subsets[j]].count++;
                }
            }
        }

        if (isDup) {
            _tracker.bonusballTicketCounts[_bonusball].dupCount++;
        } else {
            _tracker.bonusballTicketCounts[_bonusball].count++;
        }

        // Add the bonusball to the bit vector
        ticketNumbers = set |= 1 << (_bonusball + _tracker.normalMax);
    }

    function _countSubsetMatches(
        Tracker storage _tracker,
        uint256 _normalBallsBitVector,
        uint8 _bonusball
    )
        private
        view
        returns (uint256[] memory matches, uint256[] memory dupMatches)
    {
        matches = new uint256[]((_tracker.normalTiers+1)*2);
        dupMatches = new uint256[]((_tracker.normalTiers+1)*2);
        
        for (uint8 i = 1; i <= _tracker.bonusballMax; i++) {
            for (uint8 k = 1; k <= _tracker.normalTiers; k++) {
                uint256[] memory subsets = Combinations.generateSubsets(_normalBallsBitVector, k);
                for (uint256 l = 0; l < subsets.length; l++) {
                    if (i == _bonusball) {
                        matches[(k*2)+1] += _tracker.comboCounts[i][subsets[l]].count;
                        dupMatches[k*2+1] += _tracker.comboCounts[i][subsets[l]].dupCount;
                    } else {
                        matches[(k*2)] += _tracker.comboCounts[i][subsets[l]].count;
                        dupMatches[k*2] += _tracker.comboCounts[i][subsets[l]].dupCount;
                    }
                }
            }
        }
    }

    function _applyInclusionExclusionPrinciple(
        Tracker storage _tracker,
        uint256[] memory _matches,
        uint256[] memory _dupMatches
    )
        private
        view
        returns (uint256[] memory result, uint256[] memory dupResult)
    {
        result = new uint256[](_matches.length);
        dupResult = new uint256[](_dupMatches.length);
        
        // Solve top-down (starting from "all matched")
        for (uint256 k = _tracker.normalTiers; k >= 1; --k) {
            uint256 s = _matches[2*k];
            uint256 sp = _matches[2*k+1];
            uint256 sd = _dupMatches[2*k];
            uint256 sdp = _dupMatches[2*k+1];
            
            // Repeatedly subtract higher-tier counts that spill over into this tier
            for (uint256 m = k + 1; m <= _tracker.normalTiers; ++m) {
                // Each higher-tier ticket contributes C(m,k) subsets to this tier
                uint256 c = Combinations.choose(m, k);
                s -= c * result[2*m];
                sp -= c * result[2*m+1];
                sd -= c * dupResult[2*m];
                sdp -= c * dupResult[2*m+1];
            }
            
            result[2*k] = s;
            result[2*k+1] = sp;
            dupResult[2*k] = sd;
            dupResult[2*k+1] = sdp;
        }
    }

    function _calculateBonusballOnlyMatches(
        Tracker storage _tracker,
        uint8 _bonusball,
        uint256[] memory _uniqueResult,
        uint256[] memory _dupResult
    )
        private
        view
    {
        // Start with all bonusball-only tickets
        _uniqueResult[1] = _tracker.bonusballTicketCounts[_bonusball].count;
        _dupResult[1] = _tracker.bonusballTicketCounts[_bonusball].dupCount;
        
        // Subtract tickets that also match normal balls (they're counted in higher tiers)
        for (uint256 i = 1; i <= _tracker.normalTiers; i++) {
            _uniqueResult[1] -= _uniqueResult[2*i + 1];
            _dupResult[1] -= _dupResult[2*i + 1];
        }
    }

    /**
     * @notice Calculates winning ticket counts across all tiers for given winning numbers
     * @dev Implements three-phase calculation to determine exact winner counts per tier:
     *      1. Count all subset matches across bonusball values
     *      2. Apply inclusion-exclusion principle to remove double-counting
     *      3. Calculate bonusball-only matches (0 normal matches + bonusball)
     * @param _tracker Storage reference to the tracker
     * @param _normalBalls Array of winning normal ball numbers
     * @param _bonusball Winning bonusball number
     * @return winningTicket Bit vector representation of the winning combination
     * @return uniqueResult Array of unique winner counts per tier (indexed by tier ID)
     * @return dupResult Array of duplicate winner counts per tier (indexed by tier ID)
     * @custom:effects
     * - Generates comprehensive winner statistics for payout calculations
     * - Separates unique and duplicate winners for accurate settlement
     * - Covers all 12 tiers: matches(0-5) + bonusball(0/1)
     * @custom:security
     * - Read-only operation with no state changes
     * - Uses mathematical inclusion-exclusion for accurate counting
     * - Prevents over-counting tickets in multiple tiers
     */
    function countTierMatchesWithBonusball(
        Tracker storage _tracker,
        uint8[] memory _normalBalls,
        uint8 _bonusball
    )
        internal
        view
        returns (uint256 winningTicket, uint256[] memory uniqueResult, uint256[] memory dupResult)
    {
        uint256 set = toNormalsBitVector(_normalBalls, _tracker.normalMax);
        winningTicket = set | (1 << (_bonusball + _tracker.normalMax));

        // Step 1: Count all subset matches across all bonusballs
        (uint256[] memory matches, uint256[] memory dupMatches) = _countSubsetMatches(_tracker, set, _bonusball);
        
        // Step 2: Apply inclusion-exclusion principle to remove double counting
        (uniqueResult, dupResult) = _applyInclusionExclusionPrinciple(_tracker, matches, dupMatches);
        
        // Step 3: Calculate bonusball-only matches (no normal balls matched)
        _calculateBonusballOnlyMatches(_tracker, _bonusball, uniqueResult, dupResult);
    }

    /**
     * @notice Checks if a ticket combination has already been inserted into the tracker
     * @dev Efficiently determines duplicate status by checking if the exact combination
     *      has a non-zero count in the tracker's storage.
     * @param _tracker Storage reference to the tracker
     * @param _normalBalls Array of normal ball numbers to check
     * @param _bonusball Bonusball number to check
     * @return True if this exact combination exists in the tracker
     * @custom:requirements
     * - Normal balls array length must match tracker configuration
     * - All ball numbers must be valid per tracker setup
     * @custom:security
     * - Read-only operation with no state changes
     * - Validates input format before processing
     * - Uses efficient bit vector lookup for O(1) duplicate detection
     */
    function isDuplicate(
        Tracker storage _tracker,
        uint8[] memory _normalBalls,
        uint8 _bonusball
    )
        internal
        view
        returns (bool)
    {
        require(_normalBalls.length == _tracker.normalTiers, "Invalid set length");
        uint256 set = toNormalsBitVector(_normalBalls, _tracker.normalMax);
        return _tracker.comboCounts[_bonusball][set].count > 0;
    }

    /**
     * @notice Unpacks a bit vector representation back into separate normal balls and bonusball number
     * @dev Extracts individual ball numbers from a packed ticket by scanning bit positions.
     *      Normal balls are stored at bit positions 1 to _normalMax, bonusball at position (_normalMax + bonusball_value).
     *      Uses LibBit operations for efficient bit scanning and counting to reconstruct original ticket.
     * @param _packedTicket Bit vector representation of the complete ticket
     * @param _normalMax Maximum value for normal balls (defines boundary between normal and bonusball bits)
     * @return normalBalls Array of normal ball numbers extracted from bit positions 1 to _normalMax
     * @return bonusball Bonusball number calculated from highest set bit position minus _normalMax
     * @custom:requirements
     * - Packed ticket must contain valid bit pattern with at least one bonusball bit set
     * - Normal ball bits must be within positions 1 to _normalMax if present
     * - Bonusball bit must be at position > _normalMax
     * @custom:effects
     * - Scans bit vector to extract individual ball numbers in ascending order
     * - Reconstructs original ticket structure from packed representation
     * - No state changes as this is a pure function
     * @custom:security
     * - Read-only operation with no side effects or external calls
     * - Uses efficient LibBit operations to prevent gas issues with large bit vectors
     * - Handles edge cases like single-ball tickets and sparse patterns gracefully
     */
    function unpackTicket(
        uint256 _packedTicket,
        uint8 _normalMax
    )
        internal
        pure
        returns (uint8[] memory normalBalls, uint8 bonusball)
    {
        uint256 ballCount = LibBit.popCount(_packedTicket);
        normalBalls = new uint8[](ballCount - 1);
        uint256 p;
        for (uint256 i = 1; i <= _normalMax; i++) {
            if (_packedTicket & (1 << i) != 0) {
                normalBalls[p++] = uint8(i);
            }
        }

        // Find the bonusball bit position and subtract _normalMax to get the bonusball value
        bonusball = uint8(LibBit.fls(_packedTicket) - _normalMax);
    }
}
END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

/// @notice Library for bit twiddling and boolean operations.
/// @author Solady (https://github.com/vectorized/solady/blob/main/src/utils/LibBit.sol)
/// @author Inspired by (https://graphics.stanford.edu/~seander/bithacks.html)
library LibBit {
    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                  BIT TWIDDLING OPERATIONS                  */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev Find last set.
    /// Returns the index of the most significant bit of `x`,
    /// counting from the least significant bit position.
    /// If `x` is zero, returns 256.
    function fls(uint256 x) internal pure returns (uint256 r) {
        /// @solidity memory-safe-assembly
        assembly {
            r := or(shl(8, iszero(x)), shl(7, lt(0xffffffffffffffffffffffffffffffff, x)))
            r := or(r, shl(6, lt(0xffffffffffffffff, shr(r, x))))
            r := or(r, shl(5, lt(0xffffffff, shr(r, x))))
            r := or(r, shl(4, lt(0xffff, shr(r, x))))
            r := or(r, shl(3, lt(0xff, shr(r, x))))
            // forgefmt: disable-next-item
            r := or(r, byte(and(0x1f, shr(shr(r, x), 0x8421084210842108cc6318c6db6d54be)),
                0x0706060506020504060203020504030106050205030304010505030400000000))
        }
    }

    /// @dev Count leading zeros.
    /// Returns the number of zeros preceding the most significant one bit.
    /// If `x` is zero, returns 256.
    function clz(uint256 x) internal pure returns (uint256 r) {
        /// @solidity memory-safe-assembly
        assembly {
            r := shl(7, lt(0xffffffffffffffffffffffffffffffff, x))
            r := or(r, shl(6, lt(0xffffffffffffffff, shr(r, x))))
            r := or(r, shl(5, lt(0xffffffff, shr(r, x))))
            r := or(r, shl(4, lt(0xffff, shr(r, x))))
            r := or(r, shl(3, lt(0xff, shr(r, x))))
            // forgefmt: disable-next-item
            r := add(xor(r, byte(and(0x1f, shr(shr(r, x), 0x8421084210842108cc6318c6db6d54be)),
                0xf8f9f9faf9fdfafbf9fdfcfdfafbfcfef9fafdfafcfcfbfefafafcfbffffffff)), iszero(x))
        }
    }

    /// @dev Find first set.
    /// Returns the index of the least significant bit of `x`,
    /// counting from the least significant bit position.
    /// If `x` is zero, returns 256.
    /// Equivalent to `ctz` (count trailing zeros), which gives
    /// the number of zeros following the least significant one bit.
    function ffs(uint256 x) internal pure returns (uint256 r) {
        /// @solidity memory-safe-assembly
        assembly {
            // Isolate the least significant bit.
            x := and(x, add(not(x), 1))
            // For the upper 3 bits of the result, use a De Bruijn-like lookup.
            // Credit to adhusson: https://blog.adhusson.com/cheap-find-first-set-evm/
            // forgefmt: disable-next-item
            r := shl(5, shr(252, shl(shl(2, shr(250, mul(x,
                0xb6db6db6ddddddddd34d34d349249249210842108c6318c639ce739cffffffff))),
                0x8040405543005266443200005020610674053026020000107506200176117077)))
            // For the lower 5 bits of the result, use a De Bruijn lookup.
            // forgefmt: disable-next-item
            r := or(r, byte(and(div(0xd76453e0, shr(r, x)), 0x1f),
                0x001f0d1e100c1d070f090b19131c1706010e11080a1a141802121b1503160405))
        }
    }

    /// @dev Returns the number of set bits in `x`.
    function popCount(uint256 x) internal pure returns (uint256 c) {
        /// @solidity memory-safe-assembly
        assembly {
            let max := not(0)
            let isMax := eq(x, max)
            x := sub(x, and(shr(1, x), div(max, 3)))
            x := add(and(x, div(max, 5)), and(shr(2, x), div(max, 5)))
            x := and(add(x, shr(4, x)), div(max, 17))
            c := or(shl(8, isMax), shr(248, mul(x, div(max, 255))))
        }
    }

    /// @dev Returns the number of zero bytes in `x`.
    /// To get the number of non-zero bytes, simply do `32 - countZeroBytes(x)`.
    function countZeroBytes(uint256 x) internal pure returns (uint256 c) {
        /// @solidity memory-safe-assembly
        assembly {
            let m := 0x7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f
            c := byte(0, mul(shr(7, not(m)), shr(7, not(or(or(add(and(x, m), m), x), m)))))
        }
    }

    /// @dev Returns the number of zero bytes in `s`.
    /// To get the number of non-zero bytes, simply do `s.length - countZeroBytes(s)`.
    function countZeroBytes(bytes memory s) internal pure returns (uint256 c) {
        /// @solidity memory-safe-assembly
        assembly {
            function czb(x_) -> _c {
                let _m := 0x7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f
                _c := shr(7, not(or(or(add(and(x_, _m), _m), x_), _m)))
                _c := byte(0, mul(shr(7, not(_m)), _c))
            }
            let n := mload(s)
            let l := shl(5, shr(5, n))
            s := add(s, 0x20)
            for { let i } xor(i, l) { i := add(i, 0x20) } { c := add(czb(mload(add(s, i))), c) }
            if lt(l, n) { c := add(czb(or(shr(shl(3, sub(n, l)), not(0)), mload(add(s, l)))), c) }
        }
    }

    /// @dev Returns the number of zero bytes in `s`.
    /// To get the number of non-zero bytes, simply do `s.length - countZeroBytes(s)`.
    function countZeroBytesCalldata(bytes calldata s) internal pure returns (uint256 c) {
        /// @solidity memory-safe-assembly
        assembly {
            function czb(x_) -> _c {
                let _m := 0x7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f
                _c := shr(7, not(or(or(add(and(x_, _m), _m), x_), _m)))
                _c := byte(0, mul(shr(7, not(_m)), _c))
            }
            let l := shl(5, shr(5, s.length))
            for { let i } xor(i, l) { i := add(i, 0x20) } {
                c := add(czb(calldataload(add(s.offset, i))), c)
            }
            if lt(l, s.length) {
                let m := shr(shl(3, sub(s.length, l)), not(0))
                c := add(czb(or(m, calldataload(add(s.offset, l)))), c)
            }
        }
    }

    /// @dev Returns whether `x` is a power of 2.
    function isPo2(uint256 x) internal pure returns (bool result) {
        /// @solidity memory-safe-assembly
        assembly {
            // Equivalent to `x && !(x & (x - 1))`.
            result := iszero(add(and(x, sub(x, 1)), iszero(x)))
        }
    }

    /// @dev Returns `x` reversed at the bit level.
    function reverseBits(uint256 x) internal pure returns (uint256 r) {
        uint256 m0 = 0x0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f;
        uint256 m1 = m0 ^ (m0 << 2);
        uint256 m2 = m1 ^ (m1 << 1);
        r = reverseBytes(x);
        r = (m2 & (r >> 1)) | ((m2 & r) << 1);
        r = (m1 & (r >> 2)) | ((m1 & r) << 2);
        r = (m0 & (r >> 4)) | ((m0 & r) << 4);
    }

    /// @dev Returns `x` reversed at the byte level.
    function reverseBytes(uint256 x) internal pure returns (uint256 r) {
        unchecked {
            // Computing masks on-the-fly reduces bytecode size by about 200 bytes.
            uint256 m0 = 0x100000000000000000000000000000001 * (~toUint(x == uint256(0)) >> 192);
            uint256 m1 = m0 ^ (m0 << 32);
            uint256 m2 = m1 ^ (m1 << 16);
            uint256 m3 = m2 ^ (m2 << 8);
            r = (m3 & (x >> 8)) | ((m3 & x) << 8);
            r = (m2 & (r >> 16)) | ((m2 & r) << 16);
            r = (m1 & (r >> 32)) | ((m1 & r) << 32);
            r = (m0 & (r >> 64)) | ((m0 & r) << 64);
            r = (r >> 128) | (r << 128);
        }
    }

    /// @dev Returns the common prefix of `x` and `y` at the bit level.
    function commonBitPrefix(uint256 x, uint256 y) internal pure returns (uint256) {
        unchecked {
            uint256 s = 256 - clz(x ^ y);
            return (x >> s) << s;
        }
    }

    /// @dev Returns the common prefix of `x` and `y` at the nibble level.
    function commonNibblePrefix(uint256 x, uint256 y) internal pure returns (uint256) {
        unchecked {
            uint256 s = (64 - (clz(x ^ y) >> 2)) << 2;
            return (x >> s) << s;
        }
    }

    /// @dev Returns the common prefix of `x` and `y` at the byte level.
    function commonBytePrefix(uint256 x, uint256 y) internal pure returns (uint256) {
        unchecked {
            uint256 s = (32 - (clz(x ^ y) >> 3)) << 3;
            return (x >> s) << s;
        }
    }

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                     BOOLEAN OPERATIONS                     */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    // A Solidity bool on the stack or memory is represented as a 256-bit word.
    // Non-zero values are true, zero is false.
    // A clean bool is either 0 (false) or 1 (true) under the hood.
    // Usually, if not always, the bool result of a regular Solidity expression,
    // or the argument of a public/external function will be a clean bool.
    // You can usually use the raw variants for more performance.
    // If uncertain, test (best with exact compiler settings).
    // Or use the non-raw variants (compiler can sometimes optimize out the double `iszero`s).

    /// @dev Returns `x & y`. Inputs must be clean.
    function rawAnd(bool x, bool y) internal pure returns (bool z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := and(x, y)
        }
    }

    /// @dev Returns `x & y`.
    function and(bool x, bool y) internal pure returns (bool z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := and(iszero(iszero(x)), iszero(iszero(y)))
        }
    }

    /// @dev Returns `w & x & y`.
    function and(bool w, bool x, bool y) internal pure returns (bool z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := iszero(or(iszero(w), or(iszero(x), iszero(y))))
        }
    }

    /// @dev Returns `v & w & x & y`.
    function and(bool v, bool w, bool x, bool y) internal pure returns (bool z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := iszero(or(or(iszero(v), iszero(w)), or(iszero(x), iszero(y))))
        }
    }

    /// @dev Returns `x | y`. Inputs must be clean.
    function rawOr(bool x, bool y) internal pure returns (bool z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := or(x, y)
        }
    }

    /// @dev Returns `x | y`.
    function or(bool x, bool y) internal pure returns (bool z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := iszero(iszero(or(x, y)))
        }
    }

    /// @dev Returns `w | x | y`.
    function or(bool w, bool x, bool y) internal pure returns (bool z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := iszero(iszero(or(w, or(x, y))))
        }
    }

    /// @dev Returns `v | w | x | y`.
    function or(bool v, bool w, bool x, bool y) internal pure returns (bool z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := iszero(iszero(or(v, or(w, or(x, y)))))
        }
    }

    /// @dev Returns 1 if `b` is true, else 0. Input must be clean.
    function rawToUint(bool b) internal pure returns (uint256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := b
        }
    }

    /// @dev Returns 1 if `b` is true, else 0.
    function toUint(bool b) internal pure returns (uint256 z) {
        /// @solidity memory-safe-assembly
        assembly {
            z := iszero(iszero(b))
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8;

import { LibBit } from "solady/src/utils/LibBit.sol";

library Combinations {
    uint256 constant UINT256_BIT_WIDTH = 256;
    /// @notice Compute number of combinations of size k from a set of n
    /// @param n Size of set to choose from
    /// @param k Size of subsets to choose
    function choose(
        uint256 n,
        uint256 k
    ) internal pure returns (uint256 result) {
        assert(n >= k);
        assert(n <= 128); // Artificial limit to avoid overflow
        // "How to calculate binomial coefficients"
        // From: https://blog.plover.com/math/choose.html
        // This algorithm computes multiplication and division in alternation
        // to avoid overflow as much as possible.
        unchecked {
            uint256 out = 1;
            for (uint256 d = 1; d <= k; ++d) {
                out *= n--;
                out /= d;
            }
            return out;
        }
    }

    /// @notice Generate all possible subsets of size k from a bit vector.
    /// @param set Bit vector to generate subsets from
    /// @param k Size of subsets to generate
    function generateSubsets(
        uint256 set,
        uint256 k
    ) internal pure returns (uint256[] memory subsets) {
        unchecked {
            uint256 n = LibBit.popCount(set);
            assert(k <= n);
            subsets = new uint256[](choose(n, k));

            uint256 bound = 1 << n;
            uint256 comb = (1 << k) - 1;
            uint256 count;
            while (comb < bound) {
                uint256 mapped;
                uint256 _set = set;
                uint256 _comb = comb;
                for (uint256 i; i < UINT256_BIT_WIDTH && _set != 0; ++i) {
                    if (_set & 1 == 1) {
                        if (_comb & 1 == 1) {
                            mapped |= (1 << i);
                        }
                        _comb >>= 1;
                    }
                    _set >>= 1;
                }

                subsets[count++] = mapped;

                // "Gosper's hack"
                uint256 c = comb & uint256(-int256(comb));
                uint256 r = comb + c;
                comb = (((r ^ comb) >> 2) / c) | r;
            }
            assert(count == choose(n, k));
        }
    }
}

## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS

## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

