# Accepted H/M Findings: Phat Contract Runtime

# [M-01] Limited availability of balance_of(...) method

- **Contest:** Phat Contract Runtime
- **Slug:** 2024-03-phat-contract-runtime
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-phat-contract-runtime
- **Source snapshot:** competitions/2024-03-phat-contract-runtime/final_report.html

balance_of(...) method Submitted by 0xTheC0der According to the documentation ( online and in-line ), the availability of the balance_of(…) method (see code below) should be any contract instead of system only which is caused by the present ensure_system check.

fn balance_of ( & self, account: ext::AccountId, ) -> Result <(pink::Balance, pink::Balance), Self::Error> { self.

ensure_system ()?; // @audit Availability should be 'any contract' instead of 'system only' let account: AccountId32 = account.

convert_to (); let total = crate::runtime::Balances::

total_balance (&account); let free = crate::runtime::Balances::

free_balance (&account); Ok((total, free)) } The ensure_system(…) method returns a BadOrigin error in case the caller/origin is not the system contract.

fn ensure_system (& self ) -> Result <(), DispatchError> { let contract: AccountId32 = self.address.

convert_to (); if Some(contract) != PalletPink::

system_contract () { return Err(DispatchError::BadOrigin); } Ok(()) } Consequence The availability of the balance_of(…) method is limited to the system contract instead of being accessible to anyone. Therefore, user contracts relying on this method will inevitably fail.

For comparison:

The import_latest_system_code(...) method has consistent system only availability according to the implementation and documentation.

## Recommended Mitigation Steps

Remove the ensure_system check from the balance_of(…) method to ensure availability for any contract.

## Assessed type

Invalid Validation kvinwang (Phala) confirmed

# [M-02] An attacker can bloat the Pink runtime storage with zero costs

- **Contest:** Phat Contract Runtime
- **Slug:** 2024-03-phat-contract-runtime
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-phat-contract-runtime
- **Source snapshot:** competitions/2024-03-phat-contract-runtime/final_report.html

Submitted by DadeKuma An attacker can perform a bloat attack by creating a very high amount of dust accounts. This can occur with a minimal cost for the attacker, and results in a very high and increased cost in terms of storage and fees.

## Recommended Mitigation Steps

Consider using a reasonable Existential Deposit. I recommend at least one CENTS (i.e.

1_000_000_000 ):

- pub const ExistentialDeposit: Balance = 1; + pub const ExistentialDeposit: Balance = 1 * CENTS;

## Assessed type

Decimal kvinwang (Phala) confirmed

# [M-03] A cache that times out can be recovered

- **Contest:** Phat Contract Runtime
- **Slug:** 2024-03-phat-contract-runtime
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-phat-contract-runtime
- **Source snapshot:** competitions/2024-03-phat-contract-runtime/final_report.html

Submitted by zhaojie Recover a cached value that has timed out, and a malicious user or contract can exploit this bug to fool other users or cause other unknown problems.

## Recommended Mitigation Steps

pub fn set_expire(&mut self, id: Cow<[u8]>, key: Cow<[u8]>, expire: u64) { - self.maybe_clear_expired(); + self.clear_expired(); if expire == 0 { let _ = self.remove(id.as_ref(), key.as_ref()); } else if let Some(v) = self.storages.get_mut(id.as_ref()).and_then(|storage| storage.kvs.get_mut(key.as_ref())) { v.expire_at = now().saturating_add(expire) } kvinwang (Phala) confirmed, but disagreed with severity and commented:

This is a good catch.

I’m not sure if this should be classified as High Risk level. The purpose of the local cache is to store volatile data, such as information fetched from an HTTP server. This data is expected to be lost at any time and may vary between different workers for the same contract. If the data is lost, the contract will re-fetch it from the external system.

Therefore, I don’t think it is suitable for storing on-chain assets that users can query.

Lambda (judge) decreased severity to Medium and commented:

Good finding, but agree that Medium is more appropriate, as the finding does not show any direct way how this can be used to steal funds, but only speculates about potential methods (which I am not sure if they can happen in practice and even if, they would have many assumptions).

DadeKuma (warden) commented:

I’m a bit skeptical about this finding. First of all, the docs state that the cache will store off-chain computations and not on-chain data that users can query/call:

//! The LocalCache provides a local KV cache for contracts to do some offchain computation. //! When we say local, it means that the data stored in the cache is different in different //! machines of the same contract. And the data might be lost when the runtime restart or caused //! by some kind of cache expiring mechanism.

- https://github.com/code-423n4/2024-03-phala-network/blob/a01ffbe992560d8d0f17deadfb9b9a2bed38377e/phala-blockchain/crates/pink/chain-extension/src/local_cache.rs#L1-L4
There is no proof that these off-chain computations can be leveraged to impact the protocol or leak value in any way, especially because:

This data is expected to be lost at any time and can vary between different workers.

These are off-chain computations which are not queriable by users.

For these reasons, I believe this finding should be capped to QA/Low, not Medium risk.

zhaojie (warden) commented:

Although the user cannot query the cached data directly, the user can query it indirectly through the contract. The cache stores off-chain data, which can also cause problems if it is recovered.

For example, price data - the attacker lets the expired cache recover, there may be 2 different prices, which will lead to price manipulation. The report says that storing xxxx in the cache is just an assumption.

Severity is determined by the judge, and I think it should at least remain Medium.

DadeKuma (warden) commented:

The set_expire function is never called inside the audit repository, nor in the main Phala repository, and the only proof of how it will be used is inside the docs in the file itself, which points to off-chain computations.

Using this cache to store on-chain data would be a misuse and a user error (and I don’t think it would even make sense as the data is volatile/incongruent between the workers). This is also confirmed by the Sponsor in the comment above.

Of course, the Judge will decide the final severity. I was just adding some details that might have been missed in the initial submission.

EDIT:

set_expire is actually called, GitHub search is broken; see comment below. My point on docs/normal usage stands.

zhaojie (warden) commented:

@DadeKuma - You should search for set_expiration.

pub fn set_expiration (contract: &[ u8 ], key: &[ u8 ], expiration:

u64 ) { with_global_cache (|cache| cache.

set_expire (contract.

into (), key.

into (), expiration)) }

- https://github.com/code-423n4/2024-03-phala-network/blob/a01ffbe992560d8d0f17deadfb9b9a2bed38377e/phala-blockchain/crates/pink/chain-extension/src/local_cache.rs#L273
Lambda (judge) commented:

First of all, the docs state that the cache will store off-chain computations and not on-chain data that users can query/call:

I agree with that. If it were used for on-chain data such as balances, High would be more appropriate. However, even for off-chain data, this behaviour could lead to problems. For instance, if the cache is used for caching some API/web responses (which seems to be one of the most common use cases for the cache), it is not unreasonable that a developer wants to have a maximum age of the response (and with cache_set_expire, there is an exposed function for exactly doing that, which does not always correctly as the warden has shown).

In these cases, it also does not matter that the data is volatile/different between workers, because you care about the maximum age, but fetching newer data is fine. One example I can think of is betting on some result of an API service that refreshes daily where you would set the expiration such that no new requests are made until the next refresh. Of course, this has some assumptions, but I think they are pretty reasonable for such a runtime and it is well possible that there could be contracts that trigger this issue.

# [M-04] An attacker can crash the cluster system by sending an HTTP request with a huge timeout

- **Contest:** Phat Contract Runtime
- **Slug:** 2024-03-phat-contract-runtime
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-phat-contract-runtime
- **Source snapshot:** competitions/2024-03-phat-contract-runtime/final_report.html

Submitted by DadeKuma, also found by Koolex and zhaojie Any user can intentionally crash a worker by sending a maliciously crafted request with a huge timeout. This attack has no costs for the attacker, and it can result in a DoS of the worker/cluster system.

## Recommended Mitigation Steps

Consider using saturating_add instead:

tokio::time::timeout( - Duration::from_millis(timeout_ms + 200), + Duration::from_millis(timeout_ms.saturating_add(200)), futures::future::join_all(futs), )

## Assessed type

Invalid Validation kvinwang (Phala) confirmed, but disagreed with severity and commented:

The runtime doesn’t call the implementation directly. Instead, it calls into the worker, via ocalls here, and the timeout is actually clamped in the worker side. However, the suggested change is good to have. This might be a QA or Mid Risk level report.

Lambda (judge) commented:

Not sure about the severity here. @kvinwang - Could you point out where the clamping happens? Because in the linked code it is a normal u64 that could potentially be set to e.g.

u64::MAX - 1 to trigger the issue.

kvinwang (Phala) commented:

This is the OCalls implementation in the worker, where the time remaining is less than the MAX QUERY TIME.

Lambda (judge) decreased severity to Medium and commented:

Great, thanks for the link. In that case, I am downgrading this to a medium. It is not directly exploitable as an attacker, but the issue itself still exists within the codebase and if a future worker would integrate it differently/without limit, it could become exploitable.
