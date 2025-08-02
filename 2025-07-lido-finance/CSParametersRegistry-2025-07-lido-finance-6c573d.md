
### Storage layout (CSParametersRegistry) 

```text
defaultKeyRemovalCharge uint256
_keyRemovalCharges mapping(uint256 => ICSParametersRegistry.MarkedUint248)
defaultElRewardsStealingAdditionalFine uint256
_elRewardsStealingAdditionalFines mapping(uint256 => ICSParametersRegistry.MarkedUint248)
defaultKeysLimit uint256
_keysLimits mapping(uint256 => ICSParametersRegistry.MarkedUint248)
defaultQueueConfig ICSParametersRegistry.QueueConfig
_queueConfigs mapping(uint256 => ICSParametersRegistry.QueueConfig)
defaultRewardShare uint256
_rewardShareData mapping(uint256 => ICSParametersRegistry.KeyNumberValueInterval[])
defaultPerformanceLeeway uint256
_performanceLeewayData mapping(uint256 => ICSParametersRegistry.KeyNumberValueInterval[])
defaultStrikesParams ICSParametersRegistry.StrikesParams
_strikesParams mapping(uint256 => ICSParametersRegistry.StrikesParams)
defaultBadPerformancePenalty uint256
_badPerformancePenalties mapping(uint256 => ICSParametersRegistry.MarkedUint248)
defaultPerformanceCoefficients ICSParametersRegistry.PerformanceCoefficients
_performanceCoefficients mapping(uint256 => ICSParametersRegistry.PerformanceCoefficients)
defaultAllowedExitDelay uint256
_allowedExitDelay mapping(uint256 => uint256)
defaultExitDelayPenalty uint256
_exitDelayPenalties mapping(uint256 => ICSParametersRegistry.MarkedUint248)
defaultMaxWithdrawalRequestFee uint256
_maxWithdrawalRequestFees mapping(uint256 => ICSParametersRegistry.MarkedUint248)

```
#### CSParametersRegistry.setDefaultKeyRemovalCharge(uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_17(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
 _setDefaultKeyRemovalCharge(keyRemovalCharge)
INTERNAL_CALL, CSParametersRegistry._setDefaultKeyRemovalCharge(uint256)(keyRemovalCharge_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_17)
```
#### CSParametersRegistry.setDefaultElRewardsStealingAdditionalFine(uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_19(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
 _setDefaultElRewardsStealingAdditionalFine(fine)
INTERNAL_CALL, CSParametersRegistry._setDefaultElRewardsStealingAdditionalFine(uint256)(fine_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_19)
```
#### CSParametersRegistry.setDefaultKeysLimit(uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_21(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
 _setDefaultKeysLimit(limit)
INTERNAL_CALL, CSParametersRegistry._setDefaultKeysLimit(uint256)(limit_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_21)
```
#### CSParametersRegistry.setDefaultRewardShare(uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_23(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
 _setDefaultRewardShare(share)
INTERNAL_CALL, CSParametersRegistry._setDefaultRewardShare(uint256)(share_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_23)
```
#### CSParametersRegistry.setDefaultPerformanceLeeway(uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_25(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
 _setDefaultPerformanceLeeway(leeway)
INTERNAL_CALL, CSParametersRegistry._setDefaultPerformanceLeeway(uint256)(leeway_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_25)
```
#### CSParametersRegistry.setDefaultStrikesParams(uint256,uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_27(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
 _setDefaultStrikesParams(lifetime,threshold)
INTERNAL_CALL, CSParametersRegistry._setDefaultStrikesParams(uint256,uint256)(lifetime_1,threshold_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_27)
```
#### CSParametersRegistry.setDefaultBadPerformancePenalty(uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_29(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
 _setDefaultBadPerformancePenalty(penalty)
INTERNAL_CALL, CSParametersRegistry._setDefaultBadPerformancePenalty(uint256)(penalty_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_29)
```
#### CSParametersRegistry.setDefaultPerformanceCoefficients(uint256,uint256,uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_31(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
 _setDefaultPerformanceCoefficients(attestationsWeight,blocksWeight,syncWeight)
INTERNAL_CALL, CSParametersRegistry._setDefaultPerformanceCoefficients(uint256,uint256,uint256)(attestationsWeight_1,blocksWeight_1,syncWeight_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_31)
```
#### CSParametersRegistry.setDefaultAllowedExitDelay(uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_35(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
 _setDefaultAllowedExitDelay(delay)
INTERNAL_CALL, CSParametersRegistry._setDefaultAllowedExitDelay(uint256)(delay_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_35)
```
#### CSParametersRegistry.setDefaultExitDelayPenalty(uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_37(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
 _setDefaultExitDelayPenalty(penalty)
INTERNAL_CALL, CSParametersRegistry._setDefaultExitDelayPenalty(uint256)(penalty_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_37)
```
#### CSParametersRegistry.setDefaultMaxWithdrawalRequestFee(uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_39(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
 _setDefaultMaxWithdrawalRequestFee(fee)
INTERNAL_CALL, CSParametersRegistry._setDefaultMaxWithdrawalRequestFee(uint256)(fee_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_39)
```
#### CSParametersRegistry.setKeyRemovalCharge(uint256,uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_41(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
 _keyRemovalCharges[curveId] = MarkedUint248(keyRemovalCharge.toUint248(),true)
REF_1223(ICSParametersRegistry.MarkedUint248) -> _keyRemovalCharges_0[curveId_1]
TMP_3055(uint248) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint248(uint256), arguments:['keyRemovalCharge_1'] 
TMP_3056(ICSParametersRegistry.MarkedUint248) = new MarkedUint248(TMP_3055,True)
_keyRemovalCharges_1(mapping(uint256 => ICSParametersRegistry.MarkedUint248)) := phi(['_keyRemovalCharges_0'])
REF_1223(ICSParametersRegistry.MarkedUint248) (->_keyRemovalCharges_1) := TMP_3056(ICSParametersRegistry.MarkedUint248)
 KeyRemovalChargeSet(curveId,keyRemovalCharge)
Emit KeyRemovalChargeSet(curveId_1,keyRemovalCharge_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_41)
```
#### CSParametersRegistry.unsetKeyRemovalCharge(uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_43(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
_keyRemovalCharges_2(mapping(uint256 => ICSParametersRegistry.MarkedUint248)) := phi(['_keyRemovalCharges_4', '_keyRemovalCharges_1', '_keyRemovalCharges_0', '_keyRemovalCharges_5'])
 delete _keyRemovalCharges[curveId]
REF_1225(ICSParametersRegistry.MarkedUint248) -> _keyRemovalCharges_3[curveId_1]
_keyRemovalCharges_4 = delete REF_1225 
 KeyRemovalChargeUnset(curveId)
Emit KeyRemovalChargeUnset(curveId_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_43)
```
#### CSParametersRegistry.getKeyRemovalCharge(uint256) [EXTERNAL]
```slithir
defaultKeyRemovalCharge_1(uint256) := phi(['defaultKeyRemovalCharge_2', 'defaultKeyRemovalCharge_0'])
_keyRemovalCharges_5(mapping(uint256 => ICSParametersRegistry.MarkedUint248)) := phi(['_keyRemovalCharges_4', '_keyRemovalCharges_1', '_keyRemovalCharges_0', '_keyRemovalCharges_5'])
 data = _keyRemovalCharges[curveId]
REF_1274(ICSParametersRegistry.MarkedUint248) -> _keyRemovalCharges_5[curveId_1]
data_1 (-> ['_keyRemovalCharges'])(ICSParametersRegistry.MarkedUint248) := REF_1274(ICSParametersRegistry.MarkedUint248)
 data.isValue
REF_1275(bool) -> data_1 (-> ['_keyRemovalCharges']).isValue
CONDITION REF_1275
 data.value
REF_1276(uint248) -> data_1 (-> ['_keyRemovalCharges']).value
RETURN REF_1276
 defaultKeyRemovalCharge
RETURN defaultKeyRemovalCharge_1
 keyRemovalCharge
```
#### CSParametersRegistry.setElRewardsStealingAdditionalFine(uint256,uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_45(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
 _elRewardsStealingAdditionalFines[curveId] = MarkedUint248(fine.toUint248(),true)
REF_1226(ICSParametersRegistry.MarkedUint248) -> _elRewardsStealingAdditionalFines_0[curveId_1]
TMP_3061(uint248) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint248(uint256), arguments:['fine_1'] 
TMP_3062(ICSParametersRegistry.MarkedUint248) = new MarkedUint248(TMP_3061,True)
_elRewardsStealingAdditionalFines_1(mapping(uint256 => ICSParametersRegistry.MarkedUint248)) := phi(['_elRewardsStealingAdditionalFines_0'])
REF_1226(ICSParametersRegistry.MarkedUint248) (->_elRewardsStealingAdditionalFines_1) := TMP_3062(ICSParametersRegistry.MarkedUint248)
 ElRewardsStealingAdditionalFineSet(curveId,fine)
Emit ElRewardsStealingAdditionalFineSet(curveId_1,fine_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_45)
```
#### CSParametersRegistry.unsetElRewardsStealingAdditionalFine(uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_47(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
_elRewardsStealingAdditionalFines_2(mapping(uint256 => ICSParametersRegistry.MarkedUint248)) := phi(['_elRewardsStealingAdditionalFines_5', '_elRewardsStealingAdditionalFines_1', '_elRewardsStealingAdditionalFines_0', '_elRewardsStealingAdditionalFines_4'])
 delete _elRewardsStealingAdditionalFines[curveId]
REF_1228(ICSParametersRegistry.MarkedUint248) -> _elRewardsStealingAdditionalFines_3[curveId_1]
_elRewardsStealingAdditionalFines_4 = delete REF_1228 
 ElRewardsStealingAdditionalFineUnset(curveId)
Emit ElRewardsStealingAdditionalFineUnset(curveId_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_47)
```
#### CSParametersRegistry.getElRewardsStealingAdditionalFine(uint256) [EXTERNAL]
```slithir
defaultElRewardsStealingAdditionalFine_1(uint256) := phi(['defaultElRewardsStealingAdditionalFine_0', 'defaultElRewardsStealingAdditionalFine_2'])
_elRewardsStealingAdditionalFines_5(mapping(uint256 => ICSParametersRegistry.MarkedUint248)) := phi(['_elRewardsStealingAdditionalFines_5', '_elRewardsStealingAdditionalFines_1', '_elRewardsStealingAdditionalFines_0', '_elRewardsStealingAdditionalFines_4'])
 data = _elRewardsStealingAdditionalFines[curveId]
REF_1277(ICSParametersRegistry.MarkedUint248) -> _elRewardsStealingAdditionalFines_5[curveId_1]
data_1 (-> ['_elRewardsStealingAdditionalFines'])(ICSParametersRegistry.MarkedUint248) := REF_1277(ICSParametersRegistry.MarkedUint248)
 data.isValue
REF_1278(bool) -> data_1 (-> ['_elRewardsStealingAdditionalFines']).isValue
CONDITION REF_1278
 data.value
REF_1279(uint248) -> data_1 (-> ['_elRewardsStealingAdditionalFines']).value
RETURN REF_1279
 defaultElRewardsStealingAdditionalFine
RETURN defaultElRewardsStealingAdditionalFine_1
 fine
```
#### CSParametersRegistry.setKeysLimit(uint256,uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_49(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
 _keysLimits[curveId] = MarkedUint248(limit.toUint248(),true)
REF_1229(ICSParametersRegistry.MarkedUint248) -> _keysLimits_0[curveId_1]
TMP_3067(uint248) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint248(uint256), arguments:['limit_1'] 
TMP_3068(ICSParametersRegistry.MarkedUint248) = new MarkedUint248(TMP_3067,True)
_keysLimits_1(mapping(uint256 => ICSParametersRegistry.MarkedUint248)) := phi(['_keysLimits_0'])
REF_1229(ICSParametersRegistry.MarkedUint248) (->_keysLimits_1) := TMP_3068(ICSParametersRegistry.MarkedUint248)
 KeysLimitSet(curveId,limit)
Emit KeysLimitSet(curveId_1,limit_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_49)
```
#### CSParametersRegistry.unsetKeysLimit(uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_51(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
_keysLimits_2(mapping(uint256 => ICSParametersRegistry.MarkedUint248)) := phi(['_keysLimits_4', '_keysLimits_0', '_keysLimits_1', '_keysLimits_5'])
 delete _keysLimits[curveId]
REF_1231(ICSParametersRegistry.MarkedUint248) -> _keysLimits_3[curveId_1]
_keysLimits_4 = delete REF_1231 
 KeysLimitUnset(curveId)
Emit KeysLimitUnset(curveId_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_51)
```
#### CSParametersRegistry.getKeysLimit(uint256) [EXTERNAL]
```slithir
defaultKeysLimit_1(uint256) := phi(['defaultKeysLimit_2', 'defaultKeysLimit_0'])
_keysLimits_5(mapping(uint256 => ICSParametersRegistry.MarkedUint248)) := phi(['_keysLimits_4', '_keysLimits_0', '_keysLimits_1', '_keysLimits_5'])
 data = _keysLimits[curveId]
REF_1280(ICSParametersRegistry.MarkedUint248) -> _keysLimits_5[curveId_1]
data_1 (-> ['_keysLimits'])(ICSParametersRegistry.MarkedUint248) := REF_1280(ICSParametersRegistry.MarkedUint248)
 data.isValue
REF_1281(bool) -> data_1 (-> ['_keysLimits']).isValue
CONDITION REF_1281
 data.value
REF_1282(uint248) -> data_1 (-> ['_keysLimits']).value
RETURN REF_1282
 defaultKeysLimit
RETURN defaultKeysLimit_1
 limit
```
#### CSParametersRegistry.setRewardShareData(uint256,ICSParametersRegistry.KeyNumberValueInterval[]) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_53(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
_rewardShareData_1(mapping(uint256 => ICSParametersRegistry.KeyNumberValueInterval[])) := phi(['_rewardShareData_3', '_rewardShareData_8', '_rewardShareData_0', '_rewardShareData_7', '_rewardShareData_4'])
 _validateKeyNumberValueIntervals(data)
INTERNAL_CALL, CSParametersRegistry._validateKeyNumberValueIntervals(ICSParametersRegistry.KeyNumberValueInterval[])(data_1)
 intervals = _rewardShareData[curveId]
REF_1232(ICSParametersRegistry.KeyNumberValueInterval[]) -> _rewardShareData_3[curveId_1]
intervals_1 (-> [])(ICSParametersRegistry.KeyNumberValueInterval[]) = ['REF_1232(ICSParametersRegistry.KeyNumberValueInterval[])']
 intervals.length > 0
REF_1233 -> LENGTH intervals_1 (-> [])
TMP_3074(bool) = REF_1233 > 0
CONDITION TMP_3074
 delete _rewardShareData[curveId]
REF_1234(ICSParametersRegistry.KeyNumberValueInterval[]) -> _rewardShareData_3[curveId_1]
_rewardShareData_4 = delete REF_1234 
 i = 0
i_1(uint256) := 0(uint256)
 i < data.length
intervals_2 (-> [])(ICSParametersRegistry.KeyNumberValueInterval[]) := phi(['intervals_4 (-> [])', 'intervals_1 (-> [])'])
i_2(uint256) := phi(['i_3', 'i_1'])
REF_1235 -> LENGTH data_1
TMP_3075(bool) = i_2 < REF_1235
CONDITION TMP_3075
 intervals.push(data[i])
REF_1237(ICSParametersRegistry.KeyNumberValueInterval) -> data_1[i_2]
REF_1238 -> LENGTH intervals_2 (-> [])
TMP_3077(uint256) := REF_1238(uint256)
TMP_3078(uint256) = TMP_3077 (c)+ 1
intervals_3 (-> [])(ICSParametersRegistry.KeyNumberValueInterval[]) := phi(['intervals_2 (-> [])'])
REF_1238(uint256) (->intervals_3 (-> [])) := TMP_3078(uint256)
REF_1239(ICSParametersRegistry.KeyNumberValueInterval) -> intervals_3 (-> [])[TMP_3077]
intervals_4 (-> [])(ICSParametersRegistry.KeyNumberValueInterval[]) := phi(['intervals_3 (-> [])'])
REF_1239(ICSParametersRegistry.KeyNumberValueInterval) (->intervals_4 (-> [])) := REF_1237(ICSParametersRegistry.KeyNumberValueInterval)
 ++ i
i_3(uint256) = i_2 (c)+ 1
 RewardShareDataSet(curveId,data)
Emit RewardShareDataSet(curveId_1,data_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_53)
```
#### CSParametersRegistry.unsetRewardShareData(uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_55(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
_rewardShareData_5(mapping(uint256 => ICSParametersRegistry.KeyNumberValueInterval[])) := phi(['_rewardShareData_3', '_rewardShareData_8', '_rewardShareData_0', '_rewardShareData_7', '_rewardShareData_4'])
 delete _rewardShareData[curveId]
REF_1240(ICSParametersRegistry.KeyNumberValueInterval[]) -> _rewardShareData_6[curveId_1]
_rewardShareData_7 = delete REF_1240 
 RewardShareDataUnset(curveId)
Emit RewardShareDataUnset(curveId_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_55)
```
#### CSParametersRegistry.getRewardShareData(uint256) [EXTERNAL]
```slithir
defaultRewardShare_1(uint256) := phi(['defaultRewardShare_2', 'defaultRewardShare_0'])
_rewardShareData_8(mapping(uint256 => ICSParametersRegistry.KeyNumberValueInterval[])) := phi(['_rewardShareData_3', '_rewardShareData_8', '_rewardShareData_0', '_rewardShareData_7', '_rewardShareData_4'])
 data = _rewardShareData[curveId]
REF_1283(ICSParametersRegistry.KeyNumberValueInterval[]) -> _rewardShareData_8[curveId_1]
data_1(ICSParametersRegistry.KeyNumberValueInterval[]) = ['REF_1283(ICSParametersRegistry.KeyNumberValueInterval[])']
 data.length == 0
REF_1284 -> LENGTH data_1
TMP_3141(bool) = REF_1284 == 0
CONDITION TMP_3141
 data = new ICSParametersRegistry.KeyNumberValueInterval[](1)
TMP_3143(ICSParametersRegistry.KeyNumberValueInterval[])  = new ICSParametersRegistry.KeyNumberValueInterval[](1)
data_2(ICSParametersRegistry.KeyNumberValueInterval[]) = ['TMP_3143(ICSParametersRegistry.KeyNumberValueInterval[])']
 data[0] = KeyNumberValueInterval(1,defaultRewardShare)
REF_1285(ICSParametersRegistry.KeyNumberValueInterval) -> data_2[0]
TMP_3144(ICSParametersRegistry.KeyNumberValueInterval) = new KeyNumberValueInterval(1,defaultRewardShare_1)
data_3(ICSParametersRegistry.KeyNumberValueInterval[]) := phi(['data_2'])
REF_1285(ICSParametersRegistry.KeyNumberValueInterval) (->data_3) := TMP_3144(ICSParametersRegistry.KeyNumberValueInterval)
data_4(ICSParametersRegistry.KeyNumberValueInterval[]) := phi(['data_3', 'data_1'])
 data
RETURN data_4
```
#### CSParametersRegistry.setDefaultQueueConfig(uint256,uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_33(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
 _setDefaultQueueConfig(priority,maxDeposits)
INTERNAL_CALL, CSParametersRegistry._setDefaultQueueConfig(uint256,uint256)(priority_1,maxDeposits_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_33)
```
#### CSParametersRegistry.setQueueConfig(uint256,uint256,uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_73(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
 _validateQueueConfig(priority,maxDeposits)
INTERNAL_CALL, CSParametersRegistry._validateQueueConfig(uint256,uint256)(priority_1,maxDeposits_1)
 _queueConfigs[curveId] = QueueConfig({priority:priority.toUint32(),maxDeposits:maxDeposits.toUint32()})
REF_1262(ICSParametersRegistry.QueueConfig) -> _queueConfigs_0[curveId_1]
TMP_3117(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['priority_1'] 
TMP_3118(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['maxDeposits_1'] 
TMP_3119(ICSParametersRegistry.QueueConfig) = new QueueConfig(TMP_3117,TMP_3118)
_queueConfigs_1(mapping(uint256 => ICSParametersRegistry.QueueConfig)) := phi(['_queueConfigs_0'])
REF_1262(ICSParametersRegistry.QueueConfig) (->_queueConfigs_1) := TMP_3119(ICSParametersRegistry.QueueConfig)
 QueueConfigSet(curveId,priority,maxDeposits)
Emit QueueConfigSet(curveId_1,priority_1,maxDeposits_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_73)
```
#### CSParametersRegistry.unsetQueueConfig(uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_75(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
_queueConfigs_2(mapping(uint256 => ICSParametersRegistry.QueueConfig)) := phi(['_queueConfigs_0', '_queueConfigs_4', '_queueConfigs_1', '_queueConfigs_5'])
 delete _queueConfigs[curveId]
REF_1265(ICSParametersRegistry.QueueConfig) -> _queueConfigs_3[curveId_1]
_queueConfigs_4 = delete REF_1265 
 QueueConfigUnset(curveId)
Emit QueueConfigUnset(curveId_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_75)
```
#### CSParametersRegistry.getQueueConfig(uint256) [EXTERNAL]
```slithir
defaultQueueConfig_1(ICSParametersRegistry.QueueConfig) := phi(['defaultQueueConfig_2', 'defaultQueueConfig_0', 'defaultQueueConfig_1'])
_queueConfigs_5(mapping(uint256 => ICSParametersRegistry.QueueConfig)) := phi(['_queueConfigs_0', '_queueConfigs_4', '_queueConfigs_1', '_queueConfigs_5'])
 config = _queueConfigs[curveId]
REF_1308(ICSParametersRegistry.QueueConfig) -> _queueConfigs_5[curveId_1]
config_1 (-> ['_queueConfigs'])(ICSParametersRegistry.QueueConfig) := REF_1308(ICSParametersRegistry.QueueConfig)
 config.maxDeposits == 0
REF_1309(uint32) -> config_1 (-> ['_queueConfigs']).maxDeposits
TMP_3155(bool) = REF_1309 == 0
CONDITION TMP_3155
 (defaultQueueConfig.priority,defaultQueueConfig.maxDeposits)
REF_1310(uint32) -> defaultQueueConfig_1.priority
REF_1311(uint32) -> defaultQueueConfig_1.maxDeposits
RETURN REF_1310,REF_1311
 (config.priority,config.maxDeposits)
REF_1312(uint32) -> config_1 (-> ['_queueConfigs']).priority
REF_1313(uint32) -> config_1 (-> ['_queueConfigs']).maxDeposits
RETURN REF_1312,REF_1313
 (queuePriority,maxDeposits)
```
#### CSParametersRegistry.setPerformanceLeewayData(uint256,ICSParametersRegistry.KeyNumberValueInterval[]) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_57(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
_performanceLeewayData_1(mapping(uint256 => ICSParametersRegistry.KeyNumberValueInterval[])) := phi(['_performanceLeewayData_7', '_performanceLeewayData_4', '_performanceLeewayData_0', '_performanceLeewayData_8', '_performanceLeewayData_3'])
 _validateKeyNumberValueIntervals(data)
INTERNAL_CALL, CSParametersRegistry._validateKeyNumberValueIntervals(ICSParametersRegistry.KeyNumberValueInterval[])(data_1)
 intervals = _performanceLeewayData[curveId]
REF_1241(ICSParametersRegistry.KeyNumberValueInterval[]) -> _performanceLeewayData_3[curveId_1]
intervals_1 (-> [])(ICSParametersRegistry.KeyNumberValueInterval[]) = ['REF_1241(ICSParametersRegistry.KeyNumberValueInterval[])']
 intervals.length > 0
REF_1242 -> LENGTH intervals_1 (-> [])
TMP_3084(bool) = REF_1242 > 0
CONDITION TMP_3084
 delete _performanceLeewayData[curveId]
REF_1243(ICSParametersRegistry.KeyNumberValueInterval[]) -> _performanceLeewayData_3[curveId_1]
_performanceLeewayData_4 = delete REF_1243 
 i = 0
i_1(uint256) := 0(uint256)
 i < data.length
intervals_2 (-> [])(ICSParametersRegistry.KeyNumberValueInterval[]) := phi(['intervals_1 (-> [])', 'intervals_4 (-> [])'])
i_2(uint256) := phi(['i_3', 'i_1'])
REF_1244 -> LENGTH data_1
TMP_3085(bool) = i_2 < REF_1244
CONDITION TMP_3085
 intervals.push(data[i])
REF_1246(ICSParametersRegistry.KeyNumberValueInterval) -> data_1[i_2]
REF_1247 -> LENGTH intervals_2 (-> [])
TMP_3087(uint256) := REF_1247(uint256)
TMP_3088(uint256) = TMP_3087 (c)+ 1
intervals_3 (-> [])(ICSParametersRegistry.KeyNumberValueInterval[]) := phi(['intervals_2 (-> [])'])
REF_1247(uint256) (->intervals_3 (-> [])) := TMP_3088(uint256)
REF_1248(ICSParametersRegistry.KeyNumberValueInterval) -> intervals_3 (-> [])[TMP_3087]
intervals_4 (-> [])(ICSParametersRegistry.KeyNumberValueInterval[]) := phi(['intervals_3 (-> [])'])
REF_1248(ICSParametersRegistry.KeyNumberValueInterval) (->intervals_4 (-> [])) := REF_1246(ICSParametersRegistry.KeyNumberValueInterval)
 ++ i
i_3(uint256) = i_2 (c)+ 1
 PerformanceLeewayDataSet(curveId,data)
Emit PerformanceLeewayDataSet(curveId_1,data_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_57)
```
#### CSParametersRegistry.unsetPerformanceLeewayData(uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_59(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
_performanceLeewayData_5(mapping(uint256 => ICSParametersRegistry.KeyNumberValueInterval[])) := phi(['_performanceLeewayData_7', '_performanceLeewayData_4', '_performanceLeewayData_0', '_performanceLeewayData_8', '_performanceLeewayData_3'])
 delete _performanceLeewayData[curveId]
REF_1249(ICSParametersRegistry.KeyNumberValueInterval[]) -> _performanceLeewayData_6[curveId_1]
_performanceLeewayData_7 = delete REF_1249 
 PerformanceLeewayDataUnset(curveId)
Emit PerformanceLeewayDataUnset(curveId_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_59)
```
#### CSParametersRegistry.getPerformanceLeewayData(uint256) [EXTERNAL]
```slithir
defaultPerformanceLeeway_1(uint256) := phi(['defaultPerformanceLeeway_2', 'defaultPerformanceLeeway_0'])
_performanceLeewayData_8(mapping(uint256 => ICSParametersRegistry.KeyNumberValueInterval[])) := phi(['_performanceLeewayData_7', '_performanceLeewayData_4', '_performanceLeewayData_0', '_performanceLeewayData_8', '_performanceLeewayData_3'])
 data = _performanceLeewayData[curveId]
REF_1286(ICSParametersRegistry.KeyNumberValueInterval[]) -> _performanceLeewayData_8[curveId_1]
data_1(ICSParametersRegistry.KeyNumberValueInterval[]) = ['REF_1286(ICSParametersRegistry.KeyNumberValueInterval[])']
 data.length == 0
REF_1287 -> LENGTH data_1
TMP_3145(bool) = REF_1287 == 0
CONDITION TMP_3145
 data = new ICSParametersRegistry.KeyNumberValueInterval[](1)
TMP_3147(ICSParametersRegistry.KeyNumberValueInterval[])  = new ICSParametersRegistry.KeyNumberValueInterval[](1)
data_2(ICSParametersRegistry.KeyNumberValueInterval[]) = ['TMP_3147(ICSParametersRegistry.KeyNumberValueInterval[])']
 data[0] = KeyNumberValueInterval(1,defaultPerformanceLeeway)
REF_1288(ICSParametersRegistry.KeyNumberValueInterval) -> data_2[0]
TMP_3148(ICSParametersRegistry.KeyNumberValueInterval) = new KeyNumberValueInterval(1,defaultPerformanceLeeway_1)
data_3(ICSParametersRegistry.KeyNumberValueInterval[]) := phi(['data_2'])
REF_1288(ICSParametersRegistry.KeyNumberValueInterval) (->data_3) := TMP_3148(ICSParametersRegistry.KeyNumberValueInterval)
data_4(ICSParametersRegistry.KeyNumberValueInterval[]) := phi(['data_1', 'data_3'])
 data
RETURN data_4
```
#### CSParametersRegistry.setStrikesParams(uint256,uint256,uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_61(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
 _validateStrikesParams(lifetime,threshold)
INTERNAL_CALL, CSParametersRegistry._validateStrikesParams(uint256,uint256)(lifetime_1,threshold_1)
 _strikesParams[curveId] = StrikesParams(lifetime.toUint32(),threshold.toUint32())
REF_1250(ICSParametersRegistry.StrikesParams) -> _strikesParams_0[curveId_1]
TMP_3094(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['lifetime_1'] 
TMP_3095(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['threshold_1'] 
TMP_3096(ICSParametersRegistry.StrikesParams) = new StrikesParams(TMP_3094,TMP_3095)
_strikesParams_1(mapping(uint256 => ICSParametersRegistry.StrikesParams)) := phi(['_strikesParams_0'])
REF_1250(ICSParametersRegistry.StrikesParams) (->_strikesParams_1) := TMP_3096(ICSParametersRegistry.StrikesParams)
 StrikesParamsSet(curveId,lifetime,threshold)
Emit StrikesParamsSet(curveId_1,lifetime_1,threshold_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_61)
```
#### CSParametersRegistry.unsetStrikesParams(uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_63(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
_strikesParams_2(mapping(uint256 => ICSParametersRegistry.StrikesParams)) := phi(['_strikesParams_0', '_strikesParams_1', '_strikesParams_4', '_strikesParams_5'])
 delete _strikesParams[curveId]
REF_1253(ICSParametersRegistry.StrikesParams) -> _strikesParams_3[curveId_1]
_strikesParams_4 = delete REF_1253 
 StrikesParamsUnset(curveId)
Emit StrikesParamsUnset(curveId_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_63)
```
#### CSParametersRegistry.getStrikesParams(uint256) [EXTERNAL]
```slithir
defaultStrikesParams_1(ICSParametersRegistry.StrikesParams) := phi(['defaultStrikesParams_1', 'defaultStrikesParams_2', 'defaultStrikesParams_0'])
_strikesParams_5(mapping(uint256 => ICSParametersRegistry.StrikesParams)) := phi(['_strikesParams_0', '_strikesParams_1', '_strikesParams_4', '_strikesParams_5'])
 params = _strikesParams[curveId]
REF_1289(ICSParametersRegistry.StrikesParams) -> _strikesParams_5[curveId_1]
params_1 (-> ['_strikesParams'])(ICSParametersRegistry.StrikesParams) := REF_1289(ICSParametersRegistry.StrikesParams)
 params.threshold == 0
REF_1290(uint32) -> params_1 (-> ['_strikesParams']).threshold
TMP_3149(bool) = REF_1290 == 0
CONDITION TMP_3149
 (defaultStrikesParams.lifetime,defaultStrikesParams.threshold)
REF_1291(uint32) -> defaultStrikesParams_1.lifetime
REF_1292(uint32) -> defaultStrikesParams_1.threshold
RETURN REF_1291,REF_1292
 (params.lifetime,params.threshold)
REF_1293(uint32) -> params_1 (-> ['_strikesParams']).lifetime
REF_1294(uint32) -> params_1 (-> ['_strikesParams']).threshold
RETURN REF_1293,REF_1294
 (lifetime,threshold)
```
#### CSParametersRegistry.setBadPerformancePenalty(uint256,uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_65(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
 _badPerformancePenalties[curveId] = MarkedUint248(penalty.toUint248(),true)
REF_1254(ICSParametersRegistry.MarkedUint248) -> _badPerformancePenalties_0[curveId_1]
TMP_3101(uint248) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint248(uint256), arguments:['penalty_1'] 
TMP_3102(ICSParametersRegistry.MarkedUint248) = new MarkedUint248(TMP_3101,True)
_badPerformancePenalties_1(mapping(uint256 => ICSParametersRegistry.MarkedUint248)) := phi(['_badPerformancePenalties_0'])
REF_1254(ICSParametersRegistry.MarkedUint248) (->_badPerformancePenalties_1) := TMP_3102(ICSParametersRegistry.MarkedUint248)
 BadPerformancePenaltySet(curveId,penalty)
Emit BadPerformancePenaltySet(curveId_1,penalty_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_65)
```
#### CSParametersRegistry.unsetBadPerformancePenalty(uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_67(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
_badPerformancePenalties_2(mapping(uint256 => ICSParametersRegistry.MarkedUint248)) := phi(['_badPerformancePenalties_0', '_badPerformancePenalties_4', '_badPerformancePenalties_1', '_badPerformancePenalties_5'])
 delete _badPerformancePenalties[curveId]
REF_1256(ICSParametersRegistry.MarkedUint248) -> _badPerformancePenalties_3[curveId_1]
_badPerformancePenalties_4 = delete REF_1256 
 BadPerformancePenaltyUnset(curveId)
Emit BadPerformancePenaltyUnset(curveId_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_67)
```
#### CSParametersRegistry.getBadPerformancePenalty(uint256) [EXTERNAL]
```slithir
defaultBadPerformancePenalty_1(uint256) := phi(['defaultBadPerformancePenalty_0', 'defaultBadPerformancePenalty_2'])
_badPerformancePenalties_5(mapping(uint256 => ICSParametersRegistry.MarkedUint248)) := phi(['_badPerformancePenalties_0', '_badPerformancePenalties_4', '_badPerformancePenalties_1', '_badPerformancePenalties_5'])
 data = _badPerformancePenalties[curveId]
REF_1295(ICSParametersRegistry.MarkedUint248) -> _badPerformancePenalties_5[curveId_1]
data_1 (-> ['_badPerformancePenalties'])(ICSParametersRegistry.MarkedUint248) := REF_1295(ICSParametersRegistry.MarkedUint248)
 data.isValue
REF_1296(bool) -> data_1 (-> ['_badPerformancePenalties']).isValue
CONDITION REF_1296
 data.value
REF_1297(uint248) -> data_1 (-> ['_badPerformancePenalties']).value
RETURN REF_1297
 defaultBadPerformancePenalty
RETURN defaultBadPerformancePenalty_1
 penalty
```
#### CSParametersRegistry.setPerformanceCoefficients(uint256,uint256,uint256,uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_69(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
 _validatePerformanceCoefficients(attestationsWeight,blocksWeight,syncWeight)
INTERNAL_CALL, CSParametersRegistry._validatePerformanceCoefficients(uint256,uint256,uint256)(attestationsWeight_1,blocksWeight_1,syncWeight_1)
 _performanceCoefficients[curveId] = PerformanceCoefficients(attestationsWeight.toUint32(),blocksWeight.toUint32(),syncWeight.toUint32())
REF_1257(ICSParametersRegistry.PerformanceCoefficients) -> _performanceCoefficients_0[curveId_1]
TMP_3108(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['attestationsWeight_1'] 
TMP_3109(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['blocksWeight_1'] 
TMP_3110(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['syncWeight_1'] 
TMP_3111(ICSParametersRegistry.PerformanceCoefficients) = new PerformanceCoefficients(TMP_3108,TMP_3109,TMP_3110)
_performanceCoefficients_1(mapping(uint256 => ICSParametersRegistry.PerformanceCoefficients)) := phi(['_performanceCoefficients_0'])
REF_1257(ICSParametersRegistry.PerformanceCoefficients) (->_performanceCoefficients_1) := TMP_3111(ICSParametersRegistry.PerformanceCoefficients)
 PerformanceCoefficientsSet(curveId,attestationsWeight,blocksWeight,syncWeight)
Emit PerformanceCoefficientsSet(curveId_1,attestationsWeight_1,blocksWeight_1,syncWeight_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_69)
```
#### CSParametersRegistry.unsetPerformanceCoefficients(uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_71(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
_performanceCoefficients_2(mapping(uint256 => ICSParametersRegistry.PerformanceCoefficients)) := phi(['_performanceCoefficients_1', '_performanceCoefficients_4', '_performanceCoefficients_5', '_performanceCoefficients_0'])
 delete _performanceCoefficients[curveId]
REF_1261(ICSParametersRegistry.PerformanceCoefficients) -> _performanceCoefficients_3[curveId_1]
_performanceCoefficients_4 = delete REF_1261 
 PerformanceCoefficientsUnset(curveId)
Emit PerformanceCoefficientsUnset(curveId_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_71)
```
#### CSParametersRegistry.getPerformanceCoefficients(uint256) [EXTERNAL]
```slithir
defaultPerformanceCoefficients_1(ICSParametersRegistry.PerformanceCoefficients) := phi(['defaultPerformanceCoefficients_1', 'defaultPerformanceCoefficients_2', 'defaultPerformanceCoefficients_0'])
_performanceCoefficients_5(mapping(uint256 => ICSParametersRegistry.PerformanceCoefficients)) := phi(['_performanceCoefficients_1', '_performanceCoefficients_4', '_performanceCoefficients_5', '_performanceCoefficients_0'])
 coefficients = _performanceCoefficients[curveId]
REF_1298(ICSParametersRegistry.PerformanceCoefficients) -> _performanceCoefficients_5[curveId_1]
coefficients_1 (-> ['_performanceCoefficients'])(ICSParametersRegistry.PerformanceCoefficients) := REF_1298(ICSParametersRegistry.PerformanceCoefficients)
 coefficients.attestationsWeight == 0 && coefficients.blocksWeight == 0 && coefficients.syncWeight == 0
REF_1299(uint32) -> coefficients_1 (-> ['_performanceCoefficients']).attestationsWeight
TMP_3150(bool) = REF_1299 == 0
REF_1300(uint32) -> coefficients_1 (-> ['_performanceCoefficients']).blocksWeight
TMP_3151(bool) = REF_1300 == 0
TMP_3152(bool) = TMP_3150 && TMP_3151
REF_1301(uint32) -> coefficients_1 (-> ['_performanceCoefficients']).syncWeight
TMP_3153(bool) = REF_1301 == 0
TMP_3154(bool) = TMP_3152 && TMP_3153
CONDITION TMP_3154
 (defaultPerformanceCoefficients.attestationsWeight,defaultPerformanceCoefficients.blocksWeight,defaultPerformanceCoefficients.syncWeight)
REF_1302(uint32) -> defaultPerformanceCoefficients_1.attestationsWeight
REF_1303(uint32) -> defaultPerformanceCoefficients_1.blocksWeight
REF_1304(uint32) -> defaultPerformanceCoefficients_1.syncWeight
RETURN REF_1302,REF_1303,REF_1304
 (coefficients.attestationsWeight,coefficients.blocksWeight,coefficients.syncWeight)
REF_1305(uint32) -> coefficients_1 (-> ['_performanceCoefficients']).attestationsWeight
REF_1306(uint32) -> coefficients_1 (-> ['_performanceCoefficients']).blocksWeight
REF_1307(uint32) -> coefficients_1 (-> ['_performanceCoefficients']).syncWeight
RETURN REF_1305,REF_1306,REF_1307
 (attestationsWeight,blocksWeight,syncWeight)
```
#### CSParametersRegistry.setAllowedExitDelay(uint256,uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_77(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
 _validateAllowedExitDelay(delay)
INTERNAL_CALL, CSParametersRegistry._validateAllowedExitDelay(uint256)(delay_1)
 _allowedExitDelay[curveId] = delay
REF_1266(uint256) -> _allowedExitDelay_0[curveId_1]
_allowedExitDelay_1(mapping(uint256 => uint256)) := phi(['_allowedExitDelay_0'])
REF_1266(uint256) (->_allowedExitDelay_1) := delay_1(uint256)
 AllowedExitDelaySet(curveId,delay)
Emit AllowedExitDelaySet(curveId_1,delay_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_77)
```
#### CSParametersRegistry.unsetAllowedExitDelay(uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_79(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
_allowedExitDelay_2(mapping(uint256 => uint256)) := phi(['_allowedExitDelay_1', '_allowedExitDelay_4', '_allowedExitDelay_0', '_allowedExitDelay_5'])
 delete _allowedExitDelay[curveId]
REF_1267(uint256) -> _allowedExitDelay_3[curveId_1]
_allowedExitDelay_4 = delete REF_1267 
 AllowedExitDelayUnset(curveId)
Emit AllowedExitDelayUnset(curveId_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_79)
```
#### CSParametersRegistry.getAllowedExitDelay(uint256) [EXTERNAL]
```slithir
defaultAllowedExitDelay_1(uint256) := phi(['defaultAllowedExitDelay_2', 'defaultAllowedExitDelay_0'])
_allowedExitDelay_5(mapping(uint256 => uint256)) := phi(['_allowedExitDelay_1', '_allowedExitDelay_4', '_allowedExitDelay_0', '_allowedExitDelay_5'])
 delay = _allowedExitDelay[curveId]
REF_1314(uint256) -> _allowedExitDelay_5[curveId_1]
delay_1(uint256) := REF_1314(uint256)
 delay == 0
TMP_3156(bool) = delay_1 == 0
CONDITION TMP_3156
 defaultAllowedExitDelay
RETURN defaultAllowedExitDelay_1
 delay
RETURN delay_1
```
#### CSParametersRegistry.setExitDelayPenalty(uint256,uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_81(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
 _exitDelayPenalties[curveId] = MarkedUint248(penalty.toUint248(),true)
REF_1268(ICSParametersRegistry.MarkedUint248) -> _exitDelayPenalties_0[curveId_1]
TMP_3129(uint248) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint248(uint256), arguments:['penalty_1'] 
TMP_3130(ICSParametersRegistry.MarkedUint248) = new MarkedUint248(TMP_3129,True)
_exitDelayPenalties_1(mapping(uint256 => ICSParametersRegistry.MarkedUint248)) := phi(['_exitDelayPenalties_0'])
REF_1268(ICSParametersRegistry.MarkedUint248) (->_exitDelayPenalties_1) := TMP_3130(ICSParametersRegistry.MarkedUint248)
 ExitDelayPenaltySet(curveId,penalty)
Emit ExitDelayPenaltySet(curveId_1,penalty_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_81)
```
#### CSParametersRegistry.unsetExitDelayPenalty(uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_83(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
_exitDelayPenalties_2(mapping(uint256 => ICSParametersRegistry.MarkedUint248)) := phi(['_exitDelayPenalties_5', '_exitDelayPenalties_0', '_exitDelayPenalties_1', '_exitDelayPenalties_4'])
 delete _exitDelayPenalties[curveId]
REF_1270(ICSParametersRegistry.MarkedUint248) -> _exitDelayPenalties_3[curveId_1]
_exitDelayPenalties_4 = delete REF_1270 
 ExitDelayPenaltyUnset(curveId)
Emit ExitDelayPenaltyUnset(curveId_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_83)
```
#### CSParametersRegistry.getExitDelayPenalty(uint256) [EXTERNAL]
```slithir
defaultExitDelayPenalty_1(uint256) := phi(['defaultExitDelayPenalty_2', 'defaultExitDelayPenalty_0'])
_exitDelayPenalties_5(mapping(uint256 => ICSParametersRegistry.MarkedUint248)) := phi(['_exitDelayPenalties_5', '_exitDelayPenalties_0', '_exitDelayPenalties_1', '_exitDelayPenalties_4'])
 data = _exitDelayPenalties[curveId]
REF_1315(ICSParametersRegistry.MarkedUint248) -> _exitDelayPenalties_5[curveId_1]
data_1(ICSParametersRegistry.MarkedUint248) := REF_1315(ICSParametersRegistry.MarkedUint248)
 data.isValue
REF_1316(bool) -> data_1.isValue
CONDITION REF_1316
 data.value
REF_1317(uint248) -> data_1.value
RETURN REF_1317
 defaultExitDelayPenalty
RETURN defaultExitDelayPenalty_1
 penalty
```
#### CSParametersRegistry.setMaxWithdrawalRequestFee(uint256,uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_85(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
 _maxWithdrawalRequestFees[curveId] = MarkedUint248(fee.toUint248(),true)
REF_1271(ICSParametersRegistry.MarkedUint248) -> _maxWithdrawalRequestFees_0[curveId_1]
TMP_3135(uint248) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint248(uint256), arguments:['fee_1'] 
TMP_3136(ICSParametersRegistry.MarkedUint248) = new MarkedUint248(TMP_3135,True)
_maxWithdrawalRequestFees_1(mapping(uint256 => ICSParametersRegistry.MarkedUint248)) := phi(['_maxWithdrawalRequestFees_0'])
REF_1271(ICSParametersRegistry.MarkedUint248) (->_maxWithdrawalRequestFees_1) := TMP_3136(ICSParametersRegistry.MarkedUint248)
 MaxWithdrawalRequestFeeSet(curveId,fee)
Emit MaxWithdrawalRequestFeeSet(curveId_1,fee_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_85)
```
#### CSParametersRegistry.unsetMaxWithdrawalRequestFee(uint256) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_87(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
_maxWithdrawalRequestFees_2(mapping(uint256 => ICSParametersRegistry.MarkedUint248)) := phi(['_maxWithdrawalRequestFees_0', '_maxWithdrawalRequestFees_4', '_maxWithdrawalRequestFees_5', '_maxWithdrawalRequestFees_1'])
 delete _maxWithdrawalRequestFees[curveId]
REF_1273(ICSParametersRegistry.MarkedUint248) -> _maxWithdrawalRequestFees_3[curveId_1]
_maxWithdrawalRequestFees_4 = delete REF_1273 
 MaxWithdrawalRequestFeeUnset(curveId)
Emit MaxWithdrawalRequestFeeUnset(curveId_1)
 onlyRole(DEFAULT_ADMIN_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(DEFAULT_ADMIN_ROLE_87)
```
#### CSParametersRegistry.getMaxWithdrawalRequestFee(uint256) [EXTERNAL]
```slithir
defaultMaxWithdrawalRequestFee_1(uint256) := phi(['defaultMaxWithdrawalRequestFee_2', 'defaultMaxWithdrawalRequestFee_0'])
_maxWithdrawalRequestFees_5(mapping(uint256 => ICSParametersRegistry.MarkedUint248)) := phi(['_maxWithdrawalRequestFees_0', '_maxWithdrawalRequestFees_4', '_maxWithdrawalRequestFees_5', '_maxWithdrawalRequestFees_1'])
 data = _maxWithdrawalRequestFees[curveId]
REF_1318(ICSParametersRegistry.MarkedUint248) -> _maxWithdrawalRequestFees_5[curveId_1]
data_1(ICSParametersRegistry.MarkedUint248) := REF_1318(ICSParametersRegistry.MarkedUint248)
 data.isValue
REF_1319(bool) -> data_1.isValue
CONDITION REF_1319
 data.value
REF_1320(uint248) -> data_1.value
RETURN REF_1320
 defaultMaxWithdrawalRequestFee
RETURN defaultMaxWithdrawalRequestFee_1
 fee
```
#### CSParametersRegistry.getInitializedVersion() [EXTERNAL]
```slithir
 _getInitializedVersion()
TMP_3157(uint64) = INTERNAL_CALL, Initializable._getInitializedVersion()()
RETURN TMP_3157
```
#### CSParametersRegistry.constructor(uint256) [PUBLIC]
```slithir
 queueLowestPriority == 0
TMP_3009(bool) = queueLowestPriority_1 == 0
CONDITION TMP_3009
 revert ZeroQueueLowestPriority()()
TMP_3010(None) = SOLIDITY_CALL revert ZeroQueueLowestPriority()()
 QUEUE_LOWEST_PRIORITY = queueLowestPriority
QUEUE_LOWEST_PRIORITY_1(uint256) := queueLowestPriority_1(uint256)
 QUEUE_LEGACY_PRIORITY = queueLowestPriority - 1
TMP_3011(uint256) = queueLowestPriority_1 (c)- 1
QUEUE_LEGACY_PRIORITY_1(uint256) := TMP_3011(uint256)
 _disableInitializers()
INTERNAL_CALL, Initializable._disableInitializers()()
```
#### CSParametersRegistry.initialize(address,ICSParametersRegistry.InitializationData) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_28', 'DEFAULT_ADMIN_ROLE_20', 'DEFAULT_ADMIN_ROLE_68', 'DEFAULT_ADMIN_ROLE_16', 'DEFAULT_ADMIN_ROLE_36', 'DEFAULT_ADMIN_ROLE_64', 'DEFAULT_ADMIN_ROLE_60', 'DEFAULT_ADMIN_ROLE_52', 'DEFAULT_ADMIN_ROLE_30', 'DEFAULT_ADMIN_ROLE_88', 'DEFAULT_ADMIN_ROLE_48', 'DEFAULT_ADMIN_ROLE_58', 'DEFAULT_ADMIN_ROLE_84', 'DEFAULT_ADMIN_ROLE_22', 'DEFAULT_ADMIN_ROLE_44', 'DEFAULT_ADMIN_ROLE_80', 'DEFAULT_ADMIN_ROLE_38', 'DEFAULT_ADMIN_ROLE_72', 'DEFAULT_ADMIN_ROLE_18', 'DEFAULT_ADMIN_ROLE_76', 'DEFAULT_ADMIN_ROLE_24', 'DEFAULT_ADMIN_ROLE_40', 'DEFAULT_ADMIN_ROLE_62', 'DEFAULT_ADMIN_ROLE_32', 'DEFAULT_ADMIN_ROLE_66', 'DEFAULT_ADMIN_ROLE_70', 'DEFAULT_ADMIN_ROLE_50', 'DEFAULT_ADMIN_ROLE_26', 'DEFAULT_ADMIN_ROLE_86', 'DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_46', 'DEFAULT_ADMIN_ROLE_82', 'DEFAULT_ADMIN_ROLE_56', 'DEFAULT_ADMIN_ROLE_42', 'DEFAULT_ADMIN_ROLE_74', 'DEFAULT_ADMIN_ROLE_34', 'DEFAULT_ADMIN_ROLE_54', 'DEFAULT_ADMIN_ROLE_78'])
 admin == address(0)
TMP_3013 = CONVERT 0 to address
TMP_3014(bool) = admin_1 == TMP_3013
CONDITION TMP_3014
 revert ZeroAdminAddress()()
TMP_3015(None) = SOLIDITY_CALL revert ZeroAdminAddress()()
 _setDefaultKeyRemovalCharge(data.keyRemovalCharge)
REF_1207(uint256) -> data_1.keyRemovalCharge
INTERNAL_CALL, CSParametersRegistry._setDefaultKeyRemovalCharge(uint256)(REF_1207)
 _setDefaultElRewardsStealingAdditionalFine(data.elRewardsStealingAdditionalFine)
REF_1208(uint256) -> data_1.elRewardsStealingAdditionalFine
INTERNAL_CALL, CSParametersRegistry._setDefaultElRewardsStealingAdditionalFine(uint256)(REF_1208)
 _setDefaultKeysLimit(data.keysLimit)
REF_1209(uint256) -> data_1.keysLimit
INTERNAL_CALL, CSParametersRegistry._setDefaultKeysLimit(uint256)(REF_1209)
 _setDefaultRewardShare(data.rewardShare)
REF_1210(uint256) -> data_1.rewardShare
INTERNAL_CALL, CSParametersRegistry._setDefaultRewardShare(uint256)(REF_1210)
 _setDefaultPerformanceLeeway(data.performanceLeeway)
REF_1211(uint256) -> data_1.performanceLeeway
INTERNAL_CALL, CSParametersRegistry._setDefaultPerformanceLeeway(uint256)(REF_1211)
 _setDefaultStrikesParams(data.strikesLifetime,data.strikesThreshold)
REF_1212(uint256) -> data_1.strikesLifetime
REF_1213(uint256) -> data_1.strikesThreshold
INTERNAL_CALL, CSParametersRegistry._setDefaultStrikesParams(uint256,uint256)(REF_1212,REF_1213)
 _setDefaultBadPerformancePenalty(data.badPerformancePenalty)
REF_1214(uint256) -> data_1.badPerformancePenalty
INTERNAL_CALL, CSParametersRegistry._setDefaultBadPerformancePenalty(uint256)(REF_1214)
 _setDefaultPerformanceCoefficients(data.attestationsWeight,data.blocksWeight,data.syncWeight)
REF_1215(uint256) -> data_1.attestationsWeight
REF_1216(uint256) -> data_1.blocksWeight
REF_1217(uint256) -> data_1.syncWeight
INTERNAL_CALL, CSParametersRegistry._setDefaultPerformanceCoefficients(uint256,uint256,uint256)(REF_1215,REF_1216,REF_1217)
 _setDefaultQueueConfig(data.defaultQueuePriority,data.defaultQueueMaxDeposits)
REF_1218(uint256) -> data_1.defaultQueuePriority
REF_1219(uint256) -> data_1.defaultQueueMaxDeposits
INTERNAL_CALL, CSParametersRegistry._setDefaultQueueConfig(uint256,uint256)(REF_1218,REF_1219)
 _setDefaultAllowedExitDelay(data.defaultAllowedExitDelay)
REF_1220(uint256) -> data_1.defaultAllowedExitDelay
INTERNAL_CALL, CSParametersRegistry._setDefaultAllowedExitDelay(uint256)(REF_1220)
 _setDefaultExitDelayPenalty(data.defaultExitDelayPenalty)
REF_1221(uint256) -> data_1.defaultExitDelayPenalty
INTERNAL_CALL, CSParametersRegistry._setDefaultExitDelayPenalty(uint256)(REF_1221)
 _setDefaultMaxWithdrawalRequestFee(data.defaultMaxWithdrawalRequestFee)
REF_1222(uint256) -> data_1.defaultMaxWithdrawalRequestFee
INTERNAL_CALL, CSParametersRegistry._setDefaultMaxWithdrawalRequestFee(uint256)(REF_1222)
 __AccessControlEnumerable_init()
INTERNAL_CALL, AccessControlEnumerableUpgradeable.__AccessControlEnumerable_init()()
 _grantRole(DEFAULT_ADMIN_ROLE,admin)
TMP_3029(bool) = INTERNAL_CALL, AccessControlEnumerableUpgradeable._grantRole(bytes32,address)(DEFAULT_ADMIN_ROLE_15,admin_1)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### CSParametersRegistry._setDefaultKeyRemovalCharge(uint256) [INTERNAL]
```slithir
keyRemovalCharge_1(uint256) := phi(['keyRemovalCharge_1', 'REF_1207'])
 defaultKeyRemovalCharge = keyRemovalCharge
defaultKeyRemovalCharge_2(uint256) := keyRemovalCharge_1(uint256)
 DefaultKeyRemovalChargeSet(keyRemovalCharge)
Emit DefaultKeyRemovalChargeSet(keyRemovalCharge_1)
```
#### CSParametersRegistry._setDefaultElRewardsStealingAdditionalFine(uint256) [INTERNAL]
```slithir
fine_1(uint256) := phi(['REF_1208', 'fine_1'])
 defaultElRewardsStealingAdditionalFine = fine
defaultElRewardsStealingAdditionalFine_2(uint256) := fine_1(uint256)
 DefaultElRewardsStealingAdditionalFineSet(fine)
Emit DefaultElRewardsStealingAdditionalFineSet(fine_1)
```
#### CSParametersRegistry._setDefaultKeysLimit(uint256) [INTERNAL]
```slithir
limit_1(uint256) := phi(['REF_1209', 'limit_1'])
 defaultKeysLimit = limit
defaultKeysLimit_2(uint256) := limit_1(uint256)
 DefaultKeysLimitSet(limit)
Emit DefaultKeysLimitSet(limit_1)
```
#### CSParametersRegistry._setDefaultRewardShare(uint256) [INTERNAL]
```slithir
share_1(uint256) := phi(['share_1', 'REF_1210'])
MAX_BP_1(uint256) := phi(['MAX_BP_0'])
 share > MAX_BP
TMP_3161(bool) = share_1 > MAX_BP_1
CONDITION TMP_3161
 revert InvalidRewardShareData()()
TMP_3162(None) = SOLIDITY_CALL revert InvalidRewardShareData()()
 defaultRewardShare = share
defaultRewardShare_2(uint256) := share_1(uint256)
 DefaultRewardShareSet(share)
Emit DefaultRewardShareSet(share_1)
```
#### CSParametersRegistry._setDefaultPerformanceLeeway(uint256) [INTERNAL]
```slithir
leeway_1(uint256) := phi(['REF_1211', 'leeway_1'])
MAX_BP_2(uint256) := phi(['MAX_BP_0'])
 leeway > MAX_BP
TMP_3164(bool) = leeway_1 > MAX_BP_2
CONDITION TMP_3164
 revert InvalidPerformanceLeewayData()()
TMP_3165(None) = SOLIDITY_CALL revert InvalidPerformanceLeewayData()()
 defaultPerformanceLeeway = leeway
defaultPerformanceLeeway_2(uint256) := leeway_1(uint256)
 DefaultPerformanceLeewaySet(leeway)
Emit DefaultPerformanceLeewaySet(leeway_1)
```
#### CSParametersRegistry._setDefaultStrikesParams(uint256,uint256) [INTERNAL]
```slithir
lifetime_1(uint256) := phi(['lifetime_1', 'REF_1212'])
threshold_1(uint256) := phi(['threshold_1', 'REF_1213'])
 _validateStrikesParams(lifetime,threshold)
INTERNAL_CALL, CSParametersRegistry._validateStrikesParams(uint256,uint256)(lifetime_1,threshold_1)
 defaultStrikesParams = StrikesParams({lifetime:lifetime.toUint32(),threshold:threshold.toUint32()})
TMP_3168(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['lifetime_1'] 
TMP_3169(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['threshold_1'] 
TMP_3170(ICSParametersRegistry.StrikesParams) = new StrikesParams(TMP_3168,TMP_3169)
defaultStrikesParams_2(ICSParametersRegistry.StrikesParams) := TMP_3170(ICSParametersRegistry.StrikesParams)
 DefaultStrikesParamsSet(lifetime,threshold)
Emit DefaultStrikesParamsSet(lifetime_1,threshold_1)
```
#### CSParametersRegistry._setDefaultBadPerformancePenalty(uint256) [INTERNAL]
```slithir
penalty_1(uint256) := phi(['REF_1214', 'penalty_1'])
 defaultBadPerformancePenalty = penalty
defaultBadPerformancePenalty_2(uint256) := penalty_1(uint256)
 DefaultBadPerformancePenaltySet(penalty)
Emit DefaultBadPerformancePenaltySet(penalty_1)
```
#### CSParametersRegistry._setDefaultPerformanceCoefficients(uint256,uint256,uint256) [INTERNAL]
```slithir
attestationsWeight_1(uint256) := phi(['attestationsWeight_1', 'REF_1215'])
blocksWeight_1(uint256) := phi(['REF_1216', 'blocksWeight_1'])
syncWeight_1(uint256) := phi(['REF_1217', 'syncWeight_1'])
 _validatePerformanceCoefficients(attestationsWeight,blocksWeight,syncWeight)
INTERNAL_CALL, CSParametersRegistry._validatePerformanceCoefficients(uint256,uint256,uint256)(attestationsWeight_1,blocksWeight_1,syncWeight_1)
 defaultPerformanceCoefficients = PerformanceCoefficients({attestationsWeight:attestationsWeight.toUint32(),blocksWeight:blocksWeight.toUint32(),syncWeight:syncWeight.toUint32()})
TMP_3174(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['attestationsWeight_1'] 
TMP_3175(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['blocksWeight_1'] 
TMP_3176(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['syncWeight_1'] 
TMP_3177(ICSParametersRegistry.PerformanceCoefficients) = new PerformanceCoefficients(TMP_3174,TMP_3175,TMP_3176)
defaultPerformanceCoefficients_2(ICSParametersRegistry.PerformanceCoefficients) := TMP_3177(ICSParametersRegistry.PerformanceCoefficients)
 DefaultPerformanceCoefficientsSet(attestationsWeight,blocksWeight,syncWeight)
Emit DefaultPerformanceCoefficientsSet(attestationsWeight_1,blocksWeight_1,syncWeight_1)
```
#### CSParametersRegistry._setDefaultQueueConfig(uint256,uint256) [INTERNAL]
```slithir
priority_1(uint256) := phi(['REF_1218', 'priority_1'])
maxDeposits_1(uint256) := phi(['REF_1219', 'maxDeposits_1'])
 _validateQueueConfig(priority,maxDeposits)
INTERNAL_CALL, CSParametersRegistry._validateQueueConfig(uint256,uint256)(priority_1,maxDeposits_1)
 defaultQueueConfig = QueueConfig({priority:priority.toUint32(),maxDeposits:maxDeposits.toUint32()})
TMP_3180(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['priority_1'] 
TMP_3181(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['maxDeposits_1'] 
TMP_3182(ICSParametersRegistry.QueueConfig) = new QueueConfig(TMP_3180,TMP_3181)
defaultQueueConfig_2(ICSParametersRegistry.QueueConfig) := TMP_3182(ICSParametersRegistry.QueueConfig)
 DefaultQueueConfigSet(priority,maxDeposits)
Emit DefaultQueueConfigSet(priority_1,maxDeposits_1)
```
#### CSParametersRegistry._setDefaultAllowedExitDelay(uint256) [INTERNAL]
```slithir
delay_1(uint256) := phi(['REF_1220', 'delay_1'])
 _validateAllowedExitDelay(delay)
INTERNAL_CALL, CSParametersRegistry._validateAllowedExitDelay(uint256)(delay_1)
 defaultAllowedExitDelay = delay
defaultAllowedExitDelay_2(uint256) := delay_1(uint256)
 DefaultAllowedExitDelaySet(delay)
Emit DefaultAllowedExitDelaySet(delay_1)
```
#### CSParametersRegistry._setDefaultExitDelayPenalty(uint256) [INTERNAL]
```slithir
penalty_1(uint256) := phi(['penalty_1', 'REF_1221'])
 defaultExitDelayPenalty = penalty
defaultExitDelayPenalty_2(uint256) := penalty_1(uint256)
 DefaultExitDelayPenaltySet(penalty)
Emit DefaultExitDelayPenaltySet(penalty_1)
```
#### CSParametersRegistry._setDefaultMaxWithdrawalRequestFee(uint256) [INTERNAL]
```slithir
fee_1(uint256) := phi(['fee_1', 'REF_1222'])
 defaultMaxWithdrawalRequestFee = fee
defaultMaxWithdrawalRequestFee_2(uint256) := fee_1(uint256)
 DefaultMaxWithdrawalRequestFeeSet(fee)
Emit DefaultMaxWithdrawalRequestFeeSet(fee_1)
```
#### CSParametersRegistry._validateQueueConfig(uint256,uint256) [INTERNAL]
```slithir
priority_1(uint256) := phi(['priority_1', 'priority_1'])
maxDeposits_1(uint256) := phi(['maxDeposits_1', 'maxDeposits_1'])
QUEUE_LOWEST_PRIORITY_2(uint256) := phi(['QUEUE_LOWEST_PRIORITY_0', 'QUEUE_LOWEST_PRIORITY_1'])
QUEUE_LEGACY_PRIORITY_2(uint256) := phi(['QUEUE_LEGACY_PRIORITY_0', 'QUEUE_LEGACY_PRIORITY_1'])
 priority > QUEUE_LOWEST_PRIORITY || priority == QUEUE_LEGACY_PRIORITY
TMP_3188(bool) = priority_1 > QUEUE_LOWEST_PRIORITY_2
TMP_3189(bool) = priority_1 == QUEUE_LEGACY_PRIORITY_2
TMP_3190(bool) = TMP_3188 || TMP_3189
CONDITION TMP_3190
 revert QueueCannotBeUsed()()
TMP_3191(None) = SOLIDITY_CALL revert QueueCannotBeUsed()()
 maxDeposits == 0
TMP_3192(bool) = maxDeposits_1 == 0
CONDITION TMP_3192
 revert ZeroMaxDeposits()()
TMP_3193(None) = SOLIDITY_CALL revert ZeroMaxDeposits()()
```
#### CSParametersRegistry._validateStrikesParams(uint256,uint256) [INTERNAL]
```slithir
lifetime_1(uint256) := phi(['lifetime_1', 'lifetime_1'])
threshold_1(uint256) := phi(['threshold_1', 'threshold_1'])
 threshold == 0 || lifetime == 0
TMP_3194(bool) = threshold_1 == 0
TMP_3195(bool) = lifetime_1 == 0
TMP_3196(bool) = TMP_3194 || TMP_3195
CONDITION TMP_3196
 revert InvalidStrikesParams()()
TMP_3197(None) = SOLIDITY_CALL revert InvalidStrikesParams()()
```
#### CSParametersRegistry._validateAllowedExitDelay(uint256) [INTERNAL]
```slithir
delay_1(uint256) := phi(['delay_1', 'delay_1'])
 delay == 0
TMP_3198(bool) = delay_1 == 0
CONDITION TMP_3198
 revert InvalidAllowedExitDelay()()
TMP_3199(None) = SOLIDITY_CALL revert InvalidAllowedExitDelay()()
```
#### CSParametersRegistry._validatePerformanceCoefficients(uint256,uint256,uint256) [INTERNAL]
```slithir
attestationsWeight_1(uint256) := phi(['attestationsWeight_1', 'attestationsWeight_1'])
blocksWeight_1(uint256) := phi(['blocksWeight_1', 'blocksWeight_1'])
syncWeight_1(uint256) := phi(['syncWeight_1', 'syncWeight_1'])
 attestationsWeight == 0 && blocksWeight == 0 && syncWeight == 0
TMP_3200(bool) = attestationsWeight_1 == 0
TMP_3201(bool) = blocksWeight_1 == 0
TMP_3202(bool) = TMP_3200 && TMP_3201
TMP_3203(bool) = syncWeight_1 == 0
TMP_3204(bool) = TMP_3202 && TMP_3203
CONDITION TMP_3204
 revert InvalidPerformanceCoefficients()()
TMP_3205(None) = SOLIDITY_CALL revert InvalidPerformanceCoefficients()()
```
#### CSParametersRegistry._validateKeyNumberValueIntervals(ICSParametersRegistry.KeyNumberValueInterval[]) [PRIVATE]
```slithir
intervals_1(ICSParametersRegistry.KeyNumberValueInterval[]) := phi(['data_1', 'data_1'])
MAX_BP_3(uint256) := phi(['MAX_BP_0'])
 intervals.length == 0
REF_1328 -> LENGTH intervals_1
TMP_3206(bool) = REF_1328 == 0
CONDITION TMP_3206
 revert InvalidKeyNumberValueIntervals()()
TMP_3207(None) = SOLIDITY_CALL revert InvalidKeyNumberValueIntervals()()
 intervals[0].minKeyNumber != 1
REF_1329(ICSParametersRegistry.KeyNumberValueInterval) -> intervals_1[0]
REF_1330(uint256) -> REF_1329.minKeyNumber
TMP_3208(bool) = REF_1330 != 1
CONDITION TMP_3208
 revert InvalidKeyNumberValueIntervals()()
TMP_3209(None) = SOLIDITY_CALL revert InvalidKeyNumberValueIntervals()()
 intervals[0].value > MAX_BP
REF_1331(ICSParametersRegistry.KeyNumberValueInterval) -> intervals_1[0]
REF_1332(uint256) -> REF_1331.value
TMP_3210(bool) = REF_1332 > MAX_BP_3
CONDITION TMP_3210
 revert InvalidKeyNumberValueIntervals()()
TMP_3211(None) = SOLIDITY_CALL revert InvalidKeyNumberValueIntervals()()
 i = 1
i_1(uint256) := 1(uint256)
 i < intervals.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_1333 -> LENGTH intervals_1
TMP_3212(bool) = i_2 < REF_1333
CONDITION TMP_3212
 intervals[i].minKeyNumber <= intervals[i - 1].minKeyNumber
REF_1334(ICSParametersRegistry.KeyNumberValueInterval) -> intervals_1[i_2]
REF_1335(uint256) -> REF_1334.minKeyNumber
TMP_3213(uint256) = i_2 - 1
REF_1336(ICSParametersRegistry.KeyNumberValueInterval) -> intervals_1[TMP_3213]
REF_1337(uint256) -> REF_1336.minKeyNumber
TMP_3214(bool) = REF_1335 <= REF_1337
CONDITION TMP_3214
 revert InvalidKeyNumberValueIntervals()()
TMP_3215(None) = SOLIDITY_CALL revert InvalidKeyNumberValueIntervals()()
 intervals[i].value > MAX_BP
REF_1338(ICSParametersRegistry.KeyNumberValueInterval) -> intervals_1[i_2]
REF_1339(uint256) -> REF_1338.value
TMP_3216(bool) = REF_1339 > MAX_BP_3
CONDITION TMP_3216
 revert InvalidKeyNumberValueIntervals()()
TMP_3217(None) = SOLIDITY_CALL revert InvalidKeyNumberValueIntervals()()
 ++ i
i_3(uint256) = i_2 (c)+ 1
```
#### CSModule.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 DEFAULT_ADMIN_ROLE = 0x00
 RESUME_SINCE_TIMESTAMP_POSITION = keccak256(bytes)(lido.PausableUntil.resumeSinceTimestamp)
 PAUSE_INFINITELY = type()(uint256).max
 PAUSE_ROLE = keccak256(bytes)(PAUSE_ROLE)
 RESUME_ROLE = keccak256(bytes)(RESUME_ROLE)
 STAKING_ROUTER_ROLE = keccak256(bytes)(STAKING_ROUTER_ROLE)
 REPORT_EL_REWARDS_STEALING_PENALTY_ROLE = keccak256(bytes)(REPORT_EL_REWARDS_STEALING_PENALTY_ROLE)
 SETTLE_EL_REWARDS_STEALING_PENALTY_ROLE = keccak256(bytes)(SETTLE_EL_REWARDS_STEALING_PENALTY_ROLE)
 VERIFIER_ROLE = keccak256(bytes)(VERIFIER_ROLE)
 RECOVERER_ROLE = keccak256(bytes)(RECOVERER_ROLE)
 CREATE_NODE_OPERATOR_ROLE = keccak256(bytes)(CREATE_NODE_OPERATOR_ROLE)
 DEPOSIT_SIZE = 32000000000000000000
 FORCED_TARGET_LIMIT_MODE_ID = 2
 OPERATORS_CREATED_IN_TX_MAP_TSLOT = 0x1b07bc0838fdc4254cbabb5dd0c94d936f872c6758547168d513d8ad1dc3a500
 _checkPaused()
INTERNAL_CALL, PausableUntil._checkPaused()()
 _checkResumed()
INTERNAL_CALL, PausableUntil._checkResumed()()
role_1(bytes32) := phi(['CREATE_NODE_OPERATOR_ROLE_1', 'REPORT_EL_REWARDS_STEALING_PENALTY_ROLE_1', 'STAKING_ROUTER_ROLE_21', 'STAKING_ROUTER_ROLE_13', 'STAKING_ROUTER_ROLE_17', 'REPORT_EL_REWARDS_STEALING_PENALTY_ROLE_3', 'STAKING_ROUTER_ROLE_23', 'STAKING_ROUTER_ROLE_15', 'TMP_2452', 'STAKING_ROUTER_ROLE_19', 'TMP_2454', 'RESUME_ROLE_1', 'SETTLE_EL_REWARDS_STEALING_PENALTY_ROLE_1', 'STAKING_ROUTER_ROLE_7', 'TMP_2457', 'PAUSE_ROLE_1', 'VERIFIER_ROLE_1', 'TMP_2459', 'STAKING_ROUTER_ROLE_9', 'STAKING_ROUTER_ROLE_11'])
 _checkRole(role)
INTERNAL_CALL, AccessControlUpgradeable._checkRole(bytes32)(role_1)
 $ = _getInitializableStorage()
TMP_2913(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_2913'])(Initializable.InitializableStorage) := TMP_2913(Initializable.InitializableStorage)
 isTopLevelCall = ! $._initializing
REF_1158(bool) -> $_1 (-> ['TMP_2913'])._initializing
TMP_2914 = UnaryType.BANG REF_1158 
isTopLevelCall_1(bool) := TMP_2914(bool)
 initialized = $._initialized
REF_1159(uint64) -> $_1 (-> ['TMP_2913'])._initialized
initialized_1(uint64) := REF_1159(uint64)
 initialSetup = initialized == 0 && isTopLevelCall
TMP_2915(bool) = initialized_1 == 0
TMP_2916(bool) = TMP_2915 && isTopLevelCall_1
initialSetup_1(bool) := TMP_2916(bool)
 construction = initialized == 1 && address(this).code.length == 0
TMP_2917(bool) = initialized_1 == 1
TMP_2918 = CONVERT this to address
TMP_2919(bytes) = SOLIDITY_CALL code(address)(TMP_2918)
REF_1160 -> LENGTH TMP_2919
TMP_2920(bool) = REF_1160 == 0
TMP_2921(bool) = TMP_2917 && TMP_2920
construction_1(bool) := TMP_2921(bool)
 ! initialSetup && ! construction
TMP_2922 = UnaryType.BANG initialSetup_1 
TMP_2923 = UnaryType.BANG construction_1 
TMP_2924(bool) = TMP_2922 && TMP_2923
CONDITION TMP_2924
 revert InvalidInitialization()()
TMP_2925(None) = SOLIDITY_CALL revert InvalidInitialization()()
 $._initialized = 1
REF_1161(uint64) -> $_1 (-> ['TMP_2913'])._initialized
$_2 (-> ['TMP_2913'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_2913'])"])
REF_1161(uint64) (->$_2 (-> ['TMP_2913'])) := 1(uint256)
TMP_2913(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_2913'])"])
 isTopLevelCall
CONDITION isTopLevelCall_1
 $._initializing = true
REF_1162(bool) -> $_2 (-> ['TMP_2913'])._initializing
$_3 (-> ['TMP_2913'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_2913'])"])
REF_1162(bool) (->$_3 (-> ['TMP_2913'])) := True(bool)
TMP_2913(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_2913'])"])
$_4 (-> ['TMP_2913'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_2913'])", "$_2 (-> ['TMP_2913'])"])
 isTopLevelCall
CONDITION isTopLevelCall_1
 $._initializing = false
REF_1163(bool) -> $_4 (-> ['TMP_2913'])._initializing
$_5 (-> ['TMP_2913'])(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_2913'])"])
REF_1163(bool) (->$_5 (-> ['TMP_2913'])) := False(bool)
TMP_2913(Initializable.InitializableStorage) := phi(["$_5 (-> ['TMP_2913'])"])
 Initialized(1)
Emit Initialized(1)
 $ = _getInitializableStorage()
TMP_2927(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_2927'])(Initializable.InitializableStorage) := TMP_2927(Initializable.InitializableStorage)
 $._initializing || $._initialized >= version
REF_1164(bool) -> $_1 (-> ['TMP_2927'])._initializing
REF_1165(uint64) -> $_1 (-> ['TMP_2927'])._initialized
TMP_2928(bool) = REF_1165 >= version_1
TMP_2929(bool) = REF_1164 || TMP_2928
CONDITION TMP_2929
 revert InvalidInitialization()()
TMP_2930(None) = SOLIDITY_CALL revert InvalidInitialization()()
 $._initialized = version
REF_1166(uint64) -> $_1 (-> ['TMP_2927'])._initialized
$_2 (-> ['TMP_2927'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_2927'])"])
REF_1166(uint64) (->$_2 (-> ['TMP_2927'])) := version_1(uint64)
TMP_2927(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_2927'])"])
 $._initializing = true
REF_1167(bool) -> $_2 (-> ['TMP_2927'])._initializing
$_3 (-> ['TMP_2927'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_2927'])"])
REF_1167(bool) (->$_3 (-> ['TMP_2927'])) := True(bool)
TMP_2927(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_2927'])"])
 $._initializing = false
REF_1168(bool) -> $_3 (-> ['TMP_2927'])._initializing
$_4 (-> ['TMP_2927'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_2927'])"])
REF_1168(bool) (->$_4 (-> ['TMP_2927'])) := False(bool)
TMP_2927(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_2927'])"])
 Initialized(version)
Emit Initialized(version_1)
 _checkInitializing()
INTERNAL_CALL, Initializable._checkInitializing()()
```
#### SafeCast.toUint248(uint256) [INTERNAL]
```slithir
 value > type()(uint248).max
TMP_453(uint248) := 452312848583266388373324160190187140051835877600158453279131187530910662655(uint248)
TMP_454(bool) = value_1 > TMP_453
CONDITION TMP_454
 revert SafeCastOverflowedUintDowncast(uint8,uint256)(248,value)
TMP_455(None) = SOLIDITY_CALL revert SafeCastOverflowedUintDowncast(uint8,uint256)(248,value_1)
 uint248(value)
TMP_456 = CONVERT value_1 to uint248
RETURN TMP_456
```
#### SafeCast.toUint32(uint256) [INTERNAL]
```slithir
 value > type()(uint32).max
TMP_588(uint32) := 4294967295(uint32)
TMP_589(bool) = value_1 > TMP_588
CONDITION TMP_589
 revert SafeCastOverflowedUintDowncast(uint8,uint256)(32,value)
TMP_590(None) = SOLIDITY_CALL revert SafeCastOverflowedUintDowncast(uint8,uint256)(32,value_1)
 uint32(value)
TMP_591 = CONVERT value_1 to uint32
RETURN TMP_591
```
