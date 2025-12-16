[H-1] Unoptimized subset matches counting implementation will exceed tx gas limit on base chain
Jackpot.sol

Impact: High
**Status**: We Did NOT find this
WHY YOU MISSED THIS
1. Different Analysis Focus
What you found:

✅ Bit-packing overflow (H-2, M-5)
✅ Price desync issues (H-16)
✅ Fee parameter issues (M-18)
✅ Payout calculator swap (M-13)
What you missed:

❌ Computational complexity analysis of settlement functions
❌ Gas profiling under realistic high-value scenarios
❌ Chain-specific gas limits (Base = 25M gas)
2. Missing Gas Profiling
You likely didn't:

❌ Run gas benchmarks on _countSubsetMatches with high bonusballMax
❌ Test settlement with realistic large prize pools ($16M+)
❌ Profile nested loop complexity: O(bonusballMax * normalTiers * combinations)
❌ Check against Base chain's specific 25M gas limit
3. Overlooked Nested Loop Complexity
The vulnerable pattern:

Complexity: O(bonusballMax * normalTiers * C(normalBallMax, tier))

With bonusballMax=129, normalBallMax=30, normalTiers=5:

Outer loop: 129 iterations
Inner loop: 5 iterations per outer
generateSubsets: Combinatorial explosion (C(30,5) = 142,506 combinations)
Total: MASSIVE gas consumption
4. Didn't Test Edge Cases
You likely tested:

✅ Normal scenarios (bonusballMax = 5-10)
✅ Moderate prize pools ($100K - $1M)
You didn't test:

❌ Extreme growth scenarios (bonusballMax = 129)
❌ Large prize pools ($16M+)
❌ Chain-specific limits (Base's 25M gas)
🎯 HOW TO CATCH THIS IN FUTURE AUDITS
1. Always Perform Gas Profiling
2. Identify Nested Loop Patterns
Red flags to look for:

Questions to ask:

What's the maximum value of loop counters?
What's the complexity of operations inside loops?
Can this exceed block gas limit?
3. Test Extreme Scenarios
Create specific test cases:

4. Analyze Computational Complexity
For each critical function:

Identify loops: How many iterations?
Identify expensive operations: Combinatorics, external calls, storage writes
Calculate worst-case complexity: O(n²)? O(n³)? O(2^n)?
Estimate gas: Multiply iterations × operation cost
Example analysis:

5. Check Chain-Specific Constraints
Different chains have different limits:

Chain	Gas Limit	Notes
Ethereum	30M	Higher limit
Base	25M	Lower limit (this protocol's target)
Arbitrum	32M	Higher limit
Optimism	30M	Same as Ethereum
Always verify:

📋 UPDATED AUDIT CHECKLIST
Add these items to your future audits:

Gas Analysis Section:
Profile gas for all settlement/callback functions
Test with MAXIMUM realistic parameters (not just typical)
Identify all nested loops and analyze complexity
Check against target chain's specific gas limit
Test extreme scenarios (large prize pools, high bonusballMax)
Look for combinatorial explosions (factorials, combinations, permutations)
Verify unbounded loops have reasonable upper bounds
Specific Patterns to Flag:
Nested loops with dynamic/large counters
Combinatorial operations (generateSubsets, permutations)
Unbounded iterations in callbacks
External calls inside loops
Storage writes inside nested loops
S-210

Finding description and impact
During a drawing settlement, there is a very expensive calculation that counts all subset matches:

The stacktrace is displayed here:

File: 2025-11-megapot/contracts/Jackpot.sol

717:     function scaledEntropyCallback(
718:         bytes32,
719:         uint256[][] memory _randomNumbers,
720:         bytes memory
721:     )
722:         external
723:         nonReentrant
724:         onlyEntropy
725:     {
... // @audit trace 1
732:@>       (uint256 winningNumbers, uint256 drawingUserWinnings) = _calculateDrawingUserWinnings(currentDrawingState, _randomNumbers);
...
1614:     function _calculateDrawingUserWinnings(
1615:         DrawingState storage _currentDrawingState,
1616:         uint256[][] memory _unPackedWinningNumbers
1617:     )
1618:         internal
1619:         returns(uint256 winningNumbers, uint256 drawingUserWinnings)
1620:     {
1621:         // Note that the total amount of winning tickets for a given tier is the sum of result and dupResult
1622:         (
1623:             uint256 winningTicket,
1624:             uint256[] memory uniqueResult,
1625:             uint256[] memory dupResult
// @audit trace 2
1626:@>       ) = TicketComboTracker.countTierMatchesWithBonusball(drawingEntries[currentDrawingId],
1627:             _unPackedWinningNumbers[0].toUint8Array(),      // normal balls
1628:             _unPackedWinningNumbers[1][0].toUint8()         // bonusball
1629:         );
File: 2025-11-megapot/contracts/lib/TicketComboTracker.sol

250:     function countTierMatchesWithBonusball(
251:         Tracker storage _tracker,
252:         uint8[] memory _normalBalls,
253:         uint8 _bonusball
254:     )
255:         internal
256:         view
257:         returns (uint256 winningTicket, uint256[] memory uniqueResult, uint256[] memory dupResult)
258:     {
...// @audit trace 3
263:@>       (uint256[] memory matches, uint256[] memory dupMatches) = _countSubsetMatches(_tracker, set, _bonusball);
...
145:     function _countSubsetMatches(
146:         Tracker storage _tracker,
147:         uint256 _normalBallsBitVector,
148:         uint8 _bonusball
149:     )
150:         private
151:         view
152:         returns (uint256[] memory matches, uint256[] memory dupMatches)
153:     {
154:         matches = new uint256[]((_tracker.normalTiers+1)*2);
155:         dupMatches = new uint256[]((_tracker.normalTiers+1)*2);
156:// @audit trace 4: the final culprit         
157:@>       for (uint8 i = 1; i <= _tracker.bonusballMax; i++) {
158:@>           for (uint8 k = 1; k <= _tracker.normalTiers; k++) {
159:@>               uint256[] memory subsets = Combinations.generateSubsets(_normalBallsBitVector, k);
We're generating subsets of _normalBallsBitVector for bonusballMax * 5 times.

For sufficiently high bonusballMax, the gas limit will exceed tx gas limit of 25M on base chain.

For example, as we'll see in the POC, in the following configuration:

normallBallMax: 30
poolCap: 16Me6 USDC (worth of 16M USD)
bonusBallMax: 129
Gas consumption is estimated to be 25,834,562

Impact

Due to tx gas limit violation , Pyth network's entropy provider will not be able to invoke the callback
As a result, drawing can never be settled




S-731

[H-2] Attacker can steal `JackpotTicketNFT`'s from `JackpotBridgeManager.sol`


Impact: High
**Status**: NOT FOUND
💭 LESSONS LEARNED
Why was this missed?

Focus on accounting bugs: Your audit focused heavily on price desync, fee issues, bit-packing
Missed arbitrary call pattern: The _bridgeFunds function performs an arbitrary external call with user-controlled data
Insufficient validation: Only USDC balance is checked, not NFT custody
How to catch this in future:

✅ Always flag arbitrary external calls (.call() with user data)
✅ Check ALL asset types (not just USDC - also NFTs!)
✅ Validate invariants AFTER external calls (NFT custody should be checked)

Finding Description
The JackpotBridgeManager contract facilitates cross-chain ticket purchases and winnings claims for the Jackpot system. It acts as a custodian for NFTs representing tickets that are purchased from other chains. However, NFTs held by JackpotBridgeManager can be stolen due to an unsafe external call pattern.

Cross-chain users purchase tickets through the JackpotBridgeManager::buyTickets function. This function interacts with the Jackpot contract to mint tickets (NFTs), which are held in custody by JackpotBridgeManager. The contract internally tracks ownership of these NFTs to ensure users from different chains can later claim their winnings.

After the Jackpot draw concludes, users can claim their winnings by calling JackpotBridgeManager::claimWinnings. This function retrieves the claimed winnings from the Jackpot contract, then bridges the funds to the destination chain via the _bridgeFunds function.

The vulnerability arises in the _bridgeFunds function:

function _bridgeFunds(RelayTxData memory _bridgeDetails, uint256 _claimedAmount) private {
    if (_bridgeDetails.approveTo != address(0)) {
        usdc.approve(_bridgeDetails.approveTo, _claimedAmount);
    }

    uint256 preUSDCBalance = usdc.balanceOf(address(this));
    (bool success,) = _bridgeDetails.to.call(_bridgeDetails.data);

    if (!success) revert BridgeFundsFailed();
    uint256 postUSDCBalance = usdc.balanceOf(address(this));

    if (preUSDCBalance - postUSDCBalance != _claimedAmount) revert NotAllFundsBridged();

    emit FundsBridged(_bridgeDetails.to, _claimedAmount);
}
The _bridgeFunds function performs an external call to _bridgeDetails.to, which is user-controlled. This allows an attacker to craft arbitrary call data that executes malicious logic. By leveraging this external call, an attacker can manipulate contract state and steal NFTs held by JackpotBridgeManager.

Exploitation Scenario
The attacker purchases two tickets via JackpotBridgeManager::buyTickets.

Assume the JackpotBridgeManager is already holding multiple NFTs on behalf of legitimate cross-chain users.

After the jackpot draw, the attacker has some legitimate winning tickets but identifies a winning NFT held on behalf of a victim.

The attacker crafts a malicious claimWinnings transaction as follows:

_userTicketIds: Attacker’s own ticket IDs.
_bridgeDetails:
approveTo: Address of an attacker-controlled exploit contract.
to: Address of the jackpotNFT contract.
data: Encoded call data for safeTransferFrom(address from, address to, uint256 tokenId, bytes data),
transferring the victim’s NFT from JackpotBridgeManager to the exploit contract.
Attack Flow
The attacker calls claimWinnings, causing JackpotBridgeManager to approve the attacker’s contract for _claimedAmount.
The _bridgeFunds function then executes an external call to the jackpotNFT contract using attacker-supplied data.
This triggers safeTransferFrom, transferring the victim’s NFT to the attacker's exploit contract.
During the transfer, the onERC721Received function in the exploit contract executes, which immediately pulls the approved USDC from JackpotBridgeManager, ensuring the USDC balance decreases by exactly _claimedAmount.
As a result, the post-call balance check
if (preUSDCBalance - postUSDCBalance != _claimedAmount) revert NotAllFundsBridged();
passes successfully, allowing the transaction to complete without reverting.
The victim’s NFT is now transferred to the attacker’s contract, resulting in loss of user assets.
Recommended mitigation steps
Validate RelayTxData before performing the external call or perform external call only on whitelisted addresses.



[H-3] Sum of bonusballMax and normalBallMax Can Exceed 255, Causing Systemic Bit-Shift Failures
Jackpot.sol

**Status**: FOUND BUT Classified as dup of V12 incorrectly!!

Finding description and impact
The LP value accumulates from the value of tickets sold. When the current drawing is finalized, the new LP value for the subsequent drawing can become excessively large. This leads to a high bonusballMax value for the new drawing, creating a risk that bonusballMax + normalBallMax > 255.

Jackpot.sol#L1494-L1497:

        uint256 newPrizePool = _newLpValue * (PRECISE_UNIT - reserveRatio) / PRECISE_UNIT;

        // ... additional contract logic

        uint256 combosPerBonusball = Combinations.choose(normalBallMax, NORMAL_BALL_COUNT);
        uint256 minNumberTickets = newPrizePool * PRECISE_UNIT / ((PRECISE_UNIT - lpEdgeTarget) * ticketPrice);
        uint8 newBonusball = uint8(Math.max(bonusballMin, Math.ceilDiv(minNumberTickets, combosPerBonusball)));
        newDrawingState.bonusballMax = newBonusball;
The calculation of newBonusball lacks an upper bound constraint. When the LP value is too high, the computed value may exceed the maximum limit of a uint8(255).

Example Scenario:

normalBallMax = 30
reserveRatio = 20%
lpEdgeTarget = 30%
ticketPrice = 1 USDC
_newLpValue = 28,200,000 USDC
In this case, newBonusball would calculate to 227.

When a user purchases a ticket, if the sum of the chosen _bonusball and _tracker.normalMax exceeds 255, the bit-shift operation 1 << (_bonusball + _tracker.normalMax) will revert. This occurs because the shift amount exceeds the bit length of a uint256, preventing the transaction from succeeding and blocking ticket purchases.

Jackpot.sol#L1590:

            (uint256 packedTicket, bool isDup) = TicketComboTracker.insert(currentDrawingEntries, ticket.normals, _tickets[i].bonusball);
TicketComboTracker.sol#L141-L142:

        // Add the bonusball to the bit vector
        ticketNumbers = set |= 1 << (_bonusball + _tracker.normalMax);
During the entropy provider's callback with random numbers, if the calculated winning _bonusball plus _tracker.normalMax exceeds 255, the same bit-shift overflow will occur. This causes the entire drawing settlement process to fail irrecoverably, halting the system's core functionality and representing the most severe consequence.

Jackpot.sol#L1622-L1629:

        (
            uint256 winningTicket,
            uint256[] memory uniqueResult,
            uint256[] memory dupResult
        ) = TicketComboTracker.countTierMatchesWithBonusball(drawingEntries[currentDrawingId],
            _unPackedWinningNumbers[0].toUint8Array(),      // normal balls
            _unPackedWinningNumbers[1][0].toUint8()         // bonusball
        );
TicketComboTracker.sol#L260:

        winningTicket = set | (1 << (_bonusball + _tracker.normalMax));
Recommended mitigation steps
It is recommended to implement an upper bound check when setting newDrawingState.bonusballMax (e.g., uint8(Math.min(calculatedValue, 255 - normalBallMax))) to ensure the long-term stability and operability of the system.




[M-4] If bonus ball max equals normal ball max then ticket buyers gain excessive edge
Jackpot.sol

**Status**: Did NOT find this

💭 LESSONS LEARNED: Why This Was Missed
1. Different Analysis Domain
Your Focus:

✅ Integer overflow/underflow
✅ Accounting invariants
✅ Access control
✅ State corruption
This Finding Requires:

❌ Cryptographic analysis (seed reuse)
❌ Probability theory (statistical advantage)
❌ Game theory (optimal player strategy)
❌ Randomness quality (deterministic shuffle properties)
2. Missed Seed Reuse Pattern
The vulnerable code:

You should have asked:

❓ What happens when normalBallMax == bonusballMax?
❓ Does Fisher-Yates use the same seed for both draws?
❓ Are the draws independent?
3. No Probabilistic Analysis
You didn't:

❌ Calculate expected value for players
❌ Model optimal player strategy
❌ Analyze LP edge under different parameter combinations
❌ Test randomness quality/independence
4. Focused on "Overflow" Not "Equality"
Pattern recognition bias:

You found normalBallMax + bonusballMax > 255 (overflow) everywhere
You anchored on this pattern
You missed the simpler case: normalBallMax == bonusballMax (equality)
🎓 HOW TO CATCH THIS IN FUTURE
1. Add Randomness Quality Analysis
Checklist:

Are random seeds reused across multiple draws?
Are draws from the same seed independent?
Can players exploit deterministic properties of the RNG?
What happens when ranges overlap or are identical?
2. Perform Game Theory Analysis
Questions to ask:

What's the optimal strategy for a rational player?
Can players gain positive expected value?
What's the LP edge under different parameter combinations?
Are there parameter combinations that break the economic model?
3. Test Edge Cases for Equality
Don't just test overflow:

4. Analyze Cryptographic Primitives
For any randomness generation:

Identify all uses of the same seed
Check if draws are truly independent
Verify nonce/salt is used to separate draws
Test deterministic properties (same input → same output)
5. Calculate Expected Values
For lottery/gambling protocols:

📋 UPDATED AUDIT CHECKLIST
Add these items for lottery/randomness protocols:

Randomness Quality Section:
Identify all RNG calls and their seeds
Check for seed reuse across multiple draws
Verify draws are independent (different nonces/salts)
Test what happens when ranges are identical
Analyze deterministic properties of shuffle algorithms
Game Theory Section:
Calculate player expected value for all parameter combinations
Model optimal player strategy
Verify LP edge is maintained under all scenarios
Test economic model with edge cases (equal ranges, minimal ranges, etc.)
Specific Patterns to Flag:
Same seed used for multiple draws
Identical ranges in Fisher-Yates or similar algorithms
Missing nonce/salt to separate draws
Deterministic shuffle with predictable properties
✅ FINAL VERDICT
Status: ❌ NOT FOUND in any of your audit reports (r1, r2, r3)

This is a CRITICAL MISS! 🚨

Why it's different from your findings:

Different root cause: Seed reuse (not bit-pack overflow)
Different condition: normalBallMax == bonusballMax (not > 255)
Different impact: Statistical advantage (not DoS)
Different analysis: Probabilistic/game theory (not integer overflow)
This finding requires a COMPLETELY DIFFERENT analysis approach that you didn't apply in your audit.

Finding description and impact
The random number selection is generated using a common seed for both the normal balls and the bonus ball. It is possible that normalBallMax is equal to bonusBallMax because bonusBallMax is calculated using this formula below (abbreviated version of Jackpot.sol:1494-1496):

newBonusball = max(
  bonusBallMin,
  (prizePool / (1 - lpEdgeTarget)) / choose(normalBallMax, 5)
)
If prizePool is increased sufficiently then it's possible for the above formula to reach normalBallMax. The sponsor indicated via Q&A that the typical starting range for normalBallMax is 30 to 35 and it may be increased/decreased depending on pool size. Therefore under normal circumstances, if the admin is always monitoring the pool size, then it would be rare (although not impossible) for prizePool to increase enough such that normalBallMax becomes equal to bonusBallMax. We can't discount this possibility as there is nothing to garantuee the admin is constantly monitoring the situation.

Secondly, there is another way in which the prize pool could be forcibly increased by an attacker: they could deliberately buy many of the same ticket (likely to have minimal winnings) which will add to the new LP value during the scaledEntropyCallback:

    function scaledEntropyCallback(
        bytes32,
        uint256[][] memory _randomNumbers,
        bytes memory
    )
        external
        nonReentrant
        onlyEntropy
    {
        // [...]
        (
            uint256 newLpValue,
            uint256 newAccumulatorValue
        ) = jackpotLPManager.processDrawingSettlement(
            currentDrawingId,
            currentDrawingState.lpEarnings,
            drawingUserWinnings,
            protocolFeeAmount
        );

        _setNewDrawingState(newLpValue, currentDrawingState.drawingTime + drawingDurationInSeconds);

        // [...]
    }

    function _setNewDrawingState(uint256 _newLpValue, uint256 _nextDrawingTime) internal {
        // [ ... ]

        uint256 combosPerBonusball = Combinations.choose(normalBallMax, NORMAL_BALL_COUNT);
        uint256 minNumberTickets = newPrizePool * PRECISE_UNIT / ((PRECISE_UNIT - lpEdgeTarget) * ticketPrice);
        uint8 newBonusball = uint8(Math.max(bonusballMin, Math.ceilDiv(minNumberTickets, combosPerBonusball)));
        newDrawingState.bonusballMax = newBonusball;
        
        TicketComboTracker.init(drawingEntries[currentDrawingId], normalBallMax, newBonusball, NORMAL_BALL_COUNT);

        // [ ... ]
    }
Again, the admin could in theory increase the normalBallMax prior to scaledEntropyCallback being called, and this would prevent the undesired situation where normalBallMax = bonusBallMax. However, it seems very unlikely they would be closely monitoring the actual tickets bought to figure the expected payout for LPs in order to prevent this from occurring. The attacker will obviously incur significant cost in doing this but as shown below they gain a large statistical advantage from which a profit could potentially be made during the next drawing.

In this situation where normalBallMax = bonusBallMax = N, the selection process in FisherYatesRejection.draw will always produce the same shuffle of the numbers in pool (the set from 1 to N):

    function draw(
        uint256 minRange,
        uint256 maxRange,
        uint256 count,
        uint256 seed
    ) external pure returns (uint256[] memory result) {
        require(count <= maxRange - minRange + 1, "Too many draws");

        // Build pool [1, 2, ..., range]
        uint256 rangeSize = maxRange - minRange + 1;
        uint256[] memory pool = new uint256[](rangeSize);
        for (uint256 i = 0; i < rangeSize; i++) {
            pool[i] = i + minRange;
        }

        uint256 nonce = 0;

        // Fisher-Yates shuffle with rejection sampling
        for (uint256 i = rangeSize - 1; i > 0; i--) {
            uint256 rand;
            while (true) {
                rand = uint256(keccak256(abi.encode(seed, nonce)));
                uint256 limit = (MAX_UINT / (i + 1)) * (i + 1);

                if (rand < limit) {
                    rand = rand % (i + 1);
                    break;
                }
                nonce++;
            }

            // Swap pool[i] and pool[rand]
            (pool[i], pool[rand]) = (pool[rand], pool[i]);
            nonce++;
        }

        // Take first `count` numbers
        result = new uint256[](count);
        for (uint256 j = 0; j < count; j++) {
            result[j] = pool[j];
        }
    }
The only difference in arguments to this function for the two draws is that for the normal balls the first 5 elements of pool are returned whereas for the bonus ball only the first element is returned. Therefore in this case a gambler knows that for any draw the bonus ball will be contained within the normal ball set.

This issues gives a statistical advantage for the gambler at the expense of LPs. Let's denote the probability of the gambler correctly guessing k out of the 5 normal balls as P(k). This is unchanged by the advantage compared to normal conditions. However, the probability of them getting a bonus ball match is higher. Normally this probability should be P(k) * (1/N), as there should be an independent, 1 in N chance of getting the bonus ball. By always using one of their 5 normal ball picks as the bonus ball pick, the gambler increased the chance of a bonus ball match to:

P(k) * (k/5) * (1/5) = P(k) * k / 25
This is because they have a k in 5 chance of selecting the bonus ball within their set of k correct guesses, and then a 1 in 5 chance of picking the right one as the bonus ball out of the 5 normal they've chosen.

Their probability of bonus ball match therefore goes up by a factor of k * N / 25 for each tier level k. Plugging in the numbers when N = 30, we get:

k	Factor by which chance of bonus ball match increases
1	1.2
2	2.4
3	3.6
4	4.8
5	6
 
Conclusion: the gambler gets significant statistical advantage. It is very possible that their expected value is positive from playing the game. Impact: financial loss to LPs.

Recommended mitigation steps
Consider providing separate random seeds in each draw done within ScaledEntropyProvider:

--- a/contracts/ScaledEntropyProvider.sol
+++ b/contracts/ScaledEntropyProvider.sol
@@ -258,54 +258,55 @@ contract ScaledEntropyProvider is Ownable, IScaledEntropyProvider, IEntropyConsu
 
     function _getScaledRandomness(
         bytes32 _randomNumber,
         SetRequest[] memory _setRequests
     )
         internal
         pure
         returns (uint256[][] memory requestsOutputs)
     {
         requestsOutputs = new uint256[][](_setRequests.length);
         
         for (uint256 i = 0; i < _setRequests.length; i++) {
             if (!_setRequests[i].withReplacement) {
                 requestsOutputs[i] = FisherYatesRejection.draw(
                     _setRequests[i].minRange,
                     _setRequests[i].maxRange,
                     _setRequests[i].samples,
                     uint256(_randomNumber)
                 );
             } else {
                 requestsOutputs[i] = _drawWithReplacement(
                     _setRequests[i].minRange,
                     _setRequests[i].maxRange,
                     _setRequests[i].samples,
                     uint256(_randomNumber)
                 );
             }
+            _randomNumber = keccak256(abi.encode(_randomNumber));
         }
     }


[M-5] Global Variable Manipulation During Active Draw Alters End Result

**STATUS:** We found it!
📋 LESSONS LEARNED
How to Catch ALL Parameter Manipulation Issues:
1. Systematic State Audit

2. Trace Settlement Flow

3. Test Matrix

✅ FINAL VERDICT
Status: ✅ PARTIALLY FOUND (75% coverage)

What you found:

✅ payoutCalculator manipulation (M-13, M-8, M-7, M-12)
✅ referralFee manipulation (M-18, M-6, M-8)
✅ ticketPrice manipulation (H-16, H-4)
What you missed:

❌ protocolFee manipulation
❌ entropy provider manipulation
❌ jackpotLPManager manipulation
Root Cause: Same as what you found (global variables not snapshotted per-drawing)

Your findings are VALID and VALUABLE, but you didn't achieve complete coverage of all mutable parameters. The external finding is more comprehensive in identifying ALL affected parameters.
Medium

Finding description and impact
The core issue lies in the ability of the owner to modify global configuration variables during an active jackpot draw. These parameters directly influence jackpot settlement logic, fee distribution, payout calculation, and even randomness handling.
Because these values are read during settlement (after tickets have been purchased but before the draw is finalized), changing them mid-round allows the owner to unfairly alter the outcome of the draw or cause settlement failures.

Specifically, the following global variables can be updated during an active draw:

protocolFee
referralFee
payoutCalculator
entropy (entropy provider)
jackpotLPManager
Each of these variables can alter the draw’s behavior or payout path:

protocolFee / referralFee — allow manipulation of fee distribution to reduce/increse rewards to players.
payoutCalculator — can redirect or alter payout logic to arbitrary addresses.
entropy — can manipulate randomness or prevent valid settlement.
jackpotLPManager — can revert settlements or redirect LP-related funds.
Impact:
This undermines jackpot integrity, enabling admin-based manipulation of winnings, payout denial, or DoS of settlement — a severe trust and fairness violation affecting all players.

Recommended mitigation steps
Restrict all configuration-changing functions (those modifying global variables like the above) to be callable only when no active draw is in progress.
Introduce a locking mechanism that freezes sensitive parameters once a draw is initialized (initializeJackpot() called) until settlement completes.


[M-6] Deliberately increasing liquidity can DoS updates to the protocol’s governance parameters.
JackpotLPManager.sol

Impact: Medium

**Status:**: Not Found!

This is a CRITICAL MISS! 🚨

Why it's different from your findings:

Different root cause: Governance DoS (not parameter manipulation)
Different attack vector: LP frontrunning (not admin changes)
Different impact: Governance blocked (not settlement corruption)
Different analysis: Incentive analysis (not data flow analysis)
Different attacker: Rational LP (not malicious admin)
This finding requires a COMPLETELY DIFFERENT analysis approach that you didn't apply in your audit:

❌ Governance function failure mode analysis
❌ Frontrunning attack analysis
❌ LP incentive alignment analysis
❌ Economic griefing attack analysis


Finding description and impact
The JackpotLPManager::setLPPoolCap function sets the lpPoolCap for LPs.
However, if the current lpPool + pendingDeposits exceeds the desired new cap, the transaction reverts.

It is trivial for LPs to increase lpPool + pendingDeposits simply by making deposits, effectively blocking the cap update.

    function processDeposit(uint256 _drawingId, address _lpAddress, uint256 _amount) external onlyJackpot() {
        // Note: this check also prevents users from depositing before initializeLPDeposits() is called since the pool cap will be 0
        // We will exclude pending withdrawals since the amount withdrawn is dependent on the post-drawing LP value. This makes this
        // check more conservative.
        uint256 totalPoolValue = lpDrawingState[_drawingId].lpPoolTotal + lpDrawingState[_drawingId].pendingDeposits;
        if (_amount + totalPoolValue > lpPoolCap) revert JackpotErrors.ExceedsPoolCap();

        LP storage lp = lpInfo[_lpAddress];

        _consolidateDeposits(lp, _drawingId);

        lp.lastDeposit.amount += _amount;
        lp.lastDeposit.drawingId = _drawingId;

        lpDrawingState[_drawingId].pendingDeposits += _amount;

        emit LpDeposited(_lpAddress, _drawingId, _amount, lpDrawingState[_drawingId].pendingDeposits);
    }

    function setLPPoolCap(uint256 _drawingId, uint256 _lpPoolCap) external onlyJackpot() {
        LPDrawingState storage currentLP = lpDrawingState[_drawingId];
        if (_lpPoolCap < currentLP.lpPoolTotal + currentLP.pendingDeposits) revert InvalidLPPoolCap();
        lpPoolCap = _lpPoolCap;
    }
The call to setLPPoolCap is triggered when updating governance parameters in the Jackpot contract.

function setNormalBallMax(uint8 _normalBallMax) external onlyOwner {
        // Note: we do not need to check if _normalBallMax is greater than 255 because it is enforced by uint8 type
        uint8 oldNormalBallMax = normalBallMax;
        jackpotLPManager.setLPPoolCap(currentDrawingId, _calculateLpPoolCap(_normalBallMax));
        normalBallMax = _normalBallMax;
        
        emit NormalBallMaxUpdated(currentDrawingId, oldNormalBallMax, _normalBallMax);
    }

function setGovernancePoolCap(uint256 _governancePoolCap) external onlyOwner {
        if (_governancePoolCap == 0) revert JackpotErrors.InvalidGovernancePoolCap();

        uint256 oldGovernancePoolCap = governancePoolCap;
        governancePoolCap = _governancePoolCap;
        jackpotLPManager.setLPPoolCap(currentDrawingId, _calculateLpPoolCap(normalBallMax));
        
        emit GovernancePoolCapUpdated(currentDrawingId, oldGovernancePoolCap, _governancePoolCap);
    }

function setLpEdgeTarget(uint256 _lpEdgeTarget) external onlyOwner {
        if (_lpEdgeTarget == 0 || _lpEdgeTarget >= PRECISE_UNIT) revert JackpotErrors.InvalidLpEdgeTarget();
        uint256 oldLpEdgeTarget = lpEdgeTarget;
        lpEdgeTarget = _lpEdgeTarget;

        jackpotLPManager.setLPPoolCap(currentDrawingId, _calculateLpPoolCap(normalBallMax));
        
        emit LpEdgeTargetUpdated(currentDrawingId, oldLpEdgeTarget, _lpEdgeTarget);
    }

function setReserveRatio(uint256 _reserveRatio) external onlyOwner {
        if (_reserveRatio >= PRECISE_UNIT) revert JackpotErrors.InvalidReserveRatio();
        uint256 oldReserveRatio = reserveRatio;
        reserveRatio = _reserveRatio;

        jackpotLPManager.setLPPoolCap(currentDrawingId, _calculateLpPoolCap(normalBallMax));
        
        emit ReserveRatioUpdated(currentDrawingId, oldReserveRatio, _reserveRatio);
    }

function setTicketPrice(uint256 _ticketPrice) external onlyOwner {
        if (_ticketPrice == 0) revert JackpotErrors.InvalidTicketPrice();
        uint256 oldTicketPrice = ticketPrice;
        ticketPrice = _ticketPrice;
        jackpotLPManager.setLPPoolCap(currentDrawingId, _calculateLpPoolCap(normalBallMax));
        
        emit TicketPriceUpdated(currentDrawingId, oldTicketPrice, _ticketPrice);
    }

function _calculateLpPoolCap(uint256 _normalBallMax) internal view returns (uint256) {
        // We use MAX_BIT_VECTOR_SIZE because that's the max number that can be packed in a uint256 bit vector
        uint256 maxAllowableTickets = Combinations.choose(_normalBallMax, NORMAL_BALL_COUNT) * (MAX_BIT_VECTOR_SIZE - _normalBallMax);
        uint256 maxPrizePool = maxAllowableTickets * ticketPrice * (PRECISE_UNIT - lpEdgeTarget) / PRECISE_UNIT;

        // We need to make sure that the lpPoolCap is not greater than the governance pool cap
        return Math.min(maxPrizePool * PRECISE_UNIT / (PRECISE_UNIT - reserveRatio), governancePoolCap);
    }
From the formula in _calculateLpPoolCap, it is clear which parameter changes reduce lpPoolCap:

Decreasing governancePoolCap
Decreasing reserveRatio
Decreasing ticketPrice
Increasing normalBallMax
Decreasing lpEdgeTarget
Each of these changes can be disadvantageous to LPs for various reasons.
The most obvious examples:

Lowering ticketPrice reduces LP earnings per ticket.
Lowering lpEdgeTarget reduces the guaranteed LP share from each drawing.
Therefore, LP providers have a clear incentive to DoS governance parameter updates that would reduce lpPoolCap.

To DoS such parameter changes, an LP only needs to frontrun the governance update with a deposit transaction.
The deposit must be large enough so that the new lpPoolTotal exceeds the value allowed by the updated parameters.
In that case, the update cannot take effect in the current drawing and will be postponed to the next one.

Practically, this means that after a successful DoS, the governance changes can only be applied after two drawings, not the current one.

It is also important to note that this attack introduces no additional risk to the LP provider.
They are simply depositing liquidity as usual, which means their risk exposure remains exactly the same as before.

Given that this is an easy DoS of the governance functionality for an undefined period of time, medium severity is appropriate.

Recommended mitigation steps
Make governance parameter updates less dependent on LP behavior.




[M-7] Changes to Pyth entropy provider used by `ScaledEntropyProvider` allow attacker to fix jackpot result
Jackpot.sol

Severity: Medium (SOLO!)

✅ FINAL VERDICT
Status: ❌ COMPLETELY MISSED - This is a CRITICAL SOLO FINDING

Severity: 🚨 HIGH (Complete jackpot theft)

Why you missed it:

Didn't understand Pyth Entropy architecture - Sequence numbers are per-provider
Didn't analyze storage collision scenarios - pending[sequence] lacks provider context
Didn't consider partial state overwrites - Mixed overwrite/append behavior
Didn't analyze cross-transaction attacks - Multi-step sequence manipulation
Didn't audit configuration change impacts - Provider change creates collision window
This finding requires:

✅ Deep understanding of external protocol (Pyth)
✅ Storage collision analysis
✅ Solidity storage behavior expertise
✅ Multi-step attack sequencing
✅ Economic feasibility analysis
This is one of the most sophisticated findings possible - it combines:

External protocol knowledge
Storage collision vulnerabilities
Callback manipulation
Multi-transaction attack coordination
Economic incentive analysis



Finding description and impact
When the Jackpot requests entropy from the ScaledEntropyProvider during Jackpot::runJackpot, the ScaledEntropyProvider tracks each request by the sequence number returned from the Pyth Network Entropy contract:

    function requestAndCallbackScaledRandomness(
        uint32 _gasLimit,
        SetRequest[] memory _requests,
        bytes4 _selector,
        bytes memory _context
    )
        external
        payable
        returns (uint64 sequence)
    {
        // We assume that the caller has already checked that the fee is sufficient
        if (msg.value < getFee(_gasLimit)) revert InsufficientFee();
        if (_selector == bytes4(0)) revert InvalidSelector();
        _validateRequests(_requests);

        sequence = entropy.requestV2{value: msg.value}(entropyProvider, _gasLimit);
        _storePendingRequest(sequence, _selector, _context, _requests);
    }

    // [...]

    function _storePendingRequest(
        uint64 sequence,
        bytes4 _selector,
        bytes memory _context,
        SetRequest[] memory _setRequests
    ) internal {
        pending[sequence].callback = msg.sender;
        pending[sequence].selector = _selector;
        pending[sequence].context = _context;
        for (uint256 i = 0; i < _setRequests.length; i++) {
            pending[sequence].setRequests.push(_setRequests[i]);
        }
    }
The entropyProvider storage variable used above is the Pyth entropy provider. For each Pyth entropy provider the sequence number is a unique value (incremented with each requestV2 call). The problem is that different Pyth entropy providers may share the same sequence number at some point. We can see the sequence numbers are tracked per provider address by the Pyth Entropy contract here.

Consider this scenario:

Attacker observes from the mempool that the owner is about to call ScaledEntropyProvider::setEntropyProvider to change Pyth entropy provider to a new address.
Attacker front-runs the admin by calling ScaledEntropyProvider::requestAndCallbackScaledRandomness which registers their callback at s, the current sequence number. They provide _requests of length 2 where the first element specifies 5 samples with minRange = 1 and maxRange = 5 - without replacement (this will always produce the same selection of all numbers 1 to 5). The second element - for the bonus ball - can have minRange = maxRange = 1 so the result is always pre-determined to be 1. The attacker can use any account/contract with callback that reverts, there by in case ScaledEntropyProvider::_entropyCallback is executed for their callback, the storage value pending[s] is never cleared due to the revert on ScaledEntropyProvider.sol:253.
Admin's call to ScaledEntropyProvider::setEntropyProvider is executed. Let's assume the current sequence number of the new Pyth entropy provider is less than s.
Attacker buys one or more lottery tickets with numbers to match the desired outcome from step 2.
Attacker directly calls Entropy::requestV2 for the new Pyth entropy provider until its sequence number reaches s - 1.
In the same transaction as the previous step, the attacker calls Jackpot::runJackpot which will cause pending[s] to be modified: callback, selector and context are over-written to the values required by the Jackpot. Requests will be appended onto the end of pending[s].setRequests, but the attacker's original requests are left as-is.
New Pyth entropy provider will call Entropy::reveal which causes Jackpot::scaledEntropyCallback to be executed and only the attacker's desired "random" numbers will be used (as they are at indices 0 and 1 in the _randomNumbers array).
Attacker will have the winning ticket and can claim their winnings.
Impact: attacker forces the outcome of the jackpot and claims the winning ticket at the expense of honest users and LPs.

Notes on attack feasibility:

If in the case the new entropy provider has higher sequence number that the old one, it is possible for the attacker to front run the admin change and directly call Entropy::requestV2 several times for the old provider until its sequence number exceeds that of the new provider.

As at the time of writing this submission the sequence number for the default provider of the Entropy contract on Base mainnet is in the order of a few hundred thousand. If the difference between the old and new provider sequence numbers are at this order of magnitude, there by requiring the attacker call Entropy::requestV2 about this many times, then this does incur a significant cost. However, if we consider the gas price of a layer 2 like Base and the potential earnings the attacker can make from the lottery win, the attack is still feasible. The attacker could split the calls up across different transactions/blocks as necessary. Additionally, if the new provider has lower sequence number than the old one, the attacker could just wait until the sequence number catches up due to normal use of the Pyth network.

Conclusion: any time the admin changes the Pyth entropy provider they put the protocol at significant risk of being exploited.

Recommended mitigation steps
Consider changing the ScaledEntropyProvider to store requests based on sequence number and entropy provider. E.g. use a nested mapping:

--- a/contracts/ScaledEntropyProvider.sol
+++ b/contracts/ScaledEntropyProvider.sol
@@ -68,7 +68,7 @@ contract ScaledEntropyProvider is Ownable, IScaledEntropyProvider, IEntropyConsu
 
     IEntropyV2 private entropy;
     address private entropyProvider;
-    mapping(uint64 => PendingRequest) private pending;
+    mapping(address => mapping(uint64 => PendingRequest)) private pending;


[M-8] `lpEarnings` generated in emergency mode become stuck on the contract
Jackpot.sol

**STATUS:** 
💡 WHY YOU MISSED THIS
1. Didn't Analyze Emergency Mode Comprehensively
What you found:

✅ Emergency refunds use wrong referralFee (M-18, M-6, M-8)
✅ Emergency refunds don't unwind tracker (M-7, M-19)
What you missed:

❌ Which functions CAN run during emergency mode?
❌ Which functions CANNOT run during emergency mode?
❌ What happens to state modified by allowed functions?
2. Didn't Map Modifier Coverage
Missing analysis:

Red flag: claimWinnings has NEITHER modifier!

3. Didn't Trace State Modifications in Emergency Mode
Missing analysis:

4. Didn't Consider "Unrecoverable Emergency Mode"
Your mental model:

Emergency mode is temporary
Protocol can exit emergency mode
State can be cleaned up later
Reality (per README):

Emergency mode is UNRECOVERABLE
Protocol NEVER exits emergency mode
Current drawing NEVER settles
Any state depending on settlement is PERMANENTLY STUCK



Finding description and impact
As stated in the contest README, emergency mode is an unrecoverable state.
This means the protocol does not intend to exit emergency mode once it is activated.

This means that once the jackpot enters emergency mode, the current drawing will not be completed, since runJackpot is protected by the noEmergencyMode modifier.

function runJackpot() external payable nonReentrant noEmergencyMode {
This means that the lpEarnings for the current drawing will not be included in the accumulator update (cause there will be no upgrade) and will therefore remain stuck in the protocol.

LP earnings originate from two sources:

Ticket sales
Referral win shares distributed when claiming rewards for tickets without referrers
Ticket sales are not counted during emergency mode, since users receive their funds back through emergencyRefundTickets.

However, referral win shares can still be generated in any drawing

function _payReferrersWinnings(
        bytes32 _referralSchemeId,
        uint256 _winningAmount,
        uint256 _referralWinShare
    )         internal
        returns (uint256)
{
...
uint256 referrerShare = _winningAmount * _referralWinShare / PRECISE_UNIT;
        // If referrer scheme is empty then the referrer share goes to LPs so we just add the amount to lpEarnings
        // in order to make sure our system accounts for it
        if (_referralSchemeId == bytes32(0)) {
            drawingState[currentDrawingId].lpEarnings += referrerShare;
            emit LpEarningsUpdated(currentDrawingId, referrerShare);
            return referrerShare;
        }
...
}
Let’s look more closely at the claimWinnings call.
A user can invoke this function to claim rewards from any previous drawing (even on emergency mode).
If a ticket has no referrers, the referral win share (a percentage of the prize) is credited as lpEarnings for the current drawing, as shown in the code.

function claimWinnings(uint256[] memory _userTicketIds) external nonReentrant {
        if (_userTicketIds.length == 0) revert JackpotErrors.NoTicketsToClaim();
        uint256 totalClaimAmount = 0;
        for (uint256 i = 0; i < _userTicketIds.length; i++) {
            uint256 ticketId = _userTicketIds[i];
            IJackpotTicketNFT.TrackedTicket memory ticketInfo = jackpotNFT.getTicketInfo(ticketId);
            uint256 drawingId = ticketInfo.drawingId;
            if (IERC721(address(jackpotNFT)).ownerOf(ticketId) != msg.sender) revert JackpotErrors.NotTicketOwner();
            if (drawingId >= currentDrawingId) revert JackpotErrors.TicketFromFutureDrawing();

            DrawingState memory winningDrawingState = drawingState[drawingId];
            uint256 tierId = _calculateTicketTierId(ticketInfo.packedTicket, winningDrawingState.winningTicket, winningDrawingState.ballMax);
            jackpotNFT.burnTicket(ticketId);
            
            uint256 winningAmount = payoutCalculator.getTierPayout(drawingId, tierId);
            uint256 referrerShare = _payReferrersWinnings( // @audit lp earnings distributions here
                ticketInfo.referralScheme,
                winningAmount,
                winningDrawingState.referralWinShare
            );
            
            totalClaimAmount += winningAmount - referrerShare;
            emit TicketWinningsClaimed(
                msg.sender,
                drawingId,
                ticketId,
                tierId / 2,             // matches
                (tierId % 2) == 1,      // bonusball match
                winningAmount - referrerShare
            );
        }

        usdc.safeTransfer(msg.sender, totalClaimAmount);
    }
Thus, during emergency mode, the claimWinnings function continues generating lpEarnings, but these amounts will never be accounted for going forward.
They simply remain stuck in the protocol.

Recommended mitigation steps
Add a dedicated function that allows withdrawing the stuck funds while the protocol is in emergency mode.



[M-9] Incorrect ticket price reference in JackpotBridgeManager causes user overpayment after price updates

**STatus:** Not Found!

Incorrect ticket price reference in JackpotBridgeManager causes user overpayment after price updates
Avatar for avoloder
avoloder

84

Finding description and impact
In the Jackpot.sol contract, several parameters define each drawing (such as ticketPrice, bonusBall, etc.). These parameters are set at the beginning of a drawing and remain immutable for its duration. Any updates made by governance or an admin only take effect in subsequent drawings; the parameters of the current drawing are never affected.

Critical Timing Considerations:

Drawing Parameter Isolation: All drawing parameters (ticketPrice, normalBallMax, bonusballMax, referralWinShare) are frozen when the drawing is initialized
Mid-Drawing Safety: Global parameter changes during active drawings do NOT affect current ticket purchases
Next Drawing Impact: All parameter changes only take effect in the next drawing parameterization
This finding also addresses the following guiding question:

Can admin changes (e.g., ticketPrice, normalBallMax, fees) made mid-drawing create inconsistent states or violate expectations for players/LPs?

JackpotBridgeManager is a cross-chain bridge that enables ticket purchases and winnings claims across different blockchains. It acts as a custodian and defines the following flow for ticket purchases:

The user initiates a ticket purchase through the JackpotBridgeManager, providing all required information.
The JackpotBridgeManager fetches the current single-ticket price and calculates a total amount based on the number of tickets user wants. It then pulls the corresponding funds from the user.
Afterwards, It approves the Jackpot contract to spend the same amount, allowing the Jackpot contract to pull the funds when needed.
The JackpotBridgeManager calls the buyTickets function on the Jackpot contract to execute the purchase.
The Jackpot contract pulls the required funds from the JackpotBridgeManager to complete the transaction.
The problem is that the JackpotBridgeManager fetches the current ticket price defined in the Jackpot contract and not the ticket price of an actual drawing. This leads to a two scenarios if the ticket price is updated:

Price Increase
If the ticket price is increased after a drawing has started, users who purchase tickets through the JackpotBridgeManager will overpay, as it fetches the latest global ticket price rather than the price fixed for the current drawing. The excess funds instead remain locked inside the JackpotBridgeManager contract.

Price decrease
If the ticket price is decreased after a drawing has started, it would result in a complete denial of service (DoS) of the manager’s buyTickets function. Because the price is lower, the manager would pull fewer funds than required for the actual purchase and, as a result, would not approve a sufficient amount for the Jackpot contract. This leads to an “insufficient approval” revert when the Jackpot contract attempts to pull the funds from the manager

Impact
Impact is High, as both likelihood and impact (Loss of funds, DoS) are High

Recommended mitigation steps
Make sure to fetch the ticket price of an actual drawing and not the latest one from the Jackpot when purchasing tickets through JackpotBridgeManager

uint256 ticketPrice = jackpot.getDrawingState(currentDrawingId).ticketPrice;




[M-10] Global Variable Manipulation During Active Draw Alters End Result

**STATUS:** We found it!

Finding description and impact
The core issue lies in the ability of the owner to modify global configuration variables during an active jackpot draw. These parameters directly influence jackpot settlement logic, fee distribution, payout calculation, and even randomness handling.
Because these values are read during settlement (after tickets have been purchased but before the draw is finalized), changing them mid-round allows the owner to unfairly alter the outcome of the draw or cause settlement failures.

Specifically, the following global variables can be updated during an active draw:

protocolFee
referralFee
payoutCalculator
entropy (entropy provider)
jackpotLPManager
Each of these variables can alter the draw’s behavior or payout path:

protocolFee / referralFee — allow manipulation of fee distribution to reduce/increse rewards to players.
payoutCalculator — can redirect or alter payout logic to arbitrary addresses.
entropy — can manipulate randomness or prevent valid settlement.
jackpotLPManager — can revert settlements or redirect LP-related funds.
Impact:
This undermines jackpot integrity, enabling admin-based manipulation of winnings, payout denial, or DoS of settlement — a severe trust and fairness violation affecting all players.

Recommended mitigation steps
Restrict all configuration-changing functions (those modifying global variables like the above) to be callable only when no active draw is in progress.
Introduce a locking mechanism that freezes sensitive parameters once a draw is initialized (initializeJackpot() called) until settlement completes.


[M-11] Global Variable Manipulation During Active Draw Alters End Result

**STATUS:** We found it!

Finding description and impact
The core issue lies in the ability of the owner to modify global configuration variables during an active jackpot draw. These parameters directly influence jackpot settlement logic, fee distribution, payout calculation, and even randomness handling.
Because these values are read during settlement (after tickets have been purchased but before the draw is finalized), changing them mid-round allows the owner to unfairly alter the outcome of the draw or cause settlement failures.

Specifically, the following global variables can be updated during an active draw:

protocolFee
referralFee
payoutCalculator
entropy (entropy provider)
jackpotLPManager
Each of these variables can alter the draw’s behavior or payout path:

protocolFee / referralFee — allow manipulation of fee distribution to reduce/increse rewards to players.
payoutCalculator — can redirect or alter payout logic to arbitrary addresses.
entropy — can manipulate randomness or prevent valid settlement.
jackpotLPManager — can revert settlements or redirect LP-related funds.
Impact:
This undermines jackpot integrity, enabling admin-based manipulation of winnings, payout denial, or DoS of settlement — a severe trust and fairness violation affecting all players.

Recommended mitigation steps
Restrict all configuration-changing functions (those modifying global variables like the above) to be callable only when no active draw is in progress.
Introduce a locking mechanism that freezes sensitive parameters once a draw is initialized (initializeJackpot() called) until settlement completes.
