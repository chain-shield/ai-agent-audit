




#### ValidatorFacet.addValidator(uint16,uint256,address,address,string,string,address,uint256) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13555(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13555'])(PlumeStakingStorage.Layout) := TMP_13555(PlumeStakingStorage.Layout)
 $.validatorExists[validatorId]
REF_3700(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13555']).validatorExists
REF_3701(bool) -> REF_3700[validatorId_1]
CONDITION REF_3701
 revert ValidatorAlreadyExists(uint16)(validatorId)
TMP_13556(None) = SOLIDITY_CALL revert ValidatorAlreadyExists(uint16)(validatorId_1)
 l2AdminAddress == address(0)
TMP_13557 = CONVERT 0 to address
TMP_13558(bool) = l2AdminAddress_1 == TMP_13557
CONDITION TMP_13558
 revert ZeroAddress(string)(l2AdminAddress)
TMP_13559(None) = SOLIDITY_CALL revert ZeroAddress(string)(l2AdminAddress)
 l2WithdrawAddress == address(0)
TMP_13560 = CONVERT 0 to address
TMP_13561(bool) = l2WithdrawAddress_1 == TMP_13560
CONDITION TMP_13561
 revert ZeroAddress(string)(l2WithdrawAddress)
TMP_13562(None) = SOLIDITY_CALL revert ZeroAddress(string)(l2WithdrawAddress)
 commission > $.maxAllowedValidatorCommission
REF_3702(uint256) -> $_1 (-> ['TMP_13555']).maxAllowedValidatorCommission
TMP_13563(bool) = commission_1 > REF_3702
CONDITION TMP_13563
 revert CommissionExceedsMaxAllowed(uint256,uint256)(commission,$.maxAllowedValidatorCommission)
REF_3703(uint256) -> $_1 (-> ['TMP_13555']).maxAllowedValidatorCommission
TMP_13564(None) = SOLIDITY_CALL revert CommissionExceedsMaxAllowed(uint256,uint256)(commission_1,REF_3703)
 $.isAdminAssigned[l2AdminAddress]
REF_3704(mapping(address => bool)) -> $_1 (-> ['TMP_13555']).isAdminAssigned
REF_3705(bool) -> REF_3704[l2AdminAddress_1]
CONDITION REF_3705
 revert AdminAlreadyAssigned(address)(l2AdminAddress)
TMP_13565(None) = SOLIDITY_CALL revert AdminAlreadyAssigned(address)(l2AdminAddress_1)
 validator = $.validators[validatorId]
REF_3706(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13555']).validators
REF_3707(PlumeStakingStorage.ValidatorInfo) -> REF_3706[validatorId_1]
validator_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_3707(PlumeStakingStorage.ValidatorInfo)
 validator.validatorId = validatorId
REF_3708(uint16) -> validator_1 (-> ['$']).validatorId
validator_2 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_1 (-> ['$'])"])
REF_3708(uint16) (->validator_2 (-> ['$'])) := validatorId_1(uint16)
$_9 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_2 (-> ['$'])"])
 validator.commission = commission
REF_3709(uint256) -> validator_2 (-> ['$']).commission
validator_3 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_2 (-> ['$'])"])
REF_3709(uint256) (->validator_3 (-> ['$'])) := commission_1(uint256)
$_10 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_3 (-> ['$'])"])
 validator.delegatedAmount = 0
REF_3710(uint256) -> validator_3 (-> ['$']).delegatedAmount
validator_4 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_3 (-> ['$'])"])
REF_3710(uint256) (->validator_4 (-> ['$'])) := 0(uint256)
$_11 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_4 (-> ['$'])"])
 validator.l2AdminAddress = l2AdminAddress
REF_3711(address) -> validator_4 (-> ['$']).l2AdminAddress
validator_5 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_4 (-> ['$'])"])
REF_3711(address) (->validator_5 (-> ['$'])) := l2AdminAddress_1(address)
$_12 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_5 (-> ['$'])"])
 validator.l2WithdrawAddress = l2WithdrawAddress
REF_3712(address) -> validator_5 (-> ['$']).l2WithdrawAddress
validator_6 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_5 (-> ['$'])"])
REF_3712(address) (->validator_6 (-> ['$'])) := l2WithdrawAddress_1(address)
$_13 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_6 (-> ['$'])"])
 validator.l1ValidatorAddress = l1ValidatorAddress
REF_3713(string) -> validator_6 (-> ['$']).l1ValidatorAddress
validator_7 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_6 (-> ['$'])"])
REF_3713(string) (->validator_7 (-> ['$'])) := l1ValidatorAddress_1(string)
$_14 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_7 (-> ['$'])"])
 validator.l1AccountAddress = l1AccountAddress
REF_3714(string) -> validator_7 (-> ['$']).l1AccountAddress
validator_8 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_7 (-> ['$'])"])
REF_3714(string) (->validator_8 (-> ['$'])) := l1AccountAddress_1(string)
$_15 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_8 (-> ['$'])"])
 validator.l1AccountEvmAddress = l1AccountEvmAddress
REF_3715(address) -> validator_8 (-> ['$']).l1AccountEvmAddress
validator_9 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_8 (-> ['$'])"])
REF_3715(address) (->validator_9 (-> ['$'])) := l1AccountEvmAddress_1(address)
$_16 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_9 (-> ['$'])"])
 validator.active = true
REF_3716(bool) -> validator_9 (-> ['$']).active
validator_10 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_9 (-> ['$'])"])
REF_3716(bool) (->validator_10 (-> ['$'])) := True(bool)
$_17 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_10 (-> ['$'])"])
 validator.slashed = false
REF_3717(bool) -> validator_10 (-> ['$']).slashed
validator_11 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_10 (-> ['$'])"])
REF_3717(bool) (->validator_11 (-> ['$'])) := False(bool)
$_18 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_11 (-> ['$'])"])
 validator.maxCapacity = maxCapacity
REF_3718(uint256) -> validator_11 (-> ['$']).maxCapacity
validator_12 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_11 (-> ['$'])"])
REF_3718(uint256) (->validator_12 (-> ['$'])) := maxCapacity_1(uint256)
$_19 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_12 (-> ['$'])"])
 $.validatorIds.push(validatorId)
REF_3719(uint16[]) -> $_1 (-> ['TMP_13555']).validatorIds
REF_3721 -> LENGTH REF_3719
TMP_13567(uint256) := REF_3721(uint256)
TMP_13568(uint256) = TMP_13567 (c)+ 1
$_2 (-> ['TMP_13555'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13555'])"])
REF_3721(uint256) (->$_3 (-> ['TMP_13555'])) := TMP_13568(uint256)
REF_3722(uint16) -> REF_3719[TMP_13567]
$_3 (-> ['TMP_13555'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13555'])"])
REF_3722(uint16) (->$_3 (-> ['TMP_13555'])) := validatorId_1(uint16)
TMP_13555(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13555'])"])
 $.validatorExists[validatorId] = true
REF_3723(mapping(uint16 => bool)) -> $_3 (-> ['TMP_13555']).validatorExists
REF_3724(bool) -> REF_3723[validatorId_1]
$_4 (-> ['TMP_13555'])(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13555'])"])
REF_3724(bool) (->$_4 (-> ['TMP_13555'])) := True(bool)
TMP_13555(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13555'])"])
 $.adminToValidatorId[l2AdminAddress] = validatorId
REF_3725(mapping(address => uint16)) -> $_4 (-> ['TMP_13555']).adminToValidatorId
REF_3726(uint16) -> REF_3725[l2AdminAddress_1]
$_5 (-> ['TMP_13555'])(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13555'])"])
REF_3726(uint16) (->$_5 (-> ['TMP_13555'])) := validatorId_1(uint16)
TMP_13555(PlumeStakingStorage.Layout) := phi(["$_5 (-> ['TMP_13555'])"])
 $.isAdminAssigned[l2AdminAddress] = true
REF_3727(mapping(address => bool)) -> $_5 (-> ['TMP_13555']).isAdminAssigned
REF_3728(bool) -> REF_3727[l2AdminAddress_1]
$_6 (-> ['TMP_13555'])(PlumeStakingStorage.Layout) := phi(["$_5 (-> ['TMP_13555'])"])
REF_3728(bool) (->$_6 (-> ['TMP_13555'])) := True(bool)
TMP_13555(PlumeStakingStorage.Layout) := phi(["$_6 (-> ['TMP_13555'])"])
 rewardTokens = $.rewardTokens
REF_3729(address[]) -> $_6 (-> ['TMP_13555']).rewardTokens
rewardTokens_1(address[]) = ['REF_3729(address[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < rewardTokens.length
$_7 (-> ['TMP_13555'])(PlumeStakingStorage.Layout) := phi(["$_6 (-> ['TMP_13555'])", "$_8 (-> ['TMP_13555'])"])
i_2(uint256) := phi(['i_1', 'i_3'])
REF_3730 -> LENGTH rewardTokens_1
TMP_13569(bool) = i_2 < REF_3730
CONDITION TMP_13569
 token = rewardTokens[i]
REF_3731(address) -> rewardTokens_1[i_2]
token_1(address) := REF_3731(address)
 $.validatorLastUpdateTimes[validatorId][token] = block.timestamp
REF_3732(mapping(uint16 => mapping(address => uint256))) -> $_7 (-> ['TMP_13555']).validatorLastUpdateTimes
REF_3733(mapping(address => uint256)) -> REF_3732[validatorId_1]
REF_3734(uint256) -> REF_3733[token_1]
$_8 (-> ['TMP_13555'])(PlumeStakingStorage.Layout) := phi(["$_7 (-> ['TMP_13555'])"])
REF_3734(uint256) (->$_8 (-> ['TMP_13555'])) := block.timestamp(uint256)
TMP_13555(PlumeStakingStorage.Layout) := phi(["$_8 (-> ['TMP_13555'])"])
 i ++
TMP_13570(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 ValidatorAdded(validatorId,commission,l2AdminAddress,l2WithdrawAddress,l1ValidatorAddress,l1AccountAddress,l1AccountEvmAddress)
Emit ValidatorAdded(validatorId_1,commission_1,l2AdminAddress_1,l2WithdrawAddress_1,l1ValidatorAddress_1,l1AccountAddress_1,l1AccountEvmAddress_1)
 onlyRole(PlumeRoles.VALIDATOR_ROLE)
REF_3735(bytes32) -> PlumeRoles.VALIDATOR_ROLE
MODIFIER_CALL, ValidatorFacet.onlyRole(bytes32)(REF_3735)
 onlyRole(PlumeRoles.VALIDATOR_ROLE)
REF_3736(bytes32) -> PlumeRoles.VALIDATOR_ROLE
MODIFIER_CALL, ValidatorFacet.onlyRole(bytes32)(REF_3736)
```
#### ValidatorFacet.setValidatorCapacity(uint16,uint256) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13574(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13574'])(PlumeStakingStorage.Layout) := TMP_13574(PlumeStakingStorage.Layout)
 validator = $.validators[validatorId]
REF_3738(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13574']).validators
REF_3739(PlumeStakingStorage.ValidatorInfo) -> REF_3738[validatorId_1]
validator_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_3739(PlumeStakingStorage.ValidatorInfo)
 ! validator.active || validator.slashed
REF_3740(bool) -> validator_1 (-> ['$']).active
TMP_13575 = UnaryType.BANG REF_3740 
REF_3741(bool) -> validator_1 (-> ['$']).slashed
TMP_13576(bool) = TMP_13575 || REF_3741
CONDITION TMP_13576
 revert ValidatorInactive(uint16)(validatorId)
TMP_13577(None) = SOLIDITY_CALL revert ValidatorInactive(uint16)(validatorId_1)
 oldCapacity = validator.maxCapacity
REF_3742(uint256) -> validator_1 (-> ['$']).maxCapacity
oldCapacity_1(uint256) := REF_3742(uint256)
 validator.maxCapacity = maxCapacity
REF_3743(uint256) -> validator_1 (-> ['$']).maxCapacity
validator_2 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_1 (-> ['$'])"])
REF_3743(uint256) (->validator_2 (-> ['$'])) := maxCapacity_1(uint256)
$_2 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_2 (-> ['$'])"])
 ValidatorCapacityUpdated(validatorId,oldCapacity,maxCapacity)
Emit ValidatorCapacityUpdated(validatorId_1,oldCapacity_1,maxCapacity_1)
 onlyRole(PlumeRoles.VALIDATOR_ROLE)
REF_3744(bytes32) -> PlumeRoles.VALIDATOR_ROLE
MODIFIER_CALL, ValidatorFacet.onlyRole(bytes32)(REF_3744)
 onlyRole(PlumeRoles.VALIDATOR_ROLE)
REF_3745(bytes32) -> PlumeRoles.VALIDATOR_ROLE
MODIFIER_CALL, ValidatorFacet.onlyRole(bytes32)(REF_3745)
 _validateValidatorExists(validatorId)
MODIFIER_CALL, ValidatorFacet._validateValidatorExists(uint16)(validatorId_1)
```
#### ValidatorFacet.setValidatorStatus(uint16,bool) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13582(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13582'])(PlumeStakingStorage.Layout) := TMP_13582(PlumeStakingStorage.Layout)
 validator = $.validators[validatorId]
REF_3747(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13582']).validators
REF_3748(PlumeStakingStorage.ValidatorInfo) -> REF_3747[validatorId_1]
validator_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_3748(PlumeStakingStorage.ValidatorInfo)
 newActiveStatus && validator.slashed
REF_3749(bool) -> validator_1 (-> ['$']).slashed
TMP_13583(bool) = newActiveStatus_1 && REF_3749
CONDITION TMP_13583
 revert ValidatorAlreadySlashed(uint16)(validatorId)
TMP_13584(None) = SOLIDITY_CALL revert ValidatorAlreadySlashed(uint16)(validatorId_1)
 validator.active = newActiveStatus
REF_3750(bool) -> validator_1 (-> ['$']).active
validator_2 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_1 (-> ['$'])"])
REF_3750(bool) (->validator_2 (-> ['$'])) := newActiveStatus_1(bool)
$_2 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_2 (-> ['$'])"])
 ValidatorStatusUpdated(validatorId,newActiveStatus,validator.slashed)
REF_3751(bool) -> validator_2 (-> ['$']).slashed
Emit ValidatorStatusUpdated(validatorId_1,newActiveStatus_1,REF_3751)
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_3752(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ValidatorFacet.onlyRole(bytes32)(REF_3752)
 onlyRole(PlumeRoles.ADMIN_ROLE)
REF_3753(bytes32) -> PlumeRoles.ADMIN_ROLE
MODIFIER_CALL, ValidatorFacet.onlyRole(bytes32)(REF_3753)
 _validateValidatorExists(validatorId)
MODIFIER_CALL, ValidatorFacet._validateValidatorExists(uint16)(validatorId_1)
```
#### ValidatorFacet.setValidatorCommission(uint16,uint256) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13589(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13589'])(PlumeStakingStorage.Layout) := TMP_13589(PlumeStakingStorage.Layout)
 validator = $.validators[validatorId]
REF_3755(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13589']).validators
REF_3756(PlumeStakingStorage.ValidatorInfo) -> REF_3755[validatorId_1]
validator_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_3756(PlumeStakingStorage.ValidatorInfo)
 ! validator.active || validator.slashed
REF_3757(bool) -> validator_1 (-> ['$']).active
TMP_13590 = UnaryType.BANG REF_3757 
REF_3758(bool) -> validator_1 (-> ['$']).slashed
TMP_13591(bool) = TMP_13590 || REF_3758
CONDITION TMP_13591
 revert ValidatorInactive(uint16)(validatorId)
TMP_13592(None) = SOLIDITY_CALL revert ValidatorInactive(uint16)(validatorId_1)
 newCommission > $.maxAllowedValidatorCommission
REF_3759(uint256) -> $_1 (-> ['TMP_13589']).maxAllowedValidatorCommission
TMP_13593(bool) = newCommission_1 > REF_3759
CONDITION TMP_13593
 revert CommissionExceedsMaxAllowed(uint256,uint256)(newCommission,$.maxAllowedValidatorCommission)
REF_3760(uint256) -> $_1 (-> ['TMP_13589']).maxAllowedValidatorCommission
TMP_13594(None) = SOLIDITY_CALL revert CommissionExceedsMaxAllowed(uint256,uint256)(newCommission_1,REF_3760)
 oldCommission = validator.commission
REF_3761(uint256) -> validator_1 (-> ['$']).commission
oldCommission_1(uint256) := REF_3761(uint256)
 oldCommission != newCommission
TMP_13595(bool) = oldCommission_1 != newCommission_1
CONDITION TMP_13595
 PlumeRewardLogic._settleCommissionForValidatorUpToNow($,validatorId)
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic._settleCommissionForValidatorUpToNow(PlumeStakingStorage.Layout,uint16), arguments:["$_1 (-> ['TMP_13589'])", 'validatorId_1'] 
 validator.commission = newCommission
REF_3763(uint256) -> validator_1 (-> ['$']).commission
validator_2 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_1 (-> ['$'])"])
REF_3763(uint256) (->validator_2 (-> ['$'])) := newCommission_1(uint256)
$_2 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_2 (-> ['$'])"])
 PlumeRewardLogic.createCommissionRateCheckpoint($,validatorId,newCommission)
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.createCommissionRateCheckpoint(PlumeStakingStorage.Layout,uint16,uint256), arguments:["$_1 (-> ['TMP_13589'])", 'validatorId_1', 'newCommission_1'] 
 validator.commission = newCommission
REF_3765(uint256) -> validator_1 (-> ['$']).commission
validator_3 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_1 (-> ['$'])"])
REF_3765(uint256) (->validator_3 (-> ['$'])) := newCommission_1(uint256)
$_3 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_3 (-> ['$'])"])
 ValidatorCommissionSet(validatorId,oldCommission,newCommission)
Emit ValidatorCommissionSet(validatorId_1,oldCommission_1,newCommission_1)
 onlyValidatorAdmin(validatorId)
MODIFIER_CALL, ValidatorFacet.onlyValidatorAdmin(uint16)(validatorId_1)
```
#### ValidatorFacet.setValidatorAddresses(uint16,address,address,string,string,address) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13600(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13600'])(PlumeStakingStorage.Layout) := TMP_13600(PlumeStakingStorage.Layout)
 validator = $.validators[validatorId]
REF_3767(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13600']).validators
REF_3768(PlumeStakingStorage.ValidatorInfo) -> REF_3767[validatorId_1]
validator_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_3768(PlumeStakingStorage.ValidatorInfo)
 ! validator.active || validator.slashed
REF_3769(bool) -> validator_1 (-> ['$']).active
TMP_13601 = UnaryType.BANG REF_3769 
REF_3770(bool) -> validator_1 (-> ['$']).slashed
TMP_13602(bool) = TMP_13601 || REF_3770
CONDITION TMP_13602
 revert ValidatorInactive(uint16)(validatorId)
TMP_13603(None) = SOLIDITY_CALL revert ValidatorInactive(uint16)(validatorId_1)
 oldL2AdminAddress = validator.l2AdminAddress
REF_3771(address) -> validator_1 (-> ['$']).l2AdminAddress
oldL2AdminAddress_1(address) := REF_3771(address)
 oldL2WithdrawAddress = validator.l2WithdrawAddress
REF_3772(address) -> validator_1 (-> ['$']).l2WithdrawAddress
oldL2WithdrawAddress_1(address) := REF_3772(address)
 oldL1ValidatorAddress = validator.l1ValidatorAddress
REF_3773(string) -> validator_1 (-> ['$']).l1ValidatorAddress
oldL1ValidatorAddress_1(string) := REF_3773(string)
 oldL1AccountAddress = validator.l1AccountAddress
REF_3774(string) -> validator_1 (-> ['$']).l1AccountAddress
oldL1AccountAddress_1(string) := REF_3774(string)
 oldL1AccountEvmAddress = validator.l1AccountEvmAddress
REF_3775(address) -> validator_1 (-> ['$']).l1AccountEvmAddress
oldL1AccountEvmAddress_1(address) := REF_3775(address)
 newL2AdminAddress != address(0) && newL2AdminAddress != validator.l2AdminAddress
TMP_13604 = CONVERT 0 to address
TMP_13605(bool) = newL2AdminAddress_1 != TMP_13604
REF_3776(address) -> validator_1 (-> ['$']).l2AdminAddress
TMP_13606(bool) = newL2AdminAddress_1 != REF_3776
TMP_13607(bool) = TMP_13605 && TMP_13606
CONDITION TMP_13607
 $.isAdminAssigned[newL2AdminAddress]
REF_3777(mapping(address => bool)) -> $_1 (-> ['TMP_13600']).isAdminAssigned
REF_3778(bool) -> REF_3777[newL2AdminAddress_1]
CONDITION REF_3778
 revert AdminAlreadyAssigned(address)(newL2AdminAddress)
TMP_13608(None) = SOLIDITY_CALL revert AdminAlreadyAssigned(address)(newL2AdminAddress_1)
 currentAdminAddress = validator.l2AdminAddress
REF_3779(address) -> validator_1 (-> ['$']).l2AdminAddress
currentAdminAddress_1(address) := REF_3779(address)
 validator.l2AdminAddress = newL2AdminAddress
REF_3780(address) -> validator_1 (-> ['$']).l2AdminAddress
validator_2 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_1 (-> ['$'])"])
REF_3780(address) (->validator_2 (-> ['$'])) := newL2AdminAddress_1(address)
$_9 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_2 (-> ['$'])"])
 delete $.adminToValidatorId[currentAdminAddress]
REF_3781(mapping(address => uint16)) -> $_1 (-> ['TMP_13600']).adminToValidatorId
REF_3782(uint16) -> REF_3781[currentAdminAddress_1]
REF_3781 = delete REF_3782 
 $.adminToValidatorId[newL2AdminAddress] = validatorId
REF_3783(mapping(address => uint16)) -> $_1 (-> ['TMP_13600']).adminToValidatorId
REF_3784(uint16) -> REF_3783[newL2AdminAddress_1]
$_2 (-> ['TMP_13600'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13600'])"])
REF_3784(uint16) (->$_2 (-> ['TMP_13600'])) := validatorId_1(uint16)
TMP_13600(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13600'])"])
 $.isAdminAssigned[currentAdminAddress] = false
REF_3785(mapping(address => bool)) -> $_2 (-> ['TMP_13600']).isAdminAssigned
REF_3786(bool) -> REF_3785[currentAdminAddress_1]
$_3 (-> ['TMP_13600'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13600'])"])
REF_3786(bool) (->$_3 (-> ['TMP_13600'])) := False(bool)
TMP_13600(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13600'])"])
 $.isAdminAssigned[newL2AdminAddress] = true
REF_3787(mapping(address => bool)) -> $_3 (-> ['TMP_13600']).isAdminAssigned
REF_3788(bool) -> REF_3787[newL2AdminAddress_1]
$_4 (-> ['TMP_13600'])(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13600'])"])
REF_3788(bool) (->$_4 (-> ['TMP_13600'])) := True(bool)
TMP_13600(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13600'])"])
validator_3 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_2 (-> ['$'])", "validator_1 (-> ['$'])"])
 newL2WithdrawAddress != address(0) && newL2WithdrawAddress != validator.l2WithdrawAddress
TMP_13609 = CONVERT 0 to address
TMP_13610(bool) = newL2WithdrawAddress_1 != TMP_13609
REF_3789(address) -> validator_3 (-> ['$']).l2WithdrawAddress
TMP_13611(bool) = newL2WithdrawAddress_1 != REF_3789
TMP_13612(bool) = TMP_13610 && TMP_13611
CONDITION TMP_13612
 newL2WithdrawAddress == address(0)
TMP_13613 = CONVERT 0 to address
TMP_13614(bool) = newL2WithdrawAddress_1 == TMP_13613
CONDITION TMP_13614
 revert ZeroAddress(string)(newL2WithdrawAddress)
TMP_13615(None) = SOLIDITY_CALL revert ZeroAddress(string)(newL2WithdrawAddress)
 validator.l2WithdrawAddress = newL2WithdrawAddress
REF_3790(address) -> validator_3 (-> ['$']).l2WithdrawAddress
validator_4 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_3 (-> ['$'])"])
REF_3790(address) (->validator_4 (-> ['$'])) := newL2WithdrawAddress_1(address)
$_5 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_4 (-> ['$'])"])
validator_5 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_4 (-> ['$'])", "validator_1 (-> ['$'])"])
 bytes(newL1ValidatorAddress).length > 0
TMP_13616 = CONVERT newL1ValidatorAddress_1 to bytes
REF_3791 -> LENGTH TMP_13616
TMP_13617(bool) = REF_3791 > 0
CONDITION TMP_13617
 validator.l1ValidatorAddress = newL1ValidatorAddress
REF_3792(string) -> validator_5 (-> ['$']).l1ValidatorAddress
validator_6 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_5 (-> ['$'])"])
REF_3792(string) (->validator_6 (-> ['$'])) := newL1ValidatorAddress_1(string)
$_6 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_6 (-> ['$'])"])
validator_7 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_6 (-> ['$'])", "validator_1 (-> ['$'])"])
 bytes(newL1AccountAddress).length > 0
TMP_13618 = CONVERT newL1AccountAddress_1 to bytes
REF_3793 -> LENGTH TMP_13618
TMP_13619(bool) = REF_3793 > 0
CONDITION TMP_13619
 validator.l1AccountAddress = newL1AccountAddress
REF_3794(string) -> validator_7 (-> ['$']).l1AccountAddress
validator_8 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_7 (-> ['$'])"])
REF_3794(string) (->validator_8 (-> ['$'])) := newL1AccountAddress_1(string)
$_7 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_8 (-> ['$'])"])
validator_9 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_8 (-> ['$'])", "validator_1 (-> ['$'])"])
 newL1AccountEvmAddress != address(0)
TMP_13620 = CONVERT 0 to address
TMP_13621(bool) = newL1AccountEvmAddress_1 != TMP_13620
CONDITION TMP_13621
 validator.l1AccountEvmAddress = newL1AccountEvmAddress
REF_3795(address) -> validator_9 (-> ['$']).l1AccountEvmAddress
validator_10 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_9 (-> ['$'])"])
REF_3795(address) (->validator_10 (-> ['$'])) := newL1AccountEvmAddress_1(address)
$_8 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validator_10 (-> ['$'])"])
validator_11 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validator_10 (-> ['$'])", "validator_1 (-> ['$'])"])
 ValidatorAddressesSet(validatorId,oldL2AdminAddress,validator.l2AdminAddress,oldL2WithdrawAddress,validator.l2WithdrawAddress,oldL1ValidatorAddress,validator.l1ValidatorAddress,oldL1AccountAddress,validator.l1AccountAddress,oldL1AccountEvmAddress,validator.l1AccountEvmAddress)
REF_3796(address) -> validator_11 (-> ['$']).l2AdminAddress
REF_3797(address) -> validator_11 (-> ['$']).l2WithdrawAddress
REF_3798(string) -> validator_11 (-> ['$']).l1ValidatorAddress
REF_3799(string) -> validator_11 (-> ['$']).l1AccountAddress
REF_3800(address) -> validator_11 (-> ['$']).l1AccountEvmAddress
Emit ValidatorAddressesSet(validatorId_1,oldL2AdminAddress_1,REF_3796,oldL2WithdrawAddress_1,REF_3797,oldL1ValidatorAddress_1,REF_3798,oldL1AccountAddress_1,REF_3799,oldL1AccountEvmAddress_1,REF_3800)
 onlyValidatorAdmin(validatorId)
MODIFIER_CALL, ValidatorFacet.onlyValidatorAdmin(uint16)(validatorId_1)
```
#### ValidatorFacet.requestCommissionClaim(uint16,address) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13624(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13624'])(PlumeStakingStorage.Layout) := TMP_13624(PlumeStakingStorage.Layout)
 validator = $.validators[validatorId]
REF_3802(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13624']).validators
REF_3803(PlumeStakingStorage.ValidatorInfo) -> REF_3802[validatorId_1]
validator_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_3803(PlumeStakingStorage.ValidatorInfo)
 ! validator.active || validator.slashed
REF_3804(bool) -> validator_1 (-> ['$']).active
TMP_13625 = UnaryType.BANG REF_3804 
REF_3805(bool) -> validator_1 (-> ['$']).slashed
TMP_13626(bool) = TMP_13625 || REF_3805
CONDITION TMP_13626
 revert ValidatorInactive(uint16)(validatorId)
TMP_13627(None) = SOLIDITY_CALL revert ValidatorInactive(uint16)(validatorId_1)
 amount = $.validatorAccruedCommission[validatorId][token]
REF_3806(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> ['TMP_13624']).validatorAccruedCommission
REF_3807(mapping(address => uint256)) -> REF_3806[validatorId_1]
REF_3808(uint256) -> REF_3807[token_1]
amount_1(uint256) := REF_3808(uint256)
 amount == 0
TMP_13628(bool) = amount_1 == 0
CONDITION TMP_13628
 revert InvalidAmount(uint256)(0)
TMP_13629(None) = SOLIDITY_CALL revert InvalidAmount(uint256)(0)
 $.pendingCommissionClaims[validatorId][token].amount > 0
REF_3809(mapping(uint16 => mapping(address => PlumeStakingStorage.PendingCommissionClaim))) -> $_1 (-> ['TMP_13624']).pendingCommissionClaims
REF_3810(mapping(address => PlumeStakingStorage.PendingCommissionClaim)) -> REF_3809[validatorId_1]
REF_3811(PlumeStakingStorage.PendingCommissionClaim) -> REF_3810[token_1]
REF_3812(uint256) -> REF_3811.amount
TMP_13630(bool) = REF_3812 > 0
CONDITION TMP_13630
 revert PendingClaimExists(uint16,address)(validatorId,token)
TMP_13631(None) = SOLIDITY_CALL revert PendingClaimExists(uint16,address)(validatorId_1,token_1)
 recipient = validator.l2WithdrawAddress
REF_3813(address) -> validator_1 (-> ['$']).l2WithdrawAddress
recipient_1(address) := REF_3813(address)
 nowTs = block.timestamp
nowTs_1(uint256) := block.timestamp(uint256)
 $.pendingCommissionClaims[validatorId][token] = PlumeStakingStorage.PendingCommissionClaim({amount:amount,requestTimestamp:nowTs,token:token,recipient:recipient})
REF_3814(mapping(uint16 => mapping(address => PlumeStakingStorage.PendingCommissionClaim))) -> $_1 (-> ['TMP_13624']).pendingCommissionClaims
REF_3815(mapping(address => PlumeStakingStorage.PendingCommissionClaim)) -> REF_3814[validatorId_1]
REF_3816(PlumeStakingStorage.PendingCommissionClaim) -> REF_3815[token_1]
TMP_13632(PlumeStakingStorage.PendingCommissionClaim) = new PendingCommissionClaim(amount_1,nowTs_1,token_1,recipient_1)
$_2 (-> ['TMP_13624'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13624'])"])
REF_3816(PlumeStakingStorage.PendingCommissionClaim) (->$_2 (-> ['TMP_13624'])) := TMP_13632(PlumeStakingStorage.PendingCommissionClaim)
TMP_13624(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13624'])"])
 $.validatorAccruedCommission[validatorId][token] = 0
REF_3818(mapping(uint16 => mapping(address => uint256))) -> $_2 (-> ['TMP_13624']).validatorAccruedCommission
REF_3819(mapping(address => uint256)) -> REF_3818[validatorId_1]
REF_3820(uint256) -> REF_3819[token_1]
$_3 (-> ['TMP_13624'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13624'])"])
REF_3820(uint256) (->$_3 (-> ['TMP_13624'])) := 0(uint256)
TMP_13624(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13624'])"])
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
TMP_13638(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13638'])(PlumeStakingStorage.Layout) := TMP_13638(PlumeStakingStorage.Layout)
 validator = $.validators[validatorId]
REF_3822(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13638']).validators
REF_3823(PlumeStakingStorage.ValidatorInfo) -> REF_3822[validatorId_1]
validator_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_3823(PlumeStakingStorage.ValidatorInfo)
 ! validator.active || validator.slashed
REF_3824(bool) -> validator_1 (-> ['$']).active
TMP_13639 = UnaryType.BANG REF_3824 
REF_3825(bool) -> validator_1 (-> ['$']).slashed
TMP_13640(bool) = TMP_13639 || REF_3825
CONDITION TMP_13640
 revert ValidatorInactive(uint16)(validatorId)
TMP_13641(None) = SOLIDITY_CALL revert ValidatorInactive(uint16)(validatorId_1)
 claim = $.pendingCommissionClaims[validatorId][token]
REF_3826(mapping(uint16 => mapping(address => PlumeStakingStorage.PendingCommissionClaim))) -> $_1 (-> ['TMP_13638']).pendingCommissionClaims
REF_3827(mapping(address => PlumeStakingStorage.PendingCommissionClaim)) -> REF_3826[validatorId_1]
REF_3828(PlumeStakingStorage.PendingCommissionClaim) -> REF_3827[token_1]
claim_1 (-> ['$'])(PlumeStakingStorage.PendingCommissionClaim) := REF_3828(PlumeStakingStorage.PendingCommissionClaim)
 claim.amount == 0
REF_3829(uint256) -> claim_1 (-> ['$']).amount
TMP_13642(bool) = REF_3829 == 0
CONDITION TMP_13642
 revert NoPendingClaim(uint16,address)(validatorId,token)
TMP_13643(None) = SOLIDITY_CALL revert NoPendingClaim(uint16,address)(validatorId_1,token_1)
 readyTimestamp = claim.requestTimestamp + PlumeStakingStorage.COMMISSION_CLAIM_TIMELOCK
REF_3830(uint256) -> claim_1 (-> ['$']).requestTimestamp
REF_3831(uint256) -> PlumeStakingStorage.COMMISSION_CLAIM_TIMELOCK
TMP_13644(uint256) = REF_3830 (c)+ REF_3831
readyTimestamp_1(uint256) := TMP_13644(uint256)
 block.timestamp < readyTimestamp
TMP_13645(bool) = block.timestamp < readyTimestamp_1
CONDITION TMP_13645
 revert ClaimNotReady(uint16,address,uint256)(validatorId,token,readyTimestamp)
TMP_13646(None) = SOLIDITY_CALL revert ClaimNotReady(uint16,address,uint256)(validatorId_1,token_1,readyTimestamp_1)
 amount = claim.amount
REF_3832(uint256) -> claim_1 (-> ['$']).amount
amount_1(uint256) := REF_3832(uint256)
 recipient = claim.recipient
REF_3833(address) -> claim_1 (-> ['$']).recipient
recipient_1(address) := REF_3833(address)
 delete $.pendingCommissionClaims[validatorId][token]
REF_3834(mapping(uint16 => mapping(address => PlumeStakingStorage.PendingCommissionClaim))) -> $_1 (-> ['TMP_13638']).pendingCommissionClaims
REF_3835(mapping(address => PlumeStakingStorage.PendingCommissionClaim)) -> REF_3834[validatorId_1]
REF_3836(PlumeStakingStorage.PendingCommissionClaim) -> REF_3835[token_1]
REF_3835 = delete REF_3836 
 treasury = RewardsFacet(address(this)).getTreasury()
TMP_13647 = CONVERT this to address
TMP_13648 = CONVERT TMP_13647 to RewardsFacet
TMP_13649(address) = HIGH_LEVEL_CALL, dest:TMP_13648(RewardsFacet), function:getTreasury, arguments:[]  
treasury_1(address) := TMP_13649(address)
 treasury == address(0)
TMP_13650 = CONVERT 0 to address
TMP_13651(bool) = treasury_1 == TMP_13650
CONDITION TMP_13651
 revert TreasuryNotSet()()
TMP_13652(None) = SOLIDITY_CALL revert TreasuryNotSet()()
 IPlumeStakingRewardTreasury(treasury).distributeReward(token,amount,recipient)
TMP_13653 = CONVERT treasury_1 to IPlumeStakingRewardTreasury
HIGH_LEVEL_CALL, dest:TMP_13653(IPlumeStakingRewardTreasury), function:distributeReward, arguments:['token_1', 'amount_1', 'recipient_1']  
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
validatorId_1(uint16) := phi(['validatorId_1', 'maliciousValidatorId_1', 'validatorId_1'])
 $ = PlumeStakingStorage.layout()
TMP_13658(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13658'])(PlumeStakingStorage.Layout) := TMP_13658(PlumeStakingStorage.Layout)
 voteCount = $.slashVoteCounts[validatorId]
REF_3840(mapping(uint16 => uint256)) -> $_1 (-> ['TMP_13658']).slashVoteCounts
REF_3841(uint256) -> REF_3840[validatorId_1]
voteCount_1(uint256) := REF_3841(uint256)
 voteCount == 0
TMP_13659(bool) = voteCount_1 == 0
CONDITION TMP_13659
 0
RETURN 0
 allValidatorIds = $.validatorIds
REF_3842(uint16[]) -> $_1 (-> ['TMP_13658']).validatorIds
allValidatorIds_1(uint16[]) = ['REF_3842(uint16[])']
 newActiveVoteCount_scope_0 = 0
newActiveVoteCount_scope_0_1(uint256) := 0(uint256)
 i = 0
i_1(uint256) := 0(uint256)
 i < allValidatorIds.length
i_2(uint256) := phi(['i_3', 'i_1'])
REF_3843 -> LENGTH allValidatorIds_1
TMP_13660(bool) = i_2 < REF_3843
CONDITION TMP_13660
 voterValidatorId = allValidatorIds[i]
REF_3844(uint16) -> allValidatorIds_1[i_2]
voterValidatorId_1(uint16) := REF_3844(uint16)
 voterValidatorId == validatorId
TMP_13661(bool) = voterValidatorId_1 == validatorId_1
CONDITION TMP_13661
 voteExpiration = $.slashingVotes[validatorId][voterValidatorId]
REF_3845(mapping(uint16 => mapping(uint16 => uint256))) -> $_1 (-> ['TMP_13658']).slashingVotes
REF_3846(mapping(uint16 => uint256)) -> REF_3845[validatorId_1]
REF_3847(uint256) -> REF_3846[voterValidatorId_1]
voteExpiration_1(uint256) := REF_3847(uint256)
 voteExpiration > 0
TMP_13662(bool) = voteExpiration_1 > 0
CONDITION TMP_13662
 voterValidator = $.validators[voterValidatorId]
REF_3848(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13658']).validators
REF_3849(PlumeStakingStorage.ValidatorInfo) -> REF_3848[voterValidatorId_1]
voterValidator_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_3849(PlumeStakingStorage.ValidatorInfo)
 voterIsEligible = voterValidator.active && ! voterValidator.slashed
REF_3850(bool) -> voterValidator_1 (-> ['$']).active
REF_3851(bool) -> voterValidator_1 (-> ['$']).slashed
TMP_13663 = UnaryType.BANG REF_3851 
TMP_13664(bool) = REF_3850 && TMP_13663
voterIsEligible_1(bool) := TMP_13664(bool)
 voteHasExpired = block.timestamp >= voteExpiration
TMP_13665(bool) = block.timestamp >= voteExpiration_1
voteHasExpired_1(bool) := TMP_13665(bool)
 voterIsEligible && ! voteHasExpired
TMP_13666 = UnaryType.BANG voteHasExpired_1 
TMP_13667(bool) = voterIsEligible_1 && TMP_13666
CONDITION TMP_13667
 newActiveVoteCount_scope_0 ++
TMP_13668(uint256) := newActiveVoteCount_scope_0_1(uint256)
newActiveVoteCount_scope_0_2(uint256) = newActiveVoteCount_scope_0_1 (c)+ 1
 delete $.slashingVotes[validatorId][voterValidatorId]
REF_3852(mapping(uint16 => mapping(uint16 => uint256))) -> $_1 (-> ['TMP_13658']).slashingVotes
REF_3853(mapping(uint16 => uint256)) -> REF_3852[validatorId_1]
REF_3854(uint256) -> REF_3853[voterValidatorId_1]
REF_3853 = delete REF_3854 
$_2 (-> ['TMP_13658'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13658'])", "$_1 (-> ['TMP_13658'])"])
newActiveVoteCount_scope_0_3(uint256) := phi(['newActiveVoteCount_scope_0_2', 'newActiveVoteCount_scope_0_1'])
 i ++
TMP_13669(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 $.slashVoteCounts[validatorId] = newActiveVoteCount_scope_0
REF_3855(mapping(uint16 => uint256)) -> $_1 (-> ['TMP_13658']).slashVoteCounts
REF_3856(uint256) -> REF_3855[validatorId_1]
$_3 (-> ['TMP_13658'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13658'])"])
REF_3856(uint256) (->$_3 (-> ['TMP_13658'])) := newActiveVoteCount_scope_0_1(uint256)
TMP_13658(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13658'])"])
 newActiveVoteCount_scope_0
RETURN newActiveVoteCount_scope_0_1
 newActiveVoteCount
```
#### ValidatorFacet.voteToSlashValidator(uint16,uint256) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13670(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13670'])(PlumeStakingStorage.Layout) := TMP_13670(PlumeStakingStorage.Layout)
 voterAdmin = msg.sender
voterAdmin_1(address) := msg.sender(address)
 voterValidatorId = $.adminToValidatorId[voterAdmin]
REF_3858(mapping(address => uint16)) -> $_1 (-> ['TMP_13670']).adminToValidatorId
REF_3859(uint16) -> REF_3858[voterAdmin_1]
voterValidatorId_1(uint16) := REF_3859(uint16)
 $.validators[voterValidatorId].l2AdminAddress != voterAdmin || ! $.validators[voterValidatorId].active
REF_3860(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13670']).validators
REF_3861(PlumeStakingStorage.ValidatorInfo) -> REF_3860[voterValidatorId_1]
REF_3862(address) -> REF_3861.l2AdminAddress
TMP_13671(bool) = REF_3862 != voterAdmin_1
REF_3863(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13670']).validators
REF_3864(PlumeStakingStorage.ValidatorInfo) -> REF_3863[voterValidatorId_1]
REF_3865(bool) -> REF_3864.active
TMP_13672 = UnaryType.BANG REF_3865 
TMP_13673(bool) = TMP_13671 || TMP_13672
CONDITION TMP_13673
 revert NotValidatorAdmin(address)(voterAdmin)
TMP_13674(None) = SOLIDITY_CALL revert NotValidatorAdmin(address)(voterAdmin_1)
 targetValidator = $.validators[maliciousValidatorId]
REF_3866(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13670']).validators
REF_3867(PlumeStakingStorage.ValidatorInfo) -> REF_3866[maliciousValidatorId_1]
targetValidator_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_3867(PlumeStakingStorage.ValidatorInfo)
 ! $.validatorExists[maliciousValidatorId]
REF_3868(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13670']).validatorExists
REF_3869(bool) -> REF_3868[maliciousValidatorId_1]
TMP_13675 = UnaryType.BANG REF_3869 
CONDITION TMP_13675
 revert ValidatorDoesNotExist(uint16)(maliciousValidatorId)
TMP_13676(None) = SOLIDITY_CALL revert ValidatorDoesNotExist(uint16)(maliciousValidatorId_1)
 targetValidator.slashed
REF_3870(bool) -> targetValidator_1 (-> ['$']).slashed
CONDITION REF_3870
 revert ValidatorAlreadySlashed(uint16)(maliciousValidatorId)
TMP_13677(None) = SOLIDITY_CALL revert ValidatorAlreadySlashed(uint16)(maliciousValidatorId_1)
 ! targetValidator.active
REF_3871(bool) -> targetValidator_1 (-> ['$']).active
TMP_13678 = UnaryType.BANG REF_3871 
CONDITION TMP_13678
 revert ValidatorInactive(uint16)(maliciousValidatorId)
TMP_13679(None) = SOLIDITY_CALL revert ValidatorInactive(uint16)(maliciousValidatorId_1)
 voterValidatorId == maliciousValidatorId
TMP_13680(bool) = voterValidatorId_1 == maliciousValidatorId_1
CONDITION TMP_13680
 revert CannotVoteForSelf()()
TMP_13681(None) = SOLIDITY_CALL revert CannotVoteForSelf()()
 voteExpiration <= block.timestamp || $.maxSlashVoteDurationInSeconds == 0 || voteExpiration > block.timestamp + $.maxSlashVoteDurationInSeconds
TMP_13682(bool) = voteExpiration_1 <= block.timestamp
REF_3872(uint256) -> $_1 (-> ['TMP_13670']).maxSlashVoteDurationInSeconds
TMP_13683(bool) = REF_3872 == 0
TMP_13684(bool) = TMP_13682 || TMP_13683
REF_3873(uint256) -> $_1 (-> ['TMP_13670']).maxSlashVoteDurationInSeconds
TMP_13685(uint256) = block.timestamp (c)+ REF_3873
TMP_13686(bool) = voteExpiration_1 > TMP_13685
TMP_13687(bool) = TMP_13684 || TMP_13686
CONDITION TMP_13687
 revert SlashVoteDurationTooLong()()
TMP_13688(None) = SOLIDITY_CALL revert SlashVoteDurationTooLong()()
 currentVoteExpiration = $.slashingVotes[maliciousValidatorId][voterValidatorId]
REF_3874(mapping(uint16 => mapping(uint16 => uint256))) -> $_1 (-> ['TMP_13670']).slashingVotes
REF_3875(mapping(uint16 => uint256)) -> REF_3874[maliciousValidatorId_1]
REF_3876(uint256) -> REF_3875[voterValidatorId_1]
currentVoteExpiration_1(uint256) := REF_3876(uint256)
 currentVoteExpiration >= block.timestamp
TMP_13689(bool) = currentVoteExpiration_1 >= block.timestamp
CONDITION TMP_13689
 revert AlreadyVotedToSlash(uint16,uint16)(maliciousValidatorId,voterValidatorId)
TMP_13690(None) = SOLIDITY_CALL revert AlreadyVotedToSlash(uint16,uint16)(maliciousValidatorId_1,voterValidatorId_1)
 _cleanupExpiredVotes(maliciousValidatorId)
TMP_13691(uint256) = INTERNAL_CALL, ValidatorFacet._cleanupExpiredVotes(uint16)(maliciousValidatorId_1)
 $.slashingVotes[maliciousValidatorId][voterValidatorId] = voteExpiration
REF_3877(mapping(uint16 => mapping(uint16 => uint256))) -> $_1 (-> ['TMP_13670']).slashingVotes
REF_3878(mapping(uint16 => uint256)) -> REF_3877[maliciousValidatorId_1]
REF_3879(uint256) -> REF_3878[voterValidatorId_1]
$_2 (-> ['TMP_13670'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13670'])"])
REF_3879(uint256) (->$_2 (-> ['TMP_13670'])) := voteExpiration_1(uint256)
TMP_13670(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13670'])"])
 $.slashVoteCounts[maliciousValidatorId] ++
REF_3880(mapping(uint16 => uint256)) -> $_2 (-> ['TMP_13670']).slashVoteCounts
REF_3881(uint256) -> REF_3880[maliciousValidatorId_1]
TMP_13692(uint256) := REF_3881(uint256)
$_3 (-> ['TMP_13670'])(PlumeStakingStorage.Layout) := phi(["$_2 (-> ['TMP_13670'])"])
REF_3881(-> $_3 (-> ['TMP_13670'])) = REF_3881 (c)+ 1
TMP_13670(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13670'])"])
 SlashVoteCast(maliciousValidatorId,voterValidatorId,voteExpiration)
Emit SlashVoteCast(maliciousValidatorId_1,voterValidatorId_1,voteExpiration_1)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardUpgradeable.nonReentrant()()
```
#### ValidatorFacet.slashValidator(uint16) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13695(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13695'])(PlumeStakingStorage.Layout) := TMP_13695(PlumeStakingStorage.Layout)
 ! $.validatorExists[validatorId]
REF_3883(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13695']).validatorExists
REF_3884(bool) -> REF_3883[validatorId_1]
TMP_13696 = UnaryType.BANG REF_3884 
CONDITION TMP_13696
 revert ValidatorDoesNotExist(uint16)(validatorId)
TMP_13697(None) = SOLIDITY_CALL revert ValidatorDoesNotExist(uint16)(validatorId_1)
 validatorToSlash = $.validators[validatorId]
REF_3885(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13695']).validators
REF_3886(PlumeStakingStorage.ValidatorInfo) -> REF_3885[validatorId_1]
validatorToSlash_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_3886(PlumeStakingStorage.ValidatorInfo)
 validatorToSlash.slashed
REF_3887(bool) -> validatorToSlash_1 (-> ['$']).slashed
CONDITION REF_3887
 revert ValidatorAlreadySlashed(uint16)(validatorId)
TMP_13698(None) = SOLIDITY_CALL revert ValidatorAlreadySlashed(uint16)(validatorId_1)
 validVotesAgainst = _cleanupExpiredVotes(validatorId)
TMP_13699(uint256) = INTERNAL_CALL, ValidatorFacet._cleanupExpiredVotes(uint16)(validatorId_1)
validVotesAgainst_1(uint256) := TMP_13699(uint256)
 otherActiveNonSlashedValidators = 0
otherActiveNonSlashedValidators_1(uint256) := 0(uint256)
 i = 0
i_1(uint256) := 0(uint256)
 i < $.validatorIds.length
i_2(uint256) := phi(['i_3', 'i_1'])
REF_3888(uint16[]) -> $_1 (-> ['TMP_13695']).validatorIds
REF_3889 -> LENGTH REF_3888
TMP_13700(bool) = i_2 < REF_3889
CONDITION TMP_13700
 currentValId = $.validatorIds[i]
REF_3890(uint16[]) -> $_1 (-> ['TMP_13695']).validatorIds
REF_3891(uint16) -> REF_3890[i_2]
currentValId_1(uint16) := REF_3891(uint16)
 currentValId == validatorId
TMP_13701(bool) = currentValId_1 == validatorId_1
CONDITION TMP_13701
 $.validators[currentValId].active && ! $.validators[currentValId].slashed
REF_3892(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13695']).validators
REF_3893(PlumeStakingStorage.ValidatorInfo) -> REF_3892[currentValId_1]
REF_3894(bool) -> REF_3893.active
REF_3895(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13695']).validators
REF_3896(PlumeStakingStorage.ValidatorInfo) -> REF_3895[currentValId_1]
REF_3897(bool) -> REF_3896.slashed
TMP_13702 = UnaryType.BANG REF_3897 
TMP_13703(bool) = REF_3894 && TMP_13702
CONDITION TMP_13703
 otherActiveNonSlashedValidators ++
TMP_13704(uint256) := otherActiveNonSlashedValidators_1(uint256)
otherActiveNonSlashedValidators_2(uint256) = otherActiveNonSlashedValidators_1 (c)+ 1
otherActiveNonSlashedValidators_3(uint256) := phi(['otherActiveNonSlashedValidators_2', 'otherActiveNonSlashedValidators_1'])
 i ++
TMP_13705(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 otherActiveNonSlashedValidators == 0
TMP_13706(bool) = otherActiveNonSlashedValidators_1 == 0
CONDITION TMP_13706
 $.validatorIds.length > 1
REF_3898(uint16[]) -> $_1 (-> ['TMP_13695']).validatorIds
REF_3899 -> LENGTH REF_3898
TMP_13707(bool) = REF_3899 > 1
CONDITION TMP_13707
 validVotesAgainst < otherActiveNonSlashedValidators
TMP_13708(bool) = validVotesAgainst_1 < otherActiveNonSlashedValidators_1
CONDITION TMP_13708
 revert UnanimityNotReached(uint256,uint256)(validVotesAgainst,otherActiveNonSlashedValidators)
TMP_13709(None) = SOLIDITY_CALL revert UnanimityNotReached(uint256,uint256)(validVotesAgainst_1,otherActiveNonSlashedValidators_1)
 i_scope_0 = 0
i_scope_0_1(uint256) := 0(uint256)
 i_scope_0 < $.validatorIds.length
i_scope_0_2(uint256) := phi(['i_scope_0_1', 'i_scope_0_3'])
REF_3900(uint16[]) -> $_1 (-> ['TMP_13695']).validatorIds
REF_3901 -> LENGTH REF_3900
TMP_13710(bool) = i_scope_0_2 < REF_3901
CONDITION TMP_13710
 voterValidatorId = $.validatorIds[i_scope_0]
REF_3902(uint16[]) -> $_1 (-> ['TMP_13695']).validatorIds
REF_3903(uint16) -> REF_3902[i_scope_0_2]
voterValidatorId_1(uint16) := REF_3903(uint16)
 voterValidatorId != validatorId
TMP_13711(bool) = voterValidatorId_1 != validatorId_1
CONDITION TMP_13711
 delete $.slashingVotes[validatorId][voterValidatorId]
REF_3904(mapping(uint16 => mapping(uint16 => uint256))) -> $_1 (-> ['TMP_13695']).slashingVotes
REF_3905(mapping(uint16 => uint256)) -> REF_3904[validatorId_1]
REF_3906(uint256) -> REF_3905[voterValidatorId_1]
REF_3905 = delete REF_3906 
$_2 (-> ['TMP_13695'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13695'])", "$_1 (-> ['TMP_13695'])"])
 i_scope_0 ++
TMP_13712(uint256) := i_scope_0_2(uint256)
i_scope_0_3(uint256) = i_scope_0_2 (c)+ 1
 $.slashVoteCounts[validatorId] = 0
REF_3907(mapping(uint16 => uint256)) -> $_1 (-> ['TMP_13695']).slashVoteCounts
REF_3908(uint256) -> REF_3907[validatorId_1]
$_3 (-> ['TMP_13695'])(PlumeStakingStorage.Layout) := phi(["$_1 (-> ['TMP_13695'])"])
REF_3908(uint256) (->$_3 (-> ['TMP_13695'])) := 0(uint256)
TMP_13695(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13695'])"])
 stakersToPreserve = $.validatorStakers[validatorId]
REF_3909(mapping(uint16 => address[])) -> $_3 (-> ['TMP_13695']).validatorStakers
REF_3910(address[]) -> REF_3909[validatorId_1]
stakersToPreserve_1(address[]) = ['REF_3910(address[])']
 rewardTokens = $.rewardTokens
REF_3911(address[]) -> $_3 (-> ['TMP_13695']).rewardTokens
rewardTokens_1(address[]) = ['REF_3911(address[])']
 i_scope_1 = 0
i_scope_1_1(uint256) := 0(uint256)
 i_scope_1 < stakersToPreserve.length
i_scope_1_2(uint256) := phi(['i_scope_1_3', 'i_scope_1_1'])
REF_3912 -> LENGTH stakersToPreserve_1
TMP_13713(bool) = i_scope_1_2 < REF_3912
CONDITION TMP_13713
 staker = stakersToPreserve[i_scope_1]
REF_3913(address) -> stakersToPreserve_1[i_scope_1_2]
staker_1(address) := REF_3913(address)
 userStakedAmount = $.userValidatorStakes[staker][validatorId].staked
REF_3914(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_3 (-> ['TMP_13695']).userValidatorStakes
REF_3915(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_3914[staker_1]
REF_3916(PlumeStakingStorage.UserValidatorStake) -> REF_3915[validatorId_1]
REF_3917(uint256) -> REF_3916.staked
userStakedAmount_1(uint256) := REF_3917(uint256)
 userStakedAmount > 0
TMP_13714(bool) = userStakedAmount_1 > 0
CONDITION TMP_13714
 j = 0
j_1(uint256) := 0(uint256)
 j < rewardTokens.length
$_14 (-> ['TMP_13695'])(PlumeStakingStorage.Layout) := phi(["$_20 (-> ['TMP_13695'])", "$_3 (-> ['TMP_13695'])"])
j_2(uint256) := phi(['j_1', 'j_3'])
REF_3918 -> LENGTH rewardTokens_1
TMP_13715(bool) = j_2 < REF_3918
CONDITION TMP_13715
 token = rewardTokens[j]
REF_3919(address) -> rewardTokens_1[j_2]
token_1(address) := REF_3919(address)
 PlumeRewardLogic.updateRewardPerTokenForValidator($,token,validatorId)
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.updateRewardPerTokenForValidator(PlumeStakingStorage.Layout,address,uint16), arguments:["$_14 (-> ['TMP_13695'])", 'token_1', 'validatorId_1'] 
 (userRewardDelta,None,None) = PlumeRewardLogic.calculateRewardsWithCheckpoints($,staker,validatorId,token,userStakedAmount)
TUPLE_91(uint256,uint256,uint256) = LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic.calculateRewardsWithCheckpoints(PlumeStakingStorage.Layout,address,uint16,address,uint256), arguments:["$_14 (-> ['TMP_13695'])", 'staker_1', 'validatorId_1', 'token_1', 'userStakedAmount_1'] 
userRewardDelta_1(uint256)= UNPACK TUPLE_91 index: 0 
 userRewardDelta > 0
TMP_13717(bool) = userRewardDelta_1 > 0
CONDITION TMP_13717
 $.userRewards[staker][validatorId][token] += userRewardDelta
REF_3922(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_14 (-> ['TMP_13695']).userRewards
REF_3923(mapping(uint16 => mapping(address => uint256))) -> REF_3922[staker_1]
REF_3924(mapping(address => uint256)) -> REF_3923[validatorId_1]
REF_3925(uint256) -> REF_3924[token_1]
$_15 (-> ['TMP_13695'])(PlumeStakingStorage.Layout) := phi(["$_14 (-> ['TMP_13695'])"])
REF_3925(-> $_15 (-> ['TMP_13695'])) = REF_3925 (c)+ userRewardDelta_1
TMP_13695(PlumeStakingStorage.Layout) := phi(["$_15 (-> ['TMP_13695'])"])
 $.totalClaimableByToken[token] += userRewardDelta
REF_3926(mapping(address => uint256)) -> $_15 (-> ['TMP_13695']).totalClaimableByToken
REF_3927(uint256) -> REF_3926[token_1]
$_16 (-> ['TMP_13695'])(PlumeStakingStorage.Layout) := phi(["$_15 (-> ['TMP_13695'])"])
REF_3927(-> $_16 (-> ['TMP_13695'])) = REF_3927 (c)+ userRewardDelta_1
TMP_13695(PlumeStakingStorage.Layout) := phi(["$_16 (-> ['TMP_13695'])"])
 $.userHasPendingRewards[staker][validatorId] = true
REF_3928(mapping(address => mapping(uint16 => bool))) -> $_16 (-> ['TMP_13695']).userHasPendingRewards
REF_3929(mapping(uint16 => bool)) -> REF_3928[staker_1]
REF_3930(bool) -> REF_3929[validatorId_1]
$_17 (-> ['TMP_13695'])(PlumeStakingStorage.Layout) := phi(["$_16 (-> ['TMP_13695'])"])
REF_3930(bool) (->$_17 (-> ['TMP_13695'])) := True(bool)
TMP_13695(PlumeStakingStorage.Layout) := phi(["$_17 (-> ['TMP_13695'])"])
$_18 (-> ['TMP_13695'])(PlumeStakingStorage.Layout) := phi(["$_17 (-> ['TMP_13695'])", "$_3 (-> ['TMP_13695'])"])
 $.userValidatorRewardPerTokenPaid[staker][validatorId][token] = $.validatorRewardPerTokenCumulative[validatorId][token]
REF_3931(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_18 (-> ['TMP_13695']).userValidatorRewardPerTokenPaid
REF_3932(mapping(uint16 => mapping(address => uint256))) -> REF_3931[staker_1]
REF_3933(mapping(address => uint256)) -> REF_3932[validatorId_1]
REF_3934(uint256) -> REF_3933[token_1]
REF_3935(mapping(uint16 => mapping(address => uint256))) -> $_18 (-> ['TMP_13695']).validatorRewardPerTokenCumulative
REF_3936(mapping(address => uint256)) -> REF_3935[validatorId_1]
REF_3937(uint256) -> REF_3936[token_1]
$_19 (-> ['TMP_13695'])(PlumeStakingStorage.Layout) := phi(["$_18 (-> ['TMP_13695'])"])
REF_3934(uint256) (->$_19 (-> ['TMP_13695'])) := REF_3937(uint256)
TMP_13695(PlumeStakingStorage.Layout) := phi(["$_19 (-> ['TMP_13695'])"])
 $.userValidatorRewardPerTokenPaidTimestamp[staker][validatorId][token] = block.timestamp
REF_3938(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_19 (-> ['TMP_13695']).userValidatorRewardPerTokenPaidTimestamp
REF_3939(mapping(uint16 => mapping(address => uint256))) -> REF_3938[staker_1]
REF_3940(mapping(address => uint256)) -> REF_3939[validatorId_1]
REF_3941(uint256) -> REF_3940[token_1]
$_20 (-> ['TMP_13695'])(PlumeStakingStorage.Layout) := phi(["$_19 (-> ['TMP_13695'])"])
REF_3941(uint256) (->$_20 (-> ['TMP_13695'])) := block.timestamp(uint256)
TMP_13695(PlumeStakingStorage.Layout) := phi(["$_20 (-> ['TMP_13695'])"])
 j ++
TMP_13718(uint256) := j_2(uint256)
j_3(uint256) = j_2 (c)+ 1
 i_scope_1 ++
TMP_13719(uint256) := i_scope_1_2(uint256)
i_scope_1_3(uint256) = i_scope_1_2 (c)+ 1
 validatorToSlash.active = false
REF_3942(bool) -> validatorToSlash_1 (-> ['$']).active
validatorToSlash_2 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validatorToSlash_1 (-> ['$'])"])
REF_3942(bool) (->validatorToSlash_2 (-> ['$'])) := False(bool)
$_21 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validatorToSlash_2 (-> ['$'])"])
 validatorToSlash.slashed = true
REF_3943(bool) -> validatorToSlash_2 (-> ['$']).slashed
validatorToSlash_3 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validatorToSlash_2 (-> ['$'])"])
REF_3943(bool) (->validatorToSlash_3 (-> ['$'])) := True(bool)
$_22 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validatorToSlash_3 (-> ['$'])"])
 validatorToSlash.slashedAtTimestamp = block.timestamp
REF_3944(uint256) -> validatorToSlash_3 (-> ['$']).slashedAtTimestamp
validatorToSlash_4 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validatorToSlash_3 (-> ['$'])"])
REF_3944(uint256) (->validatorToSlash_4 (-> ['$'])) := block.timestamp(uint256)
$_23 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validatorToSlash_4 (-> ['$'])"])
 stakeLost = $.validatorTotalStaked[validatorId]
REF_3945(mapping(uint16 => uint256)) -> $_3 (-> ['TMP_13695']).validatorTotalStaked
REF_3946(uint256) -> REF_3945[validatorId_1]
stakeLost_1(uint256) := REF_3946(uint256)
 cooledLost = $.validatorTotalCooling[validatorId]
REF_3947(mapping(uint16 => uint256)) -> $_3 (-> ['TMP_13695']).validatorTotalCooling
REF_3948(uint256) -> REF_3947[validatorId_1]
cooledLost_1(uint256) := REF_3948(uint256)
 $.validatorTotalStaked[validatorId] = 0
REF_3949(mapping(uint16 => uint256)) -> $_9 (-> ['TMP_13695']).validatorTotalStaked
REF_3950(uint256) -> REF_3949[validatorId_1]
$_10 (-> ['TMP_13695'])(PlumeStakingStorage.Layout) := phi(["$_9 (-> ['TMP_13695'])"])
REF_3950(uint256) (->$_10 (-> ['TMP_13695'])) := 0(uint256)
TMP_13695(PlumeStakingStorage.Layout) := phi(["$_10 (-> ['TMP_13695'])"])
 $.validatorTotalCooling[validatorId] = 0
REF_3951(mapping(uint16 => uint256)) -> $_10 (-> ['TMP_13695']).validatorTotalCooling
REF_3952(uint256) -> REF_3951[validatorId_1]
$_11 (-> ['TMP_13695'])(PlumeStakingStorage.Layout) := phi(["$_10 (-> ['TMP_13695'])"])
REF_3952(uint256) (->$_11 (-> ['TMP_13695'])) := 0(uint256)
TMP_13695(PlumeStakingStorage.Layout) := phi(["$_11 (-> ['TMP_13695'])"])
 validatorToSlash.delegatedAmount = 0
REF_3953(uint256) -> validatorToSlash_4 (-> ['$']).delegatedAmount
validatorToSlash_5 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := phi(["validatorToSlash_4 (-> ['$'])"])
REF_3953(uint256) (->validatorToSlash_5 (-> ['$'])) := 0(uint256)
$_24 (-> ['$'])(PlumeStakingStorage.Layout) := phi(["validatorToSlash_5 (-> ['$'])"])
 delete $.validatorStakers[validatorId]
REF_3954(mapping(uint16 => address[])) -> $_11 (-> ['TMP_13695']).validatorStakers
REF_3955(address[]) -> REF_3954[validatorId_1]
REF_3954 = delete REF_3955 
 j_scope_2 = 0
j_scope_2_1(uint256) := 0(uint256)
 j_scope_2 < rewardTokens.length
j_scope_2_2(uint256) := phi(['j_scope_2_1', 'j_scope_2_3'])
REF_3956 -> LENGTH rewardTokens_1
TMP_13720(bool) = j_scope_2_2 < REF_3956
CONDITION TMP_13720
 token_scope_3 = rewardTokens[j_scope_2]
REF_3957(address) -> rewardTokens_1[j_scope_2_2]
token_scope_3_1(address) := REF_3957(address)
 $.pendingCommissionClaims[validatorId][token_scope_3].amount > 0
REF_3958(mapping(uint16 => mapping(address => PlumeStakingStorage.PendingCommissionClaim))) -> $_11 (-> ['TMP_13695']).pendingCommissionClaims
REF_3959(mapping(address => PlumeStakingStorage.PendingCommissionClaim)) -> REF_3958[validatorId_1]
REF_3960(PlumeStakingStorage.PendingCommissionClaim) -> REF_3959[token_scope_3_1]
REF_3961(uint256) -> REF_3960.amount
TMP_13721(bool) = REF_3961 > 0
CONDITION TMP_13721
 delete $.pendingCommissionClaims[validatorId][token_scope_3]
REF_3962(mapping(uint16 => mapping(address => PlumeStakingStorage.PendingCommissionClaim))) -> $_11 (-> ['TMP_13695']).pendingCommissionClaims
REF_3963(mapping(address => PlumeStakingStorage.PendingCommissionClaim)) -> REF_3962[validatorId_1]
REF_3964(PlumeStakingStorage.PendingCommissionClaim) -> REF_3963[token_scope_3_1]
REF_3963 = delete REF_3964 
$_13 (-> ['TMP_13695'])(PlumeStakingStorage.Layout) := phi(["$_11 (-> ['TMP_13695'])", "$_11 (-> ['TMP_13695'])"])
 j_scope_2 ++
TMP_13722(uint256) := j_scope_2_2(uint256)
j_scope_2_3(uint256) = j_scope_2_2 (c)+ 1
 $.adminToValidatorId[validatorToSlash.l2AdminAddress] == validatorId
REF_3965(mapping(address => uint16)) -> $_11 (-> ['TMP_13695']).adminToValidatorId
REF_3966(address) -> validatorToSlash_5 (-> ['$']).l2AdminAddress
REF_3967(uint16) -> REF_3965[REF_3966]
TMP_13723(bool) = REF_3967 == validatorId_1
CONDITION TMP_13723
 delete $.adminToValidatorId[validatorToSlash.l2AdminAddress]
REF_3968(mapping(address => uint16)) -> $_11 (-> ['TMP_13695']).adminToValidatorId
REF_3969(address) -> validatorToSlash_5 (-> ['$']).l2AdminAddress
REF_3970(uint16) -> REF_3968[REF_3969]
REF_3968 = delete REF_3970 
 $.isAdminAssigned[validatorToSlash.l2AdminAddress] = false
REF_3971(mapping(address => bool)) -> $_11 (-> ['TMP_13695']).isAdminAssigned
REF_3972(address) -> validatorToSlash_5 (-> ['$']).l2AdminAddress
REF_3973(bool) -> REF_3971[REF_3972]
$_12 (-> ['TMP_13695'])(PlumeStakingStorage.Layout) := phi(["$_11 (-> ['TMP_13695'])"])
REF_3973(bool) (->$_12 (-> ['TMP_13695'])) := False(bool)
TMP_13695(PlumeStakingStorage.Layout) := phi(["$_12 (-> ['TMP_13695'])"])
 ValidatorSlashed(validatorId,msg.sender,stakeLost + cooledLost)
TMP_13724(uint256) = stakeLost_1 (c)+ cooledLost_1
Emit ValidatorSlashed(validatorId_1,msg.sender,TMP_13724)
 ValidatorStatusUpdated(validatorId,false,true)
Emit ValidatorStatusUpdated(validatorId_1,False,True)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardUpgradeable.nonReentrant()()
 onlyRole(PlumeRoles.TIMELOCK_ROLE)
REF_3974(bytes32) -> PlumeRoles.TIMELOCK_ROLE
MODIFIER_CALL, ValidatorFacet.onlyRole(bytes32)(REF_3974)
 onlyRole(PlumeRoles.TIMELOCK_ROLE)
REF_3975(bytes32) -> PlumeRoles.TIMELOCK_ROLE
MODIFIER_CALL, ValidatorFacet.onlyRole(bytes32)(REF_3975)
 otherActiveNonSlashedValidators > 0
TMP_13730(bool) = otherActiveNonSlashedValidators_1 > 0
CONDITION TMP_13730
 revert UnanimityNotReached(uint256,uint256)(validVotesAgainst,otherActiveNonSlashedValidators)
TMP_13731(None) = SOLIDITY_CALL revert UnanimityNotReached(uint256,uint256)(validVotesAgainst_1,otherActiveNonSlashedValidators_1)
 revert UnanimityNotReached(uint256,uint256)(validVotesAgainst,1)
TMP_13732(None) = SOLIDITY_CALL revert UnanimityNotReached(uint256,uint256)(validVotesAgainst_1,1)
 $.totalStaked >= stakeLost
REF_3976(uint256) -> $_3 (-> ['TMP_13695']).totalStaked
TMP_13733(bool) = REF_3976 >= stakeLost_1
CONDITION TMP_13733
 $.totalStaked = $.totalStaked - stakeLost
REF_3977(uint256) -> $_3 (-> ['TMP_13695']).totalStaked
REF_3978(uint256) -> $_3 (-> ['TMP_13695']).totalStaked
TMP_13734(uint256) = REF_3978 (c)- stakeLost_1
$_4 (-> ['TMP_13695'])(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13695'])"])
REF_3977(uint256) (->$_4 (-> ['TMP_13695'])) := TMP_13734(uint256)
TMP_13695(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13695'])"])
 $.totalStaked = 0
REF_3979(uint256) -> $_3 (-> ['TMP_13695']).totalStaked
$_5 (-> ['TMP_13695'])(PlumeStakingStorage.Layout) := phi(["$_3 (-> ['TMP_13695'])"])
REF_3979(uint256) (->$_5 (-> ['TMP_13695'])) := 0(uint256)
TMP_13695(PlumeStakingStorage.Layout) := phi(["$_5 (-> ['TMP_13695'])"])
$_6 (-> ['TMP_13695'])(PlumeStakingStorage.Layout) := phi(["$_4 (-> ['TMP_13695'])", "$_5 (-> ['TMP_13695'])"])
 $.totalCooling >= cooledLost
REF_3980(uint256) -> $_6 (-> ['TMP_13695']).totalCooling
TMP_13735(bool) = REF_3980 >= cooledLost_1
CONDITION TMP_13735
 $.totalCooling = $.totalCooling - cooledLost
REF_3981(uint256) -> $_6 (-> ['TMP_13695']).totalCooling
REF_3982(uint256) -> $_6 (-> ['TMP_13695']).totalCooling
TMP_13736(uint256) = REF_3982 (c)- cooledLost_1
$_7 (-> ['TMP_13695'])(PlumeStakingStorage.Layout) := phi(["$_6 (-> ['TMP_13695'])"])
REF_3981(uint256) (->$_7 (-> ['TMP_13695'])) := TMP_13736(uint256)
TMP_13695(PlumeStakingStorage.Layout) := phi(["$_7 (-> ['TMP_13695'])"])
 $.totalCooling = 0
REF_3983(uint256) -> $_6 (-> ['TMP_13695']).totalCooling
$_8 (-> ['TMP_13695'])(PlumeStakingStorage.Layout) := phi(["$_6 (-> ['TMP_13695'])"])
REF_3983(uint256) (->$_8 (-> ['TMP_13695'])) := 0(uint256)
TMP_13695(PlumeStakingStorage.Layout) := phi(["$_8 (-> ['TMP_13695'])"])
$_9 (-> ['TMP_13695'])(PlumeStakingStorage.Layout) := phi(["$_7 (-> ['TMP_13695'])", "$_8 (-> ['TMP_13695'])"])
```
#### ValidatorFacet.forceSettleValidatorCommission(uint16) [EXTERNAL]
```slithir
 $s = PlumeStakingStorage.layout()
TMP_13737(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$s_1 (-> ['TMP_13737'])(PlumeStakingStorage.Layout) := TMP_13737(PlumeStakingStorage.Layout)
 ! $s.validatorExists[validatorId]
REF_3985(mapping(uint16 => bool)) -> $s_1 (-> ['TMP_13737']).validatorExists
REF_3986(bool) -> REF_3985[validatorId_1]
TMP_13738 = UnaryType.BANG REF_3986 
CONDITION TMP_13738
 revert ValidatorDoesNotExist(uint16)(validatorId)
TMP_13739(None) = SOLIDITY_CALL revert ValidatorDoesNotExist(uint16)(validatorId_1)
 PlumeRewardLogic._settleCommissionForValidatorUpToNow($s,validatorId)
LIBRARY_CALL, dest:PlumeRewardLogic, function:PlumeRewardLogic._settleCommissionForValidatorUpToNow(PlumeStakingStorage.Layout,uint16), arguments:["$s_1 (-> ['TMP_13737'])", 'validatorId_1']
```
#### ValidatorFacet.cleanupExpiredVotes(uint16) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13741(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13741'])(PlumeStakingStorage.Layout) := TMP_13741(PlumeStakingStorage.Layout)
 ! $.validatorExists[validatorId]
REF_3989(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13741']).validatorExists
REF_3990(bool) -> REF_3989[validatorId_1]
TMP_13742 = UnaryType.BANG REF_3990 
CONDITION TMP_13742
 revert ValidatorDoesNotExist(uint16)(validatorId)
TMP_13743(None) = SOLIDITY_CALL revert ValidatorDoesNotExist(uint16)(validatorId_1)
 _cleanupExpiredVotes(validatorId)
TMP_13744(uint256) = INTERNAL_CALL, ValidatorFacet._cleanupExpiredVotes(uint16)(validatorId_1)
RETURN TMP_13744
 validVoteCount
```
#### ValidatorFacet.getValidatorInfo(uint16) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13745(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13745'])(PlumeStakingStorage.Layout) := TMP_13745(PlumeStakingStorage.Layout)
 ! $.validatorExists[validatorId]
REF_3992(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13745']).validatorExists
REF_3993(bool) -> REF_3992[validatorId_1]
TMP_13746 = UnaryType.BANG REF_3993 
CONDITION TMP_13746
 revert ValidatorDoesNotExist(uint16)(validatorId)
TMP_13747(None) = SOLIDITY_CALL revert ValidatorDoesNotExist(uint16)(validatorId_1)
 info = $.validators[validatorId]
REF_3994(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13745']).validators
REF_3995(PlumeStakingStorage.ValidatorInfo) -> REF_3994[validatorId_1]
info_1(PlumeStakingStorage.ValidatorInfo) := REF_3995(PlumeStakingStorage.ValidatorInfo)
 totalStaked = $.validatorTotalStaked[validatorId]
REF_3996(mapping(uint16 => uint256)) -> $_1 (-> ['TMP_13745']).validatorTotalStaked
REF_3997(uint256) -> REF_3996[validatorId_1]
totalStaked_1(uint256) := REF_3997(uint256)
 stakersCount = $.validatorStakers[validatorId].length
REF_3998(mapping(uint16 => address[])) -> $_1 (-> ['TMP_13745']).validatorStakers
REF_3999(address[]) -> REF_3998[validatorId_1]
REF_4000 -> LENGTH REF_3999
stakersCount_1(uint256) := REF_4000(uint256)
 (info,totalStaked,stakersCount)
RETURN info_1,totalStaked_1,stakersCount_1
 (info,totalStaked,stakersCount)
```
#### ValidatorFacet.getValidatorStats(uint16) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13748(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13748'])(PlumeStakingStorage.Layout) := TMP_13748(PlumeStakingStorage.Layout)
 ! $.validatorExists[validatorId]
REF_4002(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13748']).validatorExists
REF_4003(bool) -> REF_4002[validatorId_1]
TMP_13749 = UnaryType.BANG REF_4003 
CONDITION TMP_13749
 revert ValidatorDoesNotExist(uint16)(validatorId)
TMP_13750(None) = SOLIDITY_CALL revert ValidatorDoesNotExist(uint16)(validatorId_1)
 validator = $.validators[validatorId]
REF_4004(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13748']).validators
REF_4005(PlumeStakingStorage.ValidatorInfo) -> REF_4004[validatorId_1]
validator_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_4005(PlumeStakingStorage.ValidatorInfo)
 active = validator.active
REF_4006(bool) -> validator_1 (-> ['$']).active
active_1(bool) := REF_4006(bool)
 commission = validator.commission
REF_4007(uint256) -> validator_1 (-> ['$']).commission
commission_1(uint256) := REF_4007(uint256)
 totalStaked = $.validatorTotalStaked[validatorId]
REF_4008(mapping(uint16 => uint256)) -> $_1 (-> ['TMP_13748']).validatorTotalStaked
REF_4009(uint256) -> REF_4008[validatorId_1]
totalStaked_1(uint256) := REF_4009(uint256)
 stakersCount = $.validatorStakers[validatorId].length
REF_4010(mapping(uint16 => address[])) -> $_1 (-> ['TMP_13748']).validatorStakers
REF_4011(address[]) -> REF_4010[validatorId_1]
REF_4012 -> LENGTH REF_4011
stakersCount_1(uint256) := REF_4012(uint256)
 (active,commission,totalStaked,stakersCount)
RETURN active_1,commission_1,totalStaked_1,stakersCount_1
 (active,commission,totalStaked,stakersCount)
```
#### ValidatorFacet.getUserValidators(address) [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13751(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13751'])(PlumeStakingStorage.Layout) := TMP_13751(PlumeStakingStorage.Layout)
 userAssociatedValidators = $.userValidators[user]
REF_4014(mapping(address => uint16[])) -> $_1 (-> ['TMP_13751']).userValidators
REF_4015(uint16[]) -> REF_4014[user_1]
userAssociatedValidators_1 (-> [])(uint16[]) = ['REF_4015(uint16[])']
 associatedCount = userAssociatedValidators.length
REF_4016 -> LENGTH userAssociatedValidators_1 (-> [])
associatedCount_1(uint256) := REF_4016(uint256)
 associatedCount == 0
TMP_13752(bool) = associatedCount_1 == 0
CONDITION TMP_13752
 new uint16[](0)
TMP_13754(uint16[])  = new uint16[](0)
RETURN TMP_13754
 tempNonSlashedValidators = new uint16[](associatedCount)
TMP_13756(uint16[])  = new uint16[](associatedCount_1)
tempNonSlashedValidators_1(uint16[]) = ['TMP_13756(uint16[])']
 actualCount = 0
actualCount_1(uint256) := 0(uint256)
 i = 0
i_1(uint256) := 0(uint256)
 i < associatedCount
i_2(uint256) := phi(['i_1', 'i_3'])
TMP_13757(bool) = i_2 < associatedCount_1
CONDITION TMP_13757
 valId = userAssociatedValidators[i]
REF_4017(uint16) -> userAssociatedValidators_1 (-> [])[i_2]
valId_1(uint16) := REF_4017(uint16)
 $.validatorExists[valId] && ! $.validators[valId].slashed
REF_4018(mapping(uint16 => bool)) -> $_1 (-> ['TMP_13751']).validatorExists
REF_4019(bool) -> REF_4018[valId_1]
REF_4020(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13751']).validators
REF_4021(PlumeStakingStorage.ValidatorInfo) -> REF_4020[valId_1]
REF_4022(bool) -> REF_4021.slashed
TMP_13758 = UnaryType.BANG REF_4022 
TMP_13759(bool) = REF_4019 && TMP_13758
CONDITION TMP_13759
 tempNonSlashedValidators[actualCount] = valId
REF_4023(uint16) -> tempNonSlashedValidators_1[actualCount_1]
tempNonSlashedValidators_2(uint16[]) := phi(['tempNonSlashedValidators_1'])
REF_4023(uint16) (->tempNonSlashedValidators_2) := valId_1(uint16)
 actualCount ++
TMP_13760(uint256) := actualCount_1(uint256)
actualCount_2(uint256) = actualCount_1 (c)+ 1
actualCount_3(uint256) := phi(['actualCount_2', 'actualCount_1'])
 i ++
TMP_13761(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 finalNonSlashedValidators = new uint16[](actualCount)
TMP_13763(uint16[])  = new uint16[](actualCount_1)
finalNonSlashedValidators_1(uint16[]) = ['TMP_13763(uint16[])']
 i_scope_0 = 0
i_scope_0_1(uint256) := 0(uint256)
 i_scope_0 < actualCount
i_scope_0_2(uint256) := phi(['i_scope_0_1', 'i_scope_0_3'])
TMP_13764(bool) = i_scope_0_2 < actualCount_1
CONDITION TMP_13764
 finalNonSlashedValidators[i_scope_0] = tempNonSlashedValidators[i_scope_0]
REF_4024(uint16) -> finalNonSlashedValidators_1[i_scope_0_2]
REF_4025(uint16) -> tempNonSlashedValidators_1[i_scope_0_2]
finalNonSlashedValidators_2(uint16[]) := phi(['finalNonSlashedValidators_1'])
REF_4024(uint16) (->finalNonSlashedValidators_2) := REF_4025(uint16)
 i_scope_0 ++
TMP_13765(uint256) := i_scope_0_2(uint256)
i_scope_0_3(uint256) = i_scope_0_2 (c)+ 1
 finalNonSlashedValidators
RETURN finalNonSlashedValidators_1
```
#### ValidatorFacet.getAccruedCommission(uint16,address) [PUBLIC]
```slithir
 $s = PlumeStakingStorage.layout()
TMP_13766(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$s_1 (-> ['TMP_13766'])(PlumeStakingStorage.Layout) := TMP_13766(PlumeStakingStorage.Layout)
 ! $s.validatorExists[validatorId]
REF_4027(mapping(uint16 => bool)) -> $s_1 (-> ['TMP_13766']).validatorExists
REF_4028(bool) -> REF_4027[validatorId_1]
TMP_13767 = UnaryType.BANG REF_4028 
CONDITION TMP_13767
 revert ValidatorDoesNotExist(uint16)(validatorId)
TMP_13768(None) = SOLIDITY_CALL revert ValidatorDoesNotExist(uint16)(validatorId_1)
 ! $s.isRewardToken[token]
REF_4029(mapping(address => bool)) -> $s_1 (-> ['TMP_13766']).isRewardToken
REF_4030(bool) -> REF_4029[token_1]
TMP_13769 = UnaryType.BANG REF_4030 
CONDITION TMP_13769
 revert TokenDoesNotExist(address)(token)
TMP_13770(None) = SOLIDITY_CALL revert TokenDoesNotExist(address)(token_1)
 $s.validatorAccruedCommission[validatorId][token]
REF_4031(mapping(uint16 => mapping(address => uint256))) -> $s_1 (-> ['TMP_13766']).validatorAccruedCommission
REF_4032(mapping(address => uint256)) -> REF_4031[validatorId_1]
REF_4033(uint256) -> REF_4032[token_1]
RETURN REF_4033
```
#### ValidatorFacet.getValidatorsList() [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13771(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13771'])(PlumeStakingStorage.Layout) := TMP_13771(PlumeStakingStorage.Layout)
 ids = $.validatorIds
REF_4035(uint16[]) -> $_1 (-> ['TMP_13771']).validatorIds
ids_1(uint16[]) = ['REF_4035(uint16[])']
 numValidators = ids.length
REF_4036 -> LENGTH ids_1
numValidators_1(uint256) := REF_4036(uint256)
 list = new ValidatorFacet.ValidatorListData[](numValidators)
TMP_13773(ValidatorFacet.ValidatorListData[])  = new ValidatorFacet.ValidatorListData[](numValidators_1)
list_1(ValidatorFacet.ValidatorListData[]) = ['TMP_13773(ValidatorFacet.ValidatorListData[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < numValidators
list_2(ValidatorFacet.ValidatorListData[]) := phi(['list_3', 'list_1'])
i_2(uint256) := phi(['i_1', 'i_3'])
TMP_13774(bool) = i_2 < numValidators_1
CONDITION TMP_13774
 id = ids[i]
REF_4037(uint16) -> ids_1[i_2]
id_1(uint16) := REF_4037(uint16)
 info = $.validators[id]
REF_4038(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13771']).validators
REF_4039(PlumeStakingStorage.ValidatorInfo) -> REF_4038[id_1]
info_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_4039(PlumeStakingStorage.ValidatorInfo)
 list[i] = ValidatorFacet.ValidatorListData({id:id,totalStaked:$.validatorTotalStaked[id],commission:info.commission})
REF_4040(ValidatorFacet.ValidatorListData) -> list_2[i_2]
REF_4042(mapping(uint16 => uint256)) -> $_1 (-> ['TMP_13771']).validatorTotalStaked
REF_4043(uint256) -> REF_4042[id_1]
REF_4044(uint256) -> info_1 (-> ['$']).commission
TMP_13775(ValidatorFacet.ValidatorListData) = new ValidatorListData(id_1,REF_4043,REF_4044)
list_3(ValidatorFacet.ValidatorListData[]) := phi(['list_2'])
REF_4040(ValidatorFacet.ValidatorListData) (->list_3) := TMP_13775(ValidatorFacet.ValidatorListData)
 i ++
TMP_13776(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 list
RETURN list_2
```
#### ValidatorFacet.getActiveValidatorCount() [EXTERNAL]
```slithir
 $ = PlumeStakingStorage.layout()
TMP_13777(PlumeStakingStorage.Layout) = LIBRARY_CALL, dest:PlumeStakingStorage, function:PlumeStakingStorage.layout(), arguments:[] 
$_1 (-> ['TMP_13777'])(PlumeStakingStorage.Layout) := TMP_13777(PlumeStakingStorage.Layout)
 ids = $.validatorIds
REF_4046(uint16[]) -> $_1 (-> ['TMP_13777']).validatorIds
ids_1(uint16[]) = ['REF_4046(uint16[])']
 count = 0
count_1(uint256) := 0(uint256)
 i = 0
i_1(uint256) := 0(uint256)
 i < ids.length
i_2(uint256) := phi(['i_3', 'i_1'])
REF_4047 -> LENGTH ids_1
TMP_13778(bool) = i_2 < REF_4047
CONDITION TMP_13778
 $.validators[ids[i]].active
REF_4048(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> ['TMP_13777']).validators
REF_4049(uint16) -> ids_1[i_2]
REF_4050(PlumeStakingStorage.ValidatorInfo) -> REF_4048[REF_4049]
REF_4051(bool) -> REF_4050.active
CONDITION REF_4051
 count ++
TMP_13779(uint256) := count_1(uint256)
count_2(uint256) = count_1 (c)+ 1
count_3(uint256) := phi(['count_1', 'count_2'])
 i ++
TMP_13780(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 count
RETURN count_1
 count
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
REF_4290(address[]) -> $_1 (-> []).rewardTokens
rewardTokens_1(address[]) = ['REF_4290(address[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < rewardTokens.length
i_2(uint256) := phi(['i_3', 'i_1'])
REF_4291 -> LENGTH rewardTokens_1
TMP_14012(bool) = i_2 < REF_4291
CONDITION TMP_14012
 token = rewardTokens[i]
REF_4292(address) -> rewardTokens_1[i_2]
token_1(address) := REF_4292(address)
 updateRewardPerTokenForValidator($,token,validatorId)
INTERNAL_CALL, PlumeRewardLogic.updateRewardPerTokenForValidator(PlumeStakingStorage.Layout,address,uint16)($_1 (-> []),token_1,validatorId_1)
 i ++
TMP_14014(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
```
#### PlumeRewardLogic.createCommissionRateCheckpoint(PlumeStakingStorage.Layout,uint16,uint256) [INTERNAL]
```slithir
 checkpoint = PlumeStakingStorage.RateCheckpoint({timestamp:block.timestamp,rate:commissionRate,cumulativeIndex:0})
TMP_14007(PlumeStakingStorage.RateCheckpoint) = new RateCheckpoint(block.timestamp,commissionRate_1,0)
checkpoint_1(PlumeStakingStorage.RateCheckpoint) := TMP_14007(PlumeStakingStorage.RateCheckpoint)
 $.validatorCommissionCheckpoints[validatorId].push(checkpoint)
REF_4285(mapping(uint16 => PlumeStakingStorage.RateCheckpoint[])) -> $_1 (-> []).validatorCommissionCheckpoints
REF_4286(PlumeStakingStorage.RateCheckpoint[]) -> REF_4285[validatorId_1]
REF_4288 -> LENGTH REF_4286
TMP_14009(uint256) := REF_4288(uint256)
TMP_14010(uint256) = TMP_14009 (c)+ 1
$_2 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])'])
REF_4288(uint256) (->$_3 (-> [])) := TMP_14010(uint256)
REF_4289(PlumeStakingStorage.RateCheckpoint) -> REF_4286[TMP_14009]
$_3 (-> [])(PlumeStakingStorage.Layout) := phi(['$_2 (-> [])'])
REF_4289(PlumeStakingStorage.RateCheckpoint) (->$_3 (-> [])) := checkpoint_1(PlumeStakingStorage.RateCheckpoint)
 ValidatorCommissionCheckpointCreated(validatorId,commissionRate,block.timestamp)
Emit ValidatorCommissionCheckpointCreated(validatorId_1,commissionRate_1,block.timestamp)
```
#### RewardsFacet.getTreasury() [EXTERNAL]
```slithir
 getTreasuryAddress()
TMP_13189(address) = INTERNAL_CALL, RewardsFacet.getTreasuryAddress()()
RETURN TMP_13189
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
REF_4143(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> []).validators
REF_4144(PlumeStakingStorage.ValidatorInfo) -> REF_4143[validatorId_1]
validator_1 (-> ['$'])(PlumeStakingStorage.ValidatorInfo) := REF_4144(PlumeStakingStorage.ValidatorInfo)
 validator.slashed
REF_4145(bool) -> validator_1 (-> ['$']).slashed
CONDITION REF_4145
 slashTs = validator.slashedAtTimestamp
REF_4146(uint256) -> validator_1 (-> ['$']).slashedAtTimestamp
slashTs_1(uint256) := REF_4146(uint256)
 currentLastUpdateTime = $.validatorLastUpdateTimes[validatorId][token]
REF_4147(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> []).validatorLastUpdateTimes
REF_4148(mapping(address => uint256)) -> REF_4147[validatorId_1]
REF_4149(uint256) -> REF_4148[token_1]
currentLastUpdateTime_1(uint256) := REF_4149(uint256)
 currentLastUpdateTime < effectiveTimestampForUpdate
TMP_13858(bool) = currentLastUpdateTime_1 < effectiveTimestampForUpdate_3
CONDITION TMP_13858
 totalStakedForCalc = $.validatorTotalStaked[validatorId]
REF_4150(mapping(uint16 => uint256)) -> $_1 (-> []).validatorTotalStaked
REF_4151(uint256) -> REF_4150[validatorId_1]
totalStakedForCalc_1(uint256) := REF_4151(uint256)
 totalStakedForCalc > 0
TMP_13859(bool) = totalStakedForCalc_1 > 0
CONDITION TMP_13859
 timeDelta = effectiveTimestampForUpdate - currentLastUpdateTime
TMP_13860(uint256) = effectiveTimestampForUpdate_3 (c)- currentLastUpdateTime_1
timeDelta_1(uint256) := TMP_13860(uint256)
 effectiveRewardRateChk = getEffectiveRewardRateAt($,token,validatorId,effectiveTimestampForUpdate)
TMP_13861(PlumeStakingStorage.RateCheckpoint) = INTERNAL_CALL, PlumeRewardLogic.getEffectiveRewardRateAt(PlumeStakingStorage.Layout,address,uint16,uint256)($_1 (-> []),token_1,validatorId_1,effectiveTimestampForUpdate_3)
effectiveRewardRateChk_1(PlumeStakingStorage.RateCheckpoint) := TMP_13861(PlumeStakingStorage.RateCheckpoint)
 effectiveRewardRate = effectiveRewardRateChk.rate
REF_4152(uint256) -> effectiveRewardRateChk_1.rate
effectiveRewardRate_1(uint256) := REF_4152(uint256)
 effectiveRewardRate > 0 && timeDelta > 0
TMP_13862(bool) = effectiveRewardRate_1 > 0
TMP_13863(bool) = timeDelta_1 > 0
TMP_13864(bool) = TMP_13862 && TMP_13863
CONDITION TMP_13864
 rewardPerTokenIncrease = timeDelta * effectiveRewardRate
TMP_13865(uint256) = timeDelta_1 (c)* effectiveRewardRate_1
rewardPerTokenIncrease_1(uint256) := TMP_13865(uint256)
 $.validatorRewardPerTokenCumulative[validatorId][token] += rewardPerTokenIncrease
REF_4153(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> []).validatorRewardPerTokenCumulative
REF_4154(mapping(address => uint256)) -> REF_4153[validatorId_1]
REF_4155(uint256) -> REF_4154[token_1]
$_3 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])'])
REF_4155(-> $_3 (-> [])) = REF_4155 (c)+ rewardPerTokenIncrease_1
 commissionRateForSegment = getEffectiveCommissionRateAt($,validatorId,currentLastUpdateTime)
TMP_13866(uint256) = INTERNAL_CALL, PlumeRewardLogic.getEffectiveCommissionRateAt(PlumeStakingStorage.Layout,uint16,uint256)($_3 (-> []),validatorId_1,currentLastUpdateTime_1)
commissionRateForSegment_1(uint256) := TMP_13866(uint256)
 grossRewardForValidatorThisSegment = (totalStakedForCalc * rewardPerTokenIncrease) / PlumeStakingStorage.REWARD_PRECISION
TMP_13867(uint256) = totalStakedForCalc_1 (c)* rewardPerTokenIncrease_1
REF_4156(uint256) -> PlumeStakingStorage.REWARD_PRECISION
TMP_13868(uint256) = TMP_13867 (c)/ REF_4156
grossRewardForValidatorThisSegment_1(uint256) := TMP_13868(uint256)
 commissionDeltaForValidator = (grossRewardForValidatorThisSegment * commissionRateForSegment) / PlumeStakingStorage.REWARD_PRECISION
TMP_13869(uint256) = grossRewardForValidatorThisSegment_1 (c)* commissionRateForSegment_1
REF_4157(uint256) -> PlumeStakingStorage.REWARD_PRECISION
TMP_13870(uint256) = TMP_13869 (c)/ REF_4157
commissionDeltaForValidator_1(uint256) := TMP_13870(uint256)
 commissionDeltaForValidator > 0
TMP_13871(bool) = commissionDeltaForValidator_1 > 0
CONDITION TMP_13871
 $.validatorAccruedCommission[validatorId][token] += commissionDeltaForValidator
REF_4158(mapping(uint16 => mapping(address => uint256))) -> $_3 (-> []).validatorAccruedCommission
REF_4159(mapping(address => uint256)) -> REF_4158[validatorId_1]
REF_4160(uint256) -> REF_4159[token_1]
$_4 (-> [])(PlumeStakingStorage.Layout) := phi(['$_3 (-> [])'])
REF_4160(-> $_4 (-> [])) = REF_4160 (c)+ commissionDeltaForValidator_1
$_5 (-> [])(PlumeStakingStorage.Layout) := phi(['$_4 (-> [])', '$_3 (-> [])'])
$_6 (-> [])(PlumeStakingStorage.Layout) := phi(['$_3 (-> [])', '$_1 (-> [])'])
 $.validatorLastUpdateTimes[validatorId][token] = effectiveTimestampForUpdate
REF_4161(mapping(uint16 => mapping(address => uint256))) -> $_6 (-> []).validatorLastUpdateTimes
REF_4162(mapping(address => uint256)) -> REF_4161[validatorId_1]
REF_4163(uint256) -> REF_4162[token_1]
$_7 (-> [])(PlumeStakingStorage.Layout) := phi(['$_6 (-> [])'])
REF_4163(uint256) (->$_7 (-> [])) := effectiveTimestampForUpdate_3(uint256)
 block.timestamp > $.validatorLastUpdateTimes[validatorId][token]
REF_4164(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> []).validatorLastUpdateTimes
REF_4165(mapping(address => uint256)) -> REF_4164[validatorId_1]
REF_4166(uint256) -> REF_4165[token_1]
TMP_13872(bool) = block.timestamp > REF_4166
CONDITION TMP_13872
 $.validatorLastUpdateTimes[validatorId][token] = block.timestamp
REF_4167(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> []).validatorLastUpdateTimes
REF_4168(mapping(address => uint256)) -> REF_4167[validatorId_1]
REF_4169(uint256) -> REF_4168[token_1]
$_2 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])'])
REF_4169(uint256) (->$_2 (-> [])) := block.timestamp(uint256)
 totalStaked = $.validatorTotalStaked[validatorId]
REF_4170(mapping(uint16 => uint256)) -> $_1 (-> []).validatorTotalStaked
REF_4171(uint256) -> REF_4170[validatorId_1]
totalStaked_1(uint256) := REF_4171(uint256)
 oldLastUpdateTime = $.validatorLastUpdateTimes[validatorId][token]
REF_4172(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> []).validatorLastUpdateTimes
REF_4173(mapping(address => uint256)) -> REF_4172[validatorId_1]
REF_4174(uint256) -> REF_4173[token_1]
oldLastUpdateTime_1(uint256) := REF_4174(uint256)
 block.timestamp > oldLastUpdateTime
TMP_13873(bool) = block.timestamp > oldLastUpdateTime_1
CONDITION TMP_13873
 totalStaked > 0
TMP_13874(bool) = totalStaked_1 > 0
CONDITION TMP_13874
 timeDelta_scope_0 = block.timestamp - oldLastUpdateTime
TMP_13875(uint256) = block.timestamp (c)- oldLastUpdateTime_1
timeDelta_scope_0_1(uint256) := TMP_13875(uint256)
 effectiveRewardRateChk_scope_1 = getEffectiveRewardRateAt($,token,validatorId,block.timestamp)
TMP_13876(PlumeStakingStorage.RateCheckpoint) = INTERNAL_CALL, PlumeRewardLogic.getEffectiveRewardRateAt(PlumeStakingStorage.Layout,address,uint16,uint256)($_1 (-> []),token_1,validatorId_1,block.timestamp)
effectiveRewardRateChk_scope_1_1(PlumeStakingStorage.RateCheckpoint) := TMP_13876(PlumeStakingStorage.RateCheckpoint)
 effectiveRewardRate_scope_2 = effectiveRewardRateChk_scope_1.rate
REF_4175(uint256) -> effectiveRewardRateChk_scope_1_1.rate
effectiveRewardRate_scope_2_1(uint256) := REF_4175(uint256)
 effectiveRewardRate_scope_2 > 0
TMP_13877(bool) = effectiveRewardRate_scope_2_1 > 0
CONDITION TMP_13877
 rewardPerTokenIncrease_scope_3 = timeDelta_scope_0 * effectiveRewardRate_scope_2
TMP_13878(uint256) = timeDelta_scope_0_1 (c)* effectiveRewardRate_scope_2_1
rewardPerTokenIncrease_scope_3_1(uint256) := TMP_13878(uint256)
 $.validatorRewardPerTokenCumulative[validatorId][token] += rewardPerTokenIncrease_scope_3
REF_4176(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> []).validatorRewardPerTokenCumulative
REF_4177(mapping(address => uint256)) -> REF_4176[validatorId_1]
REF_4178(uint256) -> REF_4177[token_1]
$_8 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])'])
REF_4178(-> $_8 (-> [])) = REF_4178 (c)+ rewardPerTokenIncrease_scope_3_1
 commissionRateForSegment_scope_4 = getEffectiveCommissionRateAt($,validatorId,oldLastUpdateTime)
TMP_13879(uint256) = INTERNAL_CALL, PlumeRewardLogic.getEffectiveCommissionRateAt(PlumeStakingStorage.Layout,uint16,uint256)($_8 (-> []),validatorId_1,oldLastUpdateTime_1)
commissionRateForSegment_scope_4_1(uint256) := TMP_13879(uint256)
 grossRewardForValidatorThisSegment_scope_5 = (totalStaked * rewardPerTokenIncrease_scope_3) / PlumeStakingStorage.REWARD_PRECISION
TMP_13880(uint256) = totalStaked_1 (c)* rewardPerTokenIncrease_scope_3_1
REF_4179(uint256) -> PlumeStakingStorage.REWARD_PRECISION
TMP_13881(uint256) = TMP_13880 (c)/ REF_4179
grossRewardForValidatorThisSegment_scope_5_1(uint256) := TMP_13881(uint256)
 commissionDeltaForValidator_scope_6 = (grossRewardForValidatorThisSegment_scope_5 * commissionRateForSegment_scope_4) / PlumeStakingStorage.REWARD_PRECISION
TMP_13882(uint256) = grossRewardForValidatorThisSegment_scope_5_1 (c)* commissionRateForSegment_scope_4_1
REF_4180(uint256) -> PlumeStakingStorage.REWARD_PRECISION
TMP_13883(uint256) = TMP_13882 (c)/ REF_4180
commissionDeltaForValidator_scope_6_1(uint256) := TMP_13883(uint256)
 commissionDeltaForValidator_scope_6 > 0
TMP_13884(bool) = commissionDeltaForValidator_scope_6_1 > 0
CONDITION TMP_13884
 $.validatorAccruedCommission[validatorId][token] += commissionDeltaForValidator_scope_6
REF_4181(mapping(uint16 => mapping(address => uint256))) -> $_8 (-> []).validatorAccruedCommission
REF_4182(mapping(address => uint256)) -> REF_4181[validatorId_1]
REF_4183(uint256) -> REF_4182[token_1]
$_9 (-> [])(PlumeStakingStorage.Layout) := phi(['$_8 (-> [])'])
REF_4183(-> $_9 (-> [])) = REF_4183 (c)+ commissionDeltaForValidator_scope_6_1
$_10 (-> [])(PlumeStakingStorage.Layout) := phi(['$_8 (-> [])', '$_9 (-> [])'])
$_11 (-> [])(PlumeStakingStorage.Layout) := phi(['$_8 (-> [])', '$_1 (-> [])'])
 $.validatorLastUpdateTimes[validatorId][token] = block.timestamp
REF_4184(mapping(uint16 => mapping(address => uint256))) -> $_11 (-> []).validatorLastUpdateTimes
REF_4185(mapping(address => uint256)) -> REF_4184[validatorId_1]
REF_4186(uint256) -> REF_4185[token_1]
$_12 (-> [])(PlumeStakingStorage.Layout) := phi(['$_11 (-> [])'])
REF_4186(uint256) (->$_12 (-> [])) := block.timestamp(uint256)
 block.timestamp < slashTs
TMP_13885(bool) = block.timestamp < slashTs_1
CONDITION TMP_13885
 effectiveTimestampForUpdate = block.timestamp
effectiveTimestampForUpdate_1(uint256) := block.timestamp(uint256)
 effectiveTimestampForUpdate = slashTs
effectiveTimestampForUpdate_2(uint256) := slashTs_1(uint256)
effectiveTimestampForUpdate_3(uint256) := phi(['effectiveTimestampForUpdate_1', 'effectiveTimestampForUpdate_2'])
```
#### PlumeRewardLogic.calculateRewardsWithCheckpoints(PlumeStakingStorage.Layout,address,uint16,address,uint256) [INTERNAL]
```slithir
$_1 (-> [])(PlumeStakingStorage.Layout) := phi(['$_4 (-> [])'])
user_1(address) := phi(['user_1'])
validatorId_1(uint16) := phi(['validatorId_1'])
token_1(address) := phi(['token_1'])
userStakedAmount_1(uint256) := phi(['userStakedAmount_1'])
 updateRewardPerTokenForValidator($,token,validatorId)
INTERNAL_CALL, PlumeRewardLogic.updateRewardPerTokenForValidator(PlumeStakingStorage.Layout,address,uint16)($_1 (-> []),token_1,validatorId_1)
 lastUserPaidCumulativeRewardPerToken = $.userValidatorRewardPerTokenPaid[user][validatorId][token]
REF_4187(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_1 (-> []).userValidatorRewardPerTokenPaid
REF_4188(mapping(uint16 => mapping(address => uint256))) -> REF_4187[user_1]
REF_4189(mapping(address => uint256)) -> REF_4188[validatorId_1]
REF_4190(uint256) -> REF_4189[token_1]
lastUserPaidCumulativeRewardPerToken_1(uint256) := REF_4190(uint256)
 finalCumulativeRewardPerToken = $.validatorRewardPerTokenCumulative[validatorId][token]
REF_4191(mapping(uint16 => mapping(address => uint256))) -> $_1 (-> []).validatorRewardPerTokenCumulative
REF_4192(mapping(address => uint256)) -> REF_4191[validatorId_1]
REF_4193(uint256) -> REF_4192[token_1]
finalCumulativeRewardPerToken_1(uint256) := REF_4193(uint256)
 lastUserRewardUpdateTime = $.userValidatorRewardPerTokenPaidTimestamp[user][validatorId][token]
REF_4194(mapping(address => mapping(uint16 => mapping(address => uint256)))) -> $_1 (-> []).userValidatorRewardPerTokenPaidTimestamp
REF_4195(mapping(uint16 => mapping(address => uint256))) -> REF_4194[user_1]
REF_4196(mapping(address => uint256)) -> REF_4195[validatorId_1]
REF_4197(uint256) -> REF_4196[token_1]
lastUserRewardUpdateTime_1(uint256) := REF_4197(uint256)
 lastUserRewardUpdateTime == 0
TMP_13887(bool) = lastUserRewardUpdateTime_1 == 0
CONDITION TMP_13887
 lastUserRewardUpdateTime = $.userValidatorStakeStartTime[user][validatorId]
REF_4198(mapping(address => mapping(uint16 => uint256))) -> $_1 (-> []).userValidatorStakeStartTime
REF_4199(mapping(uint16 => uint256)) -> REF_4198[user_1]
REF_4200(uint256) -> REF_4199[validatorId_1]
lastUserRewardUpdateTime_2(uint256) := REF_4200(uint256)
 lastUserRewardUpdateTime == 0 && $.userValidatorStakes[user][validatorId].staked > 0
TMP_13888(bool) = lastUserRewardUpdateTime_2 == 0
REF_4201(mapping(address => mapping(uint16 => PlumeStakingStorage.UserValidatorStake))) -> $_1 (-> []).userValidatorStakes
REF_4202(mapping(uint16 => PlumeStakingStorage.UserValidatorStake)) -> REF_4201[user_1]
REF_4203(PlumeStakingStorage.UserValidatorStake) -> REF_4202[validatorId_1]
REF_4204(uint256) -> REF_4203.staked
TMP_13889(bool) = REF_4204 > 0
TMP_13890(bool) = TMP_13888 && TMP_13889
CONDITION TMP_13890
 lastUserRewardUpdateTime = block.timestamp
lastUserRewardUpdateTime_3(uint256) := block.timestamp(uint256)
lastUserRewardUpdateTime_4(uint256) := phi(['lastUserRewardUpdateTime_2', 'lastUserRewardUpdateTime_3'])
lastUserRewardUpdateTime_5(uint256) := phi(['lastUserRewardUpdateTime_2', 'lastUserRewardUpdateTime_1'])
 block.timestamp <= lastUserRewardUpdateTime || finalCumulativeRewardPerToken <= lastUserPaidCumulativeRewardPerToken
TMP_13891(bool) = block.timestamp <= lastUserRewardUpdateTime_5
TMP_13892(bool) = finalCumulativeRewardPerToken_1 <= lastUserPaidCumulativeRewardPerToken_1
TMP_13893(bool) = TMP_13891 || TMP_13892
CONDITION TMP_13893
 (0,0,0)
RETURN 0,0,0
 effectiveTimeDelta = block.timestamp - lastUserRewardUpdateTime
TMP_13894(uint256) = block.timestamp (c)- lastUserRewardUpdateTime_5
effectiveTimeDelta_1(uint256) := TMP_13894(uint256)
 distinctTimestamps = getDistinctTimestamps($,validatorId,token,lastUserRewardUpdateTime,block.timestamp)
TMP_13895(uint256[]) = INTERNAL_CALL, PlumeRewardLogic.getDistinctTimestamps(PlumeStakingStorage.Layout,uint16,address,uint256,uint256)($_1 (-> []),validatorId_1,token_1,lastUserRewardUpdateTime_5,block.timestamp)
distinctTimestamps_1(uint256[]) = ['TMP_13895(uint256[])']
 distinctTimestamps.length < 2
REF_4205 -> LENGTH distinctTimestamps_1
TMP_13896(bool) = REF_4205 < 2
CONDITION TMP_13896
 (0,0,0)
RETURN 0,0,0
 rptTracker = lastUserPaidCumulativeRewardPerToken
rptTracker_1(uint256) := lastUserPaidCumulativeRewardPerToken_1(uint256)
 k = 0
k_1(uint256) := 0(uint256)
 k < distinctTimestamps.length - 1
k_2(uint256) := phi(['k_1', 'k_3'])
REF_4206 -> LENGTH distinctTimestamps_1
TMP_13897(uint256) = REF_4206 (c)- 1
TMP_13898(bool) = k_2 < TMP_13897
CONDITION TMP_13898
 segmentStartTime = distinctTimestamps[k]
REF_4207(uint256) -> distinctTimestamps_1[k_2]
segmentStartTime_1(uint256) := REF_4207(uint256)
 segmentEndTime = distinctTimestamps[k + 1]
TMP_13899(uint256) = k_2 (c)+ 1
REF_4208(uint256) -> distinctTimestamps_1[TMP_13899]
segmentEndTime_1(uint256) := REF_4208(uint256)
 segmentEndTime <= segmentStartTime
TMP_13900(bool) = segmentEndTime_1 <= segmentStartTime_1
CONDITION TMP_13900
 k == 0
TMP_13901(bool) = k_2 == 0
CONDITION TMP_13901
 rptAtSegmentStart = lastUserPaidCumulativeRewardPerToken
rptAtSegmentStart_1(uint256) := lastUserPaidCumulativeRewardPerToken_1(uint256)
 rptAtSegmentStart = rptTracker
rptAtSegmentStart_2(uint256) := rptTracker_1(uint256)
rptAtSegmentStart_3(uint256) := phi(['rptAtSegmentStart_1', 'rptAtSegmentStart_2'])
 rewardRateInfoForSegment = getEffectiveRewardRateAt($,token,validatorId,segmentStartTime)
TMP_13902(PlumeStakingStorage.RateCheckpoint) = INTERNAL_CALL, PlumeRewardLogic.getEffectiveRewardRateAt(PlumeStakingStorage.Layout,address,uint16,uint256)($_1 (-> []),token_1,validatorId_1,segmentStartTime_1)
rewardRateInfoForSegment_1(PlumeStakingStorage.RateCheckpoint) := TMP_13902(PlumeStakingStorage.RateCheckpoint)
 effectiveRewardRate = rewardRateInfoForSegment.rate
REF_4209(uint256) -> rewardRateInfoForSegment_1.rate
effectiveRewardRate_1(uint256) := REF_4209(uint256)
 segmentDuration = segmentEndTime - segmentStartTime
TMP_13903(uint256) = segmentEndTime_1 (c)- segmentStartTime_1
segmentDuration_1(uint256) := TMP_13903(uint256)
 rptIncreaseInSegment = 0
rptIncreaseInSegment_1(uint256) := 0(uint256)
 effectiveRewardRate > 0 && segmentDuration > 0
TMP_13904(bool) = effectiveRewardRate_1 > 0
TMP_13905(bool) = segmentDuration_1 > 0
TMP_13906(bool) = TMP_13904 && TMP_13905
CONDITION TMP_13906
 rptIncreaseInSegment = segmentDuration * effectiveRewardRate
TMP_13907(uint256) = segmentDuration_1 (c)* effectiveRewardRate_1
rptIncreaseInSegment_2(uint256) := TMP_13907(uint256)
rptIncreaseInSegment_3(uint256) := phi(['rptIncreaseInSegment_2', 'rptIncreaseInSegment_1'])
 rptAtSegmentEnd = rptAtSegmentStart + rptIncreaseInSegment
TMP_13908(uint256) = rptAtSegmentStart_3 (c)+ rptIncreaseInSegment_3
rptAtSegmentEnd_1(uint256) := TMP_13908(uint256)
 rewardPerTokenDeltaForUserInSegment = rptAtSegmentEnd - rptAtSegmentStart
TMP_13909(uint256) = rptAtSegmentEnd_1 (c)- rptAtSegmentStart_3
rewardPerTokenDeltaForUserInSegment_1(uint256) := TMP_13909(uint256)
 rewardPerTokenDeltaForUserInSegment > 0 && userStakedAmount > 0
TMP_13910(bool) = rewardPerTokenDeltaForUserInSegment_1 > 0
TMP_13911(bool) = userStakedAmount_1 > 0
TMP_13912(bool) = TMP_13910 && TMP_13911
CONDITION TMP_13912
 grossRewardForSegment = (userStakedAmount * rewardPerTokenDeltaForUserInSegment) / PlumeStakingStorage.REWARD_PRECISION
TMP_13913(uint256) = userStakedAmount_1 (c)* rewardPerTokenDeltaForUserInSegment_1
REF_4210(uint256) -> PlumeStakingStorage.REWARD_PRECISION
TMP_13914(uint256) = TMP_13913 (c)/ REF_4210
grossRewardForSegment_1(uint256) := TMP_13914(uint256)
 effectiveCommissionRate = getEffectiveCommissionRateAt($,validatorId,segmentStartTime)
TMP_13915(uint256) = INTERNAL_CALL, PlumeRewardLogic.getEffectiveCommissionRateAt(PlumeStakingStorage.Layout,uint16,uint256)($_1 (-> []),validatorId_1,segmentStartTime_1)
effectiveCommissionRate_1(uint256) := TMP_13915(uint256)
 commissionForThisSegment = _ceilDiv(grossRewardForSegment * effectiveCommissionRate,PlumeStakingStorage.REWARD_PRECISION)
TMP_13916(uint256) = grossRewardForSegment_1 (c)* effectiveCommissionRate_1
REF_4211(uint256) -> PlumeStakingStorage.REWARD_PRECISION
TMP_13917(uint256) = INTERNAL_CALL, PlumeRewardLogic._ceilDiv(uint256,uint256)(TMP_13916,REF_4211)
commissionForThisSegment_1(uint256) := TMP_13917(uint256)
 grossRewardForSegment >= commissionForThisSegment
TMP_13918(bool) = grossRewardForSegment_1 >= commissionForThisSegment_1
CONDITION TMP_13918
 totalUserRewardDelta += (grossRewardForSegment - commissionForThisSegment)
TMP_13919(uint256) = grossRewardForSegment_1 (c)- commissionForThisSegment_1
totalUserRewardDelta_1(uint256) = totalUserRewardDelta_0 (c)+ TMP_13919
totalUserRewardDelta_2(uint256) := phi(['totalUserRewardDelta_1', 'totalUserRewardDelta_0'])
 totalCommissionAmountDelta += commissionForThisSegment
totalCommissionAmountDelta_1(uint256) = totalCommissionAmountDelta_0 (c)+ commissionForThisSegment_1
totalCommissionAmountDelta_2(uint256) := phi(['totalCommissionAmountDelta_1', 'totalCommissionAmountDelta_0'])
 rptTracker = rptAtSegmentEnd
rptTracker_3(uint256) := rptAtSegmentEnd_1(uint256)
 ++ k
rptTracker_2(uint256) := phi(['rptTracker_1', 'rptTracker_3'])
k_3(uint256) = k_2 (c)+ 1
 (totalUserRewardDelta,totalCommissionAmountDelta,effectiveTimeDelta)
RETURN totalUserRewardDelta_0,totalCommissionAmountDelta_0,effectiveTimeDelta_1
 (totalUserRewardDelta,totalCommissionAmountDelta,effectiveTimeDelta)
```
#### RewardsFacet.getTreasuryAddress() [INTERNAL]
```slithir
TREASURY_STORAGE_POSITION_1(bytes32) := phi(['TREASURY_STORAGE_POSITION_0'])
 position = TREASURY_STORAGE_POSITION
position_1(bytes32) := TREASURY_STORAGE_POSITION_1(bytes32)
 treasuryAddress = sload(uint256)(position)
TMP_13021(uint256) = SOLIDITY_CALL sload(uint256)(position_1)
treasuryAddress_1(address) := TMP_13021(uint256)
 treasuryAddress
RETURN treasuryAddress_1
```
#### PlumeRewardLogic.getEffectiveCommissionRateAt(PlumeStakingStorage.Layout,uint16,uint256) [INTERNAL]
```slithir
$_1 (-> [])(PlumeStakingStorage.Layout) := phi(['$_8 (-> [])', '$_3 (-> [])', '$_1 (-> [])'])
validatorId_1(uint16) := phi(['validatorId_1', 'validatorId_1'])
timestamp_1(uint256) := phi(['oldLastUpdateTime_1', 'segmentStartTime_1', 'currentLastUpdateTime_1'])
 checkpoints = $.validatorCommissionCheckpoints[validatorId]
REF_4245(mapping(uint16 => PlumeStakingStorage.RateCheckpoint[])) -> $_1 (-> []).validatorCommissionCheckpoints
REF_4246(PlumeStakingStorage.RateCheckpoint[]) -> REF_4245[validatorId_1]
checkpoints_1 (-> [])(PlumeStakingStorage.RateCheckpoint[]) = ['REF_4246(PlumeStakingStorage.RateCheckpoint[])']
 chkCount = checkpoints.length
REF_4247 -> LENGTH checkpoints_1 (-> [])
chkCount_1(uint256) := REF_4247(uint256)
 chkCount > 0
TMP_13975(bool) = chkCount_1 > 0
CONDITION TMP_13975
 idx = findCommissionCheckpointIndexAtOrBefore($,validatorId,timestamp)
TMP_13976(uint256) = INTERNAL_CALL, PlumeRewardLogic.findCommissionCheckpointIndexAtOrBefore(PlumeStakingStorage.Layout,uint16,uint256)($_1 (-> []),validatorId_1,timestamp_1)
idx_1(uint256) := TMP_13976(uint256)
 idx < chkCount && checkpoints[idx].timestamp <= timestamp
TMP_13977(bool) = idx_1 < chkCount_1
REF_4248(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[idx_1]
REF_4249(uint256) -> REF_4248.timestamp
TMP_13978(bool) = REF_4249 <= timestamp_1
TMP_13979(bool) = TMP_13977 && TMP_13978
CONDITION TMP_13979
 checkpoints[idx].rate
REF_4250(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[idx_1]
REF_4251(uint256) -> REF_4250.rate
RETURN REF_4251
 fallbackComm = $.validators[validatorId].commission
REF_4252(mapping(uint16 => PlumeStakingStorage.ValidatorInfo)) -> $_1 (-> []).validators
REF_4253(PlumeStakingStorage.ValidatorInfo) -> REF_4252[validatorId_1]
REF_4254(uint256) -> REF_4253.commission
fallbackComm_1(uint256) := REF_4254(uint256)
 fallbackComm
RETURN fallbackComm_1
```
#### PlumeRewardLogic.getEffectiveRewardRateAt(PlumeStakingStorage.Layout,address,uint16,uint256) [INTERNAL]
```slithir
$_1 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])', '$_1 (-> [])'])
token_1(address) := phi(['token_1', 'token_1'])
validatorId_1(uint16) := phi(['validatorId_1', 'validatorId_1'])
timestamp_1(uint256) := phi(['segmentStartTime_1', 'effectiveTimestampForUpdate_3', 'block.timestamp'])
 checkpoints = $.validatorRewardRateCheckpoints[validatorId][token]
REF_4231(mapping(uint16 => mapping(address => PlumeStakingStorage.RateCheckpoint[]))) -> $_1 (-> []).validatorRewardRateCheckpoints
REF_4232(mapping(address => PlumeStakingStorage.RateCheckpoint[])) -> REF_4231[validatorId_1]
REF_4233(PlumeStakingStorage.RateCheckpoint[]) -> REF_4232[token_1]
checkpoints_1 (-> [])(PlumeStakingStorage.RateCheckpoint[]) = ['REF_4233(PlumeStakingStorage.RateCheckpoint[])']
 chkCount = checkpoints.length
REF_4234 -> LENGTH checkpoints_1 (-> [])
chkCount_1(uint256) := REF_4234(uint256)
 chkCount > 0
TMP_13965(bool) = chkCount_1 > 0
CONDITION TMP_13965
 idx = findRewardRateCheckpointIndexAtOrBefore($,validatorId,token,timestamp)
TMP_13966(uint256) = INTERNAL_CALL, PlumeRewardLogic.findRewardRateCheckpointIndexAtOrBefore(PlumeStakingStorage.Layout,uint16,address,uint256)($_1 (-> []),validatorId_1,token_1,timestamp_1)
idx_1(uint256) := TMP_13966(uint256)
 idx < chkCount && checkpoints[idx].timestamp <= timestamp
TMP_13967(bool) = idx_1 < chkCount_1
REF_4235(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[idx_1]
REF_4236(uint256) -> REF_4235.timestamp
TMP_13968(bool) = REF_4236 <= timestamp_1
TMP_13969(bool) = TMP_13967 && TMP_13968
CONDITION TMP_13969
 idx + 1 < chkCount && checkpoints[idx + 1].timestamp <= timestamp
TMP_13970(uint256) = idx_1 (c)+ 1
TMP_13971(bool) = TMP_13970 < chkCount_1
TMP_13972(uint256) = idx_1 (c)+ 1
REF_4237(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[TMP_13972]
REF_4238(uint256) -> REF_4237.timestamp
TMP_13973(bool) = REF_4238 <= timestamp_1
TMP_13974(bool) = TMP_13971 && TMP_13973
CONDITION TMP_13974
 checkpoints[idx]
REF_4239(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[idx_1]
RETURN REF_4239
 effectiveCheckpoint.rate = $.rewardRates[token]
REF_4240(uint256) -> effectiveCheckpoint_0.rate
REF_4241(mapping(address => uint256)) -> $_1 (-> []).rewardRates
REF_4242(uint256) -> REF_4241[token_1]
effectiveCheckpoint_1(PlumeStakingStorage.RateCheckpoint) := phi(['effectiveCheckpoint_0'])
REF_4240(uint256) (->effectiveCheckpoint_1) := REF_4242(uint256)
 effectiveCheckpoint.timestamp = timestamp
REF_4243(uint256) -> effectiveCheckpoint_1.timestamp
effectiveCheckpoint_2(PlumeStakingStorage.RateCheckpoint) := phi(['effectiveCheckpoint_1'])
REF_4243(uint256) (->effectiveCheckpoint_2) := timestamp_1(uint256)
 effectiveCheckpoint.cumulativeIndex = 0
REF_4244(uint256) -> effectiveCheckpoint_2.cumulativeIndex
effectiveCheckpoint_3(PlumeStakingStorage.RateCheckpoint) := phi(['effectiveCheckpoint_2'])
REF_4244(uint256) (->effectiveCheckpoint_3) := 0(uint256)
 effectiveCheckpoint
RETURN effectiveCheckpoint_3
 effectiveCheckpoint
```
#### PlumeRewardLogic.getDistinctTimestamps(PlumeStakingStorage.Layout,uint16,address,uint256,uint256) [INTERNAL]
```slithir
$_1 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])'])
validatorId_1(uint16) := phi(['validatorId_1'])
token_1(address) := phi(['token_1'])
periodStart_1(uint256) := phi(['lastUserRewardUpdateTime_5'])
periodEnd_1(uint256) := phi(['block.timestamp'])
 rewardCheckpoints = $.validatorRewardRateCheckpoints[validatorId][token]
REF_4212(mapping(uint16 => mapping(address => PlumeStakingStorage.RateCheckpoint[]))) -> $_1 (-> []).validatorRewardRateCheckpoints
REF_4213(mapping(address => PlumeStakingStorage.RateCheckpoint[])) -> REF_4212[validatorId_1]
REF_4214(PlumeStakingStorage.RateCheckpoint[]) -> REF_4213[token_1]
rewardCheckpoints_1 (-> [])(PlumeStakingStorage.RateCheckpoint[]) = ['REF_4214(PlumeStakingStorage.RateCheckpoint[])']
 commissionCheckpoints = $.validatorCommissionCheckpoints[validatorId]
REF_4215(mapping(uint16 => PlumeStakingStorage.RateCheckpoint[])) -> $_1 (-> []).validatorCommissionCheckpoints
REF_4216(PlumeStakingStorage.RateCheckpoint[]) -> REF_4215[validatorId_1]
commissionCheckpoints_1 (-> [])(PlumeStakingStorage.RateCheckpoint[]) = ['REF_4216(PlumeStakingStorage.RateCheckpoint[])']
 len1 = rewardCheckpoints.length
REF_4217 -> LENGTH rewardCheckpoints_1 (-> [])
len1_1(uint256) := REF_4217(uint256)
 len2 = commissionCheckpoints.length
REF_4218 -> LENGTH commissionCheckpoints_1 (-> [])
len2_1(uint256) := REF_4218(uint256)
 periodStart > periodEnd
TMP_13924(bool) = periodStart_1 > periodEnd_1
CONDITION TMP_13924
 new uint256[](0)
TMP_13926(uint256[])  = new uint256[](0)
RETURN TMP_13926
 periodStart == periodEnd
TMP_13927(bool) = periodStart_1 == periodEnd_1
CONDITION TMP_13927
 singlePoint = new uint256[](1)
TMP_13929(uint256[])  = new uint256[](1)
singlePoint_1(uint256[]) = ['TMP_13929(uint256[])']
 singlePoint[0] = periodStart
REF_4219(uint256) -> singlePoint_1[0]
singlePoint_2(uint256[]) := phi(['singlePoint_1'])
REF_4219(uint256) (->singlePoint_2) := periodStart_1(uint256)
 singlePoint
RETURN singlePoint_2
 result = new uint256[](len1 + len2 + 2)
TMP_13931(uint256) = len1_1 (c)+ len2_1
TMP_13932(uint256) = TMP_13931 (c)+ 2
TMP_13933(uint256[])  = new uint256[](TMP_13932)
result_1(uint256[]) = ['TMP_13933(uint256[])']
 i = 0
i_1(uint256) := 0(uint256)
 j = 0
j_1(uint256) := 0(uint256)
 k = 0
k_1(uint256) := 0(uint256)
 result[k ++] = periodStart
TMP_13934(uint256) := k_1(uint256)
k_2(uint256) = k_1 (c)+ 1
REF_4220(uint256) -> result_1[TMP_13934]
result_2(uint256[]) := phi(['result_1'])
REF_4220(uint256) (->result_2) := periodStart_1(uint256)
 lastAddedTimestamp = periodStart
lastAddedTimestamp_1(uint256) := periodStart_1(uint256)
 i < len1 && rewardCheckpoints[i].timestamp <= periodStart
i_2(uint256) := phi(['i_5', 'i_1'])
TMP_13935(bool) = i_2 < len1_1
REF_4221(PlumeStakingStorage.RateCheckpoint) -> rewardCheckpoints_1 (-> [])[i_2]
REF_4222(uint256) -> REF_4221.timestamp
TMP_13936(bool) = REF_4222 <= periodStart_1
TMP_13937(bool) = TMP_13935 && TMP_13936
CONDITION TMP_13937
 i ++
TMP_13938(uint256) := i_2(uint256)
i_5(uint256) = i_2 (c)+ 1
 j < len2 && commissionCheckpoints[j].timestamp <= periodStart
j_2(uint256) := phi(['j_3', 'j_1'])
TMP_13939(bool) = j_2 < len2_1
REF_4223(PlumeStakingStorage.RateCheckpoint) -> commissionCheckpoints_1 (-> [])[j_2]
REF_4224(uint256) -> REF_4223.timestamp
TMP_13940(bool) = REF_4224 <= periodStart_1
TMP_13941(bool) = TMP_13939 && TMP_13940
CONDITION TMP_13941
 j ++
TMP_13942(uint256) := j_2(uint256)
j_3(uint256) = j_2 (c)+ 1
 i < len1 || j < len2
TMP_13943(bool) = i_2 < len1_1
TMP_13944(bool) = j_2 < len2_1
TMP_13945(bool) = TMP_13943 || TMP_13944
CONDITION TMP_13945
 advanceI = false
advanceI_1(bool) := False(bool)
 advanceJ = false
advanceJ_1(bool) := False(bool)
 t1 < t2
TMP_13946(bool) = t1_3 < t2_3
CONDITION TMP_13946
 currentTimestampToAdd = t1
currentTimestampToAdd_1(uint256) := t1_3(uint256)
 advanceI = true
advanceI_2(bool) := True(bool)
 t2 < t1
TMP_13947(bool) = t2_3 < t1_3
CONDITION TMP_13947
 currentTimestampToAdd = t2
currentTimestampToAdd_3(uint256) := t2_3(uint256)
 advanceJ = true
advanceJ_3(bool) := True(bool)
 t1 != type()(uint256).max
TMP_13949(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
TMP_13950(bool) = t1_3 != TMP_13949
CONDITION TMP_13950
 currentTimestampToAdd = t1
currentTimestampToAdd_2(uint256) := t1_3(uint256)
 advanceI = true
advanceI_3(bool) := True(bool)
 advanceJ = true
advanceJ_2(bool) := True(bool)
currentTimestampToAdd_4(uint256) := phi(['currentTimestampToAdd_2', 'currentTimestampToAdd_3', 'currentTimestampToAdd_0'])
advanceI_4(bool) := phi(['advanceI_3', 'advanceI_1'])
advanceJ_4(bool) := phi(['advanceJ_1', 'advanceJ_3', 'advanceJ_2'])
currentTimestampToAdd_5(uint256) := phi(['currentTimestampToAdd_1', 'currentTimestampToAdd_0'])
advanceI_5(bool) := phi(['advanceI_1', 'advanceI_2'])
 currentTimestampToAdd >= periodEnd
TMP_13951(bool) = currentTimestampToAdd_5 >= periodEnd_1
CONDITION TMP_13951
 currentTimestampToAdd > lastAddedTimestamp
TMP_13952(bool) = currentTimestampToAdd_5 > lastAddedTimestamp_1
CONDITION TMP_13952
 result[k ++] = currentTimestampToAdd
TMP_13953(uint256) := k_2(uint256)
k_3(uint256) = k_2 (c)+ 1
REF_4225(uint256) -> result_2[TMP_13953]
result_3(uint256[]) := phi(['result_2'])
REF_4225(uint256) (->result_3) := currentTimestampToAdd_5(uint256)
 lastAddedTimestamp = currentTimestampToAdd
lastAddedTimestamp_2(uint256) := currentTimestampToAdd_5(uint256)
k_4(uint256) := phi(['k_2', 'k_3'])
lastAddedTimestamp_3(uint256) := phi(['lastAddedTimestamp_2', 'lastAddedTimestamp_1'])
 advanceI
CONDITION advanceI_5
 i ++
TMP_13954(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
i_4(uint256) := phi(['i_3', 'i_1'])
 advanceJ
CONDITION advanceJ_4
 j ++
TMP_13955(uint256) := j_2(uint256)
j_4(uint256) = j_2 (c)+ 1
j_5(uint256) := phi(['j_4', 'j_1'])
 lastAddedTimestamp < periodEnd
TMP_13956(bool) = lastAddedTimestamp_1 < periodEnd_1
CONDITION TMP_13956
 result[k ++] = periodEnd
TMP_13957(uint256) := k_2(uint256)
k_5(uint256) = k_2 (c)+ 1
REF_4226(uint256) -> result_2[TMP_13957]
result_4(uint256[]) := phi(['result_2'])
REF_4226(uint256) (->result_4) := periodEnd_1(uint256)
result_5(uint256[]) := phi(['result_4', 'result_2'])
k_6(uint256) := phi(['k_5', 'k_2'])
 mstore(uint256,uint256)(result,k)
TMP_13958(None) = SOLIDITY_CALL mstore(uint256,uint256)(result_5,k_6)
 result
RETURN result_5
 (i < len1)
TMP_13959(bool) = i_2 < len1_1
CONDITION TMP_13959
 t1 = rewardCheckpoints[i].timestamp
REF_4227(PlumeStakingStorage.RateCheckpoint) -> rewardCheckpoints_1 (-> [])[i_2]
REF_4228(uint256) -> REF_4227.timestamp
t1_1(uint256) := REF_4228(uint256)
 t1 = type()(uint256).max
TMP_13961(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
t1_2(uint256) := TMP_13961(uint256)
t1_3(uint256) := phi(['t1_1', 't1_2'])
 (j < len2)
TMP_13962(bool) = j_2 < len2_1
CONDITION TMP_13962
 t2 = commissionCheckpoints[j].timestamp
REF_4229(PlumeStakingStorage.RateCheckpoint) -> commissionCheckpoints_1 (-> [])[j_2]
REF_4230(uint256) -> REF_4229.timestamp
t2_1(uint256) := REF_4230(uint256)
 t2 = type()(uint256).max
TMP_13964(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
t2_2(uint256) := TMP_13964(uint256)
t2_3(uint256) := phi(['t2_1', 't2_2'])
```
#### PlumeRewardLogic._ceilDiv(uint256,uint256) [INTERNAL]
```slithir
a_1(uint256) := phi(['TMP_13916'])
b_1(uint256) := phi(['REF_4211'])
 b == 0
TMP_13920(bool) = b_1 == 0
CONDITION TMP_13920
 0
RETURN 0
 (a + b - 1) / b
TMP_13921(uint256) = a_1 (c)+ b_1
TMP_13922(uint256) = TMP_13921 (c)- 1
TMP_13923(uint256) = TMP_13922 (c)/ b_1
RETURN TMP_13923
 result
```
#### PlumeRewardLogic.findCommissionCheckpointIndexAtOrBefore(PlumeStakingStorage.Layout,uint16,uint256) [INTERNAL]
```slithir
$_1 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])'])
validatorId_1(uint16) := phi(['validatorId_1'])
timestamp_1(uint256) := phi(['timestamp_1'])
 checkpoints = $.validatorCommissionCheckpoints[validatorId]
REF_4261(mapping(uint16 => PlumeStakingStorage.RateCheckpoint[])) -> $_1 (-> []).validatorCommissionCheckpoints
REF_4262(PlumeStakingStorage.RateCheckpoint[]) -> REF_4261[validatorId_1]
checkpoints_1 (-> [])(PlumeStakingStorage.RateCheckpoint[]) = ['REF_4262(PlumeStakingStorage.RateCheckpoint[])']
 len = checkpoints.length
REF_4263 -> LENGTH checkpoints_1 (-> [])
len_1(uint256) := REF_4263(uint256)
 len == 0
TMP_13990(bool) = len_1 == 0
CONDITION TMP_13990
 0
RETURN 0
 low = 0
low_1(uint256) := 0(uint256)
 high = len - 1
TMP_13991(uint256) = len_1 (c)- 1
high_1(uint256) := TMP_13991(uint256)
 ans = 0
ans_1(uint256) := 0(uint256)
 foundSuitable = false
foundSuitable_1(bool) := False(bool)
 low <= high
TMP_13992(bool) = low_1 <= high_1
CONDITION TMP_13992
 mid = low + (high - low) / 2
TMP_13993(uint256) = high_1 (c)- low_1
TMP_13994(uint256) = TMP_13993 (c)/ 2
TMP_13995(uint256) = low_1 (c)+ TMP_13994
mid_1(uint256) := TMP_13995(uint256)
 checkpoints[mid].timestamp <= timestamp
REF_4264(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[mid_1]
REF_4265(uint256) -> REF_4264.timestamp
TMP_13996(bool) = REF_4265 <= timestamp_1
CONDITION TMP_13996
 ans = mid
ans_2(uint256) := mid_1(uint256)
 foundSuitable = true
foundSuitable_2(bool) := True(bool)
 low = mid + 1
TMP_13997(uint256) = mid_1 (c)+ 1
low_2(uint256) := TMP_13997(uint256)
 mid == 0
TMP_13998(bool) = mid_1 == 0
CONDITION TMP_13998
 high = mid - 1
TMP_13999(uint256) = mid_1 (c)- 1
high_2(uint256) := TMP_13999(uint256)
low_3(uint256) := phi(['low_1', 'low_2'])
high_3(uint256) := phi(['high_2', 'high_1'])
ans_3(uint256) := phi(['ans_1', 'ans_2'])
 ans
RETURN ans_1
```
#### PlumeRewardLogic.findRewardRateCheckpointIndexAtOrBefore(PlumeStakingStorage.Layout,uint16,address,uint256) [INTERNAL]
```slithir
$_1 (-> [])(PlumeStakingStorage.Layout) := phi(['$_1 (-> [])'])
validatorId_1(uint16) := phi(['validatorId_1'])
token_1(address) := phi(['token_1'])
timestamp_1(uint256) := phi(['timestamp_1'])
 checkpoints = $.validatorRewardRateCheckpoints[validatorId][token]
REF_4255(mapping(uint16 => mapping(address => PlumeStakingStorage.RateCheckpoint[]))) -> $_1 (-> []).validatorRewardRateCheckpoints
REF_4256(mapping(address => PlumeStakingStorage.RateCheckpoint[])) -> REF_4255[validatorId_1]
REF_4257(PlumeStakingStorage.RateCheckpoint[]) -> REF_4256[token_1]
checkpoints_1 (-> [])(PlumeStakingStorage.RateCheckpoint[]) = ['REF_4257(PlumeStakingStorage.RateCheckpoint[])']
 len = checkpoints.length
REF_4258 -> LENGTH checkpoints_1 (-> [])
len_1(uint256) := REF_4258(uint256)
 len == 0
TMP_13980(bool) = len_1 == 0
CONDITION TMP_13980
 0
RETURN 0
 low = 0
low_1(uint256) := 0(uint256)
 high = len - 1
TMP_13981(uint256) = len_1 (c)- 1
high_1(uint256) := TMP_13981(uint256)
 ans = 0
ans_1(uint256) := 0(uint256)
 foundSuitable = false
foundSuitable_1(bool) := False(bool)
 low <= high
TMP_13982(bool) = low_1 <= high_1
CONDITION TMP_13982
 mid = low + (high - low) / 2
TMP_13983(uint256) = high_1 (c)- low_1
TMP_13984(uint256) = TMP_13983 (c)/ 2
TMP_13985(uint256) = low_1 (c)+ TMP_13984
mid_1(uint256) := TMP_13985(uint256)
 checkpoints[mid].timestamp <= timestamp
REF_4259(PlumeStakingStorage.RateCheckpoint) -> checkpoints_1 (-> [])[mid_1]
REF_4260(uint256) -> REF_4259.timestamp
TMP_13986(bool) = REF_4260 <= timestamp_1
CONDITION TMP_13986
 ans = mid
ans_2(uint256) := mid_1(uint256)
 foundSuitable = true
foundSuitable_2(bool) := True(bool)
 low = mid + 1
TMP_13987(uint256) = mid_1 (c)+ 1
low_2(uint256) := TMP_13987(uint256)
 mid == 0
TMP_13988(bool) = mid_1 == 0
CONDITION TMP_13988
 high = mid - 1
TMP_13989(uint256) = mid_1 (c)- 1
high_2(uint256) := TMP_13989(uint256)
low_3(uint256) := phi(['low_1', 'low_2'])
high_3(uint256) := phi(['high_1', 'high_2'])
ans_3(uint256) := phi(['ans_2', 'ans_1'])
 ans
RETURN ans_1
```
