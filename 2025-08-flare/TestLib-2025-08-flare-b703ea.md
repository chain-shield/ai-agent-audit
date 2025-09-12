
#### TestLib.diamondStorage() [INTERNAL]
```slithir
DIAMOND_STORAGE_POSITION_1(bytes32) := phi(['DIAMOND_STORAGE_POSITION_0'])
 position = DIAMOND_STORAGE_POSITION
position_1(bytes32) := DIAMOND_STORAGE_POSITION_1(bytes32)
 ds = position
ds_1 (-> ['position'])(TestLib.TestState) := position_1(bytes32)
 ds
RETURN ds_1 (-> ['position'])
```
#### TestLib.getMyAddress() [INTERNAL]
```slithir
 testState = diamondStorage()
TMP_7567(TestLib.TestState) = INTERNAL_CALL, TestLib.diamondStorage()()
testState_1 (-> ['TMP_7567'])(TestLib.TestState) := TMP_7567(TestLib.TestState)
 testState.myAddress
REF_4902(address) -> testState_1 (-> ['TMP_7567']).myAddress
RETURN REF_4902
```
#### TestLib.setMyAddress(address) [INTERNAL]
```slithir
 testState = diamondStorage()
TMP_7566(TestLib.TestState) = INTERNAL_CALL, TestLib.diamondStorage()()
testState_1 (-> ['TMP_7566'])(TestLib.TestState) := TMP_7566(TestLib.TestState)
 testState.myAddress = _myAddress
REF_4901(address) -> testState_1 (-> ['TMP_7566']).myAddress
testState_2 (-> ['TMP_7566'])(TestLib.TestState) := phi(["testState_1 (-> ['TMP_7566'])"])
REF_4901(address) (->testState_2 (-> ['TMP_7566'])) := _myAddress_1(address)
TMP_7566(TestLib.TestState) := phi(["testState_2 (-> ['TMP_7566'])"])
```

