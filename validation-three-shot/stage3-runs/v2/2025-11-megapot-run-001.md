# 2025-11-megapot Three-Shot Stage 3 run-001

Status: Completed
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

### M-3 / `ghJ1xAP05g3yFJvWi6lh9`
- Finding Title: Entropy callback ignores the request id, allowing a stale request to settle a later locked drawing
- Decision: Invalid
- Confidence: High
- Bug Exists: No
- Severity Assessment: Low / QA
- Root Cause Family: `entropy-request-binding`
- Checklist Gates Passed: `Stage0, scope, code-path`
- Checklist Gates Failed: `bug-existence, exploitability`
- Detailed Reason: Jackpot ignores the sequence argument, but the claimed stale-request-to-later-drawing path is not reachable under normal current code. A Jackpot drawing cannot advance to a later `currentDrawingId` until a successful callback settles the existing locked drawing, and `ScaledEntropyProvider` stores the callback by sequence and deletes it when delivered. A stale callback for drawing `d` therefore cannot normally be held and then settle drawing `d+1` without an additional trusted/manual recovery flow not proven by this finding.
- Code Evidence: `contracts/Jackpot.sol::runJackpot` locks the current drawing before requesting entropy, and `scaledEntropyCallback` advances `currentDrawingId` only after settlement. `contracts/ScaledEntropyProvider.sol::entropyCallback` loads `pending[sequence]`, deletes it, and calls the stored callback, so only a pending Jackpot-created request can call Jackpot through the provider.

### M-4 / `ZwhrSB7704IKw_ypW0qEm`
- Finding Title: Bridge claim signatures never expire and can be executed long after the user signed them
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `signature-no-deadline`
- Checklist Gates Passed: `Stage0, scope, code-path, root-cause`
- Checklist Gates Failed: `H/M-impact, unauthorized-impact`
- Detailed Reason: The signed bridge messages have no nonce or deadline, so stale first-use execution is possible while the ticket remains bridge-custodied. However the signed payload fully binds the ticket IDs and recipient or opaque route, and successful execution transfers or burns the ticket, preventing repeat use. The harm depends on a relayer withholding a user-authorized action or the user signing a route whose intent later changes, which is a low-severity authorization-lifetime weakness rather than a Medium runtime exploit.
- Code Evidence: `contracts/JackpotBridgeManager.sol::createClaimWinningsEIP712Hash` and `createClaimTicketEIP712Hash` hash only ticket IDs plus `RelayTxData` or recipient. `claimWinnings` and `claimTickets` validate the recovered signer against `ticketOwner`, but there is no nonce, deadline, cancellation, or consumed digest mapping.

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

### M-7 / `b_aiHuotWEfaTycI8agH9`
- Finding Title: Changing payoutCalculator mid-drawing can settle active tickets with an unsnapshotted calculator
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `payout-calculator-drift`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: A payout calculator rotation is documented as future-facing, but Jackpot stores only one global calculator pointer. If governance installs a fresh calculator during an active drawing, that calculator has not received `setDrawingTierInfo(currentDrawingId)`, so settlement can calculate using default zero tier config and store zero payouts. This is a normal privileged workflow exposing user payout loss, not merely malicious use of authority.
- Code Evidence: `contracts/Jackpot.sol::setPayoutCalculator` only assigns the global `payoutCalculator`. `_setNewDrawingState` snapshots tier info only on the calculator current at drawing initialization, while `_calculateDrawingUserWinnings` later calls the live `payoutCalculator.calculateAndStoreDrawingUserWinnings`.

### M-8 / `-QwYrwqDqTJ71r1NZS63J`
- Finding Title: Changing Jackpot entropy address while a draw is pending bricks the authorized callback
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-authority-drift`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The pending callback source is not snapshotted per drawing. A valid future-intended entropy rotation after `runJackpot` but before fulfillment makes the request-time entropy contract fail `onlyEntropy`, leaving the drawing locked. This directly violates the docs' future-effects description and the benchmark's mid-drawing admin-change concern.
- Code Evidence: `contracts/Jackpot.sol::runJackpot` requests through the current `entropy`; `setEntropy` changes that global while locked; `scaledEntropyCallback` is guarded by `onlyEntropy`, which checks `msg.sender != address(entropy)` against the live pointer.

### M-9 / `4fPnoqYOKR9JwLwxDlbeJ`
- Finding Title: Bonusballs above bit capacity are erased, allowing uncounted tickets to claim winning bonusball payouts
- Decision: Invalid
- Confidence: High
- Bug Exists: No
- Severity Assessment: Low / QA
- Root Cause Family: `bonusball-bit-bound`
- Checklist Gates Passed: `Stage0, scope, code-path`
- Checklist Gates Failed: `bug-existence`
- Detailed Reason: The claimed "erased bonus bit causing false claims" does not match the compiled Solidity path. In `TicketComboTracker`, `_bonusball` and `_tracker.normalMax` are `uint8`, so values whose sum exceeds 255 revert under Solidity 0.8 checked arithmetic before the shift is evaluated; they are not silently shifted by 256 into zero. A separate settlement DoS exists for unsafe high bonusball ranges, but this finding's false-claim/corrupted-packing mechanism is not proven.
- Code Evidence: `contracts/lib/TicketComboTracker.sol::insert` and `countTierMatchesWithBonusball` use `1 << (_bonusball + _tracker.normalMax)` with both operands typed as `uint8`. `contracts/Jackpot.sol::_validateAndStoreTickets` calls `insert` before minting the NFT, so an overflowing high bonusball reverts before a corrupted ticket can be stored.

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

### M-11 / `PPrrZKotDc61i9N4tnTCl`
- Finding Title: Changing entropy while a drawing is pending bricks the original callback and leaves the jackpot locked
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-authority-drift`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The request-time entropy contract is not bound to the locked drawing, while `setEntropy` can change the live authorization address. A future-intended entropy change during a pending request makes the original callback unauthorized and blocks normal settlement. This is an in-scope liveness failure from global parameter drift.
- Code Evidence: `contracts/Jackpot.sol::runJackpot` locks and requests through `entropy`; `contracts/Jackpot.sol::setEntropy` has no locked-drawing guard; `scaledEntropyCallback` uses `onlyEntropy`, which checks the live global pointer.

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

### M-13 / `twUQAbk8uX7jwTke8KIfR`
- Finding Title: Unfulfillable SetRequests are accepted and later revert during entropy fulfillment
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `entropy-request-validation`
- Checklist Gates Passed: `Stage0, scope, code-path, root-cause`
- Checklist Gates Failed: `H/M-impact, current-Jackpot-exploitability`
- Detailed Reason: `ScaledEntropyProvider` does accept malformed public requests that later revert during fulfillment, but a public caller's callback is fixed to `msg.sender`, so an attacker cannot aim these arbitrary bad requests at Jackpot. The in-scope Jackpot's own request shape is fixed to two bounded requests under normal configuration. This is a real robustness and fee-loss issue for direct users of the provider, but not a demonstrated H/M protocol impact.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::requestAndCallbackScaledRandomness` calls `_validateRequests`, which checks only non-empty requests, ordered ranges, and nonzero samples. Fulfillment later calls `_getScaledRandomness`, which can revert in `FisherYatesRejection.draw`; however `_storePendingRequest` sets `pending[sequence].callback = msg.sender`.

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

### M-15 / `kieHSGUnSmvztUc_OBnAx`
- Finding Title: Missing bonusball packing bound corrupts tickets when normalBallMax + bonusballMax reaches 256
- Decision: Invalid
- Confidence: High
- Bug Exists: No
- Severity Assessment: Low / QA
- Root Cause Family: `bonusball-bit-bound`
- Checklist Gates Passed: `Stage0, scope, code-path`
- Checklist Gates Failed: `bug-existence`
- Detailed Reason: The described corrupted NFT state cannot be created as stated. With `normalMax = 128` and `bonusball = 128`, `TicketComboTracker.insert` evaluates a checked `uint8 + uint8` addition and reverts before returning a packed ticket, so `JackpotTicketNFT.mintTicket` is never reached. The missing boundary can cause purchase or settlement reverts, but not the silent bit erasure and stored corrupted ticket claimed here.
- Code Evidence: `contracts/lib/TicketComboTracker.sol::insert` packs with `1 << (_bonusball + _tracker.normalMax)` using `uint8` operands. `contracts/Jackpot.sol::_validateAndStoreTickets` calls `TicketComboTracker.insert` before computing the ticket ID and before `jackpotNFT.mintTicket`.

### M-16 / `v2sgzi0QYHrqs1cThqt8a`
- Finding Title: Bridge claim signatures have no nonce or deadline and remain executable indefinitely
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `signature-no-deadline`
- Checklist Gates Passed: `Stage0, scope, code-path, root-cause`
- Checklist Gates Failed: `H/M-impact, unauthorized-impact`
- Detailed Reason: The missing nonce/deadline is real, but the signature authorizes the exact ticket IDs and recipient or bridge calldata, and the first successful execution consumes the ticket by transfer or burn. The remaining harm is stale execution of an already authorized intent by a signature holder, not a current Medium unauthorized transfer path.
- Code Evidence: `contracts/JackpotBridgeManager.sol::createClaimTicketEIP712Hash` and `createClaimWinningsEIP712Hash` omit nonce and deadline fields. `claimTickets` transfers NFTs and `claimWinnings` burns tickets through Jackpot, preventing exact replay after success.

### M-17 / `EjJJ7I_tm_zeL_HoMTLjP`
- Finding Title: ECDSA-only bridge authorization permanently locks tickets owned by ERC-1271 smart wallets
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `erc1271-unsupported`
- Checklist Gates Passed: `Stage0, scope, supported-behavior, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The bridge records contract recipients as owners but validates only ECDSA-recovered EOAs for both ticket withdrawal and winnings claims. A smart wallet cannot satisfy `ticketOwner[ticketId] == recoveredEOA`, and the manager has no alternate path to release or claim those custodied tickets. This can materially lock bridge tickets and attached winnings for accepted recipients.
- Code Evidence: `contracts/JackpotBridgeManager.sol::buyTickets` stores `_recipient` in `ticketOwner` with no contract-wallet restriction. `claimTickets` and `claimWinnings` use `ECDSA.recover` and `_validateTicketOwnership`; there is no ERC-1271 validation path.

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

### M-21 / `PMFE9hYicCVTJCz09VHk6`
- Finding Title: Perpetual EIP-712 bridge signatures can be executed after user intent expires
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `signature-no-deadline`
- Checklist Gates Passed: `Stage0, scope, code-path, root-cause`
- Checklist Gates Failed: `H/M-impact, unauthorized-impact`
- Detailed Reason: The hashes are perpetual until ticket state changes, but they bind the exact signed ticket set and recipient or route. A relayer withholding or later using a signature is stale execution of the user's own signed authorization, not an independently exploitable Medium bug in the current code. This should be treated as low-severity signature hygiene.
- Code Evidence: `contracts/JackpotBridgeManager.sol::createClaimWinningsEIP712Hash` and `createClaimTicketEIP712Hash` have no deadline, nonce, salt, or consumed-digest state. The only validation in `claimWinnings` and `claimTickets` is recovered signer ownership.

### M-22 / `OYZfVCU_Jw3zjvZAlSLNS`
- Finding Title: Nonce-less ticket withdrawal signatures stay valid indefinitely and can steal later winning tickets
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `signature-no-deadline`
- Checklist Gates Passed: `Stage0, scope, code-path, root-cause`
- Checklist Gates Failed: `H/M-impact, unauthorized-impact`
- Detailed Reason: `ClaimTicketData` lacks expiry and cancellation, so a signed withdrawal remains usable until the ticket leaves bridge custody. But the recipient is part of the signed payload, and a successful withdrawal deletes bridge ownership and transfers the NFT according to that signed recipient. The report's "steal later winning tickets" impact depends on a stale or leaked user authorization, so it does not reach H/M.
- Code Evidence: `contracts/JackpotBridgeManager.sol::createClaimTicketEIP712Hash` hashes only `ticketIds` and `_recipient`. `claimTickets` recovers the signer, validates `ticketOwner`, and calls `_updateTicketOwnership`, which deletes bridge ownership and transfers the NFT.

### M-23 / `XboZ6rstW75a7OfO7LUPg`
- Finding Title: Pending LP deposits can be rounded to zero after accumulator inflation
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `lp-zero-share-rounding`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `high-confidence-impact`
- Detailed Reason: Pending deposits are later converted to shares with floor division and then unconditionally deleted. If the settled drawing accumulator has been inflated enough relative to a pending deposit, the depositor receives zero shares while the deposited USDC remains included in LP value. This exceeds ordinary dust when the accumulator can become very large in thin-liquidity states, though the exploit is state-dependent enough to assess as Medium rather than High here.
- Code Evidence: `contracts/JackpotLPManager.sol::processDeposit` stores pending deposits as raw USDC in `lp.lastDeposit`. `_consolidateDeposits` does `_lp.consolidatedShares += amount * 1e18 / drawingAccumulator[drawingId]` and deletes `lastDeposit` even when the quotient is zero. `processDrawingSettlement` can raise `drawingAccumulator[_drawingId]` with `postDrawLpValue / currentLP.lpPoolTotal`.

### M-24 / `_uzGLqresOu7Dk6U-KMjO`
- Finding Title: No-referral winner shares are credited to the claim-time drawing instead of the settled drawing
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `referral-share-wrong-epoch`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The no-referral retained share is accounted against `currentDrawingId` at claim time, while `claimWinnings` can process tickets from any completed drawing. This lets a delayed winner shift the retained referral share away from the LP cohort that funded the payout and into a later active drawing. The amount can be material because referral win share is governance-configurable up to 100%.
- Code Evidence: `contracts/Jackpot.sol::claimWinnings` requires only `ticketInfo.drawingId < currentDrawingId` and passes no drawing ID into `_payReferrersWinnings`. The no-referral branch in `_payReferrersWinnings` updates `drawingState[currentDrawingId].lpEarnings`.

### M-25 / `-HhMgy6frh49i966ZDULK`
- Finding Title: Full uint256 range requests are accepted but overflow during randomness scaling
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `entropy-request-validation`
- Checklist Gates Passed: `Stage0, scope, code-path, root-cause`
- Checklist Gates Failed: `H/M-impact, current-Jackpot-exploitability`
- Detailed Reason: The provider does accept a full inclusive uint256 range that overflows when computing `maxRange - minRange + 1` during fulfillment. That is a real validation bug for direct callers, but the callback address is the requester and the Jackpot integration never constructs such a range. The report does not prove a current attacker-controlled path to block Jackpot, so the issue is Low/QA.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::_validateRequests` checks only `minRange <= maxRange` and `samples != 0`. `_drawWithReplacement` and `FisherYatesRejection.draw` both compute `maxRange - minRange + 1`, which overflows for `[0, type(uint256).max]`.

### H-26 / `Zy2Cwvh0tHGOtbihykmbO`
- Finding Title: Pending LP deposits can be zeroed by accumulator inflation before settlement
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `lp-zero-share-rounding`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `high-confidence-impact`
- Detailed Reason: The zero-share conversion path is real: a prior-round pending deposit is converted with floor division against the settled drawing accumulator and is deleted even if zero shares are minted. In thin-pool/high-earnings states, the zeroing threshold can exceed ordinary dust, so a depositor can lose their entire pending deposit and that value remains in the LP pool. I classify it as Medium because the exploit depends on unusual accumulator inflation and deposit sizing rather than a simple universal drain.
- Code Evidence: `contracts/JackpotLPManager.sol::_consolidateDeposits` converts `lastDeposit.amount` using `(amount * PRECISE_UNIT) / drawingAccumulator[lastDeposit.drawingId]` and then deletes the deposit. `processDrawingSettlement` sets `drawingAccumulator[_drawingId] = drawingAccumulator[_drawingId - 1] * postDrawLpValue / currentLP.lpPoolTotal`.

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

### M-28 / `4J7yK8OUsFrvGAhCtYqjh`
- Finding Title: Full uint256 range requests overflow during scaling after being accepted
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `entropy-request-validation`
- Checklist Gates Passed: `Stage0, scope, code-path, root-cause`
- Checklist Gates Failed: `H/M-impact, current-Jackpot-exploitability`
- Detailed Reason: The overflow exists for direct `ScaledEntropyProvider` users, but it is self-targeted because pending callbacks are bound to `msg.sender`. The current Jackpot integration uses fixed bounded ranges and cannot be forced by an attacker to request `[0, type(uint256).max]`. This is Low/QA hardening, not a Medium jackpot liveness bug.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::_validateRequests` accepts ordered ranges without overflow-safe range sizing. `_drawWithReplacement` and `contracts/lib/FisherYatesWithRejection.sol::draw` compute the inclusive range size during callback-time scaling.

### M-29 / `qYfnKos-ZnhIgN86d2poe`
- Finding Title: Bridge claim signatures have no nonce or deadline, allowing stale execution through obsolete bridge routes
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `signature-no-deadline`
- Checklist Gates Passed: `Stage0, scope, code-path, root-cause`
- Checklist Gates Failed: `H/M-impact, unauthorized-impact`
- Detailed Reason: The missing expiry and nonce are real, but the route itself is included in the signed digest and success consumes the ticket. Executing an old route before the user changes state is stale use of a previously authorized payload and depends on relayer withholding or user intent changing off-chain. It is not a standalone H/M exploit.
- Code Evidence: `contracts/JackpotBridgeManager.sol::createClaimWinningsEIP712Hash` includes `keccak256(_bridgeDetails.data)` and the route addresses, but no nonce, deadline, or cancellation state. `claimWinnings` only validates recovered signer ownership.

### M-30 / `l5OqMxsLjuLw9XhbtQprM`
- Finding Title: Bridge ticket and winnings signatures never expire and can be executed long after user intent changes
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `signature-no-deadline`
- Checklist Gates Passed: `Stage0, scope, code-path, root-cause`
- Checklist Gates Failed: `H/M-impact, unauthorized-impact`
- Detailed Reason: Time-invariant claim hashes are a real weakness, but the authorized recipient or bridge details are exactly what the user signed, and the ticket state changes after successful use. The report's loss scenario relies on a signature holder delaying or exposing stale user intent, which is insufficient for Medium under the delegated-authority and practical-exploitability gates.
- Code Evidence: `contracts/JackpotBridgeManager.sol::createClaimTicketEIP712Hash` and `createClaimWinningsEIP712Hash` do not include time, nonce, or amount bounds. `claimTickets` and `claimWinnings` validate only recovered signer ownership before executing the signed intent.

### M-32 / `Nlj-UYTyD4z1UkRYuxpyz`
- Finding Title: Invalid full-range randomness requests are accepted and later revert during entropy fulfillment
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `entropy-request-validation`
- Checklist Gates Passed: `Stage0, scope, code-path, root-cause`
- Checklist Gates Failed: `H/M-impact, current-Jackpot-exploitability`
- Detailed Reason: A full uint256 range request can be accepted and later revert, but arbitrary public requests callback to their own requester. The report does not show the current Jackpot can be driven into that request shape by an unprivileged actor. The issue is real provider hardening, but Low/QA for this benchmark.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::_validateRequests` omits an overflow-safe inclusive range-size check; `_getScaledRandomness` later reaches `_drawWithReplacement` or `FisherYatesRejection.draw`, both using `maxRange - minRange + 1`.

### M-33 / `oGmvAj73gSj07W0Jh5QNn`
- Finding Title: Impossible no-replacement draws are accepted and can fail only after entropy is consumed
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `entropy-request-validation`
- Checklist Gates Passed: `Stage0, scope, code-path, root-cause`
- Checklist Gates Failed: `H/M-impact, current-Jackpot-exploitability`
- Detailed Reason: `samples > rangeSize` for no-replacement requests is not rejected until fulfillment, so direct provider users can create unfulfillable requests. But a malicious caller cannot set Jackpot as the callback, and Jackpot's own normal request is fixed at 5 samples from the configured normal range. Without a current in-scope path to strand Jackpot, this remains Low/QA.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::_validateRequests` lacks a `samples <= rangeSize` check for `withReplacement == false`. `contracts/lib/FisherYatesWithRejection.sol::draw` enforces that invariant later with `require(count <= maxRange - minRange + 1)`.

### M-34 / `DTCE_kPYivgorgy9AIkkR`
- Finding Title: Missing feasibility validation lets callers create entropy requests that can never be fulfilled
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `entropy-request-validation`
- Checklist Gates Passed: `Stage0, scope, code-path, root-cause`
- Checklist Gates Failed: `H/M-impact, current-Jackpot-exploitability`
- Detailed Reason: The combined missing bounds are genuine for direct calls to `ScaledEntropyProvider`, but the harmful requests are requester-scoped and do not compromise other callbacks. Current Jackpot requests are fixed-size and bounded under valid drawing configuration. This is not a demonstrated Medium protocol DoS.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::_validateRequests` checks only array non-emptiness, ordered ranges, and nonzero samples. `_getScaledRandomness` later executes potentially reverting or gas-heavy scaling for the stored requester.

### M-35 / `et7UofFB1msp9Vt7VVQ2v`
- Finding Title: Stale entropy callbacks can settle the wrong Jackpot drawing because request IDs are not bound to drawing IDs
- Decision: Invalid
- Confidence: High
- Bug Exists: No
- Severity Assessment: Low / QA
- Root Cause Family: `entropy-request-binding`
- Checklist Gates Passed: `Stage0, scope, code-path`
- Checklist Gates Failed: `bug-existence, exploitability`
- Detailed Reason: Although Jackpot ignores the sequence parameter, the claimed stale callback settling a later drawing is not reachable through normal lifecycle. `currentDrawingId` advances only in the successful callback, so an unresolved old request prevents the system from reaching a later locked drawing. The provider also stores the callback by sequence. The finding does not prove an attacker-controlled path from current code to wrong-drawing settlement.
- Code Evidence: `contracts/Jackpot.sol::scaledEntropyCallback` is the only path that calls `_setNewDrawingState` after a locked draw. `contracts/ScaledEntropyProvider.sol::entropyCallback` delivers to the callback stored in `pending[sequence]`, and arbitrary callers cannot register Jackpot as callback because `_storePendingRequest` uses `msg.sender`.

### M-36 / `N8k9IZrZQviCeyyaNHOQ4`
- Finding Title: Changing entropy provider while a request is pending permanently rejects the valid callback and locks the drawing
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-authority-drift`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: This is the same live entropy-address drift: the old provider is the only contract with the in-flight request, but authorization checks the new global address after `setEntropy`. A future-intended rotation can therefore lock the active drawing and block claims/LP progression until privileged recovery.
- Code Evidence: `contracts/Jackpot.sol::setEntropy` updates the global pointer without checking pending requests or lock state. `scaledEntropyCallback` uses the `onlyEntropy` modifier against that live pointer.

### M-37 / `dFlfXpkDW49CviZDQWS2A`
- Finding Title: Residual bridge approvals let approved spenders drain future USDC entering JackpotBridgeManager
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bridge-stale-allowance`
- Checklist Gates Passed: `Stage0, scope, supported-behavior, root-cause, safeguards, bounded-impact`
- Checklist Gates Failed: `high-impact`
- Detailed Reason: The allowance is left reusable if the arbitrary bridge call moves the claimed amount without consuming the approved allowance. The strongest normal impact is theft of idle bridge-manager surplus, not arbitrary future transient funds, because normal claim and purchase flows do not expose a separate transaction window. Still, the missing cleanup creates a real bounded theft path when surplus exists.
- Code Evidence: `contracts/JackpotBridgeManager.sol::_bridgeFunds` approves `approveTo`, performs arbitrary `to.call(data)`, and checks only the USDC balance delta. It never calls `approve(approveTo, 0)` or checks final allowance.

### H-38 / `o3d2w1GwCZw3kuQKHepDU`
- Finding Title: Opaque bridge calldata can pass balance checks while sending winnings to an attacker
- Decision: Invalid
- Confidence: High
- Bug Exists: No
- Severity Assessment: Low / QA
- Root Cause Family: `signed-opaque-route`
- Checklist Gates Passed: `Stage0, scope, code-path`
- Checklist Gates Failed: `unauthorized-impact, practical-exploitability`
- Detailed Reason: `_bridgeFunds` does execute opaque signed calldata, but for the claimant's own winnings the route is included in the EIP-712 digest. Sending those winnings to an attacker requires the ticket owner to sign calldata that does exactly that, or to rely on a malicious/off-chain route builder whose bytes they authorize. Under the validation rules, victim confusion or malicious use of a delegated signed route is not an H/M on-chain bug.
- Code Evidence: `contracts/JackpotBridgeManager.sol::createClaimWinningsEIP712Hash` hashes `approveTo`, `to`, and `keccak256(data)`. `claimWinnings` validates that signature before `_bridgeFunds` moves only the current claimed amount.

### H-39 / `yx3YULUi7UPyOA6c8ez2L`
- Finding Title: Arbitrary bridge call can transfer other users' custodied ticket NFTs from JackpotBridgeManager
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: High
- Root Cause Family: `bridge-arbitrary-call-custody`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The arbitrary call is executed from the custody contract, not from an isolated per-claim escrow. A claimant with positive winnings can target the ticket NFT and transfer a victim bridge-custodied ticket because the manager is the ERC-721 owner, while using the approved claimed USDC to satisfy the balance-delta check. This is a permissionless theft path against unrelated ticket owners.
- Code Evidence: `contracts/JackpotBridgeManager.sol::_bridgeFunds` performs arbitrary `to.call(data)` from the manager. `buyTickets` mints NFTs to `address(this)` and only separately records logical `ticketOwner`, which `_bridgeFunds` does not preserve or validate for unrelated tickets.

### M-40 / `JjYFCeoUYvcJcIX6_pK_0`
- Finding Title: Invalid no-replacement requests are accepted then permanently fail during entropy fulfillment
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `entropy-request-validation`
- Checklist Gates Passed: `Stage0, scope, code-path, root-cause`
- Checklist Gates Failed: `H/M-impact, current-Jackpot-exploitability`
- Detailed Reason: The no-replacement feasibility check is missing at request time, but a public bad request affects the caller's own callback and paid entropy sequence. The current Jackpot integration does not expose user-controlled set requests. This is real hardening but not a Medium protocol liveness issue.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::_validateRequests` does not compare `samples` against range size. `FisherYatesWithRejection.sol::draw` reverts later when the invariant is violated.

### M-41 / `JmNO-knM1BokEuNBDWxTv`
- Finding Title: ECDSA-only bridge signatures permanently lock tickets and winnings owned by smart wallets
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `erc1271-unsupported`
- Checklist Gates Passed: `Stage0, scope, supported-behavior, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Smart-wallet bridge recipients are accepted into `ticketOwner`, but the only authorization path is ECDSA recovery to the stored address. ERC-1271 contract signatures cannot recover to the contract wallet itself, so those users have no working path to withdraw or claim bridge-custodied tickets. This can materially lock assets for accepted owners.
- Code Evidence: `contracts/JackpotBridgeManager.sol::buyTickets` accepts any nonzero `_recipient`; `claimTickets` and `claimWinnings` recover an EOA with OpenZeppelin `ECDSA` and require it equal every `ticketOwner[ticketId]`.

### M-42 / `bEM7bCscGCIhiKqM40GUJ`
- Finding Title: Stale bridge claim signatures can be replayed indefinitely until the ticket is consumed
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `signature-no-deadline`
- Checklist Gates Passed: `Stage0, scope, code-path, root-cause`
- Checklist Gates Failed: `H/M-impact, unauthorized-impact`
- Detailed Reason: A valid claim signature remains first-use executable while the ticket is still mapped to the signer, but the signed payload fixes the recipient or bridge calldata and state consumption prevents repeat execution. The report does not show an unauthorized transfer independent of stale user authorization. This is Low/QA.
- Code Evidence: `contracts/JackpotBridgeManager.sol` has no nonce/deadline fields in either EIP-712 claim hash and no used-digest mapping; `claimTickets` and `claimWinnings` rely only on `ECDSA.recover` plus `ticketOwner`.

### M-43 / `anKPmMCB1WwZFh-pgeeD3`
- Finding Title: No-referral win share is credited to the current drawing, letting later LPs capture prior drawing value
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `referral-share-wrong-epoch`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: For no-referral claims, the withheld referral share is allocated to whatever drawing is active at claim time. A winner can delay claiming until a later LP cohort is favorable, moving value that was funded by the settled drawing. The code has no safeguard tying retained referral winnings to `ticketInfo.drawingId`.
- Code Evidence: `contracts/Jackpot.sol::claimWinnings` handles historical tickets, while `_payReferrersWinnings` writes no-referral shares to `drawingState[currentDrawingId].lpEarnings` rather than the ticket drawing.

### M-44 / `CJfADIczTrhQXGEfG6myl`
- Finding Title: Ticket purchases remain open after drawingTime until runJackpot is mined
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `late-ticket-entry`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: `drawingTime` is enforced only by `runJackpot`, not by ticket purchase. Once the scheduled time has passed but before a keeper transaction locks the drawing, a user can still buy into that drawing and then have it settled. Given the protocol docs frame drawing duration and fairness as core concerns, the absence of a purchase cutoff creates a practical last-mover fairness issue and can dilute earlier users.
- Code Evidence: `contracts/Jackpot.sol::_validateBuyTicketInputs` checks lock, prize pool, global purchase flag, count, recipient, and referral array shape, but not `block.timestamp`. `runJackpot` checks `drawingTime` and then calls `_lockJackpot`, so purchases remain possible until that transaction executes.

### M-46 / `3nNN8Mv0tqj6acsGySPOG`
- Finding Title: Unsnapshotted drawing payouts can be calculated as zero in GuaranteedMinimumPayoutCalculator.calculateAndStoreDrawingUserWinnings
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `payout-calculator-drift`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The calculator has no initialized flag for per-drawing tier info, and Jackpot can call a newly rotated calculator for an active drawing that was initialized on the old calculator. The default tier info causes all tiers to be skipped and returns zero payout despite winners and prize pool. This is a material active-drawing payout loss from a normal future-intended rotation.
- Code Evidence: `contracts/GuaranteedMinimumPayoutCalculator.sol::calculateAndStoreDrawingUserWinnings` reads `drawingTierInfo[_drawingId]` and skips tiers when min flags and weights are default zero. `contracts/Jackpot.sol::setPayoutCalculator` does not call `setDrawingTierInfo(currentDrawingId)` on the new calculator.

### M-47 / `DY8_8XaFt5YlWy8JdsHMf`
- Finding Title: No-referral winning share is credited to the active drawing, letting winners redirect prior LP value to future LPs
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `referral-share-wrong-epoch`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The finding correctly identifies that a delayed no-referral claim can redirect retained win share from the settled drawing's LP accounting into the active drawing. Since winners can choose claim timing and LP positions can change between drawings, the value movement is attacker-influenced and can be material for large payouts.
- Code Evidence: In `contracts/Jackpot.sol`, `claimWinnings` computes the ticket's `drawingId` but `_payReferrersWinnings` credits `drawingState[currentDrawingId].lpEarnings` in the no-referral branch.

### H-48 / `x-waZ76GXiDz3-pI_Wxg2`
- Finding Title: Bonusball bit overflow corrupts tier calculation and can brick valid Jackpot.claimWinnings payouts
- Decision: Invalid
- Confidence: High
- Bug Exists: No
- Severity Assessment: Low / QA
- Root Cause Family: `bonusball-bit-bound`
- Checklist Gates Passed: `Stage0, scope, code-path`
- Checklist Gates Failed: `bug-existence`
- Detailed Reason: The claim-time underflow from erased high bonusball bits is not reachable as described. In the real packing functions, high `normalMax + bonusball` values overflow checked `uint8` addition and revert before producing a packed ticket or winning ticket with a missing bonus bit. Unsafe ranges can brick settlement when entropy returns an unencodable bonusball, but this finding's stored-corruption and claim-time false match path is not the actual code behavior.
- Code Evidence: `contracts/lib/TicketComboTracker.sol::insert` and `countTierMatchesWithBonusball` both pack with `1 << (_bonusball + _tracker.normalMax)` where both operands are `uint8`. `contracts/Jackpot.sol::_calculateTicketTierId` only operates on packed values that would not be produced for overflowing bonusballs.

### M-49 / `uvZ6f7Zqp3uFwf09w21c3`
- Finding Title: runJackpot requires an unrewarded caller to pay entropy fees for protocol liveness
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `keeper-incentive`
- Checklist Gates Passed: `Stage0, scope, code-path`
- Checklist Gates Failed: `H/M-impact, attacker-controlled-path`
- Detailed Reason: `runJackpot` is unpaid and requires the caller to provide the entropy fee, so the liveness incentive design is weak. But this is not an attacker-controlled exploit path by itself; it depends on all actors refusing to perform expected maintenance, and owner/keeper operations can still progress the protocol. The issue is best classified as Low/QA incentive hardening.
- Code Evidence: `contracts/Jackpot.sol::runJackpot` requires `msg.value >= entropy.getFee(entropyGasLimit)`, refunds only excess, and emits no reward or reimbursement. `claimWinnings` requires `drawingId < currentDrawingId`, so delayed execution delays claims.

### M-50 / `4BnVTywm1gum72Soijtty`
- Finding Title: Unpaid jackpot execution leaves expired drawings open for late ticket buyers
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `late-ticket-entry`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The important exploitable part is not just keeper incentives, but that expired drawings remain open for ticket purchases until someone pays to run and lock the jackpot. A strategic buyer can enter after `drawingTime` and before calling or front-running `runJackpot`, which weakens the advertised drawing cutoff and fairness. This is a live permissionless path.
- Code Evidence: `contracts/Jackpot.sol::buyTickets` does not check `drawingTime`; `_validateBuyTicketInputs` only checks `jackpotLock` and global purchase state. `runJackpot` enforces `drawingTime` only when locking the drawing.

### M-52 / `ZSK-vXUjmoLQ2ZqQGSEpm`
- Finding Title: Unbounded request ranges and batch sizes can make entropy callbacks run out of gas
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `entropy-request-gas-bounds`
- Checklist Gates Passed: `Stage0, scope, code-path, root-cause`
- Checklist Gates Failed: `H/M-impact, current-Jackpot-exploitability`
- Detailed Reason: The provider lacks request-count and range-size bounds, and direct callers can create requests that are too expensive to fulfill. However, the callback is bound to the requesting contract, and Jackpot constructs a fixed two-request batch with bounded ranges under normal configuration. The report's H/M impact depends on a downstream integration accepting attacker-controlled request parameters, not on the current Jackpot flow.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::_validateRequests` has no length or range-size cap. `_getScaledRandomness` loops over every request and `FisherYatesRejection.draw` allocates a full range array for no-replacement draws.

### M-53 / `TkptkGk_kcgTRVqNNHeX8`
- Finding Title: Reverting requester callback rolls back pending cleanup and leaves consumed entropy requests stale
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `entropy-callback-failure`
- Checklist Gates Passed: `Stage0, scope, code-path, root-cause`
- Checklist Gates Failed: `H/M-impact, current-Jackpot-exploitability`
- Detailed Reason: A failing callback reverts the whole `entropyCallback`, including the prior delete, so stale pending storage can remain. For arbitrary requesters this is self-inflicted, and the report does not show a current attacker-controlled way to make Jackpot's callback fail through this root alone. It is a real provider robustness issue but Low/QA here.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::entropyCallback` copies `pending[sequence]`, deletes it, calls `req.callback`, and reverts `CallbackFailed` if the callback fails; Solidity reverts the delete as part of the same transaction.

### M-54 / `cn7PMWjyPYAdapY1lFl5n`
- Finding Title: Bridge claim signatures have no nonce or deadline, allowing leaked or old authorizations to be executed later
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `signature-no-deadline`
- Checklist Gates Passed: `Stage0, scope, code-path, root-cause`
- Checklist Gates Failed: `H/M-impact, unauthorized-impact`
- Detailed Reason: Old signatures remain executable until ticket state changes, but the route or recipient is part of the signed digest and successful use consumes custody. Leaked or delayed signatures are a low-severity authorization-lifetime and UX risk; the report does not establish independent unauthorized H/M impact.
- Code Evidence: `contracts/JackpotBridgeManager.sol` EIP-712 claim hashes omit nonces, deadlines, relayer binding, and consumed digest state. `claimWinnings` and `claimTickets` use `ECDSA.recover` and `ticketOwner` equality as the only authorization check.

### L-55 / `JGgR2xyH0uL0CDYaPTq9A`
- Finding Title: claimWinnings leaves burned tickets recorded as owned in bridge accounting
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `bridge-stale-ticket-accounting`
- Checklist Gates Passed: `Stage0, scope, bug-existence, low-impact`
- Checklist Gates Failed: `H/M-impact`
- Detailed Reason: After a bridge winnings claim, Jackpot burns the NFT but the bridge manager does not clear its separate `ticketOwner` mapping. This can make `getUserTickets` report burned tickets and later `claimTickets` pass bridge-level ownership before failing at ERC-721 transfer. Core funds are not stolen and the effect is stale accounting/query availability, so Low/QA is appropriate.
- Code Evidence: `contracts/JackpotBridgeManager.sol::claimWinnings` validates ownership and calls `jackpot.claimWinnings`, but unlike `_updateTicketOwnership`, it never deletes `ticketOwner[ticketId]`. `getUserTickets` filters solely by `ticketOwner[ticketId] == _user`.

### H-56 / `ScHOGndBOIJkw9PUyhBrl`
- Finding Title: Arbitrary bridge call lets a winning claimant transfer other users' custodied ticket NFTs
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: High
- Root Cause Family: `bridge-arbitrary-call-custody`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The bridge manager executes arbitrary calldata while holding all bridge NFTs. A winning claimant can use their own claim to make the manager call the NFT contract and transfer a victim ticket, while consuming the claimant's USDC allowance to satisfy the balance check. This is direct theft of unrelated custodied assets and reaches High severity.
- Code Evidence: `contracts/JackpotBridgeManager.sol::_bridgeFunds` exposes arbitrary `to.call(data)` from the manager and checks only USDC balance decrease. `buyTickets` mints bridge tickets to `address(this)` and records logical ownership separately, which `_bridgeFunds` does not protect.

### M-57 / `xY6OOMdx8rewLd4XafLh_`
- Finding Title: Opaque bridge calldata can steal all claimed winnings because claimWinnings does not bind amount or recipient
- Decision: Invalid
- Confidence: High
- Bug Exists: No
- Severity Assessment: Low / QA
- Root Cause Family: `signed-opaque-route`
- Checklist Gates Passed: `Stage0, scope, code-path`
- Checklist Gates Failed: `unauthorized-impact, practical-exploitability`
- Detailed Reason: The claimant signs the opaque route, including target and calldata hash. If that route sends the claimant's own winnings to an attacker, the on-chain contract is executing the signed authorization; the harm requires malicious route construction or victim misunderstanding of signed bytes. That is not a current H/M code vulnerability under the practical attack and user-error gates.
- Code Evidence: `contracts/JackpotBridgeManager.sol::createClaimWinningsEIP712Hash` binds `RelayTxData` by hashing `approveTo`, `to`, and `data`. `claimWinnings` validates the signer against `ticketOwner` before moving the claimed amount through that signed route.

### H-58 / `I7FqeqgXJk0H66gi0fIkj`
- Finding Title: Signed bridge route can approve one address while arbitrary call drains the claimed USDC through another address
- Decision: Invalid
- Confidence: High
- Bug Exists: No
- Severity Assessment: Low / QA
- Root Cause Family: `signed-opaque-route`
- Checklist Gates Passed: `Stage0, scope, code-path`
- Checklist Gates Failed: `unauthorized-impact, practical-exploitability`
- Detailed Reason: Approving one address while the call transfers funds through another is possible because the route is arbitrary, but that route is exactly what the ticket owner signs for their own claimed winnings. The report's loss requires a malicious route builder or user signing attacker-directed calldata, not an unauthorized on-chain bypass. This is not H/M as stated.
- Code Evidence: `contracts/JackpotBridgeManager.sol::_bridgeFunds` allows separate `approveTo` and `to`, but `createClaimWinningsEIP712Hash` includes both addresses and `keccak256(data)` in the signed digest.

### M-59 / `AmvuAyw8y2V0rkSpDDxav`
- Finding Title: Arbitrary bridge call can leave reusable USDC allowance and steal pre-existing bridge surplus
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bridge-stale-allowance`
- Checklist Gates Passed: `Stage0, scope, supported-behavior, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `high-impact`
- Detailed Reason: This narrower surplus-drain version is valid. A claimant can move the current claimed amount with a direct token transfer while leaving the separate `approveTo` allowance unused, then use the stale allowance to pull idle USDC already held by the bridge manager. The impact is bounded by existing surplus and the claimed amount, so Medium is appropriate.
- Code Evidence: `contracts/JackpotBridgeManager.sol::_bridgeFunds` sets `usdc.approve(approveTo, claimedAmount)` before the arbitrary call and never clears it. Its only postcondition is the exact USDC balance delta during the call.

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

### M-61 / `ca2NbWJQODDuT28JrEfvS`
- Finding Title: Large no-replacement ranges can force fulfillment-time memory exhaustion
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `entropy-request-gas-bounds`
- Checklist Gates Passed: `Stage0, scope, code-path, root-cause`
- Checklist Gates Failed: `H/M-impact, current-Jackpot-exploitability`
- Detailed Reason: A large no-replacement range is accepted and can exhaust callback gas, but the request harms the requester whose address is stored as callback. The current Jackpot constructs bounded set requests and does not expose arbitrary range selection. This is Low/QA provider hardening.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::_validateRequests` has no no-replacement range-size cap. `contracts/lib/FisherYatesWithRejection.sol::draw` allocates and initializes `new uint256[](rangeSize)`.

### M-62 / `62WtuEiLCFmhhhAkFFXso`
- Finding Title: Unbounded request arrays can make entropy fulfillment run out of gas and leave drawings stuck
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `entropy-request-gas-bounds`
- Checklist Gates Passed: `Stage0, scope, code-path, root-cause`
- Checklist Gates Failed: `H/M-impact, current-Jackpot-exploitability`
- Detailed Reason: The provider accepts unbounded request arrays and can later run out of gas encoding or delivering outputs. But public requests are callback-scoped to the requester, and the Jackpot integration always sends a fixed two-element array. The claimed drawing-stuck impact is not reachable from current Jackpot behavior, so Low/QA.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::_storePendingRequest` pushes every supplied request, `_getScaledRandomness` allocates an output array with the same length, and `_validateRequests` imposes no batch-size or total-output bound.

### M-63 / `VYM7kgJV8rdR7r9C3Ouaf`
- Finding Title: Invalid no-replacement entropy requests are accepted then permanently revert during fulfillment
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `entropy-request-validation`
- Checklist Gates Passed: `Stage0, scope, code-path, root-cause`
- Checklist Gates Failed: `H/M-impact, current-Jackpot-exploitability`
- Detailed Reason: The exact invalid request is accepted and later reverts, but only for the requester that submitted it. The finding does not establish that an attacker can make Jackpot submit an impossible no-replacement request through normal behavior. This is Low/QA for this benchmark.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::_validateRequests` omits `samples <= rangeSize`; `FisherYatesWithRejection.draw` enforces it later during fulfillment.

### M-64 / `qTFiUtYrdGZ5IC_nE_NzD`
- Finding Title: Full uint256 ranges are accepted but overflow during scaled randomness fulfillment
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `entropy-request-validation`
- Checklist Gates Passed: `Stage0, scope, code-path, root-cause`
- Checklist Gates Failed: `H/M-impact, current-Jackpot-exploitability`
- Detailed Reason: The full-range overflow is a real request-time validation gap, but it is self-contained to public provider users because callback ownership is `msg.sender`. Jackpot does not request full uint256 ranges. The issue should remain Low/QA absent a current integration path to strand the jackpot.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::_drawWithReplacement` computes `uint256 range = _maxRange - _minRange + 1`, and `FisherYatesWithRejection.draw` performs the same range-size expression after `_validateRequests` allowed the request.

### M-65 / `mrBG0kQc_1aaDkHc5rWc-`
- Finding Title: Missing request bounds allow entropy requests that pass validation but revert or exhaust gas during fulfillment
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `entropy-request-validation`
- Checklist Gates Passed: `Stage0, scope, code-path, root-cause`
- Checklist Gates Failed: `H/M-impact, current-Jackpot-exploitability`
- Detailed Reason: The combined request validation defects exist, but their demonstrated impact is to the direct requester or a hypothetical integration that passes attacker-controlled set requests. The current Jackpot request construction is fixed and not user-controlled. This is a real Low/QA robustness issue, not an H/M finding here.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::_validateRequests` lacks feasibility, overflow-safe range, batch-size, and gas-bound checks. `_getScaledRandomness`, `_drawWithReplacement`, and `FisherYatesRejection.draw` can later revert or exhaust gas.

### L-67 / `NmeIslHIena89r8c9Tp8O`
- Finding Title: Unbounded per-user ticket enumeration can make JackpotTicketNFT.getUserTickets unusable for large holders
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Low / QA
- Root Cause Family: `view-enumeration-unbounded`
- Checklist Gates Passed: `Stage0, scope, bug-existence, low-impact`
- Checklist Gates Failed: `H/M-impact`
- Detailed Reason: `getUserTickets` is unpaginated and can become expensive for large holders, but it is a view/helper path and core claim/refund functions accept explicit ticket IDs. This is correctly Low/QA availability/UX risk, not H/M.
- Code Evidence: `contracts/JackpotTicketNFT.sol::getUserTickets` allocates an array of `totalTicketsBought` and loops over every ticket, calling `_getExtendedTicketInfo`, which in turn calls `jackpot.getUnpackedTicket`.

### M-69 / `dTJQMBRcGvOjORilkVUlf`
- Finding Title: Bonusball ranges above bit 255 corrupt packed tickets and can brick or mis-tier claims
- Decision: Invalid
- Confidence: High
- Bug Exists: No
- Severity Assessment: Low / QA
- Root Cause Family: `bonusball-bit-bound`
- Checklist Gates Passed: `Stage0, scope, code-path`
- Checklist Gates Failed: `bug-existence`
- Detailed Reason: The report's corruption and false-tier mechanism is incorrect for this source because overflowing `normalMax + bonusball` is checked as `uint8` arithmetic and reverts before a packed value is produced. A separate high-bonusball settlement DoS is real, but this finding is framed around erased bits and mis-tiered claims, which cannot occur through `insert` or `countTierMatchesWithBonusball` as written.
- Code Evidence: `contracts/lib/TicketComboTracker.sol::insert` and `countTierMatchesWithBonusball` use `_bonusball + _tracker.normalMax` with `uint8` operands. `contracts/Jackpot.sol::_validateAndStoreTickets` mints only after `insert` returns successfully.

### M-70 / `JNirY2A5L7N-4sAPEQSSD`
- Finding Title: Entropy provider migration can overwrite pending requests with colliding sequence numbers
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-provider-sequence-collision`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Pending entropy requests are keyed by sequence alone even though the callback includes provider identity and provider rotations are documented as future-only. A valid provider migration while old requests are pending can let a new-provider request overwrite the old sequence slot, causing misdelivery or loss of a Jackpot callback. The owner action is a normal maintenance workflow; the overwrite can be triggered by subsequent public requests.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::setEntropyProvider` updates `entropyProvider` without pending checks. `_storePendingRequest` writes `pending[sequence]`, and `entropyCallback` ignores its `provider` argument.

### M-71 / `qQy9NQ79-RRyMstIQufsw`
- Finding Title: Provider sequence collisions can overwrite pending entropy requests after provider rotation
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-provider-sequence-collision`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The same provider-scoped sequence collision is live: after an owner rotates providers with pending requests, a colliding new-provider sequence overwrites the old `pending[sequence]` entry. Because fulfillment ignores provider, entropy can be delivered to the wrong callback or the intended callback can be lost. This can block Jackpot settlement for an in-flight draw.
- Code Evidence: `contracts/ScaledEntropyProvider.sol` uses `mapping(uint64 => PendingRequest) private pending`; `_storePendingRequest` has no overwrite check; `entropyCallback(uint64 sequence, address /*provider*/, ...)` reads only `pending[sequence]`.

### M-72 / `J1Dqv-7r4rRmkAq_ocKmr`
- Finding Title: Bonusball bit positions at or above 256 corrupt ticket packing and bonusball match accounting
- Decision: Invalid
- Confidence: High
- Bug Exists: No
- Severity Assessment: Low / QA
- Root Cause Family: `bonusball-bit-bound`
- Checklist Gates Passed: `Stage0, scope, code-path`
- Checklist Gates Failed: `bug-existence`
- Detailed Reason: This false-match claim depends on Solidity evaluating a shift by 256 and returning zero. The actual source first evaluates a checked `uint8 + uint8`; if the bit position would exceed 255, the addition reverts and no corrupted packed ticket or winning ticket is produced. The finding does not prove the claimed claim-time aliasing bug.
- Code Evidence: `contracts/lib/TicketComboTracker.sol::insert` and `countTierMatchesWithBonusball` calculate the shift index from `uint8 _bonusball` and `uint8 normalMax`. Overflowing configurations revert before `Jackpot._calculateTicketTierId` can see aliased packed values.

### M-73 / `FgRFBZujzUDHT5TlbsOr6`
- Finding Title: Entropy provider rotation can overwrite pending randomness requests with colliding sequence numbers
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-provider-sequence-collision`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The provider/sequence namespace bug is real, and this version also notes that overwriting a reused mapping slot appends request data into the existing dynamic array rather than clearing it first. A normal provider rotation with pending requests can corrupt callback metadata and request parameters, which can strand or misdeliver Jackpot randomness. That is a material liveness/fairness impact.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::_storePendingRequest` assigns callback, selector, and context at `pending[sequence]` and then pushes each request into `pending[sequence].setRequests`. `entropyCallback` ignores provider and deletes only by sequence after delivery.

### M-74 / `HCZnk6xd_GnFZCH0X04fe`
- Finding Title: Bonusball bounds can exceed uint256 bit domain and corrupt ticket packing
- Decision: Invalid
- Confidence: High
- Bug Exists: No
- Severity Assessment: Low / QA
- Root Cause Family: `bonusball-bit-bound`
- Checklist Gates Passed: `Stage0, scope, code-path`
- Checklist Gates Failed: `bug-existence`
- Detailed Reason: The actual effect of `normalBallMax + bonusball` exceeding 255 is a checked arithmetic revert, not an accepted ticket with an omitted bonus bit. The report's packed-ticket corruption and collision path therefore does not exist as stated, though a distinct settlement DoS from the same missing bound is valid in findings that claim reversion.
- Code Evidence: `contracts/lib/TicketComboTracker.sol::insert` performs `1 << (_bonusball + _tracker.normalMax)` with `uint8` operands. `contracts/Jackpot.sol::_validateAndStoreTickets` validates bonusball range, then calls `insert`, then mints only after successful packing.

### M-76 / `fVsK-LAQ7IPQ6NH8i6z49`
- Finding Title: Valid high bonusball values overflow ticket bitpacking and can lock settlement
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `bonusball-settlement-dos`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: This finding correctly frames the missing bound as a reversion/DoS. A drawing can be initialized with a bonusball range whose high values cannot be encoded with `normalMax + bonusball <= 255`. If entropy returns one of those in-range high values, `countTierMatchesWithBonusball` reverts during the locked callback, blocking normal settlement and requiring emergency or privileged recovery.
- Code Evidence: `contracts/Jackpot.sol::_setNewDrawingState` sets `newDrawingState.bonusballMax` and calls `TicketComboTracker.init` without checking `normalBallMax + newBonusball <= 255`. `contracts/lib/TicketComboTracker.sol::countTierMatchesWithBonusball` packs the winning bonusball with checked `uint8` addition in `1 << (_bonusball + _tracker.normalMax)`.

### M-77 / `YrzefVI2gaRcji1F4O2lj`
- Finding Title: Provider rotation can collide Pyth sequence IDs and overwrite pending entropy requests
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-provider-sequence-collision`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Provider rotation while requests are pending is allowed, but pending requests are not namespaced by provider and are overwritten by sequence. Because Pyth V2 request identity is provider-scoped, a new provider can reuse an old sequence and corrupt or consume the wrong callback. This can strand an in-flight Jackpot drawing after a normal migration.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::setEntropyProvider` changes `entropyProvider`; `requestAndCallbackScaledRandomness` stores by returned `sequence`; `entropyCallback` ignores the provider argument.

### M-78 / `uxV4GUomOSDggeWGNWLva`
- Finding Title: Entropy provider rotation can overwrite pending randomness requests with colliding sequence numbers
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-provider-sequence-collision`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The sequence-only pending key makes old-provider and new-provider requests collide after rotation. Once a later request overwrites the slot, an old fulfillment no longer reaches the intended Jackpot callback, so the locked drawing can remain unsettled. The path is current code and not blocked by any pending-request guard.
- Code Evidence: `contracts/ScaledEntropyProvider.sol` declares `mapping(uint64 => PendingRequest) private pending`; `_storePendingRequest(sequence, ...)` overwrites that slot; `entropyCallback` uses only `sequence` to load and delete.

### M-79 / `Eo7DO9DiVWIGLlFfnfy0D`
- Finding Title: Entropy pending requests are keyed only by sequence and collide across provider rotations
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-provider-sequence-collision`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The provider argument supplied by Pyth is ignored even though sequence numbers are provider-local. A provider rotation can therefore cause callback metadata for one request domain to be used for another. For Jackpot, that can misdeliver or lose the request needed to unlock settlement.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::entropyCallback(uint64 sequence, address /*provider*/, ...)` explicitly discards provider and reads `pending[sequence]`; `setEntropyProvider` can be called without draining pending requests.

### M-80 / `epV1g2oCGbgjEd1MKVJMs`
- Finding Title: Invalid normalBallMax plus bonusballMax configuration causes ticket bit collisions and false jackpot-tier claims
- Decision: Invalid
- Confidence: High
- Bug Exists: No
- Severity Assessment: Low / QA
- Root Cause Family: `bonusball-bit-bound`
- Checklist Gates Passed: `Stage0, scope, code-path`
- Checklist Gates Failed: `bug-existence`
- Detailed Reason: The stated false jackpot-tier claim relies on high bonusball shifts silently producing zero. In the real code, high values overflow the checked `uint8` addition before shifting, so two high bonusballs cannot be stored as identical packed tickets through `TicketComboTracker.insert`. The missing bound can cause reverts, not the claimed false payouts.
- Code Evidence: `contracts/lib/TicketComboTracker.sol::insert` computes the bonus shift from `uint8` values and reverts for sums above 255. Settlement counting and claim-time tiering never receive successfully packed overflow-bonus tickets from this path.

### M-81 / `8enSiBNKwJR94z-IbBs8s`
- Finding Title: Bonusball bit positions at 256 or higher corrupt packed tickets and let non-winning tickets claim as bonus matches
- Decision: Invalid
- Confidence: High
- Bug Exists: No
- Severity Assessment: Low / QA
- Root Cause Family: `bonusball-bit-bound`
- Checklist Gates Passed: `Stage0, scope, code-path`
- Checklist Gates Failed: `bug-existence`
- Detailed Reason: Non-winning tickets cannot be packed with omitted high bonus bits as described because the packing expression reverts on `uint8` addition overflow. The report's divergence between settlement winner counts and claim-time false bonusball matches is therefore not reachable. Reverting high values are covered by the separate settlement-DoS finding.
- Code Evidence: `contracts/lib/TicketComboTracker.sol::insert` and `countTierMatchesWithBonusball` both use checked `uint8` arithmetic for `_bonusball + normalMax`; `contracts/Jackpot.sol::claimWinnings` only tiers tickets whose packed value was minted after successful `insert`.

### M-82 / `c1LUgH768vdleCQv2vutO`
- Finding Title: Bonusball bit positions above 255 alias distinct tickets and can create false winning claims
- Decision: Invalid
- Confidence: High
- Bug Exists: No
- Severity Assessment: Low / QA
- Root Cause Family: `bonusball-bit-bound`
- Checklist Gates Passed: `Stage0, scope, code-path`
- Checklist Gates Failed: `bug-existence`
- Detailed Reason: Distinct high bonusballs do not alias into identical stored tickets through the production library because the high shift index is not reached; checked `uint8` addition reverts first. Thus the claimed false winning claims and overpayment path do not exist in current code.
- Code Evidence: `contracts/lib/TicketComboTracker.sol::insert` packs the bonusball using `_bonusball + _tracker.normalMax` with both operands as `uint8`. `Jackpot._validateAndStoreTickets` mints only after this library call succeeds.

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

### M-84 / `up_Ag7R-61tUyZTohHm-S`
- Finding Title: Changing entropy during a locked drawing permanently rejects the pending callback
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-authority-drift`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: A locked drawing authorizes callbacks against the live global entropy address rather than the request-time source. Changing entropy during the locked window makes the old request's valid callback revert and the new entropy contract has no corresponding request. This can leave the drawing locked until privileged intervention.
- Code Evidence: `contracts/Jackpot.sol::runJackpot` requests randomness through the current `entropy`; `setEntropy` has no lock guard; `onlyEntropy` on `scaledEntropyCallback` checks `msg.sender` against the updated global.

### M-85 / `z6UFaJb3e9UR2WAFVsTsW`
- Finding Title: Entropy requests from different providers collide because pending requests are keyed only by sequence
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-provider-sequence-collision`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The pending request key omits provider identity, so provider-local sequence numbers can collide after a provider rotation. Because there is no overwrite protection and callback delivery ignores provider, in-flight Jackpot randomness can be lost or misrouted. The issue is current and material under a normal migration workflow with pending requests.
- Code Evidence: `contracts/ScaledEntropyProvider.sol` has `mapping(uint64 => PendingRequest) private pending`; `_storePendingRequest` overwrites by sequence; `entropyCallback` ignores `provider`.

### M-86 / `VumA1LcRqul120TjtxTRb`
- Finding Title: Rotating payoutCalculator after settlement can zero or change unclaimed winning tickets
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `payout-calculator-drift`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Settled tier payouts live in whichever calculator was active at settlement, but claims read the live global calculator. Rotating to a new calculator before all winners claim can make historical tickets read zero or incompatible payouts. This violates the expected future-only behavior of payout changes and can burn winning tickets for no payment.
- Code Evidence: `contracts/Jackpot.sol::claimWinnings` burns the ticket and then reads `payoutCalculator.getTierPayout(drawingId, tierId)` from the mutable global. `setPayoutCalculator` does not snapshot per drawing or migrate old payouts.

### M-87 / `dUualzZdMcwOLTWqMmzY9`
- Finding Title: Entropy provider rotation lets per-provider sequence collisions overwrite pending jackpot randomness
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-provider-sequence-collision`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: After a valid provider rotation, public requests through the new provider can collide with an old provider's pending sequence and replace its callback metadata. If the old sequence belonged to a Jackpot drawing, the callback needed for settlement can be consumed by another requester, leaving the drawing locked. No provider namespace or overwrite guard prevents this.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::requestAndCallbackScaledRandomness` stores by returned `sequence`; `setEntropyProvider` changes the provider without pending counts; `entropyCallback` discards provider.

### M-88 / `SIZoP2wmWR2-DqOorjm21`
- Finding Title: Payout calculator rotation can zero or alter unclaimed winnings from already-settled drawings
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `payout-calculator-drift`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Already-settled ticket claims are not bound to the calculator that stored their tier payouts. A later calculator rotation changes the source used by `claimWinnings`, so unclaimed winners can receive zero or different payouts despite the drawing having settled. This is material user fund loss from global pointer drift.
- Code Evidence: `contracts/Jackpot.sol::_calculateDrawingUserWinnings` stores payouts in the active calculator at settlement, while `claimWinnings` later queries the live `payoutCalculator`; `setPayoutCalculator` simply replaces that global pointer.

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

### M-92 / `MZxhqjaBRZw38hY7UoPkd`
- Finding Title: Allowed normalBallMax plus bonusballMin can exceed the 255-bit ticket packing domain
- Decision: Invalid
- Confidence: High
- Bug Exists: No
- Severity Assessment: Low / QA
- Root Cause Family: `bonusball-bit-bound`
- Checklist Gates Passed: `Stage0, scope, code-path`
- Checklist Gates Failed: `bug-existence`
- Detailed Reason: To the extent this finding claims packed-ticket corruption or bit erasure, it is not correct: over-bound high bonusball values revert during checked `uint8` addition before packing. The missing bound is real only as a purchase/settlement reversion issue, not as accepted corrupted tickets or false claims.
- Code Evidence: `contracts/lib/TicketComboTracker.sol` packs bonusballs with `_bonusball + _tracker.normalMax` using `uint8` values. Overflowing configurations revert in `insert` or `countTierMatchesWithBonusball`.

### M-93 / `PR2G588QO-Kwr3c5Fhe4w`
- Finding Title: Provider sequence collision can overwrite pending jackpot randomness and leave a drawing permanently locked
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-provider-sequence-collision`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: This is a concrete Jackpot-impact version of the provider sequence namespace bug. If the ScaledEntropyProvider owner rotates providers while a Jackpot request is pending, a public request on the new provider can collide with and overwrite the old sequence. The old fulfillment then no longer calls Jackpot, so the drawing can remain locked.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::_storePendingRequest` has no check for an existing `pending[sequence]`, and `entropyCallback` ignores provider. `contracts/Jackpot.sol::runJackpot` depends on that callback to unlock and advance the drawing.

### M-94 / `hwZlFmHk1XCY6j0kVxNjK`
- Finding Title: Payout calculator rotation reprices already settled unclaimed tickets
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `payout-calculator-drift`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: Historical claims read from the live `payoutCalculator`, not the calculator that settled the drawing. A legitimate future-intended migration can therefore make old winners read zero or altered tier payouts and lose matured winnings when their tickets are burned. This is a material accounting bug.
- Code Evidence: `contracts/Jackpot.sol::claimWinnings` calls `payoutCalculator.getTierPayout(drawingId, tierId)` after burn. `setPayoutCalculator` provides no migration or drawing-level snapshot.

### M-96 / `4N5SxmjaZ5ZFkldy20qul`
- Finding Title: Rotating payoutCalculator makes old winning tickets read payouts from the wrong calculator
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `payout-calculator-drift`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The wrong-calculator read is exactly present. Old tickets use their original `drawingId`, but payout lookup goes through the current global calculator rather than a per-drawing calculator. A post-settlement rotation can underpay or zero unclaimed winners and burn their tickets in the process.
- Code Evidence: `contracts/Jackpot.sol::claimWinnings` retrieves `ticketInfo.drawingId` but then calls the mutable global `payoutCalculator.getTierPayout`. `contracts/Jackpot.sol::setPayoutCalculator` has no claim-outstanding guard.

### M-97 / `c1taCy-rUpLHmD9KeiD5V`
- Finding Title: Entropy address rotation during a pending request permanently rejects the valid callback
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-authority-drift`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The pending request source is not stored, and callback authorization uses the live entropy address. Rotating entropy after `runJackpot` but before fulfillment rejects the valid old callback and leaves the locked drawing without a normal settlement path. This is within the benchmark's explicit admin-mid-flow concern.
- Code Evidence: `contracts/Jackpot.sol::runJackpot` calls the current `entropy`; `setEntropy` mutates the same global; `onlyEntropy` on `scaledEntropyCallback` compares against the updated global.

### M-98 / `bqDE4tP4lSLVtXr7da9Tk`
- Finding Title: Changing Jackpot.entropy after runJackpot makes the in-flight entropy callback unauthorized and bricks settlement
- Decision: Valid
- Confidence: High
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-authority-drift`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: This is the same live-authority drift with clear code support: the request is made to `E1`, but after `setEntropy(E2)` only `E2` can call `scaledEntropyCallback`. Since `E2` has no pending request, the active drawing can stay locked until governance recovery. It is a material liveness failure under normal upgrade/migration behavior.
- Code Evidence: `contracts/Jackpot.sol::setEntropy` lacks a pending/locked guard. `scaledEntropyCallback` uses `onlyEntropy`, and `runJackpot` stores no request-time entropy address or request ID.

### M-100 / `pxtxFK-1LCZ2MRtDy6Fgn`
- Finding Title: Provider sequence collisions can replay entropy into the wrong request after entropy provider rotation
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-provider-sequence-collision`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: The provider-scoped sequence collision can cause entropy for one provider/request domain to be delivered using another request's pending metadata. This can misroute or lose callbacks after a normal provider rotation, including Jackpot drawing callbacks. The absence of provider namespacing or overwrite checks is the independent root cause.
- Code Evidence: `contracts/ScaledEntropyProvider.sol` stores pending requests in `mapping(uint64 => PendingRequest)`, ignores the `provider` callback argument, and allows `setEntropyProvider` while pending requests exist.

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

### M-102 / `Qet8mUOMbbhvJ91BSgZAI`
- Finding Title: Provider-scoped entropy sequence numbers collide in ScaledEntropyProvider and can strand Jackpot drawings
- Decision: Valid
- Confidence: Med
- Bug Exists: Yes
- Severity Assessment: Medium
- Root Cause Family: `entropy-provider-sequence-collision`
- Checklist Gates Passed: `Stage0, scope, root-cause, safeguards, exploitability, impact`
- Checklist Gates Failed: `-`
- Detailed Reason: After provider rotation, an unprivileged requester can eventually create a new-provider request with the same provider-local sequence as an old pending Jackpot request, overwriting the callback metadata. The old fulfillment is then delivered to the wrong callback or consumes the slot, leaving Jackpot locked. The root is current code and the impact is material when migration occurs with pending requests.
- Code Evidence: `contracts/ScaledEntropyProvider.sol::requestAndCallbackScaledRandomness` stores `pending[sequence]` for `msg.sender`, `setEntropyProvider` changes the provider without pending isolation, and `entropyCallback` ignores provider when loading/deleting the pending request.
