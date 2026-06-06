# 2025-11-megapot Three-Shot Stage 2 Unsupported-Token Screen

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
| H-1 | Arbitrary bridge call leaves reusable USDC allowance that can drain later bridge-manager funds | Keep | High | supported-behavior | Uses ordinary USDC allowance persistence, not unsupported token behavior. |
| M-2 | ECDSA-only bridge claims permanently lock tickets owned by ERC-1271 smart wallets | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-3 | Entropy callback ignores the request id, allowing a stale request to settle a later locked drawing | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-4 | Bridge claim signatures never expire and can be executed long after the user signed them | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-5 | Changing entropy provider while a drawing is pending permanently blocks settlement | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-6 | LPs can frontrun pool-cap reductions to make governance risk updates revert | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-7 | Changing payoutCalculator mid-drawing can settle active tickets with an unsnapshotted calculator | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-8 | Changing Jackpot entropy address while a draw is pending bricks the authorized callback | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-9 | Bonusballs above bit capacity are erased, allowing uncounted tickets to claim winning bonusball payouts | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-10 | No-referral winner share is credited to the current drawing instead of the settled drawing | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-11 | Changing entropy while a drawing is pending bricks the original callback and leaves the jackpot locked | Keep | High | supported-behavior | No unsupported token semantics involved. |
| H-12 | Arbitrary bridge call lets a winning ticket holder steal other users' custodied ticket NFTs | Keep | High | supported-behavior | Uses ordinary USDC allowance transfer behavior, not unsupported token behavior. |
| M-13 | Unfulfillable SetRequests are accepted and later revert during entropy fulfillment | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-14 | Mid-drawing payout calculator update causes active drawing to settle with zero stored payouts | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-15 | Missing bonusball packing bound corrupts tickets when normalBallMax + bonusballMax reaches 256 | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-16 | Bridge claim signatures have no nonce or deadline and remain executable indefinitely | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-17 | ECDSA-only bridge authorization permanently locks tickets owned by ERC-1271 smart wallets | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-18 | Changing entropy providers can overwrite pending randomness requests with colliding sequence IDs | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-20 | Bridge ticket purchases use live global ticketPrice instead of the active drawing price, causing overcharges or purchase DoS | Keep | High | supported-behavior | Uses documented USDC ticket payments, not unsupported asset behavior. |
| M-21 | Perpetual EIP-712 bridge signatures can be executed after user intent expires | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-22 | Nonce-less ticket withdrawal signatures stay valid indefinitely and can steal later winning tickets | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-23 | Pending LP deposits can be rounded to zero after accumulator inflation | Keep | High | supported-behavior | Uses documented USDC accounting, not unsupported token behavior. |
| M-24 | No-referral winner shares are credited to the claim-time drawing instead of the settled drawing | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-25 | Full uint256 range requests are accepted but overflow during randomness scaling | Keep | High | supported-behavior | No unsupported token semantics involved. |
| H-26 | Pending LP deposits can be zeroed by accumulator inflation before settlement | Keep | High | supported-behavior | Uses documented USDC accounting, not unsupported token behavior. |
| M-27 | Tier zero winners are excluded from settlement obligations but can still claim stored tier-zero payouts | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-28 | Full uint256 range requests overflow during scaling after being accepted | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-29 | Bridge claim signatures have no nonce or deadline, allowing stale execution through obsolete bridge routes | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-30 | Bridge ticket and winnings signatures never expire and can be executed long after user intent changes | Keep | High | supported-behavior | No unsupported token semantics involved. |
| L-31 | Burned ticket metadata remains readable as active ticket data in JackpotTicketNFT | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-32 | Invalid full-range randomness requests are accepted and later revert during entropy fulfillment | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-33 | Impossible no-replacement draws are accepted and can fail only after entropy is consumed | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-34 | Missing feasibility validation lets callers create entropy requests that can never be fulfilled | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-35 | Stale entropy callbacks can settle the wrong Jackpot drawing because request IDs are not bound to drawing IDs | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-36 | Changing entropy provider while a request is pending permanently rejects the valid callback and locks the drawing | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-37 | Residual bridge approvals let approved spenders drain future USDC entering JackpotBridgeManager | Keep | High | supported-behavior | Uses ordinary USDC allowance persistence, not unsupported token behavior. |
| H-38 | Opaque bridge calldata can pass balance checks while sending winnings to an attacker | Keep | High | supported-behavior | Uses documented USDC transfers, not unsupported token behavior. |
| H-39 | Arbitrary bridge call can transfer other users' custodied ticket NFTs from JackpotBridgeManager | Keep | High | supported-behavior | Uses ordinary USDC allowance transfer behavior, not unsupported token behavior. |
| M-40 | Invalid no-replacement requests are accepted then permanently fail during entropy fulfillment | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-41 | ECDSA-only bridge signatures permanently lock tickets and winnings owned by smart wallets | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-42 | Stale bridge claim signatures can be replayed indefinitely until the ticket is consumed | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-43 | No-referral win share is credited to the current drawing, letting later LPs capture prior drawing value | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-44 | Ticket purchases remain open after drawingTime until runJackpot is mined | Keep | High | supported-behavior | Uses documented USDC ticket payments, not unsupported token behavior. |
| M-45 | LPs can frontrun cap reductions by depositing before setGovernancePoolCap reverts | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-46 | Unsnapshotted drawing payouts can be calculated as zero in GuaranteedMinimumPayoutCalculator.calculateAndStoreDrawingUserWinnings | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-47 | No-referral winning share is credited to the active drawing, letting winners redirect prior LP value to future LPs | Keep | High | supported-behavior | No unsupported token semantics involved. |
| H-48 | Bonusball bit overflow corrupts tier calculation and can brick valid Jackpot.claimWinnings payouts | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-49 | runJackpot requires an unrewarded caller to pay entropy fees for protocol liveness | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-50 | Unpaid jackpot execution leaves expired drawings open for late ticket buyers | Keep | High | supported-behavior | Uses documented USDC ticket payments, not unsupported token behavior. |
| M-52 | Unbounded request ranges and batch sizes can make entropy callbacks run out of gas | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-53 | Reverting requester callback rolls back pending cleanup and leaves consumed entropy requests stale | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-54 | Bridge claim signatures have no nonce or deadline, allowing leaked or old authorizations to be executed later | Keep | High | supported-behavior | No unsupported token semantics involved. |
| L-55 | claimWinnings leaves burned tickets recorded as owned in bridge accounting | Keep | High | supported-behavior | No unsupported token semantics involved. |
| H-56 | Arbitrary bridge call lets a winning claimant transfer other users' custodied ticket NFTs | Keep | High | supported-behavior | Uses ordinary USDC allowance transfer behavior, not unsupported token behavior. |
| M-57 | Opaque bridge calldata can steal all claimed winnings because claimWinnings does not bind amount or recipient | Keep | High | supported-behavior | Uses documented USDC transfers, not unsupported token behavior. |
| H-58 | Signed bridge route can approve one address while arbitrary call drains the claimed USDC through another address | Keep | High | supported-behavior | Uses ordinary USDC allowance transfer behavior, not unsupported token behavior. |
| M-59 | Arbitrary bridge call can leave reusable USDC allowance and steal pre-existing bridge surplus | Keep | High | supported-behavior | Uses ordinary USDC allowance persistence, not unsupported token behavior. |
| H-60 | ScaledEntropyProvider reuses one entropy seed for all random sets, correlating normal balls and bonusball | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-61 | Large no-replacement ranges can force fulfillment-time memory exhaustion | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-62 | Unbounded request arrays can make entropy fulfillment run out of gas and leave drawings stuck | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-63 | Invalid no-replacement entropy requests are accepted then permanently revert during fulfillment | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-64 | Full uint256 ranges are accepted but overflow during scaled randomness fulfillment | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-65 | Missing request bounds allow entropy requests that pass validation but revert or exhaust gas during fulfillment | Keep | High | supported-behavior | No unsupported token semantics involved. |
| L-66 | Burned JackpotTicketNFT tokens keep active-looking ticket metadata in public getters | Keep | High | supported-behavior | No unsupported token semantics involved. |
| L-67 | Unbounded per-user ticket enumeration can make JackpotTicketNFT.getUserTickets unusable for large holders | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-69 | Bonusball ranges above bit 255 corrupt packed tickets and can brick or mis-tier claims | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-70 | Entropy provider migration can overwrite pending requests with colliding sequence numbers | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-71 | Provider sequence collisions can overwrite pending entropy requests after provider rotation | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-72 | Bonusball bit positions at or above 256 corrupt ticket packing and bonusball match accounting | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-73 | Entropy provider rotation can overwrite pending randomness requests with colliding sequence numbers | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-74 | Bonusball bounds can exceed uint256 bit domain and corrupt ticket packing | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-76 | Valid high bonusball values overflow ticket bitpacking and can lock settlement | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-77 | Provider rotation can collide Pyth sequence IDs and overwrite pending entropy requests | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-78 | Entropy provider rotation can overwrite pending randomness requests with colliding sequence numbers | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-79 | Entropy pending requests are keyed only by sequence and collide across provider rotations | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-80 | Invalid normalBallMax plus bonusballMax configuration causes ticket bit collisions and false jackpot-tier claims | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-81 | Bonusball bit positions at 256 or higher corrupt packed tickets and let non-winning tickets claim as bonus matches | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-82 | Bonusball bit positions above 255 alias distinct tickets and can create false winning claims | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-83 | Bonusball ranges above the bit-packing boundary can make entropy settlement revert and lock the drawing | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-84 | Changing entropy during a locked drawing permanently rejects the pending callback | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-85 | Entropy requests from different providers collide because pending requests are keyed only by sequence | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-86 | Rotating payoutCalculator after settlement can zero or change unclaimed winning tickets | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-87 | Entropy provider rotation lets per-provider sequence collisions overwrite pending jackpot randomness | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-88 | Payout calculator rotation can zero or alter unclaimed winnings from already-settled drawings | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-90 | Small normalBallMax values can permanently DoS settlement through impossible-tier combination asserts | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-91 | Payout calculator rotation reprices already initialized or settled ticket claims | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-92 | Allowed normalBallMax plus bonusballMin can exceed the 255-bit ticket packing domain | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-93 | Provider sequence collision can overwrite pending jackpot randomness and leave a drawing permanently locked | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-94 | Payout calculator rotation reprices already settled unclaimed tickets | Keep | High | supported-behavior | No unsupported token semantics involved. |
| L-95 | Allowed normalBallMax values below 10 can brick payout calculation during settlement | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-96 | Rotating payoutCalculator makes old winning tickets read payouts from the wrong calculator | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-97 | Entropy address rotation during a pending request permanently rejects the valid callback | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-98 | Changing Jackpot.entropy after runJackpot makes the in-flight entropy callback unauthorized and bricks settlement | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-100 | Provider sequence collisions can replay entropy into the wrong request after entropy provider rotation | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-101 | Updating the payout calculator breaks already-settled ticket claims by reading payouts from the new calculator | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-102 | Provider-scoped entropy sequence numbers collide in ScaledEntropyProvider and can strand Jackpot drawings | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-104 | LP deposits can frontrun pool-cap reductions and block risk-reducing governance updates | Keep | High | supported-behavior | No unsupported token semantics involved. |
| M-105 | LPs can frontrun pool-cap reductions by depositing before JackpotLPManager.setLPPoolCap executes | Keep | High | supported-behavior | No unsupported token semantics involved. |
