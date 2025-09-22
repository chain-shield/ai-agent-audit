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


### Storage layout (CheckPointable) 

```text
balanceHistory CheckPointsByAddress.CheckPointsByAddressState
totalSupply CheckPointHistory.CheckPointHistoryState
cleanupBlockNumber uint256
cleanerContract address
balanceHistory CheckPointsByAddress.CheckPointsByAddressState
totalSupply CheckPointHistory.CheckPointHistoryState
cleanupBlockNumber uint256
cleanerContract address

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
#### CheckPointable._updateBalanceHistoryAtTransfer(address,address,uint256) [INTERNAL]
```slithir
 _from == address(0)
TMP_7598 = CONVERT 0 to address
TMP_7599(bool) = _from_1 == TMP_7598
CONDITION TMP_7599
 _mintForAtNow(_to,_amount)
INTERNAL_CALL, CheckPointable._mintForAtNow(address,uint256)(_to_1,_amount_1)
 _to == address(0)
TMP_7601 = CONVERT 0 to address
TMP_7602(bool) = _to_1 == TMP_7601
CONDITION TMP_7602
 _burnForAtNow(_from,_amount)
INTERNAL_CALL, CheckPointable._burnForAtNow(address,uint256)(_from_1,_amount_1)
 _transmitAtNow(_from,_to,_amount)
INTERNAL_CALL, CheckPointable._transmitAtNow(address,address,uint256)(_from_1,_to_1,_amount_1)
```
#### ERC20.balanceOf(address) [PUBLIC]
```slithir
_balances_1(mapping(address => uint256)) := phi(['_balances_11', '_balances_1', '_balances_5', '_balances_8', '_balances_0'])
 _balances[account]
REF_73(uint256) -> _balances_1[account_1]
RETURN REF_73
```
#### AssetManagerMock.transfersEmergencyPaused() [EXTERNAL]
```slithir
 false
RETURN False
```
#### ERC20._burn(address,uint256) [INTERNAL]
```slithir
_balances_9(mapping(address => uint256)) := phi(['_balances_11', '_balances_1', '_balances_5', '_balances_8', '_balances_0'])
_totalSupply_5(uint256) := phi(['_totalSupply_0', '_totalSupply_4', '_totalSupply_7'])
 require(bool,string)(account != address(0),ERC20: burn from the zero address)
TMP_216 = CONVERT 0 to address
TMP_217(bool) = account_1 != TMP_216
TMP_218(None) = SOLIDITY_CALL require(bool,string)(TMP_217,ERC20: burn from the zero address)
 _beforeTokenTransfer(account,address(0),amount)
TMP_219 = CONVERT 0 to address
INTERNAL_CALL, ERC20._beforeTokenTransfer(address,address,uint256)(account_1,TMP_219,amount_1)
 accountBalance = _balances[account]
REF_80(uint256) -> _balances_10[account_1]
accountBalance_1(uint256) := REF_80(uint256)
 require(bool,string)(accountBalance >= amount,ERC20: burn amount exceeds balance)
TMP_221(bool) = accountBalance_1 >= amount_1
TMP_222(None) = SOLIDITY_CALL require(bool,string)(TMP_221,ERC20: burn amount exceeds balance)
 _balances[account] = accountBalance - amount
REF_81(uint256) -> _balances_10[account_1]
TMP_223(uint256) = accountBalance_1 - amount_1
_balances_11(mapping(address => uint256)) := phi(['_balances_10'])
REF_81(uint256) (->_balances_11) := TMP_223(uint256)
 _totalSupply -= amount
_totalSupply_7(uint256) = _totalSupply_6 - amount_1
 Transfer(account,address(0),amount)
TMP_224 = CONVERT 0 to address
Emit Transfer(account_1,TMP_224,amount_1)
 _afterTokenTransfer(account,address(0),amount)
TMP_226 = CONVERT 0 to address
INTERNAL_CALL, ERC20._afterTokenTransfer(address,address,uint256)(account_1,TMP_226,amount_1)
```
#### CheckPointable._cleanupBlockNumber() [INTERNAL]
```slithir
cleanupBlockNumber_10(uint256) := phi(['cleanupBlockNumber_6', 'cleanupBlockNumber_12', 'cleanupBlockNumber_0', 'cleanupBlockNumber_9', 'cleanupBlockNumber_3', 'cleanupBlockNumber_14'])
 cleanupBlockNumber
RETURN cleanupBlockNumber_10
```
#### ERC1967Upgrade._getImplementation() [INTERNAL]
```slithir
_IMPLEMENTATION_SLOT_1(bytes32) := phi(['_IMPLEMENTATION_SLOT_4', '_IMPLEMENTATION_SLOT_0'])
 StorageSlot.getAddressSlot(_IMPLEMENTATION_SLOT).value
TMP_60(StorageSlot.AddressSlot) = LIBRARY_CALL, dest:StorageSlot, function:StorageSlot.getAddressSlot(bytes32), arguments:['_IMPLEMENTATION_SLOT_1'] 
REF_26(address) -> TMP_60.value
RETURN REF_26
```
#### EIP712.initializeEIP712(string,string) [INTERNAL]
```slithir
 state = _getEIP712State()
TMP_10403(EIP712.EIP712State) = INTERNAL_CALL, EIP712._getEIP712State()()
state_1 (-> ['TMP_10403'])(EIP712.EIP712State) := TMP_10403(EIP712.EIP712State)
 state.name = name
REF_6178(string) -> state_1 (-> ['TMP_10403']).name
state_2 (-> ['TMP_10403'])(EIP712.EIP712State) := phi(["state_1 (-> ['TMP_10403'])"])
REF_6178(string) (->state_2 (-> ['TMP_10403'])) := name_1(string)
TMP_10403(EIP712.EIP712State) := phi(["state_2 (-> ['TMP_10403'])"])
 state.version = version
REF_6179(string) -> state_2 (-> ['TMP_10403']).version
state_3 (-> ['TMP_10403'])(EIP712.EIP712State) := phi(["state_2 (-> ['TMP_10403'])"])
REF_6179(string) (->state_3 (-> ['TMP_10403'])) := version_1(string)
TMP_10403(EIP712.EIP712State) := phi(["state_3 (-> ['TMP_10403'])"])
 state.hashedName = keccak256(bytes)(bytes(name))
REF_6180(bytes32) -> state_3 (-> ['TMP_10403']).hashedName
TMP_10404 = CONVERT name_1 to bytes
TMP_10405(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_10404)
state_4 (-> ['TMP_10403'])(EIP712.EIP712State) := phi(["state_3 (-> ['TMP_10403'])"])
REF_6180(bytes32) (->state_4 (-> ['TMP_10403'])) := TMP_10405(bytes32)
TMP_10403(EIP712.EIP712State) := phi(["state_4 (-> ['TMP_10403'])"])
 state.hashedVersion = keccak256(bytes)(bytes(version))
REF_6181(bytes32) -> state_4 (-> ['TMP_10403']).hashedVersion
TMP_10406 = CONVERT version_1 to bytes
TMP_10407(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_10406)
state_5 (-> ['TMP_10403'])(EIP712.EIP712State) := phi(["state_4 (-> ['TMP_10403'])"])
REF_6181(bytes32) (->state_5 (-> ['TMP_10403'])) := TMP_10407(bytes32)
TMP_10403(EIP712.EIP712State) := phi(["state_5 (-> ['TMP_10403'])"])
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
#### CheckPointable._setCleanerContract(address) [INTERNAL]
```slithir
 cleanerContract = _cleanerContract
cleanerContract_1(address) := _cleanerContract_1(address)
```
#### CheckPointable._setCleanupBlockNumber(uint256) [INTERNAL]
```slithir
cleanupBlockNumber_8(uint256) := phi(['cleanupBlockNumber_6', 'cleanupBlockNumber_12', 'cleanupBlockNumber_0', 'cleanupBlockNumber_9', 'cleanupBlockNumber_3', 'cleanupBlockNumber_14'])
 require(bool,error)(_blockNumber >= cleanupBlockNumber,revert CleanupBlockNumberMustNeverDecrease()())
TMP_7592(bool) = _blockNumber_1 >= cleanupBlockNumber_8
TMP_7593(None) = SOLIDITY_CALL revert CleanupBlockNumberMustNeverDecrease()()
TMP_7594(None) = SOLIDITY_CALL require(bool,error)(TMP_7592,TMP_7593)
 require(bool,error)(_blockNumber < block.number,revert CleanupBlockMustBeInThePast()())
TMP_7595(bool) = _blockNumber_1 < block.number
TMP_7596(None) = SOLIDITY_CALL revert CleanupBlockMustBeInThePast()()
TMP_7597(None) = SOLIDITY_CALL require(bool,error)(TMP_7595,TMP_7596)
 cleanupBlockNumber = _blockNumber
cleanupBlockNumber_9(uint256) := _blockNumber_1(uint256)
```
#### CheckPointable._burnForAtNow(address,uint256) [INTERNAL]
```slithir
_owner_1(address) := phi(['_from_1'])
_amount_1(uint256) := phi(['_amount_1'])
CLEANUP_COUNT_1(uint256) := phi(['CLEANUP_COUNT_0', 'CLEANUP_COUNT_6', 'CLEANUP_COUNT_3'])
balanceHistory_3(CheckPointsByAddress.CheckPointsByAddressState) := phi(['balanceHistory_9', 'balanceHistory_2', 'balanceHistory_0', 'balanceHistory_4', 'balanceHistory_6'])
totalSupply_1(CheckPointHistory.CheckPointHistoryState) := phi(['totalSupply_8', 'totalSupply_3', 'totalSupply_0', 'totalSupply_10', 'totalSupply_6'])
cleanupBlockNumber_1(uint256) := phi(['cleanupBlockNumber_6', 'cleanupBlockNumber_12', 'cleanupBlockNumber_0', 'cleanupBlockNumber_9', 'cleanupBlockNumber_3', 'cleanupBlockNumber_14'])
 newBalance = balanceOfAt(_owner,block.number) - _amount
TMP_7571(uint256) = INTERNAL_CALL, CheckPointable.balanceOfAt(address,uint256)(_owner_1,block.number)
TMP_7572(uint256) = TMP_7571 (c)- _amount_1
newBalance_1(uint256) := TMP_7572(uint256)
 balanceHistory.writeValue(_owner,newBalance)
LIBRARY_CALL, dest:CheckPointsByAddress, function:CheckPointsByAddress.writeValue(CheckPointsByAddress.CheckPointsByAddressState,address,uint256), arguments:['balanceHistory_4', '_owner_1', 'newBalance_1'] 
 balanceHistory.cleanupOldCheckpoints(_owner,CLEANUP_COUNT,cleanupBlockNumber)
TMP_7574(uint256) = LIBRARY_CALL, dest:CheckPointsByAddress, function:CheckPointsByAddress.cleanupOldCheckpoints(CheckPointsByAddress.CheckPointsByAddressState,address,uint256,uint256), arguments:['balanceHistory_4', '_owner_1', 'CLEANUP_COUNT_2', 'cleanupBlockNumber_2'] 
 totalSupply.writeValue(totalSupplyAt(block.number) - _amount)
TMP_7575(uint256) = INTERNAL_CALL, CheckPointable.totalSupplyAt(uint256)(block.number)
totalSupply_3(CheckPointHistory.CheckPointHistoryState) := phi(['totalSupply_8'])
TMP_7576(uint256) = TMP_7575 (c)- _amount_1
LIBRARY_CALL, dest:CheckPointHistory, function:CheckPointHistory.writeValue(CheckPointHistory.CheckPointHistoryState,uint256), arguments:['totalSupply_3', 'TMP_7576'] 
 totalSupply.cleanupOldCheckpoints(CLEANUP_COUNT,cleanupBlockNumber)
TMP_7578(uint256) = LIBRARY_CALL, dest:CheckPointHistory, function:CheckPointHistory.cleanupOldCheckpoints(CheckPointHistory.CheckPointHistoryState,uint256,uint256), arguments:['totalSupply_3', 'CLEANUP_COUNT_3', 'cleanupBlockNumber_3']
```
#### CheckPointable._mintForAtNow(address,uint256) [INTERNAL]
```slithir
_owner_1(address) := phi(['_to_1'])
_amount_1(uint256) := phi(['_amount_1'])
CLEANUP_COUNT_4(uint256) := phi(['CLEANUP_COUNT_0', 'CLEANUP_COUNT_6', 'CLEANUP_COUNT_3'])
balanceHistory_5(CheckPointsByAddress.CheckPointsByAddressState) := phi(['balanceHistory_9', 'balanceHistory_2', 'balanceHistory_0', 'balanceHistory_4', 'balanceHistory_6'])
totalSupply_4(CheckPointHistory.CheckPointHistoryState) := phi(['totalSupply_8', 'totalSupply_3', 'totalSupply_0', 'totalSupply_10', 'totalSupply_6'])
cleanupBlockNumber_4(uint256) := phi(['cleanupBlockNumber_6', 'cleanupBlockNumber_12', 'cleanupBlockNumber_0', 'cleanupBlockNumber_9', 'cleanupBlockNumber_3', 'cleanupBlockNumber_14'])
 newBalance = balanceOfAt(_owner,block.number) + _amount
TMP_7579(uint256) = INTERNAL_CALL, CheckPointable.balanceOfAt(address,uint256)(_owner_1,block.number)
TMP_7580(uint256) = TMP_7579 (c)+ _amount_1
newBalance_1(uint256) := TMP_7580(uint256)
 balanceHistory.writeValue(_owner,newBalance)
LIBRARY_CALL, dest:CheckPointsByAddress, function:CheckPointsByAddress.writeValue(CheckPointsByAddress.CheckPointsByAddressState,address,uint256), arguments:['balanceHistory_6', '_owner_1', 'newBalance_1'] 
 balanceHistory.cleanupOldCheckpoints(_owner,CLEANUP_COUNT,cleanupBlockNumber)
TMP_7582(uint256) = LIBRARY_CALL, dest:CheckPointsByAddress, function:CheckPointsByAddress.cleanupOldCheckpoints(CheckPointsByAddress.CheckPointsByAddressState,address,uint256,uint256), arguments:['balanceHistory_6', '_owner_1', 'CLEANUP_COUNT_5', 'cleanupBlockNumber_5'] 
 totalSupply.writeValue(totalSupplyAt(block.number) + _amount)
TMP_7583(uint256) = INTERNAL_CALL, CheckPointable.totalSupplyAt(uint256)(block.number)
totalSupply_6(CheckPointHistory.CheckPointHistoryState) := phi(['totalSupply_8'])
TMP_7584(uint256) = TMP_7583 (c)+ _amount_1
LIBRARY_CALL, dest:CheckPointHistory, function:CheckPointHistory.writeValue(CheckPointHistory.CheckPointHistoryState,uint256), arguments:['totalSupply_6', 'TMP_7584'] 
 totalSupply.cleanupOldCheckpoints(CLEANUP_COUNT,cleanupBlockNumber)
TMP_7586(uint256) = LIBRARY_CALL, dest:CheckPointHistory, function:CheckPointHistory.cleanupOldCheckpoints(CheckPointHistory.CheckPointHistoryState,uint256,uint256), arguments:['totalSupply_6', 'CLEANUP_COUNT_6', 'cleanupBlockNumber_6']
```
#### CheckPointable._transmitAtNow(address,address,uint256) [INTERNAL]
```slithir
_from_1(address) := phi(['_from_1'])
_to_1(address) := phi(['_to_1'])
_amount_1(uint256) := phi(['_amount_1'])
CLEANUP_COUNT_7(uint256) := phi(['CLEANUP_COUNT_0', 'CLEANUP_COUNT_6', 'CLEANUP_COUNT_3'])
balanceHistory_7(CheckPointsByAddress.CheckPointsByAddressState) := phi(['balanceHistory_9', 'balanceHistory_2', 'balanceHistory_0', 'balanceHistory_4', 'balanceHistory_6'])
cleanupBlockNumber_7(uint256) := phi(['cleanupBlockNumber_6', 'cleanupBlockNumber_12', 'cleanupBlockNumber_0', 'cleanupBlockNumber_9', 'cleanupBlockNumber_3', 'cleanupBlockNumber_14'])
 balanceHistory.transmit(_from,_to,_amount)
LIBRARY_CALL, dest:CheckPointsByAddress, function:CheckPointsByAddress.transmit(CheckPointsByAddress.CheckPointsByAddressState,address,address,uint256), arguments:['balanceHistory_7', '_from_1', '_to_1', '_amount_1'] 
 balanceHistory.cleanupOldCheckpoints(_from,CLEANUP_COUNT,cleanupBlockNumber)
TMP_7590(uint256) = LIBRARY_CALL, dest:CheckPointsByAddress, function:CheckPointsByAddress.cleanupOldCheckpoints(CheckPointsByAddress.CheckPointsByAddressState,address,uint256,uint256), arguments:['balanceHistory_7', '_from_1', 'CLEANUP_COUNT_7', 'cleanupBlockNumber_7'] 
 balanceHistory.cleanupOldCheckpoints(_to,CLEANUP_COUNT,cleanupBlockNumber)
TMP_7591(uint256) = LIBRARY_CALL, dest:CheckPointsByAddress, function:CheckPointsByAddress.cleanupOldCheckpoints(CheckPointsByAddress.CheckPointsByAddressState,address,uint256,uint256), arguments:['balanceHistory_7', '_to_1', 'CLEANUP_COUNT_7', 'cleanupBlockNumber_7']
```
#### StorageSlot.getAddressSlot(bytes32) [INTERNAL]
```slithir
 r = slot
r_1 (-> ['slot'])(StorageSlot.AddressSlot) := slot_1(bytes32)
 r
RETURN r_1 (-> ['slot'])
```
#### EIP712._getEIP712State() [PRIVATE]
```slithir
EIP712_STORAGE_1(bytes32) := phi(['EIP712_STORAGE_0'])
 _state = EIP712_STORAGE
_state_1 (-> ['EIP712_STORAGE'])(EIP712.EIP712State) := EIP712_STORAGE_1(bytes32)
 _state
RETURN _state_1 (-> ['EIP712_STORAGE'])
```
#### CheckPointable.balanceOfAt(address,uint256) [PUBLIC]
```slithir
_owner_1(address) := phi(['_owner_1', '_owner_1'])
_blockNumber_1(uint256) := phi(['block.number'])
balanceHistory_1(CheckPointsByAddress.CheckPointsByAddressState) := phi(['balanceHistory_9', 'balanceHistory_2', 'balanceHistory_0', 'balanceHistory_4', 'balanceHistory_6'])
 balanceHistory.valueOfAt(_owner,_blockNumber)
TMP_7569(uint256) = LIBRARY_CALL, dest:CheckPointsByAddress, function:CheckPointsByAddress.valueOfAt(CheckPointsByAddress.CheckPointsByAddressState,address,uint256), arguments:['balanceHistory_2', '_owner_1', '_blockNumber_1'] 
RETURN TMP_7569
 notBeforeCleanupBlock(_blockNumber)
MODIFIER_CALL, CheckPointable.notBeforeCleanupBlock(uint256)(_blockNumber_1)
 _balance
```
#### CheckPointable.totalSupplyAt(uint256) [PUBLIC]
```slithir
_blockNumber_1(uint256) := phi(['block.number'])
totalSupply_7(CheckPointHistory.CheckPointHistoryState) := phi(['totalSupply_8', 'totalSupply_3', 'totalSupply_0', 'totalSupply_10', 'totalSupply_6'])
 totalSupply.valueAt(_blockNumber)
TMP_7587(uint256) = LIBRARY_CALL, dest:CheckPointHistory, function:CheckPointHistory.valueAt(CheckPointHistory.CheckPointHistoryState,uint256), arguments:['totalSupply_8', '_blockNumber_1'] 
RETURN TMP_7587
 notBeforeCleanupBlock(_blockNumber)
MODIFIER_CALL, CheckPointable.notBeforeCleanupBlock(uint256)(_blockNumber_1)
 _totalSupply
```
#### CheckPointHistory.cleanupOldCheckpoints(CheckPointHistory.CheckPointHistoryState,uint256,uint256) [INTERNAL]
```slithir
 _cleanupBlockNumber == 0
TMP_8004(bool) = _cleanupBlockNumber_1 == 0
CONDITION TMP_8004
 0
RETURN 0
 length = _self.endIndex
REF_5062(uint64) -> _self_1 (-> []).endIndex
length_1(uint256) := REF_5062(uint64)
 length == 0
TMP_8005(bool) = length_1 == 0
CONDITION TMP_8005
 0
RETURN 0
 startIndex = _self.startIndex
REF_5063(uint64) -> _self_1 (-> []).startIndex
startIndex_1(uint256) := REF_5063(uint64)
 endIndex = Math.min(startIndex + _count,length - 1)
TMP_8006(uint256) = startIndex_1 (c)+ _count_1
TMP_8007(uint256) = length_1 (c)- 1
TMP_8008(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['TMP_8006', 'TMP_8007'] 
endIndex_1(uint256) := TMP_8008(uint256)
 index = startIndex
index_1(uint256) := startIndex_1(uint256)
 index < endIndex && _self.checkpoints[index + 1].fromBlock <= _cleanupBlockNumber
_self_2 (-> [])(CheckPointHistory.CheckPointHistoryState) := phi(['_self_1 (-> [])', '_self_2 (-> [])'])
index_2(uint256) := phi(['index_3', 'index_1'])
TMP_8009(bool) = index_2 < endIndex_1
REF_5065(mapping(uint256 => CheckPointHistory.CheckPoint)) -> _self_2 (-> []).checkpoints
TMP_8010(uint256) = index_2 (c)+ 1
REF_5066(CheckPointHistory.CheckPoint) -> REF_5065[TMP_8010]
REF_5067(uint64) -> REF_5066.fromBlock
TMP_8011(bool) = REF_5067 <= _cleanupBlockNumber_1
TMP_8012(bool) = TMP_8009 && TMP_8011
CONDITION TMP_8012
 delete _self.checkpoints[index]
REF_5068(mapping(uint256 => CheckPointHistory.CheckPoint)) -> _self_2 (-> []).checkpoints
REF_5069(CheckPointHistory.CheckPoint) -> REF_5068[index_2]
REF_5068 = delete REF_5069 
 index ++
TMP_8013(uint256) := index_2(uint256)
index_3(uint256) = index_2 (c)+ 1
 index > startIndex
TMP_8014(bool) = index_2 > startIndex_1
CONDITION TMP_8014
 _self.startIndex = index.toUint64()
REF_5070(uint64) -> _self_2 (-> []).startIndex
TMP_8015(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['index_2'] 
_self_3 (-> [])(CheckPointHistory.CheckPointHistoryState) := phi(['_self_2 (-> [])'])
REF_5070(uint64) (->_self_3 (-> [])) := TMP_8015(uint64)
 index - startIndex
TMP_8016(uint256) = index_2 (c)- startIndex_1
RETURN TMP_8016
```
#### CheckPointHistory.writeValue(CheckPointHistory.CheckPointHistoryState,uint256) [INTERNAL]
```slithir
 historyCount = _self.endIndex
REF_5049(uint64) -> _self_1 (-> []).endIndex
historyCount_1(uint256) := REF_5049(uint64)
 historyCount == 0
TMP_7990(bool) = historyCount_1 == 0
CONDITION TMP_7990
 _self.checkpoints[0] = CheckPoint({fromBlock:block.number.toUint64(),value:_toUint192(_value)})
REF_5050(mapping(uint256 => CheckPointHistory.CheckPoint)) -> _self_1 (-> []).checkpoints
REF_5051(CheckPointHistory.CheckPoint) -> REF_5050[0]
TMP_7991(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['block.number'] 
TMP_7992(uint192) = INTERNAL_CALL, CheckPointHistory._toUint192(uint256)(_value_1)
TMP_7993(CheckPointHistory.CheckPoint) = new CheckPoint(TMP_7992,TMP_7991)
_self_4 (-> [])(CheckPointHistory.CheckPointHistoryState) := phi(['_self_1 (-> [])'])
REF_5051(CheckPointHistory.CheckPoint) (->_self_4 (-> [])) := TMP_7993(CheckPointHistory.CheckPoint)
 _self.endIndex = 1
REF_5053(uint64) -> _self_4 (-> []).endIndex
_self_5 (-> [])(CheckPointHistory.CheckPointHistoryState) := phi(['_self_4 (-> [])'])
REF_5053(uint64) (->_self_5 (-> [])) := 1(uint256)
 lastCheckpoint = _self.checkpoints[historyCount - 1]
REF_5054(mapping(uint256 => CheckPointHistory.CheckPoint)) -> _self_1 (-> []).checkpoints
TMP_7994(uint256) = historyCount_1 (c)- 1
REF_5055(CheckPointHistory.CheckPoint) -> REF_5054[TMP_7994]
lastCheckpoint_1 (-> ['_self'])(CheckPointHistory.CheckPoint) := REF_5055(CheckPointHistory.CheckPoint)
 lastBlock = lastCheckpoint.fromBlock
REF_5056(uint64) -> lastCheckpoint_1 (-> ['_self']).fromBlock
lastBlock_1(uint256) := REF_5056(uint64)
 block.number == lastBlock
TMP_7995(bool) = block.number == lastBlock_1
CONDITION TMP_7995
 lastCheckpoint.value = _toUint192(_value)
REF_5057(uint192) -> lastCheckpoint_1 (-> ['_self']).value
TMP_7996(uint192) = INTERNAL_CALL, CheckPointHistory._toUint192(uint256)(_value_1)
lastCheckpoint_2 (-> ['_self'])(CheckPointHistory.CheckPoint) := phi(["lastCheckpoint_1 (-> ['_self'])"])
REF_5057(uint192) (->lastCheckpoint_2 (-> ['_self'])) := TMP_7996(uint192)
_self_6 (-> ['_self'])(CheckPointHistory.CheckPointHistoryState) := phi(["lastCheckpoint_2 (-> ['_self'])"])
 assert(bool)(block.number > lastBlock)
TMP_7997(bool) = block.number > lastBlock_1
TMP_7998(None) = SOLIDITY_CALL assert(bool)(TMP_7997)
 _self.checkpoints[historyCount] = CheckPoint({fromBlock:block.number.toUint64(),value:_toUint192(_value)})
REF_5058(mapping(uint256 => CheckPointHistory.CheckPoint)) -> _self_1 (-> []).checkpoints
REF_5059(CheckPointHistory.CheckPoint) -> REF_5058[historyCount_1]
TMP_7999(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['block.number'] 
TMP_8000(uint192) = INTERNAL_CALL, CheckPointHistory._toUint192(uint256)(_value_1)
TMP_8001(CheckPointHistory.CheckPoint) = new CheckPoint(TMP_8000,TMP_7999)
_self_2 (-> [])(CheckPointHistory.CheckPointHistoryState) := phi(['_self_1 (-> [])'])
REF_5059(CheckPointHistory.CheckPoint) (->_self_2 (-> [])) := TMP_8001(CheckPointHistory.CheckPoint)
 _self.endIndex = uint64(historyCount + 1)
REF_5061(uint64) -> _self_2 (-> []).endIndex
TMP_8002(uint256) = historyCount_1 (c)+ 1
TMP_8003 = CONVERT TMP_8002 to uint64
_self_3 (-> [])(CheckPointHistory.CheckPointHistoryState) := phi(['_self_2 (-> [])'])
REF_5061(uint64) (->_self_3 (-> [])) := TMP_8003(uint64)
```

#### CheckPointsByAddress.writeValue(CheckPointsByAddress.CheckPointsByAddressState,address,uint256) [INTERNAL]
```slithir
_self_1 (-> [])(CheckPointsByAddress.CheckPointsByAddressState) := phi(['_self_1 (-> [])'])
_owner_1(address) := phi(['_from_1', '_to_1'])
_value_1(uint256) := phi(['newValueTo_1', 'newValueFrom_1'])
 history = _self.historyByAddress[_owner]
REF_5078(mapping(address => CheckPointHistory.CheckPointHistoryState)) -> _self_1 (-> []).historyByAddress
REF_5079(CheckPointHistory.CheckPointHistoryState) -> REF_5078[_owner_1]
history_1 (-> ['_self'])(CheckPointHistory.CheckPointHistoryState) := REF_5079(CheckPointHistory.CheckPointHistoryState)
 history.writeValue(_value)
LIBRARY_CALL, dest:CheckPointHistory, function:CheckPointHistory.writeValue(CheckPointHistory.CheckPointHistoryState,uint256), arguments:["history_1 (-> ['_self'])", '_value_1']
```
#### CheckPointsByAddress.transmit(CheckPointsByAddress.CheckPointsByAddressState,address,address,uint256) [INTERNAL]
```slithir
 _amount == 0
TMP_8022(bool) = _amount_1 == 0
CONDITION TMP_8022
 assert(bool)(! (_from == address(0) && _to == address(0)))
TMP_8023 = CONVERT 0 to address
TMP_8024(bool) = _from_1 == TMP_8023
TMP_8025 = CONVERT 0 to address
TMP_8026(bool) = _to_1 == TMP_8025
TMP_8027(bool) = TMP_8024 && TMP_8026
TMP_8028 = UnaryType.BANG TMP_8027 
TMP_8029(None) = SOLIDITY_CALL assert(bool)(TMP_8028)
 _from != address(0)
TMP_8030 = CONVERT 0 to address
TMP_8031(bool) = _from_1 != TMP_8030
CONDITION TMP_8031
 newValueFrom = valueOfAtNow(_self,_from) - _amount
TMP_8032(uint256) = INTERNAL_CALL, CheckPointsByAddress.valueOfAtNow(CheckPointsByAddress.CheckPointsByAddressState,address)(_self_1 (-> []),_from_1)
TMP_8033(uint256) = TMP_8032 (c)- _amount_1
newValueFrom_1(uint256) := TMP_8033(uint256)
 writeValue(_self,_from,newValueFrom)
INTERNAL_CALL, CheckPointsByAddress.writeValue(CheckPointsByAddress.CheckPointsByAddressState,address,uint256)(_self_1 (-> []),_from_1,newValueFrom_1)
 _to != address(0)
TMP_8035 = CONVERT 0 to address
TMP_8036(bool) = _to_1 != TMP_8035
CONDITION TMP_8036
 newValueTo = valueOfAtNow(_self,_to) + _amount
TMP_8037(uint256) = INTERNAL_CALL, CheckPointsByAddress.valueOfAtNow(CheckPointsByAddress.CheckPointsByAddressState,address)(_self_1 (-> []),_to_1)
TMP_8038(uint256) = TMP_8037 (c)+ _amount_1
newValueTo_1(uint256) := TMP_8038(uint256)
 writeValue(_self,_to,newValueTo)
INTERNAL_CALL, CheckPointsByAddress.writeValue(CheckPointsByAddress.CheckPointsByAddressState,address,uint256)(_self_1 (-> []),_to_1,newValueTo_1)
```
