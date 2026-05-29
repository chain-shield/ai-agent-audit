# Accepted H/M Findings: Neobase Invitational

# [H-01] If a gauge that a user has voted for gets removed, their voting power allocated for that gauge will be lost

- **Contest:** Neobase Invitational
- **Slug:** 2024-03-neobase-invitational
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-03-neobase-invitational
- **Source snapshot:** competitions/2024-03-neobase-invitational/final_report.html

Submitted by Arabadzhiev, also found by said

- https://github.com/code-423n4/2024-03-neobase/blob/d6e6127e6763b93c23ee95cdf7622fe950d9ed30/src/GaugeController.sol#L224-L229
- https://github.com/code-423n4/2024-03-neobase/blob/d6e6127e6763b93c23ee95cdf7622fe950d9ed30/src/GaugeController.sol#L402

## Impact

When a gauge that an user has voted for gets removed by the governance, their voting power allocated for that gauge will be lost forever.

## Recommended Mitigation Steps

Remove the additional require statement that checks whether the gauge type for the _gauge_addr is different from 0, in order to allow users to remove their votes from removed gauges:

function vote_for_gauge_weights(address _gauge_addr, uint256 _user_weight) external { require(_user_weight >= 0 && _user_weight <= 10_000, "Invalid user weight"); require(_user_weight == 0 || gauge_types_[_gauge_addr] != 0, "Can only vote 0 on non-gauges"); // We allow withdrawing voting power from invalid (removed) gauges VotingEscrow ve = votingEscrow; (, /*int128 bias*/ int128 slope_, /*uint256 ts*/ ) = ve.getLastUserPoint(msg.sender); require(slope_ >= 0, "Invalid slope"); uint256 slope = uint256(uint128(slope_)); uint256 lock_end = ve.lockEnd(msg.sender); uint256 next_time = ((block.timestamp + WEEK) / WEEK) * WEEK; require(lock_end > next_time, "Lock expires too soon"); int128 gauge_type = gauge_types_[_gauge_addr] - 1;

- require(gauge_type >= 0, "Gauge not added");...

}

## Assessed type

Invalid Validation zjesko (Neobase) confirmed 0xTheC0der (judge) commented:

Voting power is considered an asset and can be lost in this scenario without malicious governance intent or mistake.

Medium Risk Findings (7)

# [M-01] In case a gauge weight reduction is performed via GaugeController::change_gauge_weight , it is possible that all functions will stop functioning permanently for that gauge

- **Contest:** Neobase Invitational
- **Slug:** 2024-03-neobase-invitational
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-neobase-invitational
- **Source snapshot:** competitions/2024-03-neobase-invitational/final_report.html

GaugeController::change_gauge_weight, it is possible that all functions will stop functioning permanently for that gauge Submitted by Arabadzhiev If the weight of a given gauge has been manually changed using the GaugeController::change_gauge_weight function, all functions related to that gauge might become permanently DoSed.

## Recommended Mitigation Steps

Whenever pt.slope is less than d.slope, set the value of pt.slope to 0:

if (pt.bias > d_bias) { pt.bias -= d_bias; uint256 d_slope = changes_weight[_gauge_addr][t]; - pt.slope -= d_slope; + if(pt.slope >= d.slope) { + pt.slope -= d_slope; + } else { + pt.slope = 0; + } } else { pt.bias = 0; pt.slope = 0; }

## Assessed type

Under/Overflow zjesko (Neobase) confirmed 0xTheC0der (judge) decreased severity to Medium and commented:

Assets not at direct risk, but the function of the protocol or its availability could be impacted.

change_gauge_weight is only callable by the governance, but no admin mistake is required for this to happen.

Arabadzhiev (warden) commented:

This issue will actually lead to an absolutely certain loss of funds in the event where a gauge weight reduction is performed via GaugeController::change_gauge_weight. This is because inside of the LendingLedger::update_market function,which is called inside of both LendingLedger::claim and LendingLedger::sync_ledger there is a call to GaugeController::gauge_relative_weight_write:

market.

accCantoPerShare += uint128 ( blockDelta * cantoPerBlock [ epoch ] * gaugeController.

gauge_relative_weight_write ( _market, epochTime )) / marketSupply ); What this means is that since GaugeController::gauge_relative_weight_write calls GaugeController::_get_weight internally, whenever a gauge weight reduction is performed, all of those functions will become permanently DoSed; making it impossible for users to claim their rewards for a given market/gauge from the LendingLedger. Also, if the gauge that the weight reduction is performed on uses a LiquidityGauge, the users that have deposited into that liquidity gauge won’t be able to withdraw their underlying assets from it, since in the LiquidityGauge::_afterTokenTransfer hook there are calls to LendingLedger::sync_ledger.

Because of that, I believe that this issue deserves to be judged as one of a High severity, rather than a Medium.

0xTheC0der (judge) commented:

@Arabadzhiev - From a judging perspective my reasoning is as follows:

The original report, which is already quite minimalist, states:

…, all functions related to that gauge might become permanently DoSed.

Furthermore, we have a verdict about additional warden output during PJQA:

No new information should be introduced and considered in PJQA. Elaborations of the already introduced information can be considered (e.g. tweaking a POC), from either the Judge or the Warden, but they will only count towards the validity of the issue, not its quality score.

Although the underlying issue might qualify for High severity in case more elaboration of impacts and/or a PoC was provided in the original report, it seems to be fair and most in line with our present ruling to maintain Medium severity.

Arabadzhiev (warden) commented:

I have one final note to make. In this report from the previous audit of the codebase that I have linked to in my report, the following is stated in its impact section:

… the impact is that the entire gauge is useless, voting powers are permanently locked there and its weight is impossible to change, so the impact is high.

Given that this is actually a part of the original report and that issue #18 was judged as a High under pretty much the same reasoning, doesn’t this issue also fall within the High severity category because of that?

0xTheC0der (judge) commented:

I treat it as a supporting reference but not as a “replacement” for your report.

# [M-02] GaugeController::remove_gauge will always revert whenever the gauge it’s being called or has any weight attached to it

- **Contest:** Neobase Invitational
- **Slug:** 2024-03-neobase-invitational
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-neobase-invitational
- **Source snapshot:** competitions/2024-03-neobase-invitational/final_report.html

GaugeController::remove_gauge will always revert whenever the gauge it’s being called or has any weight attached to it Submitted by Arabadzhiev, also found by said GaugeController::remove_gauge will always revert when it is called for gauges with weight that is != 0.

## Recommended Mitigation Steps

Call _remove_gauge_weight prior to erasing the gauge type of the gauge:

function remove_gauge(address _gauge) external onlyGovernance { require(gauge_types_[_gauge] != 0, "Invalid gauge address"); - gauge_types_[_gauge] = 0; - _remove_gauge_weight(_gauge); + _remove_gauge_weight(_gauge); + gauge_types_[_gauge] = 0; emit GaugeRemoved(_gauge); }

## Assessed type

Under/Overflow zjesko (Neobase) confirmed

# [M-03] Issue from previous audit still present: Gauge can have bigger weight than was intended by protocol

- **Contest:** Neobase Invitational
- **Slug:** 2024-03-neobase-invitational
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-neobase-invitational
- **Source snapshot:** competitions/2024-03-neobase-invitational/final_report.html

Submitted by rvierdiiev M-01 from previous audit is still present. In this issue, the sponsor said they fixed it. I tried to check how it was fixed, but the link doesn’t work for me.

But change_gauge_weight function still exists which makes it possible to reproduce this. Also remove_gauge_weight function is present, that allows to completely remove gauge and i think that fix was to remove change_gauge_weight function.

## Impact

Gauge can have bigger weight than was intended by protocol.

Tools Used VsCode Recommendation Remove change_gauge_weight function.

## Assessed type

Error zjesko (Neobase) confirmed

# [M-04] Improper adjustment of Lending Ledger configuration

- **Contest:** Neobase Invitational
- **Slug:** 2024-03-neobase-invitational
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-neobase-invitational
- **Source snapshot:** competitions/2024-03-neobase-invitational/final_report.html

Submitted by 0xsomeone, also found by carrotsmuggler

- https://github.com/code-423n4/2024-03-neobase/blob/main/src/LendingLedger.sol#L136-L149
- https://github.com/code-423n4/2024-03-neobase/blob/main/src/LendingLedger.sol#L173-L185
Description The LendingLedger utilizes an approximation system (to evaluate the time that has elapsed between two block numbers) which contains adjustable values. Additionally, it permits the governance to adjust the cantoPerBlock value of multiple epochs at any time.

The way these values are adjusted is insecure and thus can cause them to retroactively apply to markets that have not yet been updated.

In the case of the LendingLedger::setBlockTimeParameters, a notice exists within the LendingLedger::update_market function that warns that:

If this ever drifts significantly, the average block time and/or reference block time & number can be updated. However, update_market needs to be called for all markets beforehand.

I do not consider the warning sufficient, as the LendingLedger::setRewards function illustrates that the problem is not understood accurately.

Any market that was not updated on the exact same block that either rewards or the block time parameters are adjusted will have these adjustments retroactively applied, leading to over-estimations or under-estimations of the rewards that should be attributed to the market.

## Impact

Reward measurements for markets that were not updated in the exact same block that rewards and/or block-time parameters were re-configured will result in over- or under-estimations, depending on the direction of these configurations.

Severity Rationalization Administrator mistakes usually fall under QA/Analysis reports; however, in this circumstance, the mistake is not based on input but rather on the state of the contract. Additionally, there are cases whereby the change cannot be performed securely (i.e. if the number of markets introduced would reach the gas limit if all are attempted to be updated at the same block).

Based on the above, I believe this constitutes a valid operational vulnerability that stems from an improperly coded configuration procedure for both rewards and block time parameters.

While there is a warning for the LendingLedger::setBlockTimeParameters function, it is located in an entirely different function and is insufficient, in my opinion. Even if considered sufficient, there is absolutely no warning or indication that the same restriction applies for the LendingLedger::setRewards function.

This submission may be split into two distinct ones if the reward-related and block-time-related impacts are considered distinct; however, I grouped them under the same submission as they pertain to the same operation (update of all markets) being absent albeit from two different code segments.

## Recommended Mitigation Steps

I advise all markets added to a LendingLedger to be tracked and iterated whenever either of the submission’s referenced functions is executed, ensuring that all markets are indeed updated in the same block.

As a gas-optimal alternative, the total markets of the LendingLedger could be tracked as a number. The aforementioned adjustment functions could then accept an input array that would contain all markets, permitting a for loop to iterate them, ensure they are distinct (i.e. in strictly ascending order), and ensure that the total number of input markets is equal to the total number of registered markets.

Alternatively, and as a solution solely for the LendingLedger::setRewards function, the epochs that are mutated could be restricted to be future ones and thus never result in a retroactive application.

## Assessed type

Governance 0xTheC0der (judge) commented:

Referring to README:

Publicly Known Issues Mistakes by Governance:

We assume that all calls that are performed by the governance address are performed with the correct parameters.

However, it’s not just about correct call parameters but also about correct call timing ( reward-related and block-time-related impacts ); therefore, leaving this for sponsor review.

The current duplicate #19 is more affected by the README; will reconsider during judging.

OpenCoreCH (Neobase) confirmed 0xTheC0der (judge) commented:

Administrator mistakes usually fall under QA/Analysis reports; however, in this circumstance, the mistake is not based on input but rather on the state of the contract.

I agree with this assessment in the report.

# [M-05] Truncation exploitation of partial transfer system

- **Contest:** Neobase Invitational
- **Slug:** 2024-03-neobase-invitational
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-neobase-invitational
- **Source snapshot:** competitions/2024-03-neobase-invitational/final_report.html

Submitted by 0xsomeone The LendingLedger::sync_ledger function is meant to permit partial transfers of each user’s positions by updating the amount, rewardDebt, and secRewardDebt variables.

In the current system, the partial transfers performed are insecure as a non-zero _delta can be transferred with zero-value rewardDebt and secRewardDebt increments.

Specifically, any non-zero _delta value that would result in (_delta * market.accCanotPerShare)/ 1e18 to result in a truncation can be transferred multiple times to exploit the truncation.

As the LendingLedger::claim function will utilize the cumulative user.amount value, multiple _delta transfers can result in a non-truncated (user.amount * market.accCanotPerShare) / 1e18 value that would result in immediate profit as the user would be able to instantly claim the truncated amounts.

## Impact

A user is able to claim rewards from the LendingLedger without any time elapsing and without being eligible for them by taking advantage of debt truncations in the LendingLedger::sync_ledger function.

This exploit can be repeated infinitely to compound the rewards extracted via this mechanism.

## Recommended Mitigation Steps

For the mechanism to behave correctly, it should penalize rewards rather than penalize debt when truncation occurs. To achieve this, the subtraction execution path of LendingLedger::sync_ledger should continue subtracting the rounded-down amount from debt while the addition execution path should round the debt added upwards.

## Assessed type

Math OpenCoreCH (Neobase) confirmed and commented:

That’s true and rounding up there is probably a good idea (although it could generate underflows in claim if this is not changed as well). However, I am not fully convinced about the impact. If (uint256(_delta) * market.accCantoPerShare) is for instance 1e18 - 1, the “correct” debt value is 0.999999..., so rounded up it would be 1. We then underestimate the debt by exactly 1 token, which has a value of 10^{-18} USD in the case of cNOTE.

0xTheC0der (judge) commented:

Limited impact but high likelihood since there is no boundary on the repeatability of the attack, which can lead to further impacts (see Severity Rationalization of report.)

# [M-06] Improper parallel time system

- **Contest:** Neobase Invitational
- **Slug:** 2024-03-neobase-invitational
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-neobase-invitational
- **Source snapshot:** competitions/2024-03-neobase-invitational/final_report.html

Submitted by 0xsomeone, also found by carrotsmuggler, rvierdiiev, and said The LendingLedger system will employ a distinct time system from the VotingEscrow and GaugeController implementations whereby rewards are denoted per block rather than based on time.

From an implementation perspective, there is no real reason to implement this distinction as the LendingLedger can be simply updated to utilize a reward-per-second system instead.

To work around this discrepancy, the LendingLedger implements an imprecise time measurement for evaluating the epochTime whenever a market update occurs through LendingLedger::update_market.

In detail, the following variables are configured and utilized:

referenceBlockTime: Indicates the time from which a reference data point is drawn from.

referenceBlockNumber: Indicates the block number from which we should count the blocks that have elapsed since the reference block time.

averageBlockTime: The average block time per block expressed in milliseconds.

The vulnerability lies in the fact that an overestimation of the time (i.e. a future time) will result in a IGaugeController::gauge_relative_weight_write result of 0, effectively nullifying the full reward of the epoch which may be up to 1 week worth of rewards.

Specifically, GaugeController::_gauge_relative_weight will yield 0 in the case the total weight of the epoch is 0 ( points_total ), a case that will occur 100% if the input time of the function exceeds the current block.timestamp to an extent that it would cause it to flow into the next WEEK.

This error does not need to necessarily be a full WEEK, as a timestamp at 5 weeks - 1 will fall in week 4 which can be calculated, while a timestamp at 5 weeks will fall in week 5 which would be in the future and thus not calculate-able.

It is impossible to programmatically approximate the number of blocks in a particular time period for any blockchain; meaning, that the vulnerability will manifest itself with a high possibility as either an overestimation or underestimation of time elapsed for the number of blocks will occur in the LendingLedger.

An overestimation of time elapsed by the LendingLedger::update_market function will lead to the rewards for a particular epoch being 0 even though the gauge may have had a non-zero relative weight due to the weight not having been tracked yet for the miscalculated future timestamp.

## Impact

The non-zero rewards that should have been distributed for the latest epoch will not be processed correctly and would be considered 0.

## Recommended Mitigation Steps

I would advise the LendingLedger implementation to be updated to a time-based reward mechanism instead, ensuring that it remains in sync with the GaugeController.

## Assessed type

Math OpenCoreCH (Neobase) confirmed 0xTheC0der (judge) commented:

Core issue of this group if findings: Parallel time system and resulting rewards epoch drift/mismatch due to time estimation based on block.number and related parameters (see report).

Selected for report due to overall best elaboration of the “parallel time system” and its impacts.

# [M-07] When the unlockOverride flag is true, users can “freely” vote for gauge weights.

- **Contest:** Neobase Invitational
- **Slug:** 2024-03-neobase-invitational
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-neobase-invitational
- **Source snapshot:** competitions/2024-03-neobase-invitational/final_report.html

unlockOverride flag is true, users can “freely” vote for gauge weights.

Submitted by said, also found by Arabadzhiev

- https://github.com/code-423n4/2024-03-neobase/blob/main/src/VotingEscrow.sol#L295-L311
- https://github.com/code-423n4/2024-03-neobase/blob/main/src/VotingEscrow.sol#L315-L350

## Impact

Due to the lack of restrictions on createLock and increaseAmount when the unlockOverride flag is set to true, users can createLock / increaseAmount on VotingEscrow, vote for a gauge inside the GaugeController to increase the desired gauge’s weight, and then immediately withdraw the locked native token from VotingEscrow to boost their desired gauge’s weight for “free”.

## Recommended Mitigation Steps

Considering that unlockOverride is potentially used to finish the lock time early for all users inside VotingEscrow, consider adding a check inside createLock and increaseAmount. If unlockOverride is set to true, revert the operation.

## Assessed type

Invalid Validation zjesko (Neobase) confirmed
