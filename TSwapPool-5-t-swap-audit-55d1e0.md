### Storage layout (TSwapPool) 

```text
swap_count uint256

```


#### TSwapPool.constructor(address,address,string,string) [INTERNAL]
```slithir
 i_wethToken = IERC20(wethToken)
TMP_181 = CONVERT wethToken_1 to IERC20
i_wethToken_1(IERC20) := TMP_181(IERC20)
 i_poolToken = IERC20(poolToken)
TMP_182 = CONVERT poolToken_1 to IERC20
i_poolToken_1(IERC20) := TMP_182(IERC20)
 ERC20(liquidityTokenName,liquidityTokenSymbol)
INTERNAL_CALL, ERC20.constructor(string,string)(liquidityTokenName_1,liquidityTokenSymbol_1)
```
#### TSwapPool.deposit(uint256,uint256,uint256,uint64) [EXTERNAL]
```slithir
i_wethToken_2(IERC20) := phi(['i_wethToken_19', 'i_wethToken_17', 'i_wethToken_4', 'i_wethToken_0', 'i_wethToken_32', 'i_wethToken_23', 'i_wethToken_28', 'i_wethToken_7', 'i_wethToken_1'])
i_poolToken_2(IERC20) := phi(['i_poolToken_8', 'i_poolToken_28', 'i_poolToken_23', 'i_poolToken_18', 'i_poolToken_4', 'i_poolToken_0', 'i_poolToken_1', 'i_poolToken_32', 'i_poolToken_20'])
MINIMUM_WETH_LIQUIDITY_1(uint256) := phi(['MINIMUM_WETH_LIQUIDITY_2', 'MINIMUM_WETH_LIQUIDITY_0'])
 wethToDeposit < MINIMUM_WETH_LIQUIDITY
TMP_184(bool) = wethToDeposit_1 < MINIMUM_WETH_LIQUIDITY_2
CONDITION TMP_184
 revert TSwapPool__WethDepositAmountTooLow(uint256,uint256)(MINIMUM_WETH_LIQUIDITY,wethToDeposit)
TMP_185(None) = SOLIDITY_CALL revert TSwapPool__WethDepositAmountTooLow(uint256,uint256)(MINIMUM_WETH_LIQUIDITY_2,wethToDeposit_1)
 totalLiquidityTokenSupply() > 0
TMP_186(uint256) = INTERNAL_CALL, TSwapPool.totalLiquidityTokenSupply()()
TMP_187(bool) = TMP_186 > 0
CONDITION TMP_187
 wethReserves = i_wethToken.balanceOf(address(this))
TMP_188 = CONVERT this to address
TMP_189(uint256) = HIGH_LEVEL_CALL, dest:i_wethToken_4(IERC20), function:balanceOf, arguments:['TMP_188']  
i_wethToken_5(IERC20) := phi(['i_wethToken_19', 'i_wethToken_17', 'i_wethToken_4', 'i_wethToken_32', 'i_wethToken_23', 'i_wethToken_28', 'i_wethToken_7', 'i_wethToken_1'])
i_poolToken_5(IERC20) := phi(['i_poolToken_8', 'i_poolToken_28', 'i_poolToken_23', 'i_poolToken_18', 'i_poolToken_4', 'i_poolToken_1', 'i_poolToken_32', 'i_poolToken_20'])
wethReserves_1(uint256) := TMP_189(uint256)
 poolTokenReserves = i_poolToken.balanceOf(address(this))
TMP_190 = CONVERT this to address
TMP_191(uint256) = HIGH_LEVEL_CALL, dest:i_poolToken_5(IERC20), function:balanceOf, arguments:['TMP_190']  
i_poolToken_6(IERC20) := phi(['i_poolToken_8', 'i_poolToken_28', 'i_poolToken_23', 'i_poolToken_5', 'i_poolToken_18', 'i_poolToken_4', 'i_poolToken_1', 'i_poolToken_32', 'i_poolToken_20'])
poolTokenReserves_1(uint256) := TMP_191(uint256)
 poolTokensToDeposit = getPoolTokensToDepositBasedOnWeth(wethToDeposit)
TMP_192(uint256) = INTERNAL_CALL, TSwapPool.getPoolTokensToDepositBasedOnWeth(uint256)(wethToDeposit_1)
poolTokensToDeposit_1(uint256) := TMP_192(uint256)
 maximumPoolTokensToDeposit < poolTokensToDeposit
TMP_193(bool) = maximumPoolTokensToDeposit_1 < poolTokensToDeposit_1
CONDITION TMP_193
 revert TSwapPool__MaxPoolTokenDepositTooHigh(uint256,uint256)(maximumPoolTokensToDeposit,poolTokensToDeposit)
TMP_194(None) = SOLIDITY_CALL revert TSwapPool__MaxPoolTokenDepositTooHigh(uint256,uint256)(maximumPoolTokensToDeposit_1,poolTokensToDeposit_1)
 liquidityTokensToMint = (wethToDeposit * totalLiquidityTokenSupply()) / wethReserves
TMP_195(uint256) = INTERNAL_CALL, TSwapPool.totalLiquidityTokenSupply()()
TMP_196(uint256) = wethToDeposit_1 (c)* TMP_195
TMP_197(uint256) = TMP_196 (c)/ wethReserves_1
liquidityTokensToMint_1(uint256) := TMP_197(uint256)
 liquidityTokensToMint < minimumLiquidityTokensToMint
TMP_198(bool) = liquidityTokensToMint_1 < minimumLiquidityTokensToMint_1
CONDITION TMP_198
 revert TSwapPool__MinLiquidityTokensToMintTooLow(uint256,uint256)(minimumLiquidityTokensToMint,liquidityTokensToMint)
TMP_199(None) = SOLIDITY_CALL revert TSwapPool__MinLiquidityTokensToMintTooLow(uint256,uint256)(minimumLiquidityTokensToMint_1,liquidityTokensToMint_1)
 _addLiquidityMintAndTransfer(wethToDeposit,poolTokensToDeposit,liquidityTokensToMint)
INTERNAL_CALL, TSwapPool._addLiquidityMintAndTransfer(uint256,uint256,uint256)(wethToDeposit_1,poolTokensToDeposit_1,liquidityTokensToMint_1)
 _addLiquidityMintAndTransfer(wethToDeposit,maximumPoolTokensToDeposit,wethToDeposit)
INTERNAL_CALL, TSwapPool._addLiquidityMintAndTransfer(uint256,uint256,uint256)(wethToDeposit_1,maximumPoolTokensToDeposit_1,wethToDeposit_1)
 liquidityTokensToMint = wethToDeposit
liquidityTokensToMint_2(uint256) := wethToDeposit_1(uint256)
liquidityTokensToMint_3(uint256) := phi(['liquidityTokensToMint_1', 'liquidityTokensToMint_0', 'liquidityTokensToMint_2'])
 revertIfZero(wethToDeposit)
MODIFIER_CALL, TSwapPool.revertIfZero(uint256)(wethToDeposit_1)
 liquidityTokensToMint
RETURN liquidityTokensToMint_3
```
#### TSwapPool._addLiquidityMintAndTransfer(uint256,uint256,uint256) [PRIVATE]
```slithir
wethToDeposit_1(uint256) := phi(['wethToDeposit_1'])
poolTokensToDeposit_1(uint256) := phi(['poolTokensToDeposit_1', 'maximumPoolTokensToDeposit_1'])
liquidityTokensToMint_1(uint256) := phi(['wethToDeposit_1', 'liquidityTokensToMint_1'])
i_wethToken_6(IERC20) := phi(['i_wethToken_19', 'i_wethToken_17', 'i_wethToken_4', 'i_wethToken_0', 'i_wethToken_32', 'i_wethToken_23', 'i_wethToken_28', 'i_wethToken_7', 'i_wethToken_1'])
i_poolToken_7(IERC20) := phi(['i_poolToken_8', 'i_poolToken_28', 'i_poolToken_23', 'i_poolToken_18', 'i_poolToken_4', 'i_poolToken_0', 'i_poolToken_1', 'i_poolToken_32', 'i_poolToken_20'])
 _mint(msg.sender,liquidityTokensToMint)
INTERNAL_CALL, ERC20._mint(address,uint256)(msg.sender,liquidityTokensToMint_1)
 LiquidityAdded(msg.sender,poolTokensToDeposit,wethToDeposit)
Emit LiquidityAdded(msg.sender,poolTokensToDeposit_1,wethToDeposit_1)
 i_wethToken.safeTransferFrom(msg.sender,address(this),wethToDeposit)
TMP_205 = CONVERT this to address
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransferFrom(IERC20,address,address,uint256), arguments:['i_wethToken_7', 'msg.sender', 'TMP_205', 'wethToDeposit_1'] 
 i_poolToken.safeTransferFrom(msg.sender,address(this),poolTokensToDeposit)
TMP_207 = CONVERT this to address
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransferFrom(IERC20,address,address,uint256), arguments:['i_poolToken_8', 'msg.sender', 'TMP_207', 'poolTokensToDeposit_1']
```
#### TSwapPool.withdraw(uint256,uint256,uint256,uint64) [EXTERNAL]
```slithir
i_wethToken_8(IERC20) := phi(['i_wethToken_19', 'i_wethToken_17', 'i_wethToken_4', 'i_wethToken_0', 'i_wethToken_32', 'i_wethToken_23', 'i_wethToken_28', 'i_wethToken_7', 'i_wethToken_1'])
i_poolToken_9(IERC20) := phi(['i_poolToken_8', 'i_poolToken_28', 'i_poolToken_23', 'i_poolToken_18', 'i_poolToken_4', 'i_poolToken_0', 'i_poolToken_1', 'i_poolToken_32', 'i_poolToken_20'])
 wethToWithdraw = (liquidityTokensToBurn * i_wethToken.balanceOf(address(this))) / totalLiquidityTokenSupply()
TMP_209 = CONVERT this to address
TMP_210(uint256) = HIGH_LEVEL_CALL, dest:i_wethToken_12(IERC20), function:balanceOf, arguments:['TMP_209']  
i_wethToken_13(IERC20) := phi(['i_wethToken_19', 'i_wethToken_17', 'i_wethToken_4', 'i_wethToken_32', 'i_wethToken_23', 'i_wethToken_12', 'i_wethToken_28', 'i_wethToken_7', 'i_wethToken_1'])
i_poolToken_14(IERC20) := phi(['i_poolToken_8', 'i_poolToken_28', 'i_poolToken_23', 'i_poolToken_18', 'i_poolToken_4', 'i_poolToken_1', 'i_poolToken_32', 'i_poolToken_20', 'i_poolToken_13'])
TMP_211(uint256) = liquidityTokensToBurn_1 (c)* TMP_210
TMP_212(uint256) = INTERNAL_CALL, TSwapPool.totalLiquidityTokenSupply()()
TMP_213(uint256) = TMP_211 (c)/ TMP_212
wethToWithdraw_1(uint256) := TMP_213(uint256)
 poolTokensToWithdraw = (liquidityTokensToBurn * i_poolToken.balanceOf(address(this))) / totalLiquidityTokenSupply()
TMP_214 = CONVERT this to address
TMP_215(uint256) = HIGH_LEVEL_CALL, dest:i_poolToken_15(IERC20), function:balanceOf, arguments:['TMP_214']  
i_wethToken_15(IERC20) := phi(['i_wethToken_19', 'i_wethToken_17', 'i_wethToken_4', 'i_wethToken_32', 'i_wethToken_14', 'i_wethToken_23', 'i_wethToken_28', 'i_wethToken_7', 'i_wethToken_1'])
i_poolToken_16(IERC20) := phi(['i_poolToken_8', 'i_poolToken_28', 'i_poolToken_23', 'i_poolToken_18', 'i_poolToken_4', 'i_poolToken_1', 'i_poolToken_32', 'i_poolToken_15', 'i_poolToken_20'])
TMP_216(uint256) = liquidityTokensToBurn_1 (c)* TMP_215
TMP_217(uint256) = INTERNAL_CALL, TSwapPool.totalLiquidityTokenSupply()()
TMP_218(uint256) = TMP_216 (c)/ TMP_217
poolTokensToWithdraw_1(uint256) := TMP_218(uint256)
 wethToWithdraw < minWethToWithdraw
TMP_219(bool) = wethToWithdraw_1 < minWethToWithdraw_1
CONDITION TMP_219
 revert TSwapPool__OutputTooLow(uint256,uint256)(wethToWithdraw,minWethToWithdraw)
TMP_220(None) = SOLIDITY_CALL revert TSwapPool__OutputTooLow(uint256,uint256)(wethToWithdraw_1,minWethToWithdraw_1)
 poolTokensToWithdraw < minPoolTokensToWithdraw
TMP_221(bool) = poolTokensToWithdraw_1 < minPoolTokensToWithdraw_1
CONDITION TMP_221
 revert TSwapPool__OutputTooLow(uint256,uint256)(poolTokensToWithdraw,minPoolTokensToWithdraw)
TMP_222(None) = SOLIDITY_CALL revert TSwapPool__OutputTooLow(uint256,uint256)(poolTokensToWithdraw_1,minPoolTokensToWithdraw_1)
 _burn(msg.sender,liquidityTokensToBurn)
INTERNAL_CALL, ERC20._burn(address,uint256)(msg.sender,liquidityTokensToBurn_1)
 LiquidityRemoved(msg.sender,wethToWithdraw,poolTokensToWithdraw)
Emit LiquidityRemoved(msg.sender,wethToWithdraw_1,poolTokensToWithdraw_1)
 i_wethToken.safeTransfer(msg.sender,wethToWithdraw)
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['i_wethToken_17', 'msg.sender', 'wethToWithdraw_1'] 
 i_poolToken.safeTransfer(msg.sender,poolTokensToWithdraw)
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['i_poolToken_18', 'msg.sender', 'poolTokensToWithdraw_1'] 
 revertIfDeadlinePassed(deadline)
MODIFIER_CALL, TSwapPool.revertIfDeadlinePassed(uint64)(deadline_1)
 revertIfZero(liquidityTokensToBurn)
MODIFIER_CALL, TSwapPool.revertIfZero(uint256)(liquidityTokensToBurn_1)
 revertIfZero(minWethToWithdraw)
MODIFIER_CALL, TSwapPool.revertIfZero(uint256)(minWethToWithdraw_1)
 revertIfZero(minPoolTokensToWithdraw)
MODIFIER_CALL, TSwapPool.revertIfZero(uint256)(minPoolTokensToWithdraw_1)
```
#### TSwapPool.getOutputAmountBasedOnInput(uint256,uint256,uint256) [PUBLIC]
```slithir
inputAmount_1(uint256) := phi(['inputAmount_1'])
inputReserves_1(uint256) := phi(['inputReserves_1', 'TMP_296', 'TMP_291'])
outputReserves_1(uint256) := phi(['TMP_298', 'TMP_293', 'outputReserves_1'])
 inputAmountMinusFee = inputAmount * 997
TMP_231(uint256) = inputAmount_1 (c)* 997
inputAmountMinusFee_1(uint256) := TMP_231(uint256)
 numerator = inputAmountMinusFee * outputReserves
TMP_232(uint256) = inputAmountMinusFee_1 (c)* outputReserves_1
numerator_1(uint256) := TMP_232(uint256)
 denominator = (inputReserves * 1000) + inputAmountMinusFee
TMP_233(uint256) = inputReserves_1 (c)* 1000
TMP_234(uint256) = TMP_233 (c)+ inputAmountMinusFee_1
denominator_1(uint256) := TMP_234(uint256)
 numerator / denominator
TMP_235(uint256) = numerator_1 (c)/ denominator_1
RETURN TMP_235
 revertIfZero(inputAmount)
MODIFIER_CALL, TSwapPool.revertIfZero(uint256)(inputAmount_1)
 revertIfZero(outputReserves)
MODIFIER_CALL, TSwapPool.revertIfZero(uint256)(outputReserves_1)
 outputAmount
```
#### TSwapPool.getInputAmountBasedOnOutput(uint256,uint256,uint256) [PUBLIC]
```slithir
outputAmount_1(uint256) := phi(['outputAmount_1'])
inputReserves_1(uint256) := phi(['inputReserves_1'])
outputReserves_1(uint256) := phi(['outputReserves_1'])
 ((inputReserves * outputAmount) * 10000) / ((outputReserves - outputAmount) * 997)
TMP_238(uint256) = inputReserves_1 (c)* outputAmount_1
TMP_239(uint256) = TMP_238 (c)* 10000
TMP_240(uint256) = outputReserves_1 (c)- outputAmount_1
TMP_241(uint256) = TMP_240 (c)* 997
TMP_242(uint256) = TMP_239 (c)/ TMP_241
RETURN TMP_242
 revertIfZero(outputAmount)
MODIFIER_CALL, TSwapPool.revertIfZero(uint256)(outputAmount_1)
 revertIfZero(outputReserves)
MODIFIER_CALL, TSwapPool.revertIfZero(uint256)(outputReserves_1)
 inputAmount
```
#### TSwapPool.swapExactInput(IERC20,uint256,IERC20,uint256,uint64) [PUBLIC]
```slithir
 inputReserves = inputToken.balanceOf(address(this))
TMP_245 = CONVERT this to address
TMP_246(uint256) = HIGH_LEVEL_CALL, dest:inputToken_1(IERC20), function:balanceOf, arguments:['TMP_245']  
inputReserves_1(uint256) := TMP_246(uint256)
 outputReserves = outputToken.balanceOf(address(this))
TMP_247 = CONVERT this to address
TMP_248(uint256) = HIGH_LEVEL_CALL, dest:outputToken_1(IERC20), function:balanceOf, arguments:['TMP_247']  
outputReserves_1(uint256) := TMP_248(uint256)
 outputAmount = getOutputAmountBasedOnInput(inputAmount,inputReserves,outputReserves)
TMP_249(uint256) = INTERNAL_CALL, TSwapPool.getOutputAmountBasedOnInput(uint256,uint256,uint256)(inputAmount_1,inputReserves_1,outputReserves_1)
outputAmount_1(uint256) := TMP_249(uint256)
 outputAmount < minOutputAmount
TMP_250(bool) = outputAmount_1 < minOutputAmount_1
CONDITION TMP_250
 revert TSwapPool__OutputTooLow(uint256,uint256)(outputAmount,minOutputAmount)
TMP_251(None) = SOLIDITY_CALL revert TSwapPool__OutputTooLow(uint256,uint256)(outputAmount_1,minOutputAmount_1)
 _swap(inputToken,inputAmount,outputToken,outputAmount)
INTERNAL_CALL, TSwapPool._swap(IERC20,uint256,IERC20,uint256)(inputToken_1,inputAmount_1,outputToken_1,outputAmount_1)
 revertIfZero(inputAmount)
MODIFIER_CALL, TSwapPool.revertIfZero(uint256)(inputAmount_1)
 revertIfDeadlinePassed(deadline)
MODIFIER_CALL, TSwapPool.revertIfDeadlinePassed(uint64)(deadline_1)
 output
RETURN output_0
```
#### TSwapPool.swapExactOutput(IERC20,IERC20,uint256,uint64) [PUBLIC]
```slithir
inputToken_1(IERC20) := phi(['i_poolToken_19'])
outputToken_1(IERC20) := phi(['i_wethToken_18'])
outputAmount_1(uint256) := phi(['poolTokenAmount_1'])
deadline_1(uint64) := phi(['TMP_263'])
 inputReserves = inputToken.balanceOf(address(this))
TMP_255 = CONVERT this to address
TMP_256(uint256) = HIGH_LEVEL_CALL, dest:inputToken_1(IERC20), function:balanceOf, arguments:['TMP_255']  
inputReserves_1(uint256) := TMP_256(uint256)
 outputReserves = outputToken.balanceOf(address(this))
TMP_257 = CONVERT this to address
TMP_258(uint256) = HIGH_LEVEL_CALL, dest:outputToken_1(IERC20), function:balanceOf, arguments:['TMP_257']  
outputReserves_1(uint256) := TMP_258(uint256)
 inputAmount = getInputAmountBasedOnOutput(outputAmount,inputReserves,outputReserves)
TMP_259(uint256) = INTERNAL_CALL, TSwapPool.getInputAmountBasedOnOutput(uint256,uint256,uint256)(outputAmount_1,inputReserves_1,outputReserves_1)
inputAmount_1(uint256) := TMP_259(uint256)
 _swap(inputToken,inputAmount,outputToken,outputAmount)
INTERNAL_CALL, TSwapPool._swap(IERC20,uint256,IERC20,uint256)(inputToken_1,inputAmount_1,outputToken_1,outputAmount_1)
 revertIfZero(outputAmount)
MODIFIER_CALL, TSwapPool.revertIfZero(uint256)(outputAmount_1)
 revertIfDeadlinePassed(deadline)
MODIFIER_CALL, TSwapPool.revertIfDeadlinePassed(uint64)(deadline_1)
 inputAmount
RETURN inputAmount_1
```
#### TSwapPool.sellPoolTokens(uint256) [EXTERNAL]
```slithir
i_wethToken_18(IERC20) := phi(['i_wethToken_19', 'i_wethToken_17', 'i_wethToken_4', 'i_wethToken_0', 'i_wethToken_32', 'i_wethToken_23', 'i_wethToken_28', 'i_wethToken_7', 'i_wethToken_1'])
i_poolToken_19(IERC20) := phi(['i_poolToken_8', 'i_poolToken_28', 'i_poolToken_23', 'i_poolToken_18', 'i_poolToken_4', 'i_poolToken_0', 'i_poolToken_1', 'i_poolToken_32', 'i_poolToken_20'])
 swapExactOutput(i_poolToken,i_wethToken,poolTokenAmount,uint64(block.timestamp))
TMP_263 = CONVERT block.timestamp to uint64
TMP_264(uint256) = INTERNAL_CALL, TSwapPool.swapExactOutput(IERC20,IERC20,uint256,uint64)(i_poolToken_19,i_wethToken_18,poolTokenAmount_1,TMP_263)
RETURN TMP_264
 wethAmount
```
#### TSwapPool._swap(IERC20,uint256,IERC20,uint256) [PRIVATE]
```slithir
inputToken_1(IERC20) := phi(['inputToken_1', 'inputToken_1'])
inputAmount_1(uint256) := phi(['inputAmount_1', 'inputAmount_1'])
outputToken_1(IERC20) := phi(['outputToken_1', 'outputToken_1'])
outputAmount_1(uint256) := phi(['outputAmount_1', 'outputAmount_1'])
swap_count_1(uint256) := phi(['swap_count_6', 'swap_count_0'])
SWAP_COUNT_MAX_1(uint256) := phi(['SWAP_COUNT_MAX_0', 'SWAP_COUNT_MAX_3'])
 _isUnknown(inputToken) || _isUnknown(outputToken) || inputToken == outputToken
TMP_265(bool) = INTERNAL_CALL, TSwapPool._isUnknown(IERC20)(inputToken_1)
TMP_266(bool) = INTERNAL_CALL, TSwapPool._isUnknown(IERC20)(outputToken_1)
TMP_267(bool) = TMP_265 || TMP_266
TMP_268(bool) = inputToken_1 == outputToken_1
TMP_269(bool) = TMP_267 || TMP_268
CONDITION TMP_269
 revert TSwapPool__InvalidToken()()
TMP_270(None) = SOLIDITY_CALL revert TSwapPool__InvalidToken()()
 swap_count ++
TMP_271(uint256) := swap_count_3(uint256)
swap_count_4(uint256) = swap_count_3 (c)+ 1
 swap_count >= SWAP_COUNT_MAX
TMP_272(bool) = swap_count_4 >= SWAP_COUNT_MAX_3
CONDITION TMP_272
 swap_count = 0
swap_count_5(uint256) := 0(uint256)
 outputToken.safeTransfer(msg.sender,1_000_000_000_000_000_000)
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['outputToken_1', 'msg.sender', '1000000000000000000'] 
swap_count_6(uint256) := phi(['swap_count_4', 'swap_count_5'])
 Swap(msg.sender,inputToken,inputAmount,outputToken,outputAmount)
Emit Swap(msg.sender,inputToken_1,inputAmount_1,outputToken_1,outputAmount_1)
 inputToken.safeTransferFrom(msg.sender,address(this),inputAmount)
TMP_275 = CONVERT this to address
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransferFrom(IERC20,address,address,uint256), arguments:['inputToken_1', 'msg.sender', 'TMP_275', 'inputAmount_1'] 
 outputToken.safeTransfer(msg.sender,outputAmount)
LIBRARY_CALL, dest:SafeERC20, function:SafeERC20.safeTransfer(IERC20,address,uint256), arguments:['outputToken_1', 'msg.sender', 'outputAmount_1']
```
#### TSwapPool._isUnknown(IERC20) [PRIVATE]
```slithir
token_1(IERC20) := phi(['inputToken_1', 'outputToken_1'])
i_wethToken_20(IERC20) := phi(['i_wethToken_19', 'i_wethToken_17', 'i_wethToken_4', 'i_wethToken_0', 'i_wethToken_32', 'i_wethToken_23', 'i_wethToken_28', 'i_wethToken_7', 'i_wethToken_1'])
i_poolToken_21(IERC20) := phi(['i_poolToken_8', 'i_poolToken_28', 'i_poolToken_23', 'i_poolToken_18', 'i_poolToken_4', 'i_poolToken_0', 'i_poolToken_1', 'i_poolToken_32', 'i_poolToken_20'])
 token != i_wethToken && token != i_poolToken
TMP_278(bool) = token_1 != i_wethToken_20
TMP_279(bool) = token_1 != i_poolToken_21
TMP_280(bool) = TMP_278 && TMP_279
CONDITION TMP_280
 true
RETURN True
 false
RETURN False
```
#### TSwapPool.getPoolTokensToDepositBasedOnWeth(uint256) [PUBLIC]
```slithir
wethToDeposit_1(uint256) := phi(['wethToDeposit_1'])
i_wethToken_21(IERC20) := phi(['i_wethToken_19', 'i_wethToken_17', 'i_wethToken_4', 'i_wethToken_0', 'i_wethToken_32', 'i_wethToken_23', 'i_wethToken_28', 'i_wethToken_7', 'i_wethToken_1'])
i_poolToken_22(IERC20) := phi(['i_poolToken_8', 'i_poolToken_28', 'i_poolToken_23', 'i_poolToken_18', 'i_poolToken_4', 'i_poolToken_0', 'i_poolToken_1', 'i_poolToken_32', 'i_poolToken_20'])
 poolTokenReserves = i_poolToken.balanceOf(address(this))
TMP_281 = CONVERT this to address
TMP_282(uint256) = HIGH_LEVEL_CALL, dest:i_poolToken_22(IERC20), function:balanceOf, arguments:['TMP_281']  
i_wethToken_22(IERC20) := phi(['i_wethToken_19', 'i_wethToken_17', 'i_wethToken_4', 'i_wethToken_21', 'i_wethToken_32', 'i_wethToken_23', 'i_wethToken_28', 'i_wethToken_7', 'i_wethToken_1'])
i_poolToken_23(IERC20) := phi(['i_poolToken_8', 'i_poolToken_28', 'i_poolToken_23', 'i_poolToken_18', 'i_poolToken_4', 'i_poolToken_22', 'i_poolToken_1', 'i_poolToken_32', 'i_poolToken_20'])
poolTokenReserves_1(uint256) := TMP_282(uint256)
 wethReserves = i_wethToken.balanceOf(address(this))
TMP_283 = CONVERT this to address
TMP_284(uint256) = HIGH_LEVEL_CALL, dest:i_wethToken_22(IERC20), function:balanceOf, arguments:['TMP_283']  
i_wethToken_23(IERC20) := phi(['i_wethToken_19', 'i_wethToken_17', 'i_wethToken_4', 'i_wethToken_22', 'i_wethToken_32', 'i_wethToken_23', 'i_wethToken_28', 'i_wethToken_7', 'i_wethToken_1'])
wethReserves_1(uint256) := TMP_284(uint256)
 (wethToDeposit * poolTokenReserves) / wethReserves
TMP_285(uint256) = wethToDeposit_1 (c)* poolTokenReserves_1
TMP_286(uint256) = TMP_285 (c)/ wethReserves_1
RETURN TMP_286
```
#### TSwapPool.totalLiquidityTokenSupply() [PUBLIC]
```slithir
 totalSupply()
TMP_287(uint256) = INTERNAL_CALL, ERC20.totalSupply()()
RETURN TMP_287
```
#### TSwapPool.getPoolToken() [EXTERNAL]
```slithir
i_poolToken_24(IERC20) := phi(['i_poolToken_8', 'i_poolToken_28', 'i_poolToken_23', 'i_poolToken_18', 'i_poolToken_4', 'i_poolToken_0', 'i_poolToken_1', 'i_poolToken_32', 'i_poolToken_20'])
 address(i_poolToken)
TMP_288 = CONVERT i_poolToken_24 to address
RETURN TMP_288
```
#### TSwapPool.getWeth() [EXTERNAL]
```slithir
i_wethToken_24(IERC20) := phi(['i_wethToken_19', 'i_wethToken_17', 'i_wethToken_4', 'i_wethToken_0', 'i_wethToken_32', 'i_wethToken_23', 'i_wethToken_28', 'i_wethToken_7', 'i_wethToken_1'])
 address(i_wethToken)
TMP_289 = CONVERT i_wethToken_24 to address
RETURN TMP_289
```
#### TSwapPool.getMinimumWethDepositAmount() [EXTERNAL]
```slithir
MINIMUM_WETH_LIQUIDITY_3(uint256) := phi(['MINIMUM_WETH_LIQUIDITY_2', 'MINIMUM_WETH_LIQUIDITY_0'])
 MINIMUM_WETH_LIQUIDITY
RETURN MINIMUM_WETH_LIQUIDITY_3
```
#### TSwapPool.getPriceOfOneWethInPoolTokens() [EXTERNAL]
```slithir
i_wethToken_25(IERC20) := phi(['i_wethToken_19', 'i_wethToken_17', 'i_wethToken_4', 'i_wethToken_0', 'i_wethToken_32', 'i_wethToken_23', 'i_wethToken_28', 'i_wethToken_7', 'i_wethToken_1'])
i_poolToken_25(IERC20) := phi(['i_poolToken_8', 'i_poolToken_28', 'i_poolToken_23', 'i_poolToken_18', 'i_poolToken_4', 'i_poolToken_0', 'i_poolToken_1', 'i_poolToken_32', 'i_poolToken_20'])
 getOutputAmountBasedOnInput(1e18,i_wethToken.balanceOf(address(this)),i_poolToken.balanceOf(address(this)))
TMP_290 = CONVERT this to address
TMP_291(uint256) = HIGH_LEVEL_CALL, dest:i_wethToken_25(IERC20), function:balanceOf, arguments:['TMP_290']  
i_wethToken_26(IERC20) := phi(['i_wethToken_19', 'i_wethToken_17', 'i_wethToken_4', 'i_wethToken_25', 'i_wethToken_32', 'i_wethToken_23', 'i_wethToken_28', 'i_wethToken_7', 'i_wethToken_1'])
i_poolToken_26(IERC20) := phi(['i_poolToken_8', 'i_poolToken_28', 'i_poolToken_25', 'i_poolToken_23', 'i_poolToken_18', 'i_poolToken_4', 'i_poolToken_1', 'i_poolToken_32', 'i_poolToken_20'])
TMP_292 = CONVERT this to address
TMP_293(uint256) = HIGH_LEVEL_CALL, dest:i_poolToken_26(IERC20), function:balanceOf, arguments:['TMP_292']  
i_wethToken_27(IERC20) := phi(['i_wethToken_19', 'i_wethToken_17', 'i_wethToken_4', 'i_wethToken_26', 'i_wethToken_32', 'i_wethToken_23', 'i_wethToken_28', 'i_wethToken_7', 'i_wethToken_1'])
i_poolToken_27(IERC20) := phi(['i_poolToken_8', 'i_poolToken_28', 'i_poolToken_26', 'i_poolToken_23', 'i_poolToken_18', 'i_poolToken_4', 'i_poolToken_1', 'i_poolToken_32', 'i_poolToken_20'])
TMP_294(uint256) = INTERNAL_CALL, TSwapPool.getOutputAmountBasedOnInput(uint256,uint256,uint256)(1000000000000000000,TMP_291,TMP_293)
RETURN TMP_294
```
#### TSwapPool.getPriceOfOnePoolTokenInWeth() [EXTERNAL]
```slithir
i_wethToken_29(IERC20) := phi(['i_wethToken_19', 'i_wethToken_17', 'i_wethToken_4', 'i_wethToken_0', 'i_wethToken_32', 'i_wethToken_23', 'i_wethToken_28', 'i_wethToken_7', 'i_wethToken_1'])
i_poolToken_29(IERC20) := phi(['i_poolToken_8', 'i_poolToken_28', 'i_poolToken_23', 'i_poolToken_18', 'i_poolToken_4', 'i_poolToken_0', 'i_poolToken_1', 'i_poolToken_32', 'i_poolToken_20'])
 getOutputAmountBasedOnInput(1e18,i_poolToken.balanceOf(address(this)),i_wethToken.balanceOf(address(this)))
TMP_295 = CONVERT this to address
TMP_296(uint256) = HIGH_LEVEL_CALL, dest:i_poolToken_29(IERC20), function:balanceOf, arguments:['TMP_295']  
i_wethToken_30(IERC20) := phi(['i_wethToken_19', 'i_wethToken_17', 'i_wethToken_4', 'i_wethToken_32', 'i_wethToken_23', 'i_wethToken_29', 'i_wethToken_28', 'i_wethToken_7', 'i_wethToken_1'])
i_poolToken_30(IERC20) := phi(['i_poolToken_8', 'i_poolToken_28', 'i_poolToken_23', 'i_poolToken_29', 'i_poolToken_18', 'i_poolToken_4', 'i_poolToken_1', 'i_poolToken_32', 'i_poolToken_20'])
TMP_297 = CONVERT this to address
TMP_298(uint256) = HIGH_LEVEL_CALL, dest:i_wethToken_30(IERC20), function:balanceOf, arguments:['TMP_297']  
i_wethToken_31(IERC20) := phi(['i_wethToken_30', 'i_wethToken_19', 'i_wethToken_17', 'i_wethToken_4', 'i_wethToken_32', 'i_wethToken_23', 'i_wethToken_28', 'i_wethToken_7', 'i_wethToken_1'])
i_poolToken_31(IERC20) := phi(['i_poolToken_8', 'i_poolToken_28', 'i_poolToken_23', 'i_poolToken_18', 'i_poolToken_4', 'i_poolToken_1', 'i_poolToken_32', 'i_poolToken_20', 'i_poolToken_30'])
TMP_299(uint256) = INTERNAL_CALL, TSwapPool.getOutputAmountBasedOnInput(uint256,uint256,uint256)(1000000000000000000,TMP_296,TMP_298)
RETURN TMP_299
```
#### TSwapPool.slitherConstructorVariables() [INTERNAL]
```slithir
 swap_count = 0
```
#### TSwapPool.slitherConstructorConstantVariables() [INTERNAL]
```slithir
Expression: MINIMUM_WETH_LIQUIDITY = 1_000_000_000
Expression: SWAP_COUNT_MAX = 10
IRs:
deadline_1(uint64) := phi(['deadline_1', 'deadline_1', 'deadline_1'])
Expression: deadline < uint64(block.timestamp)
IRs:
TMP_300 = CONVERT block.timestamp to uint64
TMP_301(bool) = deadline_1 < TMP_300
CONDITION TMP_301
Expression: revert TSwapPool__DeadlineHasPassed(uint64)(deadline)
IRs:
TMP_302(None) = SOLIDITY_CALL revert TSwapPool__DeadlineHasPassed(uint64)(deadline_1)
IRs:
amount_1(uint256) := phi(['minWethToWithdraw_1', 'inputAmount_1', 'wethToDeposit_1', 'outputAmount_1', 'minPoolTokensToWithdraw_1', 'outputReserves_1', 'liquidityTokensToBurn_1', 'outputReserves_1', 'inputAmount_1', 'outputAmount_1'])
Expression: amount == 0
IRs:
TMP_303(bool) = amount_1 == 0
CONDITION TMP_303
Expression: revert TSwapPool__MustBeMoreThanZero()()
IRs:
TMP_304(None) = SOLIDITY_CALL revert TSwapPool__MustBeMoreThanZero()()
```
#### SafeERC20.safeTransferFrom(IERC20,address,address,uint256) [INTERNAL]
```slithir
 _callOptionalReturn(token,abi.encodeCall(token.transferFrom,(from,to,value)))
REF_11(transferFrom) -> token_1.transferFrom
TMP_56(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_11,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0x107000320>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0x1070007d0>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0x1070008c0>])
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_56)
```
#### SafeERC20.safeTransfer(IERC20,address,uint256) [INTERNAL]
```slithir
 _callOptionalReturn(token,abi.encodeCall(token.transfer,(to,value)))
REF_9(transfer) -> token_1.transfer
TMP_54(bytes) = SOLIDITY_CALL abi.encodeCall()(REF_9,[<slither.slithir.variables.local_variable.LocalIRVariable object at 0x107000050>, <slither.slithir.variables.local_variable.LocalIRVariable object at 0x107000230>])
INTERNAL_CALL, SafeERC20._callOptionalReturn(IERC20,bytes)(token_1,TMP_54)
```
#### SafeERC20._callOptionalReturn(IERC20,bytes) [PRIVATE]
```slithir
token_1(IERC20) := phi(['token_1', 'token_1', 'token_1'])
data_1(bytes) := phi(['TMP_71', 'approvalCall_1', 'TMP_54', 'TMP_56'])
 returndata = address(token).functionCall(data)
TMP_74 = CONVERT token_1 to address
TMP_75(bytes) = LIBRARY_CALL, dest:Address, function:Address.functionCall(address,bytes), arguments:['TMP_74', 'data_1'] 
returndata_1(bytes) := TMP_75(bytes)
 returndata.length != 0 && ! abi.decode(returndata,(bool))
REF_19 -> LENGTH returndata_1
TMP_76(bool) = REF_19 != 0
TMP_77(bool) = SOLIDITY_CALL abi.decode()(returndata_1,bool)
TMP_78 = UnaryType.BANG TMP_77 
TMP_79(bool) = TMP_76 && TMP_78
CONDITION TMP_79
 revert SafeERC20FailedOperation(address)(address(token))
TMP_80 = CONVERT token_1 to address
TMP_81(None) = SOLIDITY_CALL revert SafeERC20FailedOperation(address)(TMP_80)
```
#### Address.functionCall(address,bytes) [INTERNAL]
```slithir
 functionCallWithValue(target,data,0)
TMP_98(bytes) = INTERNAL_CALL, Address.functionCallWithValue(address,bytes,uint256)(target_1,data_1,0)
RETURN TMP_98
```
