
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {ERC721} from "solady/tokens/ERC721.sol";
import {LibString} from "solady/utils/LibString.sol";
import {Ownable} from "solady/auth/Ownable.sol";

import {IBaseNonfungibleToken} from "../interfaces/IBaseNonfungibleToken.sol";

/// @title Base Nonfungible Token
/// @notice Abstract NFT contract where tokens can be minted and burned freely, and the owner can change the metadata
/// @dev This contract provides a base implementation for NFTs with deterministic ID generation based on minter and salt.
///      Token IDs are generated using keccak256(minter, salt, chainid, contract_address) to ensure uniqueness.
///      The contract allows free minting and burning, with metadata management restricted to the owner.
abstract contract BaseNonfungibleToken is IBaseNonfungibleToken, Ownable, ERC721 {
    /// @notice Thrown when a caller is not authorized to perform an action on a specific token
    /// @param caller The address that attempted the unauthorized action
    /// @param id The token ID for which authorization was required
    error NotUnauthorizedForToken(address caller, uint256 id);

    /// @dev The name of the NFT collection
    string private _name;

    /// @dev The symbol of the NFT collection
    string private _symbol;

    /// @notice The base URL used for constructing token URIs
    /// @dev Token URIs are constructed by concatenating this base URL with the token ID
    string public baseUrl;

    /// @notice Initializes the contract with the specified owner
    /// @param owner The address that will be set as the owner of the contract
    constructor(address owner) {
        _initializeOwner(owner);
    }

    /// @notice Updates the metadata for the NFT collection
    /// @dev Only the contract owner can call this function
    /// @param newName The new name for the NFT collection
    /// @param newSymbol The new symbol for the NFT collection
    /// @param newBaseUrl The new base URL for token metadata
    function setMetadata(string memory newName, string memory newSymbol, string memory newBaseUrl) external onlyOwner {
        _name = newName;
        _symbol = newSymbol;
        baseUrl = newBaseUrl;
    }

    /// @notice Returns the name of the NFT collection
    /// @return The name of the collection
    function name() public view override returns (string memory) {
        return _name;
    }

    /// @notice Returns the symbol of the NFT collection
    /// @return The symbol of the collection
    function symbol() public view override returns (string memory) {
        return _symbol;
    }

    /// @notice Returns the URI for a given token ID
    /// @dev Constructs the URI by concatenating the base URL with the token ID.
    ///      If baseUrl is empty, returns just the stringified token ID.
    /// @param id The token ID to get the URI for
    /// @return The complete URI for the token
    function tokenURI(uint256 id) public view virtual override returns (string memory) {
        return string(
            abi.encodePacked(
                baseUrl,
                LibString.toString(block.chainid),
                "/",
                LibString.toHexStringChecksummed(address(this)),
                "/",
                LibString.toString(id)
            )
        );
    }

    /// @notice Modifier to ensure the caller is authorized to perform actions on a specific token
    /// @dev Checks if the caller is the owner or approved for the token
    /// @param id The token ID to check authorization for
    modifier authorizedForNft(uint256 id) {
        if (!_isApprovedOrOwner(msg.sender, id)) {
            revert NotUnauthorizedForToken(msg.sender, id);
        }
        _;
    }

    /// @inheritdoc IBaseNonfungibleToken
    /// @dev Uses keccak256 hash of minter, salt, chain ID, and contract address to generate unique IDs.
    ///      IDs are deterministic per (minter, salt, chainId, contract) tuple; the same pair on a
    ///      different chain or contract yields a different ID.
    function saltToId(address minter, bytes32 salt) public view returns (uint256 result) {
        assembly ("memory-safe") {
            let free := mload(0x40)
            mstore(free, minter)
            mstore(add(free, 32), salt)
            mstore(add(free, 64), chainid())
            mstore(add(free, 96), address())

            result := keccak256(free, 128)
        }
    }

    /// @inheritdoc IBaseNonfungibleToken
    /// @dev Generates a salt using prevrandao() and gas() for pseudorandomness.
    ///      Note: This can encounter conflicts if a sender sends two identical transactions
    ///      in the same block that consume exactly the same amount of gas.
    ///      No fees are collected; any msg.value sent is ignored.
    function mint() public payable returns (uint256 id) {
        bytes32 salt;
        assembly ("memory-safe") {
            mstore(0, prevrandao())
            mstore(32, gas())
            salt := keccak256(0, 64)
        }
        id = mint(salt);
    }

    /// @inheritdoc IBaseNonfungibleToken
    /// @dev The token ID is generated using saltToId(msg.sender, salt). This prevents the need
    ///      to store a counter of how many tokens were minted, as IDs are deterministic.
    ///      No fees are collected; any msg.value sent is ignored.
    function mint(bytes32 salt) public payable returns (uint256 id) {
        id = saltToId(msg.sender, salt);
        _mint(msg.sender, id);
    }

    /// @inheritdoc IBaseNonfungibleToken
    /// @dev Can be used to refund some gas after the NFT is no longer needed.
    ///      The same ID can be recreated by the original minter by reusing the salt.
    ///      Only the token owner or approved addresses can burn the token.
    ///      No fees are collected; any msg.value sent is ignored.
    function burn(uint256 id) external payable authorizedForNft(id) {
        _burn(id);
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

/// @notice Simple single owner authorization mixin.
/// @author Solady (https://github.com/vectorized/solady/blob/main/src/auth/Ownable.sol)
///
/// @dev Note:
/// This implementation does NOT auto-initialize the owner to `msg.sender`.
/// You MUST call the `_initializeOwner` in the constructor / initializer.
///
/// While the ownable portion follows
/// [EIP-173](https://eips.ethereum.org/EIPS/eip-173) for compatibility,
/// the nomenclature for the 2-step ownership handover may be unique to this codebase.
abstract contract Ownable {
    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                       CUSTOM ERRORS                        */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev The caller is not authorized to call the function.
    error Unauthorized();

    /// @dev The `newOwner` cannot be the zero address.
    error NewOwnerIsZeroAddress();

    /// @dev The `pendingOwner` does not have a valid handover request.
    error NoHandoverRequest();

    /// @dev Cannot double-initialize.
    error AlreadyInitialized();

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                           EVENTS                           */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev The ownership is transferred from `oldOwner` to `newOwner`.
    /// This event is intentionally kept the same as OpenZeppelin's Ownable to be
    /// compatible with indexers and [EIP-173](https://eips.ethereum.org/EIPS/eip-173),
    /// despite it not being as lightweight as a single argument event.
    event OwnershipTransferred(address indexed oldOwner, address indexed newOwner);

    /// @dev An ownership handover to `pendingOwner` has been requested.
    event OwnershipHandoverRequested(address indexed pendingOwner);

    /// @dev The ownership handover to `pendingOwner` has been canceled.
    event OwnershipHandoverCanceled(address indexed pendingOwner);

    /// @dev `keccak256(bytes("OwnershipTransferred(address,address)"))`.
    uint256 private constant _OWNERSHIP_TRANSFERRED_EVENT_SIGNATURE =
        0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0;

    /// @dev `keccak256(bytes("OwnershipHandoverRequested(address)"))`.
    uint256 private constant _OWNERSHIP_HANDOVER_REQUESTED_EVENT_SIGNATURE =
        0xdbf36a107da19e49527a7176a1babf963b4b0ff8cde35ee35d6cd8f1f9ac7e1d;

    /// @dev `keccak256(bytes("OwnershipHandoverCanceled(address)"))`.
    uint256 private constant _OWNERSHIP_HANDOVER_CANCELED_EVENT_SIGNATURE =
        0xfa7b8eab7da67f412cc9575ed43464468f9bfbae89d1675917346ca6d8fe3c92;

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                          STORAGE                           */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev The owner slot is given by:
    /// `bytes32(~uint256(uint32(bytes4(keccak256("_OWNER_SLOT_NOT")))))`.
    /// It is intentionally chosen to be a high value
    /// to avoid collision with lower slots.
    /// The choice of manual storage layout is to enable compatibility
    /// with both regular and upgradeable contracts.
    bytes32 internal constant _OWNER_SLOT =
        0xffffffffffffffffffffffffffffffffffffffffffffffffffffffff74873927;

    /// The ownership handover slot of `newOwner` is given by:
    /// ```
    ///     mstore(0x00, or(shl(96, user), _HANDOVER_SLOT_SEED))
    ///     let handoverSlot := keccak256(0x00, 0x20)
    /// ```
    /// It stores the expiry timestamp of the two-step ownership handover.
    uint256 private constant _HANDOVER_SLOT_SEED = 0x389a75e1;

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                     INTERNAL FUNCTIONS                     */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev Override to return true to make `_initializeOwner` prevent double-initialization.
    function _guardInitializeOwner() internal pure virtual returns (bool guard) {}

    /// @dev Initializes the owner directly without authorization guard.
    /// This function must be called upon initialization,
    /// regardless of whether the contract is upgradeable or not.
    /// This is to enable generalization to both regular and upgradeable contracts,
    /// and to save gas in case the initial owner is not the caller.
    /// For performance reasons, this function will not check if there
    /// is an existing owner.
    function _initializeOwner(address newOwner) internal virtual {
        if (_guardInitializeOwner()) {
            /// @solidity memory-safe-assembly
            assembly {
                let ownerSlot := _OWNER_SLOT
                if sload(ownerSlot) {
                    mstore(0x00, 0x0dc149f0) // `AlreadyInitialized()`.
                    revert(0x1c, 0x04)
                }
                // Clean the upper 96 bits.
                newOwner := shr(96, shl(96, newOwner))
                // Store the new value.
                sstore(ownerSlot, or(newOwner, shl(255, iszero(newOwner))))
                // Emit the {OwnershipTransferred} event.
                log3(0, 0, _OWNERSHIP_TRANSFERRED_EVENT_SIGNATURE, 0, newOwner)
            }
        } else {
            /// @solidity memory-safe-assembly
            assembly {
                // Clean the upper 96 bits.
                newOwner := shr(96, shl(96, newOwner))
                // Store the new value.
                sstore(_OWNER_SLOT, newOwner)
                // Emit the {OwnershipTransferred} event.
                log3(0, 0, _OWNERSHIP_TRANSFERRED_EVENT_SIGNATURE, 0, newOwner)
            }
        }
    }

    /// @dev Sets the owner directly without authorization guard.
    function _setOwner(address newOwner) internal virtual {
        if (_guardInitializeOwner()) {
            /// @solidity memory-safe-assembly
            assembly {
                let ownerSlot := _OWNER_SLOT
                // Clean the upper 96 bits.
                newOwner := shr(96, shl(96, newOwner))
                // Emit the {OwnershipTransferred} event.
                log3(0, 0, _OWNERSHIP_TRANSFERRED_EVENT_SIGNATURE, sload(ownerSlot), newOwner)
                // Store the new value.
                sstore(ownerSlot, or(newOwner, shl(255, iszero(newOwner))))
            }
        } else {
            /// @solidity memory-safe-assembly
            assembly {
                let ownerSlot := _OWNER_SLOT
                // Clean the upper 96 bits.
                newOwner := shr(96, shl(96, newOwner))
                // Emit the {OwnershipTransferred} event.
                log3(0, 0, _OWNERSHIP_TRANSFERRED_EVENT_SIGNATURE, sload(ownerSlot), newOwner)
                // Store the new value.
                sstore(ownerSlot, newOwner)
            }
        }
    }

    /// @dev Throws if the sender is not the owner.
    function _checkOwner() internal view virtual {
        /// @solidity memory-safe-assembly
        assembly {
            // If the caller is not the stored owner, revert.
            if iszero(eq(caller(), sload(_OWNER_SLOT))) {
                mstore(0x00, 0x82b42900) // `Unauthorized()`.
                revert(0x1c, 0x04)
            }
        }
    }

    /// @dev Returns how long a two-step ownership handover is valid for in seconds.
    /// Override to return a different value if needed.
    /// Made internal to conserve bytecode. Wrap it in a public function if needed.
    function _ownershipHandoverValidFor() internal view virtual returns (uint64) {
        return 48 * 3600;
    }

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                  PUBLIC UPDATE FUNCTIONS                   */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev Allows the owner to transfer the ownership to `newOwner`.
    function transferOwnership(address newOwner) public payable virtual onlyOwner {
        /// @solidity memory-safe-assembly
        assembly {
            if iszero(shl(96, newOwner)) {
                mstore(0x00, 0x7448fbae) // `NewOwnerIsZeroAddress()`.
                revert(0x1c, 0x04)
            }
        }
        _setOwner(newOwner);
    }

    /// @dev Allows the owner to renounce their ownership.
    function renounceOwnership() public payable virtual onlyOwner {
        _setOwner(address(0));
    }

    /// @dev Request a two-step ownership handover to the caller.
    /// The request will automatically expire in 48 hours (172800 seconds) by default.
    function requestOwnershipHandover() public payable virtual {
        unchecked {
            uint256 expires = block.timestamp + _ownershipHandoverValidFor();
            /// @solidity memory-safe-assembly
            assembly {
                // Compute and set the handover slot to `expires`.
                mstore(0x0c, _HANDOVER_SLOT_SEED)
                mstore(0x00, caller())
                sstore(keccak256(0x0c, 0x20), expires)
                // Emit the {OwnershipHandoverRequested} event.
                log2(0, 0, _OWNERSHIP_HANDOVER_REQUESTED_EVENT_SIGNATURE, caller())
            }
        }
    }

    /// @dev Cancels the two-step ownership handover to the caller, if any.
    function cancelOwnershipHandover() public payable virtual {
        /// @solidity memory-safe-assembly
        assembly {
            // Compute and set the handover slot to 0.
            mstore(0x0c, _HANDOVER_SLOT_SEED)
            mstore(0x00, caller())
            sstore(keccak256(0x0c, 0x20), 0)
            // Emit the {OwnershipHandoverCanceled} event.
            log2(0, 0, _OWNERSHIP_HANDOVER_CANCELED_EVENT_SIGNATURE, caller())
        }
    }

    /// @dev Allows the owner to complete the two-step ownership handover to `pendingOwner`.
    /// Reverts if there is no existing ownership handover requested by `pendingOwner`.
    function completeOwnershipHandover(address pendingOwner) public payable virtual onlyOwner {
        /// @solidity memory-safe-assembly
        assembly {
            // Compute and set the handover slot to 0.
            mstore(0x0c, _HANDOVER_SLOT_SEED)
            mstore(0x00, pendingOwner)
            let handoverSlot := keccak256(0x0c, 0x20)
            // If the handover does not exist, or has expired.
            if gt(timestamp(), sload(handoverSlot)) {
                mstore(0x00, 0x6f5e8818) // `NoHandoverRequest()`.
                revert(0x1c, 0x04)
            }
            // Set the handover slot to 0.
            sstore(handoverSlot, 0)
        }
        _setOwner(pendingOwner);
    }

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                   PUBLIC READ FUNCTIONS                    */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev Returns the owner of the contract.
    function owner() public view virtual returns (address result) {
        /// @solidity memory-safe-assembly
        assembly {
            result := sload(_OWNER_SLOT)
        }
    }

    /// @dev Returns the expiry timestamp for the two-step ownership handover to `pendingOwner`.
    function ownershipHandoverExpiresAt(address pendingOwner)
        public
        view
        virtual
        returns (uint256 result)
    {
        /// @solidity memory-safe-assembly
        assembly {
            // Compute the handover slot.
            mstore(0x0c, _HANDOVER_SLOT_SEED)
            mstore(0x00, pendingOwner)
            // Load the handover slot.
            result := sload(keccak256(0x0c, 0x20))
        }
    }

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                         MODIFIERS                          */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev Marks a function as only callable by the owner.
    modifier onlyOwner() virtual {
        _checkOwner();
        _;
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

/// @title Base Nonfungible Token Interface
/// @notice Interface for the base NFT functionality used by Positions and Orders contracts
/// @dev Defines the essential NFT functions needed by other contracts in the system
interface IBaseNonfungibleToken {
    function setMetadata(string memory newName, string memory newSymbol, string memory newBaseUrl) external;

    // BaseNonfungibleToken specific functions
    /// @notice Converts a minter address and salt to a token ID
    /// @param minter The minter address
    /// @param salt The salt value
    /// @return result The resulting token ID
    function saltToId(address minter, bytes32 salt) external view returns (uint256 result);

    /// @notice Mints a new token with a random salt
    /// @return id The minted token ID
    function mint() external payable returns (uint256 id);

    /// @notice Mints a new token with a specific salt
    /// @param salt The salt for deterministic ID generation
    /// @return id The minted token ID
    function mint(bytes32 salt) external payable returns (uint256 id);

    /// @notice Burns a token (only authorized addresses)
    /// @param id The token ID to burn
    function burn(uint256 id) external payable;
}


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS
// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {BaseLocker} from "./base/BaseLocker.sol";
import {UsesCore} from "./base/UsesCore.sol";
import {ICore} from "./interfaces/ICore.sol";
import {IOrders} from "./interfaces/IOrders.sol";
import {PayableMulticallable} from "./base/PayableMulticallable.sol";
import {TWAMMLib} from "./libraries/TWAMMLib.sol";
import {ITWAMM} from "./interfaces/extensions/ITWAMM.sol";
import {OrderKey} from "./types/orderKey.sol";
import {computeSaleRate} from "./math/twamm.sol";
import {BaseNonfungibleToken} from "./base/BaseNonfungibleToken.sol";
import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";
import {SafeCastLib} from "solady/utils/SafeCastLib.sol";
import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";
import {NATIVE_TOKEN_ADDRESS} from "./math/constants.sol";
import {FlashAccountantLib} from "./libraries/FlashAccountantLib.sol";

/// @title Ekubo Protocol Orders
/// @author Moody Salem <moody@ekubo.org>
/// @notice Tracks TWAMM (Time-Weighted Average Market Maker) orders in Ekubo Protocol as NFTs
/// @dev Manages long-term orders that execute over time through the TWAMM extension
contract Orders is IOrders, UsesCore, PayableMulticallable, BaseLocker, BaseNonfungibleToken {
    using TWAMMLib for *;
    using FlashAccountantLib for *;

    uint256 private constant CALL_TYPE_CHANGE_SALE_RATE = 0;
    uint256 private constant CALL_TYPE_COLLECT_PROCEEDS = 1;

    /// @notice The TWAMM extension contract that handles order execution
    ITWAMM public immutable TWAMM_EXTENSION;

    /// @notice Constructs the Orders contract
    /// @param core The core contract instance
    /// @param _twamm The TWAMM extension contract
    /// @param owner The owner of the contract (for access control)
    constructor(ICore core, ITWAMM _twamm, address owner) BaseNonfungibleToken(owner) BaseLocker(core) UsesCore(core) {
        TWAMM_EXTENSION = _twamm;
    }

    /// @inheritdoc IOrders
    function mintAndIncreaseSellAmount(OrderKey memory orderKey, uint112 amount, uint112 maxSaleRate)
        public
        payable
        returns (uint256 id, uint112 saleRate)
    {
        id = mint();
        saleRate = increaseSellAmount(id, orderKey, amount, maxSaleRate);
    }

    /// @inheritdoc IOrders
    function increaseSellAmount(uint256 id, OrderKey memory orderKey, uint128 amount, uint112 maxSaleRate)
        public
        payable
        authorizedForNft(id)
        returns (uint112 saleRate)
    {
        uint256 realStart = FixedPointMathLib.max(block.timestamp, orderKey.config.startTime());

        unchecked {
            if (orderKey.config.endTime() <= realStart) {
                revert OrderAlreadyEnded();
            }

            saleRate = uint112(computeSaleRate(amount, uint32(orderKey.config.endTime() - realStart)));

            if (saleRate > maxSaleRate) {
                revert MaxSaleRateExceeded();
            }
        }

        lock(abi.encode(CALL_TYPE_CHANGE_SALE_RATE, msg.sender, id, orderKey, saleRate));
    }

    /// @inheritdoc IOrders
    function decreaseSaleRate(uint256 id, OrderKey memory orderKey, uint112 saleRateDecrease, address recipient)
        public
        payable
        authorizedForNft(id)
        returns (uint112 refund)
    {
        refund = uint112(
            uint256(
                -abi.decode(
                    lock(
                        abi.encode(
                            CALL_TYPE_CHANGE_SALE_RATE, recipient, id, orderKey, -int256(uint256(saleRateDecrease))
                        )
                    ),
                    (int256)
                )
            )
        );
    }

    /// @inheritdoc IOrders
    function decreaseSaleRate(uint256 id, OrderKey memory orderKey, uint112 saleRateDecrease)
        external
        payable
        returns (uint112 refund)
    {
        refund = decreaseSaleRate(id, orderKey, saleRateDecrease, msg.sender);
    }

    /// @inheritdoc IOrders
    function collectProceeds(uint256 id, OrderKey memory orderKey, address recipient)
        public
        payable
        authorizedForNft(id)
        returns (uint128 proceeds)
    {
        proceeds = abi.decode(lock(abi.encode(CALL_TYPE_COLLECT_PROCEEDS, id, orderKey, recipient)), (uint128));
    }

    /// @inheritdoc IOrders
    function collectProceeds(uint256 id, OrderKey memory orderKey) external payable returns (uint128 proceeds) {
        proceeds = collectProceeds(id, orderKey, msg.sender);
    }

    /// @inheritdoc IOrders
    function executeVirtualOrdersAndGetCurrentOrderInfo(uint256 id, OrderKey memory orderKey)
        external
        returns (uint112 saleRate, uint256 amountSold, uint256 remainingSellAmount, uint128 purchasedAmount)
    {
        (saleRate, amountSold, remainingSellAmount, purchasedAmount) =
            TWAMM_EXTENSION.executeVirtualOrdersAndGetCurrentOrderInfo(address(this), bytes32(id), orderKey);
    }

    /// @notice Handles lock callback data for order operations
    /// @dev Internal function that processes different types of order operations
    /// @param data Encoded operation data
    /// @return result Encoded result data
    function handleLockData(uint256, bytes memory data) internal override returns (bytes memory result) {
        uint256 callType = abi.decode(data, (uint256));

        if (callType == CALL_TYPE_CHANGE_SALE_RATE) {
            (, address recipientOrPayer, uint256 id, OrderKey memory orderKey, int256 saleRateDelta) =
                abi.decode(data, (uint256, address, uint256, OrderKey, int256));

            int256 amount =
                CORE.updateSaleRate(TWAMM_EXTENSION, bytes32(id), orderKey, SafeCastLib.toInt112(saleRateDelta));

            if (amount != 0) {
                address sellToken = orderKey.sellToken();
                if (saleRateDelta > 0) {
                    if (sellToken == NATIVE_TOKEN_ADDRESS) {
                        SafeTransferLib.safeTransferETH(address(ACCOUNTANT), uint256(amount));
                    } else {
                        ACCOUNTANT.payFrom(recipientOrPayer, sellToken, uint256(amount));
                    }
                } else {
                    unchecked {
                        // we know amount will never exceed the uint128 type because of limitations on sale rate (fixed point 80.32) and duration (uint32)
                        ACCOUNTANT.withdraw(sellToken, recipientOrPayer, uint128(uint256(-amount)));
                    }
                }
            }

            result = abi.encode(amount);
        } else if (callType == CALL_TYPE_COLLECT_PROCEEDS) {
            (, uint256 id, OrderKey memory orderKey, address recipient) =
                abi.decode(data, (uint256, uint256, OrderKey, address));

            uint128 proceeds = CORE.collectProceeds(TWAMM_EXTENSION, bytes32(id), orderKey);

            if (proceeds != 0) {
                ACCOUNTANT.withdraw(orderKey.buyToken(), recipient, proceeds);
            }

            result = abi.encode(proceeds);
        } else {
            revert();
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {CallPoints, addressToCallPoints} from "./types/callPoints.sol";
import {PoolKey} from "./types/poolKey.sol";
import {PoolConfig} from "./types/poolConfig.sol";
import {PositionId} from "./types/positionId.sol";
import {FeesPerLiquidity, feesPerLiquidityFromAmounts} from "./types/feesPerLiquidity.sol";
import {Position} from "./types/position.sol";
import {tickToSqrtRatio, sqrtRatioToTick} from "./math/ticks.sol";
import {CoreStorageLayout} from "./libraries/CoreStorageLayout.sol";
import {ExtensionCallPointsLib} from "./libraries/ExtensionCallPointsLib.sol";
import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";
import {SafeCastLib} from "solady/utils/SafeCastLib.sol";
import {ExposedStorage} from "./base/ExposedStorage.sol";
import {liquidityDeltaToAmountDelta, addLiquidityDelta} from "./math/liquidity.sol";
import {findNextInitializedTick, findPrevInitializedTick, flipTick} from "./math/tickBitmap.sol";
import {ICore, IExtension} from "./interfaces/ICore.sol";
import {FlashAccountant} from "./base/FlashAccountant.sol";
import {MIN_TICK, MAX_TICK, NATIVE_TOKEN_ADDRESS} from "./math/constants.sol";
import {MIN_SQRT_RATIO, MAX_SQRT_RATIO, SqrtRatio} from "./types/sqrtRatio.sol";
import {PoolState, createPoolState} from "./types/poolState.sol";
import {SwapParameters} from "./types/swapParameters.sol";
import {TickInfo, createTickInfo} from "./types/tickInfo.sol";
import {PoolBalanceUpdate, createPoolBalanceUpdate} from "./types/poolBalanceUpdate.sol";
import {PoolId} from "./types/poolId.sol";
import {Locker} from "./types/locker.sol";
import {computeFee, amountBeforeFee} from "./math/fee.sol";
import {nextSqrtRatioFromAmount0, nextSqrtRatioFromAmount1} from "./math/sqrtRatio.sol";
import {
    amount0Delta,
    amount1Delta,
    amount0DeltaSorted,
    amount1DeltaSorted,
    sortAndConvertToFixedSqrtRatios
} from "./math/delta.sol";
import {StorageSlot} from "./types/storageSlot.sol";
import {LibBit} from "solady/utils/LibBit.sol";

/// @title Ekubo Protocol Core
/// @author Moody Salem <moody@ekubo.org>
/// @notice Singleton contract holding all tokens and containing all possible operations in Ekubo Protocol
/// @dev Implements the core AMM functionality including pools, positions, swaps, and fee collection
/// @dev Note this code is under the terms of the Ekubo DAO Shared Revenue License 1.0.
/// @dev The full terms of the license can be found at the contenthash specified at ekubo-license-v1.eth.
contract Core is ICore, FlashAccountant, ExposedStorage {
    using ExtensionCallPointsLib for *;

    /// @inheritdoc ICore
    function registerExtension(CallPoints memory expectedCallPoints) external {
        CallPoints memory computed = addressToCallPoints(msg.sender);
        if (!computed.eq(expectedCallPoints) || !computed.isValid()) {
            revert FailedRegisterInvalidCallPoints();
        }
        StorageSlot isExtensionRegisteredSlot = CoreStorageLayout.isExtensionRegisteredSlot(msg.sender);
        if (isExtensionRegisteredSlot.load() != bytes32(0)) revert ExtensionAlreadyRegistered();

        isExtensionRegisteredSlot.store(bytes32(LibBit.rawToUint(true)));

        emit ExtensionRegistered(msg.sender);
    }

    function readPoolState(PoolId poolId) internal view returns (PoolState state) {
        state = PoolState.wrap(CoreStorageLayout.poolStateSlot(poolId).load());
    }

    function writePoolState(PoolId poolId, PoolState state) internal {
        CoreStorageLayout.poolStateSlot(poolId).store(PoolState.unwrap(state));
    }

    /// @inheritdoc ICore
    function initializePool(PoolKey memory poolKey, int32 tick) external returns (SqrtRatio sqrtRatio) {
        poolKey.validate();

        address extension = poolKey.config.extension();
        if (extension != address(0)) {
            StorageSlot isExtensionRegisteredSlot = CoreStorageLayout.isExtensionRegisteredSlot(extension);

            if (isExtensionRegisteredSlot.load() == bytes32(0)) {
                revert ExtensionNotRegistered();
            }

            IExtension(extension).maybeCallBeforeInitializePool(msg.sender, poolKey, tick);
        }

        PoolId poolId = poolKey.toPoolId();
        PoolState state = readPoolState(poolId);
        if (state.isInitialized()) revert PoolAlreadyInitialized();

        sqrtRatio = tickToSqrtRatio(tick);
        writePoolState(poolId, createPoolState({_sqrtRatio: sqrtRatio, _tick: tick, _liquidity: 0}));

        // initialize these slots so the first swap or deposit on the pool is the same cost as any other swap
        StorageSlot fplSlot0 = CoreStorageLayout.poolFeesPerLiquiditySlot(poolId);
        fplSlot0.store(bytes32(uint256(1)));
        fplSlot0.next().store(bytes32(uint256(1)));

        emit PoolInitialized(poolId, poolKey, tick, sqrtRatio);

        IExtension(extension).maybeCallAfterInitializePool(msg.sender, poolKey, tick, sqrtRatio);
    }

    /// @inheritdoc ICore
    function prevInitializedTick(PoolId poolId, int32 fromTick, uint32 tickSpacing, uint256 skipAhead)
        external
        view
        returns (int32 tick, bool isInitialized)
    {
        (tick, isInitialized) =
            findPrevInitializedTick(CoreStorageLayout.tickBitmapsSlot(poolId), fromTick, tickSpacing, skipAhead);
    }

    /// @inheritdoc ICore
    function nextInitializedTick(PoolId poolId, int32 fromTick, uint32 tickSpacing, uint256 skipAhead)
        external
        view
        returns (int32 tick, bool isInitialized)
    {
        (tick, isInitialized) =
            findNextInitializedTick(CoreStorageLayout.tickBitmapsSlot(poolId), fromTick, tickSpacing, skipAhead);
    }

    /// @inheritdoc ICore
    function updateSavedBalances(
        address token0,
        address token1,
        bytes32,
        // positive is saving, negative is loading
        int256 delta0,
        int256 delta1
    )
        external
        payable
    {
        if (token0 >= token1) revert SavedBalanceTokensNotSorted();

        (uint256 id, address lockerAddr) = _requireLocker().parse();

        assembly ("memory-safe") {
            function addDelta(u, i) -> result {
                // full‐width sum mod 2^256
                let sum := add(u, i)
                // 1 if i<0 else 0
                let sign := shr(255, i)
                // if sum > type(uint128).max || (i>=0 && sum<u) || (i<0 && sum>u) ⇒ 256-bit wrap or underflow
                if or(shr(128, sum), or(and(iszero(sign), lt(sum, u)), and(sign, gt(sum, u)))) {
                    mstore(0x00, 0x1293d6fa) // `SavedBalanceOverflow()`
                    revert(0x1c, 0x04)
                }
                result := sum
            }

            // we can cheaply calldatacopy the arguments into memory, hence no call to CoreStorageLayout#savedBalancesSlot
            let free := mload(0x40)
            mstore(free, lockerAddr)
            // copy the first 3 arguments in the same order
            calldatacopy(add(free, 0x20), 4, 96)
            let slot := keccak256(free, 128)
            let balances := sload(slot)

            let b0 := shr(128, balances)
            let b1 := shr(128, shl(128, balances))

            let b0Next := addDelta(b0, delta0)
            let b1Next := addDelta(b1, delta1)

            sstore(slot, add(shl(128, b0Next), b1Next))
        }

        _updatePairDebtWithNative(id, token0, token1, delta0, delta1);
    }

    /// @notice Returns the pool fees per liquidity inside the given bounds
    /// @dev Internal function that calculates fees per liquidity within position bounds
    /// @param poolId Unique identifier for the pool
    /// @param tick Current tick of the pool
    /// @param tickLower Lower tick of the price range to get the snapshot of
    /// @param tickLower Upper tick of the price range to get the snapshot of
    /// @return feesPerLiquidityInside Accumulated fees per liquidity snapshot inside the bounds. Note this is a relative value.
    function _getPoolFeesPerLiquidityInside(PoolId poolId, int32 tick, int32 tickLower, int32 tickUpper)
        internal
        view
        returns (FeesPerLiquidity memory feesPerLiquidityInside)
    {
        uint256 lower0;
        uint256 lower1;
        uint256 upper0;
        uint256 upper1;
        {
            (StorageSlot l0, StorageSlot l1) = CoreStorageLayout.poolTickFeesPerLiquidityOutsideSlot(poolId, tickLower);
            (lower0, lower1) = (uint256(l0.load()), uint256(l1.load()));

            (StorageSlot u0, StorageSlot u1) = CoreStorageLayout.poolTickFeesPerLiquidityOutsideSlot(poolId, tickUpper);
            (upper0, upper1) = (uint256(u0.load()), uint256(u1.load()));
        }

        unchecked {
            if (tick < tickLower) {
                feesPerLiquidityInside.value0 = lower0 - upper0;
                feesPerLiquidityInside.value1 = lower1 - upper1;
            } else if (tick < tickUpper) {
                uint256 global0;
                uint256 global1;
                {
                    (bytes32 g0, bytes32 g1) = CoreStorageLayout.poolFeesPerLiquiditySlot(poolId).loadTwo();
                    (global0, global1) = (uint256(g0), uint256(g1));
                }

                feesPerLiquidityInside.value0 = global0 - upper0 - lower0;
                feesPerLiquidityInside.value1 = global1 - upper1 - lower1;
            } else {
                feesPerLiquidityInside.value0 = upper0 - lower0;
                feesPerLiquidityInside.value1 = upper1 - lower1;
            }
        }
    }

    /// @inheritdoc ICore
    function getPoolFeesPerLiquidityInside(PoolId poolId, int32 tickLower, int32 tickUpper)
        external
        view
        returns (FeesPerLiquidity memory)
    {
        return _getPoolFeesPerLiquidityInside(poolId, readPoolState(poolId).tick(), tickLower, tickUpper);
    }

    /// @inheritdoc ICore
    function accumulateAsFees(PoolKey memory poolKey, uint128 _amount0, uint128 _amount1) external payable {
        (uint256 id, address lockerAddr) = _requireLocker().parse();
        require(lockerAddr == poolKey.config.extension());

        PoolId poolId = poolKey.toPoolId();

        uint256 amount0;
        uint256 amount1;
        assembly ("memory-safe") {
            amount0 := _amount0
            amount1 := _amount1
        }

        // Note we do not check pool is initialized. If the extension calls this for a pool that does not exist,
        //  the fees are simply burned since liquidity is 0.

        if (amount0 != 0 || amount1 != 0) {
            uint256 liquidity;
            {
                uint128 _liquidity = readPoolState(poolId).liquidity();
                assembly ("memory-safe") {
                    liquidity := _liquidity
                }
            }

            unchecked {
                if (liquidity != 0) {
                    StorageSlot slot0 = CoreStorageLayout.poolFeesPerLiquiditySlot(poolId);

                    if (amount0 != 0) {
                        slot0.store(
                            bytes32(uint256(slot0.load()) + FixedPointMathLib.rawDiv(amount0 << 128, liquidity))
                        );
                    }
                    if (amount1 != 0) {
                        StorageSlot slot1 = slot0.next();
                        slot1.store(
                            bytes32(uint256(slot1.load()) + FixedPointMathLib.rawDiv(amount1 << 128, liquidity))
                        );
                    }
                }
            }
        }

        // whether the fees are actually accounted to any position, the caller owes the debt
        _updatePairDebtWithNative(id, poolKey.token0, poolKey.token1, int256(amount0), int256(amount1));

        emit FeesAccumulated(poolId, _amount0, _amount1);
    }

    /// @notice Updates tick information when liquidity is added or removed
    /// @dev Private function that handles tick initialization and liquidity tracking
    /// @param poolId Unique identifier for the pool
    /// @param tick Tick to update
    /// @param poolConfig Pool configuration containing tick spacing
    /// @param liquidityDelta Change in liquidity
    /// @param isUpper Whether this is the upper bound of a position
    function _updateTick(PoolId poolId, int32 tick, PoolConfig poolConfig, int128 liquidityDelta, bool isUpper)
        private
    {
        StorageSlot tickInfoSlot = CoreStorageLayout.poolTicksSlot(poolId, tick);

        (int128 currentLiquidityDelta, uint128 currentLiquidityNet) = TickInfo.wrap(tickInfoSlot.load()).parse();
        uint128 liquidityNetNext = addLiquidityDelta(currentLiquidityNet, liquidityDelta);
        // this is checked math
        int128 liquidityDeltaNext =
            isUpper ? currentLiquidityDelta - liquidityDelta : currentLiquidityDelta + liquidityDelta;

        // Check that liquidityNet doesn't exceed max liquidity per tick
        uint128 maxLiquidity = poolConfig.concentratedMaxLiquidityPerTick();
        if (liquidityNetNext > maxLiquidity) {
            revert MaxLiquidityPerTickExceeded(tick, liquidityNetNext, maxLiquidity);
        }

        if ((currentLiquidityNet == 0) != (liquidityNetNext == 0)) {
            flipTick(CoreStorageLayout.tickBitmapsSlot(poolId), tick, poolConfig.concentratedTickSpacing());

            (StorageSlot fplSlot0, StorageSlot fplSlot1) =
                CoreStorageLayout.poolTickFeesPerLiquidityOutsideSlot(poolId, tick);

            bytes32 v;
            assembly ("memory-safe") {
                v := gt(liquidityNetNext, 0)
            }

            // initialize the storage slots for the fees per liquidity outside to non-zero so tick crossing is cheaper
            fplSlot0.store(v);
            fplSlot1.store(v);
        }

        tickInfoSlot.store(TickInfo.unwrap(createTickInfo(liquidityDeltaNext, liquidityNetNext)));
    }

    /// @notice Updates debt for a token pair, handling native token payments for token0
    /// @dev Optimized version that updates both tokens' debts in a single operation when possible.
    ///      Assumes token0 < token1 (tokens are sorted).
    /// @param id Lock ID for debt tracking
    /// @param token0 Address of token0 (must be < token1)
    /// @param token1 Address of token1 (must be > token0)
    /// @param debtChange0 Change in debt amount for token0
    /// @param debtChange1 Change in debt amount for token1
    function _updatePairDebtWithNative(
        uint256 id,
        address token0,
        address token1,
        int256 debtChange0,
        int256 debtChange1
    ) private {
        if (msg.value == 0) {
            // No native token payment included in the call, so use optimized pair update
            _updatePairDebt(id, token0, token1, debtChange0, debtChange1);
        } else {
            if (token0 == NATIVE_TOKEN_ADDRESS) {
                unchecked {
                    // token0 is native, so we can still use pair update with adjusted debtChange0
                    // Subtraction is safe because debtChange0 and msg.value are both bounded by int128/uint128
                    _updatePairDebt(id, token0, token1, debtChange0 - int256(msg.value), debtChange1);
                }
            } else {
                // token0 is not native, and since token0 < token1, token1 cannot be native either
                // Update the token0, token1 debt and then update native token debt separately
                unchecked {
                    _updatePairDebt(id, token0, token1, debtChange0, debtChange1);
                    _accountDebt(id, NATIVE_TOKEN_ADDRESS, -int256(msg.value));
                }
            }
        }
    }

    /// @inheritdoc ICore
    function updatePosition(PoolKey memory poolKey, PositionId positionId, int128 liquidityDelta)
        external
        payable
        returns (PoolBalanceUpdate balanceUpdate)
    {
        positionId.validate(poolKey.config);

        Locker locker = _requireLocker();

        IExtension(poolKey.config.extension())
            .maybeCallBeforeUpdatePosition(locker, poolKey, positionId, liquidityDelta);

        PoolId poolId = poolKey.toPoolId();
        PoolState state = readPoolState(poolId);
        if (!state.isInitialized()) revert PoolNotInitialized();

        if (liquidityDelta != 0) {
            (SqrtRatio sqrtRatioLower, SqrtRatio sqrtRatioUpper) =
                (tickToSqrtRatio(positionId.tickLower()), tickToSqrtRatio(positionId.tickUpper()));

            (int128 delta0, int128 delta1) =
                liquidityDeltaToAmountDelta(state.sqrtRatio(), liquidityDelta, sqrtRatioLower, sqrtRatioUpper);

            StorageSlot positionSlot = CoreStorageLayout.poolPositionsSlot(poolId, locker.addr(), positionId);
            Position storage position;
            assembly ("memory-safe") {
                position.slot := positionSlot
            }

            uint128 liquidityNext = addLiquidityDelta(position.liquidity, liquidityDelta);

            FeesPerLiquidity memory feesPerLiquidityInside;

            if (poolKey.config.isConcentrated()) {
                // the position is fully withdrawn
                if (liquidityNext == 0) {
                    // we need to fetch it before the tick fees per liquidity outside is deleted
                    feesPerLiquidityInside = _getPoolFeesPerLiquidityInside(
                        poolId, state.tick(), positionId.tickLower(), positionId.tickUpper()
                    );
                }

                _updateTick(poolId, positionId.tickLower(), poolKey.config, liquidityDelta, false);
                _updateTick(poolId, positionId.tickUpper(), poolKey.config, liquidityDelta, true);

                if (liquidityNext != 0) {
                    feesPerLiquidityInside = _getPoolFeesPerLiquidityInside(
                        poolId, state.tick(), positionId.tickLower(), positionId.tickUpper()
                    );
                }

                if (state.tick() >= positionId.tickLower() && state.tick() < positionId.tickUpper()) {
                    state = createPoolState({
                        _sqrtRatio: state.sqrtRatio(),
                        _tick: state.tick(),
                        _liquidity: addLiquidityDelta(state.liquidity(), liquidityDelta)
                    });
                    writePoolState(poolId, state);
                }
            } else {
                // we store the active liquidity in the liquidity slot for stableswap pools
                state = createPoolState({
                    _sqrtRatio: state.sqrtRatio(),
                    _tick: state.tick(),
                    _liquidity: addLiquidityDelta(state.liquidity(), liquidityDelta)
                });
                writePoolState(poolId, state);
                StorageSlot fplFirstSlot = CoreStorageLayout.poolFeesPerLiquiditySlot(poolId);
                feesPerLiquidityInside.value0 = uint256(fplFirstSlot.load());
                feesPerLiquidityInside.value1 = uint256(fplFirstSlot.next().load());
            }

            if (liquidityNext == 0) {
                position.liquidity = 0;
                position.feesPerLiquidityInsideLast = FeesPerLiquidity(0, 0);
            } else {
                (uint128 fees0, uint128 fees1) = position.fees(feesPerLiquidityInside);
                position.liquidity = liquidityNext;
                position.feesPerLiquidityInsideLast =
                    feesPerLiquidityInside.sub(feesPerLiquidityFromAmounts(fees0, fees1, liquidityNext));
            }

            _updatePairDebtWithNative(locker.id(), poolKey.token0, poolKey.token1, delta0, delta1);

            balanceUpdate = createPoolBalanceUpdate(delta0, delta1);
            emit PositionUpdated(locker.addr(), poolId, positionId, liquidityDelta, balanceUpdate, state);
        }

        IExtension(poolKey.config.extension())
            .maybeCallAfterUpdatePosition(locker, poolKey, positionId, liquidityDelta, balanceUpdate, state);
    }

    /// @inheritdoc ICore
    function setExtraData(PoolId poolId, PositionId positionId, bytes16 _extraData) external {
        StorageSlot firstSlot = CoreStorageLayout.poolPositionsSlot(poolId, msg.sender, positionId);

        bytes32 extraData;
        assembly ("memory-safe") {
            extraData := _extraData
        }

        firstSlot.store(((firstSlot.load() >> 128) << 128) | (extraData >> 128));
    }

    /// @inheritdoc ICore
    function collectFees(PoolKey memory poolKey, PositionId positionId)
        external
        returns (uint128 amount0, uint128 amount1)
    {
        Locker locker = _requireLocker();

        IExtension(poolKey.config.extension()).maybeCallBeforeCollectFees(locker, poolKey, positionId);

        PoolId poolId = poolKey.toPoolId();

        Position storage position;
        StorageSlot positionSlot = CoreStorageLayout.poolPositionsSlot(poolId, locker.addr(), positionId);
        assembly ("memory-safe") {
            position.slot := positionSlot
        }

        FeesPerLiquidity memory feesPerLiquidityInside;
        if (poolKey.config.isStableswap()) {
            // Stableswap pools: use global fees per liquidity
            StorageSlot fplFirstSlot = CoreStorageLayout.poolFeesPerLiquiditySlot(poolId);
            feesPerLiquidityInside.value0 = uint256(fplFirstSlot.load());
            feesPerLiquidityInside.value1 = uint256(fplFirstSlot.next().load());
        } else {
            // Concentrated pools: calculate fees per liquidity inside the position bounds
            feesPerLiquidityInside = _getPoolFeesPerLiquidityInside(
                poolId, readPoolState(poolId).tick(), positionId.tickLower(), positionId.tickUpper()
            );
        }

        (amount0, amount1) = position.fees(feesPerLiquidityInside);

        position.feesPerLiquidityInsideLast = feesPerLiquidityInside;

        _updatePairDebt(
            locker.id(), poolKey.token0, poolKey.token1, -int256(uint256(amount0)), -int256(uint256(amount1))
        );

        emit PositionFeesCollected(locker.addr(), poolId, positionId, amount0, amount1);

        IExtension(poolKey.config.extension()).maybeCallAfterCollectFees(locker, poolKey, positionId, amount0, amount1);
    }

    /// @inheritdoc ICore
    function swap_6269342730() external payable {
        unchecked {
            PoolKey memory poolKey;
            address token0;
            address token1;
            PoolConfig config;

            SwapParameters params;

            assembly ("memory-safe") {
                token0 := calldataload(4)
                token1 := calldataload(36)
                config := calldataload(68)
                params := calldataload(100)
                calldatacopy(poolKey, 4, 96)
            }

            SqrtRatio sqrtRatioLimit = params.sqrtRatioLimit();
            if (!sqrtRatioLimit.isValid()) revert InvalidSqrtRatioLimit();

            Locker locker = _requireLocker();

            IExtension(config.extension()).maybeCallBeforeSwap(locker, poolKey, params);

            PoolId poolId = poolKey.toPoolId();

            PoolState stateAfter = readPoolState(poolId);

            if (!stateAfter.isInitialized()) revert PoolNotInitialized();

            int256 amountRemaining = params.amount();

            PoolBalanceUpdate balanceUpdate;

            // 0 swap amount or sqrt ratio limit == sqrt ratio is no-op
            if (amountRemaining != 0 && stateAfter.sqrtRatio() != sqrtRatioLimit) {
                (SqrtRatio sqrtRatio, int32 tick, uint128 liquidity) = stateAfter.parse();

                bool isToken1 = params.isToken1();

                bool isExactOut = amountRemaining < 0;
                bool increasing;
                assembly ("memory-safe") {
                    increasing := xor(isToken1, isExactOut)
                }

                if ((sqrtRatioLimit < sqrtRatio) == increasing) {
                    revert SqrtRatioLimitWrongDirection();
                }

                int256 calculatedAmount;

                // fees per liquidity only for the input token
                uint256 inputTokenFeesPerLiquidity;

                // 0 = not loaded & not updated, 1 = loaded & not updated, 2 = loaded & updated
                uint256 feesAccessed;

                while (true) {
                    int32 nextTick;
                    bool isInitialized;
                    SqrtRatio nextTickSqrtRatio;

                    // For stableswap pools, determine active liquidity for this step
                    uint128 stepLiquidity = liquidity;

                    if (config.isStableswap()) {
                        if (config.isFullRange()) {
                            // special case since we don't need to compute min/max tick sqrt ratio
                            (nextTick, nextTickSqrtRatio) =
                                increasing ? (MAX_TICK, MAX_SQRT_RATIO) : (MIN_TICK, MIN_SQRT_RATIO);
                        } else {
                            (int32 lower, int32 upper) = config.stableswapActiveLiquidityTickRange();

                            bool inRange;
                            assembly ("memory-safe") {
                                inRange := and(slt(tick, upper), iszero(slt(tick, lower)))
                            }
                            if (inRange) {
                                nextTick = increasing ? upper : lower;
                                nextTickSqrtRatio = tickToSqrtRatio(nextTick);
                            } else {
                                if (tick < lower) {
                                    (nextTick, nextTickSqrtRatio) =
                                        increasing ? (lower, tickToSqrtRatio(lower)) : (MIN_TICK, MIN_SQRT_RATIO);
                                } else {
                                    // tick >= upper implied
                                    (nextTick, nextTickSqrtRatio) =
                                        increasing ? (MAX_TICK, MAX_SQRT_RATIO) : (upper, tickToSqrtRatio(upper));
                                }
                                stepLiquidity = 0;
                            }
                        }
                    } else {
                        // concentrated liquidity pools use the tick bitmaps
                        (nextTick, isInitialized) = increasing
                            ? findNextInitializedTick(
                                CoreStorageLayout.tickBitmapsSlot(poolId),
                                tick,
                                config.concentratedTickSpacing(),
                                params.skipAhead()
                            )
                            : findPrevInitializedTick(
                                CoreStorageLayout.tickBitmapsSlot(poolId),
                                tick,
                                config.concentratedTickSpacing(),
                                params.skipAhead()
                            );

                        nextTickSqrtRatio = tickToSqrtRatio(nextTick);
                    }

                    SqrtRatio limitedNextSqrtRatio =
                        increasing ? nextTickSqrtRatio.min(sqrtRatioLimit) : nextTickSqrtRatio.max(sqrtRatioLimit);

                    SqrtRatio sqrtRatioNext;

                    if (stepLiquidity == 0) {
                        // if the pool is empty, the swap will always move all the way to the limit price
                        sqrtRatioNext = limitedNextSqrtRatio;
                    } else {
                        // this amount is what moves the price
                        int128 priceImpactAmount;
                        if (isExactOut) {
                            assembly ("memory-safe") {
                                priceImpactAmount := amountRemaining
                            }
                        } else {
                            uint128 amountU128;
                            assembly ("memory-safe") {
                                // cast is safe because amountRemaining is g.t. 0 and fits in int128
                                amountU128 := amountRemaining
                            }
                            uint128 feeAmount = computeFee(amountU128, config.fee());
                            assembly ("memory-safe") {
                                // feeAmount will never exceed amountRemaining since fee is < 100%
                                priceImpactAmount := sub(amountRemaining, feeAmount)
                            }
                        }

                        SqrtRatio sqrtRatioNextFromAmount = isToken1
                            ? nextSqrtRatioFromAmount1(sqrtRatio, stepLiquidity, priceImpactAmount)
                            : nextSqrtRatioFromAmount0(sqrtRatio, stepLiquidity, priceImpactAmount);

                        bool hitLimit;
                        assembly ("memory-safe") {
                            // Branchless limit check: (increasing && next > limit) || (!increasing && next < limit)
                            let exceedsUp := and(increasing, gt(sqrtRatioNextFromAmount, limitedNextSqrtRatio))
                            let exceedsDown :=
                                and(iszero(increasing), lt(sqrtRatioNextFromAmount, limitedNextSqrtRatio))
                            hitLimit := or(exceedsUp, exceedsDown)
                        }

                        // the change in fees per liquidity for this step of the iteration
                        uint256 stepFeesPerLiquidity;

                        if (hitLimit) {
                            (uint256 sqrtRatioLower, uint256 sqrtRatioUpper) =
                                sortAndConvertToFixedSqrtRatios(limitedNextSqrtRatio, sqrtRatio);
                            (uint128 limitSpecifiedAmountDelta, uint128 limitCalculatedAmountDelta) = isToken1
                                ? (
                                    amount1DeltaSorted(sqrtRatioLower, sqrtRatioUpper, stepLiquidity, !isExactOut),
                                    amount0DeltaSorted(sqrtRatioLower, sqrtRatioUpper, stepLiquidity, isExactOut)
                                )
                                : (
                                    amount0DeltaSorted(sqrtRatioLower, sqrtRatioUpper, stepLiquidity, !isExactOut),
                                    amount1DeltaSorted(sqrtRatioLower, sqrtRatioUpper, stepLiquidity, isExactOut)
                                );

                            if (isExactOut) {
                                uint128 beforeFee = amountBeforeFee(limitCalculatedAmountDelta, config.fee());
                                assembly ("memory-safe") {
                                    calculatedAmount := add(calculatedAmount, beforeFee)
                                    amountRemaining := add(amountRemaining, limitSpecifiedAmountDelta)
                                    stepFeesPerLiquidity := div(
                                        shl(128, sub(beforeFee, limitCalculatedAmountDelta)),
                                        stepLiquidity
                                    )
                                }
                            } else {
                                uint128 beforeFee = amountBeforeFee(limitSpecifiedAmountDelta, config.fee());
                                assembly ("memory-safe") {
                                    calculatedAmount := sub(calculatedAmount, limitCalculatedAmountDelta)
                                    amountRemaining := sub(amountRemaining, beforeFee)
                                    stepFeesPerLiquidity := div(
                                        shl(128, sub(beforeFee, limitSpecifiedAmountDelta)),
                                        stepLiquidity
                                    )
                                }
                            }

                            sqrtRatioNext = limitedNextSqrtRatio;
                        } else if (sqrtRatioNextFromAmount != sqrtRatio) {
                            uint128 calculatedAmountWithoutFee = isToken1
                                ? amount0Delta(sqrtRatioNextFromAmount, sqrtRatio, stepLiquidity, isExactOut)
                                : amount1Delta(sqrtRatioNextFromAmount, sqrtRatio, stepLiquidity, isExactOut);

                            if (isExactOut) {
                                uint128 includingFee = amountBeforeFee(calculatedAmountWithoutFee, config.fee());
                                assembly ("memory-safe") {
                                    calculatedAmount := add(calculatedAmount, includingFee)
                                    stepFeesPerLiquidity := div(
                                        shl(128, sub(includingFee, calculatedAmountWithoutFee)),
                                        stepLiquidity
                                    )
                                }
                            } else {
                                assembly ("memory-safe") {
                                    calculatedAmount := sub(calculatedAmount, calculatedAmountWithoutFee)
                                    stepFeesPerLiquidity := div(
                                        shl(128, sub(amountRemaining, priceImpactAmount)),
                                        stepLiquidity
                                    )
                                }
                            }

                            amountRemaining = 0;
                            sqrtRatioNext = sqrtRatioNextFromAmount;
                        } else {
                            // for an exact output swap, the price should always move since we have to round away from the current price
                            assert(!isExactOut);

                            // consume the entire input amount as fees since the price did not move
                            assembly ("memory-safe") {
                                stepFeesPerLiquidity := div(shl(128, amountRemaining), stepLiquidity)
                            }
                            amountRemaining = 0;
                            sqrtRatioNext = sqrtRatio;
                        }

                        // only if fees per liquidity was updated in this swap iteration
                        if (stepFeesPerLiquidity != 0) {
                            if (feesAccessed == 0) {
                                // this loads only the input token fees per liquidity
                                inputTokenFeesPerLiquidity = uint256(
                                    CoreStorageLayout.poolFeesPerLiquiditySlot(poolId).add(LibBit.rawToUint(increasing))
                                        .load()
                                ) + stepFeesPerLiquidity;
                            } else {
                                inputTokenFeesPerLiquidity += stepFeesPerLiquidity;
                            }

                            feesAccessed = 2;
                        }
                    }

                    if (sqrtRatioNext == nextTickSqrtRatio) {
                        sqrtRatio = sqrtRatioNext;
                        assembly ("memory-safe") {
                            // no overflow danger because nextTick is always inside the valid tick bounds
                            tick := sub(nextTick, iszero(increasing))
                        }

                        if (isInitialized) {
                            bytes32 tickValue = CoreStorageLayout.poolTicksSlot(poolId, nextTick).load();
                            assembly ("memory-safe") {
                                // if increasing, we add the liquidity delta, otherwise we subtract it
                                let liquidityDelta :=
                                    mul(signextend(15, tickValue), sub(increasing, iszero(increasing)))
                                liquidity := add(liquidity, liquidityDelta)
                            }

                            (StorageSlot tickFplFirstSlot, StorageSlot tickFplSecondSlot) =
                                CoreStorageLayout.poolTickFeesPerLiquidityOutsideSlot(poolId, nextTick);

                            if (feesAccessed == 0) {
                                inputTokenFeesPerLiquidity = uint256(
                                    CoreStorageLayout.poolFeesPerLiquiditySlot(poolId).add(LibBit.rawToUint(increasing))
                                        .load()
                                );
                                feesAccessed = 1;
                            }

                            uint256 globalFeesPerLiquidityOther = uint256(
                                CoreStorageLayout.poolFeesPerLiquiditySlot(poolId).add(LibBit.rawToUint(!increasing))
                                    .load()
                            );

                            // if increasing, it means the pool is receiving token1 so the input fees per liquidity is token1
                            if (increasing) {
                                tickFplFirstSlot.store(
                                    bytes32(globalFeesPerLiquidityOther - uint256(tickFplFirstSlot.load()))
                                );
                                tickFplSecondSlot.store(
                                    bytes32(inputTokenFeesPerLiquidity - uint256(tickFplSecondSlot.load()))
                                );
                            } else {
                                tickFplFirstSlot.store(
                                    bytes32(inputTokenFeesPerLiquidity - uint256(tickFplFirstSlot.load()))
                                );
                                tickFplSecondSlot.store(
                                    bytes32(globalFeesPerLiquidityOther - uint256(tickFplSecondSlot.load()))
                                );
                            }
                        }
                    } else if (sqrtRatio != sqrtRatioNext) {
                        sqrtRatio = sqrtRatioNext;
                        tick = sqrtRatioToTick(sqrtRatio);
                    }

                    if (amountRemaining == 0 || sqrtRatio == sqrtRatioLimit) {
                        break;
                    }
                }

                int128 calculatedAmountDelta =
                    SafeCastLib.toInt128(FixedPointMathLib.max(type(int128).min, calculatedAmount));

                int128 specifiedAmountDelta;
                int128 specifiedAmount = params.amount();
                assembly ("memory-safe") {
                    specifiedAmountDelta := sub(specifiedAmount, amountRemaining)
                }

                balanceUpdate = isToken1
                    ? createPoolBalanceUpdate(calculatedAmountDelta, specifiedAmountDelta)
                    : createPoolBalanceUpdate(specifiedAmountDelta, calculatedAmountDelta);

                stateAfter = createPoolState({_sqrtRatio: sqrtRatio, _tick: tick, _liquidity: liquidity});

                writePoolState(poolId, stateAfter);

                if (feesAccessed == 2) {
                    // this stores only the input token fees per liquidity
                    CoreStorageLayout.poolFeesPerLiquiditySlot(poolId).add(LibBit.rawToUint(increasing))
                        .store(bytes32(inputTokenFeesPerLiquidity));
                }

                _updatePairDebtWithNative(locker.id(), token0, token1, balanceUpdate.delta0(), balanceUpdate.delta1());

                assembly ("memory-safe") {
                    let o := mload(0x40)
                    mstore(o, shl(96, locker))
                    mstore(add(o, 20), poolId)
                    mstore(add(o, 52), balanceUpdate)
                    mstore(add(o, 84), stateAfter)
                    log0(o, 116)
                }
            }

            IExtension(config.extension()).maybeCallAfterSwap(locker, poolKey, params, balanceUpdate, stateAfter);

            assembly ("memory-safe") {
                mstore(0, balanceUpdate)
                mstore(32, stateAfter)
                return(0, 64)
            }
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {OrderKey} from "../types/orderKey.sol";
import {IBaseNonfungibleToken} from "./IBaseNonfungibleToken.sol";

/// @title Orders Interface
/// @notice Interface for managing TWAMM (Time-Weighted Average Market Maker) orders as NFTs
/// @dev Defines the interface for creating, modifying, and collecting proceeds from long-term orders
interface IOrders is IBaseNonfungibleToken {
    /// @notice Thrown when trying to modify an order that has already ended
    error OrderAlreadyEnded();

    /// @notice Thrown when the calculated sale rate exceeds the maximum allowed
    error MaxSaleRateExceeded();

    /// @notice Mints a new NFT and creates a TWAMM order
    /// @param orderKey Key identifying the order parameters
    /// @param amount Amount of tokens to sell over the order duration
    /// @param maxSaleRate Maximum acceptable sale rate (for slippage protection)
    /// @return id The newly minted NFT token ID
    /// @return saleRate The calculated sale rate for the order
    function mintAndIncreaseSellAmount(OrderKey memory orderKey, uint112 amount, uint112 maxSaleRate)
        external
        payable
        returns (uint256 id, uint112 saleRate);

    /// @notice Increases the sell amount for an existing TWAMM order
    /// @param id The NFT token ID representing the order
    /// @param orderKey Key identifying the order parameters
    /// @param amount Additional amount of tokens to sell
    /// @param maxSaleRate Maximum acceptable sale rate (for slippage protection)
    /// @return saleRate The calculated sale rate for the additional amount
    function increaseSellAmount(uint256 id, OrderKey memory orderKey, uint128 amount, uint112 maxSaleRate)
        external
        payable
        returns (uint112 saleRate);

    /// @notice Decreases the sale rate for an existing TWAMM order
    /// @param id The NFT token ID representing the order
    /// @param orderKey Key identifying the order parameters
    /// @param saleRateDecrease Amount to decrease the sale rate by
    /// @param recipient Address to receive the refunded tokens
    /// @return refund Amount of tokens refunded
    function decreaseSaleRate(uint256 id, OrderKey memory orderKey, uint112 saleRateDecrease, address recipient)
        external
        payable
        returns (uint112 refund);

    /// @notice Decreases the sale rate for an existing TWAMM order (refund to msg.sender)
    /// @param id The NFT token ID representing the order
    /// @param orderKey Key identifying the order parameters
    /// @param saleRateDecrease Amount to decrease the sale rate by
    /// @return refund Amount of tokens refunded
    function decreaseSaleRate(uint256 id, OrderKey memory orderKey, uint112 saleRateDecrease)
        external
        payable
        returns (uint112 refund);

    /// @notice Collects the proceeds from a TWAMM order
    /// @param id The NFT token ID representing the order
    /// @param orderKey Key identifying the order parameters
    /// @param recipient Address to receive the proceeds
    /// @return proceeds Amount of tokens collected as proceeds
    function collectProceeds(uint256 id, OrderKey memory orderKey, address recipient)
        external
        payable
        returns (uint128 proceeds);

    /// @notice Collects the proceeds from a TWAMM order (to msg.sender)
    /// @param id The NFT token ID representing the order
    /// @param orderKey Key identifying the order parameters
    /// @return proceeds Amount of tokens collected as proceeds
    function collectProceeds(uint256 id, OrderKey memory orderKey) external payable returns (uint128 proceeds);

    /// @notice Executes virtual orders and returns current order information
    /// @dev Updates the order state by executing any pending virtual orders
    /// @param id The NFT token ID representing the order
    /// @param orderKey Key identifying the order parameters
    /// @return saleRate Current sale rate of the order
    /// @return amountSold Total amount sold so far
    /// @return remainingSellAmount Amount remaining to be sold
    /// @return purchasedAmount Amount of tokens purchased (proceeds available)
    function executeVirtualOrdersAndGetCurrentOrderInfo(uint256 id, OrderKey memory orderKey)
        external
        returns (uint112 saleRate, uint256 amountSold, uint256 remainingSellAmount, uint128 purchasedAmount);
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {BasePositions} from "./base/BasePositions.sol";
import {ICore} from "./interfaces/ICore.sol";
import {PoolKey} from "./types/poolKey.sol";
import {computeFee} from "./math/fee.sol";

/// @title Ekubo Protocol Positions
/// @author Moody Salem <moody@ekubo.org>
/// @notice Tracks liquidity positions in Ekubo Protocol as NFTs
/// @dev Manages liquidity positions, fee collection, and protocol fees
contract Positions is BasePositions {
    /// @notice Protocol fee rate for swaps (as a fraction of 2^64)
    uint64 public immutable SWAP_PROTOCOL_FEE_X64;

    /// @notice Denominator for withdrawal protocol fee calculation
    uint64 public immutable WITHDRAWAL_PROTOCOL_FEE_DENOMINATOR;

    /// @notice Constructs the Positions contract
    /// @param core The core contract instance
    /// @param owner The owner of the contract (for access control)
    /// @param _swapProtocolFeeX64 Protocol fee rate for swaps
    /// @param _withdrawalProtocolFeeDenominator Denominator for withdrawal protocol fee
    constructor(ICore core, address owner, uint64 _swapProtocolFeeX64, uint64 _withdrawalProtocolFeeDenominator)
        BasePositions(core, owner)
    {
        SWAP_PROTOCOL_FEE_X64 = _swapProtocolFeeX64;
        WITHDRAWAL_PROTOCOL_FEE_DENOMINATOR = _withdrawalProtocolFeeDenominator;
    }

    /// @notice Handles protocol fee collection during fee collection
    /// @dev Implements the abstract method from BasePositions
    /// @param amount0 The amount of token0 fees collected before protocol fee deduction
    /// @param amount1 The amount of token1 fees collected before protocol fee deduction
    /// @return protocolFee0 The amount of token0 protocol fees to collect
    /// @return protocolFee1 The amount of token1 protocol fees to collect
    function _computeSwapProtocolFees(PoolKey memory, uint128 amount0, uint128 amount1)
        internal
        view
        override
        returns (uint128 protocolFee0, uint128 protocolFee1)
    {
        if (SWAP_PROTOCOL_FEE_X64 != 0) {
            protocolFee0 = computeFee(amount0, SWAP_PROTOCOL_FEE_X64);
            protocolFee1 = computeFee(amount1, SWAP_PROTOCOL_FEE_X64);
        }
    }

    /// @notice Handles protocol fee collection during liquidity withdrawal
    /// @dev Implements the abstract method from BasePositions
    /// @param poolKey The pool key for the position
    /// @param amount0 The amount of token0 being withdrawn before protocol fee deduction
    /// @param amount1 The amount of token1 being withdrawn before protocol fee deduction
    /// @return protocolFee0 The amount of token0 protocol fees to collect
    /// @return protocolFee1 The amount of token1 protocol fees to collect
    function _computeWithdrawalProtocolFees(PoolKey memory poolKey, uint128 amount0, uint128 amount1)
        internal
        view
        override
        returns (uint128 protocolFee0, uint128 protocolFee1)
    {
        uint64 fee = poolKey.config.fee();
        if (fee != 0 && WITHDRAWAL_PROTOCOL_FEE_DENOMINATOR != 0) {
            protocolFee0 = computeFee(amount0, fee / WITHDRAWAL_PROTOCOL_FEE_DENOMINATOR);
            protocolFee1 = computeFee(amount1, fee / WITHDRAWAL_PROTOCOL_FEE_DENOMINATOR);
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {PoolKey} from "../types/poolKey.sol";
import {SqrtRatio} from "../types/sqrtRatio.sol";
import {IBaseNonfungibleToken} from "./IBaseNonfungibleToken.sol";

/// @title Positions Interface
/// @notice Interface for managing liquidity positions as NFTs in Ekubo Protocol
/// @dev Defines the interface for depositing, withdrawing, and collecting fees from liquidity positions
interface IPositions is IBaseNonfungibleToken {
    /// @notice Thrown when deposit fails due to insufficient liquidity for the given slippage tolerance
    /// @param liquidity The actual liquidity that would be provided
    /// @param minLiquidity The minimum liquidity required
    error DepositFailedDueToSlippage(uint128 liquidity, uint128 minLiquidity);

    /// @notice Thrown when deposit amount would cause overflow
    error DepositOverflow();

    /// @notice Thrown when the specified withdraw liquidity amount overflows type(int128).max
    error WithdrawOverflow();

    /// @notice Gets the liquidity, principal amounts, and accumulated fees for a position
    /// @param id The NFT token ID representing the position
    /// @param poolKey Pool key identifying the pool
    /// @param tickLower Lower tick of the price range of the position
    /// @param tickUpper Upper tick of the price range of the position
    /// @return liquidity Current liquidity in the position
    /// @return principal0 Principal amount of token0 in the position
    /// @return principal1 Principal amount of token1 in the position
    /// @return fees0 Accumulated fees in token0
    /// @return fees1 Accumulated fees in token1
    function getPositionFeesAndLiquidity(uint256 id, PoolKey memory poolKey, int32 tickLower, int32 tickUpper)
        external
        view
        returns (uint128 liquidity, uint128 principal0, uint128 principal1, uint128 fees0, uint128 fees1);

    /// @notice Deposits tokens into a liquidity position
    /// @param id The NFT token ID representing the position
    /// @param poolKey Pool key identifying the pool
    /// @param tickLower Lower tick of the price range of the position
    /// @param tickUpper Upper tick of the price range of the position
    /// @param maxAmount0 Maximum amount of token0 to deposit
    /// @param maxAmount1 Maximum amount of token1 to deposit
    /// @param minLiquidity Minimum liquidity to receive (for slippage protection)
    /// @return liquidity Amount of liquidity added to the position
    /// @return amount0 Actual amount of token0 deposited
    /// @return amount1 Actual amount of token1 deposited
    function deposit(
        uint256 id,
        PoolKey memory poolKey,
        int32 tickLower,
        int32 tickUpper,
        uint128 maxAmount0,
        uint128 maxAmount1,
        uint128 minLiquidity
    ) external payable returns (uint128 liquidity, uint128 amount0, uint128 amount1);

    /// @notice Collects accumulated fees from a position to msg.sender
    /// @param id The NFT token ID representing the position
    /// @param poolKey Pool key identifying the pool
    /// @param tickLower Lower tick of the price range of the position
    /// @param tickUpper Upper tick of the price range of the position
    /// @return amount0 Amount of token0 fees collected
    /// @return amount1 Amount of token1 fees collected
    function collectFees(uint256 id, PoolKey memory poolKey, int32 tickLower, int32 tickUpper)
        external
        payable
        returns (uint128 amount0, uint128 amount1);

    /// @notice Collects accumulated fees from a position to a specified recipient
    /// @param id The NFT token ID representing the position
    /// @param poolKey Pool key identifying the pool
    /// @param tickLower Lower tick of the price range of the position
    /// @param tickUpper Upper tick of the price range of the position
    /// @param recipient Address to receive the collected fees
    /// @return amount0 Amount of token0 fees collected
    /// @return amount1 Amount of token1 fees collected
    function collectFees(uint256 id, PoolKey memory poolKey, int32 tickLower, int32 tickUpper, address recipient)
        external
        payable
        returns (uint128 amount0, uint128 amount1);

    /// @notice Withdraws liquidity from a position
    /// @param id The NFT token ID representing the position
    /// @param poolKey Pool key identifying the pool
    /// @param tickLower Lower tick of the price range of the position
    /// @param tickUpper Upper tick of the price range of the position
    /// @param liquidity Amount of liquidity to withdraw
    /// @param recipient Address to receive the withdrawn tokens
    /// @param withFees Whether to also collect accumulated fees
    /// @return amount0 Amount of token0 withdrawn
    /// @return amount1 Amount of token1 withdrawn
    function withdraw(
        uint256 id,
        PoolKey memory poolKey,
        int32 tickLower,
        int32 tickUpper,
        uint128 liquidity,
        address recipient,
        bool withFees
    ) external payable returns (uint128 amount0, uint128 amount1);

    /// @notice Withdraws liquidity from a position to msg.sender with fees
    /// @param id The NFT token ID representing the position
    /// @param poolKey Pool key identifying the pool
    /// @param tickLower Lower tick of the price range of the position
    /// @param tickUpper Upper tick of the price range of the position
    /// @param liquidity Amount of liquidity to withdraw
    /// @return amount0 Amount of token0 withdrawn
    /// @return amount1 Amount of token1 withdrawn
    function withdraw(uint256 id, PoolKey memory poolKey, int32 tickLower, int32 tickUpper, uint128 liquidity)
        external
        payable
        returns (uint128 amount0, uint128 amount1);

    /// @notice Initializes a pool if it hasn't been initialized yet
    /// @param poolKey Pool key identifying the pool
    /// @param tick Initial tick for the pool if initialization is needed
    /// @return initialized Whether the pool was initialized by this call
    /// @return sqrtRatio The sqrt price ratio of the pool (existing or newly set)
    function maybeInitializePool(PoolKey memory poolKey, int32 tick)
        external
        payable
        returns (bool initialized, SqrtRatio sqrtRatio);

    /// @notice Mints a new NFT and deposits liquidity into it
    /// @param poolKey Pool key identifying the pool
    /// @param tickLower Lower tick of the price range of the position
    /// @param tickUpper Upper tick of the price range of the position
    /// @param maxAmount0 Maximum amount of token0 to deposit
    /// @param maxAmount1 Maximum amount of token1 to deposit
    /// @param minLiquidity Minimum liquidity to receive (for slippage protection)
    /// @return id The newly minted NFT token ID
    /// @return liquidity Amount of liquidity added to the position
    /// @return amount0 Actual amount of token0 deposited
    /// @return amount1 Actual amount of token1 deposited
    function mintAndDeposit(
        PoolKey memory poolKey,
        int32 tickLower,
        int32 tickUpper,
        uint128 maxAmount0,
        uint128 maxAmount1,
        uint128 minLiquidity
    ) external payable returns (uint256 id, uint128 liquidity, uint128 amount0, uint128 amount1);

    /// @notice Mints a new NFT with a specific salt and deposits liquidity into it
    /// @param salt Salt for deterministic NFT ID generation
    /// @param poolKey Pool key identifying the pool
    /// @param tickLower Lower tick of the price range of the position
    /// @param tickUpper Upper tick of the price range of the position
    /// @param maxAmount0 Maximum amount of token0 to deposit
    /// @param maxAmount1 Maximum amount of token1 to deposit
    /// @param minLiquidity Minimum liquidity to receive (for slippage protection)
    /// @return id The newly minted NFT token ID
    /// @return liquidity Amount of liquidity added to the position
    /// @return amount0 Actual amount of token0 deposited
    /// @return amount1 Actual amount of token1 deposited
    function mintAndDepositWithSalt(
        bytes32 salt,
        PoolKey memory poolKey,
        int32 tickLower,
        int32 tickUpper,
        uint128 maxAmount0,
        uint128 maxAmount1,
        uint128 minLiquidity
    ) external payable returns (uint256 id, uint128 liquidity, uint128 amount0, uint128 amount1);

    /// @notice Withdraws accumulated protocol fees (only callable by owner)
    /// @param token0 Address of token0
    /// @param token1 Address of token1
    /// @param amount0 Amount of token0 fees to withdraw
    /// @param amount1 Amount of token1 fees to withdraw
    /// @param recipient Address to receive the protocol fees
    function withdrawProtocolFees(address token0, address token1, uint128 amount0, uint128 amount1, address recipient)
        external
        payable;

    /// @notice Gets the accumulated protocol fees for a token pair
    /// @param token0 Address of token0
    /// @param token1 Address of token1
    /// @return amount0 Amount of token0 protocol fees
    /// @return amount1 Amount of token1 protocol fees
    function getProtocolFees(address token0, address token1) external view returns (uint128 amount0, uint128 amount1);
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {ILocker, IForwardee} from "../IFlashAccountant.sol";
import {IExtension} from "../ICore.sol";
import {IExposedStorage} from "../IExposedStorage.sol";
import {PoolKey} from "../../types/poolKey.sol";
import {OrderKey} from "../../types/orderKey.sol";
import {OrderConfig} from "../../types/orderConfig.sol";
import {PoolId} from "../../types/poolId.sol";

/// @title TWAMM Interface
/// @notice Interface for the Ekubo TWAMM Extension
/// @dev Extension for Ekubo Protocol that enables creation of DCA orders that are executed over time
interface ITWAMM is IExposedStorage, IExtension, ILocker, IForwardee {
    /// @notice Emitted when an order is updated
    /// @param owner Address of the order owner
    /// @param salt Unique salt for the order
    /// @param orderKey Order key identifying the order
    /// @param saleRateDelta Change in sale rate applied
    event OrderUpdated(address owner, bytes32 salt, OrderKey orderKey, int112 saleRateDelta);

    /// @notice Emitted when proceeds are withdrawn from an order
    /// @param owner Address of the order owner
    /// @param salt Unique salt for the order
    /// @param orderKey Order key identifying the order
    /// @param amount Amount of tokens withdrawn
    event OrderProceedsWithdrawn(address owner, bytes32 salt, OrderKey orderKey, uint128 amount);

    /// @notice Thrown when the number of orders at a time would overflow
    error TimeNumOrdersOverflow();

    /// @notice Thrown when tick spacing is not the maximum allowed value
    error FullRangePoolOnly();

    /// @notice Thrown when trying to modify an order that has already ended
    error OrderAlreadyEnded();

    /// @notice Thrown when order timestamps are invalid
    error InvalidTimestamps();

    /// @notice Thrown when sale rate delta exceeds maximum allowed value
    error MaxSaleRateDeltaPerTime();

    /// @notice Thrown when trying to operate on an uninitialized pool
    error PoolNotInitialized();

    /// @notice Gets the reward rate inside a time range for a specific token
    /// @dev Used to calculate how much of the buy token an order has earned
    /// @param poolId Unique identifier for the pool
    /// @param config The order config that is being checked
    /// @return result The reward rate inside the specified range
    function getRewardRateInside(PoolId poolId, OrderConfig config) external view returns (uint256 result);

    /// @notice Locks core and executes virtual orders for the given pool key
    /// @dev The pool key must use this extension, which is checked in the locked callback
    /// @param poolKey Pool key identifying the pool
    function lockAndExecuteVirtualOrders(PoolKey memory poolKey) external;
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {CallPoints} from "../types/callPoints.sol";
import {PoolKey} from "../types/poolKey.sol";
import {PositionId} from "../types/positionId.sol";
import {SqrtRatio, MIN_SQRT_RATIO, MAX_SQRT_RATIO} from "../types/sqrtRatio.sol";
import {ICore, IExtension} from "../interfaces/ICore.sol";
import {ITWAMM} from "../interfaces/extensions/ITWAMM.sol";
import {TimeInfo, createTimeInfo} from "../types/timeInfo.sol";
import {CoreLib} from "../libraries/CoreLib.sol";
import {ExposedStorage} from "../base/ExposedStorage.sol";
import {TWAMMStorageLayout} from "../libraries/TWAMMStorageLayout.sol";
import {StorageSlot} from "../types/storageSlot.sol";
import {BaseExtension} from "../base/BaseExtension.sol";
import {BaseForwardee} from "../base/BaseForwardee.sol";
import {PoolState} from "../types/poolState.sol";
import {PoolBalanceUpdate, createPoolBalanceUpdate} from "../types/poolBalanceUpdate.sol";
import {TwammPoolState, createTwammPoolState} from "../types/twammPoolState.sol";
import {OrderKey} from "../types/orderKey.sol";
import {OrderConfig} from "../types/orderConfig.sol";
import {OrderState, createOrderState} from "../types/orderState.sol";
import {searchForNextInitializedTime, flipTime} from "../math/timeBitmap.sol";
import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";
import {FeesPerLiquidity} from "../types/feesPerLiquidity.sol";
import {computeFee} from "../math/fee.sol";
import {
    computeNextSqrtRatio,
    computeAmountFromSaleRate,
    computeRewardAmount,
    addSaleRateDelta
} from "../math/twamm.sol";
import {isTimeValid, MAX_ABS_VALUE_SALE_RATE_DELTA} from "../math/time.sol";
import {PoolId} from "../types/poolId.sol";
import {OrderId} from "../types/orderId.sol";
import {SwapParameters, createSwapParameters} from "../types/swapParameters.sol";
import {Locker} from "../types/locker.sol";
import {LibBit} from "solady/utils/LibBit.sol";

/// @notice Returns the call points configuration for the TWAMM extension
/// @dev Specifies which hooks TWAMM needs to execute virtual orders and manage DCA functionality
/// @return The call points configuration for TWAMM functionality
function twammCallPoints() pure returns (CallPoints memory) {
    return CallPoints({
        beforeInitializePool: false,
        afterInitializePool: true,
        beforeUpdatePosition: true,
        afterUpdatePosition: false,
        beforeSwap: true,
        afterSwap: false,
        beforeCollectFees: true,
        afterCollectFees: false
    });
}

/// @title Ekubo TWAMM (Time-Weighted Average Market Maker)
/// @author Moody Salem <moody@ekubo.org>
/// @notice Extension for Ekubo Protocol that enables creation of DCA (Dollar Cost Averaging) orders that are executed over time
/// @dev Implements virtual order execution that spreads trades over time periods to reduce price impact and provide better execution
contract TWAMM is ITWAMM, ExposedStorage, BaseExtension, BaseForwardee {
    using CoreLib for *;

    constructor(ICore core) BaseExtension(core) BaseForwardee(core) {}

    /// @notice Emits an event for virtual order execution
    /// @dev Emits an event for the virtual order execution. Assumes that saleRateToken0 and saleRateToken1 are <= type(uint112).max
    /// @param poolId The unique identifier for the pool
    /// @param saleRateToken0 The sale rate for token0 orders
    /// @param saleRateToken1 The sale rate for token1 orders
    function _emitVirtualOrdersExecuted(PoolId poolId, uint256 saleRateToken0, uint256 saleRateToken1) internal {
        assembly ("memory-safe") {
            // by writing it backwards, we overwrite only the empty bits with each subsequent write
            // 28-60, only 46-60 can be non-zero
            mstore(28, saleRateToken1)
            // 14-46, only 32-46 can be non-zero
            mstore(14, saleRateToken0)
            mstore(0, poolId)

            log0(0, 60)
        }
    }

    /// @inheritdoc ITWAMM
    function getRewardRateInside(PoolId poolId, OrderConfig config) public view returns (uint256 result) {
        if (block.timestamp >= config.endTime()) {
            uint256 offset = LibBit.rawToUint(!config.isToken1());
            uint256 rewardRateStart =
                uint256(TWAMMStorageLayout.poolRewardRatesBeforeSlot(poolId, config.startTime()).add(offset).load());

            uint256 rewardRateEnd =
                uint256(TWAMMStorageLayout.poolRewardRatesBeforeSlot(poolId, config.endTime()).add(offset).load());

            unchecked {
                result = rewardRateEnd - rewardRateStart;
            }
        } else if (block.timestamp > config.startTime()) {
            uint256 offset = LibBit.rawToUint(!config.isToken1());

            //  note that we check gt because if it's equal to start time, then the reward rate inside is necessarily 0
            uint256 rewardRateStart =
                uint256(TWAMMStorageLayout.poolRewardRatesBeforeSlot(poolId, config.startTime()).add(offset).load());
            uint256 rewardRateCurrent = uint256(TWAMMStorageLayout.poolRewardRatesSlot(poolId).add(offset).load());

            unchecked {
                result = rewardRateCurrent - rewardRateStart;
            }
        } else {
            // less than or equal to start time
            // returns 0
        }
    }

    /// @notice Safely adds a change to a sale rate delta with overflow protection
    /// @dev Ensures the resulting sale rate delta doesn't exceed the maximum allowed value
    /// @param saleRateDelta The current sale rate delta
    /// @param saleRateDeltaChange The change to apply to the sale rate delta
    /// @return saleRateDeltaNext The new sale rate delta after applying the change
    function _addConstrainSaleRateDelta(int112 saleRateDelta, int256 saleRateDeltaChange)
        internal
        pure
        returns (int112 saleRateDeltaNext)
    {
        int256 result = int256(saleRateDelta) + saleRateDeltaChange;

        // checked addition, no overflow of int112 type
        if (FixedPointMathLib.abs(result) > MAX_ABS_VALUE_SALE_RATE_DELTA) {
            revert MaxSaleRateDeltaPerTime();
        }

        // we know cast is safe because abs(result) is less than MAX_ABS_VALUE_SALE_RATE_DELTA which fits in a int112
        saleRateDeltaNext = int112(result);
    }

    /// @notice Updates time-specific information for TWAMM orders
    /// @dev Manages the sale rate deltas and order counts for a specific time point
    /// @param poolId The unique identifier for the pool
    /// @param time The timestamp to update
    /// @param saleRateDelta The change in sale rate for this time
    /// @param isToken1 True if updating token1 sale rate, false for token0
    /// @param numOrdersChange The change in number of orders referencing this time
    function _updateTime(PoolId poolId, uint256 time, int256 saleRateDelta, bool isToken1, int256 numOrdersChange)
        internal
    {
        TimeInfo timeInfo = TimeInfo.wrap(TWAMMStorageLayout.poolTimeInfosSlot(poolId, time).load());
        (uint32 numOrders, int112 saleRateDeltaToken0, int112 saleRateDeltaToken1) = timeInfo.parse();

        // note we assume this will never overflow, since it would require 2**32 separate orders to be placed
        uint32 numOrdersNext;
        assembly ("memory-safe") {
            numOrdersNext := add(numOrders, numOrdersChange)
            if gt(numOrdersNext, 0xffffffff) {
                // cast sig "TimeNumOrdersOverflow()"
                mstore(0, shl(224, 0x6916a952))
                revert(0, 4)
            }
        }

        bool flip = (numOrders == 0) != (numOrdersNext == 0);

        // write the poolRewardRatesBefore[poolId][time] = (1,1) if any orders still reference the time, or write (0,0) otherwise
        // we assume `_updateTime` is being called only for times that are greater than block.timestamp, i.e. have not been crossed yet
        // this reduces the cost of crossing that timestamp to a warm write instead of a cold write
        if (flip) {
            bytes32 zeroNumOrders = bytes32(LibBit.rawToUint(numOrders == 0));

            TWAMMStorageLayout.poolRewardRatesBeforeSlot(poolId, time).storeTwo(zeroNumOrders, zeroNumOrders);

            flipTime(TWAMMStorageLayout.poolInitializedTimesBitmapSlot(poolId), time);
        }

        if (isToken1) {
            saleRateDeltaToken1 = _addConstrainSaleRateDelta(saleRateDeltaToken1, saleRateDelta);
        } else {
            saleRateDeltaToken0 = _addConstrainSaleRateDelta(saleRateDeltaToken0, saleRateDelta);
        }

        TWAMMStorageLayout.poolTimeInfosSlot(poolId, time)
            .store(TimeInfo.unwrap(createTimeInfo(numOrdersNext, saleRateDeltaToken0, saleRateDeltaToken1)));
    }

    /// @notice Returns the call points configuration for this extension
    /// @dev Overrides the base implementation to return TWAMM-specific call points
    /// @return The call points configuration
    function getCallPoints() internal pure override returns (CallPoints memory) {
        return twammCallPoints();
    }

    ///////////////////////// Callbacks /////////////////////////

    function handleForwardData(Locker original, bytes memory data) internal override returns (bytes memory result) {
        unchecked {
            uint256 callType = abi.decode(data, (uint256));
            address owner = original.addr();

            if (callType == 0) {
                (, bytes32 salt, OrderKey memory orderKey, int112 saleRateDelta) =
                    abi.decode(data, (uint256, bytes32, OrderKey, int112));

                (uint64 startTime, uint64 endTime) = (orderKey.config.startTime(), orderKey.config.endTime());

                if (endTime <= block.timestamp) revert OrderAlreadyEnded();

                if (
                    !isTimeValid(block.timestamp, startTime) || !isTimeValid(block.timestamp, endTime)
                        || startTime >= endTime
                ) {
                    revert InvalidTimestamps();
                }

                PoolKey memory poolKey = orderKey.toPoolKey(address(this));
                PoolId poolId = poolKey.toPoolId();
                _executeVirtualOrdersFromWithinLock(poolKey, poolId);

                OrderId orderId = orderKey.toOrderId();

                StorageSlot orderStateSlot =
                    TWAMMStorageLayout.orderStateSlotFollowedByOrderRewardRateSnapshotSlot(owner, salt, orderId);

                StorageSlot orderRewardRateSnapshotSlot = orderStateSlot.next();

                OrderState order = OrderState.wrap(orderStateSlot.load());
                uint256 rewardRateSnapshot = uint256(orderRewardRateSnapshotSlot.load());

                uint256 rewardRateInside = getRewardRateInside(poolId, orderKey.config);

                (uint32 lastUpdateTime, uint112 saleRate, uint112 amountSold) = order.parse();

                uint256 purchasedAmount = computeRewardAmount(rewardRateInside - rewardRateSnapshot, saleRate);

                uint256 saleRateNext = addSaleRateDelta(saleRate, saleRateDelta);

                uint256 rewardRateSnapshotAdjusted;
                int256 numOrdersChange;
                assembly ("memory-safe") {
                    rewardRateSnapshotAdjusted := mul(
                        sub(rewardRateInside, div(shl(128, purchasedAmount), saleRateNext)),
                        // if saleRateNext is zero, write 0 for the reward rate snapshot adjusted
                        iszero(iszero(saleRateNext))
                    )

                    // if current is zero, and next is zero, then 1-1 = 0
                    // if current is nonzero, and next is nonzero, then 0-0 = 0
                    // if current is zero, and next is nonzero, then we get 1-0 = 1
                    // if current is nonzero, and next is zero, then we get 0-1 = -1 = (type(uint256).max)
                    numOrdersChange := sub(iszero(saleRate), iszero(saleRateNext))
                }

                orderStateSlot.store(
                    OrderState.unwrap(
                        createOrderState({
                            _lastUpdateTime: uint32(block.timestamp),
                            _saleRate: uint112(saleRateNext),
                            _amountSold: uint112(
                                amountSold
                                    + computeAmountFromSaleRate({
                                        saleRate: saleRate,
                                        duration: FixedPointMathLib.min(
                                            uint32(block.timestamp) - lastUpdateTime,
                                            uint32(uint64(block.timestamp) - startTime)
                                        ),
                                        roundUp: false
                                    })
                            )
                        })
                    )
                );
                orderRewardRateSnapshotSlot.store(bytes32(rewardRateSnapshotAdjusted));

                bool isToken1 = orderKey.config.isToken1();

                if (block.timestamp < startTime) {
                    _updateTime(poolId, startTime, saleRateDelta, isToken1, numOrdersChange);
                    _updateTime(poolId, endTime, -int256(saleRateDelta), isToken1, numOrdersChange);
                } else {
                    // we know block.timestamp < orderKey.endTime because we validate that first
                    // and we know the order is active, so we have to apply its delta to the current pool state
                    StorageSlot currentStateSlot = TWAMMStorageLayout.twammPoolStateSlot(poolId);
                    TwammPoolState currentState = TwammPoolState.wrap(currentStateSlot.load());
                    (uint32 lastTime, uint112 rate0, uint112 rate1) = currentState.parse();

                    if (isToken1) {
                        currentState = createTwammPoolState({
                            _lastVirtualOrderExecutionTime: lastTime,
                            _saleRateToken0: rate0,
                            _saleRateToken1: uint112(addSaleRateDelta(rate1, saleRateDelta))
                        });
                    } else {
                        currentState = createTwammPoolState({
                            _lastVirtualOrderExecutionTime: lastTime,
                            _saleRateToken0: uint112(addSaleRateDelta(rate0, saleRateDelta)),
                            _saleRateToken1: rate1
                        });
                    }

                    currentStateSlot.store(TwammPoolState.unwrap(currentState));

                    // only update the end time
                    _updateTime(poolId, endTime, -int256(saleRateDelta), isToken1, numOrdersChange);
                }

                // we know this will fit in a uint32 because otherwise isValidTime would fail for the end time
                uint256 durationRemaining = endTime - FixedPointMathLib.max(block.timestamp, startTime);

                // the amount required for executing at the next sale rate for the remaining duration of the order
                uint256 amountRequired =
                    computeAmountFromSaleRate({saleRate: saleRateNext, duration: durationRemaining, roundUp: true});

                // subtract the remaining sell amount to get the delta
                int256 amountDelta;

                uint256 remainingSellAmount =
                    computeAmountFromSaleRate({saleRate: saleRate, duration: durationRemaining, roundUp: true});

                assembly ("memory-safe") {
                    amountDelta := sub(amountRequired, remainingSellAmount)
                }

                // user is withdrawing tokens, so they need to pay a fee to the liquidity providers
                if (amountDelta < 0) {
                    // negation and downcast will never overflow, since max sale rate times max duration is at most type(uint112).max
                    uint128 fee = computeFee(uint128(uint256(-amountDelta)), poolKey.config.fee());
                    if (isToken1) {
                        CORE.accumulateAsFees(poolKey, 0, fee);
                        CORE.updateSavedBalances(poolKey.token0, poolKey.token1, bytes32(0), 0, amountDelta);
                    } else {
                        CORE.accumulateAsFees(poolKey, fee, 0);
                        CORE.updateSavedBalances(poolKey.token0, poolKey.token1, bytes32(0), amountDelta, 0);
                    }

                    amountDelta += int128(fee);
                } else {
                    if (isToken1) {
                        CORE.updateSavedBalances(poolKey.token0, poolKey.token1, bytes32(0), 0, amountDelta);
                    } else {
                        CORE.updateSavedBalances(poolKey.token0, poolKey.token1, bytes32(0), amountDelta, 0);
                    }
                }

                emit OrderUpdated(owner, salt, orderKey, saleRateDelta);

                result = abi.encode(amountDelta);
            } else if (callType == 1) {
                (, bytes32 salt, OrderKey memory orderKey) = abi.decode(data, (uint256, bytes32, OrderKey));

                PoolKey memory poolKey = orderKey.toPoolKey(address(this));
                PoolId poolId = poolKey.toPoolId();
                _executeVirtualOrdersFromWithinLock(poolKey, poolId);

                OrderId orderId = orderKey.toOrderId();

                StorageSlot orderStateSlot =
                    TWAMMStorageLayout.orderStateSlotFollowedByOrderRewardRateSnapshotSlot(owner, salt, orderId);

                StorageSlot orderRewardRateSnapshotSlot = orderStateSlot.next();

                OrderState order = OrderState.wrap(orderStateSlot.load());
                uint256 rewardRateSnapshot = uint256(orderRewardRateSnapshotSlot.load());

                uint256 rewardRateInside = getRewardRateInside(poolId, orderKey.config);

                uint256 purchasedAmount = computeRewardAmount(rewardRateInside - rewardRateSnapshot, order.saleRate());

                orderRewardRateSnapshotSlot.store(bytes32(rewardRateInside));

                if (purchasedAmount != 0) {
                    if (orderKey.config.isToken1()) {
                        CORE.updateSavedBalances(
                            poolKey.token0, poolKey.token1, bytes32(0), -int256(purchasedAmount), 0
                        );
                    } else {
                        CORE.updateSavedBalances(
                            poolKey.token0, poolKey.token1, bytes32(0), 0, -int256(purchasedAmount)
                        );
                    }
                }

                emit OrderProceedsWithdrawn(original.addr(), salt, orderKey, uint128(purchasedAmount));

                result = abi.encode(purchasedAmount);
            } else {
                revert();
            }
        }
    }

    function _executeVirtualOrdersFromWithinLock(PoolKey memory poolKey, PoolId poolId) internal {
        unchecked {
            StorageSlot stateSlot = TWAMMStorageLayout.twammPoolStateSlot(poolId);
            TwammPoolState state = TwammPoolState.wrap(stateSlot.load());

            // we only conditionally load this if the state is coincidentally zero,
            // in order to not lock the pool if state is 0 but the pool _is_ initialized
            // this can only happen iff a pool has zero sale rates **and** an execution of virtual orders
            // happens on the uint32 boundary
            if (TwammPoolState.unwrap(state) == bytes32(0)) {
                if (poolKey.config.extension() != address(this) || !CORE.poolState(poolId).isInitialized()) {
                    revert PoolNotInitialized();
                }
            }

            uint256 realLastVirtualOrderExecutionTime = state.realLastVirtualOrderExecutionTime();

            // no-op if already executed in this block
            if (realLastVirtualOrderExecutionTime != block.timestamp) {
                // initialize the values that are handled once per execution
                FeesPerLiquidity memory rewardRates;

                // 0 = not loaded & not updated, 1 = loaded & not updated, 2 = loaded & updated
                uint256 rewardRate0Access;
                uint256 rewardRate1Access;

                int256 saveDelta0;
                int256 saveDelta1;
                PoolState corePoolState;
                uint256 time = realLastVirtualOrderExecutionTime;

                while (time != block.timestamp) {
                    StorageSlot initializedTimesBitmapSlot = TWAMMStorageLayout.poolInitializedTimesBitmapSlot(poolId);

                    (uint256 nextTime, bool initialized) = searchForNextInitializedTime({
                        slot: initializedTimesBitmapSlot,
                        lastVirtualOrderExecutionTime: realLastVirtualOrderExecutionTime,
                        fromTime: time,
                        untilTime: block.timestamp
                    });

                    // it is assumed that this will never return a value greater than type(uint32).max
                    uint256 timeElapsed = nextTime - time;

                    uint256 amount0 = computeAmountFromSaleRate({
                        saleRate: state.saleRateToken0(), duration: timeElapsed, roundUp: false
                    });

                    uint256 amount1 = computeAmountFromSaleRate({
                        saleRate: state.saleRateToken1(), duration: timeElapsed, roundUp: false
                    });

                    int256 rewardDelta0;
                    int256 rewardDelta1;
                    // if both sale rates are non-zero but amounts are zero, we will end up doing the math for no reason since we swap 0
                    if (amount0 != 0 && amount1 != 0) {
                        if (!corePoolState.isInitialized()) {
                            corePoolState = CORE.poolState(poolId);
                        }
                        SqrtRatio sqrtRatioNext = computeNextSqrtRatio({
                            sqrtRatio: corePoolState.sqrtRatio(),
                            liquidity: corePoolState.liquidity(),
                            saleRateToken0: state.saleRateToken0(),
                            saleRateToken1: state.saleRateToken1(),
                            timeElapsed: timeElapsed,
                            fee: poolKey.config.fee()
                        });

                        PoolBalanceUpdate swapBalanceUpdate;
                        if (sqrtRatioNext > corePoolState.sqrtRatio()) {
                            (swapBalanceUpdate, corePoolState) = CORE.swap(
                                0,
                                poolKey,
                                createSwapParameters({
                                    _sqrtRatioLimit: sqrtRatioNext,
                                    _amount: int128(uint128(amount1)),
                                    _isToken1: true,
                                    _skipAhead: 0
                                })
                            );
                        } else if (sqrtRatioNext < corePoolState.sqrtRatio()) {
                            (swapBalanceUpdate, corePoolState) = CORE.swap(
                                0,
                                poolKey,
                                createSwapParameters({
                                    _sqrtRatioLimit: sqrtRatioNext,
                                    _amount: int128(uint128(amount0)),
                                    _isToken1: false,
                                    _skipAhead: 0
                                })
                            );
                        }

                        saveDelta0 -= swapBalanceUpdate.delta0();
                        saveDelta1 -= swapBalanceUpdate.delta1();

                        // this cannot overflow or underflow because swapDelta0 is constrained to int128,
                        // and amounts computed from uint112 sale rates cannot exceed uint112.max
                        rewardDelta0 = swapBalanceUpdate.delta0() - int256(uint256(amount0));
                        rewardDelta1 = swapBalanceUpdate.delta1() - int256(uint256(amount1));
                    } else if (amount0 != 0 || amount1 != 0) {
                        PoolBalanceUpdate swapBalanceUpdate;
                        if (amount0 != 0) {
                            (swapBalanceUpdate, corePoolState) = CORE.swap(
                                0,
                                poolKey,
                                createSwapParameters({
                                    _sqrtRatioLimit: MIN_SQRT_RATIO,
                                    _amount: int128(uint128(amount0)),
                                    _isToken1: false,
                                    _skipAhead: 0
                                })
                            );
                        } else {
                            (swapBalanceUpdate, corePoolState) = CORE.swap(
                                0,
                                poolKey,
                                createSwapParameters({
                                    _sqrtRatioLimit: MAX_SQRT_RATIO,
                                    _amount: int128(uint128(amount1)),
                                    _isToken1: true,
                                    _skipAhead: 0
                                })
                            );
                        }

                        (rewardDelta0, rewardDelta1) = (swapBalanceUpdate.delta0(), swapBalanceUpdate.delta1());
                        saveDelta0 -= rewardDelta0;
                        saveDelta1 -= rewardDelta1;
                    }

                    if (rewardDelta0 < 0) {
                        if (rewardRate0Access == 0) {
                            rewardRates.value0 = uint256(TWAMMStorageLayout.poolRewardRatesSlot(poolId).load());
                        }
                        rewardRate0Access = 2;
                        rewardRates.value0 += FixedPointMathLib.rawDiv(
                            uint256(-rewardDelta0) << 128, state.saleRateToken1()
                        );
                    }

                    if (rewardDelta1 < 0) {
                        if (rewardRate1Access == 0) {
                            rewardRates.value1 = uint256(TWAMMStorageLayout.poolRewardRatesSlot(poolId).next().load());
                        }
                        rewardRate1Access = 2;
                        rewardRates.value1 += FixedPointMathLib.rawDiv(
                            uint256(-rewardDelta1) << 128, state.saleRateToken0()
                        );
                    }

                    if (initialized) {
                        if (rewardRate0Access == 0) {
                            rewardRates.value0 = uint256(TWAMMStorageLayout.poolRewardRatesSlot(poolId).load());
                            rewardRate0Access = 1;
                        }
                        if (rewardRate1Access == 0) {
                            rewardRates.value1 = uint256(TWAMMStorageLayout.poolRewardRatesSlot(poolId).next().load());
                            rewardRate1Access = 1;
                        }

                        TWAMMStorageLayout.poolRewardRatesBeforeSlot(poolId, nextTime)
                            .storeTwo(bytes32(rewardRates.value0), bytes32(rewardRates.value1));

                        StorageSlot timeInfoSlot = TWAMMStorageLayout.poolTimeInfosSlot(poolId, nextTime);
                        (, int112 saleRateDeltaToken0, int112 saleRateDeltaToken1) =
                            TimeInfo.wrap(timeInfoSlot.load()).parse();

                        state = createTwammPoolState({
                            _lastVirtualOrderExecutionTime: uint32(nextTime),
                            _saleRateToken0: uint112(addSaleRateDelta(state.saleRateToken0(), saleRateDeltaToken0)),
                            _saleRateToken1: uint112(addSaleRateDelta(state.saleRateToken1(), saleRateDeltaToken1))
                        });

                        // this time is _consumed_, will never be crossed again, so we delete the info we no longer need.
                        // this helps reduce the cost of executing virtual orders.
                        timeInfoSlot.store(0);

                        flipTime(initializedTimesBitmapSlot, nextTime);
                    } else {
                        state = createTwammPoolState({
                            _lastVirtualOrderExecutionTime: uint32(nextTime),
                            _saleRateToken0: state.saleRateToken0(),
                            _saleRateToken1: state.saleRateToken1()
                        });
                    }

                    time = nextTime;
                }

                if (saveDelta0 != 0 || saveDelta1 != 0) {
                    CORE.updateSavedBalances(poolKey.token0, poolKey.token1, bytes32(0), saveDelta0, saveDelta1);
                }

                if (rewardRate0Access == 2) {
                    TWAMMStorageLayout.poolRewardRatesSlot(poolId).store(bytes32(rewardRates.value0));
                }
                if (rewardRate1Access == 2) {
                    TWAMMStorageLayout.poolRewardRatesSlot(poolId).next().store(bytes32(rewardRates.value1));
                }

                stateSlot.store(TwammPoolState.unwrap(state));

                _emitVirtualOrdersExecuted(poolId, state.saleRateToken0(), state.saleRateToken1());
            }
        }
    }

    // Executes virtual orders for the specified initialized pool key. Protected because it is only called by core.
    function locked_6416899205(uint256) external override onlyCore {
        PoolKey memory poolKey;
        assembly ("memory-safe") {
            // copy the poolkey out of calldata at the solidity-allocated address
            calldatacopy(poolKey, 36, 96)
        }
        _executeVirtualOrdersFromWithinLock(poolKey, poolKey.toPoolId());
    }

    /// @inheritdoc ITWAMM
    function lockAndExecuteVirtualOrders(PoolKey memory poolKey) public {
        // the only thing we lock for is executing virtual orders, so all we need to encode is the pool key
        // so we call lock on the core contract with the pool key after it
        address target = address(CORE);
        assembly ("memory-safe") {
            let o := mload(0x40)
            mstore(o, shl(224, 0xf83d08ba))
            mcopy(add(o, 4), poolKey, 96)

            // If the call failed, pass through the revert
            if iszero(call(gas(), target, 0, o, 100, 0, 0)) {
                returndatacopy(o, 0, returndatasize())
                revert(o, returndatasize())
            }
        }
    }

    ///////////////////////// Extension call points /////////////////////////

    // This method must be protected because it sets state directly
    function afterInitializePool(address, PoolKey memory key, int32, SqrtRatio)
        external
        override(BaseExtension, IExtension)
        onlyCore
    {
        if (!key.config.isFullRange()) revert FullRangePoolOnly();

        PoolId poolId = key.toPoolId();

        TWAMMStorageLayout.twammPoolStateSlot(poolId)
            .store(
                TwammPoolState.unwrap(
                    createTwammPoolState({
                        _lastVirtualOrderExecutionTime: uint32(block.timestamp), _saleRateToken0: 0, _saleRateToken1: 0
                    })
                )
            );

        _emitVirtualOrdersExecuted({poolId: poolId, saleRateToken0: 0, saleRateToken1: 0});
    }

    // Since anyone can call the method `#lockAndExecuteVirtualOrders`, the method is not protected
    function beforeSwap(Locker, PoolKey memory poolKey, SwapParameters) external override(BaseExtension, IExtension) {
        lockAndExecuteVirtualOrders(poolKey);
    }

    // Since anyone can call the method `#lockAndExecuteVirtualOrders`, the method is not protected
    function beforeUpdatePosition(Locker, PoolKey memory poolKey, PositionId, int128)
        external
        override(BaseExtension, IExtension)
    {
        lockAndExecuteVirtualOrders(poolKey);
    }

    // Since anyone can call the method `#lockAndExecuteVirtualOrders`, the method is not protected
    function beforeCollectFees(Locker, PoolKey memory poolKey, PositionId)
        external
        override(BaseExtension, IExtension)
    {
        lockAndExecuteVirtualOrders(poolKey);
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity ^0.8.30;

import {ICore, PoolKey, PositionId, CallPoints} from "../interfaces/ICore.sol";
import {IMEVCapture} from "../interfaces/extensions/IMEVCapture.sol";
import {IExtension} from "../interfaces/ICore.sol";
import {BaseExtension} from "../base/BaseExtension.sol";
import {BaseForwardee} from "../base/BaseForwardee.sol";
import {amountBeforeFee, computeFee} from "../math/fee.sol";
import {ExposedStorage} from "../base/ExposedStorage.sol";
import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";
import {SafeCastLib} from "solady/utils/SafeCastLib.sol";
import {CoreLib} from "../libraries/CoreLib.sol";
import {ExposedStorageLib} from "../libraries/ExposedStorageLib.sol";
import {CoreStorageLayout} from "../libraries/CoreStorageLayout.sol";
import {PoolState} from "../types/poolState.sol";
import {MEVCapturePoolState, createMEVCapturePoolState} from "../types/mevCapturePoolState.sol";
import {SwapParameters} from "../types/swapParameters.sol";
import {PoolId} from "../types/poolId.sol";
import {Locker} from "../types/locker.sol";
import {StorageSlot} from "../types/storageSlot.sol";
import {PoolBalanceUpdate, createPoolBalanceUpdate} from "../types/poolBalanceUpdate.sol";

function mevCaptureCallPoints() pure returns (CallPoints memory) {
    return CallPoints({
        // to store the initial tick
        beforeInitializePool: true,
        afterInitializePool: false,
        // so that we can prevent swaps that are not made via forward
        beforeSwap: true,
        afterSwap: false,
        // in order to accumulate any collected fees
        beforeUpdatePosition: true,
        afterUpdatePosition: false,
        // in order to accumulate any collected fees
        beforeCollectFees: true,
        afterCollectFees: false
    });
}

/// @notice Charges additional fees based on the relative size of the priority fee
contract MEVCapture is IMEVCapture, BaseExtension, BaseForwardee, ExposedStorage {
    using CoreLib for *;
    using ExposedStorageLib for *;

    constructor(ICore core) BaseExtension(core) BaseForwardee(core) {}

    function getPoolState(PoolId poolId) private view returns (MEVCapturePoolState state) {
        assembly ("memory-safe") {
            state := sload(poolId)
        }
    }

    function setPoolState(PoolId poolId, MEVCapturePoolState state) private {
        assembly ("memory-safe") {
            sstore(poolId, state)
        }
    }

    function getCallPoints() internal pure override returns (CallPoints memory) {
        return mevCaptureCallPoints();
    }

    function beforeInitializePool(address, PoolKey memory poolKey, int32 tick)
        external
        override(BaseExtension, IExtension)
        onlyCore
    {
        if (poolKey.config.isStableswap()) {
            revert ConcentratedLiquidityPoolsOnly();
        }
        if (poolKey.config.fee() == 0) {
            // nothing to multiply == no-op extension
            revert NonzeroFeesOnly();
        }

        setPoolState({
            poolId: poolKey.toPoolId(),
            state: createMEVCapturePoolState({_lastUpdateTime: uint32(block.timestamp), _tickLast: tick})
        });
    }

    /// @notice We only allow swapping via forward to this extension
    function beforeSwap(Locker, PoolKey memory, SwapParameters) external pure override(BaseExtension, IExtension) {
        revert SwapMustHappenThroughForward();
    }

    // Allows users to collect pending fees before the first swap in the block happens
    function beforeCollectFees(Locker, PoolKey memory poolKey, PositionId)
        external
        override(BaseExtension, IExtension)
    {
        accumulatePoolFees(poolKey);
    }

    /// Prevents new liquidity from collecting on fees
    function beforeUpdatePosition(Locker, PoolKey memory poolKey, PositionId, int128)
        external
        override(BaseExtension, IExtension)
    {
        accumulatePoolFees(poolKey);
    }

    /// @inheritdoc IMEVCapture
    function accumulatePoolFees(PoolKey memory poolKey) public {
        PoolId poolId = poolKey.toPoolId();
        MEVCapturePoolState state = getPoolState(poolId);

        // the only thing we lock for is accumulating fees when the pool has not been updated in this block
        if (state.lastUpdateTime() != uint32(block.timestamp)) {
            address target = address(CORE);
            assembly ("memory-safe") {
                let o := mload(0x40)
                mstore(o, shl(224, 0xf83d08ba))
                mcopy(add(o, 4), poolKey, 96)
                mstore(add(o, 100), poolId)

                // If the call failed, pass through the revert
                if iszero(call(gas(), target, 0, o, 132, 0, 0)) {
                    returndatacopy(o, 0, returndatasize())
                    revert(o, returndatasize())
                }
            }
        }
    }

    function locked_6416899205(uint256) external onlyCore {
        PoolKey memory poolKey;
        PoolId poolId;
        assembly ("memory-safe") {
            // copy the poolkey out of calldata
            calldatacopy(poolKey, 36, 96)
            poolId := calldataload(132)
        }

        (int32 tick, uint128 fees0, uint128 fees1) = loadCoreState(poolId, poolKey.token0, poolKey.token1);

        if (fees0 != 0 || fees1 != 0) {
            CORE.accumulateAsFees(poolKey, fees0, fees1);
            unchecked {
                CORE.updateSavedBalances(
                    poolKey.token0,
                    poolKey.token1,
                    PoolId.unwrap(poolId),
                    -int256(uint256(fees0)),
                    -int256(uint256(fees1))
                );
            }
        }

        setPoolState({
            poolId: poolId,
            state: createMEVCapturePoolState({_lastUpdateTime: uint32(block.timestamp), _tickLast: tick})
        });
    }

    function loadCoreState(PoolId poolId, address token0, address token1)
        private
        view
        returns (int32 tick, uint128 fees0, uint128 fees1)
    {
        StorageSlot stateSlot = CoreStorageLayout.poolStateSlot(poolId);
        StorageSlot feesSlot = CoreStorageLayout.savedBalancesSlot(address(this), token0, token1, PoolId.unwrap(poolId));

        (bytes32 v0, bytes32 v1) = CORE.sload(stateSlot, feesSlot);
        tick = PoolState.wrap(v0).tick();

        assembly ("memory-safe") {
            fees0 := shr(128, v1)
            fees0 := sub(fees0, gt(fees0, 0))

            fees1 := shr(128, shl(128, v1))
            fees1 := sub(fees1, gt(fees1, 0))
        }
    }

    function handleForwardData(Locker, bytes memory data) internal override returns (bytes memory result) {
        unchecked {
            (PoolKey memory poolKey, SwapParameters params) = abi.decode(data, (PoolKey, SwapParameters));

            PoolId poolId = poolKey.toPoolId();
            MEVCapturePoolState state = getPoolState(poolId);
            uint32 lastUpdateTime = state.lastUpdateTime();
            int32 tickLast = state.tickLast();

            uint32 currentTime = uint32(block.timestamp);

            int256 saveDelta0;
            int256 saveDelta1;

            if (lastUpdateTime != currentTime) {
                (int32 tick, uint128 fees0, uint128 fees1) =
                    loadCoreState({poolId: poolId, token0: poolKey.token0, token1: poolKey.token1});

                if (fees0 != 0 || fees1 != 0) {
                    CORE.accumulateAsFees(poolKey, fees0, fees1);
                    // never overflows int256 container
                    saveDelta0 -= int256(uint256(fees0));
                    saveDelta1 -= int256(uint256(fees1));
                }

                tickLast = tick;
                setPoolState({
                    poolId: poolId,
                    state: createMEVCapturePoolState({_lastUpdateTime: currentTime, _tickLast: tickLast})
                });
            }

            (PoolBalanceUpdate balanceUpdate, PoolState stateAfter) = CORE.swap(0, poolKey, params);

            // however many tick spacings were crossed is the fee multiplier
            uint256 feeMultiplierX64 =
                (FixedPointMathLib.abs(stateAfter.tick() - tickLast) << 64) / poolKey.config.concentratedTickSpacing();
            uint64 poolFee = poolKey.config.fee();
            uint64 additionalFee = uint64(FixedPointMathLib.min(type(uint64).max, (feeMultiplierX64 * poolFee) >> 64));

            if (additionalFee != 0) {
                if (params.isExactOut()) {
                    // take an additional fee from the calculated input amount equal to the `additionalFee - poolFee`
                    if (balanceUpdate.delta0() > 0) {
                        uint128 inputAmount = uint128(uint256(int256(balanceUpdate.delta0())));
                        // first remove the fee to get the original input amount before we compute the additional fee
                        inputAmount -= computeFee(inputAmount, poolFee);
                        int128 fee = SafeCastLib.toInt128(amountBeforeFee(inputAmount, additionalFee) - inputAmount);

                        saveDelta0 += fee;
                        balanceUpdate = createPoolBalanceUpdate(balanceUpdate.delta0() + fee, balanceUpdate.delta1());
                    } else if (balanceUpdate.delta1() > 0) {
                        uint128 inputAmount = uint128(uint256(int256(balanceUpdate.delta1())));
                        // first remove the fee to get the original input amount before we compute the additional fee
                        inputAmount -= computeFee(inputAmount, poolFee);
                        int128 fee = SafeCastLib.toInt128(amountBeforeFee(inputAmount, additionalFee) - inputAmount);

                        saveDelta1 += fee;
                        balanceUpdate = createPoolBalanceUpdate(balanceUpdate.delta0(), balanceUpdate.delta1() + fee);
                    }
                } else {
                    if (balanceUpdate.delta0() < 0) {
                        uint128 outputAmount = uint128(uint256(-int256(balanceUpdate.delta0())));
                        int128 fee = SafeCastLib.toInt128(computeFee(outputAmount, additionalFee));

                        saveDelta0 += fee;
                        balanceUpdate = createPoolBalanceUpdate(balanceUpdate.delta0() + fee, balanceUpdate.delta1());
                    } else if (balanceUpdate.delta1() < 0) {
                        uint128 outputAmount = uint128(uint256(-int256(balanceUpdate.delta1())));
                        int128 fee = SafeCastLib.toInt128(computeFee(outputAmount, additionalFee));

                        saveDelta1 += fee;
                        balanceUpdate = createPoolBalanceUpdate(balanceUpdate.delta0(), balanceUpdate.delta1() + fee);
                    }
                }
            }

            if (saveDelta0 != 0 || saveDelta1 != 0) {
                CORE.updateSavedBalances(poolKey.token0, poolKey.token1, PoolId.unwrap(poolId), saveDelta0, saveDelta1);
            }

            result = abi.encode(balanceUpdate, stateAfter);
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {CallPoints} from "../types/callPoints.sol";
import {PoolKey, PoolConfig} from "../types/poolKey.sol";
import {PositionId} from "../types/positionId.sol";
import {ICore, IExtension} from "../interfaces/ICore.sol";
import {IOracle} from "../interfaces/extensions/IOracle.sol";
import {CoreLib} from "../libraries/CoreLib.sol";
import {ExposedStorage} from "../base/ExposedStorage.sol";
import {BaseExtension} from "../base/BaseExtension.sol";
import {NATIVE_TOKEN_ADDRESS} from "../math/constants.sol";
import {FixedPointMathLib} from "solady/utils/FixedPointMathLib.sol";
import {Snapshot, createSnapshot} from "../types/snapshot.sol";
import {Counts, createCounts} from "../types/counts.sol";
import {Observation, createObservation} from "../types/observation.sol";
import {PoolId} from "../types/poolId.sol";
import {PoolState} from "../types/poolState.sol";
import {SwapParameters} from "../types/swapParameters.sol";
import {Locker} from "../types/locker.sol";

/// @notice Returns the call points configuration for the Oracle extension
/// @dev Specifies which hooks the Oracle needs to capture price and liquidity data
/// @return The call points configuration for Oracle functionality
function oracleCallPoints() pure returns (CallPoints memory) {
    return CallPoints({
        beforeInitializePool: true,
        afterInitializePool: false,
        beforeUpdatePosition: true,
        afterUpdatePosition: false,
        beforeSwap: true,
        afterSwap: false,
        beforeCollectFees: false,
        afterCollectFees: false
    });
}

/// @notice Converts a logical index to a storage index for circular snapshot array
/// @dev Because the snapshots array is circular, the storage index of the most recently written snapshot can be any value in [0,c.count).
///      To simplify the code, we operate on the logical indices, rather than the storage indices.
///      For logical indices, the most recently written value is always at logicalIndex = c.count-1 and the earliest snapshot is always at logicalIndex = 0.
/// @param index The index of the most recently written snapshot
/// @param count The total number of snapshots that have been written
/// @param logicalIndex The index of the snapshot for which to compute the storage index
/// @return The storage index corresponding to the logical index
function logicalIndexToStorageIndex(uint256 index, uint256 count, uint256 logicalIndex) pure returns (uint256) {
    // We assume index < count and logicalIndex < count
    unchecked {
        return (index + 1 + logicalIndex) % count;
    }
}

/// @title Ekubo Oracle Extension
/// @author Moody Salem <moody@ekubo.org>
/// @notice Records price and liquidity into accumulators enabling a separate contract to compute a manipulation resistant average price and liquidity
contract Oracle is IOracle, ExposedStorage, BaseExtension {
    using CoreLib for ICore;

    constructor(ICore core) BaseExtension(core) {}

    /// @notice Emits a snapshot event for off-chain indexing
    /// @dev Uses assembly for gas-efficient event emission
    /// @param token The token address for the snapshot
    /// @param snapshot The snapshot that was just written
    function _emitSnapshotEvent(address token, Snapshot snapshot) private {
        unchecked {
            assembly ("memory-safe") {
                mstore(0, shl(96, token))
                mstore(20, snapshot)
                log0(0, 52)
            }
        }
    }

    /// @inheritdoc IOracle
    function getPoolKey(address token) public view returns (PoolKey memory) {
        PoolConfig config;
        assembly ("memory-safe") {
            config := shl(96, address())
        }
        return PoolKey({token0: NATIVE_TOKEN_ADDRESS, token1: token, config: config});
    }

    /// @notice Returns the call points configuration for this extension
    /// @dev Overrides the base implementation to return Oracle-specific call points
    /// @return The call points configuration
    function getCallPoints() internal pure override returns (CallPoints memory) {
        return oracleCallPoints();
    }

    /// @notice Inserts a new snapshot if enough time has passed since the last one
    /// @dev Only inserts if block.timestamp > lastTimestamp to avoid duplicate snapshots
    /// @param poolId The unique identifier for the pool
    /// @param token The token address for the oracle data
    function maybeInsertSnapshot(PoolId poolId, address token) private {
        unchecked {
            Counts c;
            assembly ("memory-safe") {
                c := sload(token)
            }

            uint32 timePassed = uint32(block.timestamp) - c.lastTimestamp();
            if (timePassed == 0) return;

            uint32 index = c.index();

            // we know count is always g.t. 0 in the places this is called
            Snapshot last;
            assembly ("memory-safe") {
                last := sload(or(shl(32, token), index))
            }

            PoolState state = CORE.poolState(poolId);

            uint128 liquidity = state.liquidity();
            uint256 nonZeroLiquidity;
            assembly ("memory-safe") {
                nonZeroLiquidity := add(liquidity, iszero(liquidity))
            }

            Snapshot snapshot = createSnapshot({
                _timestamp: uint32(block.timestamp),
                _secondsPerLiquidityCumulative: last.secondsPerLiquidityCumulative()
                    + uint160(FixedPointMathLib.rawDiv(uint256(timePassed) << 128, nonZeroLiquidity)),
                _tickCumulative: last.tickCumulative() + int64(uint64(timePassed)) * state.tick()
            });

            uint32 count = c.count();
            uint32 capacity = c.capacity();

            bool isLastIndex = index == count - 1;
            bool incrementCount = isLastIndex && capacity > count;

            if (incrementCount) count++;
            index = (index + 1) % count;
            uint32 lastTimestamp = uint32(block.timestamp);

            c = createCounts({_index: index, _count: count, _capacity: capacity, _lastTimestamp: lastTimestamp});
            assembly ("memory-safe") {
                sstore(token, c)
                sstore(or(shl(32, token), index), snapshot)
            }

            _emitSnapshotEvent(token, snapshot);
        }
    }

    /// @notice Called before a pool is initialized to set up Oracle tracking
    /// @dev Validates pool configuration and initializes the first snapshot
    function beforeInitializePool(address, PoolKey calldata key, int32)
        external
        override(BaseExtension, IExtension)
        onlyCore
    {
        if (key.token0 != NATIVE_TOKEN_ADDRESS) revert PairsWithNativeTokenOnly();
        if (key.config.fee() != 0) revert FeeMustBeZero();
        if (!key.config.isFullRange()) revert FullRangePoolOnly();

        address token = key.token1;

        // in case expandCapacity is called before the pool is initialized:
        //  remember we have the capacity since the snapshot storage has been initialized
        uint32 lastTimestamp = uint32(block.timestamp);

        Counts c;
        assembly ("memory-safe") {
            c := sload(token)
        }

        c = createCounts({
            _index: 0,
            _count: 1,
            _capacity: uint32(FixedPointMathLib.max(1, c.capacity())),
            _lastTimestamp: lastTimestamp
        });

        Snapshot snapshot =
            createSnapshot({_timestamp: lastTimestamp, _secondsPerLiquidityCumulative: 0, _tickCumulative: 0});

        assembly ("memory-safe") {
            sstore(token, c)
            sstore(shl(32, token), snapshot)
        }

        _emitSnapshotEvent(token, snapshot);
    }

    /// @notice Called before a position is updated to capture price/liquidity snapshot
    /// @dev Inserts a new snapshot if liquidity is changing
    function beforeUpdatePosition(Locker, PoolKey memory poolKey, PositionId, int128 liquidityDelta)
        external
        override(BaseExtension, IExtension)
        onlyCore
    {
        if (liquidityDelta != 0) {
            maybeInsertSnapshot(poolKey.toPoolId(), poolKey.token1);
        }
    }

    /// @notice Called before a swap to capture price/liquidity snapshot
    /// @dev Inserts a new snapshot if a swap is occurring
    function beforeSwap(Locker, PoolKey memory poolKey, SwapParameters params)
        external
        override(BaseExtension, IExtension)
        onlyCore
    {
        if (params.amount() != 0) {
            maybeInsertSnapshot(poolKey.toPoolId(), poolKey.token1);
        }
    }

    /// @inheritdoc IOracle
    function expandCapacity(address token, uint32 minCapacity) external returns (uint32 capacity) {
        Counts c;
        assembly ("memory-safe") {
            c := sload(token)
        }

        if (c.capacity() < minCapacity) {
            for (uint256 i = c.capacity(); i < minCapacity; i++) {
                assembly ("memory-safe") {
                    // Simply initialize the slot, it will be overwritten when the index is reached
                    sstore(or(shl(32, token), i), 1)
                }
            }
            c = createCounts({
                _index: c.index(), _count: c.count(), _capacity: minCapacity, _lastTimestamp: c.lastTimestamp()
            });
            assembly ("memory-safe") {
                sstore(token, c)
            }
        }

        capacity = c.capacity();
    }

    /// @notice Searches for the latest snapshot with timestamp <= time within a logical range
    /// @dev Searches the logical range [min, maxExclusive) for the latest snapshot with timestamp <= time.
    ///      See logicalIndexToStorageIndex for an explanation of logical indices.
    ///      We make the assumption that all snapshots for the token were written within (2**32 - 1) seconds of the current block timestamp
    /// @param c The counts containing metadata about the snapshots array
    /// @param token The token address to search snapshots for
    /// @param time The target timestamp to search for
    /// @param logicalMin The minimum logical index to search from
    /// @param logicalMaxExclusive The maximum logical index to search to (exclusive)
    /// @return logicalIndex The logical index of the found snapshot
    /// @return snapshot The snapshot data at the found index
    function searchRangeForPrevious(
        Counts c,
        address token,
        uint256 time,
        uint256 logicalMin,
        uint256 logicalMaxExclusive
    ) private view returns (uint256 logicalIndex, Snapshot snapshot) {
        unchecked {
            if (logicalMin >= logicalMaxExclusive) {
                revert NoPreviousSnapshotExists(token, time);
            }

            uint32 current = uint32(block.timestamp);
            uint32 targetDiff = current - uint32(time);

            uint256 left = logicalMin;
            uint256 right = logicalMaxExclusive - 1;
            while (left < right) {
                uint256 mid = (left + right + 1) >> 1;
                uint256 storageIndex = logicalIndexToStorageIndex(c.index(), c.count(), mid);
                Snapshot midSnapshot;
                assembly ("memory-safe") {
                    midSnapshot := sload(or(shl(32, token), storageIndex))
                }
                if (current - midSnapshot.timestamp() >= targetDiff) {
                    left = mid;
                } else {
                    right = mid - 1;
                }
            }

            uint256 resultIndex = logicalIndexToStorageIndex(c.index(), c.count(), left);
            assembly ("memory-safe") {
                snapshot := sload(or(shl(32, token), resultIndex))
            }
            if (current - snapshot.timestamp() < targetDiff) {
                revert NoPreviousSnapshotExists(token, time);
            }
            return (left, snapshot);
        }
    }

    /// @inheritdoc IOracle
    function findPreviousSnapshot(address token, uint256 time)
        public
        view
        returns (uint256 count, uint256 logicalIndex, Snapshot snapshot)
    {
        if (time > block.timestamp) revert FutureTime();

        Counts c;
        assembly ("memory-safe") {
            c := sload(token)
        }
        count = c.count();
        (logicalIndex, snapshot) = searchRangeForPrevious(c, token, time, 0, count);
    }

    /// @notice Computes cumulative values at a given time by extrapolating from a previous snapshot
    /// @dev Uses linear interpolation between snapshots or current pool state for extrapolation
    /// @param c The counts containing metadata about the snapshots array
    /// @param token The token address to extrapolate for
    /// @param atTime The timestamp to extrapolate to
    /// @param logicalIndex The logical index of the base snapshot
    /// @param snapshot The base snapshot to extrapolate from
    /// @return secondsPerLiquidityCumulative The extrapolated seconds per liquidity cumulative
    /// @return tickCumulative The extrapolated tick cumulative
    function extrapolateSnapshotInternal(
        Counts c,
        address token,
        uint256 atTime,
        uint256 logicalIndex,
        Snapshot snapshot
    ) private view returns (uint160 secondsPerLiquidityCumulative, int64 tickCumulative) {
        unchecked {
            secondsPerLiquidityCumulative = snapshot.secondsPerLiquidityCumulative();
            tickCumulative = snapshot.tickCumulative();
            uint32 timePassed = uint32(atTime) - snapshot.timestamp();
            if (timePassed != 0) {
                if (logicalIndex == c.count() - 1) {
                    // Use current pool state.
                    PoolId poolId = getPoolKey(token).toPoolId();
                    PoolState state = CORE.poolState(poolId);

                    tickCumulative += int64(state.tick()) * int64(uint64(timePassed));
                    secondsPerLiquidityCumulative += uint160(
                        FixedPointMathLib.rawDiv(
                            uint256(timePassed) << 128, FixedPointMathLib.max(1, state.liquidity())
                        )
                    );
                } else {
                    // Use the next snapshot.
                    uint256 logicalIndexNext = logicalIndexToStorageIndex(c.index(), c.count(), logicalIndex + 1);
                    Snapshot next;
                    assembly ("memory-safe") {
                        next := sload(or(shl(32, token), logicalIndexNext))
                    }

                    uint32 timestampDifference = next.timestamp() - snapshot.timestamp();

                    tickCumulative += int64(
                        FixedPointMathLib.rawSDiv(
                            int256(uint256(timePassed)) * (next.tickCumulative() - snapshot.tickCumulative()),
                            int256(uint256(timestampDifference))
                        )
                    );
                    secondsPerLiquidityCumulative += uint160(
                        (uint256(timePassed)
                                * (next.secondsPerLiquidityCumulative() - snapshot.secondsPerLiquidityCumulative()))
                            / timestampDifference
                    );
                }
            }
        }
    }

    /// @inheritdoc IOracle
    function extrapolateSnapshot(address token, uint256 atTime)
        public
        view
        returns (uint160 secondsPerLiquidityCumulative, int64 tickCumulative)
    {
        if (atTime > block.timestamp) revert FutureTime();

        Counts c;
        assembly ("memory-safe") {
            c := sload(token)
        }
        (uint256 logicalIndex, Snapshot snapshot) = searchRangeForPrevious(c, token, atTime, 0, c.count());
        (secondsPerLiquidityCumulative, tickCumulative) =
            extrapolateSnapshotInternal(c, token, atTime, logicalIndex, snapshot);
    }

    /// @inheritdoc IOracle
    function getExtrapolatedSnapshotsForSortedTimestamps(address token, uint256[] memory timestamps)
        public
        view
        returns (Observation[] memory observations)
    {
        unchecked {
            if (timestamps.length == 0) revert ZeroTimestampsProvided();
            uint256 startTime = timestamps[0];
            uint256 endTime = timestamps[timestamps.length - 1];
            if (endTime < startTime) revert EndTimeLessThanStartTime();

            Counts c;
            assembly ("memory-safe") {
                c := sload(token)
            }
            (uint256 indexFirst,) = searchRangeForPrevious(c, token, startTime, 0, c.count());
            (uint256 indexLast,) = searchRangeForPrevious(c, token, endTime, indexFirst, c.count());

            observations = new Observation[](timestamps.length);
            uint256 lastTimestamp;
            for (uint256 i = 0; i < timestamps.length; i++) {
                uint256 timestamp = timestamps[i];

                if (timestamp < lastTimestamp) {
                    revert TimestampsNotSorted();
                } else if (timestamp > block.timestamp) {
                    revert FutureTime();
                }

                (uint256 logicalIndex, Snapshot snapshot) =
                    searchRangeForPrevious(c, token, timestamp, indexFirst, indexLast + 1);
                (uint160 spcCumulative, int64 tcCumulative) =
                    extrapolateSnapshotInternal(c, token, timestamp, logicalIndex, snapshot);
                observations[i] = createObservation(spcCumulative, tcCumulative);
                indexFirst = logicalIndex;
                lastTimestamp = timestamp;
            }
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {BaseLocker} from "./BaseLocker.sol";
import {UsesCore} from "./UsesCore.sol";
import {ICore} from "../interfaces/ICore.sol";
import {IPositions} from "../interfaces/IPositions.sol";
import {CoreLib} from "../libraries/CoreLib.sol";
import {FlashAccountantLib} from "../libraries/FlashAccountantLib.sol";
import {PoolKey} from "../types/poolKey.sol";
import {PositionId, createPositionId} from "../types/positionId.sol";
import {FeesPerLiquidity} from "../types/feesPerLiquidity.sol";
import {Position} from "../types/position.sol";
import {tickToSqrtRatio} from "../math/ticks.sol";
import {maxLiquidity, liquidityDeltaToAmountDelta} from "../math/liquidity.sol";
import {PayableMulticallable} from "./PayableMulticallable.sol";
import {SqrtRatio} from "../types/sqrtRatio.sol";
import {SafeCastLib} from "solady/utils/SafeCastLib.sol";
import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";
import {BaseNonfungibleToken} from "./BaseNonfungibleToken.sol";
import {NATIVE_TOKEN_ADDRESS} from "../math/constants.sol";
import {PoolId} from "../types/poolId.sol";
import {PoolBalanceUpdate} from "../types/poolBalanceUpdate.sol";

/// @title Base Positions Contract
/// @author Moody Salem <moody@ekubo.org>
/// @notice Abstract base contract for tracking liquidity positions in Ekubo Protocol as NFTs
/// @dev Provides core position management functionality with abstract protocol fee collection methods
abstract contract BasePositions is IPositions, UsesCore, PayableMulticallable, BaseLocker, BaseNonfungibleToken {
    using CoreLib for *;
    using FlashAccountantLib for *;

    /// @notice Constructs the BasePositions contract
    /// @param core The core contract instance
    /// @param owner The owner of the contract (for access control)
    constructor(ICore core, address owner) BaseNonfungibleToken(owner) BaseLocker(core) UsesCore(core) {}

    uint256 private constant CALL_TYPE_DEPOSIT = 0;
    uint256 private constant CALL_TYPE_WITHDRAW = 1;
    uint256 private constant CALL_TYPE_WITHDRAW_PROTOCOL_FEES = 2;

    /// @inheritdoc IPositions
    function getPositionFeesAndLiquidity(uint256 id, PoolKey memory poolKey, int32 tickLower, int32 tickUpper)
        external
        view
        returns (uint128 liquidity, uint128 principal0, uint128 principal1, uint128 fees0, uint128 fees1)
    {
        PoolId poolId = poolKey.toPoolId();
        SqrtRatio sqrtRatio = CORE.poolState(poolId).sqrtRatio();
        PositionId positionId =
            createPositionId({_salt: bytes24(uint192(id)), _tickLower: tickLower, _tickUpper: tickUpper});
        Position memory position = CORE.poolPositions(poolId, address(this), positionId);

        liquidity = position.liquidity;

        // the sqrt ratio may be 0 (because the pool is uninitialized) but this is
        // fine since amount0Delta isn't called with it in this case
        (int128 delta0, int128 delta1) = liquidityDeltaToAmountDelta(
            sqrtRatio, -SafeCastLib.toInt128(position.liquidity), tickToSqrtRatio(tickLower), tickToSqrtRatio(tickUpper)
        );

        (principal0, principal1) = (uint128(-delta0), uint128(-delta1));

        FeesPerLiquidity memory feesPerLiquidityInside = poolKey.config.isFullRange()
            ? CORE.getPoolFeesPerLiquidity(poolId)
            : CORE.getPoolFeesPerLiquidityInside(poolId, tickLower, tickUpper);
        (fees0, fees1) = position.fees(feesPerLiquidityInside);
    }

    /// @inheritdoc IPositions
    function deposit(
        uint256 id,
        PoolKey memory poolKey,
        int32 tickLower,
        int32 tickUpper,
        uint128 maxAmount0,
        uint128 maxAmount1,
        uint128 minLiquidity
    ) public payable authorizedForNft(id) returns (uint128 liquidity, uint128 amount0, uint128 amount1) {
        SqrtRatio sqrtRatio = CORE.poolState(poolKey.toPoolId()).sqrtRatio();

        liquidity =
            maxLiquidity(sqrtRatio, tickToSqrtRatio(tickLower), tickToSqrtRatio(tickUpper), maxAmount0, maxAmount1);

        if (liquidity < minLiquidity) {
            revert DepositFailedDueToSlippage(liquidity, minLiquidity);
        }

        if (liquidity > uint128(type(int128).max)) {
            revert DepositOverflow();
        }

        (amount0, amount1) = abi.decode(
            lock(abi.encode(CALL_TYPE_DEPOSIT, msg.sender, id, poolKey, tickLower, tickUpper, liquidity)),
            (uint128, uint128)
        );
    }

    /// @inheritdoc IPositions
    function collectFees(uint256 id, PoolKey memory poolKey, int32 tickLower, int32 tickUpper)
        public
        payable
        authorizedForNft(id)
        returns (uint128 amount0, uint128 amount1)
    {
        (amount0, amount1) = collectFees(id, poolKey, tickLower, tickUpper, msg.sender);
    }

    /// @inheritdoc IPositions
    function collectFees(uint256 id, PoolKey memory poolKey, int32 tickLower, int32 tickUpper, address recipient)
        public
        payable
        authorizedForNft(id)
        returns (uint128 amount0, uint128 amount1)
    {
        (amount0, amount1) = withdraw(id, poolKey, tickLower, tickUpper, 0, recipient, true);
    }

    /// @inheritdoc IPositions
    function withdraw(
        uint256 id,
        PoolKey memory poolKey,
        int32 tickLower,
        int32 tickUpper,
        uint128 liquidity,
        address recipient,
        bool withFees
    ) public payable authorizedForNft(id) returns (uint128 amount0, uint128 amount1) {
        (amount0, amount1) = abi.decode(
            lock(abi.encode(CALL_TYPE_WITHDRAW, id, poolKey, tickLower, tickUpper, liquidity, recipient, withFees)),
            (uint128, uint128)
        );
    }

    /// @inheritdoc IPositions
    function withdraw(uint256 id, PoolKey memory poolKey, int32 tickLower, int32 tickUpper, uint128 liquidity)
        public
        payable
        returns (uint128 amount0, uint128 amount1)
    {
        (amount0, amount1) = withdraw(id, poolKey, tickLower, tickUpper, liquidity, address(msg.sender), true);
    }

    /// @inheritdoc IPositions
    function maybeInitializePool(PoolKey memory poolKey, int32 tick)
        external
        payable
        returns (bool initialized, SqrtRatio sqrtRatio)
    {
        // the before update position hook shouldn't be taken into account here
        sqrtRatio = CORE.poolState(poolKey.toPoolId()).sqrtRatio();
        if (sqrtRatio.isZero()) {
            initialized = true;
            sqrtRatio = CORE.initializePool(poolKey, tick);
        }
    }

    /// @inheritdoc IPositions
    function mintAndDeposit(
        PoolKey memory poolKey,
        int32 tickLower,
        int32 tickUpper,
        uint128 maxAmount0,
        uint128 maxAmount1,
        uint128 minLiquidity
    ) external payable returns (uint256 id, uint128 liquidity, uint128 amount0, uint128 amount1) {
        id = mint();
        (liquidity, amount0, amount1) = deposit(id, poolKey, tickLower, tickUpper, maxAmount0, maxAmount1, minLiquidity);
    }

    /// @inheritdoc IPositions
    function mintAndDepositWithSalt(
        bytes32 salt,
        PoolKey memory poolKey,
        int32 tickLower,
        int32 tickUpper,
        uint128 maxAmount0,
        uint128 maxAmount1,
        uint128 minLiquidity
    ) external payable returns (uint256 id, uint128 liquidity, uint128 amount0, uint128 amount1) {
        id = mint(salt);
        (liquidity, amount0, amount1) = deposit(id, poolKey, tickLower, tickUpper, maxAmount0, maxAmount1, minLiquidity);
    }

    /// @inheritdoc IPositions
    function withdrawProtocolFees(address token0, address token1, uint128 amount0, uint128 amount1, address recipient)
        external
        payable
        onlyOwner
    {
        lock(abi.encode(CALL_TYPE_WITHDRAW_PROTOCOL_FEES, token0, token1, amount0, amount1, recipient));
    }

    /// @inheritdoc IPositions
    function getProtocolFees(address token0, address token1) external view returns (uint128 amount0, uint128 amount1) {
        (amount0, amount1) = CORE.savedBalances(address(this), token0, token1, bytes32(0));
    }

    /// @notice Handles protocol fee collection during fee collection
    /// @dev Abstract method that must be implemented by concrete contracts
    /// @param poolKey The pool key for the position
    /// @param amount0 The amount of token0 fees collected before protocol fee deduction
    /// @param amount1 The amount of token1 fees collected before protocol fee deduction
    /// @return protocolFee0 The amount of token0 protocol fees to collect
    /// @return protocolFee1 The amount of token1 protocol fees to collect
    function _computeSwapProtocolFees(PoolKey memory poolKey, uint128 amount0, uint128 amount1)
        internal
        view
        virtual
        returns (uint128 protocolFee0, uint128 protocolFee1);

    /// @notice Handles protocol fee collection during liquidity withdrawal
    /// @dev Abstract method that must be implemented by concrete contracts
    /// @param poolKey The pool key for the position
    /// @param amount0 The amount of token0 being withdrawn before protocol fee deduction
    /// @param amount1 The amount of token1 being withdrawn before protocol fee deduction
    /// @return protocolFee0 The amount of token0 protocol fees to collect
    /// @return protocolFee1 The amount of token1 protocol fees to collect
    function _computeWithdrawalProtocolFees(PoolKey memory poolKey, uint128 amount0, uint128 amount1)
        internal
        view
        virtual
        returns (uint128 protocolFee0, uint128 protocolFee1);

    /// @notice Handles lock callback data for position operations
    /// @dev Internal function that processes different types of position operations
    /// @param data Encoded operation data
    /// @return result Encoded result data
    function handleLockData(uint256, bytes memory data) internal override returns (bytes memory result) {
        uint256 callType = abi.decode(data, (uint256));

        if (callType == CALL_TYPE_DEPOSIT) {
            (
                ,
                address caller,
                uint256 id,
                PoolKey memory poolKey,
                int32 tickLower,
                int32 tickUpper,
                uint128 liquidity
            ) = abi.decode(data, (uint256, address, uint256, PoolKey, int32, int32, uint128));

            PoolBalanceUpdate balanceUpdate = CORE.updatePosition(
                poolKey,
                createPositionId({_salt: bytes24(uint192(id)), _tickLower: tickLower, _tickUpper: tickUpper}),
                int128(liquidity)
            );

            uint128 amount0 = uint128(balanceUpdate.delta0());
            uint128 amount1 = uint128(balanceUpdate.delta1());

            // Use multi-token payment for ERC20-only pools, fall back to individual payments for native token pools
            if (poolKey.token0 != NATIVE_TOKEN_ADDRESS) {
                ACCOUNTANT.payTwoFrom(caller, poolKey.token0, poolKey.token1, amount0, amount1);
            } else {
                if (amount0 != 0) {
                    SafeTransferLib.safeTransferETH(address(ACCOUNTANT), amount0);
                }
                if (amount1 != 0) {
                    ACCOUNTANT.payFrom(caller, poolKey.token1, amount1);
                }
            }

            result = abi.encode(amount0, amount1);
        } else if (callType == CALL_TYPE_WITHDRAW) {
            (
                ,
                uint256 id,
                PoolKey memory poolKey,
                int32 tickLower,
                int32 tickUpper,
                uint128 liquidity,
                address recipient,
                bool withFees
            ) = abi.decode(data, (uint256, uint256, PoolKey, int32, int32, uint128, address, bool));

            if (liquidity > uint128(type(int128).max)) revert WithdrawOverflow();

            uint128 amount0;
            uint128 amount1;

            // collect first in case we are withdrawing the entire amount
            if (withFees) {
                (amount0, amount1) = CORE.collectFees(
                    poolKey,
                    createPositionId({_salt: bytes24(uint192(id)), _tickLower: tickLower, _tickUpper: tickUpper})
                );

                // Collect swap protocol fees
                (uint128 swapProtocolFee0, uint128 swapProtocolFee1) =
                    _computeSwapProtocolFees(poolKey, amount0, amount1);

                if (swapProtocolFee0 != 0 || swapProtocolFee1 != 0) {
                    CORE.updateSavedBalances(
                        poolKey.token0, poolKey.token1, bytes32(0), int128(swapProtocolFee0), int128(swapProtocolFee1)
                    );

                    amount0 -= swapProtocolFee0;
                    amount1 -= swapProtocolFee1;
                }
            }

            if (liquidity != 0) {
                PoolBalanceUpdate balanceUpdate = CORE.updatePosition(
                    poolKey,
                    createPositionId({_salt: bytes24(uint192(id)), _tickLower: tickLower, _tickUpper: tickUpper}),
                    -int128(liquidity)
                );

                uint128 withdrawnAmount0 = uint128(-balanceUpdate.delta0());
                uint128 withdrawnAmount1 = uint128(-balanceUpdate.delta1());

                // Collect withdrawal protocol fees
                (uint128 withdrawalFee0, uint128 withdrawalFee1) =
                    _computeWithdrawalProtocolFees(poolKey, withdrawnAmount0, withdrawnAmount1);

                if (withdrawalFee0 != 0 || withdrawalFee1 != 0) {
                    // we know cast won't overflow because delta0 and delta1 were originally int128
                    CORE.updateSavedBalances(
                        poolKey.token0, poolKey.token1, bytes32(0), int128(withdrawalFee0), int128(withdrawalFee1)
                    );
                }

                amount0 += withdrawnAmount0 - withdrawalFee0;
                amount1 += withdrawnAmount1 - withdrawalFee1;
            }

            ACCOUNTANT.withdrawTwo(poolKey.token0, poolKey.token1, recipient, amount0, amount1);

            result = abi.encode(amount0, amount1);
        } else if (callType == CALL_TYPE_WITHDRAW_PROTOCOL_FEES) {
            (, address token0, address token1, uint128 amount0, uint128 amount1, address recipient) =
                abi.decode(data, (uint256, address, address, uint128, uint128, address));

            CORE.updateSavedBalances(token0, token1, bytes32(0), -int256(uint256(amount0)), -int256(uint256(amount1)));
            ACCOUNTANT.withdrawTwo(token0, token1, recipient, amount0, amount1);
        } else {
            // Will never actually happen
            revert();
        }
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {ICore, IExtension} from "../interfaces/ICore.sol";
import {CallPoints} from "../types/callPoints.sol";
import {PoolKey} from "../types/poolKey.sol";
import {PositionId} from "../types/positionId.sol";
import {SqrtRatio} from "../types/sqrtRatio.sol";
import {PoolState} from "../types/poolState.sol";
import {SwapParameters} from "../types/swapParameters.sol";
import {UsesCore} from "./UsesCore.sol";
import {Locker} from "../types/locker.sol";
import {PoolBalanceUpdate} from "../types/poolBalanceUpdate.sol";

/// @title Base Extension
/// @notice Abstract base contract for creating extensions to the Ekubo Protocol
/// @dev Extensions can hook into various pool operations to add custom functionality
///      Derived contracts must implement getCallPoints() and the specific hook functions they want to use
abstract contract BaseExtension is IExtension, UsesCore {
    /// @notice Thrown when a call point is not implemented by the extension
    error CallPointNotImplemented();

    /// @notice Constructs the BaseExtension and optionally registers it with the core
    /// @param core The core contract instance
    constructor(ICore core) UsesCore(core) {
        if (_registerInConstructor()) core.registerExtension(getCallPoints());
    }

    /// @notice Determines whether the extension should register itself in the constructor
    /// @dev Can be overridden by derived contracts to control registration timing
    /// @return True if the extension should register in the constructor
    function _registerInConstructor() internal pure virtual returns (bool) {
        return true;
    }

    /// @notice Returns the call points configuration for this extension
    /// @dev Must be implemented by derived contracts to specify which hooks they use
    /// @return The call points configuration
    function getCallPoints() internal virtual returns (CallPoints memory);

    /// @inheritdoc IExtension
    function beforeInitializePool(address, PoolKey calldata, int32) external virtual {
        revert CallPointNotImplemented();
    }

    /// @inheritdoc IExtension
    function afterInitializePool(address, PoolKey calldata, int32, SqrtRatio) external virtual {
        revert CallPointNotImplemented();
    }

    /// @inheritdoc IExtension
    function beforeUpdatePosition(Locker, PoolKey memory, PositionId, int128) external virtual {
        revert CallPointNotImplemented();
    }

    /// @inheritdoc IExtension
    function afterUpdatePosition(Locker, PoolKey memory, PositionId, int128, PoolBalanceUpdate, PoolState)
        external
        virtual
    {
        revert CallPointNotImplemented();
    }

    /// @inheritdoc IExtension
    function beforeSwap(Locker, PoolKey memory, SwapParameters) external virtual {
        revert CallPointNotImplemented();
    }

    /// @inheritdoc IExtension
    function afterSwap(Locker, PoolKey memory, SwapParameters, PoolBalanceUpdate, PoolState) external virtual {
        revert CallPointNotImplemented();
    }

    /// @inheritdoc IExtension
    function beforeCollectFees(Locker, PoolKey memory, PositionId) external virtual {
        revert CallPointNotImplemented();
    }

    /// @inheritdoc IExtension
    function afterCollectFees(Locker, PoolKey memory, PositionId, uint128, uint128) external virtual {
        revert CallPointNotImplemented();
    }
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {PoolKey} from "../../types/poolKey.sol";
import {ILocker, IForwardee} from "../IFlashAccountant.sol";
import {IExtension} from "../ICore.sol";
import {IExposedStorage} from "../IExposedStorage.sol";

/// @title MEV Capture Interface
/// @notice Interface for the Ekubo MEV Capture Extension
/// @dev Extension that charges additional fees based on the relative size of the priority fee and tick movement during swaps
interface IMEVCapture is IExposedStorage, ILocker, IForwardee, IExtension {
    /// @notice Thrown when trying to use MEV capture on a full-range-only pool
    /// @dev MEV capture only works with concentrated liquidity pools that have discrete tick spacing
    error ConcentratedLiquidityPoolsOnly();

    /// @notice Thrown when trying to use MEV capture on a pool with zero fees
    /// @dev MEV capture multiplies the base fee, so a zero fee would result in no additional fees
    error NonzeroFeesOnly();

    /// @notice Thrown when attempting to swap directly without using the forward mechanism
    /// @dev All swaps must go through the forward mechanism to ensure proper MEV fee calculation
    error SwapMustHappenThroughForward();

    /// @notice Accumulates any pending pool fees from past blocks
    /// @dev This function can be called by anyone to trigger fee accumulation for a pool
    /// @dev Fees are accumulated when the pool hasn't been updated in the current block
    /// @param poolKey The pool key identifying the pool to accumulate fees for
    function accumulatePoolFees(PoolKey memory poolKey) external;
}

// SPDX-License-Identifier: ekubo-license-v1.eth
pragma solidity >=0.8.30;

import {PoolKey} from "../../types/poolKey.sol";
import {IExtension} from "../ICore.sol";
import {IExposedStorage} from "../IExposedStorage.sol";
import {Snapshot} from "../../types/snapshot.sol";
import {Observation} from "../../types/observation.sol";

/// @title Oracle Interface
/// @notice Interface for the Ekubo Oracle Extension
/// @dev Records price and liquidity into accumulators enabling a separate contract to compute a manipulation resistant average price and liquidity
interface IOracle is IExposedStorage, IExtension {
    /// @notice Thrown when trying to create a pool that doesn't pair with the native token
    error PairsWithNativeTokenOnly();

    /// @notice Thrown when the pool fee is not zero
    error FeeMustBeZero();

    /// @notice Thrown when the tick spacing is not the maximum allowed value
    error FullRangePoolOnly();

    /// @notice Thrown when querying data for a future timestamp
    error FutureTime();

    /// @notice Thrown when no previous snapshot exists for the given time
    /// @param token The token address
    /// @param time The requested timestamp
    error NoPreviousSnapshotExists(address token, uint256 time);

    /// @notice Thrown when the end time is less than the start time
    error EndTimeLessThanStartTime();

    /// @notice Thrown when timestamps are not provided in sorted order
    error TimestampsNotSorted();

    /// @notice Thrown when zero timestamps are provided to a function that requires at least one
    error ZeroTimestampsProvided();

    /// @notice Gets the pool key for the given token
    /// @dev The only allowed pool key for the given token pairs with the native token
    /// @param token The token address
    /// @return The pool key for the token paired with native token
    function getPoolKey(address token) external view returns (PoolKey memory);

    /// @notice Expands the capacity of the list of snapshots for the given token
    /// @param token The token address
    /// @param minCapacity The minimum capacity required
    /// @return capacity The actual capacity after expansion
    function expandCapacity(address token, uint32 minCapacity) external returns (uint32 capacity);

    /// @notice Finds the snapshot with the greatest timestamp ≤ the given time
    /// @param token The token address
    /// @param time The target timestamp
    /// @return count The total number of snapshots for the token
    /// @return logicalIndex The logical index of the found snapshot
    /// @return snapshot The snapshot data
    function findPreviousSnapshot(address token, uint256 time)
        external
        view
        returns (uint256 count, uint256 logicalIndex, Snapshot snapshot);

    /// @notice Returns cumulative snapshot values at a specific time
    /// @dev Extrapolates values from the most recent snapshot before the given time
    /// @param token The token address
    /// @param atTime The target timestamp
    /// @return secondsPerLiquidityCumulative The cumulative seconds per liquidity at the target time
    /// @return tickCumulative The cumulative tick value at the target time
    function extrapolateSnapshot(address token, uint256 atTime)
        external
        view
        returns (uint160 secondsPerLiquidityCumulative, int64 tickCumulative);

    /// @notice Returns extrapolated snapshots at each of the provided sorted timestamps
    /// @dev Efficiently computes observations for multiple timestamps in a single call
    /// @param token The token address
    /// @param timestamps Array of timestamps in ascending order
    /// @return observations Array of observations corresponding to each timestamp
    function getExtrapolatedSnapshotsForSortedTimestamps(address token, uint256[] memory timestamps)
        external
        view
        returns (Observation[] memory observations);
}


## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

