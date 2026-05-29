# Benchmark Ground Truth: Optimism Superchain

## Accepted H/M Findings

# Accepted H/M Findings: Optimism Superchain

# [H-01] Invalid DISPUTED_L2_BLOCK_NUMBER is passed to VM

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-optimism-superchain
- **Source snapshot:** competitions/2024-07-optimism-superchain/final_report.html

DISPUTED_L2_BLOCK_NUMBER is passed to VM Submitted by xuwinnie The span of the game tree at split depth is far larger than the length between the starting block and the claimed block. When starting block + trace index + 1 > claimed block, honest party should continue to commit to the root of the claimed block. However, the DISPUTED_L2_BLOCK_NUMBER passed to the VM is always starting block + trace index + 1, which means the op-program (at inter-block perspective) will not stop until it reaches the l2 safe head (corresponding to parenthash), and if the claimed block is earlier than the safe head, it can be challenged and will be considered invalid.

## Recommended Mitigation Steps

uint256 l2Number = min(startingOutputRoot.l2BlockNumber + disputedPos.traceIndex(SPLIT_DEPTH) + 1, l2BlockNumber()); Note func (d *Driver) ValidateClaim(l2ClaimBlockNum uint64, claimedOutputRoot eth.Bytes32) error { l2Head:= d.SafeHead() outputRoot, err:= d.l2OutputRoot(min(l2ClaimBlockNum, l2Head.Number)) if err != nil { return fmt.Errorf("calculate L2 output root: %w", err) } d.logger.Info("Validating claim", "head", l2Head, "output", outputRoot, "claim", claimedOutputRoot) if claimedOutputRoot != outputRoot { return fmt.Errorf("%w: claim: %v actual: %v", ErrClaimNotValid, claimedOutputRoot, outputRoot) } return nil } Here l2ClaimBlockNum is just DISPUTED_L2_BLOCK_NUMBER, so DISPUTED_L2_BLOCK_NUMBER

clearly should be capped at claimed l2 block number; otherwise, the inter-block op-program execution will never stop until it reaches safe head, which means all claims earlier than safe head is invalid in op-program’s perspective.

## Assessed type

Context ajsutton (Optimism) confirmed and commented:

This is valid and an excellent analysis. I’ve seen a number of claims that the DISPUTED_L2_BLOCK_NUMBER needs to be capped but before this all of them have provided an example where trace extension still results in the correct game resolution. This is the only case I’ve seen that identifies the actual case that I believe will give the wrong result. It does require a few more details to make the attack actually work though.

Specifically, if the correct output root is proposed for block 13, the attacker could counter using a trace that includes valid blocks up to block 14. The value the attacker uses for block 14 must be the honest value and block 14 must be safe at the game’s L1 head. In the honest actor’s trace, the output for block 13 is extended to the end of the game so the only difference is the attacker includes one extra block in the trace they use. At the split depth both actors would agree on block 13 and disagree on block 14 and then execute cannon/op-program to decide if the transition from block 13 to 14 is valid or not. Since block 14 is a valid safe block, op-program will be able to continue the derivation past block 13 and resolve that the claimed block 14 is valid even though it’s past the proposed output root.

In addition, you have to manipulate the game so that either the dishonest actor posts the first claim in the bottom half or that the honest actor is required to post the first claim of the bottom half such that the post state is an honest claim. Otherwise the restrictions on the required status code for the root of the bottom game and not being able to defend the root of the bottom claim prevent the attack.

# [H-02] The LPP challenge period can cause malicious and freeloader claims to be uncounterable and can also cause freeloader claims to be abused to entrap honest challengers

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-optimism-superchain
- **Source snapshot:** competitions/2024-07-optimism-superchain/final_report.html

Submitted by RadiantLabs, also found by Zubat and alexfilippov314 In some cases, the honest challenger must pass an LPP in order to counter a MAX_GAME_DEPTH claim through step(). For instance, “in the event that the preimage is too large to be submitted through calldata in a single block, challengers must resort to the streaming option” as described in the Optimism specs.

However, an issue arises because to pass an LPP, the honest challenger must wait for CHALLENGE_PERIOD seconds in order to pass the LPP to be able to step() against a malicious MAX_GAME_DEPTH claim. The honest challenger will have a minimum of one CLOCK_EXTENSION (3 hours on mainnet) period on their chess clock, which cannot cover the CHALLENGE_PERIOD for an LPP (1 day on mainnet).

There are essentially 3 impacts that this can cause:

Impact 1: A malicious MAX_GAME_DEPTH claim can be uncounterable if honest challenger does not have time left.

If the honest challenger has less than 1 day left in their chess clock when reaching the malicious claim at MAX_GAME_DEPTH, they cannot possibly pass an LPP in time. This will cause the malicious claim to be uncountered while the honest challenger’s claim will be countered leading to an incorrect game result.

Impact 2: Uncounterable freeloader claims can be made by forcing an honest challenger to inherit a clock smaller than CHALLENGE_PERIOD.

The CLOCK_EXTENSION is a feature meant for honest challengers to counter a freeloader claim, a freeloader claim is described as follows:

Consider an attacker claim denoted by A in the subtree below. Suppose that this claim is valid, then the correct move for the honest party is to defend the claim. The honest party’s claim is denoted by claim H.

A / \ F H To prevent themselves from losing the bond, suppose the attacker then attacks claim A with the claim F. Due to the leftmost priority, they would receive the bond if their freeloader claim F remains uncountered. This is possible if the attacker’s chess clock has very little time left because the honest party needs to use the attacker’s chess clock to counter the freeloader claim.

To resolve this, Optimism uses clock extensions, where if a chess clock has less than CLOCK_EXTENSION seconds left, it is extended to CLOCK_EXTENSION seconds.

If the honest challenger must pass an LPP to counter the freeloader claim at MAX_GAME_DEPTH via step() it will not have enough time to do so, resulting in the freeloader claim stealing the bond from the honest party’s claim.

Impact 3: Freeloader claims can now be used to bait honest challengers to respond and then entrap them, winning all the bonds they used to counter the freeloader claim.

Combining the points raised in both impacts earlier, we can now see how freeloader claims can be used as bait to steal all the bonds used to counter the freeloader claim. The key point here is that when the honest challenger counters a freeloader claim, they essentially “swap” chess clocks with the attacker. All further moves made by the honest challenger now use the attacker’s chess clock which can be manipulated to CLOCK_EXTENSION earlier. Now, let’s take a look at the following scenario:

A / \ F1 H1 | H2 | F2 Here, the attacker has responded to the honest user’s claim H2, with their new claim F2. To counter F2, the honest challenger will still be using the attacker’s chess clock which will be manipulated to CLOCK_EXTENSION time. Let’s suppose F2 is a claim at MAX_GAME_DEPTH and to counter it the honest challenger must pass an LPP, as elaborated earlier they won’t be able to do so and they essentially cannot counter F2. But now, they also will have their bonds for posting H2 stolen from them, as F2 will now counter H2. Furthermore, the freeloader claim F1 will still steal the bond from A using the leftmost priority.

Therefore, this technique can be used attackers to entrap honest challengers, by purposefully posting an invalid state that has a valid state preceding it that requires a max size LPP to execute step() on, they can further steal bonds away from the honest challengers to counter the freeloader claim.

## Recommended Mitigation Steps

Apply a bigger extension such as CHALLENGE_PERIOD + CLOCK_EXTENSION at the MAX_GAME_DEPTH - 1 claim, as shown below.

+ IPreimageOracle oracle = VM.oracle(); + if (nextPositionDepth == MAX_GAME_DEPTH - 1 && nextDuration.raw() > MAX_CLOCK_DURATION.raw() - oracle.challengePeriod() - CLOCK_EXTENSION.raw()) { + // Apply 1 CHALLENGE_PERIOD + 1 CLOCK_EXTENSION here.

+ nextDuration = Duration.wrap(MAX_CLOCK_DURATION.raw() - oracle.challengePeriod() - CLOCK_EXTENSION.raw()); + } + else if (nextDuration.raw() > MAX_CLOCK_DURATION.raw() - CLOCK_EXTENSION.raw()) { - if (nextDuration.raw() > MAX_CLOCK_DURATION.raw() - CLOCK_EXTENSION.raw()) { // If the potential grandchild is an execution trace bisection root, double the clock extension.

uint64 extensionPeriod = nextPositionDepth == SPLIT_DEPTH - 1 ? CLOCK_EXTENSION.raw() * 2: CLOCK_EXTENSION.raw(); nextDuration = Duration.wrap(MAX_CLOCK_DURATION.raw() - extensionPeriod); } This will solve all of the scenarios above.

ajsutton (Optimism) confirmed and commented:

Impact 1 is invalid - the honest actor should be responding reasonably quickly and is expected to still have sufficient time on the clock to allow for the large preimage proposal time.

However, the impact of this on freeloader claims and the clock extension being insufficient in the case of large preimages being required is valid and something we will need to fix.

obront (judge) decreased severity to Medium and commented:

This issue is valid, but given the clarification from the sponsor that Impact 1 is invalid, I believe it’s likely that a Medium severity is more appropriate (i.e., worst case does not allow spoofing invalid state, only stealing some bonds).

I’ll also need to consider whether the dups are valid as dups (or should receive partial credit), as this report was the most clear on the other implications.

The other reports do identify the freeloader claims, which is the core issue. I will downgrade to Medium and leave the dups as-is receiving full credit.

haxatron (warden) commented:

@obront, we believe high is more appropriate for this finding. From impact 3 about entrapping the honest challenger, we note the loss of funds of the honest challenger which is very different from the loss of incentive. Here, “loss of funds” refers the funds lost by the honest challenger when posting their bonds and getting them stolen by the attacker and “loss of incentive” refers to the bonds entitled by the honest challenger for correctly countering the adversary claim.

Furthermore, the loss of fund impact is high, if we look at our example in impact 3, the honest challenger will post the bond H2 and lose their bond H2, which as MAX_GAME_DEPTH - 1 comes down to 270_000_000 * 200 gwei = 54 ETH which is by no means a small amount. This loss is also cumulative, as the honest challenger will lose all their bonds they posted to counter the freeloader claim all the way down to MAX_GAME_DEPTH.

All in all, the “loss of funds” impact to the honest challenger is high, so we also believe this should be a high.

EV_om (warden) commented:

@obront Issue #26, on the other hand, only points out impact 2 in this finding. The main difference between impacts 2 and 3 is:

Impact 2 only relates to uncounterable freeloader claims at depth MAX_GAME_DEPTH. These claims will have the effect of the attacker being able to reclaim their own bond from the parent claim, instead of it being paid out to the honest challenger for countering it. This represents a loss of incentives for honest challengers.

Impact 3 relates to uncounterable freeloader claims at lower depths, and has the effect of all bonds posted by honest challengers from a freeloader claim at any depth all the way down to MAX_GAME_DEPTH being paid out to the attacker. This represents a loss of funds for honest challengers.

Because #26 only points out impact 2 and not the higher impact 3, partial credit would seem fair.

obront (judge) increased severity to High and commented:

After much consideration, I’m in agreement with @haxatron and @EV_om here on all counts.

For consistency in judging, the other issues deemed High either allowed the VM to act in a way that would allow incorrect output roots to be proven, or would allow valid claims to be challenged in a predictable way to allow intentionally stealing a large amount of bonds. This falls into the latter category, so will be upgraded to High.

Issue #26 does capture the root cause here, and gets most of the way there, but on its own would be judged as a Medium because it doesn’t show how bonds could be stolen. I will be moving to partial credit, but because so much of the exploit was understood, I will award 75%.

Note: For full discussion, see here.

# [H-03] LPP metadata can be altered after the challenge period is over, allowing incorrect states to be proven

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-optimism-superchain
- **Source snapshot:** competitions/2024-07-optimism-superchain/final_report.html

Submitted by RadiantLabs, also found by kuprum, n4nika, alexfilippov314, and Femboys

- https://github.com/code-423n4/2024-07-optimism/blob/70556044e5e080930f686c4e5acde420104bb2c4/packages/contracts-bedrock/src/cannon/PreimageOracle.sol#L417
- https://github.com/code-423n4/2024-07-optimism/blob/70556044e5e080930f686c4e5acde420104bb2c4/packages/contracts-bedrock/src/cannon/PreimageOracle.sol#L431

## Vulnerability details

It was reported in the Spearbit review (5.2.5 “Preimage proposals can be initialized multiple times”) that it is possible to re-initialize LPPs, at a loss for the caller, because the initLPP function doesn’t check for the LPP to exist already.

If we look at the initLPP() function, however, we can see that there is another vulnerability that can be chained: at L431, the to-be-initialized LPPMetadata is unnecessarily read from storage instead of being initialized empty:

File:

PreimageOracle.

sol 417:

function initLPP ( uint256 _uuid, uint32 _partOffset, uint32 _claimedSize ) external payable { 418:

// The bond provided must be at least `MIN_BOND_SIZE`.

419:

if ( msg.

value < MIN_BOND_SIZE ) revert InsufficientBond (); 420:

421:

// The caller of `addLeavesLPP` must be an EOA, so that the call inputs are always available in block bodies.

422:

if ( msg.

sender != tx.

origin ) revert NotEOA (); 423:

424:

// The part offset must be within the bounds of the claimed size + 8.

425:

if ( _partOffset >= _claimedSize + 8 ) revert PartOffsetOOB (); 426:

427:

// The claimed size must be at least `MIN_LPP_SIZE_BYTES`.

428:

if ( _claimedSize < MIN_LPP_SIZE_BYTES ) revert InvalidInputSize (); 429:

430:

// Initialize the proposal metadata.

431:

LPPMetaData metaData = proposalMetadata [ msg.

sender ][ _uuid ]; 432:

proposalMetadata [ msg.

sender ][ _uuid ] = metaData.

setPartOffset ( _partOffset ).

setClaimedSize ( _claimedSize ); 433:

proposals.

push ( LargePreimageProposalKeys ( msg.

sender, _uuid )); 434:

435:

// Assign the bond to the proposal.

436:

proposalBonds [ msg.

sender ][ _uuid ] = msg.

value; 437: } Because of this, L431 and L432 in fact allow to arbitrarily change an existing LPP’s partOffset and claimedSize metadata, while leaving unchanged its other metadata, including its finalization timestamp if set.

Additionally, squeezeLPP() does not check that bytesProcessed == claimedSize since this check is already performed on finalization in addLeavesLPP().

By exploiting these three issues, it is then possible that:

An LPP is first created and populated using correct data.

It is finalized, and passes its challenge period.

After the challenge period is over, its claimedSize is changed to an arbitrary length by calling initLPP again.

Immediately after (without an opportunity for a challenge to happen), the LPP data is stored in preimageParts with a squeezeLPP() call along with an incorrect claimedSize stored in preimageLengths.

With the incorrect preimage length, it is possible to trick the MIPS VM to produce an incorrect state by, for example, having the preimage length be less than the part offset + length of data to read which will cause a lesser number of bytes being copied over to memory during a read syscall, thereby producing an incorrect memRoot and thus an incorrect state (demonstrated in the
## Impact

Malicious actors can trick the VM into producing an incorrect state and therefore, allow a dishonest participant to win the fault dispute game, stealing the bonds of honest participants.

## Recommended Mitigation Steps

Consider fixing the three issues exploited in the PoC:

Do not allow calling initLPP() again on proposals with non-zero proposalMetadata.

Within initLPP(), remove the unnecessary storage read at L431.

Within squeezeLPP, consider introducing a sanity check that bytesProcessed == claimedSize as on LPP finalization.

## Assessed type

Invalid Validation ajsutton (Optimism) confirmed via duplicate Issue #14

# [H-04] L2 precompile calls can be impossible to reproduce on L1

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Finding ID:** H-04
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-optimism-superchain
- **Source snapshot:** competitions/2024-07-optimism-superchain/final_report.html

Submitted by Zubat, also found by RadiantLabs When certain precompiles are invoked in an L2 transaction, the OP fault proof program substitutes their logic with “type 6” preimage oracle queries. Reviewing the op-program code shows that the ecrecover ( 0x1 ), bn256Pairing ( 0x8 ), and kzgPointEvaluation ( 0xa ) precompile addresses are currently part of this “precompile accelerators” system. For this system to function correctly, a precompile call on L2 must always be reproducible on L1 via the loadPrecompilePreimagePart() function in the PreimageOracle. This is currently not guaranteed in at least one scenario on OP mainnet.

Specifically, note that the bn256Pairing precompile has an unbounded gas cost of 34_000 * k + 45_000, where k is the input size divided by 192. This means that a user can call the bn256Pairing precompile on L2 with an input that uses nearly the entire L2 block gas limit. Currently, the block gas limit on both Ethereum mainnet and OP mainnet is 30_000_000. Since the block gas limits are the same, every precompile call on L2 is theoretically reproducible on L1 in isolation. However, since the loadPrecompilePreimagePart() function has its own overhead gas costs, it can be shown that some L2 precompile calls can be impossible to reproduce for the purposes of the PreimageOracle.

For example, consider a scenario where the following smart contract is successfully called on L2:

pragma solidity 0.8.

18; contract L2GasLimit { fallback () external { assembly { let success:= staticcall ( gas (), 0x08, 0x80, 165504, 0x0, 0x0 ) if iszero ( success ) { revert ( 0, 0 ) } } Since the input is 165_504 bytes, the cost of calling the bn256Pairing precompile is 29_353_000 gas. However, since EIP-150 limits the amount of gas the PreimageOracle can forward to 63/64 of its available gas, the loadPrecompilePreimagePart() function would require 29_353_000 * 64 / 63 = 29_818_920 gas in order to reproduce this precompile call successfully. There are two options for a user to attempt to meet this threshold:

The user calls loadPrecompilePreimagePart() from an EOA. Note that the input passed to the precompile needs to be provided as an argument:

function loadPrecompilePreimagePart ( uint256 _partOffset, address _precompile, bytes calldata _input ) external { //...

} Since the user spends 4 gas for every zero byte of calldata, this means they will have spent at least 4 * 165_504 = 662_016 gas before the function even begins. This implies the function can begin with at most 30_000_000 - 662_016 = 29_337_984 gas, which is insufficient for the precompile call alone.

Side note: there is no trick that can get around providing all 165_504 zero bytes in calldata, because Solidity does not allow calldata to point out-of-bounds.

The user calls loadPrecompilePreimagePart() using another smart contract. Since EIP-150 would also limit this contract to forwarding at most 63/64 of its available gas, and since 30_000_000 * 63 / 64 = 29_531_250, this method also cannot possibly start loadPrecompilePreimagePart() with enough gas.

As a result, this L2 precompile transaction will lead to a preimage oracle query that is impossible to satisfy on L1. This would make the corresponding step() impossible to execute, which would prevent invalid state transitions from being countered.

## Recommended Mitigation Steps

If possible, the “accelerated precompiles” system should avoid precompiles that have dynamic gas costs. Although this would imply a larger execution trace for precompile calls that would otherwise be supported, this seems to be the safest option in the long term. This is especially true if the block gas limits on L2 are ever changed to exceed the L1 block gas limit.

ajsutton (Optimism) confirmed via duplicate Issue #28 Inphi (Optimism) commented:

This is valid but out of scope as it was highlighted in the Spearbit audit. See section 5.1.1 in the audit report.

obront (judge) commented:

While it is similar, I don’t believe this is a dup of the known issue.

That issue specifically points to the more serious concern issue that the loadPrecompilePart() function can be called with ANY amount of gas, causing it to revert when it succeeded on L2. Basically, there is no connection between L2 gas for the precompile and L1.

This issue is focused on that fact that, even if such a connection existed, there are precompile calls on L2 that can not be included on L1.

Inphi (Optimism) commented:

On second thought, I agree with your assessment for the above reason.

# [H-05] An attacker can bypass the challenge period during LPP finalization

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Finding ID:** H-05
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-07-optimism-superchain
- **Source snapshot:** competitions/2024-07-optimism-superchain/final_report.html

Submitted by alexfilippov314, also found by kuprum, 0x1771, Zubat, ether_sky, and Femboys Large preimage proposals (LPP) allow submitters to prove that certain data is a fixed part of the large preimage that produces a specific Keccak-256 hash value. Since the preimage is large, the process of LPP finalization involves multiple transactions. Because the intermediate steps are not verified on-chain, LPP flow requires a challenge period during which challengers can verify the correctness of the LPP and dispute it on-chain via the challengeLPP and challengeFirstLPP functions.

The issue arises from the fact that the current implementation of the squeezeLPP function allows a malicious submitter to bypass the challenge period and finalize an invalid proposal. I’ve provided a detailed description of why it is possible below.

The squeezeLPP function checks that the challenge period is still active using this check:

if ( block.

timestamp - metaData.

timestamp () <= CHALLENGE_PERIOD ) revert ActiveProposal (); While it looks correct, the problem here is that the timestamp is not initialized in the initLPP function. If the metadata.timestamp() is zero this check will always succeed (assuming that block.timestamp > CHALLENGE_PERIOD ). The only place where metadata.timestamp is set is in the addLeavesLPP function, if the attacker provided a true value for the _finalize argument.

if ( _finalize ) { metaData = metaData.

setTimestamp ( uint64 ( block.

timestamp )); // If the number of bytes processed is not equal to the claimed size, the proposal cannot be finalized.

if ( metaData.

bytesProcessed () != metaData.

claimedSize ()) revert InvalidInputSize (); } While it may seem logical, the current flow does not require the submitter to provide a true value for the _finalize argument in any of their calls. This fact is demonstrated in the POC below. This means that the submitter can make the necessary number of addLeavesLPP calls with _finalize = false and then call squeezeLPP immediately after (since the metadata.timestamp remains uninitialized), without having to wait for the challenge period to pass. Since there is no challenge period, a malicious submitter can submit invalid proposals without the risk of being challenged.

The full attack is demonstrated in the POC below.

## Impact

This issue demonstrates that a malicious submitter can bypass the challenge period and finalize invalid LPPs. Since this data is assumed to be correct and is used in the MIPS.sol, this attack allows a malicious submitter to successfully challenge valid claims and forge invalid claims that cannot be challenged. In summary, the attack has no preconditions, can be executed by anyone, and completely breaks the fault dispute game logic. That’s why I believe the severity is HIGH.

## Recommended Mitigation Steps

Consider checking that the proposal was finalized in the squeezeLPP function:

if ( metaData.

timestamp () == 0 || block.

timestamp - metaData.

timestamp () <= CHALLENGE_PERIOD ) revert ActiveProposal ();

## Assessed type

Invalid Validation ajsutton (Optimism) confirmed Medium Risk Findings (11)

# [M-01] Multiplication overflow leading to memory corruption and incorrect register write-back

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-optimism-superchain
- **Source snapshot:** competitions/2024-07-optimism-superchain/final_report.html

Submitted by Topmark

- https://github.com/code-423n4/2024-07-optimism/blob/main/packages/contracts-bedrock/src/cannon/MIPS.sol#L967
- https://github.com/code-423n4/2024-07-optimism/blob/main/packages/contracts-bedrock/src/cannon/MIPS.sol#L763

## Impact

The unchecked multiplication of int32 values (rs and rt) and type conversion within the execute function in MIPS contract can result in an overflow, leading to the assignment of an incorrect value to the val variable in the step(…) function. This incorrect value is subsequently written to memory and the destination register in the step function. This can cause data corruption in contract and break of Proper MIPS contract functionality and storage

## Recommended Mitigation Steps

To prevent overflow, the multiplication should be checked for overflow before proceeding with the operation. One way to achieve this is by using SafeMath libraries that provide arithmetic operations with built-in overflow checks or by simply doing the validation as provided in the code below by first doing the multiplication within int64 and validating that it is below maximum of int32 before type conversion.

+++ import "@openzeppelin/contracts/utils/math/SafeMath.sol"; function execute ( uint32 insn, uint32 rs, uint32 rt, uint32 mem ) internal pure returns ( uint32 out ) {...

if ( opcode == 0x1C ) { uint32 func = insn & 0x3f; // 6-bits // mul if ( func == 0x2 ) { --- return uint32 ( int32 ( rs ) * int32 ( rt )); +++ int64 result = int64 ( int32 ( rs )) * int64 ( int32 ( rt )); +++ require ( result <= int32.

max && result >= int32.

min, "Multiplication overflow" ); +++ return uint32 ( int32 ( result )); } // clz, clo else if ( func == 0x20 || func == 0x21 ) {...

}...

}

## Assessed type

Under/Overflow clabby (Optimism) confirmed and commented:

This report is partially valid, and points out a divergence from the MIPS specification. However, the mention of disallowing overflow is incorrect, as the specification directly calls for producing a 64-bit result and retaining the low-order 32 bits of it, and never throwing an arithmetic exception.

In the MIPS specification, the MUL instruction description states:

The 32-bit word value in GPR rs is multiplied by the 32-bit value in GPR rt, treating both operands as signed values, to produce a 64-bit result.

The least significant 32 bits of the product are sign-extended and written to GPR rd. The contents of HI and LO are UNPREDICTABLE after the operation.

No arithmetic exception occurs under any circumstances.

The current implementation does not perform signed 64-bit arithmetic, casting down the result to a 32 bit value. We should not throw an exception, but we should adhere to the MUL spec.

Inphi (Optimism) commented:

Sounds like this should be downgraded to medium in that case. I see no evidence that the Go compiler emits invalid code that would otherwise cause a MIPS exception.

obront (judge) decreased severity to Medium and commented:

Since there is no evidence that the program can be impacted by this divergence from spec, will award in line with other “divergence that could cause problem but isn’t clear exactly how” and downgrade this to Medium.

3docSec (warden) commented:

@obront, in light of the fact that “wrap around on overflow” is indeed the correct behavior as per spec, we believe this report is invalid, because it points out to wrapping as root cause and as mitigation it recommends the very same operation with an added require that throws in case of overflow (which is not compliant).

To make the point that the suggested mitigation is functionally equivalent and doesn’t yield any difference in result, we investigated the difference between the following 2 implementations:

current implementation:

return uint32 ( int32 ( rs ) * int32 ( rt )); proposed implementation without the non-compliant throw:

int64 result = int64 ( int32 ( rs )) * int64 ( int32 ( rt )) return uint32 ( int32 ( result )); and found them to be functionally equivalent.

This is because execution only cares about the lower 32-bits of the result, so we don’t need to store the result in int64 and the upper 32 bits therefore can be safely ignored.

The following fuzz test demonstrates the equivalence of the original vs mitigation code:

pragma solidity ^ 0.8.

0; import "forge-std/Test.sol"; contract

# [M-02] Missing address check for instructions LH and LHU

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-optimism-superchain
- **Source snapshot:** competitions/2024-07-optimism-superchain/final_report.html

Submitted by zraxx, also found by RadiantLabs

- https://github.com/code-423n4/2024-07-optimism/blob/main/packages/contracts-bedrock/src/cannon/MIPS.sol#L990-L993
- https://github.com/code-423n4/2024-07-optimism/blob/main/packages/contracts-bedrock/src/cannon/MIPS.sol#L1008-L1011

## Impact

Instruction processing cannot detect problems in time.

Details According to this, page 99, Section Restrictions, the address must be naturally aligned. If the least-significant bit of the address is non-zero, an Address Error exception occurs. However, in the contract, the address is not checked.

Tools Used Vscode

## Recommended Mitigation Steps

Check the address and when the least-significant bit of the address is non-zero, revert.

clabby (Optimism) confirmed and commented:

This report is valid. The MIPS VM currently does not check for half-word alignment within the LH and LHU instruction implementations. Every piece of memory is guaranteed to be 4-byte aligned via the 0xFFFFFFFC mask prior to the readMem call, but the ISA specification explicitly states a requirement for 2-byte alignment for these instructions.

obront (judge) commented:

@clabby - If I’m understanding correctly, you’re saying that there is no possible risk to this in Optimism’s context, but that it is confirmed that it’s not following the MIPS spec? If that’s the case, I will plan to downgrade to low/QA.

clabby (Optimism) commented:

@obront, I can confirm that we’ve seen no adverse effects from the out-of-spec implementation of these instructions to date. Though the surface is too large to make a blanket statement on there being no possible risk. We do intend to fix this and align with the MIPS specification.

obront (judge) commented:

Because this is a confirmed divergence from the spec and we don’t have a firm guarantee that it won’t be reached in the program, I will be considering it a valid Medium.

# [M-03] Addresses can be pre-populated with bad data

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-optimism-superchain
- **Source snapshot:** competitions/2024-07-optimism-superchain/final_report.html

Submitted by bronze_pickaxe, also found by Zubat Inside PreimageOracle.sol, anyone can call the loadPrecompilePreimagePart() to load data into the contract. The problem is that anyone is able to specify the address _precompile parameter. This opens up the door for malicious users to pre-populate data and use it to validate wrong data.

# [M-04] Unvalidated memory access in readMem and writeMem functions

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-optimism-superchain
- **Source snapshot:** competitions/2024-07-optimism-superchain/final_report.html

readMem and writeMem functions Submitted by K42 The readMem() and writeMem() functions do not properly validate the memory address being accessed. This could lead to unauthorized access to memory outside the intended ranges, which could then allow a black hat to read or modify sensitive data, in creative ways. As the contract is using a 32-bit address space uint32 for addresses without implementing proper bounds checking, possibly then allowing access to the entire 4GB address space.

Specific attack vectors:

Overwriting the preimageKey and/or preimageOffset to gain unauthorized access to preimage data.

Manipulating the pc or nextPC values to alter program flows, even possible paths to bypass security checks.

Adjusting with intent the heap value to cause memory allocation issues.

Altering the exitCode or exited flags to prematurely terminate or continue execution inappropriately.

## Recommended Mitigation Steps

Add these strict bounds checking statements in readMem() and writeMem():

uint32 constant MAX_MEMORY_ADDRESS = 0x7FFFFFFF; // Define this better to prevent function readMem ( uint32 _addr, uint8 _proofIndex ) internal pure returns ( uint32 out_ ) { unchecked { require ( _addr <= MAX_MEMORY_ADDRESS, "Memory address out of bounds" ); // rest is same } function writeMem ( uint32 _addr, uint8 _proofIndex, uint32 _val ) internal pure { unchecked { require ( _addr <= MAX_MEMORY_ADDRESS, "Memory address out of bounds" ); // rest is same } Make sure to carefully choose the value for MAX_MEMORY_ADDRESS based on the specific MIPS implementation as of current and memory layout used in the contract. The constant should be set to limit the accessible memory range to only what is actually necessary, bounded specifically, to prevent gaps that can be targeted by creative black hats.

## Assessed type

Context mbaxter (Optimism) disputed and commented:

There is no sensitive data to read or write - the contracts are stateless. An attacker also has no control over the program running in the VM - that is defined by the absolute prestate. The attack vectors mentioned all require manipulating fields in the state, but this is the same as posting an invalid claim. An honest actor will just counter the invalid claim.

obront (judge) commented:

After further discussion with the sponsor, I believe this issue fits into the same category of others that have been accepted:

Potential issue in the MIPS implementation.

No known current way to exploit it.

If it was exploitable, it would be extremely bad.

No guarantees that it can’t be exploited.

While this doesn’t point to a specific deviation from the MIPS spec, it does fit the same criteria as above. For that reason, I will be upgrading the issue back to Medium (as the others of this type are) and including it in the final report.

# [M-05] Panic in MIPS VM could lead to unchallengeable L2 output root claim

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-optimism-superchain
- **Source snapshot:** competitions/2024-07-optimism-superchain/final_report.html

Submitted by Zubat, also found by Femboys and n4nika

- https://github.com/code-423n4/2024-07-optimism/blob/70556044e5e080930f686c4e5acde420104bb2c4/packages/contracts-bedrock/src/dispute/FaultDisputeGame.sol#L883-L895

## Impact

Suppose we have a situation where the MIPS geth goes wrong and always panics. The vmStatus can only be either UNFINISHED or PANIC.

In the current execution bisection game system, the UNFINISHED state can not be used as the root claim; the PANIC state can always be attacked. So we can counter every bisection game at SPLIT_DEPTH + 1; none of the claims at SPLIT_DEPTH are challenged. Inductively, claims agreeing with SPLIT_DEPTH are not challengable.

In the current configuration, SPLIT_DEPTH is set to 30, so we can deny any dispute against the root claim at depth 0. If that were to happen, the state of the L2 output root could be hijacked and unrecoverable.

It’s unclear if there are easy methods to trigger panic in the MIPS VM.

One possible way is to leverage the privilege, as the batcher is currently controlled by a centralized trusted entity. Ordinary users cannot construct the batches and channels for the rollup, but the batcher can freely set the payload blob to L1.

- https://github.com/code-423n4/2024-07-optimism/blob/main/op-node/rollup/derive/channel.go#L169-L205
A malicious batcher could compress a huge amount of zero blobs, upload them to L1, and force the MIPS VM to decompress them. Considering the limited memory resources of the MIPS VM and the absence of garbage collection in the MIPS go-ethereum, the VM could be very easily corrupted.

## Recommended Mitigation Steps

Set the split_depth to an odd number.

## Assessed type

DoS Inphi (Optimism) confirmed and commented:

This is valid. Note that an implicit assumption of the FaultDisputeGame is that the program being verified must be not contain bugs or panic unexpectedly. But we have not documented this assumption clearly, so we would still like to acknowledge this report.

Also note that there are other problems introduced by adjusting the split depth to fix this. Really fixing this will require a multi-proof architecture, to mitigate against a single faulty program taking down the system, rather than tweaks to a dispute game.

# [M-06] In some cases, proper CLOCK_EXTENTSION time cannot be ensured to generate the initial instruction trace

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-optimism-superchain
- **Source snapshot:** competitions/2024-07-optimism-superchain/final_report.html

CLOCK_EXTENTSION time cannot be ensured to generate the initial instruction trace Submitted by oakcobalt, also found by xuwinnie, RadiantLabs, Zubat, and ether_sky In some cases, a team will not be granted enough CLOCK_EXTENSION time to generate the initial instruction trace and perform a counter claim.

## Recommended Mitigation Steps

Consider change to the following:...

uint64 extensionPeriod; if ( nextPositionDepth != SPLIT_DEPTH - 1 && ( nextDuration.

raw () > MAX_CLOCK_DURATION.

raw () - CLOCK_EXTENSION.

raw ())) { extensionPeriod = CLOCK_EXTENSION.

raw (); } else if ( nextPositionDepth == SPLIT_DEPTH - 1 && ( nextDuration.

raw () > MAX_CLOCK_DURATION.

raw () - 2 * CLOCK_EXTENSION.

raw ())) { extensionPeriod = CLOCK_EXTENSION.

raw () * 2; } nextDuration = Duration.

wrap ( MAX_CLOCK_DURATION.

raw () - extensionPeriod );...

ajsutton (Optimism) confirmed and commented:

This is accurate. There was a fair bit of debate about whether 2 * CLOCK_EXTENSION was actually required at the split depth so this doesn’t concern me, though if we extend the limit for one side it seems reasonable to expect it to be extended for the other side as well.

# [M-07] MIPS - Incorrect implementation of SRAV instruction

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-optimism-superchain
- **Source snapshot:** competitions/2024-07-optimism-superchain/final_report.html

MIPS - Incorrect implementation of SRAV instruction Submitted by KupiaSec, also found by Zubat Incorrect dispute game resolution - it will make True root claim to be False.

## Recommended Mitigation Steps

As it did for other shifts, it should apply & 0x1F before shifting.

else if ( func == 0x07 ) { return SE ( rt >> ( rs & 0x1F ), 32 - ( rs - 0x1F )); }

## Assessed type

Context mbaxter (Optimism) confirmed and commented:

Looks right - originally misread the code but looks like we do need to mask rs.

Note: For full discussion, see here.

# [M-08] The LPP proposer may not be reimbursed their gas costs by the bonds at MAX_GAME_DEPTH because step() does not check if the LPP proposer is the one that called it

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Finding ID:** M-08
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-optimism-superchain
- **Source snapshot:** competitions/2024-07-optimism-superchain/final_report.html

MAX_GAME_DEPTH because step() does not check if the LPP proposer is the one that called it Submitted by RadiantLabs Part of the reason bonds exist is to refund honest challengers for the gas costs incurred from countering malicious claims. We can observe in the Optimism specs how bond prices are chosen:

FPM bonds are priced in the amount of gas that they are intended to cover. Bonds start at the very first depth of the game at a baseline of 400_000 gas. The 400_000 value is chosen as a deterrence amount that is approximately double the cost to respond at the top level. Bonds scale up to a value of 300_000_000 gas, a value chosen to cover approximately double the cost of a max-size Large Preimage Proposal.

As per to Optimism specs, we can see that at MAX_GAME_DEPTH the bond price is specifically chosen to reimburse the winning party for the high gas costs of proposing an LPP if necessary, as the proposer must call addLeavesLPP() many times. The intention is that the honest party creates an LPP and subsequently calls step() to receive the bond of the MAX_GAME_DEPTH claim they are countering.

However, the step() function is actually permissionless: it does not check if the user who proposes an LPP or loads data into the preimage oracle is the caller of step() when it accesses such data, and it always grants the bond to the caller.

This means that an attacker can call step() (and squeezeLPP() before that if necessary, as soon as the challenge period has passed) before the proposer to receive the bond, stealing their reimbursement for the high gas costs of proposing the LPP.

File:

FaultDisputeGame.

sol 234:

function step ( 235:

uint256 _claimIndex, 236:

bool _isAttack, 237:

bytes calldata _stateData, 238:

bytes calldata _proof 239: ) 240:

public 241:

virtual 242: {...

309:

// Set the parent claim as countered. We do not need to append a new claim to the game; 310:

// instead, we can just set the existing parent as countered.

311:

parent.

counteredBy = msg.

sender; 312: }

## Impact

Honest LPP proposers may not be reimbursed for the high gas costs of proposing an LPP, defeating the intended purpose of bonds and leading to misaligned incentives.

## Recommended Mitigation Steps

If a step() reads from a preimage oracle, the bonds from step() should be assigned to the one that loaded the data into the oracle.

A possible way to do this would be to set a new variable currentPreimageProposer using a preimageProposer mapping during the readPreimage call, and then read this value from the oracle to find out who to send the step() bond to:

/// @inheritdoc IFaultDisputeGame function step( uint256 _claimIndex, bool _isAttack, bytes calldata _stateData, bytes calldata _proof ) public virtual {...

+ IPreimageOracle oracle = VM.oracle(); + // Clear the previous preimage proposer if it exists.

+ oracle.clearPreimageProposer(); bool validStep = VM.step(_stateData, _proof, uuid.raw()) == postState.claim.raw(); bool parentPostAgree = (parentPos.depth() - postState.position.depth()) % 2 == 0; if (parentPostAgree == validStep) revert ValidStep(); // INVARIANT: A step cannot be made against a claim for a second time.

if (parent.counteredBy != address(0)) revert DuplicateStep(); // Set the parent claim as countered. We do not need to append a new claim to the game; // instead, we can just set the existing parent as countered.

- parent.counteredBy = msg.sender; + address preimageProposer = oracle.currentPreimageProposer(); + parent.counteredBy = preimageProposer == address(0) ? msg.sender: preimageProposer; } + function clearPreimageProposer() external { + currentPreimageProposer = address(0); + } - function readPreimage(bytes32 _key, uint256 _offset) external view returns (bytes32 dat_, uint256 datLen_) { + function readPreimage(bytes32 _key, uint256 _offset) external returns (bytes32 dat_, uint256 datLen_) {...

+ currentPreimageProposer = preimageProposer[_key][_offset]; } obront (judge) commented:

squeezeLPP() pays out to the claimant, not caller, and that’s the reward for doing addLeavesLPP(), so it is correct.

haxatron (warden) commented:

@obront, I believe there is a misunderstanding in your comment here.

squeezeLPP pays out to the claimant, not caller, and that’s the reward for doing addLeavesLPP Here, the bonds we are referring to are the FDG bonds, not the LPP bonds. To pass an LPP the proposer will stake the bonds in the initLPP and after the challenge period, squeezeLPP can be called to pay back the bond to the proposer.

However, the gas required for a maximum size LPP proposal is extremely large ( 150_000_000 ), and that is why the FDG bonds at MAX_GAME_DEPTH are 300_000_000 * 200 gwei in order to reimburse the gas cost of proposing a maximum size LPP, as we already pointed out:

FPM bonds are priced in the amount of gas that they are intended to cover. Bonds start at the very first depth of the game at a baseline of 400_000 gas. The 400_000 value is chosen as a deterrence amount that is approximately double the cost to respond at the top level. Bonds scale up to a value of 300_000_000 gas, a value chosen to cover approximately double the cost of a max-size Large Preimage Proposal.

Here, we show that an attacker can call squeezeLPP, and then call step(), which will cause the FDG bond to go to the attacker and the proposer will not be reimbursed.

obront (judge) commented:

@clabby - Can you weigh in? It seems from the code like the intention is that the LPP bonds should cover the LPP costs, and the FDG bonds should focus on FDG itself.

But the warden points out that the docs explain that FDG bonds are based on LPP cost.

Can you provide some more context?

clabby (Optimism) commented:

@obront - The LPP bond offers an incentive to challenge the LPP; The incentive for creating the LPP itself is encapsulated by the incentive within the FDG, as creating the LPP may be required for performing a successful step.

@haxatron is correct here that another party can take advantage of someone else’s work in the large preimage proposal in order to receive the bond over in the FDG. You could do this by waiting until a valid large preimage proposal for the step has been finalized and is ready to be squeezed, and then finally using it to call step (where the LPP proposer would not be the one that received the incentive over in the FDG).

There is not an expectation within the system that the LPP submitter will always be the one that actually receives the reward; it does also hinge upon them being the first party to call step using their LPP.

haxatron (warden) commented:

Whether this is intentional or not, the LPP submitter will have to spend 150 million gas to propose a max size LPP and as @clabby has mentioned - “The incentive for creating the LPP itself is encapsulated by the incentive within the FDG”.

But the problem is that anyone, even the attacker himself, can steal this incentive by calling squeezeLPP() after 1 day challenge period and then call step() to receive the FDG bond. Meanwhile, the LPP submitter will not be reimbursed the 150 million gas cost of creating the LPP.

obront (judge) commented:

@haxatron, I’ve thought about this a lot more and I’m in agreement. If the FDG bonds are intended to cover the LPP, this system is clearly not reliable.

Given there is no way for anyone to create an LPP with confidence they’ll get the reward, this is a fundamental problem that a user cannot protect themselves against.

xuwinnie (warden) commented:

@obront, let me explain why I think the system is still reliable.

The reliability of the system is ensured not only by incentivizing the honest party to act, but also by disincentivizing the evil party to act.

This issue can be simplified to, honest actor spend X ETH but may not get the bond (2X ETH), and the evil party could lose the bond (2X ETH). The safety is guaranteed by “evil party’s loss will be larger than honest party’s loss”, so if the ratio (2) is large enough, we can consider the system as safe.

Let’s use top level move as another example. Evil party can risk to lose 2Y ETH to post an invalid root. But an honest actor’s move could also be frontrunned (even they send the tx privately they still cannot guarantee it to be the first), and they could lose the gas they spent (Y ETH). It is unavoidable that a malicious party can grief an honest party, causing them to lose money. So I’d rather call this a design tradeoff rather than a security issue.

obront (judge) commented:

@xuwinnie - I see your point, but the unavoidable downside to challengers makes this more than a design trade off, in my opinion. I’m going to stick with the decision to reward as a Medium.

Optimism confirmed

# [M-09] Honest party’s move could become invalid when re-org takes place

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Finding ID:** M-09
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-optimism-superchain
- **Source snapshot:** competitions/2024-07-optimism-superchain/final_report.html

Submitted by xuwinnie

- https://github.com/code-423n4/2024-07-optimism/blob/70556044e5e080930f686c4e5acde420104bb2c4/packages/contracts-bedrock/src/dispute/DisputeGameFactory.sol#L116
- https://github.com/Vectorized/solady/blob/a95f6714868cfe5d590145f936d0661bddff40d2/src/utils/LibClone.sol#L458
- https://github.com/code-423n4/2024-07-optimism/blob/70556044e5e080930f686c4e5acde420104bb2c4/packages/contracts-bedrock/src/dispute/FaultDisputeGame.sol#L319

## Impact

When block re-org takes place, honest party’s move could become invalid. A similar issue has been raised earlier, and this report shows two new scenarios, which the fix fails to address.

## Recommended Mitigation Steps

Use cloneDeterministic instead.

claimHash(child) = keccak256(claim(child), position(child), claimHash(parent)); Then check claimHash is indeed the expected one.

## Assessed type

Context Inphi (Optimism) confirmed and commented:

This is valid. I’ll note that, by default, the op-node trails behind L1 by 5 blocks. So a challenger using it as its source of outputs wouldn’t accidentally move against the wrong claim/game.

EV_om (warden) commented:

@obront for context, as I’m sure you’re aware, the decision to accept the referenced finding as a valid Med the the below reasoning was controversial.

This was initially deemed invalid by a strict interpretation of our judging guidelines (“Chain re-org and network liveness related issues are not considered valid.”). This rule exists as the “blockchain is trusted” from the perspective of app builders. However, a different trust level applies when building an L1/L2.

In our opinion, this is not the case here because this code better fits the definition of an “app on L1” than “L2 core contracts on L1”.

For this reason, we decided to include a reorg finding of ours in the QA report instead of submitting it as a separate finding, which we would like to ask to be assessed with the same severity as this (be it Med or Low). I believe it provides sufficient context despite its brevity, but let us know if you’d like us to provide more detail.

obront (judge) commented:

I am going to keep the originally judged severities here.

The QA report mentioned by EV_om is identical to xuwinnie issue #10, which was downgraded to QA. This issue is different, as it points to something that could happen within the game, as opposed to just in the creation. The abilities for games to swap addresses (intentionally or not) and have moves on them persist on the wrong game is separate and more important.

While EV_om is right that the Sherlock decision was controversial, that is because Sherlock’s rules explicitly rule out reorg issues, while this is not the case on C4.

# [M-10] The MIPS doesn’t implement ADD , ADDI , and SUB instructions correctly

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Finding ID:** M-10
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-optimism-superchain
- **Source snapshot:** competitions/2024-07-optimism-superchain/final_report.html

MIPS doesn’t implement ADD, ADDI, and SUB instructions correctly Submitted by alexfilippov314, also found by RadiantLabs and OMEN

- https://github.com/code-423n4/2024-07-optimism/blob/70556044e5e080930f686c4e5acde420104bb2c4/packages/contracts-bedrock/src/cannon/MIPS.sol#L921
- https://github.com/code-423n4/2024-07-optimism/blob/70556044e5e080930f686c4e5acde420104bb2c4/packages/contracts-bedrock/src/cannon/MIPS.sol#L929

## Vulnerability details

According to the specification, ADD (page A-28), ADDI (page A-29), and SUB (page A-144) instructions should raise an Integer Overflow exception if overflow occurs. The current implementation simply wraps the result in such cases and does not raise any exceptions....

function execute ( uint32 insn, uint32 rs, uint32 rt, uint32 mem ) internal pure returns ( uint32 out ) { unchecked {...

else if ( func == 0x20 ) { return ( rs + rt ); }...

else if ( func == 0x22 ) { return ( rs - rt ); }...

}...

This inconsistency leads to a situation where MIPS contract can’t correctly emulate such cases and therefore allows malicious actors to successfully forge invalid claims and challenge valid claims.

## Impact

An inconsistent implementation of big-endian 32-bit MIPS32 architecture in the MIPS contract allows malicious actors to successfully forge invalid claims and challenge valid claims.

## Recommended Mitigation Steps

Consider raising Integer Overflow exception for ADD, ADDI, and SUB instructions if overflow occurs.

## Assessed type

Math clabby (Optimism) confirmed and commented:

This report is valid, and it is a duplicate of 3.3.4 from the Cantina report on the MIPS VM.

KupiaSec (warden) commented:

@obront - op-program ran means there was no overflow exception happened in runtime, so the issue is not valid.

rileyholterhus (warden) commented:

Without commenting on individual issue validity or how things should be grouped, I would like to point out that the following issues are currently judged as medium-severity:

#89 #85 #82 #41 The above issues all attempt to point out deviations from the MIPS spec that may or may not be reachable in op-program. I think this issue is another example of this, but it is currently high-severity.

obront (judge) decreased severity to Medium and commented:

The original thinking when judging this issue was that it could be presently exploited (whereas the other issues @rileyholterhus mentioned exposed deviations from the spec, but that we don’t currently have a way to exploit).

After some more digging and experimentation, it appears that there isn’t a current way to trigger this issue, as the overflow would revert when op-program is running.

This puts it in the same bucket as the other issues mentioned:

It is a deviation from spec.

If we could trigger it, we could get on chain MIPS to provide invalid values.

This could be exploited in incredibly harmful ways.

But presently, there is no known way to trigger it (and it shouldn’t be possible, given that op-program ran).

All issues that fit this criteria will be awarded as Medium, so this will be downgraded to align with that.

# [M-11] Attacker can continuously create games for not yet safe l2 blocks to prevent the update of anchor state

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Finding ID:** M-11
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-07-optimism-superchain
- **Source snapshot:** competitions/2024-07-optimism-superchain/final_report.html

Submitted by xuwinnie, also found by 0x1771 When creating a dispute game, the output root should be one of the safe l2 block’s. However, attacker can continuously create games for not yet safe l2 blocks. Honest party can no longer propose the root after it becomes safe, since UUID must be unique. As a result, no root can be successfully defended, the anchor state can no longer be updated and the fund on L2 will be frozen.

## Assessed type

Context ajsutton (Optimism) acknowledged and commented:

The report is correct; however, the claimed loss is dramatically overstated since it is not expected that the entire TVL of op-mainnet and base are trying to be withdrawn. Additionally, the attacker would have to ensure they create a proposal for every unsafe block before they become safe - if a single block is missed all pending withdrawals could be proven against the proposal for that block.

The attacker would also be relying on the batcher submitting transactions that exactly match the unsafe blocks. While this is typically the case and is intended in the smooth operation of a chain, it is not guaranteed. If the actual safe block did differ from the attacker’s proposed root the actual root could still be used for a proposal. For any in progress games, the attacker would be at risk of losing the bond for both their root claim and their counter claim, as the value specified in the counter claim would now be invalid, allowing an honest actor to counter it and post their own counter to the root claim.

The attacker would also lose money in gas costs for creating each game (~421k gas each), posting the counter claim to avoid losing their bond (~231k gas each) and then reclaiming their bonds after the lock up period.

obront (judge) decreased severity to Medium and commented:

I respect the sponsor’s perspective here that this issue is unlikely to become a serious concern, but after carefully considering it, I’ve decided that the ability to create failing games for valid output roots that will block the correct game from being created seems to be a valid issue. I will be downgrading to Medium and accepting.

EV_om (warden) commented:

the ability to create failing games for valid output roots that will block the correct game from being created seems to be a valid issue I completely agree with the above and I think this is a really nice finding.

However, the severity should be assessed according to the highest realistic impact and this finding fails to provide one that would qualify it for Medium severity.

Let’s examine the costs of this attack. As @ajsutton mentioned, the costs are not limited to the opportunity cost of the significant capital required, but also include the gas cost of creating and resolving claims for every single block, in particular for:

creating each game (~421k gas each), posting the counter claim to avoid losing their bond (~231k gas each) and then reclaiming their bonds after the lock up period.

The latter requires:

2 calls to resolveClaim() (~111k gas each).

1 call to resolve() (~39k gas).

1 call to claimCredit() (~55k gas).

All of this put together leads to a total of ~857k gas per censored block, which at an ETH price of $3k, a reasonable gas price of 20 gwei and a 2s L2 block time amount to a cost of $2,2 million per DAY that the attack is ongoing in addition to the already mentioned capital requirements and opportunity loss.

This together with the fact that the attack can be easily mitigated by migrating to a new game type means the likelihood that this is exploited (to no benefit for the attacker) and actually impacts the availability of the protocol is essentially non-existent. (Edit: just a quick note that migrating to a new game is not one of the safeguards that should be considered nonexistent for this audit) The issue was also brought up here in the previous audit (specific attack vector of the L1 block being too early is mentioned “e.g., the L1 parent block is too early” as well as the inability to create another game, without the DOS claims).

xuwinnie (warden) commented:

@EV_om The impact is critical, not low. How could you call “freeze whole l2 fund for one year” low?

Gas price has dropped after Cancun and will continue to drop. It should not be used as an argument. OP TVL will grow to 1 trillion and the attack cost will eventually be negligible.

Attacker can short $OP to take profit, panic would be huge.

In my report, neither L1 nor L2 block is too early.

The report you mentioned misses two key points. It does not point of proposed block should be safe It does not utilize create game for every block to DOS. Basically, it just says there could be something wrong without describing how it could actually be wrong.

For the sake of this audit, you should pretend the safeguards don’t exist.

Under the context of this audit, the max damage could be permanently freezing the l2 fund. So I believe this is a high severity issue @obront The attacker would also be relying on the batcher submitting transactions that exactly match the unsafe blocks.

Even they don’t match, attacker can still monitor mempool and frontrun batcher’s the transaction.

obront (judge) commented:

While I agree with @EV_om that the cost to the attacker is large and doesn’t provide obvious benefit (such that I don’t think DOS’ing the chain for a year is at all likely), I do believe a Medium severity is justified.

As @xuwinnie mentioned, gas prices have dropped a lot. I don’t think it’s unreasonable to think that 2 gwei will often be a better estimate than 20 gwei.

At least to start, every OP Mainnet block doesn’t need to be proven. Proposals are created once per hour (

- https://etherscan.io/address/0xe5965Ab5962eDc7477C8520243A95517CD252fA9
), and if we read the op-proposer code (

- https://github.com/ethereum-optimism/optimism/blob/d283e9be6e3ff9294c61634bda0131c373ecaaf8/op-proposer/proposer/driver.go#L471-L492
) we can see that it doesn’t matter whether past proposals landed. It just tries at regular interval from config, which is set to 1 hour. Until OP team changed the config and relaunched the proposer, it would only take a couple txs per hour to make sure it was blocked.

We can’t predict situations where delaying ability to withdraw for some period of time would be harmful.

I agree that this is unlikely and benefit is unclear enough that it shouldn’t be a High, but I think Medium is warranted and the code should be corrected, so will keep judging as-is.

ajsutton (Optimism) commented:

I’m staying well out of the debate around severity levels but I think it’s worth clarifying:

At least to start, every OP Mainnet block doesn’t need to be proven. Proposals are created once per hour.

Proposals are permissionless, so any user whose withdrawal is blocked can propose for any L2 block (which could then be used for any pending withdrawals). So for this attack to be at all effective you need to create a game for every L2 block, not just the block that winds up being the next safe head. Any user who would be harmed by a delay to their withdrawal would also be incentivised to make their own proposal.

obront (judge) commented:

@ajsutton - That’s fair. I still believe Medium is the right classification, but appreciate the correction.

## Rejected Primary Findings

# Rejected Primary Findings: Optimism Superchain

# Attacker can reverse chess clock by attacking a defense

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-73
- **Submitter:** Bac0nj
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/73
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-73.md

## Brief Summary

Attack and Defense for a claim both use the same clock. Attacker can take advantage of this to reverse chess clock. Impact Malicious attacker can gain more time while shorten the others', make the game resolved incorrectly. Vulnerability Detail 1. Here's the chain of attack: `0(root claimer) <- 1(attacker) <- 2(root claimer) <- 3(attacker) <- 4(root claimer)`, 2. When the time to challenge 2 is about to end, honest player Bob come to support root by creating a defense 5 to 2. 3. Attacker then attack Bob's defense 5, thus reverse the chess clock.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Create a game with big MAX_GAME_DEPTH could cost too much to win

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-74
- **Submitter:** Bac0nj
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/74
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-74.md

## Brief Summary

If a game is created with big MAX_GAME_DEPTH and attacker keep attacking, it would cost too much for a honest player to win the game. Vulnerability Detail For a honest party to win a game, player should keep attacking malicious players when they make a claim. If the MAX_GAME_DEPTH is too big, player should put lots of bonds to get to the final step.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Current method of hashing the leaves make it vulnerable to the second preimage attack

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-54
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/54
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-54.md

## Brief Summary

Preimage proposals are susceptible to the second preimage attacks

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# `FaultDisputeGame` claims could be unresolvable in some edge cases

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-62
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/62
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-62.md

## Brief Summary

An unwanted state would be reached where we would have an unresolvable claim.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_10_group

# The `getChallengerDuration` function lacks handling the case when the parent indexer does not exist

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-66
- **Submitter:** DanielTan_MetaTrust
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/66
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-66.md

## Brief Summary

The vulnerability exists in the `getChallengerDuration` function, which calculates the remaining time for a challenger to respond to a claim based on the parent’s clock. The specific issue arises when the parent indexer does not exist, and the function uses the default value of `type(uint32).max`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_08_group

# Immutable ABSOLUTE_PRESTATE forces frequent redeployments and allows proving invalid withdrawals

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-109
- **Submitter:** Dup1337
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/109
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-109.md

## Brief Summary

Immutable ABSOLUTE_PRESTATE forces freqent redeployments and allows proving invalid withdrawals in case of block number since `ABSOLUTE_PRESTATE` is bigger than `1 << SPLIT_INDEX`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Redundancy

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-58
- **Submitter:** Kavin
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/58
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-58.md

## Brief Summary

There is redundancy in the client initialization process where the same URL is used to initialize multiple types of clients (ethclient).

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unrestricted Access to resolveClaim Function Allows Anyone to Modify Status Value

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-25
- **Submitter:** Mrxstrange
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/25
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-25.md

## Brief Summary

* The `resolveClaim()` function initially checks the status. If it is not in the `IN_PROGRESS` stage, the function call will revert. However, the status value can be changed by anyone simply by calling the `resolve()` function, as there are no access control modifiers or user requirement checks. This lack of control can affect the contract's function process.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_12_group

# L1 re-orgs could cause `Position` to be different, resulting in `move()` being called on a different node

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-42
- **Submitter:** RI_trollers
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/42
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-42.md

## Brief Summary

`FaultDisputeGame.sol`'s `move()` requires participants to provide the index(in `claimData`) of the parent claim. After that, it checks that `Claim _disputed` provided by the user is equals to the `parent.claim` which was introduced after another re-org related bug was found in the previous optimism sherlock audit. However, <ins>the function fails to check that the `Position` of the node remains the same</ins>, which makes it susceptible to a different re-org attack. Proof of code Let's consider an oridinary binary tree. <ins>**Sequence Walkthrough:**<ins> It can be assumed in the code that `Alice` is the honest challenger while `Miner` is the malicious actor. 1. `Alice` disagrees with the...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Incorrect Bit Manipulation in `setCountered` Function

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-121
- **Submitter:** Sparrow
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/121
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-121.md

## Brief Summary

The `setCountered` function in the `CannonTypes.sol` does not correctly clear the relevant bits before setting the countered flag. The other setter functions (`setTimestamp`, `setPartOffset`, etc.) follow a consistent pattern of first clearing the target bits and then setting the new value. This pattern ensures that the new value is correctly applied without residual bits from previous values. The `setCountered` function should follow the same pattern to maintain consistency and correctness. It also employs a mask designed for a 64-bit segment instead of targeting the intended single bit. The operation attempts to extract the entire 64-bit segment, instead of isolating the 255th bit, which...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Loop Condition in `findlatestgames`

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-122
- **Submitter:** Sparrow
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/122
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-122.md

## Brief Summary

The vulnerability in the `findLatestGames` function stems from an incorrect loop condition that prevents the function from iterating over the intended range of indices in the `_disputeGameList`. Specifically, the loop is designed to search backward from the index `_start` down to 0, but the condition `i >= 0 && i <= _start` ensures that the loop either does not execute at all (if `_start` is not 0) or executes just once (if `_start` is 0) The loop condition consists of two parts combined with a logical AND operator (`&&`): - `i >= 0`: This condition checks if the current index `i` is greater than or equal to 0. Since `i` is initialized to `_start`, and `_start` is presumably a positive inte...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# A defense can be made against the root claim of the execution trace bisection subgames

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-123
- **Submitter:** Udsen
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/123
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-123.md

## Brief Summary

The `FaultDisputeGame.move` function is used as the generic move function for both `attack` and `defend` moves. In the function execution there is following `Invariant` which is secured. INVARIANT: A defense can never be made against the root claim of either the output root game or any of the execution trace bisection subgames. The implementation of the above `Invariant` is given as follows: As per the `invariant` it is stated that the `defense` can never be made against the root claim of the execution trace bisection subgames. The root claim of the execution trace subgames is the `SPLIT_DEPTH + 1` which is one level below the `SPLIT_DEPTH`, but in the above logic implementation of the inva...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Re-Entrancy issue in claimCredit Function

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-114
- **Submitter:** XDZIBECX
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/114
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-114.md

## Brief Summary

The claimCredit function is to allow users to claim their credited balance from the contract. The function is correctly sets the user's credit balance to 0 before making an external call to the WETH contract to withdraw the corresponding amount. the attacker can re-enter the claimCredit function during the withdrawal process and claim additional credit before the initial call completes cause there is a miss of a re-entrancy guard the function. the function is sets the credit balance to 0 before making the external call, it does not prevent re-entrancy during the WETH.withdraw call. Impact the issue can lead to loss of funds , and it's allowing an attacker to drain the entire balance of the...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_18_group

# clone can be manupilated

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-129
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/129
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-129.md

## Brief Summary

Detailed description of the impact of this finding. clone can be manupliated as we are taking all the input from the user.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Attacker can challenge a game with correct root and win by spoofing one of the local preimages

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-119
- **Submitter:** niroh
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/119
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-119.md

## Brief Summary

When the game reaches MAX_DEPTH the last claim in the game can be challanged with a step. If it's an honest challanger's turn in the game, they submit a step transaction with state data that matches the prestate commitment. If the state data indicates that the step requires a local preimage, they need to also call addLocalData before they call Step, so that the data is available for the Mips Step function. There is however a scenario where a malicious user can cause a game with a valid root to fail (while gaining all bonds) using a loophole in the preimage system:

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_17_group

# Inverted Logic in resolveClaim Function May Cause Unresolved Disputes and Financial Losses

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-115
- **Submitter:** obingo76
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/115
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-115.md

## Brief Summary

The line in question is intended to prevent the resolution of a subgame that has already been resolved. However, the logic appears inverted, which could lead to incorrect behavior. Potential Issues Inverted Logic: The condition checks if the subgame is not resolved This should instead check if the subgame is resolved. The revert message ``ClaimAlreadyResolved()`` is misleading because the condition checks for unresolved subgames. Impact of this bug can prevent the correct resolution of subgames, leading to unresolved disputes and potential loss of funds or incorrect game outcomes.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# The bisection root should be at SPLIT_DEPTH + 1, not SPLIT_DEPTH - 1

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-59
- **Submitter:** peanuts
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/59
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-59.md

## Brief Summary

The bisection root should be at SPLIT_DEPTH + 1, not SPLIT_DEPTH - 1

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_11_group

# A Fault Dispute Game can be played solo.

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-61
- **Submitter:** peanuts
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/61
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-61.md

## Brief Summary

The purpose of a back-and-forth is not established.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# A claimant with a valid claim will lose their bond and will be considered if there is a valid defence but no attack against their claim

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-116
- **Submitter:** silver_eth
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/116
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-116.md

## Brief Summary

A valid claim could be considered invalid

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_21_group

# Unchecked External Calls

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-86
- **Submitter:** soffije
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/86
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-86.md

## Brief Summary

**Unchecked External Call in `claimCredit`**: - The external call to `WETH.withdraw` in the `claimCredit` function is not checked for success. **Unchecked External Call**: - The contract makes an unchecked external call to `ANCHOR_STATE_REGISTRY.tryUpdateAnchorState()` which may fail silently. - **Mitigation**: Ensure the success of the external call using proper error handling mechanisms. Suggested Improvements **External Call Validation**:

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_28_group

# Blockhash Function Limitation

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-88
- **Submitter:** soffije
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/88
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-88.md

## Brief Summary

The `blockhash` function only provides access to block hashes of the most recent 256 blocks. **Impact:** If the dispute game creation relies on a block hash older than 256 blocks, the `blockhash` function will return zero, leading to potential failures or incorrect initializations. **Solution:** Add a check to ensure that the parent block hash is not zero.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Constructor Misuse

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-89
- **Submitter:** soffije
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/89
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-89.md

## Brief Summary

The constructor calls the `initialize` function with `address(0)`, potentially leaving the contract in an uninitialized state. **Impact:** If the contract is deployed without proper initialization, it may be vulnerable to unauthorized access or other unintended behaviors. **Solution:** Separate the constructor logic from the initialization logic, ensuring the constructor disables initializers and proper initialization is handled separately.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_32_group

# Incorrect Use of Assembly for Array Management

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-90
- **Submitter:** soffije
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/90
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-90.md

## Brief Summary

**Incorrect Use of Assembly for Array Management** **Issue:** The use of inline assembly for array manipulation in `findLatestGames` is complex and prone to errors. **Impact:** Improper use of assembly can lead to unexpected behavior, memory corruption, or inefficient gas usage. **Solution:** Use native Solidity constructs to handle dynamic arrays, making the code more readable and safer.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Input Validation

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-91
- **Submitter:** soffije
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/91
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-91.md

## Brief Summary

Functions like `setImplementation` and `setInitBond` do not validate the provided `_gameType` and `_impl`. **Impact:** If invalid inputs are provided, it could lead to incorrect state updates and unintended behavior. **Solution:** Add checks to validate the inputs before performing state updates.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_29_group

# `load**` functions don't check key exists and don't have access control

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-46
- **Submitter:** twcctop
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/46
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-46.md

## Brief Summary

anyone can call `load**` functions and modify the data, the fucntion `readPreimage` may get wrong data.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# the proof data of function `squeezeLPP` can be used to call `challengeLPP`

- **Contest:** Optimism Superchain
- **Slug:** 2024-07-optimism-superchain
- **Submission:** V-50
- **Submitter:** twcctop
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-07-optimism-validation/issues/50
- **Source snapshot:** competitions/2024-07-optimism-superchain/submissions/raw/V-50.md

## Brief Summary

when try to finalize prososal, malicious user is possible to front run get proof data and call `challengeLPP` to get profit

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_39_group
