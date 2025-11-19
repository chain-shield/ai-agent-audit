## Verified Patterns Found: 4

## Verified Patterns Found in following Categories:

- StorageCollisionOrSelectorClash
- AccessControlOrAuthByPass
- GriefableCallbacks



## Summary of Patterns

Router-held funds can be drained by anyone via unrestricted execute/multicall

Tstorish storage flag collides under delegatecall and can break sentinel reads

Native sweep/refund can be griefed by recipient contracts reverting or gas-bombing

injectAndCall lets anyone drain TrailsRouter contract balances

## Patterns



 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: TrailsRouter.execute / pullAmountAndExecute / injectAndCall

 ### Title
Router-held funds can be drained by anyone via unrestricted execute/multicall
 ### Description/Code Snippet
TrailsRouter exposes public functions that allow arbitrary calls to be executed from the router’s own address, using the router’s ETH and ERC-20 balances, without any access control. If any tokens or ETH end up held by the TrailsRouter contract (for example, via mis-integration, accidental transfers, or multicall routes that leave residual balances), any external user can steal those funds.

Key points:
- execute(bytes calldata data) is public and payable and only calls _validateRouterCall(data) (which restricts the Multicall3 selector and allowFailure flags) before doing a delegatecall into the canonical Multicall3 contract:
  - (bool success, bytes memory returnData) = MULTICALL3.delegatecall(data);
  - Since this is a delegatecall, Multicall3.aggregate3Value runs in TrailsRouter’s context; its internal target.call{value: call.value}(callData) calls will spend ETH and tokens from address(this) == TrailsRouter.
  - There is no restriction on the Call3Value.target or callData fields, so a caller can build a single-call array where target is an ERC-20 token contract and callData is transfer(attacker, amount), causing the token to transfer from TrailsRouter to the attacker.
- pullAndExecute and pullAmountAndExecute similarly end by delegatecalling Multicall3, and thus also allow arbitrary calls from TrailsRouter’s context once any funds are present on the router.
- injectAndCall(address token, address target, bytes calldata callData, uint256 amountOffset, bytes32 placeholder) is also public and uses _getSelfBalance(token) to determine callerBalance as the full token or ETH balance of address(this) before forwarding it:
  - For ETH: target.call{value: callerBalance}(callData) sends the entire router ETH balance to the chosen target.
  - For ERC-20: SafeERC20.forceApprove(token, target, callerBalance) then target.call(callData) lets the target pull up to callerBalance tokens from the router.
  - None of these paths are restricted by onlyDelegatecall or any ownership/role mechanism.

Because TrailsRouter is intended to be stateless and non-custodial, the authors rely on the assumption that it should not hold persistent user funds. However, that assumption is not enforced on-chain. Any integration mistake or direct user multicall that leaves tokens or ETH at the router address makes those funds globally stealable by anyone who notices the balance and calls execute or injectAndCall with appropriate targets and calldata.

Relevant code locations:
- TrailsRouter.execute: unrestricted delegatecall to MULTICALL3 using router balance.
- TrailsRouter.pullAmountAndExecute: pulls tokens into router then delegatecalls MULTICALL3.
- TrailsRouter.injectAndCall and _injectAndExecuteCall: read full router balance via _getSelfBalance(token) and forward it to arbitrary target without auth.
 ### Static Signals
no onlyOwner/role checks on execute(), delegatecall to external MULTICALL3 with arbitrary Call3Value.target, _getSelfBalance(token) used to forward entire router balance, no sentinel or opHash gating on these paths
 ### Assets at Risk
ETH accidentally left on TrailsRouter, ERC-20 tokens mistakenly or residually held by TrailsRouter, funds from misconfigured or third-party integrations using the router directly
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: StorageCollisionOrSelectorClash

 ### Relevant Function/Location: TrailsRouter.validateOpHashAndSweep

 ### Title
Tstorish storage flag collides under delegatecall and can break sentinel reads
 ### Description/Code Snippet
TrailsRouter is explicitly designed to be used via delegatecall from a Sequence v3 wallet or from TrailsRouterShim:

```solidity
contract TrailsRouter is IDelegatedExtension, ITrailsRouter, DelegatecallGuard, Tstorish {
    ...
    function validateOpHashAndSweep(bytes32 opHash, address _token, address _recipient)
        public
        payable
        onlyDelegatecall
    {
        uint256 slot = TrailsSentinelLib.successSlot(opHash);
        if (_getTstorish(slot) != TrailsSentinelLib.SUCCESS_VALUE) {
            revert SuccessSentinelNotSet();
        }
        sweep(_token, _recipient);
    }
}
```

TrailsRouter inherits `Tstorish`, which internally uses a non‑namespaced storage boolean `_tstoreSupport` to decide whether to read from transient storage (`tload`) or regular storage (`sload`):

```solidity
contract Tstorish {
    bool private _tstoreSupport; // slot 0 in the *executing* contract
    ...
    function _getTstorishWithSloadFallback(uint256 storageSlot)
        private
        view
        returns (uint256 value)
    {
        if (_tstoreSupport) {
            assembly { value := tload(storageSlot) }
        } else {
            assembly { value := sload(storageSlot) }
        }
    }
}
```

On chains where `TSTORE` is not available at deployment, the Tstorish constructor sets the `_getTstorish` function pointer to `_getTstorishWithSloadFallback`. This is correct for **direct calls** to TrailsRouter, where `_tstoreSupport` lives in TrailsRouter’s own storage and remains `false` unless `__activateTstore()` is called.

However, in the Trails design, `validateOpHashAndSweep` is meant to be executed via **delegatecall** in the context of a Sequence v3 intent wallet (through `TrailsRouterShim`). Under delegatecall, *all* storage accesses in TrailsRouter/Tstorish operate on the caller’s storage layout – i.e., the wallet’s storage – not the router’s.

This means:

- The `_tstoreSupport` read inside `_getTstorishWithSloadFallback` actually reads **slot 0 of the intent wallet**, not a dedicated flag owned by Tstorish.
- If the wallet’s slot 0 happens to be non‑zero (very common if slot 0 is used for an address, counter, or other state), `_tstoreSupport` will be interpreted as `true` even though Tstorish never ran `__activateTstore()` in that storage context.
- On chains where `TSTORE` is **not supported** (the reason `_getTstorishWithSloadFallback` was selected in the first place), this will cause `_getTstorish` to execute the `tload` opcode and hit an invalid opcode, reverting the entire `validateOpHashAndSweep` call.

Consequences:

- Sentinel checks in `validateOpHashAndSweep` can become **unusable** when TrailsRouter is used via delegatecall from a wallet whose first storage slot is non‑zero on a chain that does not yet support `TSTORE`.
- As a result, any intent flow that relies on `validateOpHashAndSweep` to gate fee collection or sweeping based on the success sentinel may revert systematically, potentially leaving bridged or routed funds stuck in the intent wallet until an alternate sweep path is constructed.
- Because `_tstoreSupport` is un‑namespaced and interpreted from the caller’s storage, **any** host contract that uses TrailsRouter via delegatecall (not just Sequence wallets) can encounter this behavior depending on its slot‑0 contents.

This is a classic storage‑collision issue specific to delegatecall‑based modules: a library (`Tstorish`) assumes ownership of a plain storage slot (`_tstoreSupport`), but when used as a delegatecall extension, that slot is actually controlled by the host. The decision to use `tload` vs `sload` in `_getTstorish` is therefore made based on unrelated host state, which can break the sentinel invariant that `validateOpHashAndSweep` depends on.

Mitigations could include:
- Namespacing `_tstoreSupport` (e.g., storing it at a keccak‑derived slot) so it does not overlap with host storage, or
- Avoiding the fallback that conditionally uses `tload` based on a mutable boolean when the contract is intended to be used as a delegatecall module.

 ### Static Signals
contract intended for delegatecall use inherits library with regular storage variable, _tstoreSupport controls choice between tload and sload, validateOpHashAndSweep reads sentinel via _getTstorish() in delegatecall context
 ### Assets at Risk
intent wallet token balances, user cross-chain funds that rely on sentinel-gated sweeping
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless




 ### Issue Type: GriefableCallbacks

 ### Relevant Function/Location: TrailsRouter.sweep

 ### Title
Native sweep/refund can be griefed by recipient contracts reverting or gas-bombing
 ### Description/Code Snippet
The TrailsRouter makes low-level native-token transfers to arbitrary, user-specified recipients in `sweep` and `refundAndSweep` via the internal `_transferNative` helper:

```solidity
function _transferNative(address _to, uint256 _amount) internal {
    (bool success,) = payable(_to).call{value: _amount}("");
    if (!success) revert NativeTransferFailed();
}

function sweep(address _token, address _recipient) public payable onlyDelegatecall {
    uint256 amount = _getSelfBalance(_token);
    if (amount > 0) {
        if (_token == address(0)) {
            _transferNative(_recipient, amount);
        } else {
            _transferERC20(_token, _recipient, amount);
        }
        emit Sweep(_token, _recipient, amount);
    }
}

function refundAndSweep(address _token, address _refundRecipient, uint256 _refundAmount, address _sweepRecipient)
    public
    payable
    onlyDelegatecall
{
    uint256 current = _getSelfBalance(_token);
    uint256 actualRefund = _refundAmount > current ? current : _refundAmount;
    ...
    if (actualRefund > 0) {
        if (_token == address(0)) {
            _transferNative(_refundRecipient, actualRefund);
        } else {
            _transferERC20(_token, _refundRecipient, actualRefund);
        }
        emit Refund(_token, _refundRecipient, actualRefund);
    }
    uint256 remaining = _getSelfBalance(_token);
    if (remaining > 0) {
        if (_token == address(0)) {
            _transferNative(_sweepRecipient, remaining);
        } else {
            _transferERC20(_token, _sweepRecipient, remaining);
        }
        emit Sweep(_token, _sweepRecipient, remaining);
    }
    emit RefundAndSweep(...);
}
```

Because `_transferNative` uses a raw `.call{value: _amount}("")` to an arbitrary recipient with no gas limit and no try/catch, **any recipient contract can unilaterally cause `sweep` / `refundAndSweep` to revert** by reverting in its `receive`/`fallback` or by consuming all forwarded gas (gas bomb). These functions are intended to be used from a Sequence wallet via `delegatecall` to move all remaining native balance to a refund or sweep recipient.

If a malicious or badly implemented recipient contract is used (e.g., as the `_refundRecipient` or `_sweepRecipient`), every attempt to execute these flows for that wallet will revert. This can grief Trails’ automated refund/sweep logic and leave funds stranded in the wallet (the wallet still owns the funds, but the Trails automation can be effectively DoS’d for that leg). There is no way in the router to bypass or downgrade the failing callback (no gas cap, no best-effort send, no alternative path), so the success of these core flows is fully dependent on untrusted recipient behavior.
 ### Static Signals
low-level call to arbitrary recipient, no try/catch around external native transfer, callback gas not limited (forwards all gas), success of transfer required for core sweep/refund flow to proceed
 ### Assets at Risk
user wallet native balance on Sequence wallet, cross-chain routed native funds awaiting sweep/refund
 ### Minimum Privilege Required to Exploit Vulnerability: RequiresRole




 ### Issue Type: AccessControlOrAuthByPass

 ### Relevant Function/Location: TrailsRouter.injectAndCall

 ### Title
injectAndCall lets anyone drain TrailsRouter contract balances
 ### Description/Code Snippet
The injectAndCall function is documented as intended for delegatecall use from Sequence wallets, where address(this) refers to the wallet and _getSelfBalance(token) reads the wallet balance. However, it is declared public with no onlyDelegatecall guard, so it can also be called directly on the TrailsRouter implementation contract.

Relevant code:

function injectAndCall(
    address token,
    address target,
    bytes calldata callData,
    uint256 amountOffset,
    bytes32 placeholder
) public payable {
    uint256 callerBalance = _getSelfBalance(token);
    if (callerBalance == 0) {
        if (token == address(0)) {
            revert NoEthAvailable();
        } else {
            revert NoTokensToSweep();
        }
    }

    _injectAndExecuteCall(token, target, callData, amountOffset, placeholder, callerBalance);
}

function _injectAndExecuteCall(
    address token,
    address target,
    bytes memory callData,
    uint256 amountOffset,
    bytes32 placeholder,
    uint256 callerBalance
) internal {
    ...
    if (token == address(0)) {
        (bool success, bytes memory result) = target.call{value: callerBalance}(callData);
        ...
    } else {
        IERC20 erc20 = IERC20(token);
        SafeERC20.forceApprove(erc20, target, callerBalance);
        (bool success, bytes memory result) = target.call(callData);
        ...
    }
}

When called directly (not via delegatecall), _getSelfBalance(token) reads the TrailsRouter contract's own ETH or ERC20 balance. The function then unconditionally forwards the entire balance to an arbitrary user-controlled target: for ETH via target.call{value: callerBalance}(...), and for ERC20 via forceApprove(token, target, callerBalance) followed by target.call(...), allowing the target to transferFrom the router.

This means any ETH or tokens that end up at the TrailsRouter address (for example, mistaken transfers, dust from misconfigured calls with nonzero msg.value, or other integrations that temporarily park funds on the router) can be drained by any arbitrary caller, not by a privileged role or the original sender. The documentation suggests the router is meant to be stateless and used via delegatecall, but the implementation exposes a permissionless drain of the router's own balances.

While the core Trails design tries to avoid holding funds on the router, in practice contracts often do receive accidental transfers or leftover value. The lack of an access-control check or delegatecall guard on injectAndCall turns these balances into publicly stealable funds rather than simply stuck funds.
 ### Static Signals
no onlyDelegatecall on function, uses _getSelfBalance(token) (contract balance) as amount, forwards full balance via target.call{value:callerBalance}, forceApprove(token, target, callerBalance) to user-controlled target, no access control on sweeping router-held funds
 ### Assets at Risk
Router ETH balance, Router ERC20 balances, mis-sent user funds or dust held on TrailsRouter
 ### Minimum Privilege Required to Exploit Vulnerability: Permissionless

