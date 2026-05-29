# Benchmark Ground Truth: Lambo.win

## Accepted H/M Findings

# Accepted H/M Findings: Lambo.win

# [H-01] Loss of User Funds in VirtualToken’s cashIn Function Due to Incorrect Amount Minting

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-12-lambowin
- **Source snapshot:** competitions/2024-12-lambowin/final_report.html

cashIn Function Due to Incorrect Amount Minting Submitted by aldarion, also found by 056Security, 0xaudron, 0xbrett8571, 0xGondar, 0xgremlincat, 0xiehnnkta, 0xiehnnkta, 0xKann, 0xLasadie, 0xleadwizard, 0xLeveler, 0xMitev, 0xMosh, 4B, 4rdiii, Agontuk, Akay, anonymousjoe, ast3ros, aster, aua_oo7, Bauchibred, BenRai, BenRai, Bryan_Conquer, bumbleb33, c0pp3rscr3w3r, chaduke, Coldless, Coldless, CrazyMoose, crmx_lom, dd0x7e8, dhank, DharkArtz, dic0de, EchoKly, eLSeR17, EPSec, ETHworker, Evo, FalseGenius, farismaulana, favelanky, Fitro, Fon, franfran20, gkrastenov, Gosho, harry_cryptodev, honey-k12, hyuunn, icy_petal, Infect3d, inh3l, IzuMan,

jaraxxus, jesusrod15, Jiri123, jkk812812, John_Femi, jrstrunk, jyjh, KiteWeb3, KKaminsk, komorebi, KupiaSec, lanyi2023, Le_Rems, Le_Rems, LeFy, LordAdhaar, m4k2, m4k2, macart224, Matin, mgf15, montecristo, Moyinmaala, MrPotatoMagic, mrudenko, newspacexyz, NexusAudits, OpaBatyo, Oxsadeeq, parishill24, pfapostol, pontifex, prapandey031, Prosperity, PumpkingWok, rare_one, Rhaydden, rilwan99, Robinx33, rouhsamad, rspadi, saikumar279, Shubham, silver_eth, Silverwind, slowbugmayor, SpicyMeatball, Stingo, stuart_the_minion, Summer, TenderBeastJr, threadmodeling, tpiliposian, tusharr1411, Tychai0s, typicalHuman, udo, Vagabond, Vasquez, viking71, vladi319, web3km, willycode20, X0sauce, xiao, YoanYJD, zaevlad, zaevlad, ZhengZuo999, zxriptor, and zzebra83

- https://github.com/code-423n4/2024-12-lambowin/blob/874fafc7b27042c59bdd765073f5e412a3b79192/src/VirtualToken.sol#L78
In the VirtualToken contract cashIn() function uses msg.value instead of amount for minting tokens when dealing with ERC20 tokens. This causes users to lose their deposited ERC20 tokens as they receive 0 virtual tokens in return.

## Recommended mitigation steps

function cashIn ( uint256 amount ) external payable onlyWhiteListed { if ( underlyingToken == LaunchPadUtils.

NATIVE_TOKEN ) { require ( msg.

value == amount, "Invalid ETH amount" ); } else { _transferAssetFromUser ( amount ); } _mint ( msg.

sender, amount ); // Use amount instead of msg.value } Shaneson (Lambo.win) confirmed and commented:

VirtualToken should support USDT, USDC in the future, so cashIn should use amount instead of msg.value. This is the fixed PR, please review.

# [H-02] LamboFactory can be permanently DoS-ed due to createPair call reversal

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-12-lambowin
- **Source snapshot:** competitions/2024-12-lambowin/final_report.html

createPair call reversal Submitted by zxriptor, also found by ast3ros, Evo, FalseGenius, Giorgio, Infect3d, inh3l, Le_Rems, m4k2, mrudenko, paco, rouhsamad, shaflow2, SpicyMeatball, TheFabled, threadmodeling, and web3km

- https://github.com/code-423n4/2024-12-lambowin/blob/main/src/LamboFactory.sol#L72
LamboFactory.createLaunchPad deploys new token contract and immediately sets up a new Uniswap V2 pool by calling createPair. This can be frontrun by the attacker by setting up a pool for the next token to be deployed.

Contract addresses are deterministic and can be calculated in advance. That opens a possibility for the attacker to pre-calculate the address of the next LamboToken to be deployed. As can be seen below, LamboFactory uses clone () method from OpenZeppelin Clones library, which uses CREATE EMV opCode under the hood.

function _deployLamboToken ( string memory name, string memory tickname ) internal returns ( address quoteToken ) { // Create a deterministic clone of the LamboToken implementation >>> quoteToken = Clones.

clone ( lamboTokenImplementation ); // Initialize the cloned LamboToken LamboToken ( quoteToken ).

initialize ( name, tickname ); emit TokenDeployed ( quoteToken ); } CREATE opcode calculates new contract address based on factory contract address and nonce (number of deployed contracts the factory has previously deployed):

The destination address is calculated as the rightmost 20 bytes (160 bits) of the Keccak-256 hash of the rlp encoding of the sender address followed by its nonce. That is: address = keccak256(rlp([sender address,sender nonce]))[12:]

- https://www.evm.codes/#f0
Hence an attacker can calculate the address of the next token to be deployed and directly call UniswapV2Factory.createPair which will result in a new liquidity pool being created BEFORE the token has been deployed.

Such state will lead all subsequent calls to LamboFactory.createLaunchPad to revert, because of the pair existence check in Uniswap code, without the possibility to fix that:

- https://github.com/Uniswap/v2-core/blob/master/contracts/UniswapV2Factory.sol#L27
function createPair ( address tokenA, address tokenB ) external returns ( address pair ) { require ( tokenA != tokenB, 'UniswapV2: IDENTICAL_ADDRESSES' ); ( address token0, address token1 ) = tokenA < tokenB ? ( tokenA, tokenB ): ( tokenB, tokenA ); require ( token0 != address ( 0 ), 'UniswapV2: ZERO_ADDRESS' ); >>> require ( getPair [ token0 ][ token1 ] == address ( 0 ), 'UniswapV2: PAIR_EXISTS' ); // single check is sufficient //... the rest of the code is ommitted...

}

## Recommended mitigation steps

Check pool existence using IUniswapV2Factory.getPair().

Shaneson (Lambo.win) commented:

We would use cloneDeterministic instead of clone, and the backend will pass the random salt from off-chain.

And this is the fixed PR.

Koolex (judge) commented:

I believe with this fix, front-run can still be done. It is better to check if the pair exists, then simply don’t create it. This way, there is zero DoS.

# [H-03] Calculation for directionMask is incorrect

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-12-lambowin
- **Source snapshot:** competitions/2024-12-lambowin/final_report.html

directionMask is incorrect Submitted by 0xleadwizard, also found by Agontuk, BenRai, Infect3d, Jiri123, NexusAudits, Rhaydden, rouhsamad, SpicyMeatball, and ZhengZuo999

- https://github.com/code-423n4/2024-12-lambowin/blob/main/src/rebalance/LamboRebalanceOnUniwap.sol#L165
The _getQuoteAndDirection function’s flawed logic can cause incorrect direction determination in the UniswapV3 pool. The recommended mitigation ensures that the function dynamically identifies token0 and token1 and assigns the correct direction mask. This prevents potential financial losses and ensures accurate rebalancing.

## Finding description and impact

The function previewRebalance is called off-chain, to calculate values that can be passed to the function rebalance when making a call for balancing the uniswapV3 vETH/WETH pool.

function previewRebalance () public view returns ( bool result, uint256 directionMask, uint256 amountIn, uint256 amountOut ) { address tokenIn; address tokenOut; ( tokenIn, tokenOut, amountIn ) = _getTokenInOut (); ( amountOut, directionMask ) = _getQuoteAndDirection ( tokenIn, tokenOut, amountIn ); result = amountOut > amountIn; } The function _getQuoteAndDirection takes tokenIn, tokenOut & amountIn as parameter to output amountOut & directionMask.

directionMask is used to decide if the swap is zero-for-one or one-for-zero in OKX.

The _getQuoteAndDirection function assumes that WETH is always token1, which can lead to incorrect direction determination in cases where WETH is actually token0. This is due to the fact that Uniswap sorts token0 and token1 lexicographically by their addresses, and not based on their logical roles.

function _getQuoteAndDirection ( address tokenIn, address tokenOut, uint256 amountIn ) internal view returns ( uint256 amountOut, uint256 directionMask ) { ( amountOut,,, ) = IQuoter ( quoter ).

quoteExactInputSingleWithPool ( IQuoter.

QuoteExactInputSingleWithPoolParams ({ tokenIn:

tokenIn, tokenOut:

tokenOut, amountIn:

amountIn, fee:

fee, pool:

uniswapPool, sqrtPriceLimitX96:

0 }) ); >> directionMask = ( tokenIn == weth ) ?

_BUY_MASK:

_SELL_MASK; } Example: If the UniswapV3 pool has token0 as WETH (lower address value) and token1 as vETH (higher address value), and the pool has more vETH than WETH, the tokenIn will be WETH. However, because WETH is token0 in this case, the correct direction would be zero-for-one. The current logic mistakenly assumes WETH is token1, leading to an incorrect direction of one-for-zero.

For context, here is how the MASK is used in OKX:

MASK defined uint256 private constant _ONE_FOR_ZERO_MASK = 1 << 255; // Mask for identifying if the swap is one-for-zero MASK used let zeroForOne:= eq ( and ( _pool, _ONE_FOR_ZERO_MASK ), 0 )

## Recommended mitigation steps

Add the logic for considering if the tokenIn is token0 or token1.

function _getQuoteAndDirection ( address tokenIn, address tokenOut, uint256 amountIn ) internal view returns ( uint256 amountOut, uint256 directionMask ) { // Retrieve token0 and token1 from the Uniswap pool address token0 = IUniswapV3Pool ( uniswapPool ).

token0 (); address token1 = IUniswapV3Pool ( uniswapPool ).

token1 (); // Call the quoter to get the amountOut ( amountOut,,, ) = IQuoter ( quoter ).

quoteExactInputSingleWithPool ( IQuoter.

QuoteExactInputSingleWithPoolParams ({ tokenIn:

tokenIn, tokenOut:

tokenOut, amountIn:

amountIn, fee:

fee, pool:

uniswapPool, sqrtPriceLimitX96:

0 }) ); // Determine directionMask based on tokenIn position (token0 or token1) if ( tokenIn == token0 ) { directionMask = _SELL_MASK; // Zero-for-one direction } else { directionMask = _BUY_MASK; // One-for-zero direction } Shaneson (Lambo.win) acknowledged and commented:

When the VETH is deployed, the direction will be updated. But yes, this is still a good suggestion.

# [H-04] Anyone can call LamboRebalanceOnUniwap.sol::rebalance() function with any arbitrary value, leading to rebalancing goal i.e. (1:1 peg) unsuccessful.

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Finding ID:** H-04
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-12-lambowin
- **Source snapshot:** competitions/2024-12-lambowin/final_report.html

LamboRebalanceOnUniwap.sol::rebalance() function with any arbitrary value, leading to rebalancing goal i.e. (1:1 peg) unsuccessful.

Submitted by orangesantra, also found by EPSec and Evo Anyone can call LamboRebalanceOnUniwap.sol::rebalance() function with any arbitrary value, leading to rebalancing goal i.e. (1:1 peg) unsuccessful.

The parameters required in rebalance() function will are, uint256 directionMask, uint256 amountIn, uint256 amountOut. The typical value should be - directionMask = 0 or 1<<255 amountIn and amountOut obtained from LamboRebalanceOnUniwap.sol::previewRebalance() But since there is no check, to ensure the typical values of parameter in the function, this can cause the flashloan for wrong amount or flashloan reverting if directionMask is any other value apart from 0 or 1<<255.

If flashloan of wrong amount occurs it means the pool will be unbalanced again with different value instead of balancing.

## Recommended mitigation steps

Check the parameter of rebalance() function whether they are legit or not, i.e. as per flashloan requirement.

Shaneson (Lambo.win) acknowledged Medium Risk Findings (10)

# [M-01] Since the cost of launching a new pool is minimal, an attacker can maliciously consume VirtualTokens

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-lambowin
- **Source snapshot:** competitions/2024-12-lambowin/final_report.html

VirtualTokens Submitted by shaflow2, also found by 0xD4n13l, 0xGondar, c0pp3rscr3w3r, Coldless, EPSec, Evo, farismaulana, Fitro, Fon, Infect3d, jaraxxus, Jiri123, jkk812812, kodyvim, Le_Rems, m4k2, macart224, MrPotatoMagic, Mushow, NexusAudits, NexusAudits, parishill24, pontifex, prapandey031, rouhsamad, rspadi, threadmodeling, Tychai0s, typicalHuman, Vasquez, zxriptor, and zzebra83 When launching a new pool, the factory contract needs to call the takeLoan function to intervene with virtual liquidity. The amount that can be borrowed is limited to 300 ether per block.

github:

- https://github.com/code-423n4/2024-12-lambowin/blob/874fafc7b27042c59bdd765073f5e412a3b79192/src/VirtualToken.sol#L93
function takeLoan ( address to, uint256 amount ) external payable onlyValidFactory { if ( block.

number > lastLoanBlock ) { lastLoanBlock = block.

number; loanedAmountThisBlock = 0; } require ( loanedAmountThisBlock + amount <= MAX_LOAN_PER_BLOCK, "Loan limit per block exceeded" ); loanedAmountThisBlock += amount; _mint ( to, amount ); _increaseDebt ( to, amount ); emit LoanTaken ( to, amount ); } However, when launching a new pool, the amount of virtual liquidity is controlled by users, and the minimum cost to launch a new pool is very low, requiring only gas fees and a small buy-in fee. This allows attackers to launch malicious new pools in each block, consuming the borrowing limit, which prevents legitimate users from launching new pools.

## Recommended mitigation steps

Charge a launch fee for new pools to increase attack costs Limit the maximum virtual liquidity a user can consume per transaction Shaneson (Lambo.win) acknowledged

# [M-02] LamboRebalanceOnUniswap::_getTokenInOut formula used to compute rebalancing amount is wrong for a UniV3 pool

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-lambowin
- **Source snapshot:** competitions/2024-12-lambowin/final_report.html

LamboRebalanceOnUniswap::_getTokenInOut formula used to compute rebalancing amount is wrong for a UniV3 pool Submitted by Infect3d, also found by EPSec, ka14ar, King_9aimon, KupiaSec, and pontifex

- https://github.com/code-423n4/2024-12-lambowin/blob/b8b8b0b1d7c9733a7bd9536e027886adb78ff83a/src/rebalance/LamboRebalanceOnUniwap.sol#L116-L148
The formula implemented assumes that the pool is based on a constant sum AMM formula ( x+y = k ), and also eludes the fact that reserves in a UniV3 pool do not directly relate to the price because of the 1-sided ticks liquidity.

This make the function imprecise at best, and highly imprecise when liquidity is deposited in distant ticks, with no risk involved for actors depositing in those ticks.

## Vulnerability details

The previewRebalance function has been developed to output all the necessary input parameters required to call the rebalance function, which goal is to swap tokens in order to keep the peg of the virtual token in comparison to its counterpart (e.g keep vETH/ETH prices = 1):

File:

src / rebalance / LamboRebalanceOnUniwap.

sol 128:

function _getTokenBalances () internal view returns ( uint256 wethBalance, uint256 vethBalance ) { 129:

wethBalance = IERC20 ( weth ).

balanceOf ( uniswapPool ); <<❌( 1 ) this does not represent the active tick 130:

vethBalance = IERC20 ( veth ).

balanceOf ( uniswapPool ); 131: } 132:

133:

function _getTokenInOut () internal view returns ( address tokenIn, address tokenOut, uint256 amountIn ) { 134: ( uint256 wethBalance, uint256 vethBalance ) = _getTokenBalances (); 135:

uint256 targetBalance = ( wethBalance + vethBalance ) / 2; <<❌( 2 ) wrong formula 136:

137:

if ( vethBalance > targetBalance ) { 138:

amountIn = vethBalance - targetBalance; 139:

tokenIn = weth; 140:

tokenOut = veth; 141: } else { 142:

amountIn = wethBalance - targetBalance; 143:

tokenIn = veth; 144:

tokenOut = weth; 145: } 146:

147:

require ( amountIn > 0, "amountIn must be greater than zero" ); 148: } 149:

The implemented formula is incorrect, as it will not rebalance the pool for 2 reasons:

In Uniswap V3, LPs can deposit tokens in any ticks they want, even though those ticks are not active and do not participate to the actual price. But those tokens will be held by the pool, and thus be measured by _getTokenBalances The formula used to compute the targetBalance is incorrect because of how works the constant product formula x*y=k Regarding (2), consider this situation: WETH balance: 1000 vETH balance: 900 targetBalance = (1000 + 900) / 2 = 950 amountIn = 1000 - 950 = 50 (vETH) Swapping 50 vETH into the pool will not return 50 WETH because of the inherent slippage of the constant product formula.

Now, add to this bullet (1), and the measured balance will be wrong anyway because of the liquidity deposited in inactive ticks, making the result even more shifted from the optimal result.

## Impact

The function is not performing as intended, leading to invalid results which complicates the computation of rebalancing amounts necessary to maintain the peg.

Since this function is important to maintain the health of vETH as it has access to on-chain values, allowing precise rebalancing, failing to devise and implement a reliable solution for rebalancing before launch could result in significant issues.

## Recommended Mitigation Steps

Reconsider the computations of rebalancing amounts for a more precise one if keeping a 1:1 peg is important.

You might want to get inspiration from USSDRebalancer::rebalance().

Shaneson (Lambo.win) acknowledged

# [M-03] sellQuote and buyQuote are missing deadline check in LamboVEthRouter

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-lambowin
- **Source snapshot:** competitions/2024-12-lambowin/final_report.html

sellQuote and buyQuote are missing deadline check in LamboVEthRouter Submitted by Infect3d, also found by 0xDemon, Bryan_Conquer, Evo, hyuunn, SpicyMeatball, KupiaSec, NexusAudits, OpaBatyo, and pumba

- https://github.com/code-423n4/2024-12-lambowin/blob/main/src/LamboVEthRouter.sol#L102-L102
- https://github.com/code-423n4/2024-12-lambowin/blob/main/src/LamboVEthRouter.sol#L148-L148
sellQuote and buyQuote are missing deadline check in LamboVEthRouter.

Because of that, transactions can still be stuck in the mempool and be executed a long time after the transaction is initially called. During this time, the price in the Uniswap pool can change. In this case, the slippage parameters can become outdated and the swap will become vulnerable to sandwich attacks.

## Vulnerability details

The protocol has made the choice to develop its own router to swap tokens for users, which imply calling the low level UniswapV2Pair::swap function:

// this low-level function should be called from a contract which performs important safety checks function swap(uint amount0Out, uint amount1Out, address to, bytes calldata data) external lock { require(amount0Out > 0 || amount1Out > 0, 'UniswapV2: INSUFFICIENT_OUTPUT_AMOUNT'); (uint112 _reserve0, uint112 _reserve1,) = getReserves(); // gas savings require(amount0Out < _reserve0 && amount1Out < _reserve1, 'UniswapV2: INSUFFICIENT_LIQUIDITY'); As the comment indicates, this function require important safety checks to be performed.

A good example of safe implementation of such call can be found in the UniswapV2Router02::swapExactTokensForTokens function:

function swapExactTokensForTokens( uint amountIn, uint amountOutMin, address[] calldata path, address to, uint deadline ) external virtual override ensure(deadline) returns (uint[] memory amounts) { amounts = UniswapV2Library.getAmountsOut(factory, amountIn, path); require(amounts[amounts.length - 1] >= amountOutMin, 'UniswapV2Router: INSUFFICIENT_OUTPUT_AMOUNT'); TransferHelper.safeTransferFrom( path[0], msg.sender, UniswapV2Library.pairFor(factory, path[0], path[1]), amounts[0] ); _swap(amounts, path, to); } As we can see, 2 safety parameters are present here:

amountOutMin and deadline.

Now, if we look at SellQuote ( buyQuote having the same issue):

File: src/LamboVEthRouter.sol 148: function _buyQuote(address quoteToken, uint256 amountXIn, uint256 minReturn) internal returns (uint256 amountYOut) { 149: require(msg.value >= amountXIn, "Insufficient msg.value"); 150:...:...: //* ---------- some code ---------- *//...:

168: require(amountYOut >= minReturn, "Insufficient output amount"); We can see that no deadline parameter is present.

## Impact

The transaction can still be stuck in the mempool and be executed a long time after the transaction is initially called. During this time, the price in the Uniswap pool can change. In this case, the slippage parameters can become outdated and the swap will become vulnerable to sandwich attacks.

## Recommended Mitigation Steps

Add a deadline parameter.

Shaneson (Lambo.win) acknowledged

# [M-04] Accumulated ETH in the LamboVEthRouter will be irretrievable

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-lambowin
- **Source snapshot:** competitions/2024-12-lambowin/final_report.html

Submitted by inh3l, also found by 0xLasadie, aua_oo7, bareli, bumbleb33, Daniel526, Daniel526, eta, Evo, gajiknownnothing, inh3l, Ryonen, Le_Rems, m4k2, mansa11, MrMatrix, phenom80, saikumar279, Shubham, and Vagabond

- https://github.com/code-423n4/2024-12-lambowin/blob/874fafc7b27042c59bdd765073f5e412a3b79192/src/LamboVEthRouter.sol#L179-L183
- https://github.com/code-423n4/2024-12-lambowin/blob/874fafc7b27042c59bdd765073f5e412a3b79192/src/LamboVEthRouter.sol#L188
Over time, ETH will be accumulated in the LamboVEthRouter and it will be irretrievable leading to loss of funds.

## Recommended mitigation steps

Include a sweep function in the contract, or refund actual excess amount to the users.

Shaneson (Lambo.win) acknowledged

# [M-05] Incorrect Struct Field and Hardcoded sqrtPriceLimitX96 in _getQuoteAndDirection

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-lambowin
- **Source snapshot:** competitions/2024-12-lambowin/final_report.html

sqrtPriceLimitX96 in _getQuoteAndDirection Submitted by Daniel526 The absence of a properly set sqrtPriceLimitX96 allows swaps to execute at prices far beyond expected limits, exposing the contract to unfavorable trade outcomes. The function is also likely to fail at runtime due to a mismatch in struct field names ( amountIn instead of amount ).

## Recommended mitigation steps

Update _getQuoteAndDirection to correctly reference the struct fields and provide configurable sqrtPriceLimitX96 if needed:

( amountOut,,, ) = IQuoter ( quoter ).

quoteExactInputSingleWithPool ( IQuoter.

QuoteExactInputSingleWithPoolParams ({ tokenIn:

tokenIn, tokenOut:

tokenOut, amount:

amountIn, // Correct field usage fee:

fee, pool:

uniswapPool, sqrtPriceLimitX96:

sqrtPriceLimitX96 // Example of allowing no limit }) ); Shaneson (Lambo.win) confirmed and commented:

Good suggestion.

Accept.

# [M-06] Attacker can capture VETH-WETH depeg profits through a malicious pool, rendering rebalancer useless if VETH Price > WETH Price

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-lambowin
- **Source snapshot:** competitions/2024-12-lambowin/final_report.html

VETH-WETH depeg profits through a malicious pool, rendering rebalancer useless if VETH Price > WETH Price Submitted by rouhsamad, also found by m4k2 and zaevlad

- https://github.com/code-423n4/2024-12-lambowin/blob/main/src/rebalance/LamboRebalanceOnUniwap.sol#L76
- https://github.com/code-423n4/2024-12-lambowin/blob/main/src/rebalance/LamboRebalanceOnUniwap.sol#L109-L114
LamboRebalanceOnUniwap::rebalance accepts a directionMask argument, an arbitrary uint256 mask. It “OR”s this mask with uniswapPool and passes it to the OKX Router to perform zeroForOne or oneForZero swaps (using the MSB).

function rebalance ( uint256 directionMask, uint256 amountIn, uint256 amountOut ) external nonReentrant { uint256 balanceBefore = IERC20 ( weth ).

balanceOf ( address ( this )); bytes memory data = abi.

encode ( directionMask, amountIn, amountOut ); IMorpho ( morphoVault ).

flashLoan ( weth, amountIn, data ); uint256 balanceAfter = IERC20 ( weth ).

balanceOf ( address ( this )); uint256 profit = balanceAfter - balanceBefore; require ( profit > 0, "No profit made" ); } However, this lets an attacker find a pool address with a malicious token. Attacker needs to find a pool with a malicious coin (discussed in PoC) so that given:

malicious_mask = malicious_pool_address & (~uniswapPool) we will have:

malicious_mask | uniswapPool = maliciousPool Which allows attacker to insert the malicious pool here by passing malicious_mask to rebalance function:

// given our malicious mask, _v3Pool is the desired pool uint256 _v3pool = uint256 ( uint160 ( uniswapPool )) | ( directionMask ); uint256 [] memory pools = new uint256 []( 1 ); pools [ 0 ] = _v3pool; Later, the first condition is not met since the directionMask is not equal to (1 << 255). The code goes to the second condition:

if ( directionMask == _BUY_MASK ) { _executeBuy ( amountIn, pools ); Second condition:

else { _executeSell ( amountIn, pools ); } _executeSell calls OKXRouter with newly minted VETH (which we received at a discount, if VETH is priced higher than WETH). It then sends VETH to the malicious pool and receives malicious tokens + flash-loaned amount + 1 wei as the profit. the fact that its possible to replace the uniswapPool with our desired malicious pool opens up an attack path which if carefully executed, gives attacker the opportunity to profit from WETH-VETH depeg, leaving Lambo.win no profits at all.

The only difficult part of this attack is that attacker needs to deploy the malicious coin (which is paired with VETH) at a specific address so that their v3 pair address satisfies this condition:

malicious_mask = malicious_pool_address & (~uniswapPool) malicious_mask | uniswapPool = maliciousPool Basically the malicious_pool_address must have at least the same bits as the uniswapPool so that we can use a malicious mask on it.

Finding an address to satisfy this is hard (but not impossible, given current hardware advancements). For the sake of this PoC to be runnable, I have assumed the address of uniswapPool is 0x0000000000000000000000000000000000000093, so that we can find a malicious pool address and token easily. The actual difficulty of this attack depends on the actual address of WETH-VETH pool; however, I have used a simpler address, just to show that the attack is possible, given enough time.

After attacker deployed the right contracts, he can use them to profit from WETH-VETH depeges forever (unless new rebalancer is created).

## Recommended mitigation steps

Make sure that directionMask is either (1 << 255) or 0.

Shaneson (Lambo.win) confirmed and commented:

Very good finding. Thanks, talented auditors.

# [M-07] Rebalance profit requirement prevents maintaining VETH/WETH peg

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-lambowin
- **Source snapshot:** competitions/2024-12-lambowin/final_report.html

Submitted by Evo

- https://github.com/code-423n4/2024-12-lambowin/blob/b8b8b0b1d7c9733a7bd9536e027886adb78ff83a/src/rebalance/LamboRebalanceOnUniwap.sol#L62
The profit > 0 requirement in the rebalance function actively prevents the protocol from maintaining the VETH/WETH 1:1 peg during unprofitable market conditions, when profit is ZERO.

## Recommended Mitigation Steps

Update the require(profit > 0) to require(profit >= 0).

Shaneson (Lambo.win) acknowledged

# [M-08] Users can prevent protocol from rebalancing for his gain and cause loss of funds for protocol and its users

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Finding ID:** M-08
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-lambowin
- **Source snapshot:** competitions/2024-12-lambowin/final_report.html

Submitted by mrMorningstar, also found by 0xGondar, bumbleb33, bumbleb33, and Evo

- https://github.com/code-423n4/2024-12-lambowin/blame/874fafc7b27042c59bdd765073f5e412a3b79192/src/rebalance/LamboRebalanceOnUniwap.sol#L62
The protocol have a vETH token that aims to be pegged to the ETH so the ration of vETH -> ETH = 1:1. When depeg happens the protocol can mitigate that via rebalance function in LamboRebalanceOnUniwap that looks like this:

function rebalance(uint256 directionMask, uint256 amountIn, uint256 amountOut) external nonReentrant { uint256 balanceBefore = IERC20(weth).balanceOf(address(this)); bytes memory data = abi.encode(directionMask, amountIn, amountOut); IMorpho(morphoVault).flashLoan(weth, amountIn, data); uint256 balanceAfter = IERC20(weth).balanceOf(address(this)); uint256 profit = balanceAfter - balanceBefore; require(profit > 0, "No profit made"); } This function is designed to rebalance ratio by taking a flashloan from MorphoVault, which will be used on UniswapV3 to make a swap, while at the same time make sure there is a profit for the caller which later can be transferred via the extractProfit function in the same contract.

## Impact

Repeg can be DoS-ed which will prevent the protocol from rebalancing and will incur loss of funds.

## Recommended mitigation steps

There is no easy solution but, one solution is to use previewRebalance in the rebalance function and compare the returned values and profitability with inputted amounts by the user so even if the attacker front-runs it, the function reverts even before flashloan is taken. That can save additional gas cost and it will make each time more expensive and unprofitable for the attacker to perform that attack. Also to include a mechanism that will account for the possibility of inflated vETH values in LamboVETHRouter.

# [M-09] Rebalance will be completely dossed if OKX commision rate goes beyond the fee limits

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Finding ID:** M-09
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-lambowin
- **Source snapshot:** competitions/2024-12-lambowin/final_report.html

Submitted by inh3l, also found by Bauchibred, Evo, and MSaptarshi

- https://github.com/code-423n4/2024-12-lambowin/blob/874fafc7b27042c59bdd765073f5e412a3b79192/src/rebalance/LamboRebalanceOnUniwap.sol#L89-L114
Rebalancing interacts with OKXRouter to swap weth for tokens in certain pools and vice versa. But OKXRouter may charge commissions on both the from token and to token, which reduces potential profit to be made from the rebalance operation, if high enough causes there to be no profit made, which will cause the operation to fail, or in extreme cases, if the commision is set beyond its expected limits, cause a permanent dos of the function.

# [M-10] LP for v3 pool of underlying tokens with decimals != 18 would have incorrect NFT metadata

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Finding ID:** M-10
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-lambowin
- **Source snapshot:** competitions/2024-12-lambowin/final_report.html

!= 18 would have incorrect NFT metadata Submitted by prapandey031, also found by Agontuk, bumbleb33, Coldless, Daniel526, MrPotatoMagic, and TenderBeastJr

- https://github.com/code-423n4/2024-12-lambowin/blob/main/src/VirtualToken.sol#L10
The VirtualToken.sol has a hardcoded decimal value of 18 even if the underlying token has a decimal value of 6 (for eg, USDC). This would not let the liquidity providers of the (vUSDC, USDC) uniswap v3 pool to get the correct metadata for their NFT liquidity position.

Impact: Correct NFT metadata is important for NFT marketplaces. Therefore the NFT functionality stands broken. The imapct is Medium.

Likelihood: The likelihood is High.

Therefore, the severity is Medium.

## Recommended mitigation steps

It is recommended to use the underlying token’s decimals in VirtualToken.sol.

## Rejected Primary Findings

# Rejected Primary Findings: Lambo.win

# Incorrect pool parameters when previewing rebalance via `previewRebalance (...)` function

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Submission:** F-146
- **Submitter:** dic0de
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-12-lambowin/submissions/F-146
- **Source snapshot:** competitions/2024-12-lambowin/submissions/raw/F-146.txt

## Brief Summary

The previewRebalance(...) function is used to preview the rebalance action. Probably this would be used off-chain to determine whether calling rebalance(...) is appropriate or not. However the function calls the internal function _getQuoteAndDirection(...) . The _getQuoteAndDirection(...) function uses LamboRebalanceOnUniswap contract fee amount and sets sqrtPriceLimitX96 as 0 instead of using the uniswapPool's parameters to get the most accurate quote as seen below: address tokenIn, address tokenOut, uint256 amountIn ) internal view returns (uint256 amountOut, uint256 directionMask) { (amountOut, , , ) = IQuoter(quoter).quoteExactInputSingleWithPool( IQuoter.QuoteExactInputSingleWithPoolPa...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# If underlying token is not NATIVE_TOKEN and ETH is sent to VirtualToken.sol, it will be locked

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Submission:** F-444
- **Submitter:** hubble
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-12-lambowin/submissions/F-444
- **Source snapshot:** competitions/2024-12-lambowin/submissions/raw/F-444.txt

## Brief Summary

In VirtualToken.sol, the function cashIn is payable. In the case where the underlying token is not LaunchPadUtils.NATIVE_TOKEN, and by user mistake, when using the functions createLaunchPadAndInitialBuy or buyQuote, which are payable and ETH is sent along with the transaction, the protocol does not give any error, and transaction succeeds. Its expected that this corner case is handled properly in the protocol and loss to user is avoided. Impact The ETH sent along with the transactions in the above situation is locked and cannot be recovered. Since this is user initiated mistake, filing this issue as Medium instead of High.

## Rejection Reason

Final severity/evaluation: Low; Valid Primary Sufficient

# Excessive Fee Rate Allows Complete Denial of Service to Traders

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Submission:** F-367
- **Submitter:** ChainSentry
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-12-lambowin/submissions/F-367
- **Source snapshot:** competitions/2024-12-lambowin/submissions/raw/F-367.txt

## Brief Summary

The contract owner can set the trading fee rate (feeRate) to 100%, causing all user inputs in trades (buying or selling) to be captured entirely as fees. This effectively nullifies any return to the user and turns the exchange into a black hole, discouraging or outright blocking normal usage of the protocol. In short, a malicious or compromised owner can at will halt all trading by making trades yield zero outputs, amounting to a Denial of Service (DoS). Impact: Denial of Service: By imposing a 100% fee, the protocol effectively stops functioning as a viable exchange since no user can make profitable trades. Centralization Risk: A malicious owner (or a compromised owner key) can unilaterall...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Incorrect Validation of vETH Underlying Token as ETH

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Submission:** F-447
- **Submitter:** 0xiehnnkta
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-12-lambowin/submissions/F-447
- **Source snapshot:** competitions/2024-12-lambowin/submissions/raw/F-447.txt

## Brief Summary

createLaunchPadAndInitialBuy function in lamboVEthRouter.sol contract reverts if the underlying token for vETH is an ERC20 token. Vulnerability Detail createLaunchPadAndInitialBuy function must check weather the underlying token of vETH is ETH or ERC20. If it is ERC20, then the function reverts when it calls the _buyQuote function as it primirily checks msg.value >= amountXIn. But here it is zero, so it reverts the function. function createLaunchPadAndInitialBuy( address lamboFactory, string memory name, string memory tickname, uint256 virtualLiquidityAmount, uint256 buyAmount ) public payable nonReentrant returns (address quoteToken, address pool, uint256 amountYOut) { require(VirtualToken...

## Rejection Reason

Final severity/evaluation: Low; Valid Primary Sufficient

# Possibility of `vETH` and `ETH` being depegged and causing huge losses for users

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Submission:** F-163
- **Submitter:** uuzall
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-lambowin/submissions/F-163
- **Source snapshot:** competitions/2024-12-lambowin/submissions/raw/F-163.txt

## Brief Summary

The protocol has a virtual token (henceforth called vETH) that should be pegged to the price of ETH. This is the foundation of the entire platform where they aim to provide liquidity through loans of this pegged vETH, this is also mentioned by the protocol in their docs: Undoubtedly, the 1:1 peg between ETH and vETH is the most crucial aspect of LamboV2. To maintain a ratio of 1:1 between vETH and ETH, they utilize a method they call a "rebalance" method. This essentially sells the commodity with the higher price for more of the lower priced tokens to make a profit, and keep the tokens pegged. The process to rebalance the tokens is to first call the LamboRebalanceOnUniwap.sol::previewRebala...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Incompatibility with Non-WETH ERC20 Tokens in LamboRebalanceOnUniwap and LamboVethRouter

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Submission:** F-377
- **Submitter:** Mushow
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** out_of_scope
- **Rejection category:** out_of_scope
- **Source URL:** https://code4rena.com/audits/2024-12-lambowin/submissions/F-377
- **Source snapshot:** competitions/2024-12-lambowin/submissions/raw/F-377.txt

## Brief Summary

Considering the USDC and USDT ERC20 Tokens have been added in scope. The currently provided contracts, including LamboRebalanceOnUniwap and LamboVethRouter contracts are designed with a heavy reliance on WETH (Wrapped Ether) and msg.value to handle Ether transactions. This reliance on WETH and Ether-specific mechanisms (like msg.value) creates a significant limitation when attempting to work with other ERC20 tokens such as USDC, USDT, or any other virtual token. For LamboRebalanceOnUniwap, the contract uses hardcoded WETH token addresses, and functions like _executeBuy and _executeSell depend on methods like IWETH(weth).deposit and IWETH(weth).withdraw, which are specific to WETH. Similarly...

## Rejection Reason

Marked out of scope in authenticated Code4rena submission detail/table.

# User can avoid paying protocol fee when using `LamboVEthRouter::createLaunchPadAndInitialBuy` by setting low `buyAmount`

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Submission:** F-305
- **Submitter:** farismaulana
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-12-lambowin/submissions/F-305
- **Source snapshot:** competitions/2024-12-lambowin/submissions/raw/F-305.txt

## Brief Summary

Because of lack minimum amount of initial buy when calling createLaunchPadAndInitialBuy, any user can set small amount of msg.value so the precision loss would incurr when calculating the formula below in the _buyQuote function: LamboVEthRouter.sol#L152 // handle fee uint256 fee = (amountXIn * feeRate) / feeDenominator; with feeRate = 100 and feeDenominator = 10000 as long as the amountXIn * feeRate < feeDenominator is true, then the amount of fee would be 0 because of the rounding down. this can be achieved if the amountXIn less than 100.

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Lack of incentive for users to rebalance positions

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Submission:** F-159
- **Submitter:** m4k2
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-12-lambowin/submissions/F-159
- **Source snapshot:** competitions/2024-12-lambowin/submissions/raw/F-159.txt

## Brief Summary

The rebalance() function in LamboRebalanceOnUniwap.sol allows anyone to call it and rebalance the Uniswap V3 position between WETH and vETH. However, there is no incentive mechanism in place to encourage users to actually perform this rebalancing. The function takes a flash loan from Morpho equal to amountIn, performs the specified swap (either buying vETH with WETH or selling vETH for WETH depending on directionMask), and then checks that the balance of WETH in the contract has increased, signifying a profit was made. However, since the caller of rebalance() does not receive any of this profit, there is little reason for them to spend the gas to execute this function in the first place. Th...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Rebalance operations vulnerable to MEV sandwich attacks due to insufficient profit threshold

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Submission:** F-288
- **Submitter:** bumbleb33
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-12-lambowin/submissions/F-288
- **Source snapshot:** competitions/2024-12-lambowin/submissions/raw/F-288.txt

## Brief Summary

The rebalance() function in LamboRebalanceOnUniwap.sol only checks for positive profit (profit > 0), making it vulnerable to MEV sandwich attacks that can extract the majority of arbitrage profits while still satisfying this minimal check. Vulnerability Details: The current implementation only verifies that some profit was made: function rebalance(uint256 directionMask, uint256 amountIn, uint256 amountOut) external nonReentrant { uint256 balanceBefore = IERC20(weth).balanceOf(address(this)); bytes memory data = abi.encode(directionMask, amountIn, amountOut); IMorpho(morphoVault).flashLoan(weth, amountIn, data); uint256 balanceAfter = IERC20(weth).balanceOf(address(this)); uint256 profit = b...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Lack of Flexibility in VirtualToken Contract for Supporting ETH and ERC20 Tokens

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Submission:** F-443
- **Submitter:** Coldless
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-12-lambowin/submissions/F-443
- **Source snapshot:** competitions/2024-12-lambowin/submissions/raw/F-443.txt

## Brief Summary

The VirtualToken contract is designed to handle ETH and specific ERC20 tokens based on an immutable underlyingToken variable. However, the current design introduces a significant limitation: the contract is unable to dynamically handle both ETH and multiple ERC20 tokens (e.g., USDC and USDT) in a single instance. The affected functions (cashIn, _transferAssetFromUser, and _transferAssetToUser) use conditional logic to handle ETH or ERC20 transfers, but the lack of flexibility creates a rigid structure that requires separate contract instances for each token. This design flaw has multiple impacts: Limited Scalability The protocol must deploy a new VirtualToken instance for each supported tok...

## Rejection Reason

Final severity/evaluation: Low; Valid Primary Sufficient

# Potential Loss of Eth when calling `cashIn (...)` function resulting to double spenditure

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Submission:** F-28
- **Submitter:** dic0de
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-lambowin/submissions/F-28
- **Source snapshot:** competitions/2024-12-lambowin/submissions/raw/F-28.txt

## Brief Summary

The cashIn(...) function allows for users to mint the VirtualToken by depositing an underlyingTokento the contract. The underlyingToken can either be NATIVE_TOKEN or an ERC20. Therefore, the cashIn(...) function is marked as payable to enable native token transfers. However, a user can pass msg.value value when the underlyingToken is an ERC20. This will transfer the native token to the contract as well as the underlying ERC20 token. Therefore, the user would have double spent and be minted the msg.value only.

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# `VirtualToken` accounting might break if `USDT` or `USDC` fees get activated

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Submission:** F-25
- **Submitter:** Infect3d
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-12-lambowin/submissions/F-25
- **Source snapshot:** competitions/2024-12-lambowin/submissions/raw/F-25.txt

## Brief Summary

While USDC and USDT fees are not activated right now, they can be activated at anytime. The issue is that cashIn and cashOut functions rely on the amount parameter to decide how much virtual counterpart to mint/burn to/from msg.sender. Doing so will break accounting.

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Lack of MEV bot incentives for rebalancing creates risk of protocol instability

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Submission:** F-287
- **Submitter:** bumbleb33
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-12-lambowin/submissions/F-287
- **Source snapshot:** competitions/2024-12-lambowin/submissions/raw/F-287.txt

## Brief Summary

The protocol relies on external actors (particularly MEV bots) to call the rebalance() function to maintain price stability, but provides no direct incentives for doing so. While MEV bots are typically efficient at capturing arbitrage opportunities, the current implementation requires them to actively call rebalance() instead of performing direct swaps, which adds complexity and gas costs without additional rewards. function rebalance(uint256 directionMask, uint256 amountIn, uint256 amountOut) external nonReentrant { uint256 balanceBefore = IERC20(weth).balanceOf(address(this)); bytes memory data = abi.encode(directionMask, amountIn, amountOut); IMorpho(morphoVault).flashLoan(weth, amountIn...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# LamboRebalanceOnUniwap lacks mechanism to execute unprofitable but necessary rebalances

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Submission:** F-155
- **Submitter:** Evo
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-12-lambowin/submissions/F-155
- **Source snapshot:** competitions/2024-12-lambowin/submissions/raw/F-155.txt

## Brief Summary

The protocol cannot maintain its core 1:1 VETH/WETH peg during market conditions that require unprofitable rebalancing, as the current implementation only allows profitable rebalances. This limitation could lead to sustained depegging events, undermining the protocol's primary purpose.

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Premature Rebalancing Triggered at Tick -40 or +40 Instead of -60 or +60

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Submission:** F-173
- **Submitter:** saikumar279
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-12-lambowin/submissions/F-173
- **Source snapshot:** competitions/2024-12-lambowin/submissions/raw/F-173.txt

## Brief Summary

The protocol specifies that rebalancing should be triggered when the price reaches tick -60 or +60(As attached in this picture https://github.com/code-423n4/2024-12-lambowin/tree/main#peg-and-repeg). However, the actual implementation triggers rebalancing prematurely at tick -40 or +40

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# There is currently no incentive to rebalance

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Submission:** F-95
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-12-lambowin/submissions/F-95
- **Source snapshot:** competitions/2024-12-lambowin/submissions/raw/F-95.txt

## Brief Summary

There is currently no incentive to rebalance

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Fee calculation allows fee bypass through small initial buy amounts in createLaunchPadAndInitialBuy

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Submission:** F-294
- **Submitter:** Evo
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-lambowin/submissions/F-294
- **Source snapshot:** competitions/2024-12-lambowin/submissions/raw/F-294.txt

## Brief Summary

Users can bypass fees when creating launchpads by specifying small initial buy amounts where integer division results in 0 fees. This impact is magnified when feeRate is lowered by the owner, allowing increasingly larger amounts to completely avoid fees during launchpad creation, resulting in loss of fee revenue for the protocol.

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# It is not possible to buy a quoteToken through LamboVEthRouter

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Submission:** F-446
- **Submitter:** crmx_lom
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-12-lambowin/submissions/F-446
- **Source snapshot:** competitions/2024-12-lambowin/submissions/raw/F-446.txt

## Brief Summary

When the VirtualToken::underlyingToken is not ETH but an ERC20 token, the user cannot buy a quoteToken using the LamboVEthRouter. The buy of the token takes place in the _buyQuote function of the LamboVEthRouter contract. function _buyQuote(address quoteToken, uint256 amountXIn, uint256 minReturn) internal returns (uint256 amountYOut) { @> require(msg.value >= amountXIn, "Insufficient msg.value"); // handle fee uint256 fee = (amountXIn * feeRate) / feeDenominator; amountXIn = amountXIn - fee; @> (bool success,) = payable(owner()).call{value: fee}(""); require(success, "Transfer to Owner failed"); ... } The function contains a check require(msg.value >= amountXIn, "Insufficient msg.value");...

## Rejection Reason

Final severity/evaluation: Low; Valid Primary Sufficient

# If underlying token is not NATIVE_TOKEN, the createLaunchPadAndInitialBuy() function is broken

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Submission:** F-445
- **Submitter:** hubble
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-12-lambowin/submissions/F-445
- **Source snapshot:** competitions/2024-12-lambowin/submissions/raw/F-445.txt

## Brief Summary

If the underlying token in VirtualToken.sol is not LaunchPadUtils.NATIVE_TOKEN, and other ERC20 token for example USDC is used, then the createLaunchPadAndInitialBuy() and buyQuote() functions in LamboVEthRouter.sol is broken, and will revert due to missing code. If the underlying token is any ERC20 token, then code for transfering the token from msg.sender into the LamboVEthRouter and its approval to VirtualToken is missing. Due to this both the above functions will fail, and the project cannot launch its meme token via virtual liquidity. Also there is missing code for processing the fees in the ERC20 token. Impact The primary function of launching meme token via virtual liquidity is broke...

## Rejection Reason

Final severity/evaluation: Low; Valid Primary Sufficient

# Improper Validation in `LamboVEthRouter::updateFeeRate` Allows Fee Misconfiguration, Breaking Swap Functionality in the `LamboVEthRouter::_buyQuote` function

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Submission:** F-98
- **Submitter:** deeney
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-12-lambowin/submissions/F-98
- **Source snapshot:** competitions/2024-12-lambowin/submissions/raw/F-98.txt

## Brief Summary

The updateFeeRate function lacks proper validation which allows setting the newFeeRate <= feeDenominator. This misconfiguration allows the fee to consume 100% of the input amount (amountXIn) during the _buyQuote function. As a result, the adjusted amountXIn becomes zero. Note the feeDenominator has a constant value of 10000 Write a detailed description of the root cause and impact(s) of this finding. Impact: if the owner update the feeRate to feeDenominator (100%) using the updateFeeRate function results in 100% of amountXIn being taken as a fee, leaving amountXIn = 0 for the swap. This causes the _buyQuote function to revert due to insufficient output (amountYOut < minReturn), effectively...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# No way to upgrade OKX router & approveProxy

- **Contest:** Lambo.win
- **Slug:** 2024-12-lambowin
- **Submission:** F-19
- **Submitter:** 0xleadwizard
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-12-lambowin/submissions/F-19
- **Source snapshot:** competitions/2024-12-lambowin/submissions/raw/F-19.txt

## Brief Summary

OKX router is getting used for swapping while rebalancing in the Rebalancer contract. The router address & the token proxy address are hard coded as constants but looking at the OKX documentation both addresses might need to be updated due to upgrades in future. The contract addresses of the DEX router and token approval may be subject to replacement due to contract upgrades. To ensure uninterrupted use of the API, we recommend using the contract addresses returned by the response parameters: /approve-transaction API and /swap API for approvals and transactions. source: https://www.okx.com/web3/build/docs/waas/dex-smart-contract This implies that any upgrade to the OKX contracts could rende...

## Rejection Reason

Final severity/evaluation: Low; Valid Insufficient
