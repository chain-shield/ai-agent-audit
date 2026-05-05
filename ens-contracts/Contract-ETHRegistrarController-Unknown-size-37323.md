
 ------------ ## *MAIN TARGET CONTRACT* TO REVIEW

//SPDX-License-Identifier: MIT
pragma solidity ~0.8.17;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {ERC165} from "@openzeppelin/contracts/utils/introspection/ERC165.sol";
import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";

import {BaseRegistrarImplementation} from "./BaseRegistrarImplementation.sol";
import {StringUtils} from "../utils/StringUtils.sol";
import {Resolver} from "../resolvers/Resolver.sol";
import {ENS} from "../registry/ENS.sol";
import {IReverseRegistrar} from "../reverseRegistrar/IReverseRegistrar.sol";
import {IDefaultReverseRegistrar} from "../reverseRegistrar/IDefaultReverseRegistrar.sol";
import {IETHRegistrarController, IPriceOracle} from "./IETHRegistrarController.sol";
import {ERC20Recoverable} from "../utils/ERC20Recoverable.sol";

/// @dev A registrar controller for registering and renewing names at fixed cost.
contract ETHRegistrarController is
    Ownable,
    IETHRegistrarController,
    ERC165,
    ERC20Recoverable
{
    using StringUtils for *;

    /// @notice The bitmask for the Ethereum reverse record.
    uint8 constant REVERSE_RECORD_ETHEREUM_BIT = 1;

    /// @notice The bitmask for the default reverse record.
    uint8 constant REVERSE_RECORD_DEFAULT_BIT = 2;

    /// @notice The minimum duration for a registration.
    uint256 public constant MIN_REGISTRATION_DURATION = 28 days;

    // @notice The node (i.e. namehash) for the eth TLD.
    bytes32 private constant ETH_NODE =
        0x93cdeb708b7545dc668eb9280176169d1c33cfd8ed6f04690a0bcc88a93fc4ae;

    /// @notice The maximum expiry time for a registration.
    uint64 private constant MAX_EXPIRY = type(uint64).max;

    /// @notice The ENS registry.
    ENS public immutable ens;

    // @notice The base registrar implementation for the eth TLD.
    BaseRegistrarImplementation immutable base;

    /// @notice The minimum time a commitment must exist to be valid.
    uint256 public immutable minCommitmentAge;

    /// @notice The maximum time a commitment can exist to be valid.
    uint256 public immutable maxCommitmentAge;

    /// @notice The registrar for addr.reverse. (i.e. reverse for coinType 60)
    IReverseRegistrar public immutable reverseRegistrar;

    /// @notice The registrar for default.reverse. (i.e. fallback reverse for all EVM chains)
    IDefaultReverseRegistrar public immutable defaultReverseRegistrar;

    /// @notice The price oracle for the eth TLD.
    IPriceOracle public immutable prices;

    /// @notice A mapping of commitments to their timestamp.
    mapping(bytes32 => uint256) public commitments;

    /// @notice Thrown when a commitment is not found.
    error CommitmentNotFound(bytes32 commitment);

    /// @notice Thrown when a commitment is too new.
    error CommitmentTooNew(
        bytes32 commitment,
        uint256 minimumCommitmentTimestamp,
        uint256 currentTimestamp
    );

    /// @notice Thrown when a commitment is too old.
    error CommitmentTooOld(
        bytes32 commitment,
        uint256 maximumCommitmentTimestamp,
        uint256 currentTimestamp
    );

    /// @notice Thrown when a name is not available to register.
    error NameNotAvailable(string name);

    /// @notice Thrown when the duration supplied for a registration is too short.
    error DurationTooShort(uint256 duration);

    /// @notice Thrown when data is supplied for a registration without a resolver.
    error ResolverRequiredWhenDataSupplied();

    /// @notice Thrown when a reverse record is requested without a resolver.
    error ResolverRequiredForReverseRecord();

    /// @notice Thrown when a matching unexpired commitment exists.
    error UnexpiredCommitmentExists(bytes32 commitment);

    /// @notice Thrown when the value sent for a registration is insufficient.
    error InsufficientValue();

    /// @notice Thrown when the maximum commitment age is too low.
    error MaxCommitmentAgeTooLow();

    /// @notice Thrown when the maximum commitment age is too high.
    error MaxCommitmentAgeTooHigh();

    /// @notice Emitted when a name is registered.
    ///
    /// @param label The label of the name.
    /// @param labelhash The keccak256 hash of the label.
    /// @param owner The owner of the name.
    /// @param baseCost The base cost of the name.
    /// @param premium The premium cost of the name.
    /// @param expires The expiry time of the name.
    /// @param referrer The referrer of the registration.
    event NameRegistered(
        string label,
        bytes32 indexed labelhash,
        address indexed owner,
        uint256 baseCost,
        uint256 premium,
        uint256 expires,
        bytes32 referrer
    );

    /// @notice Emitted when a name is renewed.
    ///
    /// @param label The label of the name.
    /// @param labelhash The keccak256 hash of the label.
    /// @param cost The cost of the name.
    /// @param expires The expiry time of the name.
    /// @param referrer The referrer of the registration.
    event NameRenewed(
        string label,
        bytes32 indexed labelhash,
        uint256 cost,
        uint256 expires,
        bytes32 referrer
    );

    /// @notice Constructor for the ETHRegistrarController.
    ///
    /// @param _base The base registrar implementation for the eth TLD.
    /// @param _prices The price oracle for the eth TLD.
    /// @param _minCommitmentAge The minimum time a commitment must exist to be valid.
    /// @param _maxCommitmentAge The maximum time a commitment can exist to be valid.
    /// @param _reverseRegistrar The registrar for addr.reverse.
    /// @param _defaultReverseRegistrar The registrar for default.reverse.
    /// @param _ens The ENS registry.
    constructor(
        BaseRegistrarImplementation _base,
        IPriceOracle _prices,
        uint256 _minCommitmentAge,
        uint256 _maxCommitmentAge,
        IReverseRegistrar _reverseRegistrar,
        IDefaultReverseRegistrar _defaultReverseRegistrar,
        ENS _ens
    ) {
        if (_maxCommitmentAge <= _minCommitmentAge)
            revert MaxCommitmentAgeTooLow();

        if (_maxCommitmentAge > block.timestamp)
            revert MaxCommitmentAgeTooHigh();

        ens = _ens;
        base = _base;
        prices = _prices;
        minCommitmentAge = _minCommitmentAge;
        maxCommitmentAge = _maxCommitmentAge;
        reverseRegistrar = _reverseRegistrar;
        defaultReverseRegistrar = _defaultReverseRegistrar;
    }

    /// @notice Returns the price of a registration for the given label and duration.
    ///
    /// @param label The label of the name.
    /// @param duration The duration of the registration.
    /// @return price The price of the registration.
    function rentPrice(
        string calldata label,
        uint256 duration
    ) public view override returns (IPriceOracle.Price memory price) {
        bytes32 labelhash = keccak256(bytes(label));
        price = _rentPrice(label, labelhash, duration);
    }

    /// @notice Returns true if the label is valid for registration.
    ///
    /// @param label The label to check.
    /// @return True if the label is valid, false otherwise.
    function valid(string calldata label) public pure returns (bool) {
        return label.strlen() >= 3;
    }

    /// @notice Returns true if the label is valid and available for registration.
    ///
    /// @param label The label to check.
    /// @return True if the label is valid and available, false otherwise.
    function available(
        string calldata label
    ) public view override returns (bool) {
        bytes32 labelhash = keccak256(bytes(label));
        return _available(label, labelhash);
    }

    /// @notice Returns the commitment for a registration.
    ///
    /// @param registration The registration to make a commitment for.
    /// @return commitment The commitment for the registration.
    function makeCommitment(
        Registration calldata registration
    ) public pure override returns (bytes32 commitment) {
        if (registration.data.length > 0 && registration.resolver == address(0))
            revert ResolverRequiredWhenDataSupplied();

        if (
            registration.reverseRecord != 0 &&
            registration.resolver == address(0)
        ) revert ResolverRequiredForReverseRecord();

        if (registration.duration < MIN_REGISTRATION_DURATION)
            revert DurationTooShort(registration.duration);

        return keccak256(abi.encode(registration));
    }

    /// @notice Commits a registration.
    ///
    /// @param commitment The commitment to commit.
    function commit(bytes32 commitment) public override {
        if (commitments[commitment] + maxCommitmentAge >= block.timestamp) {
            revert UnexpiredCommitmentExists(commitment);
        }
        commitments[commitment] = block.timestamp;
    }

    /// @notice Registers a name.
    ///
    /// @param registration The registration to register.
    /// @param registration.label The label of the name.
    /// @param registration.owner The owner of the name.
    /// @param registration.duration The duration of the registration.
    /// @param registration.resolver The resolver for the name.
    /// @param registration.data The data for the name.
    /// @param registration.reverseRecord Which reverse record(s) to set.
    /// @param registration.referrer The referrer of the registration.
    function register(
        Registration calldata registration
    ) public payable override {
        bytes32 labelhash = keccak256(bytes(registration.label));
        IPriceOracle.Price memory price = _rentPrice(
            registration.label,
            labelhash,
            registration.duration
        );
        uint256 totalPrice = price.base + price.premium;
        if (msg.value < totalPrice) revert InsufficientValue();

        if (!_available(registration.label, labelhash))
            revert NameNotAvailable(registration.label);

        bytes32 commitment = makeCommitment(registration);
        uint256 commitmentTimestamp = commitments[commitment];

        // Require an old enough commitment.
        if (commitmentTimestamp + minCommitmentAge > block.timestamp)
            revert CommitmentTooNew(
                commitment,
                commitmentTimestamp + minCommitmentAge,
                block.timestamp
            );

        // If the commitment is too old, or the name is registered, stop
        if (commitmentTimestamp + maxCommitmentAge <= block.timestamp) {
            if (commitmentTimestamp == 0) revert CommitmentNotFound(commitment);
            revert CommitmentTooOld(
                commitment,
                commitmentTimestamp + maxCommitmentAge,
                block.timestamp
            );
        }

        delete (commitments[commitment]);

        uint256 expires;

        if (registration.resolver == address(0)) {
            expires = base.register(
                uint256(labelhash),
                registration.owner,
                registration.duration
            );
        } else {
            expires = base.register(
                uint256(labelhash),
                address(this),
                registration.duration
            );

            bytes32 namehash = keccak256(abi.encodePacked(ETH_NODE, labelhash));
            ens.setRecord(
                namehash,
                registration.owner,
                registration.resolver,
                0
            );
            if (registration.data.length > 0)
                Resolver(registration.resolver).multicallWithNodeCheck(
                    namehash,
                    registration.data
                );

            base.transferFrom(
                address(this),
                registration.owner,
                uint256(labelhash)
            );

            if (registration.reverseRecord & REVERSE_RECORD_ETHEREUM_BIT != 0)
                reverseRegistrar.setNameForAddr(
                    msg.sender,
                    msg.sender,
                    registration.resolver,
                    string.concat(registration.label, ".eth")
                );
            if (registration.reverseRecord & REVERSE_RECORD_DEFAULT_BIT != 0)
                defaultReverseRegistrar.setNameForAddr(
                    msg.sender,
                    string.concat(registration.label, ".eth")
                );
        }

        emit NameRegistered(
            registration.label,
            labelhash,
            registration.owner,
            price.base,
            price.premium,
            expires,
            registration.referrer
        );

        if (msg.value > totalPrice)
            payable(msg.sender).transfer(msg.value - totalPrice);
    }

    /// @notice Renews a name.
    ///
    /// @param label The label of the name.
    /// @param duration The duration of the registration.
    /// @param referrer The referrer of the registration.
    function renew(
        string calldata label,
        uint256 duration,
        bytes32 referrer
    ) external payable override {
        bytes32 labelhash = keccak256(bytes(label));

        IPriceOracle.Price memory price = _rentPrice(
            label,
            labelhash,
            duration
        );
        if (msg.value < price.base) revert InsufficientValue();

        uint256 expires = base.renew(uint256(labelhash), duration);

        emit NameRenewed(label, labelhash, price.base, expires, referrer);

        if (msg.value > price.base)
            payable(msg.sender).transfer(msg.value - price.base);
    }

    /// @notice Withdraws the balance of the contract to the owner.
    function withdraw() public {
        payable(owner()).transfer(address(this).balance);
    }

    /// @inheritdoc IERC165
    function supportsInterface(
        bytes4 interfaceID
    ) public view override returns (bool) {
        return
            interfaceID == type(IETHRegistrarController).interfaceId ||
            super.supportsInterface(interfaceID);
    }

    /* Internal functions */

    function _rentPrice(
        string calldata label,
        bytes32 labelhash,
        uint256 duration
    ) internal view returns (IPriceOracle.Price memory price) {
        price = prices.price(
            label,
            base.nameExpires(uint256(labelhash)),
            duration
        );
    }

    function _available(
        string calldata label,
        bytes32 labelhash
    ) internal view returns (bool) {
        return valid(label) && base.available(uint256(labelhash));
    }
}
 ------------
 ------------ END OF MAIN TARGET CONTRACT ------------ 

 ------------ ## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES ------------ 
pragma solidity ^0.8.4;

contract Ownable {
    address public owner;

    event OwnershipTransferred(
        address indexed previousOwner,
        address indexed newOwner
    );

    modifier onlyOwner() {
        require(isOwner(msg.sender));
        _;
    }

    constructor() public {
        owner = msg.sender;
    }

    function transferOwnership(address newOwner) public onlyOwner {
        emit OwnershipTransferred(owner, newOwner);
        owner = newOwner;
    }

    function isOwner(address addr) public view returns (bool) {
        return owner == addr;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.8.4;

library StringUtils {
    /// @dev Returns the length of a given string
    /// @param s The string to measure the length of
    /// @return The length of the input string
    function strlen(string memory s) internal pure returns (uint256) {
        uint256 len;
        uint256 i = 0;
        uint256 bytelength = bytes(s).length;
        for (len = 0; i < bytelength; len++) {
            bytes1 b = bytes(s)[i];
            if (b < 0x80) {
                i += 1;
            } else if (b < 0xE0) {
                i += 2;
            } else if (b < 0xF0) {
                i += 3;
            } else if (b < 0xF8) {
                i += 4;
            } else if (b < 0xFC) {
                i += 5;
            } else {
                i += 6;
            }
        }
        return len;
    }

    /// @dev Escapes special characters in a given string
    /// @param str The string to escape
    /// @return The escaped string
    function escape(string memory str) internal pure returns (string memory) {
        bytes memory strBytes = bytes(str);
        uint extraChars = 0;

        // count extra space needed for escaping
        for (uint i = 0; i < strBytes.length; i++) {
            if (_needsEscaping(strBytes[i])) {
                extraChars++;
            }
        }

        // allocate buffer with the exact size needed
        bytes memory buffer = new bytes(strBytes.length + extraChars);
        uint index = 0;

        // escape characters
        for (uint i = 0; i < strBytes.length; i++) {
            if (_needsEscaping(strBytes[i])) {
                buffer[index++] = "\\";
                buffer[index++] = _getEscapedChar(strBytes[i]);
            } else {
                buffer[index++] = strBytes[i];
            }
        }

        return string(buffer);
    }

    // determine if a character needs escaping
    function _needsEscaping(bytes1 char) private pure returns (bool) {
        return
            char == '"' ||
            char == "/" ||
            char == "\\" ||
            char == "\n" ||
            char == "\r" ||
            char == "\t";
    }

    // get the escaped character
    function _getEscapedChar(bytes1 char) private pure returns (bytes1) {
        if (char == "\n") return "n";
        if (char == "\r") return "r";
        if (char == "\t") return "t";
        return char;
    }
}

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
pragma solidity >=0.8.4;

interface ITextResolver {
    event TextChanged(
        bytes32 indexed node,
        string indexed indexedKey,
        string key,
        string value
    );

    /// Returns the text data associated with an ENS node and key.
    /// @param node The ENS node to query.
    /// @param key The text data key to query.
    /// @return The associated text data.
    function text(
        bytes32 node,
        string calldata key
    ) external view returns (string memory);
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.8.4;

interface IPubkeyResolver {
    event PubkeyChanged(bytes32 indexed node, bytes32 x, bytes32 y);

    /// Returns the SECP256k1 public key associated with an ENS node.
    /// Defined in EIP 619.
    /// @param node The ENS node to query
    /// @return x The X coordinate of the curve point for the public key.
    /// @return y The Y coordinate of the curve point for the public key.
    function pubkey(bytes32 node) external view returns (bytes32 x, bytes32 y);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

interface IExtendedResolver {
    function resolve(
        bytes memory name,
        bytes memory data
    ) external view returns (bytes memory);
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
pragma solidity >=0.8.4;

interface IInterfaceResolver {
    event InterfaceChanged(
        bytes32 indexed node,
        bytes4 indexed interfaceID,
        address implementer
    );

    /// Returns the address of a contract that implements the specified interface for this name.
    /// If an implementer has not been set for this interfaceID and name, the resolver will query
    /// the contract at `addr()`. If `addr()` is set, a contract exists at that address, and that
    /// contract implements EIP165 and returns `true` for the specified interfaceID, its address
    /// will be returned.
    /// @param node The ENS node to query.
    /// @param interfaceID The EIP 165 interface ID to check for.
    /// @return The address that implements this interface, or 0 if the interface is unsupported.
    function interfaceImplementer(
        bytes32 node,
        bytes4 interfaceID
    ) external view returns (address);
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.8.4;

interface IDNSRecordResolver {
    // DNSRecordChanged is emitted whenever a given node/name/resource's RRSET is updated.
    event DNSRecordChanged(
        bytes32 indexed node,
        bytes name,
        uint16 resource,
        bytes record
    );
    // DNSRecordDeleted is emitted whenever a given node/name/resource's RRSET is deleted.
    event DNSRecordDeleted(bytes32 indexed node, bytes name, uint16 resource);

    /// Obtain a DNS record.
    /// @param node the namehash of the node for which to fetch the record
    /// @param name the keccak-256 hash of the fully-qualified name for which to fetch the record
    /// @param resource the ID of the resource as per https://en.wikipedia.org/wiki/List_of_DNS_record_types
    /// @return the DNS record in wire format if present, otherwise empty
    function dnsRecord(
        bytes32 node,
        bytes32 name,
        uint16 resource
    ) external view returns (bytes memory);
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.8.4;

interface IABIResolver {
    event ABIChanged(bytes32 indexed node, uint256 indexed contentType);

    /// Returns the ABI associated with an ENS node.
    /// Defined in EIP205.
    /// @param node The ENS node to query
    /// @param contentTypes A bitwise OR of the ABI formats accepted by the caller.
    /// @return contentType The content type of the return value
    /// @return data The ABI data
    function ABI(
        bytes32 node,
        uint256 contentTypes
    ) external view returns (uint256, bytes memory);
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.8.4;

interface IDNSZoneResolver {
    // DNSZonehashChanged is emitted whenever a given node's zone hash is updated.
    event DNSZonehashChanged(
        bytes32 indexed node,
        bytes lastzonehash,
        bytes zonehash
    );

    /// zonehash obtains the hash for the zone.
    /// @param node The ENS node to query.
    /// @return The associated contenthash.
    function zonehash(bytes32 node) external view returns (bytes memory);
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.8.4;

/// Interface for the legacy (ETH-only) addr function.
interface IAddrResolver {
    event AddrChanged(bytes32 indexed node, address a);

    /// Returns the address associated with an ENS node.
    /// @param node The ENS node to query.
    /// @return The associated address.
    function addr(bytes32 node) external view returns (address payable);
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.8.4;

interface INameResolver {
    event NameChanged(bytes32 indexed node, string name);

    /// Returns the name associated with an ENS node, for reverse records.
    /// Defined in EIP181.
    /// @param node The ENS node to query.
    /// @return The associated name.
    function name(bytes32 node) external view returns (string memory);
}

//SPDX-License-Identifier: MIT
pragma solidity ~0.8.17;

import "./IPriceOracle.sol";

interface IETHRegistrarController {
    struct Registration {
        string label;
        address owner;
        uint256 duration;
        bytes32 secret;
        address resolver;
        bytes[] data;
        uint8 reverseRecord;
        bytes32 referrer;
    }

    function rentPrice(
        string memory label,
        uint256 duration
    ) external view returns (IPriceOracle.Price memory);

    function available(string memory label) external returns (bool);

    function makeCommitment(
        Registration memory registration
    ) external pure returns (bytes32 commitment);

    function commit(bytes32 commitment) external;

    function register(Registration memory registration) external payable;

    function renew(
        string calldata label,
        uint256 duration,
        bytes32 referrer
    ) external payable;
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.8.4;

interface IContentHashResolver {
    event ContenthashChanged(bytes32 indexed node, bytes hash);

    /// Returns the contenthash associated with an ENS node.
    /// @param node The ENS node to query.
    /// @return The associated contenthash.
    function contenthash(bytes32 node) external view returns (bytes memory);
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
pragma solidity >=0.8.4;

/// Interface for the new (multicoin) addr function.
interface IAddressResolver {
    event AddressChanged(
        bytes32 indexed node,
        uint256 coinType,
        bytes newAddress
    );

    function addr(
        bytes32 node,
        uint256 coinType
    ) external view returns (bytes memory);
}

pragma solidity >=0.8.4;

interface IReverseRegistrar {
    function setDefaultResolver(address resolver) external;

    function claim(address owner) external returns (bytes32);

    function claimForAddr(
        address addr,
        address owner,
        address resolver
    ) external returns (bytes32);

    function claimWithResolver(
        address owner,
        address resolver
    ) external returns (bytes32);

    function setName(string memory name) external returns (bytes32);

    function setNameForAddr(
        address addr,
        address owner,
        address resolver,
        string memory name
    ) external returns (bytes32);

    function node(address addr) external pure returns (bytes32);
}

//SPDX-License-Identifier: MIT
pragma solidity >=0.8.17 <0.9.0;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

/// @notice Contract is used to recover ERC20 tokens sent to the contract by mistake.

contract ERC20Recoverable is Ownable {
    /// @notice Recover ERC20 tokens sent to the contract by mistake.
    /// @dev The contract is Ownable and only the owner can call the recover function.
    /// @param _to The address to send the tokens to.
    /// @param _token The address of the ERC20 token to recover
    /// @param _amount The amount of tokens to recover.
    function recoverFunds(
        address _token,
        address _to,
        uint256 _amount
    ) external onlyOwner {
        IERC20(_token).transfer(_to, _amount);
    }
}

//SPDX-License-Identifier: MIT
pragma solidity >=0.8.4;

interface ENS {
    // Logged when the owner of a node assigns a new owner to a subnode.
    event NewOwner(bytes32 indexed node, bytes32 indexed label, address owner);

    // Logged when the owner of a node transfers ownership to a new account.
    event Transfer(bytes32 indexed node, address owner);

    // Logged when the resolver for a node changes.
    event NewResolver(bytes32 indexed node, address resolver);

    // Logged when the TTL of a node changes
    event NewTTL(bytes32 indexed node, uint64 ttl);

    // Logged when an operator is added or removed.
    event ApprovalForAll(
        address indexed owner,
        address indexed operator,
        bool approved
    );

    function setRecord(
        bytes32 node,
        address owner,
        address resolver,
        uint64 ttl
    ) external;

    function setSubnodeRecord(
        bytes32 node,
        bytes32 label,
        address owner,
        address resolver,
        uint64 ttl
    ) external;

    function setSubnodeOwner(
        bytes32 node,
        bytes32 label,
        address owner
    ) external returns (bytes32);

    function setResolver(bytes32 node, address resolver) external;

    function setOwner(bytes32 node, address owner) external;

    function setTTL(bytes32 node, uint64 ttl) external;

    function setApprovalForAll(address operator, bool approved) external;

    function owner(bytes32 node) external view returns (address);

    function resolver(bytes32 node) external view returns (address);

    function ttl(bytes32 node) external view returns (uint64);

    function recordExists(bytes32 node) external view returns (bool);

    function isApprovedForAll(
        address owner,
        address operator
    ) external view returns (bool);
}

//SPDX-License-Identifier: MIT
pragma solidity >=0.8.4;

interface ENS {
    // Logged when the owner of a node assigns a new owner to a subnode.
    event NewOwner(bytes32 indexed node, bytes32 indexed label, address owner);

    // Logged when the owner of a node transfers ownership to a new account.
    event Transfer(bytes32 indexed node, address owner);

    // Logged when the resolver for a node changes.
    event NewResolver(bytes32 indexed node, address resolver);

    // Logged when the TTL of a node changes
    event NewTTL(bytes32 indexed node, uint64 ttl);

    // Logged when an operator is added or removed.
    event ApprovalForAll(
        address indexed owner,
        address indexed operator,
        bool approved
    );

    function setRecord(
        bytes32 node,
        address owner,
        address resolver,
        uint64 ttl
    ) external;

    function setSubnodeRecord(
        bytes32 node,
        bytes32 label,
        address owner,
        address resolver,
        uint64 ttl
    ) external;

    function setSubnodeOwner(
        bytes32 node,
        bytes32 label,
        address owner
    ) external returns (bytes32);

    function setResolver(bytes32 node, address resolver) external;

    function setOwner(bytes32 node, address owner) external;

    function setTTL(bytes32 node, uint64 ttl) external;

    function setApprovalForAll(address operator, bool approved) external;

    function owner(bytes32 node) external view returns (address);

    function resolver(bytes32 node) external view returns (address);

    function ttl(bytes32 node) external view returns (uint64);

    function recordExists(bytes32 node) external view returns (bool);

    function isApprovedForAll(
        address owner,
        address operator
    ) external view returns (bool);
}

//SPDX-License-Identifier: MIT
pragma solidity >=0.8.17 <0.9.0;

interface IPriceOracle {
    struct Price {
        uint256 base;
        uint256 premium;
    }

    /// @dev Returns the price to register or renew a name.
    /// @param name The name being registered or renewed.
    /// @param expires When the name presently expires (0 if this is a new registration).
    /// @param duration How long the name is being registered or extended for, in seconds.
    /// @return base premium tuple of base price + premium price
    function price(
        string calldata name,
        uint256 expires,
        uint256 duration
    ) external view returns (Price calldata);
}

//SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

import "../registry/ENS.sol";
import "@openzeppelin/contracts/token/ERC721/IERC721.sol";

interface IBaseRegistrar is IERC721 {
    event ControllerAdded(address indexed controller);
    event ControllerRemoved(address indexed controller);
    event NameMigrated(
        uint256 indexed id,
        address indexed owner,
        uint256 expires
    );
    event NameRegistered(
        uint256 indexed id,
        address indexed owner,
        uint256 expires
    );
    event NameRenewed(uint256 indexed id, uint256 expires);

    // Authorises a controller, who can register and renew domains.
    function addController(address controller) external;

    // Revoke controller permission for an address.
    function removeController(address controller) external;

    // Set the resolver for the TLD this registrar manages.
    function setResolver(address resolver) external;

    // Returns the expiration timestamp of the specified label hash.
    function nameExpires(uint256 id) external view returns (uint256);

    // Returns true if the specified name is available for registration.
    function available(uint256 id) external view returns (bool);

    /// @dev Register a name.
    function register(
        uint256 id,
        address owner,
        uint256 duration
    ) external returns (uint256);

    function renew(uint256 id, uint256 duration) external returns (uint256);

    /// @dev Reclaim ownership of a name in ENS, if you own it in the registrar.
    function reclaim(uint256 id, address owner) external;
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

pragma solidity >=0.8.4;

import "../registry/ENS.sol";
import "./IBaseRegistrar.sol";
import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract BaseRegistrarImplementation is ERC721, IBaseRegistrar, Ownable {
    // A map of expiry times
    mapping(uint256 => uint256) expiries;
    // The ENS registry
    ENS public ens;
    // The namehash of the TLD this registrar owns (eg, .eth)
    bytes32 public baseNode;
    // A map of addresses that are authorised to register and renew names.
    mapping(address => bool) public controllers;
    uint256 public constant GRACE_PERIOD = 90 days;
    bytes4 private constant INTERFACE_META_ID =
        bytes4(keccak256("supportsInterface(bytes4)"));
    bytes4 private constant ERC721_ID =
        bytes4(
            keccak256("balanceOf(address)") ^
                keccak256("ownerOf(uint256)") ^
                keccak256("approve(address,uint256)") ^
                keccak256("getApproved(uint256)") ^
                keccak256("setApprovalForAll(address,bool)") ^
                keccak256("isApprovedForAll(address,address)") ^
                keccak256("transferFrom(address,address,uint256)") ^
                keccak256("safeTransferFrom(address,address,uint256)") ^
                keccak256("safeTransferFrom(address,address,uint256,bytes)")
        );
    bytes4 private constant RECLAIM_ID =
        bytes4(keccak256("reclaim(uint256,address)"));

    /// v2.1.3 version of _isApprovedOrOwner which calls ownerOf(tokenId) and takes grace period into consideration instead of ERC721.ownerOf(tokenId);
    /// https://github.com/OpenZeppelin/openzeppelin-contracts/blob/v2.1.3/contracts/token/ERC721/ERC721.sol#L187
    /// @dev Returns whether the given spender can transfer a given token ID
    /// @param spender address of the spender to query
    /// @param tokenId uint256 ID of the token to be transferred
    /// @return bool whether the msg.sender is approved for the given token ID,
    ///              is an operator of the owner, or is the owner of the token
    function _isApprovedOrOwner(
        address spender,
        uint256 tokenId
    ) internal view override returns (bool) {
        address owner = ownerOf(tokenId);
        return (spender == owner ||
            getApproved(tokenId) == spender ||
            isApprovedForAll(owner, spender));
    }

    constructor(ENS _ens, bytes32 _baseNode) ERC721("", "") {
        ens = _ens;
        baseNode = _baseNode;
    }

    modifier live() {
        require(ens.owner(baseNode) == address(this));
        _;
    }

    modifier onlyController() {
        require(controllers[msg.sender]);
        _;
    }

    /// @dev Gets the owner of the specified token ID. Names become unowned
    ///      when their registration expires.
    /// @param tokenId uint256 ID of the token to query the owner of
    /// @return address currently marked as the owner of the given token ID
    function ownerOf(
        uint256 tokenId
    ) public view override(IERC721, ERC721) returns (address) {
        require(expiries[tokenId] > block.timestamp);
        return super.ownerOf(tokenId);
    }

    // Authorises a controller, who can register and renew domains.
    function addController(address controller) external override onlyOwner {
        controllers[controller] = true;
        emit ControllerAdded(controller);
    }

    // Revoke controller permission for an address.
    function removeController(address controller) external override onlyOwner {
        controllers[controller] = false;
        emit ControllerRemoved(controller);
    }

    // Set the resolver for the TLD this registrar manages.
    function setResolver(address resolver) external override onlyOwner {
        ens.setResolver(baseNode, resolver);
    }

    // Returns the expiration timestamp of the specified id.
    function nameExpires(uint256 id) external view override returns (uint256) {
        return expiries[id];
    }

    // Returns true iff the specified name is available for registration.
    function available(uint256 id) public view override returns (bool) {
        // Not available if it's registered here or in its grace period.
        return expiries[id] + GRACE_PERIOD < block.timestamp;
    }

    /// @dev Register a name.
    /// @param id The token ID (keccak256 of the label).
    /// @param owner The address that should own the registration.
    /// @param duration Duration in seconds for the registration.
    function register(
        uint256 id,
        address owner,
        uint256 duration
    ) external override returns (uint256) {
        return _register(id, owner, duration, true);
    }

    /// @dev Register a name, without modifying the registry.
    /// @param id The token ID (keccak256 of the label).
    /// @param owner The address that should own the registration.
    /// @param duration Duration in seconds for the registration.
    function registerOnly(
        uint256 id,
        address owner,
        uint256 duration
    ) external returns (uint256) {
        return _register(id, owner, duration, false);
    }

    function _register(
        uint256 id,
        address owner,
        uint256 duration,
        bool updateRegistry
    ) internal live onlyController returns (uint256) {
        require(available(id));
        require(
            block.timestamp + duration + GRACE_PERIOD >
                block.timestamp + GRACE_PERIOD
        ); // Prevent future overflow

        expiries[id] = block.timestamp + duration;
        if (_exists(id)) {
            // Name was previously owned, and expired
            _burn(id);
        }
        _mint(owner, id);
        if (updateRegistry) {
            ens.setSubnodeOwner(baseNode, bytes32(id), owner);
        }

        emit NameRegistered(id, owner, block.timestamp + duration);

        return block.timestamp + duration;
    }

    function renew(
        uint256 id,
        uint256 duration
    ) external override live onlyController returns (uint256) {
        require(expiries[id] + GRACE_PERIOD >= block.timestamp); // Name must be registered here or in grace period
        require(
            expiries[id] + duration + GRACE_PERIOD > duration + GRACE_PERIOD
        ); // Prevent future overflow

        expiries[id] += duration;
        emit NameRenewed(id, expiries[id]);
        return expiries[id];
    }

    /// @dev Reclaim ownership of a name in ENS, if you own it in the registrar.
    function reclaim(uint256 id, address owner) external override live {
        require(_isApprovedOrOwner(msg.sender, id));
        ens.setSubnodeOwner(baseNode, bytes32(id), owner);
    }

    function supportsInterface(
        bytes4 interfaceID
    ) public view override(ERC721, IERC165) returns (bool) {
        return
            interfaceID == INTERFACE_META_ID ||
            interfaceID == ERC721_ID ||
            interfaceID == RECLAIM_ID;
    }
}


## ------------ SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS ------------ 
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {AbstractVerifier, IVerifierHooks} from '../AbstractVerifier.sol';
import {GatewayRequest, GatewayVM, ProofSequence} from '../GatewayVM.sol';

contract UncheckedVerifier is AbstractVerifier {
    constructor(
        string[] memory urls,
        uint256 window,
        IVerifierHooks hooks
    ) AbstractVerifier(urls, window, hooks) {}

    function getLatestContext() external view returns (bytes memory) {
        return abi.encode(block.timestamp);
    }

    function getStorageValues(
        bytes memory context,
        GatewayRequest memory req,
        bytes memory proof
    ) external view returns (bytes[] memory values, uint8 exitCode) {
        uint256 t1 = abi.decode(context, (uint256));
        (uint256 t, bytes[] memory proofs, bytes memory order) = abi.decode(
            proof,
            (uint256, bytes[], bytes)
        );
        _checkWindow(t1, t);
        return
            GatewayVM.evalRequest(
                req,
                ProofSequence(0, bytes32(0), proofs, order, _hooks)
            );
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

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {ERC165} from "@openzeppelin/contracts-v5/utils/introspection/ERC165.sol";
import {Ownable} from "@openzeppelin/contracts-v5/access/Ownable.sol";

import {GatewayFetchTarget, IGatewayVerifier} from "@unruggable/gateways/GatewayFetchTarget.sol";
import {GatewayFetcher, GatewayRequest, RequestOverflow} from "@unruggable/gateways/GatewayFetcher.sol";

import {AbstractReverseResolver} from "./AbstractReverseResolver.sol";
import {IStandaloneReverseRegistrar} from "../reverseRegistrar/IStandaloneReverseRegistrar.sol";
import {IVerifiableResolver} from "../resolvers/profiles/IVerifiableResolver.sol";
import {INameReverser} from "./INameReverser.sol";
import {ENSIP19} from "../utils/ENSIP19.sol";

/// @title Chain Reverse Resolver
/// @notice Reverses an EVM address using the first non-null response from the following sources:
///
/// 1. `L2ReverseRegistrar` on L2 chain via Unruggable Gateway
/// 2. `IStandaloneReverseRegistrar` for "default.reverse"
///
contract ChainReverseResolver is
    AbstractReverseResolver,
	IVerifiableResolver,
    GatewayFetchTarget,
    Ownable
{
    using GatewayFetcher for GatewayRequest;

    /// @notice Storage slot for the names mapping in `L2ReverseRegistrar`.
    uint256 constant NAMES_SLOT = 0;

    /// @notice The reverse registrar contract for "default.reverse".
    IStandaloneReverseRegistrar public immutable defaultRegistrar;

    /// @notice The verifier contract for the L2 chain.
    IGatewayVerifier public gatewayVerifier;

    /// @notice Gateway URLs for the verifier contract.
    string[] public gatewayURLs;

    /// @notice Emitted when the gateway verifier is changed.
    event GatewayVerifierChanged(address verifier);

    /// @notice Emitted when the gateway URLs are changed.
    event GatewayURLsChanged(string[] urls);

    constructor(
        address _owner,
        uint256 coinType,
        IStandaloneReverseRegistrar _defaultRegistrar,
        address _chainRegistrar,
        IGatewayVerifier verifier,
        string[] memory gateways
    ) Ownable(_owner) AbstractReverseResolver(coinType, _chainRegistrar) {
        defaultRegistrar = _defaultRegistrar;
        gatewayVerifier = verifier;
        gatewayURLs = gateways;
    }

    /// @inheritdoc ERC165
    function supportsInterface(
        bytes4 interfaceId
    ) public view override returns (bool) {
        return
            interfaceId == type(IVerifiableResolver).interfaceId ||
            super.supportsInterface(interfaceId);
    }

	/// @inheritdoc IVerifiableResolver
    function verifierMetadata(
        bytes memory name
    ) external view returns (address verifier, string[] memory gateways) {
		 (bytes memory a, uint256 ct) = ENSIP19.parse(name);
		 if (a.length == 20 && ct == coinType) {
			return (address(gatewayVerifier), gatewayURLs);
		 }
	}

    /// @notice Set gateway URLs.
    /// @param gateways The new gateway URLs.
    function setGatewayURLs(string[] memory gateways) external onlyOwner {
        gatewayURLs = gateways;
        emit GatewayURLsChanged(gateways);
    }

    /// @notice Set the verifier contract.
    /// @param verifier The new verifier contract.
    function setGatewayVerifier(address verifier) external onlyOwner {
        gatewayVerifier = IGatewayVerifier(verifier);
        emit GatewayVerifierChanged(verifier);
    }

    /// @inheritdoc AbstractReverseResolver
    function _resolveName(
        address addr
    ) internal view override returns (string memory) {
        GatewayRequest memory req = GatewayFetcher.newRequest(1);
        req.setTarget(chainRegistrar);
        req.setSlot(NAMES_SLOT).push(addr).follow().readBytes(); // names[addr]
        req.setOutput(0);
        fetch(
            gatewayVerifier,
            req,
            this.resolveNameCallback.selector, // ==> step 2
            abi.encode(addr),
            gatewayURLs
        );
    }

    /// @dev CCIP-Read callback for `_resolveName()`.
    /// @param values The outputs for `GatewayRequest` (1 name).
    /// @param extraData The contextual data passed from `_resolveName()`.
    /// @return result The abi-encoded name for the given address.
    function resolveNameCallback(
        bytes[] memory values,
        uint8 /* exitCode */,
        bytes calldata extraData
    ) external view returns (bytes memory result) {
        string memory name = string(values[0]);
        if (bytes(name).length == 0) {
            address addr = abi.decode(extraData, (address));
            name = defaultRegistrar.nameForAddr(addr);
        }
        result = abi.encode(name);
    }

    /// @inheritdoc INameReverser
    /// @dev Reverts with a variety of errors.
    /// - reverts `RequestOverflow` if too many addresses.
    /// - Gateway request may fail if too many proofs.
    /// - Gateway response may run out of gas.
    function resolveNames(
        address[] memory addrs
    ) external view returns (string[] memory) {
        if (addrs.length > 255) {
            revert RequestOverflow();
        }
        GatewayRequest memory req = GatewayFetcher.newRequest(
            uint8(addrs.length)
        );
        req.setTarget(chainRegistrar); // target L2 registrar
        for (uint256 i; i < addrs.length; ++i) {
            req.setSlot(NAMES_SLOT).push(addrs[i]).follow().readBytes(); // names[addr[i]]
            req.setOutput(uint8(i));
        }
        fetch(
            gatewayVerifier,
            req,
            this.resolveNamesCallback.selector, // ==> step 2
            abi.encode(addrs),
            gatewayURLs
        );
    }

    /// @dev CCIP-Read callback for `_resolveNames()`.
    ///      Recursive if there are still addresses to resolve.
    /// @param values The outputs for `GatewayRequest` (N names).
    /// @param extraData The contextual data passed from `_resolveNames()`.
    /// @return names The resolved names.
    function resolveNamesCallback(
        bytes[] memory values,
        uint8 /* exitCode */,
        bytes calldata extraData
    ) external view returns (string[] memory names) {
        address[] memory addrs = abi.decode(extraData, (address[]));
        names = new string[](addrs.length);
        for (uint256 i; i < addrs.length; ++i) {
            string memory name = string(values[i]);
            if (bytes(name).length == 0) {
                name = defaultRegistrar.nameForAddr(addrs[i]);
            }
            names[i] = name;
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {AbstractVerifier, IVerifierHooks} from '../AbstractVerifier.sol';
import {GatewayRequest, GatewayVM, ProofSequence} from '../GatewayVM.sol';
import {RLPReader, RLPReaderExt} from '../RLPReaderExt.sol';

// https://github.com/ethereum-optimism/optimism/blob/develop/packages/contracts-bedrock/src/L2/L1Block.sol
interface IL1Block {
    function number() external view returns (uint256);
}

contract ReverseOPVerifier is AbstractVerifier {
    uint256 immutable SLOT_HASH = 2;
    IL1Block immutable _l1Block;

    constructor(
        string[] memory urls,
        uint256 window,
        IVerifierHooks hooks,
        IL1Block l1Block
    ) AbstractVerifier(urls, window, hooks) {
        _l1Block = l1Block;
    }

    function getLatestContext() external view returns (bytes memory) {
        return abi.encode(_l1Block.number());
    }

    struct GatewayProof {
        bytes rlpEncodedL1Block;
        bytes rlpEncodedL2Block;
        bytes accountProof;
        bytes storageProof;
        bytes[] proofs;
        bytes order;
    }

    function getStorageValues(
        bytes memory context,
        GatewayRequest memory req,
        bytes memory proof
    ) external view returns (bytes[] memory, uint8 exitCode) {
        uint256 blockNumber1 = abi.decode(context, (uint256));
        GatewayProof memory p = abi.decode(proof, (GatewayProof));
        RLPReader.RLPItem[] memory v = RLPReader.readList(p.rlpEncodedL2Block);
        bytes32 blockHash = blockhash(_extractBlockNumber(v));
        require(
            blockHash == keccak256(p.rlpEncodedL2Block),
            'ReverseOP: hash2'
        );
        bytes32 stateRoot = RLPReaderExt.strictBytes32FromRLP(v[3]);
        bytes32 storageRoot = _hooks.verifyAccountState(
            stateRoot,
            address(_l1Block),
            p.accountProof
        );
        blockHash = _hooks.verifyStorageValue(
            storageRoot,
            address(_l1Block),
            SLOT_HASH,
            p.storageProof
        );
        require(
            blockHash == keccak256(p.rlpEncodedL1Block),
            'ReverseOP: hash1'
        );
        v = RLPReader.readList(p.rlpEncodedL1Block);
        _checkWindow(blockNumber1, _extractBlockNumber(v));
        stateRoot = RLPReaderExt.strictBytes32FromRLP(v[3]);
        return
            GatewayVM.evalRequest(
                req,
                ProofSequence(0, stateRoot, p.proofs, p.order, _hooks)
            );
    }

    function _extractBlockNumber(
        RLPReader.RLPItem[] memory v
    ) internal pure returns (uint256) {
        return uint256(RLPReaderExt.bytes32FromRLP(v[8]));
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {AbstractVerifier, IVerifierHooks} from '../AbstractVerifier.sol';
import {GatewayRequest, GatewayVM, ProofSequence} from '../GatewayVM.sol';

// https://github.com/taikoxyz/taiko-mono/blob/6e5b8c800c0128ecf080567dc6daadd75c79319d/packages/protocol/contracts/layer1/based/ITaikoInbox.sol

struct TransitionState {
    bytes32 parentHash;
    bytes32 blockHash;
    bytes32 stateRoot;
    address prover;
    bool inProvingWindow;
    uint48 createdAt;
}

interface ITaiko {
    function getTransitionById(
        uint64 blockId,
        uint24 tid
    ) external view returns (TransitionState memory);
    function getLastSyncedTransition()
        external
        view
        returns (uint64 batchId, uint64 blockId, TransitionState memory ts);
}

contract TaikoVerifier is AbstractVerifier {
    ITaiko immutable _rollup;

    constructor(
        string[] memory urls,
        uint256 window,
        IVerifierHooks hooks,
        ITaiko rollup
    ) AbstractVerifier(urls, window, hooks) {
        _rollup = rollup;
    }

    function getLatestContext() external view returns (bytes memory) {
        (uint64 batchId, , TransitionState memory ts) = _rollup
            .getLastSyncedTransition();
        return abi.encode(batchId, ts.createdAt);
    }

    struct GatewayProof {
        uint64 batchId;
        uint24 tid;
        bytes[] proofs;
        bytes order;
    }

    function getStorageValues(
        bytes memory context,
        GatewayRequest memory req,
        bytes memory proof
    ) external view returns (bytes[] memory, uint8 exitCode) {
        (, uint256 createdAt) = abi.decode(context, (uint64, uint48));
        GatewayProof memory p = abi.decode(proof, (GatewayProof));
        TransitionState memory ts = _rollup.getTransitionById(p.batchId, p.tid); // reverts if invalid
        _checkWindow(createdAt, ts.createdAt);
        return
            GatewayVM.evalRequest(
                req,
                ProofSequence(0, ts.stateRoot, p.proofs, p.order, _hooks)
            );
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Ownable} from '@openzeppelin/contracts/access/Ownable.sol';
import {ERC165} from '@openzeppelin/contracts/utils/introspection/ERC165.sol';

import {IStandardGatewayVerifier, IGatewayVerifier} from './IStandardGatewayVerifier.sol';
import {IVerifierHooks} from './IVerifierHooks.sol';

abstract contract AbstractVerifier is IStandardGatewayVerifier, Ownable, ERC165 {
    event GatewayURLsChanged();

    string[] _urls;
    uint256 immutable _window;
    IVerifierHooks immutable _hooks;

    constructor(
        string[] memory urls,
        uint256 window,
        IVerifierHooks hooks
    ) Ownable(msg.sender) {
        _urls = urls;
        _window = window;
        _hooks = hooks;
    }

    /// @inheritdoc ERC165
    function supportsInterface(
        bytes4 interfaceId
    ) public view virtual override returns (bool) {
        return
            interfaceId == type(IGatewayVerifier).interfaceId ||
            interfaceId == type(IStandardGatewayVerifier).interfaceId ||
            super.supportsInterface(interfaceId);
    }

    function setGatewayURLs(string[] memory urls) external onlyOwner {
        _urls = urls;
        emit GatewayURLsChanged();
    }

    /// @inheritdoc IGatewayVerifier
    function gatewayURLs() external view returns (string[] memory) {
        return _urls;
    }

    /// @inheritdoc IStandardGatewayVerifier
    function getWindow() external view returns (uint256) {
        return _window;
    }

    /// @inheritdoc IStandardGatewayVerifier
    function getHooks() external view returns (IVerifierHooks) {
        return _hooks;
    }

    function _checkWindow(uint256 latest, uint256 got) internal view {
        if (got + _window < latest) revert CommitTooOld(latest, got, _window);
        if (got > latest) revert CommitTooNew(latest, got);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {AbstractVerifier, IVerifierHooks} from './AbstractVerifier.sol';
import {GatewayRequest, GatewayVM, ProofSequence} from './GatewayVM.sol';
import {RLPReader, RLPReaderExt} from './RLPReaderExt.sol';

contract SelfVerifier is AbstractVerifier {
    constructor(
        string[] memory urls,
        uint256 window,
        IVerifierHooks hooks
    ) AbstractVerifier(urls, window, hooks) {}

    function getLatestContext() external view returns (bytes memory) {
        return abi.encode(block.number - 1);
    }

    struct GatewayProof {
        bytes rlpEncodedBlock;
        bytes[] proofs;
        bytes order;
    }

    function getStorageValues(
        bytes memory context,
        GatewayRequest memory req,
        bytes memory proof
    ) external view returns (bytes[] memory, uint8 exitCode) {
        uint256 blockNumber1 = abi.decode(context, (uint256));
        GatewayProof memory p = abi.decode(proof, (GatewayProof));
        RLPReader.RLPItem[] memory v = RLPReader.readList(p.rlpEncodedBlock);
        uint256 blockNumber = uint256(RLPReaderExt.bytes32FromRLP(v[8]));
        _checkWindow(blockNumber1, blockNumber);
        // TODO: change this to https://eips.ethereum.org/EIPS/eip-2935
        bytes32 blockHash = blockhash(blockNumber);
        require(blockHash == keccak256(p.rlpEncodedBlock), 'Self: blockhash');
        bytes32 stateRoot = RLPReaderExt.strictBytes32FromRLP(v[3]);
        return verify(req, stateRoot, p.proofs, p.order);
    }

    function verify(
        GatewayRequest memory req,
        bytes32 stateRoot,
        bytes[] memory proofs,
        bytes memory order
    ) public view returns (bytes[] memory outputs, uint8 exitCode) {
        return
            GatewayVM.evalRequest(
                req,
                ProofSequence(0, stateRoot, proofs, order, _hooks)
            );
    }
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.8.4;

import {IExtendedResolver} from "./IExtendedResolver.sol";

/// @notice A resolver that calls other resolvers.
/// @dev Interface selector: `0xc7e45d73`
interface ICompositeResolver is IExtendedResolver {
    /// @notice Fetch the underlying resolver for `name`.
    ///         Callers should enable EIP-3668.
    ///
    /// * If `offchain`, additional information is necessary to locate `resolver`.
    /// * If `resolver` is null, `offchain` is irrelevant.
    ///
    /// @param name The DNS-encoded name.
    ///
    /// @return resolver The underlying resolver address.
    /// @return offchain `true` if `resolver` is offchain.
    function getResolver(
        bytes memory name
    ) external view returns (address resolver, bool offchain);

    /// @notice Determine if resolving `name` requires offchain data.
    ///
    /// @param name The DNS-encoded name.
    ///
    /// @return `true` if requires offchain data.
    function requiresOffchain(bytes calldata name) external view returns (bool);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {AbstractVerifier, IVerifierHooks} from '../AbstractVerifier.sol';
import {GatewayRequest, GatewayVM, ProofSequence} from '../GatewayVM.sol';
import {RLPReader, RLPReaderExt} from '../RLPReaderExt.sol';

interface IRollup {
    function stateRoot() external view returns (bytes32);
    function stateBlockNumber() external view returns (uint256);
}

// https://github.com/starkware-libs/cairo-lang/blob/master/src/starkware/starknet/solidity/Starknet.sol#L60
uint256 constant SLOT_STATE_ROOT = uint256(
    keccak256('STARKNET_1.0_INIT_STARKNET_STATE_STRUCT')
);

contract StarknetVerifier is AbstractVerifier {
    IRollup immutable _rollup;
    IVerifierHooks immutable _ethHooks;

    constructor(
        string[] memory urls,
        uint256 window,
        IVerifierHooks hooks,
        IRollup rollup,
        IVerifierHooks ethHooks
    ) AbstractVerifier(urls, window, hooks) {
        _rollup = rollup;
        _ethHooks = ethHooks;
    }

    function getLatestContext() external view returns (bytes memory) {
        return abi.encode(_rollup.stateBlockNumber());
    }

    struct GatewayProof {
        uint256 blockNumber;
        bytes rlpEncodedL1Block;
        bytes accountProof;
        bytes storageProof;
        bytes[] proofs;
        bytes order;
    }

    function getStorageValues(
        bytes memory context,
        GatewayRequest memory req,
        bytes memory proof
    ) external view returns (bytes[] memory, uint8 exitCode) {
        uint256 blockNumber1 = abi.decode(context, (uint256));
        GatewayProof memory p = abi.decode(proof, (GatewayProof));
        bytes32 stateRoot;
        if (blockNumber1 == p.blockNumber) {
            stateRoot = _rollup.stateRoot();
        } else {
            //_checkWindow(blockNumber1, p.blockNumber);
            RLPReader.RLPItem[] memory v = RLPReader.readList(
                p.rlpEncodedL1Block
            );
            bytes32 blockHash = blockhash(
                uint256(RLPReaderExt.bytes32FromRLP(v[8]))
            );
            require(
                blockHash == keccak256(p.rlpEncodedL1Block),
                'Starknet: blockhash'
            );
            stateRoot = RLPReaderExt.strictBytes32FromRLP(v[3]);
            bytes32 storageRoot = _ethHooks.verifyAccountState(
                stateRoot,
                address(_rollup),
                p.accountProof
            );
            stateRoot = _ethHooks.verifyStorageValue(
                storageRoot,
                address(_rollup),
                SLOT_STATE_ROOT,
                p.storageProof
            );
        }
        return
            GatewayVM.evalRequest(
                req,
                ProofSequence(0, stateRoot, p.proofs, p.order, _hooks)
            );
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {LazyTrustedVerifier, IVerifierHooks} from './LazyTrustedVerifier.sol';

contract TrustedVerifier is LazyTrustedVerifier {
    constructor(
        IVerifierHooks hooks,
        string[] memory urls,
        address[] memory signers,
        uint256 expSec
    ) {
        initialize(msg.sender, hooks, urls, signers, expSec);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {AbstractReverseResolver} from "./AbstractReverseResolver.sol";
import {ENS} from "../registry/ENS.sol";
import {INameResolver} from "../resolvers/profiles/INameResolver.sol";
import {IStandaloneReverseRegistrar} from "../reverseRegistrar/IStandaloneReverseRegistrar.sol";
import {INameReverser} from "./INameReverser.sol";
import {COIN_TYPE_ETH} from "../utils/ENSIP19.sol";
import {NameCoder} from "../utils/NameCoder.sol";
import {HexUtils} from "../utils/HexUtils.sol";
import {LibABI} from "../utils/LibABI.sol";

/// @title Ethereum Reverse Resolver
/// @notice Reverses an EVM address using the first non-null response from the following sources:
///
/// 1. `IStandaloneReverseRegistrar` for "addr.reverse"
/// 2. `name()` from "{addr}.addr.reverse" in V1 Registry
/// 3. `IStandaloneReverseRegistrar` for "default.reverse"
///
contract ETHReverseResolver is AbstractReverseResolver {
    /// @dev Namehash of "addr.reverse"
    bytes32 constant ADDR_REVERSE_NODE =
        0x91d1777781884d03a6757a803996e38de2a42967fb37eeaca72729271025a9e2;

    /// @notice The ENS registry contract.
    ENS immutable ens;

    /// @notice The reverse registrar contract for "default.reverse".
    IStandaloneReverseRegistrar public immutable defaultRegistrar;

    constructor(
        ENS _ens,
        IStandaloneReverseRegistrar addrRegistrar,
        IStandaloneReverseRegistrar _defaultRegistrar
    ) AbstractReverseResolver(COIN_TYPE_ETH, address(addrRegistrar)) {
        ens = _ens;
        defaultRegistrar = _defaultRegistrar;
    }

    /// @inheritdoc AbstractReverseResolver
    function _resolveName(
        address addr
    ) internal view override returns (string memory name) {
        name = IStandaloneReverseRegistrar(chainRegistrar).nameForAddr(addr);
        if (bytes(name).length > 0) {
            return name;
        }
        bytes32 node = NameCoder.namehash(
            ADDR_REVERSE_NODE,
            keccak256(bytes(HexUtils.addressToHex(addr)))
        );
        address resolver = ens.resolver(node);
        if (resolver != address(0)) {
            // note: this only supports onchain direct calls (no extended, no offchain)
            (bool ok, bytes memory v) = resolver.staticcall{gas: 100_000}(
                abi.encodeCall(INameResolver.name, (node))
            );
            if (ok) {
                (ok, v) = LibABI.tryDecodeBytes(v);
            }
            if (!ok) {
                return ""; // terminate on revert or decode failure
            }
            if (v.length > 0) {
                return string(v);
            }
        }
        return defaultRegistrar.nameForAddr(addr);
    }

    /// @inheritdoc INameReverser
    function resolveNames(
        address[] memory addrs
    ) external view returns (string[] memory names) {
        names = new string[](addrs.length);
        for (uint256 i; i < addrs.length; ++i) {
            names[i] = _resolveName(addrs[i]);
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.8.4;

import "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import "../ResolverBase.sol";
import "./AddrResolver.sol";
import "./IInterfaceResolver.sol";

abstract contract InterfaceResolver is IInterfaceResolver, AddrResolver {
    mapping(uint64 => mapping(bytes32 => mapping(bytes4 => address))) versionable_interfaces;

    /// Sets an interface associated with a name.
    /// Setting the address to 0 restores the default behaviour of querying the contract at `addr()` for interface support.
    /// @param node The node to update.
    /// @param interfaceID The EIP 165 interface ID.
    /// @param implementer The address of a contract that implements this interface for this node.
    function setInterface(
        bytes32 node,
        bytes4 interfaceID,
        address implementer
    ) external virtual authorised(node) {
        versionable_interfaces[recordVersions[node]][node][
            interfaceID
        ] = implementer;
        emit InterfaceChanged(node, interfaceID, implementer);
    }

    /// Returns the address of a contract that implements the specified interface for this name.
    /// If an implementer has not been set for this interfaceID and name, the resolver will query
    /// the contract at `addr()`. If `addr()` is set, a contract exists at that address, and that
    /// contract implements EIP165 and returns `true` for the specified interfaceID, its address
    /// will be returned.
    /// @param node The ENS node to query.
    /// @param interfaceID The EIP 165 interface ID to check for.
    /// @return The address that implements this interface, or 0 if the interface is unsupported.
    function interfaceImplementer(
        bytes32 node,
        bytes4 interfaceID
    ) external view virtual override returns (address) {
        address implementer = versionable_interfaces[recordVersions[node]][
            node
        ][interfaceID];
        if (implementer != address(0)) {
            return implementer;
        }

        address a = addr(node);
        if (a == address(0)) {
            return address(0);
        }

        (bool success, bytes memory returnData) = a.staticcall(
            abi.encodeWithSignature(
                "supportsInterface(bytes4)",
                type(IERC165).interfaceId
            )
        );
        if (!success || returnData.length < 32 || returnData[31] == 0) {
            // EIP 165 not supported by target
            return address(0);
        }

        (success, returnData) = a.staticcall(
            abi.encodeWithSignature("supportsInterface(bytes4)", interfaceID)
        );
        if (!success || returnData.length < 32 || returnData[31] == 0) {
            // Specified interface not supported by target
            return address(0);
        }

        return a;
    }

    function supportsInterface(
        bytes4 interfaceID
    ) public view virtual override returns (bool) {
        return
            interfaceID == type(IInterfaceResolver).interfaceId ||
            super.supportsInterface(interfaceID);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {AbstractReverseResolver} from "./AbstractReverseResolver.sol";
import {IStandaloneReverseRegistrar} from "../reverseRegistrar/IStandaloneReverseRegistrar.sol";
import {INameReverser} from "./INameReverser.sol";
import {COIN_TYPE_DEFAULT} from "../utils/ENSIP19.sol";

/// @title Default Reverse Resolver
/// @notice Reverses an EVM address using the `IStandaloneReverseRegistrar` for "default.reverse".
contract DefaultReverseResolver is AbstractReverseResolver {
    constructor(
        IStandaloneReverseRegistrar defaultRegistrar
    ) AbstractReverseResolver(COIN_TYPE_DEFAULT, address(defaultRegistrar)) {}

    /// @inheritdoc AbstractReverseResolver
    function _resolveName(
        address addr
    ) internal view override returns (string memory name) {
        name = IStandaloneReverseRegistrar(chainRegistrar).nameForAddr(addr);
    }

    /// @inheritdoc INameReverser
    function resolveNames(
        address[] memory addrs
    ) external view returns (string[] memory names) {
        names = new string[](addrs.length);
        for (uint256 i; i < addrs.length; ++i) {
            names[i] = _resolveName(addrs[i]);
        }
    }
}

pragma solidity >=0.8.4;

import "./ENS.sol";
import "./ENSRegistry.sol";

/// The ENS registry contract.
contract ENSRegistryWithFallback is ENSRegistry {
    ENS public old;

    /// @dev Constructs a new ENS registrar.
    constructor(ENS _old) public ENSRegistry() {
        old = _old;
    }

    /// @dev Returns the address of the resolver for the specified node.
    /// @param node The specified node.
    /// @return address of the resolver.
    function resolver(bytes32 node) public view override returns (address) {
        if (!recordExists(node)) {
            return old.resolver(node);
        }

        return super.resolver(node);
    }

    /// @dev Returns the address that owns the specified node.
    /// @param node The specified node.
    /// @return address of the owner.
    function owner(bytes32 node) public view override returns (address) {
        if (!recordExists(node)) {
            return old.owner(node);
        }

        return super.owner(node);
    }

    /// @dev Returns the TTL of a node, and any records associated with it.
    /// @param node The specified node.
    /// @return ttl of the node.
    function ttl(bytes32 node) public view override returns (uint64) {
        if (!recordExists(node)) {
            return old.ttl(node);
        }

        return super.ttl(node);
    }

    function _setOwner(bytes32 node, address owner) internal override {
        address addr = owner;
        if (addr == address(0x0)) {
            addr = address(this);
        }

        super._setOwner(node, addr);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {AbstractVerifier, IVerifierHooks} from '../AbstractVerifier.sol';
import {GatewayRequest, GatewayVM, ProofSequence} from '../GatewayVM.sol';
import {ILineaRollup} from './ILineaRollup.sol';

//import 'forge-std/console.sol';

contract UnfinalizedLineaVerifier is AbstractVerifier {
    ILineaRollup immutable _rollup;

    constructor(
        string[] memory urls,
        uint256 window,
        IVerifierHooks hooks,
        ILineaRollup rollup
    ) AbstractVerifier(urls, window, hooks) {
        _rollup = rollup;
    }

    function getLatestContext() external view returns (bytes memory) {
        return abi.encode(block.number - 1);
    }

    struct GatewayProof {
        uint256 l1BlockNumber;
        bytes abiEncodedTuple;
        bytes[] proofs;
        bytes order;
    }

    function getStorageValues(
        bytes memory /*context*/,
        GatewayRequest memory req,
        bytes memory proof
    ) external view returns (bytes[] memory, uint8 exitCode) {
        //uint256 l1BlockNumber1 = abi.decode(context, (uint256));
        GatewayProof memory p = abi.decode(proof, (GatewayProof));
        // TODO: must prove a time constraint on the shnarf
        // ideas:
        // 1.) prove l1BlockNumber contains the transaction that that commit this shnarf
        // 2.) prove some L1 state on L2 using l2BlockNumber and stateRoot
        // 3.) use some heuristic based on L2.lastAnchoredL1MessageNumber and L1.nextMessageNumber?
        //_checkWindow(p.l1BlockNumber, l1BlockNumber1);
        bytes32 stateRoot = _extractStateRoot(p.abiEncodedTuple);
        uint256 l2BlockNumber = _rollup.shnarfFinalBlockNumbers(
            keccak256(p.abiEncodedTuple)
        );
        // this is the only guard available
        // the shnarf must be newer than the finalization
        if (l2BlockNumber < _rollup.currentL2BlockNumber()) {
            // TODO: remove this once we have a time constraint
            // if it's older than the finalization, it must match
            require(
                stateRoot == _rollup.stateRootHashes(l2BlockNumber),
                'UnfinalizedLinea: not finalized'
            );
        }
        return
            GatewayVM.evalRequest(
                req,
                ProofSequence(0, stateRoot, p.proofs, p.order, _hooks)
            );
    }

    function _extractStateRoot(
        bytes memory v
    ) internal pure returns (bytes32 stateRoot) {
        assembly {
            stateRoot := mload(add(v, 96)) // see: ShnarfData
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {AbstractVerifier, IVerifierHooks} from '../AbstractVerifier.sol';
import {GatewayRequest, GatewayVM, ProofSequence} from '../GatewayVM.sol';

interface IScrollChain {
    function lastFinalizedBatchIndex() external view returns (uint256);
    function finalizedStateRoots(
        uint256 batchIndex
    ) external view returns (bytes32);
}

contract ScrollVerifier is AbstractVerifier {
    IScrollChain immutable _rollup;

    constructor(
        string[] memory urls,
        uint256 window,
        IVerifierHooks hooks,
        IScrollChain rollup
    ) AbstractVerifier(urls, window, hooks) {
        _rollup = rollup;
    }

    function getLatestContext() external view returns (bytes memory) {
        return abi.encode(_rollup.lastFinalizedBatchIndex());
    }

    struct GatewayProof {
        uint256 batchIndex;
        bytes[] proofs;
        bytes order;
    }

    function getStorageValues(
        bytes memory context,
        GatewayRequest memory req,
        bytes memory proof
    ) external view returns (bytes[] memory, uint8 exitCode) {
        uint256 batchIndex1 = abi.decode(context, (uint256));
        GatewayProof memory p = abi.decode(proof, (GatewayProof));
        _checkWindow(batchIndex1, p.batchIndex);
        bytes32 stateRoot = _rollup.finalizedStateRoots(p.batchIndex);
        require(stateRoot != bytes32(0), 'Scroll: not finalized');
        return
            GatewayVM.evalRequest(
                req,
                ProofSequence(0, stateRoot, p.proofs, p.order, _hooks)
            );
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {ArbitrumVerifier, IVerifierHooks} from './ArbitrumVerifier.sol';
import {NitroVerifierLib} from './NitroVerifierLib.sol';
import {GatewayRequest, GatewayVM, ProofSequence} from '../GatewayVM.sol';

contract DoubleArbitrumVerifier is ArbitrumVerifier {
    GatewayRequest public request;

    constructor(
        string[] memory urls,
        uint256 window,
        IVerifierHooks hooks,
        address rollup12,
        uint256 minAgeBlocks12,
        bool isBoLD12,
        GatewayRequest memory _request
    )
        ArbitrumVerifier(
            urls,
            window,
            hooks,
            rollup12,
            minAgeBlocks12,
            isBoLD12
        )
    {
        request = _request;
    }

    function getStorageValues(
        bytes memory context,
        GatewayRequest memory req,
        bytes memory proof
    ) external view override returns (bytes[] memory, uint8 exitCode) {
        GatewayProof[2] memory ps = abi.decode(proof, (GatewayProof[2]));
        bytes32 stateRoot = _verifyRollup(ps[0], context);
        (bytes[] memory outputs, ) = GatewayVM.evalRequest(
            request,
            ProofSequence(0, stateRoot, ps[0].proofs, ps[0].order, _hooks)
        );
        // outputs[0] = node
        // outputs[1] = confirmData
        // outputs[2] = createdAtBlock (not used yet)
        NitroVerifierLib.RollupProof memory p = abi.decode(
            ps[1].rollupProof,
            (NitroVerifierLib.RollupProof)
        );
        stateRoot = NitroVerifierLib.verifyStateRoot(p, bytes32(outputs[1]));
        return
            GatewayVM.evalRequest(
                req,
                ProofSequence(0, stateRoot, ps[1].proofs, ps[1].order, _hooks)
            );
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {GatewayRequest, GatewayVM, ProofSequence} from '../GatewayVM.sol';
import {AbstractVerifier, IVerifierHooks} from '../AbstractVerifier.sol';
import {NitroVerifierLib} from './NitroVerifierLib.sol';
import {BoLDVerifierLib} from './BoLDVerifierLib.sol';

contract ArbitrumVerifier is AbstractVerifier {
    address public immutable rollup;
    uint256 public immutable minAgeBlocks;
    bool public immutable isBoLD;

    constructor(
        string[] memory urls,
        uint256 window,
        IVerifierHooks hooks,
        address _rollup,
        uint256 _minAgeBlocks,
        bool _isBoLD
    ) AbstractVerifier(urls, window, hooks) {
        rollup = _rollup;
        minAgeBlocks = _minAgeBlocks;
        isBoLD = _isBoLD;
    }

    function getLatestContext() external view returns (bytes memory) {
        return
            abi.encode(
                isBoLD
                    ? BoLDVerifierLib.latestIndex(rollup, minAgeBlocks)
                    : NitroVerifierLib.latestIndex(rollup, minAgeBlocks)
            );
    }

    struct GatewayProof {
        bytes rollupProof;
        bytes[] proofs;
        bytes order;
    }

    function _verifyRollup(
        GatewayProof memory p,
        bytes memory context
    ) internal view returns (bytes32 stateRoot) {
        uint256 latest = abi.decode(context, (uint256));
        uint256 got;
        if (isBoLD) {
            (stateRoot, got) = BoLDVerifierLib.verifyRollup(
                rollup,
                minAgeBlocks,
                p.rollupProof
            );
        } else {
            (stateRoot, latest, got) = NitroVerifierLib.verifyRollup(
                rollup,
                minAgeBlocks,
                p.rollupProof,
                uint64(latest)
            );
        }
        _checkWindow(latest, got);
    }

    function getStorageValues(
        bytes memory context,
        GatewayRequest memory req,
        bytes memory proof
    ) external view virtual returns (bytes[] memory, uint8 exitCode) {
        GatewayProof memory p = abi.decode(proof, (GatewayProof));
        bytes32 stateRoot = _verifyRollup(p, context);
        return
            GatewayVM.evalRequest(
                req,
                ProofSequence(0, stateRoot, p.proofs, p.order, _hooks)
            );
    }
}

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
pragma solidity ~0.8.17;

import "./IPriceOracle.sol";
import "../utils/StringUtils.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/introspection/IERC165.sol";

interface AggregatorInterface {
    function latestAnswer() external view returns (int256);
}

// StablePriceOracle sets a price in USD, based on an oracle.
contract StablePriceOracle is IPriceOracle {
    using StringUtils for *;

    // Rent in base price units by length
    uint256 public immutable price1Letter;
    uint256 public immutable price2Letter;
    uint256 public immutable price3Letter;
    uint256 public immutable price4Letter;
    uint256 public immutable price5Letter;

    // Oracle address
    AggregatorInterface public immutable usdOracle;

    event RentPriceChanged(uint256[] prices);

    constructor(AggregatorInterface _usdOracle, uint256[] memory _rentPrices) {
        usdOracle = _usdOracle;
        price1Letter = _rentPrices[0];
        price2Letter = _rentPrices[1];
        price3Letter = _rentPrices[2];
        price4Letter = _rentPrices[3];
        price5Letter = _rentPrices[4];
    }

    function price(
        string calldata name,
        uint256 expires,
        uint256 duration
    ) external view override returns (IPriceOracle.Price memory) {
        uint256 len = name.strlen();
        uint256 basePrice;

        if (len >= 5) {
            basePrice = price5Letter * duration;
        } else if (len == 4) {
            basePrice = price4Letter * duration;
        } else if (len == 3) {
            basePrice = price3Letter * duration;
        } else if (len == 2) {
            basePrice = price2Letter * duration;
        } else {
            basePrice = price1Letter * duration;
        }

        return
            IPriceOracle.Price({
                base: attoUSDToWei(basePrice),
                premium: attoUSDToWei(_premium(name, expires, duration))
            });
    }

    /// @dev Returns the pricing premium in wei.
    function premium(
        string calldata name,
        uint256 expires,
        uint256 duration
    ) external view returns (uint256) {
        return attoUSDToWei(_premium(name, expires, duration));
    }

    /// @dev Returns the pricing premium in internal base units.
    function _premium(
        string memory name,
        uint256 expires,
        uint256 duration
    ) internal view virtual returns (uint256) {
        return 0;
    }

    function attoUSDToWei(uint256 amount) internal view returns (uint256) {
        uint256 ethPrice = uint256(usdOracle.latestAnswer());
        return (amount * 1e8) / ethPrice;
    }

    function weiToAttoUSD(uint256 amount) internal view returns (uint256) {
        uint256 ethPrice = uint256(usdOracle.latestAnswer());
        return (amount * ethPrice) / 1e8;
    }

    function supportsInterface(
        bytes4 interfaceID
    ) public view virtual returns (bool) {
        return
            interfaceID == type(IERC165).interfaceId ||
            interfaceID == type(IPriceOracle).interfaceId;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import {
    ERC165
} from "@openzeppelin/contracts-v5/utils/introspection/ERC165.sol";
import {IExtendedResolver} from "../resolvers/profiles/IExtendedResolver.sol";
import {IAddressResolver} from "../resolvers/profiles/IAddressResolver.sol";
import {IAddrResolver} from "../resolvers/profiles/IAddrResolver.sol";
import {INameResolver} from "../resolvers/profiles/INameResolver.sol";
import {INameReverser} from "./INameReverser.sol";
import {IERC7996} from "../utils/IERC7996.sol";
import {ENSIP19, COIN_TYPE_DEFAULT, COIN_TYPE_ETH} from "../utils/ENSIP19.sol";

abstract contract AbstractReverseResolver is
    IExtendedResolver,
    INameReverser,
    IERC7996,
    ERC165
{
    /// @inheritdoc INameReverser
    uint256 public immutable coinType;

    /// @inheritdoc INameReverser
    address public immutable chainRegistrar;

    /// @notice `resolve()` was called with a profile other than `name()` or `addr(*)`.
    /// @dev Error selector: `0x7b1c461b`
    error UnsupportedResolverProfile(bytes4 selector);

    /// @notice `name` is not a valid DNS-encoded ENSIP-19 reverse name or namespace.
    /// @dev Error selector: `0x5fe9a5df`
    error UnreachableName(bytes name);

    constructor(uint256 _coinType, address registrar) {
        coinType = _coinType;
        chainRegistrar = registrar;
    }

    /// @inheritdoc ERC165
    function supportsInterface(
        bytes4 interfaceId
    ) public view virtual override returns (bool) {
        return
            interfaceId == type(IExtendedResolver).interfaceId ||
            interfaceId == type(INameReverser).interfaceId ||
            interfaceId == type(IERC7996).interfaceId ||
            super.supportsInterface(interfaceId);
    }

    /// @inheritdoc IERC7996
    function supportsFeature(bytes4) external pure returns (bool) {
        return false;
    }

    /// @inheritdoc INameReverser
    function chainId() external view returns (uint32) {
        return ENSIP19.chainFromCoinType(coinType);
    }

    /// @dev Resolve one address to a name.
    ///      If this reverts `OffchainLookup`, it must return an abi-encoded result since
    ///      it is invoked during `resolve()`.
    function _resolveName(
        address addr
    ) internal view virtual returns (string memory name);

    /// @notice Resolves the following profiles according to ENSIP-10:
    ///         - `name()` if `name` is an ENSIP-19 reverse name of an EVM address for `coinType`.
    ///         - `addr(*) = registrar` if `name` is an ENSIP-19 reverse namespace for `coinType`.
    ///         Caller should enable EIP-3668.
    /// @dev This function may execute over multiple steps.
    /// @param name The reverse name to resolve, in normalised and DNS-encoded form.
    /// @param data The resolution data, as specified in ENSIP-10.
    /// @return result The encoded response for the requested profile.
    function resolve(
        bytes calldata name,
        bytes calldata data
    ) external view returns (bytes memory result) {
        bytes4 selector = bytes4(data);
        if (selector == INameResolver.name.selector) {
            (bytes memory a, uint256 ct) = ENSIP19.parse(name);
            if (
                a.length != 20 ||
                !(
                    coinType == COIN_TYPE_DEFAULT
                        ? ENSIP19.isEVMCoinType(ct)
                        : ct == coinType
                )
            ) {
                revert UnreachableName(name);
            }
            address addr = address(bytes20(a));
            return abi.encode(_resolveName(addr));
        } else if (selector == IAddrResolver.addr.selector) {
            (bool valid, ) = ENSIP19.parseNamespace(name, 0);
            if (!valid) revert UnreachableName(name);
            return
                abi.encode(
                    coinType == COIN_TYPE_ETH ? chainRegistrar : address(0)
                );
        } else if (selector == IAddressResolver.addr.selector) {
            (bool valid, ) = ENSIP19.parseNamespace(name, 0);
            if (!valid) revert UnreachableName(name);
            (, uint256 ct) = abi.decode(data[4:], (bytes32, uint256));
            return
                abi.encode(
                    coinType == ct
                        ? abi.encodePacked(chainRegistrar)
                        : new bytes(0)
                );
        } else {
            revert UnsupportedResolverProfile(selector);
        }
    }

    // `INameReverser.resolveNames()` is not implemented here because it causes
    // an incorrect "Unreachable code" compiler warning if `_resolveName()` reverts.
    // https://github.com/ethereum/solidity/issues/15426#issuecomment-2917868211
    //
    // /// @inheritdoc INameReverser
    // function resolveNames(
    //     address[] memory addrs,
    //     uint8 /*perPage*/
    // ) external view returns (string[] memory names) {
    //     names = new string[](addrs.length);
    //     for (uint256 i; i < addrs.length; i++) {
    //         names[i] = _resolveName(addrs[i]);
    //     }
    // }
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
pragma solidity ^0.8.23;

import {AbstractVerifier, IVerifierHooks} from './AbstractVerifier.sol';
import {GatewayRequest, GatewayVM, ProofSequence} from './GatewayVM.sol';

contract InteractiveVerifier is AbstractVerifier {
    event NewStateRoot(
        uint256 indexed prevIndex,
        uint256 indexed index,
        bytes32 stateRoot
    );

    struct Commit {
        bytes32 stateRoot;
        uint256 prevIndex;
    }

    mapping(uint256 => Commit) public commits;
    uint256 public latestIndex;

    constructor(
        string[] memory urls,
        uint256 window,
        IVerifierHooks hooks
    ) AbstractVerifier(urls, window, hooks) {}

    function setStateRoot(uint256 index, bytes32 stateRoot) external onlyOwner {
        require(index > latestIndex, 'out of order');
        commits[index] = Commit(stateRoot, latestIndex);
        emit NewStateRoot(latestIndex, index, stateRoot);
        latestIndex = index;
    }

    function getLatestContext() external view returns (bytes memory) {
        return abi.encode(latestIndex);
    }

    function getStorageValues(
        bytes memory context,
        GatewayRequest memory req,
        bytes memory proof
    ) external view returns (bytes[] memory values, uint8 exitCode) {
        uint256 index1 = abi.decode(context, (uint256));
        (uint256 index, bytes[] memory proofs, bytes memory order) = abi.decode(
            proof,
            (uint256, bytes[], bytes)
        );
        _checkWindow(index1, index);
        return
            GatewayVM.evalRequest(
                req,
                ProofSequence(
                    0,
                    commits[index].stateRoot,
                    proofs,
                    order,
                    _hooks
                )
            );
    }
}

pragma solidity >=0.8.4;

import "@openzeppelin/contracts/access/Ownable.sol";

import "../registry/ENS.sol";
import "./IReverseRegistrar.sol";
import "../root/Controllable.sol";

abstract contract NameResolver {
    function setName(bytes32 node, string memory name) public virtual;
}

bytes32 constant lookup = 0x3031323334353637383961626364656600000000000000000000000000000000;

bytes32 constant ADDR_REVERSE_NODE = 0x91d1777781884d03a6757a803996e38de2a42967fb37eeaca72729271025a9e2;

// namehash('addr.reverse')

contract ReverseRegistrar is Ownable, Controllable, IReverseRegistrar {
    ENS public immutable ens;
    NameResolver public defaultResolver;

    event ReverseClaimed(address indexed addr, bytes32 indexed node);
    event DefaultResolverChanged(NameResolver indexed resolver);

    /// @dev Constructor
    /// @param ensAddr The address of the ENS registry.
    constructor(ENS ensAddr) {
        ens = ensAddr;

        // Assign ownership of the reverse record to our deployer
        ReverseRegistrar oldRegistrar = ReverseRegistrar(
            ensAddr.owner(ADDR_REVERSE_NODE)
        );
        if (address(oldRegistrar) != address(0x0)) {
            oldRegistrar.claim(msg.sender);
        }
    }

    modifier authorised(address addr) {
        require(
            addr == msg.sender ||
                controllers[msg.sender] ||
                ens.isApprovedForAll(addr, msg.sender) ||
                ownsContract(addr),
            "ReverseRegistrar: Caller is not a controller or authorised by address or the address itself"
        );
        _;
    }

    function setDefaultResolver(address resolver) public override onlyOwner {
        require(
            address(resolver) != address(0),
            "ReverseRegistrar: Resolver address must not be 0"
        );
        defaultResolver = NameResolver(resolver);
        emit DefaultResolverChanged(NameResolver(resolver));
    }

    /// @dev Transfers ownership of the reverse ENS record associated with the
    ///      calling account.
    /// @param owner The address to set as the owner of the reverse record in ENS.
    /// @return The ENS node hash of the reverse record.
    function claim(address owner) public override returns (bytes32) {
        return claimForAddr(msg.sender, owner, address(defaultResolver));
    }

    /// @dev Transfers ownership of the reverse ENS record associated with the
    ///      calling account.
    /// @param addr The reverse record to set
    /// @param owner The address to set as the owner of the reverse record in ENS.
    /// @param resolver The resolver of the reverse node
    /// @return The ENS node hash of the reverse record.
    function claimForAddr(
        address addr,
        address owner,
        address resolver
    ) public override authorised(addr) returns (bytes32) {
        bytes32 labelHash = sha3HexAddress(addr);
        bytes32 reverseNode = keccak256(
            abi.encodePacked(ADDR_REVERSE_NODE, labelHash)
        );
        emit ReverseClaimed(addr, reverseNode);
        ens.setSubnodeRecord(ADDR_REVERSE_NODE, labelHash, owner, resolver, 0);
        return reverseNode;
    }

    /// @dev Transfers ownership of the reverse ENS record associated with the
    ///      calling account.
    /// @param owner The address to set as the owner of the reverse record in ENS.
    /// @param resolver The address of the resolver to set; 0 to leave unchanged.
    /// @return The ENS node hash of the reverse record.
    function claimWithResolver(
        address owner,
        address resolver
    ) public override returns (bytes32) {
        return claimForAddr(msg.sender, owner, resolver);
    }

    /// @dev Sets the `name()` record for the reverse ENS record associated with
    /// the calling account. First updates the resolver to the default reverse
    /// resolver if necessary.
    /// @param name The name to set for this address.
    /// @return The ENS node hash of the reverse record.
    function setName(string memory name) public override returns (bytes32) {
        return
            setNameForAddr(
                msg.sender,
                msg.sender,
                address(defaultResolver),
                name
            );
    }

    /// @dev Sets the `name()` record for the reverse ENS record associated with
    /// the account provided. Updates the resolver to a designated resolver
    /// Only callable by controllers and authorised users
    /// @param addr The reverse record to set
    /// @param owner The owner of the reverse node
    /// @param resolver The resolver of the reverse node
    /// @param name The name to set for this address.
    /// @return The ENS node hash of the reverse record.
    function setNameForAddr(
        address addr,
        address owner,
        address resolver,
        string memory name
    ) public override returns (bytes32) {
        bytes32 node = claimForAddr(addr, owner, resolver);
        NameResolver(resolver).setName(node, name);
        return node;
    }

    /// @dev Returns the node hash for a given account's reverse records.
    /// @param addr The address to hash
    /// @return The ENS node hash.
    function node(address addr) public pure override returns (bytes32) {
        return
            keccak256(
                abi.encodePacked(ADDR_REVERSE_NODE, sha3HexAddress(addr))
            );
    }

    /// @dev An optimised function to compute the sha3 of the lower-case
    ///      hexadecimal representation of an Ethereum address.
    /// @param addr The address to hash
    /// @return ret The SHA3 hash of the lower-case hexadecimal encoding of the
    ///         input address.
    function sha3HexAddress(address addr) private pure returns (bytes32 ret) {
        assembly {
            for {
                let i := 40
            } gt(i, 0) {} {
                i := sub(i, 1)
                mstore8(i, byte(and(addr, 0xf), lookup))
                addr := div(addr, 0x10)
                i := sub(i, 1)
                mstore8(i, byte(and(addr, 0xf), lookup))
                addr := div(addr, 0x10)
            }

            ret := keccak256(0, 40)
        }
    }

    function ownsContract(address addr) internal view returns (bool) {
        try Ownable(addr).owner() returns (address owner) {
            return owner == msg.sender;
        } catch {
            return false;
        }
    }
}

pragma solidity >=0.8.4;

import "./ENS.sol";

/// The ENS registry contract.
contract ENSRegistry is ENS {
    struct Record {
        address owner;
        address resolver;
        uint64 ttl;
    }

    mapping(bytes32 => Record) records;
    mapping(address => mapping(address => bool)) operators;

    // Permits modifications only by the owner of the specified node.
    modifier authorised(bytes32 node) {
        address owner = records[node].owner;
        require(owner == msg.sender || operators[owner][msg.sender]);
        _;
    }

    /// @dev Constructs a new ENS registry.
    constructor() public {
        records[0x0].owner = msg.sender;
    }

    /// @dev Sets the record for a node.
    /// @param node The node to update.
    /// @param owner The address of the new owner.
    /// @param resolver The address of the resolver.
    /// @param ttl The TTL in seconds.
    function setRecord(
        bytes32 node,
        address owner,
        address resolver,
        uint64 ttl
    ) external virtual override {
        setOwner(node, owner);
        _setResolverAndTTL(node, resolver, ttl);
    }

    /// @dev Sets the record for a subnode.
    /// @param node The parent node.
    /// @param label The hash of the label specifying the subnode.
    /// @param owner The address of the new owner.
    /// @param resolver The address of the resolver.
    /// @param ttl The TTL in seconds.
    function setSubnodeRecord(
        bytes32 node,
        bytes32 label,
        address owner,
        address resolver,
        uint64 ttl
    ) external virtual override {
        bytes32 subnode = setSubnodeOwner(node, label, owner);
        _setResolverAndTTL(subnode, resolver, ttl);
    }

    /// @dev Transfers ownership of a node to a new address. May only be called by the current owner of the node.
    /// @param node The node to transfer ownership of.
    /// @param owner The address of the new owner.
    function setOwner(
        bytes32 node,
        address owner
    ) public virtual override authorised(node) {
        _setOwner(node, owner);
        emit Transfer(node, owner);
    }

    /// @dev Transfers ownership of a subnode keccak256(node, label) to a new address. May only be called by the owner of the parent node.
    /// @param node The parent node.
    /// @param label The hash of the label specifying the subnode.
    /// @param owner The address of the new owner.
    function setSubnodeOwner(
        bytes32 node,
        bytes32 label,
        address owner
    ) public virtual override authorised(node) returns (bytes32) {
        bytes32 subnode = keccak256(abi.encodePacked(node, label));
        _setOwner(subnode, owner);
        emit NewOwner(node, label, owner);
        return subnode;
    }

    /// @dev Sets the resolver address for the specified node.
    /// @param node The node to update.
    /// @param resolver The address of the resolver.
    function setResolver(
        bytes32 node,
        address resolver
    ) public virtual override authorised(node) {
        emit NewResolver(node, resolver);
        records[node].resolver = resolver;
    }

    /// @dev Sets the TTL for the specified node.
    /// @param node The node to update.
    /// @param ttl The TTL in seconds.
    function setTTL(
        bytes32 node,
        uint64 ttl
    ) public virtual override authorised(node) {
        emit NewTTL(node, ttl);
        records[node].ttl = ttl;
    }

    /// @dev Enable or disable approval for a third party ("operator") to manage
    ///      all of `msg.sender`'s ENS records. Emits the ApprovalForAll event.
    /// @param operator Address to add to the set of authorized operators.
    /// @param approved True if the operator is approved, false to revoke approval.
    function setApprovalForAll(
        address operator,
        bool approved
    ) external virtual override {
        operators[msg.sender][operator] = approved;
        emit ApprovalForAll(msg.sender, operator, approved);
    }

    /// @dev Returns the address that owns the specified node.
    /// @param node The specified node.
    /// @return address of the owner.
    function owner(
        bytes32 node
    ) public view virtual override returns (address) {
        address addr = records[node].owner;
        if (addr == address(this)) {
            return address(0x0);
        }

        return addr;
    }

    /// @dev Returns the address of the resolver for the specified node.
    /// @param node The specified node.
    /// @return address of the resolver.
    function resolver(
        bytes32 node
    ) public view virtual override returns (address) {
        return records[node].resolver;
    }

    /// @dev Returns the TTL of a node, and any records associated with it.
    /// @param node The specified node.
    /// @return ttl of the node.
    function ttl(bytes32 node) public view virtual override returns (uint64) {
        return records[node].ttl;
    }

    /// @dev Returns whether a record has been imported to the registry.
    /// @param node The specified node.
    /// @return Bool if record exists
    function recordExists(
        bytes32 node
    ) public view virtual override returns (bool) {
        return records[node].owner != address(0x0);
    }

    /// @dev Query if an address is an authorized operator for another address.
    /// @param owner The address that owns the records.
    /// @param operator The address that acts on behalf of the owner.
    /// @return True if `operator` is an approved operator for `owner`, false otherwise.
    function isApprovedForAll(
        address owner,
        address operator
    ) external view virtual override returns (bool) {
        return operators[owner][operator];
    }

    function _setOwner(bytes32 node, address owner) internal virtual {
        records[node].owner = owner;
    }

    function _setResolverAndTTL(
        bytes32 node,
        address resolver,
        uint64 ttl
    ) internal {
        if (resolver != records[node].resolver) {
            records[node].resolver = resolver;
            emit NewResolver(node, resolver);
        }

        if (ttl != records[node].ttl) {
            records[node].ttl = ttl;
            emit NewTTL(node, ttl);
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {AbstractVerifier, IVerifierHooks} from '../AbstractVerifier.sol';
import {GatewayRequest, GatewayVM, ProofSequence} from '../GatewayVM.sol';
import {Hashing, Types} from '../../lib/optimism/packages/contracts-bedrock/src/libraries/Hashing.sol';
import { IOptimismPortal, IOPFaultGameFinder, IDisputeGame, OPFaultParams } from './OPInterfaces.sol';



contract OPFaultVerifier is AbstractVerifier {
    IOptimismPortal immutable _portal;
    IOPFaultGameFinder immutable _gameFinder;
    OPFaultParams private _params;

    constructor(
        string[] memory urls,
        uint256 window,
        IVerifierHooks hooks,
        IOPFaultGameFinder gameFinder,
        OPFaultParams memory params
    ) AbstractVerifier(urls, window, hooks) {
        _portal = params.portal;
        _gameFinder = gameFinder;
        _params = params;
    }

    function getLatestContext() external view virtual returns (bytes memory) {
        return
            abi.encode(
                _gameFinder.findGameIndex(
                    _params,
                    0
                )
            );
    }

    struct GatewayProof {
        uint256 gameIndex;
        Types.OutputRootProof outputRootProof;
        bytes[] proofs;
        bytes order;
    }

    function getStorageValues(
        bytes memory context,
        GatewayRequest memory req,
        bytes memory proof
    ) external view returns (bytes[] memory, uint8 exitCode) {
        uint256 gameIndex1 = abi.decode(context, (uint256));
        GatewayProof memory p = abi.decode(proof, (GatewayProof));
        (, , IDisputeGame gameProxy, uint256 blockNumber,) = _gameFinder
            .gameAtIndex(_params, p.gameIndex);
        require(blockNumber != 0, 'OPFault: invalid game');
        if (p.gameIndex != gameIndex1) {
            (, , IDisputeGame gameProxy1) = _portal
                .disputeGameFactory()
                .gameAtIndex(gameIndex1);
            _checkWindow(_getGameTime(gameProxy1), _getGameTime(gameProxy));
        }
        require(
            gameProxy.rootClaim() ==
                Hashing.hashOutputRootProof(p.outputRootProof),
            'OPFault: rootClaim'
        );
        return
            GatewayVM.evalRequest(
                req,
                ProofSequence(
                    0,
                    p.outputRootProof.stateRoot,
                    p.proofs,
                    p.order,
                    _hooks
                )
            );
    }

    function _getGameTime(IDisputeGame g) internal view returns (uint256) {
        return
            _params.minAgeSec == 0 ? g.resolvedAt() : g.createdAt();
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

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {AbstractVerifier, IVerifierHooks} from '../AbstractVerifier.sol';
import {GatewayRequest, GatewayVM, ProofSequence} from '../GatewayVM.sol';

interface IZKSyncDiamond {
    function storedBatchHash(
        uint256 batchNumber
    ) external view returns (bytes32);
    function l2LogsRootHash(
        uint256 batchNumber
    ) external view returns (bytes32);
    function getTotalBatchesExecuted() external view returns (uint256);
}

struct StoredBatchInfo {
    uint64 batchNumber;
    bytes32 batchHash;
    uint64 indexRepeatedStorageChanges;
    uint256 numberOfLayer1Txs;
    bytes32 priorityOperationsHash;
    bytes32 l2LogsTreeRoot;
    uint256 timestamp;
    bytes32 commitment;
}

contract ZKSyncVerifier is AbstractVerifier {
    IZKSyncDiamond immutable _diamond;

    constructor(
        string[] memory urls,
        uint256 window,
        IVerifierHooks hooks,
        IZKSyncDiamond diamond
    ) AbstractVerifier(urls, window, hooks) {
        _diamond = diamond;
    }

    function getLatestContext() external view returns (bytes memory) {
        return abi.encode(_diamond.getTotalBatchesExecuted() - 1);
    }

    struct GatewayProof {
        bytes encodedBatch;
        bytes[] proofs;
        bytes order;
    }

    function getStorageValues(
        bytes memory context,
        GatewayRequest memory req,
        bytes memory proof
    ) external view returns (bytes[] memory, uint8 exitCode) {
        uint256 batchIndex1 = abi.decode(context, (uint256));
        GatewayProof memory p = abi.decode(proof, (GatewayProof));
        StoredBatchInfo memory batchInfo = abi.decode(
            p.encodedBatch,
            (StoredBatchInfo)
        );
        _checkWindow(batchIndex1, batchInfo.batchNumber);
        require(
            keccak256(p.encodedBatch) ==
                _diamond.storedBatchHash(batchInfo.batchNumber),
            'ZKS: batchHash'
        );
        require(
            batchInfo.l2LogsTreeRoot ==
                _diamond.l2LogsRootHash(batchInfo.batchNumber),
            'ZKS: l2LogsRootHash'
        );
        return
            GatewayVM.evalRequest(
                req,
                ProofSequence(0, batchInfo.batchHash, p.proofs, p.order, _hooks)
            );
    }
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.8.4;

import {ResolverBase, IERC165} from "../ResolverBase.sol";
import {IAddrResolver} from "./IAddrResolver.sol";
import {IAddressResolver} from "./IAddressResolver.sol";
import {IHasAddressResolver} from "./IHasAddressResolver.sol";
import {ENSIP19, COIN_TYPE_ETH, COIN_TYPE_DEFAULT} from "../../utils/ENSIP19.sol";

abstract contract AddrResolver is
    IAddrResolver,
    IAddressResolver,
    IHasAddressResolver,
    ResolverBase
{
    mapping(uint64 => mapping(bytes32 => mapping(uint256 => bytes))) versionable_addresses;

    /// @notice The supplied address could not be converted to `address`.
    /// @dev Error selector: `0x8d666f60`
    error InvalidEVMAddress(bytes addressBytes);

    /// @notice Set `addr(60)` of the associated ENS node.
    ///         `address(0)` is stored as `new bytes(20)`.
    /// @param node The node to update.
    /// @param _addr The address to set.
    function setAddr(
        bytes32 node,
        address _addr
    ) external virtual authorised(node) {
        setAddr(node, COIN_TYPE_ETH, abi.encodePacked(_addr));
    }

    /// @notice Get `addr(60)` as `address` of the associated ENS node.
    /// @param node The node to query.
    /// @return The associated address.
    function addr(
        bytes32 node
    ) public view virtual override returns (address payable) {
        return payable(address(bytes20(addr(node, COIN_TYPE_ETH))));
    }

    /// @notice Set the address for coin type of the associated ENS node.
    ///         Reverts `InvalidEVMAddress` if coin type is EVM and not 0 or 20 bytes.
    /// @param node The node to update.
    /// @param coinType The coin type.
    /// @param addressBytes The address to set.
    function setAddr(
        bytes32 node,
        uint256 coinType,
        bytes memory addressBytes
    ) public virtual authorised(node) {
        if (
            addressBytes.length != 0 &&
            addressBytes.length != 20 &&
            ENSIP19.isEVMCoinType(coinType)
        ) {
            revert InvalidEVMAddress(addressBytes);
        }
        emit AddressChanged(node, coinType, addressBytes);
        if (coinType == COIN_TYPE_ETH) {
            emit AddrChanged(node, address(bytes20(addressBytes)));
        }
        versionable_addresses[recordVersions[node]][node][
            coinType
        ] = addressBytes;
    }

    /// @notice Get the address for coin type of the associated ENS node.
    ///         If coin type is EVM and empty, defaults to `addr(COIN_TYPE_DEFAULT)`.
    /// @param node The node to query.
    /// @param coinType The coin type.
    /// @return addressBytes The assocated address.
    function addr(
        bytes32 node,
        uint256 coinType
    ) public view virtual override returns (bytes memory addressBytes) {
        mapping(uint256 => bytes) storage addrs = versionable_addresses[
            recordVersions[node]
        ][node];
        addressBytes = addrs[coinType];
        if (
            addressBytes.length == 0 && ENSIP19.chainFromCoinType(coinType) > 0
        ) {
            addressBytes = addrs[COIN_TYPE_DEFAULT];
        }
    }

    /// @inheritdoc IHasAddressResolver
    function hasAddr(
        bytes32 node,
        uint256 coinType
    ) external view returns (bool) {
        return
            versionable_addresses[recordVersions[node]][node][coinType].length >
            0;
    }

    /// @inheritdoc IERC165
    function supportsInterface(
        bytes4 interfaceId
    ) public view virtual override returns (bool) {
        return
            type(IAddrResolver).interfaceId == interfaceId ||
            type(IAddressResolver).interfaceId == interfaceId ||
            type(IHasAddressResolver).interfaceId == interfaceId ||
            super.supportsInterface(interfaceId);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {AbstractVerifier, IVerifierHooks} from '../AbstractVerifier.sol';
import {GatewayRequest, GatewayVM, ProofSequence} from '../GatewayVM.sol';
import {ILineaRollup} from './ILineaRollup.sol';

contract LineaVerifier is AbstractVerifier {
    ILineaRollup immutable _rollup;

    constructor(
        string[] memory urls,
        uint256 window,
        IVerifierHooks hooks,
        ILineaRollup rollup
    ) AbstractVerifier(urls, window, hooks) {
        _rollup = rollup;
    }

    function getLatestContext() external view returns (bytes memory) {
        return abi.encode(_rollup.currentL2BlockNumber());
    }

    struct GatewayProof {
        uint256 l2BlockNumber;
        bytes[] proofs;
        bytes order;
    }

    function getStorageValues(
        bytes memory context,
        GatewayRequest memory req,
        bytes memory proof
    ) external view returns (bytes[] memory, uint8 exitCode) {
        uint256 l2BlockNumber1 = abi.decode(context, (uint256));
        GatewayProof memory p = abi.decode(proof, (GatewayProof));
        _checkWindow(l2BlockNumber1, p.l2BlockNumber);
        bytes32 stateRoot = _rollup.stateRootHashes(p.l2BlockNumber);
        if (stateRoot == bytes32(0)) revert('Linea: not finalized');
        return
            GatewayVM.evalRequest(
                req,
                ProofSequence(0, stateRoot, p.proofs, p.order, _hooks)
            );
    }
}

//SPDX-License-Identifier: MIT
pragma solidity ~0.8.17;

import "./SafeMath.sol";
import "./StablePriceOracle.sol";

contract LinearPremiumPriceOracle is StablePriceOracle {
    using SafeMath for *;

    uint256 immutable GRACE_PERIOD = 90 days;

    uint256 public immutable initialPremium;
    uint256 public immutable premiumDecreaseRate;

    bytes4 private constant TIME_UNTIL_PREMIUM_ID =
        bytes4(keccak256("timeUntilPremium(uint,uint"));

    constructor(
        AggregatorInterface _usdOracle,
        uint256[] memory _rentPrices,
        uint256 _initialPremium,
        uint256 _premiumDecreaseRate
    ) public StablePriceOracle(_usdOracle, _rentPrices) {
        initialPremium = _initialPremium;
        premiumDecreaseRate = _premiumDecreaseRate;
    }

    function _premium(
        string memory name,
        uint256 expires,
        uint256 /*duration*/
    ) internal view override returns (uint256) {
        expires = expires.add(GRACE_PERIOD);
        if (expires > block.timestamp) {
            // No premium for renewals
            return 0;
        }

        // Calculate the discount off the maximum premium
        uint256 discount = premiumDecreaseRate.mul(
            block.timestamp.sub(expires)
        );

        // If we've run out the premium period, return 0.
        if (discount > initialPremium) {
            return 0;
        }

        return initialPremium - discount;
    }

    /// @dev Returns the timestamp at which a name with the specified expiry date will have
    ///      the specified re-registration price premium.
    /// @param expires The timestamp at which the name expires.
    /// @param amount The amount, in wei, the caller is willing to pay
    /// @return The timestamp at which the premium for this domain will be `amount`.
    function timeUntilPremium(
        uint256 expires,
        uint256 amount
    ) external view returns (uint256) {
        amount = weiToAttoUSD(amount);
        require(amount <= initialPremium);

        expires = expires.add(GRACE_PERIOD);

        uint256 discount = initialPremium.sub(amount);
        uint256 duration = discount.div(premiumDecreaseRate);
        return expires.add(duration);
    }

    function supportsInterface(
        bytes4 interfaceID
    ) public view virtual override returns (bool) {
        return
            (interfaceID == TIME_UNTIL_PREMIUM_ID) ||
            super.supportsInterface(interfaceID);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {IGatewayVerifier} from '../IGatewayVerifier.sol';
import {IVerifierHooks} from '../IVerifierHooks.sol';
import {GatewayRequest, GatewayVM, ProofSequence} from '../GatewayVM.sol';
import {ECDSA} from '@openzeppelin/contracts/utils/cryptography/ECDSA.sol';
import {LazyOwnable} from './LazyOwnable.sol';

event TrustedVerifierChanged();

contract LazyTrustedVerifier is LazyOwnable, IGatewayVerifier {

    IVerifierHooks _hooks;
    mapping(address => bool) _signers;
    uint256 _expSec;
    string[] _urls;

    function init(
        address _owner,
        IVerifierHooks hooks,
        string[] memory urls,
        address[] memory signers,
        uint256 expSec
    ) external {
        initialize(_owner, hooks, urls, signers, expSec);
    }

    function initialize(
        address _owner,
        IVerifierHooks hooks,
        string[] memory urls,
        address[] memory signers,
        uint256 expSec
    ) internal {
        super.initialize(_owner);
        _hooks = hooks;
        _urls = urls;
        for (uint256 i; i < signers.length; i++) {
            _signers[signers[i]] = true;
        }
        _expSec = expSec;
    }

    function getExpSec() external view returns (uint256) {
        return _expSec;
    }

    function getHooks() external view returns (IVerifierHooks) {
        return _hooks;
    }

    function isSigner(address signer) external view returns (bool) {
        return _signers[signer];
    }

    function setGatewayURLs(string[] memory urls) external onlyOwner {
        _urls = urls;
        emit TrustedVerifierChanged();
    }

    function setHooks(IVerifierHooks hooks) external onlyOwner {
        _hooks = hooks;
        emit TrustedVerifierChanged();
    }

    function setExpSec(uint256 expSec) external onlyOwner {
        _expSec = expSec;
        emit TrustedVerifierChanged();
    }

    function setSigner(address signer, bool allow) external onlyOwner {
        _signers[signer] = allow;
        emit TrustedVerifierChanged();
    }

    function gatewayURLs() external view returns (string[] memory) {
        return _urls;
    }

    function getLatestContext() external view returns (bytes memory) {
        return abi.encode(block.timestamp);
    }

    struct GatewayProof {
        bytes signature;
        uint64 signedAt;
        bytes32 stateRoot;
        bytes[] proofs;
        bytes order;
    }

    function getStorageValues(
        bytes memory context,
        GatewayRequest memory req,
        bytes memory proof
    ) external view returns (bytes[] memory, uint8 exitCode) {
        uint256 t = abi.decode(context, (uint256));
        GatewayProof memory p = abi.decode(proof, (GatewayProof));
        bytes32 hash = keccak256(
            // https://github.com/ethereum/eips/issues/191
            abi.encodePacked(
                hex'1900', // magic + version(0)
                address(0), // unbound
                p.signedAt,
                p.stateRoot
            )
        );
        address signer = ECDSA.recover(hash, p.signature);
        require(_signers[signer], 'Trusted: signer');
        uint256 dt = p.signedAt > t ? p.signedAt - t : t - p.signedAt;
        require(dt <= _expSec, 'Trusted: expired');
        return
            GatewayVM.evalRequest(
                req,
                ProofSequence(0, p.stateRoot, p.proofs, p.order, _hooks)
            );
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

import {Address} from "@openzeppelin/contracts/utils/Address.sol";
import {ERC165, IERC165} from "@openzeppelin/contracts/utils/introspection/ERC165.sol";

import "../../contracts/resolvers/profiles/IAddrResolver.sol";
import "../../contracts/resolvers/profiles/IExtendedResolver.sol";
import "../../contracts/resolvers/profiles/IExtendedDNSResolver.sol";
import "../dnssec-oracle/DNSSEC.sol";
import "../dnssec-oracle/RRUtils.sol";
import "../registry/ENSRegistry.sol";
import "../utils/HexUtils.sol";
import "../utils/BytesUtils.sol";
import {IDNSGateway} from "../dnssec-oracle/IDNSGateway.sol";
import {OffchainLookup} from "../ccipRead/EIP3668.sol";
import {LowLevelCallUtils} from "../utils/LowLevelCallUtils.sol";

error InvalidOperation();

uint16 constant CLASS_INET = 1;
uint16 constant TYPE_TXT = 16;

contract OffchainDNSResolver is IExtendedResolver, IERC165 {
    using RRUtils for *;
    using Address for address;
    using BytesUtils for bytes;
    using HexUtils for bytes;

    ENS public immutable ens;
    DNSSEC public immutable oracle;
    string public gatewayURL;

    error CouldNotResolve(bytes name);

    constructor(ENS _ens, DNSSEC _oracle, string memory _gatewayURL) {
        ens = _ens;
        oracle = _oracle;
        gatewayURL = _gatewayURL;
    }

    function supportsInterface(
        bytes4 interfaceId
    ) external pure override returns (bool) {
        return interfaceId == type(IExtendedResolver).interfaceId;
    }

    function resolve(
        bytes calldata name,
        bytes calldata data
    ) external view returns (bytes memory) {
        revertWithDefaultOffchainLookup(name, data);
    }

    function resolveCallback(
        bytes calldata response,
        bytes calldata extraData
    ) external view returns (bytes memory) {
        (bytes memory name, bytes memory query, bytes4 selector) = abi.decode(
            extraData,
            (bytes, bytes, bytes4)
        );

        if (selector != bytes4(0)) {
            (bytes memory targetData, address targetResolver) = abi.decode(
                query,
                (bytes, address)
            );
            return
                callWithOffchainLookupPropagation(
                    targetResolver,
                    name,
                    query,
                    abi.encodeWithSelector(
                        selector,
                        response,
                        abi.encode(targetData, address(this))
                    )
                );
        }

        DNSSEC.RRSetWithSignature[] memory rrsets = abi.decode(
            response,
            (DNSSEC.RRSetWithSignature[])
        );

        (bytes memory data, ) = oracle.verifyRRSet(rrsets);
        for (
            RRUtils.RRIterator memory iter = data.iterateRRs(0);
            !iter.done();
            iter.next()
        ) {
            // Ignore records with wrong name, type, or class
            bytes memory rrname = RRUtils.readName(iter.data, iter.offset);
            if (
                !rrname.equals(name) ||
                iter.class != CLASS_INET ||
                iter.dnstype != TYPE_TXT
            ) {
                continue;
            }

            // Look for a valid ENS-DNS TXT record
            (address dnsresolver, bytes memory context) = parseRR(
                iter.data,
                iter.rdataOffset,
                iter.nextOffset
            );

            // If we found a valid record, try to resolve it
            if (dnsresolver != address(0)) {
                if (
                    IERC165(dnsresolver).supportsInterface(
                        IExtendedDNSResolver.resolve.selector
                    )
                ) {
                    return
                        callWithOffchainLookupPropagation(
                            dnsresolver,
                            name,
                            query,
                            abi.encodeCall(
                                IExtendedDNSResolver.resolve,
                                (name, query, context)
                            )
                        );
                } else if (
                    IERC165(dnsresolver).supportsInterface(
                        IExtendedResolver.resolve.selector
                    )
                ) {
                    return
                        callWithOffchainLookupPropagation(
                            dnsresolver,
                            name,
                            query,
                            abi.encodeCall(
                                IExtendedResolver.resolve,
                                (name, query)
                            )
                        );
                } else {
                    (bool ok, bytes memory ret) = address(dnsresolver)
                        .staticcall(query);
                    if (ok) {
                        return ret;
                    } else {
                        revert CouldNotResolve(name);
                    }
                }
            }
        }

        // No valid records; revert.
        revert CouldNotResolve(name);
    }

    function parseRR(
        bytes memory data,
        uint256 idx,
        uint256 lastIdx
    ) internal view returns (address, bytes memory) {
        bytes memory txt = readTXT(data, idx, lastIdx);

        // Must start with the magic word
        if (txt.length < 5 || !txt.equals(0, "ENS1 ", 0, 5)) {
            return (address(0), "");
        }

        // Parse the name or address
        uint256 lastTxtIdx = txt.find(5, txt.length - 5, " ");
        if (lastTxtIdx > txt.length) {
            address dnsResolver = parseAndResolve(txt, 5, txt.length);
            return (dnsResolver, "");
        } else {
            address dnsResolver = parseAndResolve(txt, 5, lastTxtIdx);
            return (
                dnsResolver,
                txt.substring(lastTxtIdx + 1, txt.length - lastTxtIdx - 1)
            );
        }
    }

    function readTXT(
        bytes memory data,
        uint256 startIdx,
        uint256 lastIdx
    ) internal pure returns (bytes memory) {
        // TODO: Concatenate multiple text fields
        uint256 fieldLength = data.readUint8(startIdx);
        assert(startIdx + fieldLength < lastIdx);
        return data.substring(startIdx + 1, fieldLength);
    }

    function parseAndResolve(
        bytes memory nameOrAddress,
        uint256 idx,
        uint256 lastIdx
    ) internal view returns (address) {
        if (nameOrAddress[idx] == "0" && nameOrAddress[idx + 1] == "x") {
            (address ret, bool valid) = nameOrAddress.hexToAddress(
                idx + 2,
                lastIdx
            );
            if (valid) {
                return ret;
            }
        }
        return resolveName(nameOrAddress, idx, lastIdx);
    }

    function resolveName(
        bytes memory name,
        uint256 idx,
        uint256 lastIdx
    ) internal view returns (address) {
        bytes32 node = textNamehash(name, idx, lastIdx);
        address resolver = ens.resolver(node);
        if (resolver == address(0)) {
            return address(0);
        }
        return IAddrResolver(resolver).addr(node);
    }

    /// @dev Namehash function that operates on dot-separated names (not dns-encoded names)
    /// @param name Name to hash
    /// @param idx Index to start at
    /// @param lastIdx Index to end at
    function textNamehash(
        bytes memory name,
        uint256 idx,
        uint256 lastIdx
    ) internal view returns (bytes32) {
        uint256 separator = name.find(idx, name.length - idx, bytes1("."));
        bytes32 parentNode = bytes32(0);
        if (separator < lastIdx) {
            parentNode = textNamehash(name, separator + 1, lastIdx);
        } else {
            separator = lastIdx;
        }
        return
            keccak256(
                abi.encodePacked(parentNode, name.keccak(idx, separator - idx))
            );
    }

    function callWithOffchainLookupPropagation(
        address target,
        bytes memory name,
        bytes memory innerdata,
        bytes memory data
    ) internal view returns (bytes memory) {
        if (!target.isContract()) {
            revertWithDefaultOffchainLookup(name, innerdata);
        }

        bool result = LowLevelCallUtils.functionStaticCall(
            address(target),
            data
        );
        uint256 size = LowLevelCallUtils.returnDataSize();
        if (result) {
            bytes memory returnData = LowLevelCallUtils.readReturnData(0, size);
            return abi.decode(returnData, (bytes));
        }
        // Failure
        if (size >= 4) {
            bytes memory errorId = LowLevelCallUtils.readReturnData(0, 4);
            if (bytes4(errorId) == OffchainLookup.selector) {
                // Offchain lookup. Decode the revert message and create our own that nests it.
                bytes memory revertData = LowLevelCallUtils.readReturnData(
                    4,
                    size - 4
                );
                handleOffchainLookupError(revertData, target, name);
            }
        }
        LowLevelCallUtils.propagateRevert();
    }

    function revertWithDefaultOffchainLookup(
        bytes memory name,
        bytes memory data
    ) internal view {
        string[] memory urls = new string[](1);
        urls[0] = gatewayURL;

        revert OffchainLookup(
            address(this),
            urls,
            abi.encodeCall(IDNSGateway.resolve, (name, TYPE_TXT)),
            OffchainDNSResolver.resolveCallback.selector,
            abi.encode(name, data, bytes4(0))
        );
    }

    function handleOffchainLookupError(
        bytes memory returnData,
        address target,
        bytes memory name
    ) internal view {
        (
            address sender,
            string[] memory urls,
            bytes memory callData,
            bytes4 innerCallbackFunction,
            bytes memory extraData
        ) = abi.decode(returnData, (address, string[], bytes, bytes4, bytes));

        if (sender != target) {
            revert InvalidOperation();
        }

        revert OffchainLookup(
            address(this),
            urls,
            callData,
            OffchainDNSResolver.resolveCallback.selector,
            abi.encode(name, extraData, innerCallbackFunction)
        );
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {AbstractVerifier, IVerifierHooks} from '../AbstractVerifier.sol';
import {GatewayRequest, GatewayVM, ProofSequence} from '../GatewayVM.sol';
import {RLPReader, RLPReaderExt} from '../RLPReaderExt.sol';

interface IRootChain {
    // https://github.com/0xPolygon/pos-contracts/blob/main/contracts/root/IRootChain.sol
    function currentHeaderBlock() external view returns (uint256);
    // https://github.com/0xPolygon/pos-contracts/blob/main/contracts/root/RootChainStorage.sol
    function headerBlocks(
        uint256
    )
        external
        view
        returns (
            bytes32 root,
            uint256 start,
            uint256 end,
            uint256 createdAt,
            address proposer
        );
}

contract PolygonPoSVerifier is AbstractVerifier {
    IRootChain immutable _rootChain;
    address immutable _poster;

    constructor(
        string[] memory urls,
        uint256 window,
        IVerifierHooks hooks,
        IRootChain rootChain,
        address poster
    ) AbstractVerifier(urls, window, hooks) {
        _rootChain = rootChain;
        _poster = poster;
    }

    function getPoster() external view returns (address) {
        return _poster;
    }

    function getLatestContext() external view returns (bytes memory) {
        return abi.encode(_rootChain.currentHeaderBlock());
    }

    struct GatewayProof {
        bytes rlpEncodedProof;
        bytes rlpEncodedBlock;
        bytes[] proofs;
        bytes order;
    }

    // gas to prove stateRoot:
    // 20240828: ~100k gas
    // 20240829: ~275k gas (forgot receipt proof)
    function getStorageValues(
        bytes memory context,
        GatewayRequest memory req,
        bytes memory proof
    ) external view returns (bytes[] memory, uint8 exitCode) {
        uint256 headerBlock1 = abi.decode(context, (uint256));
        GatewayProof memory p = abi.decode(proof, (GatewayProof));
        RLPReader.RLPItem[] memory v = RLPReader.readList(p.rlpEncodedProof);
        uint256 headerBlock = uint256(RLPReaderExt.bytes32FromRLP(v[0]));
        (
            bytes32 rootHash,
            uint256 l2BlockNumberStart,
            ,
            uint256 createdAt,

        ) = _rootChain.headerBlocks(headerBlock);
        require(rootHash != bytes32(0), 'PolygonPoS: checkpoint');
        if (headerBlock1 != headerBlock) {
            (, , , uint256 createdAt1, ) = _rootChain.headerBlocks(
                headerBlock1
            );
            _checkWindow(createdAt1, createdAt);
        }
        bytes memory receipt = _proveReceiptInCheckpoint(
            v,
            rootHash,
            l2BlockNumberStart
        );
        bytes32 prevBlockHash = _extractPrevBlockHash(
            receipt,
            uint256(RLPReaderExt.bytes32FromRLP(v[9])) // logIndex
        );
        require(
            prevBlockHash == keccak256(p.rlpEncodedBlock),
            'PolygonPoS: blockHash'
        );
        v = RLPReader.readList(p.rlpEncodedBlock);
        bytes32 stateRoot = RLPReaderExt.strictBytes32FromRLP(v[3]);
        return
            GatewayVM.evalRequest(
                req,
                ProofSequence(0, stateRoot, p.proofs, p.order, _hooks)
            );
    }

    function _extractPrevBlockHash(
        bytes memory receipt,
        uint256 logIndex
    ) internal view returns (bytes32) {
        if (uint8(receipt[0]) != 0) {
            // remove transaction type prefix
            assembly {
                mstore(add(receipt, 1), sub(mload(receipt), 1))
                receipt := add(receipt, 1)
            }
        }
        RLPReader.RLPItem[] memory v = RLPReader.readList(receipt); // receipt
        v = RLPReader.readList(v[3]); // logs
        require(v.length > logIndex, 'PolygonPoS: logIndex');
        v = RLPReader.readList(v[logIndex]); // log
        address poster = address(
            uint160(uint256(RLPReaderExt.bytes32FromRLP(v[0])))
        );
        require(poster == _poster, 'PolygonPoS: poster');
        v = RLPReader.readList(v[1]); // topics
        return RLPReaderExt.strictBytes32FromRLP(v[1]); // prevBlockHash
    }

    function _proveReceiptInCheckpoint(
        RLPReader.RLPItem[] memory v,
        bytes32 rootHash,
        uint256 l2BlockNumberStart
    ) internal pure returns (bytes memory receipt) {
        uint256 l2BlockNumber = uint256(RLPReaderExt.bytes32FromRLP(v[2]));
        bytes32 receiptsRoot = RLPReaderExt.strictBytes32FromRLP(v[5]);
        bytes32 leafHash = keccak256(
            abi.encode(
                l2BlockNumber,
                RLPReaderExt.bytes32FromRLP(v[3]), // timestamp
                RLPReaderExt.strictBytes32FromRLP(v[4]), // transactionRoot
                receiptsRoot
            )
        );
        bytes32 computedRootHash = _computeRootHash(
            leafHash,
            RLPReader.readBytes(v[1]),
            l2BlockNumber - l2BlockNumberStart
        );
        require(rootHash == computedRootHash, 'PolygonPoS: rootHash');
        receipt = RLPReader.readBytes(v[6]);
        require(
            keccak256(receipt) ==
                _computeReceiptHash(
                    receiptsRoot,
                    RLPReader.readList(RLPReader.readBytes(v[7])), // branches
                    RLPReader.readBytes(v[8]) // path using hp-encoding
                ),
            'PolygonPos: receiptsRoot'
        );
    }

    function _computeRootHash(
        bytes32 leafHash,
        bytes memory proof,
        uint256 index
    ) internal pure returns (bytes32 ret) {
        ret = leafHash;
        for (uint256 i; i < proof.length; index >>= 1) {
            bytes32 next;
            assembly {
                i := add(i, 32)
                next := mload(add(proof, i))
            }
            if (index & 1 == 0) {
                ret = keccak256(abi.encodePacked(ret, next));
            } else {
                ret = keccak256(abi.encodePacked(next, ret));
            }
        }
    }

    function _computeReceiptHash(
        bytes32 root,
        RLPReader.RLPItem[] memory parentNodes,
        bytes memory path
    ) internal pure returns (bytes32 ret) {
        path = _nibblesFromHexPrefixed(path);
        bytes32 nodeKey = root;
        uint256 pathPtr;
        for (
            uint256 i = 0;
            i < parentNodes.length && pathPtr <= path.length;
            i++
        ) {
            if (nodeKey != RLPReaderExt.keccak256FromRawRLP(parentNodes[i]))
                break;
            RLPReader.RLPItem[] memory v = RLPReader.readList(parentNodes[i]);
            if (v.length == 17) {
                // branch
                if (pathPtr == path.length) {
                    ret = keccak256(RLPReader.readBytes(v[16]));
                    break;
                }
                uint8 next = uint8(path[pathPtr]);
                if (next > 16) break;
                nodeKey = RLPReaderExt.strictBytes32FromRLP(v[next]);
                pathPtr += 1;
            } else if (v.length == 2) {
                // extension/leaf
                bytes memory frag = _nibblesFromHexPrefixed(
                    RLPReader.readBytes(v[0])
                );
                uint256 shared;
                while (
                    shared < frag.length &&
                    path[pathPtr + shared] == frag[shared]
                ) shared++;
                if (pathPtr + shared == path.length) {
                    ret = keccak256(RLPReader.readBytes(v[1]));
                    break;
                }
                if (shared == 0) break; // extension
                pathPtr += shared;
                nodeKey = RLPReaderExt.strictBytes32FromRLP(v[1]);
            } else {
                break;
            }
        }
    }

    // https://ethereum.org/en/developers/docs/data-structures-and-encoding/patricia-merkle-trie/#specification
    function _nibblesFromHexPrefixed(
        bytes memory v
    ) internal pure returns (bytes memory nibbles) {
        if (v.length != 0) {
            uint256 start = _nibbleAt(v, 0) & 1 == 0 ? 2 : 1;
            nibbles = new bytes((v.length << 1) - start);
            for (uint256 i; i < nibbles.length; i++) {
                nibbles[i] = bytes1(_nibbleAt(v, start + i));
            }
        }
    }
    function _nibbleAt(bytes memory v, uint256 i) private pure returns (uint8) {
        uint8 b = uint8(v[i >> 1]);
        return i & 1 == 0 ? b >> 4 : b & 15;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {IGatewayVerifier} from './IGatewayVerifier.sol';
import {IVerifierHooks} from './IVerifierHooks.sol';

interface IStandardGatewayVerifier is IGatewayVerifier {
    function getHooks() external view returns (IVerifierHooks);
    function getWindow() external view returns (uint256);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import {AbstractVerifier, IVerifierHooks} from '../AbstractVerifier.sol';
import {GatewayRequest, GatewayVM, ProofSequence} from '../GatewayVM.sol';
import {Hashing, Types} from '../../lib/optimism/packages/contracts-bedrock/src/libraries/Hashing.sol';

interface IOPOutputFinder {
    function findOutputIndex(address portal, uint256 minAgeSec) external view returns (uint256);
    function getOutput(
        address portal,
        uint256 outputIndex
    ) external view returns (Types.OutputProposal memory);
}

contract OPVerifier is AbstractVerifier {
    address immutable _portal;
    IOPOutputFinder immutable _outputFinder;
    uint256 immutable _minAgeSec;

    constructor(
        string[] memory urls,
        uint256 window,
        IVerifierHooks hooks,
        address portal,
		IOPOutputFinder outputFinder,
        uint256 minAgeSec
    ) AbstractVerifier(urls, window, hooks) {
        _portal = portal;
		_outputFinder = outputFinder;
        _minAgeSec = minAgeSec;
    }

    function getLatestContext() external view returns (bytes memory) {
        return abi.encode(_outputFinder.findOutputIndex(_portal, _minAgeSec));
    }

    struct GatewayProof {
        uint256 outputIndex;
        Types.OutputRootProof outputRootProof;
        bytes[] proofs;
        bytes order;
    }

    function getStorageValues(
        bytes memory context,
        GatewayRequest memory req,
        bytes memory proof
    ) external view returns (bytes[] memory, uint8 exitCode) {
        uint256 outputIndex1 = abi.decode(context, (uint256));
        GatewayProof memory p = abi.decode(proof, (GatewayProof));
        Types.OutputProposal memory output = _outputFinder.getOutput(
            _portal, 
            p.outputIndex
        );
        if (p.outputIndex != outputIndex1) {
            Types.OutputProposal memory output1 = _outputFinder.getOutput(
                _portal, outputIndex1
            );
            _checkWindow(output1.timestamp, output.timestamp);
            // NOTE: no addtional checks are required
            // newer outputs will fail window check
            // older outputs will be older (by definition)
            // therefore, older finalized outputs are also finalized 
        }
        bytes32 computedRoot = Hashing.hashOutputRootProof(p.outputRootProof);
        require(computedRoot == output.outputRoot, 'OP: invalid root');
        return
            GatewayVM.evalRequest(
                req,
                ProofSequence(
                    0,
                    p.outputRootProof.stateRoot,
                    p.proofs,
                    p.order,
                    _hooks
                )
            );
    }
}

//SPDX-License-Identifier: MIT
pragma solidity ~0.8.17;

import "./StablePriceOracle.sol";

contract ExponentialPremiumPriceOracle is StablePriceOracle {
    uint256 constant GRACE_PERIOD = 90 days;
    uint256 immutable startPremium;
    uint256 immutable endValue;

    constructor(
        AggregatorInterface _usdOracle,
        uint256[] memory _rentPrices,
        uint256 _startPremium,
        uint256 totalDays
    ) StablePriceOracle(_usdOracle, _rentPrices) {
        startPremium = _startPremium;
        endValue = _startPremium >> totalDays;
    }

    uint256 constant PRECISION = 1e18;
    uint256 constant bit1 = 999989423469314432; // 0.5 ^ 1/65536 * (10 ** 18)
    uint256 constant bit2 = 999978847050491904; // 0.5 ^ 2/65536 * (10 ** 18)
    uint256 constant bit3 = 999957694548431104;
    uint256 constant bit4 = 999915390886613504;
    uint256 constant bit5 = 999830788931929088;
    uint256 constant bit6 = 999661606496243712;
    uint256 constant bit7 = 999323327502650752;
    uint256 constant bit8 = 998647112890970240;
    uint256 constant bit9 = 997296056085470080;
    uint256 constant bit10 = 994599423483633152;
    uint256 constant bit11 = 989228013193975424;
    uint256 constant bit12 = 978572062087700096;
    uint256 constant bit13 = 957603280698573696;
    uint256 constant bit14 = 917004043204671232;
    uint256 constant bit15 = 840896415253714560;
    uint256 constant bit16 = 707106781186547584;

    /// @dev Returns the pricing premium in internal base units.
    function _premium(
        string memory,
        uint256 expires,
        uint256
    ) internal view override returns (uint256) {
        expires = expires + GRACE_PERIOD;
        if (expires > block.timestamp) {
            return 0;
        }

        uint256 elapsed = block.timestamp - expires;
        uint256 premium = decayedPremium(startPremium, elapsed);
        if (premium >= endValue) {
            return premium - endValue;
        }
        return 0;
    }

    /// @dev Returns the premium price at current time elapsed
    /// @param startPremium starting price
    /// @param elapsed time past since expiry
    function decayedPremium(
        uint256 startPremium,
        uint256 elapsed
    ) public pure returns (uint256) {
        uint256 daysPast = (elapsed * PRECISION) / 1 days;
        uint256 intDays = daysPast / PRECISION;
        uint256 premium = startPremium >> intDays;
        uint256 partDay = (daysPast - intDays * PRECISION);
        uint256 fraction = (partDay * (2 ** 16)) / PRECISION;
        uint256 totalPremium = addFractionalPremium(fraction, premium);
        return totalPremium;
    }

    function addFractionalPremium(
        uint256 fraction,
        uint256 premium
    ) internal pure returns (uint256) {
        if (fraction & (1 << 0) != 0) {
            premium = (premium * bit1) / PRECISION;
        }
        if (fraction & (1 << 1) != 0) {
            premium = (premium * bit2) / PRECISION;
        }
        if (fraction & (1 << 2) != 0) {
            premium = (premium * bit3) / PRECISION;
        }
        if (fraction & (1 << 3) != 0) {
            premium = (premium * bit4) / PRECISION;
        }
        if (fraction & (1 << 4) != 0) {
            premium = (premium * bit5) / PRECISION;
        }
        if (fraction & (1 << 5) != 0) {
            premium = (premium * bit6) / PRECISION;
        }
        if (fraction & (1 << 6) != 0) {
            premium = (premium * bit7) / PRECISION;
        }
        if (fraction & (1 << 7) != 0) {
            premium = (premium * bit8) / PRECISION;
        }
        if (fraction & (1 << 8) != 0) {
            premium = (premium * bit9) / PRECISION;
        }
        if (fraction & (1 << 9) != 0) {
            premium = (premium * bit10) / PRECISION;
        }
        if (fraction & (1 << 10) != 0) {
            premium = (premium * bit11) / PRECISION;
        }
        if (fraction & (1 << 11) != 0) {
            premium = (premium * bit12) / PRECISION;
        }
        if (fraction & (1 << 12) != 0) {
            premium = (premium * bit13) / PRECISION;
        }
        if (fraction & (1 << 13) != 0) {
            premium = (premium * bit14) / PRECISION;
        }
        if (fraction & (1 << 14) != 0) {
            premium = (premium * bit15) / PRECISION;
        }
        if (fraction & (1 << 15) != 0) {
            premium = (premium * bit16) / PRECISION;
        }
        return premium;
    }

    function supportsInterface(
        bytes4 interfaceID
    ) public view virtual override returns (bool) {
        return super.supportsInterface(interfaceID);
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


## ------------ SUPPORTING CONTEXT: EXTERNAL LIBRARIES ------------ 

 ------------ END OF SUPPORTING CONTRACTS AND INTERFACES ------------ 


 ------------ ## DEPLOYMENT SCRIPTS ------------ 

