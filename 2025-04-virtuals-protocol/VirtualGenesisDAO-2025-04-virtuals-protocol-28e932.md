
### Storage layout (VirtualGenesisDAO) 

```text
_earlyExecutions mapping(uint256 => bool)
_quorumCheckpoints Checkpoints.Trace224
_quorum uint256

```


#### VirtualGenesisDAO._propose(address[],uint256[],bytes[],string,address) [INTERNAL]
```slithir
targets_1(address[]) := phi(['targets_1'])
values_1(uint256[]) := phi(['values_1'])
calldatas_1(bytes[]) := phi(['calldatas_1'])
description_1(string) := phi(['description_1'])
proposer_1(address) := phi(['proposer_1'])
 super._propose(targets,values,calldatas,description,proposer)
TMP_9906(uint256) = INTERNAL_CALL, GovernorStorage._propose(address[],uint256[],bytes[],string,address)(targets_1,values_1,calldatas_1,description_1,proposer_1)
RETURN TMP_9906
```
#### VirtualGenesisDAO.constructor(IVotes,uint48,uint32,uint256) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_6'])
_quorumCheckpoints_1(Checkpoints.Trace224) := phi(['_quorumCheckpoints_0', '_quorumCheckpoints_4', '_quorumCheckpoints_8'])
 _quorumCheckpoints.push(0,10000e18)
TUPLE_101(uint224,uint224) = LIBRARY_CALL, dest:Checkpoints, function:Checkpoints.push(Checkpoints.Trace224,uint32,uint224), arguments:['_quorumCheckpoints_4', '0', '10000000000000000000000'] 
 _grantRole(DEFAULT_ADMIN_ROLE,_msgSender())
TMP_9897(address) = INTERNAL_CALL, Context._msgSender()()
TMP_9898(bool) = INTERNAL_CALL, AccessControl._grantRole(bytes32,address)(DEFAULT_ADMIN_ROLE_5,TMP_9897)
 Governor(VirtualGenesis)
INTERNAL_CALL, Governor.constructor(string)(VirtualGenesis)
 GovernorSettings(initialVotingDelay,initialVotingPeriod,initialProposalThreshold)
INTERNAL_CALL, GovernorSettings.constructor(uint48,uint32,uint256)(initialVotingDelay_1,initialVotingPeriod_1,initialProposalThreshold_1)
 GovernorVotes(token)
INTERNAL_CALL, GovernorVotes.constructor(IVotes)(token_1)
```
#### VirtualGenesisDAO.earlyExecute(uint256) [PUBLIC]
```slithir
_earlyExecutions_1(mapping(uint256 => bool)) := phi(['_earlyExecutions_8', '_earlyExecutions_7', '_earlyExecutions_0'])
EXECUTOR_ROLE_1(bytes32) := phi(['EXECUTOR_ROLE_2', 'EXECUTOR_ROLE_0'])
 (targets,values,calldatas,descriptionHash) = proposalDetails(proposalId)
TUPLE_102(address[],uint256[],bytes[],bytes32) = INTERNAL_CALL, GovernorStorage.proposalDetails(uint256)(proposalId_1)
targets_1(address[])= UNPACK TUPLE_102 index: 0 
values_1(uint256[])= UNPACK TUPLE_102 index: 1 
calldatas_1(bytes[])= UNPACK TUPLE_102 index: 2 
descriptionHash_1(bytes32)= UNPACK TUPLE_102 index: 3 
 require(bool,string)(state(proposalId) == ProposalState.Active && _voteSucceeded(proposalId) && _quorumReached(proposalId) && ! _earlyExecutions[proposalId],Proposal not ready for early execution)
TMP_9914(IGovernor.ProposalState) = INTERNAL_CALL, VirtualGenesisDAO.state(uint256)(proposalId_1)
_earlyExecutions_4(mapping(uint256 => bool)) := phi(['_earlyExecutions_8'])
REF_4043(IGovernor.ProposalState) -> ProposalState.Active
TMP_9915(bool) = TMP_9914 == REF_4043
TMP_9916(bool) = INTERNAL_CALL, GovernorCountingSimple._voteSucceeded(uint256)(proposalId_1)
TMP_9917(bool) = TMP_9915 && TMP_9916
TMP_9918(bool) = INTERNAL_CALL, GovernorCountingSimple._quorumReached(uint256)(proposalId_1)
TMP_9919(bool) = TMP_9917 && TMP_9918
REF_4044(bool) -> _earlyExecutions_6[proposalId_1]
TMP_9920 = UnaryType.BANG REF_4044 
TMP_9921(bool) = TMP_9919 && TMP_9920
TMP_9922(None) = SOLIDITY_CALL require(bool,string)(TMP_9921,Proposal not ready for early execution)
 _earlyExecutions[proposalId] = true
REF_4045(bool) -> _earlyExecutions_6[proposalId_1]
_earlyExecutions_7(mapping(uint256 => bool)) := phi(['_earlyExecutions_6'])
REF_4045(bool) (->_earlyExecutions_7) := True(bool)
 _executeOperations(proposalId,targets,values,calldatas,descriptionHash)
INTERNAL_CALL, Governor._executeOperations(uint256,address[],uint256[],bytes[],bytes32)(proposalId_1,targets_1,values_1,calldatas_1,descriptionHash_1)
 ProposalExecuted(proposalId)
Emit ProposalExecuted(proposalId_1)
 proposalId
RETURN proposalId_1
 onlyRole(EXECUTOR_ROLE)
MODIFIER_CALL, AccessControl.onlyRole(bytes32)(EXECUTOR_ROLE_1)
```
#### VirtualGenesisDAO.proposalThreshold() [PUBLIC]
```slithir
 super.proposalThreshold()
TMP_9904(uint256) = INTERNAL_CALL, GovernorSettings.proposalThreshold()()
RETURN TMP_9904
```
#### VirtualGenesisDAO.propose(address[],uint256[],bytes[],string) [PUBLIC]
```slithir
 super.propose(targets,values,calldatas,description)
TMP_9905(uint256) = INTERNAL_CALL, Governor.propose(address[],uint256[],bytes[],string)(targets_1,values_1,calldatas_1,description_1)
RETURN TMP_9905
```
#### VirtualGenesisDAO.quorum(uint256) [PUBLIC]
```slithir
blockNumber_1(uint256) := phi(['TMP_9599'])
_quorumCheckpoints_5(Checkpoints.Trace224) := phi(['_quorumCheckpoints_0', '_quorumCheckpoints_4', '_quorumCheckpoints_8'])
 length = _quorumCheckpoints.length()
TMP_9907(uint256) = LIBRARY_CALL, dest:Checkpoints, function:Checkpoints.length(Checkpoints.Trace224), arguments:['_quorumCheckpoints_5'] 
length_1(uint256) := TMP_9907(uint256)
 latest = _quorumCheckpoints.at(SafeCast.toUint32(length - 1))
TMP_9908(uint256) = length_1 (c)- 1
TMP_9909(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['TMP_9908'] 
TMP_9910(Checkpoints.Checkpoint224) = LIBRARY_CALL, dest:Checkpoints, function:Checkpoints.at(Checkpoints.Trace224,uint32), arguments:['_quorumCheckpoints_5', 'TMP_9909'] 
latest_1(Checkpoints.Checkpoint224) := TMP_9910(Checkpoints.Checkpoint224)
 latestKey = latest._key
REF_4039(uint32) -> latest_1._key
latestKey_1(uint48) := REF_4039(uint32)
 latestValue = latest._value
REF_4040(uint224) -> latest_1._value
latestValue_1(uint224) := REF_4040(uint224)
 latestKey <= blockNumber
TMP_9911(bool) = latestKey_1 <= blockNumber_1
CONDITION TMP_9911
 latestValue
RETURN latestValue_1
 _quorumCheckpoints.upperLookupRecent(SafeCast.toUint32(blockNumber))
TMP_9912(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['blockNumber_1'] 
TMP_9913(uint224) = LIBRARY_CALL, dest:Checkpoints, function:Checkpoints.upperLookupRecent(Checkpoints.Trace224,uint32), arguments:['_quorumCheckpoints_5', 'TMP_9912'] 
RETURN TMP_9913
```
#### GovernorCountingSimple.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 BALLOT_TYPEHASH = keccak256(bytes)(Ballot(uint256 proposalId,uint8 support,address voter,uint256 nonce))
 EXTENDED_BALLOT_TYPEHASH = keccak256(bytes)(ExtendedBallot(uint256 proposalId,uint8 support,address voter,uint256 nonce,string reason,bytes params))
 _checkGovernance()
INTERNAL_CALL, Governor._checkGovernance()()
```
#### VirtualGenesisDAO.state(uint256) [PUBLIC]
```slithir
proposalId_1(uint256) := phi(['proposalId_1', 'proposalId_1', 'proposalId_1'])
_earlyExecutions_8(mapping(uint256 => bool)) := phi(['_earlyExecutions_8', '_earlyExecutions_7', '_earlyExecutions_0'])
 _earlyExecutions[proposalId]
REF_4046(bool) -> _earlyExecutions_8[proposalId_1]
CONDITION REF_4046
 ProposalState.Executed
REF_4047(IGovernor.ProposalState) -> ProposalState.Executed
RETURN REF_4047
 super.state(proposalId)
TMP_9926(IGovernor.ProposalState) = INTERNAL_CALL, Governor.state(uint256)(proposalId_1)
RETURN TMP_9926
```
#### VirtualGenesisDAO.supportsInterface(bytes4) [PUBLIC]
```slithir
 super.supportsInterface(interfaceId)
TMP_9933(bool) = INTERNAL_CALL, AccessControl.supportsInterface(bytes4)(interfaceId_1)
RETURN TMP_9933
```
#### VirtualGenesisDAO.updateQuorum(uint224) [PUBLIC]
```slithir
_quorumCheckpoints_6(Checkpoints.Trace224) := phi(['_quorumCheckpoints_0', '_quorumCheckpoints_4', '_quorumCheckpoints_8'])
 oldQuorum = _quorumCheckpoints.latest()
TMP_9927(uint224) = LIBRARY_CALL, dest:Checkpoints, function:Checkpoints.latest(Checkpoints.Trace224), arguments:['_quorumCheckpoints_7'] 
oldQuorum_1(uint224) := TMP_9927(uint224)
 _quorumCheckpoints.push(SafeCast.toUint32(clock()),SafeCast.toUint208(newQuorum))
TMP_9928(uint48) = INTERNAL_CALL, GovernorVotes.clock()()
TMP_9929(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['TMP_9928'] 
TMP_9930(uint208) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint208(uint256), arguments:['newQuorum_1'] 
TUPLE_103(uint224,uint224) = LIBRARY_CALL, dest:Checkpoints, function:Checkpoints.push(Checkpoints.Trace224,uint32,uint224), arguments:['_quorumCheckpoints_8', 'TMP_9929', 'TMP_9930'] 
 QuorumUpdated(oldQuorum,newQuorum)
Emit QuorumUpdated(oldQuorum_1,newQuorum_1)
 onlyGovernance()
MODIFIER_CALL, Governor.onlyGovernance()()
```
#### VirtualGenesisDAO.votingDelay() [PUBLIC]
```slithir
 super.votingDelay()
TMP_9902(uint256) = INTERNAL_CALL, GovernorSettings.votingDelay()()
RETURN TMP_9902
```
#### VirtualGenesisDAO.votingPeriod() [PUBLIC]
```slithir
 super.votingPeriod()
TMP_9903(uint256) = INTERNAL_CALL, GovernorSettings.votingPeriod()()
RETURN TMP_9903
```
#### Checkpoints.push(Checkpoints.Trace160,uint96,uint160) [INTERNAL]
```slithir
 _insert(self._checkpoints,key,value)
REF_2212(Checkpoints.Checkpoint160[]) -> self_1 (-> [])._checkpoints
TUPLE_69(uint160,uint160) = INTERNAL_CALL, Checkpoints._insert(Checkpoints.Checkpoint160[],uint96,uint160)(REF_2212,key_1,value_1)
RETURN TUPLE_69
```
#### SafeCast.toUint32(uint256) [INTERNAL]
```slithir
 value > type()(uint32).max
TMP_6002(uint32) := 4294967295(uint32)
TMP_6003(bool) = value_1 > TMP_6002
CONDITION TMP_6003
 revert SafeCastOverflowedUintDowncast(uint8,uint256)(32,value)
TMP_6004(None) = SOLIDITY_CALL revert SafeCastOverflowedUintDowncast(uint8,uint256)(32,value_1)
 uint32(value)
TMP_6005 = CONVERT value_1 to uint32
RETURN TMP_6005
```
#### Checkpoints.at(Checkpoints.Trace160,uint32) [INTERNAL]
```slithir
 self._checkpoints[pos]
REF_2242(Checkpoints.Checkpoint160[]) -> self_1 (-> [])._checkpoints
REF_2243(Checkpoints.Checkpoint160) -> REF_2242[pos_1]
RETURN REF_2243
```
#### Checkpoints.length(Checkpoints.Trace160) [INTERNAL]
```slithir
 self._checkpoints.length
REF_2240(Checkpoints.Checkpoint160[]) -> self_1 (-> [])._checkpoints
REF_2241 -> LENGTH REF_2240
RETURN REF_2241
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
#### Checkpoints.latest(Checkpoints.Trace160) [INTERNAL]
```slithir
 pos = self._checkpoints.length
REF_2231(Checkpoints.Checkpoint160[]) -> self_1 (-> [])._checkpoints
REF_2232 -> LENGTH REF_2231
pos_1(uint256) := REF_2232(uint256)
 pos == 0
TMP_6260(bool) = pos_1 == 0
CONDITION TMP_6260
 0
RETURN 0
 _unsafeAccess(self._checkpoints,pos - 1)._value
REF_2233(Checkpoints.Checkpoint160[]) -> self_1 (-> [])._checkpoints
TMP_6261(uint256) = pos_1 (c)- 1
TMP_6262(Checkpoints.Checkpoint160) = INTERNAL_CALL, Checkpoints._unsafeAccess(Checkpoints.Checkpoint160[],uint256)(REF_2233,TMP_6261)
REF_2234(uint160) -> TMP_6262._value
RETURN REF_2234
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
