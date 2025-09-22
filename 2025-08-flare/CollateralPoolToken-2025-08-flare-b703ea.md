

### Storage layout (CollateralPoolToken) 

```text
collateralPool address
tokenName string
tokenSymbol string
timelocksByAccount mapping(address => CollateralPoolToken.TimelockQueue)
ignoreTimelocked bool
initialized bool

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

#### IIAssetManager.getCollateralPoolTokenTimelockSeconds() [EXTERNAL]
```slithir

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
#### IICollateralPool.debtFreeTokensOf(address) [EXTERNAL]
```slithir

```
#### IICollateralPool.debtLockedTokensOf(address) [EXTERNAL]
```slithir

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
