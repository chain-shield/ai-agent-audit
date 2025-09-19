
### Storage layout (CollateralPool) 

```text
agentVault address
assetManager IIAssetManager
fAsset IFAsset
token IICollateralPoolToken
wNat IWNat
exitCollateralRatioBIPS uint32
__topupCollateralRatioBIPS uint32
__topupTokenPriceFactorBIPS uint16
internalWithdrawal bool
initialized bool
_fAssetFeeDebtOf mapping(address => int256)
totalFAssetFeeDebt int256
totalFAssetFees uint256
totalCollateral uint256

```
### Storage layout (ERC20) 

```text
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string

```




### Storage layout (WNatMock) 

```text
governanceVP IGovernanceVotePower
delegations mapping(address => WNatMock.Delegation[])
delegators mapping(address => EnumerableSet.AddressSet)

```


### Storage layout (AgentOwnerRegistry) 

```text
manager address
whitelist mapping(address => bool)
workToMgmtAddress mapping(address => address)
mgmtToWorkAddress mapping(address => address)
agentName mapping(address => string)
agentDescription mapping(address => string)
agentIconUrl mapping(address => string)
agentTouUrl mapping(address => string)

```

### Storage layout (AssetManagerMock) 

```text
wNat IWNat
fasset IIFAsset
commonOwner address
checkForValidAgentVaultAddress bool
collateralPool address
maxRedemption uint256
fassetsBackedByPool uint256
timelockDuration uint256
assetPriceMul uint256
assetPriceDiv uint256
lotSize uint256
minPoolCollateralRatioBIPS uint256
assetMintingGranularityUBA uint256

```


#### Minting.calculateCurrentPoolFeeUBA(Agent.State,uint256) [INTERNAL]
```slithir
 mintingFeeUBA = _mintingValueUBA.mulBips(_agent.feeBIPS)
REF_3359(uint16) -> _agent_1 (-> []).feeBIPS
TMP_4852(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulBips(uint256,uint256), arguments:['_mintingValueUBA_1', 'REF_3359'] 
mintingFeeUBA_1(uint256) := TMP_4852(uint256)
 _calculatePoolFeeUBA(mintingFeeUBA,_agent.poolFeeShareBIPS)
REF_3360(uint16) -> _agent_1 (-> []).poolFeeShareBIPS
TMP_4853(uint256) = INTERNAL_CALL, Minting._calculatePoolFeeUBA(uint256,uint16)(mintingFeeUBA_1,REF_3360)
RETURN TMP_4853
```
#### Minting.calculatePoolFeeUBA(Agent.State,CollateralReservation.Data) [INTERNAL]
```slithir
_agent_1 (-> ['TMP_4821'])(Agent.State) := phi(["agent_1 (-> ['TMP_4821'])"])
_crt_1 (-> [])(CollateralReservation.Data) := phi(['_crt_1 (-> [])'])
 storedPoolFeeShareBIPS = _crt.poolFeeShareBIPS
REF_3355(uint16) -> _crt_1 (-> []).poolFeeShareBIPS
storedPoolFeeShareBIPS_1(uint16) := REF_3355(uint16)
 _calculatePoolFeeUBA(_crt.underlyingFeeUBA,poolFeeShareBIPS)
REF_3356(uint128) -> _crt_1 (-> []).underlyingFeeUBA
TMP_4849(uint256) = INTERNAL_CALL, Minting._calculatePoolFeeUBA(uint256,uint16)(REF_3356,poolFeeShareBIPS_3)
RETURN TMP_4849
 storedPoolFeeShareBIPS > 0
TMP_4850(bool) = storedPoolFeeShareBIPS_1 > 0
CONDITION TMP_4850
 poolFeeShareBIPS = storedPoolFeeShareBIPS - 1
TMP_4851(uint16) = storedPoolFeeShareBIPS_1 (c)- 1
poolFeeShareBIPS_1(uint16) := TMP_4851(uint16)
 poolFeeShareBIPS = _agent.poolFeeShareBIPS
REF_3357(uint16) -> _agent_1 (-> ['TMP_4821']).poolFeeShareBIPS
poolFeeShareBIPS_2(uint16) := REF_3357(uint16)
poolFeeShareBIPS_3(uint16) := phi(['poolFeeShareBIPS_1', 'poolFeeShareBIPS_2'])
```
#### Minting.checkMintingCap(uint64) [INTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_4838(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4838'])(AssetManagerState.State) := TMP_4838(AssetManagerState.State)
 settings = Globals.getSettings()
TMP_4839(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4839'])(AssetManagerSettings.Data) := TMP_4839(AssetManagerSettings.Data)
 mintingCapAMG = settings.mintingCapAMG
REF_3350(uint64) -> settings_1 (-> ['TMP_4839']).mintingCapAMG
mintingCapAMG_1(uint256) := REF_3350(uint64)
 mintingCapAMG == 0
TMP_4840(bool) = mintingCapAMG_1 == 0
CONDITION TMP_4840
 totalMintedUBA = IERC20(settings.fAsset).totalSupply()
REF_3351(address) -> settings_1 (-> ['TMP_4839']).fAsset
TMP_4841 = CONVERT REF_3351 to IERC20
TMP_4842(uint256) = HIGH_LEVEL_CALL, dest:TMP_4841(IERC20), function:totalSupply, arguments:[]  
totalMintedUBA_1(uint256) := TMP_4842(uint256)
 totalAMG = state.totalReservedCollateralAMG + Conversion.convertUBAToAmg(totalMintedUBA)
REF_3353(uint64) -> state_1 (-> ['TMP_4838']).totalReservedCollateralAMG
TMP_4843(uint64) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertUBAToAmg(uint256), arguments:['totalMintedUBA_1'] 
TMP_4844(uint64) = REF_3353 (c)+ TMP_4843
totalAMG_1(uint256) := TMP_4844(uint64)
 require(bool,error)(totalAMG + _increaseAMG <= mintingCapAMG,revert MintingCapExceeded()())
TMP_4845(uint256) = totalAMG_1 (c)+ _increaseAMG_1
TMP_4846(bool) = TMP_4845 <= mintingCapAMG_1
TMP_4847(None) = SOLIDITY_CALL revert MintingCapExceeded()()
TMP_4848(None) = SOLIDITY_CALL require(bool,error)(TMP_4846,TMP_4847)
```
#### Minting.distributeCollateralReservationFee(Agent.State,uint256) [INTERNAL]
```slithir
 _fee == 0
TMP_4806(bool) = _fee_1 == 0
CONDITION TMP_4806
 poolFeeShare = _fee.mulBips(_agent.poolFeeShareBIPS)
REF_3317(uint16) -> _agent_1 (-> []).poolFeeShareBIPS
TMP_4807(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulBips(uint256,uint256), arguments:['_fee_1', 'REF_3317'] 
poolFeeShare_1(uint256) := TMP_4807(uint256)
 _agent.collateralPool.depositNat{value: poolFeeShare}()
REF_3318(IICollateralPool) -> _agent_1 (-> []).collateralPool
HIGH_LEVEL_CALL, dest:REF_3318(IICollateralPool), function:depositNat, arguments:[] value:poolFeeShare_1 
 Transfers.depositWNat(Globals.getWNat(),Agents.getOwnerPayAddress(_agent),_fee - poolFeeShare)
TMP_4809(IWNat) = LIBRARY_CALL, dest:Globals, function:Globals.getWNat(), arguments:[] 
TMP_4810(address) = LIBRARY_CALL, dest:Agents, function:Agents.getOwnerPayAddress(Agent.State), arguments:['_agent_1 (-> [])'] 
TMP_4811(uint256) = _fee_1 (c)- poolFeeShare_1
LIBRARY_CALL, dest:Transfers, function:Transfers.depositWNat(IWNat,address,uint256), arguments:['TMP_4809', 'TMP_4810', 'TMP_4811']
```
#### Minting.getCollateralReservation(uint256,bool) [INTERNAL]
```slithir
 require(bool,error)(_crtId > 0,revert InvalidCrtId()())
TMP_4828(bool) = _crtId_1 > 0
TMP_4829(None) = SOLIDITY_CALL revert InvalidCrtId()()
TMP_4830(None) = SOLIDITY_CALL require(bool,error)(TMP_4828,TMP_4829)
 state = AssetManagerState.get()
TMP_4831(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4831'])(AssetManagerState.State) := TMP_4831(AssetManagerState.State)
 _crt = state.crts[_crtId]
REF_3343(mapping(uint256 => CollateralReservation.Data)) -> state_1 (-> ['TMP_4831']).crts
REF_3344(CollateralReservation.Data) -> REF_3343[_crtId_1]
_crt_1 (-> ['state'])(CollateralReservation.Data) := REF_3344(CollateralReservation.Data)
 require(bool,error)(_crt.valueAMG != 0,revert InvalidCrtId()())
REF_3345(uint64) -> _crt_1 (-> ['state']).valueAMG
TMP_4832(bool) = REF_3345 != 0
TMP_4833(None) = SOLIDITY_CALL revert InvalidCrtId()()
TMP_4834(None) = SOLIDITY_CALL require(bool,error)(TMP_4832,TMP_4833)
 _requireActive
CONDITION _requireActive_1
 require(bool,error)(_crt.status == CollateralReservation.Status.ACTIVE,revert InvalidCrtId()())
REF_3346(CollateralReservation.Status) -> _crt_1 (-> ['state']).status
REF_3347(CollateralReservation.Status) -> Status.ACTIVE
TMP_4835(bool) = REF_3346 == REF_3347
TMP_4836(None) = SOLIDITY_CALL revert InvalidCrtId()()
TMP_4837(None) = SOLIDITY_CALL require(bool,error)(TMP_4835,TMP_4836)
 _crt
RETURN _crt_1 (-> ['state'])
```
#### Minting.payOrBurnExecutorFee(CollateralReservation.Data) [INTERNAL]
```slithir
 executorFeeNatWei = _crt.executorFeeNatGWei * Conversion.GWEI
REF_3323(uint64) -> _crt_1 (-> []).executorFeeNatGWei
REF_3324(uint256) -> Conversion.GWEI
TMP_4813(uint64) = REF_3323 (c)* REF_3324
executorFeeNatWei_1(uint256) := TMP_4813(uint64)
 executorFeeNatWei > 0
TMP_4814(bool) = executorFeeNatWei_1 > 0
CONDITION TMP_4814
 _crt.executorFeeNatGWei = 0
REF_3325(uint64) -> _crt_1 (-> []).executorFeeNatGWei
_crt_2 (-> [])(CollateralReservation.Data) := phi(['_crt_1 (-> [])'])
REF_3325(uint64) (->_crt_2 (-> [])) := 0(uint256)
 msg.sender == _crt.executor
REF_3326(address) -> _crt_2 (-> []).executor
TMP_4815(bool) = msg.sender == REF_3326
CONDITION TMP_4815
 Transfers.depositWNat(Globals.getWNat(),_crt.executor,executorFeeNatWei)
TMP_4816(IWNat) = LIBRARY_CALL, dest:Globals, function:Globals.getWNat(), arguments:[] 
REF_3329(address) -> _crt_2 (-> []).executor
LIBRARY_CALL, dest:Transfers, function:Transfers.depositWNat(IWNat,address,uint256), arguments:['TMP_4816', 'REF_3329', 'executorFeeNatWei_1'] 
 Globals.getBurnAddress().transfer(executorFeeNatWei)
TMP_4818(address) = LIBRARY_CALL, dest:Globals, function:Globals.getBurnAddress(), arguments:[] 
Transfer dest:TMP_4818 value:executorFeeNatWei_1
```
#### Minting.releaseCollateralReservation(CollateralReservation.Data,CollateralReservation.Status) [INTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_4820(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4820'])(AssetManagerState.State) := TMP_4820(AssetManagerState.State)
 agent = Agent.get(_crt.agentVault)
REF_3334(address) -> _crt_1 (-> []).agentVault
TMP_4821(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['REF_3334'] 
agent_1 (-> ['TMP_4821'])(Agent.State) := TMP_4821(Agent.State)
 reservationAMG = _crt.valueAMG + Conversion.convertUBAToAmg(calculatePoolFeeUBA(agent,_crt))
REF_3335(uint64) -> _crt_1 (-> []).valueAMG
TMP_4822(uint256) = INTERNAL_CALL, Minting.calculatePoolFeeUBA(Agent.State,CollateralReservation.Data)(agent_1 (-> ['TMP_4821']),_crt_1 (-> []))
TMP_4823(uint64) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertUBAToAmg(uint256), arguments:['TMP_4822'] 
TMP_4824(uint64) = REF_3335 (c)+ TMP_4823
reservationAMG_1(uint64) := TMP_4824(uint64)
 agent.reservedAMG = agent.reservedAMG - reservationAMG
REF_3337(uint64) -> agent_1 (-> ['TMP_4821']).reservedAMG
REF_3338(uint64) -> agent_1 (-> ['TMP_4821']).reservedAMG
TMP_4825(uint64) = REF_3338 (c)- reservationAMG_1
agent_2 (-> ['TMP_4821'])(Agent.State) := phi(["agent_1 (-> ['TMP_4821'])"])
REF_3337(uint64) (->agent_2 (-> ['TMP_4821'])) := TMP_4825(uint64)
TMP_4821(Agent.State) := phi(["agent_2 (-> ['TMP_4821'])"])
 state.totalReservedCollateralAMG -= reservationAMG
REF_3339(uint64) -> state_1 (-> ['TMP_4820']).totalReservedCollateralAMG
state_2 (-> ['TMP_4820'])(AssetManagerState.State) := phi(["state_1 (-> ['TMP_4820'])"])
REF_3339(-> state_2 (-> ['TMP_4820'])) = REF_3339 (c)- reservationAMG_1
TMP_4820(AssetManagerState.State) := phi(["state_2 (-> ['TMP_4820'])"])
 assert(bool)(_status != CollateralReservation.Status.ACTIVE)
REF_3340(CollateralReservation.Status) -> Status.ACTIVE
TMP_4826(bool) = _status_1 != REF_3340
TMP_4827(None) = SOLIDITY_CALL assert(bool)(TMP_4826)
 _crt.status = _status
REF_3341(CollateralReservation.Status) -> _crt_1 (-> []).status
_crt_2 (-> [])(CollateralReservation.Data) := phi(['_crt_1 (-> [])'])
REF_3341(CollateralReservation.Status) (->_crt_2 (-> [])) := _status_1(CollateralReservation.Status)
```
#### Conversion.roundUBAToAmg(uint256) [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4643(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4643'])(AssetManagerSettings.Data) := TMP_4643(AssetManagerSettings.Data)
 _valueUBA - (_valueUBA % settings.assetMintingGranularityUBA)
REF_3150(uint64) -> settings_1 (-> ['TMP_4643']).assetMintingGranularityUBA
TMP_4644(uint256) = _valueUBA_1 % REF_3150
TMP_4645(uint256) = _valueUBA_1 (c)- TMP_4644
RETURN TMP_4645
```
#### SafePct.mulBips(uint256,uint256) [INTERNAL]
```slithir
MAX_BIPS_1(uint256) := phi(['MAX_BIPS_2', 'MAX_BIPS_0'])
 mulDiv(x,y,MAX_BIPS)
TMP_10535(uint256) = INTERNAL_CALL, SafePct.mulDiv(uint256,uint256,uint256)(x_1,y_1,MAX_BIPS_1)
RETURN TMP_10535
```
#### Conversion.convertUBAToAmg(uint256) [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4640(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4640'])(AssetManagerSettings.Data) := TMP_4640(AssetManagerSettings.Data)
 SafeCast.toUint64(_valueUBA / settings.assetMintingGranularityUBA)
REF_3148(uint64) -> settings_1 (-> ['TMP_4640']).assetMintingGranularityUBA
TMP_4641(uint256) = _valueUBA_1 (c)/ REF_3148
TMP_4642(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['TMP_4641'] 
RETURN TMP_4642
```
#### ERC20.totalSupply() [PUBLIC]
```slithir
_totalSupply_1(uint256) := phi(['_totalSupply_0', '_totalSupply_4', '_totalSupply_7'])
 _totalSupply
RETURN _totalSupply_1
```
#### Globals.getSettings() [INTERNAL]
```slithir
ASSET_MANAGER_SETTINGS_POSITION_1(bytes32) := phi(['ASSET_MANAGER_SETTINGS_POSITION_0'])
 position = ASSET_MANAGER_SETTINGS_POSITION
position_1(bytes32) := ASSET_MANAGER_SETTINGS_POSITION_1(bytes32)
 _settings = position
_settings_1 (-> ['position'])(AssetManagerSettings.Data) := position_1(bytes32)
 _settings
RETURN _settings_1 (-> ['position'])
```
#### AssetManagerState.get() [INTERNAL]
```slithir
STATE_POSITION_1(bytes32) := phi(['STATE_POSITION_0'])
 position = STATE_POSITION
position_1(bytes32) := STATE_POSITION_1(bytes32)
 _state = position
_state_1 (-> ['position'])(AssetManagerState.State) := position_1(bytes32)
 _state
RETURN _state_1 (-> ['position'])
```
#### Agents.getOwnerPayAddress(Agent.State) [INTERNAL]
```slithir
 workAddress = getWorkAddress(_agent)
TMP_4483(address) = INTERNAL_CALL, Agents.getWorkAddress(Agent.State)(_agent_1 (-> []))
workAddress_1(address) := TMP_4483(address)
 workAddress != address(0)
TMP_4484 = CONVERT 0 to address
TMP_4485(bool) = workAddress_1 != TMP_4484
CONDITION TMP_4485
 address(workAddress)
TMP_4486 = CONVERT workAddress_1 to address
RETURN TMP_4486
 address(_agent.ownerManagementAddress)
REF_2993(address) -> _agent_1 (-> []).ownerManagementAddress
TMP_4487 = CONVERT REF_2993 to address
RETURN TMP_4487
```
#### Globals.getWNat() [INTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_4731(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4731'])(AssetManagerState.State) := TMP_4731(AssetManagerState.State)
 IWNat(address(state.collateralTokens[state.poolCollateralIndex].token))
REF_3231(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_4731']).collateralTokens
REF_3232(uint16) -> state_1 (-> ['TMP_4731']).poolCollateralIndex
REF_3233(CollateralTypeInt.Data) -> REF_3231[REF_3232]
REF_3234(IERC20) -> REF_3233.token
TMP_4732 = CONVERT REF_3234 to address
TMP_4733 = CONVERT TMP_4732 to IWNat
RETURN TMP_4733
```
#### CollateralPool.depositNat() [EXTERNAL]
```slithir
 _depositWNat()
INTERNAL_CALL, CollateralPool._depositWNat()()
 onlyAssetManager()
MODIFIER_CALL, CollateralPool.onlyAssetManager()()
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### Transfers.depositWNat(IWNat,address,uint256) [INTERNAL]
```slithir
 _amount > 0
TMP_10540(bool) = _amount_1 > 0
CONDITION TMP_10540
 _wNat.depositTo{value: _amount}(_recipient)
HIGH_LEVEL_CALL, dest:_wNat_1(IWNat), function:depositTo, arguments:['_recipient_1'] value:_amount_1
```
#### Globals.getBurnAddress() [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4739(AssetManagerSettings.Data) = INTERNAL_CALL, Globals.getSettings()()
settings_1 (-> ['TMP_4739'])(AssetManagerSettings.Data) := TMP_4739(AssetManagerSettings.Data)
 settings.burnAddress
REF_3244(address) -> settings_1 (-> ['TMP_4739']).burnAddress
RETURN REF_3244
```
#### Agent.get(address) [INTERNAL]
```slithir
 agent = getWithoutCheck(_address)
TMP_5309(Agent.State) = INTERNAL_CALL, Agent.getWithoutCheck(address)(_address_1)
agent_1 (-> ['TMP_5309'])(Agent.State) := TMP_5309(Agent.State)
 status = agent.status
REF_3748(Agent.Status) -> agent_1 (-> ['TMP_5309']).status
status_1(Agent.Status) := REF_3748(Agent.Status)
 require(bool,error)(status != Agent.Status.EMPTY && status != Agent.Status.DESTROYED,revert InvalidAgentVaultAddress()())
REF_3749(Agent.Status) -> Status.EMPTY
TMP_5310(bool) = status_1 != REF_3749
REF_3750(Agent.Status) -> Status.DESTROYED
TMP_5311(bool) = status_1 != REF_3750
TMP_5312(bool) = TMP_5310 && TMP_5311
TMP_5313(None) = SOLIDITY_CALL revert InvalidAgentVaultAddress()()
TMP_5314(None) = SOLIDITY_CALL require(bool,error)(TMP_5312,TMP_5313)
 agent
RETURN agent_1 (-> ['TMP_5309'])
```
#### SafePct.mulDiv(uint256,uint256,uint256) [INTERNAL]
```slithir
x_1(uint256) := phi(['x_1', 'x_1'])
y_1(uint256) := phi(['y_1', 'y_1'])
z_1(uint256) := phi(['z_1', 'MAX_BIPS_1'])
 require(bool,error)(z > 0,revert DivisionByZero()())
TMP_10510(bool) = z_1 > 0
TMP_10511(None) = SOLIDITY_CALL revert DivisionByZero()()
TMP_10512(None) = SOLIDITY_CALL require(bool,error)(TMP_10510,TMP_10511)
 x == 0
TMP_10513(bool) = x_1 == 0
CONDITION TMP_10513
 0
RETURN 0
 xy = x * y
TMP_10514(uint256) = x_1 * y_1
xy_1(uint256) := TMP_10514(uint256)
 xy / x == y
TMP_10515(uint256) = xy_1 / x_1
TMP_10516(bool) = TMP_10515 == y_1
CONDITION TMP_10516
 xy / z
TMP_10517(uint256) = xy_1 / z_1
RETURN TMP_10517
 a = x / z
TMP_10518(uint256) = x_1 (c)/ z_1
a_1(uint256) := TMP_10518(uint256)
 b = x % z
TMP_10519(uint256) = x_1 % z_1
b_1(uint256) := TMP_10519(uint256)
 c = y / z
TMP_10520(uint256) = y_1 (c)/ z_1
c_1(uint256) := TMP_10520(uint256)
 d = y % z
TMP_10521(uint256) = y_1 % z_1
d_1(uint256) := TMP_10521(uint256)
 (a * c * z) + (a * d) + (b * c) + (b * d / z)
TMP_10522(uint256) = a_1 (c)* c_1
TMP_10523(uint256) = TMP_10522 (c)* z_1
TMP_10524(uint256) = a_1 (c)* d_1
TMP_10525(uint256) = TMP_10523 (c)+ TMP_10524
TMP_10526(uint256) = b_1 (c)* c_1
TMP_10527(uint256) = TMP_10525 (c)+ TMP_10526
TMP_10528(uint256) = b_1 (c)* d_1
TMP_10529(uint256) = TMP_10528 (c)/ z_1
TMP_10530(uint256) = TMP_10527 (c)+ TMP_10529
RETURN TMP_10530
```
#### SafeCast.toUint64(uint256) [INTERNAL]
```slithir
 require(bool,string)(value <= type()(uint64).max,SafeCast: value doesn't fit in 64 bits)
TMP_747(uint64) := 18446744073709551615(uint64)
TMP_748(bool) = value_1 <= TMP_747
TMP_749(None) = SOLIDITY_CALL require(bool,string)(TMP_748,SafeCast: value doesn't fit in 64 bits)
 uint64(value)
TMP_750 = CONVERT value_1 to uint64
RETURN TMP_750
```
#### Agents.getWorkAddress(Agent.State) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])', '_agent_1 (-> [])'])
 Globals.getAgentOwnerRegistry().getWorkAddress(_agent.ownerManagementAddress)
TMP_4481(IAgentOwnerRegistry) = LIBRARY_CALL, dest:Globals, function:Globals.getAgentOwnerRegistry(), arguments:[] 
REF_2992(address) -> _agent_1 (-> []).ownerManagementAddress
TMP_4482(address) = HIGH_LEVEL_CALL, dest:TMP_4481(IAgentOwnerRegistry), function:getWorkAddress, arguments:['REF_2992']  
RETURN TMP_4482
```
#### CollateralPool._depositWNat() [INTERNAL]
```slithir
agentVault_29(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_1', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_0', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_28(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_14', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_0', 'assetManager_30'])
wNat_7(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_0', 'wNat_25', 'wNat_15', 'wNat_36'])
totalCollateral_36(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_0', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
 msg.value > 0
TMP_6312(bool) = msg.value > 0
CONDITION TMP_6312
 totalCollateral += msg.value
totalCollateral_37(uint256) = totalCollateral_36 (c)+ msg.value
 wNat.deposit{value: msg.value}()
HIGH_LEVEL_CALL, dest:wNat_7(IWNat), function:deposit, arguments:[] value:msg.value 
agentVault_30(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_29', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_29(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_28', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
wNat_8(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_7', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_25', 'wNat_15', 'wNat_36'])
 assetManager.updateCollateral(agentVault,wNat)
HIGH_LEVEL_CALL, dest:assetManager_29(IIAssetManager), function:updateCollateral, arguments:['agentVault_30', 'wNat_8']  
agentVault_31(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_30', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_30(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_29', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
wNat_9(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_8', 'wNat_28', 'wNat_43', 'wNat_25', 'wNat_15', 'wNat_36'])
```
#### WNatMock.depositTo(address) [PUBLIC]
```slithir
 _mint(_recipient,msg.value)
INTERNAL_CALL, ERC20._mint(address,uint256)(_recipient_1,msg.value)
```
#### Agent.getWithoutCheck(address) [INTERNAL]
```slithir
_address_1(address) := phi(['_address_1', '_address_1'])
AGENTS_POSITION_1(bytes32) := phi(['AGENTS_POSITION_0'])
 position = bytes32(uint256(AGENTS_POSITION) ^ (uint256(uint160(_address)) << 64))
TMP_5319 = CONVERT AGENTS_POSITION_1 to uint256
TMP_5320 = CONVERT _address_1 to uint160
TMP_5321 = CONVERT TMP_5320 to uint256
TMP_5322(uint256) = TMP_5321 << 64
TMP_5323(uint256) = TMP_5319 ^ TMP_5322
TMP_5324 = CONVERT TMP_5323 to bytes32
position_1(bytes32) := TMP_5324(bytes32)
 _agent = position
_agent_1 (-> ['position'])(Agent.State) := position_1(bytes32)
 _agent
RETURN _agent_1 (-> ['position'])
```
#### Globals.getAgentOwnerRegistry() [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4737(AssetManagerSettings.Data) = INTERNAL_CALL, Globals.getSettings()()
settings_1 (-> ['TMP_4737'])(AssetManagerSettings.Data) := TMP_4737(AssetManagerSettings.Data)
 IAgentOwnerRegistry(settings.agentOwnerRegistry)
REF_3242(address) -> settings_1 (-> ['TMP_4737']).agentOwnerRegistry
TMP_4738 = CONVERT REF_3242 to IAgentOwnerRegistry
RETURN TMP_4738
```
#### AgentOwnerRegistry.getWorkAddress(address) [EXTERNAL]
```slithir
mgmtToWorkAddress_4(mapping(address => address)) := phi(['mgmtToWorkAddress_3', 'mgmtToWorkAddress_0', 'mgmtToWorkAddress_4'])
 mgmtToWorkAddress[_managementAddress]
REF_311(address) -> mgmtToWorkAddress_4[_managementAddress_1]
RETURN REF_311
```
#### AssetManagerMock.updateCollateral(address,IERC20) [EXTERNAL]
```slithir
checkForValidAgentVaultAddress_1(bool) := phi(['checkForValidAgentVaultAddress_0', 'checkForValidAgentVaultAddress_2'])
 require(bool,error)(! checkForValidAgentVaultAddress,Agent.InvalidAgentVaultAddress())
TMP_5462 = UnaryType.BANG checkForValidAgentVaultAddress_1 
TMP_5463(None) = SOLIDITY_CALL revert InvalidAgentVaultAddress()()
TMP_5464(None) = SOLIDITY_CALL require(bool,error)(TMP_5462,TMP_5463)
 CollateralUpdated(_agentVault,address(_token))
TMP_5465 = CONVERT _token_1 to address
Emit CollateralUpdated(_agentVault_1,TMP_5465)
```
#### WNatMock.deposit() [PUBLIC]
```slithir
 _mint(msg.sender,msg.value)
INTERNAL_CALL, ERC20._mint(address,uint256)(msg.sender,msg.value)
```
#### ERC20._mint(address,uint256) [INTERNAL]
```slithir
_balances_6(mapping(address => uint256)) := phi(['_balances_11', '_balances_1', '_balances_5', '_balances_8', '_balances_0'])
_totalSupply_2(uint256) := phi(['_totalSupply_0', '_totalSupply_4', '_totalSupply_7'])
 require(bool,string)(account != address(0),ERC20: mint to the zero address)
TMP_207 = CONVERT 0 to address
TMP_208(bool) = account_1 != TMP_207
TMP_209(None) = SOLIDITY_CALL require(bool,string)(TMP_208,ERC20: mint to the zero address)
 _beforeTokenTransfer(address(0),account,amount)
TMP_210 = CONVERT 0 to address
INTERNAL_CALL, ERC20._beforeTokenTransfer(address,address,uint256)(TMP_210,account_1,amount_1)
 _totalSupply += amount
_totalSupply_4(uint256) = _totalSupply_3 (c)+ amount_1
 _balances[account] += amount
REF_79(uint256) -> _balances_7[account_1]
_balances_8(mapping(address => uint256)) := phi(['_balances_7'])
REF_79(-> _balances_8) = REF_79 + amount_1
 Transfer(address(0),account,amount)
TMP_212 = CONVERT 0 to address
Emit Transfer(TMP_212,account_1,amount_1)
 _afterTokenTransfer(address(0),account,amount)
TMP_214 = CONVERT 0 to address
INTERNAL_CALL, ERC20._afterTokenTransfer(address,address,uint256)(TMP_214,account_1,amount_1)
```
