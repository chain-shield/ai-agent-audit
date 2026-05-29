# 2025-11-megapot Three-Shot Stage 2 Unsupported-Token Screen

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

## Decisions

| Finding | Finding Title | Decision | Confidence | Reason Category | Reason |
| --- | --- | --- | --- | --- | --- |
| H-1 / emCZQy_NnRkOsuezTEB29 | Arbitrary bridge call leaves reusable USDC allowance that can drain later bridge-manager funds | Keep | High | supported-behavior | Uses ordinary USDC allowance persistence, not unsupported token behavior. |
| M-2 / rQOhmiuSK6JH_yE1hbzbl | ECDSA-only bridge claims permanently lock tickets owned by ERC-1271 smart wallets | Keep | High | supported-behavior | Smart-wallet signature handling does not depend on token semantics. |
| M-3 / ghJ1xAP05g3yFJvWi6lh9 | Entropy callback ignores the request id, allowing a stale request to settle a later locked drawing | Keep | High | supported-behavior | Entropy request binding issue has no ERC20 dependency. |
| M-4 / ZwhrSB7704IKw_ypW0qEm | Bridge claim signatures never expire and can be executed long after the user signed them | Keep | High | supported-behavior | Signature expiry issue does not rely on unsupported token behavior. |
| M-5 / WoebK6L4bf9CMQmO1z0JH | Changing entropy provider while a drawing is pending permanently blocks settlement | Keep | High | supported-behavior | Entropy provider rotation issue has no token semantics dependency. |
| M-7 / b_aiHuotWEfaTycI8agH9 | Changing payoutCalculator mid-drawing can settle active tickets with an unsnapshotted calculator | Keep | High | supported-behavior | Payout calculator snapshot issue does not rely on token behavior. |
| M-8 / -QwYrwqDqTJ71r1NZS63J | Changing Jackpot entropy address while a draw is pending bricks the authorized callback | Keep | High | supported-behavior | Entropy authorization issue has no ERC20 dependency. |
| M-9 / 4fPnoqYOKR9JwLwxDlbeJ | Bonusballs above bit capacity are erased, allowing uncounted tickets to claim winning bonusball payouts | Keep | High | supported-behavior | Ticket bitpacking issue does not rely on unsupported assets. |
| M-10 / VcshH0W6i-ZnZDjOPbtST | No-referral winner share is credited to the current drawing instead of the settled drawing | Keep | High | supported-behavior | Accounting target issue uses normal USDC-denominated protocol accounting. |
| M-11 / PPrrZKotDc61i9N4tnTCl | Changing entropy while a drawing is pending bricks the original callback and leaves the jackpot locked | Keep | High | supported-behavior | Entropy rotation issue has no token semantics dependency. |
| H-12 / kjDblGFEXt_ejozdhDT3K | Arbitrary bridge call lets a winning ticket holder steal other users' custodied ticket NFTs | Keep | High | supported-behavior | NFT custody and ordinary USDC allowance use do not require exotic tokens. |
| M-13 / twUQAbk8uX7jwTke8KIfR | Unfulfillable SetRequests are accepted and later revert during entropy fulfillment | Keep | High | supported-behavior | Randomness request validation issue has no ERC20 dependency. |
| M-14 / RJM_fI7cXw-dK2wVfx0Xs | Mid-drawing payout calculator update causes active drawing to settle with zero stored payouts | Keep | High | supported-behavior | Payout calculator issue does not rely on token semantics. |
| M-15 / kieHSGUnSmvztUc_OBnAx | Missing bonusball packing bound corrupts tickets when normalBallMax + bonusballMax reaches 256 | Keep | High | supported-behavior | Ticket packing issue has no unsupported-token dependency. |
| M-16 / v2sgzi0QYHrqs1cThqt8a | Bridge claim signatures have no nonce or deadline and remain executable indefinitely | Keep | High | supported-behavior | Signature freshness issue does not rely on ERC20 behavior. |
| M-17 / EjJJ7I_tm_zeL_HoMTLjP | ECDSA-only bridge authorization permanently locks tickets owned by ERC-1271 smart wallets | Keep | High | supported-behavior | Smart-wallet signature handling does not depend on token semantics. |
| M-18 / oZQ2pM2T1ltePA70cdJcZ | Changing entropy providers can overwrite pending randomness requests with colliding sequence IDs | Keep | High | supported-behavior | Entropy sequence collision issue has no token dependency. |
| M-20 / p2_DN-XODOvXu_tNksgej | Bridge ticket purchases use live global ticketPrice instead of the active drawing price, causing overcharges or purchase DoS | Keep | High | supported-behavior | Uses standard USDC payment accounting, not unsupported token behavior. |
| M-21 / PMFE9hYicCVTJCz09VHk6 | Perpetual EIP-712 bridge signatures can be executed after user intent expires | Keep | High | supported-behavior | Signature expiry issue has no unsupported-token dependency. |
| M-22 / OYZfVCU_Jw3zjvZAlSLNS | Nonce-less ticket withdrawal signatures stay valid indefinitely and can steal later winning tickets | Keep | High | supported-behavior | Ticket authorization issue does not rely on token semantics. |
| M-23 / XboZ6rstW75a7OfO7LUPg | Pending LP deposits can be rounded to zero after accumulator inflation | Keep | High | supported-behavior | Share rounding issue uses normal USDC-denominated accounting. |
| M-24 / _uzGLqresOu7Dk6U-KMjO | No-referral winner shares are credited to the claim-time drawing instead of the settled drawing | Keep | High | supported-behavior | Accounting bucket issue does not require unsupported token behavior. |
| M-25 / -HhMgy6frh49i966ZDULK | Full uint256 range requests are accepted but overflow during randomness scaling | Keep | High | supported-behavior | Randomness range issue has no token dependency. |
| H-26 / Zy2Cwvh0tHGOtbihykmbO | Pending LP deposits can be zeroed by accumulator inflation before settlement | Keep | High | supported-behavior | Deposit share issue uses normal USDC-denominated accounting. |
| M-27 / rbb-sLW7w5f9_y-bT86sa | Tier zero winners are excluded from settlement obligations but can still claim stored tier-zero payouts | Keep | High | supported-behavior | Payout counting issue does not rely on unsupported assets. |
| M-28 / 4J7yK8OUsFrvGAhCtYqjh | Full uint256 range requests overflow during scaling after being accepted | Keep | High | supported-behavior | Randomness range issue has no ERC20 dependency. |
| M-29 / qYfnKos-ZnhIgN86d2poe | Bridge claim signatures have no nonce or deadline, allowing stale execution through obsolete bridge routes | Keep | High | supported-behavior | Signature freshness issue does not depend on token semantics. |
| M-30 / l5OqMxsLjuLw9XhbtQprM | Bridge ticket and winnings signatures never expire and can be executed long after user intent changes | Keep | High | supported-behavior | Signature expiry issue has no unsupported-token dependency. |
| M-32 / Nlj-UYTyD4z1UkRYuxpyz | Invalid full-range randomness requests are accepted and later revert during entropy fulfillment | Keep | High | supported-behavior | Randomness validation issue has no token dependency. |
| M-33 / oGmvAj73gSj07W0Jh5QNn | Impossible no-replacement draws are accepted and can fail only after entropy is consumed | Keep | High | supported-behavior | Randomness validation issue has no ERC20 dependency. |
| M-34 / DTCE_kPYivgorgy9AIkkR | Missing feasibility validation lets callers create entropy requests that can never be fulfilled | Keep | High | supported-behavior | Entropy request feasibility issue has no token dependency. |
| M-35 / et7UofFB1msp9Vt7VVQ2v | Stale entropy callbacks can settle the wrong Jackpot drawing because request IDs are not bound to drawing IDs | Keep | High | supported-behavior | Entropy callback binding issue does not rely on token behavior. |
| M-36 / N8k9IZrZQviCeyyaNHOQ4 | Changing entropy provider while a request is pending permanently rejects the valid callback and locks the drawing | Keep | High | supported-behavior | Entropy provider rotation issue has no token semantics dependency. |
| M-37 / dFlfXpkDW49CviZDQWS2A | Residual bridge approvals let approved spenders drain future USDC entering JackpotBridgeManager | Keep | High | supported-behavior | Uses ordinary USDC approval persistence, not unsupported ERC20 quirks. |
| H-38 / o3d2w1GwCZw3kuQKHepDU | Opaque bridge calldata can pass balance checks while sending winnings to an attacker | Keep | High | supported-behavior | Uses standard USDC transfer and balance semantics. |
| H-39 / yx3YULUi7UPyOA6c8ez2L | Arbitrary bridge call can transfer other users' custodied ticket NFTs from JackpotBridgeManager | Keep | High | supported-behavior | NFT custody issue with ordinary USDC checks only. |
| M-40 / JjYFCeoUYvcJcIX6_pK_0 | Invalid no-replacement requests are accepted then permanently fail during entropy fulfillment | Keep | High | supported-behavior | Entropy request validation issue has no token dependency. |
| M-41 / JmNO-knM1BokEuNBDWxTv | ECDSA-only bridge signatures permanently lock tickets and winnings owned by smart wallets | Keep | High | supported-behavior | Smart-wallet signature issue does not rely on token semantics. |
| M-42 / bEM7bCscGCIhiKqM40GUJ | Stale bridge claim signatures can be replayed indefinitely until the ticket is consumed | Keep | High | supported-behavior | Signature replay issue has no unsupported-token dependency. |
| M-43 / anKPmMCB1WwZFh-pgeeD3 | No-referral win share is credited to the current drawing, letting later LPs capture prior drawing value | Keep | High | supported-behavior | Accounting bucket issue uses supported USDC value units. |
| M-44 / CJfADIczTrhQXGEfG6myl | Ticket purchases remain open after drawingTime until runJackpot is mined | Keep | High | supported-behavior | Drawing timing issue does not rely on token semantics. |
| M-46 / 3nNN8Mv0tqj6acsGySPOG | Unsnapshotted drawing payouts can be calculated as zero in GuaranteedMinimumPayoutCalculator.calculateAndStoreDrawingUserWinnings | Keep | High | supported-behavior | Payout snapshot issue has no unsupported-token dependency. |
| M-47 / DY8_8XaFt5YlWy8JdsHMf | No-referral winning share is credited to the active drawing, letting winners redirect prior LP value to future LPs | Keep | High | supported-behavior | Referral accounting issue uses normal USDC-denominated accounting. |
| H-48 / x-waZ76GXiDz3-pI_Wxg2 | Bonusball bit overflow corrupts tier calculation and can brick valid Jackpot.claimWinnings payouts | Keep | High | supported-behavior | Ticket tiering issue does not rely on unsupported token behavior. |
| M-49 / uvZ6f7Zqp3uFwf09w21c3 | runJackpot requires an unrewarded caller to pay entropy fees for protocol liveness | Keep | High | supported-behavior | Entropy fee liveness issue has no ERC20 dependency. |
| M-50 / 4BnVTywm1gum72Soijtty | Unpaid jackpot execution leaves expired drawings open for late ticket buyers | Keep | High | supported-behavior | Drawing execution timing issue does not rely on token semantics. |
| M-52 / ZSK-vXUjmoLQ2ZqQGSEpm | Unbounded request ranges and batch sizes can make entropy callbacks run out of gas | Keep | High | supported-behavior | Entropy gas issue has no token dependency. |
| M-53 / TkptkGk_kcgTRVqNNHeX8 | Reverting requester callback rolls back pending cleanup and leaves consumed entropy requests stale | Keep | High | supported-behavior | Callback cleanup issue has no ERC20 dependency. |
| M-54 / cn7PMWjyPYAdapY1lFl5n | Bridge claim signatures have no nonce or deadline, allowing leaked or old authorizations to be executed later | Keep | High | supported-behavior | Signature freshness issue has no unsupported-token dependency. |
| L-55 / JGgR2xyH0uL0CDYaPTq9A | claimWinnings leaves burned tickets recorded as owned in bridge accounting | Keep | High | supported-behavior | Bridge accounting issue does not rely on token semantics. |
| H-56 / ScHOGndBOIJkw9PUyhBrl | Arbitrary bridge call lets a winning claimant transfer other users' custodied ticket NFTs | Keep | High | supported-behavior | NFT custody issue with ordinary USDC balance checks only. |
| M-57 / xY6OOMdx8rewLd4XafLh_ | Opaque bridge calldata can steal all claimed winnings because claimWinnings does not bind amount or recipient | Keep | High | supported-behavior | Uses standard USDC allowance and transfer behavior. |
| H-58 / I7FqeqgXJk0H66gi0fIkj | Signed bridge route can approve one address while arbitrary call drains the claimed USDC through another address | Keep | High | supported-behavior | Uses ordinary USDC transfer and allowance semantics. |
| M-59 / AmvuAyw8y2V0rkSpDDxav | Arbitrary bridge call can leave reusable USDC allowance and steal pre-existing bridge surplus | Keep | High | supported-behavior | Uses standard USDC approval persistence, not zero-reset quirks. |
| H-60 / DCLCnC_dWz7GX-y5h94Yu | ScaledEntropyProvider reuses one entropy seed for all random sets, correlating normal balls and bonusball | Keep | High | supported-behavior | Randomness correlation issue has no token dependency. |
| M-61 / ca2NbWJQODDuT28JrEfvS | Large no-replacement ranges can force fulfillment-time memory exhaustion | Keep | High | supported-behavior | Entropy memory issue has no ERC20 dependency. |
| M-62 / 62WtuEiLCFmhhhAkFFXso | Unbounded request arrays can make entropy fulfillment run out of gas and leave drawings stuck | Keep | High | supported-behavior | Entropy gas issue has no token semantics dependency. |
| M-63 / VYM7kgJV8rdR7r9C3Ouaf | Invalid no-replacement entropy requests are accepted then permanently revert during fulfillment | Keep | High | supported-behavior | Entropy validation issue has no token dependency. |
| M-64 / qTFiUtYrdGZ5IC_nE_NzD | Full uint256 ranges are accepted but overflow during scaled randomness fulfillment | Keep | High | supported-behavior | Randomness range issue has no ERC20 dependency. |
| M-65 / mrBG0kQc_1aaDkHc5rWc- | Missing request bounds allow entropy requests that pass validation but revert or exhaust gas during fulfillment | Keep | High | supported-behavior | Entropy validation issue has no token dependency. |
| L-67 / NmeIslHIena89r8c9Tp8O | Unbounded per-user ticket enumeration can make JackpotTicketNFT.getUserTickets unusable for large holders | Keep | High | supported-behavior | NFT enumeration issue does not rely on token semantics. |
| M-69 / dTJQMBRcGvOjORilkVUlf | Bonusball ranges above bit 255 corrupt packed tickets and can brick or mis-tier claims | Keep | High | supported-behavior | Ticket bitpacking issue has no unsupported-token dependency. |
| M-70 / JNirY2A5L7N-4sAPEQSSD | Entropy provider migration can overwrite pending requests with colliding sequence numbers | Keep | High | supported-behavior | Entropy sequence issue has no token dependency. |
| M-71 / qQy9NQ79-RRyMstIQufsw | Provider sequence collisions can overwrite pending entropy requests after provider rotation | Keep | High | supported-behavior | Entropy sequence issue has no token semantics dependency. |
| M-72 / J1Dqv-7r4rRmkAq_ocKmr | Bonusball bit positions at or above 256 corrupt ticket packing and bonusball match accounting | Keep | High | supported-behavior | Ticket packing issue does not rely on unsupported assets. |
| M-73 / FgRFBZujzUDHT5TlbsOr6 | Entropy provider rotation can overwrite pending randomness requests with colliding sequence numbers | Keep | High | supported-behavior | Entropy provider issue has no token dependency. |
| M-74 / HCZnk6xd_GnFZCH0X04fe | Bonusball bounds can exceed uint256 bit domain and corrupt ticket packing | Keep | High | supported-behavior | Ticket bitpacking issue has no unsupported-token dependency. |
| M-76 / fVsK-LAQ7IPQ6NH8i6z49 | Valid high bonusball values overflow ticket bitpacking and can lock settlement | Keep | High | supported-behavior | Ticket packing issue has no token dependency. |
| M-77 / YrzefVI2gaRcji1F4O2lj | Provider rotation can collide Pyth sequence IDs and overwrite pending entropy requests | Keep | High | supported-behavior | Entropy sequence issue has no token semantics dependency. |
| M-78 / uxV4GUomOSDggeWGNWLva | Entropy provider rotation can overwrite pending randomness requests with colliding sequence numbers | Keep | High | supported-behavior | Entropy provider issue has no token dependency. |
| M-79 / Eo7DO9DiVWIGLlFfnfy0D | Entropy pending requests are keyed only by sequence and collide across provider rotations | Keep | High | supported-behavior | Entropy request keying issue has no ERC20 dependency. |
| M-80 / epV1g2oCGbgjEd1MKVJMs | Invalid normalBallMax plus bonusballMax configuration causes ticket bit collisions and false jackpot-tier claims | Keep | High | supported-behavior | Ticket configuration issue does not rely on token semantics. |
| M-81 / 8enSiBNKwJR94z-IbBs8s | Bonusball bit positions at 256 or higher corrupt packed tickets and let non-winning tickets claim as bonus matches | Keep | High | supported-behavior | Ticket bitpacking issue has no unsupported-token dependency. |
| M-82 / c1LUgH768vdleCQv2vutO | Bonusball bit positions above 255 alias distinct tickets and can create false winning claims | Keep | High | supported-behavior | Ticket packing issue does not rely on unsupported assets. |
| M-83 / eU8MFp1NT2zpXY6vsm4RZ | Bonusball ranges above the bit-packing boundary can make entropy settlement revert and lock the drawing | Keep | High | supported-behavior | Ticket packing issue has no token dependency. |
| M-84 / up_Ag7R-61tUyZTohHm-S | Changing entropy during a locked drawing permanently rejects the pending callback | Keep | High | supported-behavior | Entropy authorization issue has no token semantics dependency. |
| M-85 / z6UFaJb3e9UR2WAFVsTsW | Entropy requests from different providers collide because pending requests are keyed only by sequence | Keep | High | supported-behavior | Entropy request keying issue has no ERC20 dependency. |
| M-86 / VumA1LcRqul120TjtxTRb | Rotating payoutCalculator after settlement can zero or change unclaimed winning tickets | Keep | High | supported-behavior | Payout calculator issue does not rely on token semantics. |
| M-87 / dUualzZdMcwOLTWqMmzY9 | Entropy provider rotation lets per-provider sequence collisions overwrite pending jackpot randomness | Keep | High | supported-behavior | Entropy sequence issue has no token dependency. |
| M-88 / SIZoP2wmWR2-DqOorjm21 | Payout calculator rotation can zero or alter unclaimed winnings from already-settled drawings | Keep | High | supported-behavior | Payout calculator issue has no unsupported-token dependency. |
| M-91 / GPliAtb0Dot7OjQ4XXxl1 | Payout calculator rotation reprices already initialized or settled ticket claims | Keep | High | supported-behavior | Payout calculator issue does not rely on token semantics. |
| M-92 / MZxhqjaBRZw38hY7UoPkd | Allowed normalBallMax plus bonusballMin can exceed the 255-bit ticket packing domain | Keep | High | supported-behavior | Ticket packing issue has no unsupported-token dependency. |
| M-93 / PR2G588QO-Kwr3c5Fhe4w | Provider sequence collision can overwrite pending jackpot randomness and leave a drawing permanently locked | Keep | High | supported-behavior | Entropy request keying issue has no token dependency. |
| M-94 / hwZlFmHk1XCY6j0kVxNjK | Payout calculator rotation reprices already settled unclaimed tickets | Keep | High | supported-behavior | Payout calculator issue does not rely on token semantics. |
| M-96 / 4N5SxmjaZ5ZFkldy20qul | Rotating payoutCalculator makes old winning tickets read payouts from the wrong calculator | Keep | High | supported-behavior | Payout calculator issue has no unsupported-token dependency. |
| M-97 / c1taCy-rUpLHmD9KeiD5V | Entropy address rotation during a pending request permanently rejects the valid callback | Keep | High | supported-behavior | Entropy authorization issue has no token dependency. |
| M-98 / bqDE4tP4lSLVtXr7da9Tk | Changing Jackpot.entropy after runJackpot makes the in-flight entropy callback unauthorized and bricks settlement | Keep | High | supported-behavior | Entropy authorization issue has no token semantics dependency. |
| M-100 / pxtxFK-1LCZ2MRtDy6Fgn | Provider sequence collisions can replay entropy into the wrong request after entropy provider rotation | Keep | High | supported-behavior | Entropy sequence issue has no token dependency. |
| M-101 / OyeL2vMtI2i2rZz06wHLH | Updating the payout calculator breaks already-settled ticket claims by reading payouts from the new calculator | Keep | High | supported-behavior | Payout calculator issue does not rely on token semantics. |
| M-102 / Qet8mUOMbbhvJ91BSgZAI | Provider-scoped entropy sequence numbers collide in ScaledEntropyProvider and can strand Jackpot drawings | Keep | High | supported-behavior | Entropy request keying issue has no ERC20 dependency. |
