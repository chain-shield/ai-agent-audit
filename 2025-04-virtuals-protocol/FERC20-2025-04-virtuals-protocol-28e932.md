### Storage layout (FERC20) 

```text
_totalSupply uint256
_name string
_symbol string
maxTx uint256
_maxTxAmount uint256
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
isExcludedFromMaxTx mapping(address => bool)

```
#### FERC20._approve(address,address,uint256) [PRIVATE]
```slithir
owner_1(address) := phi(['TMP_8221', 'sender_1'])
spender_1(address) := phi(['TMP_8224', 'spender_1'])
amount_1(uint256) := phi(['amount_1', 'TMP_8226'])
 require(bool,string)(owner != address(0),ERC20: approve from the zero address)
TMP_8228 = CONVERT 0 to address
TMP_8229(bool) = owner_1 != TMP_8228
TMP_8230(None) = SOLIDITY_CALL require(bool,string)(TMP_8229,ERC20: approve from the zero address)
 require(bool,string)(spender != address(0),ERC20: approve to the zero address)
TMP_8231 = CONVERT 0 to address
TMP_8232(bool) = spender_1 != TMP_8231
TMP_8233(None) = SOLIDITY_CALL require(bool,string)(TMP_8232,ERC20: approve to the zero address)
 _allowances[owner][spender] = amount
REF_3329(mapping(address => uint256)) -> _allowances_6[owner_1]
REF_3330(uint256) -> REF_3329[spender_1]
_allowances_7(mapping(address => mapping(address => uint256))) := phi(['_allowances_6'])
REF_3330(uint256) (->_allowances_7) := amount_1(uint256)
 Approval(owner,spender,amount)
Emit Approval(owner_1,spender_1,amount_1)
```
#### FERC20._burn(address,uint256) [INTERNAL]
```slithir
_balances_6(mapping(address => uint256)) := phi(['_balances_7', '_balances_1', '_balances_0', '_balances_5', '_balances_2', '_balances_10'])
 require(bool,string)(user != address(0),Invalid address)
TMP_8258 = CONVERT 0 to address
TMP_8259(bool) = user_1 != TMP_8258
TMP_8260(None) = SOLIDITY_CALL require(bool,string)(TMP_8259,Invalid address)
 _balances[user] = _balances[user] - amount
REF_3337(uint256) -> _balances_6[user_1]
REF_3338(uint256) -> _balances_6[user_1]
TMP_8261(uint256) = REF_3338 (c)- amount_1
_balances_7(mapping(address => uint256)) := phi(['_balances_6'])
REF_3337(uint256) (->_balances_7) := TMP_8261(uint256)
```
#### FERC20._transfer(address,address,uint256) [PRIVATE]
```slithir
from_1(address) := phi(['TMP_8219', 'sender_1'])
to_1(address) := phi(['recipient_1', 'recipient_1'])
amount_1(uint256) := phi(['amount_1', 'amount_1'])
_maxTxAmount_1(uint256) := phi(['_maxTxAmount_0', '_maxTxAmount_2'])
_balances_3(mapping(address => uint256)) := phi(['_balances_7', '_balances_1', '_balances_0', '_balances_5', '_balances_2', '_balances_10'])
isExcludedFromMaxTx_3(mapping(address => bool)) := phi(['isExcludedFromMaxTx_2', 'isExcludedFromMaxTx_0', 'isExcludedFromMaxTx_3', 'isExcludedFromMaxTx_4'])
 require(bool,string)(from != address(0),ERC20: transfer from the zero address)
TMP_8235 = CONVERT 0 to address
TMP_8236(bool) = from_1 != TMP_8235
TMP_8237(None) = SOLIDITY_CALL require(bool,string)(TMP_8236,ERC20: transfer from the zero address)
 require(bool,string)(to != address(0),ERC20: transfer to the zero address)
TMP_8238 = CONVERT 0 to address
TMP_8239(bool) = to_1 != TMP_8238
TMP_8240(None) = SOLIDITY_CALL require(bool,string)(TMP_8239,ERC20: transfer to the zero address)
 require(bool,string)(amount > 0,Transfer amount must be greater than zero)
TMP_8241(bool) = amount_1 > 0
TMP_8242(None) = SOLIDITY_CALL require(bool,string)(TMP_8241,Transfer amount must be greater than zero)
 ! isExcludedFromMaxTx[from]
REF_3331(bool) -> isExcludedFromMaxTx_3[from_1]
TMP_8243 = UnaryType.BANG REF_3331 
CONDITION TMP_8243
 require(bool,string)(amount <= _maxTxAmount,Exceeds MaxTx)
TMP_8244(bool) = amount_1 <= _maxTxAmount_1
TMP_8245(None) = SOLIDITY_CALL require(bool,string)(TMP_8244,Exceeds MaxTx)
 _balances[from] = _balances[from] - amount
REF_3332(uint256) -> _balances_3[from_1]
REF_3333(uint256) -> _balances_3[from_1]
TMP_8246(uint256) = REF_3333 (c)- amount_1
_balances_4(mapping(address => uint256)) := phi(['_balances_3'])
REF_3332(uint256) (->_balances_4) := TMP_8246(uint256)
 _balances[to] = _balances[to] + amount
REF_3334(uint256) -> _balances_4[to_1]
REF_3335(uint256) -> _balances_4[to_1]
TMP_8247(uint256) = REF_3335 (c)+ amount_1
_balances_5(mapping(address => uint256)) := phi(['_balances_4'])
REF_3334(uint256) (->_balances_5) := TMP_8247(uint256)
 Transfer(from,to,amount)
Emit Transfer(from_1,to_1,amount_1)
```
#### FERC20._updateMaxTx(uint256) [INTERNAL]
```slithir
_maxTx_1(uint256) := phi(['_maxTx_1', '_maxTx_1'])
_totalSupply_7(uint256) := phi(['_totalSupply_0', '_totalSupply_5'])
 maxTx = _maxTx
maxTx_1(uint256) := _maxTx_1(uint256)
 _maxTxAmount = (maxTx * _totalSupply) / 100
TMP_8249(uint256) = maxTx_1 (c)* _totalSupply_7
TMP_8250(uint256) = TMP_8249 (c)/ 100
_maxTxAmount_2(uint256) := TMP_8250(uint256)
 MaxTxUpdated(_maxTx)
Emit MaxTxUpdated(_maxTx_1)
```
#### FERC20.allowance(address,address) [PUBLIC]
```slithir
_allowances_1(mapping(address => mapping(address => uint256))) := phi(['_allowances_6', '_allowances_1', '_allowances_7', '_allowances_0'])
 _allowances[owner][spender]
REF_3325(mapping(address => uint256)) -> _allowances_1[owner_1]
REF_3326(uint256) -> REF_3325[spender_1]
RETURN REF_3326
```
#### FERC20.approve(address,uint256) [PUBLIC]
```slithir
 _approve(_msgSender(),spender,amount)
TMP_8221(address) = INTERNAL_CALL, Context._msgSender()()
INTERNAL_CALL, FERC20._approve(address,address,uint256)(TMP_8221,spender_1,amount_1)
 true
RETURN True
```
#### FERC20.balanceOf(address) [PUBLIC]
```slithir
_balances_2(mapping(address => uint256)) := phi(['_balances_7', '_balances_1', '_balances_0', '_balances_5', '_balances_2', '_balances_10'])
 _balances[account]
REF_3324(uint256) -> _balances_2[account_1]
RETURN REF_3324
```
#### FERC20.burnFrom(address,uint256) [PUBLIC][OWNER]
```slithir
_balances_8(mapping(address => uint256)) := phi(['_balances_7', '_balances_1', '_balances_0', '_balances_5', '_balances_2', '_balances_10'])
 require(bool,string)(user != address(0),Invalid address)
TMP_8262 = CONVERT 0 to address
TMP_8263(bool) = user_1 != TMP_8262
TMP_8264(None) = SOLIDITY_CALL require(bool,string)(TMP_8263,Invalid address)
 _balances[user] = _balances[user] - amount
REF_3339(uint256) -> _balances_9[user_1]
REF_3340(uint256) -> _balances_9[user_1]
TMP_8265(uint256) = REF_3340 (c)- amount_1
_balances_10(mapping(address => uint256)) := phi(['_balances_9'])
REF_3339(uint256) (->_balances_10) := TMP_8265(uint256)
 Transfer(user,address(0),amount)
TMP_8266 = CONVERT 0 to address
Emit Transfer(user_1,TMP_8266,amount_1)
 onlyOwner()
MODIFIER_CALL, Ownable.onlyOwner()()
```
#### FERC20.constructor(string,string,uint256,uint256) [PUBLIC]
```slithir
_decimals_1(uint8) := phi(['_decimals_2', '_decimals_0'])
 _name = name_
_name_1(string) := name__1(string)
 _symbol = symbol_
_symbol_1(string) := symbol__1(string)
 _totalSupply = supply * 10 ** _decimals
TMP_8209(uint256) = 10 (c)** _decimals_2
TMP_8210(uint256) = supply_1 (c)* TMP_8209
_totalSupply_1(uint256) := TMP_8210(uint256)
 _balances[_msgSender()] = _totalSupply
TMP_8211(address) = INTERNAL_CALL, Context._msgSender()()
REF_3321(uint256) -> _balances_0[TMP_8211]
_balances_1(mapping(address => uint256)) := phi(['_balances_0'])
REF_3321(uint256) (->_balances_1) := _totalSupply_2(uint256)
 isExcludedFromMaxTx[_msgSender()] = true
TMP_8212(address) = INTERNAL_CALL, Context._msgSender()()
REF_3322(bool) -> isExcludedFromMaxTx_0[TMP_8212]
isExcludedFromMaxTx_1(mapping(address => bool)) := phi(['isExcludedFromMaxTx_0'])
REF_3322(bool) (->isExcludedFromMaxTx_1) := True(bool)
 isExcludedFromMaxTx[address(this)] = true
TMP_8213 = CONVERT this to address
REF_3323(bool) -> isExcludedFromMaxTx_1[TMP_8213]
isExcludedFromMaxTx_2(mapping(address => bool)) := phi(['isExcludedFromMaxTx_1'])
REF_3323(bool) (->isExcludedFromMaxTx_2) := True(bool)
 _updateMaxTx(_maxTx)
INTERNAL_CALL, FERC20._updateMaxTx(uint256)(_maxTx_1)
 Transfer(address(0),_msgSender(),_totalSupply)
TMP_8215 = CONVERT 0 to address
TMP_8216(address) = INTERNAL_CALL, Context._msgSender()()
Emit Transfer(TMP_8215,TMP_8216,_totalSupply_5)
 Ownable(msg.sender)
INTERNAL_CALL, Ownable.constructor(address)(msg.sender)
```
#### FERC20.decimals() [PUBLIC]
```slithir
_decimals_3(uint8) := phi(['_decimals_2', '_decimals_0'])
 _decimals
RETURN _decimals_3
```
#### FERC20.excludeFromMaxTx(address) [PUBLIC][OWNER]
```slithir
 require(bool,string)(user != address(0),ERC20: Exclude Max Tx from the zero address)
TMP_8254 = CONVERT 0 to address
TMP_8255(bool) = user_1 != TMP_8254
TMP_8256(None) = SOLIDITY_CALL require(bool,string)(TMP_8255,ERC20: Exclude Max Tx from the zero address)
 isExcludedFromMaxTx[user] = true
REF_3336(bool) -> isExcludedFromMaxTx_3[user_1]
isExcludedFromMaxTx_4(mapping(address => bool)) := phi(['isExcludedFromMaxTx_3'])
REF_3336(bool) (->isExcludedFromMaxTx_4) := True(bool)
 onlyOwner()
MODIFIER_CALL, Ownable.onlyOwner()()
```
#### FERC20.name() [PUBLIC]
```slithir
_name_2(string) := phi(['_name_1', '_name_0'])
 _name
RETURN _name_2
```
#### Bonding.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 K = 3_000_000_000_000
 _checkOwner()
INTERNAL_CALL, OwnableUpgradeable._checkOwner()()
 $ = _getInitializableStorage()
TMP_8167(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_8167'])(Initializable.InitializableStorage) := TMP_8167(Initializable.InitializableStorage)
 isTopLevelCall = ! $._initializing
REF_3310(bool) -> $_1 (-> ['TMP_8167'])._initializing
TMP_8168 = UnaryType.BANG REF_3310 
isTopLevelCall_1(bool) := TMP_8168(bool)
 initialized = $._initialized
REF_3311(uint64) -> $_1 (-> ['TMP_8167'])._initialized
initialized_1(uint64) := REF_3311(uint64)
 initialSetup = initialized == 0 && isTopLevelCall
TMP_8169(bool) = initialized_1 == 0
TMP_8170(bool) = TMP_8169 && isTopLevelCall_1
initialSetup_1(bool) := TMP_8170(bool)
 construction = initialized == 1 && address(this).code.length == 0
TMP_8171(bool) = initialized_1 == 1
TMP_8172 = CONVERT this to address
TMP_8173(bytes) = SOLIDITY_CALL code(address)(TMP_8172)
REF_3312 -> LENGTH TMP_8173
TMP_8174(bool) = REF_3312 == 0
TMP_8175(bool) = TMP_8171 && TMP_8174
construction_1(bool) := TMP_8175(bool)
 ! initialSetup && ! construction
TMP_8176 = UnaryType.BANG initialSetup_1 
TMP_8177 = UnaryType.BANG construction_1 
TMP_8178(bool) = TMP_8176 && TMP_8177
CONDITION TMP_8178
 revert InvalidInitialization()()
TMP_8179(None) = SOLIDITY_CALL revert InvalidInitialization()()
 $._initialized = 1
REF_3313(uint64) -> $_1 (-> ['TMP_8167'])._initialized
$_2 (-> ['TMP_8167'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_8167'])"])
REF_3313(uint64) (->$_2 (-> ['TMP_8167'])) := 1(uint256)
TMP_8167(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_8167'])"])
 isTopLevelCall
CONDITION isTopLevelCall_1
 $._initializing = true
REF_3314(bool) -> $_2 (-> ['TMP_8167'])._initializing
$_3 (-> ['TMP_8167'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_8167'])"])
REF_3314(bool) (->$_3 (-> ['TMP_8167'])) := True(bool)
TMP_8167(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_8167'])"])
$_4 (-> ['TMP_8167'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_8167'])", "$_2 (-> ['TMP_8167'])"])
 isTopLevelCall
CONDITION isTopLevelCall_1
 $._initializing = false
REF_3315(bool) -> $_4 (-> ['TMP_8167'])._initializing
$_5 (-> ['TMP_8167'])(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_8167'])"])
REF_3315(bool) (->$_5 (-> ['TMP_8167'])) := False(bool)
TMP_8167(Initializable.InitializableStorage) := phi(["$_5 (-> ['TMP_8167'])"])
 Initialized(1)
Emit Initialized(1)
 $ = _getInitializableStorage()
TMP_8181(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_8181'])(Initializable.InitializableStorage) := TMP_8181(Initializable.InitializableStorage)
 $._initializing || $._initialized >= version
REF_3316(bool) -> $_1 (-> ['TMP_8181'])._initializing
REF_3317(uint64) -> $_1 (-> ['TMP_8181'])._initialized
TMP_8182(bool) = REF_3317 >= version_1
TMP_8183(bool) = REF_3316 || TMP_8182
CONDITION TMP_8183
 revert InvalidInitialization()()
TMP_8184(None) = SOLIDITY_CALL revert InvalidInitialization()()
 $._initialized = version
REF_3318(uint64) -> $_1 (-> ['TMP_8181'])._initialized
$_2 (-> ['TMP_8181'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_8181'])"])
REF_3318(uint64) (->$_2 (-> ['TMP_8181'])) := version_1(uint64)
TMP_8181(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_8181'])"])
 $._initializing = true
REF_3319(bool) -> $_2 (-> ['TMP_8181'])._initializing
$_3 (-> ['TMP_8181'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_8181'])"])
REF_3319(bool) (->$_3 (-> ['TMP_8181'])) := True(bool)
TMP_8181(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_8181'])"])
 $._initializing = false
REF_3320(bool) -> $_3 (-> ['TMP_8181'])._initializing
$_4 (-> ['TMP_8181'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_8181'])"])
REF_3320(bool) (->$_4 (-> ['TMP_8181'])) := False(bool)
TMP_8181(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_8181'])"])
 Initialized(version)
Emit Initialized(version_1)
 _checkInitializing()
INTERNAL_CALL, Initializable._checkInitializing()()
 _nonReentrantBefore()
INTERNAL_CALL, ReentrancyGuardUpgradeable._nonReentrantBefore()()
 _nonReentrantAfter()
INTERNAL_CALL, ReentrancyGuardUpgradeable._nonReentrantAfter()()
```
#### FERC20.symbol() [PUBLIC]
```slithir
_symbol_2(string) := phi(['_symbol_1', '_symbol_0'])
 _symbol
RETURN _symbol_2
```
#### FERC20.totalSupply() [PUBLIC]
```slithir
_totalSupply_6(uint256) := phi(['_totalSupply_0', '_totalSupply_5'])
 _totalSupply
RETURN _totalSupply_6
```
#### FERC20.transfer(address,uint256) [PUBLIC]
```slithir
 _transfer(_msgSender(),recipient,amount)
TMP_8219(address) = INTERNAL_CALL, Context._msgSender()()
INTERNAL_CALL, FERC20._transfer(address,address,uint256)(TMP_8219,recipient_1,amount_1)
 true
RETURN True
```
#### FERC20.transferFrom(address,address,uint256) [PUBLIC]
```slithir
_allowances_2(mapping(address => mapping(address => uint256))) := phi(['_allowances_6', '_allowances_1', '_allowances_7', '_allowances_0'])
 _transfer(sender,recipient,amount)
INTERNAL_CALL, FERC20._transfer(address,address,uint256)(sender_1,recipient_1,amount_1)
 _approve(sender,_msgSender(),_allowances[sender][_msgSender()] - amount)
TMP_8224(address) = INTERNAL_CALL, Context._msgSender()()
REF_3327(mapping(address => uint256)) -> _allowances_4[sender_1]
TMP_8225(address) = INTERNAL_CALL, Context._msgSender()()
REF_3328(uint256) -> REF_3327[TMP_8225]
TMP_8226(uint256) = REF_3328 (c)- amount_1
INTERNAL_CALL, FERC20._approve(address,address,uint256)(sender_1,TMP_8224,TMP_8226)
_allowances_6(mapping(address => mapping(address => uint256))) := phi(['_allowances_7'])
 true
RETURN True
```
#### FERC20.updateMaxTx(uint256) [PUBLIC][OWNER]
```slithir
 _updateMaxTx(_maxTx)
INTERNAL_CALL, FERC20._updateMaxTx(uint256)(_maxTx_1)
 onlyOwner()
MODIFIER_CALL, Ownable.onlyOwner()()
```
