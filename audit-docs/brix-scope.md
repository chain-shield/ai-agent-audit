## Publicly known issues

_Anything included in this section is considered a publicly known issue and is therefore ineligible for awards._

### Centralization Risks

Any centralization risks are out-of-scope for the purposes of this audit contest.

### Zellic Audit Report Issues

The codebase has undergone a Zellic audit with a fix review pending. The following issues identified in the Zellic audit are considered out-of-scope, with some being fixed in the current iteration of the codebase:

-  Blacklisted user can transfer tokens on behalf of non-blacklisted users using allowance - `_beforeTokenTransfer` does not validate `msg.sender`, a blacklisted caller can still initiate a same-chain token transfer on behalf of a non-blacklisted user as long as allowance exists.
- Griefing attacks around the `MIN_SHARES` variable of the ERC2646 vault: The protocol will perform an initial deposit to offset this risk. 
- The `redistributeLockedAmount` does not validate that the resulted `totalSupply` is not less than the minimum threshold. As a result of executing the `redistributeLockedAmount` function, the `totalSupply` amount may fall within a prohibited range between 0 and `MIN_SHARES` amount. And subsequent legitimate
deposits or withdrawals operations, which do not increase totalSupply to the `MIN_SHARES` value will be blocked.
- iTRY backing can fall below 1:1 on NAV drop. If NAV drops below 1, iTRY becomes undercollateralized with no guaranteed, on-chain remediation. Holders bear insolvency risk until a top-up or discretionary admin intervention occurs.
- Native fee loss on failed `wiTryVaultComposer.LzReceive` execution. In the case of underpayment, users will lose their fee and will have to pay twice to complete the unstake request.
- Non-standard ERC20 tokens may break the transfer function. If a non-standard token is recovered using a raw transfer, the function may appear to succeed, even though no tokens were transferred, or it may revert unexpectedly. This can result in tokens becoming stuck in the contract, which breaks the tokens rescue mechanism.

# Overview

The protocol enables the minting and redemption of iTRY tokens (and their staked counterpart wiTRY), which are backed by Digital Liquidity Fund (DLF) tokens representing shares of a traditional fund investing in Turkish Money Market Funds (MMF). The protocol includes cross-chain functionality via LayerZero integration.

## Links

- **Previous audits:** Zellic Report - [See known issues](#zellic-audit-report-issues)
- **Documentation:** https://hackmd.io/@EKJz7PaeT2GeAUJS83WWVw/SJPLb3QZWe
- **Website:** https://www.brix.money/
- **X/Twitter:** https://x.com/brix_money

### Files out of scope

| File         |
| ------------ |
|[script/config/\*\*.\*\*](https://github.com/code-423n4/2025-11-brix-money/tree/main/script/config)|
|[script/deploy/\*\*.\*\*](https://github.com/code-423n4/2025-11-brix-money/tree/main/script/deploy)|
|[script/old/\*\*.\*\*](https://github.com/code-423n4/2025-11-brix-money/tree/main/script/old)|
|[script/test/bridge/\*\*.\*\*](https://github.com/code-423n4/2025-11-brix-money/tree/main/script/test/bridge)|
|[script/test/composer/\*\*.\*\*](https://github.com/code-423n4/2025-11-brix-money/tree/main/script/test/composer)|
|[script/utils/\*\*.\*\*](https://github.com/code-423n4/2025-11-brix-money/tree/main/script/utils)|
|[src/external/\*\*.\*\*](https://github.com/code-423n4/2025-11-brix-money/tree/main/src/external)|
|[src/protocol/interfaces/\*\*.\*\*](https://github.com/code-423n4/2025-11-brix-money/tree/main/src/protocol/interfaces)|
|[src/protocol/periphery/\*\*.\*\*](https://github.com/code-423n4/2025-11-brix-money/tree/main/src/protocol/periphery)|
|[src/protocol/\*\*.\*\*](https://github.com/code-423n4/2025-11-brix-money/tree/main/rc/protocol)|
|[src/token/iTRY/\*\*.\*\*](https://github.com/code-423n4/2025-11-brix-money/tree/main/src/token/iTRY)|
|[src/token/wiTRY/crosschain/interfaces/\*\*.\*\*](https://github.com/code-423n4/2025-11-brix-money/tree/main/src/token/wiTRY/crosschain/interfaces)|
|[src/token/wiTRY/crosschain/libraries/\*\*.\*\*](https://github.com/code-423n4/2025-11-brix-money/tree/main/src/token/wiTRY/crosschain/libraries)|
|[src/token/wiTRY/interfaces/\*\*.\*\*](https://github.com/code-423n4/2025-11-brix-money/tree/main/src/token/wiTRY/interfaces)|
|[src/utils/\*\*.\*\*](https://github.com/code-423n4/2025-11-brix-money/tree/main/src/utils)|
|[test/\*\*.\*\*](https://github.com/code-423n4/2025-11-brix-money/tree/main/test)|
| Totals: 92 |

*For a machine-readable version, see [out_of_scope.txt](https://github.com/code-423n4/2025-11-brix-money/blob/main/out_of_scope.txt)*


# Additional context

## Areas of concern (where to focus for bugs)

The issues we are most concerned are those related to unbacked minting of iTry, the theft or loss of funds when staking/unstaking (particularly crosschain), and blacklist/whitelist bugs that would impair rescue operations in case of hacks or similar black swan events. More generally, the areas we want to verify are:

- Are the extensions to the Ethena Staking contract secure?
- Is the Composer contract secure?
- Can the system result in minting unbacked iTry?
- Are the access controls effective?
- Can the system deal with black swan scenarios?

## Main invariants

- The total issued iTry in the Issuer contract should always be be equal or lower to the total value of the DLF under custody. It should not be possible to mint "unbacked" iTry through the issuer. This does not mean that _totalIssuedITry needs to be equal to iTry.totalSupply(), though: there could be more than one minter contract using different backing assets.
- In the context of this audit, the NAV price queried can be assumed to be correct. The Oracle implementation will perform additional checks on the data feed and revert if it encounters issues.
- Blacklisted users cannot send/receive/mint/burn iTry tokens in any case.
- Only whitelisted user can send/receive/burn iTry tokens in a WHITELIST_ENABLED transfer state.
- Only non-blacklisted addresses can send/receive/burn iTry tokens in a FULLY_ENABLED transfer state.
- No adresses can send/receive tokens in a FULLY_DISABLED transfer state.

## All trusted roles in the protocol

| Role                                | Description                       | Privileges | Controlled By |
| --------------------------------------- | ---------------------------- | --------| -------- | 
| Owner                          | Admin             | Root | Protocol Multisig
| Blacklist Manager	| Manages blacklists in the system	| add/remove Blacklist entries for iTry and wiTry | Multisig |
| Whitelist Manager	| Manages whitelists in the system	| add/remove Blacklist entries for iTry	| Multisig |
| Soft Restricted Staker	| Can transfer the wiTry token, but cannot stake it	| Normal token usage, except staking to wiTry contract	| Blacklist Manager |
| Composer	| Accesses crosschain specific function in wiTry that allow to operate in name of another user |	Can use the composer-specific functions in the wiTry contract	| Owner |
| Minter |	Can mint iTry |	Can mint iTry	| Owner| 
| Blacklisted	| Cannot transfer funds |	none |	Blacklist Manager |
| Whitelisted	| Can transfer funds in “onlyWhitelist” state	| Token transfer |	Whitelist Manager | 
| Yield Processor	| Can trigger Yield distribution on the Issuer contract	|  Can call "processAccumulatedYield" in the iTryIssuer |	Owner | 

