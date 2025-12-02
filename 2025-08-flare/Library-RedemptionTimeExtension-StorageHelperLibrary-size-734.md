
## *MAIN TARGET CONTRACT* TO REVIEW

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

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
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

## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS

## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

