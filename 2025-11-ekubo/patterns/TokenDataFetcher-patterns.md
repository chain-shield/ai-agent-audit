## Verified Patterns Found: 5

## Verified Patterns Found in following Categories:

- UnboundedLoops
- StandardViolation
- GriefableCallbacks



## Summary of Patterns

Unbounded Nested Loop Vulnerability

Strict return data length check in `allowance` ignores non-standard tokens

DoS in Lens Contract via Unhandled Revert in `balanceOf`

Batch query DoS via unhandled token reverts in `balanceOf` calls

Return Bomb Denial of Service in Batch Allowance Query

## Patterns



 ### Issue Type: UnboundedLoops

 ### Relevant Function/Location: TokenDataFetcher.getNonzeroBalancesAndAllowances

 ### Title
Unbounded Nested Loop Vulnerability
 ### Description/Code Snippet
The function `getNonzeroBalancesAndAllowances` contains a nested loop structure: it iterates over `tokens` (outer loop) and `spenders` (inner loop). This O(N*M) complexity means that relatively small input arrays (e.g., 50 tokens and 50 spenders) result in 2,500 external calls and state reads. Since each iteration involves an external `staticcall` and multiple memory operations, the gas cost scales rapidly, making it trivial to hit the block gas limit. This renders the function unusable for fetching data on larger sets of tokens/spenders in a single call.
 ### Static Signals
nested loops in external functions, loops over user-controlled arrays
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: TokenDataFetcher.getNonzeroBalancesAndAllowances

 ### Title
Strict return data length check in `allowance` ignores non-standard tokens
 ### Description/Code Snippet
The `allowance` fetching logic explicitly requires `result.length == 32`. While standard ERC20s return exactly 32 bytes, some valid tokens (e.g., proxies returning extra data or non-standard implementations) may return more than 32 bytes. `abi.decode` can normally handle this by reading the first 32 bytes, but the strict length check forces the contract to discard valid allowance data from such tokens, returning incorrect zero values to the caller.
 ### Static Signals
result.length == 32, incorrect return values/events per standard spec
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: TokenDataFetcher.getNonzeroBalancesAndAllowances

 ### Title
DoS in Lens Contract via Unhandled Revert in `balanceOf`
 ### Description/Code Snippet
The `getNonzeroBalancesAndAllowances` function is designed to batch-fetch data for a list of tokens. While it robustly handles failures in `allowance` calls using `staticcall` and a success check, it relies on `SafeTransferLib.balanceOf` for fetching balances. Solady's `SafeTransferLib.balanceOf` typically bubbles up reverts from the underlying token contract. If a single token in the provided `tokens` list reverts (e.g., due to a paused state, proxy misconfiguration, or malicious logic), the entire batch call will revert. This creates a denial of service vector where one 'poisoned' token prevents the retrieval of data for all other valid tokens.
 ### Static Signals
SafeTransferLib.balanceOf used without try/catch, Manual staticcall used for allowance but not balance, Iterates over user-supplied token list
 ### Assets at Risk
Integration/UI Availability
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: TokenDataFetcher.getNonzeroBalancesAndAllowances

 ### Title
Batch query DoS via unhandled token reverts in `balanceOf` calls
 ### Description/Code Snippet
The function `getNonzeroBalancesAndAllowances` iterates through a user-supplied list of tokens and calls `SafeTransferLib.balanceOf(token, owner)`. Solady's `SafeTransferLib.balanceOf` propagates reverts from the underlying token contract. If a single token in the list reverts (e.g. due to pausing, malicious logic, or unhandled errors), the entire batch query reverts. This creates a Denial of Service (DoS) vulnerability for off-chain components fetching portfolios, where one 'poisoned' token prevents viewing data for all other tokens.
 ### Static Signals
external call in loop without failure isolation, calls SafeTransferLib.balanceOf inside loop
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: TokenDataFetcher.getNonzeroBalancesAndAllowances

 ### Title
Return Bomb Denial of Service in Batch Allowance Query
 ### Description/Code Snippet
The `getNonzeroBalancesAndAllowances` function performs a raw `token.staticcall` to fetch allowances inside a nested loop. Unlike the `SafeTransferLib.balanceOf` call used for balances (which uses assembly to limit return data reads), this `staticcall` allows Solidity to copy the entire return payload into memory. A malicious token can utilize a 'Return Bomb' (returning a massive data payload) to trigger excessive memory expansion and gas consumption, causing the entire batch read operation to revert due to Out-Of-Gas, effectively preventing the user from retrieving data for any legitimate tokens in the same batch.
 ### Static Signals
external call in loop, staticcall returning bytes memory, no return data size limit
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

