







#### AgentSettingsFacet._executeUpdate(Agent.State,bytes32,uint256) [PRIVATE]
```slithir
_agent_1 (-> ['TMP_1615'])(Agent.State) := phi(["agent_1 (-> ['TMP_1615'])"])
_hash_1(bytes32) := phi(['hash_1'])
_value_1(uint256) := phi(['REF_688'])
FEE_BIPS_3(bytes32) := phi(['FEE_BIPS_2', 'FEE_BIPS_0'])
POOL_FEE_SHARE_BIPS_3(bytes32) := phi(['POOL_FEE_SHARE_BIPS_2', 'POOL_FEE_SHARE_BIPS_0'])
REDEMPTION_POOL_FEE_SHARE_BIPS_3(bytes32) := phi(['REDEMPTION_POOL_FEE_SHARE_BIPS_2', 'REDEMPTION_POOL_FEE_SHARE_BIPS_0'])
MINTING_VAULT_COLLATERAL_RATIO_BIPS_3(bytes32) := phi(['MINTING_VAULT_COLLATERAL_RATIO_BIPS_2', 'MINTING_VAULT_COLLATERAL_RATIO_BIPS_0'])
MINTING_POOL_COLLATERAL_RATIO_BIPS_3(bytes32) := phi(['MINTING_POOL_COLLATERAL_RATIO_BIPS_0', 'MINTING_POOL_COLLATERAL_RATIO_BIPS_2'])
BUY_FASSET_BY_AGENT_FACTOR_BIPS_3(bytes32) := phi(['BUY_FASSET_BY_AGENT_FACTOR_BIPS_0', 'BUY_FASSET_BY_AGENT_FACTOR_BIPS_2'])
POOL_EXIT_COLLATERAL_RATIO_BIPS_3(bytes32) := phi(['POOL_EXIT_COLLATERAL_RATIO_BIPS_2', 'POOL_EXIT_COLLATERAL_RATIO_BIPS_0'])
 _hash == FEE_BIPS
TMP_1642(bool) = _hash_1 == FEE_BIPS_3
CONDITION TMP_1642
 AgentUpdates.setFeeBIPS(_agent,_value)
LIBRARY_CALL, dest:AgentUpdates, function:AgentUpdates.setFeeBIPS(Agent.State,uint256), arguments:["_agent_1 (-> ['TMP_1615'])", '_value_1'] 
 _hash == POOL_FEE_SHARE_BIPS
TMP_1644(bool) = _hash_1 == POOL_FEE_SHARE_BIPS_3
CONDITION TMP_1644
 AgentUpdates.setPoolFeeShareBIPS(_agent,_value)
LIBRARY_CALL, dest:AgentUpdates, function:AgentUpdates.setPoolFeeShareBIPS(Agent.State,uint256), arguments:["_agent_1 (-> ['TMP_1615'])", '_value_1'] 
 _hash == REDEMPTION_POOL_FEE_SHARE_BIPS
TMP_1646(bool) = _hash_1 == REDEMPTION_POOL_FEE_SHARE_BIPS_3
CONDITION TMP_1646
 AgentUpdates.setRedemptionPoolFeeShareBIPS(_agent,_value)
LIBRARY_CALL, dest:AgentUpdates, function:AgentUpdates.setRedemptionPoolFeeShareBIPS(Agent.State,uint256), arguments:["_agent_1 (-> ['TMP_1615'])", '_value_1'] 
 _hash == MINTING_VAULT_COLLATERAL_RATIO_BIPS
TMP_1648(bool) = _hash_1 == MINTING_VAULT_COLLATERAL_RATIO_BIPS_3
CONDITION TMP_1648
 AgentUpdates.setMintingVaultCollateralRatioBIPS(_agent,_value)
LIBRARY_CALL, dest:AgentUpdates, function:AgentUpdates.setMintingVaultCollateralRatioBIPS(Agent.State,uint256), arguments:["_agent_1 (-> ['TMP_1615'])", '_value_1'] 
 _hash == MINTING_POOL_COLLATERAL_RATIO_BIPS
TMP_1650(bool) = _hash_1 == MINTING_POOL_COLLATERAL_RATIO_BIPS_3
CONDITION TMP_1650
 AgentUpdates.setMintingPoolCollateralRatioBIPS(_agent,_value)
LIBRARY_CALL, dest:AgentUpdates, function:AgentUpdates.setMintingPoolCollateralRatioBIPS(Agent.State,uint256), arguments:["_agent_1 (-> ['TMP_1615'])", '_value_1'] 
 _hash == BUY_FASSET_BY_AGENT_FACTOR_BIPS
TMP_1652(bool) = _hash_1 == BUY_FASSET_BY_AGENT_FACTOR_BIPS_3
CONDITION TMP_1652
 AgentUpdates.setBuyFAssetByAgentFactorBIPS(_agent,_value)
LIBRARY_CALL, dest:AgentUpdates, function:AgentUpdates.setBuyFAssetByAgentFactorBIPS(Agent.State,uint256), arguments:["_agent_1 (-> ['TMP_1615'])", '_value_1'] 
 _hash == POOL_EXIT_COLLATERAL_RATIO_BIPS
TMP_1654(bool) = _hash_1 == POOL_EXIT_COLLATERAL_RATIO_BIPS_3
CONDITION TMP_1654
 AgentUpdates.setPoolExitCollateralRatioBIPS(_agent,_value)
LIBRARY_CALL, dest:AgentUpdates, function:AgentUpdates.setPoolExitCollateralRatioBIPS(Agent.State,uint256), arguments:["_agent_1 (-> ['TMP_1615'])", '_value_1'] 
 assert(bool)(false)
TMP_1656(None) = SOLIDITY_CALL assert(bool)(False)
```
#### AgentSettingsFacet._getAndCheckHash(string) [PRIVATE]
```slithir
_name_1(string) := phi(['_name_1', '_name_1', '_name_1'])
FEE_BIPS_5(bytes32) := phi(['FEE_BIPS_2', 'FEE_BIPS_0'])
POOL_FEE_SHARE_BIPS_5(bytes32) := phi(['POOL_FEE_SHARE_BIPS_2', 'POOL_FEE_SHARE_BIPS_0'])
REDEMPTION_POOL_FEE_SHARE_BIPS_5(bytes32) := phi(['REDEMPTION_POOL_FEE_SHARE_BIPS_2', 'REDEMPTION_POOL_FEE_SHARE_BIPS_0'])
MINTING_VAULT_COLLATERAL_RATIO_BIPS_5(bytes32) := phi(['MINTING_VAULT_COLLATERAL_RATIO_BIPS_2', 'MINTING_VAULT_COLLATERAL_RATIO_BIPS_0'])
MINTING_POOL_COLLATERAL_RATIO_BIPS_5(bytes32) := phi(['MINTING_POOL_COLLATERAL_RATIO_BIPS_0', 'MINTING_POOL_COLLATERAL_RATIO_BIPS_2'])
BUY_FASSET_BY_AGENT_FACTOR_BIPS_5(bytes32) := phi(['BUY_FASSET_BY_AGENT_FACTOR_BIPS_0', 'BUY_FASSET_BY_AGENT_FACTOR_BIPS_2'])
POOL_EXIT_COLLATERAL_RATIO_BIPS_4(bytes32) := phi(['POOL_EXIT_COLLATERAL_RATIO_BIPS_2', 'POOL_EXIT_COLLATERAL_RATIO_BIPS_0'])
 hash = keccak256(bytes)(bytes(_name))
TMP_1668 = CONVERT _name_1 to bytes
TMP_1669(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_1668)
hash_1(bytes32) := TMP_1669(bytes32)
 settingNameValid = hash == FEE_BIPS || hash == POOL_FEE_SHARE_BIPS || hash == REDEMPTION_POOL_FEE_SHARE_BIPS || hash == MINTING_VAULT_COLLATERAL_RATIO_BIPS || hash == MINTING_POOL_COLLATERAL_RATIO_BIPS || hash == BUY_FASSET_BY_AGENT_FACTOR_BIPS || hash == POOL_EXIT_COLLATERAL_RATIO_BIPS
TMP_1670(bool) = hash_1 == FEE_BIPS_5
TMP_1671(bool) = hash_1 == POOL_FEE_SHARE_BIPS_5
TMP_1672(bool) = TMP_1670 || TMP_1671
TMP_1673(bool) = hash_1 == REDEMPTION_POOL_FEE_SHARE_BIPS_5
TMP_1674(bool) = TMP_1672 || TMP_1673
TMP_1675(bool) = hash_1 == MINTING_VAULT_COLLATERAL_RATIO_BIPS_5
TMP_1676(bool) = TMP_1674 || TMP_1675
TMP_1677(bool) = hash_1 == MINTING_POOL_COLLATERAL_RATIO_BIPS_5
TMP_1678(bool) = TMP_1676 || TMP_1677
TMP_1679(bool) = hash_1 == BUY_FASSET_BY_AGENT_FACTOR_BIPS_5
TMP_1680(bool) = TMP_1678 || TMP_1679
TMP_1681(bool) = hash_1 == POOL_EXIT_COLLATERAL_RATIO_BIPS_4
TMP_1682(bool) = TMP_1680 || TMP_1681
settingNameValid_1(bool) := TMP_1682(bool)
 require(bool,error)(settingNameValid,revert InvalidSettingName()())
TMP_1683(None) = SOLIDITY_CALL revert InvalidSettingName()()
TMP_1684(None) = SOLIDITY_CALL require(bool,error)(settingNameValid_1,TMP_1683)
 hash
RETURN hash_1
```
#### AgentSettingsFacet._getTimelock(bytes32) [PRIVATE]
```slithir
_hash_1(bytes32) := phi(['hash_1'])
FEE_BIPS_4(bytes32) := phi(['FEE_BIPS_2', 'FEE_BIPS_0'])
POOL_FEE_SHARE_BIPS_4(bytes32) := phi(['POOL_FEE_SHARE_BIPS_2', 'POOL_FEE_SHARE_BIPS_0'])
REDEMPTION_POOL_FEE_SHARE_BIPS_4(bytes32) := phi(['REDEMPTION_POOL_FEE_SHARE_BIPS_2', 'REDEMPTION_POOL_FEE_SHARE_BIPS_0'])
MINTING_VAULT_COLLATERAL_RATIO_BIPS_4(bytes32) := phi(['MINTING_VAULT_COLLATERAL_RATIO_BIPS_2', 'MINTING_VAULT_COLLATERAL_RATIO_BIPS_0'])
MINTING_POOL_COLLATERAL_RATIO_BIPS_4(bytes32) := phi(['MINTING_POOL_COLLATERAL_RATIO_BIPS_0', 'MINTING_POOL_COLLATERAL_RATIO_BIPS_2'])
BUY_FASSET_BY_AGENT_FACTOR_BIPS_4(bytes32) := phi(['BUY_FASSET_BY_AGENT_FACTOR_BIPS_0', 'BUY_FASSET_BY_AGENT_FACTOR_BIPS_2'])
 settings = Globals.getSettings()
TMP_1657(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_1657'])(AssetManagerSettings.Data) := TMP_1657(AssetManagerSettings.Data)
 _hash == FEE_BIPS || _hash == POOL_FEE_SHARE_BIPS || _hash == REDEMPTION_POOL_FEE_SHARE_BIPS || _hash == BUY_FASSET_BY_AGENT_FACTOR_BIPS
TMP_1658(bool) = _hash_1 == FEE_BIPS_4
TMP_1659(bool) = _hash_1 == POOL_FEE_SHARE_BIPS_4
TMP_1660(bool) = TMP_1658 || TMP_1659
TMP_1661(bool) = _hash_1 == REDEMPTION_POOL_FEE_SHARE_BIPS_4
TMP_1662(bool) = TMP_1660 || TMP_1661
TMP_1663(bool) = _hash_1 == BUY_FASSET_BY_AGENT_FACTOR_BIPS_4
TMP_1664(bool) = TMP_1662 || TMP_1663
CONDITION TMP_1664
 settings.agentFeeChangeTimelockSeconds
REF_710(uint64) -> settings_1 (-> ['TMP_1657']).agentFeeChangeTimelockSeconds
RETURN REF_710
 _hash == MINTING_VAULT_COLLATERAL_RATIO_BIPS || _hash == MINTING_POOL_COLLATERAL_RATIO_BIPS
TMP_1665(bool) = _hash_1 == MINTING_VAULT_COLLATERAL_RATIO_BIPS_4
TMP_1666(bool) = _hash_1 == MINTING_POOL_COLLATERAL_RATIO_BIPS_4
TMP_1667(bool) = TMP_1665 || TMP_1666
CONDITION TMP_1667
 settings.agentMintingCRChangeTimelockSeconds
REF_711(uint64) -> settings_1 (-> ['TMP_1657']).agentMintingCRChangeTimelockSeconds
RETURN REF_711
 settings.poolExitCRChangeTimelockSeconds
REF_712(uint64) -> settings_1 (-> ['TMP_1657']).poolExitCRChangeTimelockSeconds
RETURN REF_712
```
#### AgentSettingsFacet.announceAgentSettingUpdate(address,string,uint256) [EXTERNAL]
```slithir
 agent = Agent.get(_agentVault)
TMP_1606(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_1606'])(Agent.State) := TMP_1606(Agent.State)
 hash = _getAndCheckHash(_name)
TMP_1607(bytes32) = INTERNAL_CALL, AgentSettingsFacet._getAndCheckHash(string)(_name_1)
hash_1(bytes32) := TMP_1607(bytes32)
 _updateAllowedAt = block.timestamp + _getTimelock(hash)
TMP_1608(uint64) = INTERNAL_CALL, AgentSettingsFacet._getTimelock(bytes32)(hash_1)
TMP_1609(uint256) = block.timestamp (c)+ TMP_1608
_updateAllowedAt_1(uint256) := TMP_1609(uint256)
 agent.settingUpdates[hash] = Agent.SettingUpdate({value:_value.toUint128(),validAt:_updateAllowedAt.toUint64()})
REF_674(mapping(bytes32 => Agent.SettingUpdate)) -> agent_1 (-> ['TMP_1606']).settingUpdates
REF_675(Agent.SettingUpdate) -> REF_674[hash_1]
TMP_1610(uint128) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint128(uint256), arguments:['_value_1'] 
TMP_1611(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['_updateAllowedAt_1'] 
TMP_1612(Agent.SettingUpdate) = new SettingUpdate(TMP_1610,TMP_1611)
agent_2 (-> ['TMP_1606'])(Agent.State) := phi(["agent_1 (-> ['TMP_1606'])"])
REF_675(Agent.SettingUpdate) (->agent_2 (-> ['TMP_1606'])) := TMP_1612(Agent.SettingUpdate)
TMP_1606(Agent.State) := phi(["agent_2 (-> ['TMP_1606'])"])
 IAssetManagerEvents.AgentSettingChangeAnnounced(_agentVault,_name,_value,_updateAllowedAt)
Emit AgentSettingChangeAnnounced(_agentVault_1,_name_1,_value_1,_updateAllowedAt_1)
 onlyAgentVaultOwner(_agentVault)
MODIFIER_CALL, AssetManagerBase.onlyAgentVaultOwner(address)(_agentVault_1)
 _updateAllowedAt
RETURN _updateAllowedAt_1
```
#### AgentSettingsFacet.executeAgentSettingUpdate(address,string) [EXTERNAL]
```slithir
 agent = Agent.get(_agentVault)
TMP_1615(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_1615'])(Agent.State) := TMP_1615(Agent.State)
 hash = _getAndCheckHash(_name)
TMP_1616(bytes32) = INTERNAL_CALL, AgentSettingsFacet._getAndCheckHash(string)(_name_1)
hash_1(bytes32) := TMP_1616(bytes32)
 update = agent.settingUpdates[hash]
REF_681(mapping(bytes32 => Agent.SettingUpdate)) -> agent_1 (-> ['TMP_1615']).settingUpdates
REF_682(Agent.SettingUpdate) -> REF_681[hash_1]
update_1 (-> ['agent'])(Agent.SettingUpdate) := REF_682(Agent.SettingUpdate)
 require(bool,error)(update.validAt != 0,revert NoPendingUpdate()())
REF_683(uint64) -> update_1 (-> ['agent']).validAt
TMP_1617(bool) = REF_683 != 0
TMP_1618(None) = SOLIDITY_CALL revert NoPendingUpdate()()
TMP_1619(None) = SOLIDITY_CALL require(bool,error)(TMP_1617,TMP_1618)
 require(bool,error)(update.validAt <= block.timestamp,revert UpdateNotValidYet()())
REF_684(uint64) -> update_1 (-> ['agent']).validAt
TMP_1620(bool) = REF_684 <= block.timestamp
TMP_1621(None) = SOLIDITY_CALL revert UpdateNotValidYet()()
TMP_1622(None) = SOLIDITY_CALL require(bool,error)(TMP_1620,TMP_1621)
 settings = Globals.getSettings()
TMP_1623(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
settings_1 (-> ['TMP_1623'])(AssetManagerSettings.Data) := TMP_1623(AssetManagerSettings.Data)
 require(bool,error)(update.validAt + settings.agentTimelockedOperationWindowSeconds >= block.timestamp,revert UpdateNotValidAnymore()())
REF_686(uint64) -> update_1 (-> ['agent']).validAt
REF_687(uint64) -> settings_1 (-> ['TMP_1623']).agentTimelockedOperationWindowSeconds
TMP_1624(uint64) = REF_686 (c)+ REF_687
TMP_1625(bool) = TMP_1624 >= block.timestamp
TMP_1626(None) = SOLIDITY_CALL revert UpdateNotValidAnymore()()
TMP_1627(None) = SOLIDITY_CALL require(bool,error)(TMP_1625,TMP_1626)
 _executeUpdate(agent,hash,update.value)
REF_688(uint128) -> update_1 (-> ['agent']).value
INTERNAL_CALL, AgentSettingsFacet._executeUpdate(Agent.State,bytes32,uint256)(agent_1 (-> ['TMP_1615']),hash_1,REF_688)
 IAssetManagerEvents.AgentSettingChanged(_agentVault,_name,update.value)
REF_690(uint128) -> update_1 (-> ['agent']).value
Emit AgentSettingChanged(_agentVault_1,_name_1,REF_690)
 delete agent.settingUpdates[hash]
REF_691(mapping(bytes32 => Agent.SettingUpdate)) -> agent_1 (-> ['TMP_1615']).settingUpdates
REF_692(Agent.SettingUpdate) -> REF_691[hash_1]
REF_691 = delete REF_692 
 onlyAgentVaultOwner(_agentVault)
MODIFIER_CALL, AssetManagerBase.onlyAgentVaultOwner(address)(_agentVault_1)
```
#### AgentSettingsFacet.getAgentSetting(address,string) [EXTERNAL]
```slithir
FEE_BIPS_1(bytes32) := phi(['FEE_BIPS_2', 'FEE_BIPS_0'])
POOL_FEE_SHARE_BIPS_1(bytes32) := phi(['POOL_FEE_SHARE_BIPS_2', 'POOL_FEE_SHARE_BIPS_0'])
REDEMPTION_POOL_FEE_SHARE_BIPS_1(bytes32) := phi(['REDEMPTION_POOL_FEE_SHARE_BIPS_2', 'REDEMPTION_POOL_FEE_SHARE_BIPS_0'])
MINTING_VAULT_COLLATERAL_RATIO_BIPS_1(bytes32) := phi(['MINTING_VAULT_COLLATERAL_RATIO_BIPS_2', 'MINTING_VAULT_COLLATERAL_RATIO_BIPS_0'])
MINTING_POOL_COLLATERAL_RATIO_BIPS_1(bytes32) := phi(['MINTING_POOL_COLLATERAL_RATIO_BIPS_0', 'MINTING_POOL_COLLATERAL_RATIO_BIPS_2'])
BUY_FASSET_BY_AGENT_FACTOR_BIPS_1(bytes32) := phi(['BUY_FASSET_BY_AGENT_FACTOR_BIPS_0', 'BUY_FASSET_BY_AGENT_FACTOR_BIPS_2'])
POOL_EXIT_COLLATERAL_RATIO_BIPS_1(bytes32) := phi(['POOL_EXIT_COLLATERAL_RATIO_BIPS_2', 'POOL_EXIT_COLLATERAL_RATIO_BIPS_0'])
 agent = Agent.get(_agentVault)
TMP_1631(Agent.State) = LIBRARY_CALL, dest:Agent, function:Agent.get(address), arguments:['_agentVault_1'] 
agent_1 (-> ['TMP_1631'])(Agent.State) := TMP_1631(Agent.State)
 hash = _getAndCheckHash(_name)
TMP_1632(bytes32) = INTERNAL_CALL, AgentSettingsFacet._getAndCheckHash(string)(_name_1)
hash_1(bytes32) := TMP_1632(bytes32)
 hash == FEE_BIPS
TMP_1633(bool) = hash_1 == FEE_BIPS_2
CONDITION TMP_1633
 agent.feeBIPS
REF_694(uint16) -> agent_1 (-> ['TMP_1631']).feeBIPS
RETURN REF_694
 hash == POOL_FEE_SHARE_BIPS
TMP_1634(bool) = hash_1 == POOL_FEE_SHARE_BIPS_2
CONDITION TMP_1634
 agent.poolFeeShareBIPS
REF_695(uint16) -> agent_1 (-> ['TMP_1631']).poolFeeShareBIPS
RETURN REF_695
 hash == REDEMPTION_POOL_FEE_SHARE_BIPS
TMP_1635(bool) = hash_1 == REDEMPTION_POOL_FEE_SHARE_BIPS_2
CONDITION TMP_1635
 agent.redemptionPoolFeeShareBIPS
REF_696(uint16) -> agent_1 (-> ['TMP_1631']).redemptionPoolFeeShareBIPS
RETURN REF_696
 hash == MINTING_VAULT_COLLATERAL_RATIO_BIPS
TMP_1636(bool) = hash_1 == MINTING_VAULT_COLLATERAL_RATIO_BIPS_2
CONDITION TMP_1636
 agent.mintingVaultCollateralRatioBIPS
REF_697(uint32) -> agent_1 (-> ['TMP_1631']).mintingVaultCollateralRatioBIPS
RETURN REF_697
 hash == MINTING_POOL_COLLATERAL_RATIO_BIPS
TMP_1637(bool) = hash_1 == MINTING_POOL_COLLATERAL_RATIO_BIPS_2
CONDITION TMP_1637
 agent.mintingPoolCollateralRatioBIPS
REF_698(uint32) -> agent_1 (-> ['TMP_1631']).mintingPoolCollateralRatioBIPS
RETURN REF_698
 hash == BUY_FASSET_BY_AGENT_FACTOR_BIPS
TMP_1638(bool) = hash_1 == BUY_FASSET_BY_AGENT_FACTOR_BIPS_2
CONDITION TMP_1638
 agent.buyFAssetByAgentFactorBIPS
REF_699(uint16) -> agent_1 (-> ['TMP_1631']).buyFAssetByAgentFactorBIPS
RETURN REF_699
 hash == POOL_EXIT_COLLATERAL_RATIO_BIPS
TMP_1639(bool) = hash_1 == POOL_EXIT_COLLATERAL_RATIO_BIPS_2
CONDITION TMP_1639
 agent.collateralPool.exitCollateralRatioBIPS()
REF_700(IICollateralPool) -> agent_1 (-> ['TMP_1631']).collateralPool
TMP_1640(uint32) = HIGH_LEVEL_CALL, dest:REF_700(IICollateralPool), function:exitCollateralRatioBIPS, arguments:[]  
RETURN TMP_1640
 assert(bool)(false)
TMP_1641(None) = SOLIDITY_CALL assert(bool)(False)
 _value
RETURN _value_0
```

#### AgentUpdates.setBuyFAssetByAgentFactorBIPS(Agent.State,uint256) [INTERNAL]
```slithir
 require(bool,error)(_buyFAssetByAgentFactorBIPS <= SafePct.MAX_BIPS,revert ValueTooHigh()())
REF_2961(uint256) -> SafePct.MAX_BIPS
TMP_4436(bool) = _buyFAssetByAgentFactorBIPS_1 <= REF_2961
TMP_4437(None) = SOLIDITY_CALL revert ValueTooHigh()()
TMP_4438(None) = SOLIDITY_CALL require(bool,error)(TMP_4436,TMP_4437)
 require(bool,error)(_buyFAssetByAgentFactorBIPS >= 9000,revert ValueTooLow()())
TMP_4439(bool) = _buyFAssetByAgentFactorBIPS_1 >= 9000
TMP_4440(None) = SOLIDITY_CALL revert ValueTooLow()()
TMP_4441(None) = SOLIDITY_CALL require(bool,error)(TMP_4439,TMP_4440)
 _agent.buyFAssetByAgentFactorBIPS = _buyFAssetByAgentFactorBIPS.toUint16()
REF_2962(uint16) -> _agent_1 (-> []).buyFAssetByAgentFactorBIPS
TMP_4442(uint16) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint16(uint256), arguments:['_buyFAssetByAgentFactorBIPS_1'] 
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_2962(uint16) (->_agent_2 (-> [])) := TMP_4442(uint16)
```
#### AgentUpdates.setFeeBIPS(Agent.State,uint256) [INTERNAL]
```slithir
 require(bool,error)(_feeBIPS <= SafePct.MAX_BIPS,revert FeeTooHigh()())
REF_2952(uint256) -> SafePct.MAX_BIPS
TMP_4424(bool) = _feeBIPS_1 <= REF_2952
TMP_4425(None) = SOLIDITY_CALL revert FeeTooHigh()()
TMP_4426(None) = SOLIDITY_CALL require(bool,error)(TMP_4424,TMP_4425)
 _agent.feeBIPS = _feeBIPS.toUint16()
REF_2953(uint16) -> _agent_1 (-> []).feeBIPS
TMP_4427(uint16) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint16(uint256), arguments:['_feeBIPS_1'] 
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_2953(uint16) (->_agent_2 (-> [])) := TMP_4427(uint16)
```
#### AgentUpdates.setMintingPoolCollateralRatioBIPS(Agent.State,uint256) [INTERNAL]
```slithir
 collateral = Agents.getPoolCollateral(_agent)
TMP_4419(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getPoolCollateral(Agent.State), arguments:['_agent_1 (-> [])'] 
collateral_1 (-> ['TMP_4419'])(CollateralTypeInt.Data) := TMP_4419(CollateralTypeInt.Data)
 require(bool,error)(_mintingPoolCollateralRatioBIPS >= collateral.minCollateralRatioBIPS,revert CollateralRatioTooSmall()())
REF_2949(uint32) -> collateral_1 (-> ['TMP_4419']).minCollateralRatioBIPS
TMP_4420(bool) = _mintingPoolCollateralRatioBIPS_1 >= REF_2949
TMP_4421(None) = SOLIDITY_CALL revert CollateralRatioTooSmall()()
TMP_4422(None) = SOLIDITY_CALL require(bool,error)(TMP_4420,TMP_4421)
 _agent.mintingPoolCollateralRatioBIPS = _mintingPoolCollateralRatioBIPS.toUint32()
REF_2950(uint32) -> _agent_1 (-> []).mintingPoolCollateralRatioBIPS
TMP_4423(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['_mintingPoolCollateralRatioBIPS_1'] 
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_2950(uint32) (->_agent_2 (-> [])) := TMP_4423(uint32)
```
#### AgentUpdates.setMintingVaultCollateralRatioBIPS(Agent.State,uint256) [INTERNAL]
```slithir
 collateral = Agents.getVaultCollateral(_agent)
TMP_4414(CollateralTypeInt.Data) = LIBRARY_CALL, dest:Agents, function:Agents.getVaultCollateral(Agent.State), arguments:['_agent_1 (-> [])'] 
collateral_1 (-> ['TMP_4414'])(CollateralTypeInt.Data) := TMP_4414(CollateralTypeInt.Data)
 require(bool,error)(_mintingVaultCollateralRatioBIPS >= collateral.minCollateralRatioBIPS,revert CollateralRatioTooSmall()())
REF_2945(uint32) -> collateral_1 (-> ['TMP_4414']).minCollateralRatioBIPS
TMP_4415(bool) = _mintingVaultCollateralRatioBIPS_1 >= REF_2945
TMP_4416(None) = SOLIDITY_CALL revert CollateralRatioTooSmall()()
TMP_4417(None) = SOLIDITY_CALL require(bool,error)(TMP_4415,TMP_4416)
 _agent.mintingVaultCollateralRatioBIPS = _mintingVaultCollateralRatioBIPS.toUint32()
REF_2946(uint32) -> _agent_1 (-> []).mintingVaultCollateralRatioBIPS
TMP_4418(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['_mintingVaultCollateralRatioBIPS_1'] 
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_2946(uint32) (->_agent_2 (-> [])) := TMP_4418(uint32)
```

#### AgentUpdates.setPoolFeeShareBIPS(Agent.State,uint256) [INTERNAL]
```slithir
 require(bool,error)(_poolFeeShareBIPS <= SafePct.MAX_BIPS,revert ValueTooHigh()())
REF_2955(uint256) -> SafePct.MAX_BIPS
TMP_4428(bool) = _poolFeeShareBIPS_1 <= REF_2955
TMP_4429(None) = SOLIDITY_CALL revert ValueTooHigh()()
TMP_4430(None) = SOLIDITY_CALL require(bool,error)(TMP_4428,TMP_4429)
 _agent.poolFeeShareBIPS = _poolFeeShareBIPS.toUint16()
REF_2956(uint16) -> _agent_1 (-> []).poolFeeShareBIPS
TMP_4431(uint16) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint16(uint256), arguments:['_poolFeeShareBIPS_1'] 
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_2956(uint16) (->_agent_2 (-> [])) := TMP_4431(uint16)
```
#### AgentUpdates.setRedemptionPoolFeeShareBIPS(Agent.State,uint256) [INTERNAL]
```slithir
 require(bool,error)(_redemptionPoolFeeShareBIPS <= SafePct.MAX_BIPS,revert ValueTooHigh()())
REF_2958(uint256) -> SafePct.MAX_BIPS
TMP_4432(bool) = _redemptionPoolFeeShareBIPS_1 <= REF_2958
TMP_4433(None) = SOLIDITY_CALL revert ValueTooHigh()()
TMP_4434(None) = SOLIDITY_CALL require(bool,error)(TMP_4432,TMP_4433)
 _agent.redemptionPoolFeeShareBIPS = _redemptionPoolFeeShareBIPS.toUint16()
REF_2959(uint16) -> _agent_1 (-> []).redemptionPoolFeeShareBIPS
TMP_4435(uint16) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint16(uint256), arguments:['_redemptionPoolFeeShareBIPS_1'] 
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_2959(uint16) (->_agent_2 (-> [])) := TMP_4435(uint16)
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
#### SafeCast.toUint128(uint256) [INTERNAL]
```slithir
 require(bool,string)(value <= type()(uint128).max,SafeCast: value doesn't fit in 128 bits)
TMP_707(uint128) := 340282366920938463463374607431768211455(uint128)
TMP_708(bool) = value_1 <= TMP_707
TMP_709(None) = SOLIDITY_CALL require(bool,string)(TMP_708,SafeCast: value doesn't fit in 128 bits)
 uint128(value)
TMP_710 = CONVERT value_1 to uint128
RETURN TMP_710
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
#### SafeCast.toUint16(uint256) [INTERNAL]
```slithir
 require(bool,string)(value <= type()(uint16).max,SafeCast: value doesn't fit in 16 bits)
TMP_777(uint16) := 65535(uint16)
TMP_778(bool) = value_1 <= TMP_777
TMP_779(None) = SOLIDITY_CALL require(bool,string)(TMP_778,SafeCast: value doesn't fit in 16 bits)
 uint16(value)
TMP_780 = CONVERT value_1 to uint16
RETURN TMP_780
```
#### Agents.getPoolCollateral(Agent.State) [INTERNAL]
```slithir
 state = AssetManagerState.get()
TMP_4516(AssetManagerState.State) = LIBRARY_CALL, dest:AssetManagerState, function:AssetManagerState.get(), arguments:[] 
state_1 (-> ['TMP_4516'])(AssetManagerState.State) := TMP_4516(AssetManagerState.State)
 state.collateralTokens[_agent.poolCollateralIndex]
REF_3015(CollateralTypeInt.Data[]) -> state_1 (-> ['TMP_4516']).collateralTokens
REF_3016(uint16) -> _agent_1 (-> []).poolCollateralIndex
REF_3017(CollateralTypeInt.Data) -> REF_3015[REF_3016]
RETURN REF_3017
```
#### SafeCast.toUint32(uint256) [INTERNAL]
```slithir
 require(bool,string)(value <= type()(uint32).max,SafeCast: value doesn't fit in 32 bits)
TMP_767(uint32) := 4294967295(uint32)
TMP_768(bool) = value_1 <= TMP_767
TMP_769(None) = SOLIDITY_CALL require(bool,string)(TMP_768,SafeCast: value doesn't fit in 32 bits)
 uint32(value)
TMP_770 = CONVERT value_1 to uint32
RETURN TMP_770
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
#### IICollateralPool.setExitCollateralRatioBIPS(uint256) [EXTERNAL]
```slithir

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
