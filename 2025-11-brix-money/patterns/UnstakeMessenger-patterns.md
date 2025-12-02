## Verified Patterns Found: 8

## Verified Patterns Found in following Categories:

- StandardViolation
- UnsafeAssembyTypeCasts
- AccountingInvariantViolation
- SlippageMissingOrInsufficient
- ForcedAssetVsStrictEquality



## Summary of Patterns

Unsafe cast of `returnTripAllocation` to `uint128`

Strict msg.value check in inherited _payNative breaks fee buffer logic

Hardcoded destination gas limit prevents execution if costs increase

Unsafe Downcast of returnTripAllocation

UnstakeMessenger inherits OAppSender strict fee check, breaking buffer feature

Unsafe downcasting of returnTripAllocation enables silent truncation

Buffer Mechanism Revert and Missing ETH Refund

Unsafe Downcasting of returnTripAllocation

## Patterns



 ### Issue Type: UnsafeAssembyTypeCasts

 ### Relevant Function/Location: UnstakeMessenger.unstake

 ### Title
Unsafe cast of `returnTripAllocation` to `uint128`
 ### Description/Code Snippet
The `unstake` function explicitly casts the user-provided `returnTripAllocation` (uint256) to `uint128` without checking for overflow. If a user provides a value larger than `type(uint128).max` (e.g., in a chain with high-supply native tokens), the value will be truncated. This results in the LayerZero executor receiving a much smaller `nativeDrop` amount than intended, likely causing the return message on the hub chain to fail due to insufficient gas, while the user still pays the full outbound fee.
 ### Static Signals
uint128(returnTripAllocation), missing range check before cast
 ### Assets at Risk
user funds (cross-chain message failure)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: ForcedAssetVsStrictEquality

 ### Relevant Function/Location: UnstakeMessenger.unstake

 ### Title
Strict msg.value check in inherited _payNative breaks fee buffer logic
 ### Description/Code Snippet
The `UnstakeMessenger` contract allows users to send `msg.value` greater than the required LayerZero fee to provide a safety buffer against gas price fluctuations (via `quoteUnstakeWithBuffer`). It explicitly checks `msg.value < fee.nativeFee` and intends to refund the excess. However, it inherits `OAppSender._payNative`, which enforces a strict equality check: `if (msg.value != _nativeFee) revert NotEnoughNative(msg.value);`. Since `UnstakeMessenger` does not override this function, any transaction sending more than the exact fee (i.e., using the buffer feature) will revert, causing a Denial of Service for the intended buffering functionality.
 ### Static Signals
if (msg.value != _nativeFee), revert NotEnoughNative
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: UnstakeMessenger.unstake

 ### Title
Hardcoded destination gas limit prevents execution if costs increase
 ### Description/Code Snippet
The `unstake` function constructs LayerZero options using a hardcoded gas limit `LZ_RECEIVE_GAS` (350,000) for the execution of the `lzReceive` handler on the Hub chain. This value is stored in a constant variable and cannot be adjusted by the user or the admin. If the gas requirements for the destination function (`unstakeThroughComposer`) exceed 350,000 due to protocol upgrades, EVM gas schedule changes, or state growth, cross-chain messages will permanently fail on the destination with no mechanism for the user to provide a higher gas limit.
 ### Static Signals
constant LZ_RECEIVE_GAS, addExecutorLzReceiveOption(LZ_RECEIVE_GAS, ...)
 ### Assets at Risk

 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: SlippageMissingOrInsufficient

 ### Relevant Function/Location: UnstakeMessenger.unstake

 ### Title
Unsafe Downcast of returnTripAllocation
 ### Description/Code Snippet
In `UnstakeMessenger.unstake`, the user-supplied `returnTripAllocation` (uint256) is cast to `uint128` without checking for overflow. If a user provides a value exceeding `type(uint128).max` (e.g. due to input error or using a high-decimal token standard elsewhere), the value is silently truncated. This results in the Hub chain receiving a drastically insufficient `msg.value` for the return trip, causing the cross-chain operation to fail while the user pays the initial fee.
 ### Static Signals
uint128(returnTripAllocation), disable-next-line(unsafe-typecast)
 ### Assets at Risk
user funds (fees)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: UnstakeMessenger.unstake

 ### Title
UnstakeMessenger inherits OAppSender strict fee check, breaking buffer feature
 ### Description/Code Snippet
The `UnstakeMessenger` contract enables users to include a fee buffer (`feeBufferBPS`) via `msg.value` to handle gas fluctuations (`quoteUnstakeWithBuffer`). However, it inherits `OAppSender` without overriding `_payNative`. The default `OAppSender._payNative` implementation enforces `if (msg.value != _nativeFee) revert NotEnoughNative(msg.value);`. This causes `unstake()` to revert whenever a buffer is provided (`msg.value > fee.nativeFee`), breaking the core fee-buffering functionality. Additionally, the contract lacks logic to refund the buffer amount locally, as `_lzSend` only forwards the exact fee to the endpoint.
 ### Static Signals
inherits OAppSender, msg.value >= fee check, does not override _payNative
 ### Assets at Risk
user funds (DoS of buffer feature)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StandardViolation

 ### Relevant Function/Location: UnstakeMessenger.unstake

 ### Title
Unsafe downcasting of returnTripAllocation enables silent truncation
 ### Description/Code Snippet
The `unstake` function explicitly downcasts the user-provided `returnTripAllocation` (uint256) to `uint128` when building options, with a comment asserting safety but no code enforcing it. If a user provides a value exceeding `type(uint128).max`, it will silently truncate, leading to incorrect value forwarding to the destination chain and potential failure of the cross-chain logic.
 ### Static Signals
uint128(variable), missing require check before cast, comment claiming safety without validation
 ### Assets at Risk
Cross-chain execution integrity
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: UnstakeMessenger.unstake

 ### Title
Buffer Mechanism Revert and Missing ETH Refund
 ### Description/Code Snippet
The `unstake` function is designed to accept a `feeBuffer` (excess `msg.value` > `fee.nativeFee`) to handle gas fluctuations. However, the contract inherits `OAppSender` which implements `_payNative` with a strict equality check (`msg.value == _nativeFee`), causing any transaction with a buffer to revert. Furthermore, the `unstake` function lacks explicit logic to refund `msg.value - fee.nativeFee` to the user; `_lzSend` only consumes the exact fee, so even if the strict check were bypassed, the excess ETH would remain stuck in the contract.
 ### Static Signals
if (msg.value < fee.nativeFee), no override of _payNative, no msg.sender.call{value: ...} for refund
 ### Assets at Risk
user ETH
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: AccountingInvariantViolation

 ### Relevant Function/Location: UnstakeMessenger.unstake

 ### Title
Unsafe Downcasting of returnTripAllocation
 ### Description/Code Snippet
The `unstake` function accepts a `uint256 returnTripAllocation` but casts it to `uint128` when building the LayerZero options via `OptionsBuilder.newOptions().addExecutorLzReceiveOption(..., uint128(returnTripAllocation))`. While comments claim safety, there is no check ensuring `returnTripAllocation <= type(uint128).max`. A user supplying a larger value (intentionally or by error) will suffer silent truncation, likely causing the cross-chain execution to fail on the destination due to insufficient native value, wasting the outbound fees.
 ### Static Signals
uint128(returnTripAllocation), disable-next-line(unsafe-typecast)
 ### Assets at Risk
user ETH (fees)
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

