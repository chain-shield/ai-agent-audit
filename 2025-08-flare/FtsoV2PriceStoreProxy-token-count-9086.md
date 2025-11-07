
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {IGovernanceSettings} from "@flarenetwork/flare-periphery-contracts/flare/IGovernanceSettings.sol";
import {FtsoV2PriceStore} from "./FtsoV2PriceStore.sol";


contract FtsoV2PriceStoreProxy is ERC1967Proxy {
    constructor(
        address _implementationAddress,
        IGovernanceSettings _governanceSettings,
        address _initialGovernance,
        address _addressUpdater,
        uint64 _firstVotingRoundStartTs,
        uint8 _votingEpochDurationSeconds,
        uint8 _ftsoProtocolId
    )
        ERC1967Proxy(_implementationAddress,
            abi.encodeCall(FtsoV2PriceStore.initialize, (
                _governanceSettings, _initialGovernance, _addressUpdater,
                _firstVotingRoundStartTs, _votingEpochDurationSeconds, _ftsoProtocolId))
        )
    {
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;


interface IPriceReader {

    /**
     * Returns the price for the given symbol.
     * @param _symbol The symbol.
     * @return _price The price.
     * @return _timestamp The timestamp of the voting round for which the price was calculated.
     * @return _priceDecimals The price decimals.
     */
    function getPrice(string memory _symbol)
        external view
        returns (uint256 _price, uint256 _timestamp, uint256 _priceDecimals);

    /**
     * Returns the price for the given symbol that was calculated by trusted providers.
     * @param _symbol The symbol.
     * @return _price The price.
     * @return _timestamp The timestamp of the voting round for which the price was calculated.
     * @return _priceDecimals The price decimals.
     */
    function getPriceFromTrustedProviders(string memory _symbol)
        external view
        returns (uint256 _price, uint256 _timestamp, uint256 _priceDecimals);

    /**
     * Returns the price for the given symbol that was calculated by trusted providers.
     * @param _symbol The symbol.
     * @return _price The price.
     * @return _timestamp The timestamp of the voting round for which the price was calculated.
     * @return _priceDecimals The price decimals.
     * @return _numberOfSubmits The number of submits that were used to calculate the price.
     */
    function getPriceFromTrustedProvidersWithQuality(string memory _symbol)
        external view
        returns (uint256 _price, uint256 _timestamp, uint256 _priceDecimals, uint8 _numberOfSubmits);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {MerkleProof} from "@openzeppelin/contracts/utils/cryptography/MerkleProof.sol";
import {IRelay} from "@flarenetwork/flare-periphery-contracts/flare/IRelay.sol";
import {GovernedUUPSProxyImplementation} from "../../governance/implementation/GovernedUUPSProxyImplementation.sol";
import {AddressUpdatable} from "../../flareSmartContracts/implementation/AddressUpdatable.sol";
import {IPriceReader} from "../../ftso/interfaces/IPriceReader.sol";
import {IPricePublisher} from "../interfaces/IPricePublisher.sol";
import {IGovernanceSettings} from "@flarenetwork/flare-periphery-contracts/flare/IGovernanceSettings.sol";


contract FtsoV2PriceStore is
    GovernedUUPSProxyImplementation,
    IPriceReader,
    IPricePublisher,
    IERC165,
    AddressUpdatable
{
    using MerkleProof for bytes32[];

    uint256 internal constant MAX_BIPS = 1e4;

    struct PriceStore {
        uint32 votingRoundId;
        uint32 value;
        int8 decimals;

        uint32 trustedVotingRoundId;
        uint32 trustedValue;
        int8 trustedDecimals;
        uint8 numberOfSubmits;
    }

    error InvalidStartTime();
    error VotingEpochDurationTooShort();
    error WrongNumberOfProofs();
    error PricesAlreadyPublished();
    error SubmissionWindowNotClosed();
    error VotingRoundIdMismatch();
    error FeedIdMismatch();
    error ValueMustBeNonNegative();
    error MerkleProofInvalid();
    error OnlyTrustedProvider();
    error AllPricesMustBeProvided();
    error SubmissionWindowClosed();
    error AlreadySubmitted();
    error DecimalsMismatch();
    error LengthMismatch();
    error MaxSpreadTooBig();
    error TooManyTrustedProviders();
    error ThresholdTooHigh();
    error SymbolNotSupported();

    /// Timestamp when the first voting epoch started, in seconds since UNIX epoch.
    uint64 public firstVotingRoundStartTs;
    /// Duration of voting epochs, in seconds.
    uint64 public votingEpochDurationSeconds;
    /// Duration of a window for submitting trusted prices, in seconds.
    uint64 public submitTrustedPricesWindowSeconds;
    /// The FTSO protocol id.
    uint8 public ftsoProtocolId;

    /// The list of required feed ids to be published.
    bytes21[] internal feedIds;
    /// Mapping from symbol to feed id - used for price lookups (backwards compatibility).
    mapping(string symbol => bytes21 feedId) internal symbolToFeedId;
    /// Mapping from feed id to symbol - used for list of supported symbols.
    mapping(bytes21 feedId => string symbol) internal feedIdToSymbol;
    /// Mapping from feed id to price store which holds the latest published FTSO scaling price and trusted price.
    mapping(bytes21 feedId => PriceStore) internal latestPrices;
    /// Mapping from feed id to submitted trusted prices for the given voting round.
    mapping(bytes21 feedId => mapping (uint32 votingRoundId => bytes)) internal submittedTrustedPrices;
    /// Mapping from trusted provider to the last submitted voting epoch id.
    mapping(address trustedProvider => uint256 lastVotingEpochId) internal lastVotingEpochIdByProvider;

    /// The list of trusted providers.
    address[] internal trustedProviders;
    mapping(address trustedProvider => bool isTrustedProvider) internal trustedProvidersMap;
    /// Trusted providers threshold for calculating the median price.
    uint8 public trustedProvidersThreshold;
    /// The maximum spread between the median price and the nearby trusted prices in BIPS in order to update the price.
    uint16 public maxSpreadBIPS;

    /// The Relay contract.
    IRelay public relay;
    /// The last published voting round id.
    uint32 public lastPublishedVotingRoundId;

    event PricesPublished(uint32 indexed votingRoundId);

    constructor()
        GovernedUUPSProxyImplementation()   // marks as initialized
        AddressUpdatable(address(0))
    {}

    function initialize(
        IGovernanceSettings _governanceSettings,
        address _initialGovernance,
        address _addressUpdater,
        uint64 _firstVotingRoundStartTs,
        uint8 _votingEpochDurationSeconds,
        uint8 _ftsoProtocolId
    )
        external
    {
        require(_firstVotingRoundStartTs + _votingEpochDurationSeconds <= block.timestamp, InvalidStartTime());
        require(_votingEpochDurationSeconds > 1, VotingEpochDurationTooShort()); // 90 s

        initialise(_governanceSettings, _initialGovernance);    // also marks as initialized
        setAddressUpdaterValue(_addressUpdater);
        firstVotingRoundStartTs = _firstVotingRoundStartTs;
        votingEpochDurationSeconds = _votingEpochDurationSeconds;
        submitTrustedPricesWindowSeconds = _votingEpochDurationSeconds / 2; // 45 s
        ftsoProtocolId = _ftsoProtocolId;
        lastPublishedVotingRoundId = _getPreviousVotingEpochId();
    }

    /**
     * @inheritdoc IPricePublisher
     */
    function publishPrices(FeedWithProof[] calldata _proofs) external {
        uint32 votingRoundId = 0;
        require(_proofs.length == feedIds.length, WrongNumberOfProofs());
        for (uint256 i = 0; i < _proofs.length; i++) {
            FeedWithProof calldata proof = _proofs[i];
            Feed calldata feed = proof.body;
            if (i == 0) {
                votingRoundId = feed.votingRoundId;
                require(votingRoundId > lastPublishedVotingRoundId, PricesAlreadyPublished());
                require(_getEndTimestamp(votingRoundId) + submitTrustedPricesWindowSeconds <= block.timestamp,
                    SubmissionWindowNotClosed());
                // update last published voting round id
                lastPublishedVotingRoundId = votingRoundId;
                // emit event
                emit PricesPublished(votingRoundId);
            } else {
                require(feed.votingRoundId == votingRoundId, VotingRoundIdMismatch());
            }
            bytes21 feedId = feedIds[i];
            require(feed.id == feedId, FeedIdMismatch());
            require(feed.value >= 0, ValueMustBeNonNegative());

            bytes32 feedHash = keccak256(abi.encode(feed));
            bytes32 merkleRoot = relay.merkleRoots(ftsoProtocolId, votingRoundId);
            require(proof.proof.verifyCalldata(merkleRoot, feedHash), MerkleProofInvalid());

            PriceStore storage priceStore = latestPrices[feedId];
            priceStore.votingRoundId = feed.votingRoundId;
            priceStore.value = uint32(feed.value);
            priceStore.decimals = feed.decimals;

            // calculate trusted prices for the same voting round
            bytes memory trustedPrices = submittedTrustedPrices[feedId][votingRoundId];
            if (trustedPrices.length > 0 && trustedPrices.length >= 4 * trustedProvidersThreshold) {
                // calculate median price
                (uint256 medianPrice, bool priceOk) = _calculateMedian(trustedPrices);
                if (priceOk) {
                    // store the median price
                    priceStore.trustedVotingRoundId = votingRoundId;
                    priceStore.trustedValue = uint32(medianPrice);
                    priceStore.numberOfSubmits = uint8(trustedPrices.length / 4);
                }
                // delete submitted trusted prices
                delete submittedTrustedPrices[feedId][votingRoundId];
            }
        }
    }

    /**
     * @inheritdoc IPricePublisher
     * @dev The function can be called by trusted providers only.
     */
    function submitTrustedPrices(uint32 _votingRoundId, TrustedProviderFeed[] calldata _feeds) external {
        require(trustedProvidersMap[msg.sender], OnlyTrustedProvider());
        require(_feeds.length == feedIds.length, AllPricesMustBeProvided());
        uint32 previousVotingEpochId = _getPreviousVotingEpochId();
        require(_votingRoundId == previousVotingEpochId, VotingRoundIdMismatch());
        // end of previous voting epoch = start of current voting epoch
        uint256 startTimestamp = _getEndTimestamp(previousVotingEpochId);
        uint256 endTimestamp = startTimestamp + submitTrustedPricesWindowSeconds;
        require(block.timestamp >= startTimestamp && block.timestamp < endTimestamp, SubmissionWindowClosed());
        require(lastVotingEpochIdByProvider[msg.sender] < previousVotingEpochId, AlreadySubmitted());
        // mark the trusted provider submission
        lastVotingEpochIdByProvider[msg.sender] = previousVotingEpochId;

        for (uint256 i = 0; i < _feeds.length; i++) {
            TrustedProviderFeed calldata feed = _feeds[i];
            bytes21 feedId = feedIds[i];
            require(feed.id == feedId, FeedIdMismatch());
            require(feed.decimals == latestPrices[feedId].trustedDecimals, DecimalsMismatch());
            submittedTrustedPrices[feedId][previousVotingEpochId] =
                bytes.concat(submittedTrustedPrices[feedId][previousVotingEpochId], bytes4(feed.value));
        }
    }

    /**
     * Updates the settings.
     * @param _feedIds The list of feed ids.
     * @param _symbols The list of symbols.
     * @param _trustedDecimals The list of trusted decimals.
     * @param _maxSpreadBIPS The maximum spread between the median price and the nearby trusted prices in BIPS.
     * @dev Can only be called by the governance.
     */
    function updateSettings(
        bytes21[] calldata _feedIds,
        string[] calldata _symbols,
        int8[] calldata _trustedDecimals,
        uint16 _maxSpreadBIPS
    )
        external onlyGovernance
    {
        require(_feedIds.length == _symbols.length && _feedIds.length == _trustedDecimals.length, LengthMismatch());
        require(_maxSpreadBIPS <= MAX_BIPS, MaxSpreadTooBig());
        maxSpreadBIPS = _maxSpreadBIPS;
        feedIds = _feedIds;
        for (uint256 i = 0; i < _feedIds.length; i++) {
            bytes21 feedId = _feedIds[i];
            symbolToFeedId[_symbols[i]] = feedId;
            feedIdToSymbol[feedId] = _symbols[i];
            PriceStore storage latestPrice = latestPrices[feedId];
            if (latestPrice.trustedDecimals != _trustedDecimals[i]) {
                latestPrice.trustedDecimals = _trustedDecimals[i];
                latestPrice.trustedValue = 0;
                latestPrice.trustedVotingRoundId = 0;
                // delete all submitted trusted prices for the symbol
                for (uint32 j = lastPublishedVotingRoundId + 1; j <= _getPreviousVotingEpochId(); j++) {
                    delete submittedTrustedPrices[feedId][j];
                }
            }
        }
    }

    /**
     * Sets the trusted providers.
     * @param _trustedProviders The list of trusted providers.
     * @param _trustedProvidersThreshold The trusted providers threshold for calculating the median price.
     * @dev Can only be called by the governance.
     */
    function setTrustedProviders(
        address[] calldata _trustedProviders,
        uint8 _trustedProvidersThreshold
    )
        external onlyGovernance
    {
        require(_trustedProviders.length < 2**8, TooManyTrustedProviders());
        require(_trustedProviders.length >= _trustedProvidersThreshold, ThresholdTooHigh());
        trustedProvidersThreshold = _trustedProvidersThreshold;
        // reset all trusted providers
        for (uint256 i = 0; i < trustedProviders.length; i++) {
            trustedProvidersMap[trustedProviders[i]] = false;
        }
        // set new trusted providers
        trustedProviders = _trustedProviders;
        for (uint256 i = 0; i < _trustedProviders.length; i++) {
            trustedProvidersMap[_trustedProviders[i]] = true;
        }
    }

    /**
     * @inheritdoc IPriceReader
     */
    function getPrice(string memory _symbol)
        external view
        returns (uint256 _price, uint256 _timestamp, uint256 _priceDecimals)
    {
        bytes21 feedId = symbolToFeedId[_symbol];
        require(feedId != bytes21(0), SymbolNotSupported());
        PriceStore storage feed = latestPrices[feedId];
        _price = feed.value;
        _timestamp = _getEndTimestamp(feed.votingRoundId);
        int256 decimals = feed.decimals; // int8
        if (decimals < 0) {
            _priceDecimals = 0;
            _price *= 10 ** uint256(-decimals);
        } else {
            _priceDecimals = uint256(decimals);
        }
    }

    /**
     * @inheritdoc IPriceReader
     */
    function getPriceFromTrustedProviders(string memory _symbol)
        external view
        returns (uint256 _price, uint256 _timestamp, uint256 _priceDecimals)
    {
        bytes21 feedId = symbolToFeedId[_symbol];
        require(feedId != bytes21(0), SymbolNotSupported());
        PriceStore storage feed = latestPrices[feedId];
        (_price, _timestamp, _priceDecimals) = _getPriceFromTrustedProviders(feed);
    }

    /**
     * @inheritdoc IPriceReader
     */
    function getPriceFromTrustedProvidersWithQuality(string memory _symbol)
        external view
        returns (uint256 _price, uint256 _timestamp, uint256 _priceDecimals, uint8 _numberOfSubmits)
    {
        bytes21 feedId = symbolToFeedId[_symbol];
        require(feedId != bytes21(0), SymbolNotSupported());
        PriceStore storage feed = latestPrices[feedId];
        (_price, _timestamp, _priceDecimals) = _getPriceFromTrustedProviders(feed);
        _numberOfSubmits = feed.numberOfSubmits;
    }

    /**
     * @inheritdoc IPricePublisher
     */
    function getFeedIds() external view returns (bytes21[] memory) {
        return feedIds;
    }

    /**
     * @inheritdoc IPricePublisher
     */
    function getFeedIdsWithDecimals() external view returns (bytes21[] memory _feedIds, int8[] memory _decimals) {
        _feedIds = feedIds;
        _decimals = new int8[](_feedIds.length);
        for (uint256 i = 0; i < _feedIds.length; i++) {
            _decimals[i] = latestPrices[_feedIds[i]].trustedDecimals;
        }
    }

    /**
     * @inheritdoc IPricePublisher
     */
    function getSymbols() external view returns (string[] memory _symbols) {
        _symbols = new string[](feedIds.length);
        for (uint256 i = 0; i < feedIds.length; i++) {
            _symbols[i] = feedIdToSymbol[feedIds[i]];
        }
    }

    /**
     * @inheritdoc IPricePublisher
     */
    function getFeedId(string memory _symbol) external view returns (bytes21) {
        return symbolToFeedId[_symbol];
    }

    /**
     * @inheritdoc IPricePublisher
     */
    function getTrustedProviders() external view returns (address[] memory) {
        return trustedProviders;
    }

    /**
     * @notice virtual method that a contract extending AddressUpdatable must implement
     */
    function _updateContractAddresses(
        bytes32[] memory _contractNameHashes,
        address[] memory _contractAddresses
    )
        internal override
    {
        relay = IRelay(_getContractAddress(_contractNameHashes, _contractAddresses, "Relay"));
    }

    /**
     * Returns the previous voting epoch id.
     */
    function _getPreviousVotingEpochId() internal view returns(uint32) {
        return uint32((block.timestamp - firstVotingRoundStartTs) / votingEpochDurationSeconds) - 1;
    }

    /**
     * Returns the end timestamp for the given voting epoch id.
     */
    function _getEndTimestamp(uint256 _votingEpochId) internal view returns(uint256) {
        return firstVotingRoundStartTs + (_votingEpochId + 1) * votingEpochDurationSeconds;
    }

    /**
     * Returns price data from trusted providers.
     */
    function _getPriceFromTrustedProviders(PriceStore storage _feed)
        internal view
        returns (uint256 _price, uint256 _timestamp, uint256 _priceDecimals)
    {
        _price = _feed.trustedValue;
        _timestamp = _getEndTimestamp(_feed.trustedVotingRoundId);
        int256 decimals = _feed.trustedDecimals; // int8
        if (decimals < 0) {
            _priceDecimals = 0;
            _price *= 10 ** uint256(-decimals);
        } else {
            _priceDecimals = uint256(decimals);
        }
    }

    /**
     * @notice Calculates the simple median price (using insertion sort) - sorts original array
     * @param _prices positional array of prices to be sorted
     * @return _medianPrice median price
     * @return _priceOk true if the median price is within the spread
     */
    function _calculateMedian(bytes memory _prices) internal view returns (uint256 _medianPrice, bool _priceOk) {
        uint256 length = _prices.length;
        assert(length > 0 && length % 4 == 0);
        length /= 4;
        uint256[] memory prices = new uint256[](length);
        for (uint256 i = 0; i < length; i++) {
            bytes memory price = new bytes(4);
            for (uint256 j = 0; j < 4; j++) {
                price[j] = _prices[i * 4 + j];
            }
            prices[i] = uint32(bytes4(price));
        }

        for (uint256 i = 1; i < length; i++) {
            // price to sort next
            uint256 currentPrice = prices[i];

            // shift bigger prices right
            uint256 j = i;
            while (j > 0 && prices[j - 1] > currentPrice) {
                prices[j] = prices[j - 1];
                j--; // no underflow
            }
            // insert
            prices[j] = currentPrice;
        }

        uint256 spread = 0;
        uint256 middleIndex = length / 2;
        if (length % 2 == 1) {
            _medianPrice = prices[middleIndex];
            if (length >= 3) {
                spread = (prices[middleIndex + 1] - prices[middleIndex - 1]) / 2;
            }
        } else {
            // if median is "in the middle", take the average price of the two consecutive prices
            _medianPrice = (prices[middleIndex - 1] + prices[middleIndex]) / 2;
            spread = prices[middleIndex] - prices[middleIndex - 1];
        }
        // check if spread is within the limit
        _priceOk = spread <= maxSpreadBIPS * _medianPrice / MAX_BIPS; // no overflow
    }

    /**
     * Implementation of ERC-165 interface.
     */
    function supportsInterface(bytes4 _interfaceId)
        external pure override
        returns (bool)
    {
        return _interfaceId == type(IERC165).interfaceId
            || _interfaceId == type(IPriceReader).interfaceId
            || _interfaceId == type(IPricePublisher).interfaceId;
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IIAddressUpdatable}
    from "@flarenetwork/flare-periphery-contracts/flare/addressUpdater/interfaces/IIAddressUpdatable.sol";
import {IAddressUpdatable} from "../interfaces/IAddressUpdatable.sol";


abstract contract AddressUpdatable is IAddressUpdatable, IIAddressUpdatable {

    // https://docs.soliditylang.org/en/v0.8.7/contracts.html#constant-and-immutable-state-variables
    // No storage slot is allocated
    bytes32 internal constant ADDRESS_STORAGE_POSITION =
        keccak256("flare.diamond.AddressUpdatable.ADDRESS_STORAGE_POSITION");

    modifier onlyAddressUpdater() {
        require (msg.sender == getAddressUpdater(), OnlyAddressUpdater());
        _;
    }

    constructor(address _addressUpdater) {
        setAddressUpdaterValue(_addressUpdater);
    }

    function getAddressUpdater() public view returns (address _addressUpdater) {
        // Only direct constants are allowed in inline assembly, so we assign it here
        bytes32 position = ADDRESS_STORAGE_POSITION;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            _addressUpdater := sload(position)
        }
    }

    /**
     * @notice external method called from AddressUpdater only
     */
    function updateContractAddresses(
        bytes32[] memory _contractNameHashes,
        address[] memory _contractAddresses
    )
        external override
        onlyAddressUpdater
    {
        // update addressUpdater address
        setAddressUpdaterValue(_getContractAddress(_contractNameHashes, _contractAddresses, "AddressUpdater"));
        // update all other addresses
        _updateContractAddresses(_contractNameHashes, _contractAddresses);
    }

    /**
     * @notice virtual method that a contract extending AddressUpdatable must implement
     */
    function _updateContractAddresses(
        bytes32[] memory _contractNameHashes,
        address[] memory _contractAddresses
    ) internal virtual;

    /**
     * @notice helper method to get contract address
     * @dev it reverts if contract name does not exist
     */
    function _getContractAddress(
        bytes32[] memory _nameHashes,
        address[] memory _addresses,
        string memory _nameToFind
    )
        internal pure
        returns(address)
    {
        bytes32 nameHash = keccak256(abi.encode(_nameToFind));
        address a = address(0);
        for (uint256 i = 0; i < _nameHashes.length; i++) {
            if (nameHash == _nameHashes[i]) {
                a = _addresses[i];
                break;
            }
        }
        require(a != address(0), AUAddressZero());
        return a;
    }

    function setAddressUpdaterValue(address _addressUpdater) internal {
        // Only direct constants are allowed in inline assembly, so we assign it here
        bytes32 position = ADDRESS_STORAGE_POSITION;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            sstore(position, _addressUpdater)
        }
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import { UUPSUpgradeable } from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import { GovernedProxyImplementation } from "./GovernedProxyImplementation.sol";
import { IUUPSUpgradeable } from "../../utils/interfaces/IUUPSUpgradeable.sol";

/**
 * Implementation of UUPS proxy that uses Flare governance with timelock.
 **/
abstract contract GovernedUUPSProxyImplementation is
    UUPSUpgradeable,
    GovernedProxyImplementation,
    IUUPSUpgradeable
{
    constructor()
        GovernedProxyImplementation()
    {}

    /**
     * See UUPSUpgradeable.upgradeTo
     */
    function upgradeTo(address newImplementation)
        public override (IUUPSUpgradeable, UUPSUpgradeable)
        onlyGovernance
        onlyProxy
    {
        _upgradeToAndCallUUPS(newImplementation, new bytes(0), false);
    }

    /**
     * See UUPSUpgradeable.upgradeToAndCall
     */
    function upgradeToAndCall(address newImplementation, bytes memory data)
        public payable override (IUUPSUpgradeable, UUPSUpgradeable)
        onlyGovernance
        onlyProxy
    {
        _upgradeToAndCallUUPS(newImplementation, data, true);
    }

    /**
     * Unused. Only present to satisfy UUPSUpgradeable requirement.
     * The real check is in onlyGovernance modifier on upgradeTo and upgradeToAndCall.
     */
    function _authorizeUpgrade(address  /* _newImplementation */)
        internal pure override
    {
        assert(false);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;


interface IPricePublisher {

    /// The FTSO feed struct.
    struct Feed {
        uint32 votingRoundId;
        bytes21 id;
        int32 value;
        uint16 turnoutBIPS;
        int8 decimals;
    }

    /// The FTSO feed with proof struct.
    struct FeedWithProof {
        bytes32[] proof;
        Feed body;
    }

    /// The trusted provider feed struct.
    struct TrustedProviderFeed {
        bytes21 id;
        uint32 value;
        int8 decimals;
    }

    /**
     * Publishes the FTSO scaling prices and calculates the median of the trusted prices.
     * It must be called for all feeds ordered as in the `getFeedIds()` list.
     * @param _proofs The list of FTSO scaling feeds with Merkle proofs.
     */
    function publishPrices(FeedWithProof[] calldata _proofs) external;

    /**
     * Submits trusted prices for the voting round id.
     * It must be called for all feeds ordered as in the `getFeedIds()` list.
     * @param _votingRoundId The previous voting round id.
     * @param _feeds The list of trusted provider feeds.
     */
    function submitTrustedPrices(uint32 _votingRoundId, TrustedProviderFeed[] calldata _feeds) external;

    /**
     * Returns the list of required feed ids.
     * @return The list of feed ids.
     */
    function getFeedIds() external view returns (bytes21[] memory);

    /**
     * Returns the list of required feed ids with decimals (for the trusted providers).
     * @return _feedIds The list of feed ids.
     * @return _decimals The list of feed decimals.
     */
    function getFeedIdsWithDecimals() external view returns (bytes21[] memory _feedIds, int8[] memory _decimals);

    /**
     * Returns the list of supported symbols.
     * @return _symbols The list of symbols.
     */
    function getSymbols() external view returns (string[] memory _symbols);

    /**
     * Returns the feed id for the given symbol.
     * @param _symbol The symbol.
     * @return The feed id.
     */
    function getFeedId(string memory _symbol) external view returns (bytes21);

    /**
     * Returns the trusted providers list.
     * @return The list of trusted providers.
     */
    function getTrustedProviders() external view returns (address[] memory);
}


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS
// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import "./RandomNumberV2Interface.sol";

/**
 * Relay interface.
 */
interface IRelay is RandomNumberV2Interface {
    struct FeeConfig {
        uint8 protocolId; // Protocol id for which the fee is set
        uint256 feeInWei; // Fee in wei
    }

    struct RelayInitialConfig {
        uint32 initialRewardEpochId; // The initial reward epoch id.
        uint32 startingVotingRoundIdForInitialRewardEpochId; // The starting voting round id for the initial
        // reward epoch.
        bytes32 initialSigningPolicyHash; // The initial signing policy hash.
        uint8 randomNumberProtocolId; // The protocol id of the random number protocol.
        uint32 firstVotingRoundStartTs; // The timestamp of the first voting round start.
        uint8 votingEpochDurationSeconds; // The duration of a voting epoch in seconds.
        uint32 firstRewardEpochStartVotingRoundId; // The start voting round id of the first reward epoch.
        uint16 rewardEpochDurationInVotingEpochs; // The duration of a reward epoch in voting epochs.
        uint16 thresholdIncreaseBIPS; // The threshold increase in BIPS for signing with
        // old signing policy.
        uint32 messageFinalizationWindowInRewardEpochs; // The window of reward epochs for finalizing
        // the protocol messages.
        address payable feeCollectionAddress; // Fee collection address
        FeeConfig[] feeConfigs; // Fee configurations
    }

    struct RelayGovernanceConfig {
        bytes32 descriptionHash; // Description hash (should be keccak256("RelayGovernance")
        uint256 chainId; // Chain id on which is the relay is deployed
        FeeConfig[] newFeeConfigs; // Fee configurations
    }

    // Event is emitted when a new signing policy is initialized by the signing policy setter.
    event SigningPolicyInitialized(
        uint24 indexed rewardEpochId, // Reward epoch id
        uint32 startVotingRoundId, // First voting round id of validity.
        // Usually it is the first voting round of reward epoch rewardEpochId.
        // It can be later,
        // if the confirmation of the signing policy on Flare blockchain gets delayed.
        uint16 threshold, // Confirmation threshold (absolute value of noramalised weights).
        uint256 seed, // Random seed.
        address[] voters, // The list of eligible voters in the canonical order.
        uint16[] weights, // The corresponding list of normalised signing weights of eligible voters.
        // Normalisation is done by compressing the weights from 32-byte values to
        // 2 bytes, while approximately keeping the weight relations.
        bytes signingPolicyBytes, // The full signing policy byte encoded.
        uint64 timestamp // Timestamp when this happened
    );

    // Event is emitted when a signing policy is relayed.
    // It contains minimalistic data in order to save gas. Data about the signing policy are
    // extractable from the calldata, assuming prefered usage of direct top-level call to relay().
    event SigningPolicyRelayed(
        uint256 indexed rewardEpochId // Reward epoch id
    );

    // Event is emitted when a protocol message is relayed.
    event ProtocolMessageRelayed(
        uint8 indexed protocolId, // Protocol id
        uint32 indexed votingRoundId, // Voting round id
        bool isSecureRandom, // Secure random flag
        bytes32 merkleRoot // Merkle root of the protocol message
    );

    /**
     * Checks the relay message for sufficient weight of signatures for the _messageHash
     * signed for protocol message Merkle root of the form (1, 0, 0, _messageHash).
     * If the check is successful, reward epoch id of the signing policy is returned.
     * Otherwise the function reverts.
     * @param _relayMessage The relay message.
     * @param _messageHash The hash of the message.
     * @return _rewardEpochId The reward epoch id of the signing policy.
     */
    function verifyCustomSignature(
        bytes calldata _relayMessage,
        bytes32 _messageHash
    ) external returns (uint256 _rewardEpochId);

    /**
     * Checks the relay message for sufficient weight of signatures of the hash of the _config data.
     * If the check is successful, the relay contract is configured with the new _config data, which
     * in particular means that fee configurations are updated.
     * Otherwise the function reverts.
     * @param _relayMessage The relay message.
     * @param _config The new relay configuration.
     */
    function governanceFeeSetup(
        bytes calldata _relayMessage,
        RelayGovernanceConfig calldata _config
    ) external;

    /**
     * Finalization function for new signing policies and protocol messages.
     * It can be used as finalization contract on Flare chain or as relay contract on other EVM chain.
     * Can be called in two modes. It expects calldata that is parsed in a custom manner.
     * Hence the transaction calls should assemble relevant calldata in the 'data' field.
     * Depending on the data provided, the contract operations in essentially two modes:
     * (1) Relaying signing policy. The structure of the calldata is:
     *        function signature (4 bytes) + active signing policy
     *             + 0 (1 byte) + new signing policy,
     *     total of exactly 4423 bytes.
     * (2) Relaying signed message. The structure of the calldata is:
     *        function signature (4 bytes) + signing policy
     *           + signed message (38 bytes) + ECDSA signatures with indices (67 bytes each)
     *     This case splits into two subcases:
     *     - protocolMessageId = 1: Message id must be of the form (protocolMessageId, 0, 0, merkleRoot).
     *       The validity of the signatures of sufficient weight is checked and if
     *       successful, the merkleRoot from the message is returned (32 bytes) and the
     *       reward epoch id of the signing policy as well (additional 3 bytes)
     *     - protocolMessageId > 1: The validity of the signatures of sufficient weight is checked and if
     *       it is valid, the merkleRoot is published for protocolId and votingRoundId.
     * Reverts if relaying is not successful.
     */
    function relay() external returns (bytes memory);

    /**
     * Verifies the leaf (or intermediate node) with the Merkle proof against the Merkle root
     * for given protocol id and voting round id.
     * A fee may need to be paid. It is protocol specific.
     * **NOTE:** Overpayment is not refunded.
     * @param _protocolId The protocol id.
     * @param _votingRoundId The voting round id.
     * @param _leaf The leaf (or intermediate node) to verify.
     * @param _proof The Merkle proof.
     * @return True if the verification is successful.
     */
    function verify(
        uint256 _protocolId,
        uint256 _votingRoundId,
        bytes32 _leaf,
        bytes32[] calldata _proof
    ) external payable returns (bool);

    /**
     * Returns the signing policy hash for given reward epoch id.
     * The function is reverted if signingPolicySetter is set, hence on all
     * deployments where the contract is used as a pure relay.
     * @param _rewardEpochId The reward epoch id.
     * @return _signingPolicyHash The signing policy hash.
     */
    function toSigningPolicyHash(
        uint256 _rewardEpochId
    ) external view returns (bytes32 _signingPolicyHash);

    /**
     * Returns true if there is finalization for a given protocol id and voting round id.
     * @param _protocolId The protocol id.
     * @param _votingRoundId The voting round id.
     */
    function isFinalized(
        uint256 _protocolId,
        uint256 _votingRoundId
    ) external view returns (bool);

    /**
     * Returns the Merkle root for given protocol id and voting round id.
     * The function is reverted if signingPolicySetter is set, hence on all
     * deployments where the contract is used as a pure relay.
     * @param _protocolId The protocol id.
     * @param _votingRoundId The voting round id.
     * @return _merkleRoot The Merkle root.
     */
    function merkleRoots(
        uint256 _protocolId,
        uint256 _votingRoundId
    ) external view returns (bytes32 _merkleRoot);

    /**
     * Returns the start voting round id for given reward epoch id.
     * @param _rewardEpochId The reward epoch id.
     * @return _startingVotingRoundId The start voting round id.
     */
    function startingVotingRoundIds(
        uint256 _rewardEpochId
    ) external view returns (uint256 _startingVotingRoundId);

    /**
     * Returns the voting round id for given timestamp.
     * @param _timestamp The timestamp.
     * @return _votingRoundId The voting round id.
     */
    function getVotingRoundId(
        uint256 _timestamp
    ) external view returns (uint256 _votingRoundId);

    /**
     * Returns last initialized reward epoch data.
     * @return _lastInitializedRewardEpoch Last initialized reward epoch.
     * @return _startingVotingRoundIdForLastInitializedRewardEpoch Starting voting round id for it.
     */
    function lastInitializedRewardEpochData()
        external
        view
        returns (
            uint32 _lastInitializedRewardEpoch,
            uint32 _startingVotingRoundIdForLastInitializedRewardEpoch
        );

    /**
     * Returns fee collection address.
     */
    function feeCollectionAddress() external view returns (address payable);

    /**
     * Returns fee in wei for one verification of a given protocol id.
     * @param _protocolId The protocol id.
     */
    function protocolFeeInWei(
        uint256 _protocolId
    ) external view returns (uint256);
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

/**
 * A special contract that holds Flare governance address.
 * This contract enables updating governance address and timelock only by hard forking the network,
 * meaning only by updating validator code.
 */
interface IGovernanceSettings {
    /**
     * Get the governance account address.
     * The governance address can only be changed by a hardfork.
     */
    function getGovernanceAddress() external view returns (address);

    /**
     * Get the time in seconds that must pass between a governance call and execution.
     * The timelock value can only be changed by a hardfork.
     */
    function getTimelock() external view returns (uint256);

    /**
     * Get the addresses of the accounts that are allowed to execute the timelocked governance calls
     * once the timelock period expires.
     * Executors can be changed without a hardfork, via a normal governance call.
     */
    function getExecutors() external view returns (address[] memory);

    /**
     * Check whether an address is one of the executors.
     */
    function isExecutor(address _address) external view returns (bool);
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import "../../IRelay.sol";

/**
 * Relay internal interface.
 */
interface IIRelay is IRelay {
    struct SigningPolicy {
        uint24 rewardEpochId; // Reward epoch id.
        uint32 startVotingRoundId; // First voting round id of validity.
        // Usually it is the first voting round of reward epoch rID.
        // It can be later,
        // if the confirmation of the signing policy on Flare blockchain gets delayed.
        uint16 threshold; // Confirmation threshold (absolute value of noramalised weights).
        uint256 seed; // Random seed.
        address[] voters; // The list of eligible voters in the canonical order.
        uint16[] weights; // The corresponding list of normalised signing weights of eligible voters.
        // Normalisation is done by compressing the weights from 32-byte values to 2 bytes,
        // while approximately keeping the weight relations.
    }

    /**
     * Sets the signing policy.
     * @param _signingPolicy Signing policy.
     * @return Returns signing policy hash.
     * @dev This method can only be called by the signing policy setter.
     */
    function setSigningPolicy(
        SigningPolicy memory _signingPolicy
    ) external returns (bytes32);
}


## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

