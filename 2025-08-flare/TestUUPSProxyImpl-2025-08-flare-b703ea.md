### Storage layout (TestUUPSProxyImpl) 

```text
_dummy uint256[1000]
message string
initialized bool

```
#### TestUUPSProxyImpl._authorizeUpgrade(address) [INTERNAL]
```slithir
newImplementation_1(address) := phi(['newImplementation_1', 'newImplementation_1'])
```
#### TestUUPSProxyImpl.implementation() [EXTERNAL]
```slithir
 _getImplementation()
TMP_10850(address) = INTERNAL_CALL, ERC1967Upgrade._getImplementation()()
RETURN TMP_10850
```
#### TestUUPSProxyImpl.initialize(string) [EXTERNAL]
```slithir
 message = _message
message_1(string) := _message_1(string)
 initialized = true
initialized_1(bool) := True(bool)
```

#### TestUUPSProxyImpl.testResult() [EXTERNAL]
```slithir
message_2(string) := phi(['message_0', 'message_1'])
initialized_2(bool) := phi(['initialized_0', 'initialized_1'])
 initialized
CONDITION initialized_2
 message
RETURN message_2
 test proxy
RETURN test proxy
```
