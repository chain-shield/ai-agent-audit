


### Storage layout (AgentDAO) 

```text
_scores mapping(address => Checkpoints.Trace208)
_proposalMaturities mapping(uint256 => uint256)
_totalScore uint256
_agentNft address

```





#### AgentDAO._calcMaturity(uint256,uint8[]) [INTERNAL]
```slithir
proposalId_1(uint256) := phi(['proposalId_1'])
votes_1(uint8[]) := phi(['votes_1'])
_agentNft_11(address) := phi(['_agentNft_1', '_agentNft_10', '_agentNft_16', '_agentNft_8', '_agentNft_0'])
 contributionNft = IAgentNft(_agentNft).getContributionNft()
TMP_12060 = CONVERT _agentNft_11 to IAgentNft
TMP_12061(address) = HIGH_LEVEL_CALL, dest:TMP_12060(IAgentNft), function:getContributionNft, arguments:[]  
_agentNft_12(address) := phi(['_agentNft_1', '_agentNft_10', '_agentNft_8', '_agentNft_16', '_agentNft_11'])
contributionNft_1(address) := TMP_12061(address)
 serviceNft = IAgentNft(_agentNft).getServiceNft()
TMP_12062 = CONVERT _agentNft_12 to IAgentNft
TMP_12063(address) = HIGH_LEVEL_CALL, dest:TMP_12062(IAgentNft), function:getServiceNft, arguments:[]  
_agentNft_13(address) := phi(['_agentNft_1', '_agentNft_12', '_agentNft_10', '_agentNft_8', '_agentNft_16'])
serviceNft_1(address) := TMP_12063(address)
 virtualId = IContributionNft(contributionNft).tokenVirtualId(proposalId)
TMP_12064 = CONVERT contributionNft_1 to IContributionNft
TMP_12065(uint256) = HIGH_LEVEL_CALL, dest:TMP_12064(IContributionNft), function:tokenVirtualId, arguments:['proposalId_1']  
_agentNft_14(address) := phi(['_agentNft_13', '_agentNft_1', '_agentNft_10', '_agentNft_8', '_agentNft_16'])
virtualId_1(uint256) := TMP_12065(uint256)
 core = IContributionNft(contributionNft).getCore(proposalId)
TMP_12066 = CONVERT contributionNft_1 to IContributionNft
TMP_12067(uint8) = HIGH_LEVEL_CALL, dest:TMP_12066(IContributionNft), function:getCore, arguments:['proposalId_1']  
_agentNft_15(address) := phi(['_agentNft_1', '_agentNft_10', '_agentNft_8', '_agentNft_16', '_agentNft_14'])
core_1(uint8) := TMP_12067(uint8)
 coreService = IServiceNft(serviceNft).getCoreService(virtualId,core)
TMP_12068 = CONVERT serviceNft_1 to IServiceNft
TMP_12069(uint256) = HIGH_LEVEL_CALL, dest:TMP_12068(IServiceNft), function:getCoreService, arguments:['virtualId_1', 'core_1']  
_agentNft_16(address) := phi(['_agentNft_1', '_agentNft_10', '_agentNft_8', '_agentNft_16', '_agentNft_15'])
coreService_1(uint256) := TMP_12069(uint256)
 maturity = 100
maturity_1(uint256) := 100(uint256)
 coreService > 0
TMP_12070(bool) = coreService_1 > 0
CONDITION TMP_12070
 maturity = IServiceNft(serviceNft).getMaturity(coreService)
TMP_12071 = CONVERT serviceNft_1 to IServiceNft
TMP_12072(uint256) = HIGH_LEVEL_CALL, dest:TMP_12071(IServiceNft), function:getMaturity, arguments:['coreService_1']  
_agentNft_17(address) := phi(['_agentNft_10', '_agentNft_8', '_agentNft_16', '_agentNft_1'])
maturity_2(uint256) := TMP_12072(uint256)
 maturity = IEloCalculator(IAgentNft(_agentNft).getEloCalculator()).battleElo(maturity,votes)
TMP_12073 = CONVERT _agentNft_17 to IAgentNft
TMP_12074(address) = HIGH_LEVEL_CALL, dest:TMP_12073(IAgentNft), function:getEloCalculator, arguments:[]  
_agentNft_18(address) := phi(['_agentNft_1', '_agentNft_17', '_agentNft_10', '_agentNft_8', '_agentNft_16'])
TMP_12075 = CONVERT TMP_12074 to IEloCalculator
TMP_12076(uint256) = HIGH_LEVEL_CALL, dest:TMP_12075(IEloCalculator), function:battleElo, arguments:['maturity_2', 'votes_1']  
_agentNft_19(address) := phi(['_agentNft_18', '_agentNft_1', '_agentNft_10', '_agentNft_8', '_agentNft_16'])
maturity_3(uint256) := TMP_12076(uint256)
maturity_4(uint256) := phi(['maturity_3', 'maturity_1'])
 maturity
RETURN maturity_4
```
#### AgentDAO._castVote(uint256,address,uint8,string,bytes) [INTERNAL]
```slithir
proposalId_1(uint256) := phi(['proposalId_1', 'proposalId_1', 'proposalId_1'])
account_1(address) := phi(['voter_1', 'account_1', 'voter_1'])
support_1(uint8) := phi(['support_1', 'support_1', 'support_1'])
reason_1(string) := phi(['reason_1', 'reason_1', 'reason_1'])
params_1(bytes) := phi(['params_1', 'params_1', 'TMP_11799'])
_scores_4(mapping(address => Checkpoints.Trace208)) := phi(['_scores_0', '_scores_7', '_scores_1', '_scores_3'])
_totalScore_1(uint256) := phi(['_totalScore_0', '_totalScore_4'])
 votedPreviously = hasVoted(proposalId,account)
TMP_12027(bool) = INTERNAL_CALL, GovernorCountingSimpleUpgradeable.hasVoted(uint256,address)(proposalId_1,account_1)
votedPreviously_1(bool) := TMP_12027(bool)
 weight = super._castVote(proposalId,account,support,reason,params)
TMP_12028(uint256) = INTERNAL_CALL, GovernorUpgradeable._castVote(uint256,address,uint8,string,bytes)(proposalId_1,account_1,support_1,reason_1,params_1)
weight_1(uint256) := TMP_12028(uint256)
 ! votedPreviously && hasVoted(proposalId,account)
TMP_12029 = UnaryType.BANG votedPreviously_1 
TMP_12030(bool) = INTERNAL_CALL, GovernorCountingSimpleUpgradeable.hasVoted(uint256,address)(proposalId_1,account_1)
TMP_12031(bool) = TMP_12029 && TMP_12030
CONDITION TMP_12031
 ++ _totalScore
_totalScore_5(uint256) = _totalScore_4 (c)+ 1
 _scores[account].push(SafeCast.toUint48(block.number),SafeCast.toUint208(scoreOf(account)) + 1)
REF_4895(Checkpoints.Trace208) -> _scores_7[account_1]
TMP_12032(uint48) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint48(uint256), arguments:['block.number'] 
TMP_12033(uint256) = INTERNAL_CALL, AgentDAO.scoreOf(address)(account_1)
_scores_8(mapping(address => Checkpoints.Trace208)) := phi(['_scores_1'])
TMP_12034(uint208) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint208(uint256), arguments:['TMP_12033'] 
TMP_12035(uint208) = TMP_12034 (c)+ 1
TUPLE_121(uint208,uint208) = LIBRARY_CALL, dest:Checkpoints, function:Checkpoints.push(Checkpoints.Trace208,uint48,uint208), arguments:['REF_4895', 'TMP_12032', 'TMP_12035'] 
 params.length > 0 && support == 1
REF_4899 -> LENGTH params_1
TMP_12036(bool) = REF_4899 > 0
TMP_12037(bool) = support_1 == 1
TMP_12038(bool) = TMP_12036 && TMP_12037
CONDITION TMP_12038
 _updateMaturity(account,proposalId,weight,params)
INTERNAL_CALL, AgentDAO._updateMaturity(address,uint256,uint256,bytes)(account_1,proposalId_1,weight_1,params_1)
 support == 1
TMP_12040(bool) = support_1 == 1
CONDITION TMP_12040
 _tryAutoExecute(proposalId)
INTERNAL_CALL, AgentDAO._tryAutoExecute(uint256)(proposalId_1)
 weight
RETURN weight_1
```
#### AgentDAO._propose(address[],uint256[],bytes[],string,address) [INTERNAL]
```slithir
targets_1(address[]) := phi(['targets_1', 'targets_1'])
values_1(uint256[]) := phi(['values_1', 'values_1'])
calldatas_1(bytes[]) := phi(['calldatas_1', 'calldatas_1'])
description_1(string) := phi(['description_1', 'description_1'])
proposer_1(address) := phi(['proposer_1', 'proposer_1'])
 super._propose(targets,values,calldatas,description,proposer)
TMP_12019(uint256) = INTERNAL_CALL, GovernorStorageUpgradeable._propose(address[],uint256[],bytes[],string,address)(targets_1,values_1,calldatas_1,description_1,proposer_1)
RETURN TMP_12019
```
#### AgentDAO._tryAutoExecute(uint256) [INTERNAL]
```slithir
proposalId_1(uint256) := phi(['proposalId_1'])
 (None,forVotes,None) = proposalVotes(proposalId)
TUPLE_122(uint256,uint256,uint256) = INTERNAL_CALL, GovernorCountingSimpleUpgradeable.proposalVotes(uint256)(proposalId_1)
forVotes_1(uint256)= UNPACK TUPLE_122 index: 1 
 forVotes == token().getPastTotalSupply(proposalSnapshot(proposalId))
TMP_12042(IERC5805) = INTERNAL_CALL, GovernorVotesUpgradeable.token()()
TMP_12043(uint256) = INTERNAL_CALL, GovernorUpgradeable.proposalSnapshot(uint256)(proposalId_1)
TMP_12044(uint256) = HIGH_LEVEL_CALL, dest:TMP_12042(IERC5805), function:getPastTotalSupply, arguments:['TMP_12043']  
TMP_12045(bool) = forVotes_1 == TMP_12044
CONDITION TMP_12045
 execute(proposalId)
INTERNAL_CALL, GovernorStorageUpgradeable.execute(uint256)(proposalId_1)
```
#### AgentDAO._updateMaturity(address,uint256,uint256,bytes) [INTERNAL]
```slithir
account_1(address) := phi(['account_1'])
proposalId_1(uint256) := phi(['proposalId_1'])
weight_1(uint256) := phi(['weight_1'])
params_1(bytes) := phi(['params_1'])
_proposalMaturities_1(mapping(uint256 => uint256)) := phi(['_proposalMaturities_4', '_proposalMaturities_3', '_proposalMaturities_0', '_proposalMaturities_8', '_proposalMaturities_6'])
_agentNft_9(address) := phi(['_agentNft_1', '_agentNft_10', '_agentNft_16', '_agentNft_8', '_agentNft_0'])
 contributionNft = IAgentNft(_agentNft).getContributionNft()
TMP_12047 = CONVERT _agentNft_9 to IAgentNft
TMP_12048(address) = HIGH_LEVEL_CALL, dest:TMP_12047(IAgentNft), function:getContributionNft, arguments:[]  
_proposalMaturities_2(mapping(uint256 => uint256)) := phi(['_proposalMaturities_4', '_proposalMaturities_3', '_proposalMaturities_1', '_proposalMaturities_8', '_proposalMaturities_6'])
_agentNft_10(address) := phi(['_agentNft_1', '_agentNft_9', '_agentNft_10', '_agentNft_8', '_agentNft_16'])
contributionNft_1(address) := TMP_12048(address)
 owner = IERC721(contributionNft).ownerOf(proposalId)
TMP_12049 = CONVERT contributionNft_1 to IERC721
TMP_12050(address) = HIGH_LEVEL_CALL, dest:TMP_12049(IERC721), function:ownerOf, arguments:['proposalId_1']  
_proposalMaturities_3(mapping(uint256 => uint256)) := phi(['_proposalMaturities_4', '_proposalMaturities_2', '_proposalMaturities_3', '_proposalMaturities_8', '_proposalMaturities_6'])
owner_1(address) := TMP_12050(address)
 owner == address(0)
TMP_12051 = CONVERT 0 to address
TMP_12052(bool) = owner_1 == TMP_12051
CONDITION TMP_12052
 isModel = IContributionNft(contributionNft).isModel(proposalId)
TMP_12053 = CONVERT contributionNft_1 to IContributionNft
TMP_12054(bool) = HIGH_LEVEL_CALL, dest:TMP_12053(IContributionNft), function:isModel, arguments:['proposalId_1']  
_proposalMaturities_4(mapping(uint256 => uint256)) := phi(['_proposalMaturities_8', '_proposalMaturities_3', '_proposalMaturities_4', '_proposalMaturities_6'])
isModel_1(bool) := TMP_12054(bool)
 ! isModel
TMP_12055 = UnaryType.BANG isModel_1 
CONDITION TMP_12055
 votes = abi.decode(params,(uint8[]))
TMP_12056(uint8[]) = SOLIDITY_CALL abi.decode()(params_1,uint8[])
votes_1(uint8[]) = ['TMP_12056(uint8[])']
 maturity = _calcMaturity(proposalId,votes)
TMP_12057(uint256) = INTERNAL_CALL, AgentDAO._calcMaturity(uint256,uint8[])(proposalId_1,votes_1)
maturity_1(uint256) := TMP_12057(uint256)
 _proposalMaturities[proposalId] += (maturity * weight)
REF_4905(uint256) -> _proposalMaturities_5[proposalId_1]
TMP_12058(uint256) = maturity_1 (c)* weight_1
_proposalMaturities_6(mapping(uint256 => uint256)) := phi(['_proposalMaturities_5'])
REF_4905(-> _proposalMaturities_6) = REF_4905 (c)+ TMP_12058
 ValidatorEloRating(proposalId,account,maturity,votes)
Emit ValidatorEloRating(proposalId_1,account_1,maturity_1,votes_1)
```
#### AgentDAO.constructor() [PUBLIC]
```slithir
 _disableInitializers()
INTERNAL_CALL, Initializable._disableInitializers()()
```
#### AgentDAO.getMaturity(uint256) [PUBLIC]
```slithir
_proposalMaturities_7(mapping(uint256 => uint256)) := phi(['_proposalMaturities_4', '_proposalMaturities_3', '_proposalMaturities_0', '_proposalMaturities_8', '_proposalMaturities_6'])
 (None,forVotes,None) = proposalVotes(proposalId)
TUPLE_123(uint256,uint256,uint256) = INTERNAL_CALL, GovernorCountingSimpleUpgradeable.proposalVotes(uint256)(proposalId_1)
forVotes_1(uint256)= UNPACK TUPLE_123 index: 1 
 Math.min(10000,_proposalMaturities[proposalId] / forVotes)
REF_4915(uint256) -> _proposalMaturities_8[proposalId_1]
TMP_12077(uint256) = REF_4915 (c)/ forVotes_1
TMP_12078(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['10000', 'TMP_12077'] 
RETURN TMP_12078
```
#### AgentDAO.getPastScore(address,uint256) [EXTERNAL]
```slithir
_scores_2(mapping(address => Checkpoints.Trace208)) := phi(['_scores_0', '_scores_7', '_scores_1', '_scores_3'])
 currentTimepoint = clock()
TMP_12022(uint48) = INTERNAL_CALL, GovernorVotesUpgradeable.clock()()
currentTimepoint_1(uint48) := TMP_12022(uint48)
 timepoint >= currentTimepoint
TMP_12023(bool) = timepoint_1 >= currentTimepoint_1
CONDITION TMP_12023
 revert ERC5805FutureLookup(uint256,uint48)(timepoint,currentTimepoint)
TMP_12024(None) = SOLIDITY_CALL revert ERC5805FutureLookup(uint256,uint48)(timepoint_1,currentTimepoint_1)
 _scores[account].upperLookupRecent(SafeCast.toUint48(timepoint))
REF_4892(Checkpoints.Trace208) -> _scores_3[account_1]
TMP_12025(uint48) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint48(uint256), arguments:['timepoint_1'] 
TMP_12026(uint208) = LIBRARY_CALL, dest:Checkpoints, function:Checkpoints.upperLookupRecent(Checkpoints.Trace208,uint48), arguments:['REF_4892', 'TMP_12025'] 
RETURN TMP_12026
```
#### AgentDAO.initialize(string,IVotes,address,uint256,uint32) [EXTERNAL]
```slithir
 __Governor_init(name)
INTERNAL_CALL, GovernorUpgradeable.__Governor_init(string)(name_1)
 __GovernorSettings_init(0,votingPeriod_,threshold)
INTERNAL_CALL, GovernorSettingsUpgradeable.__GovernorSettings_init(uint48,uint32,uint256)(0,votingPeriod__1,threshold_1)
 __GovernorCountingSimple_init()
INTERNAL_CALL, GovernorCountingSimpleUpgradeable.__GovernorCountingSimple_init()()
 __GovernorVotes_init(token)
INTERNAL_CALL, GovernorVotesUpgradeable.__GovernorVotes_init(IVotes)(token_1)
 __GovernorVotesQuorumFraction_init(5100)
INTERNAL_CALL, GovernorVotesQuorumFractionUpgradeable.__GovernorVotesQuorumFraction_init(uint256)(5100)
 __GovernorStorage_init()
INTERNAL_CALL, GovernorStorageUpgradeable.__GovernorStorage_init()()
 _agentNft = agentNft
_agentNft_1(address) := agentNft_1(address)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### AgentDAO.proposalCount() [PUBLIC]
```slithir
 super.proposalCount()
TMP_12020(uint256) = INTERNAL_CALL, GovernorStorageUpgradeable.proposalCount()()
RETURN TMP_12020
```
#### AgentDAO.proposalThreshold() [PUBLIC]
```slithir
 super.proposalThreshold()
TMP_12001(uint256) = INTERNAL_CALL, GovernorSettingsUpgradeable.proposalThreshold()()
RETURN TMP_12001
```
#### AgentDAO.propose(address[],uint256[],bytes[],string) [PUBLIC]
```slithir
_agentNft_2(address) := phi(['_agentNft_1', '_agentNft_10', '_agentNft_16', '_agentNft_8', '_agentNft_0'])
 proposer = _msgSender()
TMP_12002(address) = INTERNAL_CALL, ContextUpgradeable._msgSender()()
proposer_1(address) := TMP_12002(address)
 ! _isValidDescriptionForProposer(proposer,description)
TMP_12003(bool) = INTERNAL_CALL, GovernorUpgradeable._isValidDescriptionForProposer(address,string)(proposer_1,description_1)
TMP_12004 = UnaryType.BANG TMP_12003 
CONDITION TMP_12004
 revert GovernorRestrictedProposer(address)(proposer)
TMP_12005(None) = SOLIDITY_CALL revert GovernorRestrictedProposer(address)(proposer_1)
 proposerVotes = getVotes(proposer,clock() - 1)
TMP_12006(uint48) = INTERNAL_CALL, GovernorVotesUpgradeable.clock()()
TMP_12007(uint48) = TMP_12006 (c)- 1
TMP_12008(uint256) = INTERNAL_CALL, GovernorUpgradeable.getVotes(address,uint256)(proposer_1,TMP_12007)
proposerVotes_1(uint256) := TMP_12008(uint256)
 votesThreshold = proposalThreshold()
TMP_12009(uint256) = INTERNAL_CALL, AgentDAO.proposalThreshold()()
votesThreshold_1(uint256) := TMP_12009(uint256)
 contributionNft = IAgentNft(_agentNft).getContributionNft()
TMP_12010 = CONVERT _agentNft_7 to IAgentNft
TMP_12011(address) = HIGH_LEVEL_CALL, dest:TMP_12010(IAgentNft), function:getContributionNft, arguments:[]  
_agentNft_8(address) := phi(['_agentNft_1', '_agentNft_10', '_agentNft_8', '_agentNft_16', '_agentNft_7'])
contributionNft_1(address) := TMP_12011(address)
 proposerVotes < votesThreshold && proposer != IContributionNft(contributionNft).getAdmin()
TMP_12012(bool) = proposerVotes_1 < votesThreshold_1
TMP_12013 = CONVERT contributionNft_1 to IContributionNft
TMP_12014(address) = HIGH_LEVEL_CALL, dest:TMP_12013(IContributionNft), function:getAdmin, arguments:[]  
TMP_12015(bool) = proposer_1 != TMP_12014
TMP_12016(bool) = TMP_12012 && TMP_12015
CONDITION TMP_12016
 revert GovernorInsufficientProposerVotes(address,uint256,uint256)(proposer,proposerVotes,votesThreshold)
TMP_12017(None) = SOLIDITY_CALL revert GovernorInsufficientProposerVotes(address,uint256,uint256)(proposer_1,proposerVotes_1,votesThreshold_1)
 _propose(targets,values,calldatas,description,proposer)
TMP_12018(uint256) = INTERNAL_CALL, AgentDAO._propose(address[],uint256[],bytes[],string,address)(targets_1,values_1,calldatas_1,description_1,proposer_1)
RETURN TMP_12018
```
#### AgentDAO.quorum(uint256) [PUBLIC]
```slithir
blockNumber_1(uint256) := phi(['TMP_11953'])
 super.quorum(blockNumber)
TMP_12079(uint256) = INTERNAL_CALL, GovernorVotesQuorumFractionUpgradeable.quorum(uint256)(blockNumber_1)
RETURN TMP_12079
```
#### AgentDAO.quorumDenominator() [PUBLIC]
```slithir
 10000
RETURN 10000
```
#### AgentDAO.scoreOf(address) [PUBLIC]
```slithir
account_1(address) := phi(['account_1'])
_scores_1(mapping(address => Checkpoints.Trace208)) := phi(['_scores_0', '_scores_7', '_scores_1', '_scores_3'])
 _scores[account].latest()
REF_4890(Checkpoints.Trace208) -> _scores_1[account_1]
TMP_12021(uint208) = LIBRARY_CALL, dest:Checkpoints, function:Checkpoints.latest(Checkpoints.Trace208), arguments:['REF_4890'] 
RETURN TMP_12021
```

#### AgentDAO.state(uint256) [PUBLIC]
```slithir
proposalId_1(uint256) := phi(['proposalId_1', 'proposalId_1'])
 currentState = super.state(proposalId)
TMP_12080(IGovernor.ProposalState) = INTERNAL_CALL, GovernorUpgradeable.state(uint256)(proposalId_1)
currentState_1(IGovernor.ProposalState) := TMP_12080(IGovernor.ProposalState)
 currentState == ProposalState.Active
REF_4916(IGovernor.ProposalState) -> ProposalState.Active
TMP_12081(bool) = currentState_1 == REF_4916
CONDITION TMP_12081
 (None,forVotes,None) = proposalVotes(proposalId)
TUPLE_124(uint256,uint256,uint256) = INTERNAL_CALL, GovernorCountingSimpleUpgradeable.proposalVotes(uint256)(proposalId_1)
forVotes_1(uint256)= UNPACK TUPLE_124 index: 1 
 forVotes == token().getPastTotalSupply(proposalSnapshot(proposalId))
TMP_12082(IERC5805) = INTERNAL_CALL, GovernorVotesUpgradeable.token()()
TMP_12083(uint256) = INTERNAL_CALL, GovernorUpgradeable.proposalSnapshot(uint256)(proposalId_1)
TMP_12084(uint256) = HIGH_LEVEL_CALL, dest:TMP_12082(IERC5805), function:getPastTotalSupply, arguments:['TMP_12083']  
TMP_12085(bool) = forVotes_1 == TMP_12084
CONDITION TMP_12085
 ProposalState.Succeeded
REF_4918(IGovernor.ProposalState) -> ProposalState.Succeeded
RETURN REF_4918
 currentState
RETURN currentState_1
```
#### AgentDAO.totalScore() [PUBLIC]
```slithir
_totalScore_6(uint256) := phi(['_totalScore_0', '_totalScore_4'])
 _totalScore
RETURN _totalScore_6
```
#### AgentDAO.votingDelay() [PUBLIC]
```slithir
 super.votingDelay()
TMP_11999(uint256) = INTERNAL_CALL, GovernorSettingsUpgradeable.votingDelay()()
RETURN TMP_11999
```
#### AgentDAO.votingPeriod() [PUBLIC]
```slithir
 super.votingPeriod()
TMP_12000(uint256) = INTERNAL_CALL, GovernorSettingsUpgradeable.votingPeriod()()
RETURN TMP_12000
```
#### IContributionNft.getCore(uint256) [EXTERNAL]
```slithir

```
#### IContributionNft.tokenVirtualId(uint256) [EXTERNAL]
```slithir

```
#### IServiceNft.getCoreService(uint256,uint8) [EXTERNAL]
```slithir

```
#### IServiceNft.getMaturity(uint256) [EXTERNAL]
```slithir

```
#### IAgentNft.getContributionNft() [EXTERNAL]
```slithir

```

#### IAgentNft.getServiceNft() [EXTERNAL]
```slithir

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
#### Checkpoints.push(Checkpoints.Trace160,uint96,uint160) [INTERNAL]
```slithir
 _insert(self._checkpoints,key,value)
REF_2212(Checkpoints.Checkpoint160[]) -> self_1 (-> [])._checkpoints
TUPLE_69(uint160,uint160) = INTERNAL_CALL, Checkpoints._insert(Checkpoints.Checkpoint160[],uint96,uint160)(REF_2212,key_1,value_1)
RETURN TUPLE_69
```
#### IERC721.ownerOf(uint256) [EXTERNAL]
```slithir

```
#### IContributionNft.isModel(uint256) [EXTERNAL]
```slithir

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
#### IContributionNft.getAdmin() [EXTERNAL]
```slithir

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

#### Math.average(uint256,uint256) [INTERNAL]
```slithir
 (a & b) + (a ^ b) / 2
TMP_5709(uint256) = a_1 & b_1
TMP_5710(uint256) = a_1 ^ b_1
TMP_5711(uint256) = TMP_5710 (c)/ 2
TMP_5712(uint256) = TMP_5709 (c)+ TMP_5711
RETURN TMP_5712
```
