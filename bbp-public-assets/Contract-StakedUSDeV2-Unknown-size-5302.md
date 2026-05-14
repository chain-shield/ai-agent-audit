
 ------------ ## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.20;

/* solhint-disable var-name-mixedcase */

import "./StakedUSDe.sol";
import "./interfaces/IStakedUSDeCooldown.sol";
import "./USDeSilo.sol";

/**
 * @title StakedUSDeV2
 * @notice The StakedUSDeV2 contract allows users to stake USDe tokens and earn a portion of protocol LST and perpetual yield that is allocated
 * to stakers by the Ethena DAO governance voted yield distribution algorithm.  The algorithm seeks to balance the stability of the protocol by funding
 * the protocol's insurance fund, DAO activities, and rewarding stakers with a portion of the protocol's yield.
 * @dev If cooldown duration is set to zero, the StakedUSDeV2 behavior changes to follow ERC4626 standard and disables cooldownShares and cooldownAssets methods. If cooldown duration is greater than zero, the ERC4626 withdrawal and redeem functions are disabled, breaking the ERC4626 standard, and enabling the cooldownShares and the cooldownAssets functions.
 */
contract StakedUSDeV2 is IStakedUSDeCooldown, StakedUSDe {
  using SafeERC20 for IERC20;

  mapping(address => UserCooldown) public cooldowns;

  USDeSilo public immutable silo;

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

  /// @notice Constructor for StakedUSDeV2 contract.
  /// @param _asset The address of the USDe token.
  /// @param initialRewarder The address of the initial rewarder.
  /// @param _owner The address of the admin role.
  constructor(IERC20 _asset, address initialRewarder, address _owner) StakedUSDe(_asset, initialRewarder, _owner) {
    silo = new USDeSilo(address(this), address(_asset));
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

  /// @notice Set cooldown duration. If cooldown duration is set to zero, the StakedUSDeV2 behavior changes to follow ERC4626 standard and disables cooldownShares and cooldownAssets methods. If cooldown duration is greater than zero, the ERC4626 withdrawal and redeem functions are disabled, breaking the ERC4626 standard, and enabling the cooldownShares and the cooldownAssets functions.
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
 ------------
 ------------ END OF MAIN TARGET CONTRACT ------------ 

 ------------ ## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES ------------ 
// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.20;

import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/interfaces/IERC5313.sol";
import "./interfaces/ISingleAdminAccessControl.sol";

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

// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.20;

/* solhint-disable private-vars-leading-underscore */

import "@openzeppelin/contracts/token/ERC20/extensions/ERC4626.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol";
import "./SingleAdminAccessControl.sol";
import "./interfaces/IStakedUSDe.sol";

/**
 * @title StakedUSDe
 * @notice The StakedUSDe contract allows users to stake USDe tokens and earn a portion of protocol LST and perpetual yield that is allocated
 * to stakers by the Ethena DAO governance voted yield distribution algorithm.  The algorithm seeks to balance the stability of the protocol by funding
 * the protocol's insurance fund, DAO activities, and rewarding stakers with a portion of the protocol's yield.
 */
contract StakedUSDe is SingleAdminAccessControl, ReentrancyGuard, ERC20Permit, ERC4626, IStakedUSDe {
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
  /// @notice The vesting period of lastDistributionAmount over which it increasingly becomes available to stakers
  uint256 private constant VESTING_PERIOD = 8 hours;
  /// @notice Minimum non-zero shares amount to prevent donation attack
  uint256 private constant MIN_SHARES = 1 ether;

  /* ------------- STATE VARIABLES ------------- */

  /// @notice The amount of the last asset distribution from the controller contract into this
  /// contract + any unvested remainder at that time
  uint256 public vestingAmount;

  /// @notice The timestamp of the last asset distribution from the controller contract into this contract
  uint256 public lastDistributionTimestamp;

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
   * @notice Constructor for StakedUSDe contract.
   * @param _asset The address of the USDe token.
   * @param _initialRewarder The address of the initial rewarder.
   * @param _owner The address of the admin role.
   *
   */
  constructor(IERC20 _asset, address _initialRewarder, address _owner)
    ERC20("Staked USDe", "sUSDe")
    ERC4626(_asset)
    ERC20Permit("sUSDe")
  {
    if (_owner == address(0) || _initialRewarder == address(0) || address(_asset) == address(0)) {
      revert InvalidZeroAddress();
    }

    _grantRole(REWARDER_ROLE, _initialRewarder);
    _grantRole(DEFAULT_ADMIN_ROLE, _owner);
  }

  /* ------------- EXTERNAL ------------- */

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
   * Note that the owner cannot rescue USDe tokens because they functionally sit here
   * and belong to stakers but can rescue staked USDe as they should never actually
   * sit in this contract and a staker may well transfer them here by accident.
   * @param token The token to be rescued.
   * @param amount The amount of tokens to be rescued.
   * @param to Where to send rescued tokens
   */
  function rescueTokens(address token, uint256 amount, address to) external nonReentrant onlyRole(DEFAULT_ADMIN_ROLE) {
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
      uint256 usdeToVest = previewRedeem(amountToDistribute);
      _burn(from, amountToDistribute);
      // to address of address(0) enables burning
      if (to == address(0)) {
        _updateVestingAmount(usdeToVest);
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
   * @notice Returns the amount of USDe tokens that are vested in the contract.
   */
  function totalAssets() public view override returns (uint256) {
    return IERC20(asset()).balanceOf(address(this)) - getUnvestedAmount();
  }

  /**
   * @notice Returns the amount of USDe tokens that are unvested in the contract.
   */
  function getUnvestedAmount() public view returns (uint256) {
    uint256 timeSinceLastDistribution = block.timestamp - lastDistributionTimestamp;

    if (timeSinceLastDistribution >= VESTING_PERIOD) {
      return 0;
    }

    uint256 deltaT;
    unchecked {
      deltaT = (VESTING_PERIOD - timeSinceLastDistribution);
    }
    return (deltaT * vestingAmount) / VESTING_PERIOD;
  }

  /// @dev Necessary because both ERC20 (from ERC20Permit) and ERC4626 declare decimals()
  function decimals() public pure override(ERC4626, ERC20) returns (uint8) {
    return 18;
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
pragma solidity ^0.8.0;

/* solhint-disable var-name-mixedcase  */

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "../contracts/interfaces/IUSDeSiloDefinitions.sol";

/**
 * @title USDeSilo
 * @notice The Silo allows to store USDe during the stake cooldown process.
 */
contract USDeSilo is IUSDeSiloDefinitions {
  address immutable _STAKING_VAULT;
  IERC20 immutable _USDE;

  constructor(address stakingVault, address usde) {
    _STAKING_VAULT = stakingVault;
    _USDE = IERC20(usde);
  }

  modifier onlyStakingVault() {
    if (msg.sender != _STAKING_VAULT) revert OnlyStakingVault();
    _;
  }

  function withdraw(address to, uint256 amount) external onlyStakingVault {
    _USDE.transfer(to, amount);
  }
}

// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.20;

interface IUSDeSiloDefinitions {
  /// @notice Error emitted when the staking vault is not the caller
  error OnlyStakingVault();
}

// SPDX-License-Identifier: MIT

pragma solidity 0.8.20;

import "./IStakedUSDe.sol";

struct UserCooldown {
  uint104 cooldownEnd;
  uint152 underlyingAmount;
}

interface IStakedUSDeCooldown is IStakedUSDe {
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
pragma solidity 0.8.20;

interface ISingleAdminAccessControl {
  error InvalidAdminChange();
  error NotPendingAdmin();

  event AdminTransferred(address indexed oldAdmin, address indexed newAdmin);
  event AdminTransferRequested(address indexed oldAdmin, address indexed newAdmin);
}

// SPDX-License-Identifier: MIT

pragma solidity 0.8.20;

interface IStakedUSDe {
  // Events //
  /// @notice Event emitted when the rewards are received
  event RewardsReceived(uint256 amount);
  /// @notice Event emitted when the balance from an FULL_RESTRICTED_STAKER_ROLE user are redistributed
  event LockedAmountRedistributed(address indexed from, address indexed to, uint256 amount);

  // Errors //
  /// @notice Error emitted shares or assets equal zero.
  error InvalidAmount();
  /// @notice Error emitted when owner attempts to rescue USDe tokens.
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

  function transferInRewards(uint256 amount) external;

  function rescueTokens(address token, uint256 amount, address to) external;

  function getUnvestedAmount() external view returns (uint256);
}

// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.20;

interface IUSDeSiloDefinitions {
  /// @notice Error emitted when the staking vault is not the caller
  error OnlyStakingVault();
}


## ------------ SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS ------------ 

## ------------ SUPPORTING CONTEXT: EXTERNAL LIBRARIES ------------ 

 ------------ END OF SUPPORTING CONTRACTS AND INTERFACES ------------ 


 ------------ ## DEPLOYMENT SCRIPTS ------------ 

