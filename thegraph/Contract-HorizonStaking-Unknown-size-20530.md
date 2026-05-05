
 ------------ ## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: GPL-2.0-or-later

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable gas-strict-inequalities
// solhint-disable gas-increment-by-one
// solhint-disable function-max-lines

pragma solidity 0.8.27 || 0.8.33;

import { IGraphToken } from "@graphprotocol/interfaces/contracts/contracts/token/IGraphToken.sol";
import { IHorizonStakingMain } from "@graphprotocol/interfaces/contracts/horizon/internal/IHorizonStakingMain.sol";
import { IHorizonStakingExtension } from "@graphprotocol/interfaces/contracts/horizon/internal/IHorizonStakingExtension.sol";
import { IGraphPayments } from "@graphprotocol/interfaces/contracts/horizon/IGraphPayments.sol";
import { ILinkedList } from "@graphprotocol/interfaces/contracts/horizon/internal/ILinkedList.sol";

import { TokenUtils } from "@graphprotocol/contracts/contracts/utils/TokenUtils.sol";
import { MathUtils } from "../libraries/MathUtils.sol";
import { PPMMath } from "../libraries/PPMMath.sol";
import { LinkedList } from "../libraries/LinkedList.sol";

import { HorizonStakingBase } from "./HorizonStakingBase.sol";

/**
 * @title HorizonStaking contract
 * @author Edge & Node
 * @notice The {HorizonStaking} contract allows service providers to stake and provision tokens to verifiers to be used
 * as economic security for a service. It also allows delegators to delegate towards a service provider provision.
 * @dev Implements the {IHorizonStakingMain} interface.
 * @dev This is the main Staking contract in The Graph protocol after the Horizon upgrade.
 * It is designed to be deployed as an upgrade to the L2Staking contract from the legacy contracts package.
 * @dev It uses a {HorizonStakingExtension} contract to implement the full {IHorizonStaking} interface through delegatecalls.
 * This is due to the contract size limit on Arbitrum (24kB). The extension contract implements functionality to support
 * the legacy staking functions. It can be eventually removed without affecting the main staking contract.
 * @custom:security-contact Please email security+contracts@thegraph.com if you find any
 * bugs. We may have an active bug bounty program.
 */
contract HorizonStaking is HorizonStakingBase, IHorizonStakingMain {
    using TokenUtils for IGraphToken;
    using PPMMath for uint256;
    using LinkedList for ILinkedList.List;

    /// @dev Maximum number of simultaneous stake thaw requests (per provision) or undelegations (per delegation)
    uint256 private constant MAX_THAW_REQUESTS = 1_000;

    /// @dev Address of the staking extension contract
    address private immutable STAKING_EXTENSION_ADDRESS;

    /// @dev Minimum amount of delegation.
    uint256 private constant MIN_DELEGATION = 1e18;

    // forge-lint: disable-next-item(unwrapped-modifier-logic)
    /**
     * @notice Checks that the caller is authorized to operate over a provision.
     * @param serviceProvider The address of the service provider.
     * @param verifier The address of the verifier.
     */
    modifier onlyAuthorized(address serviceProvider, address verifier) {
        require(
            _isAuthorized(serviceProvider, verifier, msg.sender),
            HorizonStakingNotAuthorized(serviceProvider, verifier, msg.sender)
        );
        _;
    }

    // forge-lint: disable-next-item(unwrapped-modifier-logic)
    /**
     * @notice Checks that the caller is authorized to operate over a provision or it is the verifier.
     * @param serviceProvider The address of the service provider.
     * @param verifier The address of the verifier.
     */
    modifier onlyAuthorizedOrVerifier(address serviceProvider, address verifier) {
        require(
            _isAuthorized(serviceProvider, verifier, msg.sender) || msg.sender == verifier,
            HorizonStakingNotAuthorized(serviceProvider, verifier, msg.sender)
        );
        _;
    }

    /**
     * @notice The staking contract is upgradeable however we still use the constructor to set a few immutable variables
     * @param controller The address of the Graph controller contract
     * @param stakingExtensionAddress The address of the staking extension contract
     * @param subgraphDataServiceAddress The address of the subgraph data service
     */
    constructor(
        address controller,
        address stakingExtensionAddress,
        address subgraphDataServiceAddress
    ) HorizonStakingBase(controller, subgraphDataServiceAddress) {
        STAKING_EXTENSION_ADDRESS = stakingExtensionAddress;
    }

    /**
     * @notice Delegates the current call to the StakingExtension implementation.
     * @dev This function does not return to its internal call site, it will return directly to the
     * external caller.
     */
    fallback() external {
        // solhint-disable-previous-line payable-fallback, no-complex-fallback
        address extensionImpl = STAKING_EXTENSION_ADDRESS;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            // (a) get free memory pointer
            let ptr := mload(0x40)

            // (1) copy incoming call data
            calldatacopy(ptr, 0, calldatasize())

            // (2) forward call to logic contract
            let result := delegatecall(gas(), extensionImpl, ptr, calldatasize(), 0, 0)
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

    /*
     * STAKING
     */

    /// @inheritdoc IHorizonStakingMain
    function stake(uint256 tokens) external override notPaused {
        _stakeTo(msg.sender, tokens);
    }

    /// @inheritdoc IHorizonStakingMain
    function stakeTo(address serviceProvider, uint256 tokens) external override notPaused {
        _stakeTo(serviceProvider, tokens);
    }

    /// @inheritdoc IHorizonStakingMain
    function stakeToProvision(
        address serviceProvider,
        address verifier,
        uint256 tokens
    ) external override notPaused onlyAuthorizedOrVerifier(serviceProvider, verifier) {
        _stakeTo(serviceProvider, tokens);
        _addToProvision(serviceProvider, verifier, tokens);
    }

    /// @inheritdoc IHorizonStakingMain
    function unstake(uint256 tokens) external override notPaused {
        _unstake(tokens);
    }

    /// @inheritdoc IHorizonStakingMain
    function withdraw() external override notPaused {
        _withdraw(msg.sender);
    }

    /*
     * PROVISIONS
     */

    /// @inheritdoc IHorizonStakingMain
    function provision(
        address serviceProvider,
        address verifier,
        uint256 tokens,
        uint32 maxVerifierCut,
        uint64 thawingPeriod
    ) external override notPaused onlyAuthorized(serviceProvider, verifier) {
        _createProvision(serviceProvider, tokens, verifier, maxVerifierCut, thawingPeriod);
    }

    /// @inheritdoc IHorizonStakingMain
    function addToProvision(
        address serviceProvider,
        address verifier,
        uint256 tokens
    ) external override notPaused onlyAuthorized(serviceProvider, verifier) {
        _addToProvision(serviceProvider, verifier, tokens);
    }

    /// @inheritdoc IHorizonStakingMain
    function thaw(
        address serviceProvider,
        address verifier,
        uint256 tokens
    ) external override notPaused onlyAuthorized(serviceProvider, verifier) returns (bytes32) {
        return _thaw(serviceProvider, verifier, tokens);
    }

    /// @inheritdoc IHorizonStakingMain
    function deprovision(
        address serviceProvider,
        address verifier,
        uint256 nThawRequests
    ) external override onlyAuthorized(serviceProvider, verifier) notPaused {
        _deprovision(serviceProvider, verifier, nThawRequests);
    }

    /// @inheritdoc IHorizonStakingMain
    function reprovision(
        address serviceProvider,
        address oldVerifier,
        address newVerifier,
        uint256 nThawRequests
    )
        external
        override
        notPaused
        onlyAuthorized(serviceProvider, oldVerifier)
        onlyAuthorized(serviceProvider, newVerifier)
    {
        uint256 tokensThawed = _deprovision(serviceProvider, oldVerifier, nThawRequests);
        _addToProvision(serviceProvider, newVerifier, tokensThawed);
    }

    /// @inheritdoc IHorizonStakingMain
    function setProvisionParameters(
        address serviceProvider,
        address verifier,
        uint32 newMaxVerifierCut,
        uint64 newThawingPeriod
    ) external override notPaused onlyAuthorized(serviceProvider, verifier) {
        // Provision must exist
        Provision storage prov = _provisions[serviceProvider][verifier];
        require(prov.createdAt != 0, HorizonStakingInvalidProvision(serviceProvider, verifier));

        bool verifierCutChanged = prov.maxVerifierCutPending != newMaxVerifierCut;
        bool thawingPeriodChanged = prov.thawingPeriodPending != newThawingPeriod;

        if (verifierCutChanged || thawingPeriodChanged) {
            if (verifierCutChanged) {
                require(PPMMath.isValidPPM(newMaxVerifierCut), HorizonStakingInvalidMaxVerifierCut(newMaxVerifierCut));
                prov.maxVerifierCutPending = newMaxVerifierCut;
            }
            if (thawingPeriodChanged) {
                require(
                    newThawingPeriod <= _maxThawingPeriod,
                    HorizonStakingInvalidThawingPeriod(newThawingPeriod, _maxThawingPeriod)
                );
                prov.thawingPeriodPending = newThawingPeriod;
            }

            prov.lastParametersStagedAt = block.timestamp;
            emit ProvisionParametersStaged(serviceProvider, verifier, newMaxVerifierCut, newThawingPeriod);
        }
    }

    /// @inheritdoc IHorizonStakingMain
    function acceptProvisionParameters(address serviceProvider) external override notPaused {
        address verifier = msg.sender;

        // Provision must exist
        Provision storage prov = _provisions[serviceProvider][verifier];
        require(prov.createdAt != 0, HorizonStakingInvalidProvision(serviceProvider, verifier));

        if ((prov.maxVerifierCutPending != prov.maxVerifierCut) || (prov.thawingPeriodPending != prov.thawingPeriod)) {
            prov.maxVerifierCut = prov.maxVerifierCutPending;
            prov.thawingPeriod = prov.thawingPeriodPending;
            emit ProvisionParametersSet(serviceProvider, verifier, prov.maxVerifierCut, prov.thawingPeriod);
        }
    }

    /*
     * DELEGATION
     */

    /// @inheritdoc IHorizonStakingMain
    function delegate(
        address serviceProvider,
        address verifier,
        uint256 tokens,
        uint256 minSharesOut
    ) external override notPaused {
        require(tokens != 0, HorizonStakingInvalidZeroTokens());
        _graphToken().pullTokens(msg.sender, tokens);
        _delegate(serviceProvider, verifier, tokens, minSharesOut);
    }

    /// @inheritdoc IHorizonStakingMain
    function addToDelegationPool(
        address serviceProvider,
        address verifier,
        uint256 tokens
    ) external override notPaused {
        require(tokens != 0, HorizonStakingInvalidZeroTokens());

        // Provision must exist before adding to delegation pool
        Provision memory prov = _provisions[serviceProvider][verifier];
        require(prov.createdAt != 0, HorizonStakingInvalidProvision(serviceProvider, verifier));

        // Delegation pool must exist before adding tokens
        DelegationPoolInternal storage pool = _getDelegationPool(serviceProvider, verifier);
        require(pool.shares > 0, HorizonStakingInvalidDelegationPool(serviceProvider, verifier));

        pool.tokens = pool.tokens + tokens;
        _graphToken().pullTokens(msg.sender, tokens);
        emit TokensToDelegationPoolAdded(serviceProvider, verifier, tokens);
    }

    /// @inheritdoc IHorizonStakingMain
    function undelegate(
        address serviceProvider,
        address verifier,
        uint256 shares
    ) external override notPaused returns (bytes32) {
        return _undelegate(serviceProvider, verifier, shares);
    }

    /// @inheritdoc IHorizonStakingMain
    function withdrawDelegated(
        address serviceProvider,
        address verifier,
        uint256 nThawRequests
    ) external override notPaused {
        _withdrawDelegated(serviceProvider, verifier, address(0), address(0), 0, nThawRequests);
    }

    /// @inheritdoc IHorizonStakingMain
    function redelegate(
        address oldServiceProvider,
        address oldVerifier,
        address newServiceProvider,
        address newVerifier,
        uint256 minSharesForNewProvider,
        uint256 nThawRequests
    ) external override notPaused {
        require(newServiceProvider != address(0), HorizonStakingInvalidServiceProviderZeroAddress());
        require(newVerifier != address(0), HorizonStakingInvalidVerifierZeroAddress());
        _withdrawDelegated(
            oldServiceProvider,
            oldVerifier,
            newServiceProvider,
            newVerifier,
            minSharesForNewProvider,
            nThawRequests
        );
    }

    /// @inheritdoc IHorizonStakingMain
    function setDelegationFeeCut(
        address serviceProvider,
        address verifier,
        IGraphPayments.PaymentTypes paymentType,
        uint256 feeCut
    ) external override notPaused onlyAuthorized(serviceProvider, verifier) {
        require(PPMMath.isValidPPM(feeCut), HorizonStakingInvalidDelegationFeeCut(feeCut));
        _delegationFeeCut[serviceProvider][verifier][paymentType] = feeCut;
        emit DelegationFeeCutSet(serviceProvider, verifier, paymentType, feeCut);
    }

    /// @inheritdoc IHorizonStakingMain
    function delegate(address serviceProvider, uint256 tokens) external override notPaused {
        require(tokens != 0, HorizonStakingInvalidZeroTokens());
        _graphToken().pullTokens(msg.sender, tokens);
        _delegate(serviceProvider, SUBGRAPH_DATA_SERVICE_ADDRESS, tokens, 0);
    }

    /// @inheritdoc IHorizonStakingMain
    function undelegate(address serviceProvider, uint256 shares) external override notPaused {
        _undelegate(serviceProvider, SUBGRAPH_DATA_SERVICE_ADDRESS, shares);
    }

    /// @inheritdoc IHorizonStakingMain
    function withdrawDelegated(
        address serviceProvider,
        address // deprecated - kept for backwards compatibility
    ) external override notPaused returns (uint256) {
        // Get the delegation pool of the indexer
        address delegator = msg.sender;
        DelegationPoolInternal storage pool = _legacyDelegationPools[serviceProvider];
        DelegationInternal storage delegation = pool.delegators[delegator];

        // Validation
        uint256 tokensToWithdraw = 0;
        uint256 currentEpoch = _graphEpochManager().currentEpoch();
        if (
            delegation.__DEPRECATED_tokensLockedUntil > 0 && currentEpoch >= delegation.__DEPRECATED_tokensLockedUntil
        ) {
            tokensToWithdraw = delegation.__DEPRECATED_tokensLocked;
        }
        require(tokensToWithdraw > 0, HorizonStakingNothingToWithdraw());

        // Reset lock
        delegation.__DEPRECATED_tokensLocked = 0;
        delegation.__DEPRECATED_tokensLockedUntil = 0;

        emit StakeDelegatedWithdrawn(serviceProvider, delegator, tokensToWithdraw);

        // -- Interactions --

        // Return tokens to the delegator
        _graphToken().pushTokens(delegator, tokensToWithdraw);

        return tokensToWithdraw;
    }

    /*
     * SLASHING
     */

    /// @inheritdoc IHorizonStakingMain
    function slash(
        address serviceProvider,
        uint256 tokens,
        uint256 tokensVerifier,
        address verifierDestination
    ) external override notPaused {
        // TRANSITION PERIOD: remove after the transition period
        // Check if sender is authorized to slash on the deprecated list
        if (__DEPRECATED_slashers[msg.sender]) {
            // Forward call to staking extension
            // solhint-disable-next-line avoid-low-level-calls
            (bool success, ) = STAKING_EXTENSION_ADDRESS.delegatecall(
                abi.encodeCall(
                    IHorizonStakingExtension.legacySlash,
                    (serviceProvider, tokens, tokensVerifier, verifierDestination)
                )
            );
            require(success, HorizonStakingLegacySlashFailed());
            return;
        }

        address verifier = msg.sender;
        Provision storage prov = _provisions[serviceProvider][verifier];
        DelegationPoolInternal storage pool = _getDelegationPool(serviceProvider, verifier);
        uint256 tokensProvisionTotal = prov.tokens + pool.tokens;
        require(tokensProvisionTotal != 0, HorizonStakingNoTokensToSlash());

        uint256 tokensToSlash = MathUtils.min(tokens, tokensProvisionTotal);

        // Slash service provider first
        // - A portion goes to verifier as reward
        // - A portion gets burned
        uint256 providerTokensSlashed = MathUtils.min(prov.tokens, tokensToSlash);
        if (providerTokensSlashed > 0) {
            // Pay verifier reward - must be within the maxVerifierCut percentage
            uint256 maxVerifierTokens = providerTokensSlashed.mulPPM(prov.maxVerifierCut);
            require(
                maxVerifierTokens >= tokensVerifier,
                HorizonStakingTooManyTokens(tokensVerifier, maxVerifierTokens)
            );
            if (tokensVerifier > 0) {
                _graphToken().pushTokens(verifierDestination, tokensVerifier);
                emit VerifierTokensSent(serviceProvider, verifier, verifierDestination, tokensVerifier);
            }

            // Burn remainder
            _graphToken().burnTokens(providerTokensSlashed - tokensVerifier);

            // Provision accounting - round down, 1 wei max precision loss
            prov.tokensThawing = (prov.tokensThawing * (prov.tokens - providerTokensSlashed)) / prov.tokens;
            prov.tokens = prov.tokens - providerTokensSlashed;

            // If the slashing leaves the thawing shares with no thawing tokens, cancel pending thawings by:
            // - deleting all thawing shares
            // - incrementing the nonce to invalidate pending thaw requests
            if (prov.sharesThawing != 0 && prov.tokensThawing == 0) {
                prov.sharesThawing = 0;
                prov.thawingNonce++;
            }

            // Service provider accounting
            _serviceProviders[serviceProvider].tokensProvisioned =
                _serviceProviders[serviceProvider].tokensProvisioned - providerTokensSlashed;
            _serviceProviders[serviceProvider].tokensStaked =
                _serviceProviders[serviceProvider].tokensStaked - providerTokensSlashed;

            emit ProvisionSlashed(serviceProvider, verifier, providerTokensSlashed);
        }

        // Slash delegators if needed
        // - Slashed delegation is entirely burned
        // Since tokensToSlash is already limited above, this subtraction will remain within pool.tokens.
        tokensToSlash = tokensToSlash - providerTokensSlashed;
        if (tokensToSlash > 0) {
            if (_delegationSlashingEnabled) {
                // Burn tokens
                _graphToken().burnTokens(tokensToSlash);

                // Delegation pool accounting - round down, 1 wei max precision loss
                pool.tokensThawing = (pool.tokensThawing * (pool.tokens - tokensToSlash)) / pool.tokens;
                pool.tokens = pool.tokens - tokensToSlash;

                // If the slashing leaves the thawing shares with no thawing tokens, cancel pending thawings by:
                // - deleting all thawing shares
                // - incrementing the nonce to invalidate pending thaw requests
                // Note that thawing shares are completely lost, delegators won't get back the corresponding
                // delegation pool shares.
                if (pool.sharesThawing != 0 && pool.tokensThawing == 0) {
                    pool.sharesThawing = 0;
                    pool.thawingNonce++;
                }

                emit DelegationSlashed(serviceProvider, verifier, tokensToSlash);
            } else {
                emit DelegationSlashingSkipped(serviceProvider, verifier, tokensToSlash);
            }
        }
    }

    /*
     * LOCKED VERIFIERS
     */

    /// @inheritdoc IHorizonStakingMain
    function provisionLocked(
        address serviceProvider,
        address verifier,
        uint256 tokens,
        uint32 maxVerifierCut,
        uint64 thawingPeriod
    ) external override notPaused onlyAuthorized(serviceProvider, verifier) {
        require(_allowedLockedVerifiers[verifier], HorizonStakingVerifierNotAllowed(verifier));
        _createProvision(serviceProvider, tokens, verifier, maxVerifierCut, thawingPeriod);
    }

    /// @inheritdoc IHorizonStakingMain
    function setOperatorLocked(address verifier, address operator, bool allowed) external override notPaused {
        require(_allowedLockedVerifiers[verifier], HorizonStakingVerifierNotAllowed(verifier));
        _setOperator(verifier, operator, allowed);
    }

    /*
     * GOVERNANCE
     */

    /// @inheritdoc IHorizonStakingMain
    function setAllowedLockedVerifier(address verifier, bool allowed) external override onlyGovernor {
        _allowedLockedVerifiers[verifier] = allowed;
        emit AllowedLockedVerifierSet(verifier, allowed);
    }

    /// @inheritdoc IHorizonStakingMain
    function setDelegationSlashingEnabled() external override onlyGovernor {
        _delegationSlashingEnabled = true;
        emit DelegationSlashingEnabled();
    }

    /// @inheritdoc IHorizonStakingMain
    function clearThawingPeriod() external override onlyGovernor {
        __DEPRECATED_thawingPeriod = 0;
        emit ThawingPeriodCleared();
    }

    /// @inheritdoc IHorizonStakingMain
    function setMaxThawingPeriod(uint64 maxThawingPeriod) external override onlyGovernor {
        _maxThawingPeriod = maxThawingPeriod;
        emit MaxThawingPeriodSet(_maxThawingPeriod);
    }

    /*
     * OPERATOR
     */

    /// @inheritdoc IHorizonStakingMain
    function setOperator(address verifier, address operator, bool allowed) external override notPaused {
        _setOperator(verifier, operator, allowed);
    }

    /// @inheritdoc IHorizonStakingMain
    function isAuthorized(
        address serviceProvider,
        address verifier,
        address operator
    ) external view override returns (bool) {
        return _isAuthorized(serviceProvider, verifier, operator);
    }

    /*
     * GETTERS
     */

    /// @inheritdoc IHorizonStakingMain
    function getStakingExtension() external view override returns (address) {
        return STAKING_EXTENSION_ADDRESS;
    }

    /*
     * PRIVATE FUNCTIONS
     */

    /**
     * @notice Deposit tokens on the service provider stake, on behalf of the service provider.
     * @dev Pulls tokens from the caller.
     * @param _serviceProvider Address of the service provider
     * @param _tokens Amount of tokens to stake
     */
    function _stakeTo(address _serviceProvider, uint256 _tokens) private {
        require(_tokens != 0, HorizonStakingInvalidZeroTokens());

        // Transfer tokens to stake from caller to this contract
        _graphToken().pullTokens(msg.sender, _tokens);

        // Stake the transferred tokens
        _stake(_serviceProvider, _tokens);
    }

    /**
     * @notice Move idle stake back to the owner's account.
     * Stake is removed from the protocol:
     * - During the transition period it's locked for a period of time before it can be withdrawn
     *   by calling {withdraw}.
     * - After the transition period it's immediately withdrawn.
     * Note that after the transition period if there are tokens still locked they will have to be
     * withdrawn by calling {withdraw}.
     * @param _tokens Amount of tokens to unstake
     */
    function _unstake(uint256 _tokens) private {
        address serviceProvider = msg.sender;
        require(_tokens != 0, HorizonStakingInvalidZeroTokens());
        uint256 tokensIdle = _getIdleStake(serviceProvider);
        require(_tokens <= tokensIdle, HorizonStakingInsufficientIdleStake(_tokens, tokensIdle));

        ServiceProviderInternal storage sp = _serviceProviders[serviceProvider];
        uint256 stakedTokens = sp.tokensStaked;

        // This is also only during the transition period: we need
        // to ensure tokens stay locked after closing legacy allocations.
        // After sufficient time (56 days?) we should remove the closeAllocation function
        // and set the thawing period to 0.
        uint256 lockingPeriod = __DEPRECATED_thawingPeriod;
        if (lockingPeriod == 0) {
            sp.tokensStaked = stakedTokens - _tokens;
            _graphToken().pushTokens(serviceProvider, _tokens);
            emit HorizonStakeWithdrawn(serviceProvider, _tokens);
        } else {
            // Before locking more tokens, withdraw any unlocked ones if possible
            if (sp.__DEPRECATED_tokensLocked != 0 && block.number >= sp.__DEPRECATED_tokensLockedUntil) {
                _withdraw(serviceProvider);
            }
            // TRANSITION PERIOD: remove after the transition period
            // Take into account period averaging for multiple unstake requests
            if (sp.__DEPRECATED_tokensLocked > 0) {
                lockingPeriod = MathUtils.weightedAverageRoundingUp(
                    MathUtils.diffOrZero(sp.__DEPRECATED_tokensLockedUntil, block.number), // Remaining thawing period
                    sp.__DEPRECATED_tokensLocked, // Weighted by remaining unstaked tokens
                    lockingPeriod, // Thawing period
                    _tokens // Weighted by new tokens to unstake
                );
            }

            // Update balances
            sp.__DEPRECATED_tokensLocked = sp.__DEPRECATED_tokensLocked + _tokens;
            sp.__DEPRECATED_tokensLockedUntil = block.number + lockingPeriod;
            emit HorizonStakeLocked(serviceProvider, sp.__DEPRECATED_tokensLocked, sp.__DEPRECATED_tokensLockedUntil);
        }
    }

    /**
     * @notice Withdraw service provider tokens once the thawing period (initiated by {unstake}) has passed.
     * All thawed tokens are withdrawn.
     * @dev TRANSITION PERIOD: This is only needed during the transition period while we still have
     * a global lock. After that, unstake() will automatically withdraw.
     * @param _serviceProvider Address of service provider to withdraw funds from
     */
    function _withdraw(address _serviceProvider) private {
        // Get tokens available for withdraw and update balance
        ServiceProviderInternal storage sp = _serviceProviders[_serviceProvider];
        uint256 tokensToWithdraw = sp.__DEPRECATED_tokensLocked;
        require(tokensToWithdraw != 0, HorizonStakingInvalidZeroTokens());
        require(
            block.number >= sp.__DEPRECATED_tokensLockedUntil,
            HorizonStakingStillThawing(sp.__DEPRECATED_tokensLockedUntil)
        );

        // Reset locked tokens
        sp.__DEPRECATED_tokensLocked = 0;
        sp.__DEPRECATED_tokensLockedUntil = 0;

        sp.tokensStaked = sp.tokensStaked - tokensToWithdraw;

        // Return tokens to the service provider
        _graphToken().pushTokens(_serviceProvider, tokensToWithdraw);

        emit HorizonStakeWithdrawn(_serviceProvider, tokensToWithdraw);
    }

    /**
     * @notice Provision stake to a verifier. The tokens will be locked with a thawing period
     * and will be slashable by the verifier. This is the main mechanism to provision stake to a data
     * service, where the data service is the verifier.
     * This function can be called by the service provider or by an operator authorized by the provider
     * for this specific verifier.
     * @dev TRANSITION PERIOD: During the transition period, only the subgraph data service can be used as a verifier. This
     * prevents an escape hatch for legacy allocation stake.
     * @param _serviceProvider The service provider address
     * @param _tokens The amount of tokens that will be locked and slashable
     * @param _verifier The verifier address for which the tokens are provisioned (who will be able to slash the tokens)
     * @param _maxVerifierCut The maximum cut, expressed in PPM, that a verifier can transfer instead of burning when slashing
     * @param _thawingPeriod The period in seconds that the tokens will be thawing before they can be removed from the provision
     */
    function _createProvision(
        address _serviceProvider,
        uint256 _tokens,
        address _verifier,
        uint32 _maxVerifierCut,
        uint64 _thawingPeriod
    ) private {
        require(_tokens > 0, HorizonStakingInvalidZeroTokens());
        // TRANSITION PERIOD: Remove this after the transition period - it prevents an early escape hatch for legacy allocations
        require(
            _verifier == SUBGRAPH_DATA_SERVICE_ADDRESS || __DEPRECATED_thawingPeriod == 0,
            HorizonStakingInvalidVerifier(_verifier)
        );
        require(PPMMath.isValidPPM(_maxVerifierCut), HorizonStakingInvalidMaxVerifierCut(_maxVerifierCut));
        require(
            _thawingPeriod <= _maxThawingPeriod,
            HorizonStakingInvalidThawingPeriod(_thawingPeriod, _maxThawingPeriod)
        );
        require(_provisions[_serviceProvider][_verifier].createdAt == 0, HorizonStakingProvisionAlreadyExists());
        uint256 tokensIdle = _getIdleStake(_serviceProvider);
        require(_tokens <= tokensIdle, HorizonStakingInsufficientIdleStake(_tokens, tokensIdle));

        _provisions[_serviceProvider][_verifier] = Provision({
            tokens: _tokens,
            tokensThawing: 0,
            sharesThawing: 0,
            maxVerifierCut: _maxVerifierCut,
            thawingPeriod: _thawingPeriod,
            createdAt: uint64(block.timestamp),
            maxVerifierCutPending: _maxVerifierCut,
            thawingPeriodPending: _thawingPeriod,
            lastParametersStagedAt: 0,
            thawingNonce: 0
        });

        ServiceProviderInternal storage sp = _serviceProviders[_serviceProvider];
        sp.tokensProvisioned = sp.tokensProvisioned + _tokens;

        emit ProvisionCreated(_serviceProvider, _verifier, _tokens, _maxVerifierCut, _thawingPeriod);
    }

    /**
     * @notice Adds tokens from the service provider's idle stake to a provision
     * @param _serviceProvider The service provider address
     * @param _verifier The verifier address
     * @param _tokens The amount of tokens to add to the provision
     */
    function _addToProvision(address _serviceProvider, address _verifier, uint256 _tokens) private {
        require(_tokens != 0, HorizonStakingInvalidZeroTokens());

        Provision storage prov = _provisions[_serviceProvider][_verifier];
        require(prov.createdAt != 0, HorizonStakingInvalidProvision(_serviceProvider, _verifier));
        uint256 tokensIdle = _getIdleStake(_serviceProvider);
        require(_tokens <= tokensIdle, HorizonStakingInsufficientIdleStake(_tokens, tokensIdle));

        prov.tokens = prov.tokens + _tokens;
        _serviceProviders[_serviceProvider].tokensProvisioned =
            _serviceProviders[_serviceProvider].tokensProvisioned + _tokens;
        emit ProvisionIncreased(_serviceProvider, _verifier, _tokens);
    }

    /**
     * @notice Start thawing tokens to remove them from a provision.
     * This function can be called by the service provider or by an operator authorized by the provider
     * for this specific verifier.
     *
     * Note that removing tokens from a provision is a two step process:
     * - First the tokens are thawed using this function.
     * - Then after the thawing period, the tokens are removed from the provision using {deprovision}
     *   or {reprovision}.
     *
     * @dev We use a thawing pool to keep track of tokens thawing for multiple thaw requests.
     * If due to slashing the thawing pool loses all of its tokens, the pool is reset and all pending thaw
     * requests are invalidated.
     *
     * @param _serviceProvider The service provider address
     * @param _verifier The verifier address for which the tokens are provisioned
     * @param _tokens The amount of tokens to thaw
     * @return The ID of the thaw request
     */
    function _thaw(address _serviceProvider, address _verifier, uint256 _tokens) private returns (bytes32) {
        require(_tokens != 0, HorizonStakingInvalidZeroTokens());
        uint256 tokensAvailable = _getProviderTokensAvailable(_serviceProvider, _verifier);
        require(tokensAvailable >= _tokens, HorizonStakingInsufficientTokens(tokensAvailable, _tokens));

        Provision storage prov = _provisions[_serviceProvider][_verifier];

        // Calculate shares to issue
        // Thawing pool is reset/initialized when the pool is empty: prov.tokensThawing == 0
        // Round thawing shares up to ensure fairness and avoid undervaluing the shares due to rounding down.
        uint256 thawingShares = prov.tokensThawing == 0
            ? _tokens
            : ((prov.sharesThawing * _tokens + prov.tokensThawing - 1) / prov.tokensThawing);
        uint64 thawingUntil = uint64(block.timestamp + uint256(prov.thawingPeriod));

        prov.sharesThawing = prov.sharesThawing + thawingShares;
        prov.tokensThawing = prov.tokensThawing + _tokens;

        bytes32 thawRequestId = _createThawRequest(
            ThawRequestType.Provision,
            _serviceProvider,
            _verifier,
            _serviceProvider,
            thawingShares,
            thawingUntil,
            prov.thawingNonce
        );
        emit ProvisionThawed(_serviceProvider, _verifier, _tokens);
        return thawRequestId;
    }

    /**
     * @notice Remove tokens from a provision and move them back to the service provider's idle stake.
     * @dev The parameter `nThawRequests` can be set to a non zero value to fulfill a specific number of thaw
     * requests in the event that fulfilling all of them results in a gas limit error. Otherwise, the function
     * will attempt to fulfill all thaw requests until the first one that is not yet expired is found.
     * @param _serviceProvider The service provider address
     * @param _verifier The verifier address
     * @param _nThawRequests The number of thaw requests to fulfill. Set to 0 to fulfill all thaw requests.
     * @return The amount of tokens that were removed from the provision
     */
    function _deprovision(
        address _serviceProvider,
        address _verifier,
        uint256 _nThawRequests
    ) private returns (uint256) {
        Provision storage prov = _provisions[_serviceProvider][_verifier];

        uint256 tokensThawed_ = 0;
        uint256 sharesThawing = prov.sharesThawing;
        uint256 tokensThawing = prov.tokensThawing;

        FulfillThawRequestsParams memory params = FulfillThawRequestsParams({
            requestType: ThawRequestType.Provision,
            serviceProvider: _serviceProvider,
            verifier: _verifier,
            owner: _serviceProvider,
            tokensThawing: tokensThawing,
            sharesThawing: sharesThawing,
            nThawRequests: _nThawRequests,
            thawingNonce: prov.thawingNonce
        });
        (tokensThawed_, tokensThawing, sharesThawing) = _fulfillThawRequests(params);

        prov.tokens = prov.tokens - tokensThawed_;
        prov.sharesThawing = sharesThawing;
        prov.tokensThawing = tokensThawing;
        _serviceProviders[_serviceProvider].tokensProvisioned -= tokensThawed_;

        emit TokensDeprovisioned(_serviceProvider, _verifier, tokensThawed_);
        return tokensThawed_;
    }

    /**
     * @notice Delegate tokens to a provision.
     * @dev Note that this function does not pull the delegated tokens from the caller. It expects that to
     * have been done before calling this function.
     * @param _serviceProvider The service provider address
     * @param _verifier The verifier address
     * @param _tokens The amount of tokens to delegate
     * @param _minSharesOut The minimum amount of shares to accept, slippage protection.
     */
    function _delegate(address _serviceProvider, address _verifier, uint256 _tokens, uint256 _minSharesOut) private {
        // Enforces a minimum delegation amount to prevent share manipulation attacks.
        // This stops attackers from inflating share value and blocking other delegators.
        require(_tokens >= MIN_DELEGATION, HorizonStakingInsufficientDelegationTokens(_tokens, MIN_DELEGATION));
        require(
            _provisions[_serviceProvider][_verifier].createdAt != 0,
            HorizonStakingInvalidProvision(_serviceProvider, _verifier)
        );

        DelegationPoolInternal storage pool = _getDelegationPool(_serviceProvider, _verifier);
        DelegationInternal storage delegation = pool.delegators[msg.sender];

        // An invalid delegation pool has shares but no tokens
        require(
            pool.tokens != 0 || pool.shares == 0,
            HorizonStakingInvalidDelegationPoolState(_serviceProvider, _verifier)
        );

        // Calculate shares to issue
        // Delegation pool is reset/initialized in any of the following cases:
        // - pool.tokens == 0 and pool.shares == 0, pool is completely empty. Note that we don't test shares == 0 because
        //   the invalid delegation pool check already ensures shares are 0 if tokens are 0
        // - pool.tokens == pool.tokensThawing, the entire pool is thawing
        bool initializePool = pool.tokens == 0 || pool.tokens == pool.tokensThawing;
        uint256 shares = initializePool ? _tokens : ((_tokens * pool.shares) / (pool.tokens - pool.tokensThawing));
        require(shares != 0 && shares >= _minSharesOut, HorizonStakingSlippageProtection(shares, _minSharesOut));

        pool.tokens = pool.tokens + _tokens;
        pool.shares = pool.shares + shares;

        delegation.shares = delegation.shares + shares;

        emit TokensDelegated(_serviceProvider, _verifier, msg.sender, _tokens, shares);
    }

    /**
     * @notice Undelegate tokens from a provision and start thawing them.
     * Note that undelegating tokens from a provision is a two step process:
     * - First the tokens are thawed using this function.
     * - Then after the thawing period, the tokens are removed from the provision using {withdrawDelegated}.
     * @dev To allow delegation to be slashable even while thawing without breaking accounting
     * the delegation pool shares are burned and replaced with thawing pool shares.
     * @dev Note that due to slashing the delegation pool can enter an invalid state if all it's tokens are slashed.
     * An invalid pool can only be recovered by adding back tokens into the pool with {IHorizonStakingMain-addToDelegationPool}.
     * Any time the delegation pool is invalidated, the thawing pool is also reset and any pending undelegate requests get
     * invalidated.
     * @dev Note that delegation that is caught thawing when the pool is invalidated will be completely lost! However delegation shares
     * that were not thawing will be preserved.
     * @param _serviceProvider The service provider address
     * @param _verifier The verifier address
     * @param _shares The amount of shares to undelegate
     * @return The ID of the thaw request
     */
    function _undelegate(address _serviceProvider, address _verifier, uint256 _shares) private returns (bytes32) {
        require(_shares > 0, HorizonStakingInvalidZeroShares());
        DelegationPoolInternal storage pool = _getDelegationPool(_serviceProvider, _verifier);
        DelegationInternal storage delegation = pool.delegators[msg.sender];
        require(delegation.shares >= _shares, HorizonStakingInsufficientShares(delegation.shares, _shares));

        // An invalid delegation pool has shares but no tokens (previous require check ensures shares > 0)
        require(pool.tokens != 0, HorizonStakingInvalidDelegationPoolState(_serviceProvider, _verifier));

        // Calculate thawing shares to issue - convert delegation pool shares to thawing pool shares
        // delegation pool shares -> delegation pool tokens -> thawing pool shares
        // Thawing pool is reset/initialized when the pool is empty: prov.tokensThawing == 0
        uint256 tokens = (_shares * (pool.tokens - pool.tokensThawing)) / pool.shares;

        // Thawing shares are rounded down to protect the pool and avoid taking extra tokens from other participants.
        uint256 thawingShares = pool.tokensThawing == 0 ? tokens : ((tokens * pool.sharesThawing) / pool.tokensThawing);
        uint64 thawingUntil = uint64(block.timestamp + uint256(_provisions[_serviceProvider][_verifier].thawingPeriod));

        pool.tokensThawing = pool.tokensThawing + tokens;
        pool.sharesThawing = pool.sharesThawing + thawingShares;

        pool.shares = pool.shares - _shares;
        delegation.shares = delegation.shares - _shares;
        if (delegation.shares != 0) {
            uint256 remainingTokens = (delegation.shares * (pool.tokens - pool.tokensThawing)) / pool.shares;
            require(
                remainingTokens >= MIN_DELEGATION,
                HorizonStakingInsufficientTokens(remainingTokens, MIN_DELEGATION)
            );
        }

        bytes32 thawRequestId = _createThawRequest(
            ThawRequestType.Delegation,
            _serviceProvider,
            _verifier,
            msg.sender,
            thawingShares,
            thawingUntil,
            pool.thawingNonce
        );

        emit TokensUndelegated(_serviceProvider, _verifier, msg.sender, tokens, _shares);
        return thawRequestId;
    }

    /**
     * @notice Withdraw undelegated tokens from a provision after thawing.
     * @dev The parameter `nThawRequests` can be set to a non zero value to fulfill a specific number of thaw
     * requests in the event that fulfilling all of them results in a gas limit error. Otherwise, the function
     * will attempt to fulfill all thaw requests until the first one that is not yet expired is found.
     * @dev If the delegation pool was completely slashed before withdrawing, calling this function will fulfill
     * the thaw requests with an amount equal to zero.
     * @param _serviceProvider The service provider address
     * @param _verifier The verifier address
     * @param _newServiceProvider The new service provider address
     * @param _newVerifier The new verifier address
     * @param _minSharesForNewProvider The minimum number of shares for the new service provider
     * @param _nThawRequests The number of thaw requests to fulfill. Set to 0 to fulfill all thaw requests.
     */
    function _withdrawDelegated(
        address _serviceProvider,
        address _verifier,
        address _newServiceProvider,
        address _newVerifier,
        uint256 _minSharesForNewProvider,
        uint256 _nThawRequests
    ) private {
        DelegationPoolInternal storage pool = _getDelegationPool(_serviceProvider, _verifier);

        // An invalid delegation pool has shares but no tokens
        require(
            pool.tokens != 0 || pool.shares == 0,
            HorizonStakingInvalidDelegationPoolState(_serviceProvider, _verifier)
        );

        uint256 tokensThawed = 0;
        uint256 sharesThawing = pool.sharesThawing;
        uint256 tokensThawing = pool.tokensThawing;

        FulfillThawRequestsParams memory params = FulfillThawRequestsParams({
            requestType: ThawRequestType.Delegation,
            serviceProvider: _serviceProvider,
            verifier: _verifier,
            owner: msg.sender,
            tokensThawing: tokensThawing,
            sharesThawing: sharesThawing,
            nThawRequests: _nThawRequests,
            thawingNonce: pool.thawingNonce
        });
        (tokensThawed, tokensThawing, sharesThawing) = _fulfillThawRequests(params);

        // The next subtraction should never revert becase: pool.tokens >= pool.tokensThawing and pool.tokensThawing >= tokensThawed
        // In the event the pool gets completely slashed tokensThawed will fulfil to 0.
        pool.tokens = pool.tokens - tokensThawed;
        pool.sharesThawing = sharesThawing;
        pool.tokensThawing = tokensThawing;

        if (tokensThawed != 0) {
            if (_newServiceProvider != address(0) && _newVerifier != address(0)) {
                _delegate(_newServiceProvider, _newVerifier, tokensThawed, _minSharesForNewProvider);
            } else {
                _graphToken().pushTokens(msg.sender, tokensThawed);
                emit DelegatedTokensWithdrawn(_serviceProvider, _verifier, msg.sender, tokensThawed);
            }
        }
    }

    /**
     * @notice Creates a thaw request.
     * Allows creating thaw requests up to a maximum of `MAX_THAW_REQUESTS` per owner.
     * Thaw requests are stored in a linked list per owner (and service provider, verifier) to allow for efficient
     * processing.
     * @param _requestType The type of thaw request.
     * @param _serviceProvider The address of the service provider
     * @param _verifier The address of the verifier
     * @param _owner The address of the owner of the thaw request
     * @param _shares The number of shares to thaw
     * @param _thawingUntil The timestamp until which the shares are thawing
     * @param _thawingNonce Owner's validity nonce for the thaw request
     * @return The ID of the thaw request
     */
    function _createThawRequest(
        ThawRequestType _requestType,
        address _serviceProvider,
        address _verifier,
        address _owner,
        uint256 _shares,
        uint64 _thawingUntil,
        uint256 _thawingNonce
    ) private returns (bytes32) {
        require(_shares != 0, HorizonStakingInvalidZeroShares());
        ILinkedList.List storage thawRequestList = _getThawRequestList(
            _requestType,
            _serviceProvider,
            _verifier,
            _owner
        );
        require(thawRequestList.count < MAX_THAW_REQUESTS, HorizonStakingTooManyThawRequests());

        // forge-lint: disable-next-item(asm-keccak256)
        bytes32 thawRequestId = keccak256(abi.encodePacked(_serviceProvider, _verifier, _owner, thawRequestList.nonce));
        ThawRequest storage thawRequest = _getThawRequest(_requestType, thawRequestId);
        thawRequest.shares = _shares;
        thawRequest.thawingUntil = _thawingUntil;
        thawRequest.nextRequest = bytes32(0);
        thawRequest.thawingNonce = _thawingNonce;

        if (thawRequestList.count != 0) _getThawRequest(_requestType, thawRequestList.tail).nextRequest = thawRequestId;
        thawRequestList.addTail(thawRequestId);

        emit ThawRequestCreated(
            _requestType,
            _serviceProvider,
            _verifier,
            _owner,
            _shares,
            _thawingUntil,
            thawRequestId,
            _thawingNonce
        );
        return thawRequestId;
    }

    /**
     * @notice Traverses a thaw request list and fulfills expired thaw requests.
     * @dev Note that the list is traversed by creation date not by thawing until date. Traversing will stop
     * when the first thaw request that is not yet expired is found even if later thaw requests have expired. This
     * could happen for example when the thawing period is shortened.
     * @param _params The parameters for fulfilling thaw requests
     * @return The amount of thawed tokens
     * @return The amount of tokens still thawing
     * @return The amount of shares still thawing
     */
    function _fulfillThawRequests(
        FulfillThawRequestsParams memory _params
    ) private returns (uint256, uint256, uint256) {
        ILinkedList.List storage thawRequestList = _getThawRequestList(
            _params.requestType,
            _params.serviceProvider,
            _params.verifier,
            _params.owner
        );
        require(thawRequestList.count > 0, HorizonStakingNothingThawing());

        TraverseThawRequestsResults memory results = _traverseThawRequests(_params, thawRequestList);

        emit ThawRequestsFulfilled(
            _params.requestType,
            _params.serviceProvider,
            _params.verifier,
            _params.owner,
            results.requestsFulfilled,
            results.tokensThawed
        );

        return (results.tokensThawed, results.tokensThawing, results.sharesThawing);
    }

    /**
     * @notice Traverses a thaw request list and fulfills expired thaw requests.
     * @param _params The parameters for fulfilling thaw requests
     * @param _thawRequestList The list of thaw requests to traverse
     * @return The results of the traversal
     */
    function _traverseThawRequests(
        FulfillThawRequestsParams memory _params,
        ILinkedList.List storage _thawRequestList
    ) private returns (TraverseThawRequestsResults memory) {
        function(bytes32) view returns (bytes32) getNextItem = _getNextThawRequest(_params.requestType);
        function(bytes32) deleteItem = _getDeleteThawRequest(_params.requestType);

        bytes memory acc = abi.encode(
            _params.requestType,
            uint256(0),
            _params.tokensThawing,
            _params.sharesThawing,
            _params.thawingNonce
        );
        (uint256 thawRequestsFulfilled, bytes memory data) = _thawRequestList.traverse(
            getNextItem,
            _fulfillThawRequest,
            deleteItem,
            acc,
            _params.nThawRequests
        );

        (, uint256 tokensThawed, uint256 tokensThawing, uint256 sharesThawing) = abi.decode(
            data,
            (ThawRequestType, uint256, uint256, uint256)
        );

        return
            TraverseThawRequestsResults({
                requestsFulfilled: thawRequestsFulfilled,
                tokensThawed: tokensThawed,
                tokensThawing: tokensThawing,
                sharesThawing: sharesThawing
            });
    }

    /**
     * @notice Fulfills a thaw request.
     * @dev This function is used as a callback in the thaw requests linked list traversal.
     * @param _thawRequestId The ID of the current thaw request
     * @param _acc The accumulator data for the thaw requests being fulfilled
     * @return Whether the thaw request is still thawing, indicating that the traversal should continue or stop.
     * @return The updated accumulator data
     */
    function _fulfillThawRequest(bytes32 _thawRequestId, bytes memory _acc) private returns (bool, bytes memory) {
        // decode
        (
            ThawRequestType requestType,
            uint256 tokensThawed,
            uint256 tokensThawing,
            uint256 sharesThawing,
            uint256 thawingNonce
        ) = abi.decode(_acc, (ThawRequestType, uint256, uint256, uint256, uint256));

        ThawRequest storage thawRequest = _getThawRequest(requestType, _thawRequestId);

        // early exit
        if (thawRequest.thawingUntil > block.timestamp) {
            return (true, LinkedList.NULL_BYTES);
        }

        // process - only fulfill thaw requests for the current valid nonce
        uint256 tokens = 0;
        bool validThawRequest = thawRequest.thawingNonce == thawingNonce;
        if (validThawRequest) {
            // sharesThawing cannot be zero if there is a valid thaw request so the next division is safe
            tokens = (thawRequest.shares * tokensThawing) / sharesThawing;
            tokensThawing = tokensThawing - tokens;
            sharesThawing = sharesThawing - thawRequest.shares;
            tokensThawed = tokensThawed + tokens;
        }
        emit ThawRequestFulfilled(
            requestType,
            _thawRequestId,
            tokens,
            thawRequest.shares,
            thawRequest.thawingUntil,
            validThawRequest
        );

        // encode
        _acc = abi.encode(requestType, tokensThawed, tokensThawing, sharesThawing, thawingNonce);
        return (false, _acc);
    }

    /**
     * @notice Deletes a thaw request for a provision.
     * @param _thawRequestId The ID of the thaw request to delete.
     */
    function _deleteProvisionThawRequest(bytes32 _thawRequestId) private {
        delete _thawRequests[ThawRequestType.Provision][_thawRequestId];
    }

    /**
     * @notice Deletes a thaw request for a delegation.
     * @param _thawRequestId The ID of the thaw request to delete.
     */
    function _deleteDelegationThawRequest(bytes32 _thawRequestId) private {
        delete _thawRequests[ThawRequestType.Delegation][_thawRequestId];
    }

    /**
     * @notice Authorize or unauthorize an address to be an operator for the caller on a data service.
     * @dev Note that this function handles the special case where the verifier is the subgraph data service,
     * where the operator settings are stored in the legacy mapping.
     * @param _verifier The verifier / data service on which they'll be allowed to operate
     * @param _operator Address to authorize or unauthorize
     * @param _allowed Whether the operator is authorized or not
     */
    function _setOperator(address _verifier, address _operator, bool _allowed) private {
        require(_operator != msg.sender, HorizonStakingCallerIsServiceProvider());
        if (_verifier == SUBGRAPH_DATA_SERVICE_ADDRESS) {
            _legacyOperatorAuth[msg.sender][_operator] = _allowed;
        } else {
            _operatorAuth[msg.sender][_verifier][_operator] = _allowed;
        }
        emit OperatorSet(msg.sender, _verifier, _operator, _allowed);
    }

    /**
     * @notice Check if an operator is authorized for the caller on a specific verifier / data service.
     * @dev Note that this function handles the special case where the verifier is the subgraph data service,
     * where the operator settings are stored in the legacy mapping.
     * @param _serviceProvider The service provider on behalf of whom they're claiming to act
     * @param _verifier The verifier / data service on which they're claiming to act
     * @param _operator The address to check for auth
     * @return Whether the operator is authorized or not
     */
    function _isAuthorized(address _serviceProvider, address _verifier, address _operator) private view returns (bool) {
        if (_operator == _serviceProvider) {
            return true;
        }
        if (_verifier == SUBGRAPH_DATA_SERVICE_ADDRESS) {
            return _legacyOperatorAuth[_serviceProvider][_operator];
        } else {
            return _operatorAuth[_serviceProvider][_verifier][_operator];
        }
    }

    /**
     * @notice Determines the correct callback function for `deleteItem` based on the request type.
     * @param _requestType The type of thaw request (Provision or Delegation).
     * @return A function pointer to the appropriate `deleteItem` callback.
     */
    function _getDeleteThawRequest(ThawRequestType _requestType) private pure returns (function(bytes32)) {
        if (_requestType == ThawRequestType.Provision) {
            return _deleteProvisionThawRequest;
        } else if (_requestType == ThawRequestType.Delegation) {
            return _deleteDelegationThawRequest;
        } else {
            revert HorizonStakingInvalidThawRequestType();
        }
    }
}
 ------------
 ------------ END OF MAIN TARGET CONTRACT ------------ 

 ------------ ## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES ------------ 
// SPDX-License-Identifier: GPL-2.0-or-later

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable gas-strict-inequalities

pragma solidity 0.8.27 || 0.8.33;

import { IHorizonStakingTypes } from "@graphprotocol/interfaces/contracts/horizon/internal/IHorizonStakingTypes.sol";
import { IHorizonStakingBase } from "@graphprotocol/interfaces/contracts/horizon/internal/IHorizonStakingBase.sol";
import { IGraphPayments } from "@graphprotocol/interfaces/contracts/horizon/IGraphPayments.sol";
import { ILinkedList } from "@graphprotocol/interfaces/contracts/horizon/internal/ILinkedList.sol";

import { MathUtils } from "../libraries/MathUtils.sol";
import { LinkedList } from "../libraries/LinkedList.sol";

import { Multicall } from "@openzeppelin/contracts/utils/Multicall.sol";
import { GraphUpgradeable } from "@graphprotocol/contracts/contracts/upgrades/GraphUpgradeable.sol";
import { Managed } from "./utilities/Managed.sol";
import { HorizonStakingV1Storage } from "./HorizonStakingStorage.sol";

/**
 * @title HorizonStakingBase contract
 * @author Edge & Node
 * @notice This contract is the base staking contract implementing storage getters for both internal
 * and external use.
 * @dev Implementation of the {IHorizonStakingBase} interface.
 * @dev It's meant to be inherited by the {HorizonStaking} and {HorizonStakingExtension}
 * contracts so some internal functions are also included here.
 * @custom:security-contact Please email security+contracts@thegraph.com if you find any
 * bugs. We may have an active bug bounty program.
 */
abstract contract HorizonStakingBase is
    Multicall,
    Managed,
    HorizonStakingV1Storage,
    GraphUpgradeable,
    IHorizonStakingTypes,
    IHorizonStakingBase
{
    using LinkedList for ILinkedList.List;

    /**
     * @notice The address of the subgraph data service.
     * @dev Require to handle the special case when the verifier is the subgraph data service.
     */
    address internal immutable SUBGRAPH_DATA_SERVICE_ADDRESS;

    /**
     * @notice The staking contract is upgradeable however we still use the constructor to set a few immutable variables
     * @param controller The address of the Graph controller contract
     * @param subgraphDataServiceAddress The address of the subgraph data service
     */
    constructor(address controller, address subgraphDataServiceAddress) Managed(controller) {
        SUBGRAPH_DATA_SERVICE_ADDRESS = subgraphDataServiceAddress;
    }

    /// @inheritdoc IHorizonStakingBase
    /// @dev Removes deprecated fields from the return value.
    function getServiceProvider(address serviceProvider) external view override returns (ServiceProvider memory) {
        ServiceProvider memory sp;
        ServiceProviderInternal storage spInternal = _serviceProviders[serviceProvider];
        sp.tokensStaked = spInternal.tokensStaked;
        sp.tokensProvisioned = spInternal.tokensProvisioned;
        return sp;
    }

    /// @inheritdoc IHorizonStakingBase
    function getStake(address serviceProvider) external view override returns (uint256) {
        return _serviceProviders[serviceProvider].tokensStaked;
    }

    /// @inheritdoc IHorizonStakingBase
    function getIdleStake(address serviceProvider) external view override returns (uint256) {
        return _getIdleStake(serviceProvider);
    }

    /// @inheritdoc IHorizonStakingBase
    /// @dev Removes deprecated fields from the return value.
    function getDelegationPool(
        address serviceProvider,
        address verifier
    ) external view override returns (DelegationPool memory) {
        DelegationPool memory pool;
        DelegationPoolInternal storage poolInternal = _getDelegationPool(serviceProvider, verifier);
        pool.tokens = poolInternal.tokens;
        pool.shares = poolInternal.shares;
        pool.tokensThawing = poolInternal.tokensThawing;
        pool.sharesThawing = poolInternal.sharesThawing;
        pool.thawingNonce = poolInternal.thawingNonce;
        return pool;
    }

    /// @inheritdoc IHorizonStakingBase
    /// @dev Removes deprecated fields from the return value.
    function getDelegation(
        address serviceProvider,
        address verifier,
        address delegator
    ) external view override returns (Delegation memory) {
        Delegation memory delegation;
        DelegationPoolInternal storage poolInternal = _getDelegationPool(serviceProvider, verifier);
        delegation.shares = poolInternal.delegators[delegator].shares;
        return delegation;
    }

    /// @inheritdoc IHorizonStakingBase
    function getDelegationFeeCut(
        address serviceProvider,
        address verifier,
        IGraphPayments.PaymentTypes paymentType
    ) external view override returns (uint256) {
        return _delegationFeeCut[serviceProvider][verifier][paymentType];
    }

    /// @inheritdoc IHorizonStakingBase
    function getProvision(address serviceProvider, address verifier) external view override returns (Provision memory) {
        return _provisions[serviceProvider][verifier];
    }

    /// @inheritdoc IHorizonStakingBase
    function getTokensAvailable(
        address serviceProvider,
        address verifier,
        uint32 delegationRatio
    ) external view override returns (uint256) {
        uint256 tokensAvailableProvider = _getProviderTokensAvailable(serviceProvider, verifier);
        uint256 tokensAvailableDelegated = _getDelegatedTokensAvailable(serviceProvider, verifier);

        uint256 tokensDelegatedMax = tokensAvailableProvider * (uint256(delegationRatio));
        uint256 tokensDelegatedCapacity = MathUtils.min(tokensAvailableDelegated, tokensDelegatedMax);

        return tokensAvailableProvider + tokensDelegatedCapacity;
    }

    /// @inheritdoc IHorizonStakingBase
    function getProviderTokensAvailable(
        address serviceProvider,
        address verifier
    ) external view override returns (uint256) {
        return _getProviderTokensAvailable(serviceProvider, verifier);
    }

    /// @inheritdoc IHorizonStakingBase
    function getDelegatedTokensAvailable(
        address serviceProvider,
        address verifier
    ) external view override returns (uint256) {
        return _getDelegatedTokensAvailable(serviceProvider, verifier);
    }

    /// @inheritdoc IHorizonStakingBase
    function getThawRequest(
        ThawRequestType requestType,
        bytes32 thawRequestId
    ) external view override returns (ThawRequest memory) {
        return _getThawRequest(requestType, thawRequestId);
    }

    /// @inheritdoc IHorizonStakingBase
    function getThawRequestList(
        ThawRequestType requestType,
        address serviceProvider,
        address verifier,
        address owner
    ) external view override returns (ILinkedList.List memory) {
        return _getThawRequestList(requestType, serviceProvider, verifier, owner);
    }

    /// @inheritdoc IHorizonStakingBase
    function getThawedTokens(
        ThawRequestType requestType,
        address serviceProvider,
        address verifier,
        address owner
    ) external view override returns (uint256) {
        ILinkedList.List storage thawRequestList = _getThawRequestList(requestType, serviceProvider, verifier, owner);
        if (thawRequestList.count == 0) {
            return 0;
        }

        uint256 thawedTokens = 0;
        Provision storage prov = _provisions[serviceProvider][verifier];
        uint256 tokensThawing = prov.tokensThawing;
        uint256 sharesThawing = prov.sharesThawing;

        bytes32 thawRequestId = thawRequestList.head;
        while (thawRequestId != bytes32(0)) {
            ThawRequest storage thawRequest = _getThawRequest(requestType, thawRequestId);
            if (thawRequest.thawingNonce == prov.thawingNonce) {
                if (thawRequest.thawingUntil <= block.timestamp) {
                    // sharesThawing cannot be zero if there is a valid thaw request so the next division is safe
                    uint256 tokens = (thawRequest.shares * tokensThawing) / sharesThawing;
                    tokensThawing = tokensThawing - tokens;
                    sharesThawing = sharesThawing - thawRequest.shares;
                    thawedTokens = thawedTokens + tokens;
                } else {
                    break;
                }
            }

            thawRequestId = thawRequest.nextRequest;
        }
        return thawedTokens;
    }

    /// @inheritdoc IHorizonStakingBase
    function getMaxThawingPeriod() external view override returns (uint64) {
        return _maxThawingPeriod;
    }

    /// @inheritdoc IHorizonStakingBase
    function isAllowedLockedVerifier(address verifier) external view returns (bool) {
        return _allowedLockedVerifiers[verifier];
    }

    /// @inheritdoc IHorizonStakingBase
    function isDelegationSlashingEnabled() external view returns (bool) {
        return _delegationSlashingEnabled;
    }

    /**
     * @notice Deposit tokens into the service provider stake.
     * @dev TRANSITION PERIOD: After transition period move to IHorizonStakingMain. Temporarily it
     * needs to be here since it's used by both {HorizonStaking} and {HorizonStakingExtension}.
     *
     * Emits a {HorizonStakeDeposited} event.
     * @param _serviceProvider The address of the service provider.
     * @param _tokens The amount of tokens to deposit.
     */
    function _stake(address _serviceProvider, uint256 _tokens) internal {
        _serviceProviders[_serviceProvider].tokensStaked = _serviceProviders[_serviceProvider].tokensStaked + _tokens;
        emit HorizonStakeDeposited(_serviceProvider, _tokens);
    }

    /**
     * @notice Gets the service provider's idle stake which is the stake that is not being
     * used for any provision. Note that this only includes service provider's self stake.
     * @dev Note that the calculation considers tokens that were locked in the legacy staking contract.
     * @dev TRANSITION PERIOD: update the calculation after the transition period.
     * @param _serviceProvider The address of the service provider.
     * @return The amount of tokens that are idle.
     */
    function _getIdleStake(address _serviceProvider) internal view returns (uint256) {
        uint256 tokensUsed = _serviceProviders[_serviceProvider].tokensProvisioned +
            _serviceProviders[_serviceProvider].__DEPRECATED_tokensAllocated +
            _serviceProviders[_serviceProvider].__DEPRECATED_tokensLocked;
        uint256 tokensStaked = _serviceProviders[_serviceProvider].tokensStaked;
        return tokensStaked > tokensUsed ? tokensStaked - tokensUsed : 0;
    }

    /**
     * @notice Gets the details of delegation pool.
     * @dev Note that this function handles the special case where the verifier is the subgraph data service,
     * where the pools are stored in the legacy mapping.
     * @param _serviceProvider The address of the service provider.
     * @param _verifier The address of the verifier.
     * @return The delegation pool details.
     */
    function _getDelegationPool(
        address _serviceProvider,
        address _verifier
    ) internal view returns (DelegationPoolInternal storage) {
        if (_verifier == SUBGRAPH_DATA_SERVICE_ADDRESS) {
            return _legacyDelegationPools[_serviceProvider];
        } else {
            return _delegationPools[_serviceProvider][_verifier];
        }
    }

    /**
     * @notice Gets the service provider's tokens available in a provision.
     * @dev Calculated as the tokens available minus the tokens thawing.
     * @param _serviceProvider The address of the service provider.
     * @param _verifier The address of the verifier.
     * @return The amount of tokens available.
     */
    function _getProviderTokensAvailable(address _serviceProvider, address _verifier) internal view returns (uint256) {
        return _provisions[_serviceProvider][_verifier].tokens - _provisions[_serviceProvider][_verifier].tokensThawing;
    }

    /**
     * @notice Retrieves the next thaw request for a provision.
     * @param _thawRequestId The ID of the current thaw request.
     * @return The ID of the next thaw request in the list.
     */
    function _getNextProvisionThawRequest(bytes32 _thawRequestId) internal view returns (bytes32) {
        return _thawRequests[ThawRequestType.Provision][_thawRequestId].nextRequest;
    }

    /**
     * @notice Retrieves the next thaw request for a delegation.
     * @param _thawRequestId The ID of the current thaw request.
     * @return The ID of the next thaw request in the list.
     */
    function _getNextDelegationThawRequest(bytes32 _thawRequestId) internal view returns (bytes32) {
        return _thawRequests[ThawRequestType.Delegation][_thawRequestId].nextRequest;
    }

    /**
     * @notice Retrieves the thaw request list for the given request type.
     * @dev Uses the `ThawRequestType` to determine which mapping to access.
     * Reverts if the request type is unknown.
     * @param _requestType The type of thaw request (Provision or Delegation).
     * @param _serviceProvider The address of the service provider.
     * @param _verifier The address of the verifier.
     * @param _owner The address of the owner of the thaw request.
     * @return The linked list of thaw requests for the specified request type.
     */
    function _getThawRequestList(
        ThawRequestType _requestType,
        address _serviceProvider,
        address _verifier,
        address _owner
    ) internal view returns (ILinkedList.List storage) {
        return _thawRequestLists[_requestType][_serviceProvider][_verifier][_owner];
    }

    /**
     * @notice Retrieves a specific thaw request for the given request type.
     * @dev Uses the `ThawRequestType` to determine which mapping to access.
     * @param _requestType The type of thaw request (Provision or Delegation).
     * @param _thawRequestId The unique ID of the thaw request.
     * @return The thaw request data for the specified request type and ID.
     */
    function _getThawRequest(
        ThawRequestType _requestType,
        bytes32 _thawRequestId
    ) internal view returns (IHorizonStakingTypes.ThawRequest storage) {
        return _thawRequests[_requestType][_thawRequestId];
    }

    /**
     * @notice Determines the correct callback function for `getNextItem` based on the request type.
     * @param _requestType The type of thaw request (Provision or Delegation).
     * @return A function pointer to the appropriate `getNextItem` callback.
     */
    function _getNextThawRequest(
        ThawRequestType _requestType
    ) internal pure returns (function(bytes32) view returns (bytes32)) {
        if (_requestType == ThawRequestType.Provision) {
            return _getNextProvisionThawRequest;
        } else if (_requestType == ThawRequestType.Delegation) {
            return _getNextDelegationThawRequest;
        } else {
            revert HorizonStakingInvalidThawRequestType();
        }
    }

    /**
     * @notice Gets the delegator's tokens available in a provision.
     * @dev Calculated as the tokens available minus the tokens thawing.
     * @param _serviceProvider The address of the service provider.
     * @param _verifier The address of the verifier.
     * @return The amount of tokens available.
     */
    function _getDelegatedTokensAvailable(address _serviceProvider, address _verifier) private view returns (uint256) {
        DelegationPoolInternal storage poolInternal = _getDelegationPool(_serviceProvider, _verifier);
        return poolInternal.tokens - poolInternal.tokensThawing;
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity 0.8.27 || 0.8.33;

// TODO: Re-enable and fix issues when publishing a new version
// forge-lint: disable-start(mixed-case-variable)

import { IHorizonStakingExtension } from "@graphprotocol/interfaces/contracts/horizon/internal/IHorizonStakingExtension.sol";
import { IHorizonStakingTypes } from "@graphprotocol/interfaces/contracts/horizon/internal/IHorizonStakingTypes.sol";
import { IGraphPayments } from "@graphprotocol/interfaces/contracts/horizon/IGraphPayments.sol";
import { ILinkedList } from "@graphprotocol/interfaces/contracts/horizon/internal/ILinkedList.sol";

/* solhint-disable max-states-count */

/**
 * @title HorizonStakingV1Storage
 * @author Edge & Node
 * @notice This contract holds all the storage variables for the Staking contract
 * @dev Deprecated variables are kept to support the transition to Horizon Staking.
 * They can eventually be collapsed into a single storage slot.
 * @custom:security-contact Please email security+contracts@thegraph.com if you find any
 * bugs. We may have an active bug bounty program.
 */
abstract contract HorizonStakingV1Storage {
    // -- Staking --

    /// @dev Minimum amount of tokens an indexer needs to stake.
    /// Deprecated, now enforced by each data service (verifier)
    uint256 internal __DEPRECATED_minimumIndexerStake;

    /// @dev Time in blocks to unstake
    /// Deprecated, now enforced by each data service (verifier)
    uint32 internal __DEPRECATED_thawingPeriod; // in blocks

    /// @dev Percentage of fees going to curators
    /// Parts per million. (Allows for 4 decimal points, 999,999 = 99.9999%)
    /// Deprecated, now enforced by each data service (verifier)
    uint32 internal __DEPRECATED_curationPercentage;

    /// @dev Percentage of fees burned as protocol fee
    /// Parts per million. (Allows for 4 decimal points, 999,999 = 99.9999%)
    /// Deprecated, now enforced by each data service (verifier)
    uint32 internal __DEPRECATED_protocolPercentage;

    /// @dev Period for allocation to be finalized
    /// Deprecated with exponential rebates.
    uint32 private __DEPRECATED_channelDisputeEpochs;

    /// @dev Maximum allocation time.
    /// Deprecated, allocations now live on the subgraph service contract.
    uint32 internal __DEPRECATED_maxAllocationEpochs;

    /// @dev Rebate alpha numerator
    /// Originally used for Cobb-Douglas rebates, now used for exponential rebates
    /// Deprecated, any rebate mechanism is now applied on the subgraph data service.
    uint32 internal __DEPRECATED_alphaNumerator;

    /// @dev Rebate alpha denominator
    /// Originally used for Cobb-Douglas rebates, now used for exponential rebates
    /// Deprecated, any rebate mechanism is now applied on the subgraph data service.
    uint32 internal __DEPRECATED_alphaDenominator;

    /// @dev Service providers details, tracks stake utilization.
    mapping(address serviceProvider => IHorizonStakingTypes.ServiceProviderInternal details) internal _serviceProviders;

    /// @dev Allocation details.
    /// Deprecated, now applied on the subgraph data service
    mapping(address allocationId => IHorizonStakingExtension.Allocation allocation) internal __DEPRECATED_allocations;

    /// @dev Subgraph allocations, tracks the tokens allocated to a subgraph deployment
    /// Deprecated, now applied on the SubgraphService
    mapping(bytes32 subgraphDeploymentId => uint256 tokens) internal __DEPRECATED_subgraphAllocations;

    /// @dev Rebate pool details per epoch
    /// Deprecated with exponential rebates.
    mapping(uint256 epoch => uint256 rebates) private __DEPRECATED_rebates;

    // -- Slashing --

    /// @dev List of addresses allowed to slash stakes
    /// Deprecated, now each verifier can slash the corresponding provision.
    mapping(address slasher => bool allowed) internal __DEPRECATED_slashers;

    // -- Delegation --

    /// @dev Delegation capacity multiplier defined by the delegation ratio
    /// Deprecated, enforced by each data service as needed.
    uint32 internal __DEPRECATED_delegationRatio;

    /// @dev Time in blocks an indexer needs to wait to change delegation parameters
    /// Deprecated, enforced by each data service as needed.
    uint32 internal __DEPRECATED_delegationParametersCooldown;

    /// @dev Time in epochs a delegator needs to wait to withdraw delegated stake
    /// Deprecated, now only enforced during a transition period
    uint32 internal __DEPRECATED_delegationUnbondingPeriod;

    /// @dev Percentage of tokens to tax a delegation deposit
    /// Parts per million. (Allows for 4 decimal points, 999,999 = 99.9999%)
    /// Deprecated, no tax is applied now.
    uint32 internal __DEPRECATED_delegationTaxPercentage;

    /// @dev Delegation pools (legacy).
    /// Only used when the verifier is the subgraph data service.
    mapping(address serviceProvider => IHorizonStakingTypes.DelegationPoolInternal delegationPool)
        internal _legacyDelegationPools;

    // -- Operators --

    /// @dev Operator allow list (legacy)
    /// Only used when the verifier is the subgraph data service.
    mapping(address serviceProvider => mapping(address legacyOperator => bool authorized)) internal _legacyOperatorAuth;

    // -- Asset Holders --

    /// @dev Asset holder allow list
    /// Deprecated with permissionless payers
    mapping(address assetHolder => bool allowed) private __DEPRECATED_assetHolders;

    /// @dev Destination of accrued indexing rewards
    /// Deprecated, defined by each data service as needed
    mapping(address serviceProvider => address rewardsDestination) internal __DEPRECATED_rewardsDestination;

    /// @dev Address of the counterpart Staking contract on L1/L2
    /// Deprecated, transfer tools no longer enabled.
    address internal __DEPRECATED_counterpartStakingAddress;

    /// @dev Address of the StakingExtension implementation
    /// This is now an immutable variable to save some gas.
    address internal __DEPRECATED_extensionImpl;

    /// @dev Rebate lambda numerator for exponential rebates
    /// Deprecated, any rebate mechanism is now applied on the subgraph data service.
    uint32 internal __DEPRECATED_lambdaNumerator;

    /// @dev Rebate lambda denominator for exponential rebates
    /// Deprecated, any rebate mechanism is now applied on the subgraph data service.
    uint32 internal __DEPRECATED_lambdaDenominator;

    // -- Horizon Staking --

    /// @dev Maximum thawing period, in seconds, for a provision
    /// Note that to protect delegation from being unfairly locked this should be set to a sufficiently low value
    /// Additionally note that setting this to a high enough value could lead to overflow when calculating thawing until
    /// dates. For practical purposes this should not be an issue but we recommend using a value like 1e18 to represent
    /// "infinite thawing" if that is the intent.
    uint64 internal _maxThawingPeriod;

    /// @dev Provisions from each service provider for each data service
    mapping(address serviceProvider => mapping(address verifier => IHorizonStakingTypes.Provision provision))
        internal _provisions;

    /// @dev Delegation fee cuts for each service provider on each provision, by fee type:
    /// This is the effective delegator fee cuts for each (data-service-defined) fee type (e.g. indexing fees, query fees).
    /// This is in PPM and is the cut taken by the service provider from the fees that correspond to delegators.
    /// (based on stake vs delegated stake proportion).
    /// The cuts are applied in GraphPayments so apply to all data services that use it.
    mapping(address serviceProvider => mapping(address verifier => mapping(IGraphPayments.PaymentTypes paymentType => uint256 feeCut)))
        internal _delegationFeeCut;

    /// @dev Thaw requests
    /// Details for each thawing operation in the staking contract (for both service providers and delegators).
    mapping(IHorizonStakingTypes.ThawRequestType thawRequestType => mapping(bytes32 thawRequestId => IHorizonStakingTypes.ThawRequest thawRequest))
        internal _thawRequests;

    /// @dev Thaw request lists
    /// Metadata defining linked lists of thaw requests for each service provider or delegator (owner)
    mapping(IHorizonStakingTypes.ThawRequestType thawRequestType => mapping(address serviceProvider => mapping(address verifier => mapping(address owner => ILinkedList.List list))))
        internal _thawRequestLists;

    /// @dev Operator allow list
    /// Used for all verifiers except the subgraph data service.
    mapping(address serviceProvider => mapping(address verifier => mapping(address operator => bool authorized)))
        internal _operatorAuth;

    /// @dev Flag to enable or disable delegation slashing
    bool internal _delegationSlashingEnabled;

    /// @dev Delegation pools for each service provider and verifier
    mapping(address serviceProvider => mapping(address verifier => IHorizonStakingTypes.DelegationPoolInternal delegationPool))
        internal _delegationPools;

    /// @dev Allowed verifiers for locked provisions (i.e. from GraphTokenLockWallets)
    // Verifiers are whitelisted to ensure locked tokens cannot escape using an arbitrary verifier.
    mapping(address verifier => bool allowed) internal _allowedLockedVerifiers;
}

// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity 0.8.27 || 0.8.33;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable gas-increment-by-one, gas-strict-inequalities

import { ILinkedList } from "@graphprotocol/interfaces/contracts/horizon/internal/ILinkedList.sol";

/**
 * @title LinkedList library
 * @author Edge & Node
 * @notice A library to manage singly linked lists.
 *
 * The library makes no assumptions about the contents of the items, the only
 * requirements on the items are:
 * - they must be represented by a unique bytes32 id
 * - the id of the item must not be bytes32(0)
 * - each item must have a reference to the next item in the list
 * - the list cannot have more than `MAX_ITEMS` items
 *
 * A contract using this library must store:
 * - a LinkedList.List to keep track of the list metadata
 * - a mapping from bytes32 to the item data
 * @custom:security-contact Please email security+contracts@thegraph.com if you find any
 * bugs. We may have an active bug bounty program.
 */
library LinkedList {
    using LinkedList for ILinkedList.List;

    /// @notice Empty bytes constant
    bytes internal constant NULL_BYTES = bytes("");

    /// @notice Maximum amount of items allowed in the list
    uint256 internal constant MAX_ITEMS = 10_000;

    /**
     * @notice Adds an item to the list.
     * The item is added to the end of the list.
     * @dev Note that this function will not take care of linking the
     * old tail to the new item. The caller should take care of this.
     * It will also not ensure id uniqueness.
     * @dev There is a maximum number of elements that can be added to the list.
     * @param self The list metadata
     * @param id The id of the item to add
     */
    function addTail(ILinkedList.List storage self, bytes32 id) internal {
        require(self.count < MAX_ITEMS, ILinkedList.LinkedListMaxElementsExceeded());
        require(id != bytes32(0), ILinkedList.LinkedListInvalidZeroId());
        self.tail = id;
        self.nonce += 1;
        if (self.count == 0) self.head = id;
        self.count += 1;
    }

    /**
     * @notice Removes an item from the list.
     * The item is removed from the beginning of the list.
     * @param self The list metadata
     * @param getNextItem A function to get the next item in the list. It should take
     * the id of the current item and return the id of the next item.
     * @param deleteItem A function to delete an item. This should delete the item from
     * the contract storage. It takes the id of the item to delete.
     * @return The id of the head of the list.
     */
    function removeHead(
        ILinkedList.List storage self,
        function(bytes32) view returns (bytes32) getNextItem,
        function(bytes32) deleteItem
    ) internal returns (bytes32) {
        require(self.count > 0, ILinkedList.LinkedListEmptyList());
        bytes32 nextItem = getNextItem(self.head);
        deleteItem(self.head);
        self.count -= 1;
        self.head = nextItem;
        if (self.count == 0) self.tail = bytes32(0);
        return self.head;
    }

    /**
     * @notice Traverses the list and processes each item.
     * It deletes the processed items from both the list and the storage mapping.
     * @param self The list metadata
     * @param getNextItem A function to get the next item in the list. It should take
     * the id of the current item and return the id of the next item.
     * @param processItem A function to process an item. The function should take the id of the item
     * and an accumulator, and return:
     * - a boolean indicating whether the traversal should stop
     * - an accumulator to pass data between iterations
     * @param deleteItem A function to delete an item. This should delete the item from
     * the contract storage. It takes the id of the item to delete.
     * @param processInitAcc The initial accumulator data
     * @param iterations The maximum number of iterations to perform. If 0, the traversal will continue
     * until the end of the list.
     * @return The number of items processed
     * @return The final accumulator data.
     */
    function traverse(
        ILinkedList.List storage self,
        function(bytes32) view returns (bytes32) getNextItem,
        function(bytes32, bytes memory) returns (bool, bytes memory) processItem,
        function(bytes32) deleteItem,
        bytes memory processInitAcc,
        uint256 iterations
    ) internal returns (uint256, bytes memory) {
        require(iterations <= self.count, ILinkedList.LinkedListInvalidIterations());

        uint256 itemCount = 0;
        iterations = (iterations == 0) ? self.count : iterations;

        bytes32 cursor = self.head;

        while (cursor != bytes32(0) && iterations > 0) {
            (bool shouldBreak, bytes memory acc_) = processItem(cursor, processInitAcc);

            if (shouldBreak) break;

            processInitAcc = acc_;
            cursor = self.removeHead(getNextItem, deleteItem);

            iterations--;
            itemCount++;
        }

        return (itemCount, processInitAcc);
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

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.27 || 0.8.33;

// TODO: Re-enable and fix issues when publishing a new version
// solhint-disable gas-strict-inequalities
// forge-lint: disable-start(mixed-case-function)

/**
 * @title PPMMath library
 * @author Edge & Node
 * @notice A library for handling calculations with parts per million (PPM) amounts.
 * @custom:security-contact Please email security+contracts@thegraph.com if you find any
 * bugs. We may have an active bug bounty program.
 */
library PPMMath {
    /// @notice Maximum value (100%) in parts per million (PPM).
    uint256 internal constant MAX_PPM = 1_000_000;

    /**
     * @notice Thrown when a value is expected to be in PPM but is not.
     * @param value The value that is not in PPM.
     */
    error PPMMathInvalidPPM(uint256 value);

    /**
     * @notice Thrown when no value in a multiplication is in PPM.
     * @param a The first value in the multiplication.
     * @param b The second value in the multiplication.
     */
    error PPMMathInvalidMulPPM(uint256 a, uint256 b);

    /**
     * @notice Multiplies two values, one of which must be in PPM.
     * @param a The first value.
     * @param b The second value.
     * @return The result of the multiplication.
     */
    function mulPPM(uint256 a, uint256 b) internal pure returns (uint256) {
        require(isValidPPM(a) || isValidPPM(b), PPMMathInvalidMulPPM(a, b));
        return (a * b) / MAX_PPM;
    }

    /**
     * @notice Multiplies two values, the second one must be in PPM, and rounds up the result.
     * @dev requirements:
     * - The second value must be in PPM.
     * @param a The first value.
     * @param b The second value.
     * @return The result of the multiplication.
     */
    function mulPPMRoundUp(uint256 a, uint256 b) internal pure returns (uint256) {
        require(isValidPPM(b), PPMMathInvalidPPM(b));
        return a - mulPPM(a, MAX_PPM - b);
    }

    /**
     * @notice Checks if a value is in PPM.
     * @dev A valid PPM value is between 0 and MAX_PPM.
     * @param value The value to check.
     * @return true if the value is in PPM, false otherwise.
     */
    function isValidPPM(uint256 value) internal pure returns (bool) {
        return value <= MAX_PPM;
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


## ------------ SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS ------------ 

## ------------ SUPPORTING CONTEXT: EXTERNAL LIBRARIES ------------ 

 ------------ END OF SUPPORTING CONTRACTS AND INTERFACES ------------ 


 ------------ ## DEPLOYMENT SCRIPTS ------------ 

