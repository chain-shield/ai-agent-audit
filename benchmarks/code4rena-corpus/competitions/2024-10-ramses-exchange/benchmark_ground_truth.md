# Benchmark Ground Truth: Ramses Exchange

## Accepted H/M Findings

# Accepted H/M Findings: Ramses Exchange

# [M-01] Inflated GaugeV3 rewards when period is skipped

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-10-ramses-exchange
- **Source snapshot:** competitions/2024-10-ramses-exchange/final_report.html

GaugeV3 rewards when period is skipped Submitted by rileyholterhus, also found by MrPotatoMagic The GaugeV3 contract distributes rewards based on the proportion of liquidity that each position had in range over each 1-week “period”. This is calculated by cachePeriodEarned() in the gauge, which calls positionPeriodSecondsInRange() on the RamsesV3Pool. A key part of this calculation is the periodCumulativesInside() function, which computes the total seconds per liquidity within a tick range for that period.

In periodCumulativesInside(), one sub-case occurs when the period is in the past, and the tick range was active at the end of that period:

function periodCumulativesInside ( /*... */ ) /*... */ { //...

if ( lastTick < tickLower ) { //...

} else if ( lastTick < tickUpper ) { //...

if ( currentPeriod <= period ) { //...

} else { cache.

secondsPerLiquidityCumulativeX128 = $.

periods [ period ].

endSecondsPerLiquidityPeriodX128; } return cache.

secondsPerLiquidityCumulativeX128 - snapshot.

secondsPerLiquidityOutsideLowerX128 - snapshot.

secondsPerLiquidityOutsideUpperX128; } else { //...

} Notice that this sub-case relies on $.periods[period].endSecondsPerLiquidityPeriodX128, which is meant to represent the total seconds per liquidity when the period ended. However, this value is actually more accurately described as “the seconds per liquidity at the start of the next period”, which can be seen in how _advancePeriod() and newPeriod() are implemented:

function _advancePeriod () /*... */ { //...

if (( _blockTimestamp () / 1 weeks ) != _lastPeriod ) { //...

uint160 secondsPerLiquidityCumulativeX128 = Oracle.

newPeriod ( $.

observations, _slot0.

observationIndex, period ); //...

$.

periods [ _lastPeriod ].

endSecondsPerLiquidityPeriodX128 = secondsPerLiquidityCumulativeX128; //...

} function newPeriod ( /*... */ ) /*... */ { //...

uint32 delta = uint32 ( period ) * 1 weeks - 1 - last.

blockTimestamp; secondsPerLiquidityCumulativeX128 = last.

secondsPerLiquidityCumulativeX128 + (( uint160 ( delta ) << 128 ) / ( $.

liquidity > 0 ?

$.

liquidity:

1 )); //...

} So, this means that if a period is skipped (meaning no activity happens in the pool during the period), the next time _advancePeriod() is called, the endSecondsPerLiquidityPeriodX128 for the last period will be set to just before the start of the period after the skipped period. This is effectively one week after the last period actually ended. This adds extra time to the sub-case mentioned above, which leads to inflated rewards for users.

## Impact

If period p has gauge rewards and period p+1 has no pool activity, the reward calculation for period p will be inflated, allowing users to claim more tokens than they should.

## Recommended Mitigation Steps

One initial idea for a fix might be to modify _advancePeriod() and newPeriod() to only extrapolate the seconds per liquidity up to the end of the period, rather than to the start of the next period:

function _advancePeriod () /*... */ { //...

if (( _blockTimestamp () / 1 weeks ) != _lastPeriod ) { //...

uint160 secondsPerLiquidityCumulativeX128 = Oracle.

newPeriod ( $.

observations, _slot0.

observationIndex, _lastPeriod // << CHANGE: pass _lastPeriod instead of period ); //...

$.

periods [ _lastPeriod ].

endSecondsPerLiquidityPeriodX128 = secondsPerLiquidityCumulativeX128; //...

} function newPeriod ( /*... */ ) /*... */ { //...

uint32 delta = uint32 ( period + 1 ) * 1 weeks - 1 - last.

blockTimestamp; // << CHANGE: use end of period instead of start secondsPerLiquidityCumulativeX128 = last.

secondsPerLiquidityCumulativeX128 + (( uint160 ( delta ) << 128 ) / ( $.

liquidity > 0 ?

$.

liquidity:

1 )); //...

} However, the current behavior of endSecondsPerLiquidityPeriodX128 is actually important to maintain for the following logic in periodCumulativesInside():

function periodCumulativesInside ( /*... */ ) /*... */ { //...

snapshot.

secondsPerLiquidityOutsideLowerX128 = uint160 ( lower.

periodSecondsPerLiquidityOutsideX128 [ period ]); if ( tickLower <= startTick && snapshot.

secondsPerLiquidityOutsideLowerX128 == 0 ) { snapshot.

secondsPerLiquidityOutsideLowerX128 = $.

periods [ previousPeriod ].

endSecondsPerLiquidityPeriodX128; } snapshot.

secondsPerLiquidityOutsideUpperX128 = uint160 ( upper.

periodSecondsPerLiquidityOutsideX128 [ period ]); if ( tickUpper <= startTick && snapshot.

secondsPerLiquidityOutsideUpperX128 == 0 ) { snapshot.

secondsPerLiquidityOutsideUpperX128 = $.

periods [ previousPeriod ].

endSecondsPerLiquidityPeriodX128; } //...

} So, it is instead recommended to introduce separate startSecondsPerLiquidityPeriodX128 and endSecondsPerLiquidityPeriodX128 variables in the PeriodInfo struct, so that the code can correctly distinguish between the two different time points in the calculations.

gzeon (judge) commented:

_advancePeriod is public and this issue also seems to be possible on pool with no activity at all.

cc @keccakdog keccakdog (Ramses) commented:

@gzeon - In GaugeV3::notifyRewardAmount(), _advancePeriod() is called at the start of the function. This means if there are any gauge rewards for the week due to voting, advance period would be called.

The one case this would occur is if someone called notifyRewardAmountNextPeriod, or the ” forPeriod ” variant + had no votes for the week + had no swaps or liq adds or removes, for the entire week.

As you can imagine this is a close to 0% chance of happening, since if there are rewards there is likely at least 1 interaction the entire period, or else these rewards are pointless. TLDR is this is a very very very very unlikely case (only possible if our entire project is dead and nobody is interacting 😓).

It may be beneficial to document it or maybe add a safety check in case, but I do not find this as more than a Low finding at best since the assumption of our project being dead is essentially required for it work.

gzeon (judge) decreased severity to Low/Non-Critical rileyholterhus (warden) commented:

Hello judge/sponsor, thank you for your comments. I would like to escalate this issue for two reasons:

I believe the bug has been misunderstood. The above comments are saying it’s unlikely for a period p to be skipped while still having gauge rewards, as notifyRewardAmount() triggers _advancePeriod(), so rewards for a skipped period would need to be given in advance using notifyRewardAmountNextPeriod() or notifyRewardAmountForPeriod(). However this argument is not relevant to the bug. With this bug, inflated rewards in period p are due to period p+1 being skipped and not period p being skipped. This can be seen in the PoC - notice that a theft is demonstrated using notifyRewardAmount(), while notifyRewardAmountNextPeriod() / notifyRewardAmountForPeriod() are never used.

The above comments are focusing on how likely the bug is to be triggered by accident, but the bug can also be exploited intentionally. For one example, an attacker could deploy a pool with minimal liquidity, allocate gauge rewards at the last moment before the period switch, and then simply wait as further periods pass. Since the attacker independently deployed the pool and only provided minimal liquidity, others would be unlikely to engage with it at first, and the inflated rewards would silently build up. This is especially dangerous if the attacker knows a pool might gain popularity later, for example by knowing that a partner protocol plans to incentivize liquidity for a specific token pair in the future. So, I believe this bug should not remain unaddressed, and is high severity and not QA.

keccakdog (Ramses) commented:

Hey Riley, thank you for following up on this. While I see what you mean, the reason this was labeled a lesser severity is because the situation you are explaining is rather impossible. If there are meaningful rewards, at least ONE person would be LPing and interacting. Also, if there was lots of rewards and then nothing— the gauge would see people removing liquidity, which unless I’m mistaken, nullifies this. Someone making a pool last second and voting for it is fine since if it was somehow a malicious gauge it could be killed and prevent abuse. The system has lots of checks in place to prevent gaming like this, and non-active pools are not profitable for people to vote on, so the only way period P rewards would be significant and P+1 being completely empty would mean not a single interaction occurs in P+1, which as you can imagine is extremely unlikely that someone would vote for a pool with 0 rewards in hopes of attempting to get more gauge rewards, when others can join in and take the rewards during the period as well. Since the damage is a multiplier of the existing rewards being inflated in the future— the vulnerability requires heavy voting power to be worth anything.

Hopefully that makes sense. I don’t disagree with your finding that it is possible; but I disagree on the severity being very low due to the likelihood being close to 0 rileyholterhus (warden) commented:

Hi @keccakdog - thank you for the follow-up. Since understanding the issue requires a lot of context, I would like to leave the following notes for the judge, and also respond to your points. Please feel free to correct me if I’m wrong in any of the following:

- I believe we’re in agreement that the initial comment that downgraded this issue is not relevant to this bug. However, note that the initial comment is relevant for issue 40.

- Part of the most recent comment is a counter-argument to my example of how an attacker could intentionally exploit the bug. I have the following response to those points:

“If there are meaningful rewards, at least ONE person would be LPing and interacting.” This is why I think the attacker would allocate gauge rewards at the last moment before the period switch. This timing would leave no opportunity for others to react and compete for the rewards before the period ends, so there is no incentive-based reason for anyone to interact with the pool and inadvertently prevent the exploit.

“Also, if there was lots of rewards and then nothing— the gauge would see people removing liquidity, which unless I’m mistaken, nullifies this.” The attacker only needs to provide a few wei of liquidity, since they aren’t competing with anyone in the setup. So they incur no significant cost by abandoning their dust liquidity, and they could even choose to withdraw it in period p+2 if needed.

“Someone making a pool last second and voting for it is fine since if it was somehow a malicious gauge it could be killed and prevent abuse.” I think this point addresses how the issue could be mitigated, which is separate from the severity of the issue itself. An admin can only prevent the issue if they are aware of the bug. Therefore I believe this finding should be considered high-severity.

- I believe the remaining part of the above comment argues that it’s unlikely for this bug to occur accidentally, which I agree with:

“The system has lots of checks in place to prevent gaming like this, and non-active pools are not profitable for people to vote on, so the only way period P rewards would be significant and P+1 being completely empty would mean not a single interaction occurs in P+1, which as you can imagine is extremely unlikely that someone would vote for a pool with 0 rewards in hopes of attempting to get more gauge rewards, when others can join in and take the rewards during the period as well. Since the damage is a multiplier of the existing rewards being inflated in the future— the vulnerability requires heavy voting power to be worth anything.” Thanks again for the consideration everyone.

gzeon (judge) commented:

This is a tough call, but I think it is still more appropriate to have this as low risk.

The situation as described is very unlikely as the sponsor explained. There are protocol incentives to make sure pool with gauge should have at least some activity. So unless this can be intentionally exploited, I think this is really quite impossible.

rileyholterhus (warden) commented:

Hi @gzeon, thanks for the follow-up. Apologies for all the back-and-forth, but I think there may be a misunderstanding.

For this bug/exploit, no voting power is required. Notice that the notifyRewardAmount() function is permissionless, and in the PoC there is no voting power logic used.

Of course calling notifyRewardAmount() is not free - anyone who calls it will be transferring their own tokens to the in-range LPs for the period. But in the case of the following:

“For one example, an attacker could deploy a pool with minimal liquidity, allocate gauge rewards at the last moment before the period switch, and then simply wait as further periods pass.” The attacker is the sole in-range LP, so calling notifyRewardAmount() is just a self-transfer to give themselves a reward balance in the gauge, which is the first step to exploiting this bug.

Do you see what I mean? I still believe this issue is not low-severity, and I’m happy to expand on any other parts of the discussion if additional clarification is needed. Thanks again!

gzeon (judge) commented:

“notifyRewardAmount() function is permissionless” If the attacker choose to reward themselves, I don’t see that as an issue. While Voter.sol is out-of-scope, according to Ramses v3 doc:

“A voting incentive is designated at anytime during the current EPOCH and paid out in lump sum at the start of the following EPOCH.” “Once an LP incentive is deposited it will distribute that token and the amount deposited for the next 7 days.” So I think it is fair to assume it is expected to have reward to be notified near the start of a period. Any reward would incentivize LP activity and thus a period is unlikely to be skipped. Note even p+1 does not have any reward, the lack of incentive is a incentive for LPs in p to remove liquidity, which also advances the period. If there are no other LP because the attacker is the sole LP, they would receive 100% of the reward regardless of this issue.

Hence, it appears to me the incentives are well designed to make skipping a period p or p+1 where p have non negligible incentive can be considered as impossible.

rileyholterhus (warden) commented:

Hi @gzeon thank you for the follow-up. I believe there’s still a misunderstanding.

“If the attacker choose to reward themselves, I don’t see that as an issue.” I agree - an attacker transferring funds to themselves isn’t inherently an exploit. However, the point I was making is this:

“calling notifyRewardAmount() is just a self-transfer to give themselves a reward balance in the gauge, which is the first step to exploiting this bug.” In other words, if an attacker rewards themselves right before the period switch, they spend nothing (since it’s a self-transfer), allocate rewards exclusively in period p, and the timing doesn’t leave any opportunity for others to be incentivized to LP and interfere. This leads into the next point:

“If there are no other LP because the attacker is the sole LP, they would receive 100% of the reward regardless of this issue.” I agree here as well, but since this is being used as a counter-argument to invalidate the finding, I think there’s a misunderstanding of the core bug. By establishing a reward balance in period p, the attacker gains an inflated reward balance with each subsequent skipped period. This is the main issue.

gzeon (judge) increased severity to Medium and commented:

“identify an obsolete pool (it’s inevitable that at least one pool will become inactive over time) and use its gauge to initiate the exploit. Instead of setting up an inflated balance to exploit future users, this would allow the attacker to inflate their balance to steal any unclaimed rewards from past activity” Alright I think I am getting convinced, this attack does seems to work; I previously thought it requires the pool to have no liquidity (contradict with leftover reward), it actually only requires no liquidity in the active tick. It is conceivable that an obsolete pool may have stale liquidity in an inactive range where the attacker can deposit 2 wei liquidity from 2 account in the active range, notify reward equal to the leftover at the last second of a period and hope for no activity for the next period. There are no cost (except gas) for this attack because the attacker own 100% of the active liquidity during the period they paid for the reward.

In terms of severity, I think Medium is appropriate given the pre-condition required.

# [M-02] The fee for the protocol in the function RamsesV3Pool::flash() is not calculated correctly

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-10-ramses-exchange
- **Source snapshot:** competitions/2024-10-ramses-exchange/final_report.html

RamsesV3Pool::flash() is not calculated correctly Submitted by BenRai, also found by 0x37 and wasm_it

## Impact

Because the fee for the protocol is not calculated correctly, the split of fees is wrong resulting in less fees for the protocol.

## Recommended Mitigation Steps

To ensure the right amount of fees are distributed to the protocol when a user initiates a flashloan change the current calculations for token0 and token 1 to the following:

function flash(address recipient, uint256 amount0, uint256 amount1, bytes calldata data) external override lock { … if (paid0 > 0) { - uint256 pFees0 = feeProtocol == 0 ? 0: paid0 / feeProtocol; + uint256 pFees0 = feeProtocol == 0 ? 0: paid0 * feeProtocol / 100; if (uint128(pFees0) > 0) $.protocolFees.token0 += uint128(pFees0); $.feeGrowthGlobal0X128 += FullMath.mulDiv(paid0 - pFees0, FixedPoint128.Q128, _liquidity); } if (paid1 > 0) { - uint256 pFees1 = feeProtocol == 0 ? 0: paid1 / feeProtocol; + uint256 pFees1 = feeProtocol == 0 ? 0: paid1 * feeProtocol / 100; if (uint128(pFees1) > 0) $.protocolFees.token1 += uint128(pFees1); $.feeGrowthGlobal1X128 += FullMath.mulDiv(paid1 - pFees1, FixedPoint128.Q128, _liquidity);

} … } keccakdog (Ramses) commented:

While it passes the UniswapV3Pool.spec before the fix, this is an oversight due to feeProtocol no longer using bitshifting (like in regular UniswapV3). Our live V2 contracts were upgraded to make feeProtocol operate like this, and handle it — but was not translated over.

Seems to be a valid finding by reading, would need to run test(s) to ensure it isn’t handled elsewhere. 👍 keccakdog (Ramses) confirmed, but disagreed with severity and commented:

Valid finding; however, this should be downgraded to a Low due to no funds at risk.

gzeon (judge) commented:

“2 — Med: Assets not at direct risk, but the function of the protocol or its availability could be impacted, or leak value with a hypothetical attack path with stated assumptions, but external requirements.”

## Rejected Primary Findings

# Rejected Primary Findings: Ramses Exchange

# Wrong cast in the `Tick::tickSpacingToMaxLiquidityPerTick` function, which can lead to an unintended conversion.

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-111
- **Submitter:** 0XRolko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/111
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-111.md

## Brief Summary

Convert `int24` to `uint24` without limiting the range of `int24`. Impact Overflow Risk and unexpected behavior: If we attempt to cast a negative `int24` to `uint24` without checking its value, it could lead to an unintended conversion, potentially resulting in very large numbers due to how negative values are represented in memory.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of address(0) check when initializing the `FeeCollector` contract.

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-112
- **Submitter:** 0XRolko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/112
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-112.md

## Brief Summary

Risk of loss of funds when `treasury=address(0)`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Use of `upgradable` tools as part of a `non-upgradable` contract in the `FeeCollector` contract.

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-113
- **Submitter:** 0XRolko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/113
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-113.md

## Brief Summary

The logic explained in the v3 documentation is not the same as that implemented in the `FeeCollector`contract. Impact Unnecessary Complexity: Upgradable tools like `Initializable` and the `initializer` modifier are specifically designed for upgradable contracts using proxy patterns. In a non-upgradable contract, using these tools adds complexity that isn't required, as constructors can handle initialization directly. This increases the risk of mistakes during contract deployment, such as forgetting to call the `initialize` function, which can leave the contract in an uninitialized state and potentially vulnerable.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of zero check in the `RamsesV3Factory::setFee` function.

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-114
- **Submitter:** 0XRolko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/114
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-114.md

## Brief Summary

In the `RamsesV3Factory::setFee` function, `_fee` param can take `zero` value. Impact Unexpected behavior and all those who are supposed to be remunerated thanks to these fees will receive nothing at the end of the day.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unnecessary storage write and event emission in the `RamsesV3Factory::setFeeCollector` function.

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-115
- **Submitter:** 0XRolko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/115
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-115.md

## Brief Summary

Redundant storage writes and event emissions.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect initialization order in pool initialize()

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-4
- **Submitter:** 0x37
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/4
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-4.md

## Brief Summary

Incorrect initialization order in pool initialize()

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# `PUSH0` opcode is Not Supported on Linea

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-5
- **Submitter:** 0xDemon
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/5
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-5.md

## Brief Summary

Deploying the protocol on `Linea` with the current Solidity version (`^0.8.20`) may result in unexpected behavior or failure due to the unsupported `PUSH0` opcode

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Inconsistent Cached Rewards in GaugeV3 Contract

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-11
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/11
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-11.md

## Brief Summary

The `cachePeriodEarned` function checks if the period data has been written to storage using the `periodAmountsWritten` mapping. If it hasn't been written and `caching` is `false`, it calculates the amount but doesn't store it. This leads to inconsistencies between cached and actual earned rewards. [GaugeV3.sol#cachePeriodEarned](https://github.com/code-423n4/2024-10-ramses-exchange/blob/236e9e9e0cf452828ab82620b6c36c1e6c7bb441/contracts/CL/gauge/GaugeV3.sol#L278-L335) The fact that the function proceeds to calculate and return an amount even when `caching` is false and the period data hasn't been written to storage. This can lead to inconsistencies between cached and actual earned rewards....

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# "Stack too deep" Compilation Error in `_updatePosition` Function

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-6
- **Submitter:** 0xbrett8571
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/6
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-6.md

## Brief Summary

The `RamsesV3Pool.sol` contract utilizes the `Position` library defined in `Position.sol`. Within the `Position` library, there is a function named `_updatePosition` that is causing a _"Stack too deep" compilation error._ The error occurs specifically on line 329 of Position.sol, which is part of the `_updatePosition` function. The line of code causing the error is: [Position.sol#L329](https://github.com/code-423n4/2024-10-ramses-exchange/blob/236e9e9e0cf452828ab82620b6c36c1e6c7bb441/contracts/CL/core/libraries/Position.sol#L329) The _"Stack too deep"_ error happens when the number of local variables and function parameters in a function exceeds the maximum stack depth limit of the Ethereum...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# getPeriodReward() function uses caller's address as the position owner address

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-12
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/12
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-12.md

## Brief Summary

getPeriodReward() function uses caller's address as the position owner address

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# lastClaimByToken is incorrectly updated by assigning the period value to the one that was before the current one

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-13
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/13
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-13.md

## Brief Summary

lastClaimByToken is incorrectly updated by assigning the period value to the one that was before the current one

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Amount to claim is incorrectly calculated

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-14
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/14
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-14.md

## Brief Summary

Amount to claim is incorrectly calculated

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# periodSecondsInsideX96 is calculated using liquidity value which should not be the case

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-16
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/16
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-16.md

## Brief Summary

periodSecondsInsideX96 is calculated using liquidity value which should not be the case

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Validation in Pool Initialization Leading to Potential Control Hijack

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-116
- **Submitter:** Auditor2947
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/116
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-116.md

## Brief Summary

Missing Validation in Pool Initialization Leading to Potential Control Hijack

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Missing Input Validation in Batch Functions Leading to Potential Gas Wastage

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-117
- **Submitter:** Auditor2947
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/117
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-117.md

## Brief Summary

In the `setPoolFeeProtocolBatch` functions, there are no checks to prevent empty arrays from being passed as arguments. This lack of input validation could result in the function executing unnecessarily when empty arrays are provided, leading to wasted gas and potential logic issues in batch operations. Impact: Passing empty arrays as arguments (e.g., `[]`) would still cause the function to execute, consuming gas without performing any meaningful operations. This could result in higher costs for users and inefficient use of network resources. Additionally, empty input might lead to unintended behavior, as the loop still processes without any actual data, which could introduce subtle logic e...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# DoS of a valid tickspacing due to inadequate check in enableTickspacing.

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-22
- **Submitter:** BugHunters1
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/22
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-22.md

## Brief Summary

There is a problem that the function [enableTickSpacing](https://github.com/code-423n4/2024-10-ramses-exchange/blob/236e9e9e0cf452828ab82620b6c36c1e6c7bb441/contracts/CL/core/RamsesV3Factory.sol#L106-L116) would fail even when a particular valid tickspacing parameter is passed to the function. From the comment in the code which describes the conditions that the function must hold, it says This implies that the tickspacing should be less than or equal to 16384. Hence, when a user sets his/her tickspacing to 16384, it should work. However, it doesn't work from the current implementation of the code as seen below; This is because the check is inadequate. This results in Denial of Service for v...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Tick spacing and overflow risks which could lead to overflow or calculation issues and may result in severe financial loss

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-148
- **Submitter:** DCENT09
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/148
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-148.md

## Brief Summary

Tick spacing and overflow risks which could lead to overflow or calculation issues and may result in severe financial loss

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Immutable variables exposure could lead to funds being locked or lost.

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-158
- **Submitter:** DCENT09
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/158
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-158.md

## Brief Summary

Immutable variables exposure could lead to funds being locked or lost.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_18_group

# Lack of parameter validation can lead to contract malfunction.

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-163
- **Submitter:** DCENT09
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/163
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-163.md

## Brief Summary

Lack of parameter validation can lead to contract malfunction.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_31_group

# Approval Reuse Vulnerability could lead to financial loss for the protocol.

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-164
- **Submitter:** DCENT09
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/164
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-164.md

## Brief Summary

Approval Reuse Vulnerability could lead to financial loss for the protocol.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_21_group

# Insecure Use of periodSecondsInsideX96 Calculation could lead to an incorrect distribution of rewards, either under or over-paying users.

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-168
- **Submitter:** DCENT09
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/168
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-168.md

## Brief Summary

Insecure Use of periodSecondsInsideX96 Calculation could lead to an incorrect distribution of rewards, either under or over-paying users.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Exposure of Sensitive Information via tokenURI could enable front-running, diminishing user returns or revealing user behavior that can be exploited.

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-173
- **Submitter:** DCENT09
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/173
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-173.md

## Brief Summary

Exposure of Sensitive Information via tokenURI could enable front-running, diminishing user returns or revealing user behavior that can be exploited.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Improper Authorization Check in burn Function could lead to unintended financial losses for users, as they may find their tokens permanently removed.

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-174
- **Submitter:** DCENT09
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/174
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-174.md

## Brief Summary

Improper Authorization Check in burn Function could lead to unintended financial losses for users, as they may find their tokens permanently removed.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Insufficient Input validation could lead to unexpected losses for users or allow for denial of service if certain conditions are not met.

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-178
- **Submitter:** DCENT09
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/178
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-178.md

## Brief Summary

Insufficient Input validation could lead to unexpected losses for users or allow for denial of service if certain conditions are not met.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_05_group

# Untrusted external calls can lead to potential manipulation of contract logic or unintended fund transfers.

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-180
- **Submitter:** DCENT09
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/180
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-180.md

## Brief Summary

Untrusted external calls can lead to potential manipulation of contract logic or unintended fund transfers.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_13_group

# Integer Overflow Risk in Protocol Fee Calculation

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-24
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/24
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-24.md

## Brief Summary

- **Financial Discrepancies**: If the protocol fee is miscalculated due to overflow, it could lead to incorrect distributions of fees among liquidity providers or the protocol itself. This could ultimately result in financial losses for users and damage to the protocol's integrity. Summary - **Bug**: The risk of integer overflow when casting `delta` to `uint128`. - **Cause**: The calculation of `delta` can exceed the maximum value of `uint128` if `step.feeAmount` and `cache.feeProtocol` produce a large enough product. - **Likelihood**: This issue is likely to occur under conditions of high trading volume or high fees, where `step.feeAmount` may reach significant values. - **Impact**: Miscal...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# `getFeeGrowthInside` can incorrectly return zero because of fee growth underﬂow

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-26
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/26
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-26.md

## Brief Summary

some user transactions will revert.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# `Oracle.sol#observeSingle()` function calculates `tickCumulative` with big error.

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-181
- **Submitter:** DanielArmstrong
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/181
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-181.md

## Brief Summary

`Oracle.sol#observeSingle()` function calculates `tickCumulative` with big error.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Position can be opened even when the particle position manger does not hold the Uniswap V3 Position NFT

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-28
- **Submitter:** Emmanuel_odhiambo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/28
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-28.md

## Brief Summary

Position can be opened even when the particle position manger does not hold the Uniswap V3 Position NFT

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Lack of Input Validation in `periodCumulativesInside` Function

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-121
- **Submitter:** JuggerNaut63
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/121
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-121.md

## Brief Summary

- The absence of validation can result in incorrect fee and liquidity calculations. - Accessing invalid tick data can lead to runtime errors, potentially causing the contract to revert or behave unexpectedly.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Mismatched Data Type in Fee Growth Calculation

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-122
- **Submitter:** JuggerNaut63
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/122
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-122.md

## Brief Summary

Since Solidity 8.0+ has fixed the underflow/overflow issue, users trying to perform a swap will experience transaction failure, resulting in service disruption to the affected functions.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Incorrect Liquidity Adjustment Due to Unsafe Casting in RamsesV3Pool

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-31
- **Submitter:** JuggerNaut63
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/31
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-31.md

## Brief Summary

The RamsesV3Pool contract contains a critical vulnerability in its liquidity adjustment mechanism due to unsafe casting operations. This can lead to incorrect liquidity management. Root Cause The vulnerability arises from the unsafe casting of large values into smaller data types without proper checks. This casting operation is used in the `_modifyPosition` function when adjusting liquidity, particularly in operations such as mint and burn. Initial Conditions: - The `_modifyPosition` function that controls the liquidity changes in the pool. - `amount` is the value we want to add or subtract from the liquidity, and the loss is very large, for example `amount = 2^130`. Scenario: 1. Data Type...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Concurrent Liquidity Collection Issue in RamsesV3Pool

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-32
- **Submitter:** JuggerNaut63
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/32
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-32.md

## Brief Summary

The `collect` function allows multiple transactions to access and modify the same liquidity position at the same time. This can lead to situations where users withdraw more tokens than they are entitled to, due to overlapping transaction processing. Root Cause The issue arises from the absence of a mechanism to ensure that only one transaction can modify a liquidity position at any given time. The operations that read and modify `tokensOwed0` and `tokensOwed1` are not protected against simultaneous execution by multiple transactions. Initial Conditions: - User A has a liquidity position with `tokensOwed0` and `tokensOwed1` of 100 tokens each. - The `collect` function does not have a locking...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Wrong period linking in multiple period advancements

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-33
- **Submitter:** K42
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/33
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-33.md

## Brief Summary

Wrong period linking in multiple period advancements

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# CREATE2 address collision during pool deployment may result in complete draining of the pool

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-34
- **Submitter:** MSaptarshi
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/34
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-34.md

## Brief Summary

CREATE2 address collision during pool deployment may result in complete draining of the pool

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# RamsesV3 pools continue using outdated default feeProtocol values leading to incorrect fee splits

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-123
- **Submitter:** MrPotatoMagic
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/123
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-123.md

## Brief Summary

When fees are collected from users on swap() and flash() function calls, the protocol charges `feeProtocol`. This `feeProtocol` percentage is sent to the FeeCollector while the remaining is kept for the LPs to claim ([reference](https://discord.com/channels/810916927919620096/1290740440873697320/1293674297616896093)). When pools do not have a specific feeProtocol value, they use the default `feeProtocol` value (currently 80) from the factory. **Issue:** The issue is that the swap() and flash() function calls retrieve the `feeProtocol` value from slot0(). This means that if the `feeProtocol` value is updated using the [setFeeProtocol()](https://github.com/code-423n4/2024-10-ramses-exchange/b...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Attacker can advance period for uninitialized pools to permanently manipulate tickCumulative

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-185
- **Submitter:** MrPotatoMagic
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/185
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-185.md

## Brief Summary

When a pool is created using the function [createPool()](https://github.com/code-423n4/2024-10-ramses-exchange/blob/4a40eba36bc47eba8179d4f6203a4b84561a4415/contracts/CL/core/RamsesV3Factory.sol#L71), we know it is possible to not initialize it in the same call as seen [here](https://github.com/code-423n4/2024-10-ramses-exchange/blob/4a40eba36bc47eba8179d4f6203a4b84561a4415/contracts/CL/core/RamsesV3Factory.sol#L99C9-L102C10). This behaviour is further backed by the fact that we know functions swap(), flash(), increaseObservationCardinalityNext(), mint(), collect(), burn(), setFee(), setFeeProtocol() and collectProtocol() are disallowed from calling until the pool is initialized. This is be...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing tokenTotalSupplyByPeriod decrement after reward claim could lead to users tapping into other period rewards

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-188
- **Submitter:** MrPotatoMagic
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/188
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-188.md

## Brief Summary

Missing tokenTotalSupplyByPeriod decrement after reward claim could lead to users tapping into other period rewards

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_07_group

# Incorrect remainingTime calculation in left() leads to incorrect distributions

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-37
- **Submitter:** MrPotatoMagic
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/37
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-37.md

## Brief Summary

The [left()](https://github.com/code-423n4/2024-10-ramses-exchange-test-coverage/blob/9da1c98ab902b2dbcb1e121542de28022099b1e7/contracts/CL/gauge/GaugeV3.sol#L114) function in GaugeV3.sol will always return the full `tokenTotalSupplyByPeriod`. The issue occurs since the `remainingTime` variable calculation is incorrect and will always return 1 WEEK. Due to this, since the left() function is utilized in the Voter.sol contract's [_distribute()](https://github.com/code-423n4/2024-10-ramses-exchange-test-coverage/blob/9da1c98ab902b2dbcb1e121542de28022099b1e7/contracts/Voter.sol#L819) function, the distribution logic executed would be incorrect.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Missing Access Control in `initialize` Function

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-192
- **Submitter:** Mrxstrange
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/192
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-192.md

## Brief Summary

* The `initialize` function lacks access control, allowing any external caller to invoke it and potentially reinitialize high contract state variables, such as `tick`, `observationIndex`, and `feeProtocol`. This could lead to manipulation of the pool's parameters and unauthorized reinitializations if the `sqrtPriceX96` reset due to an error or bug. **Vulnerable Code**: RamsesV3Pool.sol#L152 **Impact**: - **Unauthorized Reinitialization**: An attacker could reset critical pool parameters, leading to incorrect calculations, logging, or protocol manipulation. - **State Inconsistency**: Modifying parameters like `tick` and `feeProtocol` could destabilize or disrupt pool functionality.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_30_group

# Redundant and Inconsistent Period Advancement in `swap` and `_advancePeriod` Functions

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-193
- **Submitter:** Mrxstrange
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/193
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-193.md

## Brief Summary

The `swap` function and `_advancePeriod` function both contain logic for advancing the period, which may lead to redundant updates and inconsistencies. The `_advancePeriod` function is `public`, allowing it to be called externally, resulting in possible unexpected or duplicate period changes that disrupt `swap`'s calculations. **Vulnerable Code**: RamsesV3Pool.sol#L420 **Impact**: - **Redundant Period Updates**: Both functions try to handle period transitions independently, leading to duplicate state updates within the same week. - **Inconsistent Observations**: Direct calls to `_advancePeriod` during a swap could lead to incorrect logging, state inconsistencies, and observation inaccuracie...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_19_group

# Accounting Error in NonfungiblePositionManager Prevents Users from Burning Positions

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-125
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/125
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-125.md

## Brief Summary

The current implementation in the `NonfungiblePositionManager` contract has a critical issue that can potentially block users from exiting their positions. This problem stems from the discrepancy between actual collected amounts and recorded amounts in the `collect` function, which directly impacts the `burn` function.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Timing Flaw in NonfungiblePositionManager Allows Unearned Fee Collection on New Positions

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-126
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/126
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-126.md

## Brief Summary

The NonfungiblePositionManager contract is responsible for managing liquidity positions as NFTs in a decentralized exchange system. The mint function is used to create new liquidity positions. There's a timing discrepancy in the mint function where fee growth snapshots are taken after liquidity is added, potentially allowing new positions to claim fees for a period before they were actually created. In the current implementation of mint(), liquidity is added to the pool before fee growth snapshots are taken. These snapshots are then used as the initial values for the new position. This sequence allows the position to claim fees generated during the brief period between liquidity addition an...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Fee Calculation Error in decreaseLiquidity Leads to Overpayment on Partial Liquidity Removal

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-127
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/127
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-127.md

## Brief Summary

The NonfungiblePositionManager contract manages liquidity positions as NFTs and handles fee collection for liquidity providers. The fee calculation relies on tracking fee growth values for each position. The issue is in the decreaseLiquidity function. When liquidity is decreased, the fee growth calculation doesn't account for the reduced liquidity, potentially leading to an overestimation of fees owed. The fee calculation uses the full position liquidity (positionLiquidity) instead of the remaining liquidity after the decrease. This can result in overpayment of fees when a user partially removes liquidity.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unchecked Liquidity Addition in increaseLiquidity Function Leads to Protocol-Wide Position and Fee Calculation Corruption

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-194
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/194
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-194.md

## Brief Summary

The `increaseLiquidity` function in the Ramses V3 Position Manager contains an arithmetic overflow vulnerability in its liquidity accounting due to an unchecked addition operation on a uint128 value. The vulnerability exists where new liquidity is added to an existing position's balance within an unchecked block, likely implemented as a gas optimization. A silent overflow in position liquidity tracking could lead to incorrect position accounting, fee calculations, and pool share computations throughout the protocol. Impact When examining the integer overflow vulnerability in the `increaseLiquidity` function, we need to understand how it propagates through the contract's core accounting mech...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_14_group

# Unvalidated Reward Token Removal Can Delete Wrong Token Due to Default Index Value

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-196
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/196
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-196.md

## Brief Summary

The `removeRewards` function has a vulnerability where it fails to validate whether a reward token was actually found in the `rewards` array before proceeding with removal operations. When attempting to remove a token that exists in `isReward` mapping but not in the `rewards` array, the function will remove the first token in the array (at index 0) due to Solidity's default value initialization of `uint256 idx` to 0. While this issue requires pre-existing state inconsistency between `rewards` array and `isReward` mapping to be exploitable, it represents a failure of defensive programming that could lead to unintended reward token removals and disruption of the reward distribution system. Co...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Inefficient Array Shifting in removeRewards function Leads to Excessive Gas Costs and Potential DOS

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-197
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/197
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-197.md

## Brief Summary

The current `removeRewards` function implementation uses a shifting pattern that incurs significant gas costs, particularly for large arrays: Each shift operation costs approximately 3000 gas (SLOAD + SSTORE), and removing an element at index `i` requires `(n-i-1)` shifts. For a rewards array of size n, this results in O(n) gas complexity, making it prohibitively expensive for large arrays. The optimal solution is to use swap-and-pop: This reduces the operation to O(1) complexity with only two storage writes, regardless of array size or removal position. While this changes array ordering, it's an acceptable tradeoff since reward token order isn't functionally important in this contract. The...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Slippage Protection in RamsesV3Pool Fee Collection

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-198
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/198
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-198.md

## Brief Summary

The FeeCollector contract collects protocol fees from RamsesV3Pool pools for distribution to treasury and fee distributors. The collection process uses maximum amounts without validating actual received values. The contract always requests maximum uint128 fees without enforcing minimum received amounts: Since there's no validation on `_amount0` and `_amount1` returned from `collectProtocol()`, callers cannot specify minimum expected amounts. Impact - Protocol fees can be collected with unexpectedly low values - No way to revert transactions when collected amounts are below expectations - Integration risk for systems relying on minimum fee collection amounts

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Single-Signer Voter Control Over Gauge Creation Without Timelock Enables Irreversible Protocol Takeover

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-200
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/200
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-200.md

## Brief Summary

The ClGaugeFactory implements a single-signer authorization model where the voter address has unrestricted power to deploy new gauges. If this key is compromised, an attacker gains complete control over gauge deployment, potentially leading to system-wide manipulation of liquidity incentives and rewards distribution. Impact This seemingly simple authorization check, `require(msg.sender == voter, "AUTH")`, opens up a dangerous security hole that's exacerbated by multiple design decisions. Not only does the contract trust this single address for all gauge deployments, but it also executes these deployments instantly without any safety checks or cooling periods. An attacker who compromises the...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_22_group

# Tick Reinitialization Loses Historical Fee Growth Data Leading to Incorrect Fee Distribution

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-203
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/203
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-203.md

## Brief Summary

The `update` function makes a critical assumption during tick initialization that all fee growth and accumulator values occurred below the tick when `tick <= tickCurrent`. This oversimplified initialization model can lead to incorrect fee accounting, especially in volatile market conditions or when ticks are reinitialized after prior usage. The assumption fails to account for complex market movements and can result in improper fee distribution. The vulnerable section of code lies in the initialization logic of the `update` function: The issue becomes apparent when we consider how this interacts with fee accounting. Let's examine a sequence of market events: The problem becomes evident in th...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Tick Range and Spacing Validation Allows Creation of Invalid Positions Outside Protocol Boundaries

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-204
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/204
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-204.md

## Brief Summary

The `update` function lacks comprehensive validation of tick parameters, particularly regarding tick range bounds and spacing requirements. This allows initialization of ticks outside protocol-defined boundaries and with invalid spacing, potentially leading to pool invariant violations, integration issues with price oracles, and systemic risks to the AMM's operation. The `update` function accepts tick parameters without proper validation: Impact This seemingly simple oversight opens up a series of complex issues. The AMM protocol defines specific boundaries and spacing requirements for ticks, but these constraints are not enforced at the tick level. Let's explore why this is problematic. In...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_29_group

# Unbounded Oracle Growth Enables DoS Attacks in AMM Pool

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-43
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/43
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-43.md

## Brief Summary

The Oracle library is designed to manage price and liquidity observations for an AMM pool. The grow function is meant to increase the capacity of the observation array, allowing for more historical data to be stored. The grow function allows for increasing the size of the observation array without an upper bound check. Code Impact This can lead to several issues: 1. Unbounded Growth: The function allows `next` to be any value up to 65535 (the maximum value of uint16) without any upper bound check. This can lead to the array growing to an impractically large size. 2. Gas Limit Vulnerability: If `next` is set to a very large value, the for-loop could consume an enormous amount of gas, potenti...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Risk of Unauthorized Liquidity Modification and Position Tampering

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-208
- **Submitter:** Sathish9098
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/208
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-208.md

## Brief Summary

Unauthorized liquidity modification disrupts ``ownership integrity``, allowing attackers to alter positions without consent. This leads to inconsistent fee accumulation, causing misaligned earnings for the owner. Unauthorized liquidity injections can also skew capital efficiency, exposing the position to impermanent loss or unexpected risks. Excessive liquidity additions may lock the owner's capital, limiting their ability to rebalance or withdraw. Risk Explanation In the provided code, the ``increaseLiquidity`` function lacks a proper authorization check to verify whether the caller (``msg.sender``) is the owner or an approved operator for the position identified by ``tokenId``. Without ac...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_00_group

# Accumulated Token Allowances in FeeCollector Contract

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-46
- **Submitter:** Sparrow
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/46
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-46.md

## Brief Summary

Accumulated Token Allowances in FeeCollector Contract

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Position::positionHash() function is vulnerable to Hash collision

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-47
- **Submitter:** TECHFUND-inc
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/47
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-47.md

## Brief Summary

Position::positionHash() function is vulnerable to Hash collision

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Tick data in the storage could be stale, primarily due to how tick data is being deleted

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-48
- **Submitter:** TECHFUND-inc
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/48
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-48.md

## Brief Summary

Tick data in the storage could be stale, primarily due to how tick data is being deleted

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Anyone can change protocol fee for pool

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-50
- **Submitter:** Timenov
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/50
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-50.md

## Brief Summary

In `RamsesV3Pool.sol`, there is a function `setFeeProtocol` which will change the `feeProtocol`. This function inherits `IRamsesV3PoolOwnerActions`. This interface holds crucial functions just like `collectProtocol` and `setFee`. However the `setFeeProtocol` is not protected and anyone can call it. Impact Having no access control on this crucial function, means that attacker can either set the protocol fee to 0 and not pay fee or increase the fee to grief other users.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# OOG / unexpected reverts due to incorrect usage of staticcall.

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-214
- **Submitter:** Tomasleocadio
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/214
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-214.md

## Brief Summary

According to the solidity docs, if a staticcall encounters a state change, it burns up all gas and returns. The function `GaugeV3::periodEarned` makes a staticcal to the `GaugeV3::cachePeriodEarned` wich makes a staticcal to `IRamsesV3PoolState`. The issue is that this burns up all the gas sent with the call. According to EIP150, a call gets allocated 63/64 bits of the gas, and the entire 63/64 parts of the gas is burnt up after the staticcall, since the staticcall will always encounter a storage change. Impact This causes the contract to burn up 63/64 bits of gas in a single check. In the functions mentioned this will lead to DoS of the contract, the call can revert due to running out of g...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Use of `slot0` to get `sqrtPriceLimitX96` can lead to price manipulation.

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-215
- **Submitter:** Tomasleocadio
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/215
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-215.md

## Brief Summary

In `RamsesV3Pool`, the function `swap` use `UniswapV3.slot0` to get the value of `sqrtPriceX96`, which is used to perform the swap. However, the `sqrtPriceX96` is pulled from `Uniswap.slot0`, which is the most recent data point and can be manipulated easily via `MEV` bots and `Flashloans` with sandwich attacks; which can cause the loss of funds when interacting with the `Uniswap.swap` function.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# The pool creation in `PoolAddress` is prone to collision attacks. Wich will lead to malicious pool and lost of all funds.

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-216
- **Submitter:** Tomasleocadio
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/216
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-216.md

## Brief Summary

Using computed pool addresses to create the pool is collision attack prone and can be abused in order to steal all token allowances of the contract. Vulnerability Detail In the `NonfungiblePositionManager` contract, the functions `collect` and `decreaseLiquidity` (**payable** functions) use the `computeAddress` function in `PoolAddress` contract. This function is prone to collision attack as the computation of the Create2 address is done by truncating a 256 bit keccak256 hash to 160 bits, meaning that for each address there are 2^96 possible hashes that will result in it after being truncated. Furthermore, what this means is that if a `deployer` and `key` combination that results in an addr...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_11_group

# Risk of locked assets due to use of `_mint` instead of `_safeMint`

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-217
- **Submitter:** Tomasleocadio
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/217
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-217.md

## Brief Summary

The `NonfungiblePositionManager` contract uses ERC-721 tokens that are minted via the `_mint` function rather than the `_safeMint` function. The `_safeMint` function includes a necessary safety check that validates a recipient contract’s ability to receive and handle ERC-721 tokens. Without this safeguard, tokens can inadvertently be sent to an incompatible contract, causing them, and any assets they hold, to become irretrievable. The _safeMint function’s built-in safety check ensures that the recipient contract has the necessary ERC721Receiver implementation, verifying the contract’s ability to receive and manage ERC-721 tokens. [openzeppelin-contracts/contracts/token/ERC721/ERC721.sol](ht...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# mismatch parameter in swap function

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-128
- **Submitter:** air_0x
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/128
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-128.md

## Brief Summary

The [swap( )](https://github.com/code-423n4/2024-10-ramses-exchange/blob/4a40eba36bc47eba8179d4f6203a4b84561a4415/contracts/CL/core/RamsesV3Pool.sol#L363) function is broken due to mismatch parameters

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Mismatch in order of parameters

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-129
- **Submitter:** air_0x
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/129
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-129.md

## Brief Summary

The [addLiquidity](https://github.com/code-423n4/2024-10-ramses-exchange/blob/4a40eba36bc47eba8179d4f6203a4b84561a4415/contracts/CL/periphery/NonfungiblePositionManager.sol#L210) function is called in the [increaseLiquidity()](https://github.com/code-423n4/2024-10-ramses-exchange/blob/4a40eba36bc47eba8179d4f6203a4b84561a4415/contracts/CL/periphery/NonfungiblePositionManager.sol#L196) function . However due to mismatch in order of paramters , the call will be broken.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# owner check can be bypassed to claim reward

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-130
- **Submitter:** air_0x
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/130
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-130.md

## Brief Summary

In the function [getPeriodReward()](https://github.com/code-423n4/2024-10-ramses-exchange/blob/4a40eba36bc47eba8179d4f6203a4b84561a4415/contracts/CL/gauge/GaugeV3.sol#L489), the check `require(msg.sender == owner)` can be bypassed to claim rewards. This issue arises because the owner parameter is passed by the caller, meaning an they could pass their own address as owner to pass the check. This will allow an attacker to claim reward from any position

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# updating of minter positions depends on `slot0` price which can be manipulated

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-51
- **Submitter:** air_0x
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/51
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-51.md

## Brief Summary

Anyone can invoke the `mint( )` function to add LP position within their desired range. The function depends on _modifyPosition( ) to calculate `amount0Int` and `amount1Int` to be transferred to the pool. However, it depends on `slot0` price, which can be manipulated, potentially leading to the LP position being out of the current range and resulting in a loss of yield. Impact Attackers can manipulate the `slot0` price data points to cause loss of yield for the protocol.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Common tokens such as WETH9 work differently on chains such a Blast which isn't taken into account during transfer calls

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-131
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/131
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-131.md

## Brief Summary

Common tokens such as WETH9 work differently on chains such a Blast which isn't taken into account during transfer calls

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# No check for ticks range in periodCumulativesInside

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-52
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/52
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-52.md

## Brief Summary

No check for ticks range in periodCumulativesInside

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# safeMint should be used in place of mint

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-53
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/53
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-53.md

## Brief Summary

safeMint should be used in place of mint

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# check for pool address in createPool.

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-55
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/55
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-55.md

## Brief Summary

check for pool address in createPool.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# KatanaV3Pool.observe() will likely revert when the current timestamp wraps around the 2**32 boundary, and after time >= 2**32, observations before < 2**32 all become not available to observe,

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-220
- **Submitter:** chaduke
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/220
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-220.md

## Brief Summary

KatanaV3Pool.observe() will likely revert when the current timestamp wraps around the 2**32 boundary, and after time >= 2**32, observations before < 2**32 all become not available to observe,

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# NonfungiblePositionManager.mint() might leave some ETH In the contract, subject to being stolen by another user.

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-221
- **Submitter:** chaduke
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/221
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-221.md

## Brief Summary

NonfungiblePositionManager.mint() might leave some ETH In the contract, subject to being stolen by another user.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_08_group

# Setting the fee protocol for a pool in RamesV3Factory will NOT change the actual fee protocol for that pool IMMEDATELY.

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-222
- **Submitter:** chaduke
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/222
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-222.md

## Brief Summary

Setting the fee protocol for a pool in RamesV3Factory will NOT change the actual fee protocol for that pool IMMEDATELY.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_37_group

# GaugeV3.notifyRewardAmount() does not roll over unclaimed emission due to no stakers from previous period, these emission will be stuck in the gauge forever.

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-223
- **Submitter:** chaduke
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/223
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-223.md

## Brief Summary

GaugeV3.notifyRewardAmount() does not roll over unclaimed emission due to no stakers from previous period, these emission will be stuck in the gauge forever.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# The two getPeriodReward() uses different way of calcuating positionHash, so one of them must be wrong.

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-226
- **Submitter:** chaduke
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/226
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-226.md

## Brief Summary

The two getPeriodReward() uses different way of calcuating positionHash, so one of them must be wrong.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Use of block.timestamp for Period Calculation introduces a risk of manipulation

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-227
- **Submitter:** chupinexx
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/227
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-227.md

## Brief Summary

Use of block.timestamp for Period Calculation introduces a risk of manipulation

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unrestricted Access to collectProtocolFees and Missing Initial Treasury Fee Setting

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-229
- **Submitter:** chupinexx
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/229
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-229.md

## Brief Summary

The `collectProtocolFees` function is publicly accessible, meaning any address can call it without restriction. This open access introduces potential risks, as unauthorized users could trigger fund distribution and potentially interfere with protocol operations. Additionally, if the `setTreasuryFees` function is not called to set an initial treasury fee after the deployment of `Feecollector` contract , the default `treasuryFees` value remains at `zero`. As a result, any funds collected by `collectProtocolFees` would be distributed entirely to `voters`, leaving `the treasury` with `zero` allocation. This could occur due to an oversight or misconfiguration post-deployment, inadvertently depri...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_04_group

# Unrestricted Pool Creation Leads to Potential Denial of Service in `setPoolFeeProtocolBatch`

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-232
- **Submitter:** chupinexx
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/232
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-232.md

## Brief Summary

The `createPool` function allows any user to create a pool without restrictions on frequency or quantity, leading to a risk of excessive pool creation. The `setPoolFeeProtocolBatch` function subsequently iterates over an array of pool addresses to set the fee protocol, looping through each pool in the array. Due to the unrestricted pool creation, a malicious user could inflate the number of pools to the point where executing `setPoolFeeProtocolBatch` results in excessive gas consumption or even out-of-gas errors. Additionally, if any call to `setFeeProtocol` within this function fails, the entire batch operation reverts, potentially blocking protocol updates. Impact: This vulnerability allo...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Inconsistent Basis Point (BPS) Standards for Percentage Calculations

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-234
- **Submitter:** chupinexx
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/234
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-234.md

## Brief Summary

The RamsesV3 contract suite demonstrates inconsistent use of basis points (BPS) when calculating and setting percentages across different functions. There are at least three different standards observed: -10,000 BPS = 100% (standard convention where 1 BPS = 0.01%) -100 BPS = 100% (interpreted as a percentage directly out of 100) -1,000,000 BPS = 100% (implying 1 BPS = 0.0001%) This lack of uniformity can lead to serious miscalculations in fee adjustments, especially in functions that handle sensitive financial operations. For instance: -In `setTreasuryFees`, BASIS is defined as 10,000, making 100% equal to 10,000 BPS. -In `setFeeProtocol`, the maximum `_feeProtocol` is 100, suggesting that...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_42_group

# Unclear Error Handling in setPoolFeeProtocolBatch

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-238
- **Submitter:** coders99
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/238
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-238.md

## Brief Summary

Unclear Error Handling in setPoolFeeProtocolBatch

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_38_group

# ClGaugeFactory is missing re-entrancy guard on the createGauge function

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-134
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/134
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-134.md

## Brief Summary

The createGauge function lacks a reentrancy guard, making it susceptible to reentrancy attacks. vulnerability to repeatedly call createGauge before the state is updated, potentially deploying multiple GaugeV3 contracts for the same pool. Specifically, the state update getGauge[pool] = gauge; occurs after the deployment of the new GaugeV3 contract. If the GaugeV3 constructor or any external call within it can re-enter the createGauge function, it may bypass the require(getGauge[pool] == address(0), "GE") check, allowing multiple gauges to be created for the same pool.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# GaugeV3 has Off-by-One Error in Reward Accounting Allows Double-Claiming of Rewards

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-135
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/135
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-135.md

## Brief Summary

The GaugeV3 smart contract contains an off-by-one error in the _getAllRewards function. This flaw allows users to double-claim rewards for the period immediately preceding the current period, leading to potential over-distribution of rewards. In the _getAllRewards function, after processing and distributing rewards for all periods up to the current period, the contract updates the lastClaimByToken mapping by setting it to currentPeriod - 1. But the loop runs to the currentPeriod - 0. This results in the previous period (currentPeriod - 1) being eligible for reward claims again in subsequent calls, enabling users to claim rewards for the same period multiple times.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# NonfungiblePositionManager has Incorrect Inheritance Order Causes Balance Not Updating in NonfungiblePositionManager

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-60
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/60
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-60.md

## Brief Summary

The NonfungiblePositionManager contract fails to update user balances correctly due to improper inheritance order and incorrect use of super in overridden functions. This results in incorrect balance tracking, which can lead to discrepancies in token ownership, unauthorized transfers, and severe security vulnerabilities in the token management system. Incorrect inheritance declaration Overridden functions with incorrect super calls

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Position has The _updatePosition function should be internal not external

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-63
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/63
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-63.md

## Brief Summary

The _updatePosition function in the Position library is marked as external. Allowing it to be called by any external contract or user. Given the underscore _ prefix conventionally indicates an internal function intended for use within the contract. Exposing it externally can lead to unintended access. External callers can invoke _updatePosition, bypassing any intended access controls. Vulnerable Code

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# FeeCollector has Missing Access Control on collectProtocolFees Function

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-64
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/64
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-64.md

## Brief Summary

The collectProtocolFees function is missing the onlyTreasury modifier that allows only the treasury user to call this function. This can lead to unauthorized fee collection and potential mismanagement of protocol fees. Unauthorized users can call the collectProtocolFees function, potentially disrupting the protocol’s fee collection and distribution mechanism. Vulnerable Code The function is missing the onlyTreasury modifier, which should restrict access to the treasury address.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# RamsesV3Pool Contract Reentrancy Vulnerability in mint Function

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-67
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/67
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-67.md

## Brief Summary

Allows reentrancy in the mint function leading to inconsistent state or potential exploits. The RamsesV3Pool contract’s mint function is vulnerable to a reentrancy attack. The issue arises because the advancePeriod modifier is publicly accessible and can be called externally. Since the lock modifier is applied before advancePeriod, an attacker can re-enter the mint function through the advancePeriod function before the state changes are fully applied. This can lead to inconsistent state updates or enable other malicious behaviors. Vulnerable Code

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Pool Price Initialization in `RamsesV3Factory.createPool` Due to Token Reordering

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-136
- **Submitter:** elvin-a-block
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/136
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-136.md

## Brief Summary

In the `RamsesV3Factory.createPool` function, the input tokens (`tokenA` and `tokenB`) are lexicographically sorted to determine the final `token0` and `token1`. However, the `sqrtPriceX96` parameter, which represents the square root of the initial price of the pool, is passed by the external caller assuming the price reflects the ratio between `tokenA` and `tokenB`. Since the function internally reorders the tokens based on their address values (`token0 = min(tokenA, tokenB)`), there is a mismatch in the interpretation of `sqrtPriceX96`. If the tokens are reordered, the price ratio changes (it should be inverted), but the external caller may not account for this, resulting in an incorrect...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# High Centralization Risk Due to Single-Entity Access Control

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-68
- **Submitter:** enami_el
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/68
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-68.md

## Brief Summary

**Description**: The contract heavily relies on the `AccessManaged` contract for access control. This means that a single entity (the access manager) has significant control over critical functions, posing a centralization risk that goes against the principles of decentralization in DeFi. **Vulnerable Code**:

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Unchecked External Call in Pool Deployment

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-69
- **Submitter:** enami_el
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/69
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-69.md

## Brief Summary

**Title**: Lack of Error Handling in External Pool Deployment Call **Description**: The `createPool` function in the `RamsesV3Factory` contract makes an external call to the `deploy` function of `IRamsesV3PoolDeployer` without proper error handling. The `deploy` function itself uses the `new` keyword to create a new `RamsesV3Pool` contract, which can potentially fail. **Vulnerable Code**: In `RamsesV3Factory`: In `IRamsesV3PoolDeployer`: **Impact**: 1. If the pool deployment fails (e.g., due to out-of-gas errors or contract size limitations), the `createPool` function will not catch this failure. 2. This could lead to an inconsistent state where the factory contract believes a pool has been...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Slippage Protection and Deadline in Pool Initialization

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-70
- **Submitter:** enami_el
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/70
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-70.md

## Brief Summary

The `createPool` function allows initializing a new pool with a specific square root price (`sqrtPriceX96`). However, it lacks two critical protection mechanisms: 1. Slippage protection: There's no way to specify an acceptable price range. 2. Transaction deadline: There's no expiration for the pool creation transaction. **Vulnerable Code**: **Impact**: 1. **Price Manipulation**: In volatile markets, the initial price set during pool creation could be significantly different from the current market price by the time the transaction is mined, potentially leading to immediate arbitrage opportunities against the pool creator. 2. **Front-running**: Malicious actors could front-run the pool creat...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Insufficient Validation of Tick Spacing

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-71
- **Submitter:** enami_el
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/71
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-71.md

## Brief Summary

**Title**: Potential for Unbounded Growth in Tick Spacings **Description**: The `enableTickSpacing` function allows for the dynamic addition of new tick spacings. While there are some bounds checks, this feature could lead to an ever-growing list of tick spacings over time. **Vulnerable Code**: **Impact**: 1. **Gas Cost Increase**: Functions that iterate over all tick spacings could become increasingly expensive as more tick spacings are added. 2. **Potential DoS**: If too many tick spacings are added, functions that loop through all tick spacings might hit the block gas limit, causing transactions to fail. 3. **Complexity in Pool Management**: A large number of tick spacings could make poo...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Potential for Sandwich Attacks

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-73
- **Submitter:** enami_el
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/73
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-73.md

## Brief Summary

The current implementation of the `swap` function is vulnerable to sandwich attacks. In a sandwich attack, a malicious actor observes a pending large swap transaction, front-runs it with their own transaction to manipulate the price, allows the victim's transaction to execute at an unfavorable price, and then back-runs with another transaction to profit from the price change. **Vulnerable Code**: The entire `swap` function is vulnerable, but particularly: **Impact**: 1. Users could suffer significant financial losses due to unfavorable execution prices. 2. The pool's reputation could be damaged, leading to reduced liquidity and usage. **

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Token Recovery Mechanism

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-75
- **Submitter:** enami_el
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/75
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-75.md

## Brief Summary

**Title**: Absence of a Token Recovery Function for Accidentally Sent Tokens **Description**: The RamsesV3Pool contract does not have a mechanism to recover tokens that might be accidentally sent directly to the contract address. This could lead to permanent loss of tokens for users who mistakenly transfer tokens to the pool contract outside of the intended functions. **Vulnerable Code**: The vulnerability is not in existing code, but rather in the absence of a recovery function. **Impact**: 1. Users who accidentally send tokens directly to the pool contract address may permanently lose access to those tokens. 2. The contract could accumulate "stuck" tokens over time, which cannot be utiliz...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Address Verification in setFeeCollector

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-80
- **Submitter:** enami_el
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/80
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-80.md

## Brief Summary

**Title**: Insufficient Address Validation in setFeeCollector Function **Description**: The `setFeeCollector` function allows changing the address that collects fees without proper validation. It doesn't check if the new address is valid (not zero) or if it's different from the current address. This could lead to accidental loss of fee collection capabilities if an invalid or incorrect address is set. **Vulnerable Code**: **Impact**: If the `_feeCollector` address is set to the zero address or an address that cannot receive funds, it could result in permanent loss of fee collection functionality. This could lead to significant financial losses for the protocol and its stakeholders.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Repeated Reward Claims Due to Inadequate Supply Tracking

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-245
- **Submitter:** firmanregar
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/245
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-245.md

## Brief Summary

Repeated Reward Claims Due to Inadequate Supply Tracking

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_09_group

# Unsafe casting leads to incorrect calculations

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-247
- **Submitter:** firmanregar
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/247
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-247.md

## Brief Summary

Unsafe casting leads to incorrect calculations

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Error Message in `require()` Statement

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-250
- **Submitter:** godwin-x
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/250
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-250.md

## Brief Summary

The `require` statement lacks an error message, which is against Solidity best practices. In the event this require condition fails, the transaction will revert without a descriptive message, making it difficult for developers or users to understand why the function call failed. Including an error message in require statements is essential for debugging, operational clarity, and code maintainability. Suggested Fix Add a meaningful error message to the `require` statement, such as: ETC...!!!

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# protocol do not work with rebasing tokens

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-81
- **Submitter:** grearlake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/81
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-81.md

## Brief Summary

In `RamsesV3Pool` contract, current design of protocol are not able to work with rebasing token, as there is no mechanism to know how many token are rebased up/down when burning position: function burn( uint256 index, int24 tickLower, int24 tickUpper, uint128 amount ) external override lock advancePeriod returns (uint256 amount0, uint256 amount1) { unchecked { (PositionInfo storage position, int256 amount0Int, int256 amount1Int) = _modifyPosition( ModifyPositionParams({ owner: msg.sender, index: index, tickLower: tickLower, tickUpper: tickUpper, liquidityDelta: -int256(uint256(amount)).toInt128() }) ); amount0 = uint256(-amount0Int); amount1 = uint256(-amount1Int); if (amount0 > 0 || amount...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unchecked Subtraction Vulnerability Report: Mitigation and Security Analysis

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-253
- **Submitter:** hackeroid8080
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/253
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-253.md

## Brief Summary

In the function transform, the line uint32 delta = blockTimestamp - last.blockTimestamp; performs a subtraction inside an unchecked block. Here, underflow could occur if blockTimestamp is less than last.blockTimestamp, resulting in delta wrapping around to a high value, potentially causing unintended behavior downstream. Vulnerability Level Severity: Medium Impact: If underflow occurs, it could lead to a miscalculation of tickCumulative and secondsPerLiquidityCumulativeX128, propagating incorrect values throughout the contract and potentially affecting any logic that relies on accurate cumulative values. Suggested Mitigation To mitigate this, add an explicit check outside the unchecked bloc...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_60_group

# ClGaugeFactory and Gauge Creation is vulnerable to re-org attacks

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-82
- **Submitter:** holydevoti0n
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/82
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-82.md

## Brief Summary

The `createGauge` function in `ClGaugeFactory` uses `new` to create a contract without a salt, making it vulnerable to reorg attacks. Attackers can front-run and deploy a malicious contract at the same address before the legitimate transaction is mined.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Oracle price broken due to `newPeriod` logic conflicting with `transform` when writing observations

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-83
- **Submitter:** holydevoti0n
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/83
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-83.md

## Brief Summary

Ramses introduces a new concept of weekly periods registered in the Oracle's observations. Problem is that due to the logic of how a period is calculated: This causes the Oracle to override a previously registered observation(without the newPeriod logic, for instance when the user calls `swap` or `modifyLiquidity`) and vice versa. When user swaps or modify liquidity, the current timestamp is used to register/update the observation. i.e: Now that we understand how observations are handled let's understand why this is a problem: The calculation for the period rounds the value down which makes sense as `newPeriods` are created once a new week has been reached. And they are registered as follow...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# User can steal rewards due to staled `secondsDebtX96`

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-84
- **Submitter:** holydevoti0n
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/84
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-84.md

## Brief Summary

When a user modifies liquidity multiple times within the same period within the same tick range (thus generating the same hash for their position), the `initializeSecondsStart` function is only triggered during the **first liquidity update**. While the position hash remains the same, subsequent liquidity changes do not correctly update the reward-tracking variables (`secondsPerLiquidityPeriodX128` and `secondsDebtX96`), resulting in over-rewarding or under-rewarding the user due to stale values being used. Let's see how this happens: `mint/burn` => `_modifyPosition` => `_updatePosition`: In a nutshell, after the first liquidity update in a period, the `initialized` flag prevents re-initiali...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Compiling the contract in a version above 0.8.13 and below 0.8.27 means that the use of customErrors in the required functions does not work correctly

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-85
- **Submitter:** iamcarllosjr
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/85
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-85.md

## Brief Summary

Compiling the contract in a version above 0.8.13 and below 0.8.27 means that the use of customErrors in the required functions does not work correctly

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Implicit casting in uint128 can cause truncation if delta is greater than 2^128 - 1.

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-86
- **Submitter:** iamcarllosjr
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/86
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-86.md

## Brief Summary

Implicit casting in uint128 can cause truncation if delta is greater than 2^128 - 1.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Potential Colllision in the PositionHash

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-254
- **Submitter:** ifex445
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/254
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-254.md

## Brief Summary

Potential Colllision in the PositionHash

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Potential Read of Uninitialized Data in Oracle Initialization

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-255
- **Submitter:** ifex445
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/255
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-255.md

## Brief Summary

Potential Read of Uninitialized Data in Oracle Initialization

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Non-Existent AccessManaged Import from OpenZeppelin in RamseV3Factory.sol

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-257
- **Submitter:** ifex445
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/257
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-257.md

## Brief Summary

Non-Existent AccessManaged Import from OpenZeppelin in RamseV3Factory.sol

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Missing Upper Limit on Cardinality in increaseObservationCardinalityNext Function

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-258
- **Submitter:** ifex445
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/258
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-258.md

## Brief Summary

Missing Upper Limit on Cardinality in increaseObservationCardinalityNext Function

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Liquidity Threshold Check in flash Function for Flashloan

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-260
- **Submitter:** ifex445
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/260
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-260.md

## Brief Summary

Lack of Liquidity Threshold Check in flash Function for Flashloan

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Manipulation of `slot0` Price Can Lead to Inefficient Liquidity Adjustments in `_modifyPosition`

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-87
- **Submitter:** igdbase
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/87
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-87.md

## Brief Summary

The function `_modifyPosition` relies on the current price and tick from `slot0` to determine whether liquidity adjustments are required for a given position in a Uniswap V3-like liquidity pool. However, the `slot0` price is susceptible to manipulation by external actors, allowing them to force unnecessary liquidity adjustments. This can result in misallocation of tokens (either token0 or token1) and yield losses for the protocol, as the pool’s liquidity is incorrectly managed based on an artificially influenced price. If an attacker controls the tick, they can deliberately disrupt the liquidity distribution and reduce the protocol's efficiency.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# If the limit price for slippage protection is reached the transaction should revert

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-88
- **Submitter:** ignite256
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/88
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-88.md

## Brief Summary

The behaviour of Uniswap, Balancer, Curve and other AMMs is that if the price moves against the user above a defined limit the transaction reverts. This is crucial for sophisticated users who rely on Flashbots RPC or other direct access to the mempool. These users expect their transactions to be included in a block only if they execute as intended. Unfortunately, the RamsesV3Pool contract does not provide this clear expectation. The outcome is not predictable (ie. either filled as desired or nothing), which is a significant disadvantage.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Integer Overflow in 2024-10-ramses-exchange/contracts/CL/periphery/NonfungiblePositionManager.sol

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-262
- **Submitter:** ihuntpackets
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/262
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-262.md

## Brief Summary

Integer Overflow in 2024-10-ramses-exchange/contracts/CL/periphery/NonfungiblePositionManager.sol

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Access Control Issue in 2024-10-ramses-exchange/contracts/CL/core/RamsesV3PoolDeployer.sol

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-264
- **Submitter:** ihuntpackets
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/264
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-264.md

## Brief Summary

Access Control Issue in 2024-10-ramses-exchange/contracts/CL/core/RamsesV3PoolDeployer.sol

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_03_group

# Delegate Injection in 2024-10-ramses-exchange/contracts/CL/periphery/NonfungiblePositionManager.sol

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-265
- **Submitter:** ihuntpackets
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/265
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-265.md

## Brief Summary

Delegate Injection in 2024-10-ramses-exchange/contracts/CL/periphery/NonfungiblePositionManager.sol

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Reentrancy in 2024-10-ramses-exchange/contracts/CL/gauge/ClGaugeFactory.sol

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-266
- **Submitter:** ihuntpackets
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/266
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-266.md

## Brief Summary

Reentrancy in 2024-10-ramses-exchange/contracts/CL/gauge/ClGaugeFactory.sol

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_02_group

# Authorization Bypass in 2024-10-ramses-exchange/contracts/CL/core/libraries/Position.sol

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-267
- **Submitter:** ihuntpackets
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/267
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-267.md

## Brief Summary

Authorization Bypass in 2024-10-ramses-exchange/contracts/CL/core/libraries/Position.sol

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Reliance on library that uses code that wouldn't work on ZKEVMs

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-90
- **Submitter:** inh3l
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/90
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-90.md

## Brief Summary

Reliance on library that uses code that wouldn't work on ZKEVMs

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Initial rewards supplied to a gauge prior may be lost

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-92
- **Submitter:** inh3l
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/92
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-92.md

## Brief Summary

Initial rewards supplied to a gauge prior may be lost

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Slot0 should not be used as it is easy to manipulate

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-94
- **Submitter:** lightoasis
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/94
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-94.md

## Brief Summary

UniswapV3 slot0 should not be used to get price as it can be manipulated Vulnerability Details The use of slot0 to obtain sqrtPrice is heavily discouraged as it is easy to manipulate. slot0 represents the current price rather a time weighted price. An attacker can therefore use flash loans to shift the slot0 by doing large swaps on Uniswap.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Potencial division by zero error

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-268
- **Submitter:** linemi
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/268
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-268.md

## Brief Summary

The division by zero vulnerability has several serious implications: - Transaction Reverts: The protocol’s core functions will revert when liquidity is zero, preventing essential operations such as calculating historical liquidity and tick data. - Denial of Service (DoS): If an attacker can manipulate liquidity to be zero, they could exploit this to cause a DoS condition by repeatedly triggering the division by zero and making it impossible to execute certain functions. - Inaccurate Calculations: In cases where the code handles division by zero by returning fallback values (e.g., 1), the accuracy of liquidity and tick calculations may degrade, potentially leading to mispriced swaps and inco...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Tick=>update not allow unchecked

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-97
- **Submitter:** minglei-wang-3570
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/97
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-97.md

## Brief Summary

Tick=>update not allow unchecked

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# On Pool initialization Oracle.initialize is called with a value for time 0

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-269
- **Submitter:** nslavchev
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/269
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-269.md

## Brief Summary

On Pool initialization Oracle.initialize is called with a value for time 0

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Lack of Access Control in collectProtocolFees() Allows Unauthorized Fee Collection

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-138
- **Submitter:** rabTAI
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/138
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-138.md

## Brief Summary

Lack of Access Control in collectProtocolFees() Allows Unauthorized Fee Collection

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Potential for Overflow in Unchecked Math Operations

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-139
- **Submitter:** rabTAI
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/139
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-139.md

## Brief Summary

Potential for Overflow in Unchecked Math Operations

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Reentrancy Guard in mint() Function Can Lead to Double Minting and Fund Loss

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-140
- **Submitter:** rabTAI
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/140
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-140.md

## Brief Summary

Missing Reentrancy Guard in mint() Function Can Lead to Double Minting and Fund Loss

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Return Value Check for ERC20 transfer and transferFrom Operations

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-141
- **Submitter:** rabTAI
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/141
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-141.md

## Brief Summary

Missing Return Value Check for ERC20 transfer and transferFrom Operations

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Exchangers can get more benefit by dividing the tokens they want to exchange into smaller amounts and swapping them all at once

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-272
- **Submitter:** robertauditor
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/272
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-272.md

## Brief Summary

Exchangers can get more benefit by dividing the tokens they want to exchange into smaller amounts and swapping them all at once

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_10_group

# Owner can receive bonus tokens by burning tokens first before mint

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-273
- **Submitter:** robertauditor
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/273
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-273.md

## Brief Summary

Owner can receive bonus tokens by burning tokens first before mint

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Lack of period validation in `snapshotCumulativesInside` function allows invalid comparisons in `Oracle.sol`

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-274
- **Submitter:** safie
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/274
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-274.md

## Brief Summary

The `snapshotCumulativesInside` function in the provided code lacks proper validation to ensure that snapshots being compared are taken within a valid period where a position existed. This omission could allow invalid comparisons of cumulative tick and liquidity data, resulting in incorrect liquidity calculations and potential data integrity issues within the pool. Exploiting this vulnerability can lead to an inaccurate understanding of pool liquidity, which may be leveraged for malicious arbitrage or exploitation of liquidity providers. Description The `snapshotCumulativesInside` function is designed to return a snapshot of the tick cumulative, seconds per liquidity, and seconds inside a s...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Gas optimization and execution order vulnerability in `FeeCollector.sol`

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-275
- **Submitter:** safie
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/275
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-275.md

## Brief Summary

The vulnerability affects the efficiency of the contract, increasing the gas cost of the `collectProtocolFees` function. Additionally, it allows for fee collection even when certain conditions, such as gauge inactivity or the token being non-live, should prevent it. The function could perform unnecessary fee collection, costing users or protocols additional gas. Fees may be sent to the treasury or fee distributor without proper checks, leading to incorrect distributions.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# No calculation for `feeProtocol` on `swap` function of `RamesesV3Pool.sol`

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-278
- **Submitter:** sakibcy
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/278
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-278.md

## Brief Summary

No calculation for `feeProtocol` on `swap` function of `RamesesV3Pool.sol`

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Anyone can call the functions which are `restricted` on `RamesesV3Factory.sol`

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-279
- **Submitter:** sakibcy
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/279
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-279.md

## Brief Summary

Anyone can call the functions which are `restricted` on `RamesesV3Factory.sol`

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Accumulated fees could become 0 because of unchecked wrapping

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-99
- **Submitter:** typicalHuman
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/99
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-99.md

## Brief Summary

Accumulated fees could become 0 because of unchecked wrapping

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# The `_updatePosition` in `Position.sol` has no restrictions on who can call it.

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-280
- **Submitter:** uuzall
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/280
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-280.md

## Brief Summary

The `_updatePosition` in `Position.sol` has no restrictions on who can call it.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Possible race condition risk on the [`FeeCollector::initialize()`] due to lack of access control allowing a malicious actor calling this function upon deployment

- **Contest:** Ramses Exchange
- **Slug:** 2024-10-ramses-exchange
- **Submission:** V-104
- **Submitter:** willycode20
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-10-ramses-exchange-validation/issues/104
- **Source snapshot:** competitions/2024-10-ramses-exchange/submissions/raw/V-104.md

## Brief Summary

Possible race condition risk on the [`FeeCollector::initialize()`] due to lack of access control allowing a malicious actor calling this function upon deployment

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary
