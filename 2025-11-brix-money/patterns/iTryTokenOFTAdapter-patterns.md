## Verified Patterns Found: 11

## Verified Patterns Found in following Categories:

- ERC20DecimalsMismatch
- FeeOnTransferAssumption
- AccountingInvariantViolation
- GriefableCallbacks
- AccessControlOrAuthByPass
- StandardViolation
- UncheckedLowLevelCallResults
- UnsafeRecipient



## Summary of Patterns

Deployer retains administrative ownership due to constructor parameter mismatch

Adapter cached decimals desynchronization on underlying token upgrade

Cross-Chain DoS via Unhandled Blacklist Reverts

Cross-chain transfers to restricted recipients revert and permanently lock funds (UnsafeRecipient/DoS)

Immutable Decimal Cache Mismatch on Token Upgrade

Cross-chain bridge denial-of-service via blacklisted user blocking ordered message queue

Cross-Chain Whitelist Desync Locks User Funds

Compliance Invariant Violation: Bridged Collateral Cannot Be Seized

Incompatible with Fee-on-Transfer tokens due to missing balance delta checks

DoS of LayerZero bridge via blacklisted recipient in `_credit`

Insolvency Risk due to Lack of Fee-on-Transfer Support in OFTAdapter

## Patterns



 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: iTryTokenOFTAdapter.constructor

 ### Title
Deployer retains administrative ownership due to constructor parameter mismatch
 ### Description/Code Snippet
The `iTryTokenOFTAdapter` constructor accepts an `_owner` parameter but passes it to the `OFTAdapter` constructor as the `_delegate` argument. In standard `OFTCore`/`Ownable` implementations, contract ownership is initialized to `msg.sender` (the deployer), while the passed address only becomes the LayerZero Endpoint Delegate. This mismatch means the intended `_owner` (e.g., protocol multisig) fails to receive the `onlyOwner` privileges required to manage peers and critical configuration, leaving control with the deployer account.
 ### Static Signals
constructor argument _owner passed to _delegate param, no transferOwnership call
 ### Assets at Risk
protocol configuration, bridge security
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: iTryTokenOFTAdapter.constructor

 ### Title
Adapter cached decimals desynchronization on underlying token upgrade
 ### Description/Code Snippet
The `OFTAdapter` caches the underlying `innerToken` decimals in immutable storage within `OFTCore` during construction to handle `sharedDecimals` conversion. The `iTRY` token is documented as UUPS-upgradeable. If the `iTRY` token logic is upgraded to change its decimals, the Adapter will continue using the stale cached decimals value. This will result in incorrect amounts being minted on Spoke chains or unlocked on the Hub chain, permanently breaking the 1:1 accounting invariant.
 ### Static Signals
caching decimals in constructor, underlying token is upgradeable
 ### Assets at Risk
token supply invariant, user funds
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: UncheckedLowLevelCallResults

 ### Relevant Function/Location: iTryTokenOFTAdapter._credit

 ### Title
Cross-Chain DoS via Unhandled Blacklist Reverts
 ### Description/Code Snippet
The `_credit` function executes `innerToken.safeTransfer` to deliver tokens. Since the `iTRY` token implements a blacklist that causes transfers to revert, a cross-chain message destined for a blacklisted address will revert the `lzReceive` transaction. In LayerZero's default ordered message channel, a single failing message blocks the delivery of all subsequent messages. The contract fails to catch this revert (e.g., via try/catch) and handle the failure gracefully (e.g., by sending to a fallback address), allowing a malicious or blacklisted user to denial-of-service the bridge.
 ### Static Signals
safeTransfer in _credit, innerToken has blacklist, no try/catch block
 ### Assets at Risk
Cross-chain message availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: UnsafeRecipient

 ### Relevant Function/Location: iTryTokenOFTAdapter._credit

 ### Title
Cross-chain transfers to restricted recipients revert and permanently lock funds (UnsafeRecipient/DoS)
 ### Description/Code Snippet
The `iTryTokenOFTAdapter` inherits the default `OFTAdapter._credit` implementation, which calls `innerToken.safeTransfer(_to, amount)`. `iTRY` is a regulatory-compliant token with strict Whitelist/Blacklist enforcement. If a user initiates a cross-chain transfer (burn on Spoke) but their address is blacklisted or removed from the whitelist on the Hub chain before the message is delivered, the `safeTransfer` call will revert. In LayerZero, a reverting `lzReceive` causes the message to fail. Unlike robust bridge implementations that catch failed transfers and store the funds for manual claiming (or redirect to a recovery address), this implementation reverts the entire transaction. Since the message payload (recipient address) is immutable, the user's funds remain permanently locked in the Adapter contract on the Hub chain with no mechanism to retry or recover them.
 ### Static Signals
safeTransfer without try/catch, token has blacklist/whitelist, cross-chain message handling
 ### Assets at Risk
User funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ERC20DecimalsMismatch

 ### Relevant Function/Location: iTryTokenOFTAdapter.constructor

 ### Title
Immutable Decimal Cache Mismatch on Token Upgrade
 ### Description/Code Snippet
The `OFTAdapter` initializes its local decimal configuration by reading `IERC20Metadata(_token).decimals()` inside the `constructor` and passing it to `OFTCore`, where it is stored as an immutable variable. Since `iTryToken` is upgradeable, a future upgrade could fundamentally change the token's decimals (e.g., during a redenomination). The Adapter would fail to detect this change and continue using the stale immutable decimal value for `amountLD` <-> `amountSD` conversions, resulting in cross-chain minting amounts that are incorrect by orders of magnitude.
 ### Static Signals
decimals() read in constructor, immutable decimals in OFTCore, Upgradeable innerToken
 ### Assets at Risk
locked iTRY collateral, user funds
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: iTryTokenOFTAdapter._credit

 ### Title
Cross-chain bridge denial-of-service via blacklisted user blocking ordered message queue
 ### Description/Code Snippet
The `iTryTokenOFTAdapter` relies on `safeTransfer` to credit tokens to users on the Hub chain upon receiving a cross-chain message. The underlying `iTRY` token implements a blacklist which causes transfers to blocked users to revert. If a user is blacklisted on the Hub chain (Ethereum) but initiates a bridge transfer from a Spoke chain, the resulting `_credit` call in the Adapter will revert. In LayerZero v2's default ordered execution mode, this failing transaction will block the nonce channel, preventing all subsequent valid cross-chain transfers from that Spoke chain from being processed, effectively DoSing the bridge.
 ### Static Signals
safeTransfer used in _credit, underlying token has blacklist/revert-on-transfer logic
 ### Assets at Risk
cross-chain liquidity, user funds stuck in transit
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: iTryTokenOFTAdapter._credit

 ### Title
Cross-Chain Whitelist Desync Locks User Funds
 ### Description/Code Snippet
The protocol enforces whitelist compliance on both the Hub (via `iTRY` token) and Spoke (via `iTryTokenOFT`). If a user initiates a transfer from Hub to Spoke while whitelisted on the Hub but not on the Spoke (due to state desynchronization or race conditions), the transaction will successfully lock tokens on the Hub but revert during the `_mint` process on the Spoke. LayerZero v2 will store the failed message payload, but there is no automatic refund mechanism. The user's funds remain locked in the Hub adapter indefinitely, effectively burning them.
 ### Static Signals
emitted != claimed + unclaimed, no revert bubble
 ### Assets at Risk
user funds
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: iTryTokenOFTAdapter.N/A

 ### Title
Compliance Invariant Violation: Bridged Collateral Cannot Be Seized
 ### Description/Code Snippet
The protocol's compliance model requires that the Admin can seize funds from blacklisted users via `redistributeLockedAmount`. However, when users bridge `iTry` to a spoke chain, their tokens are locked in the `iTryTokenOFTAdapter` contract. The Adapter lacks any function to allow the Admin to extract or seize these locked tokens. Consequently, if a user is blacklisted while their funds are bridged (or bridging), the Admin loses the ability to seize the underlying collateral, violating a critical regulatory compliance invariant.
 ### Static Signals
No rescue/seize function, Adapter holds user collateral, Token has seizure logic
 ### Assets at Risk
Blacklisted user collateral
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: FeeOnTransferAssumption

 ### Relevant Function/Location: iTryTokenOFTAdapter._debit

 ### Title
Incompatible with Fee-on-Transfer tokens due to missing balance delta checks
 ### Description/Code Snippet
The `iTryTokenOFTAdapter` inherits the default `OFTAdapter` implementation of `_debit`, which calculates `amountSentLD` and transfers it from the user via `innerToken.safeTransferFrom`. It assumes the adapter receives exactly `amountSentLD`. However, the `iTry` token is UUPS-upgradeable. If the token logic is upgraded to include transfer fees or deflationary mechanisms, the adapter will receive less than `amountSentLD`, but the cross-chain message will likely still credit the full amount (or an amount calculated without knowledge of the fee). This discrepancy breaks the 1:1 backing invariant, leading to bridge insolvency. The `OFTAdapter` source code explicitly warns that the default implementation assumes lossless transfers and fails if fees are applied.
 ### Static Signals
uses input amount instead of post-transfer delta, no balanceBefore/After check, assumes transferFrom(amount) credits exactly amount
 ### Assets at Risk
innerToken
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: iTryTokenOFTAdapter._credit

 ### Title
DoS of LayerZero bridge via blacklisted recipient in `_credit`
 ### Description/Code Snippet
The `_credit` function in `OFTAdapter` (inherited by `iTryTokenOFTAdapter`) executes `innerToken.safeTransfer(_to, _amountLD)` to unlock tokens on the destination chain. The underlying `iTRY` token implements a blacklist that reverts transfers to restricted addresses. If a cross-chain message targets a user who is blacklisted (or becomes blacklisted while the message is in flight), the `safeTransfer` will revert. In LayerZero, a reverting `lzReceive` execution prevents the nonce from advancing, effectively blocking the message channel and causing a Denial of Service for all subsequent legitimate users on that path until the blocked message is manually handled.
 ### Static Signals
no try/catch around external hook, callback success required for core flow to proceed, external call to user-controlled address
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: iTryTokenOFTAdapter._debit

 ### Title
Insolvency Risk due to Lack of Fee-on-Transfer Support in OFTAdapter
 ### Description/Code Snippet
The `OFTAdapter` implementation assumes lossless 1:1 transfers in `_debit`. It calculates `amountSentLD` equal to the requested amount and instructs the destination chain to mint that amount. However, `iTryToken` is UUPS-upgradeable and could be upgraded to include transfer fees (a common feature in regulated stablecoins). If fees are introduced, the Adapter will lock fewer tokens than are minted on the spoke chain, leading to immediate protocol insolvency. The adapter code warns about this but does not enforce a balance check.
 ### Static Signals
Upgradeable innerToken, No pre/post balance check in _debit, Assumes amountSentLD == amountLD
 ### Assets at Risk
iTryToken backing collateral
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole

