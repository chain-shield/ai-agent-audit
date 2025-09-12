


#### Test1Facet.test1Func1() [EXTERNAL]
```slithir
 TestLib.setMyAddress(address(this))
TMP_7563 = CONVERT this to address
LIBRARY_CALL, dest:TestLib, function:TestLib.setMyAddress(address), arguments:['TMP_7563']
```
#### Test1Facet.test1Func10() [EXTERNAL]
```slithir

```
#### Test1Facet.test1Func11() [EXTERNAL]
```slithir

```
#### Test1Facet.test1Func12() [EXTERNAL]
```slithir

```
#### Test1Facet.test1Func13() [EXTERNAL]
```slithir

```
#### Test1Facet.test1Func14() [EXTERNAL]
```slithir

```
#### Test1Facet.test1Func15() [EXTERNAL]
```slithir

```
#### Test1Facet.test1Func16() [EXTERNAL]
```slithir

```
#### Test1Facet.test1Func17() [EXTERNAL]
```slithir

```
#### Test1Facet.test1Func18() [EXTERNAL]
```slithir

```
#### Test1Facet.test1Func19() [EXTERNAL]
```slithir

```
#### Test1Facet.test1Func2() [EXTERNAL]
```slithir
 TestLib.getMyAddress()
TMP_7565(address) = LIBRARY_CALL, dest:TestLib, function:TestLib.getMyAddress(), arguments:[] 
RETURN TMP_7565
```
#### Test1Facet.test1Func20() [EXTERNAL]
```slithir

```
#### Test1Facet.test1Func3() [EXTERNAL]
```slithir

```
#### Test1Facet.test1Func4() [EXTERNAL]
```slithir

```
#### Test1Facet.test1Func5() [EXTERNAL]
```slithir

```
#### Test1Facet.test1Func6() [EXTERNAL]
```slithir

```
#### Test1Facet.test1Func7() [EXTERNAL]
```slithir

```
#### Test1Facet.test1Func8() [EXTERNAL]
```slithir

```
#### Test1Facet.test1Func9() [EXTERNAL]
```slithir

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
#### TestLib.getMyAddress() [INTERNAL]
```slithir
 testState = diamondStorage()
TMP_7567(TestLib.TestState) = INTERNAL_CALL, TestLib.diamondStorage()()
testState_1 (-> ['TMP_7567'])(TestLib.TestState) := TMP_7567(TestLib.TestState)
 testState.myAddress
REF_4902(address) -> testState_1 (-> ['TMP_7567']).myAddress
RETURN REF_4902
```
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
