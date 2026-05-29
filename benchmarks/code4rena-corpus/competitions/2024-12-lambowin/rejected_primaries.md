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
