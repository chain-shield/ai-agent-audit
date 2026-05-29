# Accepted H/M Findings: Kleidi

# [M-01] Gas griefing/attack via creating the proposals

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-10-kleidi
- **Source snapshot:** competitions/2024-10-kleidi/final_report.html

Submitted by Allarious, also found by Allarious, DemoreX, Japy69, Drynooo, KlosMitSoss, gesha17 ( 1, 2 ), tenge_club, Brene, and JustAWanderKid

- https://github.com/code-423n4/2024-10-kleidi/blob/c474b9480850d08514c100b415efcbc962608c62/src/Timelock.sol#L512-L539
- https://github.com/code-423n4/2024-10-kleidi/blob/c474b9480850d08514c100b415efcbc962608c62/src/Timelock.sol#L652-L665
The timelock acts in a way that once the proposals are submitted, they need to be cancelled or executed. This behaviour opens up a griefing attack vector towards the owners of the vault in case at least threshold amount of owners’ private keys are exposed.

When the keys are exposed, the attackers can send as many transactions as they need to the network from the safe with different salts. Even if one of the transactions go through, funds can be stolen. The protocol defence mechanisms in these situations is (1) Pause guardian can cancel all the proposals (2) Cold signers can cancel proposals.

Both these defence mechanisms require gas usage from the victim’s accounts, and it is important to note that they can not use the funds inside the Kleidi wallet. This can lead to a gas war between attackers and the victims and can cause them to at least cause a griefing attack.

## Impact

Assumption in this section is that the victims do not get external help and they have invested most of their liquidity inside Kleidi, and only kept minimal amounts out for gas payments.

Imagine if victims have access to F amounts of funds, and 95% of those funds is locked into Kleidi.

The proof of concept below shows that the gas consumption of cancel is close to 5% of schedule.

In case the keys are compromised, attackers can send many transactions spending G amount of gas. The requires the victims to need to spend 0.05 * G in gas to cancel those proposals.

The reward for attackers, only if one of their transactions go through, is 0.95 * F.

Given that victims only have access to 0.05 * F to pay for 0.05 * G, if attackers pay more than the funds inside the protocol, meaning ( G > F ), they can claim the funds in the protocol and drain it as victims do not have enough funds to cancel all proposals.

At the end, attackers can re-claim most of what they spent. Overall spending G - 0.95 * F = G - 0.95 * G = 0.05 * G, and steal 0.95 * G from the user.

Note: In case the victims have invested more than ~95% into the Kleidi, attackers will be able to make profit.

## Recommended Mitigation Steps

Add epochs to the timelock, each time the contract is paused, move the epoch to the next variable. Also, include epochs in the transaction hashes, and only execute transactions from this epoch. This way, the pause guardian does not need to clear all the transactions one by one, and once the epoch is moved to the next stage, all the previous transactions will be automatically invalidated.

Alex the Entreprenerd (judge) decreased severity to Medium and commented:

I adapted the test, that can be dropped in Timelock.t.sol to verify my statements:

function testGasConsumption () public { bytes32 scheduleSalt = bytes32 ( "saltxyz" ); uint256 numOfProposals = 1000; bytes32 [] memory saltArray = new bytes32 []( numOfProposals ); for ( uint i; i < numOfProposals; i ++) { saltArray [ i ] = keccak256 ( abi.

encodePacked ( "salt", bytes32 ( i + 1 ))); } bytes memory scheduleData = abi.

encode ( timelock.

updateDelay, MINIMUM_DELAY ); address timelockAddress = address ( timelock ); // initial call costs more gas vm.

prank ( address ( safe )); timelock.

schedule ( timelockAddress, 0, scheduleData, scheduleSalt, MINIMUM_DELAY ); // Schedule until we consume 30 MLN Gas vm.

startPrank ( address ( safe )); uint256 gasBeforeSchedule = gasleft (); uint256 count; while ( true ) { timelock.

schedule ( timelockAddress, 0, scheduleData, saltArray [ count ], MINIMUM_DELAY ); count ++; // Stop at 30 MLN gas used if ( gasBeforeSchedule - gasleft () > 30e6 ) { break; } console.

log ( "count", count ); uint256 gasAfterSchedule = gasleft (); vm.

stopPrank (); vm.

startPrank ( timelock.

pauseGuardian ()); uint256 gasBeforeCancel = gasleft (); timelock.

pause (); // 10000 -> 32,260,154 4.6% uint256 gasAfterCancel = gasleft (); vm.

stopPrank (); // vm.startPrank(address(safe)); // uint256 gasBeforeCancel = gasleft(); // for(uint256 i; i < numOfProposals; i++){ // timelock.cancel(ids[i]); // 10000 -> 44,890,040 448,900,040 6% // } // uint256 gasAfterCancel = gasleft(); // vm.stopPrank(); // For 100,000 proposals // shecdule 7,398,200,040 // pause guardian pause 340,048,201 ~ 4.6% // safe cancel 448,900,040 ~ 6% console.

log ( "Gas consumption of schedule: ", gasBeforeSchedule - gasAfterSchedule ); // 10000 -> 739,820,040 7,398,200,040 console.

log ( "Gas consumption of cancel: ", gasBeforeCancel - gasAfterCancel ); } It’s worth noting that the POC doesn’t work in isolation, leading me to believe that the math given is incorrect.

I have ran my POC in both modes, and both versions seems to indicate that the cost to attack is a lot higher than the cost to defend, specifically the attack is 7 times more expensive than defending.

I’m not fully confident that Foundry treats the calls as isolated in this way, so I’m happy to be corrected.

Result from forge test --match-test testGasConsumption -vv --isolate Ran 1 test for test / unit / Timelock.

t.

sol:

TimelockUnitTest [ PASS ] testGasConsumption () ( gas:

33562952 ) Logs:

count 282 Gas consumption of schedule: 30021964 Gas consumption of cancel: 4053325 7 times more expensive Result from forge test --match-test testGasConsumption -vv Ran 1 test for test / unit / Timelock.

t.

sol:

TimelockUnitTest [ PASS ] testGasConsumption () ( gas:

25463501 ) Logs:

count 403 Gas consumption of schedule: 30049168 Gas consumption of cancel: 1307414 22 times more expensive Barring a mistake from me, I think the finding is valid and Medium is the most appropriate as the guardian can with some likelihood prevent it as the cost of the attack and the setup is higher than the cost to defend.

Also the attack must be done over multiple blocks.

Alex the Entreprenerd (judge) commented:

Mitigation would require changing the way initiatives are tracked.

By simply shifting a “valid ts” all initiatives created and queued before it can be made invalid, this makes the change a O(1) meaning it should not longer be dossable.

ElliotFriedman (Kleidi) commented:

I think that cost on the attack side is likely more expensive than your PoC shows because it just pranks as the safe, and doesn’t generate the signatures, have them validated in the gnosis safe + increment the nonce in the gnosis safe + 21k base transaction cost. When you add all of that together, it would have to be at least 30x more expensive to attack than to defend.

Mitigation is in here:

- https://github.com/solidity-labs-io/kleidi/pull/53.

Alex the Entreprenerd (judge) commented:

I generally agree, when also considering memory expansion costs that should happen when dealing with so many signatures.

I think Medium severity is the most appropriate because the attack is IMO not possible in one block, but to say this could have been prevented would be incorrect.

Fundamentally, if the guardian doesn’t urgently pause, they may not be able to within a few blocks (strictly more than 1).

Medium seems appropriate given this.

# [M-02] Wrong handling of call data check indices, forcing it sometimes to revert

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-10-kleidi
- **Source snapshot:** competitions/2024-10-kleidi/final_report.html

Submitted by 0xAlix2, also found by DemoreX, DanielArmstrong, 0x37, jsonDoge, dhank, KlosMitSoss, stuart_the_minion, Allarious, and klau5 Cold signers can add call data checks as whitelisted checks that hot signers could execute without timelocks, the call data checks depend on the indices of the encoded call. However, the protocol invalidly handles these indices in 2 separate places:

- https://github.com/code-423n4/2024-10-kleidi/blob/main/src/Timelock.sol#L1136
- https://github.com/code-423n4/2024-10-kleidi/blob/main/src/BytesHelper.sol#L50
Where length is computed as end index—start index, which is usually wrong as index subtraction needs +1 to be translated to a length. For most of the scenario, this is okay; however, if a parameter that is being checked filled all of its bytes then this would be an issue (PoC is an example). For example, a uint256 filling all of its 32 bytes.

NB:

This is not caught in the unit tests because there isn’t any test that checks this edge case, where a parameter that fills all its bytes is being checked.

This forces the whitelisted call to revert.

## Recommended Mitigation Steps

In BytesHelper.sol:

function sliceBytes(bytes memory toSlice, uint256 start, uint256 end) public pure returns (bytes memory) {...

- uint256 length = end - start; + uint256 length = end - start + 1; bytes memory sliced = new bytes(length);...

} In Timelock.sol:

function _addCalldataCheck( address contractAddress, bytes4 selector, uint16 startIndex, uint16 endIndex, bytes[] memory data ) private {...

for (uint256 i = 0; i < data.length; i++) { /// data length must equal delta index require( - data[i].length == endIndex - startIndex, + data[i].length == endIndex - startIndex + 1, "CalldataList: Data length mismatch" ); bytes32 dataHash = keccak256(data[i]); /// make require instead of assert to have clear error messages require( indexes[targetIndex].dataHashes.add(dataHash), "CalldataList: Duplicate data" ); }...

} ElliotFriedman (Kleidi) confirmed and commented:

Good finding, valid medium!

Alex the Entreprenerd (judge) commented:

Seems to be very closely related to issue #2.

I’d be careful about mitigating these. Probably best to use both cases for tests and then mitigate in one go.

Alex the Entreprenerd (judge) commented:

I’m not fully confident this bug is not different from the addCalldataChecks, checking in with the Sponsor to see how the bugs are mitigated.

@ElliotFriedman can you please confirm if you fixed this issue separately, or if you fixed it by fixing the finding from issue #2 ?

Alex the Entreprenerd (judge) commented:

As discussed am making a duplicate of the rest of the reports tied to indices.

@ElliotFriedman would appreciate if you can re-link all fixes tied to indices as to ensure the issues and gotchas were fixed.

ElliotFriedman (Kleidi) commented:

Mitigated with this PR

- https://github.com/solidity-labs-io/kleidi/pull/54/files
All other changes were backed out as we realized the previous ways of fixing things were incomplete.

# [M-03] UpdateExpirattionPeriod() cannot be executed when the newExpirationPeriod is less than currentExpirationPeriod

- **Contest:** Kleidi
- **Slug:** 2024-10-kleidi
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-10-kleidi
- **Source snapshot:** competitions/2024-10-kleidi/final_report.html

UpdateExpirattionPeriod() cannot be executed when the newExpirationPeriod is less than currentExpirationPeriod Submitted by dhank, also found by KlosMitSoss Safe cannot reduce expirationPeriod to a newExpirationPeriod when currentTimeStamp < timestamp[id] + expirationPeriod and currentTimeStamp >= timestamp[id] + newExpirationPeriod where id is the hash of updateExpirationPeriod() and timestamp[id] is the timestamp when the id can be executed.

Safe should be able to update the expirationPeriod to any values >= MIN_DELAY by scheduling the updateExpirationPeriod() and later execute from timelock when the operation is ready (before the expiry).

require ( newPeriod >= MIN_DELAY, "Timelock: delay out of bounds" ); But the protocol has overlooked the situation and added an reduntant check inside _afterCall() which is executed at the end of _execute().

function _afterCall ( bytes32 id ) private { /// unreachable state because removing the proposal id from the /// _liveProposals set prevents this function from being called on the /// same id twice require ( isOperationReady ( id ), "Timelock: operation is not ready" ); //@audit timestamps [ id ] = _DONE_TIMESTAMP; } Here the isOperationReady(id) will be executed with the newExpirationPeriod.

code function isOperationReady ( bytes32 id ) public view returns ( bool ) { /// cache timestamp, save up to 2 extra SLOADs uint256 timestamp = timestamps [ id ]; return timestamp > _DONE_TIMESTAMP && timestamp <= block.

timestamp => && timestamp + expirationPeriod > block.

timestamp; } There it is checking whether the currentTimestamp is less than the timestamp + updated EpirationPeriod instead of the actual expirationPeriod.

## Recommended Mitigation Steps

function _afterCall ( bytes32 id ) private { //no need to check timestamps [ id ] = _DONE_TIMESTAMP; } ElliotFriedman (Kleidi) confirmed, but disagreed with severity and commented:

Seems like this is a valid issue, but it’s valid only if you execute the proposal more than min delay after the transaction becomes executable and you are lowering the expiration period.

The title is misleading because you can execute this operation, but you just have to execute it within the new expiration period.

Feels more like a low severity than a medium.

Alex the Entreprenerd (judge) commented:

I need to think about it a bit more, but fundamentally it seems to be something that the owner would cause to themselves.

Alex the Entreprenerd (judge) decreased severity to Low/Non-Critical and commented:

With a similar point to issue #21 this is an operative mistake that the user can make.

Because this is a gotcha, where under valid use no harm would be done, I think the finding is best categorized as QA.

Alex the Entreprenerd (judge) commented:

After running the test, and reviewing the code, I see the issue.

The new expiration is being used to validate the executed function.

I see that this is a valid bug and am leaning towards raising the severity to Medium.

ElliotFriedman (Kleidi) commented:

I agree that this is a valid finding, so now we’re just talking about impact and severity. The solution for the end user is just execute the transaction before the new expiration period takes place. We can warn on the UI about this.

@Alex the Entreprenerd - Will leave severity of finding to your judgement.

Alex the Entreprenerd (judge) increased severity to Medium and commented:

The finding is a bit of an edge case, when changing a proposal expiration to a smaller value, the OZ reentrancy guard will use the expiration that was newly set, causing the execution to revert.

Fundamentally given this specific scenario, a proposal will not be executable, this leads me to agree with Medium severity.
