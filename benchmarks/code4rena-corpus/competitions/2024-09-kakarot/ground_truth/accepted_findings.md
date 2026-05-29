# Accepted H/M Findings: Kakarot

# [H-01] Unauthorized contracts can bypass precompile authorization via delegatecall in Kakarot zkEVM

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-09-kakarot
- **Source snapshot:** competitions/2024-09-kakarot/final_report.html

delegatecall in Kakarot zkEVM Submitted by 0xastronatey, also found by RadiantLabs In the Kakarot zkEVM implementation exec_precompile, unauthorized contracts can bypass the precompile authorization mechanism using the delegatecall. This allows unauthorized contracts to execute privileged precompile functions intended only for authorized (whitelisted) contracts, compromising the security and integrity of the system.

Vulnerability Details:

Let’s take a look at the exec_precompile function with the unessential part omitted for brevity:

func exec_precompile{ syscall_ptr: felt*, pedersen_ptr: HashBuiltin*, range_check_ptr, bitwise_ptr: BitwiseBuiltin*, }( precompile_address: felt, input_len: felt, input: felt*, caller_code_address: felt, caller_address: felt, ) -> (output_len: felt, output: felt*, gas_used: felt, reverted: felt) { //SNIP...

// Authorization for Kakarot precompiles let is_kakarot_precompile = PrecompilesHelpers.is_kakarot_precompile(precompile_address); if is_kakarot_precompile != 0 { // Check if caller is whitelisted let is_whitelisted = KakarotPrecompiles.is_caller_whitelisted(caller_code_address); if is_whitelisted == FALSE { jmp unauthorized_call; } // Proceed with precompile execution //...

} else { jmp unauthorized_call; } The issue arises because the authorization mechanism relies solely on caller_code_address, which refers to the code being executed, and does not consider caller_address; the actual address of the contract making the call. This approach is insufficient and can be exploited using delegatecall.

If an EVM contract A delegatecall ’s EVM contract B, and B is allowlisted, then the call to the Cairo precompile succeeds because the allowlist is based on the code address (of B) and not on the execution address (of A).

This means that an unauthorized contract (Contract A) can exploit this behavior to execute privileged operations by delegatecalling a whitelisted contract (Contract B) and passing the authorization check incorrectly.

As such, it is an issue for other whitelisted smart contracts allowing arbitrary calls to external smart contracts. For example, this is the case of some smart contracts allowing callbacks. In those situations, a malicious user could make a DEX execute a call to the malicious smart contract that would be able to access the precompiles pretending to be the DEX.

## Impact

This issue could allow unauthorized contracts to perform operations that should be restricted to whitelisted contracts like in the case of DEX that supports callbacks and has been added to the allowistlist. Moreover, all contracts within the allowistlist must be written with the assumption that they can also be delegatecall ed; i.e., they must not make any assumptions about their storage or their own EVM address ( address(this) ).

Prior to my discussion with the sponsor in a private thread about the implications of this attack surface, the sponsor intended to eventually remove the whitelist mechanism, not fully considering the dangers of delegatecalls to precompiles, which would have been a bad idea.

With the team’s plan to eventually remove the allowlist, prior to knowing about this attack surface, it gives any contract the opportunity to exploit this on KakarotEVM; which, as we’ve seen, can be catastrophic; especially in the case of MoonBeam’s msg.sender impersonation disclosure by pwning.eth.

Recommendation In the interim, the obvious mitigation is to ensure that all whitelisted contracts are written with the assumption that they can also be delegatecall ed. And more importantly, the whitelist mechanism should never be removed, as this would open a floodgate of vulnerabilities to the protocol.

A permanent solution to this issue would be to disable Delegatecall s for custom precompiles.

## Assessed type

call / delegatecall ClementWalter (Kakarot) confirmed and commented:

Severity: High Kakarot mitigated:

This PR fixes the issue raised during the audit about eventual honeypots that could lure a user and manipulate starknet tokens without explicit approvals:

Remove dependence on the code_address for whitelisting, which were reported dangerous.

Disable ability of having nested a delegatecall to 0x75001 at protocol level. (no longer possible to do User A -call-> contract C -delegatecall->DualVmToken -delegatecall->0x75001 as a byproduct of 1.

Added noDelegateCall modifier to L2KakarotMessaging and DualVMToken for extra security.

Added associated tests under Security.

Status:

Mitigation confirmed.

# [H-02] Prover can cheat in felt_to_bytes_little due to value underflow

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-09-kakarot
- **Source snapshot:** competitions/2024-09-kakarot/final_report.html

felt_to_bytes_little due to value underflow Submitted by muellerberndt, also found by superpozycja and RadiantLabs The function felt_to_bytes_little() in bytes.cairo converts a felt into an array of bytes.

The prover can cheat by returning a near arbitrary string that does not correspond to the input felt, whereby, the spoofed output bytes and `bytes_len, bytes must fulfill some specific conditions (but, if carefully crafted, can contain almost arbitrary sequences of bytes).

This issue affects the function felt_to_bytes_little() as well as other functions that depend on it:

- felt_to_bytes() - uint256_to_bytes_little() - uint256_to_bytes() - uint256_to_bytes32() - bigint_to_bytes_array() Those functions are used throughout the code, notably in get_create_address() and get_create2_address(), which an attacker could exploit to deploy L2 smart contracts from a spoofed sender address (e.g., to steal funds from wallets that use account abstraction).

Details First, note the following loop:

body:

let range_check_ptr = [ap - 3]; let value = [ap - 2]; let bytes_len = [ap - 1]; let bytes = cast([fp - 4], felt*); let output = bytes + bytes_len; let base = 2 ** 8; let bound = base; %{ memory[ids.output] = res = (int(ids.value) % PRIME) % ids.base assert res < ids.bound, f'split_int(): Limb {res} is out of range.' %} let byte = [output]; with_attr error_message("felt_to_bytes_little: byte value is too big") { assert_nn_le(byte, bound - 1); } tempvar value = (value - byte) / base; tempvar range_check_ptr = range_check_ptr; tempvar value = value; tempvar bytes_len = bytes_len + 1; jmp body if value != 0; We observe that:

Value can underflow if the byte returned in the last iteration is greater than the value remaining in the felt. For example, if the remaining value is 1 but the prover/hint returns 2, value will underflow into STARKNET_PRIME - 1. Consequently, the loop will continue running as value != 0.

In the following iterations of the loop, the prover can return arbitrary values, they just need to ensure that value eventually becomes 0. They can use as many iterations as required, but it is important that bytes_len ends up at a specific value (see 3).

Finally, at the end of the function, there is a check that bytes_len is the minimal one possible to represent the value. The lower bound and upper bound for this check is read from pow256_table at the offset bytes_len. The malicious prover must ensure that the value at this memory address (somewhere into the code segment) is lower than initial_value. Any nearby position where the Cairo bytecode contains a zero works.

## Recommended Mitigation Steps

The easiest fix is to do range checks on value to prevent underflows.

## Assessed type

Math ClementWalter (Kakarot) confirmed and commented:

Severity: High Kakarot mitigated:

This PR fixes felt_to_bytes_little loop stop condition. The loop will stop at 31 bytes.

Status:

Mitigation confirmed.

# [H-03] Missing constraint in default_dict_copy

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-09-kakarot
- **Source snapshot:** competitions/2024-09-kakarot/final_report.html

default_dict_copy Submitted by g_x_w

- https://github.com/kkrt-labs/kakarot/blob/7411a5520e8a00be6f5243a50c160e66ad285563/src/kakarot/account.cairo#L83
- https://github.com/kkrt-labs/kakarot/blob/7411a5520e8a00be6f5243a50c160e66ad285563/src/utils/dict.cairo#L83

## Vulnerability details

In CairoZero, the correct usage of dict objects created via default_dict_new must be paired with a call to default_dict_finalize to ensure the integrity and prevent malicious prover’s manipulation of its contents. However, this constraint is missing in the handling of transient_storage, storage and valid_jumpdests, leading to severe vulnerabilities when executing smart contracts.

Description of the Issue According to CairoZero’s documentation ( link to default_dict ), a proper workflow involving default_dict_new includes a finalization step using default_dict_finalize. This ensures the correct initialization of dictionary elements and prevents malicious provers from manipulating dictionary values through hints. Specifically, default_dict_finalize enforces the constraint that the initial value of the first element’s prev_value in the dictionary must equal to the default_value.

However, in the case of transient_storage, storage and valid_jumpdests, this crucial constraint is missing. I will illustrate this issue using transient_storage. First, transient_storage is initialized in Account.init() as follows:

let (transient_storage_start) = default_dict_new ( 0 ); However, there is no subsequent call to default_dict_finalize(transient_storage_start, transient_storage, 0) to finalize the storage. Instead, the function default_dict_copy() is called on transient_storage multiple times during a transaction through the Account.copy() function:

let (transient_storage_start, transient_storage) = default_dict_copy ( self.transient_storage_start, self.transient_storage ); This copy operation starts by calling dict_squash on the original transient_storage:

func default_dict_copy{range_check_ptr}(start: DictAccess*, end: DictAccess*) -> ( DictAccess*, DictAccess* ) { alloc_locals; let (squashed_start, squashed_end) = dict_squash (start, end); local range_check_ptr = range_check_ptr; let dict_len = squashed_end - squashed_start; local default_value; if (dict_len == 0 ) { assert default_value = 0; } else { assert default_value = squashed_start.prev_value; } let (local new_start) = default_dict_new (default_value);...

dict_squash itself does not assert the prev_value of the first element in the dictionary. As a result, the subsequent copied transient_storage ’s initial value is also under the malicious prover’s control.

let (local new_start) = default_dict_new(default_value); If we look at the source code of default_dict_finalize, we will notice an extra constraint in the default_dict_finalize_inner function:

func default_dict_finalize{range_check_ptr}( dict_accesses_start: DictAccess*, dict_accesses_end: DictAccess*, default_value: felt ) -> (squashed_dict_start: DictAccess*, squashed_dict_end: DictAccess*) { alloc_locals; let (local squashed_dict_start, local squashed_dict_end) = dict_squash ( dict_accesses_start, dict_accesses_end ); local range_check_ptr = range_check_ptr; default_dict_finalize_inner ( dict_accesses_start=squashed_dict_start, n_accesses=(squashed_dict_end - squashed_dict_start) / DictAccess.SIZE, default_value=default_value, ); return (squashed_dict_start=squashed_dict_start, squashed_dict_end=squashed_dict_end); } func default_dict_finalize_inner ( dict_accesses_start: DictAccess*, n_accesses: felt, default_value: felt

) {...

assert dict_accesses_start.prev_value = default_value;...

} As shown above, the additional check besides dict_squash is:

assert dict_accesses_start.prev_value = default_value; This constraint ensures that any uninitialized read from the dictionary returns the correct default value ( 0 in this case). However, in default_dict_copy, this constraint is absent; meaning the prev_value for the first dictionary entry of transient_storage is not guaranteed to match the expected default value.

## Impact

A malicious prover could manipulate the value read from transient_storage, storage and valid_jumpdests. Specifically, they could fabricate a proof where uninitialized keys in the dictionary return values other than the intended default ( 0 ). This could lead to unintended or unauthorized access to funds, manipulation of contract state, or other security breaches depending on the logic in the upper-level EVM contract.

## Assessed type

Invalid Validation ClementWalter (Kakarot) confirmed and commented:

Severity: High ClementWalter (Kakarot) commented:

After another round of review from Zellic, it appears that this is invalid (went to fast in the validation of the finding) as the default_value is asserted below in the loop

- https://github.com/kkrt-labs/kakarot/commit/474951c7babbd57f06d074aa15ba7a2f1911af21#diff-18450849ea8d97868875c0b5eeb0934a496dbf959c60a5bcf9273ff5f2bbc298L117
We pick as default value the first prev_value.

We consider that it’s the default value for the whole dict.

We squash and copy the squashed dict into a new default_dict with the given default_value.

In the loop, we assert that the squashed.prev_value == default_value.

EV_om (Zenith) commented:

@ClementWalter, we picked a long time at this finding and were under the impression that it was valid. It would be great to get your take on the below POC which we believe would work:

Construct a transaction the attempts to copy a dictionary with no subsequent reads from the same key.

Modify all.prev_value s so the dictionary can be successfully squashed (the prover has the ability to do this).

default_value of the new dict is set to our modified prev_value.

The check on L117 always passes.

In the next call stack, reads will return the modified value.

ClementWalter (Kakarot) commented:

I don’t get the first part:

Construct a transaction the attempts to copy a dictionary with no subsequent reads from the same key Modify all.prev_values so the dictionary can be successfully squashed (the prover has the ability to do this) What dict squashing does is that it enforces the fact that the sequence of DictAccess* (filled by the prover at their will) is actually a valid sequence of dict read and write.

So the dict_squash part is fine and after it, it’s guaranteed that the dict was correctly used, but not yet that it was a default dict with a given value.

Then we pick the first prev as default_value and we copy the squash dict into a new dict, asserting in the mean time that all the prev_values are actually equal to the one picked.

Eventually, it means that The DictAccess* was a legit dict.

All the prev_values were equal to the same value.

Maybe you can try to craft a failing test case?

EV_om (Zenith) commented:

@ClementWalter, a full POC would take a bit of time, but maybe @3docSec has something lying around.

I don’t get the first part Construct a transaction the attempts to copy a dictionary with no subsequent reads from the same key.

You’re right, this may not be necessary. Squashing prevents modifying the prev_value of an arbitrary entry if there are writes after a read (since it could be constrained to a previously written value), but it should still be possible to modify the default prev_value.

Otherwise, as the finding claims:

Specifically, default_dict_finalize enforces the constraint that the initial value of the first element’s prev_value in the dictionary must equal to the default_value.

This was never enforced in the code in scope. So we did have that:

The DictAccess* was a valid sequence of dict read and write.

All the prev_values were equal to the same value.

But it was not verified that:

All the prev_values were equal to the original dictionary’s default value, as they should be after squashing.

3docSec (Zenith) commented:

Just adding that if we look at the code:

if (dict_len == 0) { assert default_value = 0; } else { assert default_value = squashed_start.prev_value; } In the else branch, squashed_start.prev_value is the free bird that is never validated: if it was asserted to be 0 in any place, the code would’ve been safe.

ClementWalter (Kakarot) commented:

There is a list of DictAccess*, filled initially at the prover’s will (prover can put any values in these slots in dict_read and dict_write.

Now squashing enforces that this “random” list is actually a valid list of DictAccess*, properly logging read and write of a regular dict (precisely that prev value is always sound with current value).

The “default” part of the dict is ignored in the initial squash_dict where the prover could still have populated the dict with any initial values (possibly all different).

From this squash dict, the code pick the first prev_value (whatever it is is up to the prover).

Then, it enforces that all the prev keys are actually of this same prev value, making it actually a default_dict(prev_value).

At this point the prev_value is up to the prover, but it’s guaranteed that it’s still a default_dict.

Note that this function is default_dict_*copy*_, so the purpose is only to copy a dict with the same default value, not to finalize it. At other places in the code we do use default_dict_finalize(0) to actually enforce the default values are 0.

Now, the issue may still be relevant if at some places, we don’t call this default_dict_finalize, which actually happens (!) for the account’s storage and valid_jumpdest.

So I guess that the issue is valid, not because the method itself, but because of missing default_dict_finalize for the account’s dict.

Kakarot mitigated:

PR fix:

default_dict_copy finalize with default value 0.

Status:

Mitigation confirmed.

# [H-04] Non-finalized dictionary in RIPEMD160 allows forging of output

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Finding ID:** H-04
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-09-kakarot
- **Source snapshot:** competitions/2024-09-kakarot/final_report.html

Submitted by RadiantLabs When ripemd160.finish() does not enter the if (next_block == FALSE) condition at L456, the dictionary x initialized at the beginning of the function is not finalized. Instead, x is reassigned to reference a new dictionary at L470:

File: ripemd160.cairo 456: if (next_block == FALSE) {...

462: default_dict_finalize(start, x, 0); 463: let (res, rsize) = compress(buf, bufsize, arr_x, 16); 464: return (res=res, rsize=rsize); 465: } 466: let (local arr_x: felt*) = alloc(); 467: dict_to_array{dict_ptr=x}(arr_x, 16); 468: let (buf, bufsize) = compress(buf, bufsize, arr_x, 16); 469: // reset dict to all 0.

470: let (x) = default_dict_new(0); Because the old dictionary is never finalized, it is possible to insert incorrect values for read operations on the old dictionary, which allows proving an incorrect output for any input of size > 55 (56 with the fix to our separate vulnerability on this value).

Read operations on the old dictionary are performed in ripemd160::absorb_data:

File: ripemd160.cairo 148: func absorb_data{range_check_ptr, bitwise_ptr: BitwiseBuiltin*, dict_ptr: DictAccess*}( 149: data: felt*, len: felt, index: felt 150: ) { 151: alloc_locals; 152: if (index - len == 0) { 153: return (); 154: } 155:

156: let (index_4, _) = unsigned_div_rem(index, 4); 157: let (index_and_3) = uint32_and(index, 3); 158: let (factor) = uint32_mul(8, index_and_3); 159: let (factor) = pow2(factor); 160: let (tmp) = uint32_mul([data], factor); 161: let (old_val) = dict_read{dict_ptr=dict_ptr}(index_4); 162: let (val) = uint32_xor(old_val, tmp); 163: dict_write{dict_ptr=dict_ptr}(index_4, val); 164:

165: absorb_data{dict_ptr=dict_ptr}(data + 1, len, index + 1); 166: return (); 167: }

## Recommended Mitigation Steps

Call default_dict_finalize(start, x, 0); before let (x) = default_dict_new(0);.

ClementWalter (Kakarot) confirmed and commented:

Severity: High Kakarot mitigated:

PR fix: finalize dictionary in RIPEMD160. Finalizes the dict and reassign to start after resetting the dict.

Status:

Mitigation confirmed.

# [H-05] RIPEMD-160 precompile yields wrong hashes for large set of inputs due to off-by-one error

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Finding ID:** H-05
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-09-kakarot
- **Source snapshot:** competitions/2024-09-kakarot/final_report.html

RIPEMD-160 precompile yields wrong hashes for large set of inputs due to off-by-one error Submitted by RadiantLabs, also found by gumgumzum The RIPEMD-160 digest integrates the input data with one additional block including the message length.

If we look at the Cairo code that achieves this:

File: ripemd160.cairo 455: let next_block = is_nn_le(55, len); 456: if (next_block == FALSE) { 457: dict_write{dict_ptr=x}(14, val); 458: dict_write{dict_ptr=x}(15, val_15); 459:

460: let (local arr_x: felt*) = alloc(); 461: dict_to_array{dict_ptr=x}(arr_x, 16); 462: default_dict_finalize(start, x, 0); 463: let (res, rsize) = compress(buf, bufsize, arr_x, 16); 464: return (res=res, rsize=rsize); 465: } 466: let (local arr_x: felt*) = alloc(); 467: dict_to_array{dict_ptr=x}(arr_x, 16); 468: let (buf, bufsize) = compress(buf, bufsize, arr_x, 16); … we see that the if (next_block == false) executes one block of code in case len >= 55, and the other otherwise.

If we compare this check with the equivalent implementation in Go (used by Geth), for example:

if tc% 64 < 56 { d.

Write (tmp[ 0:

56 -tc% 64 ]) } else { d.

Write (tmp[ 0:

64 + 56 -tc% 64 ]) } We see that this time, the selection is made with len < 56 as discriminator; if we imagine swapping the if and the else blocks of the Go implementation, the condition becomes len >= 56.

So for the edge case of len == 55, Kakarot and Geth will take different actions, which,

len here represents the length of the input modulo 64, hence the RIPEMD-160 precompile will yield wrong outputs for inputs of length 55 + k*64.

Because this precompile is a cryptographic hash function, it can be used in many ways by EVM applications, including access control on funds.

## Recommended Mitigation Steps

Change the check at L455 to is_nn_le(56, len).

ClementWalter (Kakarot) confirmed and commented:

Severity: High Kakarot mitigated:

This PR fixes off by one error ripemd-160.

Status:

Mitigation confirmed.

# [H-06] Three valid signatures for the same message

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Finding ID:** H-06
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-09-kakarot
- **Source snapshot:** competitions/2024-09-kakarot/final_report.html

Submitted by fyamf, also found by 0xastronatey, Davymutinda77, and AshutoshSB On Ethereum, it’s possible to create two different valid signatures for the same message (known as ECDSA Signature Malleability). However, on Kakarot, you can create three different valid signatures for the same message. This happens because, on Ethereum, if the value of s is higher than the maximum valid range, it returns address(0). On Kakarot, in this case, it returns a valid address.

# [M-01] RIPEMD160 precompile crashes with a Cairo exception for some input lengths

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-09-kakarot
- **Source snapshot:** competitions/2024-09-kakarot/final_report.html

RIPEMD160 precompile crashes with a Cairo exception for some input lengths Submitted by muellerberndt, also found by RadiantLabs, gumgumzum, and ahmedaghadi Calling the RIPEMD160 precompile with certain input lengths results in a Cairo exception. As a result, some L2 smart contracts that use this precompile cannot be executed.

The bug seems to occur in line 477 of the precompile. The relevant code:

func finish{range_check_ptr, bitwise_ptr: BitwiseBuiltin*}( buf: felt*, bufsize: felt, data: felt*, dsize: felt, mswlen: felt ) -> (res: felt*, rsize: felt) { alloc_locals; let (x) = default_dict_new(0); tempvar start = x; (...) let (local arr_x: felt*) = alloc(); dict_to_array{dict_ptr=x}(arr_x, 16); let (buf, bufsize) = compress(buf, bufsize, arr_x, 16); // reset dict to all 0.

let (x) = default_dict_new(0); dict_write{dict_ptr=x}(14, val); dict_write{dict_ptr=x}(15, val_15); let (local arr_x: felt*) = alloc(); dict_to_array{dict_ptr=x}(arr_x, 16); default_dict_finalize(start, x, 0); let (res, rsize) = compress(buf, bufsize, arr_x, 16); return (res=res, rsize=rsize); The lower part of the code is reached for some input lengths. Note that x is redefined with the line:

let (x) = default_dict_new(0); However, start still points to the first element of the original dict x initialized at the start of the function. Consequently, squashing the dict with default_dict_finalize(start, x, 0) will fail because start and x point to different segments.

## Recommended Mitigation Steps

Set the pointer start to the correct value. It is also very important to finalize the previously initialized dict x as the prover can cheat otherwise.

Double-check the invocation path of the precompile exec_precompile(), and add end-to-end tests for all precompiles to make sure that they work as expected when called from L2.

## Assessed type

Error ClementWalter (Kakarot) confirmed and commented:

Severity: Medium Ok, but assets are not directly at risk considering that the tx will revert.

LSDan (judge) decreased severity to Medium Kakarot mitigated:

PR fix: finalize dictionary in RIPEMD160.

Status:

Mitigation confirmed.

# [M-02] Address aliasing is wrongfully applied even to EOAs

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-09-kakarot
- **Source snapshot:** competitions/2024-09-kakarot/final_report.html

Submitted by Bauchibred, also found by minglei-wang-3570, ulas, and RadiantLabs Kakarot inherits the address aliasing logic from Optimism as hinted in AddressAliasHelper.sol:

// from https://github.com/ethereum-optimism/optimism/blob/a080bd23666513269ff241f1b7bc3bce74b6ad15/packages/contracts-bedrock/src/vendor/AddressAliasHelper.sol pragma solidity ^ 0.8.

0; library AddressAliasHelper {..

snip } Now this is a key property of the Optimism bridge, which is that all contract addresses are aliased. This is done in order to avoid a contract on L1 to be able to send messages as the same address on L2, because often these contracts will have different owners.

To go into more details about why & how this is used, please review here.

Address aliasing is an important security feature that impacts the behavior of transactions sent from L1 to L2 by smart contracts. Make sure to read this section carefully if you are working with cross-chain transactions. Note that the CrossChainMessenger contracts will handle address aliasing internally on your behalf.

When transactions are sent from L1 to L2 by an Externally Owned Account, the address of the sender of the transaction on L2 will be set to the address of the sender of the transaction on L1. However, the address of the sender of a transaction on L2 will be different if the transaction was triggered by a smart contract on L1.

Because of the behavior of the CREATE opcode, it is possible to create a contract on both L1 and on L2 that share the same address but have different bytecode. Even though these contracts share the same address, they are fundamentally two different smart contracts and cannot be treated as the same contract. As a result, the sender of a transaction sent from L1 to L2 by a smart contract cannot be the address of the smart contract on L1 or the smart contract on L1 could act as if it were the smart contract on L2 (because the two contracts share the same address).

To prevent this sort of impersonation, the sender of a transaction is slightly modified when a transaction is sent from L1 to L2 by a smart contract. Instead of appearing to be sent from the actual L1 contract address, the L2 transaction appears to be sent from an “aliased” version of the L1 contract address. This aliased address is a constant offset from the actual L1 contract address such that the aliased address will never conflict with any other address on L2 and the original L1 address can easily be recovered from the aliased address.

This change in sender address is only applied to L2 transactions sent by L1 smart contracts. In all other cases, the transaction sender address is set according to the same rules used by Ethereum.

Transaction Source Sender Address L2 user (Externally Owned Account) The user’s address (same as in Ethereum) L1 user (Externally Owned Account) The user’s address (same as in Ethereum) L1 contract (using OptimismPortal.depositTransaction) L1_contract_address + 0x1111000000000000000000000000000000001111 That’s to say we expect this aliasing logic to:

Only be applied to contracts.

However, the issue is that Kakarot applies this aliasing logic not only on contracts but even EOAs, see here:

function sendMessageToL2 ( address to, uint248 value, bytes calldata data ) external payable { uint256 totalLength = data.

length + 4; uint256 [] memory convertedData = new uint256 []( totalLength ); convertedData [ 0 ] = uint256 ( uint160 ( AddressAliasHelper.

applyL1ToL2Alias ( msg.

sender ))); convertedData [ 1 ] = uint256 ( uint160 ( to )); convertedData [ 2 ] = uint256 ( value ); convertedData [ 3 ] = data.

length; for ( uint256 i = 4; i < totalLength; ++ i ) { convertedData [ i ] = uint256 ( uint8 ( data [ i - 4 ])); } starknetMessaging.

sendMessageToL2 {value:

msg.

value }( kakarotAddress, HANDLE_L1_MESSAGE_SELECTOR, convertedData ); } Evidently, the classic contract check of msg.sender != tx.origin is missing which means even when the caller is an EOA it’s still get aliased.

## Impact

Broken functionality for aliasing senders considering even EOAs get aliased instead of it just being restricted to smart contracts; i.e., if developers implement access control checks on the sender of the transactions, say a deposit tx for e., they would have to believe that the address of the contract from the L1 is not aliased when it is an EOA. However, in Kakarot’s case, it is. Therefore, it would result in unintentional bugs where the access control will be implemented incorrectly and these transactions will always fail.

## Recommended Mitigation Steps

Apply an if msg.sender != tx.origin check instead and in the case where this ends up being true then alias the address otherwise let it be.

## Assessed type

Context ClementWalter (Kakarot) confirmed and commented:

Severity: Low There is no security risk of any kind. But this will be fixed.

LSDan (judge) commented:

I think this fits as a medium given that functionality is unexpectedly impaired/blocked.

obatirou (Kakarot) commented:

Following

- https://docs.code4rena.com/awarding/judging-criteria/severity-categorization

# [M-03] No way to cancel l1 -< l2 messages

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-09-kakarot
- **Source snapshot:** competitions/2024-09-kakarot/final_report.html

l1 -< l2 messages Submitted by 1AutumnLeaf777, also found by Bauchibred, wasm_it, RadiantLabs, and oakcobalt There is no api to allow cancellation of l1->l2 messages. In the event of an issue in the Kakarot contracts, this will result in the fee being permanently lost since the user has no ability to reclaim the funds.

- https://github.com/kkrt-labs/kakarot/blob/7411a5520e8a00be6f5243a50c160e66ad285563/solidity_contracts/src/L1L2Messaging/L1KakarotMessaging.sol#L26-L61
As we can see there are only functions to either send the message from l1 -> l2 or consume an l2 message. There is no cancelL1toL2Message present.

## Recommended Mitigation Steps

Introduce the following API to let users cancel their messages after waiting the time limit so that they can reclaim funds.

- https://docs.starknet.io/architecture-and-concepts/network-architecture/messaging-mechanism/#l2-l1_message_cancellation

## Assessed type

Context ClementWalter (Kakarot) confirmed and commented:

Severity: Low Only fees would be lost as it is not a bridge. No other funds at risk. But cancellation will be implemented.

LSDan (judge) commented:

Fee loss is still a value leak/loss of funds. Medium is appropriate.

obatirou (Kakarot) commented:

As per the doc, see

- https://docs.code4rena.com/awarding/judging-criteria/severity-categorization#loss-of-fees-as-low
loss of fees is a LOW.

LSDan (judge) commented:

Loss of fees should be regarded as an impact similar to any other loss of capital Context: I helped write this rule as one of the 3 SC judges that agreed on it.

Loss of unmatured yield or yield in motion shall be capped to medium severity.

The intent is that loss of fees is not higher than medium, even if they are substantial yield rewards.

Loss of dust amounts are QA Only loss of dust is specifically stated as low.

Loss of real amounts depends on specific conditions and likelihood considerations.

The condition above is very likely, making this a valid medium.

Kakarot mitigated:

PR 1623 and lib PR 12 removed messaging.

Status:

Mitigation confirmed.

# [M-04] decode_legacy_tx allows validation of signatures with chain_id that are larger than felt, and overflows

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-09-kakarot
- **Source snapshot:** competitions/2024-09-kakarot/final_report.html

decode_legacy_tx allows validation of signatures with chain_id that are larger than felt, and overflows Submitted by Emmanuel, also found by 20centclub In decode_legacy_tx, data_len for chain_id is not constrained to be <31, which allows validation of signatures with chain_id that are larger than felt, and overflows.

## Recommended Mitigation Steps

Assert that items\[6].data_len <= 31:

assertassert_nn(31 - items[6].data_len);

## Assessed type

Under/Overflow ClementWalter (Kakarot) confirmed and commented:

Severity: Medium Kakarot mitigated:

This PR decodes legacy chain id overflow. Assert the chain id does not overflow for legacy post eip 155 tx.

Status:

Mitigation confirmed.

# [M-05] ExponentiationImpl::pow() returns 0 for 0^0

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-09-kakarot
- **Source snapshot:** competitions/2024-09-kakarot/final_report.html

ExponentiationImpl::pow() returns 0 for 0^0 Submitted by RadiantLabs, also found by Bauchibred The ExponentiationImpl::pow() function in math.cairo incorrectly returns 0 when computing 0^0, instead of the mathematically accepted value of 1. This breaks a fundamental mathematical convention that is relied upon in many mathematical contexts, including polynomial evaluation, Taylor series, and combinatorial calculations.

The issue occurs because the function first checks if the base is zero and returns zero if true, without considering the special case where the exponent is also zero. This early return means that 0^0 evaluates to 0 instead of 1:

fn pow ( self: T, mut exponent: T) -> T { let zero = Zero::

zero (); if self.

is_zero () { return zero; }...

The mathematical definition of 0^0 = 1 is not arbitrary. It is the natural definition that makes many mathematical formulas and theorems work correctly. For example, this definition is necessary for:

The binomial theorem to work correctly when x=0.

Power series expansions to be valid at x=0.

Combinatorial formulas involving empty sets.

Preserving continuity in certain mathematical limits.

This function is not currently being used to compute 0^0 in the code in scope. However, given the critical nature of the function and fundamental incorrectness of its output, the expectation of this issue causing vulnerabilities in future code is fulfilled.

## Impact

Mathematical operations that rely on the standard convention of 0^0 = 1 will produce incorrect results.

Future code that reaches this case in core Kakarot contracts, protocols built on top of Kakarot’s codebase or borrowing from it will experience material errors when processing edge cases

## Recommended Mitigation Steps

Add a check for the 0^0 case before checking if the base is zero:

fn pow(self: T, mut exponent: T) -> T { // Handle 0^0 case first if self.is_zero() && exponent.is_zero() { return One::one(); } // Rest of the existing function...

if self.is_zero() { return Zero::zero(); } //...

} This change preserves the mathematically correct behavior while maintaining all other functionality of the power function.

ClementWalter (Kakarot) confirmed and commented:

Severity: Medium Kakarot mitigated:

PR 1579 and ssj PR 1022 fixes pow in SSJ.

Status:

Mitigation confirmed.

# [M-06] Reentrancy check in account_contract can be easily circumvented

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-09-kakarot
- **Source snapshot:** competitions/2024-09-kakarot/final_report.html

account_contract can be easily circumvented Submitted by RadiantLabs, also found by muellerberndt and zhaojie The account_contract has a reentrancy check in execute_starknet_call to prevent calls made from the Kakarot EVM to Starknet to re-enter the Kakarot EVM.

The check is implemented by checking that execute_starknet_call calls Kakarot’s address only with the get_starknet_address getter, which is indeed harmless:

File: account_contract.cairo 332: @external 333: func execute_starknet_call{syscall_ptr: felt*, pedersen_ptr: HashBuiltin*, range_check_ptr}( 334: to: felt, function_selector: felt, calldata_len: felt, calldata: felt* 335: ) -> (retdata_len: felt, retdata: felt*, success: felt) { 336: Ownable.assert_only_owner(); 337: let (kakarot_address) = Ownable.owner(); 338: let is_get_starknet_address = Helpers.is_zero( 339: GET_STARKNET_ADDRESS_SELECTOR - function_selector 340: ); 341: let is_kakarot = Helpers.is_zero(kakarot_address - to); 342: tempvar is_forbidden = is_kakarot * (1 - is_get_starknet_address); 343: if (is_forbidden != FALSE) { 344: let (error_len, error) = Errors.kakarotReentrancy(); 345: return (error_len, error, FALSE);

346: } 347: let (retdata_len, retdata) = call_contract(to, function_selector, calldata_len, calldata); 348: return (retdata_len, retdata, TRUE); 349: } 350:

However, this check leaves another possibility open, that is that the account_contract could call itself or another account_contract with a signed transaction to re-enter the Kakarot EVM, which is extremely vulnerable to reentrancy because of its extensive use of cached data.

While an exploit via this path can have critical impact on the integrity of the EVM, its likelihood is extremely low because execute_starknet_call is accessible only via whitelisted contracts.

## Recommended Mitigation Steps

Consider removing the reentrancy check in account_contract, and adding a reentrancy guard on the Kakarot.eth_call function.

ClementWalter (Kakarot) confirmed and commented:

Severity: Medium Re-entrancy is actually possible should the attacker be in possession of the signed tx before this tx is actually executed, and can front run it to store it is starknet before it’s processed. It’s possible, though seems very unlikely. We’ll fix the re-entrancy check to put in in the Kakarot contract level.

obatirou (Kakarot) commented:

On second thoughts because of the whitelisting it is impossible to exploit. The only way to access the execute_starknet_call is through the cairo_precompile but this precompile is whitelisted. At the specified commit for C4, only the DualVMToken contract is whitelisted which cannot lead to the exploit.

We will still do the mitigation with a re-entrancy check at Kakarot contract level.

LSDan (judge) commented:

I’m not willing to jump to “whitelisting makes this impossible to exploit” and still think medium applies. Reentrancy is no joke and a hand wavy hypothetical is enough in this case.

ahmedaghadi (warden) commented:

@LSDan - As attack is only possible through “whitelisting”, doesn’t it comes under Centralization Risks ?

LSDan (judge) commented:

Not in this case. The potential for damage is enough for me to keep it in place as a medium.

Kakarot mitigated:

In PR 1582 the reentrancy check is moved at Kakarot level in eth_call.

Status:

Mitigation confirmed.

# [M-07] Account contract does not gracefully handle panics in called contracts

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-09-kakarot
- **Source snapshot:** competitions/2024-09-kakarot/final_report.html

Submitted by RadiantLabs, also found by oakcobalt The DualVmToken is a utility EVM contract allowing Kakarot EVM accounts to operate Starknet ERC-20s via classic balanceOf(), transfer() etc. operations.

Calls to the DualVmToken contract are routed via CairoLib -> kakarot_precompiles.cairo -> account_contract.cairo, which finally makes a call to the call_contract() Cairo 0 syscall to invoke the appropriate Starknet ERC-20 contract.

If we look at how the execute_starknet_call() function in account_countract.cairo is implemented:

File: account_contract.cairo 332: @external 333: func execute_starknet_call{syscall_ptr: felt*, pedersen_ptr: HashBuiltin*, range_check_ptr}( 334: to: felt, function_selector: felt, calldata_len: felt, calldata: felt* 335: ) -> (retdata_len: felt, retdata: felt*, success: felt) { 336: Ownable.assert_only_owner(); 337: let (kakarot_address) = Ownable.owner(); 338: let is_get_starknet_address = Helpers.is_zero( 339: GET_STARKNET_ADDRESS_SELECTOR - function_selector 340: ); 341: let is_kakarot = Helpers.is_zero(kakarot_address - to); 342: tempvar is_forbidden = is_kakarot * (1 - is_get_starknet_address); 343: if (is_forbidden != FALSE) { 344: let (error_len, error) = Errors.kakarotReentrancy(); 345: return (error_len, error, FALSE);

346: } 347: let (retdata_len, retdata) = call_contract(to, function_selector, calldata_len, calldata); 348: return (retdata_len, retdata, TRUE); We see that the function admits graceful failures (by returning a success: felt value that can be FALSE ), which are then properly handled in the calling precompile. However, the actual call to the target contract (at L347) does not return a success boolean, and instead uses the call_contract syscall which panics on failures.

This means that any call that panics in the called contract can cause a revert at RPC level.

To illustrate how this is not a hypothetical scenario, but a very likely one that can be achieved also with the DualVmToken contract that is in scope and consequently planned to be whitelisted to make this call, we make a simple example:

an EVM contract uses DualVmToken to transfer more tokens than what it has in its balance In this scenario, standard OpenZeppelin ERC-20 implementations panic, bubbling up the error up to the Kakarot RPC level.

This also means that most of the ERC-20 behaviours Kakarot intends to support will lead to reverts at the RPC level if encountered. The following table shows the effect of each behaviour if encountered in a call to a DualVmToken:

Feature Status Missing return values Supported Upgradeability - Pausability RPC-level reverts Revert on approval to zero address RPC-level reverts Revert on zero value approvals RPC-level reverts Revert on zero value transfers RPC-level reverts Revert on transfer to the zero address RPC-level reverts Revert on large approvals and/or transfers RPC-level reverts Doesn’t revert on failure Unsupported, see separate finding Blocklists RPC-level reverts

## Impact

Any contract can use the above “over-transfer” call to cause RPC-level crashes in the Kakarot EVM, regardless of how defensively they were called. Protective measures generally considered safe in the EVM space like ExcessivelySafeCall (which is used in LayerZero cross-chain applications to prevent channel DoSes that can permanently freeze in-flight tokens), can be bypassed by causing a RPC-level cairo revert.

More generally, any transaction containing a call to a reverting DualVmToken at any depth will revert. This means contracts whose logic requires such a call to succeed will be temporarily (if the call eventually succeeds) or permanently bricked.

## Recommended Mitigation Steps

The new AccountContract Cairo 1 implementation uses the Cairo 1 call_contract syscall, which offers a recoverable error interface through a nopanic signature returning a Result. If it is not possible to deploy the new version, the Cairo 1 syscall can be used from the Cairo 0 account_contract via a library call as is done with other functionality.

ClementWalter (Kakarot) confirmed and commented:

Severity: Informative This is a known limitation of starknet. It is not a problem as relayer can decide to not relay the tx.

LSDan (judge) decreased severity to Medium and commented:

I find it hard to look at this as anything other than an impact to expected functionality/availability of functionality, making the severity medium. As shown above, this is likely to be encountered often with very standard functionality, much of which is included in OZ EVM implementations of common token patterns. The potential for novel uses of this dynamic to cause hypothetical exploits or denial of service attacks doesn’t seem like something that should be ignored or written off to a “limitation of starknet”.

obatirou (Kakarot) commented:

To summarize:

There is no way to handle a failed syscall in starknet currently. The whole tx panics and stops.

DualVM token are starknet token under the hood.

As such, patterns such as the excessively safe call mentioned by @EV_om in #48 are actually not working as expected.

A possible mitigation for the DualVMToken especially is to reimplement all the validation logic in solidity and perform the cairo calls only if it will surely succeed.

Let’s keep it Medium.

Kakarot mitigated:

This PR uses a call to the Cairo1Helpers class to access the “new” call_contract syscall, that will in the future have the ability to gracefully handle failed contract calls.

Status:

Mitigation confirmed.

# [M-08] DualVmToken can be abused to cause RPC-level reverts by revoking native token approval to Kakarot

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Finding ID:** M-08
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-09-kakarot
- **Source snapshot:** competitions/2024-09-kakarot/final_report.html

DualVmToken can be abused to cause RPC-level reverts by revoking native token approval to Kakarot Submitted by RadiantLabs, also found by minglei-wang-3570 The DualVmToken is a utility EVM contract allowing Kakarot EVM accounts to operate Starknet ERC-20s via classic balanceOf(), transfer(), etc., operations.

These classic operations are offered in two flavors, one that accepts Starknet addresses ( uint256 ) and one that accepts EVM addresses ( address ). In case an EVM address is provided, the corresponding Starknet address is looked up through Kakarot first.

If we look at the approve(uint256,...) function in DualVmToken:

File:

DualVmToken.

sol 206:

/// @dev Approve a starknet address for a specific amount 207:

/// @param spender The starknet address to approve 208:

/// @param amount The amount of tokens to approve 209:

/// @return True if the approval was successful 210:

function approve ( uint256 spender, uint256 amount ) external returns ( bool ) { 211:

_approve ( spender, amount ); 212:

emit Approval ( msg.

sender, spender, amount ); 213:

return true; 214: } 215:

216:

function _approve ( uint256 spender, uint256 amount ) private { 217:

if ( spender >= STARKNET_FIELD_PRIME ) { 218:

revert InvalidStarknetAddress (); 219: } 220:

// Split amount in [low, high] 221:

uint128 amountLow = uint128 ( amount ); 222:

uint128 amountHigh = uint128 ( amount >> 128 ); 223:

uint256 [] memory approveCallData = new uint256 []( 3 ); 224:

approveCallData [ 0 ] = spender; 225:

approveCallData [ 1 ] = uint256 ( amountLow ); 226:

approveCallData [ 2 ] = uint256 ( amountHigh ); 227:

228:

starknetToken.

delegatecallCairo ( "approve", approveCallData ); 229: } We can see that no check whatsoever is done on the spender input, except for felt overflow at L217.

This means that the provided address could be any Starknet account, including a contract that is not an EVM account, and most importantly including the Kakarot contract.

This is particularly relevant because native token transfers in the Kakarot EVM work under the assumption that Starknet account contracts have infinite approval granted to the Kakarot contract, as we can see from the account_contract initialization:

File: library.cairo 083: func initialize{ 084: syscall_ptr: felt*, 085: pedersen_ptr: HashBuiltin*, 086: range_check_ptr, 087: bitwise_ptr: BitwiseBuiltin*, 088: }(evm_address: felt) { --- 101: IERC20.approve(native_token_address, kakarot_address, infinite); By removing this approval and attempting to move native tokens, an EVM contract account can jeopardize EVM state finalization (that is where native token transfers are settled on the Starknet ERC20) and cause RPC-level crashes in the Kakarot EVM, regardless of how defensively they were called. Protective measures generally considered safe in the EVM space like ExcessivelySafeCall (which is used in LayerZero cross-chain applications to prevent channel DoSes that can permanently freeze in-flight tokens), can be bypassed by causing a RPC-level Cairo revert.

## Recommended Mitigation Steps

Consider adding the following check to the DualVmToken.approve function:

require ( spender != kakarot ); ClementWalter (Kakarot) confirmed and commented:

Severity: Low This is a low issue as there is no clear external attack path. A user would need to decide to disabling its own account by making an approval to kakarot with 0.

LSDan (judge) decreased severity to Medium and commented:

Availability and expected functionality are impacted but there are no funds at risk as far as I can see. This can be used for hypothetical griefing attacks.

EV_om (warden) commented:

@LSDan - just to explain our High severity assessment for this finding, as well as #40, #49, #52 and #53, which share the root cause of RPC-level reverts that is very specific to this codebase.

The scenario we had in mind (which we could admittedly have made a better job of explaining) was that in which a contract requires a call to be made to reach a subsequent state. For example: a bridge contract that requires messages to be processed sequentially. It is ok for the call not to succeed, but it must be attempted.

In this situation, the call reverting at the RPC level leads to the transaction never succeeding, and hence, all funds in the contract being locked irreversibly.

This pattern is quite widely used, such as in LayerZero’s _blockingLzReceive, and is not safe on Kakarot, with the highest impact being the permanent freezing of funds.

LSDan (judge) commented:

Ruling stands. You’re relying on handwavy hypotheticals and external factors. Medium is correct.

Kakarot mitigated:

This PR checks kakarot approval dualVMToken.

Status:

Mitigation confirmed.

# [M-09] Jump in creation code leads to reverting of the starknet transaction

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Finding ID:** M-09
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-09-kakarot
- **Source snapshot:** competitions/2024-09-kakarot/final_report.html

Submitted by ahmedaghadi, also found by gumgumzum If the creation code consists of JUMP or JUMPI opcode for offset less than creation code length, then the starknet transaction reverts.

The evm.cairo::jump function is called whenever the JUMP or JUMPI opcode is executed. This function is as follows:

// @notice Update the program counter.

// @dev The program counter is updated to a given value. This is only ever called by JUMP or JUMPI.

// @param self The pointer to the execution context.

// @param new_pc_offset The value to update the program counter by.

// @return model.EVM* The pointer to the updated execution context.

func jump{syscall_ptr: felt*, pedersen_ptr: HashBuiltin*, range_check_ptr, state: model.State*}( self: model.EVM*, new_pc_offset: felt ) -> model.EVM* { -> let out_of_range = is_nn(new_pc_offset - self.message.bytecode_len); if (out_of_range != FALSE) { let (revert_reason_len, revert_reason) = Errors.invalidJumpDestError(); let evm = EVM.stop(self, revert_reason_len, revert_reason, Errors.EXCEPTIONAL_HALT); return evm; } let valid_jumpdests = self.message.valid_jumpdests; with valid_jumpdests { -> let is_valid_jumpdest = Internals.is_valid_jumpdest( self.message.code_address, new_pc_offset ); } //...

} It can be seen that, it checks for out_of_range which is new_pc_offset - self.message.bytecode_len. For the creation code, technically bytecode_length should be 0 but here, it will consist the length of the creation code. So, if the new_pc_offset is less than the creation code length, then this check would pass and then it checks for is_valid_jumpdest by calling evm.cairo::is_valid_jumpdest, which is as follows:

func is_valid_jumpdest{ syscall_ptr: felt*, pedersen_ptr: HashBuiltin*, range_check_ptr, valid_jumpdests: DictAccess*, state: model.State*, }(code_address: model.Address*, index: felt) -> felt { alloc_locals; let (is_cached) = dict_read{dict_ptr=valid_jumpdests}(index); if (is_cached != 0) { return TRUE; } // If the account was created in the same transaction, // a cache miss is an invalid jumpdest as all valid jumpdests were cached on deployment.

let code_account = State.get_account(code_address.evm); if (code_account.created != 0) { return FALSE; } -> let (is_valid) = IAccount.is_valid_jumpdest(code_address.starknet, index); dict_write{dict_ptr=valid_jumpdests}(index, is_valid); return is_valid; } Here, let (is_valid) = IAccount.is_valid_jumpdest(code_address.starknet, index); would revert the whole starknet transaction as it will call zero address which isn’t IAccount contract. So any contract using CREATE or CREATE2 opcode will be vulnerable to this issue or if a contract makes a callback to another contract by making sure to only send limited gas and handle the revert case properly, the other contract can use CREATE or CREATE2 opcode to make the transaction revert forcefully.

# [M-10] Incorrect totalsupply value will be returned due to erroneous return data decode implementation

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Finding ID:** M-10
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-09-kakarot
- **Source snapshot:** competitions/2024-09-kakarot/final_report.html

totalsupply value will be returned due to erroneous return data decode implementation Submitted by oakcobalt, also found by RadiantLabs and gumgumzum The returned value of totalSupply() in a starknet ERC20 contract is expected to fit in uint256, which is expressed in (uint128, uint128) with the first uint128 representing the lower 128bits.

The issue is current implementation of DualVMToken::totalSupply incorrectly decodes returnData as uint256 instead of (uint128, uint128). Because the first uint128 is the lower 128bits of a uint256 number. This means, totalSupply will return an incorrect value because it only reads the lower 128bits.

//kakarot/solidity_contracts/lib/kakarot-lib/src/CairoLib.sol function totalSupply () external view returns ( uint256 ) { bytes memory returnData = starknetToken.

staticcallCairo ( "total_supply" ); |> return abi.

decode ( returnData, ( uint256 )); }

- https://github.com/kkrt-labs/kakarot/blob/7411a5520e8a00be6f5243a50c160e66ad285563/solidity_contracts/src/CairoPrecompiles/DualVmToken.sol#L82
Impacts totalSupply will return incorrect value. Due to totalSupply is critical in many defi or accounting based logic, this potentially leads to fund loss and errorneas accounting in any user application that uses DualVMToken.sol. Depending on the context of the application that calls DualVMToken.sol, the fund loss could be critical.

## Recommended Mitigation Steps

In totalSupply(), change the decode following _balanceOf ’s implementation:...

( uint128 valueLow, uint128 valueHigh ) = abi.

decode ( returnData, ( uint128, uint128 ) ); return uint256 ( valueLow ) + ( uint256 ( valueHigh ) << 128 );

## Assessed type

Error ClementWalter (Kakarot) confirmed and commented:

Severity: Low LSDan (judge) commented:

Medium is appropriate here. Receiving an incorrect value for TotalSupply can cause untold amounts of hand wavy hypothetical misery.

ClementWalter (Kakarot) commented:

Ok for Medium.

2 — Med: Assets not at direct risk, but the function of the protocol or its availability could be impacted, or leak value with a hypothetical attack path with stated assumptions, but external requirements.

There might be paths where totalSupply can be used to trigger actions and eventually drain funds.

Kakarot mitigated:

This PR fixes an issue where totalSupply was skipping the high-part of the u256.

Correctly deserialize return value.

Update test to launch a token with populated (low, high) parts of the u256 struct.

Status:

Mitigation confirmed.

# [M-11] handle_l1_message may unfairly revert l2 tx with sufficient l1 sender balance, due to vulnerable fee charge implementation

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Finding ID:** M-11
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-09-kakarot
- **Source snapshot:** competitions/2024-09-kakarot/final_report.html

handle_l1_message may unfairly revert l2 tx with sufficient l1 sender balance, due to vulnerable fee charge implementation Submitted by oakcobalt, also found by muellerberndt and RadiantLabs handle_l1_message can only be invoked by starknet os( @l1_handler ) and is not a regular user invoked transaction from eth_send_raw_unsigned_tx flow.

kakarot::handle_l1_message -> [ library:[handle_l1_message ](

- https://github.com/kkrt-labs/kakarot/blob/7411a5520e8a00be6f5243a50c160e66ad285563/src/kakarot/library.cairo#L421
) -> Interpreter::execute ) handle_l1_message hardcodes EVM gaslimit(2100000000) and gasprice(1) for every L1->L2 message regardless of the complexity of the actual l2 tx. In interpreter::execute, L1sender ’s cached balance will be subtracted with the max_fee (2100000000 x 1) first before performing ETH transfer or running EVM.

Case: L1 sender performs minimal operations on L2:

For an L1 sender who only transfers some ETH to a L2 address or perform simple opcodes, the actual gas cost ( required_gas ) may be very close to the intrinsic gas cost(21000). This means the actual_fee l1sender is required to pay is around 21000 x 1, which is far less than the calculated max_fee 2100000000 x 1.

In this case, interpreter::execute will first subtract max_fee ( 2100000000 ) from L1sender ’s cached balance ( Account.set_balance(sender, &new_balance) ). Note that this cached balance subtraction is done before ETH value transfer and run(evm), which means any subsequent logic will be using L1sender ’s new_balance (e.g., X - 2100000000 ).

//src/kakarot/interpreter.cairo func execute{...

}( env: model.Environment*, address: model.Address*, is_deploy_tx: felt, bytecode_len: felt, bytecode: felt*, calldata_len: felt, calldata: felt*, value: Uint256*, gas_limit: felt, access_list_len: felt, access_list: felt*, ) -> (model.EVM*, model.Stack*, model.Memory*, model.State*, felt, felt) {...

|> let max_fee = gas_limit * env.gas_price; let (fee_high, fee_low) = split_felt (max_fee); let max_fee_u256 = Uint256 (low=fee_low, high=fee_high); with state { let sender = State.

get_account (env.origin); //@audit L1->L2 flow: L1 sender's cached is first subtracted with max_fee based on hardcoded values, regardless of the actual fee required based on L2tx's complexity |> let (local new_balance) = uint256_sub ([sender.balance], max_fee_u256); let sender = Account.

set_balance (sender, &new_balance);...

let transfer = model.

Transfer (sender.address, address, [value]); let success = State.

add_transfer (transfer);...

if (success == 0 ) { let (revert_reason_len, revert_reason) = Errors.

balanceError (); tempvar evm = EVM.

stop (evm, revert_reason_len, revert_reason, Errors.EXCEPTIONAL_HALT); } else { tempvar evm = evm; } with stack, memory, state { let evm = run (evm); } let required_gas = gas_limit - evm.gas_left;...

let actual_fee = total_gas_used * env.gas_price; let (fee_high, fee_low) = split_felt (actual_fee); let actual_fee_u256 = Uint256 (low=fee_low, high=fee_high); let transfer = model.

Transfer (sender.address, coinbase.address, actual_fee_u256); with state { State.

add_transfer (transfer); State.

finalize (); } return (evm, stack, memory, state, total_gas_used, required_gas);

- https://github.com/kkrt-labs/kakarot/blob/7411a5520e8a00be6f5243a50c160e66ad285563/src/kakarot/interpreter.cairo#L950-L951

## Impact

L1sender ’s L2 tx can be unfairly reverted when they have enough balance to cover transfer value and the actual fee.

Because handle_l1_message is not invoked by RPC endpoint users, L1sender doesn’t have a chance to specify max_fee value. In such cases, L1sender might only reasonably expect to have the EVM gas cost sufficient based on the complexity of the L2 tx, which can be much lower than max_fee. Although max_fee is marginal in ETH value, L1sender ’s L2 tx can still fail due to the sequence of fee deduction in the cached state. The failure is not the L1sender ’s fault.

## Recommended Mitigation Steps

Because handle_l1_message is different from eth_send_raw_unsigned_tx in its invocation and max_fee setting, consider allowing L1sender to input gas_limit from L1, and decode the user input gas_limit in handle_l1_message to compute max_fee. Or, refactor the control flow to skip max fee deduction when called from handle_l1_message.

ClementWalter (Kakarot) confirmed and commented:

Severity: Medium Ok, the gas price for handle_l1_message should be 0.

Kakarot mitigated:

PR 1584 fix:

handle_l1_message gas price is set to 0; superseded by PR 1623 fix: remove messaging and PR 12 fix: remove messaging Status:

Mitigation error. Full details in reports from RadiantLabs included in the
