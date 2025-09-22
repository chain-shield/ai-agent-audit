

#### SettingsValidators.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 MAXIMUM_PROOF_WINDOW = 86400
```
#### TransactionAttestation.verifyAddressValidity(IAddressValidity.Proof) [INTERNAL]
```slithir
 _settings = Globals.getSettings()
TMP_5269(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
_settings_1 (-> ['TMP_5269'])(AssetManagerSettings.Data) := TMP_5269(AssetManagerSettings.Data)
 fdcVerification = IFdcVerification(_settings.fdcVerification)
REF_3694(address) -> _settings_1 (-> ['TMP_5269']).fdcVerification
TMP_5270 = CONVERT REF_3694 to IFdcVerification
fdcVerification_1(IFdcVerification) := TMP_5270(IFdcVerification)
 require(bool,error)(_proof.data.sourceId == _settings.chainId,revert InvalidChain()())
REF_3695(IAddressValidity.Response) -> _proof_1.data
REF_3696(bytes32) -> REF_3695.sourceId
REF_3697(bytes32) -> _settings_1 (-> ['TMP_5269']).chainId
TMP_5271(bool) = REF_3696 == REF_3697
TMP_5272(None) = SOLIDITY_CALL revert InvalidChain()()
TMP_5273(None) = SOLIDITY_CALL require(bool,error)(TMP_5271,TMP_5272)
 require(bool,error)(fdcVerification.verifyAddressValidity(_proof),revert AddressValidityNotProven()())
TMP_5274(bool) = HIGH_LEVEL_CALL, dest:fdcVerification_1(IFdcVerification), function:verifyAddressValidity, arguments:['_proof_1']  
TMP_5275(None) = SOLIDITY_CALL revert AddressValidityNotProven()()
TMP_5276(None) = SOLIDITY_CALL require(bool,error)(TMP_5274,TMP_5275)
```
#### TransactionAttestation.verifyBalanceDecreasingTransaction(IBalanceDecreasingTransaction.Proof) [INTERNAL]
```slithir
 _settings = Globals.getSettings()
TMP_5245(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
_settings_1 (-> ['TMP_5245'])(AssetManagerSettings.Data) := TMP_5245(AssetManagerSettings.Data)
 fdcVerification = IFdcVerification(_settings.fdcVerification)
REF_3676(address) -> _settings_1 (-> ['TMP_5245']).fdcVerification
TMP_5246 = CONVERT REF_3676 to IFdcVerification
fdcVerification_1(IFdcVerification) := TMP_5246(IFdcVerification)
 require(bool,error)(_proof.data.sourceId == _settings.chainId,revert InvalidChain()())
REF_3677(IBalanceDecreasingTransaction.Response) -> _proof_1.data
REF_3678(bytes32) -> REF_3677.sourceId
REF_3679(bytes32) -> _settings_1 (-> ['TMP_5245']).chainId
TMP_5247(bool) = REF_3678 == REF_3679
TMP_5248(None) = SOLIDITY_CALL revert InvalidChain()()
TMP_5249(None) = SOLIDITY_CALL require(bool,error)(TMP_5247,TMP_5248)
 require(bool,error)(fdcVerification.verifyBalanceDecreasingTransaction(_proof),revert TransactionNotProven()())
TMP_5250(bool) = HIGH_LEVEL_CALL, dest:fdcVerification_1(IFdcVerification), function:verifyBalanceDecreasingTransaction, arguments:['_proof_1']  
TMP_5251(None) = SOLIDITY_CALL revert TransactionNotProven()()
TMP_5252(None) = SOLIDITY_CALL require(bool,error)(TMP_5250,TMP_5251)
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
#### TransactionAttestation.verifyPayment(IPayment.Proof) [INTERNAL]
```slithir
_proof_1(IPayment.Proof) := phi(['_proof_1'])
 _settings = Globals.getSettings()
TMP_5237(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
_settings_1 (-> ['TMP_5237'])(AssetManagerSettings.Data) := TMP_5237(AssetManagerSettings.Data)
 fdcVerification = IFdcVerification(_settings.fdcVerification)
REF_3670(address) -> _settings_1 (-> ['TMP_5237']).fdcVerification
TMP_5238 = CONVERT REF_3670 to IFdcVerification
fdcVerification_1(IFdcVerification) := TMP_5238(IFdcVerification)
 require(bool,error)(_proof.data.sourceId == _settings.chainId,revert InvalidChain()())
REF_3671(IPayment.Response) -> _proof_1.data
REF_3672(bytes32) -> REF_3671.sourceId
REF_3673(bytes32) -> _settings_1 (-> ['TMP_5237']).chainId
TMP_5239(bool) = REF_3672 == REF_3673
TMP_5240(None) = SOLIDITY_CALL revert InvalidChain()()
TMP_5241(None) = SOLIDITY_CALL require(bool,error)(TMP_5239,TMP_5240)
 require(bool,error)(fdcVerification.verifyPayment(_proof),revert LegalPaymentNotProven()())
TMP_5242(bool) = HIGH_LEVEL_CALL, dest:fdcVerification_1(IFdcVerification), function:verifyPayment, arguments:['_proof_1']  
TMP_5243(None) = SOLIDITY_CALL revert LegalPaymentNotProven()()
TMP_5244(None) = SOLIDITY_CALL require(bool,error)(TMP_5242,TMP_5243)
```
#### TransactionAttestation.verifyPaymentSuccess(IPayment.Proof) [INTERNAL]
```slithir
PAYMENT_SUCCESS_1(uint8) := phi(['PAYMENT_SUCCESS_0'])
 require(bool,error)(_proof.data.responseBody.status == PAYMENT_SUCCESS,revert PaymentFailed()())
REF_3666(IPayment.Response) -> _proof_1.data
REF_3667(IPayment.ResponseBody) -> REF_3666.responseBody
REF_3668(uint8) -> REF_3667.status
TMP_5233(bool) = REF_3668 == PAYMENT_SUCCESS_1
TMP_5234(None) = SOLIDITY_CALL revert PaymentFailed()()
TMP_5235(None) = SOLIDITY_CALL require(bool,error)(TMP_5233,TMP_5234)
 verifyPayment(_proof)
INTERNAL_CALL, TransactionAttestation.verifyPayment(IPayment.Proof)(_proof_1)
```
#### TransactionAttestation.verifyReferencedPaymentNonexistence(IReferencedPaymentNonexistence.Proof) [INTERNAL]
```slithir
 _settings = Globals.getSettings()
TMP_5261(AssetManagerSettings.Data) = LIBRARY_CALL, dest:Globals, function:Globals.getSettings(), arguments:[] 
_settings_1 (-> ['TMP_5261'])(AssetManagerSettings.Data) := TMP_5261(AssetManagerSettings.Data)
 fdcVerification = IFdcVerification(_settings.fdcVerification)
REF_3688(address) -> _settings_1 (-> ['TMP_5261']).fdcVerification
TMP_5262 = CONVERT REF_3688 to IFdcVerification
fdcVerification_1(IFdcVerification) := TMP_5262(IFdcVerification)
 require(bool,error)(_proof.data.sourceId == _settings.chainId,revert InvalidChain()())
REF_3689(IReferencedPaymentNonexistence.Response) -> _proof_1.data
REF_3690(bytes32) -> REF_3689.sourceId
REF_3691(bytes32) -> _settings_1 (-> ['TMP_5261']).chainId
TMP_5263(bool) = REF_3690 == REF_3691
TMP_5264(None) = SOLIDITY_CALL revert InvalidChain()()
TMP_5265(None) = SOLIDITY_CALL require(bool,error)(TMP_5263,TMP_5264)
 require(bool,error)(fdcVerification.verifyReferencedPaymentNonexistence(_proof),revert NonPaymentNotProven()())
TMP_5266(bool) = HIGH_LEVEL_CALL, dest:fdcVerification_1(IFdcVerification), function:verifyReferencedPaymentNonexistence, arguments:['_proof_1']  
TMP_5267(None) = SOLIDITY_CALL revert NonPaymentNotProven()()
TMP_5268(None) = SOLIDITY_CALL require(bool,error)(TMP_5266,TMP_5267)
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
