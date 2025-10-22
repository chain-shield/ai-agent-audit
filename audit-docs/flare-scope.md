**NOTE**: ALL PRIVILEGED ROLES ARE TRUSTED, ONLY FOCUS ON PERMISSIONLESS EXPLOITS

### Tokens in scope

The FAssets system is able to support wrapped tokens for XRP, BTC and DOGE. However, the initial deployment will only have XRP (FXRP) enabled and that will be the sole scope of this audit competition. Any attacks related to FBTC, FDOGE, or UTXO-based logic in general, are out of scope.


## Areas of concern (where to focus for bugs)
- Bugs in Core Vault logic and interaction
- Bugs in smart contracts, protocol bugs.
- Accounting bugs, mostly when interacting cross chain.

## All trusted roles in the protocol

| Role                                | Description                       |
| --------------------------------------- | ---------------------------- |
| Governance (multi-sig)                          | controls protocol settings               |
| Agents                             |  provide minting and redeeming services. While Agents undergo KYC, they cannot be considered fully trusted—especially if significant potential gains could incentivize malicious behavior.                       |

**Note:** Vulnerabilities requiring access to the Agent role might be limited to medium severity due to accountability/recourse (subject to the discretion of the judge)

