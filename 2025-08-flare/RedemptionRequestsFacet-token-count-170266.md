
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Math} from "@openzeppelin/contracts/utils/math/Math.sol";
import {IAddressValidity} from "@flarenetwork/flare-periphery-contracts/flare/IFdcVerification.sol";
import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {AssetManagerBase} from "./AssetManagerBase.sol";
import {ReentrancyGuard} from "../../openzeppelin/security/ReentrancyGuard.sol";
import {Agents} from "../library/Agents.sol";
import {AgentBacking} from "../library/AgentBacking.sol";
import {AgentPayout} from "../library/AgentPayout.sol";
import {Globals} from "../library/Globals.sol";
import {RedemptionRequests} from "../library/RedemptionRequests.sol";
import {Agent} from "../library/data/Agent.sol";
import {AssetManagerSettings} from "../../userInterfaces/data/AssetManagerSettings.sol";
import {SafePct} from "../../utils/library/SafePct.sol";
import {AssetManagerState} from "../library/data/AssetManagerState.sol";
import {IAssetManagerEvents} from "../../userInterfaces/IAssetManagerEvents.sol";
import {Conversion} from "../library/Conversion.sol";
import {Redemptions} from "../library/Redemptions.sol";
import {Liquidation} from "../library/Liquidation.sol";
import {TransactionAttestation} from "../library/TransactionAttestation.sol";
import {RedemptionQueue} from "../library/data/RedemptionQueue.sol";
import {Redemption} from "../library/data/Redemption.sol";


contract RedemptionRequestsFacet is AssetManagerBase, ReentrancyGuard {
    using SafePct for uint256;
    using SafeCast for uint256;
    using Agent for Agent.State;
    using RedemptionQueue for RedemptionQueue.State;

    error SelfCloseOfZero();
    error AddressValid();
    error WrongAddress();
    error InvalidRedemptionStatus();
    error RedemptionOfZero();
    error RedeemZeroLots();

    /**
     * Redeem (up to) `_lots` lots of f-assets. The corresponding amount of the f-assets belonging
     * to the redeemer will be burned and the redeemer will get paid by the agent in underlying currency
     * (or, in case of agent's payment default, by agent's collateral with a premium).
     * NOTE: in some cases not all sent f-assets can be redeemed (either there are not enough tickets or
     * more than a fixed limit of tickets should be redeemed). In this case only part of the approved assets
     * are burned and redeemed and the redeemer can execute this method again for the remaining lots.
     * In such case `RedemptionRequestIncomplete` event will be emitted, indicating the number of remaining lots.
     * Agent receives redemption request id and instructions for underlying payment in
     * RedemptionRequested event and has to pay `value - fee` and use the provided payment reference.
     * The agent can also reject the redemption request. In that case any other agent can take over the redemption.
     * If no agent takes over the redemption, the redeemer can request the default payment.
     * @param _lots number of lots to redeem
     * @param _redeemerUnderlyingAddressString the address to which the agent must transfer underlying amount
     * @param _executor the account that is allowed to execute redemption default (besides redeemer and agent)
     * @return _redeemedAmountUBA the actual redeemed amount; may be less than requested if there are not enough
     *      redemption tickets available or the maximum redemption ticket limit is reached
     */
    function redeem(
        uint256 _lots,
        string memory _redeemerUnderlyingAddressString,
        address payable _executor
    )
        external payable
        notEmergencyPaused
        nonReentrant
        returns (uint256 _redeemedAmountUBA)
    {
        uint256 maxRedeemedTickets = Globals.getSettings().maxRedeemedTickets;
        RedemptionRequests.AgentRedemptionList memory redemptionList = RedemptionRequests.AgentRedemptionList({
            length: 0,
            items: new RedemptionRequests.AgentRedemptionData[](maxRedeemedTickets)
        });
        uint256 redeemedLots = 0;
        for (uint256 i = 0; i < maxRedeemedTickets && redeemedLots < _lots; i++) {
            // redemption queue empty?
            if (AssetManagerState.get().redemptionQueue.firstTicketId == 0) {
                require(redeemedLots != 0, RedeemZeroLots());
                break;
            }
            // each loop, firstTicketId will change since we delete the first ticket
            redeemedLots += _redeemFirstTicket(_lots - redeemedLots, redemptionList);
        }
        uint256 executorFeeNatGWei = msg.value / Conversion.GWEI;
        for (uint256 i = 0; i < redemptionList.length; i++) {
            // distribute executor fee over redemption request with at most 1 gwei leftover
            uint256 currentExecutorFeeNatGWei = executorFeeNatGWei / (redemptionList.length - i);
            executorFeeNatGWei -= currentExecutorFeeNatGWei;
            RedemptionRequests.createRedemptionRequest(redemptionList.items[i], msg.sender,
                _redeemerUnderlyingAddressString, false, _executor, currentExecutorFeeNatGWei.toUint64(), 0, false);
        }
        // notify redeemer of incomplete requests
        if (redeemedLots < _lots) {
            emit IAssetManagerEvents.RedemptionRequestIncomplete(msg.sender, _lots - redeemedLots);
        }
        // burn the redeemed value of fassets
        uint256 redeemedUBA = Conversion.convertLotsToUBA(redeemedLots);
        Redemptions.burnFAssets(msg.sender, redeemedUBA);
        return redeemedUBA;
    }

    /**
     * Create a redemption from a single agent. Used in self-close exit from the collateral pool.
     * Note: only collateral pool can call this method.
     */
    function redeemFromAgent(
        address _agentVault,
        address _receiver,
        uint256 _amountUBA,
        string memory _receiverUnderlyingAddress,
        address payable _executor
    )
        external payable
        notEmergencyPaused
        nonReentrant
    {
        Agent.State storage agent = Agent.get(_agentVault);
        Agents.requireCollateralPool(agent);
        require(_amountUBA != 0, RedemptionOfZero());
        // close redemption tickets
        uint64 amountAMG = Conversion.convertUBAToAmg(_amountUBA);
        (uint64 closedAMG, uint256 closedUBA) = Redemptions.closeTickets(agent, amountAMG, false);
        // create redemption request
        RedemptionRequests.AgentRedemptionData memory redemption =
            RedemptionRequests.AgentRedemptionData(_agentVault, closedAMG);
        RedemptionRequests.createRedemptionRequest(redemption, _receiver, _receiverUnderlyingAddress, true,
            _executor, (msg.value / Conversion.GWEI).toUint64(), 0, false);
        // burn the closed assets
        Redemptions.burnFAssets(msg.sender, closedUBA);
    }

    /**
     * Burn fassets from  a single agent and get paid in vault collateral by the agent.
     * Price is FTSO price, multiplied by factor buyFAssetByAgentFactorBIPS (set by agent).
     * Used in self-close exit from the collateral pool when requested or when self-close amount is less than 1 lot.
     * Note: only collateral pool can call this method.
     */
    function redeemFromAgentInCollateral(
        address _agentVault,
        address _receiver,
        uint256 _amountUBA
    )
        external
        notEmergencyPaused
        nonReentrant
    {
        Agent.State storage agent = Agent.get(_agentVault);
        Agents.requireCollateralPool(agent);
        require(_amountUBA != 0, RedemptionOfZero());
        // close redemption tickets
        uint64 amountAMG = Conversion.convertUBAToAmg(_amountUBA);
        (uint64 closedAMG, uint256 closedUBA) = Redemptions.closeTickets(agent, amountAMG, true);
        // pay in collateral
        uint256 priceAmgToWei = Conversion.currentAmgPriceInTokenWei(agent.vaultCollateralIndex);
        uint256 paymentWei = Conversion.convertAmgToTokenWei(closedAMG, priceAmgToWei)
            .mulBips(agent.buyFAssetByAgentFactorBIPS);
        AgentPayout.payoutFromVault(agent, _receiver, paymentWei);
        emit IAssetManagerEvents.RedeemedInCollateral(_agentVault, _receiver, closedUBA, paymentWei);
        // burn the closed assets
        Redemptions.burnFAssets(msg.sender, closedUBA);
    }

    /**
     * To avoid unlimited work, the maximum number of redemption tickets closed in redemption, self close
     * or liquidation is limited. This means that a single redemption/self close/liquidation is limited.
     * This function calculates the maximum single redemption amount.
     */
    function maxRedemptionFromAgent(
        address _agentVault
    )
        external view
        returns (uint256)
    {
        Agent.State storage agent = Agent.get(_agentVault);
        AssetManagerState.State storage state = AssetManagerState.get();
        uint64 maxRedemptionAMG = agent.dustAMG;
        uint256 maxRedeemedTickets = Globals.getSettings().maxRedeemedTickets;
        uint64 ticketId = state.redemptionQueue.agents[agent.vaultAddress()].firstTicketId;
        for (uint256 i = 0; ticketId != 0 && i < maxRedeemedTickets; i++) {
            RedemptionQueue.Ticket storage ticket = state.redemptionQueue.getTicket(ticketId);
            maxRedemptionAMG += ticket.valueAMG;
            ticketId = ticket.nextForAgent;
        }
        return Conversion.convertAmgToUBA(maxRedemptionAMG);
    }

    /**
     * If the redeemer provides invalid address, the agent should provide the proof of address invalidity from the
     * Flare data connector. With this, the agent's obligations are fulfilled and they can keep the underlying.
     * NOTE: may only be called by the owner of the agent vault in the redemption request
     * NOTE: also checks that redeemer's address is normalized, so the redeemer must normalize their address,
     *   otherwise it will be rejected!
     * @param _proof proof that the address is invalid
     * @param _redemptionRequestId id of an existing redemption request
     */
    function rejectInvalidRedemption(
        IAddressValidity.Proof calldata _proof,
        uint256 _redemptionRequestId
    )
        external
        nonReentrant
    {
        Redemption.Request storage request = Redemptions.getRedemptionRequest(_redemptionRequestId, true);
        assert(!request.transferToCoreVault);   // we have a problem if core vault has invalid address
        Agent.State storage agent = Agent.get(request.agentVault);
        // check status
        require(request.status == Redemption.Status.ACTIVE, InvalidRedemptionStatus());
        // only owner can call
        Agents.requireAgentVaultOwner(agent);
        // check proof
        TransactionAttestation.verifyAddressValidity(_proof);
        // the actual redeemer's address must be validated
        bytes32 addressHash = keccak256(bytes(_proof.data.requestBody.addressStr));
        require(addressHash == request.redeemerUnderlyingAddressHash, WrongAddress());
        // and the address must be invalid or not normalized
        bool valid = _proof.data.responseBody.isValid &&
            _proof.data.responseBody.standardAddressHash == request.redeemerUnderlyingAddressHash;
        require(!valid, AddressValid());
        // release agent collateral
        AgentBacking.endRedeemingAssets(agent, request.valueAMG, request.poolSelfClose);
        // burn the executor fee
        Redemptions.burnExecutorFee(request);
        // emit event
        uint256 valueUBA = Conversion.convertAmgToUBA(request.valueAMG);
        emit IAssetManagerEvents.RedemptionRejected(request.agentVault, request.redeemer,
            _redemptionRequestId, valueUBA);
        // finish redemption request at end
        Redemptions.finishRedemptionRequest(_redemptionRequestId, request, Redemption.Status.REJECTED);
    }

    /**
     * Agent can "redeem against himself" by calling selfClose, which burns agent's own f-assets
     * and unlocks agent's collateral. The underlying funds backing the f-assets are released
     * as agent's free underlying funds and can be later withdrawn after announcement.
     * NOTE: may only be called by the agent vault owner.
     * @param _agentVault agent vault address
     * @param _amountUBA amount of f-assets to self-close
     * @return _closedAmountUBA the actual self-closed amount, may be less than requested if there are not enough
     *      redemption tickets available or the maximum redemption ticket limit is reached
     */
    function selfClose(
        address _agentVault,
        uint256 _amountUBA
    )
        external
        notEmergencyPaused
        nonReentrant
        onlyAgentVaultOwner(_agentVault)
        returns (uint256 _closedAmountUBA)
    {
        Agent.State storage agent = Agent.get(_agentVault);
        require(_amountUBA != 0, SelfCloseOfZero());
        uint64 amountAMG = Conversion.convertUBAToAmg(_amountUBA);
        (, uint256 closedUBA) = Redemptions.closeTickets(agent, amountAMG, true);
        // burn the self-closed assets
        Redemptions.burnFAssets(msg.sender, closedUBA);
        // try to pull agent out of liquidation
        Liquidation.endLiquidationIfHealthy(agent);
        // send event
        emit IAssetManagerEvents.SelfClose(_agentVault, closedUBA);
        return closedUBA;
    }

    /**
     * After a lot size change by the governance, it may happen that after a redemption
     * there remains less than one lot on a redemption ticket. This is named "dust" and
     * can be self closed or liquidated, but not redeemed. However, after several such redemptions,
     * the total dust can amount to more than one lot. Using this method, the amount, rounded down
     * to a whole number of lots, can be converted to a new redemption ticket.
     * NOTE: we do NOT check that the caller is the agent vault owner, since we want to
     * allow anyone to convert dust to tickets to increase asset fungibility.
     * @param _agentVault agent vault address
     */
    function convertDustToTicket(
        address _agentVault
    )
        external
        nonReentrant
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        Agent.State storage agent = Agent.get(_agentVault);
        // if dust is more than 1 lot, create a new redemption ticket
        if (agent.dustAMG >= settings.lotSizeAMG) {
            uint64 remainingDustAMG = agent.dustAMG % settings.lotSizeAMG;
            uint64 ticketValueAMG = agent.dustAMG - remainingDustAMG;
            AgentBacking.createRedemptionTicket(agent, ticketValueAMG);
            AgentBacking.changeDust(agent, remainingDustAMG);
        }
    }

    function _redeemFirstTicket(
        uint256 _lots,
        RedemptionRequests.AgentRedemptionList memory _list
    )
        private
        returns (uint256 _redeemedLots)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        uint64 ticketId = state.redemptionQueue.firstTicketId;
        if (ticketId == 0) {
            return 0;    // empty redemption queue
        }
        RedemptionQueue.Ticket storage ticket = state.redemptionQueue.getTicket(ticketId);
        address agentVault = ticket.agentVault;
        Agent.State storage agent = Agent.get(agentVault);
        uint256 maxRedeemLots = (ticket.valueAMG + agent.dustAMG) / settings.lotSizeAMG;
        _redeemedLots = Math.min(_lots, maxRedeemLots);
        if (_redeemedLots > 0) {
            uint64 redeemedAMG = Conversion.convertLotsToAMG(_redeemedLots);
            // find list index for ticket's agent
            uint256 index = 0;
            while (index < _list.length && _list.items[index].agentVault != agentVault) {
                ++index;
            }
            // add to list item or create new item
            if (index < _list.length) {
                _list.items[index].valueAMG = _list.items[index].valueAMG + redeemedAMG;
            } else {
                _list.items[_list.length++] = RedemptionRequests.AgentRedemptionData({
                    agentVault: agentVault,
                    valueAMG: redeemedAMG
                });
            }
            // _removeFromTicket may delete ticket data, so we call it at end
            Redemptions.removeFromTicket(ticketId, redeemedAMG);
        } else {
            // this will just convert ticket to dust
            Redemptions.removeFromTicket(ticketId, 0);
        }
    }
}
END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {SafePct} from "../../utils/library/SafePct.sol";
import {Math} from "@openzeppelin/contracts/utils/math/Math.sol";
import {AssetManagerState} from "./data/AssetManagerState.sol";
import {Collateral} from "./data/Collateral.sol";
import {Globals} from "./Globals.sol";
import {Conversion} from "./Conversion.sol";
import {Agent} from "./data/Agent.sol";
import {RedemptionQueue} from "./data/RedemptionQueue.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {CollateralTypeInt} from "./data/CollateralTypeInt.sol";
import {IWNat} from "../../flareSmartContracts/interfaces/IWNat.sol";
import {AgentInfo} from "../../userInterfaces/data/AgentInfo.sol";

library Agents {
    using SafeCast for uint256;
    using SafePct for uint256;
    using Agent for Agent.State;
    using RedemptionQueue for RedemptionQueue.State;

    error AgentNotWhitelisted();
    error OnlyAgentVaultOwner();
    error OnlyCollateralPool();


    function getAllAgents(
        uint256 _start,
        uint256 _end
    )
        internal view
        returns (address[] memory _agents, uint256 _totalLength)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        _totalLength = state.allAgents.length;
        _end = Math.min(_end, _totalLength);
        _start = Math.min(_start, _end);
        _agents = new address[](_end - _start);
        for (uint256 i = _start; i < _end; i++) {
            _agents[i - _start] = state.allAgents[i];
        }
    }

    function getAgentStatus(
        Agent.State storage _agent
    )
        internal view
        returns (AgentInfo.Status)
    {
        Agent.Status status = _agent.status;
        if (status == Agent.Status.NORMAL) {
            return AgentInfo.Status.NORMAL;
        } else if (status == Agent.Status.LIQUIDATION) {
            return AgentInfo.Status.LIQUIDATION;
        } else if (status == Agent.Status.FULL_LIQUIDATION) {
            return AgentInfo.Status.FULL_LIQUIDATION;
        } else if (status == Agent.Status.DESTROYING) {
            return AgentInfo.Status.DESTROYING;
        } else {
            assert (status == Agent.Status.DESTROYED);
            return AgentInfo.Status.DESTROYED;
        }
    }

    function isOwner(
        Agent.State storage _agent,
        address _address
    )
        internal view
        returns (bool)
    {
        return _address == _agent.ownerManagementAddress || _address == getWorkAddress(_agent);
    }

    function getWorkAddress(Agent.State storage _agent)
        internal view
        returns (address)
    {
        return Globals.getAgentOwnerRegistry().getWorkAddress(_agent.ownerManagementAddress);
    }

    function getOwnerPayAddress(Agent.State storage _agent)
        internal view
        returns (address payable)
    {
        address workAddress = getWorkAddress(_agent);
        return workAddress != address(0) ? payable(workAddress) : payable(_agent.ownerManagementAddress);
    }

    function requireWhitelisted(
        address _ownerManagementAddress
    )
        internal view
    {
        require(Globals.getAgentOwnerRegistry().isWhitelisted(_ownerManagementAddress),
            AgentNotWhitelisted());
    }

    function requireWhitelistedAgentVaultOwner(
        Agent.State storage _agent
    )
        internal view
    {
        requireWhitelisted(_agent.ownerManagementAddress);
    }

    function requireAgentVaultOwner(
        address _agentVault
    )
        internal view
    {
        require(isOwner(Agent.get(_agentVault), msg.sender), OnlyAgentVaultOwner());
    }

    function requireAgentVaultOwner(
        Agent.State storage _agent
    )
        internal view
    {
        require(isOwner(_agent, msg.sender), OnlyAgentVaultOwner());
    }

    function requireCollateralPool(
        Agent.State storage _agent
    )
        internal view
    {
        require(msg.sender == address(_agent.collateralPool), OnlyCollateralPool());
    }

    function isCollateralToken(
        Agent.State storage _agent,
        IERC20 _token
    )
        internal view
        returns (bool)
    {
        return _token == getPoolWNat(_agent) || _token == getVaultCollateralToken(_agent);
    }

    function getVaultCollateralToken(Agent.State storage _agent)
        internal view
        returns (IERC20)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        return state.collateralTokens[_agent.vaultCollateralIndex].token;
    }

    function getVaultCollateral(Agent.State storage _agent)
        internal view
        returns (CollateralTypeInt.Data storage)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        return state.collateralTokens[_agent.vaultCollateralIndex];
    }

    function convertUSD5ToVaultCollateralWei(Agent.State storage _agent, uint256 _amountUSD5)
        internal view
        returns (uint256)
    {
        return Conversion.convertFromUSD5(_amountUSD5, getVaultCollateral(_agent));
    }

    function getPoolWNat(Agent.State storage _agent)
        internal view
        returns (IWNat)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        return IWNat(address(state.collateralTokens[_agent.poolCollateralIndex].token));
    }

    function getPoolCollateral(Agent.State storage _agent)
        internal view
        returns (CollateralTypeInt.Data storage)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        return state.collateralTokens[_agent.poolCollateralIndex];
    }

    function getCollateral(Agent.State storage _agent, Collateral.Kind _kind)
        internal view
        returns (CollateralTypeInt.Data storage)
    {
        assert (_kind != Collateral.Kind.AGENT_POOL);   // there is no agent pool collateral token
        AssetManagerState.State storage state = AssetManagerState.get();
        if (_kind == Collateral.Kind.VAULT) {
            return state.collateralTokens[_agent.vaultCollateralIndex];
        } else {
            return state.collateralTokens[_agent.poolCollateralIndex];
        }
    }

    function collateralUnderwater(Agent.State storage _agent, Collateral.Kind _kind)
        internal view
        returns (bool)
    {
        if (_kind == Collateral.Kind.VAULT) {
            return (_agent.collateralsUnderwater & Agent.LF_VAULT) != 0;
        } else {
            // AGENT_POOL collateral cannot be underwater (it only affects minting),
            // so this function will only be used for VAULT and POOL
            assert(_kind == Collateral.Kind.POOL);
            return (_agent.collateralsUnderwater & Agent.LF_POOL) != 0;
        }
    }

    function withdrawalAnnouncement(Agent.State storage _agent, Collateral.Kind _kind)
        internal view
        returns (Agent.WithdrawalAnnouncement storage)
    {
        assert (_kind != Collateral.Kind.POOL);     // agent cannot withdraw from pool
        return _kind == Collateral.Kind.VAULT
            ? _agent.vaultCollateralWithdrawalAnnouncement
            : _agent.poolTokenWithdrawalAnnouncement;
    }

    function totalBackedAMG(Agent.State storage _agent)
        internal view
        returns (uint64)
    {
        // this must always hold, so assert it is true, otherwise the following line
        // would need `max(redeemingAMG, poolRedeemingAMG)`
        assert(_agent.poolRedeemingAMG <= _agent.redeemingAMG);
        return _agent.mintedAMG + _agent.reservedAMG + _agent.redeemingAMG;
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;


library SafeMath64 {
    uint256 internal constant MAX_UINT64 = type(uint64).max;
    int256 internal constant MAX_INT64 = type(int64).max;

    error ConversionOverflow();
    error NegativeValue();

    // 64 bit signed/unsigned conversion

    function toUint64(int256 a) internal pure returns (uint64) {
        require(a >= 0, NegativeValue());
        require(a <= int256(MAX_UINT64), ConversionOverflow());
        return uint64(uint256(a));
    }

    function toInt64(uint256 a) internal pure returns (int64) {
        require(a <= uint256(MAX_INT64), ConversionOverflow());
        return int64(int256(a));
    }

    function max64(uint64 a, uint64 b) internal pure returns (uint64) {
        return a >= b ? a : b;
    }

    function min64(uint64 a, uint64 b) internal pure returns (uint64) {
        return a <= b ? a : b;
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

import {AssetManagerState} from "./data/AssetManagerState.sol";
import {IAssetManagerEvents} from "../../userInterfaces/IAssetManagerEvents.sol";
import {Globals} from "./Globals.sol";
import {Conversion} from "./Conversion.sol";
import {Agent} from "./data/Agent.sol";
import {RedemptionQueue} from "./data/RedemptionQueue.sol";
import {AssetManagerSettings} from "../../userInterfaces/data/AssetManagerSettings.sol";


library AgentBacking {
    using Agent for Agent.State;
    using RedemptionQueue for RedemptionQueue.State;

    function releaseMintedAssets(
        Agent.State storage _agent,
        uint64 _valueAMG
    )
        internal
    {
        _agent.mintedAMG = _agent.mintedAMG - _valueAMG;
    }

    function startRedeemingAssets(
        Agent.State storage _agent,
        uint64 _valueAMG,
        bool _poolSelfCloseRedemption
    )
        internal
    {
        _agent.redeemingAMG += _valueAMG;
        if (!_poolSelfCloseRedemption) {
            _agent.poolRedeemingAMG += _valueAMG;
        }
        releaseMintedAssets(_agent, _valueAMG);
    }

    function endRedeemingAssets(
        Agent.State storage _agent,
        uint64 _valueAMG,
        bool _poolSelfCloseRedemption
    )
        internal
    {
        _agent.redeemingAMG = _agent.redeemingAMG - _valueAMG;
        if (!_poolSelfCloseRedemption) {
            _agent.poolRedeemingAMG = _agent.poolRedeemingAMG - _valueAMG;
        }
    }

    function createNewMinting(
        Agent.State storage _agent,
        uint64 _valueAMG
    )
        internal
    {
        // allocate minted assets
        _agent.mintedAMG += _valueAMG;

        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // Add value with dust, then take the whole number of lots from it to create the new ticket,
        // and the remainder as new dust. At the end, there will always be less than 1 lot of dust left.
        uint64 valueWithDustAMG = _agent.dustAMG + _valueAMG;
        uint64 newDustAMG = valueWithDustAMG % settings.lotSizeAMG;
        uint64 ticketValueAMG = valueWithDustAMG - newDustAMG;
        // create ticket and change dust
        if (ticketValueAMG > 0) {
            createRedemptionTicket(_agent, ticketValueAMG);
        }
        changeDust(_agent, newDustAMG);
    }

    function createRedemptionTicket(
        Agent.State storage _agent,
        uint64 _ticketValueAMG
    )
        internal
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        if (_ticketValueAMG == 0) return;
        address vaultAddress = _agent.vaultAddress();
        uint64 lastTicketId = state.redemptionQueue.lastTicketId;
        RedemptionQueue.Ticket storage lastTicket = state.redemptionQueue.getTicket(lastTicketId);
        if (lastTicket.agentVault == vaultAddress) {
            // last ticket is from the same agent - merge the new ticket with the last
            lastTicket.valueAMG += _ticketValueAMG;
            uint256 ticketValueUBA = Conversion.convertAmgToUBA(lastTicket.valueAMG);
            emit IAssetManagerEvents.RedemptionTicketUpdated(vaultAddress, lastTicketId, ticketValueUBA);
        } else {
            // either queue is empty or the last ticket belongs to another agent - create new ticket
            uint64 ticketId = state.redemptionQueue.createRedemptionTicket(vaultAddress, _ticketValueAMG);
            uint256 ticketValueUBA = Conversion.convertAmgToUBA(_ticketValueAMG);
            emit IAssetManagerEvents.RedemptionTicketCreated(vaultAddress, ticketId, ticketValueUBA);
        }
    }

    function changeDust(
        Agent.State storage _agent,
        uint64 _newDustAMG
    )
        internal
    {
        if (_agent.dustAMG == _newDustAMG) return;
        _agent.dustAMG = _newDustAMG;
        uint256 dustUBA = Conversion.convertAmgToUBA(_newDustAMG);
        emit IAssetManagerEvents.DustChanged(_agent.vaultAddress(), dustUBA);
    }

    function decreaseDust(
        Agent.State storage _agent,
        uint64 _dustDecreaseAMG
    )
        internal
    {
        uint64 newDustAMG = _agent.dustAMG - _dustDecreaseAMG;
        changeDust(_agent, newDustAMG);
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Math} from "@openzeppelin/contracts/utils/math/Math.sol";
import {IIAgentVault} from "../../agentVault/interfaces/IIAgentVault.sol";
import {Globals} from "./Globals.sol";
import {Agent} from "./data/Agent.sol";
import {CollateralTypeInt} from "./data/CollateralTypeInt.sol";
import {AssetManagerSettings} from "../../userInterfaces/data/AssetManagerSettings.sol";
import {Agents} from "./Agents.sol";


library AgentPayout {
    using Agent for Agent.State;

    function payoutFromVault(
        Agent.State storage _agent,
        address _receiver,
        uint256 _amountWei
    )
        internal
        returns (uint256 _amountPaid)
    {
        CollateralTypeInt.Data storage collateral = Agents.getVaultCollateral(_agent);
        // don't want the calling method to fail due to too small balance for payout
        IIAgentVault vault = IIAgentVault(_agent.vaultAddress());
        _amountPaid = Math.min(_amountWei, collateral.token.balanceOf(address(vault)));
        vault.payout(collateral.token, _receiver, _amountPaid);
    }

    function tryPayoutFromVault(
        Agent.State storage _agent,
        address _receiver,
        uint256 _amountWei
    )
        internal
        returns (bool _success, uint256 _amountPaid)
    {
        CollateralTypeInt.Data storage collateral = Agents.getVaultCollateral(_agent);
        // don't want the calling method to fail due to too small balance for payout
        IIAgentVault vault = IIAgentVault(_agent.vaultAddress());
        _amountPaid = Math.min(_amountWei, collateral.token.balanceOf(address(vault)));
        try vault.payout(collateral.token, _receiver, _amountPaid) {
            _success = true;
        } catch {
            _success = false;
            _amountPaid = 0;
        }
    }

    function payoutFromPool(
        Agent.State storage _agent,
        address _receiver,
        uint256 _amountWei,
        uint256 _agentResponsibilityWei
    )
        internal
        returns (uint256 _amountPaid)
    {
        // don't want the calling method to fail due to too small balance for payout
        uint256 poolBalance = _agent.collateralPool.totalCollateral();
        _amountPaid = Math.min(_amountWei, poolBalance);
        _agentResponsibilityWei = Math.min(_agentResponsibilityWei, _amountPaid);
        _agent.collateralPool.payout(_receiver, _amountPaid, _agentResponsibilityWei);
    }

    function payForConfirmationByOthers(
        Agent.State storage _agent,
        address _receiver
    )
        internal
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        uint256 amount = Agents.convertUSD5ToVaultCollateralWei(_agent, settings.confirmationByOthersRewardUSD5);
        payoutFromVault(_agent, _receiver, amount);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IERC20Permit} from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Permit.sol";
import {IERC20Metadata} from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {IERC5267} from "@openzeppelin/contracts/interfaces/IERC5267.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import {IIFAsset} from "../interfaces/IIFAsset.sol";
import {ERC20Permit} from "../../openzeppelin/token/ERC20Permit.sol";
import {CheckPointable} from "./CheckPointable.sol";
import {IAssetManager} from "../../userInterfaces/IAssetManager.sol";
import {IICleanable} from "@flarenetwork/flare-periphery-contracts/flare/token/interfaces/IICleanable.sol";
import {IFAsset} from "../../userInterfaces/IFAsset.sol";
import {IICheckPointable} from "../interfaces/IICheckPointable.sol";


contract FAsset is IIFAsset, IERC165, ERC20, CheckPointable, UUPSUpgradeable, ERC20Permit {
    error OnlyAssetManager();
    error AlreadyInitialized();
    error AlreadyUpgraded();
    error OnlyDeployer();
    error ZeroAssetManager();
    error CannotReplaceAssetManager();
    error OnlyCleanupBlockManager();
    error FAssetTerminated();
    error FAssetBalanceTooLow();
    error CannotTransferToSelf();
    error EmergencyPauseOfTransfersActive();

    /**
     * The name of the underlying asset.
     */
    string public override assetName;

    /**
     * The symbol of the underlying asset.
     */
    string public override assetSymbol;

    /**
     * The contract that is allowed to set cleanupBlockNumber.
     * Usually this will be an instance of CleanupBlockNumberManager.
     */
    address public cleanupBlockNumberManager;

    /**
     * Get the asset manager, corresponding to this fAsset.
     * fAssets and asset managers are in 1:1 correspondence.
     */
    address public override assetManager;

    uint64 private __terminatedAt; // only storage placeholder

    string private _name;
    string private _symbol;
    uint8 private _decimals;

    // the address that created this contract and is allowed to set initial settings
    address private _deployer;
    bool private _initialized;
    uint16 private _version;

    modifier onlyAssetManager() {
        require(msg.sender == assetManager, OnlyAssetManager());
        _;
    }

    constructor()
        ERC20("", "")
    {
        _initialized = true;
        _version = 1000;
    }

    function initialize(
        string memory name_,
        string memory symbol_,
        string memory assetName_,
        string memory assetSymbol_,
        uint8 decimals_
    )
        external
    {
        require(!_initialized, AlreadyInitialized());
        _initialized = true;
        _deployer = msg.sender;
        _name = name_;
        _symbol = symbol_;
        _decimals = decimals_;
        assetName = assetName_;
        assetSymbol = assetSymbol_;
        initializeV1r1();
    }

    function initializeV1r1() public {
        require(_version == 0, AlreadyUpgraded());
        _version = 1;
        initializeEIP712(_name, "1");
    }

    /**
     * Set asset manager contract this can be done only once and must be just after deploy
     * (otherwise nothing can be minted).
     */
    function setAssetManager(address _assetManager)
        external
    {
        require (msg.sender == _deployer, OnlyDeployer());
        require(_assetManager != address(0), ZeroAssetManager());
        require(assetManager == address(0), CannotReplaceAssetManager());
        assetManager = _assetManager;
    }

    /**
     * Mints `_amount` od fAsset.
     * Only the assetManager corresponding to this fAsset may call `mint()`.
     */
    function mint(address _owner, uint256 _amount)
        external override
        onlyAssetManager
    {
        _mint(_owner, _amount);
    }

    /**
     * Burns `_amount` od fAsset.
     * Only the assetManager corresponding to this fAsset may call `burn()`.
     */
    function burn(address _owner, uint256 _amount)
        external override
        onlyAssetManager
    {
        _burn(_owner, _amount);
    }

    /**
     * Returns the name of the token.
     */
    function name() public view virtual override(ERC20, IERC20Metadata) returns (string memory) {
        return _name;
    }

    /**
     * Returns the symbol of the token, usually a shorter version of the name.
     */
    function symbol() public view virtual override(ERC20, IERC20Metadata) returns (string memory) {
        return _symbol;
    }
    /**
     * Implements IERC20Metadata method and returns configurable number of decimals.
     */
    function decimals() public view virtual override(ERC20, IERC20Metadata) returns (uint8) {
        return _decimals;
    }

    /**
     * Set the cleanup block number.
     * Historic data for the blocks before `cleanupBlockNumber` can be erased,
     * history before that block should never be used since it can be inconsistent.
     * In particular, cleanup block number must be before current vote power block.
     * @param _blockNumber The new cleanup block number.
     */
    function setCleanupBlockNumber(uint256 _blockNumber)
        external override
    {
        require(msg.sender == cleanupBlockNumberManager, OnlyCleanupBlockManager());
        _setCleanupBlockNumber(_blockNumber);
    }

    /**
     * Get the current cleanup block number.
     */
    function cleanupBlockNumber()
        external view override
        returns (uint256)
    {
        return _cleanupBlockNumber();
    }

    /**
     * Set the contract that is allowed to call history cleaning methods.
     */
    function setCleanerContract(address _cleanerContract)
        external override
        onlyAssetManager
    {
        _setCleanerContract(_cleanerContract);
    }

    /**
     * Set the contract that is allowed to set cleanupBlockNumber.
     * Usually this will be an instance of CleanupBlockNumberManager.
     */
    function setCleanupBlockNumberManager(address _cleanupBlockNumberManager)
        external
        onlyAssetManager
    {
        cleanupBlockNumberManager = _cleanupBlockNumberManager;
    }

    function _beforeTokenTransfer(address _from, address _to, uint256 _amount)
        internal override
    {
        require(_from == address(0) || balanceOf(_from) >= _amount, FAssetBalanceTooLow());
        require(_from != _to, CannotTransferToSelf());
        // mint and redeem are allowed on transfer pause, but not transfer
        require(_from == address(0) || _to == address(0) || !IAssetManager(assetManager).transfersEmergencyPaused(),
            EmergencyPauseOfTransfersActive());
        // update balance history
        _updateBalanceHistoryAtTransfer(_from, _to, _amount);
    }

    /**
     * Implementation of ERC-165 interface.
     */
    function supportsInterface(bytes4 _interfaceId)
        external pure override
        returns (bool)
    {
        return _interfaceId == type(IERC165).interfaceId
            || _interfaceId == type(IERC20).interfaceId
            || _interfaceId == type(IERC20Metadata).interfaceId
            || _interfaceId == type(IERC5267).interfaceId
            || _interfaceId == type(IERC20Permit).interfaceId
            || _interfaceId == type(IICheckPointable).interfaceId
            || _interfaceId == type(IFAsset).interfaceId
            || _interfaceId == type(IIFAsset).interfaceId
            || _interfaceId == type(IICleanable).interfaceId;
    }

    // support for ERC20Permit
    function _approve(address _owner, address _spender, uint256 _amount)
        internal virtual override (ERC20, ERC20Permit)
    {
        ERC20._approve(_owner, _spender, _amount);
    }

    ////////////////////////////////////////////////////////////////////////////////////
    // UUPS proxy upgrade

    function implementation() external view returns (address) {
        return _getImplementation();
    }

    /**
     * Upgrade calls can only arrive through asset manager.
     * See UUPSUpgradeable._authorizeUpgrade.
     */
    function _authorizeUpgrade(address /* _newImplementation */)
        internal virtual override
        onlyAssetManager
    { // solhint-disable-line no-empty-blocks
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;


library PaymentReference {
    uint256 private constant TYPE_SHIFT = 192;
    uint256 private constant TYPE_MASK = ((1 << 64) - 1) << TYPE_SHIFT;
    uint256 private constant LOW_BITS_MASK = (1 << TYPE_SHIFT) - 1;
    uint256 private constant ID_RANDOMIZATION = 1000;
    uint256 private constant MAX_ID = (1 << 64) - 1;

    // common prefix 0x464250526641 = hex('FBPRfA' - Flare Bridge Payment Reference / fAsset)

    uint256 internal constant MINTING = 0x4642505266410001 << TYPE_SHIFT;
    uint256 internal constant REDEMPTION = 0x4642505266410002 << TYPE_SHIFT;
    uint256 internal constant ANNOUNCED_WITHDRAWAL = 0x4642505266410003 << TYPE_SHIFT;
    uint256 internal constant RETURN_FROM_CORE_VAULT = 0x4642505266410004 << TYPE_SHIFT;
    uint256 internal constant REDEMPTION_FROM_CORE_VAULT = 0x4642505266410005 << TYPE_SHIFT;
    uint256 internal constant TOPUP = 0x4642505266410011 << TYPE_SHIFT;
    uint256 internal constant SELF_MINT = 0x4642505266410012 << TYPE_SHIFT;

    // create various payment references

    function minting(uint256 _id) internal pure returns (bytes32) {
        assert(_id <= MAX_ID);
        return bytes32(_id | MINTING);
    }

    function redemption(uint256 _id) internal pure returns (bytes32) {
        assert(_id <= MAX_ID);
        return bytes32(_id | REDEMPTION);
    }

    function announcedWithdrawal(uint256 _id) internal pure returns (bytes32) {
        assert(_id <= MAX_ID);
        return bytes32(_id | ANNOUNCED_WITHDRAWAL);
    }

    function returnFromCoreVault(uint256 _id) internal pure returns (bytes32) {
        assert(_id <= MAX_ID);
        return bytes32(_id | RETURN_FROM_CORE_VAULT);
    }

    function redemptionFromCoreVault(uint256 _id) internal pure returns (bytes32) {
        assert(_id <= MAX_ID);
        return bytes32(_id | REDEMPTION_FROM_CORE_VAULT);
    }

    function topup(address _agentVault) internal pure returns (bytes32) {
        return bytes32(uint256(uint160(_agentVault)) | TOPUP);
    }

    function selfMint(address _agentVault) internal pure returns (bytes32) {
        return bytes32(uint256(uint160(_agentVault)) | SELF_MINT);
    }

    // verify and decode payment references

    function isValid(bytes32 _reference, uint256 _type) internal pure returns (bool) {
        uint256 refType = uint256(_reference) & TYPE_MASK;
        uint256 refLowBits = uint256(_reference) & LOW_BITS_MASK;
        // for valid reference, type must match and low bits may never be 0 (are either id or address)
        return refType == _type && refLowBits != 0;
    }

    function decodeId(bytes32 _reference) internal pure returns (uint256) {
        return uint256(_reference) & LOW_BITS_MASK;
    }

    function randomizedIdSkip() internal view returns (uint64) {
        // This is rather weak randomization, but it's ok for the purpose of preventing speculative underlying
        // payments, since there is only one guess possible - the first mistake makes agent liquidated.
        //slither-disable-next-line weak-prng
        return uint64(block.number % ID_RANDOMIZATION + 1);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {SafeMath64} from "../../../utils/library/SafeMath64.sol";


library RedemptionTimeExtension {
    using SafeCast for uint256;
    using SafeMath64 for uint64;

    struct AgentTimeExtensionData {
        uint64 extendedTimestamp;
    }

    struct State {
        // settings
        uint64 redemptionPaymentExtensionSeconds;

        // per agent state
        mapping(address _agentVault => AgentTimeExtensionData) agents;
    }

    /**
     * Calculates the redemption time extension when there are multiple redemption requests in short time.
     * Implements "leaky bucket" algorithm, popular in rate-limiters.
     * @param _agentVault the agent vault address being redeemed
     */
    function extendTimeForRedemption(address _agentVault)
        internal
        returns (uint64)
    {
        State storage state = getState();
        AgentTimeExtensionData storage agentData = state.agents[_agentVault];
        uint64 timestamp = block.timestamp.toUint64();
        uint64 accumulatedTimestamp = agentData.extendedTimestamp + state.redemptionPaymentExtensionSeconds;
        agentData.extendedTimestamp = SafeMath64.max64(accumulatedTimestamp, timestamp);
        return agentData.extendedTimestamp - timestamp;
    }

    function setRedemptionPaymentExtensionSeconds(uint256 _value)
        internal
    {
        State storage state = getState();
        state.redemptionPaymentExtensionSeconds = _value.toUint64();
    }

    function redemptionPaymentExtensionSeconds()
        internal view
        returns (uint256)
    {
        State storage state = getState();
        return state.redemptionPaymentExtensionSeconds;
    }

    bytes32 internal constant STATE_POSITION = keccak256("fasset.RedemptionTimeExtension.State");

    function getState()
        internal pure
        returns (State storage _state)
    {
        bytes32 position = STATE_POSITION;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            _state.slot := position
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {EnumerableSet} from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import {IICollateralPool} from "../../../collateralPool/interfaces/IICollateralPool.sol";


library Agent {
    error InvalidAgentVaultAddress();

    enum Status {
        EMPTY,              // agent does not exist
        NORMAL,
        LIQUIDATION,        // liquidation due to CR - ends when agent is healthy
        FULL_LIQUIDATION,   // illegal payment liquidation - must liquidate all and close vault
        DESTROYING,         // agent announced destroy, cannot mint again
        DESTROYED           // agent has been destroyed, cannot do anything except return info
    }

    // For agents to withdraw NAT collateral, they must first announce it and then wait
    // withdrawalAnnouncementSeconds.
    // The announced amount cannot be used as collateral for minting during that time.
    // This makes sure that agents cannot just remove all collateral if they are challenged.
    struct WithdrawalAnnouncement {
        // Announce amount in collateral token's minimum unit (wei).
        uint128 amountWei;

        // The timestamp when withdrawal can be executed.
        uint64 allowedAt;
    }

    // Struct to store agent's pending setting updates.
    struct SettingUpdate {
        uint128 value;
        uint64 validAt;
    }

    struct State {
        IICollateralPool collateralPool;

        // Address of the agent owner. This is the management address, which is immutable.
        // The work address can be retrieved from the global state mapping between
        // management and work addresses.
        address ownerManagementAddress;

        // Current underlying address for this agent vault.
        // The address is immutable.
        string underlyingAddressString;

        // `underlyingAddressString` is only used for sending the minter a correct payment address;
        // for matching payment addresses we always use `underlyingAddressHash = keccak256(underlyingAddressString)`
        bytes32 underlyingAddressHash;

        // Current status of the agent (changes for liquidation).
        Agent.Status status;

        // Index of collateral vault token.
        // The data is obtained as state.collateralTokens[vaultCollateralIndex].
        uint16 vaultCollateralIndex;

        // Index of token in collateral pool. This is always wrapped FLR/SGB, however the wrapping
        // contract (WNat) may change. In such case we add new collateral token with class POOL but the
        // agent must call a method to upgrade to new contract, se we must track the actual token used.
        uint16 poolCollateralIndex;

        // Position of this agent in the list of agents available for minting.
        // Value is actually `list index + 1`, so that 0 means 'not in the list'.
        uint32 availableAgentsPos;

        // Minting fee in BIPS (collected in underlying currency).
        uint16 feeBIPS;

        // Share of the minting fee that goes to the pool as percentage of the minting fee.
        uint16 poolFeeShareBIPS;

        // Collateral ratio at which we calculate locked collateral and collateral available for minting.
        // Agent may set own value for minting collateral ratio when entering the available agent list,
        // but it must always be greater than minimum collateral ratio.
        uint32 mintingVaultCollateralRatioBIPS;

        // Collateral ratio at which we calculate locked collateral and collateral available for minting.
        // Agent may set own value for minting collateral ratio when entering the available agent list,
        // but it must always be greater than minimum collateral ratio.
        uint32 mintingPoolCollateralRatioBIPS;

        // Timestamp of the startLiquidation (or liquidate) call.
        uint64 liquidationStartedAt;

        // Liquidation phase at the time when liquidation started.
        uint8 __initialLiquidationPhase; // only storage placeholder

        // Bitmap signifying which collateral type(s) triggered liquidation (LF_VAULT | LF_POOL).
        uint8 collateralsUnderwater;

        // Amount of collateral locked by collateral reservation.
        uint64 reservedAMG;

        // Amount of collateral backing minted fassets.
        uint64 mintedAMG;

        // The amount of fassets being redeemed. In this case, the fassets were already burned,
        // but the collateral must still be locked to allow payment in case of redemption failure.
        // The distinction between 'minted' and 'redeemed' assets is important in case of challenge.
        uint64 redeemingAMG;

        // The amount of fassets being redeemed EXCEPT those from pool self-close exits.
        // Unlike normal redemption, pool collateral was already withdrawn, so the redeeming collateral
        // must only be accounted for / locked for vault collateral.
        // On redemption payment failure, redeemer will be paid only in vault collateral in this case
        // (and will be paid less if there isn't enough - small extra risk for pool token holders).
        // There will always be `poolRedeemingAMG <= redeemingAMG`.
        uint64 poolRedeemingAMG;

        // When lot size changes, there may be some leftover after redemption that doesn't fit
        // a whole lot size. It is added to dustAMG and can be recovered via self-close.
        // Unlike redeemingAMG, dustAMG is still counted in the mintedAMG.
        uint64 dustAMG;

        // The amount of funds that on the agent's underlying address.
        // If it is higher than the amount needed to back mintings, it can be withdrawn after announcement.
        // It is signed int, because unreported deposits combined with other operations can in principle
        // make it negative. We could truncate it at 0, but if deposit report comes later, this would make
        // the value wrong.
        int128 underlyingBalanceUBA;

        // There can be only one announced underlying withdrawal per agent active at any time.
        // This variable holds the id, or 0 if there is no announced underlying withdrawal going on.
        uint64 announcedUnderlyingWithdrawalId;

        // The time when ongoing underlying withdrawal was announced.
        uint64 underlyingWithdrawalAnnouncedAt;

        // Announcement for vault collateral withdrawal.
        WithdrawalAnnouncement vaultCollateralWithdrawalAnnouncement;

        // Announcement for pool token withdrawal (which also means pool collateral withdrawal).
        WithdrawalAnnouncement poolTokenWithdrawalAnnouncement;

        // Underlying block when the agent was created.
        // Challenger's should track underlying address activity since this block
        // and topups are only valid after this block (both inclusive).
        uint64 underlyingBlockAtCreation;

        // The time when ongoing agent vault destroy was announced.
        uint64 destroyAllowedAt;

        // The factor set by the agent to multiply the price at which agent buys f-assets from pool
        // token holders on self-close exit (when requested or the redeemed amount is less than 1 lot).
        uint16 buyFAssetByAgentFactorBIPS;

        // The announced time when the agent is exiting available agents list.
        uint64 exitAvailableAfterTs;

        // The position of the agent in the list of all agents.
        uint32 allAgentsPos;

        // Agent's pending setting updates.
        mapping(bytes32 => SettingUpdate) settingUpdates;

        // Agent's handshake type - minting or redeeming can be rejected.
        // 0 - no verification, 1 - manual verification, ...
        uint32 __handshakeType; // only storage placeholder

        // There can only be one transfer to core vault per agent active at any time.
        uint64 activeTransferToCoreVault;

        // the request id of the active return from core vault
        uint64 activeReturnFromCoreVaultId;

        // part of the agent's reservedAMG for the core vault return
        uint64 returnFromCoreVaultReservedAMG;

        // The redemption fee share paid to the pool (as FAssets).
        // In redemption dominated situations (when agent requests return from core vault to earn
        // from redemption fees), pool can get some share to make it sustainable for pool users.
        // NOTE: the pool fee share is locked at the redemption request time, but is charged at the redemption
        // confirmation time. If agent uses all the redemption fee for transaction fees, this could make the
        // agent's free underlying balance negative.
        uint16 redemptionPoolFeeShareBIPS;

        EnumerableSet.AddressSet alwaysAllowedMinters;

        // Only used for calculating Agent.State size. See deleteStorage() below.
        uint256[1] _endMarker;
    }

    // underwater collateral classes
    uint8 internal constant LF_VAULT = 1 << 0;
    uint8 internal constant LF_POOL = 1 << 1;

    // diamond state accessors

    bytes32 internal constant AGENTS_POSITION = keccak256("fasset.AssetManager.Agent");

    // only return valid agent - fail if status is EMPTY or DESTROYED
    function get(address _address)
        internal view
        returns (Agent.State storage)
    {
        Agent.State storage agent = getWithoutCheck(_address);
        Agent.Status status = agent.status;
        require(status != Agent.Status.EMPTY && status != Agent.Status.DESTROYED, InvalidAgentVaultAddress());
        return agent;
    }

    // Like get, but only fail if status is EMPTY.
    // This is useful for reading agent info after the agent has been destroyed.
    function getAllowDestroyed(address _address)
        internal view
        returns (Agent.State storage)
    {
        Agent.State storage agent = getWithoutCheck(_address);
        require(agent.status != Agent.Status.EMPTY, InvalidAgentVaultAddress());
        return agent;
    }

    function getWithoutCheck(address _address)
        internal pure
        returns (Agent.State storage _agent)
    {
        bytes32 position = bytes32(uint256(AGENTS_POSITION) ^ (uint256(uint160(_address)) << 64));
        // solhint-disable-next-line no-inline-assembly
        assembly {
            _agent.slot := position
        }
    }

    function vaultAddress(Agent.State storage _agent)
        internal pure
        returns (address)
    {
        bytes32 position;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            position := _agent.slot
        }
        return address(uint160((uint256(position) ^ uint256(AGENTS_POSITION)) >> 64));
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Math} from "@openzeppelin/contracts/utils/math/Math.sol";
import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {SafePct} from "../../utils/library/SafePct.sol";
import {MathUtils} from "../../utils/library/MathUtils.sol";
import {IAssetManagerEvents} from "../../userInterfaces/IAssetManagerEvents.sol";
import {Agents} from "./Agents.sol";
import {Conversion} from "./Conversion.sol";
import {AgentCollateral} from "./AgentCollateral.sol";
import {Agent} from "./data/Agent.sol";
import {Collateral} from "./data/Collateral.sol";
import {CollateralTypeInt} from "./data/CollateralTypeInt.sol";
import {CollateralTypes} from "./CollateralTypes.sol";


library Liquidation {
    using SafeCast for uint256;
    using MathUtils for uint256;
    using SafePct for uint256;
    using Agent for Agent.State;
    using Agents for Agent.State;

    struct CRData {
        uint256 vaultCR;
        uint256 poolCR;
        uint256 amgToC1WeiPrice;
        uint256 amgToPoolWeiPrice;
    }

    // Start full agent liquidation (Agent.Status.FULL_LIQUIDATION)
    function startFullLiquidation(
        Agent.State storage _agent
    )
        internal
    {
        // if already in full liquidation or destroying, do nothing
        if (_agent.status == Agent.Status.FULL_LIQUIDATION
            || _agent.status == Agent.Status.DESTROYING) return;
        if (_agent.liquidationStartedAt == 0) {
            _agent.liquidationStartedAt = block.timestamp.toUint64();
        }
        _agent.status = Agent.Status.FULL_LIQUIDATION;
        emit IAssetManagerEvents.FullLiquidationStarted(_agent.vaultAddress(), block.timestamp);
    }

    // Cancel liquidation if the agent is healthy.
    function endLiquidationIfHealthy(
        Agent.State storage _agent
    )
        internal
    {
        // can only stop plain liquidation (full liquidation can only stop when there are no more minted assets)
        if (_agent.status != Agent.Status.LIQUIDATION) return;
        // agent's current collateral ratio
        CRData memory cr = getCollateralRatiosBIPS(_agent);
        // target ratio is minCollateralRatioBIPS if collateral not underwater, otherwise safetyMinCollateralRatioBIPS
        uint256 targetRatioVaultCollateralBIPS = _targetRatioBIPS(_agent, Collateral.Kind.VAULT);
        uint256 targetRatioPoolBIPS = _targetRatioBIPS(_agent, Collateral.Kind.POOL);
        // if agent is safe, restore status to NORMAL
        if (cr.vaultCR >= targetRatioVaultCollateralBIPS && cr.poolCR >= targetRatioPoolBIPS) {
            _agent.status = Agent.Status.NORMAL;
            _agent.liquidationStartedAt = 0;
            _agent.collateralsUnderwater = 0;
            emit IAssetManagerEvents.LiquidationEnded(_agent.vaultAddress());
        }
    }

    function getCollateralRatiosBIPS(
        Agent.State storage _agent
    )
        internal view
        returns (CRData memory)
    {
        (uint256 vaultCR, uint256 amgToC1WeiPrice) = getCollateralRatioBIPS(_agent, Collateral.Kind.VAULT);
        (uint256 poolCR, uint256 amgToPoolWeiPrice) = getCollateralRatioBIPS(_agent, Collateral.Kind.POOL);
        return CRData({
            vaultCR: vaultCR,
            poolCR: poolCR,
            amgToC1WeiPrice: amgToC1WeiPrice,
            amgToPoolWeiPrice: amgToPoolWeiPrice
        });
    }

    // The collateral ratio (BIPS) for deciding whether agent is in liquidation is the maximum
    // of the ratio calculated from FTSO price and the ratio calculated from trusted voters' price.
    // In this way, liquidation due to bad FTSO providers bunching together is less likely.
    function getCollateralRatioBIPS(
        Agent.State storage _agent,
        Collateral.Kind _collateralKind
    )
        internal view
        returns (uint256 _collateralRatioBIPS, uint256 _amgToTokenWeiPrice)
    {
        (Collateral.Data memory _data, Collateral.Data memory _trustedData) =
            _collateralDataWithTrusted(_agent, _collateralKind);
        uint256 ratio = AgentCollateral.collateralRatioBIPS(_data, _agent);
        uint256 ratioTrusted = AgentCollateral.collateralRatioBIPS(_trustedData, _agent);
        _amgToTokenWeiPrice = _data.amgToTokenWeiPrice;
        _collateralRatioBIPS = Math.max(ratio, ratioTrusted);
    }

    // Calculate the amount of liquidation that gets agent to safety.
    // assumed: agentStatus == LIQUIDATION/FULL_LIQUIDATION
    function maxLiquidationAmountAMG(
        Agent.State storage _agent,
        uint256 _collateralRatioBIPS,
        uint256 _factorBIPS,
        Collateral.Kind _collateralKind
    )
        internal view
        returns (uint256)
    {
        // for full liquidation, all minted amount can be liquidated
        if (_agent.status == Agent.Status.FULL_LIQUIDATION) {
            return _agent.mintedAMG;
        }
        // otherwise, liquidate just enough to get agent to safety
        uint256 targetRatioBIPS = _targetRatioBIPS(_agent, _collateralKind);
        if (targetRatioBIPS <= _collateralRatioBIPS) {
            return 0;               // agent already safe
        }
        if (_collateralRatioBIPS <= _factorBIPS) {
            return _agent.mintedAMG; // cannot achieve target - liquidate all
        }
        uint256 maxLiquidatedAMG = AgentCollateral.totalBackedAMG(_agent, _collateralKind)
            .mulDivRoundUp(targetRatioBIPS - _collateralRatioBIPS, targetRatioBIPS - _factorBIPS);
        return Math.min(maxLiquidatedAMG, _agent.mintedAMG);
    }

    function _targetRatioBIPS(
        Agent.State storage _agent,
        Collateral.Kind _collateralKind
    )
        private view
        returns (uint256)
    {
        CollateralTypeInt.Data storage collateral = _agent.getCollateral(_collateralKind);
        if (!_agent.collateralUnderwater(_collateralKind)) {
            return collateral.minCollateralRatioBIPS;
        } else {
            return collateral.safetyMinCollateralRatioBIPS;
        }
    }

    // Used for calculating liquidation collateral ratio.
    function _collateralDataWithTrusted(
        Agent.State storage _agent,
        Collateral.Kind _kind
    )
        private view
        returns (Collateral.Data memory _data, Collateral.Data memory _trustedData)
    {
        CollateralTypeInt.Data storage collateral = _agent.getCollateral(_kind);
        uint256 fullCollateral = _getCollateralAmount(_agent, _kind, collateral);
        (uint256 price, uint256 trusted) = Conversion.currentAmgPriceInTokenWeiWithTrusted(collateral);
        _data = Collateral.Data({ kind: _kind, fullCollateral: fullCollateral, amgToTokenWeiPrice: price });
        _trustedData = Collateral.Data({ kind: _kind, fullCollateral: fullCollateral, amgToTokenWeiPrice: trusted });
    }

    function _getCollateralAmount(
        Agent.State storage _agent,
        Collateral.Kind _kind,
        CollateralTypeInt.Data storage collateral
    )
        private view
        returns (uint256)
    {
        if (!CollateralTypes.isValid(collateral)) {
            // A simple way to force agents still holding expired collateral tokens into liquidation is just to
            // set fullCollateral for expired types to 0.
            // This will also make sure all liquidation payments are in the other collateral type.
            return 0;
        } else if (_kind == Collateral.Kind.POOL) {
            // Return tracked collateral amount in the pool.
            return _agent.collateralPool.totalCollateral();
        } else {
            // Return amount of vault collateral.
            return collateral.token.balanceOf(_agent.vaultAddress());
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {SafeMath64} from "../../utils/library/SafeMath64.sol";
import {Transfers} from "../../utils/library/Transfers.sol";
import {AssetManagerState} from "./data/AssetManagerState.sol";
import {Conversion} from "./Conversion.sol";
import {AgentBacking} from "./AgentBacking.sol";
import {Agent} from "../../assetManager/library/data/Agent.sol";
import {RedemptionQueue} from "./data/RedemptionQueue.sol";
import {Redemption} from "./data/Redemption.sol";
import {Globals} from "./Globals.sol";
import {IAssetManagerEvents} from "../../userInterfaces/IAssetManagerEvents.sol";


library Redemptions {
    using Agent for Agent.State;
    using RedemptionQueue for RedemptionQueue.State;

    error InvalidRequestId();

    function closeTickets(
        Agent.State storage _agent,
        uint64 _amountAMG,
        bool _immediatelyReleaseMinted
    )
        internal
        returns (uint64 _closedAMG, uint256 _closedUBA)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        // redemption tickets
        uint256 maxRedeemedTickets = Globals.getSettings().maxRedeemedTickets;
        uint64 lotSize = Globals.getSettings().lotSizeAMG;
        for (uint256 i = 0; i < maxRedeemedTickets && _closedAMG < _amountAMG; i++) {
            // each loop, firstTicketId will change since we delete the first ticket
            uint64 ticketId = state.redemptionQueue.agents[_agent.vaultAddress()].firstTicketId;
            if (ticketId == 0) {
                break;  // no more tickets for this agent
            }
            RedemptionQueue.Ticket storage ticket = state.redemptionQueue.getTicket(ticketId);
            uint64 maxTicketRedeemAMG = ticket.valueAMG + _agent.dustAMG;
            maxTicketRedeemAMG -= maxTicketRedeemAMG % lotSize; // round down to whole lots
            uint64 ticketRedeemAMG = SafeMath64.min64(_amountAMG - _closedAMG, maxTicketRedeemAMG);
            // only remove from tickets and add to total, do everything else after the loop
            removeFromTicket(ticketId, ticketRedeemAMG);
            _closedAMG += ticketRedeemAMG;
        }
        // now close the dust if anything remains (e.g. if there were not enough tickets to redeem)
        uint64 closeDustAMG = SafeMath64.min64(_amountAMG - _closedAMG, _agent.dustAMG);
        if (closeDustAMG > 0) {
            _closedAMG += closeDustAMG;
            AgentBacking.decreaseDust(_agent, closeDustAMG);
        }
        // self-close or liquidation is one step, so we can release minted assets without redeeming step
        if (_immediatelyReleaseMinted) {
            AgentBacking.releaseMintedAssets(_agent, _closedAMG);
        }
        // return
        _closedUBA = Conversion.convertAmgToUBA(_closedAMG);
    }

    function removeFromTicket(
        uint64 _redemptionTicketId,
        uint64 _redeemedAMG
    )
        internal
    {
        RedemptionQueue.State storage redemptionQueue = AssetManagerState.get().redemptionQueue;
        RedemptionQueue.Ticket storage ticket = redemptionQueue.getTicket(_redemptionTicketId);
        Agent.State storage agent = Agent.get(ticket.agentVault);
        uint64 lotSize = Globals.getSettings().lotSizeAMG;
        uint64 remainingAMG = ticket.valueAMG + agent.dustAMG - _redeemedAMG;
        uint64 remainingAMGDust = remainingAMG % lotSize;
        uint64 remainingAMGLots = remainingAMG - remainingAMGDust;
        if (remainingAMGLots == 0) {
            redemptionQueue.deleteRedemptionTicket(_redemptionTicketId);
            emit IAssetManagerEvents.RedemptionTicketDeleted(agent.vaultAddress(), _redemptionTicketId);
        } else if (remainingAMGLots != ticket.valueAMG) {
            ticket.valueAMG = remainingAMGLots;
            uint256 remainingUBA = Conversion.convertAmgToUBA(remainingAMGLots);
            emit IAssetManagerEvents.RedemptionTicketUpdated(agent.vaultAddress(), _redemptionTicketId, remainingUBA);
        }
        AgentBacking.changeDust(agent, remainingAMGDust);
    }

    function burnFAssets(
        address _owner,
        uint256 _amountUBA
    )
        internal
    {
        Globals.getFAsset().burn(_owner, _amountUBA);
    }

    // pay executor for executor calls in WNat, otherwise burn executor fee
    function payOrBurnExecutorFee(
        Redemption.Request storage _request
    )
        internal
    {
        uint256 executorFeeNatWei = _request.executorFeeNatGWei * Conversion.GWEI;
        if (executorFeeNatWei > 0) {
            _request.executorFeeNatGWei = 0;
            if (msg.sender == _request.executor) {
                Transfers.depositWNat(Globals.getWNat(), _request.executor, executorFeeNatWei);
            } else {
                Globals.getBurnAddress().transfer(executorFeeNatWei);
            }
        }
    }

    // burn executor fee
    function burnExecutorFee(
        Redemption.Request storage _request
    )
        internal
    {
        uint256 executorFeeNatWei = _request.executorFeeNatGWei * Conversion.GWEI;
        if (executorFeeNatWei > 0) {
            _request.executorFeeNatGWei = 0;
            Globals.getBurnAddress().transfer(executorFeeNatWei);
        }
    }

    function reCreateRedemptionTicket(
        Agent.State storage _agent,
        Redemption.Request storage _request
    )
        internal
    {
        AgentBacking.endRedeemingAssets(_agent, _request.valueAMG, _request.poolSelfClose);
        AgentBacking.createNewMinting(_agent, _request.valueAMG);
    }

    function finishRedemptionRequest(
        uint256 _redemptionRequestId,
        Redemption.Request storage _request,
        Redemption.Status _status
    )
        internal
    {
        assert(_status >= Redemption.Status.SUCCESSFUL);    // must be a final status
        _request.status = _status;
        releaseTransferToCoreVault(_redemptionRequestId, _request);
    }

    function releaseTransferToCoreVault(
        uint256 _redemptionRequestId,
        Redemption.Request storage _request
    )
        internal
    {
        if (_request.transferToCoreVault) {
            Agent.State storage agent = Agent.get(_request.agentVault);
            if (agent.activeTransferToCoreVault == _redemptionRequestId) {
                agent.activeTransferToCoreVault = 0;
            }
        }
    }

    function getRedemptionRequest(
        uint256 _redemptionRequestId,
        bool _requireUnconfirmed
    )
        internal view
        returns (Redemption.Request storage _request)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        require(_redemptionRequestId != 0, InvalidRequestId());
        _request = state.redemptionRequests[_redemptionRequestId];
        if (_requireUnconfirmed) {
            require(isOpen(_request), InvalidRequestId());
        } else {
            require(_request.status != Redemption.Status.EMPTY, InvalidRequestId());
        }
    }

    // true if redemption is valid and has not been confirmed yet
    function isOpen(Redemption.Request storage _request)
        internal view
        returns (bool)
    {
        Redemption.Status status = _request.status;
        return status == Redemption.Status.ACTIVE || status == Redemption.Status.DEFAULTED;
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {SafePct} from "../../utils/library/SafePct.sol";
import {AssetManagerState} from "./data/AssetManagerState.sol";
import {RedemptionTimeExtension} from "./data/RedemptionTimeExtension.sol";
import {IAssetManagerEvents} from "../../userInterfaces/IAssetManagerEvents.sol";
import {Conversion} from "./Conversion.sol";
import {Redemption} from "./data/Redemption.sol";
import {Agent} from "./data/Agent.sol";
import {Globals} from "./Globals.sol";
import {AgentBacking} from "./AgentBacking.sol";
import {AssetManagerSettings} from "../../userInterfaces/data/AssetManagerSettings.sol";
import {PaymentReference} from "./data/PaymentReference.sol";


library RedemptionRequests {
    using SafePct for uint256;
    using SafeCast for uint256;

    error CannotRedeemToAgentsAddress();
    error UnderlyingAddressTooLong();
    error ExecutorFeeWithoutExecutor();

    struct AgentRedemptionData {
        address agentVault;
        uint64 valueAMG;
    }

    struct AgentRedemptionList {
        AgentRedemptionData[] items;
        uint256 length;
    }

    function createRedemptionRequest(
        AgentRedemptionData memory _data,
        address _redeemer,
        string memory _redeemerUnderlyingAddressString,
        bool _poolSelfClose,
        address payable _executor,
        uint64 _executorFeeNatGWei,
        uint64 _additionalPaymentTime,
        bool _transferToCoreVault
    )
        internal
        returns (uint64 _requestId)
    {
        require(_executorFeeNatGWei == 0 || _executor != address(0), ExecutorFeeWithoutExecutor());
        AssetManagerState.State storage state = AssetManagerState.get();
        Agent.State storage agent = Agent.get(_data.agentVault);
        // validate redemption address
        require(bytes(_redeemerUnderlyingAddressString).length < 128, UnderlyingAddressTooLong());
        bytes32 underlyingAddressHash = keccak256(bytes(_redeemerUnderlyingAddressString));
        // both addresses must be normalized (agent's address is checked at vault creation,
        // and if redeemer address isn't normalized, the agent can trigger rejectInvalidRedemption),
        // so this comparison quarantees the redemption is not to the agent's address
        require(underlyingAddressHash != agent.underlyingAddressHash,
            CannotRedeemToAgentsAddress());
        // create request
        uint128 redeemedValueUBA = Conversion.convertAmgToUBA(_data.valueAMG).toUint128();
        _requestId = _newRequestId(_poolSelfClose);
        // create in-memory request and then put it to storage to not go out-of-stack
        Redemption.Request memory request;
        request.redeemerUnderlyingAddressHash = underlyingAddressHash;
        request.underlyingValueUBA = redeemedValueUBA;
        request.firstUnderlyingBlock = state.currentUnderlyingBlock;
        (request.lastUnderlyingBlock, request.lastUnderlyingTimestamp) =
            _lastPaymentBlock(_data.agentVault, _additionalPaymentTime);
        request.timestamp = block.timestamp.toUint64();
        request.underlyingFeeUBA = _transferToCoreVault ?
            0 : uint256(redeemedValueUBA).mulBips(Globals.getSettings().redemptionFeeBIPS).toUint128();
        request.redeemer = _redeemer;
        request.agentVault = _data.agentVault;
        request.valueAMG = _data.valueAMG;
        request.status = Redemption.Status.ACTIVE;
        request.poolSelfClose = _poolSelfClose;
        request.executor = _executor;
        request.executorFeeNatGWei = _executorFeeNatGWei;
        request.redeemerUnderlyingAddressString = _redeemerUnderlyingAddressString;
        request.transferToCoreVault = _transferToCoreVault;
        request.poolFeeShareBIPS = agent.redemptionPoolFeeShareBIPS;
        state.redemptionRequests[_requestId] = request;
        // decrease mintedAMG and mark it to redeemingAMG
        // do not add it to freeBalance yet (only after failed redemption payment)
        AgentBacking.startRedeemingAssets(agent, _data.valueAMG, _poolSelfClose);
        // emit event to remind agent to pay
        _emitRedemptionRequestedEvent(request, _requestId, _redeemerUnderlyingAddressString);
    }

    function _emitRedemptionRequestedEvent(
        Redemption.Request memory _request,
        uint64 _requestId,
        string memory _redeemerUnderlyingAddressString
    )
        private
    {
        emit IAssetManagerEvents.RedemptionRequested(
            _request.agentVault,
            _request.redeemer,
            _requestId,
            _redeemerUnderlyingAddressString,
            _request.underlyingValueUBA,
            _request.underlyingFeeUBA,
            _request.firstUnderlyingBlock,
            _request.lastUnderlyingBlock,
            _request.lastUnderlyingTimestamp,
            PaymentReference.redemption(_requestId),
            _request.executor,
            _request.executorFeeNatGWei * Conversion.GWEI);
    }

    function _newRequestId(bool _poolSelfClose)
        private
        returns (uint64)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        uint64 nextRequestId = state.newRedemptionRequestId + PaymentReference.randomizedIdSkip();
        // the requestId will indicate in the lowest bit whether it is a pool self close redemption
        // (+1 is added so that the request id still increases after clearing lowest bit)
        uint64 requestId = ((nextRequestId + 1) & ~uint64(1)) | (_poolSelfClose ? 1 : 0);
        state.newRedemptionRequestId = requestId;
        return requestId;
    }

    function _lastPaymentBlock(address _agentVault, uint64 _additionalPaymentTime)
        private
        returns (uint64 _lastUnderlyingBlock, uint64 _lastUnderlyingTimestamp)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // timeshift amortizes for the time that passed from the last underlying block update;
        // it also adds redemption time extension when there are many redemption requests in short time
        uint64 timeshift = block.timestamp.toUint64() - state.currentUnderlyingBlockUpdatedAt
            + RedemptionTimeExtension.extendTimeForRedemption(_agentVault)
            + _additionalPaymentTime;
        uint64 blockshift = (uint256(timeshift) * 1000 / settings.averageBlockTimeMS).toUint64();
        _lastUnderlyingBlock =
            state.currentUnderlyingBlock + blockshift + settings.underlyingBlocksForPayment;
        _lastUnderlyingTimestamp =
            state.currentUnderlyingBlockTimestamp + timeshift + settings.underlyingSecondsForPayment;
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IIFAsset} from "../../fassetToken/interfaces/IIFAsset.sol";
import {IWNat} from "../../flareSmartContracts/interfaces/IWNat.sol";
import {AssetManagerSettings} from "../../userInterfaces/data/AssetManagerSettings.sol";
import {IAgentOwnerRegistry} from "../../userInterfaces/IAgentOwnerRegistry.sol";
import {AssetManagerState} from "./data/AssetManagerState.sol";
import {CollateralTypeInt} from "./data/CollateralTypeInt.sol";


// global state helpers
library Globals {
    bytes32 internal constant ASSET_MANAGER_SETTINGS_POSITION = keccak256("fasset.AssetManager.Settings");

    function getSettings()
        internal pure
        returns (AssetManagerSettings.Data storage _settings)
    {
        bytes32 position = ASSET_MANAGER_SETTINGS_POSITION;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            _settings.slot := position
        }
    }

    function getWNat()
        internal view
        returns (IWNat)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        return IWNat(address(state.collateralTokens[state.poolCollateralIndex].token));
    }

    function getPoolCollateral()
        internal view
        returns (CollateralTypeInt.Data storage)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        return state.collateralTokens[state.poolCollateralIndex];
    }

    function getFAsset()
        internal view
        returns (IIFAsset)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        return IIFAsset(settings.fAsset);
    }

    function getAgentOwnerRegistry()
        internal view
        returns (IAgentOwnerRegistry)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        return IAgentOwnerRegistry(settings.agentOwnerRegistry);
    }

    function getBurnAddress()
        internal view
        returns (address payable)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        return settings.burnAddress;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

/**
 * @dev Compute percentages safely without phantom overflows.
 *
 * Intermediate operations can overflow even when the result will always
 * fit into computed type. Developers usually
 * assume that overflows raise errors. `SafePct` restores this intuition by
 * reverting the transaction when such an operation overflows.
 *
 * Using this library instead of the unchecked operations eliminates an entire
 * class of bugs, so it's recommended to use it always.
 */
library SafePct {
    uint256 internal constant MAX_BIPS = 10_000;

    error DivisionByZero();

    /**
     * Calculates `floor(x * y / z)`, reverting on overflow, but only if the result overflows.
     * Requirement: intermediate operations must revert on overflow.
     */
    function mulDiv(uint256 x, uint256 y, uint256 z) internal pure returns (uint256) {
        require(z > 0, DivisionByZero());

        if (x == 0) return 0;
        unchecked {
            uint256 xy = x * y;
            if (xy / x == y) { // no overflow happened (works in unchecked)
                return xy / z;
            }
        }

        //slither-disable-next-line divide-before-multiply
        uint256 a = x / z;
        uint256 b = x % z; // x = a * z + b

        //slither-disable-next-line divide-before-multiply
        uint256 c = y / z;
        uint256 d = y % z; // y = c * z + d

        return (a * c * z) + (a * d) + (b * c) + (b * d / z);
    }

    /**
     * Calculates `ceiling(x * y / z)`.
     */
    function mulDivRoundUp(uint256 x, uint256 y, uint256 z) internal pure returns (uint256) {
        uint256 resultRoundDown = mulDiv(x, y, z);
        unchecked {
            // safe - if z == 0, above mulDiv call would revert
            uint256 remainder = mulmod(x, y, z);
            // safe - overflow only possible if z == 1, but then remainder == 0
            return remainder == 0 ? resultRoundDown : resultRoundDown + 1;
        }
    }

    /**
     * Return `x * y BIPS` = `x * y / 10_000`, rounded down.
     */
    function mulBips(uint256 x, uint256 y) internal pure returns (uint256) {
        return mulDiv(x, y, MAX_BIPS);
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {IPriceReader} from "../../ftso/interfaces/IPriceReader.sol";
import {SafePct} from "../../utils/library/SafePct.sol";
import {AssetManagerState} from "./data/AssetManagerState.sol";
import {Globals} from "./Globals.sol";
import {CollateralTypeInt} from "./data/CollateralTypeInt.sol";
import {AssetManagerSettings} from "../../userInterfaces/data/AssetManagerSettings.sol";


library Conversion {
    using SafePct for uint256;

    uint256 internal constant AMG_TOKEN_WEI_PRICE_SCALE_EXP = 9;
    uint256 internal constant AMG_TOKEN_WEI_PRICE_SCALE = 10 ** AMG_TOKEN_WEI_PRICE_SCALE_EXP;
    uint256 internal constant NAT_WEI = 1e18;
    uint256 internal constant GWEI = 1e9;

    function currentAmgPriceInTokenWei(
        uint256 _tokenType
    )
        internal view
        returns (uint256 _price)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        (_price,,) = currentAmgPriceInTokenWeiWithTs(state.collateralTokens[_tokenType], false);
    }

    function currentAmgPriceInTokenWei(
        CollateralTypeInt.Data storage _token
    )
        internal view
        returns (uint256 _price)
    {
        (_price,,) = currentAmgPriceInTokenWeiWithTs(_token, false);
    }

    function currentAmgPriceInTokenWeiWithTrusted(
        CollateralTypeInt.Data storage _token
    )
        internal view
        returns (uint256 _ftsoPrice, uint256 _trustedPrice)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        (uint256 ftsoPrice, uint256 assetTimestamp, uint256 tokenTimestamp) =
            currentAmgPriceInTokenWeiWithTs(_token, false);
        (uint256 trustedPrice, uint256 assetTimestampTrusted, uint256 tokenTimestampTrusted) =
            currentAmgPriceInTokenWeiWithTs(_token, true);
        bool trustedPriceFresh = tokenTimestampTrusted + settings.maxTrustedPriceAgeSeconds >= tokenTimestamp
                && assetTimestampTrusted + settings.maxTrustedPriceAgeSeconds >= assetTimestamp;
        _ftsoPrice = ftsoPrice;
        _trustedPrice = trustedPriceFresh ? trustedPrice : ftsoPrice;
    }

    function convertAmgToUBA(
        uint64 _valueAMG
    )
        internal view
        returns (uint256)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // safe multiplication - both values are 64 bit
        return uint256(_valueAMG) * settings.assetMintingGranularityUBA;
    }

    function convertUBAToAmg(
        uint256 _valueUBA
    )
        internal view
        returns (uint64)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        return SafeCast.toUint64(_valueUBA / settings.assetMintingGranularityUBA);
    }

    function roundUBAToAmg(
        uint256 _valueUBA
    )
        internal view
        returns (uint256)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        return _valueUBA - (_valueUBA % settings.assetMintingGranularityUBA);
    }

    function convertLotsToAMG(
        uint256 _lots
    )
        internal view
        returns (uint64)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        return SafeCast.toUint64(_lots * settings.lotSizeAMG);
    }

    function convertLotsToUBA(
        uint256 _lots
    )
        internal view
        returns (uint256)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // this should not overflow - all values are 64 bit (except _lots which is limited by minted lots)
        return _lots * settings.lotSizeAMG * settings.assetMintingGranularityUBA;
    }

    function convert(
        uint256 _amount,
        CollateralTypeInt.Data storage _fromToken,
        CollateralTypeInt.Data storage _toToken
    )
        internal view
        returns (uint256)
    {
        uint256 priceMul = currentAmgPriceInTokenWei(_toToken);
        uint256 priceDiv = currentAmgPriceInTokenWei(_fromToken);
        return _amount.mulDiv(priceMul, priceDiv);
    }

    function convertFromUSD5(
        uint256 _amountUSD5,
        CollateralTypeInt.Data storage _token
    )
        internal view
        returns (uint256)
    {
        // if tokenFtsoSymbol is empty, it is assumed that the token is a USD-like stablecoin
        // so `_amountUSD5` is (approximately) the correct amount of tokens
        if (bytes(_token.tokenFtsoSymbol).length == 0) {
            return _amountUSD5;
        }
        (uint256 tokenPrice,, uint256 tokenFtsoDec) = readFtsoPrice(_token.tokenFtsoSymbol, false);
        // 5 is for 5 decimals of USD5
        uint256 expPlus = _token.decimals + tokenFtsoDec - 5;
        return _amountUSD5.mulDiv(10 ** expPlus, tokenPrice);
    }

    function currentAmgPriceInTokenWeiWithTs(
        CollateralTypeInt.Data storage _token,
        bool _fromTrustedProviders
    )
        internal view
        returns (uint256 /*_price*/, uint256 /*_assetTimestamp*/, uint256 /*_tokenTimestamp*/)
    {
        (uint256 assetPrice, uint256 assetTs, uint256 assetFtsoDec) =
            readFtsoPrice(_token.assetFtsoSymbol, _fromTrustedProviders);
        if (_token.directPricePair) {
            uint256 price = calcAmgToTokenWeiPrice(_token.decimals, 1, 0, assetPrice, assetFtsoDec);
            return (price, assetTs, assetTs);
        } else {
            (uint256 tokenPrice, uint256 tokenTs, uint256 tokenFtsoDec) =
                readFtsoPrice(_token.tokenFtsoSymbol, _fromTrustedProviders);
            uint256 price =
                calcAmgToTokenWeiPrice(_token.decimals, tokenPrice, tokenFtsoDec, assetPrice, assetFtsoDec);
            return (price, assetTs, tokenTs);
        }
    }

    function readFtsoPrice(string memory _symbol, bool _fromTrustedProviders)
        internal view
        returns (uint256, uint256, uint256)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        IPriceReader priceReader = IPriceReader(settings.priceReader);
        if (_fromTrustedProviders) {
            return priceReader.getPriceFromTrustedProviders(_symbol);
        } else {
            return priceReader.getPrice(_symbol);
        }
    }

    function calcAmgToTokenWeiPrice(
        uint256 _tokenDecimals,
        uint256 _tokenPrice,
        uint256 _tokenFtsoDecimals,
        uint256 _assetPrice,
        uint256 _assetFtsoDecimals
    )
        internal view
        returns (uint256)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        uint256 expPlus = _tokenDecimals + _tokenFtsoDecimals + AMG_TOKEN_WEI_PRICE_SCALE_EXP;
        uint256 expMinus = settings.assetMintingDecimals + _assetFtsoDecimals;
        // If negative, price would probably always be 0 after division, so this is forbidden.
        // Anyway, we should know about this before we add the token and/or asset, since
        // token decimals and ftso decimals typically never change.
        assert(expPlus >= expMinus);
        return _assetPrice.mulDiv(10 ** (expPlus - expMinus), _tokenPrice);
    }

    function convertAmgToTokenWei(uint256 _valueAMG, uint256 _amgToTokenWeiPrice) internal pure returns (uint256) {
        return _valueAMG.mulDiv(_amgToTokenWeiPrice, AMG_TOKEN_WEI_PRICE_SCALE);
    }

    function convertTokenWeiToAMG(uint256 _valueNATWei, uint256 _amgToTokenWeiPrice) internal pure returns (uint256) {
        return _valueNATWei.mulDiv(AMG_TOKEN_WEI_PRICE_SCALE, _amgToTokenWeiPrice);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IFdcVerification, IPayment, IBalanceDecreasingTransaction, IConfirmedBlockHeightExists,
        IReferencedPaymentNonexistence, IAddressValidity}
    from "@flarenetwork/flare-periphery-contracts/flare/IFdcVerification.sol";
import {AssetManagerSettings} from "../../userInterfaces/data/AssetManagerSettings.sol";
import {Globals} from "./Globals.sol";


library TransactionAttestation {

    // payment status constants
    uint8 internal constant PAYMENT_SUCCESS = 0;
    uint8 internal constant PAYMENT_FAILED = 1;
    uint8 internal constant PAYMENT_BLOCKED = 2;

    error PaymentFailed();
    error InvalidChain();
    error LegalPaymentNotProven();
    error TransactionNotProven();
    error BlockHeightNotProven();
    error NonPaymentNotProven();
    error AddressValidityNotProven();


    function verifyPaymentSuccess(
        IPayment.Proof calldata _proof
    )
        internal view
    {
        require(_proof.data.responseBody.status == PAYMENT_SUCCESS, PaymentFailed());
        verifyPayment(_proof);
    }

    function verifyPayment(
        IPayment.Proof calldata _proof
    )
        internal view
    {
        AssetManagerSettings.Data storage _settings = Globals.getSettings();
        IFdcVerification fdcVerification = IFdcVerification(_settings.fdcVerification);
        require(_proof.data.sourceId == _settings.chainId, InvalidChain());
        require(fdcVerification.verifyPayment(_proof), LegalPaymentNotProven());
    }

    function verifyBalanceDecreasingTransaction(
        IBalanceDecreasingTransaction.Proof calldata _proof
    )
        internal view
    {
        AssetManagerSettings.Data storage _settings = Globals.getSettings();
        IFdcVerification fdcVerification = IFdcVerification(_settings.fdcVerification);
        require(_proof.data.sourceId == _settings.chainId, InvalidChain());
        require(fdcVerification.verifyBalanceDecreasingTransaction(_proof), TransactionNotProven());
    }

    function verifyConfirmedBlockHeightExists(
        IConfirmedBlockHeightExists.Proof calldata _proof
    )
        internal view
    {
        AssetManagerSettings.Data storage _settings = Globals.getSettings();
        IFdcVerification fdcVerification = IFdcVerification(_settings.fdcVerification);
        require(_proof.data.sourceId == _settings.chainId, InvalidChain());
        require(fdcVerification.verifyConfirmedBlockHeightExists(_proof), BlockHeightNotProven());
    }

    function verifyReferencedPaymentNonexistence(
        IReferencedPaymentNonexistence.Proof calldata _proof
    )
        internal view
    {
        AssetManagerSettings.Data storage _settings = Globals.getSettings();
        IFdcVerification fdcVerification = IFdcVerification(_settings.fdcVerification);
        require(_proof.data.sourceId == _settings.chainId, InvalidChain());
        require(fdcVerification.verifyReferencedPaymentNonexistence(_proof), NonPaymentNotProven());
    }

    function verifyAddressValidity(
        IAddressValidity.Proof calldata _proof
    )
        internal view
    {
        AssetManagerSettings.Data storage _settings = Globals.getSettings();
        IFdcVerification fdcVerification = IFdcVerification(_settings.fdcVerification);
        require(_proof.data.sourceId == _settings.chainId, InvalidChain());
        require(fdcVerification.verifyAddressValidity(_proof), AddressValidityNotProven());
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import {ReentrancyGuard} from "../../openzeppelin/security/ReentrancyGuard.sol";
import {IIAgentVault} from "../../agentVault/interfaces/IIAgentVault.sol";
import {IAgentVault} from "../../userInterfaces/IAgentVault.sol";
import {IIAssetManager} from "../../assetManager/interfaces/IIAssetManager.sol";
import {ICollateralPool} from "../../userInterfaces/ICollateralPool.sol";


contract AgentVault is ReentrancyGuard, UUPSUpgradeable, IIAgentVault, IERC165 {
    using SafeERC20 for IERC20;

    IIAssetManager public assetManager; // practically immutable

    bool private initialized;

    IERC20[] private __usedTokens; // only storage placeholder
    mapping(IERC20 => uint256) private __tokenUseFlags; // only storage placeholder
    bool private __internalWithdrawal; // only storage placeholder

    bool private destroyed;

    modifier onlyOwner {
        require(isOwner(msg.sender), OnlyOwner());
        _;
    }

    modifier onlyAssetManager {
        require(msg.sender == address(assetManager), OnlyAssetManager());
        _;
    }

    modifier onlyKnownToken(IERC20 _token) {
        _validateToken(_token);
        _;
    }

    // Only used in some tests.
    // The implementation in production will always be deployed with address(0) for _assetManager.
    constructor(IIAssetManager _assetManager) {
        initialize(_assetManager);
    }

    function initialize(IIAssetManager _assetManager) public {
        require(!initialized, AlreadyInitialized());
        initialized = true;
        assetManager = _assetManager;
        initializeReentrancyGuard();
    }

    function buyCollateralPoolTokens()
        external payable
        onlyOwner
    {
        collateralPool().enter{value: msg.value}();
    }

    function withdrawPoolFees(uint256 _amount, address _recipient)
        external
        onlyOwner
    {
        collateralPool().withdrawFeesTo(_amount, _recipient);
    }

    function redeemCollateralPoolTokens(uint256 _amount, address payable _recipient)
        external
        onlyOwner
        nonReentrant
    {
        ICollateralPool pool = collateralPool();
        assetManager.beforeCollateralWithdrawal(pool.poolToken(), _amount);
        pool.exitTo(_amount, _recipient);
    }

    // must call `token.approve(vault, amount)` before for each token in _tokens
    function depositCollateral(IERC20 _token, uint256 _amount)
        external override
        onlyOwner
        onlyKnownToken(_token)
    {
        _token.safeTransferFrom(msg.sender, address(this), _amount);
        assetManager.updateCollateral(address(this), _token);
    }

    // update collateral after `transfer(vault, some amount)` was called (alternative to depositCollateral)
    function updateCollateral(IERC20 _token)
        external override
        onlyOwner
        onlyKnownToken(_token)
    {
        assetManager.updateCollateral(address(this), _token);
    }

    function withdrawCollateral(IERC20 _token, uint256 _amount, address _recipient)
        external override
        onlyOwner
        onlyKnownToken(_token)
        nonReentrant
    {
        // check that enough was announced and reduce announcement (not relevant after destroy)
        if (!destroyed) {
            assetManager.beforeCollateralWithdrawal(_token, _amount);
        }
        // transfer tokens to recipient
        _token.safeTransfer(_recipient, _amount);
    }

    // Allow transferring a token, airdropped to the agent vault, to the owner (management address).
    // Doesn't work for collateral tokens because this would allow withdrawing the locked collateral.
    function transferExternalToken(IERC20 _token, uint256 _amount)
        external override
        onlyOwner
    {
        require(destroyed || !assetManager.isLockedVaultToken(address(this), _token), OnlyNonCollateralTokens());
        address ownerManagementAddress = assetManager.getAgentVaultOwner(address(this));
        _token.safeTransfer(ownerManagementAddress, _amount);
    }

    /**
     * Used by asset manager when destroying agent.
     * Marks agent as destroyed so that funds can be withdrawn by the agent owner.
     * Note: Can only be called by the asset manager.
     */
    function destroy()
        external override
        onlyAssetManager
        nonReentrant
    {
        destroyed = true;
    }

    // Used by asset manager for liquidation and failed redemption.
    // Is nonReentrant to prevent reentrancy in case the token has receive hooks.
    // No need for onlyKnownToken here, because asset manager will always send valid token.
    function payout(IERC20 _token, address _recipient, uint256 _amount)
        external override
        onlyAssetManager
        nonReentrant
    {
        _token.safeTransfer(_recipient, _amount);
    }

    function collateralPool()
        public view
        returns (ICollateralPool)
    {
        return ICollateralPool(assetManager.getCollateralPool(address(this)));
    }

    function isOwner(address _address)
        public view
        returns (bool)
    {
        return assetManager.isAgentVaultOwner(address(this), _address);
    }

    /**
     * Implementation of ERC-165 interface.
     */
    function supportsInterface(bytes4 _interfaceId)
        external pure override
        returns (bool)
    {
        return _interfaceId == type(IERC165).interfaceId
            || _interfaceId == type(IAgentVault).interfaceId
            || _interfaceId == type(IIAgentVault).interfaceId;
    }

    ////////////////////////////////////////////////////////////////////////////////////
    // UUPS proxy upgrade

    function implementation() external view returns (address) {
        return _getImplementation();
    }

    /**
     * Upgrade calls can only arrive through asset manager.
     * See UUPSUpgradeable._authorizeUpgrade.
     */
    function _authorizeUpgrade(address /* _newImplementation */)
        internal virtual override
        onlyAssetManager
    { // solhint-disable-line no-empty-blocks
    }

    // Check if the token is one of known collateral tokens (not necessarily still valid as collateral),
    // to prevent agent owners attacking the system with malicious tokens.
    function _validateToken(IERC20 _token) private view {
        require(assetManager.isVaultCollateralToken(_token), UnknownToken());
    }
}

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

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;
pragma abicoder v2;

import {IAgentVault} from "../../userInterfaces/IAgentVault.sol";
import {IIAssetManager} from "../../assetManager/interfaces/IIAssetManager.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";


interface IIAgentVault is IAgentVault {
    /**
     * Used by asset manager when destroying agent.
     * Marks agent as destroyed so that funds can be withdrawn by the agent owner.
     * Note: Can only be called by the asset manager.
     */
    function destroy() external;

    // Used by asset manager for liquidation and failed redemption.
    // Is nonReentrant to prevent reentrancy in case the token has receive hooks.
    // onlyAssetManager
    function payout(IERC20 _token, address _recipient, uint256 _amount) external;

    // Returns the asset manager to which this vault belongs.
    function assetManager() external view returns (IIAssetManager);

    // Enables owner checks in the asset manager.
    function isOwner(address _address) external view returns (bool);
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import {IICleanable} from "@flarenetwork/flare-periphery-contracts/flare/token/interfaces/IICleanable.sol";
import {IFAsset} from "../../userInterfaces/IFAsset.sol";
import {IICheckPointable} from "./IICheckPointable.sol";


interface IIFAsset is IFAsset, IICheckPointable, IICleanable {
    /**
     * Mints `_amount` od fAsset.
     * Only the assetManager corresponding to this fAsset may call `mint()`.
     */
    function mint(address _owner, uint256 _amount) external;

    /**
     * Burns `_amount` od fAsset.
     * Only the assetManager corresponding to this fAsset may call `burn()`.
     */
    function burn(address _owner, uint256 _amount) external;

    /**
     * Set the contract that is allowed to set cleanupBlockNumber.
     * Usually this will be an instance of CleanupBlockNumberManager.
     */
    function setCleanupBlockNumberManager(address _cleanupBlockNumberManager) external;

    /**
     * The contract that is allowed to set cleanupBlockNumber.
     * Usually this will be an instance of CleanupBlockNumberManager.
     */
    function cleanupBlockNumberManager() external view returns (address);
}

// SPDX-License-Identifier: MIT

// OpenZeppelin Contracts (last updated v4.9.0) (security/ReentrancyGuard.sol)
// Modified by FlareLabs to use diamond storage

pragma solidity ^0.8.27;

import {Reentrancy} from "../library/Reentrancy.sol";

/**
 * @dev Contract module that helps prevent reentrant calls to a function.
 *
 * Inheriting from `ReentrancyGuard` will make the {nonReentrant} modifier
 * available, which can be applied to functions to make sure there are no nested
 * (reentrant) calls to them.
 *
 * Note that because there is a single `nonReentrant` guard, functions marked as
 * `nonReentrant` may not call one another. This can be worked around by making
 * those functions `private`, and then adding `external` `nonReentrant` entry
 * points to them.
 *
 * TIP: If you would like to learn more about reentrancy and alternative ways
 * to protect against it, check out our blog post
 * https://blog.openzeppelin.com/reentrancy-after-istanbul/[Reentrancy After Istanbul].
 */
abstract contract ReentrancyGuard {
    /**
     * @dev Prevents a contract from calling itself, directly or indirectly.
     * Calling a `nonReentrant` function from another `nonReentrant`
     * function is not supported. It is possible to prevent this from happening
     * by making the `nonReentrant` function external, and making it call a
     * `private` function that does the actual work.
     */
    modifier nonReentrant() {
        Reentrancy.nonReentrantBefore();
        _;
        Reentrancy.nonReentrantAfter();
    }

    /**
     * Should be called once at construction time of the main contract (not a constructor, to allow proxies/diamond).
     * Not a big issue if it is never called - just the first nonReentrant method call will use more gas.
     */
    function initializeReentrancyGuard() internal {
        Reentrancy.initializeReentrancyGuard();
    }

    /**
     * Marks a piece of code that can only be executed within a `nonReentrant` method.
     * Useful to prevent e.g. NAT transfers that don't properly guard against reentrancy
     * and to make them fail at test time.
     */
    function requireReentrancyGuard() internal view {
        Reentrancy.requireReentrancyGuard();
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Agents} from "../library/Agents.sol";
import {Globals} from "../library/Globals.sol";
import {AssetManagerState} from "../library/data/AssetManagerState.sol";
import {AssetManagerSettings} from "../../userInterfaces/data/AssetManagerSettings.sol";


abstract contract AssetManagerBase {
    error OnlyAssetManagerController();
    error NotAttached();
    error NotWhitelisted();
    error EmergencyPauseActive();

    modifier onlyAssetManagerController {
        _checkOnlyAssetManagerController();
        _;
    }

    modifier onlyAttached {
        _checkOnlyAttached();
        _;
    }

    modifier notEmergencyPaused {
        _checkEmergencyPauseNotActive();
        _;
    }

    modifier onlyAgentVaultOwner(address _agentVault) {
        Agents.requireAgentVaultOwner(_agentVault);
        _;
    }

    function _checkOnlyAssetManagerController() private view {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        require(msg.sender == settings.assetManagerController, OnlyAssetManagerController());
    }

    function _checkOnlyAttached() private view {
        require(AssetManagerState.get().attached, NotAttached());
    }

    function _checkEmergencyPauseNotActive() private view {
        AssetManagerState.State storage state = AssetManagerState.get();
        require(state.emergencyPausedUntil <= block.timestamp, EmergencyPauseActive());
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IICheckPointable} from "../interfaces/IICheckPointable.sol";
import {CheckPointHistory} from "../library/CheckPointHistory.sol";
import {CheckPointsByAddress} from "../library/CheckPointsByAddress.sol";

/**
 * @title Check Pointable ERC20 Behavior
 * @notice ERC20 behavior which adds balance check point features.
 **/
abstract contract CheckPointable is IICheckPointable {
    error CheckPointableReadingFromCleanedupBlock();
    error OnlyCleanerContract();
    error CleanupBlockNumberMustNeverDecrease();
    error CleanupBlockMustBeInThePast();

    using CheckPointHistory for CheckPointHistory.CheckPointHistoryState;
    using CheckPointsByAddress for CheckPointsByAddress.CheckPointsByAddressState;

    // The number of history cleanup steps executed for every write operation.
    // It is more than 1 to make as certain as possible that all history gets cleaned eventually.
    uint256 private constant CLEANUP_COUNT = 2;

    // Private member variables
    CheckPointsByAddress.CheckPointsByAddressState private balanceHistory;
    CheckPointHistory.CheckPointHistoryState private totalSupply;

    // Historic data for the blocks before `cleanupBlockNumber` can be erased,
    // history before that block should never be used since it can be inconsistent.
    uint256 private cleanupBlockNumber;

    // Address of the contract that is allowed to call methods for history cleaning.
    address public cleanerContract;

    /**
     * Emitted when a total supply cache entry is created.
     * Allows history cleaners to track total supply cache cleanup opportunities off-chain.
     */
    event CreatedTotalSupplyCache(uint256 _blockNumber);

    // Most cleanup opportunities can be deduced from standard event
    // Transfer(from, to, amount):
    //   - balance history for `from` (if nonzero) and `to` (if nonzero)
    //   - total supply history when either `from` or `to` is zero

    modifier notBeforeCleanupBlock(uint256 _blockNumber) {
        require(_blockNumber >= cleanupBlockNumber, CheckPointableReadingFromCleanedupBlock());
        _;
    }

    modifier onlyCleaner {
        require(msg.sender == cleanerContract, OnlyCleanerContract());
        _;
    }

    /**
     * @dev Queries the token balance of `_owner` at a specific `_blockNumber`.
     * @param _owner The address from which the balance will be retrieved.
     * @param _blockNumber The block number when the balance is queried.
     * @return _balance The balance at `_blockNumber`.
     **/
    function balanceOfAt(address _owner, uint256 _blockNumber)
        public virtual view
        notBeforeCleanupBlock(_blockNumber)
        returns (uint256 _balance)
    {
        return balanceHistory.valueOfAt(_owner, _blockNumber);
    }

    /**
     * @notice Burn current token `amount` for `owner` of checkpoints at current block.
     * @param _owner The address of the owner to burn tokens.
     * @param _amount The amount to burn.
     */
    function _burnForAtNow(address _owner, uint256 _amount) internal virtual {
        uint256 newBalance = balanceOfAt(_owner, block.number) - _amount;
        balanceHistory.writeValue(_owner, newBalance);
        balanceHistory.cleanupOldCheckpoints(_owner, CLEANUP_COUNT, cleanupBlockNumber);
        totalSupply.writeValue(totalSupplyAt(block.number) - _amount);
        totalSupply.cleanupOldCheckpoints(CLEANUP_COUNT, cleanupBlockNumber);
    }

    /**
     * @notice Mint current token `amount` for `owner` of checkpoints at current block.
     * @param _owner The address of the owner to burn tokens.
     * @param _amount The amount to burn.
     */
    function _mintForAtNow(address _owner, uint256 _amount) internal virtual {
        uint256 newBalance = balanceOfAt(_owner, block.number) + _amount;
        balanceHistory.writeValue(_owner, newBalance);
        balanceHistory.cleanupOldCheckpoints(_owner, CLEANUP_COUNT, cleanupBlockNumber);
        totalSupply.writeValue(totalSupplyAt(block.number) + _amount);
        totalSupply.cleanupOldCheckpoints(CLEANUP_COUNT, cleanupBlockNumber);
    }

    /**
     * @notice Total amount of tokens at a specific `_blockNumber`.
     * @param _blockNumber The block number when the _totalSupply is queried
     * @return _totalSupply The total amount of tokens at `_blockNumber`
     **/
    function totalSupplyAt(uint256 _blockNumber)
        public virtual view
        notBeforeCleanupBlock(_blockNumber)
        returns(uint256 _totalSupply)
    {
        return totalSupply.valueAt(_blockNumber);
    }

    /**
     * @notice Transmit token `_amount` `_from` address `_to` address of checkpoints at current block.
     * @param _from The address of the sender.
     * @param _to The address of the receiver.
     * @param _amount The amount to transmit.
     */
    function _transmitAtNow(address _from, address _to, uint256 _amount) internal virtual {
        balanceHistory.transmit(_from, _to, _amount);
        balanceHistory.cleanupOldCheckpoints(_from, CLEANUP_COUNT, cleanupBlockNumber);
        balanceHistory.cleanupOldCheckpoints(_to, CLEANUP_COUNT, cleanupBlockNumber);
    }

    /**
     * Set the cleanup block number.
     */
    function _setCleanupBlockNumber(uint256 _blockNumber) internal {
        require(_blockNumber >= cleanupBlockNumber, CleanupBlockNumberMustNeverDecrease());
        require(_blockNumber < block.number, CleanupBlockMustBeInThePast());
        cleanupBlockNumber = _blockNumber;
    }

    /**
     * Get the cleanup block number.
     */
    function _cleanupBlockNumber() internal view returns (uint256) {
        return cleanupBlockNumber;
    }

    /**
     * @notice Update history at token transfer, the CheckPointable part of `_beforeTokenTransfer` hook.
     * @param _from The address of the sender.
     * @param _to The address of the receiver.
     * @param _amount The amount to transmit.
     */
    function _updateBalanceHistoryAtTransfer(address _from, address _to, uint256 _amount) internal virtual {
        if (_from == address(0)) {
            // mint checkpoint balance data for transferee
            _mintForAtNow(_to, _amount);
        } else if (_to == address(0)) {
            // burn checkpoint data for transferer
            _burnForAtNow(_from, _amount);
        } else {
            // transfer checkpoint balance data
            _transmitAtNow(_from, _to, _amount);
        }
    }

    // history cleanup methods

    /**
     * Set the contract that is allowed to call history cleaning methods.
     */
    function _setCleanerContract(address _cleanerContract) internal {
        cleanerContract = _cleanerContract;
    }

    /**
     * Delete balance checkpoints that expired (i.e. are before `cleanupBlockNumber`).
     * Method can only be called from the `cleanerContract` (which may be a proxy to external cleaners).
     * @param _owner balance owner account address
     * @param _count maximum number of checkpoints to delete
     * @return the number of checkpoints deleted
     */
    function balanceHistoryCleanup(address _owner, uint256 _count) external onlyCleaner returns (uint256) {
        return balanceHistory.cleanupOldCheckpoints(_owner, _count, cleanupBlockNumber);
    }

    /**
     * Delete total supply checkpoints that expired (i.e. are before `cleanupBlockNumber`).
     * Method can only be called from the `cleanerContract` (which may be a proxy to external cleaners).
     * @param _count maximum number of checkpoints to delete
     * @return the number of checkpoints deleted
     */
    function totalSupplyHistoryCleanup(uint256 _count) external onlyCleaner returns (uint256) {
        return totalSupply.cleanupOldCheckpoints(_count, cleanupBlockNumber);
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {EnumerableSet} from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import {IICollateralPool} from "../../../collateralPool/interfaces/IICollateralPool.sol";


library Agent {
    error InvalidAgentVaultAddress();

    enum Status {
        EMPTY,              // agent does not exist
        NORMAL,
        LIQUIDATION,        // liquidation due to CR - ends when agent is healthy
        FULL_LIQUIDATION,   // illegal payment liquidation - must liquidate all and close vault
        DESTROYING,         // agent announced destroy, cannot mint again
        DESTROYED           // agent has been destroyed, cannot do anything except return info
    }

    // For agents to withdraw NAT collateral, they must first announce it and then wait
    // withdrawalAnnouncementSeconds.
    // The announced amount cannot be used as collateral for minting during that time.
    // This makes sure that agents cannot just remove all collateral if they are challenged.
    struct WithdrawalAnnouncement {
        // Announce amount in collateral token's minimum unit (wei).
        uint128 amountWei;

        // The timestamp when withdrawal can be executed.
        uint64 allowedAt;
    }

    // Struct to store agent's pending setting updates.
    struct SettingUpdate {
        uint128 value;
        uint64 validAt;
    }

    struct State {
        IICollateralPool collateralPool;

        // Address of the agent owner. This is the management address, which is immutable.
        // The work address can be retrieved from the global state mapping between
        // management and work addresses.
        address ownerManagementAddress;

        // Current underlying address for this agent vault.
        // The address is immutable.
        string underlyingAddressString;

        // `underlyingAddressString` is only used for sending the minter a correct payment address;
        // for matching payment addresses we always use `underlyingAddressHash = keccak256(underlyingAddressString)`
        bytes32 underlyingAddressHash;

        // Current status of the agent (changes for liquidation).
        Agent.Status status;

        // Index of collateral vault token.
        // The data is obtained as state.collateralTokens[vaultCollateralIndex].
        uint16 vaultCollateralIndex;

        // Index of token in collateral pool. This is always wrapped FLR/SGB, however the wrapping
        // contract (WNat) may change. In such case we add new collateral token with class POOL but the
        // agent must call a method to upgrade to new contract, se we must track the actual token used.
        uint16 poolCollateralIndex;

        // Position of this agent in the list of agents available for minting.
        // Value is actually `list index + 1`, so that 0 means 'not in the list'.
        uint32 availableAgentsPos;

        // Minting fee in BIPS (collected in underlying currency).
        uint16 feeBIPS;

        // Share of the minting fee that goes to the pool as percentage of the minting fee.
        uint16 poolFeeShareBIPS;

        // Collateral ratio at which we calculate locked collateral and collateral available for minting.
        // Agent may set own value for minting collateral ratio when entering the available agent list,
        // but it must always be greater than minimum collateral ratio.
        uint32 mintingVaultCollateralRatioBIPS;

        // Collateral ratio at which we calculate locked collateral and collateral available for minting.
        // Agent may set own value for minting collateral ratio when entering the available agent list,
        // but it must always be greater than minimum collateral ratio.
        uint32 mintingPoolCollateralRatioBIPS;

        // Timestamp of the startLiquidation (or liquidate) call.
        uint64 liquidationStartedAt;

        // Liquidation phase at the time when liquidation started.
        uint8 __initialLiquidationPhase; // only storage placeholder

        // Bitmap signifying which collateral type(s) triggered liquidation (LF_VAULT | LF_POOL).
        uint8 collateralsUnderwater;

        // Amount of collateral locked by collateral reservation.
        uint64 reservedAMG;

        // Amount of collateral backing minted fassets.
        uint64 mintedAMG;

        // The amount of fassets being redeemed. In this case, the fassets were already burned,
        // but the collateral must still be locked to allow payment in case of redemption failure.
        // The distinction between 'minted' and 'redeemed' assets is important in case of challenge.
        uint64 redeemingAMG;

        // The amount of fassets being redeemed EXCEPT those from pool self-close exits.
        // Unlike normal redemption, pool collateral was already withdrawn, so the redeeming collateral
        // must only be accounted for / locked for vault collateral.
        // On redemption payment failure, redeemer will be paid only in vault collateral in this case
        // (and will be paid less if there isn't enough - small extra risk for pool token holders).
        // There will always be `poolRedeemingAMG <= redeemingAMG`.
        uint64 poolRedeemingAMG;

        // When lot size changes, there may be some leftover after redemption that doesn't fit
        // a whole lot size. It is added to dustAMG and can be recovered via self-close.
        // Unlike redeemingAMG, dustAMG is still counted in the mintedAMG.
        uint64 dustAMG;

        // The amount of funds that on the agent's underlying address.
        // If it is higher than the amount needed to back mintings, it can be withdrawn after announcement.
        // It is signed int, because unreported deposits combined with other operations can in principle
        // make it negative. We could truncate it at 0, but if deposit report comes later, this would make
        // the value wrong.
        int128 underlyingBalanceUBA;

        // There can be only one announced underlying withdrawal per agent active at any time.
        // This variable holds the id, or 0 if there is no announced underlying withdrawal going on.
        uint64 announcedUnderlyingWithdrawalId;

        // The time when ongoing underlying withdrawal was announced.
        uint64 underlyingWithdrawalAnnouncedAt;

        // Announcement for vault collateral withdrawal.
        WithdrawalAnnouncement vaultCollateralWithdrawalAnnouncement;

        // Announcement for pool token withdrawal (which also means pool collateral withdrawal).
        WithdrawalAnnouncement poolTokenWithdrawalAnnouncement;

        // Underlying block when the agent was created.
        // Challenger's should track underlying address activity since this block
        // and topups are only valid after this block (both inclusive).
        uint64 underlyingBlockAtCreation;

        // The time when ongoing agent vault destroy was announced.
        uint64 destroyAllowedAt;

        // The factor set by the agent to multiply the price at which agent buys f-assets from pool
        // token holders on self-close exit (when requested or the redeemed amount is less than 1 lot).
        uint16 buyFAssetByAgentFactorBIPS;

        // The announced time when the agent is exiting available agents list.
        uint64 exitAvailableAfterTs;

        // The position of the agent in the list of all agents.
        uint32 allAgentsPos;

        // Agent's pending setting updates.
        mapping(bytes32 => SettingUpdate) settingUpdates;

        // Agent's handshake type - minting or redeeming can be rejected.
        // 0 - no verification, 1 - manual verification, ...
        uint32 __handshakeType; // only storage placeholder

        // There can only be one transfer to core vault per agent active at any time.
        uint64 activeTransferToCoreVault;

        // the request id of the active return from core vault
        uint64 activeReturnFromCoreVaultId;

        // part of the agent's reservedAMG for the core vault return
        uint64 returnFromCoreVaultReservedAMG;

        // The redemption fee share paid to the pool (as FAssets).
        // In redemption dominated situations (when agent requests return from core vault to earn
        // from redemption fees), pool can get some share to make it sustainable for pool users.
        // NOTE: the pool fee share is locked at the redemption request time, but is charged at the redemption
        // confirmation time. If agent uses all the redemption fee for transaction fees, this could make the
        // agent's free underlying balance negative.
        uint16 redemptionPoolFeeShareBIPS;

        EnumerableSet.AddressSet alwaysAllowedMinters;

        // Only used for calculating Agent.State size. See deleteStorage() below.
        uint256[1] _endMarker;
    }

    // underwater collateral classes
    uint8 internal constant LF_VAULT = 1 << 0;
    uint8 internal constant LF_POOL = 1 << 1;

    // diamond state accessors

    bytes32 internal constant AGENTS_POSITION = keccak256("fasset.AssetManager.Agent");

    // only return valid agent - fail if status is EMPTY or DESTROYED
    function get(address _address)
        internal view
        returns (Agent.State storage)
    {
        Agent.State storage agent = getWithoutCheck(_address);
        Agent.Status status = agent.status;
        require(status != Agent.Status.EMPTY && status != Agent.Status.DESTROYED, InvalidAgentVaultAddress());
        return agent;
    }

    // Like get, but only fail if status is EMPTY.
    // This is useful for reading agent info after the agent has been destroyed.
    function getAllowDestroyed(address _address)
        internal view
        returns (Agent.State storage)
    {
        Agent.State storage agent = getWithoutCheck(_address);
        require(agent.status != Agent.Status.EMPTY, InvalidAgentVaultAddress());
        return agent;
    }

    function getWithoutCheck(address _address)
        internal pure
        returns (Agent.State storage _agent)
    {
        bytes32 position = bytes32(uint256(AGENTS_POSITION) ^ (uint256(uint160(_address)) << 64));
        // solhint-disable-next-line no-inline-assembly
        assembly {
            _agent.slot := position
        }
    }

    function vaultAddress(Agent.State storage _agent)
        internal pure
        returns (address)
    {
        bytes32 position;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            position := _agent.slot
        }
        return address(uint160((uint256(position) ^ uint256(AGENTS_POSITION)) >> 64));
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {SafeMath64} from "../../utils/library/SafeMath64.sol";
import {Transfers} from "../../utils/library/Transfers.sol";
import {AssetManagerState} from "./data/AssetManagerState.sol";
import {Conversion} from "./Conversion.sol";
import {AgentBacking} from "./AgentBacking.sol";
import {Agent} from "../../assetManager/library/data/Agent.sol";
import {RedemptionQueue} from "./data/RedemptionQueue.sol";
import {Redemption} from "./data/Redemption.sol";
import {Globals} from "./Globals.sol";
import {IAssetManagerEvents} from "../../userInterfaces/IAssetManagerEvents.sol";


library Redemptions {
    using Agent for Agent.State;
    using RedemptionQueue for RedemptionQueue.State;

    error InvalidRequestId();

    function closeTickets(
        Agent.State storage _agent,
        uint64 _amountAMG,
        bool _immediatelyReleaseMinted
    )
        internal
        returns (uint64 _closedAMG, uint256 _closedUBA)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        // redemption tickets
        uint256 maxRedeemedTickets = Globals.getSettings().maxRedeemedTickets;
        uint64 lotSize = Globals.getSettings().lotSizeAMG;
        for (uint256 i = 0; i < maxRedeemedTickets && _closedAMG < _amountAMG; i++) {
            // each loop, firstTicketId will change since we delete the first ticket
            uint64 ticketId = state.redemptionQueue.agents[_agent.vaultAddress()].firstTicketId;
            if (ticketId == 0) {
                break;  // no more tickets for this agent
            }
            RedemptionQueue.Ticket storage ticket = state.redemptionQueue.getTicket(ticketId);
            uint64 maxTicketRedeemAMG = ticket.valueAMG + _agent.dustAMG;
            maxTicketRedeemAMG -= maxTicketRedeemAMG % lotSize; // round down to whole lots
            uint64 ticketRedeemAMG = SafeMath64.min64(_amountAMG - _closedAMG, maxTicketRedeemAMG);
            // only remove from tickets and add to total, do everything else after the loop
            removeFromTicket(ticketId, ticketRedeemAMG);
            _closedAMG += ticketRedeemAMG;
        }
        // now close the dust if anything remains (e.g. if there were not enough tickets to redeem)
        uint64 closeDustAMG = SafeMath64.min64(_amountAMG - _closedAMG, _agent.dustAMG);
        if (closeDustAMG > 0) {
            _closedAMG += closeDustAMG;
            AgentBacking.decreaseDust(_agent, closeDustAMG);
        }
        // self-close or liquidation is one step, so we can release minted assets without redeeming step
        if (_immediatelyReleaseMinted) {
            AgentBacking.releaseMintedAssets(_agent, _closedAMG);
        }
        // return
        _closedUBA = Conversion.convertAmgToUBA(_closedAMG);
    }

    function removeFromTicket(
        uint64 _redemptionTicketId,
        uint64 _redeemedAMG
    )
        internal
    {
        RedemptionQueue.State storage redemptionQueue = AssetManagerState.get().redemptionQueue;
        RedemptionQueue.Ticket storage ticket = redemptionQueue.getTicket(_redemptionTicketId);
        Agent.State storage agent = Agent.get(ticket.agentVault);
        uint64 lotSize = Globals.getSettings().lotSizeAMG;
        uint64 remainingAMG = ticket.valueAMG + agent.dustAMG - _redeemedAMG;
        uint64 remainingAMGDust = remainingAMG % lotSize;
        uint64 remainingAMGLots = remainingAMG - remainingAMGDust;
        if (remainingAMGLots == 0) {
            redemptionQueue.deleteRedemptionTicket(_redemptionTicketId);
            emit IAssetManagerEvents.RedemptionTicketDeleted(agent.vaultAddress(), _redemptionTicketId);
        } else if (remainingAMGLots != ticket.valueAMG) {
            ticket.valueAMG = remainingAMGLots;
            uint256 remainingUBA = Conversion.convertAmgToUBA(remainingAMGLots);
            emit IAssetManagerEvents.RedemptionTicketUpdated(agent.vaultAddress(), _redemptionTicketId, remainingUBA);
        }
        AgentBacking.changeDust(agent, remainingAMGDust);
    }

    function burnFAssets(
        address _owner,
        uint256 _amountUBA
    )
        internal
    {
        Globals.getFAsset().burn(_owner, _amountUBA);
    }

    // pay executor for executor calls in WNat, otherwise burn executor fee
    function payOrBurnExecutorFee(
        Redemption.Request storage _request
    )
        internal
    {
        uint256 executorFeeNatWei = _request.executorFeeNatGWei * Conversion.GWEI;
        if (executorFeeNatWei > 0) {
            _request.executorFeeNatGWei = 0;
            if (msg.sender == _request.executor) {
                Transfers.depositWNat(Globals.getWNat(), _request.executor, executorFeeNatWei);
            } else {
                Globals.getBurnAddress().transfer(executorFeeNatWei);
            }
        }
    }

    // burn executor fee
    function burnExecutorFee(
        Redemption.Request storage _request
    )
        internal
    {
        uint256 executorFeeNatWei = _request.executorFeeNatGWei * Conversion.GWEI;
        if (executorFeeNatWei > 0) {
            _request.executorFeeNatGWei = 0;
            Globals.getBurnAddress().transfer(executorFeeNatWei);
        }
    }

    function reCreateRedemptionTicket(
        Agent.State storage _agent,
        Redemption.Request storage _request
    )
        internal
    {
        AgentBacking.endRedeemingAssets(_agent, _request.valueAMG, _request.poolSelfClose);
        AgentBacking.createNewMinting(_agent, _request.valueAMG);
    }

    function finishRedemptionRequest(
        uint256 _redemptionRequestId,
        Redemption.Request storage _request,
        Redemption.Status _status
    )
        internal
    {
        assert(_status >= Redemption.Status.SUCCESSFUL);    // must be a final status
        _request.status = _status;
        releaseTransferToCoreVault(_redemptionRequestId, _request);
    }

    function releaseTransferToCoreVault(
        uint256 _redemptionRequestId,
        Redemption.Request storage _request
    )
        internal
    {
        if (_request.transferToCoreVault) {
            Agent.State storage agent = Agent.get(_request.agentVault);
            if (agent.activeTransferToCoreVault == _redemptionRequestId) {
                agent.activeTransferToCoreVault = 0;
            }
        }
    }

    function getRedemptionRequest(
        uint256 _redemptionRequestId,
        bool _requireUnconfirmed
    )
        internal view
        returns (Redemption.Request storage _request)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        require(_redemptionRequestId != 0, InvalidRequestId());
        _request = state.redemptionRequests[_redemptionRequestId];
        if (_requireUnconfirmed) {
            require(isOpen(_request), InvalidRequestId());
        } else {
            require(_request.status != Redemption.Status.EMPTY, InvalidRequestId());
        }
    }

    // true if redemption is valid and has not been confirmed yet
    function isOpen(Redemption.Request storage _request)
        internal view
        returns (bool)
    {
        Redemption.Status status = _request.status;
        return status == Redemption.Status.ACTIVE || status == Redemption.Status.DEFAULTED;
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {SafeMath64} from "../../../utils/library/SafeMath64.sol";


library RedemptionTimeExtension {
    using SafeCast for uint256;
    using SafeMath64 for uint64;

    struct AgentTimeExtensionData {
        uint64 extendedTimestamp;
    }

    struct State {
        // settings
        uint64 redemptionPaymentExtensionSeconds;

        // per agent state
        mapping(address _agentVault => AgentTimeExtensionData) agents;
    }

    /**
     * Calculates the redemption time extension when there are multiple redemption requests in short time.
     * Implements "leaky bucket" algorithm, popular in rate-limiters.
     * @param _agentVault the agent vault address being redeemed
     */
    function extendTimeForRedemption(address _agentVault)
        internal
        returns (uint64)
    {
        State storage state = getState();
        AgentTimeExtensionData storage agentData = state.agents[_agentVault];
        uint64 timestamp = block.timestamp.toUint64();
        uint64 accumulatedTimestamp = agentData.extendedTimestamp + state.redemptionPaymentExtensionSeconds;
        agentData.extendedTimestamp = SafeMath64.max64(accumulatedTimestamp, timestamp);
        return agentData.extendedTimestamp - timestamp;
    }

    function setRedemptionPaymentExtensionSeconds(uint256 _value)
        internal
    {
        State storage state = getState();
        state.redemptionPaymentExtensionSeconds = _value.toUint64();
    }

    function redemptionPaymentExtensionSeconds()
        internal view
        returns (uint256)
    {
        State storage state = getState();
        return state.redemptionPaymentExtensionSeconds;
    }

    bytes32 internal constant STATE_POSITION = keccak256("fasset.RedemptionTimeExtension.State");

    function getState()
        internal pure
        returns (State storage _state)
    {
        bytes32 position = STATE_POSITION;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            _state.slot := position
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {SafePct} from "../../utils/library/SafePct.sol";
import {Math} from "@openzeppelin/contracts/utils/math/Math.sol";
import {AssetManagerState} from "./data/AssetManagerState.sol";
import {Collateral} from "./data/Collateral.sol";
import {Globals} from "./Globals.sol";
import {Conversion} from "./Conversion.sol";
import {Agent} from "./data/Agent.sol";
import {RedemptionQueue} from "./data/RedemptionQueue.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {CollateralTypeInt} from "./data/CollateralTypeInt.sol";
import {IWNat} from "../../flareSmartContracts/interfaces/IWNat.sol";
import {AgentInfo} from "../../userInterfaces/data/AgentInfo.sol";

library Agents {
    using SafeCast for uint256;
    using SafePct for uint256;
    using Agent for Agent.State;
    using RedemptionQueue for RedemptionQueue.State;

    error AgentNotWhitelisted();
    error OnlyAgentVaultOwner();
    error OnlyCollateralPool();


    function getAllAgents(
        uint256 _start,
        uint256 _end
    )
        internal view
        returns (address[] memory _agents, uint256 _totalLength)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        _totalLength = state.allAgents.length;
        _end = Math.min(_end, _totalLength);
        _start = Math.min(_start, _end);
        _agents = new address[](_end - _start);
        for (uint256 i = _start; i < _end; i++) {
            _agents[i - _start] = state.allAgents[i];
        }
    }

    function getAgentStatus(
        Agent.State storage _agent
    )
        internal view
        returns (AgentInfo.Status)
    {
        Agent.Status status = _agent.status;
        if (status == Agent.Status.NORMAL) {
            return AgentInfo.Status.NORMAL;
        } else if (status == Agent.Status.LIQUIDATION) {
            return AgentInfo.Status.LIQUIDATION;
        } else if (status == Agent.Status.FULL_LIQUIDATION) {
            return AgentInfo.Status.FULL_LIQUIDATION;
        } else if (status == Agent.Status.DESTROYING) {
            return AgentInfo.Status.DESTROYING;
        } else {
            assert (status == Agent.Status.DESTROYED);
            return AgentInfo.Status.DESTROYED;
        }
    }

    function isOwner(
        Agent.State storage _agent,
        address _address
    )
        internal view
        returns (bool)
    {
        return _address == _agent.ownerManagementAddress || _address == getWorkAddress(_agent);
    }

    function getWorkAddress(Agent.State storage _agent)
        internal view
        returns (address)
    {
        return Globals.getAgentOwnerRegistry().getWorkAddress(_agent.ownerManagementAddress);
    }

    function getOwnerPayAddress(Agent.State storage _agent)
        internal view
        returns (address payable)
    {
        address workAddress = getWorkAddress(_agent);
        return workAddress != address(0) ? payable(workAddress) : payable(_agent.ownerManagementAddress);
    }

    function requireWhitelisted(
        address _ownerManagementAddress
    )
        internal view
    {
        require(Globals.getAgentOwnerRegistry().isWhitelisted(_ownerManagementAddress),
            AgentNotWhitelisted());
    }

    function requireWhitelistedAgentVaultOwner(
        Agent.State storage _agent
    )
        internal view
    {
        requireWhitelisted(_agent.ownerManagementAddress);
    }

    function requireAgentVaultOwner(
        address _agentVault
    )
        internal view
    {
        require(isOwner(Agent.get(_agentVault), msg.sender), OnlyAgentVaultOwner());
    }

    function requireAgentVaultOwner(
        Agent.State storage _agent
    )
        internal view
    {
        require(isOwner(_agent, msg.sender), OnlyAgentVaultOwner());
    }

    function requireCollateralPool(
        Agent.State storage _agent
    )
        internal view
    {
        require(msg.sender == address(_agent.collateralPool), OnlyCollateralPool());
    }

    function isCollateralToken(
        Agent.State storage _agent,
        IERC20 _token
    )
        internal view
        returns (bool)
    {
        return _token == getPoolWNat(_agent) || _token == getVaultCollateralToken(_agent);
    }

    function getVaultCollateralToken(Agent.State storage _agent)
        internal view
        returns (IERC20)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        return state.collateralTokens[_agent.vaultCollateralIndex].token;
    }

    function getVaultCollateral(Agent.State storage _agent)
        internal view
        returns (CollateralTypeInt.Data storage)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        return state.collateralTokens[_agent.vaultCollateralIndex];
    }

    function convertUSD5ToVaultCollateralWei(Agent.State storage _agent, uint256 _amountUSD5)
        internal view
        returns (uint256)
    {
        return Conversion.convertFromUSD5(_amountUSD5, getVaultCollateral(_agent));
    }

    function getPoolWNat(Agent.State storage _agent)
        internal view
        returns (IWNat)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        return IWNat(address(state.collateralTokens[_agent.poolCollateralIndex].token));
    }

    function getPoolCollateral(Agent.State storage _agent)
        internal view
        returns (CollateralTypeInt.Data storage)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        return state.collateralTokens[_agent.poolCollateralIndex];
    }

    function getCollateral(Agent.State storage _agent, Collateral.Kind _kind)
        internal view
        returns (CollateralTypeInt.Data storage)
    {
        assert (_kind != Collateral.Kind.AGENT_POOL);   // there is no agent pool collateral token
        AssetManagerState.State storage state = AssetManagerState.get();
        if (_kind == Collateral.Kind.VAULT) {
            return state.collateralTokens[_agent.vaultCollateralIndex];
        } else {
            return state.collateralTokens[_agent.poolCollateralIndex];
        }
    }

    function collateralUnderwater(Agent.State storage _agent, Collateral.Kind _kind)
        internal view
        returns (bool)
    {
        if (_kind == Collateral.Kind.VAULT) {
            return (_agent.collateralsUnderwater & Agent.LF_VAULT) != 0;
        } else {
            // AGENT_POOL collateral cannot be underwater (it only affects minting),
            // so this function will only be used for VAULT and POOL
            assert(_kind == Collateral.Kind.POOL);
            return (_agent.collateralsUnderwater & Agent.LF_POOL) != 0;
        }
    }

    function withdrawalAnnouncement(Agent.State storage _agent, Collateral.Kind _kind)
        internal view
        returns (Agent.WithdrawalAnnouncement storage)
    {
        assert (_kind != Collateral.Kind.POOL);     // agent cannot withdraw from pool
        return _kind == Collateral.Kind.VAULT
            ? _agent.vaultCollateralWithdrawalAnnouncement
            : _agent.poolTokenWithdrawalAnnouncement;
    }

    function totalBackedAMG(Agent.State storage _agent)
        internal view
        returns (uint64)
    {
        // this must always hold, so assert it is true, otherwise the following line
        // would need `max(redeemingAMG, poolRedeemingAMG)`
        assert(_agent.poolRedeemingAMG <= _agent.redeemingAMG);
        return _agent.mintedAMG + _agent.reservedAMG + _agent.redeemingAMG;
    }
}
// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;
pragma abicoder v2;

import {ICollateralPool} from "../../userInterfaces/ICollateralPool.sol";
import {IIAssetManager} from "../../assetManager/interfaces/IIAssetManager.sol";
import {IWNat} from "../../flareSmartContracts/interfaces/IWNat.sol";

/**
 * Collateral pool methods that are only callable by the asset manager or pool token.
 */
interface IICollateralPool is ICollateralPool {
    function setPoolToken(address _poolToken) external;

    function depositNat() external payable;

    function payout(
        address _receiver,
        uint256 _amountWei,
        uint256 _agentResponsibilityWei
    ) external;

    function destroy(address payable _recipient) external;

    function upgradeWNatContract(IWNat newWNat) external;

    function setExitCollateralRatioBIPS(uint256 _value) external;

    function fAssetFeeDeposited(uint256 _amount) external;

    function wNat() external view returns (IWNat);

    function debtFreeTokensOf(address _account) external view returns (uint256);

    function debtLockedTokensOf(
        address _account
    ) external view returns (uint256);

    function assetManager() external view returns (IIAssetManager);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {SafePct} from "../../utils/library/SafePct.sol";
import {AssetManagerState} from "./data/AssetManagerState.sol";
import {IAssetManagerEvents} from "../../userInterfaces/IAssetManagerEvents.sol";
import {CollateralType} from "../../userInterfaces/data/CollateralType.sol";
import {CollateralTypeInt} from "./data/CollateralTypeInt.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {Conversion} from "./Conversion.sol";


library CollateralTypes {
    using SafeCast for uint256;

    error InvalidCollateralRatios();
    error CannotAddDeprecatedToken();
    error TokenAlreadyExists();
    error TokenZero();
    error PriceNotInitialized();
    error UnknownToken();
    error NotAVaultCollateral();
    error NotAPoolCollateralAtZero();
    error AtLeastTwoCollateralsRequired();

    function initialize(
        CollateralType.Data[] memory _data
    )
        internal
    {
        require(_data.length >= 2, AtLeastTwoCollateralsRequired());
        // initial pool collateral token
        require(_data[0].collateralClass == CollateralType.Class.POOL, NotAPoolCollateralAtZero());
        _add(_data[0]);
        _setPoolCollateralTypeIndex(0);
        // initial vault collateral tokens
        for (uint256 i = 1; i < _data.length; i++) {
            require(_data[i].collateralClass == CollateralType.Class.VAULT, NotAVaultCollateral());
            _add(_data[i]);
        }
    }

    function add(
        CollateralType.Data memory _data
    )
        internal
    {
        require(_data.collateralClass == CollateralType.Class.VAULT, NotAVaultCollateral());
        _add(_data);
    }

    function setPoolWNatCollateralType(
        CollateralType.Data memory _data
    )
        internal
    {
        uint256 index = _add(_data);
        _setPoolCollateralTypeIndex(index);
    }

    function getInfo(
        CollateralType.Class _collateralClass,
        IERC20 _token
    )
        internal view
        returns (CollateralType.Data memory)
    {
        CollateralTypeInt.Data storage token = CollateralTypes.get(_collateralClass, _token);
        return _getInfo(token);
    }

    function getAllInfos()
        internal view
        returns (CollateralType.Data[] memory _result)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        uint256 length = state.collateralTokens.length;
        _result = new CollateralType.Data[](length);
        for (uint256 i = 0; i < length; i++) {
            _result[i] = _getInfo(state.collateralTokens[i]);
        }
    }

    function get(
        CollateralType.Class _collateralClass,
        IERC20 _token
    )
        internal view
        returns (CollateralTypeInt.Data storage)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        uint256 index = state.collateralTokenIndex[_tokenKey(_collateralClass, _token)];
        require(index > 0, UnknownToken());
        return state.collateralTokens[index - 1];
    }

    function getIndex(
        CollateralType.Class _collateralClass,
        IERC20 _token
    )
        internal view
        returns (uint256)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        uint256 index = state.collateralTokenIndex[_tokenKey(_collateralClass, _token)];
        require(index > 0, UnknownToken());
        return index - 1;
    }

    function exists(
        CollateralType.Class _collateralClass,
        IERC20 _token
    )
        internal view
        returns (bool)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        uint256 index = state.collateralTokenIndex[_tokenKey(_collateralClass, _token)];
        return index > 0;
    }

    function isValid(CollateralTypeInt.Data storage _token)
        internal view
        returns (bool)
    {
        return _token.validUntil == 0 || _token.validUntil > block.timestamp;
    }

    function _add(CollateralType.Data memory _data) private returns (uint256) {
        AssetManagerState.State storage state = AssetManagerState.get();
        // validation of collateralClass is done before call to _add
        require(address(_data.token) != address(0), TokenZero());
        bytes32 tokenKey = _tokenKey(_data.collateralClass, _data.token);
        require(state.collateralTokenIndex[tokenKey] == 0, TokenAlreadyExists());
        require(_data.validUntil == 0, CannotAddDeprecatedToken());
        // validate collateral ratios
        bool ratiosValid =
            SafePct.MAX_BIPS < _data.minCollateralRatioBIPS &&
            _data.minCollateralRatioBIPS <= _data.safetyMinCollateralRatioBIPS;
        require(ratiosValid, InvalidCollateralRatios());
        // check that prices are initialized in FTSO price reader
        (uint256 assetPrice,,) = Conversion.readFtsoPrice(_data.assetFtsoSymbol, false);
        require(assetPrice != 0, PriceNotInitialized());
        if (!_data.directPricePair) {
            (uint256 tokenPrice,,) = Conversion.readFtsoPrice(_data.tokenFtsoSymbol, false);
            require(tokenPrice != 0, PriceNotInitialized());
        }
        // add token
        uint256 newTokenIndex = state.collateralTokens.length;
        state.collateralTokens.push(CollateralTypeInt.Data({
            token: _data.token,
            collateralClass: _data.collateralClass,
            decimals: _data.decimals.toUint8(),
            validUntil: _data.validUntil.toUint64(),
            directPricePair: _data.directPricePair,
            assetFtsoSymbol: _data.assetFtsoSymbol,
            tokenFtsoSymbol: _data.tokenFtsoSymbol,
            minCollateralRatioBIPS: _data.minCollateralRatioBIPS.toUint32(),
            __ccbMinCollateralRatioBIPS: 0, // no longer used
            safetyMinCollateralRatioBIPS: _data.safetyMinCollateralRatioBIPS.toUint32()
        }));
        state.collateralTokenIndex[tokenKey] = newTokenIndex + 1;   // 0 means empty
        emit IAssetManagerEvents.CollateralTypeAdded(uint8(_data.collateralClass), address(_data.token),
            _data.decimals, _data.directPricePair, _data.assetFtsoSymbol, _data.tokenFtsoSymbol,
            _data.minCollateralRatioBIPS, _data.safetyMinCollateralRatioBIPS);
        return newTokenIndex;
    }

    function _setPoolCollateralTypeIndex(uint256 _index) private {
        AssetManagerState.State storage state = AssetManagerState.get();
        CollateralTypeInt.Data storage token = state.collateralTokens[_index];
        assert(token.collateralClass == CollateralType.Class.POOL);
        state.poolCollateralIndex = _index.toUint16();
    }

    function _getInfo(CollateralTypeInt.Data storage token)
        private view
        returns (CollateralType.Data memory)
    {
        return CollateralType.Data({
            token: token.token,
            collateralClass: token.collateralClass,
            decimals: token.decimals,
            validUntil: token.validUntil,
            directPricePair: token.directPricePair,
            assetFtsoSymbol: token.assetFtsoSymbol,
            tokenFtsoSymbol: token.tokenFtsoSymbol,
            minCollateralRatioBIPS: token.minCollateralRatioBIPS,
            safetyMinCollateralRatioBIPS: token.safetyMinCollateralRatioBIPS
        });
    }

    function _tokenKey(
        CollateralType.Class _collateralClass,
        IERC20 _token
    )
        private pure
        returns (bytes32)
    {
        return bytes32((uint256(_collateralClass) << 160) | uint256(uint160(address(_token))));
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Math} from "@openzeppelin/contracts/utils/math/Math.sol";
import {SafePct} from "../../utils/library/SafePct.sol";
import {MathUtils} from "../../utils/library/MathUtils.sol";
import {Collateral} from "./data/Collateral.sol";
import {Conversion} from "./Conversion.sol";
import {Agents} from "./Agents.sol";
import {Agent} from "./data/Agent.sol";
import {CollateralTypeInt} from "./data/CollateralTypeInt.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {AssetManagerSettings} from "../../userInterfaces/data/AssetManagerSettings.sol";
import {Globals} from "./Globals.sol";


library AgentCollateral {
    using SafePct for uint256;
    using Agent for Agent.State;
    using Agents for Agent.State;

    function combinedData(
        Agent.State storage _agent
    )
        internal view
        returns (Collateral.CombinedData memory)
    {
        Collateral.Data memory poolCollateral = poolCollateralData(_agent);
        return Collateral.CombinedData({
            agentCollateral: agentVaultCollateralData(_agent),
            poolCollateral: poolCollateral,
            agentPoolTokens: agentsPoolTokensCollateralData(_agent, poolCollateral)
        });
    }

    function singleCollateralData(
        Agent.State storage _agent,
        Collateral.Kind _kind
    )
        internal view
        returns (Collateral.Data memory)
    {
        if (_kind == Collateral.Kind.VAULT) {
            return agentVaultCollateralData(_agent);
        } else if (_kind == Collateral.Kind.POOL) {
            return poolCollateralData(_agent);
        } else {
            return agentsPoolTokensCollateralData(_agent, poolCollateralData(_agent));
        }
    }

    function agentVaultCollateralData(
        Agent.State storage _agent
    )
        internal view
        returns (Collateral.Data memory)
    {
        CollateralTypeInt.Data storage collateral = _agent.getVaultCollateral();
        return Collateral.Data({
            kind: Collateral.Kind.VAULT,
            fullCollateral: collateral.token.balanceOf(_agent.vaultAddress()),
            amgToTokenWeiPrice: Conversion.currentAmgPriceInTokenWei(collateral)
        });
    }

    function poolCollateralData(
        Agent.State storage _agent
    )
        internal view
        returns (Collateral.Data memory)
    {
        CollateralTypeInt.Data storage collateral = _agent.getPoolCollateral();
        return Collateral.Data({
            kind: Collateral.Kind.POOL,
            fullCollateral: _agent.collateralPool.totalCollateral(),
            amgToTokenWeiPrice: Conversion.currentAmgPriceInTokenWei(collateral)
        });
    }

    function agentsPoolTokensCollateralData(
        Agent.State storage _agent,
        Collateral.Data memory _poolCollateral
    )
        internal view
        returns (Collateral.Data memory)
    {
        IERC20 poolToken = _agent.collateralPool.poolToken();
        uint256 agentPoolTokens = poolToken.balanceOf(_agent.vaultAddress());
        uint256 totalPoolTokens = poolToken.totalSupply();
        uint256 amgToPoolTokenWeiPrice = _poolCollateral.fullCollateral != 0
            ? _poolCollateral.amgToTokenWeiPrice.mulDiv(totalPoolTokens, _poolCollateral.fullCollateral)
            : _poolCollateral.amgToTokenWeiPrice;   // price for empty pool is 1 token/NAT
        return Collateral.Data({
            kind: Collateral.Kind.AGENT_POOL,
            fullCollateral: agentPoolTokens,
            amgToTokenWeiPrice: amgToPoolTokenWeiPrice
        });
    }

    // The max number of lots the agent can mint
    function freeCollateralLots(
        Collateral.CombinedData memory _data,
        Agent.State storage _agent
    )
        internal view
        returns (uint256 _lots)
    {
        return freeCollateralLotsOptionalFee(_data, _agent, true);
    }

    function freeCollateralLotsOptionalFee(
        Collateral.CombinedData memory _data,
        Agent.State storage _agent,
        bool _chargePoolFee
    )
        internal view
        returns (uint256 _lots)
    {
        uint256 agentLots = freeSingleCollateralLots(_data.agentCollateral, _agent, _chargePoolFee);
        uint256 poolLots = freeSingleCollateralLots(_data.poolCollateral, _agent, _chargePoolFee);
        uint256 agentPoolTokenLots = freeSingleCollateralLots(_data.agentPoolTokens, _agent, _chargePoolFee);
        return Math.min(agentLots, Math.min(poolLots, agentPoolTokenLots));
    }

    function freeSingleCollateralLots(
        Collateral.Data memory _data,
        Agent.State storage _agent,
        bool _chargePoolFee
    )
        internal view
        returns (uint256)
    {
        uint256 collateralWei = freeCollateralWei(_data, _agent);
        uint256 lotWei = mintingLotCollateralWei(_data, _agent, _chargePoolFee);
        // lotWei=0 is possible only for agent's pool token collateral if pool balance in NAT is 0
        // so then we can safely return 0 here, since minting is impossible
        return lotWei != 0 ? collateralWei / lotWei : 0;
    }

    function freeCollateralWei(
        Collateral.Data memory _data,
        Agent.State storage _agent
    )
        internal view
        returns (uint256)
    {
        uint256 lockedCollateral = lockedCollateralWei(_data, _agent);
        return MathUtils.subOrZero(_data.fullCollateral, lockedCollateral);
    }

    // Amount of collateral NOT available for new minting or withdrawal.
    function lockedCollateralWei(
        Collateral.Data memory _data,
        Agent.State storage _agent
    )
        internal view
        returns (uint256)
    {
        (uint256 mintingMinCollateralRatioBIPS, uint256 systemMinCollateralRatioBIPS) =
            mintingMinCollateralRatio(_agent, _data.kind);
        uint256 backedAMG = uint256(_agent.reservedAMG) + uint256(_agent.mintedAMG);
        uint256 mintingCollateral = Conversion.convertAmgToTokenWei(backedAMG, _data.amgToTokenWeiPrice)
            .mulBips(mintingMinCollateralRatioBIPS);
        uint64 redeemingAMG = _data.kind == Collateral.Kind.POOL ? _agent.poolRedeemingAMG : _agent.redeemingAMG;
        uint256 redeemingCollateral = Conversion.convertAmgToTokenWei(redeemingAMG, _data.amgToTokenWeiPrice)
            .mulBips(systemMinCollateralRatioBIPS);
        uint256 announcedWithdrawal =
            _data.kind != Collateral.Kind.POOL ? _agent.withdrawalAnnouncement(_data.kind).amountWei : 0;
        return mintingCollateral + redeemingCollateral + announcedWithdrawal;
    }

    function mintingLotCollateralWei(
        Collateral.Data memory _data,
        Agent.State storage _agent,
        bool _chargePoolFee
    )
        internal view
        returns (uint256)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        return collateralRequiredToMintAmount(_data, _agent, settings.lotSizeAMG, _chargePoolFee);
    }

    function collateralRequiredToMintAmount(
        Collateral.Data memory _data,
        Agent.State storage _agent,
        uint256 _amountAMG,
        bool _chargePoolFee
    )
        internal view
        returns (uint256)
    {
        uint256 amountPoolFeeAMG =
            _chargePoolFee ? _amountAMG.mulBips(_agent.feeBIPS).mulBips(_agent.poolFeeShareBIPS) : 0;
        uint256 totalMintAmountAMG = _amountAMG + amountPoolFeeAMG;
        uint256 totalMintAmountWei = Conversion.convertAmgToTokenWei(totalMintAmountAMG, _data.amgToTokenWeiPrice);
        (uint256 mintingCollateralRatio,) = mintingMinCollateralRatio(_agent, _data.kind);
        return totalMintAmountWei.mulBips(mintingCollateralRatio);
    }

    function mintingMinCollateralRatio(
        Agent.State storage _agent,
        Collateral.Kind _kind
    )
        internal view
        returns (uint256 _mintingMinCollateralRatioBIPS, uint256 _systemMinCollateralRatioBIPS)
    {
        if (_kind == Collateral.Kind.AGENT_POOL) {
            uint256 mintingPoolHoldingsRequiredBIPS = Globals.getSettings().mintingPoolHoldingsRequiredBIPS;
            _systemMinCollateralRatioBIPS = mintingPoolHoldingsRequiredBIPS;
            _mintingMinCollateralRatioBIPS = mintingPoolHoldingsRequiredBIPS;
        } else if (_kind == Collateral.Kind.POOL) {
            _systemMinCollateralRatioBIPS = _agent.getPoolCollateral().minCollateralRatioBIPS;
            _mintingMinCollateralRatioBIPS =
                Math.max(_agent.mintingPoolCollateralRatioBIPS, _systemMinCollateralRatioBIPS);
        } else {
            _systemMinCollateralRatioBIPS = _agent.getVaultCollateral().minCollateralRatioBIPS;
            // agent's minCollateralRatioBIPS must be greater than minCollateralRatioBIPS when set, but
            // minCollateralRatioBIPS can change later so we always use the max of both
            _mintingMinCollateralRatioBIPS =
                Math.max(_agent.mintingVaultCollateralRatioBIPS, _systemMinCollateralRatioBIPS);
        }
    }

    // Used for redemption default payment - calculate all types of collateral at the same rate, so that
    // future redemptions don't get less than this one (unless the price changes).
    // Ignores collateral announced for withdrawal (redemption has priority over withdrawal).
    function maxRedemptionCollateral(
        Collateral.Data memory _data,
        Agent.State storage _agent,
        uint256 _valueAMG
    )
        internal view
        returns (uint256)
    {
        if (_valueAMG == 0) return 0;
        // Assume: collateral kind is VAULT or redemption is NOT a pool self close redemption.
        // For pool self close redemptions, pool collateral is never paid, so this method is not used.
        uint256 redeemingAMG = _data.kind == Collateral.Kind.POOL ? _agent.poolRedeemingAMG : _agent.redeemingAMG;
        assert(_valueAMG <= redeemingAMG);
        uint256 totalAMG = uint256(_agent.mintedAMG) + uint256(_agent.reservedAMG) + uint256(redeemingAMG);
        return _data.fullCollateral.mulDiv(_valueAMG, totalAMG); // totalAMG > 0 (guarded by assert)
    }

    // Agent's collateral ratio for single collateral type (BIPS) - used in liquidation.
    // Ignores collateral announced for withdrawal (withdrawals are forbidden during liquidation).
    function collateralRatioBIPS(
        Collateral.Data memory _data,
        Agent.State storage _agent
    )
        internal view
        returns (uint256)
    {
        uint256 totalAMG = totalBackedAMG(_agent, _data.kind);
        uint256 backingTokenWei = Conversion.convertAmgToTokenWei(totalAMG, _data.amgToTokenWeiPrice);
        if (backingTokenWei == 0) return 1e10;    // nothing minted - ~infinite collateral ratio (but avoid overflows)
        return _data.fullCollateral.mulDiv(SafePct.MAX_BIPS, backingTokenWei);
    }

    function totalBackedAMG(
        Agent.State storage _agent,
        Collateral.Kind _collateralKind
    )
        internal view
        returns (uint256)
    {
        uint256 redeemingAMG = _collateralKind == Collateral.Kind.POOL ? _agent.poolRedeemingAMG : _agent.redeemingAMG;
        return uint256(_agent.mintedAMG) + uint256(_agent.reservedAMG) + uint256(redeemingAMG);
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


library Collateral {
    enum Kind {
        VAULT,   // vault collateral (tokens in in agent vault)
        POOL,           // pool collateral (NAT)
        AGENT_POOL      // agent's pool tokens (expressed in NAT) - only important for minting
    }

    struct Data {
        Kind kind;
        uint256 fullCollateral;
        uint256 amgToTokenWeiPrice;
    }

    struct CombinedData {
        Collateral.Data agentCollateral;
        Collateral.Data poolCollateral;
        Collateral.Data agentPoolTokens;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

library MathUtils {
    /**
     * Increases the value `x` to a whole multiple of `rounding`.
     */
    function roundUp(uint256 x, uint256 rounding) internal pure returns (uint256) {
        // division by 0 and overflow checks preformed by Solidity >= 0.8
        uint256 remainder = x % rounding;
        return remainder == 0 ? x : x - remainder + rounding;
    }

    /**
     * Return the positive part of `_a - _b`.
     */
    function subOrZero(uint256 _a, uint256 _b) internal pure returns (uint256) {
        return _a > _b ? _a - _b : 0;
    }

    /**
     * Returns _x if it is positive, otherwise 0.
     */
    function positivePart(int256 _x) internal pure returns (uint256) {
        return _x >= 0 ? uint256(_x) : 0;
    }

    /**
     * Returns `_a <= _b`; works correctly when `_b` is any signed value.
     */
    function mixedLTE(uint256 _a, int256 _b) internal pure returns (bool) {
        return _b >= 0 && _a <= uint256(_b);
    }

    /**
     * Returns `_a <= _b`; works correctly when `_a` is any signed value.
     */
    function mixedLTE(int256 _a, uint256 _b) internal pure returns (bool) {
        return _a <= 0 || uint256(_a) <= _b;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {AssetManagerState} from "./data/AssetManagerState.sol";
import {IAssetManagerEvents} from "../../userInterfaces/IAssetManagerEvents.sol";
import {Globals} from "./Globals.sol";
import {Conversion} from "./Conversion.sol";
import {Agent} from "./data/Agent.sol";
import {RedemptionQueue} from "./data/RedemptionQueue.sol";
import {AssetManagerSettings} from "../../userInterfaces/data/AssetManagerSettings.sol";


library AgentBacking {
    using Agent for Agent.State;
    using RedemptionQueue for RedemptionQueue.State;

    function releaseMintedAssets(
        Agent.State storage _agent,
        uint64 _valueAMG
    )
        internal
    {
        _agent.mintedAMG = _agent.mintedAMG - _valueAMG;
    }

    function startRedeemingAssets(
        Agent.State storage _agent,
        uint64 _valueAMG,
        bool _poolSelfCloseRedemption
    )
        internal
    {
        _agent.redeemingAMG += _valueAMG;
        if (!_poolSelfCloseRedemption) {
            _agent.poolRedeemingAMG += _valueAMG;
        }
        releaseMintedAssets(_agent, _valueAMG);
    }

    function endRedeemingAssets(
        Agent.State storage _agent,
        uint64 _valueAMG,
        bool _poolSelfCloseRedemption
    )
        internal
    {
        _agent.redeemingAMG = _agent.redeemingAMG - _valueAMG;
        if (!_poolSelfCloseRedemption) {
            _agent.poolRedeemingAMG = _agent.poolRedeemingAMG - _valueAMG;
        }
    }

    function createNewMinting(
        Agent.State storage _agent,
        uint64 _valueAMG
    )
        internal
    {
        // allocate minted assets
        _agent.mintedAMG += _valueAMG;

        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // Add value with dust, then take the whole number of lots from it to create the new ticket,
        // and the remainder as new dust. At the end, there will always be less than 1 lot of dust left.
        uint64 valueWithDustAMG = _agent.dustAMG + _valueAMG;
        uint64 newDustAMG = valueWithDustAMG % settings.lotSizeAMG;
        uint64 ticketValueAMG = valueWithDustAMG - newDustAMG;
        // create ticket and change dust
        if (ticketValueAMG > 0) {
            createRedemptionTicket(_agent, ticketValueAMG);
        }
        changeDust(_agent, newDustAMG);
    }

    function createRedemptionTicket(
        Agent.State storage _agent,
        uint64 _ticketValueAMG
    )
        internal
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        if (_ticketValueAMG == 0) return;
        address vaultAddress = _agent.vaultAddress();
        uint64 lastTicketId = state.redemptionQueue.lastTicketId;
        RedemptionQueue.Ticket storage lastTicket = state.redemptionQueue.getTicket(lastTicketId);
        if (lastTicket.agentVault == vaultAddress) {
            // last ticket is from the same agent - merge the new ticket with the last
            lastTicket.valueAMG += _ticketValueAMG;
            uint256 ticketValueUBA = Conversion.convertAmgToUBA(lastTicket.valueAMG);
            emit IAssetManagerEvents.RedemptionTicketUpdated(vaultAddress, lastTicketId, ticketValueUBA);
        } else {
            // either queue is empty or the last ticket belongs to another agent - create new ticket
            uint64 ticketId = state.redemptionQueue.createRedemptionTicket(vaultAddress, _ticketValueAMG);
            uint256 ticketValueUBA = Conversion.convertAmgToUBA(_ticketValueAMG);
            emit IAssetManagerEvents.RedemptionTicketCreated(vaultAddress, ticketId, ticketValueUBA);
        }
    }

    function changeDust(
        Agent.State storage _agent,
        uint64 _newDustAMG
    )
        internal
    {
        if (_agent.dustAMG == _newDustAMG) return;
        _agent.dustAMG = _newDustAMG;
        uint256 dustUBA = Conversion.convertAmgToUBA(_newDustAMG);
        emit IAssetManagerEvents.DustChanged(_agent.vaultAddress(), dustUBA);
    }

    function decreaseDust(
        Agent.State storage _agent,
        uint64 _dustDecreaseAMG
    )
        internal
    {
        uint64 newDustAMG = _agent.dustAMG - _dustDecreaseAMG;
        changeDust(_agent, newDustAMG);
    }
}
// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;


library AssetManagerSettings {
    struct Data {
        // Required contracts.
        // Only used to verify that calls come from assetManagerController.
        // Type: AssetManagerController
        // changed via address updater
        address assetManagerController;

        // The f-asset contract managed by this asset manager.
        // Type: IIFAsset
        // immutable
        address fAsset;

        // Factory for creating new agent vaults.
        // Type: IIAgentVaultFactory
        // timelocked
        address agentVaultFactory;

        // Factory for creating new agent collateral pools.
        // Type: IICollateralPoolFactory
        // timelocked
        address collateralPoolFactory;

        // Factory for creating new agent collateral pool tokens.
        // Type: IICollateralPoolTokenFactory
        // timelocked
        address collateralPoolTokenFactory;

        // The suffix to pool token name and symbol that identifies new vault's collateral pool token.
        // When vault is created, the owner passes own suffix which will be appended to this.
        string poolTokenSuffix;

        // If set, the whitelist contains a list of accounts that can call public methods
        // (minting, redeeming, challenging, etc.)
        // This can be `address(0)`, in which case no whitelist checks are done.
        // Type: IWhitelist
        // timelocked
        address __whitelist; // only storage placeholder

        // If set, the owner address registry contains a list of allowed agent owner's
        // management addresses and mappings from management to work address.
        // Type: IAgentOwnerRegistry
        // timelocked
        address agentOwnerRegistry;

        // Attestation client verifies and decodes attestation proofs.
        // Type: IFdcVerification
        // changed via address updater
        address fdcVerification;

        // The address where burned NAT is sent.
        // immutable
        address payable burnAddress;

        // The contract that reads prices from FTSO system in an FTSO version independent way.
        // Type: IPriceReader
        // timelocked
        address priceReader;

        // Same as assetToken.decimals()
        // immutable
        uint8 assetDecimals;

        // Number of decimals of precision of minted amounts.
        // assetMintingGranularityUBA = 10 ** (assetDecimals - assetMintingDecimals)
        // immutable
        uint8 assetMintingDecimals;

        // Must match attestation data chainId.
        // immutable
        bytes32 chainId;

        // Average time between two successive blocks on the underlying chain, in milliseconds.
        // rate-limited
        uint32 averageBlockTimeMS;

        // The minimum amount of pool tokens the agent must hold to be able to mint.
        // To be able to mint, the NAT value of all backed fassets together with new ones times this percentage
        // must be smaller than the agent's pool tokens' amount converted to NAT.
        // rate-limited
        uint32 mintingPoolHoldingsRequiredBIPS;

        // Collateral reservation fee that must be paid by the minter.
        // Payment is in NAT, but is proportional to the value of assets to be minted.
        // rate-limited
        uint16 collateralReservationFeeBIPS;

        // Asset unit value (e.g. 1 BTC or 1 ETH) in UBA = 10 ** assetToken.decimals()
        // immutable
        uint64 assetUnitUBA;

        // The granularity in which lots are measured = the value of AMG (asset minting granularity) in UBA.
        // Can only be changed via redeploy of AssetManager.
        // AMG is used internally instead of UBA so that minted quantities fit into 64bits to reduce storage.
        // So assetMintingGranularityUBA should be set so that the max supply in AMG of this currency
        // in foreseeable time (say 100yr) cannot overflow 64 bits.
        // immutable
        uint64 assetMintingGranularityUBA;

        // Lot size in asset minting granularity. May change, which affects subsequent mintings and redemptions.
        // timelocked
        uint64 lotSizeAMG;

        // The percentage of minted f-assets that the agent must hold in his underlying address.
        uint16 __minUnderlyingBackingBIPS; // only storage placeholder

        // for some chains (e.g. Ethereum) we require that agent proves that underlying address is an EOA address
        // this must be done by presenting a payment proof from that address
        // immutable
        bool __requireEOAAddressProof; // only storage placeholder

        // Maximum minted amount of the f-asset.
        // rate-limited
        uint64 mintingCapAMG;

        // Number of underlying blocks that the minter or agent is allowed to pay underlying value.
        // If payment not reported in that time, minting/redemption can be challenged and default action triggered.
        // CAREFUL: Count starts from the current proved block height, so the minters and agents should
        // make sure that current block height is fresh, otherwise they might not have enough time for payment.
        // timelocked
        uint64 underlyingBlocksForPayment;

        // Minimum time to allow agent to pay for redemption or minter to pay for minting.
        // This is useful for fast chains, when there can be more than one block per second.
        // Redemption/minting payment failure can be called only after underlyingSecondsForPayment have elapsed
        // on underlying chain.
        // CAREFUL: Count starts from the current proved block timestamp, so the minters and agents should
        // make sure that current block timestamp is fresh, otherwise they might not have enough time for payment.
        // This is partially mitigated by adding local duration since the last block height update to
        // the current underlying block timestamp.
        // timelocked
        uint64 underlyingSecondsForPayment;

        // Redemption fee in underlying currency base amount (UBA).
        // rate-limited
        uint16 redemptionFeeBIPS;

        // On redemption underlying payment failure, redeemer is compensated with
        // redemption value recalculated in flare/sgb times redemption failure factor.
        // Expressed in BIPS, e.g. 12000 for factor of 1.2.
        // This is the part of factor paid from agent's vault collateral.
        // rate-limited
        uint32 redemptionDefaultFactorVaultCollateralBIPS;

        // This is the part of redemption factor paid from agent's pool collateral.
        // rate-limited
        uint32 __redemptionDefaultFactorPoolBIPS; // only storage placeholder

        // If the agent or redeemer becomes unresponsive, we still need payment or non-payment confirmations
        // to be presented eventually to properly track agent's underlying balance.
        // Therefore we allow anybody to confirm payments/non-payments this many seconds after request was made.
        // rate-limited
        uint64 confirmationByOthersAfterSeconds;

        // The user who makes abandoned redemption confirmations gets rewarded by the following amount.
        // rate-limited
        uint128 confirmationByOthersRewardUSD5;

        // To prevent unbounded work, the number of tickets redeemed in a single request is limited.
        // rate-limited
        // >= 1
        uint16 maxRedeemedTickets;

        // Challenge reward can be composed of two part - fixed and proportional (any of them can be zero).
        // This is the proportional part (in BIPS).
        // rate-limited
        uint16 paymentChallengeRewardBIPS;

        // Challenge reward can be composed of two part - fixed and proportional (any of them can be zero).
        // This is the fixed part (in vault collateral token wei).
        // rate-limited
        uint128 paymentChallengeRewardUSD5;

        // Agent has to announce any collateral withdrawal ar vault destroy and then wait for at least
        // withdrawalWaitMinSeconds. This prevents challenged agent to remove all collateral before
        // challenge can be proved.
        // rate-limited
        uint64 withdrawalWaitMinSeconds;

        // Maximum age that trusted price feed is valid.
        // Otherwise (if there were no trusted votes for that long) just use generic ftso price feed.
        // rate-limited
        uint64 maxTrustedPriceAgeSeconds;

        // Agent can remain in CCB for this much time, after that liquidation starts automatically.
        // rate-limited
        uint64 __ccbTimeSeconds; // only storage placeholder

        // Amount of seconds (typically 1 day) that the payment/non-payment proofs must be available.
        // This setting is used in `unstickMinting` and `finishRedemptionWithoutPayment` to prove that the time when
        // payment/non-payment could be proved has already passed.
        // rate-limited
        uint64 attestationWindowSeconds;

        // Minimum time after an update of a setting before the same setting can be updated again.
        // timelocked
        uint64 minUpdateRepeatTimeSeconds;

        // Ratio at which the agents can buy back their collateral when f-asset is terminated.
        // Typically a bit more than 1 to incentivize agents to buy f-assets and self-close instead.
        // immutable
        uint64 __buybackCollateralFactorBIPS; // only storage placeholder

        // Minimum time that has to pass between underlying withdrawal announcement and the confirmation.
        // Any value is ok, but higher values give more security against multiple announcement attack by a miner.
        // Shouldn't be much bigger than Flare data connector response time, so that payments can be confirmed without
        // extra wait. Should be smaller than confirmationByOthersAfterSeconds (e.g. less than 1 hour).
        // rate-limited
        uint64 __announcedUnderlyingConfirmationMinSeconds;

        // Minimum time from the moment token is deprecated to when it becomes invalid and agents still using
        // it as vault collateral get liquidated.
        // timelocked
        uint64 tokenInvalidationTimeMinSeconds;

        // On some rare occasions (stuck minting), the agent has to unlock collateral.
        // For this, part of collateral corresponding to FTSO asset value is burned and the rest is released.
        // However, we cannot burn typical vault collateral (stablecoins), so the agent must buy them for NAT
        // at FTSO price multiplied with this factor (should be a bit above 1) and then we burn the NATs.
        // timelocked
        uint32 vaultCollateralBuyForFlareFactorBIPS;

        // Amount of seconds that have to pass between available list exit announcement and execution.
        // rate-limited
        uint64 agentExitAvailableTimelockSeconds;

        // Amount of seconds that have to pass between agent fee and pool fee share change announcement and execution.
        // rate-limited
        uint64 agentFeeChangeTimelockSeconds;

        // Amount of seconds that have to pass between agent-set minting collateral ratio (vault or pool)
        // change announcement and execution.
        // rate-limited
        uint64 agentMintingCRChangeTimelockSeconds;

        // Amount of seconds that have to pass between agent-set settings for pool exit collateral ratio
        // change announcement and execution.
        // rate-limited
        uint64 poolExitCRChangeTimelockSeconds;

        // Amount of seconds that an agent is allowed to execute an update once it is allowed.
        // rate-limited
        uint64 agentTimelockedOperationWindowSeconds;

        // duration of the timelock for collateral pool tokens after minting
        uint32 collateralPoolTokenTimelockSeconds;

        // If there was no liquidator for the current liquidation offer,
        // go to the next step of liquidation after a certain period of time.
        // rate-limited
        uint64 liquidationStepSeconds;

        // Factor with which to multiply the asset price in native currency to obtain the payment
        // to the liquidator.
        // Expressed in BIPS, e.g. [12000, 16000, 20000] means that the liquidator will be paid 1.2, 1.6 and 2.0
        // times the market price of the liquidated assets after each `liquidationStepSeconds`.
        // Values in the array must increase and be greater than 100%.
        // rate-limited
        uint256[] liquidationCollateralFactorBIPS;

        // How much of the liquidation is paid in vault collateral.
        // The remainder will be paid in pool NAT collateral.
        uint256[] liquidationFactorVaultCollateralBIPS;

        // Minimum time that the system must wait before performing diamond cut.
        // The actual timelock is the maximum of this setting and GovernanceSettings.timelock.
        uint64 diamondCutMinTimelockSeconds;

        // The maximum total pause that can be triggered by non-governance (but governance allowed) caller.
        // The duration count can be reset by the governance.
        uint64 maxEmergencyPauseDurationSeconds;

        // The amount of time since last emergency pause after which the total pause duration counter
        // will reset automatically.
        uint64 emergencyPauseDurationResetAfterSeconds;

        // The amount of time after which the collateral reservation can be cancelled if the
        // handshake is not completed.
        // rate-limited
        uint64 __cancelCollateralReservationAfterSeconds; // only storage placeholder

        // The amount of collateral reservation fee returned to the minter in case of rejection or cancellation.
        // Expressed in BIPS, e.g. 9500 for factor of 0.95, max 10000 for factor of 1.0.
        // rate-limited
        uint16 __rejectOrCancelCollateralReservationReturnFactorBIPS; // only storage placeholder

        // Time window inside which the agent can reject the redemption request.
        // rate-limited
        uint64 __rejectRedemptionRequestWindowSeconds; // only storage placeholder

        // Time window inside which the agent can take over the redemption request from another agent
        // that has rejected it.
        // rate-limited
        uint64 __takeOverRedemptionRequestWindowSeconds; // only storage placeholder

        // On redemption rejection, without take over, redeemer is compensated with
        // redemption value recalculated in flare/sgb times redemption failure factor.
        // Expressed in BIPS, e.g. 12000 for factor of 1.2.
        // This is the part of factor paid from agent's vault collateral.
        // rate-limited
        uint32 __rejectedRedemptionDefaultFactorVaultCollateralBIPS; // only storage placeholder

        // This is the part of rejected redemption factor paid from agent's pool collateral.
        // rate-limited
        uint32 __rejectedRedemptionDefaultFactorPoolBIPS; // only storage placeholder
    }
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;
pragma abicoder v2;

import {IDistributionToDelegators} from "@flarenetwork/flare-periphery-contracts/flare/IDistributionToDelegators.sol";
import {IRewardManager} from "@flarenetwork/flare-periphery-contracts/flare/IRewardManager.sol";
import {ICollateralPoolToken} from "./ICollateralPoolToken.sol";


interface ICollateralPool {
    event CPEntered(
        address indexed tokenHolder,
        uint256 amountNatWei,
        uint256 receivedTokensWei,
        uint256 timelockExpiresAt);

    event CPExited(
        address indexed tokenHolder,
        uint256 burnedTokensWei,
        uint256 receivedNatWei);

    event CPSelfCloseExited(
        address indexed tokenHolder,
        uint256 burnedTokensWei,
        uint256 receivedNatWei,
        uint256 closedFAssetsUBA);

    event CPFeeDebtPaid(
        address indexed tokenHolder,
        uint256 paidFeesUBA);

    event CPFeesWithdrawn(
        address indexed tokenHolder,
        uint256 withdrawnFeesUBA);

    event CPFeeDebtChanged(
        address indexed tokenHolder,
        int256 newFeeDebtUBA);

    // Emitted when asset manager forces payout from the pool
    event CPPaidOut(
        address indexed recipient,
        uint256 paidNatWei,
        uint256 burnedTokensWei);

    event CPClaimedReward(
        uint256 amountNatWei,
        uint8 rewardType);

    error OnlyAssetManager();
    error OnlyAgent();
    error AlreadyInitialized();
    error OnlyInternalUse();
    error PoolTokenAlreadySet();
    error AmountOfNatTooLow();
    error AmountOfCollateralTooLow();
    error DepositResultsInZeroTokens();
    error TokenShareIsZero();
    error TokenBalanceTooLow();
    error SentAmountTooLow();
    error CollateralRatioFallsBelowExitCR();
    error InvalidRecipientAddress();
    error RedemptionRequiresClosingTooManyTickets();
    error FreeFAssetBalanceTooSmall();
    error WithdrawZeroFAsset();
    error ZeroFAssetDebtPayment();
    error PaymentLargerThanFeeDebt();
    error FAssetAllowanceTooSmall();
    error TokenSupplyAfterExitTooLow();
    error CollateralAfterExitTooLow();
    error CannotDestroyPoolWithIssuedTokens();

    /**
     * Enters the collateral pool by depositing NAT.
     * The tokens have a timelock before which exiting or transferring tokens is not possible.
     * If there are some FAsset fees already in the pool, tokens also have some fee debt, which
     * has to be paid off to make tokens transferable (but they can be redeemed).
     */
    function enter()
        external payable
        returns (uint256 _receivedTokens, uint256 _timelockExpiresAt);

    /**
     * Exits the pool by redeeming the given amount of pool tokens for a share of NAT and f-asset fees.
     * Exiting with non-transferable tokens awards the user with NAT only, while transferable tokens also entitle
     * one to a share of f-asset fees. As there are multiple ways to split spending transferable and
     * non-transferable tokens, the method also takes a parameter called `_exitType`.
     * Exiting with collateral that sinks pool's collateral ratio below exit CR is not allowed and
     *  will revert. In that case, see selfCloseExit.
     * @param _tokenShare   The amount of pool tokens to be redeemed
     */
    function exit(uint256 _tokenShare)
        external
        returns (uint256 _natShare);

    /**
     * Exits the pool by redeeming the given amount of pool tokens and burning f-assets in a way that doesn't
     * endanger the pool collateral ratio. Specifically, if pool's collateral ratio is above exit CR, then
     * the method burns an amount of user's f-assets that do not lower collateral ratio below exit CR. If, on
     * the other hand, collateral pool is below exit CR, then the method burns an amount of user's f-assets
     * that preserve the pool's collateral ratio.
     * F-assets will be redeemed in collateral if their value does not exceed one lot, regardless of
     *  `_redeemToCollateral` value.
     * Method first tries to satisfy the condition by taking f-assets out of sender's f-asset fee share,
     *  specified by `_tokenShare`. If it is not enough it moves on to spending total sender's f-asset fees. If they
     *  are not enough, it takes from the sender's f-asset balance. Spending sender's f-asset fees means that
     *  transferable tokens are converted to non-transferable.
     * In case of self-close via redemption, the user can set executor to trigger possible default.
     * In this case, some NAT can be sent with transaction, to pay the executor's fee.
     * @param _tokenShare                   The amount of pool tokens to be liquidated
     * @param _redeemToCollateral           Specifies if redeemed f-assets should be exchanged to vault collateral
     *                                      by the agent
     * @param _redeemerUnderlyingAddress    Redeemer's address on the underlying chain
     * @param _executor                     The account that is allowed to execute redemption default
     */
    function selfCloseExit(
        uint256 _tokenShare,
        bool _redeemToCollateral,
        string memory _redeemerUnderlyingAddress,
        address payable _executor
    ) external payable;

    /**
     * Collect f-asset fees by locking an appropriate ratio of transferable tokens
     * @param _amount  The amount of f-asset fees to withdraw.
     *                 Must be positive and smaller or equal to the sender's fAsset fees.
     */
    function withdrawFees(uint256 _amount) external;

    /**
     * Exits the pool by redeeming the given amount of pool tokens for a share of NAT and f-asset fees.
     * Exiting with non-transferable tokens awards the user with NAT only, while transferable tokens also entitle
     * one to a share of f-asset fees. As there are multiple ways to split spending transferable and
     * non-transferable tokens, the method also takes a parameter called `_exitType`.
     * Exiting with collateral that sinks pool's collateral ratio below exit CR is not allowed and
     *  will revert. In that case, see selfCloseExit.
     * @param _tokenShare   The amount of pool tokens to be redeemed
     * @param _recipient    The address to which NATs and FAsset fees will be transferred
     */
    function exitTo(uint256 _tokenShare, address payable _recipient)
        external
        returns (uint256 _natShare);

    /**
     * Exits the pool by redeeming the given amount of pool tokens and burning f-assets in a way that doesn't
     * endanger the pool collateral ratio. Specifically, if pool's collateral ratio is above exit CR, then
     * the method burns an amount of user's f-assets that do not lower collateral ratio below exit CR. If, on
     * the other hand, collateral pool is below exit CR, then the method burns an amount of user's f-assets
     * that preserve the pool's collateral ratio.
     * F-assets will be redeemed in collateral if their value does not exceed one lot, regardless of
     *  `_redeemToCollateral` value.
     * Method first tries to satisfy the condition by taking f-assets out of sender's f-asset fee share,
     *  specified by `_tokenShare`. If it is not enough it moves on to spending total sender's f-asset fees. If they
     *  are not enough, it takes from the sender's f-asset balance. Spending sender's f-asset fees means that
     *  transferable tokens are converted to non-transferable.
     * In case of self-close via redemption, the user can set executor to trigger possible default.
     * In this case, some NAT can be sent with transaction, to pay the executor's fee.
     * @param _tokenShare                   The amount of pool tokens to be liquidated
     * @param _redeemToCollateral           Specifies if redeemed f-assets should be exchanged to vault collateral
     *                                      by the agent
     * @param _recipient                    The address to which NATs and FAsset fees will be transferred
     * @param _redeemerUnderlyingAddress    Redeemer's address on the underlying chain
     * @param _executor                     The account that is allowed to execute redemption default
     */
    function selfCloseExitTo(
        uint256 _tokenShare,
        bool _redeemToCollateral,
        address payable _recipient,
        string memory _redeemerUnderlyingAddress,
        address payable _executor
    ) external payable;

    /**
     * Collect f-asset fees by locking an appropriate ratio of transferable tokens
     * @param _amount       The amount of f-asset fees to withdraw.
     *                      Must be positive and smaller or equal to the sender's fAsset fees.
     * @param _recipient    The address to which FAsset fees will be transferred
     */
    function withdrawFeesTo(uint256 _amount, address _recipient) external;

    /**
     * Unlock pool tokens by paying f-asset fee debt
     * @param _fassets  The amount of debt f-asset fees to pay for
     */
    function payFAssetFeeDebt(uint256 _fassets) external;

    /**
     * Claim airdrops earned by holding wrapped native tokens in the pool.
     * NOTE: only the owner of the pool's corresponding agent vault may call this method.
     */
    function claimAirdropDistribution(
        IDistributionToDelegators _distribution,
        uint256 _month
    ) external
        returns(uint256 _claimedAmount);

    /**
     * Opt out of airdrops for wrapped native tokens in the pool.
     * NOTE: only the owner of the pool's corresponding agent vault may call this method.
     */
    function optOutOfAirdrop(
        IDistributionToDelegators _distribution
    ) external;

    /**
     * Delegate WNat vote power for the wrapped native tokens held in this vault.
     * NOTE: only the owner of the pool's corresponding agent vault may call this method.
     */
    function delegate(address _to, uint256 _bips) external;

    /**
     * Clear WNat delegation.
     */
    function undelegateAll() external;

    /**
     * Claim the rewards earned by delegating the vote power for the pool.
     * NOTE: only the owner of the pool's corresponding agent vault may call this method.
     */
    function claimDelegationRewards(
        IRewardManager _rewardManager,
        uint24 _lastRewardEpoch,
        IRewardManager.RewardClaimWithProof[] calldata _proofs
    ) external
        returns(uint256 _claimedAmount);

    /**
     * Get the ERC20 pool token used by this collateral pool
     */
    function poolToken()
        external view
        returns (ICollateralPoolToken);

    /**
     * Get the vault of the agent that owns this collateral pool
     */
    function agentVault()
        external view
        returns (address);

    /**
     * Get the exit collateral ratio in BIPS
     * This is the collateral ratio below which exiting the pool is not allowed
     */
    function exitCollateralRatioBIPS()
        external view
        returns (uint32);


    /**
     * Return total amount of collateral in the pool.
     * This can be different to WNat.balanceOf(poolAddress), because the collateral has to be tracked
     * to prevent unexpected deposit type of attacks on the pool.
     */
    function totalCollateral()
        external view
        returns (uint256);

    /**
     * Returns the f-asset fees belonging to this user.
     * This is the amount of f-assets the user can withdraw by burning transferable pool tokens.
     * @param _account User address
     */
    function fAssetFeesOf(address _account)
        external view
        returns (uint256);

    /**
     * Returns the total f-asset fees in the pool.
     * This can be different to FAsset.balanceOf(poolAddress), because the collateral has to be tracked
     * to prevent unexpected deposit type of attacks on the pool.
     */
    function totalFAssetFees()
        external view
        returns (uint256);

    /**
     * Returns the user's f-asset fee debt.
     * This is the amount of f-assets the user has to pay to make all pool tokens transferable.
     * The debt is created on entering the pool if the user doesn't provide the f-assets corresponding
     * to the share of the f-asset fees already in the pool.
     * @param _account User address
     */
    function fAssetFeeDebtOf(address _account)
        external view
        returns (int256);

    /**
     * Returns the total f-asset fee debt for all users.
     */
    function totalFAssetFeeDebt()
        external view
        returns (int256);

    /**
     * Get the amount of fassets that need to be burned to perform self close exit.
     */
    function fAssetRequiredForSelfCloseExit(uint256 _tokenAmountWei)
        external view
        returns (uint256);
}
// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import "./IPayment.sol";

interface IPaymentVerification {
    function verifyPayment(
        IPayment.Proof calldata _proof
    ) external view returns (bool _proved);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Math} from "@openzeppelin/contracts/utils/math/Math.sol";
import {SafePct} from "../../utils/library/SafePct.sol";
import {MathUtils} from "../../utils/library/MathUtils.sol";
import {Collateral} from "./data/Collateral.sol";
import {Conversion} from "./Conversion.sol";
import {Agents} from "./Agents.sol";
import {Agent} from "./data/Agent.sol";
import {CollateralTypeInt} from "./data/CollateralTypeInt.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {AssetManagerSettings} from "../../userInterfaces/data/AssetManagerSettings.sol";
import {Globals} from "./Globals.sol";


library AgentCollateral {
    using SafePct for uint256;
    using Agent for Agent.State;
    using Agents for Agent.State;

    function combinedData(
        Agent.State storage _agent
    )
        internal view
        returns (Collateral.CombinedData memory)
    {
        Collateral.Data memory poolCollateral = poolCollateralData(_agent);
        return Collateral.CombinedData({
            agentCollateral: agentVaultCollateralData(_agent),
            poolCollateral: poolCollateral,
            agentPoolTokens: agentsPoolTokensCollateralData(_agent, poolCollateral)
        });
    }

    function singleCollateralData(
        Agent.State storage _agent,
        Collateral.Kind _kind
    )
        internal view
        returns (Collateral.Data memory)
    {
        if (_kind == Collateral.Kind.VAULT) {
            return agentVaultCollateralData(_agent);
        } else if (_kind == Collateral.Kind.POOL) {
            return poolCollateralData(_agent);
        } else {
            return agentsPoolTokensCollateralData(_agent, poolCollateralData(_agent));
        }
    }

    function agentVaultCollateralData(
        Agent.State storage _agent
    )
        internal view
        returns (Collateral.Data memory)
    {
        CollateralTypeInt.Data storage collateral = _agent.getVaultCollateral();
        return Collateral.Data({
            kind: Collateral.Kind.VAULT,
            fullCollateral: collateral.token.balanceOf(_agent.vaultAddress()),
            amgToTokenWeiPrice: Conversion.currentAmgPriceInTokenWei(collateral)
        });
    }

    function poolCollateralData(
        Agent.State storage _agent
    )
        internal view
        returns (Collateral.Data memory)
    {
        CollateralTypeInt.Data storage collateral = _agent.getPoolCollateral();
        return Collateral.Data({
            kind: Collateral.Kind.POOL,
            fullCollateral: _agent.collateralPool.totalCollateral(),
            amgToTokenWeiPrice: Conversion.currentAmgPriceInTokenWei(collateral)
        });
    }

    function agentsPoolTokensCollateralData(
        Agent.State storage _agent,
        Collateral.Data memory _poolCollateral
    )
        internal view
        returns (Collateral.Data memory)
    {
        IERC20 poolToken = _agent.collateralPool.poolToken();
        uint256 agentPoolTokens = poolToken.balanceOf(_agent.vaultAddress());
        uint256 totalPoolTokens = poolToken.totalSupply();
        uint256 amgToPoolTokenWeiPrice = _poolCollateral.fullCollateral != 0
            ? _poolCollateral.amgToTokenWeiPrice.mulDiv(totalPoolTokens, _poolCollateral.fullCollateral)
            : _poolCollateral.amgToTokenWeiPrice;   // price for empty pool is 1 token/NAT
        return Collateral.Data({
            kind: Collateral.Kind.AGENT_POOL,
            fullCollateral: agentPoolTokens,
            amgToTokenWeiPrice: amgToPoolTokenWeiPrice
        });
    }

    // The max number of lots the agent can mint
    function freeCollateralLots(
        Collateral.CombinedData memory _data,
        Agent.State storage _agent
    )
        internal view
        returns (uint256 _lots)
    {
        return freeCollateralLotsOptionalFee(_data, _agent, true);
    }

    function freeCollateralLotsOptionalFee(
        Collateral.CombinedData memory _data,
        Agent.State storage _agent,
        bool _chargePoolFee
    )
        internal view
        returns (uint256 _lots)
    {
        uint256 agentLots = freeSingleCollateralLots(_data.agentCollateral, _agent, _chargePoolFee);
        uint256 poolLots = freeSingleCollateralLots(_data.poolCollateral, _agent, _chargePoolFee);
        uint256 agentPoolTokenLots = freeSingleCollateralLots(_data.agentPoolTokens, _agent, _chargePoolFee);
        return Math.min(agentLots, Math.min(poolLots, agentPoolTokenLots));
    }

    function freeSingleCollateralLots(
        Collateral.Data memory _data,
        Agent.State storage _agent,
        bool _chargePoolFee
    )
        internal view
        returns (uint256)
    {
        uint256 collateralWei = freeCollateralWei(_data, _agent);
        uint256 lotWei = mintingLotCollateralWei(_data, _agent, _chargePoolFee);
        // lotWei=0 is possible only for agent's pool token collateral if pool balance in NAT is 0
        // so then we can safely return 0 here, since minting is impossible
        return lotWei != 0 ? collateralWei / lotWei : 0;
    }

    function freeCollateralWei(
        Collateral.Data memory _data,
        Agent.State storage _agent
    )
        internal view
        returns (uint256)
    {
        uint256 lockedCollateral = lockedCollateralWei(_data, _agent);
        return MathUtils.subOrZero(_data.fullCollateral, lockedCollateral);
    }

    // Amount of collateral NOT available for new minting or withdrawal.
    function lockedCollateralWei(
        Collateral.Data memory _data,
        Agent.State storage _agent
    )
        internal view
        returns (uint256)
    {
        (uint256 mintingMinCollateralRatioBIPS, uint256 systemMinCollateralRatioBIPS) =
            mintingMinCollateralRatio(_agent, _data.kind);
        uint256 backedAMG = uint256(_agent.reservedAMG) + uint256(_agent.mintedAMG);
        uint256 mintingCollateral = Conversion.convertAmgToTokenWei(backedAMG, _data.amgToTokenWeiPrice)
            .mulBips(mintingMinCollateralRatioBIPS);
        uint64 redeemingAMG = _data.kind == Collateral.Kind.POOL ? _agent.poolRedeemingAMG : _agent.redeemingAMG;
        uint256 redeemingCollateral = Conversion.convertAmgToTokenWei(redeemingAMG, _data.amgToTokenWeiPrice)
            .mulBips(systemMinCollateralRatioBIPS);
        uint256 announcedWithdrawal =
            _data.kind != Collateral.Kind.POOL ? _agent.withdrawalAnnouncement(_data.kind).amountWei : 0;
        return mintingCollateral + redeemingCollateral + announcedWithdrawal;
    }

    function mintingLotCollateralWei(
        Collateral.Data memory _data,
        Agent.State storage _agent,
        bool _chargePoolFee
    )
        internal view
        returns (uint256)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        return collateralRequiredToMintAmount(_data, _agent, settings.lotSizeAMG, _chargePoolFee);
    }

    function collateralRequiredToMintAmount(
        Collateral.Data memory _data,
        Agent.State storage _agent,
        uint256 _amountAMG,
        bool _chargePoolFee
    )
        internal view
        returns (uint256)
    {
        uint256 amountPoolFeeAMG =
            _chargePoolFee ? _amountAMG.mulBips(_agent.feeBIPS).mulBips(_agent.poolFeeShareBIPS) : 0;
        uint256 totalMintAmountAMG = _amountAMG + amountPoolFeeAMG;
        uint256 totalMintAmountWei = Conversion.convertAmgToTokenWei(totalMintAmountAMG, _data.amgToTokenWeiPrice);
        (uint256 mintingCollateralRatio,) = mintingMinCollateralRatio(_agent, _data.kind);
        return totalMintAmountWei.mulBips(mintingCollateralRatio);
    }

    function mintingMinCollateralRatio(
        Agent.State storage _agent,
        Collateral.Kind _kind
    )
        internal view
        returns (uint256 _mintingMinCollateralRatioBIPS, uint256 _systemMinCollateralRatioBIPS)
    {
        if (_kind == Collateral.Kind.AGENT_POOL) {
            uint256 mintingPoolHoldingsRequiredBIPS = Globals.getSettings().mintingPoolHoldingsRequiredBIPS;
            _systemMinCollateralRatioBIPS = mintingPoolHoldingsRequiredBIPS;
            _mintingMinCollateralRatioBIPS = mintingPoolHoldingsRequiredBIPS;
        } else if (_kind == Collateral.Kind.POOL) {
            _systemMinCollateralRatioBIPS = _agent.getPoolCollateral().minCollateralRatioBIPS;
            _mintingMinCollateralRatioBIPS =
                Math.max(_agent.mintingPoolCollateralRatioBIPS, _systemMinCollateralRatioBIPS);
        } else {
            _systemMinCollateralRatioBIPS = _agent.getVaultCollateral().minCollateralRatioBIPS;
            // agent's minCollateralRatioBIPS must be greater than minCollateralRatioBIPS when set, but
            // minCollateralRatioBIPS can change later so we always use the max of both
            _mintingMinCollateralRatioBIPS =
                Math.max(_agent.mintingVaultCollateralRatioBIPS, _systemMinCollateralRatioBIPS);
        }
    }

    // Used for redemption default payment - calculate all types of collateral at the same rate, so that
    // future redemptions don't get less than this one (unless the price changes).
    // Ignores collateral announced for withdrawal (redemption has priority over withdrawal).
    function maxRedemptionCollateral(
        Collateral.Data memory _data,
        Agent.State storage _agent,
        uint256 _valueAMG
    )
        internal view
        returns (uint256)
    {
        if (_valueAMG == 0) return 0;
        // Assume: collateral kind is VAULT or redemption is NOT a pool self close redemption.
        // For pool self close redemptions, pool collateral is never paid, so this method is not used.
        uint256 redeemingAMG = _data.kind == Collateral.Kind.POOL ? _agent.poolRedeemingAMG : _agent.redeemingAMG;
        assert(_valueAMG <= redeemingAMG);
        uint256 totalAMG = uint256(_agent.mintedAMG) + uint256(_agent.reservedAMG) + uint256(redeemingAMG);
        return _data.fullCollateral.mulDiv(_valueAMG, totalAMG); // totalAMG > 0 (guarded by assert)
    }

    // Agent's collateral ratio for single collateral type (BIPS) - used in liquidation.
    // Ignores collateral announced for withdrawal (withdrawals are forbidden during liquidation).
    function collateralRatioBIPS(
        Collateral.Data memory _data,
        Agent.State storage _agent
    )
        internal view
        returns (uint256)
    {
        uint256 totalAMG = totalBackedAMG(_agent, _data.kind);
        uint256 backingTokenWei = Conversion.convertAmgToTokenWei(totalAMG, _data.amgToTokenWeiPrice);
        if (backingTokenWei == 0) return 1e10;    // nothing minted - ~infinite collateral ratio (but avoid overflows)
        return _data.fullCollateral.mulDiv(SafePct.MAX_BIPS, backingTokenWei);
    }

    function totalBackedAMG(
        Agent.State storage _agent,
        Collateral.Kind _collateralKind
    )
        internal view
        returns (uint256)
    {
        uint256 redeemingAMG = _collateralKind == Collateral.Kind.POOL ? _agent.poolRedeemingAMG : _agent.redeemingAMG;
        return uint256(_agent.mintedAMG) + uint256(_agent.reservedAMG) + uint256(redeemingAMG);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Math} from "@openzeppelin/contracts/utils/math/Math.sol";
import {IIAgentVault} from "../../agentVault/interfaces/IIAgentVault.sol";
import {Globals} from "./Globals.sol";
import {Agent} from "./data/Agent.sol";
import {CollateralTypeInt} from "./data/CollateralTypeInt.sol";
import {AssetManagerSettings} from "../../userInterfaces/data/AssetManagerSettings.sol";
import {Agents} from "./Agents.sol";


library AgentPayout {
    using Agent for Agent.State;

    function payoutFromVault(
        Agent.State storage _agent,
        address _receiver,
        uint256 _amountWei
    )
        internal
        returns (uint256 _amountPaid)
    {
        CollateralTypeInt.Data storage collateral = Agents.getVaultCollateral(_agent);
        // don't want the calling method to fail due to too small balance for payout
        IIAgentVault vault = IIAgentVault(_agent.vaultAddress());
        _amountPaid = Math.min(_amountWei, collateral.token.balanceOf(address(vault)));
        vault.payout(collateral.token, _receiver, _amountPaid);
    }

    function tryPayoutFromVault(
        Agent.State storage _agent,
        address _receiver,
        uint256 _amountWei
    )
        internal
        returns (bool _success, uint256 _amountPaid)
    {
        CollateralTypeInt.Data storage collateral = Agents.getVaultCollateral(_agent);
        // don't want the calling method to fail due to too small balance for payout
        IIAgentVault vault = IIAgentVault(_agent.vaultAddress());
        _amountPaid = Math.min(_amountWei, collateral.token.balanceOf(address(vault)));
        try vault.payout(collateral.token, _receiver, _amountPaid) {
            _success = true;
        } catch {
            _success = false;
            _amountPaid = 0;
        }
    }

    function payoutFromPool(
        Agent.State storage _agent,
        address _receiver,
        uint256 _amountWei,
        uint256 _agentResponsibilityWei
    )
        internal
        returns (uint256 _amountPaid)
    {
        // don't want the calling method to fail due to too small balance for payout
        uint256 poolBalance = _agent.collateralPool.totalCollateral();
        _amountPaid = Math.min(_amountWei, poolBalance);
        _agentResponsibilityWei = Math.min(_agentResponsibilityWei, _amountPaid);
        _agent.collateralPool.payout(_receiver, _amountPaid, _agentResponsibilityWei);
    }

    function payForConfirmationByOthers(
        Agent.State storage _agent,
        address _receiver
    )
        internal
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        uint256 amount = Agents.convertUSD5ToVaultCollateralWei(_agent, settings.confirmationByOthersRewardUSD5);
        payoutFromVault(_agent, _receiver, amount);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import "./IReferencedPaymentNonexistence.sol";

interface IReferencedPaymentNonexistenceVerification {
    function verifyReferencedPaymentNonexistence(
        IReferencedPaymentNonexistence.Proof calldata _proof
    ) external view returns (bool _proved);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/******************************************************************************\
* Author: Nick Mudge <nick@perfectabstractions.com> (https://twitter.com/mudgen)
* EIP-2535 Diamonds: https://eips.ethereum.org/EIPS/eip-2535
/******************************************************************************/

import { IDiamond } from "./IDiamond.sol";

interface IDiamondCut is IDiamond {

    /// @notice Add/replace/remove any number of functions and optionally execute
    ///         a function with delegatecall
    /// @param _diamondCut Contains the facet addresses and function selectors
    /// @param _init The address of the contract or facet to execute _calldata
    /// @param _calldata A function call, including function selector and arguments
    ///                  _calldata is executed with delegatecall on _init
    function diamondCut(
        FacetCut[] calldata _diamondCut,
        address _init,
        bytes calldata _calldata
    ) external;
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import "./IConfirmedBlockHeightExists.sol";

interface IConfirmedBlockHeightExistsVerification {
    function verifyConfirmedBlockHeightExists(
        IConfirmedBlockHeightExists.Proof calldata _proof
    ) external view returns (bool _proved);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {IPriceReader} from "../../ftso/interfaces/IPriceReader.sol";
import {SafePct} from "../../utils/library/SafePct.sol";
import {AssetManagerState} from "./data/AssetManagerState.sol";
import {Globals} from "./Globals.sol";
import {CollateralTypeInt} from "./data/CollateralTypeInt.sol";
import {AssetManagerSettings} from "../../userInterfaces/data/AssetManagerSettings.sol";


library Conversion {
    using SafePct for uint256;

    uint256 internal constant AMG_TOKEN_WEI_PRICE_SCALE_EXP = 9;
    uint256 internal constant AMG_TOKEN_WEI_PRICE_SCALE = 10 ** AMG_TOKEN_WEI_PRICE_SCALE_EXP;
    uint256 internal constant NAT_WEI = 1e18;
    uint256 internal constant GWEI = 1e9;

    function currentAmgPriceInTokenWei(
        uint256 _tokenType
    )
        internal view
        returns (uint256 _price)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        (_price,,) = currentAmgPriceInTokenWeiWithTs(state.collateralTokens[_tokenType], false);
    }

    function currentAmgPriceInTokenWei(
        CollateralTypeInt.Data storage _token
    )
        internal view
        returns (uint256 _price)
    {
        (_price,,) = currentAmgPriceInTokenWeiWithTs(_token, false);
    }

    function currentAmgPriceInTokenWeiWithTrusted(
        CollateralTypeInt.Data storage _token
    )
        internal view
        returns (uint256 _ftsoPrice, uint256 _trustedPrice)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        (uint256 ftsoPrice, uint256 assetTimestamp, uint256 tokenTimestamp) =
            currentAmgPriceInTokenWeiWithTs(_token, false);
        (uint256 trustedPrice, uint256 assetTimestampTrusted, uint256 tokenTimestampTrusted) =
            currentAmgPriceInTokenWeiWithTs(_token, true);
        bool trustedPriceFresh = tokenTimestampTrusted + settings.maxTrustedPriceAgeSeconds >= tokenTimestamp
                && assetTimestampTrusted + settings.maxTrustedPriceAgeSeconds >= assetTimestamp;
        _ftsoPrice = ftsoPrice;
        _trustedPrice = trustedPriceFresh ? trustedPrice : ftsoPrice;
    }

    function convertAmgToUBA(
        uint64 _valueAMG
    )
        internal view
        returns (uint256)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // safe multiplication - both values are 64 bit
        return uint256(_valueAMG) * settings.assetMintingGranularityUBA;
    }

    function convertUBAToAmg(
        uint256 _valueUBA
    )
        internal view
        returns (uint64)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        return SafeCast.toUint64(_valueUBA / settings.assetMintingGranularityUBA);
    }

    function roundUBAToAmg(
        uint256 _valueUBA
    )
        internal view
        returns (uint256)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        return _valueUBA - (_valueUBA % settings.assetMintingGranularityUBA);
    }

    function convertLotsToAMG(
        uint256 _lots
    )
        internal view
        returns (uint64)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        return SafeCast.toUint64(_lots * settings.lotSizeAMG);
    }

    function convertLotsToUBA(
        uint256 _lots
    )
        internal view
        returns (uint256)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // this should not overflow - all values are 64 bit (except _lots which is limited by minted lots)
        return _lots * settings.lotSizeAMG * settings.assetMintingGranularityUBA;
    }

    function convert(
        uint256 _amount,
        CollateralTypeInt.Data storage _fromToken,
        CollateralTypeInt.Data storage _toToken
    )
        internal view
        returns (uint256)
    {
        uint256 priceMul = currentAmgPriceInTokenWei(_toToken);
        uint256 priceDiv = currentAmgPriceInTokenWei(_fromToken);
        return _amount.mulDiv(priceMul, priceDiv);
    }

    function convertFromUSD5(
        uint256 _amountUSD5,
        CollateralTypeInt.Data storage _token
    )
        internal view
        returns (uint256)
    {
        // if tokenFtsoSymbol is empty, it is assumed that the token is a USD-like stablecoin
        // so `_amountUSD5` is (approximately) the correct amount of tokens
        if (bytes(_token.tokenFtsoSymbol).length == 0) {
            return _amountUSD5;
        }
        (uint256 tokenPrice,, uint256 tokenFtsoDec) = readFtsoPrice(_token.tokenFtsoSymbol, false);
        // 5 is for 5 decimals of USD5
        uint256 expPlus = _token.decimals + tokenFtsoDec - 5;
        return _amountUSD5.mulDiv(10 ** expPlus, tokenPrice);
    }

    function currentAmgPriceInTokenWeiWithTs(
        CollateralTypeInt.Data storage _token,
        bool _fromTrustedProviders
    )
        internal view
        returns (uint256 /*_price*/, uint256 /*_assetTimestamp*/, uint256 /*_tokenTimestamp*/)
    {
        (uint256 assetPrice, uint256 assetTs, uint256 assetFtsoDec) =
            readFtsoPrice(_token.assetFtsoSymbol, _fromTrustedProviders);
        if (_token.directPricePair) {
            uint256 price = calcAmgToTokenWeiPrice(_token.decimals, 1, 0, assetPrice, assetFtsoDec);
            return (price, assetTs, assetTs);
        } else {
            (uint256 tokenPrice, uint256 tokenTs, uint256 tokenFtsoDec) =
                readFtsoPrice(_token.tokenFtsoSymbol, _fromTrustedProviders);
            uint256 price =
                calcAmgToTokenWeiPrice(_token.decimals, tokenPrice, tokenFtsoDec, assetPrice, assetFtsoDec);
            return (price, assetTs, tokenTs);
        }
    }

    function readFtsoPrice(string memory _symbol, bool _fromTrustedProviders)
        internal view
        returns (uint256, uint256, uint256)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        IPriceReader priceReader = IPriceReader(settings.priceReader);
        if (_fromTrustedProviders) {
            return priceReader.getPriceFromTrustedProviders(_symbol);
        } else {
            return priceReader.getPrice(_symbol);
        }
    }

    function calcAmgToTokenWeiPrice(
        uint256 _tokenDecimals,
        uint256 _tokenPrice,
        uint256 _tokenFtsoDecimals,
        uint256 _assetPrice,
        uint256 _assetFtsoDecimals
    )
        internal view
        returns (uint256)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        uint256 expPlus = _tokenDecimals + _tokenFtsoDecimals + AMG_TOKEN_WEI_PRICE_SCALE_EXP;
        uint256 expMinus = settings.assetMintingDecimals + _assetFtsoDecimals;
        // If negative, price would probably always be 0 after division, so this is forbidden.
        // Anyway, we should know about this before we add the token and/or asset, since
        // token decimals and ftso decimals typically never change.
        assert(expPlus >= expMinus);
        return _assetPrice.mulDiv(10 ** (expPlus - expMinus), _tokenPrice);
    }

    function convertAmgToTokenWei(uint256 _valueAMG, uint256 _amgToTokenWeiPrice) internal pure returns (uint256) {
        return _valueAMG.mulDiv(_amgToTokenWeiPrice, AMG_TOKEN_WEI_PRICE_SCALE);
    }

    function convertTokenWeiToAMG(uint256 _valueNATWei, uint256 _amgToTokenWeiPrice) internal pure returns (uint256) {
        return _valueNATWei.mulDiv(AMG_TOKEN_WEI_PRICE_SCALE, _amgToTokenWeiPrice);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {CheckPointHistory} from "./CheckPointHistory.sol";


/**
 * @title Check Points By Address library
 * @notice A contract to manage checkpoint history for a collection of addresses.
 * @dev Store value history by address, and then by block number.
 **/
library CheckPointsByAddress {
    using CheckPointHistory for CheckPointHistory.CheckPointHistoryState;

    struct CheckPointsByAddressState {
        // `historyByAddress` is the map that stores the check point history of each address
        mapping(address => CheckPointHistory.CheckPointHistoryState) historyByAddress;
    }

    /**
    /**
     * @notice Send `amount` value to `to` address from `from` address.
     * @param _self A CheckPointsByAddressState instance to manage.
     * @param _from Address of the history of from values
     * @param _to Address of the history of to values
     * @param _amount The amount of value to be transferred
     **/
    function transmit(
        CheckPointsByAddressState storage _self,
        address _from,
        address _to,
        uint256 _amount
    )
        internal
    {
        // Shortcut
        if (_amount == 0) return;

        // Both from and to can never be zero
        assert(!(_from == address(0) && _to == address(0)));

        // Update transferer value
        if (_from != address(0)) {
            // Compute the new from balance
            uint256 newValueFrom = valueOfAtNow(_self, _from) - _amount;
            writeValue(_self, _from, newValueFrom);
        }

        // Update transferee value
        if (_to != address(0)) {
            // Compute the new to balance
            uint256 newValueTo = valueOfAtNow(_self, _to) + _amount;
            writeValue(_self, _to, newValueTo);
        }
    }

    /**
     * @notice Queries the value of `_owner` at a specific `_blockNumber`.
     * @param _self A CheckPointsByAddressState instance to manage.
     * @param _owner The address from which the value will be retrieved.
     * @param _blockNumber The block number to query for the then current value.
     * @return The value at `_blockNumber` for `_owner`.
     **/
    function valueOfAt(
        CheckPointsByAddressState storage _self,
        address _owner,
        uint256 _blockNumber
    )
        internal view
        returns (uint256)
    {
        // Get history for _owner
        CheckPointHistory.CheckPointHistoryState storage history = _self.historyByAddress[_owner];
        // Return value at given block
        return history.valueAt(_blockNumber);
    }

    /**
     * @notice Get the value of the `_owner` at the current `block.number`.
     * @param _self A CheckPointsByAddressState instance to manage.
     * @param _owner The address of the value is being requested.
     * @return The value of `_owner` at the current block.
     **/
    function valueOfAtNow(CheckPointsByAddressState storage _self, address _owner) internal view returns (uint256) {
        // Get history for _owner
        CheckPointHistory.CheckPointHistoryState storage history = _self.historyByAddress[_owner];
        // Return value at now
        return history.valueAtNow();
    }

    /**
     * @notice Writes the `value` at the current block number for `_owner`.
     * @param _self A CheckPointsByAddressState instance to manage.
     * @param _owner The address of `_owner` to write.
     * @param _value The value to write.
     * @dev Sender must be the owner of the contract.
     **/
    function writeValue(
        CheckPointsByAddressState storage _self,
        address _owner,
        uint256 _value
    )
        internal
    {
        // Get history for _owner
        CheckPointHistory.CheckPointHistoryState storage history = _self.historyByAddress[_owner];
        // Write the value
        history.writeValue(_value);
    }

    /**
     * Delete at most `_count` of the oldest checkpoints.
     * At least one checkpoint at or before `_cleanupBlockNumber` will remain
     * (unless the history was empty to start with).
     */
    function cleanupOldCheckpoints(
        CheckPointsByAddressState storage _self,
        address _owner,
        uint256 _count,
        uint256 _cleanupBlockNumber
    )
        internal
        returns (uint256)
    {
        if (_owner != address(0)) {
            return _self.historyByAddress[_owner].cleanupOldCheckpoints(_count, _cleanupBlockNumber);
        }
        return 0;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {EnumerableSet} from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import {IICollateralPool} from "../../../collateralPool/interfaces/IICollateralPool.sol";


library Agent {
    error InvalidAgentVaultAddress();

    enum Status {
        EMPTY,              // agent does not exist
        NORMAL,
        LIQUIDATION,        // liquidation due to CR - ends when agent is healthy
        FULL_LIQUIDATION,   // illegal payment liquidation - must liquidate all and close vault
        DESTROYING,         // agent announced destroy, cannot mint again
        DESTROYED           // agent has been destroyed, cannot do anything except return info
    }

    // For agents to withdraw NAT collateral, they must first announce it and then wait
    // withdrawalAnnouncementSeconds.
    // The announced amount cannot be used as collateral for minting during that time.
    // This makes sure that agents cannot just remove all collateral if they are challenged.
    struct WithdrawalAnnouncement {
        // Announce amount in collateral token's minimum unit (wei).
        uint128 amountWei;

        // The timestamp when withdrawal can be executed.
        uint64 allowedAt;
    }

    // Struct to store agent's pending setting updates.
    struct SettingUpdate {
        uint128 value;
        uint64 validAt;
    }

    struct State {
        IICollateralPool collateralPool;

        // Address of the agent owner. This is the management address, which is immutable.
        // The work address can be retrieved from the global state mapping between
        // management and work addresses.
        address ownerManagementAddress;

        // Current underlying address for this agent vault.
        // The address is immutable.
        string underlyingAddressString;

        // `underlyingAddressString` is only used for sending the minter a correct payment address;
        // for matching payment addresses we always use `underlyingAddressHash = keccak256(underlyingAddressString)`
        bytes32 underlyingAddressHash;

        // Current status of the agent (changes for liquidation).
        Agent.Status status;

        // Index of collateral vault token.
        // The data is obtained as state.collateralTokens[vaultCollateralIndex].
        uint16 vaultCollateralIndex;

        // Index of token in collateral pool. This is always wrapped FLR/SGB, however the wrapping
        // contract (WNat) may change. In such case we add new collateral token with class POOL but the
        // agent must call a method to upgrade to new contract, se we must track the actual token used.
        uint16 poolCollateralIndex;

        // Position of this agent in the list of agents available for minting.
        // Value is actually `list index + 1`, so that 0 means 'not in the list'.
        uint32 availableAgentsPos;

        // Minting fee in BIPS (collected in underlying currency).
        uint16 feeBIPS;

        // Share of the minting fee that goes to the pool as percentage of the minting fee.
        uint16 poolFeeShareBIPS;

        // Collateral ratio at which we calculate locked collateral and collateral available for minting.
        // Agent may set own value for minting collateral ratio when entering the available agent list,
        // but it must always be greater than minimum collateral ratio.
        uint32 mintingVaultCollateralRatioBIPS;

        // Collateral ratio at which we calculate locked collateral and collateral available for minting.
        // Agent may set own value for minting collateral ratio when entering the available agent list,
        // but it must always be greater than minimum collateral ratio.
        uint32 mintingPoolCollateralRatioBIPS;

        // Timestamp of the startLiquidation (or liquidate) call.
        uint64 liquidationStartedAt;

        // Liquidation phase at the time when liquidation started.
        uint8 __initialLiquidationPhase; // only storage placeholder

        // Bitmap signifying which collateral type(s) triggered liquidation (LF_VAULT | LF_POOL).
        uint8 collateralsUnderwater;

        // Amount of collateral locked by collateral reservation.
        uint64 reservedAMG;

        // Amount of collateral backing minted fassets.
        uint64 mintedAMG;

        // The amount of fassets being redeemed. In this case, the fassets were already burned,
        // but the collateral must still be locked to allow payment in case of redemption failure.
        // The distinction between 'minted' and 'redeemed' assets is important in case of challenge.
        uint64 redeemingAMG;

        // The amount of fassets being redeemed EXCEPT those from pool self-close exits.
        // Unlike normal redemption, pool collateral was already withdrawn, so the redeeming collateral
        // must only be accounted for / locked for vault collateral.
        // On redemption payment failure, redeemer will be paid only in vault collateral in this case
        // (and will be paid less if there isn't enough - small extra risk for pool token holders).
        // There will always be `poolRedeemingAMG <= redeemingAMG`.
        uint64 poolRedeemingAMG;

        // When lot size changes, there may be some leftover after redemption that doesn't fit
        // a whole lot size. It is added to dustAMG and can be recovered via self-close.
        // Unlike redeemingAMG, dustAMG is still counted in the mintedAMG.
        uint64 dustAMG;

        // The amount of funds that on the agent's underlying address.
        // If it is higher than the amount needed to back mintings, it can be withdrawn after announcement.
        // It is signed int, because unreported deposits combined with other operations can in principle
        // make it negative. We could truncate it at 0, but if deposit report comes later, this would make
        // the value wrong.
        int128 underlyingBalanceUBA;

        // There can be only one announced underlying withdrawal per agent active at any time.
        // This variable holds the id, or 0 if there is no announced underlying withdrawal going on.
        uint64 announcedUnderlyingWithdrawalId;

        // The time when ongoing underlying withdrawal was announced.
        uint64 underlyingWithdrawalAnnouncedAt;

        // Announcement for vault collateral withdrawal.
        WithdrawalAnnouncement vaultCollateralWithdrawalAnnouncement;

        // Announcement for pool token withdrawal (which also means pool collateral withdrawal).
        WithdrawalAnnouncement poolTokenWithdrawalAnnouncement;

        // Underlying block when the agent was created.
        // Challenger's should track underlying address activity since this block
        // and topups are only valid after this block (both inclusive).
        uint64 underlyingBlockAtCreation;

        // The time when ongoing agent vault destroy was announced.
        uint64 destroyAllowedAt;

        // The factor set by the agent to multiply the price at which agent buys f-assets from pool
        // token holders on self-close exit (when requested or the redeemed amount is less than 1 lot).
        uint16 buyFAssetByAgentFactorBIPS;

        // The announced time when the agent is exiting available agents list.
        uint64 exitAvailableAfterTs;

        // The position of the agent in the list of all agents.
        uint32 allAgentsPos;

        // Agent's pending setting updates.
        mapping(bytes32 => SettingUpdate) settingUpdates;

        // Agent's handshake type - minting or redeeming can be rejected.
        // 0 - no verification, 1 - manual verification, ...
        uint32 __handshakeType; // only storage placeholder

        // There can only be one transfer to core vault per agent active at any time.
        uint64 activeTransferToCoreVault;

        // the request id of the active return from core vault
        uint64 activeReturnFromCoreVaultId;

        // part of the agent's reservedAMG for the core vault return
        uint64 returnFromCoreVaultReservedAMG;

        // The redemption fee share paid to the pool (as FAssets).
        // In redemption dominated situations (when agent requests return from core vault to earn
        // from redemption fees), pool can get some share to make it sustainable for pool users.
        // NOTE: the pool fee share is locked at the redemption request time, but is charged at the redemption
        // confirmation time. If agent uses all the redemption fee for transaction fees, this could make the
        // agent's free underlying balance negative.
        uint16 redemptionPoolFeeShareBIPS;

        EnumerableSet.AddressSet alwaysAllowedMinters;

        // Only used for calculating Agent.State size. See deleteStorage() below.
        uint256[1] _endMarker;
    }

    // underwater collateral classes
    uint8 internal constant LF_VAULT = 1 << 0;
    uint8 internal constant LF_POOL = 1 << 1;

    // diamond state accessors

    bytes32 internal constant AGENTS_POSITION = keccak256("fasset.AssetManager.Agent");

    // only return valid agent - fail if status is EMPTY or DESTROYED
    function get(address _address)
        internal view
        returns (Agent.State storage)
    {
        Agent.State storage agent = getWithoutCheck(_address);
        Agent.Status status = agent.status;
        require(status != Agent.Status.EMPTY && status != Agent.Status.DESTROYED, InvalidAgentVaultAddress());
        return agent;
    }

    // Like get, but only fail if status is EMPTY.
    // This is useful for reading agent info after the agent has been destroyed.
    function getAllowDestroyed(address _address)
        internal view
        returns (Agent.State storage)
    {
        Agent.State storage agent = getWithoutCheck(_address);
        require(agent.status != Agent.Status.EMPTY, InvalidAgentVaultAddress());
        return agent;
    }

    function getWithoutCheck(address _address)
        internal pure
        returns (Agent.State storage _agent)
    {
        bytes32 position = bytes32(uint256(AGENTS_POSITION) ^ (uint256(uint160(_address)) << 64));
        // solhint-disable-next-line no-inline-assembly
        assembly {
            _agent.slot := position
        }
    }

    function vaultAddress(Agent.State storage _agent)
        internal pure
        returns (address)
    {
        bytes32 position;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            position := _agent.slot
        }
        return address(uint160((uint256(position) ^ uint256(AGENTS_POSITION)) >> 64));
    }
}
// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IERC20Metadata} from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";


interface IFAsset is IERC20, IERC20Metadata {
    ////////////////////////////////////////////////////////////////////////////////////
    // System information

    /**
     * The name of the underlying asset.
     */
    function assetName() external view returns (string memory);

    /**
     * The symbol of the underlying asset.
     */
    function assetSymbol() external view returns (string memory);

    /**
     * Get the asset manager, corresponding to this fAsset.
     * fAssets and asset managers are in 1:1 correspondence.
     */
    function assetManager() external view returns (address);
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;


/**
 * All asset manager events.
 */
interface IAssetManagerEvents {
    struct AgentVaultCreationData {
        address collateralPool;
        address collateralPoolToken;
        string underlyingAddress;
        address vaultCollateralToken;
        address poolWNatToken;
        uint256 feeBIPS;
        uint256 poolFeeShareBIPS;
        uint256 mintingVaultCollateralRatioBIPS;
        uint256 mintingPoolCollateralRatioBIPS;
        uint256 buyFAssetByAgentFactorBIPS;
        uint256 poolExitCollateralRatioBIPS;
        uint256 redemptionPoolFeeShareBIPS;
    }

    /**
     * A new agent vault was created.
     */
    event AgentVaultCreated(
        address indexed owner,
        address indexed agentVault,
        AgentVaultCreationData creationData);

    /**
     * Agent has announced destroy (close) of agent vault and will be able to
     * perform destroy after the timestamp `destroyAllowedAt`.
     */
    event AgentDestroyAnnounced(
        address indexed agentVault,
        uint256 destroyAllowedAt);

    /**
     * Agent has destroyed (closed) the agent vault.
     */
    event AgentDestroyed(
        address indexed agentVault);

    /**
     * Agent has announced a withdrawal of collateral and will be able to
     * withdraw the announced amount after timestamp `withdrawalAllowedAt`.
     * If withdrawal was canceled (announced with amount 0), amountWei and withdrawalAllowedAt are zero.
     */
    event VaultCollateralWithdrawalAnnounced(
        address indexed agentVault,
        uint256 amountWei,
        uint256 withdrawalAllowedAt);

    /**
     * Agent has announced a withdrawal of collateral and will be able to
     * redeem the announced amount of pool tokens after the timestamp `withdrawalAllowedAt`.
     * If withdrawal was canceled (announced with amount 0), amountWei and withdrawalAllowedAt are zero.
     */
    event PoolTokenRedemptionAnnounced(
        address indexed agentVault,
        uint256 amountWei,
        uint256 withdrawalAllowedAt);

    /**
     * Agent was added to the list of available agents and can accept collateral reservation requests.
     */
    event AgentAvailable(
        address indexed agentVault,
        uint256 feeBIPS,
        uint256 mintingVaultCollateralRatioBIPS,
        uint256 mintingPoolCollateralRatioBIPS,
        uint256 freeCollateralLots);

    /**
     * Agent exited from available agents list.
     * The agent can exit the available list after the timestamp `exitAllowedAt`.
     */
    event AvailableAgentExitAnnounced(
        address indexed agentVault,
        uint256 exitAllowedAt);

    /**
     * Agent exited from available agents list.
     */
    event AvailableAgentExited(
        address indexed agentVault);

    /**
     * Agent has initiated setting change (fee or some agent collateral ratio change).
     * The setting change can be executed after the timestamp `validAt`.
     */
    event AgentSettingChangeAnnounced(
        address indexed agentVault,
        string name,
        uint256 value,
        uint256 validAt);

    /**
     * Agent has executed setting change (fee or some agent collateral ratio change).
     */
    event AgentSettingChanged(
        address indexed agentVault,
        string name,
        uint256 value);

    /**
     * Agent or agent's collateral pool has changed token contract.
     */
    event AgentCollateralTypeChanged(
        address indexed agentVault,
        uint8 collateralClass,
        address token);

    /**
     * Minter reserved collateral, paid the reservation fee, and is expected to pay the underlying funds.
     * Agent's collateral was reserved.
     */
    event CollateralReserved(
        address indexed agentVault,
        address indexed minter,
        uint256 indexed collateralReservationId,
        uint256 valueUBA,
        uint256 feeUBA,
        uint256 firstUnderlyingBlock,
        uint256 lastUnderlyingBlock,
        uint256 lastUnderlyingTimestamp,
        string paymentAddress,
        bytes32 paymentReference,
        address executor,
        uint256 executorFeeNatWei);

    /**
     * Minter paid underlying funds in time and received the fassets.
     * The agent's collateral is locked.
     */
    event MintingExecuted(
        address indexed agentVault,
        uint256 indexed collateralReservationId,
        uint256 mintedAmountUBA,
        uint256 agentFeeUBA,
        uint256 poolFeeUBA);

    /**
     * Minter failed to pay underlying funds in time. Collateral reservation fee was paid to the agent.
     * Reserved collateral was released.
     */
    event MintingPaymentDefault(
        address indexed agentVault,
        address indexed minter,
        uint256 indexed collateralReservationId,
        uint256 reservedAmountUBA);

    /**
     * Both minter and agent failed to present any proof within attestation time window, so
     * the agent called `unstickMinting` to release reserved collateral.
     */
    event CollateralReservationDeleted(
        address indexed agentVault,
        address indexed minter,
        uint256 indexed collateralReservationId,
        uint256 reservedAmountUBA);

    /**
     * Agent performed self minting, either by executing selfMint with underlying deposit or
     * by executing mintFromFreeUnderlying (in this case, `mintFromFreeUnderlying` is true and
     * `depositedAmountUBA` is zero).
     */
    event SelfMint(
        address indexed agentVault,
        bool mintFromFreeUnderlying,
        uint256 mintedAmountUBA,
        uint256 depositedAmountUBA,
        uint256 poolFeeUBA);

    /**
     * Redeemer started the redemption process and provided fassets.
     * The amount of fassets corresponding to valueUBA was burned.
     * Several RedemptionRequested events are emitted, one for every agent redeemed against
     * (but multiple tickets for the same agent are combined).
     * The agent's collateral is still locked.
     */
    event RedemptionRequested(
        address indexed agentVault,
        address indexed redeemer,
        uint256 indexed requestId,
        string paymentAddress,
        uint256 valueUBA,
        uint256 feeUBA,
        uint256 firstUnderlyingBlock,
        uint256 lastUnderlyingBlock,
        uint256 lastUnderlyingTimestamp,
        bytes32 paymentReference,
        address executor,
        uint256 executorFeeNatWei);

    /**
     * Agent rejected the redemption payment because the redeemer's address is invalid.
     */
    event RedemptionRejected(
        address indexed agentVault,
        address indexed redeemer,
        uint256 indexed requestId,
        uint256 redemptionAmountUBA);

    /**
     * In case there were not enough tickets or more than allowed number would have to be redeemed,
     * only partial redemption is done and the `remainingLots` lots of the fassets are returned to
     * the redeemer.
     */
    event RedemptionRequestIncomplete(
        address indexed redeemer,
        uint256 remainingLots);

    /**
     * Agent provided proof of redemption payment.
     * Agent's collateral is released.
     */
    event RedemptionPerformed(
        address indexed agentVault,
        address indexed redeemer,
        uint256 indexed requestId,
        bytes32 transactionHash,
        uint256 redemptionAmountUBA,
        int256 spentUnderlyingUBA);

    /**
     * The time for redemption payment is over and payment proof was not provided.
     * Redeemer was paid in the collateral (with extra).
     * The rest of the agent's collateral is released.
     * The corresponding amount of underlying currency, held by the agent, is released
     * and the agent can withdraw it (after underlying withdrawal announcement).
     */
    event RedemptionDefault(
        address indexed agentVault,
        address indexed redeemer,
        uint256 indexed requestId,
        uint256 redemptionAmountUBA,
        uint256 redeemedVaultCollateralWei,
        uint256 redeemedPoolCollateralWei);

    /**
     * Agent provided the proof that redemption payment was attempted, but failed due to
     * the redeemer's address being blocked (or burning more than allowed amount of gas).
     * Redeemer is not paid and all of the agent's collateral is released.
     * The underlying currency is also released to the agent.
     */
    event RedemptionPaymentBlocked(
        address indexed agentVault,
        address indexed redeemer,
        uint256 indexed requestId,
        bytes32 transactionHash,
        uint256 redemptionAmountUBA,
        int256 spentUnderlyingUBA);

    /**
     * Agent provided the proof that redemption payment was attempted, but failed due to
     * his own error. Also triggers payment default, unless the redeemer has done it already.
     */
    event RedemptionPaymentFailed(
        address indexed agentVault,
        address indexed redeemer,
        uint256 indexed requestId,
        bytes32 transactionHash,
        int256 spentUnderlyingUBA,
        string failureReason);

    /**
     * At the end of a successful redemption, part of the redemption fee is re-minted as FAssets
     * and paid to the agent's collateral pool as fee.
     */
    event RedemptionPoolFeeMinted(
        address indexed agentVault,
        uint256 indexed requestId,
        uint256 poolFeeUBA);

    /**
     * Due to self-close exit, some of the agent's backed fAssets were redeemed,
     * but the redemption was immediately paid in collateral so no redemption process is started.
     */
    event RedeemedInCollateral(
        address indexed agentVault,
        address indexed redeemer,
        uint256 redemptionAmountUBA,
        uint256 paidVaultCollateralWei);

    /**
     * Agent self-closed valueUBA of backing fassets.
     */
    event SelfClose(
        address indexed agentVault,
        uint256 valueUBA);

    /**
     * Redemption ticket with given value was created (when minting was executed).
     */
    event RedemptionTicketCreated(
        address indexed agentVault,
        uint256 indexed redemptionTicketId,
        uint256 ticketValueUBA);

    /**
     * Redemption ticket value was changed (partially redeemed).
     * @param ticketValueUBA the ticket value after update
     */
    event RedemptionTicketUpdated(
        address indexed agentVault,
        uint256 indexed redemptionTicketId,
        uint256 ticketValueUBA);

    /**
     * Redemption ticket was deleted.
     */
    event RedemptionTicketDeleted(
        address indexed agentVault,
        uint256 indexed redemptionTicketId);

    /**
     * Due to lot size change, some dust was created for this agent during
     * redemption. Value `dustUBA` is the new amount of dust. Dust cannot be directly redeemed,
     * but it can be self-closed or liquidated and if it accumulates to more than 1 lot,
     * it can be converted to a new redemption ticket.
     */
    event DustChanged(
        address indexed agentVault,
        uint256 dustUBA);

    /**
     * Agent entered liquidation state due to unhealthy position.
     * The liquidation ends when the agent is again healthy or the agent's position is fully liquidated.
     */
    event LiquidationStarted(
        address indexed agentVault,
        uint256 timestamp);

    /**
     * Agent entered liquidation state due to illegal payment.
     * Full liquidation will always liquidate the whole agent's position and
     * the agent can never use the same vault and underlying address for minting again.
     */
    event FullLiquidationStarted(
        address indexed agentVault,
        uint256 timestamp);

    /**
     * Some of the agent's position was liquidated, by burning liquidator's fassets.
     * Liquidator was paid in collateral with extra.
     * The corresponding amount of underlying currency, held by the agent, is released
     * and the agent can withdraw it (after underlying withdrawal announcement).
     */
    event LiquidationPerformed(
        address indexed agentVault,
        address indexed liquidator,
        uint256 valueUBA,
        uint256 paidVaultCollateralWei,
        uint256 paidPoolCollateralWei);

    /**
     * Agent exited liquidation state as agent's position was healthy again and not in full liquidation.
     */
    event LiquidationEnded(
        address indexed agentVault);

    /**
     * Part of the balance in the agent's underlying address is "free balance" that the agent can withdraw.
     * It is obtained from minting / redemption fees and self-closed fassets.
     * Some of this amount should be left for paying redemption (and withdrawal) gas fees,
     * and the rest can be withdrawn by the agent.
     * However, withdrawal has to be announced, otherwise it can be challenged as illegal payment.
     * Only one announcement can exist per agent - agent has to present payment proof for withdrawal
     * before starting a new one.
     */
    event UnderlyingWithdrawalAnnounced(
        address indexed agentVault,
        uint256 indexed announcementId,
        bytes32 paymentReference);

    /**
     * After announcing legal underlying withdrawal and creating transaction,
     * the agent must confirm the transaction. This frees the announcement so the agent can create another one.
     * If the agent doesn't confirm in time, anybody can confirm the transaction after several hours.
     * Failed payments must also be confirmed.
     */
    event UnderlyingWithdrawalConfirmed(
        address indexed agentVault,
        uint256 indexed announcementId,
        int256 spentUBA,
        bytes32 transactionHash);

    /**
     * After announcing legal underlying withdrawal agent can cancel ongoing withdrawal.
     * The reason for doing that would be in resetting announcement timestamp due to any problems with underlying
     * withdrawal - in order to prevent others to confirm withdrawal before agent and get some of his collateral.
     */
    event UnderlyingWithdrawalCancelled(
        address indexed agentVault,
        uint256 indexed announcementId);

    /**
     * Emitted when the agent tops up the underlying address balance.
     */
    event UnderlyingBalanceToppedUp(
        address indexed agentVault,
        bytes32 transactionHash,
        uint256 depositedUBA);

    /**
     * Emitted whenever the tracked underlying balance changes.
     */
    event UnderlyingBalanceChanged(
        address indexed agentVault,
        int256 underlyingBalanceUBA);

    /**
     * An unexpected transaction from the agent's underlying address was proved.
     * Whole agent's position goes into liquidation.
     * The challenger is rewarded from the agent's collateral.
     */
    event IllegalPaymentConfirmed(
        address indexed agentVault,
        bytes32 transactionHash);

    /**
     * Two transactions with the same payment reference, both from the agent's underlying address, were proved.
     * Whole agent's position goes into liquidation.
     * The challenger is rewarded from the agent's collateral.
     */
    event DuplicatePaymentConfirmed(
        address indexed agentVault,
        bytes32 transactionHash1,
        bytes32 transactionHash2);

    /**
     * Agent's underlying balance became lower than required for backing f-assets (either through payment or via
     * a challenge. Agent goes to a full liquidation.
     * The challenger is rewarded from the agent's collateral.
     */
    event UnderlyingBalanceTooLow(
        address indexed agentVault,
        int256 balance,
        uint256 requiredBalance);

    /**
     * A setting has changed.
     */
    event SettingChanged(
        string name,
        uint256 value);

    /**
     * A setting has changed.
     */
    event SettingArrayChanged(
        string name,
        uint256[] value);

    /**
     * A contract in the settings has changed.
     */
    event ContractChanged(
        string name,
        address value);

    /**
     * Current underlying block number or timestamp has been updated.
     */
    event CurrentUnderlyingBlockUpdated(
        uint256 underlyingBlockNumber,
        uint256 underlyingBlockTimestamp,
        uint256 updatedAt);

    /**
     * New collateral token has been added.
     */
    event CollateralTypeAdded(
        uint8 collateralClass,
        address token,
        uint256 decimals,
        bool directPricePair,
        string assetFtsoSymbol,
        string tokenFtsoSymbol,
        uint256 minCollateralRatioBIPS,
        uint256 safetyMinCollateralRatioBIPS);

    /**
     * System defined collateral ratios for the token have changed (minimal and safety collateral ratio).
     */
    event CollateralRatiosChanged(
        uint8 collateralClass,
        address collateralToken,
        uint256 minCollateralRatioBIPS,
        uint256 safetyMinCollateralRatioBIPS);

    /**
     * Collateral token has been marked as deprecated. After the timestamp `validUntil` passes, it will be
     * considered invalid and the agents who haven't switched their collateral before will be liquidated.
     */
    event CollateralTypeDeprecated(
        uint8 collateralClass,
        address collateralToken,
        uint256 validUntil);

    /**
     * Emergency pause was triggered.
     */
    event EmergencyPauseTriggered(
        uint256 pausedUntil);

    /**
     * Emergency pause was canceled.
     */
    event EmergencyPauseCanceled();

    /**
     * Emergency pause transfers was triggered.
     */
    event EmergencyPauseTransfersTriggered(
        uint256 pausedUntil);

    /**
     * Emergency pause transfers was canceled.
     */
    event EmergencyPauseTransfersCanceled();
}

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


library RedemptionRequestInfo {
    enum Status {
        ACTIVE,                 // waiting for confirmation/default
        DEFAULTED_UNCONFIRMED,  // default called, failed or late payment can still be confirmed
        // final statuses - there can be no valid payment for this redemption anymore
        SUCCESSFUL,             // successful payment confirmed
        DEFAULTED_FAILED,       // payment failed   (default was paid)
        BLOCKED,                // payment blocked
        REJECTED                // redemption request rejected due to invalid redeemer's address
    }

    struct Data {
        // The id used for confirming or defaulting the request.
        uint64 redemptionRequestId;

        // Redemption status. Note that on payment confirmation the request is deleted, so there is no success status.
        RedemptionRequestInfo.Status status;

        // The redeemed agent vault.
        address agentVault;

        // Native redeemer address - the address that receives collateral in case of default.
        address redeemer;

        // The underlying address to which the redeemed assets should be paid by the agent.
        string paymentAddress;

        // Payment reference that must be part of the agent's redemption payment.
        bytes32 paymentReference;

        // The amount of the FAsset the redeemer has burned. Note that this is not the amount of underlying
        // the redeemer will receive - the redemption payment amount is this minus the underlyingFeeUBA.
        uint128 valueUBA;

        // The redemption fee that remain on agent's underlying address.
        // Part of it will be reminted as pool fee share and the rest becomes the agent's free underlying.
        uint128 feeUBA;

        // Proportional part of the underlyingFeeUBA that is re-minted on successful redemption
        // and goes to the collateral pool.
        uint16 poolFeeShareBIPS;

        // The underlying block (approximate - as known by the asset manager) when the request occurred.
        uint64 firstUnderlyingBlock;

        // The last underlying block and timestamp for redemption payment. Redemption is defaulted if
        // there is no payment by the time BOTH lastUnderlyingBlock and lastUnderlyingTimestamp have passed.
        uint64 lastUnderlyingBlock;
        uint64 lastUnderlyingTimestamp;

        // The native (Flare/Songbird) chain timestamp when the request occurred.
        uint64 timestamp;

        // True if redemption was created by a selfCloseExit on the collateral pool.
        bool poolSelfClose;

        // True if redemption was initiated by an agent for transfer to core vault.
        bool transferToCoreVault;

        // The executor, optionally assigned by the redeemer to execute the default if needed.
        // (Only redeemer, agent or executor may execute the default.)
        address executor;

        // The fee in NAT that the executor receives if they successfully call default.
        uint256 executorFeeNatWei;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;


library CollateralReservationInfo {
    enum Status {
        ACTIVE,         // the minting process hasn't finished yet
        SUCCESSFUL,     // the payment has been confirmed and the FAssets minted
        DEFAULTED,      // the payment has defaulted and the agent received the collateral reservation fee
        EXPIRED         // the confirmation time has expired and the agent called unstickMinting
    }

    struct Data {
        // The id used for executing or defaulting the minting.
        uint64 collateralReservationId;

        // The agent vault whose collateral is reserved.
        address agentVault;

        // The minter address - the address that will receive the minted FAssets.
        address minter;

        // The agent's underlying address to which the underlying assets should be paid by the minter.
        string paymentAddress;

        // Payment reference that must be part of the agent's redemption payment.
        bytes32 paymentReference;

        // The amount of FAssets that the minter will receive. Always a whole number of lots.
        uint256 valueUBA;

        // The underlying fee. The total amount the minter has to deposit is `valueUBA + mintingFeeUBA`.
        // Part of the fee is minted as pool fee share and the rest becomes agent's free underlying.
        uint128 mintingFeeUBA;

        // The fee that was paid at the collateral reservation time.
        // Part of the fee is goes to the pool and the rest to the agent vault as WNAT.
        uint128 reservationFeeNatWei;

        // Proportion of the mintingFeeUBA and reservationFeeNatWei that belongs to the collateral pool.
        uint16 poolFeeShareBIPS;

        // The underlying block (approximate - as known by the asset manager) when the reservation occurred.
        uint64 firstUnderlyingBlock;

        // The last underlying block and timestamp for redemption payment. Redemption is defaulted if
        // there is no payment by the time BOTH lastUnderlyingBlock and lastUnderlyingTimestamp have passed.
        uint64 lastUnderlyingBlock;
        uint64 lastUnderlyingTimestamp;

        // The executor, optionally assigned by the minter to execute the minting.
        // (Only minter, agent or executor may execute the minting.)
        address executor;

        // The fee in NAT that the executor receives if they successfully execute the minting.
        uint256 executorFeeNatWei;

        // If the minting process has finished, indication of success/default. Otherwise ACTIVE.
        CollateralReservationInfo.Status status;
    }
}

// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v4.9.0) (token/ERC20/extensions/ERC20Permit.sol)

pragma solidity ^0.8.0;

import {IERC20Permit} from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Permit.sol";
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {Counters} from "@openzeppelin/contracts/utils/Counters.sol";
import {EIP712} from "../utils/EIP712.sol";

/**
 * @dev Implementation of the ERC20 Permit extension allowing approvals to be made via signatures, as defined in
 * https://eips.ethereum.org/EIPS/eip-2612[EIP-2612].
 *
 * Adds the {permit} method, which can be used to change an account's ERC20 allowance (see {IERC20-allowance}) by
 * presenting a message signed by the account. By not relying on `{IERC20-approve}`, the token holder account doesn't
 * need to send a transaction, and thus is not required to hold Ether at all.
 *
 * @dev Copied from @openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol but updated
 * to be used in upgradable contracts. The original implementation uses EIP712 with immutable variables,
 * which makes it non-upgradable.
 * Moreover, ERC20 is not imported to make sure there are no storage issues. So instead of inherited _approve,
 * an abstract method is used which must be implemented by calling ERC20._approve.
 */
abstract contract ERC20Permit is IERC20Permit, EIP712 {
    using Counters for Counters.Counter;

    error ERC20PermitExpiredDeadline();
    error ERC20PermitInvalidSignature();

    struct ERC20PermitState {
        mapping(address => Counters.Counter) nonces;
    }

    bytes32 private constant _PERMIT_TYPEHASH =
        keccak256("Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)");

    // should be implemented by calling ERC20._approve
    function _approve(address owner, address spender, uint256 amount) internal virtual;

    /**
     * @dev See {IERC20Permit-permit}.
     */
    function permit(
        address owner,
        address spender,
        uint256 value,
        uint256 deadline,
        uint8 v,
        bytes32 r,
        bytes32 s
    ) public virtual override {
        require(block.timestamp <= deadline, ERC20PermitExpiredDeadline());

        bytes32 structHash =
            keccak256(abi.encode(_PERMIT_TYPEHASH, owner, spender, value, _useNonce(owner), deadline));

        bytes32 hash = _hashTypedDataV4(structHash);

        address signer = ECDSA.recover(hash, v, r, s);
        require(signer == owner, ERC20PermitInvalidSignature());

        _approve(owner, spender, value);
    }

    /**
     * @dev See {IERC20Permit-nonces}.
     */
    function nonces(address owner) public view virtual override returns (uint256) {
        return _getERC20PermitState().nonces[owner].current();
    }

    /**
     * @dev See {IERC20Permit-DOMAIN_SEPARATOR}.
     */
    // solhint-disable-next-line func-name-mixedcase
    function DOMAIN_SEPARATOR() external view override returns (bytes32) {
        return _domainSeparatorV4();
    }

    /**
     * @dev "Consume a nonce": return the current value and increment.
     *
     * _Available since v4.1._
     */
    function _useNonce(address owner) internal virtual returns (uint256 current) {
        Counters.Counter storage nonce = _getERC20PermitState().nonces[owner];
        current = nonce.current();
        nonce.increment();
    }

    // keccak256(abi.encode(uint256(keccak256("fasset.openzeppelin.ERC20Permit")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant ERC20PERMIT_STORAGE = 0x361beb44631d074062265988ad0b416453c05cfc27148633e5eff4da54187b00;

    function _getERC20PermitState()
        private pure
        returns (ERC20PermitState storage _state)
    {
        // solhint-disable-next-line no-inline-assembly
        assembly {
            _state.slot := ERC20PERMIT_STORAGE
        }
    }
}
// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

interface IAgentPing {
    /**
     * Agent bot liveness check.
     * @param agentVault the agent vault whose owner bot to ping
     * @param sender the account that triggered ping; helps bot decide whether it is important to answer
     * @param query off-chain defined id of the query
     */
    event AgentPing(
        address indexed agentVault,
        address indexed sender,
        uint256 query);

    /**
     * Response to agent bot liveness check.
     * @param agentVault the pinged agent vault
     * @param owner owner of the agent vault (management address)
     * @param query repeated `query` from the AgentPing event
     * @param response response data to the query
     */
    event AgentPingResponse(
        address indexed agentVault,
        address indexed owner,
        uint256 query,
        string response);

    /**
     * Used for liveness checks, simply emits AgentPing event.
     * @param _agentVault the agent vault whose owner bot to ping
     * @param _query off-chain defined id of the query
     */
    function agentPing(
        address _agentVault,
        uint256 _query
    ) external;

    /**
     * Used for liveness checks, the bot's response to AgentPing event.
     * Simply emits AgentPingResponse event identifying the owner.
     * NOTE: may only be called by the agent vault owner
     * @param _agentVault the pinged agent vault
     * @param _query repeated `_query` from the agentPing
     * @param _response response data to the query
     */
    function agentPingResponse(
        address _agentVault,
        uint256 _query,
        string memory _response
    ) external;
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IWNat} from "../../flareSmartContracts/interfaces/IWNat.sol";
import {Reentrancy} from "../../openzeppelin/library/Reentrancy.sol";


library Transfers {
    uint256 internal constant TRANSFER_GAS_ALLOWANCE = 100_000;

    error TransferFailed();

    // make sure the transfer is only called in non-reentrant method
    modifier requireReentrancyGuard {
        Reentrancy.requireReentrancyGuard();
        _;
    }

    /**
     * Transfer the given amount of NAT to recipient without gas limit of `address.transfer()`.
     *
     * **Warning**: Must guard with nonReentrant, otherwise the method will fail.
     *
     * **Warning 2**: may fail, so only use when the top-level transaction sender controls recipient address
     * (and therefore expects to fail if there is something strange at that address).
     *
     * @param _recipient the recipient address
     * @param _amount the amount in NAT Wei
     */
    function transferNAT(address payable _recipient, uint256 _amount)
        internal
        requireReentrancyGuard
    {
        if (_amount > 0) {
            /* solhint-disable avoid-low-level-calls */
            //slither-disable-next-line arbitrary-send-eth
            (bool success, ) = _recipient.call{value: _amount, gas: TRANSFER_GAS_ALLOWANCE}("");
            /* solhint-enable avoid-low-level-calls */
            require(success, TransferFailed());
        }
    }

    /**
     * Deposits the given amount of NAT to recipient on WNat contract.
     *
     * @param _wNat the WNat contract address
     * @param _recipient the recipient address
     * @param _amount the amount in NAT Wei
     */
    function depositWNat(IWNat _wNat, address _recipient, uint256 _amount)
        internal
    {
        if (_amount > 0) {
            _wNat.depositTo{value: _amount}(_recipient);
        }
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {SafePct} from "../../utils/library/SafePct.sol";
import {AssetManagerState} from "./data/AssetManagerState.sol";
import {RedemptionTimeExtension} from "./data/RedemptionTimeExtension.sol";
import {IAssetManagerEvents} from "../../userInterfaces/IAssetManagerEvents.sol";
import {Conversion} from "./Conversion.sol";
import {Redemption} from "./data/Redemption.sol";
import {Agent} from "./data/Agent.sol";
import {Globals} from "./Globals.sol";
import {AgentBacking} from "./AgentBacking.sol";
import {AssetManagerSettings} from "../../userInterfaces/data/AssetManagerSettings.sol";
import {PaymentReference} from "./data/PaymentReference.sol";


library RedemptionRequests {
    using SafePct for uint256;
    using SafeCast for uint256;

    error CannotRedeemToAgentsAddress();
    error UnderlyingAddressTooLong();
    error ExecutorFeeWithoutExecutor();

    struct AgentRedemptionData {
        address agentVault;
        uint64 valueAMG;
    }

    struct AgentRedemptionList {
        AgentRedemptionData[] items;
        uint256 length;
    }

    function createRedemptionRequest(
        AgentRedemptionData memory _data,
        address _redeemer,
        string memory _redeemerUnderlyingAddressString,
        bool _poolSelfClose,
        address payable _executor,
        uint64 _executorFeeNatGWei,
        uint64 _additionalPaymentTime,
        bool _transferToCoreVault
    )
        internal
        returns (uint64 _requestId)
    {
        require(_executorFeeNatGWei == 0 || _executor != address(0), ExecutorFeeWithoutExecutor());
        AssetManagerState.State storage state = AssetManagerState.get();
        Agent.State storage agent = Agent.get(_data.agentVault);
        // validate redemption address
        require(bytes(_redeemerUnderlyingAddressString).length < 128, UnderlyingAddressTooLong());
        bytes32 underlyingAddressHash = keccak256(bytes(_redeemerUnderlyingAddressString));
        // both addresses must be normalized (agent's address is checked at vault creation,
        // and if redeemer address isn't normalized, the agent can trigger rejectInvalidRedemption),
        // so this comparison quarantees the redemption is not to the agent's address
        require(underlyingAddressHash != agent.underlyingAddressHash,
            CannotRedeemToAgentsAddress());
        // create request
        uint128 redeemedValueUBA = Conversion.convertAmgToUBA(_data.valueAMG).toUint128();
        _requestId = _newRequestId(_poolSelfClose);
        // create in-memory request and then put it to storage to not go out-of-stack
        Redemption.Request memory request;
        request.redeemerUnderlyingAddressHash = underlyingAddressHash;
        request.underlyingValueUBA = redeemedValueUBA;
        request.firstUnderlyingBlock = state.currentUnderlyingBlock;
        (request.lastUnderlyingBlock, request.lastUnderlyingTimestamp) =
            _lastPaymentBlock(_data.agentVault, _additionalPaymentTime);
        request.timestamp = block.timestamp.toUint64();
        request.underlyingFeeUBA = _transferToCoreVault ?
            0 : uint256(redeemedValueUBA).mulBips(Globals.getSettings().redemptionFeeBIPS).toUint128();
        request.redeemer = _redeemer;
        request.agentVault = _data.agentVault;
        request.valueAMG = _data.valueAMG;
        request.status = Redemption.Status.ACTIVE;
        request.poolSelfClose = _poolSelfClose;
        request.executor = _executor;
        request.executorFeeNatGWei = _executorFeeNatGWei;
        request.redeemerUnderlyingAddressString = _redeemerUnderlyingAddressString;
        request.transferToCoreVault = _transferToCoreVault;
        request.poolFeeShareBIPS = agent.redemptionPoolFeeShareBIPS;
        state.redemptionRequests[_requestId] = request;
        // decrease mintedAMG and mark it to redeemingAMG
        // do not add it to freeBalance yet (only after failed redemption payment)
        AgentBacking.startRedeemingAssets(agent, _data.valueAMG, _poolSelfClose);
        // emit event to remind agent to pay
        _emitRedemptionRequestedEvent(request, _requestId, _redeemerUnderlyingAddressString);
    }

    function _emitRedemptionRequestedEvent(
        Redemption.Request memory _request,
        uint64 _requestId,
        string memory _redeemerUnderlyingAddressString
    )
        private
    {
        emit IAssetManagerEvents.RedemptionRequested(
            _request.agentVault,
            _request.redeemer,
            _requestId,
            _redeemerUnderlyingAddressString,
            _request.underlyingValueUBA,
            _request.underlyingFeeUBA,
            _request.firstUnderlyingBlock,
            _request.lastUnderlyingBlock,
            _request.lastUnderlyingTimestamp,
            PaymentReference.redemption(_requestId),
            _request.executor,
            _request.executorFeeNatGWei * Conversion.GWEI);
    }

    function _newRequestId(bool _poolSelfClose)
        private
        returns (uint64)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        uint64 nextRequestId = state.newRedemptionRequestId + PaymentReference.randomizedIdSkip();
        // the requestId will indicate in the lowest bit whether it is a pool self close redemption
        // (+1 is added so that the request id still increases after clearing lowest bit)
        uint64 requestId = ((nextRequestId + 1) & ~uint64(1)) | (_poolSelfClose ? 1 : 0);
        state.newRedemptionRequestId = requestId;
        return requestId;
    }

    function _lastPaymentBlock(address _agentVault, uint64 _additionalPaymentTime)
        private
        returns (uint64 _lastUnderlyingBlock, uint64 _lastUnderlyingTimestamp)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // timeshift amortizes for the time that passed from the last underlying block update;
        // it also adds redemption time extension when there are many redemption requests in short time
        uint64 timeshift = block.timestamp.toUint64() - state.currentUnderlyingBlockUpdatedAt
            + RedemptionTimeExtension.extendTimeForRedemption(_agentVault)
            + _additionalPaymentTime;
        uint64 blockshift = (uint256(timeshift) * 1000 / settings.averageBlockTimeMS).toUint64();
        _lastUnderlyingBlock =
            state.currentUnderlyingBlock + blockshift + settings.underlyingBlocksForPayment;
        _lastUnderlyingTimestamp =
            state.currentUnderlyingBlockTimestamp + timeshift + settings.underlyingSecondsForPayment;
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Math} from "@openzeppelin/contracts/utils/math/Math.sol";
import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";


/**
 * @title Check Point History library
 * @notice A contract to manage checkpoints as of a given block.
 * @dev Store value history by block number with detachable state.
 **/
library CheckPointHistory {
    using SafeCast for uint256;

    error ValueDoesNotFitInOneNineTwoBits();
    error CheckPointHistoryReadingFromCleanedupBlock();

    /**
     * @dev `CheckPoint` is the structure that attaches a block number to a
     *  given value; the block number attached is the one that last changed the
     *  value
     **/
    struct CheckPoint {
        // `value` is the amount of tokens at a specific block number
        uint192 value;
        // `fromBlock` is the block number that the value was generated from
        uint64 fromBlock;
    }

    struct CheckPointHistoryState {
        // `checkpoints` is an array that tracks values at non-contiguous block numbers
        mapping(uint256 => CheckPoint) checkpoints;
        // `checkpoints` before `startIndex` have been deleted
        // INVARIANT: checkpoints.endIndex == 0 || startIndex < checkpoints.endIndex      (strict!)
        // startIndex and endIndex are both less then fromBlock, so 64 bits is enough
        uint64 startIndex;
        // the index AFTER last
        uint64 endIndex;
    }

    /**
     * @notice Binary search of _checkpoints array.
     * @param _checkpoints An array of CheckPoint to search.
     * @param _startIndex Smallest possible index to be returned.
     * @param _blockNumber The block number to search for.
     */
    function _indexOfGreatestBlockLessThan(
        mapping(uint256 => CheckPoint) storage _checkpoints,
        uint256 _startIndex,
        uint256 _endIndex,
        uint256 _blockNumber
    )
        private view
        returns (uint256 index)
    {
        // Binary search of the value by given block number in the array
        uint256 min = _startIndex;
        uint256 max = _endIndex - 1;
        while (max > min) {
            uint256 mid = (max + min + 1) / 2;
            if (_checkpoints[mid].fromBlock <= _blockNumber) {
                min = mid;
            } else {
                max = mid - 1;
            }
        }
        return min;
    }

    /**
     * @notice Queries the value at a specific `_blockNumber`
     * @param _self A CheckPointHistoryState instance to manage.
     * @param _blockNumber The block number of the value active at that time
     * @return _value The value at `_blockNumber`
     **/
    function valueAt(
        CheckPointHistoryState storage _self,
        uint256 _blockNumber
    )
        internal view
        returns (uint256 _value)
    {
        uint256 historyCount = _self.endIndex;

        // No _checkpoints, return 0
        if (historyCount == 0) return 0;

        // Shortcut for the actual value (extra optimized for current block, to save one storage read)
        // historyCount - 1 is safe, since historyCount != 0
        if (_blockNumber >= block.number || _blockNumber >= _self.checkpoints[historyCount - 1].fromBlock) {
            return _self.checkpoints[historyCount - 1].value;
        }

        // guard values at start
        uint256 startIndex = _self.startIndex;
        if (_blockNumber < _self.checkpoints[startIndex].fromBlock) {
            // reading data before `startIndex` is only safe before first cleanup
            require(startIndex == 0, CheckPointHistoryReadingFromCleanedupBlock());
            return 0;
        }

        // Find the block with number less than or equal to block given
        uint256 index = _indexOfGreatestBlockLessThan(_self.checkpoints, startIndex, _self.endIndex, _blockNumber);

        return _self.checkpoints[index].value;
    }

    /**
     * @notice Queries the value at `block.number`
     * @param _self A CheckPointHistoryState instance to manage.
     * @return _value The value at `block.number`
     **/
    function valueAtNow(CheckPointHistoryState storage _self) internal view returns (uint256 _value) {
        uint256 historyCount = _self.endIndex;
        // No _checkpoints, return 0
        if (historyCount == 0) return 0;
        // Return last value
        return _self.checkpoints[historyCount - 1].value;
    }

    /**
     * @notice Writes the value at the current block.
     * @param _self A CheckPointHistoryState instance to manage.
     * @param _value Value to write.
     **/
    function writeValue(
        CheckPointHistoryState storage _self,
        uint256 _value
    )
        internal
    {
        uint256 historyCount = _self.endIndex;
        if (historyCount == 0) {
            // checkpoints array empty, push new CheckPoint
            _self.checkpoints[0] =
                CheckPoint({ fromBlock: block.number.toUint64(), value: _toUint192(_value) });
            _self.endIndex = 1;
        } else {
            // historyCount - 1 is safe, since historyCount != 0
            CheckPoint storage lastCheckpoint = _self.checkpoints[historyCount - 1];
            uint256 lastBlock = lastCheckpoint.fromBlock;
            // slither-disable-next-line incorrect-equality
            if (block.number == lastBlock) {
                // If last check point is the current block, just update
                lastCheckpoint.value = _toUint192(_value);
            } else {
                // we should never have future blocks in history
                assert (block.number > lastBlock);
                // push new CheckPoint
                _self.checkpoints[historyCount] =
                    CheckPoint({ fromBlock: block.number.toUint64(), value: _toUint192(_value) });
                _self.endIndex = uint64(historyCount + 1);  // 64 bit safe, because historyCount <= block.number
            }
        }
    }

    /**
     * Delete at most `_count` of the oldest checkpoints.
     * At least one checkpoint at or before `_cleanupBlockNumber` will remain
     * (unless the history was empty to start with).
     */
    function cleanupOldCheckpoints(
        CheckPointHistoryState storage _self,
        uint256 _count,
        uint256 _cleanupBlockNumber
    )
        internal
        returns (uint256)
    {
        if (_cleanupBlockNumber == 0) return 0;   // optimization for when cleaning is not enabled
        uint256 length = _self.endIndex;
        if (length == 0) return 0;
        uint256 startIndex = _self.startIndex;
        // length - 1 is safe, since length != 0 (check above)
        uint256 endIndex = Math.min(startIndex + _count, length - 1);    // last element can never be deleted
        uint256 index = startIndex;
        // we can delete `checkpoint[index]` while the next checkpoint is at `_cleanupBlockNumber` or before
        while (index < endIndex && _self.checkpoints[index + 1].fromBlock <= _cleanupBlockNumber) {
            delete _self.checkpoints[index];
            index++;
        }
        if (index > startIndex) {   // index is the first not deleted index
            _self.startIndex = index.toUint64();
        }
        return index - startIndex;  // safe: index >= startIndex at start and then increases
    }

    // SafeCast lib is missing cast to uint192
    function _toUint192(uint256 _value) internal pure returns (uint192) {
        require(_value < 2**192, ValueDoesNotFitInOneNineTwoBits());
        return uint192(_value);
    }
}
// SPDX-License-Identifier: MIT

// OpenZeppelin Contracts (last updated v4.9.0) (security/ReentrancyGuard.sol)
// Modified by FlareLabs to use diamond storage

pragma solidity ^0.8.27;


/**
 * Code for the `ReentrancyGuard` contract.
 */
library Reentrancy {

    error ReentrancyGuardReentrantCall();
    error ReentrancyGuardRequired();

    // Booleans are more expensive than uint256 or any type that takes up a full
    // word because each write operation emits an extra SLOAD to first read the
    // slot's contents, replace the bits taken up by the boolean, and then write
    // back. This is the compiler's defense against contract upgrades and
    // pointer aliasing, and it cannot be disabled.

    // The values being non-zero value makes deployment a bit more expensive,
    // but in exchange the refund on every call to nonReentrant will be lower in
    // amount. Since refunds are capped to a percentage of the total
    // transaction's gas, it is best to keep them low in cases like this one, to
    // increase the likelihood of the full refund coming into effect.
    uint256 private constant _NOT_ENTERED = 1;
    uint256 private constant _ENTERED = 2;

    struct ReentrancyGuardState {
        uint256 status;
    }

    /**
     * Should be called once at construction time of the main diamond contract.
     * Not a big issue if it is never called - just the first nonReentrant method call will use more gas.
     */
    function initializeReentrancyGuard() internal {
        ReentrancyGuardState storage state = _reentrancyGuardState();
        state.status = _NOT_ENTERED;
    }

    function nonReentrantBefore() internal {
        ReentrancyGuardState storage state = _reentrancyGuardState();
        // On the first call to nonReentrant, state.status will be _NOT_ENTERED
        require(state.status != _ENTERED, ReentrancyGuardReentrantCall());

        // Any calls to nonReentrant after this point will fail
        state.status = _ENTERED;
    }

    function nonReentrantAfter() internal {
        ReentrancyGuardState storage state = _reentrancyGuardState();
        // By storing the original value once again, a refund is triggered (see
        // https://eips.ethereum.org/EIPS/eip-2200)
        state.status = _NOT_ENTERED;
    }

    /**
     * @dev Returns true if the reentrancy guard is currently set to "entered", which indicates there is a
     * `nonReentrant` function in the call stack.
     */
    function reentrancyGuardEntered() internal view returns (bool) {
        ReentrancyGuardState storage state = _reentrancyGuardState();
        return state.status == _ENTERED;
    }

    /**
     * Marks a piece of code that can only be executed within a `nonReentrant` method.
     * Useful to prevent e.g. NAT transfers that don't properly guard against reentrancy
     * and to make them fail at test time.
     */
    function requireReentrancyGuard() internal view {
        require(reentrancyGuardEntered(), ReentrancyGuardRequired());
    }

    function _reentrancyGuardState() private pure returns (ReentrancyGuardState storage _state) {
        bytes32 position = keccak256("utils.ReentrancyGuard.ReentrancyGuardState");
        // solhint-disable-next-line no-inline-assembly
        assembly {
            _state.slot := position
        }
    }
}
// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

library AgentInfo {
    enum Status {
        // agent is operating normally
        NORMAL,
        // liquidation due to collateral ratio - ends when agent is healthy
        LIQUIDATION,
        // illegal payment liquidation - always liquidates all and then agent must close vault
        FULL_LIQUIDATION,
        // agent announced destroy, cannot mint again; all existing mintings have been redeemed before
        DESTROYING,
        // agent has been destroyed, cannot do anything except return info
        // owner can still withdraw tokens from the vault
        DESTROYED
    }

    struct Info {
        // Current agent's status.
        AgentInfo.Status status;
        // Agent vault owner's management address, used for occasional administration.
        // Immutable.
        address ownerManagementAddress;
        // Agent vault owner's work address, used for automatic operations.
        // Can be changed by a call from the owner's management address.
        address ownerWorkAddress;
        // Agent's collateral pool address
        address collateralPool;
        // Agent collateral pool's pool token address
        address collateralPoolToken;
        // Underlying address as string - to be used for minting payments.
        // For most other purposes, you use underlyingAddressHash, which is `keccak256(underlyingAddressString)`.
        string underlyingAddressString;
        // If true, anybody can mint against this agent.
        // If false, the agent can only self-mint.
        // Once minted, all redemption tickets go to the same (public) queue, regardless of this flag.
        bool publiclyAvailable;
        // Current fee the agent charges for minting (paid in underlying currency).
        uint256 feeBIPS;
        // Share of the minting fee that goes to the pool as percentage of the minting fee.
        // This share of fee is minted as f-assets and belongs to the pool.
        uint256 poolFeeShareBIPS;
        // The token identifier of the agent's current vault collateral.
        // Token identifier can be used to call AssetManager.getCollateralType().
        IERC20 vaultCollateralToken;
        // Amount, set by agent, at which locked and free collateral are calculated for new mintings.
        // For agent's vault collateral.
        uint256 mintingVaultCollateralRatioBIPS;
        // Amount, set by agent, at which locked and free collateral are calculated for new mintings.
        // For pool collateral.
        uint256 mintingPoolCollateralRatioBIPS;
        // The maximum number of lots that the agent can mint.
        // This can change any moment due to minting, redemption or price changes.
        uint256 freeCollateralLots;
        // Total amount of vault collateral in agent's vault.
        uint256 totalVaultCollateralWei;
        // Free collateral, available for new mintings.
        // Note: this value doesn't tell you anything about agent being near liquidation, since it is
        // calculated at agentMinCollateralRatio, not minCollateralRatio.
        // Use collateralRatioBIPS to see whether the agent is near liquidation.
        uint256 freeVaultCollateralWei;
        // The actual agent's collateral ratio, as it is used in liquidation.
        // For calculation, the system checks both FTSO prices and trusted provider's prices and uses
        // the ones that give higher ratio.
        uint256 vaultCollateralRatioBIPS;
        // The token identifier of the agent's current vault collateral.
        // Token identifier can be used to call AssetManager.getCollateralType().
        IERC20 poolWNatToken;
        // Total amount of NAT collateral in agent's pool.
        uint256 totalPoolCollateralNATWei;
        // Free NAT pool collateral (see vault collateral for details).
        uint256 freePoolCollateralNATWei;
        // The actual pool collateral ratio (see vault collateral for details).
        uint256 poolCollateralRatioBIPS;
        // The amount of pool tokens that belong to agent's vault. This limits the amount of possible
        // minting: to be able to mint, the NAT value of all backed fassets together with new ones, times
        // mintingPoolHoldingsRequiredBIPS, must be smaller than the agent's pool tokens amount converted to NAT.
        // Note: the amount of agent's pool tokens only affects minting, not liquidation.
        uint256 totalAgentPoolTokensWei;
        // The amount of vault collateral that will be withdrawn by the agent.
        uint256 announcedVaultCollateralWithdrawalWei;
        // The amount of pool tokens that will be withdrawn by the agent.
        uint256 announcedPoolTokensWithdrawalWei;
        // Free agent's pool tokens.
        uint256 freeAgentPoolTokensWei;
        // Total amount of minted f-assets.
        uint256 mintedUBA;
        // Total amount reserved for ongoing mintings.
        uint256 reservedUBA;
        // Total amount of ongoing redemptions.
        uint256 redeemingUBA;
        // Total amount of ongoing redemptions that lock the pool collateral.
        // (In pool self-close exits, pool collateral is not locked. So the amount of locked
        // collateral in the pool can be less than the amount of locked vault collateral.)
        uint256 poolRedeemingUBA;
        // Total amount of dust (unredeemable minted f-assets).
        // Note: dustUBA is part of mintedUBA, so the amount of redeemable f-assets is calculated as
        // `mintedUBA - dustUBA`
        uint256 dustUBA;
        // Liquidation info
        // If the agent is in LIQUIDATION or FULL_LIQUIDATION, the time agent entered liquidation.
        // If status is neither of that, returns 0.
        // Can be used for calculating current liquidation premium, which depends on time since liquidation started.
        uint256 liquidationStartTimestamp;
        // When agent is in liquidation, this is the amount o FAssets that need to be liquidated to bring the agent's
        // position to safety. When performing liquidation, only up to this amount of FAssets will be liquidated.
        // If not in liquidation, this value is 0.
        // Since the liquidation state may need to be upgraded by, call `startLiquidation` before
        // `getAgentInfo` to get the value that will actually be used in liquidation.
        uint256 maxLiquidationAmountUBA;
        // When agent is in liquidation, this is the factor (in BIPS) of the converted value of the liquidated
        // FAssets paid by the vault collateral. If not in liquidation, this value is 0.
        uint256 liquidationPaymentFactorVaultBIPS;
        // When agent is in liquidation, this is the factor (in BIPS) of the converted value of the liquidated
        // FAssets paid by the pool collateral. If not in liquidation, this value is 0.
        uint256 liquidationPaymentFactorPoolBIPS;
        // Total underlying balance (backing and free).
        int256 underlyingBalanceUBA;
        // The minimum underlying balance that has to be held by the agent. Below this, agent is liquidated.
        uint256 requiredUnderlyingBalanceUBA;
        // Underlying balance not backing anything (can be used for gas/fees or withdrawn after announcement).
        int256 freeUnderlyingBalanceUBA;
        // Current underlying withdrawal announcement (or 0 if no announcement was made).
        uint256 announcedUnderlyingWithdrawalId;
        // The factor set by the agent to multiply the price at which agent buys f-assets from pool
        // token holders on self-close exit (when requested or the redeemed amount is less than 1 lot).
        uint256 buyFAssetByAgentFactorBIPS;
        // The minimum collateral ratio above which a staker can exit the pool
        // (this is CR that must be left after exit).
        // Must be higher than system minimum collateral ratio for pool collateral.
        uint256 poolExitCollateralRatioBIPS;
        // The redemption fee share paid to the pool (as FAssets).
        // In redemption dominated situations (when agent requests return from core vault to earn
        // from redemption fees), pool can get some share to make it sustainable for pool users.
        // NOTE: the pool fee share is locked at the redemption request time, but is charged at the redemption
        // confirmation time. If agent uses all the redemption fee for transaction fees, this could make the
        // agent's free underlying balance negative.
        uint256 redemptionPoolFeeShareBIPS;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;
pragma abicoder v2;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

interface ICollateralPoolToken is IERC20 {

    /**
     * Returns the address of the collateral pool that issued this token.
     */
    function collateralPool()
        external view
        returns (address);

    /**
     * Returns the amount of tokens that is locked for transferring.
     */
    function lockedBalanceOf(address _account)
        external view
        returns (uint256);

    /**
     * Returns the amount of tokens that can be transferred.
     */
    function transferableBalanceOf(address _account)
        external view
        returns (uint256);

    /**
     * Returns the amount of account's tokens that are considered debt.
     */
    function debtLockedBalanceOf(address _account)
        external view
        returns (uint256);

    /**
     * Returns the amount of account's tokens that are not considered debt.
     */
    function debtFreeBalanceOf(address _account)
        external view
        returns (uint256);

    /**
     * Returns the amount of account's tokens that are timelocked.
     */
    function timelockedBalanceOf(address _account)
        external view
        returns (uint256);

    /**
     * Returns the amount of account's tokens that are timelocked.
     */
    function nonTimelockedBalanceOf(address _account)
        external view
        returns (uint256);
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

interface IRedemptionTimeExtension {
    function setRedemptionPaymentExtensionSeconds(uint256 _value)
        external;

    function redemptionPaymentExtensionSeconds()
        external view
        returns (uint256);
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import {IPayment} from "@flarenetwork/flare-periphery-contracts/flare/IFdcVerification.sol";


/**
 * Core vault
 */
interface ICoreVaultClient {
    /**
     * Agent has requested transfer of (some of) their backing to the core vault.
     */
    event TransferToCoreVaultStarted(
        address indexed agentVault,
        uint256 indexed transferRedemptionRequestId,
        uint256 valueUBA);

    /**
     * Agent has cancelled transfer to the core vault without paying.
     * The amount of `valueUBA` has been re-minted./
     */
    event TransferToCoreVaultDefaulted(
        address indexed agentVault,
        uint256 indexed transferRedemptionRequestId,
        uint256 remintedUBA);

    /**
     * The transfer of underlying to the core vault was successfully completed.
     */
    event TransferToCoreVaultSuccessful(
        address indexed agentVault,
        uint256 indexed transferRedemptionRequestId,
        uint256 valueUBA);

    /**
     * The agent has requested return of some of the underlying from the core vault to the agent's underlying address.
     */
    event ReturnFromCoreVaultRequested(
        address indexed agentVault,
        uint256 indexed requestId,
        bytes32 paymentReference,
        uint256 valueUBA);

    /**
     * The agent has cancelled the return request.
     */
    event ReturnFromCoreVaultCancelled(
        address indexed agentVault,
        uint256 indexed requestId);

    /**
     * The payment from core vault to the agent's underlying address has been confirmed.
     */
    event ReturnFromCoreVaultConfirmed(
        address indexed agentVault,
        uint256 indexed requestId,
        uint256 receivedUnderlyingUBA,
        uint256 remintedUBA);

    /**
     * Redemption was requested from a core vault.
     * Can only be redeemed to a payment address from to the `allowedDestinations` list in the core vault manager.
     */
    event CoreVaultRedemptionRequested(
        address indexed redeemer,
        string paymentAddress,
        bytes32 paymentReference,
        uint256 valueUBA,
        uint256 feeUBA);

    /**
     * Agent can transfer their backing to core vault.
     * They then get a redemption requests which the owner pays just like any other redemption request.
     * After that, the agent's collateral is released.
     * NOTE: only agent vault owner can call
     * @param _agentVault the agent vault address
     * @param _amountUBA the amount to transfer to the core vault
     */
    function transferToCoreVault(address _agentVault, uint256 _amountUBA)
        external;

    /**
     * Request that core vault transfers funds to the agent's underlying address,
     * which makes them available for redemptions. This method reserves agent's collateral.
     * This may be sent by an agent when redemptions dominate mintings, so that the agents
     * are empty but want to earn from redemptions.
     * NOTE: only agent vault owner can call
     * NOTE: there can be only one active return request (until it is confirmed or cancelled).
     * @param _agentVault the agent vault address
     * @param _lots number of lots (same lots as for minting and redemptions)
     */
    function requestReturnFromCoreVault(address _agentVault, uint256 _lots)
        external;

    /**
     * Before the return request is processed, it can be cancelled, releasing the agent's reserved collateral.
     * @param _agentVault the agent vault address
     */
    function cancelReturnFromCoreVault(address _agentVault)
        external;

    /**
     * Confirm the payment from core vault to the agent's underlying address.
     * This adds the reserved funds to the agent's backing.
     * @param _payment FDC payment proof
     * @param _agentVault the agent vault address
     */
    function confirmReturnFromCoreVault(IPayment.Proof calldata _payment, address _agentVault)
        external;

    /**
     * Directly redeem from core vault by a user holding FAssets.
     * This is like ordinary redemption, but the redemption time is much longer (a day or more)
     * and there is no possibility of redemption default.
     * @param _lots the number of lots, must be larger than `coreVaultMinimumRedeemLots` setting
     * @param _redeemerUnderlyingAddress the underlying address to which the assets will be redeemed;
     *      must have been added to the `allowedDestinations` list in the core vault manager by
     *      the governance before the redemption request.
     */
    function redeemFromCoreVault(uint256 _lots, string memory _redeemerUnderlyingAddress)
        external;

    /**
     * Return the maximum amount that can be transferred and the minimum amount that
     * has to remain on the agent vault's underlying address.
     * @param _agentVault the agent vault address
     * @return _maximumTransferUBA maximum amount that can be transferred
     * @return _minimumLeftAmountUBA the minimum amount that has to remain on the agent vault's underlying address
     *  after the transfer
     */
    function maximumTransferToCoreVault(
        address _agentVault
    ) external view
        returns (uint256 _maximumTransferUBA, uint256 _minimumLeftAmountUBA);

    /**
     * Returns the amount available on the core vault - this is the maximum amount that can be returned to agent or
     * redeemed directly from the core vault.
     * @return _immediatelyAvailableUBA the amount on the core vault operating account - returns and redemptions
     * within this amount will be paid out quickly
     * @return _totalAvailableUBA the total amount on the core vault, including all escrows
     */
    function coreVaultAvailableAmount()
        external view
        returns (uint256 _immediatelyAvailableUBA, uint256 _totalAvailableUBA);
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import {IGovernanceSettings} from "@flarenetwork/flare-periphery-contracts/flare/IGovernanceSettings.sol";


interface IGoverned {

    error OnlyExecutor();
    error OnlyGovernance();
    error TimelockInvalidSelector();
    error TimelockNotAllowedYet();
    error AlreadyInProductionMode();
    error GovernedAlreadyInitialized();
    error GovernedAddressZero();

    /**
     * Governance call was timelocked. It can be executed after `allowedAfterTimestamp` by one of the executors.
     * @param encodedCall ABI encoded call data, to be used in executeGovernanceCall
     * @param encodedCallHash keccak256 hash of the ABI encoded call data
     * @param allowedAfterTimestamp the earliest timestamp when the call can be executed
     */
    event GovernanceCallTimelocked(bytes encodedCall, bytes32 encodedCallHash, uint256 allowedAfterTimestamp);

    /**
     * Previously timelocked governance call was executed.
     * @param encodedCallHash keccak256 hash of the ABI encoded call data
     *      (same as `GovernanceCallTimelocked.encodedCallHash`)
     */
    event TimelockedGovernanceCallExecuted(bytes32 encodedCallHash);

    /**
     * Previously timelocked governance call was canceled.
     * @param encodedCallHash keccak256 hash of the ABI encoded call data
     *      (same as `GovernanceCallTimelocked.encodedCallHash`)
     */
    event TimelockedGovernanceCallCanceled(bytes32 encodedCallHash);

    /**
     * Governed contract was initialised (not yet in production mode).
     * @param initialGovernance the governance address used until switch to production mode
     */
    event GovernanceInitialised(address initialGovernance);

    /**
     * The governed contract has switched to production mode
     * Timelocks are now enabled and the governance address is `governanceSettings.getGovernanceAddress()`.
     * @param governanceSettings the system contract holding governance address, timelock and executors settings
     */
    event GovernedProductionModeEntered(address governanceSettings);

    /**
     * @notice Execute the timelocked governance calls once the timelock period expires.
     * @dev Only executor can call this method.
     * @param _encodedCall ABI encoded call data (signature and parameters).
     *      You should use `encodedCall` parameter from `GovernanceCallTimelocked` event.
     */
    function executeGovernanceCall(bytes calldata _encodedCall) external;

    /**
     * Cancel a timelocked governance call before it has been executed.
     * @dev Only governance can call this method.
     * @param _encodedCall ABI encoded call data (signature and parameters).
     *      You should use `encodedCall` parameter from `GovernanceCallTimelocked` event.
     */
    function cancelGovernanceCall(bytes calldata _encodedCall) external;

    /**
     * Enter the production mode after all the initial governance settings have been set.
     * This enables timelocks and the governance is afterwards obtained by calling
     * `governanceSettings.getGovernanceAddress()`.
     */
    function switchToProductionMode() external;

    /**
     * Returns the governance settings contract address.
     */
    function governanceSettings() external view returns (IGovernanceSettings);

    /**
     * True after switching to production mode (see `switchToProductionMode()`).
     */
    function productionMode() external view returns (bool);

    /**
     * Returns the current effective governance address.
     * Before switching to production, the effective governance is `initialGovernance`,
     * and afterwards it is `governanceSettings.getGovernanceAddress()`.
     */
    function governance() external view returns (address);

    /**
     * Check if an address is one of the executors defined in `governanceSettings`.
     */
    function isExecutor(address _address) external view returns (bool);
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;


library SafeMath64 {
    uint256 internal constant MAX_UINT64 = type(uint64).max;
    int256 internal constant MAX_INT64 = type(int64).max;

    error ConversionOverflow();
    error NegativeValue();

    // 64 bit signed/unsigned conversion

    function toUint64(int256 a) internal pure returns (uint64) {
        require(a >= 0, NegativeValue());
        require(a <= int256(MAX_UINT64), ConversionOverflow());
        return uint64(uint256(a));
    }

    function toInt64(uint256 a) internal pure returns (int64) {
        require(a <= uint256(MAX_INT64), ConversionOverflow());
        return int64(int256(a));
    }

    function max64(uint64 a, uint64 b) internal pure returns (uint64) {
        return a >= b ? a : b;
    }

    function min64(uint64 a, uint64 b) internal pure returns (uint64) {
        return a <= b ? a : b;
    }
}
// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import "./IWeb2Json.sol";

interface IWeb2JsonVerification {
    function verifyJsonApi(
        IWeb2Json.Proof calldata _proof
    ) external view returns (bool _proved);
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
pragma solidity >=0.7.6 <0.9;


interface IAgentAlwaysAllowedMinters {
    function addAlwaysAllowedMinterForAgent(address _agentVault, address _minter)
        external;

    function removeAlwaysAllowedMinterForAgent(address _agentVault, address _minter)
        external;

    function alwaysAllowedMintersForAgent(address _agentVault)
        external view
        returns (address[] memory);
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;
pragma abicoder v2;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ICollateralPool} from "./ICollateralPool.sol";


interface IAgentVault {
    error OnlyOwner();
    error OnlyAssetManager();
    error AlreadyInitialized();
    error UnknownToken();
    error OnlyNonCollateralTokens();

    /**
     * Deposit vault collateral.
     * Parameter `_token` is explicit to allow depositing before collateral switch.
     * NOTE: owner must call `token.approve(vault, amount)` before calling this method.
     * NOTE: only the owner of the agent vault may call this method. If the agent wants to deposit from
     * some other wallet, he can just do `ERC20.transfer()` and then call `updateCollateral()`.
     */
    function depositCollateral(IERC20 _token, uint256 _amount) external;

    /**
     * Update collateral after `transfer(vault, some amount)` was called (alternative to depositCollateral).
     * Parameter `_token` is explicit to allow depositing before collateral switch.
     * NOTE: only the owner of the agent vault may call this method.
     */
    function updateCollateral(IERC20 _token) external;

    /**
     * Withdraw vault collateral. This method will work for any token, but for vault collateral and agent pool tokens
     * (which are locked because they may be backing f-assets) there is a check that there was prior announcement
     * by calling `assetManager.announceVaultCollateralWithdrawal(...)`.
     * NOTE: only the owner of the agent vault may call this method.
     */
    function withdrawCollateral(IERC20 _token, uint256 _amount, address _recipient) external;

    /**
     * Allow transferring a token, airdropped to the agent vault, to the owner (management address).
     * Doesn't work for vault collateral tokens or agent's pool tokens  because this would allow
     * withdrawing the locked collateral.
     * NOTE: only the owner of the agent vault may call this method.
     */
    function transferExternalToken(IERC20 _token, uint256 _amount) external;

    /**
     * Buy collateral pool tokens for NAT.
     * Holding enough pool tokens in the vault is required for minting.
     * NOTE: anybody can call this method, to allow the owner to deposit from any source.
     */
    function buyCollateralPoolTokens() external payable;

    /**
     * Collateral pool tokens which must be held by the agent accrue minting fees in form of f-assets.
     * These fees can be withdrawn using this method.
     * NOTE: only the owner of the agent vault may call this method.
     */
    function withdrawPoolFees(uint256 _amount, address _recipient) external;

    /**
     * This method allows the agent to convert collateral pool tokens back to NAT.
     * Prior announcement is required by calling `assetManager.announceAgentPoolTokenRedemption(...)`.
     * NOTE: only the owner of the agent vault may call this method.
     * NOTE: using unknown address as `_recipient` may make the caller vulnerable to gas wasting attacks
     * (but not reentrancy attacks). It is recommended that `_recipient` is one of the addresses controlled by
     * the agent vault owner, e.g. owner's management or work address.
     */
    function redeemCollateralPoolTokens(uint256 _amount, address payable _recipient) external;

    /**
     * Get the address of the collateral pool contract corresponding to this agent vault
     * (there is 1:1 correspondence between agent vault and collateral pools).
     */
    function collateralPool()
        external view
        returns (ICollateralPool);
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

import {AgentInfo} from "./AgentInfo.sol";

library AvailableAgentInfo {
    struct Data {
        // Agent vault address.
        address agentVault;
        // The management address of the agent vault's owner.
        address ownerManagementAddress;
        // Agent's minting fee in BIPS.
        uint256 feeBIPS;
        // Minimum agent vault collateral ratio needed for minting.
        uint256 mintingVaultCollateralRatioBIPS;
        // Minimum pool collateral ratio needed for minting.
        uint256 mintingPoolCollateralRatioBIPS;
        // The number of lots that can be minted by this agent.
        // Note: the value is only informative since it can can change at any time
        // due to price changes, reservation, minting, redemption, or even lot size change.
        uint256 freeCollateralLots;
        // The agent status, as for getAgentInfo().
        AgentInfo.Status status;
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
pragma solidity >=0.7.6 <0.9;


interface IICheckPointable {
    /**
     * @notice Total amount of tokens at a specific `_blockNumber`.
     * @param _blockNumber The block number when the totalSupply is queried
     * @return The total amount of tokens at `_blockNumber`
     **/
    function totalSupplyAt(uint256 _blockNumber) external view returns(uint256);

    /**
     * @dev Queries the token balance of `_owner` at a specific `_blockNumber`.
     * @param _owner The address from which the balance will be retrieved.
     * @param _blockNumber The block number when the balance is queried.
     * @return The balance at `_blockNumber`.
     **/
    function balanceOfAt(address _owner, uint256 _blockNumber) external view returns (uint256);
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

/**
 * @dev Compute percentages safely without phantom overflows.
 *
 * Intermediate operations can overflow even when the result will always
 * fit into computed type. Developers usually
 * assume that overflows raise errors. `SafePct` restores this intuition by
 * reverting the transaction when such an operation overflows.
 *
 * Using this library instead of the unchecked operations eliminates an entire
 * class of bugs, so it's recommended to use it always.
 */
library SafePct {
    uint256 internal constant MAX_BIPS = 10_000;

    error DivisionByZero();

    /**
     * Calculates `floor(x * y / z)`, reverting on overflow, but only if the result overflows.
     * Requirement: intermediate operations must revert on overflow.
     */
    function mulDiv(uint256 x, uint256 y, uint256 z) internal pure returns (uint256) {
        require(z > 0, DivisionByZero());

        if (x == 0) return 0;
        unchecked {
            uint256 xy = x * y;
            if (xy / x == y) { // no overflow happened (works in unchecked)
                return xy / z;
            }
        }

        //slither-disable-next-line divide-before-multiply
        uint256 a = x / z;
        uint256 b = x % z; // x = a * z + b

        //slither-disable-next-line divide-before-multiply
        uint256 c = y / z;
        uint256 d = y % z; // y = c * z + d

        return (a * c * z) + (a * d) + (b * c) + (b * d / z);
    }

    /**
     * Calculates `ceiling(x * y / z)`.
     */
    function mulDivRoundUp(uint256 x, uint256 y, uint256 z) internal pure returns (uint256) {
        uint256 resultRoundDown = mulDiv(x, y, z);
        unchecked {
            // safe - if z == 0, above mulDiv call would revert
            uint256 remainder = mulmod(x, y, z);
            // safe - overflow only possible if z == 1, but then remainder == 0
            return remainder == 0 ? resultRoundDown : resultRoundDown + 1;
        }
    }

    /**
     * Return `x * y BIPS` = `x * y / 10_000`, rounded down.
     */
    function mulBips(uint256 x, uint256 y) internal pure returns (uint256) {
        return mulDiv(x, y, MAX_BIPS);
    }
}
// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

/**
 * Core vault settings
 */
interface ICoreVaultClientSettings {
    function setCoreVaultManager(address _coreVaultManager)
        external;

    function setCoreVaultNativeAddress(address payable _nativeAddress)
        external;

    function setCoreVaultTransferTimeExtensionSeconds(uint256 _transferTimeExtensionSeconds)
        external;

    function setCoreVaultRedemptionFeeBIPS(uint256 _redemptionFeeBIPS)
        external;

    function setCoreVaultMinimumAmountLeftBIPS(uint256 _minimumAmountLeftBIPS)
        external;

    function setCoreVaultMinimumRedeemLots(uint256 _minimumRedeemLots)
        external;

    function getCoreVaultManager()
        external view
        returns (address);

    function getCoreVaultNativeAddress()
        external view
        returns (address);

    function getCoreVaultTransferTimeExtensionSeconds()
        external view
        returns (uint256);

    function getCoreVaultRedemptionFeeBIPS()
        external view
        returns (uint256);

    function getCoreVaultMinimumAmountLeftBIPS()
        external view
        returns (uint256);

    function getCoreVaultMinimumRedeemLots()
        external view
        returns (uint256);
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import {IVPToken} from "@flarenetwork/flare-periphery-contracts/flare/IVPToken.sol";

/**
 * @title Wrapped Native token
 * @notice Accept native token deposits and mint ERC20 WNAT (wrapped native) tokens 1-1.
 */
interface IWNat is IVPToken {
    /**
     * @notice Deposit Native and mint wNat ERC20.
     */
    function deposit() external payable;

    /**
     * @notice Deposit Native from msg.sender and mints WNAT ERC20 to recipient address.
     * @param recipient An address to receive minted WNAT.
     */
    function depositTo(address recipient) external payable;

    /**
     * @notice Withdraw Native and burn WNAT ERC20.
     * @param amount The amount to withdraw.
     */
    function withdraw(uint256 amount) external;

    /**
     * @notice Withdraw WNAT from an owner and send native tokens to msg.sender given an allowance.
     * @param owner An address spending the Native tokens.
     * @param amount The amount to spend.
     *
     * Requirements:
     *
     * - `owner` must have a balance of at least `amount`.
     * - the caller must have allowance for `owners`'s tokens of at least
     * `amount`.
     */
    function withdrawFrom(address owner, uint256 amount) external;
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;
pragma abicoder v2;

import {IWNat} from "../../flareSmartContracts/interfaces/IWNat.sol";


interface IISettingsManagement {
    function updateSystemContracts(address _controller, IWNat _wNat)
        external;

    function setAgentOwnerRegistry(address _value)
        external;

    function setAgentVaultFactory(address _value)
        external;

    function setCollateralPoolFactory(address _value)
        external;

    function setCollateralPoolTokenFactory(address _value)
        external;

    function setPriceReader(address _value)
        external;

    function setFdcVerification(address _value)
        external;

    function setCleanerContract(address _value)
        external;

    function setCleanupBlockNumberManager(address _value)
        external;

    function upgradeFAssetImplementation(address _value, bytes memory callData)
        external;

    function setTimeForPayment(uint256 _underlyingBlocks, uint256 _underlyingSeconds)
        external;

    function setPaymentChallengeReward(uint256 _rewardNATWei, uint256 _rewardBIPS)
        external;

    function setMinUpdateRepeatTimeSeconds(uint256 _value)
        external;

    function setLotSizeAmg(uint256 _value)
        external;

    function setMaxTrustedPriceAgeSeconds(uint256 _value)
        external;

    function setCollateralReservationFeeBips(uint256 _value)
        external;

    function setRedemptionFeeBips(uint256 _value)
        external;

    function setRedemptionDefaultFactorVaultCollateralBIPS(uint256 _value)
        external;

    function setConfirmationByOthersAfterSeconds(uint256 _value)
        external;

    function setConfirmationByOthersRewardUSD5(uint256 _value)
        external;

    function setMaxRedeemedTickets(uint256 _value)
        external;

    function setWithdrawalOrDestroyWaitMinSeconds(uint256 _value)
        external;

    function setAttestationWindowSeconds(uint256 _value)
        external;

    function setAverageBlockTimeMS(uint256 _value)
        external;

    function setMintingPoolHoldingsRequiredBIPS(uint256 _value)
        external;

    function setMintingCapAmg(uint256 _value)
        external;

    function setTokenInvalidationTimeMinSeconds(uint256 _value)
        external;

    function setVaultCollateralBuyForFlareFactorBIPS(uint256 _value)
        external;

    function setAgentExitAvailableTimelockSeconds(uint256 _value)
        external;

    function setAgentFeeChangeTimelockSeconds(uint256 _value)
        external;

    function setAgentMintingCRChangeTimelockSeconds(uint256 _value)
        external;

    function setPoolExitCRChangeTimelockSeconds(uint256 _value)
        external;

    function setAgentTimelockedOperationWindowSeconds(uint256 _value)
        external;

    function setCollateralPoolTokenTimelockSeconds(uint256 _value)
        external;

    function setLiquidationStepSeconds(uint256 _stepSeconds)
        external;

    function setLiquidationPaymentFactors(
        uint256[] memory _liquidationFactors,
        uint256[] memory _vaultCollateralFactors
    ) external;

    function setMaxEmergencyPauseDurationSeconds(uint256 _value)
        external;

    function setEmergencyPauseDurationResetAfterSeconds(uint256 _value)
        external;
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IWNat} from "../../flareSmartContracts/interfaces/IWNat.sol";
import {Reentrancy} from "../../openzeppelin/library/Reentrancy.sol";


library Transfers {
    uint256 internal constant TRANSFER_GAS_ALLOWANCE = 100_000;

    error TransferFailed();

    // make sure the transfer is only called in non-reentrant method
    modifier requireReentrancyGuard {
        Reentrancy.requireReentrancyGuard();
        _;
    }

    /**
     * Transfer the given amount of NAT to recipient without gas limit of `address.transfer()`.
     *
     * **Warning**: Must guard with nonReentrant, otherwise the method will fail.
     *
     * **Warning 2**: may fail, so only use when the top-level transaction sender controls recipient address
     * (and therefore expects to fail if there is something strange at that address).
     *
     * @param _recipient the recipient address
     * @param _amount the amount in NAT Wei
     */
    function transferNAT(address payable _recipient, uint256 _amount)
        internal
        requireReentrancyGuard
    {
        if (_amount > 0) {
            /* solhint-disable avoid-low-level-calls */
            //slither-disable-next-line arbitrary-send-eth
            (bool success, ) = _recipient.call{value: _amount, gas: TRANSFER_GAS_ALLOWANCE}("");
            /* solhint-enable avoid-low-level-calls */
            require(success, TransferFailed());
        }
    }

    /**
     * Deposits the given amount of NAT to recipient on WNat contract.
     *
     * @param _wNat the WNat contract address
     * @param _recipient the recipient address
     * @param _amount the amount in NAT Wei
     */
    function depositWNat(IWNat _wNat, address _recipient, uint256 _amount)
        internal
    {
        if (_amount > 0) {
            _wNat.depositTo{value: _amount}(_recipient);
        }
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

/**
 * @dev Compute percentages safely without phantom overflows.
 *
 * Intermediate operations can overflow even when the result will always
 * fit into computed type. Developers usually
 * assume that overflows raise errors. `SafePct` restores this intuition by
 * reverting the transaction when such an operation overflows.
 *
 * Using this library instead of the unchecked operations eliminates an entire
 * class of bugs, so it's recommended to use it always.
 */
library SafePct {
    uint256 internal constant MAX_BIPS = 10_000;

    error DivisionByZero();

    /**
     * Calculates `floor(x * y / z)`, reverting on overflow, but only if the result overflows.
     * Requirement: intermediate operations must revert on overflow.
     */
    function mulDiv(uint256 x, uint256 y, uint256 z) internal pure returns (uint256) {
        require(z > 0, DivisionByZero());

        if (x == 0) return 0;
        unchecked {
            uint256 xy = x * y;
            if (xy / x == y) { // no overflow happened (works in unchecked)
                return xy / z;
            }
        }

        //slither-disable-next-line divide-before-multiply
        uint256 a = x / z;
        uint256 b = x % z; // x = a * z + b

        //slither-disable-next-line divide-before-multiply
        uint256 c = y / z;
        uint256 d = y % z; // y = c * z + d

        return (a * c * z) + (a * d) + (b * c) + (b * d / z);
    }

    /**
     * Calculates `ceiling(x * y / z)`.
     */
    function mulDivRoundUp(uint256 x, uint256 y, uint256 z) internal pure returns (uint256) {
        uint256 resultRoundDown = mulDiv(x, y, z);
        unchecked {
            // safe - if z == 0, above mulDiv call would revert
            uint256 remainder = mulmod(x, y, z);
            // safe - overflow only possible if z == 1, but then remainder == 0
            return remainder == 0 ? resultRoundDown : resultRoundDown + 1;
        }
    }

    /**
     * Return `x * y BIPS` = `x * y / 10_000`, rounded down.
     */
    function mulBips(uint256 x, uint256 y) internal pure returns (uint256) {
        return mulDiv(x, y, MAX_BIPS);
    }
}
// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;
pragma abicoder v2;

/**
 * Agent owner management and work address management
 */
interface IAgentOwnerRegistry {

    event Whitelisted(address value);
    event WhitelistingRevoked(address value);

    /**
     * Agent owner's work address has been set.
     */
    event WorkAddressChanged(
        address indexed managementAddress,
        address prevWorkAddress,
        address workAddress);

    event AgentDataChanged(
        address indexed managementAddress,
        string name,
        string description,
        string iconUrl,
        string termsOfUseUrl);

    error AgentNotWhitelisted();
    error WorkAddressInUse();


    /**
     * Returns true if the address is whitelisted, false otherwise.
     * @param _address address to check
     */
    function isWhitelisted(address _address) external view returns (bool);

    /**
     * Return agent owner's name.
     * @param _managementAddress agent owner's management address
     */
    function getAgentName(address _managementAddress)
        external view
        returns (string memory);

    /**
     * Return agent owner's description.
     * @param _managementAddress agent owner's management address
     */
    function getAgentDescription(address _managementAddress)
        external view
        returns (string memory);

    /**
     * Return url of the agent owner's icon.
     * @param _managementAddress agent owner's management address
     */
    function getAgentIconUrl(address _managementAddress)
        external view
        returns (string memory);

    /**
     * Return url of the agent's page with terms of use.
     * @param _managementAddress agent owner's management address
     */
    function getAgentTermsOfUseUrl(address _managementAddress)
        external view
        returns (string memory);

    /**
     * Get the (unique) work address for the given management address.
     */
    function getWorkAddress(address _managementAddress)
        external view
        returns (address);

    /**
     * Get the (unique) management address for the given work address.
     */
    function getManagementAddress(address _workAddress)
        external view
        returns (address);
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IFdcVerification, IPayment, IBalanceDecreasingTransaction, IConfirmedBlockHeightExists,
        IReferencedPaymentNonexistence, IAddressValidity}
    from "@flarenetwork/flare-periphery-contracts/flare/IFdcVerification.sol";
import {AssetManagerSettings} from "../../userInterfaces/data/AssetManagerSettings.sol";
import {Globals} from "./Globals.sol";


library TransactionAttestation {

    // payment status constants
    uint8 internal constant PAYMENT_SUCCESS = 0;
    uint8 internal constant PAYMENT_FAILED = 1;
    uint8 internal constant PAYMENT_BLOCKED = 2;

    error PaymentFailed();
    error InvalidChain();
    error LegalPaymentNotProven();
    error TransactionNotProven();
    error BlockHeightNotProven();
    error NonPaymentNotProven();
    error AddressValidityNotProven();


    function verifyPaymentSuccess(
        IPayment.Proof calldata _proof
    )
        internal view
    {
        require(_proof.data.responseBody.status == PAYMENT_SUCCESS, PaymentFailed());
        verifyPayment(_proof);
    }

    function verifyPayment(
        IPayment.Proof calldata _proof
    )
        internal view
    {
        AssetManagerSettings.Data storage _settings = Globals.getSettings();
        IFdcVerification fdcVerification = IFdcVerification(_settings.fdcVerification);
        require(_proof.data.sourceId == _settings.chainId, InvalidChain());
        require(fdcVerification.verifyPayment(_proof), LegalPaymentNotProven());
    }

    function verifyBalanceDecreasingTransaction(
        IBalanceDecreasingTransaction.Proof calldata _proof
    )
        internal view
    {
        AssetManagerSettings.Data storage _settings = Globals.getSettings();
        IFdcVerification fdcVerification = IFdcVerification(_settings.fdcVerification);
        require(_proof.data.sourceId == _settings.chainId, InvalidChain());
        require(fdcVerification.verifyBalanceDecreasingTransaction(_proof), TransactionNotProven());
    }

    function verifyConfirmedBlockHeightExists(
        IConfirmedBlockHeightExists.Proof calldata _proof
    )
        internal view
    {
        AssetManagerSettings.Data storage _settings = Globals.getSettings();
        IFdcVerification fdcVerification = IFdcVerification(_settings.fdcVerification);
        require(_proof.data.sourceId == _settings.chainId, InvalidChain());
        require(fdcVerification.verifyConfirmedBlockHeightExists(_proof), BlockHeightNotProven());
    }

    function verifyReferencedPaymentNonexistence(
        IReferencedPaymentNonexistence.Proof calldata _proof
    )
        internal view
    {
        AssetManagerSettings.Data storage _settings = Globals.getSettings();
        IFdcVerification fdcVerification = IFdcVerification(_settings.fdcVerification);
        require(_proof.data.sourceId == _settings.chainId, InvalidChain());
        require(fdcVerification.verifyReferencedPaymentNonexistence(_proof), NonPaymentNotProven());
    }

    function verifyAddressValidity(
        IAddressValidity.Proof calldata _proof
    )
        internal view
    {
        AssetManagerSettings.Data storage _settings = Globals.getSettings();
        IFdcVerification fdcVerification = IFdcVerification(_settings.fdcVerification);
        require(_proof.data.sourceId == _settings.chainId, InvalidChain());
        require(fdcVerification.verifyAddressValidity(_proof), AddressValidityNotProven());
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
pragma solidity >=0.7.6 <0.9;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

library AgentSettings {
    struct Data {
        // The token used as vault collateral. Must be one of the tokens obtained by `getCollateralTypes()`,
        // with class VAULT.
        IERC20 vaultCollateralToken;
        // The suffix to pool token name and symbol that identifies new vault's collateral pool token.
        // Must be unique within an asset manager.
        string poolTokenSuffix;
        // Minting fee. Normally charged to minters for publicly available agents, but must be set
        // also for self-minting agents to pay part of it to collateral pool.
        // Fee is paid in underlying currency along with backing assets.
        uint256 feeBIPS;
        // Share of the minting fee that goes to the pool as percentage of the minting fee.
        // This share of fee is minted as f-assets and belongs to the pool.
        uint256 poolFeeShareBIPS;
        // Collateral ratio at which we calculate locked collateral and collateral available for minting.
        // Agent may set own value for minting collateral ratio on creation.
        // The value must always be greater than system minimum collateral ratio for vault collateral.
        // Warning: having this value near global min collateral ratio can quickly lead to liquidation for public
        // agents, so it is advisable to set it significantly higher.
        uint256 mintingVaultCollateralRatioBIPS;
        // Collateral ratio at which we calculate locked collateral and collateral available for minting.
        // Agent may set own value for minting collateral ratio on creation.
        // The value must always be greater than system minimum collateral ratio for pool collateral.
        // Warning: having this value near global min collateral ratio can quickly lead to liquidation for public
        // agents, so it is advisable to set it significantly higher.
        uint256 mintingPoolCollateralRatioBIPS;
        // The factor set by the agent to multiply the price at which agent buys f-assets from pool
        // token holders on self-close exit (when requested or the redeemed amount is less than 1 lot).
        uint256 buyFAssetByAgentFactorBIPS;
        // The minimum collateral ratio above which a staker can exit the pool
        // (this is CR that must be left after exit).
        // Must be higher than system minimum collateral ratio for pool collateral.
        uint256 poolExitCollateralRatioBIPS;
        // The redemption fee share paid to the pool (as FAssets).
        // In redemption dominated situations (when agent requests return from core vault to earn
        // from redemption fees), pool can get some share to make it sustainable for pool users.
        // NOTE: the pool fee share is locked at the redemption request time, but is charged at the redemption
        // confirmation time. If agent uses all the redemption fee for transaction fees, this could make the
        // agent's free underlying balance negative.
        uint256 redemptionPoolFeeShareBIPS;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import {IDiamondCut} from "../../diamond/interfaces/IDiamondCut.sol";
import {IGoverned} from "../../governance/interfaces/IGoverned.sol";
import {IAssetManager} from "../../userInterfaces/IAssetManager.sol";
import {IWNat} from "../../flareSmartContracts/interfaces/IWNat.sol";
import {IISettingsManagement} from "./IISettingsManagement.sol";
import {CollateralType} from "../../userInterfaces/data/CollateralType.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";


/**
 * Asset Manager methods used internally in AgentVault, CollateralPool and AssetManagerController.
 */
interface IIAssetManager is IAssetManager, IGoverned, IDiamondCut, IISettingsManagement {
    ////////////////////////////////////////////////////////////////////////////////////
    // Settings update

    /**
     * When `attached` is true, asset manager has been added to the asset manager controller.
     * Even though the asset manager controller address is set at the construction time, the manager may not
     * be able to be added to the controller immediately because the method addAssetManager must be called
     * by the governance multisig (with timelock). During this time it is impossible to verify through the
     * controller that the asset manager is legit.
     * Therefore creating agents and minting is disabled until the asset manager controller notifies
     * the asset manager that it has been added.
     * The `attached` can be set to false when the retired asset manager is removed from the controller.
     * NOTE: this method will be called automatically when the asset manager is added to a controller
     *      and cannot be called directly.
     */
    function attachController(bool attached) external;

    ////////////////////////////////////////////////////////////////////////////////////
    // Emergency pause

    /**
     * Trigger pause of most operations.
     */
    function emergencyPause(bool _byGovernance, uint256 _duration)
        external;

    /**
     * Reset total duration of 3rd party pauses, so that they can trigger pause again.
     * Otherwise, the total duration is automatically reset emergencyPauseDurationResetAfterSeconds after last pause.
     */
    function resetEmergencyPauseTotalDuration()
        external;

    /**
     * Emergency pause details, useful for monitors.
     */
    function emergencyPauseDetails()
        external view
        returns (uint256 _pausedUntil, uint256 _totalPauseDuration, bool _pausedByGovernance);

    ////////////////////////////////////////////////////////////////////////////////////
    // Emergency transfer pause

    /**
     * Trigger pause of most operations.
     */
    function emergencyPauseTransfers(bool _byGovernance, uint256 _duration)
        external;

    /**
     * Reset total duration of 3rd party pauses, so that they can trigger pause again.
     * Otherwise, the total duration is automatically reset emergencyPauseDurationResetAfterSeconds after last pause.
     */
    function resetEmergencyPauseTransfersTotalDuration()
        external;

    /**
     * Emergency pause details, useful for monitors.
     */
    function emergencyPauseTransfersDetails()
        external view
        returns (uint256 _pausedUntil, uint256 _totalPauseDuration, bool _pausedByGovernance);

    ////////////////////////////////////////////////////////////////////////////////////
    // Upgrade

    /**
     * When asset manager is paused, no new minting can be made.
     * All other operations continue normally.
     * NOTE: may not be called directly - only through asset manager controller by governance.
     */
    function pauseMinting() external;

    /**
     * Minting can continue.
     * NOTE: may not be called directly - only through asset manager controller by governance.
     */
    function unpauseMinting() external;

    /**
     * When agent vault, collateral pool or collateral pool token factory is upgraded, new agent vaults
     * automatically get the new implementation from the factory. The existing vaults can be batch updated
     * by this method.
     * Parameters `_start` and `_end` allow limiting the upgrades to a selection of all agents, to avoid
     * breaking the block gas limit.
     * NOTE: may not be called directly - only through asset manager controller by governance.
     * @param _start the start index of the list of agent vaults (in getAllAgents()) to upgrade
     * @param _end the end index (exclusive) of the list of agent vaults to upgrade;
     *  can be larger then the number of agents, if gas is not an issue
     */
    function upgradeAgentVaultsAndPools(
        uint256 _start,
        uint256 _end
    ) external;

    ////////////////////////////////////////////////////////////////////////////////////
    // Collateral type management

    /**
     * Add new vault collateral type (new token type and initial collateral ratios).
     * NOTE: may not be called directly - only through asset manager controller by governance.
     */
    function addCollateralType(
        CollateralType.Data calldata _data
    ) external;

    /**
     * Update collateral ratios for collateral type identified by `_collateralClass` and `_token`.
     * NOTE: may not be called directly - only through asset manager controller by governance.
     */
    function setCollateralRatiosForToken(
        CollateralType.Class _collateralClass,
        IERC20 _token,
        uint256 _minCollateralRatioBIPS,
        uint256 _safetyMinCollateralRatioBIPS
    ) external;

    /**
     * Deprecate collateral type identified by `_collateralClass` and `_token`.
     * After `_invalidationTimeSec` the collateral will become invalid and all the agents
     * that still use it as collateral will be liquidated.
     * NOTE: may not be called directly - only through asset manager controller by governance.
     */
    function deprecateCollateralType(
        CollateralType.Class _collateralClass,
        IERC20 _token,
        uint256 _invalidationTimeSec
    ) external;

    ////////////////////////////////////////////////////////////////////////////////////
    // Collateral pool redemptions

    /**
     * Create a redemption from a single agent. Used in self-close exit from the collateral pool.
     * NOTE: only collateral pool can call this method.
     */
    function redeemFromAgent(
        address _agentVault,
        address _receiver,
        uint256 _amountUBA,
        string memory _receiverUnderlyingAddress,
        address payable _executor
    ) external payable;

    /**
     * Burn fassets from  a single agent and get paid in vault collateral by the agent.
     * Price is FTSO price, multiplied by factor buyFAssetByAgentFactorBIPS (set by agent).
     * Used in self-close exit from the collateral pool when requested or when self-close amount is less than 1 lot.
     * NOTE: only collateral pool can call this method.
     */
    function redeemFromAgentInCollateral(
        address _agentVault,
        address _receiver,
        uint256 _amountUBA
    ) external;

    /**
     * To avoid unlimited work, the maximum number of redemption tickets closed in redemption, self close
     * or liquidation is limited. This means that a single redemption/self close/liquidation is limited.
     * This function calculates the maximum single redemption amount.
     */
    function maxRedemptionFromAgent(address _agentVault)
        external view
        returns (uint256);

    ////////////////////////////////////////////////////////////////////////////////////
    // Functions, used by agent vault during collateral deposit/withdraw

    /**
     * Called by AgentVault when agent calls `withdraw()`.
     * NOTE: may only be called from an agent vault, not from an EOA address.
     * @param _valueNATWei the withdrawn amount
     */
    function beforeCollateralWithdrawal(
        IERC20 _token,
        uint256 _valueNATWei
    ) external;

    /**
     * Called by AgentVault when there was a deposit.
     * May pull agent out of liquidation.
     * NOTE: may only be called from an agent vault or collateral pool, not from an EOA address.
     */
    function updateCollateral(
        address _agentVault,
        IERC20 _token
    ) external;

    ////////////////////////////////////////////////////////////////////////////////////
    // View functions used internally by agent vault and collateral pool.

    /**
     * Get current WNat contract set in the asset manager.
     * Used internally by agent vault and collateral pool.
     * @return WNat contract
     */
    function getWNat()
        external view
        returns (IWNat);

    /**
     * Returns price of asset (UBA) in NAT Wei as a fraction.
     * Used internally by collateral pool.
     */
    function assetPriceNatWei()
        external view
        returns (uint256 _multiplier, uint256 _divisor);

    /**
     * Returns the number of f-assets that the agent's pool identified by `_agentVault` is backing.
     * This is the same as the number of f-assets the agent is backing, but excluding
     * f-assets being redeemed by pool self-close redemptions.
     * Used internally by collateral pool.
     */
    function getFAssetsBackedByPool(address _agentVault)
        external view
        returns (uint256);

    /**
     * Returns the duration for which the collateral pool tokens are timelocked after minting.
     * Timelocking is done to battle sandwich attacks aimed at stealing newly deposited f-asset
     * fees from the pool.
     */
    function getCollateralPoolTokenTimelockSeconds()
        external view
        returns (uint256);

    /**
     * Check if `_token` is either vault collateral token for `_agentVault` or the pool token.
     * These types of tokens cannot be simply transferred from the agent vault, but can only be
     * withdrawn after announcement if they are not backing any f-assets.
     * Used internally by agent vault.
     */
    function isLockedVaultToken(address _agentVault, IERC20 _token)
        external view
        returns (bool);

    /**
     * Check if `_token` is any of the vault collateral tokens (including already invalidated).
     */
    function isVaultCollateralToken(IERC20 _token)
        external view
        returns (bool);

    /**
     * True if `_address` is either work or management address of the owner of the agent identified by `_agentVault`.
     * Used internally by agent vault.
     */
    function isAgentVaultOwner(address _agentVault, address _address)
        external view
        returns (bool);

    /**
     * Return the work address for the given management address.
     */
    function getWorkAddress(address _managementAddress)
        external view
        returns (address);
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;


library RedemptionTicketInfo {
    struct Data {
        // The id of the ticket, same as returned in RedemptionTicketCreated/Updated/Deleted events.
        uint256 redemptionTicketId;

        // Backing agent vault address.
        address agentVault;

        // The amount of FAsset on the ticket.
        uint256 ticketValueUBA;
    }
}

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

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;


library PaymentReference {
    uint256 private constant TYPE_SHIFT = 192;
    uint256 private constant TYPE_MASK = ((1 << 64) - 1) << TYPE_SHIFT;
    uint256 private constant LOW_BITS_MASK = (1 << TYPE_SHIFT) - 1;
    uint256 private constant ID_RANDOMIZATION = 1000;
    uint256 private constant MAX_ID = (1 << 64) - 1;

    // common prefix 0x464250526641 = hex('FBPRfA' - Flare Bridge Payment Reference / fAsset)

    uint256 internal constant MINTING = 0x4642505266410001 << TYPE_SHIFT;
    uint256 internal constant REDEMPTION = 0x4642505266410002 << TYPE_SHIFT;
    uint256 internal constant ANNOUNCED_WITHDRAWAL = 0x4642505266410003 << TYPE_SHIFT;
    uint256 internal constant RETURN_FROM_CORE_VAULT = 0x4642505266410004 << TYPE_SHIFT;
    uint256 internal constant REDEMPTION_FROM_CORE_VAULT = 0x4642505266410005 << TYPE_SHIFT;
    uint256 internal constant TOPUP = 0x4642505266410011 << TYPE_SHIFT;
    uint256 internal constant SELF_MINT = 0x4642505266410012 << TYPE_SHIFT;

    // create various payment references

    function minting(uint256 _id) internal pure returns (bytes32) {
        assert(_id <= MAX_ID);
        return bytes32(_id | MINTING);
    }

    function redemption(uint256 _id) internal pure returns (bytes32) {
        assert(_id <= MAX_ID);
        return bytes32(_id | REDEMPTION);
    }

    function announcedWithdrawal(uint256 _id) internal pure returns (bytes32) {
        assert(_id <= MAX_ID);
        return bytes32(_id | ANNOUNCED_WITHDRAWAL);
    }

    function returnFromCoreVault(uint256 _id) internal pure returns (bytes32) {
        assert(_id <= MAX_ID);
        return bytes32(_id | RETURN_FROM_CORE_VAULT);
    }

    function redemptionFromCoreVault(uint256 _id) internal pure returns (bytes32) {
        assert(_id <= MAX_ID);
        return bytes32(_id | REDEMPTION_FROM_CORE_VAULT);
    }

    function topup(address _agentVault) internal pure returns (bytes32) {
        return bytes32(uint256(uint160(_agentVault)) | TOPUP);
    }

    function selfMint(address _agentVault) internal pure returns (bytes32) {
        return bytes32(uint256(uint160(_agentVault)) | SELF_MINT);
    }

    // verify and decode payment references

    function isValid(bytes32 _reference, uint256 _type) internal pure returns (bool) {
        uint256 refType = uint256(_reference) & TYPE_MASK;
        uint256 refLowBits = uint256(_reference) & LOW_BITS_MASK;
        // for valid reference, type must match and low bits may never be 0 (are either id or address)
        return refType == _type && refLowBits != 0;
    }

    function decodeId(bytes32 _reference) internal pure returns (uint256) {
        return uint256(_reference) & LOW_BITS_MASK;
    }

    function randomizedIdSkip() internal view returns (uint64) {
        // This is rather weak randomization, but it's ok for the purpose of preventing speculative underlying
        // payments, since there is only one guess possible - the first mistake makes agent liquidated.
        //slither-disable-next-line weak-prng
        return uint64(block.number % ID_RANDOMIZATION + 1);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;


library Collateral {
    enum Kind {
        VAULT,   // vault collateral (tokens in in agent vault)
        POOL,           // pool collateral (NAT)
        AGENT_POOL      // agent's pool tokens (expressed in NAT) - only important for minting
    }

    struct Data {
        Kind kind;
        uint256 fullCollateral;
        uint256 amgToTokenWeiPrice;
    }

    struct CombinedData {
        Collateral.Data agentCollateral;
        Collateral.Data poolCollateral;
        Collateral.Data agentPoolTokens;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;


library SafeMath64 {
    uint256 internal constant MAX_UINT64 = type(uint64).max;
    int256 internal constant MAX_INT64 = type(int64).max;

    error ConversionOverflow();
    error NegativeValue();

    // 64 bit signed/unsigned conversion

    function toUint64(int256 a) internal pure returns (uint64) {
        require(a >= 0, NegativeValue());
        require(a <= int256(MAX_UINT64), ConversionOverflow());
        return uint64(uint256(a));
    }

    function toInt64(uint256 a) internal pure returns (int64) {
        require(a <= uint256(MAX_INT64), ConversionOverflow());
        return int64(int256(a));
    }

    function max64(uint64 a, uint64 b) internal pure returns (uint64) {
        return a >= b ? a : b;
    }

    function min64(uint64 a, uint64 b) internal pure returns (uint64) {
        return a <= b ? a : b;
    }
}
// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import "./IAddressValidity.sol";

interface IAddressValidityVerification {
    function verifyAddressValidity(
        IAddressValidity.Proof calldata _proof
    ) external view returns (bool _proved);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

library MathUtils {
    /**
     * Increases the value `x` to a whole multiple of `rounding`.
     */
    function roundUp(uint256 x, uint256 rounding) internal pure returns (uint256) {
        // division by 0 and overflow checks preformed by Solidity >= 0.8
        uint256 remainder = x % rounding;
        return remainder == 0 ? x : x - remainder + rounding;
    }

    /**
     * Return the positive part of `_a - _b`.
     */
    function subOrZero(uint256 _a, uint256 _b) internal pure returns (uint256) {
        return _a > _b ? _a - _b : 0;
    }

    /**
     * Returns _x if it is positive, otherwise 0.
     */
    function positivePart(int256 _x) internal pure returns (uint256) {
        return _x >= 0 ? uint256(_x) : 0;
    }

    /**
     * Returns `_a <= _b`; works correctly when `_b` is any signed value.
     */
    function mixedLTE(uint256 _a, int256 _b) internal pure returns (bool) {
        return _b >= 0 && _a <= uint256(_b);
    }

    /**
     * Returns `_a <= _b`; works correctly when `_a` is any signed value.
     */
    function mixedLTE(int256 _a, uint256 _b) internal pure returns (bool) {
        return _a <= 0 || uint256(_a) <= _b;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Math} from "@openzeppelin/contracts/utils/math/Math.sol";
import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {SafePct} from "../../utils/library/SafePct.sol";
import {MathUtils} from "../../utils/library/MathUtils.sol";
import {IAssetManagerEvents} from "../../userInterfaces/IAssetManagerEvents.sol";
import {Agents} from "./Agents.sol";
import {Conversion} from "./Conversion.sol";
import {AgentCollateral} from "./AgentCollateral.sol";
import {Agent} from "./data/Agent.sol";
import {Collateral} from "./data/Collateral.sol";
import {CollateralTypeInt} from "./data/CollateralTypeInt.sol";
import {CollateralTypes} from "./CollateralTypes.sol";


library Liquidation {
    using SafeCast for uint256;
    using MathUtils for uint256;
    using SafePct for uint256;
    using Agent for Agent.State;
    using Agents for Agent.State;

    struct CRData {
        uint256 vaultCR;
        uint256 poolCR;
        uint256 amgToC1WeiPrice;
        uint256 amgToPoolWeiPrice;
    }

    // Start full agent liquidation (Agent.Status.FULL_LIQUIDATION)
    function startFullLiquidation(
        Agent.State storage _agent
    )
        internal
    {
        // if already in full liquidation or destroying, do nothing
        if (_agent.status == Agent.Status.FULL_LIQUIDATION
            || _agent.status == Agent.Status.DESTROYING) return;
        if (_agent.liquidationStartedAt == 0) {
            _agent.liquidationStartedAt = block.timestamp.toUint64();
        }
        _agent.status = Agent.Status.FULL_LIQUIDATION;
        emit IAssetManagerEvents.FullLiquidationStarted(_agent.vaultAddress(), block.timestamp);
    }

    // Cancel liquidation if the agent is healthy.
    function endLiquidationIfHealthy(
        Agent.State storage _agent
    )
        internal
    {
        // can only stop plain liquidation (full liquidation can only stop when there are no more minted assets)
        if (_agent.status != Agent.Status.LIQUIDATION) return;
        // agent's current collateral ratio
        CRData memory cr = getCollateralRatiosBIPS(_agent);
        // target ratio is minCollateralRatioBIPS if collateral not underwater, otherwise safetyMinCollateralRatioBIPS
        uint256 targetRatioVaultCollateralBIPS = _targetRatioBIPS(_agent, Collateral.Kind.VAULT);
        uint256 targetRatioPoolBIPS = _targetRatioBIPS(_agent, Collateral.Kind.POOL);
        // if agent is safe, restore status to NORMAL
        if (cr.vaultCR >= targetRatioVaultCollateralBIPS && cr.poolCR >= targetRatioPoolBIPS) {
            _agent.status = Agent.Status.NORMAL;
            _agent.liquidationStartedAt = 0;
            _agent.collateralsUnderwater = 0;
            emit IAssetManagerEvents.LiquidationEnded(_agent.vaultAddress());
        }
    }

    function getCollateralRatiosBIPS(
        Agent.State storage _agent
    )
        internal view
        returns (CRData memory)
    {
        (uint256 vaultCR, uint256 amgToC1WeiPrice) = getCollateralRatioBIPS(_agent, Collateral.Kind.VAULT);
        (uint256 poolCR, uint256 amgToPoolWeiPrice) = getCollateralRatioBIPS(_agent, Collateral.Kind.POOL);
        return CRData({
            vaultCR: vaultCR,
            poolCR: poolCR,
            amgToC1WeiPrice: amgToC1WeiPrice,
            amgToPoolWeiPrice: amgToPoolWeiPrice
        });
    }

    // The collateral ratio (BIPS) for deciding whether agent is in liquidation is the maximum
    // of the ratio calculated from FTSO price and the ratio calculated from trusted voters' price.
    // In this way, liquidation due to bad FTSO providers bunching together is less likely.
    function getCollateralRatioBIPS(
        Agent.State storage _agent,
        Collateral.Kind _collateralKind
    )
        internal view
        returns (uint256 _collateralRatioBIPS, uint256 _amgToTokenWeiPrice)
    {
        (Collateral.Data memory _data, Collateral.Data memory _trustedData) =
            _collateralDataWithTrusted(_agent, _collateralKind);
        uint256 ratio = AgentCollateral.collateralRatioBIPS(_data, _agent);
        uint256 ratioTrusted = AgentCollateral.collateralRatioBIPS(_trustedData, _agent);
        _amgToTokenWeiPrice = _data.amgToTokenWeiPrice;
        _collateralRatioBIPS = Math.max(ratio, ratioTrusted);
    }

    // Calculate the amount of liquidation that gets agent to safety.
    // assumed: agentStatus == LIQUIDATION/FULL_LIQUIDATION
    function maxLiquidationAmountAMG(
        Agent.State storage _agent,
        uint256 _collateralRatioBIPS,
        uint256 _factorBIPS,
        Collateral.Kind _collateralKind
    )
        internal view
        returns (uint256)
    {
        // for full liquidation, all minted amount can be liquidated
        if (_agent.status == Agent.Status.FULL_LIQUIDATION) {
            return _agent.mintedAMG;
        }
        // otherwise, liquidate just enough to get agent to safety
        uint256 targetRatioBIPS = _targetRatioBIPS(_agent, _collateralKind);
        if (targetRatioBIPS <= _collateralRatioBIPS) {
            return 0;               // agent already safe
        }
        if (_collateralRatioBIPS <= _factorBIPS) {
            return _agent.mintedAMG; // cannot achieve target - liquidate all
        }
        uint256 maxLiquidatedAMG = AgentCollateral.totalBackedAMG(_agent, _collateralKind)
            .mulDivRoundUp(targetRatioBIPS - _collateralRatioBIPS, targetRatioBIPS - _factorBIPS);
        return Math.min(maxLiquidatedAMG, _agent.mintedAMG);
    }

    function _targetRatioBIPS(
        Agent.State storage _agent,
        Collateral.Kind _collateralKind
    )
        private view
        returns (uint256)
    {
        CollateralTypeInt.Data storage collateral = _agent.getCollateral(_collateralKind);
        if (!_agent.collateralUnderwater(_collateralKind)) {
            return collateral.minCollateralRatioBIPS;
        } else {
            return collateral.safetyMinCollateralRatioBIPS;
        }
    }

    // Used for calculating liquidation collateral ratio.
    function _collateralDataWithTrusted(
        Agent.State storage _agent,
        Collateral.Kind _kind
    )
        private view
        returns (Collateral.Data memory _data, Collateral.Data memory _trustedData)
    {
        CollateralTypeInt.Data storage collateral = _agent.getCollateral(_kind);
        uint256 fullCollateral = _getCollateralAmount(_agent, _kind, collateral);
        (uint256 price, uint256 trusted) = Conversion.currentAmgPriceInTokenWeiWithTrusted(collateral);
        _data = Collateral.Data({ kind: _kind, fullCollateral: fullCollateral, amgToTokenWeiPrice: price });
        _trustedData = Collateral.Data({ kind: _kind, fullCollateral: fullCollateral, amgToTokenWeiPrice: trusted });
    }

    function _getCollateralAmount(
        Agent.State storage _agent,
        Collateral.Kind _kind,
        CollateralTypeInt.Data storage collateral
    )
        private view
        returns (uint256)
    {
        if (!CollateralTypes.isValid(collateral)) {
            // A simple way to force agents still holding expired collateral tokens into liquidation is just to
            // set fullCollateral for expired types to 0.
            // This will also make sure all liquidation payments are in the other collateral type.
            return 0;
        } else if (_kind == Collateral.Kind.POOL) {
            // Return tracked collateral amount in the pool.
            return _agent.collateralPool.totalCollateral();
        } else {
            // Return amount of vault collateral.
            return collateral.token.balanceOf(_agent.vaultAddress());
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import {IConfirmedBlockHeightExists, IPayment, IAddressValidity, IReferencedPaymentNonexistence,
        IBalanceDecreasingTransaction}
    from "@flarenetwork/flare-periphery-contracts/flare/IFdcVerification.sol";
import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {IDiamondLoupe} from "../diamond/interfaces/IDiamondLoupe.sol";
import {AssetManagerSettings} from "./data/AssetManagerSettings.sol";
import {CollateralType} from "./data/CollateralType.sol";
import {AgentInfo} from "./data/AgentInfo.sol";
import {AgentSettings} from "./data/AgentSettings.sol";
import {AvailableAgentInfo} from "./data/AvailableAgentInfo.sol";
import {RedemptionTicketInfo} from "./data/RedemptionTicketInfo.sol";
import {RedemptionRequestInfo} from "./data/RedemptionRequestInfo.sol";
import {CollateralReservationInfo} from "./data/CollateralReservationInfo.sol";
import {IAssetManagerEvents} from "./IAssetManagerEvents.sol";
import {IAgentPing} from "./IAgentPing.sol";
import {IRedemptionTimeExtension} from "./IRedemptionTimeExtension.sol";
import {ICoreVaultClient} from "./ICoreVaultClient.sol";
import {ICoreVaultClientSettings} from "./ICoreVaultClientSettings.sol";
import {IAgentAlwaysAllowedMinters} from "./IAgentAlwaysAllowedMinters.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";


/**
 * Asset manager publicly callable methods.
 */
interface IAssetManager is
    IERC165,
    IDiamondLoupe,
    IAssetManagerEvents,
    IAgentPing,
    IRedemptionTimeExtension,
    ICoreVaultClient,
    ICoreVaultClientSettings,
    IAgentAlwaysAllowedMinters
{
    ////////////////////////////////////////////////////////////////////////////////////
    // Basic system information

    /**
     * Get the asset manager controller, the only address that can change settings.
     * Asset manager must be attached to the asset manager controller in the system contract registry.
     */
    function assetManagerController()
        external view
        returns (address);

    /**
     * Get the f-asset contract managed by this asset manager instance.
     */
    function fAsset()
        external view
        returns (IERC20);

    /**
     * Get the price reader contract used by this asset manager instance.
     */
    function priceReader()
        external view
        returns (address);

    /**
     * Return lot size in UBA (underlying base amount - smallest amount on underlying chain, e.g. satoshi).
     */
    function lotSize()
        external view
        returns (uint256 _lotSizeUBA);

    /**
     * Return asset minting granularity - smallest unit of f-asset stored internally
     * within this asset manager instance.
     */
    function assetMintingGranularityUBA()
        external view
        returns (uint256);

    /**
     * Return asset minting decimals - the number of decimals of precision for minting.

     */
    function assetMintingDecimals()
        external view
        returns (uint256);

    ////////////////////////////////////////////////////////////////////////////////////
    // System settings

    /**
     * Get complete current settings.
     * @return the current settings
     */
    function getSettings()
        external view
        returns (AssetManagerSettings.Data memory);

    /**
     * When `controllerAttached` is true, asset manager has been added to the asset manager controller.
     * This is required for the asset manager to be operational (create agent and minting don't work otherwise).
     */
    function controllerAttached()
        external view
        returns (bool);

    ////////////////////////////////////////////////////////////////////////////////////
    // Emergency pause

    /**
     * If true, the system is in emergency pause mode and most operations (mint, redeem, liquidate) are disabled.
     */
    function emergencyPaused()
        external view
        returns (bool);

    /**
     * The time when emergency pause mode will end automatically.
     */
    function emergencyPausedUntil()
        external view
        returns (uint256);

    ////////////////////////////////////////////////////////////////////////////////////
    // Emergency pause transfers

    /**
     * If true, the system is in emergency pause mode and most operations (mint, redeem, liquidate) are disabled.
     */
    function transfersEmergencyPaused()
        external view
        returns (bool);

    /**
     * The time when emergency pause mode will end automatically.
     */
    function transfersEmergencyPausedUntil()
        external view
        returns (uint256);

    ////////////////////////////////////////////////////////////////////////////////////
    // Asset manager upgrading state

    /**
     * True if the asset manager is paused.
     * In the paused state, minting is disabled, but all other operations (e.g. redemptions, liquidation) still work.
     * Paused asset manager can be later unpaused.
     */
    function mintingPaused()
        external view
        returns (bool);

    ////////////////////////////////////////////////////////////////////////////////////
    // Timekeeping for underlying chain

    /**
     * Prove that a block with given number and timestamp exists and
     * update the current underlying block info if the provided data is higher.
     * This method should be called by minters before minting and by agent's regularly
     * to prevent current block being too outdated, which gives too short time for
     * minting or redemption payment.
     * NOTE: anybody can call.
     * @param _proof proof that a block with given number and timestamp exists
     */
    function updateCurrentBlock(
        IConfirmedBlockHeightExists.Proof calldata _proof
    ) external;

    /**
     * Get block number and timestamp of the current underlying block known to the f-asset system.
     * @return _blockNumber current underlying block number tracked by asset manager
     * @return _blockTimestamp current underlying block timestamp tracked by asset manager
     * @return _lastUpdateTs the timestamp on this chain when the current underlying block was last updated
     */
    function currentUnderlyingBlock()
        external view
        returns (uint256 _blockNumber, uint256 _blockTimestamp, uint256 _lastUpdateTs);

    ////////////////////////////////////////////////////////////////////////////////////
    // Available collateral types

    /**
     * Get collateral  information about a token.
     */
    function getCollateralType(CollateralType.Class _collateralClass, IERC20 _token)
        external view
        returns (CollateralType.Data memory);

    /**
     * Get the list of all available and deprecated tokens used for collateral.
     */
    function getCollateralTypes()
        external view
        returns (CollateralType.Data[] memory);

    ////////////////////////////////////////////////////////////////////////////////////
    // Agent create / destroy

    /**
     * Create an agent vault.
     * The agent will always be identified by `_agentVault` address.
     * (Externally, one account may own several agent vaults,
     *  but in fasset system, each agent vault acts as an independent agent.)
     * NOTE: may only be called by an agent on the allowed agent list.
     * Can be called from the management or the work agent owner address.
     * @return _agentVault new agent vault address
     */
    function createAgentVault(
        IAddressValidity.Proof calldata _addressProof,
        AgentSettings.Data calldata _settings
    ) external
        returns (address _agentVault);

    /**
     * Announce that the agent is going to be destroyed. At this time, the agent must not have any mintings
     * or collateral reservations and must not be on the available agents list.
     * NOTE: may only be called by the agent vault owner.
     * @return _destroyAllowedAt the timestamp at which the destroy can be executed
     */
    function announceDestroyAgent(
        address _agentVault
    ) external
        returns (uint256 _destroyAllowedAt);

    /**
     * Delete all agent data, self destruct agent vault and send remaining collateral to the `_recipient`.
     * Procedure for destroying agent:
     * - exit available agents list
     * - wait until all assets are redeemed or perform self-close
     * - announce destroy (and wait the required time)
     * - call destroyAgent()
     * NOTE: may only be called by the agent vault owner.
     * NOTE: the remaining funds from the vault will be transferred to the provided recipient.
     * @param _agentVault address of the agent's vault to destroy
     * @param _recipient address that receives the remaining funds and possible vault balance
     */
    function destroyAgent(
        address _agentVault,
        address payable _recipient
    ) external;

    /**
     * When agent vault, collateral pool or collateral pool token factory is upgraded, new agent vaults
     * automatically get the new implementation from the factory. But the existing agent vaults must
     * be upgraded by their owners using this method.
     * NOTE: may only be called by the agent vault owner.
     * @param _agentVault address of the agent's vault; both vault, its corresponding pool, and
     *  its pool token will be upgraded to the newest implementations
     */
    function upgradeAgentVaultAndPool(
        address _agentVault
    ) external;

    /**
     * Check if the collateral pool token has been used already by some vault.
     * @param _suffix the suffix to check
     */
    function isPoolTokenSuffixReserved(
        string memory _suffix
    ) external view
        returns (bool);

    ////////////////////////////////////////////////////////////////////////////////////
    // Agent settings update

    /**
     * Due to the effect on the pool, all agent settings are timelocked.
     * This method announces a setting change. The change can be executed after the timelock expires.
     * NOTE: may only be called by the agent vault owner.
     * @param _agentVault agent vault address
     * @param _name setting name, same as for `getAgentSetting`
     * @return _updateAllowedAt the timestamp at which the update can be executed
     */
    function announceAgentSettingUpdate(
        address _agentVault,
        string memory _name,
        uint256 _value
    ) external
        returns (uint256 _updateAllowedAt);

    /**
     * Due to the effect on the pool, all agent settings are timelocked.
     * This method executes a setting change after the timelock expires.
     * NOTE: may only be called by the agent vault owner.
     * @param _agentVault agent vault address
     * @param _name setting name, same as for `getAgentSetting`
     */
    function executeAgentSettingUpdate(
        address _agentVault,
        string memory _name
    ) external;

    /**
     * If the current agent's vault collateral token gets deprecated, the agent must switch with this method.
     * NOTE: may only be called by the agent vault owner.
     * NOTE: at the time of switch, the agent must have enough of both collaterals in the vault.
     */
    function switchVaultCollateral(
        address _agentVault,
        IERC20 _token
    ) external;

    /**
     * When current pool collateral token contract (WNat) is replaced by the method setPoolWNatCollateralType,
     * pools don't switch automatically. Instead, the agent must call this method that swaps old WNat tokens for
     * new ones and sets it for use by the pool.
     * NOTE: may only be called by the agent vault owner.
     */
    function upgradeWNatContract(
        address _agentVault
    ) external;

    ////////////////////////////////////////////////////////////////////////////////////
    // Collateral withdrawal announcement

    /**
     * The agent is going to withdraw `_valueNATWei` amount of collateral from the agent vault.
     * This has to be announced and the agent must then wait `withdrawalWaitMinSeconds` time.
     * After that time, the agent can call `withdrawCollateral(_vaultCollateralToken, _valueNATWei)`
     * on the agent vault.
     * NOTE: may only be called by the agent vault owner.
     * @param _agentVault agent vault address
     * @param _valueNATWei the amount to be withdrawn
     * @return _withdrawalAllowedAt the timestamp when the withdrawal can be made
     */
    function announceVaultCollateralWithdrawal(
        address _agentVault,
        uint256 _valueNATWei
    ) external
        returns (uint256 _withdrawalAllowedAt);

    /**
     * The agent is going to redeem `_valueWei` collateral pool tokens in the agent vault.
     * This has to be announced and the agent must then wait `withdrawalWaitMinSeconds` time.
     * After that time, the agent can call `redeemCollateralPoolTokens(_valueNATWei)` on the agent vault.
     * NOTE: may only be called by the agent vault owner.
     * @param _agentVault agent vault address
     * @param _valueNATWei the amount to be withdrawn
     * @return _redemptionAllowedAt the timestamp when the redemption can be made
     */
    function announceAgentPoolTokenRedemption(
        address _agentVault,
        uint256 _valueNATWei
    ) external
        returns (uint256 _redemptionAllowedAt);

    ////////////////////////////////////////////////////////////////////////////////////
    // Underlying balance topup

    /**
     * When the agent tops up his underlying address, it has to be confirmed by calling this method,
     * which updates the underlying free balance value.
     * NOTE: may only be called by the agent vault owner.
     * @param _payment proof of the underlying payment; must include payment
     *      reference of the form `0x4642505266410011000...0<agents_vault_address>`
     * @param _agentVault agent vault address
     */
    function confirmTopupPayment(
        IPayment.Proof calldata _payment,
        address _agentVault
    ) external;

    ////////////////////////////////////////////////////////////////////////////////////
    // Underlying withdrawal announcements

    /**
     * Announce withdrawal of underlying currency.
     * In the event UnderlyingWithdrawalAnnounced the agent receives payment reference, which must be
     * added to the payment, otherwise it can be challenged as illegal.
     * Until the announced withdrawal is performed and confirmed or canceled, no other withdrawal can be announced.
     * NOTE: may only be called by the agent vault owner.
     * @param _agentVault agent vault address
     */
    function announceUnderlyingWithdrawal(
        address _agentVault
    ) external;

    /**
     * Agent must provide confirmation of performed underlying withdrawal, which updates free balance with used gas
     * and releases announcement so that a new one can be made.
     * If the agent doesn't call this method, anyone can call it after a time (`confirmationByOthersAfterSeconds`).
     * NOTE: may only be called by the owner of the agent vault
     *   except if enough time has passed without confirmation - then it can be called by anybody.
     * @param _payment proof of the underlying payment
     * @param _agentVault agent vault address
     */
    function confirmUnderlyingWithdrawal(
        IPayment.Proof calldata _payment,
        address _agentVault
    ) external;

    /**
     * Cancel ongoing withdrawal of underlying currency.
     * Needed in order to reset announcement timestamp, so that others cannot front-run the agent at
     * `confirmUnderlyingWithdrawal` call. This could happen if withdrawal would be performed more
     * than `confirmationByOthersAfterSeconds` seconds after announcement.
     * NOTE: may only be called by the agent vault owner.
     * @param _agentVault agent vault address
     */
    function cancelUnderlyingWithdrawal(
        address _agentVault
    ) external;

    ////////////////////////////////////////////////////////////////////////////////////
    // Agent information

    /**
     * Get (a part of) the list of all agents.
     * The list must be retrieved in parts since retrieving the whole list can consume too much gas for one block.
     * @param _start first index to return from the available agent's list
     * @param _end end index (one above last) to return from the available agent's list
     */
    function getAllAgents(uint256 _start, uint256 _end)
        external view
        returns (address[] memory _agents, uint256 _totalLength);

    /**
     * Return detailed info about an agent, typically needed by a minter.
     * @param _agentVault agent vault address
     * @return structure containing agent's minting fee (BIPS), min collateral ratio (BIPS),
     *      and current free collateral (lots)
     */
    function getAgentInfo(address _agentVault)
        external view
        returns (AgentInfo.Info memory);

    /**
     * Get agent's setting by name.
     * This allows reading individual settings.
     * @param _agentVault agent vault address
     * @param _name setting name, one of: `feeBIPS`, `poolFeeShareBIPS`, `redemptionPoolFeeShareBIPS`,
     *  `mintingVaultCollateralRatioBIPS`, `mintingPoolCollateralRatioBIPS`,`buyFAssetByAgentFactorBIPS`,
     *  `poolExitCollateralRatioBIPS`
     */
    function getAgentSetting(address _agentVault, string memory _name)
        external view
        returns (uint256);

    /**
     * Returns the collateral pool address of the agent identified by `_agentVault`.
     */
    function getCollateralPool(address _agentVault)
        external view
        returns (address);

    /**
     * Return the management address of the owner of the agent identified by `_agentVault`.
     */
    function getAgentVaultOwner(address _agentVault)
        external view
        returns (address _ownerManagementAddress);

    /**
     * Return vault collateral ERC20 token chosen by the agent identified by `_agentVault`.
     */
    function getAgentVaultCollateralToken(address _agentVault)
        external view
        returns (IERC20);

    /**
     * Return full vault collateral (free + locked) deposited in the vault `_agentVault`.
     */
    function getAgentFullVaultCollateral(address _agentVault)
        external view
        returns (uint256);

    /**
     * Return full pool NAT collateral (free + locked) deposited in the vault `_agentVault`.
     */
    function getAgentFullPoolCollateral(address _agentVault)
        external view
        returns (uint256);

    /**
     * Return the current liquidation factors and max liquidation amount of the agent
     * identified by `_agentVault`.
     */
    function getAgentLiquidationFactorsAndMaxAmount(address _agentVault)
        external view
        returns (
            uint256 liquidationPaymentFactorVaultBIPS,
            uint256 liquidationPaymentFactorPoolBIPS,
            uint256 maxLiquidationAmountUBA
        );

    /**
     * Return the minimum collateral ratio of the pool collateral owned by vault `_agentVault`.
     */
    function getAgentMinPoolCollateralRatioBIPS(address _agentVault)
        external view
        returns (uint256);

    /**
     * Return the minimum collateral ratio of the vault collateral owned by vault `_agentVault`.
     */
    function getAgentMinVaultCollateralRatioBIPS(address _agentVault)
        external view
        returns (uint256);

    ////////////////////////////////////////////////////////////////////////////////////
    // List of available agents (i.e. publicly available for minting).

    /**
     * Add the agent to the list of publicly available agents.
     * Other agents can only self-mint.
     * NOTE: may only be called by the agent vault owner.
     * @param _agentVault agent vault address
     */
    function makeAgentAvailable(
        address _agentVault
    ) external;

    /**
     * Announce exit from the publicly available agents list.
     * NOTE: may only be called by the agent vault owner.
     * @param _agentVault agent vault address
     * @return _exitAllowedAt the timestamp when the agent can exit
     */
    function announceExitAvailableAgentList(
        address _agentVault
    ) external
        returns (uint256 _exitAllowedAt);

    /**
     * Exit the publicly available agents list.
     * NOTE: may only be called by the agent vault owner and after announcement.
     * @param _agentVault agent vault address
     */
    function exitAvailableAgentList(
        address _agentVault
    ) external;

    /**
     * Get (a part of) the list of available agents.
     * The list must be retrieved in parts since retrieving the whole list can consume too much gas for one block.
     * @param _start first index to return from the available agent's list
     * @param _end end index (one above last) to return from the available agent's list
     */
    function getAvailableAgentsList(uint256 _start, uint256 _end)
        external view
        returns (address[] memory _agents, uint256 _totalLength);

    /**
     * Get (a part of) the list of available agents with extra information about agents' fee, min collateral ratio
     * and available collateral (in lots).
     * The list must be retrieved in parts since retrieving the whole list can consume too much gas for one block.
     * NOTE: agent's available collateral can change anytime due to price changes, minting, or changes
     * in agent's min collateral ratio, so it is only to be used as an estimate.
     * @param _start first index to return from the available agent's list
     * @param _end end index (one above last) to return from the available agent's list
     */
    function getAvailableAgentsDetailedList(uint256 _start, uint256 _end)
        external view
        returns (AvailableAgentInfo.Data[] memory _agents, uint256 _totalLength);

    ////////////////////////////////////////////////////////////////////////////////////
    // Minting

    /**
     * Before paying underlying assets for minting, minter has to reserve collateral and
     * pay collateral reservation fee. Collateral is reserved at ratio of agent's agentMinCollateralRatio
     * to requested lots NAT market price.
     * The minter receives instructions for underlying payment
     * (value, fee and payment reference) in event CollateralReserved.
     * Then the minter has to pay `value + fee` on the underlying chain.
     * If the minter pays the underlying amount, minter obtains f-assets.
     * The collateral reservation fee is split between the agent and the collateral pool.
     * NOTE: the owner of the agent vault must be in the AgentOwnerRegistry.
     * @param _agentVault agent vault address
     * @param _lots the number of lots for which to reserve collateral
     * @param _maxMintingFeeBIPS maximum minting fee (BIPS) that can be charged by the agent - best is just to
     *      copy current agent's published fee; used to prevent agent from front-running reservation request
     *      and increasing fee (that would mean that the minter would have to pay raised fee or forfeit
     *      collateral reservation fee)
     * @param _executor the account that is allowed to execute minting (besides minter and agent)
     */
    function reserveCollateral(
        address _agentVault,
        uint256 _lots,
        uint256 _maxMintingFeeBIPS,
        address payable _executor
    ) external payable
        returns (uint256 _collateralReservationId);

    /**
     * Return the collateral reservation fee amount that has to be passed to the `reserveCollateral` method.
     * NOTE: the amount paid may be larger than the required amount, but the difference is not returned.
     * It is advised that the minter pays the exact amount, but when the amount is so small that the revert
     * would cost more than the lost difference, the minter may want to send a slightly larger amount to compensate
     * for the possibility of a FTSO price change between obtaining this value and calling `reserveCollateral`.
     * @param _lots the number of lots for which to reserve collateral
     * @return _reservationFeeNATWei the amount of reservation fee in NAT wei
     */
    function collateralReservationFee(uint256 _lots)
        external view
        returns (uint256 _reservationFeeNATWei);

    /**
     * Returns the data about the collateral reservation for an ongoing minting.
     * Note: once the minting is executed or defaulted, the collateral reservation is deleted and this method fails.
     * @param _collateralReservationId the collateral reservation id, as used for executing or defaulting the minting
     */
    function collateralReservationInfo(uint256 _collateralReservationId)
        external view
        returns (CollateralReservationInfo.Data memory);

    /**
     * After obtaining proof of underlying payment, the minter calls this method to finish the minting
     * and collect the minted f-assets.
     * NOTE: may only be called by the minter (= creator of CR, the collateral reservation request),
     *   the executor appointed by the minter, or the agent owner (= owner of the agent vault in CR).
     * @param _payment proof of the underlying payment (must contain exact `value + fee` amount and correct
     *      payment reference)
     * @param _collateralReservationId collateral reservation id
     */
    function executeMinting(
        IPayment.Proof calldata _payment,
        uint256 _collateralReservationId
    ) external;

    /**
     * When the time for the minter to pay the underlying amount is over (i.e. the last underlying block has passed),
     * the agent can declare payment default. Then the agent collects the collateral reservation fee
     * (it goes directly to the vault), and the reserved collateral is unlocked.
     * NOTE: The attestation request must be done with `checkSourceAddresses=false`.
     * NOTE: may only be called by the owner of the agent vault in the collateral reservation request.
     * @param _proof proof that the minter didn't pay with correct payment reference on the underlying chain
     * @param _collateralReservationId id of a collateral reservation created by the minter
     */
    function mintingPaymentDefault(
        IReferencedPaymentNonexistence.Proof calldata _proof,
        uint256 _collateralReservationId
    ) external;

    /**
     * If a collateral reservation request exists for more than 24 hours, payment or non-payment proof are no longer
     * available. In this case the agent can call this method, which burns reserved collateral at market price
     * and releases the remaining collateral (CRF is also burned).
     * NOTE: may only be called by the owner of the agent vault in the collateral reservation request.
     * NOTE: the agent (management address) receives the vault collateral and NAT is burned instead. Therefore
     *      this method is `payable` and the caller must provide enough NAT to cover the received vault collateral
     *      amount multiplied by `vaultCollateralBuyForFlareFactorBIPS`.
     * @param _proof proof that the attestation query window can not not contain
     *      the payment/non-payment proof anymore
     * @param _collateralReservationId collateral reservation id
     */
    function unstickMinting(
        IConfirmedBlockHeightExists.Proof calldata _proof,
        uint256 _collateralReservationId
    ) external payable;

    /**
     * Agent can mint against himself.
     * This is a one-step process, skipping collateral reservation and collateral reservation fee payment.
     * Moreover, the agent doesn't have to be on the publicly available agents list to self-mint.
     * NOTE: may only be called by the agent vault owner.
     * NOTE: the caller must be a whitelisted agent.
     * @param _payment proof of the underlying payment; must contain payment reference of the form
     *      `0x4642505266410012000...0<agent_vault_address>`
     * @param _agentVault agent vault address
     * @param _lots number of lots to mint
     */
    function selfMint(
        IPayment.Proof calldata _payment,
        address _agentVault,
        uint256 _lots
    ) external;

    /**
     * If an agent has enough free underlying, they can mint immediately without any underlying payment.
     * This is a one-step process, skipping collateral reservation and collateral reservation fee payment.
     * Moreover, the agent doesn't have to be on the publicly available agents list to self-mint.
     * NOTE: may only be called by the agent vault owner.
     * NOTE: the caller must be a whitelisted agent.
     * @param _agentVault agent vault address
     * @param _lots number of lots to mint
     */
    function mintFromFreeUnderlying(
        address _agentVault,
        uint64 _lots
    ) external;

    ////////////////////////////////////////////////////////////////////////////////////
    // Redemption

    /**
     * Redeem (up to) `_lots` lots of f-assets. The corresponding amount of the f-assets belonging
     * to the redeemer will be burned and the redeemer will get paid by the agent in underlying currency
     * (or, in case of agent's payment default, by agent's collateral with a premium).
     * NOTE: in some cases not all sent f-assets can be redeemed (either there are not enough tickets or
     * more than a fixed limit of tickets should be redeemed). In this case only part of the approved assets
     * are burned and redeemed and the redeemer can execute this method again for the remaining lots.
     * In such a case the `RedemptionRequestIncomplete` event will be emitted, indicating the number
     * of remaining lots.
     * Agent receives redemption request id and instructions for underlying payment in
     * RedemptionRequested event and has to pay `value - fee` and use the provided payment reference.
     * @param _lots number of lots to redeem
     * @param _redeemerUnderlyingAddressString the address to which the agent must transfer underlying amount
     * @param _executor the account that is allowed to execute redemption default (besides redeemer and agent)
     * @return _redeemedAmountUBA the actual redeemed amount; may be less than requested if there are not enough
     *      redemption tickets available or the maximum redemption ticket limit is reached
     */
    function redeem(
        uint256 _lots,
        string memory _redeemerUnderlyingAddressString,
        address payable _executor
    ) external payable
        returns (uint256 _redeemedAmountUBA);

    /**
     * If the redeemer provides invalid address, the agent should provide the proof of address invalidity from the
     * Flare data connector. With this, the agent's obligations are fulfilled and they can keep the underlying.
     * NOTE: may only be called by the owner of the agent vault in the redemption request
     * NOTE: also checks that redeemer's address is normalized, so the redeemer must normalize their address,
     *   otherwise it will be rejected!
     * @param _proof proof that the address is invalid
     * @param _redemptionRequestId id of an existing redemption request
     */
    function rejectInvalidRedemption(
        IAddressValidity.Proof calldata _proof,
        uint256 _redemptionRequestId
    ) external;

    /**
     * After paying to the redeemer, the agent must call this method to unlock the collateral
     * and to make sure that the redeemer cannot demand payment in collateral on timeout.
     * The same method must be called for any payment status (SUCCESS, FAILED, BLOCKED).
     * In case of FAILED, it just releases the agent's underlying funds and the redeemer gets paid in collateral
     * after calling redemptionPaymentDefault.
     * In case of SUCCESS or BLOCKED, remaining underlying funds and collateral are released to the agent.
     * If the agent doesn't confirm payment in enough time (several hours, setting
     * `confirmationByOthersAfterSeconds`), anybody can do it and get rewarded from the agent's vault.
     * NOTE: may only be called by the owner of the agent vault in the redemption request
     *   except if enough time has passed without confirmation - then it can be called by anybody
     * @param _payment proof of the underlying payment (must contain exact `value - fee` amount and correct
     *      payment reference)
     * @param _redemptionRequestId id of an existing redemption request
     */
    function confirmRedemptionPayment(
        IPayment.Proof calldata _payment,
        uint256 _redemptionRequestId
    ) external;

    /**
     * If the agent doesn't transfer the redeemed underlying assets in time (until the last allowed block on
     * the underlying chain), the redeemer calls this method and receives payment in collateral (with some extra).
     * The agent can also call default if the redeemer is unresponsive, to payout the redeemer and free the
     * remaining collateral.
     * NOTE: The attestation request must be done with `checkSourceAddresses=false`.
     * NOTE: may only be called by the redeemer (= creator of the redemption request),
     *   the executor appointed by the redeemer,
     *   or the agent owner (= owner of the agent vault in the redemption request)
     * @param _proof proof that the agent didn't pay with correct payment reference on the underlying chain
     * @param _redemptionRequestId id of an existing redemption request
     */
    function redemptionPaymentDefault(
        IReferencedPaymentNonexistence.Proof calldata _proof,
        uint256 _redemptionRequestId
    ) external;

    /**
     * If the agent hasn't performed the payment, the agent can close the redemption request to free underlying funds.
     * It can be done immediately after the redeemer or agent calls `redemptionPaymentDefault`,
     * or this method can trigger the default payment without proof, but only after enough time has passed so that
     * attestation proof of non-payment is not available any more.
     * NOTE: may only be called by the owner of the agent vault in the redemption request.
     * @param _proof proof that the attestation query window can not not contain
     *      the payment/non-payment proof anymore
     * @param _redemptionRequestId id of an existing, but already defaulted, redemption request
     */
    function finishRedemptionWithoutPayment(
        IConfirmedBlockHeightExists.Proof calldata _proof,
        uint256 _redemptionRequestId
    ) external;

    /**
     * Returns the data about an ongoing redemption request.
     * Note: once the redemptions is confirmed, the request is deleted and this method fails.
     * However, if there is no payment and the redemption defaults, the method works and returns status DEFAULTED.
     * @param _redemptionRequestId the redemption request id, as used for confirming or defaulting the redemption
     */
    function redemptionRequestInfo(uint256 _redemptionRequestId)
        external view
        returns (RedemptionRequestInfo.Data memory);

    /**
     * Agent can "redeem against himself" by calling `selfClose`, which burns agent's own f-assets
     * and unlocks agent's collateral. The underlying funds backing the f-assets are released
     * as agent's free underlying funds and can be later withdrawn after announcement.
     * NOTE: may only be called by the agent vault owner.
     * @param _agentVault agent vault address
     * @param _amountUBA amount of f-assets to self-close
     * @return _closedAmountUBA the actual self-closed amount, may be less than requested if there are not enough
     *      redemption tickets available or the maximum redemption ticket limit is reached
     */
    function selfClose(
        address _agentVault,
        uint256 _amountUBA
    ) external
        returns (uint256 _closedAmountUBA);

    ////////////////////////////////////////////////////////////////////////////////////
    // Redemption queue info

    /**
     * Return (part of) the redemption queue.
     * @param _firstRedemptionTicketId the ticket id to start listing from; if 0, starts from the beginning
     * @param _pageSize the maximum number of redemption tickets to return
     * @return _queue the (part of) the redemption queue; maximum length is _pageSize
     * @return _nextRedemptionTicketId works as a cursor - if the _pageSize is reached and there are more tickets,
     *  it is the first ticket id not returned; if the end is reached, it is 0
     */
    function redemptionQueue(
        uint256 _firstRedemptionTicketId,
        uint256 _pageSize
    ) external view
        returns (RedemptionTicketInfo.Data[] memory _queue, uint256 _nextRedemptionTicketId);

    /**
     * Return (part of) the redemption queue for a specific agent.
     * @param _agentVault the agent vault address of the queried agent
     * @param _firstRedemptionTicketId the ticket id to start listing from; if 0, starts from the beginning
     * @param _pageSize the maximum number of redemption tickets to return
     * @return _queue the (part of) the redemption queue; maximum length is _pageSize
     * @return _nextRedemptionTicketId works as a cursor - if the _pageSize is reached and there are more tickets,
     *  it is the first ticket id not returned; if the end is reached, it is 0
     */
    function agentRedemptionQueue(
        address _agentVault,
        uint256 _firstRedemptionTicketId,
        uint256 _pageSize
    ) external view
        returns (RedemptionTicketInfo.Data[] memory _queue, uint256 _nextRedemptionTicketId);

    ////////////////////////////////////////////////////////////////////////////////////
    // Dust

    /**
     * Due to the minting pool fees or after a lot size change by the governance,
     * it may happen that less than one lot remains on a redemption ticket. This is named "dust" and
     * can be self closed or liquidated, but not redeemed. However, after several additions,
     * the total dust can amount to more than one lot. Using this method, the amount, rounded down
     * to a whole number of lots, can be converted to a new redemption ticket.
     * NOTE: we do NOT check that the caller is the agent vault owner, since we want to
     * allow anyone to convert dust to tickets to increase asset fungibility.
     * NOTE: dust above 1 lot is actually added to ticket at every minting, so this function need
     * only be called when the agent doesn't have any minting.
     * @param _agentVault agent vault address
     */
    function convertDustToTicket(
        address _agentVault
    ) external;

    ////////////////////////////////////////////////////////////////////////////////////
    // Liquidation

    /**
     * Checks that the agent's collateral is too low and if true, starts agent's liquidation.
     * If the agent is already in liquidation, returns the timestamp when liquidation started.
     * @param _agentVault agent vault address
     * @return _liquidationStartTs timestamp when liquidation started
     */
    function startLiquidation(
        address _agentVault
    ) external
        returns (uint256 _liquidationStartTs);

    /**
     * Burns up to `_amountUBA` f-assets owned by the caller and pays
     * the caller the corresponding amount of native currency with premium
     * (premium depends on the liquidation state).
     * If the agent isn't in liquidation yet, but satisfies conditions,
     * automatically puts the agent in liquidation status.
     * @param _agentVault agent vault address
     * @param _amountUBA the amount of f-assets to liquidate
     * @return _liquidatedAmountUBA liquidated amount of f-asset
     * @return _amountPaidVault amount paid to liquidator (in agent's vault collateral)
     * @return _amountPaidPool amount paid to liquidator (in NAT from pool)
     */
    function liquidate(
        address _agentVault,
        uint256 _amountUBA
    ) external
        returns (uint256 _liquidatedAmountUBA, uint256 _amountPaidVault, uint256 _amountPaidPool);

    /**
     * When the agent's collateral reaches the safe level during liquidation, the liquidation
     * process can be stopped by calling this method.
     * Full liquidation (i.e. the liquidation triggered by illegal underlying payment)
     * cannot be stopped.
     * NOTE: anybody can call.
     * NOTE: if the method succeeds, the agent's liquidation has ended.
     * @param _agentVault agent vault address
     */
    function endLiquidation(
        address _agentVault
    ) external;

    ////////////////////////////////////////////////////////////////////////////////////
    // Challenges

    /**
     * Called with a proof of payment made from the agent's underlying address, for which
     * no valid payment reference exists (valid payment references are from redemption and
     * underlying withdrawal announcement calls).
     * On success, immediately triggers full agent liquidation and rewards the caller.
     * @param _payment proof of a transaction from the agent's underlying address
     * @param _agentVault agent vault address
     */
    function illegalPaymentChallenge(
        IBalanceDecreasingTransaction.Proof calldata _payment,
        address _agentVault
    ) external;

    /**
     * Called with proofs of two payments made from the agent's underlying address
     * with the same payment reference (each payment reference is valid for only one payment).
     * On success, immediately triggers full agent liquidation and rewards the caller.
     * @param _payment1 proof of first payment from the agent's underlying address
     * @param _payment2 proof of second payment from the agent's underlying address
     * @param _agentVault agent vault address
     */
    function doublePaymentChallenge(
        IBalanceDecreasingTransaction.Proof calldata _payment1,
        IBalanceDecreasingTransaction.Proof calldata _payment2,
        address _agentVault
    ) external;

    /**
     * Called with proofs of several (otherwise legal) payments, which together make the agent's
     * underlying free balance negative (i.e. the underlying address balance is less than
     * the total amount of backed f-assets).
     * On success, immediately triggers full agent liquidation and rewards the caller.
     * @param _payments proofs of several distinct payments from the agent's underlying address
     * @param _agentVault agent vault address
     */
    function freeBalanceNegativeChallenge(
        IBalanceDecreasingTransaction.Proof[] calldata _payments,
        address _agentVault
    ) external;
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IIFAsset} from "../../fassetToken/interfaces/IIFAsset.sol";
import {IWNat} from "../../flareSmartContracts/interfaces/IWNat.sol";
import {AssetManagerSettings} from "../../userInterfaces/data/AssetManagerSettings.sol";
import {IAgentOwnerRegistry} from "../../userInterfaces/IAgentOwnerRegistry.sol";
import {AssetManagerState} from "./data/AssetManagerState.sol";
import {CollateralTypeInt} from "./data/CollateralTypeInt.sol";


// global state helpers
library Globals {
    bytes32 internal constant ASSET_MANAGER_SETTINGS_POSITION = keccak256("fasset.AssetManager.Settings");

    function getSettings()
        internal pure
        returns (AssetManagerSettings.Data storage _settings)
    {
        bytes32 position = ASSET_MANAGER_SETTINGS_POSITION;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            _settings.slot := position
        }
    }

    function getWNat()
        internal view
        returns (IWNat)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        return IWNat(address(state.collateralTokens[state.poolCollateralIndex].token));
    }

    function getPoolCollateral()
        internal view
        returns (CollateralTypeInt.Data storage)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        return state.collateralTokens[state.poolCollateralIndex];
    }

    function getFAsset()
        internal view
        returns (IIFAsset)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        return IIFAsset(settings.fAsset);
    }

    function getAgentOwnerRegistry()
        internal view
        returns (IAgentOwnerRegistry)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        return IAgentOwnerRegistry(settings.agentOwnerRegistry);
    }

    function getBurnAddress()
        internal view
        returns (address payable)
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        return settings.burnAddress;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import "./IBalanceDecreasingTransaction.sol";

interface IBalanceDecreasingTransactionVerification {
    function verifyBalanceDecreasingTransaction(
        IBalanceDecreasingTransaction.Proof calldata _proof
    ) external view returns (bool _proved);
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import "./IEVMTransaction.sol";

interface IEVMTransactionVerification {
    function verifyEVMTransaction(
        IEVMTransaction.Proof calldata _proof
    ) external view returns (bool _proved);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {SafePct} from "../../utils/library/SafePct.sol";
import {AssetManagerState} from "./data/AssetManagerState.sol";
import {IAssetManagerEvents} from "../../userInterfaces/IAssetManagerEvents.sol";
import {CollateralType} from "../../userInterfaces/data/CollateralType.sol";
import {CollateralTypeInt} from "./data/CollateralTypeInt.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {Conversion} from "./Conversion.sol";


library CollateralTypes {
    using SafeCast for uint256;

    error InvalidCollateralRatios();
    error CannotAddDeprecatedToken();
    error TokenAlreadyExists();
    error TokenZero();
    error PriceNotInitialized();
    error UnknownToken();
    error NotAVaultCollateral();
    error NotAPoolCollateralAtZero();
    error AtLeastTwoCollateralsRequired();

    function initialize(
        CollateralType.Data[] memory _data
    )
        internal
    {
        require(_data.length >= 2, AtLeastTwoCollateralsRequired());
        // initial pool collateral token
        require(_data[0].collateralClass == CollateralType.Class.POOL, NotAPoolCollateralAtZero());
        _add(_data[0]);
        _setPoolCollateralTypeIndex(0);
        // initial vault collateral tokens
        for (uint256 i = 1; i < _data.length; i++) {
            require(_data[i].collateralClass == CollateralType.Class.VAULT, NotAVaultCollateral());
            _add(_data[i]);
        }
    }

    function add(
        CollateralType.Data memory _data
    )
        internal
    {
        require(_data.collateralClass == CollateralType.Class.VAULT, NotAVaultCollateral());
        _add(_data);
    }

    function setPoolWNatCollateralType(
        CollateralType.Data memory _data
    )
        internal
    {
        uint256 index = _add(_data);
        _setPoolCollateralTypeIndex(index);
    }

    function getInfo(
        CollateralType.Class _collateralClass,
        IERC20 _token
    )
        internal view
        returns (CollateralType.Data memory)
    {
        CollateralTypeInt.Data storage token = CollateralTypes.get(_collateralClass, _token);
        return _getInfo(token);
    }

    function getAllInfos()
        internal view
        returns (CollateralType.Data[] memory _result)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        uint256 length = state.collateralTokens.length;
        _result = new CollateralType.Data[](length);
        for (uint256 i = 0; i < length; i++) {
            _result[i] = _getInfo(state.collateralTokens[i]);
        }
    }

    function get(
        CollateralType.Class _collateralClass,
        IERC20 _token
    )
        internal view
        returns (CollateralTypeInt.Data storage)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        uint256 index = state.collateralTokenIndex[_tokenKey(_collateralClass, _token)];
        require(index > 0, UnknownToken());
        return state.collateralTokens[index - 1];
    }

    function getIndex(
        CollateralType.Class _collateralClass,
        IERC20 _token
    )
        internal view
        returns (uint256)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        uint256 index = state.collateralTokenIndex[_tokenKey(_collateralClass, _token)];
        require(index > 0, UnknownToken());
        return index - 1;
    }

    function exists(
        CollateralType.Class _collateralClass,
        IERC20 _token
    )
        internal view
        returns (bool)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        uint256 index = state.collateralTokenIndex[_tokenKey(_collateralClass, _token)];
        return index > 0;
    }

    function isValid(CollateralTypeInt.Data storage _token)
        internal view
        returns (bool)
    {
        return _token.validUntil == 0 || _token.validUntil > block.timestamp;
    }

    function _add(CollateralType.Data memory _data) private returns (uint256) {
        AssetManagerState.State storage state = AssetManagerState.get();
        // validation of collateralClass is done before call to _add
        require(address(_data.token) != address(0), TokenZero());
        bytes32 tokenKey = _tokenKey(_data.collateralClass, _data.token);
        require(state.collateralTokenIndex[tokenKey] == 0, TokenAlreadyExists());
        require(_data.validUntil == 0, CannotAddDeprecatedToken());
        // validate collateral ratios
        bool ratiosValid =
            SafePct.MAX_BIPS < _data.minCollateralRatioBIPS &&
            _data.minCollateralRatioBIPS <= _data.safetyMinCollateralRatioBIPS;
        require(ratiosValid, InvalidCollateralRatios());
        // check that prices are initialized in FTSO price reader
        (uint256 assetPrice,,) = Conversion.readFtsoPrice(_data.assetFtsoSymbol, false);
        require(assetPrice != 0, PriceNotInitialized());
        if (!_data.directPricePair) {
            (uint256 tokenPrice,,) = Conversion.readFtsoPrice(_data.tokenFtsoSymbol, false);
            require(tokenPrice != 0, PriceNotInitialized());
        }
        // add token
        uint256 newTokenIndex = state.collateralTokens.length;
        state.collateralTokens.push(CollateralTypeInt.Data({
            token: _data.token,
            collateralClass: _data.collateralClass,
            decimals: _data.decimals.toUint8(),
            validUntil: _data.validUntil.toUint64(),
            directPricePair: _data.directPricePair,
            assetFtsoSymbol: _data.assetFtsoSymbol,
            tokenFtsoSymbol: _data.tokenFtsoSymbol,
            minCollateralRatioBIPS: _data.minCollateralRatioBIPS.toUint32(),
            __ccbMinCollateralRatioBIPS: 0, // no longer used
            safetyMinCollateralRatioBIPS: _data.safetyMinCollateralRatioBIPS.toUint32()
        }));
        state.collateralTokenIndex[tokenKey] = newTokenIndex + 1;   // 0 means empty
        emit IAssetManagerEvents.CollateralTypeAdded(uint8(_data.collateralClass), address(_data.token),
            _data.decimals, _data.directPricePair, _data.assetFtsoSymbol, _data.tokenFtsoSymbol,
            _data.minCollateralRatioBIPS, _data.safetyMinCollateralRatioBIPS);
        return newTokenIndex;
    }

    function _setPoolCollateralTypeIndex(uint256 _index) private {
        AssetManagerState.State storage state = AssetManagerState.get();
        CollateralTypeInt.Data storage token = state.collateralTokens[_index];
        assert(token.collateralClass == CollateralType.Class.POOL);
        state.poolCollateralIndex = _index.toUint16();
    }

    function _getInfo(CollateralTypeInt.Data storage token)
        private view
        returns (CollateralType.Data memory)
    {
        return CollateralType.Data({
            token: token.token,
            collateralClass: token.collateralClass,
            decimals: token.decimals,
            validUntil: token.validUntil,
            directPricePair: token.directPricePair,
            assetFtsoSymbol: token.assetFtsoSymbol,
            tokenFtsoSymbol: token.tokenFtsoSymbol,
            minCollateralRatioBIPS: token.minCollateralRatioBIPS,
            safetyMinCollateralRatioBIPS: token.safetyMinCollateralRatioBIPS
        });
    }

    function _tokenKey(
        CollateralType.Class _collateralClass,
        IERC20 _token
    )
        private pure
        returns (bytes32)
    {
        return bytes32((uint256(_collateralClass) << 160) | uint256(uint160(address(_token))));
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

/**
 * @dev Compute percentages safely without phantom overflows.
 *
 * Intermediate operations can overflow even when the result will always
 * fit into computed type. Developers usually
 * assume that overflows raise errors. `SafePct` restores this intuition by
 * reverting the transaction when such an operation overflows.
 *
 * Using this library instead of the unchecked operations eliminates an entire
 * class of bugs, so it's recommended to use it always.
 */
library SafePct {
    uint256 internal constant MAX_BIPS = 10_000;

    error DivisionByZero();

    /**
     * Calculates `floor(x * y / z)`, reverting on overflow, but only if the result overflows.
     * Requirement: intermediate operations must revert on overflow.
     */
    function mulDiv(uint256 x, uint256 y, uint256 z) internal pure returns (uint256) {
        require(z > 0, DivisionByZero());

        if (x == 0) return 0;
        unchecked {
            uint256 xy = x * y;
            if (xy / x == y) { // no overflow happened (works in unchecked)
                return xy / z;
            }
        }

        //slither-disable-next-line divide-before-multiply
        uint256 a = x / z;
        uint256 b = x % z; // x = a * z + b

        //slither-disable-next-line divide-before-multiply
        uint256 c = y / z;
        uint256 d = y % z; // y = c * z + d

        return (a * c * z) + (a * d) + (b * c) + (b * d / z);
    }

    /**
     * Calculates `ceiling(x * y / z)`.
     */
    function mulDivRoundUp(uint256 x, uint256 y, uint256 z) internal pure returns (uint256) {
        uint256 resultRoundDown = mulDiv(x, y, z);
        unchecked {
            // safe - if z == 0, above mulDiv call would revert
            uint256 remainder = mulmod(x, y, z);
            // safe - overflow only possible if z == 1, but then remainder == 0
            return remainder == 0 ? resultRoundDown : resultRoundDown + 1;
        }
    }

    /**
     * Return `x * y BIPS` = `x * y / 10_000`, rounded down.
     */
    function mulBips(uint256 x, uint256 y) internal pure returns (uint256) {
        return mulDiv(x, y, MAX_BIPS);
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;


library SafeMath64 {
    uint256 internal constant MAX_UINT64 = type(uint64).max;
    int256 internal constant MAX_INT64 = type(int64).max;

    error ConversionOverflow();
    error NegativeValue();

    // 64 bit signed/unsigned conversion

    function toUint64(int256 a) internal pure returns (uint64) {
        require(a >= 0, NegativeValue());
        require(a <= int256(MAX_UINT64), ConversionOverflow());
        return uint64(uint256(a));
    }

    function toInt64(uint256 a) internal pure returns (int64) {
        require(a <= uint256(MAX_INT64), ConversionOverflow());
        return int64(int256(a));
    }

    function max64(uint64 a, uint64 b) internal pure returns (uint64) {
        return a >= b ? a : b;
    }

    function min64(uint64 a, uint64 b) internal pure returns (uint64) {
        return a <= b ? a : b;
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/******************************************************************************\
* Author: Nick Mudge <nick@perfectabstractions.com> (https://twitter.com/mudgen)
* EIP-2535 Diamonds: https://eips.ethereum.org/EIPS/eip-2535
/******************************************************************************/

// A loupe is a small magnifying glass used to look at diamonds.
// These functions look at diamonds
interface IDiamondLoupe {
    /// These functions are expected to be called frequently
    /// by tools.

    struct Facet {
        address facetAddress;
        bytes4[] functionSelectors;
    }

    /// @notice Gets all facet addresses and their four byte function selectors.
    /// @return facets_ Facet
    function facets() external view returns (Facet[] memory facets_);

    /// @notice Gets all the function selectors supported by a specific facet.
    /// @param _facet The facet address.
    /// @return facetFunctionSelectors_
    function facetFunctionSelectors(address _facet) external view returns (bytes4[] memory facetFunctionSelectors_);

    /// @notice Get all the facet addresses used by a diamond.
    /// @return facetAddresses_
    function facetAddresses() external view returns (address[] memory facetAddresses_);

    /// @notice Gets the facet that supports the given selector.
    /// @dev If facet is not found return address(0).
    /// @param _functionSelector The function selector.
    /// @return facetAddress_ The facet address.
    function facetAddress(bytes4 _functionSelector) external view returns (address facetAddress_);
}

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

import {EnumerableSet} from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import {IICollateralPool} from "../../../collateralPool/interfaces/IICollateralPool.sol";


library Agent {
    error InvalidAgentVaultAddress();

    enum Status {
        EMPTY,              // agent does not exist
        NORMAL,
        LIQUIDATION,        // liquidation due to CR - ends when agent is healthy
        FULL_LIQUIDATION,   // illegal payment liquidation - must liquidate all and close vault
        DESTROYING,         // agent announced destroy, cannot mint again
        DESTROYED           // agent has been destroyed, cannot do anything except return info
    }

    // For agents to withdraw NAT collateral, they must first announce it and then wait
    // withdrawalAnnouncementSeconds.
    // The announced amount cannot be used as collateral for minting during that time.
    // This makes sure that agents cannot just remove all collateral if they are challenged.
    struct WithdrawalAnnouncement {
        // Announce amount in collateral token's minimum unit (wei).
        uint128 amountWei;

        // The timestamp when withdrawal can be executed.
        uint64 allowedAt;
    }

    // Struct to store agent's pending setting updates.
    struct SettingUpdate {
        uint128 value;
        uint64 validAt;
    }

    struct State {
        IICollateralPool collateralPool;

        // Address of the agent owner. This is the management address, which is immutable.
        // The work address can be retrieved from the global state mapping between
        // management and work addresses.
        address ownerManagementAddress;

        // Current underlying address for this agent vault.
        // The address is immutable.
        string underlyingAddressString;

        // `underlyingAddressString` is only used for sending the minter a correct payment address;
        // for matching payment addresses we always use `underlyingAddressHash = keccak256(underlyingAddressString)`
        bytes32 underlyingAddressHash;

        // Current status of the agent (changes for liquidation).
        Agent.Status status;

        // Index of collateral vault token.
        // The data is obtained as state.collateralTokens[vaultCollateralIndex].
        uint16 vaultCollateralIndex;

        // Index of token in collateral pool. This is always wrapped FLR/SGB, however the wrapping
        // contract (WNat) may change. In such case we add new collateral token with class POOL but the
        // agent must call a method to upgrade to new contract, se we must track the actual token used.
        uint16 poolCollateralIndex;

        // Position of this agent in the list of agents available for minting.
        // Value is actually `list index + 1`, so that 0 means 'not in the list'.
        uint32 availableAgentsPos;

        // Minting fee in BIPS (collected in underlying currency).
        uint16 feeBIPS;

        // Share of the minting fee that goes to the pool as percentage of the minting fee.
        uint16 poolFeeShareBIPS;

        // Collateral ratio at which we calculate locked collateral and collateral available for minting.
        // Agent may set own value for minting collateral ratio when entering the available agent list,
        // but it must always be greater than minimum collateral ratio.
        uint32 mintingVaultCollateralRatioBIPS;

        // Collateral ratio at which we calculate locked collateral and collateral available for minting.
        // Agent may set own value for minting collateral ratio when entering the available agent list,
        // but it must always be greater than minimum collateral ratio.
        uint32 mintingPoolCollateralRatioBIPS;

        // Timestamp of the startLiquidation (or liquidate) call.
        uint64 liquidationStartedAt;

        // Liquidation phase at the time when liquidation started.
        uint8 __initialLiquidationPhase; // only storage placeholder

        // Bitmap signifying which collateral type(s) triggered liquidation (LF_VAULT | LF_POOL).
        uint8 collateralsUnderwater;

        // Amount of collateral locked by collateral reservation.
        uint64 reservedAMG;

        // Amount of collateral backing minted fassets.
        uint64 mintedAMG;

        // The amount of fassets being redeemed. In this case, the fassets were already burned,
        // but the collateral must still be locked to allow payment in case of redemption failure.
        // The distinction between 'minted' and 'redeemed' assets is important in case of challenge.
        uint64 redeemingAMG;

        // The amount of fassets being redeemed EXCEPT those from pool self-close exits.
        // Unlike normal redemption, pool collateral was already withdrawn, so the redeeming collateral
        // must only be accounted for / locked for vault collateral.
        // On redemption payment failure, redeemer will be paid only in vault collateral in this case
        // (and will be paid less if there isn't enough - small extra risk for pool token holders).
        // There will always be `poolRedeemingAMG <= redeemingAMG`.
        uint64 poolRedeemingAMG;

        // When lot size changes, there may be some leftover after redemption that doesn't fit
        // a whole lot size. It is added to dustAMG and can be recovered via self-close.
        // Unlike redeemingAMG, dustAMG is still counted in the mintedAMG.
        uint64 dustAMG;

        // The amount of funds that on the agent's underlying address.
        // If it is higher than the amount needed to back mintings, it can be withdrawn after announcement.
        // It is signed int, because unreported deposits combined with other operations can in principle
        // make it negative. We could truncate it at 0, but if deposit report comes later, this would make
        // the value wrong.
        int128 underlyingBalanceUBA;

        // There can be only one announced underlying withdrawal per agent active at any time.
        // This variable holds the id, or 0 if there is no announced underlying withdrawal going on.
        uint64 announcedUnderlyingWithdrawalId;

        // The time when ongoing underlying withdrawal was announced.
        uint64 underlyingWithdrawalAnnouncedAt;

        // Announcement for vault collateral withdrawal.
        WithdrawalAnnouncement vaultCollateralWithdrawalAnnouncement;

        // Announcement for pool token withdrawal (which also means pool collateral withdrawal).
        WithdrawalAnnouncement poolTokenWithdrawalAnnouncement;

        // Underlying block when the agent was created.
        // Challenger's should track underlying address activity since this block
        // and topups are only valid after this block (both inclusive).
        uint64 underlyingBlockAtCreation;

        // The time when ongoing agent vault destroy was announced.
        uint64 destroyAllowedAt;

        // The factor set by the agent to multiply the price at which agent buys f-assets from pool
        // token holders on self-close exit (when requested or the redeemed amount is less than 1 lot).
        uint16 buyFAssetByAgentFactorBIPS;

        // The announced time when the agent is exiting available agents list.
        uint64 exitAvailableAfterTs;

        // The position of the agent in the list of all agents.
        uint32 allAgentsPos;

        // Agent's pending setting updates.
        mapping(bytes32 => SettingUpdate) settingUpdates;

        // Agent's handshake type - minting or redeeming can be rejected.
        // 0 - no verification, 1 - manual verification, ...
        uint32 __handshakeType; // only storage placeholder

        // There can only be one transfer to core vault per agent active at any time.
        uint64 activeTransferToCoreVault;

        // the request id of the active return from core vault
        uint64 activeReturnFromCoreVaultId;

        // part of the agent's reservedAMG for the core vault return
        uint64 returnFromCoreVaultReservedAMG;

        // The redemption fee share paid to the pool (as FAssets).
        // In redemption dominated situations (when agent requests return from core vault to earn
        // from redemption fees), pool can get some share to make it sustainable for pool users.
        // NOTE: the pool fee share is locked at the redemption request time, but is charged at the redemption
        // confirmation time. If agent uses all the redemption fee for transaction fees, this could make the
        // agent's free underlying balance negative.
        uint16 redemptionPoolFeeShareBIPS;

        EnumerableSet.AddressSet alwaysAllowedMinters;

        // Only used for calculating Agent.State size. See deleteStorage() below.
        uint256[1] _endMarker;
    }

    // underwater collateral classes
    uint8 internal constant LF_VAULT = 1 << 0;
    uint8 internal constant LF_POOL = 1 << 1;

    // diamond state accessors

    bytes32 internal constant AGENTS_POSITION = keccak256("fasset.AssetManager.Agent");

    // only return valid agent - fail if status is EMPTY or DESTROYED
    function get(address _address)
        internal view
        returns (Agent.State storage)
    {
        Agent.State storage agent = getWithoutCheck(_address);
        Agent.Status status = agent.status;
        require(status != Agent.Status.EMPTY && status != Agent.Status.DESTROYED, InvalidAgentVaultAddress());
        return agent;
    }

    // Like get, but only fail if status is EMPTY.
    // This is useful for reading agent info after the agent has been destroyed.
    function getAllowDestroyed(address _address)
        internal view
        returns (Agent.State storage)
    {
        Agent.State storage agent = getWithoutCheck(_address);
        require(agent.status != Agent.Status.EMPTY, InvalidAgentVaultAddress());
        return agent;
    }

    function getWithoutCheck(address _address)
        internal pure
        returns (Agent.State storage _agent)
    {
        bytes32 position = bytes32(uint256(AGENTS_POSITION) ^ (uint256(uint160(_address)) << 64));
        // solhint-disable-next-line no-inline-assembly
        assembly {
            _agent.slot := position
        }
    }

    function vaultAddress(Agent.State storage _agent)
        internal pure
        returns (address)
    {
        bytes32 position;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            position := _agent.slot
        }
        return address(uint160((uint256(position) ^ uint256(AGENTS_POSITION)) >> 64));
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

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IGovernanceVotePower} from "./IGovernanceVotePower.sol";
import {IVPContractEvents} from "./IVPContractEvents.sol";

interface IVPToken is IERC20 {
    /**
     * @notice Delegate by percentage `_bips` of voting power to `_to` from `msg.sender`.
     * @param _to The address of the recipient
     * @param _bips The percentage of voting power to be delegated expressed in basis points (1/100 of one percent).
     *   Not cumulative - every call resets the delegation value (and value of 0 undelegates `to`).
     **/
    function delegate(address _to, uint256 _bips) external;

    /**
     * @notice Undelegate all percentage delegations from the sender and then delegate corresponding
     *   `_bips` percentage of voting power from the sender to each member of `_delegatees`.
     * @param _delegatees The addresses of the new recipients.
     * @param _bips The percentages of voting power to be delegated expressed in basis points (1/100 of one percent).
     *   Total of all `_bips` values must be at most 10000.
     **/
    function batchDelegate(
        address[] memory _delegatees,
        uint256[] memory _bips
    ) external;

    /**
     * @notice Explicitly delegate `_amount` of voting power to `_to` from `msg.sender`.
     * @param _to The address of the recipient
     * @param _amount An explicit vote power amount to be delegated.
     *   Not cumulative - every call resets the delegation value (and value of 0 undelegates `to`).
     **/
    function delegateExplicit(address _to, uint _amount) external;

    /**
     * @notice Revoke all delegation from sender to `_who` at given block.
     *    Only affects the reads via `votePowerOfAtCached()` in the block `_blockNumber`.
     *    Block `_blockNumber` must be in the past.
     *    This method should be used only to prevent rogue delegate voting in the current voting block.
     *    To stop delegating use delegate/delegateExplicit with value of 0 or undelegateAll/undelegateAllExplicit.
     * @param _who Address of the delegatee
     * @param _blockNumber The block number at which to revoke delegation.
     */
    function revokeDelegationAt(address _who, uint _blockNumber) external;

    /**
     * @notice Undelegate all voting power for delegates of `msg.sender`
     *    Can only be used with percentage delegation.
     *    Does not reset delegation mode back to NOTSET.
     **/
    function undelegateAll() external;

    /**
     * @notice Undelegate all explicit vote power by amount delegates for `msg.sender`.
     *    Can only be used with explicit delegation.
     *    Does not reset delegation mode back to NOTSET.
     * @param _delegateAddresses Explicit delegation does not store delegatees' addresses,
     *   so the caller must supply them.
     * @return The amount still delegated (in case the list of delegates was incomplete).
     */
    function undelegateAllExplicit(
        address[] memory _delegateAddresses
    ) external returns (uint256);

    /**
     * @dev Should be compatible with ERC20 method
     */
    function name() external view returns (string memory);

    /**
     * @dev Should be compatible with ERC20 method
     */
    function symbol() external view returns (string memory);

    /**
     * @dev Should be compatible with ERC20 method
     */
    function decimals() external view returns (uint8);

    /**
     * @notice Total amount of tokens at a specific `_blockNumber`.
     * @param _blockNumber The block number when the totalSupply is queried
     * @return The total amount of tokens at `_blockNumber`
     **/
    function totalSupplyAt(uint _blockNumber) external view returns (uint256);

    /**
     * @dev Queries the token balance of `_owner` at a specific `_blockNumber`.
     * @param _owner The address from which the balance will be retrieved.
     * @param _blockNumber The block number when the balance is queried.
     * @return The balance at `_blockNumber`.
     **/
    function balanceOfAt(
        address _owner,
        uint _blockNumber
    ) external view returns (uint256);

    /**
     * @notice Get the current total vote power.
     * @return The current total vote power (sum of all accounts' vote powers).
     */
    function totalVotePower() external view returns (uint256);

    /**
     * @notice Get the total vote power at block `_blockNumber`
     * @param _blockNumber The block number at which to fetch.
     * @return The total vote power at the block  (sum of all accounts' vote powers).
     */
    function totalVotePowerAt(
        uint _blockNumber
    ) external view returns (uint256);

    /**
     * @notice Get the current vote power of `_owner`.
     * @param _owner The address to get voting power.
     * @return Current vote power of `_owner`.
     */
    function votePowerOf(address _owner) external view returns (uint256);

    /**
     * @notice Get the vote power of `_owner` at block `_blockNumber`
     * @param _owner The address to get voting power.
     * @param _blockNumber The block number at which to fetch.
     * @return Vote power of `_owner` at `_blockNumber`.
     */
    function votePowerOfAt(
        address _owner,
        uint256 _blockNumber
    ) external view returns (uint256);

    /**
     * @notice Get the vote power of `_owner` at block `_blockNumber`, ignoring revocation information (and cache).
     * @param _owner The address to get voting power.
     * @param _blockNumber The block number at which to fetch.
     * @return Vote power of `_owner` at `_blockNumber`. Result doesn't change if vote power is revoked.
     */
    function votePowerOfAtIgnoringRevocation(
        address _owner,
        uint256 _blockNumber
    ) external view returns (uint256);

    /**
     * @notice Get the delegation mode for '_who'. This mode determines whether vote power is
     *  allocated by percentage or by explicit value. Once the delegation mode is set,
     *  it never changes, even if all delegations are removed.
     * @param _who The address to get delegation mode.
     * @return delegation mode: 0 = NOTSET, 1 = PERCENTAGE, 2 = AMOUNT (i.e. explicit)
     */
    function delegationModeOf(address _who) external view returns (uint256);

    /**
     * @notice Get current delegated vote power `_from` delegator delegated `_to` delegatee.
     * @param _from Address of delegator
     * @param _to Address of delegatee
     * @return The delegated vote power.
     */
    function votePowerFromTo(
        address _from,
        address _to
    ) external view returns (uint256);

    /**
     * @notice Get delegated the vote power `_from` delegator delegated `_to` delegatee at `_blockNumber`.
     * @param _from Address of delegator
     * @param _to Address of delegatee
     * @param _blockNumber The block number at which to fetch.
     * @return The delegated vote power.
     */
    function votePowerFromToAt(
        address _from,
        address _to,
        uint _blockNumber
    ) external view returns (uint256);

    /**
     * @notice Compute the current undelegated vote power of `_owner`
     * @param _owner The address to get undelegated voting power.
     * @return The unallocated vote power of `_owner`
     */
    function undelegatedVotePowerOf(
        address _owner
    ) external view returns (uint256);

    /**
     * @notice Get the undelegated vote power of `_owner` at given block.
     * @param _owner The address to get undelegated voting power.
     * @param _blockNumber The block number at which to fetch.
     * @return The undelegated vote power of `_owner` (= owner's own balance minus all delegations from owner)
     */
    function undelegatedVotePowerOfAt(
        address _owner,
        uint256 _blockNumber
    ) external view returns (uint256);

    /**
     * @notice Get the vote power delegation `delegationAddresses`
     *  and `_bips` of `_who`. Returned in two separate positional arrays.
     * @param _who The address to get delegations.
     * @return _delegateAddresses Positional array of delegation addresses.
     * @return _bips Positional array of delegation percents specified in basis points (1/100 or 1 percent)
     * @return _count The number of delegates.
     * @return _delegationMode The mode of the delegation (NOTSET=0, PERCENTAGE=1, AMOUNT=2).
     */
    function delegatesOf(
        address _who
    )
        external
        view
        returns (
            address[] memory _delegateAddresses,
            uint256[] memory _bips,
            uint256 _count,
            uint256 _delegationMode
        );

    /**
     * @notice Get the vote power delegation `delegationAddresses`
     *  and `pcts` of `_who`. Returned in two separate positional arrays.
     * @param _who The address to get delegations.
     * @param _blockNumber The block for which we want to know the delegations.
     * @return _delegateAddresses Positional array of delegation addresses.
     * @return _bips Positional array of delegation percents specified in basis points (1/100 or 1 percent)
     * @return _count The number of delegates.
     * @return _delegationMode The mode of the delegation (NOTSET=0, PERCENTAGE=1, AMOUNT=2).
     */
    function delegatesOfAt(
        address _who,
        uint256 _blockNumber
    )
        external
        view
        returns (
            address[] memory _delegateAddresses,
            uint256[] memory _bips,
            uint256 _count,
            uint256 _delegationMode
        );

    /**
     * Returns VPContract used for readonly operations (view methods).
     * The only non-view method that might be called on it is `revokeDelegationAt`.
     *
     * @notice `readVotePowerContract` is almost always equal to `writeVotePowerContract`
     * except during upgrade from one VPContract to a new version (which should happen
     * rarely or never and will be anounced before).
     *
     * @notice You shouldn't call any methods on VPContract directly, all are exposed
     * via VPToken (and state changing methods are forbidden from direct calls).
     * This is the reason why this method returns `IVPContractEvents` - it should only be used
     * for listening to events (`Revoke` only).
     */
    function readVotePowerContract() external view returns (IVPContractEvents);

    /**
     * Returns VPContract used for state changing operations (non-view methods).
     * The only non-view method that might be called on it is `revokeDelegationAt`.
     *
     * @notice `writeVotePowerContract` is almost always equal to `readVotePowerContract`
     * except during upgrade from one VPContract to a new version (which should happen
     * rarely or never and will be anounced before). In the case of upgrade,
     * `writeVotePowerContract` will be replaced first to establish delegations, and
     * after some perio (e.g. after a reward epoch ends) `readVotePowerContract` will be set equal to it.
     *
     * @notice You shouldn't call any methods on VPContract directly, all are exposed
     * via VPToken (and state changing methods are forbidden from direct calls).
     * This is the reason why this method returns `IVPContractEvents` - it should only be used
     * for listening to events (`Delegate` and `Revoke` only).
     */
    function writeVotePowerContract() external view returns (IVPContractEvents);

    /**
     * When set, allows token owners to participate in governance voting
     * and delegate governance vote power.
     */
    function governanceVotePower() external view returns (IGovernanceVotePower);
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

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import "./RewardsV2Interface.sol";

/**
 * RewardManager interface.
 */
interface IRewardManager is RewardsV2Interface {
    /// Struct used for storing unclaimed reward data.
    struct UnclaimedRewardState {
        bool initialised; // Information if already initialised
        // amount and weight might be 0 if all users already claimed
        uint120 amount; // Total unclaimed amount.
        uint128 weight; // Total unclaimed weight.
    }

    /**
     * Emitted when rewards are claimed.
     * @param beneficiary Address of the beneficiary (voter or node id) that accrued the reward.
     * @param rewardOwner Address that was eligible for the rewards.
     * @param recipient Address that received the reward.
     * @param rewardEpochId Id of the reward epoch where the reward was accrued.
     * @param claimType Claim type
     * @param amount Amount of rewarded native tokens (wei).
     */
    event RewardClaimed(
        address indexed beneficiary,
        address indexed rewardOwner,
        address indexed recipient,
        uint24 rewardEpochId,
        ClaimType claimType,
        uint120 amount
    );

    /**
     * Unclaimed rewards have expired and are now inaccessible.
     *
     * `getUnclaimedRewardState()` can be used to retrieve more information.
     * @param rewardEpochId Id of the reward epoch that has just expired.
     */
    event RewardClaimsExpired(uint256 indexed rewardEpochId);

    /**
     * Emitted when reward claims have been enabled.
     * @param rewardEpochId First claimable reward epoch.
     */
    event RewardClaimsEnabled(uint256 indexed rewardEpochId);

    /**
     * Claim rewards for `_rewardOwners` and their PDAs.
     * Rewards are deposited to the WNAT (to reward owner or PDA if enabled).
     * It can be called by reward owner or its authorized executor.
     * Only claiming from weight based claims is supported.
     * @param _rewardOwners Array of reward owners.
     * @param _rewardEpochId Id of the reward epoch up to which the rewards are claimed.
     * @param _proofs Array of reward claims with merkle proofs.
     */
    function autoClaim(
        address[] calldata _rewardOwners,
        uint24 _rewardEpochId,
        RewardClaimWithProof[] calldata _proofs
    ) external;

    /**
     * Initialises weight based claims.
     * @param _proofs Array of reward claims with merkle proofs.
     */
    function initialiseWeightBasedClaims(
        RewardClaimWithProof[] calldata _proofs
    ) external;

    /**
     * Returns the reward manager id.
     */
    function rewardManagerId() external view returns (uint256);

    /**
     * Returns the number of weight based claims that have been initialised.
     * @param _rewardEpochId Reward epoch id.
     */
    function noOfInitialisedWeightBasedClaims(
        uint256 _rewardEpochId
    ) external view returns (uint256);

    /**
     * Get the current cleanup block number.
     * @return The currently set cleanup block number.
     */
    function cleanupBlockNumber() external view returns (uint256);

    /**
     * Returns the state of rewards for a given address at a specific reward epoch.
     * @param _rewardOwner Address of the reward owner.
     * @param _rewardEpochId Reward epoch id.
     * @return _rewardStates Array of reward states.
     */
    function getStateOfRewardsAt(
        address _rewardOwner,
        uint24 _rewardEpochId
    ) external view returns (RewardState[] memory _rewardStates);

    /**
     * Gets the unclaimed reward state for a beneficiary, reward epoch id and claim type.
     * @param _beneficiary Address of the beneficiary to query.
     * @param _rewardEpochId Id of the reward epoch to query.
     * @param _claimType Claim type to query.
     * @return _state Unclaimed reward state.
     */
    function getUnclaimedRewardState(
        address _beneficiary,
        uint24 _rewardEpochId,
        ClaimType _claimType
    ) external view returns (UnclaimedRewardState memory _state);

    /**
     * Returns totals.
     * @return _totalRewardsWei Total rewards (wei).
     * @return _totalInflationRewardsWei Total inflation rewards (wei).
     * @return _totalClaimedWei Total claimed rewards (wei).
     * @return _totalBurnedWei Total burned rewards (wei).
     */
    function getTotals()
        external
        view
        returns (
            uint256 _totalRewardsWei,
            uint256 _totalInflationRewardsWei,
            uint256 _totalClaimedWei,
            uint256 _totalBurnedWei
        );

    /**
     * Returns reward epoch totals.
     * @param _rewardEpochId Reward epoch id.
     * @return _totalRewardsWei Total rewards (inflation + community) for the epoch (wei).
     * @return _totalInflationRewardsWei Total inflation rewards for the epoch (wei).
     * @return _initialisedRewardsWei Initialised rewards of all claim types for the epoch (wei).
     * @return _claimedRewardsWei Claimed rewards for the epoch (wei).
     * @return _burnedRewardsWei Burned rewards for the epoch (wei).
     */
    function getRewardEpochTotals(
        uint24 _rewardEpochId
    )
        external
        view
        returns (
            uint256 _totalRewardsWei,
            uint256 _totalInflationRewardsWei,
            uint256 _initialisedRewardsWei,
            uint256 _claimedRewardsWei,
            uint256 _burnedRewardsWei
        );

    /**
     * Returns current reward epoch id.
     */
    function getCurrentRewardEpochId() external view returns (uint24);

    /**
     * Returns initial reward epoch id.
     */
    function getInitialRewardEpochId() external view returns (uint256);

    /**
     * Returns the reward epoch id that will expire next once a new reward epoch starts.
     */
    function getRewardEpochIdToExpireNext() external view returns (uint256);

    /**
     * The first reward epoch id that was claimable.
     */
    function firstClaimableRewardEpochId() external view returns (uint24);
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

interface IICleanable {
    /**
     * Set the contract that is allowed to call history cleaning methods.
     */
    function setCleanerContract(address _cleanerContract) external;

    /**
     * Set the cleanup block number.
     * Historic data for the blocks before `cleanupBlockNumber` can be erased,
     * history before that block should never be used since it can be inconsistent.
     * In particular, cleanup block number must be before current vote power block.
     * @param _blockNumber The new cleanup block number.
     */
    function setCleanupBlockNumber(uint256 _blockNumber) external;

    /**
     * Get the current cleanup block number.
     */
    function cleanupBlockNumber() external view returns (uint256);
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;
pragma abicoder v2;

interface IDistributionToDelegators {
    // Events
    event UseGoodRandomSet(
        bool useGoodRandom,
        uint256 maxWaitForGoodRandomSeconds
    );
    event EntitlementStart(uint256 entitlementStartTs);
    event AccountClaimed(
        address indexed whoClaimed,
        address indexed sentTo,
        uint256 month,
        uint256 amountWei
    );
    event AccountOptOut(address indexed theAccount, bool confirmed);

    // Methods
    /**
     * @notice Allows the sender to claim or wrap rewards for reward owner.
     * @notice The caller does not have to be the owner, but must be approved by the owner to claim on his behalf,
     *   this approval is done by calling `setClaimExecutors`.
     * @notice It is actually safe for this to be called by anybody (nothing can be stolen), but by limiting who can
     *   call, we allow the owner to control the timing of the calls.
     * @notice Reward owner can claim to any `_recipient`, while the executor can only claim to the reward owner,
     *   reward owners's personal delegation account or one of the addresses set by `setAllowedClaimRecipients`.
     * @param _rewardOwner          address of the reward owner
     * @param _recipient            address to transfer funds to
     * @param _month                last month to claim for
     * @param _wrap                 should reward be wrapped immediately
     * @return _rewardAmount        amount of total claimed rewards
     */
    function claim(
        address _rewardOwner,
        address _recipient,
        uint256 _month,
        bool _wrap
    ) external returns (uint256 _rewardAmount);

    /**
     * @notice Allows batch claiming for the list of '_rewardOwners' up to given '_month'.
     * @notice If reward owner has enabled delegation account, rewards are also claimed for that delegation account and
     *   total claimed amount is sent to that delegation account, otherwise claimed amount is sent to owner's account.
     * @notice Claimed amount is automatically wrapped.
     * @notice Method can be used by reward owner or executor. If executor is registered with fee > 0,
     *   then fee is paid to executor for each claimed address from the list.
     * @param _rewardOwners         list of reward owners to claim for
     * @param _month                last month to claim for
     */
    function autoClaim(
        address[] calldata _rewardOwners,
        uint256 _month
    ) external;

    /**
     * @notice Method to opt-out of receiving airdrop rewards
     */
    function optOutOfAirdrop() external;

    /**
     * @notice Returns the next claimable month for '_rewardOwner'.
     * @param _rewardOwner          address of the reward owner
     */
    function nextClaimableMonth(
        address _rewardOwner
    ) external view returns (uint256);

    /**
     * @notice get claimable amount of wei for requesting account for specified month
     * @param _month month of interest
     * @return _amountWei amount of wei available for this account and provided month
     */
    function getClaimableAmount(
        uint256 _month
    ) external view returns (uint256 _amountWei);

    /**
     * @notice get claimable amount of wei for account for specified month
     * @param _account the address of an account we want to get the claimable amount of wei
     * @param _month month of interest
     * @return _amountWei amount of wei available for provided account and month
     */
    function getClaimableAmountOf(
        address _account,
        uint256 _month
    ) external view returns (uint256 _amountWei);

    /**
     * @notice Returns the current month
     * @return _currentMonth Current month, 0 before entitlementStartTs
     */
    function getCurrentMonth() external view returns (uint256 _currentMonth);

    /**
     * @notice Returns the month that will expire next
     * @return _monthToExpireNext Month that will expire next, 36 when last month expired
     */
    function getMonthToExpireNext()
        external
        view
        returns (uint256 _monthToExpireNext);

    /**
     * @notice Returns claimable months - reverts if none
     * @return _startMonth first claimable month
     * @return _endMonth last claimable month
     */
    function getClaimableMonths()
        external
        view
        returns (uint256 _startMonth, uint256 _endMonth);
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

/**
 * @custom:name IReferencedPaymentNonexistence
 * @custom:id 0x04
 * @custom:supported BTC, DOGE, XRP
 * @author Flare
 * @notice Assertion that an agreed-upon payment has not been made by a certain deadline.
 * A confirmed request shows that a transaction meeting certain criteria (address, amount, reference)
 * did not appear in the specified block range.
 *
 *
 * This type of attestation can be used to e.g. provide grounds to liquidate funds locked by a smart
 * contract on Flare when a payment is missed.
 *
 * @custom:verification If `firstOverflowBlock` cannot be determined or does not have a sufficient
 * number of confirmations, the attestation request is rejected.
 * If `firstOverflowBlockNumber` is higher or equal to `minimalBlockNumber`, the request is rejected.
 * The search range are blocks between heights including `minimalBlockNumber` and excluding `firstOverflowBlockNumber`.
 * If the verifier does not have a view of all blocks from `minimalBlockNumber` to `firstOverflowBlockNumber`,
 * the attestation request is rejected.
 *
 * The request is confirmed if no transaction meeting the specified criteria is found in the search range.
 * The criteria and timestamp are chain specific.
 * ### UTXO (Bitcoin and Dogecoin)
 *
 *
 * Criteria for the transaction:
 *
 *
 * - It is not coinbase transaction.
 * - The transaction has the specified standardPaymentReference.
 * - The sum of values of all outputs with the specified address minus the sum of values of all inputs with
 * the specified address is greater than `amount` (in practice the sum of all values of the inputs with the
 * specified address is zero).
 *
 *
 * Timestamp is `mediantime`.
 * ### XRPL
 *
 *
 *
 * Criteria for the transaction:
 * - The transaction is of type payment.
 * - The transaction has the specified standardPaymentReference,
 * - One of the following is true:
 *   - Transaction status is `SUCCESS` and the amount received by the specified destination address is
 * greater than the specified `value`.
 *   - Transaction status is `RECEIVER_FAILURE` and the specified destination address would receive an
 * amount greater than the specified `value` had the transaction been successful.
 *
 *
 * Timestamp is `close_time` converted to UNIX time.
 *
 * @custom:lut `minimalBlockTimestamp`
 * @custom:lutlimit `0x127500`, `0x127500`, `0x127500`
 */
interface IReferencedPaymentNonexistence {
    /**
     * @notice Toplevel request
     * @param attestationType ID of the attestation type.
     * @param sourceId ID of the data source.
     * @param messageIntegrityCode `MessageIntegrityCode` that is derived from the expected response as defined.
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
     * @param lowestUsedTimestamp The lowest timestamp used to generate the response.
     * @param requestBody Extracted from the request.
     * @param responseBody Data defining the response. The verification rules for the construction of the response
     * body and the type are defined per specific `attestationType`.
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
     * @notice Request body for ReferencePaymentNonexistence attestation type
     * @param minimalBlockNumber The start block of the search range.
     * @param deadlineBlockNumber The blockNumber to be included in the search range.
     * @param deadlineTimestamp The timestamp to be included in the search range.
     * @param destinationAddressHash The standard address hash of the address to which the payment had to be done.
     * @param amount The requested amount in minimal units that had to be payed.
     * @param standardPaymentReference The requested standard payment reference.
     * @param checkSourceAddresses If true, the source address root is checked (only full match).
     * @param sourceAddressesRoot The root of the Merkle tree of the source addresses.
     * @custom:below The `standardPaymentReference` should not be zero (as a 32-byte sequence).
     */
    struct RequestBody {
        uint64 minimalBlockNumber;
        uint64 deadlineBlockNumber;
        uint64 deadlineTimestamp;
        bytes32 destinationAddressHash;
        uint256 amount;
        bytes32 standardPaymentReference;
        bool checkSourceAddresses;
        bytes32 sourceAddressesRoot;
    }

    /**
     * @notice Response body for ReferencePaymentNonexistence attestation type.
     * @param minimalBlockTimestamp The timestamp of the minimalBlock.
     * @param firstOverflowBlockNumber The height of the firstOverflowBlock.
     * @param firstOverflowBlockTimestamp The timestamp of the firstOverflowBlock.
     * @custom:below `firstOverflowBlock` is the first block that has block number higher than
     * `deadlineBlockNumber` and timestamp later than `deadlineTimestamp`.
     * The specified search range are blocks between heights including `minimalBlockNumber`
     * and excluding `firstOverflowBlockNumber`.
     */
    struct ResponseBody {
        uint64 minimalBlockTimestamp;
        uint64 firstOverflowBlockNumber;
        uint64 firstOverflowBlockTimestamp;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

/**
 * @custom:name IAddressValidity
 * @custom:id 0x05
 * @custom:supported BTC, DOGE, XRP
 * @author Flare
 * @notice An assertion whether a string represents a valid address on an external chain.
 * @custom:verification The address is checked against all validity criteria of the chain with `sourceId`.
 * Indicator of validity is provided.
 * If the address is valid, its standard form and standard hash are computed.
 * Validity criteria for each supported chain:
 * - [BTC](/specs/attestations/external-chains/address-validity/BTC.md)
 * - [DOGE](/specs/attestations/external-chains/address-validity/DOGE.md)
 * - [XRPL](/specs/attestations/external-chains/address-validity/XRPL.md)
 * @custom:lut `0xffffffffffffffff` ($2^{64}-1$ in hex)
 * @custom:lutlimit `0xffffffffffffffff`, `0xffffffffffffffff`, `0xffffffffffffffff`
 */
interface IAddressValidity {
    /**
     * @notice Toplevel request
     * @param attestationType ID of the attestation type.
     * @param sourceId Id of the data source.
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
     * @notice Request body for IAddressValidity attestation type
     * @param addressStr Address to be verified.
     */
    struct RequestBody {
        string addressStr;
    }

    /**
     * @notice Response body for IAddressValidity attestation type
     * @param isValid Boolean indicator of the address validity.
     * @param standardAddress If `isValid`, standard form of the validated address. Otherwise an empty string.
     * @param standardAddressHash If `isValid`, standard address hash of the validated address.
     * Otherwise a zero bytes32 string.
     */
    struct ResponseBody {
        bool isValid;
        string standardAddress;
        bytes32 standardAddressHash;
    }
}

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
 * @custom:name IConfirmedBlockHeightExists
 * @custom:id 0x02
 * @custom:supported BTC, DOGE, XRP
 * @author Flare
 * @notice An assertion that a block with `blockNumber` is confirmed.
 * It also provides data to compute the block production rate in the given time range.
 * @custom:verification It is checked that the block with `blockNumber` is confirmed by at
 * least `numberOfConfirmations`.
 * If it is not, the request is rejected. We note a block on the tip of the chain is confirmed by 1 block.
 * Then `lowestQueryWindowBlock` is determined and its number and timestamp are extracted.
 *
 *
 * Current confirmation heights consensus:
 *
 *
 * | `Chain` | `chainId` | `numberOfConfirmations` | `timestamp ` |
 * | ------- | --------- | ----------------------- | ------------ |
 * | `BTC`   | 0         | 6                       | mediantime   |
 * | `DOGE`  | 2         | 60                      | mediantime   |
 * | `XRP`   | 3         | 3                       | close_time   |
 *
 *
 * @custom:lut `lowestQueryWindowBlockTimestamp`
 * @custom:lutlimit `0x127500`, `0x127500`, `0x127500`
 */
interface IConfirmedBlockHeightExists {
    /**
     * @notice Toplevel request
     * @param attestationType ID of the attestation type.
     * @param sourceId ID of the data source.
     * @param messageIntegrityCode `MessageIntegrityCode` that is derived from the expected response as defined.
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
     * @notice Request body for ConfirmedBlockHeightExistsType attestation type
     * @param blockNumber The number of the block the request wants a confirmation of.
     * @param queryWindow The length of the period in which the block production rate is to be computed.
     */
    struct RequestBody {
        uint64 blockNumber;
        uint64 queryWindow;
    }

    /**
     * @notice Response body for ConfirmedBlockHeightExistsType attestation type
     * @custom:below `blockNumber`, `lowestQueryWindowBlockNumber`, `blockTimestamp`, `lowestQueryWindowBlockTimestamp`
     * can be used to compute the average block production time in the specified block range.
     * @param blockTimestamp The timestamp of the block with `blockNumber`.
     * @param numberOfConfirmations The depth at which a block is considered confirmed depending on the chain.
     * All attestation providers must agree on this number.
     * @param lowestQueryWindowBlockNumber The block number of the latest block that has a timestamp strictly smaller
     * than `blockTimestamp` - `queryWindow`.
     * @param lowestQueryWindowBlockTimestamp The timestamp of the block at height `lowestQueryWindowBlockNumber`.
     */
    struct ResponseBody {
        uint64 blockTimestamp;
        uint64 numberOfConfirmations;
        uint64 lowestQueryWindowBlockNumber;
        uint64 lowestQueryWindowBlockTimestamp;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import "../../IVPToken.sol";
import "../../IGovernanceVotePower.sol";
import "./IIVPContract.sol";
import "./IIGovernanceVotePower.sol";
import "./IICleanable.sol";

interface IIVPToken is IVPToken, IICleanable {
    /**
     * Set the contract that is allowed to set cleanupBlockNumber.
     * Usually this will be an instance of CleanupBlockNumberManager.
     */
    function setCleanupBlockNumberManager(
        address _cleanupBlockNumberManager
    ) external;

    /**
     * Sets new governance vote power contract that allows token owners to participate in governance voting
     * and delegate governance vote power.
     */
    function setGovernanceVotePower(
        IIGovernanceVotePower _governanceVotePower
    ) external;

    /**
     * @notice Get the total vote power at block `_blockNumber` using cache.
     *   It tries to read the cached value and if not found, reads the actual value and stores it in cache.
     *   Can only be used if `_blockNumber` is in the past, otherwise reverts.
     * @param _blockNumber The block number at which to fetch.
     * @return The total vote power at the block (sum of all accounts' vote powers).
     */
    function totalVotePowerAtCached(
        uint256 _blockNumber
    ) external returns (uint256);

    /**
     * @notice Get the vote power of `_owner` at block `_blockNumber` using cache.
     *   It tries to read the cached value and if not found, reads the actual value and stores it in cache.
     *   Can only be used if _blockNumber is in the past, otherwise reverts.
     * @param _owner The address to get voting power.
     * @param _blockNumber The block number at which to fetch.
     * @return Vote power of `_owner` at `_blockNumber`.
     */
    function votePowerOfAtCached(
        address _owner,
        uint256 _blockNumber
    ) external returns (uint256);

    /**
     * Return vote powers for several addresses in a batch.
     * @param _owners The list of addresses to fetch vote power of.
     * @param _blockNumber The block number at which to fetch.
     * @return A list of vote powers.
     */
    function batchVotePowerOfAt(
        address[] memory _owners,
        uint256 _blockNumber
    ) external view returns (uint256[] memory);
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {EnumerableSet} from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import {Address} from "@openzeppelin/contracts/utils/Address.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import {IIAddressUpdater}
    from "@flarenetwork/flare-periphery-contracts/flare/addressUpdater/interfaces/IIAddressUpdater.sol";
import {IWNat} from "../../flareSmartContracts/interfaces/IWNat.sol";
import {IISettingsManagement} from "../../assetManager/interfaces/IISettingsManagement.sol";
import {IIAssetManagerController} from "../interfaces/IIAssetManagerController.sol";
import {GovernedProxyImplementation} from "../../governance/implementation/GovernedProxyImplementation.sol";
import {AddressUpdatable} from "../../flareSmartContracts/implementation/AddressUpdatable.sol";
import {IIAssetManager} from "../../assetManager/interfaces/IIAssetManager.sol";
import {IGovernanceSettings} from "@flarenetwork/flare-periphery-contracts/flare/IGovernanceSettings.sol";
import {IAssetManager} from "../../userInterfaces/IAssetManager.sol";
import {IUUPSUpgradeable} from "../../utils/interfaces/IUUPSUpgradeable.sol";
import {CollateralType} from "../../userInterfaces/data/CollateralType.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IGoverned} from "../../governance/interfaces/IGoverned.sol";
import {IAssetManagerController} from "../../userInterfaces/IAssetManagerController.sol";
import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {IIAddressUpdatable}
    from "@flarenetwork/flare-periphery-contracts/flare/addressUpdater/interfaces/IIAddressUpdatable.sol";
import {IAddressUpdatable} from "../../flareSmartContracts/interfaces/IAddressUpdatable.sol";
import {IRedemptionTimeExtension} from "../../userInterfaces/IRedemptionTimeExtension.sol";
import {GovernedBase} from "../../governance/implementation/GovernedBase.sol";

contract AssetManagerController is
    UUPSUpgradeable,
    GovernedProxyImplementation,
    AddressUpdatable,
    IIAssetManagerController
{
    using EnumerableSet for EnumerableSet.AddressSet;

    error AssetManagerNotManaged();
    error OnlyGovernanceOrEmergencyPauseSenders();
    error AddressZero();

    /**
     * New address in case this controller was replaced.
     * Note: this code contains no checks that replacedBy==0, because when replaced,
     * all calls to AssetManager's updateSettings/pause will fail anyway
     * since they will arrive from wrong controller address.
     */
    address public replacedBy;

    mapping(address => uint256) private assetManagerIndex;
    IIAssetManager[] private assetManagers;

    EnumerableSet.AddressSet private emergencyPauseSenders;

    constructor()
        GovernedProxyImplementation()
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
        address _addressUpdater
    )
        external
    {
        GovernedBase.initialise(_governanceSettings, _initialGovernance);
        AddressUpdatable.setAddressUpdaterValue(_addressUpdater);
    }

    /**
     * Add an asset manager to this controller. The asset manager controller address in the settings of the
     * asset manager must match this. This method automatically marks the asset manager as attached.
     */
    function addAssetManager(IIAssetManager _assetManager)
        external
        onlyGovernance
    {
        if (assetManagerIndex[address(_assetManager)] != 0) return;
        assetManagers.push(_assetManager);
        assetManagerIndex[address(_assetManager)] = assetManagers.length;  // 1+index, so that 0 means empty
        // have to check, otherwise it fails when the controller is replaced
        if (_assetManager.assetManagerController() == address(this)) {
            _assetManager.attachController(true);
        }
    }

    /**
     * Remove an asset manager from this controller, if it is attached to this controller.
     * The asset manager won't be attached any more, so it will be unusable.
     */
    function removeAssetManager(IIAssetManager _assetManager)
        external
        onlyGovernance
    {
        uint256 position = assetManagerIndex[address(_assetManager)];
        if (position == 0) return;
        uint256 index = position - 1;   // the real index, can be 0
        uint256 lastIndex = assetManagers.length - 1;
        if (index < lastIndex) {
            assetManagers[index] = assetManagers[lastIndex];
            assetManagerIndex[address(assetManagers[index])] = index + 1;
        }
        assetManagers.pop();
        assetManagerIndex[address(_assetManager)] = 0;
        // have to check, otherwise it fails when the controller is replaced
        if (_assetManager.assetManagerController() == address(this)) {
            _assetManager.attachController(false);
        }
    }

    /**
     * Return the list of all asset managers managed by this controller.
     */
    function getAssetManagers()
        external view
        returns (IAssetManager[] memory _assetManagers)
    {
        uint256 length = assetManagers.length;
        _assetManagers = new IAssetManager[](length);
        for (uint256 i = 0; i < length; i++) {
            _assetManagers[i] = assetManagers[i];
        }
    }

    /**
     * Check whether the asset manager is managed by this controller.
     * @param _assetManager an asset manager address
     */
    function assetManagerExists(address _assetManager)
        external view
        returns (bool)
    {
        return assetManagerIndex[_assetManager] != 0;
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // UUPS Proxy

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
    function _authorizeUpgrade(address /* _newImplementation */)
        internal pure override
    {
        assert(false);
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // Setters

    function setAgentOwnerRegistry(IIAssetManager[] memory _assetManagers, address _value)
        external
        onlyGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setAgentOwnerRegistry.selector, _value);
    }

    function setAgentVaultFactory(IIAssetManager[] memory _assetManagers, address _value)
        external
        onlyGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setAgentVaultFactory.selector, _value);
    }

    function setCollateralPoolFactory(IIAssetManager[] memory _assetManagers, address _value)
        external
        onlyGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setCollateralPoolFactory.selector, _value);
    }

    function setCollateralPoolTokenFactory(IIAssetManager[] memory _assetManagers, address _value)
        external
        onlyGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setCollateralPoolTokenFactory.selector, _value);
    }

    function upgradeAgentVaultsAndPools(IIAssetManager[] memory _assetManagers, uint256 _start, uint256 _end)
        external
        onlyImmediateGovernance
    {
        _callOnManagers(_assetManagers,
            abi.encodeCall(IIAssetManager.upgradeAgentVaultsAndPools, (_start, _end)));
    }

    function setPriceReader(IIAssetManager[] memory _assetManagers, address _value)
        external
        onlyGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setPriceReader.selector, _value);
    }

    function setFdcVerification(IIAssetManager[] memory _assetManagers, address _value)
        external
        onlyGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setFdcVerification.selector, _value);
    }

    function setCleanerContract(IIAssetManager[] memory _assetManagers, address _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setCleanerContract.selector, _value);
    }

    function setCleanupBlockNumberManager(IIAssetManager[] memory _assetManagers, address _value)
        external
        onlyGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setCleanupBlockNumberManager.selector, _value);
    }

    // if callData is not empty, it is abi encoded call to init function in the new proxy implementation
    function upgradeFAssetImplementation(
        IIAssetManager[] memory _assetManagers,
        address _implementation,
        bytes memory _callData
    )
        external
        onlyGovernance
    {
        _callOnManagers(_assetManagers,
            abi.encodeCall(IISettingsManagement.upgradeFAssetImplementation, (_implementation, _callData)));
    }

    function setMinUpdateRepeatTimeSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setMinUpdateRepeatTimeSeconds.selector, _value);
    }

    function setLotSizeAmg(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setLotSizeAmg.selector, _value);
    }

    function setTimeForPayment(
        IIAssetManager[] memory _assetManagers,
        uint256 _underlyingBlocks,
        uint256 _underlyingSeconds
    )
        external
        onlyGovernance
    {
        _callOnManagers(_assetManagers,
            abi.encodeCall(IISettingsManagement.setTimeForPayment, (_underlyingBlocks, _underlyingSeconds)));
    }

    function setPaymentChallengeReward(
        IIAssetManager[] memory _assetManagers,
        uint256 _rewardVaultCollateralWei,
        uint256 _rewardBIPS
    )
        external
        onlyImmediateGovernance
    {
        _callOnManagers(_assetManagers,
            abi.encodeCall(IISettingsManagement.setPaymentChallengeReward, (_rewardVaultCollateralWei, _rewardBIPS)));
    }

    function setMaxTrustedPriceAgeSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setMaxTrustedPriceAgeSeconds.selector, _value);
    }

    function setCollateralReservationFeeBips(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setCollateralReservationFeeBips.selector, _value);
    }

    function setRedemptionFeeBips(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setRedemptionFeeBips.selector, _value);
    }

    function setRedemptionDefaultFactorVaultCollateralBIPS(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setRedemptionDefaultFactorVaultCollateralBIPS.selector, _value);
    }

    function setConfirmationByOthersAfterSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setConfirmationByOthersAfterSeconds.selector, _value);
    }

    function setConfirmationByOthersRewardUSD5(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setConfirmationByOthersRewardUSD5.selector, _value);
    }

    function setMaxRedeemedTickets(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setMaxRedeemedTickets.selector, _value);
    }

    function setWithdrawalOrDestroyWaitMinSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setWithdrawalOrDestroyWaitMinSeconds.selector, _value);
    }

    function setAttestationWindowSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setAttestationWindowSeconds.selector, _value);
    }

    function setAverageBlockTimeMS(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setAverageBlockTimeMS.selector, _value);
    }

    function setMintingPoolHoldingsRequiredBIPS(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setMintingPoolHoldingsRequiredBIPS.selector, _value);
    }

    function setMintingCapAmg(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setMintingCapAmg.selector, _value);
    }

    function setTokenInvalidationTimeMinSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setTokenInvalidationTimeMinSeconds.selector, _value);
    }

    function setVaultCollateralBuyForFlareFactorBIPS(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setVaultCollateralBuyForFlareFactorBIPS.selector, _value);
    }

    function setAgentExitAvailableTimelockSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setAgentExitAvailableTimelockSeconds.selector, _value);
    }

    function setAgentFeeChangeTimelockSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setAgentFeeChangeTimelockSeconds.selector, _value);
    }

    function setAgentMintingCRChangeTimelockSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setAgentMintingCRChangeTimelockSeconds.selector, _value);
    }

    function setPoolExitCRChangeTimelockSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setPoolExitCRChangeTimelockSeconds.selector, _value);
    }

    function setAgentTimelockedOperationWindowSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setAgentTimelockedOperationWindowSeconds.selector, _value);
    }

    function setCollateralPoolTokenTimelockSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setCollateralPoolTokenTimelockSeconds.selector, _value);
    }

    function setLiquidationStepSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setLiquidationStepSeconds.selector, _value);
    }

    function setLiquidationPaymentFactors(
        IIAssetManager[] memory _assetManagers,
        uint256[] memory _paymentFactors,
        uint256[] memory _vaultCollateralFactors
    )
        external
        onlyGovernance
    {
        _callOnManagers(_assetManagers,
            abi.encodeCall(IISettingsManagement.setLiquidationPaymentFactors,
                (_paymentFactors, _vaultCollateralFactors)));
    }

    function setRedemptionPaymentExtensionSeconds(
        IIAssetManager[] memory _assetManagers,
        uint256 _value
    )
        external
        onlyImmediateGovernance
    {
        _setValueOnManagers(_assetManagers,
            IRedemptionTimeExtension.setRedemptionPaymentExtensionSeconds.selector, _value);
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // Collateral tokens

    function addCollateralType(
        IIAssetManager[] memory _assetManagers,
        CollateralType.Data calldata _data
    )
        external
        onlyImmediateGovernance
    {
        _callOnManagers(_assetManagers,
            abi.encodeCall(IIAssetManager.addCollateralType, (_data)));
    }

    function setCollateralRatiosForToken(
        IIAssetManager[] memory _assetManagers,
        CollateralType.Class _class,
        IERC20 _token,
        uint256 _minCollateralRatioBIPS,
        uint256 _safetyMinCollateralRatioBIPS
    )
        external
        onlyGovernance
    {
        _callOnManagers(_assetManagers,
            abi.encodeCall(IIAssetManager.setCollateralRatiosForToken,
                (_class, _token, _minCollateralRatioBIPS, _safetyMinCollateralRatioBIPS)));
    }

    function deprecateCollateralType(
        IIAssetManager[] memory _assetManagers,
        CollateralType.Class _class,
        IERC20 _token,
        uint256 _invalidationTimeSec
    )
        external
        onlyImmediateGovernance
    {
        _callOnManagers(_assetManagers,
            abi.encodeCall(IIAssetManager.deprecateCollateralType, (_class, _token, _invalidationTimeSec)));
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // Upgrade (second phase)

    /**
     * When asset manager is paused, no new minting can be made.
     * All other operations continue normally.
     */
    function pauseMinting(IIAssetManager[] calldata _assetManagers)
        external
        onlyImmediateGovernance
    {
        _callOnManagers(_assetManagers, abi.encodeCall(IIAssetManager.pauseMinting, ()));
    }

    /**
     * Minting can continue.
     */
    function unpauseMinting(IIAssetManager[] calldata _assetManagers)
        external
        onlyImmediateGovernance
    {
        _callOnManagers(_assetManagers, abi.encodeCall(IIAssetManager.unpauseMinting, ()));
    }

    ////////////////////////////////////////////////////////////////////////////////////
    // ERC 165

    /**
     * Implementation of ERC-165 interface.
     */
    function supportsInterface(bytes4 _interfaceId)
        external pure override
        returns (bool)
    {
        return _interfaceId == type(IERC165).interfaceId
            || _interfaceId == type(IAddressUpdatable).interfaceId
            || _interfaceId == type(IIAddressUpdatable).interfaceId
            || _interfaceId == type(IAssetManagerController).interfaceId
            || _interfaceId == type(IIAssetManagerController).interfaceId
            || _interfaceId == type(IGoverned).interfaceId;
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // Update contracts

    /**
     * Can be called to update address updater managed contracts if there are too many asset managers
     * to update in one block. In such a case, running AddressUpdater.updateContractAddresses will fail
     * and there will be no way to update contracts. This method allow the update to only change some
     * of the asset managers.
     */
    function updateContracts(IIAssetManager[] calldata _assetManagers)
        external
    {
        // read contract addresses
        IIAddressUpdater addressUpdater = IIAddressUpdater(getAddressUpdater());
        address newAddressUpdater = addressUpdater.getContractAddress("AddressUpdater");
        address assetManagerController = addressUpdater.getContractAddress("AssetManagerController");
        address wNat = addressUpdater.getContractAddress("WNat");
        require(newAddressUpdater != address(0) && assetManagerController != address(0) && wNat != address(0),
            AddressZero());
        _updateContracts(_assetManagers, newAddressUpdater, assetManagerController, wNat);
    }

    // called by AddressUpdater.update or AddressUpdater.updateContractAddresses
    function _updateContractAddresses(
        bytes32[] memory _contractNameHashes,
        address[] memory _contractAddresses
    )
        internal override
    {
        address addressUpdater =
            _getContractAddress(_contractNameHashes, _contractAddresses, "AddressUpdater");
        address assetManagerController =
            _getContractAddress(_contractNameHashes, _contractAddresses, "AssetManagerController");
        address wNat =
            _getContractAddress(_contractNameHashes, _contractAddresses, "WNat");
        _updateContracts(assetManagers, addressUpdater, assetManagerController, wNat);
    }

    function _updateContracts(
        IIAssetManager[] memory _assetManagers,
        address addressUpdater,
        address assetManagerController,
        address wNat
    )
        private
    {
        // update address updater if necessary
        if (addressUpdater != getAddressUpdater()) {
            setAddressUpdaterValue(addressUpdater);
        }
        // update contracts on asset managers
        _callOnManagers(_assetManagers,
            abi.encodeCall(IISettingsManagement.updateSystemContracts,
                (assetManagerController, IWNat(wNat))));
        // if this controller was replaced, set forwarding address
        if (assetManagerController != address(this)) {
            replacedBy = assetManagerController;
        }
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // Emergency pause

    function emergencyPause(IIAssetManager[] memory _assetManagers, uint256 _duration)
        external
    {
        bool byGovernance = msg.sender == governance();
        require(byGovernance || emergencyPauseSenders.contains(msg.sender),
            OnlyGovernanceOrEmergencyPauseSenders());
        _callOnManagers(_assetManagers,
            abi.encodeCall(IIAssetManager.emergencyPause, (byGovernance, _duration)));
    }

    function emergencyPauseTransfers(IIAssetManager[] memory _assetManagers, uint256 _duration)
        external
    {
        bool byGovernance = msg.sender == governance();
        require(byGovernance || emergencyPauseSenders.contains(msg.sender),
            OnlyGovernanceOrEmergencyPauseSenders());
        _callOnManagers(_assetManagers,
            abi.encodeCall(IIAssetManager.emergencyPauseTransfers, (byGovernance, _duration)));
    }

    function resetEmergencyPauseTotalDuration(IIAssetManager[] memory _assetManagers)
        external
        onlyImmediateGovernance
    {
        _callOnManagers(_assetManagers,
            abi.encodeCall(IIAssetManager.resetEmergencyPauseTotalDuration, ()));
        _callOnManagers(_assetManagers,
            abi.encodeCall(IIAssetManager.resetEmergencyPauseTransfersTotalDuration, ()));
    }

    function addEmergencyPauseSender(address _address)
        external
        onlyImmediateGovernance
    {
        emergencyPauseSenders.add(_address);
    }

    function removeEmergencyPauseSender(address _address)
        external
        onlyImmediateGovernance
    {
        emergencyPauseSenders.remove(_address);
    }

    function setMaxEmergencyPauseDurationSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setMaxEmergencyPauseDurationSeconds.selector, _value);
    }

    function setEmergencyPauseDurationResetAfterSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external
        onlyGovernance
    {
        _setValueOnManagers(_assetManagers,
            IISettingsManagement.setEmergencyPauseDurationResetAfterSeconds.selector, _value);
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // Helpers

    function _setValueOnManagers(IIAssetManager[] memory _assetManagers, bytes4 _selector, address _value) private {
        _callOnManagers(_assetManagers, abi.encodeWithSelector(_selector, (_value)));
    }

    function _setValueOnManagers(IIAssetManager[] memory _assetManagers, bytes4 _selector, uint256 _value) private {
        _callOnManagers(_assetManagers, abi.encodeWithSelector(_selector, (_value)));
    }

    function _callOnManagers(IIAssetManager[] memory _assetManagers, bytes memory _calldata) private {
        for (uint256 i = 0; i < _assetManagers.length; i++) {
            address assetManager = address(_assetManagers[i]);
            require(assetManagerIndex[assetManager] != 0, AssetManagerNotManaged());
            Address.functionCall(assetManager, _calldata);
        }
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

import {Math} from "@openzeppelin/contracts/utils/math/Math.sol";
import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {AssetManagerBase} from "./AssetManagerBase.sol";
import {Globals} from "../library/Globals.sol";
import {AssetManagerState} from "../library/data/AssetManagerState.sol";
import {AssetManagerSettings} from "../../userInterfaces/data/AssetManagerSettings.sol";
import {IAssetManagerEvents} from "../../userInterfaces/IAssetManagerEvents.sol";


contract EmergencyPauseTransfersFacet is AssetManagerBase, IAssetManagerEvents {
    using SafeCast for uint256;

    error PausedByGovernance();

    function emergencyPauseTransfers(bool _byGovernance, uint256 _duration)
        external
        onlyAssetManagerController
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        bool pausedAtStart = _transfersPaused();
        if (_byGovernance) {
            state.transfersEmergencyPausedUntil = (block.timestamp + _duration).toUint64();
            state.transfersEmergencyPausedByGovernance = true;
        } else {
            if (pausedAtStart && state.transfersEmergencyPausedByGovernance) {
                revert PausedByGovernance();
            }
            AssetManagerSettings.Data storage settings = Globals.getSettings();
            uint256 resetTs = state.transfersEmergencyPausedUntil + settings.emergencyPauseDurationResetAfterSeconds;
            if (resetTs <= block.timestamp) {
                state.transfersEmergencyPausedTotalDuration = 0;
            }
            uint256 currentPauseEndTime = Math.max(state.transfersEmergencyPausedUntil, block.timestamp);
            uint256 projectedStartTime =
                Math.min(currentPauseEndTime - state.transfersEmergencyPausedTotalDuration, block.timestamp);
            uint256 maxEndTime = projectedStartTime + settings.maxEmergencyPauseDurationSeconds;
            uint256 endTime = Math.min(block.timestamp + _duration, maxEndTime);
            state.transfersEmergencyPausedUntil = endTime.toUint64();
            state.transfersEmergencyPausedTotalDuration = (endTime - projectedStartTime).toUint64();
            state.transfersEmergencyPausedByGovernance = false;
        }
        if (_transfersPaused()) {
            emit EmergencyPauseTransfersTriggered(state.transfersEmergencyPausedUntil);
        } else if (pausedAtStart) {
            emit EmergencyPauseTransfersCanceled();
        }
    }

    function resetEmergencyPauseTransfersTotalDuration()
        external
        onlyAssetManagerController
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        state.transfersEmergencyPausedTotalDuration = 0;
    }

    function transfersEmergencyPaused()
        external view
        returns (bool)
    {
        return _transfersPaused();
    }

    function transfersEmergencyPausedUntil()
        external view
        returns (uint256)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        return _transfersPaused() ? state.transfersEmergencyPausedUntil : 0;
    }

    function emergencyPauseTransfersDetails()
        external view
        returns (uint256 _pausedUntil, uint256 _totalPauseDuration, bool _pausedByGovernance)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        return (state.transfersEmergencyPausedUntil, state.transfersEmergencyPausedTotalDuration,
            state.transfersEmergencyPausedByGovernance);
    }

    function _transfersPaused() private view returns (bool) {
        AssetManagerState.State storage state = AssetManagerState.get();
        return state.transfersEmergencyPausedUntil > block.timestamp;
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {AssetManagerBase} from "./AssetManagerBase.sol";
import {IISettingsManagement} from "../interfaces/IISettingsManagement.sol";
import {CollateralTypes} from "../library/CollateralTypes.sol";
import {Globals} from "../library/Globals.sol";
import {SettingsUpdater} from "../library/SettingsUpdater.sol";
import {SettingsValidators} from "../library/SettingsValidators.sol";
import {IIFAsset} from "../../fassetToken/interfaces/IIFAsset.sol";
import {IWNat} from "../../flareSmartContracts/interfaces/IWNat.sol";
import {AssetManagerSettings} from "../../userInterfaces/data/AssetManagerSettings.sol";
import {CollateralType} from "../../userInterfaces/data/CollateralType.sol";
import {IAssetManagerEvents} from "../../userInterfaces/IAssetManagerEvents.sol";
import {IUpgradableProxy} from "../../utils/interfaces/IUpgradableProxy.sol";
import {SafePct} from "../../utils/library/SafePct.sol";


contract SettingsManagementFacet is AssetManagerBase, IAssetManagerEvents, IISettingsManagement {
    using SafeCast for uint256;
    using SafePct for uint256;

    error InvalidAddress();
    error CannotBeZero();
    error IncreaseTooBig();
    error DecreaseTooBig();
    error ValueTooSmall();
    error ValueTooBig();
    error FeeIncreaseTooBig();
    error FeeDecreaseTooBig();
    error LotSizeIncreaseTooBig();
    error LotSizeDecreaseTooBig();
    error LotSizeBiggerThanMintingCap();
    error BipsValueTooHigh();
    error BipsValueTooLow();
    error MustBeAtLeastTwoHours();
    error WindowTooSmall();
    error ConfirmationTimeTooBig();

    struct UpdaterState {
        mapping (bytes4 => uint256) lastUpdate;
    }

    bytes32 internal constant UPDATES_STATE_POSITION = keccak256("fasset.AssetManager.UpdaterState");

    modifier rateLimited() {
        SettingsUpdater.checkEnoughTimeSinceLastUpdate();
        _;
    }

    function updateSystemContracts(address _controller, IWNat _wNat)
        external
        onlyAssetManagerController
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // update assetManagerController
        if (settings.assetManagerController != _controller) {
            settings.assetManagerController = _controller;
            emit ContractChanged("assetManagerController", address(_controller));
        }
        // update wNat
        IWNat oldWNat = Globals.getWNat();
        if (oldWNat != _wNat) {
            CollateralType.Data memory data = CollateralTypes.getInfo(CollateralType.Class.POOL, oldWNat);
            data.validUntil = 0;
            data.token = _wNat;
            CollateralTypes.setPoolWNatCollateralType(data);
            emit ContractChanged("wNat", address(_wNat));
        }
    }

    function setAgentOwnerRegistry(address _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value != address(0), InvalidAddress());
        // update
        settings.agentOwnerRegistry = _value;
        emit ContractChanged("agentOwnerRegistry", _value);
    }

    function setAgentVaultFactory(address _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value != address(0), InvalidAddress());
        // update
        settings.agentVaultFactory = _value;
        emit ContractChanged("agentVaultFactory", _value);
    }

    function setCollateralPoolFactory(address _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value != address(0), InvalidAddress());
        // update
        settings.collateralPoolFactory = _value;
        emit ContractChanged("collateralPoolFactory", _value);
    }

    function setCollateralPoolTokenFactory(address _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value != address(0), InvalidAddress());
        // update
        settings.collateralPoolTokenFactory = _value;
        emit ContractChanged("collateralPoolTokenFactory", _value);
    }

    function setPriceReader(address _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value != address(0), InvalidAddress());
        // update
        settings.priceReader = _value;
        emit ContractChanged("priceReader", _value);
    }

    function setFdcVerification(address _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value != address(0), InvalidAddress());
        // update
        settings.fdcVerification = _value;
        emit IAssetManagerEvents.ContractChanged("fdcVerification", _value);
    }

    function setCleanerContract(address _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        IIFAsset fAsset = Globals.getFAsset();
        // validate
        // update
        fAsset.setCleanerContract(_value);
        emit ContractChanged("cleanerContract", _value);
    }

    function setCleanupBlockNumberManager(address _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        IIFAsset fAsset = Globals.getFAsset();
        // validate
        // update
        fAsset.setCleanupBlockNumberManager(_value);
        emit ContractChanged("cleanupBlockNumberManager", _value);
    }

    function upgradeFAssetImplementation(address _value, bytes memory callData)
        external
        onlyAssetManagerController
        rateLimited
    {
        IUpgradableProxy fAssetProxy = IUpgradableProxy(address(Globals.getFAsset()));
        // validate
        require(_value != address(0), InvalidAddress());
        // update
        if (callData.length > 0) {
            fAssetProxy.upgradeToAndCall(_value, callData);
        } else {
            fAssetProxy.upgradeTo(_value);
        }
        emit ContractChanged("fAsset", _value);
    }

    function setTimeForPayment(uint256 _underlyingBlocks, uint256 _underlyingSeconds)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_underlyingSeconds > 0, CannotBeZero());
        require(_underlyingBlocks > 0, CannotBeZero());
        SettingsValidators.validateTimeForPayment(_underlyingBlocks, _underlyingSeconds, settings.averageBlockTimeMS);
        // update
        settings.underlyingBlocksForPayment = _underlyingBlocks.toUint64();
        settings.underlyingSecondsForPayment = _underlyingSeconds.toUint64();
        emit SettingChanged("underlyingBlocksForPayment", _underlyingBlocks);
        emit SettingChanged("underlyingSecondsForPayment", _underlyingSeconds);
    }

    function setPaymentChallengeReward(uint256 _rewardNATWei, uint256 _rewardBIPS)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_rewardNATWei <= (settings.paymentChallengeRewardUSD5 * 4) + 100 ether, IncreaseTooBig());
        require(_rewardNATWei >= (settings.paymentChallengeRewardUSD5) / 4, DecreaseTooBig());
        require(_rewardBIPS <= (settings.paymentChallengeRewardBIPS * 4) + 100, IncreaseTooBig());
        require(_rewardBIPS >= (settings.paymentChallengeRewardBIPS) / 4, DecreaseTooBig());
        // update
        settings.paymentChallengeRewardUSD5 = _rewardNATWei.toUint128();
        settings.paymentChallengeRewardBIPS = _rewardBIPS.toUint16();
        emit SettingChanged("paymentChallengeRewardUSD5", _rewardNATWei);
        emit SettingChanged("paymentChallengeRewardBIPS", _rewardBIPS);
    }

    function setMinUpdateRepeatTimeSeconds(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value > 0, CannotBeZero());
        // update
        settings.minUpdateRepeatTimeSeconds = _value.toUint64();
        emit SettingChanged("minUpdateRepeatTimeSeconds", _value);
    }

    function setLotSizeAmg(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        // huge lot size increase is very dangerous, because it breaks redemption
        // (converts all tickets to dust)
        require(_value > 0, CannotBeZero());
        require(_value <= settings.lotSizeAMG * 10, LotSizeIncreaseTooBig());
        require(_value >= settings.lotSizeAMG / 10, LotSizeDecreaseTooBig());
        require(settings.mintingCapAMG == 0 || settings.mintingCapAMG >= _value,
            LotSizeBiggerThanMintingCap());
        // update
        settings.lotSizeAMG = _value.toUint64();
        emit SettingChanged("lotSizeAMG", _value);
    }

    function setMaxTrustedPriceAgeSeconds(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value > 0, CannotBeZero());
        require(_value <= settings.maxTrustedPriceAgeSeconds * 2, FeeIncreaseTooBig());
        require(_value >= settings.maxTrustedPriceAgeSeconds / 2, FeeDecreaseTooBig());
        // update
        settings.maxTrustedPriceAgeSeconds = _value.toUint64();
        emit SettingChanged("maxTrustedPriceAgeSeconds", _value);
    }

    function setCollateralReservationFeeBips(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value > 0, CannotBeZero());
        require(_value <= SafePct.MAX_BIPS, BipsValueTooHigh());
        require(_value <= settings.collateralReservationFeeBIPS * 4, FeeIncreaseTooBig());
        require(_value >= settings.collateralReservationFeeBIPS / 4, FeeDecreaseTooBig());
        // update
        settings.collateralReservationFeeBIPS = _value.toUint16();
        emit SettingChanged("collateralReservationFeeBIPS", _value);
    }

    function setRedemptionFeeBips(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value > 0, CannotBeZero());
        require(_value <= SafePct.MAX_BIPS, BipsValueTooHigh());
        require(_value <= settings.redemptionFeeBIPS * 4, FeeIncreaseTooBig());
        require(_value >= settings.redemptionFeeBIPS / 4, FeeDecreaseTooBig());
        // update
        settings.redemptionFeeBIPS = _value.toUint16();
        emit SettingChanged("redemptionFeeBIPS", _value);
    }

    function setRedemptionDefaultFactorVaultCollateralBIPS(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value > SafePct.MAX_BIPS,
            BipsValueTooLow());
        require(_value <= uint256(settings.redemptionDefaultFactorVaultCollateralBIPS).mulBips(12000) + 1000,
            FeeIncreaseTooBig());
        require(_value >= uint256(settings.redemptionDefaultFactorVaultCollateralBIPS).mulBips(8333),
            FeeDecreaseTooBig());
        // update
        settings.redemptionDefaultFactorVaultCollateralBIPS = _value.toUint32();
        emit SettingChanged("redemptionDefaultFactorVaultCollateralBIPS", _value);
    }

    function setConfirmationByOthersAfterSeconds(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value >= 2 hours, MustBeAtLeastTwoHours());
        // update
        settings.confirmationByOthersAfterSeconds = _value.toUint64();
        emit SettingChanged("confirmationByOthersAfterSeconds", _value);
    }

    function setConfirmationByOthersRewardUSD5(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value > 0, CannotBeZero());
        require(_value <= settings.confirmationByOthersRewardUSD5 * 4, FeeIncreaseTooBig());
        require(_value >= settings.confirmationByOthersRewardUSD5 / 4, FeeDecreaseTooBig());
        // update
        settings.confirmationByOthersRewardUSD5 = _value.toUint128();
        emit SettingChanged("confirmationByOthersRewardUSD5", _value);
    }

    function setMaxRedeemedTickets(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value > 0, CannotBeZero());
        require(_value <= settings.maxRedeemedTickets * 2, IncreaseTooBig());
        require(_value >= settings.maxRedeemedTickets / 4, DecreaseTooBig());
        // update
        settings.maxRedeemedTickets = _value.toUint16();
        emit SettingChanged("maxRedeemedTickets", _value);
    }

    function setWithdrawalOrDestroyWaitMinSeconds(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        // making this _value small doesn't present huge danger, so we don't limit decrease
        require(_value > 0, CannotBeZero());
        require(_value <= settings.withdrawalWaitMinSeconds + 10 minutes, IncreaseTooBig());
        // update
        settings.withdrawalWaitMinSeconds = _value.toUint64();
        emit SettingChanged("withdrawalWaitMinSeconds", _value);
    }

    function setAttestationWindowSeconds(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value >= 1 days, WindowTooSmall());
        // update
        settings.attestationWindowSeconds = _value.toUint64();
        emit SettingChanged("attestationWindowSeconds", _value);
    }

    function setAverageBlockTimeMS(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value > 0, CannotBeZero());
        require(_value <= settings.averageBlockTimeMS * 2, IncreaseTooBig());
        require(_value >= settings.averageBlockTimeMS / 2, DecreaseTooBig());
        // update
        settings.averageBlockTimeMS = _value.toUint32();
        emit SettingChanged("averageBlockTimeMS", _value);
    }

    function setMintingPoolHoldingsRequiredBIPS(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value <= settings.mintingPoolHoldingsRequiredBIPS * 4 + SafePct.MAX_BIPS, ValueTooBig());
        // update
        settings.mintingPoolHoldingsRequiredBIPS = _value.toUint32();
        emit SettingChanged("mintingPoolHoldingsRequiredBIPS", _value);
    }

    function setMintingCapAmg(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value == 0 || _value >= settings.lotSizeAMG, ValueTooSmall());
        // update
        settings.mintingCapAMG = _value.toUint64();
        emit SettingChanged("mintingCapAMG", _value);
    }

    function setTokenInvalidationTimeMinSeconds(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        // update
        settings.tokenInvalidationTimeMinSeconds = _value.toUint64();
        emit SettingChanged("tokenInvalidationTimeMinSeconds", _value);
    }

    function setVaultCollateralBuyForFlareFactorBIPS(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value >= SafePct.MAX_BIPS, ValueTooSmall());
        // update
        settings.vaultCollateralBuyForFlareFactorBIPS = _value.toUint32();
        emit SettingChanged("vaultCollateralBuyForFlareFactorBIPS", _value);
    }

    function setAgentExitAvailableTimelockSeconds(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value <= settings.agentExitAvailableTimelockSeconds * 4 + 1 weeks, ValueTooBig());
        // update
        settings.agentExitAvailableTimelockSeconds = _value.toUint64();
        emit SettingChanged("agentExitAvailableTimelockSeconds", _value);
    }

    function setAgentFeeChangeTimelockSeconds(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value <= settings.agentFeeChangeTimelockSeconds * 4 + 1 days, ValueTooBig());
        // update
        settings.agentFeeChangeTimelockSeconds = _value.toUint64();
        emit SettingChanged("agentFeeChangeTimelockSeconds", _value);
    }

    function setAgentMintingCRChangeTimelockSeconds(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value <= settings.agentMintingCRChangeTimelockSeconds * 4 + 1 days, ValueTooBig());
        // update
        settings.agentMintingCRChangeTimelockSeconds = _value.toUint64();
        emit SettingChanged("agentMintingCRChangeTimelockSeconds", _value);
    }

    function setPoolExitCRChangeTimelockSeconds(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value <= settings.poolExitCRChangeTimelockSeconds * 4 + 1 days, ValueTooBig());
        // update
        settings.poolExitCRChangeTimelockSeconds = _value.toUint64();
        emit SettingChanged("poolExitCRChangeTimelockSeconds", _value);
    }

    function setAgentTimelockedOperationWindowSeconds(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value >= 1 hours, ValueTooSmall());
        // update
        settings.agentTimelockedOperationWindowSeconds = _value.toUint64();
        emit SettingChanged("agentTimelockedOperationWindowSeconds", _value);
    }

    function setCollateralPoolTokenTimelockSeconds(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value >= 1 minutes, ValueTooSmall());
        // update
        settings.collateralPoolTokenTimelockSeconds = _value.toUint32();
        emit SettingChanged("collateralPoolTokenTimelockSeconds", _value);
    }

    function setLiquidationStepSeconds(uint256 _stepSeconds)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_stepSeconds > 0, CannotBeZero());
        require(_stepSeconds <= settings.liquidationStepSeconds * 2, IncreaseTooBig());
        require(_stepSeconds >= settings.liquidationStepSeconds / 2, DecreaseTooBig());
        // update
        settings.liquidationStepSeconds = _stepSeconds.toUint64();
        emit SettingChanged("liquidationStepSeconds", _stepSeconds);
    }

    function setLiquidationPaymentFactors(
        uint256[] memory _liquidationFactors,
        uint256[] memory _vaultCollateralFactors
    )
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        SettingsValidators.validateLiquidationFactors(_liquidationFactors, _vaultCollateralFactors);
        // update
        delete settings.liquidationCollateralFactorBIPS;
        delete settings.liquidationFactorVaultCollateralBIPS;
        for (uint256 i = 0; i < _liquidationFactors.length; i++) {
            settings.liquidationCollateralFactorBIPS.push(_liquidationFactors[i].toUint32());
            settings.liquidationFactorVaultCollateralBIPS.push(_vaultCollateralFactors[i].toUint32());
        }
        // emit events
        emit SettingArrayChanged("liquidationCollateralFactorBIPS", _liquidationFactors);
        emit SettingArrayChanged("liquidationFactorVaultCollateralBIPS", _vaultCollateralFactors);
    }

    function setMaxEmergencyPauseDurationSeconds(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value > 0, CannotBeZero());
        require(_value <= settings.maxEmergencyPauseDurationSeconds * 4 + 60, IncreaseTooBig());
        require(_value >= settings.maxEmergencyPauseDurationSeconds / 4, DecreaseTooBig());
        // update
        settings.maxEmergencyPauseDurationSeconds = _value.toUint64();
        // emit events
        emit SettingChanged("maxEmergencyPauseDurationSeconds", _value);
    }

    function setEmergencyPauseDurationResetAfterSeconds(uint256 _value)
        external
        onlyAssetManagerController
        rateLimited
    {
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        // validate
        require(_value > 0, CannotBeZero());
        require(_value <= settings.emergencyPauseDurationResetAfterSeconds * 4 + 3600, IncreaseTooBig());
        require(_value >= settings.emergencyPauseDurationResetAfterSeconds / 4, DecreaseTooBig());
        // update
        settings.emergencyPauseDurationResetAfterSeconds = _value.toUint64();
        // emit events
        emit SettingChanged("emergencyPauseDurationResetAfterSeconds", _value);
    }
}
// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;
pragma abicoder v2;

import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {ICollateralPoolToken} from "../../userInterfaces/ICollateralPoolToken.sol";


interface IICollateralPoolToken is ICollateralPoolToken, IERC165 {

    function mint(address _account, uint256 _amount) external returns (uint256 _timelockExpiresAt);
    function burn(address _account, uint256 _amount, bool _ignoreTimelocked) external;
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

import {IGovernanceSettings} from "@flarenetwork/flare-periphery-contracts/flare/IGovernanceSettings.sol";
import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {ReentrancyGuard} from "../../openzeppelin/security/ReentrancyGuard.sol";
import {CollateralTypes} from "../library/CollateralTypes.sol";
import {SettingsInitializer} from "../library/SettingsInitializer.sol";
import {IIAssetManager} from "../../assetManager/interfaces/IIAssetManager.sol";
import {IDiamondCut} from "../../diamond/interfaces/IDiamondCut.sol";
import {IDiamondLoupe} from "../../diamond/interfaces/IDiamondLoupe.sol";
import {LibDiamond} from "../../diamond/library/LibDiamond.sol";
import {IGoverned} from "../../governance/interfaces/IGoverned.sol";
import {GovernedBase} from "../../governance/implementation/GovernedBase.sol";
import {GovernedProxyImplementation} from "../../governance/implementation/GovernedProxyImplementation.sol";
import {AssetManagerSettings} from "../../userInterfaces/data/AssetManagerSettings.sol";
import {CollateralType} from "../../userInterfaces/data/CollateralType.sol";
import {IAgentPing} from "../../userInterfaces/IAgentPing.sol";
import {IAssetManager} from "../../userInterfaces/IAssetManager.sol";


contract AssetManagerInit is GovernedProxyImplementation, ReentrancyGuard {
    error NotInitialized();

    function init(
        IGovernanceSettings _governanceSettings,
        address _initialGovernance,
        AssetManagerSettings.Data memory _settings,
        CollateralType.Data[] memory _initialCollateralTypes
    )
        external
    {
        GovernedBase.initialise(_governanceSettings, _initialGovernance);
        ReentrancyGuard.initializeReentrancyGuard();
        SettingsInitializer.validateAndSet(_settings);
        CollateralTypes.initialize(_initialCollateralTypes);
        _initIERC165();
    }

    /**
     * If a diamond cut adds methods to one of the declared interfaces, it should call this method in initialization.
     * In this way ERC165 identifiers for both old and new version of interface will be marked as supported,
     * which is correct since the new interface should be backward compatible with the old one.
     */
    function upgradeERC165Identifiers() external {
        LibDiamond.DiamondStorage storage ds = LibDiamond.diamondStorage();
        require(ds.supportedInterfaces[type(IERC165).interfaceId], NotInitialized());
        ds.supportedInterfaces[type(IGoverned).interfaceId] = true;
        ds.supportedInterfaces[type(IAssetManager).interfaceId] = true;
        ds.supportedInterfaces[type(IIAssetManager).interfaceId] = true;
        ds.supportedInterfaces[type(IAgentPing).interfaceId] = true;
    }

    function _initIERC165() private {
        LibDiamond.DiamondStorage storage ds = LibDiamond.diamondStorage();
        ds.supportedInterfaces[type(IERC165).interfaceId] = true;
        ds.supportedInterfaces[type(IDiamondLoupe).interfaceId] = true;
        ds.supportedInterfaces[type(IDiamondCut).interfaceId] = true;
        ds.supportedInterfaces[type(IGoverned).interfaceId] = true;
        ds.supportedInterfaces[type(IAssetManager).interfaceId] = true;
        ds.supportedInterfaces[type(IIAssetManager).interfaceId] = true;
        ds.supportedInterfaces[type(IAgentPing).interfaceId] = true;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import { IGovernanceSettings, GovernedBase } from "./GovernedBase.sol";


/**
 * Base class for proxy implementations or diamond facets that expose governed methods -
 * prevents initialization of the implementation/facet as contract (to avoid selfdestruct by attackers).
 *
 * The GovernedBase.initialise can later be called only through a proxy. It should be
 * called through proxy constructor or in diamond cut initializer.
 **/
abstract contract GovernedProxyImplementation is GovernedBase {
    address private constant EMPTY_ADDRESS = 0x0000000000000000000000000000000000001111;

    // Mark as initialised and set governance to an invalid address.
    constructor() {
        initialise(IGovernanceSettings(EMPTY_ADDRESS), EMPTY_ADDRESS);
    }
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

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
/******************************************************************************\
* Author: Nick Mudge <nick@perfectabstractions.com> (https://twitter.com/mudgen)
* EIP-2535 Diamonds: https://eips.ethereum.org/EIPS/eip-2535
/******************************************************************************/

// The functions in DiamondLoupeFacet MUST be added to a diamond.
// The EIP-2535 Diamond standard requires these functions.

import { IERC165 } from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import { IDiamondLoupe } from "../interfaces/IDiamondLoupe.sol";
import { LibDiamond } from  "../library/LibDiamond.sol";

// solhint-disable no-inline-assembly
contract DiamondLoupeFacet is IDiamondLoupe, IERC165 {
    // Diamond Loupe Functions
    ////////////////////////////////////////////////////////////////////
    /// These functions are expected to be called frequently by tools.
    //
    // struct Facet {
    //     address facetAddress;
    //     bytes4[] functionSelectors;
    // }

    /// @notice Gets all facets and their selectors.
    /// @return facets_ Facet
    function facets()
        external override view
        returns (Facet[] memory facets_)
    {
        LibDiamond.DiamondStorage storage ds = LibDiamond.diamondStorage();
        uint256 selectorCount = ds.selectors.length;
        // create an array set to the maximum size possible
        facets_ = new Facet[](selectorCount);
        // create an array for counting the number of selectors for each facet
        uint16[] memory numFacetSelectors = new uint16[](selectorCount);
        // total number of facets
        uint256 numFacets;
        // loop through function selectors
        for (uint256 selectorIndex; selectorIndex < selectorCount; selectorIndex++) {
            bytes4 selector = ds.selectors[selectorIndex];
            address facetAddress_ = ds.facetAddressAndSelectorPosition[selector].facetAddress;
            bool continueLoop = false;
            // find the functionSelectors array for selector and add selector to it
            for (uint256 facetIndex; facetIndex < numFacets; facetIndex++) {
                if (facets_[facetIndex].facetAddress == facetAddress_) {
                    facets_[facetIndex].functionSelectors[numFacetSelectors[facetIndex]] = selector;
                    numFacetSelectors[facetIndex]++;
                    continueLoop = true;
                    break;
                }
            }
            // if functionSelectors array exists for selector then continue loop
            if (continueLoop) {
                continueLoop = false;
                continue;
            }
            // create a new functionSelectors array for selector
            facets_[numFacets].facetAddress = facetAddress_;
            facets_[numFacets].functionSelectors = new bytes4[](selectorCount);
            facets_[numFacets].functionSelectors[0] = selector;
            numFacetSelectors[numFacets] = 1;
            numFacets++;
        }
        for (uint256 facetIndex; facetIndex < numFacets; facetIndex++) {
            uint256 numSelectors = numFacetSelectors[facetIndex];
            bytes4[] memory selectors = facets_[facetIndex].functionSelectors;
            // setting the number of selectors
            assembly {
                mstore(selectors, numSelectors)
            }
        }
        // setting the number of facets
        assembly {
            mstore(facets_, numFacets)
        }
    }

    /// @notice Gets all the function selectors supported by a specific facet.
    /// @param _facet The facet address.
    /// @return _facetFunctionSelectors The selectors associated with a facet address.
    function facetFunctionSelectors(address _facet)
        external override view
        returns (bytes4[] memory _facetFunctionSelectors)
    {
        LibDiamond.DiamondStorage storage ds = LibDiamond.diamondStorage();
        uint256 selectorCount = ds.selectors.length;
        uint256 numSelectors;
        _facetFunctionSelectors = new bytes4[](selectorCount);
        // loop through function selectors
        for (uint256 selectorIndex; selectorIndex < selectorCount; selectorIndex++) {
            bytes4 selector = ds.selectors[selectorIndex];
            address facetAddress_ = ds.facetAddressAndSelectorPosition[selector].facetAddress;
            if (_facet == facetAddress_) {
                _facetFunctionSelectors[numSelectors] = selector;
                numSelectors++;
            }
        }
        // Set the number of selectors in the array
        assembly {
            mstore(_facetFunctionSelectors, numSelectors)
        }
    }

    /// @notice Get all the facet addresses used by a diamond.
    /// @return facetAddresses_
    function facetAddresses()
        external override view
        returns (address[] memory facetAddresses_)
    {
        LibDiamond.DiamondStorage storage ds = LibDiamond.diamondStorage();
        uint256 selectorCount = ds.selectors.length;
        // create an array set to the maximum size possible
        facetAddresses_ = new address[](selectorCount);
        uint256 numFacets;
        // loop through function selectors
        for (uint256 selectorIndex; selectorIndex < selectorCount; selectorIndex++) {
            bytes4 selector = ds.selectors[selectorIndex];
            address facetAddress_ = ds.facetAddressAndSelectorPosition[selector].facetAddress;
            bool continueLoop = false;
            // see if we have collected the address already and break out of loop if we have
            for (uint256 facetIndex; facetIndex < numFacets; facetIndex++) {
                if (facetAddress_ == facetAddresses_[facetIndex]) {
                    continueLoop = true;
                    break;
                }
            }
            // continue loop if we already have the address
            if (continueLoop) {
                continueLoop = false;
                continue;
            }
            // include address
            facetAddresses_[numFacets] = facetAddress_;
            numFacets++;
        }
        // Set the number of facet addresses in the array
        assembly {
            mstore(facetAddresses_, numFacets)
        }
    }

    /// @notice Gets the facet address that supports the given selector.
    /// @dev If facet is not found return address(0).
    /// @param _functionSelector The function selector.
    /// @return facetAddress_ The facet address.
    function facetAddress(bytes4 _functionSelector)
        external override view
        returns (address facetAddress_)
    {
        LibDiamond.DiamondStorage storage ds = LibDiamond.diamondStorage();
        facetAddress_ = ds.facetAddressAndSelectorPosition[_functionSelector].facetAddress;
    }

    // This implements ERC-165.
    function supportsInterface(bytes4 _interfaceId)
        external override view
        returns (bool)
    {
        LibDiamond.DiamondStorage storage ds = LibDiamond.diamondStorage();
        return ds.supportedInterfaces[_interfaceId];
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

import {IAgentPing} from "../../userInterfaces/IAgentPing.sol";
import {AssetManagerBase} from "./AssetManagerBase.sol";
import {Agent} from "../../assetManager/library/data/Agent.sol";


contract AgentPingFacet is AssetManagerBase, IAgentPing {
    /**
     * @inheritdoc IAgentPing
     */
    function agentPing(address _agentVault, uint256 _query) external {
        emit AgentPing(_agentVault, msg.sender, _query);
    }

    /**
     * @inheritdoc IAgentPing
     */
    function agentPingResponse(address _agentVault, uint256 _query, string memory _response)
        external
        onlyAgentVaultOwner(_agentVault)
    {
        Agent.State storage agent = Agent.get(_agentVault);
        emit AgentPingResponse(_agentVault, agent.ownerManagementAddress, _query, _response);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Math} from "@openzeppelin/contracts/utils/math/Math.sol";
import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {AssetManagerBase} from "./AssetManagerBase.sol";
import {Globals} from "../library/Globals.sol";
import {AssetManagerState} from "../library/data/AssetManagerState.sol";
import {AssetManagerSettings} from "../../userInterfaces/data/AssetManagerSettings.sol";
import {IAssetManagerEvents} from "../../userInterfaces/IAssetManagerEvents.sol";


contract EmergencyPauseFacet is AssetManagerBase, IAssetManagerEvents {
    using SafeCast for uint256;

    error PausedByGovernance();

    function emergencyPause(bool _byGovernance, uint256 _duration)
        external
        onlyAssetManagerController
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        bool pausedAtStart = _paused();
        if (_byGovernance) {
            state.emergencyPausedUntil = (block.timestamp + _duration).toUint64();
            state.emergencyPausedByGovernance = true;
        } else {
            if (pausedAtStart && state.emergencyPausedByGovernance) {
                revert PausedByGovernance();
            }
            AssetManagerSettings.Data storage settings = Globals.getSettings();
            if (state.emergencyPausedUntil + settings.emergencyPauseDurationResetAfterSeconds <= block.timestamp) {
                state.emergencyPausedTotalDuration = 0;
            }
            uint256 currentPauseEndTime = Math.max(state.emergencyPausedUntil, block.timestamp);
            uint256 projectedStartTime =
                Math.min(currentPauseEndTime - state.emergencyPausedTotalDuration, block.timestamp);
            uint256 maxEndTime = projectedStartTime + settings.maxEmergencyPauseDurationSeconds;
            uint256 endTime = Math.min(block.timestamp + _duration, maxEndTime);
            state.emergencyPausedUntil = endTime.toUint64();
            state.emergencyPausedTotalDuration = (endTime - projectedStartTime).toUint64();
            state.emergencyPausedByGovernance = false;
        }
        if (_paused()) {
            emit EmergencyPauseTriggered(state.emergencyPausedUntil);
        } else if (pausedAtStart) {
            emit EmergencyPauseCanceled();
        }
    }

    function resetEmergencyPauseTotalDuration()
        external
        onlyAssetManagerController
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        state.emergencyPausedTotalDuration = 0;
    }

    function emergencyPaused()
        external view
        returns (bool)
    {
        return _paused();
    }

    function emergencyPausedUntil()
        external view
        returns (uint256)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        return _paused() ? state.emergencyPausedUntil : 0;
    }

    function emergencyPauseDetails()
        external view
        returns (uint256 _pausedUntil, uint256 _totalPauseDuration, bool _pausedByGovernance)
    {
        AssetManagerState.State storage state = AssetManagerState.get();
        return (state.emergencyPausedUntil, state.emergencyPausedTotalDuration, state.emergencyPausedByGovernance);
    }

    function _paused() private view returns (bool) {
        AssetManagerState.State storage state = AssetManagerState.get();
        return state.emergencyPausedUntil > block.timestamp;
    }
}
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

import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import {UUPSUpgradeable} from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {IICollateralPoolToken} from "../interfaces/IICollateralPoolToken.sol";
import {IICollateralPool} from "../../collateralPool/interfaces/IICollateralPool.sol";
import {IIAssetManager} from "../../assetManager/interfaces/IIAssetManager.sol";
import {ICollateralPoolToken} from "../../userInterfaces/ICollateralPoolToken.sol";


contract CollateralPoolToken is IICollateralPoolToken, ERC20, UUPSUpgradeable {
    using SafeCast for uint256;

    error OnlyAssetManager();
    error InsufficientNonTimelockedBalance();
    error InsufficientTransferableBalance();
    error AlreadyInitialized();
    error OnlyCollateralPool();

    struct Timelock {
        uint128 amount;
        uint64 endTime;
    }

    struct TimelockQueue {
        mapping(uint256 => Timelock) data;
        uint128 start;
        uint128 end;
    }

    address public collateralPool;  // practically immutable because there is no setter

    string private tokenName;       // practically immutable because there is no setter
    string private tokenSymbol;     // practically immutable because there is no setter

    mapping(address => TimelockQueue) private timelocksByAccount;
    bool private ignoreTimelocked;
    bool private initialized;

    modifier onlyCollateralPool {
        require(msg.sender == collateralPool, OnlyCollateralPool());
        _;
    }

    // Only used in some tests.
    // The implementation in production will always be deployed with all zero address for collateral pool.
    constructor(
        address _collateralPool,
        string memory _tokenName,
        string memory _tokenSymbol
    )
        ERC20(_tokenName, _tokenSymbol)
    {
        initialize(_collateralPool, _tokenName, _tokenSymbol);
    }

    function initialize(
        address _collateralPool,
        string memory _tokenName,
        string memory _tokenSymbol
    )
        public
    {
        require(!initialized, AlreadyInitialized());
        initialized = true;
        // init vars
        collateralPool = _collateralPool;
        tokenName = _tokenName;
        tokenSymbol = _tokenSymbol;
    }

    /**
     * @dev Returns the name of the token.
     */
    function name() public view virtual override returns (string memory) {
        return tokenName;
    }

    /**
     * @dev Returns the symbol of the token, usually a shorter version of the
     * name.
     */
    function symbol() public view virtual override returns (string memory) {
        return tokenSymbol;
    }

    function mint(
        address _account,
        uint256 _amount
    )
        external
        onlyCollateralPool
        returns (uint256 _timelockExpiresAt)
    {
        _mint(_account, _amount);
        uint256 timelockDuration = _getTimelockDuration();
        _timelockExpiresAt = block.timestamp + timelockDuration;
        if (timelockDuration > 0 && _amount > 0) {
            TimelockQueue storage timelocks = timelocksByAccount[_account];
            timelocks.data[timelocks.end++] = Timelock({
                amount: _amount.toUint128(),
                endTime: _timelockExpiresAt.toUint64()
            });
        }
    }

    function burn(
        address _account,
        uint256 _amount,
        bool _ignoreTimelocked
    )
        external
        onlyCollateralPool
    {
        if (_ignoreTimelocked) {
            ignoreTimelocked = true;
        }
        _burn(_account, _amount);
        if (_ignoreTimelocked) {
            ignoreTimelocked = false;
        }
    }

    function lockedBalanceOf(
        address _account
    )
        external view
        returns (uint256)
    {
        uint256 debtLockedBalance = debtLockedBalanceOf(_account);
        uint256 timelockedBalance = timelockedBalanceOf(_account);
        return (debtLockedBalance > timelockedBalance) ? debtLockedBalance : timelockedBalance;
    }

    function transferableBalanceOf(
        address _account
    )
        external view
        returns (uint256)
    {
        uint256 debtFreeBalance = debtFreeBalanceOf(_account);
        uint256 nonTimelockedBalance = nonTimelockedBalanceOf(_account);
        return (debtFreeBalance < nonTimelockedBalance) ? debtFreeBalance : nonTimelockedBalance;
    }

    function debtFreeBalanceOf(
        address _account
    )
        public view
        returns (uint256)
    {
        return IICollateralPool(collateralPool).debtFreeTokensOf(_account);
    }

    function debtLockedBalanceOf(
        address _account
    )
        public view
        returns (uint256)
    {
        return IICollateralPool(collateralPool).debtLockedTokensOf(_account);
    }

    function timelockedBalanceOf(
        address _account
    )
        public view
        returns (uint256 _timelocked)
    {
        TimelockQueue storage timelocks = timelocksByAccount[_account];
        uint256 end = timelocks.end;
        for (uint256 i = timelocks.start; i < end; i++) {
            Timelock storage timelock = timelocks.data[i];
            if (timelock.endTime > block.timestamp) {
                _timelocked += timelock.amount;
            }
        }
        // in agent payout, locked tokens can be burnt without a timelock update,
        // which makes timelockedBalance > totalBalance
        uint256 totalBalance = balanceOf(_account);
        _timelocked = (_timelocked < totalBalance) ? _timelocked : totalBalance;
    }

    function nonTimelockedBalanceOf(
        address _account
    )
        public view
        returns (uint256)
    {
        return balanceOf(_account) - timelockedBalanceOf(_account);
    }

    function _beforeTokenTransfer(
        address _from, address /* _to */, uint256 _amount
    )
        internal override
    {
        if (msg.sender != collateralPool) {
            uint256 transferable = debtFreeBalanceOf(_from);
            require(_amount <= transferable, InsufficientTransferableBalance());
        }
        // either user transfer or non-minting collateral pool with ignoreTimelocked=false flag
        if (!ignoreTimelocked && _from != address(0)) {
            // 10 is some arbitrary number that is usually enough; however, there isn't much damage
            // if it is too little - just the non-timelocked balance may be too small and you have to call again
            cleanupExpiredTimelocks(_from, 10);
            uint256 nonTimelocked = nonTimelockedBalanceOf(_from);
            require(_amount <= nonTimelocked, InsufficientNonTimelockedBalance());
        }
        // if ignoreTimelock, then we are spending from timelocked balance,
        // the reason why it is not updated is because it might not fit in one transaction
        // (if timelock data is too large), which could block the asset manager from making
        // agent payout from the pool
    }

    // this can be called externally by anyone with different _maxTimelockedEntries,
    // if there are too many timelocked entries to clear in one transaction
    // (should be rare, especially if timelock duration is short - e.g. <= day)
    function cleanupExpiredTimelocks(
        address _account,
        uint256 _maxTimelockedEntries
    )
        public
        returns (bool _cleanedAllExpired)
    {
        TimelockQueue storage timelocks = timelocksByAccount[_account];
        uint256 start = timelocks.start;
        for (uint256 count = 0; count < _maxTimelockedEntries; count++) {
            if (start >= timelocks.end || timelocks.data[start].endTime > block.timestamp) {
                break;
            }
            delete timelocks.data[start++];
        }
        timelocks.start = start.toUint128();
        return start >= timelocks.end || timelocks.data[start].endTime > block.timestamp;
    }

    function _getTimelockDuration()
        internal view
        returns (uint256)
    {
        IIAssetManager assetManager = IICollateralPool(collateralPool).assetManager();
        return assetManager.getCollateralPoolTokenTimelockSeconds();
    }

    /**
     * Implementation of ERC-165 interface.
     */
    function supportsInterface(bytes4 _interfaceId)
        external pure override
        returns (bool)
    {
        return _interfaceId == type(IERC165).interfaceId
            || _interfaceId == type(IERC20).interfaceId
            || _interfaceId == type(ICollateralPoolToken).interfaceId;
    }

    ////////////////////////////////////////////////////////////////////////////////////
    // UUPS proxy upgrade

    function implementation() external view returns (address) {
        return _getImplementation();
    }

    /**
     * Upgrade calls can only arrive through asset manager.
     * See UUPSUpgradeable._authorizeUpgrade.
     */
    function _authorizeUpgrade(address /* _newImplementation */)
        internal virtual override
    {
        IIAssetManager assetManager = IICollateralPool(collateralPool).assetManager();
        require(msg.sender == address(assetManager), OnlyAssetManager());
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import { GovernedBase } from "./GovernedBase.sol";
import { IGovernanceSettings } from "@flarenetwork/flare-periphery-contracts/flare/IGovernanceSettings.sol";


/**
 * @title Governed
 * @dev For deployed, governed contracts, enforce non-zero addresses at create time.
 **/
abstract contract Governed is GovernedBase {
    constructor(IGovernanceSettings _governanceSettings, address _initialGovernance) {
        initialise(_governanceSettings, _initialGovernance);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IPayment} from "@flarenetwork/flare-periphery-contracts/flare/IFdcVerification.sol";
import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {ICoreVaultClient} from "../../userInterfaces/ICoreVaultClient.sol";
import {AssetManagerBase} from "./AssetManagerBase.sol";
import {ReentrancyGuard} from "../../openzeppelin/security/ReentrancyGuard.sol";
import {Conversion} from "../library/Conversion.sol";
import {CoreVaultClient} from "../library/CoreVaultClient.sol";
import {Agent} from "../library/data/Agent.sol";
import {SafePct} from "../../utils/library/SafePct.sol";
import {Math} from "@openzeppelin/contracts/utils/math/Math.sol";
import {AssetManagerState} from "../library/data/AssetManagerState.sol";
import {PaymentReference} from "../library/data/PaymentReference.sol";
import {AgentCollateral} from "../library/AgentCollateral.sol";
import {Redemptions} from "../library/Redemptions.sol";
import {RedemptionRequests} from "../library/RedemptionRequests.sol";
import {UnderlyingBalance} from "../library/UnderlyingBalance.sol";
import {Collateral} from "../library/data/Collateral.sol";
import {PaymentConfirmations} from "../library/data/PaymentConfirmations.sol";
import {AgentBacking} from "../library/AgentBacking.sol";
import {SafeMath64} from "../../utils/library/SafeMath64.sol";
import {TransactionAttestation} from "../library/TransactionAttestation.sol";
import {UnderlyingBlockUpdater} from "../library/UnderlyingBlockUpdater.sol";


contract CoreVaultClientFacet is AssetManagerBase, ReentrancyGuard, ICoreVaultClient {
    using SafePct for uint256;
    using SafeCast for uint256;
    using SafeCast for int256;
    using AgentCollateral for Collateral.CombinedData;
    using PaymentConfirmations for PaymentConfirmations.State;

    error CannotReturnZeroLots();
    error InvalidAgentStatus();
    error InvalidPaymentReference();
    error NoActiveReturnRequest();
    error NotEnoughAvailableOnCoreVault();
    error NotEnoughFreeCollateral();
    error NotEnoughUnderlying();
    error NothingMinted();
    error PaymentNotFromCoreVault();
    error PaymentNotToAgentsAddress();
    error RequestedAmountTooSmall();
    error ReturnFromCoreVaultAlreadyRequested();
    error TooLittleMintingLeftAfterTransfer();
    error TransferAlreadyActive();
    error ZeroTransferNotAllowed();

    // core vault may not be enabled on all chains
    modifier onlyEnabled {
        CoreVaultClient.checkEnabled();
        _;
    }

    // prevent initialization of implementation contract
    constructor() {
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        state.initialized = true;
    }

    /**
     * Agent can transfer their backing to core vault.
     * They then get a redemption requests which the owner pays just like any other redemption request.
     * After that, the agent's collateral is released.
     * NOTE: only agent vault owner can call
     * @param _agentVault the agent vault address
     * @param _amountUBA the amount to transfer to the core vault
     */
    function transferToCoreVault(
        address _agentVault,
        uint256 _amountUBA
    )
        external
        onlyEnabled
        notEmergencyPaused
        nonReentrant
        onlyAgentVaultOwner(_agentVault)
    {
        Agent.State storage agent = Agent.get(_agentVault);
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        // for agent in full liquidation, the system cannot know if there is enough underlying for the transfer
        require(agent.status != Agent.Status.FULL_LIQUIDATION, InvalidAgentStatus());
        // forbid 0 transfer
        require(_amountUBA > 0, ZeroTransferNotAllowed());
        // agent must have enough underlying for the transfer (if the required backing < 100%, they may have less)
        require(_amountUBA.toInt256() <= agent.underlyingBalanceUBA, NotEnoughUnderlying());
        // only one transfer can be active
        require(agent.activeTransferToCoreVault == 0, TransferAlreadyActive());
        // close agent's redemption tickets
        uint64 amountAMG = Conversion.convertUBAToAmg(_amountUBA);
        (uint64 transferredAMG,) = Redemptions.closeTickets(agent, amountAMG, false);
        require(transferredAMG > 0, NothingMinted());
        // check the remaining amount
        (uint256 maximumTransferAMG,) = CoreVaultClient.maximumTransferToCoreVaultAMG(agent);
        require(transferredAMG <= maximumTransferAMG, TooLittleMintingLeftAfterTransfer());
        // create ordinary redemption request to core vault address
        string memory underlyingAddress = state.coreVaultManager.coreVaultAddress();
        // NOTE: there will be no redemption fee, so the agent needs enough free underlying for the
        // underlying transaction fee, otherwise they will go into full liquidation
        uint64 redemptionRequestId = RedemptionRequests.createRedemptionRequest(
            RedemptionRequests.AgentRedemptionData(_agentVault, transferredAMG),
            state.nativeAddress, underlyingAddress, false, payable(address(0)), 0,
            state.transferTimeExtensionSeconds, true);
        // set the active request
        agent.activeTransferToCoreVault = redemptionRequestId;
        // send event
        uint256 transferredUBA = Conversion.convertAmgToUBA(transferredAMG);
        emit TransferToCoreVaultStarted(_agentVault, redemptionRequestId, transferredUBA);
    }

    /**
     * Request that core vault transfers funds to the agent's underlying address,
     * which makes them available for redemptions. This method reserves agent's collateral.
     * This may be sent by an agent when redemptions dominate mintings, so that the agents
     * are empty but want to earn from redemptions.
     * NOTE: only agent vault owner can call
     * NOTE: there can be only one active return request (until it is confirmed or cancelled).
     * @param _agentVault the agent vault address
     * @param _lots number of lots (same lots as for minting and redemptions)
     */
    function requestReturnFromCoreVault(
        address _agentVault,
        uint256 _lots
    )
        external
        onlyEnabled
        notEmergencyPaused
        nonReentrant
        onlyAgentVaultOwner(_agentVault)
    {
        Agent.State storage agent = Agent.get(_agentVault);
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        require(agent.activeReturnFromCoreVaultId == 0, ReturnFromCoreVaultAlreadyRequested());
        Collateral.CombinedData memory collateralData = AgentCollateral.combinedData(agent);
        require(_lots > 0, CannotReturnZeroLots());
        require(agent.status == Agent.Status.NORMAL, InvalidAgentStatus());
        require(collateralData.freeCollateralLotsOptionalFee(agent, false) >= _lots, NotEnoughFreeCollateral());
        uint256 availableLots = CoreVaultClient.coreVaultAmountLots();
        require(_lots <= availableLots, NotEnoughAvailableOnCoreVault());
        // create new request id
        state.newTransferFromCoreVaultId += PaymentReference.randomizedIdSkip();
        uint64 requestId = state.newTransferFromCoreVaultId;
        agent.activeReturnFromCoreVaultId = requestId;
        // reserve collateral
        assert(agent.returnFromCoreVaultReservedAMG == 0);
        uint64 amountAMG = Conversion.convertLotsToAMG(_lots);
        agent.returnFromCoreVaultReservedAMG = amountAMG;
        agent.reservedAMG += amountAMG;
        // request
        bytes32 paymentReference = PaymentReference.returnFromCoreVault(requestId);
        uint128 amountUBA = Conversion.convertAmgToUBA(amountAMG).toUint128();
        state.coreVaultManager.requestTransferFromCoreVault(
            agent.underlyingAddressString, paymentReference, amountUBA, true);
        emit ReturnFromCoreVaultRequested(_agentVault, requestId, paymentReference, amountUBA);
    }

    /**
     * Before the return request is processed, it can be cancelled, releasing the agent's reserved collateral.
     * @param _agentVault the agent vault address
     */
    function cancelReturnFromCoreVault(
        address _agentVault
    )
        external
        onlyEnabled
        nonReentrant
        onlyAgentVaultOwner(_agentVault)
    {
        Agent.State storage agent = Agent.get(_agentVault);
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        uint256 requestId = agent.activeReturnFromCoreVaultId;
        require(requestId != 0, NoActiveReturnRequest());
        state.coreVaultManager.cancelTransferRequestFromCoreVault(agent.underlyingAddressString);
        CoreVaultClient.deleteReturnFromCoreVaultRequest(agent);
        emit ReturnFromCoreVaultCancelled(_agentVault, requestId);
    }

    /**
     * Confirm the payment from core vault to the agent's underlying address.
     * This adds the reserved funds to the agent's backing.
     * @param _payment FDC payment proof
     * @param _agentVault the agent vault address
     */
    function confirmReturnFromCoreVault(
        IPayment.Proof calldata _payment,
        address _agentVault
    )
        external
        onlyEnabled
        nonReentrant
        onlyAgentVaultOwner(_agentVault)
    {
        Agent.State storage agent = Agent.get(_agentVault);
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        TransactionAttestation.verifyPaymentSuccess(_payment);
        uint64 requestId = agent.activeReturnFromCoreVaultId;
        require(requestId != 0, NoActiveReturnRequest());
        require(_payment.data.responseBody.sourceAddressHash == state.coreVaultManager.coreVaultAddressHash(),
            PaymentNotFromCoreVault());
        require(_payment.data.responseBody.receivingAddressHash == agent.underlyingAddressHash,
            PaymentNotToAgentsAddress());
        require(_payment.data.responseBody.standardPaymentReference == PaymentReference.returnFromCoreVault(requestId),
            InvalidPaymentReference());
        // make sure payment isn't used again
        AssetManagerState.get().paymentConfirmations.confirmIncomingPayment(_payment);
        // we account for the option that CV pays more or less than the reserved amount:
        // - if less, only the amount received gets converted to redemption ticket
        // - if more, the extra amount becomes the agent's free underlying
        uint256 receivedAmountUBA = _payment.data.responseBody.receivedAmount.toUint256();
        uint64 receivedAmountAMG = Conversion.convertUBAToAmg(receivedAmountUBA);
        uint64 remintedAMG = SafeMath64.min64(agent.returnFromCoreVaultReservedAMG, receivedAmountAMG);
        // create redemption ticket
        AgentBacking.createNewMinting(agent, remintedAMG);
        // update underlying amount
        UnderlyingBalance.increaseBalance(agent, receivedAmountUBA);
        // update underlying block
        UnderlyingBlockUpdater.updateCurrentBlockForVerifiedPayment(_payment);
        // clear the reservation
        CoreVaultClient.deleteReturnFromCoreVaultRequest(agent);
        // send event
        uint256 remintedUBA = Conversion.convertAmgToUBA(remintedAMG);
        emit ReturnFromCoreVaultConfirmed(_agentVault, requestId, receivedAmountUBA, remintedUBA);
    }

    /**
     * Directly redeem from core vault by a user holding FAssets.
     * This is like ordinary redemption, but the redemption time is much longer (a day or more)
     * and there is no possibility of redemption.
     * @param _lots the number of lots, must be larger than `coreVaultMinimumRedeemLots` setting
     * @param _redeemerUnderlyingAddress the underlying address to which the assets will be redeemed;
     *      must have been added to the `allowedDestinations` list in the core vault manager by
     *      the governance before the redemption request.
     */
    function redeemFromCoreVault(
        uint256 _lots,
        string memory _redeemerUnderlyingAddress
    )
        external
        onlyEnabled
        notEmergencyPaused
        nonReentrant
    {
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        uint256 availableLots = CoreVaultClient.coreVaultAmountLots();
        require(_lots <= availableLots, NotEnoughAvailableOnCoreVault());
        uint256 minimumRedeemLots = Math.min(state.minimumRedeemLots, availableLots);
        require(_lots >= minimumRedeemLots, RequestedAmountTooSmall());
        // burn the senders fassets
        uint256 redeemedUBA = Conversion.convertLotsToUBA(_lots);
        Redemptions.burnFAssets(msg.sender, redeemedUBA);
        // subtract the redemption fee
        uint256 redemptionFeeUBA = redeemedUBA.mulBips(state.redemptionFeeBIPS);
        uint128 paymentUBA = (redeemedUBA - redemptionFeeUBA).toUint128();
        // create new request id
        state.newRedemptionFromCoreVaultId += PaymentReference.randomizedIdSkip();
        bytes32 paymentReference = PaymentReference.redemptionFromCoreVault(state.newRedemptionFromCoreVaultId);
        // transfer from core vault (paymentReference may change when the request is merged)
        paymentReference = state.coreVaultManager.requestTransferFromCoreVault(
            _redeemerUnderlyingAddress, paymentReference, paymentUBA, false);
        emit CoreVaultRedemptionRequested(msg.sender, _redeemerUnderlyingAddress, paymentReference,
            redeemedUBA, redemptionFeeUBA);
    }

    function maximumTransferToCoreVault(
        address _agentVault
    )
        external view
        returns (uint256 _maximumTransferUBA, uint256 _minimumLeftAmountUBA)
    {
        Agent.State storage agent = Agent.get(_agentVault);
        (uint256 _maximumTransferAMG, uint256 _minimumLeftAmountAMG) =
             CoreVaultClient.maximumTransferToCoreVaultAMG(agent);
        _maximumTransferUBA = Conversion.convertAmgToUBA(_maximumTransferAMG.toUint64());
        _minimumLeftAmountUBA = Conversion.convertAmgToUBA(_minimumLeftAmountAMG.toUint64());
    }

    function coreVaultAvailableAmount()
        external view
        returns (uint256 _immediatelyAvailableUBA, uint256 _totalAvailableUBA)
    {
        return CoreVaultClient.coreVaultAvailableAmount();
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {AssetManagerBase} from "./AssetManagerBase.sol";
import {CoreVaultClient} from "../library/CoreVaultClient.sol";
import {IICoreVaultManager} from "../../coreVaultManager/interfaces/IICoreVaultManager.sol";
import {LibDiamond} from "../../diamond/library/LibDiamond.sol";
import {GovernedProxyImplementation} from "../../governance/implementation/GovernedProxyImplementation.sol";
import {ICoreVaultClientSettings} from "../../userInterfaces/ICoreVaultClientSettings.sol";
import {IAssetManager} from "../../userInterfaces/IAssetManager.sol";
import {IAssetManagerEvents} from "../../userInterfaces/IAssetManagerEvents.sol";
import {ICoreVaultClient} from "../../userInterfaces/ICoreVaultClient.sol";
import {SafePct} from "../../utils/library/SafePct.sol";


contract CoreVaultClientSettingsFacet is AssetManagerBase, GovernedProxyImplementation, ICoreVaultClientSettings {
    using SafeCast for uint256;

    error WrongAssetManager();
    error CannotDisable();
    error DiamondNotInitialized();
    error AlreadyInitialized();
    error BipsValueTooHigh();

    // prevent initialization of implementation contract
    constructor() {
        CoreVaultClient.getState().initialized = true;
    }

    function initCoreVaultFacet(
        IICoreVaultManager _coreVaultManager,
        address payable _nativeAddress,
        uint256 _transferTimeExtensionSeconds,
        uint256 _redemptionFeeBIPS,
        uint256 _minimumAmountLeftBIPS,
        uint256 _minimumRedeemLots
    )
        external
    {
        updateInterfacesAtCoreVaultDeploy();
        // init settings
        require(_redemptionFeeBIPS <= SafePct.MAX_BIPS, BipsValueTooHigh());
        require(_minimumAmountLeftBIPS <= SafePct.MAX_BIPS, BipsValueTooHigh());
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        require(!state.initialized, AlreadyInitialized());
        state.initialized = true;
        state.coreVaultManager = _coreVaultManager;
        state.nativeAddress = _nativeAddress;
        state.transferTimeExtensionSeconds = _transferTimeExtensionSeconds.toUint64();
        state.redemptionFeeBIPS = _redemptionFeeBIPS.toUint16();
        state.minimumAmountLeftBIPS = _minimumAmountLeftBIPS.toUint16();
        state.minimumRedeemLots = _minimumRedeemLots.toUint64();
    }

    function updateInterfacesAtCoreVaultDeploy()
        public
    {
        LibDiamond.DiamondStorage storage ds = LibDiamond.diamondStorage();
        require(ds.supportedInterfaces[type(IERC165).interfaceId], DiamondNotInitialized());
        // IAssetManager has new methods (at CoreVaultClient deploy on Songbird)
        ds.supportedInterfaces[type(IAssetManager).interfaceId] = true;
        // Core Vault interfaces added
        ds.supportedInterfaces[type(ICoreVaultClient).interfaceId] = true;
        ds.supportedInterfaces[type(ICoreVaultClientSettings).interfaceId] = true;
    }

    ///////////////////////////////////////////////////////////////////////////////////
    // Settings

    function setCoreVaultManager(
        address _coreVaultManager
    )
        external
        onlyGovernance
    {
        // core vault cannot be disabled once it has been enabled (it can be disabled initially
        // in initCoreVaultFacet method, for chains where core vault is not supported)
        require(_coreVaultManager != address(0), CannotDisable());
        IICoreVaultManager coreVaultManager = IICoreVaultManager(_coreVaultManager);
        require(coreVaultManager.assetManager() == address(this), WrongAssetManager());
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        state.coreVaultManager = coreVaultManager;
        emit IAssetManagerEvents.ContractChanged("coreVaultManager", _coreVaultManager);
    }

    function setCoreVaultNativeAddress(
        address payable _nativeAddress
    )
        external
        onlyImmediateGovernance
    {
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        state.nativeAddress = _nativeAddress;
        // not really a contract, but works for any address - event name is a bit unfortunate
        // but we don't want to change it now to keep backward compatibility
        emit IAssetManagerEvents.ContractChanged("coreVaultNativeAddress", _nativeAddress);
    }

    function setCoreVaultTransferTimeExtensionSeconds(
        uint256 _transferTimeExtensionSeconds
    )
        external
        onlyImmediateGovernance
    {
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        state.transferTimeExtensionSeconds = _transferTimeExtensionSeconds.toUint64();
        emit IAssetManagerEvents.SettingChanged("coreVaultTransferTimeExtensionSeconds",
            _transferTimeExtensionSeconds);
    }

    function setCoreVaultRedemptionFeeBIPS(
        uint256 _redemptionFeeBIPS
    )
        external
        onlyImmediateGovernance
    {
        require(_redemptionFeeBIPS <= SafePct.MAX_BIPS, BipsValueTooHigh());
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        state.redemptionFeeBIPS = _redemptionFeeBIPS.toUint16();
        emit IAssetManagerEvents.SettingChanged("coreVaultRedemptionFeeBIPS", _redemptionFeeBIPS);
    }

    function setCoreVaultMinimumAmountLeftBIPS(
        uint256 _minimumAmountLeftBIPS
    )
        external
        onlyImmediateGovernance
    {
        require(_minimumAmountLeftBIPS <= SafePct.MAX_BIPS, BipsValueTooHigh());
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        state.minimumAmountLeftBIPS = _minimumAmountLeftBIPS.toUint16();
        emit IAssetManagerEvents.SettingChanged("coreVaultMinimumAmountLeftBIPS", _minimumAmountLeftBIPS);
    }

    function setCoreVaultMinimumRedeemLots(
        uint256 _minimumRedeemLots
    )
        external
        onlyImmediateGovernance
    {
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        state.minimumRedeemLots = _minimumRedeemLots.toUint64();
        emit IAssetManagerEvents.SettingChanged("coreVaultMinimumRedeemLots", _minimumRedeemLots);
    }

    function getCoreVaultManager()
        external view
        returns (address)
    {
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        return address(state.coreVaultManager);
    }

    function getCoreVaultNativeAddress()
        external view
        returns (address)
    {
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        return state.nativeAddress;
    }

    function getCoreVaultTransferTimeExtensionSeconds()
        external view
        returns (uint256)
    {
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        return state.transferTimeExtensionSeconds;
    }

    function getCoreVaultRedemptionFeeBIPS()
        external view
        returns (uint256)
    {
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        return state.redemptionFeeBIPS;
    }

    function getCoreVaultMinimumAmountLeftBIPS()
        external view
        returns (uint256)
    {
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        return state.minimumAmountLeftBIPS;
    }

    function getCoreVaultMinimumRedeemLots()
        external view
        returns (uint256)
    {
        CoreVaultClient.State storage state = CoreVaultClient.getState();
        return state.minimumRedeemLots;
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IAgentOwnerRegistry} from "../../userInterfaces/IAgentOwnerRegistry.sol";
import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {GovernedUUPSProxyImplementation} from "../../governance/implementation/GovernedUUPSProxyImplementation.sol";
import {IGovernanceSettings} from "@flarenetwork/flare-periphery-contracts/flare/IGovernanceSettings.sol";


contract AgentOwnerRegistry is GovernedUUPSProxyImplementation, IERC165, IAgentOwnerRegistry {

    event ManagerChanged(address manager);

    error AddressZero();
    error OnlyGovernanceOrManager();

    /**
     * When nonzero, this is the address that can perform whitelisting operations
     * instead of the governance.
     */
    address public manager;

    mapping(address => bool) private whitelist;

    mapping(address => address) private workToMgmtAddress;
    mapping(address => address) private mgmtToWorkAddress;

    mapping(address => string) private agentName;
    mapping(address => string) private agentDescription;
    mapping(address => string) private agentIconUrl;
    mapping(address => string) private agentTouUrl;

    modifier onlyGovernanceOrManager {
        require(msg.sender == manager || msg.sender == governance(), OnlyGovernanceOrManager());
        _;
    }

    function initialize(IGovernanceSettings _governanceSettings, address _initialGovernance) external {
        initialise(_governanceSettings, _initialGovernance);    // also marks as initialized
    }

    function revokeAddress(address _address) external onlyGovernanceOrManager {
        _removeAddressFromWhitelist(_address);
    }

    function setManager(address _manager) external onlyGovernance {
        manager = _manager;
        emit ManagerChanged(_manager);
    }

    /**
     * Add agent to the whitelist and set data for agent presentation.
     * If the agent is already whitelisted, only updates agent presentation data.
     * @param _managementAddress the agent owner's address
     * @param _name agent owner's name
     * @param _description agent owner's description
     * @param _iconUrl url of the agent owner's icon image; governance or manager should check it is in correct format
     *      and size and it is on a server where it cannot change or be deleted
     * @param _touUrl url of the agent's page with terms of use; similar considerations apply as for icon url
     */
    function whitelistAndDescribeAgent(
        address _managementAddress,
        string memory _name,
        string memory _description,
        string memory _iconUrl,
        string memory _touUrl
    )
        external
        onlyGovernanceOrManager
    {
        _addAddressToWhitelist(_managementAddress);
        _setAgentData(_managementAddress, _name, _description, _iconUrl, _touUrl);
    }

    /**
     * Associate a work address with the agent owner's management address.
     * Every owner (management address) can have only one work address, so as soon as the new one is set, the old
     * one stops working.
     * NOTE: May only be called by an agent on the allowed agent list and only from the management address.
     */
    function setWorkAddress(address _ownerWorkAddress)
        external
    {
        require(isWhitelisted(msg.sender), AgentNotWhitelisted());
        require(_ownerWorkAddress == address(0) || workToMgmtAddress[_ownerWorkAddress] == address(0),
               WorkAddressInUse());
        // delete old work to management mapping
        address oldWorkAddress = mgmtToWorkAddress[msg.sender];
        if (oldWorkAddress != address(0)) {
            workToMgmtAddress[oldWorkAddress] = address(0);
        }
        // create a new bidirectional mapping
        mgmtToWorkAddress[msg.sender] = _ownerWorkAddress;
        if (_ownerWorkAddress != address(0)) {
            workToMgmtAddress[_ownerWorkAddress] = msg.sender;
        }
        emit WorkAddressChanged(msg.sender, oldWorkAddress, _ownerWorkAddress);
    }

    /**
     * Set agent owner's name.
     * @param _managementAddress agent owner's management address
     * @param _name new agent owner's name
     */
    function setAgentName(address _managementAddress, string memory _name)
        external
        onlyGovernanceOrManager
    {
        agentName[_managementAddress] = _name;
        _emitDataChanged(_managementAddress);
    }

    /**
     * Set agent owner's description.
     * @param _managementAddress agent owner's management address
     * @param _description new agent owner's description
     */
    function setAgentDescription(address _managementAddress, string memory _description)
        external
        onlyGovernanceOrManager
    {
        agentDescription[_managementAddress] = _description;
        _emitDataChanged(_managementAddress);
    }

    /**
     * Set url of the agent owner's icon.
     * @param _managementAddress agent owner's management address
     * @param _iconUrl new url of the agent owner's icon
     */
    function setAgentIconUrl(address _managementAddress, string memory _iconUrl)
        external
        onlyGovernanceOrManager
    {
        agentIconUrl[_managementAddress] = _iconUrl;
        _emitDataChanged(_managementAddress);
    }

    /**
     * Set url of the agent's page with terms of use.
     * @param _managementAddress agent owner's management address
     * @param _touUrl new url of the agent's page with terms of use
     */
    function setAgentTermsOfUseUrl(address _managementAddress, string memory _touUrl)
        external
        onlyGovernanceOrManager
    {
        agentTouUrl[_managementAddress] = _touUrl;
        _emitDataChanged(_managementAddress);
    }

    /**
     * Return agent owner's name.
     * @param _managementAddress agent owner's management address
     */
    function getAgentName(address _managementAddress)
        external view override
        returns (string memory)
    {
        return agentName[_managementAddress];
    }

    /**
     * Return agent owner's description.
     * @param _managementAddress agent owner's management address
     */
    function getAgentDescription(address _managementAddress)
        external view override
        returns (string memory)
    {
        return agentDescription[_managementAddress];
    }

    /**
     * Return url of the agent owner's icon.
     * @param _managementAddress agent owner's management address
     */
    function getAgentIconUrl(address _managementAddress)
        external view override
        returns (string memory)
    {
        return agentIconUrl[_managementAddress];
    }

    /**
     * Return url of the agent's page with terms of use.
     * @param _managementAddress agent owner's management address
     */
    function getAgentTermsOfUseUrl(address _managementAddress)
        external view override
        returns (string memory)
    {
        return agentTouUrl[_managementAddress];
    }

    /**
     * Get the (unique) work address for the given management address.
     */
    function getWorkAddress(address _managementAddress)
        external view override
        returns (address)
    {
        return mgmtToWorkAddress[_managementAddress];
    }

    /**
     * Get the (unique) management address for the given work address.
     */
    function getManagementAddress(address _workAddress)
        external view override
        returns (address)
    {
        return workToMgmtAddress[_workAddress];
    }

    function isWhitelisted(address _address) public view override returns (bool) {
        return whitelist[_address];
    }

    function _addAddressToWhitelist(address _address) internal {
        require(_address != address(0), AddressZero());
        if (whitelist[_address]) return;
        whitelist[_address] = true;
        emit Whitelisted(_address);
    }

    function _removeAddressFromWhitelist(address _address) internal {
        if (!whitelist[_address]) return;
        delete whitelist[_address];
        emit WhitelistingRevoked(_address);
    }

    function _setAgentData(
        address _managementAddress,
        string memory _name,
        string memory _description,
        string memory _iconUrl,
        string memory _touUrl
    ) private {
        agentName[_managementAddress] = _name;
        agentDescription[_managementAddress] = _description;
        agentIconUrl[_managementAddress] = _iconUrl;
        agentTouUrl[_managementAddress] = _touUrl;
        emit AgentDataChanged(_managementAddress, _name, _description, _iconUrl, _touUrl);
    }

    function _emitDataChanged(address _managementAddress) private {
        emit AgentDataChanged(_managementAddress,
            agentName[_managementAddress],
            agentDescription[_managementAddress],
            agentIconUrl[_managementAddress],
            agentTouUrl[_managementAddress]);
    }

    /**
     * Implementation of ERC-165 interface.
     */
    function supportsInterface(bytes4 _interfaceId)
        public pure override
        returns (bool)
    {
        return _interfaceId == type(IERC165).interfaceId
            || _interfaceId == type(IAgentOwnerRegistry).interfaceId;
    }
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
pragma solidity ^0.8.27;

import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {AssetManagerBase} from "./AssetManagerBase.sol";
import {Globals} from "../library/Globals.sol";
import {SettingsUpdater} from "../library/SettingsUpdater.sol";
import {RedemptionTimeExtension} from "../library/data/RedemptionTimeExtension.sol";
import {LibDiamond} from "../../diamond/library/LibDiamond.sol";
import {AssetManagerSettings} from "../../userInterfaces/data/AssetManagerSettings.sol";
import {IAssetManagerEvents} from "../../userInterfaces/IAssetManagerEvents.sol";
import {IRedemptionTimeExtension} from "../../userInterfaces/IRedemptionTimeExtension.sol";


contract RedemptionTimeExtensionFacet is AssetManagerBase, IRedemptionTimeExtension {

    error ValueMustBeNonzero();
    error DecreaseTooBig();
    error IncreaseTooBig();
    error AlreadyInitialized();
    error DiamondNotInitialized();

    constructor() {
        // implementation initialization - to prevent reinitialization
        RedemptionTimeExtension.setRedemptionPaymentExtensionSeconds(1);
    }

    // this method is not accessible through diamond proxy
    // it is only used for initialization when the contract is added after proxy deploy
    function initRedemptionTimeExtensionFacet(uint256 _redemptionPaymentExtensionSeconds)
        external
    {
        LibDiamond.DiamondStorage storage ds = LibDiamond.diamondStorage();
        require(ds.supportedInterfaces[type(IERC165).interfaceId], DiamondNotInitialized());
        ds.supportedInterfaces[type(IRedemptionTimeExtension).interfaceId] = true;
        require(RedemptionTimeExtension.redemptionPaymentExtensionSeconds() == 0, AlreadyInitialized());
        // init settings
        RedemptionTimeExtension.setRedemptionPaymentExtensionSeconds(_redemptionPaymentExtensionSeconds);
    }

    function setRedemptionPaymentExtensionSeconds(uint256 _value)
        external
        onlyAssetManagerController
    {
        SettingsUpdater.checkEnoughTimeSinceLastUpdate();
        // validate
        AssetManagerSettings.Data storage settings = Globals.getSettings();
        uint256 currentValue = RedemptionTimeExtension.redemptionPaymentExtensionSeconds();
        require(_value <= currentValue * 4 + settings.averageBlockTimeMS / 1000, IncreaseTooBig());
        require(_value >= currentValue / 4, DecreaseTooBig());
        require(_value > 0, ValueMustBeNonzero());
        // update
        RedemptionTimeExtension.setRedemptionPaymentExtensionSeconds(_value);
        emit IAssetManagerEvents.SettingChanged("redemptionPaymentExtensionSeconds", _value);
    }

    function redemptionPaymentExtensionSeconds()
        external view
        returns (uint256)
    {
        return RedemptionTimeExtension.redemptionPaymentExtensionSeconds();
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Diamond} from "../../diamond/implementation/Diamond.sol";
import {LibDiamond} from "../../diamond/library/LibDiamond.sol";
import {IAssetManagerEvents} from "../../userInterfaces/IAssetManagerEvents.sol";
import {IDiamondCut} from "../../diamond/interfaces/IDiamondCut.sol";

/**
 * The contract that can mint and burn f-assets while managing collateral and backing funds.
 * There is one instance of AssetManager per f-asset type.
 */
contract AssetManager is Diamond, IAssetManagerEvents {
    // IAssetManagerEvents interface is included so that blockchain explorers will be able
    // to decode events for verified AssetManager instances.
    constructor(IDiamondCut.FacetCut[] memory _diamondCut, address _init, bytes memory _initCalldata) payable {
        LibDiamond.diamondCut(_diamondCut, _init, _initCalldata);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {Math} from "@openzeppelin/contracts/utils/math/Math.sol";
import {ReentrancyGuard} from "../../openzeppelin/security/ReentrancyGuard.sol";
import {SafePct} from "../../utils/library/SafePct.sol";
import {Transfers} from "../../utils/library/Transfers.sol";
import {MathUtils} from "../../utils/library/MathUtils.sol";
import {IFAsset} from "../../userInterfaces/IFAsset.sol";
import {IWNat} from "../../flareSmartContracts/interfaces/IWNat.sol";
import {IIAssetManager} from "../../assetManager/interfaces/IIAssetManager.sol";
import {IICollateralPool} from "../../collateralPool/interfaces/IICollateralPool.sol";
import {IICollateralPoolToken} from "../interfaces/IICollateralPoolToken.sol";
import {ICollateralPoolToken} from "../../userInterfaces/ICollateralPoolToken.sol";
import {IRewardManager} from "@flarenetwork/flare-periphery-contracts/flare/IRewardManager.sol";
import {IDistributionToDelegators} from "@flarenetwork/flare-periphery-contracts/flare/IDistributionToDelegators.sol";
import {ICollateralPool} from "../../userInterfaces/ICollateralPool.sol";


//slither-disable reentrancy    // all possible reentrancies guarded by nonReentrant
contract CollateralPool is IICollateralPool, ReentrancyGuard, UUPSUpgradeable, IERC165 {
    using SafeCast for uint256;
    using SafeCast for int256;
    using SafePct for uint256;
    using SafeERC20 for IFAsset;
    using SafeERC20 for IWNat;

    struct AssetPrice {
        uint256 mul;
        uint256 div;
    }

    uint256 public constant MIN_NAT_TO_ENTER = 1 ether;
    uint256 public constant MIN_TOKEN_SUPPLY_AFTER_EXIT = 1 ether;
    uint256 public constant MIN_NAT_BALANCE_AFTER_EXIT = 1 ether;

    address public agentVault;          // practically immutable because there is no setter
    IIAssetManager public assetManager; // practically immutable because there is no setter
    IFAsset public fAsset;              // practically immutable because there is no setter
    IICollateralPoolToken public token; // only changed once at deploy time

    IWNat public wNat;
    uint32 public exitCollateralRatioBIPS;

    uint32 private __topupCollateralRatioBIPS; // only storage placeholder
    uint16 private __topupTokenPriceFactorBIPS; // only storage placeholder

    bool private internalWithdrawal;
    bool private initialized;

    mapping(address => int256) private _fAssetFeeDebtOf;
    int256 public totalFAssetFeeDebt;
    uint256 public totalFAssetFees;
    uint256 public totalCollateral;

    modifier onlyAssetManager {
        require(msg.sender == address(assetManager), OnlyAssetManager());
        _;
    }

    modifier onlyAgent {
        require(isAgentVaultOwner(msg.sender), OnlyAgent());
        _;
    }

    // Only used in some tests.
    // The implementation in production will always be deployed with all zero addresses and parameters.
    constructor (
        address _agentVault,
        address _assetManager,
        address _fAsset,
        uint32 _exitCollateralRatioBIPS
    ) {
        initialize(_agentVault, _assetManager, _fAsset, _exitCollateralRatioBIPS);
    }

    function initialize(
        address _agentVault,
        address _assetManager,
        address _fAsset,
        uint32 _exitCollateralRatioBIPS
    )
        public
    {
        require(!initialized, AlreadyInitialized());
        initialized = true;
        // init vars
        agentVault = _agentVault;
        assetManager = IIAssetManager(_assetManager);
        fAsset = IFAsset(_fAsset);
        // for proxy implementation, assetManager will be 0
        wNat = address(assetManager) != address(0) ? assetManager.getWNat() : IWNat(address(0));
        exitCollateralRatioBIPS = _exitCollateralRatioBIPS;
        initializeReentrancyGuard();
    }

    receive() external payable {
        require(internalWithdrawal, OnlyInternalUse());
    }

    function setPoolToken(address _poolToken)
        external
        onlyAssetManager
    {
        require(address(token) == address(0), PoolTokenAlreadySet());
        token = IICollateralPoolToken(_poolToken);
    }

    /**
     * @notice Returns the collateral pool token contract used by this contract
     */
    function poolToken() external view returns (ICollateralPoolToken) {
        return token;
    }

    function setExitCollateralRatioBIPS(uint256 _exitCollateralRatioBIPS)
        external
        onlyAssetManager
    {
        exitCollateralRatioBIPS = _exitCollateralRatioBIPS.toUint32();
    }

    /**
     * @notice Enters the collateral pool by depositing some NAT
     */
    // slither-disable-next-line reentrancy-eth         // guarded by nonReentrant
    function enter()
        external payable
        nonReentrant
        returns (uint256, uint256)
    {
        require(msg.value >= MIN_NAT_TO_ENTER, AmountOfNatTooLow());
        uint256 totalPoolTokens = token.totalSupply();
        if (totalPoolTokens == 0) {
            // this conditions are set for keeping a stable token value
            require(msg.value >= totalCollateral, AmountOfCollateralTooLow());
            AssetPrice memory assetPrice = _getAssetPrice();
            require(msg.value >= totalFAssetFees.mulDiv(assetPrice.mul, assetPrice.div), AmountOfCollateralTooLow());
        }
        // calculate obtained pool tokens and free f-assets
        uint256 tokenShare = _collateralToTokenShare(msg.value);
        require(tokenShare > 0, DepositResultsInZeroTokens());
        // calculate and create fee debt
        uint256 feeDebt = totalPoolTokens > 0 ? _totalVirtualFees().mulDiv(tokenShare, totalPoolTokens) : 0;
        _createFAssetFeeDebt(msg.sender, feeDebt);
        // deposit collateral
        _depositWNat();
        // mint pool tokens to the sender
        uint256 timelockExp = token.mint(msg.sender, tokenShare);
        // emit event
        emit CPEntered(msg.sender, msg.value, tokenShare, timelockExp);
        return (tokenShare, timelockExp);
    }

    /**
     * @notice Exits the pool by liquidating the given amount of pool tokens
     * @param _tokenShare   The amount of pool tokens to be liquidated
     *                      Must be positive and smaller or equal to the sender's token balance
     */
    // slither-disable-next-line reentrancy-eth         // guarded by nonReentrant
    function exit(uint256 _tokenShare)
        external
        nonReentrant
        returns (uint256)
    {
        return _exitTo(_tokenShare, payable(msg.sender));
    }

    /**
     * @notice Exits the pool by liquidating the given amount of pool tokens
     * @param _tokenShare   The amount of pool tokens to be liquidated
     *                      Must be positive and smaller or equal to the sender's token balance
     * @param _recipient    The address to which NATs and FAsset fees will be transferred
     */
    // slither-disable-next-line reentrancy-eth         // guarded by nonReentrant
    function exitTo(uint256 _tokenShare, address payable _recipient)
        external
        nonReentrant
        returns (uint256)
    {
        return _exitTo(_tokenShare, _recipient);
    }

    // slither-disable-next-line reentrancy-eth         // guarded by nonReentrant
    function _exitTo(uint256 _tokenShare, address payable _recipient)
        private
        returns (uint256)
    {
        require(_tokenShare > 0, TokenShareIsZero());
        require(_tokenShare <= token.balanceOf(msg.sender), TokenBalanceTooLow());
        _requireMinTokenSupplyAfterExit(_tokenShare);
        // token.totalSupply() >= token.balanceOf(msg.sender) >= _tokenShare > 0
        uint256 natShare = totalCollateral.mulDiv(_tokenShare, token.totalSupply());
        require(natShare > 0, SentAmountTooLow());
        _requireMinNatSupplyAfterExit(natShare);
        require(_staysAboveExitCR(natShare), CollateralRatioFallsBelowExitCR());
        // update the fasset fee debt
        uint256 debtFAssetFeeShare = _tokensToVirtualFeeShare(_tokenShare);
        _deleteFAssetFeeDebt(msg.sender, debtFAssetFeeShare);
        token.burn(msg.sender, _tokenShare, false);
        _withdrawWNatTo(_recipient, natShare);
        // emit event
        emit CPExited(msg.sender, _tokenShare, natShare);
        return natShare;
    }

    /**
     * @notice Exits the pool by liquidating the given amount of pool tokens and redeeming
     *  f-assets in a way that either preserves the pool collateral ratio or keeps it above exit CR
     * @param _tokenShare                   The amount of pool tokens to be liquidated
     *                                      Must be positive and smaller or equal to the sender's token balance
     * @param _redeemToCollateral           Specifies if redeemed f-assets should be exchanged to vault collateral
     *                                      by the agent
     * @param _redeemerUnderlyingAddress    Redeemer's address on the underlying chain
     * @param _executor                     The account that is allowed to execute redemption default
     * @notice F-assets will be redeemed in collateral if their value does not exceed one lot
     * @notice All f-asset fees will be redeemed along with potential additionally required f-assets taken
     *  from the sender's f-asset account
     */
    // slither-disable-next-line reentrancy-eth         // guarded by nonReentrant
    function selfCloseExit(
        uint256 _tokenShare,
        bool _redeemToCollateral,
        string memory _redeemerUnderlyingAddress,
        address payable _executor
    )
        external payable
        nonReentrant
    {
        _selfCloseExitTo(_tokenShare, _redeemToCollateral, payable(msg.sender), _redeemerUnderlyingAddress, _executor);
    }

    /**
     * @notice Exits the pool by liquidating the given amount of pool tokens and redeeming
     *  f-assets in a way that either preserves the pool collateral ratio or keeps it above exit CR
     * @param _tokenShare                   The amount of pool tokens to be liquidated
     *                                      Must be positive and smaller or equal to the sender's token balance
     * @param _redeemToCollateral           Specifies if redeemed f-assets should be exchanged to vault collateral
     *                                      by the agent
     * @param _recipient                    The address to which NATs and FAsset fees will be transferred
     * @param _redeemerUnderlyingAddress    Redeemer's address on the underlying chain
     * @param _executor                     The account that is allowed to execute redemption default
     * @notice F-assets will be redeemed in collateral if their value does not exceed one lot
     * @notice All f-asset fees will be redeemed along with potential additionally required f-assets taken
     *  from the sender's f-asset account
     */
    // slither-disable-next-line reentrancy-eth         // guarded by nonReentrant
    function selfCloseExitTo(
        uint256 _tokenShare,
        bool _redeemToCollateral,
        address payable _recipient,
        string memory _redeemerUnderlyingAddress,
        address payable _executor
    )
        external payable
        nonReentrant
    {
        require(_recipient != address(0) && _recipient != address(this) && _recipient != agentVault,
            InvalidRecipientAddress());
        _selfCloseExitTo(_tokenShare, _redeemToCollateral, _recipient, _redeemerUnderlyingAddress, _executor);
    }

    // slither-disable-next-line reentrancy-eth         // guarded by nonReentrant
    function _selfCloseExitTo(
        uint256 _tokenShare,
        bool _redeemToCollateral,
        address payable _recipient,
        string memory _redeemerUnderlyingAddress,
        address payable _executor
    )
        private
    {
        require(_tokenShare > 0, TokenShareIsZero());
        require(_tokenShare <= token.balanceOf(msg.sender), TokenBalanceTooLow());
        _requireMinTokenSupplyAfterExit(_tokenShare);
        // token.totalSupply() >= token.balanceOf(msg.sender) >= _tokenShare > 0
        uint256 natShare = totalCollateral.mulDiv(_tokenShare, token.totalSupply());
        require(natShare > 0, SentAmountTooLow());
        _requireMinNatSupplyAfterExit(natShare);
        uint256 maxAgentRedemption = assetManager.maxRedemptionFromAgent(agentVault);
        uint256 requiredFAssets = _getFAssetRequiredToNotSpoilCR(natShare);
        // Rare case: if agent has too many low-valued open tickets they can't redeem the requiredFAssets
        // in one transaction. In that case, we revert and the user should retry with lower amount.
        require(maxAgentRedemption > requiredFAssets, RedemptionRequiresClosingTooManyTickets());
        // get owner f-asset fees to be spent (maximize fee withdrawal to cover the potentially necessary f-assets)
        uint256 debtFAssetFeeShare = _tokensToVirtualFeeShare(_tokenShare);
        // transfer the owner's fassets that will be redeemed
        require(fAsset.allowance(msg.sender, address(this)) >= requiredFAssets, FAssetAllowanceTooSmall());
        fAsset.safeTransferFrom(msg.sender, address(this), requiredFAssets);
        // redeem f-assets if necessary
        bool returnFunds = true;
        if (requiredFAssets > 0) {
            if (requiredFAssets < assetManager.lotSize() || _redeemToCollateral) {
                assetManager.redeemFromAgentInCollateral(agentVault, _recipient, requiredFAssets);
            } else {
                returnFunds = _executor == address(0);
                // pass `msg.value` to `redeemFromAgent` for the executor fee if `_executor` is set
                assetManager.redeemFromAgent{ value: returnFunds ? 0 : msg.value }(
                    agentVault, _recipient, requiredFAssets, _redeemerUnderlyingAddress, _executor);
            }
        }
        _deleteFAssetFeeDebt(msg.sender, debtFAssetFeeShare);
        token.burn(msg.sender, _tokenShare, false);
        _withdrawWNatTo(_recipient, natShare);
        if (returnFunds) {
            // return any NAT included by mistake to the recipient
            Transfers.transferNAT(_recipient, msg.value);
        }
        // emit event
        emit CPSelfCloseExited(msg.sender, _tokenShare, natShare, requiredFAssets);
    }

    /**
     * Get the amount of fassets that need to be burned to perform self close exit.
     */
    function fAssetRequiredForSelfCloseExit(uint256 _tokenAmountWei)
        external view
        returns (uint256)
    {
        uint256 tokenNatWeiEquiv = totalCollateral.mulDiv(_tokenAmountWei, token.totalSupply());
        return _getFAssetRequiredToNotSpoilCR(tokenNatWeiEquiv);
    }

    /**
     * @notice Collect f-asset fees by locking free tokens
     * @param _fAssets  The amount of f-asset fees to withdraw
     *                  Must be positive and smaller or equal to the sender's reward f-assets
     */
    function withdrawFees(uint256 _fAssets)
        external
        nonReentrant
    {
        _withdrawFeesTo(_fAssets, msg.sender);
    }

    /**
     * @notice Collect f-asset fees by locking free tokens
     * @param _fAssets      The amount of f-asset fees to withdraw
     *                      Must be positive and smaller or equal to the sender's fAsset fees.
     * @param _recipient    The address to which FAsset fees will be transferred
     */
    function withdrawFeesTo(uint256 _fAssets, address _recipient)
        external
        nonReentrant
    {
        _withdrawFeesTo(_fAssets, _recipient);
    }

    /**
     * @notice Collect f-asset fees by locking free tokens
     * @param _fAssets      The amount of f-asset fees to withdraw
     *                      Must be positive and smaller or equal to the sender's reward f-assets
     * @param _recipient    The address to which NATs and FAsset fees will be transferred
     */
    function _withdrawFeesTo(uint256 _fAssets, address _recipient)
        private
    {
        require(_fAssets > 0, WithdrawZeroFAsset());
        uint256 freeFAssetFeeShare = _fAssetFeesOf(msg.sender);
        require(_fAssets <= freeFAssetFeeShare, FreeFAssetBalanceTooSmall());
        _createFAssetFeeDebt(msg.sender, _fAssets);
        _transferFAssetTo(_recipient, _fAssets);
        // emit event
        emit CPFeesWithdrawn(msg.sender, _fAssets);
    }

    /**
     * @notice Free debt tokens by paying f-assets
     * @param _fAssets  Amount of payed f-assets
     *                  _fAssets must be positive and smaller or equal to the sender's debt f-assets
     */
    function payFAssetFeeDebt(uint256 _fAssets)
        external
        nonReentrant
    {
        require(_fAssets != 0, ZeroFAssetDebtPayment());
        require(_fAssets.toInt256() <= _fAssetFeeDebtOf[msg.sender], PaymentLargerThanFeeDebt());
        require(fAsset.allowance(msg.sender, address(this)) >= _fAssets, FAssetAllowanceTooSmall());
        _deleteFAssetFeeDebt(msg.sender, _fAssets);
        _transferFAssetFrom(msg.sender, _fAssets);
        // emit event
        emit CPFeeDebtPaid(msg.sender, _fAssets);
    }

    // support for liquidation / redemption default payments
    // slither-disable-next-line reentrancy-eth         // guarded by nonReentrant
    function payout(
        address _recipient,
        uint256 _amount,
        uint256 _agentResponsibilityWei
    )
        external
        onlyAssetManager
        nonReentrant
    {
        // slash agent vault's pool tokens worth _agentResponsibilityWei in FLR (or less if there is not enough)
        uint256 agentTokenBalance = token.balanceOf(agentVault);
        uint256 maxSlashedTokens = totalCollateral > 0 ?
            token.totalSupply().mulDivRoundUp(_agentResponsibilityWei, totalCollateral) : agentTokenBalance;
        uint256 slashedTokens = Math.min(maxSlashedTokens, agentTokenBalance);
        if (slashedTokens > 0) {
            uint256 debtFAssetFeeShare = _tokensToVirtualFeeShare(slashedTokens);
            _deleteFAssetFeeDebt(agentVault, debtFAssetFeeShare);
            token.burn(agentVault, slashedTokens, true);
        }
        // transfer collateral to the recipient
        _transferWNatTo(_recipient, _amount);
        emit CPPaidOut(_recipient, _amount, slashedTokens);
    }

    function _collateralToTokenShare(
        uint256 _collateral
    )
        internal view
        returns (uint256)
    {
        uint256 totalPoolTokens = token.totalSupply();
        if (totalCollateral == 0 || totalPoolTokens == 0) { // pool is empty
            return _collateral;
        }
        return totalPoolTokens.mulDiv(_collateral, totalCollateral);
    }

    // _tokens is assumed to be smaller or equal to _account's token balance
    function _tokensToVirtualFeeShare(
        uint256 _tokens
    )
        internal view
        returns (uint256)
    {
        if (_tokens == 0) return 0;
        uint256 totalPoolTokens = token.totalSupply();
        assert(_tokens <= totalPoolTokens);
        // poolTokenSupply >= _tokens AND _tokens > 0 together imply poolTokenSupply != 0
        return _totalVirtualFees().mulDiv(_tokens, totalPoolTokens);
    }

    function _getFAssetRequiredToNotSpoilCR(
        uint256 _natShare
    )
        internal view
        returns (uint256)
    {
        // calculate f-assets required for CR to stay above max(exitCR, poolCR) when taking out _natShare
        // if pool is below exitCR, we shouldn't require it be increased above exitCR, only preserved
        // if pool is above exitCR, we require only for it to stay that way (like in the normal exit)
        AssetPrice memory assetPrice = _getAssetPrice();
        uint256 exitCR = _safeExitCR();
        uint256 backedFAssets = _agentBackedFAssets();
        uint256 resultWithoutRounding;
        if (_isAboveCR(assetPrice, backedFAssets, totalCollateral, exitCR)) {
            // f-asset required for CR to stay above exitCR (might not be needed)
            // solve (N - n) / (p / q (F - f)) >= cr get f = max(0, F - q (N - n) / (p cr))
            // assetPrice.mul > 0, exitCR > 1
            resultWithoutRounding = MathUtils.subOrZero(backedFAssets,
                assetPrice.div * (totalCollateral - _natShare) * SafePct.MAX_BIPS / (assetPrice.mul * exitCR));
        } else {
            // f-asset that preserves pool CR (assume poolNatBalance >= natShare > 0)
            // solve (N - n) / (F - f) = N / F get f = n F / N
            resultWithoutRounding = backedFAssets.mulDivRoundUp(_natShare, totalCollateral);
        }
        return MathUtils.roundUp(resultWithoutRounding, assetManager.assetMintingGranularityUBA());
    }

    function _staysAboveExitCR(
        uint256 _withdrawnNat
    )
        internal view
        returns (bool)
    {
        return _isAboveCR(_getAssetPrice(), _agentBackedFAssets(), totalCollateral - _withdrawnNat, _safeExitCR());
    }

    function _isAboveCR(
        AssetPrice memory _assetPrice,
        uint256 _backedFAssets,
        uint256 _poolCollateralNat,
        uint256 _crBIPS
    )
        internal pure
        returns (bool)
    {
        // check (N - n) / (F p / q) >= cr get (N - n) q >= F p cr
        return _poolCollateralNat * _assetPrice.div >= (_backedFAssets * _assetPrice.mul).mulBips(_crBIPS);
    }

    function _agentBackedFAssets()
        internal view
        returns (uint256)
    {
        return assetManager.getFAssetsBackedByPool(agentVault);
    }

    function _virtualFAssetFeesOf(
        address _account
    )
        internal view
        returns (uint256)
    {
        uint256 tokens = token.balanceOf(_account);
        return _tokensToVirtualFeeShare(tokens);
    }

    function _fAssetFeesOf(
        address _account
    )
        internal view
        returns (uint256)
    {
        int256 virtualFAssetFees = _virtualFAssetFeesOf(_account).toInt256();
        int256 accountFeeDebt = _fAssetFeeDebtOf[_account];
        int256 userFees = virtualFAssetFees - accountFeeDebt;
        // note: rounding errors can make debtFassets larger than virtualFassets by at most one
        // this can happen only when user has no free f-assets (that is why MathUtils.subOrZero)
        // note: rounding errors can make freeFassets larger than total pool f-asset fees by small amounts
        // (The reason for Math.min and Math.positivePart is to restrict to interval [0, poolFAssetFees])
        return Math.min(MathUtils.positivePart(userFees), totalFAssetFees);
    }

    function _debtFreeTokensOf(
        address _account
    )
        internal view
        returns (uint256)
    {
        int256 accountFeeDebt = _fAssetFeeDebtOf[_account];
        if (accountFeeDebt <= 0) {
            // with no debt, all tokens are free
            // this avoids the case where freeFassets == poolVirtualFAssetFees == 0
            return token.balanceOf(_account);
        }
        uint256 virtualFassets = _virtualFAssetFeesOf(_account);
        assert(virtualFassets <= _totalVirtualFees());
        uint256 freeFassets = MathUtils.positivePart(virtualFassets.toInt256() - accountFeeDebt);
        if (freeFassets == 0) return 0;
        // nonzero divisor: _totalVirtualFees() >= virtualFassets >= freeFassets > 0
        return token.totalSupply().mulDiv(freeFassets, _totalVirtualFees());
    }

    function _getAssetPrice()
        internal view
        returns (AssetPrice memory)
    {
        (uint256 assetPriceMul, uint256 assetPriceDiv) = assetManager.assetPriceNatWei();
        return AssetPrice({
            mul: assetPriceMul,
            div: assetPriceDiv
        });
    }

    function _totalVirtualFees()
        internal view
        returns (uint256)
    {
        int256 virtualFees = totalFAssetFees.toInt256() + totalFAssetFeeDebt;
        // Invariant: virtualFees >= 0 always (otherwise the following line will revert).
        // Proof: the places where `totalFAssetFees` and `totalFAssetFeeDebt` change are: `enter`,
        // `exit`/`selfCloseExit`, `withdrawFees` and `payFAssetFeeDebt`.
        // In `withdrawFees` and `payFAssetFeeDebt`, amounts of `totalFAssetFees` and `totalFAssetFeeDebt`
        // change with opposite sign, so virtualFees is unchanged.
        // In `enter`, the `totalFAssetFeeDebt` increases and the other is unchanged, so virtualFees increases.
        // Thus the only place where `totalFAssetFeeDebt` and thus virtualFees decreases is in`exit`/`selfCloseExit`.
        // The decrease there is by `_tokensToVirtualFeeShare()`, which is virtualFees times a factor
        // `tokenShare/totalTokens`, which is checked to be at most 1.
        return virtualFees.toUint256();
    }

    // if governance changes `minPoolCollateralRatioBIPS` it can be higher than `exitCollateralRatioBIPS`
    function _safeExitCR()
        internal view
        returns (uint256)
    {
        uint256 minPoolCollateralRatioBIPS = assetManager.getAgentMinPoolCollateralRatioBIPS(agentVault);
        return Math.max(minPoolCollateralRatioBIPS, exitCollateralRatioBIPS);
    }

    function _requireMinTokenSupplyAfterExit(
        uint256 _tokenShare
    )
        internal view
    {
        uint256 totalPoolTokens = token.totalSupply();
        require(totalPoolTokens == _tokenShare || totalPoolTokens - _tokenShare >= MIN_TOKEN_SUPPLY_AFTER_EXIT,
            TokenSupplyAfterExitTooLow());
    }

    function _requireMinNatSupplyAfterExit(
        uint256 _natShare
    )
        internal view
    {
        require(totalCollateral == _natShare || totalCollateral - _natShare >= MIN_NAT_BALANCE_AFTER_EXIT,
            CollateralAfterExitTooLow());
    }

    ////////////////////////////////////////////////////////////////////////////////////
    // tracking wNat collateral and f-asset fees

    function depositNat()
        external payable
        onlyAssetManager
        nonReentrant
    {
        _depositWNat();
    }

    // this is needed to track asset manager's minting fee deposit
    function fAssetFeeDeposited(
        uint256 _amount
    )
        external
        onlyAssetManager
    {
        totalFAssetFees += _amount;
    }

    function _createFAssetFeeDebt(address _account, uint256 _fAssets)
        internal
    {
        if (_fAssets == 0) return;
        int256 fAssets = _fAssets.toInt256();
        _fAssetFeeDebtOf[_account] += fAssets;
        totalFAssetFeeDebt += fAssets;
        emit CPFeeDebtChanged(_account, _fAssetFeeDebtOf[_account]);
    }

    // _fAssets should be smaller or equal to _account's f-asset debt
    function _deleteFAssetFeeDebt(address _account, uint256 _fAssets)
        internal
    {
        if (_fAssets == 0) return;
        int256 fAssets = _fAssets.toInt256();
        _fAssetFeeDebtOf[_account] -= fAssets;
        totalFAssetFeeDebt -= fAssets;
        emit CPFeeDebtChanged(_account, _fAssetFeeDebtOf[_account]);
    }

    function _transferFAssetFrom(
        address _from,
        uint256 _amount
    )
        internal
    {
        if (_amount > 0) {
            totalFAssetFees += _amount;
            fAsset.safeTransferFrom(_from, address(this), _amount);
        }
    }

    function _transferFAssetTo(
        address _to,
        uint256 _amount
    )
        internal
    {
        if (_amount > 0) {
            totalFAssetFees -= _amount;
            fAsset.safeTransfer(_to, _amount);
        }
    }

    function _transferWNatTo(
        address _to,
        uint256 _amount
    )
        internal
    {
        if (_amount > 0) {
            totalCollateral -= _amount;
            wNat.safeTransfer(_to, _amount);
        }
    }

    function _withdrawWNatTo(
        address payable _recipient,
        uint256 _amount
    )
        internal
    {
        if (_amount > 0) {
            totalCollateral -= _amount;
            internalWithdrawal = true;
            wNat.withdraw(_amount);
            internalWithdrawal = false;
            Transfers.transferNAT(_recipient, _amount);
        }
    }

    function _depositWNat()
        internal
    {
        // msg.value is always > 0 in this contract
        if (msg.value > 0) {
            totalCollateral += msg.value;
            wNat.deposit{value: msg.value}();
            assetManager.updateCollateral(agentVault, wNat);
        }
    }

    ////////////////////////////////////////////////////////////////////////////////////
    // methods for viewing user balances

    /**
     * @notice Returns the sum of the user's reward f-assets and their corresponding f-asset debt
     * @param _account  User address
     */
    function virtualFAssetOf(address _account)
        external view
        returns (uint256)
    {
        return _virtualFAssetFeesOf(_account);
    }

    /**
     * @notice Returns user's reward f-assets
     * @param _account  User address
     */
    function fAssetFeesOf(address _account)
        external view
        returns (uint256)
    {
        return _fAssetFeesOf(_account);
    }

    /**
     * @notice Returns user's f-asset debt
     * @param _account  User address
     */
    function fAssetFeeDebtOf(address _account)
        external view
        returns (int256)
    {
        return _fAssetFeeDebtOf[_account];
    }

    /**
     * @notice Returns user's debt tokens
     * @param _account  User address
     */
    function debtLockedTokensOf(address _account)
        external view
        returns (uint256)
    {
        return MathUtils.subOrZero(token.balanceOf(_account), _debtFreeTokensOf(_account));
    }

    /**
     * @notice Returns user's free tokens
     * @param _account  User address
     */
    function debtFreeTokensOf(address _account)
        external view
        returns (uint256)
    {
        return _debtFreeTokensOf(_account);
    }

    ////////////////////////////////////////////////////////////////////////////////////
    // Methods to allow for management and destruction of the pool

    function destroy(address payable _recipient)
        external
        onlyAssetManager
        nonReentrant
    {
        require(token.totalSupply() == 0, CannotDestroyPoolWithIssuedTokens());
        // transfer native balance as WNat, if any
        Transfers.depositWNat(wNat, _recipient, address(this).balance);
        // transfer untracked f-assets and wNat, if any
        uint256 untrackedWNat = wNat.balanceOf(address(this));
        uint256 untrackedFAsset = fAsset.balanceOf(address(this));
        if (untrackedWNat > 0) {
            wNat.safeTransfer(_recipient, untrackedWNat);
        }
        if (untrackedFAsset > 0) {
            fAsset.safeTransfer(_recipient, untrackedFAsset);
        }
    }

    // slither-disable-next-line reentrancy-eth         // guarded by nonReentrant
    function upgradeWNatContract(IWNat _newWNat)
        external
        onlyAssetManager
        nonReentrant
    {
        if (_newWNat == wNat) return;
        // transfer all funds to new WNat
        uint256 balance = wNat.balanceOf(address(this));
        internalWithdrawal = true;
        wNat.withdraw(balance);
        internalWithdrawal = false;
        _newWNat.deposit{value: balance}();
        // set new WNat contract
        wNat = _newWNat;
        assetManager.updateCollateral(agentVault, wNat);
    }

    ////////////////////////////////////////////////////////////////////////////////////
    // Delegation of the pool's collateral and airdrop claiming (same as in AgentVault)

    function delegate(address _to, uint256 _bips) external onlyAgent {
        wNat.delegate(_to, _bips);
    }

    function undelegateAll() external onlyAgent {
        wNat.undelegateAll();
    }

    function delegateGovernance(address _to) external onlyAgent {
        wNat.governanceVotePower().delegate(_to);
    }

    function undelegateGovernance() external onlyAgent {
        wNat.governanceVotePower().undelegate();
    }

    function claimDelegationRewards(
        IRewardManager _rewardManager,
        uint24 _lastRewardEpoch,
        IRewardManager.RewardClaimWithProof[] calldata _proofs
    )
        external
        onlyAgent
        nonReentrant
        returns (uint256)
    {
        uint256 balanceBefore = wNat.balanceOf(address(this));
        _rewardManager.claim(address(this), payable(address(this)), _lastRewardEpoch, true, _proofs);
        uint256 balanceAfter = wNat.balanceOf(address(this));
        uint256 claimed = balanceAfter - balanceBefore;
        totalCollateral += claimed;
        assetManager.updateCollateral(agentVault, wNat);
        emit CPClaimedReward(claimed, 1);
        return claimed;
    }

    function claimAirdropDistribution(
        IDistributionToDelegators _distribution,
        uint256 _month
    )
        external
        onlyAgent
        nonReentrant
        returns(uint256)
    {
        uint256 balanceBefore = wNat.balanceOf(address(this));
        _distribution.claim(address(this), payable(address(this)), _month, true);
        uint256 balanceAfter = wNat.balanceOf(address(this));
        uint256 claimed = balanceAfter - balanceBefore;
        totalCollateral += claimed;
        assetManager.updateCollateral(agentVault, wNat);
        emit CPClaimedReward(claimed, 0);
        return claimed;
    }

    function optOutOfAirdrop(
        IDistributionToDelegators _distribution
    )
        external
        onlyAgent
        nonReentrant
    {
        _distribution.optOutOfAirdrop();
    }

    ////////////////////////////////////////////////////////////////////////////////////
    // UUPS proxy upgrade

    function implementation() external view returns (address) {
        return _getImplementation();
    }

    /**
     * Upgrade calls can only arrive through asset manager.
     * See UUPSUpgradeable._authorizeUpgrade.
     */
    function _authorizeUpgrade(address /* _newImplementation */)
        internal virtual override
        onlyAssetManager
    { // solhint-disable-line no-empty-blocks
    }

    ////////////////////////////////////////////////////////////////////////////////////
    // The rest

    function isAgentVaultOwner(address _address)
        internal view
        returns (bool)
    {
        return assetManager.isAgentVaultOwner(agentVault, _address);
    }

    /**
     * Implementation of ERC-165 interface.
     */
    function supportsInterface(bytes4 _interfaceId)
        external pure override
        returns (bool)
    {
        return _interfaceId == type(IERC165).interfaceId
            || _interfaceId == type(ICollateralPool).interfaceId
            || _interfaceId == type(IICollateralPool).interfaceId;
    }
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/******************************************************************************\
* Author: Nick Mudge <nick@perfectabstractions.com> (https://twitter.com/mudgen)
* EIP-2535 Diamonds: https://eips.ethereum.org/EIPS/eip-2535
/******************************************************************************/

import { Globals } from "../library/Globals.sol";
import { IDiamondCut } from "../../diamond/interfaces/IDiamondCut.sol";
import { LibDiamond } from "../../diamond/library/LibDiamond.sol";
import { GovernedProxyImplementation } from "../../governance/implementation/GovernedProxyImplementation.sol";

// DiamondCutFacet that also respects diamondCutMinTimelockSeconds setting.

// Remember to add the loupe functions from DiamondLoupeFacet to the diamond.
// The loupe functions are required by the EIP2535 Diamonds standard

contract AssetManagerDiamondCutFacet is IDiamondCut, GovernedProxyImplementation {
    /// @notice Add/replace/remove any number of functions and optionally execute
    ///         a function with delegatecall
    /// @param _diamondCut Contains the facet addresses and function selectors
    /// @param _init The address of the contract or facet to execute _calldata
    /// @param _calldata A function call, including function selector and arguments
    ///                  _calldata is executed with delegatecall on _init
    function diamondCut(
        FacetCut[] calldata _diamondCut,
        address _init,
        bytes calldata _calldata
    )
        external override
        onlyGovernanceWithTimelockAtLeast(Globals.getSettings().diamondCutMinTimelockSeconds)
    {
        LibDiamond.diamondCut(_diamondCut, _init, _calldata);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import "../../IVPToken.sol";
import "../../IVPContractEvents.sol";
import "./IICleanable.sol";

interface IIVPContract is IICleanable, IVPContractEvents {
    /**
     * Update vote powers when tokens are transfered.
     * Also update delegated vote powers for percentage delegation
     * and check for enough funds for explicit delegations.
     **/
    function updateAtTokenTransfer(
        address _from,
        address _to,
        uint256 _fromBalance,
        uint256 _toBalance,
        uint256 _amount
    ) external;

    /**
     * @notice Delegate `_bips` percentage of voting power to `_to` from `_from`
     * @param _from The address of the delegator
     * @param _to The address of the recipient
     * @param _balance The delegator's current balance
     * @param _bips The percentage of voting power to be delegated expressed in basis points (1/100 of one percent).
     *   Not cumulative - every call resets the delegation value (and value of 0 revokes delegation).
     **/
    function delegate(
        address _from,
        address _to,
        uint256 _balance,
        uint256 _bips
    ) external;

    /**
     * @notice Explicitly delegate `_amount` of voting power to `_to` from `msg.sender`.
     * @param _from The address of the delegator
     * @param _to The address of the recipient
     * @param _balance The delegator's current balance
     * @param _amount An explicit vote power amount to be delegated.
     *   Not cumulative - every call resets the delegation value (and value of 0 undelegates `to`).
     **/
    function delegateExplicit(
        address _from,
        address _to,
        uint256 _balance,
        uint _amount
    ) external;

    /**
     * @notice Revoke all delegation from sender to `_who` at given block.
     *    Only affects the reads via `votePowerOfAtCached()` in the block `_blockNumber`.
     *    Block `_blockNumber` must be in the past.
     *    This method should be used only to prevent rogue delegate voting in the current voting block.
     *    To stop delegating use delegate/delegateExplicit with value of 0 or undelegateAll/undelegateAllExplicit.
     * @param _from The address of the delegator
     * @param _who Address of the delegatee
     * @param _balance The delegator's current balance
     * @param _blockNumber The block number at which to revoke delegation.
     **/
    function revokeDelegationAt(
        address _from,
        address _who,
        uint256 _balance,
        uint _blockNumber
    ) external;

    /**
     * @notice Undelegate all voting power for delegates of `msg.sender`
     *    Can only be used with percentage delegation.
     *    Does not reset delegation mode back to NOTSET.
     * @param _from The address of the delegator
     **/
    function undelegateAll(address _from, uint256 _balance) external;

    /**
     * @notice Undelegate all explicit vote power by amount delegates for `msg.sender`.
     *    Can only be used with explicit delegation.
     *    Does not reset delegation mode back to NOTSET.
     * @param _from The address of the delegator
     * @param _delegateAddresses Explicit delegation does not store delegatees' addresses,
     *   so the caller must supply them.
     * @return The amount still delegated (in case the list of delegates was incomplete).
     */
    function undelegateAllExplicit(
        address _from,
        address[] memory _delegateAddresses
    ) external returns (uint256);

    /**
     * @notice Get the vote power of `_who` at block `_blockNumber`
     *   Reads/updates cache and upholds revocations.
     * @param _who The address to get voting power.
     * @param _blockNumber The block number at which to fetch.
     * @return Vote power of `_who` at `_blockNumber`.
     */
    function votePowerOfAtCached(
        address _who,
        uint256 _blockNumber
    ) external returns (uint256);

    /**
     * @notice Get the current vote power of `_who`.
     * @param _who The address to get voting power.
     * @return Current vote power of `_who`.
     */
    function votePowerOf(address _who) external view returns (uint256);

    /**
     * @notice Get the vote power of `_who` at block `_blockNumber`
     * @param _who The address to get voting power.
     * @param _blockNumber The block number at which to fetch.
     * @return Vote power of `_who` at `_blockNumber`.
     */
    function votePowerOfAt(
        address _who,
        uint256 _blockNumber
    ) external view returns (uint256);

    /**
     * @notice Get the vote power of `_who` at block `_blockNumber`, ignoring revocation information (and cache).
     * @param _who The address to get voting power.
     * @param _blockNumber The block number at which to fetch.
     * @return Vote power of `_who` at `_blockNumber`. Result doesn't change if vote power is revoked.
     */
    function votePowerOfAtIgnoringRevocation(
        address _who,
        uint256 _blockNumber
    ) external view returns (uint256);

    /**
     * Return vote powers for several addresses in a batch.
     * @param _owners The list of addresses to fetch vote power of.
     * @param _blockNumber The block number at which to fetch.
     * @return A list of vote powers.
     */
    function batchVotePowerOfAt(
        address[] memory _owners,
        uint256 _blockNumber
    ) external view returns (uint256[] memory);

    /**
     * @notice Get current delegated vote power `_from` delegator delegated `_to` delegatee.
     * @param _from Address of delegator
     * @param _to Address of delegatee
     * @param _balance The delegator's current balance
     * @return The delegated vote power.
     */
    function votePowerFromTo(
        address _from,
        address _to,
        uint256 _balance
    ) external view returns (uint256);

    /**
     * @notice Get delegated the vote power `_from` delegator delegated `_to` delegatee at `_blockNumber`.
     * @param _from Address of delegator
     * @param _to Address of delegatee
     * @param _balance The delegator's current balance
     * @param _blockNumber The block number at which to fetch.
     * @return The delegated vote power.
     */
    function votePowerFromToAt(
        address _from,
        address _to,
        uint256 _balance,
        uint _blockNumber
    ) external view returns (uint256);

    /**
     * @notice Compute the current undelegated vote power of `_owner`
     * @param _owner The address to get undelegated voting power.
     * @param _balance Owner's current balance
     * @return The unallocated vote power of `_owner`
     */
    function undelegatedVotePowerOf(
        address _owner,
        uint256 _balance
    ) external view returns (uint256);

    /**
     * @notice Get the undelegated vote power of `_owner` at given block.
     * @param _owner The address to get undelegated voting power.
     * @param _blockNumber The block number at which to fetch.
     * @return The undelegated vote power of `_owner` (= owner's own balance minus all delegations from owner)
     */
    function undelegatedVotePowerOfAt(
        address _owner,
        uint256 _balance,
        uint256 _blockNumber
    ) external view returns (uint256);

    /**
     * @notice Get the delegation mode for '_who'. This mode determines whether vote power is
     *  allocated by percentage or by explicit value.
     * @param _who The address to get delegation mode.
     * @return Delegation mode (NOTSET=0, PERCENTAGE=1, AMOUNT=2))
     */
    function delegationModeOf(address _who) external view returns (uint256);

    /**
     * @notice Get the vote power delegation `_delegateAddresses`
     *  and `pcts` of an `_owner`. Returned in two separate positional arrays.
     * @param _owner The address to get delegations.
     * @return _delegateAddresses Positional array of delegation addresses.
     * @return _bips Positional array of delegation percents specified in basis points (1/100 or 1 percent)
     * @return _count The number of delegates.
     * @return _delegationMode The mode of the delegation (NOTSET=0, PERCENTAGE=1, AMOUNT=2).
     */
    function delegatesOf(
        address _owner
    )
        external
        view
        returns (
            address[] memory _delegateAddresses,
            uint256[] memory _bips,
            uint256 _count,
            uint256 _delegationMode
        );

    /**
     * @notice Get the vote power delegation `delegationAddresses`
     *  and `pcts` of an `_owner`. Returned in two separate positional arrays.
     * @param _owner The address to get delegations.
     * @param _blockNumber The block for which we want to know the delegations.
     * @return _delegateAddresses Positional array of delegation addresses.
     * @return _bips Positional array of delegation percents specified in basis points (1/100 or 1 percent)
     * @return _count The number of delegates.
     * @return _delegationMode The mode of the delegation (NOTSET=0, PERCENTAGE=1, AMOUNT=2).
     */
    function delegatesOfAt(
        address _owner,
        uint256 _blockNumber
    )
        external
        view
        returns (
            address[] memory _delegateAddresses,
            uint256[] memory _bips,
            uint256 _count,
            uint256 _delegationMode
        );

    /**
     * The VPToken (or some other contract) that owns this VPContract.
     * All state changing methods may be called only from this address.
     * This is because original msg.sender is sent in `_from` parameter
     * and we must be sure that it cannot be faked by directly calling VPContract.
     * Owner token is also used in case of replacement to recover vote powers from balances.
     */
    function ownerToken() external view returns (IVPToken);

    /**
     * Return true if this IIVPContract is configured to be used as a replacement for other contract.
     * It means that vote powers are not necessarily correct at the initialization, therefore
     * every method that reads vote power must check whether it is initialized for that address and block.
     */
    function isReplacement() external view returns (bool);
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;
pragma abicoder v2;

import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {IAssetManagerController} from "../../userInterfaces/IAssetManagerController.sol";
import {IAddressUpdatable} from "../../flareSmartContracts/interfaces/IAddressUpdatable.sol";
import {IUUPSUpgradeable} from "../../utils/interfaces/IUUPSUpgradeable.sol";
import {IIAssetManager} from "../../assetManager/interfaces/IIAssetManager.sol";
import {IGoverned} from "../../governance/interfaces/IGoverned.sol";
import {CollateralType} from "../../userInterfaces/data/CollateralType.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";



interface IIAssetManagerController is
    IERC165,
    IAssetManagerController,
    IGoverned,
    IAddressUpdatable,
    IUUPSUpgradeable
{
    /**
     * New address in case this controller was replaced.
     * Note: this code contains no checks that replacedBy==0, because when replaced,
     * all calls to AssetManager's updateSettings/pause will fail anyway
     * since they will arrive from wrong controller address.
     */
    function replacedBy() external view returns (address);

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // Manage list of asset managers

    /**
     * Add an asset manager to this controller. The asset manager controller address in the settings of the
     * asset manager must match this. This method automatically marks the asset manager as attached.
     */
    function addAssetManager(IIAssetManager _assetManager)
        external;

    /**
     * Remove an asset manager from this controller, if it is attached to this controller.
     * The asset manager won't be attached any more, so it will be unusable.
     */
    function removeAssetManager(IIAssetManager _assetManager)
        external;

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // Setters

    function setAgentOwnerRegistry(IIAssetManager[] memory _assetManagers, address _value)
        external;

    function setAgentVaultFactory(IIAssetManager[] memory _assetManagers, address _value)
        external;

    function setCollateralPoolFactory(IIAssetManager[] memory _assetManagers, address _value)
        external;

    function setCollateralPoolTokenFactory(IIAssetManager[] memory _assetManagers, address _value)
        external;

    function upgradeAgentVaultsAndPools(IIAssetManager[] memory _assetManagers, uint256 _start, uint256 _end)
        external;

    function setPriceReader(IIAssetManager[] memory _assetManagers, address _value)
        external;

    function setFdcVerification(IIAssetManager[] memory _assetManagers, address _value)
        external;

    function setCleanerContract(IIAssetManager[] memory _assetManagers, address _value)
        external;

    function setCleanupBlockNumberManager(IIAssetManager[] memory _assetManagers, address _value)
        external;

    // if callData is not empty, it is abi encoded call to init function in the new proxy implementation
    function upgradeFAssetImplementation(
        IIAssetManager[] memory _assetManagers,
        address _implementation,
        bytes memory _callData
    ) external;

    function setMinUpdateRepeatTimeSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setLotSizeAmg(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setTimeForPayment(
        IIAssetManager[] memory _assetManagers,
        uint256 _underlyingBlocks,
        uint256 _underlyingSeconds
    ) external;

    function setPaymentChallengeReward(
        IIAssetManager[] memory _assetManagers,
        uint256 _rewardVaultCollateralWei,
        uint256 _rewardBIPS
    ) external;

    function setMaxTrustedPriceAgeSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setCollateralReservationFeeBips(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setRedemptionFeeBips(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setRedemptionDefaultFactorVaultCollateralBIPS(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setConfirmationByOthersAfterSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setConfirmationByOthersRewardUSD5(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setMaxRedeemedTickets(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setWithdrawalOrDestroyWaitMinSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setAttestationWindowSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setAverageBlockTimeMS(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setMintingPoolHoldingsRequiredBIPS(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setMintingCapAmg(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setTokenInvalidationTimeMinSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setVaultCollateralBuyForFlareFactorBIPS(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setAgentExitAvailableTimelockSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setAgentFeeChangeTimelockSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setAgentMintingCRChangeTimelockSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setPoolExitCRChangeTimelockSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setAgentTimelockedOperationWindowSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setCollateralPoolTokenTimelockSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setLiquidationStepSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setLiquidationPaymentFactors(
        IIAssetManager[] memory _assetManagers,
        uint256[] memory _paymentFactors,
        uint256[] memory _vaultCollateralFactors
    ) external;

    function setRedemptionPaymentExtensionSeconds(
        IIAssetManager[] memory _assetManagers,
        uint256 _value
    ) external;

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // Collateral tokens

    function addCollateralType(
        IIAssetManager[] memory _assetManagers,
        CollateralType.Data calldata _data
    ) external;

    function setCollateralRatiosForToken(
        IIAssetManager[] memory _assetManagers,
        CollateralType.Class _class,
        IERC20 _token,
        uint256 _minCollateralRatioBIPS,
        uint256 _safetyMinCollateralRatioBIPS
    ) external;

    function deprecateCollateralType(
        IIAssetManager[] memory _assetManagers,
        CollateralType.Class _class,
        IERC20 _token,
        uint256 _invalidationTimeSec
    ) external;

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // Upgrade (second phase)

    /**
     * When asset manager is paused, no new minting can be made.
     * All other operations continue normally.
     */
    function pauseMinting(IIAssetManager[] calldata _assetManagers)
        external;

    /**
     * Minting can continue.
     */
    function unpauseMinting(IIAssetManager[] calldata _assetManagers)
        external;

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // Update contracts

    /**
     * Can be called to update address updater managed contracts if there are too many asset managers
     * to update in one block. In such a case, running AddressUpdater.updateContractAddresses will fail
     * and there will be no way to update contracts. This method allow the update to only change some
     * of the asset managers.
     */
    function updateContracts(IIAssetManager[] calldata _assetManagers)
        external;

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // Emergency pause

    function emergencyPause(IIAssetManager[] memory _assetManagers, uint256 _duration)
        external;

    function emergencyPauseTransfers(IIAssetManager[] memory _assetManagers, uint256 _duration)
        external;

    function resetEmergencyPauseTotalDuration(IIAssetManager[] memory _assetManagers)
        external;

    function addEmergencyPauseSender(address _address)
        external;

    function removeEmergencyPauseSender(address _address)
        external;

    function setMaxEmergencyPauseDurationSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;

    function setEmergencyPauseDurationResetAfterSeconds(IIAssetManager[] memory _assetManagers, uint256 _value)
        external;
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6 <0.9;

import "../../IRewardManager.sol";

/**
 * RewardManager internal interface.
 */
interface IIRewardManager is IRewardManager {
    /**
     * Claim rewards for `_rewardOwner` and transfer them to `_recipient`.
     * It can be called only by FtsoRewardManagerProxy contract.
     * @param _msgSender Address of the message sender.
     * @param _rewardOwner Address of the reward owner.
     * @param _recipient Address of the reward recipient.
     * @param _rewardEpochId Id of the reward epoch up to which the rewards are claimed.
     * @param _wrap Indicates if the reward should be wrapped (deposited) to the WNAT contract.
     * @param _proofs Array of reward claims with merkle proofs.
     * @return _rewardAmountWei Amount of rewarded native tokens (wei).
     */
    function claimProxy(
        address _msgSender,
        address _rewardOwner,
        address payable _recipient,
        uint24 _rewardEpochId,
        bool _wrap,
        RewardClaimWithProof[] calldata _proofs
    ) external returns (uint256 _rewardAmountWei);

    /**
     * Receives funds from reward offers manager.
     * @param _rewardEpochId ID of the reward epoch for which the funds are received.
     * @param _inflation Indicates if the funds come from the inflation (true) or from the community (false).
     * @dev Only reward offers manager can call this method.
     */
    function receiveRewards(
        uint24 _rewardEpochId,
        bool _inflation
    ) external payable;

    /**
     * Collects funds from expired reward epoch and calculates totals.
     *
     * Triggered by FlareSystemsManager on finalization of a reward epoch.
     * Operation is irreversible: when some reward epoch is closed according to current
     * settings, it cannot be reopened even if new parameters would
     * allow it, because `nextRewardEpochIdToExpire` in FlareSystemsManager never decreases.
     * @param _rewardEpochId Id of the reward epoch to close.
     */
    function closeExpiredRewardEpoch(uint256 _rewardEpochId) external;
}


## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

