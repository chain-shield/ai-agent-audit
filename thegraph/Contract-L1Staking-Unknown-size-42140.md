
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
// SPDX-License-Identifier: Apache-2.0

/*
 * Copyright 2020, Offchain Labs, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *    http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * Originally copied from:
 * https://github.com/OffchainLabs/arbitrum/tree/e3a6307ad8a2dc2cad35728a2a9908cfd8dd8ef9/packages/arb-bridge-peripherals
 *
 * MODIFIED from Offchain Labs' implementation:
 * - Changed solidity version to 0.7.6 (pablo@edgeandnode.com)
 *
 */

pragma solidity ^0.7.3 || ^0.8.0;

/**
 * @title Token Gateway Interface
 * @author Edge & Node
 * @notice Interface for token gateways that handle cross-chain token transfers
 */
interface ITokenGateway {
    /// @notice event deprecated in favor of DepositInitiated and WithdrawalInitiated
    // event OutboundTransferInitiated(
    //     address token,
    //     address indexed _from,
    //     address indexed _to,
    //     uint256 indexed _transferId,
    //     uint256 _amount,
    //     bytes _data
    // );

    /// @notice event deprecated in favor of DepositFinalized and WithdrawalFinalized
    // event InboundTransferFinalized(
    //     address token,
    //     address indexed _from,
    //     address indexed _to,
    //     uint256 indexed _transferId,
    //     uint256 _amount,
    //     bytes _data
    // );

    /**
     * @notice Transfer tokens from L1 to L2 or L2 to L1
     * @param token Address of the token being transferred
     * @param to Recipient address on the destination chain
     * @param amount Amount of tokens to transfer
     * @param maxGas Maximum gas for the transaction
     * @param gasPriceBid Gas price bid for the transaction
     * @param data Additional data for the transfer
     * @return Transaction data
     */
    function outboundTransfer(
        address token,
        address to,
        uint256 amount,
        uint256 maxGas,
        uint256 gasPriceBid,
        bytes calldata data
    ) external payable returns (bytes memory);

    /**
     * @notice Finalize an inbound token transfer
     * @param token Address of the token being transferred
     * @param from Sender address on the source chain
     * @param to Recipient address on the destination chain
     * @param amount Amount of tokens being transferred
     * @param data Additional data for the transfer
     */
    function finalizeInboundTransfer(
        address token,
        address from,
        address to,
        uint256 amount,
        bytes calldata data
    ) external payable;

    /**
     * @notice Calculate the address used when bridging an ERC20 token
     * @dev the L1 and L2 address oracles may not always be in sync.
     * For example, a custom token may have been registered but not deployed or the contract self destructed.
     * @param l1ERC20 address of L1 token
     * @return L2 address of a bridged ERC20 token
     */
    function calculateL2TokenAddress(address l1ERC20) external view returns (address);
}

// SPDX-License-Identifier: MIT

pragma solidity ^0.7.3;
pragma experimental ABIEncoderV2;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable function-max-lines, gas-indexed-events, gas-strict-inequalities, use-natspec
// solhint-disable named-parameters-mapping

import { AddressUpgradeable } from "@openzeppelin/contracts-upgradeable/utils/AddressUpgradeable.sol";
import { ITokenGateway } from "@graphprotocol/interfaces/contracts/contracts/arbitrum/ITokenGateway.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { L2GraphTokenLockManager } from "./L2GraphTokenLockManager.sol";
import { GraphTokenLockWallet } from "./GraphTokenLockWallet.sol";
import { MinimalProxyFactory } from "./MinimalProxyFactory.sol";
import { IGraphTokenLock } from "./IGraphTokenLock.sol";
import { Ownable as OwnableInitializable } from "./Ownable.sol";
import { SafeMathUpgradeable } from "@openzeppelin/contracts-upgradeable/math/SafeMathUpgradeable.sol";
import { Initializable } from "@openzeppelin/contracts-upgradeable/proxy/Initializable.sol";

/**
 * @title L1GraphTokenLockTransferTool contract
 * @notice This contract is used to transfer GRT from GraphTokenLockWallets
 * to a counterpart on L2. It is deployed on L1 and will send the GRT through
 * the L1GraphTokenGateway with a callhook to the L2GraphTokenLockManager, including
 * data to create a L2GraphTokenLockWallet on L2.
 *
 * Note that the L2GraphTokenLockWallet will not allow releasing any GRT until the end of
 * the vesting timeline, but will allow sending the GRT back to the L1 wallet.
 *
 * Beneficiaries for a GraphTokenLockWallet can perform the depositToL2Locked call
 * as many times as they want, and the GRT will be sent to the same L2GraphTokenLockWallet.
 *
 * Since all retryable tickets to send transactions to L2 require ETH for gas, this
 * contract also allows users to deposit ETH to be used for gas on L2, both for
 * the depositToL2Locked calls and for the transfer tools in the Staking contract for
 * The Graph.
 *
 * See GIP-0046 for more details: https://forum.thegraph.com/t/4023
 */
contract L1GraphTokenLockTransferTool is OwnableInitializable, Initializable, MinimalProxyFactory {
    using SafeMathUpgradeable for uint256;

    /// Address of the L1 GRT token contract
    // solhint-disable-next-line immutable-vars-naming
    IERC20 public immutable graphToken;
    /// Address of the L2GraphTokenLockWallet implementation in L2, used to compute L2 wallet addresses
    // solhint-disable-next-line immutable-vars-naming
    address public immutable l2Implementation;
    /// Address of the L1GraphTokenGateway contract
    // solhint-disable-next-line immutable-vars-naming
    ITokenGateway public immutable l1Gateway;
    /// Address of the Staking contract, used to pull ETH for L2 ticket gas
    // solhint-disable-next-line immutable-vars-naming
    address payable public immutable staking;
    /// L2 lock manager for each L1 lock manager.
    /// L1 GraphTokenLockManager => L2GraphTokenLockManager
    mapping(address => address) public l2LockManager;
    /// L2 wallet owner for each L1 wallet owner.
    /// L1 wallet owner => L2 wallet owner
    mapping(address => address) public l2WalletOwner;
    /// L2 wallet address for each L1 wallet address.
    /// L1 wallet => L2 wallet
    mapping(address => address) public l2WalletAddress;
    /// ETH balance from each token lock, used to pay for L2 gas:
    /// L1 wallet address => ETH balance
    mapping(address => uint256) public tokenLockETHBalances;
    /// L2 beneficiary corresponding to each L1 wallet address.
    /// L1 wallet => L2 beneficiary
    mapping(address => address) public l2Beneficiary;
    /// Indicates whether an L2 wallet address for a wallet
    /// has been set manually, in which case it can't call depositToL2Locked.
    /// L1 wallet => bool
    mapping(address => bool) public l2WalletAddressSetManually;

    /// @dev Emitted when the L2 lock manager for an L1 lock manager is set
    event L2LockManagerSet(address indexed l1LockManager, address indexed l2LockManager);
    /// @dev Emitted when the L2 wallet owner for an L1 wallet owner is set
    event L2WalletOwnerSet(address indexed l1WalletOwner, address indexed l2WalletOwner);
    /// @dev Emitted when GRT is sent to L2 from a token lock
    event LockedFundsSentToL2(
        address indexed l1Wallet,
        address indexed l2Wallet,
        address indexed l1LockManager,
        address l2LockManager,
        uint256 amount
    );
    /// @dev Emitted when an L2 wallet address is set for an L1 wallet
    event L2WalletAddressSet(address indexed l1Wallet, address indexed l2Wallet);
    /// @dev Emitted when ETH is deposited to a token lock's account
    event ETHDeposited(address indexed tokenLock, uint256 amount);
    /// @dev Emitted when ETH is withdrawn from a token lock's account
    event ETHWithdrawn(address indexed tokenLock, address indexed destination, uint256 amount);
    /// @dev Emitted when ETH is pulled from a token lock's account by Staking or this tool to pay for an L2 ticket
    event ETHPulled(address indexed tokenLock, uint256 amount);
    /// @dev Emitted when the L2 beneficiary for a partially vested L1 lock is set
    event L2BeneficiarySet(address indexed l1Wallet, address indexed l2Beneficiary);

    /**
     * @notice Construct a new L1GraphTokenLockTransferTool contract
     * @dev The deployer of the contract will become its owner.
     * Note this contract is meant to be deployed behind a transparent proxy,
     * so this will run at the implementation's storage context; it will set
     * immutable variables and make the implementation be owned by the deployer.
     * @param _graphToken Address of the L1 GRT token contract
     * @param _l2Implementation Address of the L2GraphTokenLockWallet implementation in L2
     * @param _l1Gateway Address of the L1GraphTokenGateway contract
     * @param _staking Address of the Staking contract
     */
    constructor(
        IERC20 _graphToken,
        address _l2Implementation,
        ITokenGateway _l1Gateway,
        address payable _staking
    ) initializer {
        OwnableInitializable._initialize(msg.sender);
        graphToken = _graphToken;
        l2Implementation = _l2Implementation;
        l1Gateway = _l1Gateway;
        staking = _staking;
    }

    /**
     * @notice Initialize the L1GraphTokenLockTransferTool contract
     * @dev This function will run in the proxy's storage context, so it will
     * set the owner of the proxy contract which can be different from the implementation owner.
     * @param _owner Address of the owner of the L1GraphTokenLockTransferTool contract
     */
    function initialize(address _owner) external initializer {
        OwnableInitializable._initialize(_owner);
    }

    /**
     * @notice Set the L2 lock manager that corresponds to an L1 lock manager
     * @param _l1LockManager Address of the L1 lock manager
     * @param _l2LockManager Address of the L2 lock manager (in L2)
     */
    function setL2LockManager(address _l1LockManager, address _l2LockManager) external onlyOwner {
        l2LockManager[_l1LockManager] = _l2LockManager;
        emit L2LockManagerSet(_l1LockManager, _l2LockManager);
    }

    /**
     * @notice Set the L2 wallet owner that corresponds to an L1 wallet owner
     * @param _l1WalletOwner Address of the L1 wallet owner
     * @param _l2WalletOwner Address of the L2 wallet owner (in L2)
     */
    function setL2WalletOwner(address _l1WalletOwner, address _l2WalletOwner) external onlyOwner {
        l2WalletOwner[_l1WalletOwner] = _l2WalletOwner;
        emit L2WalletOwnerSet(_l1WalletOwner, _l2WalletOwner);
    }

    /**
     * @notice Deposit ETH on a token lock's account, to be used for L2 retryable ticket gas.
     * This function can be called by anyone, but the ETH will be credited to the token lock.
     * DO NOT try to call this through the token lock, as locks do not forward ETH value (and the
     * function call should not be allowlisted).
     * @param _tokenLock Address of the L1 GraphTokenLockWallet that will own the ETH
     */
    function depositETH(address _tokenLock) external payable {
        tokenLockETHBalances[_tokenLock] = tokenLockETHBalances[_tokenLock].add(msg.value);
        emit ETHDeposited(_tokenLock, msg.value);
    }

    /**
     * @notice Withdraw ETH from a token lock's account.
     * This function must be called from the token lock contract, but the destination
     * _must_ be a different address, as any ETH sent to the token lock would otherwise be
     * lost.
     * @param _destination Address to send the ETH
     * @param _amount Amount of ETH to send
     */
    function withdrawETH(address _destination, uint256 _amount) external {
        require(_amount > 0, "INVALID_AMOUNT");
        // We can't send eth to a token lock or it will be stuck
        require(msg.sender != _destination, "INVALID_DESTINATION");
        require(tokenLockETHBalances[msg.sender] >= _amount, "INSUFFICIENT_BALANCE");
        tokenLockETHBalances[msg.sender] -= _amount;
        // solhint-disable-next-line avoid-low-level-calls
        (bool success, ) = payable(_destination).call{ value: _amount }("");
        require(success, "TRANSFER_FAILED");
        emit ETHWithdrawn(msg.sender, _destination, _amount);
    }

    /**
     * @notice Pull ETH from a token lock's account, to be used for L2 retryable ticket gas.
     * This can only be called by the Staking contract.
     * @param _tokenLock GraphTokenLockWallet that owns the ETH that will be debited
     * @param _amount Amount of ETH to pull
     */
    function pullETH(address _tokenLock, uint256 _amount) external {
        require(msg.sender == staking, "ONLY_STAKING");
        require(tokenLockETHBalances[_tokenLock] >= _amount, "INSUFFICIENT_BALANCE");
        tokenLockETHBalances[_tokenLock] -= _amount;
        // solhint-disable-next-line avoid-low-level-calls
        (bool success, ) = staking.call{ value: _amount }("");
        require(success, "TRANSFER_FAILED");
        emit ETHPulled(_tokenLock, _amount);
    }

    /**
     * @notice Deposit GRT to L2, from a token lock in L1 to a token lock in L2.
     * If the token lock in L2 does not exist, it will be created when the message is received
     * by the L2GraphTokenLockManager.
     * Before calling this (which must be done through the token lock wallet), make sure
     * there is enough ETH in the token lock's account to cover the L2 retryable ticket gas.
     * Note that L2 submission fee and gas refunds will be lost.
     * You can add ETH to the token lock's account by calling depositETH().
     * Note that after calling this, you will NOT be able to use setL2WalletAddressManually() to
     * set an L2 wallet address, as the L2 wallet address will be set automatically when the
     * message is received by the L2GraphTokenLockManager.
     * @dev The gas parameters for L2 can be estimated using the Arbitrum SDK.
     * @param _amount Amount of GRT to deposit
     * @param _l2Beneficiary Address of the beneficiary for the token lock in L2. Must be the same for subsequent calls of this function, and not an L1 contract.
     * @param _maxGas Maximum gas to use for the L2 retryable ticket
     * @param _gasPriceBid Gas price to use for the L2 retryable ticket
     * @param _maxSubmissionCost Max submission cost for the L2 retryable ticket
     */
    function depositToL2Locked(
        uint256 _amount,
        address _l2Beneficiary,
        uint256 _maxGas,
        uint256 _gasPriceBid,
        uint256 _maxSubmissionCost
    ) external {
        // Check that msg.sender is a GraphTokenLockWallet
        // That uses GRT and has a corresponding manager set in L2.
        GraphTokenLockWallet wallet = GraphTokenLockWallet(msg.sender);
        require(wallet.token() == graphToken, "INVALID_TOKEN");
        address l1Manager = address(wallet.manager());
        address l2Manager = l2LockManager[l1Manager];
        require(l2Manager != address(0), "INVALID_MANAGER");
        require(wallet.isInitialized(), "!INITIALIZED");
        require(wallet.revocable() != IGraphTokenLock.Revocability.Enabled, "REVOCABLE");
        require(_amount <= graphToken.balanceOf(msg.sender), "INSUFFICIENT_BALANCE");
        require(_amount != 0, "ZERO_AMOUNT");

        if (l2Beneficiary[msg.sender] == address(0)) {
            require(_l2Beneficiary != address(0), "INVALID_BENEFICIARY_ZERO");
            require(!AddressUpgradeable.isContract(_l2Beneficiary), "INVALID_BENEFICIARY_CONTRACT");
            l2Beneficiary[msg.sender] = _l2Beneficiary;
            emit L2BeneficiarySet(msg.sender, _l2Beneficiary);
        } else {
            require(l2Beneficiary[msg.sender] == _l2Beneficiary, "INVALID_BENEFICIARY");
        }

        uint256 expectedEth = _maxSubmissionCost.add(_maxGas.mul(_gasPriceBid));
        require(tokenLockETHBalances[msg.sender] >= expectedEth, "INSUFFICIENT_ETH_BALANCE");
        tokenLockETHBalances[msg.sender] -= expectedEth;

        bytes memory encodedData;
        {
            address l2Owner = l2WalletOwner[wallet.owner()];
            require(l2Owner != address(0), "L2_OWNER_NOT_SET");
            // Extract all the storage variables from the GraphTokenLockWallet
            L2GraphTokenLockManager.TransferredWalletData memory data = L2GraphTokenLockManager.TransferredWalletData({
                l1Address: msg.sender,
                owner: l2Owner,
                beneficiary: l2Beneficiary[msg.sender],
                managedAmount: wallet.managedAmount(),
                startTime: wallet.startTime(),
                endTime: wallet.endTime()
            });
            encodedData = abi.encode(data);
        }

        if (l2WalletAddress[msg.sender] == address(0)) {
            require(wallet.endTime() >= block.timestamp, "FULLY_VESTED_USE_MANUAL_ADDRESS");
            address newAddress = getDeploymentAddress(keccak256(encodedData), l2Implementation, l2Manager);
            l2WalletAddress[msg.sender] = newAddress;
            emit L2WalletAddressSet(msg.sender, newAddress);
        } else {
            require(!l2WalletAddressSetManually[msg.sender], "CANT_DEPOSIT_TO_MANUAL_ADDRESS");
        }

        graphToken.transferFrom(msg.sender, address(this), _amount);

        // Send the tokens with a message through the L1GraphTokenGateway to the L2GraphTokenLockManager
        graphToken.approve(address(l1Gateway), _amount);
        {
            bytes memory transferData = abi.encode(_maxSubmissionCost, encodedData);
            l1Gateway.outboundTransfer{ value: expectedEth }(
                address(graphToken),
                l2Manager,
                _amount,
                _maxGas,
                _gasPriceBid,
                transferData
            );
        }
        emit ETHPulled(msg.sender, expectedEth);
        emit LockedFundsSentToL2(msg.sender, l2WalletAddress[msg.sender], l1Manager, l2Manager, _amount);
    }

    /**
     * @notice Manually set the L2 wallet address for a token lock in L1.
     * This will only work for token locks that have not been initialized in L2 yet, and
     * that are fully vested (endTime < current timestamp).
     * This address can then be used to send stake or delegation to L2 on the Staking contract.
     * After calling this, the vesting lock will NOT be allowed to use depositToL2Locked
     * to send GRT to L2, the beneficiary must withdraw the tokens and bridge them manually.
     * @param _l2Wallet Address of the L2 wallet
     */
    function setL2WalletAddressManually(address _l2Wallet) external {
        // Check that msg.sender is a GraphTokenLockWallet
        // That uses GRT and has a corresponding manager set in L2.
        GraphTokenLockWallet wallet = GraphTokenLockWallet(msg.sender);
        require(wallet.token() == graphToken, "INVALID_TOKEN");
        address l1Manager = address(wallet.manager());
        address l2Manager = l2LockManager[l1Manager];
        require(l2Manager != address(0), "INVALID_MANAGER");
        require(wallet.isInitialized(), "!INITIALIZED");

        // Check that the wallet is fully vested
        require(wallet.endTime() < block.timestamp, "NOT_FULLY_VESTED");

        // Check that the wallet has not set an L2 wallet yet
        require(l2WalletAddress[msg.sender] == address(0), "L2_WALLET_ALREADY_SET");

        // Check that the L2 address is not zero
        require(_l2Wallet != address(0), "ZERO_ADDRESS");
        // Set the L2 wallet address
        l2WalletAddress[msg.sender] = _l2Wallet;
        l2WalletAddressSetManually[msg.sender] = true;
        emit L2WalletAddressSet(msg.sender, _l2Wallet);
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity ^0.7.6;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable gas-indexed-events, gas-small-strings, gas-strict-inequalities

import { SafeMath } from "@openzeppelin/contracts/math/SafeMath.sol";

import { GraphUpgradeable } from "../upgrades/GraphUpgradeable.sol";
import { Managed } from "../governance/Managed.sol";

import { EpochManagerV1Storage } from "./EpochManagerStorage.sol";
import { IEpochManager } from "@graphprotocol/interfaces/contracts/contracts/epochs/IEpochManager.sol";

/**
 * @title EpochManager contract
 * @author Edge & Node
 * @notice Produce epochs based on a number of blocks to coordinate contracts in the protocol.
 */
contract EpochManager is EpochManagerV1Storage, GraphUpgradeable, IEpochManager {
    using SafeMath for uint256;

    // -- Events --

    /**
     * @notice Emitted when an epoch is run
     * @param epoch The epoch number that was run
     * @param caller Address that called runEpoch()
     */
    event EpochRun(uint256 indexed epoch, address caller);

    /**
     * @notice Emitted when the epoch length is updated
     * @param epoch The epoch when the length was updated
     * @param epochLength The new epoch length in blocks
     */
    event EpochLengthUpdate(uint256 indexed epoch, uint256 epochLength);

    /**
     * @notice Initialize this contract.
     * @param _controller Address of the Controller contract
     * @param _epochLength Length of each epoch in blocks
     */
    function initialize(address _controller, uint256 _epochLength) external onlyImpl {
        require(_epochLength > 0, "Epoch length cannot be 0");

        Managed._initialize(_controller);

        // NOTE: We make the first epoch to be one instead of zero to avoid any issue
        // with composing contracts that may use zero as an empty value
        lastLengthUpdateEpoch = 1;
        lastLengthUpdateBlock = blockNum();
        epochLength = _epochLength;

        emit EpochLengthUpdate(lastLengthUpdateEpoch, epochLength);
    }

    /**
     * @inheritdoc IEpochManager
     */
    function setEpochLength(uint256 _epochLength) external override onlyGovernor {
        require(_epochLength > 0, "Epoch length cannot be 0");
        require(_epochLength != epochLength, "Epoch length must be different to current");

        lastLengthUpdateEpoch = currentEpoch();
        lastLengthUpdateBlock = currentEpochBlock();
        epochLength = _epochLength;

        emit EpochLengthUpdate(lastLengthUpdateEpoch, epochLength);
    }

    /**
     * @inheritdoc IEpochManager
     */
    function runEpoch() external override {
        // Check if already called for the current epoch
        require(!isCurrentEpochRun(), "Current epoch already run");

        lastRunEpoch = currentEpoch();

        // Hook for protocol general state updates

        emit EpochRun(lastRunEpoch, msg.sender);
    }

    /**
     * @inheritdoc IEpochManager
     */
    function isCurrentEpochRun() public view override returns (bool) {
        return lastRunEpoch == currentEpoch();
    }

    /**
     * @inheritdoc IEpochManager
     */
    function blockNum() public view override returns (uint256) {
        return block.number;
    }

    /**
     * @inheritdoc IEpochManager
     */
    function blockHash(uint256 _block) external view override returns (bytes32) {
        uint256 currentBlock = blockNum();

        require(_block < currentBlock, "Can only retrieve past block hashes");
        require(currentBlock < 256 || _block >= currentBlock - 256, "Can only retrieve hashes for last 256 blocks");

        return blockhash(_block);
    }

    /**
     * @inheritdoc IEpochManager
     */
    function currentEpoch() public view override returns (uint256) {
        return lastLengthUpdateEpoch.add(epochsSinceUpdate());
    }

    /**
     * @inheritdoc IEpochManager
     */
    function currentEpochBlock() public view override returns (uint256) {
        return lastLengthUpdateBlock.add(epochsSinceUpdate().mul(epochLength));
    }

    /**
     * @inheritdoc IEpochManager
     */
    function currentEpochBlockSinceStart() external view override returns (uint256) {
        return blockNum() - currentEpochBlock();
    }

    /**
     * @inheritdoc IEpochManager
     */
    function epochsSince(uint256 _epoch) external view override returns (uint256) {
        uint256 epoch = currentEpoch();
        return _epoch < epoch ? epoch.sub(_epoch) : 0;
    }

    /**
     * @inheritdoc IEpochManager
     */
    function epochsSinceUpdate() public view override returns (uint256) {
        return blockNum().sub(lastLengthUpdateBlock).div(epochLength);
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

// SPDX-License-Identifier: MIT

pragma solidity ^0.7.3;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable use-natspec, gas-small-strings

/**
 * @dev Contract module which provides a basic access control mechanism, where
 * there is an account (an owner) that can be granted exclusive access to
 * specific functions.
 *
 * The owner account will be passed on initialization of the contract. This
 * can later be changed with {transferOwnership}.
 *
 * This module is used through inheritance. It will make available the modifier
 * `onlyOwner`, which can be applied to your functions to restrict their use to
 * the owner.
 */
contract Ownable {
    /// @dev Owner of the contract, can be retrieved with the public owner() function
    address private _owner;
    /// @dev Since upgradeable contracts might inherit this, we add a storage gap
    /// to allow adding variables here without breaking the proxy storage layout
    uint256[50] private __gap;

    /// @dev Emitted when ownership of the contract is transferred
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);

    /**
     * @dev Initializes the contract setting the deployer as the initial owner.
     */
    function _initialize(address owner) internal {
        _owner = owner;
        emit OwnershipTransferred(address(0), owner);
    }

    /**
     * @dev Returns the address of the current owner.
     */
    function owner() public view returns (address) {
        return _owner;
    }

    /**
     * @dev Throws if called by any account other than the owner.
     */
    modifier onlyOwner() {
        require(_owner == msg.sender, "Ownable: caller is not the owner");
        _;
    }

    /**
     * @dev Leaves the contract without owner. It will not be possible to call
     * `onlyOwner` functions anymore. Can only be called by the current owner.
     *
     * NOTE: Renouncing ownership will leave the contract without an owner,
     * thereby removing any functionality that is only available to the owner.
     */
    function renounceOwnership() external virtual onlyOwner {
        emit OwnershipTransferred(_owner, address(0));
        _owner = address(0);
    }

    /**
     * @dev Transfers ownership of the contract to a new account (`newOwner`).
     * Can only be called by the current owner.
     */
    function transferOwnership(address newOwner) external virtual onlyOwner {
        require(newOwner != address(0), "Ownable: new owner is the zero address");
        emit OwnershipTransferred(_owner, newOwner);
        _owner = newOwner;
    }
}

// SPDX-License-Identifier: MIT

pragma solidity ^0.7.3;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable use-natspec

import { Address } from "@openzeppelin/contracts/utils/Address.sol";
import { Create2 } from "@openzeppelin/contracts/utils/Create2.sol";

/**
 * @title MinimalProxyFactory: a factory contract for creating minimal proxies
 * @notice Adapted from https://github.com/OpenZeppelin/openzeppelin-sdk/blob/v2.5.0/packages/lib/contracts/upgradeability/ProxyFactory.sol
 * Based on https://eips.ethereum.org/EIPS/eip-1167
 */
contract MinimalProxyFactory {
    /// @dev Emitted when a new proxy is created
    event ProxyCreated(address indexed proxy);

    /**
     * @notice Gets the deterministic CREATE2 address for MinimalProxy with a particular implementation
     * @dev Uses address(this) as deployer to compute the address. Only for backwards compatibility.
     * @param _salt Bytes32 salt to use for CREATE2
     * @param _implementation Address of the proxy target implementation
     * @return Address of the counterfactual MinimalProxy
     */
    function getDeploymentAddress(bytes32 _salt, address _implementation) public view returns (address) {
        return getDeploymentAddress(_salt, _implementation, address(this));
    }

    /**
     * @notice Gets the deterministic CREATE2 address for MinimalProxy with a particular implementation
     * @param _salt Bytes32 salt to use for CREATE2
     * @param _implementation Address of the proxy target implementation
     * @param _deployer Address of the deployer that creates the contract
     * @return Address of the counterfactual MinimalProxy
     */
    function getDeploymentAddress(
        bytes32 _salt,
        address _implementation,
        address _deployer
    ) public pure returns (address) {
        return Create2.computeAddress(_salt, keccak256(_getContractCreationCode(_implementation)), _deployer);
    }

    /**
     * @dev Deploys a MinimalProxy with CREATE2
     * @param _salt Bytes32 salt to use for CREATE2
     * @param _implementation Address of the proxy target implementation
     * @param _data Bytes with the initializer call
     * @return Address of the deployed MinimalProxy
     */
    function _deployProxy2(bytes32 _salt, address _implementation, bytes memory _data) internal returns (address) {
        address proxyAddress = Create2.deploy(0, _salt, _getContractCreationCode(_implementation));

        emit ProxyCreated(proxyAddress);

        // Call function with data
        if (_data.length > 0) {
            Address.functionCall(proxyAddress, _data);
        }

        return proxyAddress;
    }

    /**
     * @dev Gets the MinimalProxy bytecode
     * @param _implementation Address of the proxy target implementation
     * @return MinimalProxy bytecode
     */
    function _getContractCreationCode(address _implementation) internal pure returns (bytes memory) {
        bytes10 creation = 0x3d602d80600a3d3981f3;
        bytes10 prefix = 0x363d3d373d3d3d363d73;
        bytes20 targetBytes = bytes20(_implementation);
        bytes15 suffix = 0x5af43d82803e903d91602b57fd5bf3;
        return abi.encodePacked(creation, prefix, targetBytes, suffix);
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

pragma solidity ^0.7.6;

import { Managed } from "../governance/Managed.sol";

/**
 * @title Epoch Manager Storage V1
 * @author Edge & Node
 * @notice Storage contract for the Epoch Manager
 */
contract EpochManagerV1Storage is Managed {
    // -- State --

    /// @notice Epoch length in blocks
    uint256 public epochLength;

    /// @notice Epoch that was last run
    uint256 public lastRunEpoch;

    /// @notice Epoch when epoch length was last updated
    uint256 public lastLengthUpdateEpoch;
    /// @notice Block when epoch length was last updated
    uint256 public lastLengthUpdateBlock;
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

// SPDX-License-Identifier: MIT

pragma solidity ^0.7.3;
pragma experimental ABIEncoderV2;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable gas-indexed-events, use-natspec
// solhint-disable named-parameters-mapping

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/SafeERC20.sol";

import { ICallhookReceiver } from "@graphprotocol/interfaces/contracts/contracts/gateway/ICallhookReceiver.sol";
import { GraphTokenLockManager } from "./GraphTokenLockManager.sol";
import { L2GraphTokenLockWallet } from "./L2GraphTokenLockWallet.sol";

/**
 * @title L2GraphTokenLockManager
 * @notice This contract manages a list of authorized function calls and targets that can be called
 * by any TokenLockWallet contract and it is a factory of TokenLockWallet contracts.
 *
 * This contract receives funds to make the process of creating TokenLockWallet contracts
 * easier by distributing them the initial tokens to be managed.
 *
 * In particular, this L2 variant is designed to receive token lock wallets from L1,
 * through the GRT bridge. These transferred wallets will not allow releasing funds in L2 until
 * the end of the vesting timeline, but they can allow withdrawing funds back to L1 using
 * the L2GraphTokenLockTransferTool contract.
 *
 * The owner can setup a list of token destinations that will be used by TokenLock contracts to
 * approve the pulling of funds, this way in can be guaranteed that only protocol contracts
 * will manipulate users funds.
 */
contract L2GraphTokenLockManager is GraphTokenLockManager, ICallhookReceiver {
    using SafeERC20 for IERC20;

    /// @dev Struct to hold the data of a transferred wallet; this is
    /// the data that must be encoded in L1 to send a wallet to L2.
    struct TransferredWalletData {
        address l1Address;
        address owner;
        address beneficiary;
        uint256 managedAmount;
        uint256 startTime;
        uint256 endTime;
    }

    /// Address of the L2GraphTokenGateway
    // solhint-disable-next-line immutable-vars-naming
    address public immutable l2Gateway;
    /// Address of the L1 transfer tool contract (in L1, no aliasing)
    // solhint-disable-next-line immutable-vars-naming
    address public immutable l1TransferTool;
    /// Mapping of each L1 wallet to its L2 wallet counterpart (populated when each wallet is received)
    /// L1 address => L2 address
    mapping(address => address) public l1WalletToL2Wallet;
    /// Mapping of each L2 wallet to its L1 wallet counterpart (populated when each wallet is received)
    /// L2 address => L1 address
    mapping(address => address) public l2WalletToL1Wallet;

    /// @dev Event emitted when a wallet is received and created from L1
    event TokenLockCreatedFromL1(
        address indexed contractAddress,
        bytes32 initHash,
        address indexed beneficiary,
        uint256 managedAmount,
        uint256 startTime,
        uint256 endTime,
        address indexed l1Address
    );

    /// @dev Emitted when locked tokens are received from L1 (whether the wallet
    /// had already been received or not)
    event LockedTokensReceivedFromL1(address indexed l1Address, address indexed l2Address, uint256 amount);

    /**
     * @dev Checks that the sender is the L2GraphTokenGateway.
     */
    modifier onlyL2Gateway() {
        require(msg.sender == l2Gateway, "ONLY_GATEWAY");
        _;
    }

    /**
     * @notice Constructor for the L2GraphTokenLockManager contract.
     * @param _graphToken Address of the L2 GRT token contract
     * @param _masterCopy Address of the master copy of the L2GraphTokenLockWallet implementation
     * @param _l2Gateway Address of the L2GraphTokenGateway contract
     * @param _l1TransferTool Address of the L1 transfer tool contract (in L1, without aliasing)
     */
    constructor(
        IERC20 _graphToken,
        address _masterCopy,
        address _l2Gateway,
        address _l1TransferTool
    ) GraphTokenLockManager(_graphToken, _masterCopy) {
        l2Gateway = _l2Gateway;
        l1TransferTool = _l1TransferTool;
    }

    /**
     * @notice This function is called by the L2GraphTokenGateway when tokens are sent from L1.
     * @dev This function will create a new wallet if it doesn't exist yet, or send the tokens to
     * the existing wallet if it does.
     * @param _from Address of the sender in L1, which must be the L1GraphTokenLockTransferTool
     * @param _amount Amount of tokens received
     * @param _data Encoded data of the transferred wallet, which must be an ABI-encoded TransferredWalletData struct
     */
    function onTokenTransfer(address _from, uint256 _amount, bytes calldata _data) external override onlyL2Gateway {
        require(_from == l1TransferTool, "ONLY_TRANSFER_TOOL");
        TransferredWalletData memory walletData = abi.decode(_data, (TransferredWalletData));

        if (l1WalletToL2Wallet[walletData.l1Address] != address(0)) {
            // If the wallet was already received, just send the tokens to the L2 address
            _token.safeTransfer(l1WalletToL2Wallet[walletData.l1Address], _amount);
        } else {
            // Create contract using a minimal proxy and call initializer
            (bytes32 initHash, address contractAddress) = _deployFromL1(keccak256(_data), walletData);
            l1WalletToL2Wallet[walletData.l1Address] = contractAddress;
            l2WalletToL1Wallet[contractAddress] = walletData.l1Address;

            // Send managed amount to the created contract
            _token.safeTransfer(contractAddress, _amount);

            emit TokenLockCreatedFromL1(
                contractAddress,
                initHash,
                walletData.beneficiary,
                walletData.managedAmount,
                walletData.startTime,
                walletData.endTime,
                walletData.l1Address
            );
        }
        emit LockedTokensReceivedFromL1(walletData.l1Address, l1WalletToL2Wallet[walletData.l1Address], _amount);
    }

    /**
     * @dev Deploy a token lock wallet with data received from L1
     * @param _salt Salt for the CREATE2 call, which must be the hash of the wallet data
     * @param _walletData Data of the wallet to be created
     * @return Hash of the initialization calldata
     * @return Address of the created contract
     */
    function _deployFromL1(
        bytes32 _salt,
        TransferredWalletData memory _walletData
    ) internal returns (bytes32, address) {
        bytes memory initializer = _encodeInitializer(_walletData);
        address contractAddress = _deployProxy2(_salt, masterCopy, initializer);
        return (keccak256(initializer), contractAddress);
    }

    /**
     * @dev Encode the initializer for the token lock wallet received from L1
     * @param _walletData Data of the wallet to be created
     * @return Encoded initializer calldata, including the function signature
     */
    function _encodeInitializer(TransferredWalletData memory _walletData) internal view returns (bytes memory) {
        return
            abi.encodeWithSelector(
                L2GraphTokenLockWallet.initializeFromL1.selector,
                address(this),
                address(_token),
                _walletData
            );
    }
}

// SPDX-License-Identifier: MIT

pragma solidity ^0.7.3;
pragma experimental ABIEncoderV2;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable use-natspec

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { GraphTokenLockWallet } from "./GraphTokenLockWallet.sol";
import { Ownable as OwnableInitializable } from "./Ownable.sol";
import { L2GraphTokenLockManager } from "./L2GraphTokenLockManager.sol";

/**
 * @title L2GraphTokenLockWallet
 * @notice This contract is built on top of the base GraphTokenLock functionality.
 * It allows wallet beneficiaries to use the deposited funds to perform specific function calls
 * on specific contracts.
 *
 * The idea is that supporters with locked tokens can participate in the protocol
 * but disallow any release before the vesting/lock schedule.
 * The beneficiary can issue authorized function calls to this contract that will
 * get forwarded to a target contract. A target contract is any of our protocol contracts.
 * The function calls allowed are queried to the GraphTokenLockManager, this way
 * the same configuration can be shared for all the created lock wallet contracts.
 *
 * This L2 variant includes a special initializer so that it can be created from
 * a wallet's data received from L1. These transferred wallets will not allow releasing
 * funds in L2 until the end of the vesting timeline, but they can allow withdrawing
 * funds back to L1 using the L2GraphTokenLockTransferTool contract.
 *
 * Note that surplusAmount and releasedAmount in L2 will be skewed for wallets received from L1,
 * so releasing surplus tokens might also only be possible by bridging tokens back to L1.
 *
 * NOTE: Contracts used as target must have its function signatures checked to avoid collisions
 * with any of this contract functions.
 * Beneficiaries need to approve the use of the tokens to the protocol contracts. For convenience
 * the maximum amount of tokens is authorized.
 * Function calls do not forward ETH value so DO NOT SEND ETH TO THIS CONTRACT.
 */
contract L2GraphTokenLockWallet is GraphTokenLockWallet {
    // Initializer when created from a message from L1
    function initializeFromL1(
        address _manager,
        address _token,
        L2GraphTokenLockManager.TransferredWalletData calldata _walletData
    ) external {
        require(!isInitialized, "Already initialized");
        isInitialized = true;

        OwnableInitializable._initialize(_walletData.owner);
        beneficiary = _walletData.beneficiary;
        token = IERC20(_token);

        managedAmount = _walletData.managedAmount;

        startTime = _walletData.startTime;
        endTime = _walletData.endTime;
        periods = 1;
        isAccepted = true;

        // Optionals
        releaseStartTime = _walletData.endTime;
        revocable = Revocability.Disabled;

        _setManager(_manager);
    }
}

// SPDX-License-Identifier: MIT

pragma solidity ^0.7.3;
pragma experimental ABIEncoderV2;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable use-natspec

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { IGraphTokenLock } from "./IGraphTokenLock.sol";

interface IGraphTokenLockManager {
    // -- Factory --

    function setMasterCopy(address _masterCopy) external;

    function createTokenLockWallet(
        address _owner,
        address _beneficiary,
        uint256 _managedAmount,
        uint256 _startTime,
        uint256 _endTime,
        uint256 _periods,
        uint256 _releaseStartTime,
        uint256 _vestingCliffTime,
        IGraphTokenLock.Revocability _revocable
    ) external;

    // -- Funds Management --

    function token() external returns (IERC20);

    function deposit(uint256 _amount) external;

    function withdraw(uint256 _amount) external;

    // -- Allowed Funds Destinations --

    function addTokenDestination(address _dst) external;

    function removeTokenDestination(address _dst) external;

    function isTokenDestination(address _dst) external view returns (bool);

    function getTokenDestinations() external view returns (address[] memory);

    // -- Function Call Authorization --

    function setAuthFunctionCall(string calldata _signature, address _target) external;

    function unsetAuthFunctionCall(string calldata _signature) external;

    function setAuthFunctionCallMany(string[] calldata _signatures, address[] calldata _targets) external;

    function getAuthFunctionCallTarget(bytes4 _sigHash) external view returns (address);

    function isAuthFunctionCall(bytes4 _sigHash) external view returns (bool);
}

// SPDX-License-Identifier: MIT

pragma solidity ^0.7.3;
pragma experimental ABIEncoderV2;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable use-natspec

interface IGraphTokenLock {
    enum Revocability {
        NotSet,
        Enabled,
        Disabled
    }

    // -- Balances --

    function currentBalance() external view returns (uint256);

    // -- Time & Periods --

    function currentTime() external view returns (uint256);

    function duration() external view returns (uint256);

    function sinceStartTime() external view returns (uint256);

    function amountPerPeriod() external view returns (uint256);

    function periodDuration() external view returns (uint256);

    function currentPeriod() external view returns (uint256);

    function passedPeriods() external view returns (uint256);

    // -- Locking & Release Schedule --

    function availableAmount() external view returns (uint256);

    function vestedAmount() external view returns (uint256);

    function releasableAmount() external view returns (uint256);

    function totalOutstandingAmount() external view returns (uint256);

    function surplusAmount() external view returns (uint256);

    // -- Value Transfer --

    function release() external;

    function withdrawSurplus(uint256 _amount) external;

    function revoke() external;
}

// SPDX-License-Identifier: MIT

pragma solidity ^0.7.3;
pragma experimental ABIEncoderV2;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable use-natspec, gas-increment-by-one, gas-strict-inequalities, gas-small-strings

import { Address } from "@openzeppelin/contracts/utils/Address.sol";
import { SafeMath } from "@openzeppelin/contracts/math/SafeMath.sol";

import { GraphTokenLock } from "./GraphTokenLock.sol";
import { IGraphTokenLock } from "./IGraphTokenLock.sol";
import { IGraphTokenLockManager } from "./IGraphTokenLockManager.sol";

/**
 * @title GraphTokenLockWallet
 * @notice This contract is built on top of the base GraphTokenLock functionality.
 * It allows wallet beneficiaries to use the deposited funds to perform specific function calls
 * on specific contracts.
 *
 * The idea is that supporters with locked tokens can participate in the protocol
 * but disallow any release before the vesting/lock schedule.
 * The beneficiary can issue authorized function calls to this contract that will
 * get forwarded to a target contract. A target contract is any of our protocol contracts.
 * The function calls allowed are queried to the GraphTokenLockManager, this way
 * the same configuration can be shared for all the created lock wallet contracts.
 *
 * NOTE: Contracts used as target must have its function signatures checked to avoid collisions
 * with any of this contract functions.
 * Beneficiaries need to approve the use of the tokens to the protocol contracts. For convenience
 * the maximum amount of tokens is authorized.
 * Function calls do not forward ETH value so DO NOT SEND ETH TO THIS CONTRACT.
 */
contract GraphTokenLockWallet is GraphTokenLock {
    using SafeMath for uint256;

    // -- State --

    IGraphTokenLockManager public manager;

    // -- Events --

    event ManagerUpdated(address indexed _oldManager, address indexed _newManager);
    event TokenDestinationsApproved();
    event TokenDestinationsRevoked();

    // Initializer
    function initialize(
        address _manager,
        address _owner,
        address _beneficiary,
        address _token,
        uint256 _managedAmount,
        uint256 _startTime,
        uint256 _endTime,
        uint256 _periods,
        uint256 _releaseStartTime,
        uint256 _vestingCliffTime,
        IGraphTokenLock.Revocability _revocable
    ) external {
        _initialize(
            _owner,
            _beneficiary,
            _token,
            _managedAmount,
            _startTime,
            _endTime,
            _periods,
            _releaseStartTime,
            _vestingCliffTime,
            _revocable
        );
        _setManager(_manager);
    }

    // -- Admin --

    /**
     * @notice Sets a new manager for this contract
     * @param _newManager Address of the new manager
     */
    function setManager(address _newManager) external onlyOwner {
        _setManager(_newManager);
    }

    /**
     * @dev Sets a new manager for this contract
     * @param _newManager Address of the new manager
     */
    function _setManager(address _newManager) internal {
        require(_newManager != address(0), "Manager cannot be empty");
        require(Address.isContract(_newManager), "Manager must be a contract");

        address oldManager = address(manager);
        manager = IGraphTokenLockManager(_newManager);

        emit ManagerUpdated(oldManager, _newManager);
    }

    // -- Beneficiary --

    /**
     * @notice Approves protocol access of the tokens managed by this contract
     * @dev Approves all token destinations registered in the manager to pull tokens
     */
    function approveProtocol() external onlyBeneficiary {
        address[] memory dstList = manager.getTokenDestinations();
        for (uint256 i = 0; i < dstList.length; i++) {
            // Note this is only safe because we are using the max uint256 value
            token.approve(dstList[i], type(uint256).max);
        }
        emit TokenDestinationsApproved();
    }

    /**
     * @notice Revokes protocol access of the tokens managed by this contract
     * @dev Revokes approval to all token destinations in the manager to pull tokens
     */
    function revokeProtocol() external onlyBeneficiary {
        address[] memory dstList = manager.getTokenDestinations();
        for (uint256 i = 0; i < dstList.length; i++) {
            // Note this is only safe cause we're using 0 as the amount
            token.approve(dstList[i], 0);
        }
        emit TokenDestinationsRevoked();
    }

    /**
     * @notice Forward authorized contract calls to protocol contracts
     * @dev Fallback function can be called by the beneficiary only if function call is allowed
     */
    // solhint-disable-next-line no-complex-fallback
    fallback() external {
        // Only beneficiary can forward calls
        require(msg.sender == beneficiary, "Unauthorized caller");

        // Only non-revocable contracts can forward calls
        require(revocable == Revocability.Disabled, "Revocable contracts cannot forward calls");

        // Function call validation
        address _target = manager.getAuthFunctionCallTarget(msg.sig);
        require(_target != address(0), "Unauthorized function");

        // Call function with data
        Address.functionCall(_target, msg.data);
    }
}

// SPDX-License-Identifier: MIT

pragma solidity ^0.7.3;
pragma experimental ABIEncoderV2;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable use-natspec, gas-indexed-events, gas-strict-inequalities, gas-increment-by-one
// solhint-disable named-parameters-mapping

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/SafeERC20.sol";
import { Address } from "@openzeppelin/contracts/utils/Address.sol";
import { EnumerableSet } from "@openzeppelin/contracts/utils/EnumerableSet.sol";
import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";

import { MinimalProxyFactory } from "./MinimalProxyFactory.sol";
import { IGraphTokenLockManager } from "./IGraphTokenLockManager.sol";
import { IGraphTokenLock } from "./IGraphTokenLock.sol";
import { GraphTokenLockWallet } from "./GraphTokenLockWallet.sol";

/**
 * @title GraphTokenLockManager
 * @notice This contract manages a list of authorized function calls and targets that can be called
 * by any TokenLockWallet contract and it is a factory of TokenLockWallet contracts.
 *
 * This contract receives funds to make the process of creating TokenLockWallet contracts
 * easier by distributing them the initial tokens to be managed.
 *
 * The owner can setup a list of token destinations that will be used by TokenLock contracts to
 * approve the pulling of funds, this way in can be guaranteed that only protocol contracts
 * will manipulate users funds.
 */
contract GraphTokenLockManager is Ownable, MinimalProxyFactory, IGraphTokenLockManager {
    using SafeERC20 for IERC20;
    using EnumerableSet for EnumerableSet.AddressSet;

    // -- State --

    mapping(bytes4 => address) public authFnCalls;
    EnumerableSet.AddressSet private _tokenDestinations;

    address public masterCopy;
    IERC20 internal _token;

    // -- Events --

    event MasterCopyUpdated(address indexed masterCopy);
    event TokenLockCreated(
        address indexed contractAddress,
        bytes32 indexed initHash,
        address indexed beneficiary,
        address token,
        uint256 managedAmount,
        uint256 startTime,
        uint256 endTime,
        uint256 periods,
        uint256 releaseStartTime,
        uint256 vestingCliffTime,
        IGraphTokenLock.Revocability revocable
    );

    event TokensDeposited(address indexed sender, uint256 amount);
    event TokensWithdrawn(address indexed sender, uint256 amount);

    event FunctionCallAuth(address indexed caller, bytes4 indexed sigHash, address indexed target, string signature);
    event TokenDestinationAllowed(address indexed dst, bool allowed);

    /**
     * Constructor.
     * @param _graphToken Token to use for deposits and withdrawals
     * @param _masterCopy Address of the master copy to use to clone proxies
     */
    constructor(IERC20 _graphToken, address _masterCopy) {
        require(address(_graphToken) != address(0), "Token cannot be zero");
        _token = _graphToken;
        setMasterCopy(_masterCopy);
    }

    // -- Factory --

    /**
     * @notice Sets the masterCopy bytecode to use to create clones of TokenLock contracts
     * @param _masterCopy Address of contract bytecode to factory clone
     */
    function setMasterCopy(address _masterCopy) public override onlyOwner {
        require(_masterCopy != address(0), "MasterCopy cannot be zero");
        masterCopy = _masterCopy;
        emit MasterCopyUpdated(_masterCopy);
    }

    /**
     * @notice Creates and fund a new token lock wallet using a minimum proxy
     * @param _owner Address of the contract owner
     * @param _beneficiary Address of the beneficiary of locked tokens
     * @param _managedAmount Amount of tokens to be managed by the lock contract
     * @param _startTime Start time of the release schedule
     * @param _endTime End time of the release schedule
     * @param _periods Number of periods between start time and end time
     * @param _releaseStartTime Override time for when the releases start
     * @param _revocable Whether the contract is revocable
     */
    function createTokenLockWallet(
        address _owner,
        address _beneficiary,
        uint256 _managedAmount,
        uint256 _startTime,
        uint256 _endTime,
        uint256 _periods,
        uint256 _releaseStartTime,
        uint256 _vestingCliffTime,
        IGraphTokenLock.Revocability _revocable
    ) external override onlyOwner {
        require(_token.balanceOf(address(this)) >= _managedAmount, "Not enough tokens to create lock");

        // Create contract using a minimal proxy and call initializer
        bytes memory initializer = abi.encodeWithSelector(
            GraphTokenLockWallet.initialize.selector,
            address(this),
            _owner,
            _beneficiary,
            address(_token),
            _managedAmount,
            _startTime,
            _endTime,
            _periods,
            _releaseStartTime,
            _vestingCliffTime,
            _revocable
        );
        address contractAddress = _deployProxy2(keccak256(initializer), masterCopy, initializer);

        // Send managed amount to the created contract
        _token.safeTransfer(contractAddress, _managedAmount);

        emit TokenLockCreated(
            contractAddress,
            keccak256(initializer),
            _beneficiary,
            address(_token),
            _managedAmount,
            _startTime,
            _endTime,
            _periods,
            _releaseStartTime,
            _vestingCliffTime,
            _revocable
        );
    }

    // -- Funds Management --

    /**
     * @notice Gets the GRT token address
     * @return Token used for transfers and approvals
     */
    function token() external view override returns (IERC20) {
        return _token;
    }

    /**
     * @notice Deposits tokens into the contract
     * @dev Even if the ERC20 token can be transferred directly to the contract
     * this function provide a safe interface to do the transfer and avoid mistakes
     * @param _amount Amount to deposit
     */
    function deposit(uint256 _amount) external override {
        require(_amount > 0, "Amount cannot be zero");
        _token.safeTransferFrom(msg.sender, address(this), _amount);
        emit TokensDeposited(msg.sender, _amount);
    }

    /**
     * @notice Withdraws tokens from the contract
     * @dev Escape hatch in case of mistakes or to recover remaining funds
     * @param _amount Amount of tokens to withdraw
     */
    function withdraw(uint256 _amount) external override onlyOwner {
        require(_amount > 0, "Amount cannot be zero");
        _token.safeTransfer(msg.sender, _amount);
        emit TokensWithdrawn(msg.sender, _amount);
    }

    // -- Token Destinations --

    /**
     * @notice Adds an address that can be allowed by a token lock to pull funds
     * @param _dst Destination address
     */
    function addTokenDestination(address _dst) external override onlyOwner {
        require(_dst != address(0), "Destination cannot be zero");
        require(_tokenDestinations.add(_dst), "Destination already added");
        emit TokenDestinationAllowed(_dst, true);
    }

    /**
     * @notice Removes an address that can be allowed by a token lock to pull funds
     * @param _dst Destination address
     */
    function removeTokenDestination(address _dst) external override onlyOwner {
        require(_tokenDestinations.remove(_dst), "Destination already removed");
        emit TokenDestinationAllowed(_dst, false);
    }

    /**
     * @notice Returns True if the address is authorized to be a destination of tokens
     * @param _dst Destination address
     * @return True if authorized
     */
    function isTokenDestination(address _dst) external view override returns (bool) {
        return _tokenDestinations.contains(_dst);
    }

    /**
     * @notice Returns an array of authorized destination addresses
     * @return Array of addresses authorized to pull funds from a token lock
     */
    function getTokenDestinations() external view override returns (address[] memory) {
        address[] memory dstList = new address[](_tokenDestinations.length());
        for (uint256 i = 0; i < _tokenDestinations.length(); i++) {
            dstList[i] = _tokenDestinations.at(i);
        }
        return dstList;
    }

    // -- Function Call Authorization --

    /**
     * @notice Sets an authorized function call to target
     * @dev Input expected is the function signature as 'transfer(address,uint256)'
     * @param _signature Function signature
     * @param _target Address of the destination contract to call
     */
    function setAuthFunctionCall(string calldata _signature, address _target) external override onlyOwner {
        _setAuthFunctionCall(_signature, _target);
    }

    /**
     * @notice Unsets an authorized function call to target
     * @dev Input expected is the function signature as 'transfer(address,uint256)'
     * @param _signature Function signature
     */
    function unsetAuthFunctionCall(string calldata _signature) external override onlyOwner {
        bytes4 sigHash = _toFunctionSigHash(_signature);
        authFnCalls[sigHash] = address(0);

        emit FunctionCallAuth(msg.sender, sigHash, address(0), _signature);
    }

    /**
     * @notice Sets an authorized function call to target in bulk
     * @dev Input expected is the function signature as 'transfer(address,uint256)'
     * @param _signatures Function signatures
     * @param _targets Address of the destination contract to call
     */
    function setAuthFunctionCallMany(
        string[] calldata _signatures,
        address[] calldata _targets
    ) external override onlyOwner {
        require(_signatures.length == _targets.length, "Array length mismatch");

        for (uint256 i = 0; i < _signatures.length; i++) {
            _setAuthFunctionCall(_signatures[i], _targets[i]);
        }
    }

    /**
     * @notice Sets an authorized function call to target
     * @dev Input expected is the function signature as 'transfer(address,uint256)'
     * @dev Function signatures of Graph Protocol contracts to be used are known ahead of time
     * @param _signature Function signature
     * @param _target Address of the destination contract to call
     */
    function _setAuthFunctionCall(string calldata _signature, address _target) internal {
        require(_target != address(this), "Target must be other contract");
        require(Address.isContract(_target), "Target must be a contract");

        bytes4 sigHash = _toFunctionSigHash(_signature);
        authFnCalls[sigHash] = _target;

        emit FunctionCallAuth(msg.sender, sigHash, _target, _signature);
    }

    /**
     * @notice Gets the target contract to call for a particular function signature
     * @param _sigHash Function signature hash
     * @return Address of the target contract where to send the call
     */
    function getAuthFunctionCallTarget(bytes4 _sigHash) public view override returns (address) {
        return authFnCalls[_sigHash];
    }

    /**
     * @notice Returns true if the function call is authorized
     * @param _sigHash Function signature hash
     * @return True if authorized
     */
    function isAuthFunctionCall(bytes4 _sigHash) external view override returns (bool) {
        return getAuthFunctionCallTarget(_sigHash) != address(0);
    }

    /**
     * @dev Converts a function signature string to 4-bytes hash
     * @param _signature Function signature string
     * @return Function signature hash
     */
    function _toFunctionSigHash(string calldata _signature) internal pure returns (bytes4) {
        return _convertToBytes4(abi.encodeWithSignature(_signature));
    }

    /**
     * @dev Converts function signature bytes to function signature hash (bytes4)
     * @param _signature Function signature
     * @return Function signature in bytes4
     */
    function _convertToBytes4(bytes memory _signature) internal pure returns (bytes4) {
        require(_signature.length == 4, "Invalid method signature");
        bytes4 sigHash;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            sigHash := mload(add(_signature, 32))
        }
        return sigHash;
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

// SPDX-License-Identifier: MIT

pragma solidity ^0.7.3;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable use-natspec, gas-indexed-events, gas-strict-inequalities, gas-small-strings

import { SafeMath } from "@openzeppelin/contracts/math/SafeMath.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/SafeERC20.sol";

import { Ownable as OwnableInitializable } from "./Ownable.sol";
import { MathUtils } from "./MathUtils.sol";
import { IGraphTokenLock } from "./IGraphTokenLock.sol";

/**
 * @title GraphTokenLock
 * @notice Contract that manages an unlocking schedule of tokens.
 * @dev The contract lock manage a number of tokens deposited into the contract to ensure that
 * they can only be released under certain time conditions.
 *
 * This contract implements a release scheduled based on periods and tokens are released in steps
 * after each period ends. It can be configured with one period in which case it is like a plain TimeLock.
 * It also supports revocation to be used for vesting schedules.
 *
 * The contract supports receiving extra funds than the managed tokens ones that can be
 * withdrawn by the beneficiary at any time.
 *
 * A releaseStartTime parameter is included to override the default release schedule and
 * perform the first release on the configured time. After that it will continue with the
 * default schedule.
 */
abstract contract GraphTokenLock is OwnableInitializable, IGraphTokenLock {
    using SafeMath for uint256;
    using SafeERC20 for IERC20;

    uint256 private constant MIN_PERIOD = 1;

    // -- State --

    IERC20 public token;
    address public beneficiary;

    // Configuration

    // Amount of tokens managed by the contract schedule
    uint256 public managedAmount;

    uint256 public startTime; // Start datetime (in unixtimestamp)
    uint256 public endTime; // Datetime after all funds are fully vested/unlocked (in unixtimestamp)
    uint256 public periods; // Number of vesting/release periods

    // First release date for tokens (in unixtimestamp)
    // If set, no tokens will be released before releaseStartTime ignoring
    // the amount to release each period
    uint256 public releaseStartTime;
    // A cliff set a date to which a beneficiary needs to get to vest
    // all preceding periods
    uint256 public vestingCliffTime;
    IGraphTokenLock.Revocability public revocable; // Whether to use vesting for locked funds

    // State

    bool public isRevoked;
    bool public isInitialized;
    bool public isAccepted;
    uint256 public releasedAmount;
    uint256 public revokedAmount;

    // -- Events --

    event TokensReleased(address indexed beneficiary, uint256 amount);
    event TokensWithdrawn(address indexed beneficiary, uint256 amount);
    event TokensRevoked(address indexed beneficiary, uint256 amount);
    event BeneficiaryChanged(address newBeneficiary);
    event LockAccepted();
    event LockCanceled();

    /**
     * @dev Only allow calls from the beneficiary of the contract
     */
    modifier onlyBeneficiary() {
        require(msg.sender == beneficiary, "!auth");
        _;
    }

    /**
     * @notice Initializes the contract
     * @param _owner Address of the contract owner
     * @param _beneficiary Address of the beneficiary of locked tokens
     * @param _managedAmount Amount of tokens to be managed by the lock contract
     * @param _startTime Start time of the release schedule
     * @param _endTime End time of the release schedule
     * @param _periods Number of periods between start time and end time
     * @param _releaseStartTime Override time for when the releases start
     * @param _vestingCliffTime Override time for when the vesting start
     * @param _revocable Whether the contract is revocable
     */
    function _initialize(
        address _owner,
        address _beneficiary,
        address _token,
        uint256 _managedAmount,
        uint256 _startTime,
        uint256 _endTime,
        uint256 _periods,
        uint256 _releaseStartTime,
        uint256 _vestingCliffTime,
        IGraphTokenLock.Revocability _revocable
    ) internal {
        require(!isInitialized, "Already initialized");
        require(_owner != address(0), "Owner cannot be zero");
        require(_beneficiary != address(0), "Beneficiary cannot be zero");
        require(_token != address(0), "Token cannot be zero");
        require(_managedAmount > 0, "Managed tokens cannot be zero");
        require(_startTime != 0, "Start time must be set");
        require(_startTime < _endTime, "Start time > end time");
        require(_periods >= MIN_PERIOD, "Periods cannot be below minimum");
        require(_revocable != IGraphTokenLock.Revocability.NotSet, "Must set a revocability option");
        require(_releaseStartTime < _endTime, "Release start time must be before end time");
        require(_vestingCliffTime < _endTime, "Cliff time must be before end time");

        isInitialized = true;

        OwnableInitializable._initialize(_owner);
        beneficiary = _beneficiary;
        token = IERC20(_token);

        managedAmount = _managedAmount;

        startTime = _startTime;
        endTime = _endTime;
        periods = _periods;

        // Optionals
        releaseStartTime = _releaseStartTime;
        vestingCliffTime = _vestingCliffTime;
        revocable = _revocable;
    }

    /**
     * @notice Change the beneficiary of funds managed by the contract
     * @dev Can only be called by the beneficiary
     * @param _newBeneficiary Address of the new beneficiary address
     */
    function changeBeneficiary(address _newBeneficiary) external onlyBeneficiary {
        require(_newBeneficiary != address(0), "Empty beneficiary");
        beneficiary = _newBeneficiary;
        emit BeneficiaryChanged(_newBeneficiary);
    }

    /**
     * @notice Beneficiary accepts the lock, the owner cannot retrieve back the tokens
     * @dev Can only be called by the beneficiary
     */
    function acceptLock() external onlyBeneficiary {
        isAccepted = true;
        emit LockAccepted();
    }

    /**
     * @notice Owner cancel the lock and return the balance in the contract
     * @dev Can only be called by the owner
     */
    function cancelLock() external onlyOwner {
        require(isAccepted == false, "Cannot cancel accepted contract");

        token.safeTransfer(owner(), currentBalance());

        emit LockCanceled();
    }

    // -- Balances --

    /**
     * @notice Returns the amount of tokens currently held by the contract
     * @return Tokens held in the contract
     */
    function currentBalance() public view override returns (uint256) {
        return token.balanceOf(address(this));
    }

    // -- Time & Periods --

    /**
     * @notice Returns the current block timestamp
     * @return Current block timestamp
     */
    function currentTime() public view override returns (uint256) {
        return block.timestamp;
    }

    /**
     * @notice Gets duration of contract from start to end in seconds
     * @return Amount of seconds from contract startTime to endTime
     */
    function duration() public view override returns (uint256) {
        return endTime.sub(startTime);
    }

    /**
     * @notice Gets time elapsed since the start of the contract
     * @dev Returns zero if called before conctract starTime
     * @return Seconds elapsed from contract startTime
     */
    function sinceStartTime() public view override returns (uint256) {
        uint256 current = currentTime();
        if (current <= startTime) {
            return 0;
        }
        return current.sub(startTime);
    }

    /**
     * @notice Returns amount available to be released after each period according to schedule
     * @return Amount of tokens available after each period
     */
    function amountPerPeriod() public view override returns (uint256) {
        return managedAmount.div(periods);
    }

    /**
     * @notice Returns the duration of each period in seconds
     * @return Duration of each period in seconds
     */
    function periodDuration() public view override returns (uint256) {
        return duration().div(periods);
    }

    /**
     * @notice Gets the current period based on the schedule
     * @return A number that represents the current period
     */
    function currentPeriod() public view override returns (uint256) {
        return sinceStartTime().div(periodDuration()).add(MIN_PERIOD);
    }

    /**
     * @notice Gets the number of periods that passed since the first period
     * @return A number of periods that passed since the schedule started
     */
    function passedPeriods() public view override returns (uint256) {
        return currentPeriod().sub(MIN_PERIOD);
    }

    // -- Locking & Release Schedule --

    /**
     * @notice Gets the currently available token according to the schedule
     * @dev Implements the step-by-step schedule based on periods for available tokens
     * @return Amount of tokens available according to the schedule
     */
    function availableAmount() public view override returns (uint256) {
        uint256 current = currentTime();

        // Before contract start no funds are available
        if (current < startTime) {
            return 0;
        }

        // After contract ended all funds are available
        if (current > endTime) {
            return managedAmount;
        }

        // Get available amount based on period
        return passedPeriods().mul(amountPerPeriod());
    }

    /**
     * @notice Gets the amount of currently vested tokens
     * @dev Similar to available amount, but is fully vested when contract is non-revocable
     * @return Amount of tokens already vested
     */
    function vestedAmount() public view override returns (uint256) {
        // If non-revocable it is fully vested
        if (revocable == IGraphTokenLock.Revocability.Disabled) {
            return managedAmount;
        }

        // Vesting cliff is activated and it has not passed means nothing is vested yet
        if (vestingCliffTime > 0 && currentTime() < vestingCliffTime) {
            return 0;
        }

        return availableAmount();
    }

    /**
     * @notice Gets tokens currently available for release
     * @dev Considers the schedule and takes into account already released tokens
     * @return Amount of tokens ready to be released
     */
    function releasableAmount() public view virtual override returns (uint256) {
        // If a release start time is set no tokens are available for release before this date
        // If not set it follows the default schedule and tokens are available on
        // the first period passed
        if (releaseStartTime > 0 && currentTime() < releaseStartTime) {
            return 0;
        }

        // Vesting cliff is activated and it has not passed means nothing is vested yet
        // so funds cannot be released
        if (
            revocable == IGraphTokenLock.Revocability.Enabled &&
            vestingCliffTime > 0 &&
            currentTime() < vestingCliffTime
        ) {
            return 0;
        }

        // A beneficiary can never have more releasable tokens than the contract balance
        uint256 releasable = availableAmount().sub(releasedAmount);
        return MathUtils.min(currentBalance(), releasable);
    }

    /**
     * @notice Gets the outstanding amount yet to be released based on the whole contract lifetime
     * @dev Does not consider schedule but just global amounts tracked
     * @return Amount of outstanding tokens for the lifetime of the contract
     */
    function totalOutstandingAmount() public view override returns (uint256) {
        return managedAmount.sub(releasedAmount).sub(revokedAmount);
    }

    /**
     * @notice Gets surplus amount in the contract based on outstanding amount to release
     * @dev All funds over outstanding amount is considered surplus that can be withdrawn by beneficiary.
     * Note this might not be the correct value for wallets transferred to L2 (i.e. an L2GraphTokenLockWallet), as the released amount will be
     * skewed, so the beneficiary might have to bridge back to L1 to release the surplus.
     * @return Amount of tokens considered as surplus
     */
    function surplusAmount() public view override returns (uint256) {
        uint256 balance = currentBalance();
        uint256 outstandingAmount = totalOutstandingAmount();
        if (balance > outstandingAmount) {
            return balance.sub(outstandingAmount);
        }
        return 0;
    }

    // -- Value Transfer --

    /**
     * @notice Releases tokens based on the configured schedule
     * @dev All available releasable tokens are transferred to beneficiary
     */
    function release() external override onlyBeneficiary {
        uint256 amountToRelease = releasableAmount();
        require(amountToRelease > 0, "No available releasable amount");

        releasedAmount = releasedAmount.add(amountToRelease);

        token.safeTransfer(beneficiary, amountToRelease);

        emit TokensReleased(beneficiary, amountToRelease);
    }

    /**
     * @notice Withdraws surplus, unmanaged tokens from the contract
     * @dev Tokens in the contract over outstanding amount are considered as surplus
     * @param _amount Amount of tokens to withdraw
     */
    function withdrawSurplus(uint256 _amount) external override onlyBeneficiary {
        require(_amount > 0, "Amount cannot be zero");
        require(surplusAmount() >= _amount, "Amount requested > surplus available");

        token.safeTransfer(beneficiary, _amount);

        emit TokensWithdrawn(beneficiary, _amount);
    }

    /**
     * @notice Revokes a vesting schedule and return the unvested tokens to the owner
     * @dev Vesting schedule is always calculated based on managed tokens
     */
    function revoke() external override onlyOwner {
        require(revocable == IGraphTokenLock.Revocability.Enabled, "Contract is non-revocable");
        require(isRevoked == false, "Already revoked");

        uint256 unvestedAmount = managedAmount.sub(vestedAmount());
        require(unvestedAmount > 0, "No available unvested amount");

        revokedAmount = unvestedAmount;
        isRevoked = true;

        token.safeTransfer(owner(), unvestedAmount);

        emit TokensRevoked(beneficiary, unvestedAmount);
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


## ------------ SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS ------------ 
// SPDX-License-Identifier: MIT

pragma solidity ^0.7.3;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable use-natspec

import { GraphTokenLock } from "./GraphTokenLock.sol";
import { IGraphTokenLock } from "./IGraphTokenLock.sol";
import { Ownable as OwnableInitializable } from "./Ownable.sol";

/**
 * @title GraphTokenLockSimple
 * @notice This contract is the concrete simple implementation built on top of the base
 * GraphTokenLock functionality for use when we only need the token lock schedule
 * features but no interaction with the network.
 *
 * This contract is designed to be deployed without the use of a TokenManager.
 */
contract GraphTokenLockSimple is GraphTokenLock {
    // Constructor
    constructor() {
        OwnableInitializable._initialize(msg.sender);
    }

    // Initializer
    function initialize(
        address _owner,
        address _beneficiary,
        address _token,
        uint256 _managedAmount,
        uint256 _startTime,
        uint256 _endTime,
        uint256 _periods,
        uint256 _releaseStartTime,
        uint256 _vestingCliffTime,
        IGraphTokenLock.Revocability _revocable
    ) external onlyOwner {
        _initialize(
            _owner,
            _beneficiary,
            _token,
            _managedAmount,
            _startTime,
            _endTime,
            _periods,
            _releaseStartTime,
            _vestingCliffTime,
            _revocable
        );
    }
}


## ------------ SUPPORTING CONTEXT: EXTERNAL LIBRARIES ------------ 

 ------------ END OF SUPPORTING CONTRACTS AND INTERFACES ------------ 


 ------------ ## DEPLOYMENT SCRIPTS ------------ 

