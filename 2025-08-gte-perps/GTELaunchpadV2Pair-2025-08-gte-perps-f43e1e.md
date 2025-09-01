

### Storage layout (GTELaunchpadV2Pair) 

```text
launchpadLp address
launchpadFeeDistributor address
factory address
token0 address
token1 address
reserve0 uint112
reserve1 uint112
blockTimestampLast uint32
price0CumulativeLast uint256
price1CumulativeLast uint256
kLast uint256
accruedLaunchpadFee0 uint112
accruedLaunchpadFee1 uint112
rewardsPoolActive uint256
unlocked uint256

```




#### GTELaunchpadV2Pair._distributeLaunchpadFees(uint112,uint112) [INTERNAL]
```slithir
fee0_1(uint112) := phi(['totalLaunchpadFee0_1'])
fee1_1(uint112) := phi(['totalLaunchpadFee1_1'])
launchpadFeeDistributor_12(address) := phi(['launchpadFeeDistributor_1', 'launchpadFeeDistributor_11', 'launchpadFeeDistributor_0'])
token0_16(address) := phi(['token0_23', 'token0_15', 'token0_18', 'token0_0', 'token0_9', 'token0_12', 'token0_1', 'token0_5'])
token1_17(address) := phi(['token1_16', 'token1_24', 'token1_10', 'token1_0', 'token1_13', 'token1_19', 'token1_1', 'token1_5'])
 (fee0 | fee1) > 0
TMP_3830(uint112) = fee0_1 | fee1_1
TMP_3831(bool) = TMP_3830 > 0
CONDITION TMP_3831
 _token0 = token0
_token0_1(address) := token0_16(address)
 _token1 = token1
_token1_1(address) := token1_17(address)
 distributor = launchpadFeeDistributor
distributor_1(address) := launchpadFeeDistributor_12(address)
 fee0 > 0
TMP_3832(bool) = fee0_1 > 0
CONDITION TMP_3832
 _safeApprove(_token0,distributor,uint256(fee0))
TMP_3833 = CONVERT fee0_1 to uint256
INTERNAL_CALL, GTELaunchpadV2Pair._safeApprove(address,address,uint256)(_token0_1,distributor_1,TMP_3833)
 fee1 > 0
TMP_3835(bool) = fee1_1 > 0
CONDITION TMP_3835
 _safeApprove(_token1,distributor,uint256(fee1))
TMP_3836 = CONVERT fee1_1 to uint256
INTERNAL_CALL, GTELaunchpadV2Pair._safeApprove(address,address,uint256)(_token1_1,distributor_1,TMP_3836)
 IDistributor(distributor).addRewards(_token0,_token1,uint128(fee0),uint128(fee1))
TMP_3838 = CONVERT distributor_1 to IDistributor
TMP_3839 = CONVERT fee0_1 to uint128
TMP_3840 = CONVERT fee1_1 to uint128
HIGH_LEVEL_CALL, dest:TMP_3838(IDistributor), function:addRewards, arguments:['_token0_1', '_token1_1', 'TMP_3839', 'TMP_3840']  
 LaunchpadFeesCollected(fee0,fee1)
Emit LaunchpadFeesCollected(fee0_1,fee1_1)
```
#### GTELaunchpadV2Pair._getLaunchpadFees(uint256,uint256) [INTERNAL]
```slithir
amount0In_1(uint256) := phi(['amount0In_3'])
amount1In_1(uint256) := phi(['amount1In_3'])
REWARDS_FEE_SHARE_1(uint256) := phi(['REWARDS_FEE_SHARE_3', 'REWARDS_FEE_SHARE_0'])
MINIMUM_LIQUIDITY_8(uint256) := phi(['MINIMUM_LIQUIDITY_6', 'MINIMUM_LIQUIDITY_0', 'MINIMUM_LIQUIDITY_7', 'MINIMUM_LIQUIDITY_10'])
launchpadLp_2(address) := phi(['launchpadLp_1', 'launchpadLp_0', 'launchpadLp_4'])
 totalLpBal = this.totalSupply()
TMP_3815(uint256) = HIGH_LEVEL_CALL, dest:this(address), function:totalSupply, arguments:[]  
REWARDS_FEE_SHARE_2(uint256) := phi(['REWARDS_FEE_SHARE_3', 'REWARDS_FEE_SHARE_1'])
MINIMUM_LIQUIDITY_9(uint256) := phi(['MINIMUM_LIQUIDITY_6', 'MINIMUM_LIQUIDITY_7', 'MINIMUM_LIQUIDITY_8', 'MINIMUM_LIQUIDITY_10'])
launchpadLp_3(address) := phi(['launchpadLp_1', 'launchpadLp_4', 'launchpadLp_2'])
totalLpBal_1(uint256) := TMP_3815(uint256)
 launchpadLpBal = this.balanceOf(launchpadLp) + MINIMUM_LIQUIDITY
TMP_3816(uint256) = HIGH_LEVEL_CALL, dest:this(address), function:balanceOf, arguments:['launchpadLp_3']  
REWARDS_FEE_SHARE_3(uint256) := phi(['REWARDS_FEE_SHARE_3', 'REWARDS_FEE_SHARE_2'])
MINIMUM_LIQUIDITY_10(uint256) := phi(['MINIMUM_LIQUIDITY_9', 'MINIMUM_LIQUIDITY_6', 'MINIMUM_LIQUIDITY_7', 'MINIMUM_LIQUIDITY_10'])
launchpadLp_4(address) := phi(['launchpadLp_1', 'launchpadLp_3', 'launchpadLp_4'])
TMP_3817(uint256) = TMP_3816 (c)+ MINIMUM_LIQUIDITY_10
launchpadLpBal_1(uint256) := TMP_3817(uint256)
 amount0In > 0
TMP_3818(bool) = amount0In_1 > 0
CONDITION TMP_3818
 fee0 = uint112(amount0In.mul(REWARDS_FEE_SHARE).mul(launchpadLpBal) / (totalLpBal * 1000))
TMP_3819(uint256) = LIBRARY_CALL, dest:SafeMath, function:SafeMath.mul(uint256,uint256), arguments:['amount0In_1', 'REWARDS_FEE_SHARE_3'] 
TMP_3820(uint256) = LIBRARY_CALL, dest:SafeMath, function:SafeMath.mul(uint256,uint256), arguments:['TMP_3819', 'launchpadLpBal_1'] 
TMP_3821(uint256) = totalLpBal_1 (c)* 1000
TMP_3822(uint256) = TMP_3820 (c)/ TMP_3821
TMP_3823 = CONVERT TMP_3822 to uint112
fee0_1(uint112) := TMP_3823(uint112)
fee0_2(uint112) := phi(['fee0_0', 'fee0_1'])
 amount1In > 0
TMP_3824(bool) = amount1In_1 > 0
CONDITION TMP_3824
 fee1 = uint112(amount1In.mul(REWARDS_FEE_SHARE).mul(launchpadLpBal) / (totalLpBal * 1000))
TMP_3825(uint256) = LIBRARY_CALL, dest:SafeMath, function:SafeMath.mul(uint256,uint256), arguments:['amount1In_1', 'REWARDS_FEE_SHARE_3'] 
TMP_3826(uint256) = LIBRARY_CALL, dest:SafeMath, function:SafeMath.mul(uint256,uint256), arguments:['TMP_3825', 'launchpadLpBal_1'] 
TMP_3827(uint256) = totalLpBal_1 (c)* 1000
TMP_3828(uint256) = TMP_3826 (c)/ TMP_3827
TMP_3829 = CONVERT TMP_3828 to uint112
fee1_1(uint112) := TMP_3829(uint112)
fee1_2(uint112) := phi(['fee1_0', 'fee1_1'])
 (fee0,fee1)
RETURN fee0_2,fee1_2
 (fee0,fee1)
```
#### GTELaunchpadV2Pair._mintFee(uint112,uint112) [PRIVATE]
```slithir
_reserve0_1(uint112) := phi(['_reserve0_1', '_reserve0_1'])
_reserve1_1(uint112) := phi(['_reserve1_1', '_reserve1_1'])
totalSupply_5(uint256) := phi(['totalSupply_4', 'totalSupply_0', 'totalSupply_6', 'totalSupply_2', 'totalSupply_12', 'totalSupply_18'])
factory_3(address) := phi(['factory_4', 'factory_0', 'factory_1'])
kLast_1(uint256) := phi(['kLast_0', 'kLast_4', 'kLast_5', 'kLast_2', 'kLast_3'])
 feeTo = IUniswapV2Factory(factory).feeTo()
TMP_3675 = CONVERT factory_3 to IUniswapV2Factory
TMP_3676(address) = HIGH_LEVEL_CALL, dest:TMP_3675(IUniswapV2Factory), function:feeTo, arguments:[]  
totalSupply_6(uint256) := phi(['totalSupply_4', 'totalSupply_6', 'totalSupply_2', 'totalSupply_12', 'totalSupply_18', 'totalSupply_5'])
factory_4(address) := phi(['factory_4', 'factory_3', 'factory_1'])
kLast_2(uint256) := phi(['kLast_1', 'kLast_4', 'kLast_5', 'kLast_2', 'kLast_3'])
feeTo_1(address) := TMP_3676(address)
 feeOn = feeTo != address(0)
TMP_3677 = CONVERT 0 to address
TMP_3678(bool) = feeTo_1 != TMP_3677
feeOn_1(bool) := TMP_3678(bool)
 _kLast = kLast
_kLast_1(uint256) := kLast_2(uint256)
 feeOn
CONDITION feeOn_1
 _kLast != 0
TMP_3679(bool) = _kLast_1 != 0
CONDITION TMP_3679
 rootK = Math.sqrt(uint256(_reserve0).mul(_reserve1))
TMP_3680 = CONVERT _reserve0_1 to uint256
TMP_3681(uint256) = LIBRARY_CALL, dest:SafeMath, function:SafeMath.mul(uint256,uint256), arguments:['TMP_3680', '_reserve1_1'] 
TMP_3682(uint256) = LIBRARY_CALL, dest:Math, function:Math.sqrt(uint256), arguments:['TMP_3681'] 
rootK_1(uint256) := TMP_3682(uint256)
 rootKLast = Math.sqrt(_kLast)
TMP_3683(uint256) = LIBRARY_CALL, dest:Math, function:Math.sqrt(uint256), arguments:['_kLast_1'] 
rootKLast_1(uint256) := TMP_3683(uint256)
 rootK > rootKLast
TMP_3684(bool) = rootK_1 > rootKLast_1
CONDITION TMP_3684
 numerator = totalSupply.mul(rootK.sub(rootKLast))
TMP_3685(uint256) = LIBRARY_CALL, dest:SafeMath, function:SafeMath.sub(uint256,uint256), arguments:['rootK_1', 'rootKLast_1'] 
TMP_3686(uint256) = LIBRARY_CALL, dest:SafeMath, function:SafeMath.mul(uint256,uint256), arguments:['totalSupply_6', 'TMP_3685'] 
numerator_1(uint256) := TMP_3686(uint256)
 denominator = rootK.mul(5).add(rootKLast)
TMP_3687(uint256) = LIBRARY_CALL, dest:SafeMath, function:SafeMath.mul(uint256,uint256), arguments:['rootK_1', '5'] 
TMP_3688(uint256) = LIBRARY_CALL, dest:SafeMath, function:SafeMath.add(uint256,uint256), arguments:['TMP_3687', 'rootKLast_1'] 
denominator_1(uint256) := TMP_3688(uint256)
 liquidity = numerator / denominator
TMP_3689(uint256) = numerator_1 (c)/ denominator_1
liquidity_1(uint256) := TMP_3689(uint256)
 liquidity > 0
TMP_3690(bool) = liquidity_1 > 0
CONDITION TMP_3690
 _mint(feeTo,liquidity)
INTERNAL_CALL, UniswapV2ERC20._mint(address,uint256)(feeTo_1,liquidity_1)
 _kLast != 0
TMP_3692(bool) = _kLast_1 != 0
CONDITION TMP_3692
 kLast = 0
kLast_3(uint256) := 0(uint256)
 feeOn
RETURN feeOn_1
```
#### GTELaunchpadV2Pair._safeApprove(address,address,uint256) [PRIVATE]
```slithir
token_1(address) := phi(['_token1_1', '_token0_1'])
to_1(address) := phi(['distributor_1'])
value_1(uint256) := phi(['TMP_3836', 'TMP_3833'])
APPROVE_SELECTOR_1(bytes4) := phi(['APPROVE_SELECTOR_0', 'APPROVE_SELECTOR_2'])
 (success,data) = token.call(abi.encodeWithSelector(APPROVE_SELECTOR,to,value))
TMP_3610(bytes) = SOLIDITY_CALL abi.encodeWithSelector()(APPROVE_SELECTOR_1,to_1,value_1)
TUPLE_34(bool,bytes) = LOW_LEVEL_CALL, dest:token_1, function:call, arguments:['TMP_3610']  
APPROVE_SELECTOR_2(bytes4) := phi(['APPROVE_SELECTOR_1', 'APPROVE_SELECTOR_2'])
success_1(bool)= UNPACK TUPLE_34 index: 0 
data_1(bytes)= UNPACK TUPLE_34 index: 1 
 ! success || ! (data.length == 0 || abi.decode(data,(bool)))
TMP_3611 = UnaryType.BANG success_1 
REF_1336 -> LENGTH data_1
TMP_3612(bool) = REF_1336 == 0
TMP_3613(bool) = SOLIDITY_CALL abi.decode()(data_1,bool)
TMP_3614(bool) = TMP_3612 || TMP_3613
TMP_3615 = UnaryType.BANG TMP_3614 
TMP_3616(bool) = TMP_3611 || TMP_3615
CONDITION TMP_3616
 revert(string)(UniswapV2: APPROVAL_FAILED)
TMP_3617(None) = SOLIDITY_CALL revert(string)(UniswapV2: APPROVAL_FAILED)
```
#### GTELaunchpadV2Pair._safeTransfer(address,address,uint256) [PRIVATE]
```slithir
token_1(address) := phi(['_token1_1', '_token1_1', '_token1_1', '_token0_1', '_token0_1', '_token0_1'])
to_1(address) := phi(['to_1', 'to_1', 'to_1'])
value_1(uint256) := phi(['TMP_3853', 'amount0_1', 'amount0Out_1', 'amount1_1', 'TMP_3847', 'amount1Out_1'])
TRANSFER_SELECTOR_1(bytes4) := phi(['TRANSFER_SELECTOR_0', 'TRANSFER_SELECTOR_2'])
 (success,data) = token.call(abi.encodeWithSelector(TRANSFER_SELECTOR,to,value))
TMP_3602(bytes) = SOLIDITY_CALL abi.encodeWithSelector()(TRANSFER_SELECTOR_1,to_1,value_1)
TUPLE_33(bool,bytes) = LOW_LEVEL_CALL, dest:token_1, function:call, arguments:['TMP_3602']  
TRANSFER_SELECTOR_2(bytes4) := phi(['TRANSFER_SELECTOR_1', 'TRANSFER_SELECTOR_2'])
success_1(bool)= UNPACK TUPLE_33 index: 0 
data_1(bytes)= UNPACK TUPLE_33 index: 1 
 ! success || ! (data.length == 0 || abi.decode(data,(bool)))
TMP_3603 = UnaryType.BANG success_1 
REF_1332 -> LENGTH data_1
TMP_3604(bool) = REF_1332 == 0
TMP_3605(bool) = SOLIDITY_CALL abi.decode()(data_1,bool)
TMP_3606(bool) = TMP_3604 || TMP_3605
TMP_3607 = UnaryType.BANG TMP_3606 
TMP_3608(bool) = TMP_3603 || TMP_3607
CONDITION TMP_3608
 revert(string)(UniswapV2: TRANSFER_FAILED)
TMP_3609(None) = SOLIDITY_CALL revert(string)(UniswapV2: TRANSFER_FAILED)
```
#### GTELaunchpadV2Pair._update(uint256,uint256,uint112,uint112,uint112,uint112) [PRIVATE]
```slithir
balance0_1(uint256) := phi(['TMP_3624', 'balance0_1', 'TMP_3858', 'balance0_2', 'balance0_1'])
balance1_1(uint256) := phi(['balance1_1', 'TMP_3627', 'TMP_3861', 'balance1_2', 'balance1_1'])
_reserve0_1(uint112) := phi(['_reserve0_1', 'reserve0_35', '_reserve0_1', '_reserve0_1', 'reserve0_4'])
_reserve1_1(uint112) := phi(['reserve1_4', '_reserve1_1', 'reserve1_37', '_reserve1_1', '_reserve1_1'])
newLaunchpadFee0_1(uint112) := phi(['TMP_3749', 'TMP_3628', 'launchpadFee0_3', 'TMP_3862', 'TMP_3716'])
newLaunchpadFee1_1(uint112) := phi(['TMP_3717', 'TMP_3750', 'TMP_3629', 'launchpadFee1_3', 'TMP_3863'])
launchpadFeeDistributor_3(address) := phi(['launchpadFeeDistributor_1', 'launchpadFeeDistributor_11', 'launchpadFeeDistributor_0'])
blockTimestampLast_3(uint32) := phi(['blockTimestampLast_4', 'blockTimestampLast_0'])
price0CumulativeLast_1(uint256) := phi(['price0CumulativeLast_2', 'price0CumulativeLast_0'])
price1CumulativeLast_1(uint256) := phi(['price1CumulativeLast_2', 'price1CumulativeLast_0'])
accruedLaunchpadFee0_4(uint112) := phi(['accruedLaunchpadFee0_0', 'accruedLaunchpadFee0_10', 'accruedLaunchpadFee0_3', 'accruedLaunchpadFee0_6', 'accruedLaunchpadFee0_5'])
accruedLaunchpadFee1_4(uint112) := phi(['accruedLaunchpadFee1_3', 'accruedLaunchpadFee1_6', 'accruedLaunchpadFee1_0', 'accruedLaunchpadFee1_5', 'accruedLaunchpadFee1_12'])
 balance0 > type()(uint112).max || balance1 > type()(uint112).max
TMP_3633(uint112) := 5192296858534827628530496329220095(uint112)
TMP_3634(bool) = balance0_1 > TMP_3633
TMP_3636(uint112) := 5192296858534827628530496329220095(uint112)
TMP_3637(bool) = balance1_1 > TMP_3636
TMP_3638(bool) = TMP_3634 || TMP_3637
CONDITION TMP_3638
 revert(string)(UniswapV2: OVERFLOW)
TMP_3639(None) = SOLIDITY_CALL revert(string)(UniswapV2: OVERFLOW)
 totalLaunchpadFee0 = accruedLaunchpadFee0 + newLaunchpadFee0
TMP_3640(uint112) = accruedLaunchpadFee0_4 (c)+ newLaunchpadFee0_1
totalLaunchpadFee0_1(uint112) := TMP_3640(uint112)
 totalLaunchpadFee1 = accruedLaunchpadFee1 + newLaunchpadFee1
TMP_3641(uint112) = accruedLaunchpadFee1_4 (c)+ newLaunchpadFee1_1
totalLaunchpadFee1_1(uint112) := TMP_3641(uint112)
 blockTimestamp = uint32(block.timestamp % 2 ** 32)
TMP_3642(uint256) = 2 (c)** 32
TMP_3643(uint256) = block.timestamp % TMP_3642
TMP_3644 = CONVERT TMP_3643 to uint32
blockTimestamp_1(uint32) := TMP_3644(uint32)
 timeElapsed = blockTimestamp - blockTimestampLast
TMP_3645(uint32) = blockTimestamp_1 (c)- blockTimestampLast_3
timeElapsed_1(uint32) := TMP_3645(uint32)
 timeElapsed > 0 && _reserve0 != 0 && _reserve1 != 0
TMP_3646(bool) = timeElapsed_1 > 0
TMP_3647(bool) = _reserve0_1 != 0
TMP_3648(bool) = TMP_3646 && TMP_3647
TMP_3649(bool) = _reserve1_1 != 0
TMP_3650(bool) = TMP_3648 && TMP_3649
CONDITION TMP_3650
 price0CumulativeLast += uint256(UQ112x112.encode(_reserve1).uqdiv(_reserve0)) * timeElapsed
TMP_3651(uint224) = LIBRARY_CALL, dest:UQ112x112, function:UQ112x112.encode(uint112), arguments:['_reserve1_1'] 
TMP_3652(uint224) = LIBRARY_CALL, dest:UQ112x112, function:UQ112x112.uqdiv(uint224,uint112), arguments:['TMP_3651', '_reserve0_1'] 
TMP_3653 = CONVERT TMP_3652 to uint256
TMP_3654(uint256) = TMP_3653 (c)* timeElapsed_1
price0CumulativeLast_2(uint256) = price0CumulativeLast_1 (c)+ TMP_3654
 price1CumulativeLast += uint256(UQ112x112.encode(_reserve0).uqdiv(_reserve1)) * timeElapsed
TMP_3655(uint224) = LIBRARY_CALL, dest:UQ112x112, function:UQ112x112.encode(uint112), arguments:['_reserve0_1'] 
TMP_3656(uint224) = LIBRARY_CALL, dest:UQ112x112, function:UQ112x112.uqdiv(uint224,uint112), arguments:['TMP_3655', '_reserve1_1'] 
TMP_3657 = CONVERT TMP_3656 to uint256
TMP_3658(uint256) = TMP_3657 (c)* timeElapsed_1
price1CumulativeLast_2(uint256) = price1CumulativeLast_1 (c)+ TMP_3658
 launchpadFeeDistributor > address(0)
TMP_3659 = CONVERT 0 to address
TMP_3660(bool) = launchpadFeeDistributor_3 > TMP_3659
CONDITION TMP_3660
 totalLaunchpadFee0 | totalLaunchpadFee1 > 0
TMP_3661(uint112) = totalLaunchpadFee0_1 | totalLaunchpadFee1_1
TMP_3662(bool) = TMP_3661 > 0
CONDITION TMP_3662
 delete accruedLaunchpadFee0
accruedLaunchpadFee0_5 = delete accruedLaunchpadFee0_4 
 delete accruedLaunchpadFee1
accruedLaunchpadFee1_5 = delete accruedLaunchpadFee1_4 
 _distributeLaunchpadFees(totalLaunchpadFee0,totalLaunchpadFee1)
INTERNAL_CALL, GTELaunchpadV2Pair._distributeLaunchpadFees(uint112,uint112)(totalLaunchpadFee0_1,totalLaunchpadFee1_1)
 launchpadFeeDistributor > address(0) && newLaunchpadFee0 | newLaunchpadFee1 > 0
TMP_3664 = CONVERT 0 to address
TMP_3665(bool) = launchpadFeeDistributor_3 > TMP_3664
TMP_3666(uint112) = newLaunchpadFee0_1 | newLaunchpadFee1_1
TMP_3667(bool) = TMP_3666 > 0
TMP_3668(bool) = TMP_3665 && TMP_3667
CONDITION TMP_3668
 accruedLaunchpadFee0 = totalLaunchpadFee0
accruedLaunchpadFee0_6(uint112) := totalLaunchpadFee0_1(uint112)
 accruedLaunchpadFee1 = totalLaunchpadFee1
accruedLaunchpadFee1_6(uint112) := totalLaunchpadFee1_1(uint112)
 LaunchpadFeesAccrued(newLaunchpadFee0,newLaunchpadFee1)
Emit LaunchpadFeesAccrued(newLaunchpadFee0_1,newLaunchpadFee1_1)
 reserve0 = _reserve0 = uint112(balance0) - totalLaunchpadFee0
TMP_3670 = CONVERT balance0_1 to uint112
TMP_3671(uint112) = TMP_3670 (c)- totalLaunchpadFee0_1
_reserve0_2(uint112) := TMP_3671(uint112)
reserve0_6(uint112) := _reserve0_2(uint112)
 reserve1 = _reserve1 = uint112(balance1) - totalLaunchpadFee1
TMP_3672 = CONVERT balance1_1 to uint112
TMP_3673(uint112) = TMP_3672 (c)- totalLaunchpadFee1_1
_reserve1_2(uint112) := TMP_3673(uint112)
reserve1_6(uint112) := _reserve1_2(uint112)
 blockTimestampLast = blockTimestamp
blockTimestampLast_4(uint32) := blockTimestamp_1(uint32)
 Sync(_reserve0,_reserve1)
Emit Sync(_reserve0_2,_reserve1_2)
```
#### GTELaunchpadV2Pair.burn(address) [EXTERNAL]
```slithir
totalSupply_13(uint256) := phi(['totalSupply_4', 'totalSupply_0', 'totalSupply_6', 'totalSupply_2', 'totalSupply_12', 'totalSupply_18'])
balanceOf_8(mapping(address => uint256)) := phi(['balanceOf_7', 'balanceOf_0', 'balanceOf_2', 'balanceOf_12', 'balanceOf_4'])
token0_10(address) := phi(['token0_23', 'token0_15', 'token0_18', 'token0_0', 'token0_9', 'token0_12', 'token0_1', 'token0_5'])
token1_11(address) := phi(['token1_16', 'token1_24', 'token1_10', 'token1_0', 'token1_13', 'token1_19', 'token1_1', 'token1_5'])
reserve0_16(uint112) := phi(['reserve0_31', 'reserve0_5', 'reserve0_6', 'reserve0_15', 'reserve0_36', 'reserve0_27', 'reserve0_0'])
reserve1_16(uint112) := phi(['reserve1_6', 'reserve1_33', 'reserve1_5', 'reserve1_15', 'reserve1_38', 'reserve1_27', 'reserve1_0'])
 (_reserve0,_reserve1,None) = getReserves()
TUPLE_36(uint112,uint112,uint32) = INTERNAL_CALL, GTELaunchpadV2Pair.getReserves()()
_reserve0_1(uint112)= UNPACK TUPLE_36 index: 0 
_reserve1_1(uint112)= UNPACK TUPLE_36 index: 1 
 _token0 = token0
_token0_1(address) := token0_12(address)
 _token1 = token1
_token1_1(address) := token1_13(address)
 balance0 = IERC20(_token0).balanceOf(address(this))
TMP_3723 = CONVERT _token0_1 to IERC20
TMP_3724 = CONVERT this to address
TMP_3725(uint256) = HIGH_LEVEL_CALL, dest:TMP_3723(IERC20), function:balanceOf, arguments:['TMP_3724']  
totalSupply_16(uint256) := phi(['totalSupply_15', 'totalSupply_4', 'totalSupply_6', 'totalSupply_2', 'totalSupply_12', 'totalSupply_18'])
balanceOf_11(mapping(address => uint256)) := phi(['balanceOf_10', 'balanceOf_7', 'balanceOf_2', 'balanceOf_12', 'balanceOf_4'])
reserve0_19(uint112) := phi(['reserve0_31', 'reserve0_5', 'reserve0_6', 'reserve0_15', 'reserve0_36', 'reserve0_27', 'reserve0_18'])
reserve1_19(uint112) := phi(['reserve1_18', 'reserve1_6', 'reserve1_33', 'reserve1_5', 'reserve1_15', 'reserve1_38', 'reserve1_27'])
balance0_1(uint256) := TMP_3725(uint256)
 balance1 = IERC20(_token1).balanceOf(address(this))
TMP_3726 = CONVERT _token1_1 to IERC20
TMP_3727 = CONVERT this to address
TMP_3728(uint256) = HIGH_LEVEL_CALL, dest:TMP_3726(IERC20), function:balanceOf, arguments:['TMP_3727']  
totalSupply_17(uint256) := phi(['totalSupply_4', 'totalSupply_6', 'totalSupply_2', 'totalSupply_12', 'totalSupply_18', 'totalSupply_16'])
balanceOf_12(mapping(address => uint256)) := phi(['balanceOf_7', 'balanceOf_2', 'balanceOf_12', 'balanceOf_4', 'balanceOf_11'])
reserve0_20(uint112) := phi(['reserve0_31', 'reserve0_5', 'reserve0_19', 'reserve0_6', 'reserve0_15', 'reserve0_36', 'reserve0_27'])
reserve1_20(uint112) := phi(['reserve1_6', 'reserve1_33', 'reserve1_5', 'reserve1_19', 'reserve1_15', 'reserve1_38', 'reserve1_27'])
balance1_1(uint256) := TMP_3728(uint256)
 liquidity = balanceOf[address(this)]
TMP_3729 = CONVERT this to address
REF_1365(uint256) -> balanceOf_12[TMP_3729]
liquidity_1(uint256) := REF_1365(uint256)
 feeOn = _mintFee(_reserve0,_reserve1)
TMP_3730(bool) = INTERNAL_CALL, GTELaunchpadV2Pair._mintFee(uint112,uint112)(_reserve0_1,_reserve1_1)
totalSupply_18(uint256) := phi(['totalSupply_6'])
feeOn_1(bool) := TMP_3730(bool)
 _totalSupply = totalSupply
_totalSupply_1(uint256) := totalSupply_18(uint256)
 amount0 = liquidity.mul(balance0) / _totalSupply
TMP_3731(uint256) = LIBRARY_CALL, dest:SafeMath, function:SafeMath.mul(uint256,uint256), arguments:['liquidity_1', 'balance0_1'] 
TMP_3732(uint256) = TMP_3731 (c)/ _totalSupply_1
amount0_1(uint256) := TMP_3732(uint256)
 amount1 = liquidity.mul(balance1) / _totalSupply
TMP_3733(uint256) = LIBRARY_CALL, dest:SafeMath, function:SafeMath.mul(uint256,uint256), arguments:['liquidity_1', 'balance1_1'] 
TMP_3734(uint256) = TMP_3733 (c)/ _totalSupply_1
amount1_1(uint256) := TMP_3734(uint256)
 amount0 == 0 || amount1 == 0
TMP_3735(bool) = amount0_1 == 0
TMP_3736(bool) = amount1_1 == 0
TMP_3737(bool) = TMP_3735 || TMP_3736
CONDITION TMP_3737
 revert(string)(UniswapV2: INSUFFICIENT_LIQUIDITY_BURNED)
TMP_3738(None) = SOLIDITY_CALL revert(string)(UniswapV2: INSUFFICIENT_LIQUIDITY_BURNED)
 _burn(address(this),liquidity)
TMP_3739 = CONVERT this to address
INTERNAL_CALL, UniswapV2ERC20._burn(address,uint256)(TMP_3739,liquidity_1)
 _safeTransfer(_token0,to,amount0)
INTERNAL_CALL, GTELaunchpadV2Pair._safeTransfer(address,address,uint256)(_token0_1,to_1,amount0_1)
 _safeTransfer(_token1,to,amount1)
INTERNAL_CALL, GTELaunchpadV2Pair._safeTransfer(address,address,uint256)(_token1_1,to_1,amount1_1)
 balance0 = IERC20(_token0).balanceOf(address(this))
TMP_3743 = CONVERT _token0_1 to IERC20
TMP_3744 = CONVERT this to address
TMP_3745(uint256) = HIGH_LEVEL_CALL, dest:TMP_3743(IERC20), function:balanceOf, arguments:['TMP_3744']  
reserve0_25(uint112) := phi(['reserve0_31', 'reserve0_24', 'reserve0_5', 'reserve0_6', 'reserve0_15', 'reserve0_36', 'reserve0_27'])
reserve1_25(uint112) := phi(['reserve1_6', 'reserve1_33', 'reserve1_5', 'reserve1_24', 'reserve1_15', 'reserve1_38', 'reserve1_27'])
balance0_2(uint256) := TMP_3745(uint256)
 balance1 = IERC20(_token1).balanceOf(address(this))
TMP_3746 = CONVERT _token1_1 to IERC20
TMP_3747 = CONVERT this to address
TMP_3748(uint256) = HIGH_LEVEL_CALL, dest:TMP_3746(IERC20), function:balanceOf, arguments:['TMP_3747']  
reserve0_26(uint112) := phi(['reserve0_31', 'reserve0_5', 'reserve0_25', 'reserve0_6', 'reserve0_15', 'reserve0_36', 'reserve0_27'])
reserve1_26(uint112) := phi(['reserve1_6', 'reserve1_33', 'reserve1_5', 'reserve1_25', 'reserve1_15', 'reserve1_38', 'reserve1_27'])
balance1_2(uint256) := TMP_3748(uint256)
 _update(balance0,balance1,_reserve0,_reserve1,uint112(0),uint112(0))
TMP_3749 = CONVERT 0 to uint112
TMP_3750 = CONVERT 0 to uint112
INTERNAL_CALL, GTELaunchpadV2Pair._update(uint256,uint256,uint112,uint112,uint112,uint112)(balance0_2,balance1_2,_reserve0_1,_reserve1_1,TMP_3749,TMP_3750)
reserve0_27(uint112) := phi(['reserve0_6'])
reserve1_27(uint112) := phi(['reserve1_6'])
 feeOn
CONDITION feeOn_1
 kLast = uint256(reserve0).mul(reserve1)
TMP_3752 = CONVERT reserve0_27 to uint256
TMP_3753(uint256) = LIBRARY_CALL, dest:SafeMath, function:SafeMath.mul(uint256,uint256), arguments:['TMP_3752', 'reserve1_27'] 
kLast_5(uint256) := TMP_3753(uint256)
 Burn(msg.sender,amount0,amount1,to)
Emit Burn(msg.sender,amount0_1,amount1_1,to_1)
 lock()
MODIFIER_CALL, GTELaunchpadV2Pair.lock()()
 (amount0,amount1)
RETURN amount0_1,amount1_1
```
#### GTELaunchpadV2Pair.constructor() [PUBLIC]
```slithir
 factory = msg.sender
factory_1(address) := msg.sender(address)
```
#### GTELaunchpadV2Pair.endRewardsAccrual() [EXTERNAL]
```slithir
launchpadFeeDistributor_2(address) := phi(['launchpadFeeDistributor_1', 'launchpadFeeDistributor_11', 'launchpadFeeDistributor_0'])
token0_2(address) := phi(['token0_23', 'token0_15', 'token0_18', 'token0_0', 'token0_9', 'token0_12', 'token0_1', 'token0_5'])
token1_2(address) := phi(['token1_16', 'token1_24', 'token1_10', 'token1_0', 'token1_13', 'token1_19', 'token1_1', 'token1_5'])
reserve0_2(uint112) := phi(['reserve0_31', 'reserve0_5', 'reserve0_6', 'reserve0_15', 'reserve0_36', 'reserve0_27', 'reserve0_0'])
reserve1_2(uint112) := phi(['reserve1_6', 'reserve1_33', 'reserve1_5', 'reserve1_15', 'reserve1_38', 'reserve1_27', 'reserve1_0'])
accruedLaunchpadFee0_2(uint112) := phi(['accruedLaunchpadFee0_0', 'accruedLaunchpadFee0_10', 'accruedLaunchpadFee0_3', 'accruedLaunchpadFee0_6', 'accruedLaunchpadFee0_5'])
accruedLaunchpadFee1_2(uint112) := phi(['accruedLaunchpadFee1_3', 'accruedLaunchpadFee1_6', 'accruedLaunchpadFee1_0', 'accruedLaunchpadFee1_5', 'accruedLaunchpadFee1_12'])
rewardsPoolActive_2(uint256) := phi(['rewardsPoolActive_1', 'rewardsPoolActive_0', 'rewardsPoolActive_11', 'rewardsPoolActive_3'])
 msg.sender != launchpadFeeDistributor
TMP_3620(bool) = msg.sender != launchpadFeeDistributor_2
CONDITION TMP_3620
 revert(string)(GTEUniV2: FORBIDDEN)
TMP_3621(None) = SOLIDITY_CALL revert(string)(GTEUniV2: FORBIDDEN)
 delete accruedLaunchpadFee0
accruedLaunchpadFee0_3 = delete accruedLaunchpadFee0_2 
 delete accruedLaunchpadFee1
accruedLaunchpadFee1_3 = delete accruedLaunchpadFee1_2 
 delete rewardsPoolActive
rewardsPoolActive_3 = delete rewardsPoolActive_2 
 _update(IERC20(token0).balanceOf(address(this)),IERC20(token1).balanceOf(address(this)),reserve0,reserve1,uint112(0),uint112(0))
TMP_3622 = CONVERT token0_2 to IERC20
TMP_3623 = CONVERT this to address
TMP_3624(uint256) = HIGH_LEVEL_CALL, dest:TMP_3622(IERC20), function:balanceOf, arguments:['TMP_3623']  
token0_3(address) := phi(['token0_2', 'token0_23', 'token0_15', 'token0_18', 'token0_9', 'token0_12', 'token0_1', 'token0_5'])
token1_3(address) := phi(['token1_24', 'token1_10', 'token1_13', 'token1_19', 'token1_1', 'token1_2', 'token1_16', 'token1_5'])
reserve0_3(uint112) := phi(['reserve0_31', 'reserve0_5', 'reserve0_2', 'reserve0_6', 'reserve0_15', 'reserve0_36', 'reserve0_27'])
reserve1_3(uint112) := phi(['reserve1_6', 'reserve1_33', 'reserve1_5', 'reserve1_15', 'reserve1_38', 'reserve1_27', 'reserve1_2'])
TMP_3625 = CONVERT token1_3 to IERC20
TMP_3626 = CONVERT this to address
TMP_3627(uint256) = HIGH_LEVEL_CALL, dest:TMP_3625(IERC20), function:balanceOf, arguments:['TMP_3626']  
token0_4(address) := phi(['token0_23', 'token0_15', 'token0_3', 'token0_18', 'token0_9', 'token0_12', 'token0_1', 'token0_5'])
token1_4(address) := phi(['token1_24', 'token1_10', 'token1_3', 'token1_13', 'token1_19', 'token1_1', 'token1_16', 'token1_5'])
reserve0_4(uint112) := phi(['reserve0_31', 'reserve0_5', 'reserve0_6', 'reserve0_15', 'reserve0_36', 'reserve0_27', 'reserve0_3'])
reserve1_4(uint112) := phi(['reserve1_6', 'reserve1_33', 'reserve1_5', 'reserve1_15', 'reserve1_38', 'reserve1_27', 'reserve1_3'])
TMP_3628 = CONVERT 0 to uint112
TMP_3629 = CONVERT 0 to uint112
INTERNAL_CALL, GTELaunchpadV2Pair._update(uint256,uint256,uint112,uint112,uint112,uint112)(TMP_3624,TMP_3627,reserve0_4,reserve1_4,TMP_3628,TMP_3629)
reserve0_5(uint112) := phi(['reserve0_6'])
reserve1_5(uint112) := phi(['reserve1_6'])
 RewardsPoolDeactivated()
Emit RewardsPoolDeactivated()
```
#### GTELaunchpadV2Pair.getAccruedLaunchpadFees() [PUBLIC]
```slithir
blockTimestampLast_2(uint32) := phi(['blockTimestampLast_4', 'blockTimestampLast_0'])
accruedLaunchpadFee0_1(uint112) := phi(['accruedLaunchpadFee0_0', 'accruedLaunchpadFee0_10', 'accruedLaunchpadFee0_3', 'accruedLaunchpadFee0_6', 'accruedLaunchpadFee0_5'])
accruedLaunchpadFee1_1(uint112) := phi(['accruedLaunchpadFee1_3', 'accruedLaunchpadFee1_6', 'accruedLaunchpadFee1_0', 'accruedLaunchpadFee1_5', 'accruedLaunchpadFee1_12'])
 (accruedLaunchpadFee0,accruedLaunchpadFee1,blockTimestampLast)
RETURN accruedLaunchpadFee0_1,accruedLaunchpadFee1_1,blockTimestampLast_2
```
#### GTELaunchpadV2Pair.getReserves() [PUBLIC]
```slithir
reserve0_1(uint112) := phi(['reserve0_31', 'reserve0_5', 'reserve0_6', 'reserve0_15', 'reserve0_36', 'reserve0_27', 'reserve0_0'])
reserve1_1(uint112) := phi(['reserve1_6', 'reserve1_33', 'reserve1_5', 'reserve1_15', 'reserve1_38', 'reserve1_27', 'reserve1_0'])
blockTimestampLast_1(uint32) := phi(['blockTimestampLast_4', 'blockTimestampLast_0'])
 _reserve0 = reserve0
_reserve0_1(uint112) := reserve0_1(uint112)
 _reserve1 = reserve1
_reserve1_1(uint112) := reserve1_1(uint112)
 _blockTimestampLast = blockTimestampLast
_blockTimestampLast_1(uint32) := blockTimestampLast_1(uint32)
 (_reserve0,_reserve1,_blockTimestampLast)
RETURN _reserve0_1,_reserve1_1,_blockTimestampLast_1
```
#### GTELaunchpadV2Pair.initialize(address,address,address,address) [EXTERNAL]
```slithir
factory_2(address) := phi(['factory_4', 'factory_0', 'factory_1'])
 msg.sender != factory
TMP_3618(bool) = msg.sender != factory_2
CONDITION TMP_3618
 revert(string)(UniswapV2: FORBIDDEN)
TMP_3619(None) = SOLIDITY_CALL revert(string)(UniswapV2: FORBIDDEN)
 token0 = _token0
token0_1(address) := _token0_1(address)
 token1 = _token1
token1_1(address) := _token1_1(address)
 launchpadLp = _launchpadLp
launchpadLp_1(address) := _launchpadLp_1(address)
 launchpadFeeDistributor = _launchpadFeeDistributor
launchpadFeeDistributor_1(address) := _launchpadFeeDistributor_1(address)
 rewardsPoolActive = 1
rewardsPoolActive_1(uint256) := 1(uint256)
```
#### GTELaunchpadV2Pair.mint(address) [EXTERNAL]
```slithir
totalSupply_7(uint256) := phi(['totalSupply_4', 'totalSupply_0', 'totalSupply_6', 'totalSupply_2', 'totalSupply_12', 'totalSupply_18'])
MINIMUM_LIQUIDITY_1(uint256) := phi(['MINIMUM_LIQUIDITY_6', 'MINIMUM_LIQUIDITY_0', 'MINIMUM_LIQUIDITY_7', 'MINIMUM_LIQUIDITY_10'])
token0_6(address) := phi(['token0_23', 'token0_15', 'token0_18', 'token0_0', 'token0_9', 'token0_12', 'token0_1', 'token0_5'])
token1_6(address) := phi(['token1_16', 'token1_24', 'token1_10', 'token1_0', 'token1_13', 'token1_19', 'token1_1', 'token1_5'])
reserve0_7(uint112) := phi(['reserve0_31', 'reserve0_5', 'reserve0_6', 'reserve0_15', 'reserve0_36', 'reserve0_27', 'reserve0_0'])
reserve1_7(uint112) := phi(['reserve1_6', 'reserve1_33', 'reserve1_5', 'reserve1_15', 'reserve1_38', 'reserve1_27', 'reserve1_0'])
 (_reserve0,_reserve1,None) = getReserves()
TUPLE_35(uint112,uint112,uint32) = INTERNAL_CALL, GTELaunchpadV2Pair.getReserves()()
_reserve0_1(uint112)= UNPACK TUPLE_35 index: 0 
_reserve1_1(uint112)= UNPACK TUPLE_35 index: 1 
 balance0 = IERC20(token0).balanceOf(address(this))
TMP_3693 = CONVERT token0_8 to IERC20
TMP_3694 = CONVERT this to address
TMP_3695(uint256) = HIGH_LEVEL_CALL, dest:TMP_3693(IERC20), function:balanceOf, arguments:['TMP_3694']  
totalSupply_10(uint256) := phi(['totalSupply_4', 'totalSupply_6', 'totalSupply_2', 'totalSupply_12', 'totalSupply_18', 'totalSupply_9'])
MINIMUM_LIQUIDITY_4(uint256) := phi(['MINIMUM_LIQUIDITY_3', 'MINIMUM_LIQUIDITY_6', 'MINIMUM_LIQUIDITY_7', 'MINIMUM_LIQUIDITY_10'])
token0_9(address) := phi(['token0_23', 'token0_15', 'token0_8', 'token0_18', 'token0_9', 'token0_12', 'token0_1', 'token0_5'])
token1_9(address) := phi(['token1_24', 'token1_10', 'token1_8', 'token1_13', 'token1_19', 'token1_1', 'token1_16', 'token1_5'])
reserve0_10(uint112) := phi(['reserve0_31', 'reserve0_5', 'reserve0_6', 'reserve0_15', 'reserve0_36', 'reserve0_27', 'reserve0_9'])
reserve1_10(uint112) := phi(['reserve1_6', 'reserve1_33', 'reserve1_5', 'reserve1_15', 'reserve1_38', 'reserve1_27', 'reserve1_9'])
balance0_1(uint256) := TMP_3695(uint256)
 balance1 = IERC20(token1).balanceOf(address(this))
TMP_3696 = CONVERT token1_9 to IERC20
TMP_3697 = CONVERT this to address
TMP_3698(uint256) = HIGH_LEVEL_CALL, dest:TMP_3696(IERC20), function:balanceOf, arguments:['TMP_3697']  
totalSupply_11(uint256) := phi(['totalSupply_10', 'totalSupply_4', 'totalSupply_6', 'totalSupply_2', 'totalSupply_12', 'totalSupply_18'])
MINIMUM_LIQUIDITY_5(uint256) := phi(['MINIMUM_LIQUIDITY_4', 'MINIMUM_LIQUIDITY_6', 'MINIMUM_LIQUIDITY_7', 'MINIMUM_LIQUIDITY_10'])
token1_10(address) := phi(['token1_24', 'token1_10', 'token1_13', 'token1_19', 'token1_9', 'token1_1', 'token1_16', 'token1_5'])
reserve0_11(uint112) := phi(['reserve0_31', 'reserve0_5', 'reserve0_6', 'reserve0_15', 'reserve0_36', 'reserve0_27', 'reserve0_10'])
reserve1_11(uint112) := phi(['reserve1_10', 'reserve1_6', 'reserve1_33', 'reserve1_5', 'reserve1_15', 'reserve1_38', 'reserve1_27'])
balance1_1(uint256) := TMP_3698(uint256)
 amount0 = balance0.sub(_reserve0)
TMP_3699(uint256) = LIBRARY_CALL, dest:SafeMath, function:SafeMath.sub(uint256,uint256), arguments:['balance0_1', '_reserve0_1'] 
amount0_1(uint256) := TMP_3699(uint256)
 amount1 = balance1.sub(_reserve1)
TMP_3700(uint256) = LIBRARY_CALL, dest:SafeMath, function:SafeMath.sub(uint256,uint256), arguments:['balance1_1', '_reserve1_1'] 
amount1_1(uint256) := TMP_3700(uint256)
 feeOn = _mintFee(_reserve0,_reserve1)
TMP_3701(bool) = INTERNAL_CALL, GTELaunchpadV2Pair._mintFee(uint112,uint112)(_reserve0_1,_reserve1_1)
totalSupply_12(uint256) := phi(['totalSupply_6'])
feeOn_1(bool) := TMP_3701(bool)
 _totalSupply = totalSupply
_totalSupply_1(uint256) := totalSupply_12(uint256)
 _totalSupply == 0
TMP_3702(bool) = _totalSupply_1 == 0
CONDITION TMP_3702
 liquidity = Math.sqrt(amount0.mul(amount1)).sub(MINIMUM_LIQUIDITY)
TMP_3703(uint256) = LIBRARY_CALL, dest:SafeMath, function:SafeMath.mul(uint256,uint256), arguments:['amount0_1', 'amount1_1'] 
TMP_3704(uint256) = LIBRARY_CALL, dest:Math, function:Math.sqrt(uint256), arguments:['TMP_3703'] 
TMP_3705(uint256) = LIBRARY_CALL, dest:SafeMath, function:SafeMath.sub(uint256,uint256), arguments:['TMP_3704', 'MINIMUM_LIQUIDITY_6'] 
liquidity_2(uint256) := TMP_3705(uint256)
 _mint(address(0),MINIMUM_LIQUIDITY)
TMP_3706 = CONVERT 0 to address
INTERNAL_CALL, UniswapV2ERC20._mint(address,uint256)(TMP_3706,MINIMUM_LIQUIDITY_6)
 liquidity = Math.min(amount0.mul(_totalSupply) / _reserve0,amount1.mul(_totalSupply) / _reserve1)
TMP_3708(uint256) = LIBRARY_CALL, dest:SafeMath, function:SafeMath.mul(uint256,uint256), arguments:['amount0_1', '_totalSupply_1'] 
TMP_3709(uint256) = TMP_3708 (c)/ _reserve0_1
TMP_3710(uint256) = LIBRARY_CALL, dest:SafeMath, function:SafeMath.mul(uint256,uint256), arguments:['amount1_1', '_totalSupply_1'] 
TMP_3711(uint256) = TMP_3710 (c)/ _reserve1_1
TMP_3712(uint256) = LIBRARY_CALL, dest:Math, function:Math.min(uint256,uint256), arguments:['TMP_3709', 'TMP_3711'] 
liquidity_1(uint256) := TMP_3712(uint256)
liquidity_3(uint256) := phi(['liquidity_1', 'liquidity_2'])
 liquidity == 0
TMP_3713(bool) = liquidity_3 == 0
CONDITION TMP_3713
 revert(string)(UniswapV2: INSUFFICIENT_LIQUIDITY_MINTED)
TMP_3714(None) = SOLIDITY_CALL revert(string)(UniswapV2: INSUFFICIENT_LIQUIDITY_MINTED)
 _mint(to,liquidity)
INTERNAL_CALL, UniswapV2ERC20._mint(address,uint256)(to_1,liquidity_3)
 _update(balance0,balance1,_reserve0,_reserve1,uint112(0),uint112(0))
TMP_3716 = CONVERT 0 to uint112
TMP_3717 = CONVERT 0 to uint112
INTERNAL_CALL, GTELaunchpadV2Pair._update(uint256,uint256,uint112,uint112,uint112,uint112)(balance0_1,balance1_1,_reserve0_1,_reserve1_1,TMP_3716,TMP_3717)
reserve0_15(uint112) := phi(['reserve0_6'])
reserve1_15(uint112) := phi(['reserve1_6'])
 feeOn
CONDITION feeOn_1
 kLast = uint256(reserve0).mul(reserve1)
TMP_3719 = CONVERT reserve0_15 to uint256
TMP_3720(uint256) = LIBRARY_CALL, dest:SafeMath, function:SafeMath.mul(uint256,uint256), arguments:['TMP_3719', 'reserve1_15'] 
kLast_4(uint256) := TMP_3720(uint256)
 Mint(msg.sender,amount0,amount1)
Emit Mint(msg.sender,amount0_1,amount1_1)
 lock()
MODIFIER_CALL, GTELaunchpadV2Pair.lock()()
 liquidity
RETURN liquidity_3
```
#### GTELaunchpadV2Pair.skim(address) [EXTERNAL]
```slithir
token0_17(address) := phi(['token0_23', 'token0_15', 'token0_18', 'token0_0', 'token0_9', 'token0_12', 'token0_1', 'token0_5'])
token1_18(address) := phi(['token1_16', 'token1_24', 'token1_10', 'token1_0', 'token1_13', 'token1_19', 'token1_1', 'token1_5'])
reserve0_28(uint112) := phi(['reserve0_31', 'reserve0_5', 'reserve0_6', 'reserve0_15', 'reserve0_36', 'reserve0_27', 'reserve0_0'])
reserve1_28(uint112) := phi(['reserve1_6', 'reserve1_33', 'reserve1_5', 'reserve1_15', 'reserve1_38', 'reserve1_27', 'reserve1_0'])
accruedLaunchpadFee0_7(uint112) := phi(['accruedLaunchpadFee0_0', 'accruedLaunchpadFee0_10', 'accruedLaunchpadFee0_3', 'accruedLaunchpadFee0_6', 'accruedLaunchpadFee0_5'])
accruedLaunchpadFee1_7(uint112) := phi(['accruedLaunchpadFee1_3', 'accruedLaunchpadFee1_6', 'accruedLaunchpadFee1_0', 'accruedLaunchpadFee1_5', 'accruedLaunchpadFee1_12'])
 _token0 = token0
_token0_1(address) := token0_18(address)
 _token1 = token1
_token1_1(address) := token1_19(address)
 _safeTransfer(_token0,to,IERC20(_token0).balanceOf(address(this)).sub(reserve0 + accruedLaunchpadFee0))
TMP_3843 = CONVERT _token0_1 to IERC20
TMP_3844 = CONVERT this to address
TMP_3845(uint256) = HIGH_LEVEL_CALL, dest:TMP_3843(IERC20), function:balanceOf, arguments:['TMP_3844']  
reserve0_30(uint112) := phi(['reserve0_31', 'reserve0_5', 'reserve0_6', 'reserve0_15', 'reserve0_36', 'reserve0_27', 'reserve0_29'])
reserve1_30(uint112) := phi(['reserve1_29', 'reserve1_6', 'reserve1_33', 'reserve1_5', 'reserve1_15', 'reserve1_38', 'reserve1_27'])
accruedLaunchpadFee0_9(uint112) := phi(['accruedLaunchpadFee0_8', 'accruedLaunchpadFee0_10', 'accruedLaunchpadFee0_3', 'accruedLaunchpadFee0_6', 'accruedLaunchpadFee0_5'])
accruedLaunchpadFee1_9(uint112) := phi(['accruedLaunchpadFee1_3', 'accruedLaunchpadFee1_6', 'accruedLaunchpadFee1_5', 'accruedLaunchpadFee1_8', 'accruedLaunchpadFee1_12'])
TMP_3846(uint112) = reserve0_30 (c)+ accruedLaunchpadFee0_9
TMP_3847(uint256) = LIBRARY_CALL, dest:SafeMath, function:SafeMath.sub(uint256,uint256), arguments:['TMP_3845', 'TMP_3846'] 
INTERNAL_CALL, GTELaunchpadV2Pair._safeTransfer(address,address,uint256)(_token0_1,to_1,TMP_3847)
 _safeTransfer(_token1,to,IERC20(_token1).balanceOf(address(this)).sub(reserve1 + accruedLaunchpadFee1))
TMP_3849 = CONVERT _token1_1 to IERC20
TMP_3850 = CONVERT this to address
TMP_3851(uint256) = HIGH_LEVEL_CALL, dest:TMP_3849(IERC20), function:balanceOf, arguments:['TMP_3850']  
reserve1_32(uint112) := phi(['reserve1_6', 'reserve1_31', 'reserve1_33', 'reserve1_5', 'reserve1_15', 'reserve1_38', 'reserve1_27'])
accruedLaunchpadFee1_11(uint112) := phi(['accruedLaunchpadFee1_3', 'accruedLaunchpadFee1_6', 'accruedLaunchpadFee1_5', 'accruedLaunchpadFee1_12', 'accruedLaunchpadFee1_10'])
TMP_3852(uint112) = reserve1_32 (c)+ accruedLaunchpadFee1_11
TMP_3853(uint256) = LIBRARY_CALL, dest:SafeMath, function:SafeMath.sub(uint256,uint256), arguments:['TMP_3851', 'TMP_3852'] 
INTERNAL_CALL, GTELaunchpadV2Pair._safeTransfer(address,address,uint256)(_token1_1,to_1,TMP_3853)
 lock()
MODIFIER_CALL, GTELaunchpadV2Pair.lock()()
```
#### RewardsTrackerStorage.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 LAUNCH_ASSET_TO_REWARDS_SLOT = keccak256(bytes)(abi.encode(uint256(keccak256(bytes)(rewardsTrackerPool.self.slot)) - 1)) & ~ bytes32(uint256(0xff))
```
#### GTELaunchpadV2Pair.slitherConstructorVariables() [INTERNAL]
```slithir
 rewardsPoolActive = 1
 unlocked = 1
```
#### GTELaunchpadV2Pair.swap(uint256,uint256,address,bytes) [EXTERNAL]
```slithir
launchpadFeeDistributor_4(address) := phi(['launchpadFeeDistributor_1', 'launchpadFeeDistributor_11', 'launchpadFeeDistributor_0'])
token0_13(address) := phi(['token0_23', 'token0_15', 'token0_18', 'token0_0', 'token0_9', 'token0_12', 'token0_1', 'token0_5'])
token1_14(address) := phi(['token1_16', 'token1_24', 'token1_10', 'token1_0', 'token1_13', 'token1_19', 'token1_1', 'token1_5'])
rewardsPoolActive_4(uint256) := phi(['rewardsPoolActive_1', 'rewardsPoolActive_0', 'rewardsPoolActive_11', 'rewardsPoolActive_3'])
 amount0Out == 0 && amount1Out == 0
TMP_3756(bool) = amount0Out_1 == 0
TMP_3757(bool) = amount1Out_1 == 0
TMP_3758(bool) = TMP_3756 && TMP_3757
CONDITION TMP_3758
 revert(string)(UniswapV2: INSUFFICIENT_OUTPUT_AMOUNT)
TMP_3759(None) = SOLIDITY_CALL revert(string)(UniswapV2: INSUFFICIENT_OUTPUT_AMOUNT)
 (_reserve0,_reserve1,None) = getReserves()
TUPLE_37(uint112,uint112,uint32) = INTERNAL_CALL, GTELaunchpadV2Pair.getReserves()()
_reserve0_1(uint112)= UNPACK TUPLE_37 index: 0 
_reserve1_1(uint112)= UNPACK TUPLE_37 index: 1 
 amount0Out >= _reserve0 || amount1Out >= _reserve1
TMP_3760(bool) = amount0Out_1 >= _reserve0_1
TMP_3761(bool) = amount1Out_1 >= _reserve1_1
TMP_3762(bool) = TMP_3760 || TMP_3761
CONDITION TMP_3762
 revert(string)(UniswapV2: INSUFFICIENT_LIQUIDITY)
TMP_3763(None) = SOLIDITY_CALL revert(string)(UniswapV2: INSUFFICIENT_LIQUIDITY)
 _token0 = token0
_token0_1(address) := token0_15(address)
 _token1 = token1
_token1_1(address) := token1_16(address)
 to == _token0 || to == _token1
TMP_3764(bool) = to_1 == _token0_1
TMP_3765(bool) = to_1 == _token1_1
TMP_3766(bool) = TMP_3764 || TMP_3765
CONDITION TMP_3766
 revert(string)(UniswapV2: INVALID_TO)
TMP_3767(None) = SOLIDITY_CALL revert(string)(UniswapV2: INVALID_TO)
 amount0Out > 0
TMP_3768(bool) = amount0Out_1 > 0
CONDITION TMP_3768
 _safeTransfer(_token0,to,amount0Out)
INTERNAL_CALL, GTELaunchpadV2Pair._safeTransfer(address,address,uint256)(_token0_1,to_1,amount0Out_1)
 amount1Out > 0
TMP_3770(bool) = amount1Out_1 > 0
CONDITION TMP_3770
 _safeTransfer(_token1,to,amount1Out)
INTERNAL_CALL, GTELaunchpadV2Pair._safeTransfer(address,address,uint256)(_token1_1,to_1,amount1Out_1)
 data.length > 0
REF_1371 -> LENGTH data_1
TMP_3772(bool) = REF_1371 > 0
CONDITION TMP_3772
 IUniswapV2Callee(to).uniswapV2Call(msg.sender,amount0Out,amount1Out,data)
TMP_3773 = CONVERT to_1 to IUniswapV2Callee
HIGH_LEVEL_CALL, dest:TMP_3773(IUniswapV2Callee), function:uniswapV2Call, arguments:['msg.sender', 'amount0Out_1', 'amount1Out_1', 'data_1']  
launchpadFeeDistributor_9(address) := phi(['launchpadFeeDistributor_1', 'launchpadFeeDistributor_8', 'launchpadFeeDistributor_11'])
rewardsPoolActive_9(uint256) := phi(['rewardsPoolActive_1', 'rewardsPoolActive_8', 'rewardsPoolActive_11', 'rewardsPoolActive_3'])
 balance0 = IERC20(_token0).balanceOf(address(this))
TMP_3775 = CONVERT _token0_1 to IERC20
TMP_3776 = CONVERT this to address
TMP_3777(uint256) = HIGH_LEVEL_CALL, dest:TMP_3775(IERC20), function:balanceOf, arguments:['TMP_3776']  
launchpadFeeDistributor_10(address) := phi(['launchpadFeeDistributor_1', 'launchpadFeeDistributor_9', 'launchpadFeeDistributor_11'])
rewardsPoolActive_10(uint256) := phi(['rewardsPoolActive_1', 'rewardsPoolActive_9', 'rewardsPoolActive_11', 'rewardsPoolActive_3'])
balance0_1(uint256) := TMP_3777(uint256)
 balance1 = IERC20(_token1).balanceOf(address(this))
TMP_3778 = CONVERT _token1_1 to IERC20
TMP_3779 = CONVERT this to address
TMP_3780(uint256) = HIGH_LEVEL_CALL, dest:TMP_3778(IERC20), function:balanceOf, arguments:['TMP_3779']  
launchpadFeeDistributor_11(address) := phi(['launchpadFeeDistributor_10', 'launchpadFeeDistributor_1', 'launchpadFeeDistributor_11'])
rewardsPoolActive_11(uint256) := phi(['rewardsPoolActive_1', 'rewardsPoolActive_11', 'rewardsPoolActive_10', 'rewardsPoolActive_3'])
balance1_1(uint256) := TMP_3780(uint256)
 amount0In == 0 && amount1In == 0
TMP_3781(bool) = amount0In_3 == 0
TMP_3782(bool) = amount1In_3 == 0
TMP_3783(bool) = TMP_3781 && TMP_3782
CONDITION TMP_3783
 revert(string)(UniswapV2: INSUFFICIENT_INPUT_AMOUNT)
TMP_3784(None) = SOLIDITY_CALL revert(string)(UniswapV2: INSUFFICIENT_INPUT_AMOUNT)
 balance0Adjusted = balance0.mul(1000).sub(amount0In.mul(3))
TMP_3785(uint256) = LIBRARY_CALL, dest:SafeMath, function:SafeMath.mul(uint256,uint256), arguments:['balance0_1', '1000'] 
TMP_3786(uint256) = LIBRARY_CALL, dest:SafeMath, function:SafeMath.mul(uint256,uint256), arguments:['amount0In_3', '3'] 
TMP_3787(uint256) = LIBRARY_CALL, dest:SafeMath, function:SafeMath.sub(uint256,uint256), arguments:['TMP_3785', 'TMP_3786'] 
balance0Adjusted_1(uint256) := TMP_3787(uint256)
 balance1Adjusted = balance1.mul(1000).sub(amount1In.mul(3))
TMP_3788(uint256) = LIBRARY_CALL, dest:SafeMath, function:SafeMath.mul(uint256,uint256), arguments:['balance1_1', '1000'] 
TMP_3789(uint256) = LIBRARY_CALL, dest:SafeMath, function:SafeMath.mul(uint256,uint256), arguments:['amount1In_3', '3'] 
TMP_3790(uint256) = LIBRARY_CALL, dest:SafeMath, function:SafeMath.sub(uint256,uint256), arguments:['TMP_3788', 'TMP_3789'] 
balance1Adjusted_1(uint256) := TMP_3790(uint256)
 balance0Adjusted.mul(balance1Adjusted) < uint256(_reserve0).mul(_reserve1).mul(1000 ** 2)
TMP_3791(uint256) = LIBRARY_CALL, dest:SafeMath, function:SafeMath.mul(uint256,uint256), arguments:['balance0Adjusted_1', 'balance1Adjusted_1'] 
TMP_3792 = CONVERT _reserve0_1 to uint256
TMP_3793(uint256) = LIBRARY_CALL, dest:SafeMath, function:SafeMath.mul(uint256,uint256), arguments:['TMP_3792', '_reserve1_1'] 
TMP_3794(uint256) = 1000 (c)** 2
TMP_3795(uint256) = LIBRARY_CALL, dest:SafeMath, function:SafeMath.mul(uint256,uint256), arguments:['TMP_3793', 'TMP_3794'] 
TMP_3796(bool) = TMP_3791 < TMP_3795
CONDITION TMP_3796
 revert(string)(UniswapV2: K)
TMP_3797(None) = SOLIDITY_CALL revert(string)(UniswapV2: K)
 _update(balance0,balance1,_reserve0,_reserve1,launchpadFee0,launchpadFee1)
INTERNAL_CALL, GTELaunchpadV2Pair._update(uint256,uint256,uint112,uint112,uint112,uint112)(balance0_1,balance1_1,_reserve0_1,_reserve1_1,launchpadFee0_3,launchpadFee1_3)
 Swap(msg.sender,amount0In,amount1In,amount0Out,amount1Out,to)
Emit Swap(msg.sender,amount0In_3,amount1In_3,amount0Out_1,amount1Out_1,to_1)
 lock()
MODIFIER_CALL, GTELaunchpadV2Pair.lock()()
 balance0 > _reserve0 - amount0Out
TMP_3801(uint112) = _reserve0_1 (c)- amount0Out_1
TMP_3802(bool) = balance0_1 > TMP_3801
CONDITION TMP_3802
 amount0In = balance0 - (_reserve0 - amount0Out)
TMP_3803(uint112) = _reserve0_1 (c)- amount0Out_1
TMP_3804(uint256) = balance0_1 (c)- TMP_3803
amount0In_1(uint256) := TMP_3804(uint256)
 amount0In = 0
amount0In_2(uint256) := 0(uint256)
amount0In_3(uint256) := phi(['amount0In_1', 'amount0In_2'])
 balance1 > _reserve1 - amount1Out
TMP_3805(uint112) = _reserve1_1 (c)- amount1Out_1
TMP_3806(bool) = balance1_1 > TMP_3805
CONDITION TMP_3806
 amount1In = balance1 - (_reserve1 - amount1Out)
TMP_3807(uint112) = _reserve1_1 (c)- amount1Out_1
TMP_3808(uint256) = balance1_1 (c)- TMP_3807
amount1In_1(uint256) := TMP_3808(uint256)
 amount1In = 0
amount1In_2(uint256) := 0(uint256)
amount1In_3(uint256) := phi(['amount1In_1', 'amount1In_2'])
 launchpadFeeDistributor > address(0) && rewardsPoolActive > 0
TMP_3809 = CONVERT 0 to address
TMP_3810(bool) = launchpadFeeDistributor_11 > TMP_3809
TMP_3811(bool) = rewardsPoolActive_11 > 0
TMP_3812(bool) = TMP_3810 && TMP_3811
CONDITION TMP_3812
 (launchpadFee0,launchpadFee1) = _getLaunchpadFees(amount0In,amount1In)
TUPLE_38(uint112,uint112) = INTERNAL_CALL, GTELaunchpadV2Pair._getLaunchpadFees(uint256,uint256)(amount0In_3,amount1In_3)
launchpadFee0_1(uint112)= UNPACK TUPLE_38 index: 0 
launchpadFee1_1(uint112)= UNPACK TUPLE_38 index: 1 
 (launchpadFee0,launchpadFee1) = (uint112(0),uint112(0))
TMP_3813 = CONVERT 0 to uint112
TMP_3814 = CONVERT 0 to uint112
launchpadFee0_2(uint112) := TMP_3813(uint112)
launchpadFee1_2(uint112) := TMP_3814(uint112)
launchpadFee0_3(uint112) := phi(['launchpadFee0_1', 'launchpadFee0_2'])
launchpadFee1_3(uint112) := phi(['launchpadFee1_1', 'launchpadFee1_2'])
```
#### GTELaunchpadV2Pair.sync() [EXTERNAL]
```slithir
token0_19(address) := phi(['token0_23', 'token0_15', 'token0_18', 'token0_0', 'token0_9', 'token0_12', 'token0_1', 'token0_5'])
token1_20(address) := phi(['token1_16', 'token1_24', 'token1_10', 'token1_0', 'token1_13', 'token1_19', 'token1_1', 'token1_5'])
reserve0_32(uint112) := phi(['reserve0_31', 'reserve0_5', 'reserve0_6', 'reserve0_15', 'reserve0_36', 'reserve0_27', 'reserve0_0'])
reserve1_34(uint112) := phi(['reserve1_6', 'reserve1_33', 'reserve1_5', 'reserve1_15', 'reserve1_38', 'reserve1_27', 'reserve1_0'])
 _update(IERC20(token0).balanceOf(address(this)),IERC20(token1).balanceOf(address(this)),reserve0,reserve1,uint112(0),uint112(0))
TMP_3856 = CONVERT token0_20 to IERC20
TMP_3857 = CONVERT this to address
TMP_3858(uint256) = HIGH_LEVEL_CALL, dest:TMP_3856(IERC20), function:balanceOf, arguments:['TMP_3857']  
token0_21(address) := phi(['token0_23', 'token0_20', 'token0_15', 'token0_18', 'token0_9', 'token0_12', 'token0_1', 'token0_5'])
token1_22(address) := phi(['token1_24', 'token1_21', 'token1_10', 'token1_13', 'token1_19', 'token1_1', 'token1_16', 'token1_5'])
reserve0_34(uint112) := phi(['reserve0_31', 'reserve0_5', 'reserve0_6', 'reserve0_15', 'reserve0_36', 'reserve0_27', 'reserve0_33'])
reserve1_36(uint112) := phi(['reserve1_6', 'reserve1_33', 'reserve1_5', 'reserve1_15', 'reserve1_38', 'reserve1_27', 'reserve1_35'])
TMP_3859 = CONVERT token1_22 to IERC20
TMP_3860 = CONVERT this to address
TMP_3861(uint256) = HIGH_LEVEL_CALL, dest:TMP_3859(IERC20), function:balanceOf, arguments:['TMP_3860']  
token0_22(address) := phi(['token0_23', 'token0_15', 'token0_18', 'token0_9', 'token0_12', 'token0_1', 'token0_21', 'token0_5'])
token1_23(address) := phi(['token1_24', 'token1_10', 'token1_22', 'token1_13', 'token1_19', 'token1_1', 'token1_16', 'token1_5'])
reserve0_35(uint112) := phi(['reserve0_31', 'reserve0_5', 'reserve0_6', 'reserve0_15', 'reserve0_36', 'reserve0_27', 'reserve0_34'])
reserve1_37(uint112) := phi(['reserve1_36', 'reserve1_6', 'reserve1_33', 'reserve1_5', 'reserve1_15', 'reserve1_38', 'reserve1_27'])
TMP_3862 = CONVERT 0 to uint112
TMP_3863 = CONVERT 0 to uint112
INTERNAL_CALL, GTELaunchpadV2Pair._update(uint256,uint256,uint112,uint112,uint112,uint112)(TMP_3858,TMP_3861,reserve0_35,reserve1_37,TMP_3862,TMP_3863)
reserve0_36(uint112) := phi(['reserve0_6'])
reserve1_38(uint112) := phi(['reserve1_6'])
 lock()
MODIFIER_CALL, GTELaunchpadV2Pair.lock()()
```
#### IDistributor.addRewards(address,address,uint128,uint128) [EXTERNAL]
```slithir

```

#### IUniswapV2Factory.feeTo() [EXTERNAL]
```slithir

```

#### SafeMath.add(uint256,uint256) [INTERNAL]
```slithir
 x + y
TMP_9947(uint256) = x_1 (c)+ y_1
RETURN TMP_9947
 z
```
#### SafeMath.sub(uint256,uint256) [INTERNAL]
```slithir
 x - y
TMP_9948(uint256) = x_1 (c)- y_1
RETURN TMP_9948
 z
```
#### UQ112x112.encode(uint112) [INTERNAL]
```slithir
Q112_1(uint224) := phi(['Q112_0'])
 z = uint224(y) * Q112
TMP_9950 = CONVERT y_1 to uint224
TMP_9951(uint224) = TMP_9950 (c)* Q112_1
z_1(uint224) := TMP_9951(uint224)
 z
RETURN z_1
```
#### UQ112x112.uqdiv(uint224,uint112) [INTERNAL]
```slithir
 z = x / uint224(y)
TMP_9952 = CONVERT y_1 to uint224
TMP_9953(uint224) = x_1 (c)/ TMP_9952
z_1(uint224) := TMP_9953(uint224)
 z
RETURN z_1
```
#### Math.min(uint256,uint256) [INTERNAL]
```slithir
 x < y
TMP_9938(bool) = x_1 < y_1
CONDITION TMP_9938
 z = x
z_2(uint256) := x_1(uint256)
 z = y
z_1(uint256) := y_1(uint256)
z_3(uint256) := phi(['z_1', 'z_2'])
 z
RETURN z_3
```

