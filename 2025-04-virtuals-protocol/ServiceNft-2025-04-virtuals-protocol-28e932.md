


### Storage layout (ServiceNft) 

```text
_nextTokenId uint256
personaNft address
contributionNft address
datasetImpactWeight uint16
_cores mapping(uint256 => uint8)
_maturities mapping(uint256 => uint256)
_impacts mapping(uint256 => uint256)
_coreServices mapping(uint256 => mapping(uint8 => uint256))
_coreDatasets mapping(uint256 => mapping(uint8 => uint256[]))

```

#### ServiceNft._increaseBalance(address,uint128) [INTERNAL]
```slithir
 super._increaseBalance(account,amount)
INTERNAL_CALL, ERC721EnumerableUpgradeable._increaseBalance(address,uint128)(account_1,amount_1)
RETURN TMP_7642
```
#### ERC721EnumerableUpgradeable._update(address,uint256,address) [INTERNAL]
```slithir
to_1(address) := phi(['to_1'])
tokenId_1(uint256) := phi(['tokenId_1'])
auth_1(address) := phi(['auth_1'])
 previousOwner = super._update(to,tokenId,auth)
TMP_7561(address) = INTERNAL_CALL, ERC721Upgradeable._update(address,uint256,address)(to_1,tokenId_1,auth_1)
previousOwner_1(address) := TMP_7561(address)
 previousOwner == address(0)
TMP_7562 = CONVERT 0 to address
TMP_7563(bool) = previousOwner_1 == TMP_7562
CONDITION TMP_7563
 _addTokenToAllTokensEnumeration(tokenId)
INTERNAL_CALL, ERC721EnumerableUpgradeable._addTokenToAllTokensEnumeration(uint256)(tokenId_1)
 previousOwner != to
TMP_7565(bool) = previousOwner_1 != to_1
CONDITION TMP_7565
 _removeTokenFromOwnerEnumeration(previousOwner,tokenId)
INTERNAL_CALL, ERC721EnumerableUpgradeable._removeTokenFromOwnerEnumeration(address,uint256)(previousOwner_1,tokenId_1)
 to == address(0)
TMP_7567 = CONVERT 0 to address
TMP_7568(bool) = to_1 == TMP_7567
CONDITION TMP_7568
 _removeTokenFromAllTokensEnumeration(tokenId)
INTERNAL_CALL, ERC721EnumerableUpgradeable._removeTokenFromAllTokensEnumeration(uint256)(tokenId_1)
 previousOwner != to
TMP_7570(bool) = previousOwner_1 != to_1
CONDITION TMP_7570
 _addTokenToOwnerEnumeration(to,tokenId)
INTERNAL_CALL, ERC721EnumerableUpgradeable._addTokenToOwnerEnumeration(address,uint256)(to_1,tokenId_1)
 previousOwner
RETURN previousOwner_1
```
#### ServiceNft.constructor() [PUBLIC]
```slithir
 _disableInitializers()
INTERNAL_CALL, Initializable._disableInitializers()()
```
#### ServiceNft.getCore(uint256) [PUBLIC]
```slithir
_cores_6(mapping(uint256 => uint8)) := phi(['_cores_3', '_cores_7', '_cores_5', '_cores_0'])
 _requireOwned(tokenId)
TMP_7634(address) = INTERNAL_CALL, ERC721Upgradeable._requireOwned(uint256)(tokenId_1)
 _cores[tokenId]
REF_3056(uint8) -> _cores_7[tokenId_1]
RETURN REF_3056
```
#### ServiceNft.getCoreDatasetAt(uint256,uint8,uint256) [PUBLIC]
```slithir
_coreDatasets_11(mapping(uint256 => mapping(uint8 => uint256[]))) := phi(['_coreDatasets_8', '_coreDatasets_0', '_coreDatasets_12', '_coreDatasets_10', '_coreDatasets_11', '_coreDatasets_13'])
 _coreDatasets[virtualId][coreType][index]
REF_3061(mapping(uint8 => uint256[])) -> _coreDatasets_11[virtualId_1]
REF_3062(uint256[]) -> REF_3061[coreType_1]
REF_3063(uint256) -> REF_3062[index_1]
RETURN REF_3063
```
#### ServiceNft.getCoreDatasets(uint256,uint8) [PUBLIC]
```slithir
_coreDatasets_13(mapping(uint256 => mapping(uint8 => uint256[]))) := phi(['_coreDatasets_8', '_coreDatasets_0', '_coreDatasets_12', '_coreDatasets_10', '_coreDatasets_11', '_coreDatasets_13'])
 _coreDatasets[virtualId][coreType]
REF_3067(mapping(uint8 => uint256[])) -> _coreDatasets_13[virtualId_1]
REF_3068(uint256[]) -> REF_3067[coreType_1]
RETURN REF_3068
```
#### ServiceNft.getCoreService(uint256,uint8) [PUBLIC]
```slithir
_coreServices_3(mapping(uint256 => mapping(uint8 => uint256))) := phi(['_coreServices_0', '_coreServices_3', '_coreServices_2', '_coreServices_1'])
 _coreServices[virtualId][coreType]
REF_3059(mapping(uint8 => uint256)) -> _coreServices_3[virtualId_1]
REF_3060(uint256) -> REF_3059[coreType_1]
RETURN REF_3060
```
#### ServiceNft.getImpact(uint256) [PUBLIC]
```slithir
_impacts_14(mapping(uint256 => uint256)) := phi(['_impacts_13', '_impacts_15', '_impacts_8', '_impacts_0'])
 _requireOwned(tokenId)
TMP_7636(address) = INTERNAL_CALL, ERC721Upgradeable._requireOwned(uint256)(tokenId_1)
 _impacts[tokenId]
REF_3058(uint256) -> _impacts_15[tokenId_1]
RETURN REF_3058
```
#### ServiceNft.getMaturity(uint256) [PUBLIC]
```slithir
_maturities_7(mapping(uint256 => uint256)) := phi(['_maturities_6', '_maturities_0', '_maturities_8', '_maturities_2'])
 _requireOwned(tokenId)
TMP_7635(address) = INTERNAL_CALL, ERC721Upgradeable._requireOwned(uint256)(tokenId_1)
 _maturities[tokenId]
REF_3057(uint256) -> _maturities_8[tokenId_1]
RETURN REF_3057
```
#### ServiceNft.initialize(address,address,uint16) [PUBLIC]
```slithir
 __ERC721_init(Service,VS)
INTERNAL_CALL, ERC721Upgradeable.__ERC721_init(string,string)(Service,VS)
 __ERC721Enumerable_init()
INTERNAL_CALL, ERC721EnumerableUpgradeable.__ERC721Enumerable_init()()
 __ERC721URIStorage_init()
INTERNAL_CALL, ERC721URIStorageUpgradeable.__ERC721URIStorage_init()()
 __Ownable_init(_msgSender())
TMP_7593(address) = INTERNAL_CALL, ContextUpgradeable._msgSender()()
INTERNAL_CALL, OwnableUpgradeable.__Ownable_init(address)(TMP_7593)
 personaNft = initialAgentNft
personaNft_1(address) := initialAgentNft_1(address)
 contributionNft = initialContributionNft
contributionNft_1(address) := initialContributionNft_1(address)
 datasetImpactWeight = initialDatasetImpactWeight
datasetImpactWeight_1(uint16) := initialDatasetImpactWeight_1(uint16)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### ServiceNft.mint(uint256,bytes32) [PUBLIC]
```slithir
personaNft_2(address) := phi(['personaNft_1', 'personaNft_3', 'personaNft_0'])
contributionNft_2(address) := phi(['contributionNft_11', 'contributionNft_13', 'contributionNft_9', 'contributionNft_0', 'contributionNft_1'])
_impacts_1(mapping(uint256 => uint256)) := phi(['_impacts_13', '_impacts_15', '_impacts_8', '_impacts_0'])
_coreDatasets_1(mapping(uint256 => mapping(uint8 => uint256[]))) := phi(['_coreDatasets_8', '_coreDatasets_0', '_coreDatasets_12', '_coreDatasets_10', '_coreDatasets_11', '_coreDatasets_13'])
 info = IAgentNft(personaNft).virtualInfo(virtualId)
TMP_7596 = CONVERT personaNft_2 to IAgentNft
TMP_7597(IAgentNft.VirtualInfo) = HIGH_LEVEL_CALL, dest:TMP_7596(IAgentNft), function:virtualInfo, arguments:['virtualId_1']  
personaNft_3(address) := phi(['personaNft_1', 'personaNft_3', 'personaNft_2'])
contributionNft_3(address) := phi(['contributionNft_11', 'contributionNft_13', 'contributionNft_9', 'contributionNft_1', 'contributionNft_2'])
_impacts_2(mapping(uint256 => uint256)) := phi(['_impacts_13', '_impacts_15', '_impacts_8', '_impacts_1'])
_coreDatasets_2(mapping(uint256 => mapping(uint8 => uint256[]))) := phi(['_coreDatasets_8', '_coreDatasets_12', '_coreDatasets_10', '_coreDatasets_11', '_coreDatasets_1', '_coreDatasets_13'])
info_1(IAgentNft.VirtualInfo) := TMP_7597(IAgentNft.VirtualInfo)
 require(bool,string)(_msgSender() == info.dao,Caller is not VIRTUAL DAO)
TMP_7598(address) = INTERNAL_CALL, ContextUpgradeable._msgSender()()
REF_3011(address) -> info_1.dao
TMP_7599(bool) = TMP_7598 == REF_3011
TMP_7600(None) = SOLIDITY_CALL require(bool,string)(TMP_7599,Caller is not VIRTUAL DAO)
 personaDAO = IGovernor(info.dao)
REF_3012(address) -> info_1.dao
TMP_7601 = CONVERT REF_3012 to IGovernor
personaDAO_1(IGovernor) := TMP_7601(IGovernor)
 mintCalldata = abi.encodeWithSignature(mint(uint256,bytes32),virtualId,descHash)
TMP_7602(bytes) = SOLIDITY_CALL abi.encodeWithSignature()(mint(uint256,bytes32),virtualId_1,descHash_1)
mintCalldata_1(bytes) := TMP_7602(bytes)
 targets = new address[](1)
TMP_7604(address[])  = new address[](1)
targets_1(address[]) = ['TMP_7604(address[])']
 targets[0] = address(this)
REF_3014(address) -> targets_1[0]
TMP_7605 = CONVERT this to address
targets_2(address[]) := phi(['targets_1'])
REF_3014(address) (->targets_2) := TMP_7605(address)
 values = new uint256[](1)
TMP_7607(uint256[])  = new uint256[](1)
values_1(uint256[]) = ['TMP_7607(uint256[])']
 values[0] = 0
REF_3015(uint256) -> values_1[0]
values_2(uint256[]) := phi(['values_1'])
REF_3015(uint256) (->values_2) := 0(uint256)
 calldatas = new bytes[](1)
TMP_7609(bytes[])  = new bytes[](1)
calldatas_1(bytes[]) = ['TMP_7609(bytes[])']
 calldatas[0] = mintCalldata
REF_3016(bytes) -> calldatas_1[0]
calldatas_2(bytes[]) := phi(['calldatas_1'])
REF_3016(bytes) (->calldatas_2) := mintCalldata_1(bytes)
 proposalId = personaDAO.hashProposal(targets,values,calldatas,descHash)
TMP_7610(uint256) = HIGH_LEVEL_CALL, dest:personaDAO_1(IGovernor), function:hashProposal, arguments:['targets_2', 'values_2', 'calldatas_2', 'descHash_1']  
contributionNft_5(address) := phi(['contributionNft_11', 'contributionNft_13', 'contributionNft_9', 'contributionNft_1', 'contributionNft_4'])
_impacts_4(mapping(uint256 => uint256)) := phi(['_impacts_13', '_impacts_15', '_impacts_8', '_impacts_3'])
_coreDatasets_4(mapping(uint256 => mapping(uint8 => uint256[]))) := phi(['_coreDatasets_8', '_coreDatasets_12', '_coreDatasets_10', '_coreDatasets_11', '_coreDatasets_3', '_coreDatasets_13'])
proposalId_1(uint256) := TMP_7610(uint256)
 _mint(info.tba,proposalId)
REF_3018(address) -> info_1.tba
INTERNAL_CALL, ERC721Upgradeable._mint(address,uint256)(REF_3018,proposalId_1)
 _cores[proposalId] = IContributionNft(contributionNft).getCore(proposalId)
REF_3019(uint8) -> _cores_0[proposalId_1]
TMP_7612 = CONVERT contributionNft_6 to IContributionNft
TMP_7613(uint8) = HIGH_LEVEL_CALL, dest:TMP_7612(IContributionNft), function:getCore, arguments:['proposalId_1']  
contributionNft_7(address) := phi(['contributionNft_11', 'contributionNft_13', 'contributionNft_9', 'contributionNft_1', 'contributionNft_6'])
_impacts_6(mapping(uint256 => uint256)) := phi(['_impacts_13', '_impacts_15', '_impacts_8', '_impacts_5'])
_coreDatasets_6(mapping(uint256 => mapping(uint8 => uint256[]))) := phi(['_coreDatasets_8', '_coreDatasets_12', '_coreDatasets_5', '_coreDatasets_10', '_coreDatasets_11', '_coreDatasets_13'])
_cores_1(mapping(uint256 => uint8)) := phi(['_cores_0'])
REF_3019(uint8) (->_cores_1) := TMP_7613(uint8)
 _maturities[proposalId] = IAgentDAO(info.dao).getMaturity(proposalId)
REF_3021(uint256) -> _maturities_0[proposalId_1]
REF_3022(address) -> info_1.dao
TMP_7614 = CONVERT REF_3022 to IAgentDAO
TMP_7615(uint256) = HIGH_LEVEL_CALL, dest:TMP_7614(IAgentDAO), function:getMaturity, arguments:['proposalId_1']  
contributionNft_8(address) := phi(['contributionNft_11', 'contributionNft_7', 'contributionNft_13', 'contributionNft_9', 'contributionNft_1'])
_cores_2(mapping(uint256 => uint8)) := phi(['_cores_3', '_cores_1', '_cores_7', '_cores_5'])
_impacts_7(mapping(uint256 => uint256)) := phi(['_impacts_13', '_impacts_15', '_impacts_8', '_impacts_6'])
_coreDatasets_7(mapping(uint256 => mapping(uint8 => uint256[]))) := phi(['_coreDatasets_8', '_coreDatasets_12', '_coreDatasets_10', '_coreDatasets_11', '_coreDatasets_6', '_coreDatasets_13'])
_maturities_1(mapping(uint256 => uint256)) := phi(['_maturities_0'])
REF_3021(uint256) (->_maturities_1) := TMP_7615(uint256)
 isModel = IContributionNft(contributionNft).isModel(proposalId)
TMP_7616 = CONVERT contributionNft_8 to IContributionNft
TMP_7617(bool) = HIGH_LEVEL_CALL, dest:TMP_7616(IContributionNft), function:isModel, arguments:['proposalId_1']  
contributionNft_9(address) := phi(['contributionNft_11', 'contributionNft_13', 'contributionNft_9', 'contributionNft_1', 'contributionNft_8'])
_cores_3(mapping(uint256 => uint8)) := phi(['_cores_3', '_cores_2', '_cores_7', '_cores_5'])
_maturities_2(mapping(uint256 => uint256)) := phi(['_maturities_6', '_maturities_8', '_maturities_1', '_maturities_2'])
_impacts_8(mapping(uint256 => uint256)) := phi(['_impacts_13', '_impacts_15', '_impacts_8', '_impacts_7'])
_coreDatasets_8(mapping(uint256 => mapping(uint8 => uint256[]))) := phi(['_coreDatasets_8', '_coreDatasets_12', '_coreDatasets_10', '_coreDatasets_7', '_coreDatasets_11', '_coreDatasets_13'])
isModel_1(bool) := TMP_7617(bool)
 isModel
CONDITION isModel_1
 CoreServiceUpdated(virtualId,_cores[proposalId],proposalId)
REF_3025(uint8) -> _cores_3[proposalId_1]
Emit CoreServiceUpdated(virtualId_1,REF_3025,proposalId_1)
 updateImpact(virtualId,proposalId)
INTERNAL_CALL, ServiceNft.updateImpact(uint256,uint256)(virtualId_1,proposalId_1)
_cores_4(mapping(uint256 => uint8)) := phi(['_cores_5'])
_maturities_3(mapping(uint256 => uint256)) := phi(['_maturities_6'])
_impacts_9(mapping(uint256 => uint256)) := phi(['_impacts_13'])
 _coreServices[virtualId][_cores[proposalId]] = proposalId
REF_3026(mapping(uint8 => uint256)) -> _coreServices_0[virtualId_1]
REF_3027(uint8) -> _cores_4[proposalId_1]
REF_3028(uint256) -> REF_3026[REF_3027]
_coreServices_1(mapping(uint256 => mapping(uint8 => uint256))) := phi(['_coreServices_0'])
REF_3028(uint256) (->_coreServices_1) := proposalId_1(uint256)
 _coreDatasets[virtualId][_cores[proposalId]].push(proposalId)
REF_3029(mapping(uint8 => uint256[])) -> _coreDatasets_8[virtualId_1]
REF_3030(uint8) -> _cores_3[proposalId_1]
REF_3031(uint256[]) -> REF_3029[REF_3030]
REF_3033 -> LENGTH REF_3031
TMP_7621(uint256) := REF_3033(uint256)
TMP_7622(uint256) = TMP_7621 (c)+ 1
_coreDatasets_9(mapping(uint256 => mapping(uint8 => uint256[]))) := phi(['_coreDatasets_8'])
REF_3033(uint256) (->_coreDatasets_10) := TMP_7622(uint256)
REF_3034(uint256) -> REF_3031[TMP_7621]
_coreDatasets_10(mapping(uint256 => mapping(uint8 => uint256[]))) := phi(['_coreDatasets_9'])
REF_3034(uint256) (->_coreDatasets_10) := proposalId_1(uint256)
 NewService(proposalId,_cores[proposalId],_maturities[proposalId],_impacts[proposalId],isModel)
REF_3035(uint8) -> _cores_3[proposalId_1]
REF_3036(uint256) -> _maturities_2[proposalId_1]
REF_3037(uint256) -> _impacts_8[proposalId_1]
Emit NewService(proposalId_1,REF_3035,REF_3036,REF_3037,isModel_1)
 proposalId
RETURN proposalId_1
```
#### ServiceNft.setDatasetImpactWeight(uint16) [PUBLIC][OWNER]
```slithir
 datasetImpactWeight = weight
datasetImpactWeight_4(uint16) := weight_1(uint16)
 DatasetImpactUpdated(weight)
Emit DatasetImpactUpdated(weight_1)
 onlyOwner()
MODIFIER_CALL, OwnableUpgradeable.onlyOwner()()
```
#### ServiceNft.supportsInterface(bytes4) [PUBLIC]
```slithir
 super.supportsInterface(interfaceId)
TMP_7641(bool) = INTERNAL_CALL, ERC721URIStorageUpgradeable.supportsInterface(bytes4)(interfaceId_1)
RETURN TMP_7641
```
#### ServiceNft.tokenURI(uint256) [PUBLIC]
```slithir
contributionNft_12(address) := phi(['contributionNft_11', 'contributionNft_13', 'contributionNft_9', 'contributionNft_0', 'contributionNft_1'])
 IContributionNft(contributionNft).tokenURI(tokenId)
TMP_7639 = CONVERT contributionNft_12 to IContributionNft
TMP_7640(string) = HIGH_LEVEL_CALL, dest:TMP_7639(IContributionNft), function:tokenURI, arguments:['tokenId_1']  
contributionNft_13(address) := phi(['contributionNft_11', 'contributionNft_13', 'contributionNft_9', 'contributionNft_1', 'contributionNft_12'])
RETURN TMP_7640
```
#### ServiceNft.totalCoreDatasets(uint256,uint8) [PUBLIC]
```slithir
_coreDatasets_12(mapping(uint256 => mapping(uint8 => uint256[]))) := phi(['_coreDatasets_8', '_coreDatasets_0', '_coreDatasets_12', '_coreDatasets_10', '_coreDatasets_11', '_coreDatasets_13'])
 _coreDatasets[virtualId][coreType].length
REF_3064(mapping(uint8 => uint256[])) -> _coreDatasets_12[virtualId_1]
REF_3065(uint256[]) -> REF_3064[coreType_1]
REF_3066 -> LENGTH REF_3065
RETURN REF_3066
```
#### ServiceNft.updateImpact(uint256,uint256) [PUBLIC]
```slithir
virtualId_1(uint256) := phi(['virtualId_1'])
proposalId_1(uint256) := phi(['proposalId_1'])
contributionNft_10(address) := phi(['contributionNft_11', 'contributionNft_13', 'contributionNft_9', 'contributionNft_0', 'contributionNft_1'])
datasetImpactWeight_2(uint16) := phi(['datasetImpactWeight_1', 'datasetImpactWeight_4', 'datasetImpactWeight_0', 'datasetImpactWeight_3'])
_cores_5(mapping(uint256 => uint8)) := phi(['_cores_3', '_cores_7', '_cores_5', '_cores_0'])
_maturities_4(mapping(uint256 => uint256)) := phi(['_maturities_6', '_maturities_0', '_maturities_8', '_maturities_2'])
_coreServices_2(mapping(uint256 => mapping(uint8 => uint256))) := phi(['_coreServices_0', '_coreServices_3', '_coreServices_2', '_coreServices_1'])
 prevServiceId = _coreServices[virtualId][_cores[proposalId]]
REF_3038(mapping(uint8 => uint256)) -> _coreServices_2[virtualId_1]
REF_3039(uint8) -> _cores_5[proposalId_1]
REF_3040(uint256) -> REF_3038[REF_3039]
prevServiceId_1(uint256) := REF_3040(uint256)
 datasetId = IContributionNft(contributionNft).getDatasetId(proposalId)
TMP_7624 = CONVERT contributionNft_10 to IContributionNft
TMP_7625(uint256) = HIGH_LEVEL_CALL, dest:TMP_7624(IContributionNft), function:getDatasetId, arguments:['proposalId_1']  
contributionNft_11(address) := phi(['contributionNft_11', 'contributionNft_10', 'contributionNft_13', 'contributionNft_9', 'contributionNft_1'])
datasetImpactWeight_3(uint16) := phi(['datasetImpactWeight_1', 'datasetImpactWeight_2', 'datasetImpactWeight_4', 'datasetImpactWeight_3'])
_maturities_5(mapping(uint256 => uint256)) := phi(['_maturities_6', '_maturities_4', '_maturities_8', '_maturities_2'])
datasetId_1(uint256) := TMP_7625(uint256)
 _impacts[proposalId] = rawImpact
REF_3042(uint256) -> _impacts_9[proposalId_1]
_impacts_10(mapping(uint256 => uint256)) := phi(['_impacts_9'])
REF_3042(uint256) (->_impacts_10) := rawImpact_3(uint256)
 datasetId > 0
TMP_7626(bool) = datasetId_1 > 0
CONDITION TMP_7626
 _impacts[datasetId] = (rawImpact * datasetImpactWeight) / 10000
REF_3043(uint256) -> _impacts_10[datasetId_1]
TMP_7627(uint256) = rawImpact_3 (c)* datasetImpactWeight_3
TMP_7628(uint256) = TMP_7627 (c)/ 10000
_impacts_11(mapping(uint256 => uint256)) := phi(['_impacts_10'])
REF_3043(uint256) (->_impacts_11) := TMP_7628(uint256)
 _impacts[proposalId] = rawImpact - _impacts[datasetId]
REF_3044(uint256) -> _impacts_11[proposalId_1]
REF_3045(uint256) -> _impacts_11[datasetId_1]
TMP_7629(uint256) = rawImpact_3 (c)- REF_3045
_impacts_12(mapping(uint256 => uint256)) := phi(['_impacts_11'])
REF_3044(uint256) (->_impacts_12) := TMP_7629(uint256)
 SetServiceScore(datasetId,_maturities[proposalId],_impacts[datasetId])
REF_3046(uint256) -> _maturities_5[proposalId_1]
REF_3047(uint256) -> _impacts_12[datasetId_1]
Emit SetServiceScore(datasetId_1,REF_3046,REF_3047)
 _maturities[datasetId] = _maturities[proposalId]
REF_3048(uint256) -> _maturities_5[datasetId_1]
REF_3049(uint256) -> _maturities_5[proposalId_1]
_maturities_6(mapping(uint256 => uint256)) := phi(['_maturities_5'])
REF_3048(uint256) (->_maturities_6) := REF_3049(uint256)
_impacts_13(mapping(uint256 => uint256)) := phi(['_impacts_10', '_impacts_12'])
 SetServiceScore(proposalId,_maturities[proposalId],_impacts[proposalId])
REF_3050(uint256) -> _maturities_6[proposalId_1]
REF_3051(uint256) -> _impacts_13[proposalId_1]
Emit SetServiceScore(proposalId_1,REF_3050,REF_3051)
 (_maturities[proposalId] > _maturities[prevServiceId])
REF_3052(uint256) -> _maturities_4[proposalId_1]
REF_3053(uint256) -> _maturities_4[prevServiceId_1]
TMP_7632(bool) = REF_3052 > REF_3053
CONDITION TMP_7632
 rawImpact = _maturities[proposalId] - _maturities[prevServiceId]
REF_3054(uint256) -> _maturities_4[proposalId_1]
REF_3055(uint256) -> _maturities_4[prevServiceId_1]
TMP_7633(uint256) = REF_3054 (c)- REF_3055
rawImpact_2(uint256) := TMP_7633(uint256)
 rawImpact = 0
rawImpact_1(uint256) := 0(uint256)
rawImpact_3(uint256) := phi(['rawImpact_1', 'rawImpact_2'])
```
#### IGovernor.hashProposal(address[],uint256[],bytes[],bytes32) [EXTERNAL]
```slithir

```
#### IContributionNft.getCore(uint256) [EXTERNAL]
```slithir

```
#### IContributionNft.isModel(uint256) [EXTERNAL]
```slithir

```

#### IAgentNft.virtualInfo(uint256) [EXTERNAL]
```slithir

```
#### IContributionNft.tokenURI(uint256) [EXTERNAL]
```slithir

```
#### IContributionNft.getDatasetId(uint256) [EXTERNAL]
```slithir

```
