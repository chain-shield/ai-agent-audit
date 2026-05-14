
 ------------ ## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.20;

import "@openzeppelin/contracts/access/Ownable2Step.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "./interfaces/IEthenaLPStakingDefinitions.sol";

/**
 * @title EthenaLPStaking
 * @notice Allows liquidity providers in various USDe pools to stake their LP tokens
 * in order to earn shards toward the Ethena airdrop. There will be a series of epochs,
 * with certain pools eligible to stake in a given epoch. Reward computation and distribution
 * is handled off-chain.  This contract is only used to hold the staked LP tokens with a
 * cooldown period on withdrawing stakes.
 */

contract EthenaLPStaking is Ownable2Step, IEthenaLPStakingDefinitions, ReentrancyGuard {
  using SafeERC20 for IERC20;

  // ---------------------- Constants -----------------------

  /// @notice placeholder address for ETH
  address internal constant _ETH_ADDRESS = 0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE;

  /// @notice the maximum cooldown period the owner can set for any LP token
  uint48 internal constant _MAX_COOLDOWN_PERIOD = 90 days;

  // ----------------------- Storage ------------------------

  /// @notice tracks the current epoch
  uint8 public currentEpoch;

  /// @notice tracks all stakes, indexed by user and LP token
  mapping(address => mapping(address => StakeData)) public stakes;

  /// @notice tracks stake parameters for each LP token, indexed by LP token address
  mapping(address => StakeParameters) public stakeParametersByToken;

  // --------------------- Constructor ----------------------

  constructor(address _initialOwner) {
    if (_initialOwner == address(0)) revert ZeroAddressException();
    _transferOwnership(_initialOwner);
  }

  // ---------------------- Modifiers -----------------------

  /**
   * @notice checks that the amount is not 0
   * @param amount the amount to check
   */
  modifier checkAmount(uint256 amount) {
    if (amount == 0) revert InvalidAmount();
    _;
  }

  // ------------------- Owner Functions --------------------

  /**
   * @notice owner can change epoch
   * @param newEpoch the new epoch
   */
  function setEpoch(uint8 newEpoch) external onlyOwner {
    if (newEpoch == currentEpoch) revert InvalidEpoch();
    emit NewEpoch(newEpoch, currentEpoch);
    currentEpoch = newEpoch;
  }

  /**
   * @notice owner can add/update stake parameters for a given LP token
   * @param token the LP token to update stake parameters for
   * @param epoch the epoch the token is eligible for staking
   * @param stakeLimit the maximum amount of LP tokens that can be staked
   * @param cooldown the cooldown period for withdrawing LP tokens
   */
  function updateStakeParameters(address token, uint8 epoch, uint248 stakeLimit, uint48 cooldown) external onlyOwner {
    if (cooldown > _MAX_COOLDOWN_PERIOD) revert MaxCooldownExceeded();
    StakeParameters storage stakeParameters = stakeParametersByToken[token];
    // owner cannot modify total staked or cooling down
    stakeParameters.epoch = epoch;
    stakeParameters.stakeLimit = stakeLimit;
    stakeParameters.cooldown = cooldown;
    emit StakeParametersUpdated(token, epoch, stakeLimit, cooldown);
  }

  /**
   * @notice owner can rescue tokens that were accidentally sent to the contract
   * @param token the token to transfer
   * @param to the address to send the tokens to
   * @param amount the amount of tokens to send
   */
  function rescueTokens(address token, address to, uint256 amount) external onlyOwner nonReentrant checkAmount(amount) {
    if (to == address(0)) revert ZeroAddressException();
    // contract should never hold ETH
    if (token == _ETH_ADDRESS) {
      (bool success,) = to.call{value: amount}("");
      if (!success) revert TransferFailed();
    } else {
      IERC20(token).safeTransfer(to, amount);
      _checkInvariant(token);
    }
    emit TokensRescued(token, to, amount);
  }

  /// @notice Prevents the owner from renouncing ownership, must be transferred in 2 steps
  function renounceOwnership() public view override onlyOwner {
    revert CantRenounceOwnership();
  }

  // ----------------------- User Functions ------------------------

  /**
   * @notice users can stake LP tokens to earn shards toward airdrop
   * @param token the LP token to stake
   * @param amount the amount of LP tokens to stake
   */
  function stake(address token, uint104 amount) external nonReentrant checkAmount(amount) {
    StakeParameters storage stakeParameters = stakeParametersByToken[token];
    // can only stake when it is the correct epoch
    if (currentEpoch != stakeParameters.epoch) revert InvalidEpoch();
    if (stakeParameters.totalStaked + amount > stakeParameters.stakeLimit) revert StakeLimitExceeded();
    stakeParameters.totalStaked += amount;
    stakes[msg.sender][token].stakedAmount += amount;
    IERC20(token).safeTransferFrom(msg.sender, address(this), amount);
    _checkInvariant(token);
    emit Stake(msg.sender, token, amount);
  }

  /**
   * @notice users can unstake LP tokens to initiate the cooldown period.
   * They will not be able to withdraw until the cooldown period has passed and do not earn rewards during this period.
   * @param token the LP token to unstake
   * @param amount the amount of LP tokens to unstake
   */
  function unstake(address token, uint104 amount) external nonReentrant checkAmount(amount) {
    StakeParameters storage stakeParameters = stakeParametersByToken[token];
    StakeData storage stakeData = stakes[msg.sender][token];
    if (stakeData.stakedAmount < amount) revert InvalidAmount();
    stakeData.stakedAmount -= amount;
    stakeData.coolingDownAmount += amount;
    stakeData.cooldownStartTimestamp = uint104(block.timestamp);
    stakeParameters.totalStaked -= amount;
    stakeParameters.totalCoolingDown += amount;
    _checkInvariant(token);
    emit Unstake(msg.sender, token, amount);
  }

  /**
   * @notice users can withdraw LP tokens after the cooldown period has passed
   * @param token the LP token to withdraw
   * @param amount the amount of LP tokens to withdraw
   */
  function withdraw(address token, uint104 amount) external nonReentrant checkAmount(amount) {
    StakeParameters storage stakeParameters = stakeParametersByToken[token];
    StakeData storage stakeData = stakes[msg.sender][token];
    if (stakeData.coolingDownAmount < amount) revert InvalidAmount();
    if (block.timestamp < stakeData.cooldownStartTimestamp + stakeParameters.cooldown) revert CooldownNotOver();
    stakeData.coolingDownAmount -= amount;
    stakeParameters.totalCoolingDown -= amount;
    IERC20(token).safeTransfer(msg.sender, amount);
    _checkInvariant(token);
    emit Withdraw(msg.sender, token, amount);
  }

  // ----------------------- Internal Functions ------------------------

  /**
   * @notice checks that the invariant is not broken
   * @param token the LP token to check
   * @dev the invariant is that the contract should never hold less of a token than the total staked and cooling down
   * @dev despite the higher gas cost of an extra sload here, we intentionally do not pass in the stake parameters
   * because we want to ensure that the invariant is checked against the current state of the contract
   */
  function _checkInvariant(address token) internal view {
    StakeParameters storage stakeParameters = stakeParametersByToken[token];
    uint256 balance = IERC20(token).balanceOf(address(this));
    if (balance < stakeParameters.totalStaked + stakeParameters.totalCoolingDown) revert InvariantBroken();
  }
}
 ------------
 ------------ END OF MAIN TARGET CONTRACT ------------ 

 ------------ ## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES ------------ 
// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.20;

interface IEthenaLPStakingDefinitions {
  /// @notice information about staking for a particular LP token
  struct StakeParameters {
    uint8 epoch;
    uint248 stakeLimit;
    uint104 totalStaked; // total deposited and not in cooldown
    uint104 totalCoolingDown;
    uint48 cooldown;
  }

  /// @notice information about a particular stake by user and LP token
  struct StakeData {
    uint256 stakedAmount;
    uint152 coolingDownAmount;
    uint104 cooldownStartTimestamp;
  }

  /// @notice emitted when an epoch begins
  event NewEpoch(uint8 indexed newEpoch, uint8 indexed previousEpoch);

  /// @notice emitted when staking parameters are added/updated for an LP token
  event StakeParametersUpdated(address indexed lpToken, uint8 indexed epoch, uint248 stakeLimit, uint104 cooldown);

  /// @notice emitted when a user stakes
  event Stake(address indexed user, address indexed lpToken, uint256 amount);

  /// @notice emitted when a user unstakes
  event Unstake(address indexed user, address indexed lpToken, uint256 amount);

  /// @notice emitted when a user withdraws
  event Withdraw(address indexed user, address indexed lpToken, uint256 amount);

  /// @notice emitted when tokens are rescued by owner
  event TokensRescued(address indexed token, address indexed to, uint256 amount);

  /// @notice ownership cannot be renounced
  error CantRenounceOwnership();

  /// @notice Error returned when a user tries staking more than the limit for a given token
  error StakeLimitExceeded();

  /// @notice Error returned when staking LP token during wrong epoch
  error InvalidEpoch();

  /// @notice zero amount or amount greater than a max such as amount staked
  error InvalidAmount();

  /// @notice Error returned when native ETH transfer fails
  error TransferFailed();

  /// @notice Error returned when excess balance of an LP token is less than 0
  error InvariantBroken();

  /// @notice Error returned when owner sets cooldown > 1 year
  error MaxCooldownExceeded();

  /// @notice Error returned when user attempts to withdraw before cooldown period is over
  error CooldownNotOver();

  /// @notice This error is returned if the zero address is used
  error ZeroAddressException();
}


## ------------ SUPPORTING CONTEXT: INTERFACES AND ROOT IMPLEMENTATIONS ------------ 

## ------------ SUPPORTING CONTEXT: EXTERNAL LIBRARIES ------------ 

 ------------ END OF SUPPORTING CONTRACTS AND INTERFACES ------------ 


 ------------ ## DEPLOYMENT SCRIPTS ------------ 

