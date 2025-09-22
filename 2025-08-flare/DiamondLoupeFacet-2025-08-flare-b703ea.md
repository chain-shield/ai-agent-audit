

#### DiamondLoupeFacet.facetAddress(bytes4) [EXTERNAL]
```slithir
 ds = LibDiamond.diamondStorage()
TMP_7278(LibDiamond.DiamondStorage) = LIBRARY_CALL, dest:LibDiamond, function:LibDiamond.diamondStorage(), arguments:[] 
ds_1 (-> ['TMP_7278'])(LibDiamond.DiamondStorage) := TMP_7278(LibDiamond.DiamondStorage)
 facetAddress_ = ds.facetAddressAndSelectorPosition[_functionSelector].facetAddress
REF_4739(mapping(bytes4 => LibDiamond.FacetAddressAndSelectorPosition)) -> ds_1 (-> ['TMP_7278']).facetAddressAndSelectorPosition
REF_4740(LibDiamond.FacetAddressAndSelectorPosition) -> REF_4739[_functionSelector_1]
REF_4741(address) -> REF_4740.facetAddress
facetAddress__1(address) := REF_4741(address)
 facetAddress_
RETURN facetAddress__1
```
#### DiamondLoupeFacet.facetAddresses() [EXTERNAL]
```slithir
 ds = LibDiamond.diamondStorage()
TMP_7268(LibDiamond.DiamondStorage) = LIBRARY_CALL, dest:LibDiamond, function:LibDiamond.diamondStorage(), arguments:[] 
ds_1 (-> ['TMP_7268'])(LibDiamond.DiamondStorage) := TMP_7268(LibDiamond.DiamondStorage)
 selectorCount = ds.selectors.length
REF_4729(bytes4[]) -> ds_1 (-> ['TMP_7268']).selectors
REF_4730 -> LENGTH REF_4729
selectorCount_1(uint256) := REF_4730(uint256)
 facetAddresses_ = new address[](selectorCount)
TMP_7270(address[])  = new address[](selectorCount_1)
facetAddresses__1(address[]) = ['TMP_7270(address[])']
 selectorIndex < selectorCount
selectorIndex_1(uint256) := phi(['selectorIndex_0', 'selectorIndex_2'])
TMP_7271(bool) = selectorIndex_1 < selectorCount_1
CONDITION TMP_7271
 selector = ds.selectors[selectorIndex]
REF_4731(bytes4[]) -> ds_1 (-> ['TMP_7268']).selectors
REF_4732(bytes4) -> REF_4731[selectorIndex_1]
selector_1(bytes4) := REF_4732(bytes4)
 facetAddress_ = ds.facetAddressAndSelectorPosition[selector].facetAddress
REF_4733(mapping(bytes4 => LibDiamond.FacetAddressAndSelectorPosition)) -> ds_1 (-> ['TMP_7268']).facetAddressAndSelectorPosition
REF_4734(LibDiamond.FacetAddressAndSelectorPosition) -> REF_4733[selector_1]
REF_4735(address) -> REF_4734.facetAddress
facetAddress__1(address) := REF_4735(address)
 continueLoop = false
continueLoop_1(bool) := False(bool)
continueLoop_3(bool) := phi(['continueLoop_1', 'continueLoop_2'])
 facetIndex < numFacets
facetIndex_1(uint256) := phi(['facetIndex_0', 'facetIndex_2'])
TMP_7272(bool) = facetIndex_1 < numFacets_0
CONDITION TMP_7272
 facetAddress_ == facetAddresses_[facetIndex]
REF_4736(address) -> facetAddresses__1[facetIndex_1]
TMP_7273(bool) = facetAddress__1 == REF_4736
CONDITION TMP_7273
 continueLoop = true
continueLoop_2(bool) := True(bool)
 facetIndex ++
TMP_7274(uint256) := facetIndex_1(uint256)
facetIndex_2(uint256) = facetIndex_1 (c)+ 1
 continueLoop
CONDITION continueLoop_3
 continueLoop = false
continueLoop_4(bool) := False(bool)
 facetAddresses_[numFacets] = facetAddress_
REF_4737(address) -> facetAddresses__1[numFacets_0]
facetAddresses__3(address[]) := phi(['facetAddresses__1'])
REF_4737(address) (->facetAddresses__3) := facetAddress__1(address)
 numFacets ++
TMP_7275(uint256) := numFacets_0(uint256)
numFacets_2(uint256) = numFacets_0 (c)+ 1
 selectorIndex ++
facetAddresses__2(address[]) := phi(['facetAddresses__3', 'facetAddresses__1'])
numFacets_1(uint256) := phi(['numFacets_2', 'numFacets_0'])
TMP_7276(uint256) := selectorIndex_1(uint256)
selectorIndex_2(uint256) = selectorIndex_1 (c)+ 1
 mstore(uint256,uint256)(facetAddresses_,numFacets)
TMP_7277(None) = SOLIDITY_CALL mstore(uint256,uint256)(facetAddresses__1,numFacets_0)
 facetAddresses_
RETURN facetAddresses__1
```
#### DiamondLoupeFacet.facetFunctionSelectors(address) [EXTERNAL]
```slithir
 ds = LibDiamond.diamondStorage()
TMP_7260(LibDiamond.DiamondStorage) = LIBRARY_CALL, dest:LibDiamond, function:LibDiamond.diamondStorage(), arguments:[] 
ds_1 (-> ['TMP_7260'])(LibDiamond.DiamondStorage) := TMP_7260(LibDiamond.DiamondStorage)
 selectorCount = ds.selectors.length
REF_4720(bytes4[]) -> ds_1 (-> ['TMP_7260']).selectors
REF_4721 -> LENGTH REF_4720
selectorCount_1(uint256) := REF_4721(uint256)
 _facetFunctionSelectors = new bytes4[](selectorCount)
TMP_7262(bytes4[])  = new bytes4[](selectorCount_1)
_facetFunctionSelectors_1(bytes4[]) = ['TMP_7262(bytes4[])']
 selectorIndex < selectorCount
selectorIndex_1(uint256) := phi(['selectorIndex_0', 'selectorIndex_2'])
TMP_7263(bool) = selectorIndex_1 < selectorCount_1
CONDITION TMP_7263
 selector = ds.selectors[selectorIndex]
REF_4722(bytes4[]) -> ds_1 (-> ['TMP_7260']).selectors
REF_4723(bytes4) -> REF_4722[selectorIndex_1]
selector_1(bytes4) := REF_4723(bytes4)
 facetAddress_ = ds.facetAddressAndSelectorPosition[selector].facetAddress
REF_4724(mapping(bytes4 => LibDiamond.FacetAddressAndSelectorPosition)) -> ds_1 (-> ['TMP_7260']).facetAddressAndSelectorPosition
REF_4725(LibDiamond.FacetAddressAndSelectorPosition) -> REF_4724[selector_1]
REF_4726(address) -> REF_4725.facetAddress
facetAddress__1(address) := REF_4726(address)
 _facet == facetAddress_
TMP_7264(bool) = _facet_1 == facetAddress__1
CONDITION TMP_7264
 _facetFunctionSelectors[numSelectors] = selector
REF_4727(bytes4) -> _facetFunctionSelectors_1[numSelectors_0]
_facetFunctionSelectors_2(bytes4[]) := phi(['_facetFunctionSelectors_1'])
REF_4727(bytes4) (->_facetFunctionSelectors_2) := selector_1(bytes4)
 numSelectors ++
TMP_7265(uint256) := numSelectors_0(uint256)
numSelectors_1(uint256) = numSelectors_0 (c)+ 1
_facetFunctionSelectors_3(bytes4[]) := phi(['_facetFunctionSelectors_1', '_facetFunctionSelectors_2'])
numSelectors_2(uint256) := phi(['numSelectors_1', 'numSelectors_0'])
 selectorIndex ++
TMP_7266(uint256) := selectorIndex_1(uint256)
selectorIndex_2(uint256) = selectorIndex_1 (c)+ 1
 mstore(uint256,uint256)(_facetFunctionSelectors,numSelectors)
TMP_7267(None) = SOLIDITY_CALL mstore(uint256,uint256)(_facetFunctionSelectors_1,numSelectors_0)
 _facetFunctionSelectors
RETURN _facetFunctionSelectors_1
```
#### DiamondLoupeFacet.facets() [EXTERNAL]
```slithir
 ds = LibDiamond.diamondStorage()
TMP_7242(LibDiamond.DiamondStorage) = LIBRARY_CALL, dest:LibDiamond, function:LibDiamond.diamondStorage(), arguments:[] 
ds_1 (-> ['TMP_7242'])(LibDiamond.DiamondStorage) := TMP_7242(LibDiamond.DiamondStorage)
 selectorCount = ds.selectors.length
REF_4694(bytes4[]) -> ds_1 (-> ['TMP_7242']).selectors
REF_4695 -> LENGTH REF_4694
selectorCount_1(uint256) := REF_4695(uint256)
 facets_ = new IDiamondLoupe.Facet[](selectorCount)
TMP_7244(IDiamondLoupe.Facet[])  = new IDiamondLoupe.Facet[](selectorCount_1)
facets__1(IDiamondLoupe.Facet[]) = ['TMP_7244(IDiamondLoupe.Facet[])']
 numFacetSelectors = new uint16[](selectorCount)
TMP_7246(uint16[])  = new uint16[](selectorCount_1)
numFacetSelectors_1(uint16[]) = ['TMP_7246(uint16[])']
 selectorIndex < selectorCount
selectorIndex_1(uint256) := phi(['selectorIndex_0', 'selectorIndex_2'])
TMP_7247(bool) = selectorIndex_1 < selectorCount_1
CONDITION TMP_7247
 selector = ds.selectors[selectorIndex]
REF_4696(bytes4[]) -> ds_1 (-> ['TMP_7242']).selectors
REF_4697(bytes4) -> REF_4696[selectorIndex_1]
selector_1(bytes4) := REF_4697(bytes4)
 facetAddress_ = ds.facetAddressAndSelectorPosition[selector].facetAddress
REF_4698(mapping(bytes4 => LibDiamond.FacetAddressAndSelectorPosition)) -> ds_1 (-> ['TMP_7242']).facetAddressAndSelectorPosition
REF_4699(LibDiamond.FacetAddressAndSelectorPosition) -> REF_4698[selector_1]
REF_4700(address) -> REF_4699.facetAddress
facetAddress__1(address) := REF_4700(address)
 continueLoop = false
continueLoop_1(bool) := False(bool)
facets__3(IDiamondLoupe.Facet[]) := phi(['facets__2', 'facets__1'])
continueLoop_3(bool) := phi(['continueLoop_2', 'continueLoop_1'])
 facetIndex < numFacets
facetIndex_1(uint256) := phi(['facetIndex_0', 'facetIndex_2'])
TMP_7248(bool) = facetIndex_1 < numFacets_0
CONDITION TMP_7248
 facets_[facetIndex].facetAddress == facetAddress_
REF_4701(IDiamondLoupe.Facet) -> facets__1[facetIndex_1]
REF_4702(address) -> REF_4701.facetAddress
TMP_7249(bool) = REF_4702 == facetAddress__1
CONDITION TMP_7249
 facets_[facetIndex].functionSelectors[numFacetSelectors[facetIndex]] = selector
REF_4703(IDiamondLoupe.Facet) -> facets__1[facetIndex_1]
REF_4704(bytes4[]) -> REF_4703.functionSelectors
REF_4705(uint16) -> numFacetSelectors_1[facetIndex_1]
REF_4706(bytes4) -> REF_4704[REF_4705]
facets__2(IDiamondLoupe.Facet[]) := phi(['facets__1'])
REF_4706(bytes4) (->facets__2) := selector_1(bytes4)
 numFacetSelectors[facetIndex] ++
REF_4707(uint16) -> numFacetSelectors_1[facetIndex_1]
TMP_7250(uint16) := REF_4707(uint16)
numFacetSelectors_2(uint16[]) := phi(['numFacetSelectors_1'])
REF_4707(-> numFacetSelectors_2) = REF_4707 (c)+ 1
 continueLoop = true
continueLoop_2(bool) := True(bool)
 facetIndex ++
TMP_7251(uint256) := facetIndex_1(uint256)
facetIndex_2(uint256) = facetIndex_1 (c)+ 1
 continueLoop
CONDITION continueLoop_3
 continueLoop = false
continueLoop_4(bool) := False(bool)
 facets_[numFacets].facetAddress = facetAddress_
REF_4708(IDiamondLoupe.Facet) -> facets__3[numFacets_0]
REF_4709(address) -> REF_4708.facetAddress
facets__5(IDiamondLoupe.Facet[]) := phi(['facets__3'])
REF_4709(address) (->facets__5) := facetAddress__1(address)
 facets_[numFacets].functionSelectors = new bytes4[](selectorCount)
REF_4710(IDiamondLoupe.Facet) -> facets__5[numFacets_0]
REF_4711(bytes4[]) -> REF_4710.functionSelectors
TMP_7253(bytes4[])  = new bytes4[](selectorCount_1)
facets__6(IDiamondLoupe.Facet[]) := phi(['facets__5'])
REF_4711(bytes4[]) (->facets__6) := TMP_7253(bytes4[])
 facets_[numFacets].functionSelectors[0] = selector
REF_4712(IDiamondLoupe.Facet) -> facets__6[numFacets_0]
REF_4713(bytes4[]) -> REF_4712.functionSelectors
REF_4714(bytes4) -> REF_4713[0]
facets__7(IDiamondLoupe.Facet[]) := phi(['facets__6'])
REF_4714(bytes4) (->facets__7) := selector_1(bytes4)
 numFacetSelectors[numFacets] = 1
REF_4715(uint16) -> numFacetSelectors_2[numFacets_0]
numFacetSelectors_4(uint16[]) := phi(['numFacetSelectors_2'])
REF_4715(uint16) (->numFacetSelectors_4) := 1(uint256)
 numFacets ++
TMP_7254(uint256) := numFacets_0(uint256)
numFacets_2(uint256) = numFacets_0 (c)+ 1
 selectorIndex ++
facets__4(IDiamondLoupe.Facet[]) := phi(['facets__7', 'facets__1'])
numFacetSelectors_3(uint16[]) := phi(['numFacetSelectors_1', 'numFacetSelectors_4'])
numFacets_1(uint256) := phi(['numFacets_2', 'numFacets_0'])
TMP_7255(uint256) := selectorIndex_1(uint256)
selectorIndex_2(uint256) = selectorIndex_1 (c)+ 1
 facetIndex_scope_0 < numFacets
facetIndex_scope_0_1(uint256) := phi(['facetIndex_scope_0_0', 'facetIndex_scope_0_2'])
TMP_7256(bool) = facetIndex_scope_0_1 < numFacets_0
CONDITION TMP_7256
 numSelectors = numFacetSelectors[facetIndex_scope_0]
REF_4716(uint16) -> numFacetSelectors_1[facetIndex_scope_0_1]
numSelectors_1(uint256) := REF_4716(uint16)
 selectors = facets_[facetIndex_scope_0].functionSelectors
REF_4717(IDiamondLoupe.Facet) -> facets__1[facetIndex_scope_0_1]
REF_4718(bytes4[]) -> REF_4717.functionSelectors
selectors_1(bytes4[]) = ['REF_4718(bytes4[])']
 mstore(uint256,uint256)(selectors,numSelectors)
TMP_7257(None) = SOLIDITY_CALL mstore(uint256,uint256)(selectors_1,numSelectors_1)
 facetIndex_scope_0 ++
TMP_7258(uint256) := facetIndex_scope_0_1(uint256)
facetIndex_scope_0_2(uint256) = facetIndex_scope_0_1 (c)+ 1
 mstore(uint256,uint256)(facets_,numFacets)
TMP_7259(None) = SOLIDITY_CALL mstore(uint256,uint256)(facets__1,numFacets_0)
 facets_
RETURN facets__1
```
#### IERC165.supportsInterface(bytes4) [EXTERNAL]
```slithir

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
