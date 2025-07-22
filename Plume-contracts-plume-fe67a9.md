
#### Plume._authorizeUpgrade(address) [INTERNAL]
```slithir
newImplementation_1(address) := phi(['newImplementation_1'])
UPGRADER_ROLE_17(bytes32) := phi(['UPGRADER_ROLE_13', 'UPGRADER_ROLE_16', 'UPGRADER_ROLE_0', 'UPGRADER_ROLE_18'])
 onlyRole(UPGRADER_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(UPGRADER_ROLE_17)
```
#### Plume._update(address,address,uint256) [INTERNAL]
```slithir
from_1(address) := phi(['account_1', 'TMP_12129', 'from_1'])
to_1(address) := phi(['TMP_12135', 'account_1', 'to_1'])
value_1(uint256) := phi(['value_1', 'value_1', 'value_1'])
 super._update(from,to,value)
INTERNAL_CALL, ERC20PausableUpgradeable._update(address,address,uint256)(from_1,to_1,value_1)
```
#### Plume.burn(address,uint256) [PUBLIC]
```slithir
BURNER_ROLE_12(bytes32) := phi(['BURNER_ROLE_13', 'BURNER_ROLE_0', 'BURNER_ROLE_11'])
 _burn(from,amount)
INTERNAL_CALL, ERC20Upgradeable._burn(address,uint256)(from_1,amount_1)
 onlyRole(BURNER_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(BURNER_ROLE_12)
```
#### Plume.constructor() [PUBLIC]
```slithir
 _disableInitializers()
INTERNAL_CALL, Initializable._disableInitializers()()
```
#### Plume.initialize(address) [PUBLIC]
```slithir
DEFAULT_ADMIN_ROLE_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_9', 'DEFAULT_ADMIN_ROLE_0'])
UPGRADER_ROLE_1(bytes32) := phi(['UPGRADER_ROLE_13', 'UPGRADER_ROLE_16', 'UPGRADER_ROLE_0', 'UPGRADER_ROLE_18'])
MINTER_ROLE_1(bytes32) := phi(['MINTER_ROLE_10', 'MINTER_ROLE_12', 'MINTER_ROLE_0'])
BURNER_ROLE_1(bytes32) := phi(['BURNER_ROLE_13', 'BURNER_ROLE_0', 'BURNER_ROLE_11'])
PAUSER_ROLE_1(bytes32) := phi(['PAUSER_ROLE_16', 'PAUSER_ROLE_14', 'PAUSER_ROLE_0', 'PAUSER_ROLE_12'])
 __ERC20_init(Plume,PLUME)
INTERNAL_CALL, ERC20Upgradeable.__ERC20_init(string,string)(Plume,PLUME)
 __ERC20Burnable_init()
INTERNAL_CALL, ERC20BurnableUpgradeable.__ERC20Burnable_init()()
 __ERC20Pausable_init()
INTERNAL_CALL, ERC20PausableUpgradeable.__ERC20Pausable_init()()
 __AccessControl_init()
INTERNAL_CALL, AccessControlUpgradeable.__AccessControl_init()()
 __ERC20Permit_init(Plume)
INTERNAL_CALL, ERC20PermitUpgradeable.__ERC20Permit_init(string)(Plume)
 __UUPSUpgradeable_init()
INTERNAL_CALL, UUPSUpgradeable.__UUPSUpgradeable_init()()
 _grantRole(DEFAULT_ADMIN_ROLE,owner)
TMP_12235(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(DEFAULT_ADMIN_ROLE_8,owner_1)
 _grantRole(MINTER_ROLE,owner)
TMP_12236(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(MINTER_ROLE_9,owner_1)
 _grantRole(BURNER_ROLE,owner)
TMP_12237(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(BURNER_ROLE_10,owner_1)
 _grantRole(PAUSER_ROLE,owner)
TMP_12238(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(PAUSER_ROLE_11,owner_1)
 _grantRole(UPGRADER_ROLE,owner)
TMP_12239(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(UPGRADER_ROLE_12,owner_1)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### Plume.reinitialize() [PUBLIC]
```slithir
UPGRADER_ROLE_14(bytes32) := phi(['UPGRADER_ROLE_13', 'UPGRADER_ROLE_16', 'UPGRADER_ROLE_0', 'UPGRADER_ROLE_18'])
 __ERC20_init(Plume,PLUME)
INTERNAL_CALL, ERC20Upgradeable.__ERC20_init(string,string)(Plume,PLUME)
 reinitializer(1)
MODIFIER_CALL, Initializable.reinitializer(uint64)(1)
 onlyRole(UPGRADER_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(UPGRADER_ROLE_15)
```
#### Plume.mint(address,uint256) [EXTERNAL]
```slithir
MINTER_ROLE_11(bytes32) := phi(['MINTER_ROLE_10', 'MINTER_ROLE_12', 'MINTER_ROLE_0'])
 _mint(to,amount)
INTERNAL_CALL, ERC20Upgradeable._mint(address,uint256)(to_1,amount_1)
 onlyRole(MINTER_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(MINTER_ROLE_11)
```
#### Plume.pause() [EXTERNAL]
```slithir
PAUSER_ROLE_13(bytes32) := phi(['PAUSER_ROLE_16', 'PAUSER_ROLE_14', 'PAUSER_ROLE_0', 'PAUSER_ROLE_12'])
 _pause()
INTERNAL_CALL, PausableUpgradeable._pause()()
 onlyRole(PAUSER_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(PAUSER_ROLE_13)
```
#### Plume.unpause() [EXTERNAL]
```slithir
PAUSER_ROLE_15(bytes32) := phi(['PAUSER_ROLE_16', 'PAUSER_ROLE_14', 'PAUSER_ROLE_0', 'PAUSER_ROLE_12'])
 _unpause()
INTERNAL_CALL, PausableUpgradeable._unpause()()
 onlyRole(PAUSER_ROLE)
MODIFIER_CALL, AccessControlUpgradeable.onlyRole(bytes32)(PAUSER_ROLE_15)
```

