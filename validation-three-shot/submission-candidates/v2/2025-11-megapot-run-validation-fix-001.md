# 2025-11-megapot Three-Shot Submission Candidates run-validation-fix-001

Status: Complete
Source assembled run: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/runs/v2/2025-11-megapot-run-validation-fix-001.md`
Canonicalization screen: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/dedup-screens/v2/2025-11-megapot-run-validation-fix-001.md`

## Canonicalization Summary

- Candidate reportable findings before R4: `74`
- Kept after R4 canonicalization: `29`
- Dropped by R4 cleanup: `45`
- Dropped finding ids: `M-5, M-11, M-14, M-16, M-17, M-21, M-24, H-26, M-29, M-30, M-36, M-37, H-39, M-41, M-42, M-43, M-45, M-46, M-47, M-50, M-54, H-56, M-70, M-71, M-72, M-77, M-78, M-79, M-81, M-82, M-84, M-85, M-87, M-88, M-93, M-94, L-95, M-96, M-97, M-98, M-100, M-101, M-102, M-104, M-105`

## Root Cause Groups

- `bridge-arbitrary-call-nft-theft`: H-12, H-39 -> H-12, H-56 -> H-12
- `bridge-erc1271-unsupported`: M-2, M-17 -> M-2, M-41 -> M-2
- `bridge-live-price-mismatch`: M-20
- `bridge-signature-replay`: M-4, M-16 -> M-4, M-21 -> M-4, M-22, M-29 -> M-4, M-30 -> M-4, M-42 -> M-4, M-54 -> M-4
- `bridge-stale-allowance`: H-1, M-37 -> H-1, M-59
- `entropy-provider-sequence-collision`: M-5 -> M-18, M-18, M-70 -> M-18, M-71 -> M-18, M-73, M-77 -> M-18, M-78 -> M-18, M-79 -> M-18, M-85 -> M-18, M-87 -> M-18, M-93 -> M-18, M-100 -> M-18, M-102 -> M-18
- `entropy-shared-seed-correlation`: H-60
- `jackpot-live-entropy-pointer`: M-8, M-11 -> M-8, M-36 -> M-8, M-84 -> M-8, M-97 -> M-8, M-98 -> M-8
- `late-buy-after-deadline`: M-44, M-50 -> M-44
- `lp-cap-frontrun`: M-6, M-45 -> M-6, M-104 -> M-6, M-105 -> M-6
- `lp-zero-share-inflation`: M-23, H-26 -> M-23
- `no-referral-current-drawing`: M-10, M-24 -> M-10, M-43 -> M-10, M-47 -> M-10
- `normal-range-underflow`: M-90, L-95 -> M-90
- `payout-calculator-live-pointer`: M-7, M-14 -> M-7, M-46 -> M-7, M-86, M-88 -> M-86, M-91, M-94 -> M-86, M-96 -> M-86, M-101 -> M-86
- `ticket-bitpacking-boundary`: M-9, M-15, H-48, M-69, M-72 -> M-9, M-74, M-76, M-80, M-81 -> M-9, M-82 -> M-9, M-83, M-92
- `tier-zero-settlement-gap`: M-27

## R4a V12 Sweep Summary

- Input candidates before R4a: `29`
- Kept after R4a V12 sweep: `27`
- Excluded as V12 / prior-finding overlap: `2`
- Excluded finding ids: `M-23, M-44`
- V12 sweep screen: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/v12-sweeps/v2/2025-11-megapot-run-validation-fix-001.md`

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

### M-7 / `b_aiHuotWEfaTycI8agH9`
- Finding Title: Changing the payout calculator mid-drawing can zero or corrupt active payouts
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `payout-calculator-unsnapshotted`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The current payout calculator is read at settlement and claim time rather than being snapshotted for the active drawing. A future-oriented calculator rotation can therefore make the active drawing use uninitialized tier information or different rules, causing winners to receive incorrect or zero payouts.
- Code Evidence: `contracts/Jackpot.sol::setPayoutCalculator` replaces `payoutCalculator` globally, `_setNewDrawingState` snapshots tier info only on the current calculator, `_calculateDrawingUserWinnings` calls the live calculator, and `claimWinnings` later calls `payoutCalculator.getTierPayout`.

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

### M-15 / `kieHSGUnSmvztUc_OBnAx`
- Finding Title: Missing bonusball packing bounds can corrupt NFT ticket decoding
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `ticket-bitpacking-boundary`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The contract lets governance configure ball ranges that are within the exposed parameter types but outside the ticket bit-packing model. Because bitpacking boundaries are an explicit benchmark focus and the setters do not mark these values unsupported, this is a valid missing-validation bug rather than merely a malicious trusted-role action.
- Code Evidence: `contracts/Jackpot.sol::setBonusballMin` and range-derived drawing setup lack `normalBallMax + bonusball` bounds, while `TicketComboTracker.insert` and `unpackTicket` assume a packed bonus bit exists within the 256-bit ticket representation.

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

### H-48 / `x-waZ76GXiDz3-pI_Wxg2`
- Finding Title: Bonusball bit overflow can brick or mis-tier claims
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `ticket-bitpacking-boundary`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Bonusball values can exceed the bit position available after normal-ball packing, causing tracker counts and claim tier computation to disagree with the intended ticket contents. This can lock claims or produce incorrect tiers, though Medium is the more defensible severity absent a direct deterministic pool drain.
- Code Evidence: `contracts/lib/TicketComboTracker.sol::insert` and `countTierMatchesWithBonusball` shift by `_bonusball + _tracker.normalMax`, while `contracts/Jackpot.sol::_calculateTicketTierId` extracts the bonusball using `_ticketNumbers >> (_normalBallMax + 1)`.

### M-59 / `AmvuAyw8y2V0rkSpDDxav`
- Finding Title: Arbitrary bridge calls can leave allowance and steal surplus USDC
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bridge-stale-allowance`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: A malicious approved spender can use leftover allowance after the checked route or later when surplus is present, taking funds that were not part of the signer’s claim. This is incremental unauthorized impact against bridge-held USDC.
- Code Evidence: `contracts/JackpotBridgeManager.sol::_bridgeFunds` grants `approveTo` allowance, never clears it, and only checks the balance delta during the immediate route call.

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

### M-69 / `dTJQMBRcGvOjORilkVUlf`
- Finding Title: Bonusball ranges can exceed the packed ticket bit domain
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `ticket-bitpacking-boundary`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The setters and drawing initialization permit parameter combinations that fit in `uint8` but exceed the bit-packing assumptions used for tickets. Because bitpacking boundaries are explicitly in scope, this missing validation can produce Medium-impact claim or settlement corruption.
- Code Evidence: `contracts/Jackpot.sol::setBonusballMin` and `_setNewDrawingState` lack a `normalBallMax + bonusball <= 255` invariant, while `TicketComboTracker` shifts by that sum.

### M-73 / `FgRFBZujzUDHT5TlbsOr6`
- Finding Title: Provider rotation collisions append old SetRequests to overwritten pending entries
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-sequence-collision`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: `_storePendingRequest` overwrites scalar pending fields but appends request data into the existing dynamic array. A sequence collision after provider rotation can therefore mix old and new request arrays or callbacks, breaking settlement consumers.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::_storePendingRequest` assigns `requester` and `callbackData` on `pending[_sequence]` and then pushes into `request.setRequests` without first deleting any existing array.

### M-74 / `HCZnk6xd_GnFZCH0X04fe`
- Finding Title: Bonusball bounds are not constrained to the bit-packing domain
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `ticket-bitpacking-boundary`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The protocol exposes configurable ball bounds but omits the invariant needed by its packed ticket representation. This can corrupt stored combinations and claim calculations when bonusball positions exceed the available bit domain.
- Code Evidence: `contracts/Jackpot.sol::_setNewDrawingState` chooses the next bonusball max from LP sizing without checking the packed bit limit, and `TicketComboTracker` assumes that packed bonus bit can be safely shifted and recovered.

### M-76 / `fVsK-LAQ7IPQ6NH8i6z49`
- Finding Title: High bonusball ranges can lock settlement or claims through bit overflow
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `ticket-bitpacking-boundary`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The relevant parameter combinations are accepted by the contract but incompatible with the ticket bit layout. When reached, normal settlement or claim paths can revert or calculate tiers incorrectly, so the report has a real Medium-impact path.
- Code Evidence: `contracts/Jackpot.sol::_validateAndStoreTickets` accepts bonusballs up to the drawing max, while `TicketComboTracker.insert` and `Jackpot._calculateTicketTierId` depend on consistent bit placement.

### M-80 / `epV1g2oCGbgjEd1MKVJMs`
- Finding Title: NormalBallMax plus bonus range can corrupt claim calculations
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `ticket-bitpacking-boundary`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The protocol allows range configurations that make bonus bits overlap invalid positions in the packed ticket word. Claims then decode or compare the bonusball incorrectly, creating mis-tiering or locked winnings.
- Code Evidence: `contracts/Jackpot.sol::_calculateTicketTierId` decodes the bonusball by shifting the packed ticket by `_normalBallMax + 1`, while `TicketComboTracker` inserts the bonus bit at `_bonusball + normalMax`; no shared invariant keeps these safe.

### M-83 / `eU8MFp1NT2zpXY6vsm4RZ`
- Finding Title: High bonusball ranges can make settlement revert
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `ticket-bitpacking-boundary`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Settlement relies on counting tier matches from the packed ticket tracker. If the bonusball bit position exceeds the supported domain, that counting path can revert or produce invalid results and block jackpot progression.
- Code Evidence: `contracts/Jackpot.sol::_calculateDrawingUserWinnings` calls `TicketComboTracker.countTierMatchesWithBonusball`, which builds packed bitmasks using `_bonusball + _tracker.normalMax` without a safe upper bound.

### M-86 / `VumA1LcRqul120TjtxTRb`
- Finding Title: Rotating payoutCalculator can change unclaimed settled-ticket payouts
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `payout-calculator-live-claims`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Even after settlement, ticket claims read the live payout calculator rather than the calculator that stored the drawing’s tier payouts. A future calculator rotation can therefore break or reprice old unclaimed tickets.
- Code Evidence: `contracts/Jackpot.sol::claimWinnings` calls `payoutCalculator.getTierPayout(drawingId, tier)` at claim time, and `setPayoutCalculator` changes the global calculator without per-drawing binding.

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

### M-92 / `MZxhqjaBRZw38hY7UoPkd`
- Finding Title: normalBallMax plus bonusballMin can exceed the ticket packing domain
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `ticket-bitpacking-boundary`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The protocol lacks an invariant tying normal-ball and bonusball bounds to the 256-bit packed ticket layout. Accepted range updates can make future tickets impossible to encode, count, or decode safely.
- Code Evidence: `contracts/Jackpot.sol::setBonusballMin` and `_setNewDrawingState` do not validate the combined range, while `TicketComboTracker.insert` shifts by `_bonusball + _tracker.normalMax`.
