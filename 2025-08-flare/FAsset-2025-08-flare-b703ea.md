
### Storage layout (FAsset) 

```text
assetName string
assetSymbol string
cleanupBlockNumberManager address
assetManager address
__terminatedAt uint64
_name string
_symbol string
_decimals uint8
_deployer address
_initialized bool
_version uint16

```
#### FAsset._approve(address,address,uint256) [INTERNAL]
```slithir
_owner_1(address) := phi(['owner_1', 'owner_1', 'owner_1', 'owner_1', 'owner_1'])
_spender_1(address) := phi(['spender_1', 'spender_1', 'spender_1', 'spender_1', 'spender_1'])
_amount_1(uint256) := phi(['TMP_7803', 'amount_1', 'TMP_7755', 'TMP_7749', 'value_1'])
 ERC20._approve(_owner,_spender,_amount)
INTERNAL_CALL, ERC20._approve(address,address,uint256)(_owner_1,_spender_1,_amount_1)
```
#### FAsset._authorizeUpgrade(address) [INTERNAL]
```slithir
 onlyAssetManager()
MODIFIER_CALL, FAsset.onlyAssetManager()()
```
#### FAsset._beforeTokenTransfer(address,address,uint256) [INTERNAL]
```slithir
_from_1(address) := phi(['from_1', 'account_1', 'TMP_7772'])
_to_1(address) := phi(['to_1', 'account_1', 'TMP_7781'])
_amount_1(uint256) := phi(['amount_1', 'amount_1', 'amount_1'])
assetManager_3(address) := phi(['assetManager_0', 'assetManager_2', 'assetManager_5'])
 require(bool,error)(_from == address(0) || balanceOf(_from) >= _amount,revert FAssetBalanceTooLow()())
TMP_7837 = CONVERT 0 to address
TMP_7838(bool) = _from_1 == TMP_7837
TMP_7839(uint256) = INTERNAL_CALL, ERC20.balanceOf(address)(_from_1)
TMP_7840(bool) = TMP_7839 >= _amount_1
TMP_7841(bool) = TMP_7838 || TMP_7840
TMP_7842(None) = SOLIDITY_CALL revert FAssetBalanceTooLow()()
TMP_7843(None) = SOLIDITY_CALL require(bool,error)(TMP_7841,TMP_7842)
 require(bool,error)(_from != _to,revert CannotTransferToSelf()())
TMP_7844(bool) = _from_1 != _to_1
TMP_7845(None) = SOLIDITY_CALL revert CannotTransferToSelf()()
TMP_7846(None) = SOLIDITY_CALL require(bool,error)(TMP_7844,TMP_7845)
 require(bool,error)(_from == address(0) || _to == address(0) || ! IAssetManager(assetManager).transfersEmergencyPaused(),revert EmergencyPauseOfTransfersActive()())
TMP_7847 = CONVERT 0 to address
TMP_7848(bool) = _from_1 == TMP_7847
TMP_7849 = CONVERT 0 to address
TMP_7850(bool) = _to_1 == TMP_7849
TMP_7851(bool) = TMP_7848 || TMP_7850
TMP_7852 = CONVERT assetManager_4 to IAssetManager
TMP_7853(bool) = HIGH_LEVEL_CALL, dest:TMP_7852(IAssetManager), function:transfersEmergencyPaused, arguments:[]  
assetManager_5(address) := phi(['assetManager_4', 'assetManager_2', 'assetManager_5'])
TMP_7854 = UnaryType.BANG TMP_7853 
TMP_7855(bool) = TMP_7851 || TMP_7854
TMP_7856(None) = SOLIDITY_CALL revert EmergencyPauseOfTransfersActive()()
TMP_7857(None) = SOLIDITY_CALL require(bool,error)(TMP_7855,TMP_7856)
 _updateBalanceHistoryAtTransfer(_from,_to,_amount)
INTERNAL_CALL, CheckPointable._updateBalanceHistoryAtTransfer(address,address,uint256)(_from_1,_to_1,_amount_1)
```
#### FAsset.burn(address,uint256) [EXTERNAL]
```slithir
 _burn(_owner,_amount)
INTERNAL_CALL, ERC20._burn(address,uint256)(_owner_1,_amount_1)
 onlyAssetManager()
MODIFIER_CALL, FAsset.onlyAssetManager()()
```
#### FAsset.cleanupBlockNumber() [EXTERNAL]
```slithir
 _cleanupBlockNumber()
TMP_7833(uint256) = INTERNAL_CALL, CheckPointable._cleanupBlockNumber()()
RETURN TMP_7833
```
#### FAsset.constructor() [PUBLIC]
```slithir
 _initialized = true
_initialized_1(bool) := True(bool)
 _version = 1000
_version_1(uint16) := 1000(uint256)
 ERC20(,)
INTERNAL_CALL, ERC20.constructor(string,string)(,)
```
#### FAsset.decimals() [PUBLIC]
```slithir
_decimals_2(uint8) := phi(['_decimals_0', '_decimals_1'])
 _decimals
RETURN _decimals_2
```
#### FAsset.implementation() [EXTERNAL]
```slithir
 _getImplementation()
TMP_7886(address) = INTERNAL_CALL, ERC1967Upgrade._getImplementation()()
RETURN TMP_7886
```
#### FAsset.initialize(string,string,string,string,uint8) [EXTERNAL]
```slithir
_initialized_2(bool) := phi(['_initialized_1', '_initialized_3', '_initialized_0'])
 require(bool,error)(! _initialized,revert AlreadyInitialized()())
TMP_7806 = UnaryType.BANG _initialized_2 
TMP_7807(None) = SOLIDITY_CALL revert AlreadyInitialized()()
TMP_7808(None) = SOLIDITY_CALL require(bool,error)(TMP_7806,TMP_7807)
 _initialized = true
_initialized_3(bool) := True(bool)
 _deployer = msg.sender
_deployer_1(address) := msg.sender(address)
 _name = name_
_name_1(string) := name__1(string)
 _symbol = symbol_
_symbol_1(string) := symbol__1(string)
 _decimals = decimals_
_decimals_1(uint8) := decimals__1(uint8)
 assetName = assetName_
assetName_1(string) := assetName__1(string)
 assetSymbol = assetSymbol_
assetSymbol_1(string) := assetSymbol__1(string)
 initializeV1r1()
INTERNAL_CALL, FAsset.initializeV1r1()()
```
#### FAsset.initializeV1r1() [PUBLIC]
```slithir
_name_2(string) := phi(['_name_1', '_name_0', '_name_3'])
_version_2(uint16) := phi(['_version_0', '_version_1', '_version_3'])
 require(bool,error)(_version == 0,revert AlreadyUpgraded()())
TMP_7810(bool) = _version_2 == 0
TMP_7811(None) = SOLIDITY_CALL revert AlreadyUpgraded()()
TMP_7812(None) = SOLIDITY_CALL require(bool,error)(TMP_7810,TMP_7811)
 _version = 1
_version_3(uint16) := 1(uint256)
 initializeEIP712(_name,1)
INTERNAL_CALL, EIP712.initializeEIP712(string,string)(_name_2,1)
```
#### FAsset.mint(address,uint256) [EXTERNAL]
```slithir
 _mint(_owner,_amount)
INTERNAL_CALL, ERC20._mint(address,uint256)(_owner_1,_amount_1)
 onlyAssetManager()
MODIFIER_CALL, FAsset.onlyAssetManager()()
```
#### FAsset.name() [PUBLIC]
```slithir
_name_4(string) := phi(['_name_1', '_name_0', '_name_3'])
 _name
RETURN _name_4
```
#### FAsset.setAssetManager(address) [EXTERNAL]
```slithir
assetManager_1(address) := phi(['assetManager_0', 'assetManager_2', 'assetManager_5'])
_deployer_2(address) := phi(['_deployer_0', '_deployer_1'])
 require(bool,error)(msg.sender == _deployer,revert OnlyDeployer()())
TMP_7814(bool) = msg.sender == _deployer_2
TMP_7815(None) = SOLIDITY_CALL revert OnlyDeployer()()
TMP_7816(None) = SOLIDITY_CALL require(bool,error)(TMP_7814,TMP_7815)
 require(bool,error)(_assetManager != address(0),revert ZeroAssetManager()())
TMP_7817 = CONVERT 0 to address
TMP_7818(bool) = _assetManager_1 != TMP_7817
TMP_7819(None) = SOLIDITY_CALL revert ZeroAssetManager()()
TMP_7820(None) = SOLIDITY_CALL require(bool,error)(TMP_7818,TMP_7819)
 require(bool,error)(assetManager == address(0),revert CannotReplaceAssetManager()())
TMP_7821 = CONVERT 0 to address
TMP_7822(bool) = assetManager_1 == TMP_7821
TMP_7823(None) = SOLIDITY_CALL revert CannotReplaceAssetManager()()
TMP_7824(None) = SOLIDITY_CALL require(bool,error)(TMP_7822,TMP_7823)
 assetManager = _assetManager
assetManager_2(address) := _assetManager_1(address)
```
#### FAsset.setCleanerContract(address) [EXTERNAL]
```slithir
 _setCleanerContract(_cleanerContract)
INTERNAL_CALL, CheckPointable._setCleanerContract(address)(_cleanerContract_1)
 onlyAssetManager()
MODIFIER_CALL, FAsset.onlyAssetManager()()
```
#### FAsset.setCleanupBlockNumber(uint256) [EXTERNAL]
```slithir
cleanupBlockNumberManager_1(address) := phi(['cleanupBlockNumberManager_0', 'cleanupBlockNumberManager_2'])
 require(bool,error)(msg.sender == cleanupBlockNumberManager,revert OnlyCleanupBlockManager()())
TMP_7829(bool) = msg.sender == cleanupBlockNumberManager_1
TMP_7830(None) = SOLIDITY_CALL revert OnlyCleanupBlockManager()()
TMP_7831(None) = SOLIDITY_CALL require(bool,error)(TMP_7829,TMP_7830)
 _setCleanupBlockNumber(_blockNumber)
INTERNAL_CALL, CheckPointable._setCleanupBlockNumber(uint256)(_blockNumber_1)
```
#### FAsset.setCleanupBlockNumberManager(address) [EXTERNAL]
```slithir
 cleanupBlockNumberManager = _cleanupBlockNumberManager
cleanupBlockNumberManager_2(address) := _cleanupBlockNumberManager_1(address)
 onlyAssetManager()
MODIFIER_CALL, FAsset.onlyAssetManager()()
```
#### CheckPointable.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 CLEANUP_COUNT = 2
_blockNumber_1(uint256) := phi(['_blockNumber_1', '_blockNumber_1'])
cleanupBlockNumber_15(uint256) := phi(['cleanupBlockNumber_6', 'cleanupBlockNumber_12', 'cleanupBlockNumber_0', 'cleanupBlockNumber_9', 'cleanupBlockNumber_3', 'cleanupBlockNumber_14'])
 require(bool,error)(_blockNumber >= cleanupBlockNumber,revert CheckPointableReadingFromCleanedupBlock()())
TMP_7609(bool) = _blockNumber_1 >= cleanupBlockNumber_15
TMP_7610(None) = SOLIDITY_CALL revert CheckPointableReadingFromCleanedupBlock()()
TMP_7611(None) = SOLIDITY_CALL require(bool,error)(TMP_7609,TMP_7610)
cleanerContract_2(address) := phi(['cleanerContract_0', 'cleanerContract_1'])
 require(bool,error)(msg.sender == cleanerContract,revert OnlyCleanerContract()())
TMP_7612(bool) = msg.sender == cleanerContract_2
TMP_7613(None) = SOLIDITY_CALL revert OnlyCleanerContract()()
TMP_7614(None) = SOLIDITY_CALL require(bool,error)(TMP_7612,TMP_7613)
```
#### FAsset.supportsInterface(bytes4) [EXTERNAL]
```slithir
 _interfaceId == type()(IERC165).interfaceId || _interfaceId == type()(IERC20).interfaceId || _interfaceId == type()(IERC20Metadata).interfaceId || _interfaceId == type()(IERC5267).interfaceId || _interfaceId == type()(IERC20Permit).interfaceId || _interfaceId == type()(IICheckPointable).interfaceId || _interfaceId == type()(IFAsset).interfaceId || _interfaceId == type()(IIFAsset).interfaceId || _interfaceId == type()(IICleanable).interfaceId
TMP_7859(type(IERC165)) = SOLIDITY_CALL type()(IERC165)
REF_4988(bytes4) (->None) := 33540519(bytes4)
TMP_7860(bool) = _interfaceId_1 == REF_4988
TMP_7861(type(IERC20)) = SOLIDITY_CALL type()(IERC20)
REF_4989(bytes4) (->None) := 909585159(bytes4)
TMP_7862(bool) = _interfaceId_1 == REF_4989
TMP_7863(bool) = TMP_7860 || TMP_7862
TMP_7864(type(IERC20Metadata)) = SOLIDITY_CALL type()(IERC20Metadata)
REF_4990(bytes4) (->None) := 2486078242(bytes4)
TMP_7865(bool) = _interfaceId_1 == REF_4990
TMP_7866(bool) = TMP_7863 || TMP_7865
TMP_7867(type(IERC5267)) = SOLIDITY_CALL type()(IERC5267)
REF_4991(bytes4) (->None) := 2226133358(bytes4)
TMP_7868(bool) = _interfaceId_1 == REF_4991
TMP_7869(bool) = TMP_7866 || TMP_7868
TMP_7870(type(IERC20Permit)) = SOLIDITY_CALL type()(IERC20Permit)
REF_4992(bytes4) (->None) := 2643458010(bytes4)
TMP_7871(bool) = _interfaceId_1 == REF_4992
TMP_7872(bool) = TMP_7869 || TMP_7871
TMP_7873(type(IICheckPointable)) = SOLIDITY_CALL type()(IICheckPointable)
REF_4993(bytes4) (->None) := 3606702510(bytes4)
TMP_7874(bool) = _interfaceId_1 == REF_4993
TMP_7875(bool) = TMP_7872 || TMP_7874
TMP_7876(type(IFAsset)) = SOLIDITY_CALL type()(IFAsset)
REF_4994(bytes4) (->None) := 3728808454(bytes4)
TMP_7877(bool) = _interfaceId_1 == REF_4994
TMP_7878(bool) = TMP_7875 || TMP_7877
TMP_7879(type(IIFAsset)) = SOLIDITY_CALL type()(IIFAsset)
REF_4995(bytes4) (->None) := 3754442246(bytes4)
TMP_7880(bool) = _interfaceId_1 == REF_4995
TMP_7881(bool) = TMP_7878 || TMP_7880
TMP_7882(type(IICleanable)) = SOLIDITY_CALL type()(IICleanable)
REF_4996(bytes4) (->None) := 999297213(bytes4)
TMP_7883(bool) = _interfaceId_1 == REF_4996
TMP_7884(bool) = TMP_7881 || TMP_7883
RETURN TMP_7884
```
#### FAsset.symbol() [PUBLIC]
```slithir
_symbol_2(string) := phi(['_symbol_1', '_symbol_0'])
 _symbol
RETURN _symbol_2
```
#### IAssetManager.transfersEmergencyPaused() [EXTERNAL]
```slithir

```
