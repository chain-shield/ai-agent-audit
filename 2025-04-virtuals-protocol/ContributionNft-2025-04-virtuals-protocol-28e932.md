### Storage layout (ContributionNft) 

```text
personaNft address
_contributionVirtualId mapping(uint256 => uint256)
_parents mapping(uint256 => uint256)
_children mapping(uint256 => uint256[])
_cores mapping(uint256 => uint8)
modelContributions mapping(uint256 => bool)
modelDatasets mapping(uint256 => uint256)
_admin address
_eloCalculator address

```


#### ContributionNft._increaseBalance(address,uint128) [INTERNAL]
```slithir
 super._increaseBalance(account,amount)
INTERNAL_CALL, ERC721EnumerableUpgradeable._increaseBalance(address,uint128)(account_1,amount_1)
RETURN TMP_7313
```
#### ContributionNft._update(address,uint256,address) [INTERNAL]
```slithir
to_1(address) := phi(['TMP_7163', 'to_1', 'to_1', 'to_1'])
tokenId_1(uint256) := phi(['tokenId_1', 'tokenId_1', 'tokenId_1', 'tokenId_1'])
auth_1(address) := phi(['TMP_7153', 'TMP_7173', 'TMP_7111', 'TMP_7164'])
 super._update(to,tokenId,auth)
TMP_7314(address) = INTERNAL_CALL, ERC721EnumerableUpgradeable._update(address,uint256,address)(to_1,tokenId_1,auth_1)
RETURN TMP_7314
```
#### ContributionNft.constructor() [PUBLIC]
```slithir
 _disableInitializers()
INTERNAL_CALL, Initializable._disableInitializers()()
```
#### ContributionNft.getAdmin() [PUBLIC]
```slithir
_admin_2(address) := phi(['_admin_7', '_admin_5', '_admin_1', '_admin_0'])
 _admin
RETURN _admin_2
```
#### ContributionNft.getAgentDAO(uint256) [PUBLIC]
```slithir
virtualId_1(uint256) := phi(['virtualId_1', 'virtualId_1'])
personaNft_2(address) := phi(['personaNft_3', 'personaNft_0', 'personaNft_1'])
 IGovernor(IAgentNft(personaNft).virtualInfo(virtualId).dao)
TMP_7290 = CONVERT personaNft_2 to IAgentNft
TMP_7291(IAgentNft.VirtualInfo) = HIGH_LEVEL_CALL, dest:TMP_7290(IAgentNft), function:virtualInfo, arguments:['virtualId_1']  
personaNft_3(address) := phi(['personaNft_3', 'personaNft_1', 'personaNft_2'])
REF_2875(address) -> TMP_7291.dao
TMP_7292 = CONVERT REF_2875 to IGovernor
RETURN TMP_7292
```
#### ContributionNft.getChildren(uint256) [PUBLIC]
```slithir
_children_8(mapping(uint256 => uint256[])) := phi(['_children_8', '_children_0', '_children_7'])
 _children[tokenId]
REF_2889(uint256[]) -> _children_8[tokenId_1]
RETURN REF_2889
```
#### ContributionNft.getCore(uint256) [PUBLIC]
```slithir
_cores_2(mapping(uint256 => uint8)) := phi(['_cores_2', '_cores_1', '_cores_0'])
 _cores[tokenId]
REF_2891(uint8) -> _cores_2[tokenId_1]
RETURN REF_2891
```
#### ContributionNft.getDatasetId(uint256) [EXTERNAL]
```slithir
modelDatasets_2(mapping(uint256 => uint256)) := phi(['modelDatasets_0', 'modelDatasets_2', 'modelDatasets_1'])
 modelDatasets[tokenId]
REF_2893(uint256) -> modelDatasets_2[tokenId_1]
RETURN REF_2893
```
#### ContributionNft.getEloCalculator() [EXTERNAL]
```slithir
_eloCalculator_1(address) := phi(['_eloCalculator_0', '_eloCalculator_2'])
 _eloCalculator
RETURN _eloCalculator_1
```
#### ContributionNft.getParentId(uint256) [PUBLIC]
```slithir
_parents_2(mapping(uint256 => uint256)) := phi(['_parents_0', '_parents_2', '_parents_1'])
 _parents[tokenId]
REF_2890(uint256) -> _parents_2[tokenId_1]
RETURN REF_2890
```
#### ContributionNft.initialize(address) [PUBLIC]
```slithir
 __ERC721_init(Contribution,VC)
INTERNAL_CALL, ERC721Upgradeable.__ERC721_init(string,string)(Contribution,VC)
 __ERC721Enumerable_init()
INTERNAL_CALL, ERC721EnumerableUpgradeable.__ERC721Enumerable_init()()
 __ERC721URIStorage_init()
INTERNAL_CALL, ERC721URIStorageUpgradeable.__ERC721URIStorage_init()()
 personaNft = thePersonaAddress
personaNft_1(address) := thePersonaAddress_1(address)
 _admin = _msgSender()
TMP_7288(address) = INTERNAL_CALL, ContextUpgradeable._msgSender()()
_admin_1(address) := TMP_7288(address)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### ContributionNft.isAccepted(uint256) [PUBLIC]
```slithir
_contributionVirtualId_2(mapping(uint256 => uint256)) := phi(['_contributionVirtualId_2', '_contributionVirtualId_1', '_contributionVirtualId_3', '_contributionVirtualId_0'])
 virtualId = _contributionVirtualId[tokenId]
REF_2876(uint256) -> _contributionVirtualId_2[tokenId_1]
virtualId_1(uint256) := REF_2876(uint256)
 personaDAO = getAgentDAO(virtualId)
TMP_7293(IGovernor) = INTERNAL_CALL, ContributionNft.getAgentDAO(uint256)(virtualId_1)
personaDAO_1(IGovernor) := TMP_7293(IGovernor)
 personaDAO.state(tokenId) == IGovernor.ProposalState.Succeeded
TMP_7294(IGovernor.ProposalState) = HIGH_LEVEL_CALL, dest:personaDAO_1(IGovernor), function:state, arguments:['tokenId_1']  
REF_2878(IGovernor.ProposalState) -> ProposalState.Succeeded
TMP_7295(bool) = TMP_7294 == REF_2878
RETURN TMP_7295
```
#### ContributionNft.isModel(uint256) [PUBLIC]
```slithir
modelContributions_2(mapping(uint256 => bool)) := phi(['modelContributions_1', 'modelContributions_2', 'modelContributions_0'])
 modelContributions[tokenId]
REF_2892(bool) -> modelContributions_2[tokenId_1]
RETURN REF_2892
```
#### ContributionNft.mint(address,uint256,uint8,string,uint256,uint256,bool,uint256) [EXTERNAL]
```slithir
_children_1(mapping(uint256 => uint256[])) := phi(['_children_8', '_children_0', '_children_7'])
 personaDAO = getAgentDAO(virtualId)
TMP_7296(IGovernor) = INTERNAL_CALL, ContributionNft.getAgentDAO(uint256)(virtualId_1)
personaDAO_1(IGovernor) := TMP_7296(IGovernor)
 require(bool,string)(msg.sender == personaDAO.proposalProposer(proposalId),Only proposal proposer can mint Contribution NFT)
TMP_7297(address) = HIGH_LEVEL_CALL, dest:personaDAO_1(IGovernor), function:proposalProposer, arguments:['proposalId_1']  
_children_3(mapping(uint256 => uint256[])) := phi(['_children_8', '_children_2', '_children_7'])
TMP_7298(bool) = msg.sender == TMP_7297
TMP_7299(None) = SOLIDITY_CALL require(bool,string)(TMP_7298,Only proposal proposer can mint Contribution NFT)
 require(bool,string)(parentId != proposalId,Cannot be parent of itself)
TMP_7300(bool) = parentId_1 != proposalId_1
TMP_7301(None) = SOLIDITY_CALL require(bool,string)(TMP_7300,Cannot be parent of itself)
 _mint(to,proposalId)
INTERNAL_CALL, ERC721Upgradeable._mint(address,uint256)(to_1,proposalId_1)
 _setTokenURI(proposalId,newTokenURI)
INTERNAL_CALL, ERC721URIStorageUpgradeable._setTokenURI(uint256,string)(proposalId_1,newTokenURI_1)
 _contributionVirtualId[proposalId] = virtualId
REF_2880(uint256) -> _contributionVirtualId_2[proposalId_1]
_contributionVirtualId_3(mapping(uint256 => uint256)) := phi(['_contributionVirtualId_2'])
REF_2880(uint256) (->_contributionVirtualId_3) := virtualId_1(uint256)
 _parents[proposalId] = parentId
REF_2881(uint256) -> _parents_0[proposalId_1]
_parents_1(mapping(uint256 => uint256)) := phi(['_parents_0'])
REF_2881(uint256) (->_parents_1) := parentId_1(uint256)
 _children[parentId].push(proposalId)
REF_2882(uint256[]) -> _children_5[parentId_1]
REF_2884 -> LENGTH REF_2882
TMP_7305(uint256) := REF_2884(uint256)
TMP_7306(uint256) = TMP_7305 (c)+ 1
_children_6(mapping(uint256 => uint256[])) := phi(['_children_5'])
REF_2884(uint256) (->_children_7) := TMP_7306(uint256)
REF_2885(uint256) -> REF_2882[TMP_7305]
_children_7(mapping(uint256 => uint256[])) := phi(['_children_6'])
REF_2885(uint256) (->_children_7) := proposalId_1(uint256)
 _cores[proposalId] = coreId
REF_2886(uint8) -> _cores_0[proposalId_1]
_cores_1(mapping(uint256 => uint8)) := phi(['_cores_0'])
REF_2886(uint8) (->_cores_1) := coreId_1(uint8)
 isModel_
CONDITION isModel__1
 modelContributions[proposalId] = true
REF_2887(bool) -> modelContributions_0[proposalId_1]
modelContributions_1(mapping(uint256 => bool)) := phi(['modelContributions_0'])
REF_2887(bool) (->modelContributions_1) := True(bool)
 modelDatasets[proposalId] = datasetId
REF_2888(uint256) -> modelDatasets_0[proposalId_1]
modelDatasets_1(mapping(uint256 => uint256)) := phi(['modelDatasets_0'])
REF_2888(uint256) (->modelDatasets_1) := datasetId_1(uint256)
 NewContribution(proposalId,virtualId,parentId,datasetId)
Emit NewContribution(proposalId_1,virtualId_1,parentId_1,datasetId_1)
 proposalId
RETURN proposalId_1
```
#### ContributionNft.ownerOf(uint256) [PUBLIC]
```slithir
 _ownerOf(tokenId)
TMP_7315(address) = INTERNAL_CALL, ERC721Upgradeable._ownerOf(uint256)(tokenId_1)
RETURN TMP_7315
```
#### ContributionNft.setAdmin(address) [PUBLIC]
```slithir
_admin_3(address) := phi(['_admin_7', '_admin_5', '_admin_1', '_admin_0'])
 require(bool,string)(_msgSender() == _admin,Only admin can set admin)
TMP_7308(address) = INTERNAL_CALL, ContextUpgradeable._msgSender()()
TMP_7309(bool) = TMP_7308 == _admin_4
TMP_7310(None) = SOLIDITY_CALL require(bool,string)(TMP_7309,Only admin can set admin)
 _admin = newAdmin
_admin_5(address) := newAdmin_1(address)
```

#### ContributionNft.supportsInterface(bytes4) [PUBLIC]
```slithir
 super.supportsInterface(interfaceId)
TMP_7312(bool) = INTERNAL_CALL, ERC721URIStorageUpgradeable.supportsInterface(bytes4)(interfaceId_1)
RETURN TMP_7312
```
#### ContributionNft.tokenURI(uint256) [PUBLIC]
```slithir
 super.tokenURI(tokenId)
TMP_7311(string) = INTERNAL_CALL, ERC721URIStorageUpgradeable.tokenURI(uint256)(tokenId_1)
RETURN TMP_7311
```
#### ContributionNft.tokenVirtualId(uint256) [PUBLIC]
```slithir
_contributionVirtualId_1(mapping(uint256 => uint256)) := phi(['_contributionVirtualId_2', '_contributionVirtualId_1', '_contributionVirtualId_3', '_contributionVirtualId_0'])
 _contributionVirtualId[tokenId]
REF_2873(uint256) -> _contributionVirtualId_1[tokenId_1]
RETURN REF_2873
```
#### IAgentNft.virtualInfo(uint256) [EXTERNAL]
```slithir

```
#### IGovernor.state(uint256) [EXTERNAL]
```slithir

```
#### IGovernor.proposalProposer(uint256) [EXTERNAL]
```slithir

```
