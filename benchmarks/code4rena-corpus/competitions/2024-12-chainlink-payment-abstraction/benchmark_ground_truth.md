# Benchmark Ground Truth: Chainlink Payment Abstraction

## Accepted H/M Findings

# Accepted H/M Findings: Chainlink Payment Abstraction

No accepted High/Medium findings were parsed from a final report in the current corpus.

- **Accepted H/M count:** 0
- **Final report status:** http_404
- **Submission status:** captured_from_authenticated_browser

## Rejected Primary Findings

# Rejected Primary Findings: Chainlink Payment Abstraction

# Payment Abstraction System Will get completely DoS'ed if `8` tokens or more are allowlisted

- **Contest:** Chainlink Payment Abstraction
- **Slug:** 2024-12-chainlink-payment-abstraction
- **Submission:** F-116
- **Submitter:** Al-Qa-qa
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-chainlink-payment-abstraction/submissions/F-116
- **Source snapshot:** competitions/2024-12-chainlink-payment-abstraction/submissions/raw/F-116.txt

## Brief Summary

Payment System is working by first calling checkUpkeep() then do performUpKeep() using the returned data from the checkUpKeep(). When calling checkUpKeep() we are checking all allowlisted tokens and check which tokens are available for getting swapped and transferred to Reserve contract. SwapAutomator.sol#L450 function checkUpkeep( ... ) external whenNotPaused cannotExecute ... { >> address[] memory allowlistedAssets = s_feeAggregator.getAllowlistedAssets(); IV3SwapRouter.ExactInputParams[] memory swapInputs = new IV3SwapRouter.ExactInputParams[](allowlistedAssets.length); ... >> for (uint256 i; i < allowlistedAssets.length; ++i) { ... } } We can see that in checkUpKeep(), we are calling Fe...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# `SwapAutomator` assumes heartbeat of 24 hours for all price feeds

- **Contest:** Chainlink Payment Abstraction
- **Slug:** 2024-12-chainlink-payment-abstraction
- **Submission:** F-11
- **Submitter:** zanderbyte
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-chainlink-payment-abstraction/submissions/F-11
- **Source snapshot:** competitions/2024-12-chainlink-payment-abstraction/submissions/raw/F-11.txt

## Brief Summary

The SwapAutomator contract uses a hardcoded staleness threshold of 24 hours for all price feeds: /// @dev The threshold is set to 1 day to match the Chainlink feeds heartbeat requirement uint256 private constant STALENESS_THRESHOLD = 1 days; This does not account for the fact that different price feeds have varying heartbeat intervals. For example: The ETH/USD price feed on Ethereum Mainnet has a heartbeat of 1 hour, meaning prices should not be considered valid after this interval. When a price feed becomes stale, but remains within the hardcoded STALENESS_THRESHOLD, the system incorrectly assumes the data is valid. This can lead to operations being executed based on outdated and potential...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# The `SwapAutomator::_convertToLink()` can return incorrect `linkAmount` due to the fed price decimal mismatch vulnerability

- **Contest:** Chainlink Payment Abstraction
- **Slug:** 2024-12-chainlink-payment-abstraction
- **Submission:** F-20
- **Submitter:** serial-coder
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-chainlink-payment-abstraction/submissions/F-20
- **Source snapshot:** competitions/2024-12-chainlink-payment-abstraction/submissions/raw/F-20.txt

## Brief Summary

This report describes a fed price decimal mismatch vulnerability in the SwapAutomator contract. Meanwhile, the _getAssetPrice() (see @1) is called by the checkUpkeep() to obtain asset prices. The _getValidatedAssetPrice() (@2) is called by the checkUpkeep(), performUpkeep(), and swapWithPriceFeedValidation(). In reality, the prices obtained from Chainlink feeds can be represented using different reported decimals for various assets. For instance, the LINK price is represented using 8 decimals, whereas the PEPE price is represented using 18 decimals (see below for proof). function _getAssetPrice( AggregatorV3Interface oracle ) private view returns (uint256 assetPrice, uint256 updatedAtTimest...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Both `withdrawNative()` and `withdrawNonAllowlistedAssets()` remain accessible during emergency situations.

- **Contest:** Chainlink Payment Abstraction
- **Slug:** 2024-12-chainlink-payment-abstraction
- **Submission:** F-101
- **Submitter:** 0xpiken
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-chainlink-payment-abstraction/submissions/F-101
- **Source snapshot:** competitions/2024-12-chainlink-payment-abstraction/submissions/raw/F-101.txt

## Brief Summary

WITHDRAWER_ROLE caller can withdraw assets from FeeAggregator or FeeRouter under emergency situations

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# `performUpKeep()` will revert if BNB token swapping fails

- **Contest:** Chainlink Payment Abstraction
- **Slug:** 2024-12-chainlink-payment-abstraction
- **Submission:** F-36
- **Submitter:** Al-Qa-qa
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-chainlink-payment-abstraction/submissions/F-36
- **Source snapshot:** competitions/2024-12-chainlink-payment-abstraction/submissions/raw/F-36.txt

## Brief Summary

BNB token is one of the tokens that revert on approving to zero value link. And as stated in the README, tokens that revert on zero value approval is In scope. This issue affects all tokens that revert on zero value approval, not only BNB, but we will talk about BNB as it is popular token. After DONs check which tokens to swap using checkUpKeep() they swap tokens using performUpKeep(). Tokens are swapped using UniswapV3, where they are converted into LINK tokens. SwapAutomator.sol#L596 >> IERC20(asset).safeIncreaseAllowance(address(i_uniswapRouter), amountIn); // For multiple swaps we don't want to revert the whole transaction if only some of the // swaps // fail so we catch the revert and...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Swaps that occur earlier can affect swap amount out for later swaps and lead to unexpected reverts

- **Contest:** Chainlink Payment Abstraction
- **Slug:** 2024-12-chainlink-payment-abstraction
- **Submission:** F-66
- **Submitter:** gesha17
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-chainlink-payment-abstraction/submissions/F-66
- **Source snapshot:** competitions/2024-12-chainlink-payment-abstraction/submissions/raw/F-66.txt

## Brief Summary

The checkUpkeep function will call i_uniswapQuoterV2.quoteExactInput to get the amount received for each swap according to the swap path: function checkUpkeep( bytes calldata ) external whenNotPaused cannotExecute returns (bool upkeepNeeded, bytes memory performData) { ... if (asset != address(i_linkToken)) { (amountOutUniswapQuote,,,) = i_uniswapQuoterV2.quoteExactInput(swapParams.path, swapAmountIn); ... } This call performs an individual check for each swap, irrespective of other swaps. The problem is that this may not be an accurate representation of how the swaps that will occur in performUpkeep(), where an earlier swap may affect the slippage rate for a later swap. As a result of this...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Unprofitable swaps for tokens with greater than 8 decimals price feeds

- **Contest:** Chainlink Payment Abstraction
- **Slug:** 2024-12-chainlink-payment-abstraction
- **Submission:** F-13
- **Submitter:** zanderbyte
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-chainlink-payment-abstraction/submissions/F-13
- **Source snapshot:** competitions/2024-12-chainlink-payment-abstraction/submissions/raw/F-13.txt

## Brief Summary

Before executing a swap, the checkUpkeep function is invoked to ensure that the swap will be profitable by verifying that the current USD value of the asset meets the minimum swap size usd value (availableAssetUsdValue >= swapParams.minSwapSizeUsd * assetUnit). However, the SwapParams struct defines that minSwapSizeUsd will be in 8 decimals, while some Chainlink price feeds for token/USD use 18 decimals (For example AMPL/USD on Mainnet, BONK/USD and PEPE/USD on Optimism, etc.). This can result in incorrect comparisons when determining whether a swap is profitable. A malicious actor (or inadvertently, through dust amounts) could exploit this by sending tokens associated with an 18-decimal pr...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# `SwapAutomator._getAssetPrice()` doesn't validate returned price against `minAnswer` & `maxAnswer`

- **Contest:** Chainlink Payment Abstraction
- **Slug:** 2024-12-chainlink-payment-abstraction
- **Submission:** F-95
- **Submitter:** hals
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-chainlink-payment-abstraction/submissions/F-95
- **Source snapshot:** competitions/2024-12-chainlink-payment-abstraction/submissions/raw/F-95.txt

## Brief Summary

SwapAutomator._getValidatedAssetPrice() function is used to fetch the assets prices from chainlink price feeds, and used in: chekUpkeep() : to check if there is any asset passing the minimum swap size and price is within an acceptable slippage and can be swapped into $LINK token. performUpkeep() : when the assets are swapped into $LINK tokens via UniswapV3 router. function _getValidatedAssetPrice( AggregatorV3Interface oracle ) private view returns (uint256 assetPrice) { (uint256 answer, uint256 updatedAt) = _getAssetPrice(oracle); if (answer == 0) revert Errors.ZeroOracleData(); if (updatedAt < block.timestamp - STALENESS_THRESHOLD) { revert Errors.StaleOracleData(); } return answer; } whe...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.
