
## *MAIN TARGET CONTRACT* TO REVIEW

//SPDX-License-Identifier: UNLICENSED

/*
Copyright (C) 2025 Coordination Inc.
All rights reserved.

This software is proprietary and confidential. Unauthorized copying,
distribution, or use is strictly prohibited and may result in legal action.

For licensing inquiries: legal@coordinationlabs.com
*/

pragma solidity ^0.8.28;

import { ERC721 } from "solady/src/tokens/ERC721.sol";

import { IJackpot } from "./interfaces/IJackpot.sol";
import { IJackpotTicketNFT } from "./interfaces/IJackpotTicketNFT.sol";


/**
 * @title JackpotTicketNFT
 * @notice ERC-721 implementation for jackpot tickets with tracking and transfer functionality
 * @dev Implements jackpot tickets as transferable NFTs with:
 *      - Packed ticket number storage for efficient gas usage
 *      - User ticket tracking per drawing for easy querying
 *      - Referral scheme association for winnings distribution
 *      - Automatic ticket list management on transfers
 *      - Integration with Jackpot contract for minting and burning
 */
contract JackpotTicketNFT is ERC721, IJackpotTicketNFT {

    // =============================================================
    //                           STRUCTS
    // =============================================================
    struct UserTickets {
        uint256 totalTicketsBought;
        mapping(uint256 => uint256) ticketIds;
        mapping(uint256 => uint256) indexOfTicketId;
    }

    // =============================================================
    //                       ERRORS
    // =============================================================

    error UnauthorizedCaller();

    // =============================================================
    //                       STATE VARIABLES
    // =============================================================

    // User and ticket mappings
    mapping(address => mapping(uint256 => UserTickets)) internal userTickets; // user address => drawing => UserTickets
    mapping(uint256 => TrackedTicket) public tickets; // ticketId → ticket info

    IJackpot public immutable jackpot;

    // =============================================================
    //                       MODIFIERS
    // =============================================================

    modifier onlyJackpot() {
        if (msg.sender != address(jackpot)) revert UnauthorizedCaller();
        _;
    }

    // =============================================================
    //                       CONSTRUCTOR
    // =============================================================

    /**
     * @notice Initializes the JackpotTicketNFT with the Jackpot contract reference
     * @dev Sets up the connection to the main Jackpot contract that will mint and burn tickets
     * @param _jackpot Address of the main Jackpot contract
     * @custom:effects
     * - Sets jackpot contract reference as immutable
     * - Inherits ERC721 functionality for NFT operations
     * @custom:security
     * - Immutable jackpot reference prevents unauthorized contract changes
     */
    constructor(IJackpot _jackpot) {
        jackpot = _jackpot;
    }

    // =============================================================
    //                       EXTERNAL FUNCTIONS
    // =============================================================

    /**
     * @notice Mints a new ticket NFT with jackpot information
     * @dev Creates an ERC-721 token representing a jackpot ticket with embedded metadata.
     *      Automatically adds ticket to user's ticket list for the drawing.
     * @param _recipient Address to receive the minted ticket
     * @param _ticketId Unique identifier for the ticket (used as token ID)
     * @param _drawingId Drawing the ticket is for
     * @param _packedTicket Packed ticket numbers (normal numbers + bonusball)
     * @param _referralScheme Hash of referral scheme used for this ticket
     * @custom:requirements
     * - Only Jackpot contract can call
     * - Ticket ID must be unique (ERC721 enforces this)
     * - Recipient address must not be zero (ERC721 enforces this)
     * @custom:emits Transfer (ERC-721 standard)
     * @custom:effects
     * - Mints ERC-721 token to specified address
     * - Stores ticket metadata in contract storage
     * - Adds ticket to user's ticket list via _afterTokenTransfer
     * @custom:security
     * - Access restricted to Jackpot contract
     * - Unique ticket ID enforcement via ERC721
     * - Automatic user ticket tracking
     */
    function mintTicket(
        address _recipient,
        uint256 _ticketId,
        uint256 _drawingId,
        uint256 _packedTicket,
        bytes32 _referralScheme
    ) external onlyJackpot {
        tickets[_ticketId] = TrackedTicket({
            drawingId: _drawingId,
            packedTicket: _packedTicket,
            referralScheme: _referralScheme
        });

        _mint(_recipient, _ticketId);
    }

    function burnTicket(uint256 _ticketId) external onlyJackpot {
        _burn(_ticketId);
    }

    // =============================================================
    //                       VIEW FUNCTIONS
    // =============================================================

    function getUserTickets(address _userAddress, uint256 _drawingId) external view returns (ExtendedTrackedTicket[] memory) {
        UserTickets storage userDrawingTickets = userTickets[_userAddress][_drawingId];
        ExtendedTrackedTicket[] memory userTicketsList = new ExtendedTrackedTicket[](userDrawingTickets.totalTicketsBought);
        for (uint256 i = 0; i < userDrawingTickets.totalTicketsBought; i++) {
            uint256 ticketId = userDrawingTickets.ticketIds[i];
            userTicketsList[i] = _getExtendedTicketInfo(ticketId);
        }
        return userTicketsList;
    }

    function getTicketInfo(uint256 _ticketId) external view returns (TrackedTicket memory) {
        return tickets[_ticketId];
    }

    function getExtendedTicketInfo(uint256 _ticketId) external view returns (ExtendedTrackedTicket memory) {
        return _getExtendedTicketInfo(_ticketId);
    }
    
    function name() public pure override returns (string memory) {
        return "Jackpot";
    }

    function symbol() public pure override returns (string memory) {
        return "JACKPOT";
    }

    function tokenURI(uint256 /* tokenId */) public pure override returns (string memory) {
        return "";
    }

    // =============================================================
    //                       INTERNAL FUNCTIONS
    // =============================================================

    function _beforeTokenTransfer(address _from, address /* _to */, uint256 _tokenId) internal override {
        if (_from != address(0)) {
            TrackedTicket memory ticketInfo = tickets[_tokenId];
            UserTickets storage fromTickets = userTickets[_from][ticketInfo.drawingId];
            uint256 idx = fromTickets.indexOfTicketId[_tokenId];
            uint256 lastIdx = fromTickets.totalTicketsBought - 1;
            if (idx != lastIdx) {
                uint256 swapId = fromTickets.ticketIds[lastIdx];
                fromTickets.ticketIds[idx] = swapId;
                fromTickets.indexOfTicketId[swapId] = idx;
            }
            delete fromTickets.ticketIds[lastIdx];
            delete fromTickets.indexOfTicketId[_tokenId];
            fromTickets.totalTicketsBought -= 1;
        }
    }

    function _afterTokenTransfer(address /* _from */, address _to, uint256 _tokenId) internal override {
        if (_to != address(0)) {
            TrackedTicket memory ticketInfo = tickets[_tokenId];
            UserTickets storage toTickets = userTickets[_to][ticketInfo.drawingId];
            uint256 newIdx = toTickets.totalTicketsBought;
            toTickets.ticketIds[newIdx] = _tokenId;
            toTickets.indexOfTicketId[_tokenId] = newIdx;
            toTickets.totalTicketsBought += 1;
        }
    }

    function _getExtendedTicketInfo(uint256 _ticketId) internal view returns (ExtendedTrackedTicket memory) {
        (uint8[] memory normals, uint8 bonusball) = jackpot.getUnpackedTicket(tickets[_ticketId].drawingId, tickets[_ticketId].packedTicket);
        return ExtendedTrackedTicket({
            ticketId: _ticketId,
            ticket: tickets[_ticketId],
            normals: normals,
            bonusball: bonusball
        });
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

//SPDX-License-Identifier: UNLICENSED

/*
Copyright (C) 2025 Coordination Inc.
All rights reserved.

This software is proprietary and confidential. Unauthorized copying,
distribution, or use is strictly prohibited and may result in legal action.

For licensing inquiries: legal@coordinationlabs.com
*/

pragma solidity ^0.8.28;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { IERC721 } from "@openzeppelin/contracts/token/ERC721/IERC721.sol";
import { LibBit } from "solady/src/utils/LibBit.sol";
import { Math } from "@openzeppelin/contracts/utils/math/Math.sol";
import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";
import { Ownable2Step } from "@openzeppelin/contracts/access/Ownable2Step.sol";
import { ReentrancyGuardTransient } from "@openzeppelin/contracts/utils/ReentrancyGuardTransient.sol";
import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

import { Combinations } from "./lib/Combinations.sol";
import { IJackpot } from "./interfaces/IJackpot.sol";
import { IJackpotLPManager } from "./interfaces/IJackpotLPManager.sol";
import { IJackpotTicketNFT } from "./interfaces/IJackpotTicketNFT.sol";
import { IPayoutCalculator } from "./interfaces/IPayoutCalculator.sol";
import { IScaledEntropyProvider } from "./interfaces/IScaledEntropyProvider.sol";
import { JackpotErrors } from "./lib/JackpotErrors.sol";
import { TicketComboTracker } from "./lib/TicketComboTracker.sol";
import { UintCasts } from "./lib/UintCasts.sol";

/**
 * @title Jackpot
 * @notice Main jackpot contract that orchestrates all jackpot operations including ticket purchases, drawings, and prize distribution
 * @dev Implements a decentralized jackpot system with NFT-based tickets, LP-managed prize pools, and provably fair drawings using Pyth Network entropy
 */
contract Jackpot is IJackpot, Ownable2Step, ReentrancyGuardTransient {

    using SafeERC20 for IERC20;
    using UintCasts for uint256;
    using UintCasts for uint256[];

    // =============================================================
    //                           STRUCTS
    // =============================================================

    struct DrawingState {
        uint256 prizePool;
        uint256 ticketPrice;
        uint256 edgePerTicket;
        uint256 referralWinShare;
        uint256 globalTicketsBought;
        uint256 lpEarnings;
        uint8 ballMax;
        uint8 bonusballMax;
        uint256 drawingTime;
        uint256 winningTicket;
        bool jackpotLock;
    }

    struct ReferralScheme {
        address[] referrers;
        uint256[] referralSplit;
    }

    // =============================================================
    //                           EVENTS
    // =============================================================

    event TicketOrderProcessed(
        address indexed buyer,
        address indexed recipient,
        uint256 indexed currentDrawingId,
        uint256 numberOfTickets,
        uint256 lpEarnings,
        uint256 referralFees
    );

    event TicketPurchased(
        address indexed recipient,
        uint256 indexed currentDrawingId,
        bytes32 indexed source, // Note: this is used for telemetry purposes
        uint256 userTicketId,
        uint8[] normals,
        uint8 bonusball,
        bytes32 referralScheme
    );

    event ReferralFeeCollected(
        address indexed referrer,
        uint256 amount
    );

    event ReferralSchemeAdded(
        bytes32 indexed referralSchemeId,
        address[] referrers,
        uint256[] referralSplit
    );

    event TicketWinningsClaimed(
        address indexed userAddress,
        uint256 indexed drawingId,
        uint256 userTicketId,
        uint256 matchedNormals,
        bool bonusballMatch,
        uint256 winningsAmount
    );

    event TicketRefunded(uint256 indexed ticketId);

    event ReferralFeesClaimed(
        address indexed userAddress,
        uint256 amount
    );

    event JackpotSettled(
        uint256 indexed drawingId,
        uint256 totalTicketsSold,
        uint256 userWinnings,
        uint8 winningBonusball,
        uint256 winningNumbers,
        uint256 newDrawingAccumulator
    );

    event WinnersCalculated(
        uint256 indexed drawingId,
        uint256[] winningNormals,
        uint256 winningBonusball,
        uint256[] uniqueResult,
        uint256[] dupResult
    );

    event NewDrawingInitialized(
        uint256 indexed drawingId,
        uint256 lpPoolTotal,
        uint256 prizePool,
        uint256 ticketPrice,
        uint256 normalBallMax,
        uint8 bonusballMax,
        uint256 referralWinShare,
        uint256 drawingTime
    );

    event JackpotRunRequested(
        uint256 indexed drawingId,
        uint256 entropyGasLimit,
        uint256 fee
    );

    event LpEarningsUpdated(
        uint256 indexed drawingId,
        uint256 amount
    );

    event ProtocolFeeCollected(
        uint256 indexed drawingId,
        uint256 amount
    );

    // Governance Events
    event NormalBallMaxUpdated(uint256 indexed drawingId, uint8 oldValue, uint8 newValue);
    event ProtocolFeeThresholdUpdated(uint256 indexed drawingId, uint256 oldValue, uint256 newValue);
    event ProtocolFeeUpdated(uint256 indexed drawingId, uint256 oldValue, uint256 newValue);
    event GovernancePoolCapUpdated(uint256 indexed drawingId, uint256 oldValue, uint256 newValue);
    event DrawingDurationUpdated(uint256 indexed drawingId, uint256 oldValue, uint256 newValue);
    event BonusballMinUpdated(uint256 indexed drawingId, uint8 oldValue, uint8 newValue);
    event LpEdgeTargetUpdated(uint256 indexed drawingId, uint256 oldValue, uint256 newValue);
    event ReserveRatioUpdated(uint256 indexed drawingId, uint256 oldValue, uint256 newValue);
    event ReferralFeeUpdated(uint256 indexed drawingId, uint256 oldValue, uint256 newValue);
    event ReferralWinShareUpdated(uint256 indexed drawingId, uint256 oldValue, uint256 newValue);
    event ProtocolFeeAddressUpdated(uint256 indexed drawingId, address indexed oldAddress, address indexed newAddress);
    event TicketPriceUpdated(uint256 indexed drawingId, uint256 oldValue, uint256 newValue);
    event MaxReferrersUpdated(uint256 indexed drawingId, uint256 oldValue, uint256 newValue);
    event PayoutCalculatorUpdated(uint256 indexed drawingId, address oldPayoutCalculator, address newPayoutCalculator);
    event EntropyBaseGasLimitUpdated(uint256 indexed drawingId, uint32 oldValue, uint32 newValue);
    event EntropyVariableGasLimitUpdated(uint256 indexed drawingId, uint32 oldValue, uint32 newValue);
    event JackpotLocked(uint256 indexed drawingId);
    event JackpotUnlocked(uint256 indexed drawingId);
    event TicketPurchasesEnabled(uint256 indexed drawingId);
    event TicketPurchasesDisabled(uint256 indexed drawingId);
    event EntropyUpdated(uint256 indexed drawingId, address oldEntropy, address newEntropy);
    event EmergencyModeEnabled(uint256 indexed drawingId);
    event EmergencyModeDisabled(uint256 indexed drawingId);

    // =============================================================
    //                          CONSTANTS
    // =============================================================

    uint256 constant PRECISE_UNIT = 1e18;
    uint8 constant NORMAL_BALL_COUNT = 5;
    uint8 constant MAX_BIT_VECTOR_SIZE = 255;
    uint256 constant MAX_PROTOCOL_FEE = 25e16; // 25%

    // =============================================================
    //                       STATE VARIABLES
    // =============================================================

    // User and ticket mappings
    mapping(uint256 => TicketComboTracker.Tracker) internal drawingEntries; // drawing => TicketComboTracker
    mapping(uint256 => DrawingState) internal drawingState; // drawing => drawing state
    
    // Fee and LP mappings
    mapping(address => uint256) public referralFees;
    
    // Drawing and tier mappings
    mapping(bytes32 => ReferralScheme) internal referralSchemes;
    
    // Core state variables
    uint256 public currentDrawingId;

    // Used in between drawings - should be stored in each drawing's state in case admin
    // updates are made to params for next drawing (referralWinShare technically also needs
    // to be stored in each drawing's state but it is located in fees section)
    uint256 public ticketPrice;
    uint8 public normalBallMax;
    uint8 public bonusballMin;

    // Used at drawing settlement - do not need to be stored in each drawing's state
    uint256 public drawingDurationInSeconds; // Used in drawingTime
    uint256 public reserveRatio; // Used in prize pool and ticket calcs
    uint256 public lpEdgeTarget; // Used in edgePerTicket
    
    uint256 public governancePoolCap;

    // Fees
    uint256 public referralFee;
    uint256 public referralWinShare;
    uint256 public protocolFee;
    uint256 public protocolFeeThreshold;
    address public protocolFeeAddress;
    uint256 public maxReferrers;

    bool public initialized;
    bool public allowTicketPurchases;
    bool public emergencyMode;

    // Gas limits for entropy provider; base gas is the non-variable gas limit portion of the total gas
    // Variable gas is the amount of gas that needs to be added per bonusball used (drawingState.bonusballMax)
    uint32 public entropyBaseGasLimit;
    uint32 public entropyVariableGasLimit;
    
    // External contracts and contract settings
    IERC20 public usdc;
    IJackpotLPManager public jackpotLPManager;
    IJackpotTicketNFT public jackpotNFT;
    IScaledEntropyProvider public entropy;
    IPayoutCalculator public payoutCalculator;

    // =============================================================
    //                          MODIFIERS
    // =============================================================

    modifier onlyEntropy() {
        if (msg.sender != address(entropy)) revert JackpotErrors.UnauthorizedEntropyCaller();
        _;
    }

    modifier noEmergencyMode() {
        if (emergencyMode) revert JackpotErrors.EmergencyEnabled();
        _;
    }

    modifier onlyEmergencyMode() {
        if (!emergencyMode) revert JackpotErrors.EmergencyModeNotEngaged();
        _;
    }

    // =============================================================
    //                         CONSTRUCTOR
    // =============================================================

    /**
     * @notice Initializes the Jackpot contract with core jackpot parameters
     * @dev Sets initial jackpot configuration including ball ranges, fees, and timing.
     *      Most parameters can be updated later via admin functions. 
     *      The contract requires additional initialization via initialize(), initializeLPDeposits(), and initializeJackpot().
     * @param _drawingDurationInSeconds Time between jackpot drawings in seconds
     * @param _normalBallMax Maximum value for normal ball numbers (1 to this value)
     * @param _bonusballMin Minimum number of bonusball options (affects prize pool sizing)
     * @param _lpEdgeTarget Target profit margin for liquidity providers (in PRECISE_UNIT scale)
     * @param _reserveRatio Fraction of LP pool held in reserve (in PRECISE_UNIT scale)
     * @param _referralFee Fraction of ticket price paid as referral fees (in PRECISE_UNIT scale)
     * @param _referralWinShare Fraction of winnings shared with referrers (in PRECISE_UNIT scale)
     * @param _protocolFee Fraction of excess LP earnings taken as protocol fee (in PRECISE_UNIT scale)
     * @param _protocolFeeThreshold Minimum LP profit before protocol fees apply
     * @param _ticketPrice Price per ticket in USDC wei (6 decimals)
     * @param _maxReferrers Maximum number of referrers allowed per ticket purchase
     * @param _entropyBaseGasLimit Gas limit for entropy provider callback (uint32)
     * @custom:effects
     * - Sets all core jackpot parameters
     * - Sets deployer as initial owner and protocol fee recipient
     * - Contract remains uninitialized until initialize() is called
     */
    constructor(
        uint256 _drawingDurationInSeconds,
        uint8 _normalBallMax,
        uint8 _bonusballMin,
        uint256 _lpEdgeTarget,
        uint256 _reserveRatio,
        uint256 _referralFee,
        uint256 _referralWinShare,
        uint256 _protocolFee,
        uint256 _protocolFeeThreshold,
        uint256 _ticketPrice,
        uint256 _maxReferrers,
        uint32 _entropyBaseGasLimit
    ) Ownable(msg.sender) {
        drawingDurationInSeconds = _drawingDurationInSeconds;
        normalBallMax = _normalBallMax;
        bonusballMin = _bonusballMin;
        lpEdgeTarget = _lpEdgeTarget;
        reserveRatio = _reserveRatio;
        referralFee = _referralFee;
        referralWinShare = _referralWinShare;
        protocolFee = _protocolFee;
        protocolFeeThreshold = _protocolFeeThreshold;
        ticketPrice = _ticketPrice;
        maxReferrers = _maxReferrers;
        entropyBaseGasLimit = _entropyBaseGasLimit;

        entropyVariableGasLimit = uint32(250000);
        protocolFeeAddress = msg.sender;
    }

    // =============================================================
    //                      EXTERNAL FUNCTIONS
    // =============================================================

    /**
     * @notice Allows users to purchase jackpot tickets for the current drawing
     * @dev Validates tickets, processes referral fees, mints NFT tickets, and updates drawing state.
     *      Each ticket becomes an ERC-721 NFT that can be transferred or claimed for winnings.
     *      Duplicate tickets are allowed and tracked separately in the combo tracker. When a duplicate is purchased,
     *      prizePool increases by ticketPrice*(PRECISE_UNIT - lpEdgeTarget)/PRECISE_UNIT to preserve LP edge.
     * @param _tickets Array of ticket structs containing normal numbers (5) and bonusball number
     * @param _recipient Address that will receive the minted ticket NFTs
     * @param _referrers Array of referrer addresses for fee sharing (can be empty)
     * @param _referralSplit Array of PRECISE_UNIT-scaled referral weights (must sum to PRECISE_UNIT if provided)
     * @param _source Bytes32 identifier for tracking ticket purchase source (telemetry)
     * @return ticketIds Array of minted ticket IDs (NFT token IDs)
     * @custom:requirements
     * - Ticket purchases must be enabled (allowTicketPurchases == true)
     * - Drawing must not be locked (jackpotLock == false)
     * - Drawing must have an active prize pool (prizePool > 0)
     * - Tickets must have exactly 5 normal numbers and valid bonusball
     * - Normal numbers must be in range [1, ballMax] and unique
     * - Bonusball must be in range [1, bonusballMax]
     * - Referrer arrays must match in length and sum to PRECISE_UNIT
     * - Caller must have sufficient USDC balance and approval
     * - Emergency mode must not be active
     * @custom:emits TicketOrderProcessed, TicketPurchased (per ticket), ReferralFeeCollected (per referrer)
     * @custom:effects
     * - Transfers USDC from caller to contract
     * - Mints NFT tickets to recipient
     * - Updates drawing state (lpEarnings, globalTicketsBought, prizePool if duplicates)
     * - Distributes referral fees to referrers
     * - Stores tickets in combo tracker for scalable settlement calculations
     * @custom:security
     * - Reentrancy protection via nonReentrant modifier
     * - Input validation for all parameters
     * - Safe USDC transfers with approval checks
     */
    function buyTickets(
        Ticket[] memory _tickets,
        address _recipient,
        address[] memory _referrers,
        uint256[] memory _referralSplit,
        bytes32 _source
    )
        external
        nonReentrant
        noEmergencyMode
        returns (uint256[] memory ticketIds)
    {
        _validateBuyTicketInputs(_tickets, _recipient, _referrers, _referralSplit);

        DrawingState storage currentDrawingState = drawingState[currentDrawingId];
        
        uint256 numTicketsBought = _tickets.length;
        uint256 ticketsValue = numTicketsBought * currentDrawingState.ticketPrice;
        (uint256 referralFeeTotal, bytes32 referralSchemeId) = _validateAndTrackReferrals(_referrers, _referralSplit, ticketsValue);

        usdc.safeTransferFrom(msg.sender, address(this), ticketsValue);

        ticketIds = _validateAndStoreTickets(currentDrawingState, _tickets, _recipient, referralSchemeId, _source);

        currentDrawingState.lpEarnings += ticketsValue - referralFeeTotal;
        currentDrawingState.globalTicketsBought += numTicketsBought;

        emit TicketOrderProcessed(msg.sender, _recipient, currentDrawingId, numTicketsBought, ticketsValue - referralFeeTotal, referralFeeTotal);
    }

    /**
     * @notice Allows ticket holders to claim winnings from completed drawings
     * @dev Burns ticket NFTs, calculates winnings based on tier payouts, processes referral shares,
     *      and transfers net winnings to the caller. Only ticket owners can claim their winnings.
     * @param _userTicketIds Array of ticket IDs (NFT token IDs) to claim winnings for
     * @custom:requirements
     * - Caller must own all specified tickets (verified via ERC721.ownerOf)
     * - Tickets must be from completed drawings (drawingId < currentDrawingId)
     * - At least one ticket ID must be provided
     * - Drawing results must be finalized (scaledEntropyCallback completed)
     * @custom:emits TicketWinningsClaimed (per ticket), ReferralFeeCollected (per referrer share)
     * @custom:effects
     * - Burns the ticket NFTs (prevents double-claiming)
     * - Transfers net USDC winnings to caller
     * - If no referral scheme is set for the ticket, the referrer share of the winnings is added to current drawing's lpEarnings
     * - Updates referral fee balances for associated referrers
     * - Calculates tier payouts based on number matches
     * @custom:security
     * - Reentrancy protection via nonReentrant modifier
     * - Ownership verification for each ticket
     * - Drawing completion verification
     * - Automatic NFT burning to prevent double claims
     */
    function claimWinnings(uint256[] memory _userTicketIds) external nonReentrant {
        if (_userTicketIds.length == 0) revert JackpotErrors.NoTicketsToClaim();
        
        uint256 totalClaimAmount = 0;
        for (uint256 i = 0; i < _userTicketIds.length; i++) {
            uint256 ticketId = _userTicketIds[i];
            IJackpotTicketNFT.TrackedTicket memory ticketInfo = jackpotNFT.getTicketInfo(ticketId);
            uint256 drawingId = ticketInfo.drawingId;
            if (IERC721(address(jackpotNFT)).ownerOf(ticketId) != msg.sender) revert JackpotErrors.NotTicketOwner();
            if (drawingId >= currentDrawingId) revert JackpotErrors.TicketFromFutureDrawing();

            DrawingState memory winningDrawingState = drawingState[drawingId];
            uint256 tierId = _calculateTicketTierId(ticketInfo.packedTicket, winningDrawingState.winningTicket, winningDrawingState.ballMax);
            jackpotNFT.burnTicket(ticketId);
            
            uint256 winningAmount = payoutCalculator.getTierPayout(drawingId, tierId);
            uint256 referrerShare = _payReferrersWinnings(
                ticketInfo.referralScheme,
                winningAmount,
                winningDrawingState.referralWinShare
            );
            
            totalClaimAmount += winningAmount - referrerShare;
            emit TicketWinningsClaimed(
                msg.sender,
                drawingId,
                ticketId,
                tierId / 2,             // matches
                (tierId % 2) == 1,      // bonusball match
                winningAmount - referrerShare
            );
        }

        usdc.safeTransfer(msg.sender, totalClaimAmount);
    }

    /**
     * @notice Allows liquidity providers to deposit USDC into the prize pool
     * @dev Deposits are processed immediately but are not added to the prize pool until the next drawing. 
     *      LP shares are calculated based on the accumulator at the end of the current drawing.
     *      If the LP has a previous deposit from an earlier drawing, shares are consolidated before processing the new deposit.
     * @param _amountToDeposit The amount of USDC to deposit (in wei, 6 decimals for USDC)
     * @custom:requirements
     * - Drawing must not be locked (jackpotLock == false)
     * - Deposit amount must be greater than 0
     * - Total pool size after deposit must not exceed lpPoolCap
     * - Caller must have sufficient USDC balance and approval
     * - Emergency mode must not be active
     * @custom:emits LpDeposited (emitted by LPManager)
     * @custom:effects
     * - Transfers USDC from caller to contract
     * - Creates or updates LP position in current drawing
     * - May consolidate previous deposits from earlier drawings
     * - Updates total LP pool value
     * @custom:security
     * - Reentrancy protection via nonReentrant modifier
     * - Pool cap validation to prevent over-deposits
     * - Safe USDC transfers with approval checks
     */
    function lpDeposit(uint256 _amountToDeposit) external nonReentrant noEmergencyMode {
        if (drawingState[currentDrawingId].jackpotLock) revert JackpotErrors.JackpotLocked();
        if (_amountToDeposit == 0) revert JackpotErrors.DepositAmountZero();

        usdc.safeTransferFrom(msg.sender, address(this), _amountToDeposit);

        jackpotLPManager.processDeposit(currentDrawingId, msg.sender, _amountToDeposit);
    }

    /**
     * @notice Initiates withdrawal of LP shares from the liquidity pool
     * @dev Converts consolidated shares to pending withdrawal, which can be finalized after the current drawing.
     *      Automatically consolidates any previous deposits before processing the withdrawal.
     * @param _amountToWithdrawInShares Amount of LP shares to withdraw (in PRECISE_UNIT scale)
     * @custom:requirements
     * - Drawing must not be locked (jackpotLock == false)
     * - Withdrawal amount must be greater than 0
     * - Caller must have sufficient consolidated shares
     * - Emergency mode must not be active
     * @custom:emits LpWithdrawInitiated (emitted by LPManager)
     * @custom:effects
     * - Consolidates previous deposits if from earlier drawings
     * - Moves shares from consolidated to pending withdrawal
     * - Updates drawing state pending withdrawals
     * - Shares cannot be finalized until drawing completes
     * @custom:security
     * - Share balance validation
     * - Prevents withdrawals during locked drawings
     */
    function initiateWithdraw(uint256 _amountToWithdrawInShares) external noEmergencyMode {
        if (drawingState[currentDrawingId].jackpotLock) revert JackpotErrors.JackpotLocked();
        if (_amountToWithdrawInShares == 0) revert JackpotErrors.WithdrawAmountZero();

        jackpotLPManager.processInitiateWithdraw(currentDrawingId, msg.sender, _amountToWithdrawInShares);
    }

    /**
     * @notice Finalizes LP withdrawals and transfers USDC to the caller
     * @dev Converts pending withdrawal shares to USDC using the appropriate drawing accumulator,
     *      combines with any claimable withdrawals, and transfers the total amount.
     * @custom:requirements
     * - Caller must have pending withdrawals or claimable withdrawals
     * - Pending withdrawals must be from completed drawings
     * - Emergency mode must not be active
     * @custom:emits LpWithdrawFinalized (emitted by LPManager)
     * @custom:effects
     * - Transfers USDC equivalent of withdrawn shares to caller
     * - Resets claimable withdrawals and pending withdrawals to zero
     * - Uses historical accumulator values for accurate share pricing
     * @custom:security
     * - Reentrancy protection via nonReentrant modifier
     * - Share-to-USDC conversion using verified accumulator values
     * - Safe USDC transfers
     */
    function finalizeWithdraw() external nonReentrant noEmergencyMode {
        uint256 withdrawableAmount = jackpotLPManager.processFinalizeWithdraw(currentDrawingId, msg.sender);
        usdc.safeTransfer(msg.sender, withdrawableAmount);
    }

    /**
     * @notice Emergency withdrawal function for LPs when system is stuck, intended to be used if the jackpot cannot be transitioned to a new drawing
     * @dev Allows LPs to withdraw all their deposits when emergency mode is enabled.
     *      This bypasses normal withdrawal restrictions for system recovery.
     * @custom:requirements
     * - Emergency mode must be enabled
     * - Caller must have LP positions to withdraw
     * @custom:emits EmergencyWithdrawLP (emitted by LPManager)
     * @custom:effects
     * - Withdraws all LP positions for the caller
     * - Transfers USDC equivalent to caller
     * - Removes all LP tracking for the caller
     * @custom:security
     * - Only available in emergency mode
     * - Complete LP position removal
     */
    function emergencyWithdrawLP() external nonReentrant onlyEmergencyMode {
        uint256 withdrawableAmount = jackpotLPManager.emergencyWithdrawLP(currentDrawingId, msg.sender);
        usdc.safeTransfer(msg.sender, withdrawableAmount);
    }

    /**
     * @notice Allows ticket holders to receive refunds for their tickets from the current drawing during emergency mode
     * @dev Refunds tickets from the active drawing by burning the NFTs and transferring USDC back to holders.
     *      Refund amount is the full ticket price for tickets without referrals, or ticket price minus 
     *      referral fees for tickets purchased with referral schemes.
     *      Only tickets from the current drawing are eligible since past drawings have concluded normally.
     * @param _userTicketIds Array of ticket NFT IDs to refund from the current drawing
     * @custom:requirements
     * - Emergency mode must be active
     * - Caller must own all specified ticket NFTs
     * - Tickets must be from the current drawing only (not past drawings)
     * - At least one valid ticket must be provided
     * @custom:emits TicketRefunded for each successfully refunded ticket
     * @custom:effects
     * - Burns all specified ticket NFTs permanently
     * - Transfers total refund amount in USDC to caller
     * - Removes tickets from current drawing circulation
     * @custom:security
     * - Reentrancy protection via nonReentrant modifier
     * - Emergency mode enforcement prevents normal operation interference
     * - Ownership validation prevents unauthorized refunds
     * - Current drawing restriction ensures only active tickets are refunded
     * - Batch processing reduces gas costs for multiple ticket refunds
     */
    function emergencyRefundTickets(uint256[] memory _userTicketIds) external nonReentrant onlyEmergencyMode {
        if (_userTicketIds.length == 0) revert JackpotErrors.NoTicketsProvided();
        uint256 totalRefundAmount = 0;
        for (uint256 i = 0; i < _userTicketIds.length; i++) {
            uint256 ticketId = _userTicketIds[i];
            IJackpotTicketNFT.TrackedTicket memory ticketInfo = jackpotNFT.getTicketInfo(ticketId);

            if (ticketInfo.drawingId != currentDrawingId) revert JackpotErrors.TicketNotEligibleForRefund();
            if (IERC721(address(jackpotNFT)).ownerOf(ticketId) != msg.sender) revert JackpotErrors.NotTicketOwner();

            uint256 refundAmount = ticketInfo.referralScheme == bytes32(0) ? drawingState[ticketInfo.drawingId].ticketPrice 
                : drawingState[ticketInfo.drawingId].ticketPrice * (PRECISE_UNIT - referralFee) / PRECISE_UNIT;

            totalRefundAmount += refundAmount;

            jackpotNFT.burnTicket(ticketId);
            emit TicketRefunded(ticketId);
        }

        usdc.safeTransfer(msg.sender, totalRefundAmount);
    }

    /**
     * @notice Allows referrers to claim accumulated referral fees
     * @dev Transfers all pending referral fees to the caller and resets their balance to zero.
     *      Referral fees accumulate from ticket purchases and winning claims.
     * @custom:requirements
     * - Caller must have referral fees to claim (balance > 0)
     * @custom:emits ReferralFeesClaimed
     * @custom:effects
     * - Transfers USDC referral fees to caller
     * - Resets caller's referral fee balance to zero
     * @custom:security
     * - Reentrancy protection via nonReentrant modifier
     * - Balance validation before transfer
     * - Safe USDC transfers
     */
    function claimReferralFees() external nonReentrant {
        if (referralFees[msg.sender] == 0) revert JackpotErrors.NoReferralFeesToClaim();
        uint256 transferAmount = referralFees[msg.sender];
        delete referralFees[msg.sender];
        usdc.safeTransfer(msg.sender, transferAmount);
        emit ReferralFeesClaimed(msg.sender, transferAmount);
    }

    /**
     * @notice Executes the jackpot drawing by requesting randomness from the entropy provider
     * @dev Locks the current drawing, validates timing, and initiates the random number generation process.
     *      The drawing can only be executed after the drawing time has passed.
     * @custom:requirements
     * - Drawing time must have passed (strictly after scheduled drawingTime)
     * - Drawing must not already be locked
     * - Sufficient ETH must be provided for entropy provider fees
     * @custom:emits JackpotRunRequested
     * @custom:effects
     * - Locks the current drawing (prevents further ticket purchases)
     * - Requests scaled randomness from entropy provider
     * - Refunds excess ETH to caller
     * - Sets up callback for drawing completion
     * @custom:security
     * - Permissionless (any address may call)
     * - Timing validation prevents premature execution
     * - Entropy fee validation and refund mechanism
     * - Single execution per drawing via lock mechanism
     */
    function runJackpot() external payable nonReentrant noEmergencyMode {
        DrawingState storage currentDrawingState = drawingState[currentDrawingId];
        if (currentDrawingState.jackpotLock) revert JackpotErrors.JackpotLocked();
        if (currentDrawingState.drawingTime >= block.timestamp) revert JackpotErrors.DrawingNotDue();

        _lockJackpot();

        uint32 entropyGasLimit = _calculateEntropyGasLimit(currentDrawingState.bonusballMax);
        uint256 fee = entropy.getFee(entropyGasLimit);
        if (msg.value < fee) revert JackpotErrors.InsufficientEntropyFee();
        if (msg.value > fee) {
            (bool success, bytes memory returndata) = payable(msg.sender).call{value: msg.value - fee}("");
            if (!success) {
                if (returndata.length > 0) {
                    assembly {
                        revert(add(returndata, 32), mload(returndata))
                    }
                } else {
                    revert("Refund transfer failed");
                }
            }
        }

        IScaledEntropyProvider.SetRequest[] memory setRequests = new IScaledEntropyProvider.SetRequest[](2);
        setRequests[0] = IScaledEntropyProvider.SetRequest({
            samples: NORMAL_BALL_COUNT,
            minRange: uint256(1),
            maxRange: uint256(currentDrawingState.ballMax),
            withReplacement: false
        });
        setRequests[1] = IScaledEntropyProvider.SetRequest({
            samples: 1,
            minRange: uint256(1),
            maxRange: uint256(currentDrawingState.bonusballMax),
            withReplacement: false
        });

        entropy.requestAndCallbackScaledRandomness{value: fee}(
            entropyGasLimit,
            setRequests,
            this.scaledEntropyCallback.selector,
            bytes("")
        );

        emit JackpotRunRequested(currentDrawingId, entropyGasLimit, fee);
    }

    /**
     * @notice Callback function called by the entropy provider with random numbers
     * @dev Processes the random numbers to determine the total amount of winning payouts across all prize tiers. 
     *      Updates LP with new pool size by netting revenue from tickets sold during drawing, winning payouts, and deposits/withdrawals. 
     *      Finally calculates the params for the next drawing (with a particular focus on calculating the new bonusball).
     *      Note: the first callback parameter is the provider sequence ID and is ignored by Jackpot.
     * @param _randomNumbers Array of arrays containing random numbers (5 normal balls + 1 bonusball)
     * @custom:requirements
     * - Only entropy provider can call (verified via onlyEntropy modifier)
     * - Drawing must be locked (indicates runJackpot was called)
     * - Random numbers must be properly formatted
     * @custom:emits JackpotSettled, NewDrawingInitialized
     * @custom:effects
     * - Sets winning ticket numbers for the current drawing
     * - Calculates and stores tier payouts based on matches
     * - Updates drawing accumulator and LP values
     * - Creates next drawing state with new parameters
     * - Increments current drawing ID
     * - Unlocks the system for new ticket purchases
     * @custom:security
     * - Strict access control via entropy provider verification
     * - Single execution per drawing (prevented by lock state)
     * - Comprehensive state updates in single transaction
     */
    function scaledEntropyCallback(
        bytes32,
        uint256[][] memory _randomNumbers,
        bytes memory
    )
        external
        nonReentrant
        onlyEntropy
    {
        // Note: in previous versions we had an entropyCallbackLock to prevent double calls, but this is no longer needed
        // since we use the currentDrawingState - if the scaledEntropyCallback call succeeded then currentDrawingState
        // would be the next drawing and jackpotLock would be false
        DrawingState storage currentDrawingState = drawingState[currentDrawingId];
        if (!currentDrawingState.jackpotLock) revert JackpotErrors.JackpotNotLocked();

        (uint256 winningNumbers, uint256 drawingUserWinnings) = _calculateDrawingUserWinnings(currentDrawingState, _randomNumbers);
        currentDrawingState.winningTicket = winningNumbers;

        uint256 protocolFeeAmount = _transferProtocolFee(currentDrawingState.lpEarnings, drawingUserWinnings);

        (
            uint256 newLpValue,
            uint256 newAccumulatorValue
        ) = jackpotLPManager.processDrawingSettlement(
            currentDrawingId,
            currentDrawingState.lpEarnings,
            drawingUserWinnings,
            protocolFeeAmount
        );

        _setNewDrawingState(newLpValue, currentDrawingState.drawingTime + drawingDurationInSeconds);
        emit JackpotSettled(currentDrawingId - 1, currentDrawingState.globalTicketsBought, drawingUserWinnings, _randomNumbers[1][0].toUint8(), winningNumbers, newAccumulatorValue);
    }

    // =============================================================
    //                       INITIALIZATION FUNCTIONS
    // =============================================================
    
    /**
     * @notice Initializes the contract with external dependencies
     * @dev This is the first step in the contract initialization process. Must be called before initializeLPDeposits().
     *      Sets up references to all external contracts required for operation.
     * @param _usdc The USDC token contract address
     * @param _jackpotLPManager The LP manager contract address
     * @param _jackpotNFT The ticket NFT contract address
     * @param _entropy The scaled entropy provider contract address
     * @param _payoutCalculator The payout calculator contract address
     * @custom:requirements
     * - Contract must not already be initialized
     * - All contract addresses must not be zero address
     * - Only owner can call
     * @custom:effects
     * - Sets all external contract references
     * - Marks contract as initialized
     * @custom:security
     * - Owner-only access
     * - Zero address validation for all contracts
     * - Single initialization enforcement
     */
    function initialize(
        IERC20 _usdc,
        IJackpotLPManager _jackpotLPManager,
        IJackpotTicketNFT _jackpotNFT, 
        IScaledEntropyProvider _entropy, 
        IPayoutCalculator _payoutCalculator
    )
        external
        onlyOwner
    {
        if (initialized) revert JackpotErrors.ContractAlreadyInitialized();
        if (_entropy == IScaledEntropyProvider(address(0))) revert JackpotErrors.ZeroAddress();
        if (_usdc == IERC20(address(0))) revert JackpotErrors.ZeroAddress();
        if (_payoutCalculator == IPayoutCalculator(address(0))) revert JackpotErrors.ZeroAddress();
        if (_jackpotNFT == IJackpotTicketNFT(address(0))) revert JackpotErrors.ZeroAddress();
        if (_jackpotLPManager == IJackpotLPManager(address(0))) revert JackpotErrors.ZeroAddress();

        usdc = _usdc;
        jackpotLPManager = _jackpotLPManager;
        jackpotNFT = _jackpotNFT;
        entropy = _entropy;
        payoutCalculator = _payoutCalculator;
        initialized = true;
    }

    /**
     * @notice Initializes LP deposit functionality by setting pool cap and initial accumulator
     * @dev This is the second step in initialization. Calculates the maximum LP pool capacity based on
     *      the normal ball range and sets the initial drawing accumulator to PRECISE_UNIT.
     * @param _governancePoolCap The maximum LP pool capacity as defined by governance
     * @custom:requirements
     * - Contract must be initialized first
     * - LP deposits must not already be initialized (drawingAccumulator[0] must be 0)
     * - Governance pool cap must not be 0
     * - Only owner can call
     * @custom:effects
     * - Sets lpPoolCap based on calculated maximum allowable tickets and governance pool cap
     * - Sets drawingAccumulator[0] to PRECISE_UNIT
     * - Enables LP deposit functionality
     * @custom:security
     * - Requires prior initialization
     * - Single execution enforcement
     * - Mathematical validation of pool cap calculation
     */
    function initializeLPDeposits(uint256 _governancePoolCap) external onlyOwner {
        if (!initialized) revert JackpotErrors.ContractNotInitialized();
        if (jackpotLPManager.getDrawingAccumulator(0) != 0) revert JackpotErrors.LPDepositsAlreadyInitialized();
        if (_governancePoolCap == 0) revert JackpotErrors.InvalidGovernancePoolCap();

        // Set governance pool cap first so that it is available for lpPoolCap calculation
        governancePoolCap = _governancePoolCap;

        // Set lpPoolCap and drawingAccumulator to be able to start taking deposits
        jackpotLPManager.initializeLP();
        jackpotLPManager.setLPPoolCap(currentDrawingId, _calculateLpPoolCap(normalBallMax));
    }

    /**
     * @notice Finalizes jackpot initialization and starts the first drawing
     * @dev This is the final step in initialization. Enables ticket purchases and creates the first drawing state
     *      using any pending deposits as the initial LP value.
     * @param _initialDrawingTime Unix timestamp for when the first drawing should occur
     * @custom:requirements
     * - LP deposits must be initialized first
     * - Jackpot must not already be initialized (currentDrawingId must be 0)
     * - Only owner can call
     * @custom:effects
     * - Sets allowTicketPurchases to true
     * - Creates first drawing state with initial LP value
     * - Increments currentDrawingId to 1
     * - Starts the jackpot operation
     * @custom:security
     * - Sequential initialization requirement
     * - Single execution enforcement
     * - Proper state transition validation
     */
    function initializeJackpot(uint256 _initialDrawingTime) external onlyOwner {
        if (jackpotLPManager.getDrawingAccumulator(0) == 0) revert JackpotErrors.LPDepositsNotInitialized();
        if (currentDrawingId != 0) revert JackpotErrors.JackpotAlreadyInitialized();
        if (jackpotLPManager.getLPDrawingState(0).pendingDeposits == 0) revert JackpotErrors.NoLPDeposits();

        allowTicketPurchases = true;
        (
            uint256 newLpValue,
        ) = jackpotLPManager.processDrawingSettlement(0, 0, 0, 0);    // Drawing 0 and no winnings or lp earnings
        _setNewDrawingState(newLpValue, _initialDrawingTime);
    }

    // =============================================================
    //                       ADMIN FUNCTIONS
    // =============================================================

    /**
     * @notice Updates the maximum normal ball number and recalculates LP pool cap
     * @dev Changes the range of normal ball numbers, affecting ticket combinations and pool sizing.
     *      If the new lpPoolCap is less than current lpPoolTotal it will revert.
     * @param _normalBallMax New maximum value for normal balls (1 to this value)
     * @custom:requirements
     * - Only owner can call
     * - Value is automatically constrained by uint8 type (max 255)
     * - New pool cap must not be less than current total LP pool value
     * @custom:effects
     * - Updates normalBallMax state variable
     * - Recalculates and updates LP pool cap for current drawing
     * - Affects future drawing configurations
     * @custom:security
     * - Owner-only access
     * - Automatic LP pool cap recalculation maintains system integrity
     * - Pool size validation prevents system inconsistency
     */
    function setNormalBallMax(uint8 _normalBallMax) external onlyOwner {
        // Note: we do not need to check if _normalBallMax is greater than 255 because it is enforced by uint8 type
        uint8 oldNormalBallMax = normalBallMax;
        jackpotLPManager.setLPPoolCap(currentDrawingId, _calculateLpPoolCap(_normalBallMax));
        normalBallMax = _normalBallMax;
        
        emit NormalBallMaxUpdated(currentDrawingId, oldNormalBallMax, _normalBallMax);
    }

    /**
     * @notice Updates the protocol fee threshold
     * @dev Sets minimum LP profit required before protocol fees are collected
     * @param _protocolFeeThreshold New threshold amount in USDC wei
     * @custom:requirements
     * - Only owner can call
     * @custom:effects
     * - Updates protocolFeeThreshold state variable
     * - Affects future protocol fee calculations
     */
    function setProtocolFeeThreshold(uint256 _protocolFeeThreshold) external onlyOwner {
        uint256 oldProtocolFeeThreshold = protocolFeeThreshold;
        protocolFeeThreshold = _protocolFeeThreshold;
        
        emit ProtocolFeeThresholdUpdated(currentDrawingId, oldProtocolFeeThreshold, _protocolFeeThreshold);
    }

    /**
     * @notice Updates the protocol fee percentage
     * @dev Sets the fraction of excess LP earnings taken as protocol fee
     * @param _protocolFee New protocol fee in PRECISE_UNIT scale (e.g., 0.01e18 = 1%)
     * @custom:requirements
     * - Only owner can call
     * - Should be validated to be reasonable (< PRECISE_UNIT)
     * @custom:effects
     * - Updates protocolFee state variable
     * - Affects future protocol fee calculations
     */
    function setProtocolFee(uint256 _protocolFee) external onlyOwner {
        if (_protocolFee > MAX_PROTOCOL_FEE) revert JackpotErrors.InvalidProtocolFee();
        uint256 oldProtocolFee = protocolFee;
        protocolFee = _protocolFee;
        
        emit ProtocolFeeUpdated(currentDrawingId, oldProtocolFee, _protocolFee);
    }

    /**
     * @notice Updates the governance pool cap
     * @dev Sets the maximum LP pool capacity as defined by governance
     * @param _governancePoolCap New governance pool cap in USDC wei
     * @custom:requirements
     * - Only owner can call
     * - Governance pool cap must not be 0
     * @custom:effects
     * - Updates governancePoolCap state variable
     * - Affects future lpPoolCap calculations
     * @custom:security
     * - Zero value validation prevents invalid governance pool cap
     * - Automatic lpPoolCap recalculation maintains system integrity
     * - Pool size validation prevents system inconsistency
     */
    function setGovernancePoolCap(uint256 _governancePoolCap) external onlyOwner {
        if (_governancePoolCap == 0) revert JackpotErrors.InvalidGovernancePoolCap();

        uint256 oldGovernancePoolCap = governancePoolCap;
        governancePoolCap = _governancePoolCap;
        jackpotLPManager.setLPPoolCap(currentDrawingId, _calculateLpPoolCap(normalBallMax));
        
        emit GovernancePoolCapUpdated(currentDrawingId, oldGovernancePoolCap, _governancePoolCap);
    }

    /**
     * @notice Updates the time between jackpot drawings
     * @dev Changes how frequently drawings occur
     * @param _drawingDurationInSeconds New duration between drawings in seconds
     * @custom:requirements
     * - Only owner can call
     * - Duration must be greater than 0
     * @custom:effects
     * - Updates drawingDurationInSeconds state variable
     * - Affects future drawing scheduling
     * @custom:security
     * - Zero duration validation prevents system lock
     */
    function setDrawingDurationInSeconds(uint256 _drawingDurationInSeconds) external onlyOwner {
        if (_drawingDurationInSeconds == 0) revert JackpotErrors.InvalidDrawingDuration();
        uint256 oldDrawingDurationInSeconds = drawingDurationInSeconds;
        drawingDurationInSeconds = _drawingDurationInSeconds;
        
        emit DrawingDurationUpdated(currentDrawingId, oldDrawingDurationInSeconds, _drawingDurationInSeconds);
    }

    /**
     * @notice Updates the minimum bonusball range
     * @dev Sets the minimum number of bonusball options for drawings
     * @param _bonusballMin New minimum bonusball value
     * @custom:requirements
     * - Only owner can call
     * - Value must be greater than 0
     * @custom:effects
     * - Updates bonusballMin state variable
     * - Affects future drawing configurations
     * @custom:security
     * - Zero value validation prevents invalid bonusball ranges
     */
    function setBonusballMin(uint8 _bonusballMin) external onlyOwner {
        if (_bonusballMin == 0) revert JackpotErrors.InvalidBonusballMin();
        uint8 oldBonusballMin = bonusballMin;
        bonusballMin = _bonusballMin;
        
        emit BonusballMinUpdated(currentDrawingId, oldBonusballMin, _bonusballMin);
    }

    /**
     * @notice Updates the LP edge target percentage
     * @dev Sets the target profit margin for liquidity providers
     * @param _lpEdgeTarget New LP edge target in PRECISE_UNIT scale
     * @custom:requirements
     * - Only owner can call
     * - Value must be greater than 0 and less than PRECISE_UNIT
     * @custom:effects
     * - Updates lpEdgeTarget state variable
     * - Recalculates and updates LP pool cap
     * - Affects prize pool sizing and LP profitability
     * @custom:security
     * - Range validation ensures valid percentage values
     */
    function setLpEdgeTarget(uint256 _lpEdgeTarget) external onlyOwner {
        if (_lpEdgeTarget == 0 || _lpEdgeTarget >= PRECISE_UNIT) revert JackpotErrors.InvalidLpEdgeTarget();
        uint256 oldLpEdgeTarget = lpEdgeTarget;
        lpEdgeTarget = _lpEdgeTarget;

        jackpotLPManager.setLPPoolCap(currentDrawingId, _calculateLpPoolCap(normalBallMax));
        
        emit LpEdgeTargetUpdated(currentDrawingId, oldLpEdgeTarget, _lpEdgeTarget);
    }

    /**
     * @notice Updates the reserve ratio for LP pool
     * @dev Sets the fraction of LP pool held in reserve (not available as prize pool)
     * @param _reserveRatio New reserve ratio in PRECISE_UNIT scale
     * @custom:requirements
     * - Only owner can call
     * - Value must be less than PRECISE_UNIT
     * @custom:effects
     * - Updates reserveRatio state variable
     * - Recalculates and updates LP pool cap
     * - Affects prize pool sizing relative to LP pool
     * @custom:security
     * - Upper bound validation prevents invalid ratios
     */
    function setReserveRatio(uint256 _reserveRatio) external onlyOwner {
        if (_reserveRatio >= PRECISE_UNIT) revert JackpotErrors.InvalidReserveRatio();
        uint256 oldReserveRatio = reserveRatio;
        reserveRatio = _reserveRatio;

        jackpotLPManager.setLPPoolCap(currentDrawingId, _calculateLpPoolCap(normalBallMax));
        
        emit ReserveRatioUpdated(currentDrawingId, oldReserveRatio, _reserveRatio);
    }

    /**
     * @notice Updates the referral fee percentage
     * @dev Sets the fraction of ticket price paid as referral fees
     * @param _referralFee New referral fee in PRECISE_UNIT scale
     * @custom:requirements
     * - Only owner can call
     * - Value must not exceed PRECISE_UNIT (100%)
     * @custom:effects
     * - Updates referralFee state variable
     * - Affects referral fee calculations on ticket purchases
     * @custom:security
     * - Upper bound validation prevents excessive fees
     */
    function setReferralFee(uint256 _referralFee) external onlyOwner {
        if (_referralFee > PRECISE_UNIT) revert JackpotErrors.InvalidReferralFee();
        uint256 oldReferralFee = referralFee;
        referralFee = _referralFee;
        
        emit ReferralFeeUpdated(currentDrawingId, oldReferralFee, _referralFee);
    }

    /**
     * @notice Updates the referral win share percentage
     * @dev Sets the fraction of winnings shared with referrers
     * @param _referralWinShare New referral win share in PRECISE_UNIT scale
     * @custom:requirements
     * - Only owner can call
     * - Value must not exceed PRECISE_UNIT (100%)
     * @custom:effects
     * - Updates referralWinShare state variable
     * - Affects referral fee calculations on winnings claims
     * @custom:security
     * - Upper bound validation prevents excessive sharing
     */
    function setReferralWinShare(uint256 _referralWinShare) external onlyOwner {
        if (_referralWinShare > PRECISE_UNIT) revert JackpotErrors.InvalidReferralWinShare();
        uint256 oldReferralWinShare = referralWinShare;
        referralWinShare = _referralWinShare;
        
        emit ReferralWinShareUpdated(currentDrawingId, oldReferralWinShare, _referralWinShare);
    }

    /**
     * @notice Updates the protocol fee recipient address
     * @dev Changes where protocol fees are sent
     * @param _protocolFeeAddress New protocol fee recipient address
     * @custom:requirements
     * - Only owner can call
     * - Address must not be zero address
     * @custom:effects
     * - Updates protocolFeeAddress state variable
     * - Affects future protocol fee transfers
     * @custom:security
     * - Zero address validation prevents fee loss
     */
    function setProtocolFeeAddress(address _protocolFeeAddress) external onlyOwner {
        if (_protocolFeeAddress == address(0)) revert JackpotErrors.ZeroAddress();
        address oldProtocolFeeAddress = protocolFeeAddress;
        protocolFeeAddress = _protocolFeeAddress;
        
        emit ProtocolFeeAddressUpdated(currentDrawingId, oldProtocolFeeAddress, _protocolFeeAddress);
    }

    /**
     * @notice Updates the ticket price and recalculates LP pool cap
     * @dev Changes the cost per ticket and updates pool sizing accordingly.
     *      If the new lpPoolCap is less than current lpPoolTotal it will revert.
     * @param _ticketPrice New ticket price in USDC wei
     * @custom:requirements
     * - Only owner can call
     * - Price must be greater than 0
     * - New pool cap must not be less than current total LP pool value
     * @custom:effects
     * - Updates ticketPrice state variable
     * - Recalculates and updates LP pool cap for current drawing
     * - Affects future ticket purchases and pool sizing
     * @custom:security
     * - Zero price validation prevents free tickets
     * - Automatic pool cap recalculation maintains system integrity
     * - Pool size validation prevents system inconsistency
     */
    function setTicketPrice(uint256 _ticketPrice) external onlyOwner {
        if (_ticketPrice == 0) revert JackpotErrors.InvalidTicketPrice();
        uint256 oldTicketPrice = ticketPrice;
        ticketPrice = _ticketPrice;
        jackpotLPManager.setLPPoolCap(currentDrawingId, _calculateLpPoolCap(normalBallMax));
        
        emit TicketPriceUpdated(currentDrawingId, oldTicketPrice, _ticketPrice);
    }

    /**
     * @notice Updates the maximum number of referrers per ticket
     * @dev Sets the limit on referral chain length
     * @param _maxReferrers New maximum number of referrers
     * @custom:requirements
     * - Only owner can call
     * - Value must be greater than 0
     * @custom:effects
     * - Updates maxReferrers state variable
     * - Affects referral validation in ticket purchases
     * @custom:security
     * - Zero value validation ensures referrals remain functional
     */
    function setMaxReferrers(uint256 _maxReferrers) external onlyOwner {
        if (_maxReferrers == 0) revert JackpotErrors.InvalidMaxReferrers();
        uint256 oldMaxReferrers = maxReferrers;
        maxReferrers = _maxReferrers;
        
        emit MaxReferrersUpdated(currentDrawingId, oldMaxReferrers, _maxReferrers);
    }

    /**
     * @notice Updates the payout calculator contract
     * @dev Changes the contract responsible for calculating prize payouts
     * @param _payoutCalculator New payout calculator contract address
     * @custom:requirements
     * - Only owner can call
     * - Address must not be zero address
     * @custom:effects
     * - Updates payoutCalculator state variable
     * - Affects future payout calculations
     * @custom:security
     * - Zero address validation prevents calculation failures
     * - Contract interface compatibility assumed
     */
    function setPayoutCalculator(IPayoutCalculator _payoutCalculator) external onlyOwner {
        if (_payoutCalculator == IPayoutCalculator(address(0))) revert JackpotErrors.ZeroAddress();
        IPayoutCalculator oldPayoutCalculator = payoutCalculator;
        payoutCalculator = _payoutCalculator;

        emit PayoutCalculatorUpdated(currentDrawingId, address(oldPayoutCalculator), address(_payoutCalculator));
    }

    /**
     * @notice Updates the entropy provider contract
     * @dev Changes the contract responsible for providing randomness
     * @param _entropy New entropy provider contract address
     * @custom:requirements
     * - Only owner can call
     * - Address must not be zero address
     * @custom:effects
     * - Updates entropy state variable
     * - Affects future drawing executions
     * @custom:security
     * - Zero address validation prevents drawing failures
     * - Contract interface compatibility assumed
     */
    function setEntropy(IScaledEntropyProvider _entropy) external onlyOwner {
        if (_entropy == IScaledEntropyProvider(address(0))) revert JackpotErrors.ZeroAddress();
        IScaledEntropyProvider oldEntropy = entropy;
        entropy = _entropy;

        emit EntropyUpdated(currentDrawingId, address(oldEntropy), address(_entropy));
    }

    /**
     * @notice Updates the entropy gas limit for callbacks
     * @dev Sets the gas limit used when requesting entropy from the provider
     * @param _entropyBaseGasLimit New gas limit for entropy callbacks
     * @custom:requirements
     * - Only owner can call
     * @custom:effects
     * - Updates entropyBaseGasLimit state variable
     * - Affects future entropy requests
     */
    function setEntropyBaseGasLimit(uint32 _entropyBaseGasLimit) external onlyOwner {
        uint32 oldEntropyBaseGasLimit = entropyBaseGasLimit;
        entropyBaseGasLimit = _entropyBaseGasLimit;
        
        emit EntropyBaseGasLimitUpdated(currentDrawingId, oldEntropyBaseGasLimit, _entropyBaseGasLimit);
    }

    /**
     * @notice Updates the entropy variable gas limit - the amount of gas that needs to be added per
     * bonusball used (drawingState.bonusballMax)
     * @dev Sets the variable portion of the gas limit used when requesting entropy from the provider
     * @param _entropyVariableGasLimit New variable gas limit for entropy callbacks
     * @custom:requirements
     * - Only owner can call
     * @custom:effects
     * - Updates entropyVariableGasLimit state variable
     * - Affects future entropy requests
     */
    function setEntropyVariableGasLimit(uint32 _entropyVariableGasLimit) external onlyOwner {
        uint32 oldEntropyVariableGasLimit = entropyVariableGasLimit;
        entropyVariableGasLimit = _entropyVariableGasLimit;
        
        emit EntropyVariableGasLimitUpdated(currentDrawingId, oldEntropyVariableGasLimit, _entropyVariableGasLimit);
    }

    /**
     * @notice Enables emergency mode to halt normal operations
     * @dev Activates emergency mode for system recovery or maintenance
     * @custom:requirements
     * - Only owner can call
     * - Emergency mode must not already be enabled
     * @custom:effects
     * - Sets emergencyMode to true
     * - Disables normal ticket purchases and LP operations
     * - Enables emergency withdrawal functions
     * @custom:security
     * - Owner-only access
     * - Single activation enforcement
     */
    function enableEmergencyMode() external onlyOwner {
        if (emergencyMode) revert JackpotErrors.EmergencyModeAlreadyEnabled();
        emergencyMode = true;
        emit EmergencyModeEnabled(currentDrawingId);
    }

    /**
     * @notice Disables emergency mode to resume normal operations
     * @dev Deactivates emergency mode to restore normal functionality
     * @custom:requirements
     * - Only owner can call
     * - Emergency mode must be currently enabled
     * @custom:effects
     * - Sets emergencyMode to false
     * - Re-enables normal operations
     * - Disables emergency withdrawal functions
     * @custom:security
     * - Owner-only access
     * - State validation prevents invalid transitions
     */
    function disableEmergencyMode() external onlyOwner {
        if (!emergencyMode) revert JackpotErrors.EmergencyModeAlreadyDisabled();
        emergencyMode = false;
        emit EmergencyModeDisabled(currentDrawingId);
    }

    /**
     * @notice Manually locks the current drawing
     * @dev Prevents ticket purchases and LP operations for the current drawing
     * @custom:requirements
     * - Only owner can call
     * - Drawing must not already be locked
     * @custom:effects
     * - Sets jackpotLock to true for current drawing
     * - Prevents ticket purchases and deposits
     * @custom:security
     * - Owner-only access for emergency control
     * - State validation prevents double-locking
     */
    function lockJackpot() external onlyOwner {
        if (drawingState[currentDrawingId].jackpotLock) revert JackpotErrors.JackpotLocked();
        _lockJackpot();
    }

    /**
     * @notice Manually unlocks the current drawing
     * @dev Re-enables ticket purchases and LP operations for the current drawing
     * @custom:requirements
     * - Only owner can call
     * - Drawing must be currently locked
     * @custom:effects
     * - Sets jackpotLock to false for current drawing
     * - Re-enables ticket purchases and deposits
     * @custom:security
     * - Owner-only access for emergency control
     * - State validation prevents invalid unlocking
     */
    function unlockJackpot() external onlyOwner {
        if (!drawingState[currentDrawingId].jackpotLock) revert JackpotErrors.JackpotNotLocked();
        _unlockJackpot();
    }

    /**
     * @notice Enables ticket purchases globally
     * @dev Allows users to purchase tickets (used during initialization or after maintenance)
     * @custom:requirements
     * - Only owner can call
     * - Ticket purchases must not already be enabled
     * @custom:effects
     * - Sets allowTicketPurchases to true
     * - Enables global ticket purchasing functionality
     * @custom:security
     * - Owner-only access
     * - State validation prevents redundant enabling
     */
    function enableTicketPurchases() external onlyOwner {
        if (allowTicketPurchases) revert JackpotErrors.TicketPurchasesAlreadyEnabled();
        allowTicketPurchases = true;
        
        emit TicketPurchasesEnabled(currentDrawingId);
    }

    /**
     * @notice Disables ticket purchases globally
     * @dev Prevents users from purchasing tickets (used for maintenance or shutdown)
     * @custom:requirements
     * - Only owner can call
     * - Ticket purchases must be currently enabled
     * @custom:effects
     * - Sets allowTicketPurchases to false
     * - Disables global ticket purchasing functionality
     * @custom:security
     * - Owner-only access
     * - State validation prevents redundant disabling
     */
    function disableTicketPurchases() external onlyOwner {
        if (!allowTicketPurchases) revert JackpotErrors.TicketPurchasesAlreadyDisabled();
        allowTicketPurchases = false;
        
        emit TicketPurchasesDisabled(currentDrawingId);
    }

    // =============================================================
    //                      VIEW/PURE FUNCTIONS
    // =============================================================

    /**
     * @notice Returns the complete drawing state for a given drawing ID
     * @dev Provides read-only access to drawing configuration and results
     * @param _drawingId The drawing ID to query
     * @return DrawingState struct containing all drawing information
     */
    function getDrawingState(uint256 _drawingId) external view returns (DrawingState memory) {
        return drawingState[_drawingId];
    }

    /**
     * @notice Returns the referral scheme details for a given scheme ID
     * @dev Provides access to referrer addresses and split percentages
     * @param _referralSchemeId The keccak256 hash of referrers and splits
     * @return ReferralScheme struct containing referrer information
     */
    function getReferralScheme(bytes32 _referralSchemeId) external view returns (ReferralScheme memory) {
        return referralSchemes[_referralSchemeId];
    }

    /**
     * @notice Checks if specific tickets have been purchased in a drawing
     * @dev Useful for preventing duplicate purchases or checking availability
     * @param _drawingId The drawing to check
     * @param _tickets Array of tickets to check
     * @return Array of booleans indicating if each ticket was purchased
     */
    function checkIfTicketsBought(uint256 _drawingId, Ticket[] memory _tickets) external view returns (bool[] memory) {
        bool[] memory isBought = new bool[](_tickets.length);
        for (uint256 i = 0; i < _tickets.length; i++) {
            isBought[i] = TicketComboTracker.isDuplicate(drawingEntries[_drawingId], _tickets[i].normals, _tickets[i].bonusball);
        }
        return isBought;
    }

    /**
     * @notice Returns the count of tickets matching a subset of numbers
     * @dev Useful for analyzing ticket distribution and calculating probabilities.
     *      toNormalsBitVector can take any sized array of normals as long as ≤ 5 (the amount of normals in the ticket).
     *      This function can be used to see how many of a specific subset of normals have been bought.
     * @param _drawingId The drawing to check
     * @param _normals Array of normal numbers to match (can be partial)
     * @param _bonusball Bonusball number to match
     * @return ComboCount struct containing match statistics
     */
    function getSubsetCount(uint256 _drawingId, uint8[] memory _normals, uint8 _bonusball) external view returns (TicketComboTracker.ComboCount memory) {
        uint256 subset = TicketComboTracker.toNormalsBitVector(_normals, drawingState[_drawingId].ballMax);
        return drawingEntries[_drawingId].comboCounts[ _bonusball][subset];
    }

    
    /**
     * @notice Unpacks a packed ticket into normal numbers and bonusball
     * @dev Decodes the bit-packed ticket format used by the protocol:
     *      - Normal numbers are stored in bit positions [1..ballMax]
     *      - Bonusball is stored at position (ballMax + bonusball)
     *      Uses TicketComboTracker.unpackTicket to reconstruct the ticket.
     * @param _drawingId The drawing context providing `ballMax` for unpacking
     * @param _packedTicket The packed ticket bit vector to decode
     * @return normals Array of normal numbers in ascending order
     * @return bonusball The bonusball value for the ticket
     * @custom:effects
     * - Read-only operation with no state changes
     * @custom:security
     * - Assumes `_packedTicket` follows the protocol's packing scheme
     */
    function getUnpackedTicket(uint256 _drawingId, uint256 _packedTicket) external view returns (uint8[] memory normals, uint8 bonusball) {
        return TicketComboTracker.unpackTicket(_packedTicket, drawingState[_drawingId].ballMax);
    }

    /**
     * @notice Returns tier IDs for a list of ticket NFTs based on winning numbers
     * @dev For each ticket ID, fetches its packed ticket and drawing, then computes the tier:
     *      tierId = 2 * (matchedNormals) + (bonusballMatch ? 1 : 0), in the range [0..11].
     *      Relies on the drawing's stored `winningTicket` and `ballMax`.
     * @param _ticketIds Array of ticket NFT IDs to evaluate
     * @return tierIds Array of tier IDs aligned with `_ticketIds`
     * @custom:effects
     * - Read-only operation with no state changes
     * @custom:security
     * - Assumes tickets exist and their drawings have valid `winningTicket` values
     */
    function getTicketTierIds(uint256[] memory _ticketIds) external view returns (uint256[] memory tierIds) {
        tierIds = new uint256[](_ticketIds.length);
        for (uint256 i = 0; i < _ticketIds.length; i++) {
            IJackpotTicketNFT.TrackedTicket memory ticket = jackpotNFT.getTicketInfo(_ticketIds[i]);
            DrawingState memory ticketDrawingState = drawingState[ticket.drawingId];
            tierIds[i] = _calculateTicketTierId(ticket.packedTicket, ticketDrawingState.winningTicket, ticketDrawingState.ballMax);
        }
        return tierIds;
    }

    /**
     * @notice Returns the current ETH fee (in wei) required for the entropy callback
     * @dev Computes the callback gas limit for the current drawing as:
     *      `entropyGasLimit = entropyBaseGasLimit + entropyVariableGasLimit * bonusballMax`,
     *      then queries the entropy provider for the corresponding fee. The returned value is
     *      denominated in wei and reflects the provider’s current pricing. Callers may wish to
     *      include a small buffer when funding `runJackpot` to account for fee changes between calls.
     * @return fee The ETH amount in wei required by the entropy provider for the callback
     * @custom:effects
     * - Read-only operation with no state changes
     * @custom:security
     * - Depends on provider pricing; fee may change over time or with gas limit parameter updates
     */
    function getEntropyCallbackFee() external view returns (uint256 fee) {
        uint32 entropyGasLimit = _calculateEntropyGasLimit(drawingState[currentDrawingId].bonusballMax);
        return entropy.getFee(entropyGasLimit);
    }

    // =============================================================
    //                      INTERNAL FUNCTIONS
    // =============================================================
    function _calculateLpPoolCap(uint256 _normalBallMax) internal view returns (uint256) {
        // We use MAX_BIT_VECTOR_SIZE because that's the max number that can be packed in a uint256 bit vector
        uint256 maxAllowableTickets = Combinations.choose(_normalBallMax, NORMAL_BALL_COUNT) * (MAX_BIT_VECTOR_SIZE - _normalBallMax);
        uint256 maxPrizePool = maxAllowableTickets * ticketPrice * (PRECISE_UNIT - lpEdgeTarget) / PRECISE_UNIT;

        // We need to make sure that the lpPoolCap is not greater than the governance pool cap
        return Math.min(maxPrizePool * PRECISE_UNIT / (PRECISE_UNIT - reserveRatio), governancePoolCap);
    }

    function _setNewDrawingState(uint256 _newLpValue, uint256 _nextDrawingTime) internal {
        currentDrawingId++;

        jackpotLPManager.initializeDrawingLP(currentDrawingId, _newLpValue);

        DrawingState storage newDrawingState = drawingState[currentDrawingId];
        uint256 newPrizePool = _newLpValue * (PRECISE_UNIT - reserveRatio) / PRECISE_UNIT;
        newDrawingState.prizePool = newPrizePool;
        newDrawingState.ticketPrice = ticketPrice;
        newDrawingState.edgePerTicket = lpEdgeTarget * ticketPrice / PRECISE_UNIT;
        newDrawingState.globalTicketsBought = 0;
        newDrawingState.lpEarnings = 0;
        newDrawingState.ballMax = normalBallMax;
        newDrawingState.drawingTime = _nextDrawingTime;
        newDrawingState.jackpotLock = false;

        uint256 combosPerBonusball = Combinations.choose(normalBallMax, NORMAL_BALL_COUNT);
        uint256 minNumberTickets = newPrizePool * PRECISE_UNIT / ((PRECISE_UNIT - lpEdgeTarget) * ticketPrice);
        uint8 newBonusball = uint8(Math.max(bonusballMin, Math.ceilDiv(minNumberTickets, combosPerBonusball)));
        newDrawingState.bonusballMax = newBonusball;
        
        TicketComboTracker.init(drawingEntries[currentDrawingId], normalBallMax, newBonusball, NORMAL_BALL_COUNT);

        // Note: we want to set this when we set the payouts because it is important to the product that the targeted
        // guaranteed minimums are calculated net of the correct referral win share. It is grouped here with `setDrawingTierInfo`
        // to emphasize this relationship.
        newDrawingState.referralWinShare = referralWinShare;
        payoutCalculator.setDrawingTierInfo(currentDrawingId);

        emit NewDrawingInitialized(
            currentDrawingId,
            _newLpValue,
            newPrizePool,
            ticketPrice,
            normalBallMax,
            newBonusball,
            referralWinShare,
            _nextDrawingTime
        );
    }

    function _validateBuyTicketInputs(
        Ticket[] memory _tickets,
        address _recipient,
        address[] memory _referrers,
        uint256[] memory _referralSplit
    ) internal view {
        if (drawingState[currentDrawingId].jackpotLock) revert JackpotErrors.JackpotLocked();
        if (drawingState[currentDrawingId].prizePool == 0) revert JackpotErrors.NoPrizePool();
        if (!allowTicketPurchases) revert JackpotErrors.TicketPurchasesDisabled();
        if (_tickets.length == 0) revert JackpotErrors.InvalidTicketCount();
        if (_recipient == address(0)) revert JackpotErrors.InvalidRecipient();
        if (_referrers.length != _referralSplit.length) revert JackpotErrors.ReferralSplitLengthMismatch();
        if (_referrers.length > maxReferrers) revert JackpotErrors.TooManyReferrers();
    }

    function _validateAndTrackReferrals(
        address[] memory _referrers,
        uint256[] memory _referralSplit,
        uint256 _ticketsValue
    )
        internal
        returns (uint256 referralFeeTotal, bytes32 referralSchemeId)
    {
        if (_referrers.length > 0) {
            // Calculate total amount of referral fees for the order
            referralFeeTotal = _ticketsValue * referralFee / PRECISE_UNIT;
            // Calculate the referral scheme id for the order
            referralSchemeId = keccak256(abi.encode(_referrers, _referralSplit));

            uint256 referralSplitSum = 0;
            for (uint256 i = 0; i < _referrers.length; i++) {
                if (_referrers[i] == address(0)) revert JackpotErrors.ZeroAddress();
                if (_referralSplit[i] == 0) revert JackpotErrors.InvalidReferralSplitBps();
                // Add the referral fee to the referrer's balance
                uint256 referrerFee = referralFeeTotal * _referralSplit[i] / PRECISE_UNIT;
                referralFees[_referrers[i]] += referrerFee;
                referralSplitSum += _referralSplit[i];
                emit ReferralFeeCollected(_referrers[i], referrerFee);
            }
            if (referralSplitSum != PRECISE_UNIT) revert JackpotErrors.ReferralSplitSumInvalid();
            // If the referral scheme id is not already in the mapping, add it
            if (referralSchemes[referralSchemeId].referrers.length == 0) {
                referralSchemes[referralSchemeId] = ReferralScheme({
                    referrers: _referrers,
                    referralSplit: _referralSplit
                });

                emit ReferralSchemeAdded(referralSchemeId, _referrers, _referralSplit);
            }
        }
    }

    function _validateAndStoreTickets(
        DrawingState storage _currentDrawingState,
        Ticket[] memory _tickets,
        address _recipient,
        bytes32 _referralSchemeId,
        bytes32 _source
    )
        internal
        returns (uint256[] memory ticketIds)
    {
        TicketComboTracker.Tracker storage currentDrawingEntries = drawingEntries[currentDrawingId];

        ticketIds = new uint256[](_tickets.length);
        for (uint256 i = 0; i < _tickets.length; i++) {
            Ticket memory ticket = _tickets[i];
            if (ticket.normals.length != NORMAL_BALL_COUNT) revert JackpotErrors.InvalidNormalsCount();
            if (ticket.bonusball > _currentDrawingState.bonusballMax || ticket.bonusball == 0) revert JackpotErrors.InvalidBonusball();

            // Validation to make sure that normals are in range and no duplicates take place here
            (uint256 packedTicket, bool isDup) = TicketComboTracker.insert(currentDrawingEntries, ticket.normals, _tickets[i].bonusball);
            uint256 ticketId = uint256(keccak256(abi.encode(currentDrawingId, _currentDrawingState.globalTicketsBought + i + 1, packedTicket)));
            ticketIds[i] = ticketId;

            jackpotNFT.mintTicket(_recipient, ticketId, currentDrawingId, packedTicket, _referralSchemeId);

            if (isDup) {
                // We need to add to the prize pool because it is like an additional ticket is being minted. In order to guarantee the LP
                // edge we need to make sure that only (1-lpEdgeTarget) * ticketPrice is added to the prize pool.
                _currentDrawingState.prizePool += _currentDrawingState.ticketPrice - _currentDrawingState.edgePerTicket;
            }

            emit TicketPurchased(
                _recipient,
                currentDrawingId,
                _source,
                ticketId,
                ticket.normals,
                ticket.bonusball,
                _referralSchemeId
            );
        }
    }

    function _calculateDrawingUserWinnings(
        DrawingState storage _currentDrawingState,
        uint256[][] memory _unPackedWinningNumbers
    )
        internal
        returns(uint256 winningNumbers, uint256 drawingUserWinnings)
    {
        // Note that the total amount of winning tickets for a given tier is the sum of result and dupResult
        (
            uint256 winningTicket,
            uint256[] memory uniqueResult,
            uint256[] memory dupResult
        ) = TicketComboTracker.countTierMatchesWithBonusball(drawingEntries[currentDrawingId],
            _unPackedWinningNumbers[0].toUint8Array(),      // normal balls
            _unPackedWinningNumbers[1][0].toUint8()         // bonusball
        );

        winningNumbers = winningTicket;

        drawingUserWinnings = payoutCalculator.calculateAndStoreDrawingUserWinnings(
            currentDrawingId,
            _currentDrawingState.prizePool,
            _currentDrawingState.ballMax,
            _currentDrawingState.bonusballMax,
            uniqueResult,
            dupResult
        );

        emit WinnersCalculated(
            currentDrawingId,
            _unPackedWinningNumbers[0],
            _unPackedWinningNumbers[1][0],
            uniqueResult,
            dupResult
        );
    }

    function _calculateTicketTierId(uint256 _ticketNumbers, uint256 _winningNumbers, uint256 _normalBallMax) internal pure returns (uint256) {
        uint256 matches = 0;
        
        // Count matching normal numbers by checking overlapping bits
        uint256 matchingBits = _ticketNumbers & _winningNumbers;
        
        // Count the number of set bits (matches)
        matches = LibBit.popCount(matchingBits);
        
        // Extract bonusball from both ticket and winning numbers
        // Bonusball is stored in the highest bits after the normal numbers
        uint256 ticketBonusball = _ticketNumbers >> (_normalBallMax + 1);
        uint256 winningBonusball = _winningNumbers >> (_normalBallMax + 1);

        uint256 bonusballMatch = (ticketBonusball == winningBonusball) ? 1 : 0;

        // We count all matches including the bonusball so if the bonusball is a match we need to subtract it from matches
        return 2 * (matches - bonusballMatch) + bonusballMatch;
    }

    function _payReferrersWinnings(
        bytes32 _referralSchemeId,
        uint256 _winningAmount,
        uint256 _referralWinShare
    )
        internal
        returns (uint256)
    {
        uint256 referrerShare = _winningAmount * _referralWinShare / PRECISE_UNIT;
        // If referrer scheme is empty then the referrer share goes to LPs so we just add the amount to lpEarnings
        // in order to make sure our system accounts for it
        if (_referralSchemeId == bytes32(0)) {
            drawingState[currentDrawingId].lpEarnings += referrerShare;
            emit LpEarningsUpdated(currentDrawingId, referrerShare);
            return referrerShare;
        }

        ReferralScheme memory referralScheme = referralSchemes[_referralSchemeId];

        for (uint256 i = 0; i < referralScheme.referrers.length; i++) {
            // This is safe because we validate the referrers in _validateAndTrackReferrals and this function is only called after that
            address referrer = referralScheme.referrers[i];
            uint256 referrerFee = referrerShare * referralScheme.referralSplit[i] / PRECISE_UNIT;
            referralFees[referrer] += referrerFee;
            emit ReferralFeeCollected(referrer, referrerFee);
        }
        return referrerShare;
    }

    function _transferProtocolFee(
        uint256 _lpEarnings,
        uint256 _drawingUserWinnings
    )
        internal
        returns (uint256 protocolFeeAmount)
    {
        if (_lpEarnings > _drawingUserWinnings && _lpEarnings - _drawingUserWinnings > protocolFeeThreshold) {
            protocolFeeAmount = (_lpEarnings - _drawingUserWinnings - protocolFeeThreshold) * protocolFee / PRECISE_UNIT;
            usdc.safeTransfer(protocolFeeAddress, protocolFeeAmount);
        }

        emit ProtocolFeeCollected(currentDrawingId, protocolFeeAmount);
    }

    function _calculateEntropyGasLimit(uint8 _bonusballMax) internal view returns (uint32) {
        return entropyBaseGasLimit + entropyVariableGasLimit * uint32(_bonusballMax);
    }

    function _lockJackpot() internal {
        drawingState[currentDrawingId].jackpotLock = true;
        emit JackpotLocked(currentDrawingId);
    }

    function _unlockJackpot() internal {
        drawingState[currentDrawingId].jackpotLock = false;
        emit JackpotUnlocked(currentDrawingId);
    }
}

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
//SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.28;

interface IJackpotTicketNFT {

    struct TrackedTicket {
        uint256 drawingId;
        uint256 packedTicket;
        bytes32 referralScheme;
    }

    struct ExtendedTrackedTicket {
        uint256 ticketId;
        TrackedTicket ticket;
        uint8[] normals;
        uint8 bonusball;
    }

    function mintTicket(
        address recipient, 
        uint256 ticketId, 
        uint256 drawingId,
        uint256 packedTicket, 
        bytes32 referralScheme
    ) external;
    
    function burnTicket(uint256 ticketId) external;
    function getTicketInfo(uint256 ticketId) external view returns (TrackedTicket memory);
    function getUserTickets(address user, uint256 drawingId) external view returns (ExtendedTrackedTicket[] memory);
}
// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v5.3.0) (utils/ReentrancyGuardTransient.sol)

pragma solidity ^0.8.24;

import {TransientSlot} from "./TransientSlot.sol";

/**
 * @dev Variant of {ReentrancyGuard} that uses transient storage.
 *
 * NOTE: This variant only works on networks where EIP-1153 is available.
 *
 * _Available since v5.1._
 */
abstract contract ReentrancyGuardTransient {
    using TransientSlot for *;

    // keccak256(abi.encode(uint256(keccak256("openzeppelin.storage.ReentrancyGuard")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant REENTRANCY_GUARD_STORAGE =
        0x9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f00;

    /**
     * @dev Unauthorized reentrant call.
     */
    error ReentrancyGuardReentrantCall();

    /**
     * @dev Prevents a contract from calling itself, directly or indirectly.
     * Calling a `nonReentrant` function from another `nonReentrant`
     * function is not supported. It is possible to prevent this from happening
     * by making the `nonReentrant` function external, and making it call a
     * `private` function that does the actual work.
     */
    modifier nonReentrant() {
        _nonReentrantBefore();
        _;
        _nonReentrantAfter();
    }

    function _nonReentrantBefore() private {
        // On the first call to nonReentrant, REENTRANCY_GUARD_STORAGE.asBoolean().tload() will be false
        if (_reentrancyGuardEntered()) {
            revert ReentrancyGuardReentrantCall();
        }

        // Any calls to nonReentrant after this point will fail
        REENTRANCY_GUARD_STORAGE.asBoolean().tstore(true);
    }

    function _nonReentrantAfter() private {
        REENTRANCY_GUARD_STORAGE.asBoolean().tstore(false);
    }

    /**
     * @dev Returns true if the reentrancy guard is currently set to "entered", which indicates there is a
     * `nonReentrant` function in the call stack.
     */
    function _reentrancyGuardEntered() internal view returns (bool) {
        return REENTRANCY_GUARD_STORAGE.asBoolean().tload();
    }
}

// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v5.1.0) (access/Ownable2Step.sol)

pragma solidity ^0.8.20;

import {Ownable} from "./Ownable.sol";

/**
 * @dev Contract module which provides access control mechanism, where
 * there is an account (an owner) that can be granted exclusive access to
 * specific functions.
 *
 * This extension of the {Ownable} contract includes a two-step mechanism to transfer
 * ownership, where the new owner must call {acceptOwnership} in order to replace the
 * old one. This can help prevent common mistakes, such as transfers of ownership to
 * incorrect accounts, or to contracts that are unable to interact with the
 * permission system.
 *
 * The initial owner is specified at deployment time in the constructor for `Ownable`. This
 * can later be changed with {transferOwnership} and {acceptOwnership}.
 *
 * This module is used through inheritance. It will make available all functions
 * from parent (Ownable).
 */
abstract contract Ownable2Step is Ownable {
    address private _pendingOwner;

    event OwnershipTransferStarted(address indexed previousOwner, address indexed newOwner);

    /**
     * @dev Returns the address of the pending owner.
     */
    function pendingOwner() public view virtual returns (address) {
        return _pendingOwner;
    }

    /**
     * @dev Starts the ownership transfer of the contract to a new account. Replaces the pending transfer if there is one.
     * Can only be called by the current owner.
     *
     * Setting `newOwner` to the zero address is allowed; this can be used to cancel an initiated ownership transfer.
     */
    function transferOwnership(address newOwner) public virtual override onlyOwner {
        _pendingOwner = newOwner;
        emit OwnershipTransferStarted(owner(), newOwner);
    }

    /**
     * @dev Transfers ownership of the contract to a new account (`newOwner`) and deletes any pending owner.
     * Internal function without access restriction.
     */
    function _transferOwnership(address newOwner) internal virtual override {
        delete _pendingOwner;
        super._transferOwnership(newOwner);
    }

    /**
     * @dev The new owner accepts the ownership transfer.
     */
    function acceptOwnership() public virtual {
        address sender = _msgSender();
        if (pendingOwner() != sender) {
            revert OwnableUnauthorizedAccount(sender);
        }
        _transferOwnership(sender);
    }
}

//SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.28;

interface IJackpot {

    struct Ticket {
        uint8[] normals;
        uint8 bonusball;
    }

    function buyTickets(
        Ticket[] memory _tickets,
        address _recipient,
        address[] memory _referrers,
        uint256[] memory _referralSplitBps,
        bytes32 _source
    )
        external
        returns (uint256[] memory ticketIds);

    function claimWinnings(
        uint256[] memory _userTicketIds
    )
        external;

    function ticketPrice() external view returns (uint256);
    function currentDrawingId() external view returns (uint256);
    function getUnpackedTicket(uint256 _drawingId, uint256 _packedTicket) external view returns (uint8[] memory, uint8);
}
//SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.28;

interface IJackpotLPManager {

    struct LPDrawingState {
        uint256 lpPoolTotal;
        uint256 pendingDeposits;
        uint256 pendingWithdrawals;
    }

    function processDeposit(uint256 _drawingId, address _lpAddress, uint256 _amount) external;

    function processInitiateWithdraw(uint256 _drawingId, address _lpAddress, uint256 _amountToWithdrawInShares) external;

    function processFinalizeWithdraw(uint256 _drawingId, address _lpAddress) external returns (uint256 withdrawableAmount);

    function processDrawingSettlement(
        uint256 _drawingId,
        uint256 _lpEarnings,
        uint256 _userWinnings,
        uint256 _protocolFeeAmount
    ) external returns (uint256 newLPValue, uint256 newAccumulator);

    function emergencyWithdrawLP(uint256 _drawingId, address _user) external returns (uint256 withdrawableAmount);

    function initializeDrawingLP(uint256 _drawingId, uint256 _initialLPValue) external;

    function setLPPoolCap(uint256 _drawingId, uint256 _lpPoolCap) external;

    function initializeLP() external;

    function getDrawingAccumulator(uint256 _drawingId) external view returns (uint256);
    function getLPDrawingState(uint256 _drawingId) external view returns (LPDrawingState memory);
}
//SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.28;

interface IScaledEntropyProvider {
    struct SetRequest {
        uint8 samples;
        uint256 minRange;
        uint256 maxRange;
        bool withReplacement;
    }
    function requestAndCallbackScaledRandomness(
        uint32 _gasLimit,
        SetRequest[] memory _requests,
        bytes4 _selector,
        bytes memory _context
    )
        external
        payable
        returns (uint64 requestId);
    function getFee(uint32 _gasLimit) external view returns (uint256);
}
//SPDX-License-Identifier: UNLICENSED

/*
Copyright (C) 2025 Coordination Inc.
Use of this software is govered by the Business Source License included in the LICENSE.TXT file and at www.mariadb.com/bsl11.

Change Date: 2029-12-01

On the date above, in accordance with the Business Source License, use of this software will be governed by the open source license specified in the LICENSE.TXT file.
*/

pragma solidity ^0.8.28;

interface IPayoutCalculator {
    function calculateAndStoreDrawingUserWinnings(
        uint256 _drawingId,
        uint256 _prizePool,
        uint8 _ballMax,
        uint8 _bonusballMax,
        uint256[] memory _result,
        uint256[] memory _dupResult
    ) external returns (uint256);

    function setDrawingTierInfo(uint256 _drawingId) external;

    function getTierPayout(uint256 _drawingId, uint256 _tierId) external view returns (uint256);
}
//SPDX-License-Identifier: UNLICENSED

/*
Copyright (C) 2025 Coordination Inc.
All rights reserved.

This software is proprietary and confidential. Unauthorized copying,
distribution, or use is strictly prohibited and may result in legal action.

For licensing inquiries: legal@coordinationlabs.com
*/

pragma solidity ^0.8.28;

/**
 * @title UintCasts
 * @notice Minimal helpers for safely downcasting uint256 values to uint8.
 * @dev Reverts with Uint8OutOfBounds() if a value exceeds uint8's max (255).
 */
library UintCasts {
    /// @notice Raised when a value cannot be represented as uint8 (value > 255)
    error Uint8OutOfBounds();

    /**
     * @notice Safely cast a uint256 to uint8.
     * @param _value The value to cast.
     * @return out The value as uint8 (reverts if out of range).
     */
    function toUint8(uint256 _value) internal pure returns (uint8) {
        if (_value > type(uint8).max) revert Uint8OutOfBounds();
        return uint8(_value);
    }

    /**
     * @notice Safely cast an array of uint256 to uint8[] element-wise.
     * @param _values The array of values to cast.
     * @return out The cast array (reverts if any element is out of range).
     */
    function toUint8Array(uint256[] memory _values) internal pure returns (uint8[] memory) {
        uint256 len = _values.length;
        uint8[] memory out = new uint8[](len);
        for (uint256 i = 0; i < len; ) {
            out[i] = toUint8(_values[i]);
            unchecked { ++i; }
        }
        return out;
    }
}

//SPDX-License-Identifier: UNLICENSED

/*
Copyright (C) 2025 Coordination Inc.
All rights reserved.

This software is proprietary and confidential. Unauthorized copying,
distribution, or use is strictly prohibited and may result in legal action.

For licensing inquiries: legal@coordinationlabs.com
*/

pragma solidity ^0.8.28;

library JackpotErrors {
    // =============================================================
    //                            ERRORS
    // =============================================================

    error JackpotLocked();
    error DrawingNotDue();
    error InvalidRecipient();
    error InvalidTicketCount();
    error ReferralSplitLengthMismatch();
    error TooManyReferrers();
    error ReferralSplitSumInvalid();
    error InvalidBonusball();
    error TicketAlreadyMinted();
    error NoTicketsToClaim();
    error NotTicketOwner();
    error TicketFromFutureDrawing();
    error DepositAmountZero();
    error ExceedsPoolCap();
    error WithdrawAmountZero();
    error InsufficientShares();
    error NothingToWithdraw();
    error UnauthorizedEntropyCaller();
    error EntropyAlreadyCalled();
    error JackpotNotLocked();
    error ContractAlreadyInitialized();
    error ZeroAddress();
    error ContractNotInitialized();
    error LPDepositsAlreadyInitialized();
    error LPDepositsNotInitialized();
    error JackpotAlreadyInitialized();
    error TicketPurchasesDisabled();
    error InvalidTierWeights();
    error InvalidReferralSplitBps();
    error InvalidNormalsCount();
    error InsufficientEntropyFee();
    error NoReferralFeesToClaim();
    error NoPrizePool();
    error TicketPurchasesAlreadyEnabled();
    error TicketPurchasesAlreadyDisabled();
    error InvalidNormalBallMax();
    error InvalidDrawingDuration();
    error InvalidBonusballMin();
    error InvalidLpEdgeTarget();
    error InvalidReserveRatio();
    error InvalidReferralFee();
    error InvalidReferralWinShare();
    error InvalidTicketPrice();
    error InvalidMaxReferrers();
    error EmergencyEnabled();
    error EmergencyModeNotEngaged();
    error EmergencyModeAlreadyEnabled();
    error EmergencyModeAlreadyDisabled();
    error NoLPDeposits();
    error InvalidProtocolFee();
    error InvalidGovernancePoolCap();
    error TicketNotEligibleForRefund();
    error NoTicketsProvided();
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
//SPDX-License-Identifier: UNLICENSED

/*
Copyright (C) 2025 Coordination Inc.
All rights reserved.

This software is proprietary and confidential. Unauthorized copying,
distribution, or use is strictly prohibited and may result in legal action.

For licensing inquiries: legal@coordinationlabs.com
*/

pragma solidity ^0.8.28;

import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";
import { IEntropyConsumer } from "@pythnetwork/entropy-sdk-solidity/IEntropyConsumer.sol";
import { IEntropyV2 } from "@pythnetwork/entropy-sdk-solidity/IEntropyV2.sol";

import { FisherYatesRejection } from "./lib/FisherYatesWithRejection.sol";
import { IScaledEntropyProvider } from "./interfaces/IScaledEntropyProvider.sol";

/**
 * @title ScaledEntropyProvider
 * @notice Provides scaled random number generation using Pyth Network entropy with callback functionality
 * @dev Integrates with Pyth Network's entropy service to generate cryptographically secure random numbers:
 *      - Handles entropy requests with custom scaling and range parameters
 *      - Supports both sampling with and without replacement using Fisher-Yates algorithm
 *      - Provides callback mechanism for asynchronous random number delivery
 *      - Implements unbiased rejection sampling to prevent modulo bias
 *      - Manages fee payments to entropy providers
 *      - Stores pending requests and validates callback execution
 */
contract ScaledEntropyProvider is Ownable, IScaledEntropyProvider, IEntropyConsumer {
    // =============================================================
    //                           STRUCTS
    // =============================================================
    struct PendingRequest {
        address callback;
        bytes4 selector;
        bytes context;
        bytes32 userRandomNumber;
        SetRequest[] setRequests;
    }

    // =============================================================
    //                           EVENTS
    // =============================================================

    event ScaledRandomnessDelivered(uint64 indexed sequence, address indexed callback, uint256 samples);
    event EntropyFulfilled(uint64 indexed sequence, bytes32 randomNumber);

    // =============================================================
    //                           ERRORS
    // =============================================================
    error InvalidCallback();
    error CallbackFailed(bytes4 selector);
    error ZeroAddress();
    error InvalidSelector();
    error InvalidRequests();
    error InvalidRange();
    error InvalidSamples();
    error InsufficientFee();
    error UnknownSequence();

    // =============================================================
    //                       STATE VARIABLES
    // =============================================================

    IEntropyV2 private entropy;
    address private entropyProvider;
    mapping(uint64 => PendingRequest) private pending;

    // =============================================================
    //                         CONSTRUCTOR
    // =============================================================
    
    /**
     * @notice Initializes the ScaledEntropyProvider with Pyth Network entropy configuration
     * @dev Sets up connections to Pyth Network entropy contract and provider.
     *      Both addresses are validated and stored as immutable references.
     * @param _entropy Address of the Pyth Network entropy contract
     * @param _entropyProvider Address of the specific entropy provider to use
     * @custom:requirements
     * - Entropy contract address must not be zero
     * - Entropy provider address must not be zero
     * @custom:effects
     * - Sets immutable entropy contract reference
     * - Configures entropy provider for fee calculations
     * - Sets deployer as contract owner
     * @custom:security
     * - Address validation prevents zero address configuration
     * - Immutable references prevent unauthorized changes
     * - Owner-based access control for administrative functions
     */
    constructor(address _entropy, address _entropyProvider) Ownable(msg.sender) {
        if (_entropy == address(0)) revert ZeroAddress();
        if (_entropyProvider == address(0)) revert ZeroAddress();
        entropy = IEntropyV2(_entropy);
        entropyProvider = _entropyProvider;
    }

    // =============================================================
    //                      EXTERNAL FUNCTIONS
    // =============================================================

    /**
     * @notice Requests scaled random numbers from Pyth Network with callback delivery
     * @dev Submits entropy request to Pyth Network and stores callback details for async delivery.
     *      The callback will receive scaled random numbers according to the specified requests. Developer
     *      needs to ensure that the range is not too large to be able to build an array of the appropriate
     *      size in memory in order to avoid out of gas errors during Fisher-Yates sampling.
     *      IMPORTANT: The callback address is automatically set to msg.sender (the calling contract).
     * @param _gasLimit Gas limit for the entropy callback execution
     * @param _requests Array of SetRequest structs defining random number requirements
     * @param _selector Function selector for the callback method on the calling contract
     * @param _context Additional data to pass to the callback
     * @return sequence Unique identifier for tracking this entropy request
     * @custom:requirements
     * - Calling contract (msg.sender) must implement the callback function
     * - Provided fee (msg.value) must meet minimum requirements
     * - Function selector must not be zero
     * - All set requests must be valid (proper ranges and sample counts)
     * @custom:emits None (events emitted in callback)
     * @custom:effects
     * - Submits entropy request to Pyth Network
     * - Stores pending request details with msg.sender as callback address
     * - Transfers fee to entropy provider
     * @custom:security
     * - Callback address is restricted to msg.sender preventing unauthorized callbacks
     * - Fee validation ensures sufficient payment
     * - Request validation prevents invalid random number generation
     */
    function requestAndCallbackScaledRandomness(
        uint32 _gasLimit,
        SetRequest[] memory _requests,
        bytes4 _selector,
        bytes memory _context
    )
        external
        payable
        returns (uint64 sequence)
    {
        // We assume that the caller has already checked that the fee is sufficient
        if (msg.value < getFee(_gasLimit)) revert InsufficientFee();
        if (_selector == bytes4(0)) revert InvalidSelector();
        _validateRequests(_requests);

        sequence = entropy.requestV2{value: msg.value}(entropyProvider, _gasLimit);
        _storePendingRequest(sequence, _selector, _context, _requests);
    }

    /**
     * @notice Returns the fee required for an entropy request with specified gas limit
     * @dev Queries the Pyth Network entropy contract for current fee requirements.
     *      Fee covers entropy generation and callback execution costs.
     * @param _gasLimit Gas limit for the callback execution
     * @return Fee amount in wei required for the entropy request
     */
    function getFee(uint32 _gasLimit) public view returns (uint256) {
        return entropy.getFeeV2(entropyProvider, _gasLimit);
    }

    /**
     * @notice Returns the address of the Pyth Network entropy contract
     * @dev Provides access to the entropy contract address for integration purposes.
     * @return Address of the entropy contract
     */
    function getEntropyContract() external view returns (address) {
        return address(entropy);
    }

    /**
     * @notice Returns the address of the currently configured entropy provider
     * @dev Shows which entropy provider is being used for fee calculations and requests.
     * @return Address of the entropy provider
     */
    function getEntropyProvider() external view returns (address) {
        return entropyProvider;
    }

    /**
     * @notice Returns the details of a pending entropy request
     * @dev Retrieves stored request information for a specific sequence number.
     *      Useful for debugging and monitoring pending requests.
     * @param sequence Unique identifier of the entropy request
     * @return PendingRequest struct containing callback details and request parameters
     */
    function getPendingRequest(uint64 sequence) external view returns (PendingRequest memory) {
        return pending[sequence];
    }

    // =============================================================
    //                      ADMIN FUNCTIONS
    // =============================================================

    /**
     * @notice Updates the entropy provider address
     * @dev Changes which entropy provider is used for fee calculations and requests.
     *      Only affects future requests, not pending ones.
     * @param _entropyProvider New entropy provider address
     * @custom:requirements
     * - Only owner can call
     * - Provider address must not be zero
     * @custom:emits None
     * @custom:effects
     * - Updates entropy provider for future requests
     * - Changes fee calculations for new requests
     * @custom:security
     * - Owner-only access restriction
     * - Zero address validation
     */
    function setEntropyProvider(address _entropyProvider) external onlyOwner {
        if (_entropyProvider == address(0)) revert ZeroAddress();
        entropyProvider = _entropyProvider;
    }

    // =============================================================
    //                      INTERNAL FUNCTIONS
    // =============================================================

    /**
     * @notice Processes entropy callback from Pyth Network and delivers scaled random numbers
     * @dev Called by Pyth Network when entropy is available. Processes the raw entropy into scaled
     *      random numbers according to stored request parameters and delivers via callback.
     *      This is the core function that bridges Pyth entropy with application-specific randomness.
     * @param sequence Unique identifier for the entropy request
     * @param randomNumber Raw entropy value from Pyth Network (provider parameter ignored)
     * @custom:requirements
     * - Sequence must correspond to a valid pending request
     * - Callback execution must succeed
     * - Only called by Pyth Network entropy contract
     * @custom:emits EntropyFulfilled with sequence and raw random number
     * @custom:emits ScaledRandomnessDelivered with sequence, callback address, and sample count
     * @custom:effects
     * - Retrieves and deletes pending request data
     * - Generates scaled random numbers using Fisher-Yates or replacement sampling
     * - Executes callback with scaled results and original context
     * - Cleans up pending request storage
     * @custom:security
     * - Validates sequence corresponds to pending request
     * - Ensures callback execution succeeds before cleanup
     * - Uses unbiased sampling methods to prevent statistical attacks
     * - Immediate cleanup prevents replay attacks
     */
    function entropyCallback(uint64 sequence, address /*provider*/, bytes32 randomNumber) internal override {
        PendingRequest memory req = pending[sequence];
        if (req.callback == address(0)) revert UnknownSequence();
        
        delete pending[sequence];

        uint256[][] memory scaledRandomNumbers = _getScaledRandomness(randomNumber, req.setRequests);
        (bool success, ) = req.callback.call(abi.encodeWithSelector(req.selector, sequence, scaledRandomNumbers, req.context));
        if (!success) revert CallbackFailed(req.selector);

        emit EntropyFulfilled(sequence, randomNumber);
        emit ScaledRandomnessDelivered(sequence, req.callback, scaledRandomNumbers.length);
    }

    function _getScaledRandomness(
        bytes32 _randomNumber,
        SetRequest[] memory _setRequests
    )
        internal
        pure
        returns (uint256[][] memory requestsOutputs)
    {
        requestsOutputs = new uint256[][](_setRequests.length);
        
        for (uint256 i = 0; i < _setRequests.length; i++) {
            if (!_setRequests[i].withReplacement) {
                requestsOutputs[i] = FisherYatesRejection.draw(
                    _setRequests[i].minRange,
                    _setRequests[i].maxRange,
                    _setRequests[i].samples,
                    uint256(_randomNumber)
                );
            } else {
                requestsOutputs[i] = _drawWithReplacement(
                    _setRequests[i].minRange,
                    _setRequests[i].maxRange,
                    _setRequests[i].samples,
                    uint256(_randomNumber)
                );
            }
        }
    }

    function getEntropy() internal view override returns (address) {
        return address(entropy);
    }

    function _validateRequests(SetRequest[] memory _requests) internal pure {
        if (_requests.length == 0) revert InvalidRequests();
        for (uint256 i = 0; i < _requests.length; i++) {
            if (_requests[i].minRange > _requests[i].maxRange) revert InvalidRange();
            if (_requests[i].samples == 0) revert InvalidSamples();
        }
    }

    function _storePendingRequest(
        uint64 sequence,
        bytes4 _selector,
        bytes memory _context,
        SetRequest[] memory _setRequests
    ) internal {
        pending[sequence].callback = msg.sender;
        pending[sequence].selector = _selector;
        pending[sequence].context = _context;
        for (uint256 i = 0; i < _setRequests.length; i++) {
            pending[sequence].setRequests.push(_setRequests[i]);
        }
    }

    function _drawWithReplacement(
        uint256 _minRange,
        uint256 _maxRange,
        uint8 _samples,
        uint256 _randomNumber
    ) internal pure returns (uint256[] memory) {
        uint256[] memory result = new uint256[](_samples);
        uint256 range = _maxRange - _minRange + 1;
        uint256 nonce = 0;

        for (uint256 i = 0; i < _samples; i++) {
            uint256 rand;
            while (true) {
                rand = uint256(keccak256(abi.encode(_randomNumber, nonce)));
                uint256 limit = (type(uint256).max / range) * range;

                if (rand < limit) {
                    result[i] = uint256((rand % range) + _minRange); // [1..range]
                    break;
                }
                nonce++;
            }
            nonce++;
        }

        return result;
    }
}
//SPDX-License-Identifier: UNLICENSED

/*
Copyright (C) 2025 Coordination Inc.
All rights reserved.

This software is proprietary and confidential. Unauthorized copying,
distribution, or use is strictly prohibited and may result in legal action.

For licensing inquiries: legal@coordinationlabs.com
*/

pragma solidity ^0.8.28;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";

import { IJackpot } from "./interfaces/IJackpot.sol";
import { IJackpotLPManager } from "./interfaces/IJackpotLPManager.sol";
import { JackpotErrors } from "./lib/JackpotErrors.sol";

/**
 * @title JackpotLPManager
 * @notice Manages liquidity provider deposits, withdrawals, and share calculations using an accumulator-based pricing system
 * @dev Implements an LP management system with:
 *      - Accumulator-based share pricing for fair LP value distribution
 *      - Pending deposit/withdrawal system to handle drawing transitions
 *      - Consolidation mechanics for multi-drawing LP positions
 *      - Emergency withdrawal capabilities for system recovery
 */
contract JackpotLPManager is IJackpotLPManager, Ownable {

    // =============================================================
    //                          STRUCTS
    // =============================================================

    struct DepositInfo {
        uint256 amount;
        uint256 drawingId;
    }

    struct WithdrawalInfo {
        uint256 amountInShares;
        uint256 drawingId;
    }

    struct LP {
        uint256 consolidatedShares; // Note: this is the amount in shares, not usdc
        DepositInfo lastDeposit; // Note: this is the amount in usdc, not shares
        WithdrawalInfo pendingWithdrawal; // Note: this is the amount in shares, not usdc
        uint256 claimableWithdrawals; // Note: this is the amount in usdc, not shares
    }

    struct LPValueBreakdown {
        uint256 activeDeposits;
        uint256 pendingDeposits;
        uint256 pendingWithdrawals;
        uint256 claimableWithdrawals;
    }

    // =============================================================
    //                          EVENTS
    // =============================================================
    event LpDeposited(
        address indexed lpAddress,
        uint256 indexed currentDrawingId,
        uint256 amount,
        uint256 totalPendingDeposits
    );

    event LpWithdrawInitiated(
        address indexed lpAddress,
        uint256 indexed currentDrawingId,
        uint256 amount,
        uint256 totalPendingWithdrawals
    );

    event LpWithdrawFinalized(
        address indexed lpAddress,
        uint256 indexed currentDrawingId,
        uint256 amount
    );

    // =============================================================
    //                          ERRORS
    // =============================================================

    error UnauthorizedCaller();
    error ZeroAddress();
    error InvalidLPPoolCap();

    // =============================================================
    //                          CONSTANTS
    // =============================================================

    uint256 constant PRECISE_UNIT = 1e18;

    // =============================================================
    //                          STATE VARIABLES
    // =============================================================

    mapping(uint256 => LPDrawingState) internal lpDrawingState;
    mapping(address => LP) public lpInfo;
    mapping(uint256 => uint256) public drawingAccumulator;
    uint256 public lpPoolCap;

    IJackpot public jackpot;

    // =============================================================
    //                          MODIFIERS
    // =============================================================
    
    modifier onlyJackpot() {
        if (msg.sender != address(jackpot)) revert UnauthorizedCaller();
        _;
    }

    // =============================================================
    //                          CONSTRUCTOR
    // =============================================================

    /**
     * @notice Initializes the JackpotLPManager with the Jackpot contract reference
     * @dev Sets up the connection to the main Jackpot contract that will call LP management functions
     * @param _jackpot Address of the main Jackpot contract
     * @custom:requirements
     * - Jackpot address must not be zero address
     * @custom:effects
     * - Sets jackpot contract reference
     * - Sets deployer as owner
     * @custom:security
     * - Zero address validation for jackpot contract
     */
    constructor(IJackpot _jackpot) Ownable(msg.sender) {
        if (_jackpot == IJackpot(address(0))) revert ZeroAddress();
        jackpot = _jackpot;
    }

    // =============================================================
    //                          EXTERNAL FUNCTIONS
    // =============================================================

    /**
     * @notice Initializes the LP system with the first accumulator value
     * @dev Sets up the initial accumulator value for share calculations. Called during Jackpot initialization.
     * @custom:requirements
     * - Only Jackpot contract can call
     * - Must not already be initialized
     * @custom:effects
     * - Sets drawingAccumulator[0] to PRECISE_UNIT
     * - Enables LP deposit functionality
     * @custom:security
     * - Single initialization enforcement
     * - Access restricted to Jackpot contract
     */
    function initializeLP() external onlyJackpot() {
        drawingAccumulator[0] = PRECISE_UNIT;
    }

    /**
     * @notice Processes a new LP deposit for the current drawing
     * @dev Consolidates any previous deposits from earlier drawings. If deposit already made for current drawing then amount is added to current pending amount.
     *      Deposits are held as pending until the drawing settles, then converted to shares.
     * @param _drawingId Current drawing ID
     * @param _lpAddress Address making the deposit
     * @param _amount Amount of USDC being deposited
     * @custom:requirements
     * - Only Jackpot contract can call
     * - Deposit would not exceed LP pool cap
     * - Drawing accumulator must be initialized
     * @custom:emits LpDeposited
     * @custom:effects
     * - Consolidates previous deposits if any
     * - Creates new deposit record or adds to existing pending amount
     * - Updates total pending deposits for drawing
     * @custom:security
     * - Access restricted to Jackpot contract
     * - Pool cap validation prevents over-deposits
     */
    function processDeposit(uint256 _drawingId, address _lpAddress, uint256 _amount) external onlyJackpot() {
        // Note: this check also prevents users from depositing before initializeLPDeposits() is called since the pool cap will be 0
        // We will exclude pending withdrawals since the amount withdrawn is dependent on the post-drawing LP value. This makes this
        // check more conservative.
        uint256 totalPoolValue = lpDrawingState[_drawingId].lpPoolTotal + lpDrawingState[_drawingId].pendingDeposits;
        if (_amount + totalPoolValue > lpPoolCap) revert JackpotErrors.ExceedsPoolCap();

        LP storage lp = lpInfo[_lpAddress];

        _consolidateDeposits(lp, _drawingId);

        lp.lastDeposit.amount += _amount;
        lp.lastDeposit.drawingId = _drawingId;

        lpDrawingState[_drawingId].pendingDeposits += _amount;

        emit LpDeposited(_lpAddress, _drawingId, _amount, lpDrawingState[_drawingId].pendingDeposits);
    }

    /**
     * @notice Initiates withdrawal by converting consolidated shares to pending
     * @dev Moves shares from active to pending status, preventing further use until finalization.
     *      Automatically consolidates previous deposits before processing withdrawal.
     * @param _drawingId Current drawing ID
     * @param _lpAddress Address initiating withdrawal
     * @param _amountToWithdrawInShares Amount of shares to withdraw
     * @custom:requirements
     * - Only Jackpot contract can call
     * - LP must have sufficient consolidated shares
     * @custom:emits LpWithdrawInitiated
     * @custom:effects
     * - Consolidates previous deposits if any
     * - Moves shares to pending withdrawal status
     * - Updates drawing pending withdrawal total
     * - Reduces consolidated shares balance
     * @custom:security
     * - Access restricted to Jackpot contract
     * - Share balance validation
     */
    function processInitiateWithdraw(uint256 _drawingId, address _lpAddress, uint256 _amountToWithdrawInShares) external onlyJackpot() {
        LP storage lp = lpInfo[_lpAddress];

        _consolidateDeposits(lp, _drawingId);

        if (lp.consolidatedShares < _amountToWithdrawInShares) revert JackpotErrors.InsufficientShares();

        _consolidateWithdrawals(lp, _drawingId);

        lp.pendingWithdrawal.amountInShares += _amountToWithdrawInShares;
        lp.pendingWithdrawal.drawingId = _drawingId;

        lp.consolidatedShares -= _amountToWithdrawInShares;
        lpDrawingState[_drawingId].pendingWithdrawals += _amountToWithdrawInShares;

        emit LpWithdrawInitiated(_lpAddress, _drawingId, _amountToWithdrawInShares, lpDrawingState[_drawingId].pendingWithdrawals);
    }

    /**
     * @notice Finalizes pending withdrawals and returns USDC amount
     * @dev Converts pending shares to USDC using historical accumulator values.
     *      Combines pending and claimable withdrawals for total withdrawal amount.
     * @param _drawingId Current drawing ID
     * @param _lpAddress Address finalizing withdrawal
     * @return withdrawableAmount Total USDC amount to transfer to LP
     * @custom:requirements
     * - Only Jackpot contract can call
     * - LP must have withdrawable amounts (claimable > 0)
     * @custom:emits LpWithdrawFinalized
     * @custom:effects
     * - Converts pending shares to USDC using historical accumulators
     * - Resets claimable withdrawal balance to zero
     * - Updates total LP pool value
     * @custom:security
     * - Access restricted to Jackpot contract
     * - Accurate share-to-USDC conversion using verified accumulator values
     */
    function processFinalizeWithdraw(uint256 _drawingId, address _lpAddress) external onlyJackpot() returns (uint256 withdrawableAmount) {
        LP storage lp = lpInfo[_lpAddress];

        // Accrue pending withdrawals to claimable withdrawals
        _consolidateWithdrawals(lp, _drawingId);

        if (lp.claimableWithdrawals == 0) revert JackpotErrors.NothingToWithdraw();

        withdrawableAmount = lp.claimableWithdrawals;

        lp.claimableWithdrawals = 0;
        emit LpWithdrawFinalized(_lpAddress, _drawingId, withdrawableAmount);
    }

    /**
     * @notice Emergency withdrawal for all LP positions when system is stuck
     * @dev Used when emergency mode is enabled to allow complete LP recovery.
     *      Bypasses normal withdrawal restrictions and converts all LP positions to USDC.
     * @param _drawingId Current drawing ID
     * @param _user Address making emergency withdrawal
     * @return withdrawableAmount Total USDC amount to transfer
     * @custom:requirements
     * - Only Jackpot contract can call (which enforces emergency mode)
     * - LP must have positions to withdraw
     * @custom:emits LpWithdrawFinalized
     * @custom:effects
     * - Converts all deposit types (pending, consolidated, withdrawals) to USDC
     * - Removes all LP tracking for the user
     * - Updates global LP state consistently
     * - Handles special case for drawing 0
     * @custom:security
     * - Only available through Jackpot emergency mode
     * - Complete position removal prevents partial recovery
     * - Maintains global state consistency
     */
    function emergencyWithdrawLP(uint256 _drawingId, address _user) external onlyJackpot() returns (uint256 withdrawableAmount) {
        LP storage lp = lpInfo[_user];

        if (_drawingId == 0) {
            // Note we do not need to subtract from lpPoolTotal since same round pending deposits have not been added yet
            withdrawableAmount += lp.lastDeposit.amount;
            lpDrawingState[_drawingId].pendingDeposits -= lp.lastDeposit.amount;
            delete lp.lastDeposit;
            emit LpWithdrawFinalized(_user, _drawingId, withdrawableAmount);
            return withdrawableAmount;
        }

        // lastDeposit from previous rounds to consolidated shares
        _consolidateDeposits(lp, _drawingId);
        // Add any deposits from this round (since they are denominated in usdc we can add directly to the withdrawable amount)
        if (lp.lastDeposit.amount > 0) {
            // Note we do not need to subtract from lpPoolTotal since same round pending deposits have not been added yet
            withdrawableAmount += lp.lastDeposit.amount;
            lpDrawingState[_drawingId].pendingDeposits -= lp.lastDeposit.amount;
            delete lp.lastDeposit;
        }

        // consolidated shares to usdc
        uint256 sharesToUsdc = lp.consolidatedShares * drawingAccumulator[_drawingId - 1] / PRECISE_UNIT;
        withdrawableAmount += sharesToUsdc;
        // Keep global state consistent
        lpDrawingState[_drawingId].lpPoolTotal -= sharesToUsdc;
        lp.consolidatedShares = 0;

        // pending withdrawals from previous rounds to usdc
        _consolidateWithdrawals(lp, _drawingId);
        // Add convert pending withdrawals from this round to usdc
        if (lp.pendingWithdrawal.amountInShares > 0) {
            uint256 withdrawalToUsdc = lp.pendingWithdrawal.amountInShares * drawingAccumulator[lp.pendingWithdrawal.drawingId - 1] / PRECISE_UNIT;
            withdrawableAmount += withdrawalToUsdc;
            // Keep global state consistent
            lpDrawingState[_drawingId].pendingWithdrawals -= lp.pendingWithdrawal.amountInShares;
            lpDrawingState[_drawingId].lpPoolTotal -= withdrawalToUsdc;
            delete lp.pendingWithdrawal;
        }
        
        // Do not need to update any global state since claimableWithdrawals have already been counted as out of the lpPool
        withdrawableAmount += lp.claimableWithdrawals;
        lp.claimableWithdrawals = 0;

        emit LpWithdrawFinalized(_user, _drawingId, withdrawableAmount);
    }

    /**
     * @notice Processes drawing settlement and updates accumulator values
     * @dev Runs after a drawing is settled to roll LP value forward and set the next accumulator:
     *      - Compute post-draw LP value: lpPoolTotal + lpEarnings - userWinnings - protocolFeeAmount.
     *        This must not underflow; caller must ensure payouts and fees do not exceed available value plus earnings.
     *      - Compute `newAccumulator` for the settled drawing when `_drawingId > 0`:
     *          • If `currentLP.lpPoolTotal == 0`, set to `PRECISE_UNIT` to avoid division by zero.
     *          • Else `newAccumulator = drawingAccumulator[_drawingId - 1] * postDrawLpValue / currentLP.lpPoolTotal` (rounds down).
     *        For `_drawingId == 0`, the accumulator is expected to already be initialized to `PRECISE_UNIT` via `initializeLP()`.
     *      - Convert pending withdrawals to USDC using `newAccumulator` and finalize `newLPValue` as:
     *          `newLPValue = postDrawLpValue + pendingDeposits - (pendingWithdrawals * newAccumulator / 1e18)`.
     *      Division truncation favors safety (conservative values). Deposits are denominated in USDC; withdrawals are in shares.
     * @param _drawingId Drawing that was completed
     * @param _lpEarnings Total LP earnings from ticket sales for the drawing
     * @param _userWinnings Total winnings paid to users for the drawing
     * @param _protocolFeeAmount Protocol fees collected for the drawing
     * @return newLPValue New total LP pool value to seed the next drawing
     * @return newAccumulator Accumulator for the settled drawing (unchanged for drawing 0)
     * @custom:requirements
     * - Caller must be the Jackpot contract
     * - `lpDrawingState[_drawingId]` must be initialized
     * - For `_drawingId == 0`, `drawingAccumulator[0]` must already be initialized (via `initializeLP()`)
     * - Inputs must reflect finalized drawing economics; must not cause underflow in post-draw value calculation
     * @custom:effects
     * - Updates `drawingAccumulator[_drawingId]` when `_drawingId > 0`
     * - Returns computed `newLPValue` for initializing the next drawing’s LP state
     * - Does not mutate pending deposit/withdrawal tallies here (only reads them for valuation)
     * @custom:security
     * - Access restricted to Jackpot contract
     * - Division-by-zero avoided by `PRECISE_UNIT` fallback when `lpPoolTotal == 0`
     * - Integer division truncation is conservative in favor of LP solvency
     */
    function processDrawingSettlement(
        uint256 _drawingId,
        uint256 _lpEarnings,
        uint256 _userWinnings,
        uint256 _protocolFeeAmount
    ) external onlyJackpot() returns (uint256 newLPValue, uint256 newAccumulator) {
        LPDrawingState storage currentLP = lpDrawingState[_drawingId];
        uint256 postDrawLpValue = currentLP.lpPoolTotal + _lpEarnings - _userWinnings - _protocolFeeAmount;

        // Note: we don't need to update the accumulator for the first drawing (0) since it's already set to PRECISE_UNIT
        if (_drawingId > 0) {
            // When setting for drawingId we need to use the accumulator from the previous drawing. If LP was 0 in previous
            // drawing then we need to set the accumulator to PRECISE_UNIT to avoid division by zero.
            newAccumulator = currentLP.lpPoolTotal == 0 ? PRECISE_UNIT :
                (drawingAccumulator[_drawingId - 1] * postDrawLpValue) / currentLP.lpPoolTotal;
            drawingAccumulator[_drawingId] = newAccumulator;
        }
        
        // Convert pending withdrawals to usdc to calculate the new lp value
        uint256 withdrawalsInUSDC = currentLP.pendingWithdrawals * newAccumulator / PRECISE_UNIT;
        newLPValue = postDrawLpValue + currentLP.pendingDeposits - withdrawalsInUSDC;
    }

    /**
     * @notice Initializes LP state for a new drawing
     * @dev Sets up initial LP pool total and resets pending amounts for new drawing. Called when transitioning to new drawing.
     * @param _drawingId New drawing ID
     * @param _initialLPValue Total LP value for the drawing
     * @custom:requirements
     * - Only Jackpot contract can call
     * - Drawing must not already be initialized
     * @custom:effects
     * - Creates new LPDrawingState with initial LP pool value
     * - Sets LP pool total and zeros pending deposits/withdrawals
     * @custom:security
     * - Access restricted to Jackpot contract
     */
    function initializeDrawingLP(uint256 _drawingId, uint256 _initialLPValue) external onlyJackpot() {
        lpDrawingState[_drawingId] = LPDrawingState({
            lpPoolTotal: _initialLPValue,
            pendingDeposits: 0,
            pendingWithdrawals: 0
        });
    }

    /**
     * @notice Sets the LP pool capacity limit for current drawing
     * @dev Updates maximum allowed LP pool size. Called when parameters affecting pool size change.
     * @param _drawingId Drawing to update cap for
     * @param _lpPoolCap New maximum pool size
     * @custom:requirements
     * - Only Jackpot contract can call
     * - New cap must not be less than current LP pool total
     * @custom:effects
     * - Updates lpPoolCap for deposit validation
     * - Affects future deposit limits
     * @custom:security
     * - Access restricted to Jackpot contract
     * - Validates cap against current pool size to prevent system inconsistency
     */
    function setLPPoolCap(uint256 _drawingId, uint256 _lpPoolCap) external onlyJackpot() {
        LPDrawingState storage currentLP = lpDrawingState[_drawingId];
        if (_lpPoolCap < currentLP.lpPoolTotal + currentLP.pendingDeposits) revert InvalidLPPoolCap();
        lpPoolCap = _lpPoolCap;
    }

    // =============================================================
    //                          VIEW FUNCTIONS
    // =============================================================

    /**
     * @notice Returns the drawing accumulator value for share pricing
     * @dev Accumulator is used to convert between shares and USDC based on drawing performance
     * @param _drawingId Drawing to query
     * @return Accumulator value in PRECISE_UNIT scale
     */
    function getDrawingAccumulator(uint256 _drawingId) external view returns (uint256) {
        return drawingAccumulator[_drawingId];
    }

    /**
     * @notice Returns complete LP state for an address
     * @dev Includes consolidated shares, last deposit, pending withdrawal and claimable amounts
     * @param _lpAddress Address to query
     * @return LP struct containing all LP position information
     */
    function getLpInfo(address _lpAddress) external view returns (LP memory) {
        return lpInfo[_lpAddress];
    }

    /**
     * @notice Returns a USDC-denominated breakdown of an LP’s position by state for the current drawing
     * @dev Computes non-mutating, best-effort valuations of the LP’s funds across states:
     *      - activeDeposits: Consolidated shares (including preview consolidation of a prior-round lastDeposit)
     *        valued at the last settled accumulator, i.e., drawingAccumulator[currentDrawingId - 1].
     *      - pendingDeposits: Same-round lastDeposit.amount (USDC) if lastDeposit.drawingId == currentDrawingId; otherwise 0.
     *      - pendingWithdrawals: Same-round pendingWithdrawal.amountInShares valued at drawingAccumulator[currentDrawingId - 1].
     *        This is an estimate; final conversion occurs at settlement using the current drawing’s accumulator.
     *      - claimableWithdrawals: Prior-round pendingWithdrawal valued at
     *        drawingAccumulator[pendingWithdrawal.drawingId], plus existing claimableWithdrawals.
     *      All amounts are denominated in USDC wei (6 decimals). Integer division truncates (conservative rounding).
     * @param _lpAddress Address of the LP to query
     * @return breakdown LPValueBreakdown struct containing:
     *         - activeDeposits
     *         - pendingDeposits
     *         - pendingWithdrawals (estimate until settlement)
     *         - claimableWithdrawals
     * @custom:requirements
     * - LP system should be initialized (drawingAccumulator[0] set by initializeLP()).
     * - Assumes a settled accumulator exists for currentDrawingId - 1 (i.e., currentDrawingId > 0).
     * @custom:effects
     * - Read-only view; does not mutate state.
     * @custom:security
     * - Valuations reference historical accumulators; pending withdrawals are estimates and may differ from final settled amounts.
     */
    function getLPValueBreakdown(address _lpAddress) external view returns (LPValueBreakdown memory breakdown) {
        LP storage lp = lpInfo[_lpAddress];
        uint256 currentDrawingId = jackpot.currentDrawingId();
        uint256 consolidatedShares = lp.consolidatedShares;
        if (lp.lastDeposit.drawingId < currentDrawingId && lp.lastDeposit.amount > 0) {
            consolidatedShares += (lp.lastDeposit.amount * PRECISE_UNIT) / drawingAccumulator[lp.lastDeposit.drawingId];
        }

        uint256 claimableWithdrawals = lp.claimableWithdrawals;
        if (lp.pendingWithdrawal.drawingId < currentDrawingId && lp.pendingWithdrawal.amountInShares > 0) {
            claimableWithdrawals += (lp.pendingWithdrawal.amountInShares * drawingAccumulator[lp.pendingWithdrawal.drawingId]) / PRECISE_UNIT;
        }

        return LPValueBreakdown({
            activeDeposits: consolidatedShares * drawingAccumulator[currentDrawingId - 1] / PRECISE_UNIT,
            pendingDeposits: lp.lastDeposit.drawingId == currentDrawingId ? lp.lastDeposit.amount : 0,
            pendingWithdrawals: lp.pendingWithdrawal.drawingId == currentDrawingId ? 
                lp.pendingWithdrawal.amountInShares * drawingAccumulator[currentDrawingId - 1] / PRECISE_UNIT : 0,
            claimableWithdrawals: claimableWithdrawals
        });
    }

    /**
     * @notice Returns LP drawing state for a specific drawing
     * @dev Includes lpPoolTotal, pendingDeposits, pendingWithdrawals for the drawing
     * @param _drawingId Drawing to query
     * @return LPDrawingState struct containing drawing-specific LP data
     */
    function getLPDrawingState(uint256 _drawingId) external view returns (LPDrawingState memory) {
        return lpDrawingState[_drawingId];
    }

    // =============================================================
    //                          INTERNAL FUNCTIONS
    // =============================================================

    function _consolidateDeposits(LP storage _lp, uint256 _drawingId) internal {
        if (_lp.lastDeposit.amount > 0 && _lp.lastDeposit.drawingId < _drawingId) {
            // Accumulators can never be zero after first initialization because even if entire prizePool is won the LP
            // will still receive ticket revenue
            _lp.consolidatedShares += (_lp.lastDeposit.amount * PRECISE_UNIT) / drawingAccumulator[_lp.lastDeposit.drawingId];
            delete _lp.lastDeposit;
        }
    }

    function _consolidateWithdrawals(LP storage _lp, uint256 _drawingId) internal {
        if (_lp.pendingWithdrawal.amountInShares > 0 && _lp.pendingWithdrawal.drawingId < _drawingId) {
            // Accumulators can never be zero after first initialization because even if entire prizePool is won the LP
            // will still receive ticket revenue
            _lp.claimableWithdrawals += (_lp.pendingWithdrawal.amountInShares * drawingAccumulator[_lp.pendingWithdrawal.drawingId]) / PRECISE_UNIT;
            delete _lp.pendingWithdrawal;
        }
    }
}

//SPDX-License-Identifier: UNLICENSED

/*
Copyright (C) 2025 Coordination Inc.
All rights reserved.

This software is proprietary and confidential. Unauthorized copying,
distribution, or use is strictly prohibited and may result in legal action.

For licensing inquiries: legal@coordinationlabs.com
*/

pragma solidity ^0.8.28;

import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";

import { Combinations } from "./lib/Combinations.sol";
import { IJackpot } from "./interfaces/IJackpot.sol";
import { IPayoutCalculator } from "./interfaces/IPayoutCalculator.sol";

/**
 * @title GuaranteedMinimumPayoutCalculator
 * @notice Calculates prize payouts using a two-tier system with guaranteed minimums and premium allocation
 * @dev Implements a 12-tier payout system based on jackpot match combinations:
 *      - Tiers 0-11: matches(0-5) × bonusball(no/yes) = matches*2 + (bonusballMatch ? 1 : 0)
 *      - Each tier has configurable minimum payouts and premium pool allocation weights
 *      - Premium pool is allocated proportionally after minimum payouts are satisfied
 *      - Includes both user-owned and LP-owned winning tickets in allocation calculations
 *      - Supports owner-configurable payout parameters that can be updated between drawings
 */
contract GuaranteedMinimumPayoutCalculator is IPayoutCalculator, Ownable {

    // =============================================================
    //                          STRUCTS
    // =============================================================

    struct DrawingTierInfo {
        uint256 minPayout;
        uint256 premiumTierMinAllocation;
        bool[12] minPayoutTiers;
        uint256[12] premiumTierWeights;
    }

    // =============================================================
    //                          ERRORS
    // =============================================================

    error UnauthorizedCaller();
    error ZeroAddress();
    error InvalidTierWeights();
    error InvalidPremiumTierMinimumAllocation();

    // =============================================================
    //                          CONSTANTS
    // =============================================================

    uint256 public constant PRECISE_UNIT = 1e18;
    uint8 constant NORMAL_BALL_COUNT = 5;
    uint8 constant TOTAL_TIER_COUNT = 12; // matches(0,1,2,3,4,5) * bonusball(0,1) = 6 * 2 = 12

    // =============================================================
    //                          STATE VARIABLES
    // =============================================================
    
    // Note:drawingId --> tierId (matches*2 + {1 if bonusball match}) → tier payout
    mapping(uint256=>DrawingTierInfo) public drawingTierInfo;
    mapping(uint256=>mapping(uint256 => uint256)) tierPayouts;

    // Must be 12 elements long (matches(0,1,2,3,4,5) * bonusball(0,1))
    // Allocation of remaining prize pool after minimum payout is accounted for
    uint256[TOTAL_TIER_COUNT] public premiumTierWeights;
    // All tiers eligible for the minimum payout, true if eligible, false if not
    bool[TOTAL_TIER_COUNT] public minPayoutTiers;
    uint256 public minimumPayout;
    uint256 public premiumTierMinAllocation;

    IJackpot public immutable jackpot;

    // =============================================================
    //                          MODIFIERS
    // =============================================================
    
    modifier onlyJackpot() {
        if (msg.sender != address(jackpot)) revert UnauthorizedCaller();
        _;
    }

    // =============================================================
    //                          CONSTRUCTOR
    // =============================================================
    
    /**
     * @notice Initializes the GuaranteedMinimumPayoutCalculator with payout configuration
     * @dev Sets up the connection to the Jackpot contract and initial payout parameters.
     *      Premium tier weights must sum to PRECISE_UNIT for proper allocation.
     * @param _jackpot Address of the main Jackpot contract (immutable reference)
     * @param _minimumPayout Base minimum payout amount for eligible tiers
     * @param _premiumTierMinAllocation Minimum allocation of prize pool for premium tier (in PRECISE_UNIT scale)
     * @param _minPayoutTiers Boolean array indicating which tiers receive minimum payouts (12 elements)
     * @param _premiumTierWeights Weight allocation for premium pool distribution (12 elements, must sum to PRECISE_UNIT)
     * @custom:requirements
     * - Jackpot address must not be zero
     * - Premium tier weights must sum exactly to PRECISE_UNIT
     * - Arrays must be exactly 12 elements (TOTAL_TIER_COUNT)
     * @custom:effects
     * - Sets immutable jackpot contract reference
     * - Initializes minimum payout configuration
     * - Sets up premium tier allocation weights
     * - Sets deployer as contract owner
     * @custom:security
     * - Immutable jackpot reference prevents unauthorized contract changes
     * - Weight sum validation ensures proper allocation
     */
    constructor(
        IJackpot _jackpot,
        uint256 _minimumPayout,
        uint256 _premiumTierMinAllocation,
        bool[TOTAL_TIER_COUNT] memory _minPayoutTiers,
        uint256[TOTAL_TIER_COUNT] memory _premiumTierWeights
    ) Ownable(msg.sender) {
        if (_jackpot == IJackpot(address(0))) revert ZeroAddress();
        if (_premiumTierMinAllocation > PRECISE_UNIT) revert InvalidPremiumTierMinimumAllocation();
        jackpot = _jackpot;
        minimumPayout = _minimumPayout;
        premiumTierMinAllocation = _premiumTierMinAllocation;
        minPayoutTiers = _minPayoutTiers;
        _setPremiumTierWeights(_premiumTierWeights);
    }

    // =============================================================
    //                       EXTERNAL FUNCTIONS
    // =============================================================

    /**
     * @notice Calculates and stores payout amounts for a completed drawing
     * @dev Two-phase payout calculation with premium-protection threshold:
     *      1) Compute total winning tickets per tier, including duplicate user tickets, to avoid under-collateralization.
     *      2) Compute the minimum payout allocation across eligible tiers. Apply minimum payouts only if:
     *         (prizePool * premiumTierMinAllocation / 1e18) + minimumPayoutAllocation < prizePool.
     *         - If the inequality is false (i.e., equality or greater), minimum payouts are disabled for this drawing and the
     *           entire prize pool is allocated by premium weights. This ensures the premium tier receives at least its
     *           configured minimum allocation and avoids arithmetic underflow.
     *      Integer division is used for the premium minimum allocation term; any truncation favors premium protection.
     * @param _drawingId Drawing to calculate payouts for
     * @param _prizePool Total prize pool available for distribution
     * @param _normalMax Maximum normal ball number for combination calculations (assumed valid per Jackpot constraints)
     * @param _bonusballMax Maximum bonusball number for combination calculations (assumed valid per Jackpot constraints)
     * @param _uniqueResult Array of unique winner counts per tier (12 elements, user-owned tickets)
     * @param _dupResult Array of duplicate winner counts per tier (12 elements, user-owned tickets)
     * @return totalPayout Total amount allocated to all user-owned winning tickets
     * @custom:requirements
     * - Caller must be the Jackpot contract
     * - Tier parameters must be snapshotted via setDrawingTierInfo(_drawingId)
     * - `_uniqueResult.length == 12` and `_dupResult.length == 12`
     * - `_normalMax` and `_bonusballMax` provided by Jackpot must be valid for combination math
     * @custom:effects
     * - Calculates total winning ticket counts per tier (LP + users; duplicates included)
     * - Applies minimum payouts only if the premium-protection threshold is satisfied
     * - Distributes the remaining or full prize pool by premium tier weights
     * - Stores per-tier payout amounts in `tierPayouts`
     * - Returns the sum owed to user-owned winning tickets only (unique + duplicate)
     * @custom:security
     * - Access restricted to Jackpot contract
     * - Duplicate winners included in denominator to prevent under-collateralization
     * - Guards against underflow by disabling minimum payouts when insufficient pool remains
     */
    function calculateAndStoreDrawingUserWinnings(
        uint256 _drawingId,
        uint256 _prizePool,
        uint8 _normalMax,
        uint8 _bonusballMax,
        uint256[] memory _uniqueResult,
        uint256[] memory _dupResult
    )
        external
        onlyJackpot
        returns (uint256 totalPayout)
    {   
        DrawingTierInfo storage tierInfo = drawingTierInfo[_drawingId];

        // First calculate the total number of winners for each tier including duplicates and use that to determine guaranteed
        // minimum payouts
        uint256[TOTAL_TIER_COUNT] memory tierWinners;
        uint256 minimumPayoutAllocation = 0;
        for (uint256 i = 0; i < TOTAL_TIER_COUNT; i++) {
            // If tier is not eligible for minimum payout AND gets no part of premium allocation then no winners
            if (!tierInfo.minPayoutTiers[i] && tierInfo.premiumTierWeights[i] == 0) {
                tierWinners[i] = 0;
                continue;
            }
            // Derived from index formula which is matches*2 + {1 if bonusball match} (i/2 is always floored)
            uint256 matches = i / 2;

            // Including _dupResult[i] here takes money from the premium tier pool, if we don't include it we take money from the
            // LP pool reducing edge for LPs. Including here eliminates chances of under-collateralization. The amount of winners within
            // a tier is the total amount of winning tickets available for that tier plus any duplicate winners from that tier. The logic
            // here is that some of the non-duplicate winners are held by LPs and some are held by users. All of the duplicate winners
            // are held by users.
            uint256 tierWinningTickets = _calculateTierTotalWinningCombos(matches, _normalMax, _bonusballMax, i % 2 == 1) + _dupResult[i];
            tierWinners[i] = tierWinningTickets;
            if (tierInfo.minPayoutTiers[i]) {
                minimumPayoutAllocation += tierWinningTickets * tierInfo.minPayout;
            }
        }
        
        // Only use minimum payouts if the premium tier minimum allocation + minimum payout allocation is less than the prize pool
        bool useMinimumPayouts = ((_prizePool * tierInfo.premiumTierMinAllocation / PRECISE_UNIT) + minimumPayoutAllocation) < _prizePool;

        totalPayout = _calculateAndStoreTierPayouts(
            _drawingId,
            useMinimumPayouts ? _prizePool - minimumPayoutAllocation : _prizePool,
            useMinimumPayouts ? tierInfo.minPayout : 0,
            tierWinners,
            _uniqueResult,
            _dupResult
        );
    }

    /**
     * @notice Snapshots current payout configuration for a specific drawing
     * @dev Freezes current minimumPayout, minPayoutTiers, and premiumTierWeights into drawingTierInfo,
     *      allowing future configuration changes without affecting this drawing's payout calculations.
     * @param _drawingId Drawing to set up tier information for
     * @custom:requirements
     * - Only Jackpot contract can call
     * - Should be called before drawing execution
     * @custom:emits None
     * @custom:effects
     * - Creates immutable snapshot of current payout configuration
     * - Enables payout calculations for the specific drawing
     * - Allows future parameter updates without affecting this drawing
     * @custom:security
     * - Access restricted to Jackpot contract
     * - Ensures drawing payout consistency regardless of future changes
     */
    function setDrawingTierInfo(uint256 _drawingId) external onlyJackpot {
        drawingTierInfo[_drawingId] = DrawingTierInfo({
            minPayout: minimumPayout,
            premiumTierMinAllocation: premiumTierMinAllocation,
            minPayoutTiers: minPayoutTiers,
            premiumTierWeights: premiumTierWeights
        });
    }

    // =============================================================
    //                        ADMIN FUNCTIONS
    // =============================================================
    
    /**
     * @notice Updates the base minimum payout amount
     * @dev Changes the guaranteed minimum payout for all eligible tiers in future drawings.
     *      Does not affect drawings that have already had tier info set.
     * @param _minimumPayout New minimum payout amount (in USDC wei)
     * @custom:requirements
     * - Only owner can call
     * @custom:emits None
     * @custom:effects
     * - Updates global minimum payout configuration
     * - Affects future drawings only (after setDrawingTierInfo is called)
     * @custom:security
     * - Owner-only access restriction
     */
    function setMinimumPayout(uint256 _minimumPayout) external onlyOwner {
        minimumPayout = _minimumPayout;
    }
    
    /**
     * @notice Updates which tiers are eligible for minimum guaranteed payouts
     * @dev Configures which of the 12 tiers receive minimum payout guarantees in future drawings.
     *      Tiers with false values rely solely on premium pool allocation.
     * @param _minPayoutTiers Boolean array indicating minimum payout eligibility (12 elements)
     * @custom:requirements
     * - Only owner can call
     * - Array must be exactly 12 elements (TOTAL_TIER_COUNT)
     * @custom:emits None
     * @custom:effects
     * - Updates minimum payout tier configuration
     * - Affects future drawings only (after setDrawingTierInfo is called)
     * - Tiers set to false will only receive premium pool allocation
     * @custom:security
     * - Owner-only access restriction
     * - Array length validation
     */
    function setMinPayoutTiers(bool[TOTAL_TIER_COUNT] memory _minPayoutTiers) external onlyOwner {
        minPayoutTiers = _minPayoutTiers;
    }

    /**
     * @notice Updates the minimum allocation of the prize pool reserved for premium tier distribution
     * @dev Sets the minimum percentage of the total prize pool that must be allocated to premium tiers.
     *      This ensures premium tiers receive adequate funding even when minimum payouts consume most of the pool.
     *      The allocation is enforced during payout calculations by making sure the minimum payout allocation + 
     *      minimum premium allocation is less than the prize pool. If this equality fails then minimum payouts
     *      are disabled and the entire prize pool is distributed to the premium tiers.
     * @param _premiumTierMinAllocation Minimum allocation percentage in PRECISE_UNIT scale (e.g., 0.1e18 = 10%)
     * @custom:requirements
     * - Only owner can call
     * - Allocation percentage must not exceed 100% (PRECISE_UNIT)
     * @custom:emits None
     * @custom:effects
     * - Updates global premium tier minimum allocation configuration
     * - Affects future drawings only (after setDrawingTierInfo is called)
     * - Changes how prize pool is split between minimum payouts and premium allocation
     * @custom:security
     * - Owner-only access restriction
     * - Upper bound validation prevents invalid allocation percentages
     * - Ensures balanced distribution between guaranteed minimums and premium rewards
     */
    function setPremiumTierMinAllocation(uint256 _premiumTierMinAllocation) external onlyOwner {
        if (_premiumTierMinAllocation > PRECISE_UNIT) revert InvalidPremiumTierMinimumAllocation();
        premiumTierMinAllocation = _premiumTierMinAllocation;
    }

    /**
     * @notice Updates premium tier weight allocation
     * @dev Changes how the premium prize pool (after minimum payouts) is distributed across tiers.
     *      Weights must sum to PRECISE_UNIT to ensure complete allocation.
     * @param _premiumTierWeights Array of allocation weights (12 elements, must sum to PRECISE_UNIT)
     * @custom:requirements
     * - Only owner can call
     * - Array must be exactly 12 elements (TOTAL_TIER_COUNT)
     * - Weights must sum exactly to PRECISE_UNIT (1e18)
     * @custom:emits None
     * @custom:effects
     * - Updates premium pool allocation weights
     * - Affects future drawings only (after setDrawingTierInfo is called)
     * - Changes how remaining prize pool is distributed after minimum payouts
     * @custom:security
     * - Owner-only access restriction
     * - Weight sum validation ensures complete allocation
     * - Array length validation
     */
    function setPremiumTierWeights(uint256[TOTAL_TIER_COUNT] memory _premiumTierWeights) external onlyOwner {
        _setPremiumTierWeights(_premiumTierWeights);
    }

    // =============================================================
    //                        VIEW FUNCTIONS
    // =============================================================
    
    /**
     * @notice Returns the calculated payout amount for a specific tier in a drawing
     * @dev Retrieves the final payout amount per winning ticket for the specified tier.
     *      Returns 0 if no payout has been calculated or tier had no winners.
     * @param _drawingId Drawing to query
     * @param _tierId Tier to query (0-11, calculated as matches*2 + bonusballMatch)
     * @return Payout amount per winning ticket for the tier (in USDC wei)
     */
    function getTierPayout(uint256 _drawingId, uint256 _tierId) external view returns (uint256) {
        return tierPayouts[_drawingId][_tierId];
    }

    /**
     * @notice Returns all tier payouts for a specific drawing
     * @dev Retrieves the complete array of payout amounts for all 12 tiers in a drawing.
     *      Useful for displaying complete payout structure or performing batch calculations.
     * @param _drawingId Drawing to query
     * @return Array of payout amounts for all tiers (12 elements, in USDC wei)
     */
    function getDrawingTierPayouts(uint256 _drawingId) external view returns (uint256[TOTAL_TIER_COUNT] memory) {
        uint256[TOTAL_TIER_COUNT] memory drawingTierPayouts;
        for (uint256 i = 0; i < TOTAL_TIER_COUNT; i++) {
            drawingTierPayouts[i] = tierPayouts[_drawingId][i];
        }
        return drawingTierPayouts;
    }

    /**
     * @notice Returns current minimum payout tier configuration
     * @dev Shows which tiers are currently eligible for minimum guaranteed payouts.
     *      This reflects the current configuration, not necessarily what was used for past drawings.
     * @return Boolean array indicating minimum payout eligibility for each tier (12 elements)
     */
    function getMinPayoutTiers() external view returns (bool[TOTAL_TIER_COUNT] memory) {
        return minPayoutTiers;
    }

    /**
     * @notice Returns current premium tier weight configuration
     * @dev Shows current premium pool allocation weights across all tiers.
     *      This reflects the current configuration, not necessarily what was used for past drawings.
     * @return Array of premium pool allocation weights (12 elements, sum to PRECISE_UNIT)
     */
    function getPremiumTierWeights() external view returns (uint256[TOTAL_TIER_COUNT] memory) {
        return premiumTierWeights;
    }

    /**
     * @notice Returns the complete tier configuration used for a specific drawing
     * @dev Retrieves the snapshot of payout configuration that was frozen for the drawing.
     *      This shows the exact parameters used for payout calculations.
     * @param _drawingId Drawing to query
     * @return DrawingTierInfo struct containing minimum payout, tier eligibility, and premium weights
     */
    function getDrawingTierInfo(uint256 _drawingId) external view returns (DrawingTierInfo memory) {
        return drawingTierInfo[_drawingId];
    }

    // =============================================================
    //                        INTERNAL FUNCTIONS
    // =============================================================
    function _setPremiumTierWeights(uint256[TOTAL_TIER_COUNT] memory _premiumTierWeights) internal {
        uint256 tierWeightSum = 0;
        for (uint256 i = 0; i < TOTAL_TIER_COUNT; i++) {
            tierWeightSum += _premiumTierWeights[i];
        }
        if (tierWeightSum != PRECISE_UNIT) revert InvalidTierWeights();

        premiumTierWeights = _premiumTierWeights;
    }

    function _calculateAndStoreTierPayouts(
        uint256 _drawingId,
        uint256 _remainingPrizePool,
        uint256 _minPayout,
        uint256[TOTAL_TIER_COUNT] memory _tierWinners,
        uint256[] memory _uniqueResult,
        uint256[] memory _dupResult
    )
        internal
        returns(uint256 totalPayout)
    {
        DrawingTierInfo storage tierInfo = drawingTierInfo[_drawingId];
        for (uint256 i = 0; i < TOTAL_TIER_COUNT; i++) {
            // If no winners then no payout
            if (_tierWinners[i] != 0) {
                // Calculate the payout for each tier from the (remaining prize pool * weight) / total winning tickets
                //(including LP-owned winning tickets)
                uint256 premiumTierPayoutAmount = _remainingPrizePool * tierInfo.premiumTierWeights[i] / (PRECISE_UNIT * _tierWinners[i]);
                // Add the premium tier payout to the minimum payout if the tier is eligible for the minimum payout
                uint256 tierPayout = tierInfo.minPayoutTiers[i] ? _minPayout + premiumTierPayoutAmount : premiumTierPayoutAmount;
                // Store the payout for the tier in the mapping so it can be queried later and add the total tier payout to the total payout
                tierPayouts[_drawingId][i] = tierPayout;
                // the total amount of user-owned winning tickets for a given tier is the sum of result and dupResult
                totalPayout += tierPayout * (_uniqueResult[i] + _dupResult[i]);
            }
        }
    }

    function _calculateTierTotalWinningCombos(
        uint256 _matches,
        uint8 _normalMax, 
        uint8 _bonusballMax,
        bool _bonusballMatch
    )
        internal
        pure
        returns(uint256)
    {
        if (_bonusballMatch) {
            return Combinations.choose(NORMAL_BALL_COUNT, _matches) * Combinations.choose(_normalMax - NORMAL_BALL_COUNT, NORMAL_BALL_COUNT - _matches);
        } else {
            return Combinations.choose(NORMAL_BALL_COUNT, _matches) * Combinations.choose(_normalMax - NORMAL_BALL_COUNT, NORMAL_BALL_COUNT - _matches) * (_bonusballMax - 1);
        }
    }
}


## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

