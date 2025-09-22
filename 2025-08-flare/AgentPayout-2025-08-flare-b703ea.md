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











#### AgentPayout.payoutFromPool(Agent.State,address,uint256,uint256) [INTERNAL]
```slithir
 poolBalance = _agent.collateralPool.totalCollateral()
REF_2922(IICollateralPool) -> _agent_1 (-> []).collateralPool
TMP_4394(uint256) = HIGH_LEVEL_CALL, dest:REF_2922(IICollateralPool), function:totalCollateral, arguments:[]  
poolBalance_1(uint256) := TMP_4394(uint256)
 _amountPaid = Math.min(_amountWei,poolBalance)
TMP_4395(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['_amountWei_1', 'poolBalance_1'] 
_amountPaid_1(uint256) := TMP_4395(uint256)
 _agentResponsibilityWei = Math.min(_agentResponsibilityWei,_amountPaid)
TMP_4396(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['_agentResponsibilityWei_1', '_amountPaid_1'] 
_agentResponsibilityWei_2(uint256) := TMP_4396(uint256)
 _agent.collateralPool.payout(_receiver,_amountPaid,_agentResponsibilityWei)
REF_2926(IICollateralPool) -> _agent_1 (-> []).collateralPool
HIGH_LEVEL_CALL, dest:REF_2926(IICollateralPool), function:payout, arguments:['_receiver_1', '_amountPaid_1', '_agentResponsibilityWei_2']  
 _amountPaid
RETURN _amountPaid_1
```
#### AgentPayout.payoutFromVault(Agent.State,address,uint256) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
_receiver_1(address) := phi(['_receiver_1'])
_amountWei_1(uint256) := phi(['amount_1'])
 collateral = Agents.getVaultCollateral(_agent)
TMP_4380(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getVaultCollateral(Agent.State), arguments:['_agent_1 (-> [])'] 
collateral_1 (-> ['TMP_4380'])(CollateralTypeInt.Data) := TMP_4380(CollateralTypeInt.Data)
 vault = IIAgentVault(_agent.vaultAddress())
TMP_4381(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_1 (-> [])'] 
TMP_4382 = CONVERT TMP_4381 to IIAgentVault
vault_1(IIAgentVault) := TMP_4382(IIAgentVault)
 _amountPaid = Math.min(_amountWei,collateral.token.balanceOf(address(vault)))
REF_2911(IERC20) -> collateral_1 (-> ['TMP_4380']).token
TMP_4383 = CONVERT vault_1 to address
TMP_4384(uint256) = HIGH_LEVEL_CALL, dest:REF_2911(IERC20), function:balanceOf, arguments:['TMP_4383']  
TMP_4385(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['_amountWei_1', 'TMP_4384'] 
_amountPaid_1(uint256) := TMP_4385(uint256)
 vault.payout(collateral.token,_receiver,_amountPaid)
REF_2914(IERC20) -> collateral_1 (-> ['TMP_4380']).token
HIGH_LEVEL_CALL, dest:vault_1(IIAgentVault), function:payout, arguments:['REF_2914', '_receiver_1', '_amountPaid_1']  
 _amountPaid
RETURN _amountPaid_1
```
#### AgentPayout.tryPayoutFromVault(Agent.State,address,uint256) [INTERNAL]
```slithir
 collateral = Agents.getVaultCollateral(_agent)
TMP_4387(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getVaultCollateral(Agent.State), arguments:['_agent_1 (-> [])'] 
collateral_1 (-> ['TMP_4387'])(CollateralTypeInt.Data) := TMP_4387(CollateralTypeInt.Data)
 vault = IIAgentVault(_agent.vaultAddress())
TMP_4388(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_1 (-> [])'] 
TMP_4389 = CONVERT TMP_4388 to IIAgentVault
vault_1(IIAgentVault) := TMP_4389(IIAgentVault)
 _amountPaid = Math.min(_amountWei,collateral.token.balanceOf(address(vault)))
REF_2918(IERC20) -> collateral_1 (-> ['TMP_4387']).token
TMP_4390 = CONVERT vault_1 to address
TMP_4391(uint256) = HIGH_LEVEL_CALL, dest:REF_2918(IERC20), function:balanceOf, arguments:['TMP_4390']  
TMP_4392(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['_amountWei_1', 'TMP_4391'] 
_amountPaid_1(uint256) := TMP_4392(uint256)
 vault.payout(collateral.token,_receiver,_amountPaid)
REF_2921(IERC20) -> collateral_1 (-> ['TMP_4387']).token
HIGH_LEVEL_CALL, dest:vault_1(IIAgentVault), function:payout, arguments:['REF_2921', '_receiver_1', '_amountPaid_1']  
 _success = true
_success_1(bool) := True(bool)
 _success = false
_success_3(bool) := False(bool)
 _amountPaid = 0
_amountPaid_3(uint256) := 0(uint256)
 (_success,_amountPaid)
_success_2(bool) := phi(['_success_1', '_success_3'])
_amountPaid_2(uint256) := phi(['_amountPaid_3', '_amountPaid_1'])
RETURN _success_2,_amountPaid_2
```
#### Agents.convertUSD5ToVaultCollateralWei(Agent.State,uint256) [INTERNAL]
```slithir
 Conversion.convertFromUSD5(_amountUSD5,getVaultCollateral(_agent))
TMP_4511(CollateralTypeInt.Data) = INTERNAL_CALL, Agents.getVaultCollateral(Agent.State)(_agent_1 (-> []))
TMP_4512(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertFromUSD5(uint256,CollateralTypeInt.Data), arguments:['_amountUSD5_1', 'TMP_4511'] 
RETURN TMP_4512
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
#### IICollateralPool.payout(address,uint256,uint256) [EXTERNAL]
```slithir

```
#### Math.min(uint256,uint256) [INTERNAL]
```slithir
a_1(uint256) := phi(['result_8'])
b_1(uint256) := phi(['TMP_554'])
 a < b
TMP_477(bool) = a_1 < b_1
CONDITION TMP_477
 a
RETURN a_1
 b
RETURN b_1
```
#### Agents.getVaultCollateral(Agent.State) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
 state = AssetManagerState.get()
TMP_4510(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4510'])(AssetManagerState.State) := TMP_4510(AssetManagerState.State)
 state.collateralTokens[_agent.vaultCollateralIndex]
REF_3005(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_4510']).collateralTokens
REF_3006(uint16) -> _agent_1 (-> []).vaultCollateralIndex
REF_3007(CollateralTypeInt.Data) -> REF_3005[REF_3006]
RETURN REF_3007
```
#### ERC20.balanceOf(address) [PUBLIC]
```slithir
_balances_1(mapping(address => uint256)) := phi(['_balances_11', '_balances_1', '_balances_8', '_balances_5', '_balances_0'])
 _balances[account]
REF_73(uint256) -> _balances_1[account_1]
RETURN REF_73
```
#### Agent.vaultAddress(Agent.State) [INTERNAL]
```slithir
AGENTS_POSITION_2(bytes32) := phi(['AGENTS_POSITION_0'])
 position = _agent
position_1(bytes32) := _agent_1 (-> [])(Agent.State)
 address(uint160((uint256(position) ^ uint256(AGENTS_POSITION)) >> 64))
TMP_5325 = CONVERT position_1 to uint256
TMP_5326 = CONVERT AGENTS_POSITION_2 to uint256
TMP_5327(uint256) = TMP_5325 ^ TMP_5326
TMP_5328(uint256) = TMP_5327 >> 64
TMP_5329 = CONVERT TMP_5328 to uint160
TMP_5330 = CONVERT TMP_5329 to address
RETURN TMP_5330
```
#### IIAgentVault.payout(IERC20,address,uint256) [EXTERNAL]
```slithir

```
#### Conversion.convertFromUSD5(uint256,CollateralTypeInt.Data) [INTERNAL]
```slithir
 bytes(_token.tokenFtsoSymbol).length == 0
REF_3158(string) -> _token_1 (-> []).tokenFtsoSymbol
TMP_4655 = CONVERT REF_3158 to bytes
REF_3159 -> LENGTH TMP_4655
TMP_4656(bool) = REF_3159 == 0
CONDITION TMP_4656
 _amountUSD5
RETURN _amountUSD5_1
 (tokenPrice,None,tokenFtsoDec) = readFtsoPrice(_token.tokenFtsoSymbol,false)
REF_3160(string) -> _token_1 (-> []).tokenFtsoSymbol
TUPLE_47(uint256,uint256,uint256) = INTERNAL_CALL, Conversion.readFtsoPrice(string,bool)(REF_3160,False)
tokenPrice_1(uint256)= UNPACK TUPLE_47 index: 0 
tokenFtsoDec_1(uint256)= UNPACK TUPLE_47 index: 2 
 expPlus = _token.decimals + tokenFtsoDec - 5
REF_3161(uint8) -> _token_1 (-> []).decimals
TMP_4657(uint8) = REF_3161 (c)+ tokenFtsoDec_1
TMP_4658(uint8) = TMP_4657 (c)- 5
expPlus_1(uint256) := TMP_4658(uint8)
 _amountUSD5.mulDiv(10 ** expPlus,tokenPrice)
TMP_4659(uint256) = 10 (c)** expPlus_1
TMP_4660(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['_amountUSD5_1', 'TMP_4659', 'tokenPrice_1'] 
RETURN TMP_4660
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
#### Conversion.readFtsoPrice(string,bool) [INTERNAL]
```slithir
_symbol_1(string) := phi(['REF_3163', 'REF_3166', 'REF_3160'])
_fromTrustedProviders_1(bool) := phi(['_fromTrustedProviders_1'])
 settings = Globals.getSettings()
TMP_4663(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4663'])(AssetManagerSettings.Data) := TMP_4663(AssetManagerSettings.Data)
 priceReader = IPriceReader(settings.priceReader)
REF_3169(address) -> settings_1 (-> ['TMP_4663']).priceReader
TMP_4664 = CONVERT REF_3169 to IPriceReader
priceReader_1(IPriceReader) := TMP_4664(IPriceReader)
 _fromTrustedProviders
CONDITION _fromTrustedProviders_1
 priceReader.getPriceFromTrustedProviders(_symbol)
TUPLE_50(uint256,uint256,uint256) = HIGH_LEVEL_CALL, dest:priceReader_1(IPriceReader), function:getPriceFromTrustedProviders, arguments:['_symbol_1']  
RETURN TUPLE_50
 priceReader.getPrice(_symbol)
TUPLE_51(uint256,uint256,uint256) = HIGH_LEVEL_CALL, dest:priceReader_1(IPriceReader), function:getPrice, arguments:['_symbol_1']  
RETURN TUPLE_51
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
