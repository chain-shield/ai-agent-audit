


### Storage layout (AgentInference) 

```text
inferenceCount mapping(uint256 => uint256)
token IERC20
agentNft IAgentNft

```

#### AgentInference.initialize(address,address,address) [EXTERNAL]
```slithir
DEFAULT_ADMIN_ROLE_1(bytes32) := phi(['DEFAULT_ADMIN_ROLE_0', 'DEFAULT_ADMIN_ROLE_6'])
ADMIN_ROLE_1(bytes32) := phi(['ADMIN_ROLE_0', 'ADMIN_ROLE_5'])
 __AccessControl_init()
INTERNAL_CALL, AccessControlUpgradeable.__AccessControl_init()()
 __ReentrancyGuard_init()
INTERNAL_CALL, ReentrancyGuardUpgradeable.__ReentrancyGuard_init()()
 _grantRole(ADMIN_ROLE,defaultAdmin_)
TMP_6467(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(ADMIN_ROLE_4,defaultAdmin__1)
 _grantRole(DEFAULT_ADMIN_ROLE,defaultAdmin_)
TMP_6468(bool) = INTERNAL_CALL, AccessControlUpgradeable._grantRole(bytes32,address)(DEFAULT_ADMIN_ROLE_5,defaultAdmin__1)
 token = IERC20(token_)
TMP_6469 = CONVERT token__1 to IERC20
token_1(IERC20) := TMP_6469(IERC20)
 agentNft = IAgentNft(agentNft_)
TMP_6470 = CONVERT agentNft__1 to IAgentNft
agentNft_1(IAgentNft) := TMP_6470(IAgentNft)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### AgentInference.prompt(bytes32,uint256[],uint256[],uint8[][]) [PUBLIC]
```slithir
inferenceCount_1(mapping(uint256 => uint256)) := phi(['inferenceCount_10', 'inferenceCount_0', 'inferenceCount_4'])
token_2(IERC20) := phi(['token_10', 'token_5', 'token_1', 'token_0'])
agentNft_2(IAgentNft) := phi(['agentNft_5', 'agentNft_0', 'agentNft_1', 'agentNft_10'])
 sender = _msgSender()
TMP_6472(address) = INTERNAL_CALL, ContextUpgradeable._msgSender()()
sender_1(address) := TMP_6472(address)
 total = 0
total_1(uint256) := 0(uint256)
 require(bool,string)(agentIds.length == amounts.length && agentIds.length == coreIds.length,Invalid input)
REF_2385 -> LENGTH agentIds_1
REF_2386 -> LENGTH amounts_1
TMP_6473(bool) = REF_2385 == REF_2386
REF_2387 -> LENGTH agentIds_1
REF_2388 -> LENGTH coreIds_1
TMP_6474(bool) = REF_2387 == REF_2388
TMP_6475(bool) = TMP_6473 && TMP_6474
TMP_6476(None) = SOLIDITY_CALL require(bool,string)(TMP_6475,Invalid input)
 i = 0
i_1(uint256) := 0(uint256)
 i < amounts.length
total_2(uint256) := phi(['total_1', 'total_3'])
i_2(uint256) := phi(['i_1', 'i_3'])
REF_2389 -> LENGTH amounts_1
TMP_6477(bool) = i_2 < REF_2389
CONDITION TMP_6477
 total += amounts[i]
REF_2390(uint256) -> amounts_1[i_2]
total_3(uint256) = total_2 (c)+ REF_2390
 i ++
TMP_6478(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 require(bool,string)(token.balanceOf(sender) >= total,Insufficient balance)
TMP_6479(uint256) = HIGH_LEVEL_CALL, dest:token_4(IERC20), function:balanceOf, arguments:['sender_1']  
inferenceCount_4(mapping(uint256 => uint256)) := phi(['inferenceCount_10', 'inferenceCount_3', 'inferenceCount_4'])
token_5(IERC20) := phi(['token_10', 'token_4', 'token_5', 'token_1'])
agentNft_5(IAgentNft) := phi(['agentNft_4', 'agentNft_5', 'agentNft_1', 'agentNft_10'])
TMP_6480(bool) = TMP_6479 >= total_2
TMP_6481(None) = SOLIDITY_CALL require(bool,string)(TMP_6480,Insufficient balance)
 i_scope_0 = 0
i_scope_0_1(uint256) := 0(uint256)
 i_scope_0 < agentIds.length
i_scope_0_2(uint256) := phi(['i_scope_0_1', 'i_scope_0_3'])
REF_2392 -> LENGTH agentIds_1
TMP_6482(bool) = i_scope_0_2 < REF_2392
CONDITION TMP_6482
 agentId = agentIds[i_scope_0]
REF_2393(uint256) -> agentIds_1[i_scope_0_2]
agentId_1(uint256) := REF_2393(uint256)
 agentTba = agentNft.virtualInfo(agentId).tba
TMP_6483(IAgentNft.VirtualInfo) = HIGH_LEVEL_CALL, dest:agentNft_5(IAgentNft), function:virtualInfo, arguments:['agentId_1']  
inferenceCount_5(mapping(uint256 => uint256)) := phi(['inferenceCount_10', 'inferenceCount_4'])
token_6(IERC20) := phi(['token_10', 'token_5', 'token_1'])
agentNft_6(IAgentNft) := phi(['agentNft_5', 'agentNft_1', 'agentNft_10'])
REF_2395(address) -> TMP_6483.tba
agentTba_1(address) := REF_2395(address)
 token.safeTransferFrom(sender,agentTba,amounts[i_scope_0])
REF_2397(uint256) -> amounts_1[i_scope_0_2]
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransferFrom(IERC20,address,address,uint256), arguments:['token_6', 'sender_1', 'agentTba_1', 'REF_2397'] 
 inferenceCount[agentId] ++
REF_2398(uint256) -> inferenceCount_5[agentId_1]
TMP_6485(uint256) := REF_2398(uint256)
inferenceCount_6(mapping(uint256 => uint256)) := phi(['inferenceCount_5'])
REF_2398(-> inferenceCount_6) = REF_2398 (c)+ 1
 Prompt(sender,promptHash,agentId,amounts[i_scope_0],coreIds[i_scope_0])
REF_2399(uint256) -> amounts_1[i_scope_0_2]
REF_2400(uint8[]) -> coreIds_1[i_scope_0_2]
Emit Prompt(sender_1,promptHash_1,agentId_1,REF_2399,REF_2400)
 i_scope_0 ++
TMP_6487(uint256) := i_scope_0_2(uint256)
i_scope_0_3(uint256) = i_scope_0_2 (c)+ 1
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardUpgradeable.nonReentrant()()
```
#### AgentInference.promptMulti(bytes32[],uint256[],uint256[],uint8[][]) [PUBLIC]
```slithir
inferenceCount_7(mapping(uint256 => uint256)) := phi(['inferenceCount_10', 'inferenceCount_0', 'inferenceCount_4'])
token_7(IERC20) := phi(['token_10', 'token_5', 'token_1', 'token_0'])
agentNft_7(IAgentNft) := phi(['agentNft_5', 'agentNft_0', 'agentNft_1', 'agentNft_10'])
 sender = _msgSender()
TMP_6489(address) = INTERNAL_CALL, ContextUpgradeable._msgSender()()
sender_1(address) := TMP_6489(address)
 total = 0
total_1(uint256) := 0(uint256)
 len = agentIds.length
REF_2401 -> LENGTH agentIds_1
len_1(uint256) := REF_2401(uint256)
 require(bool,string)(len == amounts.length && len == coreIds.length && len == promptHashes.length,Invalid input)
REF_2402 -> LENGTH amounts_1
TMP_6490(bool) = len_1 == REF_2402
REF_2403 -> LENGTH coreIds_1
TMP_6491(bool) = len_1 == REF_2403
TMP_6492(bool) = TMP_6490 && TMP_6491
REF_2404 -> LENGTH promptHashes_1
TMP_6493(bool) = len_1 == REF_2404
TMP_6494(bool) = TMP_6492 && TMP_6493
TMP_6495(None) = SOLIDITY_CALL require(bool,string)(TMP_6494,Invalid input)
 i = 0
i_1(uint256) := 0(uint256)
 i < len
total_2(uint256) := phi(['total_3', 'total_1'])
i_2(uint256) := phi(['i_1', 'i_3'])
TMP_6496(bool) = i_2 < len_1
CONDITION TMP_6496
 total += amounts[i]
REF_2405(uint256) -> amounts_1[i_2]
total_3(uint256) = total_2 (c)+ REF_2405
 i ++
TMP_6497(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 require(bool,string)(token.balanceOf(sender) >= total,Insufficient balance)
TMP_6498(uint256) = HIGH_LEVEL_CALL, dest:token_9(IERC20), function:balanceOf, arguments:['sender_1']  
inferenceCount_10(mapping(uint256 => uint256)) := phi(['inferenceCount_10', 'inferenceCount_9', 'inferenceCount_4'])
token_10(IERC20) := phi(['token_10', 'token_9', 'token_5', 'token_1'])
agentNft_10(IAgentNft) := phi(['agentNft_9', 'agentNft_5', 'agentNft_1', 'agentNft_10'])
TMP_6499(bool) = TMP_6498 >= total_2
TMP_6500(None) = SOLIDITY_CALL require(bool,string)(TMP_6499,Insufficient balance)
 prevAgentId = 0
prevAgentId_1(uint256) := 0(uint256)
 agentTba = address(0)
TMP_6501 = CONVERT 0 to address
agentTba_1(address) := TMP_6501(address)
 i_scope_0 = 0
i_scope_0_1(uint256) := 0(uint256)
 i_scope_0 < len
i_scope_0_2(uint256) := phi(['i_scope_0_1', 'i_scope_0_3'])
TMP_6502(bool) = i_scope_0_2 < len_1
CONDITION TMP_6502
 agentId = agentIds[i_scope_0]
REF_2407(uint256) -> agentIds_1[i_scope_0_2]
agentId_1(uint256) := REF_2407(uint256)
 prevAgentId != agentId
TMP_6503(bool) = prevAgentId_1 != agentId_1
CONDITION TMP_6503
 agentTba = agentNft.virtualInfo(agentId).tba
TMP_6504(IAgentNft.VirtualInfo) = HIGH_LEVEL_CALL, dest:agentNft_10(IAgentNft), function:virtualInfo, arguments:['agentId_1']  
inferenceCount_11(mapping(uint256 => uint256)) := phi(['inferenceCount_10', 'inferenceCount_4'])
token_11(IERC20) := phi(['token_10', 'token_5', 'token_1'])
agentNft_11(IAgentNft) := phi(['agentNft_5', 'agentNft_1', 'agentNft_10'])
REF_2409(address) -> TMP_6504.tba
agentTba_2(address) := REF_2409(address)
agentTba_3(address) := phi(['agentTba_1', 'agentTba_2'])
 token.safeTransferFrom(sender,agentTba,amounts[i_scope_0])
REF_2411(uint256) -> amounts_1[i_scope_0_2]
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransferFrom(IERC20,address,address,uint256), arguments:['token_11', 'sender_1', 'agentTba_3', 'REF_2411'] 
 inferenceCount[agentId] ++
REF_2412(uint256) -> inferenceCount_11[agentId_1]
TMP_6506(uint256) := REF_2412(uint256)
inferenceCount_12(mapping(uint256 => uint256)) := phi(['inferenceCount_11'])
REF_2412(-> inferenceCount_12) = REF_2412 (c)+ 1
 Prompt(sender,promptHashes[i_scope_0],agentId,amounts[i_scope_0],coreIds[i_scope_0])
REF_2413(bytes32) -> promptHashes_1[i_scope_0_2]
REF_2414(uint256) -> amounts_1[i_scope_0_2]
REF_2415(uint8[]) -> coreIds_1[i_scope_0_2]
Emit Prompt(sender_1,REF_2413,agentId_1,REF_2414,REF_2415)
 i_scope_0 ++
TMP_6508(uint256) := i_scope_0_2(uint256)
i_scope_0_3(uint256) = i_scope_0_2 (c)+ 1
 nonReentrant()
MODIFIER_CALL, ReentrancyGuardUpgradeable.nonReentrant()()
```

#### IERC20.balanceOf(address) [EXTERNAL]
```slithir

```
#### SafeERC20.safeTransferFrom(IERC20,address,address,uint256) [INTERNAL]
```slithir
 _callOptionalReturn(token,abi.encodeCall(token.transferFrom,(from,to,value)))
REF_2046(transferFrom) -> token_1.transferFrom
TMP_5415(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_2046,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78000160>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78001150>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78001c90>])
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_5415)
```
#### IAgentNft.virtualInfo(uint256) [EXTERNAL]
```slithir

```
#### SafeERC20._callOptionalReturn(IERC20,bytes) [PRIVATE]
```slithir
token_1(IERC20) := phi(['token_1', 'token_1', 'token_1'])
data_1(bytes) := phi(['approvalCall_1', 'TMP_5413', 'TMP_5415', 'TMP_5430'])
 returndata = address(token).functionCall(data)
TMP_5433 = CONVERT token_1 to address
TMP_5434(bytes) = LIBRARY_CALL, dest:Address, function:Address.functionCall(address,bytes), arguments:['TMP_5433', 'data_1'] 
returndata_1(bytes) := TMP_5434(bytes)
 returndata.length != 0 && ! abi.decode(returndata,(bool))
REF_2054 -> LENGTH returndata_1
TMP_5435(bool) = REF_2054 != 0
TMP_5436(bool) = SOLIDITY_CALL abi.decode()(returndata_1,bool)
TMP_5437 = UnaryType.BANG TMP_5436 
TMP_5438(bool) = TMP_5435 && TMP_5437
CONDITION TMP_5438
 revert SafeERC20FailedOperation(address)(address(token))
TMP_5439 = CONVERT token_1 to address
TMP_5440(None) = SOLIDITY_CALL revert SafeERC20FailedOperation(address)(TMP_5439)
```
#### Address.functionCall(address,bytes) [INTERNAL]
```slithir
 functionCallWithValue(target,data,0)
TMP_5457(bytes) = INTERNAL_CALL, Address.functionCallWithValue(address,bytes,uint256)(target_1,data_1,0)
RETURN TMP_5457
```
