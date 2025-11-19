
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {RedemptionQueue} from "./RedemptionQueue.sol";
import {PaymentConfirmations} from "./PaymentConfirmations.sol";
import {UnderlyingAddressOwnership} from "./UnderlyingAddressOwnership.sol";
import {CollateralReservation} from "./CollateralReservation.sol";
import {Redemption} from "./Redemption.sol";
import {CollateralTypeInt} from "./CollateralTypeInt.sol";


library AssetManagerState {
    struct State {
        // All collateral types, used for vault or pool.
        // Pool collateral (always WNat) has index 0.
        CollateralTypeInt.Data[] collateralTokens;

        // mapping((collateralClass, tokenAddress) => collateralTokens index + 1)
        mapping(bytes32 => uint256) collateralTokenIndex;

        // makes sure pool tokens have unique names and symbols
        mapping(string => bool) reservedPoolTokenSuffixes;

        // A list of all agents (for use by monitoring or challengers).
        // Type: array of agent vault addresses; when one is deleted, its position is filled with last
        address[] allAgents;

        // A list of all agents that are available for minting.
        // Type: array of agent vault addresses; when one is deleted, its position is filled with last
        address[] availableAgents;

        // Ownership of underlying source addresses is needed to prevent someone
        // overtaking the payer and presenting an underlying payment as his own.
        UnderlyingAddressOwnership.State underlyingAddressOwnership;

        // Type: mapping collateralReservationId => collateralReservation
        mapping(uint256 => CollateralReservation.Data) crts;

        // redemption queue
        RedemptionQueue.State redemptionQueue;

        // mapping redemptionRequest_id => request
        mapping(uint256 => Redemption.Request) redemptionRequests;

        // verified payment hashes; expire in 5 days
        PaymentConfirmations.State paymentConfirmations;

        // New ids (listed together to save storage); all must be incremented before assigning, so 0 means empty
        uint64 newCrtId;
        uint64 newRedemptionRequestId;
        uint64 newPaymentAnnouncementId;

        // Total collateral reservations (in underlying AMG units). Used by minting cap.
        uint64 totalReservedCollateralAMG;

        // Pool collateral is always wrapped NAT, but the wrapping contract may change.
        // In this case, new pool collateral token must be added and set as current.
        uint16 poolCollateralIndex;

        // Current block number and timestamp on the underlying chain
        uint64 currentUnderlyingBlock;
        uint64 currentUnderlyingBlockTimestamp;

        // The timestamp (on this network) when the underlying block was last updated
        uint64 currentUnderlyingBlockUpdatedAt;

        // If non-zero, minting is paused and has been paused at the time indicated by timestamp mintingPausedAt.
        // When asset manager is paused, no new mintings can be done, but redemptions still work.
        uint64 mintingPausedAt;

        // If non-zero, asset manager is paused and will be paused until the time indicated.
        // When asset manager is paused, all dangerous operations ar blocked (mintings, redemptions, etc.).
        // It is an extreme measure, which can be used in case there is a dangerous hole in the system.
        uint64 emergencyPausedUntil;

        // When emergency pause is not done by governance, the total allowed pause is limited.
        // So the caller must state the duration after which the pause will automatically end.
        // When total pauses exceed the max allowed length, pausing is only allowed by the governance.
        // An emergencyPause call by the governance optionally resets the total duration counter to 0.
        uint64 emergencyPausedTotalDuration;

        // When emergency pause was triggered by governance, only governance can unpause.
        bool emergencyPausedByGovernance;

        // If non-zero, asset manager is paused and will be paused until the time indicated.
        // When asset manager is paused, all dangerous operations ar blocked (mintings, redemptions, etc.).
        // It is an extreme measure, which can be used in case there is a dangerous hole in the system.
        uint64 transfersEmergencyPausedUntil;

        // When emergency pause is not done by governance, the total allowed pause is limited.
        // So the caller must state the duration after which the pause will automatically end.
        // When total pauses exceed the max allowed length, pausing is only allowed by the governance.
        // An emergencyPause call by the governance optionally resets the total duration counter to 0.
        uint64 transfersEmergencyPausedTotalDuration;

        // When emergency pause was triggered by governance, only governance can unpause.
        bool transfersEmergencyPausedByGovernance;

        // When true, asset manager has been added to the asset manager controller.
        // Even though the asset manager controller address is set at the construction time, the manager may not
        // be able to be added to the controller immediately because the method addAssetManager must be called
        // by the governance multisig (with timelock).
        // During this time it is impossible to verify through the controller that the asset manager is legit.
        // Therefore creating agents and minting is disabled until the asset manager controller notifies
        // the asset manager that it has been added.
        bool attached;
    }

    // diamond state access to state and settings

    bytes32 internal constant STATE_POSITION = keccak256("fasset.AssetManager.State");

    function get() internal pure returns (AssetManagerState.State storage _state) {
        // Only direct constants are allowed in inline assembly, so we assign it here
        bytes32 position = STATE_POSITION;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            _state.slot := position
        }
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;


library UnderlyingAddressOwnership {
    error InvalidAddressOwnershipProof();
    error EOAProofRequired();
    error AddressAlreadyClaimed();

    struct Ownership {
        address owner;

        // if not 0, there was a payment proof indicating this is externally owned account
        uint64 __underlyingBlockOfEOAProof; // only storage placeholder

        bool __provedEOA; // only storage placeholder
    }

    struct State {
        // mapping underlyingAddressHash => Ownership
        mapping (bytes32 => Ownership) ownership;
    }

    function claimAndTransfer(
        State storage _state,
        address _owner,
        bytes32 _underlyingAddressHash
    )
        internal
    {
        Ownership storage ownership = _state.ownership[_underlyingAddressHash];
        // check that currently unclaimed
        require(ownership.owner == address(0), AddressAlreadyClaimed());
        // set the new owner
        ownership.owner = _owner;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";


library CollateralType {
    enum Class {
        NONE,   // unused
        POOL,   // pool collateral type
        VAULT  // usable as vault collateral
    }

    // Collateral token is uniquely identified by the pair (collateralClass, token).
    struct Data {
        // The kind of collateral for this token.
        CollateralType.Class collateralClass;

        // The ERC20 token contract for this collateral type.
        IERC20 token;

        // Same as token.decimals(), when that exists.
        uint256 decimals;

        // Token invalidation time. Must be 0 on creation.
        uint256 validUntil;

        // When `true`, the FTSO with symbol `assetFtsoSymbol` returns asset price relative to this token
        // (such FTSO's will probably exist for major stablecoins).
        // When `false`, the FTSOs with symbols `assetFtsoSymbol` and `tokenFtsoSymbol` give asset and token
        // price relative to the same reference currency and the asset/token price is calculated as their ratio.
        bool directPricePair;

        // FTSO symbol for the asset, relative to this token or a reference currency
        // (it depends on the value of `directPricePair`).
        string assetFtsoSymbol;

        // FTSO symbol for this token in reference currency.
        // Used for asset/token price calculation when `directPricePair` is `false`.
        // Otherwise it is irrelevant to asset/token price calculation, but if it is nonempty,
        // it is still used in calculation of challenger and confirmation rewards
        // (otherwise we assume it approximates the value of USD and pay directly the USD amount in vault collateral).
        string tokenFtsoSymbol;

        // Minimum collateral ratio for healthy agents.
        uint256 minCollateralRatioBIPS;

        // Minimum collateral ratio required to get agent out of liquidation.
        // Will always be greater than minCollateralRatioBIPS.
        uint256 safetyMinCollateralRatioBIPS;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IPayment, IBalanceDecreasingTransaction}
    from "@flarenetwork/flare-periphery-contracts/flare/IFdcVerification.sol";


library PaymentConfirmations {
    error PaymentAlreadyConfirmed();

    struct State {
        // a store of payment hashes to prevent payment being used / challenged twice
        // structure: map of hash to the next hash in that day
        mapping(bytes32 => bytes32) verifiedPayments;
        // a linked list of payment hashes (one list per day) used for cleanup
        mapping(uint256 => bytes32) __verifiedPaymentsForDay; // only storage placeholder
        // first day number for which we are tracking verifications
        uint256 __verifiedPaymentsForDayStart; // only storage placeholder
    }

    /**
     * For payment transaction with non-unique payment reference (generated from address, not id),
     * we record `tx hash`, so that the same transaction can only be used once for payment.
     */
    function confirmIncomingPayment(
        State storage _state,
        IPayment.Proof calldata _payment
    )
        internal
    {
        _recordPaymentVerification(_state, _payment.data.requestBody.transactionId);
    }

    /**
     * For source decreasing transaction, we record `(source address, tx hash)` pair, since illegal
     * transactions on utxo chains can have multiple input addresses.
     */
    function confirmSourceDecreasingTransaction(
        State storage _state,
        IPayment.Proof calldata _payment
    )
        internal
    {
        bytes32 txKey = transactionKey(_payment.data.responseBody.sourceAddressHash,
            _payment.data.requestBody.transactionId);
        _recordPaymentVerification(_state, txKey);
    }

    /**
     * Check if source decreasing transaction was already confirmed.
     */
    function transactionConfirmed(
        State storage _state,
        IBalanceDecreasingTransaction.Proof calldata _transaction
    )
        internal view
        returns (bool)
    {
        bytes32 txKey = transactionKey(_transaction.data.responseBody.sourceAddressHash,
            _transaction.data.requestBody.transactionId);
        return _state.verifiedPayments[txKey] != 0;
    }

    // the same transaction hash could perform several underlying payments if it is smart contract
    // for now this is illegal, but might change for some smart contract chains
    // therefore the mapping key for transaction is always the combination of
    // underlying address (from which funds were removed) and transaction hash
    function transactionKey(bytes32 _underlyingSourceAddressHash, bytes32 _transactionHash)
        internal pure
        returns (bytes32)
    {
        return keccak256(abi.encode(_underlyingSourceAddressHash, _transactionHash));
    }

    function _recordPaymentVerification(
        State storage _state,
        bytes32 _txKey
    )
        private
    {
        require(_state.verifiedPayments[_txKey] == 0, PaymentAlreadyConfirmed());
        _state.verifiedPayments[_txKey] = _txKey; // any non-zero value is fine
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;


library RedemptionQueue {
    struct Ticket {
        address agentVault;
        uint64 valueAMG;
        uint64 prev;
        uint64 next;
        uint64 prevForAgent;
        uint64 nextForAgent;
    }

    struct AgentQueue {
        uint64 firstTicketId;
        uint64 lastTicketId;
    }

    struct State {
        mapping(uint64 => Ticket) tickets;      // mapping redemption_id=>ticket
        mapping(address => AgentQueue) agents;  // mapping address=>dl-list
        uint64 firstTicketId;
        uint64 lastTicketId;
        uint64 newTicketId;       // increment before assigning to ticket (to avoid 0)
    }

    function createRedemptionTicket(
        State storage _state,
        address _agentVault,
        uint64 _valueAMG
    )
        internal
        returns (uint64)
    {
        AgentQueue storage agent = _state.agents[_agentVault];
        uint64 ticketId = ++_state.newTicketId;   // pre-increment - id can never be 0
        // insert new ticket to the last place in global and agent redemption queues
        _state.tickets[ticketId] = Ticket({
            agentVault: _agentVault,
            valueAMG: _valueAMG,
            prev: _state.lastTicketId,
            next: 0,
            prevForAgent: agent.lastTicketId,
            nextForAgent: 0
        });
        // update links in global redemption queue
        if (_state.firstTicketId == 0) {
            assert(_state.lastTicketId == 0);    // empty queue - first and last must be 0
            _state.firstTicketId = ticketId;
        } else {
            assert(_state.lastTicketId != 0);    // non-empty queue - first and last must be non-zero
            _state.tickets[_state.lastTicketId].next = ticketId;
        }
        _state.lastTicketId = ticketId;
        // update links in agent redemption queue
        if (agent.firstTicketId == 0) {
            assert(agent.lastTicketId == 0);    // empty queue - first and last must be 0
            agent.firstTicketId = ticketId;
        } else {
            assert(agent.lastTicketId != 0);    // non-empty queue - first and last must be non-zero
            _state.tickets[agent.lastTicketId].nextForAgent = ticketId;
        }
        agent.lastTicketId = ticketId;
        // return the new redemption ticket's id
        return ticketId;
    }

    function deleteRedemptionTicket(
        State storage _state,
        uint64 _ticketId
    )
        internal
    {
        Ticket storage ticket = _state.tickets[_ticketId];
        assert(ticket.agentVault != address(0));
        AgentQueue storage agent = _state.agents[ticket.agentVault];
        // unlink from global queue
        if (ticket.prev == 0) {
            assert(_ticketId == _state.firstTicketId);     // ticket is first in queue
            _state.firstTicketId = ticket.next;
        } else {
            assert(_ticketId != _state.firstTicketId);     // ticket is not first in queue
            _state.tickets[ticket.prev].next = ticket.next;
        }
        if (ticket.next == 0) {
            assert(_ticketId == _state.lastTicketId);     // ticket is last in queue
            _state.lastTicketId = ticket.prev;
        } else {
            assert(_ticketId != _state.lastTicketId);     // ticket is not last in queue
            _state.tickets[ticket.next].prev = ticket.prev;
        }
        // unlink from agent queue
        if (ticket.prevForAgent == 0) {
            assert(_ticketId == agent.firstTicketId);     // ticket is first in agent queue
            agent.firstTicketId = ticket.nextForAgent;
        } else {
            assert(_ticketId != agent.firstTicketId);     // ticket is not first in agent queue
            _state.tickets[ticket.prevForAgent].nextForAgent = ticket.nextForAgent;
        }
        if (ticket.nextForAgent == 0) {
            assert(_ticketId == agent.lastTicketId);     // ticket is last in agent queue
            agent.lastTicketId = ticket.prevForAgent;
        } else {
            assert(_ticketId != agent.lastTicketId);     // ticket is not last in agent queue
            _state.tickets[ticket.nextForAgent].prevForAgent = ticket.prevForAgent;
        }
        // delete storage
        delete _state.tickets[_ticketId];
    }

    function getTicket(State storage _state, uint64 _id) internal view returns (Ticket storage) {
        return _state.tickets[_id];
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;


library CollateralReservation {
    enum Status {
        ACTIVE,         // the minting process hasn't finished yet
        SUCCESSFUL,     // the payment has been confirmed and the FAssets minted
        DEFAULTED,      // the payment has defaulted and the agent received the collateral reservation fee
        EXPIRED         // the confirmation time has expired and the agent called unstickMinting
    }

    struct Data {
        uint64 valueAMG;
        uint64 firstUnderlyingBlock;
        uint64 lastUnderlyingBlock;
        uint64 lastUnderlyingTimestamp;
        uint128 underlyingFeeUBA;
        uint128 reservationFeeNatWei;
        address agentVault;
        uint16 poolFeeShareBIPS;
        address minter;
        CollateralReservation.Status status;
        address payable executor;
        uint64 executorFeeNatGWei;
        uint64 __handshakeStartTimestamp; // only storage placeholder
        bytes32 __sourceAddressesRoot; // only storage placeholder
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

library Redemption {
    enum Status {
        EMPTY,      // redemption request with this id doesn't exist
        ACTIVE,     // waiting for confirmation/default
        DEFAULTED,  // default called, failed or late payment can still be confirmed
        // final statuses - there can be no valid payment for this redemption anymore
        SUCCESSFUL, // successful payment confirmed
        FAILED,     // payment failed
        BLOCKED,    // payment blocked
        REJECTED    // redemption request rejected due to invalid redeemer's address
    }

    struct Request {
        bytes32 redeemerUnderlyingAddressHash;
        uint128 underlyingValueUBA;
        uint128 underlyingFeeUBA;
        uint64 firstUnderlyingBlock;
        uint64 lastUnderlyingBlock;
        uint64 lastUnderlyingTimestamp;
        uint64 valueAMG;
        address redeemer;
        uint64 timestamp;
        address agentVault;
        Redemption.Status status;
        bool poolSelfClose;
        address payable executor;
        uint64 executorFeeNatGWei;
        uint64 __rejectionTimestamp; // only storage placeholder
        uint64 __takeOverTimestamp; // only storage placeholder
        string redeemerUnderlyingAddressString;
        bool transferToCoreVault;
        uint16 poolFeeShareBIPS;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {CollateralType} from "../../../userInterfaces/data/CollateralType.sol";

library CollateralTypeInt {
    struct Data {
        // The ERC20 token contract for this collateral type.
        // immutable
        IERC20 token;
        // The kind of collateral for this token.
        // immutable
        CollateralType.Class collateralClass;
        // Same as token.decimals(), when that exists.
        // immutable
        uint8 decimals;
        // If some token should not be used anymore as collateral, it has to be announced in advance and it
        // is still valid until this timestamp. After that time, the corresponding collateral is considered as
        // zero and the agents that haven't replaced it are liquidated.
        // When the invalidation has not been announced, this value is 0.
        uint64 validUntil;
        // When `true`, the FTSO with symbol `assetFtsoSymbol` returns asset price relative to this token
        // (such FTSO's will probably exist for major stablecoins).
        // When `false`, the FTSOs with symbols `assetFtsoSymbol` and `tokenFtsoSymbol` give asset and token
        // price relative to the same reference currency and the asset/token price is calculated as their ratio.
        // immutable
        bool directPricePair;
        // FTSO symbol for the asset, relative to this token or a reference currency
        // (it depends on the value of `directPricePair`).
        // immutable
        string assetFtsoSymbol;
        // FTSO symbol for this token in reference currency.
        // Used for asset/token price calculation when `directPricePair` is `false`.
        // Otherwise it is irrelevant to asset/token price calculation, but if it is nonempty,
        // it is still used in calculation of challenger and confirmation rewards
        // (otherwise we assume it approximates the value of USD and pay directly the USD amount in vault collateral).
        // immutable
        string tokenFtsoSymbol;
        // Minimum collateral ratio for healthy agents.
        // timelocked
        uint32 minCollateralRatioBIPS;
        // Minimum collateral ratio for agent in CCB (Collateral call band).
        // If the agent's collateral ratio is less than this, skip the CCB and go straight to liquidation.
        // A bit smaller than minCollateralRatioBIPS.
        // timelocked
        uint32 __ccbMinCollateralRatioBIPS; // only storage placeholder
        // Minimum collateral ratio required to get agent out of liquidation.
        // Will always be greater than minCollateralRatioBIPS.
        // timelocked
        uint32 safetyMinCollateralRatioBIPS;
    }
}


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS
// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

/**
 * @custom:name IBalanceDecreasingTransaction
 * @custom:id 0x02
 * @custom:supported BTC, DOGE, XRP
 * @author Flare
 * @notice A detection of a transaction that either decreases the balance for some address or is
 * signed by the source address.
 * Such an attestation could prove a violation of an agreement and therefore provides grounds to liquidate
 * some funds locked by a smart contract on Flare.
 *
 * A transaction is considered “balance decreasing” for the address, if the balance after the
 * transaction is lower than before or the address is among the signers of the transaction
 * (even if its balance is greater than before the transaction).
 * @custom:verification The transaction with `transactionId` is fetched from the API of the
 * source blockchain node or relevant indexer.
 * If the transaction cannot be fetched or the transaction is in a block that does not have a
 * sufficient number of confirmations, the attestation request is rejected.
 *
 * Once the transaction is received, the response fields are extracted if the transaction is balance
 * decreasing for the indicated address.
 * Some of the request and response fields are chain specific as described below.
 * The fields can be computed with the help of a balance decreasing summary.
 *
 * ### UTXO (Bitcoin and Dogecoin)
 *
 * - `sourceAddressIndicator` is the the index of the transaction input in hex padded to a 0x prefixed 32-byte string.
 * If the indicated input does not exist or the indicated input does not have the address,
 * the attestation request is rejected.
 * The `sourceAddress` is the address of the indicated transaction input.
 * - `spentAmount` is the sum of values of all inputs with sourceAddress minus the sum of
 * all outputs with `sourceAddress`.
 * Can be negative.
 * - `blockTimestamp` is the mediantime of a block.
 *
 * ### XRPL
 *
 * - `sourceAddressIndicator` is the standard address hash of the address whose balance has been decreased.
 * If the address indicated by `sourceAddressIndicator` is not among the signers of the transaction and the balance
 * of the address was not lowered in the transaction, the attestation request is rejected.
 *
 * - `spentAmount` is the difference between the balance of the indicated address after and before the transaction.
 * Can be negative.
 * - `blockTimestamp` is the close_time of a ledger converted to unix time.
 *
 * @custom:lut `blockTimestamp`
 * @custom:lutlimit `0x127500`, `0x127500`, `0x127500`
 */
interface IBalanceDecreasingTransaction {
    /**
     * @notice Toplevel request
     * @param attestationType ID of the attestation type.
     * @param sourceId ID of the data source.
     * @param messageIntegrityCode `MessageIntegrityCode` that is derived from the expected response.
     * @param requestBody Data defining the request. Type and interpretation is determined by the `attestationType`.
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
     * This is a security measure to prevent a collision of attestation hashes.
     * @param lowestUsedTimestamp The lowest timestamp used to generate the response.
     * @param requestBody Extracted from the request.
     * @param responseBody Data defining the response. The verification rules for the construction of the
     * response body and the type are defined per specific `attestationType`.
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
     * @notice Request body for IBalanceDecreasingTransaction attestation type
     * @param transactionId ID of the payment transaction.
     * @param sourceAddressIndicator The indicator of the address whose balance has been decreased.
     */
    struct RequestBody {
        bytes32 transactionId;
        bytes32 sourceAddressIndicator;
    }

    /**
     * @notice Response body for IBalanceDecreasingTransaction attestation type.
     * @param blockNumber The number of the block in which the transaction is included.
     * @param blockTimestamp The timestamp of the block in which the transaction is included.
     * @param sourceAddressHash Standard address hash of the address indicated by the `sourceAddressIndicator`.
     * @param spentAmount Amount spent by the source address in minimal units.
     * @param standardPaymentReference Standard payment reference of the transaction.
     */
    struct ResponseBody {
        uint64 blockNumber;
        uint64 blockTimestamp;
        bytes32 sourceAddressHash;
        int256 spentAmount;
        bytes32 standardPaymentReference;
    }
}

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


## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

