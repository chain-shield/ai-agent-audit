
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.28;

import {ProtocolAccessManaged} from "@summerfi/access-contracts/contracts/ProtocolAccessManaged.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IStakedSummerToken} from "../interfaces/IStakedSummerToken.sol";
import {ISummerToken} from "../interfaces/ISummerToken.sol";
import {ISummerVestingWalletsEscrow} from "../interfaces/ISummerVestingWalletsEscrow.sol";
import {EnumerableSet} from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import {EnumerableMap} from "@openzeppelin/contracts/utils/structs/EnumerableMap.sol";
import {IMinimalVestingFactory} from "../interfaces/IMinimalVestingFactory.sol";
import {IMinimalVestingWallet} from "../interfaces/IMinimalVestingWallet.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

/**
 * @title SummerVestingWalletsEscrow
 * @notice Escrow staking that mints xSUMR against SUMR balances held in vesting wallets.
 * @dev Core principles:
 *      - Users can stake governance power (xSUMR) against SUMR held in vesting wallets already owned by the escrow.
 *      - On stake: record vesting SUMR balance and `released` snapshot, then mint xSUMR 1:1 to the user.
 *      - On unstake: forward any SUMR released while staked to the user and transfer wallet ownership back; burn xSUMR.
 *      - The escrow does not move SUMR out of vesting wallets, only forwards released amounts at exit.
 *      - Vesting factories are allowlisted by governance to constrain integrations.
 */
contract SummerVestingWalletsEscrow is
    ISummerVestingWalletsEscrow,
    ProtocolAccessManaged,
    ReentrancyGuard
{
    using SafeERC20 for IStakedSummerToken;
    using SafeERC20 for ISummerToken;
    using SafeERC20 for IERC20;
    using EnumerableSet for EnumerableSet.AddressSet;
    using EnumerableMap for EnumerableMap.AddressToUintMap;

    // ============ IMMUTABLE STATE ============

    ISummerToken public immutable SUMMER_TOKEN;
    IStakedSummerToken public immutable STAKED_SUMMER_TOKEN;

    // ============ STORAGE ============

    EnumerableSet.AddressSet private _vestingFactories;
    mapping(address user => EnumerableMap.AddressToUintMap stakedVestingFactories)
        private _userStakedVestingFactoriesBalance;
    mapping(address user => EnumerableMap.AddressToUintMap stakedVestingFactoriesReleased)
        private _userStakedVestingFactoriesReleased;

    // ============ CONSTRUCTOR ============

    constructor(
        address _protocolAccessManager,
        address _summerToken,
        address _xSumr,
        address[] memory _initialVestingFactories
    ) ProtocolAccessManaged(_protocolAccessManager) {
        // Basic address validation to avoid footguns at deployment
        if (_summerToken == address(0)) {
            revert Staking_InvalidAddress(
                "Summer token address cannot be zero"
            );
        }
        if (_xSumr == address(0)) {
            revert Staking_InvalidAddress(
                "StakedSummerToken address cannot be zero"
            );
        }

        SUMMER_TOKEN = ISummerToken(_summerToken);
        STAKED_SUMMER_TOKEN = IStakedSummerToken(_xSumr);

        // Seed allowlist from constructor input
        for (uint256 i = 0; i < _initialVestingFactories.length; i++) {
            if (_initialVestingFactories[i] == address(0)) {
                revert Staking_InvalidAddress(
                    "Vesting factory address cannot be zero"
                );
            }
            _vestingFactories.add(_initialVestingFactories[i]);
        }
    }

    /// @inheritdoc ISummerVestingWalletsEscrow
    function vestingFactories()
        external
        view
        override
        returns (address[] memory)
    {
        return _vestingFactories.values();
    }

    /// @inheritdoc ISummerVestingWalletsEscrow
    function getVestingFactory(
        uint256 index
    ) external view override returns (address) {
        if (index >= _vestingFactories.length()) {
            revert Staking_InvalidIndex();
        }
        return _vestingFactories.at(index);
    }

    /// @inheritdoc ISummerVestingWalletsEscrow
    function userStakedVestingFactories(
        address _user
    ) external view override returns (address[] memory) {
        return _userStakedVestingFactoriesBalance[_user].keys();
    }

    /// @inheritdoc ISummerVestingWalletsEscrow
    function getUserStakedVestingFactory(
        address _user,
        uint256 _index
    ) external view override returns (address) {
        (address factory, ) = _userStakedVestingFactoriesBalance[_user].at(
            _index
        );
        return factory;
    }

    // ============ EXTERNAL FUNCTIONS - ADMIN ============

    /// @inheritdoc ISummerVestingWalletsEscrow
    function addVestingFactory(
        address _vestingFactory
    ) external override onlyGovernor {
        // Governance-controlled: expand integration surface area
        if (_vestingFactory == address(0)) {
            revert Staking_InvalidAddress(
                "Vesting factory address cannot be zero"
            );
        }

        if (!_vestingFactories.add(_vestingFactory)) {
            revert Staking_DuplicateFactory();
        }
        emit VestingFactoryAdded(_vestingFactory);
    }

    /// @inheritdoc ISummerVestingWalletsEscrow
    function removeVestingFactory(
        address _vestingFactory
    ) external override onlyGovernor {
        // Governance-controlled: reduce integration surface area
        if (_vestingFactory == address(0)) {
            revert Staking_InvalidAddress(
                "Vesting factory address cannot be zero"
            );
        }

        if (!_vestingFactories.remove(_vestingFactory)) {
            revert Staking_FactoryNotFound();
        }

        emit VestingFactoryRemoved(_vestingFactory);
    }
    // ============ EXTERNAL FUNCTIONS - RESCUE ============

    /// @inheritdoc ISummerVestingWalletsEscrow
    function rescueWallet(
        address _wallet,
        address _newOwner
    ) external override onlyGovernor {
        // Emergency-only: return vesting wallet ownership to a specified address
        if (_newOwner == address(0)) {
            revert Staking_InvalidAddress("New owner cannot be zero address");
        }
        IMinimalVestingWallet(_wallet).transferOwnership(_newOwner);
    }

    /// @inheritdoc ISummerVestingWalletsEscrow
    function rescueToken(
        address _token,
        address _to
    ) external override onlyGovernor {
        // Sweep arbitrary ERC20 tokens sitting on the escrow
        if (_token == address(0)) {
            revert Staking_InvalidAddress("Invalid token address");
        }
        if (_to == address(0)) {
            revert Staking_InvalidAddress("Invalid to address");
        }
        IERC20(_token).safeTransfer(
            _to,
            IERC20(_token).balanceOf(address(this))
        );
    }

    // ============ EXTERNAL FUNCTIONS - USER FLOWS ============

    /// @inheritdoc ISummerVestingWalletsEscrow
    function stakeVesting(
        address[] calldata factories
    ) public override nonReentrant {
        // Loop over requested factories and perform per-factory stake
        for (uint256 i = 0; i < factories.length; i++) {
            if (!_vestingFactories.contains(factories[i])) {
                revert Staking_FactoryNotEnabled();
            }
            _stakeFromFactory(IMinimalVestingFactory(factories[i]), msg.sender);
        }
    }

    /// @inheritdoc ISummerVestingWalletsEscrow
    function unstakeVesting(
        address[] calldata factories
    ) public override nonReentrant {
        // Process requested factories independently to allow partial exits
        for (uint256 i = 0; i < factories.length; i++) {
            _unstakeFromFactory(
                IMinimalVestingFactory(factories[i]),
                msg.sender
            );
        }
    }

    // ============ INTERNAL FUNCTIONS ============

    /**
     * @notice Stakes against a single vesting factory for `_user`, minting xSUMR equal to the vesting wallet SUMR balance.
     * @dev Requirements and side-effects:
     *      - Factory must be enabled externally via `addVestingFactory`
     *      - The escrow MUST already be the owner of the user's vesting wallet
     *      - Records the vesting wallet SUMR `balance` and `released` snapshot for later reconciliation
     *      - Emits `StakedVestingWallet(user, factory, balance, released)`
     *      - Mints xSUMR 1:1 to the user for the recorded `balance`
     *      - If the users vesting wallet received additional SUMR tokens before staking on top of vesting schedule,
     *        they will get additional xSUMR tokens
     * @param _vestingFactory The vesting factory implementation to resolve the user's vesting wallet
     * @param _user The user performing the stake
     * @custom:reverts Staking_FactoryAlreadyStaked If the user already staked this factory
     * @custom:reverts Staking_InvalidAddress If the vesting wallet cannot be resolved
     * @custom:reverts Staking_InvalidOwner If the escrow is not the current owner of the vesting wallet
     * @custom:reverts Staking_ZeroBalance If the vesting wallet SUMR balance is zero
     */
    function _stakeFromFactory(
        IMinimalVestingFactory _vestingFactory,
        address _user
    ) internal {
        // Prevent double-staking same factory to preserve invariant on accounting maps
        address factoryAddress = address(_vestingFactory);
        if (
            _userStakedVestingFactoriesBalance[_user].contains(factoryAddress)
        ) {
            revert Staking_FactoryAlreadyStaked();
        }

        // Resolve vesting wallet and validate escrow ownership
        // vestingWallets map can't be modified, always keeps the original owner
        address vestingWallet = _vestingFactory.vestingWallets(_user);
        if (vestingWallet == address(0)) {
            revert Staking_InvalidAddress("Vesting wallet not found");
        }

        _validateVestingWalletOwner(vestingWallet);

        // Snapshot SUMR balance and released amount at time of stake
        uint256 balance = SUMMER_TOKEN.balanceOf(vestingWallet);
        if (balance == 0) {
            revert Staking_ZeroBalance();
        }
        uint256 released = IMinimalVestingWallet(vestingWallet).released(
            address(SUMMER_TOKEN)
        );

        // Persist stake metadata for this user/factory pair
        _userStakedVestingFactoriesBalance[_user].set(factoryAddress, balance);
        _userStakedVestingFactoriesReleased[_user].set(
            factoryAddress,
            released
        );

        // Mint governance power (xSUMR) equal to vesting wallet SUMR balance
        STAKED_SUMMER_TOKEN.mint(_user, balance);

        emit StakedVestingWallet(_user, factoryAddress, balance, released);
    }

    /**
     * @notice Unstakes a previously staked vesting position for `_user` and `_vestingFactory`.
     * @dev Side-effects:
     *      - Computes tokens released while staked and transfers them back to the user
     *      - Transfers vesting wallet ownership from escrow back to the user
     *      - Burns xSUMR equal to recorded staked balance
     *      - Cleans recorded balance and released snapshots for the user/factory pair
     *      - Emits `UnstakedVestingWallet(user, factory, stakedBalance, releasedAtUnstake)`
     * @param _vestingFactory The vesting factory implementation to resolve the user's vesting wallet
     * @param _user The user performing the unstake
     * @custom:reverts Staking_NoStakeForFactory If the user has no stake for this factory
     * @custom:reverts Staking_InvalidAddress If the vesting wallet cannot be resolved
     * @custom:reverts Staking_InvalidOwner If the escrow is not the current owner of the vesting wallet
     */
    function _unstakeFromFactory(
        IMinimalVestingFactory _vestingFactory,
        address _user
    ) internal {
        // Ensure the user has an active stake for this factory
        address factoryAddress = address(_vestingFactory);
        if (
            !_userStakedVestingFactoriesBalance[_user].contains(factoryAddress)
        ) {
            revert Staking_NoStakeForFactory();
        }

        // Load stake state and resolve vesting wallet
        uint256 stakedBalance = _userStakedVestingFactoriesBalance[_user].get(
            factoryAddress
        );

        address vestingWallet = _vestingFactory.vestingWallets(_user);
        if (vestingWallet == address(0)) {
            revert Staking_InvalidAddress("Vesting wallet not found");
        }

        _validateVestingWalletOwner(vestingWallet);

        // Compute and forward the amount released while staked (permissionless release may be called externally)
        uint256 releasedAtStake = _userStakedVestingFactoriesReleased[_user]
            .get(address(_vestingFactory));
        uint256 releasedAtUnstake = IMinimalVestingWallet(vestingWallet)
            .released(address(SUMMER_TOKEN));
        uint256 releasedWhileStaked = releasedAtUnstake - releasedAtStake;
        if (releasedWhileStaked > 0) {
            /// @dev `release()` method is permissionless; it can be called by anyone.
            /// @dev the tokens released while staked are transferred back to the original owner
            SUMMER_TOKEN.safeTransfer(_user, releasedWhileStaked);
        }
        // Transfer vesting wallet ownership back to the user
        IMinimalVestingWallet(vestingWallet).transferOwnership(_user);

        // Clear stake state for this user/factory and burn xSUMR matching recorded balance
        _userStakedVestingFactoriesBalance[_user].remove(factoryAddress);
        _userStakedVestingFactoriesReleased[_user].remove(factoryAddress);

        STAKED_SUMMER_TOKEN.burnFrom(_user, stakedBalance);

        emit UnstakedVestingWallet(
            _user,
            factoryAddress,
            stakedBalance,
            releasedAtUnstake
        );
    }

    /**
     * @notice Validates that the escrow currently owns a vesting wallet.
     * @param _vestingWallet The vesting wallet address to check
     * @custom:reverts Staking_InvalidOwner If the vesting wallet owner is not this escrow
     */
    function _validateVestingWalletOwner(address _vestingWallet) internal view {
        if (IMinimalVestingWallet(_vestingWallet).owner() != address(this)) {
            revert Staking_InvalidOwner("Vesting wallet not owned by escrow");
        }
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: PARENT AND CALLED CONTRACTS
// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.28;

/**
 * @title ISummerVestingWalletsEscrow
 * @notice Interface for the escrow that allows staking xSUMR against SUMR balances held in vesting wallets.
 * @dev Implementations MUST enforce access control (governor) where specified and adhere to the documented
 *      revert conditions to ensure consistent behavior across integrations and tests.
 *
 * High-level Design:
 * - Users can stake governance power (xSUMR minted 1:1) against SUMR held in vesting wallets owned by the escrow.
 * - During stake, the escrow must be the owner of the vesting wallets; on unstake, ownership returns to the user.
 * - Any SUMR released while staked (via permissionless `release`) is forwarded to the user during unstake.
 * - Factories producing vesting wallets are allowlisted by governance; users can only stake from enabled factories.
 *
 * Security Considerations:
 * - Ownership precondition is enforced on stake/unstake to prevent accidental custody assumptions.
 * - The escrow never moves SUMR from the vesting wallet; it only accounts balances and mints/burns xSUMR.
 * - Rescue methods are governance-only and are intended as last-resort controls.
 */
interface ISummerVestingWalletsEscrow {
    // =============================
    //            EVENTS
    // =============================

    /**
     * @notice Emitted when a vesting factory is added to the allowed set.
     * @param vestingFactory Address of the vesting factory added.
     */
    event VestingFactoryAdded(address indexed vestingFactory);

    /**
     * @notice Emitted when a vesting factory is removed from the allowed set.
     * @param vestingFactory Address of the vesting factory removed.
     */
    event VestingFactoryRemoved(address indexed vestingFactory);

    /**
     * @notice Emitted when user staked a vesting wallet
     * @param user The user that staked the vesting wallet
     * @param vestingFactory The vesting factory that the user staked from
     * @param balance The balance of the vesting wallet at the time of staking
     * @param released The amount released from the vesting wallet at the time of staking
     */
    event StakedVestingWallet(
        address indexed user,
        address indexed vestingFactory,
        uint256 balance,
        uint256 released
    );

    /**
     * @notice Emitted when user unstaked a vesting wallet
     * @param user The user that unstaked the vesting wallet
     * @param vestingFactory The vesting factory that the user unstaked from
     * @param balance The amount originally staked from the vesting wallet
     * @param released The amount released from the vesting wallet at the time of unstaking
     */
    event UnstakedVestingWallet(
        address indexed user,
        address indexed vestingFactory,
        uint256 balance,
        uint256 released
    );

    // =============================
    //            ERRORS
    // =============================

    /**
     * @notice Thrown when a zero address or otherwise invalid address is supplied.
     * @param message Additional context for the invalid address error.
     */
    error Staking_InvalidAddress(string message);

    /**
     * @notice Thrown when a vesting wallet ownership is invalid for the attempted operation.
     * @dev Used when the staking contract is not the current owner of the vesting wallet during stake/unstake flows.
     * @param message Additional context for the invalid owner error.
     */
    error Staking_InvalidOwner(string message);

    /// @notice Thrown when an index is out of bounds for vesting factory queries.
    error Staking_InvalidIndex();

    /// @notice Thrown when attempting to add a vesting factory that already exists.
    error Staking_DuplicateFactory();

    /// @notice Thrown when attempting to remove a vesting factory that is not present.
    error Staking_FactoryNotFound();

    /// @notice Reserved for potential future use if balance sanity checks are required.
    error Staking_InvalidBalance();

    /// @notice Thrown when attempting to operate on a factory that is not enabled for staking.
    error Staking_FactoryNotEnabled();

    /// @notice Thrown when attempting to stake from a factory with zero SUMR balance.
    error Staking_ZeroBalance();

    /// @notice Thrown when attempting to unstake a factory that has not been staked by the caller.
    error Staking_NoStakeForFactory();

    /// @notice Thrown when attempting to stake a factory that is already staked by the caller.
    error Staking_FactoryAlreadyStaked();

    // =============================
    //         VIEW METHODS
    // =============================

    /// @notice Returns the list of enabled vesting factories.
    /// @return factories An array of vesting factory addresses currently allowed.
    function vestingFactories()
        external
        view
        returns (address[] memory factories);

    /// @notice Returns the vesting factory at a given index.
    /// @dev Reverts if the index is out of bounds.
    /// @param index The index in the enabled vesting factories set.
    /// @return factory The vesting factory address at the given index.
    /// @custom:reverts Staking_InvalidIndex If `index` is >= number of factories.
    function getVestingFactory(
        uint256 index
    ) external view returns (address factory);

    /// @notice Returns the list of vesting factories from which the user has staked.
    /// @param user The user to query.
    /// @return factories Array of vesting factory addresses the user has staked from.
    function userStakedVestingFactories(
        address user
    ) external view returns (address[] memory factories);

    /// @notice Returns the vesting factory address at `index` for a given user.
    /// @dev Reverts if the index is out of bounds for the user's list.
    /// @param user The user to query.
    /// @param index Index into the user's staked vesting factories list.
    /// @return factory The vesting factory address.
    function getUserStakedVestingFactory(
        address user,
        uint256 index
    ) external view returns (address factory);

    // =============================
    //        GOVERNANCE METHODS
    // =============================

    /// @notice Adds a new vesting factory to the allowed set.
    /// @dev Access restricted to governor in implementing contract.
    /// @param vestingFactory The vesting factory address to add. Must be non-zero and not already present.
    /// @custom:reverts Staking_InvalidAddress If `vestingFactory` is the zero address.
    /// @custom:reverts Staking_DuplicateFactory If `vestingFactory` already exists in the set.
    /// @custom:emits VestingFactoryAdded Emitted upon successful addition.
    function addVestingFactory(address vestingFactory) external;

    /// @notice Removes a vesting factory from the allowed set.
    /// @dev Access restricted to governor in implementing contract.
    /// @param vestingFactory The vesting factory address to remove. Must be non-zero and present.
    /// @custom:reverts Staking_InvalidAddress If `vestingFactory` is the zero address.
    /// @custom:reverts Staking_FactoryNotFound If `vestingFactory` is not present in the set.
    /// @custom:emits VestingFactoryRemoved Emitted upon successful removal.
    function removeVestingFactory(address vestingFactory) external;

    /// @notice Transfers ownership of a vesting wallet to a new owner.
    /// @dev Access restricted to governor in implementing contract. This is an emergency escape hatch; governance is
    ///      responsible for downstream reconciliation of any tokens associated with the vesting wallet.
    /// @param wallet The vesting wallet address whose ownership will be transferred.
    /// @param newOwner The new owner address. Must be non-zero.
    /// @custom:reverts Staking_InvalidAddress If `newOwner` is the zero address.
    function rescueWallet(address wallet, address newOwner) external;

    /// @notice Transfers any balance of an ERC-20 token held by the escrow to a specified address.
    /// @dev Access restricted to governor in implementing contract.
    /// @param token The ERC-20 token address to rescue.
    /// @param to The recipient of the rescued tokens.
    function rescueToken(address token, address to) external;

    // =============================
    //          USER FLOWS
    // =============================

    /// @notice Stakes against the SUMR balances held in the caller's vesting wallets for the specified factories.
    /// @dev Each factory is processed independently; for each factory this will record the staked balance and released
    ///      amount, and mint xSUMR equal to the vesting wallet SUMR balance for that factory.
    /// @param factories The list of vesting factory addresses to stake from.
    /// @custom:reverts Staking_FactoryNotEnabled If a specified factory is not enabled.
    /// @custom:reverts Staking_FactoryAlreadyStaked If a specified factory is already staked for the caller.
    /// @custom:reverts Staking_InvalidOwner If the vesting wallet is not owned by the escrow.
    /// @custom:reverts Staking_ZeroBalance If the vesting wallet SUMR balance is zero.
    function stakeVesting(address[] calldata factories) external;

    /// @notice Unstakes previously staked vesting positions for the specified factories and returns vesting wallet
    ///         ownership back to the user for each.
    /// @dev Each factory is processed independently; this burns xSUMR equal to the recorded staked balance for that
    ///      factory and transfers vesting wallet ownership back to the original owner. Any SUMR released while staked
    ///      is forwarded to the original owner. user has to approve the escrow to burn xSUMR.
    /// @param factories The list of vesting factory addresses to unstake.
    /// @custom:reverts Staking_NoStakeForFactory If a specified factory has not been staked by the caller.
    /// @custom:reverts Staking_InvalidOwner If the vesting wallet is not owned by the escrow.
    function unstakeVesting(address[] calldata factories) external;
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.28;

import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {ERC20Burnable} from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import {ERC20Pausable} from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Pausable.sol";
import {ERC20Permit} from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol";
import {ERC20Votes} from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Votes.sol";
import {Nonces} from "@openzeppelin/contracts/utils/Nonces.sol";
import {ProtocolAccessManaged} from "@summerfi/access-contracts/contracts/ProtocolAccessManaged.sol";
import {IStakedSummerToken} from "../interfaces/IStakedSummerToken.sol";

/**
 * @title StakedSummerToken (xSUMR)
 * @notice Non-transferable staked representation of SUMR used for governance and rewards accounting.
 * @dev Key properties:
 *      - Minting/Burning controlled by governance-authorized staking modules
 *      - Direct transfers disabled; only mint (from address(0)) and burn (to address(0)) allowed
 *      - Pausable by guardian/governor for emergency response
 *      - Integrates ERC20Permit and ERC20Votes for signatures and governance snapshots
 *
 * Access control model:
 *      - Governor can add/remove staking modules, which grants MINTER and BURNER roles
 *      - Only modules with MINTER_ROLE can mint
 *      - burnFrom requires either the token owner or an address with BURNER_ROLE plus standard allowance
 */
contract StakedSummerToken is
    IStakedSummerToken,
    ERC20Burnable,
    ERC20Pausable,
    ProtocolAccessManaged,
    AccessControl,
    ERC20Permit,
    ERC20Votes
{
    // ============ ROLES ============
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
    bytes32 public constant BURNER_ROLE = keccak256("BURNER_ROLE");

    // ============ CONSTRUCTOR ============
    constructor(
        address _protocolAccessManager
    )
        ERC20("StakedSummerToken", "xSUMR")
        ERC20Permit("StakedSummerToken")
        ProtocolAccessManaged(_protocolAccessManager)
    {}

    // ============ GOVERNANCE ============

    /// @inheritdoc IStakedSummerToken
    function addStakingModule(address _stakingModule) external onlyGovernor {
        if (_stakingModule == address(0)) {
            revert xSumr_InvalidStakingModule(
                "Staking module address cannot be zero"
            );
        }
        // Authorize staking module to participate in mint/burn flows
        _grantRole(MINTER_ROLE, _stakingModule);
        _grantRole(BURNER_ROLE, _stakingModule);

        emit StakingModuleAdded(_stakingModule);
    }

    /// @inheritdoc IStakedSummerToken
    function removeStakingModule(address _stakingModule) external onlyGovernor {
        // Fully deauthorize staking module by revoking both roles
        _revokeRole(MINTER_ROLE, _stakingModule);
        _revokeRole(BURNER_ROLE, _stakingModule);
        emit StakingModuleRemoved(_stakingModule);
    }

    /// @inheritdoc IStakedSummerToken
    function pause() external onlyGuardianOrGovernor {
        _pause();
    }

    /// @inheritdoc IStakedSummerToken
    function unpause() external onlyGuardianOrGovernor {
        _unpause();
    }

    // ============ MINT / BURN API ============

    /// @inheritdoc IStakedSummerToken
    function mint(address to, uint256 amount) external onlyRole(MINTER_ROLE) {
        // Only authorized staking modules are permitted to mint xSUMR
        _mint(to, amount);
    }

    /// @inheritdoc IStakedSummerToken
    function burn(
        uint256 amount
    ) public override(ERC20Burnable, IStakedSummerToken) {
        super.burn(amount);
    }

    /// @inheritdoc IStakedSummerToken
    function burnFrom(
        address from,
        uint256 amount
    ) public override(ERC20Burnable, IStakedSummerToken) {
        if (!_canBurnFrom(from, msg.sender)) {
            revert xSumr__NotAuthorized();
        }
        // Honor allowance semantics when `msg.sender != from` via ERC20Burnable
        super.burnFrom(from, amount);
    }

    // ============ ERC6372 / ERC20Votes ============
    /// @notice Returns the current clock in seconds, used by ERC20Votes for timestamp-based checkpoints.
    function clock() public view override returns (uint48) {
        return uint48(block.timestamp);
    }

    // solhint-disable-next-line func-name-mixedcase
    /// @notice Returns the clock mode string as required by ERC-6372.
    function CLOCK_MODE() public pure override returns (string memory) {
        return "mode=timestamp";
    }

    // The following functions are overrides required by Solidity.
    function _update(
        address from,
        address to,
        uint256 value
    ) internal override(ERC20, ERC20Pausable, ERC20Votes) {
        if (!_canTransfer(from, to)) {
            revert xSumr_TransferNotAllowed();
        }
        // Run pausable and votes hooks (checkpoints, etc.)
        super._update(from, to, value);
    }

    function nonces(
        address owner
    ) public view override(ERC20Permit, Nonces) returns (uint256) {
        return super.nonces(owner);
    }

    // ============ ROLE MANAGEMENT (GOVERNOR) ============

    /// @inheritdoc IStakedSummerToken
    function grantMinterRole(address _minter) external onlyGovernor {
        _grantRole(MINTER_ROLE, _minter);
    }

    /// @inheritdoc IStakedSummerToken
    function revokeMinterRole(address _minter) external onlyGovernor {
        _revokeRole(MINTER_ROLE, _minter);
    }

    /**
     * @dev Overrides the grantRole function from AccessControl to disable direct role granting.
     * @notice This function always reverts with a DirectGrantIsDisabled error.
     */
    function grantRole(bytes32, address) public view override {
        revert DirectGrantIsDisabled(msg.sender);
    }

    /**
     * @dev Overrides the revokeRole function from AccessControl to disable direct role revoking.
     * @notice This function always reverts with a DirectRevokeIsDisabled error.
     */
    function revokeRole(bytes32, address) public view override {
        revert DirectRevokeIsDisabled(msg.sender);
    }

    // ============ INTERNAL HELPERS ============

    /**
     * @dev Only allow mint (from == address(0)) and burn (to == address(0)) movements. Block user-to-user transfers.
     * @notice All staking module interactions are based on `mint()` and `burnFrom()`;
     * transfers between users are disallowed.
     * @param from The address to transfer tokens from.
     * @param to The address to transfer tokens to.
     * @return bool True if the transfer is allowed, false otherwise.
     */
    function _canTransfer(
        address from,
        address to
    ) internal pure returns (bool) {
        return from == address(0) || to == address(0);
    }

    /**
     * @notice Allows `burnFrom` only if `spender` burns its own tokens or holds `BURNER_ROLE`.
     * @dev Even with `BURNER_ROLE`, standard ERC20 allowance rules apply.
     * @param from The address to burn tokens from.
     * @param spender The address to check for `BURNER_ROLE`.
     * @return bool True if the burn is allowed, false otherwise.
     */
    function _canBurnFrom(
        address from,
        address spender
    ) internal view returns (bool) {
        return spender == from || hasRole(BURNER_ROLE, spender);
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.28;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

/**
 * @title IStakedSummerToken
 * @notice Interface for xSUMR, the non-transferable staked representation of SUMR used for governance and rewards.
 * @dev xSUMR is mint/burn controlled by approved staking modules. Direct transfers between users are disabled; only
 *      minting (from address(0)) and burning (to address(0)) are permitted movements. Implementations SHOULD enforce
 *      role-based access control for minting and restricted burning, and MAY expose pause controls via governance.
 *
 * Rationale and Invariants:
 * - Transfer path is intentionally restricted to mint/burn to avoid bypassing staking logic and snapshots.
 * - Multiple staking modules may be authorized concurrently; governance manages their lifecycle.
 * - `burnFrom` authorization requires either the owner or an address with BURNER role; allowance rules still apply.
 */
interface IStakedSummerToken is IERC20 {
    // =============================
    //            EVENTS
    // =============================

    /**
     * @notice Emitted when a staking module is granted mint/burn permissions on xSUMR.
     * @param stakingModule Address of the staking module added.
     */
    event StakingModuleAdded(address indexed stakingModule);

    /**
     * @notice Emitted when a staking module has its mint/burn permissions revoked on xSUMR.
     * @param stakingModule Address of the staking module removed.
     */
    event StakingModuleRemoved(address indexed stakingModule);

    // =============================
    //            ERRORS
    // =============================

    /**
     * @notice Thrown when a zero address or otherwise invalid staking module is supplied.
     * @param message Details about the invalid staking module input.
     */
    error xSumr_InvalidStakingModule(string message);

    /**
     * @notice Thrown when a caller attempts an operation without the required authorization.
     */
    error xSumr__NotAuthorized();

    /**
     * @notice Thrown when a forbidden token transfer is attempted (only mint/burn flows are allowed).
     */
    error xSumr_TransferNotAllowed();

    // =============================
    //          GOVERNANCE
    // =============================

    /**
     * @notice Adds a staking module with mint and burn permissions.
     * @dev Access restricted to governance in the implementing contract.
     * @param _stakingModule The staking module to authorize.
     *        Must be a non-zero address and expected to integrate with staking flows.
     * @custom:reverts xSumr_InvalidStakingModule If `_stakingModule` is the zero address.
     * @custom:emits StakingModuleAdded Emitted upon successful addition.
     */
    function addStakingModule(address _stakingModule) external;

    /**
     * @notice Removes a staking module with mint and burn permissions.
     * @dev Access restricted to governance in the implementing contract.
     * @param _stakingModule The staking module to remove.
     *        Must be a non-zero address and previously authorized.
     * @custom:reverts xSumr_InvalidStakingModule If `_stakingModule` is the zero address.
     * @custom:emits StakingModuleRemoved Emitted upon successful removal.
     */
    function removeStakingModule(address _stakingModule) external;

    /**
     * @notice Grants MINTER_ROLE to a specified address. Governor-only.
     * @dev Intended for emergency recovery scenarios (e.g., user burned xSUMR prematurely
     *      and needs redemption support). Normal mint authorization should be managed via
     *      `addStakingModule`.
     * @param _minter Address to grant MINTER_ROLE to.
     */
    function grantMinterRole(address _minter) external;

    /**
     * @notice Revokes MINTER_ROLE from a specified address. Governor-only.
     * @dev Intended for emergency recovery scenarios. Normal flow uses `removeStakingModule` for module revocation.
     * @param _minter Address to revoke MINTER_ROLE from.
     */
    function revokeMinterRole(address _minter) external;

    /**
     * @notice  Pauses token operations that honor pausability (e.g., burns).
     * @dev Callable by guardian or governor. While paused, `mint`, `burn` and `burnFrom` are blocked by ERC20Pausable.
     */
    function pause() external;

    /**
     * @notice Unpauses token operations.
     * @dev Callable by guardian or governor. Restores normal `mint`/`burn`/`burnFrom` behavior.
     */
    function unpause() external;

    // =============================
    //        MINT / BURN API
    // =============================

    /**
     * @notice Mints xSUMR to a recipient address.
     * @dev Access is expected to be restricted to authorized staking modules.
     * @param _to Recipient address for newly minted xSUMR.
     * @param _amount Amount of xSUMR to mint (1:1 to staked SUMR backing in typical flows).
     */
    function mint(address _to, uint256 _amount) external;

    /**
     * @notice Burns caller's xSUMR balance.
     * @dev Access: Token owner. Used for self-burn flows like unstaking where the owner directly initiates the burn.
     * @param _amount Amount of xSUMR to burn from the caller's balance.
     */
    function burn(uint256 _amount) external;

    /**
     * @notice Burns xSUMR from a specified address using module authorization and/or allowance
     * @dev Implementations SHOULD allow either the token owner or an authorized burner module to execute.
     * @param _from Address from which tokens will be burned.
     * @param _amount Amount of xSUMR to burn.
     * @dev Implementations SHOULD enforce either owner self-burn or burner-role authorization plus allowance.
     */
    function burnFrom(address _from, uint256 _amount) external;
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.28;

import {IAccessControlErrors} from "../interfaces/IAccessControlErrors.sol";
import {ContractSpecificRoles, IProtocolAccessManager} from "../interfaces/IProtocolAccessManager.sol";
import {ProtocolAccessManager} from "./ProtocolAccessManager.sol";

import {Context} from "@openzeppelin/contracts/utils/Context.sol";
import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";

/**
 * @title ProtocolAccessManaged
 * @notice This contract provides role-based access control functionality for protocol contracts
 * by interfacing with a central ProtocolAccessManager.
 *
 * @dev This contract is meant to be inherited by other protocol contracts that need
 * role-based access control. It provides modifiers and utilities to check various roles.
 *
 * The contract supports several key roles through modifiers:
 * 1. GOVERNOR_ROLE: System-wide administrators
 * 2. KEEPER_ROLE: Routine maintenance operators (contract-specific)
 * 3. SUPER_KEEPER_ROLE: Advanced maintenance operators (global)
 * 4. CURATOR_ROLE: Fleet-specific managers
 * 5. GUARDIAN_ROLE: Emergency response operators
 * 6. DECAY_CONTROLLER_ROLE: Specific role for decay management
 * 7. ADMIRALS_QUARTERS_ROLE: Specific role for admirals quarters bundler contract
 *
 * Usage:
 * - Inherit from this contract to gain access to role-checking modifiers
 * - Use modifiers like onlyGovernor, onlyKeeper, etc. to protect functions
 * - Access the internal _accessManager to perform custom role checks
 *
 * Security Considerations:
 * - The contract validates the access manager address during construction
 * - All role checks are performed against the immutable access manager instance
 * - Contract-specific roles are generated using the contract's address to prevent conflicts
 */
contract ProtocolAccessManaged is IAccessControlErrors, Context {
    /*//////////////////////////////////////////////////////////////
                                CONSTANTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Role identifier for protocol governors - highest privilege level with admin capabilities
    bytes32 public constant GOVERNOR_ROLE = keccak256("GOVERNOR_ROLE");

    /// @notice Role identifier for super keepers who can globally perform fleet maintanence roles
    bytes32 public constant SUPER_KEEPER_ROLE = keccak256("SUPER_KEEPER_ROLE");

    /**
     * @notice Role identifier for protocol guardians
     * @dev Guardians have emergency powers across multiple protocol components:
     * - Can pause/unpause Fleet operations for security
     * - Can pause/unpause TipJar operations
     * - Can cancel governance proposals on SummerGovernor even if they don't meet normal cancellation requirements
     * - Can cancel TipJar proposals
     *
     * The guardian role serves as an emergency backstop to protect the protocol, but with less
     * privilege than governors.
     */
    bytes32 public constant GUARDIAN_ROLE = keccak256("GUARDIAN_ROLE");

    /**
     * @notice Role identifier for decay controller
     * @dev This role allows the decay controller to manage the decay of user voting power
     */
    bytes32 public constant DECAY_CONTROLLER_ROLE =
        keccak256("DECAY_CONTROLLER_ROLE");

    /**
     * @notice Role identifier for admirals quarters bundler contract
     * @dev This role allows Admirals Quarters to unstake and withdraw assets from fleets, on behalf of users
     * @dev Withdrawn tokens go straight to users wallet, lowering the risk of manipulation if the role is compromised
     */
    bytes32 public constant ADMIRALS_QUARTERS_ROLE =
        keccak256("ADMIRALS_QUARTERS_ROLE");

    /*//////////////////////////////////////////////////////////////
                            STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /// @notice The ProtocolAccessManager instance used for access control
    ProtocolAccessManager internal immutable _accessManager;

    /*//////////////////////////////////////////////////////////////
                                CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Initializes the ProtocolAccessManaged contract
     * @param accessManager Address of the ProtocolAccessManager contract
     * @dev Validates the provided accessManager address and initializes the _accessManager
     */
    constructor(address accessManager) {
        if (accessManager == address(0)) {
            revert InvalidAccessManagerAddress(address(0));
        }

        if (
            !IERC165(accessManager).supportsInterface(
                type(IProtocolAccessManager).interfaceId
            )
        ) {
            revert InvalidAccessManagerAddress(accessManager);
        }

        _accessManager = ProtocolAccessManager(accessManager);
    }

    /*//////////////////////////////////////////////////////////////
                                MODIFIERS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Modifier to restrict access to governors only
     *
     * @dev Modifier to check that the caller has the Governor role
     * @custom:internal-logic
     * - Checks if the caller has the GOVERNOR_ROLE in the access manager
     * @custom:effects
     * - Reverts if the caller doesn't have the GOVERNOR_ROLE
     * - Allows the function to proceed if the caller has the role
     * @custom:security-considerations
     * - Ensures that only authorized governors can access critical functions
     * - Relies on the correct setup of the access manager
     */
    modifier onlyGovernor() {
        if (!_accessManager.hasRole(GOVERNOR_ROLE, msg.sender)) {
            revert CallerIsNotGovernor(msg.sender);
        }
        _;
    }

    /**
     * @notice Modifier to restrict access to keepers only
     * @dev Modifier to check that the caller has the Keeper role
     * @custom:internal-logic
     * - Checks if the caller has either the contract-specific KEEPER_ROLE or the SUPER_KEEPER_ROLE
     * @custom:effects
     * - Reverts if the caller doesn't have either of the required roles
     * - Allows the function to proceed if the caller has one of the roles
     * @custom:security-considerations
     * - Ensures that only authorized keepers can access maintenance functions
     * - Allows for both contract-specific and super keepers
     * @custom:gas-considerations
     * - Performs two role checks, which may impact gas usage
     */
    modifier onlyKeeper() {
        if (
            !_accessManager.hasRole(
                generateRole(ContractSpecificRoles.KEEPER_ROLE, address(this)),
                msg.sender
            ) && !_accessManager.hasRole(SUPER_KEEPER_ROLE, msg.sender)
        ) {
            revert CallerIsNotKeeper(msg.sender);
        }
        _;
    }

    /**
     * @notice Modifier to restrict access to super keepers only
     * @dev Modifier to check that the caller has the Super Keeper role
     * @custom:internal-logic
     * - Checks if the caller has the SUPER_KEEPER_ROLE in the access manager
     * @custom:effects
     * - Reverts if the caller doesn't have the SUPER_KEEPER_ROLE
     * - Allows the function to proceed if the caller has the role
     * @custom:security-considerations
     * - Ensures that only authorized super keepers can access advanced maintenance functions
     * - Relies on the correct setup of the access manager
     */
    modifier onlySuperKeeper() {
        if (!_accessManager.hasRole(SUPER_KEEPER_ROLE, msg.sender)) {
            revert CallerIsNotSuperKeeper(msg.sender);
        }
        _;
    }

    /**
     * @notice Modifier to restrict access to curators only
     * @param fleetAddress The address of the fleet to check the curator role for
     * @dev Checks if the caller has the contract-specific CURATOR_ROLE
     */
    modifier onlyCurator(address fleetAddress) {
        if (
            fleetAddress == address(0) ||
            !_accessManager.hasRole(
                generateRole(ContractSpecificRoles.CURATOR_ROLE, fleetAddress),
                msg.sender
            )
        ) {
            revert CallerIsNotCurator(msg.sender);
        }
        _;
    }

    /**
     * @notice Modifier to restrict access to guardians only
     * @dev Modifier to check that the caller has the Guardian role
     * @custom:internal-logic
     * - Checks if the caller has the GUARDIAN_ROLE in the access manager
     * @custom:effects
     * - Reverts if the caller doesn't have the GUARDIAN_ROLE
     * - Allows the function to proceed if the caller has the role
     * @custom:security-considerations
     * - Ensures that only authorized guardians can access emergency functions
     * - Relies on the correct setup of the access manager
     */
    modifier onlyGuardian() {
        if (!_accessManager.hasRole(GUARDIAN_ROLE, msg.sender)) {
            revert CallerIsNotGuardian(msg.sender);
        }
        _;
    }

    /**
     * @notice Modifier to restrict access to either guardians or governors
     * @dev Modifier to check that the caller has either the Guardian or Governor role
     * @custom:internal-logic
     * - Checks if the caller has either the GUARDIAN_ROLE or the GOVERNOR_ROLE
     * @custom:effects
     * - Reverts if the caller doesn't have either of the required roles
     * - Allows the function to proceed if the caller has one of the roles
     * @custom:security-considerations
     * - Ensures that only authorized guardians or governors can access certain functions
     * - Provides flexibility for functions that can be accessed by either role
     * @custom:gas-considerations
     * - Performs two role checks, which may impact gas usage
     */
    modifier onlyGuardianOrGovernor() {
        if (
            !_accessManager.hasRole(GUARDIAN_ROLE, msg.sender) &&
            !_accessManager.hasRole(GOVERNOR_ROLE, msg.sender)
        ) {
            revert CallerIsNotGuardianOrGovernor(msg.sender);
        }
        _;
    }

    /**
     * @notice Modifier to restrict access to decay controllers only
     */
    modifier onlyDecayController() {
        if (!_accessManager.hasRole(DECAY_CONTROLLER_ROLE, msg.sender)) {
            revert CallerIsNotDecayController(msg.sender);
        }
        _;
    }

    /**
     * @notice Modifier to restrict access to foundation only
     * @dev Modifier to check that the caller has the Foundation role
     * @custom:security-considerations
     * - Ensures that only the Foundation can access vesting and related functions
     * - Relies on the correct setup of the access manager
     */
    modifier onlyFoundation() {
        if (
            !_accessManager.hasRole(
                _accessManager.FOUNDATION_ROLE(),
                msg.sender
            )
        ) {
            revert CallerIsNotFoundation(msg.sender);
        }
        _;
    }

    /*//////////////////////////////////////////////////////////////
                            PUBLIC FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Generates a role identifier for a specific contract and role
     * @param roleName The name of the role
     * @param roleTargetContract The address of the contract the role is for
     * @return The generated role identifier
     * @dev This function is used to create unique role identifiers for contract-specific roles
     */
    function generateRole(
        ContractSpecificRoles roleName,
        address roleTargetContract
    ) public pure returns (bytes32) {
        return keccak256(abi.encodePacked(roleName, roleTargetContract));
    }

    /**
     * @notice Checks if an account has the Admirals Quarters role
     * @param account The address to check
     * @return bool True if the account has the Admirals Quarters role
     */
    function hasAdmiralsQuartersRole(
        address account
    ) public view returns (bool) {
        return _accessManager.hasRole(ADMIRALS_QUARTERS_ROLE, account);
    }

    /*//////////////////////////////////////////////////////////////
                            INTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Helper function to check if an address has the Governor role
     * @param account The address to check
     * @return bool True if the address has the Governor role
     */
    function _isGovernor(address account) internal view returns (bool) {
        return _accessManager.hasRole(GOVERNOR_ROLE, account);
    }

    function _isDecayController(address account) internal view returns (bool) {
        return _accessManager.hasRole(DECAY_CONTROLLER_ROLE, account);
    }

    /**
     * @notice Helper function to check if an address has the Foundation role
     * @param account The address to check
     * @return bool True if the address has the Foundation role
     */
    function _isFoundation(address account) internal view returns (bool) {
        return
            _accessManager.hasRole(_accessManager.FOUNDATION_ROLE(), account);
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.28;

import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {IERC20Permit} from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Permit.sol";
import {ISummerTokenErrors} from "../errors/ISummerTokenErrors.sol";
import {VotingDecayLibrary} from "@summerfi/voting-decay/VotingDecayLibrary.sol";
import {IGovernanceRewardsManager} from "./IGovernanceRewardsManager.sol";
import {IVotes} from "@openzeppelin/contracts/governance/extensions/GovernorVotes.sol";
import {Percentage} from "@summerfi/percentage-solidity/contracts/Percentage.sol";
import {IOFT} from "@layerzerolabs/oft-evm/contracts/interfaces/IOFT.sol";

/**
 * @title ISummerToken
 * @dev Interface for the Summer governance token, combining ERC20, permit functionality,
 * and voting decay mechanisms
 */
interface ISummerToken is
    IOFT,
    IERC20,
    IERC20Permit,
    ISummerTokenErrors,
    IVotes
{
    /*//////////////////////////////////////////////////////////////
                                STRUCTS
    //////////////////////////////////////////////////////////////*/

    /**
     * @dev Parameters required for contract construction
     * @param name The name of the token
     * @param symbol The symbol of the token
     * @param lzEndpoint The LayerZero endpoint address
     * @param initialOwner The initial owner of the contract
     * @param accessManager The access manager contract address
     * @param maxSupply The maximum token supply
     * @param transferEnableDate The timestamp when transfers can be enabled
     * @param hubChainId The chain ID of the hub chain
     */
    struct ConstructorParams {
        string name;
        string symbol;
        address lzEndpoint;
        address initialOwner;
        address accessManager;
        uint256 maxSupply;
        uint256 transferEnableDate;
        uint32 hubChainId;
    }

    /**
     * @dev Parameters required for contract initialization
     * @param initialSupply The initial token supply to mint
     * @param initialDecayFreeWindow The initial decay-free window duration in seconds
     * @param initialYearlyDecayRate The initial yearly decay rate as a percentage
     * @param initialDecayFunction The initial decay function type
     * @param vestingWalletFactory The address of the vesting wallet factory contract
     */
    struct InitializeParams {
        uint256 initialSupply;
        uint40 initialDecayFreeWindow;
        Percentage initialYearlyDecayRate;
        VotingDecayLibrary.DecayFunction initialDecayFunction;
        address vestingWalletFactory;
    }

    /*//////////////////////////////////////////////////////////////
                                ERRORS
    //////////////////////////////////////////////////////////////*/

    /*
     * @dev Error thrown when the chain is not the hub chain
     * @param chainId The chain ID
     * @param hubChainId The hub chain ID
     */
    error NotHubChain(uint256 chainId, uint256 hubChainId);

    /**
     * @notice Error thrown when transfers are not allowed
     */
    error TransferNotAllowed();

    /**
     * @notice Error thrown when transfers cannot be enabled yet
     */
    error TransfersCannotBeEnabledYet();

    /**
     * @notice Error thrown when transfers are already enabled
     */
    error TransfersAlreadyEnabled();

    /**
     * @notice Error thrown when the address length is invalid
     */
    error InvalidAddressLength();

    /*//////////////////////////////////////////////////////////////
                                EVENTS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Emitted when transfers are enabled
     */
    event TransfersEnabled();

    /**
     * @notice Error thrown when invalid peer arrays are provided
     */
    error SummerTokenInvalidPeerArrays();

    /**
     * @notice Emitted when an address is whitelisted
     * @param account The address of the whitelisted account
     */
    event AddressWhitelisted(address indexed account);

    /**
     * @notice Emitted when an address is removed from the whitelist
     * @param account The address of the removed account
     */
    event AddressRemovedFromWhitelist(address indexed account);

    /*//////////////////////////////////////////////////////////////
                            EXTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /**
     * @notice Returns the decay free window
     * @return The decay free window in seconds
     */
    function getDecayFreeWindow() external view returns (uint40);

    /**
     * @notice Returns the yearly decay rate as a percentage
     * @return The yearly decay rate as a Percentage type
     * @dev This returns the annualized rate using simple multiplication rather than
     * compound interest calculation for clarity and predictability
     */
    function getDecayRatePerYear() external view returns (Percentage);

    /**
     * @notice Returns the decay factor for an account
     * @param account The address to get the decay factor for
     * @return The decay factor for the account
     */
    function getDecayFactor(address account) external view returns (uint256);

    /**
     * @notice Returns the decay factor for an account at a specific timepoint
     * @param account The address to get the decay factor for
     * @param timepoint The timestamp to get the decay factor at
     * @return The decay factor for the account at the specified timepoint
     */
    function getPastDecayFactor(
        address account,
        uint256 timepoint
    ) external view returns (uint256);

    /**
     * @notice Returns the current votes for an account with decay factor applied
     * @param account The address to get votes for
     * @return The current voting power after applying the decay factor
     * @dev This function:
     * 1. Gets the raw votes using ERC20Votes' _getVotes
     * 2. Applies the decay factor from VotingDecayManager
     * @custom:relationship-to-votingdecay
     * - Uses VotingDecayManager.getVotingPower() to apply decay
     * - Decay factor is determined by:
     *   - Time since last update
     *   - Delegation chain (up to MAX_DELEGATION_DEPTH)
     *   - Current decayRatePerSecond and decayFreeWindow
     */
    function getVotes(address account) external view returns (uint256);

    /**
     * @notice Updates the decay factor for a specific account
     * @param account The address of the account to update
     * @dev Can only be called by the governor
     */
    function updateDecayFactor(address account) external;

    /**
     * @notice Sets the yearly decay rate for voting power decay
     * @param newYearlyRate The new decay rate per year as a Percentage
     * @dev Can only be called by the governor
     * @dev The rate is converted internally to a per-second rate using simple division
     */
    function setDecayRatePerYear(Percentage newYearlyRate) external;

    /**
     * @notice Sets the decay-free window duration
     * @param newWindow The new decay-free window duration in seconds
     * @dev Can only be called by the governor
     */
    function setDecayFreeWindow(uint40 newWindow) external;

    /**
     * @notice Sets the decay function type
     * @param newFunction The new decay function to use
     * @dev Can only be called by the governor
     */
    function setDecayFunction(
        VotingDecayLibrary.DecayFunction newFunction
    ) external;

    /**
     * @notice Enables transfers
     */
    function enableTransfers() external;

    /**
     * @notice Returns the address of the rewards manager contract
     * @return The address of the rewards manager
     */
    function rewardsManager() external view returns (address);

    /**
     * @notice Gets the length of the delegation chain for an account
     * @param account The address to check delegation chain for
     * @return The length of the delegation chain (0 for self-delegated or invalid chains)
     */
    function getDelegationChainLength(
        address account
    ) external view returns (uint256);

    /**
     * @notice Returns the raw votes (before decay) for an account at a specific timepoint
     * @param account The address to get raw votes for
     * @param timestamp The timestamp to get raw votes at
     * @return The current voting power before applying any decay factor
     * @dev This returns the total voting units including direct balance, staked tokens,
     * and vesting wallet balances, but without applying the decay factor
     */
    function getRawVotesAt(
        address account,
        uint256 timestamp
    ) external view returns (uint256);

    /**
     * @notice Returns the votes for an account at a specific past block, with decay factor applied
     * @param account The address to get votes for
     * @param timepoint The block number to get votes at
     * @return The historical voting power after applying the decay factor
     * @dev This function:
     * 1. Gets the historical raw votes using ERC20Votes' _getPastVotes
     * 2. Applies the current decay factor from VotingDecayManager
     * @custom:relationship-to-votingdecay
     * - Uses VotingDecayManager.getVotingPower() to apply decay
     * - Note: The decay factor is current, not historical
     * - This means voting power can decrease over time even for past checkpoints
     */
    function getPastVotes(
        address account,
        uint256 timepoint
    ) external view returns (uint256);
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.28;

/// @dev this is a minimal vesting wallet interface
interface IMinimalVestingWallet {
    /// @dev the balance of the vesting wallet can only go down - if it goes up - tokens were sent to the wallet (unintended bhavior)
    function balanceOf(address _user) external view returns (uint256);
    /// @dev the current owner of the vesting wallet ( might be different that owner in the factory contract)
    function owner() external view returns (address);
    /// @dev the ownership of the vesting wallet can be trnsfered - this is used to transfer the ownership of the vesting wallet to the user
    function transferOwnership(address newOwner) external;
    /// @dev the amount of tokens released from the vesting wallet
    function released(address _token) external view returns (uint256);
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.28;

/// @dev this is a minimal vesting factory interface
interface IMinimalVestingFactory {
    /// @dev each user can have a single vesting wallet - the balance of the vesting wallet can only go down
    function vestingWallets(address _user) external view returns (address);
    /// @dev the owner of the vesting wallet
    function vestingWalletOwners(
        address _wallet
    ) external view returns (address);
}

// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v5.0.1) (utils/Context.sol)

pragma solidity ^0.8.20;

/**
 * @dev Provides information about the current execution context, including the
 * sender of the transaction and its data. While these are generally available
 * via msg.sender and msg.data, they should not be accessed in such a direct
 * manner, since when dealing with meta-transactions the account sending and
 * paying for execution may not be the actual sender (as far as an application
 * is concerned).
 *
 * This contract is only required for intermediate, library-like contracts.
 */
abstract contract Context {
    function _msgSender() internal view virtual returns (address) {
        return msg.sender;
    }

    function _msgData() internal view virtual returns (bytes calldata) {
        return msg.data;
    }

    function _contextSuffixLength() internal view virtual returns (uint256) {
        return 0;
    }
}

// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.28;

import {ISummerVestingWallet} from "../interfaces/ISummerVestingWallet.sol";

/**
 * @title ISummerTokenErrors
 * @notice Interface defining custom errors for the SummerToken contract
 */
interface ISummerTokenErrors {
    /**
     * @dev Error thrown when an invalid vesting type is provided
     * @param invalidType The invalid vesting type that was provided
     */
    error InvalidVestingType(ISummerVestingWallet.VestingType invalidType);

    /**
     * @dev Error thrown when the caller is not the decay manager or governor
     * @param caller The address of the caller
     */
    error CallerIsNotAuthorized(address caller);

    /**
     * @dev Error thrown when the caller is not the decay manager
     * @param caller The address of the caller
     */
    error CallerIsNotDecayManager(address caller);

    /**
     * @dev Error thrown when the decay rate is too high
     */
    error DecayRateTooHigh(uint256 rate);

    /**
     * @dev Error thrown when the decay free window is invalid (less than 30 days or more than 365.25 days)
     * @param window The invalid window duration that was provided
     */
    error InvalidDecayFreeWindow(uint40 window);

    /**
     * @dev Error thrown when attempting to initialize the contract after it has already been initialized
     */
    error AlreadyInitialized();

    /**
     * @dev Error thrown when attempting to undelegate while staked
     */
    error CannotUndelegateWhileStaked();
}

// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v5.0.0) (governance/utils/IVotes.sol)
pragma solidity ^0.8.20;

/**
 * @dev Common interface for {ERC20Votes}, {ERC721Votes}, and other {Votes}-enabled contracts.
 */
interface IVotes {
    /**
     * @dev The signature used has expired.
     */
    error VotesExpiredSignature(uint256 expiry);

    /**
     * @dev Emitted when an account changes their delegate.
     */
    event DelegateChanged(address indexed delegator, address indexed fromDelegate, address indexed toDelegate);

    /**
     * @dev Emitted when a token transfer or delegate change results in changes to a delegate's number of voting units.
     */
    event DelegateVotesChanged(address indexed delegate, uint256 previousVotes, uint256 newVotes);

    /**
     * @dev Returns the current amount of votes that `account` has.
     */
    function getVotes(address account) external view returns (uint256);

    /**
     * @dev Returns the amount of votes that `account` had at a specific moment in the past. If the `clock()` is
     * configured to use block numbers, this will return the value at the end of the corresponding block.
     */
    function getPastVotes(address account, uint256 timepoint) external view returns (uint256);

    /**
     * @dev Returns the total supply of votes available at a specific moment in the past. If the `clock()` is
     * configured to use block numbers, this will return the value at the end of the corresponding block.
     *
     * NOTE: This value is the sum of all available votes, which is not necessarily the sum of all delegated votes.
     * Votes that have not been delegated are still part of total supply, even though they would not participate in a
     * vote.
     */
    function getPastTotalSupply(uint256 timepoint) external view returns (uint256);

    /**
     * @dev Returns the delegate that `account` has chosen.
     */
    function delegates(address account) external view returns (address);

    /**
     * @dev Delegates votes from the sender to `delegatee`.
     */
    function delegate(address delegatee) external;

    /**
     * @dev Delegates votes from signer to `delegatee`.
     */
    function delegateBySig(address delegatee, uint256 nonce, uint256 expiry, uint8 v, bytes32 r, bytes32 s) external;
}


END OF SUPPORTING CONTRACTS AND INTERFACES
