
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import './interfaces/IPair.sol';
import './interfaces/IBribe.sol';
import "./libraries/Math.sol";

import {HybraTimeLibrary} from "./libraries/HybraTimeLibrary.sol";
import './interfaces/IRHYBR.sol';
interface IRewarder {
    function onReward(
        address user,
        address recipient,
        uint256 userBalance
    ) external;
}


contract GaugeV2 is ReentrancyGuard, Ownable {

    using SafeERC20 for IERC20;

    bool public immutable isForPair;
    bool public emergency;


    IERC20 public immutable rewardToken;
    IERC20 public immutable TOKEN;
    address public immutable rHYBR;
    address public VE;
    address public DISTRIBUTION;
    address public gaugeRewarder;
    address public internal_bribe;
    address public external_bribe;

    uint256 public DURATION;
    uint256 internal _periodFinish;
    uint256 public rewardRate;
    uint256 public lastUpdateTime;
    uint256 public rewardPerTokenStored;

   

    mapping(address => uint256) public userRewardPerTokenPaid;
    mapping(address => uint256) public rewards;

    uint256 internal _totalSupply;
    mapping(address => uint256) internal _balances;
    mapping(address => uint256) public maturityTime;

    event RewardAdded(uint256 reward);
    event Deposit(address indexed user, uint256 amount);
    event Withdraw(address indexed user, uint256 amount);
    event Harvest(address indexed user, uint256 reward);

    event ClaimFees(address indexed from, uint256 claimed0, uint256 claimed1);
    event EmergencyActivated(address indexed gauge, uint256 timestamp);
    event EmergencyDeactivated(address indexed gauge, uint256 timestamp);

    modifier updateReward(address account) {
        rewardPerTokenStored = rewardPerToken();
        lastUpdateTime = lastTimeRewardApplicable();
        if (account != address(0)) {
            rewards[account] = earned(account);
            userRewardPerTokenPaid[account] = rewardPerTokenStored;
        }
        _;
    }

    modifier onlyDistribution() {
        require(msg.sender == DISTRIBUTION, "NA");
        _;
    }

  

  

    modifier isNotEmergency() {
        require(emergency == false, "EMER");
        _;
    }

    constructor(address _rewardToken,address _rHYBR,address _ve,address _token,address _distribution, address _internal_bribe, address _external_bribe, bool _isForPair) {
        rewardToken = IERC20(_rewardToken);     // main reward
        rHYBR = _rHYBR;
        VE = _ve;                               // vested
        TOKEN = IERC20(_token);                 // underlying (LP)
        DISTRIBUTION = _distribution;           // distro address (GaugeManager)
        DURATION = HybraTimeLibrary.WEEK;                   

        internal_bribe = _internal_bribe;       // lp fees goes here
        external_bribe = _external_bribe;       // bribe fees goes here


        isForPair = _isForPair;                 // pair boolean, if false no claim_fees

        emergency = false;                      // emergency flag

    }


    /* -----------------------------------------------------------------------------
    --------------------------------------------------------------------------------
    --------------------------------------------------------------------------------
                                    ONLY OWNER
    --------------------------------------------------------------------------------
    --------------------------------------------------------------------------------
    ----------------------------------------------------------------------------- */

    ///@notice set distribution address (should be GaugeManager)
    function setDistribution(address _distribution) external onlyOwner {
        require(_distribution != address(0), "ZA");
        require(_distribution != DISTRIBUTION, "SAME_ADDR");
        DISTRIBUTION = _distribution;
    }

    ///@notice set gauge rewarder address
    function setGaugeRewarder(address _gaugeRewarder) external onlyOwner {
        require(_gaugeRewarder != gaugeRewarder, "SAME_ADDR");
        gaugeRewarder = _gaugeRewarder;
    }


    ///@notice set new internal bribe contract (where to send fees)
    function setInternalBribe(address _int) external onlyOwner {
        require(_int >= address(0), "ZA");
        internal_bribe = _int;
    }

    function activateEmergencyMode() external onlyOwner {
        require(emergency == false, "EMER");
        emergency = true;
        emit EmergencyActivated(address(this), block.timestamp);
    }

    function stopEmergencyMode() external onlyOwner {

        require(emergency == true,"EMER");

        emergency = false;
        emit EmergencyDeactivated(address(this), block.timestamp);
    }


    /* -----------------------------------------------------------------------------
    --------------------------------------------------------------------------------
    --------------------------------------------------------------------------------
                                    VIEW FUNCTIONS
    --------------------------------------------------------------------------------
    --------------------------------------------------------------------------------
    ----------------------------------------------------------------------------- */

    ///@notice total supply held
    function totalSupply() public view returns (uint256) {
        return _totalSupply;
    }

    ///@notice balance of a user
    function balanceOf(address account) external view returns (uint256) {
        return _balanceOf(account);
    }

    function _balanceOf(address account) internal view returns (uint256) {
       
        return _balances[account];
    }

    ///@notice last time reward
    function lastTimeRewardApplicable() public view returns (uint256) {
        return Math.min(block.timestamp, _periodFinish);
    }

    ///@notice  reward for a sinle token
    function rewardPerToken() public view returns (uint256) {
        if (_totalSupply == 0) {
            return rewardPerTokenStored;
        } else {
            return rewardPerTokenStored + (lastTimeRewardApplicable() - lastUpdateTime) * rewardRate * 1e18 / _totalSupply; 
        }
    }

    ///@notice see earned rewards for user
    function earned(address account) public view returns (uint256) {
        return rewards[account] + _balanceOf(account) * (rewardPerToken() - userRewardPerTokenPaid[account]) / 1e18;  
    }

    ///@notice get total reward for the duration
    function rewardForDuration() external view returns (uint256) {
        return rewardRate * DURATION;
    }

    function periodFinish() external view returns (uint256) {
        return _periodFinish;
    }



    /* -----------------------------------------------------------------------------
    --------------------------------------------------------------------------------
    --------------------------------------------------------------------------------
                                    USER INTERACTION
    --------------------------------------------------------------------------------
    --------------------------------------------------------------------------------
    ----------------------------------------------------------------------------- */



    // send whole liquidity as additional param


    ///@notice deposit all TOKEN of msg.sender
    function depositAll() external {
        _deposit(TOKEN.balanceOf(msg.sender), msg.sender);
    }

    ///@notice deposit amount TOKEN
    function deposit(uint256 amount) external {
        _deposit(amount, msg.sender);
    }

    ///@notice deposit internal
    function _deposit(uint256 amount, address account) internal nonReentrant isNotEmergency updateReward(account) {
        require(amount > 0, "ZV");

        _balances[account] = _balances[account] + amount;
        _totalSupply = _totalSupply + amount;
        if (address(gaugeRewarder) != address(0)) {
            IRewarder(gaugeRewarder).onReward(account, account, _balanceOf(account));
        }

        TOKEN.safeTransferFrom(account, address(this), amount);

        emit Deposit(account, amount);
    }

    ///@notice withdraw all token
    function withdrawAll() external {
        _withdraw(_balanceOf(msg.sender));
    }

    ///@notice withdraw a certain amount of TOKEN
    function withdraw(uint256 amount) external {
        _withdraw(amount);
    }

    ///@notice withdraw internal
    function _withdraw(uint256 amount) internal nonReentrant isNotEmergency updateReward(msg.sender) {
        require(amount > 0, "ZV");
        require(_balanceOf(msg.sender) > 0, "ZV");
        require(block.timestamp >= maturityTime[msg.sender], "!MATURE");

        _totalSupply = _totalSupply - amount;
        _balances[msg.sender] = _balances[msg.sender] - amount;

        if (address(gaugeRewarder) != address(0)) {
            IRewarder(gaugeRewarder).onReward(msg.sender, msg.sender,_balanceOf(msg.sender));
        }

        TOKEN.safeTransfer(msg.sender, amount);

        emit Withdraw(msg.sender, amount);
    }

    function emergencyWithdraw() external nonReentrant {
        require(emergency, "EMER");
        uint256 _amount = _balanceOf(msg.sender);
        require(_amount > 0, "ZV");
        _totalSupply = _totalSupply - _amount;

        _balances[msg.sender] = 0;
   

        TOKEN.safeTransfer(msg.sender, _amount);
        emit Withdraw(msg.sender, _amount);
    }

    function emergencyWithdrawAmount(uint256 _amount) external nonReentrant {

        require(emergency, "EMER");
        _totalSupply = _totalSupply - _amount;

        _balances[msg.sender] = _balances[msg.sender] - _amount;

        TOKEN.safeTransfer(msg.sender, _amount);
        emit Withdraw(msg.sender, _amount);
    }

  

    ///@notice withdraw all TOKEN and harvest rewardToken
    function withdrawAllAndHarvest(uint8 _redeemType) external {
        _withdraw(_balanceOf(msg.sender));
        getReward(_redeemType);
    }

 
    ///@notice User harvest function called from distribution (GaugeManager allows harvest on multiple gauges)
    function getReward(address _user, uint8 _redeemType) public nonReentrant onlyDistribution updateReward(_user) {
        uint256 reward = rewards[_user];
        if (reward > 0) {
            rewards[_user] = 0;
            IERC20(rewardToken).safeApprove(rHYBR, reward);
            IRHYBR(rHYBR).depostionEmissionsToken(reward);
            IRHYBR(rHYBR).redeemFor(reward, _redeemType, _user);
            emit Harvest(_user, reward);
        }

        if (gaugeRewarder != address(0)) {
            IRewarder(gaugeRewarder).onReward(_user, _user, _balanceOf(_user));
        }
    }

    ///@notice User harvest function
    function getReward(uint8 _redeemType) public nonReentrant updateReward(msg.sender) {
        uint256 reward = rewards[msg.sender];
        if (reward > 0) {
            rewards[msg.sender] = 0;
            IERC20(rewardToken).safeApprove(rHYBR, reward);
            IRHYBR(rHYBR).depostionEmissionsToken(reward);
            IRHYBR(rHYBR).redeemFor(reward, _redeemType, msg.sender);
            emit Harvest(msg.sender, reward);
        }

        if (gaugeRewarder != address(0)) {
            IRewarder(gaugeRewarder).onReward(msg.sender, msg.sender, _balanceOf(msg.sender));
        }
    }








    /* -----------------------------------------------------------------------------
    --------------------------------------------------------------------------------
    --------------------------------------------------------------------------------
                                    DISTRIBUTION
    --------------------------------------------------------------------------------
    --------------------------------------------------------------------------------
    ----------------------------------------------------------------------------- */


    /// @dev Receive rewards from distribution

    function notifyRewardAmount(address token, uint256 reward) external nonReentrant isNotEmergency onlyDistribution updateReward(address(0)) {
        require(token == address(rewardToken), "IA");
        rewardToken.safeTransferFrom(DISTRIBUTION, address(this), reward);

        if (block.timestamp >= _periodFinish) {
            rewardRate = reward / DURATION;
        } else {
            uint256 remaining = _periodFinish - block.timestamp;
            uint256 leftover = remaining * rewardRate;
            rewardRate = (reward + leftover) / DURATION;
        }

        // Ensure the provided reward amount is not more than the balance in the contract.
        // This keeps the reward rate in the right range, preventing overflows due to
        // very high values of rewardRate in the earned and rewardsPerToken functions;
        // Reward + leftover must be less than 2^256 / 10^18 to avoid overflow.
        uint256 balance = rewardToken.balanceOf(address(this));
        require(rewardRate <= balance / DURATION, "REWARD_HIGH");

        lastUpdateTime = block.timestamp;
        _periodFinish = block.timestamp + DURATION;
        emit RewardAdded(reward);
    }


    function claimFees() external nonReentrant returns (uint256 claimed0, uint256 claimed1) {
        return _claimFees();
    }

     function _claimFees() internal returns (uint256 claimed0, uint256 claimed1) {
        if (!isForPair) {
            return (0, 0);
        }
        address _token = address(TOKEN);
        (claimed0, claimed1) = IPair(_token).claimFees();
        if (claimed0 > 0 || claimed1 > 0) {

            uint256 _fees0 = claimed0;
            uint256 _fees1 = claimed1;

            (address _token0, address _token1) = IPair(_token).tokens();

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

  
}

END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
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

// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

interface IPair {
    function metadata() external view returns (uint dec0, uint dec1, uint r0, uint r1, bool st, address t0, address t1);
    function claimFees() external returns (uint, uint);
    function tokens() external view returns (address, address);
    function token0() external view returns (address);
    function token1() external view returns (address);
    function transferFrom(address src, address dst, uint amount) external returns (bool);
    function permit(address owner, address spender, uint value, uint deadline, uint8 v, bytes32 r, bytes32 s) external;
    function swap(uint amount0Out, uint amount1Out, address to, bytes calldata data) external;
    function burn(address to) external returns (uint amount0, uint amount1);
    function mint(address to) external returns (uint liquidity);
    function getReserves() external view returns (uint _reserve0, uint _reserve1, uint _blockTimestampLast);
    function getAmountOut(uint, address) external view returns (uint);

    function name() external view returns(string memory);
    function symbol() external view returns(string memory);
    function totalSupply() external view returns (uint);
    function decimals() external view returns (uint8);

    function claimable0(address _user) external view returns (uint);
    function claimable1(address _user) external view returns (uint);

    function isStable() external view returns(bool);
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
// SPDX-License-Identifier: BUSL-1.1
pragma solidity =0.7.6;
interface IMinter {
    /// @notice Processes emissions and rebases. Callable once per epoch (1 week).
    /// @return _period Start of current epoch.
    function updatePeriod() external returns (uint256 _period);
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import './interfaces/IPair.sol';
import './interfaces/IBribe.sol';
import "./libraries/Math.sol";

import {HybraTimeLibrary} from "./libraries/HybraTimeLibrary.sol";
import './interfaces/IRHYBR.sol';
interface IRewarder {
    function onReward(
        address user,
        address recipient,
        uint256 userBalance
    ) external;
}


contract GaugeV2 is ReentrancyGuard, Ownable {

    using SafeERC20 for IERC20;

    bool public immutable isForPair;
    bool public emergency;


    IERC20 public immutable rewardToken;
    IERC20 public immutable TOKEN;
    address public immutable rHYBR;
    address public VE;
    address public DISTRIBUTION;
    address public gaugeRewarder;
    address public internal_bribe;
    address public external_bribe;

    uint256 public DURATION;
    uint256 internal _periodFinish;
    uint256 public rewardRate;
    uint256 public lastUpdateTime;
    uint256 public rewardPerTokenStored;

   

    mapping(address => uint256) public userRewardPerTokenPaid;
    mapping(address => uint256) public rewards;

    uint256 internal _totalSupply;
    mapping(address => uint256) internal _balances;
    mapping(address => uint256) public maturityTime;

    event RewardAdded(uint256 reward);
    event Deposit(address indexed user, uint256 amount);
    event Withdraw(address indexed user, uint256 amount);
    event Harvest(address indexed user, uint256 reward);

    event ClaimFees(address indexed from, uint256 claimed0, uint256 claimed1);
    event EmergencyActivated(address indexed gauge, uint256 timestamp);
    event EmergencyDeactivated(address indexed gauge, uint256 timestamp);

    modifier updateReward(address account) {
        rewardPerTokenStored = rewardPerToken();
        lastUpdateTime = lastTimeRewardApplicable();
        if (account != address(0)) {
            rewards[account] = earned(account);
            userRewardPerTokenPaid[account] = rewardPerTokenStored;
        }
        _;
    }

    modifier onlyDistribution() {
        require(msg.sender == DISTRIBUTION, "NA");
        _;
    }

  

  

    modifier isNotEmergency() {
        require(emergency == false, "EMER");
        _;
    }

    constructor(address _rewardToken,address _rHYBR,address _ve,address _token,address _distribution, address _internal_bribe, address _external_bribe, bool _isForPair) {
        rewardToken = IERC20(_rewardToken);     // main reward
        rHYBR = _rHYBR;
        VE = _ve;                               // vested
        TOKEN = IERC20(_token);                 // underlying (LP)
        DISTRIBUTION = _distribution;           // distro address (GaugeManager)
        DURATION = HybraTimeLibrary.WEEK;                   

        internal_bribe = _internal_bribe;       // lp fees goes here
        external_bribe = _external_bribe;       // bribe fees goes here


        isForPair = _isForPair;                 // pair boolean, if false no claim_fees

        emergency = false;                      // emergency flag

    }


    /* -----------------------------------------------------------------------------
    --------------------------------------------------------------------------------
    --------------------------------------------------------------------------------
                                    ONLY OWNER
    --------------------------------------------------------------------------------
    --------------------------------------------------------------------------------
    ----------------------------------------------------------------------------- */

    ///@notice set distribution address (should be GaugeManager)
    function setDistribution(address _distribution) external onlyOwner {
        require(_distribution != address(0), "ZA");
        require(_distribution != DISTRIBUTION, "SAME_ADDR");
        DISTRIBUTION = _distribution;
    }

    ///@notice set gauge rewarder address
    function setGaugeRewarder(address _gaugeRewarder) external onlyOwner {
        require(_gaugeRewarder != gaugeRewarder, "SAME_ADDR");
        gaugeRewarder = _gaugeRewarder;
    }


    ///@notice set new internal bribe contract (where to send fees)
    function setInternalBribe(address _int) external onlyOwner {
        require(_int >= address(0), "ZA");
        internal_bribe = _int;
    }

    function activateEmergencyMode() external onlyOwner {
        require(emergency == false, "EMER");
        emergency = true;
        emit EmergencyActivated(address(this), block.timestamp);
    }

    function stopEmergencyMode() external onlyOwner {

        require(emergency == true,"EMER");

        emergency = false;
        emit EmergencyDeactivated(address(this), block.timestamp);
    }


    /* -----------------------------------------------------------------------------
    --------------------------------------------------------------------------------
    --------------------------------------------------------------------------------
                                    VIEW FUNCTIONS
    --------------------------------------------------------------------------------
    --------------------------------------------------------------------------------
    ----------------------------------------------------------------------------- */

    ///@notice total supply held
    function totalSupply() public view returns (uint256) {
        return _totalSupply;
    }

    ///@notice balance of a user
    function balanceOf(address account) external view returns (uint256) {
        return _balanceOf(account);
    }

    function _balanceOf(address account) internal view returns (uint256) {
       
        return _balances[account];
    }

    ///@notice last time reward
    function lastTimeRewardApplicable() public view returns (uint256) {
        return Math.min(block.timestamp, _periodFinish);
    }

    ///@notice  reward for a sinle token
    function rewardPerToken() public view returns (uint256) {
        if (_totalSupply == 0) {
            return rewardPerTokenStored;
        } else {
            return rewardPerTokenStored + (lastTimeRewardApplicable() - lastUpdateTime) * rewardRate * 1e18 / _totalSupply; 
        }
    }

    ///@notice see earned rewards for user
    function earned(address account) public view returns (uint256) {
        return rewards[account] + _balanceOf(account) * (rewardPerToken() - userRewardPerTokenPaid[account]) / 1e18;  
    }

    ///@notice get total reward for the duration
    function rewardForDuration() external view returns (uint256) {
        return rewardRate * DURATION;
    }

    function periodFinish() external view returns (uint256) {
        return _periodFinish;
    }



    /* -----------------------------------------------------------------------------
    --------------------------------------------------------------------------------
    --------------------------------------------------------------------------------
                                    USER INTERACTION
    --------------------------------------------------------------------------------
    --------------------------------------------------------------------------------
    ----------------------------------------------------------------------------- */



    // send whole liquidity as additional param


    ///@notice deposit all TOKEN of msg.sender
    function depositAll() external {
        _deposit(TOKEN.balanceOf(msg.sender), msg.sender);
    }

    ///@notice deposit amount TOKEN
    function deposit(uint256 amount) external {
        _deposit(amount, msg.sender);
    }

    ///@notice deposit internal
    function _deposit(uint256 amount, address account) internal nonReentrant isNotEmergency updateReward(account) {
        require(amount > 0, "ZV");

        _balances[account] = _balances[account] + amount;
        _totalSupply = _totalSupply + amount;
        if (address(gaugeRewarder) != address(0)) {
            IRewarder(gaugeRewarder).onReward(account, account, _balanceOf(account));
        }

        TOKEN.safeTransferFrom(account, address(this), amount);

        emit Deposit(account, amount);
    }

    ///@notice withdraw all token
    function withdrawAll() external {
        _withdraw(_balanceOf(msg.sender));
    }

    ///@notice withdraw a certain amount of TOKEN
    function withdraw(uint256 amount) external {
        _withdraw(amount);
    }

    ///@notice withdraw internal
    function _withdraw(uint256 amount) internal nonReentrant isNotEmergency updateReward(msg.sender) {
        require(amount > 0, "ZV");
        require(_balanceOf(msg.sender) > 0, "ZV");
        require(block.timestamp >= maturityTime[msg.sender], "!MATURE");

        _totalSupply = _totalSupply - amount;
        _balances[msg.sender] = _balances[msg.sender] - amount;

        if (address(gaugeRewarder) != address(0)) {
            IRewarder(gaugeRewarder).onReward(msg.sender, msg.sender,_balanceOf(msg.sender));
        }

        TOKEN.safeTransfer(msg.sender, amount);

        emit Withdraw(msg.sender, amount);
    }

    function emergencyWithdraw() external nonReentrant {
        require(emergency, "EMER");
        uint256 _amount = _balanceOf(msg.sender);
        require(_amount > 0, "ZV");
        _totalSupply = _totalSupply - _amount;

        _balances[msg.sender] = 0;
   

        TOKEN.safeTransfer(msg.sender, _amount);
        emit Withdraw(msg.sender, _amount);
    }

    function emergencyWithdrawAmount(uint256 _amount) external nonReentrant {

        require(emergency, "EMER");
        _totalSupply = _totalSupply - _amount;

        _balances[msg.sender] = _balances[msg.sender] - _amount;

        TOKEN.safeTransfer(msg.sender, _amount);
        emit Withdraw(msg.sender, _amount);
    }

  

    ///@notice withdraw all TOKEN and harvest rewardToken
    function withdrawAllAndHarvest(uint8 _redeemType) external {
        _withdraw(_balanceOf(msg.sender));
        getReward(_redeemType);
    }

 
    ///@notice User harvest function called from distribution (GaugeManager allows harvest on multiple gauges)
    function getReward(address _user, uint8 _redeemType) public nonReentrant onlyDistribution updateReward(_user) {
        uint256 reward = rewards[_user];
        if (reward > 0) {
            rewards[_user] = 0;
            IERC20(rewardToken).safeApprove(rHYBR, reward);
            IRHYBR(rHYBR).depostionEmissionsToken(reward);
            IRHYBR(rHYBR).redeemFor(reward, _redeemType, _user);
            emit Harvest(_user, reward);
        }

        if (gaugeRewarder != address(0)) {
            IRewarder(gaugeRewarder).onReward(_user, _user, _balanceOf(_user));
        }
    }

    ///@notice User harvest function
    function getReward(uint8 _redeemType) public nonReentrant updateReward(msg.sender) {
        uint256 reward = rewards[msg.sender];
        if (reward > 0) {
            rewards[msg.sender] = 0;
            IERC20(rewardToken).safeApprove(rHYBR, reward);
            IRHYBR(rHYBR).depostionEmissionsToken(reward);
            IRHYBR(rHYBR).redeemFor(reward, _redeemType, msg.sender);
            emit Harvest(msg.sender, reward);
        }

        if (gaugeRewarder != address(0)) {
            IRewarder(gaugeRewarder).onReward(msg.sender, msg.sender, _balanceOf(msg.sender));
        }
    }








    /* -----------------------------------------------------------------------------
    --------------------------------------------------------------------------------
    --------------------------------------------------------------------------------
                                    DISTRIBUTION
    --------------------------------------------------------------------------------
    --------------------------------------------------------------------------------
    ----------------------------------------------------------------------------- */


    /// @dev Receive rewards from distribution

    function notifyRewardAmount(address token, uint256 reward) external nonReentrant isNotEmergency onlyDistribution updateReward(address(0)) {
        require(token == address(rewardToken), "IA");
        rewardToken.safeTransferFrom(DISTRIBUTION, address(this), reward);

        if (block.timestamp >= _periodFinish) {
            rewardRate = reward / DURATION;
        } else {
            uint256 remaining = _periodFinish - block.timestamp;
            uint256 leftover = remaining * rewardRate;
            rewardRate = (reward + leftover) / DURATION;
        }

        // Ensure the provided reward amount is not more than the balance in the contract.
        // This keeps the reward rate in the right range, preventing overflows due to
        // very high values of rewardRate in the earned and rewardsPerToken functions;
        // Reward + leftover must be less than 2^256 / 10^18 to avoid overflow.
        uint256 balance = rewardToken.balanceOf(address(this));
        require(rewardRate <= balance / DURATION, "REWARD_HIGH");

        lastUpdateTime = block.timestamp;
        _periodFinish = block.timestamp + DURATION;
        emit RewardAdded(reward);
    }


    function claimFees() external nonReentrant returns (uint256 claimed0, uint256 claimed1) {
        return _claimFees();
    }

     function _claimFees() internal returns (uint256 claimed0, uint256 claimed1) {
        if (!isForPair) {
            return (0, 0);
        }
        address _token = address(TOKEN);
        (claimed0, claimed1) = IPair(_token).claimFees();
        if (claimed0 > 0 || claimed1 > 0) {

            uint256 _fees0 = claimed0;
            uint256 _fees1 = claimed1;

            (address _token0, address _token1) = IPair(_token).tokens();

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


END OF SUPPORTING CONTRACTS AND INTERFACES


DEPLOYMENT SCRIPTS

