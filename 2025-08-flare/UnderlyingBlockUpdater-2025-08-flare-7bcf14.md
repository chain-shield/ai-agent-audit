




#### UnderlyingBlockUpdater.updateCurrentBlock(IConfirmedBlockHeightExists.Proof) [INTERNAL]
```slithir
 TransactionAttestation.verifyConfirmedBlockHeightExists(_proof)
LIBRARY_CALL, dest:TransactionAttestation, function:TransactionAttestation.verifyConfirmedBlockHeightExists(IConfirmedBlockHeightExists.Proof), arguments:['_proof_1'] 
 updateCurrentBlock(_proof.data.requestBody.blockNumber,_proof.data.responseBody.blockTimestamp,_proof.data.responseBody.numberOfConfirmations)
REF_3720(IConfirmedBlockHeightExists.Response) -> _proof_1.data
REF_3721(IConfirmedBlockHeightExists.RequestBody) -> REF_3720.requestBody
REF_3722(uint64) -> REF_3721.blockNumber
REF_3723(IConfirmedBlockHeightExists.Response) -> _proof_1.data
REF_3724(IConfirmedBlockHeightExists.ResponseBody) -> REF_3723.responseBody
REF_3725(uint64) -> REF_3724.blockTimestamp
REF_3726(IConfirmedBlockHeightExists.Response) -> _proof_1.data
REF_3727(IConfirmedBlockHeightExists.ResponseBody) -> REF_3726.responseBody
REF_3728(uint64) -> REF_3727.numberOfConfirmations
INTERNAL_CALL, UnderlyingBlockUpdater.updateCurrentBlock(uint64,uint64,uint64)(REF_3722,REF_3725,REF_3728)
```
#### UnderlyingBlockUpdater.updateCurrentBlockForVerifiedPayment(IPayment.Proof) [INTERNAL]
```slithir
 updateCurrentBlock(_proof.data.responseBody.blockNumber,_proof.data.responseBody.blockTimestamp,1)
REF_3729(IPayment.Response) -> _proof_1.data
REF_3730(IPayment.ResponseBody) -> REF_3729.responseBody
REF_3731(uint64) -> REF_3730.blockNumber
REF_3732(IPayment.Response) -> _proof_1.data
REF_3733(IPayment.ResponseBody) -> REF_3732.responseBody
REF_3734(uint64) -> REF_3733.blockTimestamp
INTERNAL_CALL, UnderlyingBlockUpdater.updateCurrentBlock(uint64,uint64,uint64)(REF_3731,REF_3734,1)
```
#### Globals.getSettings() [INTERNAL]
```slithir
ASSET_MANAGER_SETTINGS_POSITION_1(bytes32) := phi(['ASSET_MANAGER_SETTINGS_POSITION_0'])
 position = ASSET_MANAGER_SETTINGS_POSITION
position_1(bytes32) := ASSET_MANAGER_SETTINGS_POSITION_1(bytes32)
 _settings = position
_settings_1 (-> ['position'])(AssetManagerSettings.Data) := position_1(bytes32)
 _settings
RETURN _settings_1 (-> ['position'])
```
#### TransactionAttestation.verifyConfirmedBlockHeightExists(IConfirmedBlockHeightExists.Proof) [INTERNAL]
```slithir
 _settings = Globals.getSettings()
TMP_5253(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
_settings_1 (-> ['TMP_5253'])(AssetManagerSettings.Data) := TMP_5253(AssetManagerSettings.Data)
 fdcVerification = IFdcVerification(_settings.fdcVerification)
REF_3682(address) -> _settings_1 (-> ['TMP_5253']).fdcVerification
TMP_5254 = CONVERT REF_3682 to IFdcVerification
fdcVerification_1(IFdcVerification) := TMP_5254(IFdcVerification)
 require(bool,error)(_proof.data.sourceId == _settings.chainId,revert InvalidChain()())
REF_3683(IConfirmedBlockHeightExists.Response) -> _proof_1.data
REF_3684(bytes32) -> REF_3683.sourceId
REF_3685(bytes32) -> _settings_1 (-> ['TMP_5253']).chainId
TMP_5255(bool) = REF_3684 == REF_3685
TMP_5256(None) = SOLIDITY_CALL revert InvalidChain()()
TMP_5257(None) = SOLIDITY_CALL require(bool,error)(TMP_5255,TMP_5256)
 require(bool,error)(fdcVerification.verifyConfirmedBlockHeightExists(_proof),revert BlockHeightNotProven()())
TMP_5258(bool) = HIGH_LEVEL_CALL, dest:fdcVerification_1(IFdcVerification), function:verifyConfirmedBlockHeightExists, arguments:['_proof_1']  
TMP_5259(None) = SOLIDITY_CALL revert BlockHeightNotProven()()
TMP_5260(None) = SOLIDITY_CALL require(bool,error)(TMP_5258,TMP_5259)
```
#### AssetManagerState.get() [INTERNAL]
```slithir
STATE_POSITION_1(bytes32) := phi(['STATE_POSITION_0'])
 position = STATE_POSITION
position_1(bytes32) := STATE_POSITION_1(bytes32)
 _state = position
_state_1 (-> ['position'])(AssetManagerState.State) := position_1(bytes32)
 _state
RETURN _state_1 (-> ['position'])
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
