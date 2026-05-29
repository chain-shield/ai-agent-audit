# Benchmark Ground Truth: Taiko

## Accepted H/M Findings

# Accepted H/M Findings: Taiko

# [H-01] Gas issuance is inflated and will halt the chain or lead to incorrect base fee

- **Contest:** Taiko
- **Slug:** 2024-03-taiko
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-03-taiko
- **Source snapshot:** competitions/2024-03-taiko/final_report.html

Submitted by monrel

- https://github.com/code-423n4/2024-03-taiko/blob/f58384f44dbf4c6535264a472322322705133b11/packages/protocol/contracts/L2/TaikoL2.sol#L140-L143
- https://github.com/code-423n4/2024-03-taiko/blob/f58384f44dbf4c6535264a472322322705133b11/packages/protocol/contracts/L2/TaikoL2.sol#L262-L293
- https://github.com/code-423n4/2024-03-taiko/blob/f58384f44dbf4c6535264a472322322705133b11/packages/protocol/contracts/L2/TaikoL2.sol#L145-L152
- https://github.com/code-423n4/2024-03-taiko/blob/f58384f44dbf4c6535264a472322322705133b11/packages/protocol/contracts/L2/TaikoL2.sol#L140-L143
The base fee calculation in the anchor() function is incorrect. Issuance is over inflated and will either lead to the chain halting or a severely deflated base fee.

## Recommended Mitigation Steps

Issue exactly config.gasTargetPerL1Block for each L1 block.

dantaik (Taiko) confirmed and commented:

This is a valid bug report. Fixed in this PR:

- https://github.com/taikoxyz/taiko-mono/pull/16543
0xean (Judge) commented:

I don’t see a direct loss of funds here and believe M is the correct severity.

2 — Med: Assets not at direct risk, but the function of the protocol or its availability could be impacted, or leak value with a hypothetical attack path with stated assumptions, but external requirements.

3 — High: Assets can be stolen/lost/compromised directly (or indirectly if there is a valid attack path that does not have hand-wavy hypotheticals).

0xmonrel (Warden) commented:

A halted chain leads to frozen funds. The chain will progress for a minimum of 2 blocks since the calculation is correct when lastSyncedBlock =0 and when _l1BlockID-lastSyncedBlock=1 After the second block the base fee will still be correct as long as excess < issuance for both the inflated and correct calculating since both result in excess=1

- https://github.com/code-423n4/2024-03-taiko/blob/f58384f44dbf4c6535264a472322322705133b11/packages/protocol/contracts/L2/TaikoL2.sol#L279-L282
if ( numL1Blocks > 0 ) { uint256 issuance = numL1Blocks * _config.

gasTargetPerL1Block; excess = excess > issuance ?

excess - issuance:

1; } At the block where the base fee is incorrect the chain is halted and funds are locked since the anchor now reverts in perpetuity.

In practice Taiko can easily release all funds by upgrading the contracts but I believe such an intervention should not be considered when evaluating the severity of an issue. From C4 Supreme Court session, Fall 2023 Contract upgradability should never be used as a severity mitigation, i.e. we assume contracts are non-upgradable.

I therefore believe a High is fair here.

0xean (Judge) commented:

I don’t entirely agree since the chain would be halted so soon in its existence, that being said, some amount of funds, albeit small, would likely be lost. @dantaik / @adaki2004 any last comments before leaving as H severity?

adaki2004 (Taiko) commented:

Agreed, can do!

0xean (Judge) commented:

Awarding as H, final decision.

# [H-02] Validity and contests bond ca be incorrectly burned for the correct and ultimately verified transition

- **Contest:** Taiko
- **Slug:** 2024-03-taiko
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-03-taiko
- **Source snapshot:** competitions/2024-03-taiko/final_report.html

Submitted by monrel, also found by t0x1c

- https://github.com/code-423n4/2024-03-taiko/blob/f58384f44dbf4c6535264a472322322705133b11/packages/protocol/contracts/L1/libs/LibProving.sol#L387-L392
- https://github.com/code-423n4/2024-03-taiko/blob/f58384f44dbf4c6535264a472322322705133b11/packages/protocol/contracts/L1/libs/LibProving.sol#L189-L199
- https://github.com/code-423n4/2024-03-taiko/blob/f58384f44dbf4c6535264a472322322705133b11/packages/protocol/contracts/L1/libs/LibVerifying.sol#L178-L189
Both validity and contests bonds can be wrongfully slashed even if the transition ends up being the correct and verified one.

The issue comes from the fact that the history of the final verified transition is not taken into account.

Example 1: Validity bond is wrongfully burned:

Bob Proves transition T1 for parent P1 Alice contests and proves T2 for parent P1 with higher tier proof.

Guardians steps in to correctly prove T1 for parent P2.

At step 2 Bob loses his bond and is permanentley written out of the history of P1

- https://github.com/code-423n4/2024-03-taiko/blob/f58384f44dbf4c6535264a472322322705133b11/packages/protocol/contracts/L1/libs/LibProving.sol#L387-L392
_ts.

validityBond = _tier.

validityBond; _ts.

contestBond = 1; _ts.

contester = address ( 0 ); _ts.

prover = msg.

sender; _ts.

tier = _proof.

tier; Example 2: Contest bond wrongfully slashed:

Alice proves T1 for parent P1 with SGX Bob contests T1 for parent P1 Alice proves T1 with SGX_ZK parent P1 Guardian steps in to correctly disprove T1 with T2 for parent P1 Bob was correct and T1 was ultimately proven false. Bob still loses his contest bond.

When the guardian overrides the proof they can not pay back Bob’s validity or contesting bond. They are only able to pay back a liveness bond

- https://github.com/code-423n4/2024-03-taiko/blob/f58384f44dbf4c6535264a472322322705133b11/packages/protocol/contracts/L1/libs/LibProving.sol#L189-L199
if ( isTopTier ) { // A special return value from the top tier prover can signal this // contract to return all liveness bond.

bool returnLivenessBond = blk.

livenessBond > 0 && _proof.

data.

length == 32 && bytes32 ( _proof.

data ) == RETURN_LIVENESS_BOND; if ( returnLivenessBond ) { tko.

transfer ( blk.

assignedProver, blk.

livenessBond ); blk.

livenessBond = 0; } These funds are now frozen since they are sent to the Guardian contract which has no ability to recover them.

- https://github.com/code-423n4/2024-03-taiko/blob/f58384f44dbf4c6535264a472322322705133b11/packages/protocol/contracts/L1/libs/LibVerifying.sol#L178-L189
uint256 bondToReturn = uint256 ( ts.

validityBond ) + blk.

livenessBond; if ( ts.

prover != blk.

assignedProver ) { bondToReturn -= blk.

livenessBond >> 1; } IERC20 tko = IERC20 ( _resolver.

resolve ( "taiko_token", false )); tko.

transfer ( ts.

prover, bondToReturn ) ts.prover will be the Guardian since they are the last to prove the block

## Recommended Mitigation Steps

The simplest solution is to allow the guardian to pay back validity and contest bonds in the same manner as for liveness bonds. This keeps the simple design while allowing bonds to be recovered if a prover or contesters action is ultimately proven correct.

Guardian will pass in data in _proof.data that specifies the address, tiers and bond type that should be refunded. Given that Guardians already can verify any proof this does not increase centralization.

We also need to not to not recover any reward when we prove with Guardian and _overrideWithHigherProof() is called. If the ts.validityBond reward is sent to the Guardian it will be locked. Instead we need to keep it in TaikoL1 such that it can be recovered as described above +if (_tier.contestBond != 0){ unchecked { if (reward > _tier.validityBond) { _tko.transfer(msg.sender, reward - _tier.validityBond); } else { _tko.transferFrom(msg.sender, address(this), _tier.validityBond - reward); } +} dantaik (Taiko) commented:

This is a valid report but we knew this “flaw” and the current behavior is by design.

The odd that a valid transition is proven, then contested and overwritten by another proof, then proven again with even a higher tier should be rare, if this happens even once, we should know the second prover is buggy and shall change the tier configuration to remove it.

For provers who suffer a loss due to such prover bugs, Taiko foundation may send them compensation to cover there loss. We do not want to handle cover-your-loss payment in the protocol.

adaki2004 (Taiko) confirmed, but disagreed with severity and commented:

This is an attack on the tier system, right ? But the economical disincentives doing so shall be granted by the bonds - not to challenge proofs which we do know are correct, just to make someone lose money as there is no advantage. The challenger would lose even more money - and the correct prover would be refunded by Taiko Foundation.

Severity: medium, (just as:

- https://github.com/code-423n4/2024-03-taiko-findings/issues/227
) 0xean (Judge) commented:

I am going to leave as H, I think there is a direct loss of funds here.

This comment:

The challenger would lose even more money Makes me second guess that slightly, but still think H is correct.

# [H-03] Users will never be able to withdraw their claimed airdrop fully in ERC20Airdrop2.sol contract

- **Contest:** Taiko
- **Slug:** 2024-03-taiko
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-03-taiko
- **Source snapshot:** competitions/2024-03-taiko/final_report.html

Submitted by MrPotatoMagic, also found by Aymen0909, alexfilippov314, pa6kuda, and t4sk Context:

The ERC20Airdrop2.sol contract is for managing Taiko token airdrop for eligible users, but the withdrawal is not immediate and is subject to a withdrawal window.

Users can claim their tokens within claimStart and claimEnd. Once the claim window is over at claimEnd, they can withdraw their tokens between claimEnd and claimEnd + withdrawalWindow. During this withdrawal period, the tokens unlock linearly i.e. the tokens only become fully withdrawable at claimEnd + withdrawalWindow.

Issue:

The issue is that once the tokens for a user are fully unlocked, the withdraw() function cannot be called anymore due to the ongoingWithdrawals modifier having a strict claimEnd + withdrawalWindow < block.timestamp check in its second condition.

Impact:

Although the tokens become fully unlocked when block.timestamp = claimEnd + withdrawalWindow, it is extremely difficult or close to impossible for normal users to time this to get their full allocated claim amount. This means that users are always bound to lose certain amount of their eligible claim amount. This lost amount can be small for users who claim closer to claimEnd + withdrawalWindow and higher for those who partially claimed initially or did not claim at all thinking that they would claim once their tokens are fully unlocked.

## Recommended Mitigation Steps

In the modifier ongoingWithdrawals(), consider adding a buffer window in the second condition that gives users enough time to claim the fully unlocked tokens.

uint256 constant bufferWindow = X mins / hours / days; modifier ongoingWithdrawals () { if ( claimEnd > block.

timestamp || claimEnd + withdrawalWindow < block.

timestamp + bufferWindow ) { revert WITHDRAWALS_NOT_ONGOING (); } _; } dantaik (Taiko) commented:

Fixed in

- https://github.com/taikoxyz/taiko-mono/pull/16596
adaki2004 (Taiko) confirmed and commented:

It is indeed a bug in the flow, while we removed Airdrop2, it is still a confirmed finding on the repo for auditing.

# [H-04] Taiko L1 - Proposer can maliciously cause loss of funds by forcing someone else to pay prover’s fee

- **Contest:** Taiko
- **Slug:** 2024-03-taiko
- **Finding ID:** H-04
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-03-taiko
- **Source snapshot:** competitions/2024-03-taiko/final_report.html

Submitted by zzebra83, also found by MrPotatoMagic, monrel, mojito_auditor, and ladboy233

- https://github.com/code-423n4/2024-03-taiko/blob/0d081a40e0b9637eddf8e760fabbecc250f23599/packages/protocol/contracts/L1/hooks/AssignmentHook.sol#L113-L116
- https://github.com/code-423n4/2024-03-taiko/blob/0d081a40e0b9637eddf8e760fabbecc250f23599/packages/protocol/contracts/L1/libs/LibProposing.sol#L85-L87
- https://github.com/code-423n4/2024-03-taiko/blob/0d081a40e0b9637eddf8e760fabbecc250f23599/packages/protocol/contracts/L1/libs/LibProposing.sol#L249-L255
Proposal of new blocks triggers a call to proposeBlock in the libProposing library. In that function, there is this the following block of code:

if (params.coinbase == address(0)) { params.coinbase = msg.sender; } This sets the params.coinbase variable set by the caller of the function to be the msg.sender if it was empty.

As part of the process of proposal, hooks can be called of type AssignmentHook. An assignment hook’s onBlockProposed will be triggered as follows:

// When a hook is called, all ether in this contract will be send to the hook.

// If the ether sent to the hook is not used entirely, the hook shall send the Ether // back to this contract for the next hook to use.

// Proposers shall choose use extra hooks wisely.

IHook(params.hookCalls[i].hook).onBlockProposed{ value: address(this).balance }( blk, meta_, params.hookCalls[i].data ); Notice how the meta data is passed to this function. Part of the function of the onBlockProposed is to pay the assigned prover their fee and the payee should be the current proposer of the block. this is done as follows:

// The proposer irrevocably pays a fee to the assigned prover, either in // Ether or ERC20 tokens.

if (assignment.feeToken == address(0)) { // Paying Ether _blk.assignedProver.sendEther(proverFee, MAX_GAS_PAYING_PROVER); } else { // Paying ERC20 tokens IERC20(assignment.feeToken).safeTransferFrom( _meta.coinbase, _blk.assignedProver, proverFee ); } Notice how if the payment is in ERC20 tokens, the payee will be the variable _meta.coinbase, and like we showed earlier, this can be set to any arbitrary address by the proposer. This can lead to a scenario as such:

proposer A approves the assignmentHook contract to spend a portion of their tokens, the allowance is set higher than the actual fee they will be paying.

proposer A proposes a block, and a fee is charged and payed to the assigned prover, but there remains allowance that the assignment hook contract can still use.

proposer B proposes a block and sets params.coinbase as the the address of proposer A.

proposer A address will be the payee of the fee for the assigned prover for the block proposed by proposer B.

The scenario above describes how someone can be forced maliciously to pay fees for block proposals by other actors.

## Recommended Mitigation Steps

A simple fix to this to ensure the block proposer will always be the msg.sender, as such:

if (params.coinbase == address(0 || params.coinbase != msg.sender)) { params.coinbase = msg.sender; } dantaik (Taiko) confirmed and commented:

This is a valid bug report. It has been fixed here:

- https://github.com/taikoxyz/taiko-mono/pull/16327

# [H-05] Signatures can be replayed in withdraw() to withdraw more tokens than the user originally intended.

- **Contest:** Taiko
- **Slug:** 2024-03-taiko
- **Finding ID:** H-05
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-03-taiko
- **Source snapshot:** competitions/2024-03-taiko/final_report.html

withdraw() to withdraw more tokens than the user originally intended.

Submitted by lightoasis, also found by 0xleadwizard, wangxx2026, alexfilippov314, ladboy233, and Tendency Signatures can be replayed in withdraw() to withdraw more tokens than the user originally intended.

## Vulnerability Details

In the TimelockTokenPool.sol contracts, users can provide a signature to allow someone else to withdraw all their withdrawable tokens on their behalf using their signature.

TimelockTokenPool.sol#L170) function withdraw(address _to, bytes memory _sig) external { if (_to == address(0)) revert INVALID_PARAM(); bytes32 hash = keccak256(abi.encodePacked("Withdraw unlocked Taiko token to: ", _to)); @> address recipient = ECDSA.recover(hash, _sig); _withdraw(recipient, _to); } As seen from above, the signature provided does not include a nonce and this can lead to signature replay attacks. Due to the lack of a nonce, withdraw() can be called multiple times with the same signature. Therefore, if a user provides a signature to withdraw all his withdrawable tokens at one particular time, an attacker can repeatedly call withdraw() with the same signature to withdraw more tokens than the user originally intended. The vulnerability is similar to

Arbitrum H-01 where user’s signatures could be replayed to use up more votes than a user intended due to a lack of nonce.

## Recommended Mitigation Steps

Consider using a nonce or other signature replay protection in the TimelockTokenPool contract.

dantaik (Taiko) confirmed and commented:

Valid bug report, trying to fix it in this PR:

- https://github.com/taikoxyz/taiko-mono/pull/16611/files
Medium Risk Findings (14)

# [M-01] There is no slippage check for the eth deposits processing in the LibDepositing.processDeposits

- **Contest:** Taiko
- **Slug:** 2024-03-taiko
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-taiko
- **Source snapshot:** competitions/2024-03-taiko/final_report.html

LibDepositing.processDeposits Submitted by Shield, also found by ladboy233

- https://github.com/code-423n4/2024-03-taiko/blob/main/packages/protocol/contracts/L1/libs/LibDepositing.sol#L138-L142
- https://github.com/code-423n4/2024-03-taiko/blob/main/packages/protocol/contracts/L1/TaikoL1.sol#L209-L211
- https://github.com/code-423n4/2024-03-taiko/blob/main/packages/protocol/contracts/L1/libs/LibDepositing.sol#L83
- https://github.com/code-423n4/2024-03-taiko/blob/main/packages/protocol/contracts/L1/libs/LibDepositing.sol#L93
- https://github.com/code-423n4/2024-03-taiko/blob/main/packages/protocol/contracts/L1/libs/LibDepositing.sol#L101
The LibDepositing.depositEtherToL2 function is called by the TaikoL1.depositEtherToL2 to deposit ether to the L2 chain. The maximum number of unprocessed eth deposits are capped at _config.ethDepositRingBufferSize - 1 as shown here:

unchecked { return _amount >= _config.

ethDepositMinAmount && _amount <= _config.

ethDepositMaxAmount && _state.

slotA.

numEthDeposits - _state.

slotA.

nextEthDepositToProcess < _config.ethDepositRingBufferSize - 1; } The Taiko configuration states that ethDepositRingBufferSize == 1024. Hence the maximum unprocessed eth deposits allowed by the taiko L1 contract is capped at 1023.

When the L2 block is proposed by calling the LibProposing.proposeBlock function by the TaikoL1 contract, it processes the unprocessed eth deposits by calling the LibDepositing.processDeposits. But it is allowed to process at most 32 eth deposits per block as per the following conditional check in the processDeposits function.

deposits_ = new TaikoData.

EthDeposit [( numPending.

min ( _config.

ethDepositMaxCountPerBlock )); Here the ethDepositMaxCountPerBlock == 32 as configured in the TaikoL1.getConfig function.

And the fee amount for each of the eth deposits are calculated as follows:

uint96 fee = uint96 ( _config.

ethDepositMaxFee.

min ( block.

basefee * _config.

ethDepositGas )); uint96 _fee = deposits_ [ i ].

amount > fee ?

fee:

deposits_ [ i ].

amount; deposits_ [ i ].

amount -= _fee; Hence the deposited eth amount is deducted by the calculated _fee amount for the eth deposit transaction. If the basefee of the L1 block increases significantly then the maximum fee of ethDepositMaxFee: 1 ether / 10 will be applied. Thus deducting that amount from the transferred eth deposit to the recipient in L2.

Now let’s consider the following scenario:

nextEthDepositToProcess is currently 100.

numEthDeposits is 1060 currently.

The number of proposed L2 blocks required to process 1060th eth deposit is = 1060 - 100 / 32 = 30 L2 blocks.

As a result all the above 30 L2 blocks will not be proposed in a single L1 block and will require multiple L1 blocks for it.

If there is huge congestion in the mainnet during this time the block.basefee of the subsequent L1 blocks would increase. And this could prompt the maximum fee of _config.ethDepositMaxFee to be charged on the deposited amount (in the LibDepositing.processDeposits function, since subsequent L1 block would have a higher block.basefee ) thus prompting loss of funds on the recipient.

For example let’s assume the depositor deposit 1.1 ether and current gas fee is 0.01 ether. Hence the recipient expects to receive approximately 1.09 ether at the time of the deposit on L1. But when the deposit is processed in a subsequent L1 block, if the fee amount increases to the maximum amount of 0.1 ether then the recipient will only get approximately 1 ether only. This will cost the recipient a loss of 0.9 ether. If there was a slippage check a depositor can set for his eth deposits then the he can prevent excessive gas costs during processing.

## Recommended Mitigation Steps

Hence it is recommended to add a slippage check for the fee amount of the deposited eth amount in the LibDepositing.processDeposits function, since depositEtherToL2 and processDepositare two different transactions with a delay during which the L1 block basefee can increase significantly causing loss of funds to the recipient in the form of fee increase.

dantaik (Taiko) acknowledged, but disagreed with severity and commented:

Thank you for the feedback. The current issue is valid but minor as we believe users can choose not to deposit Ether using the depositEtherToL2 function if there are already many deposits pending in the queue. Going forward, the processing of such deposits will likely be moved to the node software directly.

0xean (Judge) commented:

Agree this is valid, the impact is most likely small, but I think the likelihood of it occurring at some point is relatively high. The user is exposed to non-deterministic behavior that they cannot fully understand ahead of signing a transaction.

# [M-02] The top tier prover can not re-prove

- **Contest:** Taiko
- **Slug:** 2024-03-taiko
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-taiko
- **Source snapshot:** competitions/2024-03-taiko/final_report.html

Submitted by Shield, also found by zzebra83 and Tendency

- https://github.com/code-423n4/2024-03-taiko/blob/main/packages/protocol/contracts/L1/libs/LibProving.sol#L219-L236
- https://github.com/code-423n4/2024-03-taiko/blob/main/packages/protocol/contracts/L1/libs/LibProving.sol#L389
In the LibProving.proveBlock function the top tier prover is allowed to prove a new transition as long as the new transition is different from the previous transition and assert conditional checks pass.

if ( sameTransition ) revert L1_ALREADY_PROVED (); if ( isTopTier ) { // The top tier prover re-proves.

assert ( tier.

validityBond == 0 ); assert ( ts.

validityBond == 0 && ts.

contestBond == 0 && ts.

contester == address ( 0 )); But the assert condition of this logic is wrong since it checks for the ts.contestBond == 0 where as it should be ts.contestBond == 1 since 1 is set as the default value for ts.contestBond parameter for gas savings as shown below:

ts_.

contestBond = 1; // to save gas As a result of the even though code expects the top-tier prover to re-prove a different transition, the transaction will revert.

## Recommended Mitigation Steps

Hence recommended to update the ts.contestBond == 0 in the second assert statement to ts.contestBond == 1 in the LibProving.proveBlock function.

dantaik (Taiko) confirmed and commented:

This is a valid bug report, it has been fixed already here:

- https://github.com/taikoxyz/taiko-mono/pull/16543

# [M-03] retryMessage unable to handle edge cases.

- **Contest:** Taiko
- **Slug:** 2024-03-taiko
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-taiko
- **Source snapshot:** competitions/2024-03-taiko/final_report.html

Submitted by josephdara, also found by josephdara, grearlake, Shield, MrPotatoMagic ( 1, 2 ), Aymen0909, ladboy233, iamandreiski, lanrebayode77, t0x1c, and Fassi_Security ( 1, 2 ) The function retryMessage() is unable to handle some edge scenarios listed below.

Reverting or refunding a sender when the receiver is banned after the transaction is placed in a RETRIABLE state.

Message that is suspended after the transaction is placed in a RETRIABLE state.

## Recommended Mitigation Steps

Recheck necessary details to verify that the transaction is still good to go. Check the proofReceipt[msgHash].receivedAt and the addressBanned[_addr] adaki2004 (Taiko) disputed and commented:

I’d dispute this with an addition (see end sentence).

2 cases mentioned here:

Reverting or refunding a sender when the receiver is banned after the transaction is placed in a RETRIABLE state.

Intentional to NOT refund sender when he/she is banned. (Tho we might remove the “banAddress”, because it confuses a lot of people. The original intention behind banning an address is: NOT be able to call another, very important contract ( message.to ) on behalf of the Bridge, like the SignalService.) Message that is suspended after the transaction is placed in a RETRIABLE state. Not to refund suspended messages is a feature, it is a failsafe/security mechanism. In such case we need to use it, it would be a severe situation and we do not necessary want to refund (by default) the owner, since it might be a fake message on the destination chain. (That can be one reason - so makes no sense to refund). Also suspension would never happen after RETRIABLE. If we suspend message it is between NEW and (DONE or RETRIABLE).

So as we considering removing banning addresses (and not allow SignalService to be called) for avoid confusion, but not because it is an issue, but a simplification and to avoid confusion.

- https://github.com/taikoxyz/taiko-mono/pull/16604
0xean (Judge) commented:

Using this issue to aggregate all of the issues around ban list functionality that has since been removed from the sponsors code base.

Note: For full discussion, see here.

# [M-04] A recalled ERC20 bridge transfer can lock tokens in the bridge

- **Contest:** Taiko
- **Slug:** 2024-03-taiko
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-taiko
- **Source snapshot:** competitions/2024-03-taiko/final_report.html

Submitted by monrel, also found by monrel

- https://github.com/code-423n4/2024-03-taiko/blob/f58384f44dbf4c6535264a472322322705133b11/packages/protocol/contracts/tokenvault/ERC20Vault.sol#L320-L335
- https://github.com/code-423n4/2024-03-taiko/blob/f58384f44dbf4c6535264a472322322705133b11/packages/protocol/contracts/tokenvault/adapters/USDCAdapter.sol#L43-L45
- https://github.com/circlefin/stablecoin-evm/blob/0828084aec860712531e8d79fde478927a34b3f4/contracts/v1/FiatTokenV1.sol#L133-L136
A recalling ERC20 bridge transfer can lock funds in the bridge if the call to mint tokens fail on the source chain. Depending on the Native token logic this could either be a permanent lock or a lock of an unknown period of time.

Example of how this can happen with the provided USDCAdapter:

USDC limits the amount of USDC that can be minted on each chain by giving each minter a minting allowance. If the minting allowance is reach minting will revert. If this happens in a recalled message the tokens together with the ETH value is locked.

## Recommended Mitigation Steps

Add new functionality in the vault that allows users to send a new message to the destination chain again with new message data if onMessageRecalls() can not mint tokens. We give users the ability to redeem for canonical tokens instead of being stuck.

dantaik (Taiko) acknowledged and commented:

Thank you for your feedback. In your example, if the user cannot mint tokens in USDCAdapter by using recallMessage, the user can wait and call recallMessage again. That’s why recallMessage is retriable.

There is no perfect solution here, and I personally don’t want to make the bridge too complicated by introducing this re-send-another-message feature.

Adding a warning on the bridge UI to show a warning message might be a good solution, something like “USDC on Taiko has reached 95% of its max supply cap, bridging USDC to Taiko may end up your fund becoming unavailable for some time until others bridge USDC away from Taiko”.

0xean (Judge) commented:

The sponsor comments simply show their isn’t a great solution to the problem, it still represents a loss of user funds (if it goes on forever) or a denial of service and a risk that users should be aware of.

@dontonka / @adaki2004 (Taiko) any last comments here?

adaki2004 (Taiko) commented:

We are OK with med, no issue. Please proceed accordinlgy - as we dont have:

the perfect solution to the problem the intention to fix it ATM - since we wont be using any native tokens anytime soon.

But please proceed the way which is suitable for the wardens better, we appreciate their efforts. (So not questioning the severity)

# [M-05] Bridge watcher can forge arbitrary message and drain bridge

- **Contest:** Taiko
- **Slug:** 2024-03-taiko
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-taiko
- **Source snapshot:** competitions/2024-03-taiko/final_report.html

Submitted by monrel, also found by josephdara, Shield, and t0x1c

- https://github.com/code-423n4/2024-03-taiko/blob/f58384f44dbf4c6535264a472322322705133b11/packages/protocol/contracts/bridge/Bridge.sol#L82-L95
- https://github.com/code-423n4/2024-03-taiko/blob/f58384f44dbf4c6535264a472322322705133b11/packages/protocol/contracts/bridge/Bridge.sol#L230-L231
The bridge_watchdog role can forge arbitrary messages and drain the bridge of all ETH and tokens.

## Recommended Mitigation Steps

Un-suspended messages should be set to 0 and be proven or re-proven.

function suspendMessages( bytes32[] calldata _msgHashes, bool _suspend ) external onlyFromOwnerOrNamed("bridge_watchdog") { + uint64 _timestamp = _suspend ? type(uint64).max: 0; for (uint256 i; i < _msgHashes.length; ++i) { bytes32 msgHash = _msgHashes[i]; proofReceipt[msgHash].receivedAt = _timestamp; emit MessageSuspended(msgHash, _suspend); } dantaik (Taiko) confirmed and commented:

This is a valid bug report. The bug has been fixed in this PR:

- https://github.com/taikoxyz/taiko-mono/pull/16545
0xean (Judge) decreased severity to Medium and commented:

Good report, but I am not sure it qualifies as H severity and most likely should be M.

I think there is a pre-condition here (a malicious watchdog).

2 — Med: Assets not at direct risk, but the function of the protocol or its availability could be impacted, or leak value with a hypothetical attack path with stated assumptions, but external requirements.

Agree that if this attack is feasible, it represents privilege escalation.

As pointed out previously:

Privilege escalation issues are judged by likelihood and impact and their severity is uncapped.

Note: For full discussion, see here.

# [M-06] First block proposer check in the LibProposing._isProposerPermitted function is errorneous

- **Contest:** Taiko
- **Slug:** 2024-03-taiko
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-taiko
- **Source snapshot:** competitions/2024-03-taiko/final_report.html

LibProposing._isProposerPermitted function is errorneous Submitted by Shield, also found by monrel and blockdev

- https://github.com/code-423n4/2024-03-taiko/blob/main/packages/protocol/contracts/L1/libs/LibProposing.sol#L93-L94
- https://github.com/code-423n4/2024-03-taiko/blob/main/packages/protocol/contracts/L1/libs/LibProposing.sol#L299-L317
The LibProposing.proposeBlock function calls the _isProposerPermitted private function, to ensure if the proposer is set. Only that specific address has the permission to propose the block.

In the _isProposerPermitted function, for the first block after the genesis block only the proposerOne is allowed to propose the first block as shown below:

address proposerOne = _resolver.

resolve ( "proposer_one", true ); if ( proposerOne != address ( 0 ) && msg.

sender != proposerOne ) { return false; } But the issue here is that when the msg.sender == proposerOne the function does not return true if the following conditions occur.

If the proposer != address(0) && msg.sender != proposer. In which case even though the msg.sender == proposerOne is true for the first block the _isProposerPermitted will still return false thus reverting the block proposer for the first block.

Hence even though the proposer_one is the proposer of the first block the transaction will still revert if the above mentioned conditions occur and the _isProposerPermitted returns false for the first block after the genesis block.

Hence this will break the block proposing logic since the proposal of the first block after the genesis block reverts thus not allowing subsequent blocks to be proposed.

## Recommended Mitigation Steps

It is recommended to add logic in the LibProposing._isProposerPermitted function to return true when the msg.sender == proposerOne, for proposing the first block after genesis block.

dantaik (Taiko) confirmed and commented:

I think this is a valid bug: fixing it here

# [M-07] Incorrect _ Essential init() function is used in TaikoToken making snapshooter devoid of calling snapshot()

- **Contest:** Taiko
- **Slug:** 2024-03-taiko
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-taiko
- **Source snapshot:** competitions/2024-03-taiko/final_report.html

Essential init() function is used in TaikoToken making snapshooter devoid of calling snapshot() Submitted by MrPotatoMagic, also found by Limbooo, imare, and t0x1c

- https://github.com/code-423n4/2024-03-taiko/blob/f58384f44dbf4c6535264a472322322705133b11/packages/protocol/contracts/L1/TaikoToken.sol#L34
- https://github.com/code-423n4/2024-03-taiko/blob/f58384f44dbf4c6535264a472322322705133b11/packages/protocol/contracts/L1/TaikoToken.sol#L52
The EssentialContract.sol contract is inherited by the TaikoToken contract. This essential contract contains two _ Essential init() functions, one with an owner parameter only (see here ) and the other with owner and address manager parameters (see here ).

The issue with the current code is that it uses the _ Essential init() function with the owner parameter only. This would cause the onlyFromOwnerOrNamed(“snapshooter”) modifier on the snapshot function to not be able to resolve the snapshooter role since the address manager contract was never set during initialization, thus causing a revert.

Due to this:

Snapshooter role is denied from taking snapshots.

Timely snapshots for certain periods could have failed by the snapshooter since they would have required the owner to jump in by the time the issue was realized.

Correct/Intended functionality of the protocol is affected i.e. the snapshooter role assigned to an address cannot ever perform its tasks validly.

## Recommended Mitigation Steps

In the init() function, consider using the _ Essential init() function with the owner and address manager parameters instead of the _ Essential init() function with the owner parameter. This would allow the snapshooter address to proceed with taking snapshots as expected.

dantaik (Taiko) confirmed and commented:

This is a valid bug report. The bug is fixed by

- https://github.com/taikoxyz/taiko-mono/commit/c64ec193c95113a4c33692289e23e8d9fa864073

# [M-08] Bridged tokens would be lost if sender and receiver are contracts that don’t implement fallback/receive

- **Contest:** Taiko
- **Slug:** 2024-03-taiko
- **Finding ID:** M-08
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-taiko
- **Source snapshot:** competitions/2024-03-taiko/final_report.html

Submitted by Shield, also found by Shield When a bridged token is received on the dest chain, ERC20Vault.onMessageInvocation() is being called.

onMessageInvocation() always calls to.sendEther(msg.value) even when the msg.value is zero.

sendEther() would attempt to call the contract with the value supplied and empty data. If the to address ia a contract that doesn’t implement neither the fallback function nor receive then the entire transaction would revert.

The same issue occurs during recalling the message, if the sender is also a contract that doesn’t implement neither a fallback nor receive then the recalling would fail as well.

## Impact

Funds would be lost, since the sending can’t be finalized and recovering would revert as well.

While this might be considered a user error when sending a value that’s greater than zero (they should’ve checked that the to address implements the receiver), this can’t be said about the case where the value is zero - the user won’t expect the vault to call the to contract with zero value.

## Recommended Mitigation Steps

Don’t call sendEther() when the value is zero Or modify sendEther() to return when the value is zero Find a solution for cases when the value is non-zero This one is a bit more complicated, one way might be to allow the sender to request the ERC20 token while giving up on the ETH dantaik (Taiko) confirmed and commented:

We have change the sendEther function such that if the amount is 0, there is no further action and the sendEther function simply returns.

If if default and receive functions are both unimplemented on the destination chain for the to address, then the owner can fail the message with retryMessage({…, _lastAttemp=true}); >or use failMessage(…), then on the source chain, the owner can call recallMessage to get back his tokens.

At the end of the day, the user must trust the dapp that use our Bridge to set the right message parameters.

# [M-09] LibProposing:proposeBlock allows blocks with a zero parentMetaHash to be proposed after the genesis block and avoid parent block verification

- **Contest:** Taiko
- **Slug:** 2024-03-taiko
- **Finding ID:** M-09
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-taiko
- **Source snapshot:** competitions/2024-03-taiko/final_report.html

Submitted by joaovwfreire

- https://github.com/code-423n4/2024-03-taiko/blob/main/packages/protocol/contracts/L1/libs/LibProposing.sol#L108
- https://github.com/code-423n4/2024-03-taiko/blob/main/packages/protocol/contracts/L1/libs/LibProposing.sol#L213
- https://github.com/code-423n4/2024-03-taiko/blob/main/packages/protocol/contracts/L1/libs/LibProving.sol#L121
The proposeBlock at the LibProposing library has the following check to ensure the proposed block has the correct parentMetaHash set:

function proposeBlock ( TaikoData.State storage _state, TaikoData.Config memory _config, IAddressResolver _resolver, bytes calldata _data, bytes calldata _txList ) internal returns (TaikoData.BlockMetadata memory meta_, TaikoData.EthDeposit[] memory deposits_ ) {...

if ( params.

parentMetaHash != 0 && parentMetaHash != params.

parentMetaHash ) { revert L1_UNEXPECTED_PARENT (); }...

} However, there are no sanity checks to ensure params.parentMetaHash is not zero outside of the genesis block.

## Impact

Malicious proposers can propose new blocks without any parentMetaHash. This can induce a maliciously-generated block to be artificially contested as the final block relies on data held by the meta_ variable. Snippet 1:

TaikoData.

Block memory blk = TaikoData.

Block ({ metaHash:

keccak256 ( abi.

encode ( meta_ )),...

}) This also generates issues for independent provers, as they may not utilize the proposed block’s data to attempt to prove it and utilize the correct parentMetaHash, which will make the LibProving:proveBlock call revert with an L1_BLOCK_MISTATCH error:

function proveBlock...

if (blk.blockId != _meta.id || blk.metaHash != keccak256 (abi.encode( _meta ))) { revert L1_BLOCK_MISMATCH (); }...

} Also, according to the documentation, If the parent block hash is incorrect, the winning transition won’t be used for block verification, and the prover will forfeit their validity bond entirely.

If a maliciously proposed block with zero parent block hash is contested and a higher-tier prover ends up proving the proposed block, then he/she loses its own validity bond.

## Recommended Mitigation Steps

Make sure to check the parentMetaHash value is not zero if it isn’t at the genesis block, otherwise users are going to be able to wrongly induce contestations.

adaki2004 (Taiko) confirmed

# [M-10] The decision to return the liveness bond depends solely on the last guardian

- **Contest:** Taiko
- **Slug:** 2024-03-taiko
- **Finding ID:** M-10
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-taiko
- **Source snapshot:** competitions/2024-03-taiko/final_report.html

Submitted by alexfilippov314, also found by t0x1c

- https://github.com/code-423n4/2024-03-taiko/blob/f58384f44dbf4c6535264a472322322705133b11/packages/protocol/contracts/L1/provers/GuardianProver.sol#L46
- https://github.com/code-423n4/2024-03-taiko/blob/f58384f44dbf4c6535264a472322322705133b11/packages/protocol/contracts/L1/libs/LibProving.sol#L192

## Vulnerability details

The GuardianProver contract is a multisig that might contest any proof in some exceptional cases (bugs in the prover or verifier). To contest a proof, a predefined number of guardians should approve a hash of the message that includes _meta and _tran.

function approve ( TaikoData.BlockMetadata calldata _meta, TaikoData.Transition calldata _tran, TaikoData.TierProof calldata _proof ) external whenNotPaused nonReentrant returns ( bool approved_ ) { if ( _proof.

tier != LibTiers.

TIER_GUARDIAN ) revert INVALID_PROOF (); bytes32 hash = keccak256 ( abi.

encode ( _meta, _tran )); approved_ = approve ( _meta.

id, hash ); if ( approved_ ) { deleteApproval ( hash ); ITaikoL1 ( resolve ( "taiko", false )).

proveBlock ( _meta.

id, abi.

encode ( _meta, _tran, _proof )); } emit GuardianApproval ( msg.

sender, _meta.

id, _tran.

blockHash, approved_ ); } The issue arises from the fact that the approved message doesn’t include the _proof. It means that the last approving guardian can provide any desired value in the _proof. The data from the _proof is used to determine whether it is necessary to return the liveness bond to the assigned prover or not:

if ( isTopTier ) { // A special return value from the top tier prover can signal this // contract to return all liveness bond.

bool returnLivenessBond = blk.

livenessBond > 0 && _proof.

data.

length == 32 && bytes32 ( _proof.

data ) == RETURN_LIVENESS_BOND; if ( returnLivenessBond ) { tko.

transfer ( blk.

assignedProver, blk.

livenessBond ); blk.

livenessBond = 0; } As a result, the last guardian can solely decide whether to return the liveness bond to the assigned prover or not.

## Impact

The decision to return the liveness bond depends solely on the last guardian.

## Recommended Mitigation Steps

Consider including the _proof in the approved message.

bytes32 hash = keccak256 ( abi.

encode ( _meta, _tran, _proof )); dantaik (Taiko) confirmed and commented:

Now guardian provers must also agree on whether liveness bond should be returned bytes32( proof.data) == LibStrings.H RETURN LIVENESS BOND, so the hash they sign is now:

bytes32 hash = keccak256(abi.encode(_meta, _tran, _proof.data)); rather than the previous code:

bytes32 hash = keccak256(abi.encode(_meta, _tran));

# [M-11] Proposers would choose to avoid higher tier by exploiting non-randomness of parameter used in getMinTier()

- **Contest:** Taiko
- **Slug:** 2024-03-taiko
- **Finding ID:** M-11
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-taiko
- **Source snapshot:** competitions/2024-03-taiko/final_report.html

Submitted by t0x1c, also found by Mahi_Vasisth The issue exists for both MainnetTierProvider.sol and TestnetTierProvider.sol. For this report, we shall concentrate only on describing it via MainnetTierProvider.sol.

The proving tier is chosen by the getMinTier() function which accepts a _rand param.

File:

contracts / L1 / tiers / MainnetTierProvider.

sol 66:

function getMinTier ( uint256 _rand ) public pure override returns ( uint16 ) { 67:

// 0.1% require SGX + ZKVM; all others require SGX 68: @---> if ( _rand % 1000 == 0 ) return LibTiers.

TIER_SGX_ZKVM; 69:

else return LibTiers.

TIER_SGX; 70: } If _rand % 1000 == 0, a costlier tier TIER_SGX_ZKVM is used instead of the cheaper TIER_SGX. The _rand param is passed in the form of meta_.difficulty which is calculated inside proposeBlock():

File:

contracts / L1 / libs / LibProposing.

sol 199:

// Following the Merge, the L1 mixHash incorporates the 200:

// prevrandao value from the beacon chain. Given the possibility 201:

// of multiple Taiko blocks being proposed within a single 202:

// Ethereum block, we choose to introduce a salt to this random 203:

// number as the L2 mixHash.

204: @---> meta_.

difficulty = keccak256 ( abi.

encodePacked ( block.

prevrandao, b.

numBlocks, block.

number )); 205:

206:

// Use the difficulty as a random number 207:

meta_.

minTier = ITierProvider ( _resolver.

resolve ( "tier_provider", false )).

getMinTier ( 208: @---> uint256 ( meta_.

difficulty ) 209: ); As can be seen, all the parameters used in L204 to calculate meta_.difficulty can be known in advance and hence a proposer can choose not to propose when meta_.difficulty modulus 1000 equals zero, because in such cases it will cost him more to afford the proof (sgx + zk proof in this case).

## Impact

Since the proposer will now wait for the next or any future block to call proposeBlock() instead of the current costlier one, transactions will now take longer to finalilze.

If _rand were truly random, it would have been an even playing field in all situations as the proposer wouldn’t be able to pick & choose since he won’t know in advance which tier he might get. We would then truly have:

67:

// 0.1% require SGX + ZKVM; all others require SGX

## Recommended Mitigation Steps

Consider using VRF like solutions to make _rand truly random.

dantaik (Taiko) acknowledged and commented:

This is a very well known issue.

Using VRF creates a third party dependency which may be a bigger risk for a Based rollup. We’ll monitor how this plays out and mitigate the issue later.

adaki2004 (Taiko) commented:

Eventually we will have only 1 (1 “aggregated ZK multiproof”) proof tier, which will be the default/min too. (Maybe keeping guardian for a while to be as a failsafe, but that one also cannot be “picked” with thispseudoe random calculation). Also Taiko foundation will run a proposer node, so in case noone is willing to propose to avoid fees, we will, regardless of cost - at least until we reach the 1 tier maturity.

genesiscrew (Warden) commented:

Considering this report and the responses from the sponsors, I am unable to see how this would impact the function of the protocol in such a way that would deem it a medium risk. I personally think this is informational. The report states proving will take longer because it assumes all proposers will want to avoid paying fees because they can predict the block difficulty. I find that a bit of a stretch.

adaki2004 (Taiko) commented:

Not the proving but the liveness (proposing) would take longer as provers would deny to grant signatures to prove blocks - which’s evaluation i happening during proposeBlock.

But at least +2 years post mainnet taiko foundation is commited to proposeBlock every X time intervals (even if not breaking even) to keep the liveness and get over this.

And as stated, by the time hopefully this minTier() will vanish in that time - hopefully even in months after launch (not years) when ZK is cheap enough. So for now we would say it is a known issue, we are aware of.

t0x1c (Warden) commented:

Thank you for the inputs. From what I see, this is being acknowledged by the sponsor as a valid issue which is known to the team. Also important to note that it wasn’t mentioned in the list of C4 “known issues” section on the audit page, so should qualify as a Medium.

adaki2004 (Taiko) commented:

Can accept medium.

# [M-12] Invocation delays are not honoured when protocol unpauses

- **Contest:** Taiko
- **Slug:** 2024-03-taiko
- **Finding ID:** M-12
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-taiko
- **Source snapshot:** competitions/2024-03-taiko/final_report.html

Submitted by t0x1c

- https://github.com/code-423n4/2024-03-taiko/blob/main/packages/protocol/contracts/common/EssentialContract.sol#L78
- https://github.com/code-423n4/2024-03-taiko/blob/main/packages/protocol/contracts/bridge/Bridge.sol#L258
Context:

The protocol has pause() and unpause() functions inside EssentialContract.sol which are tracked throughout the protocol via the modifiers whenPaused and whenNotPaused.

Issue:

Various delays and time lapses throughout the protocol ignore the effect of such pauses. The example in focus being that of processMessage() which does not take into account the pause duration while checking invocationDelay and invocationExtraDelay. One impact of this is that it allows a non-preferred executor to front run a preferredExecutor, after an unpause.

File:

contracts / bridge / Bridge.

sol 233: @---> ( uint256 invocationDelay, uint256 invocationExtraDelay ) = getInvocationDelays (); 234:

235:

if (!

isMessageProven ) { 236:

if (!

_proveSignalReceived ( signalService, msgHash, _message.

srcChainId, _proof )) { 237:

revert B_NOT_RECEIVED (); 238: } 239:

240:

receivedAt = uint64 ( block.

timestamp ); 241:

242:

if ( invocationDelay != 0 ) { 243:

proofReceipt [ msgHash ] = ProofReceipt ({ 244:

receivedAt:

receivedAt, 245:

preferredExecutor:

_message.

gasLimit == 0 ?

_message.

destOwner:

msg.

sender 246: }); 247: } 248: } 249:

250:

if ( invocationDelay != 0 && msg.

sender != proofReceipt [ msgHash ].

preferredExecutor ) { 251:

// If msg.sender is not the one that proved the message, then there 252:

// is an extra delay.

253:

unchecked { 254:

invocationDelay += invocationExtraDelay; 255: } 256: } 257:

258: @---> if ( block.

timestamp >= invocationDelay + receivedAt ) { Description & Impact Consider the following flow:

Assumption:

invocationDelay = 60 minutes and invocationExtraDelay = 30 minutes.

A message is sent.

First call to processMessage() occurred at t where it was proven by Bob i.e. its receivedAt = t. Bob is marked as the preferredExecutor.

Preferred executor should be able to call processMessage() at t+60 while a non-preferred executor should be able to call it only at t+90 due to the code logic on L250.

At t+55, protocol is paused.

At t+100, protocol is unpaused.

Impact:

The 30-minute time window advantage which the preferred executor had over the non-preferred one is now lost to him.

L258 now considers the invocation delays to have passed and hence the non-preferred executor can immediately call processMessage() by front-running Bob and hence pocketing the reward of message.fee on L98.

File:

contracts / bridge / Bridge.

sol 293:

// Refund the processing fee 294:

if ( msg.

sender == refundTo ) { 295:

refundTo.

sendEther ( _message.

fee + refundAmount ); 296: } else { 297:

// If sender is another address, reward it and refund the rest 298: @---> msg.

sender.

sendEther ( _message.

fee ); 299:

refundTo.

sendEther ( refundAmount ); 300: } Similar behaviour where the paused time is ignored by the protocol can be witnessed in:

recallMessage() which similarly uses invocationDelay. However, no invocationExtraDelay is used there.

TimelockTokenPool.sol for:

// If non-zero, indicates the start time for the recipient to receive // tokens, subject to an unlocking schedule.

uint64 grantStart; // If non-zero, indicates the time after which the token to be received // will be actually non-zero uint64 grantCliff; // If non-zero, specifies the total seconds required for the recipient // to fully own all granted tokens.

uint32 grantPeriod; // If non-zero, indicates the start time for the recipient to unlock // tokens.

uint64 unlockStart; // If non-zero, indicates the time after which the unlock will be // actually non-zero uint64 unlockCliff; // If non-zero, specifies the total seconds required for the recipient // to fully unlock all owned tokens.

uint32 unlockPeriod; TaikoData.sol for:

// The max period in seconds that a blob can be reused for DA.

uint24 blobExpiry;

## Recommended Mitigation Steps

Introduce a new variable which keeps track of how much time has already been spent in the valid wait window before a pause happened. Also track the last unpause timestamp (similar to how it is done in pauseProving() and unpausing mechanisms). Also refer my other recommendation under the report titled:

“Incorrect calculations for cooldownWindow and provingWindow could cause a state transition to spend more than expected time in these windows”. That will help fix the issue without any further leaks.

dantaik (Taiko) confirmed and commented:

This is a valid bug report, fixing in

- https://github.com/taikoxyz/taiko-mono/pull/16612
TimelockTokenPool.sol will not have a similar fix as the risk is very managable. Blob caching/sharing is disabled, so no fix for it as well.

# [M-13] Taiko SGX Attestation - Improper validation in certchain decoding

- **Contest:** Taiko
- **Slug:** 2024-03-taiko
- **Finding ID:** M-13
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-taiko
- **Source snapshot:** competitions/2024-03-taiko/final_report.html

Submitted by zzebra83

- https://github.com/code-423n4/2024-03-taiko/blob/0d081a40e0b9637eddf8e760fabbecc250f23599/packages/protocol/contracts/automata-attestation/lib/PEMCertChainLib.sol#L135
- https://github.com/code-423n4/2024-03-taiko/blob/0d081a40e0b9637eddf8e760fabbecc250f23599/packages/protocol/contracts/verifiers/SgxVerifier.sol#L115-L136
As part of of its ZK proof setup, Taiko leverages SGX provers. it also enables remote SGX attestation and this is possible via leveraging code from Automata, which provides a modular attestation layer extending machine-level trust to Ethereum via the AutomataDcapV3Attestation repo, which is in scope of this audit.

Anyone with SGX hardware can register their instance to be an SGX prover in the Taiko Network via calling the registerInstance function in SgxVerifier.sol. This is why attestation is critical to prove the reliability and trustworthiness of the SGX prover.

The attestation process of SGX provers is a multi fold process, and starts with calling the verifyParsedQuote function in AutomataDcapV3Attestation.sol. One of the steps involves decoding the certchain provided by the SGX prover, as seen below:

// Step 4: Parse Quote CertChain IPEMCertChainLib.ECSha256Certificate[] memory parsedQuoteCerts; TCBInfoStruct.TCBInfo memory fetchedTcbInfo; { // 536k gas parsedQuoteCerts = new IPEMCertChainLib.ECSha256Certificate[](3); for (uint256 i; i < 3; ++i) { bool isPckCert = i == 0; // additional parsing for PCKCert bool certDecodedSuccessfully; // todo! move decodeCert offchain (certDecodedSuccessfully, parsedQuoteCerts[i]) = pemCertLib.decodeCert( authDataV3.certification.decodedCertDataArray[i], isPckCert ); if (!certDecodedSuccessfully) { return (false, retData); } after this step is executed, a number of other steps are done including:

Step 5: basic PCK and TCB check Step 6: Verify TCB Level Step 7: Verify cert chain for PCK Step 8: Verify the local attestation sig and qe report sig The decoding of the certchain happens through the EMCertChainLib lib, and this involves a number of steps, one of which is to validate the decoded notBefore and notAfter tags of the certificate:

{ uint256 notBeforePtr = der.firstChildOf(tbsPtr); uint256 notAfterPtr = der.nextSiblingOf(notBeforePtr); bytes1 notBeforeTag = der[notBeforePtr.ixs()]; bytes1 notAfterTag = der[notAfterPtr.ixs()]; if ( (notBeforeTag != 0x17 && notBeforeTag == 0x18) || (notAfterTag != 0x17 && notAfterTag != 0x18) ) { return (false, cert); } cert.notBefore = X509DateUtils.toTimestamp(der.bytesAt(notBeforePtr)); cert.notAfter = X509DateUtils.toTimestamp(der.bytesAt(notAfterPtr)); } These fields determine the time format, whether the notBeforePtr and notAfterPtr are in UTC or generalized time, and are used to ensure consistency in timestamps used to determine the validity period of the certificate.

However the validation can fail because the logic above is faulty, as it will allow the attestor to pass in any value for the notBefore tag, indeeed the condition of:

(notBeforeTag != 0x17 && notBeforeTag == 0x18) will allow the attestor to pass in any beforetag because the condition will always be false.

Consider if we pass an invalid tag of 0x10:

notBeforeTag != 0x17 is True.

notBeforeTag == 0x18 is False.

full condition is False.

I believe the original intention was to ensure the beforeTag is strictly 0x17 or 0x18, just as with the afterTag. Because of this oversight, a malicious attestor could pass in any notBefore Tag as part of their certificate.

This issue requires attention given the significance of the attestation process of SGX provers within Taiko’s ZK setup. The whole point of attestation is to prove the SGX provers are secure, untampered, and trustworthy, and improper validation related to certificate validity periods can have unforeseen consequences.

## Recommended Mitigation Steps

Update the condition as below:

if ( (notBeforeTag != 0x17 && notBeforeTag != 0x18) || (notAfterTag != 0x17 && notAfterTag != 0x18) ) { return (false, cert); smtmfft (Taiko) confirmed and commented:

I think this is a valid catch, already submitted a fix.

# [M-14] Malicious caller of processMessage() can pocket the fee while forcing excessivelySafeCall() to fail

- **Contest:** Taiko
- **Slug:** 2024-03-taiko
- **Finding ID:** M-14
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-taiko
- **Source snapshot:** competitions/2024-03-taiko/final_report.html

processMessage() can pocket the fee while forcing excessivelySafeCall() to fail Submitted by t0x1c, also found by Shield and ladboy233 The logic inside function processMessage() provides a reward to the msg.sender if they are not the refundTo address. However this reward or _message.fee is awarded even if the _invokeMessageCall() on L282 fails and the message goes into a RETRIABLE state. In the retriable state, it has to be called by someone again and the current msg.sender has no obligation to be the one to call it.

This logic can be gamed by a malicious user using the 63/64th rule specified in EIP-150.

File:

contracts / bridge / Bridge.

sol 278:

// Use the specified message gas limit if called by the owner, else 279:

// use remaining gas 280: @---> uint256 gasLimit = msg.

sender == _message.

destOwner ?

gasleft ():

_message.

gasLimit; 281:

282: @---> if ( _invokeMessageCall ( _message, msgHash, gasLimit )) { 283:

_updateMessageStatus ( msgHash, Status.

DONE ); 284: } else { 285: @---> _updateMessageStatus ( msgHash, Status.

RETRIABLE ); 286: } 287: } 288:

289:

// Determine the refund recipient 290:

address refundTo = 291:

_message.

refundTo == address ( 0 ) ?

_message.

destOwner:

_message.

refundTo; 292:

293:

// Refund the processing fee 294:

if ( msg.

sender == refundTo ) { 295:

refundTo.

sendEther ( _message.

fee + refundAmount ); 296: } else { 297:

// If sender is another address, reward it and refund the rest 298: @---> msg.

sender.

sendEther ( _message.

fee ); 299:

refundTo.

sendEther ( refundAmount ); 300: } Description The _invokeMessageCall() on L282 internally calls excessivelySafeCall() on L497. When excessivelySafeCall() makes an external call, only 63/64th of the gas is used by it. Thus the following scenario can happen:

Malicious user notices that L285-L307 uses approx 165_000 gas.

He also notices that L226-L280 uses approx 94_000 gas.

He calculates that he must provide approximately a minimum of 94000 + (64 * 165000) = 10_654_000 gas so that the function execution does not revert anywhere.

Meanwhile, a message owner has message which has a _message.gasLimit of 11_000_000. This is so because the receive() function of the contract receiving ether is expected to consume gas in this range due to its internal calls & transactions. The owner expects at least 10_800_000 of gas would be used up and hence has provided some extra buffer.

Note that any message that has a processing requirement of greater than 63 * 165000 = 10_395_000 gas can now become a target of the malicious user.

Malicious user now calls processMessage() with a specific gas figure. Let’s use an example figure of {gas: 10_897_060}. This means only 63/64 * (10897060 - 94000) = 10_634_262 is forwarded to excessivelySafeCall() and 1/64 * (10897060 - 94000) = 168_797 will be kept back which is enough for executing the remaining lines of code L285-L307. Note that since (10897060 - 94000) = 10_803_060 which is less than the message owner’s provided _message.gasLimit of 11_000_000, what actually gets considered is only 10_803_060.

The external call reverts inside receive() due to out of gas error (since 10_634_262 < 10_800_000) and hence _success is set to false on L44.

The remaining L285-L307 are executed and the malicious user receives his reward.

The message goes into RETRIABLE state now and someone will need to call retryMessage() later on.

A different bug report titled “No incentive for message non-owners to retryMessage()” has also been raised which highlights the incentivization scheme of retryMessage().

## Impact

Protocol can be gamed by a user to gain rewards while additionaly saving money by providing the least possible gas.

There is no incentive for any external user now to ever provide more than {gas: 10_897_060} (approx figure).

## Recommended Mitigation Steps

Reward the msg.sender (provided it’s a non-refundTo address) with _message.fee only if _invokeMessageCall() returns true. Additionally, it is advisable to release this withheld reward after a successful retryMessage() to that function’s caller.

dantaik (Taiko) confirmed and commented:

Fixed in

- https://github.com/taikoxyz/taiko-mono/pull/16613
I don’t think paying fees only when _invokeMessageCall returns true is a good idea as this will require the relayer to simulate all transactions without guaranteed reward.

## Rejected Primary Findings

# Rejected Primary Findings: Taiko

No rejected primary findings were captured for this contest in the current corpus.

- **Rejected primary count:** 0
- **Primary submission rows captured:** 0
- **Submission status:** requires_authentication
