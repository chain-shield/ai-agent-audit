
 ------------ ## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity ^0.7.6;
pragma abicoder v2;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable gas-indexed-events, gas-small-strings, gas-strict-inequalities

import { SafeMathUpgradeable } from "@openzeppelin/contracts-upgradeable/math/SafeMathUpgradeable.sol";

import { GNS } from "../../discovery/GNS.sol";
import { ICuration } from "@graphprotocol/interfaces/contracts/contracts/curation/ICuration.sol";
import { IL2GNS } from "@graphprotocol/interfaces/contracts/contracts/l2/discovery/IL2GNS.sol";
import { L2GNSV1Storage } from "./L2GNSStorage.sol";

import { IL2Curation } from "@graphprotocol/interfaces/contracts/contracts/l2/curation/IL2Curation.sol";

/**
 * @title L2GNS
 * @author Edge & Node
 * @notice The Graph Name System contract provides a decentralized naming system for subgraphs
 * used in the scope of the Graph Network. It translates Subgraphs into Subgraph Versions.
 * Each version is associated with a Subgraph Deployment. The contract has no knowledge of
 * human-readable names. All human readable names emitted in events.
 * The contract implements a multicall behaviour to support batching multiple calls in a single
 * transaction.
 * This particular contract is meant to be deployed in L2, and includes helper functions to
 * receive subgraphs that are transferred from L1.
 */
contract L2GNS is GNS, L2GNSV1Storage, IL2GNS {
    using SafeMathUpgradeable for uint256;

    /// @notice Offset added to an L1 subgraph ID to compute the L2 subgraph ID alias
    uint256 public constant SUBGRAPH_ID_ALIAS_OFFSET =
        uint256(0x1111000000000000000000000000000000000000000000000000000000001111);

    /// @notice Maximum rounding error when receiving signal tokens from L1, in parts-per-million
    /// @dev If the error from minting signal is above this, tokens will be sent back to the curator
    uint256 public constant MAX_ROUNDING_ERROR = 1000;

    /// @dev 100% expressed in parts-per-million
    uint256 private constant MAX_PPM = 1000000;

    /// @notice Emitted when a subgraph is received from L1 through the bridge
    /// @param _l1SubgraphID Subgraph ID on L1
    /// @param _l2SubgraphID Subgraph ID on L2 (aliased)
    /// @param _owner Address of the subgraph owner
    /// @param _tokens Amount of tokens transferred with the subgraph
    event SubgraphReceivedFromL1(
        uint256 indexed _l1SubgraphID,
        uint256 indexed _l2SubgraphID,
        address indexed _owner,
        uint256 _tokens
    );
    /// @notice Emitted when a subgraph transfer from L1 is finalized, so the subgraph is published on L2
    /// @param _l2SubgraphID Subgraph ID on L2
    event SubgraphL2TransferFinalized(uint256 indexed _l2SubgraphID);
    /// @notice Emitted when the L1 balance for a curator has been claimed
    /// @param _l1SubgraphId Subgraph ID on L1
    /// @param _l2SubgraphID Subgraph ID on L2 (aliased)
    /// @param _l2Curator Address of the curator on L2
    /// @param _tokens Amount of tokens received
    event CuratorBalanceReceived(
        uint256 indexed _l1SubgraphId,
        uint256 indexed _l2SubgraphID,
        address indexed _l2Curator,
        uint256 _tokens
    );
    /// @notice Emitted when the L1 balance for a curator has been returned to the beneficiary.
    /// This can happen if the subgraph transfer was not finished when the curator's tokens arrived.
    /// @param _l1SubgraphID Subgraph ID on L1
    /// @param _l2Curator Address of the curator on L2
    /// @param _tokens Amount of tokens returned
    event CuratorBalanceReturnedToBeneficiary(
        uint256 indexed _l1SubgraphID,
        address indexed _l2Curator,
        uint256 _tokens
    );

    /**
     * @dev Checks that the sender is the L2GraphTokenGateway as configured on the Controller.
     */
    modifier onlyL2Gateway() {
        require(msg.sender == address(graphTokenGateway()), "ONLY_GATEWAY");
        _;
    }

    /**
     * @notice Receive tokens with a callhook from the bridge.
     * The callhook will receive a subgraph or a curator's balance from L1. The _data parameter
     * must contain the ABI encoding of:
     * (uint8 code, uint256 subgraphId, address beneficiary)
     * Where `code` is one of the codes defined in IL2GNS.L1MessageCodes.
     * If the code is RECEIVE_SUBGRAPH_CODE, the beneficiary is the address of the
     * owner of the subgraph on L2.
     * If the code is RECEIVE_CURATOR_BALANCE_CODE, then the beneficiary is the
     * address of the curator in L2. In this case, If the subgraph transfer was never finished
     * (or the subgraph doesn't exist), the tokens will be sent to the curator.
     * @dev This function is called by the L2GraphTokenGateway contract.
     * @param _from Token sender in L1 (must be the L1GNS)
     * @param _amount Amount of tokens that were transferred
     * @param _data ABI-encoded callhook data
     */
    function onTokenTransfer(
        address _from,
        uint256 _amount,
        bytes calldata _data
    ) external override notPartialPaused onlyL2Gateway {
        require(_from == counterpartGNSAddress, "ONLY_L1_GNS_THROUGH_BRIDGE");
        (uint8 code, uint256 l1SubgraphID, address beneficiary) = abi.decode(_data, (uint8, uint256, address));

        if (code == uint8(L1MessageCodes.RECEIVE_SUBGRAPH_CODE)) {
            _receiveSubgraphFromL1(l1SubgraphID, beneficiary, _amount);
        } else if (code == uint8(L1MessageCodes.RECEIVE_CURATOR_BALANCE_CODE)) {
            _mintSignalFromL1(l1SubgraphID, beneficiary, _amount);
        } else {
            revert("INVALID_CODE");
        }
    }

    /**
     * @inheritdoc IL2GNS
     */
    function finishSubgraphTransferFromL1(
        uint256 _l2SubgraphID,
        bytes32 _subgraphDeploymentID,
        bytes32 _subgraphMetadata,
        bytes32 _versionMetadata
    ) external override notPartialPaused onlySubgraphAuth(_l2SubgraphID) {
        IL2GNS.SubgraphL2TransferData storage transferData = subgraphL2TransferData[_l2SubgraphID];
        SubgraphData storage subgraphData = _getSubgraphData(_l2SubgraphID);
        require(transferData.subgraphReceivedOnL2BlockNumber != 0, "INVALID_SUBGRAPH");
        require(!transferData.l2Done, "ALREADY_DONE");
        transferData.l2Done = true;

        // New subgraph deployment must be non-empty
        require(_subgraphDeploymentID != 0, "GNS: deploymentID != 0");

        IL2Curation curation = IL2Curation(address(curation()));

        uint256 vSignal;
        uint256 nSignal;
        uint256 roundingError;
        uint256 tokens = transferData.tokens;
        {
            // This can't revert because the bridge ensures that _tokensIn is > 0,
            // and the minimum curation in L2 is 1 wei GRT
            uint256 tokensAfter = curation.tokensToSignalToTokensNoTax(_subgraphDeploymentID, tokens);
            roundingError = tokens.sub(tokensAfter).mul(MAX_PPM).div(tokens);
        }
        if (roundingError <= MAX_ROUNDING_ERROR) {
            vSignal = curation.mintTaxFree(_subgraphDeploymentID, tokens);
            nSignal = vSignalToNSignal(_l2SubgraphID, vSignal);
            emit SignalMinted(_l2SubgraphID, msg.sender, nSignal, vSignal, tokens);
            emit SubgraphUpgraded(_l2SubgraphID, vSignal, tokens, _subgraphDeploymentID);
        } else {
            graphToken().transfer(msg.sender, tokens);
            emit CuratorBalanceReturnedToBeneficiary(getUnaliasedL1SubgraphID(_l2SubgraphID), msg.sender, tokens);
            emit SubgraphUpgraded(_l2SubgraphID, vSignal, 0, _subgraphDeploymentID);
        }

        subgraphData.disabled = false;
        subgraphData.vSignal = vSignal;
        subgraphData.nSignal = nSignal;
        subgraphData.curatorNSignal[msg.sender] = nSignal;
        subgraphData.subgraphDeploymentID = _subgraphDeploymentID;
        // Set the token metadata
        _setSubgraphMetadata(_l2SubgraphID, _subgraphMetadata);
        emit SubgraphPublished(_l2SubgraphID, _subgraphDeploymentID, fixedReserveRatio);
        emit SubgraphVersionUpdated(_l2SubgraphID, _subgraphDeploymentID, _versionMetadata);
        emit SubgraphL2TransferFinalized(_l2SubgraphID);
    }

    /**
     * @notice Publish a new version of an existing subgraph.
     * @dev This is the same as the one in the base GNS, but skips the check for
     * a subgraph to not be pre-curated, as the reserve ratio in L2 is set to 1,
     * which prevents the risk of rug-pulling.
     * @param _subgraphID Subgraph ID
     * @param _subgraphDeploymentID Subgraph deployment ID of the new version
     * @param _versionMetadata IPFS hash for the subgraph version metadata
     */
    function publishNewVersion(
        uint256 _subgraphID,
        bytes32 _subgraphDeploymentID,
        bytes32 _versionMetadata
    ) external override notPaused onlySubgraphAuth(_subgraphID) {
        // Perform the upgrade from the current subgraph deployment to the new one.
        // This involves burning all signal from the old deployment and using the funds to buy
        // from the new deployment.
        // This will also make the change to target to the new deployment.

        // Subgraph check
        SubgraphData storage subgraphData = _getSubgraphOrRevert(_subgraphID);

        // New subgraph deployment must be non-empty
        require(_subgraphDeploymentID != 0, "GNS: Cannot set deploymentID to 0 in publish");

        // New subgraph deployment must be different than current
        require(
            _subgraphDeploymentID != subgraphData.subgraphDeploymentID,
            "GNS: Cannot publish a new version with the same subgraph deployment ID"
        );

        ICuration curation = curation();

        // Move all signal from previous version to new version
        // NOTE: We will only do this as long as there is signal on the subgraph
        if (subgraphData.nSignal != 0) {
            // Burn all version signal in the name pool for tokens (w/no slippage protection)
            // Sell all signal from the old deployment
            uint256 tokens = curation.burn(subgraphData.subgraphDeploymentID, subgraphData.vSignal, 0);

            // Take the owner cut of the curation tax, add it to the total
            // Upgrade is only callable by the owner, we assume then that msg.sender = owner
            address subgraphOwner = msg.sender;
            uint256 tokensWithTax = _chargeOwnerTax(tokens, subgraphOwner, curation.curationTaxPercentage());

            // Update pool: constant nSignal, vSignal can change (w/no slippage protection)
            // Buy all signal from the new deployment
            (subgraphData.vSignal, ) = curation.mint(_subgraphDeploymentID, tokensWithTax, 0);

            emit SubgraphUpgraded(_subgraphID, subgraphData.vSignal, tokensWithTax, _subgraphDeploymentID);
        }

        // Update target deployment
        subgraphData.subgraphDeploymentID = _subgraphDeploymentID;

        emit SubgraphVersionUpdated(_subgraphID, _subgraphDeploymentID, _versionMetadata);
    }

    /**
     * @inheritdoc IL2GNS
     */
    function getAliasedL2SubgraphID(uint256 _l1SubgraphID) public pure override returns (uint256) {
        return _l1SubgraphID + SUBGRAPH_ID_ALIAS_OFFSET;
    }

    /**
     * @inheritdoc IL2GNS
     */
    function getUnaliasedL1SubgraphID(uint256 _l2SubgraphID) public pure override returns (uint256) {
        return _l2SubgraphID - SUBGRAPH_ID_ALIAS_OFFSET;
    }

    /**
     * @notice Receive a subgraph from L1.
     * This function will initialize a subgraph received through the bridge,
     * and store the transfer data so that it's finalized later using finishSubgraphTransferFromL1.
     * @param _l1SubgraphID Subgraph ID in L1 (will be aliased)
     * @param _subgraphOwner Owner of the subgraph
     * @param _tokens Tokens to be deposited in the subgraph
     */
    function _receiveSubgraphFromL1(uint256 _l1SubgraphID, address _subgraphOwner, uint256 _tokens) internal {
        uint256 l2SubgraphID = getAliasedL2SubgraphID(_l1SubgraphID);
        SubgraphData storage subgraphData = _getSubgraphData(l2SubgraphID);
        IL2GNS.SubgraphL2TransferData storage transferData = subgraphL2TransferData[l2SubgraphID];

        subgraphData.__DEPRECATED_reserveRatio = fixedReserveRatio;
        // The subgraph will be disabled until finishSubgraphTransferFromL1 is called
        subgraphData.disabled = true;

        transferData.tokens = _tokens;
        transferData.subgraphReceivedOnL2BlockNumber = block.number;

        // Mint the NFT. Use the subgraphID as tokenID.
        // This function will check the if tokenID already exists.
        // Note we do this here so that we can later do the onlySubgraphAuth
        // check in finishSubgraphTransferFromL1.
        _mintNFT(_subgraphOwner, l2SubgraphID);

        emit SubgraphReceivedFromL1(_l1SubgraphID, l2SubgraphID, _subgraphOwner, _tokens);
    }

    /**
     * @notice Deposit GRT into a subgraph and mint signal, using tokens received from L1.
     * If the subgraph transfer was never finished (or the subgraph doesn't exist), the tokens will be sent to the curator.
     * @dev This looks a lot like GNS.mintSignal, but doesn't pull the tokens from the
     * curator and has no slippage protection.
     * @param _l1SubgraphID Subgraph ID in L1 (will be aliased)
     * @param _curator Curator address
     * @param _tokensIn The amount of tokens the nameCurator wants to deposit
     */
    function _mintSignalFromL1(uint256 _l1SubgraphID, address _curator, uint256 _tokensIn) internal {
        uint256 l2SubgraphID = getAliasedL2SubgraphID(_l1SubgraphID);
        IL2GNS.SubgraphL2TransferData storage transferData = subgraphL2TransferData[l2SubgraphID];
        SubgraphData storage subgraphData = _getSubgraphData(l2SubgraphID);

        IL2Curation curation = IL2Curation(address(curation()));
        uint256 roundingError;
        if (transferData.l2Done && !subgraphData.disabled) {
            // This can't revert because the bridge ensures that _tokensIn is > 0,
            // and the minimum curation in L2 is 1 wei GRT
            uint256 tokensAfter = curation.tokensToSignalToTokensNoTax(subgraphData.subgraphDeploymentID, _tokensIn);
            roundingError = _tokensIn.sub(tokensAfter).mul(MAX_PPM).div(_tokensIn);
        }
        // If subgraph transfer wasn't finished, we should send the tokens to the curator
        if (!transferData.l2Done || subgraphData.disabled || roundingError > MAX_ROUNDING_ERROR) {
            graphToken().transfer(_curator, _tokensIn);
            emit CuratorBalanceReturnedToBeneficiary(_l1SubgraphID, _curator, _tokensIn);
        } else {
            // Get name signal to mint for tokens deposited
            uint256 vSignal = curation.mintTaxFree(subgraphData.subgraphDeploymentID, _tokensIn);
            uint256 nSignal = vSignalToNSignal(l2SubgraphID, vSignal);

            // Update pools
            subgraphData.vSignal = subgraphData.vSignal.add(vSignal);
            subgraphData.nSignal = subgraphData.nSignal.add(nSignal);
            subgraphData.curatorNSignal[_curator] = subgraphData.curatorNSignal[_curator].add(nSignal);

            emit SignalMinted(l2SubgraphID, _curator, nSignal, vSignal, _tokensIn);
            emit CuratorBalanceReceived(_l1SubgraphID, l2SubgraphID, _curator, _tokensIn);
        }
    }

    /**
     * @notice Get subgraph data
     * @dev Since there are no legacy subgraphs in L2, we override the base
     * GNS method to save us the step of checking for legacy subgraphs
     * @param _subgraphID Subgraph ID
     * @return Subgraph Data
     */
    function _getSubgraphData(uint256 _subgraphID) internal view override returns (SubgraphData storage) {
        // Return new subgraph type
        return subgraphs[_subgraphID];
    }
}
 ------------
 ------------ END OF MAIN TARGET CONTRACT ------------ 

 ------------ ## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES ------------ 
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

// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity ^0.7.6;
pragma abicoder v2;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable function-max-lines, gas-indexed-events, gas-small-strings, gas-strict-inequalities

import { SafeMathUpgradeable } from "@openzeppelin/contracts-upgradeable/math/SafeMathUpgradeable.sol";
import { AddressUpgradeable } from "@openzeppelin/contracts-upgradeable/utils/AddressUpgradeable.sol";

import { Multicall } from "../base/Multicall.sol";
import { GraphUpgradeable } from "../upgrades/GraphUpgradeable.sol";
import { TokenUtils } from "../utils/TokenUtils.sol";
import { ICuration } from "@graphprotocol/interfaces/contracts/contracts/curation/ICuration.sol";
import { Managed } from "../governance/Managed.sol";
import { ISubgraphNFT } from "@graphprotocol/interfaces/contracts/contracts/discovery/ISubgraphNFT.sol";

import { IGNS } from "@graphprotocol/interfaces/contracts/contracts/discovery/IGNS.sol";
import { GNSV3Storage } from "./GNSStorage.sol";

/**
 * @title GNS
 * @author Edge & Node
 * @notice The Graph Name System contract provides a decentralized naming system for subgraphs
 * used in the scope of the Graph Network. It translates Subgraphs into Subgraph Versions.
 * Each version is associated with a Subgraph Deployment. The contract has no knowledge of
 * human-readable names. All human readable names emitted in events.
 * The contract implements a multicall behaviour to support batching multiple calls in a single
 * transaction.
 */
abstract contract GNS is GNSV3Storage, GraphUpgradeable, IGNS, Multicall {
    using SafeMathUpgradeable for uint256;

    // -- Constants --

    /// @dev 100% in parts per million
    uint32 private constant MAX_PPM = 1000000;

    /// @dev Equates to Connector weight on bancor formula to be CW = 1
    // solhint-disable-next-line immutable-vars-naming
    uint32 internal immutable fixedReserveRatio = MAX_PPM;

    // -- Events --

    /// @notice Emitted when the subgraph NFT contract is updated
    /// @param subgraphNFT Address of the new subgraph NFT contract
    event SubgraphNFTUpdated(address subgraphNFT);

    /**
     * @notice Emitted when graph account sets its default name
     * @param graphAccount Address of the graph account
     * @param nameSystem Name system identifier (only ENS for now)
     * @param nameIdentifier Name identifier in the name system
     * @param name Human-readable name
     */
    event SetDefaultName(
        address indexed graphAccount,
        uint256 nameSystem, // only ENS for now
        bytes32 nameIdentifier,
        string name
    );

    /**
     * @notice Emitted when the subgraph metadata is updated.
     * @param subgraphID ID of the subgraph
     * @param subgraphMetadata IPFS hash of the subgraph metadata
     */
    event SubgraphMetadataUpdated(uint256 indexed subgraphID, bytes32 subgraphMetadata);

    /**
     * @notice Emitted when a subgraph version is updated.
     * @param subgraphID ID of the subgraph
     * @param subgraphDeploymentID Subgraph deployment ID for the new version
     * @param versionMetadata IPFS hash of the version metadata
     */
    event SubgraphVersionUpdated(
        uint256 indexed subgraphID,
        bytes32 indexed subgraphDeploymentID,
        bytes32 versionMetadata
    );

    /**
     * @notice Emitted when a curator mints signal.
     * @param subgraphID ID of the subgraph
     * @param curator Address of the curator
     * @param nSignalCreated Amount of name signal created
     * @param vSignalCreated Amount of version signal created
     * @param tokensDeposited Amount of tokens deposited
     */
    event SignalMinted(
        uint256 indexed subgraphID,
        address indexed curator,
        uint256 nSignalCreated,
        uint256 vSignalCreated,
        uint256 tokensDeposited
    );

    /**
     * @notice Emitted when a curator burns signal.
     * @param subgraphID ID of the subgraph
     * @param curator Address of the curator
     * @param nSignalBurnt Amount of name signal burned
     * @param vSignalBurnt Amount of version signal burned
     * @param tokensReceived Amount of tokens received
     */
    event SignalBurned(
        uint256 indexed subgraphID,
        address indexed curator,
        uint256 nSignalBurnt,
        uint256 vSignalBurnt,
        uint256 tokensReceived
    );

    /**
     * @notice Emitted when a curator transfers signal.
     * @param subgraphID ID of the subgraph
     * @param from Address transferring the signal
     * @param to Address receiving the signal
     * @param nSignalTransferred Amount of name signal transferred
     */
    event SignalTransferred(
        uint256 indexed subgraphID,
        address indexed from,
        address indexed to,
        uint256 nSignalTransferred
    );

    /**
     * @notice Emitted when a subgraph is created.
     * @param subgraphID ID of the subgraph
     * @param subgraphDeploymentID Subgraph deployment ID
     * @param reserveRatio Reserve ratio for the bonding curve
     */
    event SubgraphPublished(uint256 indexed subgraphID, bytes32 indexed subgraphDeploymentID, uint32 reserveRatio);

    /**
     * @notice Emitted when a subgraph is upgraded to point to a new
     * subgraph deployment, burning all the old vSignal and depositing the GRT into the
     * new vSignal curve.
     * @param subgraphID ID of the subgraph
     * @param vSignalCreated Amount of version signal created in the new deployment
     * @param tokensSignalled Amount of tokens signalled in the new deployment
     * @param subgraphDeploymentID New subgraph deployment ID
     */
    event SubgraphUpgraded(
        uint256 indexed subgraphID,
        uint256 vSignalCreated,
        uint256 tokensSignalled,
        bytes32 indexed subgraphDeploymentID
    );

    /**
     * @notice Emitted when a subgraph is deprecated.
     * @param subgraphID ID of the subgraph
     * @param withdrawableGRT Amount of GRT available for withdrawal
     */
    event SubgraphDeprecated(uint256 indexed subgraphID, uint256 withdrawableGRT);

    /**
     * @notice Emitted when a curator withdraws GRT from a deprecated subgraph
     * @param subgraphID ID of the subgraph
     * @param curator Address of the curator
     * @param nSignalBurnt Amount of name signal burned
     * @param withdrawnGRT Amount of GRT withdrawn
     */
    event GRTWithdrawn(uint256 indexed subgraphID, address indexed curator, uint256 nSignalBurnt, uint256 withdrawnGRT);

    /**
     * @notice Emitted when the counterpart (L1/L2) GNS address is updated
     * @param _counterpart Address of the counterpart GNS contract
     */
    event CounterpartGNSAddressUpdated(address _counterpart);

    // -- Modifiers --

    /**
     * @notice Emitted when a legacy subgraph is claimed
     * @param graphAccount Address of the graph account that created the subgraph
     * @param subgraphNumber Sequence number of the subgraph
     */
    event LegacySubgraphClaimed(address indexed graphAccount, uint256 subgraphNumber);

    /**
     * @notice Modifier that allows only a subgraph operator to be the caller
     * @param _subgraphID ID of the subgraph to check authorization for
     */
    modifier onlySubgraphAuth(uint256 _subgraphID) {
        require(ownerOf(_subgraphID) == msg.sender, "GNS: Must be authorized");
        _;
    }

    // -- Functions --

    /**
     * @notice Initialize the GNS contract.
     * @param _controller Address of the Controller contract that manages this contract
     * @param _subgraphNFT Address of the Subgraph NFT contract
     */
    function initialize(address _controller, address _subgraphNFT) external onlyImpl initializer {
        Managed._initialize(_controller);

        // Settings
        _setOwnerTaxPercentage(500000);
        _setSubgraphNFT(_subgraphNFT);
    }

    /**
     * @inheritdoc IGNS
     */
    function approveAll() external override {
        graphToken().approve(address(curation()), type(uint256).max);
    }

    // -- Config --

    /**
     * @inheritdoc IGNS
     */
    function setOwnerTaxPercentage(uint32 _ownerTaxPercentage) external override onlyGovernor {
        _setOwnerTaxPercentage(_ownerTaxPercentage);
    }

    /**
     * @notice Set the NFT registry contract
     * NOTE: Calling this function will break the ownership model unless
     * it is replaced with a fully migrated version of the NFT contract state
     * Use with care.
     * @param _subgraphNFT Address of the ERC721 contract
     */
    function setSubgraphNFT(address _subgraphNFT) external onlyGovernor {
        _setSubgraphNFT(_subgraphNFT);
    }

    /**
     * @notice Set the counterpart (L1/L2) GNS address
     * @param _counterpart Owner tax percentage
     */
    function setCounterpartGNSAddress(address _counterpart) external onlyGovernor {
        counterpartGNSAddress = _counterpart;
        emit CounterpartGNSAddressUpdated(_counterpart);
    }

    // -- Actions --

    /**
     * @inheritdoc IGNS
     */
    function setDefaultName(
        address _graphAccount,
        uint8 _nameSystem,
        bytes32 _nameIdentifier,
        string calldata _name
    ) external override {
        require(_graphAccount == msg.sender, "GNS: Only you can set your name");
        emit SetDefaultName(_graphAccount, _nameSystem, _nameIdentifier, _name);
    }

    /**
     * @inheritdoc IGNS
     */
    function updateSubgraphMetadata(
        uint256 _subgraphID,
        bytes32 _subgraphMetadata
    ) external override onlySubgraphAuth(_subgraphID) {
        _setSubgraphMetadata(_subgraphID, _subgraphMetadata);
    }

    /**
     * @inheritdoc IGNS
     */
    function publishNewSubgraph(
        bytes32 _subgraphDeploymentID,
        bytes32 _versionMetadata,
        bytes32 _subgraphMetadata
    ) external override notPaused {
        // Subgraph deployment must be non-empty
        require(_subgraphDeploymentID != 0, "GNS: Cannot set deploymentID to 0 in publish");

        // Init the subgraph
        address subgraphOwner = msg.sender;
        uint256 subgraphID = _nextSubgraphID(subgraphOwner);
        SubgraphData storage subgraphData = _getSubgraphData(subgraphID);
        subgraphData.subgraphDeploymentID = _subgraphDeploymentID;
        subgraphData.__DEPRECATED_reserveRatio = fixedReserveRatio;

        // Mint the NFT. Use the subgraphID as tokenID.
        // This function will check the if tokenID already exists.
        _mintNFT(subgraphOwner, subgraphID);
        emit SubgraphPublished(subgraphID, _subgraphDeploymentID, fixedReserveRatio);

        // Set the token metadata
        _setSubgraphMetadata(subgraphID, _subgraphMetadata);

        emit SubgraphVersionUpdated(subgraphID, _subgraphDeploymentID, _versionMetadata);
    }

    /**
     * @inheritdoc IGNS
     */
    function publishNewVersion(
        uint256 _subgraphID,
        bytes32 _subgraphDeploymentID,
        bytes32 _versionMetadata
    ) external virtual override notPaused onlySubgraphAuth(_subgraphID) {
        // Perform the upgrade from the current subgraph deployment to the new one.
        // This involves burning all signal from the old deployment and using the funds to buy
        // from the new deployment.
        // This will also make the change to target to the new deployment.

        // Subgraph check
        SubgraphData storage subgraphData = _getSubgraphOrRevert(_subgraphID);

        // New subgraph deployment must be non-empty
        require(_subgraphDeploymentID != 0, "GNS: Cannot set deploymentID to 0 in publish");

        // New subgraph deployment must be different than current
        require(
            _subgraphDeploymentID != subgraphData.subgraphDeploymentID,
            "GNS: Cannot publish a new version with the same subgraph deployment ID"
        );

        // This is to prevent the owner from front running its name curators signal by posting
        // its own signal ahead, bringing the name curators in, and dumping on them
        ICuration curation = curation();
        require(
            !curation.isCurated(_subgraphDeploymentID),
            "GNS: Owner cannot point to a subgraphID that has been pre-curated"
        );

        // Move all signal from previous version to new version
        // NOTE: We will only do this as long as there is signal on the subgraph
        if (subgraphData.nSignal != 0) {
            // Burn all version signal in the name pool for tokens (w/no slippage protection)
            // Sell all signal from the old deployment
            uint256 tokens = curation.burn(subgraphData.subgraphDeploymentID, subgraphData.vSignal, 0);

            // Take the owner cut of the curation tax, add it to the total
            // Upgrade is only callable by the owner, we assume then that msg.sender = owner
            address subgraphOwner = msg.sender;
            uint256 tokensWithTax = _chargeOwnerTax(tokens, subgraphOwner, curation.curationTaxPercentage());

            // Update pool: constant nSignal, vSignal can change (w/no slippage protection)
            // Buy all signal from the new deployment
            (subgraphData.vSignal, ) = curation.mint(_subgraphDeploymentID, tokensWithTax, 0);

            emit SubgraphUpgraded(_subgraphID, subgraphData.vSignal, tokensWithTax, _subgraphDeploymentID);
        }

        // Update target deployment
        subgraphData.subgraphDeploymentID = _subgraphDeploymentID;

        emit SubgraphVersionUpdated(_subgraphID, _subgraphDeploymentID, _versionMetadata);
    }

    /**
     * @inheritdoc IGNS
     * @notice The bonding curve is destroyed, the vSignal is burned, and the GNS
     * contract holds the GRT from burning the vSignal, which all curators can withdraw manually.
     * Can only be done by the subgraph owner.
     */
    function deprecateSubgraph(uint256 _subgraphID) external override notPaused onlySubgraphAuth(_subgraphID) {
        // Subgraph check
        SubgraphData storage subgraphData = _getSubgraphOrRevert(_subgraphID);

        // Burn signal only if it has any available
        if (subgraphData.nSignal != 0) {
            subgraphData.withdrawableGRT = curation().burn(subgraphData.subgraphDeploymentID, subgraphData.vSignal, 0);
        }

        // Deprecate the subgraph and do cleanup
        subgraphData.disabled = true;
        subgraphData.vSignal = 0;
        subgraphData.__DEPRECATED_reserveRatio = 0;
        // NOTE: We don't reset the following variable as we use it to test if the Subgraph was ever created
        // subgraphData.subgraphDeploymentID = 0;

        // Burn the NFT
        _burnNFT(_subgraphID);

        emit SubgraphDeprecated(_subgraphID, subgraphData.withdrawableGRT);
    }

    /**
     * @inheritdoc IGNS
     */
    function mintSignal(
        uint256 _subgraphID,
        uint256 _tokensIn,
        uint256 _nSignalOutMin
    ) external override notPartialPaused {
        // Subgraph checks
        SubgraphData storage subgraphData = _getSubgraphOrRevert(_subgraphID);

        // Pull tokens from sender
        address curator = msg.sender;
        TokenUtils.pullTokens(graphToken(), curator, _tokensIn);

        // Get name signal to mint for tokens deposited
        (uint256 vSignal, ) = curation().mint(subgraphData.subgraphDeploymentID, _tokensIn, 0);
        uint256 nSignal = vSignalToNSignal(_subgraphID, vSignal);

        // Slippage protection
        require(nSignal >= _nSignalOutMin, "GNS: Slippage protection");

        // Update pools
        subgraphData.vSignal = subgraphData.vSignal.add(vSignal);
        subgraphData.nSignal = subgraphData.nSignal.add(nSignal);
        subgraphData.curatorNSignal[curator] = subgraphData.curatorNSignal[curator].add(nSignal);

        emit SignalMinted(_subgraphID, curator, nSignal, vSignal, _tokensIn);
    }

    /**
     * @inheritdoc IGNS
     */
    function burnSignal(
        uint256 _subgraphID,
        uint256 _nSignal,
        uint256 _tokensOutMin
    ) external override notPartialPaused {
        // Subgraph checks
        SubgraphData storage subgraphData = _getSubgraphOrRevert(_subgraphID);

        // Curator balance checks
        address curator = msg.sender;
        uint256 curatorNSignal = subgraphData.curatorNSignal[curator];
        require(_nSignal <= curatorNSignal, "GNS: Curator cannot withdraw more nSignal than they have");

        // Get tokens for name signal amount to burn
        uint256 vSignal = nSignalToVSignal(_subgraphID, _nSignal);
        uint256 tokens = curation().burn(subgraphData.subgraphDeploymentID, vSignal, _tokensOutMin);

        // Update pools
        subgraphData.vSignal = subgraphData.vSignal.sub(vSignal);
        subgraphData.nSignal = subgraphData.nSignal.sub(_nSignal);
        subgraphData.curatorNSignal[curator] = subgraphData.curatorNSignal[curator].sub(_nSignal);

        // Return the tokens to the nameCurator
        require(graphToken().transfer(curator, tokens), "GNS: Error sending tokens");

        emit SignalBurned(_subgraphID, curator, _nSignal, vSignal, tokens);
    }

    /**
     * @notice Move subgraph signal from sender to `_recipient`
     * @param _subgraphID Subgraph ID
     * @param _recipient Address to send the signal to
     * @param _amount The amount of nSignal to transfer
     */
    function transferSignal(
        uint256 _subgraphID,
        address _recipient,
        uint256 _amount
    ) external override notPartialPaused {
        require(_recipient != address(0), "GNS: Curator cannot transfer to the zero address");

        // Subgraph checks
        SubgraphData storage subgraphData = _getSubgraphOrRevert(_subgraphID);

        // Balance checks
        address curator = msg.sender;
        uint256 curatorBalance = subgraphData.curatorNSignal[curator];
        require(curatorBalance >= _amount, "GNS: Curator transfer amount exceeds balance");

        // Move the signal
        subgraphData.curatorNSignal[curator] = subgraphData.curatorNSignal[curator].sub(_amount);
        subgraphData.curatorNSignal[_recipient] = subgraphData.curatorNSignal[_recipient].add(_amount);

        emit SignalTransferred(_subgraphID, curator, _recipient, _amount);
    }

    /**
     * @notice Withdraw tokens from a deprecated subgraph.
     * When the subgraph is deprecated, any curator can call this function and
     * withdraw the GRT they are entitled for its original deposit
     * @param _subgraphID Subgraph ID
     */
    function withdraw(uint256 _subgraphID) external override notPartialPaused {
        // Subgraph validations
        SubgraphData storage subgraphData = _getSubgraphData(_subgraphID);
        require(subgraphData.disabled == true, "GNS: Must be disabled first");
        require(subgraphData.withdrawableGRT != 0, "GNS: No more GRT to withdraw");

        // Curator validations
        address curator = msg.sender;
        uint256 curatorNSignal = subgraphData.curatorNSignal[curator];
        require(curatorNSignal != 0, "GNS: No signal to withdraw GRT");

        // Get curator share of tokens to be withdrawn
        uint256 tokensOut = curatorNSignal.mul(subgraphData.withdrawableGRT).div(subgraphData.nSignal);
        subgraphData.curatorNSignal[curator] = 0;
        subgraphData.nSignal = subgraphData.nSignal.sub(curatorNSignal);
        subgraphData.withdrawableGRT = subgraphData.withdrawableGRT.sub(tokensOut);

        // Return tokens to the curator
        TokenUtils.pushTokens(graphToken(), curator, tokensOut);

        emit GRTWithdrawn(_subgraphID, curator, curatorNSignal, tokensOut);
    }

    /**
     * @notice Create subgraphID for legacy subgraph and mint ownership NFT.
     * @param _graphAccount Account that created the subgraph
     * @param _subgraphNumber The sequence number of the created subgraph
     * @param _subgraphMetadata IPFS hash for the subgraph metadata
     */
    function migrateLegacySubgraph(address _graphAccount, uint256 _subgraphNumber, bytes32 _subgraphMetadata) external {
        // Must be an existing legacy subgraph
        bool legacySubgraphExists = legacySubgraphData[_graphAccount][_subgraphNumber].subgraphDeploymentID != 0;
        require(legacySubgraphExists == true, "GNS: Subgraph does not exist");

        // Must not be a claimed subgraph
        uint256 subgraphID = _buildLegacySubgraphID(_graphAccount, _subgraphNumber);
        require(legacySubgraphKeys[subgraphID].account == address(0), "GNS: Subgraph was already claimed");

        // Store a reference for a legacy subgraph
        legacySubgraphKeys[subgraphID] = IGNS.LegacySubgraphKey({
            account: _graphAccount,
            accountSeqID: _subgraphNumber
        });

        // Delete state for legacy subgraph
        legacySubgraphs[_graphAccount][_subgraphNumber] = 0;

        // Mint the NFT and send to owner
        // The subgraph owner is the graph account that created it
        _mintNFT(_graphAccount, subgraphID);
        emit LegacySubgraphClaimed(_graphAccount, _subgraphNumber);

        // Set the token metadata
        _setSubgraphMetadata(subgraphID, _subgraphMetadata);
    }

    /**
     * @notice Return the total signal on the subgraph.
     * @param _subgraphID Subgraph ID
     * @return Total signal on the subgraph
     */
    function subgraphSignal(uint256 _subgraphID) external view override returns (uint256) {
        return _getSubgraphData(_subgraphID).nSignal;
    }

    /**
     * @notice Return the total tokens on the subgraph at current value.
     * @param _subgraphID Subgraph ID
     * @return Total tokens on the subgraph
     */
    function subgraphTokens(uint256 _subgraphID) external view override returns (uint256) {
        uint256 signal = _getSubgraphData(_subgraphID).nSignal;
        if (signal != 0) {
            (, uint256 tokens) = nSignalToTokens(_subgraphID, signal);
            return tokens;
        }
        return 0;
    }

    /**
     * @notice Return whether a subgraph is a legacy subgraph (created before subgraph NFTs).
     * @param _subgraphID Subgraph ID
     * @return Return true if subgraph is a legacy subgraph
     */
    function isLegacySubgraph(uint256 _subgraphID) external view override returns (bool) {
        (address account, ) = getLegacySubgraphKey(_subgraphID);
        return account != address(0);
    }

    /**
     * @notice Calculate subgraph signal to be returned for an amount of tokens.
     * @param _subgraphID Subgraph ID
     * @param _tokensIn Tokens being exchanged for subgraph signal
     * @return nSignalOut Amount of name signal minted
     * @return curationTax Amount of curation tax charged
     * @return vSignalOut Amount of version signal minted
     */
    function tokensToNSignal(
        uint256 _subgraphID,
        uint256 _tokensIn
    ) public view override returns (uint256, uint256, uint256) {
        SubgraphData storage subgraphData = _getSubgraphData(_subgraphID);
        (uint256 vSignal, uint256 curationTax) = curation().tokensToSignal(
            subgraphData.subgraphDeploymentID,
            _tokensIn
        );
        uint256 nSignal = vSignalToNSignal(_subgraphID, vSignal);
        return (vSignal, nSignal, curationTax);
    }

    /**
     * @notice Calculate tokens returned for an amount of subgraph signal.
     * @param _subgraphID Subgraph ID
     * @param _nSignalIn Subgraph signal being exchanged for tokens
     * @return vSignalOut Amount of version signal burned
     * @return tokensOut Amount of tokens returned
     */
    function nSignalToTokens(uint256 _subgraphID, uint256 _nSignalIn) public view override returns (uint256, uint256) {
        // Get subgraph or revert if not published
        // It does not make sense to convert signal from a disabled or non-existing one
        SubgraphData storage subgraphData = _getSubgraphOrRevert(_subgraphID);
        uint256 vSignal = nSignalToVSignal(_subgraphID, _nSignalIn);
        uint256 tokensOut = curation().signalToTokens(subgraphData.subgraphDeploymentID, vSignal);
        return (vSignal, tokensOut);
    }

    /**
     * @inheritdoc IGNS
     */
    function vSignalToNSignal(uint256 _subgraphID, uint256 _vSignalIn) public view override returns (uint256) {
        SubgraphData storage subgraphData = _getSubgraphData(_subgraphID);

        // Handle initialization by using 1:1 version to name signal
        if (subgraphData.vSignal == 0) {
            return _vSignalIn;
        }

        return subgraphData.nSignal.mul(_vSignalIn).div(subgraphData.vSignal);
    }

    /**
     * @inheritdoc IGNS
     */
    function nSignalToVSignal(uint256 _subgraphID, uint256 _nSignalIn) public view override returns (uint256) {
        SubgraphData storage subgraphData = _getSubgraphData(_subgraphID);
        return subgraphData.vSignal.mul(_nSignalIn).div(subgraphData.nSignal);
    }

    /**
     * @inheritdoc IGNS
     */
    function getCuratorSignal(uint256 _subgraphID, address _curator) public view override returns (uint256) {
        return _getSubgraphData(_subgraphID).curatorNSignal[_curator];
    }

    /**
     * @inheritdoc IGNS
     */
    function isPublished(uint256 _subgraphID) public view override returns (bool) {
        return _isPublished(_getSubgraphData(_subgraphID));
    }

    /**
     * @inheritdoc IGNS
     */
    function getLegacySubgraphKey(uint256 _subgraphID) public view override returns (address account, uint256 seqID) {
        LegacySubgraphKey storage legacySubgraphKey = legacySubgraphKeys[_subgraphID];
        account = legacySubgraphKey.account;
        seqID = legacySubgraphKey.accountSeqID;
    }

    /**
     * @inheritdoc IGNS
     */
    function ownerOf(uint256 _tokenID) public view override returns (address) {
        return subgraphNFT.ownerOf(_tokenID);
    }

    /**
     * @notice Calculate tax that owner will have to cover for upgrading or deprecating.
     * @param _tokens Tokens that were received from deprecating the old subgraph
     * @param _owner Subgraph owner
     * @param _curationTaxPercentage Tax percentage on curation deposits from Curation contract
     * @return Total tokens that will be sent to curation, _tokens + ownerTax
     */
    function _chargeOwnerTax(
        uint256 _tokens,
        address _owner,
        uint32 _curationTaxPercentage
    ) internal returns (uint256) {
        // If curation or owner tax are zero, we don't need to charge owner tax
        // so the amount of tokens to signal will remain the same.
        // Note if owner tax is zero but curation tax is nonzero, the curation tax
        // will still be charged (in Curation or L2Curation) - this function just calculates
        // the owner's additional tax.
        if (_curationTaxPercentage == 0 || ownerTaxPercentage == 0) {
            return _tokens;
        }

        // Tax on the total bonding curve funds
        uint256 taxOnOriginal = _tokens.mul(_curationTaxPercentage).div(MAX_PPM);
        // Total after the tax
        uint256 totalWithoutOwnerTax = _tokens.sub(taxOnOriginal);
        // The portion of tax that the owner will pay
        uint256 ownerTax = taxOnOriginal.mul(ownerTaxPercentage).div(MAX_PPM);

        uint256 totalWithOwnerTax = totalWithoutOwnerTax.add(ownerTax);

        // The total after tax, plus owner partial repay, divided by
        // the tax, to adjust it slightly upwards. ex:
        // 100 GRT, 5 GRT Tax, owner pays 100% --> 5 GRT
        // To get 100 in the protocol after tax, Owner deposits
        // ~5.26, as ~105.26 * .95 = 100
        uint256 totalAdjustedUp = totalWithOwnerTax.mul(MAX_PPM).div(
            uint256(MAX_PPM).sub(uint256(_curationTaxPercentage))
        );

        uint256 ownerTaxAdjustedUp = totalAdjustedUp.sub(_tokens);

        // Get the owner of the subgraph to reimburse the curation tax
        TokenUtils.pullTokens(graphToken(), _owner, ownerTaxAdjustedUp);

        return totalAdjustedUp;
    }

    /**
     * @notice Return the next subgraphID given the account that is creating the subgraph.
     * NOTE: This function updates the sequence ID for the account
     * @param _account The account creating the subgraph
     * @return Sequence ID for the account
     */
    function _nextSubgraphID(address _account) internal returns (uint256) {
        return _buildSubgraphID(_account, _nextAccountSeqID(_account));
    }

    /**
     * @notice Return a new consecutive sequence ID for an account and update to the next value.
     * NOTE: This function updates the sequence ID for the account
     * @param _account The account to get the next sequence ID for
     * @return Sequence ID for the account
     */
    function _nextAccountSeqID(address _account) internal returns (uint256) {
        uint256 seqID = nextAccountSeqID[_account];
        nextAccountSeqID[_account] = nextAccountSeqID[_account].add(1);
        return seqID;
    }

    /**
     * @notice Mint the NFT for the subgraph.
     * @param _owner Owner address
     * @param _tokenID Subgraph ID
     */
    function _mintNFT(address _owner, uint256 _tokenID) internal {
        subgraphNFT.mint(_owner, _tokenID);
    }

    /**
     * @notice Burn the NFT for the subgraph.
     * @param _tokenID Subgraph ID
     */
    function _burnNFT(uint256 _tokenID) internal {
        subgraphNFT.burn(_tokenID);
    }

    /**
     * @notice Set the subgraph metadata.
     * @param _tokenID Subgraph ID
     * @param _subgraphMetadata IPFS hash of the subgraph metadata
     */
    function _setSubgraphMetadata(uint256 _tokenID, bytes32 _subgraphMetadata) internal {
        subgraphNFT.setSubgraphMetadata(_tokenID, _subgraphMetadata);

        // Even if the following event is emitted in the NFT we emit it here to facilitate
        // subgraph indexing
        emit SubgraphMetadataUpdated(_tokenID, _subgraphMetadata);
    }

    /**
     * @notice Get subgraph data.
     * This function will first look for a v1 subgraph and return it if found.
     * @param _subgraphID Subgraph ID
     * @return Subgraph Data
     */
    function _getSubgraphData(uint256 _subgraphID) internal view virtual returns (SubgraphData storage) {
        // If there is a legacy subgraph created return it
        LegacySubgraphKey storage legacySubgraphKey = legacySubgraphKeys[_subgraphID];
        if (legacySubgraphKey.account != address(0)) {
            return legacySubgraphData[legacySubgraphKey.account][legacySubgraphKey.accountSeqID];
        }
        // Return new subgraph type
        return subgraphs[_subgraphID];
    }

    /**
     * @notice Return whether a subgraph is published.
     * @param _subgraphData Subgraph Data
     * @return Return true if subgraph is currently published
     */
    function _isPublished(SubgraphData storage _subgraphData) internal view returns (bool) {
        return _subgraphData.subgraphDeploymentID != 0 && _subgraphData.disabled == false;
    }

    /**
     * @notice Return the subgraph data or revert if not published or deprecated.
     * @param _subgraphID Subgraph ID
     * @return Subgraph Data
     */
    function _getSubgraphOrRevert(uint256 _subgraphID) internal view returns (SubgraphData storage) {
        SubgraphData storage subgraphData = _getSubgraphData(_subgraphID);
        require(_isPublished(subgraphData) == true, "GNS: Must be active");
        return subgraphData;
    }

    /**
     * @notice Build a subgraph ID based on the account creating it and a sequence number for that account.
     * Only used for legacy subgraphs being migrated, as new ones will also use the chainid.
     * Subgraph ID is the keccak hash of account+seqID
     * @param _account The account creating the subgraph
     * @param _seqID The sequence ID for the account
     * @return Subgraph ID
     */
    function _buildLegacySubgraphID(address _account, uint256 _seqID) internal pure returns (uint256) {
        return uint256(keccak256(abi.encodePacked(_account, _seqID)));
    }

    /**
     * @notice Build a subgraph ID based on the account creating it and a sequence number for that account.
     * Subgraph ID is the keccak hash of account+seqID
     * @param _account The account creating the subgraph
     * @param _seqID The sequence ID for the account
     * @return Subgraph ID
     */
    function _buildSubgraphID(address _account, uint256 _seqID) internal pure returns (uint256) {
        uint256 chainId;
        // Too bad solidity 0.7.6 still doesn't have block.chainid
        // solhint-disable-next-line no-inline-assembly
        assembly {
            chainId := chainid()
        }
        return uint256(keccak256(abi.encodePacked(_account, _seqID, chainId)));
    }

    /**
     * @notice Internal: Set the owner tax percentage. This is used to prevent a subgraph owner to drain all
     * the name curators tokens while upgrading or deprecating and is configurable in parts per million.
     * @param _ownerTaxPercentage Owner tax percentage
     */
    function _setOwnerTaxPercentage(uint32 _ownerTaxPercentage) private {
        require(_ownerTaxPercentage <= MAX_PPM, "Owner tax must be MAX_PPM or less");
        ownerTaxPercentage = _ownerTaxPercentage;
        emit ParameterUpdated("ownerTaxPercentage");
    }

    /**
     * @notice Internal: Set the NFT registry contract
     * @param _subgraphNFT Address of the ERC721 contract
     */
    function _setSubgraphNFT(address _subgraphNFT) private {
        require(_subgraphNFT != address(0), "NFT address cant be zero");
        require(AddressUpgradeable.isContract(_subgraphNFT), "NFT must be valid");

        subgraphNFT = ISubgraphNFT(_subgraphNFT);
        emit SubgraphNFTUpdated(_subgraphNFT);
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity ^0.7.6;
pragma abicoder v2;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable gas-indexed-events, gas-small-strings, gas-strict-inequalities

import { AddressUpgradeable } from "@openzeppelin/contracts-upgradeable/utils/AddressUpgradeable.sol";
import { SafeMathUpgradeable } from "@openzeppelin/contracts-upgradeable/math/SafeMathUpgradeable.sol";
import { ClonesUpgradeable } from "@openzeppelin/contracts-upgradeable/proxy/ClonesUpgradeable.sol";

import { GraphUpgradeable } from "../../upgrades/GraphUpgradeable.sol";
import { TokenUtils } from "../../utils/TokenUtils.sol";
import { IRewardsManager } from "@graphprotocol/interfaces/contracts/contracts/rewards/IRewardsManager.sol";
import { Managed } from "../../governance/Managed.sol";
import { IGraphToken } from "@graphprotocol/interfaces/contracts/contracts/token/IGraphToken.sol";
import { CurationV3Storage } from "../../curation/CurationStorage.sol";
import { IGraphCurationToken } from "@graphprotocol/interfaces/contracts/contracts/curation/IGraphCurationToken.sol";
import { IL2Curation } from "@graphprotocol/interfaces/contracts/contracts/l2/curation/IL2Curation.sol";

/**
 * @title L2Curation contract
 * @author Edge & Node
 * @notice Allows curators to signal on subgraph deployments that might be relevant to indexers by
 * staking Graph Tokens (GRT). Additionally, curators earn fees from the Query Market related to the
 * subgraph deployment they curate.
 * A curators deposit goes to a curation pool along with the deposits of other curators,
 * only one such pool exists for each subgraph deployment.
 * The contract mints Graph Curation Shares (GCS) according to a (flat) bonding curve for each individual
 * curation pool where GRT is deposited.
 * Holders can burn GCS using this contract to get GRT tokens back according to the
 * bonding curve.
 */
contract L2Curation is CurationV3Storage, GraphUpgradeable, IL2Curation {
    using SafeMathUpgradeable for uint256;

    /// @dev 100% in parts per million
    uint32 private constant MAX_PPM = 1000000;

    /// @dev Amount of signal you get with your minimum token deposit
    uint256 private constant SIGNAL_PER_MINIMUM_DEPOSIT = 1; // 1e-18 signal as 18 decimal number

    /// @dev Reserve ratio for all subgraphs set to 100% for a flat bonding curve
    // solhint-disable-next-line immutable-vars-naming
    uint32 private immutable fixedReserveRatio = MAX_PPM;

    // -- Events --

    /**
     * @notice Emitted when `curator` deposited `tokens` on `subgraphDeploymentID` as curation signal.
     * The `curator` receives `signal` amount according to the curation pool bonding curve.
     * An amount of `curationTax` will be collected and burned.
     * @param curator Address of the curator
     * @param subgraphDeploymentID Subgraph deployment being signaled on
     * @param tokens Amount of tokens deposited
     * @param signal Amount of signal minted
     * @param curationTax Amount of tokens burned as curation tax
     */
    event Signalled(
        address indexed curator,
        bytes32 indexed subgraphDeploymentID,
        uint256 tokens,
        uint256 signal,
        uint256 curationTax
    );

    /**
     * @notice Emitted when `curator` burned `signal` for a `subgraphDeploymentID`.
     * The curator will receive `tokens` according to the value of the bonding curve.
     * @param curator Address of the curator
     * @param subgraphDeploymentID Subgraph deployment being signaled on
     * @param tokens Amount of tokens received
     * @param signal Amount of signal burned
     */
    event Burned(address indexed curator, bytes32 indexed subgraphDeploymentID, uint256 tokens, uint256 signal);

    /**
     * @notice Emitted when `tokens` amount were collected for `subgraphDeploymentID` as part of fees
     * distributed by an indexer from query fees received from state channels.
     * @param subgraphDeploymentID Subgraph deployment that collected fees
     * @param tokens Amount of tokens collected as fees
     */
    event Collected(bytes32 indexed subgraphDeploymentID, uint256 tokens);

    /**
     * @notice Emitted when the subgraph service is set
     * @param newSubgraphService Address of the new subgraph service
     */
    event SubgraphServiceSet(address indexed newSubgraphService);

    /**
     * @dev Modifier for functions that can only be called by the GNS contract
     */
    modifier onlyGNS() {
        require(msg.sender == address(gns()), "Only the GNS can call this");
        _;
    }

    /**
     * @notice Initialize the L2Curation contract
     * @param _controller Controller contract that manages this contract
     * @param _curationTokenMaster Address of the GraphCurationToken master copy
     * @param _curationTaxPercentage Percentage of curation tax to be collected
     * @param _minimumCurationDeposit Minimum amount of tokens that can be deposited as curation signal
     */
    function initialize(
        address _controller,
        address _curationTokenMaster,
        uint32 _curationTaxPercentage,
        uint256 _minimumCurationDeposit
    ) external onlyImpl initializer {
        Managed._initialize(_controller);

        // For backwards compatibility:
        defaultReserveRatio = fixedReserveRatio;
        emit ParameterUpdated("defaultReserveRatio");
        _setCurationTaxPercentage(_curationTaxPercentage);
        _setMinimumCurationDeposit(_minimumCurationDeposit);
        _setCurationTokenMaster(_curationTokenMaster);
    }

    /**
     * @notice Set the default reserve ratio - not implemented in L2
     * @dev We only keep this for compatibility with ICuration
     */
    // solhint-disable-next-line use-natspec
    function setDefaultReserveRatio(uint32 /* _defaultReserveRatio */) external view override onlyGovernor {
        revert("Not implemented in L2");
    }

    /**
     * @dev Set the minimum deposit amount for curators.
     * @notice Update the minimum deposit amount to `_minimumCurationDeposit`
     * @param _minimumCurationDeposit Minimum amount of tokens required deposit
     */
    function setMinimumCurationDeposit(uint256 _minimumCurationDeposit) external override onlyGovernor {
        _setMinimumCurationDeposit(_minimumCurationDeposit);
    }

    /**
     * @notice Set the curation tax percentage to charge when a curator deposits GRT tokens.
     * @param _percentage Curation tax percentage charged when depositing GRT tokens
     */
    function setCurationTaxPercentage(uint32 _percentage) external override onlyGovernor {
        _setCurationTaxPercentage(_percentage);
    }

    /**
     * @notice Set the master copy to use as clones for the curation token.
     * @param _curationTokenMaster Address of implementation contract to use for curation tokens
     */
    function setCurationTokenMaster(address _curationTokenMaster) external override onlyGovernor {
        _setCurationTokenMaster(_curationTokenMaster);
    }

    /**
     * @notice Set the subgraph service address
     * @param _subgraphService Address of the subgraph service contract
     */
    function setSubgraphService(address _subgraphService) external override onlyGovernor {
        subgraphService = _subgraphService;
        emit SubgraphServiceSet(_subgraphService);
    }

    /**
     * @notice Assign Graph Tokens collected as curation fees to the curation pool reserve.
     * @dev This function can only be called by the Staking contract and will do the Bookkeeping of
     * transferred tokens into this contract.
     * @param _subgraphDeploymentID SubgraphDeployment where funds should be allocated as reserves
     * @param _tokens Amount of Graph Tokens to add to reserves
     */
    function collect(bytes32 _subgraphDeploymentID, uint256 _tokens) external override {
        // Only SubgraphService and Staking contract are authorized as callers
        require(
            msg.sender == subgraphService || msg.sender == address(staking()),
            "Caller must be the subgraph service or staking contract"
        );

        // Must be curated to accept tokens
        require(isCurated(_subgraphDeploymentID), "Subgraph deployment must be curated to collect fees");

        // Collect new funds into reserve
        CurationPool storage curationPool = pools[_subgraphDeploymentID];
        curationPool.tokens = curationPool.tokens.add(_tokens);

        emit Collected(_subgraphDeploymentID, _tokens);
    }

    /**
     * @notice Deposit Graph Tokens in exchange for signal of a SubgraphDeployment curation pool.
     * @param _subgraphDeploymentID Subgraph deployment pool from where to mint signal
     * @param _tokensIn Amount of Graph Tokens to deposit
     * @param _signalOutMin Expected minimum amount of signal to receive
     * @return Signal minted
     * @return Curation tax paid
     */
    function mint(
        bytes32 _subgraphDeploymentID,
        uint256 _tokensIn,
        uint256 _signalOutMin
    ) external override notPartialPaused returns (uint256, uint256) {
        // Need to deposit some funds
        require(_tokensIn != 0, "Cannot deposit zero tokens");

        // Exchange GRT tokens for GCS of the subgraph pool
        (uint256 signalOut, uint256 curationTax) = tokensToSignal(_subgraphDeploymentID, _tokensIn);

        // Slippage protection
        require(signalOut >= _signalOutMin, "Slippage protection");

        address curator = msg.sender;
        CurationPool storage curationPool = pools[_subgraphDeploymentID];

        // If it hasn't been curated before then initialize the curve
        if (!isCurated(_subgraphDeploymentID)) {
            // Note we don't set the reserveRatio to save the gas
            // cost, but in the pools() getter we'll inject the value.

            // If no signal token for the pool - create one
            if (address(curationPool.gcs) == address(0)) {
                // Use a minimal proxy to reduce gas cost
                IGraphCurationToken gcs = IGraphCurationToken(ClonesUpgradeable.clone(curationTokenMaster));
                gcs.initialize(address(this));
                curationPool.gcs = gcs;
            }
        }

        // Trigger update rewards calculation snapshot
        _updateRewards(_subgraphDeploymentID);

        // Transfer tokens from the curator to this contract
        // Burn the curation tax
        // NOTE: This needs to happen after _updateRewards snapshot as that function
        // is using balanceOf(curation)
        IGraphToken _graphToken = graphToken();
        TokenUtils.pullTokens(_graphToken, curator, _tokensIn);
        TokenUtils.burnTokens(_graphToken, curationTax);

        // Update curation pool
        curationPool.tokens = curationPool.tokens.add(_tokensIn.sub(curationTax));
        curationPool.gcs.mint(curator, signalOut);

        emit Signalled(curator, _subgraphDeploymentID, _tokensIn, signalOut, curationTax);

        return (signalOut, curationTax);
    }

    /**
     * @inheritdoc IL2Curation
     */
    function mintTaxFree(
        bytes32 _subgraphDeploymentID,
        uint256 _tokensIn
    ) external override notPartialPaused onlyGNS returns (uint256) {
        // Need to deposit some funds
        require(_tokensIn != 0, "Cannot deposit zero tokens");

        // Exchange GRT tokens for GCS of the subgraph pool (no tax)
        uint256 signalOut = _tokensToSignal(_subgraphDeploymentID, _tokensIn);

        address curator = msg.sender;
        CurationPool storage curationPool = pools[_subgraphDeploymentID];

        // If it hasn't been curated before then initialize the curve
        if (!isCurated(_subgraphDeploymentID)) {
            // Note we don't set the reserveRatio to save the gas
            // cost, but in the pools() getter we'll inject the value.

            // If no signal token for the pool - create one
            if (address(curationPool.gcs) == address(0)) {
                // Use a minimal proxy to reduce gas cost
                IGraphCurationToken gcs = IGraphCurationToken(ClonesUpgradeable.clone(curationTokenMaster));
                gcs.initialize(address(this));
                curationPool.gcs = gcs;
            }
        }

        // Trigger update rewards calculation snapshot
        _updateRewards(_subgraphDeploymentID);

        // Transfer tokens from the curator to this contract
        // NOTE: This needs to happen after _updateRewards snapshot as that function
        // is using balanceOf(curation)
        IGraphToken _graphToken = graphToken();
        TokenUtils.pullTokens(_graphToken, curator, _tokensIn);

        // Update curation pool
        curationPool.tokens = curationPool.tokens.add(_tokensIn);
        curationPool.gcs.mint(curator, signalOut);

        emit Signalled(curator, _subgraphDeploymentID, _tokensIn, signalOut, 0);

        return signalOut;
    }

    /**
     * @dev Return an amount of signal to get tokens back.
     * @notice Burn _signalIn from the SubgraphDeployment curation pool
     * @param _subgraphDeploymentID SubgraphDeployment for which the curator is returning signal
     * @param _signalIn Amount of signal to return
     * @param _tokensOutMin Expected minimum amount of tokens to receive
     * @return Amount of tokens returned to the sender
     */
    function burn(
        bytes32 _subgraphDeploymentID,
        uint256 _signalIn,
        uint256 _tokensOutMin
    ) external override notPartialPaused returns (uint256) {
        address curator = msg.sender;

        // Validations
        require(_signalIn != 0, "Cannot burn zero signal");
        require(getCuratorSignal(curator, _subgraphDeploymentID) >= _signalIn, "Cannot burn more signal than you own");

        // Get the amount of tokens to refund based on returned signal
        uint256 tokensOut = signalToTokens(_subgraphDeploymentID, _signalIn);

        // Slippage protection
        require(tokensOut >= _tokensOutMin, "Slippage protection");

        // Trigger update rewards calculation
        _updateRewards(_subgraphDeploymentID);

        // Update curation pool
        CurationPool storage curationPool = pools[_subgraphDeploymentID];
        curationPool.tokens = curationPool.tokens.sub(tokensOut);
        curationPool.gcs.burnFrom(curator, _signalIn);

        // If all signal burnt delete the curation pool except for the
        // curation token contract to avoid recreating it on a new mint
        if (getCurationPoolSignal(_subgraphDeploymentID) == 0) {
            curationPool.tokens = 0;
        }

        // Return the tokens to the curator
        TokenUtils.pushTokens(graphToken(), curator, tokensOut);

        emit Burned(curator, _subgraphDeploymentID, tokensOut, _signalIn);

        return tokensOut;
    }

    /**
     * @notice Get the amount of token reserves in a curation pool.
     * @param _subgraphDeploymentID Subgraph deployment curation pool
     * @return Amount of token reserves in the curation pool
     */
    function getCurationPoolTokens(bytes32 _subgraphDeploymentID) external view override returns (uint256) {
        return pools[_subgraphDeploymentID].tokens;
    }

    /**
     * @notice Check if any GRT tokens are deposited for a SubgraphDeployment.
     * @param _subgraphDeploymentID SubgraphDeployment to check if curated
     * @return True if curated
     */
    function isCurated(bytes32 _subgraphDeploymentID) public view override returns (bool) {
        return pools[_subgraphDeploymentID].tokens != 0;
    }

    /**
     * @notice Get the amount of signal a curator has in a curation pool.
     * @param _curator Curator owning the signal tokens
     * @param _subgraphDeploymentID Subgraph deployment curation pool
     * @return Amount of signal owned by a curator for the subgraph deployment
     */
    function getCuratorSignal(address _curator, bytes32 _subgraphDeploymentID) public view override returns (uint256) {
        IGraphCurationToken gcs = pools[_subgraphDeploymentID].gcs;
        return (address(gcs) == address(0)) ? 0 : gcs.balanceOf(_curator);
    }

    /**
     * @notice Get the amount of signal in a curation pool.
     * @param _subgraphDeploymentID Subgraph deployment curation pool
     * @return Amount of signal minted for the subgraph deployment
     */
    function getCurationPoolSignal(bytes32 _subgraphDeploymentID) public view override returns (uint256) {
        IGraphCurationToken gcs = pools[_subgraphDeploymentID].gcs;
        return (address(gcs) == address(0)) ? 0 : gcs.totalSupply();
    }

    /**
     * @notice Calculate amount of signal that can be bought with tokens in a curation pool.
     * This function considers and excludes the deposit tax.
     * @param _subgraphDeploymentID Subgraph deployment to mint signal
     * @param _tokensIn Amount of tokens used to mint signal
     * @return Amount of signal that can be bought
     * @return Amount of GRT that would be subtracted as curation tax
     */
    function tokensToSignal(
        bytes32 _subgraphDeploymentID,
        uint256 _tokensIn
    ) public view override returns (uint256, uint256) {
        // Calculate tokens after tax first, subtract that from the tokens in
        // to get the curation tax to avoid rounding down to zero.
        uint256 tokensAfterCurationTax = uint256(MAX_PPM).sub(curationTaxPercentage).mul(_tokensIn).div(MAX_PPM);
        uint256 curationTax = _tokensIn.sub(tokensAfterCurationTax);
        uint256 signalOut = _tokensToSignal(_subgraphDeploymentID, tokensAfterCurationTax);
        return (signalOut, curationTax);
    }

    /**
     * @inheritdoc IL2Curation
     */
    function tokensToSignalNoTax(
        bytes32 _subgraphDeploymentID,
        uint256 _tokensIn
    ) public view override returns (uint256) {
        return _tokensToSignal(_subgraphDeploymentID, _tokensIn);
    }

    /**
     * @inheritdoc IL2Curation
     */
    function tokensToSignalToTokensNoTax(
        bytes32 _subgraphDeploymentID,
        uint256 _tokensIn
    ) external view override returns (uint256) {
        require(_tokensIn != 0, "Can't calculate with 0 tokens");
        uint256 signal = _tokensToSignal(_subgraphDeploymentID, _tokensIn);
        CurationPool memory curationPool = pools[_subgraphDeploymentID];
        uint256 poolSignalAfter = getCurationPoolSignal(_subgraphDeploymentID).add(signal);
        uint256 poolTokensAfter = curationPool.tokens.add(_tokensIn);
        return poolTokensAfter.mul(signal).div(poolSignalAfter);
    }

    /**
     * @notice Calculate number of tokens to get when burning signal from a curation pool.
     * @param _subgraphDeploymentID Subgraph deployment for which to burn signal
     * @param _signalIn Amount of signal to burn
     * @return Amount of tokens to get for an amount of signal
     */
    function signalToTokens(bytes32 _subgraphDeploymentID, uint256 _signalIn) public view override returns (uint256) {
        CurationPool memory curationPool = pools[_subgraphDeploymentID];
        uint256 curationPoolSignal = getCurationPoolSignal(_subgraphDeploymentID);
        require(curationPool.tokens != 0, "Subgraph deployment must be curated to perform calculations");
        require(curationPoolSignal >= _signalIn, "Signal must be above or equal to signal issued in the curation pool");

        return curationPool.tokens.mul(_signalIn).div(curationPoolSignal);
    }

    /**
     * @notice Internal: Set the minimum deposit amount for curators.
     * Update the minimum deposit amount to `_minimumCurationDeposit`
     * @param _minimumCurationDeposit Minimum amount of tokens required deposit
     */
    function _setMinimumCurationDeposit(uint256 _minimumCurationDeposit) private {
        require(_minimumCurationDeposit != 0, "Minimum curation deposit cannot be 0");

        minimumCurationDeposit = _minimumCurationDeposit;
        emit ParameterUpdated("minimumCurationDeposit");
    }

    /**
     * @notice Internal: Set the curation tax percentage to charge when a curator deposits GRT tokens.
     * @param _percentage Curation tax percentage charged when depositing GRT tokens
     */
    function _setCurationTaxPercentage(uint32 _percentage) private {
        require(_percentage <= MAX_PPM, "Curation tax percentage must be below or equal to MAX_PPM");

        curationTaxPercentage = _percentage;
        emit ParameterUpdated("curationTaxPercentage");
    }

    /**
     * @notice Internal: Set the master copy to use as clones for the curation token.
     * @param _curationTokenMaster Address of implementation contract to use for curation tokens
     */
    function _setCurationTokenMaster(address _curationTokenMaster) private {
        require(_curationTokenMaster != address(0), "Token master must be non-empty");
        require(AddressUpgradeable.isContract(_curationTokenMaster), "Token master must be a contract");

        curationTokenMaster = _curationTokenMaster;
        emit ParameterUpdated("curationTokenMaster");
    }

    /**
     * @notice Triggers an update of rewards due to a change in signal.
     * @param _subgraphDeploymentID Subgraph deployment updated
     */
    function _updateRewards(bytes32 _subgraphDeploymentID) private {
        IRewardsManager rewardsManager = rewardsManager();
        if (address(rewardsManager) != address(0)) {
            rewardsManager.onSubgraphSignalUpdate(_subgraphDeploymentID);
        }
    }

    /**
     * @notice Calculate amount of signal that can be bought with tokens in a curation pool.
     * @param _subgraphDeploymentID Subgraph deployment to mint signal
     * @param _tokensIn Amount of tokens used to mint signal
     * @return Amount of signal that can be bought with tokens
     */
    function _tokensToSignal(bytes32 _subgraphDeploymentID, uint256 _tokensIn) private view returns (uint256) {
        // Get curation pool tokens and signal
        CurationPool memory curationPool = pools[_subgraphDeploymentID];

        // Init curation pool
        if (curationPool.tokens == 0) {
            require(_tokensIn >= minimumCurationDeposit, "Curation deposit is below minimum required");
            return
                SIGNAL_PER_MINIMUM_DEPOSIT.add(
                    SIGNAL_PER_MINIMUM_DEPOSIT.mul(_tokensIn.sub(minimumCurationDeposit)).div(minimumCurationDeposit)
                );
        }

        return getCurationPoolSignal(_subgraphDeploymentID).mul(_tokensIn).div(curationPool.tokens);
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity 0.8.27 || 0.8.33;

import { GraphDirectory } from "../../utilities/GraphDirectory.sol";

/* solhint-disable var-name-mixedcase */

/**
 * @title Graph Managed contract
 * @author Edge & Node
 * @notice The Managed contract provides an interface to interact with the Controller
 * @dev For Graph Horizon this contract is mostly a shell that uses {GraphDirectory}, however since the {HorizonStaking}
 * contract uses it we need to preserve the storage layout.
 * Inspired by Livepeer: https://github.com/livepeer/protocol/blob/streamflow/contracts/Controller.sol
 * @custom:security-contact Please email security+contracts@thegraph.com if you find any
 * bugs. We may have an active bug bounty program.
 */
abstract contract Managed is GraphDirectory {
    // -- State --

    // forge-lint: disable-next-item(mixed-case-variable)
    /// @notice Controller that manages this contract
    address private __DEPRECATED_controller;

    // forge-lint: disable-next-item(mixed-case-variable)
    /// @dev Cache for the addresses of the contracts retrieved from the controller
    mapping(bytes32 contractName => address contractAddress) private __DEPRECATED_addressCache;

    // forge-lint: disable-next-item(mixed-case-variable)
    /// @dev Gap for future storage variables
    uint256[10] private __gap;

    /**
     * @notice Thrown when a protected function is called and the contract is paused.
     */
    error ManagedIsPaused();

    /**
     * @notice Thrown when a the caller is not the expected controller address.
     */
    error ManagedOnlyController();

    /**
     * @notice Thrown when a the caller is not the governor.
     */
    error ManagedOnlyGovernor();

    // forge-lint: disable-next-item(unwrapped-modifier-logic)
    /**
     * @dev Revert if the controller is paused
     */
    modifier notPaused() {
        require(!_graphController().paused(), ManagedIsPaused());
        _;
    }

    // forge-lint: disable-next-item(unwrapped-modifier-logic)
    /**
     * @dev Revert if the caller is not the governor
     */
    modifier onlyGovernor() {
        require(msg.sender == _graphController().getGovernor(), ManagedOnlyGovernor());
        _;
    }

    /**
     * @notice Initialize the contract
     * @param controller_ The address of the Graph controller contract
     */
    constructor(address controller_) GraphDirectory(controller_) {}
}

// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity ^0.7.6 || 0.8.27 || 0.8.33;

/* solhint-disable gas-custom-errors */ // Cannot use custom errors with 0.7.6

import { IGraphToken } from "@graphprotocol/interfaces/contracts/contracts/token/IGraphToken.sol";

/**
 * @title TokenUtils library
 * @author Edge & Node
 * @notice This library contains utility functions for handling tokens (transfers and burns).
 * It is specifically adapted for the GraphToken, so does not need to handle edge cases
 * for other tokens.
 */
library TokenUtils {
    /**
     * @notice Pull tokens from an address to this contract.
     * @param _graphToken Token to transfer
     * @param _from Address sending the tokens
     * @param _amount Amount of tokens to transfer
     */
    function pullTokens(IGraphToken _graphToken, address _from, uint256 _amount) internal {
        if (_amount > 0) {
            require(_graphToken.transferFrom(_from, address(this), _amount), "!transfer");
        }
    }

    /**
     * @notice Push tokens from this contract to a receiving address.
     * @param _graphToken Token to transfer
     * @param _to Address receiving the tokens
     * @param _amount Amount of tokens to transfer
     */
    function pushTokens(IGraphToken _graphToken, address _to, uint256 _amount) internal {
        if (_amount > 0) {
            require(_graphToken.transfer(_to, _amount), "!transfer");
        }
    }

    /**
     * @notice Burn tokens held by this contract.
     * @param _graphToken Token to burn
     * @param _amount Amount of tokens to burn
     */
    function burnTokens(IGraphToken _graphToken, uint256 _amount) internal {
        if (_amount > 0) {
            _graphToken.burn(_amount);
        }
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

// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity ^0.7.6;
pragma abicoder v2;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable gas-increment-by-one

import { IMulticall } from "@graphprotocol/interfaces/contracts/contracts/base/IMulticall.sol";

// Inspired by https://github.com/Uniswap/uniswap-v3-periphery/blob/main/contracts/base/Multicall.sol
// Note: Removed payable from the multicall

/**
 * @title Multicall
 * @author Edge & Node
 * @notice Enables calling multiple methods in a single call to the contract
 */
abstract contract Multicall is IMulticall {
    /// @inheritdoc IMulticall
    function multicall(bytes[] calldata data) external override returns (bytes[] memory results) {
        results = new bytes[](data.length);
        for (uint256 i = 0; i < data.length; i++) {
            (bool success, bytes memory result) = address(this).delegatecall(data[i]); // solhint-disable-line avoid-low-level-calls

            if (!success) {
                // Next 5 lines from https://ethereum.stackexchange.com/a/83577
                if (result.length < 68) revert();
                // solhint-disable-next-line no-inline-assembly
                assembly {
                    result := add(result, 0x04)
                }
                revert(abi.decode(result, (string)));
            }

            results[i] = result;
        }
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity ^0.7.6 || 0.8.27 || 0.8.33;

/* solhint-disable gas-custom-errors */ // Cannot use custom errors with 0.7.6

import { IGraphProxy } from "@graphprotocol/interfaces/contracts/contracts/upgrades/IGraphProxy.sol";

/**
 * @title Graph Upgradeable
 * @author Edge & Node
 * @notice This contract is intended to be inherited from upgradeable contracts.
 */
abstract contract GraphUpgradeable {
    /**
     * @dev Storage slot with the address of the current implementation.
     * This is the keccak-256 hash of "eip1967.proxy.implementation" subtracted by 1, and is
     * validated in the constructor.
     */
    bytes32 internal constant IMPLEMENTATION_SLOT = 0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc;

    /**
     * @dev Check if the caller is the proxy admin.
     * @param _proxy The proxy contract to check admin for
     */
    modifier onlyProxyAdmin(IGraphProxy _proxy) {
        require(msg.sender == _proxy.admin(), "Caller must be the proxy admin");
        _;
    }

    /**
     * @dev Check if the caller is the implementation.
     */
    modifier onlyImpl() {
        require(msg.sender == _implementation(), "Only implementation");
        _;
    }

    /**
     * @notice Returns the current implementation.
     * @return impl Address of the current implementation
     */
    function _implementation() internal view returns (address impl) {
        bytes32 slot = IMPLEMENTATION_SLOT;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            impl := sload(slot)
        }
    }

    /**
     * @notice Accept to be an implementation of proxy.
     * @param _proxy Proxy to accept
     */
    function acceptProxy(IGraphProxy _proxy) external onlyProxyAdmin(_proxy) {
        _proxy.acceptUpgrade();
    }

    /**
     * @notice Accept to be an implementation of proxy and then call a function from the new
     * implementation as specified by `_data`, which should be an encoded function call. This is
     * useful to initialize new storage variables in the proxied contract.
     * @param _proxy Proxy to accept
     * @param _data Calldata for the initialization function call (including selector)
     */
    function acceptProxyAndCall(IGraphProxy _proxy, bytes calldata _data) external onlyProxyAdmin(_proxy) {
        _proxy.acceptUpgradeAndCall(_data);
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later
// solhint-disable one-contract-per-file

pragma solidity ^0.7.6;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable named-parameters-mapping

import { Initializable } from "@openzeppelin/contracts-upgradeable/proxy/Initializable.sol";

import { ICuration } from "@graphprotocol/interfaces/contracts/contracts/curation/ICuration.sol";
import { IGraphCurationToken } from "@graphprotocol/interfaces/contracts/contracts/curation/IGraphCurationToken.sol";
import { Managed } from "../governance/Managed.sol";

/**
 * @title Curation Storage version 1
 * @author Edge & Node
 * @notice This contract holds the first version of the storage variables
 * for the Curation and L2Curation contracts.
 * When adding new variables, create a new version that inherits this and update
 * the contracts to use the new version instead.
 */
abstract contract CurationV1Storage is Managed, ICuration {
    // -- Pool --

    /**
     * @dev CurationPool structure that holds the pool's state
     * for a particular subgraph deployment.
     * @param tokens GRT Tokens stored as reserves for the subgraph deployment
     * @param reserveRatio Ratio for the bonding curve, unused and deprecated in L2 where it will always be 100% but appear as 0
     * @param gcs Curation token contract for this curation pool
     */
    struct CurationPool {
        uint256 tokens; // GRT Tokens stored as reserves for the subgraph deployment
        uint32 reserveRatio; // Ratio for the bonding curve, unused and deprecated in L2 where it will always be 100% but appear as 0
        IGraphCurationToken gcs; // Curation token contract for this curation pool
    }

    // -- State --

    /// @notice Tax charged when curators deposit funds.
    /// Parts per million. (Allows for 4 decimal points, 999,999 = 99.9999%)
    uint32 public override curationTaxPercentage;

    /// @notice Default reserve ratio to configure curator shares bonding curve
    /// Parts per million. (Allows for 4 decimal points, 999,999 = 99.9999%).
    /// Unused in L2.
    uint32 public defaultReserveRatio;

    /// @notice Master copy address that holds implementation of curation token.
    /// @dev This is used as the target for GraphCurationToken clones.
    address public curationTokenMaster;

    /// @notice Minimum amount allowed to be deposited by curators to initialize a pool
    /// @dev This is the `startPoolBalance` for the bonding curve
    uint256 public minimumCurationDeposit;

    /// @notice Bonding curve library
    /// Unused in L2.
    address public bondingCurve;

    /// @notice Mapping of subgraphDeploymentID => CurationPool
    /// There is only one CurationPool per SubgraphDeploymentID
    mapping(bytes32 => CurationPool) public pools;
}

/**
 * @title Curation Storage version 2
 * @author Edge & Node
 * @notice This contract holds the second version of the storage variables
 * for the Curation and L2Curation contracts.
 * It doesn't add new variables at this contract's level, but adds the Initializable
 * contract to the inheritance chain, which includes storage variables.
 * When adding new variables, create a new version that inherits this and update
 * the contracts to use the new version instead.
 */
abstract contract CurationV2Storage is CurationV1Storage, Initializable {
    // Nothing here, just adding Initializable
}

/**
 * @title Curation Storage version 3
 * @author Edge & Node
 * @notice This contract holds the third version of the storage variables for the Curation and L2Curation contracts
 * @dev This contract holds the third version of the storage variables
 * for the Curation and L2Curation contracts.
 * It adds a new variable subgraphService to the storage.
 * When adding new variables, create a new version that inherits this and update
 * the contracts to use the new version instead.
 */
abstract contract CurationV3Storage is CurationV2Storage {
    /// @notice Address of the subgraph service
    address public subgraphService;
}

// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity 0.8.27 || 0.8.33;

import { IGraphToken } from "@graphprotocol/interfaces/contracts/contracts/token/IGraphToken.sol";
import { IHorizonStaking } from "@graphprotocol/interfaces/contracts/horizon/IHorizonStaking.sol";
import { IGraphPayments } from "@graphprotocol/interfaces/contracts/horizon/IGraphPayments.sol";
import { IPaymentsEscrow } from "@graphprotocol/interfaces/contracts/horizon/IPaymentsEscrow.sol";

import { IController } from "@graphprotocol/interfaces/contracts/contracts/governance/IController.sol";
import { IEpochManager } from "@graphprotocol/interfaces/contracts/contracts/epochs/IEpochManager.sol";
import { IRewardsManager } from "@graphprotocol/interfaces/contracts/contracts/rewards/IRewardsManager.sol";
import { ITokenGateway } from "@graphprotocol/interfaces/contracts/contracts/arbitrum/ITokenGateway.sol";
import { IGraphProxyAdmin } from "@graphprotocol/interfaces/contracts/contracts/upgrades/IGraphProxyAdmin.sol";

import { ICuration } from "@graphprotocol/interfaces/contracts/contracts/curation/ICuration.sol";

/**
 * @title GraphDirectory contract
 * @author Edge & Node
 * @notice This contract is meant to be inherited by other contracts that
 * need to keep track of the addresses in Graph Horizon contracts.
 * It fetches the addresses from the Controller supplied during construction,
 * and uses immutable variables to minimize gas costs.
 */
abstract contract GraphDirectory {
    // -- Graph Horizon contracts --

    /// @notice The Graph Token contract address
    IGraphToken private immutable GRAPH_TOKEN;

    /// @notice The Horizon Staking contract address
    IHorizonStaking private immutable GRAPH_STAKING;

    /// @notice The Graph Payments contract address
    IGraphPayments private immutable GRAPH_PAYMENTS;

    /// @notice The Payments Escrow contract address
    IPaymentsEscrow private immutable GRAPH_PAYMENTS_ESCROW;

    // -- Graph periphery contracts --

    /// @notice The Graph Controller contract address
    IController private immutable GRAPH_CONTROLLER;

    /// @notice The Epoch Manager contract address
    IEpochManager private immutable GRAPH_EPOCH_MANAGER;

    /// @notice The Rewards Manager contract address
    IRewardsManager private immutable GRAPH_REWARDS_MANAGER;

    /// @notice The Token Gateway contract address
    ITokenGateway private immutable GRAPH_TOKEN_GATEWAY;

    /// @notice The Graph Proxy Admin contract address
    IGraphProxyAdmin private immutable GRAPH_PROXY_ADMIN;

    // -- Legacy Graph contracts --
    // These are required for backwards compatibility on HorizonStakingExtension
    // TRANSITION PERIOD: remove these once HorizonStakingExtension is removed

    /// @notice The Curation contract address
    ICuration private immutable GRAPH_CURATION;

    /**
     * @notice Emitted when the GraphDirectory is initialized
     * @param graphToken The Graph Token contract address
     * @param graphStaking The Horizon Staking contract address
     * @param graphPayments The Graph Payments contract address
     * @param graphEscrow The Payments Escrow contract address
     * @param graphController The Graph Controller contract address
     * @param graphEpochManager The Epoch Manager contract address
     * @param graphRewardsManager The Rewards Manager contract address
     * @param graphTokenGateway The Token Gateway contract address
     * @param graphProxyAdmin The Graph Proxy Admin contract address
     * @param graphCuration The Curation contract address
     */
    event GraphDirectoryInitialized(
        address indexed graphToken,
        address indexed graphStaking,
        address graphPayments,
        address graphEscrow,
        address indexed graphController,
        address graphEpochManager,
        address graphRewardsManager,
        address graphTokenGateway,
        address graphProxyAdmin,
        address graphCuration
    );

    /**
     * @notice Thrown when either the controller is the zero address or a contract address is not found
     * on the controller
     * @param contractName The name of the contract that was not found, or the controller
     */
    error GraphDirectoryInvalidZeroAddress(bytes contractName);

    /**
     * @notice Constructor for the GraphDirectory contract
     * @dev Requirements:
     * - `controller` cannot be zero address
     *
     * Emits a {GraphDirectoryInitialized} event
     *
     * @param controller The address of the Graph Controller contract.
     */
    constructor(address controller) {
        require(controller != address(0), GraphDirectoryInvalidZeroAddress("Controller"));

        GRAPH_CONTROLLER = IController(controller);
        GRAPH_TOKEN = IGraphToken(_getContractFromController("GraphToken"));
        GRAPH_STAKING = IHorizonStaking(_getContractFromController("Staking"));
        GRAPH_PAYMENTS = IGraphPayments(_getContractFromController("GraphPayments"));
        GRAPH_PAYMENTS_ESCROW = IPaymentsEscrow(_getContractFromController("PaymentsEscrow"));
        GRAPH_EPOCH_MANAGER = IEpochManager(_getContractFromController("EpochManager"));
        GRAPH_REWARDS_MANAGER = IRewardsManager(_getContractFromController("RewardsManager"));
        GRAPH_TOKEN_GATEWAY = ITokenGateway(_getContractFromController("GraphTokenGateway"));
        GRAPH_PROXY_ADMIN = IGraphProxyAdmin(_getContractFromController("GraphProxyAdmin"));
        GRAPH_CURATION = ICuration(_getContractFromController("Curation"));

        emit GraphDirectoryInitialized(
            address(GRAPH_TOKEN),
            address(GRAPH_STAKING),
            address(GRAPH_PAYMENTS),
            address(GRAPH_PAYMENTS_ESCROW),
            address(GRAPH_CONTROLLER),
            address(GRAPH_EPOCH_MANAGER),
            address(GRAPH_REWARDS_MANAGER),
            address(GRAPH_TOKEN_GATEWAY),
            address(GRAPH_PROXY_ADMIN),
            address(GRAPH_CURATION)
        );
    }

    /**
     * @notice Get the Graph Token contract
     * @return The Graph Token contract
     */
    function _graphToken() internal view returns (IGraphToken) {
        return GRAPH_TOKEN;
    }

    /**
     * @notice Get the Horizon Staking contract
     * @return The Horizon Staking contract
     */
    function _graphStaking() internal view returns (IHorizonStaking) {
        return GRAPH_STAKING;
    }

    /**
     * @notice Get the Graph Payments contract
     * @return The Graph Payments contract
     */
    function _graphPayments() internal view returns (IGraphPayments) {
        return GRAPH_PAYMENTS;
    }

    /**
     * @notice Get the Payments Escrow contract
     * @return The Payments Escrow contract
     */
    function _graphPaymentsEscrow() internal view returns (IPaymentsEscrow) {
        return GRAPH_PAYMENTS_ESCROW;
    }

    /**
     * @notice Get the Graph Controller contract
     * @return The Graph Controller contract
     */
    function _graphController() internal view returns (IController) {
        return GRAPH_CONTROLLER;
    }

    /**
     * @notice Get the Epoch Manager contract
     * @return The Epoch Manager contract
     */
    function _graphEpochManager() internal view returns (IEpochManager) {
        return GRAPH_EPOCH_MANAGER;
    }

    /**
     * @notice Get the Rewards Manager contract
     * @return The Rewards Manager contract address
     */
    function _graphRewardsManager() internal view returns (IRewardsManager) {
        return GRAPH_REWARDS_MANAGER;
    }

    /**
     * @notice Get the Graph Token Gateway contract
     * @return The Graph Token Gateway contract
     */
    function _graphTokenGateway() internal view returns (ITokenGateway) {
        return GRAPH_TOKEN_GATEWAY;
    }

    /**
     * @notice Get the Graph Proxy Admin contract
     * @return The Graph Proxy Admin contract
     */
    function _graphProxyAdmin() internal view returns (IGraphProxyAdmin) {
        return GRAPH_PROXY_ADMIN;
    }

    /**
     * @notice Get the Curation contract
     * @return The Curation contract
     */
    function _graphCuration() internal view returns (ICuration) {
        return GRAPH_CURATION;
    }

    /**
     * @notice Get a contract address from the controller
     * @dev Requirements:
     * - The `_contractName` must be registered in the controller
     * @param _contractName The name of the contract to fetch from the controller
     * @return The address of the contract
     */
    function _getContractFromController(bytes memory _contractName) private view returns (address) {
        address contractAddress = GRAPH_CONTROLLER.getContractProxy(keccak256(_contractName));
        require(contractAddress != address(0), GraphDirectoryInvalidZeroAddress(_contractName));
        return contractAddress;
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable named-parameters-mapping

pragma solidity ^0.7.6;
pragma abicoder v2;

import { IL2GNS } from "@graphprotocol/interfaces/contracts/contracts/l2/discovery/IL2GNS.sol";

/**
 * @title L2GNSV1Storage
 * @author Edge & Node
 * @notice This contract holds all the L2-specific storage variables for the L2GNS contract, version 1
 */
abstract contract L2GNSV1Storage {
    /// @notice Data for subgraph transfer from L1 to L2
    mapping(uint256 => IL2GNS.SubgraphL2TransferData) public subgraphL2TransferData;
    /// @dev Storage gap to keep storage slots fixed in future versions
    uint256[50] private __gap;
}

// SPDX-License-Identifier: GPL-2.0-or-later
// solhint-disable one-contract-per-file

pragma solidity ^0.7.6;
pragma abicoder v2;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable named-parameters-mapping

import { Initializable } from "@openzeppelin/contracts-upgradeable/proxy/Initializable.sol";
import { Managed } from "../governance/Managed.sol";

import { IEthereumDIDRegistry } from "@graphprotocol/interfaces/contracts/contracts/discovery/erc1056/IEthereumDIDRegistry.sol";
import { IGNS } from "@graphprotocol/interfaces/contracts/contracts/discovery/IGNS.sol";
import { ISubgraphNFT } from "@graphprotocol/interfaces/contracts/contracts/discovery/ISubgraphNFT.sol";

/**
 * @title GNSV1Storage
 * @author Edge & Node
 * @notice This contract holds all the storage variables for the GNS contract, version 1
 */
abstract contract GNSV1Storage is Managed {
    // -- State --

    /// @notice Percentage of curation tax that must be paid by the owner, in parts per million.
    uint32 public ownerTaxPercentage;

    /// @dev [DEPRECATED] Bonding curve formula.
    address private __DEPRECATED_bondingCurve; // solhint-disable-line var-name-mixedcase

    /// @dev Stores what subgraph deployment a particular legacy subgraph targets.
    /// A subgraph is defined by (graphAccountID, subgraphNumber).
    /// A subgraph can target one subgraph deployment (bytes32 hash).
    /// (graphAccountID, subgraphNumber) => subgraphDeploymentID
    mapping(address => mapping(uint256 => bytes32)) internal legacySubgraphs;

    /// @notice Every time an account creates a subgraph it increases a per-account sequence ID.
    /// account => seqID
    mapping(address => uint256) public nextAccountSeqID;

    /// @notice Stores all the signal deposited on a legacy subgraph.
    /// (graphAccountID, subgraphNumber) => SubgraphData
    mapping(address => mapping(uint256 => IGNS.SubgraphData)) public legacySubgraphData;

    /// @dev [DEPRECATED] ERC-1056 contract reference.
    ///  This contract was used for managing identities.
    IEthereumDIDRegistry private __DEPRECATED_erc1056Registry; // solhint-disable-line var-name-mixedcase
}

/**
 * @title GNSV2Storage
 * @author Edge & Node
 * @notice This contract holds all the storage variables for the GNS contract, version 2
 */
abstract contract GNSV2Storage is GNSV1Storage {
    /// @notice Stores the account and seqID for a legacy subgraph that has been migrated.
    /// Use it whenever a legacy (v1) subgraph NFT was claimed to maintain compatibility.
    /// Keep a reference from subgraphID => (graphAccount, subgraphNumber)
    mapping(uint256 => IGNS.LegacySubgraphKey) public legacySubgraphKeys;

    /// @notice Store data for all NFT-based (v2) subgraphs.
    /// subgraphID => SubgraphData
    mapping(uint256 => IGNS.SubgraphData) public subgraphs;

    /// @notice Contract that represents subgraph ownership through an NFT
    ISubgraphNFT public subgraphNFT;
}

/**
 * @title GNSV3Storage
 * @author Edge & Node
 * @notice This contract holds all the storage variables for the base GNS contract, version 3.
 * @dev Note that this is the first version that includes a storage gap - if adding
 * future versions, make sure to move the gap to the new version and
 * reduce the size of the gap accordingly.
 */
abstract contract GNSV3Storage is GNSV2Storage, Initializable {
    /// @notice Address of the counterpart GNS contract (L1GNS/L2GNS)
    address public counterpartGNSAddress;
    /// @dev Gap to allow adding variables in future upgrades (since L1GNS and L2GNS have their own storage as well)
    uint256[50] private __gap;
}

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


## ------------ SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS ------------ 

## ------------ SUPPORTING CONTEXT: EXTERNAL LIBRARIES ------------ 

 ------------ END OF SUPPORTING CONTRACTS AND INTERFACES ------------ 


 ------------ ## DEPLOYMENT SCRIPTS ------------ 

