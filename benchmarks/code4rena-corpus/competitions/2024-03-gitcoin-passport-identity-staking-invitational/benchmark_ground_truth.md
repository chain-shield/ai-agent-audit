# Benchmark Ground Truth: Gitcoin Passport - Identity Staking Invitational

## Accepted H/M Findings

# Accepted H/M Findings: Gitcoin Passport - Identity Staking Invitational

# [H-01] userTotalStaked invariant will be broken due to vulnerable implementations in release()

- **Contest:** Gitcoin Passport - Identity Staking Invitational
- **Slug:** 2024-03-gitcoin-passport-identity-staking-invitational
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-03-gitcoin-passport-identity-staking-invitational
- **Source snapshot:** competitions/2024-03-gitcoin-passport-identity-staking-invitational/final_report.html

userTotalStaked invariant will be broken due to vulnerable implementations in release() Submitted by oakcobalt, also found by oakcobalt, Stormy, and 0xDING99YA userTotalStaked invariant will be broken due to vulnerable implementations in release(). Users might lose funds due to underflow errors in withdraw methods.

## Recommended Mitigation Steps

In release(), also update userTotalStaked.

nutrina (Gitcoin) confirmed Alex the Entreprenerd (judge) commented:

The warden has shown how, due to incorrect accounting, userTotalStaked may end up being less than intended, causing a loss of funds to users.

Due to the impact, I agree with High Severity.

Gitcoin mitigated:

This PR fixes the userTotalStaked invariant (accounting error) here.

Status:

Mitigation confirmed. Full details in reports from oakcobalt, Stormy and 0xDING99YA.

## Rejected Primary Findings

# Rejected Primary Findings: Gitcoin Passport - Identity Staking Invitational

No rejected primary findings were captured for this contest in the current corpus.

- **Rejected primary count:** 0
- **Primary submission rows captured:** 0
- **Submission status:** requires_authentication
