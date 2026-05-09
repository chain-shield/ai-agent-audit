
 ------------ ## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity ^0.7.6;
pragma abicoder v2;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable function-max-lines, gas-strict-inequalities

import { SafeMath } from "@openzeppelin/contracts/math/SafeMath.sol";

import { ITokenGateway } from "@graphprotocol/interfaces/contracts/contracts/arbitrum/ITokenGateway.sol";
import { Staking } from "./Staking.sol";
import { Stakes } from "./libs/Stakes.sol";
import { IStakes } from "@graphprotocol/interfaces/contracts/contracts/staking/libs/IStakes.sol";
import { IStakingData } from "@graphprotocol/interfaces/contracts/contracts/staking/IStakingData.sol";
import { L1StakingV1Storage } from "./L1StakingStorage.sol";
import { IGraphToken } from "@graphprotocol/interfaces/contracts/contracts/token/IGraphToken.sol";
import { IL1StakingBase } from "@graphprotocol/interfaces/contracts/contracts/staking/IL1StakingBase.sol";
import { MathUtils } from "./libs/MathUtils.sol";
import { IL1GraphTokenLockTransferTool } from "@graphprotocol/interfaces/contracts/contracts/staking/IL1GraphTokenLockTransferTool.sol";
import { IL2StakingTypes } from "@graphprotocol/interfaces/contracts/contracts/l2/staking/IL2StakingTypes.sol";

/**
 * @title L1Staking contract
 * @author Edge & Node
 * @notice This contract is the L1 variant of the Staking contract. It adds functions
 * to send an indexer's stake to L2, and to send delegation to L2 as well.
 */
contract L1Staking is Staking, L1StakingV1Storage, IL1StakingBase {
    using Stakes for IStakes.Indexer;
    using SafeMath for uint256;

    /**
     * @notice Receive ETH into the Staking contract
     * @dev Only the L1GraphTokenLockTransferTool can send ETH, as part of the
     * transfer of stake/delegation for vesting lock wallets.
     */
    receive() external payable {
        require(msg.sender == address(l1GraphTokenLockTransferTool), "Only transfer tool can send ETH");
    }

    /**
     * @inheritdoc IL1StakingBase
     */
    function setL1GraphTokenLockTransferTool(
        IL1GraphTokenLockTransferTool _l1GraphTokenLockTransferTool
    ) external override onlyGovernor {
        l1GraphTokenLockTransferTool = _l1GraphTokenLockTransferTool;
        emit L1GraphTokenLockTransferToolSet(address(_l1GraphTokenLockTransferTool));
    }

    /**
     * @inheritdoc IL1StakingBase
     */
    function transferStakeToL2(
        address _l2Beneficiary,
        uint256 _amount,
        uint256 _maxGas,
        uint256 _gasPriceBid,
        uint256 _maxSubmissionCost
    ) external payable override notPartialPaused {
        require(msg.value == _maxSubmissionCost.add(_gasPriceBid.mul(_maxGas)), "INVALID_ETH_AMOUNT");
        _transferStakeToL2(msg.sender, _l2Beneficiary, _amount, _maxGas, _gasPriceBid, _maxSubmissionCost, msg.value);
    }

    /**
     * @inheritdoc IL1StakingBase
     */
    function transferLockedStakeToL2(
        uint256 _amount,
        uint256 _maxGas,
        uint256 _gasPriceBid,
        uint256 _maxSubmissionCost
    ) external override notPartialPaused {
        address l2Beneficiary = l1GraphTokenLockTransferTool.l2WalletAddress(msg.sender);
        require(l2Beneficiary != address(0), "LOCK NOT TRANSFERRED");
        uint256 balance = address(this).balance;
        uint256 ethAmount = _maxSubmissionCost.add(_maxGas.mul(_gasPriceBid));
        l1GraphTokenLockTransferTool.pullETH(msg.sender, ethAmount);
        require(address(this).balance == balance.add(ethAmount), "ETH TRANSFER FAILED");
        _transferStakeToL2(msg.sender, l2Beneficiary, _amount, _maxGas, _gasPriceBid, _maxSubmissionCost, ethAmount);
    }

    /**
     * @inheritdoc IL1StakingBase
     */
    function transferDelegationToL2(
        address _indexer,
        address _l2Beneficiary,
        uint256 _maxGas,
        uint256 _gasPriceBid,
        uint256 _maxSubmissionCost
    ) external payable override notPartialPaused {
        require(msg.value == _maxSubmissionCost.add(_gasPriceBid.mul(_maxGas)), "INVALID_ETH_AMOUNT");
        _transferDelegationToL2(
            msg.sender,
            _indexer,
            _l2Beneficiary,
            _maxGas,
            _gasPriceBid,
            _maxSubmissionCost,
            msg.value
        );
    }

    /**
     * @inheritdoc IL1StakingBase
     */
    function transferLockedDelegationToL2(
        address _indexer,
        uint256 _maxGas,
        uint256 _gasPriceBid,
        uint256 _maxSubmissionCost
    ) external override notPartialPaused {
        address l2Beneficiary = l1GraphTokenLockTransferTool.l2WalletAddress(msg.sender);
        require(l2Beneficiary != address(0), "LOCK NOT TRANSFERRED");
        uint256 balance = address(this).balance;
        uint256 ethAmount = _maxSubmissionCost.add(_maxGas.mul(_gasPriceBid));
        l1GraphTokenLockTransferTool.pullETH(msg.sender, ethAmount);
        require(address(this).balance == balance.add(ethAmount), "ETH TRANSFER FAILED");
        _transferDelegationToL2(
            msg.sender,
            _indexer,
            l2Beneficiary,
            _maxGas,
            _gasPriceBid,
            _maxSubmissionCost,
            ethAmount
        );
    }

    /**
     * @inheritdoc IL1StakingBase
     */
    function unlockDelegationToTransferredIndexer(address _indexer) external override notPartialPaused {
        require(
            indexerTransferredToL2[_indexer] != address(0) && __stakes[_indexer].tokensStaked == 0,
            "indexer not transferred"
        );

        Delegation storage delegation = __delegationPools[_indexer].delegators[msg.sender];
        require(delegation.tokensLocked != 0, "! locked");

        // Unlock the delegation
        delegation.tokensLockedUntil = epochManager().currentEpoch();

        // After this, the delegator should be able to withdraw in the current block
        emit StakeDelegatedUnlockedDueToL2Transfer(_indexer, msg.sender);
    }

    /**
     * @notice Implements sending an indexer's stake to L2.
     * This function can only be called by the indexer (not an operator).
     * It will validate that the remaining stake is sufficient to cover all the allocated
     * stake, so the indexer might have to close some allocations before transferring.
     * It will also check that the indexer's stake is not locked for withdrawal.
     * Since the indexer address might be an L1-only contract, the function takes a beneficiary
     * address that will be the indexer's address in L2.
     * @param _indexer Address of the indexer transferring stake
     * @param _l2Beneficiary Address of the indexer in L2. If the indexer has previously transferred stake, this must match the previously-used value.
     * @param _amount Amount of stake GRT to transfer to L2
     * @param _maxGas Max gas to use for the L2 retryable ticket
     * @param _gasPriceBid Gas price bid for the L2 retryable ticket
     * @param _maxSubmissionCost Max submission cost for the L2 retryable ticket
     * @param _ethAmount Amount of ETH to send with the retryable ticket
     */
    function _transferStakeToL2(
        address _indexer,
        address _l2Beneficiary,
        uint256 _amount,
        uint256 _maxGas,
        uint256 _gasPriceBid,
        uint256 _maxSubmissionCost,
        uint256 _ethAmount
    ) internal {
        IStakes.Indexer storage indexerStake = __stakes[_indexer];
        require(indexerStake.tokensStaked != 0, "tokensStaked == 0");
        // Indexers shouldn't be trying to withdraw tokens before transferring to L2.
        // Allowing this would complicate our accounting so we require that they have no
        // tokens locked for withdrawal.
        require(indexerStake.tokensLocked == 0, "tokensLocked != 0");

        require(_l2Beneficiary != address(0), "l2Beneficiary == 0");
        if (indexerTransferredToL2[_indexer] != address(0)) {
            require(indexerTransferredToL2[_indexer] == _l2Beneficiary, "l2Beneficiary != previous");
        } else {
            indexerTransferredToL2[_indexer] = _l2Beneficiary;
            require(_amount >= __minimumIndexerStake, "!minimumIndexerStake sent");
        }
        // Ensure minimum stake
        indexerStake.tokensStaked = indexerStake.tokensStaked.sub(_amount);
        require(
            indexerStake.tokensStaked == 0 || indexerStake.tokensStaked >= __minimumIndexerStake,
            "!minimumIndexerStake remaining"
        );

        IStakingData.DelegationPool storage delegationPool = __delegationPools[_indexer];

        if (indexerStake.tokensStaked == 0) {
            // require that no allocations are open
            require(indexerStake.tokensAllocated == 0, "allocated");
        } else {
            // require that the indexer has enough stake to cover all allocations
            uint256 tokensDelegatedCap = indexerStake.tokensStaked.mul(uint256(__delegationRatio));
            uint256 tokensDelegatedCapacity = MathUtils.min(delegationPool.tokens, tokensDelegatedCap);
            require(
                indexerStake.tokensUsed() <= indexerStake.tokensStaked.add(tokensDelegatedCapacity),
                "! allocation capacity"
            );
        }

        IL2StakingTypes.ReceiveIndexerStakeData memory functionData;
        functionData.indexer = _l2Beneficiary;

        bytes memory extraData = abi.encode(
            uint8(IL2StakingTypes.L1MessageCodes.RECEIVE_INDEXER_STAKE_CODE),
            abi.encode(functionData)
        );

        _sendTokensAndMessageToL2Staking(_amount, _maxGas, _gasPriceBid, _maxSubmissionCost, _ethAmount, extraData);

        emit IndexerStakeTransferredToL2(_indexer, _l2Beneficiary, _amount);
    }

    /**
     * @notice Implements sending a delegator's delegated tokens to L2.
     * This function can only be called by the delegator.
     * This function will validate that the indexer has transferred their stake using transferStakeToL2,
     * and that the delegation is not locked for undelegation.
     * Since the delegator's address might be an L1-only contract, the function takes a beneficiary
     * address that will be the delegator's address in L2.
     * @param _delegator Address of the delegator transferring delegation
     * @param _indexer Address of the indexer (in L1, before transferring to L2)
     * @param _l2Beneficiary Address of the delegator in L2
     * @param _maxGas Max gas to use for the L2 retryable ticket
     * @param _gasPriceBid Gas price bid for the L2 retryable ticket
     * @param _maxSubmissionCost Max submission cost for the L2 retryable ticket
     * @param _ethAmount Amount of ETH to send with the retryable ticket
     */
    function _transferDelegationToL2(
        address _delegator,
        address _indexer,
        address _l2Beneficiary,
        uint256 _maxGas,
        uint256 _gasPriceBid,
        uint256 _maxSubmissionCost,
        uint256 _ethAmount
    ) internal {
        require(_l2Beneficiary != address(0), "l2Beneficiary == 0");
        require(indexerTransferredToL2[_indexer] != address(0), "indexer not transferred");

        // Get the delegation pool of the indexer
        DelegationPool storage pool = __delegationPools[_indexer];
        Delegation storage delegation = pool.delegators[_delegator];

        // Check that the delegation is not locked for undelegation
        require(delegation.tokensLocked == 0, "tokensLocked != 0");
        require(delegation.shares != 0, "delegation == 0");
        // Calculate tokens to get in exchange for the shares
        uint256 tokensToSend = delegation.shares.mul(pool.tokens).div(pool.shares);

        // Update the delegation pool
        pool.tokens = pool.tokens.sub(tokensToSend);
        pool.shares = pool.shares.sub(delegation.shares);

        // Update the delegation
        delegation.shares = 0;
        bytes memory extraData;
        {
            IL2StakingTypes.ReceiveDelegationData memory functionData;
            functionData.indexer = indexerTransferredToL2[_indexer];
            functionData.delegator = _l2Beneficiary;
            extraData = abi.encode(
                uint8(IL2StakingTypes.L1MessageCodes.RECEIVE_DELEGATION_CODE),
                abi.encode(functionData)
            );
        }

        _sendTokensAndMessageToL2Staking(
            tokensToSend,
            _maxGas,
            _gasPriceBid,
            _maxSubmissionCost,
            _ethAmount,
            extraData
        );
        emit DelegationTransferredToL2(
            _delegator,
            _l2Beneficiary,
            _indexer,
            indexerTransferredToL2[_indexer],
            tokensToSend
        );
    }

    /**
     * @notice Sends a message to the L2Staking with some extra data,
     * also sending some tokens, using the L1GraphTokenGateway.
     * @param _tokens Amount of tokens to send to L2
     * @param _maxGas Max gas to use for the L2 retryable ticket
     * @param _gasPriceBid Gas price bid for the L2 retryable ticket
     * @param _maxSubmissionCost Max submission cost for the L2 retryable ticket
     * @param _value Amount of ETH to send with the message
     * @param _extraData Extra data for the callhook on L2Staking
     */
    function _sendTokensAndMessageToL2Staking(
        uint256 _tokens,
        uint256 _maxGas,
        uint256 _gasPriceBid,
        uint256 _maxSubmissionCost,
        uint256 _value,
        bytes memory _extraData
    ) internal {
        IGraphToken grt = graphToken();
        ITokenGateway gateway = graphTokenGateway();
        grt.approve(address(gateway), _tokens);
        gateway.outboundTransfer{ value: _value }(
            address(grt),
            counterpartStakingAddress,
            _tokens,
            _maxGas,
            _gasPriceBid,
            abi.encode(_maxSubmissionCost, _extraData)
        );
    }
}
 ------------
 ------------ END OF MAIN TARGET CONTRACT ------------ 

 ------------ ## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES ------------ 
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

pragma solidity ^0.7.6 || ^0.8.0;
pragma abicoder v2;

/**
 * @title Interface for the L1GraphTokenLockTransferTool contract
 * @author Edge & Node
 * @notice This interface defines the function to get the L2 wallet address for a given L1 token lock wallet.
 * The Transfer Tool contract is implemented in the token-distribution repo: https://github.com/graphprotocol/token-distribution/pull/64
 * and is only included here to provide support in L1Staking for the transfer of stake and delegation
 * owned by token lock contracts. See GIP-0046 for details: https://forum.thegraph.com/t/4023
 */
interface IL1GraphTokenLockTransferTool {
    /**
     * @notice Pulls ETH from an L1 wallet's account to use for L2 ticket gas.
     * @dev This function is only callable by the staking contract.
     * @param l1Wallet Address of the L1 token lock wallet
     * @param amount Amount of ETH to pull from the transfer tool contract
     */
    function pullETH(address l1Wallet, uint256 amount) external;

    /**
     * @notice Get the L2 token lock wallet address for a given L1 token lock wallet
     * @dev In the actual L1GraphTokenLockTransferTool contract, this is simply the default getter for a public mapping variable.
     * @param l1Wallet Address of the L1 token lock wallet
     * @return Address of the L2 token lock wallet if the wallet has an L2 counterpart, or address zero if
     * the wallet doesn't have an L2 counterpart (or is not known to be a token lock wallet).
     */
    function l2WalletAddress(address l1Wallet) external view returns (address);
}

// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity ^0.7.6;
pragma abicoder v2;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable function-max-lines, gas-strict-inequalities

import { SafeMath } from "@openzeppelin/contracts/math/SafeMath.sol";
import { ECDSA } from "@openzeppelin/contracts/cryptography/ECDSA.sol";

import { Multicall } from "../base/Multicall.sol";
import { GraphUpgradeable } from "../upgrades/GraphUpgradeable.sol";
import { TokenUtils } from "../utils/TokenUtils.sol";
import { IGraphToken } from "@graphprotocol/interfaces/contracts/contracts/token/IGraphToken.sol";
import { IStakingBase } from "@graphprotocol/interfaces/contracts/contracts/staking/IStakingBase.sol";
import { StakingV4Storage } from "./StakingStorage.sol";
import { MathUtils } from "./libs/MathUtils.sol";
import { Stakes } from "./libs/Stakes.sol";
import { IStakes } from "@graphprotocol/interfaces/contracts/contracts/staking/libs/IStakes.sol";
import { Managed } from "../governance/Managed.sol";
import { ICuration } from "@graphprotocol/interfaces/contracts/contracts/curation/ICuration.sol";
import { IRewardsManager } from "@graphprotocol/interfaces/contracts/contracts/rewards/IRewardsManager.sol";
import { StakingExtension } from "./StakingExtension.sol";
import { LibExponential } from "./libs/Exponential.sol";

/**
 * @title Base Staking contract
 * @author Edge & Node
 * @notice The Staking contract allows Indexers to Stake on Subgraphs. Indexers Stake by creating
 * Allocations on a Subgraph. It also allows Delegators to Delegate towards an Indexer. The
 * contract also has the slashing functionality.
 * The contract is abstract as the implementation that is deployed depends on each layer: L1Staking on mainnet
 * and L2Staking on Arbitrum.
 * Note that this contract delegates part of its functionality to a StakingExtension contract.
 * This is due to the 24kB contract size limit on Ethereum.
 */
abstract contract Staking is StakingV4Storage, GraphUpgradeable, IStakingBase, Multicall {
    using SafeMath for uint256;
    using Stakes for IStakes.Indexer;

    /// @dev 100% in parts per million
    uint32 internal constant MAX_PPM = 1000000;

    // -- Events are declared in IStakingBase -- //

    /**
     * @notice Delegates the current call to the StakingExtension implementation.
     * @dev This function does not return to its internal call site, it will return directly to the
     * external caller.
     */
    fallback() external {
        // solhint-disable-previous-line payable-fallback, no-complex-fallback

        require(_implementation() != address(0), "only through proxy");

        // solhint-disable-next-line no-inline-assembly
        assembly {
            // (a) get free memory pointer
            let ptr := mload(0x40)

            // (b) get address of the implementation
            // CAREFUL here: this only works because extensionImpl is the first variable in this slot
            // (otherwise we may have to apply an offset)
            let impl := and(sload(extensionImpl.slot), 0xffffffffffffffffffffffffffffffffffffffff)

            // (1) copy incoming call data
            calldatacopy(ptr, 0, calldatasize())

            // (2) forward call to logic contract
            let result := delegatecall(gas(), impl, ptr, calldatasize(), 0, 0)
            let size := returndatasize()

            // (3) retrieve return data
            returndatacopy(ptr, 0, size)

            // (4) forward return data back to caller
            switch result
            case 0 {
                revert(ptr, size)
            }
            default {
                return(ptr, size)
            }
        }
    }

    /**
     * @inheritdoc IStakingBase
     */
    function initialize(
        address _controller,
        uint256 _minimumIndexerStake,
        uint32 _thawingPeriod,
        uint32 _protocolPercentage,
        uint32 _curationPercentage,
        uint32 _maxAllocationEpochs,
        uint32 _delegationUnbondingPeriod,
        uint32 _delegationRatio,
        RebatesParameters calldata _rebatesParameters,
        address _extensionImpl
    ) external override onlyImpl {
        Managed._initialize(_controller);

        // Settings

        _setMinimumIndexerStake(_minimumIndexerStake);
        _setThawingPeriod(_thawingPeriod);

        _setProtocolPercentage(_protocolPercentage);
        _setCurationPercentage(_curationPercentage);

        _setMaxAllocationEpochs(_maxAllocationEpochs);

        _setRebateParameters(
            _rebatesParameters.alphaNumerator,
            _rebatesParameters.alphaDenominator,
            _rebatesParameters.lambdaNumerator,
            _rebatesParameters.lambdaDenominator
        );

        extensionImpl = _extensionImpl;

        // solhint-disable-next-line avoid-low-level-calls
        (bool success, ) = extensionImpl.delegatecall(
            abi.encodeWithSelector(
                StakingExtension.initialize.selector,
                _delegationUnbondingPeriod,
                0,
                _delegationRatio,
                0
            )
        );
        require(success, "Extension init failed");
        emit ExtensionImplementationSet(_extensionImpl);
    }

    /**
     * @inheritdoc IStakingBase
     */
    function setExtensionImpl(address _extensionImpl) external override onlyGovernor {
        extensionImpl = _extensionImpl;
        emit ExtensionImplementationSet(_extensionImpl);
    }

    /**
     * @inheritdoc IStakingBase
     */
    function setCounterpartStakingAddress(address _counterpart) external override onlyGovernor {
        counterpartStakingAddress = _counterpart;
        emit ParameterUpdated("counterpartStakingAddress");
    }

    /**
     * @inheritdoc IStakingBase
     */
    function setMinimumIndexerStake(uint256 _minimumIndexerStake) external override onlyGovernor {
        _setMinimumIndexerStake(_minimumIndexerStake);
    }

    /**
     * @inheritdoc IStakingBase
     */
    function setThawingPeriod(uint32 _thawingPeriod) external override onlyGovernor {
        _setThawingPeriod(_thawingPeriod);
    }

    /**
     * @inheritdoc IStakingBase
     */
    function setCurationPercentage(uint32 _percentage) external override onlyGovernor {
        _setCurationPercentage(_percentage);
    }

    /**
     * @inheritdoc IStakingBase
     */
    function setProtocolPercentage(uint32 _percentage) external override onlyGovernor {
        _setProtocolPercentage(_percentage);
    }

    /**
     * @inheritdoc IStakingBase
     */
    function setMaxAllocationEpochs(uint32 _maxAllocationEpochs) external override onlyGovernor {
        _setMaxAllocationEpochs(_maxAllocationEpochs);
    }

    /**
     * @inheritdoc IStakingBase
     */
    function setRebateParameters(
        uint32 _alphaNumerator,
        uint32 _alphaDenominator,
        uint32 _lambdaNumerator,
        uint32 _lambdaDenominator
    ) external override onlyGovernor {
        _setRebateParameters(_alphaNumerator, _alphaDenominator, _lambdaNumerator, _lambdaDenominator);
    }

    /**
     * @inheritdoc IStakingBase
     */
    function setOperator(address _operator, bool _allowed) external override {
        require(_operator != msg.sender, "operator == sender");
        __operatorAuth[msg.sender][_operator] = _allowed;
        emit SetOperator(msg.sender, _operator, _allowed);
    }

    /**
     * @inheritdoc IStakingBase
     */
    function stake(uint256 _tokens) external override {
        stakeTo(msg.sender, _tokens);
    }

    /**
     * @inheritdoc IStakingBase
     * @dev NOTE: The function accepts an amount greater than the currently staked tokens.
     * If that happens, it will try to unstake the max amount of tokens it can.
     * The reason for this behaviour is to avoid time conditions while the transaction
     * is in flight.
     */
    function unstake(uint256 _tokens) external override notPartialPaused {
        address indexer = msg.sender;
        IStakes.Indexer storage indexerStake = __stakes[indexer];

        require(indexerStake.tokensStaked > 0, "!stake");

        // Tokens to lock is capped to the available tokens
        uint256 tokensToLock = MathUtils.min(indexerStake.tokensAvailable(), _tokens);
        require(tokensToLock > 0, "!stake-avail");

        // Ensure minimum stake
        uint256 newStake = indexerStake.tokensSecureStake().sub(tokensToLock);
        require(newStake == 0 || newStake >= __minimumIndexerStake, "!minimumIndexerStake");

        // Before locking more tokens, withdraw any unlocked ones if possible
        uint256 tokensToWithdraw = indexerStake.tokensWithdrawable();
        if (tokensToWithdraw > 0) {
            _withdraw(indexer);
        }

        // Update the indexer stake locking tokens
        indexerStake.lockTokens(tokensToLock, __thawingPeriod);

        emit StakeLocked(indexer, indexerStake.tokensLocked, indexerStake.tokensLockedUntil);
    }

    /**
     * @inheritdoc IStakingBase
     */
    function withdraw() external override notPaused {
        _withdraw(msg.sender);
    }

    /**
     * @inheritdoc IStakingBase
     */
    function setRewardsDestination(address _destination) external override {
        __rewardsDestination[msg.sender] = _destination;
        emit SetRewardsDestination(msg.sender, _destination);
    }

    /**
     * @inheritdoc IStakingBase
     */
    function allocate(
        bytes32 _subgraphDeploymentID,
        uint256 _tokens,
        address _allocationID,
        bytes32 _metadata,
        bytes calldata _proof
    ) external override notPaused {
        _allocate(msg.sender, _subgraphDeploymentID, _tokens, _allocationID, _metadata, _proof);
    }

    /**
     * @inheritdoc IStakingBase
     */
    function allocateFrom(
        address _indexer,
        bytes32 _subgraphDeploymentID,
        uint256 _tokens,
        address _allocationID,
        bytes32 _metadata,
        bytes calldata _proof
    ) external override notPaused {
        _allocate(_indexer, _subgraphDeploymentID, _tokens, _allocationID, _metadata, _proof);
    }

    /**
     * @inheritdoc IStakingBase
     * @dev To be eligible for rewards a proof of indexing must be presented.
     * Presenting a bad proof is subject to slashable condition.
     * To opt out of rewards set _poi to 0x0
     */
    function closeAllocation(address _allocationID, bytes32 _poi) external override notPaused {
        _closeAllocation(_allocationID, _poi);
    }

    /**
     * @inheritdoc IStakingBase
     * @dev We use an exponential rebate formula to calculate the amount of tokens to rebate to the indexer.
     * This implementation allows collecting multiple times on the same allocation, keeping track of the
     * total amount rebated, the total amount collected and compensating the indexer for the difference.
     */
    function collect(uint256 _tokens, address _allocationID) external override {
        // Allocation identifier validation
        require(_allocationID != address(0), "!alloc");

        // Allocation must exist
        AllocationState allocState = _getAllocationState(_allocationID);
        require(allocState != AllocationState.Null, "!collect");

        // If the query fees are zero, we don't want to revert
        // but we also don't need to do anything, so just return
        if (_tokens == 0) {
            return;
        }

        Allocation storage alloc = __allocations[_allocationID];
        bytes32 subgraphDeploymentID = alloc.subgraphDeploymentID;

        uint256 queryFees = _tokens; // Tokens collected from the channel
        uint256 protocolTax = 0; // Tokens burnt as protocol tax
        uint256 curationFees = 0; // Tokens distributed to curators as curation fees
        uint256 queryRebates = 0; // Tokens to distribute to indexer
        uint256 delegationRewards = 0; // Tokens to distribute to delegators

        {
            // -- Pull tokens from the sender --
            IGraphToken graphToken = graphToken();
            TokenUtils.pullTokens(graphToken, msg.sender, queryFees);

            // -- Collect protocol tax --
            protocolTax = _collectTax(graphToken, queryFees, __protocolPercentage);
            queryFees = queryFees.sub(protocolTax);

            // -- Collect curation fees --
            // Only if the subgraph deployment is curated
            curationFees = _collectCurationFees(graphToken, subgraphDeploymentID, queryFees, __curationPercentage);
            queryFees = queryFees.sub(curationFees);

            // -- Process rebate reward --
            // Using accumulated fees and subtracting previously distributed rebates
            // allows for multiple vouchers to be collected while following the rebate formula
            alloc.collectedFees = alloc.collectedFees.add(queryFees);

            // No rebates if indexer has no stake or if lambda is zero
            uint256 newRebates = (alloc.tokens == 0 || __lambdaNumerator == 0)
                ? 0
                : LibExponential.exponentialRebates(
                    alloc.collectedFees,
                    alloc.tokens,
                    __alphaNumerator,
                    __alphaDenominator,
                    __lambdaNumerator,
                    __lambdaDenominator
                );

            //  -- Ensure rebates to distribute are within bounds --
            // Indexers can become under or over rebated if rebate parameters (alpha, lambda)
            // change between successive collect calls for the same allocation

            // Ensure rebates to distribute are not negative (indexer is over-rebated)
            queryRebates = MathUtils.diffOrZero(newRebates, alloc.distributedRebates);

            // Ensure rebates to distribute are not greater than available (indexer is under-rebated)
            queryRebates = MathUtils.min(queryRebates, queryFees);

            // -- Burn rebates remanent --
            TokenUtils.burnTokens(graphToken, queryFees.sub(queryRebates));

            // -- Distribute rebates --
            if (queryRebates > 0) {
                alloc.distributedRebates = alloc.distributedRebates.add(queryRebates);

                // -- Collect delegation rewards into the delegation pool --
                delegationRewards = _collectDelegationQueryRewards(alloc.indexer, queryRebates);
                queryRebates = queryRebates.sub(delegationRewards);

                // -- Transfer or restake rebates --
                _sendRewards(
                    graphToken,
                    queryRebates,
                    alloc.indexer,
                    __rewardsDestination[alloc.indexer] == address(0)
                );
            }
        }

        emit RebateCollected(
            msg.sender,
            alloc.indexer,
            subgraphDeploymentID,
            _allocationID,
            epochManager().currentEpoch(),
            _tokens,
            protocolTax,
            curationFees,
            queryFees,
            queryRebates,
            delegationRewards
        );
    }

    /**
     * @inheritdoc IStakingBase
     */
    function isAllocation(address _allocationID) external view override returns (bool) {
        return _getAllocationState(_allocationID) != AllocationState.Null;
    }

    /**
     * @inheritdoc IStakingBase
     */
    function hasStake(address _indexer) external view override returns (bool) {
        return __stakes[_indexer].tokensStaked > 0;
    }

    /**
     * @inheritdoc IStakingBase
     */
    function getAllocation(address _allocationID) external view override returns (Allocation memory) {
        return __allocations[_allocationID];
    }

    /**
     * @inheritdoc IStakingBase
     */
    function getAllocationData(
        address _allocationID
    ) external view override returns (bool, address, bytes32, uint256, uint256, uint256) {
        Allocation memory alloc = __allocations[_allocationID];
        bool isActive = _getAllocationState(_allocationID) == AllocationState.Active;

        return (
            isActive,
            alloc.indexer,
            alloc.subgraphDeploymentID,
            alloc.tokens,
            alloc.accRewardsPerAllocatedToken,
            0
        );
    }

    /**
     * @inheritdoc IStakingBase
     */
    function isActiveAllocation(address _allocationID) external view override returns (bool) {
        return _getAllocationState(_allocationID) == AllocationState.Active;
    }

    /**
     * @inheritdoc IStakingBase
     */
    function getAllocationState(address _allocationID) external view override returns (AllocationState) {
        return _getAllocationState(_allocationID);
    }

    /**
     * @inheritdoc IStakingBase
     */
    function getSubgraphAllocatedTokens(bytes32 _subgraphDeploymentID) external view override returns (uint256) {
        return __subgraphAllocations[_subgraphDeploymentID];
    }

    /**
     * @inheritdoc IStakingBase
     */
    function getIndexerStakedTokens(address _indexer) external view override returns (uint256) {
        return __stakes[_indexer].tokensStaked;
    }

    /**
     * @inheritdoc IStakingBase
     */
    function stakeTo(address _indexer, uint256 _tokens) public override notPartialPaused {
        require(_tokens > 0, "!tokens");

        // Transfer tokens to stake from caller to this contract
        TokenUtils.pullTokens(graphToken(), msg.sender, _tokens);

        // Stake the transferred tokens
        _stake(_indexer, _tokens);
    }

    /**
     * @inheritdoc IStakingBase
     */
    function setDelegationParameters(
        uint32 _indexingRewardCut,
        uint32 _queryFeeCut,
        uint32 // _cooldownBlocks, deprecated
    ) public override {
        _setDelegationParameters(msg.sender, _indexingRewardCut, _queryFeeCut);
    }

    /**
     * @inheritdoc IStakingBase
     */
    function getIndexerCapacity(address _indexer) public view override returns (uint256) {
        IStakes.Indexer memory indexerStake = __stakes[_indexer];
        uint256 tokensDelegated = __delegationPools[_indexer].tokens;

        uint256 tokensDelegatedCap = indexerStake.tokensSecureStake().mul(uint256(__delegationRatio));
        uint256 tokensDelegatedCapacity = MathUtils.min(tokensDelegated, tokensDelegatedCap);

        return indexerStake.tokensAvailableWithDelegation(tokensDelegatedCapacity);
    }

    /**
     * @inheritdoc IStakingBase
     */
    function isOperator(address _operator, address _indexer) public view override returns (bool) {
        return __operatorAuth[_indexer][_operator];
    }

    /**
     * @notice Internal: Set the minimum indexer stake required.
     * @param _minimumIndexerStake Minimum indexer stake
     */
    function _setMinimumIndexerStake(uint256 _minimumIndexerStake) private {
        require(_minimumIndexerStake > 0, "!minimumIndexerStake");
        __minimumIndexerStake = _minimumIndexerStake;
        emit ParameterUpdated("minimumIndexerStake");
    }

    /**
     * @notice Internal: Set the thawing period for unstaking.
     * @param _thawingPeriod Period in blocks to wait for token withdrawals after unstaking
     */
    function _setThawingPeriod(uint32 _thawingPeriod) private {
        require(_thawingPeriod > 0, "!thawingPeriod");
        __thawingPeriod = _thawingPeriod;
        emit ParameterUpdated("thawingPeriod");
    }

    /**
     * @notice Internal: Set the curation percentage of query fees sent to curators.
     * @param _percentage Percentage of query fees sent to curators
     */
    function _setCurationPercentage(uint32 _percentage) private {
        // Must be within 0% to 100% (inclusive)
        require(_percentage <= MAX_PPM, ">percentage");
        __curationPercentage = _percentage;
        emit ParameterUpdated("curationPercentage");
    }

    /**
     * @notice Internal: Set a protocol percentage to burn when collecting query fees.
     * @param _percentage Percentage of query fees to burn as protocol fee
     */
    function _setProtocolPercentage(uint32 _percentage) private {
        // Must be within 0% to 100% (inclusive)
        require(_percentage <= MAX_PPM, ">percentage");
        __protocolPercentage = _percentage;
        emit ParameterUpdated("protocolPercentage");
    }

    /**
     * @notice Internal: Set the max time allowed for indexers stake on allocations.
     * @param _maxAllocationEpochs Allocation duration limit in epochs
     */
    function _setMaxAllocationEpochs(uint32 _maxAllocationEpochs) private {
        __maxAllocationEpochs = _maxAllocationEpochs;
        emit ParameterUpdated("maxAllocationEpochs");
    }

    /**
     * @notice Set the rebate parameters.
     * @param _alphaNumerator Numerator of `alpha` in the rebates function
     * @param _alphaDenominator Denominator of `alpha` in the rebates function
     * @param _lambdaNumerator Numerator of `lambda` in the rebates function
     * @param _lambdaDenominator Denominator of `lambda` in the rebates function
     */
    function _setRebateParameters(
        uint32 _alphaNumerator,
        uint32 _alphaDenominator,
        uint32 _lambdaNumerator,
        uint32 _lambdaDenominator
    ) private {
        require(_alphaDenominator > 0, "!alphaDenominator");
        require(_lambdaNumerator > 0, "!lambdaNumerator");
        require(_lambdaDenominator > 0, "!lambdaDenominator");
        __alphaNumerator = _alphaNumerator;
        __alphaDenominator = _alphaDenominator;
        __lambdaNumerator = _lambdaNumerator;
        __lambdaDenominator = _lambdaDenominator;
        emit ParameterUpdated("rebateParameters");
    }

    /**
     * @notice Set the delegation parameters for a particular indexer.
     * @param _indexer Indexer to set delegation parameters
     * @param _indexingRewardCut Percentage of indexing rewards left for delegators
     * @param _queryFeeCut Percentage of query fees left for delegators
     */
    function _setDelegationParameters(address _indexer, uint32 _indexingRewardCut, uint32 _queryFeeCut) internal {
        // Incentives must be within bounds
        require(_queryFeeCut <= MAX_PPM, ">queryFeeCut");
        require(_indexingRewardCut <= MAX_PPM, ">indexingRewardCut");

        DelegationPool storage pool = __delegationPools[_indexer];

        // Update delegation params
        pool.indexingRewardCut = _indexingRewardCut;
        pool.queryFeeCut = _queryFeeCut;
        pool.updatedAtBlock = block.number;

        emit DelegationParametersUpdated(_indexer, _indexingRewardCut, _queryFeeCut, 0);
    }

    /**
     * @notice Stake tokens on the indexer.
     * This function does not check minimum indexer stake requirement to allow
     * to be called by functions that increase the stake when collecting rewards
     * without reverting
     * @param _indexer Address of staking party
     * @param _tokens Amount of tokens to stake
     */
    function _stake(address _indexer, uint256 _tokens) internal {
        // Ensure minimum stake
        require(__stakes[_indexer].tokensSecureStake().add(_tokens) >= __minimumIndexerStake, "!minimumIndexerStake");

        // Deposit tokens into the indexer stake
        __stakes[_indexer].deposit(_tokens);

        // Initialize the delegation pool the first time
        if (__delegationPools[_indexer].updatedAtBlock == 0) {
            _setDelegationParameters(_indexer, MAX_PPM, MAX_PPM);
        }

        emit StakeDeposited(_indexer, _tokens);
    }

    /**
     * @notice Withdraw indexer tokens once the thawing period has passed.
     * @param _indexer Address of indexer to withdraw funds from
     */
    function _withdraw(address _indexer) private {
        // Get tokens available for withdraw and update balance
        uint256 tokensToWithdraw = __stakes[_indexer].withdrawTokens();
        require(tokensToWithdraw > 0, "!tokens");

        // Return tokens to the indexer
        TokenUtils.pushTokens(graphToken(), _indexer, tokensToWithdraw);

        emit StakeWithdrawn(_indexer, tokensToWithdraw);
    }

    /**
     * @notice Allocate available tokens to a subgraph deployment.
     * @param _indexer Indexer address to allocate funds from.
     * @param _subgraphDeploymentID ID of the SubgraphDeployment where tokens will be allocated
     * @param _tokens Amount of tokens to allocate
     * @param _allocationID The allocationID will work to identify collected funds related to this allocation
     * @param _metadata Metadata related to the allocation
     * @param _proof A 65-bytes Ethereum signed message of `keccak256(indexerAddress,allocationID)`
     */
    function _allocate(
        address _indexer,
        bytes32 _subgraphDeploymentID,
        uint256 _tokens,
        address _allocationID,
        bytes32 _metadata,
        bytes calldata _proof
    ) private {
        require(_isAuth(_indexer), "!auth");

        // Check allocation
        require(_allocationID != address(0), "!alloc");
        require(_getAllocationState(_allocationID) == AllocationState.Null, "!null");

        // Caller must prove that they own the private key for the allocationID address
        // The proof is an Ethereum signed message of KECCAK256(indexerAddress,allocationID)
        bytes32 messageHash = keccak256(abi.encodePacked(_indexer, _allocationID));
        bytes32 digest = ECDSA.toEthSignedMessageHash(messageHash);
        require(ECDSA.recover(digest, _proof) == _allocationID, "!proof");

        require(__stakes[_indexer].tokensSecureStake() >= __minimumIndexerStake, "!minimumIndexerStake");
        if (_tokens > 0) {
            // Needs to have free capacity not used for other purposes to allocate
            require(getIndexerCapacity(_indexer) >= _tokens, "!capacity");
        }

        // Creates an allocation
        // Allocation identifiers are not reused
        // Anyone can send collected funds to the allocation using collect()
        Allocation memory alloc = Allocation(
            _indexer,
            _subgraphDeploymentID,
            _tokens, // Tokens allocated
            epochManager().currentEpoch(), // createdAtEpoch
            0, // closedAtEpoch
            0, // Initialize collected fees
            0, // Initialize effective allocation (DEPRECATED)
            (_tokens > 0) ? _updateRewards(_subgraphDeploymentID) : 0, // Initialize accumulated rewards per stake allocated
            0 // Initialize distributed rebates
        );
        __allocations[_allocationID] = alloc;

        // -- Rewards Distribution --

        // Process non-zero-allocation rewards tracking
        if (_tokens > 0) {
            // Mark allocated tokens as used
            __stakes[_indexer].allocate(alloc.tokens);

            // Track total allocations per subgraph
            // Used for rewards calculations
            __subgraphAllocations[alloc.subgraphDeploymentID] = __subgraphAllocations[alloc.subgraphDeploymentID].add(
                alloc.tokens
            );
        }

        emit AllocationCreated(
            _indexer,
            _subgraphDeploymentID,
            alloc.createdAtEpoch,
            alloc.tokens,
            _allocationID,
            _metadata
        );
    }

    /**
     * @notice Close an allocation and free the staked tokens.
     * @param _allocationID The allocation identifier
     * @param _poi Proof of indexing submitted for the allocated period
     */
    function _closeAllocation(address _allocationID, bytes32 _poi) private {
        // Allocation must exist and be active
        AllocationState allocState = _getAllocationState(_allocationID);
        require(allocState == AllocationState.Active, "!active");

        // Get allocation
        Allocation memory alloc = __allocations[_allocationID];

        alloc.closedAtEpoch = epochManager().currentEpoch();

        // Allocation duration in epochs
        uint256 epochs = MathUtils.diffOrZero(alloc.closedAtEpoch, alloc.createdAtEpoch);

        // Indexer or operator can close an allocation
        // Anyone is allowed to close ONLY under two concurrent conditions
        // - After maxAllocationEpochs passed
        // - When the allocation is for non-zero amount of tokens
        bool isIndexerOrOperator = _isAuth(alloc.indexer);
        if (epochs <= __maxAllocationEpochs || alloc.tokens == 0) {
            require(isIndexerOrOperator, "!auth");
        }

        // -- Rewards Distribution --

        // Process non-zero-allocation rewards tracking
        if (alloc.tokens > 0) {
            // Distribute rewards if proof of indexing was presented by the indexer or operator
            // and the allocation is at least one epoch old (most indexed chains require the EBO
            // posting epoch block numbers to produce a valid POI which happens once per epoch)
            if (isIndexerOrOperator && _poi != 0 && epochs > 0) {
                _distributeRewards(_allocationID, alloc.indexer);
            } else {
                _updateRewards(alloc.subgraphDeploymentID);
            }

            // Free allocated tokens from use
            __stakes[alloc.indexer].unallocate(alloc.tokens);

            // Track total allocations per subgraph
            // Used for rewards calculations
            __subgraphAllocations[alloc.subgraphDeploymentID] = __subgraphAllocations[alloc.subgraphDeploymentID].sub(
                alloc.tokens
            );
        }

        // Close the allocation
        // Note that this breaks CEI pattern. We update after the rewards distribution logic as it expects the allocation
        // to still be active. There shouldn't be reentrancy risk here as all internal calls are to trusted contracts.
        __allocations[_allocationID].closedAtEpoch = alloc.closedAtEpoch;

        emit AllocationClosed(
            alloc.indexer,
            alloc.subgraphDeploymentID,
            alloc.closedAtEpoch,
            alloc.tokens,
            _allocationID,
            msg.sender,
            _poi,
            !isIndexerOrOperator
        );
    }

    /**
     * @notice Collect the delegation rewards for query fees.
     * This function will assign the collected fees to the delegation pool.
     * @param _indexer Indexer to which the tokens to distribute are related
     * @param _tokens Total tokens received used to calculate the amount of fees to collect
     * @return Amount of delegation rewards
     */
    function _collectDelegationQueryRewards(address _indexer, uint256 _tokens) private returns (uint256) {
        uint256 delegationRewards = 0;
        DelegationPool storage pool = __delegationPools[_indexer];
        if (pool.tokens > 0 && pool.queryFeeCut < MAX_PPM) {
            uint256 indexerCut = uint256(pool.queryFeeCut).mul(_tokens).div(MAX_PPM);
            delegationRewards = _tokens.sub(indexerCut);
            pool.tokens = pool.tokens.add(delegationRewards);
        }
        return delegationRewards;
    }

    /**
     * @notice Collect the delegation rewards for indexing.
     * This function will assign the collected fees to the delegation pool.
     * @param _indexer Indexer to which the tokens to distribute are related
     * @param _tokens Total tokens received used to calculate the amount of fees to collect
     * @return Amount of delegation rewards
     */
    function _collectDelegationIndexingRewards(address _indexer, uint256 _tokens) private returns (uint256) {
        uint256 delegationRewards = 0;
        DelegationPool storage pool = __delegationPools[_indexer];
        if (pool.tokens > 0 && pool.indexingRewardCut < MAX_PPM) {
            uint256 indexerCut = uint256(pool.indexingRewardCut).mul(_tokens).div(MAX_PPM);
            delegationRewards = _tokens.sub(indexerCut);
            pool.tokens = pool.tokens.add(delegationRewards);
        }
        return delegationRewards;
    }

    /**
     * @notice Collect the curation fees for a subgraph deployment from an amount of tokens.
     * This function transfer curation fees to the Curation contract by calling Curation.collect
     * @param _graphToken Token to collect
     * @param _subgraphDeploymentID Subgraph deployment to which the curation fees are related
     * @param _tokens Total tokens received used to calculate the amount of fees to collect
     * @param _curationPercentage Percentage of tokens to collect as fees
     * @return Amount of curation fees
     */
    function _collectCurationFees(
        IGraphToken _graphToken,
        bytes32 _subgraphDeploymentID,
        uint256 _tokens,
        uint256 _curationPercentage
    ) private returns (uint256) {
        if (_tokens == 0) {
            return 0;
        }

        ICuration curation = curation();
        bool isCurationEnabled = _curationPercentage > 0 && address(curation) != address(0);

        if (isCurationEnabled && curation.isCurated(_subgraphDeploymentID)) {
            // Calculate the tokens after curation fees first, and subtact that,
            // to prevent curation fees from rounding down to zero
            uint256 tokensAfterCurationFees = uint256(MAX_PPM).sub(_curationPercentage).mul(_tokens).div(MAX_PPM);
            uint256 curationFees = _tokens.sub(tokensAfterCurationFees);
            if (curationFees > 0) {
                // Transfer and call collect()
                // This function transfer tokens to a trusted protocol contracts
                // Then we call collect() to do the transfer bookkeeping
                rewardsManager().onSubgraphSignalUpdate(_subgraphDeploymentID);
                TokenUtils.pushTokens(_graphToken, address(curation), curationFees);
                curation.collect(_subgraphDeploymentID, curationFees);
            }
            return curationFees;
        }
        return 0;
    }

    /**
     * @notice Collect tax to burn for an amount of tokens.
     * @param _graphToken Token to burn
     * @param _tokens Total tokens received used to calculate the amount of tax to collect
     * @param _percentage Percentage of tokens to burn as tax
     * @return Amount of tax charged
     */
    function _collectTax(IGraphToken _graphToken, uint256 _tokens, uint256 _percentage) private returns (uint256) {
        // Calculate tokens after tax first, and subtract that,
        // to prevent the tax from rounding down to zero
        uint256 tokensAfterTax = uint256(MAX_PPM).sub(_percentage).mul(_tokens).div(MAX_PPM);
        uint256 tax = _tokens.sub(tokensAfterTax);
        TokenUtils.burnTokens(_graphToken, tax); // Burn tax if any
        return tax;
    }

    /**
     * @notice Triggers an update of rewards due to a change in allocations.
     * @param _subgraphDeploymentID Subgraph deployment updated
     * @return Accumulated rewards per allocated token for the subgraph deployment
     */
    function _updateRewards(bytes32 _subgraphDeploymentID) private returns (uint256) {
        IRewardsManager rewardsManager = rewardsManager();
        if (address(rewardsManager) == address(0)) {
            return 0;
        }
        return rewardsManager.onSubgraphAllocationUpdate(_subgraphDeploymentID);
    }

    /**
     * @notice Assign rewards for the closed allocation to indexer and delegators.
     * @param _allocationID Allocation
     * @param _indexer Address of the indexer that did the allocation
     */
    function _distributeRewards(address _allocationID, address _indexer) private {
        IRewardsManager rewardsManager = rewardsManager();
        if (address(rewardsManager) == address(0)) {
            return;
        }

        // Automatically triggers update of rewards snapshot as allocation will change
        // after this call. Take rewards mint tokens for the Staking contract to distribute
        // between indexer and delegators
        uint256 totalRewards = rewardsManager.takeRewards(_allocationID);
        if (totalRewards == 0) {
            return;
        }

        // Calculate delegation rewards and add them to the delegation pool
        uint256 delegationRewards = _collectDelegationIndexingRewards(_indexer, totalRewards);
        uint256 indexerRewards = totalRewards.sub(delegationRewards);

        // Send the indexer rewards
        _sendRewards(graphToken(), indexerRewards, _indexer, __rewardsDestination[_indexer] == address(0));
    }

    /**
     * @notice Send rewards to the appropriate destination.
     * @param _graphToken Graph token
     * @param _amount Number of rewards tokens
     * @param _beneficiary Address of the beneficiary of rewards
     * @param _restake Whether to restake or not
     */
    function _sendRewards(IGraphToken _graphToken, uint256 _amount, address _beneficiary, bool _restake) private {
        if (_amount == 0) return;

        if (_restake) {
            // Restake to place fees into the indexer stake
            _stake(_beneficiary, _amount);
        } else {
            // Transfer funds to the beneficiary's designated rewards destination if set
            address destination = __rewardsDestination[_beneficiary];
            TokenUtils.pushTokens(_graphToken, destination == address(0) ? _beneficiary : destination, _amount);
        }
    }

    /**
     * @notice Check if the caller is authorized to operate on behalf of
     * an indexer (i.e. the caller is the indexer or an operator)
     * @param _indexer Indexer address
     * @return True if the caller is authorized to operate on behalf of the indexer
     */
    function _isAuth(address _indexer) private view returns (bool) {
        return msg.sender == _indexer || isOperator(msg.sender, _indexer) == true;
    }

    /**
     * @notice Return the current state of an allocation
     * @param _allocationID Allocation identifier
     * @return AllocationState enum with the state of the allocation
     */
    function _getAllocationState(address _allocationID) private view returns (AllocationState) {
        Allocation storage alloc = __allocations[_allocationID];

        if (alloc.indexer == address(0)) {
            return AllocationState.Null;
        }

        if (alloc.createdAtEpoch != 0 && alloc.closedAtEpoch == 0) {
            return AllocationState.Active;
        }

        return AllocationState.Closed;
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable named-parameters-mapping

pragma solidity ^0.7.6;
pragma abicoder v2;

import { IL1GraphTokenLockTransferTool } from "@graphprotocol/interfaces/contracts/contracts/staking/IL1GraphTokenLockTransferTool.sol";

/**
 * @title L1StakingV1Storage
 * @author Edge & Node
 * @notice This contract holds all the L1-specific storage variables for the L1Staking contract, version 1
 * @dev When adding new versions, make sure to move the gap to the new version and
 * reduce the size of the gap accordingly.
 */
abstract contract L1StakingV1Storage {
    /// @notice If an indexer has transferred to L2, this mapping will hold the indexer's address in L2
    mapping(address => address) public indexerTransferredToL2;
    /// @dev For locked indexers/delegations, this contract holds the mapping of L1 to L2 addresses
    IL1GraphTokenLockTransferTool internal l1GraphTokenLockTransferTool;
    /// @dev Storage gap to keep storage slots fixed in future versions
    uint256[50] private __gap;
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
// solhint-disable one-contract-per-file

pragma solidity ^0.7.6;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable one-contract-per-file, max-states-count
// solhint-disable named-parameters-mapping

import { Managed } from "../governance/Managed.sol";

import { IStakingData } from "@graphprotocol/interfaces/contracts/contracts/staking/IStakingData.sol";
import { IStakes } from "@graphprotocol/interfaces/contracts/contracts/staking/libs/IStakes.sol";

/**
 * @title StakingV1Storage
 * @author Edge & Node
 * @notice This contract holds all the storage variables for the Staking contract, version 1
 * @dev Note that we use a double underscore prefix for variable names; this prefix identifies
 * variables that used to be public but are now internal, getters can be found on StakingExtension.sol.
 */
contract StakingV1Storage is Managed {
    // -- Staking --

    /// @dev Minimum amount of tokens an indexer needs to stake
    uint256 internal __minimumIndexerStake;

    /// @dev Time in blocks to unstake
    uint32 internal __thawingPeriod; // in blocks

    /// @dev Percentage of fees going to curators
    /// Parts per million. (Allows for 4 decimal points, 999,999 = 99.9999%)
    uint32 internal __curationPercentage;

    /// @dev Percentage of fees burned as protocol fee
    /// Parts per million. (Allows for 4 decimal points, 999,999 = 99.9999%)
    uint32 internal __protocolPercentage;

    /// @dev Period for allocation to be finalized
    uint32 private __DEPRECATED_channelDisputeEpochs; // solhint-disable-line var-name-mixedcase

    /// @dev Maximum allocation time
    uint32 internal __maxAllocationEpochs;

    /// @dev Rebate alpha numerator
    // Originally used for Cobb-Douglas rebates, now used for exponential rebates
    uint32 internal __alphaNumerator;

    /// @dev Rebate alpha denominator
    // Originally used for Cobb-Douglas rebates, now used for exponential rebates
    uint32 internal __alphaDenominator;

    /// @dev Indexer stakes : indexer => Stake
    mapping(address => IStakes.Indexer) internal __stakes;

    /// @dev Allocations : allocationID => Allocation
    mapping(address => IStakingData.Allocation) internal __allocations;

    /// @dev Subgraph Allocations: subgraphDeploymentID => tokens
    mapping(bytes32 => uint256) internal __subgraphAllocations;

    /// @dev Deprecated rebate pools mapping (no longer used)
    mapping(uint256 => uint256) private __DEPRECATED_rebates; // solhint-disable-line var-name-mixedcase

    // -- Slashing --

    /// @dev List of addresses allowed to slash stakes
    mapping(address => bool) internal __slashers;

    // -- Delegation --

    /// @dev Set the delegation capacity multiplier defined by the delegation ratio
    /// If delegation ratio is 100, and an Indexer has staked 5 GRT,
    /// then they can use up to 500 GRT from the delegated stake
    uint32 internal __delegationRatio;

    /// @dev Time in blocks an indexer needs to wait to change delegation parameters (deprecated)
    uint32 internal __DEPRECATED_delegationParametersCooldown; // solhint-disable-line var-name-mixedcase

    /// @dev Time in epochs a delegator needs to wait to withdraw delegated stake
    uint32 internal __delegationUnbondingPeriod; // in epochs

    /// @dev Percentage of tokens to tax a delegation deposit
    /// Parts per million. (Allows for 4 decimal points, 999,999 = 99.9999%)
    uint32 internal __delegationTaxPercentage;

    /// @dev Delegation pools : indexer => DelegationPool
    mapping(address => IStakingData.DelegationPool) internal __delegationPools;

    // -- Operators --

    /// @dev Operator auth : indexer => operator => is authorized
    mapping(address => mapping(address => bool)) internal __operatorAuth;

    // -- Asset Holders --

    /// @dev DEPRECATED: Allowed AssetHolders: assetHolder => is allowed
    mapping(address => bool) private __DEPRECATED_assetHolders; // solhint-disable-line var-name-mixedcase
}

/**
 * @title StakingV2Storage
 * @author Edge & Node
 * @notice This contract holds all the storage variables for the Staking contract, version 2
 * @dev Note that we use a double underscore prefix for variable names; this prefix identifies
 * variables that used to be public but are now internal, getters can be found on StakingExtension.sol.
 */
contract StakingV2Storage is StakingV1Storage {
    /// @dev Destination of accrued rewards : beneficiary => rewards destination
    mapping(address => address) internal __rewardsDestination;
}

/**
 * @title StakingV3Storage
 * @author Edge & Node
 * @notice This contract holds all the storage variables for the base Staking contract, version 3.
 */
contract StakingV3Storage is StakingV2Storage {
    /// @dev Address of the counterpart Staking contract on L1/L2
    address internal counterpartStakingAddress;
    /// @dev Address of the StakingExtension implementation
    address internal extensionImpl;
}

/**
 * @title StakingV4Storage
 * @author Edge & Node
 * @notice This contract holds all the storage variables for the base Staking contract, version 4.
 * @dev Note that it includes a storage gap - if adding future versions, make sure to move the gap
 * to the new version and reduce the size of the gap accordingly.
 */
contract StakingV4Storage is StakingV3Storage {
    /// @dev Numerator for the lambda parameter in exponential rebate calculations
    uint32 internal __lambdaNumerator;
    /// @dev Denominator for the lambda parameter in exponential rebate calculations
    uint32 internal __lambdaDenominator;

    /// @dev Gap to allow adding variables in future upgrades (since L1Staking and L2Staking can have their own storage as well)
    uint256[50] private __gap;
}

// SPDX-License-Identifier: GPL-2.0-or-later

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable gas-strict-inequalities

pragma solidity 0.8.27 || 0.8.33;

/**
 * @title MathUtils Library
 * @author Edge & Node
 * @notice A collection of functions to perform math operations
 * @custom:security-contact Please email security+contracts@thegraph.com if you find any
 * bugs. We may have an active bug bounty program.
 */
library MathUtils {
    /**
     * @notice Calculates the weighted average of two values pondering each of these
     * values based on configured weights
     * @dev The contribution of each value N is
     * weightN/(weightA + weightB). The calculation rounds up to ensure the result
     * is always equal or greater than the smallest of the two values.
     * @param valueA The amount for value A
     * @param weightA The weight to use for value A
     * @param valueB The amount for value B
     * @param weightB The weight to use for value B
     * @return The weighted average result
     */
    function weightedAverageRoundingUp(
        uint256 valueA,
        uint256 weightA,
        uint256 valueB,
        uint256 weightB
    ) internal pure returns (uint256) {
        return ((valueA * weightA) + (valueB * weightB) + (weightA + weightB - 1)) / (weightA + weightB);
    }

    /**
     * @notice Returns the minimum of two numbers
     * @param x The first number
     * @param y The second number
     * @return The minimum of the two numbers
     */
    function min(uint256 x, uint256 y) internal pure returns (uint256) {
        return x <= y ? x : y;
    }

    /**
     * @notice Returns the difference between two numbers or zero if negative
     * @param x The first number
     * @param y The second number
     * @return The difference between the two numbers or zero if negative
     */
    function diffOrZero(uint256 x, uint256 y) internal pure returns (uint256) {
        return (x > y) ? x - y : 0;
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity ^0.7.3;
pragma experimental ABIEncoderV2;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable use-natspec

import { SafeMath } from "@openzeppelin/contracts/math/SafeMath.sol";

/**
 * @title A collection of data structures and functions to manage the Indexer Stake state.
 *        Used for low-level state changes, require() conditions should be evaluated
 *        at the caller function scope.
 */
library Stakes {
    using SafeMath for uint256;
    using Stakes for Stakes.Indexer;

    struct Indexer {
        uint256 tokensStaked; // Tokens on the indexer stake (staked by the indexer)
        uint256 tokensAllocated; // Tokens used in allocations
        uint256 tokensLocked; // Tokens locked for withdrawal subject to thawing period
        uint256 tokensLockedUntil; // Block when locked tokens can be withdrawn
    }

    /**
     * @dev Deposit tokens to the indexer stake.
     * @param stake Stake data
     * @param _tokens Amount of tokens to deposit
     */
    function deposit(Stakes.Indexer storage stake, uint256 _tokens) internal {
        stake.tokensStaked = stake.tokensStaked.add(_tokens);
    }

    /**
     * @dev Release tokens from the indexer stake.
     * @param stake Stake data
     * @param _tokens Amount of tokens to release
     */
    function release(Stakes.Indexer storage stake, uint256 _tokens) internal {
        stake.tokensStaked = stake.tokensStaked.sub(_tokens);
    }

    /**
     * @dev Allocate tokens from the main stack to a SubgraphDeployment.
     * @param stake Stake data
     * @param _tokens Amount of tokens to allocate
     */
    function allocate(Stakes.Indexer storage stake, uint256 _tokens) internal {
        stake.tokensAllocated = stake.tokensAllocated.add(_tokens);
    }

    /**
     * @dev Unallocate tokens from a SubgraphDeployment back to the main stack.
     * @param stake Stake data
     * @param _tokens Amount of tokens to unallocate
     */
    function unallocate(Stakes.Indexer storage stake, uint256 _tokens) internal {
        stake.tokensAllocated = stake.tokensAllocated.sub(_tokens);
    }

    /**
     * @dev Lock tokens until a thawing period pass.
     * @param stake Stake data
     * @param _tokens Amount of tokens to unstake
     * @param _period Period in blocks that need to pass before withdrawal
     */
    function lockTokens(Stakes.Indexer storage stake, uint256 _tokens, uint256 _period) internal {
        // Take into account period averaging for multiple unstake requests
        uint256 lockingPeriod = _period;
        if (stake.tokensLocked > 0) {
            lockingPeriod = stake.getLockingPeriod(_tokens, _period);
        }

        // Update balances
        stake.tokensLocked = stake.tokensLocked.add(_tokens);
        stake.tokensLockedUntil = block.number.add(lockingPeriod);
    }

    /**
     * @dev Unlock tokens.
     * @param stake Stake data
     * @param _tokens Amount of tokens to unkock
     */
    function unlockTokens(Stakes.Indexer storage stake, uint256 _tokens) internal {
        stake.tokensLocked = stake.tokensLocked.sub(_tokens);
        if (stake.tokensLocked == 0) {
            stake.tokensLockedUntil = 0;
        }
    }

    /**
     * @dev Take all tokens out from the locked stake for withdrawal.
     * @param stake Stake data
     * @return Amount of tokens being withdrawn
     */
    function withdrawTokens(Stakes.Indexer storage stake) internal returns (uint256) {
        // Calculate tokens that can be released
        uint256 tokensToWithdraw = stake.tokensWithdrawable();

        if (tokensToWithdraw > 0) {
            // Reset locked tokens
            stake.unlockTokens(tokensToWithdraw);

            // Decrease indexer stake
            stake.release(tokensToWithdraw);
        }

        return tokensToWithdraw;
    }

    /**
     * @dev Get the locking period of the tokens to unstake.
     * If already unstaked before calculate the weighted average.
     * @param stake Stake data
     * @param _tokens Amount of tokens to unstake
     * @param _thawingPeriod Period in blocks that need to pass before withdrawal
     * @return True if staked
     */
    function getLockingPeriod(
        Stakes.Indexer memory stake,
        uint256 _tokens,
        uint256 _thawingPeriod
    ) internal view returns (uint256) {
        uint256 blockNum = block.number;
        uint256 periodA = (stake.tokensLockedUntil > blockNum) ? stake.tokensLockedUntil.sub(blockNum) : 0;
        uint256 periodB = _thawingPeriod;
        uint256 stakeA = stake.tokensLocked;
        uint256 stakeB = _tokens;
        return periodA.mul(stakeA).add(periodB.mul(stakeB)).div(stakeA.add(stakeB));
    }

    /**
     * @dev Return true if there are tokens staked by the Indexer.
     * @param stake Stake data
     * @return True if staked
     */
    function hasTokens(Stakes.Indexer memory stake) internal pure returns (bool) {
        return stake.tokensStaked > 0;
    }

    /**
     * @dev Return the amount of tokens used in allocations and locked for withdrawal.
     * @param stake Stake data
     * @return Token amount
     */
    function tokensUsed(Stakes.Indexer memory stake) internal pure returns (uint256) {
        return stake.tokensAllocated.add(stake.tokensLocked);
    }

    /**
     * @dev Return the amount of tokens staked not considering the ones that are already going
     * through the thawing period or are ready for withdrawal. We call it secure stake because
     * it is not subject to change by a withdraw call from the indexer.
     * @param stake Stake data
     * @return Token amount
     */
    function tokensSecureStake(Stakes.Indexer memory stake) internal pure returns (uint256) {
        return stake.tokensStaked.sub(stake.tokensLocked);
    }

    /**
     * @dev Tokens free balance on the indexer stake that can be used for any purpose.
     * Any token that is allocated cannot be used as well as tokens that are going through the
     * thawing period or are withdrawable
     * Calc: tokensStaked - tokensAllocated - tokensLocked
     * @param stake Stake data
     * @return Token amount
     */
    function tokensAvailable(Stakes.Indexer memory stake) internal pure returns (uint256) {
        return stake.tokensAvailableWithDelegation(0);
    }

    /**
     * @dev Tokens free balance on the indexer stake that can be used for allocations.
     * This function accepts a parameter for extra delegated capacity that takes into
     * account delegated tokens
     * @param stake Stake data
     * @param _delegatedCapacity Amount of tokens used from delegators to calculate availability
     * @return Token amount
     */
    function tokensAvailableWithDelegation(
        Stakes.Indexer memory stake,
        uint256 _delegatedCapacity
    ) internal pure returns (uint256) {
        uint256 tokensCapacity = stake.tokensStaked.add(_delegatedCapacity);
        uint256 _tokensUsed = stake.tokensUsed();
        // If more tokens are used than the current capacity, the indexer is overallocated.
        // This means the indexer doesn't have available capacity to create new allocations.
        // We can reach this state when the indexer has funds allocated and then any
        // of these conditions happen:
        // - The delegationCapacity ratio is reduced.
        // - The indexer stake is slashed.
        // - A delegator removes enough stake.
        if (_tokensUsed > tokensCapacity) {
            // Indexer stake is over allocated: return 0 to avoid stake to be used until
            // the overallocation is restored by staking more tokens, unallocating tokens
            // or using more delegated funds
            return 0;
        }
        return tokensCapacity.sub(_tokensUsed);
    }

    /**
     * @dev Tokens available for withdrawal after thawing period.
     * @param stake Stake data
     * @return Token amount
     */
    function tokensWithdrawable(Stakes.Indexer memory stake) internal view returns (uint256) {
        // No tokens to withdraw before locking period
        if (stake.tokensLockedUntil == 0 || block.number < stake.tokensLockedUntil) {
            return 0;
        }
        return stake.tokensLocked;
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity ^0.7.6;

import { LibFixedMath } from "./LibFixedMath.sol";

/**
 * @title LibExponential library
 * @author Edge & Node
 * @notice A library to compute query fee rebates using an exponential formula
 */
library LibExponential {
    /// @dev Maximum value of the exponent for which to compute the exponential before clamping to zero.
    uint32 private constant MAX_EXPONENT = 15;

    /// @notice The exponential formula used to compute fee-based rewards for
    ///      staking pools in a given epoch. This function does not perform
    ///      bounds checking on the inputs, but the following conditions
    ///      need to be true:
    ///         0 <= alphaNumerator / alphaDenominator <= 1
    ///         0 < lambdaNumerator / lambdaDenominator
    ///      The exponential rebates function has the form:
    ///      `(1 - alpha * exp ^ (-lambda * stake / fees)) * fees`
    /// @param fees Fees generated by indexer in the staking pool.
    /// @param stake Stake attributed to the indexer in the staking pool.
    /// @param alphaNumerator Numerator of `alpha` in the rebates function.
    /// @param alphaDenominator Denominator of `alpha` in the rebates function.
    /// @param lambdaNumerator Numerator of `lambda` in the rebates function.
    /// @param lambdaDenominator Denominator of `lambda` in the rebates function.
    /// @return rewards Rewards owed to the staking pool.
    function exponentialRebates(
        uint256 fees,
        uint256 stake,
        uint32 alphaNumerator,
        uint32 alphaDenominator,
        uint32 lambdaNumerator,
        uint32 lambdaDenominator
    ) public pure returns (uint256) {
        // If alpha is zero indexer gets 100% fees rebate
        int256 alpha = LibFixedMath.toFixed(int32(alphaNumerator), int32(alphaDenominator));
        if (alpha == 0) {
            return fees;
        }

        // No rebates if no fees...
        if (fees == 0) {
            return 0;
        }

        // Award all fees as rebate if the exponent is too large
        int256 lambda = LibFixedMath.toFixed(int32(lambdaNumerator), int32(lambdaDenominator));
        int256 exponent = LibFixedMath.mulDiv(lambda, int256(stake), int256(fees));
        if (LibFixedMath.toInteger(exponent) > MAX_EXPONENT) {
            return fees;
        }

        // Compute `1 - alpha * exp ^(-exponent)`
        int256 factor = LibFixedMath.sub(LibFixedMath.one(), LibFixedMath.mul(alpha, LibFixedMath.exp(-exponent)));

        // Weight the fees by the factor
        return LibFixedMath.uintMul(factor, fees);
    }
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

pragma solidity ^0.7.6;
pragma abicoder v2;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable gas-strict-inequalities

import { SafeMath } from "@openzeppelin/contracts/math/SafeMath.sol";
import { StakingV4Storage } from "./StakingStorage.sol";
import { IStakingExtension } from "@graphprotocol/interfaces/contracts/contracts/staking/IStakingExtension.sol";
import { TokenUtils } from "../utils/TokenUtils.sol";
import { IGraphToken } from "@graphprotocol/interfaces/contracts/contracts/token/IGraphToken.sol";
import { GraphUpgradeable } from "../upgrades/GraphUpgradeable.sol";
import { IStakes } from "@graphprotocol/interfaces/contracts/contracts/staking/libs/IStakes.sol";
import { Stakes } from "./libs/Stakes.sol";
import { IStakingData } from "@graphprotocol/interfaces/contracts/contracts/staking/IStakingData.sol";
import { MathUtils } from "./libs/MathUtils.sol";

/**
 * @title StakingExtension contract
 * @author Edge & Node
 * @notice This contract provides the logic to manage delegations and other Staking
 * extension features (e.g. storage getters). It is meant to be called through delegatecall from the
 * Staking contract, and is only kept separate to keep the Staking contract size
 * within limits.
 */
contract StakingExtension is StakingV4Storage, GraphUpgradeable, IStakingExtension {
    using SafeMath for uint256;
    using Stakes for IStakes.Indexer;

    /// @dev 100% in parts per million
    uint32 private constant MAX_PPM = 1000000;
    /// @dev Minimum amount of tokens that can be delegated
    uint256 private constant MINIMUM_DELEGATION = 1e18;

    /**
     * @dev Check if the caller is the slasher.
     */
    modifier onlySlasher() {
        require(__slashers[msg.sender] == true, "!slasher");
        _;
    }

    /**
     * @notice Initialize the StakingExtension contract
     * @dev This function is meant to be delegatecalled from the Staking contract's
     * initialize() function, so it uses the same access control check to ensure it is
     * being called by the Staking implementation as part of the proxy upgrade process.
     * @param _delegationUnbondingPeriod Delegation unbonding period in blocks
     * @param _cooldownBlocks Deprecated parameter (no longer used)
     * @param _delegationRatio Delegation capacity multiplier (e.g. 10 means 10x the indexer stake)
     * @param _delegationTaxPercentage Percentage of delegated tokens to burn as delegation tax, expressed in parts per million
     */
    function initialize(
        uint32 _delegationUnbondingPeriod,
        // solhint-disable-next-line no-unused-vars
        uint32 _cooldownBlocks, // deprecated
        uint32 _delegationRatio,
        uint32 _delegationTaxPercentage
    ) external onlyImpl {
        _setDelegationUnbondingPeriod(_delegationUnbondingPeriod);
        _setDelegationRatio(_delegationRatio);
        _setDelegationTaxPercentage(_delegationTaxPercentage);
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function setDelegationTaxPercentage(uint32 _percentage) external override onlyGovernor {
        _setDelegationTaxPercentage(_percentage);
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function setDelegationRatio(uint32 _delegationRatio) external override onlyGovernor {
        _setDelegationRatio(_delegationRatio);
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function setDelegationUnbondingPeriod(uint32 _delegationUnbondingPeriod) external override onlyGovernor {
        _setDelegationUnbondingPeriod(_delegationUnbondingPeriod);
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function setSlasher(address _slasher, bool _allowed) external override onlyGovernor {
        require(_slasher != address(0), "!slasher");
        __slashers[_slasher] = _allowed;
        emit SlasherUpdate(msg.sender, _slasher, _allowed);
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function delegate(address _indexer, uint256 _tokens) external override notPartialPaused returns (uint256) {
        address delegator = msg.sender;

        // Transfer tokens to delegate to this contract
        TokenUtils.pullTokens(graphToken(), delegator, _tokens);

        // Update state
        return _delegate(delegator, _indexer, _tokens);
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function undelegate(address _indexer, uint256 _shares) external override notPartialPaused returns (uint256) {
        return _undelegate(msg.sender, _indexer, _shares);
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function withdrawDelegated(address _indexer, address _newIndexer) external override notPaused returns (uint256) {
        return _withdrawDelegated(msg.sender, _indexer, _newIndexer);
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function slash(
        address _indexer,
        uint256 _tokens,
        uint256 _reward,
        address _beneficiary
    ) external override onlySlasher notPartialPaused {
        IStakes.Indexer storage indexerStake = __stakes[_indexer];

        // Only able to slash a non-zero number of tokens
        require(_tokens > 0, "!tokens");

        // Rewards comes from tokens slashed balance
        require(_tokens >= _reward, "rewards>slash");

        // Cannot slash stake of an indexer without any or enough stake
        require(indexerStake.tokensStaked > 0, "!stake");
        require(_tokens <= indexerStake.tokensStaked, "slash>stake");

        // Validate beneficiary of slashed tokens
        require(_beneficiary != address(0), "!beneficiary");

        // Slashing more tokens than freely available (over allocation condition)
        // Unlock locked tokens to avoid the indexer to withdraw them
        if (_tokens > indexerStake.tokensAvailable() && indexerStake.tokensLocked > 0) {
            uint256 tokensOverAllocated = _tokens.sub(indexerStake.tokensAvailable());
            uint256 tokensToUnlock = MathUtils.min(tokensOverAllocated, indexerStake.tokensLocked);
            indexerStake.unlockTokens(tokensToUnlock);
        }

        // Remove tokens to slash from the stake
        indexerStake.release(_tokens);

        // -- Interactions --

        IGraphToken graphToken = graphToken();

        // Set apart the reward for the beneficiary and burn remaining slashed stake
        TokenUtils.burnTokens(graphToken, _tokens.sub(_reward));

        // Give the beneficiary a reward for slashing
        TokenUtils.pushTokens(graphToken, _beneficiary, _reward);

        emit StakeSlashed(_indexer, _tokens, _reward, _beneficiary);
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function getDelegation(address _indexer, address _delegator) external view override returns (Delegation memory) {
        return __delegationPools[_indexer].delegators[_delegator];
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function delegationRatio() external view override returns (uint32) {
        return __delegationRatio;
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function delegationUnbondingPeriod() external view override returns (uint32) {
        return __delegationUnbondingPeriod;
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function delegationTaxPercentage() external view override returns (uint32) {
        return __delegationTaxPercentage;
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function delegationPools(address _indexer) external view override returns (DelegationPoolReturn memory) {
        DelegationPool storage pool = __delegationPools[_indexer];
        return
            DelegationPoolReturn(
                0, // Blocks to wait before updating parameters (deprecated)
                pool.indexingRewardCut, // in PPM
                pool.queryFeeCut, // in PPM
                pool.updatedAtBlock, // Block when the pool was last updated
                pool.tokens, // Total tokens as pool reserves
                pool.shares // Total shares minted in the pool
            );
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function rewardsDestination(address _indexer) external view override returns (address) {
        return __rewardsDestination[_indexer];
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function operatorAuth(address _indexer, address _maybeOperator) external view override returns (bool) {
        return __operatorAuth[_indexer][_maybeOperator];
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function subgraphAllocations(bytes32 _subgraphDeploymentId) external view override returns (uint256) {
        return __subgraphAllocations[_subgraphDeploymentId];
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function slashers(address _maybeSlasher) external view override returns (bool) {
        return __slashers[_maybeSlasher];
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function minimumIndexerStake() external view override returns (uint256) {
        return __minimumIndexerStake;
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function thawingPeriod() external view override returns (uint32) {
        return __thawingPeriod;
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function curationPercentage() external view override returns (uint32) {
        return __curationPercentage;
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function protocolPercentage() external view override returns (uint32) {
        return __protocolPercentage;
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function maxAllocationEpochs() external view override returns (uint32) {
        return __maxAllocationEpochs;
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function alphaNumerator() external view override returns (uint32) {
        return __alphaNumerator;
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function alphaDenominator() external view override returns (uint32) {
        return __alphaDenominator;
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function lambdaNumerator() external view override returns (uint32) {
        return __lambdaNumerator;
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function lambdaDenominator() external view override returns (uint32) {
        return __lambdaDenominator;
    }

    /**
     * @notice Getter for stakes[_indexer]:
     * gets the stake information for an indexer as an IStakes.Indexer struct.
     * @param _indexer Indexer address for which to query the stake information
     * @return Stake information for the specified indexer, as an IStakes.Indexer struct
     * @inheritdoc IStakingExtension
     */
    function stakes(address _indexer) external view override returns (IStakes.Indexer memory) {
        return __stakes[_indexer];
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function allocations(address _allocationID) external view override returns (IStakingData.Allocation memory) {
        return __allocations[_allocationID];
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function isDelegator(address _indexer, address _delegator) public view override returns (bool) {
        return __delegationPools[_indexer].delegators[_delegator].shares > 0;
    }

    /**
     * @inheritdoc IStakingExtension
     */
    function getWithdraweableDelegatedTokens(Delegation memory _delegation) public view override returns (uint256) {
        // There must be locked tokens and period passed
        uint256 currentEpoch = epochManager().currentEpoch();
        if (_delegation.tokensLockedUntil > 0 && currentEpoch >= _delegation.tokensLockedUntil) {
            return _delegation.tokensLocked;
        }
        return 0;
    }

    /**
     * @notice Internal: Set a delegation tax percentage to burn when delegated funds are deposited.
     * @param _percentage Percentage of delegated tokens to burn as delegation tax
     */
    function _setDelegationTaxPercentage(uint32 _percentage) private {
        // Must be within 0% to 100% (inclusive)
        require(_percentage <= MAX_PPM, ">percentage");
        __delegationTaxPercentage = _percentage;
        emit ParameterUpdated("delegationTaxPercentage");
    }

    /**
     * @notice Internal: Set the delegation ratio.
     * If set to 10 it means the indexer can use up to 10x the indexer staked amount
     * from their delegated tokens
     * @param _delegationRatio Delegation capacity multiplier
     */
    function _setDelegationRatio(uint32 _delegationRatio) private {
        __delegationRatio = _delegationRatio;
        emit ParameterUpdated("delegationRatio");
    }

    /**
     * @notice Internal: Set the period for undelegation of stake from indexer.
     * @param _delegationUnbondingPeriod Period in epochs to wait for token withdrawals after undelegating
     */
    function _setDelegationUnbondingPeriod(uint32 _delegationUnbondingPeriod) private {
        require(_delegationUnbondingPeriod > 0, "!delegationUnbondingPeriod");
        __delegationUnbondingPeriod = _delegationUnbondingPeriod;
        emit ParameterUpdated("delegationUnbondingPeriod");
    }

    /**
     * @notice Delegate tokens to an indexer.
     * @param _delegator Address of the delegator
     * @param _indexer Address of the indexer to delegate tokens to
     * @param _tokens Amount of tokens to delegate
     * @return Amount of shares issued of the delegation pool
     */
    function _delegate(address _delegator, address _indexer, uint256 _tokens) private returns (uint256) {
        // Only allow delegations over a minimum, to prevent rounding attacks
        require(_tokens >= MINIMUM_DELEGATION, "!minimum-delegation");
        // Only delegate to non-empty address
        require(_indexer != address(0), "!indexer");
        // Only delegate to staked indexer
        require(__stakes[_indexer].tokensStaked > 0, "!stake");

        // Get the delegation pool of the indexer
        DelegationPool storage pool = __delegationPools[_indexer];
        Delegation storage delegation = pool.delegators[_delegator];

        // Collect delegation tax
        uint256 delegationTax = _collectTax(graphToken(), _tokens, __delegationTaxPercentage);
        uint256 delegatedTokens = _tokens.sub(delegationTax);

        // Calculate shares to issue
        uint256 shares = (pool.tokens == 0) ? delegatedTokens : delegatedTokens.mul(pool.shares).div(pool.tokens);
        require(shares > 0, "!shares");

        // Update the delegation pool
        pool.tokens = pool.tokens.add(delegatedTokens);
        pool.shares = pool.shares.add(shares);

        // Update the individual delegation
        delegation.shares = delegation.shares.add(shares);

        emit StakeDelegated(_indexer, _delegator, delegatedTokens, shares);

        return shares;
    }

    /**
     * @notice Undelegate tokens from an indexer.
     * @param _delegator Address of the delegator
     * @param _indexer Address of the indexer where tokens had been delegated
     * @param _shares Amount of shares to return and undelegate tokens
     * @return Amount of tokens returned for the shares of the delegation pool
     */
    function _undelegate(address _delegator, address _indexer, uint256 _shares) private returns (uint256) {
        // Can only undelegate a non-zero amount of shares
        require(_shares > 0, "!shares");

        // Get the delegation pool of the indexer
        DelegationPool storage pool = __delegationPools[_indexer];
        Delegation storage delegation = pool.delegators[_delegator];

        // Delegator need to have enough shares in the pool to undelegate
        require(delegation.shares >= _shares, "!shares-avail");

        // Withdraw tokens if available
        if (getWithdraweableDelegatedTokens(delegation) > 0) {
            _withdrawDelegated(_delegator, _indexer, address(0));
        }

        uint256 poolTokens = pool.tokens;
        uint256 poolShares = pool.shares;

        // Calculate tokens to get in exchange for the shares
        uint256 tokens = _shares.mul(poolTokens).div(poolShares);

        // Update the delegation pool
        poolTokens = poolTokens.sub(tokens);
        poolShares = poolShares.sub(_shares);
        pool.tokens = poolTokens;
        pool.shares = poolShares;

        // Update the delegation
        delegation.shares = delegation.shares.sub(_shares);
        // Enforce more than the minimum delegation is left,
        // to prevent rounding attacks
        if (delegation.shares > 0) {
            uint256 remainingDelegation = delegation.shares.mul(poolTokens).div(poolShares);
            require(remainingDelegation >= MINIMUM_DELEGATION, "!minimum-delegation");
        }
        delegation.tokensLocked = delegation.tokensLocked.add(tokens);
        delegation.tokensLockedUntil = epochManager().currentEpoch().add(__delegationUnbondingPeriod);

        emit StakeDelegatedLocked(_indexer, _delegator, tokens, _shares, delegation.tokensLockedUntil);

        return tokens;
    }

    /**
     * @notice Withdraw delegated tokens once the unbonding period has passed.
     * @param _delegator Delegator that is withdrawing tokens
     * @param _indexer Withdraw available tokens delegated to indexer
     * @param _delegateToIndexer Re-delegate to indexer address if non-zero, withdraw if zero address
     * @return Amount of tokens withdrawn or re-delegated
     */
    function _withdrawDelegated(
        address _delegator,
        address _indexer,
        address _delegateToIndexer
    ) private returns (uint256) {
        // Get the delegation pool of the indexer
        DelegationPool storage pool = __delegationPools[_indexer];
        Delegation storage delegation = pool.delegators[_delegator];

        // Validation
        uint256 tokensToWithdraw = getWithdraweableDelegatedTokens(delegation);
        require(tokensToWithdraw > 0, "!tokens");

        // Reset lock
        delegation.tokensLocked = 0;
        delegation.tokensLockedUntil = 0;

        emit StakeDelegatedWithdrawn(_indexer, _delegator, tokensToWithdraw);

        // -- Interactions --

        if (_delegateToIndexer != address(0)) {
            // Re-delegate tokens to a new indexer
            _delegate(_delegator, _delegateToIndexer, tokensToWithdraw);
        } else {
            // Return tokens to the delegator
            TokenUtils.pushTokens(graphToken(), _delegator, tokensToWithdraw);
        }

        return tokensToWithdraw;
    }

    /**
     * @notice Collect tax to burn for an amount of tokens.
     * @param _graphToken Token to burn
     * @param _tokens Total tokens received used to calculate the amount of tax to collect
     * @param _percentage Percentage of tokens to burn as tax
     * @return Amount of tax charged
     */
    function _collectTax(IGraphToken _graphToken, uint256 _tokens, uint256 _percentage) private returns (uint256) {
        uint256 tax = uint256(_percentage).mul(_tokens).div(MAX_PPM);
        TokenUtils.burnTokens(_graphToken, tax); // Burn tax if any
        return tax;
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


## ------------ SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS ------------ 

## ------------ SUPPORTING CONTEXT: EXTERNAL LIBRARIES ------------ 

 ------------ END OF SUPPORTING CONTRACTS AND INTERFACES ------------ 


 ------------ ## DEPLOYMENT SCRIPTS ------------ 

