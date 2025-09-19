

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





#### AgentVaultAndPoolSupportFacet.assetPriceNatWei() [EXTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_1707(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_1707'])(AssetManagerSettings.Data) := TMP_1707(AssetManagerSettings.Data)
 _multiplier = Conversion.currentAmgPriceInTokenWei(Globals.getPoolCollateral())
TMP_1708(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getPoolCollateral(), arguments:[] 
TMP_1709(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.currentAmgPriceInTokenWei(CollateralTypeInt.Data), arguments:['TMP_1708'] 
_multiplier_1(uint256) := TMP_1709(uint256)
 _divisor = Conversion.AMG_TOKEN_WEI_PRICE_SCALE * settings.assetMintingGranularityUBA
REF_723(uint256) -> Conversion.AMG_TOKEN_WEI_PRICE_SCALE
REF_724(uint64) -> settings_1 (-> ['TMP_1707']).assetMintingGranularityUBA
TMP_1710(uint256) = REF_723 (c)* REF_724
_divisor_1(uint256) := TMP_1710(uint256)
 (_multiplier,_divisor)
RETURN _multiplier_1,_divisor_1
```
#### AgentVaultAndPoolSupportFacet.getFAssetsBackedByPool(address) [EXTERNAL]
```slithir
 agent = Agent.get(_agentVault)
TMP_1718(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_1718'])(Agent.State) := TMP_1718(Agent.State)
 Conversion.convertAmgToUBA(agent.reservedAMG + agent.mintedAMG + agent.poolRedeemingAMG)
REF_733(uint64) -> agent_1 (-> ['TMP_1718']).reservedAMG
REF_734(uint64) -> agent_1 (-> ['TMP_1718']).mintedAMG
TMP_1719(uint64) = REF_733 (c)+ REF_734
REF_735(uint64) -> agent_1 (-> ['TMP_1718']).poolRedeemingAMG
TMP_1720(uint64) = TMP_1719 (c)+ REF_735
TMP_1721(uint256) = LIBRARY_CALL, dest:Conversion, function:Conversion.convertAmgToUBA(uint64), arguments:['TMP_1720'] 
RETURN TMP_1721
```

#### AgentVaultAndPoolSupportFacet.getWorkAddress(address) [EXTERNAL]
```slithir
 Globals.getAgentOwnerRegistry().getWorkAddress(_managementAddress)
TMP_1724(IAgentOwnerRegistry) = LIBRARY_CALL, dest:Globals, function:Globals.getAgentOwnerRegistry(), arguments:[] 
TMP_1725(address) = HIGH_LEVEL_CALL, dest:TMP_1724(IAgentOwnerRegistry), function:getWorkAddress, arguments:['_managementAddress_1']  
RETURN TMP_1725
```
#### AgentVaultAndPoolSupportFacet.isAgentVaultOwner(address,address) [EXTERNAL]
```slithir
 agent = Agent.getAllowDestroyed(_agentVault)
TMP_1722(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.getAllowDestroyed(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_1722'])(Agent.State) := TMP_1722(Agent.State)
 Agents.isOwner(agent,_address)
TMP_1723(bool) = LIBRARY_CALL, dest:Agents, function:Agents.isOwner(Agent.State,address), arguments:["agent_1 (-> ['TMP_1722'])", '_address_1'] 
RETURN TMP_1723
```
#### AgentVaultAndPoolSupportFacet.isLockedVaultToken(address,IERC20) [EXTERNAL]
```slithir
 agent = Agent.get(_agentVault)
TMP_1711(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_1711'])(Agent.State) := TMP_1711(Agent.State)
 _token == agent.getVaultCollateralToken() || _token == agent.collateralPool.poolToken()
TMP_1712(IERC20) = LIBRARY_CALL, dest:Agents, function:Agents.getVaultCollateralToken(Agent.State), arguments:["agent_1 (-> ['TMP_1711'])"] 
TMP_1713(bool) = _token_1 == TMP_1712
REF_727(IICollateralPool) -> agent_1 (-> ['TMP_1711']).collateralPool
TMP_1714(ICollateralPoolToken) = HIGH_LEVEL_CALL, dest:REF_727(IICollateralPool), function:poolToken, arguments:[]  
TMP_1715(bool) = _token_1 == TMP_1714
TMP_1716(bool) = TMP_1713 || TMP_1715
RETURN TMP_1716
```
#### AgentVaultAndPoolSupportFacet.isVaultCollateralToken(IERC20) [EXTERNAL]
```slithir
 CollateralTypes.exists(CollateralType.Class.VAULT,_token)
REF_730(CollateralType.Class) -> Class.VAULT
TMP_1717(bool) = LIBRARY_CALL, dest:CollateralTypes, function:CollateralTypes.exists(CollateralType.Class,IERC20), arguments:['REF_730', '_token_1'] 
RETURN TMP_1717
```
#### Conversion.currentAmgPriceInTokenWei(CollateralTypeInt.Data) [INTERNAL]
```slithir
_token_1 (-> [])(CollateralTypeInt.Data) := phi(['_toToken_1 (-> [])', '_fromToken_1 (-> [])'])
 (_price,None,None) = currentAmgPriceInTokenWeiWithTs(_token,false)
TUPLE_44(uint256,uint256,uint256) = INTERNAL_CALL, Conversion.currentAmgPriceInTokenWeiWithTs(CollateralTypeInt.Data,bool)(_token_1 (-> []),False)
_price_1(uint256)= UNPACK TUPLE_44 index: 0 
 _price
RETURN _price_1
```
#### Globals.getPoolCollateral() [INTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_4734(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4734'])(AssetManagerState.State) := TMP_4734(AssetManagerState.State)
 state.collateralTokens[state.poolCollateralIndex]
REF_3236(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_4734']).collateralTokens
REF_3237(uint16) -> state_1 (-> ['TMP_4734']).poolCollateralIndex
REF_3238(CollateralTypeInt.Data) -> REF_3236[REF_3237]
RETURN REF_3238
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
#### Conversion.convertAmgToUBA(uint64) [INTERNAL]
```slithir
 settings = Globals.getSettings()
TMP_4637(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4637'])(AssetManagerSettings.Data) := TMP_4637(AssetManagerSettings.Data)
 uint256(_valueAMG) * settings.assetMintingGranularityUBA
TMP_4638 = CONVERT _valueAMG_1 to uint256
REF_3145(uint64) -> settings_1 (-> ['TMP_4637']).assetMintingGranularityUBA
TMP_4639(uint256) = TMP_4638 (c)* REF_3145
RETURN TMP_4639
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
#### Agents.isOwner(Agent.State,address) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['TMP_4493', '_agent_1 (-> [])'])
_address_1(address) := phi(['msg.sender'])
 _address == _agent.ownerManagementAddress || _address == getWorkAddress(_agent)
REF_2989(address) -> _agent_1 (-> []).ownerManagementAddress
TMP_4477(bool) = _address_1 == REF_2989
TMP_4478(address) = INTERNAL_CALL, Agents.getWorkAddress(Agent.State)(_agent_1 (-> []))
TMP_4479(bool) = _address_1 == TMP_4478
TMP_4480(bool) = TMP_4477 || TMP_4479
RETURN TMP_4480
```
#### Agent.getAllowDestroyed(address) [INTERNAL]
```slithir
 agent = getWithoutCheck(_address)
TMP_5315(Agent.State) = INTERNAL_CALL, Agent.getWithoutCheck(address)(_address_1)
agent_1 (-> ['TMP_5315'])(Agent.State) := TMP_5315(Agent.State)
 require(bool,error)(agent.status != Agent.Status.EMPTY,revert InvalidAgentVaultAddress()())
REF_3751(Agent.Status) -> agent_1 (-> ['TMP_5315']).status
REF_3752(Agent.Status) -> Status.EMPTY
TMP_5316(bool) = REF_3751 != REF_3752
TMP_5317(None) = SOLIDITY_CALL revert InvalidAgentVaultAddress()()
TMP_5318(None) = SOLIDITY_CALL require(bool,error)(TMP_5316,TMP_5317)
 agent
RETURN agent_1 (-> ['TMP_5315'])
```
#### Agents.getVaultCollateralToken(Agent.State) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
 state = AssetManagerState.get()
TMP_4509(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4509'])(AssetManagerState.State) := TMP_4509(AssetManagerState.State)
 state.collateralTokens[_agent.vaultCollateralIndex].token
REF_3000(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_4509']).collateralTokens
REF_3001(uint16) -> _agent_1 (-> []).vaultCollateralIndex
REF_3002(CollateralTypeInt.Data) -> REF_3000[REF_3001]
REF_3003(IERC20) -> REF_3002.token
RETURN REF_3003
```
#### CollateralTypes.exists(CollateralType.Class,IERC20) [INTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_4575(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4575'])(AssetManagerState.State) := TMP_4575(AssetManagerState.State)
 index = state.collateralTokenIndex[_tokenKey(_collateralClass,_token)]
REF_3070(mapping(bytes32 => uint256)) -> state_1 (-> ['TMP_4575']).collateralTokenIndex
TMP_4576(bytes32) = INTERNAL_CALL, CollateralTypes._tokenKey(CollateralType.Class,IERC20)(_collateralClass_1,_token_1)
REF_3071(uint256) -> REF_3070[TMP_4576]
index_1(uint256) := REF_3071(uint256)
 index > 0
TMP_4577(bool) = index_1 > 0
RETURN TMP_4577
```
#### Conversion.currentAmgPriceInTokenWeiWithTs(CollateralTypeInt.Data,bool) [INTERNAL]
```slithir
_token_1 (-> [])(CollateralTypeInt.Data) := phi(['_token_1 (-> [])', 'REF_3140', '_token_1 (-> [])'])
 (assetPrice,assetTs,assetFtsoDec) = readFtsoPrice(_token.assetFtsoSymbol,_fromTrustedProviders)
REF_3163(string) -> _token_1 (-> []).assetFtsoSymbol
TUPLE_48(uint256,uint256,uint256) = INTERNAL_CALL, Conversion.readFtsoPrice(string,bool)(REF_3163,_fromTrustedProviders_1)
assetPrice_1(uint256)= UNPACK TUPLE_48 index: 0 
assetTs_1(uint256)= UNPACK TUPLE_48 index: 1 
assetFtsoDec_1(uint256)= UNPACK TUPLE_48 index: 2 
 _token.directPricePair
REF_3164(bool) -> _token_1 (-> []).directPricePair
CONDITION REF_3164
 price = calcAmgToTokenWeiPrice(_token.decimals,1,0,assetPrice,assetFtsoDec)
REF_3165(uint8) -> _token_1 (-> []).decimals
TMP_4661(uint256) = INTERNAL_CALL, Conversion.calcAmgToTokenWeiPrice(uint256,uint256,uint256,uint256,uint256)(REF_3165,1,0,assetPrice_1,assetFtsoDec_1)
price_1(uint256) := TMP_4661(uint256)
 (price,assetTs,assetTs)
RETURN price_1,assetTs_1,assetTs_1
 (tokenPrice,tokenTs,tokenFtsoDec) = readFtsoPrice(_token.tokenFtsoSymbol,_fromTrustedProviders)
REF_3166(string) -> _token_1 (-> []).tokenFtsoSymbol
TUPLE_49(uint256,uint256,uint256) = INTERNAL_CALL, Conversion.readFtsoPrice(string,bool)(REF_3166,_fromTrustedProviders_1)
tokenPrice_1(uint256)= UNPACK TUPLE_49 index: 0 
tokenTs_1(uint256)= UNPACK TUPLE_49 index: 1 
tokenFtsoDec_1(uint256)= UNPACK TUPLE_49 index: 2 
 price_scope_0 = calcAmgToTokenWeiPrice(_token.decimals,tokenPrice,tokenFtsoDec,assetPrice,assetFtsoDec)
REF_3167(uint8) -> _token_1 (-> []).decimals
TMP_4662(uint256) = INTERNAL_CALL, Conversion.calcAmgToTokenWeiPrice(uint256,uint256,uint256,uint256,uint256)(REF_3167,tokenPrice_1,tokenFtsoDec_1,assetPrice_1,assetFtsoDec_1)
price_scope_0_1(uint256) := TMP_4662(uint256)
 (price_scope_0,assetTs,tokenTs)
RETURN price_scope_0_1,assetTs_1,tokenTs_1
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
#### Agents.getWorkAddress(Agent.State) [INTERNAL]
```slithir
_agent_1 (-> [])(Agent.State) := phi(['_agent_1 (-> [])', '_agent_1 (-> [])'])
 Globals.getAgentOwnerRegistry().getWorkAddress(_agent.ownerManagementAddress)
TMP_4481(IAgentOwnerRegistry) = LIBRARY_CALL, dest:Globals, function:Globals.getAgentOwnerRegistry(), arguments:[] 
REF_2992(address) -> _agent_1 (-> []).ownerManagementAddress
TMP_4482(address) = HIGH_LEVEL_CALL, dest:TMP_4481(IAgentOwnerRegistry), function:getWorkAddress, arguments:['REF_2992']  
RETURN TMP_4482
```

#### Conversion.calcAmgToTokenWeiPrice(uint256,uint256,uint256,uint256,uint256) [INTERNAL]
```slithir
_tokenDecimals_1(uint256) := phi(['REF_3165', 'REF_3167'])
_tokenPrice_1(uint256) := phi(['tokenPrice_1'])
_tokenFtsoDecimals_1(uint256) := phi(['tokenFtsoDec_1'])
_assetPrice_1(uint256) := phi(['assetPrice_1'])
_assetFtsoDecimals_1(uint256) := phi(['assetFtsoDec_1'])
AMG_TOKEN_WEI_PRICE_SCALE_EXP_1(uint256) := phi(['AMG_TOKEN_WEI_PRICE_SCALE_EXP_0'])
 settings = Globals.getSettings()
TMP_4665(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_4665'])(AssetManagerSettings.Data) := TMP_4665(AssetManagerSettings.Data)
 expPlus = _tokenDecimals + _tokenFtsoDecimals + AMG_TOKEN_WEI_PRICE_SCALE_EXP
TMP_4666(uint256) = _tokenDecimals_1 (c)+ _tokenFtsoDecimals_1
TMP_4667(uint256) = TMP_4666 (c)+ AMG_TOKEN_WEI_PRICE_SCALE_EXP_1
expPlus_1(uint256) := TMP_4667(uint256)
 expMinus = settings.assetMintingDecimals + _assetFtsoDecimals
REF_3173(uint8) -> settings_1 (-> ['TMP_4665']).assetMintingDecimals
TMP_4668(uint8) = REF_3173 (c)+ _assetFtsoDecimals_1
expMinus_1(uint256) := TMP_4668(uint8)
 assert(bool)(expPlus >= expMinus)
TMP_4669(bool) = expPlus_1 >= expMinus_1
TMP_4670(None) = SOLIDITY_CALL assert(bool)(TMP_4669)
 _assetPrice.mulDiv(10 ** (expPlus - expMinus),_tokenPrice)
TMP_4671(uint256) = expPlus_1 (c)- expMinus_1
TMP_4672(uint256) = 10 (c)** TMP_4671
TMP_4673(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['_assetPrice_1', 'TMP_4672', '_tokenPrice_1'] 
RETURN TMP_4673
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
