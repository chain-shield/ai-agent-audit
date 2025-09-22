### Storage layout (GovernanceSettingsMock) 

```text
governanceAddress address
timelock uint64
initialised bool
executors address[]
executorMap mapping(address => bool)

```


### Storage layout (CoreVaultManager) 

```text
assetManager address
chainId bytes32
custodianAddress string
coreVaultAddressHash bytes32
coreVaultAddress string
nextSequenceNumber uint256
fdcVerification IFdcVerification
confirmedPayments mapping(bytes32 => bool)
preimageHashes EnumerableSet.Bytes32Set
escrows ICoreVaultManager.Escrow[]
preimageHashToEscrowIndex mapping(bytes32 => uint256)
nextUnusedPreimageHashIndex uint256
nextUnprocessedEscrowIndex uint256
nextTransferRequestId uint256
cancelableTransferRequests uint256[]
nonCancelableTransferRequests uint256[]
transferRequestById mapping(uint256 => ICoreVaultManager.TransferRequest)
allowedDestinationAddresses string[]
allowedDestinationAddressIndex mapping(string => uint256)
triggeringAccounts EnumerableSet.AddressSet
emergencyPauseSenders EnumerableSet.AddressSet
escrowEndTimeSeconds uint128
escrowAmount uint128
minimalAmount uint128
fee uint128
availableFunds uint128
escrowedFunds uint128
cancelableTransferRequestsAmount uint128
nonCancelableTransferRequestsAmount uint128
paused bool

```


#### CoreVaultManager._checkNotPaused() [INTERNAL]
```slithir
paused_3(bool) := phi(['paused_0', 'paused_2', 'paused_1'])
 require(bool,error)(! paused,revert ContractPaused()())
TMP_7152 = UnaryType.BANG paused_3 
TMP_7153(None) = SOLIDITY_CALL revert ContractPaused()()
TMP_7154(None) = SOLIDITY_CALL require(bool,error)(TMP_7152,TMP_7153)
```
#### CoreVaultManager._checkOnlyAssetManager() [INTERNAL]
```slithir
assetManager_2(address) := phi(['assetManager_1', 'assetManager_0'])
 require(bool,error)(msg.sender == assetManager,revert OnlyAssetManager()())
TMP_7149(bool) = msg.sender == assetManager_2
TMP_7150(None) = SOLIDITY_CALL revert OnlyAssetManager()()
TMP_7151(None) = SOLIDITY_CALL require(bool,error)(TMP_7149,TMP_7150)
```
#### CoreVaultManager._getEscrow(uint256) [INTERNAL]
```slithir
_index_1(uint256) := phi(['escrowIndex_1', 'index_1'])
escrows_11(ICoreVaultManager.Escrow[]) := phi(['escrows_4', 'escrows_7', 'escrows_11', 'escrows_6', 'escrows_8', 'escrows_12', 'escrows_3', 'escrows_10', 'escrows_0', 'escrows_9'])
 require(bool,error)(_index != 0,revert NotFound()())
TMP_7133(bool) = _index_1 != 0
TMP_7134(None) = SOLIDITY_CALL revert NotFound()()
TMP_7135(None) = SOLIDITY_CALL require(bool,error)(TMP_7133,TMP_7134)
 escrows[_index - 1]
TMP_7136(uint256) = _index_1 (c)- 1
REF_4657(ICoreVaultManager.Escrow) -> escrows_11[TMP_7136]
RETURN REF_4657
```
#### CoreVaultManager._getNextEscrowEndTimestamp() [INTERNAL]
```slithir
escrows_12(ICoreVaultManager.Escrow[]) := phi(['escrows_4', 'escrows_7', 'escrows_11', 'escrows_6', 'escrows_8', 'escrows_12', 'escrows_3', 'escrows_10', 'escrows_0', 'escrows_9'])
nextUnprocessedEscrowIndex_7(uint256) := phi(['nextUnprocessedEscrowIndex_2', 'nextUnprocessedEscrowIndex_6', 'nextUnprocessedEscrowIndex_0'])
escrowEndTimeSeconds_3(uint128) := phi(['escrowEndTimeSeconds_0', 'escrowEndTimeSeconds_1'])
 escrowEndTimestamp = 0
escrowEndTimestamp_1(uint256) := 0(uint256)
escrowEndTimestamp_3(uint256) := phi(['escrowEndTimestamp_1', 'escrowEndTimestamp_2'])
 i = escrows.length
REF_4658 -> LENGTH escrows_12
i_1(uint256) := REF_4658(uint256)
 i > nextUnprocessedEscrowIndex
i_2(uint256) := phi(['i_3', 'i_1'])
TMP_7137(bool) = i_2 > nextUnprocessedEscrowIndex_7
CONDITION TMP_7137
 ! escrows[i - 1].finished
TMP_7138(uint256) = i_2 (c)- 1
REF_4659(ICoreVaultManager.Escrow) -> escrows_12[TMP_7138]
REF_4660(bool) -> REF_4659.finished
TMP_7139 = UnaryType.BANG REF_4660 
CONDITION TMP_7139
 escrowEndTimestamp = escrows[i - 1].expiryTs
TMP_7140(uint256) = i_2 (c)- 1
REF_4661(ICoreVaultManager.Escrow) -> escrows_12[TMP_7140]
REF_4662(uint64) -> REF_4661.expiryTs
escrowEndTimestamp_2(uint256) := REF_4662(uint64)
 i --
TMP_7141(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)- 1
 escrowEndTimestamp = Math.max(escrowEndTimestamp,block.timestamp)
TMP_7142(uint256) = LIBRARY_CALL, dest:Math, function:Math.max(uint256,uint256), arguments:['escrowEndTimestamp_3', 'block.timestamp'] 
escrowEndTimestamp_4(uint256) := TMP_7142(uint256)
 escrowEndTimestamp += 86400
escrowEndTimestamp_5(uint256) = escrowEndTimestamp_4 (c)+ 86400
 escrowEndTimestamp = escrowEndTimestamp - (escrowEndTimestamp % 86400) + escrowEndTimeSeconds
TMP_7143(uint256) = escrowEndTimestamp_5 % 86400
TMP_7144(uint256) = escrowEndTimestamp_5 (c)- TMP_7143
TMP_7145(uint256) = TMP_7144 (c)+ escrowEndTimeSeconds_3
escrowEndTimestamp_6(uint256) := TMP_7145(uint256)
 escrowEndTimestamp <= block.timestamp + 43200
TMP_7146(uint256) = block.timestamp (c)+ 43200
TMP_7147(bool) = escrowEndTimestamp_6 <= TMP_7146
CONDITION TMP_7147
 escrowEndTimestamp += 86400
escrowEndTimestamp_7(uint256) = escrowEndTimestamp_6 (c)+ 86400
escrowEndTimestamp_8(uint256) := phi(['escrowEndTimestamp_7', 'escrowEndTimestamp_6'])
 uint64(escrowEndTimestamp)
TMP_7148 = CONVERT escrowEndTimestamp_8 to uint64
RETURN TMP_7148
```
#### CoreVaultManager._processEscrows(uint256) [INTERNAL]
```slithir
_maxCount_1(uint256) := phi(['TMP_6915', '_maxCount_1'])
escrows_10(ICoreVaultManager.Escrow[]) := phi(['escrows_4', 'escrows_7', 'escrows_11', 'escrows_6', 'escrows_8', 'escrows_12', 'escrows_3', 'escrows_10', 'escrows_0', 'escrows_9'])
nextUnprocessedEscrowIndex_5(uint256) := phi(['nextUnprocessedEscrowIndex_2', 'nextUnprocessedEscrowIndex_6', 'nextUnprocessedEscrowIndex_0'])
availableFunds_16(uint128) := phi(['availableFunds_7', 'availableFunds_12', 'availableFunds_2', 'availableFunds_17', 'availableFunds_0', 'availableFunds_3', 'availableFunds_15', 'availableFunds_11'])
escrowedFunds_12(uint128) := phi(['escrowedFunds_4', 'escrowedFunds_0', 'escrowedFunds_7', 'escrowedFunds_13', 'escrowedFunds_8', 'escrowedFunds_11'])
 availableFundsTmp = availableFunds
availableFundsTmp_1(uint128) := availableFunds_16(uint128)
 escrowedFundsTmp = escrowedFunds
escrowedFundsTmp_1(uint128) := escrowedFunds_12(uint128)
 index = nextUnprocessedEscrowIndex
index_1(uint256) := nextUnprocessedEscrowIndex_5(uint256)
 _maxCount > 0 && index < escrows.length && (escrows[index].expiryTs <= block.timestamp || escrows[index].finished)
_maxCount_2(uint256) := phi(['_maxCount_3', '_maxCount_1'])
index_2(uint256) := phi(['index_1', 'index_3'])
TMP_7114(bool) = _maxCount_2 > 0
REF_4642 -> LENGTH escrows_10
TMP_7115(bool) = index_2 < REF_4642
TMP_7116(bool) = TMP_7114 && TMP_7115
REF_4643(ICoreVaultManager.Escrow) -> escrows_10[index_2]
REF_4644(uint64) -> REF_4643.expiryTs
TMP_7117(bool) = REF_4644 <= block.timestamp
REF_4645(ICoreVaultManager.Escrow) -> escrows_10[index_2]
REF_4646(bool) -> REF_4645.finished
TMP_7118(bool) = TMP_7117 || REF_4646
TMP_7119(bool) = TMP_7116 && TMP_7118
CONDITION TMP_7119
 ! escrows[index].finished
REF_4647(ICoreVaultManager.Escrow) -> escrows_10[index_2]
REF_4648(bool) -> REF_4647.finished
TMP_7120 = UnaryType.BANG REF_4648 
CONDITION TMP_7120
 escrow = escrows[index]
REF_4649(ICoreVaultManager.Escrow) -> escrows_10[index_2]
escrow_1 (-> ['escrows'])(ICoreVaultManager.Escrow) := REF_4649(ICoreVaultManager.Escrow)
 amount = escrow.amount
REF_4650(uint128) -> escrow_1 (-> ['escrows']).amount
amount_1(uint128) := REF_4650(uint128)
 availableFundsTmp += amount
availableFundsTmp_2(uint128) = availableFundsTmp_1 (c)+ amount_1
 escrowedFundsTmp -= amount
escrowedFundsTmp_2(uint128) = escrowedFundsTmp_1 (c)- amount_1
 EscrowExpired(escrow.preimageHash,amount)
REF_4651(bytes32) -> escrow_1 (-> ['escrows']).preimageHash
Emit EscrowExpired(REF_4651,amount_1)
availableFundsTmp_3(uint128) := phi(['availableFundsTmp_2', 'availableFundsTmp_1'])
escrowedFundsTmp_3(uint128) := phi(['escrowedFundsTmp_1', 'escrowedFundsTmp_2'])
 index ++
TMP_7122(uint256) := index_2(uint256)
index_3(uint256) = index_2 (c)+ 1
 _maxCount --
TMP_7123(uint256) := _maxCount_2(uint256)
_maxCount_3(uint256) = _maxCount_2 (c)- 1
 nextUnprocessedEscrowIndex = index
nextUnprocessedEscrowIndex_6(uint256) := index_2(uint256)
 availableFunds = availableFundsTmp
availableFunds_17(uint128) := availableFundsTmp_1(uint128)
 escrowedFunds = escrowedFundsTmp
escrowedFunds_13(uint128) := escrowedFundsTmp_1(uint128)
 _allProcessed = _maxCount > 0 || index == escrows.length || (escrows[index].expiryTs > block.timestamp && ! escrows[index].finished)
TMP_7124(bool) = _maxCount_2 > 0
REF_4652 -> LENGTH escrows_10
TMP_7125(bool) = index_2 == REF_4652
TMP_7126(bool) = TMP_7124 || TMP_7125
REF_4653(ICoreVaultManager.Escrow) -> escrows_10[index_2]
REF_4654(uint64) -> REF_4653.expiryTs
TMP_7127(bool) = REF_4654 > block.timestamp
REF_4655(ICoreVaultManager.Escrow) -> escrows_10[index_2]
REF_4656(bool) -> REF_4655.finished
TMP_7128 = UnaryType.BANG REF_4656 
TMP_7129(bool) = TMP_7127 && TMP_7128
TMP_7130(bool) = TMP_7126 || TMP_7129
_allProcessed_1(bool) := TMP_7130(bool)
 ! _allProcessed
TMP_7131 = UnaryType.BANG _allProcessed_1 
CONDITION TMP_7131
 NotAllEscrowsProcessed()
Emit NotAllEscrowsProcessed()
 _allProcessed
RETURN _allProcessed_1
```
#### CoreVaultManager._updateContractAddresses(bytes32[],address[]) [INTERNAL]
```slithir
_contractNameHashes_1(bytes32[]) := phi(['_contractNameHashes_1'])
_contractAddresses_1(address[]) := phi(['_contractAddresses_1'])
 fdcVerification = IFdcVerification(_getContractAddress(_contractNameHashes,_contractAddresses,FdcVerification))
TMP_7112(address) = INTERNAL_CALL, AddressUpdatable._getContractAddress(bytes32[],address[],string)(_contractNameHashes_1,_contractAddresses_1,FdcVerification)
TMP_7113 = CONVERT TMP_7112 to IFdcVerification
fdcVerification_3(IFdcVerification) := TMP_7113(IFdcVerification)
```
#### CoreVaultManager.addAllowedDestinationAddresses(string[]) [EXTERNAL]
```slithir
allowedDestinationAddresses_1(string[]) := phi(['allowedDestinationAddresses_2', 'allowedDestinationAddresses_0', 'allowedDestinationAddresses_6'])
allowedDestinationAddressIndex_4(mapping(string => uint256)) := phi(['allowedDestinationAddressIndex_11', 'allowedDestinationAddressIndex_3', 'allowedDestinationAddressIndex_5', 'allowedDestinationAddressIndex_0', 'allowedDestinationAddressIndex_8'])
 i = 0
i_1(uint256) := 0(uint256)
 i < _allowedDestinationAddresses.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_4551 -> LENGTH _allowedDestinationAddresses_1
TMP_6976(bool) = i_2 < REF_4551
CONDITION TMP_6976
 require(bool,error)(bytes(_allowedDestinationAddresses[i]).length > 0,revert InvalidAddress()())
REF_4552(string) -> _allowedDestinationAddresses_1[i_2]
TMP_6977 = CONVERT REF_4552 to bytes
REF_4553 -> LENGTH TMP_6977
TMP_6978(bool) = REF_4553 > 0
TMP_6979(None) = SOLIDITY_CALL revert InvalidAddress()()
TMP_6980(None) = SOLIDITY_CALL require(bool,error)(TMP_6978,TMP_6979)
 allowedDestinationAddressIndex[_allowedDestinationAddresses[i]] != 0
REF_4554(string) -> _allowedDestinationAddresses_1[i_2]
REF_4555(uint256) -> allowedDestinationAddressIndex_5[REF_4554]
TMP_6981(bool) = REF_4555 != 0
CONDITION TMP_6981
 allowedDestinationAddresses.push(_allowedDestinationAddresses[i])
REF_4557(string) -> _allowedDestinationAddresses_1[i_2]
REF_4558 -> LENGTH allowedDestinationAddresses_2
TMP_6983(uint256) := REF_4558(uint256)
TMP_6984(uint256) = TMP_6983 (c)+ 1
allowedDestinationAddresses_3(string[]) := phi(['allowedDestinationAddresses_2'])
REF_4558(uint256) (->allowedDestinationAddresses_3) := TMP_6984(uint256)
REF_4559(string) -> allowedDestinationAddresses_3[TMP_6983]
allowedDestinationAddresses_4(string[]) := phi(['allowedDestinationAddresses_3'])
REF_4559(string) (->allowedDestinationAddresses_4) := REF_4557(string)
 allowedDestinationAddressIndex[_allowedDestinationAddresses[i]] = allowedDestinationAddresses.length
REF_4560(string) -> _allowedDestinationAddresses_1[i_2]
REF_4561(uint256) -> allowedDestinationAddressIndex_5[REF_4560]
REF_4562 -> LENGTH allowedDestinationAddresses_4
allowedDestinationAddressIndex_6(mapping(string => uint256)) := phi(['allowedDestinationAddressIndex_5'])
REF_4561(uint256) (->allowedDestinationAddressIndex_6) := REF_4562(uint256)
 AllowedDestinationAddressAdded(_allowedDestinationAddresses[i])
REF_4563(string) -> _allowedDestinationAddresses_1[i_2]
Emit AllowedDestinationAddressAdded(REF_4563)
 i ++
TMP_6986(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 onlyGovernance()
MODIFIER_CALL, GovernedBase.onlyGovernance()()
```
#### CoreVaultManager.addEmergencyPauseSenders(address[]) [EXTERNAL]
```slithir
emergencyPauseSenders_1(EnumerableSet.AddressSet) := phi(['emergencyPauseSenders_2', 'emergencyPauseSenders_6', 'emergencyPauseSenders_4', 'emergencyPauseSenders_0'])
 i = 0
i_1(uint256) := 0(uint256)
 i < _addresses.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_4604 -> LENGTH _addresses_1
TMP_7050(bool) = i_2 < REF_4604
CONDITION TMP_7050
 emergencyPauseSenders.add(_addresses[i])
REF_4606(address) -> _addresses_1[i_2]
TMP_7051(bool) = LIBRARY_CALL, dest:EnumerableSet, function:EnumerableSet.add(EnumerableSet.AddressSet,address), arguments:['emergencyPauseSenders_2', 'REF_4606'] 
CONDITION TMP_7051
 EmergencyPauseSenderAdded(_addresses[i])
REF_4607(address) -> _addresses_1[i_2]
Emit EmergencyPauseSenderAdded(REF_4607)
 i ++
TMP_7053(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### CoreVaultManager.addPreimageHashes(bytes32[]) [EXTERNAL]
```slithir
preimageHashes_5(EnumerableSet.Bytes32Set) := phi(['preimageHashes_6', 'preimageHashes_3', 'preimageHashes_4', 'preimageHashes_0', 'preimageHashes_8'])
 i = 0
i_1(uint256) := 0(uint256)
 i < _preimageHashes.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_4587 -> LENGTH _preimageHashes_1
TMP_7022(bool) = i_2 < REF_4587
CONDITION TMP_7022
 require(bool,error)(_preimageHashes[i] != bytes32(0) && preimageHashes.add(_preimageHashes[i]),revert InvalidPreimageHash()())
REF_4588(bytes32) -> _preimageHashes_1[i_2]
TMP_7023 = CONVERT 0 to bytes32
TMP_7024(bool) = REF_4588 != TMP_7023
REF_4590(bytes32) -> _preimageHashes_1[i_2]
TMP_7025(bool) = LIBRARY_CALL, dest:EnumerableSet, function:EnumerableSet.add(EnumerableSet.Bytes32Set,bytes32), arguments:['preimageHashes_6', 'REF_4590'] 
TMP_7026(bool) = TMP_7024 && TMP_7025
TMP_7027(None) = SOLIDITY_CALL revert InvalidPreimageHash()()
TMP_7028(None) = SOLIDITY_CALL require(bool,error)(TMP_7026,TMP_7027)
 PreimageHashAdded(_preimageHashes[i])
REF_4591(bytes32) -> _preimageHashes_1[i_2]
Emit PreimageHashAdded(REF_4591)
 i ++
TMP_7030(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### CoreVaultManager.addTriggeringAccounts(address[]) [EXTERNAL]
```slithir
triggeringAccounts_3(EnumerableSet.AddressSet) := phi(['triggeringAccounts_2', 'triggeringAccounts_4', 'triggeringAccounts_0', 'triggeringAccounts_6'])
 i = 0
i_1(uint256) := 0(uint256)
 i < _triggeringAccounts.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_4578 -> LENGTH _triggeringAccounts_1
TMP_6998(bool) = i_2 < REF_4578
CONDITION TMP_6998
 triggeringAccounts.add(_triggeringAccounts[i])
REF_4580(address) -> _triggeringAccounts_1[i_2]
TMP_6999(bool) = LIBRARY_CALL, dest:EnumerableSet, function:EnumerableSet.add(EnumerableSet.AddressSet,address), arguments:['triggeringAccounts_4', 'REF_4580'] 
CONDITION TMP_6999
 TriggeringAccountAdded(_triggeringAccounts[i])
REF_4581(address) -> _triggeringAccounts_1[i_2]
Emit TriggeringAccountAdded(REF_4581)
 i ++
TMP_7001(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 onlyGovernance()
MODIFIER_CALL, GovernedBase.onlyGovernance()()
```
#### CoreVaultManager.cancelTransferRequestFromCoreVault(string) [EXTERNAL]
```slithir
cancelableTransferRequests_6(uint256[]) := phi(['cancelableTransferRequests_17', 'cancelableTransferRequests_13', 'cancelableTransferRequests_18', 'cancelableTransferRequests_0', 'cancelableTransferRequests_9', 'cancelableTransferRequests_3', 'cancelableTransferRequests_5'])
transferRequestById_6(mapping(uint256 => ICoreVaultManager.TransferRequest)) := phi(['transferRequestById_3', 'transferRequestById_8', 'transferRequestById_15', 'transferRequestById_12', 'transferRequestById_0', 'transferRequestById_11', 'transferRequestById_13', 'transferRequestById_14', 'transferRequestById_4', 'transferRequestById_5'])
cancelableTransferRequestsAmount_5(uint128) := phi(['cancelableTransferRequestsAmount_11', 'cancelableTransferRequestsAmount_4', 'cancelableTransferRequestsAmount_0', 'cancelableTransferRequestsAmount_7', 'cancelableTransferRequestsAmount_3'])
 destinationAddressHash = keccak256(bytes)(bytes(_destinationAddress))
TMP_6892 = CONVERT _destinationAddress_1 to bytes
TMP_6893(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_6892)
destinationAddressHash_1(bytes32) := TMP_6893(bytes32)
 index = 0
index_1(uint256) := 0(uint256)
 index < cancelableTransferRequests.length
index_2(uint256) := phi(['index_5', 'index_1'])
REF_4491 -> LENGTH cancelableTransferRequests_7
TMP_6894(bool) = index_2 < REF_4491
CONDITION TMP_6894
 destAddress = transferRequestById[cancelableTransferRequests[index]].destinationAddress
REF_4492(uint256) -> cancelableTransferRequests_7[index_2]
REF_4493(ICoreVaultManager.TransferRequest) -> transferRequestById_7[REF_4492]
REF_4494(string) -> REF_4493.destinationAddress
destAddress_1(string) := REF_4494(string)
 keccak256(bytes)(bytes(destAddress)) == destinationAddressHash
TMP_6895 = CONVERT destAddress_1 to bytes
TMP_6896(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_6895)
TMP_6897(bool) = TMP_6896 == destinationAddressHash_1
CONDITION TMP_6897
 index ++
TMP_6898(uint256) := index_2(uint256)
index_5(uint256) = index_2 (c)+ 1
 require(bool,error)(index < cancelableTransferRequests.length,revert NotFound()())
REF_4495 -> LENGTH cancelableTransferRequests_7
TMP_6899(bool) = index_2 < REF_4495
TMP_6900(None) = SOLIDITY_CALL revert NotFound()()
TMP_6901(None) = SOLIDITY_CALL require(bool,error)(TMP_6899,TMP_6900)
 transferRequestId = cancelableTransferRequests[index]
REF_4496(uint256) -> cancelableTransferRequests_7[index_2]
transferRequestId_1(uint256) := REF_4496(uint256)
 req = transferRequestById[transferRequestId]
REF_4497(ICoreVaultManager.TransferRequest) -> transferRequestById_7[transferRequestId_1]
req_1 (-> ['transferRequestById'])(ICoreVaultManager.TransferRequest) := REF_4497(ICoreVaultManager.TransferRequest)
 amount = req.amount
REF_4498(uint128) -> req_1 (-> ['transferRequestById']).amount
amount_1(uint128) := REF_4498(uint128)
 cancelableTransferRequestsAmount -= amount
cancelableTransferRequestsAmount_7(uint128) = cancelableTransferRequestsAmount_6 (c)- amount_1
 TransferRequestCanceled(_destinationAddress,req.paymentReference,amount)
REF_4499(bytes32) -> req_1 (-> ['transferRequestById']).paymentReference
Emit TransferRequestCanceled(_destinationAddress_1,REF_4499,amount_1)
 index < cancelableTransferRequests.length - 1
index_3(uint256) := phi(['index_4', 'index_1'])
REF_4500 -> LENGTH cancelableTransferRequests_7
TMP_6903(uint256) = REF_4500 (c)- 1
TMP_6904(bool) = index_3 < TMP_6903
CONDITION TMP_6904
 cancelableTransferRequests[index] = cancelableTransferRequests[index + 1]
REF_4501(uint256) -> cancelableTransferRequests_7[index_3]
TMP_6905(uint256) = index_3 (c)+ 1
REF_4502(uint256) -> cancelableTransferRequests_7[TMP_6905]
cancelableTransferRequests_10(uint256[]) := phi(['cancelableTransferRequests_7'])
REF_4501(uint256) (->cancelableTransferRequests_10) := REF_4502(uint256)
 index ++
TMP_6906(uint256) := index_3(uint256)
index_4(uint256) = index_3 (c)+ 1
 cancelableTransferRequests.pop()
REF_4504 -> LENGTH cancelableTransferRequests_7
TMP_6908(uint256) = REF_4504 (c)- 1
REF_4505(uint256) -> cancelableTransferRequests_7[TMP_6908]
cancelableTransferRequests_8 = delete REF_4505 
REF_4506 -> LENGTH cancelableTransferRequests_8
cancelableTransferRequests_9(uint256[]) := phi(['cancelableTransferRequests_8'])
REF_4506(uint256) (->cancelableTransferRequests_9) := TMP_6908(uint256)
 delete transferRequestById[transferRequestId]
REF_4507(ICoreVaultManager.TransferRequest) -> transferRequestById_7[transferRequestId_1]
transferRequestById_8 = delete REF_4507 
 onlyAssetManager()
MODIFIER_CALL, CoreVaultManager.onlyAssetManager()()
```
#### CoreVaultManager.confirmPayment(IPayment.Proof) [EXTERNAL]
```slithir
chainId_2(bytes32) := phi(['chainId_1', 'chainId_0'])
coreVaultAddressHash_2(bytes32) := phi(['coreVaultAddressHash_3', 'coreVaultAddressHash_0', 'coreVaultAddressHash_1'])
fdcVerification_1(IFdcVerification) := phi(['fdcVerification_2', 'fdcVerification_3', 'fdcVerification_0'])
confirmedPayments_1(mapping(bytes32 => bool)) := phi(['confirmedPayments_0', 'confirmedPayments_3', 'confirmedPayments_2'])
availableFunds_1(uint128) := phi(['availableFunds_7', 'availableFunds_12', 'availableFunds_2', 'availableFunds_17', 'availableFunds_0', 'availableFunds_3', 'availableFunds_15', 'availableFunds_11'])
 require(bool,error)(_proof.data.responseBody.status == 0,revert PaymentFailed()())
REF_4443(IPayment.Response) -> _proof_1.data
REF_4444(IPayment.ResponseBody) -> REF_4443.responseBody
REF_4445(uint8) -> REF_4444.status
TMP_6836(bool) = REF_4445 == 0
TMP_6837(None) = SOLIDITY_CALL revert PaymentFailed()()
TMP_6838(None) = SOLIDITY_CALL require(bool,error)(TMP_6836,TMP_6837)
 require(bool,error)(_proof.data.sourceId == chainId,revert InvalidChain()())
REF_4446(IPayment.Response) -> _proof_1.data
REF_4447(bytes32) -> REF_4446.sourceId
TMP_6839(bool) = REF_4447 == chainId_2
TMP_6840(None) = SOLIDITY_CALL revert InvalidChain()()
TMP_6841(None) = SOLIDITY_CALL require(bool,error)(TMP_6839,TMP_6840)
 require(bool,error)(fdcVerification.verifyPayment(_proof),revert PaymentNotProven()())
TMP_6842(bool) = HIGH_LEVEL_CALL, dest:fdcVerification_1(IFdcVerification), function:verifyPayment, arguments:['_proof_1']  
coreVaultAddressHash_3(bytes32) := phi(['coreVaultAddressHash_2', 'coreVaultAddressHash_3', 'coreVaultAddressHash_1'])
fdcVerification_2(IFdcVerification) := phi(['fdcVerification_2', 'fdcVerification_3', 'fdcVerification_1'])
confirmedPayments_2(mapping(bytes32 => bool)) := phi(['confirmedPayments_3', 'confirmedPayments_1', 'confirmedPayments_2'])
availableFunds_2(uint128) := phi(['availableFunds_7', 'availableFunds_12', 'availableFunds_2', 'availableFunds_17', 'availableFunds_3', 'availableFunds_15', 'availableFunds_11', 'availableFunds_1'])
TMP_6843(None) = SOLIDITY_CALL revert PaymentNotProven()()
TMP_6844(None) = SOLIDITY_CALL require(bool,error)(TMP_6842,TMP_6843)
 require(bool,error)(_proof.data.responseBody.receivingAddressHash == coreVaultAddressHash,revert NotCoreVault()())
REF_4449(IPayment.Response) -> _proof_1.data
REF_4450(IPayment.ResponseBody) -> REF_4449.responseBody
REF_4451(bytes32) -> REF_4450.receivingAddressHash
TMP_6845(bool) = REF_4451 == coreVaultAddressHash_3
TMP_6846(None) = SOLIDITY_CALL revert NotCoreVault()()
TMP_6847(None) = SOLIDITY_CALL require(bool,error)(TMP_6845,TMP_6846)
 require(bool,error)(_proof.data.responseBody.receivedAmount > 0,revert InvalidAmount()())
REF_4452(IPayment.Response) -> _proof_1.data
REF_4453(IPayment.ResponseBody) -> REF_4452.responseBody
REF_4454(int256) -> REF_4453.receivedAmount
TMP_6848(bool) = REF_4454 > 0
TMP_6849(None) = SOLIDITY_CALL revert InvalidAmount()()
TMP_6850(None) = SOLIDITY_CALL require(bool,error)(TMP_6848,TMP_6849)
 ! confirmedPayments[_proof.data.requestBody.transactionId]
REF_4455(IPayment.Response) -> _proof_1.data
REF_4456(IPayment.RequestBody) -> REF_4455.requestBody
REF_4457(bytes32) -> REF_4456.transactionId
REF_4458(bool) -> confirmedPayments_2[REF_4457]
TMP_6851 = UnaryType.BANG REF_4458 
CONDITION TMP_6851
 receivedAmount = uint128(uint256(_proof.data.responseBody.receivedAmount))
REF_4459(IPayment.Response) -> _proof_1.data
REF_4460(IPayment.ResponseBody) -> REF_4459.responseBody
REF_4461(int256) -> REF_4460.receivedAmount
TMP_6852 = CONVERT REF_4461 to uint256
TMP_6853 = CONVERT TMP_6852 to uint128
receivedAmount_1(uint128) := TMP_6853(uint128)
 confirmedPayments[_proof.data.requestBody.transactionId] = true
REF_4462(IPayment.Response) -> _proof_1.data
REF_4463(IPayment.RequestBody) -> REF_4462.requestBody
REF_4464(bytes32) -> REF_4463.transactionId
REF_4465(bool) -> confirmedPayments_2[REF_4464]
confirmedPayments_3(mapping(bytes32 => bool)) := phi(['confirmedPayments_2'])
REF_4465(bool) (->confirmedPayments_3) := True(bool)
 availableFunds += receivedAmount
availableFunds_3(uint128) = availableFunds_2 (c)+ receivedAmount_1
 PaymentConfirmed(_proof.data.requestBody.transactionId,_proof.data.responseBody.standardPaymentReference,receivedAmount)
REF_4466(IPayment.Response) -> _proof_1.data
REF_4467(IPayment.RequestBody) -> REF_4466.requestBody
REF_4468(bytes32) -> REF_4467.transactionId
REF_4469(IPayment.Response) -> _proof_1.data
REF_4470(IPayment.ResponseBody) -> REF_4469.responseBody
REF_4471(bytes32) -> REF_4470.standardPaymentReference
Emit PaymentConfirmed(REF_4468,REF_4471,receivedAmount_1)
```
#### CoreVaultManager.constructor() [PUBLIC]
```slithir
 GovernedUUPSProxyImplementation()
INTERNAL_CALL, GovernedUUPSProxyImplementation.constructor()()
 AddressUpdatable(address(0))
TMP_6813 = CONVERT 0 to address
INTERNAL_CALL, AddressUpdatable.constructor(address)(TMP_6813)
```
#### CoreVaultManager.getAllowedDestinationAddresses() [EXTERNAL]
```slithir
allowedDestinationAddresses_10(string[]) := phi(['allowedDestinationAddresses_2', 'allowedDestinationAddresses_0', 'allowedDestinationAddresses_6'])
 allowedDestinationAddresses
RETURN allowedDestinationAddresses_10
```
#### CoreVaultManager.getCancelableTransferRequests() [EXTERNAL]
```slithir
cancelableTransferRequests_17(uint256[]) := phi(['cancelableTransferRequests_17', 'cancelableTransferRequests_13', 'cancelableTransferRequests_18', 'cancelableTransferRequests_0', 'cancelableTransferRequests_9', 'cancelableTransferRequests_3', 'cancelableTransferRequests_5'])
transferRequestById_14(mapping(uint256 => ICoreVaultManager.TransferRequest)) := phi(['transferRequestById_3', 'transferRequestById_8', 'transferRequestById_15', 'transferRequestById_12', 'transferRequestById_0', 'transferRequestById_11', 'transferRequestById_13', 'transferRequestById_14', 'transferRequestById_4', 'transferRequestById_5'])
 _transferRequests = new ICoreVaultManager.TransferRequest[](cancelableTransferRequests.length)
REF_4626 -> LENGTH cancelableTransferRequests_17
TMP_7092(ICoreVaultManager.TransferRequest[])  = new ICoreVaultManager.TransferRequest[](REF_4626)
_transferRequests_1(ICoreVaultManager.TransferRequest[]) = ['TMP_7092(ICoreVaultManager.TransferRequest[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < cancelableTransferRequests.length
i_2(uint256) := phi(['i_3', 'i_1'])
REF_4627 -> LENGTH cancelableTransferRequests_17
TMP_7093(bool) = i_2 < REF_4627
CONDITION TMP_7093
 _transferRequests[i] = transferRequestById[cancelableTransferRequests[i]]
REF_4628(ICoreVaultManager.TransferRequest) -> _transferRequests_1[i_2]
REF_4629(uint256) -> cancelableTransferRequests_17[i_2]
REF_4630(ICoreVaultManager.TransferRequest) -> transferRequestById_14[REF_4629]
_transferRequests_2(ICoreVaultManager.TransferRequest[]) := phi(['_transferRequests_1'])
REF_4628(ICoreVaultManager.TransferRequest) (->_transferRequests_2) := REF_4630(ICoreVaultManager.TransferRequest)
 i ++
TMP_7094(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 _transferRequests
RETURN _transferRequests_1
```
#### CoreVaultManager.getEmergencyPauseSenders() [EXTERNAL]
```slithir
emergencyPauseSenders_7(EnumerableSet.AddressSet) := phi(['emergencyPauseSenders_2', 'emergencyPauseSenders_6', 'emergencyPauseSenders_4', 'emergencyPauseSenders_0'])
 emergencyPauseSenders.values()
TMP_7103(address[]) = LIBRARY_CALL, dest:EnumerableSet, function:EnumerableSet.values(EnumerableSet.AddressSet), arguments:['emergencyPauseSenders_7'] 
RETURN TMP_7103
```
#### CoreVaultManager.getEscrowByIndex(uint256) [EXTERNAL]
```slithir
escrows_9(ICoreVaultManager.Escrow[]) := phi(['escrows_4', 'escrows_7', 'escrows_11', 'escrows_6', 'escrows_8', 'escrows_12', 'escrows_3', 'escrows_10', 'escrows_0', 'escrows_9'])
 escrows[_index]
REF_4619(ICoreVaultManager.Escrow) -> escrows_9[_index_1]
RETURN REF_4619
```
#### CoreVaultManager.getEscrowByPreimageHash(bytes32) [EXTERNAL]
```slithir
preimageHashToEscrowIndex_5(mapping(bytes32 => uint256)) := phi(['preimageHashToEscrowIndex_1', 'preimageHashToEscrowIndex_0', 'preimageHashToEscrowIndex_3', 'preimageHashToEscrowIndex_5'])
 index = preimageHashToEscrowIndex[_preimageHash]
REF_4620(uint256) -> preimageHashToEscrowIndex_5[_preimageHash_1]
index_1(uint256) := REF_4620(uint256)
 _getEscrow(index)
TMP_7080(ICoreVaultManager.Escrow) = INTERNAL_CALL, CoreVaultManager._getEscrow(uint256)(index_1)
RETURN TMP_7080
```
#### CoreVaultManager.getEscrowsCount() [EXTERNAL]
```slithir
escrows_8(ICoreVaultManager.Escrow[]) := phi(['escrows_4', 'escrows_7', 'escrows_11', 'escrows_6', 'escrows_8', 'escrows_12', 'escrows_3', 'escrows_10', 'escrows_0', 'escrows_9'])
 escrows.length
REF_4618 -> LENGTH escrows_8
RETURN REF_4618
```
#### CoreVaultManager.getNonCancelableTransferRequests() [EXTERNAL]
```slithir
nonCancelableTransferRequests_12(uint256[]) := phi(['nonCancelableTransferRequests_5', 'nonCancelableTransferRequests_8', 'nonCancelableTransferRequests_13', 'nonCancelableTransferRequests_0', 'nonCancelableTransferRequests_12', 'nonCancelableTransferRequests_11', 'nonCancelableTransferRequests_3', 'nonCancelableTransferRequests_10'])
transferRequestById_15(mapping(uint256 => ICoreVaultManager.TransferRequest)) := phi(['transferRequestById_3', 'transferRequestById_8', 'transferRequestById_15', 'transferRequestById_12', 'transferRequestById_0', 'transferRequestById_11', 'transferRequestById_13', 'transferRequestById_14', 'transferRequestById_4', 'transferRequestById_5'])
 _transferRequests = new ICoreVaultManager.TransferRequest[](nonCancelableTransferRequests.length)
REF_4631 -> LENGTH nonCancelableTransferRequests_12
TMP_7096(ICoreVaultManager.TransferRequest[])  = new ICoreVaultManager.TransferRequest[](REF_4631)
_transferRequests_1(ICoreVaultManager.TransferRequest[]) = ['TMP_7096(ICoreVaultManager.TransferRequest[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < nonCancelableTransferRequests.length
i_2(uint256) := phi(['i_3', 'i_1'])
REF_4632 -> LENGTH nonCancelableTransferRequests_12
TMP_7097(bool) = i_2 < REF_4632
CONDITION TMP_7097
 _transferRequests[i] = transferRequestById[nonCancelableTransferRequests[i]]
REF_4633(ICoreVaultManager.TransferRequest) -> _transferRequests_1[i_2]
REF_4634(uint256) -> nonCancelableTransferRequests_12[i_2]
REF_4635(ICoreVaultManager.TransferRequest) -> transferRequestById_15[REF_4634]
_transferRequests_2(ICoreVaultManager.TransferRequest[]) := phi(['_transferRequests_1'])
REF_4633(ICoreVaultManager.TransferRequest) (->_transferRequests_2) := REF_4635(ICoreVaultManager.TransferRequest)
 i ++
TMP_7098(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 _transferRequests
RETURN _transferRequests_1
```
#### CoreVaultManager.getPreimageHash(uint256) [EXTERNAL]
```slithir
preimageHashes_11(EnumerableSet.Bytes32Set) := phi(['preimageHashes_6', 'preimageHashes_3', 'preimageHashes_4', 'preimageHashes_0', 'preimageHashes_8'])
 preimageHashes.at(_index)
TMP_7090(bytes32) = LIBRARY_CALL, dest:EnumerableSet, function:EnumerableSet.at(EnumerableSet.Bytes32Set,uint256), arguments:['preimageHashes_11', '_index_1'] 
RETURN TMP_7090
```
#### CoreVaultManager.getPreimageHashesCount() [EXTERNAL]
```slithir
preimageHashes_10(EnumerableSet.Bytes32Set) := phi(['preimageHashes_6', 'preimageHashes_3', 'preimageHashes_4', 'preimageHashes_0', 'preimageHashes_8'])
 preimageHashes.length()
TMP_7089(uint256) = LIBRARY_CALL, dest:EnumerableSet, function:EnumerableSet.length(EnumerableSet.Bytes32Set), arguments:['preimageHashes_10'] 
RETURN TMP_7089
```
#### CoreVaultManager.getSettings() [EXTERNAL]
```slithir
escrowEndTimeSeconds_2(uint128) := phi(['escrowEndTimeSeconds_0', 'escrowEndTimeSeconds_1'])
escrowAmount_5(uint128) := phi(['escrowAmount_3', 'escrowAmount_4', 'escrowAmount_0'])
minimalAmount_5(uint128) := phi(['minimalAmount_0', 'minimalAmount_3', 'minimalAmount_4'])
fee_5(uint128) := phi(['fee_0', 'fee_4', 'fee_3'])
 (escrowEndTimeSeconds,escrowAmount,minimalAmount,fee)
RETURN escrowEndTimeSeconds_2,escrowAmount_5,minimalAmount_5,fee_5
 (_escrowEndTimeSeconds,_escrowAmount,_minimalAmount,_fee)
```
#### CoreVaultManager.getTriggeringAccounts() [EXTERNAL]
```slithir
triggeringAccounts_7(EnumerableSet.AddressSet) := phi(['triggeringAccounts_2', 'triggeringAccounts_4', 'triggeringAccounts_0', 'triggeringAccounts_6'])
 triggeringAccounts.values()
TMP_7073(address[]) = LIBRARY_CALL, dest:EnumerableSet, function:EnumerableSet.values(EnumerableSet.AddressSet), arguments:['triggeringAccounts_7'] 
RETURN TMP_7073
```
#### CoreVaultManager.getUnprocessedEscrows() [EXTERNAL]
```slithir
escrows_7(ICoreVaultManager.Escrow[]) := phi(['escrows_4', 'escrows_7', 'escrows_11', 'escrows_6', 'escrows_8', 'escrows_12', 'escrows_3', 'escrows_10', 'escrows_0', 'escrows_9'])
nextUnprocessedEscrowIndex_4(uint256) := phi(['nextUnprocessedEscrowIndex_2', 'nextUnprocessedEscrowIndex_6', 'nextUnprocessedEscrowIndex_0'])
 length = escrows.length - nextUnprocessedEscrowIndex
REF_4615 -> LENGTH escrows_7
TMP_7074(uint256) = REF_4615 (c)- nextUnprocessedEscrowIndex_4
length_1(uint256) := TMP_7074(uint256)
 _unprocessedEscrows = new ICoreVaultManager.Escrow[](length)
TMP_7076(ICoreVaultManager.Escrow[])  = new ICoreVaultManager.Escrow[](length_1)
_unprocessedEscrows_1(ICoreVaultManager.Escrow[]) = ['TMP_7076(ICoreVaultManager.Escrow[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < length
i_2(uint256) := phi(['i_1', 'i_3'])
TMP_7077(bool) = i_2 < length_1
CONDITION TMP_7077
 _unprocessedEscrows[i] = escrows[nextUnprocessedEscrowIndex + i]
REF_4616(ICoreVaultManager.Escrow) -> _unprocessedEscrows_1[i_2]
TMP_7078(uint256) = nextUnprocessedEscrowIndex_4 (c)+ i_2
REF_4617(ICoreVaultManager.Escrow) -> escrows_7[TMP_7078]
_unprocessedEscrows_2(ICoreVaultManager.Escrow[]) := phi(['_unprocessedEscrows_1'])
REF_4616(ICoreVaultManager.Escrow) (->_unprocessedEscrows_2) := REF_4617(ICoreVaultManager.Escrow)
 i ++
TMP_7079(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 _unprocessedEscrows
RETURN _unprocessedEscrows_1
```
#### CoreVaultManager.getUnusedPreimageHashes() [EXTERNAL]
```slithir
preimageHashes_9(EnumerableSet.Bytes32Set) := phi(['preimageHashes_6', 'preimageHashes_3', 'preimageHashes_4', 'preimageHashes_0', 'preimageHashes_8'])
nextUnusedPreimageHashIndex_7(uint256) := phi(['nextUnusedPreimageHashIndex_3', 'nextUnusedPreimageHashIndex_4', 'nextUnusedPreimageHashIndex_0', 'nextUnusedPreimageHashIndex_6'])
 length = preimageHashes.length() - nextUnusedPreimageHashIndex
TMP_7081(uint256) = LIBRARY_CALL, dest:EnumerableSet, function:EnumerableSet.length(EnumerableSet.Bytes32Set), arguments:['preimageHashes_9'] 
TMP_7082(uint256) = TMP_7081 (c)- nextUnusedPreimageHashIndex_7
length_1(uint256) := TMP_7082(uint256)
 unusedPreimageHashes = new bytes32[](length)
TMP_7084(bytes32[])  = new bytes32[](length_1)
unusedPreimageHashes_1(bytes32[]) = ['TMP_7084(bytes32[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < length
i_2(uint256) := phi(['i_1', 'i_3'])
TMP_7085(bool) = i_2 < length_1
CONDITION TMP_7085
 unusedPreimageHashes[i] = preimageHashes.at(nextUnusedPreimageHashIndex + i)
REF_4622(bytes32) -> unusedPreimageHashes_1[i_2]
TMP_7086(uint256) = nextUnusedPreimageHashIndex_7 (c)+ i_2
TMP_7087(bytes32) = LIBRARY_CALL, dest:EnumerableSet, function:EnumerableSet.at(EnumerableSet.Bytes32Set,uint256), arguments:['preimageHashes_9', 'TMP_7086'] 
unusedPreimageHashes_2(bytes32[]) := phi(['unusedPreimageHashes_1'])
REF_4622(bytes32) (->unusedPreimageHashes_2) := TMP_7087(bytes32)
 i ++
TMP_7088(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 unusedPreimageHashes
RETURN unusedPreimageHashes_1
```
#### CoreVaultManager.initialize(IGovernanceSettings,address,address,address,bytes32,string,string,uint256) [EXTERNAL]
```slithir
 require(bool,error)(_assetManager != address(0),revert InvalidAddress()())
TMP_6815 = CONVERT 0 to address
TMP_6816(bool) = _assetManager_1 != TMP_6815
TMP_6817(None) = SOLIDITY_CALL revert InvalidAddress()()
TMP_6818(None) = SOLIDITY_CALL require(bool,error)(TMP_6816,TMP_6817)
 require(bool,error)(_chainId != bytes32(0),revert InvalidChain()())
TMP_6819 = CONVERT 0 to bytes32
TMP_6820(bool) = _chainId_1 != TMP_6819
TMP_6821(None) = SOLIDITY_CALL revert InvalidChain()()
TMP_6822(None) = SOLIDITY_CALL require(bool,error)(TMP_6820,TMP_6821)
 require(bool,error)(bytes(_custodianAddress).length > 0,revert InvalidAddress()())
TMP_6823 = CONVERT _custodianAddress_1 to bytes
REF_4439 -> LENGTH TMP_6823
TMP_6824(bool) = REF_4439 > 0
TMP_6825(None) = SOLIDITY_CALL revert InvalidAddress()()
TMP_6826(None) = SOLIDITY_CALL require(bool,error)(TMP_6824,TMP_6825)
 require(bool,error)(bytes(_coreVaultAddress).length > 0,revert InvalidAddress()())
TMP_6827 = CONVERT _coreVaultAddress_1 to bytes
REF_4440 -> LENGTH TMP_6827
TMP_6828(bool) = REF_4440 > 0
TMP_6829(None) = SOLIDITY_CALL revert InvalidAddress()()
TMP_6830(None) = SOLIDITY_CALL require(bool,error)(TMP_6828,TMP_6829)
 GovernedBase.initialise(_governanceSettings,_initialGovernance)
INTERNAL_CALL, GovernedBase.initialise(IGovernanceSettings,address)(_governanceSettings_1,_initialGovernance_1)
 AddressUpdatable.setAddressUpdaterValue(_addressUpdater)
INTERNAL_CALL, AddressUpdatable.setAddressUpdaterValue(address)(_addressUpdater_1)
 assetManager = _assetManager
assetManager_1(address) := _assetManager_1(address)
 chainId = _chainId
chainId_1(bytes32) := _chainId_1(bytes32)
 custodianAddress = _custodianAddress
custodianAddress_1(string) := _custodianAddress_1(string)
 coreVaultAddressHash = keccak256(bytes)(bytes(_coreVaultAddress))
TMP_6833 = CONVERT _coreVaultAddress_1 to bytes
TMP_6834(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_6833)
coreVaultAddressHash_1(bytes32) := TMP_6834(bytes32)
 coreVaultAddress = _coreVaultAddress
coreVaultAddress_1(string) := _coreVaultAddress_1(string)
 nextSequenceNumber = _nextSequenceNumber
nextSequenceNumber_1(uint256) := _nextSequenceNumber_1(uint256)
 CustodianAddressUpdated(_custodianAddress)
Emit CustodianAddressUpdated(_custodianAddress_1)
```
#### CoreVaultManager.isDestinationAddressAllowed(string) [EXTERNAL]
```slithir
allowedDestinationAddressIndex_11(mapping(string => uint256)) := phi(['allowedDestinationAddressIndex_11', 'allowedDestinationAddressIndex_3', 'allowedDestinationAddressIndex_5', 'allowedDestinationAddressIndex_0', 'allowedDestinationAddressIndex_8'])
 allowedDestinationAddressIndex[_address] > 0
REF_4613(uint256) -> allowedDestinationAddressIndex_11[_address_1]
TMP_7072(bool) = REF_4613 > 0
RETURN TMP_7072
```
#### CoreVaultManager.pause() [EXTERNAL]
```slithir
emergencyPauseSenders_5(EnumerableSet.AddressSet) := phi(['emergencyPauseSenders_2', 'emergencyPauseSenders_6', 'emergencyPauseSenders_4', 'emergencyPauseSenders_0'])
 require(bool,error)(msg.sender == governance() || emergencyPauseSenders.contains(msg.sender),revert NotAuthorized()())
TMP_7060(address) = INTERNAL_CALL, GovernedBase.governance()()
TMP_7061(bool) = msg.sender == TMP_7060
TMP_7062(bool) = LIBRARY_CALL, dest:EnumerableSet, function:EnumerableSet.contains(EnumerableSet.AddressSet,address), arguments:['emergencyPauseSenders_6', 'msg.sender'] 
TMP_7063(bool) = TMP_7061 || TMP_7062
TMP_7064(None) = SOLIDITY_CALL revert NotAuthorized()()
TMP_7065(None) = SOLIDITY_CALL require(bool,error)(TMP_7063,TMP_7064)
 paused = true
paused_1(bool) := True(bool)
 Paused()
Emit Paused()
```
#### CoreVaultManager.processEscrows(uint256) [EXTERNAL]
```slithir
 _processEscrows(_maxCount)
TMP_6910(bool) = INTERNAL_CALL, CoreVaultManager._processEscrows(uint256)(_maxCount_1)
RETURN TMP_6910
```
#### CoreVaultManager.removeAllowedDestinationAddresses(string[]) [EXTERNAL]
```slithir
allowedDestinationAddresses_5(string[]) := phi(['allowedDestinationAddresses_2', 'allowedDestinationAddresses_0', 'allowedDestinationAddresses_6'])
allowedDestinationAddressIndex_7(mapping(string => uint256)) := phi(['allowedDestinationAddressIndex_11', 'allowedDestinationAddressIndex_3', 'allowedDestinationAddressIndex_5', 'allowedDestinationAddressIndex_0', 'allowedDestinationAddressIndex_8'])
 i = 0
i_1(uint256) := 0(uint256)
 i < _allowedDestinationAddresses.length
i_2(uint256) := phi(['i_3', 'i_1'])
REF_4564 -> LENGTH _allowedDestinationAddresses_1
TMP_6988(bool) = i_2 < REF_4564
CONDITION TMP_6988
 index = allowedDestinationAddressIndex[_allowedDestinationAddresses[i]]
REF_4565(string) -> _allowedDestinationAddresses_1[i_2]
REF_4566(uint256) -> allowedDestinationAddressIndex_8[REF_4565]
index_1(uint256) := REF_4566(uint256)
 index == 0
TMP_6989(bool) = index_1 == 0
CONDITION TMP_6989
 length = allowedDestinationAddresses.length
REF_4567 -> LENGTH allowedDestinationAddresses_6
length_1(uint256) := REF_4567(uint256)
 index < length
TMP_6990(bool) = index_1 < length_1
CONDITION TMP_6990
 addressToMove = allowedDestinationAddresses[length - 1]
TMP_6991(uint256) = length_1 (c)- 1
REF_4568(string) -> allowedDestinationAddresses_6[TMP_6991]
addressToMove_1(string) := REF_4568(string)
 allowedDestinationAddresses[index - 1] = addressToMove
TMP_6992(uint256) = index_1 (c)- 1
REF_4569(string) -> allowedDestinationAddresses_6[TMP_6992]
allowedDestinationAddresses_7(string[]) := phi(['allowedDestinationAddresses_6'])
REF_4569(string) (->allowedDestinationAddresses_7) := addressToMove_1(string)
 allowedDestinationAddressIndex[addressToMove] = index
REF_4570(uint256) -> allowedDestinationAddressIndex_8[addressToMove_1]
allowedDestinationAddressIndex_9(mapping(string => uint256)) := phi(['allowedDestinationAddressIndex_8'])
REF_4570(uint256) (->allowedDestinationAddressIndex_9) := index_1(uint256)
 allowedDestinationAddresses.pop()
REF_4572 -> LENGTH allowedDestinationAddresses_7
TMP_6994(uint256) = REF_4572 (c)- 1
REF_4573(string) -> allowedDestinationAddresses_7[TMP_6994]
allowedDestinationAddresses_8 = delete REF_4573 
REF_4574 -> LENGTH allowedDestinationAddresses_8
allowedDestinationAddresses_9(string[]) := phi(['allowedDestinationAddresses_8'])
REF_4574(uint256) (->allowedDestinationAddresses_9) := TMP_6994(uint256)
 delete allowedDestinationAddressIndex[_allowedDestinationAddresses[i]]
REF_4575(string) -> _allowedDestinationAddresses_1[i_2]
REF_4576(uint256) -> allowedDestinationAddressIndex_9[REF_4575]
allowedDestinationAddressIndex_10 = delete REF_4576 
 AllowedDestinationAddressRemoved(_allowedDestinationAddresses[i])
REF_4577(string) -> _allowedDestinationAddresses_1[i_2]
Emit AllowedDestinationAddressRemoved(REF_4577)
 i ++
TMP_6996(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 onlyGovernance()
MODIFIER_CALL, GovernedBase.onlyGovernance()()
```
#### CoreVaultManager.removeEmergencyPauseSenders(address[]) [EXTERNAL]
```slithir
emergencyPauseSenders_3(EnumerableSet.AddressSet) := phi(['emergencyPauseSenders_2', 'emergencyPauseSenders_6', 'emergencyPauseSenders_4', 'emergencyPauseSenders_0'])
 i = 0
i_1(uint256) := 0(uint256)
 i < _addresses.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_4608 -> LENGTH _addresses_1
TMP_7055(bool) = i_2 < REF_4608
CONDITION TMP_7055
 emergencyPauseSenders.remove(_addresses[i])
REF_4610(address) -> _addresses_1[i_2]
TMP_7056(bool) = LIBRARY_CALL, dest:EnumerableSet, function:EnumerableSet.remove(EnumerableSet.AddressSet,address), arguments:['emergencyPauseSenders_4', 'REF_4610'] 
CONDITION TMP_7056
 EmergencyPauseSenderRemoved(_addresses[i])
REF_4611(address) -> _addresses_1[i_2]
Emit EmergencyPauseSenderRemoved(REF_4611)
 i ++
TMP_7058(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### CoreVaultManager.removeTriggeringAccounts(address[]) [EXTERNAL]
```slithir
triggeringAccounts_5(EnumerableSet.AddressSet) := phi(['triggeringAccounts_2', 'triggeringAccounts_4', 'triggeringAccounts_0', 'triggeringAccounts_6'])
 i = 0
i_1(uint256) := 0(uint256)
 i < _triggeringAccounts.length
i_2(uint256) := phi(['i_3', 'i_1'])
REF_4582 -> LENGTH _triggeringAccounts_1
TMP_7003(bool) = i_2 < REF_4582
CONDITION TMP_7003
 triggeringAccounts.remove(_triggeringAccounts[i])
REF_4584(address) -> _triggeringAccounts_1[i_2]
TMP_7004(bool) = LIBRARY_CALL, dest:EnumerableSet, function:EnumerableSet.remove(EnumerableSet.AddressSet,address), arguments:['triggeringAccounts_6', 'REF_4584'] 
CONDITION TMP_7004
 TriggeringAccountRemoved(_triggeringAccounts[i])
REF_4585(address) -> _triggeringAccounts_1[i_2]
Emit TriggeringAccountRemoved(REF_4585)
 i ++
TMP_7006(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 onlyGovernance()
MODIFIER_CALL, GovernedBase.onlyGovernance()()
```
#### CoreVaultManager.removeUnusedPreimageHashes(uint256) [EXTERNAL]
```slithir
preimageHashes_7(EnumerableSet.Bytes32Set) := phi(['preimageHashes_6', 'preimageHashes_3', 'preimageHashes_4', 'preimageHashes_0', 'preimageHashes_8'])
nextUnusedPreimageHashIndex_5(uint256) := phi(['nextUnusedPreimageHashIndex_3', 'nextUnusedPreimageHashIndex_4', 'nextUnusedPreimageHashIndex_0', 'nextUnusedPreimageHashIndex_6'])
 index = preimageHashes.length()
TMP_7032(uint256) = LIBRARY_CALL, dest:EnumerableSet, function:EnumerableSet.length(EnumerableSet.Bytes32Set), arguments:['preimageHashes_8'] 
index_1(uint256) := TMP_7032(uint256)
 _maxCount > 0 && index > nextUnusedPreimageHashIndex
_maxCount_2(uint256) := phi(['_maxCount_1', '_maxCount_3'])
index_2(uint256) := phi(['index_3', 'index_1'])
TMP_7033(bool) = _maxCount_2 > 0
TMP_7034(bool) = index_2 > nextUnusedPreimageHashIndex_6
TMP_7035(bool) = TMP_7033 && TMP_7034
CONDITION TMP_7035
 preimageHash = preimageHashes.at(-- index)
index_3(uint256) = index_2 (c)- 1
TMP_7036(bytes32) = LIBRARY_CALL, dest:EnumerableSet, function:EnumerableSet.at(EnumerableSet.Bytes32Set,uint256), arguments:['preimageHashes_8', 'index_3'] 
preimageHash_1(bytes32) := TMP_7036(bytes32)
 preimageHashes.remove(preimageHash)
TMP_7037(bool) = LIBRARY_CALL, dest:EnumerableSet, function:EnumerableSet.remove(EnumerableSet.Bytes32Set,bytes32), arguments:['preimageHashes_8', 'preimageHash_1'] 
 _maxCount --
TMP_7038(uint256) := _maxCount_2(uint256)
_maxCount_3(uint256) = _maxCount_2 (c)- 1
 UnusedPreimageHashRemoved(preimageHash)
Emit UnusedPreimageHashRemoved(preimageHash_1)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### CoreVaultManager.requestTransferFromCoreVault(string,bytes32,uint128,bool) [EXTERNAL]
```slithir
nextTransferRequestId_1(uint256) := phi(['nextTransferRequestId_5', 'nextTransferRequestId_0', 'nextTransferRequestId_4'])
cancelableTransferRequests_1(uint256[]) := phi(['cancelableTransferRequests_17', 'cancelableTransferRequests_13', 'cancelableTransferRequests_18', 'cancelableTransferRequests_0', 'cancelableTransferRequests_9', 'cancelableTransferRequests_3', 'cancelableTransferRequests_5'])
nonCancelableTransferRequests_1(uint256[]) := phi(['nonCancelableTransferRequests_5', 'nonCancelableTransferRequests_8', 'nonCancelableTransferRequests_13', 'nonCancelableTransferRequests_0', 'nonCancelableTransferRequests_12', 'nonCancelableTransferRequests_11', 'nonCancelableTransferRequests_3', 'nonCancelableTransferRequests_10'])
transferRequestById_1(mapping(uint256 => ICoreVaultManager.TransferRequest)) := phi(['transferRequestById_3', 'transferRequestById_8', 'transferRequestById_15', 'transferRequestById_12', 'transferRequestById_0', 'transferRequestById_11', 'transferRequestById_13', 'transferRequestById_14', 'transferRequestById_4', 'transferRequestById_5'])
allowedDestinationAddressIndex_1(mapping(string => uint256)) := phi(['allowedDestinationAddressIndex_11', 'allowedDestinationAddressIndex_3', 'allowedDestinationAddressIndex_5', 'allowedDestinationAddressIndex_0', 'allowedDestinationAddressIndex_8'])
availableFunds_4(uint128) := phi(['availableFunds_7', 'availableFunds_12', 'availableFunds_2', 'availableFunds_17', 'availableFunds_0', 'availableFunds_3', 'availableFunds_15', 'availableFunds_11'])
escrowedFunds_1(uint128) := phi(['escrowedFunds_4', 'escrowedFunds_0', 'escrowedFunds_7', 'escrowedFunds_13', 'escrowedFunds_8', 'escrowedFunds_11'])
cancelableTransferRequestsAmount_1(uint128) := phi(['cancelableTransferRequestsAmount_11', 'cancelableTransferRequestsAmount_4', 'cancelableTransferRequestsAmount_0', 'cancelableTransferRequestsAmount_7', 'cancelableTransferRequestsAmount_3'])
nonCancelableTransferRequestsAmount_1(uint128) := phi(['nonCancelableTransferRequestsAmount_3', 'nonCancelableTransferRequestsAmount_4', 'nonCancelableTransferRequestsAmount_8', 'nonCancelableTransferRequestsAmount_0'])
 require(bool,error)(_amount > 0,revert AmountZero()())
TMP_6855(bool) = _amount_1 > 0
TMP_6856(None) = SOLIDITY_CALL revert AmountZero()()
TMP_6857(None) = SOLIDITY_CALL require(bool,error)(TMP_6855,TMP_6856)
 require(bool,error)(allowedDestinationAddressIndex[_destinationAddress] != 0,revert DestinationNotAllowed()())
REF_4472(uint256) -> allowedDestinationAddressIndex_3[_destinationAddress_1]
TMP_6858(bool) = REF_4472 != 0
TMP_6859(None) = SOLIDITY_CALL revert DestinationNotAllowed()()
TMP_6860(None) = SOLIDITY_CALL require(bool,error)(TMP_6858,TMP_6859)
 destinationAddressHash = keccak256(bytes)(bytes(_destinationAddress))
TMP_6861 = CONVERT _destinationAddress_1 to bytes
TMP_6862(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_6861)
destinationAddressHash_1(bytes32) := TMP_6862(bytes32)
 newTransferRequest = false
newTransferRequest_1(bool) := False(bool)
 _cancelable
CONDITION _cancelable_1
 i = 0
i_1(uint256) := 0(uint256)
 i < cancelableTransferRequests.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_4473 -> LENGTH cancelableTransferRequests_3
TMP_6863(bool) = i_2 < REF_4473
CONDITION TMP_6863
 req = transferRequestById[cancelableTransferRequests[i]]
REF_4474(uint256) -> cancelableTransferRequests_3[i_2]
REF_4475(ICoreVaultManager.TransferRequest) -> transferRequestById_3[REF_4474]
req_1 (-> ['transferRequestById'])(ICoreVaultManager.TransferRequest) := REF_4475(ICoreVaultManager.TransferRequest)
 require(bool,error)(keccak256(bytes)(bytes(req.destinationAddress)) != destinationAddressHash,revert RequestExists()())
REF_4476(string) -> req_1 (-> ['transferRequestById']).destinationAddress
TMP_6864 = CONVERT REF_4476 to bytes
TMP_6865(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_6864)
TMP_6866(bool) = TMP_6865 != destinationAddressHash_1
TMP_6867(None) = SOLIDITY_CALL revert RequestExists()()
TMP_6868(None) = SOLIDITY_CALL require(bool,error)(TMP_6866,TMP_6867)
 i ++
TMP_6869(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 cancelableTransferRequestsAmount += _amount
cancelableTransferRequestsAmount_4(uint128) = cancelableTransferRequestsAmount_3 (c)+ _amount_1
 cancelableTransferRequests.push(nextTransferRequestId)
REF_4478 -> LENGTH cancelableTransferRequests_3
TMP_6871(uint256) := REF_4478(uint256)
TMP_6872(uint256) = TMP_6871 (c)+ 1
cancelableTransferRequests_4(uint256[]) := phi(['cancelableTransferRequests_3'])
REF_4478(uint256) (->cancelableTransferRequests_4) := TMP_6872(uint256)
REF_4479(uint256) -> cancelableTransferRequests_4[TMP_6871]
cancelableTransferRequests_5(uint256[]) := phi(['cancelableTransferRequests_4'])
REF_4479(uint256) (->cancelableTransferRequests_5) := nextTransferRequestId_3(uint256)
 newTransferRequest = true
newTransferRequest_2(bool) := True(bool)
 index = 0
index_1(uint256) := 0(uint256)
 index < nonCancelableTransferRequests.length
index_2(uint256) := phi(['index_3', 'index_1'])
REF_4480 -> LENGTH nonCancelableTransferRequests_3
TMP_6873(bool) = index_2 < REF_4480
CONDITION TMP_6873
 req_scope_0 = transferRequestById[nonCancelableTransferRequests[index]]
REF_4481(uint256) -> nonCancelableTransferRequests_3[index_2]
REF_4482(ICoreVaultManager.TransferRequest) -> transferRequestById_3[REF_4481]
req_scope_0_1 (-> ['transferRequestById'])(ICoreVaultManager.TransferRequest) := REF_4482(ICoreVaultManager.TransferRequest)
 keccak256(bytes)(bytes(req_scope_0.destinationAddress)) == destinationAddressHash
REF_4483(string) -> req_scope_0_1 (-> ['transferRequestById']).destinationAddress
TMP_6874 = CONVERT REF_4483 to bytes
TMP_6875(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_6874)
TMP_6876(bool) = TMP_6875 == destinationAddressHash_1
CONDITION TMP_6876
 req_scope_0.amount += _amount
REF_4484(uint128) -> req_scope_0_1 (-> ['transferRequestById']).amount
req_scope_0_2 (-> ['transferRequestById'])(ICoreVaultManager.TransferRequest) := phi(["req_scope_0_1 (-> ['transferRequestById'])"])
REF_4484(-> req_scope_0_2 (-> ['transferRequestById'])) = REF_4484 (c)+ _amount_1
transferRequestById_5(mapping(uint256 => ICoreVaultManager.TransferRequest)) := phi(["req_scope_0_2 (-> ['transferRequestById'])"])
 _paymentReference = req_scope_0.paymentReference
REF_4485(bytes32) -> req_scope_0_2 (-> ['transferRequestById']).paymentReference
_paymentReference_2(bytes32) := REF_4485(bytes32)
 index ++
TMP_6877(uint256) := index_2(uint256)
index_3(uint256) = index_2 (c)+ 1
_paymentReference_3(bytes32) := phi(['_paymentReference_1', '_paymentReference_2'])
 nonCancelableTransferRequestsAmount += _amount
nonCancelableTransferRequestsAmount_4(uint128) = nonCancelableTransferRequestsAmount_3 (c)+ _amount_1
 index == nonCancelableTransferRequests.length
REF_4486 -> LENGTH nonCancelableTransferRequests_3
TMP_6878(bool) = index_2 == REF_4486
CONDITION TMP_6878
 nonCancelableTransferRequests.push(nextTransferRequestId)
REF_4488 -> LENGTH nonCancelableTransferRequests_3
TMP_6880(uint256) := REF_4488(uint256)
TMP_6881(uint256) = TMP_6880 (c)+ 1
nonCancelableTransferRequests_4(uint256[]) := phi(['nonCancelableTransferRequests_3'])
REF_4488(uint256) (->nonCancelableTransferRequests_4) := TMP_6881(uint256)
REF_4489(uint256) -> nonCancelableTransferRequests_4[TMP_6880]
nonCancelableTransferRequests_5(uint256[]) := phi(['nonCancelableTransferRequests_4'])
REF_4489(uint256) (->nonCancelableTransferRequests_5) := nextTransferRequestId_3(uint256)
 newTransferRequest = true
newTransferRequest_3(bool) := True(bool)
newTransferRequest_4(bool) := phi(['newTransferRequest_3', 'newTransferRequest_1'])
newTransferRequest_5(bool) := phi(['newTransferRequest_1', 'newTransferRequest_2'])
 requestsAmount = totalRequestAmountWithFee()
TMP_6882(uint256) = INTERNAL_CALL, CoreVaultManager.totalRequestAmountWithFee()()
requestsAmount_1(uint256) := TMP_6882(uint256)
 require(bool,error)(requestsAmount <= availableFunds + escrowedFunds,revert InsufficientFunds()())
TMP_6883(uint128) = availableFunds_7 (c)+ escrowedFunds_4
TMP_6884(bool) = requestsAmount_1 <= TMP_6883
TMP_6885(None) = SOLIDITY_CALL revert InsufficientFunds()()
TMP_6886(None) = SOLIDITY_CALL require(bool,error)(TMP_6884,TMP_6885)
 newTransferRequest
CONDITION newTransferRequest_5
 transferRequestById[nextTransferRequestId ++] = TransferRequest({destinationAddress:_destinationAddress,paymentReference:_paymentReference,amount:_amount})
TMP_6887(uint256) := nextTransferRequestId_4(uint256)
nextTransferRequestId_5(uint256) = nextTransferRequestId_4 (c)+ 1
REF_4490(ICoreVaultManager.TransferRequest) -> transferRequestById_3[TMP_6887]
TMP_6888(ICoreVaultManager.TransferRequest) = new TransferRequest(_destinationAddress_1,_paymentReference_3,_amount_1)
transferRequestById_4(mapping(uint256 => ICoreVaultManager.TransferRequest)) := phi(['transferRequestById_3'])
REF_4490(ICoreVaultManager.TransferRequest) (->transferRequestById_4) := TMP_6888(ICoreVaultManager.TransferRequest)
 TransferRequested(_destinationAddress,_paymentReference,_amount,_cancelable)
Emit TransferRequested(_destinationAddress_1,_paymentReference_3,_amount_1,_cancelable_1)
 _paymentReference
RETURN _paymentReference_3
 onlyAssetManager()
MODIFIER_CALL, CoreVaultManager.onlyAssetManager()()
 notPaused()
MODIFIER_CALL, CoreVaultManager.notPaused()()
```
#### CoreVaultManager.setEscrowsFinished(bytes32[]) [EXTERNAL]
```slithir
preimageHashToEscrowIndex_2(mapping(bytes32 => uint256)) := phi(['preimageHashToEscrowIndex_1', 'preimageHashToEscrowIndex_0', 'preimageHashToEscrowIndex_3', 'preimageHashToEscrowIndex_5'])
nextUnprocessedEscrowIndex_1(uint256) := phi(['nextUnprocessedEscrowIndex_2', 'nextUnprocessedEscrowIndex_6', 'nextUnprocessedEscrowIndex_0'])
availableFunds_13(uint128) := phi(['availableFunds_7', 'availableFunds_12', 'availableFunds_2', 'availableFunds_17', 'availableFunds_0', 'availableFunds_3', 'availableFunds_15', 'availableFunds_11'])
escrowedFunds_9(uint128) := phi(['escrowedFunds_4', 'escrowedFunds_0', 'escrowedFunds_7', 'escrowedFunds_13', 'escrowedFunds_8', 'escrowedFunds_11'])
 availableFundsTmp = availableFunds
availableFundsTmp_1(uint128) := availableFunds_14(uint128)
 escrowedFundsTmp = escrowedFunds
escrowedFundsTmp_1(uint128) := escrowedFunds_10(uint128)
 i = 0
i_1(uint256) := 0(uint256)
 i < _preimageHashes.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_4595 -> LENGTH _preimageHashes_1
TMP_7041(bool) = i_2 < REF_4595
CONDITION TMP_7041
 escrowIndex = preimageHashToEscrowIndex[_preimageHashes[i]]
REF_4596(bytes32) -> _preimageHashes_1[i_2]
REF_4597(uint256) -> preimageHashToEscrowIndex_3[REF_4596]
escrowIndex_1(uint256) := REF_4597(uint256)
 escrow = _getEscrow(escrowIndex)
TMP_7042(ICoreVaultManager.Escrow) = INTERNAL_CALL, CoreVaultManager._getEscrow(uint256)(escrowIndex_1)
escrow_1 (-> ['TMP_7042'])(ICoreVaultManager.Escrow) := TMP_7042(ICoreVaultManager.Escrow)
 require(bool,error)(! escrow.finished,revert EscrowAlreadyFinished()())
REF_4598(bool) -> escrow_1 (-> ['TMP_7042']).finished
TMP_7043 = UnaryType.BANG REF_4598 
TMP_7044(None) = SOLIDITY_CALL revert EscrowAlreadyFinished()()
TMP_7045(None) = SOLIDITY_CALL require(bool,error)(TMP_7043,TMP_7044)
 escrow.finished = true
REF_4599(bool) -> escrow_1 (-> ['TMP_7042']).finished
escrow_2 (-> ['TMP_7042'])(ICoreVaultManager.Escrow) := phi(["escrow_1 (-> ['TMP_7042'])"])
REF_4599(bool) (->escrow_2 (-> ['TMP_7042'])) := True(bool)
TMP_7042(ICoreVaultManager.Escrow) := phi(["escrow_2 (-> ['TMP_7042'])"])
 escrowIndex <= nextUnprocessedEscrowIndex
TMP_7046(bool) = escrowIndex_1 <= nextUnprocessedEscrowIndex_3
CONDITION TMP_7046
 availableFundsTmp -= escrow.amount
REF_4600(uint128) -> escrow_2 (-> ['TMP_7042']).amount
availableFundsTmp_2(uint128) = availableFundsTmp_1 (c)- REF_4600
 escrowedFundsTmp -= escrow.amount
REF_4601(uint128) -> escrow_2 (-> ['TMP_7042']).amount
escrowedFundsTmp_2(uint128) = escrowedFundsTmp_1 (c)- REF_4601
availableFundsTmp_3(uint128) := phi(['availableFundsTmp_1', 'availableFundsTmp_2'])
escrowedFundsTmp_3(uint128) := phi(['escrowedFundsTmp_1', 'escrowedFundsTmp_2'])
 EscrowFinished(_preimageHashes[i],escrow.amount)
REF_4602(bytes32) -> _preimageHashes_1[i_2]
REF_4603(uint128) -> escrow_2 (-> ['TMP_7042']).amount
Emit EscrowFinished(REF_4602,REF_4603)
 i ++
TMP_7048(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 availableFunds = availableFundsTmp
availableFunds_15(uint128) := availableFundsTmp_1(uint128)
 escrowedFunds = escrowedFundsTmp
escrowedFunds_11(uint128) := escrowedFundsTmp_1(uint128)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```

#### CoreVaultManager.supportsInterface(bytes4) [EXTERNAL]
```slithir
 _interfaceId == type()(IERC165).interfaceId || _interfaceId == type()(IIAddressUpdatable).interfaceId || _interfaceId == type()(IICoreVaultManager).interfaceId
TMP_7104(type(IERC165)) = SOLIDITY_CALL type()(IERC165)
REF_4639(bytes4) (->None) := 33540519(bytes4)
TMP_7105(bool) = _interfaceId_1 == REF_4639
TMP_7106(type(IIAddressUpdatable)) = SOLIDITY_CALL type()(IIAddressUpdatable)
REF_4640(bytes4) (->None) := 2953579382(bytes4)
TMP_7107(bool) = _interfaceId_1 == REF_4640
TMP_7108(bool) = TMP_7105 || TMP_7107
TMP_7109(type(IICoreVaultManager)) = SOLIDITY_CALL type()(IICoreVaultManager)
REF_4641(bytes4) (->None) := 2852172343(bytes4)
TMP_7110(bool) = _interfaceId_1 == REF_4641
TMP_7111(bool) = TMP_7108 || TMP_7110
RETURN TMP_7111
```
#### CoreVaultManager.totalRequestAmountWithFee() [PUBLIC]
```slithir
cancelableTransferRequests_18(uint256[]) := phi(['cancelableTransferRequests_17', 'cancelableTransferRequests_13', 'cancelableTransferRequests_18', 'cancelableTransferRequests_0', 'cancelableTransferRequests_9', 'cancelableTransferRequests_3', 'cancelableTransferRequests_5'])
nonCancelableTransferRequests_13(uint256[]) := phi(['nonCancelableTransferRequests_5', 'nonCancelableTransferRequests_8', 'nonCancelableTransferRequests_13', 'nonCancelableTransferRequests_0', 'nonCancelableTransferRequests_12', 'nonCancelableTransferRequests_11', 'nonCancelableTransferRequests_3', 'nonCancelableTransferRequests_10'])
fee_6(uint128) := phi(['fee_0', 'fee_4', 'fee_3'])
cancelableTransferRequestsAmount_12(uint128) := phi(['cancelableTransferRequestsAmount_11', 'cancelableTransferRequestsAmount_4', 'cancelableTransferRequestsAmount_0', 'cancelableTransferRequestsAmount_7', 'cancelableTransferRequestsAmount_3'])
nonCancelableTransferRequestsAmount_9(uint128) := phi(['nonCancelableTransferRequestsAmount_3', 'nonCancelableTransferRequestsAmount_4', 'nonCancelableTransferRequestsAmount_8', 'nonCancelableTransferRequestsAmount_0'])
 nonCancelableTransferRequestsAmount + cancelableTransferRequestsAmount + (cancelableTransferRequests.length + nonCancelableTransferRequests.length) * fee
TMP_7099(uint128) = nonCancelableTransferRequestsAmount_9 (c)+ cancelableTransferRequestsAmount_12
REF_4636 -> LENGTH cancelableTransferRequests_18
REF_4637 -> LENGTH nonCancelableTransferRequests_13
TMP_7100(uint256) = REF_4636 (c)+ REF_4637
TMP_7101(uint256) = TMP_7100 (c)* fee_6
TMP_7102(uint128) = TMP_7099 (c)+ TMP_7101
RETURN TMP_7102
```
#### CoreVaultManager.triggerCustomInstructions(bytes32) [EXTERNAL]
```slithir
coreVaultAddress_6(string) := phi(['coreVaultAddress_1', 'coreVaultAddress_4', 'coreVaultAddress_0', 'coreVaultAddress_5', 'coreVaultAddress_7'])
nextSequenceNumber_7(uint256) := phi(['nextSequenceNumber_1', 'nextSequenceNumber_5', 'nextSequenceNumber_6', 'nextSequenceNumber_0', 'nextSequenceNumber_9'])
 CustomInstructions(nextSequenceNumber ++,coreVaultAddress,_instructionsHash)
TMP_7069(uint256) := nextSequenceNumber_8(uint256)
nextSequenceNumber_9(uint256) = nextSequenceNumber_8 (c)+ 1
Emit CustomInstructions(TMP_7069,coreVaultAddress_7,_instructionsHash_1)
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### CoreVaultManager.triggerInstructions() [EXTERNAL]
```slithir
custodianAddress_2(string) := phi(['custodianAddress_1', 'custodianAddress_4', 'custodianAddress_0', 'custodianAddress_5', 'custodianAddress_6'])
coreVaultAddress_2(string) := phi(['coreVaultAddress_1', 'coreVaultAddress_4', 'coreVaultAddress_0', 'coreVaultAddress_5', 'coreVaultAddress_7'])
nextSequenceNumber_2(uint256) := phi(['nextSequenceNumber_1', 'nextSequenceNumber_5', 'nextSequenceNumber_6', 'nextSequenceNumber_0', 'nextSequenceNumber_9'])
preimageHashes_1(EnumerableSet.Bytes32Set) := phi(['preimageHashes_6', 'preimageHashes_3', 'preimageHashes_4', 'preimageHashes_0', 'preimageHashes_8'])
escrows_1(ICoreVaultManager.Escrow[]) := phi(['escrows_4', 'escrows_7', 'escrows_11', 'escrows_6', 'escrows_8', 'escrows_12', 'escrows_3', 'escrows_10', 'escrows_0', 'escrows_9'])
nextUnusedPreimageHashIndex_1(uint256) := phi(['nextUnusedPreimageHashIndex_3', 'nextUnusedPreimageHashIndex_4', 'nextUnusedPreimageHashIndex_0', 'nextUnusedPreimageHashIndex_6'])
cancelableTransferRequests_11(uint256[]) := phi(['cancelableTransferRequests_17', 'cancelableTransferRequests_13', 'cancelableTransferRequests_18', 'cancelableTransferRequests_0', 'cancelableTransferRequests_9', 'cancelableTransferRequests_3', 'cancelableTransferRequests_5'])
nonCancelableTransferRequests_6(uint256[]) := phi(['nonCancelableTransferRequests_5', 'nonCancelableTransferRequests_8', 'nonCancelableTransferRequests_13', 'nonCancelableTransferRequests_0', 'nonCancelableTransferRequests_12', 'nonCancelableTransferRequests_11', 'nonCancelableTransferRequests_3', 'nonCancelableTransferRequests_10'])
transferRequestById_9(mapping(uint256 => ICoreVaultManager.TransferRequest)) := phi(['transferRequestById_3', 'transferRequestById_8', 'transferRequestById_15', 'transferRequestById_12', 'transferRequestById_0', 'transferRequestById_11', 'transferRequestById_13', 'transferRequestById_14', 'transferRequestById_4', 'transferRequestById_5'])
triggeringAccounts_1(EnumerableSet.AddressSet) := phi(['triggeringAccounts_2', 'triggeringAccounts_4', 'triggeringAccounts_0', 'triggeringAccounts_6'])
escrowAmount_1(uint128) := phi(['escrowAmount_3', 'escrowAmount_4', 'escrowAmount_0'])
minimalAmount_1(uint128) := phi(['minimalAmount_0', 'minimalAmount_3', 'minimalAmount_4'])
fee_1(uint128) := phi(['fee_0', 'fee_4', 'fee_3'])
availableFunds_8(uint128) := phi(['availableFunds_7', 'availableFunds_12', 'availableFunds_2', 'availableFunds_17', 'availableFunds_0', 'availableFunds_3', 'availableFunds_15', 'availableFunds_11'])
escrowedFunds_5(uint128) := phi(['escrowedFunds_4', 'escrowedFunds_0', 'escrowedFunds_7', 'escrowedFunds_13', 'escrowedFunds_8', 'escrowedFunds_11'])
cancelableTransferRequestsAmount_8(uint128) := phi(['cancelableTransferRequestsAmount_11', 'cancelableTransferRequestsAmount_4', 'cancelableTransferRequestsAmount_0', 'cancelableTransferRequestsAmount_7', 'cancelableTransferRequestsAmount_3'])
nonCancelableTransferRequestsAmount_5(uint128) := phi(['nonCancelableTransferRequestsAmount_3', 'nonCancelableTransferRequestsAmount_4', 'nonCancelableTransferRequestsAmount_8', 'nonCancelableTransferRequestsAmount_0'])
 require(bool,error)(triggeringAccounts.contains(msg.sender),revert NotAuthorized()())
TMP_6911(bool) = LIBRARY_CALL, dest:EnumerableSet, function:EnumerableSet.contains(EnumerableSet.AddressSet,address), arguments:['triggeringAccounts_2', 'msg.sender'] 
TMP_6912(None) = SOLIDITY_CALL revert NotAuthorized()()
TMP_6913(None) = SOLIDITY_CALL require(bool,error)(TMP_6911,TMP_6912)
 _processEscrows(type()(uint256).max)
TMP_6915(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
TMP_6916(bool) = INTERNAL_CALL, CoreVaultManager._processEscrows(uint256)(TMP_6915)
escrows_3(ICoreVaultManager.Escrow[]) := phi(['escrows_10'])
availableFunds_10(uint128) := phi(['availableFunds_17'])
escrowedFunds_7(uint128) := phi(['escrowedFunds_13'])
 availableFundsTmp = availableFunds
availableFundsTmp_1(uint128) := availableFunds_10(uint128)
 sequenceNumberTmp = nextSequenceNumber
sequenceNumberTmp_1(uint256) := nextSequenceNumber_4(uint256)
 feeTmp = fee
feeTmp_1(uint128) := fee_3(uint128)
 require(bool,error)(feeTmp > 0,revert FeeZero()())
TMP_6917(bool) = feeTmp_1 > 0
TMP_6918(None) = SOLIDITY_CALL revert FeeZero()()
TMP_6919(None) = SOLIDITY_CALL require(bool,error)(TMP_6917,TMP_6918)
 index = 0
index_1(uint256) := 0(uint256)
 length = cancelableTransferRequests.length
REF_4509 -> LENGTH cancelableTransferRequests_13
length_1(uint256) := REF_4509(uint256)
 amountTmp = cancelableTransferRequestsAmount
amountTmp_1(uint128) := cancelableTransferRequestsAmount_10(uint128)
 index < length
TMP_6920(bool) = index_1 < length_1
CONDITION TMP_6920
 transferRequestId = cancelableTransferRequests[index]
REF_4510(uint256) -> cancelableTransferRequests_13[index_1]
transferRequestId_1(uint256) := REF_4510(uint256)
 availableFundsTmp >= transferRequestById[transferRequestId].amount + feeTmp
REF_4511(ICoreVaultManager.TransferRequest) -> transferRequestById_11[transferRequestId_1]
REF_4512(uint128) -> REF_4511.amount
TMP_6921(uint128) = REF_4512 (c)+ feeTmp_1
TMP_6922(bool) = availableFundsTmp_1 >= TMP_6921
CONDITION TMP_6922
 req = transferRequestById[transferRequestId]
REF_4513(ICoreVaultManager.TransferRequest) -> transferRequestById_11[transferRequestId_1]
req_1(ICoreVaultManager.TransferRequest) := REF_4513(ICoreVaultManager.TransferRequest)
 availableFundsTmp -= (req.amount + feeTmp)
REF_4514(uint128) -> req_1.amount
TMP_6923(uint128) = REF_4514 (c)+ feeTmp_1
availableFundsTmp_2(uint128) = availableFundsTmp_1 (c)- TMP_6923
 amountTmp -= req.amount
REF_4515(uint128) -> req_1.amount
amountTmp_2(uint128) = amountTmp_1 (c)- REF_4515
 PaymentInstructions(sequenceNumberTmp ++,coreVaultAddress,req.destinationAddress,req.amount,feeTmp,req.paymentReference)
TMP_6924(uint256) := sequenceNumberTmp_1(uint256)
sequenceNumberTmp_2(uint256) = sequenceNumberTmp_1 (c)+ 1
REF_4516(string) -> req_1.destinationAddress
REF_4517(uint128) -> req_1.amount
REF_4518(bytes32) -> req_1.paymentReference
Emit PaymentInstructions(TMP_6924,coreVaultAddress_4,REF_4516,REF_4517,feeTmp_1,REF_4518)
 _numberOfInstructions ++
TMP_6926(uint256) := _numberOfInstructions_0(uint256)
_numberOfInstructions_1(uint256) = _numberOfInstructions_0 (c)+ 1
 i = index
i_1(uint256) := index_1(uint256)
 i < length - 1
i_2(uint256) := phi(['i_1', 'i_3'])
TMP_6927(uint256) = length_1 (c)- 1
TMP_6928(bool) = i_2 < TMP_6927
CONDITION TMP_6928
 cancelableTransferRequests[i] = cancelableTransferRequests[i + 1]
REF_4519(uint256) -> cancelableTransferRequests_13[i_2]
TMP_6929(uint256) = i_2 (c)+ 1
REF_4520(uint256) -> cancelableTransferRequests_13[TMP_6929]
cancelableTransferRequests_14(uint256[]) := phi(['cancelableTransferRequests_13'])
REF_4519(uint256) (->cancelableTransferRequests_14) := REF_4520(uint256)
 i ++
TMP_6930(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 cancelableTransferRequests.pop()
REF_4522 -> LENGTH cancelableTransferRequests_13
TMP_6932(uint256) = REF_4522 (c)- 1
REF_4523(uint256) -> cancelableTransferRequests_13[TMP_6932]
cancelableTransferRequests_15 = delete REF_4523 
REF_4524 -> LENGTH cancelableTransferRequests_15
cancelableTransferRequests_16(uint256[]) := phi(['cancelableTransferRequests_15'])
REF_4524(uint256) (->cancelableTransferRequests_16) := TMP_6932(uint256)
 delete transferRequestById[transferRequestId]
REF_4525(ICoreVaultManager.TransferRequest) -> transferRequestById_11[transferRequestId_1]
transferRequestById_12 = delete REF_4525 
 length --
TMP_6933(uint256) := length_1(uint256)
length_2(uint256) = length_1 (c)- 1
 index ++
TMP_6934(uint256) := index_1(uint256)
index_2(uint256) = index_1 (c)+ 1
_numberOfInstructions_2(uint256) := phi(['_numberOfInstructions_0', '_numberOfInstructions_1'])
availableFundsTmp_3(uint128) := phi(['availableFundsTmp_1', 'availableFundsTmp_2'])
sequenceNumberTmp_3(uint256) := phi(['sequenceNumberTmp_1', 'sequenceNumberTmp_2'])
index_3(uint256) := phi(['index_1', 'index_2'])
length_3(uint256) := phi(['length_1', 'length_2'])
amountTmp_3(uint128) := phi(['amountTmp_2', 'amountTmp_1'])
 cancelableTransferRequestsAmount = amountTmp
cancelableTransferRequestsAmount_11(uint128) := amountTmp_1(uint128)
 index = 0
index_4(uint256) := 0(uint256)
 length = nonCancelableTransferRequests.length
REF_4526 -> LENGTH nonCancelableTransferRequests_8
length_4(uint256) := REF_4526(uint256)
 amountTmp = nonCancelableTransferRequestsAmount
amountTmp_4(uint128) := nonCancelableTransferRequestsAmount_7(uint128)
 index < length
TMP_6935(bool) = index_4 < length_4
CONDITION TMP_6935
 transferRequestId_scope_0 = nonCancelableTransferRequests[index]
REF_4527(uint256) -> nonCancelableTransferRequests_8[index_4]
transferRequestId_scope_0_1(uint256) := REF_4527(uint256)
 availableFundsTmp >= transferRequestById[transferRequestId_scope_0].amount + feeTmp
REF_4528(ICoreVaultManager.TransferRequest) -> transferRequestById_11[transferRequestId_scope_0_1]
REF_4529(uint128) -> REF_4528.amount
TMP_6936(uint128) = REF_4529 (c)+ feeTmp_1
TMP_6937(bool) = availableFundsTmp_1 >= TMP_6936
CONDITION TMP_6937
 req_scope_1 = transferRequestById[transferRequestId_scope_0]
REF_4530(ICoreVaultManager.TransferRequest) -> transferRequestById_11[transferRequestId_scope_0_1]
req_scope_1_1(ICoreVaultManager.TransferRequest) := REF_4530(ICoreVaultManager.TransferRequest)
 availableFundsTmp -= (req_scope_1.amount + feeTmp)
REF_4531(uint128) -> req_scope_1_1.amount
TMP_6938(uint128) = REF_4531 (c)+ feeTmp_1
availableFundsTmp_4(uint128) = availableFundsTmp_1 (c)- TMP_6938
 amountTmp -= req_scope_1.amount
REF_4532(uint128) -> req_scope_1_1.amount
amountTmp_5(uint128) = amountTmp_4 (c)- REF_4532
 PaymentInstructions(sequenceNumberTmp ++,coreVaultAddress,req_scope_1.destinationAddress,req_scope_1.amount,feeTmp,req_scope_1.paymentReference)
TMP_6939(uint256) := sequenceNumberTmp_1(uint256)
sequenceNumberTmp_4(uint256) = sequenceNumberTmp_1 (c)+ 1
REF_4533(string) -> req_scope_1_1.destinationAddress
REF_4534(uint128) -> req_scope_1_1.amount
REF_4535(bytes32) -> req_scope_1_1.paymentReference
Emit PaymentInstructions(TMP_6939,coreVaultAddress_4,REF_4533,REF_4534,feeTmp_1,REF_4535)
 _numberOfInstructions ++
TMP_6941(uint256) := _numberOfInstructions_0(uint256)
_numberOfInstructions_3(uint256) = _numberOfInstructions_0 (c)+ 1
 i_scope_2 = index
i_scope_2_1(uint256) := index_4(uint256)
 i_scope_2 < length - 1
i_scope_2_2(uint256) := phi(['i_scope_2_1', 'i_scope_2_3'])
TMP_6942(uint256) = length_4 (c)- 1
TMP_6943(bool) = i_scope_2_2 < TMP_6942
CONDITION TMP_6943
 nonCancelableTransferRequests[i_scope_2] = nonCancelableTransferRequests[i_scope_2 + 1]
REF_4536(uint256) -> nonCancelableTransferRequests_8[i_scope_2_2]
TMP_6944(uint256) = i_scope_2_2 (c)+ 1
REF_4537(uint256) -> nonCancelableTransferRequests_8[TMP_6944]
nonCancelableTransferRequests_11(uint256[]) := phi(['nonCancelableTransferRequests_8'])
REF_4536(uint256) (->nonCancelableTransferRequests_11) := REF_4537(uint256)
 i_scope_2 ++
TMP_6945(uint256) := i_scope_2_2(uint256)
i_scope_2_3(uint256) = i_scope_2_2 (c)+ 1
 nonCancelableTransferRequests.pop()
REF_4539 -> LENGTH nonCancelableTransferRequests_8
TMP_6947(uint256) = REF_4539 (c)- 1
REF_4540(uint256) -> nonCancelableTransferRequests_8[TMP_6947]
nonCancelableTransferRequests_9 = delete REF_4540 
REF_4541 -> LENGTH nonCancelableTransferRequests_9
nonCancelableTransferRequests_10(uint256[]) := phi(['nonCancelableTransferRequests_9'])
REF_4541(uint256) (->nonCancelableTransferRequests_10) := TMP_6947(uint256)
 delete transferRequestById[transferRequestId_scope_0]
REF_4542(ICoreVaultManager.TransferRequest) -> transferRequestById_11[transferRequestId_scope_0_1]
transferRequestById_13 = delete REF_4542 
 length --
TMP_6948(uint256) := length_4(uint256)
length_5(uint256) = length_4 (c)- 1
 index ++
TMP_6949(uint256) := index_4(uint256)
index_5(uint256) = index_4 (c)+ 1
_numberOfInstructions_4(uint256) := phi(['_numberOfInstructions_3', '_numberOfInstructions_0'])
availableFundsTmp_5(uint128) := phi(['availableFundsTmp_1', 'availableFundsTmp_4'])
sequenceNumberTmp_5(uint256) := phi(['sequenceNumberTmp_4', 'sequenceNumberTmp_1'])
index_6(uint256) := phi(['index_4', 'index_5'])
length_6(uint256) := phi(['length_4', 'length_5'])
amountTmp_6(uint128) := phi(['amountTmp_5', 'amountTmp_4'])
 nonCancelableTransferRequestsAmount = amountTmp
nonCancelableTransferRequestsAmount_8(uint128) := amountTmp_4(uint128)
 escrowAmountTmp = escrowAmount
escrowAmountTmp_1(uint128) := escrowAmount_3(uint128)
 escrowAmountTmp == 0 || length > 0 || cancelableTransferRequests.length > 0
TMP_6950(bool) = escrowAmountTmp_1 == 0
TMP_6951(bool) = length_4 > 0
TMP_6952(bool) = TMP_6950 || TMP_6951
REF_4543 -> LENGTH cancelableTransferRequests_13
TMP_6953(bool) = REF_4543 > 0
TMP_6954(bool) = TMP_6952 || TMP_6953
CONDITION TMP_6954
 availableFunds = availableFundsTmp
availableFunds_12(uint128) := availableFundsTmp_1(uint128)
 nextSequenceNumber = sequenceNumberTmp
nextSequenceNumber_6(uint256) := sequenceNumberTmp_1(uint256)
 _numberOfInstructions
RETURN _numberOfInstructions_0
 preimageHashIndexTmp = nextUnusedPreimageHashIndex
preimageHashIndexTmp_1(uint256) := nextUnusedPreimageHashIndex_3(uint256)
 minFundsToTriggerEscrow = minimalAmount + escrowAmountTmp + feeTmp
TMP_6955(uint128) = minimalAmount_3 (c)+ escrowAmountTmp_1
TMP_6956(uint128) = TMP_6955 (c)+ feeTmp_1
minFundsToTriggerEscrow_1(uint256) := TMP_6956(uint128)
 length = preimageHashes.length()
TMP_6957(uint256) = LIBRARY_CALL, dest:EnumerableSet, function:EnumerableSet.length(EnumerableSet.Bytes32Set), arguments:['preimageHashes_3'] 
length_7(uint256) := TMP_6957(uint256)
 amountTmp = escrowedFunds
amountTmp_7(uint128) := escrowedFunds_7(uint128)
 availableFundsTmp >= minFundsToTriggerEscrow && preimageHashIndexTmp < length
TMP_6958(bool) = availableFundsTmp_1 >= minFundsToTriggerEscrow_1
TMP_6959(bool) = preimageHashIndexTmp_1 < length_7
TMP_6960(bool) = TMP_6958 && TMP_6959
CONDITION TMP_6960
 escrowEndTimestamp = _getNextEscrowEndTimestamp()
TMP_6961(uint64) = INTERNAL_CALL, CoreVaultManager._getNextEscrowEndTimestamp()()
escrows_4(ICoreVaultManager.Escrow[]) := phi(['escrows_12'])
escrowEndTimestamp_1(uint64) := TMP_6961(uint64)
 availableFundsTmp >= minFundsToTriggerEscrow && preimageHashIndexTmp < length
_numberOfInstructions_5(uint256) := phi(['_numberOfInstructions_0', '_numberOfInstructions_6'])
availableFundsTmp_6(uint128) := phi(['availableFundsTmp_1', 'availableFundsTmp_7'])
sequenceNumberTmp_6(uint256) := phi(['sequenceNumberTmp_7', 'sequenceNumberTmp_1'])
amountTmp_8(uint128) := phi(['amountTmp_9', 'amountTmp_7'])
preimageHashIndexTmp_2(uint256) := phi(['preimageHashIndexTmp_1', 'preimageHashIndexTmp_3'])
escrowEndTimestamp_2(uint64) := phi(['escrowEndTimestamp_3', 'escrowEndTimestamp_1'])
TMP_6962(bool) = availableFundsTmp_6 >= minFundsToTriggerEscrow_1
TMP_6963(bool) = preimageHashIndexTmp_2 < length_7
TMP_6964(bool) = TMP_6962 && TMP_6963
CONDITION TMP_6964
 availableFundsTmp -= (escrowAmountTmp + feeTmp)
TMP_6965(uint128) = escrowAmountTmp_1 (c)+ feeTmp_1
availableFundsTmp_7(uint128) = availableFundsTmp_6 (c)- TMP_6965
 amountTmp += escrowAmountTmp
amountTmp_9(uint128) = amountTmp_8 (c)+ escrowAmountTmp_1
 preimageHash = preimageHashes.at(preimageHashIndexTmp ++)
TMP_6966(uint256) := preimageHashIndexTmp_2(uint256)
preimageHashIndexTmp_3(uint256) = preimageHashIndexTmp_2 (c)+ 1
TMP_6967(bytes32) = LIBRARY_CALL, dest:EnumerableSet, function:EnumerableSet.at(EnumerableSet.Bytes32Set,uint256), arguments:['preimageHashes_4', 'TMP_6966'] 
preimageHash_1(bytes32) := TMP_6967(bytes32)
 escrow = Escrow({preimageHash:preimageHash,amount:escrowAmountTmp,expiryTs:escrowEndTimestamp,finished:false})
TMP_6968(ICoreVaultManager.Escrow) = new Escrow(preimageHash_1,escrowAmountTmp_1,escrowEndTimestamp_2,False)
escrow_1(ICoreVaultManager.Escrow) := TMP_6968(ICoreVaultManager.Escrow)
 escrows.push(escrow)
REF_4547 -> LENGTH escrows_4
TMP_6970(uint256) := REF_4547(uint256)
TMP_6971(uint256) = TMP_6970 (c)+ 1
escrows_5(ICoreVaultManager.Escrow[]) := phi(['escrows_4'])
REF_4547(uint256) (->escrows_5) := TMP_6971(uint256)
REF_4548(ICoreVaultManager.Escrow) -> escrows_5[TMP_6970]
escrows_6(ICoreVaultManager.Escrow[]) := phi(['escrows_5'])
REF_4548(ICoreVaultManager.Escrow) (->escrows_6) := escrow_1(ICoreVaultManager.Escrow)
 preimageHashToEscrowIndex[preimageHash] = escrows.length
REF_4549(uint256) -> preimageHashToEscrowIndex_0[preimageHash_1]
REF_4550 -> LENGTH escrows_6
preimageHashToEscrowIndex_1(mapping(bytes32 => uint256)) := phi(['preimageHashToEscrowIndex_0'])
REF_4549(uint256) (->preimageHashToEscrowIndex_1) := REF_4550(uint256)
 EscrowInstructions(sequenceNumberTmp ++,preimageHash,coreVaultAddress,custodianAddress,escrowAmountTmp,feeTmp,escrowEndTimestamp)
TMP_6972(uint256) := sequenceNumberTmp_6(uint256)
sequenceNumberTmp_7(uint256) = sequenceNumberTmp_6 (c)+ 1
Emit EscrowInstructions(TMP_6972,preimageHash_1,coreVaultAddress_5,custodianAddress_5,escrowAmountTmp_1,feeTmp_1,escrowEndTimestamp_2)
 _numberOfInstructions ++
TMP_6974(uint256) := _numberOfInstructions_5(uint256)
_numberOfInstructions_6(uint256) = _numberOfInstructions_5 (c)+ 1
 escrowEndTimestamp += 86400
escrowEndTimestamp_3(uint64) = escrowEndTimestamp_2 (c)+ 86400
 nextUnusedPreimageHashIndex = preimageHashIndexTmp
nextUnusedPreimageHashIndex_4(uint256) := preimageHashIndexTmp_2(uint256)
 availableFunds = availableFundsTmp
availableFunds_11(uint128) := availableFundsTmp_6(uint128)
 nextSequenceNumber = sequenceNumberTmp
nextSequenceNumber_5(uint256) := sequenceNumberTmp_6(uint256)
 escrowedFunds = amountTmp
escrowedFunds_8(uint128) := amountTmp_8(uint128)
 notPaused()
MODIFIER_CALL, CoreVaultManager.notPaused()()
 _numberOfInstructions
RETURN _numberOfInstructions_5
```
#### CoreVaultManager.unpause() [EXTERNAL]
```slithir
 paused = false
paused_2(bool) := False(bool)
 Unpaused()
Emit Unpaused()
 onlyImmediateGovernance()
MODIFIER_CALL, GovernedBase.onlyImmediateGovernance()()
```
#### CoreVaultManager.updateCustodianAddress(string) [EXTERNAL]
```slithir
 require(bool,error)(bytes(_custodianAddress).length > 0,revert InvalidAddress()())
TMP_7008 = CONVERT _custodianAddress_1 to bytes
REF_4586 -> LENGTH TMP_7008
TMP_7009(bool) = REF_4586 > 0
TMP_7010(None) = SOLIDITY_CALL revert InvalidAddress()()
TMP_7011(None) = SOLIDITY_CALL require(bool,error)(TMP_7009,TMP_7010)
 custodianAddress = _custodianAddress
custodianAddress_6(string) := _custodianAddress_1(string)
 CustodianAddressUpdated(_custodianAddress)
Emit CustodianAddressUpdated(_custodianAddress_1)
 onlyGovernance()
MODIFIER_CALL, GovernedBase.onlyGovernance()()
```
#### CoreVaultManager.updateSettings(uint128,uint128,uint128,uint128) [EXTERNAL]
```slithir
 require(bool,error)(_escrowEndTimeSeconds < 86400,revert InvalidEndTime()())
TMP_7014(bool) = _escrowEndTimeSeconds_1 < 86400
TMP_7015(None) = SOLIDITY_CALL revert InvalidEndTime()()
TMP_7016(None) = SOLIDITY_CALL require(bool,error)(TMP_7014,TMP_7015)
 require(bool,error)(_fee > 0,revert FeeZero()())
TMP_7017(bool) = _fee_1 > 0
TMP_7018(None) = SOLIDITY_CALL revert FeeZero()()
TMP_7019(None) = SOLIDITY_CALL require(bool,error)(TMP_7017,TMP_7018)
 escrowEndTimeSeconds = _escrowEndTimeSeconds
escrowEndTimeSeconds_1(uint128) := _escrowEndTimeSeconds_1(uint128)
 escrowAmount = _escrowAmount
escrowAmount_4(uint128) := _escrowAmount_1(uint128)
 minimalAmount = _minimalAmount
minimalAmount_4(uint128) := _minimalAmount_1(uint128)
 fee = _fee
fee_4(uint128) := _fee_1(uint128)
 SettingsUpdated(_escrowEndTimeSeconds,_escrowAmount,_minimalAmount,_fee)
Emit SettingsUpdated(_escrowEndTimeSeconds_1,_escrowAmount_1,_minimalAmount_1,_fee_1)
 onlyGovernance()
MODIFIER_CALL, GovernedBase.onlyGovernance()()
```
#### Math.max(uint256,uint256) [INTERNAL]
```slithir
 a > b
TMP_476(bool) = a_1 > b_1
CONDITION TMP_476
 a
RETURN a_1
 b
RETURN b_1
```
#### AddressUpdatable._getContractAddress(bytes32[],address[],string) [INTERNAL]
```slithir
_nameHashes_1(bytes32[]) := phi(['_contractNameHashes_1'])
_addresses_1(address[]) := phi(['_contractAddresses_1'])
 nameHash = keccak256(bytes)(abi.encode(_nameToFind))
TMP_8165(bytes) = SOLIDITY_CALL abi.encode()(_nameToFind_1)
TMP_8166(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_8165)
nameHash_1(bytes32) := TMP_8166(bytes32)
 a = address(0)
TMP_8167 = CONVERT 0 to address
a_1(address) := TMP_8167(address)
a_3(address) := phi(['a_1', 'a_2'])
 i = 0
i_1(uint256) := 0(uint256)
 i < _nameHashes.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_5176 -> LENGTH _nameHashes_1
TMP_8168(bool) = i_2 < REF_5176
CONDITION TMP_8168
 nameHash == _nameHashes[i]
REF_5177(bytes32) -> _nameHashes_1[i_2]
TMP_8169(bool) = nameHash_1 == REF_5177
CONDITION TMP_8169
 a = _addresses[i]
REF_5178(address) -> _addresses_1[i_2]
a_2(address) := REF_5178(address)
 i ++
TMP_8170(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 require(bool,error)(a != address(0),revert AUAddressZero()())
TMP_8171 = CONVERT 0 to address
TMP_8172(bool) = a_3 != TMP_8171
TMP_8173(None) = SOLIDITY_CALL revert AUAddressZero()()
TMP_8174(None) = SOLIDITY_CALL require(bool,error)(TMP_8172,TMP_8173)
 a
RETURN a_3
```
#### EnumerableSet.add(EnumerableSet.UintSet,uint256) [INTERNAL]
```slithir
 _add(set._inner,bytes32(value))
REF_232(EnumerableSet.Set) -> set_1 (-> [])._inner
TMP_940 = CONVERT value_1 to bytes32
TMP_941(bool) = INTERNAL_CALL, EnumerableSet._add(EnumerableSet.Set,bytes32)(REF_232,TMP_940)
RETURN TMP_941
```
#### EnumerableSet.values(EnumerableSet.AddressSet) [INTERNAL]
```slithir
 store = _values(set._inner)
REF_231(EnumerableSet.Set) -> set_1 (-> [])._inner
TMP_939(bytes32[]) = INTERNAL_CALL, EnumerableSet._values(EnumerableSet.Set)(REF_231)
store_1(bytes32[]) = ['TMP_939(bytes32[])']
 result = store
result_1(address[]) := store_1(bytes32[])
 result
RETURN result_1
```
#### EnumerableSet.at(EnumerableSet.UintSet,uint256) [INTERNAL]
```slithir
 uint256(_at(set._inner,index))
REF_236(EnumerableSet.Set) -> set_1 (-> [])._inner
TMP_947(bytes32) = INTERNAL_CALL, EnumerableSet._at(EnumerableSet.Set,uint256)(REF_236,index_1)
TMP_948 = CONVERT TMP_947 to uint256
RETURN TMP_948
```
#### EnumerableSet.length(EnumerableSet.UintSet) [INTERNAL]
```slithir
 _length(set._inner)
REF_235(EnumerableSet.Set) -> set_1 (-> [])._inner
TMP_946(uint256) = INTERNAL_CALL, EnumerableSet._length(EnumerableSet.Set)(REF_235)
RETURN TMP_946
```
#### GovernedBase.initialise(IGovernanceSettings,address) [INTERNAL]
```slithir
 state = _governedState()
TMP_9513(GovernedBase.GovernedState) = INTERNAL_CALL, GovernedBase._governedState()()
state_1 (-> ['TMP_9513'])(GovernedBase.GovernedState) := TMP_9513(GovernedBase.GovernedState)
 require(bool,error)(state.initialised == false,revert GovernedAlreadyInitialized()())
REF_5847(bool) -> state_1 (-> ['TMP_9513']).initialised
TMP_9514(bool) = REF_5847 == False
TMP_9515(None) = SOLIDITY_CALL revert GovernedAlreadyInitialized()()
TMP_9516(None) = SOLIDITY_CALL require(bool,error)(TMP_9514,TMP_9515)
 require(bool,error)(address(_governanceSettings) != address(0),revert GovernedAddressZero()())
TMP_9517 = CONVERT _governanceSettings_1 to address
TMP_9518 = CONVERT 0 to address
TMP_9519(bool) = TMP_9517 != TMP_9518
TMP_9520(None) = SOLIDITY_CALL revert GovernedAddressZero()()
TMP_9521(None) = SOLIDITY_CALL require(bool,error)(TMP_9519,TMP_9520)
 require(bool,error)(_initialGovernance != address(0),revert GovernedAddressZero()())
TMP_9522 = CONVERT 0 to address
TMP_9523(bool) = _initialGovernance_1 != TMP_9522
TMP_9524(None) = SOLIDITY_CALL revert GovernedAddressZero()()
TMP_9525(None) = SOLIDITY_CALL require(bool,error)(TMP_9523,TMP_9524)
 state.initialised = true
REF_5848(bool) -> state_1 (-> ['TMP_9513']).initialised
state_2 (-> ['TMP_9513'])(GovernedBase.GovernedState) := phi(["state_1 (-> ['TMP_9513'])"])
REF_5848(bool) (->state_2 (-> ['TMP_9513'])) := True(bool)
TMP_9513(GovernedBase.GovernedState) := phi(["state_2 (-> ['TMP_9513'])"])
 state.governanceSettings = _governanceSettings
REF_5849(IGovernanceSettings) -> state_2 (-> ['TMP_9513']).governanceSettings
state_3 (-> ['TMP_9513'])(GovernedBase.GovernedState) := phi(["state_2 (-> ['TMP_9513'])"])
REF_5849(IGovernanceSettings) (->state_3 (-> ['TMP_9513'])) := _governanceSettings_1(IGovernanceSettings)
TMP_9513(GovernedBase.GovernedState) := phi(["state_3 (-> ['TMP_9513'])"])
 state.initialGovernance = _initialGovernance
REF_5850(address) -> state_3 (-> ['TMP_9513']).initialGovernance
state_4 (-> ['TMP_9513'])(GovernedBase.GovernedState) := phi(["state_3 (-> ['TMP_9513'])"])
REF_5850(address) (->state_4 (-> ['TMP_9513'])) := _initialGovernance_1(address)
TMP_9513(GovernedBase.GovernedState) := phi(["state_4 (-> ['TMP_9513'])"])
 GovernanceInitialised(_initialGovernance)
Emit GovernanceInitialised(_initialGovernance_1)
```
#### AddressUpdatable.setAddressUpdaterValue(address) [INTERNAL]
```slithir
_addressUpdater_1(address) := phi(['TMP_8161', '_addressUpdater_1'])
ADDRESS_STORAGE_POSITION_2(bytes32) := phi(['ADDRESS_STORAGE_POSITION_0'])
 position = ADDRESS_STORAGE_POSITION
position_1(bytes32) := ADDRESS_STORAGE_POSITION_2(bytes32)
 sstore(uint256,uint256)(position,_addressUpdater)
TMP_8175(None) = SOLIDITY_CALL sstore(uint256,uint256)(position_1,_addressUpdater_1)
```
#### GovernedBase.governance() [PUBLIC]
```slithir
 state = _governedState()
TMP_9529(GovernedBase.GovernedState) = INTERNAL_CALL, GovernedBase._governedState()()
state_1 (-> ['TMP_9529'])(GovernedBase.GovernedState) := TMP_9529(GovernedBase.GovernedState)
 state.productionMode
REF_5853(bool) -> state_1 (-> ['TMP_9529']).productionMode
CONDITION REF_5853
 state.governanceSettings.getGovernanceAddress()
REF_5854(IGovernanceSettings) -> state_1 (-> ['TMP_9529']).governanceSettings
TMP_9530(address) = HIGH_LEVEL_CALL, dest:REF_5854(IGovernanceSettings), function:getGovernanceAddress, arguments:[]  
RETURN TMP_9530
 state.initialGovernance
REF_5856(address) -> state_1 (-> ['TMP_9529']).initialGovernance
RETURN REF_5856
```
#### EnumerableSet.contains(EnumerableSet.UintSet,uint256) [INTERNAL]
```slithir
 _contains(set._inner,bytes32(value))
REF_234(EnumerableSet.Set) -> set_1 (-> [])._inner
TMP_944 = CONVERT value_1 to bytes32
TMP_945(bool) = INTERNAL_CALL, EnumerableSet._contains(EnumerableSet.Set,bytes32)(REF_234,TMP_944)
RETURN TMP_945
```
#### EnumerableSet.remove(EnumerableSet.UintSet,uint256) [INTERNAL]
```slithir
 _remove(set._inner,bytes32(value))
REF_233(EnumerableSet.Set) -> set_1 (-> [])._inner
TMP_942 = CONVERT value_1 to bytes32
TMP_943(bool) = INTERNAL_CALL, EnumerableSet._remove(EnumerableSet.Set,bytes32)(REF_233,TMP_942)
RETURN TMP_943
```
#### EnumerableSet._add(EnumerableSet.Set,bytes32) [PRIVATE]
```slithir
set_1 (-> [])(EnumerableSet.Set) := phi(['REF_226', 'REF_220', 'REF_232'])
value_1(bytes32) := phi(['TMP_940', 'value_1', 'TMP_924'])
 ! _contains(set,value)
TMP_904(bool) = INTERNAL_CALL, EnumerableSet._contains(EnumerableSet.Set,bytes32)(set_1 (-> []),value_1)
TMP_905 = UnaryType.BANG TMP_904 
CONDITION TMP_905
 set._values.push(value)
REF_188(bytes32[]) -> set_1 (-> [])._values
REF_190 -> LENGTH REF_188
TMP_907(uint256) := REF_190(uint256)
TMP_908(uint256) = TMP_907 (c)+ 1
set_2 (-> [])(EnumerableSet.Set) := phi(['set_1 (-> [])'])
REF_190(uint256) (->set_3 (-> [])) := TMP_908(uint256)
REF_191(bytes32) -> REF_188[TMP_907]
set_3 (-> [])(EnumerableSet.Set) := phi(['set_2 (-> [])'])
REF_191(bytes32) (->set_3 (-> [])) := value_1(bytes32)
 set._indexes[value] = set._values.length
REF_192(mapping(bytes32 => uint256)) -> set_3 (-> [])._indexes
REF_193(uint256) -> REF_192[value_1]
REF_194(bytes32[]) -> set_3 (-> [])._values
REF_195 -> LENGTH REF_194
set_4 (-> [])(EnumerableSet.Set) := phi(['set_3 (-> [])'])
REF_193(uint256) (->set_4 (-> [])) := REF_195(uint256)
 true
RETURN True
 false
RETURN False
```
#### EnumerableSet._values(EnumerableSet.Set) [PRIVATE]
```slithir
set_1 (-> [])(EnumerableSet.Set) := phi(['REF_225', 'REF_231', 'REF_237'])
 set._values
REF_219(bytes32[]) -> set_1 (-> [])._values
RETURN REF_219
```
#### EnumerableSet._at(EnumerableSet.Set,uint256) [PRIVATE]
```slithir
set_1 (-> [])(EnumerableSet.Set) := phi(['REF_224', 'REF_230', 'REF_236'])
index_1(uint256) := phi(['index_1', 'index_1', 'index_1'])
 set._values[index]
REF_217(bytes32[]) -> set_1 (-> [])._values
REF_218(bytes32) -> REF_217[index_1]
RETURN REF_218
```
#### EnumerableSet._length(EnumerableSet.Set) [PRIVATE]
```slithir
set_1 (-> [])(EnumerableSet.Set) := phi(['REF_235', 'REF_229', 'REF_223'])
 set._values.length
REF_215(bytes32[]) -> set_1 (-> [])._values
REF_216 -> LENGTH REF_215
RETURN REF_216
```
#### GovernedBase._governedState() [PRIVATE]
```slithir
 position = keccak256(bytes)(fasset.GovernedBase.GovernedState)
TMP_9553(bytes32) = SOLIDITY_CALL keccak256(bytes)(fasset.GovernedBase.GovernedState)
position_1(bytes32) := TMP_9553(bytes32)
 _state = position
_state_1 (-> ['position'])(GovernedBase.GovernedState) := position_1(bytes32)
 _state
RETURN _state_1 (-> ['position'])
```
#### GovernanceSettingsMock.getGovernanceAddress() [EXTERNAL]
```slithir
governanceAddress_5(address) := phi(['governanceAddress_1', 'governanceAddress_0', 'governanceAddress_3'])
 governanceAddress
RETURN governanceAddress_5
```
#### EnumerableSet._contains(EnumerableSet.Set,bytes32) [PRIVATE]
```slithir
set_1 (-> [])(EnumerableSet.Set) := phi(['REF_234', 'set_1 (-> [])', 'REF_228', 'REF_222'])
value_1(bytes32) := phi(['value_1', 'value_1', 'TMP_944', 'TMP_932'])
 set._indexes[value] != 0
REF_213(mapping(bytes32 => uint256)) -> set_1 (-> [])._indexes
REF_214(uint256) -> REF_213[value_1]
TMP_915(bool) = REF_214 != 0
RETURN TMP_915
```
#### EnumerableSet._remove(EnumerableSet.Set,bytes32) [PRIVATE]
```slithir
set_1 (-> [])(EnumerableSet.Set) := phi(['REF_221', 'REF_233', 'REF_227'])
value_1(bytes32) := phi(['TMP_942', 'value_1', 'TMP_928'])
 valueIndex = set._indexes[value]
REF_196(mapping(bytes32 => uint256)) -> set_1 (-> [])._indexes
REF_197(uint256) -> REF_196[value_1]
valueIndex_1(uint256) := REF_197(uint256)
 valueIndex != 0
TMP_909(bool) = valueIndex_1 != 0
CONDITION TMP_909
 toDeleteIndex = valueIndex - 1
TMP_910(uint256) = valueIndex_1 (c)- 1
toDeleteIndex_1(uint256) := TMP_910(uint256)
 lastIndex = set._values.length - 1
REF_198(bytes32[]) -> set_1 (-> [])._values
REF_199 -> LENGTH REF_198
TMP_911(uint256) = REF_199 (c)- 1
lastIndex_1(uint256) := TMP_911(uint256)
 lastIndex != toDeleteIndex
TMP_912(bool) = lastIndex_1 != toDeleteIndex_1
CONDITION TMP_912
 lastValue = set._values[lastIndex]
REF_200(bytes32[]) -> set_1 (-> [])._values
REF_201(bytes32) -> REF_200[lastIndex_1]
lastValue_1(bytes32) := REF_201(bytes32)
 set._values[toDeleteIndex] = lastValue
REF_202(bytes32[]) -> set_1 (-> [])._values
REF_203(bytes32) -> REF_202[toDeleteIndex_1]
set_2 (-> [])(EnumerableSet.Set) := phi(['set_1 (-> [])'])
REF_203(bytes32) (->set_2 (-> [])) := lastValue_1(bytes32)
 set._indexes[lastValue] = valueIndex
REF_204(mapping(bytes32 => uint256)) -> set_2 (-> [])._indexes
REF_205(uint256) -> REF_204[lastValue_1]
set_3 (-> [])(EnumerableSet.Set) := phi(['set_2 (-> [])'])
REF_205(uint256) (->set_3 (-> [])) := valueIndex_1(uint256)
set_4 (-> [])(EnumerableSet.Set) := phi(['set_1 (-> [])', 'set_3 (-> [])'])
 set._values.pop()
REF_206(bytes32[]) -> set_4 (-> [])._values
REF_208 -> LENGTH REF_206
TMP_914(uint256) = REF_208 (c)- 1
REF_209(bytes32) -> REF_206[TMP_914]
REF_206 = delete REF_209 
REF_210 -> LENGTH REF_206
set_5 (-> [])(EnumerableSet.Set) := phi(['set_4 (-> [])'])
REF_210(uint256) (->set_5 (-> [])) := TMP_914(uint256)
 delete set._indexes[value]
REF_211(mapping(bytes32 => uint256)) -> set_5 (-> [])._indexes
REF_212(uint256) -> REF_211[value_1]
REF_211 = delete REF_212 
 true
RETURN True
 false
RETURN False
```
