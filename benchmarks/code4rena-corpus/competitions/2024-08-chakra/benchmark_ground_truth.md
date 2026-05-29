# Benchmark Ground Truth: Chakra

## Accepted H/M Findings

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

## Rejected Primary Findings

# Rejected Primary Findings: Chakra

# The cuurent implementation of the bridge is incompatible with Starknet's unique L1 <-> L2 messaging mechanism

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-641
- **Submitter:** 0x18a6
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/641
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-641.md

## Brief Summary

The current implementation of the bridge is incompatible with Starknet's unique L1 <-> L2 messaging mechanism. This incompatibility renders the bridge non-functional on Starknet.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Callback won't be processed if the mode is MintBurn because of incorrect use of _burnFrom

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-247
- **Submitter:** 0xAsen
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/247
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-247.md

## Brief Summary

Callback won't be able to be processed if the mode is MintBurn because of an incorrect use of _burnFrom. This is because when the mode is `MintBurn`, the contract will try to burn tokens from itself via `_burnFrom`. However, even though the contract burns tokens from itself, it still needs allowance.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_11_group

# Malicious managers can add and remove validators allowing them to validate malicious transactions

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-460
- **Submitter:** 0xAsen
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/460
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-460.md

## Brief Summary

Malicious managers can add and remove validators allowing them to set themselves as validators and validate malicious transactions. The issue is present in both the cairo and solidity part. The manager role is NOT trusted according to the contest description.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_126_group

# Protocol assume `from_handler` will be fit in `felt252`, this may cause unintended behavior

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-671
- **Submitter:** 0xDemon
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/671
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-671.md

## Brief Summary

Protocol assume `from_handler` will be fit in `felt252`, this may cause unintended behaviors, i.e `from_handler` higher than `felt252` this may cause overflow and signature verification failure

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# misspelling of the decode_transfer

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-117
- **Submitter:** 0xEllipticCurve
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/117
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-117.md

## Brief Summary

Inside the function of the ChakraSettlementHandler.sol contract, a functionality is written incorrectly if (payload_type == PayloadType.ERC20) { // Cross chain transfer { // Decode transfer payload ERC20TransferPayload memory transfer_payload = codec .deocde_transfer(msg_payload); <------

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_122_group

# Immediate Effect of Admin Actions on Validator and Handler Requirements Compromises Cross-Chain Transaction Integrity

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-461
- **Submitter:** 0xNirix
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/461
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-461.md

## Brief Summary

The current implementation of the cross-chain settlement system allows for immediate changes like to validator and handler configuration without considering in-flight transactions or synchronization across different blockchains. This can lead to various issues, including unexpected transaction failures and temporarily stuck funds. This issue is present in both solidity and cairo code. This immediate effect fails to account for: **In-flight transactions**: Cross-chain transactions that are already in progress when the changes are made may fail unexpectedly if they no longer meet the new validation or handler requirements. **Cross-chain synchronization**: Different blockchains may process the...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Missing Access Control Initialization

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-612
- **Submitter:** 0xabhay
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/612
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-612.md

## Brief Summary

This bug pertains to the absence of the `__AccessControl_init` in the `BaseSettlement::_Settlement_init` and `BaseSettlementHandler::_Settlement_handler_init` Function, which inherits from `AccessControlUpgradeable`. The lack of this initialization function can impact the proper functioning of access control roles within the contract, particularly affecting the `_grantRole` function. Without the initialization step, there is a risk of unauthorized access or incorrect role assignments, potentially compromising the security and integrity of the contract's access control mechanisms. Vulnerability Details The `BaseSettlement` contract inherits from `AccessControlUpgradeable` but lacks the ` __A...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_12_group

# Missing Length check in function “deocde transfer” in ERC20CodecV1.sol leading to unintended behaviour.

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-569
- **Submitter:** 0xpetern
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/569
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-569.md

## Brief Summary

[deocde_transfer](https://github.com/code-423n4/2024-08-chakra/blob/d0d45ae1d26ca1b87034e67180fac07ce9642fd9/solidity/handler/contracts/ERC20CodecV1.sol#L65C5-L65C29) is used to decode payload and this function is external. As seen from the code, there is no check to ensure that the payload lenght is within the acceptable range. Omitting the length check in decode_transfer function can lead to several potential issues, including: 1. Unexpected Behavior: If the payload is not the expected length, the function may try to access memory beyond the end of the provided data, leading to unexpected results or incorrect decoding. For example, if the payload is shorter than expected, accessing byte s...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Failed to use of ERC20BunableUpgradeable.sol contract in BaseSettlementHandler.sol

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-571
- **Submitter:** 0xpetern
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/571
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-571.md

## Brief Summary

BaseSettlementHandler.sol is an upgradeable contract and should import and make use of openzeppelin upgradeable contracts. It imported other upgradeable contracts but failed to use an upgradeable version of ERC20Bunable.sol which is ERC20BurnableUpgradeable.sol. It imported ERC20Burnable.sol which is not upgradeable. Impact When the protocol upgrades, ERC20Burnable.sol could behave in unintended manner because it is not upgradeable. This can lead to inconsistencies and complications in the behaviour of the protocol From https://docs.openzeppelin.com/upgrades-plugins/1.x/writing-upgradeable, it clearly warns against using OpenZeppelin contract that is not upgradeable as opposed to the upgrad...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# No zero address check on burn `to` at `ckr_btc.cairo::burn_from`

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-404
- **Submitter:** 0xwhisperingwoods
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/404
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-404.md

## Brief Summary

The `ckr_btc.cairo::burn_from` does not check the zero address in `to`. This can create serious vulnerabilities which can be further exploited. The user might burn their tokens unintentionally, leading to permanent loss. Furthermore, the invariants in contracts that assume that token cannot be burned to a zero address such as `handler_erc20.cairo::receive_cross_chain_callback` and `handler_erc20.cairo::cross_chain_erc20_settlement` could be broken, leading to flawed logical errors elsewhere in the protocol.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_19_group

# Critical Access Control Vulnerability in UUPS Upgrade Mechanism.

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-463
- **Submitter:** 14Kattel
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/463
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-463.md

## Brief Summary

The `BaseSettlementHandler` contract contains a critical vulnerability in its upgrade function. The `_authorizeUpgrade` function, which is used to authorize contract upgrades, is protected solely by the `onlyOwner` modifier. This means that if an attacker gains control of the owner's account, they could perform unauthorized upgrades. This flaw compromises the integrity and security of the contract, making it possible for malicious actors to inject malicious code or alter contract behavior. Vulnerability Details The vulnerability is present in the `_authorizeUpgrade` function: Impact The `BaseSettlementHandler.sol` contract has a critical vulnerability related to the upgrade mechanism due to...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Unrestricted Minting in ChakraToken.sol

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-465
- **Submitter:** 14Kattel
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/465
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-465.md

## Brief Summary

The ChakraToken contract contains a **medium severity vulnerability** in its minting mechanism. The minting functions (`mint` and `mint_to`) are accessible to any account with the `OPERATOR_ROLE`, and there are **no limits or constraints** on the number of tokens that can be minted. This lack of restriction can lead to significant inflation and dilution of token value, potentially resulting in financial losses for users and stakeholders. Additionally, excessive minting could disrupt dependent systems and trigger unintended consequences. The ability for `OPERATOR_ROLE` holders to mint an unlimited amount of tokens poses a serious risk of market manipulation and abuse.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_77_group

# No events emitted when handlers are being added and removed

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-624
- **Submitter:** 4B
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/624
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-624.md

## Brief Summary

When a handler is added to the whitelist or removed there is no event emission to off-chain entities. But as we can see in similar functions like add_validator, add_operator, add_manager they all emit events after execution. This very necessary in the functioning of this protocol, since there is a lot that goes on across chains.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# invalid addresses can bypass validation check in `addressCast`

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-652
- **Submitter:** 4B
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/652
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-652.md

## Brief Summary

Invalid addresses with bytes length/size less than 20 will bypass all validation checks

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# MSTORE is different on zkSync

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-657
- **Submitter:** 4B
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/657
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-657.md

## Brief Summary

On zkSync, MSTORE has a different meaning and as result applies differently as compared to ethereum. this can cause further complications or misinterpretations on these chains

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# ChakraSettlement would not work on zkSync due to the nonce ordering

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-162
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/162
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-162.md

## Brief Summary

In the current implementation of `ChakraSettlement` and `ChakraSettlementHandler` contract, there is a `nonce_manager` mapping that's increased every time there is a new cross chain transaction. The problem is that the protocol is supposed to work on zkSync and it may not work due to the problem of nonce ordering.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Owner cannot remove handler from the whitelist on Starknet

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-213
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/213
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-213.md

## Brief Summary

The owner of the handler / settlement contract has to be able to add / remove the owner of the handler. However, on the Starknet chain it cannot be done done so as `remove_handler()` functionality is missing.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Message hash is assigned incorrect values when verifying signatures on Starknet

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-440
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/440
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-440.md

## Brief Summary

In the current implementation of the `settlement` contract, `message_hash` is being reassigned inside of `receive_cross_chain_msg()` function. This is an unexpected behavior as eventually incorrect value will be verified.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_59_group

# Issues with Data Types in Event and Struct Definitions for Cross-Chain Transactions in BaseSettlementHandler.sol

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-175
- **Submitter:** Alhakista
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/175
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-175.md

## Brief Summary

This report examines the data types used in the CrossChainLocked event and CreatedCrossChainTx struct in BaseSettlementHandler.sol Solidity contract, highlighting the invalid data. There are critical issues with the usage of uint256 for fields that should be of type address. The fields to and to_token in both the event and struct definitions are incorrectly declared as uint256 when they represent addresses on the destination chain. This creates a scenario where the contract performs in an unexpected manner due to internal errors.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# `SettlementSignatureVerifier` Fails to Validate Smart Contract Wallet Signatures from Validators

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-707
- **Submitter:** Breeje
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/707
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-707.md

## Brief Summary

If a validator uses a smart contract wallet instead of an externally owned account (EOA), the `SettlementSignatureVerifier` fails to validate its signature.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Accepting all ERC20 tokens is not safe

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-187
- **Submitter:** Decap
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/187
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-187.md

## Brief Summary

Accepting all ERC20 by the bridge is not safe. Malicious user can create ERC20 token contract that will be harmful for the protocol. For example it is possible to create ERC20 token that will be very gas expensive when making transfer. This can burn out funds out of contract, cause DoS for other users, block bridge etc. Here is example contract that can simulate malicious ERC20 token

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# There is a problem with the data structure in the inheritance of upgradable contracts

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-99
- **Submitter:** Drynooo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/99
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-99.md

## Brief Summary

In contracts such as BaseSettlementHandler, gap variables are not used, nor are designated storage slots used for storage. During subsequent upgrades, variable overwriting may occur.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_49_group

# The Number Of Required Validators on Starknet Can Be Set To Zero

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-593
- **Submitter:** Respx
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/593
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-593.md

## Brief Summary

Although the likelihood of a manager setting the number of required validators to zero is low, the impact would be extreme: it would allow any user to successfully submit any cross chain message to the protocol. This would essentially unlock all tokens held by the relevant handler contract to any user, and also allow any other handlers to be compromised.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_05_group

# Incorrect message ID parsing due to off-by-one error in `decode_message`

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-250
- **Submitter:** Rhaydden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/250
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-250.md

## Brief Summary

The `decode_message` function doesnt correctly parse the `message_id` field from the payload due to an off-by-one error. Because of this, `message_id` will be misaligned, which can lead to incorrect message processing, potential misinterpretation of the message, and downstream errors in any logic that relies on the correct `message_id`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_39_group

# Missing keccak256 when computing the message_hash

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-493
- **Submitter:** SBSecurity
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/493
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-493.md

## Brief Summary

`message_hash` is incorrectly computed, which will not allow signature verification.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_129_group

# Integer overflow in mint_to funciton

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-364
- **Submitter:** Sabit
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/364
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-364.md

## Brief Summary

The actual amount minted will be much smaller than intended.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# TokenRoles contract does not implement `_authorizeUpgrade` & `__UUPSUpgradeable_init()`

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-730
- **Submitter:** Shubham
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/730
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-730.md

## Brief Summary

`ChakraToken` contract inherits the `TokenRoles` contract & initializes it in the `initialize()`. The ChakraToken contract implements the `_authorizeUpgrade()` but the `TokenRoles` contract neither implements the `__UUPSUpgradeable_init()` nor the `_authorizeUpgrade()` Impact `TokenRoles` contract cannot be upgraded in the future even though it inherits UUPSUpgradeable from oz.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_48_group

# After all validators are removed, the system can no longer perform effective signature verification.

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-622
- **Submitter:** Taiger
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/622
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-622.md

## Brief Summary

The vulnerability exists in the `remove_validator` function, where the removal of a validator is performed without any checks on the remaining number of validators. If all validators are removed from the system, the contract could become non-functional, as it would no longer have any validators available to verify signatures. This would lead to critical security risks because cross-chain transactions would no longer be validated. The absence of validators would leave the system open to attacks and manipulation, or even cause it to cease functioning entirely. Consequences: - **Loss of Signature Validation**: If all validators are removed, the `check_chakra_signatures` function will not be ab...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Cross chain callback will always fail if source `chain_name` is changed.

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-342
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/342
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-342.md

## Brief Summary

The bug will cause the cross-chain callback to fail when the `chain_name` is changed at source after a cross-chain tx has been sent.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_22_group

# Missing zero address check for owner in `TokenRoles_transferOwnership`

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-554
- **Submitter:** Tonchi
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/554
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-554.md

## Brief Summary

This contract is inherited by `ChakraToken.sol` and `ChakraTokenUpgradeTest.sol`. Ownership Transfer to Zero Address, Permanent Loss of Control, Mismanagement of Role. 1. Critical Loss of control: - Ownership loss is one of the most severe vulnerabilities because it permanently disables the contract’s governance and security features. - Without an active owner, no updates, fixes, or upgrades can be made to the contract, which is especially dangerous for upgradeable contracts like this one. - If this is a contract in charge of valuable assets (like tokens or DeFi operations), this can lead to irreversible financial damage. 2. Attack Vector for Malicious Actors: - An attacker or a malicious a...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Calldata Slicing Inefficiency Leading to Contract Disruption and Inaccessibility

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-232
- **Submitter:** Topmark
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/232
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-232.md

## Brief Summary

The current implementation of signature verification in SettlementSignatureVerifier contract using calldata slicing introduces a critical inefficiency that could lead to severe problems for the contract’s usability. calldata slicing use in as would be explained in this report results in repeated, unnecessary memory allocations, significantly increasing gas consumption for every signature processed. This inefficiency does more than just increase gas usage. In practice, this excessive gas costs can lead to transaction Reversion and any attempt to verify multiple signatures (e.g., for multi-signature transactions or consensus mechanisms) could exceed the gas limit, causing critical transaction...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_133_group

# An attacker can reuse old message with altered payload

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-271
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/271
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-271.md

## Brief Summary

In the current implementation there is no checks to make sure that the payload used is unique and has not already been used. it is important to verify the uniqueness of the payload when processing cross-chain messages. As a result, the same payload could be reused across different transactions, leading to potential replay attacks. The current txid generation logic omits the `payload_type` and `payload` fields, which can lead to potential issues: The absence of `payload_type` and `payload` in the txid hash makes the system susceptible to replay attacks or message forgery, where an attacker might reuse an old message with altered payloads, bypassing checks and causing unintended consequences....

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_57_group

# Type Mismatch in from_address Parameter `uint256` instead of `address`

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-272
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/272
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-272.md

## Brief Summary

The `from_address` parameter is represented as an address in the function `send_cross_chain_msg` but incorrectly represented as a uint256 instead of an address in the function `receive_cross_chain_msg`. From the comments This means that message_hash will have from_address as type uint256 instead of type address This type mismatch could lead to incorrect address handling, potential signature verification failures, and unexpected behavior within the smart contract. If the from_address is misinterpreted or manipulated, it could open up the contract to potential exploits, such as bypassing access controls or altering transaction outcomes. By doing this our message_hash now contains a `from_addr...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_63_group

# Signature Verification missing protection from Replay Attacks

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-273
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/273
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-273.md

## Brief Summary

Currently there is no way the function is protected from replay attacks as there is no nonce or even a deadline. Without a deadline (or expiration time) for signatures in cross-chain message verification or a nonce, an attacker could potentially reuse a valid signature indefinitely. This lack of temporal restriction can open up the system to replay attacks, where old messages are resent to manipulate the system, bypass intended logic, or disrupt operations. It is always a good idea to use deadlines and nonce whenever signatures are involved

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_115_group

# Lack of Message Hash Validation in verifyECDSA Could Allow Forgery or Replay Attacks

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-279
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/279
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-279.md

## Brief Summary

The current implementation of the `verifyECDSA` function does not validate the source or correctness of the `msgHash`. This leaves the system vulnerable to signature forgery and replay attacks. An attacker could potentially submit a valid signature for a malicious or incorrect `msgHash`, leading to unauthorized actions within the system. By not verifying that the `msgHash` corresponds to a legitimate transaction, validators could unknowingly sign off on illegitimate or fraudulent transactions, allowing attackers to execute unintended operations, such as replaying previous valid signatures in a different context, resulting in repeated execution of certain actions.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_58_group

# Off-chain validators(executors) can be griefed on multiple occasion by malicious transaction creators

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-732
- **Submitter:** b0g0
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/732
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-732.md

## Brief Summary

Malicious users can create transactions with invalid parameters due to inefficient validation when transactions are created and grief the executors of callbacks. Description Cairo contracts can be exploited for the following reasons: - cross_chain_erc20_settlement - can be called with 0 amount or address(0) for to - send_cross_chain_msg - can provide invalid `payload_type` which also revert execution later Solidity - the to_hadnler is not validated

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_15_group

# wrong implement of "verifyECDSA"

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-283
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/283
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-283.md

## Brief Summary

Detailed description of the impact of this finding. There is no check of zero address for msgHash.recover(sig).There should be a check for validators address should be not zero.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_53_group

# Too much power given to "manager" Centralization issue

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-604
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/604
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-604.md

## Brief Summary

Detailed description of the impact of this finding. In the "settlement.cairo" we are giving too much power to the "manager".as it can set the number of required_validators_num.it can also add and remove the validator.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_08_group

# Insufficient gas requirement in cross-chain ERC20 settlement

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-252
- **Submitter:** blackpanther
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/252
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-252.md

## Brief Summary

The `cross_chain_erc20_settlement` function, crucial for initiating token transfers from Ethereum to Starknet, lacks a minimum gas requirement setting. This omission can lead to transaction failures in the cross-chain messaging process, potentially resulting in stuck or failed token transfers. Vulnerability Details - **Function**: `cross_chain_erc20_settlement` - **Issue**: No minimum gas requirement set for `send_cross_chain_msg` call - **Required Gas**: Minimum 20,000 wei for L1 to L2 messaging as per [docs](https://book.cairo-lang.org/ch16-04-L1-L2-messaging.html?highlight=msg.value#sending-messages-from-ethereum-to-starknet:~:text=It's%20important%20to%20note%20that%20we,be%20deserializ...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_89_group

# Potential Overflow in Bitmasking Operations

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-254
- **Submitter:** crown22
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/254
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-254.md

## Brief Summary

The constants in the file used for bit manipulation (MASK_8, TWO_POW_8, etc.) can potentially cause overflows or incorrect handling of large numbers. When used improperly, these constants might result in unexpected values in cryptographic operations or arithmetic manipulations. If such issues occur in a cross-chain context or while calculating balances, it could lead to major inconsistencies, security exploits, or financial losses in the system.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Reentrancy Vulnerability in ERC20 Handler

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-256
- **Submitter:** crown22
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/256
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-256.md

## Brief Summary

Without reentrancy protection, an attacker can exploit ERC20 functions by recursively calling them before the state is fully updated. This could result in repeated transfers or minting of tokens, allowing attackers to drain the contract’s funds or inflate the token supply.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Inconsistent Message Encoding/Decoding

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-257
- **Submitter:** crown22
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/257
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-257.md

## Brief Summary

Incorrect or inconsistent encoding/decoding of cross-chain messages can lead to manipulated payloads being passed between chains. If a payload is incorrectly decoded or lacks strict validation, an attacker could craft a message that bypasses intended checks, causing incorrect transaction processing or unauthorized transfers.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Protection Against Malicious Contract Addresses

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-261
- **Submitter:** crown22
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/261
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-261.md

## Brief Summary

The lack of protection against malicious contract addresses poses a significant security risk, allowing attackers to replace valid contract addresses with malicious ones. This can lead to unintended interactions, such as unauthorized token transfers, reentrancy attacks, or malicious code execution that drains funds from the protocol. Without proper validation, malicious contracts can be introduced into critical functions.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_07_group

# Weak Validation of ERC20 Transfer Data

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-262
- **Submitter:** crown22
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/262
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-262.md

## Brief Summary

Weak validation of ERC20 transfer data, particularly for fields like amount, can lead to various attack vectors, including overflow attacks or unintended token transfers. This opens up the potential for malicious users to transfer more tokens than intended or to exploit overflow vulnerabilities to disrupt the contract's logic.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# TokenRoles contract has Incorrect Argument Count in __Ownable_init Call

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-16
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/16
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-16.md

## Brief Summary

Detailed description of the impact of this finding. Title: TokenRoles contract has Incorrect Argument Count in __Ownable_init Call • Severity: Medium • Impact: __Ownable_init() does not accept any arguments, leading to a type error during contract deployment. • Status: Unresolved • File: TokenRoles.sol • Lines Affected: 20 Contract Initialization Failure. The TokenRoles contract attempts to initialize the OwnableUpgradeable contract by calling __Ownable_init(_owner). However, __Ownable_init() does not accept any arguments, leading to a type error during contract deployment.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_50_group

# ChakraSettlementHandler contract has Unchecked transferFrom Return Value in _safe_transfer_from During ERC20 Locking

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-300
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/300
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-300.md

## Brief Summary

Detailed description of the impact of this finding. Title: ChakraSettlementHandler contract has Unchecked transferFrom Return Value in _safe_transfer_from During ERC20 Locking. Contract: ChakraSettlementHandler.sol. Functions: _safe_transfer_from() & _erc20_lock(). The _safe_transfer_from function does not verify the return value of the external IERC20(token).transferFrom(from, to, amount) call. According to the ERC-20 standard, the transferFrom function returns a boolean indicating whether the transfer succeeded. Some ERC-20 tokens may return false instead of reverting on failure. If the contract does not check the return value, it will assume that the transfer was successful even if it fa...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_56_group

# ERC20CodecV1 contract has Typographical Error in Function Name deocde_transfer

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-48
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/48
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-48.md

## Brief Summary

Detailed description of the impact of this finding. Title: ERC20CodecV1 contract has Typographical Error in Function Name deocde_transfer • Severity: Medium • Impact: A typographical error was found in the function name deocde_transfer within the Solidity contract and interface. • Status: Unresolved • File: ERC20CodecV1.sol • Lines Affected: 65-74 Location: • File: /2024-08-chakra/solidity/handler/contracts/ERC20CodecV1.sol • Line: 54 • File: /2024-08-chakra/solidity/handler/contracts/interfaces/IERC20CodecV1.sol • Line: 23 • Solidity File: /2024-08-chakra/solidity/handler/contracts/ERC20CodecV1.sol • Line: 54 • Interface File: /2024-08-chakra/solidity/handler/contracts/interfaces/IERC20Cod...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_40_group

# Unsafe use of `transfer()`/`transfer_from()` across Cairo contracts

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-630
- **Submitter:** devival
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/630
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-630.md

## Brief Summary

The automated bot report does not include the Cairo handler contract, even though the same issue is present in Solidity handler contract. Not all `IERC20` implementations `revert()` when there's a failure in `transfer()`/`transferFrom()`. The function signature has a `boolean` return value and they indicate errors that way instead. By not checking the return value, operations that should have marked as failed, may potentially go through without actually making a payment. Also, some tokens do not implement the ERC20 standard properly, and do not return booleans as the specification requires. Instead, they have no return value. Impact In the context of `cross_chain_erc20_settlement` function...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_29_group

# Missing parameters in txid hash compute

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-727
- **Submitter:** eierina
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/727
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-727.md

## Brief Summary

The parameters `payload_type` and `payload` are not included in the txid hash compute, allowing the `payload` / `payload_type` to be tampered with for a given txid.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# The `verifyECDSA` function doesn't handle the case of an empty signature array correctly. It immediately returns false if there are no signatures, even though the invariant expects it to return true for `sign_type == 0`.

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-115
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/115
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-115.md

## Brief Summary

The `SettlementSignatureVerifier` contract suffers from vulnerability that allows any transaction to be validated with an empty signature when the signature type is set to ECDSA (0). This flaw compromises the entire signature verification process, leading to unauthorized transactions being processed as valid. **The impact is severe, as it undermines the fundamental security assumptions of the system, allowing attackers to execute unauthorized settlements or manipulate the state of the contract without proper authentication.**

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_81_group

# The `remove_validator` function does not check if the validator being removed actually exists in the `validators` mapping. It only checks if the caller has the `MANAGER_ROLE`.

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-122
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/122
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-122.md

## Brief Summary

The impact of this bug is that it allows managers to arbitrarily decrease the `validator_count` by removing non-existent validators. Unauthorized parties can remove validators, potentially compromising the integrity of the settlement process. **The impact of this issue is twofold:** 1. The `validator_count` can be manipulated by managers, causing it to diverge from the actual number of active validators. 2. The inconsistency in the validator management system may lead to confusion and incorrect assumptions about the state of the validators. While this does not directly lead to a loss of funds, it can cause discrepancies in the validator set and undermine the integrity of the settlement sign...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_17_group

# nonce is incremented unconditionally even if the `amount` requirement fails. If `amount` is 0, the function will revert due to the failed requirement, but the nonce would have already been incremented.

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-66
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/66
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-66.md

## Brief Summary

The `ChakraSettlementHandler` contract is vulnerable in the `cross_chain_erc20_settlement` function where the nonce is incremented unconditionally, even if the settlement fails due to invalid input parameters. **The affected parties are:** * Users who initiate cross-chain ERC20 settlements with invalid parameters (e.g., zero amount). They may lose the ability to retry the settlement with the same nonce. * The protocol itself, as it may have inconsistent nonce values for users, leading to potential replay attacks and incorrect transaction handling.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_61_group

# Unsafe casting leads to sending tokens to incorrect address

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-358
- **Submitter:** firmanregar
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/358
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-358.md

## Brief Summary

The function casts a 'uint256' to an address by first converting it to a 'uint160'. 'uint160' is unsafe when you cast from larger types like 'uint256' without ensuring that the value fits within the 160-bit range, as this can result in data truncation and potential vulnerabilities. Casting a 'uint256' to 'uint160' truncates the upper 96 bits (256 - 160 = 96). This means if the original 'uint256' contains important data in those upper 96 bits, it will be lost in the conversion, leading to incorrect results. If that 'uint256' is supposed to be an address but contains extra data, you could end up with an invalid or unintended address. If the input to the cast comes from an untrusted source (su...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_67_group

# Lack of Time-Lock Enforcement in Cross-Chain Settlement Functions

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-324
- **Submitter:** igdbase
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/324
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-324.md

## Brief Summary

The Chakra Protocol claims to provide self-custodian staking by allowing users to stake Bitcoin without transferring assets out of their wallets. This is supposed to be achieved through time-lock scripts, which ensure that assets are not exposed to third-party risks. However, the current implementation of key cross-chain settlement functions, including `receive_cross_chain_msg`, `receive_cross_chain_callback`, and `cross_chain_erc20_settlement`, does not enforce any time-locks on assets. As a result, assets that should remain locked for a defined period can be settled or unlocked prematurely. This introduces risks of premature asset release and potential double-spending exploits, underminin...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Status Update in processCrossChainCallback Function

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-418
- **Submitter:** m4k2
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/418
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-418.md

## Brief Summary

In the `processCrossChainCallback` function of the ChakraSettlement contract, there's a logical error in updating the status of cross-chain transactions. Even when the `receive_cross_chain_callback` function returns true (indicating success), the status may incorrectly remain as "Pending" instead of being updated to "Success". Impact This vulnerability can lead to several significant issues: 1. Inconsistent Transaction States: Transactions that have been successfully processed may remain in a "Pending" state, leading to confusion and issues with further processing or user interactions. 2. Broken State Machine: The incorrect status update breaks the expected flow of the cross-chain transacti...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_27_group

# Incorrect Interface Dispatcher Usage in Settlement Contract

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-682
- **Submitter:** mansa11
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/682
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-682.md

## Brief Summary

The Settlement contract is attempting to use an undefined IHandlerDispatcher interface at lines 325 and 414, which is not present in the provided interfaces. This causes a mismatch between the contract implementation and the defined interfaces. Description In the Settlement contract `settlement.cairo` - cairo\handler\src\settlement.cairo, at lines `353` and `393` which are both present in the `receive_cross_chain_msg` and `receive_cross_chain_callback` respectively, the code attempts to create a dispatcher for an interface that doesn't exist: However, the `interfaces.cairo` file does not define an `IHandlerDispatcher`. Instead, it provides two relevant interfaces: `IHandler` and `IERC20Hand...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# There is actually no checks that prevents a non ERC20 token being passed

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-512
- **Submitter:** mjcpwns
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/512
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-512.md

## Brief Summary

There is a lack of check for whether an ERC 20 token is actually passed in or another type of token require(isValidPayloadType(payload_type), "Invalid payload type"); This invariant will be broken: The contract only accepts valid payload types (in this case, only ERC20 payloads). To be honest I do not think the impact will be high, maybe the user will lose their token if it is not ERC20 but I do not think it will be a big issue but I report this just in case.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# A block reorg can leave users with extra tokens

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-700
- **Submitter:** peanuts
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/700
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-700.md

## Brief Summary

A block reorg can leave users with extra tokens

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_35_group

# Missing Checks for Address(0)

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-192
- **Submitter:** pwnforce
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/192
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-192.md

## Brief Summary

Following functions miss validation checks for Address(0): 1- ChakraSettlementHandler: initialize function should check address values _owner, _token, _codec, _verifier, and _settlement. 2- ChakraSettlement: initialize function should check address values _owner and _verify_contract: 3- TokenRoles: functions __TokenRoles_init, transferOwnership, and add_operator should check address values _owner, _operator, newOwner, and newOperator.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_107_group

# Missing Checks for Removing Managers in settlement.cairo and ckr_btc.cairo

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-223
- **Submitter:** pwnforce
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/223
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-223.md

## Brief Summary

There are two `remove_manager` functions in settlement.cairo and ckr_btc.cairo: However, there are no checks to ascertain that the old_manager being removed is indeed an existing manager. For example, in functions `remove_validator` and `remove_operator` in settlement.cairo and ckr_btc.cairo respectively, it is checked that the old_validator and old_operator being removed are indeed existing validators and operators: This would result in incorrect state changes, where non-managers could be marked as removed managers.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_51_group

# Decimals assigned on mutable variable

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-513
- **Submitter:** rabTAI
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/513
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-513.md

## Brief Summary

Token decimals is stored on mutable variable which could be modified later since it is an upgradable contract. Changing the number of decimals could cause issues in external applications or token holders who assume a fixed number of decimals. `uint8 set_decimals;`

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Decoding and Encoding of Addresses

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-515
- **Submitter:** rabTAI
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/515
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-515.md

## Brief Summary

The contract uses abi.decode with uint256 for decoding addresses (from, to, from_token, to_token). However, Ethereum addresses are 20 bytes (160 bits) long, not 32 bytes. Using uint256 will result in incorrect decoding of addresses, leading to potential security vulnerabilities such as incorrectly decoded addresses, which could result in token transfers being sent to wrong or invalid addresses.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# No input validation for Encoding/Decoding Functions

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-516
- **Submitter:** rabTAI
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/516
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-516.md

## Brief Summary

There is no input validation on the encoded or decoded payloads. Without validation, malicious or malformed payloads could result in unexpected behavior, including incorrect decoding or possible denial of service (DoS) attacks.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_108_group

# Lack of Proper Event Emission in Critical Function

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-186
- **Submitter:** rare_one
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/186
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-186.md

## Brief Summary

While the function updates the internal state of the contract,the function receive_cross_chain_callback in handler_erc20.cairo contract doesn't emit any events to notify external observers (like applications or other contracts) about the successful settlement. Events are critical for providing transparency and traceability of contract interactions, especially when state-changing operations affect cross-chain transactions and token balances. IMPACTS: Potential for Malicious Activities to Go Undetected: - In the absence of emitted events, an attacker could exploit vulnerabilities or manipulate state variables without being detected. This could be used to obscure malicious activities such as t...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_86_group

# Birthday Attack Vulnerability in `txid` Generation

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-480
- **Submitter:** sivanesh_808
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/480
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-480.md

## Brief Summary

The `txid` (transaction ID) is generated using the `keccak256` hash function, a cryptographic hash function that maps an arbitrary amount of input data into a fixed-length output of 256 bits. While this ensures a unique output for different inputs, the large input space and repetitive parameters in the input to `keccak256` open up the possibility of a **birthday attack**, where two different inputs generate the same hash output. The birthday attack exploits the **birthday paradox**, which reduces the complexity of finding a hash collision from `2^256` to approximately `2^128`, making collisions significantly more feasible with enough attempts.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of message ordering may lead to failed transactions

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-568
- **Submitter:** tonisives
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/568
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-568.md

## Brief Summary

The `receive_cross_chain_msg` function in the `ChakraSettlement` contract processes cross-chain messages without ensuring that they are executed in the order they were sent. This can lead to potential transaction failures due to dependencies between messages. Impact The lack of enforced message ordering can result in unexpected behavior or failures in dependent transactions. For example, if a later message depends on the successful processing of an earlier message, processing them out of order can cause the later message to fail, leading to potential loss of funds or incorrect state.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_94_group

# Missing check on mode in ChakraSettlementHandler::_Settlement_handler_init function

- **Contest:** Chakra
- **Slug:** 2024-08-chakra
- **Submission:** V-230
- **Submitter:** yudan
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-chakra-validation/issues/230
- **Source snapshot:** competitions/2024-08-chakra/submissions/raw/V-230.md

## Brief Summary

In ChakraSettlementHandler::cross_chain_erc20_settlement function, it depends on the value of `mode` to determine if token needs to be lock or unlock, and call `_erc20_lock` or `_erc20_unlock` internal function. But there missing check that the mode may not one of value in enum `SettlementMode`, e.g. 5, which exceed the max value of enum `SettlementMode`. In this way, the function will do nothing for user's token, but construct a cross chain request. which make malicious user receive token on the other chain.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_62_group
