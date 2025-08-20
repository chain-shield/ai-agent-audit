

### Storage layout (Genesis) 

```text
mapAddrToVirtuals mapping(address => uint256)
claimableAgentTokens mapping(address => uint256)
participants address[]
refundUserCountForFailed uint256
genesisId uint256
factory FGenesis
startTime uint256
endTime uint256
genesisName string
genesisTicker string
genesisCores uint8[]
tbaSalt bytes32
tbaImplementation address
daoVotingPeriod uint32
daoThreshold uint256
agentFactoryAddress address
virtualTokenAddress address
reserveAmount uint256
maxContributionVirtualAmount uint256
agentTokenTotalSupply uint256
agentTokenLpSupply uint256
agentTokenAddress address
isFailed bool
isCancelled bool

```


#### Genesis._validateTime(uint256,uint256) [INTERNAL]
```slithir
_startTime_1(uint256) := phi(['REF_3585', 'newStartTime_1'])
_endTime_1(uint256) := phi(['REF_3586', 'newEndTime_1'])
ERR_START_TIME_FUTURE_1(string) := phi(['ERR_START_TIME_FUTURE_0'])
ERR_END_AFTER_START_1(string) := phi(['ERR_END_AFTER_START_0'])
 require(bool,string)(_startTime > block.timestamp,ERR_START_TIME_FUTURE)
TMP_8866(bool) = _startTime_1 > block.timestamp
TMP_8867(None) = SOLIDITY_CALL require(bool,string)(TMP_8866,ERR_START_TIME_FUTURE_1)
 require(bool,string)(_endTime > _startTime,ERR_END_AFTER_START)
TMP_8868(bool) = _endTime_1 > _startTime_1
TMP_8869(None) = SOLIDITY_CALL require(bool,string)(TMP_8868,ERR_END_AFTER_START_1)
```
#### Genesis.cancelGenesis() [EXTERNAL]
```slithir
FACTORY_ROLE_13(bytes32) := phi(['FACTORY_ROLE_6', 'FACTORY_ROLE_10', 'FACTORY_ROLE_12', 'FACTORY_ROLE_8', 'FACTORY_ROLE_14', 'FACTORY_ROLE_0'])
genesisId_34(uint256) := phi(['genesisId_33', 'genesisId_6', 'genesisId_26', 'genesisId_0', 'genesisId_1', 'genesisId_19', 'genesisId_40', 'genesisId_17'])
 isCancelled = true
isCancelled_2(bool) := True(bool)
 GenesisCancelled(genesisId)
Emit GenesisCancelled(genesisId_40)
 onlyRole(FACTORY_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(FACTORY_ROLE_13)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
 whenNotCancelled()
MODIFIER_CALL, Genesis.whenNotCancelled()()
 whenNotFailed()
MODIFIER_CALL, Genesis.whenNotFailed()()
 whenNotStarted()
MODIFIER_CALL, Genesis.whenNotStarted()()
 whenNotEnded()
MODIFIER_CALL, Genesis.whenNotEnded()()
```
#### Genesis.claimAgentToken(address) [EXTERNAL]
```slithir
claimableAgentTokens_2(mapping(address => uint256)) := phi(['claimableAgentTokens_1', 'claimableAgentTokens_4', 'claimableAgentTokens_0', 'claimableAgentTokens_5'])
genesisId_18(uint256) := phi(['genesisId_33', 'genesisId_6', 'genesisId_26', 'genesisId_0', 'genesisId_1', 'genesisId_19', 'genesisId_40', 'genesisId_17'])
agentTokenAddress_10(address) := phi(['agentTokenAddress_0', 'agentTokenAddress_9', 'agentTokenAddress_16', 'agentTokenAddress_11'])
 amount = claimableAgentTokens[userAddress]
REF_3659(uint256) -> claimableAgentTokens_3[userAddress_1]
amount_1(uint256) := REF_3659(uint256)
 require(bool,string)(amount > 0,No tokens to claim)
TMP_8982(bool) = amount_1 > 0
TMP_8983(None) = SOLIDITY_CALL require(bool,string)(TMP_8982,No tokens to claim)
 claimableAgentTokens[userAddress] = 0
REF_3660(uint256) -> claimableAgentTokens_3[userAddress_1]
claimableAgentTokens_4(mapping(address => uint256)) := phi(['claimableAgentTokens_3'])
REF_3660(uint256) (->claimableAgentTokens_4) := 0(uint256)
 IERC20(agentTokenAddress).safeTransfer(userAddress,amount)
TMP_8984 = CONVERT agentTokenAddress_11 to IERC20
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['TMP_8984', 'userAddress_1', 'amount_1'] 
 AgentTokenClaimed(genesisId,userAddress,amount)
Emit AgentTokenClaimed(genesisId_19,userAddress_1,amount_1)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### Genesis.getClaimableAgentToken(address) [EXTERNAL]
```slithir
claimableAgentTokens_5(mapping(address => uint256)) := phi(['claimableAgentTokens_1', 'claimableAgentTokens_4', 'claimableAgentTokens_0', 'claimableAgentTokens_5'])
 claimableAgentTokens[userAddress]
REF_3662(uint256) -> claimableAgentTokens_5[userAddress_1]
RETURN REF_3662
```
#### Genesis.getGenesisInfo() [PUBLIC]
```slithir
genesisId_27(uint256) := phi(['genesisId_33', 'genesisId_6', 'genesisId_26', 'genesisId_0', 'genesisId_1', 'genesisId_19', 'genesisId_40', 'genesisId_17'])
factory_2(FGenesis) := phi(['factory_0', 'factory_1'])
startTime_3(uint256) := phi(['startTime_12', 'startTime_0', 'startTime_1'])
endTime_3(uint256) := phi(['endTime_0', 'endTime_12', 'endTime_1'])
genesisName_11(string) := phi(['genesisName_10', 'genesisName_8', 'genesisName_0', 'genesisName_1'])
genesisTicker_11(string) := phi(['genesisTicker_8', 'genesisTicker_10', 'genesisTicker_1', 'genesisTicker_0'])
genesisCores_11(uint8[]) := phi(['genesisCores_0', 'genesisCores_1', 'genesisCores_10', 'genesisCores_8'])
tbaSalt_11(bytes32) := phi(['tbaSalt_1', 'tbaSalt_0', 'tbaSalt_8', 'tbaSalt_10'])
tbaImplementation_11(address) := phi(['tbaImplementation_10', 'tbaImplementation_1', 'tbaImplementation_8', 'tbaImplementation_0'])
daoVotingPeriod_11(uint32) := phi(['daoVotingPeriod_1', 'daoVotingPeriod_0', 'daoVotingPeriod_8', 'daoVotingPeriod_10'])
daoThreshold_11(uint256) := phi(['daoThreshold_1', 'daoThreshold_10', 'daoThreshold_0', 'daoThreshold_8'])
agentFactoryAddress_12(address) := phi(['agentFactoryAddress_0', 'agentFactoryAddress_11', 'agentFactoryAddress_1', 'agentFactoryAddress_8'])
virtualTokenAddress_25(address) := phi(['virtualTokenAddress_17', 'virtualTokenAddress_6', 'virtualTokenAddress_24', 'virtualTokenAddress_1', 'virtualTokenAddress_0'])
reserveAmount_11(uint256) := phi(['reserveAmount_8', 'reserveAmount_10', 'reserveAmount_0', 'reserveAmount_1'])
maxContributionVirtualAmount_5(uint256) := phi(['maxContributionVirtualAmount_0', 'maxContributionVirtualAmount_4', 'maxContributionVirtualAmount_1'])
agentTokenTotalSupply_12(uint256) := phi(['agentTokenTotalSupply_11', 'agentTokenTotalSupply_0', 'agentTokenTotalSupply_1', 'agentTokenTotalSupply_8'])
agentTokenLpSupply_12(uint256) := phi(['agentTokenLpSupply_11', 'agentTokenLpSupply_1', 'agentTokenLpSupply_8', 'agentTokenLpSupply_0'])
agentTokenAddress_12(address) := phi(['agentTokenAddress_0', 'agentTokenAddress_9', 'agentTokenAddress_16', 'agentTokenAddress_11'])
isFailed_2(bool) := phi(['isFailed_6', 'isFailed_8', 'isFailed_0', 'isFailed_1'])
isCancelled_1(bool) := phi(['isCancelled_0', 'isCancelled_8', 'isCancelled_2', 'isCancelled_6'])
 GenesisInfo({genesisId:genesisId,factory:address(factory),startTime:startTime,endTime:endTime,genesisName:genesisName,genesisTicker:genesisTicker,genesisCores:genesisCores,tbaSalt:tbaSalt,tbaImplementation:tbaImplementation,daoVotingPeriod:daoVotingPeriod,daoThreshold:daoThreshold,agentFactoryAddress:agentFactoryAddress,virtualTokenAddress:virtualTokenAddress,reserveAmount:reserveAmount,maxContributionVirtualAmount:maxContributionVirtualAmount,agentTokenTotalSupply:agentTokenTotalSupply,agentTokenLpSupply:agentTokenLpSupply,agentTokenAddress:agentTokenAddress,isFailed:isFailed,isCancelled:isCancelled})
TMP_9024 = CONVERT factory_2 to address
TMP_9025(Genesis.GenesisInfo) = new GenesisInfo(genesisId_27,TMP_9024,startTime_3,endTime_3,genesisName_11,genesisTicker_11,genesisCores_11,tbaSalt_11,tbaImplementation_11,daoVotingPeriod_11,daoThreshold_11,agentFactoryAddress_12,virtualTokenAddress_25,reserveAmount_11,maxContributionVirtualAmount_5,agentTokenTotalSupply_12,agentTokenLpSupply_12,agentTokenAddress_12,isFailed_2,isCancelled_1)
RETURN TMP_9025
```
#### Genesis.getParticipantCount() [EXTERNAL]
```slithir
participants_15(address[]) := phi(['participants_15', 'participants_14', 'participants_5', 'participants_0', 'participants_16', 'participants_17', 'participants_7'])
 participants.length
REF_3672 -> LENGTH participants_15
RETURN REF_3672
```
#### Genesis.getParticipantsInfo(uint256[]) [EXTERNAL]
```slithir
mapAddrToVirtuals_27(mapping(address => uint256)) := phi(['mapAddrToVirtuals_0', 'mapAddrToVirtuals_27', 'mapAddrToVirtuals_18', 'mapAddrToVirtuals_6', 'mapAddrToVirtuals_17', 'mapAddrToVirtuals_25'])
participants_17(address[]) := phi(['participants_15', 'participants_14', 'participants_5', 'participants_0', 'participants_16', 'participants_17', 'participants_7'])
 length = participantIndexes.length
REF_3678 -> LENGTH participantIndexes_1
length_1(uint256) := REF_3678(uint256)
 result = new Genesis.ParticipantInfo[](length)
TMP_9018(Genesis.ParticipantInfo[])  = new Genesis.ParticipantInfo[](length_1)
result_1(Genesis.ParticipantInfo[]) = ['TMP_9018(Genesis.ParticipantInfo[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < length
result_2(Genesis.ParticipantInfo[]) := phi(['result_1', 'result_3'])
i_2(uint256) := phi(['i_1', 'i_3'])
TMP_9019(bool) = i_2 < length_1
CONDITION TMP_9019
 require(bool,string)(participantIndexes[i] < participants.length,Index out of bounds)
REF_3679(uint256) -> participantIndexes_1[i_2]
REF_3680 -> LENGTH participants_17
TMP_9020(bool) = REF_3679 < REF_3680
TMP_9021(None) = SOLIDITY_CALL require(bool,string)(TMP_9020,Index out of bounds)
 userAddress = participants[participantIndexes[i]]
REF_3681(uint256) -> participantIndexes_1[i_2]
REF_3682(address) -> participants_17[REF_3681]
userAddress_1(address) := REF_3682(address)
 result[i] = ParticipantInfo({userAddress:userAddress,virtuals:mapAddrToVirtuals[userAddress]})
REF_3683(Genesis.ParticipantInfo) -> result_2[i_2]
REF_3684(uint256) -> mapAddrToVirtuals_27[userAddress_1]
TMP_9022(Genesis.ParticipantInfo) = new ParticipantInfo(userAddress_1,REF_3684)
result_3(Genesis.ParticipantInfo[]) := phi(['result_2'])
REF_3683(Genesis.ParticipantInfo) (->result_3) := TMP_9022(Genesis.ParticipantInfo)
 i ++
TMP_9023(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 result
RETURN result_2
```
#### Genesis.getParticipantsPaginated(uint256,uint256) [EXTERNAL]
```slithir
participants_16(address[]) := phi(['participants_15', 'participants_14', 'participants_5', 'participants_0', 'participants_16', 'participants_17', 'participants_7'])
 require(bool,string)(startIndex < participants.length,Start index out of bounds)
REF_3673 -> LENGTH participants_16
TMP_9007(bool) = startIndex_1 < REF_3673
TMP_9008(None) = SOLIDITY_CALL require(bool,string)(TMP_9007,Start index out of bounds)
 actualPageSize = pageSize
actualPageSize_1(uint256) := pageSize_1(uint256)
 startIndex + pageSize > participants.length
TMP_9009(uint256) = startIndex_1 (c)+ pageSize_1
REF_3674 -> LENGTH participants_16
TMP_9010(bool) = TMP_9009 > REF_3674
CONDITION TMP_9010
 actualPageSize = participants.length - startIndex
REF_3675 -> LENGTH participants_16
TMP_9011(uint256) = REF_3675 (c)- startIndex_1
actualPageSize_2(uint256) := TMP_9011(uint256)
actualPageSize_3(uint256) := phi(['actualPageSize_2', 'actualPageSize_1'])
 page = new address[](actualPageSize)
TMP_9013(address[])  = new address[](actualPageSize_3)
page_1(address[]) = ['TMP_9013(address[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < actualPageSize
i_2(uint256) := phi(['i_1', 'i_3'])
TMP_9014(bool) = i_2 < actualPageSize_3
CONDITION TMP_9014
 page[i] = participants[startIndex + i]
REF_3676(address) -> page_1[i_2]
TMP_9015(uint256) = startIndex_1 (c)+ i_2
REF_3677(address) -> participants_16[TMP_9015]
page_2(address[]) := phi(['page_1'])
REF_3676(address) (->page_2) := REF_3677(address)
 i ++
TMP_9016(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 page
RETURN page_1
```
#### Genesis.initialize(GenesisInitParams) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_7', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_5'])
FACTORY_ROLE_1(bytes32) := phi(['FACTORY_ROLE_6', 'FACTORY_ROLE_10', 'FACTORY_ROLE_12', 'FACTORY_ROLE_8', 'FACTORY_ROLE_14', 'FACTORY_ROLE_0'])
 __AccessControl_init()
INTERNAL_CALL, AccessControlUpgradeable.__AccessControl_init()()
 require(bool,string)(params.genesisID > 0,Invalid genesis ID)
REF_3583(uint256) -> params_1.genesisID
TMP_8871(bool) = REF_3583 > 0
TMP_8872(None) = SOLIDITY_CALL require(bool,string)(TMP_8871,Invalid genesis ID)
 require(bool,string)(params.factory != address(0),Invalid factory address)
REF_3584(address) -> params_1.factory
TMP_8873 = CONVERT 0 to address
TMP_8874(bool) = REF_3584 != TMP_8873
TMP_8875(None) = SOLIDITY_CALL require(bool,string)(TMP_8874,Invalid factory address)
 _validateTime(params.startTime,params.endTime)
REF_3585(uint256) -> params_1.startTime
REF_3586(uint256) -> params_1.endTime
INTERNAL_CALL, Genesis._validateTime(uint256,uint256)(REF_3585,REF_3586)
 require(bool,string)(bytes(params.genesisName).length > 0,Invalid genesis name)
REF_3587(string) -> params_1.genesisName
TMP_8877 = CONVERT REF_3587 to bytes
REF_3588 -> LENGTH TMP_8877
TMP_8878(bool) = REF_3588 > 0
TMP_8879(None) = SOLIDITY_CALL require(bool,string)(TMP_8878,Invalid genesis name)
 require(bool,string)(bytes(params.genesisTicker).length > 0,Invalid genesis ticker)
REF_3589(string) -> params_1.genesisTicker
TMP_8880 = CONVERT REF_3589 to bytes
REF_3590 -> LENGTH TMP_8880
TMP_8881(bool) = REF_3590 > 0
TMP_8882(None) = SOLIDITY_CALL require(bool,string)(TMP_8881,Invalid genesis ticker)
 require(bool,string)(params.genesisCores.length > 0,Invalid genesis cores)
REF_3591(uint8[]) -> params_1.genesisCores
REF_3592 -> LENGTH REF_3591
TMP_8883(bool) = REF_3592 > 0
TMP_8884(None) = SOLIDITY_CALL require(bool,string)(TMP_8883,Invalid genesis cores)
 require(bool,string)(params.tbaImplementation != address(0),Invalid TBA implementation address)
REF_3593(address) -> params_1.tbaImplementation
TMP_8885 = CONVERT 0 to address
TMP_8886(bool) = REF_3593 != TMP_8885
TMP_8887(None) = SOLIDITY_CALL require(bool,string)(TMP_8886,Invalid TBA implementation address)
 require(bool,string)(params.agentFactoryAddress != address(0),Invalid agent factory address)
REF_3594(address) -> params_1.agentFactoryAddress
TMP_8888 = CONVERT 0 to address
TMP_8889(bool) = REF_3594 != TMP_8888
TMP_8890(None) = SOLIDITY_CALL require(bool,string)(TMP_8889,Invalid agent factory address)
 require(bool,string)(params.virtualTokenAddress != address(0),Invalid virtual token address)
REF_3595(address) -> params_1.virtualTokenAddress
TMP_8891 = CONVERT 0 to address
TMP_8892(bool) = REF_3595 != TMP_8891
TMP_8893(None) = SOLIDITY_CALL require(bool,string)(TMP_8892,Invalid virtual token address)
 require(bool,string)(params.reserveAmount > 0,Reserve amount must be greater than 0)
REF_3596(uint256) -> params_1.reserveAmount
TMP_8894(bool) = REF_3596 > 0
TMP_8895(None) = SOLIDITY_CALL require(bool,string)(TMP_8894,Reserve amount must be greater than 0)
 require(bool,string)(params.maxContributionVirtualAmount > 0,Max contribution must be greater than 0)
REF_3597(uint256) -> params_1.maxContributionVirtualAmount
TMP_8896(bool) = REF_3597 > 0
TMP_8897(None) = SOLIDITY_CALL require(bool,string)(TMP_8896,Max contribution must be greater than 0)
 require(bool,string)(params.agentTokenTotalSupply > 0,Agent token total supply must be greater than 0)
REF_3598(uint256) -> params_1.agentTokenTotalSupply
TMP_8898(bool) = REF_3598 > 0
TMP_8899(None) = SOLIDITY_CALL require(bool,string)(TMP_8898,Agent token total supply must be greater than 0)
 require(bool,string)(params.agentTokenLpSupply > 0,Agent token lp supply must be greater than 0)
REF_3599(uint256) -> params_1.agentTokenLpSupply
TMP_8900(bool) = REF_3599 > 0
TMP_8901(None) = SOLIDITY_CALL require(bool,string)(TMP_8900,Agent token lp supply must be greater than 0)
 require(bool,string)(params.agentTokenTotalSupply >= params.agentTokenLpSupply,Agent token total supply must be greater than agent token lp supply)
REF_3600(uint256) -> params_1.agentTokenTotalSupply
REF_3601(uint256) -> params_1.agentTokenLpSupply
TMP_8902(bool) = REF_3600 >= REF_3601
TMP_8903(None) = SOLIDITY_CALL require(bool,string)(TMP_8902,Agent token total supply must be greater than agent token lp supply)
 genesisId = params.genesisID
REF_3602(uint256) -> params_1.genesisID
genesisId_1(uint256) := REF_3602(uint256)
 factory = FGenesis(params.factory)
REF_3603(address) -> params_1.factory
TMP_8904 = CONVERT REF_3603 to FGenesis
factory_1(FGenesis) := TMP_8904(FGenesis)
 startTime = params.startTime
REF_3604(uint256) -> params_1.startTime
startTime_1(uint256) := REF_3604(uint256)
 endTime = params.endTime
REF_3605(uint256) -> params_1.endTime
endTime_1(uint256) := REF_3605(uint256)
 genesisName = params.genesisName
REF_3606(string) -> params_1.genesisName
genesisName_1(string) := REF_3606(string)
 genesisTicker = params.genesisTicker
REF_3607(string) -> params_1.genesisTicker
genesisTicker_1(string) := REF_3607(string)
 genesisCores = params.genesisCores
REF_3608(uint8[]) -> params_1.genesisCores
genesisCores_1(uint8[]) = ['REF_3608(uint8[])']
 tbaSalt = params.tbaSalt
REF_3609(bytes32) -> params_1.tbaSalt
tbaSalt_1(bytes32) := REF_3609(bytes32)
 tbaImplementation = params.tbaImplementation
REF_3610(address) -> params_1.tbaImplementation
tbaImplementation_1(address) := REF_3610(address)
 daoVotingPeriod = params.daoVotingPeriod
REF_3611(uint32) -> params_1.daoVotingPeriod
daoVotingPeriod_1(uint32) := REF_3611(uint32)
 daoThreshold = params.daoThreshold
REF_3612(uint256) -> params_1.daoThreshold
daoThreshold_1(uint256) := REF_3612(uint256)
 agentFactoryAddress = params.agentFactoryAddress
REF_3613(address) -> params_1.agentFactoryAddress
agentFactoryAddress_1(address) := REF_3613(address)
 virtualTokenAddress = params.virtualTokenAddress
REF_3614(address) -> params_1.virtualTokenAddress
virtualTokenAddress_1(address) := REF_3614(address)
 reserveAmount = params.reserveAmount
REF_3615(uint256) -> params_1.reserveAmount
reserveAmount_1(uint256) := REF_3615(uint256)
 maxContributionVirtualAmount = params.maxContributionVirtualAmount
REF_3616(uint256) -> params_1.maxContributionVirtualAmount
maxContributionVirtualAmount_1(uint256) := REF_3616(uint256)
 agentTokenTotalSupply = params.agentTokenTotalSupply
REF_3617(uint256) -> params_1.agentTokenTotalSupply
agentTokenTotalSupply_1(uint256) := REF_3617(uint256)
 agentTokenLpSupply = params.agentTokenLpSupply
REF_3618(uint256) -> params_1.agentTokenLpSupply
agentTokenLpSupply_1(uint256) := REF_3618(uint256)
 _grantRole(DEFAULT_ADMIN_ROLE,params.factory)
REF_3619(address) -> params_1.factory
TMP_8905(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(DEFAULT_ADMIN_ROLE_4,REF_3619)
 _grantRole(FACTORY_ROLE,params.factory)
REF_3620(address) -> params_1.factory
TMP_8906(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(FACTORY_ROLE_5,REF_3620)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### Genesis.isEnded() [PUBLIC]
```slithir
endTime_2(uint256) := phi(['endTime_0', 'endTime_12', 'endTime_1'])
 block.timestamp >= endTime
TMP_9005(bool) = block.timestamp >= endTime_2
RETURN TMP_9005
```
#### Genesis.isStarted() [PUBLIC]
```slithir
startTime_2(uint256) := phi(['startTime_12', 'startTime_0', 'startTime_1'])
 block.timestamp >= startTime
TMP_9006(bool) = block.timestamp >= startTime_2
RETURN TMP_9006
```
#### Genesis.onGenesisFailed(uint256[]) [EXTERNAL]
```slithir
FACTORY_ROLE_9(bytes32) := phi(['FACTORY_ROLE_6', 'FACTORY_ROLE_10', 'FACTORY_ROLE_12', 'FACTORY_ROLE_8', 'FACTORY_ROLE_14', 'FACTORY_ROLE_0'])
mapAddrToVirtuals_19(mapping(address => uint256)) := phi(['mapAddrToVirtuals_0', 'mapAddrToVirtuals_27', 'mapAddrToVirtuals_18', 'mapAddrToVirtuals_6', 'mapAddrToVirtuals_17', 'mapAddrToVirtuals_25'])
participants_8(address[]) := phi(['participants_15', 'participants_14', 'participants_5', 'participants_0', 'participants_16', 'participants_17', 'participants_7'])
refundUserCountForFailed_7(uint256) := phi(['refundUserCountForFailed_13', 'refundUserCountForFailed_0', 'refundUserCountForFailed_6'])
genesisId_20(uint256) := phi(['genesisId_33', 'genesisId_6', 'genesisId_26', 'genesisId_0', 'genesisId_1', 'genesisId_19', 'genesisId_40', 'genesisId_17'])
virtualTokenAddress_18(address) := phi(['virtualTokenAddress_17', 'virtualTokenAddress_6', 'virtualTokenAddress_24', 'virtualTokenAddress_1', 'virtualTokenAddress_0'])
 i = 0
i_1(uint256) := 0(uint256)
 i < participantIndexes.length
i_2(uint256) := phi(['i_3', 'i_1'])
REF_3663 -> LENGTH participantIndexes_1
TMP_8988(bool) = i_2 < REF_3663
CONDITION TMP_8988
 require(bool,string)(participantIndexes[i] < participants.length,Index out of bounds)
REF_3664(uint256) -> participantIndexes_1[i_2]
REF_3665 -> LENGTH participants_14
TMP_8989(bool) = REF_3664 < REF_3665
TMP_8990(None) = SOLIDITY_CALL require(bool,string)(TMP_8989,Index out of bounds)
 participant = participants[participantIndexes[i]]
REF_3666(uint256) -> participantIndexes_1[i_2]
REF_3667(address) -> participants_14[REF_3666]
participant_1(address) := REF_3667(address)
 virtualsAmt = mapAddrToVirtuals[participant]
REF_3668(uint256) -> mapAddrToVirtuals_25[participant_1]
virtualsAmt_1(uint256) := REF_3668(uint256)
 virtualsAmt > 0
TMP_8991(bool) = virtualsAmt_1 > 0
CONDITION TMP_8991
 refundUserCountForFailed ++
TMP_8992(uint256) := refundUserCountForFailed_13(uint256)
refundUserCountForFailed_14(uint256) = refundUserCountForFailed_13 (c)+ 1
 mapAddrToVirtuals[participant] = 0
REF_3669(uint256) -> mapAddrToVirtuals_25[participant_1]
mapAddrToVirtuals_26(mapping(address => uint256)) := phi(['mapAddrToVirtuals_25'])
REF_3669(uint256) (->mapAddrToVirtuals_26) := 0(uint256)
 IERC20(virtualTokenAddress).safeTransfer(participant,virtualsAmt)
TMP_8993 = CONVERT virtualTokenAddress_24 to IERC20
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['TMP_8993', 'participant_1', 'virtualsAmt_1'] 
 RefundClaimed(genesisId,participant,virtualsAmt)
Emit RefundClaimed(genesisId_26,participant_1,virtualsAmt_1)
 i ++
TMP_8996(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 refundUserCountForFailed == participants.length
REF_3671 -> LENGTH participants_14
TMP_8997(bool) = refundUserCountForFailed_13 == REF_3671
CONDITION TMP_8997
 isFailed = true
isFailed_1(bool) := True(bool)
 GenesisFailed(genesisId)
Emit GenesisFailed(genesisId_26)
 onlyRole(FACTORY_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(FACTORY_ROLE_9)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
 whenNotCancelled()
MODIFIER_CALL, Genesis.whenNotCancelled()()
 whenNotFailed()
MODIFIER_CALL, Genesis.whenNotFailed()()
 whenTokenNotLaunched()
MODIFIER_CALL, Genesis.whenTokenNotLaunched()()
 whenEnded()
MODIFIER_CALL, Genesis.whenEnded()()
```
#### Genesis.onGenesisSuccess(address[],uint256[],address[],uint256[],address) [EXTERNAL]
```slithir
FACTORY_ROLE_7(bytes32) := phi(['FACTORY_ROLE_6', 'FACTORY_ROLE_10', 'FACTORY_ROLE_12', 'FACTORY_ROLE_8', 'FACTORY_ROLE_14', 'FACTORY_ROLE_0'])
mapAddrToVirtuals_7(mapping(address => uint256)) := phi(['mapAddrToVirtuals_0', 'mapAddrToVirtuals_27', 'mapAddrToVirtuals_18', 'mapAddrToVirtuals_6', 'mapAddrToVirtuals_17', 'mapAddrToVirtuals_25'])
refundUserCountForFailed_1(uint256) := phi(['refundUserCountForFailed_13', 'refundUserCountForFailed_0', 'refundUserCountForFailed_6'])
genesisId_7(uint256) := phi(['genesisId_33', 'genesisId_6', 'genesisId_26', 'genesisId_0', 'genesisId_1', 'genesisId_19', 'genesisId_40', 'genesisId_17'])
genesisName_2(string) := phi(['genesisName_10', 'genesisName_8', 'genesisName_0', 'genesisName_1'])
genesisTicker_2(string) := phi(['genesisTicker_8', 'genesisTicker_10', 'genesisTicker_1', 'genesisTicker_0'])
genesisCores_2(uint8[]) := phi(['genesisCores_0', 'genesisCores_1', 'genesisCores_10', 'genesisCores_8'])
tbaSalt_2(bytes32) := phi(['tbaSalt_1', 'tbaSalt_0', 'tbaSalt_8', 'tbaSalt_10'])
tbaImplementation_2(address) := phi(['tbaImplementation_10', 'tbaImplementation_1', 'tbaImplementation_8', 'tbaImplementation_0'])
daoVotingPeriod_2(uint32) := phi(['daoVotingPeriod_1', 'daoVotingPeriod_0', 'daoVotingPeriod_8', 'daoVotingPeriod_10'])
daoThreshold_2(uint256) := phi(['daoThreshold_1', 'daoThreshold_10', 'daoThreshold_0', 'daoThreshold_8'])
agentFactoryAddress_2(address) := phi(['agentFactoryAddress_0', 'agentFactoryAddress_11', 'agentFactoryAddress_1', 'agentFactoryAddress_8'])
virtualTokenAddress_7(address) := phi(['virtualTokenAddress_17', 'virtualTokenAddress_6', 'virtualTokenAddress_24', 'virtualTokenAddress_1', 'virtualTokenAddress_0'])
reserveAmount_2(uint256) := phi(['reserveAmount_8', 'reserveAmount_10', 'reserveAmount_0', 'reserveAmount_1'])
agentTokenTotalSupply_2(uint256) := phi(['agentTokenTotalSupply_11', 'agentTokenTotalSupply_0', 'agentTokenTotalSupply_1', 'agentTokenTotalSupply_8'])
agentTokenLpSupply_2(uint256) := phi(['agentTokenLpSupply_11', 'agentTokenLpSupply_1', 'agentTokenLpSupply_8', 'agentTokenLpSupply_0'])
agentTokenAddress_1(address) := phi(['agentTokenAddress_0', 'agentTokenAddress_9', 'agentTokenAddress_16', 'agentTokenAddress_11'])
 require(bool,string)(refundUserCountForFailed == 0,OnGenesisFailed already called)
TMP_8933(bool) = refundUserCountForFailed_6 == 0
TMP_8934(None) = SOLIDITY_CALL require(bool,string)(TMP_8933,OnGenesisFailed already called)
 require(bool,string)(refundVirtualsTokenUserAddresses.length == refundVirtualsTokenUserAmounts.length,Mismatched refund arrays)
REF_3629 -> LENGTH refundVirtualsTokenUserAddresses_1
REF_3630 -> LENGTH refundVirtualsTokenUserAmounts_1
TMP_8935(bool) = REF_3629 == REF_3630
TMP_8936(None) = SOLIDITY_CALL require(bool,string)(TMP_8935,Mismatched refund arrays)
 require(bool,string)(distributeAgentTokenUserAddresses.length == distributeAgentTokenUserAmounts.length,Mismatched distribution arrays)
REF_3631 -> LENGTH distributeAgentTokenUserAddresses_1
REF_3632 -> LENGTH distributeAgentTokenUserAmounts_1
TMP_8937(bool) = REF_3631 == REF_3632
TMP_8938(None) = SOLIDITY_CALL require(bool,string)(TMP_8937,Mismatched distribution arrays)
 totalRefundAmount = 0
totalRefundAmount_1(uint256) := 0(uint256)
 i = 0
i_1(uint256) := 0(uint256)
 i < refundVirtualsTokenUserAmounts.length
totalRefundAmount_2(uint256) := phi(['totalRefundAmount_1', 'totalRefundAmount_3'])
i_2(uint256) := phi(['i_1', 'i_3'])
REF_3633 -> LENGTH refundVirtualsTokenUserAmounts_1
TMP_8939(bool) = i_2 < REF_3633
CONDITION TMP_8939
 require(bool,string)(mapAddrToVirtuals[refundVirtualsTokenUserAddresses[i]] >= refundVirtualsTokenUserAmounts[i],Insufficient Virtual Token committed)
REF_3634(address) -> refundVirtualsTokenUserAddresses_1[i_2]
REF_3635(uint256) -> mapAddrToVirtuals_12[REF_3634]
REF_3636(uint256) -> refundVirtualsTokenUserAmounts_1[i_2]
TMP_8940(bool) = REF_3635 >= REF_3636
TMP_8941(None) = SOLIDITY_CALL require(bool,string)(TMP_8940,Insufficient Virtual Token committed)
 totalRefundAmount += refundVirtualsTokenUserAmounts[i]
REF_3637(uint256) -> refundVirtualsTokenUserAmounts_1[i_2]
totalRefundAmount_3(uint256) = totalRefundAmount_2 (c)+ REF_3637
 i ++
TMP_8942(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 isFirstLaunch = agentTokenAddress == address(0)
TMP_8943 = CONVERT 0 to address
TMP_8944(bool) = agentTokenAddress_6 == TMP_8943
isFirstLaunch_1(bool) := TMP_8944(bool)
 require(bool,string)(IERC20(virtualTokenAddress).balanceOf(address(this)) >= requiredVirtualsBalance,Insufficient Virtual Token balance)
TMP_8945 = CONVERT virtualTokenAddress_12 to IERC20
TMP_8946 = CONVERT this to address
TMP_8947(uint256) = HIGH_LEVEL_CALL, dest:TMP_8945(IERC20), function:balanceOf, arguments:['TMP_8946']  
mapAddrToVirtuals_13(mapping(address => uint256)) := phi(['mapAddrToVirtuals_27', 'mapAddrToVirtuals_12', 'mapAddrToVirtuals_18', 'mapAddrToVirtuals_6', 'mapAddrToVirtuals_17', 'mapAddrToVirtuals_25'])
genesisId_13(uint256) := phi(['genesisId_33', 'genesisId_26', 'genesisId_6', 'genesisId_1', 'genesisId_12', 'genesisId_19', 'genesisId_40', 'genesisId_17'])
genesisName_8(string) := phi(['genesisName_10', 'genesisName_7', 'genesisName_8', 'genesisName_1'])
genesisTicker_8(string) := phi(['genesisTicker_8', 'genesisTicker_1', 'genesisTicker_10', 'genesisTicker_7'])
genesisCores_8(uint8[]) := phi(['genesisCores_1', 'genesisCores_10', 'genesisCores_7', 'genesisCores_8'])
tbaSalt_8(bytes32) := phi(['tbaSalt_10', 'tbaSalt_7', 'tbaSalt_1', 'tbaSalt_8'])
tbaImplementation_8(address) := phi(['tbaImplementation_10', 'tbaImplementation_7', 'tbaImplementation_1', 'tbaImplementation_8'])
daoVotingPeriod_8(uint32) := phi(['daoVotingPeriod_8', 'daoVotingPeriod_1', 'daoVotingPeriod_10', 'daoVotingPeriod_7'])
daoThreshold_8(uint256) := phi(['daoThreshold_1', 'daoThreshold_10', 'daoThreshold_7', 'daoThreshold_8'])
agentFactoryAddress_8(address) := phi(['agentFactoryAddress_1', 'agentFactoryAddress_7', 'agentFactoryAddress_8', 'agentFactoryAddress_11'])
virtualTokenAddress_13(address) := phi(['virtualTokenAddress_17', 'virtualTokenAddress_12', 'virtualTokenAddress_6', 'virtualTokenAddress_24', 'virtualTokenAddress_1'])
reserveAmount_8(uint256) := phi(['reserveAmount_8', 'reserveAmount_10', 'reserveAmount_1', 'reserveAmount_7'])
agentTokenTotalSupply_8(uint256) := phi(['agentTokenTotalSupply_11', 'agentTokenTotalSupply_1', 'agentTokenTotalSupply_7', 'agentTokenTotalSupply_8'])
agentTokenLpSupply_8(uint256) := phi(['agentTokenLpSupply_11', 'agentTokenLpSupply_1', 'agentTokenLpSupply_8', 'agentTokenLpSupply_7'])
agentTokenAddress_7(address) := phi(['agentTokenAddress_6', 'agentTokenAddress_9', 'agentTokenAddress_16', 'agentTokenAddress_11'])
TMP_8948(bool) = TMP_8947 >= requiredVirtualsBalance_3
TMP_8949(None) = SOLIDITY_CALL require(bool,string)(TMP_8948,Insufficient Virtual Token balance)
 isFirstLaunch
CONDITION isFirstLaunch_1
 IERC20(virtualTokenAddress).approve(agentFactoryAddress,reserveAmount)
TMP_8950 = CONVERT virtualTokenAddress_13 to IERC20
TMP_8951(bool) = HIGH_LEVEL_CALL, dest:TMP_8950(IERC20), function:approve, arguments:['agentFactoryAddress_8', 'reserveAmount_8']  
mapAddrToVirtuals_14(mapping(address => uint256)) := phi(['mapAddrToVirtuals_27', 'mapAddrToVirtuals_18', 'mapAddrToVirtuals_6', 'mapAddrToVirtuals_13', 'mapAddrToVirtuals_17', 'mapAddrToVirtuals_25'])
genesisId_14(uint256) := phi(['genesisId_33', 'genesisId_13', 'genesisId_26', 'genesisId_6', 'genesisId_1', 'genesisId_19', 'genesisId_40', 'genesisId_17'])
genesisName_9(string) := phi(['genesisName_10', 'genesisName_8', 'genesisName_1'])
genesisTicker_9(string) := phi(['genesisTicker_8', 'genesisTicker_1', 'genesisTicker_10'])
genesisCores_9(uint8[]) := phi(['genesisCores_1', 'genesisCores_10', 'genesisCores_8'])
tbaSalt_9(bytes32) := phi(['tbaSalt_10', 'tbaSalt_1', 'tbaSalt_8'])
tbaImplementation_9(address) := phi(['tbaImplementation_10', 'tbaImplementation_1', 'tbaImplementation_8'])
daoVotingPeriod_9(uint32) := phi(['daoVotingPeriod_8', 'daoVotingPeriod_1', 'daoVotingPeriod_10'])
daoThreshold_9(uint256) := phi(['daoThreshold_1', 'daoThreshold_10', 'daoThreshold_8'])
agentFactoryAddress_9(address) := phi(['agentFactoryAddress_1', 'agentFactoryAddress_8', 'agentFactoryAddress_11'])
virtualTokenAddress_14(address) := phi(['virtualTokenAddress_17', 'virtualTokenAddress_13', 'virtualTokenAddress_6', 'virtualTokenAddress_24', 'virtualTokenAddress_1'])
reserveAmount_9(uint256) := phi(['reserveAmount_8', 'reserveAmount_10', 'reserveAmount_1'])
agentTokenTotalSupply_9(uint256) := phi(['agentTokenTotalSupply_11', 'agentTokenTotalSupply_1', 'agentTokenTotalSupply_8'])
agentTokenLpSupply_9(uint256) := phi(['agentTokenLpSupply_11', 'agentTokenLpSupply_1', 'agentTokenLpSupply_8'])
 id = IAgentFactoryV3(agentFactoryAddress).initFromBondingCurve(string.concat(genesisName, by Virtuals),genesisTicker,genesisCores,tbaSalt,tbaImplementation,daoVotingPeriod,daoThreshold,reserveAmount,creator)
TMP_8952 = CONVERT agentFactoryAddress_9 to IAgentFactoryV3
TMP_8953(string) = SOLIDITY_CALL string.concat()(genesisName_9, by Virtuals)
TMP_8954(uint256) = HIGH_LEVEL_CALL, dest:TMP_8952(IAgentFactoryV3), function:initFromBondingCurve, arguments:['TMP_8953', 'genesisTicker_9', 'genesisCores_9', 'tbaSalt_9', 'tbaImplementation_9', 'daoVotingPeriod_9', 'daoThreshold_9', 'reserveAmount_9', 'creator_1']  
mapAddrToVirtuals_15(mapping(address => uint256)) := phi(['mapAddrToVirtuals_27', 'mapAddrToVirtuals_18', 'mapAddrToVirtuals_6', 'mapAddrToVirtuals_17', 'mapAddrToVirtuals_14', 'mapAddrToVirtuals_25'])
genesisId_15(uint256) := phi(['genesisId_33', 'genesisId_14', 'genesisId_26', 'genesisId_6', 'genesisId_1', 'genesisId_19', 'genesisId_40', 'genesisId_17'])
genesisName_10(string) := phi(['genesisName_10', 'genesisName_8', 'genesisName_1', 'genesisName_9'])
genesisTicker_10(string) := phi(['genesisTicker_8', 'genesisTicker_1', 'genesisTicker_9', 'genesisTicker_10'])
genesisCores_10(uint8[]) := phi(['genesisCores_1', 'genesisCores_9', 'genesisCores_10', 'genesisCores_8'])
tbaSalt_10(bytes32) := phi(['tbaSalt_9', 'tbaSalt_10', 'tbaSalt_1', 'tbaSalt_8'])
tbaImplementation_10(address) := phi(['tbaImplementation_10', 'tbaImplementation_1', 'tbaImplementation_8', 'tbaImplementation_9'])
daoVotingPeriod_10(uint32) := phi(['daoVotingPeriod_8', 'daoVotingPeriod_1', 'daoVotingPeriod_9', 'daoVotingPeriod_10'])
daoThreshold_10(uint256) := phi(['daoThreshold_1', 'daoThreshold_9', 'daoThreshold_10', 'daoThreshold_8'])
agentFactoryAddress_10(address) := phi(['agentFactoryAddress_1', 'agentFactoryAddress_9', 'agentFactoryAddress_8', 'agentFactoryAddress_11'])
virtualTokenAddress_15(address) := phi(['virtualTokenAddress_17', 'virtualTokenAddress_6', 'virtualTokenAddress_24', 'virtualTokenAddress_14', 'virtualTokenAddress_1'])
reserveAmount_10(uint256) := phi(['reserveAmount_8', 'reserveAmount_10', 'reserveAmount_9', 'reserveAmount_1'])
agentTokenTotalSupply_10(uint256) := phi(['agentTokenTotalSupply_9', 'agentTokenTotalSupply_11', 'agentTokenTotalSupply_1', 'agentTokenTotalSupply_8'])
agentTokenLpSupply_10(uint256) := phi(['agentTokenLpSupply_9', 'agentTokenLpSupply_11', 'agentTokenLpSupply_1', 'agentTokenLpSupply_8'])
id_1(uint256) := TMP_8954(uint256)
 agentToken = IAgentFactoryV3(agentFactoryAddress).executeBondingCurveApplication(id,agentTokenTotalSupply,agentTokenLpSupply,address(this))
TMP_8955 = CONVERT agentFactoryAddress_10 to IAgentFactoryV3
TMP_8956 = CONVERT this to address
TMP_8957(address) = HIGH_LEVEL_CALL, dest:TMP_8955(IAgentFactoryV3), function:executeBondingCurveApplication, arguments:['id_1', 'agentTokenTotalSupply_10', 'agentTokenLpSupply_10', 'TMP_8956']  
mapAddrToVirtuals_16(mapping(address => uint256)) := phi(['mapAddrToVirtuals_15', 'mapAddrToVirtuals_27', 'mapAddrToVirtuals_18', 'mapAddrToVirtuals_6', 'mapAddrToVirtuals_17', 'mapAddrToVirtuals_25'])
genesisId_16(uint256) := phi(['genesisId_33', 'genesisId_15', 'genesisId_26', 'genesisId_6', 'genesisId_1', 'genesisId_19', 'genesisId_40', 'genesisId_17'])
agentFactoryAddress_11(address) := phi(['agentFactoryAddress_1', 'agentFactoryAddress_10', 'agentFactoryAddress_8', 'agentFactoryAddress_11'])
virtualTokenAddress_16(address) := phi(['virtualTokenAddress_15', 'virtualTokenAddress_17', 'virtualTokenAddress_6', 'virtualTokenAddress_24', 'virtualTokenAddress_1'])
agentTokenTotalSupply_11(uint256) := phi(['agentTokenTotalSupply_10', 'agentTokenTotalSupply_11', 'agentTokenTotalSupply_1', 'agentTokenTotalSupply_8'])
agentTokenLpSupply_11(uint256) := phi(['agentTokenLpSupply_11', 'agentTokenLpSupply_10', 'agentTokenLpSupply_1', 'agentTokenLpSupply_8'])
agentToken_1(address) := TMP_8957(address)
 require(bool,string)(agentToken != address(0),Agent token creation failed)
TMP_8958 = CONVERT 0 to address
TMP_8959(bool) = agentToken_1 != TMP_8958
TMP_8960(None) = SOLIDITY_CALL require(bool,string)(TMP_8959,Agent token creation failed)
 agentTokenAddress = agentToken
agentTokenAddress_8(address) := agentToken_1(address)
 totalDistributionAmount = 0
totalDistributionAmount_1(uint256) := 0(uint256)
 i_scope_0 = 0
i_scope_0_1(uint256) := 0(uint256)
 i_scope_0 < distributeAgentTokenUserAmounts.length
totalDistributionAmount_2(uint256) := phi(['totalDistributionAmount_1', 'totalDistributionAmount_3'])
i_scope_0_2(uint256) := phi(['i_scope_0_1', 'i_scope_0_3'])
REF_3643 -> LENGTH distributeAgentTokenUserAmounts_1
TMP_8961(bool) = i_scope_0_2 < REF_3643
CONDITION TMP_8961
 totalDistributionAmount += distributeAgentTokenUserAmounts[i_scope_0]
REF_3644(uint256) -> distributeAgentTokenUserAmounts_1[i_scope_0_2]
totalDistributionAmount_3(uint256) = totalDistributionAmount_2 (c)+ REF_3644
 i_scope_0 ++
TMP_8962(uint256) := i_scope_0_2(uint256)
i_scope_0_3(uint256) = i_scope_0_2 (c)+ 1
 require(bool,string)(IERC20(agentTokenAddress).balanceOf(address(this)) >= totalDistributionAmount,Insufficient Agent Token balance)
TMP_8963 = CONVERT agentTokenAddress_8 to IERC20
TMP_8964 = CONVERT this to address
TMP_8965(uint256) = HIGH_LEVEL_CALL, dest:TMP_8963(IERC20), function:balanceOf, arguments:['TMP_8964']  
mapAddrToVirtuals_17(mapping(address => uint256)) := phi(['mapAddrToVirtuals_27', 'mapAddrToVirtuals_16', 'mapAddrToVirtuals_18', 'mapAddrToVirtuals_6', 'mapAddrToVirtuals_17', 'mapAddrToVirtuals_25'])
genesisId_17(uint256) := phi(['genesisId_33', 'genesisId_26', 'genesisId_6', 'genesisId_16', 'genesisId_1', 'genesisId_19', 'genesisId_40', 'genesisId_17'])
virtualTokenAddress_17(address) := phi(['virtualTokenAddress_16', 'virtualTokenAddress_17', 'virtualTokenAddress_6', 'virtualTokenAddress_24', 'virtualTokenAddress_1'])
agentTokenAddress_9(address) := phi(['agentTokenAddress_8', 'agentTokenAddress_9', 'agentTokenAddress_16', 'agentTokenAddress_11'])
TMP_8966(bool) = TMP_8965 >= totalDistributionAmount_2
TMP_8967(None) = SOLIDITY_CALL require(bool,string)(TMP_8966,Insufficient Agent Token balance)
 i_scope_1 = 0
i_scope_1_1(uint256) := 0(uint256)
 i_scope_1 < refundVirtualsTokenUserAddresses.length
i_scope_1_2(uint256) := phi(['i_scope_1_3', 'i_scope_1_1'])
REF_3646 -> LENGTH refundVirtualsTokenUserAddresses_1
TMP_8968(bool) = i_scope_1_2 < REF_3646
CONDITION TMP_8968
 mapAddrToVirtuals[refundVirtualsTokenUserAddresses[i_scope_1]] -= refundVirtualsTokenUserAmounts[i_scope_1]
REF_3647(address) -> refundVirtualsTokenUserAddresses_1[i_scope_1_2]
REF_3648(uint256) -> mapAddrToVirtuals_17[REF_3647]
REF_3649(uint256) -> refundVirtualsTokenUserAmounts_1[i_scope_1_2]
mapAddrToVirtuals_18(mapping(address => uint256)) := phi(['mapAddrToVirtuals_17'])
REF_3648(-> mapAddrToVirtuals_18) = REF_3648 (c)- REF_3649
 IERC20(virtualTokenAddress).safeTransfer(refundVirtualsTokenUserAddresses[i_scope_1],refundVirtualsTokenUserAmounts[i_scope_1])
TMP_8969 = CONVERT virtualTokenAddress_17 to IERC20
REF_3651(address) -> refundVirtualsTokenUserAddresses_1[i_scope_1_2]
REF_3652(uint256) -> refundVirtualsTokenUserAmounts_1[i_scope_1_2]
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['TMP_8969', 'REF_3651', 'REF_3652'] 
 RefundClaimed(genesisId,refundVirtualsTokenUserAddresses[i_scope_1],refundVirtualsTokenUserAmounts[i_scope_1])
REF_3653(address) -> refundVirtualsTokenUserAddresses_1[i_scope_1_2]
REF_3654(uint256) -> refundVirtualsTokenUserAmounts_1[i_scope_1_2]
Emit RefundClaimed(genesisId_17,REF_3653,REF_3654)
 i_scope_1 ++
TMP_8972(uint256) := i_scope_1_2(uint256)
i_scope_1_3(uint256) = i_scope_1_2 (c)+ 1
 i_scope_2 = 0
i_scope_2_1(uint256) := 0(uint256)
 i_scope_2 < distributeAgentTokenUserAddresses.length
i_scope_2_2(uint256) := phi(['i_scope_2_3', 'i_scope_2_1'])
REF_3655 -> LENGTH distributeAgentTokenUserAddresses_1
TMP_8973(bool) = i_scope_2_2 < REF_3655
CONDITION TMP_8973
 claimableAgentTokens[distributeAgentTokenUserAddresses[i_scope_2]] = distributeAgentTokenUserAmounts[i_scope_2]
REF_3656(address) -> distributeAgentTokenUserAddresses_1[i_scope_2_2]
REF_3657(uint256) -> claimableAgentTokens_0[REF_3656]
REF_3658(uint256) -> distributeAgentTokenUserAmounts_1[i_scope_2_2]
claimableAgentTokens_1(mapping(address => uint256)) := phi(['claimableAgentTokens_0'])
REF_3657(uint256) (->claimableAgentTokens_1) := REF_3658(uint256)
 i_scope_2 ++
TMP_8974(uint256) := i_scope_2_2(uint256)
i_scope_2_3(uint256) = i_scope_2_2 (c)+ 1
 GenesisSucceeded(genesisId)
Emit GenesisSucceeded(genesisId_17)
 agentTokenAddress
RETURN agentTokenAddress_9
 onlyRole(FACTORY_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(FACTORY_ROLE_7)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
 whenNotCancelled()
MODIFIER_CALL, Genesis.whenNotCancelled()()
 whenNotFailed()
MODIFIER_CALL, Genesis.whenNotFailed()()
 whenEnded()
MODIFIER_CALL, Genesis.whenEnded()()
 isFirstLaunch
CONDITION isFirstLaunch_1
 requiredVirtualsBalance = totalRefundAmount + reserveAmount
TMP_8981(uint256) = totalRefundAmount_2 (c)+ reserveAmount_7
requiredVirtualsBalance_1(uint256) := TMP_8981(uint256)
 requiredVirtualsBalance = totalRefundAmount
requiredVirtualsBalance_2(uint256) := totalRefundAmount_2(uint256)
requiredVirtualsBalance_3(uint256) := phi(['requiredVirtualsBalance_1', 'requiredVirtualsBalance_2'])
```
#### Genesis.participate(uint256,uint256) [EXTERNAL]
```slithir
mapAddrToVirtuals_1(mapping(address => uint256)) := phi(['mapAddrToVirtuals_0', 'mapAddrToVirtuals_27', 'mapAddrToVirtuals_18', 'mapAddrToVirtuals_6', 'mapAddrToVirtuals_17', 'mapAddrToVirtuals_25'])
participants_1(address[]) := phi(['participants_15', 'participants_14', 'participants_5', 'participants_0', 'participants_16', 'participants_17', 'participants_7'])
genesisId_2(uint256) := phi(['genesisId_33', 'genesisId_6', 'genesisId_26', 'genesisId_0', 'genesisId_1', 'genesisId_19', 'genesisId_40', 'genesisId_17'])
virtualTokenAddress_2(address) := phi(['virtualTokenAddress_17', 'virtualTokenAddress_6', 'virtualTokenAddress_24', 'virtualTokenAddress_1', 'virtualTokenAddress_0'])
maxContributionVirtualAmount_2(uint256) := phi(['maxContributionVirtualAmount_0', 'maxContributionVirtualAmount_4', 'maxContributionVirtualAmount_1'])
 require(bool,string)(pointAmt > 0,Point amount must be greater than 0)
TMP_8908(bool) = pointAmt_1 > 0
TMP_8909(None) = SOLIDITY_CALL require(bool,string)(TMP_8908,Point amount must be greater than 0)
 require(bool,string)(virtualsAmt > 0,Virtuals must be greater than 0)
TMP_8910(bool) = virtualsAmt_1 > 0
TMP_8911(None) = SOLIDITY_CALL require(bool,string)(TMP_8910,Virtuals must be greater than 0)
 require(bool,string)(virtualsAmt <= maxContributionVirtualAmount,Exceeds maximum virtuals per contribution)
TMP_8912(bool) = virtualsAmt_1 <= maxContributionVirtualAmount_4
TMP_8913(None) = SOLIDITY_CALL require(bool,string)(TMP_8912,Exceeds maximum virtuals per contribution)
 require(bool,string)(IERC20(virtualTokenAddress).balanceOf(msg.sender) >= virtualsAmt,Insufficient Virtual Token balance)
TMP_8914 = CONVERT virtualTokenAddress_4 to IERC20
TMP_8915(uint256) = HIGH_LEVEL_CALL, dest:TMP_8914(IERC20), function:balanceOf, arguments:['msg.sender']  
mapAddrToVirtuals_4(mapping(address => uint256)) := phi(['mapAddrToVirtuals_27', 'mapAddrToVirtuals_18', 'mapAddrToVirtuals_6', 'mapAddrToVirtuals_3', 'mapAddrToVirtuals_17', 'mapAddrToVirtuals_25'])
participants_4(address[]) := phi(['participants_15', 'participants_14', 'participants_5', 'participants_3', 'participants_16', 'participants_17', 'participants_7'])
genesisId_5(uint256) := phi(['genesisId_33', 'genesisId_4', 'genesisId_26', 'genesisId_6', 'genesisId_1', 'genesisId_19', 'genesisId_40', 'genesisId_17'])
virtualTokenAddress_5(address) := phi(['virtualTokenAddress_17', 'virtualTokenAddress_4', 'virtualTokenAddress_6', 'virtualTokenAddress_24', 'virtualTokenAddress_1'])
TMP_8916(bool) = TMP_8915 >= virtualsAmt_1
TMP_8917(None) = SOLIDITY_CALL require(bool,string)(TMP_8916,Insufficient Virtual Token balance)
 require(bool,string)(IERC20(virtualTokenAddress).allowance(msg.sender,address(this)) >= virtualsAmt,Insufficient Virtual Token allowance)
TMP_8918 = CONVERT virtualTokenAddress_5 to IERC20
TMP_8919 = CONVERT this to address
TMP_8920(uint256) = HIGH_LEVEL_CALL, dest:TMP_8918(IERC20), function:allowance, arguments:['msg.sender', 'TMP_8919']  
mapAddrToVirtuals_5(mapping(address => uint256)) := phi(['mapAddrToVirtuals_27', 'mapAddrToVirtuals_4', 'mapAddrToVirtuals_18', 'mapAddrToVirtuals_6', 'mapAddrToVirtuals_17', 'mapAddrToVirtuals_25'])
participants_5(address[]) := phi(['participants_15', 'participants_14', 'participants_5', 'participants_4', 'participants_16', 'participants_17', 'participants_7'])
genesisId_6(uint256) := phi(['genesisId_33', 'genesisId_26', 'genesisId_6', 'genesisId_5', 'genesisId_1', 'genesisId_19', 'genesisId_40', 'genesisId_17'])
virtualTokenAddress_6(address) := phi(['virtualTokenAddress_17', 'virtualTokenAddress_6', 'virtualTokenAddress_5', 'virtualTokenAddress_24', 'virtualTokenAddress_1'])
TMP_8921(bool) = TMP_8920 >= virtualsAmt_1
TMP_8922(None) = SOLIDITY_CALL require(bool,string)(TMP_8921,Insufficient Virtual Token allowance)
 mapAddrToVirtuals[msg.sender] == 0
REF_3623(uint256) -> mapAddrToVirtuals_5[msg.sender]
TMP_8923(bool) = REF_3623 == 0
CONDITION TMP_8923
 participants.push(msg.sender)
REF_3625 -> LENGTH participants_5
TMP_8925(uint256) := REF_3625(uint256)
TMP_8926(uint256) = TMP_8925 (c)+ 1
participants_6(address[]) := phi(['participants_5'])
REF_3625(uint256) (->participants_6) := TMP_8926(uint256)
REF_3626(address) -> participants_6[TMP_8925]
participants_7(address[]) := phi(['participants_6'])
REF_3626(address) (->participants_7) := msg.sender(address)
 mapAddrToVirtuals[msg.sender] += virtualsAmt
REF_3627(uint256) -> mapAddrToVirtuals_5[msg.sender]
mapAddrToVirtuals_6(mapping(address => uint256)) := phi(['mapAddrToVirtuals_5'])
REF_3627(-> mapAddrToVirtuals_6) = REF_3627 (c)+ virtualsAmt_1
 IERC20(virtualTokenAddress).safeTransferFrom(msg.sender,address(this),virtualsAmt)
TMP_8927 = CONVERT virtualTokenAddress_6 to IERC20
TMP_8928 = CONVERT this to address
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransferFrom(IERC20,address,address,uint256), arguments:['TMP_8927', 'msg.sender', 'TMP_8928', 'virtualsAmt_1'] 
 Participated(genesisId,msg.sender,pointAmt,virtualsAmt)
Emit Participated(genesisId_6,msg.sender,pointAmt_1,virtualsAmt_1)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
 whenActive()
MODIFIER_CALL, Genesis.whenActive()()
```
#### Genesis.resetTime(uint256,uint256) [EXTERNAL]
```slithir
FACTORY_ROLE_11(bytes32) := phi(['FACTORY_ROLE_6', 'FACTORY_ROLE_10', 'FACTORY_ROLE_12', 'FACTORY_ROLE_8', 'FACTORY_ROLE_14', 'FACTORY_ROLE_0'])
startTime_4(uint256) := phi(['startTime_12', 'startTime_0', 'startTime_1'])
endTime_4(uint256) := phi(['endTime_0', 'endTime_12', 'endTime_1'])
 _validateTime(newStartTime,newEndTime)
INTERNAL_CALL, Genesis._validateTime(uint256,uint256)(newStartTime_1,newEndTime_1)
 oldStartTime = startTime
oldStartTime_1(uint256) := startTime_11(uint256)
 oldEndTime = endTime
oldEndTime_1(uint256) := endTime_11(uint256)
 startTime = newStartTime
startTime_12(uint256) := newStartTime_1(uint256)
 endTime = newEndTime
endTime_12(uint256) := newEndTime_1(uint256)
 TimeReset(oldStartTime,oldEndTime,newStartTime,newEndTime)
Emit TimeReset(oldStartTime_1,oldEndTime_1,newStartTime_1,newEndTime_1)
 onlyRole(FACTORY_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(FACTORY_ROLE_11)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
 whenNotCancelled()
MODIFIER_CALL, Genesis.whenNotCancelled()()
 whenNotFailed()
MODIFIER_CALL, Genesis.whenNotFailed()()
 whenNotStarted()
MODIFIER_CALL, Genesis.whenNotStarted()()
 whenNotEnded()
MODIFIER_CALL, Genesis.whenNotEnded()()
```
#### FGenesis.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 DEFAULT_ADMIN_ROLE = 0x00
 ADMIN_ROLE = keccak256(bytes)(ADMIN_ROLE)
 OPERATION_ROLE = keccak256(bytes)(OPERATION_ROLE)
role_1(bytes32) := phi(['TMP_8673', 'TMP_8670', 'OPERATION_ROLE_3', 'OPERATION_ROLE_7', 'ADMIN_ROLE_8', 'ADMIN_ROLE_6', 'OPERATION_ROLE_1', 'TMP_8675', 'TMP_8668', 'OPERATION_ROLE_5'])
 _checkRole(role)
INTERNAL_CALL, AccessControlUpgradeable._checkRole(bytes32)(role_1)
 $ = _getInitializableStorage()
TMP_8783(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_8783'])(Initializable.InitializableStorage) := TMP_8783(Initializable.InitializableStorage)
 isTopLevelCall = ! $._initializing
REF_3547(bool) -> $_1 (-> ['TMP_8783'])._initializing
TMP_8784 = UnaryType.BANG REF_3547 
isTopLevelCall_1(bool) := TMP_8784(bool)
 initialized = $._initialized
REF_3548(uint64) -> $_1 (-> ['TMP_8783'])._initialized
initialized_1(uint64) := REF_3548(uint64)
 initialSetup = initialized == 0 && isTopLevelCall
TMP_8785(bool) = initialized_1 == 0
TMP_8786(bool) = TMP_8785 && isTopLevelCall_1
initialSetup_1(bool) := TMP_8786(bool)
 construction = initialized == 1 && address(this).code.length == 0
TMP_8787(bool) = initialized_1 == 1
TMP_8788 = CONVERT this to address
TMP_8789(bytes) = SOLIDITY_CALL code(address)(TMP_8788)
REF_3549 -> LENGTH TMP_8789
TMP_8790(bool) = REF_3549 == 0
TMP_8791(bool) = TMP_8787 && TMP_8790
construction_1(bool) := TMP_8791(bool)
 ! initialSetup && ! construction
TMP_8792 = UnaryType.BANG initialSetup_1 
TMP_8793 = UnaryType.BANG construction_1 
TMP_8794(bool) = TMP_8792 && TMP_8793
CONDITION TMP_8794
 revert InvalidInitialization()()
TMP_8795(None) = SOLIDITY_CALL revert InvalidInitialization()()
 $._initialized = 1
REF_3550(uint64) -> $_1 (-> ['TMP_8783'])._initialized
$_2 (-> ['TMP_8783'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_8783'])"])
REF_3550(uint64) (->$_2 (-> ['TMP_8783'])) := 1(uint256)
TMP_8783(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_8783'])"])
 isTopLevelCall
CONDITION isTopLevelCall_1
 $._initializing = true
REF_3551(bool) -> $_2 (-> ['TMP_8783'])._initializing
$_3 (-> ['TMP_8783'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_8783'])"])
REF_3551(bool) (->$_3 (-> ['TMP_8783'])) := True(bool)
TMP_8783(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_8783'])"])
$_4 (-> ['TMP_8783'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_8783'])", "$_3 (-> ['TMP_8783'])"])
 isTopLevelCall
CONDITION isTopLevelCall_1
 $._initializing = false
REF_3552(bool) -> $_4 (-> ['TMP_8783'])._initializing
$_5 (-> ['TMP_8783'])(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_8783'])"])
REF_3552(bool) (->$_5 (-> ['TMP_8783'])) := False(bool)
TMP_8783(Initializable.InitializableStorage) := phi(["$_5 (-> ['TMP_8783'])"])
 Initialized(1)
Emit Initialized(1)
 $ = _getInitializableStorage()
TMP_8797(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_8797'])(Initializable.InitializableStorage) := TMP_8797(Initializable.InitializableStorage)
 $._initializing || $._initialized >= version
REF_3553(bool) -> $_1 (-> ['TMP_8797'])._initializing
REF_3554(uint64) -> $_1 (-> ['TMP_8797'])._initialized
TMP_8798(bool) = REF_3554 >= version_1
TMP_8799(bool) = REF_3553 || TMP_8798
CONDITION TMP_8799
 revert InvalidInitialization()()
TMP_8800(None) = SOLIDITY_CALL revert InvalidInitialization()()
 $._initialized = version
REF_3555(uint64) -> $_1 (-> ['TMP_8797'])._initialized
$_2 (-> ['TMP_8797'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_8797'])"])
REF_3555(uint64) (->$_2 (-> ['TMP_8797'])) := version_1(uint64)
TMP_8797(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_8797'])"])
 $._initializing = true
REF_3556(bool) -> $_2 (-> ['TMP_8797'])._initializing
$_3 (-> ['TMP_8797'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_8797'])"])
REF_3556(bool) (->$_3 (-> ['TMP_8797'])) := True(bool)
TMP_8797(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_8797'])"])
 $._initializing = false
REF_3557(bool) -> $_3 (-> ['TMP_8797'])._initializing
$_4 (-> ['TMP_8797'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_8797'])"])
REF_3557(bool) (->$_4 (-> ['TMP_8797'])) := False(bool)
TMP_8797(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_8797'])"])
 Initialized(version)
Emit Initialized(version_1)
 _checkInitializing()
INTERNAL_CALL, Initializable._checkInitializing()()
```
#### Genesis.withdrawLeftAssetsAfterFinalized(address,address,uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_6(bytes32) := phi(['DEFAULT_ADMIN_ROLE_7', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_5'])
genesisId_28(uint256) := phi(['genesisId_33', 'genesisId_6', 'genesisId_26', 'genesisId_0', 'genesisId_1', 'genesisId_19', 'genesisId_40', 'genesisId_17'])
 require(bool,string)(token != address(0),Invalid token address)
TMP_9026 = CONVERT 0 to address
TMP_9027(bool) = token_1 != TMP_9026
TMP_9028(None) = SOLIDITY_CALL require(bool,string)(TMP_9027,Invalid token address)
 require(bool,string)(amount <= IERC20(token).balanceOf(address(this)),Insufficient balance to withdraw)
TMP_9029 = CONVERT token_1 to IERC20
TMP_9030 = CONVERT this to address
TMP_9031(uint256) = HIGH_LEVEL_CALL, dest:TMP_9029(IERC20), function:balanceOf, arguments:['TMP_9030']  
genesisId_33(uint256) := phi(['genesisId_33', 'genesisId_32', 'genesisId_26', 'genesisId_6', 'genesisId_1', 'genesisId_19', 'genesisId_40', 'genesisId_17'])
TMP_9032(bool) = amount_1 <= TMP_9031
TMP_9033(None) = SOLIDITY_CALL require(bool,string)(TMP_9032,Insufficient balance to withdraw)
 IERC20(token).safeTransfer(to,amount)
TMP_9034 = CONVERT token_1 to IERC20
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['TMP_9034', 'to_1', 'amount_1'] 
 AssetsWithdrawn(genesisId,to,token,amount)
Emit AssetsWithdrawn(genesisId_33,to_1,token_1,amount_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_6)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
 whenEnded()
MODIFIER_CALL, Genesis.whenEnded()()
 whenFinalized()
MODIFIER_CALL, Genesis.whenFinalized()()
```
#### SafeERC20.safeTransfer(IERC20,address,uint256) [INTERNAL]
```slithir
 _callOptionalReturn(token,abi.encodeCall(token.transfer,(to,value)))
REF_2044(transfer) -> token_1.transfer
TMP_5413(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_2044,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78000550>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78001210>])
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_5413)
```
#### IERC20.approve(address,uint256) [EXTERNAL]
```slithir

```
#### IERC20.balanceOf(address) [EXTERNAL]
```slithir

```

#### IAgentFactoryV3.initFromBondingCurve(string,string,uint8[],bytes32,address,uint32,uint256,uint256,address) [EXTERNAL]
```slithir

```
#### IERC20.allowance(address,address) [EXTERNAL]
```slithir

```
#### SafeERC20.safeTransferFrom(IERC20,address,address,uint256) [INTERNAL]
```slithir
 _callOptionalReturn(token,abi.encodeCall(token.transferFrom,(from,to,value)))
REF_2046(transferFrom) -> token_1.transferFrom
TMP_5415(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_2046,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78000160>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78001150>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78001c90>])
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_5415)
```
#### SafeERC20._callOptionalReturn(IERC20,bytes) [PRIVATE]
```slithir
token_1(IERC20) := phi(['token_1', 'token_1', 'token_1'])
data_1(bytes) := phi(['approvalCall_1', 'TMP_5413', 'TMP_5415', 'TMP_5430'])
 returndata = address(token).functionCall(data)
TMP_5433 = CONVERT token_1 to address
TMP_5434(bytes) = LIBRARY_CALL, dest:Address, function:Address.functionCall(address,bytes), arguments:['TMP_5433', 'data_1'] 
returndata_1(bytes) := TMP_5434(bytes)
 returndata.length != 0 && ! abi.decode(returndata,(bool))
REF_2054 -> LENGTH returndata_1
TMP_5435(bool) = REF_2054 != 0
TMP_5436(bool) = SOLIDITY_CALL abi.decode()(returndata_1,bool)
TMP_5437 = UnaryType.BANG TMP_5436 
TMP_5438(bool) = TMP_5435 && TMP_5437
CONDITION TMP_5438
 revert SafeERC20FailedOperation(address)(address(token))
TMP_5439 = CONVERT token_1 to address
TMP_5440(None) = SOLIDITY_CALL revert SafeERC20FailedOperation(address)(TMP_5439)
```
#### Address.functionCall(address,bytes) [INTERNAL]
```slithir
 functionCallWithValue(target,data,0)
TMP_5457(bytes) = INTERNAL_CALL, Address.functionCallWithValue(address,bytes,uint256)(target_1,data_1,0)
RETURN TMP_5457
```
