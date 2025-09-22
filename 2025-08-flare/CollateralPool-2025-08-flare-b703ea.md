 NOTE : lots of interfaces missing , particularly II



### Storage layout (CollateralPool) 

```text
agentVault address
assetManager IIAssetManager
fAsset IFAsset
token IICollateralPoolToken
wNat IWNat
exitCollateralRatioBIPS uint32
__topupCollateralRatioBIPS uint32
__topupTokenPriceFactorBIPS uint16
internalWithdrawal bool
initialized bool
_fAssetFeeDebtOf mapping(address => int256)
totalFAssetFeeDebt int256
totalFAssetFees uint256
totalCollateral uint256

```








#### CollateralPool._agentBackedFAssets() [INTERNAL]
```slithir
agentVault_25(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_1', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_0', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_22(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_14', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_0', 'assetManager_30'])
 assetManager.getFAssetsBackedByPool(agentVault)
TMP_6252(uint256) = HIGH_LEVEL_CALL, dest:assetManager_22(IIAssetManager), function:getFAssetsBackedByPool, arguments:['agentVault_25']  
agentVault_26(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_25', 'agentVault_20', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_23(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_22', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
RETURN TMP_6252
```
#### CollateralPool._authorizeUpgrade(address) [INTERNAL]
```slithir
 onlyAssetManager()
MODIFIER_CALL, CollateralPool.onlyAssetManager()()
```
#### CollateralPool._collateralToTokenShare(uint256) [INTERNAL]
```slithir
_collateral_1(uint256) := phi(['msg.value'])
token_48(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_0', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
totalCollateral_19(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_0', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
 totalPoolTokens = token.totalSupply()
TMP_6219(uint256) = HIGH_LEVEL_CALL, dest:token_48(IICollateralPoolToken), function:totalSupply, arguments:[]  
token_49(IICollateralPoolToken) := phi(['token_13', 'token_48', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
totalCollateral_20(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_19', 'totalCollateral_17', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
totalPoolTokens_1(uint256) := TMP_6219(uint256)
 totalCollateral == 0 || totalPoolTokens == 0
TMP_6220(bool) = totalCollateral_20 == 0
TMP_6221(bool) = totalPoolTokens_1 == 0
TMP_6222(bool) = TMP_6220 || TMP_6221
CONDITION TMP_6222
 _collateral
RETURN _collateral_1
 totalPoolTokens.mulDiv(_collateral,totalCollateral)
TMP_6223(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['totalPoolTokens_1', '_collateral_1', 'totalCollateral_20'] 
RETURN TMP_6223
```
#### CollateralPool._createFAssetFeeDebt(address,uint256) [INTERNAL]
```slithir
_account_1(address) := phi(['msg.sender'])
_fAssets_1(uint256) := phi(['feeDebt_3', '_fAssets_1'])
_fAssetFeeDebtOf_6(mapping(address => int256)) := phi(['_fAssetFeeDebtOf_5', '_fAssetFeeDebtOf_0', '_fAssetFeeDebtOf_2', '_fAssetFeeDebtOf_7', '_fAssetFeeDebtOf_4', '_fAssetFeeDebtOf_9', '_fAssetFeeDebtOf_10'])
totalFAssetFeeDebt_2(int256) := phi(['totalFAssetFeeDebt_0', 'totalFAssetFeeDebt_5', 'totalFAssetFeeDebt_3'])
 _fAssets == 0
TMP_6296(bool) = _fAssets_1 == 0
CONDITION TMP_6296
 fAssets = _fAssets.toInt256()
TMP_6297(int256) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toInt256(uint256), arguments:['_fAssets_1'] 
fAssets_1(int256) := TMP_6297(int256)
 _fAssetFeeDebtOf[_account] += fAssets
REF_4248(int256) -> _fAssetFeeDebtOf_6[_account_1]
_fAssetFeeDebtOf_7(mapping(address => int256)) := phi(['_fAssetFeeDebtOf_6'])
REF_4248(-> _fAssetFeeDebtOf_7) = REF_4248 (c)+ fAssets_1
 totalFAssetFeeDebt += fAssets
totalFAssetFeeDebt_3(int256) = totalFAssetFeeDebt_2 (c)+ fAssets_1
 CPFeeDebtChanged(_account,_fAssetFeeDebtOf[_account])
REF_4249(int256) -> _fAssetFeeDebtOf_7[_account_1]
Emit CPFeeDebtChanged(_account_1,REF_4249)
```
#### CollateralPool._debtFreeTokensOf(address) [INTERNAL]
```slithir
_account_1(address) := phi(['_account_1', '_account_1'])
token_54(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_0', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
_fAssetFeeDebtOf_5(mapping(address => int256)) := phi(['_fAssetFeeDebtOf_5', '_fAssetFeeDebtOf_0', '_fAssetFeeDebtOf_2', '_fAssetFeeDebtOf_7', '_fAssetFeeDebtOf_4', '_fAssetFeeDebtOf_9', '_fAssetFeeDebtOf_10'])
 accountFeeDebt = _fAssetFeeDebtOf[_account]
REF_4235(int256) -> _fAssetFeeDebtOf_5[_account_1]
accountFeeDebt_1(int256) := REF_4235(int256)
 accountFeeDebt <= 0
TMP_6260(bool) = accountFeeDebt_1 <= 0
CONDITION TMP_6260
 token.balanceOf(_account)
TMP_6261(uint256) = HIGH_LEVEL_CALL, dest:token_54(IICollateralPoolToken), function:balanceOf, arguments:['_account_1']  
token_55(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44', 'token_54'])
RETURN TMP_6261
 virtualFassets = _virtualFAssetFeesOf(_account)
TMP_6262(uint256) = INTERNAL_CALL, CollateralPool._virtualFAssetFeesOf(address)(_account_1)
token_56(IICollateralPoolToken) := phi(['token_53'])
virtualFassets_1(uint256) := TMP_6262(uint256)
 assert(bool)(virtualFassets <= _totalVirtualFees())
TMP_6263(uint256) = INTERNAL_CALL, CollateralPool._totalVirtualFees()()
TMP_6264(bool) = virtualFassets_1 <= TMP_6263
TMP_6265(None) = SOLIDITY_CALL assert(bool)(TMP_6264)
 freeFassets = MathUtils.positivePart(virtualFassets.toInt256() - accountFeeDebt)
TMP_6266(int256) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toInt256(uint256), arguments:['virtualFassets_1'] 
TMP_6267(int256) = TMP_6266 (c)- accountFeeDebt_1
TMP_6268(uint256) = LIBRARY_CALL, dest:MathUtils, function:MathUtils.positivePart(int256), arguments:['TMP_6267'] 
freeFassets_1(uint256) := TMP_6268(uint256)
 freeFassets == 0
TMP_6269(bool) = freeFassets_1 == 0
CONDITION TMP_6269
 0
RETURN 0
 token.totalSupply().mulDiv(freeFassets,_totalVirtualFees())
TMP_6270(uint256) = HIGH_LEVEL_CALL, dest:token_57(IICollateralPoolToken), function:totalSupply, arguments:[]  
token_58(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
TMP_6271(uint256) = INTERNAL_CALL, CollateralPool._totalVirtualFees()()
TMP_6272(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['TMP_6270', 'freeFassets_1', 'TMP_6271'] 
RETURN TMP_6272
```
#### CollateralPool._deleteFAssetFeeDebt(address,uint256) [INTERNAL]
```slithir
_account_1(address) := phi(['agentVault_22', 'msg.sender'])
_fAssets_1(uint256) := phi(['debtFAssetFeeShare_1', '_fAssets_1', 'debtFAssetFeeShare_1', 'debtFAssetFeeShare_1'])
_fAssetFeeDebtOf_8(mapping(address => int256)) := phi(['_fAssetFeeDebtOf_5', '_fAssetFeeDebtOf_0', '_fAssetFeeDebtOf_2', '_fAssetFeeDebtOf_7', '_fAssetFeeDebtOf_4', '_fAssetFeeDebtOf_9', '_fAssetFeeDebtOf_10'])
totalFAssetFeeDebt_4(int256) := phi(['totalFAssetFeeDebt_0', 'totalFAssetFeeDebt_5', 'totalFAssetFeeDebt_3'])
 _fAssets == 0
TMP_6299(bool) = _fAssets_1 == 0
CONDITION TMP_6299
 fAssets = _fAssets.toInt256()
TMP_6300(int256) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toInt256(uint256), arguments:['_fAssets_1'] 
fAssets_1(int256) := TMP_6300(int256)
 _fAssetFeeDebtOf[_account] -= fAssets
REF_4251(int256) -> _fAssetFeeDebtOf_8[_account_1]
_fAssetFeeDebtOf_9(mapping(address => int256)) := phi(['_fAssetFeeDebtOf_8'])
REF_4251(-> _fAssetFeeDebtOf_9) = REF_4251 (c)- fAssets_1
 totalFAssetFeeDebt -= fAssets
totalFAssetFeeDebt_5(int256) = totalFAssetFeeDebt_4 (c)- fAssets_1
 CPFeeDebtChanged(_account,_fAssetFeeDebtOf[_account])
REF_4252(int256) -> _fAssetFeeDebtOf_9[_account_1]
Emit CPFeeDebtChanged(_account_1,REF_4252)
```
#### CollateralPool._depositWNat() [INTERNAL]
```slithir
agentVault_29(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_1', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_0', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_28(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_14', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_0', 'assetManager_30'])
wNat_7(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_0', 'wNat_25', 'wNat_15', 'wNat_36'])
totalCollateral_36(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_0', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
 msg.value > 0
TMP_6312(bool) = msg.value > 0
CONDITION TMP_6312
 totalCollateral += msg.value
totalCollateral_37(uint256) = totalCollateral_36 (c)+ msg.value
 wNat.deposit{value: msg.value}()
HIGH_LEVEL_CALL, dest:wNat_7(IWNat), function:deposit, arguments:[] value:msg.value 
agentVault_30(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_29', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_29(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_28', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
wNat_8(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_7', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_25', 'wNat_15', 'wNat_36'])
 assetManager.updateCollateral(agentVault,wNat)
HIGH_LEVEL_CALL, dest:assetManager_29(IIAssetManager), function:updateCollateral, arguments:['agentVault_30', 'wNat_8']  
agentVault_31(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_30', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_30(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_29', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
wNat_9(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_8', 'wNat_28', 'wNat_43', 'wNat_25', 'wNat_15', 'wNat_36'])
```
#### CollateralPool._exitTo(uint256,address) [PRIVATE]
```slithir
_tokenShare_1(uint256) := phi(['_tokenShare_1', '_tokenShare_1'])
_recipient_1(address) := phi(['_recipient_1', 'TMP_6091'])
token_14(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_0', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
totalCollateral_4(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_0', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
 require(bool,error)(_tokenShare > 0,revert TokenShareIsZero()())
TMP_6096(bool) = _tokenShare_1 > 0
TMP_6097(None) = SOLIDITY_CALL revert TokenShareIsZero()()
TMP_6098(None) = SOLIDITY_CALL require(bool,error)(TMP_6096,TMP_6097)
 require(bool,error)(_tokenShare <= token.balanceOf(msg.sender),revert TokenBalanceTooLow()())
TMP_6099(uint256) = HIGH_LEVEL_CALL, dest:token_14(IICollateralPoolToken), function:balanceOf, arguments:['msg.sender']  
token_15(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_14', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
totalCollateral_5(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_4', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
TMP_6100(bool) = _tokenShare_1 <= TMP_6099
TMP_6101(None) = SOLIDITY_CALL revert TokenBalanceTooLow()()
TMP_6102(None) = SOLIDITY_CALL require(bool,error)(TMP_6100,TMP_6101)
 _requireMinTokenSupplyAfterExit(_tokenShare)
INTERNAL_CALL, CollateralPool._requireMinTokenSupplyAfterExit(uint256)(_tokenShare_1)
token_16(IICollateralPoolToken) := phi(['token_61'])
 natShare = totalCollateral.mulDiv(_tokenShare,token.totalSupply())
TMP_6104(uint256) = HIGH_LEVEL_CALL, dest:token_16(IICollateralPoolToken), function:totalSupply, arguments:[]  
token_17(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_57', 'token_22', 'token_16', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
totalCollateral_7(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30', 'totalCollateral_6'])
TMP_6105(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['totalCollateral_7', '_tokenShare_1', 'TMP_6104'] 
natShare_1(uint256) := TMP_6105(uint256)
 require(bool,error)(natShare > 0,revert SentAmountTooLow()())
TMP_6106(bool) = natShare_1 > 0
TMP_6107(None) = SOLIDITY_CALL revert SentAmountTooLow()()
TMP_6108(None) = SOLIDITY_CALL require(bool,error)(TMP_6106,TMP_6107)
 _requireMinNatSupplyAfterExit(natShare)
INTERNAL_CALL, CollateralPool._requireMinNatSupplyAfterExit(uint256)(natShare_1)
 require(bool,error)(_staysAboveExitCR(natShare),revert CollateralRatioFallsBelowExitCR()())
TMP_6110(bool) = INTERNAL_CALL, CollateralPool._staysAboveExitCR(uint256)(natShare_1)
TMP_6111(None) = SOLIDITY_CALL revert CollateralRatioFallsBelowExitCR()()
TMP_6112(None) = SOLIDITY_CALL require(bool,error)(TMP_6110,TMP_6111)
 debtFAssetFeeShare = _tokensToVirtualFeeShare(_tokenShare)
TMP_6113(uint256) = INTERNAL_CALL, CollateralPool._tokensToVirtualFeeShare(uint256)(_tokenShare_1)
token_20(IICollateralPoolToken) := phi(['token_51'])
debtFAssetFeeShare_1(uint256) := TMP_6113(uint256)
 _deleteFAssetFeeDebt(msg.sender,debtFAssetFeeShare)
INTERNAL_CALL, CollateralPool._deleteFAssetFeeDebt(address,uint256)(msg.sender,debtFAssetFeeShare_1)
 token.burn(msg.sender,_tokenShare,false)
HIGH_LEVEL_CALL, dest:token_21(IICollateralPoolToken), function:burn, arguments:['msg.sender', '_tokenShare_1', 'False']  
token_22(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_21', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
 _withdrawWNatTo(_recipient,natShare)
INTERNAL_CALL, CollateralPool._withdrawWNatTo(address,uint256)(_recipient_1,natShare_1)
 CPExited(msg.sender,_tokenShare,natShare)
Emit CPExited(msg.sender,_tokenShare_1,natShare_1)
 natShare
RETURN natShare_1
```
#### CollateralPool._fAssetFeesOf(address) [INTERNAL]
```slithir
_account_1(address) := phi(['_account_1', 'msg.sender'])
_fAssetFeeDebtOf_3(mapping(address => int256)) := phi(['_fAssetFeeDebtOf_5', '_fAssetFeeDebtOf_0', '_fAssetFeeDebtOf_2', '_fAssetFeeDebtOf_7', '_fAssetFeeDebtOf_4', '_fAssetFeeDebtOf_9', '_fAssetFeeDebtOf_10'])
totalFAssetFees_5(uint256) := phi(['totalFAssetFees_3', 'totalFAssetFees_0', 'totalFAssetFees_14', 'totalFAssetFees_12', 'totalFAssetFees_6', 'totalFAssetFees_4', 'totalFAssetFees_10'])
 virtualFAssetFees = _virtualFAssetFeesOf(_account).toInt256()
TMP_6255(uint256) = INTERNAL_CALL, CollateralPool._virtualFAssetFeesOf(address)(_account_1)
TMP_6256(int256) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toInt256(uint256), arguments:['TMP_6255'] 
virtualFAssetFees_1(int256) := TMP_6256(int256)
 accountFeeDebt = _fAssetFeeDebtOf[_account]
REF_4232(int256) -> _fAssetFeeDebtOf_4[_account_1]
accountFeeDebt_1(int256) := REF_4232(int256)
 userFees = virtualFAssetFees - accountFeeDebt
TMP_6257(int256) = virtualFAssetFees_1 (c)- accountFeeDebt_1
userFees_1(int256) := TMP_6257(int256)
 Math.min(MathUtils.positivePart(userFees),totalFAssetFees)
TMP_6258(uint256) = LIBRARY_CALL, dest:MathUtils, function:MathUtils.positivePart(int256), arguments:['userFees_1'] 
TMP_6259(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['TMP_6258', 'totalFAssetFees_6'] 
RETURN TMP_6259
```
#### CollateralPool._getAssetPrice() [INTERNAL]
```slithir
assetManager_24(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_14', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_0', 'assetManager_30'])
 (assetPriceMul,assetPriceDiv) = assetManager.assetPriceNatWei()
TUPLE_69(uint256,uint256) = HIGH_LEVEL_CALL, dest:assetManager_24(IIAssetManager), function:assetPriceNatWei, arguments:[]  
assetManager_25(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_24', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
assetPriceMul_1(uint256)= UNPACK TUPLE_69 index: 0 
assetPriceDiv_1(uint256)= UNPACK TUPLE_69 index: 1 
 AssetPrice({mul:assetPriceMul,div:assetPriceDiv})
TMP_6273(CollateralPool.AssetPrice) = new AssetPrice(assetPriceMul_1,assetPriceDiv_1)
RETURN TMP_6273
```
#### CollateralPool._getFAssetRequiredToNotSpoilCR(uint256) [INTERNAL]
```slithir
_natShare_1(uint256) := phi(['tokenNatWeiEquiv_1', 'natShare_1'])
assetManager_16(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_14', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_0', 'assetManager_30'])
totalCollateral_21(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_0', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
 assetPrice = _getAssetPrice()
TMP_6230(CollateralPool.AssetPrice) = INTERNAL_CALL, CollateralPool._getAssetPrice()()
assetManager_17(IIAssetManager) := phi(['assetManager_25'])
assetPrice_1(CollateralPool.AssetPrice) := TMP_6230(CollateralPool.AssetPrice)
 exitCR = _safeExitCR()
TMP_6231(uint256) = INTERNAL_CALL, CollateralPool._safeExitCR()()
assetManager_18(IIAssetManager) := phi(['assetManager_27'])
exitCR_1(uint256) := TMP_6231(uint256)
 backedFAssets = _agentBackedFAssets()
TMP_6232(uint256) = INTERNAL_CALL, CollateralPool._agentBackedFAssets()()
assetManager_19(IIAssetManager) := phi(['assetManager_23'])
backedFAssets_1(uint256) := TMP_6232(uint256)
 _isAboveCR(assetPrice,backedFAssets,totalCollateral,exitCR)
TMP_6233(bool) = INTERNAL_CALL, CollateralPool._isAboveCR(CollateralPool.AssetPrice,uint256,uint256,uint256)(assetPrice_1,backedFAssets_1,totalCollateral_24,exitCR_1)
CONDITION TMP_6233
 resultWithoutRounding = MathUtils.subOrZero(backedFAssets,assetPrice.div * (totalCollateral - _natShare) * SafePct.MAX_BIPS / (assetPrice.mul * exitCR))
REF_4220(uint256) -> assetPrice_1.div
TMP_6234(uint256) = totalCollateral_25 (c)- _natShare_1
TMP_6235(uint256) = REF_4220 (c)* TMP_6234
REF_4221(uint256) -> SafePct.MAX_BIPS
TMP_6236(uint256) = TMP_6235 (c)* REF_4221
REF_4222(uint256) -> assetPrice_1.mul
TMP_6237(uint256) = REF_4222 (c)* exitCR_1
TMP_6238(uint256) = TMP_6236 (c)/ TMP_6237
TMP_6239(uint256) = LIBRARY_CALL, dest:MathUtils, function:MathUtils.subOrZero(uint256,uint256), arguments:['backedFAssets_1', 'TMP_6238'] 
resultWithoutRounding_1(uint256) := TMP_6239(uint256)
 resultWithoutRounding = backedFAssets.mulDivRoundUp(_natShare,totalCollateral)
TMP_6240(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDivRoundUp(uint256,uint256,uint256), arguments:['backedFAssets_1', '_natShare_1', 'totalCollateral_25'] 
resultWithoutRounding_2(uint256) := TMP_6240(uint256)
resultWithoutRounding_3(uint256) := phi(['resultWithoutRounding_1', 'resultWithoutRounding_2'])
 MathUtils.roundUp(resultWithoutRounding,assetManager.assetMintingGranularityUBA())
TMP_6241(uint256) = HIGH_LEVEL_CALL, dest:assetManager_20(IIAssetManager), function:assetMintingGranularityUBA, arguments:[]  
assetManager_21(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_20', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
TMP_6242(uint256) = LIBRARY_CALL, dest:MathUtils, function:MathUtils.roundUp(uint256,uint256), arguments:['resultWithoutRounding_3', 'TMP_6241'] 
RETURN TMP_6242
```
#### CollateralPool._isAboveCR(CollateralPool.AssetPrice,uint256,uint256,uint256) [INTERNAL]
```slithir
_assetPrice_1(CollateralPool.AssetPrice) := phi(['TMP_6243', 'assetPrice_1'])
_backedFAssets_1(uint256) := phi(['TMP_6244', 'backedFAssets_1'])
_poolCollateralNat_1(uint256) := phi(['totalCollateral_24', 'TMP_6245'])
_crBIPS_1(uint256) := phi(['exitCR_1', 'TMP_6246'])
 _poolCollateralNat * _assetPrice.div >= (_backedFAssets * _assetPrice.mul).mulBips(_crBIPS)
REF_4226(uint256) -> _assetPrice_1.div
TMP_6248(uint256) = _poolCollateralNat_1 (c)* REF_4226
REF_4227(uint256) -> _assetPrice_1.mul
TMP_6249(uint256) = _backedFAssets_1 (c)* REF_4227
TMP_6250(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulBips(uint256,uint256), arguments:['TMP_6249', '_crBIPS_1'] 
TMP_6251(bool) = TMP_6248 >= TMP_6250
RETURN TMP_6251
```
#### CollateralPool._requireMinNatSupplyAfterExit(uint256) [INTERNAL]
```slithir
_natShare_1(uint256) := phi(['natShare_1', 'natShare_1'])
MIN_NAT_BALANCE_AFTER_EXIT_1(uint256) := phi(['MIN_NAT_BALANCE_AFTER_EXIT_0'])
totalCollateral_31(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_0', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
 require(bool,error)(totalCollateral == _natShare || totalCollateral - _natShare >= MIN_NAT_BALANCE_AFTER_EXIT,revert CollateralAfterExitTooLow()())
TMP_6286(bool) = totalCollateral_31 == _natShare_1
TMP_6287(uint256) = totalCollateral_31 (c)- _natShare_1
TMP_6288(bool) = TMP_6287 >= MIN_NAT_BALANCE_AFTER_EXIT_1
TMP_6289(bool) = TMP_6286 || TMP_6288
TMP_6290(None) = SOLIDITY_CALL revert CollateralAfterExitTooLow()()
TMP_6291(None) = SOLIDITY_CALL require(bool,error)(TMP_6289,TMP_6290)
```
#### CollateralPool._requireMinTokenSupplyAfterExit(uint256) [INTERNAL]
```slithir
_tokenShare_1(uint256) := phi(['_tokenShare_1', '_tokenShare_1'])
MIN_TOKEN_SUPPLY_AFTER_EXIT_1(uint256) := phi(['MIN_TOKEN_SUPPLY_AFTER_EXIT_2', 'MIN_TOKEN_SUPPLY_AFTER_EXIT_0'])
token_60(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_0', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
 totalPoolTokens = token.totalSupply()
TMP_6279(uint256) = HIGH_LEVEL_CALL, dest:token_60(IICollateralPoolToken), function:totalSupply, arguments:[]  
MIN_TOKEN_SUPPLY_AFTER_EXIT_2(uint256) := phi(['MIN_TOKEN_SUPPLY_AFTER_EXIT_2', 'MIN_TOKEN_SUPPLY_AFTER_EXIT_1'])
token_61(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_57', 'token_22', 'token_60', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
totalPoolTokens_1(uint256) := TMP_6279(uint256)
 require(bool,error)(totalPoolTokens == _tokenShare || totalPoolTokens - _tokenShare >= MIN_TOKEN_SUPPLY_AFTER_EXIT,revert TokenSupplyAfterExitTooLow()())
TMP_6280(bool) = totalPoolTokens_1 == _tokenShare_1
TMP_6281(uint256) = totalPoolTokens_1 (c)- _tokenShare_1
TMP_6282(bool) = TMP_6281 >= MIN_TOKEN_SUPPLY_AFTER_EXIT_2
TMP_6283(bool) = TMP_6280 || TMP_6282
TMP_6284(None) = SOLIDITY_CALL revert TokenSupplyAfterExitTooLow()()
TMP_6285(None) = SOLIDITY_CALL require(bool,error)(TMP_6283,TMP_6284)
```
#### CollateralPool._safeExitCR() [INTERNAL]
```slithir
agentVault_27(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_1', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_0', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_26(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_14', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_0', 'assetManager_30'])
exitCollateralRatioBIPS_3(uint32) := phi(['exitCollateralRatioBIPS_2', 'exitCollateralRatioBIPS_1', 'exitCollateralRatioBIPS_0', 'exitCollateralRatioBIPS_4'])
 minPoolCollateralRatioBIPS = assetManager.getAgentMinPoolCollateralRatioBIPS(agentVault)
TMP_6277(uint256) = HIGH_LEVEL_CALL, dest:assetManager_26(IIAssetManager), function:getAgentMinPoolCollateralRatioBIPS, arguments:['agentVault_27']  
agentVault_28(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_27', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_27(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_26', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
exitCollateralRatioBIPS_4(uint32) := phi(['exitCollateralRatioBIPS_2', 'exitCollateralRatioBIPS_3', 'exitCollateralRatioBIPS_1', 'exitCollateralRatioBIPS_4'])
minPoolCollateralRatioBIPS_1(uint256) := TMP_6277(uint256)
 Math.max(minPoolCollateralRatioBIPS,exitCollateralRatioBIPS)
TMP_6278(uint256) = LIBRARY_CALL, dest:Math, function:Math.max(uint256,uint256), arguments:['minPoolCollateralRatioBIPS_1', 'exitCollateralRatioBIPS_4'] 
RETURN TMP_6278
```
#### CollateralPool._selfCloseExitTo(uint256,bool,address,string,address) [PRIVATE]
```slithir
_tokenShare_1(uint256) := phi(['_tokenShare_1', '_tokenShare_1'])
_redeemToCollateral_1(bool) := phi(['_redeemToCollateral_1', '_redeemToCollateral_1'])
_recipient_1(address) := phi(['TMP_6118', '_recipient_1'])
_redeemerUnderlyingAddress_1(string) := phi(['_redeemerUnderlyingAddress_1', '_redeemerUnderlyingAddress_1'])
_executor_1(address) := phi(['_executor_1', '_executor_1'])
agentVault_4(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_1', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_0', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_3(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_14', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_0', 'assetManager_30'])
fAsset_2(IFAsset) := phi(['fAsset_13', 'fAsset_0', 'fAsset_10', 'fAsset_21', 'fAsset_1'])
token_23(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_0', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
totalCollateral_8(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_0', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
 require(bool,error)(_tokenShare > 0,revert TokenShareIsZero()())
TMP_6132(bool) = _tokenShare_1 > 0
TMP_6133(None) = SOLIDITY_CALL revert TokenShareIsZero()()
TMP_6134(None) = SOLIDITY_CALL require(bool,error)(TMP_6132,TMP_6133)
 require(bool,error)(_tokenShare <= token.balanceOf(msg.sender),revert TokenBalanceTooLow()())
TMP_6135(uint256) = HIGH_LEVEL_CALL, dest:token_23(IICollateralPoolToken), function:balanceOf, arguments:['msg.sender']  
agentVault_5(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_4', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_4(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_3', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
fAsset_3(IFAsset) := phi(['fAsset_2', 'fAsset_13', 'fAsset_10', 'fAsset_21', 'fAsset_1'])
token_24(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_57', 'token_23', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
totalCollateral_9(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_8', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
TMP_6136(bool) = _tokenShare_1 <= TMP_6135
TMP_6137(None) = SOLIDITY_CALL revert TokenBalanceTooLow()()
TMP_6138(None) = SOLIDITY_CALL require(bool,error)(TMP_6136,TMP_6137)
 _requireMinTokenSupplyAfterExit(_tokenShare)
INTERNAL_CALL, CollateralPool._requireMinTokenSupplyAfterExit(uint256)(_tokenShare_1)
token_25(IICollateralPoolToken) := phi(['token_61'])
 natShare = totalCollateral.mulDiv(_tokenShare,token.totalSupply())
TMP_6140(uint256) = HIGH_LEVEL_CALL, dest:token_25(IICollateralPoolToken), function:totalSupply, arguments:[]  
agentVault_7(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_6', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_6(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_5', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
fAsset_5(IFAsset) := phi(['fAsset_4', 'fAsset_13', 'fAsset_10', 'fAsset_21', 'fAsset_1'])
token_26(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_57', 'token_25', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
totalCollateral_11(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_10', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
TMP_6141(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['totalCollateral_11', '_tokenShare_1', 'TMP_6140'] 
natShare_1(uint256) := TMP_6141(uint256)
 require(bool,error)(natShare > 0,revert SentAmountTooLow()())
TMP_6142(bool) = natShare_1 > 0
TMP_6143(None) = SOLIDITY_CALL revert SentAmountTooLow()()
TMP_6144(None) = SOLIDITY_CALL require(bool,error)(TMP_6142,TMP_6143)
 _requireMinNatSupplyAfterExit(natShare)
INTERNAL_CALL, CollateralPool._requireMinNatSupplyAfterExit(uint256)(natShare_1)
 maxAgentRedemption = assetManager.maxRedemptionFromAgent(agentVault)
TMP_6146(uint256) = HIGH_LEVEL_CALL, dest:assetManager_7(IIAssetManager), function:maxRedemptionFromAgent, arguments:['agentVault_8']  
agentVault_9(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_8', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_8(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_23', 'assetManager_7', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
fAsset_7(IFAsset) := phi(['fAsset_13', 'fAsset_10', 'fAsset_21', 'fAsset_1', 'fAsset_6'])
token_28(IICollateralPoolToken) := phi(['token_27', 'token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
maxAgentRedemption_1(uint256) := TMP_6146(uint256)
 requiredFAssets = _getFAssetRequiredToNotSpoilCR(natShare)
TMP_6147(uint256) = INTERNAL_CALL, CollateralPool._getFAssetRequiredToNotSpoilCR(uint256)(natShare_1)
assetManager_9(IIAssetManager) := phi(['assetManager_21'])
requiredFAssets_1(uint256) := TMP_6147(uint256)
 require(bool,error)(maxAgentRedemption > requiredFAssets,revert RedemptionRequiresClosingTooManyTickets()())
TMP_6148(bool) = maxAgentRedemption_1 > requiredFAssets_1
TMP_6149(None) = SOLIDITY_CALL revert RedemptionRequiresClosingTooManyTickets()()
TMP_6150(None) = SOLIDITY_CALL require(bool,error)(TMP_6148,TMP_6149)
 debtFAssetFeeShare = _tokensToVirtualFeeShare(_tokenShare)
TMP_6151(uint256) = INTERNAL_CALL, CollateralPool._tokensToVirtualFeeShare(uint256)(_tokenShare_1)
token_30(IICollateralPoolToken) := phi(['token_51'])
debtFAssetFeeShare_1(uint256) := TMP_6151(uint256)
 require(bool,error)(fAsset.allowance(msg.sender,address(this)) >= requiredFAssets,revert FAssetAllowanceTooSmall()())
TMP_6152 = CONVERT this to address
TMP_6153(uint256) = HIGH_LEVEL_CALL, dest:fAsset_9(IFAsset), function:allowance, arguments:['msg.sender', 'TMP_6152']  
agentVault_12(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_11', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_11(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_1', 'assetManager_11', 'assetManager_10', 'assetManager_15', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
fAsset_10(IFAsset) := phi(['fAsset_9', 'fAsset_13', 'fAsset_10', 'fAsset_21', 'fAsset_1'])
token_31(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_30', 'token_49', 'token_68', 'token_59', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
TMP_6154(bool) = TMP_6153 >= requiredFAssets_1
TMP_6155(None) = SOLIDITY_CALL revert FAssetAllowanceTooSmall()()
TMP_6156(None) = SOLIDITY_CALL require(bool,error)(TMP_6154,TMP_6155)
 fAsset.safeTransferFrom(msg.sender,address(this),requiredFAssets)
TMP_6157 = CONVERT this to address
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransferFrom(IERC20,address,address,uint256), arguments:['fAsset_10', 'msg.sender', 'TMP_6157', 'requiredFAssets_1'] 
 returnFunds = true
returnFunds_1(bool) := True(bool)
 requiredFAssets > 0
TMP_6159(bool) = requiredFAssets_1 > 0
CONDITION TMP_6159
 requiredFAssets < assetManager.lotSize() || _redeemToCollateral
TMP_6160(uint256) = HIGH_LEVEL_CALL, dest:assetManager_11(IIAssetManager), function:lotSize, arguments:[]  
agentVault_13(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_12(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
token_32(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_57', 'token_31', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
TMP_6161(bool) = requiredFAssets_1 < TMP_6160
TMP_6162(bool) = TMP_6161 || _redeemToCollateral_1
CONDITION TMP_6162
 assetManager.redeemFromAgentInCollateral(agentVault,_recipient,requiredFAssets)
HIGH_LEVEL_CALL, dest:assetManager_12(IIAssetManager), function:redeemFromAgentInCollateral, arguments:['agentVault_13', '_recipient_1', 'requiredFAssets_1']  
agentVault_14(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_13', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_13(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_1', 'assetManager_11', 'assetManager_12', 'assetManager_15', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
token_33(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_57', 'token_22', 'token_32', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
 returnFunds = _executor == address(0)
TMP_6164 = CONVERT 0 to address
TMP_6165(bool) = _executor_1 == TMP_6164
returnFunds_2(bool) := TMP_6165(bool)
returnFunds_3(bool) := phi(['returnFunds_1', 'returnFunds_2'])
 _deleteFAssetFeeDebt(msg.sender,debtFAssetFeeShare)
INTERNAL_CALL, CollateralPool._deleteFAssetFeeDebt(address,uint256)(msg.sender,debtFAssetFeeShare_1)
 token.burn(msg.sender,_tokenShare,false)
HIGH_LEVEL_CALL, dest:token_36(IICollateralPoolToken), function:burn, arguments:['msg.sender', '_tokenShare_1', 'False']  
token_37(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_57', 'token_36', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
 _withdrawWNatTo(_recipient,natShare)
INTERNAL_CALL, CollateralPool._withdrawWNatTo(address,uint256)(_recipient_1,natShare_1)
 returnFunds
CONDITION returnFunds_3
 Transfers.transferNAT(_recipient,msg.value)
LIBRARY_CALL, dest:Transfers, function:Transfers.transferNAT(address,uint256), arguments:['_recipient_1', 'msg.value'] 
 CPSelfCloseExited(msg.sender,_tokenShare,natShare,requiredFAssets)
Emit CPSelfCloseExited(msg.sender,_tokenShare_1,natShare_1,requiredFAssets_1)
 returnFunds
CONDITION returnFunds_2
 assetManager.redeemFromAgent{value: 0}(agentVault,_recipient,requiredFAssets,_redeemerUnderlyingAddress,_executor)
HIGH_LEVEL_CALL, dest:assetManager_12(IIAssetManager), function:redeemFromAgent, arguments:['agentVault_13', '_recipient_1', 'requiredFAssets_1', '_redeemerUnderlyingAddress_1', '_executor_1'] value:0 
agentVault_16(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_15(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
token_35(IICollateralPoolToken) := phi(['token_13', 'token_34', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
 assetManager.redeemFromAgent{value: msg.value}(agentVault,_recipient,requiredFAssets,_redeemerUnderlyingAddress,_executor)
HIGH_LEVEL_CALL, dest:assetManager_12(IIAssetManager), function:redeemFromAgent, arguments:['agentVault_13', '_recipient_1', 'requiredFAssets_1', '_redeemerUnderlyingAddress_1', '_executor_1'] value:msg.value 
agentVault_15(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_14(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
token_34(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_33', 'token_49', 'token_68', 'token_59', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
```
#### CollateralPool._staysAboveExitCR(uint256) [INTERNAL]
```slithir
_withdrawnNat_1(uint256) := phi(['natShare_1'])
totalCollateral_26(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_0', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
 _isAboveCR(_getAssetPrice(),_agentBackedFAssets(),totalCollateral - _withdrawnNat,_safeExitCR())
TMP_6243(CollateralPool.AssetPrice) = INTERNAL_CALL, CollateralPool._getAssetPrice()()
TMP_6244(uint256) = INTERNAL_CALL, CollateralPool._agentBackedFAssets()()
TMP_6245(uint256) = totalCollateral_28 (c)- _withdrawnNat_1
TMP_6246(uint256) = INTERNAL_CALL, CollateralPool._safeExitCR()()
TMP_6247(bool) = INTERNAL_CALL, CollateralPool._isAboveCR(CollateralPool.AssetPrice,uint256,uint256,uint256)(TMP_6243,TMP_6244,TMP_6245,TMP_6246)
RETURN TMP_6247
```
#### CollateralPool._tokensToVirtualFeeShare(uint256) [INTERNAL]
```slithir
_tokens_1(uint256) := phi(['slashedTokens_1', '_tokenShare_1', 'tokens_1', '_tokenShare_1'])
token_50(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_0', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
 _tokens == 0
TMP_6224(bool) = _tokens_1 == 0
CONDITION TMP_6224
 0
RETURN 0
 totalPoolTokens = token.totalSupply()
TMP_6225(uint256) = HIGH_LEVEL_CALL, dest:token_50(IICollateralPoolToken), function:totalSupply, arguments:[]  
token_51(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_50', 'token_49', 'token_68', 'token_59', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
totalPoolTokens_1(uint256) := TMP_6225(uint256)
 assert(bool)(_tokens <= totalPoolTokens)
TMP_6226(bool) = _tokens_1 <= totalPoolTokens_1
TMP_6227(None) = SOLIDITY_CALL assert(bool)(TMP_6226)
 _totalVirtualFees().mulDiv(_tokens,totalPoolTokens)
TMP_6228(uint256) = INTERNAL_CALL, CollateralPool._totalVirtualFees()()
TMP_6229(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['TMP_6228', '_tokens_1', 'totalPoolTokens_1'] 
RETURN TMP_6229
```
#### CollateralPool._totalVirtualFees() [INTERNAL]
```slithir
totalFAssetFeeDebt_1(int256) := phi(['totalFAssetFeeDebt_0', 'totalFAssetFeeDebt_5', 'totalFAssetFeeDebt_3'])
totalFAssetFees_7(uint256) := phi(['totalFAssetFees_3', 'totalFAssetFees_0', 'totalFAssetFees_14', 'totalFAssetFees_12', 'totalFAssetFees_6', 'totalFAssetFees_4', 'totalFAssetFees_10'])
 virtualFees = totalFAssetFees.toInt256() + totalFAssetFeeDebt
TMP_6274(int256) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toInt256(uint256), arguments:['totalFAssetFees_7'] 
TMP_6275(int256) = TMP_6274 (c)+ totalFAssetFeeDebt_1
virtualFees_1(int256) := TMP_6275(int256)
 virtualFees.toUint256()
TMP_6276(uint256) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint256(int256), arguments:['virtualFees_1'] 
RETURN TMP_6276
```
#### CollateralPool._transferFAssetFrom(address,uint256) [INTERNAL]
```slithir
_from_1(address) := phi(['msg.sender'])
_amount_1(uint256) := phi(['_fAssets_1'])
fAsset_14(IFAsset) := phi(['fAsset_13', 'fAsset_0', 'fAsset_10', 'fAsset_21', 'fAsset_1'])
totalFAssetFees_11(uint256) := phi(['totalFAssetFees_3', 'totalFAssetFees_0', 'totalFAssetFees_14', 'totalFAssetFees_12', 'totalFAssetFees_6', 'totalFAssetFees_4', 'totalFAssetFees_10'])
 _amount > 0
TMP_6302(bool) = _amount_1 > 0
CONDITION TMP_6302
 totalFAssetFees += _amount
totalFAssetFees_12(uint256) = totalFAssetFees_11 (c)+ _amount_1
 fAsset.safeTransferFrom(_from,address(this),_amount)
TMP_6303 = CONVERT this to address
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransferFrom(IERC20,address,address,uint256), arguments:['fAsset_14', '_from_1', 'TMP_6303', '_amount_1']
```
#### CollateralPool._transferFAssetTo(address,uint256) [INTERNAL]
```slithir
_to_1(address) := phi(['_recipient_1'])
_amount_1(uint256) := phi(['_fAssets_1'])
fAsset_15(IFAsset) := phi(['fAsset_13', 'fAsset_0', 'fAsset_10', 'fAsset_21', 'fAsset_1'])
totalFAssetFees_13(uint256) := phi(['totalFAssetFees_3', 'totalFAssetFees_0', 'totalFAssetFees_14', 'totalFAssetFees_12', 'totalFAssetFees_6', 'totalFAssetFees_4', 'totalFAssetFees_10'])
 _amount > 0
TMP_6305(bool) = _amount_1 > 0
CONDITION TMP_6305
 totalFAssetFees -= _amount
totalFAssetFees_14(uint256) = totalFAssetFees_13 (c)- _amount_1
 fAsset.safeTransfer(_to,_amount)
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['fAsset_15', '_to_1', '_amount_1']
```
#### CollateralPool._transferWNatTo(address,uint256) [INTERNAL]
```slithir
_to_1(address) := phi(['_recipient_1'])
_amount_1(uint256) := phi(['_amount_1'])
wNat_4(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_0', 'wNat_25', 'wNat_15', 'wNat_36'])
totalCollateral_32(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_0', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
 _amount > 0
TMP_6307(bool) = _amount_1 > 0
CONDITION TMP_6307
 totalCollateral -= _amount
totalCollateral_33(uint256) = totalCollateral_32 (c)- _amount_1
 wNat.safeTransfer(_to,_amount)
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['wNat_4', '_to_1', '_amount_1']
```
#### CollateralPool._virtualFAssetFeesOf(address) [INTERNAL]
```slithir
_account_1(address) := phi(['_account_1', '_account_1', '_account_1'])
token_52(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_0', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
 tokens = token.balanceOf(_account)
TMP_6253(uint256) = HIGH_LEVEL_CALL, dest:token_52(IICollateralPoolToken), function:balanceOf, arguments:['_account_1']  
token_53(IICollateralPoolToken) := phi(['token_52', 'token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
tokens_1(uint256) := TMP_6253(uint256)
 _tokensToVirtualFeeShare(tokens)
TMP_6254(uint256) = INTERNAL_CALL, CollateralPool._tokensToVirtualFeeShare(uint256)(tokens_1)
RETURN TMP_6254
```
#### CollateralPool._withdrawFeesTo(uint256,address) [PRIVATE]
```slithir
_fAssets_1(uint256) := phi(['_fAssets_1', '_fAssets_1'])
_recipient_1(address) := phi(['msg.sender', '_recipient_1'])
 require(bool,error)(_fAssets > 0,revert WithdrawZeroFAsset()())
TMP_6180(bool) = _fAssets_1 > 0
TMP_6181(None) = SOLIDITY_CALL revert WithdrawZeroFAsset()()
TMP_6182(None) = SOLIDITY_CALL require(bool,error)(TMP_6180,TMP_6181)
 freeFAssetFeeShare = _fAssetFeesOf(msg.sender)
TMP_6183(uint256) = INTERNAL_CALL, CollateralPool._fAssetFeesOf(address)(msg.sender)
freeFAssetFeeShare_1(uint256) := TMP_6183(uint256)
 require(bool,error)(_fAssets <= freeFAssetFeeShare,revert FreeFAssetBalanceTooSmall()())
TMP_6184(bool) = _fAssets_1 <= freeFAssetFeeShare_1
TMP_6185(None) = SOLIDITY_CALL revert FreeFAssetBalanceTooSmall()()
TMP_6186(None) = SOLIDITY_CALL require(bool,error)(TMP_6184,TMP_6185)
 _createFAssetFeeDebt(msg.sender,_fAssets)
INTERNAL_CALL, CollateralPool._createFAssetFeeDebt(address,uint256)(msg.sender,_fAssets_1)
 _transferFAssetTo(_recipient,_fAssets)
INTERNAL_CALL, CollateralPool._transferFAssetTo(address,uint256)(_recipient_1,_fAssets_1)
 CPFeesWithdrawn(msg.sender,_fAssets)
Emit CPFeesWithdrawn(msg.sender,_fAssets_1)
```
#### CollateralPool._withdrawWNatTo(address,uint256) [INTERNAL]
```slithir
_recipient_1(address) := phi(['_recipient_1', '_recipient_1'])
_amount_1(uint256) := phi(['natShare_1', 'natShare_1'])
wNat_5(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_0', 'wNat_25', 'wNat_15', 'wNat_36'])
totalCollateral_34(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_0', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
 _amount > 0
TMP_6309(bool) = _amount_1 > 0
CONDITION TMP_6309
 totalCollateral -= _amount
totalCollateral_35(uint256) = totalCollateral_34 (c)- _amount_1
 internalWithdrawal = true
internalWithdrawal_2(bool) := True(bool)
 wNat.withdraw(_amount)
HIGH_LEVEL_CALL, dest:wNat_5(IWNat), function:withdraw, arguments:['_amount_1']  
wNat_6(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_5', 'wNat_28', 'wNat_43', 'wNat_25', 'wNat_15', 'wNat_36'])
 internalWithdrawal = false
internalWithdrawal_3(bool) := False(bool)
 Transfers.transferNAT(_recipient,_amount)
LIBRARY_CALL, dest:Transfers, function:Transfers.transferNAT(address,uint256), arguments:['_recipient_1', '_amount_1']
```
#### CollateralPool.claimAirdropDistribution(IDistributionToDelegators,uint256) [EXTERNAL]
```slithir
agentVault_46(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_1', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_0', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_45(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_14', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_0', 'assetManager_30'])
wNat_44(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_0', 'wNat_25', 'wNat_15', 'wNat_36'])
totalCollateral_45(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_0', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
 balanceBefore = wNat.balanceOf(address(this))
TMP_6369 = CONVERT this to address
TMP_6370(uint256) = HIGH_LEVEL_CALL, dest:wNat_46(IWNat), function:balanceOf, arguments:['TMP_6369']  
agentVault_49(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_48', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_48(IIAssetManager) := phi(['assetManager_47', 'assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
wNat_47(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_46', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_25', 'wNat_15', 'wNat_36'])
totalCollateral_48(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_47', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
balanceBefore_1(uint256) := TMP_6370(uint256)
 _distribution.claim(address(this),address(address(this)),_month,true)
TMP_6371 = CONVERT this to address
TMP_6372 = CONVERT this to address
TMP_6373 = CONVERT TMP_6372 to address
TMP_6374(uint256) = HIGH_LEVEL_CALL, dest:_distribution_1(IDistributionToDelegators), function:claim, arguments:['TMP_6371', 'TMP_6373', '_month_1', 'True']  
agentVault_50(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_49', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_49(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_48', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
wNat_48(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_47', 'wNat_25', 'wNat_15', 'wNat_36'])
totalCollateral_49(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_48', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
 balanceAfter = wNat.balanceOf(address(this))
TMP_6375 = CONVERT this to address
TMP_6376(uint256) = HIGH_LEVEL_CALL, dest:wNat_48(IWNat), function:balanceOf, arguments:['TMP_6375']  
agentVault_51(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_50', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_50(IIAssetManager) := phi(['assetManager_49', 'assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
wNat_49(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_48', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_25', 'wNat_15', 'wNat_36'])
totalCollateral_50(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_49', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
balanceAfter_1(uint256) := TMP_6376(uint256)
 claimed = balanceAfter - balanceBefore
TMP_6377(uint256) = balanceAfter_1 (c)- balanceBefore_1
claimed_1(uint256) := TMP_6377(uint256)
 totalCollateral += claimed
totalCollateral_51(uint256) = totalCollateral_50 (c)+ claimed_1
 assetManager.updateCollateral(agentVault,wNat)
HIGH_LEVEL_CALL, dest:assetManager_50(IIAssetManager), function:updateCollateral, arguments:['agentVault_51', 'wNat_49']  
agentVault_52(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_51', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_51(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_50', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
wNat_50(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_49', 'wNat_25', 'wNat_15', 'wNat_36'])
 CPClaimedReward(claimed,0)
Emit CPClaimedReward(claimed_1,0)
 claimed
RETURN claimed_1
 onlyAgent()
MODIFIER_CALL, CollateralPool.onlyAgent()()
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### CollateralPool.claimDelegationRewards(IRewardManager,uint24,RewardsV2Interface.RewardClaimWithProof[]) [EXTERNAL]
```slithir
agentVault_39(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_1', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_0', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_38(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_14', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_0', 'assetManager_30'])
wNat_37(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_0', 'wNat_25', 'wNat_15', 'wNat_36'])
totalCollateral_38(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_0', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
 balanceBefore = wNat.balanceOf(address(this))
TMP_6356 = CONVERT this to address
TMP_6357(uint256) = HIGH_LEVEL_CALL, dest:wNat_39(IWNat), function:balanceOf, arguments:['TMP_6356']  
agentVault_42(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_41', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_41(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_40', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
wNat_40(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_39', 'wNat_28', 'wNat_43', 'wNat_25', 'wNat_15', 'wNat_36'])
totalCollateral_41(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_35', 'totalCollateral_40', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
balanceBefore_1(uint256) := TMP_6357(uint256)
 _rewardManager.claim(address(this),address(address(this)),_lastRewardEpoch,true,_proofs)
TMP_6358 = CONVERT this to address
TMP_6359 = CONVERT this to address
TMP_6360 = CONVERT TMP_6359 to address
TMP_6361(uint256) = HIGH_LEVEL_CALL, dest:_rewardManager_1(IRewardManager), function:claim, arguments:['TMP_6358', 'TMP_6360', '_lastRewardEpoch_1', 'True', '_proofs_1']  
agentVault_43(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_42', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_42(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_1', 'assetManager_41', 'assetManager_11', 'assetManager_15', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
wNat_41(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_40', 'wNat_25', 'wNat_15', 'wNat_36'])
totalCollateral_42(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_41', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
 balanceAfter = wNat.balanceOf(address(this))
TMP_6362 = CONVERT this to address
TMP_6363(uint256) = HIGH_LEVEL_CALL, dest:wNat_41(IWNat), function:balanceOf, arguments:['TMP_6362']  
agentVault_44(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_43', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_43(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_42', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
wNat_42(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_41', 'wNat_28', 'wNat_43', 'wNat_25', 'wNat_15', 'wNat_36'])
totalCollateral_43(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_35', 'totalCollateral_42', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
balanceAfter_1(uint256) := TMP_6363(uint256)
 claimed = balanceAfter - balanceBefore
TMP_6364(uint256) = balanceAfter_1 (c)- balanceBefore_1
claimed_1(uint256) := TMP_6364(uint256)
 totalCollateral += claimed
totalCollateral_44(uint256) = totalCollateral_43 (c)+ claimed_1
 assetManager.updateCollateral(agentVault,wNat)
HIGH_LEVEL_CALL, dest:assetManager_43(IIAssetManager), function:updateCollateral, arguments:['agentVault_44', 'wNat_42']  
agentVault_45(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_44', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_44(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_1', 'assetManager_43', 'assetManager_11', 'assetManager_15', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
wNat_43(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_42', 'wNat_25', 'wNat_15', 'wNat_36'])
 CPClaimedReward(claimed,1)
Emit CPClaimedReward(claimed_1,1)
 claimed
RETURN claimed_1
 onlyAgent()
MODIFIER_CALL, CollateralPool.onlyAgent()()
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### CollateralPool.constructor(address,address,address,uint32) [PUBLIC]
```slithir
 initialize(_agentVault,_assetManager,_fAsset,_exitCollateralRatioBIPS)
INTERNAL_CALL, CollateralPool.initialize(address,address,address,uint32)(_agentVault_1,_assetManager_1,_fAsset_1,_exitCollateralRatioBIPS_1)
```
#### CollateralPool.debtFreeTokensOf(address) [EXTERNAL]
```slithir
 _debtFreeTokensOf(_account)
TMP_6320(uint256) = INTERNAL_CALL, CollateralPool._debtFreeTokensOf(address)(_account_1)
RETURN TMP_6320
```
#### CollateralPool.debtLockedTokensOf(address) [EXTERNAL]
```slithir
token_62(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_0', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
 MathUtils.subOrZero(token.balanceOf(_account),_debtFreeTokensOf(_account))
TMP_6317(uint256) = HIGH_LEVEL_CALL, dest:token_62(IICollateralPoolToken), function:balanceOf, arguments:['_account_1']  
token_63(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_57', 'token_62', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
TMP_6318(uint256) = INTERNAL_CALL, CollateralPool._debtFreeTokensOf(address)(_account_1)
token_64(IICollateralPoolToken) := phi(['token_56', 'token_58', 'token_55'])
TMP_6319(uint256) = LIBRARY_CALL, dest:MathUtils, function:MathUtils.subOrZero(uint256,uint256), arguments:['TMP_6317', 'TMP_6318'] 
RETURN TMP_6319
```
#### CollateralPool.delegate(address,uint256) [EXTERNAL]
```slithir
wNat_23(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_0', 'wNat_25', 'wNat_15', 'wNat_36'])
 wNat.delegate(_to,_bips)
HIGH_LEVEL_CALL, dest:wNat_24(IWNat), function:delegate, arguments:['_to_1', '_bips_1']  
wNat_25(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_24', 'wNat_28', 'wNat_43', 'wNat_25', 'wNat_15', 'wNat_36'])
 onlyAgent()
MODIFIER_CALL, CollateralPool.onlyAgent()()
```
#### CollateralPool.delegateGovernance(address) [EXTERNAL]
```slithir
wNat_29(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_0', 'wNat_25', 'wNat_15', 'wNat_36'])
 wNat.governanceVotePower().delegate(_to)
TMP_6350(IGovernanceVotePower) = HIGH_LEVEL_CALL, dest:wNat_30(IWNat), function:governanceVotePower, arguments:[]  
wNat_31(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_30', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_25', 'wNat_15', 'wNat_36'])
HIGH_LEVEL_CALL, dest:TMP_6350(IGovernanceVotePower), function:delegate, arguments:['_to_1']  
wNat_32(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_31', 'wNat_25', 'wNat_15', 'wNat_36'])
 onlyAgent()
MODIFIER_CALL, CollateralPool.onlyAgent()()
```
#### CollateralPool.depositNat() [EXTERNAL]
```slithir
 _depositWNat()
INTERNAL_CALL, CollateralPool._depositWNat()()
 onlyAssetManager()
MODIFIER_CALL, CollateralPool.onlyAssetManager()()
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### CollateralPool.destroy(address) [EXTERNAL]
```slithir
fAsset_16(IFAsset) := phi(['fAsset_13', 'fAsset_0', 'fAsset_10', 'fAsset_21', 'fAsset_1'])
token_65(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_0', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
wNat_10(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_0', 'wNat_25', 'wNat_15', 'wNat_36'])
 require(bool,error)(token.totalSupply() == 0,revert CannotDestroyPoolWithIssuedTokens()())
TMP_6321(uint256) = HIGH_LEVEL_CALL, dest:token_67(IICollateralPoolToken), function:totalSupply, arguments:[]  
fAsset_19(IFAsset) := phi(['fAsset_18', 'fAsset_13', 'fAsset_10', 'fAsset_21', 'fAsset_1'])
token_68(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_57', 'token_22', 'token_67', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
wNat_13(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_25', 'wNat_12', 'wNat_15', 'wNat_36'])
TMP_6322(bool) = TMP_6321 == 0
TMP_6323(None) = SOLIDITY_CALL revert CannotDestroyPoolWithIssuedTokens()()
TMP_6324(None) = SOLIDITY_CALL require(bool,error)(TMP_6322,TMP_6323)
 Transfers.depositWNat(wNat,_recipient,address(this).balance)
TMP_6325 = CONVERT this to address
TMP_6326(uint256) = SOLIDITY_CALL balance(address)(TMP_6325)
LIBRARY_CALL, dest:Transfers, function:Transfers.depositWNat(IWNat,address,uint256), arguments:['wNat_13', '_recipient_1', 'TMP_6326'] 
 untrackedWNat = wNat.balanceOf(address(this))
TMP_6328 = CONVERT this to address
TMP_6329(uint256) = HIGH_LEVEL_CALL, dest:wNat_13(IWNat), function:balanceOf, arguments:['TMP_6328']  
fAsset_20(IFAsset) := phi(['fAsset_19', 'fAsset_13', 'fAsset_10', 'fAsset_21', 'fAsset_1'])
wNat_14(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_13', 'wNat_25', 'wNat_15', 'wNat_36'])
untrackedWNat_1(uint256) := TMP_6329(uint256)
 untrackedFAsset = fAsset.balanceOf(address(this))
TMP_6330 = CONVERT this to address
TMP_6331(uint256) = HIGH_LEVEL_CALL, dest:fAsset_20(IFAsset), function:balanceOf, arguments:['TMP_6330']  
fAsset_21(IFAsset) := phi(['fAsset_13', 'fAsset_10', 'fAsset_21', 'fAsset_20', 'fAsset_1'])
wNat_15(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_25', 'wNat_14', 'wNat_15', 'wNat_36'])
untrackedFAsset_1(uint256) := TMP_6331(uint256)
 untrackedWNat > 0
TMP_6332(bool) = untrackedWNat_1 > 0
CONDITION TMP_6332
 wNat.safeTransfer(_recipient,untrackedWNat)
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['wNat_15', '_recipient_1', 'untrackedWNat_1'] 
 untrackedFAsset > 0
TMP_6334(bool) = untrackedFAsset_1 > 0
CONDITION TMP_6334
 fAsset.safeTransfer(_recipient,untrackedFAsset)
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['fAsset_21', '_recipient_1', 'untrackedFAsset_1'] 
 onlyAssetManager()
MODIFIER_CALL, CollateralPool.onlyAssetManager()()
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### CollateralPool.enter() [EXTERNAL]
```slithir
MIN_NAT_TO_ENTER_1(uint256) := phi(['MIN_NAT_TO_ENTER_0', 'MIN_NAT_TO_ENTER_2'])
token_5(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_0', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
totalFAssetFees_1(uint256) := phi(['totalFAssetFees_3', 'totalFAssetFees_0', 'totalFAssetFees_14', 'totalFAssetFees_12', 'totalFAssetFees_6', 'totalFAssetFees_4', 'totalFAssetFees_10'])
totalCollateral_1(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_0', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
 require(bool,error)(msg.value >= MIN_NAT_TO_ENTER,revert AmountOfNatTooLow()())
TMP_6066(bool) = msg.value >= MIN_NAT_TO_ENTER_2
TMP_6067(None) = SOLIDITY_CALL revert AmountOfNatTooLow()()
TMP_6068(None) = SOLIDITY_CALL require(bool,error)(TMP_6066,TMP_6067)
 totalPoolTokens = token.totalSupply()
TMP_6069(uint256) = HIGH_LEVEL_CALL, dest:token_6(IICollateralPoolToken), function:totalSupply, arguments:[]  
token_7(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_57', 'token_6', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
totalFAssetFees_3(uint256) := phi(['totalFAssetFees_3', 'totalFAssetFees_14', 'totalFAssetFees_2', 'totalFAssetFees_12', 'totalFAssetFees_6', 'totalFAssetFees_4', 'totalFAssetFees_10'])
totalCollateral_3(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_2', 'totalCollateral_11', 'totalCollateral_30'])
totalPoolTokens_1(uint256) := TMP_6069(uint256)
 totalPoolTokens == 0
TMP_6070(bool) = totalPoolTokens_1 == 0
CONDITION TMP_6070
 require(bool,error)(msg.value >= totalCollateral,revert AmountOfCollateralTooLow()())
TMP_6071(bool) = msg.value >= totalCollateral_3
TMP_6072(None) = SOLIDITY_CALL revert AmountOfCollateralTooLow()()
TMP_6073(None) = SOLIDITY_CALL require(bool,error)(TMP_6071,TMP_6072)
 assetPrice = _getAssetPrice()
TMP_6074(CollateralPool.AssetPrice) = INTERNAL_CALL, CollateralPool._getAssetPrice()()
assetPrice_1(CollateralPool.AssetPrice) := TMP_6074(CollateralPool.AssetPrice)
 require(bool,error)(msg.value >= totalFAssetFees.mulDiv(assetPrice.mul,assetPrice.div),revert AmountOfCollateralTooLow()())
REF_4185(uint256) -> assetPrice_1.mul
REF_4186(uint256) -> assetPrice_1.div
TMP_6075(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['totalFAssetFees_4', 'REF_4185', 'REF_4186'] 
TMP_6076(bool) = msg.value >= TMP_6075
TMP_6077(None) = SOLIDITY_CALL revert AmountOfCollateralTooLow()()
TMP_6078(None) = SOLIDITY_CALL require(bool,error)(TMP_6076,TMP_6077)
 tokenShare = _collateralToTokenShare(msg.value)
TMP_6079(uint256) = INTERNAL_CALL, CollateralPool._collateralToTokenShare(uint256)(msg.value)
token_9(IICollateralPoolToken) := phi(['token_49'])
tokenShare_1(uint256) := TMP_6079(uint256)
 require(bool,error)(tokenShare > 0,revert DepositResultsInZeroTokens()())
TMP_6080(bool) = tokenShare_1 > 0
TMP_6081(None) = SOLIDITY_CALL revert DepositResultsInZeroTokens()()
TMP_6082(None) = SOLIDITY_CALL require(bool,error)(TMP_6080,TMP_6081)
 _createFAssetFeeDebt(msg.sender,feeDebt)
INTERNAL_CALL, CollateralPool._createFAssetFeeDebt(address,uint256)(msg.sender,feeDebt_3)
 _depositWNat()
INTERNAL_CALL, CollateralPool._depositWNat()()
 timelockExp = token.mint(msg.sender,tokenShare)
TMP_6085(uint256) = HIGH_LEVEL_CALL, dest:token_12(IICollateralPoolToken), function:mint, arguments:['msg.sender', 'tokenShare_1']  
token_13(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_57', 'token_22', 'token_12', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
timelockExp_1(uint256) := TMP_6085(uint256)
 CPEntered(msg.sender,msg.value,tokenShare,timelockExp)
Emit CPEntered(msg.sender,msg.value,tokenShare_1,timelockExp_1)
 (tokenShare,timelockExp)
RETURN tokenShare_1,timelockExp_1
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
 totalPoolTokens > 0
TMP_6088(bool) = totalPoolTokens_1 > 0
CONDITION TMP_6088
 feeDebt = _totalVirtualFees().mulDiv(tokenShare,totalPoolTokens)
TMP_6089(uint256) = INTERNAL_CALL, CollateralPool._totalVirtualFees()()
TMP_6090(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['TMP_6089', 'tokenShare_1', 'totalPoolTokens_1'] 
feeDebt_1(uint256) := TMP_6090(uint256)
 feeDebt = 0
feeDebt_2(uint256) := 0(uint256)
feeDebt_3(uint256) := phi(['feeDebt_1', 'feeDebt_2'])
```
#### CollateralPool.exit(uint256) [EXTERNAL]
```slithir
 _exitTo(_tokenShare,address(msg.sender))
TMP_6091 = CONVERT msg.sender to address
TMP_6092(uint256) = INTERNAL_CALL, CollateralPool._exitTo(uint256,address)(_tokenShare_1,TMP_6091)
RETURN TMP_6092
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### CollateralPool.exitTo(uint256,address) [EXTERNAL]
```slithir
 _exitTo(_tokenShare,_recipient)
TMP_6094(uint256) = INTERNAL_CALL, CollateralPool._exitTo(uint256,address)(_tokenShare_1,_recipient_1)
RETURN TMP_6094
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### CollateralPool.fAssetFeeDebtOf(address) [EXTERNAL]
```slithir
_fAssetFeeDebtOf_10(mapping(address => int256)) := phi(['_fAssetFeeDebtOf_5', '_fAssetFeeDebtOf_0', '_fAssetFeeDebtOf_2', '_fAssetFeeDebtOf_7', '_fAssetFeeDebtOf_4', '_fAssetFeeDebtOf_9', '_fAssetFeeDebtOf_10'])
 _fAssetFeeDebtOf[_account]
REF_4260(int256) -> _fAssetFeeDebtOf_10[_account_1]
RETURN REF_4260
```
#### CollateralPool.fAssetFeeDeposited(uint256) [EXTERNAL]
```slithir
totalFAssetFees_8(uint256) := phi(['totalFAssetFees_3', 'totalFAssetFees_0', 'totalFAssetFees_14', 'totalFAssetFees_12', 'totalFAssetFees_6', 'totalFAssetFees_4', 'totalFAssetFees_10'])
 totalFAssetFees += _amount
totalFAssetFees_10(uint256) = totalFAssetFees_9 (c)+ _amount_1
 onlyAssetManager()
MODIFIER_CALL, CollateralPool.onlyAssetManager()()
```
#### CollateralPool.fAssetFeesOf(address) [EXTERNAL]
```slithir
 _fAssetFeesOf(_account)
TMP_6316(uint256) = INTERNAL_CALL, CollateralPool._fAssetFeesOf(address)(_account_1)
RETURN TMP_6316
```
#### CollateralPool.fAssetRequiredForSelfCloseExit(uint256) [EXTERNAL]
```slithir
token_38(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_0', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
totalCollateral_12(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_0', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
 tokenNatWeiEquiv = totalCollateral.mulDiv(_tokenAmountWei,token.totalSupply())
TMP_6173(uint256) = HIGH_LEVEL_CALL, dest:token_38(IICollateralPoolToken), function:totalSupply, arguments:[]  
token_39(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_38', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
totalCollateral_13(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_12', 'totalCollateral_17', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
TMP_6174(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDiv(uint256,uint256,uint256), arguments:['totalCollateral_13', '_tokenAmountWei_1', 'TMP_6173'] 
tokenNatWeiEquiv_1(uint256) := TMP_6174(uint256)
 _getFAssetRequiredToNotSpoilCR(tokenNatWeiEquiv)
TMP_6175(uint256) = INTERNAL_CALL, CollateralPool._getFAssetRequiredToNotSpoilCR(uint256)(tokenNatWeiEquiv_1)
RETURN TMP_6175
```
#### CollateralPool.implementation() [EXTERNAL]
```slithir
 _getImplementation()
TMP_6385(address) = INTERNAL_CALL, ERC1967Upgrade._getImplementation()()
RETURN TMP_6385
```
#### CollateralPool.initialize(address,address,address,uint32) [PUBLIC]
```slithir
_agentVault_1(address) := phi(['_agentVault_1'])
_assetManager_1(address) := phi(['_assetManager_1'])
_fAsset_1(address) := phi(['_fAsset_1'])
_exitCollateralRatioBIPS_1(uint32) := phi(['_exitCollateralRatioBIPS_1'])
initialized_1(bool) := phi(['initialized_2', 'initialized_0'])
 require(bool,error)(! initialized,revert AlreadyInitialized()())
TMP_6043 = UnaryType.BANG initialized_1 
TMP_6044(None) = SOLIDITY_CALL revert AlreadyInitialized()()
TMP_6045(None) = SOLIDITY_CALL require(bool,error)(TMP_6043,TMP_6044)
 initialized = true
initialized_2(bool) := True(bool)
 agentVault = _agentVault
agentVault_1(address) := _agentVault_1(address)
 assetManager = IIAssetManager(_assetManager)
TMP_6046 = CONVERT _assetManager_1 to IIAssetManager
assetManager_1(IIAssetManager) := TMP_6046(IIAssetManager)
 fAsset = IFAsset(_fAsset)
TMP_6047 = CONVERT _fAsset_1 to IFAsset
fAsset_1(IFAsset) := TMP_6047(IFAsset)
 exitCollateralRatioBIPS = _exitCollateralRatioBIPS
exitCollateralRatioBIPS_1(uint32) := _exitCollateralRatioBIPS_1(uint32)
 initializeReentrancyGuard()
INTERNAL_CALL, ReentrancyGuard.initializeReentrancyGuard()()
 address(assetManager) != address(0)
TMP_6049 = CONVERT assetManager_1 to address
TMP_6050 = CONVERT 0 to address
TMP_6051(bool) = TMP_6049 != TMP_6050
CONDITION TMP_6051
 wNat = assetManager.getWNat()
TMP_6052(IWNat) = HIGH_LEVEL_CALL, dest:assetManager_1(IIAssetManager), function:getWNat, arguments:[]  
assetManager_2(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
wNat_2(IWNat) := TMP_6052(IWNat)
 wNat = IWNat(address(0))
TMP_6053 = CONVERT 0 to address
TMP_6054 = CONVERT TMP_6053 to IWNat
wNat_1(IWNat) := TMP_6054(IWNat)
wNat_3(IWNat) := phi(['wNat_1', 'wNat_2'])
```
#### CollateralPool.isAgentVaultOwner(address) [INTERNAL]
```slithir
_address_1(address) := phi(['msg.sender'])
agentVault_53(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_1', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_0', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_52(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_14', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_0', 'assetManager_30'])
 assetManager.isAgentVaultOwner(agentVault,_address)
TMP_6387(bool) = HIGH_LEVEL_CALL, dest:assetManager_52(IIAssetManager), function:isAgentVaultOwner, arguments:['agentVault_53', '_address_1']  
agentVault_54(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_53', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_53(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_1', 'assetManager_11', 'assetManager_52', 'assetManager_15', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
RETURN TMP_6387
```
#### CollateralPool.optOutOfAirdrop(IDistributionToDelegators) [EXTERNAL]
```slithir
 _distribution.optOutOfAirdrop()
HIGH_LEVEL_CALL, dest:_distribution_1(IDistributionToDelegators), function:optOutOfAirdrop, arguments:[]  
 onlyAgent()
MODIFIER_CALL, CollateralPool.onlyAgent()()
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### CollateralPool.payFAssetFeeDebt(uint256) [EXTERNAL]
```slithir
fAsset_11(IFAsset) := phi(['fAsset_13', 'fAsset_0', 'fAsset_10', 'fAsset_21', 'fAsset_1'])
_fAssetFeeDebtOf_1(mapping(address => int256)) := phi(['_fAssetFeeDebtOf_5', '_fAssetFeeDebtOf_0', '_fAssetFeeDebtOf_2', '_fAssetFeeDebtOf_7', '_fAssetFeeDebtOf_4', '_fAssetFeeDebtOf_9', '_fAssetFeeDebtOf_10'])
 require(bool,error)(_fAssets != 0,revert ZeroFAssetDebtPayment()())
TMP_6190(bool) = _fAssets_1 != 0
TMP_6191(None) = SOLIDITY_CALL revert ZeroFAssetDebtPayment()()
TMP_6192(None) = SOLIDITY_CALL require(bool,error)(TMP_6190,TMP_6191)
 require(bool,error)(_fAssets.toInt256() <= _fAssetFeeDebtOf[msg.sender],revert PaymentLargerThanFeeDebt()())
TMP_6193(int256) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toInt256(uint256), arguments:['_fAssets_1'] 
REF_4208(int256) -> _fAssetFeeDebtOf_2[msg.sender]
TMP_6194(bool) = TMP_6193 <= REF_4208
TMP_6195(None) = SOLIDITY_CALL revert PaymentLargerThanFeeDebt()()
TMP_6196(None) = SOLIDITY_CALL require(bool,error)(TMP_6194,TMP_6195)
 require(bool,error)(fAsset.allowance(msg.sender,address(this)) >= _fAssets,revert FAssetAllowanceTooSmall()())
TMP_6197 = CONVERT this to address
TMP_6198(uint256) = HIGH_LEVEL_CALL, dest:fAsset_12(IFAsset), function:allowance, arguments:['msg.sender', 'TMP_6197']  
fAsset_13(IFAsset) := phi(['fAsset_12', 'fAsset_13', 'fAsset_10', 'fAsset_21', 'fAsset_1'])
TMP_6199(bool) = TMP_6198 >= _fAssets_1
TMP_6200(None) = SOLIDITY_CALL revert FAssetAllowanceTooSmall()()
TMP_6201(None) = SOLIDITY_CALL require(bool,error)(TMP_6199,TMP_6200)
 _deleteFAssetFeeDebt(msg.sender,_fAssets)
INTERNAL_CALL, CollateralPool._deleteFAssetFeeDebt(address,uint256)(msg.sender,_fAssets_1)
 _transferFAssetFrom(msg.sender,_fAssets)
INTERNAL_CALL, CollateralPool._transferFAssetFrom(address,uint256)(msg.sender,_fAssets_1)
 CPFeeDebtPaid(msg.sender,_fAssets)
Emit CPFeeDebtPaid(msg.sender,_fAssets_1)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### CollateralPool.payout(address,uint256,uint256) [EXTERNAL]
```slithir
agentVault_17(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_1', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_0', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
token_40(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_0', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
totalCollateral_14(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_0', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
 agentTokenBalance = token.balanceOf(agentVault)
TMP_6206(uint256) = HIGH_LEVEL_CALL, dest:token_42(IICollateralPoolToken), function:balanceOf, arguments:['agentVault_19']  
agentVault_20(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_19', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
token_43(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_42', 'token_49', 'token_68', 'token_59', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
totalCollateral_17(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_35', 'totalCollateral_16', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
agentTokenBalance_1(uint256) := TMP_6206(uint256)
 slashedTokens = Math.min(maxSlashedTokens,agentTokenBalance)
TMP_6207(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['maxSlashedTokens_3', 'agentTokenBalance_1'] 
slashedTokens_1(uint256) := TMP_6207(uint256)
 slashedTokens > 0
TMP_6208(bool) = slashedTokens_1 > 0
CONDITION TMP_6208
 debtFAssetFeeShare = _tokensToVirtualFeeShare(slashedTokens)
TMP_6209(uint256) = INTERNAL_CALL, CollateralPool._tokensToVirtualFeeShare(uint256)(slashedTokens_1)
token_45(IICollateralPoolToken) := phi(['token_51'])
debtFAssetFeeShare_1(uint256) := TMP_6209(uint256)
 _deleteFAssetFeeDebt(agentVault,debtFAssetFeeShare)
INTERNAL_CALL, CollateralPool._deleteFAssetFeeDebt(address,uint256)(agentVault_22,debtFAssetFeeShare_1)
 token.burn(agentVault,slashedTokens,true)
HIGH_LEVEL_CALL, dest:token_46(IICollateralPoolToken), function:burn, arguments:['agentVault_23', 'slashedTokens_1', 'True']  
agentVault_24(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_23', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
token_47(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_46', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
 _transferWNatTo(_recipient,_amount)
INTERNAL_CALL, CollateralPool._transferWNatTo(address,uint256)(_recipient_1,_amount_1)
 CPPaidOut(_recipient,_amount,slashedTokens)
Emit CPPaidOut(_recipient_1,_amount_1,slashedTokens_1)
 onlyAssetManager()
MODIFIER_CALL, CollateralPool.onlyAssetManager()()
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
 totalCollateral > 0
TMP_6216(bool) = totalCollateral_17 > 0
CONDITION TMP_6216
 maxSlashedTokens = token.totalSupply().mulDivRoundUp(_agentResponsibilityWei,totalCollateral)
TMP_6217(uint256) = HIGH_LEVEL_CALL, dest:token_43(IICollateralPoolToken), function:totalSupply, arguments:[]  
agentVault_21(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
token_44(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
totalCollateral_18(uint256) := phi(['totalCollateral_18', 'totalCollateral_13', 'totalCollateral_20', 'totalCollateral_17', 'totalCollateral_35', 'totalCollateral_3', 'totalCollateral_7', 'totalCollateral_51', 'totalCollateral_33', 'totalCollateral_37', 'totalCollateral_44', 'totalCollateral_25', 'totalCollateral_11', 'totalCollateral_30'])
TMP_6218(uint256) = LIBRARY_CALL, dest:SafePct, function:SafePct.mulDivRoundUp(uint256,uint256,uint256), arguments:['TMP_6217', '_agentResponsibilityWei_1', 'totalCollateral_18'] 
maxSlashedTokens_1(uint256) := TMP_6218(uint256)
 maxSlashedTokens = agentTokenBalance
maxSlashedTokens_2(uint256) := agentTokenBalance_1(uint256)
maxSlashedTokens_3(uint256) := phi(['maxSlashedTokens_1', 'maxSlashedTokens_2'])
```
#### CollateralPool.poolToken() [EXTERNAL]
```slithir
token_4(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_0', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
 token
RETURN token_4
```
#### CollateralPool.receive() [EXTERNAL]
```slithir
internalWithdrawal_1(bool) := phi(['internalWithdrawal_5', 'internalWithdrawal_3', 'internalWithdrawal_0'])
 require(bool,error)(internalWithdrawal,revert OnlyInternalUse()())
TMP_6055(None) = SOLIDITY_CALL revert OnlyInternalUse()()
TMP_6056(None) = SOLIDITY_CALL require(bool,error)(internalWithdrawal_1,TMP_6055)
```
#### CollateralPool.selfCloseExit(uint256,bool,string,address) [EXTERNAL]
```slithir
 _selfCloseExitTo(_tokenShare,_redeemToCollateral,address(msg.sender),_redeemerUnderlyingAddress,_executor)
TMP_6118 = CONVERT msg.sender to address
INTERNAL_CALL, CollateralPool._selfCloseExitTo(uint256,bool,address,string,address)(_tokenShare_1,_redeemToCollateral_1,TMP_6118,_redeemerUnderlyingAddress_1,_executor_1)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### CollateralPool.selfCloseExitTo(uint256,bool,address,string,address) [EXTERNAL]
```slithir
agentVault_2(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_1', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_0', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
 require(bool,error)(_recipient != address(0) && _recipient != address(this) && _recipient != agentVault,revert InvalidRecipientAddress()())
TMP_6121 = CONVERT 0 to address
TMP_6122(bool) = _recipient_1 != TMP_6121
TMP_6123 = CONVERT this to address
TMP_6124(bool) = _recipient_1 != TMP_6123
TMP_6125(bool) = TMP_6122 && TMP_6124
TMP_6126(bool) = _recipient_1 != agentVault_3
TMP_6127(bool) = TMP_6125 && TMP_6126
TMP_6128(None) = SOLIDITY_CALL revert InvalidRecipientAddress()()
TMP_6129(None) = SOLIDITY_CALL require(bool,error)(TMP_6127,TMP_6128)
 _selfCloseExitTo(_tokenShare,_redeemToCollateral,_recipient,_redeemerUnderlyingAddress,_executor)
INTERNAL_CALL, CollateralPool._selfCloseExitTo(uint256,bool,address,string,address)(_tokenShare_1,_redeemToCollateral_1,_recipient_1,_redeemerUnderlyingAddress_1,_executor_1)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### CollateralPool.setExitCollateralRatioBIPS(uint256) [EXTERNAL]
```slithir
 exitCollateralRatioBIPS = _exitCollateralRatioBIPS.toUint32()
TMP_6064(uint32) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint32(uint256), arguments:['_exitCollateralRatioBIPS_1'] 
exitCollateralRatioBIPS_2(uint32) := TMP_6064(uint32)
 onlyAssetManager()
MODIFIER_CALL, CollateralPool.onlyAssetManager()()
```
#### CollateralPool.setPoolToken(address) [EXTERNAL]
```slithir
token_1(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_0', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
 require(bool,error)(address(token) == address(0),revert PoolTokenAlreadySet()())
TMP_6057 = CONVERT token_2 to address
TMP_6058 = CONVERT 0 to address
TMP_6059(bool) = TMP_6057 == TMP_6058
TMP_6060(None) = SOLIDITY_CALL revert PoolTokenAlreadySet()()
TMP_6061(None) = SOLIDITY_CALL require(bool,error)(TMP_6059,TMP_6060)
 token = IICollateralPoolToken(_poolToken)
TMP_6062 = CONVERT _poolToken_1 to IICollateralPoolToken
token_3(IICollateralPoolToken) := TMP_6062(IICollateralPoolToken)
 onlyAssetManager()
MODIFIER_CALL, CollateralPool.onlyAssetManager()()
```

#### CollateralPool.supportsInterface(bytes4) [EXTERNAL]
```slithir
 _interfaceId == type()(IERC165).interfaceId || _interfaceId == type()(ICollateralPool).interfaceId || _interfaceId == type()(IICollateralPool).interfaceId
TMP_6388(type(IERC165)) = SOLIDITY_CALL type()(IERC165)
REF_4289(bytes4) (->None) := 33540519(bytes4)
TMP_6389(bool) = _interfaceId_1 == REF_4289
TMP_6390(type(ICollateralPool)) = SOLIDITY_CALL type()(ICollateralPool)
REF_4290(bytes4) (->None) := 2514868256(bytes4)
TMP_6391(bool) = _interfaceId_1 == REF_4290
TMP_6392(bool) = TMP_6389 || TMP_6391
TMP_6393(type(IICollateralPool)) = SOLIDITY_CALL type()(IICollateralPool)
REF_4291(bytes4) (->None) := 4215516495(bytes4)
TMP_6394(bool) = _interfaceId_1 == REF_4291
TMP_6395(bool) = TMP_6392 || TMP_6394
RETURN TMP_6395
```
#### CollateralPool.undelegateAll() [EXTERNAL]
```slithir
wNat_26(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_0', 'wNat_25', 'wNat_15', 'wNat_36'])
 wNat.undelegateAll()
HIGH_LEVEL_CALL, dest:wNat_27(IWNat), function:undelegateAll, arguments:[]  
wNat_28(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_27', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_25', 'wNat_15', 'wNat_36'])
 onlyAgent()
MODIFIER_CALL, CollateralPool.onlyAgent()()
```
#### CollateralPool.undelegateGovernance() [EXTERNAL]
```slithir
wNat_33(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_0', 'wNat_25', 'wNat_15', 'wNat_36'])
 wNat.governanceVotePower().undelegate()
TMP_6353(IGovernanceVotePower) = HIGH_LEVEL_CALL, dest:wNat_34(IWNat), function:governanceVotePower, arguments:[]  
wNat_35(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_34', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_25', 'wNat_15', 'wNat_36'])
HIGH_LEVEL_CALL, dest:TMP_6353(IGovernanceVotePower), function:undelegate, arguments:[]  
wNat_36(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_35', 'wNat_25', 'wNat_15', 'wNat_36'])
 onlyAgent()
MODIFIER_CALL, CollateralPool.onlyAgent()()
```
#### CollateralPool.upgradeWNatContract(IWNat) [EXTERNAL]
```slithir
agentVault_32(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_1', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_0', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_31(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_14', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_0', 'assetManager_30'])
wNat_16(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_0', 'wNat_25', 'wNat_15', 'wNat_36'])
 _newWNat == wNat
TMP_6338(bool) = _newWNat_1 == wNat_18
CONDITION TMP_6338
 balance = wNat.balanceOf(address(this))
TMP_6339 = CONVERT this to address
TMP_6340(uint256) = HIGH_LEVEL_CALL, dest:wNat_18(IWNat), function:balanceOf, arguments:['TMP_6339']  
agentVault_35(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_34(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
wNat_19(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_25', 'wNat_15', 'wNat_36'])
balance_1(uint256) := TMP_6340(uint256)
 internalWithdrawal = true
internalWithdrawal_4(bool) := True(bool)
 wNat.withdraw(balance)
HIGH_LEVEL_CALL, dest:wNat_19(IWNat), function:withdraw, arguments:['balance_1']  
agentVault_36(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_35', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_35(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_23', 'assetManager_34', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
wNat_20(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_25', 'wNat_15', 'wNat_36', 'wNat_19'])
 internalWithdrawal = false
internalWithdrawal_5(bool) := False(bool)
 _newWNat.deposit{value: balance}()
HIGH_LEVEL_CALL, dest:_newWNat_1(IWNat), function:deposit, arguments:[] value:balance_1 
agentVault_37(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_1', 'agentVault_36', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_36(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_35', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
 wNat = _newWNat
wNat_21(IWNat) := _newWNat_1(IWNat)
 assetManager.updateCollateral(agentVault,wNat)
HIGH_LEVEL_CALL, dest:assetManager_36(IIAssetManager), function:updateCollateral, arguments:['agentVault_37', 'wNat_21']  
agentVault_38(address) := phi(['agentVault_45', 'agentVault_28', 'agentVault_12', 'agentVault_20', 'agentVault_16', 'agentVault_3', 'agentVault_26', 'agentVault_54', 'agentVault_21', 'agentVault_31', 'agentVault_15', 'agentVault_34', 'agentVault_37', 'agentVault_1', 'agentVault_52', 'agentVault_38', 'agentVault_14'])
assetManager_37(IIAssetManager) := phi(['assetManager_51', 'assetManager_25', 'assetManager_13', 'assetManager_44', 'assetManager_27', 'assetManager_53', 'assetManager_36', 'assetManager_1', 'assetManager_11', 'assetManager_15', 'assetManager_23', 'assetManager_33', 'assetManager_2', 'assetManager_37', 'assetManager_21', 'assetManager_14', 'assetManager_30'])
wNat_22(IWNat) := phi(['wNat_6', 'wNat_32', 'wNat_9', 'wNat_22', 'wNat_18', 'wNat_3', 'wNat_50', 'wNat_28', 'wNat_43', 'wNat_21', 'wNat_25', 'wNat_15', 'wNat_36'])
 onlyAssetManager()
MODIFIER_CALL, CollateralPool.onlyAssetManager()()
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### CollateralPool.virtualFAssetOf(address) [EXTERNAL]
```slithir
 _virtualFAssetFeesOf(_account)
TMP_6315(uint256) = INTERNAL_CALL, CollateralPool._virtualFAssetFeesOf(address)(_account_1)
RETURN TMP_6315
```
#### CollateralPool.withdrawFees(uint256) [EXTERNAL]
```slithir
 _withdrawFeesTo(_fAssets,msg.sender)
INTERNAL_CALL, CollateralPool._withdrawFeesTo(uint256,address)(_fAssets_1,msg.sender)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### CollateralPool.withdrawFeesTo(uint256,address) [EXTERNAL]
```slithir
 _withdrawFeesTo(_fAssets,_recipient)
INTERNAL_CALL, CollateralPool._withdrawFeesTo(uint256,address)(_fAssets_1,_recipient_1)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### IIAssetManager.getFAssetsBackedByPool(address) [EXTERNAL]
```slithir

```
#### SafePct.mulDiv(uint256,uint256,uint256) [INTERNAL]
```slithir
x_1(uint256) := phi(['x_1', 'x_1'])
y_1(uint256) := phi(['y_1', 'y_1'])
z_1(uint256) := phi(['z_1', 'MAX_BIPS_1'])
 require(bool,error)(z > 0,revert DivisionByZero()())
TMP_10510(bool) = z_1 > 0
TMP_10511(None) = SOLIDITY_CALL revert DivisionByZero()()
TMP_10512(None) = SOLIDITY_CALL require(bool,error)(TMP_10510,TMP_10511)
 x == 0
TMP_10513(bool) = x_1 == 0
CONDITION TMP_10513
 0
RETURN 0
 xy = x * y
TMP_10514(uint256) = x_1 * y_1
xy_1(uint256) := TMP_10514(uint256)
 xy / x == y
TMP_10515(uint256) = xy_1 / x_1
TMP_10516(bool) = TMP_10515 == y_1
CONDITION TMP_10516
 xy / z
TMP_10517(uint256) = xy_1 / z_1
RETURN TMP_10517
 a = x / z
TMP_10518(uint256) = x_1 (c)/ z_1
a_1(uint256) := TMP_10518(uint256)
 b = x % z
TMP_10519(uint256) = x_1 % z_1
b_1(uint256) := TMP_10519(uint256)
 c = y / z
TMP_10520(uint256) = y_1 (c)/ z_1
c_1(uint256) := TMP_10520(uint256)
 d = y % z
TMP_10521(uint256) = y_1 % z_1
d_1(uint256) := TMP_10521(uint256)
 (a * c * z) + (a * d) + (b * c) + (b * d / z)
TMP_10522(uint256) = a_1 (c)* c_1
TMP_10523(uint256) = TMP_10522 (c)* z_1
TMP_10524(uint256) = a_1 (c)* d_1
TMP_10525(uint256) = TMP_10523 (c)+ TMP_10524
TMP_10526(uint256) = b_1 (c)* c_1
TMP_10527(uint256) = TMP_10525 (c)+ TMP_10526
TMP_10528(uint256) = b_1 (c)* d_1
TMP_10529(uint256) = TMP_10528 (c)/ z_1
TMP_10530(uint256) = TMP_10527 (c)+ TMP_10529
RETURN TMP_10530
```

#### MathUtils.positivePart(int256) [INTERNAL]
```slithir
 _x >= 0
TMP_10427(bool) = _x_1 >= 0
CONDITION TMP_10427
 uint256(_x)
TMP_10428 = CONVERT _x_1 to uint256
RETURN TMP_10428
 0
RETURN 0
```
#### IIAssetManager.updateCollateral(address,IERC20) [EXTERNAL]
```slithir

```
#### IWNat.deposit() [EXTERNAL]
```slithir

```

#### Math.min(uint256,uint256) [INTERNAL]
```slithir
a_1(uint256) := phi(['result_8'])
b_1(uint256) := phi(['TMP_554'])
 a < b
TMP_477(bool) = a_1 < b_1
CONDITION TMP_477
 a
RETURN a_1
 b
RETURN b_1
```
#### IIAssetManager.assetPriceNatWei() [EXTERNAL]
```slithir

```
#### MathUtils.roundUp(uint256,uint256) [INTERNAL]
```slithir
 remainder = x % rounding
TMP_10421(uint256) = x_1 % rounding_1
remainder_1(uint256) := TMP_10421(uint256)
 remainder == 0
TMP_10422(bool) = remainder_1 == 0
CONDITION TMP_10422
 x
RETURN x_1
 x - remainder + rounding
TMP_10423(uint256) = x_1 (c)- remainder_1
TMP_10424(uint256) = TMP_10423 (c)+ rounding_1
RETURN TMP_10424
```
#### MathUtils.subOrZero(uint256,uint256) [INTERNAL]
```slithir
 _a > _b
TMP_10425(bool) = _a_1 > _b_1
CONDITION TMP_10425
 _a - _b
TMP_10426(uint256) = _a_1 (c)- _b_1
RETURN TMP_10426
 0
RETURN 0
```
#### SafePct.mulDivRoundUp(uint256,uint256,uint256) [INTERNAL]
```slithir
 resultRoundDown = mulDiv(x,y,z)
TMP_10531(uint256) = INTERNAL_CALL, SafePct.mulDiv(uint256,uint256,uint256)(x_1,y_1,z_1)
resultRoundDown_1(uint256) := TMP_10531(uint256)
 remainder = mulmod(uint256,uint256,uint256)(x,y,z)
TMP_10532(uint256) = SOLIDITY_CALL mulmod(uint256,uint256,uint256)(x_1,y_1,z_1)
remainder_1(uint256) := TMP_10532(uint256)
 remainder == 0
TMP_10533(bool) = remainder_1 == 0
CONDITION TMP_10533
 resultRoundDown
RETURN resultRoundDown_1
 resultRoundDown + 1
TMP_10534(uint256) = resultRoundDown_1 + 1
RETURN TMP_10534
```
#### SafePct.mulBips(uint256,uint256) [INTERNAL]
```slithir
MAX_BIPS_1(uint256) := phi(['MAX_BIPS_2', 'MAX_BIPS_0'])
 mulDiv(x,y,MAX_BIPS)
TMP_10535(uint256) = INTERNAL_CALL, SafePct.mulDiv(uint256,uint256,uint256)(x_1,y_1,MAX_BIPS_1)
RETURN TMP_10535
```
#### Math.max(uint256,uint256) [INTERNAL]
```slithir
 a > b
TMP_476(bool) = a_1 > b_1
CONDITION TMP_476
 a
RETURN a_1
 b
RETURN b_1
```
#### IIAssetManager.maxRedemptionFromAgent(address) [EXTERNAL]
```slithir

```
#### IIAssetManager.redeemFromAgent(address,address,uint256,string,address) [EXTERNAL]
```slithir

```
#### IIAssetManager.redeemFromAgentInCollateral(address,address,uint256) [EXTERNAL]
```slithir

```
#### SafeERC20.safeTransferFrom(IERC20,address,address,uint256) [INTERNAL]
```slithir
 _callOptionalReturn(token,abi.encodeWithSelector(token.transferFrom.selector,from,to,value))
REF_89(bytes4) (->None) := 599290589(bytes4)
TMP_245(bytes) = SOLIDITY_CALL abi.encodeWithSelector()(REF_89,from_1,to_1,value_1)
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_245)
```
#### Transfers.transferNAT(address,uint256) [INTERNAL]
```slithir
TRANSFER_GAS_ALLOWANCE_1(uint256) := phi(['TRANSFER_GAS_ALLOWANCE_0', 'TRANSFER_GAS_ALLOWANCE_3', 'TRANSFER_GAS_ALLOWANCE_2'])
 _amount > 0
TMP_10536(bool) = _amount_1 > 0
CONDITION TMP_10536
 (success,None) = _recipient.call{gas: TRANSFER_GAS_ALLOWANCE,value: _amount}()
TUPLE_91(bool,bytes) = LOW_LEVEL_CALL, dest:_recipient_1, function:call, arguments:[''] value:_amount_1 gas:TRANSFER_GAS_ALLOWANCE_2
TRANSFER_GAS_ALLOWANCE_3(uint256) := phi(['TRANSFER_GAS_ALLOWANCE_3', 'TRANSFER_GAS_ALLOWANCE_2'])
success_1(bool)= UNPACK TUPLE_91 index: 0 
 require(bool,error)(success,revert TransferFailed()())
TMP_10537(None) = SOLIDITY_CALL revert TransferFailed()()
TMP_10538(None) = SOLIDITY_CALL require(bool,error)(success_1,TMP_10537)
 requireReentrancyGuard()
MODIFIER_CALL, Transfers.requireReentrancyGuard()()
```
#### SafeCast.toUint256(int256) [INTERNAL]
```slithir
 require(bool,string)(value >= 0,SafeCast: value must be positive)
TMP_786(bool) = value_1 >= 0
TMP_787(None) = SOLIDITY_CALL require(bool,string)(TMP_786,SafeCast: value must be positive)
 uint256(value)
TMP_788 = CONVERT value_1 to uint256
RETURN TMP_788
```
#### SafeERC20.safeTransfer(IERC20,address,uint256) [INTERNAL]
```slithir
 _callOptionalReturn(token,abi.encodeWithSelector(token.transfer.selector,to,value))
REF_86(bytes4) (->None) := 2835717307(bytes4)
TMP_243(bytes) = SOLIDITY_CALL abi.encodeWithSelector()(REF_86,to_1,value_1)
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_243)
```
#### IWNat.withdraw(uint256) [EXTERNAL]
```slithir

```
#### IDistributionToDelegators.claim(address,address,uint256,bool) [EXTERNAL]
```slithir

```
#### IGovernanceVotePower.delegate(address) [EXTERNAL]
```slithir

```
#### Transfers.depositWNat(IWNat,address,uint256) [INTERNAL]
```slithir
 _amount > 0
TMP_10540(bool) = _amount_1 > 0
CONDITION TMP_10540
 _wNat.depositTo{value: _amount}(_recipient)
HIGH_LEVEL_CALL, dest:_wNat_1(IWNat), function:depositTo, arguments:['_recipient_1'] value:_amount_1
```
#### IICollateralPoolToken.mint(address,uint256) [EXTERNAL]
```slithir

```
#### IIAssetManager.getWNat() [EXTERNAL]
```slithir

```
#### IIAssetManager.isAgentVaultOwner(address,address) [EXTERNAL]
```slithir

```
#### IDistributionToDelegators.optOutOfAirdrop() [EXTERNAL]
```slithir

```
#### SafeCast.toUint32(uint256) [INTERNAL]
```slithir
 require(bool,string)(value <= type()(uint32).max,SafeCast: value doesn't fit in 32 bits)
TMP_767(uint32) := 4294967295(uint32)
TMP_768(bool) = value_1 <= TMP_767
TMP_769(None) = SOLIDITY_CALL require(bool,string)(TMP_768,SafeCast: value doesn't fit in 32 bits)
 uint32(value)
TMP_770 = CONVERT value_1 to uint32
RETURN TMP_770
```
#### IGovernanceVotePower.undelegate() [EXTERNAL]
```slithir

```
#### SafeERC20._callOptionalReturn(IERC20,bytes) [PRIVATE]
```slithir
token_1(IERC20) := phi(['token_1', 'token_1', 'token_1', 'token_1', 'token_1', 'token_1'])
data_1(bytes) := phi(['TMP_243', 'TMP_245', 'TMP_265', 'TMP_253', 'TMP_258', 'approvalCall_1', 'TMP_270'])
 returndata = address(token).functionCall(data,SafeERC20: low-level call failed)
TMP_279 = CONVERT token_1 to address
TMP_280(bytes) = LIBRARY_CALL, dest:Address, function:Address.functionCall(address,bytes,string), arguments:['TMP_279', 'data_1', 'SafeERC20: low-level call failed'] 
returndata_1(bytes) := TMP_280(bytes)
 require(bool,string)(returndata.length == 0 || abi.decode(returndata,(bool)),SafeERC20: ERC20 operation did not succeed)
REF_112 -> LENGTH returndata_1
TMP_281(bool) = REF_112 == 0
TMP_282(bool) = SOLIDITY_CALL abi.decode()(returndata_1,bool)
TMP_283(bool) = TMP_281 || TMP_282
TMP_284(None) = SOLIDITY_CALL require(bool,string)(TMP_283,SafeERC20: ERC20 operation did not succeed)
```
#### IWNat.depositTo(address) [EXTERNAL]
```slithir

```
#### Address.functionCall(address,bytes,string) [INTERNAL]
```slithir
 functionCallWithValue(target,data,0,errorMessage)
TMP_301(bytes) = INTERNAL_CALL, Address.functionCallWithValue(address,bytes,uint256,string)(target_1,data_1,0,errorMessage_1)
RETURN TMP_301
```
