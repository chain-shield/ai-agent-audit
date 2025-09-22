
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
#### CollateralPoolToken._authorizeUpgrade(address) [INTERNAL]
```slithir
collateralPool_9(address) := phi(['collateralPool_8', 'collateralPool_1', 'collateralPool_3', 'collateralPool_10', 'collateralPool_0', 'collateralPool_5'])
 assetManager = IICollateralPool(collateralPool).assetManager()
TMP_6620 = CONVERT collateralPool_9 to IICollateralPool
TMP_6621(IIAssetManager) = HIGH_LEVEL_CALL, dest:TMP_6620(IICollateralPool), function:assetManager, arguments:[]  
collateralPool_10(address) := phi(['collateralPool_8', 'collateralPool_9', 'collateralPool_1', 'collateralPool_3', 'collateralPool_10', 'collateralPool_5'])
assetManager_1(IIAssetManager) := TMP_6621(IIAssetManager)
 require(bool,error)(msg.sender == address(assetManager),revert OnlyAssetManager()())
TMP_6622 = CONVERT assetManager_1 to address
TMP_6623(bool) = msg.sender == TMP_6622
TMP_6624(None) = SOLIDITY_CALL revert OnlyAssetManager()()
TMP_6625(None) = SOLIDITY_CALL require(bool,error)(TMP_6623,TMP_6624)
```
#### CollateralPoolToken._beforeTokenTransfer(address,address,uint256) [INTERNAL]
```slithir
_from_1(address) := phi(['TMP_6515', 'from_1', 'account_1'])
_amount_1(uint256) := phi(['amount_1', 'amount_1', 'amount_1'])
collateralPool_6(address) := phi(['collateralPool_8', 'collateralPool_1', 'collateralPool_3', 'collateralPool_10', 'collateralPool_0', 'collateralPool_5'])
ignoreTimelocked_3(bool) := phi(['ignoreTimelocked_4', 'ignoreTimelocked_1', 'ignoreTimelocked_0', 'ignoreTimelocked_2'])
 msg.sender != collateralPool
TMP_6584(bool) = msg.sender != collateralPool_6
CONDITION TMP_6584
 transferable = debtFreeBalanceOf(_from)
TMP_6585(uint256) = INTERNAL_CALL, CollateralPoolToken.debtFreeBalanceOf(address)(_from_1)
transferable_1(uint256) := TMP_6585(uint256)
 require(bool,error)(_amount <= transferable,revert InsufficientTransferableBalance()())
TMP_6586(bool) = _amount_1 <= transferable_1
TMP_6587(None) = SOLIDITY_CALL revert InsufficientTransferableBalance()()
TMP_6588(None) = SOLIDITY_CALL require(bool,error)(TMP_6586,TMP_6587)
 ! ignoreTimelocked && _from != address(0)
TMP_6589 = UnaryType.BANG ignoreTimelocked_4 
TMP_6590 = CONVERT 0 to address
TMP_6591(bool) = _from_1 != TMP_6590
TMP_6592(bool) = TMP_6589 && TMP_6591
CONDITION TMP_6592
 cleanupExpiredTimelocks(_from,10)
TMP_6593(bool) = INTERNAL_CALL, CollateralPoolToken.cleanupExpiredTimelocks(address,uint256)(_from_1,10)
 nonTimelocked = nonTimelockedBalanceOf(_from)
TMP_6594(uint256) = INTERNAL_CALL, CollateralPoolToken.nonTimelockedBalanceOf(address)(_from_1)
nonTimelocked_1(uint256) := TMP_6594(uint256)
 require(bool,error)(_amount <= nonTimelocked,revert InsufficientNonTimelockedBalance()())
TMP_6595(bool) = _amount_1 <= nonTimelocked_1
TMP_6596(None) = SOLIDITY_CALL revert InsufficientNonTimelockedBalance()()
TMP_6597(None) = SOLIDITY_CALL require(bool,error)(TMP_6595,TMP_6596)
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
#### CollateralPoolToken.cleanupExpiredTimelocks(address,uint256) [PUBLIC]
```slithir
_account_1(address) := phi(['_from_1'])
timelocksByAccount_8(mapping(address => CollateralPoolToken.TimelockQueue)) := phi(['timelocksByAccount_0', 'timelocksByAccount_4', 'timelocksByAccount_9', 'timelocksByAccount_7'])
 timelocks = timelocksByAccount[_account]
REF_4350(CollateralPoolToken.TimelockQueue) -> timelocksByAccount_8[_account_1]
timelocks_1 (-> ['timelocksByAccount'])(CollateralPoolToken.TimelockQueue) := REF_4350(CollateralPoolToken.TimelockQueue)
 start = timelocks.start
REF_4351(uint128) -> timelocks_1 (-> ['timelocksByAccount']).start
start_1(uint256) := REF_4351(uint128)
 count = 0
count_1(uint256) := 0(uint256)
 count < _maxTimelockedEntries
timelocks_2 (-> ['timelocksByAccount'])(CollateralPoolToken.TimelockQueue) := phi(["timelocks_2 (-> ['timelocksByAccount'])", "timelocks_1 (-> ['timelocksByAccount'])"])
start_2(uint256) := phi(['start_3', 'start_1'])
count_2(uint256) := phi(['count_1', 'count_3'])
TMP_6598(bool) = count_2 < _maxTimelockedEntries_1
CONDITION TMP_6598
 start >= timelocks.end || timelocks.data[start].endTime > block.timestamp
REF_4352(uint128) -> timelocks_2 (-> ['timelocksByAccount']).end
TMP_6599(bool) = start_2 >= REF_4352
REF_4353(mapping(uint256 => CollateralPoolToken.Timelock)) -> timelocks_2 (-> ['timelocksByAccount']).data
REF_4354(CollateralPoolToken.Timelock) -> REF_4353[start_2]
REF_4355(uint64) -> REF_4354.endTime
TMP_6600(bool) = REF_4355 > block.timestamp
TMP_6601(bool) = TMP_6599 || TMP_6600
CONDITION TMP_6601
 delete timelocks.data[start ++]
REF_4356(mapping(uint256 => CollateralPoolToken.Timelock)) -> timelocks_2 (-> ['timelocksByAccount']).data
TMP_6602(uint256) := start_2(uint256)
start_3(uint256) = start_2 (c)+ 1
REF_4357(CollateralPoolToken.Timelock) -> REF_4356[TMP_6602]
REF_4356 = delete REF_4357 
 count ++
TMP_6603(uint256) := count_2(uint256)
count_3(uint256) = count_2 (c)+ 1
 timelocks.start = start.toUint128()
REF_4358(uint128) -> timelocks_2 (-> ['timelocksByAccount']).start
TMP_6604(uint128) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint128(uint256), arguments:['start_2'] 
timelocks_3 (-> ['timelocksByAccount'])(CollateralPoolToken.TimelockQueue) := phi(["timelocks_2 (-> ['timelocksByAccount'])"])
REF_4358(uint128) (->timelocks_3 (-> ['timelocksByAccount'])) := TMP_6604(uint128)
timelocksByAccount_9(mapping(address => CollateralPoolToken.TimelockQueue)) := phi(["timelocks_3 (-> ['timelocksByAccount'])"])
 start >= timelocks.end || timelocks.data[start].endTime > block.timestamp
REF_4360(uint128) -> timelocks_3 (-> ['timelocksByAccount']).end
TMP_6605(bool) = start_2 >= REF_4360
REF_4361(mapping(uint256 => CollateralPoolToken.Timelock)) -> timelocks_3 (-> ['timelocksByAccount']).data
REF_4362(CollateralPoolToken.Timelock) -> REF_4361[start_2]
REF_4363(uint64) -> REF_4362.endTime
TMP_6606(bool) = REF_4363 > block.timestamp
TMP_6607(bool) = TMP_6605 || TMP_6606
RETURN TMP_6607
 _cleanedAllExpired
```
#### CollateralPoolToken.constructor(address,string,string) [PUBLIC]
```slithir
 initialize(_collateralPool,_tokenName,_tokenSymbol)
INTERNAL_CALL, CollateralPoolToken.initialize(address,string,string)(_collateralPool_1,_tokenName_1,_tokenSymbol_1)
 ERC20(_tokenName,_tokenSymbol)
INTERNAL_CALL, ERC20.constructor(string,string)(_tokenName_1,_tokenSymbol_1)
```
#### CollateralPoolToken.debtFreeBalanceOf(address) [PUBLIC]
```slithir
_account_1(address) := phi(['_account_1', '_from_1'])
collateralPool_2(address) := phi(['collateralPool_8', 'collateralPool_1', 'collateralPool_3', 'collateralPool_10', 'collateralPool_0', 'collateralPool_5'])
 IICollateralPool(collateralPool).debtFreeTokensOf(_account)
TMP_6572 = CONVERT collateralPool_2 to IICollateralPool
TMP_6573(uint256) = HIGH_LEVEL_CALL, dest:TMP_6572(IICollateralPool), function:debtFreeTokensOf, arguments:['_account_1']  
collateralPool_3(address) := phi(['collateralPool_8', 'collateralPool_1', 'collateralPool_3', 'collateralPool_10', 'collateralPool_5', 'collateralPool_2'])
RETURN TMP_6573
```
#### CollateralPoolToken.debtLockedBalanceOf(address) [PUBLIC]
```slithir
_account_1(address) := phi(['_account_1'])
collateralPool_4(address) := phi(['collateralPool_8', 'collateralPool_1', 'collateralPool_3', 'collateralPool_10', 'collateralPool_0', 'collateralPool_5'])
 IICollateralPool(collateralPool).debtLockedTokensOf(_account)
TMP_6574 = CONVERT collateralPool_4 to IICollateralPool
TMP_6575(uint256) = HIGH_LEVEL_CALL, dest:TMP_6574(IICollateralPool), function:debtLockedTokensOf, arguments:['_account_1']  
collateralPool_5(address) := phi(['collateralPool_8', 'collateralPool_4', 'collateralPool_1', 'collateralPool_3', 'collateralPool_10', 'collateralPool_5'])
RETURN TMP_6575
```
#### CollateralPoolToken.implementation() [EXTERNAL]
```slithir
 _getImplementation()
TMP_6619(address) = INTERNAL_CALL, ERC1967Upgrade._getImplementation()()
RETURN TMP_6619
```
#### CollateralPoolToken.initialize(address,string,string) [PUBLIC]
```slithir
_collateralPool_1(address) := phi(['_collateralPool_1'])
_tokenName_1(string) := phi(['_tokenName_1'])
_tokenSymbol_1(string) := phi(['_tokenSymbol_1'])
initialized_1(bool) := phi(['initialized_2', 'initialized_0'])
 require(bool,error)(! initialized,revert AlreadyInitialized()())
TMP_6550 = UnaryType.BANG initialized_1 
TMP_6551(None) = SOLIDITY_CALL revert AlreadyInitialized()()
TMP_6552(None) = SOLIDITY_CALL require(bool,error)(TMP_6550,TMP_6551)
 initialized = true
initialized_2(bool) := True(bool)
 collateralPool = _collateralPool
collateralPool_1(address) := _collateralPool_1(address)
 tokenName = _tokenName
tokenName_1(string) := _tokenName_1(string)
 tokenSymbol = _tokenSymbol
tokenSymbol_1(string) := _tokenSymbol_1(string)
```
#### CollateralPoolToken.lockedBalanceOf(address) [EXTERNAL]
```slithir
 debtLockedBalance = debtLockedBalanceOf(_account)
TMP_6566(uint256) = INTERNAL_CALL, CollateralPoolToken.debtLockedBalanceOf(address)(_account_1)
debtLockedBalance_1(uint256) := TMP_6566(uint256)
 timelockedBalance = timelockedBalanceOf(_account)
TMP_6567(uint256) = INTERNAL_CALL, CollateralPoolToken.timelockedBalanceOf(address)(_account_1)
timelockedBalance_1(uint256) := TMP_6567(uint256)
 (debtLockedBalance > timelockedBalance)
TMP_6568(bool) = debtLockedBalance_1 > timelockedBalance_1
CONDITION TMP_6568
 debtLockedBalance
RETURN debtLockedBalance_1
 timelockedBalance
RETURN timelockedBalance_1
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
#### CollateralPoolToken.name() [PUBLIC]
```slithir
tokenName_2(string) := phi(['tokenName_0', 'tokenName_1'])
 tokenName
RETURN tokenName_2
```
#### CollateralPoolToken.nonTimelockedBalanceOf(address) [PUBLIC]
```slithir
_account_1(address) := phi(['_account_1', '_from_1'])
 balanceOf(_account) - timelockedBalanceOf(_account)
TMP_6581(uint256) = INTERNAL_CALL, ERC20.balanceOf(address)(_account_1)
TMP_6582(uint256) = INTERNAL_CALL, CollateralPoolToken.timelockedBalanceOf(address)(_account_1)
TMP_6583(uint256) = TMP_6581 (c)- TMP_6582
RETURN TMP_6583
```

#### CollateralPoolToken.supportsInterface(bytes4) [EXTERNAL]
```slithir
 _interfaceId == type()(IERC165).interfaceId || _interfaceId == type()(IERC20).interfaceId || _interfaceId == type()(ICollateralPoolToken).interfaceId
TMP_6611(type(IERC165)) = SOLIDITY_CALL type()(IERC165)
REF_4366(bytes4) (->None) := 33540519(bytes4)
TMP_6612(bool) = _interfaceId_1 == REF_4366
TMP_6613(type(IERC20)) = SOLIDITY_CALL type()(IERC20)
REF_4367(bytes4) (->None) := 909585159(bytes4)
TMP_6614(bool) = _interfaceId_1 == REF_4367
TMP_6615(bool) = TMP_6612 || TMP_6614
TMP_6616(type(ICollateralPoolToken)) = SOLIDITY_CALL type()(ICollateralPoolToken)
REF_4368(bytes4) (->None) := 3007241369(bytes4)
TMP_6617(bool) = _interfaceId_1 == REF_4368
TMP_6618(bool) = TMP_6615 || TMP_6617
RETURN TMP_6618
```
#### CollateralPoolToken.symbol() [PUBLIC]
```slithir
tokenSymbol_2(string) := phi(['tokenSymbol_1', 'tokenSymbol_0'])
 tokenSymbol
RETURN tokenSymbol_2
```
#### CollateralPoolToken.timelockedBalanceOf(address) [PUBLIC]
```slithir
_account_1(address) := phi(['_account_1', '_account_1'])
timelocksByAccount_7(mapping(address => CollateralPoolToken.TimelockQueue)) := phi(['timelocksByAccount_0', 'timelocksByAccount_4', 'timelocksByAccount_9', 'timelocksByAccount_7'])
 timelocks = timelocksByAccount[_account]
REF_4343(CollateralPoolToken.TimelockQueue) -> timelocksByAccount_7[_account_1]
timelocks_1 (-> ['timelocksByAccount'])(CollateralPoolToken.TimelockQueue) := REF_4343(CollateralPoolToken.TimelockQueue)
 end = timelocks.end
REF_4344(uint128) -> timelocks_1 (-> ['timelocksByAccount']).end
end_1(uint256) := REF_4344(uint128)
 i = timelocks.start
REF_4345(uint128) -> timelocks_1 (-> ['timelocksByAccount']).start
i_1(uint256) := REF_4345(uint128)
 i < end
i_2(uint256) := phi(['i_1', 'i_3'])
TMP_6576(bool) = i_2 < end_1
CONDITION TMP_6576
 timelock = timelocks.data[i]
REF_4346(mapping(uint256 => CollateralPoolToken.Timelock)) -> timelocks_1 (-> ['timelocksByAccount']).data
REF_4347(CollateralPoolToken.Timelock) -> REF_4346[i_2]
timelock_1 (-> ['timelocks'])(CollateralPoolToken.Timelock) := REF_4347(CollateralPoolToken.Timelock)
 timelock.endTime > block.timestamp
REF_4348(uint64) -> timelock_1 (-> ['timelocks']).endTime
TMP_6577(bool) = REF_4348 > block.timestamp
CONDITION TMP_6577
 _timelocked += timelock.amount
REF_4349(uint128) -> timelock_1 (-> ['timelocks']).amount
_timelocked_1(uint256) = _timelocked_0 (c)+ REF_4349
_timelocked_2(uint256) := phi(['_timelocked_0', '_timelocked_1'])
 i ++
TMP_6578(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 totalBalance = balanceOf(_account)
TMP_6579(uint256) = INTERNAL_CALL, ERC20.balanceOf(address)(_account_1)
totalBalance_1(uint256) := TMP_6579(uint256)
 (_timelocked < totalBalance)
TMP_6580(bool) = _timelocked_0 < totalBalance_1
CONDITION TMP_6580
 _timelocked = _timelocked
_timelocked_3(uint256) := _timelocked_0(uint256)
 _timelocked = totalBalance
_timelocked_4(uint256) := totalBalance_1(uint256)
_timelocked_5(uint256) := phi(['_timelocked_3', '_timelocked_4'])
 _timelocked
RETURN _timelocked_5
```
#### CollateralPoolToken.transferableBalanceOf(address) [EXTERNAL]
```slithir
 debtFreeBalance = debtFreeBalanceOf(_account)
TMP_6569(uint256) = INTERNAL_CALL, CollateralPoolToken.debtFreeBalanceOf(address)(_account_1)
debtFreeBalance_1(uint256) := TMP_6569(uint256)
 nonTimelockedBalance = nonTimelockedBalanceOf(_account)
TMP_6570(uint256) = INTERNAL_CALL, CollateralPoolToken.nonTimelockedBalanceOf(address)(_account_1)
nonTimelockedBalance_1(uint256) := TMP_6570(uint256)
 (debtFreeBalance < nonTimelockedBalance)
TMP_6571(bool) = debtFreeBalance_1 < nonTimelockedBalance_1
CONDITION TMP_6571
 debtFreeBalance
RETURN debtFreeBalance_1
 nonTimelockedBalance
RETURN nonTimelockedBalance_1
```

#### AssetManagerMock.getCollateralPoolTokenTimelockSeconds() [EXTERNAL]
```slithir
timelockDuration_1(uint256) := phi(['timelockDuration_0', 'timelockDuration_2'])
 timelockDuration
RETURN timelockDuration_1
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
#### ERC1967Upgrade._getImplementation() [INTERNAL]
```slithir
_IMPLEMENTATION_SLOT_1(bytes32) := phi(['_IMPLEMENTATION_SLOT_4', '_IMPLEMENTATION_SLOT_0'])
 StorageSlot.getAddressSlot(_IMPLEMENTATION_SLOT).value
TMP_60(StorageSlot.AddressSlot) = LIBRARY_CALL, dest:StorageSlot, function:StorageSlot.getAddressSlot(bytes32), arguments:['_IMPLEMENTATION_SLOT_1'] 
REF_26(address) -> TMP_60.value
RETURN REF_26
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
#### ERC20.balanceOf(address) [PUBLIC]
```slithir
_balances_1(mapping(address => uint256)) := phi(['_balances_11', '_balances_1', '_balances_5', '_balances_8', '_balances_0'])
 _balances[account]
REF_73(uint256) -> _balances_1[account_1]
RETURN REF_73
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
#### StorageSlot.getAddressSlot(bytes32) [INTERNAL]
```slithir
 r = slot
r_1 (-> ['slot'])(StorageSlot.AddressSlot) := slot_1(bytes32)
 r
RETURN r_1 (-> ['slot'])
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

