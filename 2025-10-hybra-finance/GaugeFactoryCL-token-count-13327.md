
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import '../interfaces/IPermissionsRegistry.sol';
import '../interfaces/IGaugeFactoryCL.sol';
import './GaugeCL.sol';
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {HybraTimeLibrary} from "../libraries/HybraTimeLibrary.sol";


interface IGaugeCL {
    function activateEmergencyMode() external;
    function stopEmergencyMode() external;
    function setInternalBribe(address intbribe) external;
}

contract GaugeFactoryCL is IGaugeFactoryCL, OwnableUpgradeable {

    using SafeERC20 for IERC20;

    address public last_gauge;
    address public permissionsRegistry;

    address[] internal __gauges;
    address internal rHYBR;

    
    constructor() {}

    function initialize(address _permissionRegistry) initializer  public {
        __Ownable_init();   //after deploy ownership to multisig
        permissionsRegistry = _permissionRegistry;
    }

    function setRHYBR(address _rHYBR) external {
        require(owner() == msg.sender, 'not owner');
        rHYBR = _rHYBR;
    }

 

    modifier onlyAllowed() {
        require(owner() == msg.sender || IPermissionsRegistry(permissionsRegistry).hasRole("GAUGE_ADMIN",msg.sender), 'ERR: GAUGE_ADMIN');
        _;
    }

    function setRegistry(address _registry) external {
        require(owner() == msg.sender, 'not owner');
        permissionsRegistry = _registry;
    }


    function createGauge(address _rewardToken,address _ve,address _pool,address _distribution, address _internal_bribe, address _external_bribe, bool _isPair, 
                        address nfpm) external returns (address) {
        

        last_gauge = address(new GaugeCL(_rewardToken,rHYBR,_ve,_pool,_distribution,_internal_bribe,_external_bribe,_isPair, nfpm, address(this)));
        __gauges.push(last_gauge);
        return last_gauge;
    }



    function gauges(uint256 i) external view returns(address) {
        return __gauges[i];
    }

    modifier EmergencyCouncil() {
        require( msg.sender == IPermissionsRegistry(permissionsRegistry).emergencyCouncil() );
        _;
    }

    function activateEmergencyMode( address[] memory _gauges) external EmergencyCouncil {
        uint i = 0;
        for ( i ; i < _gauges.length; i++){
            IGaugeCL(_gauges[i]).activateEmergencyMode();
        }
    }

    function stopEmergencyMode( address[] memory _gauges) external EmergencyCouncil {
        uint i = 0;
        for ( i ; i < _gauges.length; i++){
            IGaugeCL(_gauges[i]).stopEmergencyMode();
        }
    }

    function setInternalBribe(address[] memory _gauges,  address[] memory int_bribe) external onlyAllowed {
        require(_gauges.length == int_bribe.length);
        uint i = 0;
        for ( i ; i < _gauges.length; i++){
            IGaugeCL(_gauges[i]).setInternalBribe(int_bribe[i]);
        }
    }

    function length() external view returns(uint) {
        return __gauges.length;
    }

    
}
END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
pragma solidity 0.8.13;

import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import "@openzeppelin/contracts/token/ERC721/IERC721Receiver.sol";
 import "@openzeppelin/contracts/utils/math/SafeCast.sol";
import '../interfaces/IGaugeFactoryCL.sol';
import '../interfaces/IGaugeManager.sol';
import './interface/ICLPool.sol';
import './interface/ICLFactory.sol';
import './interface/INonfungiblePositionManager.sol';
import '../interfaces/IBribe.sol';
import '../interfaces/IRHYBR.sol';
import {HybraTimeLibrary} from "../libraries/HybraTimeLibrary.sol";
import {FullMath} from "./libraries/FullMath.sol";
import {FixedPoint128} from "./libraries/FixedPoint128.sol";
import '../interfaces/IRHYBR.sol';



contract GaugeCL is ReentrancyGuard, Ownable, IERC721Receiver {

    using SafeERC20 for IERC20;
    using EnumerableSet for EnumerableSet.UintSet;
    using SafeCast for uint128;
    IERC20 public immutable rewardToken;
    address public immutable rHYBR;
    address public VE;
    address public DISTRIBUTION;
    address public internal_bribe;
    address public external_bribe;

    uint256 public DURATION;
    uint256 internal _periodFinish;
    uint256 public rewardRate;
    ICLPool public clPool;
    address public poolAddress;
    INonfungiblePositionManager public nonfungiblePositionManager;
    
    bool public emergency;
    bool public immutable isForPair;
    address immutable factory;

    mapping(uint256 => uint256) public  rewardRateByEpoch; // epoch => reward rate
    mapping(address => EnumerableSet.UintSet) internal _stakes;
    mapping(uint256 => uint256) public  rewardGrowthInside;

    mapping(uint256 => uint256) public  rewards;

    mapping(uint256 => uint256) public  lastUpdateTime;

    event RewardAdded(uint256 reward);
    event Deposit(address indexed user, uint256 amount);
    event Withdraw(address indexed user, uint256 amount);
    event Harvest(address indexed user, uint256 reward);
    event ClaimFees(address indexed from, uint256 claimed0, uint256 claimed1);
    event EmergencyActivated(address indexed gauge, uint256 timestamp);
    event EmergencyDeactivated(address indexed gauge, uint256 timestamp);

    constructor(address _rewardToken, address _rHYBR, address _ve, address _pool, address _distribution, address _internal_bribe, 
        address _external_bribe, bool _isForPair, address nfpm,  address _factory) {
        factory = _factory;
        rewardToken = IERC20(_rewardToken);     // main reward
        rHYBR = _rHYBR;
        VE = _ve;                               // vested
        poolAddress = _pool;
        clPool = ICLPool(_pool);
        DISTRIBUTION = _distribution;           // distro address (GaugeManager)
        DURATION = HybraTimeLibrary.WEEK;                   

        internal_bribe = _internal_bribe;       // lp fees goes here
        external_bribe = _external_bribe;       // bribe fees goes here
        isForPair = _isForPair;
        nonfungiblePositionManager = INonfungiblePositionManager(nfpm);
        emergency = false;
    }

    modifier onlyDistribution() {
        require(msg.sender == DISTRIBUTION, "Caller is not RewardsDistribution contract");
        _;
    }

    modifier isNotEmergency() {
        require(emergency == false, "emergency");
        _;
    }


    function _updateRewards(uint256 tokenId, int24 tickLower, int24 tickUpper) internal {
        if (lastUpdateTime[tokenId] == block.timestamp) return;
        clPool.updateRewardsGrowthGlobal();
        lastUpdateTime[tokenId] = block.timestamp;
        rewards[tokenId] += _earned(tokenId);
        rewardGrowthInside[tokenId] = clPool.getRewardGrowthInside(tickLower, tickUpper, 0);
    }

    function activateEmergencyMode() external onlyOwner {
        require(emergency == false, "emergency");
        emergency = true;
        emit EmergencyActivated(address(this), block.timestamp);
    }

    function stopEmergencyMode() external onlyOwner {

        require(emergency == true,"emergency");

        emergency = false;
        emit EmergencyDeactivated(address(this), block.timestamp);
    }

    function balanceOf(uint256 tokenId) external view returns (uint256) {
        (,,,,,,,uint128 liquidity,,,,) = nonfungiblePositionManager.positions(tokenId);
        return liquidity;
    }

    function _getPoolAddress(address token0, address token1, int24 tickSpacing) internal view returns (address) {
        return ICLFactory(nonfungiblePositionManager.factory()).getPool(token0, token1, tickSpacing);
    }

    function earned(uint256 tokenId) external view returns (uint256 reward) {
        require(_stakes[msg.sender].contains(tokenId), "NA");

        uint256 reward = _earned(tokenId);
        return (reward); // bonsReward is 0 for now
    }

       function _earned(uint256 tokenId) internal view returns (uint256) {
        uint256 lastUpdated = clPool.lastUpdated();

        uint256 timeDelta = block.timestamp - lastUpdated;

        
        uint256 rewardGrowthGlobalX128 = clPool.rewardGrowthGlobalX128();
        uint256 rewardReserve = clPool.rewardReserve();

        if (timeDelta != 0 && rewardReserve > 0 && clPool.stakedLiquidity() > 0) {
            uint256 reward = rewardRate * timeDelta;
            if (reward > rewardReserve) reward = rewardReserve;

            rewardGrowthGlobalX128 += FullMath.mulDiv(reward, FixedPoint128.Q128, clPool.stakedLiquidity());
        }

        (,,,,, int24 tickLower, int24 tickUpper, uint128 liquidity,,,,) = nonfungiblePositionManager.positions(tokenId);

        uint256 rewardPerTokenInsideInitialX128 = rewardGrowthInside[tokenId];
        uint256 rewardPerTokenInsideX128 = clPool.getRewardGrowthInside(tickLower, tickUpper, rewardGrowthGlobalX128);

        uint256 claimable =
            FullMath.mulDiv(rewardPerTokenInsideX128 - rewardPerTokenInsideInitialX128, liquidity, FixedPoint128.Q128);
        return claimable;
    }

    function deposit(uint256 tokenId) external nonReentrant isNotEmergency {
        
         (,,address token0, address token1, int24 tickSpacing, int24 tickLower, int24 tickUpper, uint128 liquidity,,,,) = 
            nonfungiblePositionManager.positions(tokenId);
        
        require(liquidity > 0, "Gauge: zero liquidity");
        // Calculate pool address from position parameters
        address positionPool = _getPoolAddress(token0, token1, tickSpacing);
        // Verify that the position's pool matches this gauge's pool
        require(positionPool == poolAddress, "Pool mismatch: Position not for this gauge pool");
        // collect fees 
        nonfungiblePositionManager.collect(INonfungiblePositionManager.CollectParams({
                tokenId: tokenId,
                recipient: msg.sender,
                amount0Max: type(uint128).max,
                amount1Max: type(uint128).max
            }));

        nonfungiblePositionManager.safeTransferFrom(msg.sender, address(this), tokenId);

        clPool.stake(int128(liquidity), tickLower, tickUpper, true);


        uint256 rewardGrowth = clPool.getRewardGrowthInside(tickLower, tickUpper, 0);
        rewardGrowthInside[tokenId] = rewardGrowth;
        lastUpdateTime[tokenId] = block.timestamp;

        _stakes[msg.sender].add(tokenId);

        emit Deposit(msg.sender, tokenId);
    }

    function withdraw(uint256 tokenId, uint8 redeemType) external nonReentrant isNotEmergency {
           require(_stakes[msg.sender].contains(tokenId), "NA");

        // trigger update on staked position so NFT will be in sync with the pool
        nonfungiblePositionManager.collect(
            INonfungiblePositionManager.CollectParams({
                tokenId: tokenId,
                recipient: msg.sender,
                amount0Max: type(uint128).max,
                amount1Max: type(uint128).max
            })
        );

        (,,,,, int24 tickLower, int24 tickUpper, uint128 liquidityToStake,,,,) = nonfungiblePositionManager.positions(tokenId);
        _getReward(tickLower, tickUpper, tokenId, msg.sender, redeemType);

        // update virtual liquidity in pool only if token has existing liquidity
        // i.e. not all removed already via decreaseStakedLiquidity
        if (liquidityToStake != 0) {
            clPool.stake(-int128(liquidityToStake), tickLower, tickUpper, true);
        }

        _stakes[msg.sender].remove(tokenId);
        nonfungiblePositionManager.safeTransferFrom(address(this), msg.sender, tokenId);

        emit Withdraw(msg.sender, tokenId);
    }

    

    function getReward(uint256 tokenId, address account,uint8 redeemType ) public nonReentrant onlyDistribution {

        require(_stakes[account].contains(tokenId), "NA");

        (,,,,, int24 tickLower, int24 tickUpper,,,,,) = nonfungiblePositionManager.positions(tokenId);
        _getReward(tickLower, tickUpper, tokenId, account, redeemType);
    }


    function _getReward(int24 tickLower, int24 tickUpper, uint256 tokenId,address account, uint8 redeemType) internal {
        _updateRewards(tokenId, tickLower, tickUpper);
        uint256 rewardAmount = rewards[tokenId];
        if(rewardAmount > 0){
            delete rewards[tokenId];
            rewardToken.safeApprove(rHYBR, rewardAmount);
            IRHYBR(rHYBR).depostionEmissionsToken(rewardAmount);
            IRHYBR(rHYBR).redeemFor(rewardAmount, redeemType, account);
        }
        emit Harvest(msg.sender, rewardAmount);
    }

    function notifyRewardAmount(address token, uint256 rewardAmount) external nonReentrant
        isNotEmergency onlyDistribution returns (uint256 currentRate) {
        require(token == address(rewardToken), "Invalid reward token");

        // Update global reward growth before processing new rewards
        clPool.updateRewardsGrowthGlobal();

        // Calculate time remaining until next epoch begins
        uint256 epochTimeRemaining = HybraTimeLibrary.epochNext(block.timestamp) - block.timestamp;
        uint256 epochEndTimestamp = block.timestamp + epochTimeRemaining;

        // Include any rolled over rewards from previous period
        uint256 totalRewardAmount = rewardAmount + clPool.rollover();

        // Check if we are starting a new reward period or continuing existing one
        if (block.timestamp >= _periodFinish) {
            // New period: distribute rewards over remaining epoch time
            rewardRate = rewardAmount / epochTimeRemaining;
            clPool.syncReward({
                rewardRate: rewardRate,
                rewardReserve: totalRewardAmount,
                periodFinish: epochEndTimestamp
            });
        } else {
            // Existing period: add new rewards to pending distribution
            uint256 pendingRewards = epochTimeRemaining * rewardRate;
            rewardRate = (rewardAmount + pendingRewards) / epochTimeRemaining;
            clPool.syncReward({
                rewardRate: rewardRate,
                rewardReserve: totalRewardAmount + pendingRewards,
                periodFinish: epochEndTimestamp
            });
        }

        // Store reward rate for current epoch tracking
        rewardRateByEpoch[HybraTimeLibrary.epochStart(block.timestamp)] = rewardRate;

        // Transfer reward tokens from distributor to gauge
        rewardToken.safeTransferFrom(DISTRIBUTION, address(this), rewardAmount);

        // Verify contract has sufficient balance to support calculated reward rate
        uint256 contractBalance = rewardToken.balanceOf(address(this));
        require(rewardRate <= contractBalance / epochTimeRemaining, "Insufficient balance for reward rate");

        // Update period finish time and return current rate
        _periodFinish = epochEndTimestamp;
        currentRate = rewardRate;

        emit RewardAdded(rewardAmount);
    }

    function gaugeBalances() external view returns (uint256 token0, uint256 token1){
        
        (token0, token1) = clPool.gaugeFees();

    }

  



    function claimFees() external nonReentrant returns (uint256 claimed0, uint256 claimed1) {
        return _claimFees();
    }

    function _claimFees() internal returns (uint256 claimed0, uint256 claimed1) {
        if (!isForPair) {
            return (0, 0);
        }
        
        clPool.collectFees();
        
        address _token0 = clPool.token0();
        address _token1 = clPool.token1();
        // Fetch fee from the whole epoch which just eneded and transfer it to internal Bribe address.
        claimed0 = IERC20(_token0).balanceOf(address(this));
        claimed1 = IERC20(_token1).balanceOf(address(this));

        if (claimed0 > 0 || claimed1 > 0) {
    

            uint256 _fees0 = claimed0;
            uint256 _fees1 = claimed1;

            if (_fees0  > 0) {
                IERC20(_token0).safeApprove(internal_bribe, 0);
                IERC20(_token0).safeApprove(internal_bribe, _fees0);
                IBribe(internal_bribe).notifyRewardAmount(_token0, _fees0);
            } 
            if (_fees1  > 0) {
                IERC20(_token1).safeApprove(internal_bribe, 0);
                IERC20(_token1).safeApprove(internal_bribe, _fees1);
                IBribe(internal_bribe).notifyRewardAmount(_token1, _fees1);
            } 
            emit ClaimFees(msg.sender, claimed0, claimed1);
        }
    }

    ///@notice get total reward for the duration
    function rewardForDuration() external view returns (uint256) {
        return rewardRate * DURATION;
    }

    ///@notice set new internal bribe contract (where to send fees)
    function setInternalBribe(address _int) external onlyOwner {
        require(_int >= address(0), "zero");
        internal_bribe = _int;
    }

    function _safeTransfer(address token,address to,uint256 value) internal {
        require(token.code.length > 0);
        (bool success, bytes memory data) = token.call(abi.encodeWithSelector(IERC20.transfer.selector, to, value));
        require(success && (data.length == 0 || abi.decode(data, (bool))));
    }

    /**
     * @dev Handle the receipt of an NFT
     * @param operator The address which called `safeTransferFrom` function
     * @param from The address which previously owned the token
     * @param tokenId The NFT identifier which is being transferred
     * @param data Additional data with no specified format
     * @return bytes4 `bytes4(keccak256("onERC721Received(address,address,uint256,bytes)"))`
     */
    function onERC721Received(
        address operator,
        address from,
        uint256 tokenId,
        bytes calldata data
    ) external pure override returns (bytes4) {
        return IERC721Receiver.onERC721Received.selector;
    }

}



// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

library HybraTimeLibrary {

    // for testnet
    uint256 internal constant WEEK = 1800;
    uint internal constant NO_VOTING_WINDOW = 300;
    uint256 internal constant MAX_LOCK_DURATION = 86400 * 365 * 2;
    uint256 internal constant GENESIS_STAKING_MATURITY_TIME = 2 * 86400;
    uint256 internal constant NO_GENESIS_DEPOSIT_WINDOW = 600;

    // uint256 internal constant WEEK = 7 * 86400;
    // uint internal constant NO_VOTING_WINDOW = 3600;
    // uint256 internal constant MAX_LOCK_DURATION = 86400 * 365 * 4;
    // uint256 internal constant GENESIS_STAKING_MATURITY_TIME = 180 * 86400;
    // uint256 internal constant NO_GENESIS_DEPOSIT_WINDOW = 3 * 3600;

    /// @dev Returns start of epoch based on current timestamp
    function epochStart(uint256 timestamp) internal pure returns (uint256) {
        unchecked {
            return timestamp - (timestamp % WEEK);
        }
    }

    /// @dev Returns start of next epoch / end of current epoch
    function epochNext(uint256 timestamp) internal pure returns (uint256) {
        unchecked {
            return timestamp - (timestamp % WEEK) + WEEK;
        }
    }

    /// @dev Returns start of voting window
    function epochVoteStart(uint256 timestamp) internal pure returns (uint256) {
        unchecked {
            return timestamp - (timestamp % WEEK) + NO_VOTING_WINDOW;
        }
    }

    /// @dev Returns end of voting window / beginning of unrestricted voting window
    function epochVoteEnd(uint256 timestamp) internal pure returns (uint256) {
        unchecked {
            return timestamp - (timestamp % WEEK) + WEEK - NO_VOTING_WINDOW;
        }
    }

    /// @dev Returns the status if it is the last hour of the epoch
    function isLastHour(uint256 timestamp) internal pure returns (bool) {
        // return block.timestamp % 7 days >= 6 days + 23 hours;
        return timestamp >= HybraTimeLibrary.epochVoteEnd(timestamp) 
        && timestamp < HybraTimeLibrary.epochNext(timestamp);
    }

    /// @dev Returns duration in multiples of epoch
    function epochMultiples(uint256 duration) internal pure returns (uint256) {
        unchecked {
            return (duration / WEEK) * WEEK;
        }
    }

    /// @dev Returns duration in multiples of epoch
    function isLastEpoch(uint256 timestamp, uint256 endTime) internal pure returns (bool) {
        unchecked {
            return  endTime - WEEK <= timestamp && timestamp < endTime;
        }
    }

    /// @dev Returns duration in multiples of epoch
    function prevPreEpoch(uint256 timestamp) internal pure returns (uint256) {
        unchecked {
            return  epochStart(timestamp) - NO_GENESIS_DEPOSIT_WINDOW;
        }
    }

    /// @dev Returns duration in multiples of epoch
    function currPreEpoch(uint256 timestamp) internal pure returns (uint256) {
        unchecked {
            return  epochNext(timestamp) - NO_GENESIS_DEPOSIT_WINDOW;
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

interface IBribe {
    function deposit(uint amount, uint tokenId) external;
    function withdraw(uint amount, uint tokenId) external;
    function getRewardForAddress(address _owner, address[] memory tokens) external;
    function notifyRewardAmount(address token, uint amount) external;
    function left(address token) external view returns (uint);
    function getReward(uint tokenId, address[] memory tokens) external;
    function bribeTokens(uint256 i) external view returns(address); 
    function rewardsListLength() external view returns (uint256);
    function tokenRewardsPerEpoch(address _token, uint256 epochStart) external view returns(uint256);
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

interface IPermissionsRegistry {
    function emergencyCouncil() external view returns(address);
    function hybraTeamMultisig() external view returns(address);
    function hasRole(bytes memory role, address caller) external view returns(bool);
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

interface IGaugeCL {
    function notifyRewardAmount(address token, uint amount) external returns ( uint256 rewardRate);
    function getReward(uint256 tokenId, address account, uint8 redeemType) external;
    function claimFees() external returns (uint claimed0, uint claimed1);
    function balanceOf(uint256 tokenId) external view returns (uint256); 
    function emergency() external returns (bool);
    function gaugeBalances() external view returns (uint256 token0, uint256 token1);
    function earned(uint256 tokenId) external view returns (uint256 reward, uint256 bonusReward);   
    function totalSupply() external view returns (uint);
    function rewardRate() external view returns (uint);
    function rewardForDuration() external view returns (uint256);
    function stakedFees() external view returns (uint256, uint256);
}
// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity >=0.5.0;

import {IVoter} from "contracts/core/interfaces/IVoter.sol";
import {IFactoryRegistry} from "contracts/core/interfaces/IFactoryRegistry.sol";
import {IGaugeManager} from "contracts/core/interfaces/IGaugeManager.sol";


/// @title The interface for the CL Factory
/// @notice The CL Factory facilitates creation of CL pools and control over the protocol fees
interface ICLFactory {
    /// @notice Emitted when the owner of the factory is changed
    /// @param oldOwner The owner before the owner was changed
    /// @param newOwner The owner after the owner was changed
    event OwnerChanged(address indexed oldOwner, address indexed newOwner);

    /// @notice Emitted when the swapFeeManager of the factory is changed
    /// @param oldFeeManager The swapFeeManager before the swapFeeManager was changed
    /// @param newFeeManager The swapFeeManager after the swapFeeManager was changed
    event SwapFeeManagerChanged(address indexed oldFeeManager, address indexed newFeeManager);

    /// @notice Emitted when the swapFeeModule of the factory is changed
    /// @param oldFeeModule The swapFeeModule before the swapFeeModule was changed
    /// @param newFeeModule The swapFeeModule after the swapFeeModule was changed
    event SwapFeeModuleChanged(address indexed oldFeeModule, address indexed newFeeModule);

    /// @notice Emitted when the unstakedFeeManager of the factory is changed
    /// @param oldFeeManager The unstakedFeeManager before the unstakedFeeManager was changed
    /// @param newFeeManager The unstakedFeeManager after the unstakedFeeManager was changed
    event UnstakedFeeManagerChanged(address indexed oldFeeManager, address indexed newFeeManager);

    /// @notice Emitted when the unstakedFeeModule of the factory is changed
    /// @param oldFeeModule The unstakedFeeModule before the unstakedFeeModule was changed
    /// @param newFeeModule The unstakedFeeModule after the unstakedFeeModule was changed
    event UnstakedFeeModuleChanged(address indexed oldFeeModule, address indexed newFeeModule);

    /// @notice Emitted when the defaultUnstakedFee of the factory is changed
    /// @param oldUnstakedFee The defaultUnstakedFee before the defaultUnstakedFee was changed
    /// @param newUnstakedFee The defaultUnstakedFee after the unstakedFeeModule was changed
    event DefaultUnstakedFeeChanged(uint24 indexed oldUnstakedFee, uint24 indexed newUnstakedFee);

    /// @notice Emitted when a pool is created
    /// @param token0 The first token of the pool by address sort order
    /// @param token1 The second token of the pool by address sort order
    /// @param tickSpacing The minimum number of ticks between initialized ticks
    /// @param pool The address of the created pool
    event PoolCreated(address indexed token0, address indexed token1, int24 indexed tickSpacing, address pool);

    /// @notice Emitted when a new tick spacing is enabled for pool creation via the factory
    /// @param tickSpacing The minimum number of ticks between initialized ticks for pools
    /// @param fee The default fee for a pool created with a given tickSpacing
    event TickSpacingEnabled(int24 indexed tickSpacing, uint24 indexed fee);



    /// @notice The address of the pool implementation contract used to deploy proxies / clones
    /// @return The address of the pool implementation contract
    function poolImplementation() external view returns (address);

    /// @notice Factory registry for valid pool / gauge / rewards factories
    /// @return The address of the factory registry

    function gaugeManager() external view returns (IGaugeManager);

    /// @notice Returns the current owner of the factory
    /// @dev Can be changed by the current owner via setOwner
    /// @return The address of the factory owner
    function owner() external view returns (address);

    /// @notice Returns the current swapFeeManager of the factory
    /// @dev Can be changed by the current swap fee manager via setSwapFeeManager
    /// @return The address of the factory swapFeeManager
    function swapFeeManager() external view returns (address);

    /// @notice Returns the current protocolFeeManager of the factory
    /// @dev Can be changed by the current protocol fee manager via setProtocolFeeManager
    /// @return The address of the factory protocolFeeManager
    function protocolFeeManager() external view returns (address);

    /// @notice Returns the current swapFeeModule of the factory
    /// @dev Can be changed by the current swap fee manager via setSwapFeeModule
    /// @return The address of the factory swapFeeModule
    function swapFeeModule() external view returns (address);

    /// @notice Returns the current unstakedFeeManager of the factory
    /// @dev Can be changed by the current unstaked fee manager via setUnstakedFeeManager
    /// @return The address of the factory unstakedFeeManager
    function unstakedFeeManager() external view returns (address);

    /// @notice Returns the current unstakedFeeModule of the factory
    /// @dev Can be changed by the current unstaked fee manager via setUnstakedFeeModule
    /// @return The address of the factory unstakedFeeModule
    function unstakedFeeModule() external view returns (address);


    function protocolFeeModule() external view returns (address);

    /// @notice Returns the current defaultUnstakedFee of the factory
    /// @dev Can be changed by the current unstaked fee manager via setDefaultUnstakedFee
    /// @return The default Unstaked Fee of the factory
    function defaultUnstakedFee() external view returns (uint24);


    function defaultProtocolFee() external view returns (uint24);

    /// @notice Returns a default fee for a tick spacing.
    /// @dev Use getFee for the most up to date fee for a given pool.
    /// A tick spacing can never be removed, so this value should be hard coded or cached in the calling context
    /// @param tickSpacing The enabled tick spacing. Returns 0 if not enabled
    /// @return fee The default fee for the given tick spacing
    function tickSpacingToFee(int24 tickSpacing) external view returns (uint24 fee);

    /// @notice Returns a list of enabled tick spacings. Used to iterate through pools created by the factory
    /// @dev Tick spacings cannot be removed. Tick spacings are not ordered
    /// @return List of enabled tick spacings
    function tickSpacings() external view returns (int24[] memory);

    /// @notice Returns the pool address for a given pair of tokens and a tick spacing, or address 0 if it does not exist
    /// @dev tokenA and tokenB may be passed in either token0/token1 or token1/token0 order
    /// @param tokenA The contract address of either token0 or token1
    /// @param tokenB The contract address of the other token
    /// @param tickSpacing The tick spacing of the pool
    /// @return pool The pool address
    function getPool(address tokenA, address tokenB, int24 tickSpacing) external view returns (address pool);

    /// @notice Return address of pool created by this factory given its `index`
    /// @param index Index of the pool
    /// @return The pool address in the given index
    function allPools(uint256 index) external view returns (address);

    /// @notice Returns the number of pools created from this factory
    /// @return Number of pools created from this factory
    function allPoolsLength() external view returns (uint256);

    /// @notice Used in VotingEscrow to determine if a contract is a valid pool of the factory
    /// @param pool The address of the pool to check
    /// @return Whether the pool is a valid pool of the factory
    function isPool(address pool) external view returns (bool);

    /// @notice Get swap & flash fee for a given pool. Accounts for default and dynamic fees
    /// @dev Swap & flash fee is denominated in pips. i.e. 1e-6
    /// @param pool The pool to get the swap & flash fee for
    /// @return The swap & flash fee for the given pool
    function getSwapFee(address pool) external view returns (uint24);

    /// @notice Get unstaked fee for a given pool. Accounts for default and dynamic fees
    /// @dev Unstaked fee is denominated in pips. i.e. 1e-6
    /// @param pool The pool to get the unstaked fee for
    /// @return The unstaked fee for the given pool
    function getUnstakedFee(address pool) external view returns (uint24);

    /// @notice Get protocol fee for a given pool. Accounts for default and dynamic fees
    /// @dev Protocol fee is denominated in pips. i.e. 1e-6
    /// @param pool The pool to get the protocol fee for
    /// @return The protocol fee for the given pool
    function getProtocolFee(address pool) external view returns (uint24);

    /// @notice Creates a pool for the given two tokens and fee
    /// @param tokenA One of the two tokens in the desired pool
    /// @param tokenB The other of the two tokens in the desired pool
    /// @param tickSpacing The desired tick spacing for the pool
    /// @param sqrtPriceX96 The initial sqrt price of the pool, as a Q64.96
    /// @dev tokenA and tokenB may be passed in either order: token0/token1 or token1/token0. The call will
    /// revert if the pool already exists, the tick spacing is invalid, or the token arguments are invalid
    /// @return pool The address of the newly created pool
    function createPool(address tokenA, address tokenB, int24 tickSpacing, uint160 sqrtPriceX96)
        external
        returns (address pool);

    /// @notice Updates the owner of the factory
    /// @dev Must be called by the current owner
    /// @param _owner The new owner of the factory
    function setOwner(address _owner) external;

    /// @notice Updates the swapFeeManager of the factory
    /// @dev Must be called by the current swap fee manager
    /// @param _swapFeeManager The new swapFeeManager of the factory
    function setSwapFeeManager(address _swapFeeManager) external;

    /// @notice Updates the swapFeeModule of the factory
    /// @dev Must be called by the current swap fee manager
    /// @param _swapFeeModule The new swapFeeModule of the factory
    function setSwapFeeModule(address _swapFeeModule) external;

    /// @notice Updates the unstakedFeeManager of the factory
    /// @dev Must be called by the current unstaked fee manager
    /// @param _unstakedFeeManager The new unstakedFeeManager of the factory
    function setUnstakedFeeManager(address _unstakedFeeManager) external;

    /// @notice Updates the unstakedFeeModule of the factory
    /// @dev Must be called by the current unstaked fee manager
    /// @param _unstakedFeeModule The new unstakedFeeModule of the factory
    function setUnstakedFeeModule(address _unstakedFeeModule) external;

    /// @notice Updates the protocolFeeManager of the factory
    /// @dev Must be called by the current protocol fee manager
    /// @param _protocolFeeManager The new protocolFeeManager of the factory
    function setProtocolFeeManager(address _protocolFeeManager) external;

    /// @notice Updates the protocolFeeModule of the factory
    /// @dev Must be called by the current protocol fee manager
    /// @param _protocolFeeModule The new protocolFeeModule of the factory
    function setProtocolFeeModule(address _protocolFeeModule) external;

    /// @notice Updates the defaultUnstakedFee of the factory
    /// @dev Must be called by the current unstaked fee manager
    /// @param _defaultUnstakedFee The new defaultUnstakedFee of the factory
    function setDefaultUnstakedFee(uint24 _defaultUnstakedFee) external;

    /// @notice Enables a certain tickSpacing
    /// @dev Tick spacings may never be removed once enabled
    /// @param tickSpacing The spacing between ticks to be enforced in the pool
    /// @param fee The default fee associated with a given tick spacing
    function enableTickSpacing(int24 tickSpacing, uint24 fee) external;
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

interface IRHYBR {
    // Enums
    enum RedeemType {
        TO_HYBR,        // 0: Convert to HYBR with penalty (70%-90% rate)
        TO_VEHYBR,      // 1: Convert to veHYBR 1:1 (max lock, new NFT)
        TO_GHYBR        // 2: Convert to gHYBR at current ratio
    }

    // Events
    event Transfer(address indexed from, address indexed to, uint256 value);
    event ConvertToHYBR(address indexed user, uint256 rHYBRAmount, uint256 HYBRReceived, uint256 penalty);
    event ConvertToGHYBR(address indexed user, uint256 rHYBRAmount, uint256 gHYBRReceived);
    event ConvertToVeHYBR(address indexed user, uint256 rHYBRAmount, uint256 tokenId, uint256 lockTime);
    event RateUpdated(uint256 oldRate, uint256 newRate);
    event MinterSet(address indexed oldMinter, address indexed newMinter);
    event GHYBRSet(address indexed gHYBR);
    event ConversionRateBoundsUpdated(uint256 oldMinRate, uint256 oldMaxRate, uint256 newMinRate, uint256 newMaxRate);
    event Converted(address indexed user, uint256 amount);

    // View functions
    function name() external pure returns (string memory);
    function symbol() external pure returns (string memory);
    function decimals() external pure returns (uint8);
    function totalSupply() external view returns (uint256);
    function balanceOf(address account) external view returns (uint256);
    
    // Conversion rate parameters
    function minConversionRate() external view returns (uint256);
    function maxConversionRate() external view returns (uint256);
    function RATE_PRECISION() external pure returns (uint256);
    function RATE_INCREASE_PER_HOUR() external pure returns (uint256);
    function RATE_DECREASE_PER_CONVERSION() external pure returns (uint256);
    function MIN_DECREASE_PER_CONVERSION() external pure returns (uint256);
    
    // Dynamic rate state
    function currentConversionRate() external view returns (uint256);
    function lastConversionTime() external view returns (uint256);
    function lastRateUpdateTime() external view returns (uint256);
    
    // External contracts
    function HYBR() external view returns (address);
    function gHYBR() external view returns (address);
    function votingEscrow() external view returns (address);
    function minter() external view returns (address);
    function gaugeManager() external view returns (address);

    // Core functions
    function updateConversionRate() external;
    function depostionEmissionsToken(uint256 _amount) external;
    function withdraw(uint256 amount) external;
    function redeem(uint256 amount, uint8 redeemType) external;
    function redeemFor(uint256 amount, uint8 redeemType, address recipient) external;
    function mint(address to, uint256 amount) external;
    
    // Transfer functions
    function transfer(address to, uint256 amount) external returns (bool);
    function transferFrom(address from, address to, uint256 amount) external returns (bool);
    function approve(address spender, uint256 amount) external pure returns (bool);
    function allowance(address owner, address spender) external pure returns (uint256);
    
    // Admin functions
    function setMinter(address _minter) external;
    function setGHYBR(address _gHYBR) external;
    function setConversionRateBounds(uint256 _minRate, uint256 _maxRate) external;
    function emergencyWithdraw(address token, uint256 amount) external;
    
    // Whitelist management
    function addExempt(address account) external;
    function removeExempt(address account) external;
    function addExemptTo(address account) external;
    function removeExemptTo(address account) external;
    function setGaugeManager(address _gaugeManager) external;
    function isExempt(address account) external view returns (bool);
    function isExemptTo(address account) external view returns (bool);

    // gHYBR interface functions (for compatibility)
    function deposit(uint256 amount, address recipient) external;
    function getPenaltyReward(uint256 amount) external;
    function rebase() external;
}
// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity >=0.4.0;

/// @title FixedPoint128
/// @notice A library for handling binary fixed point numbers, see https://en.wikipedia.org/wiki/Q_(number_format)
library FixedPoint128 {
    uint256 internal constant Q128 = 0x100000000000000000000000000000000;
}

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity >=0.5.0;

import "./pool/ICLPoolConstants.sol";
import "./pool/ICLPoolState.sol";
import "./pool/ICLPoolDerivedState.sol";
import "./pool/ICLPoolActions.sol";
import "./pool/ICLPoolOwnerActions.sol";
import "./pool/ICLPoolEvents.sol";

/// @title The interface for a CL Pool
/// @notice A CL pool facilitates swapping and automated market making between any two assets that strictly conform
/// to the ERC20 specification
/// @dev The pool interface is broken up into many smaller pieces
interface ICLPool is
    ICLPoolConstants,
    ICLPoolState,
    ICLPoolDerivedState,
    ICLPoolActions,
    ICLPoolEvents,
    ICLPoolOwnerActions
{}

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity >=0.7.5;
pragma abicoder v2;

import "@openzeppelin/contracts/token/ERC721/IERC721Metadata.sol";
import "@openzeppelin/contracts/token/ERC721/IERC721Enumerable.sol";

import "./IERC721Permit.sol";
import "./IERC4906.sol";
import "./IPeripheryPayments.sol";
import "./IPeripheryImmutableState.sol";
import "../libraries/PoolAddress.sol";

/// @title Non-fungible token for positions
/// @notice Wraps CL positions in a non-fungible token interface which allows for them to be transferred
/// and authorized.
interface INonfungiblePositionManager is
    IPeripheryPayments,
    IPeripheryImmutableState,
    IERC721Metadata,
    IERC721Enumerable,
    IERC721Permit,
    IERC4906
{
    /// @notice Emitted when liquidity is increased for a position NFT
    /// @dev Also emitted when a token is minted
    /// @param tokenId The ID of the token for which liquidity was increased
    /// @param liquidity The amount by which liquidity for the NFT position was increased
    /// @param amount0 The amount of token0 that was paid for the increase in liquidity
    /// @param amount1 The amount of token1 that was paid for the increase in liquidity
    event IncreaseLiquidity(uint256 indexed tokenId, uint128 liquidity, uint256 amount0, uint256 amount1);
    /// @notice Emitted when liquidity is decreased for a position NFT
    /// @param tokenId The ID of the token for which liquidity was decreased
    /// @param liquidity The amount by which liquidity for the NFT position was decreased
    /// @param amount0 The amount of token0 that was accounted for the decrease in liquidity
    /// @param amount1 The amount of token1 that was accounted for the decrease in liquidity
    event DecreaseLiquidity(uint256 indexed tokenId, uint128 liquidity, uint256 amount0, uint256 amount1);
    /// @notice Emitted when tokens are collected for a position NFT
    /// @dev The amounts reported may not be exactly equivalent to the amounts transferred, due to rounding behavior
    /// @param tokenId The ID of the token for which underlying tokens were collected
    /// @param recipient The address of the account that received the collected tokens
    /// @param amount0 The amount of token0 owed to the position that was collected
    /// @param amount1 The amount of token1 owed to the position that was collected
    event Collect(uint256 indexed tokenId, address recipient, uint256 amount0, uint256 amount1);
    /// @notice Emitted when a new Token Descriptor is set
    /// @param tokenDescriptor Address of the new Token Descriptor
    event TokenDescriptorChanged(address indexed tokenDescriptor);
    /// @notice Emitted when a new Owner is set
    /// @param owner Address of the new Owner
    event TransferOwnership(address indexed owner);

    /// @notice Returns the position information associated with a given token ID.
    /// @dev Throws if the token ID is not valid.
    /// @param tokenId The ID of the token that represents the position
    /// @return nonce The nonce for permits
    /// @return operator The address that is approved for spending
    /// @return token0 The address of the token0 for a specific pool
    /// @return token1 The address of the token1 for a specific pool
    /// @return tickSpacing The tick spacing associated with the pool
    /// @return tickLower The lower end of the tick range for the position
    /// @return tickUpper The higher end of the tick range for the position
    /// @return liquidity The liquidity of the position
    /// @return feeGrowthInside0LastX128 The fee growth of token0 as of the last action on the individual position
    /// @return feeGrowthInside1LastX128 The fee growth of token1 as of the last action on the individual position
    /// @return tokensOwed0 The uncollected amount of token0 owed to the position as of the last computation
    /// @return tokensOwed1 The uncollected amount of token1 owed to the position as of the last computation
    function positions(uint256 tokenId)
        external
        view
        returns (
            uint96 nonce,
            address operator,
            address token0,
            address token1,
            int24 tickSpacing,
            int24 tickLower,
            int24 tickUpper,
            uint128 liquidity,
            uint256 feeGrowthInside0LastX128,
            uint256 feeGrowthInside1LastX128,
            uint128 tokensOwed0,
            uint128 tokensOwed1
        );

    /// @notice Returns the address of the Token Descriptor, that handles generating token URIs for Positions
    function tokenDescriptor() external view returns (address);

    /// @notice Returns the address of the Owner, that is allowed to set a new TokenDescriptor
    function owner() external view returns (address);

    struct MintParams {
        address token0;
        address token1;
        int24 tickSpacing;
        int24 tickLower;
        int24 tickUpper;
        uint256 amount0Desired;
        uint256 amount1Desired;
        uint256 amount0Min;
        uint256 amount1Min;
        address recipient;
        uint256 deadline;
        uint160 sqrtPriceX96;
    }

    /// @notice Creates a new position wrapped in a NFT
    /// @dev Call this when the pool does exist and is initialized. Note that if the pool is created but not initialized
    /// a method does not exist, i.e. the pool is assumed to be initialized.
    /// @param params The params necessary to mint a position, encoded as `MintParams` in calldata
    /// @return tokenId The ID of the token that represents the minted position
    /// @return liquidity The amount of liquidity for this position
    /// @return amount0 The amount of token0
    /// @return amount1 The amount of token1
    function mint(MintParams calldata params)
        external
        payable
        returns (uint256 tokenId, uint128 liquidity, uint256 amount0, uint256 amount1);

    struct IncreaseLiquidityParams {
        uint256 tokenId;
        uint256 amount0Desired;
        uint256 amount1Desired;
        uint256 amount0Min;
        uint256 amount1Min;
        uint256 deadline;
    }

    /// @notice Increases the amount of liquidity in a position, with tokens paid by the `msg.sender`
    /// @param params tokenId The ID of the token for which liquidity is being increased,
    /// amount0Desired The desired amount of token0 to be spent,
    /// amount1Desired The desired amount of token1 to be spent,
    /// amount0Min The minimum amount of token0 to spend, which serves as a slippage check,
    /// amount1Min The minimum amount of token1 to spend, which serves as a slippage check,
    /// deadline The time by which the transaction must be included to effect the change
    /// @return liquidity The new liquidity amount as a result of the increase
    /// @return amount0 The amount of token0 to acheive resulting liquidity
    /// @return amount1 The amount of token1 to acheive resulting liquidity
    function increaseLiquidity(IncreaseLiquidityParams calldata params)
        external
        payable
        returns (uint128 liquidity, uint256 amount0, uint256 amount1);

    struct DecreaseLiquidityParams {
        uint256 tokenId;
        uint128 liquidity;
        uint256 amount0Min;
        uint256 amount1Min;
        uint256 deadline;
    }

    /// @notice Decreases the amount of liquidity in a position and accounts it to the position
    /// @param params tokenId The ID of the token for which liquidity is being decreased,
    /// amount The amount by which liquidity will be decreased,
    /// amount0Min The minimum amount of token0 that should be accounted for the burned liquidity,
    /// amount1Min The minimum amount of token1 that should be accounted for the burned liquidity,
    /// deadline The time by which the transaction must be included to effect the change
    /// @return amount0 The amount of token0 accounted to the position's tokens owed
    /// @return amount1 The amount of token1 accounted to the position's tokens owed
    /// @dev The use of this function can cause a loss to users of the NonfungiblePositionManager
    /// @dev for tokens that have very high decimals.
    /// @dev The amount of tokens necessary for the loss is: 3.4028237e+38.
    /// @dev This is equivalent to 1e20 value with 18 decimals.
    function decreaseLiquidity(DecreaseLiquidityParams calldata params)
        external
        payable
        returns (uint256 amount0, uint256 amount1);

    struct CollectParams {
        uint256 tokenId;
        address recipient;
        uint128 amount0Max;
        uint128 amount1Max;
    }

    /// @notice Collects up to a maximum amount of fees owed to a specific position to the recipient
    /// @notice Used to update staked positions before deposit and withdraw
    /// @param params tokenId The ID of the NFT for which tokens are being collected,
    /// recipient The account that should receive the tokens,
    /// amount0Max The maximum amount of token0 to collect,
    /// amount1Max The maximum amount of token1 to collect
    /// @return amount0 The amount of fees collected in token0
    /// @return amount1 The amount of fees collected in token1
    function collect(CollectParams calldata params) external payable returns (uint256 amount0, uint256 amount1);

    /// @notice Burns a token ID, which deletes it from the NFT contract. The token must have 0 liquidity and all tokens
    /// must be collected first.
    /// @param tokenId The ID of the token that is being burned
    function burn(uint256 tokenId) external payable;

    /// @notice Sets a new Token Descriptor
    /// @param _tokenDescriptor Address of the new Token Descriptor to be chosen
    function setTokenDescriptor(address _tokenDescriptor) external;

    /// @notice Sets a new Owner address
    /// @param _owner Address of the new Owner to be chosen
    function setOwner(address _owner) external;
}

// SPDX-License-Identifier: MIT
pragma solidity >=0.4.0 <0.8.0;

/// @title Contains 512-bit math functions
/// @notice Facilitates multiplication and division that can have overflow of an intermediate value without any loss of precision
/// @dev Handles "phantom overflow" i.e., allows multiplication and division where an intermediate value overflows 256 bits
library FullMath {
    /// @notice Calculates floor(a×b÷denominator) with full precision. Throws if result overflows a uint256 or denominator == 0
    /// @param a The multiplicand
    /// @param b The multiplier
    /// @param denominator The divisor
    /// @return result The 256-bit result
    /// @dev Credit to Remco Bloemen under MIT license https://xn--2-umb.com/21/muldiv
    function mulDiv(uint256 a, uint256 b, uint256 denominator) internal pure returns (uint256 result) {
        // 512-bit multiply [prod1 prod0] = a * b
        // Compute the product mod 2**256 and mod 2**256 - 1
        // then use the Chinese Remainder Theorem to reconstruct
        // the 512 bit result. The result is stored in two 256
        // variables such that product = prod1 * 2**256 + prod0
        uint256 prod0; // Least significant 256 bits of the product
        uint256 prod1; // Most significant 256 bits of the product
        assembly {
            let mm := mulmod(a, b, not(0))
            prod0 := mul(a, b)
            prod1 := sub(sub(mm, prod0), lt(mm, prod0))
        }

        // Handle non-overflow cases, 256 by 256 division
        if (prod1 == 0) {
            require(denominator > 0);
            assembly {
                result := div(prod0, denominator)
            }
            return result;
        }

        // Make sure the result is less than 2**256.
        // Also prevents denominator == 0
        require(denominator > prod1);

        ///////////////////////////////////////////////
        // 512 by 256 division.
        ///////////////////////////////////////////////

        // Make division exact by subtracting the remainder from [prod1 prod0]
        // Compute remainder using mulmod
        uint256 remainder;
        assembly {
            remainder := mulmod(a, b, denominator)
        }
        // Subtract 256 bit number from 512 bit number
        assembly {
            prod1 := sub(prod1, gt(remainder, prod0))
            prod0 := sub(prod0, remainder)
        }

        // Factor powers of two out of denominator
        // Compute largest power of two divisor of denominator.
        // Always >= 1.
        uint256 twos = -denominator & denominator;
        // Divide denominator by power of two
        assembly {
            denominator := div(denominator, twos)
        }

        // Divide [prod1 prod0] by the factors of two
        assembly {
            prod0 := div(prod0, twos)
        }
        // Shift in bits from prod1 into prod0. For this we need
        // to flip `twos` such that it is 2**256 / twos.
        // If twos is zero, then it becomes one
        assembly {
            twos := add(div(sub(0, twos), twos), 1)
        }
        prod0 |= prod1 * twos;

        // Invert denominator mod 2**256
        // Now that denominator is an odd number, it has an inverse
        // modulo 2**256 such that denominator * inv = 1 mod 2**256.
        // Compute the inverse by starting with a seed that is correct
        // correct for four bits. That is, denominator * inv = 1 mod 2**4
        uint256 inv = (3 * denominator) ^ 2;
        // Now use Newton-Raphson iteration to improve the precision.
        // Thanks to Hensel's lifting lemma, this also works in modular
        // arithmetic, doubling the correct bits in each step.
        inv *= 2 - denominator * inv; // inverse mod 2**8
        inv *= 2 - denominator * inv; // inverse mod 2**16
        inv *= 2 - denominator * inv; // inverse mod 2**32
        inv *= 2 - denominator * inv; // inverse mod 2**64
        inv *= 2 - denominator * inv; // inverse mod 2**128
        inv *= 2 - denominator * inv; // inverse mod 2**256

        // Because the division is now exact we can divide by multiplying
        // with the modular inverse of denominator. This will give us the
        // correct result modulo 2**256. Since the precoditions guarantee
        // that the outcome is less than 2**256, this is the final result.
        // We don't need to compute the high bits of the result and prod1
        // is no longer required.
        result = prod0 * inv;
        return result;
    }

    /// @notice Calculates ceil(a×b÷denominator) with full precision. Throws if result overflows a uint256 or denominator == 0
    /// @param a The multiplicand
    /// @param b The multiplier
    /// @param denominator The divisor
    /// @return result The 256-bit result
    function mulDivRoundingUp(uint256 a, uint256 b, uint256 denominator) internal pure returns (uint256 result) {
        result = mulDiv(a, b, denominator);
        if (mulmod(a, b, denominator) > 0) {
            require(result < type(uint256).max);
            result++;
        }
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

import "./IGaugeManager.sol";

interface IGaugeFactoryCL {
    function createGauge(address _rewardToken,address _ve,address _token,address _distribution, address _internal_bribe, address _external_bribe, bool _isPair, address nfpm) external returns (address) ;
    function gauges(uint256 i) external view returns(address);
    function length() external view returns(uint);
}

END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";
import "./BaseDeployScript.sol";

import {GaugeFactory} from "../contracts/factories/GaugeFactory.sol";
import {GaugeFactoryCL} from "../contracts/CLGauge/GaugeFactoryCL.sol";

import "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";

contract Deploy3a1_GaugeFactories is BaseDeployScript {
    using stdJson for string;
    
    function run() external {
        uint256 deployerKey = vm.envUint("PRIVATE_KEY");
        address deployer = vm.rememberKey(deployerKey);
        
        // Load previous deployments
        string memory infraPath = getInputPath("Deploy1_Infrastructure");
        string memory tokenPath = getInputPath("Deploy2_TokenSystem");
        string memory infraJson = vm.readFile(infraPath);
        string memory tokenJson = vm.readFile(tokenPath);
        
        address proxyAdmin = abi.decode(vm.parseJson(infraJson, ".ProxyAdmin"), (address));
        address permissionsRegistry = abi.decode(vm.parseJson(infraJson, ".PermissionsRegistry"), (address));
        address rHYBR = abi.decode(vm.parseJson(tokenJson, ".RewardHYBR"), (address));
        
        console.log("=== Deploy Gauge Factories ===");
        console.log("Deployer:", deployer);
        console.log("Using PermissionsRegistry:", permissionsRegistry);
        console.log("Using rHYBR:", rHYBR);
        
        vm.startBroadcast(deployer);
        
        // 1. Deploy GaugeFactory  
        console.log("Deploying GaugeFactory...");
        GaugeFactory gaugeFactoryImpl = new GaugeFactory();
        TransparentUpgradeableProxy gaugeFactoryProxy = new TransparentUpgradeableProxy(
            address(gaugeFactoryImpl),
            proxyAdmin,
            abi.encodeWithSelector(
                GaugeFactory.initialize.selector,
                permissionsRegistry
            )
        );
        GaugeFactory gaugeFactory = GaugeFactory(address(gaugeFactoryProxy));
        console.log("GaugeFactory:", address(gaugeFactory));
        
        gaugeFactory.setRHYBR(rHYBR);
        
        // 2. Deploy GaugeFactoryCL
        console.log("Deploying GaugeFactoryCL...");
        GaugeFactoryCL gaugeFactoryCLImpl = new GaugeFactoryCL();
        TransparentUpgradeableProxy gaugeFactoryCLProxy = new TransparentUpgradeableProxy(
            address(gaugeFactoryCLImpl),
            proxyAdmin,
            abi.encodeWithSelector(
                GaugeFactoryCL.initialize.selector,
                permissionsRegistry
            )
        );
        GaugeFactoryCL gaugeFactoryCL = GaugeFactoryCL(address(gaugeFactoryCLProxy));
        console.log("GaugeFactoryCL:", address(gaugeFactoryCL));
        
        gaugeFactoryCL.setRHYBR(rHYBR);
        
        vm.stopBroadcast();
        
        // Save to JSON
        string memory path = getOutputPath("Deploy3a1_GaugeFactories");
        
        string memory json = "";
        json = vm.serializeAddress("factories", "GaugeFactory", address(gaugeFactory));
        json = vm.serializeAddress("factories", "GaugeFactoryCL", address(gaugeFactoryCL));
        
        vm.writeJson(json, path);
        console.log("Addresses saved to:", path);
    }
}
