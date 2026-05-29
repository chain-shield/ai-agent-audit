# Benchmark Ground Truth: Flex Perpetuals

## Accepted H/M Findings

# Accepted H/M Findings: Flex Perpetuals

# [M-01] Missing slippage protection in AerodromeDexter.sol swapExactTokensForTokens()

- **Contest:** Flex Perpetuals
- **Slug:** 2024-12-flex-perpetuals
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-flex-perpetuals
- **Source snapshot:** competitions/2024-12-flex-perpetuals/final_report.html

AerodromeDexter.sol swapExactTokensForTokens() Submitted by alix40, also found by 0xKann, 0xlucky, 0xpetern, 0xvd, ABAIKUNANBAEV, Abhan, air_0x, zanderbyte, bharg4v, DharkArtz, dic0de, Hajime, hals, inh3l, John_Femi, kodyvim, lightoasis, Matin, mgf15, MSaptarshi, Mushow, newspacexyz, NexusAudits, PolarizedLight, Rhaydden, Shinobi, smbv-1923, Sparrow, tpiliposian, Tumelo_Crypto, udo, vesko210, waydou, and y51r

## Finding description and impact

Even though there is no frontrunning on L2s, which could mitigate some of the MEV attacks, without defined slippage and in case of congestions the swap could be executed at unfavorable conditions presenting losses for users.

- https://github.com/code-423n4/2024-12-flex-perpetuals/blob/b84a0812c3368866964b8a16e7c36f6e7a50b655/src/extensions/dexters/AerodromeDexter.sol#L54-L56
uint256 _balanceBefore = ERC20(_tokenOut).balanceOf(address(this)); @>> router.swapExactTokensForTokens(_amountIn, 0, routeOf[_tokenIn][_tokenOut], address(this), block.timestamp); Having the minAmountOut set to 0, will make sure that the swap will always be executed no matter the market condions.

## Recommended mitigation steps

We recommend allowing users to set their desired slippage params.

function run( address _tokenIn, address _tokenOut, uint256 _amountIn, + uint256 _minAmountOut, + uint64 _deadline ) external override returns (uint256 _amountOut) { uint256 _balanceBefore = ERC20(_tokenOut).balanceOf(address(this)); - router.swapExactTokensForTokens(_amountIn, 0, routeOf[_tokenIn][_tokenOut], address(this), block.timestamp); + router.swapExactTokensForTokens(_amountIn, _minAmountOut, routeOf[_tokenIn][_tokenOut], address(this), _deadline); 0xRobocop (validator) commented:

As the minimum output is hardcoded to zero, the loss for users that use this function can be of 100% of their funds.

fpcrypto-y (Flex Perpetuals) disputed and commented:

More detailed clarification was given by the Aerodrome team:

The MEV created by Aerodrome isn’t being captured by the Base sequencer currently. As in, the Base sequencer isn’t being used to sandwich attack/frontrun, deposit JIT liquidity, or use its privilege to arb pools/backrun. So there is no sandwiching or proper JIT liquidity attacks occurring at all, due to the Base sequencer choosing not to do this, which is sensible because these attacks are adversarial and would scare users away (I’m not aware of any sandwiching/JIT attacks happening on any centralized sequencer L2s). The Base sequencer isn’t backrunning either, so arbitrage profits are captured by regular arb bots with no special privileges.

The dynamic fee upgrade is expected to greatly reduce the amount of arbitrage profits created by Aerodrome by increasing fees during volatility, essentially allowing regular CL LPs to behave more like professional market makers, because increasing fees during volatility effectively does the same thing as market makers widening their spreads in the order book during volatility. So in essence, the dynamic fee upgrade will hopefully take 80+% of the $ currently being extracted by Aerodrome arb bots and instead allocate it as fees to veAERO voters. I think this is a cleaner and much less centralized solution than creating an appchain with a centralized sequencer and running a privileged arb bot that then redistributes its profits to tokenholders or LPs or traders or whatever it may be.”

0xsomeone (judge) commented:

The Warden and its duplicates have demonstrated that the code lacks proper slippage checks in its AMM interaction within the AerodromeDexter::run function. To clarify a few discussion points:

The AMM will consistently result in a non-zero output, so 100% of the funds cannot be captured in a money-efficient manner.

While the sequencer does not permit arbitrage opportunities to be arbitrarily captured, the code will continue to result in uncontrollable outputs that might ultimately not result in the output that the caller expects.

Combining the above two points, I believe a medium-risk severity rating is more appropriate for this submission. Users are directly impacted albeit the funds that they might lose would realistically be consistently lower than if arbitrage opportunities could be captured by sandwich attacks.

# [M-02] Most of the FTC rewards can be taken by single entity

- **Contest:** Flex Perpetuals
- **Slug:** 2024-12-flex-perpetuals
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-flex-perpetuals
- **Source snapshot:** competitions/2024-12-flex-perpetuals/final_report.html

Submitted by farman1094

- https://github.com/code-423n4/2024-12-flex-perpetuals/blob/b84a0812c3368866964b8a16e7c36f6e7a50b655/src/staking/FTCHook.sol#L61

## Finding description and impact

Malicious actor can collect the most reward which supposed to be share between genuine traders.

There is feature of flex protocol of Flex Trade Credit the user will earn FTC as they trade on the Flex Perpetual. As the position increases the token minted by FTCHook, it’s staked in the TLC Staking for that specific epoch under the user name which increased the position. For that staking, the reward will be issued to the trader in form of USDbC. Instead of opening the position for trading, the user opens the position just for the token and closes it instantly in the same block. As for increasing the position, he would be awarded by FTC but as he closed, instantly no effect on FTC. Because we are doing nothing while decreasing the position.

//FTCHook::onDecreasePosition function onDecreasePosition ( address _primaryAccount, uint256, uint256, uint256 _sizeDelta, bytes32 ) external onlyWhitelistedCaller { // Do nothing }

- https://github.com/code-423n4/2024-12-flex-perpetuals/blob/b84a0812c3368866964b8a16e7c36f6e7a50b655/src/staking/FTCHook.sol#L61C1-L69C6
So what happen is, the reward supposed to be shared between all the Genuine traders. Some malicious actor can target it after checking the specific epoch which benefit him most and mint the stake reward as much as possible for small fees.

Practical example:

This is mere an example, trader can do this in small and big scale as well.

If it’s BTC or ETH market, weight 2.5, so user will receive 250,000 FTC for opening $100,000 position.

Initial collateral = 100,000 usdc 1st trade opening position trading fee = 100,000 - 100 = 99900 1st trade opening position execution fee = 99900 - 0.1 = 99899.9 2nd trade closing position trading fee = 99899.9 - 100 = 99799.9 2nd trade closing position execution fee = 99799.9 - 0.1 = 99799.8 The total fee trader paid for this 250,000 FTC is $201; this way trader can earn the gurranteed profit in small fees. When the rewards came, this traders get the most reward of because of his big number of share. This reward is supposed to be share between all the traders who’s risking their collateral.

## Recommended mitigation steps

We need to make the changes in the src/staking/FTCHook.

//FTCHook::onDecreasePosition function onDecreasePosition ( address _primaryAccount, uint256, uint256, uint256 _sizeDelta, bytes32 ) external onlyWhitelistedCaller { // Do nothing } We need to set some timeFrame if user choose to close the position instantly or within that time frame. The TLC token should be taken back. Like we’re doing in TradingStakingHook::onDecreasePosition.

flexdev (Flex Perpetuals) commented:

It’s not obvious from scoped files, but protocol has getStepMinProfitDuration configurations that set’s minimal duration of position depends on position size.

ConfigStorage(configStorage).getStepMinProfitDuration(...)

- https://docs.flex.trade/protocol-and-safety-parameters#min-profit-duration
0xsomeone (judge) commented:

The submission describes that the position creation reward is immediately disbursed by the system and does not wait for any period of time, permitting a user to instantly capture rewards that exceed the system’s imposed fees.

The sponsor claims that the system prevents such scenarios by imposing a minimum step profit duration; however, as its name implies, this time-based restriction is solely imposed on profitable positions and is not activated for zero-profit positions (i.e. ones that are opened and closed immediately).

I believe that a severity of high is appropriate given that the system’s reward mechanism will effectively be compromised and can be compromised to an arbitrary degree via the use of flash-loans or similar mechanisms.

ZanyBonzy (warden) commented:

According to the audit rules and provided

## Rejected Primary Findings

# Rejected Primary Findings: Flex Perpetuals

# Using `block.timestamp` as a deadline parameter for `swapExactTokensForTokens` offers no protection

- **Contest:** Flex Perpetuals
- **Slug:** 2024-12-flex-perpetuals
- **Submission:** F-27
- **Submitter:** zanderbyte
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-flex-perpetuals/submissions/F-27
- **Source snapshot:** competitions/2024-12-flex-perpetuals/submissions/raw/F-27.txt

## Brief Summary

The run function of the AerodromeDexter contract uses block.timestamp as the deadline parameter for swapExactTokensForTokens, which is incorrect and dangerous. For example a validator can hold the transaction and include it a later block, causing worse price for the swap and exposing users to MEV opportunities.

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# The AerodromeDexter.sol contract doesn't support tokens that revert on large approvals

- **Contest:** Flex Perpetuals
- **Slug:** 2024-12-flex-perpetuals
- **Submission:** F-28
- **Submitter:** alix40
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-flex-perpetuals/submissions/F-28
- **Source snapshot:** competitions/2024-12-flex-perpetuals/submissions/raw/F-28.txt

## Brief Summary

The AerodromeDexter.sol contract doesn't support tokens that revert on large approvals and/or transfers As mentioned in the readme such tokens are in scope

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# IntentHandler.sol has no checks validating that market is active.

- **Contest:** Flex Perpetuals
- **Slug:** 2024-12-flex-perpetuals
- **Submission:** F-30
- **Submitter:** Tumelo_Crypto
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-12-flex-perpetuals/submissions/F-30
- **Source snapshot:** competitions/2024-12-flex-perpetuals/submissions/raw/F-30.txt

## Brief Summary

Delisted Market transactions can still be executed by the IntentHandler.sol contract

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# The `AerodromeDexter.sol` contract doesn't support fee on transfer tokens

- **Contest:** Flex Perpetuals
- **Slug:** 2024-12-flex-perpetuals
- **Submission:** F-29
- **Submitter:** alix40
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-flex-perpetuals/submissions/F-29
- **Source snapshot:** competitions/2024-12-flex-perpetuals/submissions/raw/F-29.txt

## Brief Summary

In the run() function of the AerodromeDexter.sol contract, the amount to be swapped is transfered directly to the contract. This is how the run function is used inside the flex perp protcol function execute(uint256 _amount, address[] calldata _path) external returns (uint256 _amountOut) { if (_amount == 0) revert SwitchCollateral_BadAmount(); if (_path.length < 2) revert SwitchCollateral_BadPath(); for (uint i = 0; i < _path.length - 1; i++) { (address _tokenIn, address _tokenOut) = (_path[i], _path[i + 1]); IDexter _dexter = dexterOf[_tokenIn][_tokenOut]; // Check if the dexterOf[tokenIn][tokenOut] is registered. if (address(_dexter) == address(0)) revert SwitchCollateralRouter_NotFoundDex...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.
