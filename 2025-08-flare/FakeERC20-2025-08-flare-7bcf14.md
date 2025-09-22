
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
#### FakeERC20.burnAmount(uint256) [PUBLIC]
```slithir
 _burn(msg.sender,_amount)
INTERNAL_CALL, ERC20._burn(address,uint256)(msg.sender,_amount_1)
```
#### FakeERC20.constructor(IGovernanceSettings,address,string,string,uint8) [PUBLIC]
```slithir
 decimals_ = _decimals
decimals__1(uint8) := _decimals_1(uint8)
 ERC20(_name,_symbol)
INTERNAL_CALL, ERC20.constructor(string,string)(_name_1,_symbol_1)
 Governed(_governanceSettings,_initialGovernance)
INTERNAL_CALL, Governed.constructor(IGovernanceSettings,address)(_governanceSettings_1,_initialGovernance_1)
```
#### FakeERC20.decimals() [PUBLIC]
```slithir
decimals__2(uint8) := phi(['decimals__1', 'decimals__0'])
 decimals_
RETURN decimals__2
```
#### FakeERC20.mintAmount(address,uint256) [PUBLIC]
```slithir
 _mint(_target,amount)
INTERNAL_CALL, ERC20._mint(address,uint256)(_target_1,amount_1)
 onlyGovernance()
MODIFIER_CALL, GovernedBase.onlyGovernance()()
```
#### IERC165.supportsInterface(bytes4) [EXTERNAL]
```slithir

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
