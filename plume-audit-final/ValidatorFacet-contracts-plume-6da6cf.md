




#### ValidatorFacet.addValidator(uint16,uint256,address,address,string,string,address,uint256) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13715(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13715'])(PlumeStakingStorage.Layout) := TMP_13715(PlumeStakingStorage.Layout)
 $.validatorExists[validatorId]
REF_3822(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13715']).validatorExists
REF_3823(bool) -> REF_3822[validatorId_1]
CONDITION REF_3823
 revert ValidatorAlreadyExists(uint16)(validatorId)
TMP_13716(None) = SOLIDITY_CALL revert ValidatorAlreadyExists(uint16)(validatorId_1)
 l2AdminAddress == address(0)
TMP_13717 = CONVERT 0 to address
TMP_13718(bool) = l2AdminAddress_1 == TMP_13717
CONDITION TMP_13718
 revert ZeroAddress(string)(l2AdminAddress)
TMP_13719(None) = SOLIDITY_CALL revert ZeroAddress(string)(l2AdminAddress)
 l2WithdrawAddress == address(0)
TMP_13720 = CONVERT 0 to address
TMP_13721(bool) = l2WithdrawAddress_1 == TMP_13720
CONDITION TMP_13721
 revert ZeroAddress(string)(l2WithdrawAddress)
TMP_13722(None) = SOLIDITY_CALL revert ZeroAddress(string)(l2WithdrawAddress)
 commission > $.maxAllowedValidatorCommission
REF_3824(uint256) -> $_1 (-> ['TMP_13715']).maxAllowedValidatorCommission
TMP_13723(bool) = commission_1 > REF_3824
CONDITION TMP_13723
 revert CommissionExceedsMaxAllowed(uint256,uint256)(commission,$.maxAllowedValidatorCommission)
REF_3825(uint256) -> $_1 (-> ['TMP_13715']).maxAllowedValidatorCommission
TMP_13724(None) = SOLIDITY_CALL revert CommissionExceedsMaxAllowed(uint256,uint256)(commission_1,REF_3825)
 $.isAdminAssigned[l2AdminAddress]
REF_3826(mapping(address => bool)) -> $_1 (-> ['TMP_13715']).isAdminAssigned
REF_3827(bool) -> REF_3826[l2AdminAddress_1]
CONDITION REF_3827
 revert AdminAlreadyAssigned(address)(l2AdminAddress)
TMP_13725(None) = SOLIDITY_CALL revert AdminAlreadyAssigned(address)(l2AdminAddress_1)
 validator = $.validators[validatorId]
REF_3828(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13715']).validators
REF_3829(PlumeStakingStorage.ValidatorInfo) -> REF_3828[validatorId_1]
validator_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_3829(PlumeStakingStorage.ValidatorInfo)
 validator.validatorId = validatorId
REF_3830(uint16) -> validator_1 (-> ['$']).validatorId
validator_2 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_1 (-> ['$'])"])
REF_3830(uint16) (->validator_2 (-> ['$'])) := validatorId_1(uint16)
$_9 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_2 (-> ['$'])"])
 validator.commission = commission
REF_3831(uint256) -> validator_2 (-> ['$']).commission
validator_3 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_2 (-> ['$'])"])
REF_3831(uint256) (->validator_3 (-> ['$'])) := commission_1(uint256)
$_10 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_3 (-> ['$'])"])
 validator.delegatedAmount = 0
REF_3832(uint256) -> validator_3 (-> ['$']).delegatedAmount
validator_4 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_3 (-> ['$'])"])
REF_3832(uint256) (->validator_4 (-> ['$'])) := 0(uint256)
$_11 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_4 (-> ['$'])"])
 validator.l2AdminAddress = l2AdminAddress
REF_3833(address) -> validator_4 (-> ['$']).l2AdminAddress
validator_5 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_4 (-> ['$'])"])
REF_3833(address) (->validator_5 (-> ['$'])) := l2AdminAddress_1(address)
$_12 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_5 (-> ['$'])"])
 validator.l2WithdrawAddress = l2WithdrawAddress
REF_3834(address) -> validator_5 (-> ['$']).l2WithdrawAddress
validator_6 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_5 (-> ['$'])"])
REF_3834(address) (->validator_6 (-> ['$'])) := l2WithdrawAddress_1(address)
$_13 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_6 (-> ['$'])"])
 validator.l1ValidatorAddress = l1ValidatorAddress
REF_3835(string) -> validator_6 (-> ['$']).l1ValidatorAddress
validator_7 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_6 (-> ['$'])"])
REF_3835(string) (->validator_7 (-> ['$'])) := l1ValidatorAddress_1(string)
$_14 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_7 (-> ['$'])"])
 validator.l1AccountAddress = l1AccountAddress
REF_3836(string) -> validator_7 (-> ['$']).l1AccountAddress
validator_8 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_7 (-> ['$'])"])
REF_3836(string) (->validator_8 (-> ['$'])) := l1AccountAddress_1(string)
$_15 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_8 (-> ['$'])"])
 validator.l1AccountEvmAddress = l1AccountEvmAddress
REF_3837(address) -> validator_8 (-> ['$']).l1AccountEvmAddress
validator_9 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_8 (-> ['$'])"])
REF_3837(address) (->validator_9 (-> ['$'])) := l1AccountEvmAddress_1(address)
$_16 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_9 (-> ['$'])"])
 validator.active = true
REF_3838(bool) -> validator_9 (-> ['$']).active
validator_10 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_9 (-> ['$'])"])
REF_3838(bool) (->validator_10 (-> ['$'])) := True(bool)
$_17 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_10 (-> ['$'])"])
 validator.slashed = false
REF_3839(bool) -> validator_10 (-> ['$']).slashed
validator_11 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_10 (-> ['$'])"])
REF_3839(bool) (->validator_11 (-> ['$'])) := False(bool)
$_18 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_11 (-> ['$'])"])
 validator.maxCapacity = maxCapacity
REF_3840(uint256) -> validator_11 (-> ['$']).maxCapacity
validator_12 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_11 (-> ['$'])"])
REF_3840(uint256) (->validator_12 (-> ['$'])) := maxCapacity_1(uint256)
$_19 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_12 (-> ['$'])"])
 $.validatorIds.push(validatorId)
REF_3841(uint16[]) -> $_1 (-> ['TMP_13715']).validatorIds
REF_3843 -> LENGTH REF_3841
TMP_13727(uint256) := REF_3843(uint256)
TMP_13728(uint256) = TMP_13727 (c)+ 1
$_2 (-> ['TMP_13715'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13715'])"])
REF_3843(uint256) (->$_3 (-> ['TMP_13715'])) := TMP_13728(uint256)
REF_3844(uint16) -> REF_3841[TMP_13727]
$_3 (-> ['TMP_13715'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13715'])"])
REF_3844(uint16) (->$_3 (-> ['TMP_13715'])) := validatorId_1(uint16)
TMP_13715(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13715'])"])
 $.validatorExists[validatorId] = true
REF_3845(mapping(uint16 => bool)) -> $_3 (-> ['TMP_13715']).validatorExists
REF_3846(bool) -> REF_3845[validatorId_1]
$_4 (-> ['TMP_13715'])(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13715'])"])
REF_3846(bool) (->$_4 (-> ['TMP_13715'])) := True(bool)
TMP_13715(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13715'])"])
 $.adminToValidatorId[l2AdminAddress] = validatorId
REF_3847(mapping(address => uint16)) -> $_4 (-> ['TMP_13715']).adminToValidatorId
REF_3848(uint16) -> REF_3847[l2AdminAddress_1]
$_5 (-> ['TMP_13715'])(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13715'])"])
REF_3848(uint16) (->$_5 (-> ['TMP_13715'])) := validatorId_1(uint16)
TMP_13715(PlumeStakingStorage.Layout) := phi(["$_5 (-> ['TMP_13715'])"])
 $.isAdminAssigned[l2AdminAddress] = true
REF_3849(mapping(address => bool)) -> $_5 (-> ['TMP_13715']).isAdminAssigned
REF_3850(bool) -> REF_3849[l2AdminAddress_1]
$_6 (-> ['TMP_13715'])(PlumeStakingStorage.Layout) := phi(["$_5 (-> ['TMP_13715'])"])
REF_3850(bool) (->$_6 (-> ['TMP_13715'])) := True(bool)
TMP_13715(PlumeStakingStorage.Layout) := phi(["$_6 (-> ['TMP_13715'])"])
 rewardTokens = $.rewardTokens
REF_3851(address[]) -> $_6 (-> ['TMP_13715']).rewardTokens
rewardTokens_1(address[]) = ['REF_3851(address[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < rewardTokens.length
$_7 (-> ['TMP_13715'])(PlumeStakingStorage.Layout) := phi(["$_6 (-> ['TMP_13715'])", "$_8 (-> ['TMP_13715'])"])
i_2(uint256) := phi(['i_1', 'i_3'])
REF_3852 -> LENGTH rewardTokens_1
TMP_13729(bool) = i_2 < REF_3852
CONDITION TMP_13729
 token = rewardTokens[i]
REF_3853(address) -> rewardTokens_1[i_2]
token_1(address) := REF_3853(address)
 $.validatorLastUpdateTimes[validatorId][token] = block.timestamp
REF_3854(mapping(uint16 => mapping(address => uint256))) -> $_7 (-> ['TMP_13715']).validatorLastUpdateTimes
REF_3855(mapping(address => uint256)) -> REF_3854[validatorId_1]
REF_3856(uint256) -> REF_3855[token_1]
$_8 (-> ['TMP_13715'])(PlumeStakingStorage.Layout) := phi(["$_7 (-> ['TMP_13715'])"])
REF_3856(uint256) (->$_8 (-> ['TMP_13715'])) := block.timestamp(uint256)
TMP_13715(PlumeStakingStorage.Layout) := phi(["$_8 (-> ['TMP_13715'])"])
 currentGlobalRate = $.rewardRates[token]
REF_3857(mapping(address => uint256)) -> $_8 (-> ['TMP_13715']).rewardRates
REF_3858(uint256) -> REF_3857[token_1]
currentGlobalRate_1(uint256) := REF_3858(uint256)
 PlumeRewardLogic.createRewardRateCheckpoint($,token,validatorId,currentGlobalRate)
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.createRewardRateCheckpoint(PlumeStakingStorage.Layout,address,uint16,uint256), arguments:["$_8 (-> ['TMP_13715'])", 'token_1', 'validatorId_1', 'currentGlobalRate_1'] 
 i ++
TMP_13731(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 ValidatorAdded(validatorId,commission,l2AdminAddress,l2WithdrawAddress,l1ValidatorAddress,l1AccountAddress,l1AccountEvmAddress)
Emit ValidatorAdded(validatorId_1,commission_1,l2AdminAddress_1,l2WithdrawAddress_1,l1ValidatorAddress_1,l1AccountAddress_1,l1AccountEvmAddress_1)
 onlyRole(PlumeRoles.VALIDATOR_ROLE)
REF_3860(bytes32) -> PlumeRoles.VALIDATOR_ROLE
MODIFIER_CALL, ValidatorFacet.onlyRole(bytes32)(REF_3860)
 onlyRole(PlumeRoles.VALIDATOR_ROLE)
REF_3861(bytes32) -> PlumeRoles.VALIDATOR_ROLE
MODIFIER_CALL, ValidatorFacet.onlyRole(bytes32)(REF_3861)
```
#### ValidatorFacet.setValidatorCapacity(uint16,uint256) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13735(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13735'])(PlumeStakingStorage.Layout) := TMP_13735(PlumeStakingStorage.Layout)
 validator = $.validators[validatorId]
REF_3863(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13735']).validators
REF_3864(PlumeStakingStorage.ValidatorInfo) -> REF_3863[validatorId_1]
validator_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_3864(PlumeStakingStorage.ValidatorInfo)
 ! validator.active || validator.slashed
REF_3865(bool) -> validator_1 (-> ['$']).active
TMP_13736 = UnaryType.BANG REF_3865 
REF_3866(bool) -> validator_1 (-> ['$']).slashed
TMP_13737(bool) = TMP_13736 || REF_3866
CONDITION TMP_13737
 revert ValidatorInactive(uint16)(validatorId)
TMP_13738(None) = SOLIDITY_CALL revert ValidatorInactive(uint16)(validatorId_1)
 oldCapacity = validator.maxCapacity
REF_3867(uint256) -> validator_1 (-> ['$']).maxCapacity
oldCapacity_1(uint256) := REF_3867(uint256)
 validator.maxCapacity = maxCapacity
REF_3868(uint256) -> validator_1 (-> ['$']).maxCapacity
validator_2 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_1 (-> ['$'])"])
REF_3868(uint256) (->validator_2 (-> ['$'])) := maxCapacity_1(uint256)
$_2 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_2 (-> ['$'])"])
 ValidatorCapacityUpdated(validatorId,oldCapacity,maxCapacity)
Emit ValidatorCapacityUpdated(validatorId_1,oldCapacity_1,maxCapacity_1)
 onlyRole(PlumeRoles.VALIDATOR_ROLE)
REF_3869(bytes32) -> PlumeRoles.VALIDATOR_ROLE
MODIFIER_CALL, ValidatorFacet.onlyRole(bytes32)(REF_3869)
 onlyRole(PlumeRoles.VALIDATOR_ROLE)
REF_3870(bytes32) -> PlumeRoles.VALIDATOR_ROLE
MODIFIER_CALL, ValidatorFacet.onlyRole(bytes32)(REF_3870)
 _validateValidatorExists(validatorId)
MODIFIER_CALL, ValidatorFacet._validateValidatorExists(uint16)(validatorId_1)
```
#### ValidatorFacet.setValidatorStatus(uint16,bool) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13743(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13743'])(PlumeStakingStorage.Layout) := TMP_13743(PlumeStakingStorage.Layout)
 validator = $.validators[validatorId]
REF_3872(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13743']).validators
REF_3873(PlumeStakingStorage.ValidatorInfo) -> REF_3872[validatorId_1]
validator_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_3873(PlumeStakingStorage.ValidatorInfo)
 newActiveStatus && validator.slashed
REF_3874(bool) -> validator_1 (-> ['$']).slashed
TMP_13744(bool) = newActiveStatus_1 && REF_3874
CONDITION TMP_13744
 revert ValidatorAlreadySlashed(uint16)(validatorId)
TMP_13745(None) = SOLIDITY_CALL revert ValidatorAlreadySlashed(uint16)(validatorId_1)
 currentStatus = validator.active
REF_3875(bool) -> validator_1 (-> ['$']).active
currentStatus_1(bool) := REF_3875(bool)
 currentStatus != newActiveStatus
TMP_13746(bool) = currentStatus_1 != newActiveStatus_1
CONDITION TMP_13746
 rewardTokens = $.rewardTokens
REF_3876(address[]) -> $_1 (-> ['TMP_13743']).rewardTokens
rewardTokens_1(address[]) = ['REF_3876(address[])']
 ! newActiveStatus && currentStatus
TMP_13747 = UnaryType.BANG newActiveStatus_1 
TMP_13748(bool) = TMP_13747 && currentStatus_1
CONDITION TMP_13748
 PlumeRewardLogic._settleCommissionForValidatorUpToNow($,validatorId)
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic._settleCommissionForValidatorUpToNow(PlumeStakingStorage.Layout,uint16), arguments:["$_1 (-> ['TMP_13743'])", 'validatorId_1'] 
 validator.slashedAtTimestamp = block.timestamp
REF_3878(uint256) -> validator_1 (-> ['$']).slashedAtTimestamp
validator_2 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_1 (-> ['$'])"])
REF_3878(uint256) (->validator_2 (-> ['$'])) := block.timestamp(uint256)
$_6 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_2 (-> ['$'])"])
 i = 0
i_1(uint256) := 0(uint256)
 i < rewardTokens.length
i_2(uint256) := phi(['i_3', 'i_1'])
REF_3879 -> LENGTH rewardTokens_1
TMP_13750(bool) = i_2 < REF_3879
CONDITION TMP_13750
 PlumeRewardLogic.createRewardRateCheckpoint($,rewardTokens[i],validatorId,0)
REF_3881(address) -> rewardTokens_1[i_2]
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.createRewardRateCheckpoint(PlumeStakingStorage.Layout,address,uint16,uint256), arguments:["$_1 (-> ['TMP_13743'])", 'REF_3881', 'validatorId_1', '0'] 
 i ++
TMP_13752(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
validator_3 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_1 (-> ['$'])", "validator_2 (-> ['$'])"])
 validator.active = newActiveStatus
REF_3882(bool) -> validator_3 (-> ['$']).active
validator_4 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_3 (-> ['$'])"])
REF_3882(bool) (->validator_4 (-> ['$'])) := newActiveStatus_1(bool)
$_4 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_4 (-> ['$'])"])
 newActiveStatus && ! currentStatus
TMP_13753 = UnaryType.BANG currentStatus_1 
TMP_13754(bool) = newActiveStatus_1 && TMP_13753
CONDITION TMP_13754
 i_scope_0 = 0
i_scope_0_1(uint256) := 0(uint256)
 i_scope_0 < rewardTokens.length
$_2 (-> ['TMP_13743'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13743'])", "$_3 (-> ['TMP_13743'])"])
i_scope_0_2(uint256) := phi(['i_scope_0_1', 'i_scope_0_3'])
REF_3883 -> LENGTH rewardTokens_1
TMP_13755(bool) = i_scope_0_2 < REF_3883
CONDITION TMP_13755
 token = rewardTokens[i_scope_0]
REF_3884(address) -> rewardTokens_1[i_scope_0_2]
token_1(address) := REF_3884(address)
 $.validatorLastUpdateTimes[validatorId][token] = block.timestamp
REF_3885(mapping(uint16 => mapping(address => uint256))) -> $_2 (-> ['TMP_13743']).validatorLastUpdateTimes
REF_3886(mapping(address => uint256)) -> REF_3885[validatorId_1]
REF_3887(uint256) -> REF_3886[token_1]
$_3 (-> ['TMP_13743'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13743'])"])
REF_3887(uint256) (->$_3 (-> ['TMP_13743'])) := block.timestamp(uint256)
TMP_13743(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13743'])"])
 currentGlobalRate = $.rewardRates[token]
REF_3888(mapping(address => uint256)) -> $_3 (-> ['TMP_13743']).rewardRates
REF_3889(uint256) -> REF_3888[token_1]
currentGlobalRate_1(uint256) := REF_3889(uint256)
 PlumeRewardLogic.createRewardRateCheckpoint($,token,validatorId,currentGlobalRate)
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.createRewardRateCheckpoint(PlumeStakingStorage.Layout,address,uint16,uint256), arguments:["$_3 (-> ['TMP_13743'])", 'token_1', 'validatorId_1', 'currentGlobalRate_1'] 
 i_scope_0 ++
TMP_13757(uint256) := i_scope_0_2(uint256)
i_scope_0_3(uint256) = i_scope_0_2 (c)+ 1
 ! validator.slashed
REF_3891(bool) -> validator_4 (-> ['$']).slashed
TMP_13758 = UnaryType.BANG REF_3891 
CONDITION TMP_13758
 validator.slashedAtTimestamp = 0
REF_3892(uint256) -> validator_4 (-> ['$']).slashedAtTimestamp
validator_5 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_4 (-> ['$'])"])
REF_3892(uint256) (->validator_5 (-> ['$'])) := 0(uint256)
$_5 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_5 (-> ['$'])"])
validator_6 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_4 (-> ['$'])", "validator_5 (-> ['$'])"])
validator_7 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_4 (-> ['$'])", "validator_1 (-> ['$'])"])
 ValidatorStatusUpdated(validatorId,newActiveStatus,validator.slashed)
REF_3893(bool) -> validator_7 (-> ['$']).slashed
Emit ValidatorStatusUpdated(validatorId_1,newActiveStatus_1,REF_3893)
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_3894(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ValidatorFacet.onlyRole(bytes32)(REF_3894)
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_3895(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ValidatorFacet.onlyRole(bytes32)(REF_3895)
 _validateValidatorExists(validatorId)
MODIFIER_CALL, ValidatorFacet._validateValidatorExists(uint16)(validatorId_1)
```
#### ValidatorFacet.setValidatorCommission(uint16,uint256) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13763(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13763'])(PlumeStakingStorage.Layout) := TMP_13763(PlumeStakingStorage.Layout)
 validator = $.validators[validatorId]
REF_3897(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13763']).validators
REF_3898(PlumeStakingStorage.ValidatorInfo) -> REF_3897[validatorId_1]
validator_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_3898(PlumeStakingStorage.ValidatorInfo)
 ! validator.active || validator.slashed
REF_3899(bool) -> validator_1 (-> ['$']).active
TMP_13764 = UnaryType.BANG REF_3899 
REF_3900(bool) -> validator_1 (-> ['$']).slashed
TMP_13765(bool) = TMP_13764 || REF_3900
CONDITION TMP_13765
 revert ValidatorInactive(uint16)(validatorId)
TMP_13766(None) = SOLIDITY_CALL revert ValidatorInactive(uint16)(validatorId_1)
 newCommission > $.maxAllowedValidatorCommission
REF_3901(uint256) -> $_1 (-> ['TMP_13763']).maxAllowedValidatorCommission
TMP_13767(bool) = newCommission_1 > REF_3901
CONDITION TMP_13767
 revert CommissionExceedsMaxAllowed(uint256,uint256)(newCommission,$.maxAllowedValidatorCommission)
REF_3902(uint256) -> $_1 (-> ['TMP_13763']).maxAllowedValidatorCommission
TMP_13768(None) = SOLIDITY_CALL revert CommissionExceedsMaxAllowed(uint256,uint256)(newCommission_1,REF_3902)
 oldCommission = validator.commission
REF_3903(uint256) -> validator_1 (-> ['$']).commission
oldCommission_1(uint256) := REF_3903(uint256)
 oldCommission == newCommission
TMP_13769(bool) = oldCommission_1 == newCommission_1
CONDITION TMP_13769
 PlumeRewardLogic._settleCommissionForValidatorUpToNow($,validatorId)
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic._settleCommissionForValidatorUpToNow(PlumeStakingStorage.Layout,uint16), arguments:["$_1 (-> ['TMP_13763'])", 'validatorId_1'] 
 validator.commission = newCommission
REF_3905(uint256) -> validator_1 (-> ['$']).commission
validator_2 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_1 (-> ['$'])"])
REF_3905(uint256) (->validator_2 (-> ['$'])) := newCommission_1(uint256)
$_2 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_2 (-> ['$'])"])
 PlumeRewardLogic.createCommissionRateCheckpoint($,validatorId,newCommission)
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.createCommissionRateCheckpoint(PlumeStakingStorage.Layout,uint16,uint256), arguments:["$_1 (-> ['TMP_13763'])", 'validatorId_1', 'newCommission_1'] 
 ValidatorCommissionSet(validatorId,oldCommission,newCommission)
Emit ValidatorCommissionSet(validatorId_1,oldCommission_1,newCommission_1)
 onlyValidatorAdmin(validatorId)
MODIFIER_CALL, ValidatorFacet.onlyValidatorAdmin(uint16)(validatorId_1)
```
#### ValidatorFacet.setValidatorAddresses(uint16,address,address,string,string,address) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13774(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13774'])(PlumeStakingStorage.Layout) := TMP_13774(PlumeStakingStorage.Layout)
 validator = $.validators[validatorId]
REF_3908(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13774']).validators
REF_3909(PlumeStakingStorage.ValidatorInfo) -> REF_3908[validatorId_1]
validator_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_3909(PlumeStakingStorage.ValidatorInfo)
 ! validator.active || validator.slashed
REF_3910(bool) -> validator_1 (-> ['$']).active
TMP_13775 = UnaryType.BANG REF_3910 
REF_3911(bool) -> validator_1 (-> ['$']).slashed
TMP_13776(bool) = TMP_13775 || REF_3911
CONDITION TMP_13776
 revert ValidatorInactive(uint16)(validatorId)
TMP_13777(None) = SOLIDITY_CALL revert ValidatorInactive(uint16)(validatorId_1)
 oldL2AdminAddress = validator.l2AdminAddress
REF_3912(address) -> validator_1 (-> ['$']).l2AdminAddress
oldL2AdminAddress_1(address) := REF_3912(address)
 oldL2WithdrawAddress = validator.l2WithdrawAddress
REF_3913(address) -> validator_1 (-> ['$']).l2WithdrawAddress
oldL2WithdrawAddress_1(address) := REF_3913(address)
 oldL1ValidatorAddress = validator.l1ValidatorAddress
REF_3914(string) -> validator_1 (-> ['$']).l1ValidatorAddress
oldL1ValidatorAddress_1(string) := REF_3914(string)
 oldL1AccountAddress = validator.l1AccountAddress
REF_3915(string) -> validator_1 (-> ['$']).l1AccountAddress
oldL1AccountAddress_1(string) := REF_3915(string)
 oldL1AccountEvmAddress = validator.l1AccountEvmAddress
REF_3916(address) -> validator_1 (-> ['$']).l1AccountEvmAddress
oldL1AccountEvmAddress_1(address) := REF_3916(address)
 newL2AdminAddress != address(0) && newL2AdminAddress != validator.l2AdminAddress
TMP_13778 = CONVERT 0 to address
TMP_13779(bool) = newL2AdminAddress_1 != TMP_13778
REF_3917(address) -> validator_1 (-> ['$']).l2AdminAddress
TMP_13780(bool) = newL2AdminAddress_1 != REF_3917
TMP_13781(bool) = TMP_13779 && TMP_13780
CONDITION TMP_13781
 $.pendingAdmins[validatorId] = newL2AdminAddress
REF_3918(mapping(uint16 => address)) -> $_1 (-> ['TMP_13774']).pendingAdmins
REF_3919(address) -> REF_3918[validatorId_1]
$_2 (-> ['TMP_13774'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13774'])"])
REF_3919(address) (->$_2 (-> ['TMP_13774'])) := newL2AdminAddress_1(address)
TMP_13774(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13774'])"])
 AdminProposed(validatorId,newL2AdminAddress)
Emit AdminProposed(validatorId_1,newL2AdminAddress_1)
 newL2WithdrawAddress != address(0) && newL2WithdrawAddress != validator.l2WithdrawAddress
TMP_13783 = CONVERT 0 to address
TMP_13784(bool) = newL2WithdrawAddress_1 != TMP_13783
REF_3920(address) -> validator_1 (-> ['$']).l2WithdrawAddress
TMP_13785(bool) = newL2WithdrawAddress_1 != REF_3920
TMP_13786(bool) = TMP_13784 && TMP_13785
CONDITION TMP_13786
 newL2WithdrawAddress == address(0)
TMP_13787 = CONVERT 0 to address
TMP_13788(bool) = newL2WithdrawAddress_1 == TMP_13787
CONDITION TMP_13788
 revert ZeroAddress(string)(newL2WithdrawAddress)
TMP_13789(None) = SOLIDITY_CALL revert ZeroAddress(string)(newL2WithdrawAddress)
 validator.l2WithdrawAddress = newL2WithdrawAddress
REF_3921(address) -> validator_1 (-> ['$']).l2WithdrawAddress
validator_2 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_1 (-> ['$'])"])
REF_3921(address) (->validator_2 (-> ['$'])) := newL2WithdrawAddress_1(address)
$_6 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_2 (-> ['$'])"])
validator_3 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_1 (-> ['$'])", "validator_2 (-> ['$'])"])
 bytes(newL1ValidatorAddress).length > 0
TMP_13790 = CONVERT newL1ValidatorAddress_1 to bytes
REF_3922 -> LENGTH TMP_13790
TMP_13791(bool) = REF_3922 > 0
CONDITION TMP_13791
 validator.l1ValidatorAddress = newL1ValidatorAddress
REF_3923(string) -> validator_3 (-> ['$']).l1ValidatorAddress
validator_4 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_3 (-> ['$'])"])
REF_3923(string) (->validator_4 (-> ['$'])) := newL1ValidatorAddress_1(string)
$_5 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_4 (-> ['$'])"])
validator_5 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_1 (-> ['$'])", "validator_4 (-> ['$'])"])
 bytes(newL1AccountAddress).length > 0
TMP_13792 = CONVERT newL1AccountAddress_1 to bytes
REF_3924 -> LENGTH TMP_13792
TMP_13793(bool) = REF_3924 > 0
CONDITION TMP_13793
 validator.l1AccountAddress = newL1AccountAddress
REF_3925(string) -> validator_5 (-> ['$']).l1AccountAddress
validator_6 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_5 (-> ['$'])"])
REF_3925(string) (->validator_6 (-> ['$'])) := newL1AccountAddress_1(string)
$_4 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_6 (-> ['$'])"])
validator_7 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_1 (-> ['$'])", "validator_6 (-> ['$'])"])
 newL1AccountEvmAddress != address(0)
TMP_13794 = CONVERT 0 to address
TMP_13795(bool) = newL1AccountEvmAddress_1 != TMP_13794
CONDITION TMP_13795
 validator.l1AccountEvmAddress = newL1AccountEvmAddress
REF_3926(address) -> validator_7 (-> ['$']).l1AccountEvmAddress
validator_8 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_7 (-> ['$'])"])
REF_3926(address) (->validator_8 (-> ['$'])) := newL1AccountEvmAddress_1(address)
$_3 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_8 (-> ['$'])"])
validator_9 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_1 (-> ['$'])", "validator_8 (-> ['$'])"])
 ValidatorAddressesSet(validatorId,oldL2AdminAddress,validator.l2AdminAddress,oldL2WithdrawAddress,validator.l2WithdrawAddress,oldL1ValidatorAddress,validator.l1ValidatorAddress,oldL1AccountAddress,validator.l1AccountAddress,oldL1AccountEvmAddress,validator.l1AccountEvmAddress)
REF_3927(address) -> validator_9 (-> ['$']).l2AdminAddress
REF_3928(address) -> validator_9 (-> ['$']).l2WithdrawAddress
REF_3929(string) -> validator_9 (-> ['$']).l1ValidatorAddress
REF_3930(string) -> validator_9 (-> ['$']).l1AccountAddress
REF_3931(address) -> validator_9 (-> ['$']).l1AccountEvmAddress
Emit ValidatorAddressesSet(validatorId_1,oldL2AdminAddress_1,REF_3927,oldL2WithdrawAddress_1,REF_3928,oldL1ValidatorAddress_1,REF_3929,oldL1AccountAddress_1,REF_3930,oldL1AccountEvmAddress_1,REF_3931)
 onlyValidatorAdmin(validatorId)
MODIFIER_CALL, ValidatorFacet.onlyValidatorAdmin(uint16)(validatorId_1)
```
#### ValidatorFacet.acceptAdmin(uint16) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13798(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13798'])(PlumeStakingStorage.Layout) := TMP_13798(PlumeStakingStorage.Layout)
 newAdmin = msg.sender
newAdmin_1(address) := msg.sender(address)
 pendingAdmin = $.pendingAdmins[validatorId]
REF_3933(mapping(uint16 => address)) -> $_1 (-> ['TMP_13798']).pendingAdmins
REF_3934(address) -> REF_3933[validatorId_1]
pendingAdmin_1(address) := REF_3934(address)
 pendingAdmin == address(0)
TMP_13799 = CONVERT 0 to address
TMP_13800(bool) = pendingAdmin_1 == TMP_13799
CONDITION TMP_13800
 revert NoPendingAdmin(uint16)(validatorId)
TMP_13801(None) = SOLIDITY_CALL revert NoPendingAdmin(uint16)(validatorId_1)
 newAdmin != pendingAdmin
TMP_13802(bool) = newAdmin_1 != pendingAdmin_1
CONDITION TMP_13802
 revert NotPendingAdmin(address,uint16)(newAdmin,validatorId)
TMP_13803(None) = SOLIDITY_CALL revert NotPendingAdmin(address,uint16)(newAdmin_1,validatorId_1)
 $.isAdminAssigned[newAdmin]
REF_3935(mapping(address => bool)) -> $_1 (-> ['TMP_13798']).isAdminAssigned
REF_3936(bool) -> REF_3935[newAdmin_1]
CONDITION REF_3936
 revert AdminAlreadyAssigned(address)(newAdmin)
TMP_13804(None) = SOLIDITY_CALL revert AdminAlreadyAssigned(address)(newAdmin_1)
 validator = $.validators[validatorId]
REF_3937(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13798']).validators
REF_3938(PlumeStakingStorage.ValidatorInfo) -> REF_3937[validatorId_1]
validator_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_3938(PlumeStakingStorage.ValidatorInfo)
 oldAdmin = validator.l2AdminAddress
REF_3939(address) -> validator_1 (-> ['$']).l2AdminAddress
oldAdmin_1(address) := REF_3939(address)
 validator.l2AdminAddress = newAdmin
REF_3940(address) -> validator_1 (-> ['$']).l2AdminAddress
validator_2 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_1 (-> ['$'])"])
REF_3940(address) (->validator_2 (-> ['$'])) := newAdmin_1(address)
$_5 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_2 (-> ['$'])"])
 delete $.adminToValidatorId[oldAdmin]
REF_3941(mapping(address => uint16)) -> $_1 (-> ['TMP_13798']).adminToValidatorId
REF_3942(uint16) -> REF_3941[oldAdmin_1]
REF_3941 = delete REF_3942 
 $.adminToValidatorId[newAdmin] = validatorId
REF_3943(mapping(address => uint16)) -> $_1 (-> ['TMP_13798']).adminToValidatorId
REF_3944(uint16) -> REF_3943[newAdmin_1]
$_2 (-> ['TMP_13798'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13798'])"])
REF_3944(uint16) (->$_2 (-> ['TMP_13798'])) := validatorId_1(uint16)
TMP_13798(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13798'])"])
 $.isAdminAssigned[oldAdmin] = false
REF_3945(mapping(address => bool)) -> $_2 (-> ['TMP_13798']).isAdminAssigned
REF_3946(bool) -> REF_3945[oldAdmin_1]
$_3 (-> ['TMP_13798'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13798'])"])
REF_3946(bool) (->$_3 (-> ['TMP_13798'])) := False(bool)
TMP_13798(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13798'])"])
 $.isAdminAssigned[newAdmin] = true
REF_3947(mapping(address => bool)) -> $_3 (-> ['TMP_13798']).isAdminAssigned
REF_3948(bool) -> REF_3947[newAdmin_1]
$_4 (-> ['TMP_13798'])(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13798'])"])
REF_3948(bool) (->$_4 (-> ['TMP_13798'])) := True(bool)
TMP_13798(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13798'])"])
 delete $.pendingAdmins[validatorId]
REF_3949(mapping(uint16 => address)) -> $_4 (-> ['TMP_13798']).pendingAdmins
REF_3950(address) -> REF_3949[validatorId_1]
REF_3949 = delete REF_3950 
 ValidatorAddressesSet(validatorId,oldAdmin,newAdmin,validator.l2WithdrawAddress,validator.l2WithdrawAddress,validator.l1ValidatorAddress,validator.l1ValidatorAddress,validator.l1AccountAddress,validator.l1AccountAddress,validator.l1AccountEvmAddress,validator.l1AccountEvmAddress)
REF_3951(address) -> validator_2 (-> ['$']).l2WithdrawAddress
REF_3952(address) -> validator_2 (-> ['$']).l2WithdrawAddress
REF_3953(string) -> validator_2 (-> ['$']).l1ValidatorAddress
REF_3954(string) -> validator_2 (-> ['$']).l1ValidatorAddress
REF_3955(string) -> validator_2 (-> ['$']).l1AccountAddress
REF_3956(string) -> validator_2 (-> ['$']).l1AccountAddress
REF_3957(address) -> validator_2 (-> ['$']).l1AccountEvmAddress
REF_3958(address) -> validator_2 (-> ['$']).l1AccountEvmAddress
Emit ValidatorAddressesSet(validatorId_1,oldAdmin_1,newAdmin_1,REF_3951,REF_3952,REF_3953,REF_3954,REF_3955,REF_3956,REF_3957,REF_3958)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardUpgradeable.nonReentrant()()
 _validateValidatorExists(validatorId)
MODIFIER_CALL, ValidatorFacet._validateValidatorExists(uint16)(validatorId_1)
```
#### ValidatorFacet.requestCommissionClaim(uint16,address) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13808(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13808'])(PlumeStakingStorage.Layout) := TMP_13808(PlumeStakingStorage.Layout)
 validator = $.validators[validatorId]
REF_3960(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13808']).validators
REF_3961(PlumeStakingStorage.ValidatorInfo) -> REF_3960[validatorId_1]
validator_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_3961(PlumeStakingStorage.ValidatorInfo)
 ! validator.active || validator.slashed
REF_3962(bool) -> validator_1 (-> ['$']).active
TMP_13809 = UnaryType.BANG REF_3962 
REF_3963(bool) -> validator_1 (-> ['$']).slashed
TMP_13810(bool) = TMP_13809 || REF_3963
CONDITION TMP_13810
 revert ValidatorInactive(uint16)(validatorId)
TMP_13811(None) = SOLIDITY_CALL revert ValidatorInactive(uint16)(validatorId_1)
 PlumeRewardLogic._settleCommissionForValidatorUpToNow($,validatorId)
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic._settleCommissionForValidatorUpToNow(PlumeStakingStorage.Layout,uint16), arguments:["$_1 (-> ['TMP_13808'])", 'validatorId_1'] 
 amount = $.validatorAccruedCommission[validatorId][token]
REF_3965(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> ['TMP_13808']).validatorAccruedCommission
REF_3966(mapping(address => uint256)) -> REF_3965[validatorId_1]
REF_3967(uint256) -> REF_3966[token_1]
amount_1(uint256) := REF_3967(uint256)
 amount == 0
TMP_13813(bool) = amount_1 == 0
CONDITION TMP_13813
 revert InvalidAmount(uint256)(0)
TMP_13814(None) = SOLIDITY_CALL revert InvalidAmount(uint256)(0)
 $.pendingCommissionClaims[validatorId][token].amount > 0
REF_3968(mapping(uint16 => mapping(address => PlumeStakingStorage.PendingCommissionClaim))) -> $_1 (-> ['TMP_13808']).pendingCommissionClaims
REF_3969(mapping(address => PlumeStakingStorage.PendingCommissionClaim)) -> REF_3968[validatorId_1]
REF_3970(PlumeStakingStorage.PendingCommissionClaim) -> REF_3969[token_1]
REF_3971(uint256) -> REF_3970.amount
TMP_13815(bool) = REF_3971 > 0
CONDITION TMP_13815
 revert PendingClaimExists(uint16,address)(validatorId,token)
TMP_13816(None) = SOLIDITY_CALL revert PendingClaimExists(uint16,address)(validatorId_1,token_1)
 recipient = validator.l2WithdrawAddress
REF_3972(address) -> validator_1 (-> ['$']).l2WithdrawAddress
recipient_1(address) := REF_3972(address)
 nowTs = block.timestamp
nowTs_1(uint256) := block.timestamp(uint256)
 $.pendingCommissionClaims[validatorId][token] = PlumeStakingStorage.PendingCommissionClaim({amount:amount,requestTimestamp:nowTs,token:token,recipient:recipient})
REF_3973(mapping(uint16 => mapping(address => PlumeStakingStorage.PendingCommissionClaim))) -> $_1 (-> ['TMP_13808']).pendingCommissionClaims
REF_3974(mapping(address => PlumeStakingStorage.PendingCommissionClaim)) -> REF_3973[validatorId_1]
REF_3975(PlumeStakingStorage.PendingCommissionClaim) -> REF_3974[token_1]
TMP_13817(PlumeStakingStorage.PendingCommissionClaim) = new PendingCommissionClaim(amount_1,nowTs_1,token_1,recipient_1)
$_2 (-> ['TMP_13808'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13808'])"])
REF_3975(PlumeStakingStorage.PendingCommissionClaim) (->$_2 (-> ['TMP_13808'])) := TMP_13817(PlumeStakingStorage.PendingCommissionClaim)
TMP_13808(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13808'])"])
 $.validatorAccruedCommission[validatorId][token] = 0
REF_3977(mapping(uint16 => mapping(address => uint256))) -> $_2 (-> ['TMP_13808']).validatorAccruedCommission
REF_3978(mapping(address => uint256)) -> REF_3977[validatorId_1]
REF_3979(uint256) -> REF_3978[token_1]
$_3 (-> ['TMP_13808'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13808'])"])
REF_3979(uint256) (->$_3 (-> ['TMP_13808'])) := 0(uint256)
TMP_13808(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13808'])"])
 CommissionClaimRequested(validatorId,token,recipient,amount,nowTs)
Emit CommissionClaimRequested(validatorId_1,token_1,recipient_1,amount_1,nowTs_1)
 onlyValidatorAdmin(validatorId)
MODIFIER_CALL, ValidatorFacet.onlyValidatorAdmin(uint16)(validatorId_1)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardUpgradeable.nonReentrant()()
 _validateValidatorExists(validatorId)
MODIFIER_CALL, ValidatorFacet._validateValidatorExists(uint16)(validatorId_1)
 _validateIsToken(token)
MODIFIER_CALL, ValidatorFacet._validateIsToken(address)(token_1)
```
#### ValidatorFacet.finalizeCommissionClaim(uint16,address) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13823(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13823'])(PlumeStakingStorage.Layout) := TMP_13823(PlumeStakingStorage.Layout)
 validator = $.validators[validatorId]
REF_3981(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13823']).validators
REF_3982(PlumeStakingStorage.ValidatorInfo) -> REF_3981[validatorId_1]
validator_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_3982(PlumeStakingStorage.ValidatorInfo)
 claim = $.pendingCommissionClaims[validatorId][token]
REF_3983(mapping(uint16 => mapping(address => PlumeStakingStorage.PendingCommissionClaim))) -> $_1 (-> ['TMP_13823']).pendingCommissionClaims
REF_3984(mapping(address => PlumeStakingStorage.PendingCommissionClaim)) -> REF_3983[validatorId_1]
REF_3985(PlumeStakingStorage.PendingCommissionClaim) -> REF_3984[token_1]
claim_1 (-> ['$'])(PlumeStakingStorage.PendingCommissionClaim) := REF_3985(PlumeStakingStorage.PendingCommissionClaim)
 claim.amount == 0
REF_3986(uint256) -> claim_1 (-> ['$']).amount
TMP_13824(bool) = REF_3986 == 0
CONDITION TMP_13824
 revert NoPendingClaim(uint16,address)(validatorId,token)
TMP_13825(None) = SOLIDITY_CALL revert NoPendingClaim(uint16,address)(validatorId_1,token_1)
 readyTimestamp = claim.requestTimestamp + PlumeStakingStorage.COMMISSION_CLAIM_TIMELOCK
REF_3987(uint256) -> claim_1 (-> ['$']).requestTimestamp
REF_3988(uint256) -> PlumeStakingStorage.COMMISSION_CLAIM_TIMELOCK
TMP_13826(uint256) = REF_3987 (c)+ REF_3988
readyTimestamp_1(uint256) := TMP_13826(uint256)
 block.timestamp < readyTimestamp
TMP_13827(bool) = block.timestamp < readyTimestamp_1
CONDITION TMP_13827
 revert ClaimNotReady(uint16,address,uint256)(validatorId,token,readyTimestamp)
TMP_13828(None) = SOLIDITY_CALL revert ClaimNotReady(uint16,address,uint256)(validatorId_1,token_1,readyTimestamp_1)
 validator.slashed && readyTimestamp >= validator.slashedAtTimestamp
REF_3989(bool) -> validator_1 (-> ['$']).slashed
REF_3990(uint256) -> validator_1 (-> ['$']).slashedAtTimestamp
TMP_13829(bool) = readyTimestamp_1 >= REF_3990
TMP_13830(bool) = REF_3989 && TMP_13829
CONDITION TMP_13830
 revert ValidatorInactive(uint16)(validatorId)
TMP_13831(None) = SOLIDITY_CALL revert ValidatorInactive(uint16)(validatorId_1)
 ! validator.slashed && ! validator.active
REF_3991(bool) -> validator_1 (-> ['$']).slashed
TMP_13832 = UnaryType.BANG REF_3991 
REF_3992(bool) -> validator_1 (-> ['$']).active
TMP_13833 = UnaryType.BANG REF_3992 
TMP_13834(bool) = TMP_13832 && TMP_13833
CONDITION TMP_13834
 revert ValidatorInactive(uint16)(validatorId)
TMP_13835(None) = SOLIDITY_CALL revert ValidatorInactive(uint16)(validatorId_1)
 amount = claim.amount
REF_3993(uint256) -> claim_1 (-> ['$']).amount
amount_1(uint256) := REF_3993(uint256)
 recipient = claim.recipient
REF_3994(address) -> claim_1 (-> ['$']).recipient
recipient_1(address) := REF_3994(address)
 delete $.pendingCommissionClaims[validatorId][token]
REF_3995(mapping(uint16 => mapping(address => PlumeStakingStorage.PendingCommissionClaim))) -> $_1 (-> ['TMP_13823']).pendingCommissionClaims
REF_3996(mapping(address => PlumeStakingStorage.PendingCommissionClaim)) -> REF_3995[validatorId_1]
REF_3997(PlumeStakingStorage.PendingCommissionClaim) -> REF_3996[token_1]
REF_3996 = delete REF_3997 
 treasury = RewardsFacet(address(this)).getTreasury()
TMP_13836 = CONVERT this to address
TMP_13837 = CONVERT TMP_13836 to RewardsFacet
TMP_13838(address) = HIGH_LEVEL_CALL, dest:TMP_13837(RewardsFacet), function:getTreasury, arguments:[]  
treasury_1(address) := TMP_13838(address)
 treasury == address(0)
TMP_13839 = CONVERT 0 to address
TMP_13840(bool) = treasury_1 == TMP_13839
CONDITION TMP_13840
 revert TreasuryNotSet()()
TMP_13841(None) = SOLIDITY_CALL revert TreasuryNotSet()()
 IPlumeStakingRewardTreasury(treasury).distributeReward(token,amount,recipient)
TMP_13842 = CONVERT treasury_1 to IPlumeStakingRewardTreasury
HIGH_LEVEL_CALL, dest:TMP_13842(IPlumeStakingRewardTreasury), function:distributeReward, arguments:['token_1', 'amount_1', 'recipient_1']  
 CommissionClaimFinalized(validatorId,token,recipient,amount,block.timestamp)
Emit CommissionClaimFinalized(validatorId_1,token_1,recipient_1,amount_1,block.timestamp)
 amount
RETURN amount_1
 onlyValidatorAdmin(validatorId)
MODIFIER_CALL, ValidatorFacet.onlyValidatorAdmin(uint16)(validatorId_1)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardUpgradeable.nonReentrant()()
```
#### ValidatorFacet._cleanupExpiredVotes(uint16) [INTERNAL]
```slithir
validatorId_1(uint16) := phi(['validatorId_1', 'validatorId_1', 'maliciousValidatorId_1'])
 $ = PlumeStakingStorage.layout()
TMP_13847(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13847'])(PlumeStakingStorage.Layout) := TMP_13847(PlumeStakingStorage.Layout)
 voteCount = $.slashVoteCounts[validatorId]
REF_4001(mapping(uint16 => uint256)) -> $_1 (-> ['TMP_13847']).slashVoteCounts
REF_4002(uint256) -> REF_4001[validatorId_1]
voteCount_1(uint256) := REF_4002(uint256)
 voteCount == 0
TMP_13848(bool) = voteCount_1 == 0
CONDITION TMP_13848
 0
RETURN 0
 allValidatorIds = $.validatorIds
REF_4003(uint16[]) -> $_1 (-> ['TMP_13847']).validatorIds
allValidatorIds_1(uint16[]) = ['REF_4003(uint16[])']
 newActiveVoteCount_scope_0 = 0
newActiveVoteCount_scope_0_1(uint256) := 0(uint256)
 i = 0
i_1(uint256) := 0(uint256)
 i < allValidatorIds.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_4004 -> LENGTH allValidatorIds_1
TMP_13849(bool) = i_2 < REF_4004
CONDITION TMP_13849
 voterValidatorId = allValidatorIds[i]
REF_4005(uint16) -> allValidatorIds_1[i_2]
voterValidatorId_1(uint16) := REF_4005(uint16)
 voterValidatorId == validatorId
TMP_13850(bool) = voterValidatorId_1 == validatorId_1
CONDITION TMP_13850
 voteExpiration = $.slashingVotes[validatorId][voterValidatorId]
REF_4006(mapping(uint16 => mapping(uint16 => uint256))) -> $_1 (-> ['TMP_13847']).slashingVotes
REF_4007(mapping(uint16 => uint256)) -> REF_4006[validatorId_1]
REF_4008(uint256) -> REF_4007[voterValidatorId_1]
voteExpiration_1(uint256) := REF_4008(uint256)
 voteExpiration > 0
TMP_13851(bool) = voteExpiration_1 > 0
CONDITION TMP_13851
 voterValidator = $.validators[voterValidatorId]
REF_4009(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13847']).validators
REF_4010(PlumeStakingStorage.ValidatorInfo) -> REF_4009[voterValidatorId_1]
voterValidator_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_4010(PlumeStakingStorage.ValidatorInfo)
 voterIsEligible = voterValidator.active && ! voterValidator.slashed
REF_4011(bool) -> voterValidator_1 (-> ['$']).active
REF_4012(bool) -> voterValidator_1 (-> ['$']).slashed
TMP_13852 = UnaryType.BANG REF_4012 
TMP_13853(bool) = REF_4011 && TMP_13852
voterIsEligible_1(bool) := TMP_13853(bool)
 voteHasExpired = block.timestamp >= voteExpiration
TMP_13854(bool) = block.timestamp >= voteExpiration_1
voteHasExpired_1(bool) := TMP_13854(bool)
 voterIsEligible && ! voteHasExpired
TMP_13855 = UnaryType.BANG voteHasExpired_1 
TMP_13856(bool) = voterIsEligible_1 && TMP_13855
CONDITION TMP_13856
 newActiveVoteCount_scope_0 ++
TMP_13857(uint256) := newActiveVoteCount_scope_0_1(uint256)
newActiveVoteCount_scope_0_2(uint256) = newActiveVoteCount_scope_0_1 (c)+ 1
 delete $.slashingVotes[validatorId][voterValidatorId]
REF_4013(mapping(uint16 => mapping(uint16 => uint256))) -> $_1 (-> ['TMP_13847']).slashingVotes
REF_4014(mapping(uint16 => uint256)) -> REF_4013[validatorId_1]
REF_4015(uint256) -> REF_4014[voterValidatorId_1]
REF_4014 = delete REF_4015 
$_3 (-> ['TMP_13847'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13847'])", "$_1 (-> ['TMP_13847'])"])
newActiveVoteCount_scope_0_3(uint256) := phi(['newActiveVoteCount_scope_0_2', 'newActiveVoteCount_scope_0_1'])
 i ++
TMP_13858(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 $.slashVoteCounts[validatorId] = newActiveVoteCount_scope_0
REF_4016(mapping(uint16 => uint256)) -> $_1 (-> ['TMP_13847']).slashVoteCounts
REF_4017(uint256) -> REF_4016[validatorId_1]
$_2 (-> ['TMP_13847'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13847'])"])
REF_4017(uint256) (->$_2 (-> ['TMP_13847'])) := newActiveVoteCount_scope_0_1(uint256)
TMP_13847(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13847'])"])
 newActiveVoteCount_scope_0
RETURN newActiveVoteCount_scope_0_1
 newActiveVoteCount
```
#### ValidatorFacet.voteToSlashValidator(uint16,uint256) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13859(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13859'])(PlumeStakingStorage.Layout) := TMP_13859(PlumeStakingStorage.Layout)
 voterAdmin = msg.sender
voterAdmin_1(address) := msg.sender(address)
 voterValidatorId = $.adminToValidatorId[voterAdmin]
REF_4019(mapping(address => uint16)) -> $_1 (-> ['TMP_13859']).adminToValidatorId
REF_4020(uint16) -> REF_4019[voterAdmin_1]
voterValidatorId_1(uint16) := REF_4020(uint16)
 $.validators[voterValidatorId].l2AdminAddress != voterAdmin || ! $.validators[voterValidatorId].active
REF_4021(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13859']).validators
REF_4022(PlumeStakingStorage.ValidatorInfo) -> REF_4021[voterValidatorId_1]
REF_4023(address) -> REF_4022.l2AdminAddress
TMP_13860(bool) = REF_4023 != voterAdmin_1
REF_4024(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13859']).validators
REF_4025(PlumeStakingStorage.ValidatorInfo) -> REF_4024[voterValidatorId_1]
REF_4026(bool) -> REF_4025.active
TMP_13861 = UnaryType.BANG REF_4026 
TMP_13862(bool) = TMP_13860 || TMP_13861
CONDITION TMP_13862
 revert NotValidatorAdmin(address)(voterAdmin)
TMP_13863(None) = SOLIDITY_CALL revert NotValidatorAdmin(address)(voterAdmin_1)
 targetValidator = $.validators[maliciousValidatorId]
REF_4027(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13859']).validators
REF_4028(PlumeStakingStorage.ValidatorInfo) -> REF_4027[maliciousValidatorId_1]
targetValidator_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_4028(PlumeStakingStorage.ValidatorInfo)
 ! $.validatorExists[maliciousValidatorId]
REF_4029(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13859']).validatorExists
REF_4030(bool) -> REF_4029[maliciousValidatorId_1]
TMP_13864 = UnaryType.BANG REF_4030 
CONDITION TMP_13864
 revert ValidatorDoesNotExist(uint16)(maliciousValidatorId)
TMP_13865(None) = SOLIDITY_CALL revert ValidatorDoesNotExist(uint16)(maliciousValidatorId_1)
 targetValidator.slashed
REF_4031(bool) -> targetValidator_1 (-> ['$']).slashed
CONDITION REF_4031
 revert ValidatorAlreadySlashed(uint16)(maliciousValidatorId)
TMP_13866(None) = SOLIDITY_CALL revert ValidatorAlreadySlashed(uint16)(maliciousValidatorId_1)
 ! targetValidator.active
REF_4032(bool) -> targetValidator_1 (-> ['$']).active
TMP_13867 = UnaryType.BANG REF_4032 
CONDITION TMP_13867
 revert ValidatorInactive(uint16)(maliciousValidatorId)
TMP_13868(None) = SOLIDITY_CALL revert ValidatorInactive(uint16)(maliciousValidatorId_1)
 voterValidatorId == maliciousValidatorId
TMP_13869(bool) = voterValidatorId_1 == maliciousValidatorId_1
CONDITION TMP_13869
 revert CannotVoteForSelf()()
TMP_13870(None) = SOLIDITY_CALL revert CannotVoteForSelf()()
 voteExpiration <= block.timestamp || $.maxSlashVoteDurationInSeconds == 0 || voteExpiration > block.timestamp + $.maxSlashVoteDurationInSeconds
TMP_13871(bool) = voteExpiration_1 <= block.timestamp
REF_4033(uint256) -> $_1 (-> ['TMP_13859']).maxSlashVoteDurationInSeconds
TMP_13872(bool) = REF_4033 == 0
TMP_13873(bool) = TMP_13871 || TMP_13872
REF_4034(uint256) -> $_1 (-> ['TMP_13859']).maxSlashVoteDurationInSeconds
TMP_13874(uint256) = block.timestamp (c)+ REF_4034
TMP_13875(bool) = voteExpiration_1 > TMP_13874
TMP_13876(bool) = TMP_13873 || TMP_13875
CONDITION TMP_13876
 revert SlashVoteDurationTooLong()()
TMP_13877(None) = SOLIDITY_CALL revert SlashVoteDurationTooLong()()
 currentVoteExpiration = $.slashingVotes[maliciousValidatorId][voterValidatorId]
REF_4035(mapping(uint16 => mapping(uint16 => uint256))) -> $_1 (-> ['TMP_13859']).slashingVotes
REF_4036(mapping(uint16 => uint256)) -> REF_4035[maliciousValidatorId_1]
REF_4037(uint256) -> REF_4036[voterValidatorId_1]
currentVoteExpiration_1(uint256) := REF_4037(uint256)
 currentVoteExpiration > block.timestamp
TMP_13878(bool) = currentVoteExpiration_1 > block.timestamp
CONDITION TMP_13878
 revert AlreadyVotedToSlash(uint16,uint16)(maliciousValidatorId,voterValidatorId)
TMP_13879(None) = SOLIDITY_CALL revert AlreadyVotedToSlash(uint16,uint16)(maliciousValidatorId_1,voterValidatorId_1)
 _cleanupExpiredVotes(maliciousValidatorId)
TMP_13880(uint256) = INTERNAL_CALL, ValidatorFacet._cleanupExpiredVotes(uint16)(maliciousValidatorId_1)
 $.slashingVotes[maliciousValidatorId][voterValidatorId] = voteExpiration
REF_4038(mapping(uint16 => mapping(uint16 => uint256))) -> $_1 (-> ['TMP_13859']).slashingVotes
REF_4039(mapping(uint16 => uint256)) -> REF_4038[maliciousValidatorId_1]
REF_4040(uint256) -> REF_4039[voterValidatorId_1]
$_2 (-> ['TMP_13859'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13859'])"])
REF_4040(uint256) (->$_2 (-> ['TMP_13859'])) := voteExpiration_1(uint256)
TMP_13859(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13859'])"])
 $.slashVoteCounts[maliciousValidatorId] ++
REF_4041(mapping(uint16 => uint256)) -> $_2 (-> ['TMP_13859']).slashVoteCounts
REF_4042(uint256) -> REF_4041[maliciousValidatorId_1]
TMP_13881(uint256) := REF_4042(uint256)
$_3 (-> ['TMP_13859'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13859'])"])
REF_4042(-> $_3 (-> ['TMP_13859'])) = REF_4042 (c)+ 1
TMP_13859(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13859'])"])
 SlashVoteCast(maliciousValidatorId,voterValidatorId,voteExpiration)
Emit SlashVoteCast(maliciousValidatorId_1,voterValidatorId_1,voteExpiration_1)
 activeVoteCount = $.slashVoteCounts[maliciousValidatorId]
REF_4043(mapping(uint16 => uint256)) -> $_3 (-> ['TMP_13859']).slashVoteCounts
REF_4044(uint256) -> REF_4043[maliciousValidatorId_1]
activeVoteCount_1(uint256) := REF_4044(uint256)
 totalEligibleValidators = _countEligibleValidators(maliciousValidatorId)
TMP_13883(uint256) = INTERNAL_CALL, ValidatorFacet._countEligibleValidators(uint16)(maliciousValidatorId_1)
totalEligibleValidators_1(uint256) := TMP_13883(uint256)
 activeVoteCount >= totalEligibleValidators && totalEligibleValidators > 0
TMP_13884(bool) = activeVoteCount_1 >= totalEligibleValidators_1
TMP_13885(bool) = totalEligibleValidators_1 > 0
TMP_13886(bool) = TMP_13884 && TMP_13885
CONDITION TMP_13886
 _performSlash(maliciousValidatorId,msg.sender)
INTERNAL_CALL, ValidatorFacet._performSlash(uint16,address)(maliciousValidatorId_1,msg.sender)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardUpgradeable.nonReentrant()()
```
#### ValidatorFacet.slashValidator(uint16) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13889(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13889'])(PlumeStakingStorage.Layout) := TMP_13889(PlumeStakingStorage.Layout)
 ! $.validatorExists[validatorId]
REF_4046(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13889']).validatorExists
REF_4047(bool) -> REF_4046[validatorId_1]
TMP_13890 = UnaryType.BANG REF_4047 
CONDITION TMP_13890
 revert ValidatorDoesNotExist(uint16)(validatorId)
TMP_13891(None) = SOLIDITY_CALL revert ValidatorDoesNotExist(uint16)(validatorId_1)
 validatorToSlash = $.validators[validatorId]
REF_4048(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13889']).validators
REF_4049(PlumeStakingStorage.ValidatorInfo) -> REF_4048[validatorId_1]
validatorToSlash_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_4049(PlumeStakingStorage.ValidatorInfo)
 validatorToSlash.slashed
REF_4050(bool) -> validatorToSlash_1 (-> ['$']).slashed
CONDITION REF_4050
 revert ValidatorAlreadySlashed(uint16)(validatorId)
TMP_13892(None) = SOLIDITY_CALL revert ValidatorAlreadySlashed(uint16)(validatorId_1)
 activeVoteCount = _cleanupExpiredVotes(validatorId)
TMP_13893(uint256) = INTERNAL_CALL, ValidatorFacet._cleanupExpiredVotes(uint16)(validatorId_1)
activeVoteCount_1(uint256) := TMP_13893(uint256)
 totalEligibleValidators = _countEligibleValidators(validatorId)
TMP_13894(uint256) = INTERNAL_CALL, ValidatorFacet._countEligibleValidators(uint16)(validatorId_1)
totalEligibleValidators_1(uint256) := TMP_13894(uint256)
 activeVoteCount < totalEligibleValidators
TMP_13895(bool) = activeVoteCount_1 < totalEligibleValidators_1
CONDITION TMP_13895
 revert UnanimityNotReached(uint256,uint256)(activeVoteCount,totalEligibleValidators)
TMP_13896(None) = SOLIDITY_CALL revert UnanimityNotReached(uint256,uint256)(activeVoteCount_1,totalEligibleValidators_1)
 _performSlash(validatorId,msg.sender)
INTERNAL_CALL, ValidatorFacet._performSlash(uint16,address)(validatorId_1,msg.sender)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardUpgradeable.nonReentrant()()
 onlyRole(PlumeRoles.TIMELOCK_ROLE)
REF_4051(bytes32) -> PlumeRoles.TIMELOCK_ROLE
MODIFIER_CALL, ValidatorFacet.onlyRole(bytes32)(REF_4051)
 onlyRole(PlumeRoles.TIMELOCK_ROLE)
REF_4052(bytes32) -> PlumeRoles.TIMELOCK_ROLE
MODIFIER_CALL, ValidatorFacet.onlyRole(bytes32)(REF_4052)
```
#### ValidatorFacet.forceSettleValidatorCommission(uint16) [EXTERNAL]
```slithir
 $s = PlumeStakingStorage.layout()
TMP_13901(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$s_1 (-> ['TMP_13901'])(PlumeStakingStorage.Layout) := TMP_13901(PlumeStakingStorage.Layout)
 ! $s.validatorExists[validatorId]
REF_4054(mapping(uint16 => bool)) -> $s_1 (-> ['TMP_13901']).validatorExists
REF_4055(bool) -> REF_4054[validatorId_1]
TMP_13902 = UnaryType.BANG REF_4055 
CONDITION TMP_13902
 revert ValidatorDoesNotExist(uint16)(validatorId)
TMP_13903(None) = SOLIDITY_CALL revert ValidatorDoesNotExist(uint16)(validatorId_1)
 PlumeRewardLogic._settleCommissionForValidatorUpToNow($s,validatorId)
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic._settleCommissionForValidatorUpToNow(PlumeStakingStorage.Layout,uint16), arguments:["$s_1 (-> ['TMP_13901'])", 'validatorId_1']
```
#### ValidatorFacet.cleanupExpiredVotes(uint16) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13905(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13905'])(PlumeStakingStorage.Layout) := TMP_13905(PlumeStakingStorage.Layout)
 ! $.validatorExists[validatorId]
REF_4058(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13905']).validatorExists
REF_4059(bool) -> REF_4058[validatorId_1]
TMP_13906 = UnaryType.BANG REF_4059 
CONDITION TMP_13906
 revert ValidatorDoesNotExist(uint16)(validatorId)
TMP_13907(None) = SOLIDITY_CALL revert ValidatorDoesNotExist(uint16)(validatorId_1)
 _cleanupExpiredVotes(validatorId)
TMP_13908(uint256) = INTERNAL_CALL, ValidatorFacet._cleanupExpiredVotes(uint16)(validatorId_1)
RETURN TMP_13908
 validVoteCount
```
#### ValidatorFacet._performSlash(uint16,address) [INTERNAL]
```slithir
validatorId_1(uint16) := phi(['validatorId_1', 'maliciousValidatorId_1'])
slasher_1(address) := phi(['msg.sender'])
 $ = PlumeStakingStorage.layout()
TMP_13909(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13909'])(PlumeStakingStorage.Layout) := TMP_13909(PlumeStakingStorage.Layout)
 validatorToSlash = $.validators[validatorId]
REF_4061(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13909']).validators
REF_4062(PlumeStakingStorage.ValidatorInfo) -> REF_4061[validatorId_1]
validatorToSlash_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_4062(PlumeStakingStorage.ValidatorInfo)
 validatorToSlash.slashed
REF_4063(bool) -> validatorToSlash_1 (-> ['$']).slashed
CONDITION REF_4063
 stakeLost = $.validatorTotalStaked[validatorId]
REF_4064(mapping(uint16 => uint256)) -> $_1 (-> ['TMP_13909']).validatorTotalStaked
REF_4065(uint256) -> REF_4064[validatorId_1]
stakeLost_1(uint256) := REF_4065(uint256)
 cooledLost = $.validatorTotalCooling[validatorId]
REF_4066(mapping(uint16 => uint256)) -> $_1 (-> ['TMP_13909']).validatorTotalCooling
REF_4067(uint256) -> REF_4066[validatorId_1]
cooledLost_1(uint256) := REF_4067(uint256)
 validatorToSlash.active = false
REF_4068(bool) -> validatorToSlash_1 (-> ['$']).active
validatorToSlash_2 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validatorToSlash_1 (-> ['$'])"])
REF_4068(bool) (->validatorToSlash_2 (-> ['$'])) := False(bool)
$_8 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validatorToSlash_2 (-> ['$'])"])
 validatorToSlash.slashed = true
REF_4069(bool) -> validatorToSlash_2 (-> ['$']).slashed
validatorToSlash_3 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validatorToSlash_2 (-> ['$'])"])
REF_4069(bool) (->validatorToSlash_3 (-> ['$'])) := True(bool)
$_9 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validatorToSlash_3 (-> ['$'])"])
 validatorToSlash.slashedAtTimestamp = block.timestamp
REF_4070(uint256) -> validatorToSlash_3 (-> ['$']).slashedAtTimestamp
validatorToSlash_4 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validatorToSlash_3 (-> ['$'])"])
REF_4070(uint256) (->validatorToSlash_4 (-> ['$'])) := block.timestamp(uint256)
$_10 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validatorToSlash_4 (-> ['$'])"])
 $.totalStaked -= stakeLost
REF_4071(uint256) -> $_1 (-> ['TMP_13909']).totalStaked
$_2 (-> ['TMP_13909'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13909'])"])
REF_4071(-> $_2 (-> ['TMP_13909'])) = REF_4071 (c)- stakeLost_1
TMP_13909(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13909'])"])
 $.totalCooling -= cooledLost
REF_4072(uint256) -> $_2 (-> ['TMP_13909']).totalCooling
$_3 (-> ['TMP_13909'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13909'])"])
REF_4072(-> $_3 (-> ['TMP_13909'])) = REF_4072 (c)- cooledLost_1
TMP_13909(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13909'])"])
 $.validatorTotalStaked[validatorId] = 0
REF_4073(mapping(uint16 => uint256)) -> $_3 (-> ['TMP_13909']).validatorTotalStaked
REF_4074(uint256) -> REF_4073[validatorId_1]
$_4 (-> ['TMP_13909'])(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13909'])"])
REF_4074(uint256) (->$_4 (-> ['TMP_13909'])) := 0(uint256)
TMP_13909(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13909'])"])
 $.validatorTotalCooling[validatorId] = 0
REF_4075(mapping(uint16 => uint256)) -> $_4 (-> ['TMP_13909']).validatorTotalCooling
REF_4076(uint256) -> REF_4075[validatorId_1]
$_5 (-> ['TMP_13909'])(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13909'])"])
REF_4076(uint256) (->$_5 (-> ['TMP_13909'])) := 0(uint256)
TMP_13909(PlumeStakingStorage.Layout) := phi(["$_5 (-> ['TMP_13909'])"])
 validatorToSlash.delegatedAmount = 0
REF_4077(uint256) -> validatorToSlash_4 (-> ['$']).delegatedAmount
validatorToSlash_5 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validatorToSlash_4 (-> ['$'])"])
REF_4077(uint256) (->validatorToSlash_5 (-> ['$'])) := 0(uint256)
$_11 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validatorToSlash_5 (-> ['$'])"])
 delete $.validatorStakers[validatorId]
REF_4078(mapping(uint16 => address[])) -> $_5 (-> ['TMP_13909']).validatorStakers
REF_4079(address[]) -> REF_4078[validatorId_1]
REF_4078 = delete REF_4079 
 $.slashVoteCounts[validatorId] = 0
REF_4080(mapping(uint16 => uint256)) -> $_5 (-> ['TMP_13909']).slashVoteCounts
REF_4081(uint256) -> REF_4080[validatorId_1]
$_6 (-> ['TMP_13909'])(PlumeStakingStorage.Layout) := phi(["$_5 (-> ['TMP_13909'])"])
REF_4081(uint256) (->$_6 (-> ['TMP_13909'])) := 0(uint256)
TMP_13909(PlumeStakingStorage.Layout) := phi(["$_6 (-> ['TMP_13909'])"])
 validatorIds = $.validatorIds
REF_4082(uint16[]) -> $_6 (-> ['TMP_13909']).validatorIds
validatorIds_1(uint16[]) = ['REF_4082(uint16[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < validatorIds.length
$_7 (-> ['TMP_13909'])(PlumeStakingStorage.Layout) := phi(["$_6 (-> ['TMP_13909'])", "$_7 (-> ['TMP_13909'])"])
i_2(uint256) := phi(['i_1', 'i_3'])
REF_4083 -> LENGTH validatorIds_1
TMP_13910(bool) = i_2 < REF_4083
CONDITION TMP_13910
 delete $.slashingVotes[validatorId][validatorIds[i]]
REF_4084(mapping(uint16 => mapping(uint16 => uint256))) -> $_7 (-> ['TMP_13909']).slashingVotes
REF_4085(mapping(uint16 => uint256)) -> REF_4084[validatorId_1]
REF_4086(uint16) -> validatorIds_1[i_2]
REF_4087(uint256) -> REF_4085[REF_4086]
REF_4085 = delete REF_4087 
 i ++
TMP_13911(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 ValidatorSlashed(validatorId,slasher,stakeLost + cooledLost)
TMP_13912(uint256) = stakeLost_1 (c)+ cooledLost_1
Emit ValidatorSlashed(validatorId_1,slasher_1,TMP_13912)
 ValidatorStatusUpdated(validatorId,false,true)
Emit ValidatorStatusUpdated(validatorId_1,False,True)
```
#### ValidatorFacet._countEligibleValidators(uint16) [INTERNAL]
```slithir
validatorToExclude_1(uint16) := phi(['validatorId_1', 'maliciousValidatorId_1'])
 $ = PlumeStakingStorage.layout()
TMP_13915(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13915'])(PlumeStakingStorage.Layout) := TMP_13915(PlumeStakingStorage.Layout)
 totalActive = _countActiveValidators()
TMP_13916(uint256) = INTERNAL_CALL, ValidatorFacet._countActiveValidators()()
totalActive_1(uint256) := TMP_13916(uint256)
 excludedInfo = $.validators[validatorToExclude]
REF_4089(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13915']).validators
REF_4090(PlumeStakingStorage.ValidatorInfo) -> REF_4089[validatorToExclude_1]
excludedInfo_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_4090(PlumeStakingStorage.ValidatorInfo)
 excludedInfo.active && ! excludedInfo.slashed
REF_4091(bool) -> excludedInfo_1 (-> ['$']).active
REF_4092(bool) -> excludedInfo_1 (-> ['$']).slashed
TMP_13917 = UnaryType.BANG REF_4092 
TMP_13918(bool) = REF_4091 && TMP_13917
CONDITION TMP_13918
 totalActive > 0
TMP_13919(bool) = totalActive_1 > 0
CONDITION TMP_13919
 totalActive - 1
TMP_13920(uint256) = totalActive_1 (c)- 1
RETURN TMP_13920
 totalActive
RETURN totalActive_1
```
#### ValidatorFacet._countActiveValidators() [INTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13921(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13921'])(PlumeStakingStorage.Layout) := TMP_13921(PlumeStakingStorage.Layout)
 ids = $.validatorIds
REF_4094(uint16[]) -> $_1 (-> ['TMP_13921']).validatorIds
ids_1(uint16[]) = ['REF_4094(uint16[])']
 count = 0
count_1(uint256) := 0(uint256)
 i = 0
i_1(uint256) := 0(uint256)
 i < ids.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_4095 -> LENGTH ids_1
TMP_13922(bool) = i_2 < REF_4095
CONDITION TMP_13922
 info = $.validators[ids[i]]
REF_4096(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13921']).validators
REF_4097(uint16) -> ids_1[i_2]
REF_4098(PlumeStakingStorage.ValidatorInfo) -> REF_4096[REF_4097]
info_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_4098(PlumeStakingStorage.ValidatorInfo)
 info.active && ! info.slashed
REF_4099(bool) -> info_1 (-> ['$']).active
REF_4100(bool) -> info_1 (-> ['$']).slashed
TMP_13923 = UnaryType.BANG REF_4100 
TMP_13924(bool) = REF_4099 && TMP_13923
CONDITION TMP_13924
 count ++
TMP_13925(uint256) := count_1(uint256)
count_2(uint256) = count_1 (c)+ 1
count_3(uint256) := phi(['count_2', 'count_1'])
 i ++
TMP_13926(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 count
RETURN count_1
```
#### ValidatorFacet.getValidatorInfo(uint16) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13927(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13927'])(PlumeStakingStorage.Layout) := TMP_13927(PlumeStakingStorage.Layout)
 ! $.validatorExists[validatorId]
REF_4102(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13927']).validatorExists
REF_4103(bool) -> REF_4102[validatorId_1]
TMP_13928 = UnaryType.BANG REF_4103 
CONDITION TMP_13928
 revert ValidatorDoesNotExist(uint16)(validatorId)
TMP_13929(None) = SOLIDITY_CALL revert ValidatorDoesNotExist(uint16)(validatorId_1)
 info = $.validators[validatorId]
REF_4104(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13927']).validators
REF_4105(PlumeStakingStorage.ValidatorInfo) -> REF_4104[validatorId_1]
info_1(PlumeStakingStorage.ValidatorInfo) := REF_4105(PlumeStakingStorage.ValidatorInfo)
 totalStaked = $.validatorTotalStaked[validatorId]
REF_4106(mapping(uint16 => uint256)) -> $_1 (-> ['TMP_13927']).validatorTotalStaked
REF_4107(uint256) -> REF_4106[validatorId_1]
totalStaked_1(uint256) := REF_4107(uint256)
 stakersCount = $.validatorStakers[validatorId].length
REF_4108(mapping(uint16 => address[])) -> $_1 (-> ['TMP_13927']).validatorStakers
REF_4109(address[]) -> REF_4108[validatorId_1]
REF_4110 -> LENGTH REF_4109
stakersCount_1(uint256) := REF_4110(uint256)
 info.delegatedAmount = totalStaked
REF_4111(uint256) -> info_1.delegatedAmount
info_2(PlumeStakingStorage.ValidatorInfo) := phi(['info_1'])
REF_4111(uint256) (->info_2) := totalStaked_1(uint256)
 (info,totalStaked,stakersCount)
RETURN info_2,totalStaked_1,stakersCount_1
 (info,totalStaked,stakersCount)
```
#### ValidatorFacet.getValidatorStats(uint16) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13930(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13930'])(PlumeStakingStorage.Layout) := TMP_13930(PlumeStakingStorage.Layout)
 ! $.validatorExists[validatorId]
REF_4113(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13930']).validatorExists
REF_4114(bool) -> REF_4113[validatorId_1]
TMP_13931 = UnaryType.BANG REF_4114 
CONDITION TMP_13931
 revert ValidatorDoesNotExist(uint16)(validatorId)
TMP_13932(None) = SOLIDITY_CALL revert ValidatorDoesNotExist(uint16)(validatorId_1)
 validator = $.validators[validatorId]
REF_4115(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13930']).validators
REF_4116(PlumeStakingStorage.ValidatorInfo) -> REF_4115[validatorId_1]
validator_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_4116(PlumeStakingStorage.ValidatorInfo)
 active = validator.active
REF_4117(bool) -> validator_1 (-> ['$']).active
active_1(bool) := REF_4117(bool)
 commission = validator.commission
REF_4118(uint256) -> validator_1 (-> ['$']).commission
commission_1(uint256) := REF_4118(uint256)
 totalStaked = $.validatorTotalStaked[validatorId]
REF_4119(mapping(uint16 => uint256)) -> $_1 (-> ['TMP_13930']).validatorTotalStaked
REF_4120(uint256) -> REF_4119[validatorId_1]
totalStaked_1(uint256) := REF_4120(uint256)
 stakersCount = $.validatorStakers[validatorId].length
REF_4121(mapping(uint16 => address[])) -> $_1 (-> ['TMP_13930']).validatorStakers
REF_4122(address[]) -> REF_4121[validatorId_1]
REF_4123 -> LENGTH REF_4122
stakersCount_1(uint256) := REF_4123(uint256)
 (active,commission,totalStaked,stakersCount)
RETURN active_1,commission_1,totalStaked_1,stakersCount_1
 (active,commission,totalStaked,stakersCount)
```
#### ValidatorFacet.getUserValidators(address) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13933(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13933'])(PlumeStakingStorage.Layout) := TMP_13933(PlumeStakingStorage.Layout)
 userAssociatedValidators = $.userValidators[user]
REF_4125(mapping(address => uint16[])) -> $_1 (-> ['TMP_13933']).userValidators
REF_4126(uint16[]) -> REF_4125[user_1]
userAssociatedValidators_1 (-> [])(uint16[]) = ['REF_4126(uint16[])']
 associatedCount = userAssociatedValidators.length
REF_4127 -> LENGTH userAssociatedValidators_1 (-> [])
associatedCount_1(uint256) := REF_4127(uint256)
 associatedCount == 0
TMP_13934(bool) = associatedCount_1 == 0
CONDITION TMP_13934
 new uint16[](0)
TMP_13936(uint16[])  = new uint16[](0)
RETURN TMP_13936
 tempNonSlashedValidators = new uint16[](associatedCount)
TMP_13938(uint16[])  = new uint16[](associatedCount_1)
tempNonSlashedValidators_1(uint16[]) = ['TMP_13938(uint16[])']
 actualCount = 0
actualCount_1(uint256) := 0(uint256)
 i = 0
i_1(uint256) := 0(uint256)
 i < associatedCount
i_2(uint256) := phi(['i_1', 'i_3'])
TMP_13939(bool) = i_2 < associatedCount_1
CONDITION TMP_13939
 valId = userAssociatedValidators[i]
REF_4128(uint16) -> userAssociatedValidators_1 (-> [])[i_2]
valId_1(uint16) := REF_4128(uint16)
 $.validatorExists[valId] && ! $.validators[valId].slashed
REF_4129(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13933']).validatorExists
REF_4130(bool) -> REF_4129[valId_1]
REF_4131(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13933']).validators
REF_4132(PlumeStakingStorage.ValidatorInfo) -> REF_4131[valId_1]
REF_4133(bool) -> REF_4132.slashed
TMP_13940 = UnaryType.BANG REF_4133 
TMP_13941(bool) = REF_4130 && TMP_13940
CONDITION TMP_13941
 tempNonSlashedValidators[actualCount] = valId
REF_4134(uint16) -> tempNonSlashedValidators_1[actualCount_1]
tempNonSlashedValidators_2(uint16[]) := phi(['tempNonSlashedValidators_1'])
REF_4134(uint16) (->tempNonSlashedValidators_2) := valId_1(uint16)
 actualCount ++
TMP_13942(uint256) := actualCount_1(uint256)
actualCount_2(uint256) = actualCount_1 (c)+ 1
actualCount_3(uint256) := phi(['actualCount_1', 'actualCount_2'])
 i ++
TMP_13943(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 finalNonSlashedValidators = new uint16[](actualCount)
TMP_13945(uint16[])  = new uint16[](actualCount_1)
finalNonSlashedValidators_1(uint16[]) = ['TMP_13945(uint16[])']
 i_scope_0 = 0
i_scope_0_1(uint256) := 0(uint256)
 i_scope_0 < actualCount
i_scope_0_2(uint256) := phi(['i_scope_0_3', 'i_scope_0_1'])
TMP_13946(bool) = i_scope_0_2 < actualCount_1
CONDITION TMP_13946
 finalNonSlashedValidators[i_scope_0] = tempNonSlashedValidators[i_scope_0]
REF_4135(uint16) -> finalNonSlashedValidators_1[i_scope_0_2]
REF_4136(uint16) -> tempNonSlashedValidators_1[i_scope_0_2]
finalNonSlashedValidators_2(uint16[]) := phi(['finalNonSlashedValidators_1'])
REF_4135(uint16) (->finalNonSlashedValidators_2) := REF_4136(uint16)
 i_scope_0 ++
TMP_13947(uint256) := i_scope_0_2(uint256)
i_scope_0_3(uint256) = i_scope_0_2 (c)+ 1
 finalNonSlashedValidators
RETURN finalNonSlashedValidators_1
```
#### ValidatorFacet.getAccruedCommission(uint16,address) [PUBLIC]
```slithir
 $s = PlumeStakingStorage.layout()
TMP_13948(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$s_1 (-> ['TMP_13948'])(PlumeStakingStorage.Layout) := TMP_13948(PlumeStakingStorage.Layout)
 ! $s.validatorExists[validatorId]
REF_4138(mapping(uint16 => bool)) -> $s_1 (-> ['TMP_13948']).validatorExists
REF_4139(bool) -> REF_4138[validatorId_1]
TMP_13949 = UnaryType.BANG REF_4139 
CONDITION TMP_13949
 revert ValidatorDoesNotExist(uint16)(validatorId)
TMP_13950(None) = SOLIDITY_CALL revert ValidatorDoesNotExist(uint16)(validatorId_1)
 ! $s.isRewardToken[token]
REF_4140(mapping(address => bool)) -> $s_1 (-> ['TMP_13948']).isRewardToken
REF_4141(bool) -> REF_4140[token_1]
TMP_13951 = UnaryType.BANG REF_4141 
CONDITION TMP_13951
 revert TokenDoesNotExist(address)(token)
TMP_13952(None) = SOLIDITY_CALL revert TokenDoesNotExist(address)(token_1)
 $s.validatorAccruedCommission[validatorId][token]
REF_4142(mapping(uint16 => mapping(address => uint256))) -> $s_1 (-> ['TMP_13948']).validatorAccruedCommission
REF_4143(mapping(address => uint256)) -> REF_4142[validatorId_1]
REF_4144(uint256) -> REF_4143[token_1]
RETURN REF_4144
```
#### ValidatorFacet.getValidatorsList() [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13953(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13953'])(PlumeStakingStorage.Layout) := TMP_13953(PlumeStakingStorage.Layout)
 ids = $.validatorIds
REF_4146(uint16[]) -> $_1 (-> ['TMP_13953']).validatorIds
ids_1(uint16[]) = ['REF_4146(uint16[])']
 numValidators = ids.length
REF_4147 -> LENGTH ids_1
numValidators_1(uint256) := REF_4147(uint256)
 list = new ValidatorFacet.ValidatorListData[](numValidators)
TMP_13955(ValidatorFacet.ValidatorListData[])  = new ValidatorFacet.ValidatorListData[](numValidators_1)
list_1(ValidatorFacet.ValidatorListData[]) = ['TMP_13955(ValidatorFacet.ValidatorListData[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < numValidators
list_2(ValidatorFacet.ValidatorListData[]) := phi(['list_1', 'list_3'])
i_2(uint256) := phi(['i_3', 'i_1'])
TMP_13956(bool) = i_2 < numValidators_1
CONDITION TMP_13956
 id = ids[i]
REF_4148(uint16) -> ids_1[i_2]
id_1(uint16) := REF_4148(uint16)
 info = $.validators[id]
REF_4149(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13953']).validators
REF_4150(PlumeStakingStorage.ValidatorInfo) -> REF_4149[id_1]
info_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_4150(PlumeStakingStorage.ValidatorInfo)
 list[i] = ValidatorFacet.ValidatorListData({id:id,totalStaked:$.validatorTotalStaked[id],commission:info.commission})
REF_4151(ValidatorFacet.ValidatorListData) -> list_2[i_2]
REF_4153(mapping(uint16 => uint256)) -> $_1 (-> ['TMP_13953']).validatorTotalStaked
REF_4154(uint256) -> REF_4153[id_1]
REF_4155(uint256) -> info_1 (-> ['$']).commission
TMP_13957(ValidatorFacet.ValidatorListData) = new ValidatorListData(id_1,REF_4154,REF_4155)
list_3(ValidatorFacet.ValidatorListData[]) := phi(['list_2'])
REF_4151(ValidatorFacet.ValidatorListData) (->list_3) := TMP_13957(ValidatorFacet.ValidatorListData)
 i ++
TMP_13958(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 list
RETURN list_2
```
#### ValidatorFacet.getActiveValidatorCount() [EXTERNAL]
```slithir
 _countActiveValidators()
TMP_13959(uint256) = INTERNAL_CALL, ValidatorFacet._countActiveValidators()()
RETURN TMP_13959
 count
```
#### ValidatorFacet.getSlashVoteCount(uint16) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13960(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13960'])(PlumeStakingStorage.Layout) := TMP_13960(PlumeStakingStorage.Layout)
 ! $.validatorExists[validatorId]
REF_4157(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13960']).validatorExists
REF_4158(bool) -> REF_4157[validatorId_1]
TMP_13961 = UnaryType.BANG REF_4158 
CONDITION TMP_13961
 revert ValidatorDoesNotExist(uint16)(validatorId)
TMP_13962(None) = SOLIDITY_CALL revert ValidatorDoesNotExist(uint16)(validatorId_1)
 validVoteCount = 0
validVoteCount_1(uint256) := 0(uint256)
 currentTime = block.timestamp
currentTime_1(uint256) := block.timestamp(uint256)
 i = 0
i_1(uint256) := 0(uint256)
 i < $.validatorIds.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_4159(uint16[]) -> $_1 (-> ['TMP_13960']).validatorIds
REF_4160 -> LENGTH REF_4159
TMP_13963(bool) = i_2 < REF_4160
CONDITION TMP_13963
 voterValidatorId = $.validatorIds[i]
REF_4161(uint16[]) -> $_1 (-> ['TMP_13960']).validatorIds
REF_4162(uint16) -> REF_4161[i_2]
voterValidatorId_1(uint16) := REF_4162(uint16)
 voterValidatorId == validatorId
TMP_13964(bool) = voterValidatorId_1 == validatorId_1
CONDITION TMP_13964
 voteExpiration = $.slashingVotes[validatorId][voterValidatorId]
REF_4163(mapping(uint16 => mapping(uint16 => uint256))) -> $_1 (-> ['TMP_13960']).slashingVotes
REF_4164(mapping(uint16 => uint256)) -> REF_4163[validatorId_1]
REF_4165(uint256) -> REF_4164[voterValidatorId_1]
voteExpiration_1(uint256) := REF_4165(uint256)
 voteExpiration > 0
TMP_13965(bool) = voteExpiration_1 > 0
CONDITION TMP_13965
 voterValidator = $.validators[voterValidatorId]
REF_4166(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13960']).validators
REF_4167(PlumeStakingStorage.ValidatorInfo) -> REF_4166[voterValidatorId_1]
voterValidator_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_4167(PlumeStakingStorage.ValidatorInfo)
 voterIsEligible = voterValidator.active && ! voterValidator.slashed
REF_4168(bool) -> voterValidator_1 (-> ['$']).active
REF_4169(bool) -> voterValidator_1 (-> ['$']).slashed
TMP_13966 = UnaryType.BANG REF_4169 
TMP_13967(bool) = REF_4168 && TMP_13966
voterIsEligible_1(bool) := TMP_13967(bool)
 voteHasNotExpired = currentTime < voteExpiration
TMP_13968(bool) = currentTime_1 < voteExpiration_1
voteHasNotExpired_1(bool) := TMP_13968(bool)
 voterIsEligible && voteHasNotExpired
TMP_13969(bool) = voterIsEligible_1 && voteHasNotExpired_1
CONDITION TMP_13969
 validVoteCount ++
TMP_13970(uint256) := validVoteCount_1(uint256)
validVoteCount_2(uint256) = validVoteCount_1 (c)+ 1
validVoteCount_3(uint256) := phi(['validVoteCount_1', 'validVoteCount_2'])
 i ++
TMP_13971(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 validVoteCount
RETURN validVoteCount_1
```

#### PlumeRewardLogic.createRewardRateCheckpoint(PlumeStakingStorage.Layout,address,uint16,uint256) [INTERNAL]
```slithir
 updateRewardPerTokenForValidator($,token,validatorId)
INTERNAL_CALL, PlumeRewardLogic.updateRewardPerTokenForValidator(PlumeStakingStorage.Layout,address,uint16)($_1 (-> []),token_1,validatorId_1)
 currentCumulativeIndex = $.validatorRewardPerTokenCumulative[validatorId][token]
REF_4386(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> []).validatorRewardPerTokenCumulative
REF_4387(mapping(address => uint256)) -> REF_4386[validatorId_1]
REF_4388(uint256) -> REF_4387[token_1]
currentCumulativeIndex_1(uint256) := REF_4388(uint256)
 checkpoints = $.validatorRewardRateCheckpoints[validatorId][token]
REF_4389(mapping(uint16 => mapping(address => PlumeStakingStorage.RateCheckpoint[]))) -> $_1 (-> []).validatorRewardRateCheckpoints
REF_4390(mapping(address => PlumeStakingStorage.RateCheckpoint[])) -> REF_4389[validatorId_1]
REF_4391(PlumeStakingStorage.RateCheckpoint[]) -> REF_4390[token_1]
checkpoints_1 (-> [])(PlumeStakingStorage.RateCheckpoint[]) = ['REF_4391(PlumeStakingStorage.RateCheckpoint[])']
 len = checkpoints.length
REF_4392 -> LENGTH checkpoints_1 (-> [])
len_1(uint256) := REF_4392(uint256)
 checkpoint = PlumeStakingStorage.RateCheckpoint({timestamp:block.timestamp,rate:rate,cumulativeIndex:currentCumulativeIndex})
TMP_14190(PlumeStakingStorage.RateCheckpoint) = new RateCheckpoint(block.timestamp,rate_1,currentCumulativeIndex_1)
checkpoint_1(PlumeStakingStorage.RateCheckpoint) := TMP_14190(PlumeStakingStorage.RateCheckpoint)
 len > 0 && checkpoints[len - 1].timestamp == block.timestamp
TMP_14191(bool) = len_1 > 0
TMP_14192(uint256) = len_1 (c)- 1
REF_4394(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[TMP_14192]
REF_4395(uint256) -> REF_4394.timestamp
TMP_14193(bool) = REF_4395 == block.timestamp
TMP_14194(bool) = TMP_14191 && TMP_14193
CONDITION TMP_14194
 checkpoints[len - 1] = checkpoint
TMP_14195(uint256) = len_1 (c)- 1
REF_4396(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[TMP_14195]
checkpoints_2 (-> [])(PlumeStakingStorage.RateCheckpoint[]) := phi(['checkpoints_1 (-> [])'])
REF_4396(PlumeStakingStorage.RateCheckpoint) (->checkpoints_2 (-> [])) := checkpoint_1(PlumeStakingStorage.RateCheckpoint)
 checkpointIndex = len - 1
TMP_14196(uint256) = len_1 (c)- 1
checkpointIndex_1(uint256) := TMP_14196(uint256)
 checkpoints.push(checkpoint)
REF_4398 -> LENGTH checkpoints_1 (-> [])
TMP_14198(uint256) := REF_4398(uint256)
TMP_14199(uint256) = TMP_14198 (c)+ 1
checkpoints_3 (-> [])(PlumeStakingStorage.RateCheckpoint[]) := phi(['checkpoints_1 (-> [])'])
REF_4398(uint256) (->checkpoints_3 (-> [])) := TMP_14199(uint256)
REF_4399(PlumeStakingStorage.RateCheckpoint) -> checkpoints_3 (-> [])[TMP_14198]
checkpoints_4 (-> [])(PlumeStakingStorage.RateCheckpoint[]) := phi(['checkpoints_3 (-> [])'])
REF_4399(PlumeStakingStorage.RateCheckpoint) (->checkpoints_4 (-> [])) := checkpoint_1(PlumeStakingStorage.RateCheckpoint)
 checkpointIndex = len
checkpointIndex_2(uint256) := len_1(uint256)
checkpointIndex_3(uint256) := phi(['checkpointIndex_2', 'checkpointIndex_1'])
 RewardRateCheckpointCreated(token,validatorId,rate,block.timestamp,checkpointIndex,currentCumulativeIndex)
Emit RewardRateCheckpointCreated(token_1,validatorId_1,rate_1,block.timestamp,checkpointIndex_3,currentCumulativeIndex_1)
```
#### PlumeStakingStorage.layout() [INTERNAL]
```slithir
DIAMOND_STORAGE_POSITION_1(bytes32) := phi(['DIAMOND_STORAGE_POSITION_0'])
 position = DIAMOND_STORAGE_POSITION
position_1(bytes32) := DIAMOND_STORAGE_POSITION_1(bytes32)
 l = position
l_1 (-> ['position'])(PlumeStakingStorage.Layout) := position_1(bytes32)
 l
RETURN l_1 (-> ['position'])
```
#### PlumeRewardLogic._settleCommissionForValidatorUpToNow(PlumeStakingStorage.Layout,uint16) [INTERNAL]
```slithir
 rewardTokens = $.rewardTokens
REF_4413(address[]) -> $_1 (-> []).rewardTokens
rewardTokens_1(address[]) = ['REF_4413(address[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < rewardTokens.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_4414 -> LENGTH rewardTokens_1
TMP_14215(bool) = i_2 < REF_4414
CONDITION TMP_14215
 token = rewardTokens[i]
REF_4415(address) -> rewardTokens_1[i_2]
token_1(address) := REF_4415(address)
 updateRewardPerTokenForValidator($,token,validatorId)
INTERNAL_CALL, PlumeRewardLogic.updateRewardPerTokenForValidator(PlumeStakingStorage.Layout,address,uint16)($_1 (-> []),token_1,validatorId_1)
 i ++
TMP_14217(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
```
#### PlumeRewardLogic.createCommissionRateCheckpoint(PlumeStakingStorage.Layout,uint16,uint256) [INTERNAL]
```slithir
 checkpoints = $.validatorCommissionCheckpoints[validatorId]
REF_4400(mapping(uint16 => PlumeStakingStorage.RateCheckpoint[])) -> $_1 (-> []).validatorCommissionCheckpoints
REF_4401(PlumeStakingStorage.RateCheckpoint[]) -> REF_4400[validatorId_1]
checkpoints_1 (-> [])(PlumeStakingStorage.RateCheckpoint[]) = ['REF_4401(PlumeStakingStorage.RateCheckpoint[])']
 len = checkpoints.length
REF_4402 -> LENGTH checkpoints_1 (-> [])
len_1(uint256) := REF_4402(uint256)
 checkpoint = PlumeStakingStorage.RateCheckpoint({timestamp:block.timestamp,rate:commissionRate,cumulativeIndex:0})
TMP_14201(PlumeStakingStorage.RateCheckpoint) = new RateCheckpoint(block.timestamp,commissionRate_1,0)
checkpoint_1(PlumeStakingStorage.RateCheckpoint) := TMP_14201(PlumeStakingStorage.RateCheckpoint)
 len > 0 && checkpoints[len - 1].timestamp == block.timestamp
TMP_14202(bool) = len_1 > 0
TMP_14203(uint256) = len_1 (c)- 1
REF_4404(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[TMP_14203]
REF_4405(uint256) -> REF_4404.timestamp
TMP_14204(bool) = REF_4405 == block.timestamp
TMP_14205(bool) = TMP_14202 && TMP_14204
CONDITION TMP_14205
 checkpoints[len - 1] = checkpoint
TMP_14206(uint256) = len_1 (c)- 1
REF_4406(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[TMP_14206]
checkpoints_4 (-> [])(PlumeStakingStorage.RateCheckpoint[]) := phi(['checkpoints_1 (-> [])'])
REF_4406(PlumeStakingStorage.RateCheckpoint) (->checkpoints_4 (-> [])) := checkpoint_1(PlumeStakingStorage.RateCheckpoint)
 $.maxCommissionCheckpoints > 0 && len >= $.maxCommissionCheckpoints
REF_4407(uint16) -> $_1 (-> []).maxCommissionCheckpoints
TMP_14207(bool) = REF_4407 > 0
REF_4408(uint16) -> $_1 (-> []).maxCommissionCheckpoints
TMP_14208(bool) = len_1 >= REF_4408
TMP_14209(bool) = TMP_14207 && TMP_14208
CONDITION TMP_14209
 revert MaxCommissionCheckpointsExceeded(uint16,uint256)(validatorId,$.maxCommissionCheckpoints)
REF_4409(uint16) -> $_1 (-> []).maxCommissionCheckpoints
TMP_14210(None) = SOLIDITY_CALL revert MaxCommissionCheckpointsExceeded(uint16,uint256)(validatorId_1,REF_4409)
 checkpoints.push(checkpoint)
REF_4411 -> LENGTH checkpoints_1 (-> [])
TMP_14212(uint256) := REF_4411(uint256)
TMP_14213(uint256) = TMP_14212 (c)+ 1
checkpoints_2 (-> [])(PlumeStakingStorage.RateCheckpoint[]) := phi(['checkpoints_1 (-> [])'])
REF_4411(uint256) (->checkpoints_2 (-> [])) := TMP_14213(uint256)
REF_4412(PlumeStakingStorage.RateCheckpoint) -> checkpoints_2 (-> [])[TMP_14212]
checkpoints_3 (-> [])(PlumeStakingStorage.RateCheckpoint[]) := phi(['checkpoints_2 (-> [])'])
REF_4412(PlumeStakingStorage.RateCheckpoint) (->checkpoints_3 (-> [])) := checkpoint_1(PlumeStakingStorage.RateCheckpoint)
 ValidatorCommissionCheckpointCreated(validatorId,commissionRate,block.timestamp)
Emit ValidatorCommissionCheckpointCreated(validatorId_1,commissionRate_1,block.timestamp)
```
#### RewardsFacet.getTreasury() [EXTERNAL]
```slithir
 getTreasuryAddress()
TMP_13343(address) = INTERNAL_CALL, RewardsFacet.getTreasuryAddress()()
RETURN TMP_13343
```
#### IPlumeStakingRewardTreasury.distributeReward(address,uint256,address) [EXTERNAL]
```slithir

```
#### PlumeRewardLogic.updateRewardPerTokenForValidator(PlumeStakingStorage.Layout,address,uint16) [INTERNAL]
```slithir
$_1 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])', '$_1 (-> [])', '$_1 (-> [])', '$_1 (-> [])'])
token_1(address) := phi(['token_1', 'token_1', 'token_1', 'token_1'])
validatorId_1(uint16) := phi(['validatorId_1', 'validatorId_1', 'validatorId_1', 'validatorId_1'])
 validator = $.validators[validatorId]
REF_4252(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> []).validators
REF_4253(PlumeStakingStorage.ValidatorInfo) -> REF_4252[validatorId_1]
validator_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_4253(PlumeStakingStorage.ValidatorInfo)
 validator.slashed
REF_4254(bool) -> validator_1 (-> ['$']).slashed
CONDITION REF_4254
 $.validatorLastUpdateTimes[validatorId][token] = block.timestamp
REF_4255(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> []).validatorLastUpdateTimes
REF_4256(mapping(address => uint256)) -> REF_4255[validatorId_1]
REF_4257(uint256) -> REF_4256[token_1]
$_2 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])'])
REF_4257(uint256) (->$_2 (-> [])) := block.timestamp(uint256)
 $.validatorTotalStaked[validatorId] > 0
REF_4258(mapping(uint16 => uint256)) -> $_2 (-> []).validatorTotalStaked
REF_4259(uint256) -> REF_4258[validatorId_1]
TMP_14042(bool) = REF_4259 > 0
CONDITION TMP_14042
 revert InternalInconsistency(string)(Slashed validator has non-zero totalStaked)
TMP_14043(None) = SOLIDITY_CALL revert InternalInconsistency(string)(Slashed validator has non-zero totalStaked)
 ! validator.active
REF_4260(bool) -> validator_1 (-> ['$']).active
TMP_14044 = UnaryType.BANG REF_4260 
CONDITION TMP_14044
 $.validatorLastUpdateTimes[validatorId][token] = block.timestamp
REF_4261(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> []).validatorLastUpdateTimes
REF_4262(mapping(address => uint256)) -> REF_4261[validatorId_1]
REF_4263(uint256) -> REF_4262[token_1]
$_8 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])'])
REF_4263(uint256) (->$_8 (-> [])) := block.timestamp(uint256)
 totalStaked = $.validatorTotalStaked[validatorId]
REF_4264(mapping(uint16 => uint256)) -> $_1 (-> []).validatorTotalStaked
REF_4265(uint256) -> REF_4264[validatorId_1]
totalStaked_1(uint256) := REF_4265(uint256)
 oldLastUpdateTime = $.validatorLastUpdateTimes[validatorId][token]
REF_4266(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> []).validatorLastUpdateTimes
REF_4267(mapping(address => uint256)) -> REF_4266[validatorId_1]
REF_4268(uint256) -> REF_4267[token_1]
oldLastUpdateTime_1(uint256) := REF_4268(uint256)
 block.timestamp > oldLastUpdateTime
TMP_14045(bool) = block.timestamp > oldLastUpdateTime_1
CONDITION TMP_14045
 totalStaked > 0
TMP_14046(bool) = totalStaked_1 > 0
CONDITION TMP_14046
 timeDelta = block.timestamp - oldLastUpdateTime
TMP_14047(uint256) = block.timestamp (c)- oldLastUpdateTime_1
timeDelta_1(uint256) := TMP_14047(uint256)
 effectiveRewardRateChk = getEffectiveRewardRateAt($,token,validatorId,block.timestamp)
TMP_14048(PlumeStakingStorage.RateCheckpoint) = INTERNAL_CALL, PlumeRewardLogic.getEffectiveRewardRateAt(PlumeStakingStorage.Layout,address,uint16,uint256)($_1 (-> []),token_1,validatorId_1,block.timestamp)
effectiveRewardRateChk_1(PlumeStakingStorage.RateCheckpoint) := TMP_14048(PlumeStakingStorage.RateCheckpoint)
 effectiveRewardRate = effectiveRewardRateChk.rate
REF_4269(uint256) -> effectiveRewardRateChk_1.rate
effectiveRewardRate_1(uint256) := REF_4269(uint256)
 effectiveRewardRate > 0
TMP_14049(bool) = effectiveRewardRate_1 > 0
CONDITION TMP_14049
 rewardPerTokenIncrease = timeDelta * effectiveRewardRate
TMP_14050(uint256) = timeDelta_1 (c)* effectiveRewardRate_1
rewardPerTokenIncrease_1(uint256) := TMP_14050(uint256)
 $.validatorRewardPerTokenCumulative[validatorId][token] += rewardPerTokenIncrease
REF_4270(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> []).validatorRewardPerTokenCumulative
REF_4271(mapping(address => uint256)) -> REF_4270[validatorId_1]
REF_4272(uint256) -> REF_4271[token_1]
$_3 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])'])
REF_4272(-> $_3 (-> [])) = REF_4272 (c)+ rewardPerTokenIncrease_1
 commissionRateForSegment = getEffectiveCommissionRateAt($,validatorId,oldLastUpdateTime)
TMP_14051(uint256) = INTERNAL_CALL, PlumeRewardLogic.getEffectiveCommissionRateAt(PlumeStakingStorage.Layout,uint16,uint256)($_3 (-> []),validatorId_1,oldLastUpdateTime_1)
commissionRateForSegment_1(uint256) := TMP_14051(uint256)
 grossRewardForValidatorThisSegment = (totalStaked * rewardPerTokenIncrease) / PlumeStakingStorage.REWARD_PRECISION
TMP_14052(uint256) = totalStaked_1 (c)* rewardPerTokenIncrease_1
REF_4273(uint256) -> PlumeStakingStorage.REWARD_PRECISION
TMP_14053(uint256) = TMP_14052 (c)/ REF_4273
grossRewardForValidatorThisSegment_1(uint256) := TMP_14053(uint256)
 commissionDeltaForValidator = (grossRewardForValidatorThisSegment * commissionRateForSegment) / PlumeStakingStorage.REWARD_PRECISION
TMP_14054(uint256) = grossRewardForValidatorThisSegment_1 (c)* commissionRateForSegment_1
REF_4274(uint256) -> PlumeStakingStorage.REWARD_PRECISION
TMP_14055(uint256) = TMP_14054 (c)/ REF_4274
commissionDeltaForValidator_1(uint256) := TMP_14055(uint256)
 commissionDeltaForValidator > 0
TMP_14056(bool) = commissionDeltaForValidator_1 > 0
CONDITION TMP_14056
 $.validatorAccruedCommission[validatorId][token] += commissionDeltaForValidator
REF_4275(mapping(uint16 => mapping(address => uint256))) -> $_3 (-> []).validatorAccruedCommission
REF_4276(mapping(address => uint256)) -> REF_4275[validatorId_1]
REF_4277(uint256) -> REF_4276[token_1]
$_4 (-> [])(PlumeStakingStorage.Layout) := phi(['$_3 (-> [])'])
REF_4277(-> $_4 (-> [])) = REF_4277 (c)+ commissionDeltaForValidator_1
$_5 (-> [])(PlumeStakingStorage.Layout) := phi(['$_4 (-> [])', '$_3 (-> [])'])
$_6 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])', '$_3 (-> [])'])
 $.validatorLastUpdateTimes[validatorId][token] = block.timestamp
REF_4278(mapping(uint16 => mapping(address => uint256))) -> $_6 (-> []).validatorLastUpdateTimes
REF_4279(mapping(address => uint256)) -> REF_4278[validatorId_1]
REF_4280(uint256) -> REF_4279[token_1]
$_7 (-> [])(PlumeStakingStorage.Layout) := phi(['$_6 (-> [])'])
REF_4280(uint256) (->$_7 (-> [])) := block.timestamp(uint256)
```
#### RewardsFacet.getTreasuryAddress() [INTERNAL]
```slithir
TREASURY_STORAGE_POSITION_1(bytes32) := phi(['TREASURY_STORAGE_POSITION_0'])
 position = TREASURY_STORAGE_POSITION
position_1(bytes32) := TREASURY_STORAGE_POSITION_1(bytes32)
 treasuryAddress = sload(uint256)(position)
TMP_13147(uint256) = SOLIDITY_CALL sload(uint256)(position_1)
treasuryAddress_1(address) := TMP_13147(uint256)
 treasuryAddress
RETURN treasuryAddress_1
```
#### PlumeRewardLogic.getEffectiveCommissionRateAt(PlumeStakingStorage.Layout,uint16,uint256) [INTERNAL]
```slithir
$_1 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])', '$_3 (-> [])'])
validatorId_1(uint16) := phi(['validatorId_1', 'validatorId_1'])
timestamp_1(uint256) := phi(['segmentStartTime_1', 'oldLastUpdateTime_1'])
 checkpoints = $.validatorCommissionCheckpoints[validatorId]
REF_4365(mapping(uint16 => PlumeStakingStorage.RateCheckpoint[])) -> $_1 (-> []).validatorCommissionCheckpoints
REF_4366(PlumeStakingStorage.RateCheckpoint[]) -> REF_4365[validatorId_1]
checkpoints_1 (-> [])(PlumeStakingStorage.RateCheckpoint[]) = ['REF_4366(PlumeStakingStorage.RateCheckpoint[])']
 chkCount = checkpoints.length
REF_4367 -> LENGTH checkpoints_1 (-> [])
chkCount_1(uint256) := REF_4367(uint256)
 chkCount > 0
TMP_14164(bool) = chkCount_1 > 0
CONDITION TMP_14164
 idx = findCommissionCheckpointIndexAtOrBefore($,validatorId,timestamp)
TMP_14165(uint256) = INTERNAL_CALL, PlumeRewardLogic.findCommissionCheckpointIndexAtOrBefore(PlumeStakingStorage.Layout,uint16,uint256)($_1 (-> []),validatorId_1,timestamp_1)
idx_1(uint256) := TMP_14165(uint256)
 idx < chkCount && checkpoints[idx].timestamp <= timestamp
TMP_14166(bool) = idx_1 < chkCount_1
REF_4368(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[idx_1]
REF_4369(uint256) -> REF_4368.timestamp
TMP_14167(bool) = REF_4369 <= timestamp_1
TMP_14168(bool) = TMP_14166 && TMP_14167
CONDITION TMP_14168
 checkpoints[idx].rate
REF_4370(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[idx_1]
REF_4371(uint256) -> REF_4370.rate
RETURN REF_4371
 fallbackComm = $.validators[validatorId].commission
REF_4372(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> []).validators
REF_4373(PlumeStakingStorage.ValidatorInfo) -> REF_4372[validatorId_1]
REF_4374(uint256) -> REF_4373.commission
fallbackComm_1(uint256) := REF_4374(uint256)
 fallbackComm
RETURN fallbackComm_1
```
#### PlumeRewardLogic.getEffectiveRewardRateAt(PlumeStakingStorage.Layout,address,uint16,uint256) [INTERNAL]
```slithir
$_1 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])', '$_1 (-> [])', '$_1 (-> [])', '$_1 (-> [])'])
token_1(address) := phi(['token_1', 'token_1', 'token_1', 'token_1'])
validatorId_1(uint16) := phi(['validatorId_1', 'validatorId_1', 'validatorId_1', 'validatorId_1'])
timestamp_1(uint256) := phi(['segmentStartTime_1', 'block.timestamp', 'segmentStartTime_1', 'validatorLastUpdateTime_1'])
 checkpoints = $.validatorRewardRateCheckpoints[validatorId][token]
REF_4353(mapping(uint16 => mapping(address => PlumeStakingStorage.RateCheckpoint[]))) -> $_1 (-> []).validatorRewardRateCheckpoints
REF_4354(mapping(address => PlumeStakingStorage.RateCheckpoint[])) -> REF_4353[validatorId_1]
REF_4355(PlumeStakingStorage.RateCheckpoint[]) -> REF_4354[token_1]
checkpoints_1 (-> [])(PlumeStakingStorage.RateCheckpoint[]) = ['REF_4355(PlumeStakingStorage.RateCheckpoint[])']
 chkCount = checkpoints.length
REF_4356 -> LENGTH checkpoints_1 (-> [])
chkCount_1(uint256) := REF_4356(uint256)
 chkCount > 0
TMP_14154(bool) = chkCount_1 > 0
CONDITION TMP_14154
 idx = findRewardRateCheckpointIndexAtOrBefore($,validatorId,token,timestamp)
TMP_14155(uint256) = INTERNAL_CALL, PlumeRewardLogic.findRewardRateCheckpointIndexAtOrBefore(PlumeStakingStorage.Layout,uint16,address,uint256)($_1 (-> []),validatorId_1,token_1,timestamp_1)
idx_1(uint256) := TMP_14155(uint256)
 idx < chkCount && checkpoints[idx].timestamp <= timestamp
TMP_14156(bool) = idx_1 < chkCount_1
REF_4357(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[idx_1]
REF_4358(uint256) -> REF_4357.timestamp
TMP_14157(bool) = REF_4358 <= timestamp_1
TMP_14158(bool) = TMP_14156 && TMP_14157
CONDITION TMP_14158
 idx + 1 < chkCount && checkpoints[idx + 1].timestamp <= timestamp
TMP_14159(uint256) = idx_1 (c)+ 1
TMP_14160(bool) = TMP_14159 < chkCount_1
TMP_14161(uint256) = idx_1 (c)+ 1
REF_4359(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[TMP_14161]
REF_4360(uint256) -> REF_4359.timestamp
TMP_14162(bool) = REF_4360 <= timestamp_1
TMP_14163(bool) = TMP_14160 && TMP_14162
CONDITION TMP_14163
 checkpoints[idx]
REF_4361(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[idx_1]
RETURN REF_4361
 effectiveCheckpoint.rate = 0
REF_4362(uint256) -> effectiveCheckpoint_0.rate
effectiveCheckpoint_1(PlumeStakingStorage.RateCheckpoint) := phi(['effectiveCheckpoint_0'])
REF_4362(uint256) (->effectiveCheckpoint_1) := 0(uint256)
 effectiveCheckpoint.timestamp = timestamp
REF_4363(uint256) -> effectiveCheckpoint_1.timestamp
effectiveCheckpoint_2(PlumeStakingStorage.RateCheckpoint) := phi(['effectiveCheckpoint_1'])
REF_4363(uint256) (->effectiveCheckpoint_2) := timestamp_1(uint256)
 effectiveCheckpoint.cumulativeIndex = 0
REF_4364(uint256) -> effectiveCheckpoint_2.cumulativeIndex
effectiveCheckpoint_3(PlumeStakingStorage.RateCheckpoint) := phi(['effectiveCheckpoint_2'])
REF_4364(uint256) (->effectiveCheckpoint_3) := 0(uint256)
 effectiveCheckpoint
RETURN effectiveCheckpoint_3
 effectiveCheckpoint
```
