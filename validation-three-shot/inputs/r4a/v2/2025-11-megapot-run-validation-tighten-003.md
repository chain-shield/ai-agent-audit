# 2025-11-megapot Round 4a V12 Sweep Input

Source submission candidates: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/submission-candidates/v2/2025-11-megapot-run-validation-tighten-003.md`
Candidate count: `17`

This file contains only post-R4 submission candidates. R4a must decide whether each candidate is
materially distinct from the configured V12 / prior-findings context.

## Candidates

### H-1 / `emCZQy_NnRkOsuezTEB29`
- Finding Title: Stale bridge USDC allowance can drain funds deposited for later claims
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: High
- Root Cause Family: `bridge-stale-allowance`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Safeguards, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The bridge gives an arbitrary `approveTo` allowance for the current claim amount and never clears it; a malicious route spender approved by one claimant can later pull unrelated USDC that arrives in the bridge manager, which is incremental harm beyond the signer’s own authorized claim route.
- Code Evidence: `contracts/JackpotBridgeManager.sol::claimWinnings` computes `claimedAmount` after `jackpot.claimWinnings`, then `_bridgeFunds` calls `usdc.approve(_bridgeRequest.approveTo, claimedAmount)` and performs arbitrary `to.call(data)` with only a balance-delta check; there is no allowance reset or spender allowlist in `_bridgeFunds`.

### M-2 / `rQOhmiuSK6JH_yE1hbzbl`
- Finding Title: ECDSA-only bridge ownership checks lock ERC-1271 smart-wallet recipients
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bridge-erc1271-unsupported`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Bridge purchases accept any nonzero recipient, including contract wallets, but later require an ECDSA recovered signer to match that recipient. A smart-wallet recipient cannot satisfy that path, so its bridge-held tickets and winnings can become practically unclaimable through the intended bridge interface.
- Code Evidence: `contracts/JackpotBridgeManager.sol::buyTickets` stores arbitrary `_recipient` in `ticketOwner`, while `claimWinnings`, `claimTickets`, and `_validateTicketOwnership` use `ECDSA.recover` and direct address equality rather than ERC-1271 validation.

### M-6 / `89sj8KYQEZ-uL21jpBujE`
- Finding Title: LP deposits can front-run governance pool-cap reductions
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `lp-cap-frontrun`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: A normal LP can add pending deposits before a governance cap reduction, making the otherwise valid lower cap revert against live `lpPoolTotal + pendingDeposits`. This is not malicious use of trusted authority; it is permissionless state movement obstructing an in-scope administrative update.
- Code Evidence: `contracts/JackpotLPManager.sol::processDeposit` permits deposits up to the current cap and increments pending state, and `setLPPoolCap` reverts when `_lpPoolCap < lpPoolTotal + pendingDeposits`; `Jackpot.setGovernancePoolCap` forwards the new cap directly.

### M-8 / `-QwYrwqDqTJ71r1NZS63J`
- Finding Title: Entropy address changes during a pending drawing brick the callback path
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `jackpot-entropy-rotation-pending`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: `setEntropy` changes the only authorized callback sender immediately, including for the active locked drawing. If the old entropy contract has the pending request, its legitimate callback will fail `onlyEntropy`, leaving jackpot progression stuck unless governance uses manual recovery.
- Code Evidence: `contracts/Jackpot.sol::onlyEntropy` requires `msg.sender == address(entropy)`, `setEntropy` mutates that address without checking `jackpotLock` or pending requests, and `scaledEntropyCallback` is the sole normal path that unlocks and advances the drawing.

### M-9 / `4fPnoqYOKR9JwLwxDlbeJ`
- Finding Title: Bonusball values beyond the bit-packing domain can erase bonus matches and break claims
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `ticket-bitpacking-boundary`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The drawing’s bonusball upper bound can grow into a range that is valid as `uint8` but invalid for the ticket bit layout, because the code never enforces `normalBallMax + bonusball <= 255`. Tickets and winning numbers can then be packed, counted, or decoded inconsistently, producing claim failures or wrong tiers.
- Code Evidence: `contracts/Jackpot.sol::_setNewDrawingState` derives `newBonusball` without a packing-domain check, `_validateAndStoreTickets` accepts bonusballs up to the drawing max, and `TicketComboTracker.insert`/`countTierMatchesWithBonusball` compute `1 << (_bonusball + _tracker.normalMax)`.

### M-10 / `VcshH0W6i-ZnZDjOPbtST`
- Finding Title: No-referral winner share is credited to the current drawing instead of the settled drawing
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `no-referral-current-drawing`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Claims for old tickets are paid after `currentDrawingId` has advanced, but missing-referral rewards are credited to the new active drawing’s LP earnings. Claim timing can therefore move value between LP accounting periods and distort the pool value used for later settlement.
- Code Evidence: `contracts/Jackpot.sol::claimWinnings` loads `ticketInfo.drawingId` for the winning ticket, but `_payReferrersWinnings` credits `drawingState[currentDrawingId].lpEarnings` when no referral scheme is set.

### H-12 / `kjDblGFEXt_ejozdhDT3K`
- Finding Title: Arbitrary bridge calldata can steal custodied ticket NFTs
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: High
- Root Cause Family: `bridge-arbitrary-call-nft-theft`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The bridge manager custodies all bridged ticket NFTs and allows a successful winnings claimant to execute arbitrary calldata from the manager address. Because the balance-delta check only covers USDC, the call can transfer unrelated ticket NFTs owned by the manager without affecting the claimant’s USDC delta.
- Code Evidence: `contracts/JackpotBridgeManager.sol::buyTickets` mints tickets to the manager and stores logical owners separately, while `_bridgeFunds` performs arbitrary `to.call(data)` after approving USDC and does not restrict calls to `JackpotTicketNFT` transfers.

### M-18 / `oZQ2pM2T1ltePA70cdJcZ`
- Finding Title: Provider sequence collisions can overwrite pending entropy requests
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-sequence-collision`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Pending entropy requests are keyed only by sequence number even though sequence numbers can repeat across provider rotations. A new request after rotation can overwrite an old pending request, making the old callback use the wrong destination/data or leaving the original consumer stranded.
- Code Evidence: `contracts/ScaledEntropyProvider.sol` stores `mapping(uint64 => PendingRequest) pending`, `setEntropyProvider` changes provider identity, and `_storePendingRequest` overwrites `pending[_sequence]` without including provider identity or checking an existing entry.

### M-20 / `p2_DN-XODOvXu_tNksgej`
- Finding Title: Bridge ticket purchases use live ticketPrice instead of the active drawing snapshot
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bridge-live-price-mismatch`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The bridge computes payment from the mutable global ticket price, while the jackpot validates and stores tickets against the active drawing state. A mid-drawing price change can overcharge, undercharge, or leave surplus funds in the bridge for purchases that should use the drawing’s snapshot price.
- Code Evidence: `contracts/JackpotBridgeManager.sol::buyTickets` reads `jackpot.ticketPrice()` directly before forwarding funds, whereas `contracts/Jackpot.sol::_setNewDrawingState` snapshots `ticketPrice` into `drawingState[currentDrawingId]`.

### M-22 / `OYZfVCU_Jw3zjvZAlSLNS`
- Finding Title: Nonce-less ticket withdrawal signatures can transfer later-valuable tickets
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bridge-signature-no-expiry`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: A signature authorizing ticket withdrawal has no time limit or nonce and can be executed later when ticket value or owner intent has changed. That is materially worse for jackpot tickets because a previously low-value ticket can become winning before the stale withdrawal is submitted.
- Code Evidence: `contracts/JackpotBridgeManager.sol::claimTickets` validates the recovered signer for the listed ticket IDs and transfers NFTs to the signed recipient, but `hashClaimTicketsRequest` lacks nonce, deadline, and consumed-signature state.

### H-26 / `Zy2Cwvh0tHGOtbihykmbO`
- Finding Title: LP pending deposits can be zeroed by accumulator growth
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `lp-zero-share-inflation`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The deposit-to-share conversion has no minimum-share guard, so a nonzero pending deposit can become zero shares after LP accumulator growth. That produces a real LP loss path, though the practical severity is better assessed as Medium because it depends on extreme accumulator ratios and affects deposit accounting rather than immediate pool drain.
- Code Evidence: `contracts/JackpotLPManager.sol::processDeposit` records pending deposits, and `_consolidateDeposits` floors `depositAmount * 1e18 / drawingAccumulator[drawingId]` then deletes the pending deposit even when the result is zero.

### M-27 / `rbb-sLW7w5f9_y-bT86sa`
- Finding Title: Tier-zero winners are not counted at settlement but can have configured payouts
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `tier-zero-unaccounted`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The tracker does not populate zero-match/no-bonus winners, but the payout calculator supports tier 0 configuration. If tier 0 has a payout, settlement undercounts liabilities while later claims can still look up a tier-0 payout, breaking LP accounting for a supported tier.
- Code Evidence: `contracts/lib/TicketComboTracker.sol::countTierMatchesWithBonusball` only computes subset matches starting at one and bonus-only index 1, while `GuaranteedMinimumPayoutCalculator` stores and returns payouts for indexed tiers including tier 0.

### M-30 / `l5OqMxsLjuLw9XhbtQprM`
- Finding Title: Bridge ticket and winnings signatures never expire
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bridge-signature-no-expiry`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Both bridge claim paths accept perpetual signatures. Because ticket value and bridge routes can change over time, the absence of expiry or revocation creates a credible stale execution path with medium impact.
- Code Evidence: `contracts/JackpotBridgeManager.sol::claimTickets` and `claimWinnings` recover signatures from hashes that omit nonce/deadline fields and never write replay-prevention state.

### M-44 / `CJfADIczTrhQXGEfG6myl`
- Finding Title: Ticket purchases remain open after drawingTime until runJackpot is called
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `late-ticket-purchase`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The drawing cutoff is enforced only by `runJackpot`, not by ticket purchase validation. If no one immediately pays to run the drawing, ordinary users can buy tickets after the scheduled cutoff with more information about participation and timing than intended.
- Code Evidence: `contracts/Jackpot.sol::_validateBuyTicketInputs` checks lock, payment, drawing ID, and ticket shape but not `drawingTime`; `runJackpot` is the function that checks `block.timestamp >= drawingState[currentDrawingId].drawingTime`.

### M-46 / `3nNN8Mv0tqj6acsGySPOG`
- Finding Title: Unsnapshotted payout calculator can zero active drawing payouts
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `payout-calculator-unsnapshotted`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: A calculator rotation before settlement causes the active drawing to call a calculator that may not have tier data for that drawing. Winners can therefore lose expected payouts or settlement can be distorted.
- Code Evidence: `contracts/Jackpot.sol::_calculateDrawingUserWinnings` invokes the current `payoutCalculator`, and `GuaranteedMinimumPayoutCalculator.calculateAndStoreDrawingUserWinnings` relies on `drawingTierInfo[_drawingId]` that only its own `setDrawingTierInfo` populated.

### H-60 / `DCLCnC_dWz7GX-y5h94Yu`
- Finding Title: Reusing one entropy seed for all request sets correlates normal and bonus draws
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-set-seed-correlation`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Each SetRequest is derived from the same entropy value without domain separation by set index, so separate draws can be correlated when ranges overlap. For jackpot randomness, correlation between normal and bonus balls can distort outcome probabilities and strategy, though Medium is more defensible than High absent deterministic extraction.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::entropyCallback` passes the same random value into scaling for all stored requests, and the normal-ball and bonus-ball requests from `Jackpot.runJackpot` are distinct sets in the same entropy callback.

### M-90 / `aj9gdn9sR6wy8WlQZd92s`
- Finding Title: Small normalBallMax values can make payout settlement revert
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `normal-range-underflow`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The contract permits normal-ball range updates that are incompatible with payout-combination math. If a drawing is initialized with too small a normal range, settlement can revert while computing tier totals, blocking jackpot progression.
- Code Evidence: `contracts/GuaranteedMinimumPayoutCalculator.sol::_calculateTierTotalWinningCombos` calls `Combinations.choose(_normalMax - 5, 5 - matches)`, and `Combinations.choose` asserts if `n < k`; the Jackpot setters do not enforce the required lower bound.
