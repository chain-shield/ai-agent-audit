



### Storage layout (AgentVeToken) 

```text
founder address
assetToken address
agentNft address
matureAt uint256
canStake bool
initialLock uint256
_balanceCheckpoints mapping(address => Checkpoints.Trace208)
locked bool

```




#### AgentVeToken._update(address,address,uint256) [INTERNAL]
```slithir
from_1(address) := phi(['from_1', 'account_1', 'TMP_13849'])
to_1(address) := phi(['to_1', 'TMP_13855', 'account_1'])
value_1(uint256) := phi(['value_1', 'value_1', 'value_1'])
 super._update(from,to,value)
INTERNAL_CALL, ERC20VotesUpgradeable._update(address,address,uint256)(from_1,to_1,value_1)
```
#### AgentVeToken.approve(address,uint256) [PUBLIC]
```slithir
 revert(string)(Approve not supported)
TMP_13946(None) = SOLIDITY_CALL revert(string)(Approve not supported)
```
#### AgentVeToken.constructor() [PUBLIC]
```slithir
 _disableInitializers()
INTERNAL_CALL, Initializable._disableInitializers()()
```
#### AgentVeToken.getPastBalanceOf(address,uint256) [PUBLIC]
```slithir
_balanceCheckpoints_22(mapping(address => Checkpoints.Trace208)) := phi(['_balanceCheckpoints_13', '_balanceCheckpoints_23', '_balanceCheckpoints_21', '_balanceCheckpoints_0'])
 currentTimepoint = clock()
TMP_13939(uint48) = INTERNAL_CALL, VotesUpgradeable.clock()()
currentTimepoint_1(uint48) := TMP_13939(uint48)
 timepoint >= currentTimepoint
TMP_13940(bool) = timepoint_1 >= currentTimepoint_1
CONDITION TMP_13940
 revert ERC5805FutureLookup(uint256,uint48)(timepoint,currentTimepoint)
TMP_13941(None) = SOLIDITY_CALL revert ERC5805FutureLookup(uint256,uint48)(timepoint_1,currentTimepoint_1)
 _balanceCheckpoints[account].upperLookupRecent(SafeCast.toUint48(timepoint))
REF_5676(Checkpoints.Trace208) -> _balanceCheckpoints_23[account_1]
TMP_13942(uint48) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint48(uint256), arguments:['timepoint_1'] 
TMP_13943(uint208) = LIBRARY_CALL, dest:Checkpoints, function:Checkpoints.upperLookupRecent(Checkpoints.Trace208,uint48), arguments:['REF_5676', 'TMP_13942'] 
RETURN TMP_13943
```
#### IAgentVeToken.getPastDelegates(address,uint256) [PUBLIC]
```slithir

```
#### AgentVeToken.initialize(string,string,address,address,uint256,address,bool) [EXTERNAL]
```slithir
 __ERC20_init(_name,_symbol)
INTERNAL_CALL, ERC20Upgradeable.__ERC20_init(string,string)(_name_1,_symbol_1)
 __ERC20Votes_init()
INTERNAL_CALL, ERC20VotesUpgradeable.__ERC20Votes_init()()
 founder = _founder
founder_1(address) := _founder_1(address)
 matureAt = _matureAt
matureAt_1(uint256) := _matureAt_1(uint256)
 assetToken = _assetToken
assetToken_1(address) := _assetToken_1(address)
 agentNft = _agentNft
agentNft_1(address) := _agentNft_1(address)
 canStake = _canStake
canStake_1(bool) := _canStake_1(bool)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### AgentVeToken.setCanStake(bool) [PUBLIC]
```slithir
founder_2(address) := phi(['founder_0', 'founder_8', 'founder_1', 'founder_3'])
 require(bool,string)(_msgSender() == founder,Not founder)
TMP_13913(address) = INTERNAL_CALL, ContextUpgradeable._msgSender()()
TMP_13914(bool) = TMP_13913 == founder_3
TMP_13915(None) = SOLIDITY_CALL require(bool,string)(TMP_13914,Not founder)
 canStake = _canStake
canStake_4(bool) := _canStake_1(bool)
```
#### AgentVeToken.setMatureAt(uint256) [PUBLIC]
```slithir
agentNft_7(address) := phi(['agentNft_1', 'agentNft_0', 'agentNft_6', 'agentNft_9'])
 ADMIN_ROLE = keccak256(bytes)(ADMIN_ROLE)
TMP_13916(bytes32) = SOLIDITY_CALL keccak256(bytes)(ADMIN_ROLE)
ADMIN_ROLE_1(bytes32) := TMP_13916(bytes32)
 require(bool,string)(IAccessControl(agentNft).hasRole(ADMIN_ROLE,_msgSender()),Not admin)
TMP_13917 = CONVERT agentNft_7 to IAccessControl
TMP_13918(address) = INTERNAL_CALL, ContextUpgradeable._msgSender()()
TMP_13919(bool) = HIGH_LEVEL_CALL, dest:TMP_13917(IAccessControl), function:hasRole, arguments:['ADMIN_ROLE_1', 'TMP_13918']  
agentNft_9(address) := phi(['agentNft_1', 'agentNft_6', 'agentNft_9', 'agentNft_8'])
TMP_13920(None) = SOLIDITY_CALL require(bool,string)(TMP_13919,Not admin)
 matureAt = _matureAt
matureAt_2(uint256) := _matureAt_1(uint256)
```
#### AgentVeToken.stake(uint256,address,address) [PUBLIC]
```slithir
assetToken_2(address) := phi(['assetToken_0', 'assetToken_10', 'assetToken_18', 'assetToken_1'])
agentNft_2(address) := phi(['agentNft_1', 'agentNft_0', 'agentNft_6', 'agentNft_9'])
canStake_2(bool) := phi(['canStake_0', 'canStake_1', 'canStake_4', 'canStake_3'])
_balanceCheckpoints_1(mapping(address => Checkpoints.Trace208)) := phi(['_balanceCheckpoints_13', '_balanceCheckpoints_23', '_balanceCheckpoints_21', '_balanceCheckpoints_0'])
 require(bool,string)(canStake || totalSupply() == 0,Staking is disabled for private agent)
TMP_13880(uint256) = INTERNAL_CALL, ERC20Upgradeable.totalSupply()()
TMP_13881(bool) = TMP_13880 == 0
TMP_13882(bool) = canStake_3 || TMP_13881
TMP_13883(None) = SOLIDITY_CALL require(bool,string)(TMP_13882,Staking is disabled for private agent)
 sender = _msgSender()
TMP_13884(address) = INTERNAL_CALL, ContextUpgradeable._msgSender()()
sender_1(address) := TMP_13884(address)
 require(bool,string)(amount > 0,Cannot stake 0)
TMP_13885(bool) = amount_1 > 0
TMP_13886(None) = SOLIDITY_CALL require(bool,string)(TMP_13885,Cannot stake 0)
 require(bool,string)(IERC20(assetToken).balanceOf(sender) >= amount,Insufficient asset token balance)
TMP_13887 = CONVERT assetToken_4 to IERC20
TMP_13888(uint256) = HIGH_LEVEL_CALL, dest:TMP_13887(IERC20), function:balanceOf, arguments:['sender_1']  
assetToken_5(address) := phi(['assetToken_1', 'assetToken_4', 'assetToken_18', 'assetToken_10'])
agentNft_5(address) := phi(['agentNft_1', 'agentNft_6', 'agentNft_9', 'agentNft_4'])
_balanceCheckpoints_4(mapping(address => Checkpoints.Trace208)) := phi(['_balanceCheckpoints_13', '_balanceCheckpoints_23', '_balanceCheckpoints_3', '_balanceCheckpoints_21'])
TMP_13889(bool) = TMP_13888 >= amount_1
TMP_13890(None) = SOLIDITY_CALL require(bool,string)(TMP_13889,Insufficient asset token balance)
 require(bool,string)(IERC20(assetToken).allowance(sender,address(this)) >= amount,Insufficient asset token allowance)
TMP_13891 = CONVERT assetToken_5 to IERC20
TMP_13892 = CONVERT this to address
TMP_13893(uint256) = HIGH_LEVEL_CALL, dest:TMP_13891(IERC20), function:allowance, arguments:['sender_1', 'TMP_13892']  
assetToken_6(address) := phi(['assetToken_1', 'assetToken_18', 'assetToken_10', 'assetToken_5'])
agentNft_6(address) := phi(['agentNft_1', 'agentNft_5', 'agentNft_6', 'agentNft_9'])
_balanceCheckpoints_5(mapping(address => Checkpoints.Trace208)) := phi(['_balanceCheckpoints_13', '_balanceCheckpoints_4', '_balanceCheckpoints_23', '_balanceCheckpoints_21'])
TMP_13894(bool) = TMP_13893 >= amount_1
TMP_13895(None) = SOLIDITY_CALL require(bool,string)(TMP_13894,Insufficient asset token allowance)
 registry = IAgentNft(agentNft)
TMP_13896 = CONVERT agentNft_6 to IAgentNft
registry_1(IAgentNft) := TMP_13896(IAgentNft)
 virtualId = registry.stakingTokenToVirtualId(address(this))
TMP_13897 = CONVERT this to address
TMP_13898(uint256) = HIGH_LEVEL_CALL, dest:registry_1(IAgentNft), function:stakingTokenToVirtualId, arguments:['TMP_13897']  
assetToken_7(address) := phi(['assetToken_1', 'assetToken_18', 'assetToken_10', 'assetToken_6'])
_balanceCheckpoints_6(mapping(address => Checkpoints.Trace208)) := phi(['_balanceCheckpoints_13', '_balanceCheckpoints_23', '_balanceCheckpoints_5', '_balanceCheckpoints_21'])
virtualId_1(uint256) := TMP_13898(uint256)
 require(bool,string)(! registry.isBlacklisted(virtualId),Agent Blacklisted)
TMP_13899(bool) = HIGH_LEVEL_CALL, dest:registry_1(IAgentNft), function:isBlacklisted, arguments:['virtualId_1']  
assetToken_8(address) := phi(['assetToken_1', 'assetToken_18', 'assetToken_10', 'assetToken_7'])
_balanceCheckpoints_7(mapping(address => Checkpoints.Trace208)) := phi(['_balanceCheckpoints_13', '_balanceCheckpoints_23', '_balanceCheckpoints_6', '_balanceCheckpoints_21'])
TMP_13900 = UnaryType.BANG TMP_13899 
TMP_13901(None) = SOLIDITY_CALL require(bool,string)(TMP_13900,Agent Blacklisted)
 totalSupply() == 0
TMP_13902(uint256) = INTERNAL_CALL, ERC20Upgradeable.totalSupply()()
TMP_13903(bool) = TMP_13902 == 0
CONDITION TMP_13903
 initialLock = amount
initialLock_1(uint256) := amount_1(uint256)
 registry.addValidator(virtualId,delegatee)
HIGH_LEVEL_CALL, dest:registry_1(IAgentNft), function:addValidator, arguments:['virtualId_1', 'delegatee_1']  
assetToken_10(address) := phi(['assetToken_1', 'assetToken_9', 'assetToken_18', 'assetToken_10'])
_balanceCheckpoints_9(mapping(address => Checkpoints.Trace208)) := phi(['_balanceCheckpoints_13', '_balanceCheckpoints_23', '_balanceCheckpoints_21', '_balanceCheckpoints_8'])
 IERC20(assetToken).safeTransferFrom(sender,address(this),amount)
TMP_13905 = CONVERT assetToken_10 to IERC20
TMP_13906 = CONVERT this to address
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransferFrom(IERC20,address,address,uint256), arguments:['TMP_13905', 'sender_1', 'TMP_13906', 'amount_1'] 
 _mint(receiver,amount)
INTERNAL_CALL, ERC20Upgradeable._mint(address,uint256)(receiver_1,amount_1)
 _delegate(receiver,delegatee)
INTERNAL_CALL, ERC20Votes._delegate(address,address)(receiver_1,delegatee_1)
 _balanceCheckpoints[receiver].push(clock(),SafeCast.toUint208(balanceOf(receiver)))
REF_5668(Checkpoints.Trace208) -> _balanceCheckpoints_11[receiver_1]
TMP_13910(uint48) = INTERNAL_CALL, VotesUpgradeable.clock()()
TMP_13911(uint256) = INTERNAL_CALL, ERC20Upgradeable.balanceOf(address)(receiver_1)
TMP_13912(uint208) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint208(uint256), arguments:['TMP_13911'] 
TUPLE_134(uint208,uint208) = LIBRARY_CALL, dest:Checkpoints, function:Checkpoints.push(Checkpoints.Trace208,uint48,uint208), arguments:['REF_5668', 'TMP_13910', 'TMP_13912']
```
#### AgentVeToken.transfer(address,uint256) [PUBLIC]
```slithir
 revert(string)(Transfer not supported)
TMP_13944(None) = SOLIDITY_CALL revert(string)(Transfer not supported)
```
#### AgentVeToken.transferFrom(address,address,uint256) [PUBLIC]
```slithir
 revert(string)(Transfer not supported)
TMP_13945(None) = SOLIDITY_CALL revert(string)(Transfer not supported)
```
#### AgentVeToken.withdraw(uint256) [PUBLIC]
```slithir
founder_4(address) := phi(['founder_0', 'founder_8', 'founder_1', 'founder_3'])
assetToken_11(address) := phi(['assetToken_0', 'assetToken_10', 'assetToken_18', 'assetToken_1'])
matureAt_3(uint256) := phi(['matureAt_7', 'matureAt_2', 'matureAt_0', 'matureAt_1'])
initialLock_2(uint256) := phi(['initialLock_1', 'initialLock_0', 'initialLock_6'])
_balanceCheckpoints_14(mapping(address => Checkpoints.Trace208)) := phi(['_balanceCheckpoints_13', '_balanceCheckpoints_23', '_balanceCheckpoints_21', '_balanceCheckpoints_0'])
 sender = _msgSender()
TMP_13921(address) = INTERNAL_CALL, ContextUpgradeable._msgSender()()
sender_1(address) := TMP_13921(address)
 require(bool,string)(balanceOf(sender) >= amount,Insufficient balance)
TMP_13922(uint256) = INTERNAL_CALL, ERC20Upgradeable.balanceOf(address)(sender_1)
TMP_13923(bool) = TMP_13922 >= amount_1
TMP_13924(None) = SOLIDITY_CALL require(bool,string)(TMP_13923,Insufficient balance)
 (sender == founder) && ((balanceOf(sender) - amount) < initialLock)
TMP_13925(bool) = sender_1 == founder_7
TMP_13926(uint256) = INTERNAL_CALL, ERC20Upgradeable.balanceOf(address)(sender_1)
TMP_13927(uint256) = TMP_13926 (c)- amount_1
TMP_13928(bool) = TMP_13927 < initialLock_6
TMP_13929(bool) = TMP_13925 && TMP_13928
CONDITION TMP_13929
 require(bool,string)(block.timestamp >= matureAt,Not mature yet)
TMP_13930(bool) = block.timestamp >= matureAt_7
TMP_13931(None) = SOLIDITY_CALL require(bool,string)(TMP_13930,Not mature yet)
 _burn(sender,amount)
INTERNAL_CALL, ERC20Upgradeable._burn(address,uint256)(sender_1,amount_1)
 _balanceCheckpoints[sender].push(clock(),SafeCast.toUint208(balanceOf(sender)))
REF_5672(Checkpoints.Trace208) -> _balanceCheckpoints_19[sender_1]
TMP_13933(uint48) = INTERNAL_CALL, VotesUpgradeable.clock()()
TMP_13934(uint256) = INTERNAL_CALL, ERC20Upgradeable.balanceOf(address)(sender_1)
TMP_13935(uint208) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint208(uint256), arguments:['TMP_13934'] 
TUPLE_135(uint208,uint208) = LIBRARY_CALL, dest:Checkpoints, function:Checkpoints.push(Checkpoints.Trace208,uint48,uint208), arguments:['REF_5672', 'TMP_13933', 'TMP_13935'] 
 IERC20(assetToken).safeTransfer(sender,amount)
TMP_13936 = CONVERT assetToken_18 to IERC20
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['TMP_13936', 'sender_1', 'amount_1'] 
 noReentrant()
MODIFIER_CALL, AgentVeToken.noReentrant()()
```
#### SafeCast.toUint48(uint256) [INTERNAL]
```slithir
 value > type()(uint48).max
TMP_5992(uint48) := 281474976710655(uint48)
TMP_5993(bool) = value_1 > TMP_5992
CONDITION TMP_5993
 revert SafeCastOverflowedUintDowncast(uint8,uint256)(48,value)
TMP_5994(None) = SOLIDITY_CALL revert SafeCastOverflowedUintDowncast(uint8,uint256)(48,value_1)
 uint48(value)
TMP_5995 = CONVERT value_1 to uint48
RETURN TMP_5995
```
#### Checkpoints.upperLookupRecent(Checkpoints.Trace160,uint96) [INTERNAL]
```slithir
 len = self._checkpoints.length
REF_2223(Checkpoints.Checkpoint160[]) -> self_1 (-> [])._checkpoints
REF_2224 -> LENGTH REF_2223
len_1(uint256) := REF_2224(uint256)
 low = 0
low_1(uint256) := 0(uint256)
 high = len
high_1(uint256) := len_1(uint256)
 len > 5
TMP_6250(bool) = len_1 > 5
CONDITION TMP_6250
 mid = len - Math.sqrt(len)
TMP_6251(uint256) = LIBRARY_CALL, dest:Math, function:Math.sqrt(uint256), arguments:['len_1'] 
TMP_6252(uint256) = len_1 (c)- TMP_6251
mid_1(uint256) := TMP_6252(uint256)
 key < _unsafeAccess(self._checkpoints,mid)._key
REF_2226(Checkpoints.Checkpoint160[]) -> self_1 (-> [])._checkpoints
TMP_6253(Checkpoints.Checkpoint160) = INTERNAL_CALL, Checkpoints._unsafeAccess(Checkpoints.Checkpoint160[],uint256)(REF_2226,mid_1)
REF_2227(uint96) -> TMP_6253._key
TMP_6254(bool) = key_1 < REF_2227
CONDITION TMP_6254
 high = mid
high_2(uint256) := mid_1(uint256)
 low = mid + 1
TMP_6255(uint256) = mid_1 (c)+ 1
low_2(uint256) := TMP_6255(uint256)
low_3(uint256) := phi(['low_1', 'low_2'])
high_3(uint256) := phi(['high_2', 'high_1'])
 pos = _upperBinaryLookup(self._checkpoints,key,low,high)
REF_2228(Checkpoints.Checkpoint160[]) -> self_1 (-> [])._checkpoints
TMP_6256(uint256) = INTERNAL_CALL, Checkpoints._upperBinaryLookup(Checkpoints.Checkpoint160[],uint96,uint256,uint256)(REF_2228,key_1,low_3,high_3)
pos_1(uint256) := TMP_6256(uint256)
 pos == 0
TMP_6257(bool) = pos_1 == 0
CONDITION TMP_6257
 0
RETURN 0
 _unsafeAccess(self._checkpoints,pos - 1)._value
REF_2229(Checkpoints.Checkpoint160[]) -> self_1 (-> [])._checkpoints
TMP_6258(uint256) = pos_1 (c)- 1
TMP_6259(Checkpoints.Checkpoint160) = INTERNAL_CALL, Checkpoints._unsafeAccess(Checkpoints.Checkpoint160[],uint256)(REF_2229,TMP_6258)
REF_2230(uint160) -> TMP_6259._value
RETURN REF_2230
```
#### IAccessControl.hasRole(bytes32,address) [EXTERNAL]
```slithir

```
#### IERC20.allowance(address,address) [EXTERNAL]
```slithir

```
#### IERC20.balanceOf(address) [EXTERNAL]
```slithir

```
#### SafeERC20.safeTransferFrom(IERC20,address,address,uint256) [INTERNAL]
```slithir
 _callOptionalReturn(token,abi.encodeCall(token.transferFrom,(from,to,value)))
REF_2046(transferFrom) -> token_1.transferFrom
TMP_5415(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_2046,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78000160>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78001150>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78001c90>])
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_5415)
```
#### SafeCast.toUint208(uint256) [INTERNAL]
```slithir
 value > type()(uint208).max
TMP_5892(uint208) := 411376139330301510538742295639337626245683966408394965837152255(uint208)
TMP_5893(bool) = value_1 > TMP_5892
CONDITION TMP_5893
 revert SafeCastOverflowedUintDowncast(uint8,uint256)(208,value)
TMP_5894(None) = SOLIDITY_CALL revert SafeCastOverflowedUintDowncast(uint8,uint256)(208,value_1)
 uint208(value)
TMP_5895 = CONVERT value_1 to uint208
RETURN TMP_5895
```
#### Checkpoints.push(Checkpoints.Trace160,uint96,uint160) [INTERNAL]
```slithir
 _insert(self._checkpoints,key,value)
REF_2212(Checkpoints.Checkpoint160[]) -> self_1 (-> [])._checkpoints
TUPLE_69(uint160,uint160) = INTERNAL_CALL, Checkpoints._insert(Checkpoints.Checkpoint160[],uint96,uint160)(REF_2212,key_1,value_1)
RETURN TUPLE_69
```
#### IAgentNft.isBlacklisted(uint256) [EXTERNAL]
```slithir

```
#### IAgentNft.stakingTokenToVirtualId(address) [EXTERNAL]
```slithir

```
#### SafeERC20.safeTransfer(IERC20,address,uint256) [INTERNAL]
```slithir
 _callOptionalReturn(token,abi.encodeCall(token.transfer,(to,value)))
REF_2044(transfer) -> token_1.transfer
TMP_5413(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_2044,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78000550>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78001210>])
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_5413)
```
#### Math.sqrt(uint256,Math.Rounding) [INTERNAL]
```slithir
 result = sqrt(a)
TMP_5788(uint256) = INTERNAL_CALL, Math.sqrt(uint256)(a_1)
result_1(uint256) := TMP_5788(uint256)
 unsignedRoundsUp(rounding) && result * result < a
TMP_5789(bool) = INTERNAL_CALL, Math.unsignedRoundsUp(Math.Rounding)(rounding_1)
TMP_5790(uint256) = result_1 * result_1
TMP_5791(bool) = TMP_5790 < a_1
TMP_5792(bool) = TMP_5789 && TMP_5791
CONDITION TMP_5792
 result + 1
TMP_5793(uint256) = result_1 + 1
RETURN TMP_5793
 result + 0
TMP_5794(uint256) = result_1 + 0
RETURN TMP_5794
```
#### Checkpoints._unsafeAccess(Checkpoints.Checkpoint208[],uint256) [PRIVATE]
```slithir
self_1 (-> [])(Checkpoints.Checkpoint208[]) := phi(['REF_2182', 'REF_2186', 'self_1 (-> [])', 'REF_2179', 'REF_2174', 'self_1 (-> [])', 'REF_2190', 'REF_2169', 'self_1 (-> [])'])
pos_1(uint256) := phi(['TMP_6206', 'mid_1', 'TMP_6209', 'mid_1', 'TMP_6220', 'TMP_6196', 'pos_1', 'TMP_6215', 'TMP_6212', 'mid_1'])
 mstore(uint256,uint256)(0,self)
TMP_6240(None) = SOLIDITY_CALL mstore(uint256,uint256)(0,self_1 (-> []))
 result = keccak256(uint256,uint256)(0,0x20) + pos
TMP_6241(uint256) = SOLIDITY_CALL keccak256(uint256,uint256)(0,32)
TMP_6242(uint256) = TMP_6241 + pos_1
result_1 (-> ['TMP_6242'])(Checkpoints.Checkpoint208) := TMP_6242(uint256)
 result
RETURN result_1 (-> ['TMP_6242'])
```
#### Checkpoints._upperBinaryLookup(Checkpoints.Checkpoint160[],uint96,uint256,uint256) [PRIVATE]
```slithir
self_1 (-> [])(Checkpoints.Checkpoint160[]) := phi(['REF_2228', 'REF_2220'])
key_1(uint96) := phi(['key_1', 'key_1'])
low_1(uint256) := phi(['low_3'])
high_1(uint256) := phi(['high_3', 'len_1'])
 low < high
TMP_6282(bool) = low_1 < high_1
CONDITION TMP_6282
 mid = Math.average(low,high)
TMP_6283(uint256) = LIBRARY_CALL, dest:Math, function:Math.average(uint256,uint256), arguments:['low_1', 'high_1'] 
mid_1(uint256) := TMP_6283(uint256)
 _unsafeAccess(self,mid)._key > key
TMP_6284(Checkpoints.Checkpoint160) = INTERNAL_CALL, Checkpoints._unsafeAccess(Checkpoints.Checkpoint160[],uint256)(self_1 (-> []),mid_1)
REF_2256(uint96) -> TMP_6284._key
TMP_6285(bool) = REF_2256 > key_1
CONDITION TMP_6285
 high = mid
high_2(uint256) := mid_1(uint256)
 low = mid + 1
TMP_6286(uint256) = mid_1 (c)+ 1
low_2(uint256) := TMP_6286(uint256)
low_3(uint256) := phi(['low_2', 'low_1'])
high_3(uint256) := phi(['high_2', 'high_1'])
 high
RETURN high_1
```
#### SafeERC20._callOptionalReturn(IERC20,bytes) [PRIVATE]
```slithir
token_1(IERC20) := phi(['token_1', 'token_1', 'token_1'])
data_1(bytes) := phi(['approvalCall_1', 'TMP_5413', 'TMP_5415', 'TMP_5430'])
 returndata = address(token).functionCall(data)
TMP_5433 = CONVERT token_1 to address
TMP_5434(bytes) = LIBRARY_CALL, dest:Address, function:Address.functionCall(address,bytes), arguments:['TMP_5433', 'data_1'] 
returndata_1(bytes) := TMP_5434(bytes)
 returndata.length != 0 && ! abi.decode(returndata,(bool))
REF_2054 -> LENGTH returndata_1
TMP_5435(bool) = REF_2054 != 0
TMP_5436(bool) = SOLIDITY_CALL abi.decode()(returndata_1,bool)
TMP_5437 = UnaryType.BANG TMP_5436 
TMP_5438(bool) = TMP_5435 && TMP_5437
CONDITION TMP_5438
 revert SafeERC20FailedOperation(address)(address(token))
TMP_5439 = CONVERT token_1 to address
TMP_5440(None) = SOLIDITY_CALL revert SafeERC20FailedOperation(address)(TMP_5439)
```
#### Checkpoints._insert(Checkpoints.Checkpoint160[],uint96,uint160) [PRIVATE]
```slithir
self_1 (-> [])(Checkpoints.Checkpoint160[]) := phi(['REF_2212'])
key_1(uint96) := phi(['key_1'])
value_1(uint160) := phi(['value_1'])
 pos = self.length
REF_2244 -> LENGTH self_1 (-> [])
pos_1(uint256) := REF_2244(uint256)
 pos > 0
TMP_6266(bool) = pos_1 > 0
CONDITION TMP_6266
 last = _unsafeAccess(self,pos - 1)
TMP_6267(uint256) = pos_1 (c)- 1
TMP_6268(Checkpoints.Checkpoint160) = INTERNAL_CALL, Checkpoints._unsafeAccess(Checkpoints.Checkpoint160[],uint256)(self_1 (-> []),TMP_6267)
last_1(Checkpoints.Checkpoint160) := TMP_6268(Checkpoints.Checkpoint160)
 last._key > key
REF_2245(uint96) -> last_1._key
TMP_6269(bool) = REF_2245 > key_1
CONDITION TMP_6269
 revert CheckpointUnorderedInsertion()()
TMP_6270(None) = SOLIDITY_CALL revert CheckpointUnorderedInsertion()()
 last._key == key
REF_2246(uint96) -> last_1._key
TMP_6271(bool) = REF_2246 == key_1
CONDITION TMP_6271
 _unsafeAccess(self,pos - 1)._value = value
TMP_6272(uint256) = pos_1 (c)- 1
TMP_6273(Checkpoints.Checkpoint160) = INTERNAL_CALL, Checkpoints._unsafeAccess(Checkpoints.Checkpoint160[],uint256)(self_1 (-> []),TMP_6272)
REF_2247(uint160) -> TMP_6273._value
REF_2247(uint160) (->TMP_6273) := value_1(uint160)
 self.push(Checkpoint160({_key:key,_value:value}))
TMP_6274(Checkpoints.Checkpoint160) = new Checkpoint160(key_1,value_1)
REF_2249 -> LENGTH self_1 (-> [])
TMP_6276(uint256) := REF_2249(uint256)
TMP_6277(uint256) = TMP_6276 (c)+ 1
self_4 (-> [])(Checkpoints.Checkpoint160[]) := phi(['self_1 (-> [])'])
REF_2249(uint256) (->self_4 (-> [])) := TMP_6277(uint256)
REF_2250(Checkpoints.Checkpoint160) -> self_4 (-> [])[TMP_6276]
self_5 (-> [])(Checkpoints.Checkpoint160[]) := phi(['self_4 (-> [])'])
REF_2250(Checkpoints.Checkpoint160) (->self_5 (-> [])) := TMP_6274(Checkpoints.Checkpoint160)
 (last._value,value)
REF_2251(uint160) -> last_1._value
RETURN REF_2251,value_1
 self.push(Checkpoint160({_key:key,_value:value}))
TMP_6278(Checkpoints.Checkpoint160) = new Checkpoint160(key_1,value_1)
REF_2253 -> LENGTH self_1 (-> [])
TMP_6280(uint256) := REF_2253(uint256)
TMP_6281(uint256) = TMP_6280 (c)+ 1
self_2 (-> [])(Checkpoints.Checkpoint160[]) := phi(['self_1 (-> [])'])
REF_2253(uint256) (->self_2 (-> [])) := TMP_6281(uint256)
REF_2254(Checkpoints.Checkpoint160) -> self_2 (-> [])[TMP_6280]
self_3 (-> [])(Checkpoints.Checkpoint160[]) := phi(['self_2 (-> [])'])
REF_2254(Checkpoints.Checkpoint160) (->self_3 (-> [])) := TMP_6278(Checkpoints.Checkpoint160)
 (0,value)
RETURN 0,value_1
```
#### Math.log2(uint256,Math.Rounding) [INTERNAL]
```slithir
 result = log2(value)
TMP_5811(uint256) = INTERNAL_CALL, Math.log2(uint256)(value_1)
result_1(uint256) := TMP_5811(uint256)
 unsignedRoundsUp(rounding) && 1 << result < value
TMP_5812(bool) = INTERNAL_CALL, Math.unsignedRoundsUp(Math.Rounding)(rounding_1)
TMP_5813(uint256) = 1 << result_1
TMP_5814(bool) = TMP_5813 < value_1
TMP_5815(bool) = TMP_5812 && TMP_5814
CONDITION TMP_5815
 result + 1
TMP_5816(uint256) = result_1 + 1
RETURN TMP_5816
 result + 0
TMP_5817(uint256) = result_1 + 0
RETURN TMP_5817
```
#### Math.min(uint256,uint256) [INTERNAL]
```slithir
a_1(uint256) := phi(['result_8'])
b_1(uint256) := phi(['TMP_5786'])
 a < b
TMP_5708(bool) = a_1 < b_1
CONDITION TMP_5708
 a
RETURN a_1
 b
RETURN b_1
```

#### Math.average(uint256,uint256) [INTERNAL]
```slithir
 (a & b) + (a ^ b) / 2
TMP_5709(uint256) = a_1 & b_1
TMP_5710(uint256) = a_1 ^ b_1
TMP_5711(uint256) = TMP_5710 (c)/ 2
TMP_5712(uint256) = TMP_5709 (c)+ TMP_5711
RETURN TMP_5712
```
#### Address.functionCall(address,bytes) [INTERNAL]
```slithir
 functionCallWithValue(target,data,0)
TMP_5457(bytes) = INTERNAL_CALL, Address.functionCallWithValue(address,bytes,uint256)(target_1,data_1,0)
RETURN TMP_5457
```
