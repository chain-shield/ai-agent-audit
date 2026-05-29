# Benchmark Ground Truth: BakerFi Invitational

## Accepted H/M Findings

# Accepted H/M Findings: BakerFi Invitational

# [H-01] ETHOracle.getLatestPrice needs to convert to 18 decimals

- **Contest:** BakerFi Invitational
- **Slug:** 2024-05-bakerfi-invitational
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-05-bakerfi-invitational
- **Source snapshot:** competitions/2024-05-bakerfi-invitational/final_report.html

ETHOracle.getLatestPrice needs to convert to 18 decimals Submitted by bin2chen In ETHOracle.sol, getPrecision() is defined as 10 ** 18, but the actual oracle used is 8-decimals.

- https://data.chain.link/feeds/arbitrum/mainnet/eth-usd
This data feeds’ decimals is 8.

/** * ETH/USD Oracle using chainlink data feeds * * For more information about the feed go to @> * https://data.chain.link/feeds/arbitrum/mainnet/eth-usd * **/ contract ETHOracle is IOracle { IChainlinkAggregator private immutable _ethPriceFeed; @> uint256 private constant _PRECISION = 10 ** 18;....

function getLatestPrice () public view override returns (IOracle.Price memory price ) { (, int256 answer, uint256 startedAt, uint256 updatedAt,) = _ethPriceFeed.

latestRoundData (); if ( answer <= 0 ) revert InvalidPriceFromOracle (); if ( startedAt == 0 || updatedAt == 0 ) revert InvalidPriceUpdatedAt (); @> price.

price = uint256 ( answer ); //@audit 8 decimals price.

lastUpdate = updatedAt; } PythOracle is used to get prices for other tokens, also getPrecision() == 18.

contract PythOracle is IOracle {...

uint256 private constant _PRECISION = 18; function _getPriceInternal ( uint256 age ) private view returns (IOracle.Price memory outPrice ) { PythStructs.

Price memory price = age == 0 ?

_pyth.

getPriceUnsafe ( _priceID ):

_pyth.

getPriceNoOlderThan ( _priceID, age ); if ( price.

expo >= 0 ) { outPrice.

price = uint64 ( price.

price ) * uint256 ( 10 ** ( _PRECISION + uint32 ( price.

expo ))); } else { outPrice.

price = uint64 ( price.

price ) * uint256 ( 10 ** ( _PRECISION - uint32 (- price.

expo ))); } outPrice.

lastUpdate = price.

publishTime; } Since a different precision is used, then the calculation of totalCollateralInEth at StrategyLeverage will be wrong.

function _getPosition ( uint256 priceMaxAge ) internal view returns ( uint256 totalCollateralInEth, uint256 totalDebtInEth ) {...

totalCollateralInEth = 0; totalDebtInEth = 0; ( uint256 collateralBalance, uint256 debtBalance ) = _getMMPosition (); if ( collateralBalance != 0 ) { IOracle.

Price memory ethPrice = priceMaxAge == 0 ?

_ethUSDOracle.

getLatestPrice ():

_ethUSDOracle.

getSafeLatestPrice ( priceMaxAge ); IOracle.

Price memory collateralPrice = priceMaxAge == 0 ?

_collateralOracle.

getLatestPrice ():

_collateralOracle.

getSafeLatestPrice ( priceMaxAge ); if ( !( priceMaxAge == 0 || ( priceMaxAge > 0 && ( ethPrice.

lastUpdate >= ( block.

timestamp - priceMaxAge ))) || ( priceMaxAge > 0 && ( collateralPrice.

lastUpdate >= ( block.

timestamp - priceMaxAge )))) ) { revert PriceOutdated (); } @> totalCollateralInEth = ( collateralBalance * collateralPrice.

price ) / ethPrice.

price; } if ( debtBalance != 0 ) { totalDebtInEth = debtBalance; }

## Impact

Incorrect calculation of totalCollateralInEth, affecting borrowing judgment, e.g. overvaluation overborrowing, etc.

## Recommended Mitigation

Convert to 18 decimals.

hvasconcelos (BakerFi) confirmed ickas (BakerFi) commented:

Fixed →

- https://github.com/baker-fi/bakerfi-contracts/pull/40

# [H-02] Vault is vulnerable to first depositor inflation attack

- **Contest:** BakerFi Invitational
- **Slug:** 2024-05-bakerfi-invitational
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-05-bakerfi-invitational
- **Source snapshot:** competitions/2024-05-bakerfi-invitational/final_report.html

Submitted by 0xStalin, also found by bin2chen and rvierdiiev First depositor can manipulate the price of the shares at will, forcing new depositors to deposit more ETH for the same amount of shares that the first depositor paid.

## Recommended Mitigation Steps

Consider either of these options:

Consider seeding the pools during deployment. This needs to be done in the deployment transactions to avoiding front-running attacks. The amount needs to be high enough to reduce the rounding error.

Consider sending first 1000 wei of shares to the zero address. This will significantly increase the cost of the attack by forcing an attacker to pay 1000 times of the share price they want to set. For a well-intended user, 1000 wei of shares is a negligible amount that won’t diminish their share significantly.

Implement the concept of virtual shares, similar to the ERC4626 OZ contract.

More info about this concept here.

hvasconcelos (BakerFi) confirmed ickas (BakerFi) commented:

Fixed →

- https://github.com/baker-fi/bakerfi-contracts/pull/45

# [H-03] When harvesting a strategy and adjusting the debt, all the leftover collateral that is not used to swap the withdrawn collateral from Aave for WETH to repay the flashloan will be locked and lost in the Strategy contract

- **Contest:** BakerFi Invitational
- **Slug:** 2024-05-bakerfi-invitational
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-05-bakerfi-invitational
- **Source snapshot:** competitions/2024-05-bakerfi-invitational/final_report.html

Submitted by 0xStalin, also found by bin2chen and rvierdiiev

## Impact

Collateral can be locked and lost in the Strategy contract.

## Recommended Mitigation Steps

To address this problem, I’d recommend to apply the two below suggestions.

Do not set a hardcoded value for the pool fee when calling the UniQuoter, instead, send the same value of the configured pool ( swapFeeTier ).

Instead of doing the self transfer of the leftover collateral after the swap, opt to re-supply it to Aave. In this way, that leftover collateral can still be managed by the Strategy.

0xleastwood (judge) decreased severity to Medium and commented:

This seems to predominantly impact yield in two ways:

Harvest function fails to be callable, but users can still withdraw collateral.

Harvest does not fail but there is some value leakage that happens over time.

Neither of these impact user’s funds directly so medium severity seems right.

0xStalin (warden) commented:

Hello Judge @0xleastwood - I’d like to clarify the second point raised in your comment to downgrade the severity of this report to medium:

Harvest does not fail but there is some value leakage that happens over time.

Neither of these impact user’s funds directly so medium severity seems right.

Actually, when harvest does not fail, and causes the leftover collateral to be left sitting on the protocol, those funds are actually the funds deposited by the users. While it is true that the leakage happens over time, those funds are user funds, not only yield.

I’d like to ask if you could take a second look at your verdict for the severity of this report and if you would consider re-assigning the original severity based on this clarification.

0xleastwood (judge) increased severity to Medium and commented:

@0xStalin - I see what you mean, even though the amount is somewhat on the smaller side, a debt adjustment will leave some excess collateral stuck as it rebalances to maintain a target LTV.

hvasconcelos (BakerFi) confirmed ickas (BakerFi) commented:

Fixed →

- https://github.com/baker-fi/bakerfi-contracts/pull/42

# [H-04] Multiple swap lack slippage protection

- **Contest:** BakerFi Invitational
- **Slug:** 2024-05-bakerfi-invitational
- **Finding ID:** H-04
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-05-bakerfi-invitational
- **Source snapshot:** competitions/2024-05-bakerfi-invitational/final_report.html

Submitted by bin2chen, also found by bin2chen, 0xStalin, rvierdiiev, and t0x1c The current protocol requires swapping tokens in multiple places, such as weth -> ierc20A or ierc20A -> weth.

Primarily, these swaps are executed using the _swap() method.

function _swap ( ISwapHandler.SwapParams memory params ) internal override returns ( uint256 amountOut ) { if ( params.

underlyingIn == address ( 0 )) revert InvalidInputToken (); if ( params.

underlyingOut == address ( 0 )) revert InvalidOutputToken (); uint24 fee = params.

feeTier; if ( fee == 0 ) revert InvalidFeeTier (); // Exact Input if ( params.

mode == ISwapHandler.

SwapType.

EXACT_INPUT ) { amountOut = _uniRouter.

exactInputSingle ( IV3SwapRouter.

ExactInputSingleParams ({ tokenIn:

params.

underlyingIn, tokenOut:

params.

underlyingOut, amountIn:

params.

amountIn, @> amountOutMinimum:

0, //@audit miss set params.amountOut fee:

fee, recipient:

address ( this ), sqrtPriceLimitX96:

0 }) ); if ( amountOut == 0 ) { revert SwapFailed (); } emit Swap ( params.

underlyingIn, params.

underlyingOut, params.

amountIn, amountOut ); // Exact Output } else if ( params.

mode == ISwapHandler.

SwapType.

EXACT_OUTPUT ) { uint256 amountIn = _uniRouter.

exactOutputSingle ( IV3SwapRouter.

ExactOutputSingleParams ({ tokenIn:

params.

underlyingIn, tokenOut:

params.

underlyingOut, fee:

fee, recipient:

address ( this ), amountOut:

params.

amountOut, amountInMaximum:

params.

amountIn, sqrtPriceLimitX96:

0 }) ); if ( amountIn < params.

amountIn ) { IERC20 ( params.

underlyingIn ).

safeTransfer ( address ( this ), params.

amountIn - amountIn ); } emit Swap ( params.

underlyingIn, params.

underlyingOut, amountIn, params.

amountOut ); amountOut = params.

amountOut; } This method does not set amountOutMinimum.

And when call same miss set Amount Out.

abstract contract StrategyLeverage is function _convertFromWETH ( uint256 amount ) internal virtual returns ( uint256 ) { // 1. Swap WETH -> cbETH/wstETH/rETH return _swap ( ISwapHandler.

SwapParams ( wETHA (), // Asset In ierc20A (), // Asset Out ISwapHandler.

SwapType.

EXACT_INPUT, // Swap Mode amount, // Amount In //@audit miss slippage protection @> 0, // Amount Out _swapFeeTier, // Fee Pair Tier bytes ( "" ) // User Payload ) ); } These methods do not have slippage protection.

- https://docs.uniswap.org/contracts/v3/guides/swaps/single-swaps
amountOutMinimum: we are setting to zero, but this is a significant risk in production. For a real deployment, this value should be calculated using our SDK or an onchain price oracle - this helps protect against getting an unusually bad price for a trade due to a front running sandwich or another type of price manipulation Include： UseSwapper._swap() / _convertFromWETH() / _convertToWETH() / _payDebt()

## Impact

Front running sandwich or another type of price manipulation.

## Recommended Mitigation

_swap() need set amountOutMinimum = params.amountOut function _swap( ISwapHandler.SwapParams memory params ) internal override returns (uint256 amountOut) { if (params.underlyingIn == address(0)) revert InvalidInputToken(); if (params.underlyingOut == address(0)) revert InvalidOutputToken(); uint24 fee = params.feeTier; if (fee == 0) revert InvalidFeeTier(); // Exact Input if (params.mode == ISwapHandler.SwapType.EXACT_INPUT) { amountOut = _uniRouter.exactInputSingle( IV3SwapRouter.ExactInputSingleParams({ tokenIn: params.underlyingIn, tokenOut: params.underlyingOut, amountIn: params.amountIn, - amountOutMinimum: 0, + amountOutMinimum: params.amountOut fee: fee, recipient: address(this), sqrtPriceLimitX96: 0

}) ); if (amountOut == 0) { revert SwapFailed(); } Call _swap() need set params.amountOut calculating the allowed slippage value accurately.

hvasconcelos (BakerFi) confirmed ickas (BakerFi) commented:

Fixed →

- https://github.com/baker-fi/bakerfi-contracts/pull/41
Medium Risk Findings (8)

# [M-01] All supplied WETH to Aave as a deposit by a Strategy will be irrecoverable

- **Contest:** BakerFi Invitational
- **Slug:** 2024-05-bakerfi-invitational
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-05-bakerfi-invitational
- **Source snapshot:** competitions/2024-05-bakerfi-invitational/final_report.html

Submitted by 0xStalin, also found by zhaojie

## Impact

WETH supplied to Aave will be lost.

## Recommended Mitigation Steps

Instead of supplying the wethLefts, use the excess WETH to repay more WETH debt on Aave, in this way, those extra WETHs won’t be lost on Aave because the strategy doesn’t have any means to withdraw them.

By using the extra WETH to repay more debt, the loan to value is brought down even to a healthier level.

function _payDebt(uint256 debtAmount, uint256 fee) internal {...

//@audit => Instead of supplying WETH to Aave, use it to repay more debt if (wethLefts > 0) { - _supply(wETHA(), wethLefts); + _repay(wETHA(), wethLefts); } emit StrategyUndeploy(msg.sender, debtAmount); } hvasconcelos (BakerFi) confirmed ickas (BakerFi) commented:

Fixed →

- https://github.com/baker-fi/bakerfi-contracts/pull/42

# [M-02] Vault can be DoS

- **Contest:** BakerFi Invitational
- **Slug:** 2024-05-bakerfi-invitational
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-05-bakerfi-invitational
- **Source snapshot:** competitions/2024-05-bakerfi-invitational/final_report.html

Submitted by zhaojie, also found by 0xStalin and rvierdiiev

## Impact

When totalSupply = 0, the attacker donates 1wei token, causing the number of shares to remain 0 at deposit time.

## Recommended Mitigation Steps

function toBase(Rebase memory total, uint256 elastic,bool roundUp ) internal pure returns (uint256 base) { - if (total.elastic == 0) { + if (total.elastic == 0 || total.base == 0) { base = elastic; } else { //total.base = totalSupply; total.elastic = _totalAssets base = (elastic * total.base) / total.elastic; if (roundUp && (base * total.elastic) / total.base < elastic) { base++; } hvasconcelos (BakerFi) confirmed 0xleastwood (judge) decreased severity to Medium and commented:

Thinking about this more, continuous DoS of vault deployment only lasts until it is fixed and does not seem to have any impact on user funds. Downgrading to medium severity.

ickas (BakerFi) commented:

Fixed →

- https://github.com/baker-fi/bakerfi-contracts/pull/44

# [M-03] StrategyLeverage.harvest doesn’t account flashloan fee

- **Contest:** BakerFi Invitational
- **Slug:** 2024-05-bakerfi-invitational
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-05-bakerfi-invitational
- **Source snapshot:** competitions/2024-05-bakerfi-invitational/final_report.html

StrategyLeverage.harvest doesn’t account flashloan fee Submitted by rvierdiiev StrategyLeverage.harvest function checks position state. In case if position LTV is bigger than max LTV, then extra debt is repaid to decrease LTV back to normal.

In order to repay part of debt, flashloan is taken and contract should pay fee for it.

So overall after adjusting our debt is decreased with deltaDebt but our collateral is decreased with deltaDebt + fee.

The problem is that this is not reflected in the newDeployedAmount calculation as it thinks that both collateral and debt were decreased by deltaDebt.

As result of this, newDeployedAmount is bigger than it is in reality (in reality it is newDeployedAmount - fee ), which means that later when some profit accrued, protocol may not receive it. For example if profit is < fee, then protocol won’t receive it and if profit is > fee, then protocol will receive management fee based on profit - fee amount.

## Impact

Protocol may receive smaller amount of fees.

Tools Used VsCode

## Recommended Mitigation Steps

Make _adjustDebt returns fee as well and use it to decrease collateral.

hvasconcelos (BakerFi) confirmed ickas (BakerFi) commented:

Fixed →

- https://github.com/baker-fi/bakerfi-contracts/pull/48

# [M-04] deposit() afterDeposit calculation formula is incorrect

- **Contest:** BakerFi Invitational
- **Slug:** 2024-05-bakerfi-invitational
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-05-bakerfi-invitational
- **Source snapshot:** competitions/2024-05-bakerfi-invitational/final_report.html

deposit() afterDeposit calculation formula is incorrect Submitted by bin2chen, also found by bin2chen, 0xStalin, rvierdiiev, and zhaojie In Vault.deposit(), we will limit the user’s maximum deposit cannot exceed settings().getMaxDepositInETH().

function deposit ( address receiver ) external payable override nonReentrant whenNotPaused onlyWhiteListed returns ( uint256 shares ) { if ( msg.

value == 0 ) revert InvalidDepositAmount (); uint256 maxPriceAge = settings ().

getPriceMaxAge (); Rebase memory total = Rebase ( _totalAssets ( maxPriceAge ), totalSupply ()); if ( // Or the Rebase is unititialized !(( total.

elastic == 0 && total.

base == 0 ) || // Or Both are positive ( total.

base > 0 && total.

elastic > 0 )) ) revert InvalidAssetsState (); // Verify if the Deposit Value exceeds the maximum per wallet uint256 maxDeposit = settings ().

getMaxDepositInETH (); if ( maxDeposit > 0 ) { uint256 afterDeposit = msg.

value + @> (( balanceOf ( msg.

sender ) * _tokenPerETH ( maxPriceAge )) / 1e18 ); if ( afterDeposit > maxDeposit ) revert MaxDepositReached (); }....

function _tokenPerETH ( uint256 priceMaxAge ) internal view returns ( uint256 ) { uint256 position = _totalAssets ( priceMaxAge ); if ( totalSupply () == 0 || position == 0 ) { return 1 ether; } @> return ( totalSupply () * 1 ether ) / position; } The code above uses (balanceOf(msg.sender) * _tokenPerETH(maxPriceAge) / 1e18 to calculate the current ETH deposit.

Based on the definition of the _tokenPerETH() method, this formula is incorrect.

It should be balanceOf(msg.sender) * 1e18 / _tokenPerETH(maxPriceAge).

## Impact

An incorrect calculation formula can result in exceeding getMaxDepositInETH or prematurely triggering a MaxDepositReached revert. Users may not be able to deposit properly.

## Recommended Mitigation

function deposit( address receiver )...

// Verify if the Deposit Value exceeds the maximum per wallet uint256 maxDeposit = settings().getMaxDepositInETH(); if (maxDeposit > 0) { uint256 afterDeposit = msg.value + - ((balanceOf(msg.sender) * _tokenPerETH(maxPriceAge)) / 1e18); + ((balanceOf(msg.sender) * 1e18) / _tokenPerETH(maxPriceAge)); if (afterDeposit > maxDeposit) revert MaxDepositReached(); } hvasconcelos (BakerFi) confirmed ickas (BakerFi) commented:

Fixed →

- https://github.com/baker-fi/bakerfi-contracts/pull/49

# [M-05] Protocol receives less harvest fees

- **Contest:** BakerFi Invitational
- **Slug:** 2024-05-bakerfi-invitational
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-05-bakerfi-invitational
- **Source snapshot:** competitions/2024-05-bakerfi-invitational/final_report.html

Submitted by rvierdiiev In case position has grown, then protocol receives performance fee.

- https://github.com/code-423n4/2024-05-bakerfi/blob/main/contracts/core/Vault.sol#L153-L158
uint256 feeInEthScaled = uint256 ( balanceChange ) * settings ().

getPerformanceFee (); uint256 sharesToMint = ( feeInEthScaled * totalSupply ()) / _totalAssets ( maxPriceAge ) / PERCENTAGE_PRECISION; _mint ( settings ().

getFeeReceiver (), sharesToMint ); We will check how shares amount is calculated and why it’s less than it should be.

Suppose that totalSupply() == 100000 and _totalAssets(maxPriceAge) == 100100, so we earned 100 eth as additional profit.

balanceChange == 100 and performance fee is 10%, which is 10 eth.

sharesToMint = 10 * 100000 / 100100 = 9.99001 This means that with 9.990001 shares protocol should be able to grab 10 eth fee, which is indeed like that if we convert 9.99001 * 100100 / 100000 = 10.

The problem is that minting is done later, which means that totalSupply() will increase with 9.99001 shares. So if we calculate fees amount now we will get a smaller amount:

9.99001 * 100100 / 100009.99001 = 9.999001

## Impact

Protocol receives smaller amount of fees.

Tools Used VsCode

## Recommended Mitigation Steps

The formula should be adjusted to count increase of total supply.

hvasconcelos (BakerFi) acknowledged

# [M-06] Min and maxAnswer never checked for oracle price feed

- **Contest:** BakerFi Invitational
- **Slug:** 2024-05-bakerfi-invitational
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-05-bakerfi-invitational
- **Source snapshot:** competitions/2024-05-bakerfi-invitational/final_report.html

Submitted by t0x1c Chainlink aggregators have a built-in circuit breaker if the price of an asset goes outside of a predetermined price band. The result is that if an asset experiences a huge drop in value (i.e. LUNA crash) the price of the oracle will continue to return the minPrice instead of the actual price of the asset. This would allow user to continue borrowing with the asset but at the wrong price. This is exactly what happened to Venus on BSC when LUNA imploded. However, the protocol misses to implement such a check.

Link to code:

function getLatestPrice () public view override returns (IOracle.Price memory price ) { @--> (, int256 answer, uint256 startedAt, uint256 updatedAt,) = _ethPriceFeed.

latestRoundData (); if ( answer <= 0 ) revert InvalidPriceFromOracle (); if ( startedAt == 0 || updatedAt == 0 ) revert InvalidPriceUpdatedAt (); price.

price = uint256 ( answer ); price.

lastUpdate = updatedAt; } Similar past issues Risk of Incorrect Asset Pricing by StableOracle in Case of Underlying Aggregator Reaching minAnswer ChainlinkAdapterOracle will return the wrong price for asset if underlying aggregator hits minAnswer

## Recommended Mitigation Steps

Add logic along the lines of:

require ( answer >= minPrice && answer <= maxPrice, "invalid price" ); Min and max prices can be gathered using one of these ways.

hvasconcelos (BakerFi) confirmed ickas (BakerFi) commented:

Fixed →

- https://github.com/baker-fi/bakerfi-contracts/pull/46

# [M-07] Rounding-down of flashFee can result in calls to flash loan to revert

- **Contest:** BakerFi Invitational
- **Slug:** 2024-05-bakerfi-invitational
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-05-bakerfi-invitational
- **Source snapshot:** competitions/2024-05-bakerfi-invitational/final_report.html

flashFee can result in calls to flash loan to revert Submitted by t0x1c, also found by bin2chen The BalancerFlashLender::flashFee() function returns a rounded-down fee. This fee is then later used across the protocol while providing spend approval to the flash loan provider. This can result in an approval less than that expected by the provider and hence cause the call to flash loan to revert. This is because flash loan providers calculate their fee by rounding up in their favour, instead of rounding down:

File:

contracts / core / flashloan / BalancerFlashLender.

sol function flashFee ( address, uint256 amount ) external view override returns ( uint256 ) { uint256 perc = _balancerVault.

getProtocolFeesCollector ().

getFlashLoanFeePercentage (); if ( perc == 0 || amount == 0 ) { return 0; } @---> return ( amount * perc ) / _BALANCER_MAX_FEE_PERCENTAGE; } and File:

contracts / core / strategies / StrategyLeverage.

sol function deploy () external payable onlyOwner nonReentrant returns ( uint256 deployedAmount ) { if ( msg.

value == 0 ) revert InvalidDeployAmount (); // 1. Wrap Ethereum address ( wETHA ()).

functionCallWithValue ( abi.

encodeWithSignature ( "deposit()" ), msg.

value ); // 2. Initiate a WETH Flash Loan uint256 leverage = calculateLeverageRatio ( msg.

value, getLoanToValue (), getNrLoops () ); uint256 loanAmount = leverage - msg.

value; @---> uint256 fee = flashLender ().

flashFee ( wETHA (), loanAmount ); //§uint256 allowance = wETH().allowance(address(this), flashLenderA()); @---> if (!

wETH ().

approve ( flashLenderA (), loanAmount + fee )) revert FailedToApproveAllowance (); if ( !

flashLender ().

flashLoan ( IERC3156FlashBorrowerUpgradeable ( this ), wETHA (), loanAmount, abi.

encode ( msg.

value, msg.

sender, FlashLoanAction.

SUPPLY_BOORROW ) ) { revert FailedToRunFlashLoan (); } deployedAmount = _pendingAmount; _deployedAmount = _deployedAmount + deployedAmount; emit StrategyAmountUpdate ( _deployedAmount ); // Pending amount is not cleared to save gas // _pendingAmount = 0; }

## Impact

Flash loan call reverts for many amount and fee percentage combinations.

## Recommended Mitigation Steps

Round up in favour of the protocol. A library like solmate can be used which has mulDivUp:

function flashFee(address, uint256 amount) external view override returns (uint256) { uint256 perc = _balancerVault.getProtocolFeesCollector().getFlashLoanFeePercentage(); if (perc == 0 || amount == 0) { return 0; } - return (amount * perc) / _BALANCER_MAX_FEE_PERCENTAGE; + return amount.mulDivUp(perc, _BALANCER_MAX_FEE_PERCENTAGE); } hvasconcelos (BakerFi) confirmed ickas (BakerFi) commented:

Fixed →

- https://github.com/baker-fi/bakerfi-contracts/pull/47

# [M-08] BalancerFlashLender#receiveFlashLoan does not validate the originalCallData

- **Contest:** BakerFi Invitational
- **Slug:** 2024-05-bakerfi-invitational
- **Finding ID:** M-08
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-05-bakerfi-invitational
- **Source snapshot:** competitions/2024-05-bakerfi-invitational/final_report.html

BalancerFlashLender#receiveFlashLoan does not validate the originalCallData Submitted by zhaojie, also found by rvierdiiev

## Impact

receiveFlashLoan does not validate the originalCallData. The attacker can pass any parameters into the receiveFlashLoan function and execute any Strategy instruction:

_supplyBorrow _repayAndWithdraw _payDeb.

## Recommended Mitigation Steps

BalancerFlashLender#flashLoan function to record the parameters called via hash.

Verify the hash value in the receiveFlashLoan function.

hvasconcelos (BakerFi) confirmed 0xleastwood (judge) decreased severity to Medium and commented:

After further discussion, I agree that _repayAndWithdraw() would fail when a zero amount repayment is made, so the only action that is really possible is _supplyBorrow() which would require some funds to already be in the contract. Unlikely for this to ever be the case because the contract doesn’t normally hold funds that aren’t being put to use in some way, but do correct me if this assumption is incorrect.

So I’m not sure how this issue can be exploited even if I do agree that it is an issue. For the time being, I will downgrade this to medium severity because it is obvious this is not intended behavour even if it may not lead to funds being lost.

0xleastwood (judge) commented:

This is clearly unintended behaviour and should be fixed even if it is not currently exploitable. I believe medium severity is still justified.

ickas (BakerFi) commented:

Fixed →

- https://github.com/baker-fi/bakerfi-contracts/pull/50
Note: for full discussion, please see the warden’s original submission.

## Rejected Primary Findings

# Rejected Primary Findings: BakerFi Invitational

No rejected primary findings were captured for this contest in the current corpus.

- **Rejected primary count:** 0
- **Primary submission rows captured:** 0
- **Submission status:** requires_authentication
