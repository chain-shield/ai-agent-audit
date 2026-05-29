# Benchmark Ground Truth: Kakarot

## Accepted H/M Findings

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

## Rejected Primary Findings

# Rejected Primary Findings: Kakarot

# `execute_from_outside()` does not check if an account has been initialized, allowing an attacker to impersonate transactions from the Ethereum zero address on Kakarot.

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-271
- **Submitter:** 0xTonraq
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/271
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-271.md

## Brief Summary

The `execute_from_outside()` does not check if an account instance has been initialised, hence the `Account_evm_address` storage_var defaults to `0`, which when read by `execute_from_outside()` would default to `0x0...`, which is the Ethereum `zero/burn` address, therefore an attacker can successfully submit transactions made by the Ethereum `zero/burn` address. Vulnerability details Kakarot privides a mechanism for validating and executing Ethereum transactions through the `execute_from_outside()` functions present in both `src/kakarot/accounts/account_contract.cairo` and `src/kakarot/accounts/library.cairo` (core logic), These functions contains a series of validation checks and verificat...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Potential overflow in the function is_account_alive()

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-194
- **Submitter:** 0xtomsmith
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/194
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-194.md

## Brief Summary

The function `is_account_alive()` may incorrectly return `FALSE` if an overflow happens on line 427 inside state.cairo: If the calculation on line 427 `nonce + code_len + balance.low + balance.high` overflows and results in 0, then the function incorrectly returns `FALSE` due to the overflow. As a result the protocol may proceed in doing wrong calculations. For example system_operations.cairo is using `State.is_account_alive()` to determine gas costs. Also inside system_operations.cairo the function `exec_selfdestruct()` may calculate incorrect gas costs.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# `selfdestruct` function state modification is incorrect

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-122
- **Submitter:** 13u9
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/122
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-122.md

## Brief Summary

The `kakarot evm` is implemented according to the EIP instruction specifications. However, the `selfdestruct` instruction of `kakarot evm` does not adhere to the specification.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_16_group

# Redeployment to the same address is impossible using `CREATE2` in the `exec_create` instruction.

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-123
- **Submitter:** 13u9
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/123
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-123.md

## Brief Summary

In `kakarot evm`, the `exec_create` instruction generates the `CREATE` and `CREATE2` operations. The standard EIP `CREATE2` instruction is designed to allow redeployment to a deleted address (an address that has undergone `selfdestruct`). However, in `kakarot evm`, redeployment using `CREATE2` is not possible in the `exec_create` instruction.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# In the case of a non-deploy transaction, the nonce of the `env.origin` EVM account does not increase.

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-134
- **Submitter:** 13u9
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/134
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-134.md

## Brief Summary

In the Kakarot EVM, there are two types of transactions: 1. Transactions that deploy an EVM address. 2. All other transactions. Among these, in case of sending a transaction of type 2, if the transaction is a simple one that sends `value` to a `to` address, rather than containing Kakarot EVM instruction bytecode that increments the `nonce`, the `nonce` of the `evm.address account` of the `sender` does not increase.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Underflow in SSTORE gas refund calculation

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-159
- **Submitter:** 1AutumnLeaf777
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/159
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-159.md

## Brief Summary

Description: When calculating the refunds for `SSTORE`, there is an underflow in the following code: The computation of `gas_refund` can underflow in the following scenario: current_value = 0, new_value != 0, original_value !=0, new_value = original value. This will cause the `gas_refund` value to be equal to `-5000`, which means if the current `evm.gas_refund` is lower than 5000, the addition will underflow. Then in `Interpreter.execute` the gas refund is computed: https://github.com/kkrt-labs/kakarot/blob/7411a5520e8a00be6f5243a50c160e66ad285563/src/kakarot/interpreter.cairo#L1007-L1015, However the following `is_nn` check is bypassed because the gas refund is definitely larger than the m...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Authorized pre eip155 txs will revert

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-162
- **Submitter:** 1AutumnLeaf777
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/162
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-162.md

## Brief Summary

When calling a pre eip155 tx, there is a check to ensure that the msg is whitelisted: https://github.com/kkrt-labs/kakarot/blob/7411a5520e8a00be6f5243a50c160e66ad285563/src/kakarot/accounts/library.cairo#L209-L221 It will then proceed to call the `eth_send_raw_unsigned_tx` function: https://github.com/kkrt-labs/kakarot/blob/7411a5520e8a00be6f5243a50c160e66ad285563/src/kakarot/eth_rpc.cairo#L248-L254 which checks that the chain id is equal to the kakarot chain id. However pre eip155 txs are considered legacy txs: https://github.com/kkrt-labs/kakarot/blob/7411a5520e8a00be6f5243a50c160e66ad285563/src/utils/eth_transaction.cairo#L63-L93 and as a result the chain id will be set to 0 during rlp d...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Origin and extra precompiles are not warmed up when preparing a transaction

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-185
- **Submitter:** 1AutumnLeaf777
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/185
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-185.md

## Brief Summary

The origin along with kakarot and rollup precompiles are not warmed up: https://github.com/kkrt-labs/kakarot/blob/7411a5520e8a00be6f5243a50c160e66ad285563/src/kakarot/interpreter.cairo#L914-L920 This causes users to spend more gas in the case that they either invoke a precompile outside of the ethereum specified precompile list or if they call `tx.origin` in a transaction. This opcode mispricing also deviates from the EVM specification.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Reverted txs in root ctx do not give refunds

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-220
- **Submitter:** 1AutumnLeaf777
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/220
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-220.md

## Brief Summary

If a create tx fails in the root context (for reasons like nonce overflow or more practically the balance being sent is more than allowed) then geth refunds all gas: https://github.com/ethereum/go-ethereum/blob/65e5ca7d8126f7a8c708f8affb64f16c22cc63c0/core/vm/evm.go#L442-L444 (Notice the entire gas is set to `st.GasRemaining`: https://github.com/ethereum/go-ethereum/blob/65e5ca7d8126f7a8c708f8affb64f16c22cc63c0/core/state_transition.go#L454-L461 which is then refunded in `refundGas`: https://github.com/ethereum/go-ethereum/blob/65e5ca7d8126f7a8c708f8affb64f16c22cc63c0/core/state_transition.go#L491-L518) However Kakarot will consider this an `EXCEPTIONAL_HALT` and thus set the gas_left and g...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# max_fee_per_gas is not checked to have a reasonable value potentially introducing overflow

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-142
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/142
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-142.md

## Brief Summary

max_fee_per_gas is not checked to have a reasonable value potentially introducing overflow

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_47_group

# The check for the validity of the init code is incorrect

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-143
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/143
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-143.md

## Brief Summary

The check for the validity of the init code is incorrect

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Gas may not fit into felt value when computing message call gas value

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-144
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/144
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-144.md

## Brief Summary

Gas may not fit into felt value when computing message call gas value

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_04_group

# There are no static gas costs for TLOAD / TSTORE opcodes

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-212
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/212
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-212.md

## Brief Summary

There are no static gas costs for TLOAD / TSTORE opcodes

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Transient storage is not reset at the beginning of transaction execution

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-221
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/221
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-221.md

## Brief Summary

Transient storage is not reset at the beginning of transaction execution

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Access lists are not processed correctly according to the EIP-2929

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-223
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/223
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-223.md

## Brief Summary

Access lists are not processed correctly according to the EIP-2929

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Karakot VM does not follow EIP-3651 making the coinbase payments more expensive for users

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-227
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/227
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-227.md

## Brief Summary

Karakot VM does not follow EIP-3651 making the coinbase payments more expensive for users

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_06_group

# Incompatibility with Smart Contract Account Signatures in Kakarot

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-279
- **Submitter:** AshutoshSB
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/279
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-279.md

## Brief Summary

The Kakarot protocol currently verifies user signatures in a way that supports only basic, single-key accounts. This approach assumes each user account is an Externally Owned Account (EOA), where the user is the only key holder. The issue is that ecrecover can’t verify signatures from smart contract accounts, meaning: Smart Contract Accounts Won't Verify: Without support for EIP-1271, smart contract accounts won’t pass signature checks. EIP-4337 and Account Abstraction are Incompatible: If users have smart contract accounts that follow EIP-4337, their signatures can’t be verified by Kakarot. Impact : This limitation restricts the types of accounts that can interact with Kakarot: Incompatibi...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Memory Overwrite in ` felt_to_ascii` due to Uninitialized Memory Allocation

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-246
- **Submitter:** Auditor2947
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/246
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-246.md

## Brief Summary

The function `felt_to_ascii` casts `asci`i directly from `[fp]` without initializing memory. Without proper allocation, `ascii` may overwrite other memory segments used by other functions, causing unexpected behavior, data corruption, or leaks. Repeated calls to `felt_to_ascii` increase the risk of memory overlap, which could lead to data loss or even crashes. Impact: This bug could lead to memory leaks, crashes, or unexpected program behavior. It also introduces a security risk, as an attacker might manipulate memory content, compromising both function integrity and application stability.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Potential Uninitialized Pointer Usage in `dict_values` Function

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-249
- **Submitter:** Auditor2947
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/249
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-249.md

## Brief Summary

In the `dict_values` function, the `pointer` is cast to `Uint256*` without validating that `dict.new_value `is correctly initialized or points to valid memory. Accessing `pointer[0]` without this verification may result in reading uninitialized memory, leading to unpredictable behavior and potential data integrity issues. Impact Accessing uninitialized memory can produce random or unexpected values, compromising data integrity and causing inconsistent dictionary outputs, which may affect downstream functions.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Assertion on `range_check_ptr` in `decode_type_unsafe` Function

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-250
- **Submitter:** Auditor2947
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/250
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-250.md

## Brief Summary

The assertion in the `decode_type_unsafe` function checks whether `range_check_ptr` equals `string_len`. However, it does not validate the overall consistency of the data. This can lead to situations where `string_len` is inconsistent with `range_check_ptr`, allowing for erroneous data to be accepted without proper validation. Impact Inconsistencies between `range_check_ptr` and `string_len` can cause `false-positive` assertion passes. This flaw could enable invalid data to bypass crucial validation checks, potentially resulting in unpredictable behavior in downstream processes.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Carry Handling in `Addition ` and ` Subtraction ` Functions

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-253
- **Submitter:** Auditor2947
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/253
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-253.md

## Brief Summary

The `uint256_add` and `uint256_sub` functions exhibit incorrect handling of the carry during addition and subtraction operations on the `Uint256` type. When the result of an addition or subtraction exceeds the limits of a 256-bit integer, the carry should be properly propagated from the low part to the high part of the `Uint256` structure. Failure to do so may result in incorrect arithmetic outcomes. Impact: Improper carry handling can lead to erroneous results during arithmetic operations, which may introduce vulnerabilities such as `overflows` or `underflows`. These inaccuracies can affect subsequent calculations and potentially expose contracts to exploitations, especially in scenarios r...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Vulnerability in Out of Gas Error Handling for Memory Operations

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-256
- **Submitter:** Auditor2947
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/256
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-256.md

## Brief Summary

The current implementation does not adequately handle scenarios where the available gas is insufficient to cover the `memory_expansion.cost`. This oversight may result in unexpected `out-of-gas` errors, which can be exploited by malicious actors through the manipulation of memory operations, potentially exhausting the contract's gas. Impact: This vulnerability poses a significant risk as it may lead to `denial-of-service` `(DoS)` attacks. An attacker can intentionally trigger `out-of-gas` conditions, effectively locking out legitimate users and disrupting essential operations of the smart contract. This disruption not only affects user experience but also exposes the contract to further sec...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Duplicate `Event` Definitions

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-257
- **Submitter:** Auditor2947
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/257
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-257.md

## Brief Summary

The contract defines multiple `Transfer` events with the `same name` but differing parameter types. This redundancy can lead to ambiguity regarding which event is emitted, complicating event `logging` and monitoring. Impact: Inaccurate event `logging` may occur, hindering external systems (like `wallets` and `user interfaces`) from accurately tracking token transfers. This confusion can affect user experience and trust in the contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# `saturated_sub` would return wrong data in some instances

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-156
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/156
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-156.md

## Brief Summary

Any ext/internal query to this, i.e `saturated_sub` would have the protocol easily wrongly assume that the answer is wrongly non-negative whereas it is rightly non-negative. _Considering the sort of contest Kakarot is, I assume this should be flagged, as one can consider this functionality to be similar to the native underflow protected subtraction on solidity EVM._

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# SHA-256 precompile's price would be inefficient post EIP 7667

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-158
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/158
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-158.md

## Brief Summary

Completely broken logic in regards to the amount of gas used, i.e: - For small inputs (<32 bytes): Gas cost will increase from 72 to 300. - For medium-sized inputs (32-64 bytes): Gas cost will increase from 84 to 360. - For larger inputs (>64 bytes): Gas cost will increase proportionally. Which then has Kakarot undercharging for these operations and is misaligning with the actual resource consumption on the network, which could affect the economics of applications relying on this precompile. Alternatively this could cause for abuse if users exploit the difference between the reported and actual gas costs.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_35_group

# ECADD/ECMUL's implementation deviates from Ethereum's VM

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-189
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/189
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-189.md

## Brief Summary

ECADD/ECMUL deviates from EIP 196 and is not equivalent to the Ethereum VM. Since in the case the tx to call the precompile fails the gas cost is [assumed to be zero](https://github.com/kkrt-labs/kakarot-ssj/blob/d4a7873d6f071813165ca7c7adb2f029287d14ca/crates/contracts/src/cairo1_helpers.cairo#L142-L149) whereas it should still be paid in full.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_90_group

# `decode_1559()` does not work for transactions with fee market

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-190
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/190
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-190.md

## Brief Summary

Ethereum transactions with fee markets can never be decoded.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_56_group

# ecrecover precompile can never be called

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-196
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/196
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-196.md

## Brief Summary

Some precompiles can't be accessed, in this case ecrecover, which bricks any needed signature logic in the protocol.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Valid transactions like those to create contracts would always fail

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-197
- **Submitter:** Bauchibred
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/197
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-197.md

## Brief Summary

The [check ](https://github.com/kkrt-labs/kakarot/blob/6496ee1d18380dae5b069c27b3129d82ae282419/cairo_zero/utils/utils.cairo#L240) for `"Bytes has length {bytes_len}, expected 0 or 20"` is wrong and instead only enforces that the tx address length is `20` which causes for valid txs to no addresses or addresses of `0` length to always fail.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Reentrancy leading to drainage of user funds or operation disruption

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-203
- **Submitter:** DCENT09
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/203
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-203.md

## Brief Summary

Reentrancy leading to drainage of user funds or operation disruption

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_26_group

# Unrestricted delegatecall that can lead to complete loss of control over the calling contract's state, leading to significant financial and functional damage

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-205
- **Submitter:** DCENT09
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/205
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-205.md

## Brief Summary

Unrestricted delegatecall that can lead to complete loss of control over the calling contract's state, leading to significant financial and functional damage

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_18_group

# Missing Fallback for staticcallCairo which can cause the transaction to fail unexpectedly without clear error handling.

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-206
- **Submitter:** DCENT09
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/206
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-206.md

## Brief Summary

Missing Fallback for staticcallCairo which can cause the transaction to fail unexpectedly without clear error handling.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unchecked Inputs and Malformed Data can lead to Crashes, DoS, or incorrect execution

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-208
- **Submitter:** DCENT09
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/208
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-208.md

## Brief Summary

Unchecked Inputs and Malformed Data can lead to Crashes, DoS, or incorrect execution

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_02_group

# Replay Attacks Across Networks can lead to potential fund loss

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-247
- **Submitter:** DCENT09
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/247
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-247.md

## Brief Summary

Replay Attacks Across Networks can lead to potential fund loss

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Reentrancy Attacks causing state locking, check-effects-interactions pattern

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-251
- **Submitter:** DCENT09
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/251
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-251.md

## Brief Summary

Reentrancy Attacks causing state locking, check-effects-interactions pattern

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_146_group

# Integer Overflow/Underflow can lead to loss of funds, resource exhaustion, incorrect transaction processing

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-255
- **Submitter:** DCENT09
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/255
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-255.md

## Brief Summary

Integer Overflow/Underflow can lead to loss of funds, resource exhaustion, incorrect transaction processing

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_43_group

# Division by Zero could result in a runtime failure or revert the transaction, leading to potential denial-of-service scenarios.

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-263
- **Submitter:** DCENT09
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/263
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-263.md

## Brief Summary

Division by Zero could result in a runtime failure or revert the transaction, leading to potential denial-of-service scenarios.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_54_group

# Overflow in Cairo's PRIME Field could result in incorrect outputs that violate the function’s assumptions.

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-276
- **Submitter:** DCENT09
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/276
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-276.md

## Brief Summary

Overflow in Cairo's PRIME Field could result in incorrect outputs that violate the function’s assumptions.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_79_group

# Lack of Access Control in write_storage Function

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-104
- **Submitter:** Davymutinda77
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/104
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-104.md

## Brief Summary

The code lacks explicit access control mechanisms, allowing any function, such as write_storage, to be invoked by arbitrary users. For example, in the write_storage function: func write_storage{ syscall_ptr: felt*, pedersen_ptr: HashBuiltin*, range_check_ptr, state: model.State* }(evm_address: felt, key: Uint256*, value: Uint256*) { // No access control let account = get_account(evm_address); let account = Account.write_storage(account, key, value); update_account(account); } This function allows users to write to any account's storage without restrictions. Without proper checks, malicious actors could call this function and modify the storage or state of arbitrary accounts, leading to unau...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Unverified Keccak Hash Usage in Ethereum Address Recovery May Lead to Incorrect Address Derivation

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-151
- **Submitter:** Davymutinda77
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/151
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-151.md

## Brief Summary

The PrecompileEcRecover contract relies on the ICairo1Helpers interface to generate Keccak hashes for recovering Ethereum addresses. The Keccak hash is essential in this process as it transforms the public key into the corresponding Ethereum address. However, the hash generation is dependent on external helper functions from the ICairo1Helpers interface. If these external helper functions are not implemented correctly or securely, it can lead to incorrect address recovery or potentially even more severe vulnerabilities. Key Issues: Dependency on external helpers: The library_call_keccak function used to generate the Keccak hash is called through ICairo1Helpers. If this function is insecure...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Data Integrity Risks Due to Misaligned Memory Access in Store and Load Functions

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-183
- **Submitter:** Davymutinda77
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/183
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-183.md

## Brief Summary

In both the store and load functions, if the offset provided is misaligned (i.e., not a multiple of 16 bytes), the functions engage in complex operations involving the splitting and merging of 128-bit values using masking techniques. If these mask operations encounter issues such as rounding errors or overflow, it can lead to: Corrupted Data: Data written to or read from memory may become incorrect or unusable. Unexpected Contract Behavior: Misalignment can lead to unintended outcomes, affecting the integrity of the entire contract. Potential for Exploits: Malicious actors may exploit this vulnerability to manipulate contract data, potentially leading to significant security breaches. Refer...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_52_group

# Performance Degradation Risk Due to Inefficient Memory Finalization in finalize Function

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-184
- **Submitter:** Davymutinda77
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/184
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-184.md

## Brief Summary

The finalize function, responsible for memory compaction or squashing (via default_dict_finalize), does not impose explicit constraints on the size of the memory region it operates on. This could lead to performance degradation, especially in scenarios where the memory size becomes excessively large. Key risks include: Performance Bottlenecks: High memory usage can lead to slow finalization processes, degrading the overall performance of the contract. Denial of Service (DoS): In extreme cases, if memory compaction becomes too resource-intensive, it could result in a denial-of-service vulnerability, making the contract temporarily unavailable or unusable. Unbounded Memory Growth: Without res...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Input Validation in eth_send_raw_unsigned_tx Leading to Potential Overflow and Malleability Attacks

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-198
- **Submitter:** Davymutinda77
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/198
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-198.md

## Brief Summary

In the function eth_send_raw_unsigned_tx, the parameters tx_data_len and tx_data are used without any validation. This lack of validation poses a risk of unexpected behavior, such as overflow attacks, transaction malleability, and the potential to pass malformed or malicious input data. Similar validation issues may also exist in other functions like eth_call, eth_send_transaction, etc. Without proper input validation, malicious actors could pass data of incorrect lengths or formats, causing issues such as: Buffer overflows, leading to unexpected behavior or security vulnerabilities. Malleability of transactions, where attackers could modify transaction data in unintended ways. Invalid or c...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_11_group

# Unchecked Return Values in ERC20 and External Function Calls Could Lead to Loss of Funds and Functionality

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-199
- **Submitter:** Davymutinda77
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/199
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-199.md

## Brief Summary

The functions IERC20.transfer(), IERC20.transferFrom(), IERC20.approve(), and ICairo1Helpers.exec_precompile() return critical success indicators, but their return values are not consistently checked in the code. Failure to verify these return values could result in undetected errors, leading to failed transactions or operations without any visible error or revert, especially during ERC20 token transfers and approvals. This introduces potential vulnerabilities such as: Loss of Funds: If an ERC20 transfer fails but is assumed successful, funds may not reach the intended recipient. Failure to Execute Approvals or Precompile Calls: Unchecked failures in approve() or exec_precompile() can preve...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Potential Misuse of replace_class in Upgrade Function Poses Security Risks

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-95
- **Submitter:** Davymutinda77
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/95
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-95.md

## Brief Summary

The upgrade() function in the Kakarot contract uses the replace_class syscall to upgrade the contract. This operation, while necessary for upgrades, introduces significant risk. If the owner’s private key is compromised or the upgrade process is mishandled, a malicious actor could replace the entire contract code with malicious logic, potentially leading to unauthorized transfers, loss of assets, or a complete contract takeover. Location: kakarot/src/kakarot/kakarot.cairo, Line 42 replace_class(new_class_hash); Note on Testing: Although testing frameworks such as kakarot/tests/utils/syscall_handler.py implement mock environments for replace_class, these do not affect the real risks in the p...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Overflow when calculating memory_expansion_cost via calculate_gas_extend_memory due to error in memory_cost function

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-228
- **Submitter:** Emmanuel
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/228
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-228.md

## Brief Summary

Overflow when calculating memory_expansion_cost via calculate_gas_extend_memory due to error in memory_cost function

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_08_group

# Throughout the codebase, kakarot instructions deals with uint128 offset values, but geth deals with uint64 values.

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-268
- **Submitter:** Emmanuel
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/268
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-268.md

## Brief Summary

Throughout the codebase, kakarot instructions deals with uint128 offset values, but geth deals with uint64 values.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# push_uint128 does not check that element is actually < 2^128, leading to data corruption

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-272
- **Submitter:** Emmanuel
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/272
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-272.md

## Brief Summary

push_uint128 does not check that element is actually < 2^128, leading to data corruption

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Unauthorized Transaction Execution Due to Missing Account Contract Validation

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-243
- **Submitter:** Koolex
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/243
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-243.md

## Brief Summary

Any contract can bypass signature validation by directly calling `eth_send_raw_unsigned_tx`, allowing unauthorized execution of EVM transactions. This breaks the fundamental security model where transactions must be signed by the account owner. The allows: - Execution of transactions without valid signatures - Impersonation of any EVM address as long as nonce matches

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_25_group

# Nonce Increment Inconsistency in Contract Creation Against Geth

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-252
- **Submitter:** Koolex
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/252
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-252.md

## Brief Summary

Critical inconsistency between Kakarot and Geth in nonce handling during contract creation failures. Geth always increments nonce regardless of error type (except a few such as nonce overflow), while Kakarot only increments nonce in limited scenarios. This causes state divergence between Kakarot and Geth, particularly with cases like max code size exceeded.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_59_group

# Potential Infinite Recursion Leading to Gas Exhaustion in Recursive Functions

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-137
- **Submitter:** MrxSnowden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/137
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-137.md

## Brief Summary

Potential Infinite Recursion Leading to Gas Exhaustion in Recursive Functions

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Flawed Byte-to-uint256 Conversion in L1KakarotMessaging Enables Cross-Layer Message Spoofing

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-132
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/132
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-132.md

## Brief Summary

The L1KakarotMessaging contract serves as a bridge for cross-layer communication between Ethereum (L1) and StarkNet (L2) in the Kakarot project. The consumeMessageFromL2 function is responsible for processing messages sent from L2 to L1. The consumeMessageFromL2 function incorrectly converts a structured byte array into an array of uint256 values, resulting in the loss of crucial data structure and corruption of address information.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Contract Existence Checks Lead to Silent Failures in L1-L2 Communication

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-229
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/229
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-229.md

## Brief Summary

The CairoLib's implementation of Cairo contract calls lacks proper existence checks for both precompile contracts on L1 and target contracts on L2 Starknet. Due to EVM's behavior of returning success for calls to non-existent contracts, this could lead to silent failures in cross-chain communication.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Replay Protection in L2 to L1 Communication Enables Message Duplication

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-234
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/234
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-234.md

## Brief Summary

The `sendMessageToL1` function in the `CairoLib` library is vulnerable to replay attacks. This vulnerability arises from the lack of any mechanism to ensure message uniqueness or prevent the resubmission of previously processed messages. An attacker could intercept a legitimate message and replay it to the `CAIRO_MESSAGE_PRECOMPILE` contract, potentially triggering unintended consequences on the L1 chain. The `sendMessageToL1` function simply forwards the provided `payload` to the `CAIRO_MESSAGE_PRECOMPILE` contract without any checks to prevent replay attacks: This stateless message passing mechanism allows an attacker to capture a message and resubmit it later, as the `CAIRO_MESSAGE_PRECO...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_44_group

# Message Duplication and State Inconsistency During StarkNet Reorgs

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-235
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/235
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-235.md

## Brief Summary

Background: The StarkNet Block Reorganization On April 4, 2024, StarkNet experienced a significant block reorganization incident due to a rounding error bug. This event led to transaction backlog and disruption in block production, highlighting vulnerabilities in cross-layer messaging systems. The incident demonstrated that applications leveraging StarkNet's cross-layer messaging must be designed with robust reorg protection mechanisms. Description The CairoLib library implementation contains critical vulnerability in its cross-layer messaging and state management functions (`sendMessageToL1` and `callCairo`). The vulnerability becomes particularly acute during StarkNet block reorganization...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect stack handling for `CREATE` and `CREATE2` Opcodes in `exec_create` function

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-115
- **Submitter:** Rhaydden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/115
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-115.md

## Brief Summary

Incorrect stack handling for `CREATE` and `CREATE2` Opcodes in `exec_create` function

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Uninitialized caller starknet aaddress after account deployment in `cairo_precompile` function

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-71
- **Submitter:** Rhaydden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/71
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-71.md

## Brief Summary

The `caller_starknet_address` is used in the `IAccount.execute_starknet_call()` function call: If `caller_starknet_address` is uninitialized or incorrect, this call could fail or, worse, execute with the wrong caller address, potentially leading to unauthorized actions or other security issues.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# use memory safe in byteArrayToString

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-100
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/100
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-100.md

## Brief Summary

use memory safe in byteArrayToString

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_00_group

# wrong implement of pop_n

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-245
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/245
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-245.md

## Brief Summary

wrong implement of pop_n

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Inadequate Authorization in set_balance

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-165
- **Submitter:** bhatmuneeb
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/165
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-165.md

## Brief Summary

Inadequate Authorization in set_balance

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Unchecked Return Value from dict_write in Multiple Locations

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-169
- **Submitter:** bhatmuneeb
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/169
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-169.md

## Brief Summary

Unchecked Return Value from dict_write in Multiple Locations

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Resource Constraints in fetch_or_create Function

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-170
- **Submitter:** bhatmuneeb
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/170
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-170.md

## Brief Summary

Missing Resource Constraints in fetch_or_create Function

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_89_group

# Potential for Inconsistent Account Storage Initialization

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-171
- **Submitter:** bhatmuneeb
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/171
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-171.md

## Brief Summary

Potential for Inconsistent Account Storage Initialization

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Front-Running Vulnerability in Account Registration

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-175
- **Submitter:** bhatmuneeb
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/175
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-175.md

## Brief Summary

Front-Running Vulnerability in Account Registration

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# "Denial of Service (DoS) Vulnerabilities in `saturated_sub`, `initialize_jumpdests`, and `try_parse_destination_from_bytes` due to Unchecked Return Values, Improper Input Validation, and Potential Infinite Loops"

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-277
- **Submitter:** coders99
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/277
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-277.md

## Brief Summary

- In the `saturated_sub` function, there’s a return value of 0 if the subtraction result is negative. This means that an attacker can theoretically keep calling this function with values that lead to repeated calls with predictable results, thus exhausting the computational resources. 2.Denial of Service: - In the `initialize_jumpdests` function, improper handling of bytecode lengths could lead to infinite loops or excessive memory allocation for unsuccessful jump verifications. 3.Potential Infinite Loops: - The `try_parse_destination_from_bytes` function incorrectly confirms the bytes format which may lead to unnecessary computations or a potential DoS vector if the provided values are not...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_29_group

# starknet has Under-Constrained Computations in Internal Function _save_valid_jumpdests

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-105
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/105
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-105.md

## Brief Summary

Within the Internals namespace of the Cairo contract, the _save_valid_jumpdests function is designed to iterate through a dictionary of jump destinations (dict_start to dict_end) and store valid jump destinations into the StarkNet account storage. Although this function is internal and not directly exposed to external actors, the current implementation presents potential risks related to under-constrained computations that could affect the contract’s reliability and performance. The loop termination condition relies on the difference between dict_end and the current dict pointer: If the parameters dict_start and dict_end are not strictly controlled or validated before invocation, there’s a...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# account has Incorrect Assignment of code_hash in set_code Function Leading to Under-Constrained Computation

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-106
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/106
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-106.md

## Brief Summary

The set_code function within the Account namespace incorrectly assigns the code_hash of an account. Instead of utilizing the result from the compute_code_hash function, it erroneously derives code_hash from the allocation pointer (ap). This oversight leads to under-constrained computations, allowing potential manipulation of the code_hash, which is critical for account integrity and security. Vulnerable Code The code_hash does not reflect the actual hash of the provided code, undermining the contract’s integrity.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_10_group

# DualVmToken has Unchecked transferFrom Allows Unauthorized Token Transfers

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-66
- **Submitter:** debo
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/66
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-66.md

## Brief Summary

The transferFrom function in the DualVmToken contract lacks proper access control checks. Allowing any caller to transfer tokens from any account to any other account without authorization. This vulnerability can lead to a complete loss of funds for token holders. As attackers can arbitrarily transfer tokens from any address. Vulnerable Code File: DualVmToken.sol

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_69_group

# L2KakarotMessaging has Unrestricted Access to sendMessageToL1 Allows Unauthorized L2 to L1 Messaging

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-67
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/67
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-67.md

## Brief Summary

The sendMessageToL1 function in the L2KakarotMessaging contract lacks proper access control. Allowing any external account to invoke it and send arbitrary messages from L2 to L1. This can lead to unauthorized actions on the L1 contract. Vulnerable Code

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Inconsistent Chain ID Calculation in Kakarot Contracts May Lead to Security and Compatibility Issues

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-129
- **Submitter:** eta
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/129
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-129.md

## Brief Summary

The Kakarot contracts exhibit an inconsistency in how they calculate the `chain_id`. While most of the code uses the `unsigned_div_rem` function to limit the `chain_id` to 53 bits, ensuring compatibility with systems like JavaScript, the `eth_transaction.cairo` contract's decoding functions (`decode_legacy_tx`, `decode_2930`, and `decode_1559`) rely on `bytes_to_felt`, which does not enforce this limit. This discrepancy can lead to several problems, including potential security risks from chain ID mismatches, possible overflow issues, repay attacks, and integration difficulties with external systems expecting a uniform format.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Bypassing `assert_only_self`: Delegate Call Vulnerability and Reentrancy Risks in accounts/library.cairo Contracts

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-130
- **Submitter:** eta
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/130
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-130.md

## Brief Summary

Bypassing `assert_only_self`: Delegate Call Vulnerability and Reentrancy Risks in accounts/library.cairo Contracts

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Pointer Misuse in StarkNet Account Commit Function Can Lead to Memory Errors and Vulnerabilities

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-148
- **Submitter:** eta
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/148
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-148.md

## Brief Summary

The `_commit_account` function in the `starknet.cairo` contract is designed to commit account state to StarkNet’s storage backend. However, while creating the `model.Transfer` structure, the `burn_address` field is passed as a value, whereas the `recipient` field is expected to be a pointer (`Address*`). Passing the `recipient` as a value (`Address`) instead of a pointer can cause type mismatches, resulting in either compilation errors or memory access issues. This can lead to crashes, unpredictable behavior, or even security vulnerabilities like unauthorized memory access.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential Issues and Recommendations for First Element in `get_constructor_calldata` in Cairo Contracts

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-222
- **Submitter:** eta
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/222
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-222.md

## Brief Summary

The `get_constructor_calldata` function in the `account.cairo` contract hardcodes the first element of constructor data to `1`, which could lead to challenges with version control, cross-contract compatibility, and code maintainability. Hardcoding a value that may be used for versioning or validation risks security and adaptability issues. To address this, it is recommended to replace hardcoding with a constant or configurable variable, include clear comments, and consider using enums or a version control mechanism to enhance flexibility and maintainability.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Issues with Address Conversion and Validation in DualVmToken.sol Contract

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-77
- **Submitter:** eta
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/77
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-77.md

## Brief Summary

The `DualVmToken.sol` contract has issues with how Ethereum and StarkNet addresses are handled in functions like `approve` and `transfer`. Specifically, when an Ethereum address is used as `msg.sender`, it may be converted into a StarkNet address that is greater than or equal to `STARKNET_FIELD_PRIME`. Similarly, StarkNet addresses might also exceed this limit. The `_approve` and `_transfer` functions do not currently handle these conversions or validate if the converted or original StarkNet address is within the allowed range. This causes problems with querying `balanceOf` and `allowance`, and prevents authorized spenders from using the `transferFrom` function to transfer tokens on behalf...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_97_group

# Underconstrained `execute_from_outside` Leading to Arbitrary Transaction Manipulation

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-136
- **Submitter:** g_x_w
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/136
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-136.md

## Brief Summary

The `execute_from_outside` function in `account_contract.cairo` processes user transactions within the EVM. However, it lacks validation for `data_offset` in the `call_array` and `tx_data_len` in the `packed_tx_data`, leading to potential out-of-bounds access in calldata. This vulnerability allows a malicious prover to manipulate `packed_tx_data`, which could result in unintended transaction execution. The impact is critical, as it enables arbitrary transaction manipulation in the EVM. Affected Code The issue occurs in the following section of the code: Vulnerability Description The function does not validate whether `[call_array].data_offset` is within the bounds of `calldata`. If `data_of...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Out of Bounds Read in 2024-09-kakarot/kakarot/src/kakarot/errors.cairo

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-180
- **Submitter:** ihuntpackets
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/180
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-180.md

## Brief Summary

Out of Bounds Read in 2024-09-kakarot/kakarot/src/kakarot/errors.cairo

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_51_group

# Missing Return Values in IERC20.transferFrom.

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-146
- **Submitter:** lightoasis
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/146
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-146.md

## Brief Summary

Some tokens do not return a bool (e.g. USDT, BNB, OMG) on ERC20 methods. see [here](https://gist.githubusercontent.com/lukas-berlin/f587086f139df93d22987049f3d8ebd2/raw/1f937dc8eb1d6018da59881cbc633e01c0286fb0/Tokens%20missing%20return%20values%20in%20transfer) for a comprehensive (if somewhat outdated) list. Some tokens (e.g. BNB) may return a bool for some methods, but fail to do so for others. This resulted in stuck BNB tokens in Uniswap v1 ([details](https://mobile.twitter.com/UniswapProtocol/status/1072286773554876416)). Some particularly pathological tokens (e.g. Tether Gold) declare a bool return, but then return false even when the transfer was successful ([code](https://etherscan.i...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Anyone can transfer ownership of the contract to themselves because of missing access control.

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-147
- **Submitter:** lightoasis
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/147
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-147.md

## Brief Summary

Users can transfer ownership of the contract due to missing access control. Vulnerability details If we look through the admin section of the kakarot contract, (see [admin section](https://github.com/kkrt-labs/kakarot/blob/7411a5520e8a00be6f5243a50c160e66ad285563/src/kakarot/kakarot.cairo#L32-#L71)), we see that only the owner should be able to execute the following commands: * Pause / unpause the contract * Upgrade the contract * Transfer ownership of the contract, etc. But because of a missing access control on the `transfer_ownership` function, non-admin users can transfer ownership of the contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_104_group

# Attackers can steal user's tokens because there is no check for token approval in `transferFrom`.

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-63
- **Submitter:** lightoasis
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/63
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-63.md

## Brief Summary

There is no check for token allowance before transferring tokens in the `transferFrom` function. This could result in user's tokens being stolen. Vulnerability details The `transferFrom` function transfers tokens from one user to another without checking to ensure that the caller (msg.sender) has the approval of the user to transfer their tokens. this allows attackers to steal users tokens by transferring tokens from the user without the user's approval.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_37_group

# Event length is not sufficiently validated when committing the message

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-141
- **Submitter:** minglei-wang-3570
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/141
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-141.md

## Brief Summary

Event length is not sufficiently validated when committing the message

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Inconsistent gas refund logic comparing to geth implementation

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-258
- **Submitter:** minglei-wang-3570
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/258
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-258.md

## Brief Summary

In geth implementation, the code handles the [gas refund below](https://github.com/ethereum/go-ethereum/blob/80bdab757dfb0f6d73fb869d834979536fe474e5/core/state_transition.go#L459): and [st.refundGas](https://github.com/ethereum/go-ethereum/blob/80bdab757dfb0f6d73fb869d834979536fe474e5/core/state_transition.go#L496) the code actually add the refunded gas back to the user account. Instead of charge the full gas, then refund at most 4 / 5 gas amount, the code in kakarot charge [at most 4 / 5 directly.](https://github.com/kkrt-labs/kakarot/blob/7411a5520e8a00be6f5243a50c160e66ad285563/src/kakarot/interpreter.cairo#L1032) where [total_gas_used](https://github.com/kkrt-labs/kakarot/blob/7411a552...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_72_group

# Incorrect Handling of Signed Integer Conversion in `pow()` Function Can Lead to Panics

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-181
- **Submitter:** mrjorystewartbaxter
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/181
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-181.md

## Brief Summary

The `pow()` function in the `Exponentiation` from `math.cairo` (https://github.com/kkrt-labs/kakarot-ssj/blob/d4a7873d6f071813165ca7c7adb2f029287d14ca/crates/utils/src/math.cairo) trait does not correctly validate the base value, leading to potential panics in an edge case involving the conversion of negative signed integers to unsigned types. This flaw can trigger unintended behavior, particularly when using type conversions. **Vulnerability Details** The `pow()` function expects an **unsigned integer** as the base, and the current test cases only account for unsigned inputs. Although the function doesn't directly accept signed integers, an edge case arises when a negative signed integer i...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Public Key Validation for secp256r1 Coordinates

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-182
- **Submitter:** mrjorystewartbaxter
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/182
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-182.md

## Brief Summary

The `verify_signature_secp256r1()` function from cairo1_helpers.cairo (https://github.com/kkrt-labs/kakarot-ssj/blob/d4a7873d6f071813165ca7c7adb2f029287d14ca/crates/contracts/src/cairo1_helpers.cairo#L206) does not explicitly verify whether the provided `x` and `y` coordinates represent a valid point on the secp256r1 elliptic curve before the syscall. This omission could lead to potential issues if invalid public key coordinates are used during the verification process. Description In the function `verify_signature_secp256r1()`, the public key is constructed using the `x` and `y` coordinates provided as inputs. However, there is no explicit check to verify that these coordinates form a vali...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_15_group

# Incorrect Function Name

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-58
- **Submitter:** rare_one
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/58
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-58.md

## Brief Summary

The function name _transfer_eth is misleading, suggesting that it transfers Ether (ETH) instead of ERC20 tokens. And If this function is intended to transfer ETH, then using the IERC20.transferFrom method is incorrect. ETH transfers should be done through native Cairo or StarkNet-specific mechanisms for sending ETH (e.g., via system calls or contract logic for ETH). This function suggests confusion between ETH and ERC20 logic. Impact: Confusion: The misleading name could lead to confusion for developers or users, potentially resulting in incorrect usage or unintended consequences. Misuse: If developers mistakenly believe the function transfers ETH, they might use it in contexts where it's n...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_106_group

# Unchecked Return Value from fetch_balance Leading to Uninitialized Pointer Dereference

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-70
- **Submitter:** rare_one
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/70
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-70.md

## Brief Summary

The function allocates memory for balance_ptr but doesn't check if fetch_balance actually returns a value. If fetch_balance fails, the program might attempt to write to an uninitialized pointer balance_ptr Allocation: The line local balance_ptr: Uint256*; declares a local pointer to Uint256, but it does not allocate or initialize memory for this pointer yet. fetch_balance Call: The function then calls fetch_balance(address) to retrieve the balance associated with the address. However, there is no check to ensure that fetch_balance successfully returns a value or that the value it returns is valid. Vulnerability: If fetch_balance fails or returns null/undefined/invalid data (e.g., due to an...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Array Initialization/Out-of-Bounds Memory Access Due to Pointer Reassignment and Memory Initialization

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-78
- **Submitter:** rare_one
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/78
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-78.md

## Brief Summary

The vulnerability identified is related to the initialization and subsequent manipulation of the arr_x array. Here's a breakdown of the issue: Initialization: The code initially initializes the arr_x array with 12 bytes of zeros using the memset function. This effectively fills the first 12 bytes of the array with zeros. Shifting: In the following line, the arr_x array is shifted by 12 bytes using the arr_x: felt* = arr_x + 12 statement. This effectively moves the pointer to arr_x to point to the 13th byte of the original array. Uninitialized Bytes: As a result of this shifting, the first 12 bytes of the original array, which were filled with zeros during initialization, are now effectively...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Incorrect Stack Initialization Due to Duplicate Pointer Assignment

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-80
- **Submitter:** rare_one
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/80
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-80.md

## Brief Summary

In the following line of the init function: **return new model.Stack(dict_ptr_start, dict_ptr_start, 0);** The same dictionary pointer (dict_ptr_start) is passed twice to the model.Stack constructor, which might lead to an incorrect initialization of the stack. The stack structure appears to expect three arguments: two pointers (dict_ptr_start and dict_ptr) and a size. Typically, the first pointer represents the starting location of the stack, while the second pointer is expected to represent the current location within the stack. By passing the same pointer (dict_ptr_start) twice, the stack’s starting point and current point are initialized to the same value, which may not reflect the actu...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Underflow in charge_gas() could lead to a transaction being executed indefinitely

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-278
- **Submitter:** superpozycja
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/278
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-278.md

## Brief Summary

***Submitter note**: This is speculation on future code. This issue is currently not exploitable, and some stars have to align for it to become as such - but if it did happen, the consequences seem severe. This edge case is not really outlined in the [severity categorization doc](https://docs.code4rena.com/awarding/judging-criteria/severity-categorization). Therefore, according to [other risk assessment techniques](https://en.wikipedia.org/wiki/Risk_matrix), I'm defaulting to a Medium risk issue assuming the possibility of downgrading.* The tempvar `a` [might underflow](https://github.com/kkrt-labs/kakarot/blob/038b3a3fa66cd1b8959665fed3e2eb7934e146b1/src/kakarot/evm.cairo#L173) if the `amo...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# The function `ICairo1Helpers::library_call_keccak()` is not implemented.

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-218
- **Submitter:** wasm_it
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/218
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-218.md

## Brief Summary

The function `ICairo1Helpers::library_call_keccak()` is not implemented.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Insufficient validation of items_len

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-262
- **Submitter:** xiao
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/262
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-262.md

## Brief Summary

Insufficient validation of items_len

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Use iteration instead of recursion in the parse_storage_keys function

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-266
- **Submitter:** xiao
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/266
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-266.md

## Brief Summary

Use iteration instead of recursion in the parse_storage_keys function

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# _emit_events does not record the evm contract from which the event was issued

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-116
- **Submitter:** zhaojie
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/116
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-116.md

## Brief Summary

The off-chain node or cross-chain bridge cannot work

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# `handle_l1_message` may fail to execute

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-209
- **Submitter:** zhaojie
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/209
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-209.md

## Brief Summary

`handle_l1_message` fail to execute. However, the ETH of the user has been deducted from l1, resulting in a loss of user funds.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_22_group

# BlockInformation.selfbalance without calculating gas fee

- **Contest:** Kakarot
- **Slug:** 2024-09-kakarot
- **Submission:** V-97
- **Submitter:** zhaojie
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-09-kakarot-validation/issues/97
- **Source snapshot:** competitions/2024-09-kakarot/submissions/raw/V-97.md

## Brief Summary

the gas fee is not calculated, it may cause a Dos attack.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary
