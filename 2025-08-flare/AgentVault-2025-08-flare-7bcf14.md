

### Storage layout (WNatMock) 

```text
governanceVP IGovernanceVotePower
delegations mapping(address => WNatMock.Delegation[])
delegators mapping(address => EnumerableSet.AddressSet)

```
### Storage layout (AssetManagerMock) 

```text
wNat IWNat
fasset IIFAsset
commonOwner address
checkForValidAgentVaultAddress bool
collateralPool address
maxRedemption uint256
fassetsBackedByPool uint256
timelockDuration uint256
assetPriceMul uint256
assetPriceDiv uint256
lotSize uint256
minPoolCollateralRatioBIPS uint256
assetMintingGranularityUBA uint256

```
### Storage layout (ERC20) 

```text
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_totalSupply uint256
_name string
_symbol string

```
### Storage layout (AgentVault) 

```text
assetManager IIAssetManager
initialized bool
__usedTokens IERC20[]
__tokenUseFlags mapping(IERC20 => uint256)
__internalWithdrawal bool
destroyed bool

```



### Storage layout (CollateralPoolToken) 

```text
collateralPool address
tokenName string
tokenSymbol string
timelocksByAccount mapping(address => CollateralPoolToken.TimelockQueue)
ignoreTimelocked bool
initialized bool

```



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

#### AgentVault._authorizeUpgrade(address) [INTERNAL]
```slithir
 onlyAssetManager()
MODIFIER_CALL, AgentVault.onlyAssetManager()()
```
#### AgentVault._validateToken(IERC20) [PRIVATE]
```slithir
_token_1(IERC20) := phi(['_token_1'])
assetManager_29(IIAssetManager) := phi(['assetManager_7', 'assetManager_19', 'assetManager_20', 'assetManager_11', 'assetManager_26', 'assetManager_0', 'assetManager_30', 'assetManager_15', 'assetManager_28', 'assetManager_1', 'assetManager_24'])
 require(bool,error)(assetManager.isVaultCollateralToken(_token),revert UnknownToken()())
TMP_1335(bool) = HIGH_LEVEL_CALL, dest:assetManager_29(IIAssetManager), function:isVaultCollateralToken, arguments:['_token_1']  
assetManager_30(IIAssetManager) := phi(['assetManager_7', 'assetManager_19', 'assetManager_20', 'assetManager_29', 'assetManager_11', 'assetManager_26', 'assetManager_30', 'assetManager_15', 'assetManager_28', 'assetManager_1', 'assetManager_24'])
TMP_1336(None) = SOLIDITY_CALL revert UnknownToken()()
TMP_1337(None) = SOLIDITY_CALL require(bool,error)(TMP_1335,TMP_1336)
```
#### AgentVault.buyCollateralPoolTokens() [EXTERNAL][OWNER]
```slithir
 collateralPool().enter{value: msg.value}()
TMP_1278(ICollateralPool) = INTERNAL_CALL, AgentVault.collateralPool()()
TUPLE_11(uint256,uint256) = HIGH_LEVEL_CALL, dest:TMP_1278(ICollateralPool), function:enter, arguments:[] value:msg.value 
 onlyOwner()
MODIFIER_CALL, AgentVault.onlyOwner()()
```
#### AgentVault.collateralPool() [PUBLIC]
```slithir
assetManager_25(IIAssetManager) := phi(['assetManager_7', 'assetManager_19', 'assetManager_20', 'assetManager_11', 'assetManager_26', 'assetManager_0', 'assetManager_30', 'assetManager_15', 'assetManager_28', 'assetManager_1', 'assetManager_24'])
 ICollateralPool(assetManager.getCollateralPool(address(this)))
TMP_1320 = CONVERT this to address
TMP_1321(address) = HIGH_LEVEL_CALL, dest:assetManager_25(IIAssetManager), function:getCollateralPool, arguments:['TMP_1320']  
assetManager_26(IIAssetManager) := phi(['assetManager_7', 'assetManager_19', 'assetManager_20', 'assetManager_11', 'assetManager_26', 'assetManager_25', 'assetManager_30', 'assetManager_15', 'assetManager_28', 'assetManager_1', 'assetManager_24'])
TMP_1322 = CONVERT TMP_1321 to ICollateralPool
RETURN TMP_1322
```
#### AgentVault.constructor(IIAssetManager) [PUBLIC]
```slithir
 initialize(_assetManager)
INTERNAL_CALL, AgentVault.initialize(IIAssetManager)(_assetManager_1)
```
#### AgentVault.depositCollateral(IERC20,uint256) [EXTERNAL][OWNER]
```slithir
assetManager_8(IIAssetManager) := phi(['assetManager_7', 'assetManager_19', 'assetManager_20', 'assetManager_11', 'assetManager_26', 'assetManager_0', 'assetManager_30', 'assetManager_15', 'assetManager_28', 'assetManager_1', 'assetManager_24'])
 _token.safeTransferFrom(msg.sender,address(this),_amount)
TMP_1289 = CONVERT this to address
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransferFrom(IERC20,address,address,uint256), arguments:['_token_1', 'msg.sender', 'TMP_1289', '_amount_1'] 
 assetManager.updateCollateral(address(this),_token)
TMP_1291 = CONVERT this to address
HIGH_LEVEL_CALL, dest:assetManager_10(IIAssetManager), function:updateCollateral, arguments:['TMP_1291', '_token_1']  
assetManager_11(IIAssetManager) := phi(['assetManager_7', 'assetManager_19', 'assetManager_20', 'assetManager_11', 'assetManager_26', 'assetManager_10', 'assetManager_30', 'assetManager_15', 'assetManager_28', 'assetManager_1', 'assetManager_24'])
 onlyOwner()
MODIFIER_CALL, AgentVault.onlyOwner()()
 onlyKnownToken(_token)
MODIFIER_CALL, AgentVault.onlyKnownToken(IERC20)(_token_1)
```
#### AgentVault.destroy() [EXTERNAL]
```slithir
 destroyed = true
destroyed_8(bool) := True(bool)
 onlyAssetManager()
MODIFIER_CALL, AgentVault.onlyAssetManager()()
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### AgentVault.implementation() [EXTERNAL]
```slithir
 _getImplementation()
TMP_1333(address) = INTERNAL_CALL, ERC1967Upgrade._getImplementation()()
RETURN TMP_1333
```
#### AgentVault.initialize(IIAssetManager) [PUBLIC]
```slithir
_assetManager_1(IIAssetManager) := phi(['_assetManager_1'])
initialized_1(bool) := phi(['initialized_2', 'initialized_0'])
 require(bool,error)(! initialized,revert AlreadyInitialized()())
TMP_1274 = UnaryType.BANG initialized_1 
TMP_1275(None) = SOLIDITY_CALL revert AlreadyInitialized()()
TMP_1276(None) = SOLIDITY_CALL require(bool,error)(TMP_1274,TMP_1275)
 initialized = true
initialized_2(bool) := True(bool)
 assetManager = _assetManager
assetManager_1(IIAssetManager) := _assetManager_1(IIAssetManager)
 initializeReentrancyGuard()
INTERNAL_CALL, ReentrancyGuard.initializeReentrancyGuard()()
```
#### AgentVault.isOwner(address) [PUBLIC]
```slithir
_address_1(address) := phi(['msg.sender'])
assetManager_27(IIAssetManager) := phi(['assetManager_7', 'assetManager_19', 'assetManager_20', 'assetManager_11', 'assetManager_26', 'assetManager_0', 'assetManager_30', 'assetManager_15', 'assetManager_28', 'assetManager_1', 'assetManager_24'])
 assetManager.isAgentVaultOwner(address(this),_address)
TMP_1323 = CONVERT this to address
TMP_1324(bool) = HIGH_LEVEL_CALL, dest:assetManager_27(IIAssetManager), function:isAgentVaultOwner, arguments:['TMP_1323', '_address_1']  
assetManager_28(IIAssetManager) := phi(['assetManager_7', 'assetManager_19', 'assetManager_20', 'assetManager_11', 'assetManager_26', 'assetManager_27', 'assetManager_30', 'assetManager_15', 'assetManager_28', 'assetManager_1', 'assetManager_24'])
RETURN TMP_1324
```
#### AgentVault.payout(IERC20,address,uint256) [EXTERNAL]
```slithir
 _token.safeTransfer(_recipient,_amount)
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['_token_1', '_recipient_1', '_amount_1'] 
 onlyAssetManager()
MODIFIER_CALL, AgentVault.onlyAssetManager()()
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### AgentVault.redeemCollateralPoolTokens(uint256,address) [EXTERNAL][OWNER]
```slithir
assetManager_2(IIAssetManager) := phi(['assetManager_7', 'assetManager_19', 'assetManager_20', 'assetManager_11', 'assetManager_26', 'assetManager_0', 'assetManager_30', 'assetManager_15', 'assetManager_28', 'assetManager_1', 'assetManager_24'])
 pool = collateralPool()
TMP_1283(ICollateralPool) = INTERNAL_CALL, AgentVault.collateralPool()()
assetManager_5(IIAssetManager) := phi(['assetManager_26'])
pool_1(ICollateralPool) := TMP_1283(ICollateralPool)
 assetManager.beforeCollateralWithdrawal(pool.poolToken(),_amount)
TMP_1284(ICollateralPoolToken) = HIGH_LEVEL_CALL, dest:pool_1(ICollateralPool), function:poolToken, arguments:[]  
assetManager_6(IIAssetManager) := phi(['assetManager_7', 'assetManager_5', 'assetManager_19', 'assetManager_20', 'assetManager_11', 'assetManager_26', 'assetManager_30', 'assetManager_15', 'assetManager_28', 'assetManager_1', 'assetManager_24'])
HIGH_LEVEL_CALL, dest:assetManager_6(IIAssetManager), function:beforeCollateralWithdrawal, arguments:['TMP_1284', '_amount_1']  
assetManager_7(IIAssetManager) := phi(['assetManager_7', 'assetManager_19', 'assetManager_20', 'assetManager_11', 'assetManager_26', 'assetManager_30', 'assetManager_15', 'assetManager_28', 'assetManager_6', 'assetManager_1', 'assetManager_24'])
 pool.exitTo(_amount,_recipient)
TMP_1286(uint256) = HIGH_LEVEL_CALL, dest:pool_1(ICollateralPool), function:exitTo, arguments:['_amount_1', '_recipient_1']  
 onlyOwner()
MODIFIER_CALL, AgentVault.onlyOwner()()
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### AgentOwnerRegistryProxy.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 _IMPLEMENTATION_SLOT = 0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc
 _ADMIN_SLOT = 0xb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6103
 _BEACON_SLOT = 0xa3f0ad74e5423aebfd80d3ef4346578335a9a72aeaee59ff6cb3582b35133d50
```
#### AgentVault.supportsInterface(bytes4) [EXTERNAL]
```slithir
 _interfaceId == type()(IERC165).interfaceId || _interfaceId == type()(IAgentVault).interfaceId || _interfaceId == type()(IIAgentVault).interfaceId
TMP_1325(type(IERC165)) = SOLIDITY_CALL type()(IERC165)
REF_399(bytes4) (->None) := 33540519(bytes4)
TMP_1326(bool) = _interfaceId_1 == REF_399
TMP_1327(type(IAgentVault)) = SOLIDITY_CALL type()(IAgentVault)
REF_400(bytes4) (->None) := 2043119941(bytes4)
TMP_1328(bool) = _interfaceId_1 == REF_400
TMP_1329(bool) = TMP_1326 || TMP_1328
TMP_1330(type(IIAgentVault)) = SOLIDITY_CALL type()(IIAgentVault)
REF_401(bytes4) (->None) := 1342129006(bytes4)
TMP_1331(bool) = _interfaceId_1 == REF_401
TMP_1332(bool) = TMP_1329 || TMP_1331
RETURN TMP_1332
```
#### AgentVault.transferExternalToken(IERC20,uint256) [EXTERNAL][OWNER]
```slithir
assetManager_21(IIAssetManager) := phi(['assetManager_7', 'assetManager_19', 'assetManager_20', 'assetManager_11', 'assetManager_26', 'assetManager_0', 'assetManager_30', 'assetManager_15', 'assetManager_28', 'assetManager_1', 'assetManager_24'])
destroyed_5(bool) := phi(['destroyed_8', 'destroyed_4', 'destroyed_7', 'destroyed_0'])
 require(bool,error)(destroyed || ! assetManager.isLockedVaultToken(address(this),_token),revert OnlyNonCollateralTokens()())
TMP_1305 = CONVERT this to address
TMP_1306(bool) = HIGH_LEVEL_CALL, dest:assetManager_22(IIAssetManager), function:isLockedVaultToken, arguments:['TMP_1305', '_token_1']  
assetManager_23(IIAssetManager) := phi(['assetManager_7', 'assetManager_19', 'assetManager_20', 'assetManager_22', 'assetManager_11', 'assetManager_26', 'assetManager_30', 'assetManager_15', 'assetManager_28', 'assetManager_1', 'assetManager_24'])
destroyed_7(bool) := phi(['destroyed_8', 'destroyed_4', 'destroyed_7', 'destroyed_6'])
TMP_1307 = UnaryType.BANG TMP_1306 
TMP_1308(bool) = destroyed_7 || TMP_1307
TMP_1309(None) = SOLIDITY_CALL revert OnlyNonCollateralTokens()()
TMP_1310(None) = SOLIDITY_CALL require(bool,error)(TMP_1308,TMP_1309)
 ownerManagementAddress = assetManager.getAgentVaultOwner(address(this))
TMP_1311 = CONVERT this to address
TMP_1312(address) = HIGH_LEVEL_CALL, dest:assetManager_23(IIAssetManager), function:getAgentVaultOwner, arguments:['TMP_1311']  
assetManager_24(IIAssetManager) := phi(['assetManager_7', 'assetManager_19', 'assetManager_20', 'assetManager_23', 'assetManager_11', 'assetManager_26', 'assetManager_30', 'assetManager_15', 'assetManager_28', 'assetManager_1', 'assetManager_24'])
ownerManagementAddress_1(address) := TMP_1312(address)
 _token.safeTransfer(ownerManagementAddress,_amount)
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['_token_1', 'ownerManagementAddress_1', '_amount_1'] 
 onlyOwner()
MODIFIER_CALL, AgentVault.onlyOwner()()
```
#### AgentVault.updateCollateral(IERC20) [EXTERNAL][OWNER]
```slithir
assetManager_12(IIAssetManager) := phi(['assetManager_7', 'assetManager_19', 'assetManager_20', 'assetManager_11', 'assetManager_26', 'assetManager_0', 'assetManager_30', 'assetManager_15', 'assetManager_28', 'assetManager_1', 'assetManager_24'])
 assetManager.updateCollateral(address(this),_token)
TMP_1295 = CONVERT this to address
HIGH_LEVEL_CALL, dest:assetManager_14(IIAssetManager), function:updateCollateral, arguments:['TMP_1295', '_token_1']  
assetManager_15(IIAssetManager) := phi(['assetManager_7', 'assetManager_19', 'assetManager_20', 'assetManager_14', 'assetManager_11', 'assetManager_26', 'assetManager_30', 'assetManager_15', 'assetManager_28', 'assetManager_1', 'assetManager_24'])
 onlyOwner()
MODIFIER_CALL, AgentVault.onlyOwner()()
 onlyKnownToken(_token)
MODIFIER_CALL, AgentVault.onlyKnownToken(IERC20)(_token_1)
```
#### AgentVault.withdrawCollateral(IERC20,uint256,address) [EXTERNAL][OWNER]
```slithir
assetManager_16(IIAssetManager) := phi(['assetManager_7', 'assetManager_19', 'assetManager_20', 'assetManager_11', 'assetManager_26', 'assetManager_0', 'assetManager_30', 'assetManager_15', 'assetManager_28', 'assetManager_1', 'assetManager_24'])
destroyed_1(bool) := phi(['destroyed_8', 'destroyed_4', 'destroyed_7', 'destroyed_0'])
 ! destroyed
TMP_1299 = UnaryType.BANG destroyed_4 
CONDITION TMP_1299
 assetManager.beforeCollateralWithdrawal(_token,_amount)
HIGH_LEVEL_CALL, dest:assetManager_19(IIAssetManager), function:beforeCollateralWithdrawal, arguments:['_token_1', '_amount_1']  
assetManager_20(IIAssetManager) := phi(['assetManager_7', 'assetManager_19', 'assetManager_20', 'assetManager_11', 'assetManager_26', 'assetManager_30', 'assetManager_15', 'assetManager_28', 'assetManager_1', 'assetManager_24'])
 _token.safeTransfer(_recipient,_amount)
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['_token_1', '_recipient_1', '_amount_1'] 
 onlyOwner()
MODIFIER_CALL, AgentVault.onlyOwner()()
 onlyKnownToken(_token)
MODIFIER_CALL, AgentVault.onlyKnownToken(IERC20)(_token_1)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### AgentVault.withdrawPoolFees(uint256,address) [EXTERNAL][OWNER]
```slithir
 collateralPool().withdrawFeesTo(_amount,_recipient)
TMP_1280(ICollateralPool) = INTERNAL_CALL, AgentVault.collateralPool()()
HIGH_LEVEL_CALL, dest:TMP_1280(ICollateralPool), function:withdrawFeesTo, arguments:['_amount_1', '_recipient_1']  
 onlyOwner()
MODIFIER_CALL, AgentVault.onlyOwner()()
```
#### AssetManagerMock.isVaultCollateralToken(IERC20) [EXTERNAL]
```slithir
 true
RETURN True
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
#### AssetManagerMock.updateCollateral(address,IERC20) [EXTERNAL]
```slithir
checkForValidAgentVaultAddress_1(bool) := phi(['checkForValidAgentVaultAddress_0', 'checkForValidAgentVaultAddress_2'])
 require(bool,error)(! checkForValidAgentVaultAddress,Agent.InvalidAgentVaultAddress())
TMP_5462 = UnaryType.BANG checkForValidAgentVaultAddress_1 
TMP_5463(None) = SOLIDITY_CALL revert InvalidAgentVaultAddress()()
TMP_5464(None) = SOLIDITY_CALL require(bool,error)(TMP_5462,TMP_5463)
 CollateralUpdated(_agentVault,address(_token))
TMP_5465 = CONVERT _token_1 to address
Emit CollateralUpdated(_agentVault_1,TMP_5465)
```
#### SafeERC20.safeTransferFrom(IERC20,address,address,uint256) [INTERNAL]
```slithir
 _callOptionalReturn(token,abi.encodeWithSelector(token.transferFrom.selector,from,to,value))
REF_89(bytes4) (->None) := 599290589(bytes4)
TMP_245(bytes) = SOLIDITY_CALL abi.encodeWithSelector()(REF_89,from_1,to_1,value_1)
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_245)
```
#### ERC1967Upgrade._getImplementation() [INTERNAL]
```slithir
_IMPLEMENTATION_SLOT_1(bytes32) := phi(['_IMPLEMENTATION_SLOT_4', '_IMPLEMENTATION_SLOT_0'])
 StorageSlot.getAddressSlot(_IMPLEMENTATION_SLOT).value
TMP_60(StorageSlot.AddressSlot) = LIBRARY_CALL, dest:StorageSlot, function:StorageSlot.getAddressSlot(bytes32), arguments:['_IMPLEMENTATION_SLOT_1'] 
REF_26(address) -> TMP_60.value
RETURN REF_26
```
#### ReentrancyGuard.initializeReentrancyGuard() [INTERNAL]
```slithir
 Reentrancy.initializeReentrancyGuard()
LIBRARY_CALL, dest:Reentrancy, function:Reentrancy.initializeReentrancyGuard(), arguments:[]
```
#### AssetManagerMock.isAgentVaultOwner(address,address) [EXTERNAL]
```slithir
commonOwner_3(address) := phi(['commonOwner_0', 'commonOwner_1'])
 _address == commonOwner
TMP_5461(bool) = _address_1 == commonOwner_3
RETURN TMP_5461
```
#### SafeERC20.safeTransfer(IERC20,address,uint256) [INTERNAL]
```slithir
 _callOptionalReturn(token,abi.encodeWithSelector(token.transfer.selector,to,value))
REF_86(bytes4) (->None) := 2835717307(bytes4)
TMP_243(bytes) = SOLIDITY_CALL abi.encodeWithSelector()(REF_86,to_1,value_1)
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_243)
```
#### IIAssetManager.beforeCollateralWithdrawal(IERC20,uint256) [EXTERNAL]
```slithir

```
#### CollateralPool.exitTo(uint256,address) [EXTERNAL]
```slithir
 _exitTo(_tokenShare,_recipient)
TMP_6094(uint256) = INTERNAL_CALL, CollateralPool._exitTo(uint256,address)(_tokenShare_1,_recipient_1)
RETURN TMP_6094
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
```
#### CollateralPool.poolToken() [EXTERNAL]
```slithir
token_4(IICollateralPoolToken) := phi(['token_13', 'token_43', 'token_55', 'token_51', 'token_61', 'token_64', 'token_49', 'token_68', 'token_59', 'token_0', 'token_57', 'token_22', 'token_39', 'token_3', 'token_53', 'token_37', 'token_44'])
 token
RETURN token_4
```
#### IIAssetManager.isLockedVaultToken(address,IERC20) [EXTERNAL]
```slithir

```
#### CollateralPool.withdrawFeesTo(uint256,address) [EXTERNAL]
```slithir
 _withdrawFeesTo(_fAssets,_recipient)
INTERNAL_CALL, CollateralPool._withdrawFeesTo(uint256,address)(_fAssets_1,_recipient_1)
 nonReentrant()
MODIFIER_CALL, ReentrancyGuard.nonReentrant()()
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
#### CollateralPoolToken.mint(address,uint256) [EXTERNAL]
```slithir
timelocksByAccount_1(mapping(address => CollateralPoolToken.TimelockQueue)) := phi(['timelocksByAccount_0', 'timelocksByAccount_4', 'timelocksByAccount_9', 'timelocksByAccount_7'])
 _mint(_account,_amount)
INTERNAL_CALL, ERC20._mint(address,uint256)(_account_1,_amount_1)
 timelockDuration = _getTimelockDuration()
TMP_6554(uint256) = INTERNAL_CALL, CollateralPoolToken._getTimelockDuration()()
timelockDuration_1(uint256) := TMP_6554(uint256)
 _timelockExpiresAt = block.timestamp + timelockDuration
TMP_6555(uint256) = block.timestamp (c)+ timelockDuration_1
_timelockExpiresAt_1(uint256) := TMP_6555(uint256)
 timelockDuration > 0 && _amount > 0
TMP_6556(bool) = timelockDuration_1 > 0
TMP_6557(bool) = _amount_1 > 0
TMP_6558(bool) = TMP_6556 && TMP_6557
CONDITION TMP_6558
 timelocks = timelocksByAccount[_account]
REF_4335(CollateralPoolToken.TimelockQueue) -> timelocksByAccount_4[_account_1]
timelocks_1 (-> ['timelocksByAccount'])(CollateralPoolToken.TimelockQueue) := REF_4335(CollateralPoolToken.TimelockQueue)
 timelocks.data[timelocks.end ++] = Timelock({amount:_amount.toUint128(),endTime:_timelockExpiresAt.toUint64()})
REF_4336(mapping(uint256 => CollateralPoolToken.Timelock)) -> timelocks_1 (-> ['timelocksByAccount']).data
REF_4337(uint128) -> timelocks_1 (-> ['timelocksByAccount']).end
TMP_6559(uint128) := REF_4337(uint128)
timelocks_2 (-> ['timelocksByAccount'])(CollateralPoolToken.TimelockQueue) := phi(["timelocks_1 (-> ['timelocksByAccount'])"])
REF_4337(-> timelocks_2 (-> ['timelocksByAccount'])) = REF_4337 (c)+ 1
REF_4338(CollateralPoolToken.Timelock) -> REF_4336[TMP_6559]
TMP_6560(uint128) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint128(uint256), arguments:['_amount_1'] 
TMP_6561(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['_timelockExpiresAt_1'] 
TMP_6562(CollateralPoolToken.Timelock) = new Timelock(TMP_6560,TMP_6561)
timelocks_3 (-> ['timelocksByAccount'])(CollateralPoolToken.TimelockQueue) := phi(["timelocks_1 (-> ['timelocksByAccount'])"])
REF_4338(CollateralPoolToken.Timelock) (->timelocks_3 (-> ['timelocksByAccount'])) := TMP_6562(CollateralPoolToken.Timelock)
timelocksByAccount_5(mapping(address => CollateralPoolToken.TimelockQueue)) := phi(["timelocks_2 (-> ['timelocksByAccount'])"])
timelocksByAccount_6(mapping(address => CollateralPoolToken.TimelockQueue)) := phi(["timelocks_3 (-> ['timelocksByAccount'])"])
 onlyCollateralPool()
MODIFIER_CALL, CollateralPoolToken.onlyCollateralPool()()
 _timelockExpiresAt
RETURN _timelockExpiresAt_1
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
#### StorageSlot.getAddressSlot(bytes32) [INTERNAL]
```slithir
 r = slot
r_1 (-> ['slot'])(StorageSlot.AddressSlot) := slot_1(bytes32)
 r
RETURN r_1 (-> ['slot'])
```
#### Reentrancy.initializeReentrancyGuard() [INTERNAL]
```slithir
_NOT_ENTERED_1(uint256) := phi(['_NOT_ENTERED_2', '_NOT_ENTERED_0', '_NOT_ENTERED_4'])
 state = _reentrancyGuardState()
TMP_9987(Reentrancy.ReentrancyGuardState) = INTERNAL_CALL, Reentrancy._reentrancyGuardState()()
state_1 (-> ['TMP_9987'])(Reentrancy.ReentrancyGuardState) := TMP_9987(Reentrancy.ReentrancyGuardState)
 state.status = _NOT_ENTERED
REF_6037(uint256) -> state_1 (-> ['TMP_9987']).status
state_2 (-> ['TMP_9987'])(Reentrancy.ReentrancyGuardState) := phi(["state_1 (-> ['TMP_9987'])"])
REF_6037(uint256) (->state_2 (-> ['TMP_9987'])) := _NOT_ENTERED_2(uint256)
TMP_9987(Reentrancy.ReentrancyGuardState) := phi(["state_2 (-> ['TMP_9987'])"])
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

#### WNatMock.deposit() [PUBLIC]
```slithir
 _mint(msg.sender,msg.value)
INTERNAL_CALL, ERC20._mint(address,uint256)(msg.sender,msg.value)
```
#### AssetManagerMock.assetPriceNatWei() [PUBLIC]
```slithir
assetPriceMul_1(uint256) := phi(['assetPriceMul_2', 'assetPriceMul_0'])
assetPriceDiv_1(uint256) := phi(['assetPriceDiv_2', 'assetPriceDiv_0'])
 (assetPriceMul,assetPriceDiv)
RETURN assetPriceMul_1,assetPriceDiv_1
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
#### CollateralPoolToken._getTimelockDuration() [INTERNAL]
```slithir
collateralPool_7(address) := phi(['collateralPool_8', 'collateralPool_1', 'collateralPool_3', 'collateralPool_10', 'collateralPool_0', 'collateralPool_5'])
 assetManager = IICollateralPool(collateralPool).assetManager()
TMP_6608 = CONVERT collateralPool_7 to IICollateralPool
TMP_6609(IIAssetManager) = HIGH_LEVEL_CALL, dest:TMP_6608(IICollateralPool), function:assetManager, arguments:[]  
collateralPool_8(address) := phi(['collateralPool_8', 'collateralPool_1', 'collateralPool_7', 'collateralPool_3', 'collateralPool_10', 'collateralPool_5'])
assetManager_1(IIAssetManager) := TMP_6609(IIAssetManager)
 assetManager.getCollateralPoolTokenTimelockSeconds()
TMP_6610(uint256) = HIGH_LEVEL_CALL, dest:assetManager_1(IIAssetManager), function:getCollateralPoolTokenTimelockSeconds, arguments:[]  
RETURN TMP_6610
```
#### ERC20._mint(address,uint256) [INTERNAL]
```slithir
_balances_6(mapping(address => uint256)) := phi(['_balances_11', '_balances_1', '_balances_5', '_balances_8', '_balances_0'])
_totalSupply_2(uint256) := phi(['_totalSupply_0', '_totalSupply_4', '_totalSupply_7'])
 require(bool,string)(account != address(0),ERC20: mint to the zero address)
TMP_207 = CONVERT 0 to address
TMP_208(bool) = account_1 != TMP_207
TMP_209(None) = SOLIDITY_CALL require(bool,string)(TMP_208,ERC20: mint to the zero address)
 _beforeTokenTransfer(address(0),account,amount)
TMP_210 = CONVERT 0 to address
INTERNAL_CALL, ERC20._beforeTokenTransfer(address,address,uint256)(TMP_210,account_1,amount_1)
 _totalSupply += amount
_totalSupply_4(uint256) = _totalSupply_3 (c)+ amount_1
 _balances[account] += amount
REF_79(uint256) -> _balances_7[account_1]
_balances_8(mapping(address => uint256)) := phi(['_balances_7'])
REF_79(-> _balances_8) = REF_79 + amount_1
 Transfer(address(0),account,amount)
TMP_212 = CONVERT 0 to address
Emit Transfer(TMP_212,account_1,amount_1)
 _afterTokenTransfer(address(0),account,amount)
TMP_214 = CONVERT 0 to address
INTERNAL_CALL, ERC20._afterTokenTransfer(address,address,uint256)(TMP_214,account_1,amount_1)
```
#### SafeCast.toUint128(uint256) [INTERNAL]
```slithir
 require(bool,string)(value <= type()(uint128).max,SafeCast: value doesn't fit in 128 bits)
TMP_707(uint128) := 340282366920938463463374607431768211455(uint128)
TMP_708(bool) = value_1 <= TMP_707
TMP_709(None) = SOLIDITY_CALL require(bool,string)(TMP_708,SafeCast: value doesn't fit in 128 bits)
 uint128(value)
TMP_710 = CONVERT value_1 to uint128
RETURN TMP_710
```
#### SafeCast.toUint64(uint256) [INTERNAL]
```slithir
 require(bool,string)(value <= type()(uint64).max,SafeCast: value doesn't fit in 64 bits)
TMP_747(uint64) := 18446744073709551615(uint64)
TMP_748(bool) = value_1 <= TMP_747
TMP_749(None) = SOLIDITY_CALL require(bool,string)(TMP_748,SafeCast: value doesn't fit in 64 bits)
 uint64(value)
TMP_750 = CONVERT value_1 to uint64
RETURN TMP_750
```
#### Address.functionCall(address,bytes,string) [INTERNAL]
```slithir
 functionCallWithValue(target,data,0,errorMessage)
TMP_301(bytes) = INTERNAL_CALL, Address.functionCallWithValue(address,bytes,uint256,string)(target_1,data_1,0,errorMessage_1)
RETURN TMP_301
```
#### Reentrancy._reentrancyGuardState() [PRIVATE]
```slithir
 position = keccak256(bytes)(utils.ReentrancyGuard.ReentrancyGuardState)
TMP_9998(bytes32) = SOLIDITY_CALL keccak256(bytes)(utils.ReentrancyGuard.ReentrancyGuardState)
position_1(bytes32) := TMP_9998(bytes32)
 _state = position
_state_1 (-> ['position'])(Reentrancy.ReentrancyGuardState) := position_1(bytes32)
 _state
RETURN _state_1 (-> ['position'])
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
#### CollateralPoolToken.burn(address,uint256,bool) [EXTERNAL]
```slithir
 _ignoreTimelocked
CONDITION _ignoreTimelocked_1
 ignoreTimelocked = true
ignoreTimelocked_1(bool) := True(bool)
 _burn(_account,_amount)
INTERNAL_CALL, ERC20._burn(address,uint256)(_account_1,_amount_1)
 _ignoreTimelocked
CONDITION _ignoreTimelocked_1
 ignoreTimelocked = false
ignoreTimelocked_2(bool) := False(bool)
 onlyCollateralPool()
MODIFIER_CALL, CollateralPoolToken.onlyCollateralPool()()
```
#### CollateralPool._fAssetFeesOf(address) [INTERNAL]
```slithir
_account_1(address) := phi(['msg.sender', '_account_1'])
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
