# Accepted H/M Findings: Ethena Labs Invitational

# [M-01] Blacklisted user can burn tokens during WHITELIST_ENABLED state

- **Contest:** Ethena Labs Invitational
- **Slug:** 2024-11-ethena-labs-invitational
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-ethena-labs-invitational
- **Source snapshot:** competitions/2024-11-ethena-labs-invitational/final_report.html

Submitted by MrPotatoMagic, also found by SpicyMeatball Blacklisted user can burn tokens during WHITELIST_ENABLED state. This breaks the main invariant from the README. This could become an issue when the admin tries to redistribute the blacklisted user’s UStb balance using redistributeLockedAmount() but the blacklisted user frontruns it with a burn.

## Recommended Mitigation Steps

Add the conditions !hasRole(BLACKLISTED_ROLE, msg.sender) and !hasRole(BLACKLISTED_ROLE, from) to the check.

iethena (Ethena Labs) disputed via duplicate issue #3 and commented:

This can happen in 2 different variants:

A user is initially blacklisted but is later added to the whitelist without being removed from the blacklist A user is initially whitelisted but is later added to the blacklist without being removed from the whitelist In both cases the user can burn their tokens when whitelist mode is enabled for the transfer state. The likelihood of falling into this state is high because the Blacklist Manager and Whitelist Manager are intended to be different entities and as such we cannot guarantee that they will coordinate to maintain a clean separation of the blacklist/whitelist roles. If a user with a whitelist and blacklist role simultaneously, chose to burn their tokens in whitelist transfer state mode, it would have a positive impact on the protocol overall as there would be excess collateral in the protocol. We therefore believe that this issue should be marked as low as there is no incentive for a user to outright burn their tokens, in fact if a user does so, all other users in the protocol will benefit, without causing a negative impact.

Just to note that this was fixed in ethena-labs/ethena-ustb-contest/pull/2 based off of a different qa finding which ensures that whitelist/blacklist roles are mutually exclusive.

# [M-02] Non-whitelisted users can burn UStb and redeem collateral during WHITELIST_ENABLED state

- **Contest:** Ethena Labs Invitational
- **Slug:** 2024-11-ethena-labs-invitational
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-ethena-labs-invitational
- **Source snapshot:** competitions/2024-11-ethena-labs-invitational/final_report.html

Submitted by MrPotatoMagic Non-whitelisted users can redeem collateral tokens and burn their UStb even when whitelist mode has been enabled on UStb contract. This breaks the main invariant mentioned in the README.

## Recommended Mitigation Steps

Add hasRole(WHITELISTED_ROLE, from) in the check.

iethena (Ethena Labs) disputed and commented:

The Redeemer has a special role in the protocol and it is seen as a non-issue that UStb can be redeemed from a non-whitelisted address while whitelist mode is enabled. As specified in the Overview the redeem order is determined by and off-chain RFQ system, in which case the Redeemer will not provide a redemption quote if they chose not to.

For clarity, a non-whitelisted user cannot redeem collateral without the involvement of the Redeemer, as it is the Redeemer who submits the settlement transaction on-chain. We therefore argue that the likelihood of this happening is low. In addition, the impact to the protocol is low as the collateralization ratio of the minting contract would not be impacted and other users’ positions are not impacted. Although this finding is informationally correct, we view the severity to be based on the fact that a non-whitelisted address can initiate a redemption without the involvement of a trusted party within the protocol which is not the case.

We suggest marking this finding as Low severity.

EV_om (judge) commented:

One of the main invariants here was:

Only whitelisted user can send/receive/burn UStb tokens in a WHITELIST_ENABLED transfer state.

The warden could not have known whether the off-chain Redeemer would only submit transactions for whitelisted addresses. Besides, the WHITELIST_MANAGER and the REDEEMER being different entities means there can always be race conditions on address (un-)whitelisting and redemption submission.

If the contract must enforce the invariant (which seems to not necessarily be the case), this must be done onchain. But for the purpose of the audit, Medium is appropriate.
