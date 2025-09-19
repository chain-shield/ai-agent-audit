
#### LibDiamond.addFunctions(address,bytes4[]) [INTERNAL]
```slithir
_facetAddress_1(address) := phi(['facetAddress_1'])
_functionSelectors_1(bytes4[]) := phi(['functionSelectors_1'])
 _facetAddress == address(0)
TMP_7309 = CONVERT 0 to address
TMP_7310(bool) = _facetAddress_1 == TMP_7309
CONDITION TMP_7310
 revert CannotAddSelectorsToZeroAddress(bytes4[])(_functionSelectors)
TMP_7311(None) = SOLIDITY_CALL revert CannotAddSelectorsToZeroAddress(bytes4[])(_functionSelectors_1)
 ds = diamondStorage()
TMP_7312(LibDiamond.DiamondStorage) = INTERNAL_CALL, LibDiamond.diamondStorage()()
ds_1 (-> ['TMP_7312'])(LibDiamond.DiamondStorage) := TMP_7312(LibDiamond.DiamondStorage)
 selectorCount = uint16(ds.selectors.length)
REF_4761(bytes4[]) -> ds_1 (-> ['TMP_7312']).selectors
REF_4762 -> LENGTH REF_4761
TMP_7313 = CONVERT REF_4762 to uint16
selectorCount_1(uint16) := TMP_7313(uint16)
 enforceHasContractCode(_facetAddress,LibDiamondCut: Add facet has no code)
INTERNAL_CALL, LibDiamond.enforceHasContractCode(address,string)(_facetAddress_1,LibDiamondCut: Add facet has no code)
 selectorIndex < _functionSelectors.length
ds_2 (-> ['TMP_7312'])(LibDiamond.DiamondStorage) := phi(["ds_5 (-> ['TMP_7312'])", "ds_1 (-> ['TMP_7312'])"])
selectorCount_2(uint16) := phi(['selectorCount_3', 'selectorCount_1'])
selectorIndex_1(uint256) := phi(['selectorIndex_2', 'selectorIndex_0'])
REF_4763 -> LENGTH _functionSelectors_1
TMP_7315(bool) = selectorIndex_1 < REF_4763
CONDITION TMP_7315
 selector = _functionSelectors[selectorIndex]
REF_4764(bytes4) -> _functionSelectors_1[selectorIndex_1]
selector_1(bytes4) := REF_4764(bytes4)
 oldFacetAddress = ds.facetAddressAndSelectorPosition[selector].facetAddress
REF_4765(mapping(bytes4 => LibDiamond.FacetAddressAndSelectorPosition)) -> ds_2 (-> ['TMP_7312']).facetAddressAndSelectorPosition
REF_4766(LibDiamond.FacetAddressAndSelectorPosition) -> REF_4765[selector_1]
REF_4767(address) -> REF_4766.facetAddress
oldFacetAddress_1(address) := REF_4767(address)
 oldFacetAddress != address(0)
TMP_7316 = CONVERT 0 to address
TMP_7317(bool) = oldFacetAddress_1 != TMP_7316
CONDITION TMP_7317
 revert CannotAddFunctionToDiamondThatAlreadyExists(bytes4)(selector)
TMP_7318(None) = SOLIDITY_CALL revert CannotAddFunctionToDiamondThatAlreadyExists(bytes4)(selector_1)
 ds.facetAddressAndSelectorPosition[selector] = FacetAddressAndSelectorPosition(_facetAddress,selectorCount)
REF_4768(mapping(bytes4 => LibDiamond.FacetAddressAndSelectorPosition)) -> ds_2 (-> ['TMP_7312']).facetAddressAndSelectorPosition
REF_4769(LibDiamond.FacetAddressAndSelectorPosition) -> REF_4768[selector_1]
TMP_7319(LibDiamond.FacetAddressAndSelectorPosition) = new FacetAddressAndSelectorPosition(_facetAddress_1,selectorCount_2)
ds_3 (-> ['TMP_7312'])(LibDiamond.DiamondStorage) := phi(["ds_2 (-> ['TMP_7312'])"])
REF_4769(LibDiamond.FacetAddressAndSelectorPosition) (->ds_3 (-> ['TMP_7312'])) := TMP_7319(LibDiamond.FacetAddressAndSelectorPosition)
TMP_7312(LibDiamond.DiamondStorage) := phi(["ds_3 (-> ['TMP_7312'])"])
 ds.selectors.push(selector)
REF_4770(bytes4[]) -> ds_3 (-> ['TMP_7312']).selectors
REF_4772 -> LENGTH REF_4770
TMP_7321(uint256) := REF_4772(uint256)
TMP_7322(uint256) = TMP_7321 (c)+ 1
ds_4 (-> ['TMP_7312'])(LibDiamond.DiamondStorage) := phi(["ds_3 (-> ['TMP_7312'])"])
REF_4772(uint256) (->ds_5 (-> ['TMP_7312'])) := TMP_7322(uint256)
REF_4773(bytes4) -> REF_4770[TMP_7321]
ds_5 (-> ['TMP_7312'])(LibDiamond.DiamondStorage) := phi(["ds_4 (-> ['TMP_7312'])"])
REF_4773(bytes4) (->ds_5 (-> ['TMP_7312'])) := selector_1(bytes4)
TMP_7312(LibDiamond.DiamondStorage) := phi(["ds_5 (-> ['TMP_7312'])"])
 selectorCount ++
TMP_7323(uint16) := selectorCount_2(uint16)
selectorCount_3(uint16) = selectorCount_2 (c)+ 1
 selectorIndex ++
TMP_7324(uint256) := selectorIndex_1(uint256)
selectorIndex_2(uint256) = selectorIndex_1 (c)+ 1
```
#### LibDiamond.diamondCut(IDiamond.FacetCut[],address,bytes) [INTERNAL]
```slithir
 facetIndex < _diamondCut.length
facetIndex_1(uint256) := phi(['facetIndex_0', 'facetIndex_2'])
REF_4749 -> LENGTH _diamondCut_1
TMP_7295(bool) = facetIndex_1 < REF_4749
CONDITION TMP_7295
 functionSelectors = _diamondCut[facetIndex].functionSelectors
REF_4750(IDiamond.FacetCut) -> _diamondCut_1[facetIndex_1]
REF_4751(bytes4[]) -> REF_4750.functionSelectors
functionSelectors_1(bytes4[]) = ['REF_4751(bytes4[])']
 facetAddress = _diamondCut[facetIndex].facetAddress
REF_4752(IDiamond.FacetCut) -> _diamondCut_1[facetIndex_1]
REF_4753(address) -> REF_4752.facetAddress
facetAddress_1(address) := REF_4753(address)
 functionSelectors.length == 0
REF_4754 -> LENGTH functionSelectors_1
TMP_7296(bool) = REF_4754 == 0
CONDITION TMP_7296
 revert NoSelectorsProvidedForFacetForCut(address)(facetAddress)
TMP_7297(None) = SOLIDITY_CALL revert NoSelectorsProvidedForFacetForCut(address)(facetAddress_1)
 action = _diamondCut[facetIndex].action
REF_4755(IDiamond.FacetCut) -> _diamondCut_1[facetIndex_1]
REF_4756(IDiamond.FacetCutAction) -> REF_4755.action
action_1(IDiamond.FacetCutAction) := REF_4756(IDiamond.FacetCutAction)
 action == IDiamond.FacetCutAction.Add
REF_4757(IDiamond.FacetCutAction) -> FacetCutAction.Add
TMP_7298(bool) = action_1 == REF_4757
CONDITION TMP_7298
 addFunctions(facetAddress,functionSelectors)
INTERNAL_CALL, LibDiamond.addFunctions(address,bytes4[])(facetAddress_1,functionSelectors_1)
 action == IDiamond.FacetCutAction.Replace
REF_4758(IDiamond.FacetCutAction) -> FacetCutAction.Replace
TMP_7300(bool) = action_1 == REF_4758
CONDITION TMP_7300
 replaceFunctions(facetAddress,functionSelectors)
INTERNAL_CALL, LibDiamond.replaceFunctions(address,bytes4[])(facetAddress_1,functionSelectors_1)
 action == IDiamond.FacetCutAction.Remove
REF_4759(IDiamond.FacetCutAction) -> FacetCutAction.Remove
TMP_7302(bool) = action_1 == REF_4759
CONDITION TMP_7302
 removeFunctions(facetAddress,functionSelectors)
INTERNAL_CALL, LibDiamond.removeFunctions(address,bytes4[])(facetAddress_1,functionSelectors_1)
 revert IncorrectFacetCutAction(uint8)(uint8(action))
TMP_7304 = CONVERT action_1 to uint8
TMP_7305(None) = SOLIDITY_CALL revert IncorrectFacetCutAction(uint8)(TMP_7304)
 facetIndex ++
TMP_7306(uint256) := facetIndex_1(uint256)
facetIndex_2(uint256) = facetIndex_1 (c)+ 1
 IDiamond.DiamondCut(_diamondCut,_init,_calldata)
Emit DiamondCut(_diamondCut_1,_init_1,_calldata_1)
 initializeDiamondCut(_init,_calldata)
INTERNAL_CALL, LibDiamond.initializeDiamondCut(address,bytes)(_init_1,_calldata_1)
```
#### LibDiamond.diamondStorage() [INTERNAL]
```slithir
DIAMOND_STORAGE_POSITION_1(bytes32) := phi(['DIAMOND_STORAGE_POSITION_0'])
 position = DIAMOND_STORAGE_POSITION
position_1(bytes32) := DIAMOND_STORAGE_POSITION_1(bytes32)
 ds = position
ds_1 (-> ['position'])(LibDiamond.DiamondStorage) := position_1(bytes32)
 ds
RETURN ds_1 (-> ['position'])
```
#### LibDiamond.enforceHasContractCode(address,string) [INTERNAL]
```slithir
_contract_1(address) := phi(['_facetAddress_1', '_init_1', '_facetAddress_1'])
 contractSize = extcodesize(uint256)(_contract)
REF_4809 -> CODESIZE _contract_1
contractSize_1(uint256) := REF_4809(uint256)
 contractSize == 0
TMP_7365(bool) = contractSize_1 == 0
CONDITION TMP_7365
 revert NoBytecodeAtAddress(address,string)(_contract,_errorMessage)
TMP_7366(None) = SOLIDITY_CALL revert NoBytecodeAtAddress(address,string)(_contract_1,_errorMessage_1)
```
#### LibDiamond.initializeDiamondCut(address,bytes) [INTERNAL]
```slithir
_init_1(address) := phi(['_init_1'])
_calldata_1(bytes) := phi(['_calldata_1'])
 _init == address(0)
TMP_7356 = CONVERT 0 to address
TMP_7357(bool) = _init_1 == TMP_7356
CONDITION TMP_7357
 enforceHasContractCode(_init,LibDiamondCut: _init address has no code)
INTERNAL_CALL, LibDiamond.enforceHasContractCode(address,string)(_init_1,LibDiamondCut: _init address has no code)
 (success,error) = _init.delegatecall(_calldata)
TUPLE_71(bool,bytes) = LOW_LEVEL_CALL, dest:_init_1, function:delegatecall, arguments:['_calldata_1']  
success_1(bool)= UNPACK TUPLE_71 index: 0 
error_1(bytes)= UNPACK TUPLE_71 index: 1 
 ! success
TMP_7359 = UnaryType.BANG success_1 
CONDITION TMP_7359
 error.length > 0
REF_4808 -> LENGTH error_1
TMP_7360(bool) = REF_4808 > 0
CONDITION TMP_7360
 returndata_size_initializeDiamondCut_asm_0 = mload(uint256)(error)
TMP_7361(uint256) = SOLIDITY_CALL mload(uint256)(error_1)
returndata_size_initializeDiamondCut_asm_0_1(uint256) := TMP_7361(uint256)
 revert(uint256,uint256)(32 + error,returndata_size_initializeDiamondCut_asm_0)
TMP_7362(uint256) = 32 + error_1
TMP_7363(None) = SOLIDITY_CALL revert(uint256,uint256)(TMP_7362,returndata_size_initializeDiamondCut_asm_0_1)
 revert InitializationFunctionReverted(address,bytes)(_init,_calldata)
TMP_7364(None) = SOLIDITY_CALL revert InitializationFunctionReverted(address,bytes)(_init_1,_calldata_1)
```
#### LibDiamond.removeFunctions(address,bytes4[]) [INTERNAL]
```slithir
_facetAddress_1(address) := phi(['facetAddress_1'])
_functionSelectors_1(bytes4[]) := phi(['functionSelectors_1'])
 ds = diamondStorage()
TMP_7340(LibDiamond.DiamondStorage) = INTERNAL_CALL, LibDiamond.diamondStorage()()
ds_1 (-> ['TMP_7340'])(LibDiamond.DiamondStorage) := TMP_7340(LibDiamond.DiamondStorage)
 selectorCount = ds.selectors.length
REF_4782(bytes4[]) -> ds_1 (-> ['TMP_7340']).selectors
REF_4783 -> LENGTH REF_4782
selectorCount_1(uint256) := REF_4783(uint256)
 _facetAddress != address(0)
TMP_7341 = CONVERT 0 to address
TMP_7342(bool) = _facetAddress_1 != TMP_7341
CONDITION TMP_7342
 revert RemoveFacetAddressMustBeZeroAddress(address)(_facetAddress)
TMP_7343(None) = SOLIDITY_CALL revert RemoveFacetAddressMustBeZeroAddress(address)(_facetAddress_1)
 selectorIndex < _functionSelectors.length
ds_2 (-> ['TMP_7340'])(LibDiamond.DiamondStorage) := phi(["ds_6 (-> ['TMP_7340'])", "ds_1 (-> ['TMP_7340'])"])
selectorCount_2(uint256) := phi(['selectorCount_1', 'selectorCount_3'])
selectorIndex_1(uint256) := phi(['selectorIndex_0', 'selectorIndex_2'])
REF_4784 -> LENGTH _functionSelectors_1
TMP_7344(bool) = selectorIndex_1 < REF_4784
CONDITION TMP_7344
 selector = _functionSelectors[selectorIndex]
REF_4785(bytes4) -> _functionSelectors_1[selectorIndex_1]
selector_1(bytes4) := REF_4785(bytes4)
 oldFacetAddressAndSelectorPosition = ds.facetAddressAndSelectorPosition[selector]
REF_4786(mapping(bytes4 => LibDiamond.FacetAddressAndSelectorPosition)) -> ds_2 (-> ['TMP_7340']).facetAddressAndSelectorPosition
REF_4787(LibDiamond.FacetAddressAndSelectorPosition) -> REF_4786[selector_1]
oldFacetAddressAndSelectorPosition_1(LibDiamond.FacetAddressAndSelectorPosition) := REF_4787(LibDiamond.FacetAddressAndSelectorPosition)
 oldFacetAddressAndSelectorPosition.facetAddress == address(0)
REF_4788(address) -> oldFacetAddressAndSelectorPosition_1.facetAddress
TMP_7345 = CONVERT 0 to address
TMP_7346(bool) = REF_4788 == TMP_7345
CONDITION TMP_7346
 revert CannotRemoveFunctionThatDoesNotExist(bytes4)(selector)
TMP_7347(None) = SOLIDITY_CALL revert CannotRemoveFunctionThatDoesNotExist(bytes4)(selector_1)
 oldFacetAddressAndSelectorPosition.facetAddress == address(this)
REF_4789(address) -> oldFacetAddressAndSelectorPosition_1.facetAddress
TMP_7348 = CONVERT this to address
TMP_7349(bool) = REF_4789 == TMP_7348
CONDITION TMP_7349
 revert CannotRemoveImmutableFunction(bytes4)(selector)
TMP_7350(None) = SOLIDITY_CALL revert CannotRemoveImmutableFunction(bytes4)(selector_1)
 selectorCount --
TMP_7351(uint256) := selectorCount_2(uint256)
selectorCount_3(uint256) = selectorCount_2 (c)- 1
 oldFacetAddressAndSelectorPosition.selectorPosition != selectorCount
REF_4790(uint16) -> oldFacetAddressAndSelectorPosition_1.selectorPosition
TMP_7352(bool) = REF_4790 != selectorCount_3
CONDITION TMP_7352
 lastSelector = ds.selectors[selectorCount]
REF_4791(bytes4[]) -> ds_2 (-> ['TMP_7340']).selectors
REF_4792(bytes4) -> REF_4791[selectorCount_3]
lastSelector_1(bytes4) := REF_4792(bytes4)
 ds.selectors[oldFacetAddressAndSelectorPosition.selectorPosition] = lastSelector
REF_4793(bytes4[]) -> ds_2 (-> ['TMP_7340']).selectors
REF_4794(uint16) -> oldFacetAddressAndSelectorPosition_1.selectorPosition
REF_4795(bytes4) -> REF_4793[REF_4794]
ds_3 (-> ['TMP_7340'])(LibDiamond.DiamondStorage) := phi(["ds_2 (-> ['TMP_7340'])"])
REF_4795(bytes4) (->ds_3 (-> ['TMP_7340'])) := lastSelector_1(bytes4)
TMP_7340(LibDiamond.DiamondStorage) := phi(["ds_3 (-> ['TMP_7340'])"])
 ds.facetAddressAndSelectorPosition[lastSelector].selectorPosition = oldFacetAddressAndSelectorPosition.selectorPosition
REF_4796(mapping(bytes4 => LibDiamond.FacetAddressAndSelectorPosition)) -> ds_3 (-> ['TMP_7340']).facetAddressAndSelectorPosition
REF_4797(LibDiamond.FacetAddressAndSelectorPosition) -> REF_4796[lastSelector_1]
REF_4798(uint16) -> REF_4797.selectorPosition
REF_4799(uint16) -> oldFacetAddressAndSelectorPosition_1.selectorPosition
ds_4 (-> ['TMP_7340'])(LibDiamond.DiamondStorage) := phi(["ds_3 (-> ['TMP_7340'])"])
REF_4798(uint16) (->ds_4 (-> ['TMP_7340'])) := REF_4799(uint16)
TMP_7340(LibDiamond.DiamondStorage) := phi(["ds_4 (-> ['TMP_7340'])"])
ds_5 (-> ['TMP_7340'])(LibDiamond.DiamondStorage) := phi(["ds_4 (-> ['TMP_7340'])", "ds_1 (-> ['TMP_7340'])"])
 ds.selectors.pop()
REF_4800(bytes4[]) -> ds_5 (-> ['TMP_7340']).selectors
REF_4802 -> LENGTH REF_4800
TMP_7354(uint256) = REF_4802 (c)- 1
REF_4803(bytes4) -> REF_4800[TMP_7354]
REF_4800 = delete REF_4803 
REF_4804 -> LENGTH REF_4800
ds_6 (-> ['TMP_7340'])(LibDiamond.DiamondStorage) := phi(["ds_5 (-> ['TMP_7340'])"])
REF_4804(uint256) (->ds_6 (-> ['TMP_7340'])) := TMP_7354(uint256)
TMP_7340(LibDiamond.DiamondStorage) := phi(["ds_6 (-> ['TMP_7340'])"])
 delete ds.facetAddressAndSelectorPosition[selector]
REF_4805(mapping(bytes4 => LibDiamond.FacetAddressAndSelectorPosition)) -> ds_6 (-> ['TMP_7340']).facetAddressAndSelectorPosition
REF_4806(LibDiamond.FacetAddressAndSelectorPosition) -> REF_4805[selector_1]
REF_4805 = delete REF_4806 
 selectorIndex ++
TMP_7355(uint256) := selectorIndex_1(uint256)
selectorIndex_2(uint256) = selectorIndex_1 (c)+ 1
```
#### LibDiamond.replaceFunctions(address,bytes4[]) [INTERNAL]
```slithir
_facetAddress_1(address) := phi(['facetAddress_1'])
_functionSelectors_1(bytes4[]) := phi(['functionSelectors_1'])
 ds = diamondStorage()
TMP_7325(LibDiamond.DiamondStorage) = INTERNAL_CALL, LibDiamond.diamondStorage()()
ds_1 (-> ['TMP_7325'])(LibDiamond.DiamondStorage) := TMP_7325(LibDiamond.DiamondStorage)
 _facetAddress == address(0)
TMP_7326 = CONVERT 0 to address
TMP_7327(bool) = _facetAddress_1 == TMP_7326
CONDITION TMP_7327
 revert CannotReplaceFunctionsFromFacetWithZeroAddress(bytes4[])(_functionSelectors)
TMP_7328(None) = SOLIDITY_CALL revert CannotReplaceFunctionsFromFacetWithZeroAddress(bytes4[])(_functionSelectors_1)
 enforceHasContractCode(_facetAddress,LibDiamondCut: Replace facet has no code)
INTERNAL_CALL, LibDiamond.enforceHasContractCode(address,string)(_facetAddress_1,LibDiamondCut: Replace facet has no code)
 selectorIndex < _functionSelectors.length
ds_2 (-> ['TMP_7325'])(LibDiamond.DiamondStorage) := phi(["ds_1 (-> ['TMP_7325'])", "ds_3 (-> ['TMP_7325'])"])
selectorIndex_1(uint256) := phi(['selectorIndex_2', 'selectorIndex_0'])
REF_4774 -> LENGTH _functionSelectors_1
TMP_7330(bool) = selectorIndex_1 < REF_4774
CONDITION TMP_7330
 selector = _functionSelectors[selectorIndex]
REF_4775(bytes4) -> _functionSelectors_1[selectorIndex_1]
selector_1(bytes4) := REF_4775(bytes4)
 oldFacetAddress = ds.facetAddressAndSelectorPosition[selector].facetAddress
REF_4776(mapping(bytes4 => LibDiamond.FacetAddressAndSelectorPosition)) -> ds_2 (-> ['TMP_7325']).facetAddressAndSelectorPosition
REF_4777(LibDiamond.FacetAddressAndSelectorPosition) -> REF_4776[selector_1]
REF_4778(address) -> REF_4777.facetAddress
oldFacetAddress_1(address) := REF_4778(address)
 oldFacetAddress == address(this)
TMP_7331 = CONVERT this to address
TMP_7332(bool) = oldFacetAddress_1 == TMP_7331
CONDITION TMP_7332
 revert CannotReplaceImmutableFunction(bytes4)(selector)
TMP_7333(None) = SOLIDITY_CALL revert CannotReplaceImmutableFunction(bytes4)(selector_1)
 oldFacetAddress == _facetAddress
TMP_7334(bool) = oldFacetAddress_1 == _facetAddress_1
CONDITION TMP_7334
 revert CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet(bytes4)(selector)
TMP_7335(None) = SOLIDITY_CALL revert CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet(bytes4)(selector_1)
 oldFacetAddress == address(0)
TMP_7336 = CONVERT 0 to address
TMP_7337(bool) = oldFacetAddress_1 == TMP_7336
CONDITION TMP_7337
 revert CannotReplaceFunctionThatDoesNotExists(bytes4)(selector)
TMP_7338(None) = SOLIDITY_CALL revert CannotReplaceFunctionThatDoesNotExists(bytes4)(selector_1)
 ds.facetAddressAndSelectorPosition[selector].facetAddress = _facetAddress
REF_4779(mapping(bytes4 => LibDiamond.FacetAddressAndSelectorPosition)) -> ds_2 (-> ['TMP_7325']).facetAddressAndSelectorPosition
REF_4780(LibDiamond.FacetAddressAndSelectorPosition) -> REF_4779[selector_1]
REF_4781(address) -> REF_4780.facetAddress
ds_3 (-> ['TMP_7325'])(LibDiamond.DiamondStorage) := phi(["ds_2 (-> ['TMP_7325'])"])
REF_4781(address) (->ds_3 (-> ['TMP_7325'])) := _facetAddress_1(address)
TMP_7325(LibDiamond.DiamondStorage) := phi(["ds_3 (-> ['TMP_7325'])"])
 selectorIndex ++
TMP_7339(uint256) := selectorIndex_1(uint256)
selectorIndex_2(uint256) = selectorIndex_1 (c)+ 1
```

