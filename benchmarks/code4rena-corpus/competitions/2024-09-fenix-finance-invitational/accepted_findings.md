# Accepted H/M Findings: Fenix Finance Invitational

# [H-01] killGauge() will lead to wrong calculation of emission

- **Contest:** Fenix Finance Invitational
- **Slug:** 2024-09-fenix-finance-invitational
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-09-fenix-finance-invitational
- **Source snapshot:** competitions/2024-09-fenix-finance-invitational/final_report.html

killGauge() will lead to wrong calculation of emission Submitted by Ch_301, also found by KupiaSec

- https://github.com/code-423n4/2024-09-fenix-finance/blob/main/contracts/core/VoterUpgradeableV2.sol#L239
- https://github.com/code-423n4/2024-09-fenix-finance/blob/main/contracts/core/VoterUpgradeableV2.sol#L630-L639
Description The VoterUpgradeableV2.sol contract has killGauge() that disables the gauge to prevent it from further rewards distribution, only the address with GOVERNANCE_ROLE role can call it. the killGauge() only updates three state variables.

File:

VoterUpgradeableV2.

sol 227:

function killGauge ( address gauge_ ) external onlyRole ( _GOVERNANCE_ROLE ) {...

232:

delete gaugesState [ gauge_ ].

isAlive;...

236:

delete gaugesState [ gauge_ ].

claimable;...

240:

totalWeightsPerEpoch [ epochTimestamp ] -= weightsPerEpoch [ epochTimestamp ][ state.

pool ]; The distribute() function will distribute rewards to pools managed by the VoterUpgradeableV2.sol contract and it will call the Minter contract by triggering update_period() function before distributing rewards.

The timeline looks like this:

Epoch_x Epoch_x+1 |-----------x-------------------|-x--------------------- call `killGauge()` call `distribute()` When distribute() gets invoked in the timeline it will distribute the rewards of Epoch_x, The killed gauge has no weight in this epoch because its weight gets subtracted from totalWeightsPerEpoch[] in killGauge().

When the Minter invokes VoterUpgradeableV2.sol#notifyRewardAmount() to notify the contract of the reward amount to be distributed for Epoch_x, we can also find in the same function how the index value gets increased.

File:

VoterUpgradeableV2.

sol 382:

function notifyRewardAmount ( uint256 amount_ ) external {...

387:

uint256 weightAt = totalWeightsPerEpoch [ _epochTimestamp () - _WEEK ]; 388:

if ( weightAt > 0 ) { 389:

index += ( amount_ * 1e18 ) / weightAt; 390: } The index is updated as the reward amount divided by the total weights of Epoch_x, we know the weight of the disabled gauge is not included in totalWeightsPerEpoch[Epoch_x].

Back to _distribute():

File:

VoterUpgradeableV2.

sol 671:

function _distribute ( address gauge_ ) internal {...

677:

uint256 totalVotesWeight = weightsPerEpoch [ currentTimestamp - _WEEK ][ state.

pool ]; 678:

679:

if ( totalVotesWeight > 0 ) {...

684:

if ( state.

isAlive ) { 685:

gaugesState [ gauge_ ].

claimable += amount; 686: } else { 687:

IERC20Upgradeable ( token ).

safeTransfer ( minter, amount ); } Because killGauge() doesn’t delete the values of weightsPerEpoch[], it will send back amount of emissions back to Minter, which actually should get distributed between the existing pools.

To summarize, the index is directly related by the value of totalWeightsPerEpoch[Epoch_x], and the killGauge() is subtracted from the weightsPerEpoch of the disabled gauge. Therefore, the index didn’t include the weight of the killed gauge, but _distribute calculates its emission and sends it back to Minter.

To understand the impact, in case the total emissions for Epoch_x is 80e18 with three active gauges (with the same amount of votes), each pool will receive 26.5e18 tokens.

But in case one gauge gets killed, one scenario is the 1st gauge will receive 40e18 and the other 40e18 will get transferred back to Minter. This will leave the last gauge with 0 emissions (from here, the impact is related to how gauge.sol#.notifyRewardAmount() will handle this situation which is out of scope in this audit).

Another scenario is to send 40e18 to the two gauges but the disabled gauge gets revived in the next epoch and will be able to receive his 40e18 tokens because the gaugesState[gauge_].index is not updated (this will loop us to the above scenario again because the 40e18 tokens do not exist in the first time).

## Impact

One or more gauges will not receive their emissions.

Wrong calculation of gaugesState[gauge_].claimable.

The distribution system will be broken if the killed gauge gets revived again.

The impact depends on the order of the gauges array that passed to distribute() function.

## Recommended Mitigation Steps

One fix is to delete the weightsPerEpoch[][] in killGauge():

function killGauge(address gauge_) external onlyRole(_GOVERNANCE_ROLE) {...

uint256 epochTimestamp = _epochTimestamp(); totalWeightsPerEpoch[epochTimestamp] -= weightsPerEpoch[epochTimestamp][state.pool]; + delete weightsPerEpoch[epochTimestamp][state.pool]; emit GaugeKilled(gauge_); } However, the fix should take into consideration how the Minter calculates the emissions for every epoch (is it a fixed value every time or depending on how many gauges are active).

## Assessed type

Invalid Validation b-hrytsak (Fenix) confirmed alcueca (judge) commented:

Killing gauges can be considered normal operation,; therefore, the finding and severity are valid.

Medium Risk Findings (6)

# [M-01] mVeNFT DOS can’t trigger the vote function

- **Contest:** Fenix Finance Invitational
- **Slug:** 2024-09-fenix-finance-invitational
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-09-fenix-finance-invitational
- **Source snapshot:** competitions/2024-09-fenix-finance-invitational/final_report.html

mVeNFT DOS can’t trigger the vote function Submitted by Ch_301, also found by Ch_301

- https://github.com/code-423n4/2024-09-fenix-finance/blob/main/contracts/core/VoterUpgradeableV2.sol#L485
- https://github.com/code-423n4/2024-09-fenix-finance/blob/main/contracts/core/VoterUpgradeableV2.sol#L448
Description The VoterUpgradeableV2.sol contract has the function attachToManagedNFT(), users use it to delegate their veFNX voting power to a mVeNFT. One of the things this function does after receiving the new voting power is sub-call to _poke() and it will update the last voted timestamp of the mVeNFT.

lastVotedTimestamps [ tokenId_ ] = _epochTimestamp () + 1; At this point, the mVeNFT can’t trigger the vote function until the next epoch starts due to the _checkVoteDelay(). Even this check inside the vote() doesn’t help in this case.

if (!

managedNFTManagerCache.

isWhitelistedNFT ( tokenId_ )) { _checkEndVoteWindow (); } However, to make things worse this protocol is deployed on Blast transactions are too cheap malicious users can keep creating new locks every epoch with one wei in amount to bypass the zero check.

File:

VotingEscrowUpgradeableV2.

sol # _createLock () LibVotingEscrowValidation.

checkNoValueZero ( amount_ ); Then at the start of every new epoch (after the start of the voting window), just call attachToManagedNFT(). By doing this it keeps forcing the mVeNFT to vote to the same gauges.

## Impact

DOS attack where mVeNFT can’t invoke the vote function to change the weight of gauges; mVeNFT can’t reset its votes.

## Recommended Mitigation Steps

One solution is to not check the vote delay, However, I believe this comes with some trade-offs.

function vote ( uint256 tokenId_, address [] calldata poolsVotes_, uint256 [] calldata weights_ ) external nonReentrant onlyNftApprovedOrOwner ( tokenId_ ) { if ( poolsVotes_.

length != weights_.

length ) { revert ArrayLengthMismatch (); } bool x = managedNFTManagerCache.

isWhitelistedNFT ( tokenId_ ); if (!

x ) { _checkVoteDelay ( tokenId_ ); } _checkStartVoteWindow (); IManagedNFTManager managedNFTManagerCache = IManagedNFTManager ( managedNFTManager ); if ( managedNFTManagerCache.

isDisabledNFT ( tokenId_ )) { revert DisabledManagedNft (); } if (!

x ) { _checkEndVoteWindow (); } _vote ( tokenId_, poolsVotes_, weights_ ); _updateLastVotedTimestamp ( tokenId_ ); }

## Assessed type

DoS b-hrytsak (Fenix) confirmed and commented via duplicate Issue #9:

The _updateLastVotedTimestamp was not supposed to be in the _poke method, so cases like yours became possible.

/** * @dev Updates the voting preferences for a given tokenId after changes in the system.

* @param tokenId_ The tokenId for which to update voting preferences.

*/ function _poke(uint256 tokenId_) internal { //** code **// _updateLastVotedTimestamp(tokenId_); } alcueca (judge) decreased severity to Medium

# [M-02] The VoterUpgradeableV2.createV3Gauge function incorrectly uses v2GaugeFactory instead of v3GaugeFactory

- **Contest:** Fenix Finance Invitational
- **Slug:** 2024-09-fenix-finance-invitational
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-09-fenix-finance-invitational
- **Source snapshot:** competitions/2024-09-fenix-finance-invitational/final_report.html

VoterUpgradeableV2.createV3Gauge function incorrectly uses v2GaugeFactory instead of v3GaugeFactory Submitted by KupiaSec The gauges for the V3 pool are managed incorrectly by v2GaugeFactory rather than v3GaugeFactory.

## Recommended Mitigation Steps

It is recommended to change the code in the createV3Gauge function as follows:

- gauge = IGaugeFactory(v2GaugeFactory).createGauge( + gauge = IGaugeFactory(v3GaugeFactory).createGauge( token, votingEscrow, pool_, address(this), internalBribe, externalBribe, true, feeVault ); b-hrytsak (Fenix) confirmed and commented:

The problem is valid. Although there are some mitigations, as the implementations of v2/v3 factories, gauges are the same and it would not have led to any consequences at first. It is more of a flexibility for the future, regarding possible updates.

This submission is valid, it also seems to be Overseverity to the C4 description of problem severity.

alcueca (judge) decreased severity to Medium

# [M-03] If rewards are not distributed to some gauges in an epoch, it can lead to incorrect rewards distribution in the next epoch

- **Contest:** Fenix Finance Invitational
- **Slug:** 2024-09-fenix-finance-invitational
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-09-fenix-finance-invitational
- **Source snapshot:** competitions/2024-09-fenix-finance-invitational/final_report.html

Submitted by KupiaSec, also found by KupiaSec Some gauges may receive more rewards, while others may not receive any rewards at all.

## Recommended Mitigation Steps

Rewards should be distributed to all gauges per epoch, or the reward index mechanism should be improved.

b-hrytsak (Fenix) acknowledged and commented:

This issue is common for contracts like Voter ve(3,3), and it was also highlighted to us by the Hats audit providers here.

As you can see, we have the following mitigations:

The main method for distributing to the gaugeі is Voter.distributeAll(), which ensures that no gauge is skipped. In specific scenarios, other methods like Voter.distribute are also available.

Although users may be interested in calling these methods, the protocol also itself will handle this process to ensure the protocol’s viability and prevent such cases from occurring. Additionally, a distribution window has been introduced during which these calls should be made.

Skipping a gauge for an entire epoch is highly unlikely alcueca (judge) decreased severity to Medium and commented:

While the sponsor seems to be aware of this issue, and have some mitigations prepared, under the audit rules this is a valid finding because it is present in the code and wasn’t disclosed by the sponsor.

Downgraded to Medium since skipping rewards in an epoch would be an unusual precondition.

# [M-04] boostedValue should be added to permanentTotalSupply for permanently locked tokens

- **Contest:** Fenix Finance Invitational
- **Slug:** 2024-09-fenix-finance-invitational
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-09-fenix-finance-invitational
- **Source snapshot:** competitions/2024-09-fenix-finance-invitational/final_report.html

boostedValue should be added to permanentTotalSupply for permanently locked tokens Submitted by KupiaSec, also found by nnez Unlocking tokens from the permanent locking can be DoSed.

## Recommended Mitigation Steps

It is recommended to change the code in the VotingEscrowUpgradeableV2._processLockChange function as the following:

if (newLocked.isPermanentLocked) { - permanentTotalSupply += amount_; + permanentTotalSupply = permanentTotalSupply + amount_ + boostedValue; }

## Assessed type

DoS b-hrytsak (Fenix) confirmed and commented:

Indeed. After refactoring and changes, this point, although known, was missed in the new code.

It is difficult to understand the severity of this issue, it seems to be Overseverity. If we go with the worst-case scenario, the last user/users will not be able to unlock the permanent lock on their veNFTs, which will lead them to some additional temporary lock until the problem is resolved, as they would have to wait 182 days for full unlocking anyway.

alcueca (judge) decreased severity to Medium KupiaSec (warden) commented:

@alcueca - I think this is high severity. This vulnerability leads not only to a DoS but also to an incorrect calculation of voting power in the _balanceOfNFT function due to the incorrect accumulation of permanentTotalSupply.

File:

contracts \ core \ VotingEscrowUpgradeableV2.

sol 532:

function _checkpoint ( uint256 tokenId_, LockedBalance memory oldLocked_, LockedBalance memory newLocked_ ) internal { [...] 616:@> last_point.

permanent = LibVotingEscrowUtils.

toInt128 ( permanentTotalSupply ); File:

contracts \ core \ VotingEscrowUpgradeableV2.

sol 647:

function _balanceOfNFT ( uint256 tokenId_, uint256 timestamp_ ) internal view returns ( uint256 balance ) { 649:

if ( pointEpoch > 0 ) { 650:

Point memory lastPoint = nftPointHistory [ tokenId_ ][ pointEpoch ]; 651:

if ( lastPoint.

permanent > 0 ) { 652:@> return LibVotingEscrowUtils.

toUint256 ( lastPoint.

permanent ); alcueca (judge) increased severity to High and commented:

This vulnerability leads … also to an incorrect calculation of voting power This was not pointed out in the original submission, but it is right.

b-hrytsak (Fenix) commented:

@KupiaSec, @alcueca - In the balanceOfNFT calculation, data regarding the permanentTotalSupply is not used, and the impact of not accounting boostedValue in permanentTotalSupply is limited only to the calculation of the total voting power. This does not affect on votes processing, but only the outcome ( votingPowerTotalSupply() ).

The voting power for a user’s veNFT will still be calculated correctly.

This statement most likely arose because similar structures and pieces of code are used for the general voting power calculation and for the user. However, last_point in _checkpoint is from supplyPointsHistory, whereas in balanceOfNFT, nftPointHistory is used.

Ch_301 (warden) commented:

@alcueca, @KupiaSec - I believe there is some wrong assumption in this issue. The last_point.permanent and lastPoint.permanent are not the same thing in this logic.

last_point.permanent is related to supplyPointsHistory[] this mapping which is tracking the total supply changes.

However, lastPoint.permanent that used in _balanceOfNFT() is from nftPointHistory[][] this mapping which is recording the changes over time for every veNFT.

The value of lastPoint.permanent is updated here u_new.

permanent = permanent; nftPointHistory [ tokenId_ ][ nftStates [ tokenId_ ].

pointEpoch ] = u_new; } Which is acutely only this amount here. The impact is more like this QA (not a duplicate) last user can’t call unlockPermanent() successfully.

KupiaSec (warden) commented:

There is a confusion for two variables and I agree there is no incorrect calculation of voting power by the permanentTotalSupply. But there still exists DoS vulnerability.

Ch_301 (warden) commented:

@KupiaSec, I’m not sure if we can call this denial-of-service, because only the last permanent-lock is affected by losing his locked FNX tokens!

alcueca (judge) decreased severity to Medium and commented:

I think the last permanent lock being affected reasonably often merits a medium severity. Thanks @KupiaSec for retracting your previous statement about permanentTotalSupply.

Note: For full discussion, see here.

# [M-05] dettachFromManagedNFT might revert and temporarily prevent users from detaching in certain situations

- **Contest:** Fenix Finance Invitational
- **Slug:** 2024-09-fenix-finance-invitational
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-09-fenix-finance-invitational
- **Source snapshot:** competitions/2024-09-fenix-finance-invitational/final_report.html

dettachFromManagedNFT might revert and temporarily prevent users from detaching in certain situations Submitted by nnez Users’ veNFT might be temporarily undetachable, preventing users from performing action on their own veNFT.

## Recommended Mitigation

Users are expected to only include active pools in normal vote flow. If one of the pool is inactive, we can safely set its weight to zero and skip over it (gracefully, ignore it).

function _vote(uint256 tokenId_, address[] memory pools_, uint256[] memory weights_) internal { _reset(tokenId_); uint256 nftVotePower = IVotingEscrowV2(votingEscrow).balanceOfNFT(tokenId_); uint256 totalVotesWeight; uint256 totalVoterPower; for (uint256 i; i < pools_.length; i++) { GaugeState memory state = gaugesState[poolToGauge[pools_[i]]]; if (!state.isAlive) { delete weights_[i]; delete pools_[i]; continue; } totalVotesWeight += weights_[i]; } uint256 time = _epochTimestamp(); for (uint256 i; i < pools_.length; i++) { address pool = pools_[i]; if(pool == address(0)) continue; address gauge = poolToGauge[pools_[i]]; uint256 votePowerForPool = (weights_[i] * nftVotePower) / totalVotesWeight;

if (votePowerForPool == 0) { revert ZeroPowerForPool(); } if (votes[tokenId_][pool] > 0) { revert NoResetBefore(); } poolVote[tokenId_].push(pool); votes[tokenId_][pool] = votePowerForPool; weightsPerEpoch[time][pool] += votePowerForPool; totalVoterPower += votePowerForPool; IBribe(gaugesState[gauge].internalBribe).deposit(votePowerForPool, tokenId_); IBribe(gaugesState[gauge].externalBribe).deposit(votePowerForPool, tokenId_); emit Voted(_msgSender(), tokenId_, votePowerForPool); } if (totalVoterPower > 0) IVotingEscrowV2(votingEscrow).votingHook(tokenId_, true); totalWeightsPerEpoch[time] += totalVoterPower; }

## Assessed type

DoS b-hrytsak (Fenix) confirmed and commented:

Although there is a certain safe way to kill a gauge, etc., the described case is possible if the gauge is killed in the middle of an epoch for some reason, and as a result, the veNFT cannot be unhooked from the strategy for some time.

I am not sure that the recommended mitigation is optimal. Redistribution of votes between live pools decision is also not ideal

# [M-06] Potential incorrect index update in revived gauge under specific conditions

- **Contest:** Fenix Finance Invitational
- **Slug:** 2024-09-fenix-finance-invitational
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-09-fenix-finance-invitational
- **Source snapshot:** competitions/2024-09-fenix-finance-invitational/final_report.html

Submitted by nnez This vulnerability could allow revived gauges to claim more rewards than intended under specific circumstances, potentially leading to unfair distribution of rewards.

Description The reviveGauge function fails to update the gauge’s index to the current global index when reviving a previously killed gauge. While this issue is mitigated in most scenarios by the distributeAll function, which updates all gauges’ indices to the global index on each epoch, a vulnerability still exists under specific conditions.

Relevant code snippet:

function reviveGauge ( address gauge_ ) external onlyRole ( _GOVERNANCE_ROLE ) { if ( gaugesState [ gauge_ ].

isAlive ) { revert GaugeNotKilled (); } gaugesState [ gauge_ ].

isAlive = true; emit GaugeRevived ( gauge_ ); } function _distribute ( address gauge_ ) internal { GaugeState memory state = gaugesState [ gauge_ ]; uint256 currentTimestamp = _epochTimestamp (); if ( state.

lastDistributionTimestamp < currentTimestamp ) { uint256 totalVotesWeight = weightsPerEpoch [ currentTimestamp - _WEEK ][ state.

pool ]; if ( totalVotesWeight > 0 ) { uint256 delta = index - state.

index; // @contest-info outdated index can cause problem here if ( delta > 0 ) { uint256 amount = ( totalVotesWeight * delta ) / 1e18; if ( state.

isAlive ) { gaugesState [ gauge_ ].

claimable += amount; } else { IERC20Upgradeable ( token ).

safeTransfer ( minter, amount ); } gaugesState [ gauge_ ].

index = index; uint256 claimable = gaugesState [ gauge_ ].

claimable; if ( claimable > 0 && state.

isAlive ) { gaugesState [ gauge_ ].

claimable = 0; gaugesState [ gauge_ ].

lastDistributionTimestamp = currentTimestamp; IGauge ( gauge_ ).

notifyRewardAmount ( token, claimable ); emit DistributeReward ( _msgSender (), gauge_, claimable ); } The vulnerability arises in scenarios where:

There’s a large number of gauges in the protocol.

Due to gas limitations, distributeAll cannot update all gauges in a single transaction.

Manual iteration through gauges is required.

A killed gauge might not be updated before it’s revived as there is no incentive to call distribute function for a killed gauge.

In this specific scenario, a revived gauge could retain an outdated index, leading to incorrect reward calculations.

Example scenario Epoch x:

Gauge A is active with an index of 100.

Global index is 100.

Epoch x+1:

Gauge A is killed, its index stays at 100.

Global index updates to 150.

distributeAll fails to update all gauges due to gas limitations.

Epoch x+2:

Before manual updates reach Gauge A, it is revived with index still at 100.

Global index updates to 200.

When claiming rewards:

Gauge B (updated correctly) gets (200 - 150) * weight_B.

Gauge A incorrectly gets (200 - 100) * weight_A.

Gauge A claims excess rewards for the period it was killed. This discrepancy, while rare, could lead to unfair reward distribution for all gauges.

Rationale on severity High impact - Lead to loss of funds of other gauges.

Low likelihood - Only happen in specific circumstances.

Hence, Medium severity.

## Recommended Mitigation

function reviveGauge ( address gauge_ ) external onlyRole ( _GOVERNANCE_ROLE ) { if ( gaugesState [ gauge_ ].

isAlive ) { revert GaugeNotKilled (); } gaugesState [ gauge_ ].

isAlive = true; gaugesState [ gauge_ ].

index = index; // <-- update to global index emit GaugeRevived ( gauge_ ); }

## Assessed type

Context b-hrytsak (Fenix) acknowledged and commented:

Still needs some specific conditions, although this is technically a valid submission.
