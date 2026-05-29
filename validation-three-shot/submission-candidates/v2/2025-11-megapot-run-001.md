# 2025-11-megapot Three-Shot Submission Candidates run-001

Status: Complete
Source assembled run: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/runs/v2/2025-11-megapot-run-001.md`
Canonicalization screen: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/dedup-screens/v2/2025-11-megapot-run-001.md`

## Canonicalization Summary

- Candidate reportable findings before R4: `50`
- Kept after R4 canonicalization: `15`
- Dropped by R4 cleanup: `35`
- Dropped finding ids: `M-7, M-8, M-11, M-17, M-24, H-26, M-36, M-37, H-39, M-41, M-43, M-46, M-47, M-50, H-56, M-59, M-70, M-71, M-73, M-76, M-77, M-78, M-79, M-84, M-85, M-86, M-87, M-88, M-93, M-94, M-96, M-97, M-98, M-100, M-102`

## Root Cause Groups

- `bonusball-settlement-dos`: M-76 -> M-83, M-83
- `bridge-custody-arbitrary-call`: H-12, H-39 -> H-12, H-56 -> H-12
- `bridge-erc1271-lock`: M-2, M-17 -> M-2, M-41 -> M-2
- `bridge-stale-allowance`: H-1, M-37 -> H-1, M-59 -> H-1
- `bridge-ticket-price-drift`: M-20
- `entropy-authority-drift`: M-5, M-8 -> M-5, M-11 -> M-5, M-36 -> M-5, M-84 -> M-5, M-97 -> M-5, M-98 -> M-5
- `entropy-sequence-collision`: M-18, M-70 -> M-18, M-71 -> M-18, M-73 -> M-18, M-77 -> M-18, M-78 -> M-18, M-79 -> M-18, M-85 -> M-18, M-87 -> M-18, M-93 -> M-18, M-100 -> M-18, M-102 -> M-18
- `entropy-stream-correlation`: H-60
- `late-ticket-entry`: M-44, M-50 -> M-44
- `lp-zero-share-rounding`: M-23, H-26 -> M-23
- `payout-calculator-drift`: M-7 -> M-14, M-14, M-46 -> M-14, M-86 -> M-101, M-88 -> M-101, M-91, M-94 -> M-101, M-96 -> M-101, M-101
- `referral-share-wrong-epoch`: M-10, M-24 -> M-10, M-43 -> M-10, M-47 -> M-10
- `tier-zero-undercount`: M-27

## R4a V12 Sweep Summary

- Input candidates before R4a: `15`
- Kept after R4a V12 sweep: `13`
- Excluded as V12 / prior-finding overlap: `2`
- Excluded finding ids: `M-23, M-44`
- V12 sweep screen: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/v12-sweeps/v2/2025-11-megapot-run-001.md`

## Submission Candidates

### H-1 / `emCZQy_NnRkOsuezTEB29`
- Finding Title: Arbitrary bridge call leaves reusable USDC allowance that can drain later bridge-manager funds
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bridge-stale-allowance`
- Checklist Gates Passed: `Stage0, scope, supported-behavior, root-cause, safeguards, bounded-impact`
- Checklist Gates Failed: `high-impact`
- Detailed Reason: `_bridgeFunds` grants an arbitrary `approveTo` allowance, executes arbitrary signed calldata, and never clears or verifies the allowance after only checking that exactly the current claim amount left the manager. The report overstates "future winnings awaiting bridging" because normal claim and buy flows do not leave an inter-transaction balance window, but any existing or later idle bridge-manager surplus, including overcharge surplus from the live ticket-price mismatch path, can be drained up to the stale allowance. This is a real bounded asset-loss issue, not High on its own.
- Code Evidence: In `contracts/JackpotBridgeManager.sol`, `claimWinnings` receives claimed USDC then calls `_bridgeFunds`; `_bridgeFunds` calls `usdc.approve(_bridgeDetails.approveTo, _claimedAmount)`, performs `_bridgeDetails.to.call(_bridgeDetails.data)`, and checks only `preUSDCBalance - postUSDCBalance == _claimedAmount` without resetting allowance. `buyTickets` can leave surplus because it pulls `jackpot.ticketPrice()` while `Jackpot.buyTickets` charges `drawingState[currentDrawingId].ticketPrice`.

### M-2 / `rQOhmiuSK6JH_yE1hbzbl`
- Finding Title: ECDSA-only bridge claims permanently lock tickets owned by ERC-1271 smart wallets
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `erc1271-unsupported`
- Checklist Gates Passed: `Stage0, scope, supported-behavior, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The bridge accepts any nonzero recipient as the recorded ticket owner, including contract wallets, but both release paths require an ECDSA recovery result to equal that recorded owner. A contract wallet cannot produce an EOA signature that recovers to the contract address, and there is no ERC-1271 `SignatureChecker` fallback or contract-recipient rejection. For bridge-custodied tickets this can lock ticket withdrawal and any later winnings/refund route for smart-wallet owners.
- Code Evidence: `contracts/JackpotBridgeManager.sol::buyTickets` records `ticketOwner[ticketId] = _recipient` after only checking `_recipient != address(0)`. `claimWinnings` and `claimTickets` call `ECDSA.recover(...)` and `_validateTicketOwnership`, which compares `ticketOwner[ticketId]` directly to the recovered signer.

### M-5 / `WoebK6L4bf9CMQmO1z0JH`
- Finding Title: Changing entropy provider while a drawing is pending permanently blocks settlement
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-authority-drift`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: This is a real mid-flow global-state bug. `runJackpot` requests randomness from the then-current entropy contract, but the later callback authorization reads the live mutable `entropy` address. The docs say `setEntropy` affects future drawing executions and the benchmark explicitly calls out mid-drawing admin changes, so a routine rotation during an in-flight request can reject the valid old callback and leave the drawing locked until privileged recovery.
- Code Evidence: `contracts/Jackpot.sol::runJackpot` calls `entropy.requestAndCallbackScaledRandomness` after `_lockJackpot`. `setEntropy` updates the global `entropy` without checking `jackpotLock`, and the `onlyEntropy` modifier on `scaledEntropyCallback` compares `msg.sender` to the live `address(entropy)`.

### M-10 / `VcshH0W6i-ZnZDjOPbtST`
- Finding Title: No-referral winner share is credited to the current drawing instead of the settled drawing
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `referral-share-wrong-epoch`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Claiming a completed no-referral winning ticket after `currentDrawingId` has advanced credits the retained referral-win-share to the active drawing's LP earnings, not the drawing that funded the payout. Since settlement already charged the completed drawing for the full tier payout, delayed claimers can redirect material value between LP cohorts. The path is permissionless for any winning ticket holder.
- Code Evidence: `contracts/Jackpot.sol::claimWinnings` obtains `ticketInfo.drawingId` and the settled drawing state, but `_payReferrersWinnings` receives no drawing ID. In `_payReferrersWinnings`, the no-referral branch writes `drawingState[currentDrawingId].lpEarnings += referrerShare`.

### H-12 / `kjDblGFEXt_ejozdhDT3K`
- Finding Title: Arbitrary bridge call lets a winning ticket holder steal other users' custodied ticket NFTs
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: High
- Root Cause Family: `bridge-arbitrary-call-custody`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: A claimant can use their own valid winning bridge ticket to make `_bridgeFunds` execute arbitrary calldata from the bridge manager, which is the ERC-721 owner of all bridge-custodied tickets. Calling the ticket NFT to transfer a victim ticket is authorized at the NFT layer, and the USDC balance-delta check can be satisfied by spending the claimant's approved winnings during the receiver hook. This creates direct theft of unrelated users' custodied ticket NFTs and any attached future or settled value.
- Code Evidence: `contracts/JackpotBridgeManager.sol::claimWinnings` validates only the claimant's listed ticket IDs. `_bridgeFunds` then calls arbitrary `_bridgeDetails.to.call(_bridgeDetails.data)` from the manager. `JackpotBridgeManager.buyTickets` mints all bridge tickets to `address(this)`, so the manager is authorized for `JackpotTicketNFT.safeTransferFrom` on unrelated custodied tickets.

### M-14 / `RJM_fI7cXw-dK2wVfx0Xs`
- Finding Title: Mid-drawing payout calculator update causes active drawing to settle with zero stored payouts
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `payout-calculator-drift`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Replacing `payoutCalculator` during an active drawing can make settlement use a calculator that never snapshotted that drawing's tier configuration. The fresh calculator's default `DrawingTierInfo` makes all tier weights and minimum tiers zero, so winners can be assigned zero stored payouts. The docs and scope treat future-only global changes and mid-drawing effects as security-relevant.
- Code Evidence: `contracts/Jackpot.sol::setPayoutCalculator` mutates the global pointer only. `GuaranteedMinimumPayoutCalculator::calculateAndStoreDrawingUserWinnings` reads `drawingTierInfo[_drawingId]` without an initialized flag and skips every tier when the snapshot is default zero.

### M-18 / `oZQ2pM2T1ltePA70cdJcZ`
- Finding Title: Changing entropy providers can overwrite pending randomness requests with colliding sequence IDs
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-provider-sequence-collision`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: `ScaledEntropyProvider` documents provider updates as future-only, but pending requests are keyed only by `sequence` while Pyth V2 callbacks include a provider argument. After a valid owner provider rotation, a request through the new provider can reuse the old provider's sequence and overwrite the pending callback. That exposes an in-flight Jackpot request to misdelivery or loss through a normal migration plus permissionless later request.
- Code Evidence: `contracts/ScaledEntropyProvider.sol` stores `mapping(uint64 => PendingRequest) private pending`, writes in `_storePendingRequest(sequence, ...)`, and ignores the `provider` argument in `entropyCallback(uint64 sequence, address /*provider*/, ...)`. `setEntropyProvider` has no pending-request guard.

### M-20 / `p2_DN-XODOvXu_tNksgej`
- Finding Title: Bridge ticket purchases use live global ticketPrice instead of the active drawing price, causing overcharges or purchase DoS
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bridge-ticket-price-drift`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Drawing ticket price is snapshotted in `drawingState`, but the bridge manager charges callers using the live global `jackpot.ticketPrice()`. A governance price update intended for future drawings can make bridge users overpay into the manager when the global price is higher, or make bridge purchases revert when the global price is lower than the active drawing's snapshotted price. This is exactly the mid-drawing global-parameter drift class highlighted by the benchmark.
- Code Evidence: `contracts/JackpotBridgeManager.sol::buyTickets` computes `ticketCost = jackpot.ticketPrice() * _tickets.length` and transfers that amount. `contracts/Jackpot.sol::buyTickets` charges `ticketsValue = numTicketsBought * currentDrawingState.ticketPrice`, while `_setNewDrawingState` snapshots the global `ticketPrice` per drawing.

### M-27 / `rbb-sLW7w5f9_y-bT86sa`
- Finding Title: Tier zero winners are excluded from settlement obligations but can still claim stored tier-zero payouts
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `tier-zero-undercount`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The combo tracker never populates `uniqueResult[0]` or `dupResult[0]`, yet the payout calculator supports all 12 tiers and can store a nonzero tier-0 payout from configured weights/minimums. Settlement therefore can reserve zero user payout for tier-0 user tickets while later `claimWinnings` computes tier ID 0 and pays the stored tier-0 payout. This is configuration-dependent but matches the documented 12-tier payout model.
- Code Evidence: `contracts/lib/TicketComboTracker.sol::countTierMatchesWithBonusball` counts tiers with one or more normal matches and bonusball-only tier 1, leaving index 0 unset. `GuaranteedMinimumPayoutCalculator::_calculateAndStoreTierPayouts` can set `tierPayouts[drawingId][0]`, and `Jackpot.claimWinnings` pays `payoutCalculator.getTierPayout(drawingId, tierId)` for any computed tier including 0.

### H-60 / `DCLCnC_dWz7GX-y5h94Yu`
- Finding Title: ScaledEntropyProvider reuses one entropy seed for all random sets, correlating normal balls and bonusball
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-stream-correlation`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `high-impact`
- Detailed Reason: `_getScaledRandomness` reuses the same seed for every set, and each draw routine starts its nonce at zero, so separate sets are not domain-separated. Jackpot requests normal balls and bonusball as separate sets; when ranges overlap or match, the bonusball stream is correlated with the normal-ball stream, violating the intended independent lottery dimensions. The fairness and EV impact is real but probabilistic, so I assess Medium rather than High.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::_getScaledRandomness` passes `uint256(_randomNumber)` unchanged to every `FisherYatesRejection.draw` or `_drawWithReplacement` call. `contracts/lib/FisherYatesWithRejection.sol::draw` initializes `nonce = 0` for each draw, and `Jackpot.runJackpot` creates separate normal and bonusball set requests.

### M-83 / `eU8MFp1NT2zpXY6vsm4RZ`
- Finding Title: Bonusball ranges above the bit-packing boundary can make entropy settlement revert and lock the drawing
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bonusball-settlement-dos`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: This finding correctly focuses on callback reversion. `_setNewDrawingState` can initialize a drawing where some in-range bonusball values cannot be encoded with the normal-ball bit domain. Once `runJackpot` locks the drawing, an entropy result containing such a high bonusball causes `countTierMatchesWithBonusball` to revert, leaving normal settlement unavailable.
- Code Evidence: `contracts/Jackpot.sol::_setNewDrawingState` does not require `uint256(normalBallMax) + uint256(newBonusball) <= 255`. `TicketComboTracker.countTierMatchesWithBonusball` packs the returned bonusball with `_bonusball + _tracker.normalMax` during `Jackpot.scaledEntropyCallback`.

### M-91 / `GPliAtb0Dot7OjQ4XXxl1`
- Finding Title: Payout calculator rotation reprices already initialized or settled ticket claims
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `payout-calculator-drift`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Jackpot lacks a per-drawing payout calculator snapshot. Rotation can affect either an active drawing's settlement if the new calculator was not initialized for that drawing, or historical claims if old payouts remain in the prior calculator. Both are material deviations from future-only parameter changes.
- Code Evidence: `contracts/Jackpot.sol::setPayoutCalculator` mutates one global. `_setNewDrawingState` calls `payoutCalculator.setDrawingTierInfo(currentDrawingId)` only on the then-current calculator, and `claimWinnings` reads from the live calculator.

### M-101 / `OyeL2vMtI2i2rZz06wHLH`
- Finding Title: Updating the payout calculator breaks already-settled ticket claims by reading payouts from the new calculator
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `payout-calculator-drift`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: A settled drawing's tier payout data is not bound to the ticket or drawing in Jackpot. Updating the calculator before all winners claim can make old tickets query a new calculator with no stored data for that drawing, resulting in zero or wrong payouts. Because `claimWinnings` burns before transfer, the user cannot retry against the original calculator after a successful zero-payout claim.
- Code Evidence: `contracts/Jackpot.sol::claimWinnings` calls `jackpotNFT.burnTicket(ticketId)` and then reads `payoutCalculator.getTierPayout(drawingId, tierId)`. `setPayoutCalculator` only changes the global pointer.
