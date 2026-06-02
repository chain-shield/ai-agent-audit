# 2025-11-megapot Three-Shot Stage 3 run-validation-fix-001

Status: Complete
Benchmark report: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/2025-11-megapot/report/audit-report.md`
Benchmark source root: `/Users/apmfree/Desktop/Audit/2025-11-megapot-538649/2025-11-megapot`
Validation prompt: `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/prompts/validation-v2.md`

Mandatory benchmark docs:
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-11-megapot/2025-11-megapot-docs.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-11-megapot/2025-11-megapot-scope.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-11-megapot/2025-11-megapot-scope.txt`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/audit-docs/2025-11-megapot/2025-11-megapot-validation.md`
- `/Users/apmfree/Desktop/Audit/2025-11-megapot-538649/2025-11-megapot/README.md`
- `/Users/apmfree/Desktop/CHAIN SHIELD/ai-agent-audit/validation-three-shot/v12-checklist.md`

## Per-Finding Validation

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

### M-3 / `ghJ1xAP05g3yFJvWi6lh9`
- Finding Title: Jackpot entropy callbacks are not bound to the request that locked the drawing
- Decision: Needs Review
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium / Unclear
- Root Cause Family: `jackpot-unbound-entropy-request`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Safeguards`
- Checklist Gates Failed: `Exploitability Unclear`
- Detailed Reason: The code does not bind callbacks to the request created by `runJackpot`, so stale randomness can satisfy whatever drawing is currently locked. The remaining uncertainty is the practical path to have a stale request coexist with a later locked drawing without relying on trusted recovery actions, because `runJackpot` locks progression until a callback or owner intervention.
- Code Evidence: `contracts/Jackpot.sol::runJackpot` ignores the request identifier returned by `entropy.requestAndCallbackScaledRandomness`, and `scaledEntropyCallback(bytes32,bytes32[] memory)` ignores its first argument and settles `currentDrawingId` solely under `onlyEntropy` and `jackpotLock`.

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

### M-5 / `WoebK6L4bf9CMQmO1z0JH`
- Finding Title: Changing entropy provider while a drawing is pending can block settlement
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-provider-rotation-pending`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The entropy-provider setter is documented as affecting future requests, but the pending request mapping is keyed only by sequence and provider callbacks after a rotation can be rejected or overwritten. This can strand an active randomness request and prevent a locked drawing from settling through normal operation.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::setEntropyProvider` changes the provider without guarding pending requests, while `requestAndCallbackScaledRandomness` stores pending state by `sequence` and `entropyCallback` only loads `pending[sequence]` before executing the stored callback.

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

### M-11 / `PPrrZKotDc61i9N4tnTCl`
- Finding Title: Entropy rotation can strand the original pending drawing request
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `jackpot-entropy-rotation-pending`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The active randomness request is not tied to the entropy address that created it, and rotating the global entropy pointer immediately changes callback authorization. A valid callback from the old entropy source will revert, leaving the jackpot locked.
- Code Evidence: `contracts/Jackpot.sol::setEntropy` updates `entropy` without pending-request checks, while `scaledEntropyCallback` requires `onlyEntropy` and is responsible for calling `_setNewDrawingState` after settlement.

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

### M-13 / `twUQAbk8uX7jwTke8KIfR`
- Finding Title: Scaled entropy accepts unfulfillable SetRequests that revert at callback
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `entropy-request-feasibility`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause`
- Checklist Gates Failed: `Impact`
- Detailed Reason: The provider accepts malformed no-replacement or range requests that can only fail during callback-time drawing. This is a real API validation defect, but the in-scope Jackpot requests are fixed and feasible, so the demonstrated impact is mostly self-inflicted or integration-level rather than a material Jackpot H/M issue.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::_validateRequests` checks non-empty requests, min/max ordering, and sample count, but not no-replacement feasibility; callback-time drawing then routes through `FisherYatesWithRejection.draw`, which reverts when `count` exceeds the available range.

### M-14 / `RJM_fI7cXw-dK2wVfx0Xs`
- Finding Title: Mid-drawing payout calculator updates can settle with uninitialized tiers
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `payout-calculator-unsnapshotted`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Active drawings do not pin the calculator used for tier snapshots, settlement, and claims. Replacing the calculator before settlement makes the active drawing read unset `drawingTierInfo`, which can suppress payouts that should have been determined under the drawing’s original payout configuration.
- Code Evidence: `contracts/Jackpot.sol::setPayoutCalculator` changes the global pointer, `GuaranteedMinimumPayoutCalculator.calculateAndStoreDrawingUserWinnings` reads `drawingTierInfo[_drawingId]`, and `setDrawingTierInfo` is only called for new drawings in `_setNewDrawingState`.

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

### M-16 / `v2sgzi0QYHrqs1cThqt8a`
- Finding Title: Bridge signatures have no nonce or deadline
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bridge-signature-no-expiry`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Authorizations for ticket withdrawals and winnings claims remain valid forever and cannot be revoked on-chain. A stale signature can be executed after route risk, recipient intent, or ticket value changes, which is a normal EIP-712 replay class in the in-scope bridge manager.
- Code Evidence: `contracts/JackpotBridgeManager.sol::CLAIM_WINNINGS_TYPEHASH`, `CLAIM_TICKETS_TYPEHASH`, `hashClaimWinningsRequest`, and `hashClaimTicketsRequest` contain no nonce/deadline, and the claim functions do not consume replay state.

### M-17 / `EjJJ7I_tm_zeL_HoMTLjP`
- Finding Title: Smart-wallet ticket owners cannot authorize bridge claims
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bridge-erc1271-unsupported`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The bridge supports arbitrary recipients at purchase time but only supports ECDSA at claim time. ERC-1271 owners can receive bridged ticket ownership in the bridge accounting, then be unable to produce a recoverable EOA signature for the stored owner address.
- Code Evidence: `contracts/JackpotBridgeManager.sol::buyTickets` records `_recipient` without excluding contracts, while `_validateTicketOwnership` compares `ticketOwner[_ticketId]` to `ECDSA.recover(...)` and has no `isValidSignature` branch.

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

### M-21 / `PMFE9hYicCVTJCz09VHk6`
- Finding Title: Perpetual bridge EIP-712 signatures can be replayed after intent changes
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bridge-signature-no-expiry`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The bridge has no nonce, deadline, or cancellation mechanism for signed claim requests, so any valid signature remains usable indefinitely. That creates a credible stale-authorization path for bridge operations whose economic value or desired route changes over time.
- Code Evidence: `contracts/JackpotBridgeManager.sol::hashClaimWinningsRequest` and `hashClaimTicketsRequest` hash only the request payload arrays and route fields, and neither `claimWinnings` nor `claimTickets` records signature consumption.

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

### M-23 / `XboZ6rstW75a7OfO7LUPg`
- Finding Title: Pending LP deposits can round to zero shares after accumulator inflation
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `lp-zero-share-inflation`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Deposits remain pending until consolidation and are converted to shares by floor division against the historical accumulator. If the accumulator has grown enough, a positive pending deposit can be consolidated into zero shares and the deposit record is deleted, causing an LP loss through normal protocol state changes.
- Code Evidence: `contracts/JackpotLPManager.sol::_consolidateDeposits` computes `shares = amount * 1e18 / drawingAccumulator[lastDeposit.drawingId]` and deletes `lastDeposit`; `processDrawingSettlement` can increase the accumulator as LP value changes.

### M-24 / `_uzGLqresOu7Dk6U-KMjO`
- Finding Title: Missing referral rewards are booked to the active drawing
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `no-referral-current-drawing`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The contract uses the current drawing ID when redirecting no-referral rewards to LP earnings, even though the winning ticket belongs to an older settled drawing. This lets claim timing affect which LP cohort receives value.
- Code Evidence: `contracts/Jackpot.sol::claimWinnings` derives winnings from `ticketInfo.drawingId`, but `_payReferrersWinnings` adds the no-referral amount to `drawingState[currentDrawingId].lpEarnings`.

### M-25 / `-HhMgy6frh49i966ZDULK`
- Finding Title: Full uint256 entropy ranges overflow in scaled randomness
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `entropy-range-overflow`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause`
- Checklist Gates Failed: `Impact`
- Detailed Reason: A request with `min = 0` and `max = type(uint256).max` is accepted by validation but overflows when computing range size. The bug is real in the public provider API, but the Jackpot integration does not use attacker-controlled full-width ranges, so the demonstrated impact is not H/M for this benchmark.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::_validateRequests` allows any `_max >= _min`, and `_drawWithReplacement` computes `_maxRange - _minRange + 1`, which overflows for the full uint256 domain.

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

### M-28 / `4J7yK8OUsFrvGAhCtYqjh`
- Finding Title: Full uint256 randomness ranges are accepted and overflow
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `entropy-range-overflow`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause`
- Checklist Gates Failed: `Impact`
- Detailed Reason: The provider validation omits a safe range-size check and accepts a full uint256 interval that cannot be represented after adding one. This is a valid low-level bug, but current Jackpot calls use bounded ball ranges and do not expose this H/M path to ordinary jackpot users.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::_validateRequests` checks only ordering and sample count, while `_drawWithReplacement` computes `rangeSize = _maxRange - _minRange + 1`.

### M-29 / `qYfnKos-ZnhIgN86d2poe`
- Finding Title: Bridge claim routes can be executed with stale nonce-less signatures
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bridge-signature-no-expiry`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The EIP-712 request does not carry freshness constraints, so an old signed bridge route can be submitted after the signer’s intended route or risk assumptions have changed. This is a replay/freshness defect in the audited bridge manager, not a duplicate report-local issue.
- Code Evidence: `contracts/JackpotBridgeManager.sol::hashClaimWinningsRequest` includes ticket IDs and route fields but no nonce/deadline, and `claimWinnings` performs no consumed-signature tracking.

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

### L-31 / `_m-C_2EejQQtxrkGrcev0`
- Finding Title: Burned ticket metadata remains readable as if active
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `burned-ticket-stale-metadata`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause`
- Checklist Gates Failed: `Impact`
- Detailed Reason: Burning a ticket does not delete its metadata, so helper views can keep returning stale ticket information. The issue is real but does not enable double claims because ownership and ERC-721 burn checks still protect the state-changing paths.
- Code Evidence: `contracts/JackpotTicketNFT.sol::burnTicket` only calls `_burn`, while `getTicketInfo` returns `tickets[_ticketId]` without checking token existence.

### M-32 / `Nlj-UYTyD4z1UkRYuxpyz`
- Finding Title: Invalid full-range randomness requests can revert later
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `entropy-range-overflow`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause`
- Checklist Gates Failed: `Impact`
- Detailed Reason: The request validator accepts ranges that overflow during scaling, causing later callback failure. The Jackpot integration’s own requests are bounded to normal and bonus ball domains, so this remains a low/QA provider validation issue for the benchmark.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::_validateRequests` lacks a range-size overflow check, and `_drawWithReplacement` adds one to the max-min difference.

### M-33 / `oGmvAj73gSj07W0Jh5QNn`
- Finding Title: Impossible no-replacement entropy requests are accepted
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `entropy-request-feasibility`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause`
- Checklist Gates Failed: `Impact`
- Detailed Reason: Requests asking for more unique samples than the range contains are not rejected until callback-time drawing. The defect is in scope for the provider API but lacks a demonstrated material impact on Jackpot’s fixed valid randomness requests.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::_validateRequests` never checks `numSamples <= max - min + 1` for no-replacement requests, and `contracts/lib/FisherYatesWithRejection.sol::draw` enforces that condition later.

### M-34 / `DTCE_kPYivgorgy9AIkkR`
- Finding Title: Missing entropy feasibility validation permits callback-time reverts
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `entropy-request-feasibility`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause`
- Checklist Gates Failed: `Impact`
- Detailed Reason: The provider accepts requests that cannot be fulfilled safely and only discovers the problem during randomness delivery. This is a valid validation gap, but without a current Jackpot path to submit attacker-controlled invalid requests it is not a medium-severity protocol loss.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::_validateRequests` omits no-replacement sample-cap and overflow feasibility checks; callback drawing then calls `_drawWithReplacement` or `FisherYatesWithRejection.draw`.

### M-35 / `et7UofFB1msp9Vt7VVQ2v`
- Finding Title: Stale entropy callbacks can satisfy the wrong locked drawing
- Decision: Needs Review
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium / Unclear
- Root Cause Family: `jackpot-unbound-entropy-request`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Safeguards`
- Checklist Gates Failed: `Exploitability Unclear`
- Detailed Reason: The callback lacks request identity and settles the currently locked drawing, so the root bug exists. The uncertain part is whether an attacker can produce a stale callback for a later drawing without owner recovery or another trusted intervention, because the jackpot remains locked while a request is outstanding.
- Code Evidence: `contracts/Jackpot.sol::runJackpot` does not store the returned entropy sequence or request hash, and `scaledEntropyCallback` ignores its `bytes32` argument while using `currentDrawingId`.

### M-36 / `N8k9IZrZQviCeyyaNHOQ4`
- Finding Title: Entropy provider changes reject valid callbacks for pending drawings
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `jackpot-entropy-rotation-pending`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: A future-oriented entropy migration can immediately invalidate the only legitimate sender for an active pending callback. Since mid-drawing admin/global changes are in scope, this is a live liveness bug rather than a trusted-role-only issue.
- Code Evidence: `contracts/Jackpot.sol::setEntropy` changes the callback authority used by `onlyEntropy`, and `scaledEntropyCallback` cannot run from the old entropy contract once the pointer is updated.

### M-37 / `dFlfXpkDW49CviZDQWS2A`
- Finding Title: Residual bridge approvals can drain future USDC
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bridge-stale-allowance`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: After a successful bridge claim, the approved spender can retain allowance and later transfer bridge-manager USDC unrelated to the original signer. That creates an unauthorized impact against future claim funds or surplus balances.
- Code Evidence: `contracts/JackpotBridgeManager.sol::_bridgeFunds` approves `approveTo` for `claimedAmount`, performs arbitrary calldata, checks only the balance delta, and never resets the allowance to zero.

### H-38 / `o3d2w1GwCZw3kuQKHepDU`
- Finding Title: Opaque bridge calldata can route a signer’s winnings to an attacker
- Decision: Invalid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Low / Unclear
- Root Cause Family: `bridge-delegated-route`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists`
- Checklist Gates Failed: `Exploitability, Trusted/Delegated Authority, Impact`
- Detailed Reason: The arbitrary route surface exists, but the described theft of the signer’s own claimed USDC depends on the user signing calldata that sends those funds to the attacker or misunderstanding an opaque route. The bridge has no separate on-chain intended recipient for winnings, so this is victim-authorized route misuse rather than incremental unauthorized protocol impact.
- Code Evidence: `contracts/JackpotBridgeManager.sol::hashClaimWinningsRequest` includes the bridge request fields signed by the ticket owner, and `_bridgeFunds` enforces only that exactly `claimedAmount` leaves the manager during the signed route call.

### H-39 / `yx3YULUi7UPyOA6c8ez2L`
- Finding Title: Arbitrary bridge calls can transfer other users' custodied ticket NFTs
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: High
- Root Cause Family: `bridge-arbitrary-call-nft-theft`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: A claimant’s signed route can execute from the bridge manager, which is also the legal ERC-721 owner of all bridged tickets. The USDC delta check does not protect those NFTs, so a claimant can steal tickets belonging to other bridge users.
- Code Evidence: `contracts/JackpotBridgeManager.sol::buyTickets` holds NFTs in the manager and tracks logical ownership in `ticketOwner`; `_bridgeFunds` permits arbitrary external calls and does not block `JackpotTicketNFT.transferFrom` from the manager.

### M-40 / `JjYFCeoUYvcJcIX6_pK_0`
- Finding Title: Invalid no-replacement randomness requests are not rejected up front
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `entropy-request-feasibility`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause`
- Checklist Gates Failed: `Impact`
- Detailed Reason: The provider accepts requests that `FisherYatesWithRejection` cannot satisfy. This should be validated at request time, but the exploit path shown is not material to the current Jackpot flow because Jackpot’s own no-replacement normal-ball request is feasible under normal settings.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::_validateRequests` omits the `numSamples <= rangeSize` check, while `contracts/lib/FisherYatesWithRejection.sol::draw` reverts if the sample count exceeds the range.

### M-41 / `JmNO-knM1BokEuNBDWxTv`
- Finding Title: Bridge ECDSA validation excludes contract wallet owners
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bridge-erc1271-unsupported`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Contract wallet recipients are accepted as ticket owners but cannot pass the ECDSA recovery check used for all bridge claims. This can lock tickets or winnings for a normal smart-wallet user of the bridge.
- Code Evidence: `contracts/JackpotBridgeManager.sol::_validateTicketOwnership` compares `ticketOwner[_ticketId]` to an ECDSA recovered address and never calls ERC-1271 `isValidSignature`.

### M-42 / `bEM7bCscGCIhiKqM40GUJ`
- Finding Title: Stale bridge signatures are replayable indefinitely
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bridge-signature-no-expiry`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The bridge treats any historically valid signature as valid forever. For tickets whose value changes after drawing or bridge routes whose safety changes, perpetual authorization is a credible medium-severity replay risk.
- Code Evidence: `contracts/JackpotBridgeManager.sol::hashClaimWinningsRequest` and `hashClaimTicketsRequest` omit nonce, deadline, and cancellation fields, and neither claim path writes replay state.

### M-43 / `anKPmMCB1WwZFh-pgeeD3`
- Finding Title: No-referral winnings are credited to the wrong drawing's LP earnings
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `no-referral-current-drawing`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The no-referral fallback uses the active drawing rather than the ticket’s settled drawing, so normal claim timing can shift earnings between LP cohorts. This is an accounting bug with real value movement.
- Code Evidence: `contracts/Jackpot.sol::_payReferrersWinnings` writes to `drawingState[currentDrawingId].lpEarnings`, while `claimWinnings` separately obtains the winning ticket’s `ticketInfo.drawingId`.

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

### M-45 / `Io61Qmv4_cRXazVfiFprt`
- Finding Title: Permissionless LP deposits can block intended pool-cap reductions
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `lp-cap-frontrun`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The cap setter rejects values below live pool plus pending deposits, so an LP can make a planned lower cap unapplyable by depositing first. Under the round instructions, this permissionless state obstruction should be judged on its merits even though governance sets the cap.
- Code Evidence: `contracts/JackpotLPManager.sol::setLPPoolCap` requires `_lpPoolCap >= lpPoolTotal + pendingDeposits`, and `processDeposit` lets ordinary LPs increase the pending side up to the existing cap.

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

### M-47 / `DY8_8XaFt5YlWy8JdsHMf`
- Finding Title: No-referral fallback credits active drawing LPs
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `no-referral-current-drawing`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: When a winning claim has no referral scheme, the fallback reward is not tied to the ticket’s drawing. Claimants can affect which active LP accounting period receives the value by choosing when to claim.
- Code Evidence: `contracts/Jackpot.sol::_payReferrersWinnings` uses `drawingState[currentDrawingId].lpEarnings` instead of `drawingState[_ticketDrawingId]` or a settlement-specific accounting bucket.

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

### M-49 / `uvZ6f7Zqp3uFwf09w21c3`
- Finding Title: runJackpot callers are not reimbursed for entropy fees
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `keeper-incentive-gap`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause`
- Checklist Gates Failed: `Exploitability, Impact`
- Detailed Reason: `runJackpot` requires a permissionless caller to pay the entropy fee and does not reimburse them, creating a liveness incentive weakness. The finding lacks a concrete attacker-controlled harm path beyond under-incentivized progress, so it is valid as low/QA rather than Medium.
- Code Evidence: `contracts/Jackpot.sol::runJackpot` requires `msg.value >= fee` and forwards value to `entropy.requestAndCallbackScaledRandomness`, but no repayment or keeper reward is credited to the caller.

### M-50 / `4BnVTywm1gum72Soijtty`
- Finding Title: Unpaid jackpot execution leaves expired drawings open for late buys
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `late-ticket-purchase`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Because purchases do not close at `drawingTime`, failure or delay in paying to run the jackpot extends the buy window. Attackers can submit tickets after the advertised cutoff until someone executes `runJackpot`.
- Code Evidence: `contracts/Jackpot.sol::buyTickets` routes to `_validateBuyTicketInputs`, which does not compare the current timestamp to the drawing time; only `runJackpot` enforces the timestamp and sets `jackpotLock`.

### M-52 / `ZSK-vXUjmoLQ2ZqQGSEpm`
- Finding Title: Unbounded entropy ranges and batch sizes can cause gas exhaustion
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `entropy-unbounded-work`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause`
- Checklist Gates Failed: `Impact`
- Detailed Reason: The provider does not cap request count, sample count, or range size before accepting a paid request. This can make a callback run out of gas, but current Jackpot requests are small and protocol-fixed, so the demonstrated impact remains low/QA for this benchmark.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::_validateRequests` lacks upper bounds, and `contracts/lib/FisherYatesWithRejection.sol::draw` allocates an array of `rangeSize` and iterates over the requested draw count.

### M-53 / `TkptkGk_kcgTRVqNNHeX8`
- Finding Title: Reverting requester callback rolls back pending cleanup
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `entropy-callback-cleanup`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause`
- Checklist Gates Failed: `Impact`
- Detailed Reason: If the downstream callback reverts, the delete of pending request state reverts too. That is a real robustness issue for the public provider, but for Jackpot it only becomes material through another callback-reverting bug and is not independently H/M.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::entropyCallback` deletes `pending[sequence]` before the external callback, but the whole transaction reverts if the callback fails, preserving the pending entry.

### M-54 / `cn7PMWjyPYAdapY1lFl5n`
- Finding Title: Old or leaked bridge signatures remain usable forever
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bridge-signature-no-expiry`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The bridge cannot distinguish fresh user intent from an old valid authorization. This allows stale execution of ticket or winnings claims after the user would reasonably expect the authorization to be expired or cancelable.
- Code Evidence: `contracts/JackpotBridgeManager.sol::hashClaimTicketsRequest` and `hashClaimWinningsRequest` omit nonce/deadline fields, and no mapping of consumed digests is updated in either claim function.

### L-55 / `JGgR2xyH0uL0CDYaPTq9A`
- Finding Title: claimWinnings leaves burned tickets in bridge accounting
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `bridge-stale-ticket-owner`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause`
- Checklist Gates Failed: `Impact`
- Detailed Reason: After winnings are claimed, the bridge does not clear `ticketOwner` for burned tickets. The stale accounting can confuse later reads or attempts, but it does not enable a second payout because the NFT is burned and the jackpot claim has already consumed the ticket.
- Code Evidence: `contracts/JackpotBridgeManager.sol::claimWinnings` validates ownership and calls `jackpot.claimWinnings` but does not delete `ticketOwner`; `claimTickets` uses `_updateTicketOwnership`, which deletes ownership only for ticket withdrawals.

### H-56 / `ScHOGndBOIJkw9PUyhBrl`
- Finding Title: Arbitrary bridge call can steal other bridge users' NFTs
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: High
- Root Cause Family: `bridge-arbitrary-call-nft-theft`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: A bridge claimant can execute calldata from the manager, and the manager is the ERC-721 holder for all bridged tickets. That lets the claimant transfer unrelated custodied NFTs while still satisfying the USDC balance check for their own claim.
- Code Evidence: `contracts/JackpotBridgeManager.sol::_bridgeFunds` permits arbitrary `to.call(data)` and checks only USDC balance deltas; tickets are held by the manager after `buyTickets` and can be moved by ERC-721 calls from that address.

### M-57 / `xY6OOMdx8rewLd4XafLh_`
- Finding Title: Opaque bridge calldata can send claimed winnings to an unexpected recipient
- Decision: Invalid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Low / Unclear
- Root Cause Family: `bridge-delegated-route`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists`
- Checklist Gates Failed: `Exploitability, Trusted/Delegated Authority, Impact`
- Detailed Reason: The route is part of what the ticket owner signs, and the claimed USDC is the signer’s own authorized bridge output. Without a separate on-chain promised recipient or amount mismatch, the harmful path depends on the user signing a malicious or misunderstood route.
- Code Evidence: `contracts/JackpotBridgeManager.sol::hashClaimWinningsRequest` binds the `BridgeRequest` fields into the EIP-712 digest, and `_bridgeFunds` only enforces that the signed route removes exactly the claim amount.

### H-58 / `I7FqeqgXJk0H66gi0fIkj`
- Finding Title: Signed route can approve one address while calldata transfers winnings elsewhere
- Decision: Invalid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Low / Unclear
- Root Cause Family: `bridge-delegated-route`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists`
- Checklist Gates Failed: `Exploitability, Trusted/Delegated Authority, Impact`
- Detailed Reason: Although the bridge allows arbitrary calldata, the described loss is the signer’s own claimed USDC following calldata they authorized in the signed bridge request. This is not a protocol-bypass theft unless the report shows harm to assets beyond the signed claim amount, which this finding does not.
- Code Evidence: `contracts/JackpotBridgeManager.sol::_bridgeFunds` executes signed route calldata and checks the manager’s USDC decrease equals `claimedAmount`; the route fields are included in `hashClaimWinningsRequest`.

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

### M-61 / `ca2NbWJQODDuT28JrEfvS`
- Finding Title: Large no-replacement ranges can exhaust callback gas
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `entropy-unbounded-work`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause`
- Checklist Gates Failed: `Impact`
- Detailed Reason: The no-replacement implementation allocates memory proportional to the requested range, and the provider accepts large ranges. The issue is real for the public provider surface but not materially reachable through Jackpot’s fixed small draw requests.
- Code Evidence: `contracts/lib/FisherYatesWithRejection.sol::draw` allocates `new uint256[](rangeSize)`, while `ScaledEntropyProvider._validateRequests` imposes no maximum range size.

### M-62 / `62WtuEiLCFmhhhAkFFXso`
- Finding Title: Unbounded entropy request arrays can cause callback DoS
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `entropy-unbounded-work`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause`
- Checklist Gates Failed: `Impact`
- Detailed Reason: The provider accepts arbitrarily many set requests and samples, creating potential gas exhaustion for direct users. Jackpot itself submits a fixed two-request batch, so this remains a low/QA input-bound issue for the audited protocol flow.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::_validateRequests` iterates the request array but does not cap its length or total samples before storing the pending request.

### M-63 / `VYM7kgJV8rdR7r9C3Ouaf`
- Finding Title: Invalid no-replacement entropy requests are accepted
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `entropy-request-feasibility`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause`
- Checklist Gates Failed: `Impact`
- Detailed Reason: The contract should reject no-replacement requests that cannot be fulfilled, but it currently defers failure to callback time. The current Jackpot integration does not let an attacker feed such invalid requests into jackpot settlement.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::_validateRequests` does not enforce sample feasibility, and `FisherYatesWithRejection.draw` later requires enough values in the range.

### M-64 / `qTFiUtYrdGZ5IC_nE_NzD`
- Finding Title: Full uint256 range requests overflow during scaling
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `entropy-range-overflow`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause`
- Checklist Gates Failed: `Impact`
- Detailed Reason: A full-width range passes request validation but overflows while computing the inclusive range size. This is a provider correctness issue, not a demonstrated medium-severity attack on Jackpot’s bounded randomness usage.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::_drawWithReplacement` computes `_maxRange - _minRange + 1`, and `_validateRequests` lacks a guard for that overflow case.

### M-65 / `mrBG0kQc_1aaDkHc5rWc-`
- Finding Title: Missing entropy request bounds allow malformed low-impact requests
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `entropy-request-validation`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause`
- Checklist Gates Failed: `Impact`
- Detailed Reason: The combined missing bounds and feasibility checks are valid bugs in the entropy provider’s public API, but the report itself does not establish a material H/M effect on Jackpot. Current Jackpot requests are small, fixed, and normally feasible.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::_validateRequests` omits upper bounds, no-replacement feasibility, and full-range overflow checks before `_storePendingRequest`.

### L-66 / `ECun13eKcbs5_VSJ_GnnH`
- Finding Title: Burned NFT metadata remains active-looking
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `burned-ticket-stale-metadata`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause`
- Checklist Gates Failed: `Impact`
- Detailed Reason: Burned ticket metadata remains readable and may mislead off-chain consumers, but the state-changing claim path is protected by burn/ownership checks. This is a low-severity data hygiene issue.
- Code Evidence: `contracts/JackpotTicketNFT.sol::burnTicket` does not delete `tickets[_tokenId]`, and `getTicketInfo` returns that stored struct directly.

### L-67 / `NmeIslHIena89r8c9Tp8O`
- Finding Title: getUserTickets is unbounded and can become unusable
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `unbounded-view-helper`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause`
- Checklist Gates Failed: `Impact`
- Detailed Reason: The helper loops over all tickets ever bought and can become too expensive for on-chain or RPC consumers. Because it is a view helper and does not block core buy, draw, or claim state transitions, the impact is low/QA.
- Code Evidence: `contracts/JackpotTicketNFT.sol::getUserTickets` iterates from `1` to `totalTicketsBought` and checks ownership for every ticket without pagination.

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

### M-70 / `JNirY2A5L7N-4sAPEQSSD`
- Finding Title: Entropy provider rotation can collide sequence numbers and overwrite pending state
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-sequence-collision`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Pyth entropy sequences are not namespaced by provider in the pending mapping. After provider rotation, a new sequence can reuse an old number and overwrite a pending request, violating the documented future-only expectation for provider changes.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::pending` is keyed by `uint64`, `setEntropyProvider` swaps the provider, and `_storePendingRequest` writes `pending[_sequence]` without collision checks.

### M-71 / `qQy9NQ79-RRyMstIQufsw`
- Finding Title: Provider sequence collisions can misroute entropy callbacks
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-sequence-collision`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Since pending requests are identified only by sequence, an old provider callback and a new provider callback can refer to the same storage slot. That can misroute randomness or strand the original Jackpot callback after an otherwise legitimate provider migration.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::entropyCallback` loads `pending[sequence]` and ignores the provider argument for lookup, while `_storePendingRequest` overwrites the slot on new requests.

### M-72 / `J1Dqv-7r4rRmkAq_ocKmr`
- Finding Title: Bonusball bit positions at or above 256 break ticket accounting
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `ticket-bitpacking-boundary`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Valid-looking governance parameters can make the bonus bit position fall outside the 256-bit ticket word, causing inserts, counts, and decodes to lose or misinterpret the bonusball. That is a supported-parameter validation bug with material claim impact.
- Code Evidence: `contracts/lib/TicketComboTracker.sol::insert` and `countTierMatchesWithBonusball` compute `1 << (_bonusball + _tracker.normalMax)`, but no caller enforces that the shift is below 256.

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

### M-77 / `YrzefVI2gaRcji1F4O2lj`
- Finding Title: Entropy provider rotation can overwrite pending Jackpot randomness
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-sequence-collision`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The pending request namespace does not include provider identity, so a new provider can reuse the same sequence number and overwrite the Jackpot’s outstanding request. That can strand or misdirect settlement after a provider migration that should have affected only future requests.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::setEntropyProvider` changes `entropyProvider`, and `_storePendingRequest` overwrites `pending[_sequence]` for any subsequent request with the same sequence.

### M-78 / `uxV4GUomOSDggeWGNWLva`
- Finding Title: Provider rotation sequence collision can strand Jackpot settlement
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-sequence-collision`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: A sequence collision after provider rotation can cause the active Jackpot request to be overwritten by a later request. The old callback then uses the wrong pending data or no useful data, leaving the drawing unable to settle normally.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::entropyCallback` resolves callbacks through `pending[sequence]`, and `_storePendingRequest` does not reject an already-populated sequence slot.

### M-79 / `Eo7DO9DiVWIGLlFfnfy0D`
- Finding Title: Pending entropy requests are keyed only by sequence number
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-sequence-collision`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Sequence-only pending state is insufficient across provider changes, because the same sequence can represent different upstream requests. This can overwrite or misroute pending callbacks and affect Jackpot settlement liveness.
- Code Evidence: `contracts/ScaledEntropyProvider.sol` declares `mapping(uint64 => PendingRequest) pending` and `entropyCallback(uint64 sequence, address provider, bytes32 randomNumber)` does not use `provider` for pending lookup.

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

### M-81 / `8enSiBNKwJR94z-IbBs8s`
- Finding Title: Bonus bit positions at 256 can create false bonus-match behavior
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `ticket-bitpacking-boundary`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: When the computed bonus bit is outside the 256-bit range, the tracker and claim code no longer encode the same ticket contents. This can create false negatives or false positives in bonus matching depending on the path.
- Code Evidence: `contracts/lib/TicketComboTracker.sol::countTierMatchesWithBonusball` constructs `winningTicket` with `1 << (_bonusball + _tracker.normalMax)`, and `Jackpot._calculateTicketTierId` separately derives bonus matches from the packed integer.

### M-82 / `c1LUgH768vdleCQv2vutO`
- Finding Title: High bonusball configurations can produce false winning claims
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `ticket-bitpacking-boundary`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Because packed ticket encoding is not bounded, high bonusball values can make the tracker’s stored count and claim-time tier calculation diverge. That can overpay or underpay users relative to the intended winning combination.
- Code Evidence: `contracts/Jackpot.sol::_validateAndStoreTickets` mints tickets after tracker insertion, and `_calculateTicketTierId` later computes tier from the same packed number without validating the bonusball bit domain.

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

### M-84 / `up_Ag7R-61tUyZTohHm-S`
- Finding Title: Entropy address rotation rejects pending callbacks
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `jackpot-entropy-rotation-pending`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The Jackpot does not snapshot the entropy source for an outstanding request. Updating the entropy address before fulfillment makes the old source unauthorized and can permanently block the active drawing’s callback.
- Code Evidence: `contracts/Jackpot.sol::setEntropy` updates the global `entropy` reference, and `onlyEntropy` gates `scaledEntropyCallback` against the current reference rather than the request’s original source.

### M-85 / `z6UFaJb3e9UR2WAFVsTsW`
- Finding Title: Provider sequence collisions can overwrite outstanding entropy callbacks
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-sequence-collision`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Rotating providers without namespacing or draining pending requests lets a later request overwrite an earlier sequence. That violates request isolation and can affect Jackpot settlement liveness.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::_storePendingRequest` writes to `pending[_sequence]`, and `entropyCallback` later dispatches solely by `_sequence`.

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

### M-87 / `dUualzZdMcwOLTWqMmzY9`
- Finding Title: Provider rotation collision can misdeliver pending entropy
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-sequence-collision`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Pending request identity excludes provider identity, so callbacks after rotation can be associated with the wrong request. This can misdeliver randomness or strand the Jackpot request.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::pending` is keyed only by `uint64 sequence`, while the `provider` parameter in `entropyCallback` is not used to locate or validate the pending request.

### M-88 / `SIZoP2wmWR2-DqOorjm21`
- Finding Title: Payout calculator rotation can change old settled claims
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `payout-calculator-live-claims`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Settled drawing payouts are not resolved through a calculator address stored with that drawing. Users who claim after a calculator update can receive zero, revert, or receive different amounts from users who claimed before the update.
- Code Evidence: `contracts/Jackpot.sol::claimWinnings` uses the current `payoutCalculator` for historical `drawingId`, and `GuaranteedMinimumPayoutCalculator` stores tier payout state internally per calculator instance.

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

### M-93 / `PR2G588QO-Kwr3c5Fhe4w`
- Finding Title: Provider sequence collision can overwrite pending Jackpot randomness
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-sequence-collision`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The pending request mapping is not protected from sequence reuse across provider migrations. A later request can overwrite the Jackpot’s pending callback data and prevent correct settlement.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::_storePendingRequest` overwrites `pending[_sequence]`, and `Jackpot.runJackpot` relies on that provider callback to call `Jackpot.scaledEntropyCallback`.

### M-94 / `hwZlFmHk1XCY6j0kVxNjK`
- Finding Title: Payout calculator rotation reprices settled tickets
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `payout-calculator-live-claims`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Settlement does not bind each drawing to the calculator that stored its payouts. Claims after a calculator update can use an unrelated calculator instance, changing old ticket payouts.
- Code Evidence: `contracts/Jackpot.sol::claimWinnings` obtains `tierPayout` from the live `payoutCalculator`, while `setPayoutCalculator` can replace that object after the drawing has already settled.

### L-95 / `7Dgn4Kc0erelDLEzHJMPn`
- Finding Title: Low normalBallMax can brick payout calculation
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `normal-range-underflow`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Although labeled low in the report, the underlying bug can block settlement for a drawing initialized with an accepted but mathematically invalid normal-ball range. That is a Medium liveness impact if reached through the normal configuration surface.
- Code Evidence: `contracts/GuaranteedMinimumPayoutCalculator.sol::_calculateTierTotalWinningCombos` subtracts from `_normalMax` and calls `Combinations.choose`, whose asserts fail when the configured normal range is too small.

### M-96 / `4N5SxmjaZ5ZFkldy20qul`
- Finding Title: Rotating payoutCalculator uses the wrong calculator for old tickets
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `payout-calculator-live-claims`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The claim path reads historical drawing payout data through whichever calculator is currently configured. A calculator intended for future drawings can therefore break claims for old tickets.
- Code Evidence: `contracts/Jackpot.sol::setPayoutCalculator` changes the global calculator, and `claimWinnings` uses `payoutCalculator.getTierPayout(ticketInfo.drawingId, tier)` without storing a per-drawing calculator address.

### M-97 / `c1taCy-rUpLHmD9KeiD5V`
- Finding Title: Entropy address rotation can reject the pending request callback
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `jackpot-entropy-rotation-pending`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The Jackpot uses the current entropy address for callback authorization rather than the address that created the pending request. Updating it during a pending drawing can make the legitimate callback revert and stall the game.
- Code Evidence: `contracts/Jackpot.sol::onlyEntropy` compares `msg.sender` to `address(entropy)`, and `setEntropy` has no pending request or lock guard.

### M-98 / `bqDE4tP4lSLVtXr7da9Tk`
- Finding Title: Changing Jackpot.entropy after runJackpot bricks settlement
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `jackpot-entropy-rotation-pending`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: After `runJackpot` locks a drawing, the pending callback must come from the entropy address that was used for that request. The contract instead authorizes only the mutable current entropy address, so a rotation before fulfillment can strand the locked drawing.
- Code Evidence: `contracts/Jackpot.sol::runJackpot` calls the current entropy provider and locks the jackpot, while `setEntropy` can later replace the address that `scaledEntropyCallback` accepts.

### M-100 / `pxtxFK-1LCZ2MRtDy6Fgn`
- Finding Title: Provider sequence collision can replay randomness into the wrong request
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-sequence-collision`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Because pending request state is sequence-only, a callback for an old provider sequence can be applied to the pending data for a different request after rotation. That breaks request isolation and can missettle or stall the consumer.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::entropyCallback` receives `provider` but dispatches using only `pending[sequence]`; `_storePendingRequest` permits later writes to the same sequence slot.

### M-101 / `OyeL2vMtI2i2rZz06wHLH`
- Finding Title: Updating payout calculator breaks settled claims
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `payout-calculator-live-claims`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Settled tickets do not carry the payout calculator that calculated their drawing. A later calculator update can make those tickets read empty or different tier-payout data at claim time.
- Code Evidence: `contracts/Jackpot.sol::claimWinnings` queries the mutable `payoutCalculator`, and `setPayoutCalculator` has no migration or historical-calculator mapping.

### M-102 / `Qet8mUOMbbhvJ91BSgZAI`
- Finding Title: Provider-scoped sequence numbers collide and strand Jackpot requests
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-sequence-collision`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The provider’s pending request identifier should include provider identity or otherwise prevent sequence reuse. Without that, provider migration can overwrite or confuse the Jackpot request that is needed to unlock settlement.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::pending` maps only `uint64 sequence` to request state, and `_storePendingRequest` overwrites existing pending fields for the same sequence.

### M-104 / `0T-_6WNMdFXwwPiE5OA2L`
- Finding Title: LP cap reductions can be front-run by pending deposits
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `lp-cap-frontrun`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Even though the cap invariant accounts for pending deposits, an ordinary LP can use that live-state rule to make a planned governance cap reduction revert before it is applied. The round instructions specifically preserve these permissionless state-obstruction cases.
- Code Evidence: `contracts/JackpotLPManager.sol::processDeposit` increases pending deposits under the current cap, and `setLPPoolCap` rejects a new cap below `lpPoolTotal + pendingDeposits`.

### M-105 / `IUVTSe8iKGXAh0X3qzqOy`
- Finding Title: Pending LP deposits can make governance cap updates unapplyable
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `lp-cap-frontrun`
- Checklist Gates Passed: `Stage0, Scope, Bug Exists, Root Cause, Exploitability, Impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The harmful state is created by a permissionless LP action before the governance update, not by governance choosing an unsafe value. This can block an otherwise legitimate defensive cap reduction and is valid under the delegated-authority guidance for this round.
- Code Evidence: `contracts/Jackpot.sol::setGovernancePoolCap` forwards the cap to `JackpotLPManager.setLPPoolCap`, which reverts if the new cap is below the live total plus pending deposits that any LP can increase via `processDeposit`.
