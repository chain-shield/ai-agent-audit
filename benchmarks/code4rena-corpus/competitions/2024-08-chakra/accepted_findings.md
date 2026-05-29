# Accepted H/M Findings: Chakra

# [H-01] Invalid token address used in ChakraSettlementHandler::cross_chain_erc20_settlement(...) leading to invalid transaction creation and event emission

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-08-chakra
- **Source snapshot:** competitions/2024-08-chakra/final_report.html

ChakraSettlementHandler::cross_chain_erc20_settlement(...) leading to invalid transaction creation and event emission Submitted by 0xb0k0, also found by AllTooWell, rbserver, said, shaflow2, fyamf, Abdessamed, SBSecurity, calfun0x, joicygiore, ABAIKUNANBAEV, Coinymous, Audinarey, b0g0, gajiknownnothing, mojito_auditor, devival, and jasonxiale Lines of code

- https://github.com/code-423n4/2024-08-chakra/blob/d0d45ae1d26ca1b87034e67180fac07ce9642fd9/solidity/handler/contracts/ChakraSettlementHandler.sol#L160
- https://github.com/code-423n4/2024-08-chakra/blob/d0d45ae1d26ca1b87034e67180fac07ce9642fd9/solidity/handler/contracts/ChakraSettlementHandler.sol#L218

## Impact

The ChakraSettlementHandler incorrectly passes the contract address instead of the token one, when creating the create_cross_txs transaction and when emitting the CrossChainLocked event. The contract depends heavily on correct event emission as the protocol’s validators listen for these events and execute state-changing operations based on them.

## Recommended Mitigation Steps

Use the correct from_token address in ChakraSettlementHandler::cross_chain_erc20_settlement(...).

diff --git a/solidity/handler/contracts/ChakraSettlementHandler.sol b/solidity/handler/contracts/ChakraSettlementHandler.sol index 5d31ef9..b9cbd6d 100644 --- a/solidity/handler/contracts/ChakraSettlementHandler.sol +++ b/solidity/handler/contracts/ChakraSettlementHandler.sol @@ -157,7 +157,7 @@ contract ChakraSettlementHandler is BaseSettlementHandler, ISettlementHandler { to_chain, msg.sender, to, - address(this), + token, to_token, amount, CrossChainTxStatus.Pending @@ -215,7 +215,7 @@ contract ChakraSettlementHandler is BaseSettlementHandler, ISettlementHandler { to, chain, to_chain, - address(this), + token, to_token, amount, mode pidb (Chakra) confirmed via duplicate issue #175 0xsomeone (judge) increased severity to High and commented:

The Warden has identified an invalid event emission and transaction data storage both of which are imperative to the proper operation of a cross-chain bridge system.

The documentation of the project does not adequately detail how those events are consumed, and inspection of the project’s cairo code as well as the sponsor’s own affirmation on issue #175 confirms the present implementation is incorrect.

I believe a high-risk issue is acceptable for this submission due to how crucial those components are to the proper operation of a cross-chain bridge.

# [H-02] In Starknet, already processed messages can be re-submitted and by anyone

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-08-chakra
- **Source snapshot:** competitions/2024-08-chakra/final_report.html

Submitted by Moksha, also found by Abdessamed, aldarion, ABAIKUNANBAEV, 0xAsen, Tigerfrake, 0x37, blackpanther, mjcpwns, kutugu, pwnforce, avoloder, Audinarey, jasonxiale, Daniel_eth, SBSecurity, 0xNirix, Emmanuel, Respx, tekkac, peanuts, b0g0, LuarSec, haxatron, klau5, said, shaflow2, Shaheen, mojito_auditor, fyamf, devival, ZdravkoHr, Drynooo, Agontuk, AllTooWell, and PASCAL

## Impact

The impact is high as the adversary can call the receive_cross_chain_msg with the already processed and valid parameters and benefit from it. With such parameters, anyone can call not just the validators so it further affects the protocol. Due to the missing check if the txId status is UNKNOW the attack is very much feasible as shown in the below fork testing local POC. This also breaks the main invariant mentioned in the audit docs/readme that A cross-chain message can only be received once. The status of a received transaction in receive_cross_txs must be CrossChainMsgStatus.Unknow before it can be processed.

## Recommended Mitigation Steps

Add check in the receive_cross_chain_msg handler/settlement.cairo to check the cross chain status of the txid.

assert(self.created_tx.read(cross_chain_msg_id.try_into().unwrap()).tx_status == CrossChainMsgStatus::UNKNOW, 'tx status error'); zvlwwj (Chakra) confirmed via duplicate issue #131 0xsomeone (judge) commented:

The Warden and its duplicates have demonstrated that it is possible to replay transactions in the Chakra system due to a lack of transaction status validation. I believe a high-risk severity rating is appropriate for this submission as it breaches a core invariant of the system and leads to fund loss.

# [H-03] SettlementSignatureVerifier is missing check for duplicate validator signatures

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-08-chakra
- **Source snapshot:** competitions/2024-08-chakra/final_report.html

SettlementSignatureVerifier is missing check for duplicate validator signatures Submitted by SBSecurity, also found by Abdessamed, joicygiore ( 1, 2 ), 0xHelium, aldarion ( 1, 2 ), Sabit, 0x37 ( 1, 2 ), Tigerfrake, kutugu, Trooper, BY_DLIFE, Brene, jasonxiale, Coinymous ( 1, 2, 3 ), Cybrid, DanielArmstrong, IzuMan, bhilare_, 0x18a6 ( 1, 2 ), kind_killerwhale, 0xNirix, Kaysoft, calc1f4r ( 1, 2, 3 ), DimaKush, Shubham, Respx ( 1, 2 ), agent3bood, b0g0 ( 1, 2 ), Taiger ( 1, 2, 3 ), 10ap17 ( 1, 2 ), Omik, tekkac, jesjupyter, Bauchibred, Subroutine ( 1, 2 ), eierina, minato7namikazi, LuarSec ( 1, 2 ), JanuaryPersimmon2024 ( 1, 2 ), haxatron, said ( 1, 2 ), shaflow2 (

1, 2 ), mojito_auditor, fyamf ( 1, 2 ), gesha17, klau5 ( 1, 2 ), 4rdiii, ZdravkoHr, yudan, NexusAudits, Sparrow, and biakia

## Impact

SettlementSignatureVerifier::verifyECDSA and settlement::check_chakra_signatures lack checks for duplicate validators and allow passing the threshold with a single valid signature.

## Recommended Mitigation Steps

For Solidity:

+ mapping (bytes32 msgHash => mapping(address validator => bool usedSig)) public isSignatureUsed; function verifyECDSA( bytes32 msgHash, bytes calldata signatures ) internal view returns (bool) { require( signatures.length % 65 == 0, "Signature length must be a multiple of 65" ); uint256 len = signatures.length; uint256 m = 0; for (uint256 i = 0; i < len; i += 65) { bytes memory sig = signatures[i:i + 65]; + address validator = msgHash.recover(sig); if ( - validators[msgHash.recover(sig)] && ++m >= required_validators + !isSignatureUsed[msgHash][validator] && validators[validator] ) { + isSignatureUsed[msgHash][validator] = true; + m++; } + if (m >= required_validators) { + return true; + } }

return false; } For Cairo:

fn check_chakra_signatures( self: @ContractState, message_hash: felt252, signatures: Array<(felt252, felt252, bool)> ) { let mut pass_count = 0; let mut i = 0; // A set-like structure to keep track of processed public keys.

+ let mut processed_pub_keys: LegacyMap<felt252, u8> = LegacyMap::new(); loop { if i > signatures.len() - 1 { break; } let (r, s, y) = *signatures.at(i); let pub_key: felt252 = recover_public_key(message_hash, r, s, y).unwrap(); - if self.chakra_validators_pubkey.read(pub_key) > 0{ + if processed_pub_keys.get(pub_key) == 0 && self.chakra_validators_pubkey.read(pub_key) > 0 { pass_count += 1; + processed_pub_keys.append(pub_key); } i += 1; } assert(pass_count >= self.required_validators_num.read(), 'Not enough valid signatures'); } pidb (Chakra) disputed and commented on duplicate issue #4:

We added a nonce to the txid, and then will the txid be included in the signature? I think this can prevent this problem.

0xsomeone (judge) increased severity to High and commented:

The Warden and its duplicates have properly identified that a single signature can be repeated an arbitrary number of times to satisfy the required validator threshold with a single validator.

I believe a high-risk rating is appropriate for this submission as it represents a significant invariant breach and a compromise of the signature verification system employed within the system.

# [H-04] In settlement.cairo::receive_cross_chain_msg - the message will always be marked with Status::SUCCESS

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Finding ID:** H-04
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-08-chakra
- **Source snapshot:** competitions/2024-08-chakra/final_report.html

settlement.cairo::receive_cross_chain_msg - the message will always be marked with Status::SUCCESS Submitted by 0xAsen, also found by shaflow2, ABAIKUNANBAEV, SBSecurity, and Sabit

- https://github.com/code-423n4/2024-08-chakra/blob/d0d45ae1d26ca1b87034e67180fac07ce9642fd9/cairo/settlement/src/settlement.cairo#L356
- https://github.com/code-423n4/2024-08-chakra/blob/d0d45ae1d26ca1b87034e67180fac07ce9642fd9/cairo/handler/src/handler_erc20.cairo#L135

## Impact

When receiving a message via the settlement.cairo::receive_cross_chain_msg() function, a call is made to handler.receive_cross_chain_msg and the return value of that call is used to mark the message with status SUCCESS or FAILED However, the way that the handler.receive_cross_chain_msg function is implemented, it’ll always return true, therefore marking every message as successful even if the message for some reason was a corrupted one.

And breaking one of the main invariants that states the message statuses should be tracked correctly.

## Recommended Mitigation Steps

Implement the functionality like in the solidity handler function so it returns false if something goes wrong.

zvlwwj (Chakra) confirmed

# [H-05] settlement.cairo doesn’t process callback correctly leading to CrossChainMsgStatus marked as SUCCESS even if it failed on destination chain

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Finding ID:** H-05
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-08-chakra
- **Source snapshot:** competitions/2024-08-chakra/final_report.html

settlement.cairo doesn’t process callback correctly leading to CrossChainMsgStatus marked as SUCCESS even if it failed on destination chain Submitted by 0xAsen, also found by Tigerfrake, pwnforce, b0g0, ABAIKUNANBAEV, and AllTooWell When a cross-chain message is sent it can return a callback with status FAILED or SUCCESS. The problem is that even if the cross-chain message failed, the original message status on the source chain would be marked as SUCCESS.

Let’s take a look at the receive_cross_chain_callback function on the settlement.cairo contract, and more specifically the part that updates the status:

fn receive_cross_chain_callback( ref self: ContractState, cross_chain_msg_id: felt252, from_chain: felt252, to_chain: felt252, from_handler: u256, to_handler: ContractAddress, cross_chain_msg_status: u8, <-- sign_type: u8, signatures: Array<(felt252, felt252, bool)>, ) -> bool { //other functionality let success = handler.receive_cross_chain_callback(cross_chain_msg_id, from_chain, to_chain, from_handler, to_handler, cross_chain_msg_status); let mut state = CrossChainMsgStatus::PENDING; if success{ state = CrossChainMsgStatus::SUCCESS; }else{ state = CrossChainMsgStatus::FAILED; } self.created_tx.write(cross_chain_msg_id, CreatedTx{ tx_id:cross_chain_msg_id, tx_status:state, <--- update the status

from_chain: to_chain, to_chain: from_chain, from_handler: to_handler, to_handler: from_handler }); The problem is that as long as the call to handler.receive_cross_chain_callback function was successful, the message as a whole will be marked in a SUCCESS state even though that cross_chain_msg_status could be SUCCESS or FAILED depending on if the message failed on the destination chain.

This could lead to a situation where a message fails to get processed on the destination chain, a callback is returned with cross_chain_msg_status == FAILED but the message is marked as SUCCESS.

That situation could be a user trying to bridge his tokens, the bridging fails so he doesn’t receive his tokens on the destination chain, a callback is made and the message is marked as a SUCCESS even though it as not successfully executed.

And if we see the code of the handler.receive_cross_chain_callback function, we’ll see that it’d always return true as long as it doesn’t revert:

fn receive_cross_chain_callback( ref self: ContractState, cross_chain_msg_id: felt252, from_chain: felt252, to_chain: felt252, from_handler: u256, to_handler: ContractAddress, cross_chain_msg_status: u8 ) -> bool{ assert(to_handler == get_contract_address(),'error to_handler'); assert(self.settlement_address.read() == get_caller_address(), 'not settlement'); assert(self.support_handler.read((from_chain, from_handler)) && self.support_handler.read((to_chain, contract_address_to_u256(to_handler))), 'not support handler'); let erc20 = IERC20MintDispatcher{contract_address: self.token_address.read()}; if self.mode.read() == SettlementMode::MintBurn{ erc20.burn_from(get_contract_address(), self.created_tx.read(cross_chain_msg_id).amount);

} let created_tx = self.created_tx.read(cross_chain_msg_id); self.created_tx.write(cross_chain_msg_id, CreatedCrossChainTx{ tx_id: created_tx.tx_id, from_chain: created_tx.from_chain, to_chain: created_tx.to_chain, from:created_tx.from, to:created_tx.to, from_token: created_tx.from_token, to_token: created_tx.to_token, amount: created_tx.amount, tx_status: CrossChainTxStatus::SETTLED }); return true; } The function just performs validation of the handlers, if the settlement contract calls it and returns true. These validation could all be true but the initial message could still be with a FAILED status.

This is not taken into account and could lead to a failed message being marked as a successful one.

The implementation on the solidity contract is correct:

function processCrossChainCallback( uint256 txid, string memory from_chain, uint256 from_handler, address to_handler, CrossChainMsgStatus status, uint8 sign_type, bytes calldata signatures ) internal { require( create_cross_txs[txid].status == CrossChainMsgStatus.Pending, "Invalid transaction status" ); if ( ISettlementHandler(to_handler).receive_cross_chain_callback( txid, from_chain, from_handler, status, sign_type, signatures ) ) { create_cross_txs[txid].status = status; <--- } else { create_cross_txs[txid].status = CrossChainMsgStatus.Failed; } As you can see, even if the call to the handler was successful, the status is updated with the CrossChainMsgStatus that was passed to the function and it is not automatically marked with

SUCCESS.

This should be the case in the cairo function as well but right now the cross_chain_msg_status parameter is ignored.

## Recommended Mitigation Steps

Change this line from this:

if success{ state = CrossChainMsgStatus::SUCCESS; to this:

if success{ state = cross_chain_msg_status; To be consistent with the solidity implementation.

zvlwwj (Chakra) confirmed 0xsomeone (judge) commented:

The Warden has identified that the status of a cross-chain message is not properly set when a callback is performed on the source chain.

I believe a severity of high is appropriate, as a failed cross-chain transaction would indicate it was processed successfully when this vulnerability manifests.

# [H-06] Forcing Starknet handlers to be whitelisted on the same chain allows exploit of BurnUnlock mode to drain handler funds

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Finding ID:** H-06
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-08-chakra
- **Source snapshot:** competitions/2024-08-chakra/final_report.html

BurnUnlock mode to drain handler funds Submitted by Abdessamed, also found by ABAIKUNANBAEV

## Impact

In Cairo contracts, when handling cross-chain messages or callbacks, the contract ensures that the handler on the same chain (Starknet) is whitelisted:

fn receive_cross_chain_msg (ref self: ContractState, cross_chain_msg_id: u256, from_chain: felt252, to_chain: felt252, from_handler: u256, to_handler: ContractAddress, payload: Array< u8 >) -> bool { assert (to_handler == get_contract_address (), "error to_handler" ); assert ( self.settlement_address.

read () == get_caller_address (), "not settlement" ); assert ( self.support_handler.

read ((from_chain, from_handler)) && >>> self.support_handler.

read ((to_chain, contract_address_to_u256 (to_handler))), "not support handler" ); // --SNIP } fn receive_cross_chain_callback (ref self: ContractState, cross_chain_msg_id: felt252, from_chain: felt252, to_chain: felt252, from_handler: u256, to_handler: ContractAddress, cross_chain_msg_status:

u8 ) -> bool { assert (to_handler == get_contract_address (), "error to_handler" ); assert ( self.settlement_address.

read () == get_caller_address (), "not settlement" ); assert ( self.support_handler.

read ((from_chain, from_handler)) && >>> self.support_handler.

read ((to_chain, contract_address_to_u256 (to_handler))), "not support handler" ); // --SNIP } In this context, handlers on Starknet must whitelist themselves as supported handlers. However, this introduces a significant vulnerability: since handlers are self-whitelisted, a malicious user could send cross-chain messages to the same chain (Starknet to Starknet) and exploit the BurnUnlock mode as follows:

In BurnUnlock mode, when a cross-chain message is sent, the user’s tokens are burned, and when the message is received, the same amount of tokens is unlocked. This opens the door for malicious actors to repeatedly send cross-chain messages to themselves, resulting in a continuous unlock of tokens:

fn receive_cross_chain_msg (ref self: ContractState, cross_chain_msg_id: u256, from_chain: felt252, to_chain: felt252, from_handler: u256, to_handler: ContractAddress, payload: Array< u8 >) -> bool { // --SNIP let erc20 = IERC20MintDispatcher{contract_address:

self.token_address.

read ()}; let token = IERC20Dispatcher{contract_address:

self.token_address.

read ()}; if self.mode.

read () == SettlementMode::MintBurn{ erc20.

mint_to ( u256_to_contract_address (transfer.to), transfer.amount); } else if self.mode.

read () == SettlementMode::LockMint{ erc20.

mint_to ( u256_to_contract_address (transfer.to), transfer.amount); } else if self.mode.

read () == SettlementMode::BurnUnlock{ >>> token.

transfer ( u256_to_contract_address (transfer.to), transfer.amount); } else if self.mode.

read () == SettlementMode::LockUnlock{ token.

transfer ( u256_to_contract_address (transfer.to), transfer.amount); } By continuously sending cross-chain messages to the same chain, the malicious user can drain the handler’s funds by repeatedly unlocking tokens to their own address. As a result, other legitimate cross-chain ERC20 operations will fail due to the depletion of ERC20 tokens in the handler contract.

## Recommended Mitigation Steps

The self-whitelisting of handlers introduces unnecessary risk and facilitates the aforementioned vulnerability. Consider removing the check that forces handlers to be self-whitelisted:

fn receive_cross_chain_msg(ref self: ContractState, cross_chain_msg_id: u256, from_chain: felt252, to_chain: felt252, from_handler: u256, to_handler: ContractAddress, payload: Array<u8>) -> bool{ assert(to_handler == get_contract_address(),"error to_handler"); assert(self.settlement_address.read() == get_caller_address(), "not settlement"); assert(self.support_handler.read((from_chain, from_handler)) - && self.support_handler.read((to_chain, contract_address_to_u256(to_handler))), "not support handler"); // --SNIP } fn receive_cross_chain_callback(ref self: ContractState, cross_chain_msg_id: felt252, from_chain: felt252, to_chain: felt252, from_handler: u256, to_handler: ContractAddress, cross_chain_msg_status: u8) -> bool{

assert(to_handler == get_contract_address(),"error to_handler"); assert(self.settlement_address.read() == get_caller_address(), "not settlement"); assert(self.support_handler.read((from_chain, from_handler)) - && self.support_handler.read((to_chain, contract_address_to_u256(to_handler))), "not support handler"); // --SNIP } zvlwwj (Chakra) confirmed:

0xsomeone (judge) invalidated and commented:

The Warden has attempted to formulate an exploitation path of a self-cross-chain transfer; however, the assumption that a handler can be arbitrarily introduced to the system is incorrect given that the relevant function enrolling them is owner-controlled.

Abdessamed (warden) commented:

Hi @0xsomeone - “however, the assumption that a handler can be arbitrarily introduced to the system is incorrect given that the relevant function enrolling them is owner-controlled.” This is an incorrect statement because, as seen on the receive_crosschain_msg, the function enforces that both to_chain and to_handler as well as from_chain and from_handler are supported:

assert ( self.support_handler.

read ((from_chain, from_handler)) && self.support_handler.

read ((to_chain, contract_address_to_u256 (to_handler))), "not support handler" ); So, starknet handlers must be self-whitelisted. A malicious user can exploit this for handlers having the mode BurnLock and drain the handler’s funds via sending cross-chain messages from starknet to starknet to repeatedly unlock tokens to their own address, as explained in details on the report.

0xsomeone (judge) commented:

Hey @Abdessamed, thanks for your follow-up feedback. The term “self-whitelisted” is invalid as the contracts do not expose a mechanism for users to self-whitelist themselves. As whitelisting is an authoritative process (i.e. requires elevated privileges), we can safely assume that registered handlers are trusted implementations rather than arbitrary users.

Abdessamed (warden) commented:

Hi @0xsomeone - I see where the confusion is coming from. Let me reformulate:

In this function, it checks that both to_chain and to_handler as well as from_chain and from_handler are supported:

assert ( self.support_handler.

read ((from_chain, from_handler)) && self.support_handler.

read ((to_chain, contract_address_to_u256 (to_handler))), 'not support handler'); This forces the owner to already call set_support_handler to add Starknet handlers as supported on the Starknet handler. Otherwise, any call to handler_erc20::receive_cross_chain_msg will fail.

Now, given the above, an attacker makes use of that to make the attack described on the report.

0xsomeone (judge) increased severity to High and commented:

Hey @Abdessamed, thanks for clarifying! I see what the outlined vulnerability presently is, and can confirm it is a valid issue. This is indeed a good finding! I have re-instated a high-risk rating for it, and appreciate the due diligence during the PJQA process.

To note, self-whitelisting is the act of whitelisting oneself; might be useful for future submissions!

Regarding comments about how the Chakra Network operates, in line with other rulings in the audit, we cannot make any assumptions as to what operations those nodes can and cannot filter, so we assume all operations are processed at “face-value” rendering this submission to be valid.

# [H-07] Anyone can manipulate user nonce ( nonce_manager ) in settlement contract

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Finding ID:** H-07
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-08-chakra
- **Source snapshot:** competitions/2024-08-chakra/final_report.html

nonce_manager ) in settlement contract Submitted by 13u9, also found by joicygiore ( 1, 2 ), aldarion ( 1, 2 ), Agontuk, pwnforce ( 1, 2 ), 0x37 ( 1, 2 ), Daniel_eth, Abdessamed ( 1, 2 ), kutugu, bhilare_, DPS ( 1, 2 ), jasonxiale ( 1, 2 ), Cybrid, DanielArmstrong, IzuMan, AllTooWell ( 1, 2 ), SBSecurity, Draiakoo, franfran20 ( 1, 2 ), Subroutine ( 1, 2 ), 0xabhay, 0xpetern, LuarSec, shaflow2 ( 1, 2, 3 ), klau5 ( 1, 2 ), fyamf ( 1, 2 ), gesha17, ZdravkoHr, mojito_auditor, and ABAIKUNANBAEV

## Impact

There is a problem that anyone can increase another handler contract nonce ( nonce_manager ) in settlement contract.

nonce_manager is used to make txid

- https://github.com/code-423n4/2024-08-chakra/blob/main/solidity/handler/contracts/ChakraSettlementHandler.sol#L139-L150
- https://github.com/code-423n4/2024-08-chakra/blob/main/solidity/settlement/contracts/ChakraSettlement.sol#L122-L133
{ // Increment nonce for the sender nonce_manager[msg.sender] += 1; } uint256 txid = uint256( keccak256( abi.encodePacked( chain, to_chain, msg.sender, // from address for settlement to calculate txid address(this), // from handler for settlement to calculate txid to_handler, nonce_manager[msg.sender] ) ); txid is generated using several arguments and the nonce_manager[msg.sender] value as shown above.

- https://github.com/code-423n4/2024-08-chakra/blob/main/solidity/handler/contracts/ChakraSettlementHandler.sol#L212-L222
emit CrossChainLocked( txid, msg.sender, to, chain, to_chain, address(this), to_token, amount, mode ); The handler function emits the corresponding txid value as an event.

- https://github.com/code-423n4/2024-08-chakra/blob/main/solidity/handler/contracts/ChakraSettlementHandler.sol#L203-L209
///Handler code // Send the cross chain msg settlement.send_cross_chain_msg( to_chain, msg.sender, to_handler, PayloadType.ERC20, cross_chain_msg_bytes );

- https://github.com/code-423n4/2024-08-chakra/blob/main/solidity/settlement/contracts/ChakraSettlement.sol#L118-L133
- https://github.com/code-423n4/2024-08-chakra/blob/main/solidity/settlement/contracts/ChakraSettlement.sol#L146-L155
///Settlement code nonce_manager[from_address] += 1; address from_handler = msg.sender; uint256 txid = uint256( keccak256( abi.encodePacked( contract_chain_name, // from chain to_chain, from_address, // msg.sender address from_handler, // settlement handler address to_handler, nonce_manager[from_address] ) ); emit CrossChainMsg( txid, from_address, contract_chain_name, to_chain, from_handler, to_handler, payload_type, payload ); Before the event emit, the handler function calls settlement.send_cross_chain_msg. The send_cross_chain_msg function of settlement also creates a txid and emits an event using this txid.

Through this process, the handler contract and the settlement contract emit the same txid event.

The system processes this emitted txid to handle the task.

However, there is a vulnerability that allows anyone to manipulate the nonce_manager value used when generating txid in the settlement contract with someone else’s nonce_manager value, which can cause confusion in the system.

## Recommended Mitigation Steps

Set the nonce value in the settlement contract to be managed by from_handler diff --git a/solidity/settlement/contracts/BaseSettlement.sol b/solidity/settlement/contracts/BaseSettlement.sol index 7ac72a2..bd2ce00 100644 --- a/solidity/settlement/contracts/BaseSettlement.sol +++ b/solidity/settlement/contracts/BaseSettlement.sol @@ -38,7 +38,8 @@ abstract contract BaseSettlement is ISettlementSignatureVerifier public signature_verifier; // Mapping for nonce manager and validators - mapping(address => uint256) public nonce_manager; + //mapping(address => uint256) public nonce_manager; + mapping(address => mapping(address => uint256)) public nonce_manager; mapping(address => bool) public chakra_validators;

uint256 public validator_count; diff --git a/solidity/settlement/contracts/ChakraSettlement.sol b/solidity/settlement/contracts/ChakraSettlement.sol index ad764f2..4c2b0c5 100644 --- a/solidity/settlement/contracts/ChakraSettlement.sol +++ b/solidity/settlement/contracts/ChakraSettlement.sol @@ -115,9 +115,10 @@ contract ChakraSettlement is BaseSettlement { PayloadType payload_type, bytes calldata payload ) external { - nonce_manager[from_address] += 1; + // nonce_manager[from_address] += 1; address from_handler = msg.sender; + nonce_manager[from_handler][from_address] += 1; uint256 txid = uint256( keccak256( Modify nonce_manager as in the code above so that the user’s nonce_manager can increase according to the handler.

This operation prevents an attacker from manipulating someone else’s nonce by pretending to be a handler, and also prevents nonce collisions between other normal handlers.

pidb (Chakra) confirmed 0xsomeone (judge) commented:

The issue class of this submission concerns inconsistencies with regard to the nonce values utilized by the Chakra bridge.

This submission details how it can be maliciously sabotaged, and the remaining submissions detail how the nonces can naturally deviate. I believe that these issues are identical in nature and consider all to merit a high severity rating.

# [H-08] In settlement.cairo::receive_cross_chain_msg - the payload_type can be passed by the user, confusing offchain systems

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Finding ID:** H-08
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-08-chakra
- **Source snapshot:** competitions/2024-08-chakra/final_report.html

settlement.cairo::receive_cross_chain_msg - the payload_type can be passed by the user, confusing offchain systems Submitted by 0xAsen, also found by klau5 and fyamf

## Impact

In settlement.cairo::receive_cross_chain_msg - the payload_type can be passed by the user, confusing offchain systems.

The payload_type parameter is only used to emit events so that the Chakra nodes can detect and process them.

There is no validation for it and given that it is used in an event to which off-chain systems listen to, the payload_type values will be displayed in the explorer, and there may be other extensions in the future, according to the sponsor.

This can lead to incorrect data being displayed and undefined behavior in the future.

## Recommended Mitigation Steps

Include the payload_type in the message hash generation thus making sure that it’s value cannot be altered.

zvlwwj (Chakra) disputed and commented:

Only validator can call this function.

0xsomeone (judge) increased severity to High and commented:

The Warden has identified a mechanism via which the payload_type is not properly validated as having been signed by the Chakra team, permitting cross-chain messages to be received with a different payload type than the actual one.

I believe that a severity of high is appropriate as it should (in theory) result in a transaction being processed by the Cairo code but considered unfinished by the validator system.

# [H-09] Inconsistent Handler Validation Behavior in Cairo ERC20Handler’s Cross-Chain Callback

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Finding ID:** H-09
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-08-chakra
- **Source snapshot:** competitions/2024-08-chakra/final_report.html

Submitted by 0xNirix, also found by shaflow2, Draiakoo ( 1, 2 ), said, Abdessamed, and SBSecurity

## Impact

The receive_cross_chain_callback function in the Cairo implementation of ERC20Handler uses an assert statement to validate handlers, which causes the function to revert if a handler is invalid or has been removed from the whitelist. This behavior differs from the Solidity implementation, which returns false for invalid handlers.

This vulnerability can lead to:

Transactions remaining in a Pending state indefinitely in the Cairo implementation Inconsistent transaction states across different chain implementations Potential blocking of cross-chain operations Difficulty in handling and recovering from invalid handler scenarios

## Recommended Mitigation Steps

Update the Cairo implementation to check handler validity without using assert.

zvlwwj (Chakra) confirmed 0xsomeone (judge) increased severity to High

# [H-10] ChakraSettlement.receive_cross_chain_msg and ChakraSettlement.receive_cross_chain_callback functions do not ensure that receiving ChakraSettlement contract’s contract_chain_name must match to_chain

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Finding ID:** H-10
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-08-chakra
- **Source snapshot:** competitions/2024-08-chakra/final_report.html

ChakraSettlement.receive_cross_chain_msg and ChakraSettlement.receive_cross_chain_callback functions do not ensure that receiving ChakraSettlement contract’s contract_chain_name must match to_chain corresponding to respective txid input though they should Submitted by rbserver, also found by rbserver, jasonxiale, avoloder, Moksha, Coinymous, 0xNirix, SBSecurity, Respx, Emmanuel, said, haxatron, Shaheen, mojito_auditor, fyamf, klau5, ZdravkoHr, ABAIKUNANBAEV ( 1, 2 ), and Rhaydden

## Impact

Because the ChakraSettlement.receive_cross_chain_msg function does not ensure that the receiving ChakraSettlement contract’s contract_chain_name must match the to_chain corresponding to its txid input, the signatures that should only be used for calling the ChakraSettlement.receive_cross_chain_msg function on to_chain A can also be allowed to be used on to_chain B. As a result, the handler on to_chain B can incorrectly mint or transfer tokens to recipients though such recipients should only receive such token amounts on to_chain A and not on to_chain B.

Similarly, since the ChakraSettlement.receive_cross_chain_callback function does not ensure that the receiving ChakraSettlement contract’s contract_chain_name must match the to_chain associated with its txid input, the signatures that should only be used for calling the ChakraSettlement.receive_cross_chain_callback function on to_chain A can also be allowed to be used on to_chain B. As a result, to_chain B’s handler’s receive_cross_chain_callback function logics can be incorrectly triggered, such as burning the corresponding token amount held by such handler on to_chain B under the MintBurn mode though such token amount should only be burned on to_chain A and not on to_chain B.

## Recommended Mitigation Steps

The ChakraSettlement.receive_cross_chain_msg and ChakraSettlement.receive_cross_chain_callback functions can be updated to check if the receiving ChakraSettlement contract’s contract_chain_name matches the to_chain associated with the respective txid input and revert if not matched.

0xsomeone (judge) increased severity to High and commented:

The Warden outlines that replay attacks are possible when receiving funds due to the absence of the to_chain variable’s validation. This allows the same payload signed by the Chakra validators to be replayed across multiple chains incorrectly, indicating a high-risk vulnerability.

# [H-11] There is no refund mechanism in ChakraSettlement.processCrossChainCallback or ChakraSettlementHandler.receive_cross_chain_callback function

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Finding ID:** H-11
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-08-chakra
- **Source snapshot:** competitions/2024-08-chakra/final_report.html

ChakraSettlement.processCrossChainCallback or ChakraSettlementHandler.receive_cross_chain_callback function Submitted by rbserver, also found by 0xastronatey, Abdessamed ( 1, 2 ), gajiknownnothing, abhishek_thaku_r, Drynooo ( 1, 2 ), ABAIKUNANBAEV, Inspecktor, Decap, 0xSolus, Daniel_eth, mjcpwns, 0xHelium, kutugu, Trooper ( 1, 2 ), pwnforce, blackpanther ( 1, 2 ), Brene, avoloder ( 1, 2 ), AllTooWell, bhilare_, 0xNirix, DPS, SBSecurity, Draiakoo, Audinarey, Tonchi ( 1, 2, 3 ), Tumelo_Crypto ( 1, 2 ), jesjupyter, franfran20 ( 1, 2, 3, 4 ), Respx, 0x18a6, 0xDemon, Emmanuel, Bauchibred, Breeje ( 1, 2 ), agent3bood, MrPotatoMagic, b0g0, haxatron, said, shaflow2 (

1, 2 ), mojito_auditor, klau5, fyamf, gesha17, and ZdravkoHr

## Impact

Because there is no refund mechanism in the ChakraSettlement.processCrossChainCallback or ChakraSettlementHandler.receive_cross_chain_callback function, when the cross-chain ERC20 settlement fails, such as due to that the source chain’s handler can be removed from the whitelist for the destination chain after the corresponding cross-chain message is initiated on the source chain and before such message is received on the destination chain, the ChakraSettlementHandler.cross_chain_erc20_settlement function caller cannot get back and loses the tokens that have been transferred to the source chain’s handler or burned on the source chain by such caller.

## Recommended Mitigation Steps

The ChakraSettlementHandler.receive_cross_chain_callback function can be updated to transfer the failed cross-chain ERC20 settlement’s token amount to the corresponding caller of the ChakraSettlementHandler.cross_chain_erc20_settlement function under the MintBurn, LockUnlock, or LockMint mode. If possible, such function can also be updated to mint the failed cross-chain ERC20 settlement’s token amount to the corresponding caller of the ChakraSettlementHandler.cross_chain_erc20_settlement function under the BurnUnlock mode; otherwise, if the corresponding token cannot be minted by the protocol, the protocol needs to clearly communicate with its users about the inability of refunding the failed cross-chain ERC20 settlement’s token amount under the

BurnUnlock mode.

0xsomeone (judge) commented:

The submission and all its duplicates concern various ways in which the cross-chain system of the Chakra protocol can fail transactions and does not expose a mechanism to issue refunds for those failed transactions.

This issue class is significantly large, and the lack of documentation as well as insight into the Chakra node system renders it impossible to judge fairly. I believe that a refund mechanism should be set in place, and thus consider all issues related to its missing functionality to be of high severity.

To note, I believe this is a flaw arising from the design of the system itself and thus have grouped these issues based on the design principle they are based on. The Sponsor has expressed that the callback function is not invoked in all cases and is solely invoked in the MintBurn mode, indicating that a solution based on the callback mode is not in line with the desires of the Sponsor and cannot be considered the “unopinionated way” to resolve this issue.

The lack of documentation renders granular judgments of issues outlining the absence of a refund mechanism impractical to perform as any such distinction would rely on assumptions about the system that cannot be validated.

# [H-12] Handler’s receive_cross_chain_callback() will always set the tx_status to SETTLED on source chain & burn the tokens (MintBurn Mode) even when the msg fails on destination

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Finding ID:** H-12
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-08-chakra
- **Source snapshot:** competitions/2024-08-chakra/final_report.html

receive_cross_chain_callback() will always set the tx_status to SETTLED on source chain & burn the tokens (MintBurn Mode) even when the msg fails on destination Submitted by Shaheen, also found by Tigerfrake, aldarion, 0xAsen, 0x37, ABAIKUNANBAEV ( 1, 2 ), AllTooWell, pwnforce, DPS, SBSecurity, 0xNirix, Respx, haxatron, mojito_auditor, klau5, fyamf, ZdravkoHr, and Abdessamed When a validator invokes Settlement.cairo ’s receive_cross_chain_callback(), they will pass the cross_chain_msg_status as an input and base on that the msg status should be set on the source chain. Means, if the cross_chain_msg_status is given as SUCCESS then the msg status on the source chain will be set as

CrossChainTxStatus::SETTLED and if it is not SUCCESS then it should simply set the status to CrossChainTxStatus::FAILED but in the cairo implementation, the cross_chain_msg_status is completely ignored:

fn receive_cross_chain_callback(ref self: ContractState, cross_chain_msg_id: felt252, from_chain: felt252, to_chain: felt252, from_handler: u256, to_handler: ContractAddress, cross_chain_msg_status: u8) -> bool{ assert(to_handler == get_contract_address(),'error to_handler'); assert(self.settlement_address.read() == get_caller_address(), 'not settlement'); assert(self.support_handler.read((from_chain, from_handler)) && self.support_handler.read((to_chain, contract_address_to_u256(to_handler))), 'not support handler'); let erc20 = IERC20MintDispatcher{contract_address: self.token_address.read()}; if self.mode.read() == SettlementMode::MintBurn{ erc20.burn_from(get_contract_address(), self.created_tx.read(cross_chain_msg_id).amount);

} let created_tx = self.created_tx.read(cross_chain_msg_id); ///@audit-issue M cross_chain_msg_status not checked like in the solidity instance, this will always set tx_status as settled and return true and never Failed!

self.created_tx.write(cross_chain_msg_id, CreatedCrossChainTx{ tx_id: created_tx.tx_id, from_chain: created_tx.from_chain, to_chain: created_tx.to_chain, from:created_tx.from, to:created_tx.to, from_token: created_tx.from_token, to_token: created_tx.to_token, amount: created_tx.amount, @> tx_status: CrossChainTxStatus::SETTLED }); return true; } The tx_status is directly set as SETTLED, which means it will be always get marked as SETTLED even when the Msg gets failed on the destination chain.

Also, as it doesn’t check the cross_chain_msg_status, this will always burn the tokens (when the MODE is MintBurn), which is not the case when we look at the solidity instance if self.mode.read() == SettlementMode::MintBurn{ erc20.burn_from(get_contract_address(), self.created_tx.read(cross_chain_msg_id).amount); }

## Impact

Wrong state update and tokens burning.

## Recommended Mitigation Steps

Make sure to check the given status and mark the tx_status base on that.

zvlwwj (Chakra) confirmed 0xsomeone (judge) increased severity to High and commented:

The submission details how the callback leg of a MintBurn mode bridge will always burn the tokens regardless of whether the transaction actually succeeded on the destination chain.

This indicates a high-risk issue within the codebase that would lead to fund loss if the destination chain’s execution fails.

# [H-13] The LockMint and BurnUnlock modes cannot be used

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Finding ID:** H-13
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-08-chakra
- **Source snapshot:** competitions/2024-08-chakra/final_report.html

LockMint and BurnUnlock modes cannot be used Submitted by shaflow2, also found by shaflow2 ( 1, 2 ), Abdessamed, Daniel_eth, and Emmanuel ( 1, 2 ) Lines of code

- https://github.com/code-423n4/2024-08-chakra/blob/d0d45ae1d26ca1b87034e67180fac07ce9642fd9/solidity/handler/contracts/ChakraSettlementHandler.sol#L127
- https://github.com/code-423n4/2024-08-chakra/blob/d0d45ae1d26ca1b87034e67180fac07ce9642fd9/solidity/handler/contracts/ChakraSettlementHandler.sol#L338
- https://github.com/code-423n4/2024-08-chakra/blob/d0d45ae1d26ca1b87034e67180fac07ce9642fd9/cairo/handler/src/handler_erc20.cairo#L127

## Impact

LockMint and BurnUnlock modes refer to the following:

LockMint: Tokens are locked on the source chain and minted on the target chain.

BurnUnlock: Tokens are burned on the source chain and unlocked on the target chain.

However, because the ChakraSettlementHandler protocol is responsible for both processing remote cross-chain messages and sending cross-chain messages to remote chains, a single mode setting cannot handle both tasks simultaneously. This results in one of the tasks’ functionalities being unavailable due to incorrect protocol implementation.

## Recommended Mitigation Steps

The following is the correct logic:

function receive_cross_chain_msg( uint256 /**txid */, string memory from_chain, uint256 /**from_address */, uint256 from_handler, PayloadType payload_type, bytes calldata payload, uint8 /**sign type */, bytes calldata /**signaturs */ ) external onlySettlement returns (bool) { // from_handler need in whitelist if (is_valid_handler(from_chain, from_handler) == false) { return false; } bytes calldata msg_payload = MessageV1Codec.payload(payload); require(isValidPayloadType(payload_type), "Invalid payload type"); if (payload_type == PayloadType.ERC20) { // Cross chain transfer { // Decode transfer payload ERC20TransferPayload memory transfer_payload = codec.deocde_transfer(msg_payload); if (mode == SettlementMode.MintBurn) {

_erc20_mint( AddressCast.to_address(transfer_payload.to), transfer_payload.amount ); return true; } else if (mode == SettlementMode.LockUnlock) { _erc20_unlock( AddressCast.to_address(transfer_payload.to), transfer_payload.amount ); return true; } else if (mode == SettlementMode.LockMint) { - _erc20_mint( - AddressCast.to_address(transfer_payload.to), - transfer_payload.amount - ); + _erc20_unlock( + AddressCast.to_address(transfer_payload.to), + transfer_payload.amount + ); return true; } else if (mode == SettlementMode.BurnUnlock) { - _erc20_unlock( - AddressCast.to_address(transfer_payload.to), - transfer_payload.amount - ); + _erc20_mint( + AddressCast.to_address(transfer_payload.to), + transfer_payload.amount

+ ); return true; } return false; } In Cairo:

impl ERC20HandlerImpl of IERC20Handler<ContractState> { fn receive_cross_chain_msg(ref self: ContractState, cross_chain_msg_id: u256, from_chain: felt252, to_chain: felt252, from_handler: u256, to_handler: ContractAddress, payload: Array<u8>) -> bool{ assert(to_handler == get_contract_address(),'error to_handler'); assert(self.settlement_address.read() == get_caller_address(), 'not settlement'); assert(self.support_handler.read((from_chain, from_handler)) && self.support_handler.read((to_chain, contract_address_to_u256(to_handler))), 'not support handler'); let message:Message= decode_message(payload); let payload_type = message.payload_type; assert(payload_type == PayloadType::ERC20, 'payload type not erc20');

let payload_transfer = message.payload; let transfer = decode_transfer(payload_transfer); assert(transfer.method_id == ERC20Method::TRANSFER, 'ERC20Method must TRANSFER'); let erc20 = IERC20MintDispatcher{contract_address: self.token_address.read()}; let token = IERC20Dispatcher{contract_address: self.token_address.read()}; if self.mode.read() == SettlementMode::MintBurn{ erc20.mint_to(u256_to_contract_address(transfer.to), transfer.amount); }else if self.mode.read() == SettlementMode::LockMint{ - erc20.mint_to(u256_to_contract_address(transfer.to), transfer.amount); + token.transfer(u256_to_contract_address(transfer.to), transfer.amount); }else if self.mode.read() == SettlementMode::BurnUnlock{

- token.transfer(u256_to_contract_address(transfer.to), transfer.amount); + erc20.mint_to(u256_to_contract_address(transfer.to), transfer.amount); }else if self.mode.read() == SettlementMode::LockUnlock{ token.transfer(u256_to_contract_address(transfer.to), transfer.amount); } return true; } pidb (Chakra) confirmed 0xsomeone (judge) commented:

The Warden and its duplicates outline a design-related flaw that is shared across the Solidity and Cairo implementations in relation to how the LockMint and BurnUnlock systems are defined, disallowing certain configurations from ever functioning properly. These configurations are expected to be a normal likelihood event due to the system’s intents to be deployed across multiple chains, rendering this submission and its duplicates to be proper high-risk criticism of the system’s design.

# [H-14] Malicious actors can manipulate the cross_chain_callback callback

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Finding ID:** H-14
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-08-chakra
- **Source snapshot:** competitions/2024-08-chakra/final_report.html

cross_chain_callback callback Submitted by shaflow2, also found by Abdessamed, gajiknownnothing, Drynooo, 0x37, 0xAsen, Audinarey, Draiakoo, b0g0, agent3bood, MrPotatoMagic, fyamf, gesha17, and SBSecurity

## Impact

Because the validator’s signature message does not include the from_chain field, a malicious actor can preemptively execute the validator’s transaction with an incorrect from_chain field. This can cause the status of create_cross_txs[txid] to become a Failed state, preventing it from being executed again.

## Recommended Mitigation Steps

function receive_cross_chain_callback( uint256 txid, string memory from_chain, uint256 from_handler, address to_handler, CrossChainMsgStatus status, uint8 sign_type, bytes calldata signatures ) external { verifySignature( txid, + from_chain from_handler, to_handler, status, sign_type, signatures );...

} function verifySignature( uint256 txid, + string memory from_chain, uint256 from_handler, address to_handler, CrossChainMsgStatus status, uint8 sign_type, bytes calldata signatures ) internal view { bytes32 message_hash = keccak256( - abi.encodePacked(txid, from_handler, to_handler, status) + abi.encodePacked(txid, from_chain, to_handler, status) ); require( signature_verifier.verify(message_hash, signatures, sign_type), "Invalid signature" ); } pidb (Chakra) disputed and commented:

This from_chain parameter is not important, as long as you ensure that the txid unique signature cannot be constructed repeatedly, currently txid satisfies this.

I think it should be downgraded to Medium risk, because txid can prevent it.

0xsomeone (judge) commented:

The Warden has identified a mechanism via which the third leg of a transaction can be blocked via a front-running attack, causing the transaction to indicate a failure status even though it would have normally been executed otherwise.

I consider this to be a valid high-severity issue as the status of all transactions can be trivially sabotaged to a failure state during their final callback leg.

Medium Risk Findings (12)

# [M-01] Excessive Authority Granted to Managers in the ckr_btc.cairo Contract Presents Significant Management Risks

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-chakra
- **Source snapshot:** competitions/2024-08-chakra/final_report.html

ckr_btc.cairo Contract Presents Significant Management Risks Submitted by joicygiore, also found by Abdessamed, Tigerfrake, m4k2, SBSecurity, devival, haxatron, said, fyamf, and peanuts In the ckr_btc.cairo contract, managers possess broad authority and can execute all functions except upgrade(). This level of access contrasts with typical Solidity practices, where such functions would require owner privileges.

One of the primary concerns is that any existing manager has the ability to add new managers. This unchecked ability to expand the number of managers can lead to an uncontrolled increase in the number of authorized managers, creating a potential security risk. Without additional checks or approval mechanisms, this design flaw could be exploited, leading to a dilution of control and increased risk of unauthorized actions.

Furthermore, managers also have the ability to designate operator roles. Operators, in turn, have the authority to mint and burn tokens, which compounds the risk if these roles are not properly managed. This level of access could easily lead to a situation where control over critical functions is decentralized in a manner that might not have been intended by the original design.

It is unclear whether this design was intentional, but it introduces significant management risks that should be addressed.

fn add_manager ( ref self:

ContractState, new_manager:

ContractAddress ) -> bool { let caller = get_caller_address (); @> assert ( self.

chakra_managers.

read ( caller ) == 1, Errors::

NOT_MANAGER ); assert ( self.

chakra_managers.

read ( new_manager ) == 0, Errors::

ALREADY_MANAGER ); self.

chakra_managers.

write ( new_manager, 1 ); self.

emit ( ManagerAdded { operator:

caller, new_manager:

new_manager, added_at:

get_block_timestamp () } ); return self.

chakra_managers.

read ( new_manager ) == 1; } fn add_operator ( ref self:

ContractState, new_operator:

ContractAddress ) -> bool { let caller = get_caller_address (); @> assert ( self.

chakra_managers.

read ( caller ) == 1, Errors::

NOT_MANAGER ); assert ( self.

chakra_operators.

read ( new_operator ) == 0, Errors::

ALREADY_OPERATOR ); self.

chakra_operators.

write ( new_operator, 1 ); self.

emit ( OperatorAdded { new_operator:

new_operator, added_at:

get_block_timestamp () } ); return self.

chakra_operators.

read ( new_operator ) == 1; } fn mint_to ( ref self:

ContractState, to:

ContractAddress, amount:

u256 ) -> bool { let caller = get_caller_address (); @> assert ( self.

chakra_operators.

read ( caller ) == 1, Errors::

NOT_OPERATOR ); let old_balance = self.

erc20.

balance_of ( to ); self.

erc20.

mint ( to, amount ); let new_balance = self.

erc20.

balance_of ( to ); return new_balance == old_balance + amount; }

## Impact

The excessive authority granted to managers poses a significant management risk. The potential for an uncontrolled expansion of manager roles, coupled with the ability to assign operator roles, could lead to serious security vulnerabilities, including unauthorized minting or burning of tokens.

## Recommended Mitigation Steps

It is recommended to follow best practices as implemented in Solidity contracts, where only the owner has the authority to add or remove managers and operators. This can be achieved by modifying the current implementation to enforce owner checks when adding new managers or operators.

Example of a safer approach:

fn add_manager(ref self: ContractState, new_manager: ContractAddress) -> bool { let caller = get_caller_address(); - assert(self.chakra_managers.read(caller) == 1, Errors::NOT_MANAGER); + self.ownable.assert_only_owner(); assert(self.chakra_managers.read(new_manager) == 0, Errors::ALREADY_MANAGER); self.chakra_managers.write(new_manager, 1); self.emit( ManagerAdded { operator: caller, new_manager: new_manager, added_at: get_block_timestamp() } ); return self.chakra_managers.read(new_manager) == 1; } fn add_operator(ref self: ContractState, new_operator: ContractAddress) -> bool { let caller = get_caller_address(); - assert(self.chakra_managers.read(caller) == 1, Errors::NOT_MANAGER); + self.ownable.assert_only_owner();

assert(self.chakra_operators.read(new_operator) == 0, Errors::ALREADY_OPERATOR); self.chakra_operators.write(new_operator, 1); self.emit( OperatorAdded { new_operator: new_operator, added_at: get_block_timestamp() } ); return self.chakra_operators.read(new_operator) == 1; } zvlwwj (Chakra) confirmed via duplicate issue #107 0xsomeone (judge) commented:

The Warden and its duplicates have identified a potential privilege escalation exploitation path whereby a single manager can overtake the system by removing all others and then introducing malicious variants of themselves. I believe that this breaches the documentation of the system and is a form of privilege escalation as managers should collectively authorize operations.

# [M-02] Missing ERC20Method validation at destination allows non-transfer tx to be handled as transfers

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-chakra
- **Source snapshot:** competitions/2024-08-chakra/final_report.html

ERC20Method validation at destination allows non-transfer tx to be handled as transfers Submitted by Tigerfrake, also found by said

## Impact

In cross-chain transactions, the integrity and correctness of the transaction must be maintained across different blockchain environments, which might have varying security measures, protocols. However, the missing ERC20method validation at destination allows cross-chain tx with a different ERC20Method from Transfer to be handled as transfer transactions when they actually aren’t.

## Recommended Mitigation Steps

ERC20Method should be validated before handling tokens in receive_cross_chain_msg():

if (payload_type == PayloadType.ERC20) { // Cross chain transfer { // Decode transfer payload ERC20TransferPayload memory transfer_payload = codec.deocde_transfer(msg_payload); + require(transfer_payload.method_id == ERC20Method.Transfer, "ERC20Method must TRANSFER"); if (mode == SettlementMode.MintBurn) { _erc20_mint( AddressCast.to_address(transfer_payload.to), transfer_payload.amount ); return true; } ---SNIP--- } ---SNIP--- } 0xsomeone (judge) commented:

The Solidity counterpart of the Chakra system does not validate that the decoded EIP-20 operation is an ERC20Method::Transfer and ignores its status.

As the Chakra node system is out of the scope of this audit, I am unable to properly deduce the likelihood as well as the feasibility of this variable being any different.

I consider this submission to be a correct medium-risk issue as a result given that operations other than transfers would be assumed to be transfers.

# [M-03] Inconsistency in sender address when creating cross chain messages on Starknet can lead to loss of funds

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-chakra
- **Source snapshot:** competitions/2024-08-chakra/final_report.html

Submitted by Audinarey, also found by AllTooWell, said, fyamf, SBSecurity, b0g0, Tigerfrake, mojito_auditor, Draiakoo, and jasonxiale Lines of code

- https://github.com/code-423n4/2024-08-chakra/blob/d0d45ae1d26ca1b87034e67180fac07ce9642fd9/cairo/handler/src/handler_erc20.cairo#L188
- https://github.com/code-423n4/2024-08-chakra/blob/d0d45ae1d26ca1b87034e67180fac07ce9642fd9/cairo/handler/src/handler_erc20.cairo#L222

## Impact

Funds could get stuck in the handler_erc20.cairo contract.

Validators can evaluate transactions wrongly.

## Recommended Mitigation Steps

Modify the handler_erc20::cross_chain_erc20_settlement(...) function as shown below:

File: handler_erc20.cairo 167:

fn cross_chain_erc20_settlement (ref self: ContractState, to_chain: felt252, to_handler: u256, to_token: u256, to: u256, amount: u256) -> felt252{ 168:

assert ( self.support_handler.

read ((to_chain, to_handler)), 'not support handler'); 169:

let settlement = IChakraSettlementDispatcher {contract_address:

self.settlement_address.

read ()}; 170:

let from_chain = settlement.

chain_name (); 171:

let token = IERC20Dispatcher{contract_address:

self.token_address.

read ()}; 172:

let token_burnable = IERC20MintDispatcher{contract_address:

self.token_address.

read ()}; 173:

if self.mode.

read () == SettlementMode::MintBurn{ 174: token.

transfer_from ( get_caller_address (), get_contract_address (), amount); SNIP.......

182:

183:

let tx_id = LegacyHash::

hash ( get_tx_info ().

unbox ().transaction_hash, self.msg_count.

read ()); 184:

let tx: CreatedCrossChainTx = CreatedCrossChainTx{ 185: tx_id: tx_id, 186: from_chain: from_chain, 187: to_chain: to_chain, - 188: from:

get_contract_address (), + 188: from:

get_caller_address (), 189: to: to, 190: from_token:

self.token_address.

read (), 191: to_token: to_token, 192: amount: amount, 193: tx_status: CrossChainTxStatus::PENDING 194: }; SNIP.......

219:

self.

emit ( 220: CrossChainLocked{ 221: tx_id: tx_id, 222: from:

get_caller_address (), 223: to: to, 224: from_chain:

get_tx_info ().

unbox ().chain_id, 225: to_chain: to_chain, 226: from_token:

self.token_address.

read (), 227: to_token: to_token, 228: amount: amount 229: } 230: ); 231:

return tx_id; 232: } zvlwwj (Chakra) confirmed via duplicate issue #174 0xsomeone (judge) commented:

The Warden has identified an inconsistency in the way cross-chain message events are emitted and stored when the handler_erc20::cross_chain_erc20_settlement function is invoked.

Given that the documentation of the project is inadequate to properly assess the severity of the event emissions, I believe that estimating this discrepancy as a high-severity issue is appropriate given that events are usually imperative to the proper function of cross-chain bridges.

Abdessamed (warden) commented:

This is QA. There is indeed a wrong from set on the created struct, BUT, this information is not used anywhere else that could result in meaningful impact because:

The tokens are correctly locked or burned from the caller, and the Chakra nodes rely on the to argument to mint/unlock the tokens on the destination chain.

0xsomeone (judge) decreased severity to Medium and commented:

After re-evaluating this particular submission, I am inclined to downgrade it to medium severity.

I agree with @Abdessamed in that the event is actually emitted with the correct parameters but the struct is stored incorrectly which would normally make it QA, however, we have no way of knowing (nor can we make any assumptions about) how the nodes operate.

At the very least, off-chain software that attempts to showcase which transactions a user has made will fail to do so properly. In a worst-case scenario, failed transaction refunds (which we all know are nowhere implemented within the code but could theoretically be handled off-chain) would fail to be credited to the correct recipient. In a worst-case scenario, all transactions could fail to be processed due to the off-chain software attempting to validate the event’s data with the data stored on the chain.

I personally believe that the code of the Chakra nodes would hint toward a QA severity rating for this submission, however, I am inclined to consider it a medium-risk issue due to the known impact outlined above and the unknown possibility of a higher severity impact.

# [M-04] Incorrect Decimals Setting for ckrBTC Token May Lead to User Confusion and Inaccurate Transaction Amounts

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-chakra
- **Source snapshot:** competitions/2024-08-chakra/final_report.html

Submitted by Xor0v0, also found by AllTooWell and Abdessamed

## Impact

In the OpenZeppelin library, the decimals of the ERC20 component defaults to 18, which is abnormal for BTC. Incorrect decimal settings may result in users entering incorrect amounts or understanding incorrect balances during transactions. This confusion may lead to property damage or transaction failure, especially when the transaction amount is large.

On the other hand, many tools, applications, or smart contracts related to Bitcoin may rely on an accuracy of 8 decimal places. If the accuracy setting is incorrect, these tools or contracts may not work properly, thereby affecting the operation of the entire ecosystem.

## Recommended Mitigation Steps

Rewrite the decimals to 8 for the ckrBTC token.

zvlwwj (Chakra) confirmed 0xsomeone (judge) commented:

The Warden has identified that the ckrBTC token on the Starknet network has a different number of decimals than on other networks, causing the ckrBTC variant of the Starknet network to be consistently overvalued.

I do not envision any impact from this discrepancy apart from the value per whole unit of ckrBTC token on the Starknet network being higher than the value per whole unit of ckrBTC token on other networks. As such, I believe a medium-risk rating is acceptable.

# [M-05] Settlement contract is mistakenly used for the handler contract when assigning ReceivedCrossChainTx struct

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-chakra
- **Source snapshot:** competitions/2024-08-chakra/final_report.html

ReceivedCrossChainTx struct Submitted by ABAIKUNANBAEV, also found by AllTooWell, rbserver, said, Abdessamed, Draiakoo, and 0xNirix

## Impact

receive_cross_chain_msg() is located in the Settlement contract and is called after the Chakra network signature verification process. However, when creating ReceivedCrossChainTx struct for the new receive_cross_txs[txid], the contract mistakenly uses address(this) instead of to_handler address.

## Recommended Mitigation Steps

Change address(this) on to_handler address in a receive_cross_txs[txid] mapping.

pidb (Chakra) confirmed 0xsomeone (judge) commented:

The Warden has identified that the ChakraSettlement::receive_cross_chain_msg function will improperly store a cross-chain message and will specifically store the to_handler address incorrectly.

The impact of this particular submission is not able to be identified properly and, given that the function appears to be invoked as the last step in a cross-chain process, a severity of medium is considered appropriate as it should not impact the actual processing of the cross-chain transaction.

# [M-06] A cross-chain message can be initiated with invalid parameters

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-chakra
- **Source snapshot:** competitions/2024-08-chakra/final_report.html

Submitted by Tigerfrake, also found by SBSecurity, 0xAsen, blackpanther, Abdessamed, 0xb0k0, avoloder, DPS, agent3bood, ABAIKUNANBAEV, peanuts, calfun0x, ZdravkoHr, 0xNirix, and haxatron

## Impact

Anyone can initiate a cross-chain ERC20 settlement with invalid parameters, breaking an invariant due to missing validations.

For example if these values are non-existent or zero values, it would boil down to the following:

zero amount ---> Only wastes validators resources by signing the msgHash at destination only for zero tokens to be minted/unlocked to the recipient zero t0 ---> If this translates to address(0), the transfer mechanism used during unlocking will revert zero to_token ---> Recipients will expect to receive tokens at destination but if this token specified is non-existent, they wont receive any.

## Recommended Mitigation Steps

Add the following checks to the cross_chain_erc20_settlement() function in handler_erc20:

+ assert(amount > 0, 'Amount must be greater than 0'); + assert(to != 0, 'Invalid to address'); + assert(to_handler != 0, 'Invalid to handler address'); + assert(to_token != 0, 'Invalid to token address'); zvlwwj (Chakra) confirmed 0xsomeone (judge) commented:

The Warden has identified that the Cairo implementation of the cross-chain EIP-20 settlement function deviates from its specification and Solidity counterpart in the way it validates several input arguments.

I believe a medium-risk severity rating is appropriate given that no substantial impact has been demonstrated beyond a specification deviation.

# [M-07] Permanent loss of user tokens on both chains if BurnUnlock mode fails because of flawed burning pattern

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-chakra
- **Source snapshot:** competitions/2024-08-chakra/final_report.html

BurnUnlock mode fails because of flawed burning pattern Submitted by 0x18a6, also found by Abdessamed, mjcpwns, and jesjupyter

## Impact

The BurnUnlock mode in the ChakraSettlementHandler contract has a critical flaw that causes token loss on both chains. This happens because tokens are burned on the source chain before ensuring a successful unlock on the destination chain. This high-risk vulnerability can lead to significant financial loss and damage trust in the cross-chain bridge system.

Note that the same issue persists in the Cairo code.

## Vulnerability Details

In the ChakraSettlementHandler contract, we got two modes that burns tokens in the bridging process:

MintBurn: Tokens are burned on the source chain and minted on the destination chain. This mode is used when you want to permenantly move tokens from one chain to another.

BurnUnlock: Tokens are burned on the source chain and unlocked on the destination chain. This mode is used when you want to move tokens from one chain to another and decrease the supply on the source chain.

We observe two distinct pattern of burning tokens in MintBurn vs BurnUnlock modes:

MintBurn mode Source chain ( cross_chain_erc20_settlement ): Tokens are locked instead of burned initially.

solidity/handler/contracts/ChakraSettlementHandler.sol#L123-L124 if ( mode == SettlementMode.

MintBurn ) { _erc20_lock ( msg.

sender, address ( this ), amount ); } Destination chain ( receive_cross_chain_msg ): Tokens are correctly minted on the destination chain. ( the Cairo code is equivalent to this ) solidity/handler/contracts/ChakraSettlementHandler.sol#L325-L329 if ( mode == SettlementMode.

MintBurn ) { _erc20_mint ( AddressCast.

to_address ( transfer_payload.

to ), transfer_payload.

amount ); return true; } Callaback on the source chain: Tokens are burned only after a successful callback. This is a security measure to ensure that user tokens are only burned when the user has received their tokens on the destination chain. If tokens were burned initially and then the call on the destination chain failed, the user’s tokens would effectively be lost on both chains. More details on this will be provided later:

solidity/handler/contracts/ChakraSettlementHandler.sol#L383-L389 if ( status == CrossChainMsgStatus.

Success ) { if ( mode == SettlementMode.

MintBurn ) { _erc20_burn ( address ( this ), create_cross_txs [ txid ].

amount ); } create_cross_txs [ txid ].

status = CrossChainTxStatus.

Settled; } BurnUnlock mode Source chain ( cross_chain_erc20_settlement ): Tokens are immediately burned on the source chain.

solidity/handler/contracts/ChakraSettlementHandler.sol#L129-L130 if ( mode == SettlementMode.

BurnUnlock ) { _erc20_burn ( msg.

sender, amount ); } Destination chain ( receive_cross_chain_msg ): Tokens are correctly unlocked solidity/handler/contracts/ChakraSettlementHandler.sol#L344-L348 if ( mode == SettlementMode.

BurnUnlock ) { _erc20_unlock ( AddressCast.

to_address ( transfer_payload.

to ), transfer_payload.

amount ); return true; } Callback on the source chain: No specific action for BurnUnlock mode in callback.

To summarize, the two distinct pattern for burning tokens is that:

MintBurn follows a lock (source) -> mint (dest) -> burn (callback) and, BurnUnlock follows a burn (source) -> unlock (dest).

The problem is that in the BurnUnlock mode, if the execution on the destination chain fails (for instance, due to insufficient validator signatures), the user’s tokens are lost on both chains. This is because the tokens are burned on the source chain and remain locked on the destination chain, preventing the user from reclaiming them at first try, and preventing the user from reclaiming them at a latter try because user already burned their equivalant tokens in the source chain.

Note that the problem doesn’t exist in MintBurn mode (at least with this particular root cause) because we use the callback pattern. If the call to destination chain fails, the callback simply unlocks the tokens back on the source chain (which it doesn’t do in the current implementation, but is a subject of a distinct report since it’s a different root cause i.e, not implementing a callback pattern if bridging fails vs not unlocking the tokens back in the callback pattern if bridging fails).

## Recommended Mitigation Steps

In my opinion, align the BurnUnlock mode with the MintBurn mode by implementing a lock-then-burn pattern:

function cross_chain_erc20_settlement( string memory to_chain, uint256 to_handler, uint256 to_token, uint256 to, uint256 amount ) external { //...

if (mode == SettlementMode.BurnUnlock) { + _erc20_lock(msg.sender, address(this), amount); - _erc20_burn(msg.sender, address(this), amount); } //...

} Implement the callback part, and don’t forget to unlock tokens back to the poor Alice on failure:

function receive_cross_chain_callback( uint256 txid, string memory from_chain, uint256 from_handler, CrossChainMsgStatus status, uint8 /* sign_type */, bytes calldata /* signatures */ ) external onlySettlement returns (bool) { //...

if (status == CrossChainMsgStatus.Success) { + if (mode == SettlementMode.BurnUnlock) { + _erc20_burn(address(this), create_cross_txs[txid].amount); + } create_cross_txs[txid].status = CrossChainTxStatus.Settled; if (status == CrossChainMsgStatus.Failed) { + if (mode == SettlementMode.BurnUnlock) { + _erc20_unlock(create_cross_txs[txid].from, create_cross_txs[txid].amount); + } create_cross_txs[txid].status = CrossChainTxStatus.Failed; } //...

} pidb (Chakra) disputed and commented:

Callback only works in MintBurn mode. For BurnUnLock, the token will be burned in the destination chain, so the suggestion here is wrong.

0xsomeone (judge) decreased severity to Medium and commented:

The Warden and its duplicates have identified that the BurnUnlock mode is presently insecurely implemented, burning tokens prior to receiving confirmation that they have been unlocked on the destination chain.

I do not agree with the Sponsor that the callback does not work on the BurnUnlock mode as support for it should be introduced and the node system is out-of-scope of the audit. I believe a medium-risk rating is appropriate given that this issue should solely manifest in case the cross-chain transactions fail for one reason or another.

# [M-08] The receive_cross_chain_msg function has potential replay attack risks

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Finding ID:** M-08
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-chakra
- **Source snapshot:** competitions/2024-08-chakra/final_report.html

receive_cross_chain_msg function has potential replay attack risks Submitted by shaflow2

## Impact

The receive_cross_chain_msg function does not check the execution status of cross-chain messages itself and relies entirely on the checks performed by the settlement contract. If the settlement contract is updated without changes to the validators, this could lead to a risk of previously executed transactions being replayed.

## Recommended Mitigation Steps

Make the settlement_address unchange.

- fn upgrade_settlement(ref self:ContractState, new_settlement: ContractAddress){ - self.ownable.assert_only_owner(); - self.settlement_address.write(new_settlement); - } The handler stores and verifies the state of cross-chain messages.

#[storage] struct Storage { #[substorage(v0)] ownable: OwnableComponent::Storage, #[substorage(v0)] upgradeable: UpgradeableComponent::Storage, settlement_address: ContractAddress, token_address: ContractAddress, created_tx: LegacyMap<felt252, CreatedCrossChainTx>, + receive_tx: LegacyMap<felt252, CrossChainMsgStatus>, msg_count: u256, support_handler: LegacyMap<(felt252, u256), bool>, mode: u8 } fn receive_cross_chain_msg(ref self: ContractState, cross_chain_msg_id: u256, from_chain: felt252, to_chain: felt252, from_handler: u256, to_handler: ContractAddress, payload: Array<u8>) -> bool{ assert(to_handler == get_contract_address(),'error to_handler'); assert(self.settlement_address.read() == get_caller_address(), 'not settlement');

assert(self.support_handler.read((from_chain, from_handler)) && self.support_handler.read((to_chain, contract_address_to_u256(to_handler))), 'not support handler'); + assert(self.receive_tx.read(cross_chain_msg_id) == CrossChainMsgStatus::UNKNOW, 'tx status error');...

+ self.receive_tx.wirte(cross_chain_msg_id, CrossChainMsgStatus::SUCCESS); return true; } zvlwwj (Chakra) confirmed 0xsomeone (judge) commented:

The submission outlines a potential issue that might arise in an upgrade flow of the system if the validators are not changed. I believe such a scenario is likely, and thus consider this to be an acceptable medium-risk vulnerability as transaction replays would be possible in the edge case described.

0xsomeone (judge) commented:

Just to clarify, the upgrade outlined in this submission is an address replacement, not an actual code upgrade. As such, storage will be affected (i.e. the settlement address will have a fresh storage set) and thus this vulnerability is applicable. Alleviating an issue in the settlement implementation will not affect the handler itself as the handler would be pointing to a new settlement implementation if the function outlined by the exhibit is invoked.

# [M-09] Bridging from Starknet to Starknet causes mismatch between minted ckrBTC and BTC transferred to MuSig2

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Finding ID:** M-09
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-chakra
- **Source snapshot:** competitions/2024-08-chakra/final_report.html

Submitted by fyamf

## Impact

It is possible to bridge assets from Starknet to Starknet by setting to_chain and to_handler to be the same as from_chain and from_handler. This violates the core rule of the protocol, which is “The contract maintains a consistent state between locking/burning tokens on the source chain and minting/unlocking on the destination chain, depending on the settlement mode.” For example, this could result in the total supply of ckrBTC on Starknet being higher than the amount of BTC transferred to MuSig2 on the Bitcoin network.

## Recommended Mitigation Steps

When bridging from Starknet, it should ensure that the source and destination are different.

0xsomeone (judge) decreased severity to Medium and commented:

The Warden has identified a self-cross-chain-transfer exploitation path that will cause the ckrBTC token to have an incorrect total supply.

I believe a medium-risk severity level is better suited as no financial value can be exploited via the vulnerability described. I do commend the ingenuity!

# [M-10] Does not check if to_chain and to_handler is whitelisted in cross_chain_erc20_settlement

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Finding ID:** M-10
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-chakra
- **Source snapshot:** competitions/2024-08-chakra/final_report.html

to_chain and to_handler is whitelisted in cross_chain_erc20_settlement Submitted by klau5, also found by Moksha, fyamf, NexusAudits, Abdessamed, gajiknownnothing, Tigerfrake, ABAIKUNANBAEV, 0xAsen, pwnforce, Daniel_eth, atoko, avoloder, AllTooWell, 0xNirix, SBSecurity, Draiakoo, peanuts, rbserver, haxatron, said, mojito_auditor, shaflow2, bhilare_, kutugu, devival, Shaheen, and Breeje

## Impact

The handler does not prevent tokens from being sent to incorrect chains and handlers. If a message is sent to an incorrect handler, the failure cannot be handled through receive_cross_chain_callback.

## Recommended Mitigation Steps

function cross_chain_erc20_settlement( string memory to_chain, uint256 to_handler, uint256 to_token, uint256 to, uint256 amount ) external { require(amount > 0, "Amount must be greater than 0"); require(to != 0, "Invalid to address"); require(to_handler != 0, "Invalid to handler address"); require(to_token != 0, "Invalid to token address"); + require(is_valid_handler(to_chain, to_handler), "not valid destination"); if (mode == SettlementMode.MintBurn) { _erc20_lock(msg.sender, address(this), amount); } else if (mode == SettlementMode.LockUnlock) { _erc20_lock(msg.sender, address(this), amount); } else if (mode == SettlementMode.LockMint) { _erc20_lock(msg.sender, address(this), amount); } else if (mode == SettlementMode.BurnUnlock) {

_erc20_burn(msg.sender, amount); }...

} 0xsomeone (judge) commented:

The submission and its duplicates detail how a cross-chain message does not validate that the to_handler is a valid address on the to_chain, causing transactions to be permanently stuck in a pending state and thus leading to fund loss.

I believe a medium-severity rating is appropriate given that the user would have to make a mistake when submitting their transaction, however, the documentation of the system and Cairo implementation indicate that validation should occur.

# [M-11] Wrong usage of transaction originator address instead of caller address

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Finding ID:** M-11
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-chakra
- **Source snapshot:** competitions/2024-08-chakra/final_report.html

Submitted by fyamf, also found by AllTooWell, haxatron, ABAIKUNANBAEV, 0xb0k0, calc1f4r ( 1, 2 ), and Draiakoo

## Impact

The from_address parameter in the events emitted when sending a cross-chain message from Starknet is set to the transaction origin ( tx.origin ) instead of the actual caller ( msg.sender ). This causes incorrect data to be emitted in the event.

## Recommended Mitigation Steps

Following modifications are recommended:

fn cross_chain_erc20_settlement(ref self: ContractState, to_chain: felt252, to_handler: u256, to_token: u256, to: u256, amount: u256) -> felt252{ assert(self.support_handler.read((to_chain, to_handler)), 'not support handler'); let settlement = IChakraSettlementDispatcher {contract_address: self.settlement_address.read()}; let from_chain = settlement.chain_name(); let token = IERC20Dispatcher{contract_address: self.token_address.read()}; let token_burnable = IERC20MintDispatcher{contract_address: self.token_address.read()}; if self.mode.read() == SettlementMode::MintBurn{ token.transfer_from(get_caller_address(), get_contract_address(), amount); }else if self.mode.read() == SettlementMode::LockMint{

token.transfer_from(get_caller_address(), get_contract_address(), amount); }else if self.mode.read() == SettlementMode::BurnUnlock{ token_burnable.burn_from(get_caller_address(), amount); }else if self.mode.read() == SettlementMode::LockUnlock{ token.transfer_from(get_caller_address(), get_contract_address(), amount); } let tx_id = LegacyHash::hash(get_tx_info().unbox().transaction_hash, self.msg_count.read()); let tx: CreatedCrossChainTx = CreatedCrossChainTx{ tx_id: tx_id, from_chain: from_chain, to_chain: to_chain, from: get_contract_address(), to: to, from_token: self.token_address.read(), to_token: to_token, amount: amount, tx_status: CrossChainTxStatus::PENDING }; self.created_tx.write(tx_id, tx);

self.msg_count.write(self.msg_count.read()+1); // message id = message count let message_id = tx_id; let transfer = ERC20Transfer{ method_id: 1, from: contract_address_to_u256(get_caller_address()), to: to, from_token: contract_address_to_u256(self.token_address.read()), to_token: to_token, amount: amount }; let message = Message{ version: 1, message_id: message_id.into(), payload_type: PayloadType::ERC20, payload: encode_transfer(transfer), }; // send cross chain msg - settlement.send_cross_chain_msg(to_chain, to_handler, PayloadType::ERC20, encode_message(message)); + settlement.send_cross_chain_msg(to_chain, to_handler, PayloadType::ERC20, encode_message(message), + get_caller_address()); // emit CrossChainLocked

self.emit( CrossChainLocked{ tx_id: tx_id, from: get_caller_address(), to: to, from_chain: get_tx_info().unbox().chain_id, to_chain: to_chain, from_token: self.token_address.read(), to_token: to_token, amount: amount } ); return tx_id; } fn send_cross_chain_msg( ref self: ContractState, to_chain: felt252, to_handler: u256, payload_type:u8,payload: Array<u8>, + from_address: felt252, ) -> felt252 { let from_handler = get_caller_address(); let from_chain = self.chain_name.read(); let cross_chain_settlement_id = LegacyHash::hash(get_tx_info().unbox().transaction_hash, self.tx_count.read()); self.created_tx.write(cross_chain_settlement_id, CreatedTx{ tx_id:cross_chain_settlement_id, tx_status: CrossChainMsgStatus::PENDING,

from_chain: from_chain, to_chain: to_chain, from_handler: from_handler, to_handler: to_handler }); self.emit( CrossChainMsg { cross_chain_settlement_id: cross_chain_settlement_id, - from_address: get_tx_info().unbox().account_contract_address, + from_address: from_address, from_chain: from_chain, to_chain: to_chain, from_handler: from_handler, to_handler: to_handler, payload_type: payload_type, payload: payload } ); self.tx_count.write(self.tx_count.read()+1); return cross_chain_settlement_id; }

- https://github.com/code-423n4/2024-08-chakra/blob/main/cairo/handler/src/settlement.cairo#L284
zvlwwj (Chakra) confirmed 0xsomeone (judge) commented:

The Warden has outlined how the system will utilize the tx.origin when processing cross-chain transaction creations which is incorrect. I believe that this does not result in a material vulnerability and is instead an incorrect system implementation meriting a medium-severity rating.

# [M-12] SettlementSignatureVerifier’s required_validators is not updated, resulting in a low or high number of signatures being required

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Finding ID:** M-12
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-chakra
- **Source snapshot:** competitions/2024-08-chakra/final_report.html

required_validators is not updated, resulting in a low or high number of signatures being required Submitted by klau5, also found by 0x37, Topmark, Draiakoo, jesjupyter, rbserver, gesha17, Breeje, SBSecurity, and 0xNirix

## Impact

The required_validators in SettlementSignatureVerifier is not updated when updating BaseSettlement ‘s. This results in either fewer or more signatures being required when verified.

## Recommended Mitigation Steps

Call SettlementSignatureVerifier.set_required_validators_num at BaseSettlement.set_required_validators_num.

function set_required_validators_num( uint256 _required_validators ) external onlyRole(MANAGER_ROLE) { + signature_verifier.set_required_validators_num(_required_validators); uint256 old = required_validators; required_validators = _required_validators; emit RequiredValidatorsChanged(msg.sender, old, required_validators); } pidb (Chakra) disputed 0xsomeone (judge) commented:

The Warden and its duplicates have outlined how the BaseSettlement::set_required_validators_num function fails to take effect as expected, causing it to be ineffectual in relation to the actual signature validation performed within the SettlementSignatureVerifier contract.

I believe a medium-risk rating is appropriate given that functionality of the system is missing but no exploitable attack path has been defined.
