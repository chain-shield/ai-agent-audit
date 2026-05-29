# Accepted H/M Findings: Reserve Core

# [M-01] RToken can manipulate distribution to avoid paying DAO fees

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-reserve-core
- **Source snapshot:** competitions/2024-07-reserve-core/final_report.html

Submitted by RadiantLabs Revenue produced by RTokens is sold for both RSR and RTokens according to a distribution defined in the Distributor. The BackingManager splits the collateral tokens to be sold proportionately to the RSR/RToken distribution ratio and sends them to the rsrTrader and rTokenTrader. When trades settle, the obtained RSR or RTokens are sent to the Distributor, which distributes them no longer according to the RSR/RToken ratio but to the different destinations for the specific token. The sum of all destinations for each token is used to derive the ratio.

The DAO fee is added to the RSR share of the initial split and paid when RSR is distributed.

However, the current implementation allows governance to manipulate the distribution settings without much effort in a way that can significantly reduce the amount of DAO fees paid.

This can be achieved through a combination of two different root causes:

an RSR destination can be added that prevents rewards from being immediately distributed the RSR/RToken ratio is calculated twice: once in the BackingManager, and once in the Distributor, and it is can be modified between the two Essentially, the distribution can be set in a way that temporarily accumulates RSR revenue in in the rsrTrader according to one RSR/RToken ratio, and then later redistributed with a different ratio.

## Impact

RTokens can avoid paying most of the DAO fee

## Recommended Mitigation Steps

Disallowing RSR as a distribution token prevents this to a large extent.

cccz (judge) commented:

Will try to get the sponsor’s opinion, and will set it to valid before the sponsor responds.

It’s about Malicious Governance, though it actually compromises the DAO, so it’s probably a privilege escalation.

akshatmittal (Reserve) disputed and commented:

This is a known issue, changing Distributions in a specific way can change what is actually paid out to veRSR. See publicly known issues as well as the Trust report specifically.

tbrent (Reserve) commented:

The way I’m seeing it: a new finding (RSR self-entries in the distributor table cause distribution to revert) raises the previously known issue TRST-L-2 in severity from Low to Medium, due to what is effectively privilege escalation by avoiding paying the DAO fee.

Reserve mitigated Status:

Mitigation confirmed. Full details in reports from Bauchibred, ether_sky and RadiantLabs.

# [M-02] Broken assumptions can lead to the inability to seize RSR

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-reserve-core
- **Source snapshot:** competitions/2024-07-reserve-core/final_report.html

Submitted by krikolkk The seizeRSR function takes RSR from the staking contract when BackingManager wants to sell RSR, but it does not have enough. In this case, the stakers can lose a portion of their stake in order to keep the system healthy. However, an issue arises from broken assumptions about stakeRSR and totalStakes, which will make the contract unable to seize due to revert.

Impact and Likelihood The likelihood of this issue is LOW. The impact seems somewhere between HIGH and MEDIUM since a necessary rebalance action can be DoSed until it is noticed and an action is taken. Considering that the issue is possible due to a broken invariant, the severity should be judged as MEDIUM.

Recommendation One way to fix the issue would be to enforce the invariant if totalStakes == 0, then stakeRSR == 0. This could be done by assuring that amount is not 0 in _mint.

function _mint(address account, uint256 amount) internal virtual { _notZero(account); + _notZero(amount); assert(totalStakes + amount < type(uint224).max); stakes[era][account] += amount; totalStakes += amount; emit Transfer(address(0), account, amount); _afterTokenTransfer(address(0), account, amount); } Another mitigation could be to update the seizeRSR function to update the stakeRate only if both stakeRSR and totalStakes are non-zero or update the stakeRate to FIX_ONE if either of these two is zero.

tbrent (Reserve) commented:

This is a plausible issue. We would like to request

# [M-03] The default Governor Anastasius is unable to call resetStakes

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-reserve-core
- **Source snapshot:** competitions/2024-07-reserve-core/final_report.html

resetStakes Submitted by krikolkk The StRSR contract contains a function resetStakes, which is used to reset the staking of a specific RToken system, when the stake rate becomes unsafe. When this happens, the era of the staking contract is incremented, which practically resets the StRSR token. Since this is a sensitive action, only governance can call this function.

The Reserve team provides a default and recommended Governance contract with TimelockController, which handles the proposal creation and execution. The contract also ensures that proposals created in a past era can not be queued or executed in the current era since the voting conditions can differ between eras. However due to this check, it is impossible for the Governance contract to call resetStakes, since the function would increment the era, and the following check whether the proposal was proposed in the same era would not hold and revert the transaction.

Impact and Likelihood The impact of this issue is MEDIUM, as under usual conditions, an impactful governance action would be unavailable. Since the probability of the stake rates being over/under max/min safe stake rate is low ( as per inline docs ), but the probability of the issue taking place is high, the likelihood of this issue is judged MEDIUM, hence the MEDIUM severity of this issue.

Recommendation Consider changing the order of super._execute and the startedInSameEra check in the Governance::_execute function:

function _execute( uint256 proposalId, address[] memory targets, uint256[] memory values, bytes[] memory calldatas, bytes32 descriptionHash ) internal override(Governor, GovernorTimelockControl) { + require(startedInSameEra(proposalId), "new era"); super._execute(proposalId, targets, values, calldatas, descriptionHash); - require(startedInSameEra(proposalId), "new era"); } akshatmittal (Reserve) confirmed Reserve mitigated Status:

Mitigation confirmed. Full details in reports from Bauchibred, ether_sky and RadiantLabs.

# [M-04] Dutch auctions can fail to settle if any other collateral in the basket behaves unexpectedly

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-reserve-core
- **Source snapshot:** competitions/2024-07-reserve-core/final_report.html

Submitted by RadiantLabs When a Dutch auction that originated from the backing manager receives a bid, it calls BackingManager.settleTrade() to settle the auction immediately, which attempts to chain into another rebalance() call. This chaining is implemented using a try-catch block that attempts to catch out-of-gas errors.

However, this pattern is not safe because empty error data does not always indicate an out-of-gas error. Other types of errors also return no data, such as calls to empty addresses casted as contracts and revert / require statements with no error message.

The rebalance() function interacts with multiple external assets and performs several operations that can throw empty errors:

In basketsHeldBy(), which calls _quantity(), which in turn calls coll.refPerTok() (this function should in theory never revert, but in case it interacts with the underlying ERC20, its implementation may have been upgraded to one that does).

In prepareRecollateralizationTrade(), which calls basketRange(), which also calls _quantity().

In tryTrade() if a new rebalancing trade is indeed chained, which calls approve() on the token via AllowanceLib.safeApproveFallbackToMax(). This is a direct interaction with the token and hence cannot be trusted, especially if the possibility of upgradeability is considered.

If any of these operations result in an empty error, the auction settlement will fail. This can lead to the Dutch auction being unable to settle at a fair price.

Note: we have found this finding pointing out the very same issue in a previous audit, but this report highlights a different root cause in where the error originates.

## Impact

Dutch auctions may fail to settle at the appropriate price or at all.

## Recommended Mitigation Steps

Avoid usage of this pattern to catch OOG errors in any functions that cannot revert and may interact with external contracts. Instead, in such cases always employ the _reserveGas() pattern that was iterated on to mitigate previous findings ( 1, 2, 3 ) with a similar root cause. We have found no other instances in which this applies.

akshatmittal (Reserve) disputed and commented:

This is a known issue.

The ERC20 upgrade to return empty revert data on calling any of its functions seems a little far fetched.

EV_om (warden) commented:

We do not think this should be considered a known issue either, unless it was accepted in a previous competition or pointed out in an audit.

An empty revert in one function of a collateral asset being characterized as far-fetched is a little surprising, considering findings were accepted in previous Reserve audits for the same situation but the token reverting, consuming all gas and consuming a specific amount of gas.

All those findings concerned the ability to unregister a misbehaving asset, which we found to now be guaranteed. However, we found an asset misbehaving could also have the additional impact of preventing auctions from settling for a different asset. This same impact was accepted as valid for a different root cause here.

Again, an empty revert is nothing unusual and a simple require() with no error message will produce it.

We think this scenario is very much realistic and would like to kindly ask for it to be reassessed.

cccz (judge) commented:

@akshatmittal and @tbrent - This seems to be a possible upgrade, please take a look, thanks!

Again, an empty revert is nothing unusual and a simple require() with no error message will produce it.

akshatmittal (Reserve) commented:

Looking back at this again @cccz.

The first two statements which hinge on refPerTok reverting are not valid since we require refPerTok to not revert. If a collateral plugin does revert on it, it must be fixed and replaced. The third example however, the approve one, is where I can see the token revert causing issues.

I currently can not see any sane ERC20 reverting on an approve case with no message, however you may have better examples than I do. I still consider it highly unlikely, although if you do have examples to share I’ll consider them.

And honestly, I currently do not see how to do better. For a little more context on that, we want settle to start a new auction, which is why that revert exists there, and we can’t use the _reserveGas pattern here since the gas cost for rebalance is unbound.

RTokens are designed to be governance focused, and we already have the requirement for Governance to only include collaterals they absolutely trust (which is why you’d see all RTokens today use blue chip assets only ).

If you absolutely must consider it valid, I’d probably bring it down to Low/QA given the requirements here, but also looking for your thoughts.

cccz (judge) commented:

@EV_om - If there’s no example, I’ll invalidate it because the assumption isn’t valid.

EV_om (warden) commented:

@cccz - USDT and BNB throw empty errors on reverts within approve(), for instance.

These are the two largest market cap ERC-20 tokens in existence - again, this is not some theoretical esoteric behaviour but a realistic scenario.

There may not be a better approach if the gas cost of rebalance() is unbounded as you say @akshatmittal. But lack of an immediate mitigation does not invalidate the issue/make it QA.

akshatmittal (Reserve) commented:

@EV_om - Both of the examples you have mentioned throw on zero, which is a case handled within the code. (Also just saying here, BNB isn’t technically a supported token for other reasons) this is not some theoretical esoteric behaviour but a realistic scenario.

Believe me, I’m not trying to say so. I’m really trying to find a realistic case where an upgrade on the token makes it regress in a basic ERC20 function.

And yeah, I’m also not saying not having a mitigation invalidates the issue, but rather that the protocol has ways of dealing with such specific things like wrapping the tokens, etc. We already wrap tokens that we don’t like behaviours of, or tokens that have weird behaviours.

(Talking to cccz to accept this, just trying to get a better idea) cccz (judge) commented:

Although the likelihood is low, the assumed token satisfies acceptable upgradability, will upgrade it to Medium.

# [M-05] Users can dodge losses due to StRSR era changes with instant operations

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-reserve-core
- **Source snapshot:** competitions/2024-07-reserve-core/final_report.html

Submitted by RadiantLabs The StRSR contract implements an era-based wrapping of the stakeRate and draftRate exchange rates whenever these pass the maximum accepted value of MAX_STAKE_RATE and MAX_DRAFT_RATE; the effect of this action is that StRSR token holders see their balance reset ( stakeRate reset) and/or all StRSR vesting withdraws are forgotten and the relative funds lost ( draftRate reset).

From the code below, we can see that these two rates can be reset independently; File:

StRSR.

sol 424:

function seizeRSR ( uint256 rsrAmount ) external { --- 440:

// Remove RSR from stakeRSR 441:

uint256 stakeRSRToTake = ( stakeRSR * rsrAmount + ( rsrBalance - 1 )) / rsrBalance; 442:

stakeRSR -= stakeRSRToTake; 443:

seizedRSR = stakeRSRToTake; 444:

445:

// update stakeRate, possibly beginning a new stake era 446:

if ( stakeRSR != 0 ) { 447:

// Downcast is safe: totalStakes is 1e38 at most so expression maximum value is 1e56 448:

stakeRate = uint192 (( FIX_ONE_256 * totalStakes + ( stakeRSR - 1 )) / stakeRSR ); 449: } 450:

if ( stakeRSR == 0 || stakeRate > MAX_STAKE_RATE ) { 451:

seizedRSR += stakeRSR; 452:

beginEra (); 453: } 454:

455:

// Remove RSR from draftRSR 456:

uint256 draftRSRToTake = ( draftRSR * rsrAmount + ( rsrBalance - 1 )) / rsrBalance; 457:

draftRSR -= draftRSRToTake; 458:

seizedRSR += draftRSRToTake; 459:

460:

// update draftRate, possibly beginning a new draft era 461:

if ( draftRSR != 0 ) { 462:

// Downcast is safe: totalDrafts is 1e38 at most so expression maximum value is 1e56 463:

draftRate = uint192 (( FIX_ONE_256 * totalDrafts + ( draftRSR - 1 )) / draftRSR ); 464: } 465:

466:

if ( draftRSR == 0 || draftRate > MAX_DRAFT_RATE ) { 467:

seizedRSR += draftRSR; 468:

beginDraftEra (); 469: } … and we’d then argue that they will most likely happen at different times, at the very least because rewards contribute to the evolution of stakeRate only.

In the event only one of the two rates reaches the cap and undergoes a reset, stakers can dodge the bullet of collectivized losses, by:

sandwiching the stakeRate wrapping with unstake and cancelUnstake calls sandwiching the draftRate wrapping with cancelUnstake and unstake calls

## Impact

Users can avoid the collectivization of rate wrapping losses, and artificially get a share of any RSR left in the StRSR contract.

This latter quantity may be considerable, because while unlikely, it’s not impossible that the rate wrapping happens when little RSR is seized from an otherwise large RSR balance in the StRSR contract.

## Recommended Mitigation Steps

Consider wrapping both rates when at least one reaches the maximum.

tbrent (Reserve) acknowledged and commented:

The wrapping occurs at the MAX_STAKE_RATE or MAX_DRAFT_RATE, ie 1e27.

resetStakes() is available to governance above MAX_SAFE_STAKE_RATE, ie 1e24.

StRSR is unstable between these bounds, but the risk of it occurring is assumed low enough to be not worth further mitigating. See:

- https://github.com/reserve-protocol/protocol/blob/72fc1f6e41da01e733c0a7e96cdb8ebb45bf1065/contracts/p1/StRSR.sol#L490
/// The stake rate is unsafe when it is either too high or too low.

/// There is the possibility of the rate reaching the borderline of being unsafe, /// where users won't stake in fear that a reset might be executed.

/// A user may also grief this situation by staking enough RSR to vote against any reset.

/// This standoff will continue until enough RSR is staked and a reset is executed.

/// There is currently no good and easy way to mitigate the possibility of this situation, /// and the risk of it occurring is low enough that it is not worth the effort to mitigate.

The reported issue requires the same set of assumptions to get into this state, and then additional assumptions about the size of the next seizeRSR(). However, the impact trades off directly against the likelihood: the further the rate is away from the 1e27 bounds, the less likely this case is to occur. The closer to 1e27, the more clear it is to governance that they need to resetStakes(). And there are 3 orders of magnitude between 1e27 and 1e24 for this dynamic to play out.

We would like to acknowledge the issue but dispute severity to Low, given it is a subset of a case already assumed low enough probability to not be worth mitigating.

cccz (judge) decreased severity to Low 3docSec (warden) commented:

Hi @cccz and @tbrent - the require statement in resetStakes makes this function a viable option to mitigate only the issue in case of stakeRate being close to wrapping, but does not help with the other scenario presented by this issue - that is when draftRate wraps.

function resetStakes () external { _requireGovernanceOnly (); require ( stakeRate <= MIN_SAFE_STAKE_RATE || stakeRate >= MAX_SAFE_STAKE_RATE, "rate still safe" ); beginEra (); beginDraftEra (); } Because draftRate normally goes only up with seizeRSR, and stakeRate instead goes up with seizeRSR but also goes down with _payoutRewards, it is still possible that draftRate grows close to MAX_STAKE_RATE while stakeRate is still below MAX_SAFE_STAKE_RATE. This scenario would prevent governance intervention, entering a situation where a draftRate wrapping alone is inevitable.

I would therefore ask you to reconsider the Medium severity, and potentially consider another mitigation option in allowing calls to resetStakes also when draftRate grows above MAX_SAFE_STAKE_RATE.

tbrent (Reserve) commented:

That is a good point:

resetStakes() cannot be used when it is the draft rate that is out of bounds. I think there is a change to the code to make there of some variety.

Still, in order for the below to be true:

draftRate grows close to MAX_STAKE_RATE while stakeRate is still below MAX_SAFE_STAKE_RATE.

there must be at least 1000x historical appreciation of StRSR overall. This is an additional assumption beyond the initial assumptions of: (i) draftRate grows close to MAX_STAKE_RATE (requires consecutive nearly full-but-not-full seizures); (ii) can frontrun seizeRSR().

cccz (judge) increased severity to Medium and commented:

Based on the above conversation, users can frontrun resetStakes to avoid losses or get benefits. If this is correct, I would consider this issue to be low likelihood + high impact = Medium severity.

Reserve mitigated Status:

Mitigation confirmed. Full details in reports from ether_sky and Bauchibred.

# [M-06] The time available for a canceled withdrawal should not impact future unstaking processes

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-reserve-core
- **Source snapshot:** competitions/2024-07-reserve-core/final_report.html

Submitted by ether_sky, also found by Bauchibred Lines of Code

- https://github.com/code-423n4/2024-07-reserve/blob/3f133997e186465f4904553b0f8e86ecb7bbacbf/contracts/p1/StRSR.sol#L279
- https://github.com/code-423n4/2024-07-reserve/blob/3f133997e186465f4904553b0f8e86ecb7bbacbf/contracts/p1/StRSR.sol#L658
- https://github.com/code-423n4/2024-07-reserve/blob/3f133997e186465f4904553b0f8e86ecb7bbacbf/contracts/p1/StRSR.sol#L368

## Impact

When stakers want to unstake their StRSR, they cannot withdraw RSR immediately.

Instead, each withdrawal enters a queue and will become available after the unstaking delay period.

This queue operates on a FIFO basis, meaning earlier withdrawals are processed before later ones.

Stakers can also cancel their withdrawals, and the RSR will be restaked immediately.

However, even canceled withdrawals can still impact future withdrawal requests.

## Recommended Mitigation Steps

function pushDraft(address account, uint256 rsrAmount) internal returns (uint256 index, uint64 availableAt) { CumulativeDraft[] storage queue = draftQueues[draftEra][account]; index = queue.length; uint192 oldDrafts = index != 0 ? queue[index - 1].drafts: 0; - uint64 lastAvailableAt = index != 0 ? queue[index - 1].availableAt: 0; + uint64 lastAvailableAt = index != 0 && firstRemainingDraft[draftEra][account] < index ? queue[index - 1].availableAt: 0; availableAt = uint64(block.timestamp) + unstakingDelay; if (lastAvailableAt > availableAt) { availableAt = lastAvailableAt; } queue.push(CumulativeDraft(uint176(oldDrafts + draftAmount), availableAt)); } akshatmittal (Reserve) commented:

Issue confirmed.

Although, want to downgrade to Low given the requirements, actions and impact.

cccz (judge) decreased severity to Low and commented:

Considering the low likelihood (owner reduces unstakingDelay, user cancels withdrawals before that) and the earlier the cancellation, the lower the impact, it will be downgraded to Low.

ether_sky (warden) commented:

Hi @cccz - thanks for your review.

Let me clarify the

## impact

and likelihood again.

As seen in the code below, the minimum unstaking delay is 2 minutes, and the maximum is 1 year. There’s a significant difference between these two values.

uint48 private constant MIN_UNSTAKING_DELAY = 60 * 2; // {s} 2 minutes uint48 private constant MAX_UNSTAKING_DELAY = 60 * 60 * 24 * 365; // {s} 1 year Suppose the current unstaking delay is set to the 1 year.

All stakers are operating under this delay now.

For various reasons, the admin decides to reduce the unstaking delay to a shorter period, say 1 month.

Stakers will realize they can withdraw their funds after 1 month instead of 1 year.

They can cancel their existing withdrawal requests and attempt to unstake again.

However, the original 1-year delay still applies to all stakers, meaning they cannot withdraw their funds after just 1 month —even if they cancel their old withdrawals.

Their funds remain locked for 1 year.

Considering the low likelihood (owner reduces unstakingDelay, user cancels withdrawals before that) and the earlier the cancellation, the lower the impact, it will be downgraded to Low Regarding the likelihood, users do not need to cancel their withdrawals before the owner changes the unstaking delay. Even if they cancel their withdrawals anytime after the delay is changed, the old delay still impacts all stakers.

As for the

## impact, this situation results in users’ funds being locked and creates a DoS issue.

Given these points, I believe the

## impact

and likelihood are at least medium. I would appreciate it if you could reconsider this issue.

cccz (judge) increased severity to Medium and commented:

When the owner reduces unstakingDelay, the user will not be able to apply the latest unstakingDelay even if they cancel the previous withdrawal.

If this is correct, I tend to raise it to Medium.

Reserve mitigated Status:

Mitigation confirmed. Full details in reports from Bauchibred, ether_sky and RadiantLabs.

# [M-07] The tradeEnd in BackingManager isn’t updating correctly

- **Contest:** Reserve Core
- **Slug:** 2024-07-reserve-core
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-reserve-core
- **Source snapshot:** competitions/2024-07-reserve-core/final_report.html

tradeEnd in BackingManager isn’t updating correctly Submitted by ether_sky, also found by RadiantLabs and stuart_the_minion Lines of code

- https://github.com/code-423n4/2024-07-reserve/blob/3f133997e186465f4904553b0f8e86ecb7bbacbf/contracts/p1/BackingManager.sol#L114
- https://github.com/code-423n4/2024-07-reserve/blob/3f133997e186465f4904553b0f8e86ecb7bbacbf/contracts/p1/BackingManager.sol#L166

## Impact

In the BackingManager, we use the tradeEnd value for each type of trade to prevent next auction from occurring within the same block. We can find a comment in the code explaining this in line 114.

function rebalance(TradeKind kind) external nonReentrant { // DoS prevention:

114: // unless caller is self, require that the next auction is not in same block require( _msgSender() == address(this) || tradeEnd[kind] < block.timestamp, "already rebalancing" ); This approach works correctly for Batch auctions. However, with Dutch auctions, the tradeEnd value can inadvertently block the next auction from starting for a certain period.

## Recommended Mitigation Steps

function settleTrade(IERC20 sell) public override(ITrading, TradingP1) returns (ITrade trade) { delete tokensOut[sell]; trade = super.settleTrade(sell); // nonReentrant if (_msgSender() == address(trade)) { try this.rebalance(trade.KIND()) {} catch (bytes memory errData) { if (errData.length == 0) revert(); // solhint-disable-line reason-string } + tradeEnd[kind] = uint48(block.timestamp); } akshatmittal (Reserve) confirmed, but disagreed with severity and commented:

We’d like to bump this down to Low.

This is an issue, although does not impact protocol availability. The protocol can still function as expected using the other trading methods.

cccz (judge) commented:

After reconsidering the issue, I think this should be valid Medium. It indeed affects the availability of the protocol, which meets the Medium severity.

tbrent (Reserve) commented:

The issue can only impact DutchTrade because it is the only KIND that can have auctions that end before the endTime.

I only see the availability of the protocol being impacted if the GnosisTrade’ reportViolation() triggers, which would ultimately be the real cause of the loss of availability. GnosisTrade is supposed to be the fallback mechanism for trading.

Possibly relevant: the notions of disabling in the Broker are very different for dutch trade and batch trade:

in dutch trades it is intended to detect if the protocol is trading off bad pricing data, since a dutch trade necessarily involves an assumption about the highest possible price. This is possible to trigger intentionally by burning $.

in batch trades it is intended to detect if EasyAuction goes beyond the worst-case min buy amounts. Similar to if a Uniswap swap violated slippage constraints, it should not happen. This should not be possible to trigger intentionally by burning $.

ether_sky (warden) commented:

Thanks for your reveiw.

There are only two types of trades.

Because of this issue, Dutch auctions cannot occur for up to one week.

In some cases, Dutch auctions could be more efficient than Batch trades, but we are required to use Batch trades instead.

Additionally, as the sponsor described, Batch trades can be paused due to reportViolation().

In such cases, this issue could have a significant impact.

Therefore, I believe this issue deserves a Medium severity based on its impact and likelihood.

cccz (judge) commented:

Yes, as warden said, Dutch auctions being unavailable for a period of time is a concern.

akshatmittal (Reserve) commented:

@cccz - Let me clarify what @tbrent is trying to say.

Batch Auctions are always available, and the whole reason they exist is for scenarios like this. The key about reportViolation for Gnosis Trade is that it checks for an EasyAuction protocol invariant, and nothing else. That is why it’s the “backup” trading method.

Dutch Auctions and Batch Auctions have different characteristics from a what-happens-when perspective, but similar characteristics from a competition and pricing perspective. Yes, Dutch Auctions make it more efficient for the participants, which is why it exists, but it’s not something that the protocol entirely depends on.

Unavailability of Dutch Auction does not mean the protocol is at risk or is “unavailable”, which is why we disable Dutch Auctions in many more conditions than Batch Auctions. Keep in mind that early versions of the protocol did not even have Dutch Trade to begin with.

That said, we’ll leave it to you to decide here, we are internally considering it a low severity issue.

ether_sky (warden) commented:

Hi @akshatmittal - thanks for your comment.

I understood all your points. However, how can auditors know whether the Dutch Auctions is not important? The only thing which we could find is that there are only 2 types of auctions and one of them can be unavailable for some periods and each auctions can be paused due to reportViolation functionality.

Anyway, I will respect judge’s decision.

Thanks again.

Reserve mitigated Status:

Mitigation confirmed. Full details in reports from RadiantLabs, ether_sky and Bauchibred.
