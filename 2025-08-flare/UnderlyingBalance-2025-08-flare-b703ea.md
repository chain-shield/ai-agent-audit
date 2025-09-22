




#### UnderlyingBalance.increaseBalance(Agent.State,uint256) [INTERNAL]
```slithir
 _agent.underlyingBalanceUBA += _balanceIncrease.toInt256().toInt128()
REF_3709(int128) -> _agent_1 (-> []).underlyingBalanceUBA
TMP_5287(int256) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toInt256(uint256), arguments:['_balanceIncrease_1'] 
TMP_5288(int128) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toInt128(int256), arguments:['TMP_5287'] 
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_3709(-> _agent_2 (-> [])) = REF_3709 (c)+ TMP_5288
 IAssetManagerEvents.UnderlyingBalanceChanged(_agent.vaultAddress(),_agent.underlyingBalanceUBA)
TMP_5289(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_2 (-> [])'] 
REF_3714(int128) -> _agent_2 (-> []).underlyingBalanceUBA
Emit UnderlyingBalanceChanged(TMP_5289,REF_3714)
```

#### UnderlyingBalance.updateBalance(Agent.State,int256) [INTERNAL]
```slithir
 newBalance = _agent.underlyingBalanceUBA + _balanceChange
REF_3699(int128) -> _agent_1 (-> []).underlyingBalanceUBA
TMP_5277(int128) = REF_3699 (c)+ _balanceChange_1
newBalance_1(int256) := TMP_5277(int128)
 requiredBalance = requiredUnderlyingUBA(_agent)
TMP_5278(uint256) = INTERNAL_CALL, UnderlyingBalance.requiredUnderlyingUBA(Agent.State)(_agent_1 (-> []))
requiredBalance_1(uint256) := TMP_5278(uint256)
 newBalance < requiredBalance.toInt256()
TMP_5279(int256) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toInt256(uint256), arguments:['requiredBalance_1'] 
TMP_5280(bool) = newBalance_1 < TMP_5279
CONDITION TMP_5280
 IAssetManagerEvents.UnderlyingBalanceTooLow(_agent.vaultAddress(),newBalance,requiredBalance)
TMP_5281(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_1 (-> [])'] 
Emit UnderlyingBalanceTooLow(TMP_5281,newBalance_1,requiredBalance_1)
 Liquidation.startFullLiquidation(_agent)
LIBRARY_CALL, dest:Liquidation, function:Liquidation.startFullLiquidation(Agent.State), arguments:['_agent_1 (-> [])'] 
 _agent.underlyingBalanceUBA = newBalance.toInt128()
REF_3704(int128) -> _agent_1 (-> []).underlyingBalanceUBA
TMP_5284(int128) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toInt128(int256), arguments:['newBalance_1'] 
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_3704(int128) (->_agent_2 (-> [])) := TMP_5284(int128)
 IAssetManagerEvents.UnderlyingBalanceChanged(_agent.vaultAddress(),_agent.underlyingBalanceUBA)
TMP_5285(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_2 (-> [])'] 
REF_3708(int128) -> _agent_2 (-> []).underlyingBalanceUBA
Emit UnderlyingBalanceChanged(TMP_5285,REF_3708)
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
#### SafeCast.toInt128(int256) [INTERNAL]
```slithir
 downcasted = int128(value)
TMP_834 = CONVERT value_1 to int128
downcasted_1(int128) := TMP_834(int128)
 require(bool,string)(downcasted == value,SafeCast: value doesn't fit in 128 bits)
TMP_835(bool) = downcasted_1 == value_1
TMP_836(None) = SOLIDITY_CALL require(bool,string)(TMP_835,SafeCast: value doesn't fit in 128 bits)
 downcasted
RETURN downcasted_1
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
#### Liquidation.startFullLiquidation(Agent.State) [INTERNAL]
```slithir
 _agent.status == Agent.Status.FULL_LIQUIDATION || _agent.status == Agent.Status.DESTROYING
REF_3245(Agent.Status) -> _agent_1 (-> []).status
REF_3246(Agent.Status) -> Status.FULL_LIQUIDATION
TMP_4741(bool) = REF_3245 == REF_3246
REF_3247(Agent.Status) -> _agent_1 (-> []).status
REF_3248(Agent.Status) -> Status.DESTROYING
TMP_4742(bool) = REF_3247 == REF_3248
TMP_4743(bool) = TMP_4741 || TMP_4742
CONDITION TMP_4743
 _agent.liquidationStartedAt == 0
REF_3249(uint64) -> _agent_1 (-> []).liquidationStartedAt
TMP_4744(bool) = REF_3249 == 0
CONDITION TMP_4744
 _agent.liquidationStartedAt = block.timestamp.toUint64()
REF_3250(uint64) -> _agent_1 (-> []).liquidationStartedAt
TMP_4745(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['block.timestamp'] 
_agent_2 (-> [])(Agent.State) := phi(['_agent_1 (-> [])'])
REF_3250(uint64) (->_agent_2 (-> [])) := TMP_4745(uint64)
_agent_3 (-> [])(Agent.State) := phi(['_agent_2 (-> [])', '_agent_1 (-> [])'])
 _agent.status = Agent.Status.FULL_LIQUIDATION
REF_3252(Agent.Status) -> _agent_3 (-> []).status
REF_3253(Agent.Status) -> Status.FULL_LIQUIDATION
_agent_4 (-> [])(Agent.State) := phi(['_agent_3 (-> [])'])
REF_3252(Agent.Status) (->_agent_4 (-> [])) := REF_3253(Agent.Status)
 IAssetManagerEvents.FullLiquidationStarted(_agent.vaultAddress(),block.timestamp)
TMP_4746(address) = LIBRARY_CALL, dest:Agent, function:Agent.vaultAddress(Agent.State), arguments:['_agent_4 (-> [])'] 
Emit FullLiquidationStarted(TMP_4746,block.timestamp)
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
