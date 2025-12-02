
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.20;

/* solhint-disable var-name-mixedcase */

import {StakediTry, SafeERC20, IERC20} from "./StakediTry.sol";
import {IStakediTryCooldown, UserCooldown} from "./interfaces/IStakediTryCooldown.sol";
import {iTrySilo} from "./iTrySilo.sol";

/**
 * @title StakediTryV2
 * @notice The StakediTryV2 contract allows users to stake iTry tokens and earn a portion of protocol LST and perpetual yield that is allocated
 * to stakers by the Ethena DAO governance voted yield distribution algorithm.  The algorithm seeks to balance the stability of the protocol by funding
 * the protocol's insurance fund, DAO activities, and rewarding stakers with a portion of the protocol's yield.
 * @dev If cooldown duration is set to zero, the StakediTryV2 behavior changes to follow ERC4626 standard and disables cooldownShares and cooldownAssets methods. If cooldown duration is greater than zero, the ERC4626 withdrawal and redeem functions are disabled, breaking the ERC4626 standard, and enabling the cooldownShares and the cooldownAssets functions.
 */
contract StakediTryV2 is IStakediTryCooldown, StakediTry {
    using SafeERC20 for IERC20;

    mapping(address => UserCooldown) public cooldowns;

    iTrySilo public immutable silo;

    uint24 public constant MAX_COOLDOWN_DURATION = 90 days;

    uint24 public cooldownDuration;

    /// @notice ensure cooldownDuration is zero
    modifier ensureCooldownOff() {
        if (cooldownDuration != 0) revert OperationNotAllowed();
        _;
    }

    /// @notice ensure cooldownDuration is gt 0
    modifier ensureCooldownOn() {
        if (cooldownDuration == 0) revert OperationNotAllowed();
        _;
    }

    /// @notice Constructor for StakediTryV2 contract.
    /// @param _asset The address of the iTry token.
    /// @param initialRewarder The address of the initial rewarder.
    /// @param _owner The address of the admin role.
    constructor(IERC20 _asset, address initialRewarder, address _owner) StakediTry(_asset, initialRewarder, _owner) {
        silo = new iTrySilo(address(this), address(_asset));
        cooldownDuration = MAX_COOLDOWN_DURATION;
    }

    /* ------------- EXTERNAL ------------- */

    /**
     * @dev See {IERC4626-withdraw}.
     */
    function withdraw(uint256 assets, address receiver, address _owner)
        public
        virtual
        override
        ensureCooldownOff
        returns (uint256)
    {
        return super.withdraw(assets, receiver, _owner);
    }

    /**
     * @dev See {IERC4626-redeem}.
     */
    function redeem(uint256 shares, address receiver, address _owner)
        public
        virtual
        override
        ensureCooldownOff
        returns (uint256)
    {
        return super.redeem(shares, receiver, _owner);
    }

    /// @notice Claim the staking amount after the cooldown has finished. The address can only retire the full amount of assets.
    /// @dev unstake can be called after cooldown have been set to 0, to let accounts to be able to claim remaining assets locked at Silo
    /// @param receiver Address to send the assets by the staker
    function unstake(address receiver) external {
        UserCooldown storage userCooldown = cooldowns[msg.sender];
        uint256 assets = userCooldown.underlyingAmount;

        if (block.timestamp >= userCooldown.cooldownEnd || cooldownDuration == 0) {
            userCooldown.cooldownEnd = 0;
            userCooldown.underlyingAmount = 0;

            silo.withdraw(receiver, assets);
        } else {
            revert InvalidCooldown();
        }
    }

    /// @notice redeem assets and starts a cooldown to claim the converted underlying asset
    /// @param assets assets to redeem
    function cooldownAssets(uint256 assets) external ensureCooldownOn returns (uint256 shares) {
        if (assets > maxWithdraw(msg.sender)) revert ExcessiveWithdrawAmount();

        shares = previewWithdraw(assets);

        cooldowns[msg.sender].cooldownEnd = uint104(block.timestamp) + cooldownDuration;
        cooldowns[msg.sender].underlyingAmount += uint152(assets);

        _withdraw(msg.sender, address(silo), msg.sender, assets, shares);
    }

    /// @notice redeem shares into assets and starts a cooldown to claim the converted underlying asset
    /// @param shares shares to redeem
    function cooldownShares(uint256 shares) external ensureCooldownOn returns (uint256 assets) {
        if (shares > maxRedeem(msg.sender)) revert ExcessiveRedeemAmount();

        assets = previewRedeem(shares);

        cooldowns[msg.sender].cooldownEnd = uint104(block.timestamp) + cooldownDuration;
        cooldowns[msg.sender].underlyingAmount += uint152(assets);

        _withdraw(msg.sender, address(silo), msg.sender, assets, shares);
    }

    /// @notice Set cooldown duration. If cooldown duration is set to zero, the StakediTryV2 behavior changes to follow ERC4626 standard and disables cooldownShares and cooldownAssets methods. If cooldown duration is greater than zero, the ERC4626 withdrawal and redeem functions are disabled, breaking the ERC4626 standard, and enabling the cooldownShares and the cooldownAssets functions.
    /// @param duration Duration of the cooldown
    function setCooldownDuration(uint24 duration) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (duration > MAX_COOLDOWN_DURATION) {
            revert InvalidCooldown();
        }

        uint24 previousDuration = cooldownDuration;
        cooldownDuration = duration;
        emit CooldownDurationUpdated(previousDuration, cooldownDuration);
    }
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.20;

/* solhint-disable private-vars-leading-underscore */

import {ERC4626} from "@openzeppelin/contracts/token/ERC20/extensions/ERC4626.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import {ERC20Permit, ERC20, IERC20} from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol";
import {SingleAdminAccessControl} from "../../utils/SingleAdminAccessControl.sol";
import {IStakediTry} from "./interfaces/IStakediTry.sol";

/**
 * @title StakediTry
 * @notice The StakediTry contract allows users to stake iTry tokens and earn a portion of protocol LST and perpetual yield that is allocated
 * to stakers by the Ethena DAO governance voted yield distribution algorithm.  The algorithm seeks to balance the stability of the protocol by funding
 * the protocol's insurance fund, DAO activities, and rewarding stakers with a portion of the protocol's yield.
 */
contract StakediTry is SingleAdminAccessControl, ReentrancyGuard, ERC20Permit, ERC4626, IStakediTry {
    using SafeERC20 for IERC20;

    /* ------------- CONSTANTS ------------- */
    /// @notice The role that is allowed to distribute rewards to this contract
    bytes32 private constant REWARDER_ROLE = keccak256("REWARDER_ROLE");
    /// @notice The role that is allowed to blacklist and un-blacklist addresses
    bytes32 private constant BLACKLIST_MANAGER_ROLE = keccak256("BLACKLIST_MANAGER_ROLE");
    /// @notice The role which prevents an address to stake
    bytes32 private constant SOFT_RESTRICTED_STAKER_ROLE = keccak256("SOFT_RESTRICTED_STAKER_ROLE");
    /// @notice The role which prevents an address to transfer, stake, or unstake. The owner of the contract can redirect address staking balance if an address is in full restricting mode.
    bytes32 private constant FULL_RESTRICTED_STAKER_ROLE = keccak256("FULL_RESTRICTED_STAKER_ROLE");
    /// @notice Minimum non-zero shares amount to prevent donation attack
    uint256 private constant MIN_SHARES = 1 ether;
    /// @notice Minimum allowed vesting period (1 hour)
    uint256 private constant MIN_VESTING_PERIOD = 1 hours;
    /// @notice Maximum allowed vesting period (30 days)
    uint256 private constant MAX_VESTING_PERIOD = 30 days;

    /* ------------- STATE VARIABLES ------------- */

    /// @notice The amount of the last asset distribution from the controller contract into this
    /// contract + any unvested remainder at that time
    uint256 public vestingAmount;

    /// @notice The timestamp of the last asset distribution from the controller contract into this contract
    uint256 public lastDistributionTimestamp;

    /// @notice The vesting period of lastDistributionAmount over which it increasingly becomes available to stakers
    uint256 private vestingPeriod;

    /* ------------- MODIFIERS ------------- */

    /// @notice ensure input amount nonzero
    modifier notZero(uint256 amount) {
        if (amount == 0) revert InvalidAmount();
        _;
    }

    /// @notice ensures blacklist target is not owner
    modifier notOwner(address target) {
        if (target == owner()) revert CantBlacklistOwner();
        _;
    }

    /* ------------- CONSTRUCTOR ------------- */

    /**
     * @notice Constructor for StakediTry contract.
     * @param _asset The address of the iTry token.
     * @param _initialRewarder The address of the initial rewarder.
     * @param _owner The address of the admin role.
     *
     */
    constructor(IERC20 _asset, address _initialRewarder, address _owner)
        ERC20("wiTRY", "wiTRY")
        ERC4626(_asset)
        ERC20Permit("wiTRY")
    {
        if (_owner == address(0) || _initialRewarder == address(0) || address(_asset) == address(0)) {
            revert InvalidZeroAddress();
        }

        vestingPeriod = MIN_VESTING_PERIOD;

        _grantRole(REWARDER_ROLE, _initialRewarder);
        _grantRole(DEFAULT_ADMIN_ROLE, _owner);
    }

    /* ------------- EXTERNAL ------------- */

    /**
     * @notice Allows the owner to update the vesting period.
     * @dev Can only be called when there are no unvested rewards to avoid disrupting active vesting.
     * @param _vestingPeriod The new vesting period (must be between MIN_VESTING_PERIOD and MAX_VESTING_PERIOD).
     */
    function setVestingPeriod(uint256 _vestingPeriod) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (_vestingPeriod < MIN_VESTING_PERIOD || _vestingPeriod > MAX_VESTING_PERIOD) {
            revert InvalidVestingPeriod();
        }
        if (getUnvestedAmount() > 0) {
            revert StillVesting();
        }

        uint256 oldVestingPeriod = vestingPeriod;
        vestingPeriod = _vestingPeriod;

        emit VestingPeriodUpdated(oldVestingPeriod, _vestingPeriod);
    }

    /**
     * @notice Allows the owner to transfer rewards from the controller contract into this contract.
     * @param amount The amount of rewards to transfer.
     */
    function transferInRewards(uint256 amount) external nonReentrant onlyRole(REWARDER_ROLE) notZero(amount) {
        _updateVestingAmount(amount);
        // transfer assets from rewarder to this contract
        IERC20(asset()).safeTransferFrom(msg.sender, address(this), amount);

        emit RewardsReceived(amount);
    }

    /**
     * @notice Allows the owner (DEFAULT_ADMIN_ROLE) and blacklist managers to blacklist addresses.
     * @param target The address to blacklist.
     * @param isFullBlacklisting Soft or full blacklisting level.
     */
    function addToBlacklist(address target, bool isFullBlacklisting)
        external
        onlyRole(BLACKLIST_MANAGER_ROLE)
        notOwner(target)
    {
        bytes32 role = isFullBlacklisting ? FULL_RESTRICTED_STAKER_ROLE : SOFT_RESTRICTED_STAKER_ROLE;
        _grantRole(role, target);
    }

    /**
     * @notice Allows the owner (DEFAULT_ADMIN_ROLE) and blacklist managers to un-blacklist addresses.
     * @param target The address to un-blacklist.
     * @param isFullBlacklisting Soft or full blacklisting level.
     */
    function removeFromBlacklist(address target, bool isFullBlacklisting) external onlyRole(BLACKLIST_MANAGER_ROLE) {
        bytes32 role = isFullBlacklisting ? FULL_RESTRICTED_STAKER_ROLE : SOFT_RESTRICTED_STAKER_ROLE;
        _revokeRole(role, target);
    }

    /**
     * @notice Allows the owner to rescue tokens accidentally sent to the contract.
     * Note that the owner cannot rescue iTry tokens because they functionally sit here
     * and belong to stakers but can rescue staked iTry as they should never actually
     * sit in this contract and a staker may well transfer them here by accident.
     * @param token The token to be rescued.
     * @param amount The amount of tokens to be rescued.
     * @param to Where to send rescued tokens
     */
    function rescueTokens(address token, uint256 amount, address to)
        external
        nonReentrant
        onlyRole(DEFAULT_ADMIN_ROLE)
    {
        if (address(token) == asset()) revert InvalidToken();
        IERC20(token).safeTransfer(to, amount);
    }

    /**
     * @dev Burns the full restricted user amount and mints to the desired owner address.
     * @param from The address to burn the entire balance, with the FULL_RESTRICTED_STAKER_ROLE
     * @param to The address to mint the entire balance of "from" parameter.
     */
    function redistributeLockedAmount(address from, address to) external nonReentrant onlyRole(DEFAULT_ADMIN_ROLE) {
        if (hasRole(FULL_RESTRICTED_STAKER_ROLE, from) && !hasRole(FULL_RESTRICTED_STAKER_ROLE, to)) {
            uint256 amountToDistribute = balanceOf(from);
            uint256 iTryToVest = previewRedeem(amountToDistribute);
            _burn(from, amountToDistribute);
            _checkMinShares();
            // to address of address(0) enables burning
            if (to == address(0)) {
                _updateVestingAmount(iTryToVest);
            } else {
                _mint(to, amountToDistribute);
            }

            emit LockedAmountRedistributed(from, to, amountToDistribute);
        } else {
            revert OperationNotAllowed();
        }
    }

    /* ------------- PUBLIC ------------- */

    /**
     * @notice Returns the amount of iTry tokens that are vested in the contract.
     */
    function totalAssets() public view override returns (uint256) {
        return IERC20(asset()).balanceOf(address(this)) - getUnvestedAmount();
    }

    /**
     * @notice Returns the amount of iTry tokens that are unvested in the contract.
     */
    function getUnvestedAmount() public view returns (uint256) {
        uint256 timeSinceLastDistribution = block.timestamp - lastDistributionTimestamp;

        if (timeSinceLastDistribution >= vestingPeriod) {
            return 0;
        }

        uint256 deltaT;
        unchecked {
            deltaT = (vestingPeriod - timeSinceLastDistribution);
        }
        return (deltaT * vestingAmount) / vestingPeriod;
    }

    /// @dev Necessary because both ERC20 (from ERC20Permit) and ERC4626 declare decimals()
    function decimals() public pure override(ERC4626, ERC20) returns (uint8) {
        return 18;
    }

    /**
     * @notice Returns the current vesting period.
     */
    function getVestingPeriod() public view returns (uint256) {
        return vestingPeriod;
    }

    /* ------------- INTERNAL ------------- */

    /// @notice ensures a small non-zero amount of shares does not remain, exposing to donation attack
    function _checkMinShares() internal view {
        uint256 _totalSupply = totalSupply();
        if (_totalSupply > 0 && _totalSupply < MIN_SHARES) revert MinSharesViolation();
    }

    /**
     * @dev Deposit/mint common workflow.
     * @param caller sender of assets
     * @param receiver where to send shares
     * @param assets assets to deposit
     * @param shares shares to mint
     */
    function _deposit(address caller, address receiver, uint256 assets, uint256 shares)
        internal
        override
        nonReentrant
        notZero(assets)
        notZero(shares)
    {
        if (hasRole(SOFT_RESTRICTED_STAKER_ROLE, caller) || hasRole(SOFT_RESTRICTED_STAKER_ROLE, receiver)) {
            revert OperationNotAllowed();
        }
        super._deposit(caller, receiver, assets, shares);
        _checkMinShares();
    }

    /**
     * @dev Withdraw/redeem common workflow.
     * @param caller tx sender
     * @param receiver where to send assets
     * @param _owner where to burn shares from
     * @param assets asset amount to transfer out
     * @param shares shares to burn
     */
    function _withdraw(address caller, address receiver, address _owner, uint256 assets, uint256 shares)
        internal
        override
        nonReentrant
        notZero(assets)
        notZero(shares)
    {
        if (
            hasRole(FULL_RESTRICTED_STAKER_ROLE, caller) || hasRole(FULL_RESTRICTED_STAKER_ROLE, receiver)
                || hasRole(FULL_RESTRICTED_STAKER_ROLE, _owner)
        ) {
            revert OperationNotAllowed();
        }

        super._withdraw(caller, receiver, _owner, assets, shares);
        _checkMinShares();
    }

    function _updateVestingAmount(uint256 newVestingAmount) internal {
        if (getUnvestedAmount() > 0) revert StillVesting();

        vestingAmount = newVestingAmount;
        lastDistributionTimestamp = block.timestamp;
    }

    /**
     * @dev Hook that is called before any transfer of tokens. This includes
     * minting and burning. Disables transfers from or to of addresses with the FULL_RESTRICTED_STAKER_ROLE role.
     */

    function _beforeTokenTransfer(address from, address to, uint256) internal virtual override {
        if (hasRole(FULL_RESTRICTED_STAKER_ROLE, from) && to != address(0)) {
            revert OperationNotAllowed();
        }
        if (hasRole(FULL_RESTRICTED_STAKER_ROLE, to)) {
            revert OperationNotAllowed();
        }
    }

    /**
     * @dev Remove renounce role access from AccessControl, to prevent users to resign roles.
     */
    function renounceRole(bytes32, address) public virtual override {
        revert OperationNotAllowed();
    }
}

// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.20;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {IiTrySiloDefinitions} from "./interfaces/IiTrySiloDefinitions.sol";

/**
 * @title iTrySilo
 * @notice The Silo allows to store iTry during the stake cooldown process.
 */
contract iTrySilo is IiTrySiloDefinitions {
    using SafeERC20 for IERC20;

    address immutable STAKING_VAULT;
    IERC20 immutable iTry;

    constructor(address _stakingVault, address _iTryToken) {
        STAKING_VAULT = _stakingVault;
        iTry = IERC20(_iTryToken);
    }

    modifier onlyStakingVault() {
        if (msg.sender != STAKING_VAULT) revert OnlyStakingVault();
        _;
    }

    function withdraw(address to, uint256 amount) external onlyStakingVault {
        iTry.transfer(to, amount);
    }
}

// SPDX-License-Identifier: MIT

pragma solidity 0.8.20;

import {IStakediTry} from "./IStakediTry.sol";

struct UserCooldown {
    uint104 cooldownEnd;
    uint152 underlyingAmount;
}

interface IStakediTryCooldown is IStakediTry {
    // Events //
    /// @notice Event emitted when cooldown duration updates
    event CooldownDurationUpdated(uint24 previousDuration, uint24 newDuration);

    // Errors //
    /// @notice Error emitted when the shares amount to redeem is greater than the shares balance of the owner
    error ExcessiveRedeemAmount();
    /// @notice Error emitted when the shares amount to withdraw is greater than the shares balance of the owner
    error ExcessiveWithdrawAmount();
    /// @notice Error emitted when cooldown value is invalid
    error InvalidCooldown();

    function cooldownAssets(uint256 assets) external returns (uint256 shares);

    function cooldownShares(uint256 shares) external returns (uint256 assets);

    function unstake(address receiver) external;

    function setCooldownDuration(uint24 duration) external;
}

// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v4.9.0) (security/ReentrancyGuard.sol)

pragma solidity ^0.8.0;

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

    uint256 private _status;

    constructor() {
        _status = _NOT_ENTERED;
    }

    /**
     * @dev Prevents a contract from calling itself, directly or indirectly.
     * Calling a `nonReentrant` function from another `nonReentrant`
     * function is not supported. It is possible to prevent this from happening
     * by making the `nonReentrant` function external, and making it call a
     * `private` function that does the actual work.
     */
    modifier nonReentrant() {
        _nonReentrantBefore();
        _;
        _nonReentrantAfter();
    }

    function _nonReentrantBefore() private {
        // On the first call to nonReentrant, _status will be _NOT_ENTERED
        require(_status != _ENTERED, "ReentrancyGuard: reentrant call");

        // Any calls to nonReentrant after this point will fail
        _status = _ENTERED;
    }

    function _nonReentrantAfter() private {
        // By storing the original value once again, a refund is triggered (see
        // https://eips.ethereum.org/EIPS/eip-2200)
        _status = _NOT_ENTERED;
    }

    /**
     * @dev Returns true if the reentrancy guard is currently set to "entered", which indicates there is a
     * `nonReentrant` function in the call stack.
     */
    function _reentrancyGuardEntered() internal view returns (bool) {
        return _status == _ENTERED;
    }
}

// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.20;

import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {IERC5313} from "@openzeppelin/contracts/interfaces/IERC5313.sol";
import {ISingleAdminAccessControl} from "./ISingleAdminAccessControl.sol";

/**
 * @title SingleAdminAccessControl
 * @notice SingleAdminAccessControl is a contract that provides a single admin role
 * @notice This contract is a simplified alternative to OpenZeppelin's AccessControlDefaultAdminRules
 */
abstract contract SingleAdminAccessControl is IERC5313, ISingleAdminAccessControl, AccessControl {
    address private _currentDefaultAdmin;
    address private _pendingDefaultAdmin;

    modifier notAdmin(bytes32 role) {
        if (role == DEFAULT_ADMIN_ROLE) revert InvalidAdminChange();
        _;
    }

    /// @notice Transfer the admin role to a new address
    /// @notice This can ONLY be executed by the current admin
    /// @param newAdmin address
    function transferAdmin(address newAdmin) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (newAdmin == msg.sender) revert InvalidAdminChange();
        _pendingDefaultAdmin = newAdmin;
        emit AdminTransferRequested(_currentDefaultAdmin, newAdmin);
    }

    function acceptAdmin() external {
        if (msg.sender != _pendingDefaultAdmin) revert NotPendingAdmin();
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
    }

    /// @notice grant a role
    /// @notice can only be executed by the current single admin
    /// @notice admin role cannot be granted externally
    /// @param role bytes32
    /// @param account address
    function grantRole(bytes32 role, address account) public override onlyRole(DEFAULT_ADMIN_ROLE) notAdmin(role) {
        _grantRole(role, account);
    }

    /// @notice revoke a role
    /// @notice can only be executed by the current admin
    /// @notice admin role cannot be revoked
    /// @param role bytes32
    /// @param account address
    function revokeRole(bytes32 role, address account) public override onlyRole(DEFAULT_ADMIN_ROLE) notAdmin(role) {
        _revokeRole(role, account);
    }

    /// @notice renounce the role of msg.sender
    /// @notice admin role cannot be renounced
    /// @param role bytes32
    /// @param account address
    function renounceRole(bytes32 role, address account) public virtual override notAdmin(role) {
        super.renounceRole(role, account);
    }

    /**
     * @dev See {IERC5313-owner}.
     */
    function owner() public view virtual returns (address) {
        return _currentDefaultAdmin;
    }

    /**
     * @notice no way to change admin without removing old admin first
     */
    function _grantRole(bytes32 role, address account) internal override {
        if (role == DEFAULT_ADMIN_ROLE) {
            emit AdminTransferred(_currentDefaultAdmin, account);
            _revokeRole(DEFAULT_ADMIN_ROLE, _currentDefaultAdmin);
            _currentDefaultAdmin = account;
            delete _pendingDefaultAdmin;
        }
        super._grantRole(role, account);
    }
}

// SPDX-License-Identifier: MIT

pragma solidity 0.8.20;

interface IStakediTry {
    // Events //
    /// @notice Event emitted when the rewards are received
    event RewardsReceived(uint256 amount);
    /// @notice Event emitted when the balance from an FULL_RESTRICTED_STAKER_ROLE user are redistributed
    event LockedAmountRedistributed(address indexed from, address indexed to, uint256 amount);
    /// @notice Event emitted when the vesting period is updated
    event VestingPeriodUpdated(uint256 indexed oldVestingPeriod, uint256 indexed newVestingPeriod);

    // Errors //
    /// @notice Error emitted shares or assets equal zero.
    error InvalidAmount();
    /// @notice Error emitted when owner attempts to rescue iTry tokens.
    error InvalidToken();
    /// @notice Error emitted when a small non-zero share amount remains, which risks donations attack
    error MinSharesViolation();
    /// @notice Error emitted when owner is not allowed to perform an operation
    error OperationNotAllowed();
    /// @notice Error emitted when there is still unvested amount
    error StillVesting();
    /// @notice Error emitted when owner or blacklist manager attempts to blacklist owner
    error CantBlacklistOwner();
    /// @notice Error emitted when the zero address is given
    error InvalidZeroAddress();
    /// @notice Error emitted when an invalid vesting period is provided
    error InvalidVestingPeriod();

    function transferInRewards(uint256 amount) external;

    function rescueTokens(address token, uint256 amount, address to) external;

    function getUnvestedAmount() external view returns (uint256);

    function setVestingPeriod(uint256 _vestingPeriod) external;

    function getVestingPeriod() external view returns (uint256);
}

// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.20;

interface IiTrySiloDefinitions {
    /// @notice Error emitted when the staking vault is not the caller
    error OnlyStakingVault();
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

interface ISingleAdminAccessControl {
    error InvalidAdminChange();
    error NotPendingAdmin();

    event AdminTransferred(address indexed oldAdmin, address indexed newAdmin);
    event AdminTransferRequested(address indexed oldAdmin, address indexed newAdmin);
}


## SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS
// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

// TEMPORARILY COMMENTED OUT - Missing redstone-oracles-monorepo submodule
// import "@redstone-finance/evm-connector/contracts/data-services/MainDemoConsumerBase.sol";
import {IOracle} from "./periphery/IOracle.sol";

// TEMPORARILY COMMENTED OUT - Missing redstone-oracles-monorepo submodule
// contract RedstoneNAVFeed is IOracle, MainDemoConsumerBase {
contract RedstoneNAVFeed is IOracle {
    uint256 private _price;

    // Mock implementation
    function price() external view returns (uint256) {
        // return getOracleNumericValueFromTxMsg(bytes32("NAV")); // Exact Identifier TBD
        return _price;
    }

    function setPrice(uint256 newPrice) external {
        _price = newPrice;
    }
}

// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.20;

import "@openzeppelin/contracts-upgradeable/access/AccessControlUpgradeable.sol";
import "@openzeppelin/contracts/interfaces/IERC5313.sol";
import "./ISingleAdminAccessControl.sol";

/**
 * @title SingleAdminAccessControlUpgradeable
 * @notice SingleAdminAccessControlUpgradeable is a contract that provides a single admin role
 * @notice This contract is a simplified alternative to OpenZeppelin's AccessControlDefaultAdminRules
 */
abstract contract SingleAdminAccessControlUpgradeable is IERC5313, ISingleAdminAccessControl, AccessControlUpgradeable {
    address private _currentDefaultAdmin;
    address private _pendingDefaultAdmin;

    modifier notAdmin(bytes32 role) {
        if (role == DEFAULT_ADMIN_ROLE) revert InvalidAdminChange();
        _;
    }

    /// @notice Transfer the admin role to a new address
    /// @notice This can ONLY be executed by the current admin
    /// @param newAdmin address
    function transferAdmin(address newAdmin) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (newAdmin == msg.sender) revert InvalidAdminChange();
        _pendingDefaultAdmin = newAdmin;
        emit AdminTransferRequested(_currentDefaultAdmin, newAdmin);
    }

    function acceptAdmin() external {
        if (msg.sender != _pendingDefaultAdmin) revert NotPendingAdmin();
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
    }

    /// @notice grant a role
    /// @notice can only be executed by the current single admin
    /// @notice admin role cannot be granted externally
    /// @param role bytes32
    /// @param account address
    function grantRole(bytes32 role, address account) public override onlyRole(DEFAULT_ADMIN_ROLE) notAdmin(role) {
        _grantRole(role, account);
    }

    /// @notice revoke a role
    /// @notice can only be executed by the current admin
    /// @notice admin role cannot be revoked
    /// @param role bytes32
    /// @param account address
    function revokeRole(bytes32 role, address account) public override onlyRole(DEFAULT_ADMIN_ROLE) notAdmin(role) {
        _revokeRole(role, account);
    }

    /// @notice renounce the role of msg.sender
    /// @notice admin role cannot be renounced
    /// @param role bytes32
    /// @param account address
    function renounceRole(bytes32 role, address account) public virtual override notAdmin(role) {
        super.renounceRole(role, account);
    }

    /**
     * @dev See {IERC5313-owner}.
     */
    function owner() public view virtual returns (address) {
        return _currentDefaultAdmin;
    }

    /**
     * @notice no way to change admin without removing old admin first
     */
    function _grantRole(bytes32 role, address account) internal override {
        if (role == DEFAULT_ADMIN_ROLE) {
            emit AdminTransferred(_currentDefaultAdmin, account);
            _revokeRole(DEFAULT_ADMIN_ROLE, _currentDefaultAdmin);
            _currentDefaultAdmin = account;
            delete _pendingDefaultAdmin;
        }
        super._grantRole(role, account);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {IFastAccessVault} from "./interfaces/IFastAccessVault.sol";
import {IiTryIssuer} from "./interfaces/IiTryIssuer.sol";
import {CommonErrors} from "./periphery/CommonErrors.sol";

/**
 * @title FastAccessVault
 * @author Inverter Network
 * @notice Liquidity buffer vault for instant iTRY redemptions without custodian delays
 * @dev This contract maintains a configurable percentage of total DLF collateral to enable
 *      instant redemptions. It automatically rebalances between itself and the custodian to
 *      maintain optimal liquidity levels.
 *
 *      Key features:
 *      - Holds buffer of DLF tokens for instant redemptions
 *      - Automatic rebalancing based on target percentage of AUM
 *      - Minimum balance floor to ensure always-available liquidity
 *      - Fixed reference to authorized issuer contract for access control
 *      - Emergency token rescue functionality
 *
 *      The vault uses a two-tier sizing strategy:
 *      1. Target percentage: Buffer = AUM * targetBufferPercentageBPS / 10000
 *      2. Minimum balance: Buffer = max(calculated_target, minimumExpectedBalance)
 *
 * @custom:security-contact security@inverter.network
 */
contract FastAccessVault is IFastAccessVault, Ownable, ReentrancyGuard {
    // ============================================
    using SafeERC20 for IERC20;

    // ============================================
    // Constants
    // ============================================

    /// @notice Maximum buffer percentage in basis points (100% = 10000 BPS)
    uint256 public constant MAX_BUFFER_PCT_BPS = 10000;

    // ============================================
    // State Variables
    // ============================================

    /// @notice The vault token (DLF) that this contract holds for redemptions
    IERC20 public immutable _vaultToken;

    /// @notice The authorized issuer contract that can withdraw from the vault
    IiTryIssuer public immutable _issuerContract;

    /// @notice The custodian address for requesting/transferring excess funds
    address public custodian;

    /// @notice Target buffer size as percentage of total AUM in basis points (e.g., 500 = 5%)
    uint256 public targetBufferPercentageBPS;

    /// @notice Minimum balance to target regardless of percentage calculation
    uint256 public minimumExpectedBalance;

    // ============================================
    // Modifiers
    // ============================================

    /**
     * @notice Restricts function access to the authorized issuer contract only
     */
    modifier onlyIssuer() {
        if (msg.sender != address(_issuerContract)) {
            revert UnauthorizedCaller(msg.sender);
        }
        _;
    }

    // ============================================
    // Constructor
    // ============================================

    /**
     * @notice Initializes the FastAccessVault with token, issuer, custodian, and buffer parameters
     * @param __vaultToken Address of the vault token (DLF)
     * @param __issuerContract Address of the authorized issuer contract
     * @param _custodian Address of the custodian (where transfer requests are sent)
     * @param _initialTargetPercentageBPS Initial target buffer percentage in basis points (max 10000 = 100%)
     * @param _minimumExpectedBalance Initial minimum balance to maintain in vault
     * @param _initialAdmin Address to receive ownership of the vault
     */
    constructor(
        address __vaultToken,
        address __issuerContract,
        address _custodian,
        uint256 _initialTargetPercentageBPS,
        uint256 _minimumExpectedBalance,
        address _initialAdmin
    ) {
        if (__vaultToken == address(0)) revert CommonErrors.ZeroAddress();
        if (__issuerContract == address(0)) revert CommonErrors.ZeroAddress();
        if (_custodian == address(0)) revert CommonErrors.ZeroAddress();
        if (_initialAdmin == address(0)) revert CommonErrors.ZeroAddress();

        _validateBufferPercentageBPS(_initialTargetPercentageBPS);

        _vaultToken = IERC20(__vaultToken);
        _issuerContract = IiTryIssuer(__issuerContract);
        custodian = _custodian;
        targetBufferPercentageBPS = _initialTargetPercentageBPS;
        minimumExpectedBalance = _minimumExpectedBalance;

        // Transfer ownership to the initial admin
        transferOwnership(_initialAdmin);
    }

    // ============================================
    // View Functions
    // ============================================

    /// @inheritdoc IFastAccessVault
    function getAvailableBalance() public view returns (uint256) {
        return _vaultToken.balanceOf(address(this));
    }

    /// @inheritdoc IFastAccessVault
    function getIssuerContract() external view returns (address) {
        return address(_issuerContract);
    }

    /// @inheritdoc IFastAccessVault
    function getTargetBufferPercentage() external view returns (uint256) {
        return targetBufferPercentageBPS;
    }

    /// @inheritdoc IFastAccessVault
    function getMinimumBufferBalance() external view returns (uint256) {
        return minimumExpectedBalance;
    }

    // ============================================
    // State-Changing Functions - Transfer Operations
    // ============================================

    /// @inheritdoc IFastAccessVault
    function processTransfer(address _receiver, uint256 _amount) external onlyIssuer {
        if (_receiver == address(0)) revert CommonErrors.ZeroAddress();
        if (_receiver == address(this)) revert InvalidReceiver(_receiver);
        if (_amount == 0) revert CommonErrors.ZeroAmount();

        uint256 currentBalance = _vaultToken.balanceOf(address(this));
        if (currentBalance < _amount) {
            revert InsufficientBufferBalance(_amount, currentBalance);
        }

        if (!_vaultToken.transfer(_receiver, _amount)) {
            revert CommonErrors.TransferFailed();
        }
        emit TransferProcessed(_receiver, _amount, (currentBalance - _amount));
    }

    // ============================================
    // State-Changing Functions - Rebalancing
    // ============================================

    /// @inheritdoc IFastAccessVault
    function rebalanceFunds() external {
        uint256 aumReferenceValue = _issuerContract.getCollateralUnderCustody();
        uint256 targetBalance = _calculateTargetBufferBalance(aumReferenceValue);
        uint256 currentBalance = _vaultToken.balanceOf(address(this));

        if (currentBalance < targetBalance) {
            uint256 needed = targetBalance - currentBalance;
            // Emit event for off-chain custodian to process
            emit TopUpRequestedFromCustodian(address(custodian), needed, targetBalance);
        } else if (currentBalance > targetBalance) {
            uint256 excess = currentBalance - targetBalance;
            if (!_vaultToken.transfer(custodian, excess)) {
                revert CommonErrors.TransferFailed();
            }
            emit ExcessFundsTransferredToCustodian(address(custodian), excess, targetBalance);
        }
    }

    // ============================================
    // Admin Functions - Configuration
    // ============================================

    /// @inheritdoc IFastAccessVault
    function setTargetBufferPercentage(uint256 newTargetPercentageBPS) external onlyOwner {
        _validateBufferPercentageBPS(newTargetPercentageBPS);

        uint256 oldPercentageBPS = targetBufferPercentageBPS;
        targetBufferPercentageBPS = newTargetPercentageBPS;
        emit TargetBufferPercentageUpdated(oldPercentageBPS, newTargetPercentageBPS);
    }

    /// @inheritdoc IFastAccessVault
    function setMinimumBufferBalance(uint256 newMinimumBufferBalance) external onlyOwner {
        uint256 oldMinimumBalance = minimumExpectedBalance;
        minimumExpectedBalance = newMinimumBufferBalance;
        emit MinimumBufferBalanceUpdated(oldMinimumBalance, newMinimumBufferBalance);
    }

    // ============================================
    // Admin Functions - Emergency/Rescue
    // ============================================

    /**
     * @notice Rescue tokens accidentally sent to this contract
     * @dev Only callable by owner. Can rescue both ERC20 tokens and native ETH
     *      Use address(0) for rescuing ETH
     * @param token The token address to rescue (use address(0) for ETH)
     * @param to The address to send rescued tokens to
     * @param amount The amount to rescue
     */
    function rescueToken(address token, address to, uint256 amount) external onlyOwner nonReentrant {
        if (to == address(0)) revert CommonErrors.ZeroAddress();
        if (amount == 0) revert CommonErrors.ZeroAmount();

        if (token == address(0)) {
            // Rescue ETH
            (bool success,) = to.call{value: amount}("");
            if (!success) revert CommonErrors.TransferFailed();
        } else {
            // Rescue ERC20 tokens
            IERC20(token).safeTransfer(to, amount);
        }

        emit TokenRescued(token, to, amount);
    }

    // ============================================
    // Internal Functions
    // ============================================

    /**
     * @notice Calculate the target buffer balance based on reference AUM
     * @dev Uses the larger of: (referenceAUM * targetPercentage) or minimumExpectedBalance
     * @param _referenceAUM The total assets under management to base calculation on
     * @return The calculated target buffer balance
     */
    function _calculateTargetBufferBalance(uint256 _referenceAUM) internal view returns (uint256) {
        uint256 targetBufferBalance = (_referenceAUM * targetBufferPercentageBPS) / 10000;
        return (targetBufferBalance < minimumExpectedBalance) ? minimumExpectedBalance : targetBufferBalance;
    }

    /**
     * @notice Validate that buffer percentage is within acceptable range
     * @dev Internal function to consolidate BPD validation logic (DRY principle)
     * @param bps The buffer percentage in basis points to validate
     */
    function _validateBufferPercentageBPS(uint256 bps) internal pure {
        if (bps > MAX_BUFFER_PCT_BPS) revert PercentageTooHigh(bps, MAX_BUFFER_PCT_BPS);
    }

    /**
     * @notice Update the custodian address
     * @dev Only callable by owner
     * @param newCustodian The new custodian address
     */
    function setCustodian(address newCustodian) external onlyOwner {
        if (newCustodian == address(0)) revert CommonErrors.ZeroAddress();

        address oldCustodian = custodian;
        custodian = newCustodian;
        emit CustodianUpdated(oldCustodian, newCustodian);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {SingleAdminAccessControl} from "../utils/SingleAdminAccessControl.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import {IiTryIssuer} from "./interfaces/IiTryIssuer.sol";
import {IiTryToken} from "../token/iTRY/interfaces/IiTryToken.sol";
import {IFastAccessVault} from "./interfaces/IFastAccessVault.sol";
import {FastAccessVault} from "./FastAccessVault.sol";
import {IOracle} from "./periphery/IOracle.sol";
import {IYieldProcessor} from "./periphery/IYieldProcessor.sol";
import {CommonErrors} from "./periphery/CommonErrors.sol";

/**
 * @title iTryIssuer
 * @author Inverter Network
 * @notice Central issuer contract for iTRY stablecoins, managing minting, redemption, and yield distribution
 * @dev This contract acts as the controller for iTRY token supply, handling:
 *      - Minting iTRY against DLF collateral
 *      - Redeeming iTRY for DLF (from buffer vault or custodian)
 *      - Processing accumulated yield from NAV appreciation
 *      - Managing fees and whitelisted users
 *      - Coordinating between liquidity vault and custodian for collateral management
 *
 *
 *      The contract uses a role-based access control system with the following roles:
 *      - DEFAULT_ADMIN_ROLE: Can manage all roles and contract parameters
 *      - WHITELIST_MANAGER_ROLE: Can add/remove users from whitelist
 *      - YIELD_DISTRIBUTOR_ROLE: Can process and distribute yield
 *      - INTEGRATION_MANAGER_ROLE: Can set integration contract addresses
 *      - WHITELISTED_USER_ROLE: Can mint and redeem iTRY tokens
 *
 * @custom:security-contact security@inverter.network
 */

contract iTryIssuer is IiTryIssuer, SingleAdminAccessControl, ReentrancyGuard {
    using SafeERC20 for IERC20;

    // ============================================
    // Constants
    // ============================================

    /// @notice Maximum mint fee in basis points (100% = 10000 BPS)
    uint256 public constant MAX_MINT_FEE_BPS = 9999;

    /// @notice Maximum redeem fee in basis points (100% = 10000 BPS)
    uint256 public constant MAX_REDEEM_FEE_BPS = 9999;

    // ============================================
    // State Variables - Dependencies
    // ============================================

    /// @notice The iTRY token contract
    IiTryToken public immutable iTryToken;

    /// @notice The DLF collateral token contract
    IERC20 public immutable collateralToken;

    /// @notice The fast access liquidity vault for quick redemptions
    IFastAccessVault public immutable liquidityVault;

    /// @notice The oracle providing NAV price for DLF/iTRY conversion
    IOracle public oracle;

    /// @notice The custodian address for off-chain collateral management
    address public custodian;

    /// @notice The yield processor contract for distributing accumulated yield
    IYieldProcessor public yieldReceiver;

    // ============================================
    // State Variables - Fee Configuration
    // ============================================

    /// @notice Address receiving protocol fees
    address public treasury;

    /// @notice Mint fee in basis points (1 BPS = 0.01%)
    uint256 public mintFeeInBPS;

    /// @notice Redemption fee in basis points (1 BPS = 0.01%)
    uint256 public redemptionFeeInBPS;

    // ============================================
    // State Variables - Accounting
    // ============================================

    /// @notice Total amount of iTRY tokens currently in circulation
    uint256 private _totalIssuedITry;

    /// @notice Total amount of DLF collateral held under custody (vault + custodian)
    uint256 private _totalDLFUnderCustody;

    // ============================================
    // Access Control Roles
    // ============================================

    /// @notice Role that can manage the whitelist of users
    bytes32 private constant _WHITELIST_MANAGER_ROLE = keccak256("WHITELIST_MANAGER_ROLE");

    /// @notice Role that can mint new yield and distribute it
    bytes32 private constant _YIELD_DISTRIBUTOR_ROLE = keccak256("YIELD_DISTRIBUTOR_ROLE");

    /// @notice Role that can set the addresses of integration contracts
    bytes32 private constant _INTEGRATION_MANAGER_ROLE = keccak256("INTEGRATION_MANAGER_ROLE");

    /// @notice Role for whitelisted users who can mint and redeem iTRY
    bytes32 private constant _WHITELISTED_USER_ROLE = keccak256("WHITELISTED_USER_ROLE");

    // ============================================
    // Constructor
    // ============================================

    /**
     * @notice Initializes the iTryIssuer contract with all necessary dependencies
     * @param _iTryToken Address of the iTRY token contract
     * @param _collateralToken Address of the DLF collateral token
     * @param _oracle Address of the NAV price oracle
     * @param _treasury Address to receive protocol fees
     * @param _yieldReceiver Address of the yield processor contract
     * @param _custodian Address of the custodian integration
     * @param _initialAdmin Address to receive all initial admin roles
     * @param _initialIssued Initial amount of iTRY already issued (for migration scenarios)
     * @param _initialDLFUnderCustody Initial amount of DLF collateral under custody (for migration scenarios)
     * @param _vaultTargetPercentageBPS Target buffer percentage for FastAccessVault in basis points
     * @param _vaultMinimumBalance Minimum balance for FastAccessVault to maintain
     */
    constructor(
        address _iTryToken,
        address _collateralToken,
        address _oracle,
        address _treasury,
        address _yieldReceiver,
        address _custodian,
        address _initialAdmin,
        uint256 _initialIssued,
        uint256 _initialDLFUnderCustody,
        uint256 _vaultTargetPercentageBPS,
        uint256 _vaultMinimumBalance
    ) {
        if (_initialAdmin == address(0)) revert CommonErrors.ZeroAddress();
        if (_iTryToken == address(0)) revert CommonErrors.ZeroAddress();
        if (_collateralToken == address(0)) revert CommonErrors.ZeroAddress();

        // Deploy FastAccessVault internally with this contract as the issuer
        liquidityVault = IFastAccessVault(
            address(
                new FastAccessVault(
                    _collateralToken,
                    address(this), // Issuer is this contract
                    _custodian,
                    _vaultTargetPercentageBPS,
                    _vaultMinimumBalance,
                    _initialAdmin // Admin for vault ownership
                )
            )
        );

        iTryToken = IiTryToken(_iTryToken);
        collateralToken = IERC20(_collateralToken);
        _setOracle(_oracle);
        _setTreasury(_treasury);
        _setYieldReceiver(_yieldReceiver);
        _setCustodian(_custodian);

        // Set initial fees to 0
        redemptionFeeInBPS = 0;
        mintFeeInBPS = 0;

        // Set initial issued amount and collateral under custody
        _totalIssuedITry = _initialIssued;
        _totalDLFUnderCustody = _initialDLFUnderCustody;

        // Initial role setup - grant all roles to initial admin
        _grantRole(DEFAULT_ADMIN_ROLE, _initialAdmin);
        _grantRole(_WHITELIST_MANAGER_ROLE, _initialAdmin);
        _grantRole(_YIELD_DISTRIBUTOR_ROLE, _initialAdmin);
        _grantRole(_INTEGRATION_MANAGER_ROLE, _initialAdmin);

        // Note: The iTRY token admin should call addMinter(address(this)) to grant this contract the MINTER_CONTRACT role
    }

    // ============================================
    // View Functions - Minting Preview
    // ============================================

    /// @inheritdoc IiTryIssuer
    function previewMint(uint256 dlfAmount) external view returns (uint256 iTRYAmount) {
        if (dlfAmount == 0) revert CommonErrors.ZeroAmount();

        uint256 navPrice = oracle.price();
        uint256 netDlfAmount = dlfAmount;

        netDlfAmount = dlfAmount - _calculateMintFee(dlfAmount);

        // Calculate iTRY amount: netDlfAmount * navPrice / 1e18
        iTRYAmount = netDlfAmount * navPrice / 1e18;
    }

    // ============================================
    // View Functions - Redemption Preview
    // ============================================

    /// @inheritdoc IiTryIssuer
    function previewRedeem(uint256 iTRYAmount) external view returns (uint256 dlfAmount) {
        if (iTRYAmount == 0) revert CommonErrors.ZeroAmount();

        uint256 navPrice = oracle.price();

        // Calculate gross DLF amount: iTRYAmount * 1e18 / navPrice
        uint256 grossDlfAmount = iTRYAmount * 1e18 / navPrice;

        // Account for redemption fee if configured
        if (redemptionFeeInBPS > 0) {
            dlfAmount = grossDlfAmount - _calculateRedemptionFee(grossDlfAmount);
        } else {
            dlfAmount = grossDlfAmount;
        }

        return dlfAmount;
    }

    // ============================================
    // View Functions - Yield Preview
    // ============================================

    /// @inheritdoc IiTryIssuer
    function previewAccumulatedYield() external view returns (uint256) {
        uint256 navPrice = oracle.price();

        // Calculate total collateral value: _totalDLFUnderCustody * currentNAVPrice / 1e18
        uint256 currentCollateralValue = _totalDLFUnderCustody * navPrice / 1e18;
        if (currentCollateralValue <= _totalIssuedITry) {
            return 0;
        }
        return currentCollateralValue - _totalIssuedITry;
    }

    // ============================================
    // View Functions - Accounting
    // ============================================

    /// @inheritdoc IiTryIssuer
    function getTotalIssuedITry() external view returns (uint256) {
        return _totalIssuedITry;
    }

    /// @inheritdoc IiTryIssuer
    function getCollateralUnderCustody() external view returns (uint256) {
        return _totalDLFUnderCustody;
    }

    /// @inheritdoc IiTryIssuer
    function isWhitelistedUser(address user) external view returns (bool) {
        return hasRole(_WHITELISTED_USER_ROLE, user);
    }

    // ============================================
    // State-Changing Functions - Minting
    // ============================================

    /// @inheritdoc IiTryIssuer
    function mintITRY(uint256 dlfAmount, uint256 minAmountOut) external returns (uint256 iTRYAmount) {
        return mintFor(msg.sender, dlfAmount, minAmountOut);
    }

    /// @inheritdoc IiTryIssuer
    function mintFor(address recipient, uint256 dlfAmount, uint256 minAmountOut)
        public
        onlyRole(_WHITELISTED_USER_ROLE)
        nonReentrant
        returns (uint256 iTRYAmount)
    {
        // Validate recipient address
        if (recipient == address(0)) revert CommonErrors.ZeroAddress();

        // Validate dlfAmount > 0
        if (dlfAmount == 0) revert CommonErrors.ZeroAmount();

        // Get NAV price from oracle
        uint256 navPrice = oracle.price();
        if (navPrice == 0) revert InvalidNAVPrice(navPrice);

        uint256 feeAmount = _calculateMintFee(dlfAmount);
        uint256 netDlfAmount = feeAmount > 0 ? (dlfAmount - feeAmount) : dlfAmount;

        // Calculate iTRY amount: netDlfAmount * navPrice / 1e18
        iTRYAmount = netDlfAmount * navPrice / 1e18;

        if (iTRYAmount == 0) revert CommonErrors.ZeroAmount();

        // Check if output meets minimum requirement
        if (iTRYAmount < minAmountOut) {
            revert OutputBelowMinimum(iTRYAmount, minAmountOut);
        }

        // Transfer collateral into vault BEFORE minting (CEI pattern)
        _transferIntoVault(msg.sender, netDlfAmount, feeAmount);

        _mint(recipient, iTRYAmount);

        // Emit event
        emit ITRYIssued(recipient, netDlfAmount, iTRYAmount, navPrice, mintFeeInBPS);
    }

    // ============================================
    // State-Changing Functions - Redemption
    // ============================================

    /// @inheritdoc IiTryIssuer
    function redeemITRY(uint256 iTRYAmount, uint256 minAmountOut) external returns (bool fromBuffer) {
        return redeemFor(msg.sender, iTRYAmount, minAmountOut);
    }

    /// @inheritdoc IiTryIssuer
    function redeemFor(address recipient, uint256 iTRYAmount, uint256 minAmountOut)
        public
        onlyRole(_WHITELISTED_USER_ROLE)
        nonReentrant
        returns (bool fromBuffer)
    {
        // Validate recipient address
        if (recipient == address(0)) revert CommonErrors.ZeroAddress();

        // Validate iTRYAmount > 0
        if (iTRYAmount == 0) revert CommonErrors.ZeroAmount();

        if (iTRYAmount > _totalIssuedITry) {
            revert AmountExceedsITryIssuance(iTRYAmount, _totalIssuedITry);
        }

        // Get NAV price from oracle
        uint256 navPrice = oracle.price();
        if (navPrice == 0) revert InvalidNAVPrice(navPrice);

        // Calculate gross DLF amount: iTRYAmount * 1e18 / navPrice
        uint256 grossDlfAmount = iTRYAmount * 1e18 / navPrice;

        if (grossDlfAmount == 0) revert CommonErrors.ZeroAmount();

        uint256 feeAmount = _calculateRedemptionFee(grossDlfAmount);
        uint256 netDlfAmount = grossDlfAmount - feeAmount;

        // Check if output meets minimum requirement
        if (netDlfAmount < minAmountOut) {
            revert OutputBelowMinimum(netDlfAmount, minAmountOut);
        }

        _burn(msg.sender, iTRYAmount);

        // Check if buffer pool has enough DLF balance
        uint256 bufferBalance = liquidityVault.getAvailableBalance();

        if (bufferBalance >= grossDlfAmount) {
            // Buffer has enough - serve from buffer
            _redeemFromVault(recipient, netDlfAmount, feeAmount);

            fromBuffer = true;
        } else {
            // Buffer insufficient - serve from custodian
            _redeemFromCustodian(recipient, netDlfAmount, feeAmount);

            fromBuffer = false;
        }

        // Emit redemption event
        emit ITRYRedeemed(recipient, iTRYAmount, netDlfAmount, fromBuffer, redemptionFeeInBPS);
    }

 /// @inheritdoc IiTryIssuer
    function burnExcessITry(uint256 iTRYAmount)
        public
        onlyRole(DEFAULT_ADMIN_ROLE)
        nonReentrant
    {
        // Validate iTRYAmount > 0
        if (iTRYAmount == 0) revert CommonErrors.ZeroAmount();

        if (iTRYAmount > _totalIssuedITry) {
            revert AmountExceedsITryIssuance(iTRYAmount, _totalIssuedITry);
        }

        _burn(msg.sender, iTRYAmount);


        // Emit redemption event
        emit excessITryRemoved(iTRYAmount, _totalIssuedITry);
    }


    // ============================================
    // State-Changing Functions - Yield Management
    // ============================================

    /// @inheritdoc IiTryIssuer
    function processAccumulatedYield() external onlyRole(_YIELD_DISTRIBUTOR_ROLE) returns (uint256 newYield) {
        // Get current NAV price
        uint256 navPrice = oracle.price();
        if (navPrice == 0) revert InvalidNAVPrice(navPrice);

        // Calculate total collateral value: totalDLFUnderCustody * currentNAVPrice / 1e18
        uint256 currentCollateralValue = _totalDLFUnderCustody * navPrice / 1e18;

        // Calculate yield: currentCollateralValue - _totalIssuedITry
        if (currentCollateralValue <= _totalIssuedITry) {
            revert NoYieldAvailable(currentCollateralValue, _totalIssuedITry);
        }
        newYield = currentCollateralValue - _totalIssuedITry;

        // Mint yield amount to yieldReceiver contract
        _mint(address(yieldReceiver), newYield);

        // Notify yield distributor of received yield
        yieldReceiver.processNewYield(newYield);

        // Emit event
        emit YieldDistributed(newYield, address(yieldReceiver), currentCollateralValue);
    }

    // ============================================
    // Admin Functions - Fee Management
    // ============================================

    /**
     * @notice Set the redemption fee rate
     * @dev Only callable by DEFAULT_ADMIN_ROLE
     * @param newRedemptionFeeInBPS The new redemption fee in basis points (1 BPS = 0.01%)
     */
    function setRedemptionFeeInBPS(uint256 newRedemptionFeeInBPS) external onlyRole(DEFAULT_ADMIN_ROLE) {
        _validateFeeBPS(newRedemptionFeeInBPS, MAX_REDEEM_FEE_BPS);
        uint256 oldFee = redemptionFeeInBPS;
        redemptionFeeInBPS = newRedemptionFeeInBPS;
        emit RedemptionFeeUpdated(oldFee, newRedemptionFeeInBPS);
    }

    /**
     * @notice Set the mint fee rate
     * @dev Only callable by DEFAULT_ADMIN_ROLE
     * @param newMintFeeInBPS The new mint fee in basis points (1 BPS = 0.01%)
     */
    function setMintFeeInBPS(uint256 newMintFeeInBPS) external onlyRole(DEFAULT_ADMIN_ROLE) {
        _validateFeeBPS(newMintFeeInBPS, MAX_MINT_FEE_BPS);
        uint256 oldFee = mintFeeInBPS;
        mintFeeInBPS = newMintFeeInBPS;
        emit MintFeeUpdated(oldFee, newMintFeeInBPS);
    }

    // ============================================
    // Admin Functions - Integration Management
    // ============================================

    /**
     * @notice Set the address of the oracle contract
     * @dev Only callable by _INTEGRATION_MANAGER_ROLE
     * @param newOracle The address of the new oracle contract
     */
    function setOracle(address newOracle) external onlyRole(_INTEGRATION_MANAGER_ROLE) {
        _setOracle(newOracle);
    }

    /**
     * @notice Set the address of the custodian
     * @dev Only callable by _INTEGRATION_MANAGER_ROLE
     * @param newCustodian The address of the new custodian
     */
    function setCustodian(address newCustodian) external onlyRole(_INTEGRATION_MANAGER_ROLE) {
        _setCustodian(newCustodian);
    }

    /**
     * @notice Set the address of the yield receiver contract
     * @dev Only callable by _INTEGRATION_MANAGER_ROLE
     * @param newYieldReceiver The address of the new yield receiver contract
     */
    function setYieldReceiver(address newYieldReceiver) external onlyRole(_INTEGRATION_MANAGER_ROLE) {
        _setYieldReceiver(newYieldReceiver);
    }

    /**
     * @notice Internal function to set the address of the oracle contract
     * @param newOracle The address of the new oracle contract
     */
    function _setOracle(address newOracle) internal {
        if (newOracle == address(0)) revert CommonErrors.ZeroAddress();
        address oldOracle = address(oracle);
        oracle = IOracle(newOracle);
        emit OracleUpdated(oldOracle, newOracle);
    }

    /**
     * @notice Internal function to set the address of the custodian
     * @param newCustodian The address of the new custodian
     */
    function _setCustodian(address newCustodian) internal {
        if (newCustodian == address(0)) revert CommonErrors.ZeroAddress();
        address oldCustodian = custodian;
        custodian = newCustodian;
        emit CustodianUpdated(oldCustodian, newCustodian);
    }

    /**
     * @notice Internal function to set the address of the yield receiver contract
     * @param newYieldReceiver The address of the new yield receiver contract
     */
    function _setYieldReceiver(address newYieldReceiver) internal {
        if (newYieldReceiver == address(0)) revert CommonErrors.ZeroAddress();
        address oldYieldReceiver = address(yieldReceiver);
        yieldReceiver = IYieldProcessor(newYieldReceiver);
        emit YieldReceiverUpdated(oldYieldReceiver, newYieldReceiver);
    }

    /**
     * @notice Set the address of the treasury
     * @dev Only callable by _INTEGRATION_MANAGER_ROLE
     * @param newTreasury The address of the new treasury
     */
    function setTreasury(address newTreasury) external onlyRole(_INTEGRATION_MANAGER_ROLE) {
        _setTreasury(newTreasury);
    }

    /**
     * @notice Internal function to set the address of the treasury
     * @param newTreasury The address of the new treasury
     */
    function _setTreasury(address newTreasury) internal {
        if (newTreasury == address(0)) revert CommonErrors.ZeroAddress();
        address oldTreasury = treasury;
        treasury = newTreasury;
        emit TreasuryUpdated(oldTreasury, newTreasury);
    }

    /**
     * @notice Validate that fee percentage is within acceptable range
     * @dev Internal function to consolidate BPS validation logic (DRY principle)
     * @param bps The fee percentage in basis points to validate
     * @param maxBps The maximum allowed fee percentage in basis points
     */
    function _validateFeeBPS(uint256 bps, uint256 maxBps) internal pure {
        if (bps > maxBps) revert FeeTooHigh(bps, maxBps);
    }

    // ============================================
    // Admin Functions - Whitelist Management
    // ============================================

    /**
     * @notice Add an address to the whitelist, allowing them to mint and redeem iTRY
     * @dev Only callable by _WHITELIST_MANAGER_ROLE
     * @param target The address to whitelist
     */
    function addToWhitelist(address target) external onlyRole(_WHITELIST_MANAGER_ROLE) {
        _grantRole(_WHITELISTED_USER_ROLE, target);
    }

    /**
     * @notice Remove an address from the whitelist, preventing them from minting and redeeming iTRY
     * @dev Only callable by _WHITELIST_MANAGER_ROLE
     * @param target The address to remove from whitelist
     */
    function removeFromWhitelist(address target) external onlyRole(_WHITELIST_MANAGER_ROLE) {
        _revokeRole(_WHITELISTED_USER_ROLE, target);
    }

    // ============================================
    // Internal Functions - Token Operations
    // ============================================

    /**
     * @notice Internal function to mint iTRY tokens
     * @dev Updates total issued accounting and calls the iTRY token mint function
     * @param receiver The address to receive the minted tokens
     * @param amount The amount of iTRY tokens to mint
     */
    function _mint(address receiver, uint256 amount) internal {
        _totalIssuedITry += amount;
        iTryToken.mint(receiver, amount);
    }

    /**
     * @notice Internal function to burn iTRY tokens
     * @dev Updates total issued accounting and calls the iTRY token burn function
     * @param from The address whose tokens will be burned
     * @param amount The amount of iTRY tokens to burn
     */
    function _burn(address from, uint256 amount) internal {
        // Burn user's iTRY tokens
        _totalIssuedITry -= amount;
        iTryToken.burnFrom(from, amount);
    }

    // ============================================
    // Internal Functions - Collateral Management
    // ============================================

    /**
     * @notice Internal function to transfer DLF collateral from user to vault and fees to treasury
     * @dev Updates total DLF under custody and transfers tokens
     * @param from The address providing the DLF tokens
     * @param dlfAmount The net amount of DLF to transfer to the vault (after fees)
     * @param feeAmount The fee amount to transfer to treasury
     */
    function _transferIntoVault(address from, uint256 dlfAmount, uint256 feeAmount) internal {
        _totalDLFUnderCustody += dlfAmount;
        // Transfer net DLF amount to buffer pool
        if (!collateralToken.transferFrom(from, address(liquidityVault), dlfAmount)) {
            revert CommonErrors.TransferFailed();
        }

        if (feeAmount > 0) {
            // Transfer fee to treasury
            if (!collateralToken.transferFrom(from, treasury, feeAmount)) {
                revert CommonErrors.TransferFailed();
            }
            emit FeeProcessed(from, treasury, feeAmount);
        }
    }

    /**
     * @notice Internal function to process redemption from the buffer vault
     * @dev Updates total DLF under custody and instructs vault to transfer tokens
     * @param receiver The address to receive the DLF tokens
     * @param receiveAmount The net amount of DLF to transfer (after fees)
     * @param feeAmount The fee amount to transfer to treasury
     */
    function _redeemFromVault(address receiver, uint256 receiveAmount, uint256 feeAmount) internal {
        _totalDLFUnderCustody -= (receiveAmount + feeAmount);

        liquidityVault.processTransfer(receiver, receiveAmount);

        if (feeAmount > 0) {
            liquidityVault.processTransfer(treasury, feeAmount);
        }
    }

    /**
     * @notice Internal function to process redemption via custodian transfer request
     * @dev Emits events for off-chain custodian to process transfers manually
     * @param receiver The address to receive the DLF tokens
     * @param receiveAmount The net amount of DLF to transfer (after fees)
     * @param feeAmount The fee amount to transfer to treasury
     */
    function _redeemFromCustodian(address receiver, uint256 receiveAmount, uint256 feeAmount) internal {
        _totalDLFUnderCustody -= (receiveAmount + feeAmount);

        // Signal that fast access vault needs top-up from custodian
        uint256 topUpAmount = receiveAmount + feeAmount;
        emit FastAccessVaultTopUpRequested(topUpAmount);

        if (feeAmount > 0) {
            // Emit event for off-chain custodian to process
            emit CustodianTransferRequested(treasury, feeAmount);
        }

        // Emit event for off-chain custodian to process
        emit CustodianTransferRequested(receiver, receiveAmount);
    }

    // ============================================
    // Internal Functions - Fee Calculations
    // ============================================

    /**
     * @notice Calculate the mint fee for a given amount
     * @dev Fee = amount * mintFeeInBPS / 10000
     * @param amount The amount to calculate fee on
     * @return feeAmount The calculated fee amount
     */
    function _calculateMintFee(uint256 amount) internal view returns (uint256 feeAmount) {
        // Account for mint fee if configured
        if (mintFeeInBPS > 0) {
            feeAmount = amount * mintFeeInBPS / 10000;
            return feeAmount == 0 ? 1 : feeAmount; // avoid round-down to zero
        } else {
            return 0;
        }
    }

    /**
     * @notice Calculate the redemption fee for a given amount
     * @dev Fee = amount * redemptionFeeInBPS / 10000
     * @param amount The amount to calculate fee on
     * @return feeAmount The calculated fee amount
     */
    function _calculateRedemptionFee(uint256 amount) internal view returns (uint256) {
        // Account for redemption fee if configured
        if (redemptionFeeInBPS == 0) {
            return 0;
        }

        uint256 feeAmount = amount * redemptionFeeInBPS / 10000;
        return feeAmount == 0 ? 1 : feeAmount; // avoid round-down to zero
    }
}

// SPDX-License-Identifier: MIT

pragma solidity ^0.8.20;

import { ERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import { IOFT, OFTCore } from "./OFTCore.sol";

/**
 * @title OFT Contract
 * @dev OFT is an ERC-20 token that extends the functionality of the OFTCore contract.
 */
abstract contract OFT is OFTCore, ERC20 {
    /**
     * @dev Constructor for the OFT contract.
     * @param _name The name of the OFT.
     * @param _symbol The symbol of the OFT.
     * @param _lzEndpoint The LayerZero endpoint address.
     * @param _delegate The delegate capable of making OApp configurations inside of the endpoint.
     */
    constructor(
        string memory _name,
        string memory _symbol,
        address _lzEndpoint,
        address _delegate
    ) ERC20(_name, _symbol) OFTCore(decimals(), _lzEndpoint, _delegate) {}

    /**
     * @dev Retrieves the address of the underlying ERC20 implementation.
     * @return The address of the OFT token.
     *
     * @dev In the case of OFT, address(this) and erc20 are the same contract.
     */
    function token() public view returns (address) {
        return address(this);
    }

    /**
     * @notice Indicates whether the OFT contract requires approval of the 'token()' to send.
     * @return requiresApproval Needs approval of the underlying token implementation.
     *
     * @dev In the case of OFT where the contract IS the token, approval is NOT required.
     */
    function approvalRequired() external pure virtual returns (bool) {
        return false;
    }

    /**
     * @dev Burns tokens from the sender's specified balance.
     * @param _from The address to debit the tokens from.
     * @param _amountLD The amount of tokens to send in local decimals.
     * @param _minAmountLD The minimum amount to send in local decimals.
     * @param _dstEid The destination chain ID.
     * @return amountSentLD The amount sent in local decimals.
     * @return amountReceivedLD The amount received in local decimals on the remote.
     */
    function _debit(
        address _from,
        uint256 _amountLD,
        uint256 _minAmountLD,
        uint32 _dstEid
    ) internal virtual override returns (uint256 amountSentLD, uint256 amountReceivedLD) {
        (amountSentLD, amountReceivedLD) = _debitView(_amountLD, _minAmountLD, _dstEid);

        // @dev In NON-default OFT, amountSentLD could be 100, with a 10% fee, the amountReceivedLD amount is 90,
        // therefore amountSentLD CAN differ from amountReceivedLD.

        // @dev Default OFT burns on src.
        _burn(_from, amountSentLD);
    }

    /**
     * @dev Credits tokens to the specified address.
     * @param _to The address to credit the tokens to.
     * @param _amountLD The amount of tokens to credit in local decimals.
     * @dev _srcEid The source chain ID.
     * @return amountReceivedLD The amount of tokens ACTUALLY received in local decimals.
     */
    function _credit(
        address _to,
        uint256 _amountLD,
        uint32 /*_srcEid*/
    ) internal virtual override returns (uint256 amountReceivedLD) {
        if (_to == address(0x0)) _to = address(0xdead); // _mint(...) does not support address(0x0)
        // @dev Default OFT mints on dst.
        _mint(_to, _amountLD);
        // @dev In the case of NON-default OFT, the _amountLD MIGHT not be == amountReceivedLD.
        return _amountLD;
    }
}

// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {Pausable} from "@openzeppelin/contracts/security/Pausable.sol";
import {Initializable} from "@openzeppelin/contracts/proxy/utils/Initializable.sol";

contract DLFToken is Initializable, ERC20, Ownable, Pausable {
    mapping(address => bool) private _isBlacklisted;

    constructor(address owner) ERC20("Digital Liquiditiy Fund Token Mock", "DLF") {
        _transferOwnership(owner);
        _mint(owner, 1000e18); // Test mint
    }

    function pause() public onlyOwner {
        _pause();
    }

    function unpause() public onlyOwner {
        _unpause();
    }

    function _beforeTokenTransfer(address from, address to, uint256 amount) internal override whenNotPaused {
        require(!_isBlacklisted[from], "ERC20: sender is blacklisted");
        require(!_isBlacklisted[to], "ERC20: recipient is blacklisted");
        super._beforeTokenTransfer(from, to, amount);
    }

    function blacklist(address account) public onlyOwner {
        require(account != address(0), "Blacklist: account is the zero address");
        _isBlacklisted[account] = true;
    }

    function unblacklist(address account) public onlyOwner {
        require(account != address(0), "Blacklist: account is the zero address");
        _isBlacklisted[account] = false;
    }

    function isBlacklisted(address account) public view returns (bool) {
        return _isBlacklisted[account];
    }

    function mint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }

    function burn(address from, uint256 amount) public onlyOwner {
        _burn(from, amount);
    }
}

// SPDX-License-Identifier: MIT

pragma solidity ^0.8.17;

import "../mocks/RedstoneConsumerNumericMock.sol";
import "@openzeppelin/contracts/utils/Context.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @dev Implementation of the {IERC20} interface.
 *
 * This implementation is agnostic to the way tokens are created. This means
 * that a supply mechanism has to be added in a derived contract using {_mint}.
 * For a generic mechanism see {ERC20PresetMinterPauser}.
 *
 * TIP: For a detailed writeup see our guide
 * https://forum.zeppelin.solutions/t/how-to-implement-erc20-supply-mechanisms/226[How
 * to implement supply mechanisms].
 *
 * We have followed general OpenZeppelin guidelines: functions revert instead
 * of returning `false` on failure. This behavior is nonetheless conventional
 * and does not conflict with the expectations of ERC20 applications.
 *
 * Additionally, an {Approval} event is emitted on calls to {transferFrom}.
 * This allows applications to reconstruct the allowance for all accounts just
 * by listening to said events. Other implementations of the EIP may not emit
 * these events, as it isn't required by the specification.
 *
 * Finally, the non-standard {decreaseAllowance} and {increaseAllowance}
 * functions have been added to mitigate the well-known issues around setting
 * allowances. See {IERC20-approve}.
 */
contract ERC20Initializable is Context, IERC20, IERC20Metadata {
  mapping(address => uint256) private _balances;

  mapping(address => mapping(address => uint256)) private _allowances;

  uint256 private _totalSupply;

  string private _name;
  string private _symbol;

  function initialize(string memory name_, string memory symbol_) internal {
    _name = name_;
    _symbol = symbol_;
  }

  /**
   * @dev Returns the name of the token.
   */
  function name() public view virtual override returns (string memory) {
    return _name;
  }

  /**
   * @dev Returns the symbol of the token, usually a shorter version of the
   * name.
   */
  function symbol() public view virtual override returns (string memory) {
    return _symbol;
  }

  /**
   * @dev Returns the number of decimals used to get its user representation.
   * For example, if `decimals` equals `2`, a balance of `505` tokens should
   * be displayed to a user as `5,05` (`505 / 10 ** 2`).
   *
   * Tokens usually opt for a value of 18, imitating the relationship between
   * Ether and Wei. This is the value {ERC20} uses, unless this function is
   * overridden;
   *
   * NOTE: This information is only used for _display_ purposes: it in
   * no way affects any of the arithmetic of the contract, including
   * {IERC20-balanceOf} and {IERC20-transfer}.
   */
  function decimals() public view virtual override returns (uint8) {
    return 18;
  }

  /**
   * @dev See {IERC20-totalSupply}.
   */
  function totalSupply() public view virtual override returns (uint256) {
    return _totalSupply;
  }

  /**
   * @dev See {IERC20-balanceOf}.
   */
  function balanceOf(address account) public view virtual override returns (uint256) {
    return _balances[account];
  }

  /**
   * @dev See {IERC20-transfer}.
   *
   * Requirements:
   *
   * - `recipient` cannot be the zero address.
   * - the caller must have a balance of at least `amount`.
   */
  function transfer(address recipient, uint256 amount) public virtual override returns (bool) {
    _transfer(_msgSender(), recipient, amount);
    return true;
  }

  /**
   * @dev See {IERC20-allowance}.
   */
  function allowance(address owner, address spender)
    public
    view
    virtual
    override
    returns (uint256)
  {
    return _allowances[owner][spender];
  }

  /**
   * @dev See {IERC20-approve}.
   *
   * Requirements:
   *
   * - `spender` cannot be the zero address.
   */
  function approve(address spender, uint256 amount) public virtual override returns (bool) {
    _approve(_msgSender(), spender, amount);
    return true;
  }

  /**
   * @dev See {IERC20-transferFrom}.
   *
   * Emits an {Approval} event indicating the updated allowance. This is not
   * required by the EIP. See the note at the beginning of {ERC20}.
   *
   * Requirements:
   *
   * - `sender` and `recipient` cannot be the zero address.
   * - `sender` must have a balance of at least `amount`.
   * - the caller must have allowance for ``sender``'s tokens of at least
   * `amount`.
   */
  function transferFrom(
    address sender,
    address recipient,
    uint256 amount
  ) public virtual override returns (bool) {
    _transfer(sender, recipient, amount);

    uint256 currentAllowance = _allowances[sender][_msgSender()];
    require(currentAllowance >= amount, "ERC20: transfer amount exceeds allowance");
    _approve(sender, _msgSender(), currentAllowance - amount);

    return true;
  }

  /**
   * @dev Atomically increases the allowance granted to `spender` by the caller.
   *
   * This is an alternative to {approve} that can be used as a mitigation for
   * problems described in {IERC20-approve}.
   *
   * Emits an {Approval} event indicating the updated allowance.
   *
   * Requirements:
   *
   * - `spender` cannot be the zero address.
   */
  function increaseAllowance(address spender, uint256 addedValue) public virtual returns (bool) {
    _approve(_msgSender(), spender, _allowances[_msgSender()][spender] + addedValue);
    return true;
  }

  /**
   * @dev Atomically decreases the allowance granted to `spender` by the caller.
   *
   * This is an alternative to {approve} that can be used as a mitigation for
   * problems described in {IERC20-approve}.
   *
   * Emits an {Approval} event indicating the updated allowance.
   *
   * Requirements:
   *
   * - `spender` cannot be the zero address.
   * - `spender` must have allowance for the caller of at least
   * `subtractedValue`.
   */
  function decreaseAllowance(address spender, uint256 subtractedValue)
    public
    virtual
    returns (bool)
  {
    uint256 currentAllowance = _allowances[_msgSender()][spender];
    require(currentAllowance >= subtractedValue, "ERC20: decreased allowance below zero");
    _approve(_msgSender(), spender, currentAllowance - subtractedValue);

    return true;
  }

  /**
   * @dev Moves tokens `amount` from `sender` to `recipient`.
   *
   * This is internal function is equivalent to {transfer}, and can be used to
   * e.g. implement automatic token fees, slashing mechanisms, etc.
   *
   * Emits a {Transfer} event.
   *
   * Requirements:
   *
   * - `sender` cannot be the zero address.
   * - `recipient` cannot be the zero address.
   * - `sender` must have a balance of at least `amount`.
   */
  function _transfer(
    address sender,
    address recipient,
    uint256 amount
  ) internal virtual {
    require(sender != address(0), "ERC20: transfer from the zero address");
    require(recipient != address(0), "ERC20: transfer to the zero address");

    _beforeTokenTransfer(sender, recipient, amount);

    uint256 senderBalance = _balances[sender];
    require(senderBalance >= amount, "ERC20: transfer amount exceeds balance");
    _balances[sender] = senderBalance - amount;
    _balances[recipient] += amount;

    emit Transfer(sender, recipient, amount);
  }

  /** @dev Creates `amount` tokens and assigns them to `account`, increasing
   * the total supply.
   *
   * Emits a {Transfer} event with `from` set to the zero address.
   *
   * Requirements:
   *
   * - `to` cannot be the zero address.
   */
  function _mint(address account, uint256 amount) internal virtual {
    require(account != address(0), "ERC20: mint to the zero address");

    _beforeTokenTransfer(address(0), account, amount);

    _totalSupply += amount;
    _balances[account] += amount;
    emit Transfer(address(0), account, amount);
  }

  /**
   * @dev Destroys `amount` tokens from `account`, reducing the
   * total supply.
   *
   * Emits a {Transfer} event with `to` set to the zero address.
   *
   * Requirements:
   *
   * - `account` cannot be the zero address.
   * - `account` must have at least `amount` tokens.
   */
  function _burn(address account, uint256 amount) internal virtual {
    require(account != address(0), "ERC20: burn from the zero address");

    _beforeTokenTransfer(account, address(0), amount);

    uint256 accountBalance = _balances[account];
    require(accountBalance >= amount, "ERC20: burn amount exceeds balance");
    _balances[account] = accountBalance - amount;
    _totalSupply -= amount;

    emit Transfer(account, address(0), amount);
  }

  /**
   * @dev Sets `amount` as the allowance of `spender` over the `owner` s tokens.
   *
   * This internal function is equivalent to `approve`, and can be used to
   * e.g. set automatic allowances for certain subsystems, etc.
   *
   * Emits an {Approval} event.
   *
   * Requirements:
   *
   * - `owner` cannot be the zero address.
   * - `spender` cannot be the zero address.
   */
  function _approve(
    address owner,
    address spender,
    uint256 amount
  ) internal virtual {
    require(owner != address(0), "ERC20: approve from the zero address");
    require(spender != address(0), "ERC20: approve to the zero address");

    _allowances[owner][spender] = amount;
    emit Approval(owner, spender, amount);
  }

  /**
   * @dev Hook that is called before any transfer of tokens. This includes
   * minting and burning.
   *
   * Calling conditions:
   *
   * - when `from` and `to` are both non-zero, `amount` of ``from``'s tokens
   * will be to transferred to `to`.
   * - when `from` is zero, `amount` tokens will be minted for `to`.
   * - when `to` is zero, `amount` of ``from``'s tokens will be burned.
   * - `from` and `to` are never both zero.
   *
   * To learn more about hooks, head to xref:ROOT:extending-contracts.adoc#using-hooks[Using Hooks].
   */
  function _beforeTokenTransfer(
    address from,
    address to,
    uint256 amount
  ) internal virtual {}
}

contract SampleSyntheticToken is ERC20Initializable, Ownable, RedstoneConsumerNumericMock {
  bool private initialized;
  bytes32 public asset;
  address public broker;

  uint256 public constant MAX_SOLVENCY = 2**256 - 1;
  bytes32 public constant COLLATERAL_TOKEN = "ETH";
  uint256 public constant SOLVENCY_PRECISION = 1000; // 100%, 1 unit = 0.1%
  uint256 public constant MIN_SOLVENCY = 1200; // 120%, 1 unit = 0.1%
  uint256 public constant LIQUIDATION_BONUS = 50; // 5%, 1 unit = 0.1%

  mapping(address => uint256) public collateral;
  mapping(address => uint256) public debt;

  function initialize(
    bytes32 asset_,
    string memory name_,
    string memory symbol_
  ) external {
    require(!initialized);

    super.initialize(name_, symbol_);

    asset = asset_;

    initialized = true;
  }

  /**
   * @dev Mints koTokens increasing user's debt
   */
  function mint(uint256 amount) external payable remainsSolvent {
    super._mint(msg.sender, amount);
    debt[msg.sender] += amount;
    addCollateral();
  }

  /**
   * @dev Burns koTokens to reduce user debt
   */
  function burn(uint256 amount) external {
    require(debt[msg.sender] >= amount, "Cannot burn more than minted");
    debt[msg.sender] -= amount;
    super._burn(msg.sender, amount);
  }

  /**
   * @dev Adds collateral to user account
   * It could be done to increase the solvency ratio
   */
  function addCollateral() public payable virtual {
    collateral[msg.sender] += msg.value;
    emit CollateralAdded(msg.sender, msg.value, block.timestamp);
  }

  /**
   * @dev Removes outstanding collateral by paying out funds to depositor
   * The account must remain solvent after the operation
   */
  function removeCollateral(uint256 amount) external virtual remainsSolvent {
    require(collateral[msg.sender] >= amount, "Cannot remove more collateral than deposited");
    collateral[msg.sender] -= amount;
    payable(msg.sender).transfer(amount);
    emit CollateralRemoved(msg.sender, amount, block.timestamp);
  }

  /**
   * @dev Collateral amount expressed in ETH
   */
  function collateralOf(address account) public view virtual returns (uint256) {
    return collateral[account];
  }

  /**
   * @dev Collateral value expressed in USD
   */
  function collateralValueOf(address account) public view returns (uint256) {
    return collateralOf(account) * getOracleNumericValueFromTxMsg(COLLATERAL_TOKEN);
  }

  /**
   * @dev Debt of the account (number of koTokens minted)
   */
  function debtOf(address account) public view returns (uint256) {
    return debt[account];
  }

  /**
   * @dev Debt of the account expressed in USD
   */
  function debtValueOf(address account) public view returns (uint256) {
    return debt[account] * getOracleNumericValueFromTxMsg(asset);
  }

  /**
   * @dev A ratio between the value of collateral and debt of the account
   * It's expressed in permills - 0.1% (ratio 1 to 1 is equal to 1000 units)
   * To leave a margin for price movement and liquidation it must remain safely abot 1000
   */
  function solvencyOf(address account) public view returns (uint256) {
    if (debtValueOf(account) == 0) {
      return MAX_SOLVENCY;
    } else {
      return (collateralValueOf(account) * SOLVENCY_PRECISION) / debtValueOf(account);
    }
  }

  /**
   * @dev Value of komodo tokens held by given account at the current market price
   */
  function balanceValueOf(address account) public view returns (uint256) {
    return balanceOf(account) * getOracleNumericValueFromTxMsg(asset);
  }

  /**
   * @dev Total value of all minted komodo tokens at the current market price
   */
  function totalValue() public view returns (uint256) {
    return totalSupply() * getOracleNumericValueFromTxMsg(asset);
  }

  function liquidate(address account, uint256 amount) public {
    require(solvencyOf(account) < MIN_SOLVENCY, "Cannot liquidate a solvent account");
    this.transferFrom(msg.sender, account, amount);
    super._burn(account, amount);
    debt[account] -= amount;

    // Liquidator reward
    uint256 collateralRepayment = (amount * getOracleNumericValueFromTxMsg(asset)) /
      getOracleNumericValueFromTxMsg(COLLATERAL_TOKEN);
    uint256 bonus = (collateralRepayment * LIQUIDATION_BONUS) / SOLVENCY_PRECISION;

    uint256 repaymentWithBonus = collateralRepayment + bonus;
    collateral[account] -= repaymentWithBonus;
    payable(msg.sender).transfer(repaymentWithBonus);

    require(solvencyOf(account) >= MIN_SOLVENCY, "Account must be solvent after liquidation");
  }

  modifier remainsSolvent() {
    _;
    require(solvencyOf(msg.sender) >= MIN_SOLVENCY, "The account must remain solvent");
  }

  // EVENTS
  event CollateralAdded(address account, uint256 val, uint256 time);
  event CollateralRemoved(address account, uint256 val, uint256 time);
}

// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.20;

import "@openzeppelin/contracts-upgradeable/token/ERC20/extensions/ERC20PermitUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/extensions/ERC20BurnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/utils/SafeERC20Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/security/ReentrancyGuardUpgradeable.sol";
import "../../utils/SingleAdminAccessControlUpgradeable.sol";
import "./IiTryDefinitions.sol";

/**
 * @title iTry
 * @notice iTry rwa token contract
 */
contract iTry is
    ERC20BurnableUpgradeable,
    ERC20PermitUpgradeable,
    IiTryDefinitions,
    ReentrancyGuardUpgradeable,
    SingleAdminAccessControlUpgradeable
{
    using SafeERC20Upgradeable for IERC20Upgradeable;

    /// @notice The role is allowed to mint iTry. To be pointed to iTry minting contract only.
    bytes32 public constant MINTER_CONTRACT = keccak256("MINTER_CONTRACT");
    /// @notice Role that can handle Blacklisting, in addition to admin role.
    bytes32 public constant BLACKLIST_MANAGER_ROLE = keccak256("BLACKLIST_MANAGER_ROLE");
    /// @notice Role that can handle Whitelisting, in addition to admin role.
    bytes32 public constant WHITELIST_MANAGER_ROLE = keccak256("WHITELIST_MANAGER_ROLE");
    /// @notice Blacklisted role restricts funds from being moved in and out of that address
    bytes32 public constant BLACKLISTED_ROLE = keccak256("BLACKLISTED_ROLE");
    /// @notice During transferState 1, whitelisted role can still transfer
    bytes32 public constant WHITELISTED_ROLE = keccak256("WHITELISTED_ROLE");

    TransferState public transferState;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /* ------------- INITIALIZE ------------- */
    /**
     * @notice Initializer for iTry contract.
     * @param admin The address of the admin role.
     * @param minterContract The initial minterContract. Only this address can mint iTry
     */
    function initialize(address admin, address minterContract) public virtual initializer {
        __ERC20_init("iTry", "iTry");
        __ERC20Permit_init("iTry");
        __ReentrancyGuard_init();
        if (admin == address(0) || minterContract == address(0)) revert ZeroAddressException();
        transferState = TransferState.FULLY_ENABLED;
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(MINTER_CONTRACT, minterContract);
        emit MinterAdded(minterContract);
    }

    function addMinter(address minterContract) external onlyRole(DEFAULT_ADMIN_ROLE) {
        _grantRole(MINTER_CONTRACT, minterContract);
        emit MinterAdded(minterContract);
    }

    function removeMinter(address minterContract) external onlyRole(DEFAULT_ADMIN_ROLE) {
        _revokeRole(MINTER_CONTRACT, minterContract);
        emit MinterRemoved(minterContract);
    }

    /**
     * @param users List of address to be blacklisted
     * @notice It is deemed acceptable for admin or access manager roles to be blacklisted accidentally since it does not affect operations.
     */
    function addBlacklistAddress(address[] calldata users) external onlyRole(BLACKLIST_MANAGER_ROLE) {
        for (uint8 i = 0; i < users.length; i++) {
            if (hasRole(WHITELISTED_ROLE, users[i])) _revokeRole(WHITELISTED_ROLE, users[i]);
            _grantRole(BLACKLISTED_ROLE, users[i]);
        }
    }

    /**
     * @param users List of address to be removed from blacklist
     */
    function removeBlacklistAddress(address[] calldata users) external onlyRole(BLACKLIST_MANAGER_ROLE) {
        for (uint8 i = 0; i < users.length; i++) {
            _revokeRole(BLACKLISTED_ROLE, users[i]);
        }
    }

    /**
     * @param users List of address to be whitelist
     */
    function addWhitelistAddress(address[] calldata users) external onlyRole(WHITELIST_MANAGER_ROLE) {
        for (uint8 i = 0; i < users.length; i++) {
            if (!hasRole(BLACKLISTED_ROLE, users[i])) _grantRole(WHITELISTED_ROLE, users[i]);
        }
    }

    /**
     * @param users List of address to be removed from whitelist
     */
    function removeWhitelistAddress(address[] calldata users) external onlyRole(WHITELIST_MANAGER_ROLE) {
        for (uint8 i = 0; i < users.length; i++) {
            _revokeRole(WHITELISTED_ROLE, users[i]);
        }
    }

    /**
     * @dev Burns the blacklisted user iTry and mints to the desired owner address.
     * @param from The address to burn the entire balance, with the BLACKLISTED_ROLE
     * @param to The address to mint the entire balance of "from" parameter.
     */
    function redistributeLockedAmount(address from, address to) external nonReentrant onlyRole(DEFAULT_ADMIN_ROLE) {
        if (hasRole(BLACKLISTED_ROLE, from) && !hasRole(BLACKLISTED_ROLE, to)) {
            uint256 amountToDistribute = balanceOf(from);
            _burn(from, amountToDistribute);
            _mint(to, amountToDistribute);
            emit LockedAmountRedistributed(from, to, amountToDistribute);
        } else {
            revert OperationNotAllowed();
        }
    }

    /**
     * @notice Allows the owner to rescue tokens or ETH accidentally sent to the contract.
     * @param token The token to be rescued (use address(0) for ETH).
     * @param amount The amount of tokens/ETH to be rescued.
     * @param to Where to send rescued tokens/ETH
     */
    function rescueTokens(address token, uint256 amount, address to)
        external
        nonReentrant
        onlyRole(DEFAULT_ADMIN_ROLE)
    {
        if (to == address(0)) revert ZeroAddressException();
        if (amount == 0) revert ZeroAmount();

        if (token == address(0)) {
            // Rescue ETH
            (bool success,) = to.call{value: amount}("");
            if (!success) revert TransferFailed();
        } else {
            // Rescue ERC20 tokens
            IERC20Upgradeable(token).safeTransfer(to, amount);
        }

        emit TokenRescued(token, to, amount);
    }

    /**
     * @notice Mints new iTry tokens
     * @param to The address to mint tokens to
     * @param amount The amount of tokens to mint
     * @dev Only callable by MINTER_CONTRACT role
     */
    function mint(address to, uint256 amount) external onlyRole(MINTER_CONTRACT) {
        _mint(to, amount);
    }

    /**
     * @dev Remove renounce role access from AccessControl, to prevent users to resign roles.
     * @notice It's deemed preferable security-wise to ensure the contract maintains an owner,
     * over the ability to renounce roles, role renunciation can be achieved via owner revoking the role.
     */
    function renounceRole(bytes32, address) public virtual override {
        revert OperationNotAllowed();
    }

    /**
     * @param code Admin can disable all transfers, allow limited addresses only, or fully enable transfers
     */
    function updateTransferState(TransferState code) external onlyRole(DEFAULT_ADMIN_ROLE) {
        TransferState prevState = transferState;
        transferState = code;
        emit TransferStateUpdated(prevState, code);
    }

    function _beforeTokenTransfer(address from, address to, uint256) internal virtual override {
        // State 2 - Transfers fully enabled except for blacklisted addresses
        if (transferState == TransferState.FULLY_ENABLED) {
            if (hasRole(MINTER_CONTRACT, msg.sender) && !hasRole(BLACKLISTED_ROLE, from) && to == address(0)) {
                // redeeming
            } else if (hasRole(MINTER_CONTRACT, msg.sender) && from == address(0) && !hasRole(BLACKLISTED_ROLE, to)) {
                // minting
            } else if (hasRole(DEFAULT_ADMIN_ROLE, msg.sender) && hasRole(BLACKLISTED_ROLE, from) && to == address(0)) {
                // redistributing - burn
            } else if (hasRole(DEFAULT_ADMIN_ROLE, msg.sender) && from == address(0) && !hasRole(BLACKLISTED_ROLE, to))
            {
                // redistributing - mint
            } else if (
                !hasRole(BLACKLISTED_ROLE, msg.sender) && !hasRole(BLACKLISTED_ROLE, from)
                    && !hasRole(BLACKLISTED_ROLE, to)
            ) {
                // normal case
            } else {
                revert OperationNotAllowed();
            }
            // State 1 - Transfers only enabled between whitelisted addresses
        } else if (transferState == TransferState.WHITELIST_ENABLED) {
            if (hasRole(MINTER_CONTRACT, msg.sender) && !hasRole(BLACKLISTED_ROLE, from) && to == address(0)) {
                // redeeming
            } else if (hasRole(MINTER_CONTRACT, msg.sender) && from == address(0) && !hasRole(BLACKLISTED_ROLE, to)) {
                // minting
            } else if (hasRole(DEFAULT_ADMIN_ROLE, msg.sender) && hasRole(BLACKLISTED_ROLE, from) && to == address(0)) {
                // redistributing - burn
            } else if (hasRole(DEFAULT_ADMIN_ROLE, msg.sender) && from == address(0) && !hasRole(BLACKLISTED_ROLE, to))
            {
                // redistributing - mint
            } else if (hasRole(WHITELISTED_ROLE, msg.sender) && hasRole(WHITELISTED_ROLE, from) && to == address(0)) {
                // whitelisted user can burn
            } else if (
                hasRole(WHITELISTED_ROLE, msg.sender) && hasRole(WHITELISTED_ROLE, from)
                    && hasRole(WHITELISTED_ROLE, to)
            ) {
                // normal case
            } else {
                revert OperationNotAllowed();
            }
            // State 0 - Fully disabled transfers
        } else if (transferState == TransferState.FULLY_DISABLED) {
            revert OperationNotAllowed();
        }
    }
}

// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.20;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {StakediTryV2} from "./StakediTryCooldown.sol";
import {IStakediTryFastRedeem} from "./interfaces/IStakediTryFastRedeem.sol";

/**
 * @title StakediTryFastRedeem
 * @notice Extends StakediTryV2 with fast redemption functionality
 * @dev Allows users to bypass the cooldown period by paying a fee that goes to the treasury.
 *      This provides liquidity to users who need immediate access to their funds while
 *      maintaining the protocol's stability through fee collection.
 */
contract StakediTryFastRedeem is StakediTryV2, IStakediTryFastRedeem {
    using SafeERC20 for IERC20;

    /// @notice Basis points denominator for fee calculations (100% = 10000 basis points)
    uint256 private constant BASIS_POINTS = 10000;

    /// @notice Fast redemption configuration
    address public fastRedeemTreasury;
    uint16 public fastRedeemFeeInBPS;
    bool public fastRedeemEnabled;
    uint16 public constant MIN_FAST_REDEEM_FEE = 1; // 0.01% minimum fee (1 basis point)
    uint16 public constant MAX_FAST_REDEEM_FEE = 2000; // 20% maximum fee

    /// @notice ensure fast redeem is enabled
    modifier ensureFastRedeemEnabled() {
        if (!fastRedeemEnabled) revert FastRedeemDisabled();
        _;
    }

    /**
     * @notice Constructor for StakediTryFastRedeem contract
     * @param _asset The address of the iTry token
     * @param initialRewarder The address of the initial rewarder
     * @param owner The address of the admin role
     * @param _fastRedeemTreasury The address that will receive fast redemption fees
     */
    constructor(IERC20 _asset, address initialRewarder, address owner, address _fastRedeemTreasury)
        StakediTryV2(_asset, initialRewarder, owner)
    {
        if (_fastRedeemTreasury == address(0)) revert InvalidZeroAddress();

        fastRedeemTreasury = _fastRedeemTreasury;
        fastRedeemEnabled = false;
        fastRedeemFeeInBPS = MAX_FAST_REDEEM_FEE; // Start at maximum fee (20%)
    }

    /* ------------- EXTERNAL ------------- */

    /**
     * @inheritdoc IStakediTryFastRedeem
     */
    function fastRedeem(uint256 shares, address receiver, address owner)
        external
        ensureCooldownOn
        ensureFastRedeemEnabled
        returns (uint256 assets)
    {
        if (shares > maxRedeem(owner)) revert ExcessiveRedeemAmount();

        uint256 totalAssets = previewRedeem(shares);
        uint256 feeAssets = _redeemWithFee(shares, totalAssets, receiver, owner);

        emit FastRedeemed(owner, receiver, shares, totalAssets, feeAssets);

        return totalAssets - feeAssets;
    }

    /**
     * @inheritdoc IStakediTryFastRedeem
     */
    function fastWithdraw(uint256 assets, address receiver, address owner)
        external
        ensureCooldownOn
        ensureFastRedeemEnabled
        returns (uint256 shares)
    {
        if (assets > maxWithdraw(owner)) revert ExcessiveWithdrawAmount();

        uint256 totalShares = previewWithdraw(assets);
        uint256 feeAssets = _redeemWithFee(totalShares, assets, receiver, owner);

        emit FastRedeemed(owner, receiver, totalShares, assets, feeAssets);

        return totalShares;
    }

    /**
     * @inheritdoc IStakediTryFastRedeem
     */
    function setFastRedeemEnabled(bool enabled) external onlyRole(DEFAULT_ADMIN_ROLE) {
        fastRedeemEnabled = enabled;
        emit FastRedeemEnabledUpdated(enabled);
    }

    /**
     * @inheritdoc IStakediTryFastRedeem
     */
    function setFastRedeemFee(uint16 feeInBPS) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (feeInBPS < MIN_FAST_REDEEM_FEE || feeInBPS > MAX_FAST_REDEEM_FEE) {
            revert InvalidFastRedeemFee();
        }

        uint16 previousFee = fastRedeemFeeInBPS;
        fastRedeemFeeInBPS = feeInBPS;
        emit FastRedeemFeeUpdated(previousFee, feeInBPS);
    }

    /**
     * @inheritdoc IStakediTryFastRedeem
     */
    function setFastRedeemTreasury(address treasury) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (treasury == address(0)) revert InvalidZeroAddress();

        address previousTreasury = fastRedeemTreasury;
        fastRedeemTreasury = treasury;
        emit FastRedeemTreasuryUpdated(previousTreasury, treasury);
    }

    /* ------------- INTERNAL ------------- */

    /**
     * @notice Internal helper to perform redemption with fee split
     * @dev Handles the logic of splitting redemption into treasury fee and net user amount.
     *      Follows ERC4626 naming: "redeem" burns shares to withdraw assets.
     *      Reverts if the fee rounds down to zero.
     *      MIN_SHARES validation happens automatically in _withdraw() calls.
     * @param shares Total shares to burn from owner's balance
     * @param assets Total assets being withdrawn (gross amount before fee deduction)
     * @param receiver Address to receive the net assets (after fee is deducted)
     * @param owner Address that owns the shares being burned (must have approved caller if caller != owner)
     * @return feeAssets Amount of assets sent to treasury as fee
     */
    function _redeemWithFee(uint256 shares, uint256 assets, address receiver, address owner)
        internal
        returns (uint256 feeAssets)
    {
        feeAssets = (assets * fastRedeemFeeInBPS) / BASIS_POINTS;

        // Enforce that fast redemption always has a cost
        if (feeAssets == 0) revert InvalidAmount();

        uint256 feeShares = previewWithdraw(feeAssets);
        uint256 netShares = shares - feeShares;
        uint256 netAssets = assets - feeAssets;

        // Withdraw fee portion to treasury
        _withdraw(_msgSender(), fastRedeemTreasury, owner, feeAssets, feeShares);

        // Withdraw net portion to receiver
        _withdraw(_msgSender(), receiver, owner, netAssets, netShares);
    }
}

// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.20;

import {IStakediTryCooldown} from "./IStakediTryCooldown.sol";

/**
 * @title IStakediTryCrosschain
 * @notice Interface for StakediTryCrosschain variant that supports composer-managed cooldowns
 * @dev Extends StakediTryCooldown interface with role-gated helpers that allow a trusted
 *      composer (e.g. wiTryVaultComposer) to burn its own shares while crediting cooldown
 *      entitlements to a downstream redeemer that triggered a cross-chain flow.
 */
interface IStakediTryCrosschain is IStakediTryCooldown {
    /**
     * @notice Emitted when a composer starts a cooldown on behalf of a redeemer
     * @param composer Address that burned shares (trusted wiTryVaultComposer)
     * @param redeemer Address that will later call `unstake` on the hub chain
     * @param shares Amount of shares burned from the composer
     * @param assets Amount of assets locked in the silo for the redeemer
     * @param cooldownEnd Timestamp when the redeemer can claim
     */
    event ComposerCooldownInitiated(
        address indexed composer, address indexed redeemer, uint256 shares, uint256 assets, uint104 cooldownEnd
    );

    /**
     * @notice Emitted when unstake is performed through composer
     * @param composer Address that called (wiTryVaultComposer)
     * @param receiver Address that will receive iTRY
     * @param assets Amount of assets unstaked
     */
    event UnstakeThroughComposer(address indexed composer, address indexed receiver, uint256 assets);

    /**
     * @notice Emitted when fast redeem is performed through composer
     * @param composer Address that called (wiTryVaultComposer)
     * @param crosschainReceiver Address that will receive iTRY on remote chain
     * @param owner Address whose shares are being redeemed
     * @param shares Amount of shares redeemed
     * @param assets Amount of assets received (after fees)
     * @param feeAssets Amount of assets paid as fees
     */
    event FastRedeemedThroughComposer(
        address indexed composer,
        address indexed crosschainReceiver,
        address indexed owner,
        uint256 shares,
        uint256 assets,
        uint256 feeAssets
    );

    /**
     * @notice Error thrown when owner parameter doesn't match composer
     */
    error InvalidOwner();

    /**
     * @notice Initiate a cooldown by specifying the number of shares to burn
     * @dev Burns `shares` from the composer and records the resulting assets for the redeemer
     * @param shares Amount of shares to burn from the composer balance
     * @param redeemer Address that will be able to claim the cooled-down assets
     * @return assets Amount of assets that were moved into cooldown
     */
    function cooldownSharesByComposer(uint256 shares, address redeemer) external returns (uint256 assets);

    /**
     * @notice Initiate a cooldown by specifying the amount of assets to lock
     * @dev Converts assets to the corresponding share amount and burns them from the composer
     * @param assets Amount of assets to place into cooldown
     * @param redeemer Address that will be able to claim the cooled-down assets
     * @return shares Amount of shares that were burned
     */
    function cooldownAssetsByComposer(uint256 assets, address redeemer) external returns (uint256 shares);

    /**
     * @notice Returns the composer role identifier
     * @return bytes32 The keccak256 hash of "COMPOSER_ROLE"
     */
    function COMPOSER_ROLE() external view returns (bytes32);

    /**
     * @notice Unstake through composer after cooldown period
     * @dev Can only be called by composer role
     * @dev Validates cooldown completion before unstaking
     * @dev Calls silo.withdraw() to transfer assets back to composer
     *
     * @param receiver Address that initiated the unstake request
     * @return assets Amount of assets withdrawn
     */
    function unstakeThroughComposer(address receiver) external returns (uint256 assets);

    /**
     * @notice Fast redeem through composer by specifying shares
     * @dev Burns shares from composer and immediately redeems assets (bypassing cooldown with fee)
     * @param shares Amount of shares to redeem from composer balance
     * @param crosschainReceiver Address that will receive iTRY on remote chain
     * @param owner Address whose shares are being redeemed
     * @return assets Amount of assets received (after fees)
     */
    function fastRedeemThroughComposer(uint256 shares, address crosschainReceiver, address owner)
        external
        returns (uint256 assets);

    /**
     * @notice Fast redeem through composer by specifying assets
     * @dev Converts assets to shares, burns from composer, immediately redeems (bypassing cooldown with fee)
     * @param assets Amount of assets to redeem
     * @param crosschainReceiver Address that will receive iTRY on remote chain
     * @param owner Address whose shares are being redeemed
     * @return shares Amount of shares burned
     */
    function fastWithdrawThroughComposer(uint256 assets, address crosschainReceiver, address owner)
        external
        returns (uint256 shares);
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {IYieldProcessor} from "./periphery/IYieldProcessor.sol";
import {CommonErrors} from "./periphery/CommonErrors.sol";

/**
 * @title YieldForwarder
 * @author Inverter Network
 * @notice A simple yield processor that forwards received yield tokens to a designated recipient address
 * @dev This contract implements the IYieldProcessor interface and acts as a passthrough mechanism
 *      for yield distribution. When yield is processed, it transfers the entire amount to a
 *      pre-configured recipient address.
 *
 *      Key features:
 *      - Owner-controlled recipient address management
 *      - Automatic forwarding of yield tokens upon processing
 *      - Compatible with any ERC20 token
 *      - Event emission for tracking yield forwarding and recipient updates
 *
 * @custom:security-contact security@inverter.network
 */
contract YieldForwarder is IYieldProcessor, Ownable, ReentrancyGuard {
    using SafeERC20 for IERC20;

    // ============================================
    // State Variables
    // ============================================

    /// @notice The ERC20 token that will be forwarded as yield
    IERC20 public immutable yieldToken;

    /// @notice The address that will receive forwarded yield
    address public yieldRecipient;

    // ============================================
    // Events
    // ============================================

    /// @notice Emitted when yield is forwarded to the recipient
    /// @param recipient The address that received the yield
    /// @param amount The amount of yield tokens forwarded
    event YieldForwarded(address indexed recipient, uint256 amount);

    /// @notice Emitted when the yield recipient address is updated
    /// @param oldRecipient The previous recipient address
    /// @param newRecipient The new recipient address
    event YieldRecipientUpdated(address indexed oldRecipient, address indexed newRecipient);

    /// @notice Emitted when tokens are rescued by the owner
    /// @param token The address of the token that was rescued
    /// @param to The address that received the rescued tokens
    /// @param amount The amount of tokens rescued
    event TokensRescued(address indexed token, address indexed to, uint256 amount);

    // ============================================
    // Constructor
    // ============================================

    /**
     * @notice Initializes the YieldForwarder contract
     * @param _yieldToken Address of the ERC20 token to be forwarded as yield
     * @param _initialRecipient Initial address to receive forwarded yield
     */
    constructor(address _yieldToken, address _initialRecipient) {
        if (_yieldToken == address(0)) revert CommonErrors.ZeroAddress();
        if (_initialRecipient == address(0)) revert CommonErrors.ZeroAddress();

        yieldToken = IERC20(_yieldToken);
        yieldRecipient = _initialRecipient;

        emit YieldRecipientUpdated(address(0), _initialRecipient);
    }

    // ============================================
    // External Functions - IYieldProcessor Implementation
    // ============================================

    /**
     * @notice Processes new yield by forwarding it to the designated recipient
     * @dev Implements the IYieldProcessor interface. Transfers the entire yield amount
     *      from this contract to the yieldRecipient address.
     * @param _newYieldAmount The amount of yield tokens to process and forward
     *
     * Requirements:
     * - `_newYieldAmount` must be greater than zero
     * - `yieldRecipient` must not be the zero address
     * - This contract must have sufficient balance of yieldToken
     * - The transfer must succeed
     *
     * Emits a {YieldForwarded} event upon successful transfer
     */
    function processNewYield(uint256 _newYieldAmount) external override {
        if (_newYieldAmount == 0) revert CommonErrors.ZeroAmount();
        if (yieldRecipient == address(0)) revert RecipientNotSet();

        // Transfer yield tokens to the recipient
        if (!yieldToken.transfer(yieldRecipient, _newYieldAmount)) {
            revert CommonErrors.TransferFailed();
        }

        emit YieldForwarded(yieldRecipient, _newYieldAmount);
    }

    // ============================================
    // External Functions - Configuration
    // ============================================

    /**
     * @notice Updates the address that will receive forwarded yield
     * @dev Only callable by the contract owner
     * @param _newRecipient The new address to receive yield
     *
     * Requirements:
     * - Caller must be the contract owner
     * - `_newRecipient` must not be the zero address
     *
     * Emits a {YieldRecipientUpdated} event
     */
    function setYieldRecipient(address _newRecipient) external onlyOwner {
        if (_newRecipient == address(0)) revert CommonErrors.ZeroAddress();

        address oldRecipient = yieldRecipient;
        yieldRecipient = _newRecipient;

        emit YieldRecipientUpdated(oldRecipient, _newRecipient);
    }

    // ============================================
    // View Functions
    // ============================================

    /**
     * @notice Returns the current yield recipient address
     * @return The address that will receive forwarded yield
     */
    function getYieldRecipient() external view returns (address) {
        return yieldRecipient;
    }

    // ============================================
    // Emergency Functions
    // ============================================
    /*
     * @notice Rescue tokens accidentally sent to this contract
     * @dev Only callable by owner. Can rescue both ERC20 tokens and native ETH
     *      Use address(0) for rescuing ETH
     * @param token The token address to rescue (use address(0) for ETH)
     * @param to The address to send rescued tokens to
     * @param amount The amount to rescue
     */
    function rescueToken(address token, address to, uint256 amount) external onlyOwner nonReentrant {
        if (to == address(0)) revert CommonErrors.ZeroAddress();
        if (amount == 0) revert CommonErrors.ZeroAmount();

        if (token == address(0)) {
            // Rescue ETH
            (bool success,) = to.call{value: amount}("");
            if (!success) revert CommonErrors.TransferFailed();
        } else {
            // Rescue ERC20 tokens
            IERC20(token).safeTransfer(to, amount);
        }

        emit TokensRescued(token, to, amount);
    }
}

// SPDX-License-Identifier: GPL-3.0

pragma solidity 0.8.20;

import {IStakediTryCooldown} from "./IStakediTryCooldown.sol";

/**
 * @title IStakediTryFastRedeem
 * @notice Interface for fast redemption functionality
 * @dev Extends IStakediTryCooldown with fast redemption capabilities that allow users to bypass
 *      the cooldown period by paying a fee
 */
interface IStakediTryFastRedeem is IStakediTryCooldown {
    // Events //
    /// @notice Event emitted when fast redeem is enabled/disabled
    event FastRedeemEnabledUpdated(bool enabled);
    /// @notice Event emitted when fast redeem fee is updated
    event FastRedeemFeeUpdated(uint16 previousFee, uint16 newFee);
    /// @notice Event emitted when fast redeem treasury is updated
    event FastRedeemTreasuryUpdated(address previousTreasury, address newTreasury);
    /// @notice Event emitted when a fast redemption occurs
    event FastRedeemed(
        address indexed owner, address indexed receiver, uint256 shares, uint256 assets, uint256 feeAssets
    );

    // Errors //
    /// @notice Error emitted when fast redeem is disabled
    error FastRedeemDisabled();
    /// @notice Error emitted when fast redeem fee exceeds maximum or is below minimum
    error InvalidFastRedeemFee();

    /**
     * @notice Fast redeem shares for immediate withdrawal with a fee
     * @param shares Amount of shares to redeem
     * @param receiver Address to receive the net assets
     * @param owner Address that owns the shares being redeemed
     * @return assets Net assets received by the receiver (after fee)
     */
    function fastRedeem(uint256 shares, address receiver, address owner) external returns (uint256 assets);

    /**
     * @notice Fast withdraw assets for immediate withdrawal with a fee
     * @param assets Amount of assets to withdraw (gross, before fee)
     * @param receiver Address to receive the net assets
     * @param owner Address that owns the shares being burned
     * @return shares Total shares burned
     */
    function fastWithdraw(uint256 assets, address receiver, address owner) external returns (uint256 shares);

    /**
     * @notice Enable or disable fast redemption feature
     * @param enabled True to enable, false to disable
     */
    function setFastRedeemEnabled(bool enabled) external;

    /**
     * @notice Set the fast redemption fee in basis points
     * @param feeInBPS Fee in basis points (e.g., 500 = 5%)
     */
    function setFastRedeemFee(uint16 feeInBPS) external;

    /**
     * @notice Set the treasury address that receives fast redemption fees
     * @param treasury Address of the treasury
     */
    function setFastRedeemTreasury(address treasury) external;

    /**
     * @notice Get the current fast redemption treasury address
     * @return address Treasury address
     */
    function fastRedeemTreasury() external view returns (address);

    /**
     * @notice Get the current fast redemption fee in basis points
     * @return uint16 Fee in basis points
     */
    function fastRedeemFeeInBPS() external view returns (uint16);

    /**
     * @notice Check if fast redemption is currently enabled
     * @return bool True if enabled
     */
    function fastRedeemEnabled() external view returns (bool);
}

// SPDX-License-Identifier: BUSL-1.1

pragma solidity 0.8.17;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

/**
 * @title RedstoneToken
 * @dev Standard implementation of ERC20 for Redstone token
 */
contract RedstoneToken is ERC20 {
  uint256 public constant MAX_SUPPLY = 1_000_000_000e18;

  error CanNotMintMoreThanMaxSupply();
  error OnlyMinterCanMint();
  error OnlyMinterCanProposeNewMinter();
  error OnlyProposedMinterCanAcceptMinterRole();

  event MinterProposal(address indexed proposedMinter);
  event MinterUpdate(address indexed newMinter);

  address public minter;
  address public proposedMinter;

  constructor(uint256 initialSupply) ERC20("Redstone", "RED") {
    enforceMaxSupplyLimit(initialSupply);
    _mint(msg.sender, initialSupply);
    minter = msg.sender;
  }

  function mint(address account, uint256 amount) external {
    if (msg.sender != minter) {
      revert OnlyMinterCanMint();
    }
    enforceMaxSupplyLimit(totalSupply() + amount);
    _mint(account, amount);
  }

  function proposeNewMinter(address newProposedMinter) external {
    if (msg.sender != minter) {
      revert OnlyMinterCanProposeNewMinter();
    }
    proposedMinter = newProposedMinter;
    emit MinterProposal(newProposedMinter);
  }

  function acceptMinterRole() external {
    if (msg.sender != proposedMinter) {
      revert OnlyProposedMinterCanAcceptMinterRole();
    }
    minter = proposedMinter;
    proposedMinter = address(0);
    emit MinterUpdate(minter);
  }

  function enforceMaxSupplyLimit(uint256 totalSupply) internal pure {
    if (totalSupply > MAX_SUPPLY) {
      revert CanNotMintMoreThanMaxSupply();
    }
  }
}

// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import {OFT} from "@layerzerolabs/lz-evm-oapp-v2/contracts/oft/OFT.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "./../IiTryDefinitions.sol";

/**
 * @title iTryTokenOFT
 * @notice OFT representation of iTRY on spoke chains (MegaETH)
 * @dev This contract mints/burns tokens based on LayerZero messages from the hub chain
 *
 * Architecture:
 * - Hub Chain (Ethereum): iTryToken (native) + iTryTokenAdapter (locks tokens)
 * - Spoke Chain (MegaETH): iTryTokenOFT (mints/burns based on messages)
 *
 * Flow from Hub to Spoke:
 * 1. Hub adapter locks native iTRY
 * 2. LayerZero message sent to this contract
 * 3. This contract mints equivalent OFT tokens
 *
 * Flow from Spoke to Hub:
 * 1. This contract burns OFT tokens
 * 2. LayerZero message sent to hub adapter
 * 3. Hub adapter unlocks native iTRY tokens
 */
contract iTryTokenOFT is OFT, IiTryDefinitions, ReentrancyGuard {
    using SafeERC20 for IERC20;

    /// @notice Address allowed to mint iTry (typically the LayerZero endpoint)
    address public minter;

    /// @notice Mapping of blacklisted addresses
    mapping(address => bool) public blacklisted;

    /// @notice Mapping of whitelisted addresses
    mapping(address => bool) public whitelisted;

    TransferState public transferState;

    /// @notice Emitted when minter address is updated
    event MinterUpdated(address indexed oldMinter, address indexed newMinter);

    /**
     * @notice Constructor for iTryTokenOFT
     * @param _lzEndpoint LayerZero endpoint address for MegaETH
     * @param _owner Address that will own this OFT (typically deployer)
     */
    constructor(address _lzEndpoint, address _owner) OFT("iTry Token", "iTRY", _lzEndpoint, _owner) {
        transferState = TransferState.FULLY_ENABLED;
        minter = _lzEndpoint;
    }

    /**
     * @notice Sets the minter address
     * @param _newMinter The new minter address
     */
    function setMinter(address _newMinter) external onlyOwner {
        address oldMinter = minter;
        minter = _newMinter;
        emit MinterUpdated(oldMinter, _newMinter);
    }

    /**
     * @param users List of address to be blacklisted
     * @notice Owner can blacklist addresses. Blacklisted addresses cannot transfer tokens.
     */
    function addBlacklistAddress(address[] calldata users) external onlyOwner {
        for (uint8 i = 0; i < users.length; i++) {
            if (whitelisted[users[i]]) whitelisted[users[i]] = false;
            blacklisted[users[i]] = true;
        }
    }

    /**
     * @param users List of address to be removed from blacklist
     */
    function removeBlacklistAddress(address[] calldata users) external onlyOwner {
        for (uint8 i = 0; i < users.length; i++) {
            blacklisted[users[i]] = false;
        }
    }

    /**
     * @param users List of address to be whitelisted
     */
    function addWhitelistAddress(address[] calldata users) external onlyOwner {
        for (uint8 i = 0; i < users.length; i++) {
            if (!blacklisted[users[i]]) whitelisted[users[i]] = true;
        }
    }

    /**
     * @param users List of address to be removed from whitelist
     */
    function removeWhitelistAddress(address[] calldata users) external onlyOwner {
        for (uint8 i = 0; i < users.length; i++) {
            whitelisted[users[i]] = false;
        }
    }

    /**
     * @dev Burns the blacklisted user iTry and mints to the desired owner address.
     * @param from The address to burn the entire balance, must be blacklisted
     * @param to The address to mint the entire balance of "from" parameter.
     */
    function redistributeLockedAmount(address from, address to) external nonReentrant onlyOwner {
        if (blacklisted[from] && !blacklisted[to]) {
            uint256 amountToDistribute = balanceOf(from);
            _burn(from, amountToDistribute);
            _mint(to, amountToDistribute);
            emit LockedAmountRedistributed(from, to, amountToDistribute);
        } else {
            revert OperationNotAllowed();
        }
    }

    /**
     * @notice Allows the owner to rescue tokens accidentally sent to the contract.
     * @param token The token to be rescued.
     * @param amount The amount of tokens to be rescued.
     * @param to Where to send rescued tokens
     */
    function rescueTokens(address token, uint256 amount, address to) external nonReentrant onlyOwner {
        IERC20(token).safeTransfer(to, amount);
        emit TokenRescued(token, to, amount);
    }

    /**
     * @param code Owner can disable all transfers, allow limited addresses only, or fully enable transfers
     */
    function updateTransferState(TransferState code) external onlyOwner {
        TransferState prevState = transferState;
        transferState = code;
        emit TransferStateUpdated(prevState, code);
    }

    function _beforeTokenTransfer(address from, address to, uint256) internal virtual override {
        // State 2 - Transfers fully enabled except for blacklisted addresses
        if (transferState == TransferState.FULLY_ENABLED) {
            if (msg.sender == minter && !blacklisted[from] && to == address(0)) {
                // redeeming
            } else if (msg.sender == minter && from == address(0) && !blacklisted[to]) {
                // minting
            } else if (msg.sender == owner() && blacklisted[from] && to == address(0)) {
                // redistributing - burn
            } else if (msg.sender == owner() && from == address(0) && !blacklisted[to]) {
                // redistributing - mint
            } else if (!blacklisted[msg.sender] && !blacklisted[from] && !blacklisted[to]) {
                // normal case
            } else {
                revert OperationNotAllowed();
            }
            // State 1 - Transfers only enabled between whitelisted addresses
        } else if (transferState == TransferState.WHITELIST_ENABLED) {
            if (msg.sender == minter && !blacklisted[from] && to == address(0)) {
                // redeeming
            } else if (msg.sender == minter && from == address(0) && !blacklisted[to]) {
                // minting
            } else if (msg.sender == owner() && blacklisted[from] && to == address(0)) {
                // redistributing - burn
            } else if (msg.sender == owner() && from == address(0) && !blacklisted[to]) {
                // redistributing - mint
            } else if (whitelisted[msg.sender] && whitelisted[from] && to == address(0)) {
                // whitelisted user can burn
            } else if (whitelisted[msg.sender] && whitelisted[from] && whitelisted[to]) {
                // normal case
            } else {
                revert OperationNotAllowed();
            }
            // State 0 - Fully disabled transfers
        } else if (transferState == TransferState.FULLY_DISABLED) {
            revert OperationNotAllowed();
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/IERC20Permit.sol";

/**
 * @title IiTryToken
 * @notice Interface for the iTRY token contract
 * @dev This interface defines the functions that the iTryIssuer contract needs to interact with the iTRY token
 */
interface IiTryToken is IERC20, IERC20Permit, IERC20Metadata {
    /**
     * @notice Mint new iTRY tokens
     * @param to The address to receive the minted tokens
     * @param amount The amount of tokens to mint
     */
    function mint(address to, uint256 amount) external;

    /**
     * @notice Burn iTRY tokens from a specific address
     * @param from The address whose tokens will be burned
     * @param amount The amount of tokens to burn
     */
    function burnFrom(address from, uint256 amount) external;
}

// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.20;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {StakediTryFastRedeem} from "./StakediTryFastRedeem.sol";
import {UserCooldown} from "./interfaces/IStakediTryCooldown.sol";
import {IStakediTryCrosschain} from "./interfaces/IStakediTryCrosschain.sol";

/**
 * @title StakediTryCrosschain
 * @notice Extends StakediTryFastRedeem with role-gated helpers for trusted composers
 * @dev A composer (e.g. wiTryVaultComposer) can burn its own shares after bridging them in and
 *      assign the resulting cooldown entitlement to an end-user redeemer. This contract
 *      keeps the cooldown accounting in the redeemer slot while still relying on the base
 *      `_withdraw` routine to maintain iTRY system integrity.
 */
contract StakediTryCrosschain is StakediTryFastRedeem, IStakediTryCrosschain {
    /// @notice Role identifier for trusted composers
    bytes32 public constant COMPOSER_ROLE = keccak256("COMPOSER_ROLE");

    /**
     * @notice Initializes the composer-aware staking vault
     * @param _asset iTRY token address
     * @param initialRewarder Initial rewarder contract
     * @param owner Address that receives the default admin role
     * @param _fastRedeemTreasury Treasury address for fast redeem fees
     */
    constructor(IERC20 _asset, address initialRewarder, address owner, address _fastRedeemTreasury)
        StakediTryFastRedeem(_asset, initialRewarder, owner, _fastRedeemTreasury)
    {}

    /**
     * @inheritdoc IStakediTryCrosschain
     * @return assets Amount of underlying assets locked in cooldown for the redeemer
     */
    function cooldownSharesByComposer(uint256 shares, address redeemer)
        external
        onlyRole(COMPOSER_ROLE)
        ensureCooldownOn
        returns (uint256 assets)
    {
        address composer = msg.sender;
        if (redeemer == address(0)) revert InvalidZeroAddress();
        if (shares > maxRedeem(composer)) revert ExcessiveRedeemAmount();

        assets = previewRedeem(shares);
        _startComposerCooldown(composer, redeemer, shares, assets);
    }

    /**
     * @inheritdoc IStakediTryCrosschain
     * @return shares Amount of shares burned from the composer's balance
     */
    function cooldownAssetsByComposer(uint256 assets, address redeemer)
        external
        onlyRole(COMPOSER_ROLE)
        ensureCooldownOn
        returns (uint256 shares)
    {
        address composer = msg.sender;
        if (redeemer == address(0)) revert InvalidZeroAddress();
        if (assets > maxWithdraw(composer)) revert ExcessiveWithdrawAmount();

        shares = previewWithdraw(assets);
        _startComposerCooldown(composer, redeemer, shares, assets);
    }

    /**
     * @inheritdoc IStakediTryCrosschain
     * @notice Unstake through composer after cooldown period
     * @dev Can only be called by composer role
     * @dev Validates cooldown completion before unstaking
     * @dev Calls silo.withdraw() to transfer assets back to composer
     * @param receiver Address that initiated the unstake request
     * @return assets Amount of assets withdrawn
     */
    function unstakeThroughComposer(address receiver)
        external
        onlyRole(COMPOSER_ROLE)
        nonReentrant
        returns (uint256 assets)
    {
        // Validate valid receiver
        if (receiver == address(0)) revert InvalidZeroAddress();

        UserCooldown storage userCooldown = cooldowns[receiver];
        assets = userCooldown.underlyingAmount;

        if (block.timestamp >= userCooldown.cooldownEnd) {
            userCooldown.cooldownEnd = 0;
            userCooldown.underlyingAmount = 0;

            silo.withdraw(msg.sender, assets); // transfer to wiTryVaultComposer for crosschain transfer
        } else {
            revert InvalidCooldown();
        }

        emit UnstakeThroughComposer(msg.sender, receiver, assets);

        return assets;
    }

    /**
     * @inheritdoc IStakediTryCrosschain
     * @notice Fast redeem through composer by specifying shares
     * @dev Burns shares from composer and immediately redeems assets bypassing cooldown with fee
     * @param shares Amount of shares to redeem from composer balance
     * @param crosschainReceiver Address that will receive iTRY on remote chain
     * @param owner Address whose shares are being redeemed (must be composer)
     * @return assets Amount of assets received (after fees)
     */
    function fastRedeemThroughComposer(uint256 shares, address crosschainReceiver, address owner)
        external
        onlyRole(COMPOSER_ROLE)
        ensureCooldownOn
        ensureFastRedeemEnabled
        returns (uint256 assets)
    {
        address composer = msg.sender;
        if (crosschainReceiver == address(0)) revert InvalidZeroAddress();
        if (shares > maxRedeem(composer)) revert ExcessiveRedeemAmount(); // Composer holds the shares on behave of the owner

        uint256 totalAssets = previewRedeem(shares);
        uint256 feeAssets = _redeemWithFee(shares, totalAssets, composer, composer); // Composer receives the assets for further crosschain transfer

        assets = totalAssets - feeAssets;

        emit FastRedeemedThroughComposer(composer, crosschainReceiver, owner, shares, assets, feeAssets);

        return assets;
    }

    /**
     * @inheritdoc IStakediTryCrosschain
     * @notice Fast redeem through composer by specifying assets
     * @dev Converts assets to shares, burns from composer, immediately redeems bypassing cooldown with fee
     * @param assets Amount of assets to redeem
     * @param crosschainReceiver Address that will receive iTRY on remote chain
     * @param owner Address whose shares are being redeemed (must be composer)
     * @return shares Amount of shares burned
     */
    function fastWithdrawThroughComposer(uint256 assets, address crosschainReceiver, address owner)
        external
        onlyRole(COMPOSER_ROLE)
        ensureCooldownOn
        ensureFastRedeemEnabled
        returns (uint256 shares)
    {
        address composer = msg.sender;
        if (crosschainReceiver == address(0)) revert InvalidZeroAddress();
        if (assets > maxWithdraw(composer)) revert ExcessiveWithdrawAmount(); // Composer holds the assets on behave of the owner

        shares = previewWithdraw(assets);
        uint256 feeAssets = _redeemWithFee(shares, assets, composer, composer); // Composer receives the assets for further crosschain transfer

        emit FastRedeemedThroughComposer(composer, crosschainReceiver, owner, shares, assets - feeAssets, feeAssets);

        return shares;
    }

    /**
     * @dev Internal function to initiate cooldown for a redeemer using composer's shares
     * @param composer Address that owns the shares being burned
     * @param redeemer Address that will be able to claim the cooled-down assets
     * @param shares Amount of shares to burn
     * @param assets Amount of assets to place in cooldown
     * @notice Follows Checks-Effects-Interactions pattern: external call to _withdraw occurs first,
     *         then state changes. _withdraw has nonReentrant modifier from base StakediTryV2 for safety.
     */
    function _startComposerCooldown(address composer, address redeemer, uint256 shares, uint256 assets) private {
        uint104 cooldownEnd = uint104(block.timestamp) + cooldownDuration;

        // Interaction: External call to base contract (protected by nonReentrant modifier)
        _withdraw(composer, address(silo), composer, assets, shares);

        // Effects: State changes after external call (following CEI pattern)
        cooldowns[redeemer].cooldownEnd = cooldownEnd;
        cooldowns[redeemer].underlyingAmount += uint152(assets);

        emit ComposerCooldownInitiated(composer, redeemer, shares, assets, cooldownEnd);
    }
}

// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.20;

import {OFT} from "@layerzerolabs/lz-evm-oapp-v2/contracts/oft/OFT.sol";

/**
 * @title wiTryOFT
 * @notice OFT representation of wiTRY shares on spoke chains (MegaETH)
 * @dev This contract mints/burns share tokens based on LayerZero messages from the hub chain
 *
 * Architecture (Phase 1 - Instant Redeems):
 * - Hub Chain (Ethereum): StakediTry (vault) + wiTryOFTAdapter (locks shares)
 * - Spoke Chain (MegaETH): wiTryOFT (mints/burns based on messages)
 *
 * Flow from Hub to Spoke:
 * 1. Hub adapter locks native wiTRY shares
 * 2. LayerZero message sent to this contract
 * 3. This contract mints equivalent OFT share tokens
 *
 * Flow from Spoke to Hub:
 * 1. This contract burns OFT share tokens
 * 2. LayerZero message sent to hub adapter
 * 3. Hub adapter unlocks native wiTRY shares
 *
 * NOTE: These shares represent staked iTRY in the vault. The share value
 * increases as yield is distributed to the vault on the hub chain.
 */
contract wiTryOFT is OFT {
    // Address of the entity authorized to manage the blacklist
    address public blackLister;

    // Mapping to track blacklisted users
    mapping(address => bool) public blackList;

    // Events emitted on changes to the blacklist or fund redistribution
    event BlackListerSet(address indexed blackLister);
    event BlackListUpdated(address indexed user, bool isBlackListed);
    event RedistributeFunds(address indexed user, uint256 amount);

    // Errors to be thrown in case of restricted actions
    error BlackListed(address user);
    error NotBlackListed();
    error OnlyBlackLister();

    /**
     * @dev Constructor to initialize the wiTryOFT contract.
     * @param _name The name of the token.
     * @param _symbol The symbol of the token.
     * @param _lzEndpoint Address of the LZ endpoint.
     * @param _delegate Address of the delegate.
     */
    constructor(string memory _name, string memory _symbol, address _lzEndpoint, address _delegate)
        OFT(_name, _symbol, _lzEndpoint, _delegate)
    {}

    /**
     * @dev Sets the address authorized to manage the blacklist. Only callable by the owner.
     * @param _blackLister Address of the entity authorized to manage the blacklist.
     */
    function setBlackLister(address _blackLister) external onlyOwner {
        blackLister = _blackLister;
        emit BlackListerSet(_blackLister);
    }

    /**
     * @dev Updates the blacklist status of a user.
     * @param _user The user identifier to update.
     * @param _isBlackListed Boolean indicating whether the user should be blacklisted or not.
     */
    function updateBlackList(address _user, bool _isBlackListed) external {
        if (msg.sender != blackLister && msg.sender != owner()) revert OnlyBlackLister();
        blackList[_user] = _isBlackListed;
        emit BlackListUpdated(_user, _isBlackListed);
    }

    /**
     * @dev Credits tokens to the recipient while checking if the recipient is blacklisted.
     * If blacklisted, redistributes the funds to the contract owner.
     * @param _to The address of the recipient.
     * @param _amountLD The amount of tokens to credit.
     * @param _srcEid The source endpoint identifier.
     * @return amountReceivedLD The actual amount of tokens received.
     */
    function _credit(address _to, uint256 _amountLD, uint32 _srcEid)
        internal
        virtual
        override
        returns (uint256 amountReceivedLD)
    {
        // If the recipient is blacklisted, emit an event, redistribute funds, and credit the owner
        if (blackList[_to]) {
            emit RedistributeFunds(_to, _amountLD);
            return super._credit(owner(), _amountLD, _srcEid);
        } else {
            return super._credit(_to, _amountLD, _srcEid);
        }
    }

    /**
     * @dev Checks the blacklist for both sender and recipient before updating balances for a local movement.
     * @param _from The address from which tokens are transferred.
     * @param _to The address to which tokens are transferred.
     * @param _amount The amount of tokens to transfer.
     */
    function _beforeTokenTransfer(address _from, address _to, uint256 _amount) internal override {
        if (blackList[_from]) revert BlackListed(_from);
        if (blackList[_to]) revert BlackListed(_to);
        if (blackList[msg.sender]) revert BlackListed(msg.sender);
        super._beforeTokenTransfer(_from, _to, _amount);
    }

    /**
     * @dev Redistributes funds from a blacklisted address to the contract owner. Only callable by the owner.
     * @param _from The address from which funds will be redistributed.
     * @param _amount The amount of funds to redistribute.
     */
    function redistributeBlackListedFunds(address _from, uint256 _amount) external onlyOwner {
        // @dev Only allow redistribution if the address is blacklisted
        if (!blackList[_from]) revert NotBlackListed();

        // @dev Temporarily remove from the blacklist, transfer funds, and restore to the blacklist
        blackList[_from] = false;
        _transfer(_from, owner(), _amount);
        blackList[_from] = true;

        emit RedistributeFunds(_from, _amount);
    }
}


## SUPPORTING CONTEXT: EXTERNAL LIBRARIES

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

