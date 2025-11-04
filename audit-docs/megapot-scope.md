
### Arithmetic Rounding

Accumulator and payout math truncates in favor of solvency; small “dust” is expected and acceptable.

### Emergency

Emergency refunds don’t claw back past referral fees; this is a policy decision. Refunded users with referrals receive ticketPrice minus referral portion.

The emergency mode is not intended to be recoverable - it is meant to be enabled in the case where the Jackpot gets stuck and can no longer progress.

### Referral Fallback

If no referral scheme is passed for a ticket then referral fees accrue to the LP pool.

# Overview

MegaPot V2 is a decentralized jackpot protocol where users purchase NFT-based jackpot tickets and liquidity providers fund prize pools. The system uses Pyth Network entropy for provably fair drawings, automatically distributes winnings based on number matches, and includes cross-chain bridge functionality.

# Scope

### Files out of scope

| File         |
| ------------ |
| [contracts/mocks/\*\*.\*\*](https://github.com/code-423n4/2025-11-megapot/tree/main/contracts/mocks) |
| Totals: 11 |

# Additional context

## Areas of concern (where to focus for bugs)
1) Is there any way to drain funds in the jackpot via LP or referrer deposit/withdraw flows?
2) Is there any way to drain the jackpot by falsifying tickets or gaming randomness?
3) Making sure the jackpot is truly fair and cannot be exploited (ie randomness is being correctly used and creating truly random outputs)
4) Is there any way that the jackpot could end up being -EV for LPs? Can we guarantee a minimum amount of edge?
5) Is there any way the jackpot could end up under-collateralized via accounting errors? Either business logic (ie. not accounting for all ticket winners, miscounting winners, misidentifying winners on claim) or rounding errors (especially accrued over time, rounding should be conservative with respect to collateralization)
6) Is there any way that LPs, referrers, or users could not be paid out what they're owed due to faulty state tracking or math? (ie not all ticket winners accounted for)
7) Is there any way to lock funds in the jackpot for any user - winners, referrers, LPs?
8) Is there any way that the jackpot could potentially get stuck and be unable to progress to the next drawing?
9) Can EIP-712 signatures be exploited as part of the bridging manager to either "steal" someones tickets or otherwise interfere with accounting? Either from signature replays or attempting hash collision.
10) is the case where the guaranteed payouts + premium tier minimum exceed the total value of the prize pool adequately handled?
11) Is all the bitpacking logic sound? Are there any potential boundary errors that could arise either between the lower bits where the normals are or the higher bits where powerball must be less than 255 - normalBall Max?
12) Can admin changes (e.g., ticketPrice, normalBallMax, fees) made mid-drawing create inconsistent states or violate expectations for players/LPs? Can global state changes affect the outcome of prior drawings (ie using a global param in calculations concerning a prior drawing)?

## Main invariants

### Payouts & Solvency

  - Total user payouts per drawing ≤ prizePool (never over-
  allocates).
  - Minimum payouts gating: if (minimumPayoutAllocation +
  premiumFloor) > prizePool, guaranteed minimums are skipped;
  otherwise minimums are paid and the remainder is allocated by
  weights.
  - Premium tier weights sum to 1e18; all payouts denominated in USDC (6
  decimals) with truncation favoring solvency.
  - Duplicate winners are proportionally reduce the premium payouts within a tier
  - Sum of all LP deposits (active and inactive), claimed winnings, and current drawing ticket    purchases must be less than total USDC balance of the contract (we expect some rounding that leaves dust unaccounted for)

### Tickets & Tiering

  - Normals are length 5, unique, in [1..normalBallMax]; bonusball
  in [1..bonusMax].
  - Bitpacking: normals occupy bits [1..normalBallMax]; bonusball at
  (normalBallMax + bonusball).
  - Tier formula: tierId = 2*(matchedNormals) + (bonusballMatch ?
  1 : 0), range [0..11].
  - Tickets are single-claim: burn before payout; no double-claim
  possible.

### LP & Accumulator

  - Accumulator strictly > 0 for all settled drawings.
  - Share/USDC conversions use the correct historical accumulator
  for the drawing of deposit/withdrawal.
  - Pool cap respect: deposits cannot exceed (lpPoolTotal +
  pendingDeposits); cannot set cap below current total + pending
  deposits.
  - Pending deposits are valued in USDC (this round); pending
  withdrawals are shares (finalized at settlement).

### Referrals & Fees

  - Referral splits sum exactly to PRECISE_UNIT; zero addresses and
  zero splits rejected.
  - Referrer share on winnings is deducted from user payout; if no
  scheme, referrer share is added to LP earnings.
  - Protocol fee applies only when (lpEarnings − userWinnings) >
  threshold; fee = (excess − threshold) * rate.

### Entropy & Draw Lifecycle

  - runJackpot only after drawingTime and when unlocked; locks
  drawing before request.
  - Entropy callback only from the configured provider and only when
  locked; completes settlement and initializes next drawing.
  - Entropy fee uses base + variable gas per powerball;
  getEntropyCallbackFee matches fee input to runJackpot.

### Access Control & Safety

  - Critical paths are nonReentrant; token transfers use SafeERC20.
  - Emergency mode gates refunds and emergency LP withdrawals;
  normal operations disabled while active.
  - Admin parameter changes apply to future drawings (not mid-
  drawing retroactive changes).

## All trusted roles in the protocol

| Role                                | Description                       |
| --------------------------------------- | ---------------------------- |
| Owner                          | Can update various jackpot settings                |
