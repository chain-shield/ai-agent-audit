# Benchmark Ground Truth: Unruggable Invitational

## Accepted H/M Findings

# Accepted H/M Findings: Unruggable Invitational

# [M-01] has been resolved utilising a ‘Game Finder’ approach (OPOutputFinder) similar to that used in the OPFaultVerifier. This facilitates an efficient search of submitted outputs giving consideration to both finalised and unfinalised Gateway/Verifier operation. It also minimises the need for excessive RPC calls within the Javascript implementation. The OP portal and oracle contracts provide access to data pertaining to the submission of outputs and finalisation, however naive usage of this data assumes efficient, ‘correct’ proposer operation.

- **Contest:** Unruggable Invitational
- **Slug:** 2024-12-unruggable-invitational
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-unruggable-invitational
- **Source snapshot:** competitions/2024-12-unruggable-invitational/final_report.html

The OP portal and oracle contracts provide access to data pertaining to the submission of outputs and finalisation, however naive usage of this data assumes efficient, ‘correct’ proposer operation.

Whilst these changes touch numerous files, most of these changes pertain to changes to configuration passing and are simple in nature.

Fixes:

- https://github.com/unruggable-labs/unruggable-gateways/commits/audit-fixes/

# [M-02] OPFaultVerifier ingests games that resolve incorrectly

- **Contest:** Unruggable Invitational
- **Slug:** 2024-12-unruggable-invitational
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-12-unruggable-invitational
- **Source snapshot:** competitions/2024-12-unruggable-invitational/final_report.html

OPFaultVerifier ingests games that resolve incorrectly Mitigation confirmed @92c9fbb36dee3a11e0cff1096ce4958ac1491ae6 Additional finding:

[F-10] NitroVerifier lacks proper node validation when retrieving latest created nodes in unfinalized state Mitigation confirmed @daf1325791356512940511a2bdc54e28d32bba37 Mitigation of [M-01] Confirmed issue is resolved with the added MAX_TRIE_DEPTH (248) check that ensures provided proof does not exceed the max depth of 248.

Mitigation of [M-02] The fix addresses the reported issue by rejecting any DisputeGame that is blacklisted by the Guardian when the game is resolved incorrectly.

However, the fix fails to account for the DISPUTE_GAME_FINALITY_DELAY_SECONDS in OptimismPortal2.sol#L67. This is known as the Air-gap period, where a game is only considered finalized when it is resolved for at least DISPUTE_GAME_FINALITY_DELAY_SECONDS. The purpose of it is to provide sufficient time for the Guardian to blacklist any incorrectly resolved games.

Furthermore, this air-gap period check extends beyond withdrawal finalization as it is used in AnchorStateRegisty#L203-L248, to determine if the game claim is finalized in a generic manner.

/// @notice Returns whether a game is finalized.

/// @param _game The game to check.

/// @return Whether the game is finalized.

function isGameFinalized ( IDisputeGame _game ) public view returns ( bool ) { // Game must be resolved.

if (!

isGameResolved ( _game )) { return false; } // Game must be beyond the airgap period.

if (!

isGameAirgapped ( _game )) { return false; } return true; } /// @notice Returns whether a game's root claim is valid.

/// @param _game The game to check.

/// @return Whether the game's root claim is valid.

function isGameClaimValid ( IDisputeGame _game ) public view returns ( bool ) { // Game must be a proper game.

bool properGame = isGameProper ( _game ); if (!

properGame ) { return false; } // Must be respected.

bool respected = isGameRespected ( _game ); if (!

respected ) { return false; } // Game must be finalized.

bool finalized = isGameFinalized ( _game ); if (!

finalized ) { return false; } // Game must be resolved in favor of the defender.

if ( _game.

status () != GameStatus.

DEFENDER_WINS ) { return false; } return true; } Previous recommendation It is recommended to use AnchorStateRegistry.isGameClaimValid() for finalized mode and also adapt it for unfinalized mode (to check game is proper, respected and not rejected). This will ensure that the verifier mirrors the OP’s game verification.

Latest recommendation As the previous recommendations above are based on new contracts that are not yet deployed, it will be better to mirror the current OP’s game verification based on their withdrawal for the deployed OptimismPortal.

Specifically, the new recommendation is to add the following checks for finalized mode ( minAgeSec == 0 ), Check game was not created before portal.respectedGameTypeUpdatedAt.

Ensure that the game has been resolved for at least DISPUTE_GAME_FINALITY_DELAY_SECONDS (passed airgapped period).

These checks currently exists in checkWithdrawal() in OptimismPortal2.sol#L505-L518 // The game must have been created after `respectedGameTypeUpdatedAt`. This is to prevent users from creating // invalid disputes against a deployed game type while the off-chain challenge agents are not watching.

require ( createdAt >= respectedGameTypeUpdatedAt, "OptimismPortal: dispute game created before respected game type was updated" ); // Before a withdrawal can be finalized, the dispute game it was proven against must have been // resolved for at least `DISPUTE_GAME_FINALITY_DELAY_SECONDS`. This is to allow for manual // intervention in the event that a dispute game is resolved incorrectly.

require ( block.

timestamp - disputeGameProxy.

resolvedAt ().

raw () > DISPUTE_GAME_FINALITY_DELAY_SECONDS, "OptimismPortal: output proposal in air-gap" ); Unruggable Fixed here.

Zenith Resolved by adding finalized mode validation to check that the selected game is finalized as follows:

(created > respectedGameTypeUpdatedAt && status == DEFENDER_WINS && block.timestamp - gameProxy.resolvedAt()) > DISPUTE_GAME_FINALITY_DELAY_SECONDS) Note:

in finalized mode, only games with respected game type is considered valid ( gameTypeBitMask == 0 ). Also, it only considers games with createdAt > respectedGameTypeUpdatedAt as games with createdAt <= respectedGameTypeUpdatedAt are considered retired in OP now.

clowestab (Unruggable) commented:

Resolution for outstanding F-13 recommendations as of 31st January 2025:

@92c9fbb.

Mitigation of additional finding [F-10] First review The fix for F-10 does not fully address the issue as stakerCount could also include zombies (stakers that lost in a challenge). That is because when stakers lost a challenge, they are turned into zombies and not removed from stakerCount immediately. That means it is possible for rejected nodes to have stakerCount > 0.

In Nitro’s code, the definition of no stakers refers to no zombie stakers. This is evident from RollupUserLogic.sol where the rejectNextNode() checks for zero non-zombies (active stakers) to proceed with the node rejection.

function rejectNextNode ( address stakerAddress ) external onlyValidator whenNotPaused {..

// Verify that no staker is staked on this node require ( firstUnresolvedNode_.

stakerCount == countStakedZombies ( firstUnresolvedNodeNum ), "HAS_STAKERS" ); Recommendation However, my recommendation is not to use stakerCount but instead rely on firstUnresolvedNode() and latestConfirmedNode() to skip rejected nodes in the following manner:

Trasverse backward from the latestNodeCreated() to find the node that satistify _minBlocks.

If the next iteration goes past the firstUnresolvedNode(), it should then continue the traversing using latestConfirmedNode() and traverse using node.prevNum to transfer through the confirmed nodes (just like finalized mode).

Doing so will allow us to skip the rejected nodes (below) in an efficient manner.

If we look at this diagram from Arbitrum we can see two scenarios of rejected nodes.

Rejected nodes (104, 105) that are after latestConfirmedNode() - It will skip these by jumping from firstUnresolvedNode() to latestConfirmedNode(), as it will skip the nodes that had been resolved and rejected by the protocol.

Rejected nodes (101) that are before latestConfirmedNode() - These are skipped simply by trasversing confirmed nodes like finalized mode, and not using descending node number. We need to consider this case as it is possible to select a node that is older than the latestConfirmed() node.

Note:

unresolved node 111 will eventually be rejected by the protocol, as it is a child of rejected node 104. That means we could either treat it as unresolved and allow it to be selected by getLatestContext() or try to anticipate the rejection and not allow it to be selected.

My recommended solution treats node 111 as unresolved and allow it to be selected due to simplicity and respecting the protocol’s state. There is nothing wrong to reject node 111, though it is more complex as there are no direct way to check if the parent/ancestor node is rejected. Furthermore, there are also such pending rejection scenarios, such as deadline and rejection of child/decedent nodes, so its not easy to address them all.

Unruggable Fixed here.

Zenith Resolved with a new _isNodeUsable() that checks specified node is not rejected with the check node.stakerCount > _rollup.countStakedZombies(index). When node.stakerCount == _rollup.countStakedZombies(index), it means that the node has zero active stakers, which by definition is a rejected or pending rejection node.

Additional Review Summary During the additional review of the codebase, 3 findings were surfaced, 1 Medium severity and 2 Low severity. The table below provides details regarding the status of those findings from the additional review.

Issue Status Mitigation URL

## Rejected Primary Findings

# Rejected Primary Findings: Unruggable Invitational

No rejected primary findings were captured for this contest in the current corpus.

- **Rejected primary count:** 0
- **Primary submission rows captured:** 0
- **Submission status:** captured_from_authenticated_browser
