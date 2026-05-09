
 ------------ ## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity ^0.7.6;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable gas-calldata-parameters, gas-indexed-events, gas-small-strings
// solhint-disable named-parameters-mapping

import { ERC721 } from "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import { Address } from "@openzeppelin/contracts/utils/Address.sol";

import { Governed } from "../governance/Governed.sol";
import { HexStrings } from "../libraries/HexStrings.sol";
import { ISubgraphNFT } from "@graphprotocol/interfaces/contracts/contracts/discovery/ISubgraphNFT.sol";
import { ISubgraphNFTDescriptor } from "@graphprotocol/interfaces/contracts/contracts/discovery/ISubgraphNFTDescriptor.sol";

/**
 * @title NFT that represents ownership of a Subgraph
 * @author Edge & Node
 * @notice NFT that represents ownership of a Subgraph
 */
contract SubgraphNFT is Governed, ERC721, ISubgraphNFT {
    // -- State --

    /// @notice Address of the minter contract
    address public minter;
    /// @notice Address of the token descriptor contract
    ISubgraphNFTDescriptor public tokenDescriptor;
    /// @dev Mapping from token ID to subgraph metadata hash
    mapping(uint256 => bytes32) private _subgraphMetadataHashes;

    // -- Events --

    /**
     * @notice Emitted when the minter address is updated
     * @param minter Address of the new minter
     */
    event MinterUpdated(address minter);

    /**
     * @notice Emitted when the token descriptor is updated
     * @param tokenDescriptor Address of the new token descriptor
     */
    event TokenDescriptorUpdated(address tokenDescriptor);

    /**
     * @notice Emitted when subgraph metadata is updated
     * @param tokenID ID of the token
     * @param subgraphURI IPFS hash of the subgraph metadata
     */
    event SubgraphMetadataUpdated(uint256 indexed tokenID, bytes32 subgraphURI);

    // -- Modifiers --

    /// @dev Modifier to restrict access to minter only
    modifier onlyMinter() {
        require(msg.sender == minter, "Must be a minter");
        _;
    }

    /**
     * @notice Constructor for the SubgraphNFT contract
     * @param _governor Address that will have governance privileges
     */
    constructor(address _governor) ERC721("Subgraph", "SG") {
        _initialize(_governor);
    }

    // -- Config --

    /**
     * @inheritdoc ISubgraphNFT
     */
    function setMinter(address _minter) external override onlyGovernor {
        _setMinter(_minter);
    }

    /**
     * @notice Internal: Set the minter allowed to perform actions on the NFT.
     * @dev Minter can mint, burn and update the metadata. Can be set to zero.
     * @param _minter Address of the allowed minter
     */
    function _setMinter(address _minter) internal {
        minter = _minter;
        emit MinterUpdated(_minter);
    }

    /**
     * @inheritdoc ISubgraphNFT
     */
    function setTokenDescriptor(address _tokenDescriptor) external override onlyGovernor {
        _setTokenDescriptor(_tokenDescriptor);
    }

    /**
     * @notice Internal: Set the token descriptor contract used to create the ERC-721 metadata URI.
     * @param _tokenDescriptor Address of the contract that creates the NFT token URI
     */
    function _setTokenDescriptor(address _tokenDescriptor) internal {
        require(
            _tokenDescriptor == address(0) || Address.isContract(_tokenDescriptor),
            "NFT: Invalid token descriptor"
        );
        tokenDescriptor = ISubgraphNFTDescriptor(_tokenDescriptor);
        emit TokenDescriptorUpdated(_tokenDescriptor);
    }

    /**
     * @inheritdoc ISubgraphNFT
     */
    function setBaseURI(string memory _baseURI) external override onlyGovernor {
        _setBaseURI(_baseURI);
    }

    // -- Minter actions --

    /**
     * @inheritdoc ISubgraphNFT
     */
    function mint(address _to, uint256 _tokenId) external override onlyMinter {
        _mint(_to, _tokenId);
    }

    /**
     * @inheritdoc ISubgraphNFT
     */
    function burn(uint256 _tokenId) external override onlyMinter {
        _burn(_tokenId);
    }

    /**
     * @inheritdoc ISubgraphNFT
     */
    function setSubgraphMetadata(uint256 _tokenId, bytes32 _subgraphMetadata) external override onlyMinter {
        require(_exists(_tokenId), "ERC721Metadata: URI set of nonexistent token");
        _subgraphMetadataHashes[_tokenId] = _subgraphMetadata;
        emit SubgraphMetadataUpdated(_tokenId, _subgraphMetadata);
    }

    // -- NFT display --

    /// @inheritdoc ERC721
    function tokenURI(uint256 _tokenId) public view override(ERC721, ISubgraphNFT) returns (string memory) {
        require(_exists(_tokenId), "ERC721Metadata: URI query for nonexistent token");

        // Delegates rendering of the metadata to the token descriptor if existing
        // This allows for some flexibility in adapting the token URI
        if (address(tokenDescriptor) != address(0)) {
            return tokenDescriptor.tokenURI(minter, _tokenId, baseURI(), _subgraphMetadataHashes[_tokenId]);
        }

        // Default token URI
        uint256 metadata = uint256(_subgraphMetadataHashes[_tokenId]);

        string memory _subgraphURI = metadata > 0 ? HexStrings.toString(metadata) : "";
        string memory base = baseURI();

        // If there is no base URI, return the token URI.
        if (bytes(base).length == 0) {
            return _subgraphURI;
        }
        // If both are set, concatenate the baseURI and tokenURI (via abi.encodePacked).
        if (bytes(_subgraphURI).length > 0) {
            return string(abi.encodePacked(base, _subgraphURI));
        }
        // If there is a baseURI but no tokenURI, concatenate the tokenID to the baseURI.
        return string(abi.encodePacked(base, HexStrings.toString(_tokenId)));
    }
}
 ------------
 ------------ END OF MAIN TARGET CONTRACT ------------ 

 ------------ ## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES ------------ 
// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity ^0.7.6;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable gas-increment-by-one

/**
 * @title HexStrings
 * @author Edge & Node
 * @notice Library for converting values to hexadecimal string representations
 * @dev Based on https://github.com/OpenZeppelin/openzeppelin-contracts/blob/8dd744fc1843d285c38e54e9d439dea7f6b93495/contracts/utils/Strings.sol
 */
library HexStrings {
    /// @dev Hexadecimal symbols used for string conversion
    bytes16 private constant _HEX_SYMBOLS = "0123456789abcdef";

    /// @notice Converts a `uint256` to its ASCII `string` hexadecimal representation.
    /// @param value The uint256 value to convert
    /// @return The hexadecimal string representation
    function toString(uint256 value) internal pure returns (string memory) {
        if (value == 0) {
            return "0x00";
        }
        uint256 temp = value;
        uint256 length = 0;
        while (temp != 0) {
            length++;
            temp >>= 8;
        }
        return toHexString(value, length);
    }

    /// @notice Converts a `uint256` to its ASCII `string` hexadecimal representation with fixed length.
    /// @param value The uint256 value to convert
    /// @param length The fixed length of the output string
    /// @return The hexadecimal string representation with fixed length
    function toHexString(uint256 value, uint256 length) internal pure returns (string memory) {
        bytes memory buffer = new bytes(2 * length + 2);
        buffer[0] = "0";
        buffer[1] = "x";
        for (uint256 i = 2 * length + 1; i > 1; --i) {
            buffer[i] = _HEX_SYMBOLS[value & 0xf];
            value >>= 4;
        }
        require(value == 0, "Strings: hex length insufficient");
        return string(buffer);
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity ^0.7.6 || 0.8.27 || 0.8.33;

/* solhint-disable gas-custom-errors */ // Cannot use custom errors with 0.7.6

/**
 * @title Graph Governance contract
 * @author Edge & Node
 * @notice All contracts that will be owned by a Governor entity should extend this contract.
 */
abstract contract Governed {
    // -- State --

    /**
     * @notice Address of the governor
     */
    address public governor;
    /**
     * @notice Address of the new governor that is pending acceptance
     */
    address public pendingGovernor;

    // -- Events --

    /**
     * @notice Emitted when a new owner/governor has been set, but is pending acceptance
     * @param from Previous pending governor address
     * @param to New pending governor address
     */
    event NewPendingOwnership(address indexed from, address indexed to);

    /**
     * @notice Emitted when a new owner/governor has accepted their role
     * @param from Previous governor address
     * @param to New governor address
     */
    event NewOwnership(address indexed from, address indexed to);

    /**
     * @dev Check if the caller is the governor.
     */
    modifier onlyGovernor() {
        require(msg.sender == governor, "Only Governor can call");
        _;
    }

    /**
     * @notice Initialize the governor for this contract
     * @param _initGovernor Address of the governor
     */
    function _initialize(address _initGovernor) internal {
        governor = _initGovernor;
    }

    /**
     * @notice Admin function to begin change of governor. The `_newGovernor` must call
     * `acceptOwnership` to finalize the transfer.
     * @param _newGovernor Address of new `governor`
     */
    function transferOwnership(address _newGovernor) external onlyGovernor {
        require(_newGovernor != address(0), "Governor must be set");

        address oldPendingGovernor = pendingGovernor;
        pendingGovernor = _newGovernor;

        emit NewPendingOwnership(oldPendingGovernor, pendingGovernor);
    }

    /**
     * @notice Admin function for pending governor to accept role and update governor.
     * This function must called by the pending governor.
     */
    function acceptOwnership() external {
        address oldPendingGovernor = pendingGovernor;

        require(
            oldPendingGovernor != address(0) && msg.sender == oldPendingGovernor,
            "Caller must be pending governor"
        );

        address oldGovernor = governor;

        governor = oldPendingGovernor;
        pendingGovernor = address(0);

        emit NewOwnership(oldGovernor, governor);
        emit NewPendingOwnership(oldPendingGovernor, pendingGovernor);
    }
}


## ------------ SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS ------------ 

## ------------ SUPPORTING CONTEXT: EXTERNAL LIBRARIES ------------ 

 ------------ END OF SUPPORTING CONTRACTS AND INTERFACES ------------ 


 ------------ ## DEPLOYMENT SCRIPTS ------------ 

