
## *MAIN TARGET CONTRACT* TO REVIEW

// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

import './libraries/Math.sol';
import './interfaces/IVoter.sol';
import './interfaces/ITokenHandler.sol';
import './interfaces/IERC20.sol';
import './interfaces/IPairInfo.sol';
import './interfaces/IPairFactory.sol';
import './interfaces/IVotingEscrow.sol';
import './interfaces/IPermissionsRegistry.sol';
import './interfaces/IGaugeFactoryCL.sol';
import './interfaces/IGaugeManager.sol';
import './interfaces/IBribe.sol';
import './interfaces/IBribeFactory.sol';
import './interfaces/IGauge.sol';
import './interfaces/IMinter.sol';
import './interfaces/IGaugeCL.sol';
import './interfaces/IBribe.sol';
import './interfaces/IGaugeFactory.sol';
import "./CLGauge/interface/ICLPool.sol";
import {VoterFactoryLib} from "./libraries/VoterFactoryLib.sol";
import {HybraTimeLibrary} from "./libraries/HybraTimeLibrary.sol";
import "@openzeppelin/contracts-upgradeable/security/ReentrancyGuardUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/utils/SafeERC20Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/IERC20Upgradeable.sol";


contract GaugeManager is OwnableUpgradeable, ReentrancyGuardUpgradeable {
    using SafeERC20Upgradeable for IERC20Upgradeable;
    address[] public pools;
    
    address public minter; 
    uint256 internal index; 
    address internal base; 
    address public bribefactory; 
    address public _ve; 
    mapping(address => uint256) internal supplyIndex;              // gauge    => index
    mapping(address => uint256) public claimable;                  // gauge    => claimable $the
    mapping(address => address) public gauges;                  // pool     => gauge
    mapping(address => uint256) public gaugesDistributionTimestmap;// gauge    => last Distribution Time
    mapping(address => address) public poolForGauge;            // gauge    => pool    
    mapping(address => address) public internal_bribes;         // gauge    => internal bribe (only fees)
    mapping(address => address) public external_bribes;         // gauge    => external bribe (real bribes)
    
    VoterFactoryLib.Data private _factoriesData;
    address public permissionRegistry;  
    address public voter;  
    address public tokenHandler; 
    address public HybraGovernor;
    address public nfpm;
    mapping(address => bool) public isGauge;                    // gauge    => boolean [is a gauge?]
    mapping(address => bool) public isCLGauge;
    mapping(address => bool) public isAlive;                    // gauge    => boolean [is the gauge alive?]




  
    event GaugeCreated(address indexed gauge, address creator, address internal_bribe, address indexed external_bribe, address indexed pool);
    event GaugeKilled(address indexed gauge);
    event GaugeRevived(address indexed gauge);
    event NotifyReward(address indexed sender, address indexed reward, uint256 amount);
    event DistributeReward(address indexed sender, address indexed gauge, uint256 amount);
    event SetBribeFor(bool isInternal, address indexed old, address indexed latest, address indexed gauge);
    event SetMinter(address indexed old, address indexed latest);
    event SetBribeFactory(address indexed old, address indexed latest);
    event SetPermissionRegistry(address indexed old, address indexed latest);

    constructor() {}

    function initialize(address __ve, address _tokenHandler, address _gaugeFactory, address _gaugeFactoryCL, 
                        address _pairFactory, address _pairFactoryCL, address _permissionRegistory, address _nfpm) initializer public {
     __Ownable_init();
     __ReentrancyGuard_init();
      _ve = __ve;  
      base = IVotingEscrow(__ve).token();  
      tokenHandler = _tokenHandler;
       permissionRegistry = _permissionRegistory;
      _factoriesData.gaugeFactories.push(_gaugeFactory);
      _factoriesData.gaugeFactories.push(_gaugeFactoryCL);
      _factoriesData.pairFactories.push(_pairFactory);
      _factoriesData.pairFactories.push(_pairFactoryCL);
      nfpm = _nfpm;
    }

    modifier GaugeAdmin() {
        require(IPermissionsRegistry(permissionRegistry).hasRole("GAUGE_ADMIN",msg.sender), 'GAUGE_ADMIN');
        _;
    }

    modifier Governance() {
        require(IPermissionsRegistry(permissionRegistry).hasRole("GOVERNANCE",msg.sender), 'GOVERNANCE');
        _;
    }

    /// @notice Set a new Bribe Factory
    function setBribeFactory(address _bribeFactory) external GaugeAdmin {
        require(_bribeFactory.code.length > 0, "CODELEN");
        require(_bribeFactory != address(0), "ZA");
        bribefactory = _bribeFactory;
        emit SetBribeFactory(bribefactory, _bribeFactory);
    }

    /// @notice Set a new PermissionRegistry
    function setPermissionsRegistry(address _permissionRegistry) external GaugeAdmin {
        require(_permissionRegistry.code.length > 0, "CODELEN");
        require(_permissionRegistry != address(0), "ZA");
        emit SetPermissionRegistry(permissionRegistry, _permissionRegistry);
        permissionRegistry = _permissionRegistry;
    }

    function setVoter(address _voter) external GaugeAdmin{
        require(_voter.code.length > 0, "CODELEN");
        require(_voter != address(0), "ZA");
        voter = _voter;
    }

   
    function getHybraGovernor() external view returns (address){
        return HybraGovernor;
    }

    function setHybraGovernor(address _HybraGovernor) external GaugeAdmin {
        require(_HybraGovernor != address(0), "ZA");
        HybraGovernor = _HybraGovernor;
    }
    
    /* -----------------------------------------------------------------------------
    --------------------------------------------------------------------------------
    --------------------------------------------------------------------------------
                                    GAUGE CREATION
    --------------------------------------------------------------------------------
    --------------------------------------------------------------------------------
    ----------------------------------------------------------------------------- */
    /// @notice create multiple gauges
    function createGauges(address[] memory _pool, uint256[] memory _gaugeTypes) external nonReentrant returns(address[] memory, address[] memory, address[] memory)  {
        require(_pool.length == _gaugeTypes.length, "MISMATCH_LEN");
        require(_pool.length <= 10, "MAXVAL");
        address[] memory _gauge = new address[](_pool.length);
        address[] memory _int = new address[](_pool.length);
        address[] memory _ext = new address[](_pool.length);

        uint256 i = 0;
        for(i; i < _pool.length; i++){
            (_gauge[i], _int[i], _ext[i]) = _createGauge(_pool[i], _gaugeTypes[i]);
        }
        return (_gauge, _int, _ext);
    }

    /// @notice create a gauge  
    function createGauge(address _pool, uint256 _gaugeType) external nonReentrant returns (address _gauge, address _internal_bribe, address _external_bribe)  {
        (_gauge, _internal_bribe, _external_bribe) = _createGauge(_pool, _gaugeType);
    }



    /// @notice create a gauge
    /// @param  _pool       LP address 
    /// @param  _gaugeType  the type of the gauge you want to create
    /// @dev    To create stable/Volatile pair gaugeType = 0, Concentrated liqudity = 1, ...
    ///         Make sure to use the corrcet gaugeType or it will fail

    function _createGauge(address _pool, uint256 _gaugeType) internal returns (address _gauge, address _internal_bribe, address _external_bribe) {
        require(_gaugeType < _factoriesData.pairFactories.length, "GAUGETYPE");
        require(gauges[_pool] == address(0x0), "DNE");
        require(_pool.code.length > 0, "CODELEN");
        bool isPair;
        address _factory = _factoriesData.pairFactories[_gaugeType];
        address _gaugeFactory = _factoriesData.gaugeFactories[_gaugeType];
        require(_factory != address(0), "ZA");
        require(_gaugeFactory != address(0), "ZA");
        

        address tokenA = address(0);
        address tokenB = address(0);
        (tokenA) = IPairInfo(_pool).token0();
        (tokenB) = IPairInfo(_pool).token1();

        // for future implementation add isPair() in factory
        if(_gaugeType == 0){
            isPair = IPairFactory(_factory).isPair(_pool);
        } 
        if(_gaugeType == 1) {
            // removed due to code size
            // require(_pool_hyper == _pool_factory, 'wrong tokens');    
            isPair = true;
        }

        require(ITokenHandler(tokenHandler).isWhitelisted(tokenA) && ITokenHandler(tokenHandler).isWhitelisted(tokenB), "!WHITELISTED");
        require(ITokenHandler(tokenHandler).isConnector(tokenA) || ITokenHandler(tokenHandler).isConnector(tokenB), "!CONNECTOR");
        require(isPair, "!POOL");
        require(tokenA != address(0) && tokenB != address(0), "!TOKENS");

        (_internal_bribe, _external_bribe) = _deployBribes(_pool, tokenA, tokenB, _gaugeType);
        // create gauge
        if(_gaugeType == 0) {
            _gauge = IGaugeFactory(_gaugeFactory).createGauge(base, _ve, _pool, address(this), _internal_bribe, _external_bribe, isPair);
        }
        if(_gaugeType == 1) {
            _gauge = IGaugeFactoryCL(_gaugeFactory).createGauge(base, _ve, _pool, address(this), _internal_bribe, _external_bribe, isPair, nfpm);
            isCLGauge[_gauge] = true;
            ICLPool(_pool).setGaugeAndPositionManager(_gauge, nfpm);
        }
        // approve spending for $the
        IERC20(base).approve(_gauge, type(uint256).max);
        _saveBribeData(_pool, _gauge, _internal_bribe, _external_bribe);
        emit GaugeCreated(_gauge, msg.sender, _internal_bribe, _external_bribe, _pool);
    }

    function _saveBribeData(address _pool, address _gauge, address _internal_bribe, address _external_bribe) private {
        // save data
        internal_bribes[_gauge] = _internal_bribe;
        external_bribes[_gauge] = _external_bribe;
        gauges[_pool] = _gauge;
        poolForGauge[_gauge] = _pool;
        isGauge[_gauge] = true;
        isAlive[_gauge] = true;
        pools.push(_pool);

        // update index
        // todo: below line will go to ve33 rewarder. 
        supplyIndex[_gauge] = index; // new gauges are set to the default global state
    }
    
    function _deployBribes(address _pool, address tokenA, address tokenB, uint256 _gaugeType) private returns (address _internal_bribe, address _external_bribe) 
    {
        // create internal and external bribe
        address _owner = IPermissionsRegistry(permissionRegistry).hybraTeamMultisig();
        string memory _internalType;
        string memory _extrenalType;
        if(_gaugeType == 0) {
            _internalType =  string.concat("Hybra LP Fees: ", IERC20(_pool).symbol() );
            _extrenalType = string.concat("Hybra Bribes: ", IERC20(_pool).symbol() );
        }
        if(_gaugeType == 1) {
            string memory poolStr = addressToString(_pool);
            _internalType = string.concat("Hybra LP Fees: ", poolStr);
            _extrenalType = string.concat("Hybra Bribes: ", poolStr);
        }
        
        _internal_bribe = IBribeFactory(bribefactory).createBribe(_owner, tokenA, tokenB, _internalType);
        _external_bribe = IBribeFactory(bribefactory).createBribe(_owner, tokenA, tokenB, _extrenalType);
    }

    function addressToString(address _addr) internal pure returns (string memory) {
        bytes20 value = bytes20(_addr);
        bytes memory alphabet = "0123456789abcdef";

        bytes memory str = new bytes(42);
        str[0] = '0';
        str[1] = 'x';

        for (uint i = 0; i < 20; i++) {
            str[2 + i * 2] = alphabet[uint8(value[i] >> 4)];
            str[3 + i * 2] = alphabet[uint8(value[i] & 0x0f)];
        }

        return string(str);
    }



    /// @notice notify reward amount for gauge
    /// @dev    the function is called by the minter each epoch. Anyway anyone can top up some extra rewards.
    /// @param  amount  amount to distribute
    function notifyRewardAmount(uint256 amount) external {
        require(msg.sender == minter, "NA");
        IERC20Upgradeable(base).safeTransferFrom(msg.sender, address(this), amount);

        uint256 _ratio = 0;
        uint256 totalWeight = IVoter(voter).totalWeight();
        if(totalWeight > 0) _ratio = amount * 1e18 / Math.max(totalWeight, 1);     // 1e18 adjustment is removed during claim
        if (_ratio > 0) {
            index += _ratio;
        }

        emit NotifyReward(msg.sender, base, amount);
    }

    function distributeFees() external nonReentrant {
        uint256 i = 0;
        uint256 poolsLength = pools.length;
        for (i; i < poolsLength; i++) {
            address _pool = pools[i];
            _distributeFees(_pool);
        }
    }

   function distributeFees(uint256 _start, uint256 _finish) external nonReentrant {
        for (uint256 x = _start; x < _finish; x++) {
            address _pool = pools[x];
            _distributeFees(_pool);
        }
    }


    function _distributeFees(address _pool) internal {
        if (isGauge[gauges[_pool]] && isAlive[gauges[_pool]]){
            if(!isCLGauge[gauges[_pool]]) {
                IGauge(gauges[_pool]).claimFees();
            } else {
                IGaugeCL(gauges[_pool]).claimFees();
            }
        }
    }
    
    /// @notice Distribute the emission for ALL gauges 
    function distributeAll() external nonReentrant {
        
        IMinter(minter).update_period();

        uint256 x = 0;
        uint256 stop = pools.length;
        for (x; x < stop; x++) {
            _distribute(gauges[pools[x]]);
        }
    }

    function distribute(uint256 _start, uint256 _finish) external nonReentrant {
        IMinter(minter).update_period();
        for (uint256 x = _start; x < _finish; x++) {
            _distribute(gauges[pools[x]]);
        }
    }

    /// @notice distribute reward onyl for given gauges
    /// @dev    this function is used in case some distribution fails
    function distribute(address[] memory _gauges) external nonReentrant {
        IMinter(minter).update_period();
        for (uint256 x = 0; x < _gauges.length; x++) {
            _distribute(_gauges[x]);
        }
    }

    /// @notice distribute the emission
    function _distribute(address _gauge) internal {

        uint256 lastTimestamp = gaugesDistributionTimestmap[_gauge];
        uint256 currentTimestamp = HybraTimeLibrary.epochStart(block.timestamp);
        if(lastTimestamp < currentTimestamp){
            _updateForAfterDistribution(_gauge); // should set claimable to 0 if killed

            uint256 _claimable = claimable[_gauge];

            // distribute only if claimable is > 0, currentEpoch != lastepoch and gauge is alive
            if (_claimable > 0 && isAlive[_gauge] && !IGauge(_gauge).emergency()) {
                claimable[_gauge] = 0;
                gaugesDistributionTimestmap[_gauge] = currentTimestamp;
                if(!isCLGauge[_gauge]) {
                    IGauge(_gauge).notifyRewardAmount(base, _claimable);
                } else {
                    IGaugeCL(_gauge).notifyRewardAmount(base, _claimable);
                }
                emit DistributeReward(msg.sender, _gauge, _claimable);
            }
        }
    }


    /* -----------------------------------------------------------------------------
    --------------------------------------------------------------------------------
    --------------------------------------------------------------------------------
                                    HELPERS
    --------------------------------------------------------------------------------
    --------------------------------------------------------------------------------
    ----------------------------------------------------------------------------- */
 
  
    /// @notice update info for gauges
    /// @dev    this function track the gauge index to emit the correct $the amount after the distribution
    function _updateForAfterDistribution(address _gauge) private {
        address _pool = poolForGauge[_gauge];
        //uint256 _supplied = weightsPerEpoch[_time][_pool];
        uint256 _supplied = IVoter(voter).weights(_pool);

        if (_supplied > 0) {
            uint256 _supplyIndex = supplyIndex[_gauge];
            uint256 _index = index; // get global index0 for accumulated distro
            // SupplyIndex will be updated for Killed Gauges as well so we don't need to udpate index while reviving gauge.
            supplyIndex[_gauge] = _index; // update _gauge current position to global position
            uint256 _delta = _index - _supplyIndex; // see if there is any difference that need to be accrued
            if (_delta > 0) {
                uint256 _share = _supplied * _delta / 1e18; // add accrued difference for each supplied token
                if (isAlive[_gauge]) {
                    claimable[_gauge] += _share;
                } else {
                    IERC20Upgradeable(base).safeTransfer(minter, _share); // send rewards back to Minter so they're not stuck in GaugeManager
                }
            }
        } else {
            supplyIndex[_gauge] = index; // new users are set to the default global state
        }
    }

    /* -----------------------------------------------------------------------------
    --------------------------------------------------------------------------------
    --------------------------------------------------------------------------------
                                    GOVERNANCE
    --------------------------------------------------------------------------------
    --------------------------------------------------------------------------------
    ----------------------------------------------------------------------------- */
    

     /// @notice Kill a malicious gauge 
    /// @param  _gauge gauge to kill
    function killGauge(address _gauge) external Governance {
        require(isAlive[_gauge], "DEAD");
        isAlive[_gauge] = false;

        // Return claimable back to minter
        uint256 _claimable = claimable[_gauge];
        if (_claimable > 0) {
            IERC20Upgradeable(base).safeTransfer(minter, _claimable);
        }
        claimable[_gauge] = 0;

        // We shouldn't update totalWeight because if we decrease it other pools will get more emission while in current scenario 
        // emissionAmount of killed gauge will get transferred back to Minter
        // We're decreasing totalWeight in case of Reset functionality while resetting vote from killed gauge.
        //totalWeight = totalWeight - weights[poolForGauge[_gauge]];
        emit GaugeKilled(_gauge);
    }

    /// @notice Revive a malicious gauge 
    /// @param  _gauge gauge to revive
    function reviveGauge(address _gauge) external Governance {
        require(!isAlive[_gauge], "ALIVE");
        require(isGauge[_gauge], 'DEAD');
        isAlive[_gauge] = true;
        emit GaugeRevived(_gauge);
    }



      /// @notice Set a new bribes for a given gauge
    function setNewBribes(address _gauge, address _internal, address _external) external GaugeAdmin {
        require(isGauge[_gauge], "!GAUGE");
        require(_gauge.code.length > 0, "CODELEN");
        _setInternalBribe(_gauge, _internal);
        _setExternalBribe(_gauge, _external);
    }

    /// @notice Set a new internal bribe for a given gauge
    function setInternalBribeFor(address _gauge, address _internal) external GaugeAdmin {
        require(isGauge[_gauge], "!GAUGE");
        _setInternalBribe(_gauge, _internal);
    }

    /// @notice Set a new External bribe for a given gauge
    function setExternalBribeFor(address _gauge, address _external) external GaugeAdmin {
        require(isGauge[_gauge], "!GAUGE");
        _setExternalBribe(_gauge, _external);
    }

    function _setInternalBribe(address _gauge, address _internal) private {
        require(_internal.code.length > 0, "CODELEN");
        emit SetBribeFor(true, internal_bribes[_gauge], _internal, _gauge);
        internal_bribes[_gauge] = _internal;
    }

    function _setExternalBribe(address _gauge, address _external) private {
        require(_external.code.length > 0, "CODELEN");
        emit SetBribeFor(false, internal_bribes[_gauge], _external, _gauge);
        external_bribes[_gauge] = _external;
    }

    /// @notice claim LP gauge rewards
    function claimRewards(address[] memory _gauges, uint8 _redeemType) external {
        for (uint256 i = 0; i < _gauges.length; i++) {
            IGauge(_gauges[i]).getReward(msg.sender, _redeemType);
        }
    }

    /// @notice claim LP gauge rewards
    function claimRewards(address _gauge, uint256[] memory _nftIds, uint8 _redeemType) external {
        for (uint256 i = 0; i < _nftIds.length; i++) {
            IGaugeCL(_gauge).getReward(_nftIds[i], msg.sender, _redeemType);
        }
    }

    function claimAllRewards(address[] memory _gauges, uint256[][] memory _nftIds, uint8 _redeemType) external {
        for (uint256 i = 0; i < _gauges.length; i++) {
            for (uint256 j = 0; j < _nftIds[i].length; j++) {
                IGaugeCL(_gauges[i]).getReward(_nftIds[i][j], msg.sender, _redeemType);
            }
        }
    }

    /// @notice claim bribes rewards given a TokenID
    function claimBribes(address[] memory _bribes, address[][] memory _tokens, uint256 _tokenId) external {
        require(IVotingEscrow(_ve).isApprovedOrOwner(msg.sender, _tokenId), "NAO");
        for (uint256 i = 0; i < _bribes.length; i++) {
            IBribe(_bribes[i]).getReward(_tokenId, _tokens[i]);
        }
    }

    function claimAllBribes(address[] memory _bribes, address[][] memory _tokens, uint256[][] memory _nftIds) external {
        require(_bribes.length == _tokens.length && _bribes.length == _nftIds.length, "Array length mismatch");

        for (uint256 i = 0; i < _bribes.length; i++) {
            for (uint256 j = 0; j < _nftIds[i].length; j++) {
                require(IVotingEscrow(_ve).isApprovedOrOwner(msg.sender, _nftIds[i][j]), "NAO");
                IBribe(_bribes[i]).getReward(_nftIds[i][j], _tokens[i]);
            }
        }
    }

    function fetchInternalBribeFromPool(address _pool) external returns (address) {
        return internal_bribes[gauges[_pool]];
    }

    function fetchExternalBribeFromPool(address _pool) external returns (address) {
        return external_bribes[gauges[_pool]];
    }

    function isGaugeAliveForPool(address _pool) external returns (bool) {
        return isGauge[gauges[_pool]] && isAlive[gauges[_pool]];
    }

        /// @notice Set a new Minter
    function setMinter(address _minter) external GaugeAdmin {
        require(_minter != address(0), "ZA");
        require(_minter.code.length > 0, "CODELEN");
        emit SetMinter(minter, _minter);
        minter = _minter;
    }

    function addGaugeFactory(address _gaugeFactory) external GaugeAdmin {
        VoterFactoryLib.addGaugeFactory(_factoriesData, _gaugeFactory);
    }

    function replaceGaugeFactory(address _gaugeFactory, uint256 _pos) external GaugeAdmin {
        VoterFactoryLib.replaceGaugeFactory(_factoriesData, _gaugeFactory, _pos);
    }

    function removeGaugeFactory(uint256 _pos) external GaugeAdmin {
        VoterFactoryLib.removeGaugeFactory(_factoriesData, _pos);
    }

    function addPairFactory(address _pairFactory) external GaugeAdmin {
        VoterFactoryLib.addPairFactory(_factoriesData, _pairFactory);
    }

    function replacePairFactory(address _pairFactory, uint256 _pos) external GaugeAdmin {
        VoterFactoryLib.replacePairFactory(_factoriesData, _pairFactory, _pos);
    }

    function removePairFactory(uint256 _pos) external GaugeAdmin {
        VoterFactoryLib.removePairFactory(_factoriesData, _pos);
    }
    


}
END OF MAIN TARGET CONTRACT

## SUPPORTING CONTEXT: CONTRACTS, LIBRARIES & INTERFACES
// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

interface IPermissionsRegistry {
    function emergencyCouncil() external view returns(address);
    function hybraTeamMultisig() external view returns(address);
    function hasRole(bytes memory role, address caller) external view returns(bool);
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

interface IGaugeFactory {
    function createGauge(address _rewardToken,address _ve,address _token,address _distribution, address _internal_bribe, address _external_bribe, bool _isPair) external returns (address) ;
    function gauges(uint256 i) external view returns(address);
    function length() external view returns(uint);
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

interface IGauge {
    function notifyRewardAmount(address token, uint amount) external;
    function getReward(address account, address[] memory tokens, uint8 redeemType) external;
    function getReward(address account, uint8 redeemType) external;
    function claimFees() external returns (uint claimed0, uint claimed1);
    function left(address token) external view returns (uint);
    function rewardRate(address _pair) external view returns (uint);
    function balanceOf(address _account) external view returns (uint);
    function isForPair() external view returns (bool);
    function totalSupply() external view returns (uint);
    function earned(address token, address account) external view returns (uint);
    function setGenesisPool(address genesisPool) external;
    function depositsForGenesis(address tokenOwner, uint256 timestamp, uint256 liquidity) external;
    function emergency() external returns (bool);
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
pragma solidity 0.8.13;

interface IPairInfo {

    function token0() external view returns(address);
    function reserve0() external view returns(uint);
    function decimals0() external view returns(uint);
    function token1() external view returns(address);
    function reserve1() external view returns(uint);
    function decimals1() external view returns(uint);
    function isPair(address _pair) external view returns(bool);
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

// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity >=0.4.0;

/// @title FixedPoint128
/// @notice A library for handling binary fixed point numbers, see https://en.wikipedia.org/wiki/Q_(number_format)
library FixedPoint128 {
    uint256 internal constant Q128 = 0x100000000000000000000000000000000;
}

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
pragma solidity 0.8.13;

interface IPairFactory {
    function allPairsLength() external view returns (uint);
    function isPair(address pair) external view returns (bool);
    function allPairs(uint index) external view returns (address);
    function pairCodeHash() external view returns (bytes32);
    function getPair(address tokenA, address token, bool stable) external view returns (address);
    function createPair(address tokenA, address tokenB, bool stable) external returns (address pair);
    function isGenesis(address pair) external view returns (bool);
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
pragma solidity ^0.8.13;


library VotingDelegationLib {
    /// @notice A checkpoint for marking delegated tokenIds from a given timestamp
    struct Checkpoint {
        uint timestamp;
        uint[] tokenIds;
    }

    // A struct that holds all checkpoint data for different accounts.
    // The calling contract will include one instance of this struct in storage.
    struct Data {
        // For each account, store a mapping from checkpoint index to Checkpoint.
        mapping(address => mapping(uint32 => Checkpoint)) checkpoints;
        // For each account, store the number of checkpoints.
        mapping(address => uint32) numCheckpoints;
    }

    struct TokenHelpers {
        function(uint) view returns (address) ownerOfFn;
        function(address) view returns (uint) ownerToNFTokenCountFn;
        function(address, uint) view returns (uint) tokenOfOwnerByIndex;
    }

    uint public constant MAX_DELEGATES = 1024; // avoid too much gas
    /**
     * @notice Returns the checkpoint index to write for an account.
     * If the most recent checkpoint was created in the current timestamp, returns that index.
     * Otherwise, returns the current number of checkpoints (i.e. a new checkpoint index).
     */
    function findCheckpointToWrite(
        Data storage self,
        address account,
        uint256 currentTimestamp
    ) internal view returns (uint32) {
        uint32 n = self.numCheckpoints[account];
        if (n > 0 && self.checkpoints[account][n - 1].timestamp == currentTimestamp) {
            return n - 1;
        } else {
            return n;
        }
    }

    function moveTokenDelegates(
        Data storage self,
        address srcRep,
        address dstRep,
        uint _tokenId,
        function(uint) view returns (address) ownerOfFn
    ) internal {
        if (srcRep != dstRep && _tokenId > 0) {
            if (srcRep != address(0)) {
                uint32 srcRepNum = self.numCheckpoints[srcRep];
                uint[] storage srcRepOld = srcRepNum > 0
                    ? self.checkpoints[srcRep][srcRepNum - 1].tokenIds
                    : self.checkpoints[srcRep][0].tokenIds;
                uint32 nextSrcRepNum = findCheckpointToWrite(self, srcRep, block.timestamp);
                bool _isCheckpointInNewBlock = (srcRepNum > 0) ? (nextSrcRepNum != srcRepNum - 1) : true;
                Checkpoint storage cpSrcRep = self.checkpoints[srcRep][nextSrcRepNum];
                uint[] storage srcRepNew = cpSrcRep.tokenIds;
                cpSrcRep.timestamp = block.timestamp;
                // All the same except _tokenId
                uint256 length = srcRepOld.length;
                for (uint i = 0; i < length;) {
                    uint tId = srcRepOld[i];
                    if(_isCheckpointInNewBlock) {
                        if(ownerOfFn(tId) == srcRep) {
                            srcRepNew.push(tId);
                        }
                        i++;
                    } else {
                        if(ownerOfFn(tId) != srcRep) {
                            srcRepNew[i] = srcRepNew[length -1];
                            srcRepNew.pop();
                            length--;
                        } else {
                            i++;
                        }
                    }
                }
                self.numCheckpoints[srcRep] = nextSrcRepNum + 1;   
            }

            if (dstRep != address(0)) {
                uint32 dstRepNum = self.numCheckpoints[dstRep];
                uint[] storage dstRepOld = dstRepNum > 0
                    ? self.checkpoints[dstRep][dstRepNum - 1].tokenIds
                    : self.checkpoints[dstRep][0].tokenIds;
                uint32 nextDstRepNum = findCheckpointToWrite(self, dstRep, block.timestamp);
                bool _isCheckpointInNewBlock = (dstRepNum > 0) ? (nextDstRepNum != dstRepNum - 1) : true;
                Checkpoint storage cpDstRep = self.checkpoints[dstRep][nextDstRepNum];
                uint[] storage dstRepNew = cpDstRep.tokenIds;
                cpDstRep.timestamp = block.timestamp;
                require(
                    dstRepOld.length + 1 <= MAX_DELEGATES,
                    "tokens>1"
                );
                if(_isCheckpointInNewBlock) {
                    for (uint i = 0; i < dstRepOld.length; i++) {
                        uint tId = dstRepOld[i];
                        dstRepNew.push(tId);
                    }
                }
                dstRepNew.push(_tokenId);
                self.numCheckpoints[dstRep] = nextDstRepNum + 1;
            }
        }
    }

    function _moveAllDelegates(
        Data storage self,
        address owner,
        address srcRep,
        address dstRep,
        TokenHelpers memory tokenHelpers
    ) internal {
        // You can only redelegate what you own
        address _owner = owner;
        Data storage _self = self;
        address _srcRep = srcRep;
        address _dstRep = dstRep;
        TokenHelpers memory _tokenHelper = tokenHelpers;
        if (_srcRep != _dstRep) {
            if (_srcRep != address(0)) {
                uint32 srcRepNum = _self.numCheckpoints[_srcRep];
                uint[] storage srcRepOld = srcRepNum > 0
                    ? _self.checkpoints[_srcRep][srcRepNum - 1].tokenIds
                    : _self.checkpoints[_srcRep][0].tokenIds;
                uint32 nextSrcRepNum = findCheckpointToWrite(_self,_srcRep, block.timestamp);
                bool _isCheckpointInNewBlock = (srcRepNum > 0) ? (nextSrcRepNum != srcRepNum - 1) : true;
                // if(_isCheckpointInNewBlock) {
                Checkpoint storage cpSrcRep = _self.checkpoints[_srcRep][nextSrcRepNum];
                uint[] storage srcRepNew = cpSrcRep.tokenIds;
                cpSrcRep.timestamp = block.timestamp;

                uint256 length = srcRepOld.length;
                for (uint i = 0; i < length;) {
                    uint tId = srcRepOld[i];
                    if(_isCheckpointInNewBlock) {
                        if(_tokenHelper.ownerOfFn(tId) != _owner) {
                            srcRepNew.push(tId);
                        }
                        i++;
                    } else {
                        if(_tokenHelper.ownerOfFn(tId) == _owner) {
                            srcRepNew[i] = srcRepNew[length -1];
                            srcRepNew.pop();
                            length--;
                        } else {
                            i++;
                        }
                    }
                }
                _self.numCheckpoints[_srcRep] = nextSrcRepNum + 1;
            }


            if (_dstRep != address(0)) {
                uint32 dstRepNum = _self.numCheckpoints[_dstRep];
                uint[] storage dstRepOld = dstRepNum > 0
                    ? _self.checkpoints[_dstRep][dstRepNum - 1].tokenIds
                    : _self.checkpoints[_dstRep][0].tokenIds;
                uint32 nextDstRepNum = findCheckpointToWrite(_self,_dstRep, block.timestamp);
                bool _isCheckpointInNewBlock = (dstRepNum > 0) ? (nextDstRepNum != dstRepNum - 1) : true;
                Checkpoint storage cpDstRep = _self.checkpoints[_dstRep][nextDstRepNum];
                uint[] storage dstRepNew = cpDstRep.tokenIds;
                cpDstRep.timestamp = block.timestamp;
                uint ownerTokenCount = _tokenHelper.ownerToNFTokenCountFn(_owner);
                require(
                    dstRepOld.length + ownerTokenCount <= MAX_DELEGATES,
                    "tokens>1"
                );
                if(_isCheckpointInNewBlock) {
                    for (uint i = 0; i < dstRepOld.length; i++) {
                        uint tId = dstRepOld[i];
                        dstRepNew.push(tId);
                    }
                }
                // Plus all that's owned
                for (uint i = 0; i < ownerTokenCount; i++) {
                    uint tId = _tokenHelper.tokenOfOwnerByIndex(_owner,i);
                    dstRepNew.push(tId);
                }
                _self.numCheckpoints[_dstRep] = nextDstRepNum + 1;   
            }
        }
    }

    function getPastVotesIndex(Data storage data, address account, uint timestamp) internal view returns (uint32) {
        uint32 nCheckpoints = data.numCheckpoints[account];
        if (nCheckpoints == 0) {
            return 0;
        }
        // First check most recent balance
        if (data.checkpoints[account][nCheckpoints - 1].timestamp <= timestamp) {
            return (nCheckpoints - 1);
        }

        // Next check implicit zero balance
        if (data.checkpoints[account][0].timestamp > timestamp) {
            return 0;
        }

        uint32 lower = 0;
        uint32 upper = nCheckpoints - 1;
        while (upper > lower) {
            uint32 center = upper - (upper - lower) / 2; // ceil, avoiding overflow
            VotingDelegationLib.Checkpoint storage cp = data.checkpoints[account][center];
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

}
// SPDX-License-Identifier: None
// HybraHole Foundation 2025

pragma solidity 0.8.13;

import {IVotes} from "@openzeppelin/contracts/governance/utils/IVotes.sol";

interface IHybraVotes is IVotes{
}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {IVotingEscrow} from "../interfaces/IVotingEscrow.sol";
import {HybraTimeLibrary} from "./HybraTimeLibrary.sol";

library VotingBalanceLogic {

    struct Data {
        mapping(uint => IVotingEscrow.Point) point_history;
        mapping(uint => uint) user_point_epoch;
        mapping(uint => IVotingEscrow.Point[1000000000]) user_point_history; // user -> Point[user_epoch]
    }

    /// @notice Get the current voting power for `_tokenId`
    /// @dev Adheres to the ERC20 `balanceOf` interface for Aragon compatibility
    /// @param _tokenId NFT for lock
    /// @param _t Epoch time to return voting power at
    /// @return User voting power
    function balanceOfNFT(uint _tokenId, uint _t, 
        Data storage VotingBalanceLogicData
        ) external view returns (uint) {
        uint _epoch = VotingBalanceLogicData.user_point_epoch[_tokenId];
        if (_epoch == 0) {
            return 0;
        } else {
            uint userEpoch = getPastUserPointIndex(_epoch, _tokenId, _t, VotingBalanceLogicData);
            IVotingEscrow.Point memory last_point = VotingBalanceLogicData.user_point_history[_tokenId][userEpoch];
            if (last_point.permanent != 0) {
                return last_point.permanent;
            }
            else {
                last_point.bias -= last_point.slope * int128(int256(_t) - int256(last_point.ts));
                if (last_point.bias < 0) {
                    last_point.bias = 0;
                }
                return uint(int256(last_point.bias));
            }
        }
    }


    function getPastUserPointIndex(uint _epoch, 
    uint _tokenId,
    uint _t,
    Data storage votingBalanceLogicData
    ) internal view returns (uint256){
        uint lower = 0;
        uint upper = _epoch;
        while (upper > lower) {
            uint center = upper - (upper - lower) / 2; // ceil, avoiding overflow
            IVotingEscrow.Point memory userPoint = votingBalanceLogicData.user_point_history[_tokenId][center];
            if (userPoint.ts == _t) {
                return center;
            } else if (userPoint.ts < _t) {
                lower = center;
            } else {
                upper = center - 1;
            }
        }
        return lower;
    }

    /// @notice Measure voting power of `_tokenId` at block height `_block`
    /// @dev Adheres to MiniMe `balanceOfAt` interface: https://github.com/Giveth/minime
    /// @param _tokenId User's wallet NFT
    /// @param _block Block to calculate the voting power at
    /// @return Voting power
    function balanceOfAtNFT(uint _tokenId, 
        uint _block,
        Data storage VotingBalanceLogicData,
        uint epoch
        ) external view returns (uint) {
        // Copying and pasting totalSupply code because Vyper cannot pass by
        // reference yet
        assert(_block <= block.number);

        // Binary search
        uint _min = 0;
        uint _max = VotingBalanceLogicData.user_point_epoch[_tokenId];
        for (uint i = 0; i < 128; ++i) {
            // Will be always enough for 128-bit numbers
            if (_min >= _max) {
                break;
            }
            uint _mid = (_min + _max + 1) / 2;
            if (VotingBalanceLogicData.user_point_history[_tokenId][_mid].blk <= _block) {
                _min = _mid;
            } else {
                _max = _mid - 1;
            }
        }

        IVotingEscrow.Point memory upoint = VotingBalanceLogicData.user_point_history[_tokenId][_min];

        if (upoint.permanent > 0){
            return upoint.permanent;
        }

        uint max_epoch = epoch;
        uint _epoch = _find_block_epoch(_block, max_epoch, VotingBalanceLogicData);
        IVotingEscrow.Point memory point_0 = VotingBalanceLogicData.point_history[_epoch];
        uint d_block = 0;
        uint d_t = 0;
        if (_epoch < max_epoch) {
            IVotingEscrow.Point memory point_1 = VotingBalanceLogicData.point_history[_epoch + 1];
            d_block = point_1.blk - point_0.blk;
            d_t = point_1.ts - point_0.ts;
        } else {
            d_block = block.number - point_0.blk;
            d_t = block.timestamp - point_0.ts;
        }
        uint block_time = point_0.ts;
        if (d_block != 0) {
            block_time += (d_t * (_block - point_0.blk)) / d_block;
        }

        upoint.bias -= upoint.slope * int128(int256(block_time - upoint.ts));
        if (upoint.bias >= 0) {
            return uint(uint128(upoint.bias));
        } else {
            return 0;
        }
    }

    function totalSupplyAt(uint _block, uint epoch,
        Data storage VotingBalanceLogicData,
        mapping(uint => int128) storage slope_changes) public view returns (uint) {
        assert(_block <= block.number);
        uint _epoch = epoch;
        uint target_epoch = _find_block_epoch(_block, _epoch, VotingBalanceLogicData);

        IVotingEscrow.Point memory point = VotingBalanceLogicData.point_history[target_epoch];
        uint dt = 0;
        if (target_epoch < _epoch) {
            IVotingEscrow.Point memory point_next = VotingBalanceLogicData.point_history[target_epoch + 1];
            if (point.blk != point_next.blk) {
                dt = ((_block - point.blk) * (point_next.ts - point.ts)) / (point_next.blk - point.blk);
            }
        } else {
            if (point.blk != block.number) {
                dt = ((_block - point.blk) * (block.timestamp - point.ts)) / (block.number - point.blk);
            }
        }
        // Now dt contains info on how far are we beyond point
        return _supply_at(point, point.ts + dt, slope_changes);

    }

         /// @notice Binary search to estimate timestamp for block number
    /// @param _block Block to find
    /// @param max_epoch Don't go beyond this epoch
    /// @return Approximate timestamp for block
    function _find_block_epoch(uint _block, 
        uint max_epoch,
        Data storage VotingBalanceLogicData
        ) internal view returns (uint) {
        // Binary search
        uint _min = 0;
        uint _max = max_epoch;
        for (uint i = 0; i < 128; ++i) {
            // Will be always enough for 128-bit numbers
            if (_min >= _max) {
                break;
            }
            uint _mid = (_min + _max + 1) / 2;
            if (VotingBalanceLogicData.point_history[_mid].blk <= _block) {
                _min = _mid;
            } else {
                _max = _mid - 1;
            }
        }
        return _min;
    }

    /// @notice Calculate total voting power at some point in the past
    /// @param point The point (bias/slope) to start search from
    /// @param t Time to calculate the total voting power at
    /// @return Total voting power at that time
    function _supply_at(IVotingEscrow.Point memory point, 
        uint t,
        mapping(uint => int128) storage slope_changes) internal view returns (uint) {
        uint WEEK = HybraTimeLibrary.WEEK;
        IVotingEscrow.Point memory last_point = point;
        uint t_i = (last_point.ts / WEEK) * WEEK;
        for (uint i = 0; i < 255; ++i) {
            t_i += WEEK;
            int128 d_slope = 0;
            if (t_i > t) {
                t_i = t;
            } else {
                d_slope = slope_changes[t_i];
            }
            last_point.bias -= last_point.slope * int128(int256(t_i - last_point.ts));
            if (t_i == t) {
                break;
            }
            last_point.slope += d_slope;
            last_point.ts = t_i;
        }

        if (last_point.bias < 0) {
            last_point.bias = 0;
        }
        return uint(uint128(last_point.bias)) + last_point.permanent;
    }

    function getPastGlobalPointIndex(uint _epoch,
        uint _t,
        Data storage VotingBalanceLogicData) internal view returns (uint256){
        uint lower = 0;
        uint upper = _epoch;
        while (upper > lower) {
            uint center = upper - (upper - lower) / 2; // ceil, avoiding overflow
            IVotingEscrow.Point memory point = VotingBalanceLogicData.point_history[center];
            if (point.ts == _t) {
                return center;
            } else if (point.ts < _t) {
                lower = center;
            } else {
                upper = center - 1;
            }
        }
        return lower;
    }

        /// @notice Calculate total voting power
    /// @dev Adheres to the ERC20 `totalSupply` interface for Aragon compatibility
    /// @return Total voting power
    function totalSupplyAtT(uint t, uint epoch,
        mapping(uint => int128) storage slope_changes,
        Data storage VotingBalanceLogicData) external view returns (uint) {
        uint _epoch = epoch;
        if(_epoch == 0) {
            return 0;
        } else {
            uint globalEpoch = getPastGlobalPointIndex(_epoch, t, VotingBalanceLogicData);
            IVotingEscrow.Point memory last_point = VotingBalanceLogicData.point_history[globalEpoch];
            return _supply_at(last_point, t, slope_changes);
        }
    }
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
// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

interface IVeArtProxy {
    function _tokenURI(uint _tokenId, uint _balanceOf, uint _locked_end, uint _value) external pure returns (string memory output);
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

import {IERC721, IERC721Metadata} from "@openzeppelin/contracts/token/ERC721/extensions/IERC721Metadata.sol";
import {IERC721Receiver} from "@openzeppelin/contracts/token/ERC721/IERC721Receiver.sol";
import {IERC20} from "./interfaces/IERC20.sol";
import "./interfaces/IHybra.sol";
import {IHybraVotes} from "./interfaces/IHybraVotes.sol";
import {IVeArtProxy} from "./interfaces/IVeArtProxy.sol";
import {IVotingEscrow} from "./interfaces/IVotingEscrow.sol";
import {IVoter} from "./interfaces/IVoter.sol";
import {HybraTimeLibrary} from "./libraries/HybraTimeLibrary.sol";
import {VotingDelegationLib} from "./libraries/VotingDelegationLib.sol";
import {VotingBalanceLogic} from "./libraries/VotingBalanceLogic.sol";

/// @title Voting Escrow
/// @notice veNFT implementation that escrows ERC-20 tokens in the form of an ERC-721 NFT
/// @notice Votes have a weight depending on time, so that users are committed to the future of (whatever they are voting for)
/// @author Modified from Solidly (https://github.com/solidlyexchange/solidly/blob/master/contracts/ve.sol)
/// @author Modified from Curve (https://github.com/curvefi/curve-dao-contracts/blob/master/contracts/VotingEscrow.vy)
/// @author Modified from Nouns DAO (https://github.com/withtally/my-nft-dao-project/blob/main/contracts/ERC721Checkpointable.sol)
/// @dev Vote weight decays linearly over time. Lock time cannot be more than `MAXTIME` (2 years).
contract VotingEscrow is IERC721, IERC721Metadata, IHybraVotes {
    enum DepositType {
        DEPOSIT_FOR_TYPE,
        CREATE_LOCK_TYPE,
        INCREASE_LOCK_AMOUNT,
        INCREASE_UNLOCK_TIME
    }

    /*//////////////////////////////////////////////////////////////
                                 EVENTS
    //////////////////////////////////////////////////////////////*/

    event Deposit(
        address indexed provider,
        uint tokenId,
        uint value,
        uint indexed locktime,
        DepositType deposit_type,
        uint ts
    );

    event Merge(
        address indexed _sender,
        uint256 indexed _from,
        uint256 indexed _to,
        uint256 _amountFrom,
        uint256 _amountTo,
        uint256 _amountFinal,
        uint256 _locktime,
        uint256 _ts
    );
    event Split(
        uint256 indexed _from,
        uint256 indexed _tokenId1,
        uint256 indexed _tokenId2,
        address _sender,
        uint256 _splitAmount1,
        uint256 _splitAmount2,
        uint256 _locktime,
        uint256 _ts
    );
    
    event MultiSplit(
        uint256 indexed _from,
        uint256[] _newTokenIds,
        address _sender,
        uint256[] _amounts,
        uint256 _locktime,
        uint256 _ts
    );
    
    event MetadataUpdate(uint256 _tokenId);
    event BatchMetadataUpdate(uint256 _fromTokenId, uint256 _toTokenId);

    event Withdraw(address indexed provider, uint tokenId, uint value, uint ts);
    event LockPermanent(address indexed _owner, uint256 indexed _tokenId, uint256 amount, uint256 _ts);
    event UnlockPermanent(address indexed _owner, uint256 indexed _tokenId, uint256 amount, uint256 _ts);
    event Supply(uint prevSupply, uint supply);

    /*//////////////////////////////////////////////////////////////
                               CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    address public immutable token;
    address public voter;
    address public team;
    address public artProxy;
    // address public burnTokenAddress=0x000000000000000000000000000000000000dEaD;

    uint public PRECISISON = 10000;

    /// @dev Mapping of interface id to bool about whether or not it's supported
    mapping(bytes4 => bool) internal supportedInterfaces;
    mapping(uint => bool) internal isPartnerVeNFT;

    /// @dev ERC165 interface ID of ERC165
    bytes4 internal constant ERC165_INTERFACE_ID = 0x01ffc9a7;

    /// @dev ERC165 interface ID of ERC721
    bytes4 internal constant ERC721_INTERFACE_ID = 0x80ac58cd;

    /// @dev ERC165 interface ID of ERC721Metadata
    bytes4 internal constant ERC721_METADATA_INTERFACE_ID = 0x5b5e139f;

    /// @dev Current count of token
    uint internal tokenId;

    uint internal WEEK;

    uint internal MAXTIME;
    int128 internal iMAXTIME;
    IHybra public _hybr;

    // Instance of the library's storage struct
    VotingDelegationLib.Data private cpData;

    VotingBalanceLogic.Data private votingBalanceLogicData;

    /// @notice Contract constructor
    /// @param token_addr `BLACK` token address
    constructor(address token_addr, address art_proxy) {
        token = token_addr;
        voter = msg.sender;
        team = msg.sender;
        artProxy = art_proxy;
        WEEK = HybraTimeLibrary.WEEK;
        MAXTIME = HybraTimeLibrary.MAX_LOCK_DURATION;
        iMAXTIME = int128(int256(HybraTimeLibrary.MAX_LOCK_DURATION));

        votingBalanceLogicData.point_history[0].blk = block.number;
        votingBalanceLogicData.point_history[0].ts = block.timestamp;

        supportedInterfaces[ERC165_INTERFACE_ID] = true;
        supportedInterfaces[ERC721_INTERFACE_ID] = true;
        supportedInterfaces[ERC721_METADATA_INTERFACE_ID] = true;
        _hybr = IHybra(token);

        // mint-ish
        emit Transfer(address(0), address(this), tokenId);
        // burn-ish
        emit Transfer(address(this), address(0), tokenId);
    }

    /*//////////////////////////////////////////////////////////////
                                MODIFIERS
    //////////////////////////////////////////////////////////////*/

    /// @dev reentrancy guard
    uint8 internal constant _not_entered = 1;
    uint8 internal constant _entered = 2;
    uint8 internal _entered_state = 1;
    modifier nonreentrant() {
        require(_entered_state == _not_entered);
        _entered_state = _entered;
        _;
        _entered_state = _not_entered;
    }

    modifier notPartnerNFT(uint256 _tokenId) {
        require(!isPartnerVeNFT[_tokenId], "PNFT");
        _;
    }

    modifier splitAllowed(uint _from) {
        require(canSplit[msg.sender] || canSplit[address(0)], "!SPLIT");
        require(attachments[_from] == 0 && !voted[_from], "ATT");
        require(_isApprovedOrOwner(msg.sender, _from), "NAO");
        _;
    }

    

    /*///////////////////////////////////////////////////////////////
                             METADATA STORAGE
    //////////////////////////////////////////////////////////////*/

    string constant public name = "veHYBR";
    string constant public symbol = "veHYBR";
    string constant public version = "1.0.0";
    uint8 constant public decimals = 18;

    function setTeam(address _team) external {
        require(msg.sender == team);
        team = _team;
    }

    function setArtProxy(address _proxy) external {
        require(msg.sender == team);
        artProxy = _proxy;
        emit BatchMetadataUpdate(0, type(uint256).max);
    }

    /// @param _tokenId The token ID to modify
    /// @param _isPartner Whether this should be a partner veNFT
    function setPartnerVeNFT(uint _tokenId, bool _isPartner) external {
        require(msg.sender == team, "NA");
        require(idToOwner[_tokenId] != address(0), "DNE");
        isPartnerVeNFT[_tokenId] = _isPartner;
    }

    /// @dev Returns current token URI metadata
    /// @param _tokenId Token ID to fetch URI for.
    function tokenURI(uint _tokenId) external view returns (string memory) {
        require(idToOwner[_tokenId] != address(0), "DNE");
        IVotingEscrow.LockedBalance memory _locked = locked[_tokenId];
        
        return IVeArtProxy(artProxy)._tokenURI(_tokenId,VotingBalanceLogic.balanceOfNFT(_tokenId, block.timestamp, votingBalanceLogicData),_locked.end,uint(int256(_locked.amount)));
    }

    /*//////////////////////////////////////////////////////////////
                      ERC721 BALANCE/OWNER STORAGE
    //////////////////////////////////////////////////////////////*/

    /// @dev Mapping from NFT ID to the address that owns it.
    mapping(uint => address) internal idToOwner;

    /// @dev Mapping from owner address to count of his tokens.
    mapping(address => uint) internal ownerToNFTokenCount;

    /// @dev Returns the address of the owner of the NFT.
    /// @param _tokenId The identifier for an NFT.
    function ownerOf(uint _tokenId) public view returns (address) {
        return idToOwner[_tokenId];
    }

    function ownerToNFTokenCountFn(address owner) public view returns (uint) {
        
        return ownerToNFTokenCount[owner];
    }

    /// @dev Returns the number of NFTs owned by `_owner`.
    ///      Throws if `_owner` is the zero address. NFTs assigned p to the zero address are considered invalid.
    /// @param _owner Address for whom to query the balance.
    function _balance(address _owner) internal view returns (uint) {
        return ownerToNFTokenCount[_owner];
    }

    /// @dev Returns the number of NFTs owned by `_owner`.
    ///      Throws if `_owner` is the zero address. NFTs assigned to the zero address are considered invalid.
    /// @param _owner Address for whom to query the balance.
    function balanceOf(address _owner) external view returns (uint) {
        return _balance(_owner);
    }

    /*//////////////////////////////////////////////////////////////
                         ERC721 APPROVAL STORAGE
    //////////////////////////////////////////////////////////////*/

    /// @dev Mapping from NFT ID to approved address.
    mapping(uint => address) internal idToApprovals;

    /// @dev Mapping from owner address to mapping of operator addresses.
    mapping(address => mapping(address => bool)) internal ownerToOperators;

    mapping(uint => uint) public ownership_change;

    /// @dev Get the approved address for a single NFT.
    /// @param _tokenId ID of the NFT to query the approval of.
    function getApproved(uint _tokenId) external view returns (address) {
        return idToApprovals[_tokenId];
    }

    /// @dev Checks if `_operator` is an approved operator for `_owner`.
    /// @param _owner The address that owns the NFTs.
    /// @param _operator The address that acts on behalf of the owner.
    function isApprovedForAll(address _owner, address _operator) external view returns (bool) {
        return (ownerToOperators[_owner])[_operator];
    }

    /*//////////////////////////////////////////////////////////////
                              ERC721 LOGIC
    //////////////////////////////////////////////////////////////*/

    /// @dev Set or reaffirm the approved address for an NFT. The zero address indicates there is no approved address.
    ///      Throws unless `msg.sender` is the current NFT owner, or an authorized operator of the current owner.
    ///      Throws if `_tokenId` is not a valid NFT. (NOTE: This is not written the EIP)
    ///      Throws if `_approved` is the current owner. (NOTE: This is not written the EIP)
    /// @param _approved Address to be approved for the given NFT ID.
    /// @param _tokenId ID of the token to be approved.
    function approve(address _approved, uint _tokenId) public {
        address owner = idToOwner[_tokenId];
        // Throws if `_tokenId` is not a valid NFT
        require(owner != address(0), "ZA");
        // Throws if `_approved` is the current owner
        require(_approved != owner, "IA");
        // Check requirements
        bool senderIsOwner = (idToOwner[_tokenId] == msg.sender);
        bool senderIsApprovedForAll = (ownerToOperators[owner])[msg.sender];
        require(senderIsOwner || senderIsApprovedForAll, "NAO");
        // Set the approval
        idToApprovals[_tokenId] = _approved;
        emit Approval(owner, _approved, _tokenId);
    }

    /// @dev Enables or disables approval for a third party ("operator") to manage all of
    ///      `msg.sender`'s assets. It also emits the ApprovalForAll event.
    ///      Throws if `_operator` is the `msg.sender`. (NOTE: This is not written the EIP)
    /// @notice This works even if sender doesn't own any tokens at the time.
    /// @param _operator Address to add to the set of authorized operators.
    /// @param _approved True if the operators is approved, false to revoke approval.
    function setApprovalForAll(address _operator, bool _approved) external {
        // Throws if `_operator` is the `msg.sender`
        assert(_operator != msg.sender);
        ownerToOperators[msg.sender][_operator] = _approved;
        emit ApprovalForAll(msg.sender, _operator, _approved);
    }

    /* TRANSFER FUNCTIONS */
    /// @dev Clear an approval of a given address
    ///      Throws if `_owner` is not the current owner.
    function _clearApproval(address _owner, uint _tokenId) internal {
        // Throws if `_owner` is not the current owner
        assert(idToOwner[_tokenId] == _owner);
        if (idToApprovals[_tokenId] != address(0)) {
            // Reset approvals
            idToApprovals[_tokenId] = address(0);
        }
    }

    /// @dev Returns whether the given spender can transfer a given token ID
    /// @param _spender address of the spender to query
    /// @param _tokenId uint ID of the token to be transferred
    /// @return bool whether the msg.sender is approved for the given token ID, is an operator of the owner, or is the owner of the token
    function _isApprovedOrOwner(address _spender, uint _tokenId) internal view returns (bool) {
        address owner = idToOwner[_tokenId];
        bool spenderIsOwner = owner == _spender;
        bool spenderIsApproved = _spender == idToApprovals[_tokenId];
        bool spenderIsApprovedForAll = (ownerToOperators[owner])[_spender];
        return spenderIsOwner || spenderIsApproved || spenderIsApprovedForAll;
    }

    function isApprovedOrOwner(address _spender, uint _tokenId) external view returns (bool) {
        return _isApprovedOrOwner(_spender, _tokenId);
    }

    /// @dev Exeute transfer of a NFT.
    ///      Throws unless `msg.sender` is the current owner, an authorized operator, or the approved
    ///      address for this NFT. (NOTE: `msg.sender` not allowed in internal function so pass `_sender`.)
    ///      Throws if `_to` is the zero address.
    ///      Throws if `_from` is not the current owner.
    ///      Throws if `_tokenId` is not a valid NFT.
    function _transferFrom(
        address _from,
        address _to,
        uint _tokenId,
        address _sender
    ) internal notPartnerNFT(_tokenId) {
        require(attachments[_tokenId] == 0 && !voted[_tokenId], "ATT");
        // Check requirements
        require(_isApprovedOrOwner(_sender, _tokenId), "NAO");

        // Clear approval. Throws if `_from` is not the current owner
        _clearApproval(_from, _tokenId);
        // Remove NFT. Throws if `_tokenId` is not a valid NFT
        _removeTokenFrom(_from, _tokenId);
        // auto re-delegate
        VotingDelegationLib.moveTokenDelegates(cpData, delegates(_from), delegates(_to), _tokenId, ownerOf);
        // Add NFT
        _addTokenTo(_to, _tokenId);
        // Set the block of ownership transfer (for Flash NFT protection)
        ownership_change[_tokenId] = block.number;

       
        // Log the transfer
        emit Transfer(_from, _to, _tokenId);
    }

    /// @dev Throws unless `msg.sender` is the current owner, an authorized operator, or the approved address for this NFT.
    ///      Throws if `_from` is not the current owner.
    ///      Throws if `_to` is the zero address.
    ///      Throws if `_tokenId` is not a valid NFT.
    /// @notice The caller is responsible to confirm that `_to` is capable of receiving NFTs or else
    ///        they maybe be permanently lost.
    /// @param _from The current owner of the NFT.
    /// @param _to The new owner.
    /// @param _tokenId The NFT to transfer.
    function transferFrom(
        address _from,
        address _to,
        uint _tokenId
    ) external {
        _transferFrom(_from, _to, _tokenId, msg.sender);
    }

    /// @dev Transfers the ownership of an NFT from one address to another address.
    ///      Throws unless `msg.sender` is the current owner, an authorized operator, or the
    ///      approved address for this NFT.
    ///      Throws if `_from` is not the current owner.
    ///      Throws if `_to` is the zero address.
    ///      Throws if `_tokenId` is not a valid NFT.
    ///      If `_to` is a smart contract, it calls `onERC721Received` on `_to` and throws if
    ///      the return value is not `bytes4(keccak256("onERC721Received(address,address,uint,bytes)"))`.
    /// @param _from The current owner of the NFT.
    /// @param _to The new owner.
    /// @param _tokenId The NFT to transfer.
    function safeTransferFrom(
        address _from,
        address _to,
        uint _tokenId
    ) external {
        safeTransferFrom(_from, _to, _tokenId, "");
    }

    function _isContract(address account) internal view returns (bool) {
        // This method relies on extcodesize, which returns 0 for contracts in
        // construction, since the code is only stored at the end of the
        // constructor execution.
        uint size;
        assembly {
            size := extcodesize(account)
        }
        return size > 0;
    }

    /// @dev Transfers the ownership of an NFT from one address to another address.
    ///      Throws unless `msg.sender` is the current owner, an authorized operator, or the
    ///      approved address for this NFT.
    ///      Throws if `_from` is not the current owner.
    ///      Throws if `_to` is the zero address.
    ///      Throws if `_tokenId` is not a valid NFT.
    ///      If `_to` is a smart contract, it calls `onERC721Received` on `_to` and throws if
    ///      the return value is not `bytes4(keccak256("onERC721Received(address,address,uint,bytes)"))`.
    /// @param _from The current owner of the NFT.
    /// @param _to The new owner.
    /// @param _tokenId The NFT to transfer.
    /// @param _data Additional data with no specified format, sent in call to `_to`.
    function safeTransferFrom(
        address _from,
        address _to,
        uint _tokenId,
        bytes memory _data
    ) public {
        _transferFrom(_from, _to, _tokenId, msg.sender);

        if (_isContract(_to)) {
            // Throws if transfer destination is a contract which does not implement 'onERC721Received'
            try IERC721Receiver(_to).onERC721Received(msg.sender, _from, _tokenId, _data) returns (bytes4 response) {
                if (response != IERC721Receiver(_to).onERC721Received.selector) {
                    revert("E721_RJ");
                }
            } catch (bytes memory reason) {
                if (reason.length == 0) {
                    revert('E721_NRCV');
                } else {
                    assembly {
                        revert(add(32, reason), mload(reason))
                    }
                }
            }
        }
    }

    /*//////////////////////////////////////////////////////////////
                              ERC165 LOGIC
    //////////////////////////////////////////////////////////////*/

    /// @dev Interface identification is specified in ERC-165.
    /// @param _interfaceID Id of the interface
    function supportsInterface(bytes4 _interfaceID) external view returns (bool) {
        return supportedInterfaces[_interfaceID];
    }

    /*//////////////////////////////////////////////////////////////
                        INTERNAL MINT/BURN LOGIC
    //////////////////////////////////////////////////////////////*/

    /// @dev Mapping from owner address to mapping of index to tokenIds
    mapping(address => mapping(uint => uint)) internal ownerToNFTokenIdList;

    /// @dev Mapping from NFT ID to index of owner
    mapping(uint => uint) internal tokenToOwnerIndex;

    /// @dev  Get token by index
    function tokenOfOwnerByIndex(address _owner, uint _tokenIndex) public view returns (uint) {
        return ownerToNFTokenIdList[_owner][_tokenIndex];
    }

    /// @dev Add a NFT to an index mapping to a given addressndashushun
    /// @param _to address of the receiver
    /// @param _tokenId uint ID Of the token to be added
    function _addTokenToOwnerList(address _to, uint _tokenId) internal {
        uint current_count = _balance(_to);

        ownerToNFTokenIdList[_to][current_count] = _tokenId;
        tokenToOwnerIndex[_tokenId] = current_count;
    }

    /// @dev Add a NFT to a given address
    ///      Throws if `_tokenId` is owned by someone.
    function _addTokenTo(address _to, uint _tokenId) internal {
        // Throws if `_tokenId` is owned by someone
        assert(idToOwner[_tokenId] == address(0));
        // Change the owner
        idToOwner[_tokenId] = _to;
        // Update owner token index tracking
        _addTokenToOwnerList(_to, _tokenId);
        // Change count tracking
        ownerToNFTokenCount[_to] += 1;
    }

    /// @dev Function to mint tokens
    ///      Throws if `_to` is zero address.
    ///      Throws if `_tokenId` is owned by someone.
    /// @param _to The address that will receive the minted tokens.
    /// @param _tokenId The token id to mint.
    /// @return A boolean that indicates if the operation was successful.
    function _mint(address _to, uint _tokenId) internal returns (bool) {
        // Throws if `_to` is zero address
        assert(_to != address(0));
        // checkpoint for gov
        VotingDelegationLib.moveTokenDelegates(cpData, address(0), delegates(_to), _tokenId, ownerOf);
        // Add NFT. Throws if `_tokenId` is owned by someone
        _addTokenTo(_to, _tokenId);
        emit Transfer(address(0), _to, _tokenId);
        return true;
    }

    /// @dev Remove a NFT from an index mapping to a given address
    /// @param _from address of the sender
    /// @param _tokenId uint ID Of the token to be removed
    function _removeTokenFromOwnerList(address _from, uint _tokenId) internal {
        // Delete
        uint current_count = _balance(_from) - 1;
        uint current_index = tokenToOwnerIndex[_tokenId];

        if (current_count == current_index) {
            // update ownerToNFTokenIdList
            ownerToNFTokenIdList[_from][current_count] = 0;
            // update tokenToOwnerIndex
            tokenToOwnerIndex[_tokenId] = 0;
        } else {
            uint lastTokenId = ownerToNFTokenIdList[_from][current_count];

            // Add
            // update ownerToNFTokenIdList
            ownerToNFTokenIdList[_from][current_index] = lastTokenId;
            // update tokenToOwnerIndex
            tokenToOwnerIndex[lastTokenId] = current_index;

            // Delete
            // update ownerToNFTokenIdList
            ownerToNFTokenIdList[_from][current_count] = 0;
            // update tokenToOwnerIndex
            tokenToOwnerIndex[_tokenId] = 0;
        }
    }

    /// @dev Remove a NFT from a given address
    ///      Throws if `_from` is not the current owner.
    function _removeTokenFrom(address _from, uint _tokenId) internal {
        // Throws if `_from` is not the current owner
        assert(idToOwner[_tokenId] == _from);
        // Change the owner
        idToOwner[_tokenId] = address(0);
        // Update owner token index tracking
        _removeTokenFromOwnerList(_from, _tokenId);
        // Change count tracking
        ownerToNFTokenCount[_from] -= 1;
    }

    function _burn(uint _tokenId) internal {
        require(_isApprovedOrOwner(msg.sender, _tokenId), "NAO");

        address owner = ownerOf(_tokenId);

        // Clear approval
        delete idToApprovals[_tokenId];
        // Remove token
        //_removeTokenFrom(msg.sender, _tokenId);
        _removeTokenFrom(owner, _tokenId);
        // checkpoint for gov
        VotingDelegationLib.moveTokenDelegates(cpData, delegates(owner), address(0), _tokenId, ownerOf);

        emit Transfer(owner, address(0), _tokenId);
    }

    /*//////////////////////////////////////////////////////////////
                             ESCROW STORAGE
    //////////////////////////////////////////////////////////////*/

    mapping(uint => IVotingEscrow.LockedBalance) public locked;
    uint public permanentLockBalance;
    uint public epoch;
    mapping(uint => int128) public slope_changes; // time -> signed slope change
    uint public supply;
    mapping(address => bool) public canSplit;


    uint internal constant MULTIPLIER = 1 ether;

    /*//////////////////////////////////////////////////////////////
                              ESCROW LOGIC
    //////////////////////////////////////////////////////////////*/

    /// @notice Get the most recently recorded rate of voting power decrease for `_tokenId`
    /// @param _tokenId token of the NFT
    /// @return Value of the slope
    function get_last_user_slope(uint _tokenId) external view returns (int128) {
        uint uepoch = votingBalanceLogicData.user_point_epoch[_tokenId];
        return votingBalanceLogicData.user_point_history[_tokenId][uepoch].slope;
    }

    /// @notice Get the timestamp for checkpoint `_idx` for `_tokenId`
    /// @param _tokenId token of the NFT
    /// @param _idx User epoch number
    /// @return Epoch time of the checkpoint
    function user_point_history(uint _tokenId, uint _idx) external view returns (IVotingEscrow.Point memory) {
        return votingBalanceLogicData.user_point_history[_tokenId][_idx];
    }

    function point_history(uint epoch) external view returns (IVotingEscrow.Point memory) {
        return votingBalanceLogicData.point_history[epoch];
    }

    function user_point_epoch(uint tokenId) external view returns (uint) {
        return votingBalanceLogicData.user_point_epoch[tokenId];
    }

    /// @notice Record global and per-user data to checkpoint
    /// @param _tokenId NFT token ID. No user checkpoint if 0
    /// @param old_locked Pevious locked amount / end lock time for the user
    /// @param new_locked New locked amount / end lock time for the user
    function _checkpoint(
        uint _tokenId,
        IVotingEscrow.LockedBalance memory old_locked,
        IVotingEscrow.LockedBalance memory new_locked
    ) internal {
        IVotingEscrow.Point memory u_old;
        IVotingEscrow.Point memory u_new;
        int128 old_dslope = 0;
        int128 new_dslope = 0;
        uint _epoch = epoch;

        if (_tokenId != 0) {
            u_new.permanent = 0;

            if(new_locked.isPermanent){
                u_new.permanent = uint(int256(new_locked.amount));
            }

            // Calculate slopes and biases
            // Kept at zero when they have to
            if (old_locked.end > block.timestamp && old_locked.amount > 0) {
                u_old.slope = old_locked.amount / iMAXTIME;
                u_old.bias = u_old.slope * int128(int256(old_locked.end - block.timestamp));
            }
            if (new_locked.end > block.timestamp && new_locked.amount > 0) {
                u_new.slope = new_locked.amount / iMAXTIME;
                u_new.bias = u_new.slope * int128(int256(new_locked.end - block.timestamp));
            }

            // Read values of scheduled changes in the slope
            // old_locked.end can be in the past and in the future
            // new_locked.end can ONLY by in the FUTURE unless everything expired: than zeros
            old_dslope = slope_changes[old_locked.end];
            if (new_locked.end != 0) {
                if (new_locked.end == old_locked.end) {
                    new_dslope = old_dslope;
                } else {
                    new_dslope = slope_changes[new_locked.end];
                }
            }
        }

        IVotingEscrow.Point memory last_point = IVotingEscrow.Point({bias: 0, slope: 0, ts: block.timestamp, blk: block.number, permanent: 0});
        if (_epoch > 0) {
            last_point = votingBalanceLogicData.point_history[_epoch];
        }
        uint last_checkpoint = last_point.ts;
        // initial_last_point is used for extrapolation to calculate block number
        // (approximately, for *At methods) and save them
        // as we cannot figure that out exactly from inside the contract
        IVotingEscrow.Point memory initial_last_point = last_point;
        uint block_slope = 0; // dblock/dt
        if (block.timestamp > last_point.ts) {
            block_slope = (MULTIPLIER * (block.number - last_point.blk)) / (block.timestamp - last_point.ts);
        }
        // If last point is already recorded in this block, slope=0
        // But that's ok b/c we know the block in such case

        // Go over weeks to fill history and calculate what the current point is
        {
            uint t_i = (last_checkpoint / WEEK) * WEEK;
            for (uint i = 0; i < 255; ++i) {
                // Hopefully it won't happen that this won't get used in 5 years!
                // If it does, users will be able to withdraw but vote weight will be broken
                t_i += WEEK;
                int128 d_slope = 0;
                if (t_i > block.timestamp) {
                    t_i = block.timestamp;
                } else {
                    d_slope = slope_changes[t_i];
                }
                last_point.bias -= last_point.slope * int128(int256(t_i - last_checkpoint));
                last_point.slope += d_slope;
                if (last_point.bias < 0) {
                    // This can happen
                    last_point.bias = 0;
                }
                if (last_point.slope < 0) {
                    // This cannot happen - just in case
                    last_point.slope = 0;
                }
                last_checkpoint = t_i;
                last_point.ts = t_i;
                last_point.blk = initial_last_point.blk + (block_slope * (t_i - initial_last_point.ts)) / MULTIPLIER;
                _epoch += 1;
                if (t_i == block.timestamp) {
                    last_point.blk = block.number;
                    break;
                } else {
                    votingBalanceLogicData.point_history[_epoch] = last_point;
                }
            }
        }

        epoch = _epoch;
        // Now point_history is filled until t=now

        if (_tokenId != 0) {
            // If last point was in this block, the slope change has been applied already
            // But in such case we have 0 slope(s)
            last_point.slope += (u_new.slope - u_old.slope);
            last_point.bias += (u_new.bias - u_old.bias);
            if (last_point.slope < 0) {
                last_point.slope = 0;
            }
            if (last_point.bias < 0) {
                last_point.bias = 0;
            }
            last_point.permanent = permanentLockBalance;
        }

        // Record the changed point into history
        votingBalanceLogicData.point_history[_epoch] = last_point;

        if (_tokenId != 0) {
            // Schedule the slope changes (slope is going down)
            // We subtract new_user_slope from [new_locked.end]
            // and add old_user_slope to [old_locked.end]
            if (old_locked.end > block.timestamp) {
                // old_dslope was <something> - u_old.slope, so we cancel that
                old_dslope += u_old.slope;
                if (new_locked.end == old_locked.end) {
                    old_dslope -= u_new.slope; // It was a new deposit, not extension
                }
                slope_changes[old_locked.end] = old_dslope;
            }

            if (new_locked.end > block.timestamp) {
                if (new_locked.end > old_locked.end) {
                    new_dslope -= u_new.slope; // old slope disappeared at this point
                    slope_changes[new_locked.end] = new_dslope;
                }
                // else: we recorded it already in old_dslope
            }
            // Now handle user history
            uint user_epoch = votingBalanceLogicData.user_point_epoch[_tokenId] + 1;

            votingBalanceLogicData.user_point_epoch[_tokenId] = user_epoch;
            u_new.ts = block.timestamp;
            u_new.blk = block.number;
            votingBalanceLogicData.user_point_history[_tokenId][user_epoch] = u_new;
        }
    }

    /// @notice Deposit and lock tokens for a user
    /// @param _tokenId NFT that holds lock
    /// @param _value Amount to deposit
    /// @param unlock_time New time when to unlock the tokens, or 0 if unchanged
    /// @param locked_balance Previous locked amount / timestamp
    /// @param deposit_type The type of deposit
    function _deposit_for(
        uint _tokenId,
        uint _value,
        uint unlock_time,
        IVotingEscrow.LockedBalance memory locked_balance,
        DepositType deposit_type
    ) internal {
        IVotingEscrow.LockedBalance memory _locked = locked_balance;
        uint supply_before = supply;

        supply = supply_before + _value;
        IVotingEscrow.LockedBalance memory old_locked;
        (old_locked.amount, old_locked.end, old_locked.isPermanent) = (_locked.amount, _locked.end, _locked.isPermanent);
        // Adding to existing lock, or if a lock is expired - creating a new one
        _locked.amount += int128(int256(_value));
           
        if (unlock_time != 0) {
            _locked.end = unlock_time;
        }
        locked[_tokenId] = _locked;

        // Possibilities:
        // Both old_locked.end could be current or expired (>/< block.timestamp)
        // value == 0 (extend lock) or value > 0 (add to lock or extend lock)
        // _locked.end > block.timestamp (always)
        _checkpoint(_tokenId, old_locked, _locked);

        address from = msg.sender;
        if (_value != 0) {
            assert(IERC20(token).transferFrom(from, address(this), _value));
        }

        emit Deposit(from, _tokenId, _value, _locked.end, deposit_type, block.timestamp);
        emit Supply(supply_before, supply_before + _value);
    }

    /// @notice Record global data to checkpoint
    function checkpoint() external {
        _checkpoint(0, IVotingEscrow.LockedBalance(0, 0, false), IVotingEscrow.LockedBalance(0, 0, false));
    }

    /// @notice Deposit `_value` tokens for `_tokenId` and add to the lock
    /// @dev Anyone (even a smart contract) can deposit for someone else, but
    ///      cannot extend their locktime and deposit for a brand new user
    /// @param _tokenId lock NFT
    /// @param _value Amount to add to user's lock
    function deposit_for(uint _tokenId, uint _value) external nonreentrant {
        IVotingEscrow.LockedBalance memory _locked = locked[_tokenId];

        require(_value > 0, "ZV"); // dev: need non-zero value
        require(_locked.amount > 0, 'ZL');
        require(_locked.end > block.timestamp || _locked.isPermanent, 'EXP');

        if (_locked.isPermanent) permanentLockBalance += _value;

        _deposit_for(_tokenId, _value, 0, _locked, DepositType.DEPOSIT_FOR_TYPE);
            
        if(voted[_tokenId]) {
            IVoter(voter).poke(_tokenId);
        }
    }

    /// @notice Deposit `_value` tokens for `_to` and lock for `_lock_duration`
    /// @param _value Amount to deposit
    /// @param _lock_duration Number of seconds to lock tokens for (rounded down to nearest week)
    /// @param _to Address to deposit
    function _create_lock(uint _value, uint _lock_duration, address _to) internal returns (uint) {
        uint unlock_time = (block.timestamp + _lock_duration) / WEEK * WEEK; // Locktime is rounded down to weeks

        require(_value > 0, "ZV"); // dev: need non-zero value
        require(unlock_time > block.timestamp && (unlock_time <= block.timestamp + MAXTIME), 'IUT');

        ++tokenId;
        uint _tokenId = tokenId;
        _mint(_to, _tokenId);

        IVotingEscrow.LockedBalance memory _locked = locked[_tokenId];

        _deposit_for(_tokenId, _value, unlock_time, _locked, DepositType.CREATE_LOCK_TYPE);
        return _tokenId;
    }

    /// @notice Deposit `_value` tokens for `msg.sender` and lock for `_lock_duration`
    /// @param _value Amount to deposit
    /// @param _lock_duration Number of seconds to lock tokens for (rounded down to nearest week)
    function create_lock(uint _value, uint _lock_duration) external nonreentrant returns (uint) {
        return _create_lock(_value, _lock_duration, msg.sender);
    }

    /// @notice Deposit `_value` tokens for `_to` and lock for `_lock_duration`
    /// @param _value Amount to deposit
    /// @param _lock_duration Number of seconds to lock tokens for (rounded down to nearest week)
    /// @param _to Address to deposit
    function create_lock_for(uint _value, uint _lock_duration, address _to) external nonreentrant returns (uint) {
        return _create_lock(_value, _lock_duration, _to);
    }

    /// @notice Deposit `_value` additional tokens for `_tokenId` without modifying the unlock time
    /// @param _value Amount of tokens to deposit and add to the lock
    function increase_amount(uint _tokenId, uint _value) external nonreentrant {
        assert(_isApprovedOrOwner(msg.sender, _tokenId));

        IVotingEscrow.LockedBalance memory _locked = locked[_tokenId];

        assert(_value > 0); // dev: need non-zero value
        require(_locked.amount > 0, 'ZL');
        require(_locked.end > block.timestamp || _locked.isPermanent, 'EXP');
        
        if (_locked.isPermanent) permanentLockBalance += _value;
        _deposit_for(_tokenId, _value, 0, _locked, DepositType.INCREASE_LOCK_AMOUNT);

        // poke for the gained voting power 
        if(voted[_tokenId]) {
            IVoter(voter).poke(_tokenId);
        }
        emit MetadataUpdate(_tokenId);
    }

    /// @notice Extend the unlock time for `_tokenId`
    /// @param _lock_duration New number of seconds until tokens unlock
    function increase_unlock_time(uint _tokenId, uint _lock_duration) external nonreentrant {
        assert(_isApprovedOrOwner(msg.sender, _tokenId));

        IVotingEscrow.LockedBalance memory _locked = locked[_tokenId];
        require(!_locked.isPermanent, "!NORM");
        uint unlock_time = (block.timestamp + _lock_duration) / WEEK * WEEK; // Locktime is rounded down to weeks

        require(_locked.end > block.timestamp && _locked.amount > 0, 'EXP||ZV');
        require(unlock_time > _locked.end && (unlock_time <= block.timestamp + MAXTIME), 'IUT'); // IUT -> invalid unlock time

        _deposit_for(_tokenId, 0, unlock_time, _locked, DepositType.INCREASE_UNLOCK_TIME);

        // poke for the gained voting power 
        if(voted[_tokenId]) {
            IVoter(voter).poke(_tokenId);
        }
        emit MetadataUpdate(_tokenId);
    }


    /// @notice Withdraw all tokens for `_tokenId`
    /// @dev Only possible if the lock has expired
    function withdraw(uint _tokenId) external nonreentrant {
        assert(_isApprovedOrOwner(msg.sender, _tokenId));
        require(attachments[_tokenId] == 0 && !voted[_tokenId], "ATT");

        IVotingEscrow.LockedBalance memory _locked = locked[_tokenId];
        require(!_locked.isPermanent, "!NORM");
        require(block.timestamp >= _locked.end, "!EXP");
        uint value = uint(int256(_locked.amount));

        locked[_tokenId] = IVotingEscrow.LockedBalance(0, 0, false);
        uint supply_before = supply;
        supply = supply_before - value;

        // old_locked can have either expired <= timestamp or zero end
        // _locked has only 0 end
        // Both can have >= 0 amount
        _checkpoint(_tokenId, _locked, IVotingEscrow.LockedBalance(0, 0, false));

        assert(IERC20(token).transfer(msg.sender, value));

        // Burn the NFT
        _burn(_tokenId);

        emit Withdraw(msg.sender, _tokenId, value, block.timestamp);
        emit Supply(supply_before, supply_before - value);
    }

    function lockPermanent(uint _tokenId) external {
        address sender = msg.sender;
        require(_isApprovedOrOwner(sender, _tokenId), "NAO");
        
        IVotingEscrow.LockedBalance memory _newLocked = locked[_tokenId];
        require(!_newLocked.isPermanent, "!NORM");
        require(_newLocked.end > block.timestamp, "EXP");
        require(_newLocked.amount > 0, "ZV");

        uint _amount = uint(int256(_newLocked.amount));
        permanentLockBalance += _amount;
        _newLocked.end = 0;
        _newLocked.isPermanent = true;
        _checkpoint(_tokenId, locked[_tokenId], _newLocked);
        locked[_tokenId] = _newLocked;
        if(voted[_tokenId]) {
            IVoter(voter).poke(_tokenId);
        }
        emit LockPermanent(sender, _tokenId, _amount, block.timestamp);
        emit MetadataUpdate(_tokenId);
    }

    function unlockPermanent(uint _tokenId) external {
        address sender = msg.sender;
        require(_isApprovedOrOwner(msg.sender, _tokenId), "NAO");

        require(attachments[_tokenId] == 0 && !voted[_tokenId], "ATT");
        IVotingEscrow.LockedBalance memory _newLocked = locked[_tokenId];
        require(_newLocked.isPermanent, "!NORM");
        uint _amount = uint(int256(_newLocked.amount));
        permanentLockBalance -= _amount;
        _newLocked.end = ((block.timestamp + MAXTIME) / WEEK) * WEEK;
        _newLocked.isPermanent = false;

        _checkpoint(_tokenId, locked[_tokenId], _newLocked);
        locked[_tokenId] = _newLocked;

        emit UnlockPermanent(sender, _tokenId, _amount, block.timestamp);
        emit MetadataUpdate(_tokenId);
    }


    /*///////////////////////////////////////////////////////////////
                           GAUGE VOTING STORAGE
    //////////////////////////////////////////////////////////////*/

    // The following ERC20/minime-compatible methods are not real balanceOf and supply!
    // They measure the weights for the purpose of voting, so they don't represent
    // real coins.

    function balanceOfNFT(uint _tokenId) external view returns (uint) {
        if (ownership_change[_tokenId] == block.number) return 0;
        return VotingBalanceLogic.balanceOfNFT(_tokenId, block.timestamp, votingBalanceLogicData);
    }

    function balanceOfNFTAt(uint _tokenId, uint _t) external view returns (uint) {
        return VotingBalanceLogic.balanceOfNFT(_tokenId, _t, votingBalanceLogicData);
    }

    function balanceOfAtNFT(uint _tokenId, uint _block) external view returns (uint) {
        return VotingBalanceLogic.balanceOfAtNFT(_tokenId, _block, votingBalanceLogicData, epoch);
    }

    /// @notice Calculate total voting power at some point in the past
    /// @param _block Block to calculate the total voting power at
    /// @return Total voting power at `_block`
    function totalSupplyAt(uint _block) external view returns (uint) {
        return VotingBalanceLogic.totalSupplyAt(_block, epoch, votingBalanceLogicData, slope_changes);
    }

    function totalSupply() external view returns (uint) {
        return totalSupplyAtT(block.timestamp);
    }

    /// @notice Calculate total voting power
    /// @dev Adheres to the ERC20 `totalSupply` interface for Aragon compatibility
    /// @return Total voting power
    function totalSupplyAtT(uint t) public view returns (uint) {
        return VotingBalanceLogic.totalSupplyAtT(t, epoch, slope_changes,  votingBalanceLogicData);
    }


    /*///////////////////////////////////////////////////////////////
                            GAUGE VOTING LOGIC
    //////////////////////////////////////////////////////////////*/

    mapping(uint => uint) public attachments;
    mapping(uint => bool) public voted;

    function setVoter(address _voter) external {
        require(msg.sender == team);
        voter = _voter;
    }



    function voting(uint _tokenId) external {
        require(msg.sender == voter);
        voted[_tokenId] = true;
    }

    function abstain(uint _tokenId) external {
        require(msg.sender == voter, "NA");
        voted[_tokenId] = false;
    }

    function attach(uint _tokenId) external {
        require(msg.sender == voter, "NA");
        attachments[_tokenId] = attachments[_tokenId] + 1;
    }

    function detach(uint _tokenId) external {
        require(msg.sender == voter, "NA");
        attachments[_tokenId] = attachments[_tokenId] - 1;
    }

    function merge(uint _from, uint _to) external nonreentrant notPartnerNFT(_from) {
        require(attachments[_from] == 0 && !voted[_from], "ATT");
        require(_from != _to, "SAME");
        require(_isApprovedOrOwner(msg.sender, _from) && 
        _isApprovedOrOwner(msg.sender, _to), "NAO");

        IVotingEscrow.LockedBalance memory _locked0 = locked[_from];
        IVotingEscrow.LockedBalance memory _locked1 = locked[_to];
        require(_locked1.end > block.timestamp ||  _locked1.isPermanent,"EXP||PERM");
        require(_locked0.isPermanent ? _locked1.isPermanent : true, "!MERGE");
        
        uint value0 = uint(int256(_locked0.amount));
        uint end = _locked0.end >= _locked1.end ? _locked0.end : _locked1.end;

        locked[_from] = IVotingEscrow.LockedBalance(0, 0, false);
        _checkpoint(_from, _locked0, IVotingEscrow.LockedBalance(0, 0, false));
        _burn(_from);

        IVotingEscrow.LockedBalance memory newLockedTo;
        newLockedTo.isPermanent = _locked1.isPermanent;

        if (newLockedTo.isPermanent){
            newLockedTo.amount = _locked1.amount + _locked0.amount;
            if (!_locked0.isPermanent) {  // Only add if source wasn't already permanent
                permanentLockBalance += value0;
            }
        }else{
            newLockedTo.amount = _locked1.amount + _locked0.amount;
            newLockedTo.end = end;
        }

        //_checkpointDelegatee(_delegates[_to], value0, true);
        _checkpoint(_to, _locked1, newLockedTo);
        locked[_to] = newLockedTo;

        if(voted[_to]) {
            IVoter(voter).poke(_to);
        }
        emit Merge(
            msg.sender,
            _from,
            _to,
            uint(int256(_locked0.amount)),
            uint(int256(_locked1.amount)),
            uint(int256(newLockedTo.amount)),
            newLockedTo.end,
            block.timestamp
        );
        emit MetadataUpdate(_to);
    }


    // function split(
    //     uint _from,
    //     uint _amount
    // ) external nonreentrant splitAllowed(_from) notPartnerNFT(_from) returns (uint256 _tokenId1, uint256 _tokenId2) {
    //     address owner = idToOwner[_from];
        

    //     IVotingEscrow.LockedBalance memory newLocked = locked[_from];
    //     require(newLocked.end > block.timestamp || newLocked.isPermanent, "EXP");
        
    //     int128 _splitAmount = int128(int256(_amount));
        
    //     require(_splitAmount != 0, "ZV");
    //     require(newLocked.amount > _splitAmount, "BIGVAL");

    //     locked[_from] = IVotingEscrow.LockedBalance(0, 0, false);
    //     _checkpoint(_from, newLocked, IVotingEscrow.LockedBalance(0, 0, false));
    //     _burn(_from);

    //     newLocked.amount -= _splitAmount;
    //     _tokenId1 = _createSplitNFT(owner, newLocked);

    //     newLocked.amount = _splitAmount;
    //     _tokenId2 = _createSplitNFT(owner, newLocked);

    //     // emit Split(
    //     //     _from,
    //     //     _tokenId1,
    //     //     _tokenId2,
    //     //     msg.sender,
    //     //     uint(int256(locked[_tokenId1].amount)),
    //     //     uint(int256(_splitAmount)),
    //     //     newLocked.end,
    //     //     block.timestamp
    //     // );
    // }

    /// @notice Split a veNFT into multiple new veNFTs with specified weight distribution
    /// @param _from The token ID to split
    /// @param amounts Array of weights for distributing the locked amount
    /// @return newTokenIds Array of newly created token IDs
    function multiSplit(
        uint _from,
        uint[] memory amounts
    ) external nonreentrant splitAllowed(_from) notPartnerNFT(_from) returns (uint256[] memory newTokenIds) {
        require(amounts.length >= 2 && amounts.length <= 10, "MIN2MAX10");
        
        address owner = idToOwner[_from];

        
        IVotingEscrow.LockedBalance memory originalLocked = locked[_from];
        require(originalLocked.end > block.timestamp || originalLocked.isPermanent, "EXP");
        require(originalLocked.amount > 0, "ZV");
        
        // Calculate total weight
        uint totalWeight = 0;
        for(uint i = 0; i < amounts.length; i++) {
            require(amounts[i] > 0, "ZW"); // Zero weight not allowed
            totalWeight += amounts[i];
        }
        
        // Burn the original NFT
        locked[_from] = IVotingEscrow.LockedBalance(0, 0, false);
        _checkpoint(_from, originalLocked, IVotingEscrow.LockedBalance(0, 0, false));
        _burn(_from);
        
        // Create new NFTs with proportional amounts
        newTokenIds = new uint256[](amounts.length);
        uint[] memory actualAmounts = new uint[](amounts.length);
        
        for(uint i = 0; i < amounts.length; i++) {
            IVotingEscrow.LockedBalance memory newLocked = IVotingEscrow.LockedBalance({
                amount: int128(int256(uint256(int256(originalLocked.amount)) * amounts[i] / totalWeight)),
                end: originalLocked.end,
                isPermanent: originalLocked.isPermanent
            });
            
            newTokenIds[i] = _createSplitNFT(owner, newLocked);
            actualAmounts[i] = uint256(int256(newLocked.amount));
        }
        
        emit MultiSplit(
            _from,
            newTokenIds,
            msg.sender,
            actualAmounts,
            originalLocked.end,
            block.timestamp
        );
    }

    function _createSplitNFT(address _to, IVotingEscrow.LockedBalance memory _newLocked) private returns (uint256 _tokenId) {
        _tokenId = ++tokenId;
        locked[_tokenId] = _newLocked;
        _checkpoint(_tokenId, IVotingEscrow.LockedBalance(0, 0, false), _newLocked);
        _mint(_to, _tokenId);
    }

    function toggleSplit(address _account, bool _bool) external {
        require(msg.sender == team);
        canSplit[_account] = _bool;
    }

    /*///////////////////////////////////////////////////////////////
                            DAO VOTING STORAGE
    //////////////////////////////////////////////////////////////*/

    /// @notice The EIP-712 typehash for the contract's domain
    bytes32 public constant DOMAIN_TYPEHASH = keccak256("EIP712Domain(string name,uint256 chainId,address verifyingContract)");

    /// @notice The EIP-712 typehash for the delegation struct used by the contract
    bytes32 public constant DELEGATION_TYPEHASH = keccak256("Delegation(address delegatee,uint256 nonce,uint256 expiry)");

    /// @notice A record of each accounts delegate
    mapping(address => address) private _delegates;

    /// @notice A record of states for signing / validating signatures
    mapping(address => uint) public nonces;

    /**
     * @notice Overrides the standard `Comp.sol` delegates mapping to return
     * the delegator's own address if they haven't delegated.
     * This avoids having to delegate to oneself.
     */
    function delegates(address delegator) public view returns (address) {
        address current = _delegates[delegator];
        return current == address(0) ? delegator : current;
    }

    /**
     * @notice Gets the current votes balance for `account`
     * @param account The address to get votes balance
     * @return The number of current votes for `account`
     */
    function getVotes(address account) external view returns (uint) {
        uint32 nCheckpoints = cpData.numCheckpoints[account];
        if (nCheckpoints == 0) {
            return 0;
        }
        uint[] storage _tokenIds = cpData.checkpoints[account][nCheckpoints - 1].tokenIds;
        uint votes = 0;
        for (uint i = 0; i < _tokenIds.length; i++) {
            uint tId = _tokenIds[i];
            votes = votes + VotingBalanceLogic.balanceOfNFT(tId, block.timestamp, votingBalanceLogicData);
        }
        return votes;
    }

    function getPastVotes(address account, uint timestamp)
        public
        view
        returns (uint)
    {
        uint32 _checkIndex = VotingDelegationLib.getPastVotesIndex(cpData, account, timestamp);
        // Sum votes
        uint[] storage _tokenIds = cpData.checkpoints[account][_checkIndex].tokenIds;
        uint votes = 0;
        for (uint i = 0; i < _tokenIds.length; i++) {
            uint tId = _tokenIds[i];
            // Use the provided input timestamp here to get the right decay
            votes = votes + VotingBalanceLogic.balanceOfNFT(tId, timestamp,  votingBalanceLogicData);
        }

        return votes;
    }


    function getPastTotalSupply(uint256 timestamp) external view returns (uint) {
        return totalSupplyAtT(timestamp);
    }


    /*///////////////////////////////////////////////////////////////
                             DAO VOTING LOGIC
    //////////////////////////////////////////////////////////////*/
    function _delegate(address delegator, address delegatee) internal {
        /// @notice differs from `_delegate()` in `Comp.sol` to use `delegates` override method to simulate auto-delegation
        address currentDelegate = delegates(delegator);

        _delegates[delegator] = delegatee;

        emit DelegateChanged(delegator, currentDelegate, delegatee);
        VotingDelegationLib.TokenHelpers memory tokenHelpers = VotingDelegationLib.TokenHelpers({
            ownerOfFn: ownerOf,
            ownerToNFTokenCountFn: ownerToNFTokenCountFn,
            tokenOfOwnerByIndex:tokenOfOwnerByIndex
        });
        VotingDelegationLib._moveAllDelegates(cpData, delegator, currentDelegate, delegatee, tokenHelpers);
    }

    /**
     * @notice Delegate votes from `msg.sender` to `delegatee`
     * @param delegatee The address to delegate votes to
     */
    function delegate(address delegatee) public {
        if (delegatee == address(0)) delegatee = msg.sender;
        return _delegate(msg.sender, delegatee);
    }

    function delegateBySig(
        address delegatee,
        uint nonce,
        uint expiry,
        uint8 v,
        bytes32 r,
        bytes32 s
    ) public {
        require(delegatee != msg.sender, "NA");
        require(delegatee != address(0), "ZA");
        
        bytes32 domainSeparator = keccak256(
            abi.encode(
                DOMAIN_TYPEHASH,
                keccak256(bytes(name)),
                keccak256(bytes(version)),
                block.chainid,
                address(this)
            )
        );
        bytes32 structHash = keccak256(
            abi.encode(DELEGATION_TYPEHASH, delegatee, nonce, expiry)
        );
        bytes32 digest = keccak256(
            abi.encodePacked("\x19\x01", domainSeparator, structHash)
        );
        address signatory = ecrecover(digest, v, r, s);
        require(
            signatory != address(0),
            "ZA"
        );
        require(
            nonce == nonces[signatory]++,
            "!NONCE"
        );
        require(
            block.timestamp <= expiry,
            "EXP"
        );
        return _delegate(signatory, delegatee);
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


import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import '../interfaces/IPermissionsRegistry.sol';
import '../interfaces/IGaugeFactory.sol';
import '../GaugeV2.sol';


interface IGauge{
    function setDistribution(address _distro) external;
    function activateEmergencyMode() external;
    function stopEmergencyMode() external;
    function setInternalBribe(address intbribe) external;
    function setRewarderPid(uint256 pid) external;
    function setGaugeRewarder(address _gr) external;
    function setFeeVault(address _feeVault) external;
    function setGenesisPoolManager(address _genesisManager) external;
}


contract GaugeFactory is IGaugeFactory, OwnableUpgradeable {
    address public last_gauge;
    address public permissionsRegistry;

    address[] internal __gauges;
    address internal rHYBR;
    constructor() {}

    function initialize(address _permissionRegistry) initializer  public {
        __Ownable_init();   //after deploy ownership to multisig
        permissionsRegistry = _permissionRegistry;
    }

    function setRegistry(address _registry) external {
        require(owner() == msg.sender, 'NA');
        permissionsRegistry = _registry;
    }

    function setRHYBR(address _rHYBR) external {
        require(owner() == msg.sender, 'NA');
        rHYBR = _rHYBR;
    }

    function gauges(uint256 i) external view returns(address) {
        return __gauges[i];
    }

    function length() external view returns(uint) {
        return __gauges.length;
    }


    function createGauge(address _rewardToken,address _ve,address _token,address _distribution, address _internal_bribe, address _external_bribe, bool _isPair) external returns (address) {
        last_gauge = address(new GaugeV2(_rewardToken,rHYBR,_ve,_token,_distribution,_internal_bribe,_external_bribe,_isPair) );
        __gauges.push(last_gauge);
        return last_gauge;
    }


    modifier onlyAllowed() {
        require(owner() == msg.sender || IPermissionsRegistry(permissionsRegistry).hasRole("GAUGE_ADMIN",msg.sender), 'GAUGE_ADMIN');
        _;
    }

    modifier EmergencyCouncil() {
        require( msg.sender == IPermissionsRegistry(permissionsRegistry).emergencyCouncil(), "NA");
        _;
    }


    function activateEmergencyMode( address[] memory _gauges) external EmergencyCouncil {
        uint i = 0;
        for ( i ; i < _gauges.length; i++){
            IGauge(_gauges[i]).activateEmergencyMode();
        }
    }

    function stopEmergencyMode( address[] memory _gauges) external EmergencyCouncil {
        uint i = 0;
        for ( i ; i < _gauges.length; i++){
            IGauge(_gauges[i]).stopEmergencyMode();
        }
    }

 

    function setGaugeRewarder( address[] memory _gauges, address[] memory _rewarder) external onlyAllowed {
        require(_gauges.length == _rewarder.length, "EXACT_LEN");
        uint i = 0;
        for ( i ; i < _gauges.length; i++){
            IGauge(_gauges[i]).setGaugeRewarder(_rewarder[i]);
        }
    }

    function setDistribution(address[] memory _gauges,  address distro) external onlyAllowed {
        uint i = 0;
        for ( i ; i < _gauges.length; i++){
            IGauge(_gauges[i]).setDistribution(distro);
        }
    }


    function setInternalBribe(address[] memory _gauges,  address[] memory int_bribe) external onlyAllowed {
        require(_gauges.length == int_bribe.length, "EXACT_LEN");
        uint i = 0;
        for ( i ; i < _gauges.length; i++){
            IGauge(_gauges[i]).setInternalBribe(int_bribe[i]);
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
pragma solidity =0.7.6;

interface IFactoryRegistry {
    function approve(address poolFactory, address votingRewardsFactory, address gaugeFactory) external;

    function isPoolFactoryApproved(address poolFactory) external returns (bool);

    function factoriesToPoolFactory(address poolFactory)
        external
        returns (address votingRewardsFactory, address gaugeFactory);
}

pragma solidity 0.8.13;

library VoterFactoryLib {
    struct Data {
        address[] pairFactories;
        address[] gaugeFactories;
        mapping(address => bool) isFactory;
        mapping(address => bool) isGaugeFactory;
    }

    event AddPairFactories(address indexed pairfactory);
    event AddGaugeFactories(address indexed gaugefactory);
    event SetGaugeFactory(address indexed old, address indexed latest);
    event SetPairFactory(address indexed old, address indexed latest);


    function addPairFactory(Data storage self, address _pairFactory) external {
        require(_pairFactory != address(0) , 'addr0');
        require(!self.isFactory[_pairFactory], "fact");
        require(_pairFactory.code.length > 0, "!contract");
        self.pairFactories.push(_pairFactory);
        self.isFactory[_pairFactory] = true;
        emit AddPairFactories(_pairFactory);
    }

    function addGaugeFactory(Data storage self, address _gaugeFactory) external {
        require(_gaugeFactory != address(0) , 'addr0');
        require(!self.isGaugeFactory[_gaugeFactory], "gFact");
        require(_gaugeFactory.code.length > 0, "!contract");
        self.gaugeFactories.push(_gaugeFactory);
        self.isGaugeFactory[_gaugeFactory] = true;
        emit AddGaugeFactories(_gaugeFactory);
    }

    function replacePairFactory(Data storage self, address _pairFactory, uint256 _pos) external {
        require(_pairFactory != address(0), 'addr0');
        require(!self.isFactory[_pairFactory], 'fact');
        require(_pairFactory.code.length > 0, "!contract");
        address oldPF = self.pairFactories[_pos];
        self.isFactory[oldPF] = false;
        self.pairFactories[_pos] = _pairFactory;
        self.isFactory[_pairFactory] = true;

        emit SetPairFactory(oldPF, _pairFactory);
    }

    function replaceGaugeFactory(Data storage self, address _gaugeFactory, uint256 _pos) external {
        require(_gaugeFactory != address(0) , 'addr0');
        require(!self.isGaugeFactory[_gaugeFactory], 'gFact');
        require(_gaugeFactory.code.length > 0, "!contract");
        address oldGF = self.gaugeFactories[_pos];
        self.isGaugeFactory[oldGF] = false;
        self.gaugeFactories[_pos] = _gaugeFactory;
        self.isGaugeFactory[_gaugeFactory] = true;

        emit SetGaugeFactory(oldGF, _gaugeFactory);
    }

    function removePairFactory(Data storage self, uint256 _pos) external {
        address oldPF = self.pairFactories[_pos];
        require(self.isFactory[oldPF], "!exists");
        self.isFactory[oldPF] = false;
        self.pairFactories[_pos] = address(0);
        emit SetPairFactory(oldPF, address(0));
    }

    function removeGaugeFactory(Data storage self, uint256 _pos) external {
        address oldGF = self.gaugeFactories[_pos];
        require(self.isGaugeFactory[oldGF], "!exists");
        self.isGaugeFactory[oldGF] = false;
        self.gaugeFactories[_pos] = address(0);
        emit SetGaugeFactory(oldGF, address(0));
    }

}
// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

interface IHybra {
    function totalSupply() external view returns (uint);
    function balanceOf(address) external view returns (uint);
    function approve(address spender, uint value) external returns (bool);
    function transfer(address, uint) external returns (bool);
    function transferFrom(address,address,uint) external returns (bool);
    function mint(address, uint) external returns (bool);
    function minter() external returns (address);
    function burn(uint) external returns (bool);
    function burnFrom(address, uint) external returns (bool);
}

// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

interface IBribeFactory {
    function createInternalBribe(address[] memory) external returns (address);
    function createExternalBribe(address[] memory) external returns (address);
    function createBribe(address _owner,address _token0,address _token1, string memory _type) external returns (address);
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

// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";
import "./BaseDeployScript.sol";

import {MinterUpgradeable} from "../contracts/MinterUpgradeable.sol";
import {RewardsDistributor} from "../contracts/RewardsDistributor.sol";
import {GaugeManager} from "../contracts/GaugeManager.sol";
import {GrowthHYBR} from "../contracts/GovernanceHYBR.sol";

contract Deploy3a2_SetupConnections is BaseDeployScript {
    using stdJson for string;
    
    function run() external {
        uint256 deployerKey = vm.envUint("PRIVATE_KEY");
        address deployer = vm.rememberKey(deployerKey);
        
        // Load previous deployments
        string memory tokenPath = getInputPath("Deploy2_TokenSystem");
        string memory factoriesPath = getInputPath("Deploy3a_GaugeFactories");
        string memory minterPath = getInputPath("Deploy3b1_MinterRewards");
        
        string memory tokenJson = vm.readFile(tokenPath);
        string memory factoriesJson = vm.readFile(factoriesPath);
        string memory minterJson = vm.readFile(minterPath);
        
        // Load addresses
        address gHybr = abi.decode(vm.parseJson(tokenJson, ".GrowthHYBR"), (address));
        address gaugeManager = abi.decode(vm.parseJson(factoriesJson, ".GaugeManager"), (address));
        address minter = abi.decode(vm.parseJson(minterJson, ".Minter"), (address));
        address rewardsDistributor = abi.decode(vm.parseJson(minterJson, ".RewardsDistributor"), (address));
        
        console.log("=== Step 2: Setup Contract Connections ===");
        console.log("Deployer:", deployer);
        console.log("Using GrowthHYBR:", gHybr);
        console.log("Using GaugeManager:", gaugeManager);
        console.log("Using Minter:", minter);
        console.log("Using RewardsDistributor:", rewardsDistributor);
        
        vm.startBroadcast(deployer);
        
        // 1. Set Minter on GaugeManager
        console.log("Setting Minter on GaugeManager...");
        GaugeManager(gaugeManager).setMinter(minter);
        console.log("Minter set on GaugeManager");
        
        // 2. Set Minter as depositor on RewardsDistributor
        console.log("Setting Minter as depositor on RewardsDistributor...");
        RewardsDistributor(rewardsDistributor).setDepositor(minter);
        console.log("Minter set as depositor on RewardsDistributor");
        
        // 3. Set RewardsDistributor on GrowthHYBR
        console.log("Setting RewardsDistributor on GovernanceHYBR...");
        GrowthHYBR(gHybr).setRewardsDistributor(rewardsDistributor);
        console.log("RewardsDistributor set on GovernanceHYBR");
        
        // 4. Set GaugeManager on GrowthHYBR
        console.log("Setting GaugeManager on GovernanceHYBR...");
        GrowthHYBR(gHybr).setGaugeManager(gaugeManager);
        console.log("GaugeManager set on GovernanceHYBR");
        
        vm.stopBroadcast();
        
        console.log("=== All Contract Connections Complete ===");
        
        // Create final combined output
       

        console.log("Final addresses saved to:");
    }
}
// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";
import "./BaseDeployScript.sol";

import {MinterUpgradeable} from "../contracts/MinterUpgradeable.sol";
import {VoterV3} from "../contracts/VoterV3.sol";
import {BribeFactoryV3} from "../contracts/factories/BribeFactoryV3.sol";
import {GaugeManager} from "../contracts/GaugeManager.sol";
import {VotingEscrow} from "../contracts/VotingEscrow.sol";

import "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";

contract Deploy3c_Voting is BaseDeployScript {
    using stdJson for string;
    
    function run() external {
        uint256 deployerKey = vm.envUint("PRIVATE_KEY");
        address deployer =  vm.rememberKey(deployerKey);
        
        // Load previous deployments
        string memory infraPath = getInputPath("Deploy1_Infrastructure");
        string memory tokenPath = getInputPath("Deploy2_TokenSystem");
        string memory factoriesPath = getInputPath("Deploy3a_GaugeFactories");
        string memory infraJson = vm.readFile(infraPath);
        string memory tokenJson = vm.readFile(tokenPath);
        string memory factoriesJson = vm.readFile(factoriesPath);
        
        // Load addresses
        address proxyAdmin = abi.decode(vm.parseJson(infraJson, ".ProxyAdmin"), (address));
        address permissionsRegistry = abi.decode(vm.parseJson(infraJson, ".PermissionsRegistry"), (address));
        address tokenHandler = abi.decode(vm.parseJson(infraJson, ".TokenHandler"), (address));
        address votingEscrow = abi.decode(vm.parseJson(tokenJson, ".VotingEscrow"), (address));
        address gaugeManager = abi.decode(vm.parseJson(factoriesJson, ".GaugeManager"), (address));
        
        console.log("=== Deploy Voting System ===");
        console.log("Deployer:", deployer);
        console.log("Using GaugeManager:", gaugeManager);
        console.log("");
        
        vm.startBroadcast(deployer);
        
     
        
        // 2. Deploy VoterV3
        console.log("Deploying VoterV3...");
        VoterV3 voterImpl = new VoterV3();
        TransparentUpgradeableProxy voterProxy = new TransparentUpgradeableProxy(
            address(voterImpl),
            proxyAdmin,
            abi.encodeWithSelector(
                VoterV3.initialize.selector,
                votingEscrow,          // __ve
                tokenHandler,          // _tokenHandler  
                gaugeManager,          // _gaugeManager
                permissionsRegistry    // _permissionRegistry
            )
        );
        VoterV3 voter = VoterV3(address(voterProxy));
        console.log("VoterV3:", address(voter));
        
        // 3. Set Voter in GaugeManager
        console.log("\nSetting Voter in GaugeManager...");
        GaugeManager(gaugeManager).setVoter(address(voter));
        console.log("Voter set in GaugeManager:", address(voter));
        
        // 4. Set Voter in VotingEscrow
        console.log("\nSetting Voter in VotingEscrow...");
        VotingEscrow(votingEscrow).setVoter(address(voter));
        console.log("Voter set in VotingEscrow:", address(voter));
        
        vm.stopBroadcast();
        
        console.log("");
        console.log("=== Voting System Deployment and Setup Complete ===");
        console.log("Voter has been configured in both GaugeManager and VotingEscrow");
        
        // Save to JSON
        string memory path = getOutputPath("Deploy3c_Voting");
        
        string memory json = "";
        json = vm.serializeAddress("voting", "VoterV3", address(voter));
        json = vm.serializeAddress("voting", "Voter", address(voter)); // Also save as "Voter" for backward compatibility
        
        vm.writeJson(json, path);
        console.log("Addresses saved to:", path);
    }
}
// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";
import "./BaseDeployScript.sol";

import {GaugeManager} from "../contracts/GaugeManager.sol";
import {PermissionsRegistry} from "../contracts/PermissionsRegistry.sol";

contract Deploy3a3_SetupPermissions is BaseDeployScript {
    using stdJson for string;
    
    function run() external {
        uint256 deployerKey = vm.envUint("PRIVATE_KEY");
        address deployer = vm.rememberKey(deployerKey);
        
        // Load previous deployments
        string memory infraPath = getInputPath("Deploy1_Infrastructure");
        string memory gaugeManagerPath = getInputPath("Deploy3a2_GaugeManagerAndBribes");
        
        string memory infraJson = vm.readFile(infraPath);
        string memory gaugeManagerJson = vm.readFile(gaugeManagerPath);
        
        address permissionsRegistry = abi.decode(vm.parseJson(infraJson, ".PermissionsRegistry"), (address));
        address gaugeManager = abi.decode(vm.parseJson(gaugeManagerJson, ".GaugeManager"), (address));
        address bribeFactoryV3 = abi.decode(vm.parseJson(gaugeManagerJson, ".BribeFactoryV3"), (address));
        
        console.log("=== Setup Permissions and Final Configuration ===");
        console.log("Deployer:", deployer);
        console.log("Using PermissionsRegistry:", permissionsRegistry);
        console.log("Using GaugeManager:", gaugeManager);
        console.log("Using BribeFactoryV3:", bribeFactoryV3);
        
        vm.startBroadcast(deployer);
        
        // Setup permissions for GaugeManager operations
        console.log("Setting up GAUGE_ADMIN role for deployer...");
        PermissionsRegistry(permissionsRegistry).setRoleFor(deployer, "GAUGE_ADMIN");
        console.log("GAUGE_ADMIN role assigned to deployer");
        
        // Set BribeFactory on GaugeManager
        console.log("Setting BribeFactory on GaugeManager...");
        GaugeManager(gaugeManager).setBribeFactory(bribeFactoryV3);
        console.log("BribeFactory set on GaugeManager");
        
        // Ensure PermissionsRegistry is properly set (redundant but explicit)
        console.log("Confirming PermissionsRegistry on GaugeManager...");
        GaugeManager(gaugeManager).setPermissionsRegistry(permissionsRegistry);
        console.log("PermissionsRegistry confirmed on GaugeManager");
        
        vm.stopBroadcast();
        
        // Create final combined output
        string memory path = getOutputPath("Deploy3a_GaugeFactories");

        // Load all previous outputs to combine
        string memory gaugeFactoriesPath = getInputPath("Deploy3a1_GaugeFactories");
        string memory gaugeFactoriesJson = vm.readFile(gaugeFactoriesPath);
        
        address gaugeFactory = abi.decode(vm.parseJson(gaugeFactoriesJson, ".GaugeFactory"), (address));
        address gaugeFactoryCL = abi.decode(vm.parseJson(gaugeFactoriesJson, ".GaugeFactoryCL"), (address));
        
        string memory json = "";
        json = vm.serializeAddress("factories", "GaugeFactory", gaugeFactory);
        json = vm.serializeAddress("factories", "GaugeFactoryCL", gaugeFactoryCL);
        json = vm.serializeAddress("factories", "GaugeManager", gaugeManager);
        json = vm.serializeAddress("factories", "BribeFactoryV3", bribeFactoryV3);
        
        vm.writeJson(json, path);
        console.log("Complete deployment addresses saved to:", path);
    }
}
// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";

import {BribeFactoryV3} from "../contracts/factories/BribeFactoryV3.sol";
import {GaugeManager} from "../contracts/GaugeManager.sol";
import "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";

contract DeployBribeFactoryV3 is Script {
    using stdJson for string;
    
    function run() external {
        uint256 deployerKey = vm.envUint("PRIVATE_KEY");
        address deployer = vm.rememberKey(deployerKey);
        
        // Load deployed contracts
        string memory root = vm.projectRoot();
        string memory infraPath = string.concat(root, "/script/constants/output/Deploy1_Infrastructure.json");
        string memory factoriesPath = string.concat(root, "/script/constants/output/Deploy3a_GaugeFactories.json");
        string memory votingPath = string.concat(root, "/script/constants/output/Deploy3b_Voting.json");
        
        string memory infraJson = vm.readFile(infraPath);
        string memory factoriesJson = vm.readFile(factoriesPath);
        string memory votingJson = vm.readFile(votingPath);
        
        // Load required addresses
        address proxyAdmin = abi.decode(vm.parseJson(infraJson, ".ProxyAdmin"), (address));
        address permissionsRegistry = abi.decode(vm.parseJson(infraJson, ".PermissionsRegistry"), (address));
        address tokenHandler = abi.decode(vm.parseJson(infraJson, ".TokenHandler"), (address));
        address gaugeManager = abi.decode(vm.parseJson(factoriesJson, ".GaugeManager"), (address));
        address voter = abi.decode(vm.parseJson(votingJson, ".VoterV3"), (address));
        
        console.log("=== Deploy New BribeFactoryV3 ===");
        console.log("Deployer:", deployer);
        console.log("ProxyAdmin:", proxyAdmin);
        console.log("Voter:", voter);
        console.log("GaugeManager:", gaugeManager);
        console.log("PermissionsRegistry:", permissionsRegistry);
        console.log("TokenHandler:", tokenHandler);
        console.log("");
        
        vm.startBroadcast(deployer);
        
        // 1. Deploy BribeFactoryV3 implementation
        console.log("Deploying BribeFactoryV3 implementation...");
        BribeFactoryV3 bribeFactoryV3Impl = new BribeFactoryV3();
        console.log("BribeFactoryV3 implementation:", address(bribeFactoryV3Impl));
        
        // 2. Deploy proxy with initialization
        console.log("");
        console.log("Deploying BribeFactoryV3 proxy with initialization...");
        TransparentUpgradeableProxy bribeFactoryV3Proxy = new TransparentUpgradeableProxy(
            address(bribeFactoryV3Impl),
            proxyAdmin,
            abi.encodeWithSelector(
                BribeFactoryV3.initialize.selector,
                voter,                  // _voter
                gaugeManager,          // _gaugeManager
                permissionsRegistry,   // _permissionsRegistry
                tokenHandler          // _tokenHandler
            )
        );
        BribeFactoryV3 bribeFactoryV3 = BribeFactoryV3(address(bribeFactoryV3Proxy));
        console.log("BribeFactoryV3 proxy deployed and initialized at:", address(bribeFactoryV3));
        
        // 3. Set BribeFactory on GaugeManager
        console.log("");
        console.log("Setting BribeFactory on GaugeManager...");
        GaugeManager(gaugeManager).setBribeFactory(address(bribeFactoryV3));
        console.log("BribeFactory set on GaugeManager successfully");
        
        vm.stopBroadcast();
        
        // Verification
        console.log("");
        console.log("=== Verification ===");
        
        // Verify BribeFactoryV3 initialization
        address storedVoter = bribeFactoryV3.voter();
        address storedGaugeManager = bribeFactoryV3.gaugeManager();
        
        console.log("BribeFactoryV3 voter:", storedVoter);
        console.log("Expected voter:", voter);
        console.log("Voter matches:", storedVoter == voter);
        
        console.log("BribeFactoryV3 gaugeManager:", storedGaugeManager);
        console.log("Expected gaugeManager:", gaugeManager);
        console.log("GaugeManager matches:", storedGaugeManager == gaugeManager);
        
        // Save to JSON
        string memory outputPath = string.concat(root, "/script/constants/output/Deploy_BribeFactoryV3.json");
        
        string memory json = "";
        json = vm.serializeAddress("deployment", "BribeFactoryV3", address(bribeFactoryV3));
        json = vm.serializeAddress("deployment", "BribeFactoryV3Implementation", address(bribeFactoryV3Impl));
        json = vm.serializeAddress("deployment", "deployer", deployer);
        json = vm.serializeString("deployment", "timestamp", vm.toString(block.timestamp));
        
        vm.writeJson(json, outputPath);
        console.log("");
        console.log("Deployment details saved to:", outputPath);
        
        // Summary
        console.log("");
        console.log("=== Deployment Summary ===");
        console.log("New BribeFactoryV3 deployed at:", address(bribeFactoryV3));
        console.log("Implementation:", address(bribeFactoryV3Impl));
        console.log("Initialized with:");
        console.log("  - Voter:", voter);
        console.log("  - GaugeManager:", gaugeManager);
        console.log("  - PermissionsRegistry:", permissionsRegistry);
        console.log("  - TokenHandler:", tokenHandler);
        console.log("Dependencies configured:");
        console.log("  - BribeFactory set on GaugeManager");
    }
}
// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";
import "./BaseDeployScript.sol";

import {GaugeManager} from "../contracts/GaugeManager.sol";
import {BribeFactoryV3} from "../contracts/factories/BribeFactoryV3.sol";
import {PermissionsRegistry} from "../contracts/PermissionsRegistry.sol";

import "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";

contract Deploy3a2_GaugeManagerAndBribes is BaseDeployScript {
    using stdJson for string;
    
    function run() external {
        uint256 deployerKey = vm.envUint("PRIVATE_KEY");
        address deployer = vm.rememberKey(deployerKey);
        
        // Load previous deployments
        string memory infraPath = getInputPath("Deploy1_Infrastructure");
        string memory tokenPath = getInputPath("Deploy2_TokenSystem");
        string memory gaugeFactoriesPath = getInputPath("Deploy3a1_GaugeFactories");
        string memory configPath = getConfigPath();
        
        string memory infraJson = vm.readFile(infraPath);
        string memory tokenJson = vm.readFile(tokenPath);
        string memory gaugeFactoriesJson = vm.readFile(gaugeFactoriesPath);
        string memory configJson = vm.readFile(configPath);
        
        address proxyAdmin = abi.decode(vm.parseJson(infraJson, ".ProxyAdmin"), (address));
        address permissionsRegistry = abi.decode(vm.parseJson(infraJson, ".PermissionsRegistry"), (address));
        address tokenHandler = abi.decode(vm.parseJson(infraJson, ".TokenHandler"), (address));
        address pairFactory = abi.decode(vm.parseJson(configJson, ".v2Factory"), (address));
        address votingEscrow = abi.decode(vm.parseJson(tokenJson, ".VotingEscrow"), (address));
        address clFactory = abi.decode(vm.parseJson(configJson, ".clFactory"), (address));
        address nfpm = abi.decode(vm.parseJson(configJson, ".nonfungiblePositionManager"), (address));
        address gaugeFactory = abi.decode(vm.parseJson(gaugeFactoriesJson, ".GaugeFactory"), (address));
        address gaugeFactoryCL = abi.decode(vm.parseJson(gaugeFactoriesJson, ".GaugeFactoryCL"), (address));
        
        console.log("=== Deploy GaugeManager and BribeFactory ===");
        console.log("Deployer:", deployer);
        console.log("Using VotingEscrow:", votingEscrow);
        console.log("Using GaugeFactory:", gaugeFactory);
        console.log("Using GaugeFactoryCL:", gaugeFactoryCL);
        
        vm.startBroadcast(deployer);
        
        // 1. Deploy GaugeManager
        console.log("Deploying GaugeManager...");
        GaugeManager gaugeManagerImpl = new GaugeManager();
        TransparentUpgradeableProxy gaugeManagerProxy = new TransparentUpgradeableProxy(
            address(gaugeManagerImpl),
            proxyAdmin,
            ""
        );
        GaugeManager gaugeManager = GaugeManager(address(gaugeManagerProxy));
        console.log("GaugeManager:", address(gaugeManager));
        
        // Initialize GaugeManager
        console.log("Initializing GaugeManager...");
        gaugeManager.initialize(
            votingEscrow,                    // __ve
            tokenHandler,                    // _tokenHandler
            gaugeFactory,                    // _gaugeFactory
            gaugeFactoryCL,                  // _gaugeFactoryCL
            pairFactory,                     // _pairFactory
            clFactory,                       // _pairFactoryCL (external Algebra CL factory)
            permissionsRegistry,             // _permissionRegistory
            nfpm                            // _nfpm (external NonFungible Position Manager)
        );
        
        // 2. Deploy BribeFactoryV3
        console.log("Deploying BribeFactoryV3...");
        BribeFactoryV3 bribeFactoryV3Impl = new BribeFactoryV3();
        TransparentUpgradeableProxy bribeFactoryV3Proxy = new TransparentUpgradeableProxy(
            address(bribeFactoryV3Impl),
            proxyAdmin,
            ""
        );
        BribeFactoryV3 bribeFactoryV3 = BribeFactoryV3(address(bribeFactoryV3Proxy));
        console.log("BribeFactoryV3:", address(bribeFactoryV3));
        
        vm.stopBroadcast();
        
        // Save to JSON
        string memory path = getOutputPath("Deploy3a2_GaugeManagerAndBribes");
        
        string memory json = "";
        json = vm.serializeAddress("deployment", "GaugeManager", address(gaugeManager));
        json = vm.serializeAddress("deployment", "BribeFactoryV3", address(bribeFactoryV3));
        
        vm.writeJson(json, path);
        console.log("Addresses saved to:", path);
    }
}
