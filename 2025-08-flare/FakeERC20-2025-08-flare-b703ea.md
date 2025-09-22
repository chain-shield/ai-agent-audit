
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
