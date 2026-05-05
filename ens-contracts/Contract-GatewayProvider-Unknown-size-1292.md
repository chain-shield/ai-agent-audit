
 ------------ ## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {Ownable} from "@openzeppelin/contracts-v5/access/Ownable.sol";

import {IGatewayProvider} from "./IGatewayProvider.sol";

contract GatewayProvider is Ownable, IGatewayProvider {
    string[] _urls;

    constructor(address owner, string[] memory urls) Ownable(owner) {
        _urls = urls;
    }

    /// @inheritdoc IGatewayProvider
    function gateways() external view returns (string[] memory) {
        return _urls;
    }

    /// @notice Set the gateways.
    /// @param urls The gateway URLs.
    function setGateways(string[] memory urls) external onlyOwner {
        _urls = urls;
    }
}
 ------------
 ------------ END OF MAIN TARGET CONTRACT ------------ 

 ------------ ## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES ------------ 
// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v5.0.0) (access/Ownable.sol)

pragma solidity ^0.8.20;

import {Context} from "../utils/Context.sol";

/**
 * @dev Contract module which provides a basic access control mechanism, where
 * there is an account (an owner) that can be granted exclusive access to
 * specific functions.
 *
 * The initial owner is set to the address provided by the deployer. This can
 * later be changed with {transferOwnership}.
 *
 * This module is used through inheritance. It will make available the modifier
 * `onlyOwner`, which can be applied to your functions to restrict their use to
 * the owner.
 */
abstract contract Ownable is Context {
    address private _owner;

    /**
     * @dev The caller account is not authorized to perform an operation.
     */
    error OwnableUnauthorizedAccount(address account);

    /**
     * @dev The owner is not a valid owner account. (eg. `address(0)`)
     */
    error OwnableInvalidOwner(address owner);

    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);

    /**
     * @dev Initializes the contract setting the address provided by the deployer as the initial owner.
     */
    constructor(address initialOwner) {
        if (initialOwner == address(0)) {
            revert OwnableInvalidOwner(address(0));
        }
        _transferOwnership(initialOwner);
    }

    /**
     * @dev Throws if called by any account other than the owner.
     */
    modifier onlyOwner() {
        _checkOwner();
        _;
    }

    /**
     * @dev Returns the address of the current owner.
     */
    function owner() public view virtual returns (address) {
        return _owner;
    }

    /**
     * @dev Throws if the sender is not the owner.
     */
    function _checkOwner() internal view virtual {
        if (owner() != _msgSender()) {
            revert OwnableUnauthorizedAccount(_msgSender());
        }
    }

    /**
     * @dev Leaves the contract without owner. It will not be possible to call
     * `onlyOwner` functions. Can only be called by the current owner.
     *
     * NOTE: Renouncing ownership will leave the contract without an owner,
     * thereby disabling any functionality that is only available to the owner.
     */
    function renounceOwnership() public virtual onlyOwner {
        _transferOwnership(address(0));
    }

    /**
     * @dev Transfers ownership of the contract to a new account (`newOwner`).
     * Can only be called by the current owner.
     */
    function transferOwnership(address newOwner) public virtual onlyOwner {
        if (newOwner == address(0)) {
            revert OwnableInvalidOwner(address(0));
        }
        _transferOwnership(newOwner);
    }

    /**
     * @dev Transfers ownership of the contract to a new account (`newOwner`).
     * Internal function without access restriction.
     */
    function _transferOwnership(address newOwner) internal virtual {
        address oldOwner = _owner;
        _owner = newOwner;
        emit OwnershipTransferred(oldOwner, newOwner);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/// @notice Interface for shared gateway URLs.
/// @dev Interface selector: `0x093a86d3`
interface IGatewayProvider {
    /// @notice Get the gateways.
    /// @return The gateway URLs.
    function gateways() external view returns (string[] memory);
}


## ------------ SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS ------------ 
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {IGatewayProvider} from "./IGatewayProvider.sol";

// A gateway provider that randomly shuffles its underlying provider.
contract ShuffledGatewayProvider is IGatewayProvider {
    IGatewayProvider public immutable provider;

    constructor(IGatewayProvider _provider) {
        provider = _provider;
    }

    /// @inheritdoc IGatewayProvider
    function gateways() external view override returns (string[] memory urls) {
        return
            shuffledGateways(
                uint256(
                    keccak256(
                        abi.encodePacked(
                            blockhash(block.number - 1),
                            msg.sender,
                            block.timestamp
                        )
                    )
                )
            );
    }

    /// @dev Deterministically shuffle the gateway URLs.
    /// @param seed The shuffle seed.
    /// @return urls The shuffled gateway URLs.
    function shuffledGateways(
        uint256 seed
    ) public view returns (string[] memory urls) {
        urls = provider.gateways();
        uint256 n = urls.length;
        for (uint256 i = 1; i < n; ++i) {
            uint256 j = seed % (i + 1);
            (urls[i], urls[j]) = (urls[j], urls[i]);
            assembly {
                mstore(0, seed)
                seed := keccak256(0, 32)
            }
        }
    }
}


## ------------ SUPPORTING CONTEXT: EXTERNAL LIBRARIES ------------ 

 ------------ END OF SUPPORTING CONTRACTS AND INTERFACES ------------ 


 ------------ ## DEPLOYMENT SCRIPTS ------------ 

