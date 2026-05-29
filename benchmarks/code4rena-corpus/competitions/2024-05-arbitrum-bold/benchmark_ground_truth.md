# Benchmark Ground Truth: Arbitrum BoLD

## Accepted H/M Findings

# Accepted H/M Findings: Arbitrum BoLD

# [H-01] Adversary can make honest parties unable to retrieve their assertion stakes if the required amount is decreased

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-05-arbitrum-bold
- **Source snapshot:** competitions/2024-05-arbitrum-bold/final_report.html

Submitted by xuwinnie, also found by Ch_301

## Impact

When the required stake (to create a new assertions) is updated to a lower amount, adversary can make the honest party unable to retrieve their assertion stakes.

## Recommended Mitigation Steps

Ensure the following A staker is considered inactive only if her last staked assertion is confirmed.

A staker can only stake on her last staked assertion’s descendants. (otherwise Alice can switch to the correct branch and withdraw) gzeoneth (Arbitrum) confirmed and commented:

Patched with

- https://github.com/OffchainLabs/bold/pull/655.

# [H-02] Edge from dishonest challenge edge tree can inherit timer from honest tree allowing confirmation of incorrect assertion

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-05-arbitrum-bold
- **Source snapshot:** competitions/2024-05-arbitrum-bold/final_report.html

Submitted by Kow, also found by Emmanuel, xuwinnie, and SpicyMeatball

## Impact

Timers can be inherited across different challenge trees and consequently incorrect assertions can be confirmed.

## Recommended Mitigation Steps

Allow child edges (from bisection) to inherit the claimId of their parent and check that the claimId of the claiming edge matches the edgeId of the inheriting edge (this would require changes to isLayerZeroEdge ).

godzillaba (Arbitrum) confirmed gzeoneth (Arbitrum) commented:

Good catch.

Fixed in

- https://github.com/OffchainLabs/bold/pull/659.

Medium Risk Findings (2)

# [M-01] Inconsistent sequencer unexpected delay in DelayBuffer may harm users calling forceInclusion()

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-05-arbitrum-bold
- **Source snapshot:** competitions/2024-05-arbitrum-bold/final_report.html

forceInclusion() Submitted by 0x73696d616f

- https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/main/src/bridge/DelayBuffer.sol#L43
- https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/main/src/bridge/DelayBuffer.sol#L90-L98
- https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/main/src/bridge/SequencerInbox.sol#L287

## Impact

Buffer unexpected delay due to sequencer outage is inconsistent.

## Recommended Mitigation Steps

A mitigation must be carefully taken as not to introduce double accounting of the buffer delay. One solution that would fix this issue is tracking the total unexpected delay separately and making it equal to the block.number minus the maximum between the previous sequenced block number and the oldest delayed message that was not yet included. This way, by doing the maximum with the last sequenced, we guarantee that no double accounting of delays takes place. By doing the maximum with the oldest delayed message, we guarantee that the delay is real and not that no message was submitted.

Note: the following discussion has been condensed for this report. To view the full discussion, please see the original submission.

gzeoneth (Arbitrum) disputed and commented:

The delay buffer is an intermediary goal and not a final goal. The purpose of the delay buffer is to provide force inclusion assurances. The delay buffer updates are staggered and the buffer updates proactively in the force inclusion method (

- https://github.com/OffchainLabs/bold/blob/32eaf85e8ed45d069eb77e299b71fd6f3924bf40/contracts/src/bridge/SequencerInbox.sol#L309
). The behavior described is not unexpected and does not have impact on force inclusion.

Picodes (judge) decreased severity to Low/Non-Critical and commented:

This report shows how the delay buffer update could be nondeterministic and depend on how messages are included, but as shown by the sponsor fails to demonstrate why this would be important and how this fulfills the definition of Medium severity (function of the protocol or its availability could be impacted).

Picodes (judge) increased severity to Medium and commented:

After further discussion, this issue is to me somewhere between “function incorrect as to spec, state handling”, and “availability issue”.

Considering:

that the impact of the end-user not being able to force-include a message as soon as possible is that its funds may be locked for some time.

that if the sequencer is down, there may be multiple messages to force-include, and that the depletion is advertised in various places but is non-deterministic and could be “manipulated” to delay the following message, I’ll upgrade to Medium.

godzillaba (Arbitrum) commented:

that the impact of the end-user not being able to force-include a message as soon as possible is that its funds may be locked for some time @Picodes - I’m not sure this is true. In the example @0x73696d616f gave with 3 messages (m1,m2,m3 at T1,T2,T3), m1 not being included right away does not cause funds to be locked since m1 is included when m2 is included at T2, and m1 could have been included at any time between T1 and T2.

Picodes (judge) commented:

@godzillaba - I meant locked for some time. Funds can’t be withdrawn as soon as they should because the buffer isn’t depleting as fast as it should.

godzillaba (Arbitrum) commented:

If the same user (eg someone playing in an L3’s bold game) submits m2 and m3, and they force include only their own messages promptly (as in the example where there is a claimed bug) they are not affected. That user, the force includer, still can only experience some max amount of sequencer censorship over a given time period.

gzeoneth (Arbitrum) commented:

Arbitrum’s protocol has always been optimistic, we build most of our logic under the assumption of a honest participant exists. e.g.

- https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/main/README.md
It is assumed … honest validators can react to malicious action broadcasted on-chain immediately in the same block unless the malicious party spend their censorship budget.

This is unless the function is designed to be non-admin permissioned, then we enforce straighter guarantee. e.g.

There is a correct fix that is applied in the delayProofImpl() but not in forceInclusion().

Batch poster’s delayProofImpl have the “correct fix” where permissionless forceInclusion do not. This is by design to limit the complexity of the contracts.

If anyone is concerned about the total delay of delayed message, it is assumed they SHOULD force include as soon as possible. If they do not call force include, there are no protocol guarantee their message will ever be included. Any delay in calling forceInclusion on parent chain can be modeled as part of the censorship budget of an attacker.

Picodes (judge) commented:

@0x73696d616f - Can you please edit your PoC to show a force-inclusion reverting in one scenario because of the delay whereas in the other it works.

0x73696d616f (warden) commented:

Here is a POC showing how the buffer is correctly depleted only if messages are included sequentially. The buffer gets depleted due to actively including messages sequentially. If the second message in the loop is included directly instead, the buffer is not correctly depleted, and instead of ending up being the minimum, is a much larger value (600 vs 7320). Thus, users have to wait 2000 blocks instead of 600. It can be confirmed that initially they have to wait 2000 blocks, but in the end only 600, if we include sequentially and fix this bug. If we don’t include sequentially, they still have to wait 2000 blocks.

We can play around with the numbers and observe different outcomes, but this is a real, proven risk. Here users have to wait 2000/600 = 3.3 times more to get their transaction force included. This issue will happen, how it happens depends on the conditions. The buffer being depleted is an expected scenario and the code is built to handle this situation, so arguments based on the fact that this will never happen do not stand. The impact depends on delayBlocks, threshold, max, replenishRate and user behaviour. We can find a lot of combinations showing very strong impact, as well as some showing less impact, but we know for sure this is a real risk. A list of parameters or user behavior that increase/decrease the impact can be made, but it does not erase the fact that this risk exists. It fits exactly in the definition of a medium severity issue:

with a hypothetical attack path with stated assumptions, but external requirements.

function test_POC_InconsistentBuffer_Decrease () public { bool fix = false; maxTimeVariation.

delayBlocks = 2000; BufferConfig memory configBufferable = BufferConfig ({ threshold:

600, //60 * 60 * 2 / 12 max:

14400, //24 * 60 * 60 / 12 * 2 replenishRateInBasis:

714 }); ( SequencerInbox seqInbox, Bridge bridge ) = deployRollup ( false, true, configBufferable ); address delayedInboxSender = address ( 140 ); uint8 delayedInboxKind = 3; bytes32 messageDataHash = RAND.

Bytes32 (); for ( uint i = 0; i < 7; i ++) { vm.

startPrank ( dummyInbox ); bridge.

enqueueDelayedMessage ( delayedInboxKind, delayedInboxSender, messageDataHash ); vm.

roll ( block.

number + 1100 ); bridge.

enqueueDelayedMessage ( delayedInboxKind, delayedInboxSender, messageDataHash ); vm.

stopPrank (); vm.

roll ( block.

number + 2001 ); uint256 delayedMessagesRead = bridge.

delayedMessageCount (); if ( fix ) { seqInbox.

forceInclusion ( delayedMessagesRead - 1, delayedInboxKind, [ uint64 ( block.

number - 3101 ), uint64 ( block.

timestamp )], 0, delayedInboxSender, messageDataHash ); } seqInbox.

forceInclusion ( delayedMessagesRead, delayedInboxKind, [ uint64 ( block.

number - 2001 ), uint64 ( block.

timestamp )], 0, delayedInboxSender, messageDataHash ); } ( uint256 bufferBlocks,,,,,) = seqInbox.

buffer (); assertEq ( bufferBlocks, fix ?

600:

7320 ); vm.

startPrank ( dummyInbox ); bridge.

enqueueDelayedMessage ( delayedInboxKind, delayedInboxSender, messageDataHash ); vm.

stopPrank (); vm.

roll ( block.

number + 601 ); uint256 delayedMessagesRead = bridge.

delayedMessageCount (); if (!

fix ) vm.

expectRevert ( ForceIncludeBlockTooSoon.

selector ); seqInbox.

forceInclusion ( delayedMessagesRead, delayedInboxKind, [ uint64 ( block.

number - 601 ), uint64 ( block.

timestamp )], 0, delayedInboxSender, messageDataHash ); } Picodes (judge) commented:

With the above PoC in a longer sequence the effect on the sequenced - start gets indeed neutralized and we’re back with a DoS issue. This is Medium severity.

# [M-02] BOLDUpgradeAction.sol will fail to upgrade contracts due to error in the perform function

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-05-arbitrum-bold
- **Source snapshot:** competitions/2024-05-arbitrum-bold/final_report.html

BOLDUpgradeAction.sol will fail to upgrade contracts due to error in the perform function Submitted by SpicyMeatball, also found by dontonka and josephdara

## Impact

An error in the BOLDUpgradeAction.sol contract prevents it from upgrading and deploying new BOLD contracts.

## Recommended Mitigation Steps

function cleanupOldRollup() private { IOldRollupAdmin(address(OLD_ROLLUP)).pause(); uint64 stakerCount = ROLLUP_READER.stakerCount(); // since we for-loop these stakers we set an arbitrary limit - we dont // expect any instances to have close to this number of stakers if (stakerCount > 50) { stakerCount = 50; } + for (uint64 i = 0; i < stakerCount;) { address stakerAddr = ROLLUP_READER.getStakerAddress(i); OldStaker memory staker = ROLLUP_READER.getStaker(stakerAddr); if (staker.isStaked && staker.currentChallenge == 0) { address[] memory stakersToRefund = new address[](1); stakersToRefund[0] = stakerAddr; IOldRollupAdmin(address(OLD_ROLLUP)).forceRefundStaker(stakersToRefund); + stakerCount -= 1;

+ } else { + i++; + } } godzillaba (Arbitrum) confirmed gzeoneth (Arbitrum) commented:

Fixed with

- https://github.com/OffchainLabs/bold/pull/654/.

Picodes (judge) commented:

Keeping this report as best as the mitigation takes into account the if condition.

## Rejected Primary Findings

# Rejected Primary Findings: Arbitrum BoLD

# Inconsistent sequencer unexpected delay in DelayBuffer may harm users calling forceInclusion()

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-49
- **Submitter:** 0x73696d616f
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/49
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-49.md

## Brief Summary

Buffer unepected delay due to sequencer outage is inconsistent.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_84_group

# Unprotected initializer functions in `EdgeChallengeManager` and `RollupProxy` allow unauthorized initialization

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-154
- **Submitter:** 0xSecuri
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/154
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-154.md

## Brief Summary

The `initialize` function in the `EdgeChallengeManager` contract and the `initializeProxy` function in the `RollupProxy` contract lack access control, allowing anyone to call these functions. This can result in unauthorized initialization or re-initialization of the contracts, potentially leading to configuration corruption, loss of functionality, or compromise of the contract's intended behavior. Without proper access control, malicious actors can exploit these functions to manipulate the contract state, leading to severe security risks.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), :robot:_primary, :robot:_17_group

# CREATE2 address collision during pool deployment allows for complete draining of the pool before the assertion is created on the Rollup contract

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-115
- **Submitter:** 0xStalin
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/115
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-115.md

## Brief Summary

Funds transferred to an Assertion/Edge StakingPool can be drained by an attacker via a hash collision.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_06_group

# Inconsistent Length Assumption in `EdgeChallengeManager`

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-197
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/197
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-197.md

## Brief Summary

In the `initialize` function the validation of the `_stakeAmounts` array length the code checks if the length of the `_stakeAmounts` array matches the expected length based on the `_numBigStepLevel` value: [File: EdgeChallengeManager.sol#L360-L361](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/challengeV2/EdgeChallengeManager.sol#L360-L362) However, this check assumes that there should be exactly `_numBigStepLevel + 2` elements in the `_stakeAmounts` array. This assumption may not hold true if there are additional levels or edge cases that need to be considered.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_48_group

# Incorrect Recipient Address for Staking Amount in `EdgeChallengeManager`

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-198
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/198
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-198.md

## Brief Summary

The handling of the staking amount when creating a new layer zero edge, the code checks if the `stakeToken` address is non-zero and the `stakeAmounts` for the given level is non-zero. If both conditions are true, it transfers the staking amount from the caller to either the contract itself or the `excessStakeReceiver` address. However, the condition for determining the recipient address seems to be incorrect: [EdgeChallengeManager.sol#L433](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/challengeV2/EdgeChallengeManager.sol#L433) The `edgeAdded.hasRival` flag indicates whether the newly created edge already has a rival edge. Howeve...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# If the newly created lower child edge is not properly tracked and its existence is not recorded, it can lead to data loss. BisectEdge Missing EdgeAdded Event for New Child Edge (EdgeId 0).

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-199
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/199
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-199.md

## Brief Summary

[The `bisectEdge` function is responsible for creating two new child edges (lower and upper) from an existing edge.](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/challengeV2/EdgeChallengeManager.sol#L451-L488) If the lower child edge is newly created and happens to have an `edgeId` of `0`, the `lowerChildAlreadyExists` flag will be incorrectly set to true. This means that the contract will assume that the lower child edge already exists, even though it was just created. When a new edge is created, the contract should emit an `EdgeAdded` event to notify other contracts or clients about the new edge. However, due to the incorrect...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# `confirmEdgeByOneStepProof` Fails with Invalid Bridge Type.

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-202
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/202
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-202.md

## Brief Summary

[In the confirmEdgeByOneStepProof function](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/challengeV2/EdgeChallengeManager.sol#L539-L569), the way the `ExecutionContext` struct is created and passed to the `confirmEdgeByOneStepProof` function in the `EdgeChallengeManagerLib` library. The bridge field in the `ExecutionContext` struct is expected to be an instance of the `IBridge` interface. However, in the code provided, the bridge field is assigned the value returned by `assertionChain.bridge()`, which is likely a contract address. This could lead to issue when the `ExecutionContext` struct is used within the `confirmEdgeByOneSte...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# `refundStake` Fails with Non-ERC20 StakeToken.

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-203
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/203
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-203.md

## Brief Summary

The code assumes that the `stakeToken` address is a valid ERC20 token contract, but it does not check for this. If the `stakeToken` address is not a valid ERC20 token contract, calling the `safeTransfer` function on it will indeed _revert the entire transaction._ **This is a potential bug or at least a potential source of unexpected behavior.** Impact If the `stakeToken` address is not a valid ERC20 token contract, calling the `safeTransfer` function on it will revert the entire transaction, causing the `refundStake` function to fail and potentially leaving the contract in an inconsistent state. Explanation of the issue and its impact: The code assumes that the `stakeToken` address is a val...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_28_group

# The contract will revert in scenarios where it should not, and vice versa, potentially allowing invalid or malicious inputs to be accepted.

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-205
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/205
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-205.md

## Brief Summary

In the following lines: [SequencerInbox.sol#L147-L151](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/bridge/SequencerInbox.sol#L147-L151) The condition for reverting with the `DataBlobsNotSupported` error seems to be inverted. According to the code comments and the variable name `reader4844_`, it appears that the intention is to allow a non-zero `reader4844_` instance when `hostChainIsArbitrum` is false (i.e., not on the Arbitrum chain). However, the current code is doing the opposite: it reverts with `DataBlobsNotSupported` if `reader4844_` is not the zero address when `hostChainIsArbitrum` is true (on the Arbitrum chain). Impact

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# `updateRollup` May Not Update Rollup Address Correctly.

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-207
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/207
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-207.md

## Brief Summary

In the `updateRollupAddress` function. ` The issue is that the function is checking if the `rollup` address has changed by comparing the current `rollup` address with the new `rollup` address returned by `bridge.rollup()`. However, this comparison is done using the `==` operator, which compares the reference of the contract instances, not the actual contract addresses. In Solidity, contract instances are reference types, and the `==` operator compares the references, not the underlying contract addresses. This means that even if the new `rollup` address is different from the current `rollup` address, the `rollup == newRollup` condition may still evaluate to `true` if the two contract instan...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Uninitialized Variable in `getTimeBounds` Can Lead to Errors.

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-208
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/208
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-208.md

## Brief Summary

The calculation of [bounds.minBlockNumber](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/bridge/SequencerInbox.sol#L228-L230) in [getTimeBounds() function](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/bridge/SequencerInbox.sol#L216-L233). The issue here is that if `block.number` is less than or equal to `delayBlocks_`, `bounds.minBlockNumber` will not be assigned a value. This means that `bounds.minBlockNumber` could potentially be uninitialized, which could lead to unexpected behavior or errors when this value is used later in the code.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Staking token transfer to the `loserStakeEscrow` after assertions will fail when the RollupUserLogic contract holds less or no stake tokens at all

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-94
- **Submitter:** 0xrex
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/94
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-94.md

## Brief Summary

Every time the RollupUserLogic tries to transfer the calling validator's assertion stake to the `loserStakeEscrow` from its holding balance and its holding balance is less than enough to make the transfer, the call would fail as it assumes it already has a hold of the `requiredStake` amounts by then.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Whitelist validator functionality is flawed

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-260
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/260
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-260.md

## Brief Summary

In the current implementation, `createLayerZeroEdge()` function is used to create a layer zero edge on the assertion that is claimed to be true by the user. It's part of a dispute game and it can only be called if there are 2 conflicting assertions with some disagreement. However, except for different checks for the particular levels and creating the edge itself, the function verifies that the user is not able to create two layer zero edges that rival each other. The problem is that the existing validators may have already existing accounts meaning it's not possible to restrict their ability to behave this way.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_10_group

# The users will not be able to get refund of their stake for the levels lower than block level

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-269
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/269
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-269.md

## Brief Summary

In the current implementation of the `EdgeChallengeManager`, the users need to call `confirmEdgeByTime()` function in order to confirm that their edge is above the set confirmation timer. However, the problem is that the internal function checks whether the `totalTimeUnrivaled` is equal or more than the `confirmationBlockThreshold` after adding `claimedAssertionUnrivaledBlocks` to `totalTimeUnrivaled`. `claimedAssertionUnrivaledBlocks` will only have any value if the edge that was confirmed was of block level, otherwise, it'll be 0. Due to this check, edges of lower levels will not be able to set to `Confirmed` status and therefore will not be eligible for stake refund.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), :robot:_primary

# Imprecise management of stakers allowance can lead to loss of funds

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-264
- **Submitter:** Audinarey
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/264
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-264.md

## Brief Summary

Detailed description of the impact of this finding.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), :robot:_primary, :robot:_26_group

# Invalid Use of calldata in Internal Function Parameters

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-218
- **Submitter:** Avalance
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/218
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-218.md

## Brief Summary

Using calldata in internal function parameters causes a compilation error in Solidity, preventing the contract from being deployed. This is because calldata is only valid for external functions, and its misuse disrupts the contract's ability to function correctly within the Solidity language rules. calldata is more expensive to access compared to memory or storage. Using it unnecessarily in internal functions can result in inefficient gas usage

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# `RollupUserLogic#newStakeOnNewAssertion()` does not function as expected

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-350
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/350
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-350.md

## Brief Summary

Core functionality of the protocol would be affected since stakes are not allowed to have ETH attached.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_20_group

# Protocol's deployment of new contracts is unsafe

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-351
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/351
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-351.md

## Brief Summary

Deployments are unsafe in some cases are unsafe, most especially in cases where the salt is `0`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# The timer cache in `EdgeChallengeManagerLib` at a time would become unsettable

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-352
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/352
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-352.md

## Brief Summary

The availability of a core functionality of the protocol would be impacted since there would now be a restriction on the ability to accurately track and update timer-related functionalities, potentially impacting the protocol's ability to manage time-sensitive operations correctly.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Protocol hardcodes block minting time even if contracts are to deploy on other chains

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-353
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/353
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-353.md

## Brief Summary

The time to get to the `VALIDATOR_AFK_BLOCKS` blocks on arbitrum would be way different than on the mainnet showcasing a disparity in the protocol's implementation on both chains.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Possible Front-Running Attack on Edge Confirmation

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-238
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/238
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-238.md

## Brief Summary

An attacker can gain an unfair advantage by front-running the confirmation process, potentially earning rewards meant for legitimate users.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Ethereum Mainnet Deployment Limit Violation: Optimizing Bytecode Size for EdgeChallengeManager Contract

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-11
- **Submitter:** Dinesh11G
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/11
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-11.md

## Brief Summary

The primary impact of this issue is the inability to deploy the `EdgeChallengeManager` contract on the Ethereum mainnet. This limitation prevents the contract from being utilized in live scenarios, affecting the functionality and potential benefits of the system it is intended to support.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), :robot:_primary, :robot:_92_group

# Missing Implementation for Pool Existence Check

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-13
- **Submitter:** Dinesh11G
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/13
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-13.md

## Brief Summary

The absence of the logic to check for the existence of a pool and revert the transaction if it does not exist could lead to unintended behavior. Users might attempt to create or retrieve a pool that already exists, leading to potential confusion or misuse of the contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Missing Zero Withdrawal Check in `withdrawFromPool()` Function

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-289
- **Submitter:** Dinesh11G
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/289
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-289.md

## Brief Summary

Allowing transactions that result in no change to the user's balance can lead to increased gas costs for users and the network, as unnecessary transactions consume resources. Additionally, it could encourage users to repeatedly send transactions that appear to interact with the contract but do not actually change any state, potentially triggering unintended behaviors or vulnerabilities within the contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_107_group

# Inconsistent Solidity Compiler Versions Across Contracts

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-68
- **Submitter:** Dinesh11G
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/68
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-68.md

## Brief Summary

The security issue arising from the use of inconsistent Solidity compiler versions across different contracts within the same project. Specifically, some contracts are compiled with `pragma solidity >=0.6.9 <0.9.0;`, while others use `pragma solidity ^0.8.0;`. The primary impact of using different compiler versions is the potential exposure to vulnerabilities and behavioral differences between contracts. Older compiler versions may lack recent security patches and optimizations introduced in newer versions. Additionally, contracts compiled with different versions may exhibit different behaviors due to changes in language features and standards, leading to unpredictable outcomes and potentia...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Incorrect Contract Instantiation and Function Call in EdgeStakingPool

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-80
- **Submitter:** Dinesh11G
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/80
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-80.md

## Brief Summary

The issue identified in the `EdgeStakingPool` contract pertains to an incorrect attempt to instantiate the `EdgeChallengeManager` contract and call a non-existent `stakeAmounts` function. This mistake is present in the `createEdge` function, specifically at the line where `requiredStake` is calculated. The incorrect instantiation and function call in the `EdgeStakingPool` contract could lead to failed transactions, increased gas costs, and potential security vulnerabilities. Specifically, the contract attempts to create a new instance of `EdgeChallengeManager` using an existing contract address, which is not feasible in Solidity. Additionally, the call to a non-existent `stakeAmounts` funct...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), :robot:_primary, :robot:_74_group

# EdgeStakingPool.sol:: Token Contract Manipulation Vulnerability in the function createEdge()

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-377
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/377
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-377.md

## Brief Summary

The smart contract EdgeStakingPool.sol is using the ERC20 `safeIncreaseAllowance` function to set an allowance for the `challengeManager` contract before calling `createLayerZeroEdge` function in the `createEdge` function. However, there are no checks to ensure the `challengeManager` contract actually returned the same number of tokens. This way, an attacker can manipulate the `challengeManager` to not return the tokens, causing a successful attack on any malicious `challengeManager` contracts.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_80_group

# DelayBuffer.sol:: Unhandled Overflow in Buffer Calculation

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-378
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/378
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-378.md

## Brief Summary

In the contract DelayBuffer.sol, there's an arithmetic operation where the potential overflow is unhandled. Solidity provides safe math functions since version `0.8.0` to prevent overflows and underflows. But it cannot handle the overflow that may occur due the intricacy of this sequence of operations: `(elapsed * replenishRateInBasis) / BASIS`. The line of code involved is in the `calcBuffer` function: `buffer += (elapsed * replenishRateInBasis) / BASIS;` The multiplication operation may result in a number too large for a `uint256` variable, causing an overflow before the division operation is completed.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Excessive Privilege in Function removeDelayAfterFork()

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-379
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/379
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-379.md

## Brief Summary

The `removeDelayAfterFork()` function in the SequencerInbox.sol contract is part of the smart contract which allows the sequencer to bypass time-delay security measures. However, this function is not protected by any authentication, potentially allowing any malicious actor to abuse this privilege.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Arbitrary Third-party Contract Calls

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-381
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/381
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-381.md

## Brief Summary

In the `SequencerInbox` contract, certain functions like `addSequencerL2BatchFromOrigin`, `addSequencerL2BatchFromOriginDelayProof`, etc. have the implementation of the `refundsGas` function modifier that is declared in the `GasRefundEnabled` contract. This modifier allows for an external contract (gasRefunder contract) to be called. The call's argument is another external contract which is an arbitrary `IReader4844` contract. The contract does not validate that these external contracts come from a trusted source, which might lead to calls to malicious third-party contracts. Also, any external call could potentially fail and could be used to deliberately manipulate state-changing operations...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_05_group

# modifier onlyRollupOwnerOrBatchPosterManager() is incorrect

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-160
- **Submitter:** Jorgect
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/160
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-160.md

## Brief Summary

As the name of the modifier say, the `onlyRollupOwnerOrBatchPosterManager` is restricting that a function can be just called by the owner or the `batchPosterManager`. The problem is that the modifier is using and(&&) insted of or(||): [[Link]](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/bridge/SequencerInbox.sol#L108) Impact The batchPosterManager needs to have the owner role to call the function implementing this modifier, this is dangeruos since the batchPosterManager should not have access to the owner function.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# A malicious user can misconfigure the buffer.

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-162
- **Submitter:** Jorgect
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/162
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-162.md

## Brief Summary

The buffer it is a way of limiting repeated sequencer censorship of delayed inbox messages. (took it directly by @godzillaba from discord chat arbitrum code4arena) This buffer is been update in `forceInclusion` function (see the arrow below): [[Link]](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/bridge/SequencerInbox.sol#L310) [[Link]](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/bridge/DelayBuffer.sol#L68) The problem is that the `l1BlockAndTime` can be passed with a `l1BlockAndTime[0]` arbitrary value so low and the checks are passing. this successfully set the `se...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_59_group

# The perform function in the BOLDUpgradeAction contract can be griefing by front-running the RollupProxy deploymen with same `salt` param

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-195
- **Submitter:** Jorgect
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/195
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-195.md

## Brief Summary

The `perform` function is used to make the upgrade from the old protocol to the new protocol, is in charge to create the config, make the deploys and initialize the contracts: [[Link]](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/rollup/BOLDUpgradeAction.sol#L516) The problem is that the `RollupProxy` is been create by the CREATE2 opcode as the salt is deterministic an attacker can create a contract with the same salt and make the execution fail. Impact The upgrade process can be halted by an attacker

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), :robot:_primary, :robot:_02_group

# Error setting the owner in the rollup Admin contract in `perform` function in the BOLDUpgradeAction contract.

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-196
- **Submitter:** Jorgect
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/196
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-196.md

## Brief Summary

`Perform` function is used to upgrade the protocol from the old rollup to the new rollup, this functions is setting BOLDUpgradeAction as the owner of the rollup to make the set up the rollup since there are some only owner functions in the rollup. and then at the final of the function the contract is returning the owner of the rollup to the correct one (see the third arrow below): [[Link]](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/rollup/BOLDUpgradeAction.sol#L464C5-L541C6) The problem is that actually the `actualOwner` which is the `config.owner` (see the first arrow above) is been set to the BOLDUpgradeAction instead of the...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), :robot:_primary

# `safeIncreaseAllowance` could fail if USDT is used as `stakeToken`

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-153
- **Submitter:** Joshuajee
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/153
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-153.md

## Brief Summary

The `safeIncreaseAllowance` function is used to increase token allowance, the problem is that some tokens particularly USDT, requires allowance to be zero before allowance can be in set. Vulnerability Details The

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Lack of validation of parent assertion existence in `fastConfirmAssertion` function

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-342
- **Submitter:** Koolex
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/342
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-342.md

## Brief Summary

anyTrustFastConfirmer is supposed to be set only on an AnyTrust chain, it can force confirm any pending assertion, this is a feature which allows a committee members (multi-sig) to confirm assertions quicker. This can be done by calling any of those two functions: - `fastConfirmAssertion` => to confirm an existing assertion - `fastConfirmNewAssertion` => to create a new one and confirm it `fastConfirmNewAssertion` checks if the parent assertion exists. However, `fastConfirmAssertion` doesn't do that. Not sure how much `anyTrustFastConfirmer` is trusted, but I believe it shouldn't be allowed to create a completely new assertion chain based on a wrong parent assertion hash. If this happens, i...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_121_group

# Unauthenticated user manipulate the `totalDelayedMessagesRead` value

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-97
- **Submitter:** Mrxstrange
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/97
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-97.md

## Brief Summary

* The SequencerInbox.sol contract uses the `totalDelayedMessagesRead` state variable to ensure that the sequencer delay value does not decrease, thereby safeguarding the addSequencer process. Only the `addSequencerL2BatchImpl()` function can modify the `totalDelayedMessagesRead` value. This variable is utilized in various functions including `addSequencerL2BatchDelayProof()`, `addSequencerL2BatchFromOrigin()`, `addSequencerL2BatchFromOriginDelayProof()`, `addSequencerL2BatchFromBlobsDelayProof()`, `addSequencerL2Batch()`, and `addSequencerL2BatchFromBlobs()`. * All these functions are protected by access control, allowing only addresses with the `isBatchPoster` role to execute them. However...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_58_group

# Unauthorized Rollup Creation via createRollup Function

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-255
- **Submitter:** NoOne
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/255
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-255.md

## Brief Summary

The `createRollup` function lacks proper access control, allowing any external caller to create a new rollup instance. This can lead to unauthorized creation of rollups under the attacker's control, deployment of malicious contracts, and configuration of validators that can compromise the integrity of the rollup system. The attacker could gain control over the rollup, leading to significant financial losses and operational disruptions.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), edited-by-warden, :robot:_primary, :robot:_38_group

# Restricting Access to `createBridge` Function

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-259
- **Submitter:** NoOne
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/259
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-259.md

## Brief Summary

The `createBridge` function in the `BridgeCreator` contract does not currently have access control, allowing any external address to call it. This lack of restriction can lead to several security and operational issues, including unauthorized bridge creation, potential exploits, and resource abuse * Security Risks: Allowing any user to create bridges without restriction can introduce significant security vulnerabilities. Malicious actors could exploit this function to create bridges with malicious configurations, potentially compromising the integrity of the entire system. * Resource Consumption: Deploying new contracts is resource-intensive. Unrestricted access to the createBridge function...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), :robot:_primary, :robot:_45_group

# If both child edges have rivals , bottom up timer will not tell correct time

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-234
- **Submitter:** OMEN
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/234
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-234.md

## Brief Summary

timeUnrivaledTotal will retrieve incorrect timer if both child edges have rivals.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Unrestricted Access to Critical Functions in ERC20Mock Contract

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-219
- **Submitter:** Ruandevos
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/219
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-219.md

## Brief Summary

Here’s a detailed submission for the vulnerability found in the provided ERC20Mock contract. The vulnerability relates to the unrestricted access to the mint, burn, transferInternal, and approveInternal functions, which can be classified as a high-risk issue. Title

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), :robot:_primary

# Lack of Event Emission for Critical State Changes

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-359
- **Submitter:** SAQ
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/359
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-359.md

## Brief Summary

The contract's critical state-changing functions, such as `removeWhitelistAfterFork`, `removeWhitelistAfterValidatorAfk`, and `fastConfirmAssertion`, do not emit any events to log these changes. This omission can hinder monitoring and auditing efforts, making it more challenging to track significant actions within the contract. Impact: The absence of event emissions for critical state changes can lead to reduced transparency and difficulty in detecting and responding to crucial actions. Users and monitoring systems rely on events to track changes in contract state. Without these events, important state changes may go unnoticed, potentially allowing malicious activities to persist undetected...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Downcasting nextInboxPosition variable from uint256 to unit256 will truncate all the message hash above 2^64 that cause an discrepancy among the new message hash.

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-90
- **Submitter:** Satyam_Sharma
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/90
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-90.md

## Brief Summary

Downcasting nextInboxPosition from uint256 to uint64 will make the RollUpCore.createNewAssertion function unable to work for next InboxPosition messages hash due to the truncating messages hash above 2^64.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), edited-by-warden, :robot:_primary

# Transferring tokens will reverts on Arbitrum when `stakeToken` is `WETH`

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-309
- **Submitter:** Shubham
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/309
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-309.md

## Brief Summary

Quoting the sponsor's [comment](https://discord.com/channels/810916927919620096/1235688998480117790/1242767495241142333) on the contest's public Discord channel > Which token is the stakedToken? gzeon | OffchainLabs: it is assumed to be WETH or a token that is not fee-on-transfer, rebasing, have a transfer hook or otherwise have non-standard ERC20 logics. There are multiple instances in different contracts where `safeTransferFrom` is used to send the `stakeToken` from the user to the desired contract. Given the fact that the staking token is WETH, the issue is valid.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_90_group

# `Address.isContract()` is not a reliable way of checking if the input is a contract

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-314
- **Submitter:** Shubham
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/314
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-314.md

## Brief Summary

The underlying assumption of `pool` being an contract can be untrue.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_14_group

# [M-5] `EdgeChallengeManager::createLayerZeroEdge` - Insufficient Input Validation can lead to Denial of Service

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-33
- **Submitter:** Squilliam
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/33
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-33.md

## Brief Summary

The `createLayerZeroEdge` function in the `EdgeChallengeManager` contract lacks sufficient input validation for the `prefixProof` and `proof` fields of the `CreateEdgeArgs` struct. This can lead to a potential denial of service (DoS) vulnerability due to excessive gas consumption. The lack of input validation for the `prefixProof` and `proof` fields can lead to a denial of service (DoS) vulnerability in the following way: Denial of Service (DoS): If the attacker can sustain a high rate of transactions with large `prefixProof` and `proof` fields, it can lead to a denial of service situation. The contract may become overwhelmed with the processing of these transactions, causing it to consume...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), edited-by-warden, :robot:_primary

# [M-5] `RollupAdminLogic::setOwner` - Lack of Two-Step Transfer Process for Critical Privileges (Missing Confirmation Step + Potential Unauthorized Ownership Changes)

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-35
- **Submitter:** Squilliam
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/35
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-35.md

## Brief Summary

The `RollupAdminLogic` contract has a `setOwner` function that allows the current owner to directly set a new owner without a confirmation step. While the code emits an event (`OwnerFunctionCalled`) when the owner is changed, it lacks a two-step confirmation process that would provide an extra layer of security against unintentional or malicious owner changes. Transfer of critical privileges must be done in two-step process. A two-step transfer process, usually involving a request followed by a confirmation, adds an extra layer of security against unintentional or malicious owner changes. The lack of a two-step confirmation process for transferring critical privileges, such as contract owne...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), edited-by-warden, :robot:_primary

# [H-6] `MerkleTreeLib` - Missing Claimable Addresses in Leaf Hashing (Insecure Claim Process) can lead to Unauthorized Access and Theft of Assets

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-52
- **Submitter:** Squilliam
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/52
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-52.md

## Brief Summary

The `MerkleTreeLib` library does not include any specific functionality for hashing leaves with claimable addresses. The vulnerability is present throughout the library, as there are no explicit mechanisms to include claimable addresses when hashing the leaves of the Merkle tree. The absence of including claimable addresses when hashing the leaves of the Merkle tree can have significant consequences for the security and ownership of assets within the system. Attackers can exploit this vulnerability to claim assets that they are not entitled to, leading to unauthorized access and financial losses for rightful owners. The impact of this vulnerability includes: Theft of assets, as attackers ca...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), edited-by-warden, :robot:_primary

# [M-10] `RollupCreator::createRollup` - Lack of Validation on Input Data Size (Unintended Memory Access) can lead to Disruption of Protocol Functionality

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-53
- **Submitter:** Squilliam
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/53
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-53.md

## Brief Summary

The `createRollup` function in the `RollupCreator` contract does not include explicit validations on the size of input data when performing low-level operations. This lack of input data size validation could potentially lead to unintended memory access or buffer overflow vulnerabilities. The absence of proper validation on input data size in low-level operations can have a significant impact on the protocol's functionality, security, and reliability. Consequences include: Unintended Memory Access: Without proper input data size validation, an attacker can craft malicious inputs that exceed the expected boundaries. This can lead to reading or writing data to unintended memory locations, pote...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), edited-by-warden, :robot:_primary, :robot:_22_group

# [M-12] `MerkleTreeLib::verifyPrefixProof` - Unbounded Loop can lead to a DoS

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-57
- **Submitter:** Squilliam
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/57
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-57.md

## Brief Summary

The `verifyPrefixProof` function in the `MerkleTreeLib` library contains a while loop that iterates until the size reaches the `postSize`. If the `postSize` is significantly larger than the `preSize`, the loop could potentially consume a large amount of gas, leading to Out-of-Gas exceptions or Denial-of-Service (DoS) conditions. The unbounded loop in the `verifyPrefixProof` function can have the following impacts: Gas exhaustion: If the `postSize` is significantly larger than the `preSize`, the loop may iterate many times, consuming a large amount of gas. This can lead to Out-of-Gas exceptions, causing the transaction to revert and potentially making the contract unavailable. Denial-of-Serv...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), edited-by-warden, :robot:_primary

# [M-16] `AssertionStakingPool::withdrawStakeBackIntoPool` - Ignores the return value of the `withdrawStakerFunds()` function call and can lead to Contract Inconsistencies

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-67
- **Submitter:** Squilliam
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/67
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-67.md

## Brief Summary

In the `AssertionStakingPool` contract, the `withdrawStakeBackIntoPool()` function calls the `withdrawStakerFunds()` function from the `IRollupUser` interface. However, the return value of this function call is not being handled or used in any way. Ignoring the return value of the `withdrawStakerFunds()` function can lead to several issues: The `AssertionStakingPool` contract loses visibility into the outcome of the withdrawal operation. It cannot determine whether the withdrawal was successful or if any errors occurred. If the `withdrawStakerFunds()` function encounters an error or fails silently, the contract would not be aware of it, potentially leading to inconsistencies in the contract...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), edited-by-warden, :robot:_primary

# makeStakeWithdrawable() of assertionStaking.sol Is Impossible to be called

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-332
- **Submitter:** Takarez
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/332
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-332.md

## Brief Summary

The [makeStakeWithdrawable](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/assertionStakingPool/AssertionStakingPool.sol#L49-L50) function is impossible to be called This is due to the external [call](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/assertionStakingPool/AssertionStakingPool.sol#L51) that is being made to `RollUpUser` contract, what happen here is that the `AssertionStakingPool` contract will serve as the `msg.sender` when the call to the `RollUpUser` succeed. This is problematic as the [returnOldDeposit](https://github.com/code-423n4/2024-05-arbitrum-found...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_29_group

# `verifyInclusionProof()` vulnerable to forgery via proofs of incorrect lengths

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-24
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/24
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-24.md

## Brief Summary

The `MerkleTreeLib` library accepts and verifies `Merkle proofs` that the provided `leaf` is included in the `rootHash` at the specified index via `verifyInclusionProof()`. However, because the proof validation depends on the length of the `proof` rather than the `index` of the value to be proved, `Merkle proofs` with invalid lengths can be used to mislead the verifier and forge `proofs` for nonexistent transactions.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), edited-by-warden, :robot:_primary

# Use of `tx.origin` Over `msg.sender` in SequencerInbox Contract

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-265
- **Submitter:** Topmark
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/265
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-265.md

## Brief Summary

The misuse of `tx.origin` instead of `msg.sender` in SequencerInbox contract would lead to significant security risks. This can expose the contract to unauthorized access and manipulation, potentially leading to data breaches, loss of funds, or other forms of exploitation. This impact is particularly severe in Arbitrum Prototocl where such Error could result in substantial monetary losses and damage the reputation of the Protocol.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_24_group

# Wrong TotalTimeUnrivaled Implementation when Edges Upper ChildId is not Zero

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-268
- **Submitter:** Topmark
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/268
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-268.md

## Brief Summary

Wrong total Time Unrivaled calculation which will affect adequate Timer Cache By Children update at [L517](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/main/src/challengeV2/libraries/EdgeChallengeManagerLib.sol#L517) and correct confirmation of Edge By Time at [L733](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/main/src/challengeV2/libraries/EdgeChallengeManagerLib.sol#L733) of the EdgeChallengeManagerLib contract and Protocol in General

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Minimum Assertion Period is Wrongly Set During Initialization

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-277
- **Submitter:** Topmark
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/277
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-277.md

## Brief Summary

Minimum Assertion Period is Wrongly Set During Initialization as against protocol description and expectation which would allow assertions that does not meet the normal protocol minimum Delta time requirement based on [L207](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/main/src/rollup/RollupUserLogic.sol#L207) where it was used during stake on New Assertion in the RollupUserLogic contract. At first look this report might seem as a low impact as the Minimum Assertion Period can be easily reset at [L205](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/main/src/rollup/RollupAdminLogic.sol#L205) of the contract after initialization phase. However that does not s...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Complete Break of RollupAdminLogic Contract as no Validation is Present for all Functions

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-281
- **Submitter:** Topmark
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/281
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-281.md

## Brief Summary

Complete Break of RollupAdminLogic Contract as no Validation is Present for all Functions

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), :robot:_primary

# Assertion Timing can be manipulate in confirmEdgeByTime Function

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-334
- **Submitter:** XDZIBECX
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/334
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-334.md

## Brief Summary

The vulnerability is from the calculation of assertionBlocks within the confirmEdgeByTime function, The difference between the blocks at which the first and second child assertions are created is used to calculate assertionBlocks. This value is then used to determine totalTimeUnrivaled and here is the vulnerable line : here the function interacts with the assertion chain for creation blocks of the first and second child assertions. and the calculation of assertionBlocks relies on the timing between these blocks. if an attacker delays the submission of the second child assertion, they can extend assertionBlocks.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), :robot:_primary, :robot:_32_group

# there is an i ncorrect Handling of Validator AFK Check in _validatorIsAfk

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-376
- **Submitter:** XDZIBECX
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/376
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-376.md

## Brief Summary

The bug is from the logical conditions that is used in the function to determine if a validator is AFK, s the function independently checks two conditions using a logical or operator, which can lead to incorrect results here : The _validatorIsAfk function is checks if a validator is considered AFK by evaluating the block number of the last confirmed assertion and its first child block here The logic aims to return true if either condition is met: - the first child block of the latest confirmed assertion is more than VALIDATOR_AFK_BLOCKS blocks ago. - so if the creation block of the latest confirmed assertion is more than VALIDATOR_AFK_BLOCKS blocks ago. the function uses a logical or approa...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), :robot:_primary, :robot:_60_group

# Rouge oracle can be manipulated to affect the validity of assertions, potentially leading to "wrong assertions" to be confirmed

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-300
- **Submitter:** a12345
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/300
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-300.md

## Brief Summary

Detailed description of the impact of this finding. - Liveness: creating a large number of assertion branches --> triggering many dispute resolutions, exhausting resources of honest validators. - Attacker might be able to confirm "wrong assertions" and steal bond from honest validators by winning unfair challenges.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), edited-by-warden, :robot:_primary

# Unvalidated Assignment of `_validatorWalletCreator` Parameter.

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-240
- **Submitter:** aariiif
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/240
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-240.md

## Brief Summary

The issue is that the `_validatorWalletCreator` parameter is of type `address`, but it is being assigned directly to the `validatorWalletCreator` state variable, which is expected to be a contract address implementing a specific interface. code: [RollupCreator.sol:L70/79](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/rollup/RollupCreator.sol#L70-L79) The problem with this approach is that it does not validate whether the provided `_validatorWalletCreator` address is actually a contract address implementing the required interface. If an arbitrary address is passed as `_validatorWalletCreator`, it could lead to unexpected behavior...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Incorrect Initialization of Executors Array in `UpgradeExecutor` Contract

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-242
- **Submitter:** aariiif
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/242
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-242.md

## Brief Summary

The way the `executors` array is initialized and passed to the initialize function of the UpgradeExecutor contract. The code creates an array `executors` with a length of 1 and assigns the `rollupOwner` address to the first element: [RollupCreator.sol:280-281](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/rollup/RollupCreator.sol#L280-L281) However, when the initialize function is called, the entire executors array is passed as an argument: [RollupCreator.sol:282](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/rollup/RollupCreator.sol#L282) This means that if the `initi...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Missing Approval Check for ERC-20 Token Transfers in `_deployFactories` Function

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-244
- **Submitter:** aariiif
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/244
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-244.md

## Brief Summary

There's an issue with the refund logic when the `_nativeToken` is not the zero address (i.e., when using an ERC-20 token). The code attempts to transfer the required fee amount from the caller to the inbox contract before calling the `perform` function. However, there is no check to ensure that the caller has approved the required amount for the contract to transfer from their account. Here's the problematic code: [RollupCreator.sol:306-315](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/rollup/RollupCreator.sol#L306-L316) The `safeTransferFrom` will revert if the caller has not approved the contract to spend the required amount o...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Incorrect Calculation of Merkle Root in MerkleTreeLib.sol:root Function

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-245
- **Submitter:** aariiif
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/245
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-245.md

## Brief Summary

[MerkleTreeLib.sol:root](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/challengeV2/libraries/MerkleTreeLib.sol#L110-L140) The handling of the last non-zero entry in the `me` array in the root function. The code assumes that if the last entry in the `me` array is non-zero, it represents a balanced tree, and no further hashing is required. However, this assumption may not always hold true. Consider the following example: In this case, the last entry is non-zero, but the tree is not balanced. The correct root should be calculated by hashing the last entry with a zero value, as follows: However, the current implementation will return...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_30_group

# Insecure Use of Assert in MerkleTreeLib.sol:verifyPrefixProof Function

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-247
- **Submitter:** aariiif
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/247
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-247.md

## Brief Summary

The use of `assert` instead of `require` or a custom error in the [MerkleTreeLib.sol:verifyPrefixProof](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/challengeV2/libraries/MerkleTreeLib.sol#L312-L350) function can lead to potential security vulnerabilities and unexpected behavior. The issue is in the following line: [MerkleTreeLib.sol:340](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/challengeV2/libraries/MerkleTreeLib.sol#L340) The `assert` statement is used to check if the `size` variable is less than or equal to the `postSize` parameter. However, the use of `assert...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Duplicate Validators in setValidator Function Call May Lead to Unexpected Behavior or Security Issues

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-248
- **Submitter:** aariiif
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/248
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-248.md

## Brief Summary

The `setValidato` function is being called with a potentially invalid set of validators. If the validators array contains duplicate addresses, the `setValidator` function will treat them as separate validators, which may not be the intended behavior. Code block: [BOLDUpgradeAction.sol:526-533](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/rollup/BOLDUpgradeAction.sol#L526-L533) The problem is that this code assumes that all the addresses in the validators array are valid validators. However, it does not check if the `validators` array contains duplicate addresses. If there are duplicate addresses in the `validators` array, the `s...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Incorrect Handling of Maximum uint256 Value in mostSignificantBit Function

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-249
- **Submitter:** aariiif
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/249
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-249.md

## Brief Summary

In [UnitUtilsLib.sol mostSignificantBit function](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/challengeV2/libraries/UintUtilsLib.sol#L29-L70). The handling of the case where `x` is equal to `0x8000000000000000000000000000000000000000000000000000000000000000` (the maximum value for a `uint256`). In the current implementation, when `x` is equal to this maximum value, the function will incorrectly return `0` instead of `255` (the correct most significant bit position). Here's the problematic part of the code: [/UintUtilsLib.sol:32-35](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7d...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Integer Overflow Vulnerability in RollupAdminLogic.sol

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-251
- **Submitter:** aariiif
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/251
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-251.md

## Brief Summary

The handling of the `currentInboxCount` in the following lines: [RollupAdminLogic.sol:71-75](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/rollup/RollupAdminLogic.sol#L71-L75) The code is trying to ensure that the `currentInboxCount` is always greater than `config.genesisInboxCount`. However, there is issue if `config.genesisInboxCount` is set to the maximum value of `uint256` (i.e., `2^256 - 1`). In this case, when `currentInboxCount == config.genesisInboxCount`, the line `currentInboxCount += 1` will cause an integer overflow, resulting in `currentInboxCount` being set to 0 instead of the intended value of `config.genesisInboxC...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# `formBlobDataHash()` normalized gas amount may not work well with `submitBatchSpendingReport()`

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-169
- **Submitter:** adeolu
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/169
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-169.md

## Brief Summary

If the networks block.basefee > 0 and > blobCost, the blob batch poster may not get the refunds amount due to it.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), edited-by-warden, :robot:_primary, :robot:_61_group

# `firstChildBlock` value not set when first child assertion is created.

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-374
- **Submitter:** adeolu
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/374
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-374.md

## Brief Summary

`firstChildBlock` value not set when assertion is created as the first child. Child properties not updated when assertion is created during `RollUpAdminLogic.initalize()`. first child assertion creation process is incomplete

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), edited-by-warden, :robot:_primary

# isUpdateable() will return true if one of the cases are true.

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-93
- **Submitter:** adeolu
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/93
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-93.md

## Brief Summary

wrong logical operator use will cause buffer to be updateable if buffer is full or if buffer is synced.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), edited-by-warden, :robot:_primary

# RollupAdminLogic : incorrect `minimumAssertionPeriod` when the contract is created.

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-357
- **Submitter:** ak1
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/357
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-357.md

## Brief Summary

**Spam and Overuse:** Users can make assertions too frequently, potentially spamming the contract and overwhelming the system. **Network Congestion:** Frequent assertions can increase demand for block space, leading to higher gas prices and slower transaction times. **Performance Degradation:** The contract may slow down due to the need to process many assertions quickly. **Security Vulnerabilities:** Short periods can make the contract susceptible to Denial of Service (DoS) attacks. **Unfair Advantage:** Users who can afford higher gas fees might exploit the short period to dominate the contract's operations. **Increased Costs:** Users will incur higher transaction costs due to more freque...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_11_group

# Race Condition in SequencerInbox's _setBufferConfig Function

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-41
- **Submitter:** arabgodx
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/41
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-41.md

## Brief Summary

**Vulnerability Type:** Race Condition **Description:** The `_setBufferConfig` function in the `SequencerInbox` contract relies on the `isValidBufferConfig` function from the `DelayBuffer` contract to validate buffer configuration parameters. However, the state of the buffer and its configuration variables are updated based on the result of `isValidBufferConfig`. This creates a race condition where the state of `isValidBufferConfig` may change before the buffer variables are updated, potentially leading to inconsistent or unexpected behavior. **Impact:** The race condition in the `_setBufferConfig` function could lead to incorrect buffer configuration and synchronization states. This could...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), edited-by-warden, :robot:_primary, :robot:_55_group

# Missing Implementation of Functions in Arbitrum Rollup Contracts those that implements IAssertionChain

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-44
- **Submitter:** arabgodx
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/44
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-44.md

## Brief Summary

The contracts `RollupCore`, `RollupAdminLogic`, and `RollupUserLogic` inherit from `IRollupCore`, which defines the `bridge()` but not `validateConfig()` functions. However, none of these contracts provide their own implementation for these functions. This means that any contract inheriting from `RollupCore`, `RollupAdminLogic`, or `RollupUserLogic` will also lack implementations for these functions, potentially leading to unexpected behavior or errors.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), edited-by-warden, :robot:_primary, :robot:_51_group

# Inconsistent Access of validatorWhitelistDisabled in EdgeChallengeManager.sol

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-45
- **Submitter:** arabgodx
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/45
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-45.md

## Brief Summary

In the `EdgeChallengeManager.sol` contract, there's a line `bool whitelistEnabled = !assertionChain.validatorWhitelistDisabled();` where `validatorWhitelistDisabled` is accessed as a function. However, based on the referenced sections of code, `validatorWhitelistDisabled` is defined as a public boolean variable in `RollupCore.sol`, not as a function.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), edited-by-warden, :robot:_primary, :robot:_00_group

# Incorrect Implementation of SafeERC20 Functions in RollupUserLogic.sol

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-61
- **Submitter:** arabgodx
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/61
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-61.md

## Brief Summary

(https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/rollup/RollupUserLogic.sol#L367 Vulnerability details Vulnerability Details Incorrect Implementation of SafeERC20 Function The `RollupUserLogic.sol` contract in the Arbitrum Foundation repository has multiple instances of incorrect implementations of the SafeERC20 functions. These functions are supposed to ensure safe and proper token transfers, which is critical for security in smart contracts. Instances of Incorrect Implementations 1. **Instance 1: Line 215** [Link to code](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/r...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), edited-by-warden, :robot:_primary, :robot:_16_group

# Inadequate Access Control on `perform` Function in `DeployHelper`

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-75
- **Submitter:** arabgodx
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/75
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-75.md

## Brief Summary

The `perform` function in the `DeployHelper` contract is marked as `external` and lacks proper access control. This allows any external user to call this function and execute multiple `_fundAndDeploy` operations, potentially leading to unauthorized state changes and financial risks. Vulnerability Details: Location in Code: The perform function is defined in the `DeployHelper` contract. Link to Code:https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/rollup/DeployHelper.sol#L89-L133

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), edited-by-warden, :robot:_primary

# wrong implement of verifyPrefixProof

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-299
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/299
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-299.md

## Brief Summary

Detailed description of the impact of this finding. preSize can be equal to postSize.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_09_group

# amount zero transfer

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-301
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/301
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-301.md

## Brief Summary

Detailed description of the impact of this finding. here we are using safetransferFROM in receiveTokens but we are not checking whether amount is greater than 0 or not.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# divide-before-multiply : src/libraries/GasRefundEnabled.sol#L17-L51

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-104
- **Submitter:** benoitonchain
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/104
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-104.md

## Brief Summary

Performing division before multiplication in Solidity can lead to precision loss due to the way integer division truncates. In the context of the GasRefundEnabled contract, the vulnerability arises when calculating the number of calldata words and subsequently adjusting the gas left. By first dividing the calldata size by 32 to determine the number of calldata words, and then multiplying this result by 6 and adding the square of the calldata words divided by 512 to the gas left, precision loss can occur. This can potentially result in incorrect gas calculations and unexpected behavior in the contract execution.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# The upgradeSequencerInbox function might revert if postUpgradeInit is called when the buffer configuration has already been initialized

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-122
- **Submitter:** bigtone
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/122
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-122.md

## Brief Summary

The postUpgradeInit function can only be called if the contract's buffer configuration is not yet initialized. However, upgradeSequencerInbox calls this function without verifying the buffer configuration status. As a result, the function may revert, potentially disrupting all upgrade actions.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Contracts are vulnerable to fee-on-transfer accounting-related issues

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-17
- **Submitter:** caglankaan
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/17
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-17.md

## Brief Summary

The function(s) below transfer funds from the caller to the receiver via `transferFrom()`, but do not ensure that the actual number of tokens received is the same as the input amount to the transfer. If the token is a fee-on-transfer token, the balance after the transfer will be smaller than expected, leading to accounting issues. Even if there are checks later, related to a secondary transfer, an attacker may be able to use latent funds (e.g. mistakenly sent by another user) in order to get a free credit. One way to address this problem is to measure the balance before and after the transfer, and use the difference as the amount, rather than the stated amount. [312](https://github.com/code...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Insufficient Challenge Period Validation

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-367
- **Submitter:** cheatc0d3
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/367
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-367.md

## Brief Summary

The `initialize` function checks if `_challengePeriodBlocks` is zero but does not ensure that the value is within a reasonable and secure range. This oversight can lead to various critical issues. If the challenge period is too short, honest validators might not have enough time to detect and challenge fraudulent assertions, allowing malicious assertions to be confirmed unchecked. Conversely, if the challenge period is excessively long, it can cause significant delays in the confirmation of valid assertions, impacting the network’s efficiency and user experience. Additionally, long challenge periods can be exploited for Denial of Service (DoS) attacks, where malicious actors continuously op...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_27_group

# Uninitialised Local Variable in Memory

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-103
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/103
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-103.md

## Brief Summary

In the `createAssertion` function (line 74), the `AssertionNode` struct is declared but not fully initialized. Specifically, the `firstChildBlock` and `secondChildBlock` fields are not explicitly set, which means they default to zero. While this might be intended behavior, it can lead to potential issues if these fields are expected to be non-zero or if their zero values could be misinterpreted in the context of the application. This can cause unexpected behavior or logical errors in the contract's execution.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), edited-by-warden, :robot:_primary

# Reentrancy Events Emitted

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-105
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/105
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-105.md

## Brief Summary

Detailed description of the impact of this finding. Firstly, modifiers are missing from the following functions which makes them vulnerable. So, anybody can call these functions: The emit StakeWithdrawn should come before the IERC20(stakeToken).safeTransfer call to prevent re-entrancy. Here’s why: **Re-entrancy Risk:** If safeTransfer is called before updating the state and emitting the event, a re-entrant call could exploit this. Re-entrancy occurs when an external call is made before the state is updated, allowing the external contract to call back into the contract and manipulate the state. **Best Practice:** Update state variables before making external calls. Emit events after state ch...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), edited-by-warden, :robot:_primary, :robot:_21_group

# Pess-Strange-Setter in the setBufferConfig Function

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-213
- **Submitter:** debo
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/213
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-213.md

## Brief Summary

Detailed description of the impact of this finding. Location: Function `SequencerInbox.setBufferConfig(BufferConfig)` (src/bridge/SequencerInbox.sol#947-950) Description: The `setBufferConfig` function does not correctly set storage variables or pass arguments to external calls. It appears to be a setter, but it neither writes to storage nor utilizes function parameters appropriately.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), edited-by-warden, :robot:_primary

# Unchecked transfer in the createLayerZeroEdge function in EdgeChallengeManager contract using IERC20.safeTransferFrom as the function does not have a modifier

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-256
- **Submitter:** debo
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/256
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-256.md

## Brief Summary

Detailed description of the impact of this finding. Vulnerability: Unchecked Transfer in `createLayerZeroEdge` Function **Description:** The `createLayerZeroEdge` function in the `EdgeChallengeManager` contract contains a vulnerability related to the unchecked transfer of ERC20 tokens using the `IERC20.safeTransferFrom` method. This function does not have a modifier to ensure that the transfer is executed under specific conditions, which can lead to potential issues. **Details:** 1. **Function Overview:** - The `createLayerZeroEdge` function is responsible for creating a new layer zero edge and involves transferring a stake amount of ERC20 tokens from the caller to the contract or an excess...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), edited-by-warden, :robot:_primary

# Incorrect equality in the functions called getKeysetCreationBlock and packHeader function

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-362
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/362
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-362.md

## Brief Summary

Detailed description of the impact of this finding. Vulnerability: Incorrect Equality in `getKeysetCreationBlock` and `packHeader` Functions Description: **Location 1:** - **Function:** `SequencerInbox.getKeysetCreationBlock(bytes32)` - **File:** `src/bridge/SequencerInbox.sol` - **Lines:** 957-961 - **Issue:** The function uses a strict equality check (`ksInfo.creationBlock == 0`) to determine if a keyset exists. This can be manipulated by an attacker, as a valid keyset might have a `creationBlock` of zero, especially on chains where block numbers start from zero. **Location 2:** - **Function:** `SequencerInbox.packHeader(uint256)` - **File:** `src/bridge/SequencerInbox.sol` - **Lines:** 6...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_68_group

# Dubious typecast in the following functions getTimeBounds & setValidKeyset & _setBufferConfig & submiteBatchSpendingReport & _setMaxTimeVariation & formCallDataHash & packHeader functions

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-373
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/373
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-373.md

## Brief Summary

Detailed description of the impact of this finding. Vulnerability: Dubious Typecasting in Various Functions Description: The vulnerability arises from improper typecasting in several functions, leading to potential data loss or unexpected behavior. Specifically, the conversion from larger data types to smaller ones occurs without proper validation. Affected Functions and Locations: 1. **getTimeBounds** (`SequencerInbox.sol` lines 216-233): - **Issue**: Typecasting `block.timestamp` and `block.number` to `uint64`. - **Impact**: Potential overflow resulting in incorrect time bounds. -

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_41_group

# SetConfirmPeriodBlocks must used the "whenPaused" modifier

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-311
- **Submitter:** desaperh
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/311
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-311.md

## Brief Summary

All new assertions will be rejected until the admin uses forceConfirmAssertion. Some transactions may be lost in the process.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_70_group

# Unhandled Edge Cases in `multiUpdateTimeCacheByChildren` Function.

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-179
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/179
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-179.md

## Brief Summary

This function does not handle edge cases such as an empty `edgeIds` array or invalid edge IDs, it could lead to unexpected behavior or potential vulnerabilities if not handled properly. If the `edgeIds` array passed to the function is empty, the function will still execute the loop, but it won't perform any operations, it could lead to unexpected behavior or wasted gas if the function is called with an empty array. For example, if the function is called as part of a larger transaction or operation, the caller might expect certain side effects or state changes to occur, but those expectations would not be met if the `edgeIds` array is empty. Additionally, if there are any gas costs associate...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Unauthorized Access Vulnerability in `refundStake` Function.

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-180
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/180
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-180.md

## Brief Summary

`refundStake` function is marked as public, anyone can call it and potentially refund the stake for any edge, which could lead to unauthorized refunds if the function is not properly guarded. Any external party can call the `refundStake` function and refund the stake for any edge, even if they are not the legitimate staker or an authorized party. The [refundStake function](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/challengeV2/EdgeChallengeManager.sol#L572-L585) is marked as public, allowing anyone to call it without any access restrictions: This

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_109_group

# Integer Overflow Vulnerability in `_setMaxTimeVariation()` Function.

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-181
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/181
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-181.md

## Brief Summary

[_setMaxTimeVariation()](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/bridge/SequencerInbox.sol#L867-L882) casts the input values to `uint64`, which can overflow if the values are too large. > The function does not handle integer overflow correctly. Casting the input values to `uint64` without checking for overflow can lead to unexpected behavior and potential exploits.: - _If the input values are too large, the cast to uint64 will cause an overflow, resulting in an incorrect value being stored._ - _The incorrect value can lead to unexpected errors or crashes in the program._ **Exploits:** An attacker could exploit this bug by p...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Incorrect Assertion Hash Verification in `validateAssertionHash()` Function.

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-182
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/182
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-182.md

## Brief Summary

The `validateAssertionHash()` function incorrectly verifies the hash of the `prevAssertionHash` instead of the `afterState`. This issue can lead to the following potential risks: - The assertion hash is not correctly calculated, leading to incorrect validation of assertions. - Assertions that should be invalid may be accepted due to the incorrect hash verification. - Incorrectly verified assertions could potentially compromise the integrity of the rollup data.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_23_group

# Unhandled Assertion Status Values in `createAssertion()` Function.

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-183
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/183
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-183.md

## Brief Summary

If an assertion's status is set to a value that is not defined in the `AssertionStatus` enum, the behavior of the contract could become undefined or unpredictable. This could lead to unexpected and potentially harmful consequences, as the contract may not handle such cases correctly. The `AssertionStatus` enum is currently defined with only two values: `Pending` and `NoAssertion`. If you need to add more status values in the future, you would need to modify the `enum` and potentially update all the code that interacts with it. This could make the codebase harder to maintain and extend.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Zero Deposit Vulnerability in `_newStake Function` Allows Creation of Invalid Staker Records.

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-186
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/186
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-186.md

## Brief Summary

The `_newStake` function is responsible for creating a new stake for a user. However, due to the lack of validation for the `depositAmount` parameter, it is possible to create a new stake with a zero deposit amount. This means that a user could be registered as a staker without actually depositing any funds. The issue is related to the order of operations performed within the function. The function first checks if the sender is already a staker using the `isStaked` function: [#L139](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/rollup/RollupUserLogic.sol#L139) However, before creating the new stake, there is no check to ensure th...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Non-Atomic State Changes in `withdrawStakerFunds` Due to Premature Execution of `withdrawFunds()`

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-189
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/189
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-189.md

## Brief Summary

In [withdrawStakerFunds function](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/rollup/RollupUserLogic.sol#L358-L364) the order of operations first calls `withdrawFunds(msg.sender)` to calculate the `amount` to be withdrawn, and then it checks if the amount is greater than zero using the `require` statement. However, if the `withdrawFunds` function has any side effects or state changes, such as updating balances or performing other operations, these changes will be executed before the `require` check. If the `withdrawFunds` function returns zero, and it has already made state changes, those changes will persist even though the tr...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Infinite Loop in `assertionHash` Function Leading to Denial of Service (DoS) and Integrity Risks.

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-190
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/190
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-190.md

## Brief Summary

In the [assertionHash function](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/rollup/RollupLib.sol#L23-L34), the recursive call within the `assertionHash` function will cause an infinite loop, leading to a stack overflow error. _This means that any contract or function that calls `assertionHash` will eventually run out of gas and fail to execute, effectively rendering the function unusable._ - If the `assertionHash` function is called externally or by other contracts, the infinite loop and stack overflow can potentially be exploited to cause a Denial of Service (DoS) attack. An attacker could repeatedly call the function, consumi...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Initialization Oversight in `TransparentUpgradeableProxy` Deployment: Risks and Consequences.

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-191
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/191
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-191.md

## Brief Summary

In the [_createBridge function](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/rollup/BridgeCreator.sol#L63-L97), the way the `TransparentUpgradeableProxy` contracts are deployed. The third argument passed to the `TransparentUpgradeableProxy` constructor is the `data` parameter, which is expected to be the encoded function call that should be executed on the implementation contract after deployment. However, in this code, an empty string `""` is passed as the `data` argument, which means that no initialization function will be called on the implementation contracts after deployment. This can lead to issues because many of the impl...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Missing Initialization Parameter in SequencerInbox Contract: Risks and Consequences

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-192
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/192
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-192.md

## Brief Summary

In the following line: [#L122](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/rollup/BridgeCreator.sol#L122) The `SequencerInbox` contract is initialized with the `maxTimeVariation` and `bufferConfig` arguments. However, the `SequencerInbox` contract also requires the `isUsingFeeToken` parameter to be set during initialization. The `isUsingFeeToken` parameter determines whether the `SequencerInbox` contract should handle fee payments in the native token (e.g., ETH) or an ERC20 token. This parameter is crucial for the correct functioning of the `SequencerInbox` contract, as it affects how fees are processed and transferred. By not...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_78_group

# `isDelayProofRequired` Can Miss New Delayed Messages

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-222
- **Submitter:** foxb868
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/222
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-222.md

## Brief Summary

`isDelayProofRequired` function In the condition `afterDelayedMessagesRead > totalDelayedMessagesRead` assumes that `totalDelayedMessagesRead` is always less than or equal to the current number of delayed messages read. However, this assumption may not hold true in certain scenarios. For example, if the `totalDelayedMessagesRead` value is reset or modified elsewhere in the code, it could become greater than the current number of delayed messages read (`afterDelayedMessagesRead`). In such cases, the condition `afterDelayedMessagesRead > totalDelayedMessagesRead` would evaluate to `false`, even though there might be new delayed messages that require a delay proof.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_66_group

# L1 Gas Fees Not Properly Handled When Host Chain is Arbitrum.

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-224
- **Submitter:** foxb868
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/224
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-224.md

## Brief Summary

The calculation of the `extraGas` value when `hostChainIsArbitrum` is true. The code attempts to add the l1Fees to the `extraGa`s value, but the way it's done may lead to an incorrect result. Code: [src\bridge\SequencerInbox.sol:L768-L772](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/bridge/SequencerInbox.sol#L768-L772) The problem is that the `l1Fees` value is divided by `block.basefee` before being added to extraGas. However, according to the [Arbitrum documentation](https://docs.arbitrum.io/how-arbitrum-works/l1-gas-pricing), the `l1Fees` value returned by `getCurrentTxL1GasFees()` is already denominated in Wei, and it should...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Setting the gas price to `0` will likely cause the transaction to be rejected or never mined.

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-225
- **Submitter:** foxb868
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/225
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-225.md

## Brief Summary

In the [src\bridge\SequencerInbox.sol:791-823 addSequencerL2BatchImpl function](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/bridge/SequencerInbox.sol#L791-L822). In the following line: [src\bridge\SequencerInbox.sol:818-820](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/bridge/SequencerInbox.sol#L818-L821) The condition `!isUsingFeeToken` is checking if the chain is not using a fee token, which means it's using Ether (ETH) as the native currency. However, the function `submitBatchSpendingReport` is being called with a `gasPrice` argument of `0`. When submitting a tra...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# ksHash Calculation in `setValidKeyset` Can Lead to Collisions.

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-227
- **Submitter:** foxb868
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/227
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-227.md

## Brief Summary

[setValidKeyset function src\bridge\SequencerInbox.sol:903-919](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/bridge/SequencerInbox.sol#L903-L919), the calculation of the `ksHash` value: [src\bridge\SequencerInbox.sol:904-905](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/bridge/SequencerInbox.sol#L904-L905) The `ksHash` is calculated by first hashing the `keysetBytes` using `keccak256`, then concatenating the result with the hex value `0xfe`, and hashing the concatenated bytes again. The resulting 32-byte hash is then XORed with `(1 << 255)`, which is equivalent to `0...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# The order of operations when updating the `sendRoot` in the outbox contract and updating the internal state of the contract could lead to race condition.

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-228
- **Submitter:** foxb868
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/228
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-228.md

## Brief Summary

The [confirmAssertionInternal function src\rollup\RollupCore.sol:236-267](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/rollup/RollupCore.sol#L236-L267). The function first updates the `sendRoot` in the outbox contract by calling `outbox.updateSendRoot(sendRoot, blockHash)`. However, it then updates the internal state of the contract by setting `_latestConfirmed = assertionHash` and `assertion.status = AssertionStatus.Confirmed`. This order of operations can lead to a potential race condition or inconsistent state if the function is interrupted or reverted after updating the outbox contract but before updating the internal state.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Inaccurate Tracking of Total Withdrawable Funds.

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-232
- **Submitter:** foxb868
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/232
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-232.md

## Brief Summary

The `totalWithdrawableFunds` is incremented by the `amount` being added to the individual account's withdrawable funds, but it is not decremented when funds are withdrawn from an account. This means that the `totalWithdrawableFunds` value will gradually become larger than the actual sum of withdrawable funds across all accounts, leading to an inconsistent state.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_19_group

# genesisAssertionHash function may produce an incorrect hash value for the genesis assertion, which could potentially lead to inconsistencies in the system.

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-233
- **Submitter:** foxb868
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/233
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-233.md

## Brief Summary

In [genesisAssertionHash](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/rollup/RollupCore.sol#L520-L530) function the way the `emptyAssertionState` is created. The `AssertionState` struct is defined as follows: [src\rollup\AssertionState.sol:11-15](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/rollup/AssertionState.sol#L11-L15) In the `genesisAssertionHash` function, the `emptyAssertionState` is created like this: [src\rollup\RollupCore.sol:522](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/rollup/RollupCore...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Assertions that result in machineStatus.ERRORED are not marked as overflown. As a result there will be a period where only invalid assertions can be submitted and honest user transactions will revert.

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-295
- **Submitter:** gesha17
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/295
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-295.md

## Brief Summary

Assertions that overflow and result in a MachineStatus.ERRORED state are not marked as overflown. This means that an honest party will not be able to submit a valid assertion for the duration of the minimumAssertionPeriod, while a malicios actor would be able to do that if their assertion results in MachineStatus.FINISHED. This means that there will be a period in which only malicios actors can submit assertions. Also, since the honest actor must eventually be able to submit and prove their honest assertion their transactions to submit it will revert unexpectedly.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# afterGS.getInboxPosition() has a range of valid values, rather than a singular valid value, allowing multiple valid sibling assertions to be created.

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-325
- **Submitter:** gesha17
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/325
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-325.md

## Brief Summary

The Arbitrum BoLD technical deep dive specifies that: [(link)](https://github.com/OffchainLabs/bold/blob/main/docs/research-specs/TechnicalDeepDive.pdf) "The next assertion to be posted onchain must consume, at least, the specified number of inbox messages from its parent." However, the implementation is reversing that, requiring a user to supply an inboxPosition that consumes at most the target inbox position specified by the previos assertion: [createAssertion()](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/main/src/rollup/RollupCore.sol#L425C1-L427C66) in Rollup.sol As a result, creating a new assertion takes an after state inbox position parameter that does not have a...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), :robot:_primary

# When assertion hash equals zero, anyone can cause loss of funds to all stakers

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-270
- **Submitter:** josephdara
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/270
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-270.md

## Brief Summary

In the `AssertionStakingPool.sol`, people can stake their tokens in the pool to create an assertion on the rollup contract. However due to the nature of this contract, anyone can use the pooled user funds to make any assertion. Potentially causing loss of funds to all stakers if a wrong assertion is passed to the rollup contract.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), :robot:_primary, :robot:_03_group

# `BOLDUpgradeAction` breaks the Arbitrum GAC standard

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-276
- **Submitter:** josephdara
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/276
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-276.md

## Brief Summary

The link to the Governance Action contracts is provided in the readme of this contest. However here is a directt link to the documentation: This condition is violated by the `BOLDUpgradeAction.perform()`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Create new rollup will fail because of the challange manager initialize interface mismatch

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-137
- **Submitter:** ladboy233
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/137
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-137.md

## Brief Summary

Create new rollup will fail because of the challange manager initialize interface mismatch

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_69_group

# fastConfirmNewAssertion in RollupUserLogic missing access control

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-140
- **Submitter:** ladboy233
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/140
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-140.md

## Brief Summary

[this function below](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/6f861c85b281a29f04daacfe17a2099d7dad5f8f/src/rollup/RollupUserLogic.sol#L273) has no access control while the function is only meant to be called by the anyTrustFastConfirmer as the comments said: /* We trust the anyTrustFastConfirmer to not call this function multiple times on the same prev, * as doing so would result in incorrect accounting of withdrawable funds in the loserStakeEscrow. * This is because the protocol assume there is only 1 unique confirmable child assertion. */ but this function has no access control and anyone can call it multiple times to break the accounting of withdrawal funds. also,...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), :robot:_primary, :robot:_54_group

# frontrunning a assertion creation by creating the first child make staker lose fund.

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-161
- **Submitter:** ladboy233
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/161
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-161.md

## Brief Summary

frontrunning a assertion creation by creating the first child make staker lose fund.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# change of validatorWhitelistDisabled can make staker not withdraw their bond.

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-164
- **Submitter:** ladboy233
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/164
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-164.md

## Brief Summary

change of validatorWhitelistDisabled can make staker not withdraw their bond.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_64_group

# DOS in AbsBoldStakingPool ()

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-293
- **Submitter:** lanrebayode77
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/293
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-293.md

## Brief Summary

Due to the bond size required to create assertion, pools can be created to pull funds together from multiple honest validators to create an assertion. Due to a missing check, a malicious depositor can decide to deposit large amount bond to DOS the creation of assertion by front-running transaction with withdrawal

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_08_group

# createPool in EdgeStakingPoolCreator is suspicious of the reorg attack

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-333
- **Submitter:** peanuts
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/333
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-333.md

## Brief Summary

Reorg attacks causes users to deposit funds in the wrong contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_57_group

# RollupUserLogic does not expose the Pausable functionality, rendering the functions unable to be paused

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-341
- **Submitter:** peanuts
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/341
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-341.md

## Brief Summary

Functions cannot be paused/unpaused.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_53_group

# The requiredStake on stakeOnNewAssertion is directed by the previous assertion and not a base value

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-345
- **Submitter:** peanuts
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/345
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-345.md

## Brief Summary

Stake price when creating an assertion is not uniformed, and instead follow the previous staked value.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_40_group

# If both validator's and challenger's assertion is not honest, the wrong assertion can be confirmed with a winner getting back the bond

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-347
- **Submitter:** peanuts
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/347
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-347.md

## Brief Summary

Two wrong assertions can create an incorrect assertion and one will still retain its bond.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Using block.number as a time reference may be subjected to change if Ethereum upgrades

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-356
- **Submitter:** peanuts
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/356
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-356.md

## Brief Summary

The deadline of the dispute game is dependent on Ethereum. If Ethereum decreases their block time dramatically, the dispute game deadline will be very narrow.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# `forceInclusion` and `forceInclusionDeadline` potential manipulation by miners

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-279
- **Submitter:** pwning_dev
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/279
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-279.md

## Brief Summary

This vulnerability is present in functions such as forceInclusion and forceInclusionDeadline, where the contract relies on block timestamps and numbers for time-based logic without considering potential manipulation by miners. Impact - Financial Loss: Malicious miners could manipulate block timestamps or numbers to prematurely force inclusions of transactions, potentially bypassing time-based restrictions on certain operations. This could allow them to steal funds or execute unauthorized transactions, leading to financial losses for users or the contract itself. - Data Corruption: Time-based logic within the contract, such as enforcing delay periods or time windows, could be undermined by m...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), :robot:_primary

# Access Control in AbsBoldStakingPool.sol

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-123
- **Submitter:** sbh
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/123
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-123.md

## Brief Summary

The AbsBoldStakingPool contract lacks access control mechanisms for critical functions. This allows unauthorized users to deposit and withdraw funds, potentially compromising the integrity of the staking pool. Vulnerable Code: function depositIntoPool(uint256 amount) external { // Anyone can call this } function withdrawFromPool(uint256 amount) public { // Anyone can call this } Fixed Code (Secured): Adding onlyOwner ensures only the contract owner can perform these actions. import "@openzeppelin/contracts/access/Ownable.sol"; abstract contract AbsBoldStakingPool is IAbsBoldStakingPool, Ownable { using SafeERC20 for IERC20; address public immutable stakeToken; mapping(address => uint256) pu...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), :robot:_primary, :robot:_76_group

# Replay Attack in SequencerInbox.sol

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-157
- **Submitter:** sbh
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/157
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-157.md

## Brief Summary

Replay attacks involve resending the same message multiple times. Without a nonce or unique identifier, attackers can potentially replay transactions. In the provided code, there is no clear nonce handling in processMessage or addSequencerL2Batch. Nonce Handling: The provided SequencerInbox.sol code does not show nonce or replay protection mechanisms. Access Control: Critical functions like addSequencerL2Batch check if the caller is a batchPoster. However, this doesn't prevent replay attacks directly. State Update: totalDelayedMessagesRead and other state updates should be checked to ensure messages are not reprocessed.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), :robot:_primary

# most functions in contract `RollupAdminLogic` don't have access control

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-172
- **Submitter:** twcctop
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/172
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-172.md

## Brief Summary

contract `RollupAdminLogic` may not works well because most functions don't have access control, any one can change the state of the contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_44_group

# Users whose stakes were used to create the pool move can withdraw the excess stakes of other users by frontrunning them.

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-34
- **Submitter:** wisdomn_
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/34
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-34.md

## Brief Summary

Users whose stakes were used to create the pool move can exit the pool with the excess stakes of other users by frontrunning them.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), edited-by-warden, :robot:_primary

# Vulnerability in Staker Index Handling in deleteStaker Function

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-127
- **Submitter:** xiaohuanxiong_0311
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/127
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-127.md

## Brief Summary

The identified vulnerability in the given contract code lies in the mishandling of staker indices during the deletion of a staker in the `deleteStaker` function. This issue can lead to logical discrepancies and potentially unexpected behavior in the management of stakers. Title

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), :robot:_primary, :robot:_118_group

# Vulnerability in Sequence Number Validation Leading to Control Flow Manipulation

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-128
- **Submitter:** xiaohuanxiong_0311
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/128
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-128.md

## Brief Summary

This condition check is vulnerable due to how it handles the `sequenceNumber`. The `~uint256(0)` is essentially the maximum256-bit unsigned integer value. When `sequenceNumber` is set to this maximum value, no revert will be triggered despite potential discrepancies between `seqMessageIndex` and `sequenceNumber`. This allows the indirect manipulation of the control flow and validation mechanism that guards the sequence numbering of L2 batches. This unexpected behavior can disrupt the logic that relies on the validity of sequencer numbers for processing batches in an orderly and predictable manner.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), :robot:_primary

# Timing Validation Vulnerability in setMaxTimeVariation Parameter

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-129
- **Submitter:** xiaohuanxiong_0311
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/129
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-129.md

## Brief Summary

The vulnerability here involves the lack of validation for minimum sensible values for `setMaxTimeVariation`. Without proper minimum checks, users can effectively set the time variation parameters (`delayBlocks`, `futureBlocks`, `delaySeconds`, `futureSeconds`) to zero, which could have unintended consequences on the operation of the system that relies on these parameters to define time bounds for certain actions. This could lead to the inability of the system to process transactions that are slightly ahead or behind the blockchain's current time due to network latencies or block time variations, potentially causing transaction failures or other operational issues.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), :robot:_primary

# Checks that ensure the call is from EOA might not hold true in near future

- **Contest:** Arbitrum BoLD
- **Slug:** 2024-05-arbitrum-bold
- **Submission:** V-329
- **Submitter:** zanderbyte
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-arbitrum-foundation-validation/issues/329
- **Source snapshot:** competitions/2024-05-arbitrum-bold/submissions/raw/V-329.md

## Brief Summary

The contract [`SequencerInbox`](https://github.com/code-423n4/2024-05-arbitrum-foundation/blob/main/src/bridge/SequencerInbox.sol) relies on the `msg.sender == tx.origin` checks to ensure that calls are made only from EOAs. This mechanism is currently effective in distinguishing EOAs from contracts. However, the proposed [EIP-3074](https://eips.ethereum.org/EIPS/eip-3074#motivation) introduces two new operations to the EVM - `AUTH` and `AUTHCALL`, which can break this assumption. With this proposal, contracts could potentially relay transactions on behalf of EOAs, causing the above mentioned check to incorrectly pass for contract calls. The `msg.sender == tx.origin` check that ensures calls...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary
