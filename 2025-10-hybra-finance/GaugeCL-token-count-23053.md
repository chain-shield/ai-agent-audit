
## *MAIN TARGET CONTRACT* TO REVIEW

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



END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
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
pragma solidity =0.7.6;
pragma abicoder v2;

import {IVotingEscrow} from "contracts/core/interfaces/IVotingEscrow.sol";
import {IFactoryRegistry} from "contracts/core/interfaces/IFactoryRegistry.sol";

interface IVoter {
    function ve() external view returns (IVotingEscrow);

    function vote(uint256 _tokenId, address[] calldata _poolVote, uint256[] calldata _weights) external;

    function gauges(address _pool) external view returns (address);

    function gaugeToFees(address _gauge) external view returns (address);

    function gaugeToBribes(address _gauge) external view returns (address);

    function createGauge(address _poolFactory, address _pool) external returns (address);

    function distribute(address gauge) external;

    function factoryRegistry() external view returns (IFactoryRegistry);

    /// @dev Utility to distribute to gauges of pools in array.
    /// @param _gauges Array of gauges to distribute to.
    function distribute(address[] memory _gauges) external;

    function isAlive(address _gauge) external view returns (bool);

    function killGauge(address _gauge) external;

    function emergencyCouncil() external view returns (address);

    /// @notice Claim emissions from gauges.
    /// @param _gauges Array of gauges to collect emissions from.
    function claimRewards(address[] memory _gauges) external;

    /// @notice Claim fees for a given NFT.
    /// @dev Utility to help batch fee claims.
    /// @param _fees    Array of FeesVotingReward contracts to collect from.
    /// @param _tokens  Array of tokens that are used as fees.
    /// @param _tokenId Id of veNFT that you wish to claim fees for.
    function claimFees(address[] memory _fees, address[][] memory _tokens, uint256 _tokenId) external;
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
pragma solidity >=0.5.0 <0.8.0;

import "./FullMath.sol";
import "./FixedPoint128.sol";
import "./LiquidityMath.sol";

/// @title Position
/// @notice Positions represent an owner address' liquidity between a lower and upper tick boundary
/// @dev Positions store additional state for tracking fees owed to the position
library Position {
    // info stored for each user's position
    struct Info {
        // the amount of liquidity owned by this position
        uint128 liquidity;
        // fee growth per unit of liquidity as of the last update to liquidity or fees owed
        uint256 feeGrowthInside0LastX128;
        uint256 feeGrowthInside1LastX128;
        // the fees owed to the position owner in token0/token1
        uint128 tokensOwed0;
        uint128 tokensOwed1;
    }

    /// @notice Returns the Info struct of a position, given an owner and position boundaries
    /// @param self The mapping containing all user positions
    /// @param owner The address of the position owner
    /// @param tickLower The lower tick boundary of the position
    /// @param tickUpper The upper tick boundary of the position
    /// @return position The position info struct of the given owners' position
    function get(mapping(bytes32 => Info) storage self, address owner, int24 tickLower, int24 tickUpper)
        internal
        view
        returns (Position.Info storage position)
    {
        position = self[keccak256(abi.encodePacked(owner, tickLower, tickUpper))];
    }

    /// @notice Credits accumulated fees to a user's position
    /// @param self The individual position to update
    /// @param liquidityDelta The change in pool liquidity as a result of the position update
    /// @param feeGrowthInside0X128 The all-time fee growth in token0, per unit of liquidity, inside the position's tick boundaries
    /// @param feeGrowthInside1X128 The all-time fee growth in token1, per unit of liquidity, inside the position's tick boundaries
    /// @param staked Signifies if the position is staked in the gauge or not
    function update(
        Info storage self,
        int128 liquidityDelta,
        uint256 feeGrowthInside0X128,
        uint256 feeGrowthInside1X128,
        bool staked
    ) internal {
        Info memory _self = self;

        uint128 liquidityNext;
        if (liquidityDelta == 0) {
            require(_self.liquidity > 0, "NP"); // disallow pokes for 0 liquidity positions
            liquidityNext = _self.liquidity;
        } else {
            liquidityNext = LiquidityMath.addDelta(_self.liquidity, liquidityDelta);
        }

        uint128 tokensOwed0;
        uint128 tokensOwed1;
        if (!staked) {
            // calculate accumulated fees
            tokensOwed0 = uint128(
                FullMath.mulDiv(
                    feeGrowthInside0X128 - _self.feeGrowthInside0LastX128, _self.liquidity, FixedPoint128.Q128
                )
            );
            tokensOwed1 = uint128(
                FullMath.mulDiv(
                    feeGrowthInside1X128 - _self.feeGrowthInside1LastX128, _self.liquidity, FixedPoint128.Q128
                )
            );
        }

        // update the position
        if (liquidityDelta != 0) self.liquidity = liquidityNext;
        self.feeGrowthInside0LastX128 = feeGrowthInside0X128;
        self.feeGrowthInside1LastX128 = feeGrowthInside1X128;
        if (tokensOwed0 > 0 || tokensOwed1 > 0) {
            // overflow is acceptable, have to withdraw before you hit type(uint128).max fees
            self.tokensOwed0 += tokensOwed0;
            self.tokensOwed1 += tokensOwed1;
        }
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity >=0.5.0;

import "./INonfungiblePositionManager.sol";

/// @title Describes position NFT tokens via URI
interface INonfungibleTokenPositionDescriptor {
    /// @notice Produces the URI describing a particular token ID for a position manager
    /// @dev Note this URI may be a data: URI with the JSON contents directly inlined
    /// @param positionManager The position manager for which to describe the token
    /// @param tokenId The ID of the token for which to produce a description, which may not be valid
    /// @return The URI of the ERC721-compliant metadata
    function tokenURI(INonfungiblePositionManager positionManager, uint256 tokenId)
        external
        view
        returns (string memory);
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

interface ITokenHandler {
    function isWhitelisted(address token) external view returns (bool);
    function isWhitelistedNFT(uint256 token) external view returns (bool);
    function isConnector(address token) external view returns (bool);

    function whitelistToken(address _token) external;
    function blacklistToken(address _token) external;

    function whiteListed(uint256 index) external returns (address);
    function connectors(uint256 index) external returns (address);

    function whiteListedTokensLength() external returns (uint256);
    function connectorTokensLength() external returns (uint256);

    function whiteListedTokens() external view returns(address[] memory tokens);
    function connectorTokens() external view returns(address[] memory tokens);
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

library PositionKey {
    /// @dev Returns the key of the position in the core library
    function compute(address owner, int24 tickLower, int24 tickUpper) internal pure returns (bytes32) {
        return keccak256(abi.encodePacked(owner, tickLower, tickUpper));
    }
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.13;

import "./interfaces/IHybra.sol";

contract HYBR is IHybra {

    string public constant name = "HYBR";
    string public constant symbol = "HYBR";
    uint8 public constant decimals = 18;
    uint public totalSupply = 0;

    mapping(address => uint) public balanceOf;
    mapping(address => mapping(address => uint)) public allowance;

    bool public initialMinted;
    address public minter;

    event Transfer(address indexed from, address indexed to, uint value);
    event Approval(address indexed owner, address indexed spender, uint value);

    constructor() {
        minter = msg.sender;
        _mint(msg.sender, 0);
    }

    // No checks as its meant to be once off to set minting rights to BaseV1 Minter
    function setMinter(address _minter) external {
        require(msg.sender == minter);
        minter = _minter;
    }

    // Initial mint: total 50M    
    function initialMint(address _recipient) external {
        require(msg.sender == minter && !initialMinted);
        initialMinted = true;
        _mint(_recipient, 500 * 1e6 * 1e18);
    }

    function approve(address _spender, uint _value) external returns (bool) {
        allowance[msg.sender][_spender] = _value;
        emit Approval(msg.sender, _spender, _value);
        return true;
    }

    function _mint(address _to, uint _amount) internal returns (bool) {
        totalSupply += _amount;
        unchecked {
            balanceOf[_to] += _amount;
        }
        emit Transfer(address(0x0), _to, _amount);
        return true;
    }

    function _transfer(address _from, address _to, uint _value) internal returns (bool) {
        balanceOf[_from] -= _value;
        unchecked {
            balanceOf[_to] += _value;
        }
        emit Transfer(_from, _to, _value);
        return true;
    }

    function transfer(address _to, uint _value) external returns (bool) {
        return _transfer(msg.sender, _to, _value);
    }

    function transferFrom(address _from, address _to, uint _value) external returns (bool) {
        uint allowed_from = allowance[_from][msg.sender];
        if (allowed_from != type(uint).max) {
            allowance[_from][msg.sender] -= _value;
        }
        return _transfer(_from, _to, _value);
    }

    function mint(address account, uint amount) external returns (bool) {
        require(msg.sender == minter, 'not allowed');
        _mint(account, amount);
        return true;
    }

    function burn(uint256 value) external returns (bool) {
        _burn(msg.sender, value);
        return true;
    }

    function burnFrom(address _from, uint _value) external returns (bool) {
        uint allowed_from = allowance[_from][msg.sender];
        if (allowed_from != type(uint).max) {
            allowance[_from][msg.sender] -= _value;
        }
        _burn(_from, _value);
        return true;
    }

    function _burn(address _from, uint _amount) internal returns (bool) {
        totalSupply -= _amount;
        balanceOf[_from] -= _amount;
        emit Transfer(_from, address(0x0), _amount);
        return true;
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity =0.7.6;

import "../interfaces/IPeripheryImmutableState.sol";

/// @title Immutable state
/// @notice Immutable state used by periphery contracts
abstract contract PeripheryImmutableState is IPeripheryImmutableState {
    /// @inheritdoc IPeripheryImmutableState
    address public immutable override factory;
    /// @inheritdoc IPeripheryImmutableState
    address public immutable override WETH9;

    constructor(address _factory, address _WETH9) {
        factory = _factory;
        WETH9 = _WETH9;
    }
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

import "./interfaces/IMinter.sol";
import "./interfaces/IVoter.sol";
import "./interfaces/IGaugeManager.sol";
import "./interfaces/IVotingEscrow.sol";
import "./interfaces/ITokenHandler.sol";
import {HybraTimeLibrary} from "./libraries/HybraTimeLibrary.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import {Math} from "@openzeppelin/contracts/utils/math/Math.sol";


contract Bribe is ReentrancyGuard {
    using SafeERC20 for IERC20;
    uint256 public WEEK; 

    /* ========== STATE VARIABLES ========== */

    struct Checkpoint {
        uint256 timestamp;
        uint256 balanceOf;
    }

    struct SupplyCheckpoint {
        uint256 timestamp;
        uint256 supply;
    }

    mapping(address => mapping(uint256 => uint256)) public tokenRewardsPerEpoch; // token -> startTimestamp -> rewardBalance
    address public voter;
    address public gaugeManager;
    address public immutable bribeFactory;
    address public minter;
    address public immutable ve;
    address public owner;
    ITokenHandler public tokenHandler;

    string public TYPE;

    uint256 public totalSupply;
    mapping(uint256 => uint256) public balanceOf;

    mapping(address => mapping(uint256 => uint256)) public lastEarn;

    mapping(uint256 => mapping(uint256 => Checkpoint)) public checkpoints;
    mapping(uint256 => uint256) public numCheckpoints;

    mapping(uint256 => SupplyCheckpoint) public supplyCheckpoints;
    uint256 public supplyNumCheckpoints;

    mapping(address => bool) internal isBribeToken;
    address[] public bribeTokens;


    /* ========== CONSTRUCTOR ========== */

    constructor(address _owner,address _voter,address _gaugeManager, address _bribeFactory, address _tokenHandler, address _token0, address _token1, string memory _type)  {
        require(_bribeFactory != address(0) && _voter != address(0) && _gaugeManager != address(0) && _owner != address(0), "ZA");
        WEEK = HybraTimeLibrary.WEEK;
        voter = _voter;
        gaugeManager = _gaugeManager;
        bribeFactory = _bribeFactory;
        tokenHandler = ITokenHandler(_tokenHandler);
        ve = IVoter(_voter)._ve();
        minter = IGaugeManager(_gaugeManager).minter();
        require(minter != address(0), "ZA");
        owner = _owner;
        TYPE = _type;

        bribeTokens.push(_token0);
        bribeTokens.push(_token1);
        isBribeToken[_token0] = true;
        isBribeToken[_token1] = true;
    }

    function getEpochStart() public view returns(uint256){
        return IMinter(minter).active_period();
    }

    /// @notice get next epoch (where bribes are saved)
    function getNextEpochStart() public view returns(uint256){
        return HybraTimeLibrary.epochNext(block.timestamp);
    }

    /* ========== VIEWS ========== */

    /// @notice get the length of the reward tokens
    function rewardsListLength() external view returns(uint256) {
        return bribeTokens.length;
    }

    /// @notice Read earned amount given a tokenID and _rewardToken
    function earned(uint256 tokenId, address _rewardToken) public view returns(uint256){
        if (numCheckpoints[tokenId] == 0) {
            return 0;
        }
        
        uint256 reward = 0;
        uint256 _supply = 1;
        uint256 _currTs = HybraTimeLibrary.epochStart(lastEarn[_rewardToken][tokenId]); // take epoch last claimed in as starting point
        uint256 _index = getPriorBalanceIndex(tokenId, _currTs);
        Checkpoint memory cp0 = checkpoints[tokenId][_index];
        
        
        // accounts for case where lastEarn is before first checkpoint
        _currTs = Math.max(_currTs, HybraTimeLibrary.epochStart(cp0.timestamp));

        // get epochs between current epoch and first checkpoint in same epoch as last claim
        uint256 numEpochs = (HybraTimeLibrary.epochStart(block.timestamp) - _currTs) / WEEK;

        if (numEpochs > 0) {
            for (uint256 i = 0; i < numEpochs; i++) {
                // get index of last checkpoint in this epoch
                _index = getPriorBalanceIndex(tokenId, _currTs + WEEK - 1);
                // get checkpoint in this epoch
                cp0 = checkpoints[tokenId][_index];
                // get supply of last checkpoint in this epoch
                _supply = Math.max(supplyCheckpoints[getPriorSupplyIndex(_currTs + WEEK - 1)].supply, 1);
                reward += (cp0.balanceOf * tokenRewardsPerEpoch[_rewardToken][_currTs]) / _supply;
                _currTs += WEEK;
            }
        } 
        return reward;  
    }


    function getPriorBalanceIndex(uint256 tokenId, uint256 timestamp) public view returns (uint256) {
        uint256 nCheckpoints = numCheckpoints[tokenId];
        if (nCheckpoints == 0) {
            return 0;
        }

        // First check most recent balance
        if (checkpoints[tokenId][nCheckpoints - 1].timestamp <= timestamp) {
            return (nCheckpoints - 1);
        }

        // Next check implicit zero balance
        if (checkpoints[tokenId][0].timestamp > timestamp) {
            return 0;
        }

        uint256 lower = 0;
        uint256 upper = nCheckpoints - 1;
        while (upper > lower) {
            uint256 center = upper - (upper - lower) / 2; // ceil, avoiding overflow
            Checkpoint memory cp = checkpoints[tokenId][center];
            if (cp.timestamp == timestamp) {
                return center;
            } else if (cp.timestamp < timestamp) {
                lower = center;
            } else {
                upper = center - 1;
            }
        }
        return lower;
    }

    function getPriorSupplyIndex(uint256 timestamp) public view returns (uint256) {
        uint256 nCheckpoints = supplyNumCheckpoints;
        if (nCheckpoints == 0) {
            return 0;
        }

        // First check most recent balance
        if (supplyCheckpoints[nCheckpoints - 1].timestamp <= timestamp) {
            return (nCheckpoints - 1);
        }

        // Next check implicit zero balance
        if (supplyCheckpoints[0].timestamp > timestamp) {
            return 0;
        }

        uint256 lower = 0;
        uint256 upper = nCheckpoints - 1;
        while (upper > lower) {
            uint256 center = upper - (upper - lower) / 2; // ceil, avoiding overflow
            SupplyCheckpoint memory cp = supplyCheckpoints[center];
            if (cp.timestamp == timestamp) {
                return center;
            } else if (cp.timestamp < timestamp) {
                lower = center;
            } else {
                upper = center - 1;
            }
        }
        return lower;
    }

    function isRewardToken(address _rewardToken) external view returns (bool) {
        return _isRewardToken(_rewardToken);
    }

    function _isRewardToken(address _rewardToken) internal view returns (bool) {
        return isBribeToken[_rewardToken] || tokenHandler.isConnector(_rewardToken);
    }
 
    /* ========== MUTATIVE FUNCTIONS ========== */

    /// @notice User votes deposit
    /// @dev    called on voter.vote() or voter.poke()
    ///         we save into owner "address" and not "tokenID". 
    ///         Owner must reset before transferring token
    function deposit(uint256 amount, uint256 tokenId) external nonReentrant {
        require(amount > 0, "ZV");
        require(msg.sender == voter, "NA");
        totalSupply += amount;
        balanceOf[tokenId] += amount;

        _writeCheckpoint(tokenId, balanceOf[tokenId]);
        _writeSupplyCheckpoint();
        
        emit Staked(tokenId, amount);
    }

    function _writeCheckpoint(uint256 tokenId, uint256 balance) internal {
        uint256 _nCheckPoints = numCheckpoints[tokenId];
        uint256 _timestamp = block.timestamp;

        if (
            _nCheckPoints > 0 &&
            HybraTimeLibrary.epochStart(checkpoints[tokenId][_nCheckPoints - 1].timestamp) ==
            HybraTimeLibrary.epochStart(_timestamp)
        ) {
            checkpoints[tokenId][_nCheckPoints - 1] = Checkpoint(_timestamp, balance);
        } else {
            checkpoints[tokenId][_nCheckPoints] = Checkpoint(_timestamp, balance);
            numCheckpoints[tokenId] = _nCheckPoints + 1;
        }
    }


    function _writeSupplyCheckpoint() internal {
        uint256 _nCheckPoints = supplyNumCheckpoints;
        uint256 _timestamp = block.timestamp;

        if (
            _nCheckPoints > 0 &&
            HybraTimeLibrary.epochStart(supplyCheckpoints[_nCheckPoints - 1].timestamp) ==
            HybraTimeLibrary.epochStart(_timestamp)
        ) {
            supplyCheckpoints[_nCheckPoints - 1] = SupplyCheckpoint(_timestamp, totalSupply);
        } else {
            supplyCheckpoints[_nCheckPoints] = SupplyCheckpoint(_timestamp, totalSupply);
            supplyNumCheckpoints = _nCheckPoints + 1;
        }
    }

    /// @notice User votes withdrawal 
    /// @dev    called on voter.reset()
    function withdraw(uint256 amount, uint256 tokenId) external nonReentrant {
        require(amount > 0, "ZV");
        require(msg.sender == voter, "NA");
        if (amount <= balanceOf[tokenId]) {
            totalSupply -= amount;
            balanceOf[tokenId] -= amount;

            _writeCheckpoint(tokenId, balanceOf[tokenId]);
            _writeSupplyCheckpoint();
            emit Withdrawn(tokenId, amount);
        }
    }

    /// @notice Claim the TOKENID rewards
    function getReward(uint256 tokenId, address[] memory tokens) external nonReentrant  {
        address _owner = IVotingEscrow(ve).ownerOf(tokenId);
        require(msg.sender == gaugeManager, "NA");
        uint256 _length = tokens.length;
        for (uint256 i = 0; i < _length; i++) {
            uint256 _reward = earned(tokenId, tokens[i]);
            lastEarn[tokens[i]][tokenId] = block.timestamp;
            if (_reward > 0) {
                IERC20(tokens[i]).safeTransfer(_owner, _reward);
            }
        }
    }

    /// @dev Rewards are saved into Current EPOCH mapping. 
    function notifyRewardAmount(address _rewardsToken, uint256 reward) external nonReentrant {
        require(_isRewardToken(_rewardsToken), "!VERIFIED");

        if(!isBribeToken[_rewardsToken]){
            isBribeToken[_rewardsToken] = true;
            bribeTokens.push(_rewardsToken);
        }

        IERC20(_rewardsToken).safeTransferFrom(msg.sender,address(this),reward);
        uint256 epochStart = HybraTimeLibrary.epochStart(block.timestamp);
        tokenRewardsPerEpoch[_rewardsToken][epochStart] += reward;
        emit RewardAdded(_rewardsToken, reward, epochStart);
    }

    /* ========== RESTRICTED FUNCTIONS ========== */

    /// @notice Recover some ERC20 from the contract and updated given bribe
    function recoverERC20AndUpdateData(address tokenAddress, uint256 tokenAmount) external onlyAllowed {
        require(tokenAmount <= IERC20(tokenAddress).balanceOf(address(this)), "TOO_MUCH");
        
        uint256 _startTimestamp = IMinter(minter).active_period() + WEEK;
        uint256 _lastReward = tokenRewardsPerEpoch[tokenAddress][_startTimestamp];
        tokenRewardsPerEpoch[tokenAddress][_startTimestamp] = _lastReward - tokenAmount;
        IERC20(tokenAddress).safeTransfer(owner, tokenAmount);
        emit Recovered(tokenAddress, tokenAmount);
    }

    /// @notice Recover some ERC20 from the contract.
    /// @dev    Be careful --> if called then getReward() at last epoch will fail because some reward are missing! 
    ///         Think about calling recoverERC20AndUpdateData()
    function emergencyRecoverERC20(address tokenAddress, uint256 tokenAmount) external onlyAllowed {
        require(tokenAmount <= IERC20(tokenAddress).balanceOf(address(this)), "TOO_MUCH");
        IERC20(tokenAddress).safeTransfer(owner, tokenAmount);
        emit Recovered(tokenAddress, tokenAmount);
    }

    /// @notice Set a new voter
    function setVoter(address _Voter) external onlyAllowed {
        require(_Voter != address(0), "ZA");
        voter = _Voter;
    }

        /// @notice Set a new gaugeManager
    function setGaugeManager(address _gaugeManager) external onlyAllowed {
        require(_gaugeManager != address(0));
        gaugeManager = _gaugeManager;
    }

    /// @notice Set a new minter
    function setMinter(address _minter) external onlyAllowed {
        require(_minter != address(0), "ZA");
        minter = _minter;
    }


    /// @notice Set a new Owner
    event SetOwner(address indexed _owner);
    function setOwner(address _owner) external onlyAllowed {
        require(_owner != address(0), "ZA");
        owner = _owner;
        emit SetOwner(_owner);
    }



    /* ========== MODIFIERS ========== */

    modifier onlyAllowed() {
        require( (msg.sender == owner || msg.sender == bribeFactory), "NA" );
        _;
    }


    /* ========== EVENTS ========== */

    event RewardAdded(address indexed rewardToken, uint256 reward, uint256 startTimestamp);
    event Staked(uint256 indexed tokenId, uint256 amount);
    event Withdrawn(uint256 indexed tokenId, uint256 amount);
    event RewardPaid(address indexed user,address indexed rewardsToken,uint256 reward);
    event Recovered(address indexed token, uint256 amount);
}

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity =0.7.6;
pragma abicoder v2;

import "contracts/core/interfaces/ICLPool.sol";
import "contracts/core/libraries/FixedPoint128.sol";
import "contracts/core/libraries/FullMath.sol";

import "./interfaces/INonfungiblePositionManager.sol";
import "./interfaces/INonfungibleTokenPositionDescriptor.sol";
import "./libraries/PositionKey.sol";
import "./libraries/PoolAddress.sol";
import "./base/LiquidityManagement.sol";
import "./base/PeripheryImmutableState.sol";
import "./base/Multicall.sol";
import "./base/ERC721Permit.sol";
import "./base/PeripheryValidation.sol";
import "./base/SelfPermit.sol";

/// @title NFT positions
/// @notice Wraps CL positions in the ERC721 non-fungible token interface
contract NonfungiblePositionManager is
    INonfungiblePositionManager,
    Multicall,
    ERC721Permit,
    PeripheryImmutableState,
    LiquidityManagement,
    PeripheryValidation,
    SelfPermit
{
    // details about the cl position
    struct Position {
        // the nonce for permits
        uint96 nonce;
        // the address that is approved for spending this token
        address operator;
        // the ID of the pool with which this token is connected
        uint80 poolId;
        // the tick range of the position
        int24 tickLower;
        int24 tickUpper;
        // the liquidity of the position
        uint128 liquidity;
        // the fee growth of the aggregate position as of the last action on the individual position
        uint256 feeGrowthInside0LastX128;
        uint256 feeGrowthInside1LastX128;
        // how many uncollected tokens are owed to the position, as of the last computation
        uint128 tokensOwed0;
        uint128 tokensOwed1;
    }
    /// @dev Revert String Annotations:
    /// NE - ERC721: approved query for nonexistent token
    /// PS - Price slippage check
    /// ID - Invalid token ID
    /// ZA - Zero Address
    /// NA - Not approved
    /// NC - Not cleared
    /// NO - Not Owner

    /// @dev IDs of pools assigned by this contract
    mapping(address => uint80) private _poolIds;

    /// @dev Pool keys by pool ID, to save on SSTOREs for position data
    mapping(uint80 => PoolAddress.PoolKey) private _poolIdToPoolKey;

    /// @dev The token ID position data
    mapping(uint256 => Position) private _positions;

    /// @dev The ID of the next token that will be minted. Skips 0
    uint176 private _nextId = 1;
    /// @dev The ID of the next pool that is used for the first time. Skips 0
    uint80 private _nextPoolId = 1;

    /// @inheritdoc INonfungiblePositionManager
    address public override owner;

    /// @inheritdoc INonfungiblePositionManager
    address public override tokenDescriptor;

    /// @dev Prevents calling a function from anyone except owner
    modifier onlyOwner() {
        require(msg.sender == owner);
        _;
    }

    constructor(address _factory, address _WETH9, address _tokenDescriptor, string memory name, string memory symbol)
        ERC721Permit(name, symbol, "1")
        PeripheryImmutableState(_factory, _WETH9)
    {
        owner = msg.sender;
        tokenDescriptor = _tokenDescriptor;
    }

    /// @inheritdoc INonfungiblePositionManager
    function positions(uint256 tokenId)
        external
        view
        override
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
        )
    {
        Position memory position = _positions[tokenId];
        require(position.poolId != 0, "ID");
        PoolAddress.PoolKey memory poolKey = _poolIdToPoolKey[position.poolId];
        return (
            position.nonce,
            position.operator,
            poolKey.token0,
            poolKey.token1,
            poolKey.tickSpacing,
            position.tickLower,
            position.tickUpper,
            position.liquidity,
            position.feeGrowthInside0LastX128,
            position.feeGrowthInside1LastX128,
            position.tokensOwed0,
            position.tokensOwed1
        );
    }

    /// @dev Caches a pool key
    function cachePoolKey(address pool, PoolAddress.PoolKey memory poolKey) private returns (uint80 poolId) {
        poolId = _poolIds[pool];
        if (poolId == 0) {
            _poolIds[pool] = (poolId = _nextPoolId++);
            _poolIdToPoolKey[poolId] = poolKey;
        }
    }

    /// @inheritdoc INonfungiblePositionManager
    function mint(MintParams calldata params)
        external
        payable
        override
        checkDeadline(params.deadline)
        returns (uint256 tokenId, uint128 liquidity, uint256 amount0, uint256 amount1)
    {
        if (params.sqrtPriceX96 != 0) {
            ICLFactory(factory).createPool({
                tokenA: params.token0,
                tokenB: params.token1,
                tickSpacing: params.tickSpacing,
                sqrtPriceX96: params.sqrtPriceX96
            });
        }
        PoolAddress.PoolKey memory poolKey =
            PoolAddress.PoolKey({token0: params.token0, token1: params.token1, tickSpacing: params.tickSpacing});

        ICLPool pool = ICLPool(PoolAddress.computeAddress(factory, poolKey));

        (liquidity, amount0, amount1) = addLiquidity(
            AddLiquidityParams({
                poolAddress: address(pool),
                poolKey: poolKey,
                recipient: address(this),
                tickLower: params.tickLower,
                tickUpper: params.tickUpper,
                amount0Desired: params.amount0Desired,
                amount1Desired: params.amount1Desired,
                amount0Min: params.amount0Min,
                amount1Min: params.amount1Min
            })
        );

        _mint(params.recipient, (tokenId = _nextId++));

        bytes32 positionKey = PositionKey.compute(address(this), params.tickLower, params.tickUpper);
        (, uint256 feeGrowthInside0LastX128, uint256 feeGrowthInside1LastX128,,) = pool.positions(positionKey);

        // idempotent set
        uint80 poolId = cachePoolKey(address(pool), poolKey);

        _positions[tokenId] = Position({
            nonce: 0,
            operator: address(0),
            poolId: poolId,
            tickLower: params.tickLower,
            tickUpper: params.tickUpper,
            liquidity: liquidity,
            feeGrowthInside0LastX128: feeGrowthInside0LastX128,
            feeGrowthInside1LastX128: feeGrowthInside1LastX128,
            tokensOwed0: 0,
            tokensOwed1: 0
        });

        refundETH();

        emit IncreaseLiquidity(tokenId, liquidity, amount0, amount1);
    }

    modifier isAuthorizedForToken(uint256 tokenId) {
        require(_isApprovedOrOwner(msg.sender, tokenId));
        _;
    }

    function tokenURI(uint256 tokenId) public view override(ERC721, IERC721Metadata) returns (string memory) {
        require(_exists(tokenId));
        return INonfungibleTokenPositionDescriptor(tokenDescriptor).tokenURI(this, tokenId);
    }

    // save bytecode by removing implementation of unused method
    function baseURI() public pure override returns (string memory) {}

    /// @inheritdoc INonfungiblePositionManager
    function increaseLiquidity(IncreaseLiquidityParams calldata params)
        external
        payable
        override
        checkDeadline(params.deadline)
        returns (uint128 liquidity, uint256 amount0, uint256 amount1)
    {
        Position storage position = _positions[params.tokenId];

        PoolAddress.PoolKey memory poolKey = _poolIdToPoolKey[position.poolId];

        ICLPool pool = ICLPool(PoolAddress.computeAddress(factory, poolKey));

        address gauge = pool.gauge();
        bool isStaked = ownerOf(params.tokenId) == gauge;
        if (isStaked) require(msg.sender == gauge, "NG");

        (liquidity, amount0, amount1) = addLiquidity(
            AddLiquidityParams({
                poolAddress: address(pool),
                poolKey: poolKey,
                tickLower: position.tickLower,
                tickUpper: position.tickUpper,
                amount0Desired: params.amount0Desired,
                amount1Desired: params.amount1Desired,
                amount0Min: params.amount0Min,
                amount1Min: params.amount1Min,
                recipient: isStaked ? gauge : address(this)
            })
        );

        bytes32 positionKey =
            PositionKey.compute(isStaked ? gauge : address(this), position.tickLower, position.tickUpper);

        // this is now updated to the current transaction
        (, uint256 feeGrowthInside0LastX128, uint256 feeGrowthInside1LastX128,,) = pool.positions(positionKey);

        if (!isStaked) {
            position.tokensOwed0 += uint128(
                FullMath.mulDiv(
                    feeGrowthInside0LastX128 - position.feeGrowthInside0LastX128, position.liquidity, FixedPoint128.Q128
                )
            );
            position.tokensOwed1 += uint128(
                FullMath.mulDiv(
                    feeGrowthInside1LastX128 - position.feeGrowthInside1LastX128, position.liquidity, FixedPoint128.Q128
                )
            );
        }

        position.feeGrowthInside0LastX128 = feeGrowthInside0LastX128;
        position.feeGrowthInside1LastX128 = feeGrowthInside1LastX128;
        position.liquidity += liquidity;

        refundETH();

        emit MetadataUpdate(params.tokenId);
        emit IncreaseLiquidity(params.tokenId, liquidity, amount0, amount1);
    }

    /// @inheritdoc INonfungiblePositionManager
    function decreaseLiquidity(DecreaseLiquidityParams calldata params)
        external
        payable
        override
        isAuthorizedForToken(params.tokenId)
        checkDeadline(params.deadline)
        returns (uint256 amount0, uint256 amount1)
    {
        require(params.liquidity > 0);
        Position storage position = _positions[params.tokenId];

        uint128 positionLiquidity = position.liquidity;
        require(positionLiquidity >= params.liquidity);

        PoolAddress.PoolKey memory poolKey = _poolIdToPoolKey[position.poolId];
        ICLPool pool = ICLPool(PoolAddress.computeAddress(factory, poolKey));

        address gauge = pool.gauge();
        bool isStaked = ownerOf(params.tokenId) == gauge;
        if (!isStaked) {
            (amount0, amount1) = pool.burn(position.tickLower, position.tickUpper, params.liquidity);
        } else {
            (amount0, amount1) = pool.burn(position.tickLower, position.tickUpper, params.liquidity, gauge);
        }

        require(amount0 >= params.amount0Min && amount1 >= params.amount1Min, "PS");

        bytes32 positionKey =
            PositionKey.compute(isStaked ? gauge : address(this), position.tickLower, position.tickUpper);
        // this is now updated to the current transaction
        (, uint256 feeGrowthInside0LastX128, uint256 feeGrowthInside1LastX128,,) = pool.positions(positionKey);

        /// @dev Casting to u128 and the sum of tokensOwed overflow can cause a loss to users.
        /// @dev This is more probable for tokens that have very high decimals.
        /// @dev The amount of tokens necessary for the loss is: 3.4028237e+38.
        position.tokensOwed0 += uint128(amount0);
        position.tokensOwed1 += uint128(amount1);

        if (!isStaked) {
            position.tokensOwed0 += uint128(
                FullMath.mulDiv(
                    feeGrowthInside0LastX128 - position.feeGrowthInside0LastX128, positionLiquidity, FixedPoint128.Q128
                )
            );
            position.tokensOwed1 += uint128(
                FullMath.mulDiv(
                    feeGrowthInside1LastX128 - position.feeGrowthInside1LastX128, positionLiquidity, FixedPoint128.Q128
                )
            );
        }

        position.feeGrowthInside0LastX128 = feeGrowthInside0LastX128;
        position.feeGrowthInside1LastX128 = feeGrowthInside1LastX128;
        // subtraction is safe because we checked positionLiquidity is gte params.liquidity
        position.liquidity = positionLiquidity - params.liquidity;

        emit MetadataUpdate(params.tokenId);
        emit DecreaseLiquidity(params.tokenId, params.liquidity, amount0, amount1);
    }

    /// @inheritdoc INonfungiblePositionManager
    function collect(CollectParams calldata params)
        external
        payable
        override
        isAuthorizedForToken(params.tokenId)
        returns (uint256 amount0, uint256 amount1)
    {
        require(params.amount0Max > 0 || params.amount1Max > 0);
        // allow collecting to the nft position manager address with address 0
        address recipient = params.recipient == address(0) ? address(this) : params.recipient;

        Position storage position = _positions[params.tokenId];

        PoolAddress.PoolKey memory poolKey = _poolIdToPoolKey[position.poolId];

        ICLPool pool = ICLPool(PoolAddress.computeAddress(factory, poolKey));

        (uint128 tokensOwed0, uint128 tokensOwed1) = (position.tokensOwed0, position.tokensOwed1);

        address gauge = pool.gauge();
        bool isStaked = ownerOf(params.tokenId) == gauge;

        // trigger an update of the position fees owed and fee growth snapshots if it has any liquidity
        if (position.liquidity > 0) {
            uint256 feeGrowthInside0LastX128;
            uint256 feeGrowthInside1LastX128;
            if (!isStaked) {
                pool.burn(position.tickLower, position.tickUpper, 0);

                (, feeGrowthInside0LastX128, feeGrowthInside1LastX128,,) =
                    pool.positions(PositionKey.compute(address(this), position.tickLower, position.tickUpper));

                tokensOwed0 += uint128(
                    FullMath.mulDiv(
                        feeGrowthInside0LastX128 - position.feeGrowthInside0LastX128,
                        position.liquidity,
                        FixedPoint128.Q128
                    )
                );
                tokensOwed1 += uint128(
                    FullMath.mulDiv(
                        feeGrowthInside1LastX128 - position.feeGrowthInside1LastX128,
                        position.liquidity,
                        FixedPoint128.Q128
                    )
                );
            } else {
                pool.burn(position.tickLower, position.tickUpper, 0, gauge);

                (, feeGrowthInside0LastX128, feeGrowthInside1LastX128,,) =
                    pool.positions(PositionKey.compute(gauge, position.tickLower, position.tickUpper));
            }

            position.feeGrowthInside0LastX128 = feeGrowthInside0LastX128;
            position.feeGrowthInside1LastX128 = feeGrowthInside1LastX128;
        }

        // compute the arguments to give to the pool#collect method
        (uint128 amount0Collect, uint128 amount1Collect) = (
            params.amount0Max > tokensOwed0 ? tokensOwed0 : params.amount0Max,
            params.amount1Max > tokensOwed1 ? tokensOwed1 : params.amount1Max
        );

        // the actual amounts collected are returned
        if (!isStaked) {
            (amount0, amount1) =
                pool.collect(recipient, position.tickLower, position.tickUpper, amount0Collect, amount1Collect);
        } else {
            (amount0, amount1) =
                pool.collect(recipient, position.tickLower, position.tickUpper, amount0Collect, amount1Collect, gauge);
        }

        // sometimes there will be a few less wei than expected due to rounding down in core, but we just subtract the full amount expected
        // instead of the actual amount so we can burn the token
        (position.tokensOwed0, position.tokensOwed1) = (tokensOwed0 - amount0Collect, tokensOwed1 - amount1Collect);

        emit MetadataUpdate(params.tokenId);
        emit Collect(params.tokenId, recipient, amount0Collect, amount1Collect);
    }

    /// @inheritdoc INonfungiblePositionManager
    function burn(uint256 tokenId) external payable override isAuthorizedForToken(tokenId) {
        Position storage position = _positions[tokenId];
        require(position.liquidity == 0 && position.tokensOwed0 == 0 && position.tokensOwed1 == 0, "NC");
        delete _positions[tokenId];
        _burn(tokenId);
    }

    function _getAndIncrementNonce(uint256 tokenId) internal override returns (uint256) {
        return uint256(_positions[tokenId].nonce++);
    }

    /// @inheritdoc IERC721
    function getApproved(uint256 tokenId) public view override(ERC721, IERC721) returns (address) {
        require(_exists(tokenId), "NE");

        return _positions[tokenId].operator;
    }

    /// @dev Overrides _approve to use the operator in the position, which is packed with the position permit nonce
    function _approve(address to, uint256 tokenId) internal override(ERC721) {
        _positions[tokenId].operator = to;
        emit Approval(ownerOf(tokenId), to, tokenId);
    }

    /// @inheritdoc INonfungiblePositionManager
    function setTokenDescriptor(address _tokenDescriptor) external override onlyOwner {
        require(_tokenDescriptor != address(0));
        tokenDescriptor = _tokenDescriptor;
        emit BatchMetadataUpdate(0, type(uint256).max);
        emit TokenDescriptorChanged(_tokenDescriptor);
    }

    /// @inheritdoc INonfungiblePositionManager
    function setOwner(address _owner) external override onlyOwner {
        require(_owner != address(0));
        owner = _owner;
        emit TransferOwnership(_owner);
    }
}

// SPDX-License-Identifier: MIT
pragma solidity =0.7.6;

interface IVotingEscrow {
    function team() external returns (address);

    /// @notice Deposit `_value` tokens for `msg.sender` and lock for `_lockDuration`
    /// @param _value Amount to deposit
    /// @param _lockDuration Number of seconds to lock tokens for (rounded down to nearest week)
    /// @return TokenId of created veNFT
    function createLock(uint256 _value, uint256 _lockDuration) external returns (uint256);
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

// SPDX-License-Identifier: BUSL-1.1
pragma solidity =0.7.6;
interface IMinter {
    /// @notice Processes emissions and rebases. Callable once per epoch (1 week).
    /// @return _period Start of current epoch.
    function updatePeriod() external returns (uint256 _period);
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
// SPDX-License-Identifier: MIT
pragma solidity =0.7.6;

interface IFactoryRegistry {
    function approve(address poolFactory, address votingRewardsFactory, address gaugeFactory) external;

    function isPoolFactoryApproved(address poolFactory) external returns (bool);

    function factoriesToPoolFactory(address poolFactory)
        external
        returns (address votingRewardsFactory, address gaugeFactory);
}

// SPDX-License-Identifier: MIT
pragma solidity 0.7.6;

interface IGaugeManager {
    
    struct FarmingParam {
        address farmingCenter;
        address algebraEternalFarming;
        address nfpm;
    }

    function isGaugeAliveForPool(address _pool) external view returns (bool);
    function gauges(address _pair) external view returns (address);
    function isGauge(address _gauge) external view returns (bool);
    function poolForGauge(address _gauge) external view returns (address);
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

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity =0.7.6;
pragma abicoder v2;

import "contracts/core/interfaces/ICLFactory.sol";
import "contracts/core/interfaces/callback/ICLMintCallback.sol";
import "contracts/core/libraries/TickMath.sol";

import "../libraries/PoolAddress.sol";
import "../libraries/CallbackValidation.sol";
import "../libraries/LiquidityAmounts.sol";

import "./PeripheryPayments.sol";
import "./PeripheryImmutableState.sol";

/// @title Liquidity management functions
/// @notice Internal functions for safely managing liquidity in CL
abstract contract LiquidityManagement is ICLMintCallback, PeripheryImmutableState, PeripheryPayments {
    struct MintCallbackData {
        PoolAddress.PoolKey poolKey;
        address payer;
    }

    /// @inheritdoc ICLMintCallback
    function uniswapV3MintCallback(uint256 amount0Owed, uint256 amount1Owed, bytes calldata data) external override {
        MintCallbackData memory decoded = abi.decode(data, (MintCallbackData));
        CallbackValidation.verifyCallback(factory, decoded.poolKey);

        if (amount0Owed > 0) pay(decoded.poolKey.token0, decoded.payer, msg.sender, amount0Owed);
        if (amount1Owed > 0) pay(decoded.poolKey.token1, decoded.payer, msg.sender, amount1Owed);
    }

    struct AddLiquidityParams {
        address poolAddress;
        PoolAddress.PoolKey poolKey;
        address recipient;
        int24 tickLower;
        int24 tickUpper;
        uint256 amount0Desired;
        uint256 amount1Desired;
        uint256 amount0Min;
        uint256 amount1Min;
    }

    /// @notice Add liquidity to an initialized pool
    function addLiquidity(AddLiquidityParams memory params)
        internal
        returns (uint128 liquidity, uint256 amount0, uint256 amount1)
    {
        ICLPool pool = ICLPool(params.poolAddress);

        // compute the liquidity amount
        {
            (uint160 sqrtPriceX96,,,,,) = pool.slot0();
            uint160 sqrtRatioAX96 = TickMath.getSqrtRatioAtTick(params.tickLower);
            uint160 sqrtRatioBX96 = TickMath.getSqrtRatioAtTick(params.tickUpper);

            liquidity = LiquidityAmounts.getLiquidityForAmounts(
                sqrtPriceX96, sqrtRatioAX96, sqrtRatioBX96, params.amount0Desired, params.amount1Desired
            );
        }

        (amount0, amount1) = pool.mint(
            params.recipient,
            params.tickLower,
            params.tickUpper,
            liquidity,
            abi.encode(MintCallbackData({poolKey: params.poolKey, payer: msg.sender}))
        );

        require(amount0 >= params.amount0Min && amount1 >= params.amount1Min, "PSC"); // price slippage check
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity >=0.5.0;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/drafts/IERC20Permit.sol";

import "../interfaces/ISelfPermit.sol";
import "../interfaces/external/IERC20PermitAllowed.sol";

/// @title Self Permit
/// @notice Functionality to call permit on any EIP-2612-compliant token for use in the route
/// @dev These functions are expected to be embedded in multicalls to allow EOAs to approve a contract and call a function
/// that requires an approval in a single transaction.
abstract contract SelfPermit is ISelfPermit {
    /// @inheritdoc ISelfPermit
    function selfPermit(address token, uint256 value, uint256 deadline, uint8 v, bytes32 r, bytes32 s)
        public
        payable
        override
    {
        IERC20Permit(token).permit(msg.sender, address(this), value, deadline, v, r, s);
    }

    /// @inheritdoc ISelfPermit
    function selfPermitIfNecessary(address token, uint256 value, uint256 deadline, uint8 v, bytes32 r, bytes32 s)
        external
        payable
        override
    {
        if (IERC20(token).allowance(msg.sender, address(this)) < value) selfPermit(token, value, deadline, v, r, s);
    }

    /// @inheritdoc ISelfPermit
    function selfPermitAllowed(address token, uint256 nonce, uint256 expiry, uint8 v, bytes32 r, bytes32 s)
        public
        payable
        override
    {
        IERC20PermitAllowed(token).permit(msg.sender, address(this), nonce, expiry, true, v, r, s);
    }

    /// @inheritdoc ISelfPermit
    function selfPermitAllowedIfNecessary(address token, uint256 nonce, uint256 expiry, uint8 v, bytes32 r, bytes32 s)
        external
        payable
        override
    {
        if (IERC20(token).allowance(msg.sender, address(this)) < type(uint256).max) {
            selfPermitAllowed(token, nonce, expiry, v, r, s);
        }
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity =0.7.6;

import "./BlockTimestamp.sol";

abstract contract PeripheryValidation is BlockTimestamp {
    modifier checkDeadline(uint256 deadline) {
        require(_blockTimestamp() <= deadline);
        _;
    }
}

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity =0.7.6;
pragma abicoder v2;

import "../interfaces/IMulticall.sol";

/// @title Multicall
/// @notice Enables calling multiple methods in a single call to the contract
abstract contract Multicall is IMulticall {
    /// @inheritdoc IMulticall
    function multicall(bytes[] calldata data) public payable override returns (bytes[] memory results) {
        results = new bytes[](data.length);
        for (uint256 i = 0; i < data.length; i++) {
            (bool success, bytes memory result) = address(this).delegatecall(data[i]);

            if (!success) {
                // Next 5 lines from https://ethereum.stackexchange.com/a/83577
                if (result.length < 68) revert();
                assembly {
                    result := add(result, 0x04)
                }
                revert(abi.decode(result, (string)));
            }

            results[i] = result;
        }
    }
}


END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

