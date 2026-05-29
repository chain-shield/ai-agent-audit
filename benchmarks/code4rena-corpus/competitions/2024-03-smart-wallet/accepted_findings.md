# Accepted H/M Findings: Smart Wallet

# [H-01] Remove owner calls can be replayed to remove a different owner at the same index, leading to severe issues when combined with lack of last owner guard

- **Contest:** Smart Wallet
- **Slug:** 2024-03-smart-wallet
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-03-smart-wallet
- **Source snapshot:** competitions/2024-03-smart-wallet/final_report.html

Submitted by Circolors, also found by lsaudit Users are able to upgrade their account’s owners via either directly onto the contract with a regular transaction or via an ERC-4337 EntryPoint transaction calling executeWithoutChainIdValidation. If a user chooses to use a combination of these methods it’s very likely that the addresses at a particular ownership index differ across chain. Therefore if a user later calls removeOwnerAtIndex on another chain will end up removing different addresses on different chains. It is unlikely this would be the user’s intention. The severity of this ranges from minimal (the user can just add the mistakenly removed owner back) or critical (the user mistakenly removes their only accessible owner on a specific chain, permanently locking the account).

## Recommended Mitigation Steps

As MultiOwnableStorage uses a mapping to track owner information rather than a conventional array, it might be simpler to do away with the indexes entirely and have a removeOwner(bytes calldata _ownerToRemove) function. This would avoid the situations outlined above where when calling removeOwnerAtIndex removes different owners on different chains. To ensure replayability and avoid having a stuck nonce on chains where _ownerToRemove is not an owner the function should not revert in the case the owner is not there, but instead return a bool removed to indicate whether an owner was removed or not.

This would make it significantly less likely that users run into the issues stated above, without having to limit their freedom to make ownership changes manually or via ERC-4337 EntryPoint transactions.

3docSec (judge) increased severity to High and commented:

Root cause:

removeOwnerAtIndex can be replayed cross-chain despite the same index may point to a different owner. The issue was confirmed by the sponsor in the issue #57 thread and addressed in this PR.

After consulting with a fellow judge, I’m upgrading this one as high, as there is a well-defined attack path that causes the user to lose ownership of their wallet.

Coinbase mitigated:

The issue is remediated by updating the parameterization of removeOwnerAtIndex to also take an owner argument. We then check that the owner passed matches the owner found at the index. In this way, we prevent a replayable transaction removing a different owner at the same index. (PR here ) Status:

Mitigation confirmed. Full details in reports from McToady, cheatc0d3, and imare.

Medium Risk Findings (2)

# [M-01] Balance check during MagicSpend validation cannot ensure that MagicSpend has enough balance to cover the requested fund

- **Contest:** Smart Wallet
- **Slug:** 2024-03-smart-wallet
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-smart-wallet
- **Source snapshot:** competitions/2024-03-smart-wallet/final_report.html

MagicSpend validation cannot ensure that MagicSpend has enough balance to cover the requested fund Submitted by doublespending, also found by imare Balance check during MagicSpend validation cannot ensure that MagicSpend has enough balance to cover the requested fund.

// Ensure at validation that the contract has enough balance to cover the requested funds.

// NOTE: This check is necessary to enforce that the contract will be able to transfer the remaining funds // when `postOp()` is called back after the `UserOperation` has been executed.

if ( address ( this ).

balance < withdrawAmount ) { revert InsufficientBalance ( withdrawAmount, address ( this ).

balance ); }

## Recommended Mitigation Steps

Use a state instead of address(this).balance to record the remaining balance.

if ( remain < withdrawAmount ) { revert InsufficientBalance ( withdrawAmount, address ( this ).

balance ); } remain -= withdrawAmount; raymondfam (lookout) commented:

This would happen only when the entrypoint batches the transactions. Additionally, this is kind of related to the known issue from the readme: When acting as a paymaster, EntryPoint will debit MagicSpend slightly more than actualGasCost, meaning what is withheld on a gas-paying withdrawal will not cover 100% of MagicSpend’s balance decrease in the EntryPoint.

wilsoncusack (Coinbase) confirmed and commented:

Ah right I feel like we discussed this @xenoliss and then perhaps forgot in a later conversation. The only way to avoid this would be to have validation keep some tally of all the expected coming withdraws.

3docSec (judge) commented:

To be fair, one could argue that the bundlers should simulate the execution of their bundles before submission to avoid reverts; however, this is a valid way of grieving the reputation of the MagicSpend contract.

wilsoncusack (Coinbase) commented:

@3docSec - it is correct that the bundler would be expected to see this and revert the whole bundle. However, the point is valid that the guard is not fully satisfactory as it is written. It should probably be removed or fixed.

Coinbase mitigated:

This issue is complex to address. The warden suggested adding a variable to track in flight withdraws, and we pursued this. However, we realized that bundlers penalize paymasters when the UserOp behaves differently when simulated in isolation vs. in the bundle, and this would not fix this. Instead, we give the owner a tool to address this probabilistically: the owner can set a maxWithdrawDenominator and we enforce that native asset withdraws must be <= address(this).balance / maxWithdrawDenominator. For example, if maxWithdrawDenominator is set to 20, it would take 20 native asset withdraws (each withdrawing max allowed) + 1 native asset withdraw in the same transaction to cause a revert. It is of course known that this doesn’t entirely solve the issue, and the efficacy depends the value chosen and usage. (PR

here ) Status:

Mitigation confirmed. Full details in reports from imare and McToady.

# [M-02] Users can front run the signature of the paymaster operation leading to some problems

- **Contest:** Smart Wallet
- **Slug:** 2024-03-smart-wallet
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-smart-wallet
- **Source snapshot:** competitions/2024-03-smart-wallet/final_report.html

Submitted by Jorgect, also found by cheatc0d3 The paymaster is an extension of the eip-4337, normally the paymaster is willing to pay a user transaction if the account can return the amount of gas at the final of the transaction.

In the context of the coinbase smart wallet, the paymaster is the contract call magicSpend.sol, This contract exposes the normal function needed to be a paymaster:

function validatePaymasterUserOp (PackedUserOperation calldata userOp, bytes32 userOpHash, uint256 maxCost) external returns (bytes memory context, uint256 validationData); function postOp (PostOpMode mode, bytes calldata context, uint256 actualGasCost, uint256 actualUserOpFeePerGas) external; The magic spend is also implementing the entry point deposit, unlock and withdraw functions as required.

Addionally of this the magicSpend is implementing a withdraw functions for users:

file:https:

function withdraw(WithdrawRequest memory withdrawRequest) external { _validateRequest(msg.sender, withdrawRequest); if (!isValidWithdrawSignature(msg.sender, withdrawRequest)) { revert InvalidSignature(); } if (block.timestamp > withdrawRequest.expiry) { revert Expired(); } // reserve funds for gas, will credit user with difference in post op _withdraw(withdrawRequest.asset, msg.sender, withdrawRequest.amount); } Link The problem is that validatePaymasterUserOp is consuming the same signature of the withdraw function, so user can request a transaction through the paymaster, then front runt this transaction calling the withdraw function in the magicSpend (as you notice this transaction is not being processed through the bundler so user can get this withdraw transaction first if he send the correct amount of gas to be included first) making the validatePaymasterUserOp revert because the nonce was already consumed.

## Impact

Are there any griefing attacks that could cause this paymaster to be banned by bundlers?

Paymaster can be banned by bundlers because the user can trigger revert transactions which is one of the reason because bundlers can ban paymasters.

I consider that this has to be another vulnerability, but I decided to put it here because the main problem is the same.

Paymaster can be drained if user front runs the signature given to pay an operation, withdrawing directly funds in the withdraw function, user can do this repeated times until the Paymaster is completely drained.

## Recommended Mitigation Steps

Consider adding another signer for the withdraw function different from the validatePaymasterUserOp signer.

wilsoncusack (Coinbase) acknowledged, but disagreed with severity and commented:

To be clear, the same signature cannot be used twice. The front run is interesting: requires a user to submit a userOp with the withdraw signature in paymasterAndData, and then call from their SCW (via another user op or direct call from an EOA owner) and directly call withdraw. There’s a race condition here, but it could indeed hurt the paymaster’s reputation if pulled off.

3docSec (judge) decreased severity to Medium and commented:

Because the front-run signature would DoS the second one, tokens would be spent only once (invalidating point #2 of the impact), so it looks more like a grieving attack on the MS reputation with no tokens at risk => medium seems more appropriate.
