

### Storage layout (AgentToken) 

```text
uniswapV2Pair address
botProtectionDurationInSeconds uint256
_tokenHasTax bool
_uniswapRouter IUniswapV2Router02
fundedDate uint32
projectBuyTaxBasisPoints uint16
projectSellTaxBasisPoints uint16
swapThresholdBasisPoints uint16
pairToken address
_autoSwapInProgress bool
projectTaxRecipient address
projectTaxPendingSwap uint128
vault address
_name string
_symbol string
_totalSupply uint256
_balances mapping(address => uint256)
_allowances mapping(address => mapping(address => uint256))
_validCallerCodeHashes EnumerableSet.Bytes32Set
_liquidityPools EnumerableSet.AddressSet
_factory IAgentFactory

```




#### AgentToken._addInitialLiquidity(address) [INTERNAL]
```slithir
lpOwner_1(address) := phi(['lpOwner_1'])
uniswapV2Pair_2(address) := phi(['uniswapV2Pair_9', 'uniswapV2Pair_1', 'uniswapV2Pair_0'])
_uniswapRouter_7(IUniswapV2Router02) := phi(['_uniswapRouter_17', '_uniswapRouter_15', '_uniswapRouter_0', '_uniswapRouter_6', '_uniswapRouter_4', '_uniswapRouter_1', '_uniswapRouter_13'])
fundedDate_1(uint32) := phi(['fundedDate_0', 'fundedDate_2'])
pairToken_7(address) := phi(['pairToken_13', 'pairToken_1', 'pairToken_4', 'pairToken_6', 'pairToken_0'])
 fundedDate != 0
TMP_13409(bool) = fundedDate_1 != 0
CONDITION TMP_13409
 revert InitialLiquidityAlreadyAdded()()
TMP_13410(None) = SOLIDITY_CALL revert InitialLiquidityAlreadyAdded()()
 fundedDate = uint32(block.timestamp)
TMP_13411 = CONVERT block.timestamp to uint32
fundedDate_2(uint32) := TMP_13411(uint32)
 balanceOf(address(this)) == 0
TMP_13412 = CONVERT this to address
TMP_13413(uint256) = INTERNAL_CALL, AgentToken.balanceOf(address)(TMP_13412)
TMP_13414(bool) = TMP_13413 == 0
CONDITION TMP_13414
 revert NoTokenForLiquidityPair()()
TMP_13415(None) = SOLIDITY_CALL revert NoTokenForLiquidityPair()()
 _approve(address(this),address(_uniswapRouter),type()(uint256).max)
TMP_13416 = CONVERT this to address
TMP_13417 = CONVERT _uniswapRouter_8 to address
TMP_13419(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
INTERNAL_CALL, AgentToken._approve(address,address,uint256)(TMP_13416,TMP_13417,TMP_13419)
 IERC20(pairToken).approve(address(_uniswapRouter),type()(uint256).max)
TMP_13421 = CONVERT pairToken_9 to IERC20
TMP_13422 = CONVERT _uniswapRouter_9 to address
TMP_13424(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
TMP_13425(bool) = HIGH_LEVEL_CALL, dest:TMP_13421(IERC20), function:approve, arguments:['TMP_13422', 'TMP_13424']  
uniswapV2Pair_5(address) := phi(['uniswapV2Pair_9', 'uniswapV2Pair_1', 'uniswapV2Pair_4'])
_uniswapRouter_10(IUniswapV2Router02) := phi(['_uniswapRouter_17', '_uniswapRouter_15', '_uniswapRouter_6', '_uniswapRouter_4', '_uniswapRouter_1', '_uniswapRouter_9', '_uniswapRouter_13'])
pairToken_10(address) := phi(['pairToken_13', 'pairToken_1', 'pairToken_4', 'pairToken_6', 'pairToken_9'])
 (amountA,amountB,lpTokens) = _uniswapRouter.addLiquidity(address(this),pairToken,balanceOf(address(this)),IERC20(pairToken).balanceOf(address(this)),0,0,address(this),block.timestamp)
TMP_13426 = CONVERT this to address
TMP_13427 = CONVERT this to address
TMP_13428(uint256) = INTERNAL_CALL, AgentToken.balanceOf(address)(TMP_13427)
TMP_13429 = CONVERT pairToken_11 to IERC20
TMP_13430 = CONVERT this to address
TMP_13431(uint256) = HIGH_LEVEL_CALL, dest:TMP_13429(IERC20), function:balanceOf, arguments:['TMP_13430']  
uniswapV2Pair_7(address) := phi(['uniswapV2Pair_9', 'uniswapV2Pair_1', 'uniswapV2Pair_6'])
_uniswapRouter_12(IUniswapV2Router02) := phi(['_uniswapRouter_17', '_uniswapRouter_15', '_uniswapRouter_6', '_uniswapRouter_4', '_uniswapRouter_1', '_uniswapRouter_11', '_uniswapRouter_13'])
pairToken_12(address) := phi(['pairToken_13', 'pairToken_1', 'pairToken_11', 'pairToken_4', 'pairToken_6'])
TMP_13432 = CONVERT this to address
TUPLE_127(uint256,uint256,uint256) = HIGH_LEVEL_CALL, dest:_uniswapRouter_12(IUniswapV2Router02), function:addLiquidity, arguments:['TMP_13426', 'pairToken_12', 'TMP_13428', 'TMP_13431', '0', '0', 'TMP_13432', 'block.timestamp']  
uniswapV2Pair_8(address) := phi(['uniswapV2Pair_9', 'uniswapV2Pair_1', 'uniswapV2Pair_7'])
_uniswapRouter_13(IUniswapV2Router02) := phi(['_uniswapRouter_17', '_uniswapRouter_15', '_uniswapRouter_6', '_uniswapRouter_4', '_uniswapRouter_1', '_uniswapRouter_12', '_uniswapRouter_13'])
pairToken_13(address) := phi(['pairToken_13', 'pairToken_12', 'pairToken_1', 'pairToken_4', 'pairToken_6'])
amountA_1(uint256)= UNPACK TUPLE_127 index: 0 
amountB_1(uint256)= UNPACK TUPLE_127 index: 1 
lpTokens_1(uint256)= UNPACK TUPLE_127 index: 2 
 InitialLiquidityAdded(amountA,amountB,lpTokens)
Emit InitialLiquidityAdded(amountA_1,amountB_1,lpTokens_1)
 _autoSwapInProgress = false
_autoSwapInProgress_2(bool) := False(bool)
 IERC20(uniswapV2Pair).transfer(lpOwner,lpTokens)
TMP_13434 = CONVERT uniswapV2Pair_8 to IERC20
TMP_13435(bool) = HIGH_LEVEL_CALL, dest:TMP_13434(IERC20), function:transfer, arguments:['lpOwner_1', 'lpTokens_1']  
uniswapV2Pair_9(address) := phi(['uniswapV2Pair_9', 'uniswapV2Pair_1', 'uniswapV2Pair_8'])
```
#### AgentToken._afterTokenTransfer(address,address,uint256) [INTERNAL]
```slithir
from_1(address) := phi(['account_1', 'TMP_13587', 'from_1'])
to_1(address) := phi(['TMP_13600', 'to_1', 'account_1'])
amount_1(uint256) := phi(['amount_1', 'amount_1', 'amount_1'])
```
#### AgentToken._approve(address,address,uint256) [INTERNAL]
```slithir
owner_1(address) := phi(['owner_1', 'owner_1', 'owner_1', 'owner_1', 'TMP_13416'])
spender_1(address) := phi(['TMP_13417', 'spender_1', 'spender_1', 'spender_1', 'spender_1'])
amount_1(uint256) := phi(['TMP_13615', 'TMP_13419', 'amount_1', 'TMP_13481', 'TMP_13487'])
 owner == address(0)
TMP_13602 = CONVERT 0 to address
TMP_13603(bool) = owner_1 == TMP_13602
CONDITION TMP_13603
 revert ApproveFromTheZeroAddress()()
TMP_13604(None) = SOLIDITY_CALL revert ApproveFromTheZeroAddress()()
 spender == address(0)
TMP_13605 = CONVERT 0 to address
TMP_13606(bool) = spender_1 == TMP_13605
CONDITION TMP_13606
 revert ApproveToTheZeroAddress()()
TMP_13607(None) = SOLIDITY_CALL revert ApproveToTheZeroAddress()()
 _allowances[owner][spender] = amount
REF_5561(mapping(address => uint256)) -> _allowances_1[owner_1]
REF_5562(uint256) -> REF_5561[spender_1]
_allowances_2(mapping(address => mapping(address => uint256))) := phi(['_allowances_1'])
REF_5562(uint256) (->_allowances_2) := amount_1(uint256)
 Approval(owner,spender,amount)
Emit Approval(owner_1,spender_1,amount_1)
```
#### AgentToken._autoSwap(address,address) [INTERNAL]
```slithir
from__1(address) := phi(['from_1'])
to__1(address) := phi(['to_1'])
BP_DENOM_6(uint256) := phi(['BP_DENOM_5', 'BP_DENOM_0', 'BP_DENOM_3', 'BP_DENOM_7'])
MAX_SWAP_THRESHOLD_MULTIPLE_1(uint256) := phi(['MAX_SWAP_THRESHOLD_MULTIPLE_3', 'MAX_SWAP_THRESHOLD_MULTIPLE_0'])
_tokenHasTax_3(bool) := phi(['_tokenHasTax_1', '_tokenHasTax_0'])
swapThresholdBasisPoints_5(uint16) := phi(['swapThresholdBasisPoints_6', 'swapThresholdBasisPoints_0', 'swapThresholdBasisPoints_4', 'swapThresholdBasisPoints_1'])
_totalSupply_2(uint256) := phi(['_totalSupply_6', '_totalSupply_0', '_totalSupply_9', '_totalSupply_3'])
 _tokenHasTax
CONDITION _tokenHasTax_3
 contractBalance = balanceOf(address(this))
TMP_13534 = CONVERT this to address
TMP_13535(uint256) = INTERNAL_CALL, AgentToken.balanceOf(address)(TMP_13534)
contractBalance_1(uint256) := TMP_13535(uint256)
 swapBalance = contractBalance
swapBalance_1(uint256) := contractBalance_1(uint256)
 swapThresholdInTokens = (_totalSupply * swapThresholdBasisPoints) / BP_DENOM
TMP_13536(uint256) = _totalSupply_3 (c)* swapThresholdBasisPoints_6
TMP_13537(uint256) = TMP_13536 (c)/ BP_DENOM_7
swapThresholdInTokens_1(uint256) := TMP_13537(uint256)
 _eligibleForSwap(from_,to_,swapBalance,swapThresholdInTokens)
TMP_13538(bool) = INTERNAL_CALL, AgentToken._eligibleForSwap(address,address,uint256,uint256)(from__1,to__1,swapBalance_1,swapThresholdInTokens_1)
CONDITION TMP_13538
 _autoSwapInProgress = true
_autoSwapInProgress_4(bool) := True(bool)
 swapBalance > swapThresholdInTokens * MAX_SWAP_THRESHOLD_MULTIPLE
TMP_13539(uint256) = swapThresholdInTokens_1 (c)* MAX_SWAP_THRESHOLD_MULTIPLE_3
TMP_13540(bool) = swapBalance_1 > TMP_13539
CONDITION TMP_13540
 swapBalance = swapThresholdInTokens * MAX_SWAP_THRESHOLD_MULTIPLE
TMP_13541(uint256) = swapThresholdInTokens_1 (c)* MAX_SWAP_THRESHOLD_MULTIPLE_3
swapBalance_2(uint256) := TMP_13541(uint256)
swapBalance_3(uint256) := phi(['swapBalance_2', 'swapBalance_1'])
 _swapTax(swapBalance,contractBalance)
INTERNAL_CALL, AgentToken._swapTax(uint256,uint256)(swapBalance_3,contractBalance_1)
 _autoSwapInProgress = false
_autoSwapInProgress_5(bool) := False(bool)
_autoSwapInProgress_6(bool) := phi(['_autoSwapInProgress_5', '_autoSwapInProgress_4'])
```
#### AgentToken._beforeTokenTransfer(address,address,uint256) [INTERNAL]
```slithir
from_1(address) := phi(['account_1', 'TMP_13582', 'from_1'])
to_1(address) := phi(['TMP_13592', 'to_1', 'account_1'])
amount_1(uint256) := phi(['amount_1', 'amount_1', 'amount_1'])
```
#### AgentToken._burn(address,uint256) [INTERNAL]
```slithir
account_1(address) := phi(['TMP_13617', 'account_1'])
amount_1(uint256) := phi(['value_1', 'value_1'])
_totalSupply_7(uint256) := phi(['_totalSupply_6', '_totalSupply_0', '_totalSupply_9', '_totalSupply_3'])
_balances_14(mapping(address => uint256)) := phi(['_balances_10', '_balances_4', '_balances_13', '_balances_16', '_balances_0', '_balances_1', '_balances_9', '_balances_3', '_balances_7'])
 account == address(0)
TMP_13589 = CONVERT 0 to address
TMP_13590(bool) = account_1 == TMP_13589
CONDITION TMP_13590
 revert BurnFromTheZeroAddress()()
TMP_13591(None) = SOLIDITY_CALL revert BurnFromTheZeroAddress()()
 _beforeTokenTransfer(account,address(0),amount)
TMP_13592 = CONVERT 0 to address
INTERNAL_CALL, AgentToken._beforeTokenTransfer(address,address,uint256)(account_1,TMP_13592,amount_1)
 accountBalance = _balances[account]
REF_5559(uint256) -> _balances_15[account_1]
accountBalance_1(uint256) := REF_5559(uint256)
 accountBalance < amount
TMP_13594(bool) = accountBalance_1 < amount_1
CONDITION TMP_13594
 revert BurnExceedsBalance()()
TMP_13595(None) = SOLIDITY_CALL revert BurnExceedsBalance()()
 _balances[account] = accountBalance - amount
REF_5560(uint256) -> _balances_15[account_1]
TMP_13596(uint256) = accountBalance_1 - amount_1
_balances_16(mapping(address => uint256)) := phi(['_balances_15'])
REF_5560(uint256) (->_balances_16) := TMP_13596(uint256)
 _totalSupply -= uint128(amount)
TMP_13597 = CONVERT amount_1 to uint128
_totalSupply_9(uint256) = _totalSupply_8 - TMP_13597
 Transfer(account,address(0),amount)
TMP_13598 = CONVERT 0 to address
Emit Transfer(account_1,TMP_13598,amount_1)
 _afterTokenTransfer(account,address(0),amount)
TMP_13600 = CONVERT 0 to address
INTERNAL_CALL, AgentToken._afterTokenTransfer(address,address,uint256)(account_1,TMP_13600,amount_1)
```
#### AgentToken._createPair() [INTERNAL]
```slithir
_uniswapRouter_2(IUniswapV2Router02) := phi(['_uniswapRouter_17', '_uniswapRouter_15', '_uniswapRouter_0', '_uniswapRouter_6', '_uniswapRouter_4', '_uniswapRouter_1', '_uniswapRouter_13'])
pairToken_2(address) := phi(['pairToken_13', 'pairToken_1', 'pairToken_4', 'pairToken_6', 'pairToken_0'])
_liquidityPools_1(EnumerableSet.AddressSet) := phi(['_liquidityPools_3', '_liquidityPools_5', '_liquidityPools_9', '_liquidityPools_11', '_liquidityPools_0'])
 uniswapV2Pair_ = IUniswapV2Factory(_uniswapRouter.factory()).getPair(address(this),pairToken)
TMP_13395(address) = HIGH_LEVEL_CALL, dest:_uniswapRouter_2(IUniswapV2Router02), function:factory, arguments:[]  
_uniswapRouter_3(IUniswapV2Router02) := phi(['_uniswapRouter_2', '_uniswapRouter_17', '_uniswapRouter_15', '_uniswapRouter_6', '_uniswapRouter_4', '_uniswapRouter_1', '_uniswapRouter_13'])
pairToken_3(address) := phi(['pairToken_13', 'pairToken_1', 'pairToken_2', 'pairToken_4', 'pairToken_6'])
_liquidityPools_2(EnumerableSet.AddressSet) := phi(['_liquidityPools_3', '_liquidityPools_5', '_liquidityPools_1', '_liquidityPools_9', '_liquidityPools_11'])
TMP_13396 = CONVERT TMP_13395 to IUniswapV2Factory
TMP_13397 = CONVERT this to address
TMP_13398(address) = HIGH_LEVEL_CALL, dest:TMP_13396(IUniswapV2Factory), function:getPair, arguments:['TMP_13397', 'pairToken_3']  
_uniswapRouter_4(IUniswapV2Router02) := phi(['_uniswapRouter_17', '_uniswapRouter_15', '_uniswapRouter_3', '_uniswapRouter_6', '_uniswapRouter_4', '_uniswapRouter_1', '_uniswapRouter_13'])
pairToken_4(address) := phi(['pairToken_13', 'pairToken_1', 'pairToken_3', 'pairToken_4', 'pairToken_6'])
_liquidityPools_3(EnumerableSet.AddressSet) := phi(['_liquidityPools_3', '_liquidityPools_5', '_liquidityPools_9', '_liquidityPools_11', '_liquidityPools_2'])
uniswapV2Pair__1(address) := TMP_13398(address)
 uniswapV2Pair_ == address(0)
TMP_13399 = CONVERT 0 to address
TMP_13400(bool) = uniswapV2Pair__1 == TMP_13399
CONDITION TMP_13400
 uniswapV2Pair_ = IUniswapV2Factory(_uniswapRouter.factory()).createPair(address(this),pairToken)
TMP_13401(address) = HIGH_LEVEL_CALL, dest:_uniswapRouter_4(IUniswapV2Router02), function:factory, arguments:[]  
_uniswapRouter_5(IUniswapV2Router02) := phi(['_uniswapRouter_17', '_uniswapRouter_15', '_uniswapRouter_6', '_uniswapRouter_4', '_uniswapRouter_1', '_uniswapRouter_13'])
pairToken_5(address) := phi(['pairToken_1', 'pairToken_13', 'pairToken_4', 'pairToken_6'])
_liquidityPools_4(EnumerableSet.AddressSet) := phi(['_liquidityPools_9', '_liquidityPools_3', '_liquidityPools_5', '_liquidityPools_11'])
TMP_13402 = CONVERT TMP_13401 to IUniswapV2Factory
TMP_13403 = CONVERT this to address
TMP_13404(address) = HIGH_LEVEL_CALL, dest:TMP_13402(IUniswapV2Factory), function:createPair, arguments:['TMP_13403', 'pairToken_5']  
_uniswapRouter_6(IUniswapV2Router02) := phi(['_uniswapRouter_17', '_uniswapRouter_15', '_uniswapRouter_5', '_uniswapRouter_6', '_uniswapRouter_4', '_uniswapRouter_1', '_uniswapRouter_13'])
pairToken_6(address) := phi(['pairToken_13', 'pairToken_1', 'pairToken_5', 'pairToken_4', 'pairToken_6'])
_liquidityPools_5(EnumerableSet.AddressSet) := phi(['_liquidityPools_3', '_liquidityPools_5', '_liquidityPools_9', '_liquidityPools_11', '_liquidityPools_4'])
uniswapV2Pair__2(address) := TMP_13404(address)
 LiquidityPoolCreated(uniswapV2Pair_)
Emit LiquidityPoolCreated(uniswapV2Pair__2)
uniswapV2Pair__3(address) := phi(['uniswapV2Pair__1', 'uniswapV2Pair__2'])
 _liquidityPools.add(uniswapV2Pair_)
TMP_13406(bool) = LIBRARY_CALL, dest:EnumerableSet, function:EnumerableSet.add(EnumerableSet.AddressSet,address), arguments:['_liquidityPools_5', 'uniswapV2Pair__3'] 
 (uniswapV2Pair_)
RETURN uniswapV2Pair__3
 uniswapV2Pair_
```
#### AgentToken._decodeBaseParams(address,bytes) [INTERNAL]
```slithir
projectOwner__1(address) := phi(['REF_5508'])
encodedBaseParams__1(bytes) := phi(['baseParams__1'])
 _transferOwnership(projectOwner_)
INTERNAL_CALL, Ownable2StepUpgradeable._transferOwnership(address)(projectOwner__1)
 (_name,_symbol) = abi.decode(encodedBaseParams_,(string,string))
TUPLE_126(string,string) = SOLIDITY_CALL abi.decode()(encodedBaseParams__1(string,string))
_name_1(string)= UNPACK TUPLE_126 index: 0 
_symbol_1(string)= UNPACK TUPLE_126 index: 1
```
#### AgentToken._eligibleForSwap(address,address,uint256,uint256) [INTERNAL]
```slithir
from__1(address) := phi(['from__1'])
to__1(address) := phi(['to__1'])
taxBalance__1(uint256) := phi(['swapBalance_1'])
swapThresholdInTokens__1(uint256) := phi(['swapThresholdInTokens_1'])
_uniswapRouter_14(IUniswapV2Router02) := phi(['_uniswapRouter_17', '_uniswapRouter_15', '_uniswapRouter_0', '_uniswapRouter_6', '_uniswapRouter_4', '_uniswapRouter_1', '_uniswapRouter_13'])
_autoSwapInProgress_7(bool) := phi(['_autoSwapInProgress_0', '_autoSwapInProgress_2', '_autoSwapInProgress_1', '_autoSwapInProgress_8', '_autoSwapInProgress_6'])
 (taxBalance_ >= swapThresholdInTokens_ && ! _autoSwapInProgress && ! isLiquidityPool(from_) && from_ != address(_uniswapRouter) && to_ != address(_uniswapRouter))
TMP_13543(bool) = taxBalance__1 >= swapThresholdInTokens__1
TMP_13544 = UnaryType.BANG _autoSwapInProgress_7 
TMP_13545(bool) = TMP_13543 && TMP_13544
TMP_13546(bool) = INTERNAL_CALL, AgentToken.isLiquidityPool(address)(from__1)
TMP_13547 = UnaryType.BANG TMP_13546 
TMP_13548(bool) = TMP_13545 && TMP_13547
TMP_13549 = CONVERT _uniswapRouter_15 to address
TMP_13550(bool) = from__1 != TMP_13549
TMP_13551(bool) = TMP_13548 && TMP_13550
TMP_13552 = CONVERT _uniswapRouter_15 to address
TMP_13553(bool) = to__1 != TMP_13552
TMP_13554(bool) = TMP_13551 && TMP_13553
RETURN TMP_13554
```
#### AgentToken._mint(address,uint256) [INTERNAL]
```slithir
account_1(address) := phi(['TMP_13391', 'vault_3'])
amount_1(uint256) := phi(['lpMint__1', 'vaultMint__1'])
_totalSupply_4(uint256) := phi(['_totalSupply_6', '_totalSupply_0', '_totalSupply_9', '_totalSupply_3'])
_balances_11(mapping(address => uint256)) := phi(['_balances_10', '_balances_4', '_balances_13', '_balances_16', '_balances_0', '_balances_1', '_balances_9', '_balances_3', '_balances_7'])
 account == address(0)
TMP_13579 = CONVERT 0 to address
TMP_13580(bool) = account_1 == TMP_13579
CONDITION TMP_13580
 revert MintToZeroAddress()()
TMP_13581(None) = SOLIDITY_CALL revert MintToZeroAddress()()
 _beforeTokenTransfer(address(0),account,amount)
TMP_13582 = CONVERT 0 to address
INTERNAL_CALL, AgentToken._beforeTokenTransfer(address,address,uint256)(TMP_13582,account_1,amount_1)
 _totalSupply += uint128(amount)
TMP_13584 = CONVERT amount_1 to uint128
_totalSupply_6(uint256) = _totalSupply_5 (c)+ TMP_13584
 _balances[account] += amount
REF_5558(uint256) -> _balances_12[account_1]
_balances_13(mapping(address => uint256)) := phi(['_balances_12'])
REF_5558(-> _balances_13) = REF_5558 + amount_1
 Transfer(address(0),account,amount)
TMP_13585 = CONVERT 0 to address
Emit Transfer(TMP_13585,account_1,amount_1)
 _afterTokenTransfer(address(0),account,amount)
TMP_13587 = CONVERT 0 to address
INTERNAL_CALL, AgentToken._afterTokenTransfer(address,address,uint256)(TMP_13587,account_1,amount_1)
```
#### AgentToken._mintBalances(uint256,uint256) [INTERNAL]
```slithir
lpMint__1(uint256) := phi(['lpSupply_1'])
vaultMint__1(uint256) := phi(['vaultSupply_1'])
vault_2(address) := phi(['vault_0', 'vault_3', 'vault_4', 'vault_1'])
 lpMint_ > 0
TMP_13390(bool) = lpMint__1 > 0
CONDITION TMP_13390
 _mint(address(this),lpMint_)
TMP_13391 = CONVERT this to address
INTERNAL_CALL, AgentToken._mint(address,uint256)(TMP_13391,lpMint__1)
 vaultMint_ > 0
TMP_13393(bool) = vaultMint__1 > 0
CONDITION TMP_13393
 _mint(vault,vaultMint_)
INTERNAL_CALL, AgentToken._mint(address,uint256)(vault_3,vaultMint__1)
```
#### AgentToken._pretaxValidationAndLimits(address,address,uint256) [INTERNAL]
```slithir
from__1(address) := phi(['from_1'])
to__1(address) := phi(['to_1'])
amount__1(uint256) := phi(['amount_1'])
uniswapV2Pair_11(address) := phi(['uniswapV2Pair_9', 'uniswapV2Pair_1', 'uniswapV2Pair_0'])
fundedDate_3(uint32) := phi(['fundedDate_0', 'fundedDate_2'])
_balances_4(mapping(address => uint256)) := phi(['_balances_10', '_balances_4', '_balances_13', '_balances_16', '_balances_0', '_balances_1', '_balances_9', '_balances_3', '_balances_7'])
 to_ == uniswapV2Pair && from_ != address(this) && fundedDate == 0
TMP_13496(bool) = to__1 == uniswapV2Pair_11
TMP_13497 = CONVERT this to address
TMP_13498(bool) = from__1 != TMP_13497
TMP_13499(bool) = TMP_13496 && TMP_13498
TMP_13500(bool) = fundedDate_3 == 0
TMP_13501(bool) = TMP_13499 && TMP_13500
CONDITION TMP_13501
 revert InitialLiquidityNotYetAdded()()
TMP_13502(None) = SOLIDITY_CALL revert InitialLiquidityNotYetAdded()()
 from_ == address(0)
TMP_13503 = CONVERT 0 to address
TMP_13504(bool) = from__1 == TMP_13503
CONDITION TMP_13504
 revert TransferFromZeroAddress()()
TMP_13505(None) = SOLIDITY_CALL revert TransferFromZeroAddress()()
 to_ == address(0)
TMP_13506 = CONVERT 0 to address
TMP_13507(bool) = to__1 == TMP_13506
CONDITION TMP_13507
 revert TransferToZeroAddress()()
TMP_13508(None) = SOLIDITY_CALL revert TransferToZeroAddress()()
 fromBalance_ = _balances[from_]
REF_5551(uint256) -> _balances_4[from__1]
fromBalance__1(uint256) := REF_5551(uint256)
 fromBalance_ < amount_
TMP_13509(bool) = fromBalance__1 < amount__1
CONDITION TMP_13509
 revert TransferAmountExceedsBalance()()
TMP_13510(None) = SOLIDITY_CALL revert TransferAmountExceedsBalance()()
 (fromBalance_)
RETURN fromBalance__1
 fromBalance_
```
#### AgentToken._processSupplyParams(IERC20Config.ERC20SupplyParameters) [INTERNAL]
```slithir
erc20SupplyParameters__1(IERC20Config.ERC20SupplyParameters) := phi(['supplyParams_1'])
 erc20SupplyParameters_.maxSupply != (erc20SupplyParameters_.vaultSupply + erc20SupplyParameters_.lpSupply)
REF_5519(uint256) -> erc20SupplyParameters__1.maxSupply
REF_5520(uint256) -> erc20SupplyParameters__1.vaultSupply
REF_5521(uint256) -> erc20SupplyParameters__1.lpSupply
TMP_13378(uint256) = REF_5520 (c)+ REF_5521
TMP_13379(bool) = REF_5519 != TMP_13378
CONDITION TMP_13379
 revert SupplyTotalMismatch()()
TMP_13380(None) = SOLIDITY_CALL revert SupplyTotalMismatch()()
 erc20SupplyParameters_.maxSupply > type()(uint128).max
REF_5522(uint256) -> erc20SupplyParameters__1.maxSupply
TMP_13382(uint128) := 340282366920938463463374607431768211455(uint128)
TMP_13383(bool) = REF_5522 > TMP_13382
CONDITION TMP_13383
 revert MaxSupplyTooHigh()()
TMP_13384(None) = SOLIDITY_CALL revert MaxSupplyTooHigh()()
 vault = erc20SupplyParameters_.vault
REF_5523(address) -> erc20SupplyParameters__1.vault
vault_1(address) := REF_5523(address)
```
#### AgentToken._processTaxParams(IERC20Config.ERC20TaxParameters) [INTERNAL]
```slithir
erc20TaxParameters__1(IERC20Config.ERC20TaxParameters) := phi(['taxParams_1'])
 erc20TaxParameters_.projectBuyTaxBasisPoints == 0 && erc20TaxParameters_.projectSellTaxBasisPoints == 0
REF_5524(uint256) -> erc20TaxParameters__1.projectBuyTaxBasisPoints
TMP_13385(bool) = REF_5524 == 0
REF_5525(uint256) -> erc20TaxParameters__1.projectSellTaxBasisPoints
TMP_13386(bool) = REF_5525 == 0
TMP_13387(bool) = TMP_13385 && TMP_13386
CONDITION TMP_13387
 false
RETURN False
 projectBuyTaxBasisPoints = uint16(erc20TaxParameters_.projectBuyTaxBasisPoints)
REF_5526(uint256) -> erc20TaxParameters__1.projectBuyTaxBasisPoints
TMP_13388 = CONVERT REF_5526 to uint16
projectBuyTaxBasisPoints_1(uint16) := TMP_13388(uint16)
 projectSellTaxBasisPoints = uint16(erc20TaxParameters_.projectSellTaxBasisPoints)
REF_5527(uint256) -> erc20TaxParameters__1.projectSellTaxBasisPoints
TMP_13389 = CONVERT REF_5527 to uint16
projectSellTaxBasisPoints_1(uint16) := TMP_13389(uint16)
 true
RETURN True
 tokenHasTax_
```
#### AgentToken._spendAllowance(address,address,uint256) [INTERNAL]
```slithir
owner_1(address) := phi(['from_1', 'account_1'])
spender_1(address) := phi(['spender_1', 'TMP_13619'])
amount_1(uint256) := phi(['value_1', 'amount_1'])
 currentAllowance = allowance(owner,spender)
TMP_13609(uint256) = INTERNAL_CALL, AgentToken.allowance(address,address)(owner_1,spender_1)
currentAllowance_1(uint256) := TMP_13609(uint256)
 currentAllowance != type()(uint256).max
TMP_13611(uint256) := 115792089237316195423570985008687907853269984665640564039457584007913129639935(uint256)
TMP_13612(bool) = currentAllowance_1 != TMP_13611
CONDITION TMP_13612
 currentAllowance < amount
TMP_13613(bool) = currentAllowance_1 < amount_1
CONDITION TMP_13613
 revert InsufficientAllowance()()
TMP_13614(None) = SOLIDITY_CALL revert InsufficientAllowance()()
 _approve(owner,spender,currentAllowance - amount)
TMP_13615(uint256) = currentAllowance_1 - amount_1
INTERNAL_CALL, AgentToken._approve(address,address,uint256)(owner_1,spender_1,TMP_13615)
```
#### AgentToken._swapTax(uint256,uint256) [INTERNAL]
```slithir
swapBalance__1(uint256) := phi(['swapBalance_3'])
contractBalance__1(uint256) := phi(['contractBalance_1'])
_uniswapRouter_16(IUniswapV2Router02) := phi(['_uniswapRouter_17', '_uniswapRouter_15', '_uniswapRouter_0', '_uniswapRouter_6', '_uniswapRouter_4', '_uniswapRouter_1', '_uniswapRouter_13'])
pairToken_14(address) := phi(['pairToken_13', 'pairToken_1', 'pairToken_4', 'pairToken_6', 'pairToken_0'])
projectTaxRecipient_3(address) := phi(['projectTaxRecipient_0', 'projectTaxRecipient_2', 'projectTaxRecipient_4', 'projectTaxRecipient_1', 'projectTaxRecipient_6'])
 path = new address[](2)
TMP_13556(address[])  = new address[](2)
path_1(address[]) = ['TMP_13556(address[])']
 path[0] = address(this)
REF_5553(address) -> path_1[0]
TMP_13557 = CONVERT this to address
path_2(address[]) := phi(['path_1'])
REF_5553(address) (->path_2) := TMP_13557(address)
 path[1] = pairToken
REF_5554(address) -> path_2[1]
path_3(address[]) := phi(['path_2'])
REF_5554(address) (->path_3) := pairToken_14(address)
 _uniswapRouter.swapExactTokensForTokensSupportingFeeOnTransferTokens(swapBalance_,0,path,projectTaxRecipient,block.timestamp + 600)
TMP_13558(uint256) = block.timestamp (c)+ 600
HIGH_LEVEL_CALL, dest:_uniswapRouter_16(IUniswapV2Router02), function:swapExactTokensForTokensSupportingFeeOnTransferTokens, arguments:['swapBalance__1', '0', 'path_3', 'projectTaxRecipient_3', 'TMP_13558']  
_uniswapRouter_17(IUniswapV2Router02) := phi(['_uniswapRouter_16', '_uniswapRouter_17', '_uniswapRouter_15', '_uniswapRouter_6', '_uniswapRouter_4', '_uniswapRouter_1', '_uniswapRouter_13'])
projectTaxRecipient_4(address) := phi(['projectTaxRecipient_2', 'projectTaxRecipient_4', 'projectTaxRecipient_1', 'projectTaxRecipient_3', 'projectTaxRecipient_6'])
 swapBalance_ < contractBalance_
TMP_13560(bool) = swapBalance__1 < contractBalance__1
CONDITION TMP_13560
 projectTaxPendingSwap -= uint128((projectTaxPendingSwap * swapBalance_) / contractBalance_)
TMP_13561(uint128) = projectTaxPendingSwap_7 (c)* swapBalance__1
TMP_13562(uint128) = TMP_13561 (c)/ contractBalance__1
TMP_13563 = CONVERT TMP_13562 to uint128
projectTaxPendingSwap_9(uint128) = projectTaxPendingSwap_7 (c)- TMP_13563
 projectTaxPendingSwap = 0
projectTaxPendingSwap_8(uint128) := 0(uint256)
projectTaxPendingSwap_10(uint128) := phi(['projectTaxPendingSwap_9', 'projectTaxPendingSwap_8'])
 ExternalCallError(5)
Emit ExternalCallError(5)
```
#### AgentToken._taxProcessing(bool,address,address,uint256) [INTERNAL]
```slithir
applyTax__1(bool) := phi(['applyTax_1'])
to__1(address) := phi(['to_1'])
from__1(address) := phi(['from_1'])
sentAmount__1(uint256) := phi(['amount_1'])
BP_DENOM_1(uint256) := phi(['BP_DENOM_5', 'BP_DENOM_0', 'BP_DENOM_3', 'BP_DENOM_7'])
_tokenHasTax_2(bool) := phi(['_tokenHasTax_1', '_tokenHasTax_0'])
projectBuyTaxBasisPoints_6(uint16) := phi(['projectBuyTaxBasisPoints_1', 'projectBuyTaxBasisPoints_8', 'projectBuyTaxBasisPoints_10', 'projectBuyTaxBasisPoints_0', 'projectBuyTaxBasisPoints_4'])
projectSellTaxBasisPoints_6(uint16) := phi(['projectSellTaxBasisPoints_4', 'projectSellTaxBasisPoints_1', 'projectSellTaxBasisPoints_8', 'projectSellTaxBasisPoints_0'])
_autoSwapInProgress_3(bool) := phi(['_autoSwapInProgress_0', '_autoSwapInProgress_2', '_autoSwapInProgress_1', '_autoSwapInProgress_8', '_autoSwapInProgress_6'])
projectTaxPendingSwap_1(uint128) := phi(['projectTaxPendingSwap_0', 'projectTaxPendingSwap_7', 'projectTaxPendingSwap_12', 'projectTaxPendingSwap_5', 'projectTaxPendingSwap_10', 'projectTaxPendingSwap_3'])
_balances_5(mapping(address => uint256)) := phi(['_balances_10', '_balances_4', '_balances_13', '_balances_16', '_balances_0', '_balances_1', '_balances_9', '_balances_3', '_balances_7'])
 amountLessTax_ = sentAmount_
amountLessTax__1(uint256) := sentAmount__1(uint256)
 _tokenHasTax && applyTax_ && ! _autoSwapInProgress
TMP_13511(bool) = _tokenHasTax_2 && applyTax__1
TMP_13512 = UnaryType.BANG _autoSwapInProgress_3 
TMP_13513(bool) = TMP_13511 && TMP_13512
CONDITION TMP_13513
 isLiquidityPool(to_) && totalSellTaxBasisPoints() > 0
TMP_13514(bool) = INTERNAL_CALL, AgentToken.isLiquidityPool(address)(to__1)
TMP_13515(uint256) = INTERNAL_CALL, AgentToken.totalSellTaxBasisPoints()()
TMP_13516(bool) = TMP_13515 > 0
TMP_13517(bool) = TMP_13514 && TMP_13516
CONDITION TMP_13517
 projectSellTaxBasisPoints > 0
TMP_13518(bool) = projectSellTaxBasisPoints_8 > 0
CONDITION TMP_13518
 projectTax = ((sentAmount_ * projectSellTaxBasisPoints) / BP_DENOM)
TMP_13519(uint256) = sentAmount__1 * projectSellTaxBasisPoints_8
TMP_13520(uint256) = TMP_13519 / BP_DENOM_3
projectTax_1(uint256) := TMP_13520(uint256)
 projectTaxPendingSwap += uint128(projectTax)
TMP_13521 = CONVERT projectTax_1 to uint128
projectTaxPendingSwap_7(uint128) = projectTaxPendingSwap_3 + TMP_13521
 tax += projectTax
tax_3(uint256) = tax_0 + projectTax_1
tax_4(uint256) := phi(['tax_0', 'tax_3'])
 isLiquidityPool(from_) && totalBuyTaxBasisPoints() > 0
TMP_13522(bool) = INTERNAL_CALL, AgentToken.isLiquidityPool(address)(from__1)
TMP_13523(uint256) = INTERNAL_CALL, AgentToken.totalBuyTaxBasisPoints()()
TMP_13524(bool) = TMP_13523 > 0
TMP_13525(bool) = TMP_13522 && TMP_13524
CONDITION TMP_13525
 projectBuyTaxBasisPoints > 0
TMP_13526(bool) = projectBuyTaxBasisPoints_10 > 0
CONDITION TMP_13526
 projectTax_scope_0 = ((sentAmount_ * projectBuyTaxBasisPoints) / BP_DENOM)
TMP_13527(uint256) = sentAmount__1 * projectBuyTaxBasisPoints_10
TMP_13528(uint256) = TMP_13527 / BP_DENOM_5
projectTax_scope_0_1(uint256) := TMP_13528(uint256)
 projectTaxPendingSwap += uint128(projectTax_scope_0)
TMP_13529 = CONVERT projectTax_scope_0_1 to uint128
projectTaxPendingSwap_6(uint128) = projectTaxPendingSwap_5 + TMP_13529
 tax += projectTax_scope_0
tax_1(uint256) = tax_0 + projectTax_scope_0_1
tax_2(uint256) := phi(['tax_1', 'tax_0'])
 tax > 0
TMP_13530(bool) = tax_4 > 0
CONDITION TMP_13530
 _balances[address(this)] += tax
TMP_13531 = CONVERT this to address
REF_5552(uint256) -> _balances_7[TMP_13531]
_balances_10(mapping(address => uint256)) := phi(['_balances_7'])
REF_5552(-> _balances_10) = REF_5552 + tax_4
 Transfer(from_,address(this),tax)
TMP_13532 = CONVERT this to address
Emit Transfer(from__1,TMP_13532,tax_4)
 amountLessTax_ -= tax
amountLessTax__2(uint256) = amountLessTax__1 - tax_4
amountLessTax__3(uint256) := phi(['amountLessTax__2', 'amountLessTax__1'])
 (amountLessTax_)
RETURN amountLessTax__3
 amountLessTax_
```
#### AgentToken._transfer(address,address,uint256,bool) [INTERNAL]
```slithir
from_1(address) := phi(['owner_1', 'from_1', 'TMP_13566'])
to_1(address) := phi(['projectTaxRecipient_5', 'to_1', 'to_1'])
amount_1(uint256) := phi(['amount_1', 'projectDistribution_1', 'amount_1'])
applyTax_1(bool) := phi(['TMP_13469', 'TMP_13477'])
 _beforeTokenTransfer(from,to,amount)
INTERNAL_CALL, AgentToken._beforeTokenTransfer(address,address,uint256)(from_1,to_1,amount_1)
 fromBalance = _pretaxValidationAndLimits(from,to,amount)
TMP_13490(uint256) = INTERNAL_CALL, AgentToken._pretaxValidationAndLimits(address,address,uint256)(from_1,to_1,amount_1)
fromBalance_1(uint256) := TMP_13490(uint256)
 _autoSwap(from,to)
INTERNAL_CALL, AgentToken._autoSwap(address,address)(from_1,to_1)
 amountMinusTax = _taxProcessing(applyTax,to,from,amount)
TMP_13492(uint256) = INTERNAL_CALL, AgentToken._taxProcessing(bool,address,address,uint256)(applyTax_1,to_1,from_1,amount_1)
amountMinusTax_1(uint256) := TMP_13492(uint256)
 _balances[from] = fromBalance - amount
REF_5549(uint256) -> _balances_1[from_1]
TMP_13493(uint256) = fromBalance_1 (c)- amount_1
_balances_2(mapping(address => uint256)) := phi(['_balances_1'])
REF_5549(uint256) (->_balances_2) := TMP_13493(uint256)
 _balances[to] += amountMinusTax
REF_5550(uint256) -> _balances_2[to_1]
_balances_3(mapping(address => uint256)) := phi(['_balances_2'])
REF_5550(-> _balances_3) = REF_5550 (c)+ amountMinusTax_1
 Transfer(from,to,amountMinusTax)
Emit Transfer(from_1,to_1,amountMinusTax_1)
 _afterTokenTransfer(from,to,amount)
INTERNAL_CALL, AgentToken._afterTokenTransfer(address,address,uint256)(from_1,to_1,amount_1)
```
#### AgentToken.addInitialLiquidity(address) [EXTERNAL]
```slithir
 _addInitialLiquidity(lpOwner)
INTERNAL_CALL, AgentToken._addInitialLiquidity(address)(lpOwner_1)
 onlyOwnerOrFactory()
MODIFIER_CALL, AgentToken.onlyOwnerOrFactory()()
```
#### AgentToken.addLiquidityPool(address) [PUBLIC]
```slithir
_liquidityPools_8(EnumerableSet.AddressSet) := phi(['_liquidityPools_3', '_liquidityPools_5', '_liquidityPools_9', '_liquidityPools_11', '_liquidityPools_0'])
 newLiquidityPool_ == address(0)
TMP_13440 = CONVERT 0 to address
TMP_13441(bool) = newLiquidityPool__1 == TMP_13440
CONDITION TMP_13441
 revert LiquidityPoolCannotBeAddressZero()()
TMP_13442(None) = SOLIDITY_CALL revert LiquidityPoolCannotBeAddressZero()()
 newLiquidityPool_.code.length == 0
TMP_13443(bytes) = SOLIDITY_CALL code(address)(newLiquidityPool__1)
REF_5539 -> LENGTH TMP_13443
TMP_13444(bool) = REF_5539 == 0
CONDITION TMP_13444
 revert LiquidityPoolMustBeAContractAddress()()
TMP_13445(None) = SOLIDITY_CALL revert LiquidityPoolMustBeAContractAddress()()
 _liquidityPools.add(newLiquidityPool_)
TMP_13446(bool) = LIBRARY_CALL, dest:EnumerableSet, function:EnumerableSet.add(EnumerableSet.AddressSet,address), arguments:['_liquidityPools_9', 'newLiquidityPool__1'] 
 LiquidityPoolAdded(newLiquidityPool_)
Emit LiquidityPoolAdded(newLiquidityPool__1)
 onlyOwnerOrFactory()
MODIFIER_CALL, AgentToken.onlyOwnerOrFactory()()
```
#### AgentToken.addValidCaller(bytes32) [EXTERNAL]
```slithir
_validCallerCodeHashes_3(EnumerableSet.Bytes32Set) := phi(['_validCallerCodeHashes_0', '_validCallerCodeHashes_4', '_validCallerCodeHashes_6'])
 _validCallerCodeHashes.add(newValidCallerHash_)
TMP_13454(bool) = LIBRARY_CALL, dest:EnumerableSet, function:EnumerableSet.add(EnumerableSet.Bytes32Set,bytes32), arguments:['_validCallerCodeHashes_4', 'newValidCallerHash__1'] 
 ValidCallerAdded(newValidCallerHash_)
Emit ValidCallerAdded(newValidCallerHash__1)
 onlyOwnerOrFactory()
MODIFIER_CALL, AgentToken.onlyOwnerOrFactory()()
```
#### AgentToken.allowance(address,address) [PUBLIC]
```slithir
owner_1(address) := phi(['owner_1', 'owner_1', 'owner_1'])
spender_1(address) := phi(['spender_1', 'spender_1', 'spender_1'])
_allowances_1(mapping(address => mapping(address => uint256))) := phi(['_allowances_0', '_allowances_2', '_allowances_1'])
 _allowances[owner][spender]
REF_5547(mapping(address => uint256)) -> _allowances_1[owner_1]
REF_5548(uint256) -> REF_5547[spender_1]
RETURN REF_5548
```
#### AgentToken.approve(address,uint256) [PUBLIC]
```slithir
 owner = _msgSender()
TMP_13471(address) = INTERNAL_CALL, ContextUpgradeable._msgSender()()
owner_1(address) := TMP_13471(address)
 _approve(owner,spender,amount)
INTERNAL_CALL, AgentToken._approve(address,address,uint256)(owner_1,spender_1,amount_1)
 true
RETURN True
```
#### AgentToken.balanceOf(address) [PUBLIC]
```slithir
account_1(address) := phi(['TMP_13534', 'TMP_13427', 'TMP_13412'])
_balances_1(mapping(address => uint256)) := phi(['_balances_10', '_balances_4', '_balances_13', '_balances_16', '_balances_0', '_balances_1', '_balances_9', '_balances_3', '_balances_7'])
 _balances[account]
REF_5546(uint256) -> _balances_1[account_1]
RETURN REF_5546
```
#### AgentToken.burn(uint256) [PUBLIC]
```slithir
 _burn(_msgSender(),value)
TMP_13617(address) = INTERNAL_CALL, ContextUpgradeable._msgSender()()
INTERNAL_CALL, AgentToken._burn(address,uint256)(TMP_13617,value_1)
```
#### AgentToken.burnFrom(address,uint256) [PUBLIC]
```slithir
 _spendAllowance(account,_msgSender(),value)
TMP_13619(address) = INTERNAL_CALL, ContextUpgradeable._msgSender()()
INTERNAL_CALL, AgentToken._spendAllowance(address,address,uint256)(account_1,TMP_13619,value_1)
 _burn(account,value)
INTERNAL_CALL, AgentToken._burn(address,uint256)(account_1,value_1)
```
#### AgentToken.constructor() [PUBLIC]
```slithir
 _disableInitializers()
INTERNAL_CALL, Initializable._disableInitializers()()
```
#### AgentToken.decimals() [PUBLIC]
```slithir
 18
RETURN 18
```
#### AgentToken.decreaseAllowance(address,uint256) [PUBLIC]
```slithir
 owner = _msgSender()
TMP_13483(address) = INTERNAL_CALL, ContextUpgradeable._msgSender()()
owner_1(address) := TMP_13483(address)
 currentAllowance = allowance(owner,spender)
TMP_13484(uint256) = INTERNAL_CALL, AgentToken.allowance(address,address)(owner_1,spender_1)
currentAllowance_1(uint256) := TMP_13484(uint256)
 currentAllowance < subtractedValue
TMP_13485(bool) = currentAllowance_1 < subtractedValue_1
CONDITION TMP_13485
 revert AllowanceDecreasedBelowZero()()
TMP_13486(None) = SOLIDITY_CALL revert AllowanceDecreasedBelowZero()()
 _approve(owner,spender,currentAllowance - subtractedValue)
TMP_13487(uint256) = currentAllowance_1 - subtractedValue_1
INTERNAL_CALL, AgentToken._approve(address,address,uint256)(owner_1,spender_1,TMP_13487)
 true
RETURN True
```
#### AgentToken.distributeTaxTokens() [EXTERNAL]
```slithir
projectTaxRecipient_5(address) := phi(['projectTaxRecipient_0', 'projectTaxRecipient_2', 'projectTaxRecipient_4', 'projectTaxRecipient_1', 'projectTaxRecipient_6'])
projectTaxPendingSwap_11(uint128) := phi(['projectTaxPendingSwap_0', 'projectTaxPendingSwap_7', 'projectTaxPendingSwap_12', 'projectTaxPendingSwap_5', 'projectTaxPendingSwap_10', 'projectTaxPendingSwap_3'])
 projectTaxPendingSwap > 0
TMP_13565(bool) = projectTaxPendingSwap_11 > 0
CONDITION TMP_13565
 projectDistribution = projectTaxPendingSwap
projectDistribution_1(uint256) := projectTaxPendingSwap_11(uint128)
 projectTaxPendingSwap = 0
projectTaxPendingSwap_12(uint128) := 0(uint256)
 _transfer(address(this),projectTaxRecipient,projectDistribution,false)
TMP_13566 = CONVERT this to address
INTERNAL_CALL, AgentToken._transfer(address,address,uint256,bool)(TMP_13566,projectTaxRecipient_5,projectDistribution_1,False)
```
#### AgentToken.increaseAllowance(address,uint256) [PUBLIC]
```slithir
 owner = _msgSender()
TMP_13479(address) = INTERNAL_CALL, ContextUpgradeable._msgSender()()
owner_1(address) := TMP_13479(address)
 _approve(owner,spender,allowance(owner,spender) + addedValue)
TMP_13480(uint256) = INTERNAL_CALL, AgentToken.allowance(address,address)(owner_1,spender_1)
TMP_13481(uint256) = TMP_13480 (c)+ addedValue_1
INTERNAL_CALL, AgentToken._approve(address,address,uint256)(owner_1,spender_1,TMP_13481)
 true
RETURN True
```
#### AgentToken.initialize(address[3],bytes,bytes,bytes) [EXTERNAL]
```slithir
 _decodeBaseParams(integrationAddresses_[0],baseParams_)
REF_5508(address) -> integrationAddresses__1[0]
INTERNAL_CALL, AgentToken._decodeBaseParams(address,bytes)(REF_5508,baseParams__1)
 _uniswapRouter = IUniswapV2Router02(integrationAddresses_[1])
REF_5509(address) -> integrationAddresses__1[1]
TMP_13360 = CONVERT REF_5509 to IUniswapV2Router02
_uniswapRouter_1(IUniswapV2Router02) := TMP_13360(IUniswapV2Router02)
 pairToken = integrationAddresses_[2]
REF_5510(address) -> integrationAddresses__1[2]
pairToken_1(address) := REF_5510(address)
 supplyParams = abi.decode(supplyParams_,(ERC20SupplyParameters))
TMP_13361(IERC20Config.ERC20SupplyParameters) = SOLIDITY_CALL abi.decode()(supplyParams__1,ERC20SupplyParameters)
supplyParams_1(IERC20Config.ERC20SupplyParameters) := TMP_13361(IERC20Config.ERC20SupplyParameters)
 taxParams = abi.decode(taxParams_,(ERC20TaxParameters))
TMP_13362(IERC20Config.ERC20TaxParameters) = SOLIDITY_CALL abi.decode()(taxParams__1,ERC20TaxParameters)
taxParams_1(IERC20Config.ERC20TaxParameters) := TMP_13362(IERC20Config.ERC20TaxParameters)
 _processSupplyParams(supplyParams)
INTERNAL_CALL, AgentToken._processSupplyParams(IERC20Config.ERC20SupplyParameters)(supplyParams_1)
 lpSupply = supplyParams.lpSupply * (10 ** decimals())
REF_5513(uint256) -> supplyParams_1.lpSupply
TMP_13364(uint8) = INTERNAL_CALL, AgentToken.decimals()()
TMP_13365(uint256) = 10 (c)** TMP_13364
TMP_13366(uint256) = REF_5513 (c)* TMP_13365
lpSupply_1(uint256) := TMP_13366(uint256)
 vaultSupply = supplyParams.vaultSupply * (10 ** decimals())
REF_5514(uint256) -> supplyParams_1.vaultSupply
TMP_13367(uint8) = INTERNAL_CALL, AgentToken.decimals()()
TMP_13368(uint256) = 10 (c)** TMP_13367
TMP_13369(uint256) = REF_5514 (c)* TMP_13368
vaultSupply_1(uint256) := TMP_13369(uint256)
 botProtectionDurationInSeconds = supplyParams.botProtectionDurationInSeconds
REF_5515(uint256) -> supplyParams_1.botProtectionDurationInSeconds
botProtectionDurationInSeconds_1(uint256) := REF_5515(uint256)
 _tokenHasTax = _processTaxParams(taxParams)
TMP_13370(bool) = INTERNAL_CALL, AgentToken._processTaxParams(IERC20Config.ERC20TaxParameters)(taxParams_1)
_tokenHasTax_1(bool) := TMP_13370(bool)
 swapThresholdBasisPoints = uint16(taxParams.taxSwapThresholdBasisPoints)
REF_5516(uint256) -> taxParams_1.taxSwapThresholdBasisPoints
TMP_13371 = CONVERT REF_5516 to uint16
swapThresholdBasisPoints_1(uint16) := TMP_13371(uint16)
 projectTaxRecipient = taxParams.projectTaxRecipient
REF_5517(address) -> taxParams_1.projectTaxRecipient
projectTaxRecipient_1(address) := REF_5517(address)
 _mintBalances(lpSupply,vaultSupply)
INTERNAL_CALL, AgentToken._mintBalances(uint256,uint256)(lpSupply_1,vaultSupply_1)
 uniswapV2Pair = _createPair()
TMP_13373(address) = INTERNAL_CALL, AgentToken._createPair()()
uniswapV2Pair_1(address) := TMP_13373(address)
 _factory = IAgentFactory(_msgSender())
TMP_13374(address) = INTERNAL_CALL, ContextUpgradeable._msgSender()()
TMP_13375 = CONVERT TMP_13374 to IAgentFactory
_factory_1(IAgentFactory) := TMP_13375(IAgentFactory)
 _autoSwapInProgress = true
_autoSwapInProgress_1(bool) := True(bool)
 initializer()
MODIFIER_CALL, Initializable.initializer()()
```
#### AgentToken.isLiquidityPool(address) [PUBLIC]
```slithir
queryAddress__1(address) := phi(['owner_1', 'to_1', 'from__1', 'from__1', 'to__1', 'to_1', 'from_1'])
uniswapV2Pair_10(address) := phi(['uniswapV2Pair_9', 'uniswapV2Pair_1', 'uniswapV2Pair_0'])
_liquidityPools_6(EnumerableSet.AddressSet) := phi(['_liquidityPools_3', '_liquidityPools_5', '_liquidityPools_9', '_liquidityPools_11', '_liquidityPools_0'])
 (queryAddress_ == uniswapV2Pair || _liquidityPools.contains(queryAddress_))
TMP_13436(bool) = queryAddress__1 == uniswapV2Pair_10
TMP_13437(bool) = LIBRARY_CALL, dest:EnumerableSet, function:EnumerableSet.contains(EnumerableSet.AddressSet,address), arguments:['_liquidityPools_6', 'queryAddress__1'] 
TMP_13438(bool) = TMP_13436 || TMP_13437
RETURN TMP_13438
```
#### AgentToken.isValidCaller(bytes32) [PUBLIC]
```slithir
_validCallerCodeHashes_1(EnumerableSet.Bytes32Set) := phi(['_validCallerCodeHashes_0', '_validCallerCodeHashes_4', '_validCallerCodeHashes_6'])
 (_validCallerCodeHashes.contains(queryHash_))
TMP_13452(bool) = LIBRARY_CALL, dest:EnumerableSet, function:EnumerableSet.contains(EnumerableSet.Bytes32Set,bytes32), arguments:['_validCallerCodeHashes_1', 'queryHash__1'] 
RETURN TMP_13452
```
#### AgentToken.liquidityPools() [EXTERNAL]
```slithir
_liquidityPools_7(EnumerableSet.AddressSet) := phi(['_liquidityPools_3', '_liquidityPools_5', '_liquidityPools_9', '_liquidityPools_11', '_liquidityPools_0'])
 (_liquidityPools.values())
TMP_13439(address[]) = LIBRARY_CALL, dest:EnumerableSet, function:EnumerableSet.values(EnumerableSet.AddressSet), arguments:['_liquidityPools_7'] 
RETURN TMP_13439
 liquidityPools_
```
#### AgentToken.name() [PUBLIC]
```slithir
_name_2(string) := phi(['_name_0', '_name_1'])
 _name
RETURN _name_2
```
#### AgentToken.receive() [EXTERNAL]
```slithir

```
#### AgentToken.removeLiquidityPool(address) [EXTERNAL]
```slithir
_liquidityPools_10(EnumerableSet.AddressSet) := phi(['_liquidityPools_3', '_liquidityPools_5', '_liquidityPools_9', '_liquidityPools_11', '_liquidityPools_0'])
 _liquidityPools.remove(removedLiquidityPool_)
TMP_13449(bool) = LIBRARY_CALL, dest:EnumerableSet, function:EnumerableSet.remove(EnumerableSet.AddressSet,address), arguments:['_liquidityPools_11', 'removedLiquidityPool__1'] 
 LiquidityPoolRemoved(removedLiquidityPool_)
Emit LiquidityPoolRemoved(removedLiquidityPool__1)
 onlyOwnerOrFactory()
MODIFIER_CALL, AgentToken.onlyOwnerOrFactory()()
```
#### AgentToken.removeValidCaller(bytes32) [EXTERNAL]
```slithir
_validCallerCodeHashes_5(EnumerableSet.Bytes32Set) := phi(['_validCallerCodeHashes_0', '_validCallerCodeHashes_4', '_validCallerCodeHashes_6'])
 _validCallerCodeHashes.remove(removedValidCallerHash_)
TMP_13457(bool) = LIBRARY_CALL, dest:EnumerableSet, function:EnumerableSet.remove(EnumerableSet.Bytes32Set,bytes32), arguments:['_validCallerCodeHashes_6', 'removedValidCallerHash__1'] 
 ValidCallerRemoved(removedValidCallerHash_)
Emit ValidCallerRemoved(removedValidCallerHash__1)
 onlyOwnerOrFactory()
MODIFIER_CALL, AgentToken.onlyOwnerOrFactory()()
```
#### AgentToken.setProjectTaxRates(uint16,uint16) [EXTERNAL]
```slithir
projectBuyTaxBasisPoints_2(uint16) := phi(['projectBuyTaxBasisPoints_1', 'projectBuyTaxBasisPoints_8', 'projectBuyTaxBasisPoints_10', 'projectBuyTaxBasisPoints_0', 'projectBuyTaxBasisPoints_4'])
projectSellTaxBasisPoints_2(uint16) := phi(['projectSellTaxBasisPoints_4', 'projectSellTaxBasisPoints_1', 'projectSellTaxBasisPoints_8', 'projectSellTaxBasisPoints_0'])
 oldBuyTaxBasisPoints = projectBuyTaxBasisPoints
oldBuyTaxBasisPoints_1(uint16) := projectBuyTaxBasisPoints_3(uint16)
 oldSellTaxBasisPoints = projectSellTaxBasisPoints
oldSellTaxBasisPoints_1(uint16) := projectSellTaxBasisPoints_3(uint16)
 projectBuyTaxBasisPoints = newProjectBuyTaxBasisPoints_
projectBuyTaxBasisPoints_4(uint16) := newProjectBuyTaxBasisPoints__1(uint16)
 projectSellTaxBasisPoints = newProjectSellTaxBasisPoints_
projectSellTaxBasisPoints_4(uint16) := newProjectSellTaxBasisPoints__1(uint16)
 ProjectTaxBasisPointsChanged(oldBuyTaxBasisPoints,newProjectBuyTaxBasisPoints_,oldSellTaxBasisPoints,newProjectSellTaxBasisPoints_)
Emit ProjectTaxBasisPointsChanged(oldBuyTaxBasisPoints_1,newProjectBuyTaxBasisPoints__1,oldSellTaxBasisPoints_1,newProjectSellTaxBasisPoints__1)
 onlyOwnerOrFactory()
MODIFIER_CALL, AgentToken.onlyOwnerOrFactory()()
```
#### AgentToken.setProjectTaxRecipient(address) [EXTERNAL]
```slithir
 projectTaxRecipient = projectTaxRecipient_
projectTaxRecipient_2(address) := projectTaxRecipient__1(address)
 ProjectTaxRecipientUpdated(projectTaxRecipient_)
Emit ProjectTaxRecipientUpdated(projectTaxRecipient__1)
 onlyOwnerOrFactory()
MODIFIER_CALL, AgentToken.onlyOwnerOrFactory()()
```
#### AgentToken.setSwapThresholdBasisPoints(uint16) [EXTERNAL]
```slithir
swapThresholdBasisPoints_2(uint16) := phi(['swapThresholdBasisPoints_6', 'swapThresholdBasisPoints_0', 'swapThresholdBasisPoints_4', 'swapThresholdBasisPoints_1'])
 oldswapThresholdBasisPoints = swapThresholdBasisPoints
oldswapThresholdBasisPoints_1(uint256) := swapThresholdBasisPoints_3(uint16)
 swapThresholdBasisPoints = swapThresholdBasisPoints_
swapThresholdBasisPoints_4(uint16) := swapThresholdBasisPoints__1(uint16)
 AutoSwapThresholdUpdated(oldswapThresholdBasisPoints,swapThresholdBasisPoints_)
Emit AutoSwapThresholdUpdated(oldswapThresholdBasisPoints_1,swapThresholdBasisPoints__1)
 onlyOwnerOrFactory()
MODIFIER_CALL, AgentToken.onlyOwnerOrFactory()()
```
#### AgentNftV2.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 DEFAULT_ADMIN_ROLE = 0x00
 MINTER_ROLE = keccak256(bytes)(MINTER_ROLE)
 VALIDATOR_ADMIN_ROLE = keccak256(bytes)(VALIDATOR_ADMIN_ROLE)
 ADMIN_ROLE = keccak256(bytes)(ADMIN_ROLE)
 $ = _getInitializableStorage()
TMP_13275(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_13275'])(Initializable.InitializableStorage) := TMP_13275(Initializable.InitializableStorage)
 isTopLevelCall = ! $._initializing
REF_5484(bool) -> $_1 (-> ['TMP_13275'])._initializing
TMP_13276 = UnaryType.BANG REF_5484 
isTopLevelCall_1(bool) := TMP_13276(bool)
 initialized = $._initialized
REF_5485(uint64) -> $_1 (-> ['TMP_13275'])._initialized
initialized_1(uint64) := REF_5485(uint64)
 initialSetup = initialized == 0 && isTopLevelCall
TMP_13277(bool) = initialized_1 == 0
TMP_13278(bool) = TMP_13277 && isTopLevelCall_1
initialSetup_1(bool) := TMP_13278(bool)
 construction = initialized == 1 && address(this).code.length == 0
TMP_13279(bool) = initialized_1 == 1
TMP_13280 = CONVERT this to address
TMP_13281(bytes) = SOLIDITY_CALL code(address)(TMP_13280)
REF_5486 -> LENGTH TMP_13281
TMP_13282(bool) = REF_5486 == 0
TMP_13283(bool) = TMP_13279 && TMP_13282
construction_1(bool) := TMP_13283(bool)
 ! initialSetup && ! construction
TMP_13284 = UnaryType.BANG initialSetup_1 
TMP_13285 = UnaryType.BANG construction_1 
TMP_13286(bool) = TMP_13284 && TMP_13285
CONDITION TMP_13286
 revert InvalidInitialization()()
TMP_13287(None) = SOLIDITY_CALL revert InvalidInitialization()()
 $._initialized = 1
REF_5487(uint64) -> $_1 (-> ['TMP_13275'])._initialized
$_2 (-> ['TMP_13275'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_13275'])"])
REF_5487(uint64) (->$_2 (-> ['TMP_13275'])) := 1(uint256)
TMP_13275(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_13275'])"])
 isTopLevelCall
CONDITION isTopLevelCall_1
 $._initializing = true
REF_5488(bool) -> $_2 (-> ['TMP_13275'])._initializing
$_3 (-> ['TMP_13275'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_13275'])"])
REF_5488(bool) (->$_3 (-> ['TMP_13275'])) := True(bool)
TMP_13275(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_13275'])"])
$_4 (-> ['TMP_13275'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_13275'])", "$_2 (-> ['TMP_13275'])"])
 isTopLevelCall
CONDITION isTopLevelCall_1
 $._initializing = false
REF_5489(bool) -> $_4 (-> ['TMP_13275'])._initializing
$_5 (-> ['TMP_13275'])(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_13275'])"])
REF_5489(bool) (->$_5 (-> ['TMP_13275'])) := False(bool)
TMP_13275(Initializable.InitializableStorage) := phi(["$_5 (-> ['TMP_13275'])"])
 Initialized(1)
Emit Initialized(1)
 $ = _getInitializableStorage()
TMP_13289(Initializable.InitializableStorage) = INTERNAL_CALL, Initializable._getInitializableStorage()()
$_1 (-> ['TMP_13289'])(Initializable.InitializableStorage) := TMP_13289(Initializable.InitializableStorage)
 $._initializing || $._initialized >= version
REF_5490(bool) -> $_1 (-> ['TMP_13289'])._initializing
REF_5491(uint64) -> $_1 (-> ['TMP_13289'])._initialized
TMP_13290(bool) = REF_5491 >= version_1
TMP_13291(bool) = REF_5490 || TMP_13290
CONDITION TMP_13291
 revert InvalidInitialization()()
TMP_13292(None) = SOLIDITY_CALL revert InvalidInitialization()()
 $._initialized = version
REF_5492(uint64) -> $_1 (-> ['TMP_13289'])._initialized
$_2 (-> ['TMP_13289'])(Initializable.InitializableStorage) := phi(["$_1 (-> ['TMP_13289'])"])
REF_5492(uint64) (->$_2 (-> ['TMP_13289'])) := version_1(uint64)
TMP_13289(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_13289'])"])
 $._initializing = true
REF_5493(bool) -> $_2 (-> ['TMP_13289'])._initializing
$_3 (-> ['TMP_13289'])(Initializable.InitializableStorage) := phi(["$_2 (-> ['TMP_13289'])"])
REF_5493(bool) (->$_3 (-> ['TMP_13289'])) := True(bool)
TMP_13289(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_13289'])"])
 $._initializing = false
REF_5494(bool) -> $_3 (-> ['TMP_13289'])._initializing
$_4 (-> ['TMP_13289'])(Initializable.InitializableStorage) := phi(["$_3 (-> ['TMP_13289'])"])
REF_5494(bool) (->$_4 (-> ['TMP_13289'])) := False(bool)
TMP_13289(Initializable.InitializableStorage) := phi(["$_4 (-> ['TMP_13289'])"])
 Initialized(version)
Emit Initialized(version_1)
 _checkInitializing()
INTERNAL_CALL, Initializable._checkInitializing()()
role_1(bytes32) := phi(['MINTER_ROLE_3', 'ADMIN_ROLE_17', 'ADMIN_ROLE_15', 'TMP_13020', 'DEFAULT_ADMIN_ROLE_11', 'DEFAULT_ADMIN_ROLE_9', 'TMP_13017', 'TMP_13022', 'TMP_13015', 'MINTER_ROLE_1', 'ADMIN_ROLE_11', 'ADMIN_ROLE_13'])
 _checkRole(role)
INTERNAL_CALL, AccessControlUpgradeable._checkRole(bytes32)(role_1)
virtualId_1(uint256) := phi(['virtualId_1', 'virtualId_1'])
virtualInfos_27(mapping(uint256 => IAgentNft.VirtualInfo)) := phi(['virtualInfos_0', 'virtualInfos_26', 'virtualInfos_28', 'virtualInfos_22', 'virtualInfos_9', 'virtualInfos_10', 'virtualInfos_15', 'virtualInfos_18', 'virtualInfos_8', 'virtualInfos_12', 'virtualInfos_21', 'virtualInfos_11'])
 require(bool,string)(_msgSender() == virtualInfos[virtualId].dao,Caller is not VIRTUAL DAO)
TMP_13296(address) = INTERNAL_CALL, ContextUpgradeable._msgSender()()
REF_5495(IAgentNft.VirtualInfo) -> virtualInfos_28[virtualId_1]
REF_5496(address) -> REF_5495.dao
TMP_13297(bool) = TMP_13296 == REF_5496
TMP_13298(None) = SOLIDITY_CALL require(bool,string)(TMP_13297,Caller is not VIRTUAL DAO)
_serviceNft_4(address) := phi(['_serviceNft_0', '_serviceNft_1', '_serviceNft_5'])
 require(bool,string)(_msgSender() == _serviceNft,Caller is not Service NFT)
TMP_13299(address) = INTERNAL_CALL, ContextUpgradeable._msgSender()()
TMP_13300(bool) = TMP_13299 == _serviceNft_5
TMP_13301(None) = SOLIDITY_CALL require(bool,string)(TMP_13300,Caller is not Service NFT)
```
#### AgentToken.symbol() [PUBLIC]
```slithir
_symbol_2(string) := phi(['_symbol_1', '_symbol_0'])
 _symbol
RETURN _symbol_2
```
#### AgentToken.totalBuyTaxBasisPoints() [PUBLIC]
```slithir
projectBuyTaxBasisPoints_5(uint16) := phi(['projectBuyTaxBasisPoints_1', 'projectBuyTaxBasisPoints_8', 'projectBuyTaxBasisPoints_10', 'projectBuyTaxBasisPoints_0', 'projectBuyTaxBasisPoints_4'])
 projectBuyTaxBasisPoints
RETURN projectBuyTaxBasisPoints_5
```
#### AgentToken.totalSellTaxBasisPoints() [PUBLIC]
```slithir
projectSellTaxBasisPoints_5(uint16) := phi(['projectSellTaxBasisPoints_4', 'projectSellTaxBasisPoints_1', 'projectSellTaxBasisPoints_8', 'projectSellTaxBasisPoints_0'])
 projectSellTaxBasisPoints
RETURN projectSellTaxBasisPoints_5
```
#### AgentToken.totalSupply() [PUBLIC]
```slithir
_totalSupply_1(uint256) := phi(['_totalSupply_6', '_totalSupply_0', '_totalSupply_9', '_totalSupply_3'])
 _totalSupply
RETURN _totalSupply_1
```
#### AgentToken.transfer(address,uint256) [PUBLIC]
```slithir
 owner = _msgSender()
TMP_13466(address) = INTERNAL_CALL, ContextUpgradeable._msgSender()()
owner_1(address) := TMP_13466(address)
 _transfer(owner,to,amount,(isLiquidityPool(owner) || isLiquidityPool(to)))
TMP_13467(bool) = INTERNAL_CALL, AgentToken.isLiquidityPool(address)(owner_1)
TMP_13468(bool) = INTERNAL_CALL, AgentToken.isLiquidityPool(address)(to_1)
TMP_13469(bool) = TMP_13467 || TMP_13468
INTERNAL_CALL, AgentToken._transfer(address,address,uint256,bool)(owner_1,to_1,amount_1,TMP_13469)
 true
RETURN True
```
#### AgentToken.transferFrom(address,address,uint256) [PUBLIC]
```slithir
 spender = _msgSender()
TMP_13473(address) = INTERNAL_CALL, ContextUpgradeable._msgSender()()
spender_1(address) := TMP_13473(address)
 _spendAllowance(from,spender,amount)
INTERNAL_CALL, AgentToken._spendAllowance(address,address,uint256)(from_1,spender_1,amount_1)
 _transfer(from,to,amount,(isLiquidityPool(from) || isLiquidityPool(to)))
TMP_13475(bool) = INTERNAL_CALL, AgentToken.isLiquidityPool(address)(from_1)
TMP_13476(bool) = INTERNAL_CALL, AgentToken.isLiquidityPool(address)(to_1)
TMP_13477(bool) = TMP_13475 || TMP_13476
INTERNAL_CALL, AgentToken._transfer(address,address,uint256,bool)(from_1,to_1,amount_1,TMP_13477)
 true
RETURN True
```
#### AgentToken.validCallers() [EXTERNAL]
```slithir
_validCallerCodeHashes_2(EnumerableSet.Bytes32Set) := phi(['_validCallerCodeHashes_0', '_validCallerCodeHashes_4', '_validCallerCodeHashes_6'])
 (_validCallerCodeHashes.values())
TMP_13453(bytes32[]) = LIBRARY_CALL, dest:EnumerableSet, function:EnumerableSet.values(EnumerableSet.Bytes32Set), arguments:['_validCallerCodeHashes_2'] 
RETURN TMP_13453
 validCallerHashes_
```
#### AgentToken.withdrawERC20(address,uint256) [EXTERNAL]
```slithir
 token_ == address(this)
TMP_13572 = CONVERT this to address
TMP_13573(bool) = token__1 == TMP_13572
CONDITION TMP_13573
 revert CannotWithdrawThisToken()()
TMP_13574(None) = SOLIDITY_CALL revert CannotWithdrawThisToken()()
 IERC20(token_).safeTransfer(_msgSender(),amount_)
TMP_13575 = CONVERT token__1 to IERC20
TMP_13576(address) = INTERNAL_CALL, ContextUpgradeable._msgSender()()
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['TMP_13575', 'TMP_13576', 'amount__1'] 
 onlyOwnerOrFactory()
MODIFIER_CALL, AgentToken.onlyOwnerOrFactory()()
```
#### AgentToken.withdrawETH(uint256) [EXTERNAL]
```slithir
 (success,None) = _msgSender().call{value: amount_}()
TMP_13568(address) = INTERNAL_CALL, ContextUpgradeable._msgSender()()
TUPLE_128(bool,bytes) = LOW_LEVEL_CALL, dest:TMP_13568, function:call, arguments:[''] value:amount__1 
success_1(bool)= UNPACK TUPLE_128 index: 0 
 ! success
TMP_13569 = UnaryType.BANG success_1 
CONDITION TMP_13569
 revert TransferFailed()()
TMP_13570(None) = SOLIDITY_CALL revert TransferFailed()()
 onlyOwnerOrFactory()
MODIFIER_CALL, AgentToken.onlyOwnerOrFactory()()
```
#### IERC20.approve(address,uint256) [EXTERNAL]
```slithir

```
#### IERC20.balanceOf(address) [EXTERNAL]
```slithir

```
#### IERC20.transfer(address,uint256) [EXTERNAL]
```slithir

```
#### EnumerableSet.add(EnumerableSet.UintSet,uint256) [INTERNAL]
```slithir
 _add(set._inner,bytes32(value))
REF_2342(EnumerableSet.Set) -> set_1 (-> [])._inner
TMP_6356 = CONVERT value_1 to bytes32
TMP_6357(bool) = INTERNAL_CALL, EnumerableSet._add(EnumerableSet.Set,bytes32)(REF_2342,TMP_6356)
RETURN TMP_6357
```
#### IUniswapV2Factory.createPair(address,address) [EXTERNAL]
```slithir

```
#### IUniswapV2Factory.getPair(address,address) [EXTERNAL]
```slithir

```
#### IUniswapV2Router02.swapExactTokensForTokensSupportingFeeOnTransferTokens(uint256,uint256,address[],address,uint256) [EXTERNAL]
```slithir

```
#### EnumerableSet.contains(EnumerableSet.UintSet,uint256) [INTERNAL]
```slithir
 _contains(set._inner,bytes32(value))
REF_2344(EnumerableSet.Set) -> set_1 (-> [])._inner
TMP_6360 = CONVERT value_1 to bytes32
TMP_6361(bool) = INTERNAL_CALL, EnumerableSet._contains(EnumerableSet.Set,bytes32)(REF_2344,TMP_6360)
RETURN TMP_6361
```
#### EnumerableSet.values(EnumerableSet.AddressSet) [INTERNAL]
```slithir
 store = _values(set._inner)
REF_2341(EnumerableSet.Set) -> set_1 (-> [])._inner
TMP_6355(bytes32[]) = INTERNAL_CALL, EnumerableSet._values(EnumerableSet.Set)(REF_2341)
store_1(bytes32[]) = ['TMP_6355(bytes32[])']
 result = store
result_1(address[]) := store_1(bytes32[])
 result
RETURN result_1
```
#### EnumerableSet.remove(EnumerableSet.UintSet,uint256) [INTERNAL]
```slithir
 _remove(set._inner,bytes32(value))
REF_2343(EnumerableSet.Set) -> set_1 (-> [])._inner
TMP_6358 = CONVERT value_1 to bytes32
TMP_6359(bool) = INTERNAL_CALL, EnumerableSet._remove(EnumerableSet.Set,bytes32)(REF_2343,TMP_6358)
RETURN TMP_6359
```
#### SafeERC20.safeTransfer(IERC20,address,uint256) [INTERNAL]
```slithir
 _callOptionalReturn(token,abi.encodeCall(token.transfer,(to,value)))
REF_2044(transfer) -> token_1.transfer
TMP_5413(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_2044,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78000550>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0xffff78001210>])
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_5413)
```
#### EnumerableSet._add(EnumerableSet.Set,bytes32) [PRIVATE]
```slithir
set_1 (-> [])(EnumerableSet.Set) := phi(['REF_2342', 'REF_2336', 'REF_2330'])
value_1(bytes32) := phi(['TMP_6356', 'TMP_6340', 'value_1'])
 ! _contains(set,value)
TMP_6320(bool) = INTERNAL_CALL, EnumerableSet._contains(EnumerableSet.Set,bytes32)(set_1 (-> []),value_1)
TMP_6321 = UnaryType.BANG TMP_6320 
CONDITION TMP_6321
 set._values.push(value)
REF_2298(bytes32[]) -> set_1 (-> [])._values
REF_2300 -> LENGTH REF_2298
TMP_6323(uint256) := REF_2300(uint256)
TMP_6324(uint256) = TMP_6323 (c)+ 1
set_2 (-> [])(EnumerableSet.Set) := phi(['set_1 (-> [])'])
REF_2300(uint256) (->set_3 (-> [])) := TMP_6324(uint256)
REF_2301(bytes32) -> REF_2298[TMP_6323]
set_3 (-> [])(EnumerableSet.Set) := phi(['set_2 (-> [])'])
REF_2301(bytes32) (->set_3 (-> [])) := value_1(bytes32)
 set._positions[value] = set._values.length
REF_2302(mapping(bytes32 => uint256)) -> set_3 (-> [])._positions
REF_2303(uint256) -> REF_2302[value_1]
REF_2304(bytes32[]) -> set_3 (-> [])._values
REF_2305 -> LENGTH REF_2304
set_4 (-> [])(EnumerableSet.Set) := phi(['set_3 (-> [])'])
REF_2303(uint256) (->set_4 (-> [])) := REF_2305(uint256)
 true
RETURN True
 false
RETURN False
```
#### EnumerableSet._contains(EnumerableSet.Set,bytes32) [PRIVATE]
```slithir
set_1 (-> [])(EnumerableSet.Set) := phi(['REF_2344', 'set_1 (-> [])', 'REF_2332', 'REF_2338'])
value_1(bytes32) := phi(['TMP_6348', 'value_1', 'TMP_6360', 'value_1'])
 set._positions[value] != 0
REF_2323(mapping(bytes32 => uint256)) -> set_1 (-> [])._positions
REF_2324(uint256) -> REF_2323[value_1]
TMP_6331(bool) = REF_2324 != 0
RETURN TMP_6331
```
#### EnumerableSet._values(EnumerableSet.Set) [PRIVATE]
```slithir
set_1 (-> [])(EnumerableSet.Set) := phi(['REF_2335', 'REF_2341', 'REF_2347'])
 set._values
REF_2329(bytes32[]) -> set_1 (-> [])._values
RETURN REF_2329
```
#### EnumerableSet._remove(EnumerableSet.Set,bytes32) [PRIVATE]
```slithir
set_1 (-> [])(EnumerableSet.Set) := phi(['REF_2331', 'REF_2343', 'REF_2337'])
value_1(bytes32) := phi(['value_1', 'TMP_6358', 'TMP_6344'])
 position = set._positions[value]
REF_2306(mapping(bytes32 => uint256)) -> set_1 (-> [])._positions
REF_2307(uint256) -> REF_2306[value_1]
position_1(uint256) := REF_2307(uint256)
 position != 0
TMP_6325(bool) = position_1 != 0
CONDITION TMP_6325
 valueIndex = position - 1
TMP_6326(uint256) = position_1 (c)- 1
valueIndex_1(uint256) := TMP_6326(uint256)
 lastIndex = set._values.length - 1
REF_2308(bytes32[]) -> set_1 (-> [])._values
REF_2309 -> LENGTH REF_2308
TMP_6327(uint256) = REF_2309 (c)- 1
lastIndex_1(uint256) := TMP_6327(uint256)
 valueIndex != lastIndex
TMP_6328(bool) = valueIndex_1 != lastIndex_1
CONDITION TMP_6328
 lastValue = set._values[lastIndex]
REF_2310(bytes32[]) -> set_1 (-> [])._values
REF_2311(bytes32) -> REF_2310[lastIndex_1]
lastValue_1(bytes32) := REF_2311(bytes32)
 set._values[valueIndex] = lastValue
REF_2312(bytes32[]) -> set_1 (-> [])._values
REF_2313(bytes32) -> REF_2312[valueIndex_1]
set_2 (-> [])(EnumerableSet.Set) := phi(['set_1 (-> [])'])
REF_2313(bytes32) (->set_2 (-> [])) := lastValue_1(bytes32)
 set._positions[lastValue] = position
REF_2314(mapping(bytes32 => uint256)) -> set_2 (-> [])._positions
REF_2315(uint256) -> REF_2314[lastValue_1]
set_3 (-> [])(EnumerableSet.Set) := phi(['set_2 (-> [])'])
REF_2315(uint256) (->set_3 (-> [])) := position_1(uint256)
set_4 (-> [])(EnumerableSet.Set) := phi(['set_1 (-> [])', 'set_3 (-> [])'])
 set._values.pop()
REF_2316(bytes32[]) -> set_4 (-> [])._values
REF_2318 -> LENGTH REF_2316
TMP_6330(uint256) = REF_2318 (c)- 1
REF_2319(bytes32) -> REF_2316[TMP_6330]
REF_2316 = delete REF_2319 
REF_2320 -> LENGTH REF_2316
set_5 (-> [])(EnumerableSet.Set) := phi(['set_4 (-> [])'])
REF_2320(uint256) (->set_5 (-> [])) := TMP_6330(uint256)
 delete set._positions[value]
REF_2321(mapping(bytes32 => uint256)) -> set_5 (-> [])._positions
REF_2322(uint256) -> REF_2321[value_1]
REF_2321 = delete REF_2322 
 true
RETURN True
 false
RETURN False
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
