




#### CheckPointsByAddress.transmit(CheckPointsByAddress.CheckPointsByAddressState,address,address,uint256) [INTERNAL]
```slithir
 _amount == 0
TMP_8022(bool) = _amount_1 == 0
CONDITION TMP_8022
 assert(bool)(! (_from == address(0) && _to == address(0)))
TMP_8023 = CONVERT 0 to address
TMP_8024(bool) = _from_1 == TMP_8023
TMP_8025 = CONVERT 0 to address
TMP_8026(bool) = _to_1 == TMP_8025
TMP_8027(bool) = TMP_8024 && TMP_8026
TMP_8028 = UnaryType.BANG TMP_8027 
TMP_8029(None) = SOLIDITY_CALL assert(bool)(TMP_8028)
 _from != address(0)
TMP_8030 = CONVERT 0 to address
TMP_8031(bool) = _from_1 != TMP_8030
CONDITION TMP_8031
 newValueFrom = valueOfAtNow(_self,_from) - _amount
TMP_8032(uint256) = INTERNAL_CALL, CheckPointsByAddress.valueOfAtNow(CheckPointsByAddress.CheckPointsByAddressState,address)(_self_1 (-> []),_from_1)
TMP_8033(uint256) = TMP_8032 (c)- _amount_1
newValueFrom_1(uint256) := TMP_8033(uint256)
 writeValue(_self,_from,newValueFrom)
INTERNAL_CALL, CheckPointsByAddress.writeValue(CheckPointsByAddress.CheckPointsByAddressState,address,uint256)(_self_1 (-> []),_from_1,newValueFrom_1)
 _to != address(0)
TMP_8035 = CONVERT 0 to address
TMP_8036(bool) = _to_1 != TMP_8035
CONDITION TMP_8036
 newValueTo = valueOfAtNow(_self,_to) + _amount
TMP_8037(uint256) = INTERNAL_CALL, CheckPointsByAddress.valueOfAtNow(CheckPointsByAddress.CheckPointsByAddressState,address)(_self_1 (-> []),_to_1)
TMP_8038(uint256) = TMP_8037 (c)+ _amount_1
newValueTo_1(uint256) := TMP_8038(uint256)
 writeValue(_self,_to,newValueTo)
INTERNAL_CALL, CheckPointsByAddress.writeValue(CheckPointsByAddress.CheckPointsByAddressState,address,uint256)(_self_1 (-> []),_to_1,newValueTo_1)
```
#### CheckPointsByAddress.valueOfAt(CheckPointsByAddress.CheckPointsByAddressState,address,uint256) [INTERNAL]
```slithir
 history = _self.historyByAddress[_owner]
REF_5072(mapping(address => CheckPointHistory.CheckPointHistoryState)) -> _self_1 (-> []).historyByAddress
REF_5073(CheckPointHistory.CheckPointHistoryState) -> REF_5072[_owner_1]
history_1 (-> ['_self'])(CheckPointHistory.CheckPointHistoryState) := REF_5073(CheckPointHistory.CheckPointHistoryState)
 history.valueAt(_blockNumber)
TMP_8040(uint256) = LIBRARY_CALL, dest:CheckPointHistory, function:CheckPointHistory.valueAt(CheckPointHistory.CheckPointHistoryState,uint256), arguments:["history_1 (-> ['_self'])", '_blockNumber_1'] 
RETURN TMP_8040
```
#### CheckPointsByAddress.valueOfAtNow(CheckPointsByAddress.CheckPointsByAddressState,address) [INTERNAL]
```slithir
_self_1 (-> [])(CheckPointsByAddress.CheckPointsByAddressState) := phi(['_self_1 (-> [])'])
_owner_1(address) := phi(['_from_1', '_to_1'])
 history = _self.historyByAddress[_owner]
REF_5075(mapping(address => CheckPointHistory.CheckPointHistoryState)) -> _self_1 (-> []).historyByAddress
REF_5076(CheckPointHistory.CheckPointHistoryState) -> REF_5075[_owner_1]
history_1 (-> ['_self'])(CheckPointHistory.CheckPointHistoryState) := REF_5076(CheckPointHistory.CheckPointHistoryState)
 history.valueAtNow()
TMP_8041(uint256) = LIBRARY_CALL, dest:CheckPointHistory, function:CheckPointHistory.valueAtNow(CheckPointHistory.CheckPointHistoryState), arguments:["history_1 (-> ['_self'])"] 
RETURN TMP_8041
```
#### CheckPointsByAddress.writeValue(CheckPointsByAddress.CheckPointsByAddressState,address,uint256) [INTERNAL]
```slithir
_self_1 (-> [])(CheckPointsByAddress.CheckPointsByAddressState) := phi(['_self_1 (-> [])'])
_owner_1(address) := phi(['_from_1', '_to_1'])
_value_1(uint256) := phi(['newValueTo_1', 'newValueFrom_1'])
 history = _self.historyByAddress[_owner]
REF_5078(mapping(address => CheckPointHistory.CheckPointHistoryState)) -> _self_1 (-> []).historyByAddress
REF_5079(CheckPointHistory.CheckPointHistoryState) -> REF_5078[_owner_1]
history_1 (-> ['_self'])(CheckPointHistory.CheckPointHistoryState) := REF_5079(CheckPointHistory.CheckPointHistoryState)
 history.writeValue(_value)
LIBRARY_CALL, dest:CheckPointHistory, function:CheckPointHistory.writeValue(CheckPointHistory.CheckPointHistoryState,uint256), arguments:["history_1 (-> ['_self'])", '_value_1']
```
#### CheckPointHistory.cleanupOldCheckpoints(CheckPointHistory.CheckPointHistoryState,uint256,uint256) [INTERNAL]
```slithir
 _cleanupBlockNumber == 0
TMP_8004(bool) = _cleanupBlockNumber_1 == 0
CONDITION TMP_8004
 0
RETURN 0
 length = _self.endIndex
REF_5062(uint64) -> _self_1 (-> []).endIndex
length_1(uint256) := REF_5062(uint64)
 length == 0
TMP_8005(bool) = length_1 == 0
CONDITION TMP_8005
 0
RETURN 0
 startIndex = _self.startIndex
REF_5063(uint64) -> _self_1 (-> []).startIndex
startIndex_1(uint256) := REF_5063(uint64)
 endIndex = Math.min(startIndex + _count,length - 1)
TMP_8006(uint256) = startIndex_1 (c)+ _count_1
TMP_8007(uint256) = length_1 (c)- 1
TMP_8008(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['TMP_8006', 'TMP_8007'] 
endIndex_1(uint256) := TMP_8008(uint256)
 index = startIndex
index_1(uint256) := startIndex_1(uint256)
 index < endIndex && _self.checkpoints[index + 1].fromBlock <= _cleanupBlockNumber
_self_2 (-> [])(CheckPointHistory.CheckPointHistoryState) := phi(['_self_1 (-> [])', '_self_2 (-> [])'])
index_2(uint256) := phi(['index_3', 'index_1'])
TMP_8009(bool) = index_2 < endIndex_1
REF_5065(mapping(uint256 => CheckPointHistory.CheckPoint)) -> _self_2 (-> []).checkpoints
TMP_8010(uint256) = index_2 (c)+ 1
REF_5066(CheckPointHistory.CheckPoint) -> REF_5065[TMP_8010]
REF_5067(uint64) -> REF_5066.fromBlock
TMP_8011(bool) = REF_5067 <= _cleanupBlockNumber_1
TMP_8012(bool) = TMP_8009 && TMP_8011
CONDITION TMP_8012
 delete _self.checkpoints[index]
REF_5068(mapping(uint256 => CheckPointHistory.CheckPoint)) -> _self_2 (-> []).checkpoints
REF_5069(CheckPointHistory.CheckPoint) -> REF_5068[index_2]
REF_5068 = delete REF_5069 
 index ++
TMP_8013(uint256) := index_2(uint256)
index_3(uint256) = index_2 (c)+ 1
 index > startIndex
TMP_8014(bool) = index_2 > startIndex_1
CONDITION TMP_8014
 _self.startIndex = index.toUint64()
REF_5070(uint64) -> _self_2 (-> []).startIndex
TMP_8015(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['index_2'] 
_self_3 (-> [])(CheckPointHistory.CheckPointHistoryState) := phi(['_self_2 (-> [])'])
REF_5070(uint64) (->_self_3 (-> [])) := TMP_8015(uint64)
 index - startIndex
TMP_8016(uint256) = index_2 (c)- startIndex_1
RETURN TMP_8016
```
#### CheckPointHistory.valueAt(CheckPointHistory.CheckPointHistoryState,uint256) [INTERNAL]
```slithir
 historyCount = _self.endIndex
REF_5029(uint64) -> _self_1 (-> []).endIndex
historyCount_1(uint256) := REF_5029(uint64)
 historyCount == 0
TMP_7977(bool) = historyCount_1 == 0
CONDITION TMP_7977
 0
RETURN 0
 _blockNumber >= block.number || _blockNumber >= _self.checkpoints[historyCount - 1].fromBlock
TMP_7978(bool) = _blockNumber_1 >= block.number
REF_5030(mapping(uint256 => CheckPointHistory.CheckPoint)) -> _self_1 (-> []).checkpoints
TMP_7979(uint256) = historyCount_1 (c)- 1
REF_5031(CheckPointHistory.CheckPoint) -> REF_5030[TMP_7979]
REF_5032(uint64) -> REF_5031.fromBlock
TMP_7980(bool) = _blockNumber_1 >= REF_5032
TMP_7981(bool) = TMP_7978 || TMP_7980
CONDITION TMP_7981
 _self.checkpoints[historyCount - 1].value
REF_5033(mapping(uint256 => CheckPointHistory.CheckPoint)) -> _self_1 (-> []).checkpoints
TMP_7982(uint256) = historyCount_1 (c)- 1
REF_5034(CheckPointHistory.CheckPoint) -> REF_5033[TMP_7982]
REF_5035(uint192) -> REF_5034.value
RETURN REF_5035
 startIndex = _self.startIndex
REF_5036(uint64) -> _self_1 (-> []).startIndex
startIndex_1(uint256) := REF_5036(uint64)
 _blockNumber < _self.checkpoints[startIndex].fromBlock
REF_5037(mapping(uint256 => CheckPointHistory.CheckPoint)) -> _self_1 (-> []).checkpoints
REF_5038(CheckPointHistory.CheckPoint) -> REF_5037[startIndex_1]
REF_5039(uint64) -> REF_5038.fromBlock
TMP_7983(bool) = _blockNumber_1 < REF_5039
CONDITION TMP_7983
 require(bool,error)(startIndex == 0,revert CheckPointHistoryReadingFromCleanedupBlock()())
TMP_7984(bool) = startIndex_1 == 0
TMP_7985(None) = SOLIDITY_CALL revert CheckPointHistoryReadingFromCleanedupBlock()()
TMP_7986(None) = SOLIDITY_CALL require(bool,error)(TMP_7984,TMP_7985)
 0
RETURN 0
 index = _indexOfGreatestBlockLessThan(_self.checkpoints,startIndex,_self.endIndex,_blockNumber)
REF_5040(mapping(uint256 => CheckPointHistory.CheckPoint)) -> _self_1 (-> []).checkpoints
REF_5041(uint64) -> _self_1 (-> []).endIndex
TMP_7987(uint256) = INTERNAL_CALL, CheckPointHistory._indexOfGreatestBlockLessThan(mapping(uint256 => CheckPointHistory.CheckPoint),uint256,uint256,uint256)(REF_5040,startIndex_1,REF_5041,_blockNumber_1)
index_1(uint256) := TMP_7987(uint256)
 _self.checkpoints[index].value
REF_5042(mapping(uint256 => CheckPointHistory.CheckPoint)) -> _self_1 (-> []).checkpoints
REF_5043(CheckPointHistory.CheckPoint) -> REF_5042[index_1]
REF_5044(uint192) -> REF_5043.value
RETURN REF_5044
 _value
```
#### CheckPointHistory.valueAtNow(CheckPointHistory.CheckPointHistoryState) [INTERNAL]
```slithir
 historyCount = _self.endIndex
REF_5045(uint64) -> _self_1 (-> []).endIndex
historyCount_1(uint256) := REF_5045(uint64)
 historyCount == 0
TMP_7988(bool) = historyCount_1 == 0
CONDITION TMP_7988
 0
RETURN 0
 _self.checkpoints[historyCount - 1].value
REF_5046(mapping(uint256 => CheckPointHistory.CheckPoint)) -> _self_1 (-> []).checkpoints
TMP_7989(uint256) = historyCount_1 (c)- 1
REF_5047(CheckPointHistory.CheckPoint) -> REF_5046[TMP_7989]
REF_5048(uint192) -> REF_5047.value
RETURN REF_5048
 _value
```
#### CheckPointHistory.writeValue(CheckPointHistory.CheckPointHistoryState,uint256) [INTERNAL]
```slithir
 historyCount = _self.endIndex
REF_5049(uint64) -> _self_1 (-> []).endIndex
historyCount_1(uint256) := REF_5049(uint64)
 historyCount == 0
TMP_7990(bool) = historyCount_1 == 0
CONDITION TMP_7990
 _self.checkpoints[0] = CheckPoint({fromBlock:block.number.toUint64(),value:_toUint192(_value)})
REF_5050(mapping(uint256 => CheckPointHistory.CheckPoint)) -> _self_1 (-> []).checkpoints
REF_5051(CheckPointHistory.CheckPoint) -> REF_5050[0]
TMP_7991(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['block.number'] 
TMP_7992(uint192) = INTERNAL_CALL, CheckPointHistory._toUint192(uint256)(_value_1)
TMP_7993(CheckPointHistory.CheckPoint) = new CheckPoint(TMP_7992,TMP_7991)
_self_4 (-> [])(CheckPointHistory.CheckPointHistoryState) := phi(['_self_1 (-> [])'])
REF_5051(CheckPointHistory.CheckPoint) (->_self_4 (-> [])) := TMP_7993(CheckPointHistory.CheckPoint)
 _self.endIndex = 1
REF_5053(uint64) -> _self_4 (-> []).endIndex
_self_5 (-> [])(CheckPointHistory.CheckPointHistoryState) := phi(['_self_4 (-> [])'])
REF_5053(uint64) (->_self_5 (-> [])) := 1(uint256)
 lastCheckpoint = _self.checkpoints[historyCount - 1]
REF_5054(mapping(uint256 => CheckPointHistory.CheckPoint)) -> _self_1 (-> []).checkpoints
TMP_7994(uint256) = historyCount_1 (c)- 1
REF_5055(CheckPointHistory.CheckPoint) -> REF_5054[TMP_7994]
lastCheckpoint_1 (-> ['_self'])(CheckPointHistory.CheckPoint) := REF_5055(CheckPointHistory.CheckPoint)
 lastBlock = lastCheckpoint.fromBlock
REF_5056(uint64) -> lastCheckpoint_1 (-> ['_self']).fromBlock
lastBlock_1(uint256) := REF_5056(uint64)
 block.number == lastBlock
TMP_7995(bool) = block.number == lastBlock_1
CONDITION TMP_7995
 lastCheckpoint.value = _toUint192(_value)
REF_5057(uint192) -> lastCheckpoint_1 (-> ['_self']).value
TMP_7996(uint192) = INTERNAL_CALL, CheckPointHistory._toUint192(uint256)(_value_1)
lastCheckpoint_2 (-> ['_self'])(CheckPointHistory.CheckPoint) := phi(["lastCheckpoint_1 (-> ['_self'])"])
REF_5057(uint192) (->lastCheckpoint_2 (-> ['_self'])) := TMP_7996(uint192)
_self_6 (-> ['_self'])(CheckPointHistory.CheckPointHistoryState) := phi(["lastCheckpoint_2 (-> ['_self'])"])
 assert(bool)(block.number > lastBlock)
TMP_7997(bool) = block.number > lastBlock_1
TMP_7998(None) = SOLIDITY_CALL assert(bool)(TMP_7997)
 _self.checkpoints[historyCount] = CheckPoint({fromBlock:block.number.toUint64(),value:_toUint192(_value)})
REF_5058(mapping(uint256 => CheckPointHistory.CheckPoint)) -> _self_1 (-> []).checkpoints
REF_5059(CheckPointHistory.CheckPoint) -> REF_5058[historyCount_1]
TMP_7999(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['block.number'] 
TMP_8000(uint192) = INTERNAL_CALL, CheckPointHistory._toUint192(uint256)(_value_1)
TMP_8001(CheckPointHistory.CheckPoint) = new CheckPoint(TMP_8000,TMP_7999)
_self_2 (-> [])(CheckPointHistory.CheckPointHistoryState) := phi(['_self_1 (-> [])'])
REF_5059(CheckPointHistory.CheckPoint) (->_self_2 (-> [])) := TMP_8001(CheckPointHistory.CheckPoint)
 _self.endIndex = uint64(historyCount + 1)
REF_5061(uint64) -> _self_2 (-> []).endIndex
TMP_8002(uint256) = historyCount_1 (c)+ 1
TMP_8003 = CONVERT TMP_8002 to uint64
_self_3 (-> [])(CheckPointHistory.CheckPointHistoryState) := phi(['_self_2 (-> [])'])
REF_5061(uint64) (->_self_3 (-> [])) := TMP_8003(uint64)
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
#### CheckPointHistory._indexOfGreatestBlockLessThan(mapping(uint256 => CheckPointHistory.CheckPoint),uint256,uint256,uint256) [PRIVATE]
```slithir
_checkpoints_1 (-> [])(mapping(uint256 => CheckPointHistory.CheckPoint)) := phi(['REF_5040'])
_startIndex_1(uint256) := phi(['startIndex_1'])
_endIndex_1(uint256) := phi(['REF_5041'])
_blockNumber_1(uint256) := phi(['_blockNumber_1'])
 min = _startIndex
min_1(uint256) := _startIndex_1(uint256)
 max = _endIndex - 1
TMP_7970(uint256) = _endIndex_1 (c)- 1
max_1(uint256) := TMP_7970(uint256)
 max > min
TMP_7971(bool) = max_1 > min_1
CONDITION TMP_7971
 mid = (max + min + 1) / 2
TMP_7972(uint256) = max_1 (c)+ min_1
TMP_7973(uint256) = TMP_7972 (c)+ 1
TMP_7974(uint256) = TMP_7973 (c)/ 2
mid_1(uint256) := TMP_7974(uint256)
 _checkpoints[mid].fromBlock <= _blockNumber
REF_5027(CheckPointHistory.CheckPoint) -> _checkpoints_1 (-> [])[mid_1]
REF_5028(uint64) -> REF_5027.fromBlock
TMP_7975(bool) = REF_5028 <= _blockNumber_1
CONDITION TMP_7975
 min = mid
min_2(uint256) := mid_1(uint256)
 max = mid - 1
TMP_7976(uint256) = mid_1 (c)- 1
max_2(uint256) := TMP_7976(uint256)
min_3(uint256) := phi(['min_2', 'min_1'])
max_3(uint256) := phi(['max_2', 'max_1'])
 min
RETURN min_1
 index
```

