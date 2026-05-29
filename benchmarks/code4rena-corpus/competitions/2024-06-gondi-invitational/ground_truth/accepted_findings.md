# Accepted H/M Findings: Gondi Invitational

# [M-01] Delegations cannot be removed in some cases due to vulnerable revokeDelegate() implementation

- **Contest:** Gondi Invitational
- **Slug:** 2024-06-gondi-invitational
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-06-gondi-invitational
- **Source snapshot:** competitions/2024-06-gondi-invitational/final_report.html

revokeDelegate() implementation Submitted by oakcobalt An old borrower can use an old delegation to claim on behalf of a new borrower.

## Recommended Mitigation Steps

In revokeDelegate(), allow passing bytes32 _rights into delegateERC721() to correctly revoke existing delegations with custom rights.

## Assessed type

Error 0xend (Gondi) confirmed 0xsomeone (judge) commented:

The Warden has outlined how the protocol will incorrectly integrate with the DelegateRegistry system, attempting to revoke a previous delegation via an empty payload which is a futile attempt as proper revocation would require the same _rights to be passed in with a false value for the _enable flag.

I am slightly mixed in relation to this submission as the MultiSourceLoan::delegate function can be utilized with a correct payload to remove delegation from the previous user correctly. I believe that users, protocols, etc., will attempt to use the MultiSourceLoan::revokeDelegate function to revoke their delegation, and thus, a medium-risk severity rating is appropriate even though a circumvention already exists in the code.

To note, the code also goes against its interface specification ( src/interfaces/loans/IMultiSourceLoan.sol#L257-L261 ) further re-inforcing a medium-risk rating level.
