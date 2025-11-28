## Verified Patterns Found: 9

## Verified Patterns Found in following Categories:

- GriefableCallbacks
- StandardViolation
- FeeOnTransferAssumption
- AccessControlOrAuthByPass
- AccountingInvariantViolation
- UnsafeRecipient
- ReserveOrPriceDesync



## Summary of Patterns

Cross-Chain Whitelist Desynchronization Permanently Locks Bridged Funds

Cross-chain Denial of Service via Blacklist Synchronization Lag

Confiscated cross-chain assets become permanently trapped in Adapter

Accounting Invariant Broken by Blacklist Interaction in Cross-Chain Unlock

Cross-chain confiscation creates trapped assets in wiTryOFTAdapter due to accounting disconnect

Adapter assumes 1:1 transfers, creating insolvency risk with fee-on-transfer tokens

Blacklisted recipients cause permanent fund lockup in cross-chain bridge

wiTryOFTAdapter susceptible to insolvency if wiTRY introduces transfer fees

Bridge operations fail permanently for blacklisted users causing stuck funds

## Patterns



 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: wiTryOFTAdapter._credit

 ### Title
Cross-Chain Whitelist Desynchronization Permanently Locks Bridged Funds
 ### Description/Code Snippet
The `wiTRY` token enforces strict whitelist compliance on transfers. The `wiTryOFTAdapter` executes withdrawals via `_credit`, which calls `innerToken.safeTransfer(_to, _amountLD)`. If a user initiates a bridge transaction from a Spoke chain (where they are whitelisted) to the Hub chain, but is NOT whitelisted on the Hub (due to desynchronization or administrative removal), the `safeTransfer` will revert. The LayerZero message will fail and be stored in the endpoint. Since the recipient address (`_to`) in the payload cannot be modified during a retry, the funds remain permanently locked in the Adapter unless the user is explicitly re-whitelisted on the Hub.
 ### Static Signals
safeTransfer in _credit, whitelist restricted token transfers, no fallback recipient mechanism
 ### Assets at Risk
user collateral
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: wiTryOFTAdapter._credit

 ### Title
Cross-chain Denial of Service via Blacklist Synchronization Lag
 ### Description/Code Snippet
The `wiTryOFTAdapter` uses `safeTransfer` in its `_credit` function to release tokens to users bridging back from Spoke chains. `wiTRY` implements a blacklist that reverts transfers to/from blacklisted addresses. If a user is effectively blacklisted on the Hub chain (but managed to initiate a transfer from a Spoke chain, e.g. due to state lag or separate list management), the `safeTransfer` will revert. In LayerZero v2, this causes the message to fail and be stored in the Endpoint, permanently trapping the user's funds in the bridge message status. Unlike a local transfer where a revert preserves funds, this cross-chain state mismatch results in the destruction of Spoke tokens with no resulting unlock on the Hub.
 ### Static Signals
safeTransfer in _credit, Blacklist in innerToken, No fallback/rescue for blocked transfers
 ### Assets at Risk
User funds in transit
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: ReserveOrPriceDesync

 ### Relevant Function/Location: wiTryOFTAdapter.N/A

 ### Title
Confiscated cross-chain assets become permanently trapped in Adapter
 ### Description/Code Snippet
The protocol documentation specifies a compliance requirement where admins can 'confiscate' assets from blacklisted users via `redistributeLockedAmount`. However, the `wiTryOFTAdapter` (Hub) has no mechanism to sync with `wiTryOFT` (Spoke) confiscations. If an Admin confiscates/burns `wiTryOFT` from a user on a spoke chain, the corresponding backing `wiTRY` tokens remain locked in the Hub Adapter. Standard OFT Adapters do not allow extracting the inner token. This results in a Reserve Desync where the Adapter holds excess collateral that the Protocol Treasury cannot recover, effectively burning the value instead of seizing it.
 ### Static Signals
OFTAdapter locking mechanism, Protocol Confiscation/Blacklist logic, No sync/rescue function for innerToken
 ### Assets at Risk
wiTRY backing collateral
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: wiTryOFTAdapter._credit

 ### Title
Accounting Invariant Broken by Blacklist Interaction in Cross-Chain Unlock
 ### Description/Code Snippet
The wiTryOFTAdapter uses the default LayerZero OFTAdapter `_credit` implementation which attempts to unlock tokens via `innerToken.safeTransfer`. The underlying `wiTRY` token implements a blacklist mechanism (as noted in documentation). If a user is blacklisted on the Hub chain, the `safeTransfer` in `_credit` will revert, causing the LayerZero message to fail execution. Unlike a 'pull' pattern or try-catch block that would allow the user or admin to rescue the transfer, this revert permanently traps the user's assets in the adapter (as they are already burned on the source chain), violating the conservation of funds invariant between chains.
 ### Static Signals
safeTransfer used in _credit, innerToken has blacklist/pausable logic, no fallback or try/catch mechanism
 ### Assets at Risk
user deposits
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: wiTryOFTAdapter.N/A

 ### Title
Cross-chain confiscation creates trapped assets in wiTryOFTAdapter due to accounting disconnect
 ### Description/Code Snippet
The protocol implements compliance features including `redistributeLockedAmount` to confiscate funds from blacklisted users. On Spoke chains, the `wiTryOFT` (OFT implementation) allows the admin to seize/burn these tokens. However, the `wiTryOFTAdapter` on the Hub chain (which locks the backing `wiTRY` tokens) lacks a mechanism to handle these confiscation events remotely. When Spoke tokens are seized/burned, the corresponding collateral remains locked in the `wiTryOFTAdapter` contract with no way for the admin to retrieve it. This violates the accounting invariant (Spoke Supply vs Hub Locked Assets) and results in the permanent loss of the seized value rather than its recovery to the treasury/admin.
 ### Static Signals
Confiscation/Burn logic in Spoke Token, No remote seize handler in Adapter, Lock/Mint architecture
 ### Assets at Risk
wiTRY collateral locked in Adapter
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: wiTryOFTAdapter._debit

 ### Title
Adapter assumes 1:1 transfers, creating insolvency risk with fee-on-transfer tokens
 ### Description/Code Snippet
The `wiTryOFTAdapter` inherits `_debit` from `OFTAdapter`, which performs `safeTransferFrom` without checking the actual balance change of the contract. The explicit warning in `OFTAdapter` states the default implementation does not support fee-on-transfer tokens. If `wiTRY` (an RWA token subject to potential regulatory fees or taxes) implements or upgrades to include transfer fees, the adapter will credit more tokens on the destination chain than it actually locked, leading to protocol insolvency.
 ### Static Signals
assumes transferFrom(amount) credits exactly amount, no balanceBefore/After check, uses input amount instead of post-transfer delta
 ### Assets at Risk
wiTRY
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: wiTryOFTAdapter._credit

 ### Title
Blacklisted recipients cause permanent fund lockup in cross-chain bridge
 ### Description/Code Snippet
The `wiTryOFTAdapter` implements `_credit` by directly calling `innerToken.safeTransfer`. The `wiTRY` token implements strict compliance checks (blacklist/whitelist) in `_beforeTokenTransfer`. If a user is blacklisted on the hub chain while their funds are on a spoke chain, any attempt to bridge back will revert at the `safeTransfer` step. Because `OFTAdapter` lacks a `try/catch` mechanism or a fallback for failed transfers (e.g., storing pending withdrawals), the LayerZero message will permanently fail, locking the user's funds in the adapter contract indefinitely.
 ### Static Signals
no try/catch around external hook, callback success required for core flow, external call to user-controlled address
 ### Assets at Risk
wiTRY
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: wiTryOFTAdapter._debit

 ### Title
wiTryOFTAdapter susceptible to insolvency if wiTRY introduces transfer fees
 ### Description/Code Snippet
The `wiTryOFTAdapter` inherits the default `_debit` implementation from `OFTAdapter`, which assumes 1:1 lossless transfers. It uses `safeTransferFrom` to lock tokens but does not verify that the adapter's balance increased by `amountSentLD`. The `OFTAdapter` source code explicitly warns: 'IF the innerToken applies something like a transfer fee, the default will NOT work... a pre/post balance check will need to be done'. Given the protocol's extensive use of fees (mint, redeem, fast-redeem) and compliance gates, if `wiTRY` (or its upgrades) applies any transfer fee or deflationary logic, the bridge will mint full amounts on the destination while locking reduced amounts on the source, causing immediate insolvency.
 ### Static Signals
safeTransferFrom without balance check, Inheriting default OFTAdapter _debit, Protocol uses fees
 ### Assets at Risk
wiTRY backing in Adapter
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: wiTryOFTAdapter._credit

 ### Title
Bridge operations fail permanently for blacklisted users causing stuck funds
 ### Description/Code Snippet
The `wiTryOFTAdapter` uses `safeTransfer` in the `_credit` function to deliver bridged assets on the destination chain (Hub). The `wiTRY` token implements a blacklist mechanism (inherited from the protocol compliance layer). If a user initiates a bridge transaction from a Spoke chain and is subsequently blacklisted on the Hub chain before the message is executed, the `safeTransfer` call will revert. In the LayerZero OFT standard, the message payload (recipient) cannot be modified. Consequently, the transaction will permanently fail, and the bridged tokens will remain locked in the Adapter contract indefinitely, effectively burning the user's funds without recourse.
 ### Static Signals
innerToken.safeTransfer(_to, _amountLD), no try/catch block, underlying token has blacklist
 ### Assets at Risk
wiTRY
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

