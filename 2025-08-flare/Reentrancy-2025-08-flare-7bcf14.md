
#### Reentrancy._reentrancyGuardState() [PRIVATE]
```slithir
 position = keccak256(bytes)(utils.ReentrancyGuard.ReentrancyGuardState)
TMP_9998(bytes32) = SOLIDITY_CALL keccak256(bytes)(utils.ReentrancyGuard.ReentrancyGuardState)
position_1(bytes32) := TMP_9998(bytes32)
 _state = position
_state_1 (-> ['position'])(Reentrancy.ReentrancyGuardState) := position_1(bytes32)
 _state
RETURN _state_1 (-> ['position'])
```
#### Reentrancy.initializeReentrancyGuard() [INTERNAL]
```slithir
_NOT_ENTERED_1(uint256) := phi(['_NOT_ENTERED_2', '_NOT_ENTERED_0', '_NOT_ENTERED_4'])
 state = _reentrancyGuardState()
TMP_9987(Reentrancy.ReentrancyGuardState) = INTERNAL_CALL, Reentrancy._reentrancyGuardState()()
state_1 (-> ['TMP_9987'])(Reentrancy.ReentrancyGuardState) := TMP_9987(Reentrancy.ReentrancyGuardState)
 state.status = _NOT_ENTERED
REF_6037(uint256) -> state_1 (-> ['TMP_9987']).status
state_2 (-> ['TMP_9987'])(Reentrancy.ReentrancyGuardState) := phi(["state_1 (-> ['TMP_9987'])"])
REF_6037(uint256) (->state_2 (-> ['TMP_9987'])) := _NOT_ENTERED_2(uint256)
TMP_9987(Reentrancy.ReentrancyGuardState) := phi(["state_2 (-> ['TMP_9987'])"])
```
#### Reentrancy.nonReentrantAfter() [INTERNAL]
```slithir
_NOT_ENTERED_3(uint256) := phi(['_NOT_ENTERED_2', '_NOT_ENTERED_0', '_NOT_ENTERED_4'])
 state = _reentrancyGuardState()
TMP_9992(Reentrancy.ReentrancyGuardState) = INTERNAL_CALL, Reentrancy._reentrancyGuardState()()
state_1 (-> ['TMP_9992'])(Reentrancy.ReentrancyGuardState) := TMP_9992(Reentrancy.ReentrancyGuardState)
 state.status = _NOT_ENTERED
REF_6040(uint256) -> state_1 (-> ['TMP_9992']).status
state_2 (-> ['TMP_9992'])(Reentrancy.ReentrancyGuardState) := phi(["state_1 (-> ['TMP_9992'])"])
REF_6040(uint256) (->state_2 (-> ['TMP_9992'])) := _NOT_ENTERED_4(uint256)
TMP_9992(Reentrancy.ReentrancyGuardState) := phi(["state_2 (-> ['TMP_9992'])"])
```
#### Reentrancy.nonReentrantBefore() [INTERNAL]
```slithir
_ENTERED_1(uint256) := phi(['_ENTERED_0', '_ENTERED_4', '_ENTERED_2'])
 state = _reentrancyGuardState()
TMP_9988(Reentrancy.ReentrancyGuardState) = INTERNAL_CALL, Reentrancy._reentrancyGuardState()()
state_1 (-> ['TMP_9988'])(Reentrancy.ReentrancyGuardState) := TMP_9988(Reentrancy.ReentrancyGuardState)
 require(bool,error)(state.status != _ENTERED,revert ReentrancyGuardReentrantCall()())
REF_6038(uint256) -> state_1 (-> ['TMP_9988']).status
TMP_9989(bool) = REF_6038 != _ENTERED_2
TMP_9990(None) = SOLIDITY_CALL revert ReentrancyGuardReentrantCall()()
TMP_9991(None) = SOLIDITY_CALL require(bool,error)(TMP_9989,TMP_9990)
 state.status = _ENTERED
REF_6039(uint256) -> state_1 (-> ['TMP_9988']).status
state_2 (-> ['TMP_9988'])(Reentrancy.ReentrancyGuardState) := phi(["state_1 (-> ['TMP_9988'])"])
REF_6039(uint256) (->state_2 (-> ['TMP_9988'])) := _ENTERED_2(uint256)
TMP_9988(Reentrancy.ReentrancyGuardState) := phi(["state_2 (-> ['TMP_9988'])"])
```
#### Reentrancy.reentrancyGuardEntered() [INTERNAL]
```slithir
_ENTERED_3(uint256) := phi(['_ENTERED_0', '_ENTERED_4', '_ENTERED_2'])
 state = _reentrancyGuardState()
TMP_9993(Reentrancy.ReentrancyGuardState) = INTERNAL_CALL, Reentrancy._reentrancyGuardState()()
state_1 (-> ['TMP_9993'])(Reentrancy.ReentrancyGuardState) := TMP_9993(Reentrancy.ReentrancyGuardState)
 state.status == _ENTERED
REF_6041(uint256) -> state_1 (-> ['TMP_9993']).status
TMP_9994(bool) = REF_6041 == _ENTERED_4
RETURN TMP_9994
```
#### Reentrancy.requireReentrancyGuard() [INTERNAL]
```slithir
 require(bool,error)(reentrancyGuardEntered(),revert ReentrancyGuardRequired()())
TMP_9995(bool) = INTERNAL_CALL, Reentrancy.reentrancyGuardEntered()()
TMP_9996(None) = SOLIDITY_CALL revert ReentrancyGuardRequired()()
TMP_9997(None) = SOLIDITY_CALL require(bool,error)(TMP_9995,TMP_9996)
```

