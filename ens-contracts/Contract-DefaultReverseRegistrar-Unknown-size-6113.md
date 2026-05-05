
 ------------ ## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT

pragma solidity ^0.8.4;

import {Ownable} from "@openzeppelin/contracts-v5/access/Ownable.sol";
import {MessageHashUtils} from "@openzeppelin/contracts-v5/utils/cryptography/MessageHashUtils.sol";
import {ERC165} from "@openzeppelin/contracts-v5/utils/introspection/ERC165.sol";

import {IDefaultReverseRegistrar} from "./IDefaultReverseRegistrar.sol";
import {StandaloneReverseRegistrar} from "./StandaloneReverseRegistrar.sol";
import {SignatureUtils} from "./SignatureUtils.sol";
import {Controllable} from "../root/Controllable.sol";

/// @title Default Reverse Registrar
/// @notice A default reverse registrar. Only one instance of this contract is deployed.
contract DefaultReverseRegistrar is
    IDefaultReverseRegistrar,
    ERC165,
    StandaloneReverseRegistrar,
    Controllable
{
    using SignatureUtils for bytes;
    using MessageHashUtils for bytes32;

    /// @inheritdoc IDefaultReverseRegistrar
    function setName(string calldata name) external {
        _setName(msg.sender, name);
    }

    /// @inheritdoc IDefaultReverseRegistrar
    function setNameForAddrWithSignature(
        address addr,
        uint256 signatureExpiry,
        string calldata name,
        bytes calldata signature
    ) external {
        // Follow ERC191 version 0 https://eips.ethereum.org/EIPS/eip-191
        bytes32 message = keccak256(
            abi.encodePacked(
                address(this),
                this.setNameForAddrWithSignature.selector,
                addr,
                signatureExpiry,
                name
            )
        ).toEthSignedMessageHash();

        signature.validateSignatureWithExpiry(addr, message, signatureExpiry);

        _setName(addr, name);
    }

    function setNameForAddr(
        address addr,
        string calldata name
    ) external onlyController {
        _setName(addr, name);
    }

    /// @inheritdoc ERC165
    function supportsInterface(
        bytes4 interfaceID
    ) public view override(ERC165, StandaloneReverseRegistrar) returns (bool) {
        return
            interfaceID == type(IDefaultReverseRegistrar).interfaceId ||
            super.supportsInterface(interfaceID);
    }
}
 ------------
 ------------ END OF MAIN TARGET CONTRACT ------------ 

 ------------ ## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES ------------ 
// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v5.1.0) (utils/cryptography/SignatureChecker.sol)

pragma solidity ^0.8.20;

import {ECDSA} from "./ECDSA.sol";
import {IERC1271} from "../../interfaces/IERC1271.sol";

/**
 * @dev Signature verification helper that can be used instead of `ECDSA.recover` to seamlessly support both ECDSA
 * signatures from externally owned accounts (EOAs) as well as ERC-1271 signatures from smart contract wallets like
 * Argent and Safe Wallet (previously Gnosis Safe).
 */
library SignatureChecker {
    /**
     * @dev Checks if a signature is valid for a given signer and data hash. If the signer is a smart contract, the
     * signature is validated against that smart contract using ERC-1271, otherwise it's validated using `ECDSA.recover`.
     *
     * NOTE: Unlike ECDSA signatures, contract signatures are revocable, and the outcome of this function can thus
     * change through time. It could return true at block N and false at block N+1 (or the opposite).
     */
    function isValidSignatureNow(address signer, bytes32 hash, bytes memory signature) internal view returns (bool) {
        if (signer.code.length == 0) {
            (address recovered, ECDSA.RecoverError err, ) = ECDSA.tryRecover(hash, signature);
            return err == ECDSA.RecoverError.NoError && recovered == signer;
        } else {
            return isValidERC1271SignatureNow(signer, hash, signature);
        }
    }

    /**
     * @dev Checks if a signature is valid for a given signer and data hash. The signature is validated
     * against the signer smart contract using ERC-1271.
     *
     * NOTE: Unlike ECDSA signatures, contract signatures are revocable, and the outcome of this function can thus
     * change through time. It could return true at block N and false at block N+1 (or the opposite).
     */
    function isValidERC1271SignatureNow(
        address signer,
        bytes32 hash,
        bytes memory signature
    ) internal view returns (bool) {
        (bool success, bytes memory result) = signer.staticcall(
            abi.encodeCall(IERC1271.isValidSignature, (hash, signature))
        );
        return (success &&
            result.length >= 32 &&
            abi.decode(result, (bytes32)) == bytes32(IERC1271.isValidSignature.selector));
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

import {SignatureChecker} from "@openzeppelin/contracts-v5/utils/cryptography/SignatureChecker.sol";

interface IUniversalSignatureValidator {
    function isValidSig(
        address _signer,
        bytes32 _hash,
        bytes calldata _signature
    ) external returns (bool);
}

/// @notice Utility functions for validating signatures with expiry.
library SignatureUtils {
    /// @notice The ERC6492 detection suffix.
    bytes32 private constant ERC6492_DETECTION_SUFFIX =
        0x6492649264926492649264926492649264926492649264926492649264926492;

    /// @notice The universal signature validator.
    IUniversalSignatureValidator public constant validator =
        IUniversalSignatureValidator(
            0x164af34fAF9879394370C7f09064127C043A35E9
        );

    /// @notice The signature is invalid
    error InvalidSignature();

    /// @notice The signature expiry is too high
    error SignatureExpiryTooHigh();

    /// @notice The signature has expired
    error SignatureExpired();

    /// @notice Validates a signature with expiry.
    ///
    /// @param signature The signature to validate.
    /// @param addr The address that signed the message.
    /// @param message The message that was signed.
    /// @param signatureExpiry The expiry of the signature.
    function validateSignatureWithExpiry(
        bytes calldata signature,
        address addr,
        bytes32 message,
        uint256 signatureExpiry
    ) internal {
        // ERC6492 check is done internally because UniversalSigValidator is not gas efficient.
        // We only want to use UniversalSigValidator for ERC6492 signatures.
        if (
            bytes32(signature[signature.length - 32:signature.length]) ==
            ERC6492_DETECTION_SUFFIX
        ) {
            if (!validator.isValidSig(addr, message, signature))
                revert InvalidSignature();
        } else {
            if (!SignatureChecker.isValidSignatureNow(addr, message, signature))
                revert InvalidSignature();
        }
        if (signatureExpiry < block.timestamp) revert SignatureExpired();
        if (signatureExpiry > block.timestamp + 1 hours)
            revert SignatureExpiryTooHigh();
    }
}

pragma solidity ^0.8.4;

import "@openzeppelin/contracts/access/Ownable.sol";

contract Controllable is Ownable {
    mapping(address => bool) public controllers;

    event ControllerChanged(address indexed controller, bool enabled);

    modifier onlyController() {
        require(
            controllers[msg.sender],
            "Controllable: Caller is not a controller"
        );
        _;
    }

    function setController(address controller, bool enabled) public onlyOwner {
        controllers[controller] = enabled;
        emit ControllerChanged(controller, enabled);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

/// @notice Interface for the Default Reverse Registrar.
interface IDefaultReverseRegistrar {
    /// @notice Sets the `nameForAddr()` record for the calling account.
    ///
    /// @param name The name to set.
    function setName(string memory name) external;

    /// @notice Sets the `nameForAddr()` record for the addr provided account using a signature.
    ///
    /// @param addr The address to set the name for.
    /// @param name The name to set.
    /// @param signatureExpiry Date when the signature expires.
    /// @param signature The signature from the addr.
    function setNameForAddrWithSignature(
        address addr,
        uint256 signatureExpiry,
        string memory name,
        bytes memory signature
    ) external;

    function setNameForAddr(address addr, string memory name) external;
}

// SPDX-License-Identifier: MIT

pragma solidity ^0.8.4;

import {ERC165} from "@openzeppelin/contracts-v5/utils/introspection/ERC165.sol";

import {IStandaloneReverseRegistrar} from "./IStandaloneReverseRegistrar.sol";

/// @title Standalone Reverse Registrar
/// @notice A standalone reverse registrar, detached from the ENS registry.
contract StandaloneReverseRegistrar is ERC165, IStandaloneReverseRegistrar {
    /// @notice The mapping of addresses to names.
    mapping(address => string) internal _names;

    /// @inheritdoc IStandaloneReverseRegistrar
    function nameForAddr(
        address addr
    ) external view returns (string memory name) {
        name = _names[addr];
    }

    /// @notice Sets the name for an address.
    ///
    /// @dev Authorisation should be checked before calling.
    ///
    /// @param addr The address to set the name for.
    /// @param name The name to set.
    function _setName(address addr, string calldata name) internal {
        _names[addr] = name;
        emit NameForAddrChanged(addr, name);
    }

    /// @inheritdoc ERC165
    function supportsInterface(
        bytes4 interfaceID
    ) public view virtual override(ERC165) returns (bool) {
        return
            interfaceID == type(IStandaloneReverseRegistrar).interfaceId ||
            super.supportsInterface(interfaceID);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

/// @notice Interface for a standalone reverse registrar.
interface IStandaloneReverseRegistrar {
    /// @notice Emitted when the name for an address is changed.
    ///
    /// @param addr The address of the reverse record.
    /// @param name The name of the reverse record.
    event NameForAddrChanged(address indexed addr, string name);

    /// @notice Returns the name for an address.
    ///
    /// @param addr The address to get the name for.
    /// @return The name for the address.
    function nameForAddr(address addr) external view returns (string memory);
}


## ------------ SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS ------------ 
// SPDX-License-Identifier: MIT

pragma solidity ^0.8.4;

import {Ownable} from "@openzeppelin/contracts-v5/access/Ownable.sol";
import {MessageHashUtils} from "@openzeppelin/contracts-v5/utils/cryptography/MessageHashUtils.sol";
import {ERC165} from "@openzeppelin/contracts-v5/utils/introspection/ERC165.sol";

import {IL2ReverseRegistrar} from "./IL2ReverseRegistrar.sol";
import {StandaloneReverseRegistrar} from "./StandaloneReverseRegistrar.sol";
import {SignatureUtils} from "./SignatureUtils.sol";

/// @title L2 Reverse Registrar
/// @notice An L2 Reverse Registrar. Deployed to each L2 chain.
contract L2ReverseRegistrar is
    IL2ReverseRegistrar,
    ERC165,
    StandaloneReverseRegistrar
{
    using SignatureUtils for bytes;
    using MessageHashUtils for bytes32;

    /// @notice The coin type for the chain this contract is deployed to.
    uint256 public immutable coinType;

    /// @notice Thrown when the specified address is not the owner of the contract
    error NotOwnerOfContract();

    /// @notice Thrown when the coin type is not found in the provided array
    error CoinTypeNotFound();

    /// @notice The caller is not authorised to perform the action
    error Unauthorised();

    /// @notice Checks if the caller is authorised
    ///
    /// @param addr The address to check.
    modifier authorised(address addr) {
        if (addr != msg.sender && !_ownsContract(addr, msg.sender)) {
            revert Unauthorised();
        }
        _;
    }

    /// @notice Ensures the coin type of the contract is included in the provided array
    ///
    /// @param coinTypes The coin types to check.
    modifier validCoinTypes(uint256[] calldata coinTypes) {
        _validateCoinTypes(coinTypes);
        _;
    }

    /// @notice Initialises the contract by setting the coin type.
    ///
    /// @param coinType_ The cointype converted from the chainId of the chain this contract is deployed to.
    constructor(uint256 coinType_) {
        coinType = coinType_;
    }

    /// @inheritdoc IL2ReverseRegistrar
    function setName(string calldata name) external authorised(msg.sender) {
        _setName(msg.sender, name);
    }

    /// @inheritdoc IL2ReverseRegistrar
    function setNameForAddr(
        address addr,
        string calldata name
    ) external authorised(addr) {
        _setName(addr, name);
    }

    /// @inheritdoc IL2ReverseRegistrar
    function setNameForAddrWithSignature(
        address addr,
        uint256 signatureExpiry,
        string calldata name,
        uint256[] calldata coinTypes,
        bytes calldata signature
    ) external validCoinTypes(coinTypes) {
        // Follow ERC191 version 0 https://eips.ethereum.org/EIPS/eip-191
        bytes32 message = keccak256(
            abi.encodePacked(
                address(this),
                this.setNameForAddrWithSignature.selector,
                addr,
                signatureExpiry,
                name,
                coinTypes
            )
        ).toEthSignedMessageHash();

        signature.validateSignatureWithExpiry(addr, message, signatureExpiry);

        _setName(addr, name);
    }

    /// @inheritdoc IL2ReverseRegistrar
    function setNameForOwnableWithSignature(
        address contractAddr,
        address owner,
        uint256 signatureExpiry,
        string calldata name,
        uint256[] calldata coinTypes,
        bytes calldata signature
    ) external validCoinTypes(coinTypes) {
        // Follow ERC191 version 0 https://eips.ethereum.org/EIPS/eip-191
        bytes32 message = keccak256(
            abi.encodePacked(
                address(this),
                this.setNameForOwnableWithSignature.selector,
                contractAddr,
                owner,
                signatureExpiry,
                name,
                coinTypes
            )
        ).toEthSignedMessageHash();

        if (!_ownsContract(contractAddr, owner)) revert NotOwnerOfContract();

        signature.validateSignatureWithExpiry(owner, message, signatureExpiry);

        _setName(contractAddr, name);
    }

    /// @notice Checks if the provided contractAddr is a contract and is owned by the
    ///         provided addr.
    ///
    /// @param contractAddr The address of the contract to check.
    /// @param addr The address to check ownership against.
    function _ownsContract(
        address contractAddr,
        address addr
    ) internal view returns (bool) {
        if (contractAddr.code.length == 0) return false;
        try Ownable(contractAddr).owner() returns (address owner) {
            return owner == addr;
        } catch {
            return false;
        }
    }

    /// @notice Ensures the coin type for the contract is included in the provided array.
    ///
    /// @param coinTypes The coin types to check.
    function _validateCoinTypes(uint256[] calldata coinTypes) internal view {
        for (uint256 i = 0; i < coinTypes.length; i++) {
            if (coinTypes[i] == coinType) return;
        }

        revert CoinTypeNotFound();
    }

    /// @inheritdoc ERC165
    function supportsInterface(
        bytes4 interfaceID
    ) public view override(ERC165, StandaloneReverseRegistrar) returns (bool) {
        return
            interfaceID == type(IL2ReverseRegistrar).interfaceId ||
            super.supportsInterface(interfaceID);
    }
}

//SPDX-License-Identifier: MIT
pragma solidity >=0.8.17 <0.9.0;

import "../registry/ENS.sol";
import "./profiles/ABIResolver.sol";
import "./profiles/AddrResolver.sol";
import "./profiles/ContentHashResolver.sol";
import "./profiles/DataResolver.sol";
import "./profiles/DNSResolver.sol";
import "./profiles/InterfaceResolver.sol";
import "./profiles/NameResolver.sol";
import "./profiles/PubkeyResolver.sol";
import "./profiles/TextResolver.sol";
import "./Multicallable.sol";
import {ReverseClaimer} from "../reverseRegistrar/ReverseClaimer.sol";
import {INameWrapper} from "../wrapper/INameWrapper.sol";

/// A simple resolver anyone can use; only allows the owner of a node to set its
/// address.
contract PublicResolver is
    Multicallable,
    ABIResolver,
    AddrResolver,
    ContentHashResolver,
    DataResolver,
    DNSResolver,
    InterfaceResolver,
    NameResolver,
    PubkeyResolver,
    TextResolver,
    ReverseClaimer
{
    ENS immutable ens;
    INameWrapper immutable nameWrapper;
    address immutable trustedETHController;
    address immutable trustedReverseRegistrar;

    /// A mapping of operators. An address that is authorised for an address
    /// may make any changes to the name that the owner could, but may not update
    /// the set of authorisations.
    /// (owner, operator) => approved
    mapping(address => mapping(address => bool)) private _operatorApprovals;

    /// A mapping of delegates. A delegate that is authorised by an owner
    /// for a name may make changes to the name's resolver, but may not update
    /// the set of token approvals.
    /// (owner, name, delegate) => approved
    mapping(address => mapping(bytes32 => mapping(address => bool)))
        private _tokenApprovals;

    // Logged when an operator is added or removed.
    event ApprovalForAll(
        address indexed owner,
        address indexed operator,
        bool approved
    );

    // Logged when a delegate is approved or  an approval is revoked.
    event Approved(
        address owner,
        bytes32 indexed node,
        address indexed delegate,
        bool indexed approved
    );

    constructor(
        ENS _ens,
        INameWrapper wrapperAddress,
        address _trustedETHController,
        address _trustedReverseRegistrar
    ) ReverseClaimer(_ens, msg.sender) {
        ens = _ens;
        nameWrapper = wrapperAddress;
        trustedETHController = _trustedETHController;
        trustedReverseRegistrar = _trustedReverseRegistrar;
    }

    /// @dev See {IERC1155-setApprovalForAll}.
    function setApprovalForAll(address operator, bool approved) external {
        require(
            msg.sender != operator,
            "ERC1155: setting approval status for self"
        );

        _operatorApprovals[msg.sender][operator] = approved;
        emit ApprovalForAll(msg.sender, operator, approved);
    }

    /// @dev See {IERC1155-isApprovedForAll}.
    function isApprovedForAll(
        address account,
        address operator
    ) public view returns (bool) {
        return _operatorApprovals[account][operator];
    }

    /// @dev Approve a delegate to be able to updated records on a node.
    function approve(bytes32 node, address delegate, bool approved) external {
        require(msg.sender != delegate, "Setting delegate status for self");

        _tokenApprovals[msg.sender][node][delegate] = approved;
        emit Approved(msg.sender, node, delegate, approved);
    }

    /// @dev Check to see if the delegate has been approved by the owner for the node.
    function isApprovedFor(
        address owner,
        bytes32 node,
        address delegate
    ) public view returns (bool) {
        return _tokenApprovals[owner][node][delegate];
    }

    function isAuthorised(bytes32 node) internal view override returns (bool) {
        if (
            msg.sender == trustedETHController ||
            msg.sender == trustedReverseRegistrar
        ) {
            return true;
        }
        address owner = ens.owner(node);
        if (owner == address(nameWrapper)) {
            owner = nameWrapper.ownerOf(uint256(node));
        }
        return
            owner == msg.sender ||
            isApprovedForAll(owner, msg.sender) ||
            isApprovedFor(owner, node, msg.sender);
    }

    function supportsInterface(
        bytes4 interfaceID
    )
        public
        view
        override(
            Multicallable,
            ABIResolver,
            AddrResolver,
            ContentHashResolver,
            DataResolver,
            DNSResolver,
            InterfaceResolver,
            NameResolver,
            PubkeyResolver,
            TextResolver
        )
        returns (bool)
    {
        return super.supportsInterface(interfaceID);
    }
}

//SPDX-License-Identifier: MIT
pragma solidity >=0.8.4;

import "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import "./profiles/IABIResolver.sol";
import "./profiles/IAddressResolver.sol";
import "./profiles/IAddrResolver.sol";
import "./profiles/IContentHashResolver.sol";
import "./profiles/IDNSRecordResolver.sol";
import "./profiles/IDNSZoneResolver.sol";
import "./profiles/IInterfaceResolver.sol";
import "./profiles/INameResolver.sol";
import "./profiles/IPubkeyResolver.sol";
import "./profiles/ITextResolver.sol";
import "./profiles/IExtendedResolver.sol";

/// A generic resolver interface which includes all the functions including the ones deprecated
interface Resolver is
    IERC165,
    IABIResolver,
    IAddressResolver,
    IAddrResolver,
    IContentHashResolver,
    IDNSRecordResolver,
    IDNSZoneResolver,
    IInterfaceResolver,
    INameResolver,
    IPubkeyResolver,
    ITextResolver,
    IExtendedResolver
{
    /* Deprecated events */
    event ContentChanged(bytes32 indexed node, bytes32 hash);

    function setApprovalForAll(address, bool) external;

    function approve(bytes32 node, address delegate, bool approved) external;

    function isApprovedForAll(address account, address operator) external;

    function isApprovedFor(
        address owner,
        bytes32 node,
        address delegate
    ) external;

    function setABI(
        bytes32 node,
        uint256 contentType,
        bytes calldata data
    ) external;

    function setAddr(bytes32 node, address addr) external;

    function setAddr(bytes32 node, uint256 coinType, bytes calldata a) external;

    function setContenthash(bytes32 node, bytes calldata hash) external;

    function setDnsrr(bytes32 node, bytes calldata data) external;

    function setName(bytes32 node, string calldata _name) external;

    function setPubkey(bytes32 node, bytes32 x, bytes32 y) external;

    function setText(
        bytes32 node,
        string calldata key,
        string calldata value
    ) external;

    function setInterface(
        bytes32 node,
        bytes4 interfaceID,
        address implementer
    ) external;

    function multicall(
        bytes[] calldata data
    ) external returns (bytes[] memory results);

    function multicallWithNodeCheck(
        bytes32 nodehash,
        bytes[] calldata data
    ) external returns (bytes[] memory results);

    /* Deprecated functions */
    function content(bytes32 node) external view returns (bytes32);

    function multihash(bytes32 node) external view returns (bytes memory);

    function setContent(bytes32 node, bytes32 hash) external;

    function setMultihash(bytes32 node, bytes calldata hash) external;
}

// SPDX-License-Identifier: MIT

pragma solidity ^0.8.4;

import {Ownable} from "@openzeppelin/contracts-v5/access/Ownable.sol";

import {L2ReverseRegistrar} from "./L2ReverseRegistrar.sol";
import {INameResolver} from "../resolvers/profiles/INameResolver.sol";
import {AddressUtils} from "../utils/AddressUtils.sol";

/// @notice An L2 Reverse Registrar that allows migrating from a prior resolver.
contract L2ReverseRegistrarWithMigration is L2ReverseRegistrar, Ownable {
    using AddressUtils for address;

    /// @notice The old reverse resolver to migrate from
    INameResolver immutable oldReverseResolver;

    /// @notice The parent node of reverse nodes. The convention is '${coinType}.reverse'
    bytes32 immutable parentNode;

    /// @notice Initialises the contract by setting the parent node, coin type, and old reverse resolver.
    ///
    /// @param coinType_ The cointype converted from the chainId of the chain this contract is deployed to.
    /// @param owner_ The initial owner of the contract.
    /// @param parentNode_ The parent node to set. The convention is '${coinType}.reverse'.
    /// @param oldReverseResolver_ The old reverse resolver.
    constructor(
        uint256 coinType_,
        address owner_,
        bytes32 parentNode_,
        INameResolver oldReverseResolver_
    ) L2ReverseRegistrar(coinType_) Ownable(owner_) {
        parentNode = parentNode_;
        oldReverseResolver = oldReverseResolver_;
    }

    /// @notice Migrates the names from the old reverse resolver to the new one.
    ///         Only callable by the owner.
    ///
    /// @param addresses The addresses to migrate.
    function batchSetName(address[] calldata addresses) external onlyOwner {
        for (uint256 i = 0; i < addresses.length; i++) {
            // namehash of `[addresses[i]].[coinType].reverse`
            bytes32 node = keccak256(
                abi.encodePacked(parentNode, addresses[i].sha3HexAddress())
            );
            string memory name = oldReverseResolver.name(node);

            // equivalent to _setName(addresses[i], name);
            // internal because the name value isn't in calldata
            _names[addresses[i]] = name;
            emit NameForAddrChanged(addresses[i], name);
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.8.4;

import "../ResolverBase.sol";
import "./INameResolver.sol";

abstract contract NameResolver is INameResolver, ResolverBase {
    mapping(uint64 => mapping(bytes32 => string)) versionable_names;

    /// Sets the name associated with an ENS node, for reverse records.
    /// May only be called by the owner of that node in the ENS registry.
    /// @param node The node to update.
    function setName(
        bytes32 node,
        string calldata newName
    ) external virtual authorised(node) {
        versionable_names[recordVersions[node]][node] = newName;
        emit NameChanged(node, newName);
    }

    /// Returns the name associated with an ENS node, for reverse records.
    /// Defined in EIP181.
    /// @param node The ENS node to query.
    /// @return The associated name.
    function name(
        bytes32 node
    ) external view virtual override returns (string memory) {
        return versionable_names[recordVersions[node]][node];
    }

    function supportsInterface(
        bytes4 interfaceID
    ) public view virtual override returns (bool) {
        return
            interfaceID == type(INameResolver).interfaceId ||
            super.supportsInterface(interfaceID);
    }
}

//SPDX-License-Identifier: MIT
pragma solidity >=0.8.4;
import "@openzeppelin/contracts/access/Ownable.sol";
import "./profiles/ABIResolver.sol";
import "./profiles/AddrResolver.sol";
import "./profiles/ContentHashResolver.sol";
import "./profiles/DNSResolver.sol";
import "./profiles/InterfaceResolver.sol";
import "./profiles/NameResolver.sol";
import "./profiles/PubkeyResolver.sol";
import "./profiles/TextResolver.sol";
import "./profiles/ExtendedResolver.sol";

/// A simple resolver anyone can use; only allows the owner of a node to set its
/// address.
contract OwnedResolver is
    Ownable,
    ABIResolver,
    AddrResolver,
    ContentHashResolver,
    DNSResolver,
    InterfaceResolver,
    NameResolver,
    PubkeyResolver,
    TextResolver,
    ExtendedResolver
{
    function isAuthorised(bytes32) internal view override returns (bool) {
        return msg.sender == owner();
    }

    function supportsInterface(
        bytes4 interfaceID
    )
        public
        view
        virtual
        override(
            ABIResolver,
            AddrResolver,
            ContentHashResolver,
            DNSResolver,
            InterfaceResolver,
            NameResolver,
            PubkeyResolver,
            TextResolver
        )
        returns (bool)
    {
        return super.supportsInterface(interfaceID);
    }
}


## ------------ SUPPORTING CONTEXT: EXTERNAL LIBRARIES ------------ 

 ------------ END OF SUPPORTING CONTRACTS AND INTERFACES ------------ 


 ------------ ## DEPLOYMENT SCRIPTS ------------ 

