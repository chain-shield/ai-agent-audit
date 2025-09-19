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

### Storage layout (WNatMock) 

```text
governanceVP IGovernanceVotePower
delegations mapping(address => WNatMock.Delegation[])
delegators mapping(address => EnumerableSet.AddressSet)

```
#### Transfers.depositWNat(IWNat,address,uint256) [INTERNAL]
```slithir
 _amount > 0
TMP_10540(bool) = _amount_1 > 0
CONDITION TMP_10540
 _wNat.depositTo{value: _amount}(_recipient)
HIGH_LEVEL_CALL, dest:_wNat_1(IWNat), function:depositTo, arguments:['_recipient_1'] value:_amount_1
```
#### SafePct.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 MAX_BIPS = 10_000
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
#### WNatMock.depositTo(address) [PUBLIC]
```slithir
 _mint(_recipient,msg.value)
INTERNAL_CALL, ERC20._mint(address,uint256)(_recipient_1,msg.value)
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
