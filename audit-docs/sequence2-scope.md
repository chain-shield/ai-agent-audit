## V12 findings

[V12](https://v12.zellic.io/) is [Zellic](https://zellic.io)'s in-house AI auditing tool. It is the only autonomous Solidity auditor that [reliably finds Highs and Criticals](https://www.zellic.io/blog/introducing-v12/). All issues found by V12 will be judged as out of scope and ineligible for awards.

## ---------------------------V12 findings listed below --> all These finding are public and OUT OF SCOPE ----------------------------------------

# Missing Zero-Address Validation Leading to Irreversible Token/ETH Burns
- Severity: Low

## Targets
- _transferERC20 (TrailsRouter)
- sweep and refundAndSweep (TrailsRouter)
- refundAndSweep (TrailsRouter)
- sweep (TrailsRouter)
- injectSweepAndCall / _injectAndExecuteCall (TrailsRouter)

## Description

Multiple functions in the TrailsRouter contract allow asset transfers to be directed to the zero address without any validation. Both ERC-20 transfers (via `_transferERC20`) and native ETH transfers (via `_transferNative` and low-level calls) in functions such as `sweep`, `refundAndSweep`, and `injectSweepAndCall` accept user-provided recipient or target addresses without checking that the address is non-zero. Sending to `address(0)` either burns tokens/ETH irreversibly or may revert unexpectedly, resulting in permanent loss of funds or disrupted workflows.

## Root cause

The contract omits explicit checks (e.g., `require(recipient != address(0))`) on user-supplied recipient or target addresses before executing ERC-20 or native asset transfers, enabling zero-address transfers.

# Blind Reliance on safeTransfer without Post-Transfer Balance Verification Leading to Fee-on-Transfer Token Misaccounting
- Severity: Low


## Targets
- _safeTransferFrom (used by pullAmountAndExecute and injectSweepAndCall) (TrailsRouter)
- refundAndSweep (TrailsRouter)
- sweep (TrailsRouter)

## Description

Multiple functions in the TrailsRouter contract (`_safeTransferFrom` used by pullAmountAndExecute and injectSweepAndCall, refundAndSweep, and sweep) rely solely on OpenZeppelin’s SafeERC20.safeTransfer or safeTransferFrom returning true, without verifying the actual change in token balances. Fee-on-transfer (deflationary) tokens deduct fees during transfers—and malicious tokens can falsely report success—so the router and its users may receive or forward fewer tokens than assumed. This leads to incorrect downstream logic, misleading events, locked residual balances (“dust”), and unaccounted-for fund losses.

## Root cause

The router’s transfer wrappers trust only the boolean return value of ERC-20 transfer calls and never reconcile balances before and after the transfer. They assume that a non-reverting, true-returning call always moves the full requested amount, ignoring fee-on-transfer token mechanics or malicious override of transfer semantics.

# Inaccurate Sweep Event Logging for Fee-On-Transfer Tokens
- Severity: Low


## Targets
- sweep (TrailsRouter)

## Description

The Sweep event emits the router’s pre-transfer token balance as the `amount`, but fee-on-transfer or burnable tokens deduct fees during transfer, so the recipient’s actual received amount can be lower than the logged value. This leads to misleading on-chain event data.

## Root cause

Sweep calculates `amount` via `_getSelfBalance` before calling `_transferERC20`, then emits that original balance without verifying the net tokens received after fees. `_transferERC20` simply invokes `SafeERC20.safeTransfer` and does not capture post-transfer balances.

# Locked Ether in ERC-20 Branch of pullAndExecute
- Severity: Low


## Targets
- pullAndExecute (TrailsRouter)

## Description

When a user calls pullAndExecute with a non-zero msg.value and a non-zero token address (an ERC-20 token), the function ignores the ETH sent. The ERC-20 branch never validates or refunds msg.value, so any ETH included is retained by the contract permanently unless a privileged sweep or refund function is invoked.

## Root cause

The code only checks and uses msg.value when token == address(0); in the ERC-20 branch, msg.value is neither validated nor refunded, and downstream logic (_safeTransferFrom and delegatecall) makes no use of it.

# Unhandled Transfer Failures Enable Denial-of-Service in Sweep and Refund Operations
- Severity: Low


## Targets
- _transferERC20 (TrailsRouter)
- sweep (TrailsRouter)

## Description

The TrailsRouter contract’s sweep and refundAndSweep flows rely on internal transfer functions that immediately revert on any transfer failure—whether from native ETH transfers to recipient contracts or ERC-20 transfers via SafeERC20.safeTransfer—without any error handling or fallback logic. This design allows a malicious or misbehaving token or recipient contract to force a revert and block the entire operation.

## Root cause

Both `_transferNative` and `_transferERC20` in TrailsRouter unconditionally bubble up transfer reverts or failures, lacking try/catch, return-value checks, or alternative recovery paths.


## ---------------------------END OF V12 findings ----------------------------------------

## Publicly known issues

_Anything included in this section is considered a publicly known issue and is therefore ineligible for awards._

- **In‑scope contracts:** `TrailsRouter`, `TrailsRouterShim`, `TrailsIntentEntrypoint`, and their libraries. Auditors can interact via the same public API patterns documented (multicall, sweep, injection, EIP‑712 deposit/permit).  
- **Out of scope:** The closed‑source **Intent Machine** (backend), while auditors can still hit the **public API interfaces** and simulate the flows described in the flow docs.  
- **Context coupling:** These contracts are meant to operate with **Sequence v3 wallets** (delegatecall extensions); reviewers should model threats with that in mind.

# Overview

Multichain transaction rails to pay, swap, fund, or earn in 1-click with any wallet, token or chain - powered by intents.

## Summary

This transaction rails module is a chain abstraction orchestration protocol that enables 1-click transactions from any wallet seamlessly with unified user liquidity across all chains. In contrast to typical cross-chain solutions, it sources liquidity and aggregates all user balances for every token across every chain in a user’s wallet as options for any transaction on a destination chain. The module is architected as a trustless system that works on top of existing bridging, filler, or solver infrastructure. It is free to integrate and is optimized for a variety of use cases, for example:

- **Pay**: Enable cross-chain, 1-click payments with any token for ecommerce platforms, NFT marketplaces, real-world asset purchases, and minimal-slippage stablecoin transactions.
- **Swap**: Embed low-latency, highly liquid cross-chain token swaps for your applications.
- **Fund**: Maximize TVL and transaction velocity through fully brandable funding widgets for protocol deposits such as perpetual exchanges, chain deposits, and liquidity provisioning.
- **Earn**: Streamline DeFi yield opportunities by enabling deposits into tokenized vaults, lending pools, and yield strategies from any token on any chain

## Links

- **Previous audits:** [Quantstamp, October 2025](https://github.com/0xsequence/trails-contracts/blob/master/audits/quanstamp-audit-2025-10-23.pdf)
- **Documentation:**
    - https://docs.trails.build/
    - [SDK testing guide](https://github.com/code-423n4/2025-11-sequence/blob/main/SDK_TESTING_GUIDE.md): step-by-step instructions for testing the Trails contracts using the SDK
- **Website:** https://trails.build/
- **X/Twitter:** https://x.com/0xsequence

---

# Scope

### Files in scope

| File                                        | nSLOC | 
|---------------------------------------------|-------|
| [src/TrailsIntentEntrypoint.sol](https://github.com/code-423n4/2025-11-sequence/blob/main/src/TrailsIntentEntrypoint.sol) | 101   | 
| [src/TrailsRouter.sol](https://github.com/code-423n4/2025-11-sequence/blob/main/src/TrailsRouter.sol) | 236   | 
| [src/TrailsRouterShim.sol](https://github.com/code-423n4/2025-11-sequence/blob/main/src/TrailsRouterShim.sol) | 30    |
| [src/guards/DelegatecallGuard.sol](https://github.com/code-423n4/2025-11-sequence/blob/main/src/guards/DelegatecallGuard.sol) | 12    |
| [src/interfaces/IMulticall3.sol](https://github.com/code-423n4/2025-11-sequence/blob/main/src/interfaces/IMulticall3.sol) | 18    |
| [src/interfaces/ITrailsIntentEntrypoint.sol](https://github.com/code-423n4/2025-11-sequence/blob/main/src/interfaces/ITrailsIntentEntrypoint.sol) | 5     |
| [src/interfaces/ITrailsRouter.sol](https://github.com/code-423n4/2025-11-sequence/blob/main/src/interfaces/ITrailsRouter.sol) | 25    | 
| [src/interfaces/ITrailsRouterShim.sol](https://github.com/code-423n4/2025-11-sequence/blob/main/src/interfaces/ITrailsRouterShim.sol) | 4     |
| [src/libraries/TrailsSentinelLib.sol](https://github.com/code-423n4/2025-11-sequence/blob/main/src/libraries/TrailsSentinelLib.sol) | 13    |

*For a machine-readable version, see [scope.txt](https://github.com/code-423n4/2025-11-sequence/blob/main/scope.txt)*

### Files out of scope

| File         |
| ------------ |
| [script/\*\*.\*\*](https://github.com/code-423n4/2025-11-sequence/tree/main/script) |
| [test/\*\*.\*\*](https://github.com/code-423n4/2025-11-sequence/tree/main/test) |
| Total Files: 17 |


# Additional context

## Areas of concern (where to focus for bugs)
### A. Delegatecall‑only router pattern
- **Delegatecall enforcement & assumptions.** `TrailsRouter` / `TrailsRouterShim` are designed to be invoked **only via `delegatecall`** from Sequence v3 wallets; direct calls are blocked (e.g., `onlyDelegatecall`). Probe for any call paths that bypass this constraint, or any place wallet‑context assumptions (storage layout, `msg.sender`) can be violated by unintended delegatecalls.  
- **Storage sentinels & `opHash` gating.** Success/failure is tracked via a per‑op storage sentinel keyed by `opHash`; mistakes in setting/clearing, hash collisions, or re‑use could gate fee sweeps incorrectly. Validate namespacing and slot computation (e.g., `TrailsSentinelLib.successSlot(opHash)`), including Cancun tstore vs. sstore fallbacks.  
- **Multicall3 behavior.** The router composes approvals, swaps, bridges via `IMulticall3.aggregate3Value`. Stress revert bubbling, partial‑success semantics (when upstream sets `behaviorOnError = IGNORE`), and ensure approvals can’t be stranded in a half‑updated state.

### B. Balance injection & calldata surgery
- **`injectAndCall` / `injectSweepAndCall`.** Calldata manipulation uses a fixed 32‑byte placeholder and a provided `amountOffset`. Focus on: offset correctness, alignment, endianness, fee‑on‑transfer tokens, and ETH vs ERC‑20 branches (value forwarding vs approval path). Look for out‑of‑bounds writes and incorrect placeholder detection.  
- **Approval handling quirks.** Uses `SafeERC20.forceApprove` (for USDT‑like tokens). Validate no approval race or leftover unlimited approvals after failure paths.

### C. Fee collection & refund semantics
- **Conditional fee sweeps.** `validateOpHashAndSweep(opHash, token, feeCollector)` should only fire when the success sentinel was set by the shim; verify there’s no path to set the sentinel on partial/incorrect success. Ensure `refundAndSweep` cannot under‑refund the user or over‑sweep to fees when origin calls fail.  
- **Destination failures.** When destination protocol calls fail, the intended behavior is to sweep funds to the user *on the destination chain* (no “undo bridge”). Validate this always occurs and can’t be front‑run/griefed into a stuck state.

### D. Entrypoint contracts
- **`TrailsIntentEntrypoint` (EIP‑712 deposits + optional permits).** Review replay protection, deadline checks, nonces, and the “leftover allowance → `payFee` / `payFeeWithPermit`” pattern so fee collection can’t exceed expectations or happen without user intent. Check reentrancy guard coverage.  

### E. Cross‑chain assumptions
- **Non‑atomicity & monitoring.** Origin/destination legs are decoupled by bridges/relayers. Stress timing windows, reorgs around proofing, dust handling, token decimal mismatches, and MEV on destination protocol interactions (especially with balance injection).

## Main invariants

**Router/Shim invariants**
- Router/RouterShim functions **execute only via `delegatecall`** from a Sequence v3 wallet context. Any direct call must revert via `onlyDelegatecall`.  
- A fee sweep using `validateOpHashAndSweep(opHash, …)` **must** observe `SUCCESS_VALUE` at the sentinel slot computed for that `opHash`; otherwise it reverts and **no fees are taken**.  
- Fallback refund path `refundAndSweep` **only** runs when the immediately previous step reverted under `behaviorOnError = IGNORE` (“onlyFallback” semantics). On success paths, fallback calls are skipped.  
- Balance injection (`injectAndCall`) **must** replace exactly the placeholder bytes at `amountOffset` and use the *current* wallet balance/allowance at call time (ETH via `value`, ERC‑20 via `forceApprove`)—never a guessed amount.

**State/sentinels invariants**
- The success sentinel slot is **namespaced** (no collisions with wallet storage) and keyed by `opHash`; it is set **only** after `RouterShim`’s wrapped call completes successfully.

**Economic invariants**
- On **origin failure**, the user is refunded on origin (funds never bridged), and fees—if collected—come only from remaining balances after refund logic (no user loss beyond quoted fees).  
- On **destination failure**, the user receives tokens on the destination chain via a sweep; no hidden fee collection occurs there beyond the defined sweep step.

**`TrailsIntentEntrypoint` invariants**
- Deposits (`depositToIntent` / `…WithPermit`) **must** match signed EIP‑712 intent (user, token, amount, intentAddress, deadline), with replay blocked by tracked intent hashes and deadline enforced. Reentrancy is guarded.  
- Fee payments (`payFee`, `payFeeWithPermit`) can only move `feeAmount` from the user to `feeCollector` when there is sufficient allowance **or** a valid ERC‑2612 permit for that exact amount by the deadline.

## All trusted roles in the protocol

Protocol is **fully permissionless** for the in-scope contracts, with all flows gated by cryptographic validations (EIP-712 signatures, nonces, deadlines). No explicit admin or owner roles are present.

*(`TrailsRouter` / `TrailsRouterShim` execute under Sequence v3 wallet authority via `delegatecall`; there’s no standalone admin role on these stateless extensions.)*
