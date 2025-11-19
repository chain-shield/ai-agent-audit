
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {IGovernanceSettings} from "@flarenetwork/flare-periphery-contracts/flare/IGovernanceSettings.sol";
import {CoreVaultManager} from "./CoreVaultManager.sol";


contract CoreVaultManagerProxy is ERC1967Proxy {
    constructor(
        address _implementationAddress,
        IGovernanceSettings _governanceSettings,
        address _initialGovernance,
        address _addressUpdater,
        address _assetManager,
        bytes32 _chainId,
        string memory _custodianAddress,
        string memory _coreVaultAddress,
        uint256 _nextSequenceNumber
    )
        ERC1967Proxy(_implementationAddress,
            abi.encodeCall(
                CoreVaultManager.initialize,
                (_governanceSettings, _initialGovernance, _addressUpdater,
                _assetManager, _chainId, _custodianAddress, _coreVaultAddress, _nextSequenceNumber)
            )
        )
    {
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {EnumerableSet} from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import {Math} from "@openzeppelin/contracts/utils/math/Math.sol";
import {GovernedUUPSProxyImplementation} from "../../governance/implementation/GovernedUUPSProxyImplementation.sol";
import {AddressUpdatable} from "../../flareSmartContracts/implementation/AddressUpdatable.sol";
import {IICoreVaultManager} from "../interfaces/IICoreVaultManager.sol";
import {IFdcVerification, IPayment} from "@flarenetwork/flare-periphery-contracts/flare/IFdcVerification.sol";
import {IGovernanceSettings} from "@flarenetwork/flare-periphery-contracts/flare/IGovernanceSettings.sol";
import {GovernedBase} from "../../governance/implementation/GovernedBase.sol";
import {IIAddressUpdatable}
    from "@flarenetwork/flare-periphery-contracts/flare/addressUpdater/interfaces/IIAddressUpdatable.sol";

// import is needed for @inheritdoc
import {ICoreVaultManager} from "../../userInterfaces/ICoreVaultManager.sol"; // solhint-disable-line no-unused-import

//solhint-disable-next-line max-states-count
contract CoreVaultManager is
    GovernedUUPSProxyImplementation,
    AddressUpdatable,
    IICoreVaultManager,
    IERC165
{
    using EnumerableSet for EnumerableSet.AddressSet;
    using EnumerableSet for EnumerableSet.Bytes32Set;

    /// asset manager address
    address public assetManager;
    /// chain id
    bytes32 public chainId;
    /// custodian address
    string public custodianAddress;
    /// core vault address hash
    bytes32 public coreVaultAddressHash;
    /// core vault address
    string public coreVaultAddress;
    /// next sequence number for core vault instructions
    uint256 public nextSequenceNumber;

    /// FDC verification contract
    IFdcVerification public fdcVerification;
    /// confirmed payments
    mapping(bytes32 transactionId => bool) public confirmedPayments;

    EnumerableSet.Bytes32Set private preimageHashes;
    Escrow[] private escrows;
    mapping(bytes32 preimageHash => uint256 escrowIndex) private preimageHashToEscrowIndex; // 1-based index

    /// index of a next preimage hash to be used for escrow
    uint256 public nextUnusedPreimageHashIndex;
    /// index of a next unprocessed escrow
    uint256 public nextUnprocessedEscrowIndex;

    uint256 private nextTransferRequestId;

    // NOTE: There is at most one cancelableTransferRequest per agent (an agent must cancel previous
    // return request before starting a new one). The number of agents cannot increase arbitrarily,
    // as agents that are allowed return are controlled by the governance. The total number will always be < ~10.
    // Therefore loops over cancelableTransferRequests are actually bounded and will not run out of gas.
    uint256[] private cancelableTransferRequests;

    // NOTE: The nonCancelableTransferRequests correspond to requests for direct core vault redemption.
    // The addresses to which the redemptions can be made are controlled by governance and the requests
    // to the same address get merged, so there will always be a limited number of requests (< ~10).
    // Therefore loops over nonCancelableTransferRequests are actually bounded and will not run out of gas.
    uint256[] private nonCancelableTransferRequests;

    mapping(uint256 transferRequestId => TransferRequest) private transferRequestById;

    // there will probably be no more than 10 destination addresses set in the system at any time
    string[] private allowedDestinationAddresses;
    mapping(string allowedDestinationAddress => uint256) private allowedDestinationAddressIndex; // 1-based index
    EnumerableSet.AddressSet private triggeringAccounts;
    EnumerableSet.AddressSet private emergencyPauseSenders;

    // settings
    /// escrow end time during a day in seconds (UTC time)
    uint128 private escrowEndTimeSeconds;
    /// amount to be escrowed
    uint128 private escrowAmount;
    /// minimal amount left in the core vault after escrowing
    uint128 private minimalAmount;
    /// fee
    uint128 private fee;

    /// available funds in the core vault
    uint128 public availableFunds;
    /// escrowed funds
    uint128 public escrowedFunds;
    /// cancelable transfer requests amount
    uint128 private cancelableTransferRequestsAmount;
    /// non-cancelable transfer requests amount
    uint128 private nonCancelableTransferRequestsAmount;

    /// paused state
    bool public paused;

    modifier onlyAssetManager() {
        _checkOnlyAssetManager();
        _;
    }

    modifier notPaused() {
        _checkNotPaused();
        _;
    }

    constructor()
        GovernedUUPSProxyImplementation()
        AddressUpdatable(address(0))
    {
    }

    /**
     * Proxyable initialization method. Can be called only once, from the proxy constructor
     * (single call is assured by GovernedBase.initialise).
     */
    function initialize(
        IGovernanceSettings _governanceSettings,
        address _initialGovernance,
        address _addressUpdater,
        address _assetManager,
        bytes32 _chainId,
        string memory _custodianAddress,
        string memory _coreVaultAddress,
        uint256 _nextSequenceNumber
    )
        external
    {
        require(_assetManager != address(0), InvalidAddress());
        require(_chainId != bytes32(0), InvalidChain());
        require(bytes(_custodianAddress).length > 0, InvalidAddress());
        require(bytes(_coreVaultAddress).length > 0, InvalidAddress());

        GovernedBase.initialise(_governanceSettings, _initialGovernance);
        AddressUpdatable.setAddressUpdaterValue(_addressUpdater);

        assetManager = _assetManager;
        chainId = _chainId;
        custodianAddress = _custodianAddress;
        coreVaultAddressHash = keccak256(bytes(_coreVaultAddress));
        coreVaultAddress = _coreVaultAddress;
        nextSequenceNumber = _nextSequenceNumber;
        emit CustodianAddressUpdated(_custodianAddress);
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function confirmPayment(
        IPayment.Proof calldata _proof
    )
        external
    {
        require(_proof.data.responseBody.status == 0, PaymentFailed()); // 0 = payment success
        require(_proof.data.sourceId == chainId, InvalidChain());
        require(fdcVerification.verifyPayment(_proof), PaymentNotProven());
        require(_proof.data.responseBody.receivingAddressHash == coreVaultAddressHash, NotCoreVault());
        require(_proof.data.responseBody.receivedAmount > 0, InvalidAmount());
        if (!confirmedPayments[_proof.data.requestBody.transactionId]) {
            uint128 receivedAmount = uint128(uint256(_proof.data.responseBody.receivedAmount));
            confirmedPayments[_proof.data.requestBody.transactionId] = true;
            availableFunds += receivedAmount;
            emit PaymentConfirmed(
                _proof.data.requestBody.transactionId,
                _proof.data.responseBody.standardPaymentReference,
                receivedAmount
            );
        }
    }

    /**
     * @inheritdoc IICoreVaultManager
     */
    function requestTransferFromCoreVault(
        string memory _destinationAddress,
        bytes32 _paymentReference,
        uint128 _amount,
        bool _cancelable
    )
        external
        onlyAssetManager notPaused
        returns (bytes32)
    {
        require(_amount > 0, AmountZero());
        require(allowedDestinationAddressIndex[_destinationAddress] != 0, DestinationNotAllowed());
        bytes32 destinationAddressHash = keccak256(bytes(_destinationAddress));
        bool newTransferRequest = false;
        if (_cancelable) {
            // only one cancelable request per destination address
            for (uint256 i = 0; i < cancelableTransferRequests.length; i++) {
                TransferRequest storage req = transferRequestById[cancelableTransferRequests[i]];
                require(keccak256(bytes(req.destinationAddress)) != destinationAddressHash, RequestExists());
            }
            cancelableTransferRequestsAmount += _amount;
            cancelableTransferRequests.push(nextTransferRequestId);
            newTransferRequest = true;
        } else {
            uint256 index = 0;
            while (index < nonCancelableTransferRequests.length) {
                TransferRequest storage req = transferRequestById[nonCancelableTransferRequests[index]];
                if (keccak256(bytes(req.destinationAddress)) == destinationAddressHash) {
                    // add the amount to the existing request
                    req.amount += _amount;
                    _paymentReference = req.paymentReference;   // use the old payment reference when merged
                    break;
                }
                index++;
            }
            nonCancelableTransferRequestsAmount += _amount;
            // if the request does not exist, add a new one
            if (index == nonCancelableTransferRequests.length) {
                nonCancelableTransferRequests.push(nextTransferRequestId);
                newTransferRequest = true;
            }
        }

        uint256 requestsAmount = totalRequestAmountWithFee();
        require(requestsAmount <= availableFunds + escrowedFunds, InsufficientFunds());

        if (newTransferRequest) {
            transferRequestById[nextTransferRequestId++] = TransferRequest({
                destinationAddress: _destinationAddress,
                paymentReference: _paymentReference,
                amount: _amount
            });
        }
        emit TransferRequested(_destinationAddress, _paymentReference, _amount, _cancelable);
        return _paymentReference;
    }

    /**
     * @inheritdoc IICoreVaultManager
     */
    function cancelTransferRequestFromCoreVault(
        string memory _destinationAddress
    )
        external
        onlyAssetManager
    {
        bytes32 destinationAddressHash = keccak256(bytes(_destinationAddress));
        uint256 index = 0;
        while (index < cancelableTransferRequests.length) {
            string memory destAddress = transferRequestById[cancelableTransferRequests[index]].destinationAddress;
            if (keccak256(bytes(destAddress)) == destinationAddressHash) {
                break;
            }
            index++;
        }
        require (index < cancelableTransferRequests.length, NotFound());
        uint256 transferRequestId = cancelableTransferRequests[index];
        TransferRequest storage req = transferRequestById[transferRequestId];
        uint128 amount = req.amount;
        cancelableTransferRequestsAmount -= amount;
        emit TransferRequestCanceled(_destinationAddress, req.paymentReference, amount);

        // remove the transfer request - keep the order
        while (index < cancelableTransferRequests.length - 1) { // length > 0
            cancelableTransferRequests[index] = cancelableTransferRequests[index + 1]; // shift left
            index++;
        }
        cancelableTransferRequests.pop(); // remove the last element
        delete transferRequestById[transferRequestId];
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function processEscrows(uint256 _maxCount) external returns (bool) {
        return _processEscrows(_maxCount);
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function triggerInstructions() external notPaused returns (uint256 _numberOfInstructions) {
        require(triggeringAccounts.contains(msg.sender), NotAuthorized());
        _processEscrows(type(uint256).max); // process all escrows
        uint128 availableFundsTmp = availableFunds;
        uint256 sequenceNumberTmp = nextSequenceNumber;

        // process cancelable transfer requests
        uint128 feeTmp = fee;
        require(feeTmp > 0, FeeZero());
        uint256 index = 0;
        uint256 length = cancelableTransferRequests.length;
        uint128 amountTmp = cancelableTransferRequestsAmount;
        while (index < length) {
            uint256 transferRequestId = cancelableTransferRequests[index];
            if (availableFundsTmp >= transferRequestById[transferRequestId].amount + feeTmp) {
                TransferRequest memory req = transferRequestById[transferRequestId];
                availableFundsTmp -= (req.amount + feeTmp);
                amountTmp -= req.amount;
                emit PaymentInstructions(
                    sequenceNumberTmp++,
                    coreVaultAddress,
                    req.destinationAddress,
                    req.amount,
                    feeTmp,
                    req.paymentReference
                );
                _numberOfInstructions++;
                // remove the transfer request - keep the order
                for (uint256 i = index; i < length - 1; i++) { // length > 0
                    cancelableTransferRequests[i] = cancelableTransferRequests[i + 1]; // shift left
                }
                cancelableTransferRequests.pop(); // remove the last element
                delete transferRequestById[transferRequestId];
                length--;
            } else {
                index++;
            }
        }
        cancelableTransferRequestsAmount = amountTmp;

        // process non-cancelable transfer requests
        index = 0;
        length = nonCancelableTransferRequests.length;
        amountTmp = nonCancelableTransferRequestsAmount;
        while (index < length) {
            uint256 transferRequestId = nonCancelableTransferRequests[index];
            if (availableFundsTmp >= transferRequestById[transferRequestId].amount + feeTmp) {
                TransferRequest memory req = transferRequestById[transferRequestId];
                availableFundsTmp -= (req.amount + feeTmp);
                amountTmp -= req.amount;
                emit PaymentInstructions(
                    sequenceNumberTmp++,
                    coreVaultAddress,
                    req.destinationAddress,
                    req.amount,
                    feeTmp,
                    req.paymentReference
                );
                _numberOfInstructions++;
                // remove the transfer request - keep the order
                for (uint256 i = index; i < length - 1; i++) { // length > 0
                    nonCancelableTransferRequests[i] = nonCancelableTransferRequests[i + 1]; // shift left
                }
                nonCancelableTransferRequests.pop(); // remove the last element
                delete transferRequestById[transferRequestId];
                length--;
            } else {
                index++;
            }
        }
        nonCancelableTransferRequestsAmount = amountTmp;

        uint128 escrowAmountTmp = escrowAmount;
        if (escrowAmountTmp == 0 || length > 0 || cancelableTransferRequests.length > 0) {
            // update the state but skip creating new escrows
            availableFunds = availableFundsTmp;
            nextSequenceNumber = sequenceNumberTmp;
            return _numberOfInstructions;
        }

        // create escrows
        uint256 preimageHashIndexTmp = nextUnusedPreimageHashIndex;
        uint256 minFundsToTriggerEscrow = minimalAmount + escrowAmountTmp + feeTmp;
        length = preimageHashes.length();
        amountTmp = escrowedFunds;
        if (availableFundsTmp >= minFundsToTriggerEscrow && preimageHashIndexTmp < length) {
            uint64 escrowEndTimestamp = _getNextEscrowEndTimestamp();
            while (availableFundsTmp >= minFundsToTriggerEscrow && preimageHashIndexTmp < length) {
                availableFundsTmp -= (escrowAmountTmp + feeTmp);
                amountTmp += escrowAmountTmp;
                bytes32 preimageHash = preimageHashes.at(preimageHashIndexTmp++);
                Escrow memory escrow = Escrow({
                    preimageHash: preimageHash,
                    amount: escrowAmountTmp,
                    expiryTs: escrowEndTimestamp,
                    finished: false
                });
                escrows.push(escrow);
                preimageHashToEscrowIndex[preimageHash] = escrows.length;
                emit EscrowInstructions(
                    sequenceNumberTmp++,
                    preimageHash,
                    coreVaultAddress,
                    custodianAddress,
                    escrowAmountTmp,
                    feeTmp,
                    escrowEndTimestamp
                );
                _numberOfInstructions++;
                // next escrow end timestamp
                escrowEndTimestamp += 1 days;
            }
            nextUnusedPreimageHashIndex = preimageHashIndexTmp;
        }

        // update the state
        availableFunds = availableFundsTmp;
        nextSequenceNumber = sequenceNumberTmp;
        escrowedFunds = amountTmp;
    }

    /**
     * Adds allowed destination addresses.
     * @param _allowedDestinationAddresses List of allowed destination addresses to add.
     * NOTE: may only be called by the governance.
     */
    function addAllowedDestinationAddresses(
        string[] calldata _allowedDestinationAddresses
    )
        external
        onlyGovernance
    {
        for (uint256 i = 0; i < _allowedDestinationAddresses.length; i++) {
            require(bytes(_allowedDestinationAddresses[i]).length > 0, InvalidAddress());
            if (allowedDestinationAddressIndex[_allowedDestinationAddresses[i]] != 0) {
                continue;
            }
            allowedDestinationAddresses.push(_allowedDestinationAddresses[i]);
            allowedDestinationAddressIndex[_allowedDestinationAddresses[i]] = allowedDestinationAddresses.length;
            emit AllowedDestinationAddressAdded(_allowedDestinationAddresses[i]);
        }
    }

    /**
     * Removes allowed destination addresses.
     * @param _allowedDestinationAddresses List of allowed destination addresses to remove.
     * NOTE: may only be called by the governance.
     */
    function removeAllowedDestinationAddresses(
        string[] calldata _allowedDestinationAddresses
    )
        external
        onlyGovernance
    {
        for (uint256 i = 0; i < _allowedDestinationAddresses.length; i++) {
            uint256 index = allowedDestinationAddressIndex[_allowedDestinationAddresses[i]];
            if (index == 0) {
                continue;
            }
            uint256 length = allowedDestinationAddresses.length;
            if (index < length) {
                string memory addressToMove = allowedDestinationAddresses[length - 1];
                allowedDestinationAddresses[index - 1] = addressToMove;
                allowedDestinationAddressIndex[addressToMove] = index;
            }
            allowedDestinationAddresses.pop();
            delete allowedDestinationAddressIndex[_allowedDestinationAddresses[i]];
            emit AllowedDestinationAddressRemoved(_allowedDestinationAddresses[i]);
        }
    }

    /**
     * Adds the triggering accounts.
     * @param _triggeringAccounts List of triggering accounts to add.
     * NOTE: may only be called by the governance.
     */
    function addTriggeringAccounts(
        address[] calldata _triggeringAccounts
    )
        external
        onlyGovernance
    {
        for (uint256 i = 0; i < _triggeringAccounts.length; i++) {
            if (triggeringAccounts.add(_triggeringAccounts[i])) {
                emit TriggeringAccountAdded(_triggeringAccounts[i]);
            }
        }
    }

    /**
     * Removes the triggering accounts.
     * @param _triggeringAccounts List of triggering accounts to remove.
     * NOTE: may only be called by the governance.
     */
    function removeTriggeringAccounts(
        address[] calldata _triggeringAccounts
    )
        external
        onlyGovernance
    {
        for (uint256 i = 0; i < _triggeringAccounts.length; i++) {
            if (triggeringAccounts.remove(_triggeringAccounts[i])) {
                emit TriggeringAccountRemoved(_triggeringAccounts[i]);
            }
        }
    }

    /**
     * Updates the custodian address.
     * @param _custodianAddress Custodian address.
     * NOTE: may only be called by the governance.
     */
    function updateCustodianAddress(
        string calldata _custodianAddress
    )
        external
        onlyGovernance
    {
        require(bytes(_custodianAddress).length > 0, InvalidAddress());
        custodianAddress = _custodianAddress;
        emit CustodianAddressUpdated(_custodianAddress);
    }

    /**
     * Updates the settings.
     * @param _escrowEndTimeSeconds Escrow end time in seconds.
     * @param _escrowAmount Escrow amount (setting to 0 will disable escrows).
     * @param _minimalAmount Minimal amount left in the core vault after escrow.
     * @param _fee Fee.
     * NOTE: may only be called by the governance.
     */
    function updateSettings(
        uint128 _escrowEndTimeSeconds,
        uint128 _escrowAmount,
        uint128 _minimalAmount,
        uint128 _fee
    )
        external
        onlyGovernance
    {
        require(_escrowEndTimeSeconds < 1 days, InvalidEndTime());
        require(_fee > 0, FeeZero());
        escrowEndTimeSeconds = _escrowEndTimeSeconds;
        escrowAmount = _escrowAmount;
        minimalAmount = _minimalAmount;
        fee = _fee;
        emit SettingsUpdated(_escrowEndTimeSeconds, _escrowAmount, _minimalAmount, _fee);
    }

    /**
     * Adds preimage hashes.
     * @param _preimageHashes List of preimage hashes.
     * NOTE: may only be called by the governance.
     */
    function addPreimageHashes(
        bytes32[] calldata _preimageHashes
    )
        external
        onlyImmediateGovernance
    {
        for (uint256 i = 0; i < _preimageHashes.length; i++) {
            require(_preimageHashes[i] != bytes32(0) && preimageHashes.add(_preimageHashes[i]),
                InvalidPreimageHash());
            emit PreimageHashAdded(_preimageHashes[i]);
        }
    }

    /**
     * Remove last unused preimage hashes.
     * @param _maxCount Maximum number of preimage hashes to remove.
     * NOTE: may only be called by the governance.
     */
    function removeUnusedPreimageHashes(
        uint256 _maxCount
    )
        external
        onlyImmediateGovernance
    {
        uint256 index = preimageHashes.length();
        while (_maxCount > 0 && index > nextUnusedPreimageHashIndex) {
            bytes32 preimageHash = preimageHashes.at(--index);
            preimageHashes.remove(preimageHash);
            _maxCount--;
            emit UnusedPreimageHashRemoved(preimageHash);
        }
    }

    /**
     * Sets escrows as finished.
     * @param _preimageHashes List of preimage hashes.
     * NOTE: may only be called by the governance.
     */
    function setEscrowsFinished(
        bytes32[] calldata _preimageHashes
    )
        external
        onlyImmediateGovernance
    {
        uint128 availableFundsTmp = availableFunds;
        uint128 escrowedFundsTmp = escrowedFunds;
        for (uint256 i = 0; i < _preimageHashes.length; i++) {
            uint256 escrowIndex = preimageHashToEscrowIndex[_preimageHashes[i]];
            Escrow storage escrow = _getEscrow(escrowIndex);
            require(!escrow.finished, EscrowAlreadyFinished());
            escrow.finished = true;
            if (escrowIndex <= nextUnprocessedEscrowIndex) {
                availableFundsTmp -= escrow.amount;
            } else {
                escrowedFundsTmp -= escrow.amount;
            }
            emit EscrowFinished(_preimageHashes[i], escrow.amount);
        }
        availableFunds = availableFundsTmp;
        escrowedFunds = escrowedFundsTmp;
    }

    /**
     * Adds emergency pause senders.
     * @param _addresses List of emergency pause senders to add.
     * NOTE: may only be called by the governance.
     */
    function addEmergencyPauseSenders(address[] calldata _addresses)
        external
        onlyImmediateGovernance
    {
        for (uint256 i = 0; i < _addresses.length; i++) {
            if(emergencyPauseSenders.add(_addresses[i])) {
                emit EmergencyPauseSenderAdded(_addresses[i]);
            }
        }
    }

    /**
     * Removes emergency pause senders.
     * @param _addresses List of emergency pause senders to remove.
     * NOTE: may only be called by the governance.
     */
    function removeEmergencyPauseSenders(address[] calldata _addresses)
        external
        onlyImmediateGovernance
    {
        for (uint256 i = 0; i < _addresses.length; i++) {
            if (emergencyPauseSenders.remove(_addresses[i])) {
                emit EmergencyPauseSenderRemoved(_addresses[i]);
            }
        }
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function pause() external {
        require(msg.sender == governance() || emergencyPauseSenders.contains(msg.sender), NotAuthorized());
        paused = true;
        emit Paused();
    }

    /**
     * Unpauses the contract. New transfer requests and instructions can be triggered.
     * NOTE: may only be called by the governance.
     */
    function unpause() external onlyImmediateGovernance {
        paused = false;
        emit Unpaused();
    }

    /**
     * Triggers custom instructions, which are not related to payment or escrow but increases the sequence number.
     * @param _instructionsHash Hash of the instructions send off-chain.
     * NOTE: may only be called by the governance.
     */
    function triggerCustomInstructions(bytes32 _instructionsHash) external onlyImmediateGovernance {
        emit CustomInstructions(
            nextSequenceNumber++,
            coreVaultAddress,
            _instructionsHash
        );
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function getSettings()
        external view
        returns (
            uint128 _escrowEndTimeSeconds,
            uint128 _escrowAmount,
            uint128 _minimalAmount,
            uint128 _fee
        )
    {
        return (escrowEndTimeSeconds, escrowAmount, minimalAmount, fee);
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function getAllowedDestinationAddresses() external view returns (string[] memory) {
        return allowedDestinationAddresses;
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function isDestinationAddressAllowed(string memory _address) external view returns (bool) {
        return allowedDestinationAddressIndex[_address] > 0;
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function getTriggeringAccounts() external view returns (address[] memory) {
        return triggeringAccounts.values();
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function getUnprocessedEscrows() external view returns (Escrow[] memory _unprocessedEscrows) {
        uint256 length = escrows.length - nextUnprocessedEscrowIndex;
        _unprocessedEscrows = new Escrow[](length);
        for (uint256 i = 0; i < length; i++) {
            _unprocessedEscrows[i] = escrows[nextUnprocessedEscrowIndex + i];
        }
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function getEscrowsCount() external view returns (uint256) {
        return escrows.length;
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function getEscrowByIndex(uint256 _index) external view returns (Escrow memory) {
        return escrows[_index];
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function getEscrowByPreimageHash(bytes32 _preimageHash) external view returns (Escrow memory) {
        uint256 index = preimageHashToEscrowIndex[_preimageHash];
        return _getEscrow(index);
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function getUnusedPreimageHashes() external view returns (bytes32[] memory) {
        uint256 length = preimageHashes.length() - nextUnusedPreimageHashIndex;
        bytes32[] memory unusedPreimageHashes = new bytes32[](length);
        for (uint256 i = 0; i < length; i++) {
            unusedPreimageHashes[i] = preimageHashes.at(nextUnusedPreimageHashIndex + i);
        }
        return unusedPreimageHashes;
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function getPreimageHashesCount() external view returns (uint256) {
        return preimageHashes.length();
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function getPreimageHash(uint256 _index) external view returns (bytes32) {
        return preimageHashes.at(_index);
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function getCancelableTransferRequests() external view returns (TransferRequest[] memory _transferRequests) {
        _transferRequests = new TransferRequest[](cancelableTransferRequests.length);
        for (uint256 i = 0; i < cancelableTransferRequests.length; i++) {
            _transferRequests[i] = transferRequestById[cancelableTransferRequests[i]];
        }
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function getNonCancelableTransferRequests() external view returns (TransferRequest[] memory _transferRequests) {
        _transferRequests = new TransferRequest[](nonCancelableTransferRequests.length);
        for (uint256 i = 0; i < nonCancelableTransferRequests.length; i++) {
            _transferRequests[i] = transferRequestById[nonCancelableTransferRequests[i]];
        }
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function totalRequestAmountWithFee() public view returns (uint256) {
        return nonCancelableTransferRequestsAmount + cancelableTransferRequestsAmount +
            (cancelableTransferRequests.length + nonCancelableTransferRequests.length) * fee;
    }

    /**
     * @inheritdoc ICoreVaultManager
     */
    function getEmergencyPauseSenders() external view returns (address[] memory) {
        return emergencyPauseSenders.values();
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // ERC 165

    /**
     * Implementation of ERC-165 interface.
     */
    function supportsInterface(bytes4 _interfaceId)
        external pure override
        returns (bool)
    {
        return _interfaceId == type(IERC165).interfaceId
            || _interfaceId == type(IIAddressUpdatable).interfaceId
            || _interfaceId == type(IICoreVaultManager).interfaceId;
    }

    /**
     * @inheritdoc AddressUpdatable
     */
    function _updateContractAddresses(
        bytes32[] memory _contractNameHashes,
        address[] memory _contractAddresses
    )
        internal override
    {
        fdcVerification = IFdcVerification(
            _getContractAddress(_contractNameHashes, _contractAddresses, "FdcVerification"));
    }

    /**
     * Processes the escrows.
     * @param _maxCount Maximum number of escrows to process.
     * @return _allProcessed True if all escrows were processed, false otherwise.
     */
    function _processEscrows(uint256 _maxCount) internal returns (bool _allProcessed) {
        uint128 availableFundsTmp = availableFunds;
        uint128 escrowedFundsTmp = escrowedFunds;
        // process all expired or finished escrows
        uint256 index = nextUnprocessedEscrowIndex;
        while (_maxCount > 0 && index < escrows.length &&
            (escrows[index].expiryTs <= block.timestamp || escrows[index].finished))
        {
            if (!escrows[index].finished) {
                // if the escrow is not finished, add the amount to the available funds
                Escrow storage escrow = escrows[index];
                uint128 amount = escrow.amount;
                availableFundsTmp += amount;
                escrowedFundsTmp -= amount;
                emit EscrowExpired(escrow.preimageHash, amount);
            }
            index++;
            _maxCount--;
        }
        // update the state
        nextUnprocessedEscrowIndex = index;
        availableFunds = availableFundsTmp;
        escrowedFunds = escrowedFundsTmp;

        _allProcessed = _maxCount > 0 || index == escrows.length ||
            (escrows[index].expiryTs > block.timestamp && !escrows[index].finished);
        if (!_allProcessed) {
            emit NotAllEscrowsProcessed();
        }
    }

    /**
     * Gets the escrow by index.
     * @param _index Escrow index (1-based).
     * @return Escrow.
     */
    function _getEscrow(uint256 _index) internal view returns (Escrow storage) {
        require(_index != 0, NotFound());
        return escrows[_index - 1];
    }

    /**
     * Gets the next escrow end timestamp.
     * @return Next escrow end timestamp.
     */
    function _getNextEscrowEndTimestamp() internal view returns (uint64) {
        uint256 escrowEndTimestamp = 0;
        // find the last unfinished escrow
        for (uint256 i = escrows.length; i > nextUnprocessedEscrowIndex; i--) {
            if (!escrows[i - 1].finished) {
                escrowEndTimestamp = escrows[i - 1].expiryTs;
                break;
            }
        }
        escrowEndTimestamp = Math.max(escrowEndTimestamp, block.timestamp);
        escrowEndTimestamp += 1 days;
        // slither-disable-next-line weak-prng
        escrowEndTimestamp = escrowEndTimestamp - (escrowEndTimestamp % 1 days) + escrowEndTimeSeconds;
        if (escrowEndTimestamp <= block.timestamp + 12 hours) { // less than 12 hours from now, move to the next day
            escrowEndTimestamp += 1 days;
        }
        return uint64(escrowEndTimestamp);
    }

    /**
     * Checks if the caller is the asset manager.
     */
    function _checkOnlyAssetManager() internal view {
        require(msg.sender == assetManager, OnlyAssetManager());
    }

    /**
     * Checks if the contract is not paused.
     */
    function _checkNotPaused() internal view {
        require(!paused, ContractPaused());
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
pragma solidity >=0.7.6 <0.9;

import {ICoreVaultManager} from "../../userInterfaces/ICoreVaultManager.sol";

/**
 * Core vault manager internal interface
 */
interface IICoreVaultManager is ICoreVaultManager {

    /**
     * Requests transfer from core vault to destination address.
     * @param _destinationAddress destination address
     * @param _paymentReference payment reference
     * @param _amount amount
     * @param _cancelable cancelable flag (if true, the request can be canceled)
     * @return _actualPaymentReference the actual payment reference that will be used - for non-cancelable requests
     *  it can differ from the requested payment reference, because multiple queued payments to the same address
     *  are merged in which case the reference of the previous payment to the same address will be used
     * NOTE: destination address must be allowed otherwise the request will revert.
     * NOTE: may only be called by the asset manager.
     */
    function requestTransferFromCoreVault(
        string memory _destinationAddress,
        bytes32 _paymentReference,
        uint128 _amount,
        bool _cancelable
    )
        external
        returns (bytes32 _actualPaymentReference);

    /**
     * Cancels transfer request from core vault.
     * @param _destinationAddress destination address
     * NOTE: if the request does not exist (anymore), the call will revert.
     * NOTE: may only be called by the asset manager.
     */
    function cancelTransferRequestFromCoreVault(
        string memory _destinationAddress
    )
        external;
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IGovernanceSettings} from "@flarenetwork/flare-periphery-contracts/flare/IGovernanceSettings.sol";
import {IGoverned} from "../interfaces/IGoverned.sol";

/**
 * @title Governed Base
 * @notice This abstract base class defines behaviors for a governed contract.
 * @dev This class is abstract so that specific behaviors can be defined for the constructor.
 *   Contracts should not be left ungoverned, but not all contract will have a constructor
 *   (for example those pre-defined in genesis).
 * @dev This version is compatible with both Flare (where governance settings is in genesis at the address
 *   0x1000000000000000000000000000000000000007) and Songbird (where governance settings is a deployed contract).
 * @dev It also uses diamond storage for state, so it is safer tp use in diamond structures or proxies.
 **/
abstract contract GovernedBase is IGoverned {
    struct GovernedState {
        IGovernanceSettings governanceSettings;
        bool initialised;
        bool productionMode;
        bool executing;
        address initialGovernance;
        mapping(bytes32 encodedCallHash => uint256 allowedAfterTimestamp) timelockedCalls;
    }

    modifier onlyGovernance {
        if (_timeToExecute()) {
            _beforeExecute();
            _;
        } else {
            _recordTimelockedCall(msg.data, 0);
        }
    }

    modifier onlyGovernanceWithTimelockAtLeast(uint256 _minimumTimelock) {
        if (_timeToExecute()) {
            _beforeExecute();
            _;
        } else {
            _recordTimelockedCall(msg.data, _minimumTimelock);
        }
    }

    modifier onlyImmediateGovernance {
        _checkOnlyGovernance();
        _;
    }

    // solhint-disable-next-line no-empty-blocks
    constructor() {
    }

    /**
     * @notice Execute the timelocked governance calls once the timelock period expires.
     * @dev Only executor can call this method.
     * @param _encodedCall ABI encoded call data (signature and parameters).
     */
    function executeGovernanceCall(bytes calldata _encodedCall) external override {
        GovernedState storage state = _governedState();
        require(isExecutor(msg.sender), OnlyExecutor());
        bytes32 encodedCallHash = keccak256(_encodedCall);
        uint256 allowedAfterTimestamp = state.timelockedCalls[encodedCallHash];
        require(allowedAfterTimestamp != 0, TimelockInvalidSelector());
        require(block.timestamp >= allowedAfterTimestamp, TimelockNotAllowedYet());
        delete state.timelockedCalls[encodedCallHash];
        state.executing = true;
        //solhint-disable-next-line avoid-low-level-calls
        (bool success,) = address(this).call(_encodedCall);
        state.executing = false;
        emit TimelockedGovernanceCallExecuted(encodedCallHash);
        _passReturnOrRevert(success);
    }

    /**
     * Cancel a timelocked governance call before it has been executed.
     * @dev Only governance can call this method.
     * @param _encodedCall ABI encoded call data (signature and parameters).
     */
    function cancelGovernanceCall(bytes calldata _encodedCall) external override onlyImmediateGovernance {
        GovernedState storage state = _governedState();
        bytes32 encodedCallHash = keccak256(_encodedCall);
        require(state.timelockedCalls[encodedCallHash] != 0, TimelockInvalidSelector());
        emit TimelockedGovernanceCallCanceled(encodedCallHash);
        delete state.timelockedCalls[encodedCallHash];
    }

    /**
     * Enter the production mode after all the initial governance settings have been set.
     * This enables timelocks and the governance is afterwards obtained by calling
     * governanceSettings.getGovernanceAddress().
     */
    function switchToProductionMode() external onlyImmediateGovernance {
        GovernedState storage state = _governedState();
        require(!state.productionMode, AlreadyInProductionMode());
        state.initialGovernance = address(0);
        state.productionMode = true;
        emit GovernedProductionModeEntered(address(state.governanceSettings));
    }

    /**
     * @notice Initialize the governance address if not first initialized.
     */
    function initialise(IGovernanceSettings _governanceSettings, address _initialGovernance) internal virtual {
        GovernedState storage state = _governedState();
        require(state.initialised == false, GovernedAlreadyInitialized());
        require(address(_governanceSettings) != address(0), GovernedAddressZero());
        require(_initialGovernance != address(0), GovernedAddressZero());
        state.initialised = true;
        state.governanceSettings = _governanceSettings;
        state.initialGovernance = _initialGovernance;
        emit GovernanceInitialised(_initialGovernance);
    }

    /**
     * Returns the governance settings contract address.
     */
    function governanceSettings() public view returns (IGovernanceSettings) {
        return _governedState().governanceSettings;
    }

    /**
     * True after switching to production mode (see `switchToProductionMode()`).
     */
    function productionMode() public view returns (bool) {
        return _governedState().productionMode;
    }

    /**
     * Returns the current effective governance address.
     */
    function governance() public view returns (address) {
        GovernedState storage state = _governedState();
        return state.productionMode ? state.governanceSettings.getGovernanceAddress() : state.initialGovernance;
    }

    /**
     * Check if an address is one of the executors defined in governanceSettings.
     */
    function isExecutor(address _address) public view returns (bool) {
        GovernedState storage state = _governedState();
        return state.initialised && state.governanceSettings.isExecutor(_address);
    }

    function _beforeExecute() private {
        GovernedState storage state = _governedState();
        if (state.executing) {
            // can only be run from executeGovernanceCall(), where we check that only executor can call
            // make sure nothing else gets executed, even in case of reentrancy
            assert(msg.sender == address(this));
            state.executing = false;
        } else {
            // must be called with: productionMode=false
            // must check governance in this case
            _checkOnlyGovernance();
        }
    }

    function _recordTimelockedCall(bytes calldata _encodedCall, uint256 _minimumTimelock) private {
        GovernedState storage state = _governedState();
        _checkOnlyGovernance();
        bytes32 encodedCallHash = keccak256(_encodedCall);
        uint256 timelock = state.governanceSettings.getTimelock();
        if (timelock < _minimumTimelock) {
            timelock = _minimumTimelock;
        }
        uint256 allowedAt = block.timestamp + timelock;
        state.timelockedCalls[encodedCallHash] = allowedAt;
        emit GovernanceCallTimelocked(_encodedCall, encodedCallHash, allowedAt);
    }

    function _timeToExecute() private view returns (bool) {
        GovernedState storage state = _governedState();
        return state.executing || !state.productionMode;
    }

    function _checkOnlyGovernance() private view {
        require(msg.sender == governance(), OnlyGovernance());
    }

    function _governedState() private pure returns (GovernedState storage _state) {
        bytes32 position = keccak256("fasset.GovernedBase.GovernedState");
        // solhint-disable-next-line no-inline-assembly
        assembly {
            _state.slot := position
        }
    }

    function _passReturnOrRevert(bool _success) private pure {
        // pass exact return or revert data - needs to be done in assembly
        //solhint-disable-next-line no-inline-assembly
        assembly {
            let size := returndatasize()
            let ptr := mload(0x40)
            mstore(0x40, add(ptr, size))
            returndatacopy(ptr, 0, size)
            if _success {
                return(ptr, size)
            }
            revert(ptr, size)
        }
    }
}
// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import {IPayment} from "@flarenetwork/flare-periphery-contracts/flare/IFdcVerification.sol";

/**
 * Core vault manager
 */
interface ICoreVaultManager {

    error InvalidAddress();
    error InvalidChain();
    error InvalidAmount();
    error PaymentFailed();
    error PaymentNotProven();
    error NotCoreVault();
    error AmountZero();
    error DestinationNotAllowed();
    error RequestExists();
    error InsufficientFunds();
    error NotFound();
    error NotAuthorized();
    error FeeZero();
    error InvalidEndTime();
    error InvalidPreimageHash();
    error EscrowAlreadyFinished();
    error OnlyAssetManager();
    error ContractPaused();

    // Structs
    struct Escrow {
        bytes32 preimageHash;
        uint128 amount;
        uint64 expiryTs;
        bool finished;
    }

    struct TransferRequest {
        string destinationAddress;
        bytes32 paymentReference;
        uint128 amount;
    }

    // Events
    event PaymentConfirmed(
        bytes32 indexed transactionId,
        bytes32 paymentReference,
        uint256 amount
    );

    event PaymentInstructions(
        uint256 indexed sequence,
        string account,
        string destination,
        uint256 amount,
        uint256 fee,
        bytes32 paymentReference
    );

    event EscrowInstructions(
        uint256 indexed sequence,
        bytes32 indexed preimageHash,
        string account,
        string destination,
        uint256 amount,
        uint256 fee,
        uint256 cancelAfterTs
    );

    event CustomInstructions(
        uint256 indexed sequence,
        string account,
        bytes32 instructionsHash
    );

    event TransferRequested(
        string destinationAddress,
        bytes32 paymentReference,
        uint256 amount,
        bool cancelable
    );

    event TransferRequestCanceled(
        string destinationAddress,
        bytes32 paymentReference,
        uint256 amount
    );

    event EscrowExpired(
        bytes32 indexed preimageHash,
        uint256 amount);

    event NotAllEscrowsProcessed();

    event EscrowFinished(
        bytes32 indexed preimageHash,
        uint256 amount
    );

    event Paused();

    event Unpaused();

    event TriggeringAccountAdded(
        address triggeringAccount
    );

    event TriggeringAccountRemoved(
        address triggeringAccount
    );

    event AllowedDestinationAddressAdded(
        string destinationAddress
    );

    event AllowedDestinationAddressRemoved(
        string destinationAddress
    );

    event CustodianAddressUpdated(
        string custodianAddress
    );

    event SettingsUpdated(
        uint256 escrowEndTimeSeconds,
        uint256 escrowAmount,
        uint256 minimalAmount,
        uint256 fee
    );

    event PreimageHashAdded(
        bytes32 preimageHash
    );

    event UnusedPreimageHashRemoved(
        bytes32 preimageHash
    );

    event EmergencyPauseSenderAdded(
        address sender
    );

    event EmergencyPauseSenderRemoved(
        address sender
    );

    /**
     * Confirms payment to core vault address (increases available funds).
     * @param _proof payment proof
     */
    function confirmPayment(IPayment.Proof calldata _proof) external;

    /**
     * Pauses the contract. New requests and instructions cannot be triggered.
     * NOTE: may only be called by the governance or emergency pause senders.
     */
    function pause() external;

    /**
     * Trigger processing of escrows.
     * @param _maxCount Maximum number of escrows to process.
     * @return True if all escrows were processed, false otherwise.
     */
    function processEscrows(uint256 _maxCount) external returns (bool);

    /**
     * Triggers instructions - payment and escrow.
     * @return _numberOfInstructions Number of instructions triggered.
     * NOTE: cannot be called if the contract is paused.
     * NOTE: may only be called by the triggering accounts.
     */
    function triggerInstructions() external returns (uint256 _numberOfInstructions);

    /**
     * Returns the available funds.
     * @return Available funds.
     */
    function availableFunds() external view returns (uint128);

    /**
     * Returns the escrowed funds.
     * @return Escrowed funds.
     */
    function escrowedFunds() external view returns (uint128);

    /**
     * Returns the total amount requested, together with payment fee.
     */
    function totalRequestAmountWithFee() external view returns (uint256);

    /**
     * Indicates if the contract is paused. New transfer requests and instructions cannot be triggered.
     * @return True if paused, false otherwise.
     */
    function paused() external view returns (bool);

    /**
     * Gets the triggering accounts.
     * @return List of triggering accounts.
     */
    function getTriggeringAccounts() external view returns (address[] memory);


    /**
     * Returns settings.
     * @return _escrowEndTimeSeconds Escrow end time in seconds.
     * @return _escrowAmount Escrow amount.
     * @return _minimalAmount Minimal amount.
     * @return _fee Fee.
     */
    function getSettings()
        external view
        returns (
            uint128 _escrowEndTimeSeconds,
            uint128 _escrowAmount,
            uint128 _minimalAmount,
            uint128 _fee
        );

    /**
     * Gets the allowed destination addresses.
     * @return List of allowed destination addresses.
     */
    function getAllowedDestinationAddresses() external view returns (string[] memory);

    /**
     * Checks if the destination address is allowed.
     * @param _address Destination address.
     * @return True if allowed, false otherwise.
     */
    function isDestinationAddressAllowed(string memory _address) external view returns (bool);

    /**
     * Gets the core vault address.
     * @return Core vault address.
     */
    function coreVaultAddress() external view returns (string memory);

    /**
     * Gets the core vault address hash.
     * @return Core vault address hash.
     */
    function coreVaultAddressHash() external view returns (bytes32);

    /**
     * The corresponding asset manager.
     */
    function assetManager() external view returns (address);

    /**
     * Gets the custodian address.
     * @return Custodian address.
     */
    function custodianAddress() external view returns (string memory);

    /**
     * Returns next unprocessed escrow index.
     */
    function nextUnprocessedEscrowIndex() external view returns (uint256);

    /**
     * Gets unprocessed escrows.
     * @return List of unprocessed escrows.
     */
    function getUnprocessedEscrows() external view returns (Escrow[] memory);

    /**
     * Gets escrows count.
     * @return Escrows count.
     */
    function getEscrowsCount() external view returns (uint256);

    /**
     * Gets escrow by index.
     * @param _index Escrow index.
     * @return Escrow.
     */
    function getEscrowByIndex(uint256 _index) external view returns (Escrow memory);

    /**
     * Gets escrow by preimage hash.
     * @param _preimageHash Preimage hash.
     * @return Escrow.
     */
    function getEscrowByPreimageHash(bytes32 _preimageHash) external view returns (Escrow memory);

    /**
     * Returns next unused preimage hash index.
     */
    function nextUnusedPreimageHashIndex() external view returns (uint256);

    /**
     * Gets unused preimage hashes.
     * @return List of unused preimage hashes.
     */
    function getUnusedPreimageHashes() external view returns (bytes32[] memory);

    /**
     * Gets preimage hashes count.
     * @return Preimage hashes count.
     */
    function getPreimageHashesCount() external view returns (uint256);

    /**
     * Gets preimage hash by index.
     * @param _index Preimage hash index.
     * @return Preimage hash.
     */
    function getPreimageHash(uint256 _index) external view returns (bytes32);

    /**
     * Gets the cancelable transfer requests.
     * @return List of transfer cancelable requests.
     */
    function getCancelableTransferRequests() external view returns (TransferRequest[] memory);

    /**
     * Gets the non-cancelable transfer requests.
     * @return List of transfer non-cancelable requests.
     */
    function getNonCancelableTransferRequests() external view returns (TransferRequest[] memory);

    /**
     * Gets the list of emergency pause senders.
     * @return List of emergency pause senders.
     */
    function getEmergencyPauseSenders() external view returns (address[] memory);
}

## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS
// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

/**
 * @custom:name IPayment
 * @custom:id 0x01
 * @custom:supported BTC, DOGE, XRP
 * @author Flare
 * @notice A relay of a transaction on an external chain that is considered a payment in a native currency.
 * Various blockchains support different types of native payments. For each blockchain, it is specified how a payment
 * transaction should be formed to be provable by this attestation type.
 * The provable payments emulate traditional banking payments from entity A to entity B in native currency with an
 * optional payment reference.
 * @custom:verification The transaction with `transactionId` is fetched from the API of the blockchain node or
 * relevant indexer.
 * If the transaction cannot be fetched or the transaction is in a block that does not have a sufficient
 * [number of confirmations](/specs/attestations/configs.md#finalityconfirmation), the attestation request is rejected.
 *
 * Once the transaction is received, the payment summary is computed according to the rules for the source chain.
 * If the summary is successfully calculated, the response is assembled from the summary.
 * `blockNumber` and `blockTimestamp` are retrieved from the block if they are not included in the transaction data.
 * For Bitcoin and Dogecoin, `blockTimestamp` is mediantime of the block.
 * For XRPL, `blockTimestamp` is close time of the ledger converted to UNIX time.
 *
 * If the summary is not successfully calculated, the attestation request is rejected.
 * @custom:lut `blockTimestamp`
 * @custom:lutlimit `0x127500`, `0x127500`, `0x127500`
 */
interface IPayment {
    /**
     * @notice Toplevel request
     * @param attestationType ID of the attestation type.
     * @param sourceId ID of the data source.
     * @param messageIntegrityCode `MessageIntegrityCode` that is derived from the expected response.
     * @param requestBody Data defining the request. Type (struct) and interpretation is determined
     * by the `attestationType`.
     */
    struct Request {
        bytes32 attestationType;
        bytes32 sourceId;
        bytes32 messageIntegrityCode;
        RequestBody requestBody;
    }

    /**
     * @notice Toplevel response
     * @param attestationType Extracted from the request.
     * @param sourceId Extracted from the request.
     * @param votingRound The ID of the State Connector round in which the request was considered.
     * @param lowestUsedTimestamp The lowest timestamp used to generate the response.
     * @param requestBody Extracted from the request.
     * @param responseBody Data defining the response. The verification rules for the construction
     * of the response body and the type are defined per specific `attestationType`.
     */
    struct Response {
        bytes32 attestationType;
        bytes32 sourceId;
        uint64 votingRound;
        uint64 lowestUsedTimestamp;
        RequestBody requestBody;
        ResponseBody responseBody;
    }

    /**
     * @notice Toplevel proof
     * @param merkleProof Merkle proof corresponding to the attestation response.
     * @param data Attestation response.
     */
    struct Proof {
        bytes32[] merkleProof;
        Response data;
    }

    /**
     * @notice Request body for Payment attestation type
     * @param transactionId ID of the payment transaction.
     * @param inUtxo For UTXO chains, this is the index of the transaction input with source address.
     * Always 0 for the non-utxo chains.
     * @param utxo For UTXO chains, this is the index of the transaction output with receiving address.
     * Always 0 for the non-utxo chains.
     */
    struct RequestBody {
        bytes32 transactionId;
        uint256 inUtxo;
        uint256 utxo;
    }

    /**
     * @notice Response body for Payment attestation type
     * @param blockNumber Number of the block in which the transaction is included.
     * @param blockTimestamp The timestamp of the block in which the transaction is included.
     * @param sourceAddressHash Standard address hash of the source address.
     * @param sourceAddressesRoot The root of the Merkle tree of the source addresses.
     * @param receivingAddressHash Standard address hash of the receiving address.
     * The zero 32-byte string if there is no receivingAddress (if `status` is not success).
     * @param intendedReceivingAddressHash Standard address hash of the intended receiving address.
     * Relevant if the transaction is unsuccessful.
     * @param spentAmount Amount in minimal units spent by the source address.
     * @param intendedSpentAmount Amount in minimal units to be spent by the source address.
     * Relevant if the transaction status is unsuccessful.
     * @param receivedAmount Amount in minimal units received by the receiving address.
     * @param intendedReceivedAmount Amount in minimal units intended to be received by the receiving address.
     * Relevant if the transaction is unsuccessful.
     * @param standardPaymentReference Standard payment reference of the transaction.
     * @param oneToOne Indicator whether only one source and one receiver are involved in the transaction.
     * @param status Succes status of the transaction: 0 - success, 1 - failed by sender's fault,
     * 2 - failed by receiver's fault.
     */
    struct ResponseBody {
        uint64 blockNumber;
        uint64 blockTimestamp;
        bytes32 sourceAddressHash;
        bytes32 sourceAddressesRoot;
        bytes32 receivingAddressHash;
        bytes32 intendedReceivingAddressHash;
        int256 spentAmount;
        int256 intendedSpentAmount;
        int256 receivedAmount;
        int256 intendedReceivedAmount;
        bytes32 standardPaymentReference;
        bool oneToOne;
        uint8 status;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

interface IIAddressUpdatable {
    /**
     * @notice Updates contract addresses - should be called only from AddressUpdater contract
     * @param _contractNameHashes       list of keccak256(abi.encode(...)) contract names
     * @param _contractAddresses        list of contract addresses corresponding to the contract names
     */
    function updateContractAddresses(
        bytes32[] memory _contractNameHashes,
        address[] memory _contractAddresses
    ) external;
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import "./IRelay.sol";
import "./IAddressValidityVerification.sol";
import "./IBalanceDecreasingTransactionVerification.sol";
import "./IConfirmedBlockHeightExistsVerification.sol";
import "./IEVMTransactionVerification.sol";
import "./IPaymentVerification.sol";
import "./IReferencedPaymentNonexistenceVerification.sol";
import "./IWeb2JsonVerification.sol";

/**
 * FdcVerification interface.
 */
interface IFdcVerification is
    IAddressValidityVerification,
    IBalanceDecreasingTransactionVerification,
    IConfirmedBlockHeightExistsVerification,
    IEVMTransactionVerification,
    IPaymentVerification,
    IReferencedPaymentNonexistenceVerification,
    IWeb2JsonVerification
{
    /**
     * The FDC protocol id.
     */
    function fdcProtocolId() external view returns (uint8 _fdcProtocolId);

    /**
     * Relay contract address.
     */
    function relay() external view returns (IRelay);
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


## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

