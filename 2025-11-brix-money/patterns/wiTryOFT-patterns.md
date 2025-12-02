## Verified Patterns Found: 14

## Verified Patterns Found in following Categories:

- NonStandardERC20Behavior
- AccessControlOrAuthByPass
- AccountingInvariantViolation
- StandardViolation
- ConfigFootgun
- GriefableCallbacks



## Summary of Patterns

Missing Whitelist Enforcement Enables Compliance Bypass

Renouncing ownership permanently bricks cross-chain transfers to blacklisted users

Blacklisted Owner DoS breaks redistribution and blocks cross-chain channels

Blacklisted users can bypass frozen state via approve/allowance functions

Strict blacklist prevents liquidations (DoS on TransferFrom)

Blacklisted Owner disables cross-chain confiscation and redistribution

Inability to exempt LayerZero Endpoint from blacklist checks enables DoS

Potential Bridge DoS if LayerZero Endpoint is blacklisted

Misleading OFTReceived event emission during blacklist redirection

Bridge DoS and Seizure Failure when Owner is Blacklisted or Renounced

Redistribution Logic Failure if Owner is Blacklisted

OFTReceived event misrepresents recipient of redirected funds

Renouncing ownership bricks cross-chain confiscation logic for blacklisted users

Bridge DoS via Blacklisting LayerZero Endpoint

## Patterns



 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: wiTryOFT.N/A

 ### Title
Missing Whitelist Enforcement Enables Compliance Bypass
 ### Description/Code Snippet
The protocol documentation explicitly states that wiTryOFT must enforce 'KYC whitelist & blacklist' compliance rules mirroring the hub chain. However, the implemented contract entirely lacks whitelist logic, roles, or checks. This allows a blacklisted entity to bypass the blacklist by generating a fresh address (which requires no KYC) and bridging funds to it via LayerZero. The lack of whitelist enforcement directly contradicts the 'Systems-Level' compliance architecture described.
 ### Static Signals
Missing WHITELISTED_ROLE, No whitelist check in _beforeTokenTransfer
 ### Assets at Risk
Compliance integrity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ConfigFootgun

 ### Relevant Function/Location: wiTryOFT._credit

 ### Title
Renouncing ownership permanently bricks cross-chain transfers to blacklisted users
 ### Description/Code Snippet
The `_credit` function implements compliance logic by checking if the recipient `_to` is blacklisted. If so, it calls `super._credit(owner(), ...)` to redirect funds to the owner. If the protocol renounces ownership (setting `owner` to `address(0)`), this call attempts to mint tokens to the zero address, which reverts in the underlying ERC20 implementation. This causes the LayerZero message to fail on the destination chain. For ordered channels or strict nonce handling, this blocked message will prevent the execution of all subsequent messages, resulting in a Denial of Service for the bridge.
 ### Static Signals
return super._credit(owner(), ...), no address(0) check for owner()
 ### Assets at Risk
user funds, bridge availability
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: wiTryOFT.redistributeBlackListedFunds

 ### Title
Blacklisted Owner DoS breaks redistribution and blocks cross-chain channels
 ### Description/Code Snippet
The contract enforces blacklist checks in `_beforeTokenTransfer`, which reverts if `msg.sender` or `_to` is blacklisted. However, the `redistributeBlackListedFunds` function (which transfers seized funds to `owner`) and the `_credit` function (which redirects incoming funds for blacklisted users to `owner`) both require transferring tokens to the `owner` address. If the `owner` address is blacklisted (e.g., by a malicious or compromised `blackLister`), `_beforeTokenTransfer` will revert during these operations. This causes `redistributeBlackListedFunds` to fail (locking funds) and `_credit` to revert (blocking the LayerZero cross-chain channel for the blacklisted user indefinitely), effectively bricking the compliance mechanism and cross-chain messaging.
 ### Static Signals
blackList[_to] check redirects to owner(), redistribute calls _transfer to owner(), _beforeTokenTransfer reverts if blackList[_to] (owner) is true
 ### Assets at Risk
seized funds, cross-chain availability
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: wiTryOFT.approve

 ### Title
Blacklisted users can bypass frozen state via approve/allowance functions
 ### Description/Code Snippet
The contract implements a blacklist via `_beforeTokenTransfer` which correctly blocks transfers where a blacklisted user is the sender, receiver, or caller. However, standard ERC20 functions `approve`, `increaseAllowance`, and `decreaseAllowance` do not trigger `_beforeTokenTransfer`. This allows a blacklisted 'frozen' user to still modify contract state by changing allowances. While they cannot currently transfer funds (as `transferFrom` would fail), this capability violates the standard compliance invariant that a blacklisted address should be incapable of any state-changing actions.
 ### Static Signals
inherits ERC20, overrides _beforeTokenTransfer, does not override approve
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: NonStandardERC20Behavior

 ### Relevant Function/Location: wiTryOFT._beforeTokenTransfer

 ### Title
Strict blacklist prevents liquidations (DoS on TransferFrom)
 ### Description/Code Snippet
The `_beforeTokenTransfer` hook reverts if `_from` is blacklisted, even if the caller (`msg.sender`) is a trusted spender with valid allowance. This strict check breaks the assumption that approved spenders (such as Lending Protocols or Liquidation Bots) can always transfer funds up to their allowance. If `wiTryOFT` is used as collateral, this behavior causes a Denial of Service during liquidations of blacklisted users, potentially leaving the lending protocol with bad debt.
 ### Static Signals
if (blackList[_from]) revert BlackListed(_from), revert in transfer path
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: wiTryOFT._credit

 ### Title
Blacklisted Owner disables cross-chain confiscation and redistribution
 ### Description/Code Snippet
The protocol implements compliance logic where funds destined for a blacklisted user are redirected to the `owner()` (`_credit`), and locked funds can be seized to the `owner()` (`redistributeBlackListedFunds`). However, `_beforeTokenTransfer` enforces `if (blackList[_to]) revert`. If the `blackLister` role adds the `owner()` address to the blacklist, all transfers TO the owner will revert. This causes `_credit` (on cross-chain receiving) and `redistributeBlackListedFunds` to fail, creating a DoS vector for these critical compliance flows.
 ### Static Signals
super._credit(owner(), ...), transfer(_from, owner(), ...), blackList[_to] revert
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: ConfigFootgun

 ### Relevant Function/Location: wiTryOFT._beforeTokenTransfer

 ### Title
Inability to exempt LayerZero Endpoint from blacklist checks enables DoS
 ### Description/Code Snippet
The `_beforeTokenTransfer` function enforces blacklist checks on `msg.sender`. During the execution of an inbound LayerZero message (`lzReceive` -> `_credit` -> `_mint`), `msg.sender` is the LayerZero Endpoint contract. If the `blackLister` role adds the LayerZero Endpoint address to the blacklist (either accidentally or maliciously), all cross-chain minting operations will revert. This results in a complete Denial of Service for the bridge functionality.
 ### Static Signals
blackList[msg.sender], no exemption for endpoint
 ### Assets at Risk
bridge availability
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: wiTryOFT._beforeTokenTransfer

 ### Title
Potential Bridge DoS if LayerZero Endpoint is blacklisted
 ### Description/Code Snippet
The `_beforeTokenTransfer` hook strictly enforces `if (blackList[msg.sender]) revert...`. When receiving cross-chain messages via `lzReceive`, the `msg.sender` is the LayerZero Endpoint contract. If the `blackLister` role (which can be distinct from the owner) adds the Endpoint address to the blacklist, all incoming cross-chain packets will revert, causing a complete denial of service for bridging operations.
 ### Static Signals
check on msg.sender in hook, no exemption for lzEndpoint
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: wiTryOFT._credit

 ### Title
Misleading OFTReceived event emission during blacklist redirection
 ### Description/Code Snippet
The `_credit` function overrides the standard OFT behavior to redirect funds to the `owner` if the recipient (`_to`) is blacklisted. However, it returns the full minted amount to the caller (`OFTCore._lzReceive`), which subsequently emits the `OFTReceived` event with the original `_to` address and the amount. This creates a discrepancy where the event log indicates the user received funds, while the on-chain state shows the owner received them, potentially misleading off-chain indexers and wallets.
 ### Static Signals
return super._credit(owner(), _amountLD, _srcEid), emit RedistributeFunds(_to, _amountLD)
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: wiTryOFT._credit

 ### Title
Bridge DoS and Seizure Failure when Owner is Blacklisted or Renounced
 ### Description/Code Snippet
The `_credit` function implements a compliance mechanism where funds bridging to a blacklisted user are redirected to the `owner()`. This is achieved by calling `super._credit(owner(), ...)` which triggers `_mint(owner(), ...)`. The `_mint` function invokes `_beforeTokenTransfer`, which validates that the recipient is not blacklisted (`if (blackList[_to]) revert BlackListed(_to)`). 

If the `owner` address is blacklisted (e.g. by the `blackLister` role), the seizure operation will revert. This causes the inbound LayerZero message to fail, potentially blocking the cross-chain channel and preventing the intended seizure. Similarly, if ownership is renounced (`owner() == address(0)`), `_mint` to the zero address reverts, causing the same Denial of Service.
 ### Static Signals
redirects funds to owner(), mint triggers _beforeTokenTransfer, _beforeTokenTransfer checks blacklist for recipient, no check for owner() validity or status before redirect
 ### Assets at Risk
Cross-chain liquidity, Seizable funds
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: wiTryOFT._credit

 ### Title
Redistribution Logic Failure if Owner is Blacklisted
 ### Description/Code Snippet
In `_credit`, if the recipient is blacklisted, the contract attempts to redistribute funds by minting them to `owner()`. However, `_beforeTokenTransfer` validates that `_to` (the recipient) is not blacklisted. If the `owner` address itself is blacklisted (maliciously or accidentally), the redirection mint will revert. This causes the LayerZero message to fail, blocking the channel and preventing the intended confiscation/redistribution of funds.
 ### Static Signals
super._credit(owner(), ...), blackList[_to] check in _beforeTokenTransfer
 ### Assets at Risk
Confiscated funds, Cross-chain message processing
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: wiTryOFT._credit

 ### Title
OFTReceived event misrepresents recipient of redirected funds
 ### Description/Code Snippet
When `_credit` redirects funds from a blacklisted user to the `owner`, it returns the minted amount (`_amountLD`) to the caller (`OFTCore.lzReceive`). The `OFTCore` implementation then emits the standard `OFTReceived` event with the original `_to` address and the returned amount. This creates a discrepancy where the event log indicates the blacklisted user received the tokens, while the actual tokens were minted to the owner. This breaks accounting invariants for off-chain indexers and wallets relying on standard OFT events.
 ### Static Signals
emit RedistributeFunds, return super._credit(owner(), ...)
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: wiTryOFT._credit

 ### Title
Renouncing ownership bricks cross-chain confiscation logic for blacklisted users
 ### Description/Code Snippet
The `_credit` function overrides the default OFT behavior to enforce a blacklist: if the recipient `_to` is blacklisted, it attempts to mint the tokens to `owner()` instead (confiscation). However, if the contract ownership is renounced (setting owner to `address(0)`), `owner()` returns the zero address. The subsequent call to `super._credit(owner(), ...)` triggers `_mint` to the zero address, which reverts in the ERC20 implementation. This causes the LayerZero message to fail (revert) rather than successfully confiscating the funds, potentially blocking the cross-chain message channel or permanently trapping the funds in a failed state.
 ### Static Signals
super._credit(owner(), _amountLD, _srcEid), no check for owner() != address(0)
 ### Assets at Risk
Cross-chain funds of blacklisted users
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresAdminRole




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: wiTryOFT._beforeTokenTransfer

 ### Title
Bridge DoS via Blacklisting LayerZero Endpoint
 ### Description/Code Snippet
The `_beforeTokenTransfer` hook enforces a blacklist check on `msg.sender` to prevent blacklisted users from initiating transfers. However, this hook is also triggered during inbound cross-chain transfers via `_credit` (which calls `_mint`). During `_credit`, `msg.sender` is the LayerZero Endpoint contract. If the Endpoint address is added to the blacklist (e.g., by an automated compliance bot or operator error), all inbound cross-chain messages will revert, causing a complete Denial of Service of the bridge.
 ### Static Signals
blackList[msg.sender] check in _beforeTokenTransfer, _credit calls super._credit/_mint
 ### Assets at Risk
Cross-chain liquidity, Bridge availability
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole

