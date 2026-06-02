# 2025-11-megapot Three-Shot Submission Candidates run-validation-tighten-001

Status: Complete
Source assembled run: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/runs/v2/2025-11-megapot-run-validation-tighten-001.md`
Canonicalization screen: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/dedup-screens/v2/2025-11-megapot-run-validation-tighten-001.md`

## Canonicalization Summary

- Candidate reportable findings before R4: `74`
- Kept after R4 canonicalization: `16`
- Dropped by R4 cleanup: `58`
- Dropped finding ids: `M-5, M-7, M-11, M-14, M-15, M-16, M-17, M-21, M-22, M-24, H-26, M-29, M-30, M-36, M-37, H-39, M-41, M-42, M-43, M-45, M-46, M-47, H-48, M-50, M-54, H-56, M-59, M-69, M-70, M-71, M-72, M-73, M-74, M-76, M-77, M-78, M-79, M-80, M-81, M-82, M-83, M-84, M-85, M-86, M-87, M-88, M-92, M-93, M-94, L-95, M-96, M-97, M-98, M-100, M-101, M-102, M-104, M-105`

## Root Cause Groups

- `bonusball-bit-packing`: M-9, M-15 -> M-9, H-48 -> M-9, M-69 -> M-9, M-72 -> M-9, M-74 -> M-9, M-76 -> M-9, M-80 -> M-9, M-81 -> M-9, M-82 -> M-9, M-83 -> M-9, M-92 -> M-9
- `bridge-arbitrary-nft-call`: H-12, H-39 -> H-12, H-56 -> H-12
- `bridge-erc1271-unsupported`: M-2, M-17 -> M-2, M-41 -> M-2
- `bridge-live-ticket-price`: M-20
- `bridge-signature-freshness`: M-4, M-16 -> M-4, M-21 -> M-4, M-22 -> M-4, M-29 -> M-4, M-30 -> M-4, M-42 -> M-4, M-54 -> M-4
- `bridge-stale-allowance`: H-1, M-37 -> H-1, M-59 -> H-1
- `entropy-provider-sequence-collision`: M-5 -> M-18, M-18, M-70 -> M-18, M-71 -> M-18, M-73 -> M-18, M-77 -> M-18, M-78 -> M-18, M-79 -> M-18, M-85 -> M-18, M-87 -> M-18, M-93 -> M-18, M-100 -> M-18, M-102 -> M-18
- `jackpot-entropy-rotation`: M-8, M-11 -> M-8, M-36 -> M-8, M-84 -> M-8, M-97 -> M-8, M-98 -> M-8
- `late-buy-window`: M-44, M-50 -> M-44
- `lp-cap-front-run`: M-6, M-45 -> M-6, M-104 -> M-6, M-105 -> M-6
- `lp-zero-share-consolidation`: M-23, H-26 -> M-23
- `normal-range-underflow`: M-90, L-95 -> M-90
- `payout-calculator-rotation`: M-7 -> M-91, M-14 -> M-91, M-46 -> M-91, M-86 -> M-91, M-88 -> M-91, M-91, M-94 -> M-91, M-96 -> M-91, M-101 -> M-91
- `scaled-entropy-cross-set-correlation`: H-60
- `tier-zero-underaccounting`: M-27
- `wrong-drawing-referral-credit`: M-10, M-24 -> M-10, M-43 -> M-10, M-47 -> M-10

## R4a V12 Sweep Summary

- Input candidates before R4a: `16`
- Kept after R4a V12 sweep: `10`
- Excluded as V12 / prior-finding overlap: `6`
- Excluded finding ids: `M-2, M-9, M-23, M-44, H-60, M-90`
- V12 sweep screen: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/v12-sweeps/v2/2025-11-megapot-run-validation-tighten-001.md`

## Submission Candidates

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

### M-4 / `ZwhrSB7704IKw_ypW0qEm`
- Finding Title: Bridge claim signatures never expire
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bridge-signature-no-expiry`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The bridge’s EIP-712 claim authorizations contain ticket IDs and routing data but no nonce, deadline, cancellation state, or claim amount. A withheld or leaked authorization remains executable after user intent or route safety changes, which is a credible replay/stale-signature risk for the in-scope bridge workflow.
- Code Evidence: `contracts/JackpotBridgeManager.sol::hashClaimWinningsRequest` and `hashClaimTicketsRequest` omit nonce and deadline fields, and `claimWinnings`/`claimTickets` only verify the recovered signer against stored ticket ownership.

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

### M-91 / `GPliAtb0Dot7OjQ4XXxl1`
- Finding Title: Payout calculator rotation reprices active and settled claims
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `payout-calculator-live-claims`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Both active settlement and later claim payout lookup depend on the mutable global calculator. A future update can therefore affect drawings whose economic terms should already be fixed.
- Code Evidence: `contracts/Jackpot.sol::setPayoutCalculator` mutates the global reference, `_calculateDrawingUserWinnings` and `claimWinnings` both use that current reference for drawing-specific payout data.
