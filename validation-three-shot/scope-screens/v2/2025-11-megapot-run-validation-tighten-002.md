# 2025-11-megapot Three-Shot Stage 1 Scope Screen

Status: In progress
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

## Decisions

| Finding | Finding Title | Decision | Confidence | Reason Category | Reason |
| --- | --- | --- | --- | --- | --- |
| H-1 | Arbitrary bridge call leaves reusable USDC allowance that can drain later bridge-manager funds | Keep | High | in-scope | Bridge custody and arbitrary route handling are scoped and no V12 or known-issue match was found. |
| M-2 | ECDSA-only bridge claims permanently lock tickets owned by ERC-1271 smart wallets | Keep | Medium | in-scope | Bridge signatures and ticket custody are scoped and docs do not exclude contract-wallet recipients. |
| M-3 | Entropy callback ignores the request id, allowing a stale request to settle a later locked drawing | Keep | High | in-scope | Entropy request binding and stale callbacks are scoped randomness and lifecycle concerns. |
| M-4 | Bridge claim signatures never expire and can be executed long after the user signed them | Keep | High | in-scope | EIP-712 replay and stale bridge authorization are explicit review priorities. |
| M-5 | Changing entropy provider while a drawing is pending permanently blocks settlement | Keep | High | in-scope | Mid-flow entropy or global parameter changes affecting active drawings are explicitly scoped. |
| M-6 | LPs can frontrun pool-cap reductions to make governance risk updates revert | Keep | High | in-scope | Permissionless LP state movement blocking an otherwise valid cap reduction is not a configuration-only issue. |
| M-7 | Changing payoutCalculator mid-drawing can settle active tickets with an unsnapshotted calculator | Keep | High | in-scope | Mid-drawing payout calculator changes affecting active drawings are explicitly scoped. |
| M-8 | Changing Jackpot entropy address while a draw is pending bricks the authorized callback | Keep | High | in-scope | Entropy address rotation during a pending draw matches scoped mid-flow liveness risk. |
| M-9 | Bonusballs above bit capacity are erased, allowing uncounted tickets to claim winning bonusball payouts | Keep | High | in-scope | Bonusball bitpacking boundaries are explicit scope and this is distinct from the V12 uint8 downcast. |
| M-10 | No-referral winner share is credited to the current drawing instead of the settled drawing | Keep | High | in-scope | No-referral fallback is known but wrong drawing credit is a distinct LP value-flow issue. |
| M-11 | Changing entropy while a drawing is pending bricks the original callback and leaves the jackpot locked | Keep | High | in-scope | Entropy rotation during pending settlement is a scoped stuck-progression concern. |
| H-12 | Arbitrary bridge call lets a winning ticket holder steal other users' custodied ticket NFTs | Keep | High | in-scope | Bridge manager custody and arbitrary bridge calls are scoped with no known exclusion. |
| M-13 | Unfulfillable SetRequests are accepted and later revert during entropy fulfillment | Keep | Medium | in-scope | Request feasibility validation is distinct from the V12 gas-limit and overpayment issue. |
| M-14 | Mid-drawing payout calculator update causes active drawing to settle with zero stored payouts | Keep | High | in-scope | Unsnapshotted payout calculator changes are within the scoped admin-change concern. |
| M-15 | Missing bonusball packing bound corrupts tickets when normalBallMax + bonusballMax reaches 256 | Keep | High | in-scope | Bitpacking boundary errors are explicit scope and not the same root as the V12 bonus count downcast. |
| M-16 | Bridge claim signatures have no nonce or deadline and remain executable indefinitely | Keep | High | in-scope | Stale EIP-712 bridge signatures are an explicit review priority. |
| M-17 | ECDSA-only bridge authorization permanently locks tickets owned by ERC-1271 smart wallets | Keep | Medium | in-scope | Bridge authorization is scoped and contract-wallet recipients are not excluded by the docs. |
| M-18 | Changing entropy providers can overwrite pending randomness requests with colliding sequence IDs | Keep | High | in-scope | Provider sequence binding is a randomness lifecycle issue distinct from the V12 event issue. |
| M-19 | Emergency refunds for referred tickets use live referralFee and can underpay old ticket holders | Exclude | Medium | known-issue-or-oos | Referred-ticket emergency refund shortfall overlaps the public emergency refund policy and depends on mutable admin fee state. |
| M-20 | Bridge ticket purchases use live global ticketPrice instead of the active drawing price, causing overcharges or purchase DoS | Keep | High | in-scope | TicketPrice and global parameter changes mid-drawing are explicit scope. |
| M-21 | Perpetual EIP-712 bridge signatures can be executed after user intent expires | Keep | High | in-scope | Bridge signature replay and stale authorization are scoped. |
| M-22 | Nonce-less ticket withdrawal signatures stay valid indefinitely and can steal later winning tickets | Keep | High | in-scope | Missing bridge signature nonce or expiry is within the EIP-712 replay focus. |
| M-23 | Pending LP deposits can be rounded to zero after accumulator inflation | Keep | High | in-scope | Material zero-share LP loss exceeds the known dust-only rounding exclusion. |
| M-24 | No-referral winner shares are credited to the claim-time drawing instead of the settled drawing | Keep | High | in-scope | Wrong LP cohort credit is distinct from the known no-referral fallback policy. |
| M-25 | Full uint256 range requests are accepted but overflow during randomness scaling | Keep | Medium | in-scope | Full-range request overflow is a distinct request-validation path from V12 overpayment. |
| H-26 | Pending LP deposits can be zeroed by accumulator inflation before settlement | Keep | High | in-scope | Zeroing deposits is material LP accounting harm beyond expected rounding dust. |
| M-27 | Tier zero winners are excluded from settlement obligations but can still claim stored tier-zero payouts | Keep | High | in-scope | Winner counting and payout allocation are central scoped invariants. |
| M-28 | Full uint256 range requests overflow during scaling after being accepted | Keep | Medium | in-scope | Accepted full-range overflow is not a listed V12 duplicate. |
| M-29 | Bridge claim signatures have no nonce or deadline, allowing stale execution through obsolete bridge routes | Keep | High | in-scope | Stale bridge route authorization is within scoped EIP-712 replay review. |
| M-30 | Bridge ticket and winnings signatures never expire and can be executed long after user intent changes | Keep | High | in-scope | Non-expiring bridge signatures are not excluded by scope or V12 context. |
| L-31 | Burned ticket metadata remains readable as active ticket data in JackpotTicketNFT | Keep | Medium | in-scope | Stale ticket getter behavior is in scoped code and not the same root as V12 tokenURI. |
| M-32 | Invalid full-range randomness requests are accepted and later revert during entropy fulfillment | Keep | Medium | in-scope | Randomness request bounds are scoped and materially distinct from V12 gas overpayment. |
| M-33 | Impossible no-replacement draws are accepted and can fail only after entropy is consumed | Keep | Medium | in-scope | No-replacement feasibility validation is not listed as a known issue or V12 duplicate. |
| M-34 | Missing feasibility validation lets callers create entropy requests that can never be fulfilled | Keep | Medium | in-scope | Entropy SetRequest feasibility is in scope and not benchmark-declared accepted behavior. |
| M-35 | Stale entropy callbacks can settle the wrong Jackpot drawing because request IDs are not bound to drawing IDs | Keep | High | in-scope | Request-to-drawing binding is a scoped entropy lifecycle concern. |
| M-36 | Changing entropy provider while a request is pending permanently rejects the valid callback and locks the drawing | Keep | High | in-scope | Valid future-facing entropy rotation affecting an active draw is within scoped admin-change risk. |
| M-37 | Residual bridge approvals let approved spenders drain future USDC entering JackpotBridgeManager | Keep | High | in-scope | Bridge allowance cleanup and custody are scoped and not prior-listed. |
| H-38 | Opaque bridge calldata can pass balance checks while sending winnings to an attacker | Keep | High | in-scope | Bridge calldata and claimed fund routing are explicit bridge accounting concerns. |
| H-39 | Arbitrary bridge call can transfer other users' custodied ticket NFTs from JackpotBridgeManager | Keep | High | in-scope | Bridge manager custody of ticket NFTs is scoped and no V12 duplicate applies. |
| M-40 | Invalid no-replacement requests are accepted then permanently fail during entropy fulfillment | Keep | Medium | in-scope | No-replacement request validation is distinct from the known V12 entropy gas issue. |
| M-41 | ECDSA-only bridge signatures permanently lock tickets and winnings owned by smart wallets | Keep | Medium | in-scope | Bridge signature validation is scoped and smart-wallet recipients are not excluded. |
| M-42 | Stale bridge claim signatures can be replayed indefinitely until the ticket is consumed | Keep | High | in-scope | EIP-712 replay resistance is explicitly scoped. |
| M-43 | No-referral win share is credited to the current drawing, letting later LPs capture prior drawing value | Keep | High | in-scope | Wrong drawing credit is materially different from the known no-referral fallback. |
| M-44 | Ticket purchases remain open after drawingTime until runJackpot is mined | Keep | Medium | in-scope | Ticket purchase windows and drawing lifecycle are scoped and no known exclusion applies. |
| M-45 | LPs can frontrun cap reductions by depositing before setGovernancePoolCap reverts | Keep | High | in-scope | Permissionless deposits blocking a risk-reducing cap update should be kept under the stated nuance. |
| M-46 | Unsnapshotted drawing payouts can be calculated as zero in GuaranteedMinimumPayoutCalculator.calculateAndStoreDrawingUserWinnings | Keep | High | in-scope | Payout snapshots and mid-drawing global changes are scoped accounting concerns. |
| M-47 | No-referral winning share is credited to the active drawing, letting winners redirect prior LP value to future LPs | Keep | High | in-scope | Prior-drawing value redirected to current LPs is distinct from the known referral fallback. |
| H-48 | Bonusball bit overflow corrupts tier calculation and can brick valid Jackpot.claimWinnings payouts | Keep | High | in-scope | Bonusball packing boundary and claim-time tiering are explicit audit focus. |
| M-49 | runJackpot requires an unrewarded caller to pay entropy fees for protocol liveness | Keep | Medium | in-scope | Jackpot progression and entropy fee liveness are scoped and not a V12 overpayment duplicate. |
| M-50 | Unpaid jackpot execution leaves expired drawings open for late ticket buyers | Keep | Medium | in-scope | Drawing progression and post-deadline ticket purchase behavior are scoped. |
| M-51 | Emergency refunds use live referralFee, letting self-referrers drain value after fee decreases | Exclude | Medium | known-issue-or-oos | Referred-ticket emergency refund accounting is publicly known and depends on trusted fee changes. |
| M-52 | Unbounded request ranges and batch sizes can make entropy callbacks run out of gas | Keep | Medium | in-scope | Request-size bounds are a distinct entropy validation issue from V12 gas-limit overpayment. |
| M-53 | Reverting requester callback rolls back pending cleanup and leaves consumed entropy requests stale | Keep | Medium | in-scope | ScaledEntropyProvider callback cleanup is in scope and not prior-listed. |
| M-54 | Bridge claim signatures have no nonce or deadline, allowing leaked or old authorizations to be executed later | Keep | High | in-scope | Leaked or stale bridge EIP-712 authorizations are scoped. |
| L-55 | claimWinnings leaves burned tickets recorded as owned in bridge accounting | Keep | Medium | in-scope | Bridge ticket accounting is scoped and no known issue covers this state. |
| H-56 | Arbitrary bridge call lets a winning claimant transfer other users' custodied ticket NFTs | Keep | High | in-scope | Arbitrary bridge calls affecting custodied NFTs are scoped bridge custody risk. |
| M-57 | Opaque bridge calldata can steal all claimed winnings because claimWinnings does not bind amount or recipient | Keep | High | in-scope | Bridge amount and recipient binding are explicit bridge accounting concerns. |
| H-58 | Signed bridge route can approve one address while arbitrary call drains the claimed USDC through another address | Keep | High | in-scope | Arbitrary bridge approval and call semantics are scoped. |
| M-59 | Arbitrary bridge call can leave reusable USDC allowance and steal pre-existing bridge surplus | Keep | High | in-scope | Residual approvals in bridge manager custody are not excluded or V12-listed. |
| H-60 | ScaledEntropyProvider reuses one entropy seed for all random sets, correlating normal balls and bonusball | Keep | High | in-scope | Randomness fairness and scaling are explicit audit priorities. |
| M-61 | Large no-replacement ranges can force fulfillment-time memory exhaustion | Keep | Medium | in-scope | Range-size bounds are a distinct entropy request-validation concern. |
| M-62 | Unbounded request arrays can make entropy fulfillment run out of gas and leave drawings stuck | Keep | Medium | in-scope | Entropy fulfillment gas from request sizing is scoped and not the V12 gas-limit issue. |
| M-63 | Invalid no-replacement entropy requests are accepted then permanently revert during fulfillment | Keep | Medium | in-scope | No-replacement feasibility failures are not listed as known or V12 issues. |
| M-64 | Full uint256 ranges are accepted but overflow during scaled randomness fulfillment | Keep | Medium | in-scope | Accepted full-range overflow is a distinct in-scope request validation issue. |
| M-65 | Missing request bounds allow entropy requests that pass validation but revert or exhaust gas during fulfillment | Keep | Medium | in-scope | Missing SetRequest bounds differ materially from V12 overpayment and gas-limit forwarding. |
| L-66 | Burned JackpotTicketNFT tokens keep active-looking ticket metadata in public getters | Keep | Medium | in-scope | Public getter stale metadata is not decisively covered by V12 tokenURI. |
| L-67 | Unbounded per-user ticket enumeration can make JackpotTicketNFT.getUserTickets unusable for large holders | Keep | Medium | in-scope | JackpotTicketNFT user-ticket tracking is scoped and no benchmark exclusion applies. |
| M-68 | Referral fee changes alter emergency refunds for already-purchased tickets | Exclude | Medium | known-issue-or-oos | Emergency refunds for referred tickets are publicly known policy behavior. |
| M-69 | Bonusball ranges above bit 255 corrupt packed tickets and can brick or mis-tier claims | Keep | High | in-scope | Bitpacking boundary errors are explicit scope and distinct from V12 downcast. |
| M-70 | Entropy provider migration can overwrite pending requests with colliding sequence numbers | Keep | High | in-scope | Provider-scoped sequence binding is a randomness lifecycle issue, not a known duplicate. |
| M-71 | Provider sequence collisions can overwrite pending entropy requests after provider rotation | Keep | High | in-scope | Entropy provider rotation with pending requests is not benchmark-declared out of scope. |
| M-72 | Bonusball bit positions at or above 256 corrupt ticket packing and bonusball match accounting | Keep | High | in-scope | Bonusball bit positions and claim tiering are explicit review focus. |
| M-73 | Entropy provider rotation can overwrite pending randomness requests with colliding sequence numbers | Keep | High | in-scope | Sequence collision after provider rotation is not V12-listed and affects scoped entropy flow. |
| M-74 | Bonusball bounds can exceed uint256 bit domain and corrupt ticket packing | Keep | High | in-scope | Ticket bitpacking boundaries are scoped and no decisive known issue applies. |
| M-75 | Emergency refunds for referred tickets use mutable referral fee instead of purchase-time fee | Exclude | Medium | known-issue-or-oos | Referred-ticket emergency refund amount issues overlap the public emergency refund exclusion. |
| M-76 | Valid high bonusball values overflow ticket bitpacking and can lock settlement | Keep | High | in-scope | Bonusball range and settlement liveness are scoped and distinct from V12 uint8 downcast. |
| M-77 | Provider rotation can collide Pyth sequence IDs and overwrite pending entropy requests | Keep | High | in-scope | Provider sequence collisions are not a V12 duplicate and affect in-scope entropy requests. |
| M-78 | Entropy provider rotation can overwrite pending randomness requests with colliding sequence numbers | Keep | High | in-scope | Pending randomness metadata binding is a scoped lifecycle concern. |
| M-79 | Entropy pending requests are keyed only by sequence and collide across provider rotations | Keep | High | in-scope | Provider namespacing of pending entropy requests is not known or excluded. |
| M-80 | Invalid normalBallMax plus bonusballMax configuration causes ticket bit collisions and false jackpot-tier claims | Keep | High | in-scope | normalBallMax and bonusball bitpacking boundaries are explicit scope. |
| M-81 | Bonusball bit positions at 256 or higher corrupt packed tickets and let non-winning tickets claim as bonus matches | Keep | High | in-scope | Bonusball bit domain corruption is scoped ticket-tiering risk. |
| M-82 | Bonusball bit positions above 255 alias distinct tickets and can create false winning claims | Keep | High | in-scope | Ticket packing aliasing and false claims are explicit audit focus. |
| M-83 | Bonusball ranges above the bit-packing boundary can make entropy settlement revert and lock the drawing | Keep | High | in-scope | Settlement lock from bitpacking boundary failure is scoped liveness and ticket logic. |
| M-84 | Changing entropy during a locked drawing permanently rejects the pending callback | Keep | High | in-scope | Entropy updates affecting locked drawings are covered by the mid-flow global-change focus. |
| M-85 | Entropy requests from different providers collide because pending requests are keyed only by sequence | Keep | High | in-scope | Provider-scoped request identity is a scoped randomness integration concern. |
| M-86 | Rotating payoutCalculator after settlement can zero or change unclaimed winning tickets | Keep | High | in-scope | Global payout calculator changes affecting prior drawings are explicitly scoped. |
| M-87 | Entropy provider rotation lets per-provider sequence collisions overwrite pending jackpot randomness | Keep | High | in-scope | Provider rotation plus public request collision is not a trusted-role-only configuration issue. |
| M-88 | Payout calculator rotation can zero or alter unclaimed winnings from already-settled drawings | Keep | High | in-scope | Prior-drawing claim effects from mutable payoutCalculator are explicit scope. |
| M-89 | Allowed economics can set drawing accumulator to zero and brick LP consolidation | Exclude | Medium | configuration-nonissue | Harm depends solely on trusted governance selecting extreme unsafe economic parameters. |
| M-90 | Small normalBallMax values can permanently DoS settlement through impossible-tier combination asserts | Keep | Medium | in-scope | normalBallMax changes and settlement math are scoped and the parameter is not explicitly excluded. |
| M-91 | Payout calculator rotation reprices already initialized or settled ticket claims | Keep | High | in-scope | Mutable payout calculator effects on active or prior drawings are scoped. |
| M-92 | Allowed normalBallMax plus bonusballMin can exceed the 255-bit ticket packing domain | Keep | High | in-scope | normalBallMax plus bonusball bit boundary is specifically called out for review. |
| M-93 | Provider sequence collision can overwrite pending jackpot randomness and leave a drawing permanently locked | Keep | High | in-scope | Entropy provider sequence collision is in scoped randomness lifecycle and not V12-listed. |
| M-94 | Payout calculator rotation reprices already settled unclaimed tickets | Keep | High | in-scope | Prior-drawing payout lookup through mutable global state is explicitly scoped. |
| L-95 | Allowed normalBallMax values below 10 can brick payout calculation during settlement | Keep | Medium | in-scope | normalBallMax settlement effects are scoped and not benchmark-declared invalid configuration. |
| M-96 | Rotating payoutCalculator makes old winning tickets read payouts from the wrong calculator | Keep | High | in-scope | Mutable payoutCalculator affecting historical tickets is in scope. |
| M-97 | Entropy address rotation during a pending request permanently rejects the valid callback | Keep | High | in-scope | Pending entropy callback authorization is a scoped mid-flow global-change issue. |
| M-98 | Changing Jackpot.entropy after runJackpot makes the in-flight entropy callback unauthorized and bricks settlement | Keep | High | in-scope | In-flight entropy request binding is scoped and not known or V12 duplicate. |
| M-99 | Emergency refunds leave burned tickets counted and can corrupt resumed drawing settlement | Exclude | High | known-issue-or-oos | Emergency mode is documented as an unrecoverable stuck-system path, so resumed settlement corruption is out of scope. |
| M-100 | Provider sequence collisions can replay entropy into the wrong request after entropy provider rotation | Keep | High | in-scope | Provider-scoped entropy sequence handling is not excluded and not V12-listed. |
| M-101 | Updating the payout calculator breaks already-settled ticket claims by reading payouts from the new calculator | Keep | High | in-scope | Global payout calculator changes affecting prior drawings are explicit scope. |
| M-102 | Provider-scoped entropy sequence numbers collide in ScaledEntropyProvider and can strand Jackpot drawings | Keep | High | in-scope | Pending entropy request namespacing is scoped randomness integration risk. |
| M-103 | Validator/provider-manipulable entropy request can bias jackpot winning numbers | Exclude | High | trusted-role-oos | Exploit requires dishonest configured entropy provider and validator, which are trusted external dependencies. |
| M-104 | LP deposits can frontrun pool-cap reductions and block risk-reducing governance updates | Keep | High | in-scope | The stated nuance keeps permissionless LP state movement that blocks a valid risk-reducing update. |
| M-105 | LPs can frontrun pool-cap reductions by depositing before JackpotLPManager.setLPPoolCap executes | Keep | High | in-scope | Permissionless deposit obstruction of a valid cap reduction is not excluded as trusted-role misconfiguration. |
