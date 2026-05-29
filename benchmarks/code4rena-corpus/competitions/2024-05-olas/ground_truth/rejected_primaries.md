# Rejected Primary Findings: Olas

# Some of the L2s the protocol is deployed to, doesn't support PUSH0 opcode

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-93
- **Submitter:** 0xBugSlayer
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/93
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-93.md

## Brief Summary

Some of the L2s you are going to deploy to deploy the protocol, doesn't support the PUSH0 opcode

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# ArbitrumDepositProcessorL1.receieve() can't recieve ether from message sent from L2

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-133
- **Submitter:** 0xHarryBarz
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/133
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-133.md

## Brief Summary

The ArbitrumDepositProcessorL1.recievemessage() doesn't have the payable modifier, hence it can't receive ether, which will lead to funds stuck in the contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# staking batch nonce is not updated in the ArbitrumTargetDispenserL2.receive function according to the docs

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-134
- **Submitter:** 0xHarryBarz
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/134
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-134.md

## Brief Summary

Staking batch is not updated according to the docs

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# User’s Funds Would Stuck if the Message Claim Failed on the Destination Layer via GnosisDepositProcessorL1.sol

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-135
- **Submitter:** 0xHarryBarz
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/135
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-135.md

## Brief Summary

When claiming the message on the destination layer, if the message failed to execute with various reasons (e.g. wrong target contract address, wrong contract logic, out of gas, malicious contract), the Ether sent with sendMessage on the original layer will be stuck.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_28_group

# An attacker can claim the incentive belonging to owner of components due to poor access control in the tokenomics.claimOwnerIncentives

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-162
- **Submitter:** 0xHarryBarz
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/162
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-162.md

## Brief Summary

An attacker will claim the incentives belonging to an owner of components/agents due to poor access control.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_38_group

# Mismatched Array Lengths in Implementation Status Update Can Cause Misconfiguration

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-221
- **Submitter:** 0xRobsol
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/221
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-221.md

## Brief Summary

The function setImplementationsStatuses allows for setting whitelisting statuses for multiple implementations. It requires that the implementations and statuses arrays be of the same length, but if this requirement is not met, the function could either revert unnecessarily or misalign statuses with implementations. Impact A mismatch in array lengths can lead to legitimate implementations not being whitelisted due to missing status entries. This oversight could unintentionally block critical services from operating, impacting system functionality and user operations, especially if these implementations are essential for the system's core processes.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Potential Overflow in Rewards Calculation within _finalizeIncentivesForUnitId Function

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-232
- **Submitter:** 0xRobsol
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/232
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-232.md

## Brief Summary

The function performs multiple arithmetic operations on totalIncentives which can potentially exceed the uint96 data type's maximum limit, leading to integer overflow. This occurs particularly when calculating rewards based on system parameters that could escalate, such as totalTopUpsOLAS and topUpUnitFraction. Impact An overflow could result in erroneously low rewards being recorded and distributed, undermining the integrity of the incentive system and potentially causing financial discrepancies.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), :robot:_primary, :robot:_71_group

# Misordered Args in `sendTokenWithPayloadToEvm`

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-30
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/30
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-30.md

## Brief Summary

Incorrect order of arguments passed to the `sendTokenWithPayloadToEvm` function in the `_sendMessage` function can have significant implications for the functionality and security: [@>tokenomics/contracts/staking/WormholeDepositProcessorL1.sol#L96-L98](https://github.com/code-423n4/2024-05-olas/blob/e2a8bc31d2769bfb578a06cc64919ad369a82c08/tokenomics/contracts/staking/WormholeDepositProcessorL1.sol#L96-L98) The bug is related to the order of arguments passed to the `sendTokenWithPayloadToEvm` function. According to the Wormhole documentation and the source code, the correct order of arguments for `sendTokenWithPayloadToEvm` should be: https://github.com/wormhole-foundation/hello-token In th...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# WormholeDepositProcessorL1 Can Misinterpret sourceAddress, Leading to Potential Theft

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-31
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/31
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-31.md

## Brief Summary

The way the l2Dispenser address is derived from the `sourceAddress`. In the line: [@>tokenomics/contracts/staking/WormholeDepositProcessorL1.sol#L125](https://github.com/code-423n4/2024-05-olas/blob/e2a8bc31d2769bfb578a06cc64919ad369a82c08/tokenomics/contracts/staking/WormholeDepositProcessorL1.sol#L125) The `sourceAddress` is a `bytes32` value, which is directly converted to a `uint256` and then truncated to a `uint160` before being converted to an `address`. This approach assumes that the `sourceAddress` is always a valid Ethereum address encoded as a `bytes32`. If the `sourceAddress` is not a valid Ethereum address or if it contains additional data beyond the 20-byte address, this conver...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# `transferAmount` is nullified before distributing rewards

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-145
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/145
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-145.md

## Brief Summary

In the current implementation, `Dispenser` contract is used to assign `stakingIncentive` (that is formed by determined OLAS inflation amount per epoch) to a staking target that called `claimStakingIncentives()` function. The problem is that the user may not get any rewards at all as the variable can be nullified right away before distribution.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), edited-by-warden, :robot:_primary, :robot:_31_group

# `returnAmount` is inconsistently calculated when calculating incentives

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-155
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/155
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-155.md

## Brief Summary

The current implementation of `Dispenser` allows to return some amount back to `Tokenomics` if a nominee (staking target) hasn't met the minimum staking weight requirement. The problem is that the contract does it inconsistently making return amount greater than it should be in the case when the weight of the nominee is less than minimum staking weight.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), edited-by-warden, :robot:_primary, :robot:_04_group

# Approve on ERC-677 token is not called in `GnosisDepositProcessorL1`

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-164
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/164
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-164.md

## Brief Summary

In the current implementation of `GnosisDepositProcessorL1` `_sendMessage()` calls bridge function `relayTokensAndCall()` that is supposed to transfer the tokens to the L2TargetDispenser. However, the functionality deviates from the spec as it misses the approval for the ERC-677 token.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# `ArbitrumDepositProcessorL1` uses incorrect parameter for `createRetryableTicket()`

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-165
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/165
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-165.md

## Brief Summary

`ArbitrumDepositProcessorL1` is supposed to transfer the tokens from L1 to L2 using Arbitrum bridge services. However, due to mistake in `createRetryableTicket()` function, the submitted parameters can be incorrect and the transfer of funds can be blocked.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# The contract `OptimismTargetDispenserL2` uses incorrect `l1Processor`

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-166
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/166
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-166.md

## Brief Summary

`OptimismTargetDispenserL2` contract is used by te protocol to receive the messages and tokens from L1 deposit processors contracts and then complete a verification process of staking targets and send the funds to them. However, the contract uses incorrect `l1Processor` address when receiving the message.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Applying the Checks-Effects-Interactions pattern to the functions unstake and _claim functions

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-125
- **Submitter:** AerialRaider
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/125
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-125.md

## Brief Summary

By applying the Checks-Effects-Interactions pattern to the functions unstake, _claim, we ensure that all state changes are made before any external calls, mitigating the risk of reentrancy attacks.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_17_group

# looping over potentially unbounded arrays in functions _calculateStakingRewards and _evict

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-126
- **Submitter:** AerialRaider
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/126
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-126.md

## Brief Summary

To mitigate the issue of looping over potentially unbounded arrays, we need to ensure that loops in functions _calculateStakingRewards and _evict have a reasonable maximum iteration count. If necessary, break down large loops into multiple transactions.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Ensuring that serviceAgentIds in _checkTokenStakingDeposit function has a reasonable size limit to prevent gas limit issues

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-131
- **Submitter:** AerialRaider
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/131
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-131.md

## Brief Summary

1. Ensure serviceAgentIds in _checkTokenStakingDeposit has a reasonable size limit to prevent gas limit issues. 2. Suggesting using OpenZeppelin's ReentrancyGuard to protect the deposit and _withdraw functions.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Zero Value Checks: Added checks to ensure values are not zero where not expected in the _processData and redeem functions

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-138
- **Submitter:** AerialRaider
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/138
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-138.md

## Brief Summary

Suggest adding checks to ensure that addresses passed as parameters are not zero addresses in the _processData and redeem functions of the to enhance the security and robustness of the DefaultTargetDispenserL2 contract. // Check for zero address if (target == address(0)) { revert ZeroAddress(); }

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_47_group

# Reentrancy vulnerabilities can lead to attackers manipulating the contract state

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-96
- **Submitter:** Ali55
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/96
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-96.md

## Brief Summary

Reentrancy vulnerabilities can lead to attackers manipulating the contract state, which, in this case, may allow them to change the voting power of specific users maliciously.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), :robot:_primary, :robot:_62_group

# integer overflows in the _addToChanges

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-97
- **Submitter:** Ali55
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/97
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-97.md

## Brief Summary

In the `VoteWeighting` contract, I found a potential issue related to integer overflows in the `_addToChanges` function. The function calculates the new weight by adding the provided slope, but it does not validate whether the result will cause an overflow. Here's the relevant part of the code: This function can cause an integer overflow if `changesWeight[self][_time]` or `changesSum[_time]` plus the provided slope is bigger than the maximum value for `uint256`. Overflow can lead to unpredictable behavior in the contract, potentially allowing attackers to manipulate the system. To mitigate this issue, you should add checks to prevent overflows. You can use the `SafeMath` library from OpenZe...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Unchecked Return Value in getNomineeWeight Function

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-98
- **Submitter:** Ali55
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/98
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-98.md

## Brief Summary

Title: Unchecked Return Value in `getNomineeWeight` Function Issue: The `getNomineeWeight` function in the VoteWeighting contract returns a `uint256` value without checking if it is zero. Returning zero weights can lead to potential issues in the dependent systems that rely on this contract. Impact: Returning zero weights can cause unintended behavior in the dependent systems that rely on this contract. It may lead to incorrect calculations, improper voting power distribution, or other issues in the dependent systems.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), :robot:_primary, :robot:_46_group

# if the curEpochLen set to the MAX_EPOCH_LENGTH the checkpoint always fail

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-247
- **Submitter:** ArsenLupin
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/247
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-247.md

## Brief Summary

Assume, `curEpochLen` = MAX_EPOCH_LENGTH, to be precise it is 31449600 seconds. When the `checkpoint()` is invoked, it takes `diffNumSeconds`. The `diffNumSeconds` return the delta of the end of last epoch with the current timestamp. After that, the `currentEpochLen` is taken. Then, the function makes a crucial checks, which determines whether we could proceed or not. 1. If the `diffNumSeconds` is lower than `curEpochLen`, it return false. 2. Also, if the `diffNumSeconds` is more than `MAX_EPOCH_LENGTH`, it return false. Normal user or malicious one could set the `curEpochLen` to the `MAX_EPOCH_LENGTH`. And it means that he must call the `checkpoint` function at the exact second in the time...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# StakingBase: initialize function is frontrunnable

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-274
- **Submitter:** ArsenLupin
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/274
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-274.md

## Brief Summary

The Olas flow of Staking contract creation is as follows: 1. `Staking blanc implementation address` is created (so it could be passed into the factory). Additionally, the `initPayload` is encoded so, the Staking blanc implementation could be initialised during the creation in the factory. 2. In the StakingFactory.sol both this params are passed into the `createStakingInstance`, which creates the instance via create2 However, the problem is that implementation contract left uninitialised. It is initialised only after it manually passed into the `createStakingInstance` function where it is initialised via the newly created proxy The problem is that, during the time-frame when the implementati...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_02_group

# The staking token verification is restricted only to the Olas token, while in the reality it could be any ERC20 token

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-276
- **Submitter:** ArsenLupin
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/276
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-276.md

## Brief Summary

According to the devs, the staking token could be any whitelisted token, however when the staking contract is created, it goes through the verification, where there is a check that state that if token ≠ Olas Token → return false, which is incorrect since the staking token could be any token apart from olas

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_33_group

# DefaultTargetDispenserL2::_processData does check if dispender is paused

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-40
- **Submitter:** BiasedMerc
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/40
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-40.md

## Brief Summary

`DefaultTargetDispenserL2::_processData` contains no check in the process data function flow to ensure that the contract is no paused. This means that the pausing functionality of the contract is not working as intended.

## Rejection Reason

GitHub validation labels: bug, invalid, 2 (Med Risk), insufficient quality report, withdrawn by warden, edited-by-warden, :robot:_primary

# Dispenser::calculateStakingIncentives doesn't normalise decimals correctly

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-63
- **Submitter:** BiasedMerc
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/63
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-63.md

## Brief Summary

Detailed description of the impact of this finding.

## Rejection Reason

GitHub validation labels: bug, invalid, 2 (Med Risk), insufficient quality report, withdrawn by warden, :robot:_primary, :robot:_159_group

# Tokenomics::changeIncentiveFractions() incorrectly checks sum of _rewardComponentFraction and _rewardAgentFraction

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-81
- **Submitter:** BiasedMerc
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/81
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-81.md

## Brief Summary

Detailed description of the impact of this finding.

## Rejection Reason

GitHub validation labels: bug, invalid, 2 (Med Risk), insufficient quality report, withdrawn by warden, edited-by-warden, :robot:_primary

# Incorrect Verification of Multisig Proxy Bytecode Hash

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-160
- **Submitter:** Centaur
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/160
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-160.md

## Brief Summary

The current implementation for verifying that the [multisig address](https://github.com/code-423n4/2024-05-olas/blob/3ce502ec8b475885b90668e617f3983cea3ae29f/registries/contracts/staking/StakingBase.sol#L760) corresponds to the authorized multisig proxy bytecode hash is incorrect. The method used to retrieve the bytecode hash is invalid, which can lead to bypassing security checks and potentially allowing unauthorized multisig contracts.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Owner can drain tokens when the contract is in paused state

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-220
- **Submitter:** Centaur
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/220
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-220.md

## Brief Summary

The DefaulTargetDispenser.sol in question includes a drain function designed to allow the contract owner to withdraw all Ether from the contract. However, there is no check within this function to determine whether the contract is in a paused state. This omission allows the drain function to be called even when the contract is paused, potentially leading to significant security and operational risks.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_15_group

# #DefualtDeposit revert in when approve is insufficient and after deposit into the BridgeRelayer

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-89
- **Submitter:** Centaur
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/89
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-89.md

## Brief Summary

In the DefaultDepositProcessor.sol looking at the child contract - OptimismDepositProcessorL1 default is the default one as it the abstract regardless of the bridge contract helps in sending of message to the L2 target set. However, in the in the OpitmismDefaultDepositProcessor, case of revert may likely happen and any revert like this can block the L1<->L2 communication from processing to the L2 target contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# In WormholeDepositProcessorL1 and WormholeTargetDispenserL2, BRIDGE_PAYLOAD_LENGTH should be 52, not 64

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-269
- **Submitter:** Emmanuel
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/269
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-269.md

## Brief Summary

`_sendMessage` within WormholeTargetDispenserL2 and WormholeDepositProcessorL1 will revert, preventing users from claiming staking incentives

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_65_group

# Unsafe Downcasting to uint96

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-121
- **Submitter:** Hajime
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/121
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-121.md

## Brief Summary

Downcasting to uint96 can cause to lost staking incentive.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# no check for `tp.epochPoint.rewardTreasuryFraction` to be equal to zero

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-147
- **Submitter:** Hajime
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/147
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-147.md

## Brief Summary

When calculating `tp.epochPoint.rewardTreasuryFraction`, it is assumed that `rewardComponentFraction + _rewardAgentFraction + tp.epochPoint.rewardTreasuryFraction = 100 `. However, there is a chance that after the calculation `tp.epochPoint.rewardTreasuryFraction` will be zero . This may cause the value of `incentives[1]` to be zeroed.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# unsafe casting IDF without check

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-150
- **Submitter:** Hajime
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/150
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-150.md

## Brief Summary

`uint256 idf` downcasting to `uint64` without check. This may cause some values to be lost

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Risk of re-org attacks

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-16
- **Submitter:** Hajime
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/16
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-16.md

## Brief Summary

The `createStakingInstance()` function creates a service staking contract instance using the `create2`, where the `salt` includes nonce and block.chainId?, but not `msg.sender`. This is increase risk of reorgs attacks. Re-orgs can happen in all EVM chains and as confirmed the contracts will be deployed on most EVM compatible L2s including Arbitrum, etc. Attacker is able to frontrun the service staking contract instance deployment

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_29_group

# unsafe casting `l2TargetChainId` to uint16

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-178
- **Submitter:** Hajime
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/178
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-178.md

## Brief Summary

In the constructors of `WormholeDepositProcessorL1.sol` and `DepositProcessorL1.sol`, there is no explicit check for `l2TargetChainId` exceeding type(uint16).max. This could potentially lead to unexpected behavior if `l2TargetChainId` exceeds this value, as the cast to uint16 would truncate the higher bits of the uint256 value, leading to an incorrect chain ID being used in the message sent to L2. At the same time, the `wormholeTargetChainId` before casting is subjected to the necessary checks.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Reentrancy vulnerability in StakingNativeToken

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-267
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/267
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-267.md

## Brief Summary

In the _withdraw function, state variables are updated before the call function which sends ETH to an arbitrary address. This is referred to as a reentrancy vulnerability as a malicious contract could reenter the contract before the first invocation of the function was finished, potentially draining funds from the contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_32_group

# Several interfaces used in contracts in scope have not been implemented anywhere, and essentially have empty functions that are being called in those contracts.

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-85
- **Submitter:** Katrix
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/85
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-85.md

## Brief Summary

Several interfaces in `tokenomics/contracts/interfaces/` have not been implemented in any contract. Contracts in `Dispenser.sol`, `Tokenomics.sol` and a few other contracts out of scope import these interfaces and use the defined functions but the scope of those functions has not been defined anywhere and are thus, empty functions or simply dead code that is being used. Additionally, even if the functions have been implemented, the calls will not work as those interfaces have not been inherited by the contracts using them. This is a basic concept of Solidity and several other programming languages. One cannot use those interfaces if the contract isn't inheriting/extending them with the `is`...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# VoteWeighting._getWeight Loop Can Cause Excessive Gas Costs

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-48
- **Submitter:** LinKenji
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/48
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-48.md

## Brief Summary

In the current implementation, the loop in the `_getWeight` function is defined as follows: [#L267-L270](https://github.com/code-423n4/2024-05-olas/blob/e2a8bc31d2769bfb578a06cc64919ad369a82c08/governance/contracts/VoteWeighting.sol#L267-L270) The loop iterates up to `MAX_NUM_WEEKS` times, regardless of the value of `t` in relation to the current block timestamp. The issue arises when `t` exceeds `block.timestamp` during the loop iterations. Consider the following scenario. - `MAX_NUM_WEEKS` is set to a large value, such as 52 (representing one year). - The nominee's weight has not been updated for a long time, resulting in t being significantly larger than block.timestamp. In this scenario...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_06_group

# VoteWeighting.sol - Nominee Struct Leads to Data Loss and Misinterpretation.

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-51
- **Submitter:** LinKenji
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/51
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-51.md

## Brief Summary

The `Nominee` struct is defined as follows: [VoteWeighting#L121-L124](https://github.com/code-423n4/2024-05-olas/blob/e2a8bc31d2769bfb578a06cc64919ad369a82c08/governance/contracts/VoteWeighting.sol#L121-L124) The intention is to pack both the nominee's address (a 20-byte value) and the chain ID into a single `bytes32` value, which is stored in the `data` field of the `Nominee` struct. However, the way this packing is done in the `VoteWeighting:addNomineeEVM` function is incorrect: [#L340](https://github.com/code-423n4/2024-05-olas/blob/e2a8bc31d2769bfb578a06cc64919ad369a82c08/governance/contracts/VoteWeighting.sol#L340) Here's what's happening: `uint160(account)` truncates the `account` add...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Inconsistent ChainId Validation in VoteWeighting Contract Can Lead to Unforeseen Behavior.

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-52
- **Submitter:** LinKenji
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/52
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-52.md

## Brief Summary

The `addNomineeNonEVM` function responsible for adding a new non-EVM nominee address along with the associated chain ID. The function includes a check to prevent underflow conditions for the `chainId` parameter. However, the condition is implemented incorrectly, leading to the opposite behavior of what was intended. Here's the relevant

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# VoteWeighting._nomineeRelativeWeight Can Revert Due to Division by Zero

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-53
- **Submitter:** LinKenji
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/53
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-53.md

## Brief Summary

If the `totalSum` variable is zero, the line `weight = 1e18 * nomineeWeight / totalSum;` will result in a division by zero, which is not allowed in Solidity. This will cause the transaction to revert, and any state changes or operations performed before the revert will also be reverted. The code is: [_nomineeRelativeWeight#L432](https://github.com/code-423n4/2024-05-olas/blob/e2a8bc31d2769bfb578a06cc64919ad369a82c08/governance/contracts/VoteWeighting.sol#L432) This line is executed within the `_nomineeRelativeWeight` function, which is defined as. If the `_nomineeRelativeWeight` function is called in a critical path of the contract's execution, the division by zero error can lead to unexpec...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# getNominee/getRemovedNominee Can Cause Out-of-Bounds Access.

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-57
- **Submitter:** LinKenji
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/57
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-57.md

## Brief Summary

In the [VoteWeighting:getNominee](https://github.com/code-423n4/2024-05-olas/blob/e2a8bc31d2769bfb578a06cc64919ad369a82c08/governance/contracts/VoteWeighting.sol#L744-L755) and [VoteWeighting:getRemovedNominee](https://github.com/code-423n4/2024-05-olas/blob/e2a8bc31d2769bfb578a06cc64919ad369a82c08/governance/contracts/VoteWeighting.sol#L761-L772) functions, the functions handle the case when the `id` parameter is equal to the length of the respective array (`setNominees` or `setRemovedNominees`). In both functions, the following condition is used to check for overflow: [#L750-L752](https://github.com/code-423n4/2024-05-olas/blob/e2a8bc31d2769bfb578a06cc64919ad369a82c08/governance/contracts...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# StakingBase Contract - Potential for Iteration Over Empty Array in evictServices.

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-60
- **Submitter:** LinKenji
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/60
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-60.md

## Brief Summary

The `for` loop that iterates over the `evictServiceIds` array. The loop uses the length property of the array to determine how many elements to iterate over, but it does not check if the length of the array is greater than zero before doing so.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_13_group

# Missing Nonce Increment in _claim Function

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-61
- **Submitter:** LinKenji
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/61
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-61.md

## Brief Summary

In the `_claim` function, the code does not update the `sInfo.nonces` value after claiming the reward. The `nonces` field is likely used to track the number of times rewards have been claimed for a service. By not incrementing it after a successful claim, it may lead to inconsistencies or incorrect tracking of the claiming process. Referenced Code: [#StakingBase.sol:507-511](https://github.com/code-423n4/2024-05-olas/blob/e2a8bc31d2769bfb578a06cc64919ad369a82c08/registries/contracts/staking/StakingBase.sol#L507-L511) We can see, the `sInfo.nonces` value is not incremented after the `_withdraw` function call, which transfers the rewards to the service's multisig address. This missing increme...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# unsafe approve would stop bridging of tokens

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-191
- **Submitter:** MSaptarshi
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/191
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-191.md

## Brief Summary

Failure to bridge tokens to other contract of L2 from L1

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), edited-by-warden, :robot:_primary

# Use of Wrong Operator in Unstaking Condition

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-14
- **Submitter:** Rhaydden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/14
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-14.md

## Brief Summary

The incorrect logic in the `unstake` function's condition may prevent services from unstaking even when they have met the minimum staking duration requirement. This can lead to services being locked in the staking contract unintentionally.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_35_group

# Missing check for equal length arrays in `EthereumDepositProcessor`

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-286
- **Submitter:** Sparrow
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/286
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-286.md

## Brief Summary

The finding concerns the lack of input validation in the `_deposit` function of the `EthereumDepositProcessor` contract. Specifically, the function does not validate that the lengths of the `targets` and `stakingIncentives` arrays are equal before processing them. This omission could lead to undefined behavior, such as array index out-of-bounds errors, which could potentially crash the contract or lead to unintended operations. Impact - Contract Failure: If the lengths of `targets` and `stakingIncentives` arrays are different, accessing `stakingIncentives[i]` where `i` exceeds the length of `stakingIncentives` could cause the contract to revert due to an array index out-of-bounds error. - U...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Anyone can Call the Tokenomics Checkpoint Function

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-179
- **Submitter:** Topmark
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/179
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-179.md

## Brief Summary

Anyone can Call the Tokenomics Checkpoint Function due to absence of necessary validation to ensure only authorized address or owner is allowed to interact with the function.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Mismatch due State Update in reward Claim

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-278
- **Submitter:** XDZIBECX
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/278
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-278.md

## Brief Summary

The function _claimis calculates rewards for users there is an issue occurs in the function. When checkpoint is executed, it updates the state, potentially setting the reward to zero or an incorrect value before the user can claim it. the bug is occur because checkpoint updates the reward-related state variables, which can cause the _claim function to fetch an outdated or zero reward value. Impact Users might get zero rewards or incorrect amounts, leading to financial loss and decreased trust in the system. - here is a scenario that show the bug let's say we have : - sInfo.reward before checkpoint: 1000 tokens. - availableRewards before checkpoint: 5000 tokens. - execCheckPoint is set to tr...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), :robot:_primary

# overestimation of available rewards

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-281
- **Submitter:** XDZIBECX
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/281
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-281.md

## Brief Summary

The flaw in its receive function, is where any funds sent to the contract are directly added to the availableRewards balance. and This can lead to an overestimation of available rewards, and causing the contract to distribute more rewards than intended. this is a bug that is arises from the incorrect assumption that all incoming funds are meant to be rewards, which might not always be the case. This flaw can affect the contract's financial stability and reward distribution logic. Impact This can lead to an overestimation of available rewards, which can cause the contract to distribute more rewards than intend root of the bug : The receive function updates both the balance and availableRewar...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Contracts that lock Ether

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-149
- **Submitter:** a1exweb
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/149
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-149.md

## Brief Summary

Contract with a payable function, but without a withdrawal capacity. Every Ether sent to the `StakingProxy` contract will be lost.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Exploitable Hash Collision Vulnerability in Retainer Account Check

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-193
- **Submitter:** aariiif
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/193
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-193.md

## Brief Summary

The purpose of this check seems to be preventing the removal of the retainer account. However, the check is comparing the `retainerHash` with the `nomineeHash`. If the `nomineeHash` happens to match the `retainerHash`, the function will revert with the `WrongAccount` error, even if the `nomineeHash` does not correspond to the actual retainer address.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Potential Overflow Risk in syncWithheldAmount Function

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-198
- **Submitter:** aariiif
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/198
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-198.md

## Brief Summary

In this line of the syncWithheldAmount function: The problem is that the addition operation `mapChainIdWithheldAmounts[chainId] + amount` can overflow if the sum exceeds the maximum value that can be stored in a `uint256`. If an overflow occurs, the resulting `withheldAmount` will wrap around and have an incorrect value.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Normalization Bug in `syncWithheldAmountMaintenance` Function Leads to Inaccurate Amount Syncing.

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-199
- **Submitter:** aariiif
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/199
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-199.md

## Brief Summary

The normalization of the `amount` when the `bridgingDecimals` is less than 18 in `syncWithheldAmountMaintenance` function: [tokenomics/contracts/Dispenser.sol#L1220-L1225](https://github.com/code-423n4/2024-05-olas/blob/e2a8bc31d2769bfb578a06cc64919ad369a82c08/tokenomics/contracts/Dispenser.sol#L1220-L1225) The issue is that the normalization process is incorrect. The current code divides `amount` by `10 ** (18 - bridgingDecimals)` and then multiplies the result by the same factor. This effectively cancels out the division and leaves `amount` unchanged. 1. Impact on Withheld Amounts: - The purpose of the `syncWithheldAmountMaintenance` function is to manually sync the withheld amount from L...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Failure to Update Contract State on Invalid Parameter Inputs in `changeTokenomicsParameters` Function.

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-200
- **Submitter:** aariiif
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/200
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-200.md

## Brief Summary

When an invalid value is passed for `_devsPerCapital`, `_codePerDev`, `_epsilonRate`, `_epochLen`, or `_veOLASThreshold`, the function assigns the corresponding parameter to its existing value instead of the invalid value. However, this assignment is done using the input parameter name, which doesn't update the contract state variable. For example, if an invalid value is passed for `_devsPerCapital`, the function executes the following line: [tokenomics/contracts/Dispenser.sol#L656](https://github.com/code-423n4/2024-05-olas/blob/e2a8bc31d2769bfb578a06cc64919ad369a82c08/tokenomics/contracts/Tokenomics.sol#L656) This line assigns the current value of `devsPerCapital` to the input parameter `...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Miscalculation of rewardTreasuryFraction Due to Unenforced Sum Constraint

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-201
- **Submitter:** aariiif
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/201
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-201.md

## Brief Summary

tokenomics/contracts/Dispenser.sol#changeIncentiveFractions function assumes that the sum of `_rewardComponentFraction` and `_rewardAgentFraction` is always less than or equal to 100. Yet it doesn't enforce this condition properly. If the sum of these fractions is less than 100, the `rewardTreasuryFraction` will be calculated incorrectly. Here's the line: [tokenomics/contracts/Dispenser.sol#L735](https://github.com/code-423n4/2024-05-olas/blob/e2a8bc31d2769bfb578a06cc64919ad369a82c08/tokenomics/contracts/Tokenomics.sol#L735) If `_rewardComponentFraction + _rewardAgentFraction` is less than 100, the `rewardTreasuryFraction` will be set to a value greater than the actual leftover fraction.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Overflow Vulnerability in refundFromBondProgram Due to Addition Check Order.

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-204
- **Submitter:** aariiif
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/204
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-204.md

## Brief Summary

In the `refundFromBondProgram`, Currently the code performs the addition first and then checks if the result exceeds` type(uint96).max`: [tokenomics/contracts/Dispenser.sol#L814-L820](https://github.com/code-423n4/2024-05-olas/blob/e2a8bc31d2769bfb578a06cc64919ad369a82c08/tokenomics/contracts/Tokenomics.sol#L814-L820) This approach allows for an overflow to occur before the check is performed. If `effectiveBond` is already close to `type(uint96).max` and `amount` is large enough, the addition can overflow, resulting in an incorrect value for `eBond`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_49_group

# Zero Incentive Calculation Issue in _trackServiceDonations Function.

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-205
- **Submitter:** aariiif
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/205
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-205.md

## Brief Summary

The `amount` variable inside the nested loop over component and agent IDs has a problem.: [tokenomics/contracts/Dispenser.sol#L928](https://github.com/code-423n4/2024-05-olas/blob/e2a8bc31d2769bfb578a06cc64919ad369a82c08/tokenomics/contracts/Tokenomics.sol#L928) `amounts[i]` value is divided by `numServiceUnits` to calculate the amount for each unit. However, if `numServiceUnits` is greater than `amounts[i]`, the result of the division will be zero due to integer division. This means that if the donation amount for a service is less than the number of units in that service, the amount variable will be set to zero, and no incentives will be recorded for the units in that service.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Array Length Mismatch Vulnerability in trackServiceDonations Function.

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-206
- **Submitter:** aariiif
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/206
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-206.md

## Brief Summary

`trackServiceDonations` function checks if the service IDs exist in the `IServiceRegistry` contract, but it does not validate the length of the `amounts` array against the length of the `serviceIds` array. If the amounts array has a different length than the `serviceIds` array, it could lead to unexpected behavior or out-of-bounds access when calling the `_trackServiceDonations` function.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Incorrect Year Calculation in TokenomicsConstants.sol possibility

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-75
- **Submitter:** arabgodx
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/75
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-75.md

## Brief Summary

Vulnerability Report: Incorrect Year Calculation in TokenomicsConstants.sol Summary The `TokenomicsConstants.sol` contract does not correctly account for leap years in its time calculations. This leads to potential inaccuracies in the calculation of supply caps and inflation amounts over time, which could have significant effects on tokenomics projections. It affects 2024-05-olas/tokenomics/contracts /Tokenomics.sol read further Vulnerability Details **File**: [TokenomicsConstants.sol](https://github.com/code-423n4/2024-05-olas/blob/e2a8bc31d2769bfb578a06cc64919ad369a82c08/tokenomics/contracts/TokenomicsConstants.sol#L15) **Lines of Code**: - Definition of `ONE_YEAR`: - Year calculation log...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Dangerous Race Condition and Bad Practice in Array Handling VoteWeighting.sol

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-76
- **Submitter:** arabgodx
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/76
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-76.md

## Brief Summary

Vulnerability Details: Dangerous Race Condition and Bad Practice in Array Handling **Vulnerability Type:** - Dangerous Race Condition - Bad Practice in Array Handling **Location in Code:** - [VoteWeighting.sol#L621-L627](https://github.com/code-423n4/2024-05-olas/blob/e2a8bc31d2769bfb578a06cc64919ad369a82c08/governance/contracts/VoteWeighting.sol#L621-L627) **Description:** The current implementation of removing the last element from the `setNominees` array and shuffling the elements can lead to dangerous race conditions and is considered a bad practice. This is because arrays are class properties and can be accessed and modified concurrently, leading to potential inconsistencies and unexpe...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Use of O(n) implementation may cause out-of-gas issues

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-167
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/167
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-167.md

## Brief Summary

The `calculateStakingLastReward` function currently uses an array to iterate over `eligibleServiceIds`, potentially resulting in O(n) complexity where n is the number of services. As n grows, the function's gas consumption increases linearly, which can lead to high transaction costs and even out-of-gas errors for large datasets. This inefficiency limits scalability and performance.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Address Derivation Flow

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-168
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/168
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-168.md

## Brief Summary

The `getProxyAddressWithNonce` function currently utilizes a hardcoded prefix 0xff for address derivation, assuming Ethereum-like address conventions. This approach is vulnerable when deployed across multiple chains with differing address formats. Specifically: Typical address aliasing in chains like Arbitrum or Optimism might break the assumption of having the same prefix for different sub-accounts. Specifically, whenever an account is a contract and not an externally owned account (EOA), Layer 1 to Layer 2 (L1 -> L2) messages will have the msg.sender aliased with a mask. This might result in different aliased account prefixes. Chain-Specific Vulnerabilities: Celo: Incorrect address format...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing disableInitializers in OpenZeppelin Initializable Contract

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-169
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/169
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-169.md

## Brief Summary

The smart contract utilizes the OpenZeppelin initializer pattern but lacks the disableInitializers call in its constructor. This omission can lead to potential security vulnerabilities and unexpected behavior during contract deployment and initialization. OpenZeppelin's initializer pattern ensures that certain initialization functions (initializer) are only executed once when a contract is deployed. This prevents reinitialization and potential security risks associated with multiple initialization attempts. The disableInitializers function, provided by OpenZeppelin, is crucial as it disables the initializer function after the contract's deployment phase is complete.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Corruptible Upgradability Pattern for Market Contracts

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-170
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/170
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-170.md

## Brief Summary

The StakingToken contract is upgradeable, but gaps slots are not defined. The storage might be corrupted during upgrades. when dealing with upgradeable contracts using proxies, it's crucial to manage storage layout carefully. When upgrading a contract, the new implementation contract must align its storage layout with the original contract's layout to prevent storage conflicts and corruption. This is typically managed by introducing gap slots (unused variables) in the contract's storage structure.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_18_group

# Nested for loop could cause out of gas errors

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-171
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/171
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-171.md

## Brief Summary

The _calculateStakingIncentivesBatch function employs nested loops to compute staking incentives across multiple chains and targets. This approach, while functional, poses a significant risk of exceeding Ethereum's gas limits, potentially leading to transaction failures or out-of-gas errors during contract execution. Nested loops increase computational complexity, causing higher gas consumption per iteration. This can quickly deplete the gas limit allocated for Ethereum transactions, resulting in transaction failures. If the gas limit is exceeded during function execution, the transaction reverts entirely, causing wasted gas fees and failed transactions for users interacting with the contra...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_20_group

# Invalid Token Validation

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-172
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/172
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-172.md

## Brief Summary

The current implementation of token balance validation in the `claimStakingIncentives` function is susceptible to front-running attacks. This vulnerability allows malicious actors to manipulate the transaction ordering, potentially causing legitimate transactions to revert unexpectedly. By exploiting this vulnerability, attackers can disrupt the intended functionality of the contract, leading to financial losses or denial of service.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential Off-by-One Error in Time Comparison Using >= with block.timestamp

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-173
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/173
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-173.md

## Brief Summary

Using the >= operator for time comparisons against block.timestamp can introduce off-by-one errors due to the nature of how block.timestamp is updated only once per block. This can lead to unexpected behavior if the condition is met at the exact second when block.timestamp changes. This issue is especially critical in scenarios where time-sensitive operations are performed, potentially causing operations to revert unexpectedly or execute when they shouldn't.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential Denial of Service (DoS) Vulnerability Due to unsafe use of `staticcall`

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-189
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/189
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-189.md

## Brief Summary

The use of staticcall in smart contracts, particularly when interacting with other contracts, poses a significant security risk. According to the solidity docs, if a staticcall encounters a state change, it burns up all gas and returns. If the called contract contains state-changing operations, it could potentially consume all available gas during the execution of the staticcall, leading to a Denial of Service (DoS) condition. The `_checkRatioPass` function in the contract contains a vulnerability that could potentially lead to denial-of-service (DoS) attacks or out-of-gas errors. This vulnerability arises due to the use of staticcall without limiting the amount of gas forwarded, which coul...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_00_group

# No access control in "initializeTokenomics" in Tokenomics.sol.

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-248
- **Submitter:** bareli
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/248
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-248.md

## Brief Summary

Detailed description of the impact of this finding. In initializeTokenomics function there is no access control as anyone calls and inilize this function.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), :robot:_primary

# use "safetransfer" instead of transfer

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-280
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/280
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-280.md

## Brief Summary

Detailed description of the impact of this finding. okens not compliant with the ERC20 specification could return false from the transfer function call to indicate the transfer fails, while the calling contract would not notice the failure if the return value is not checked. Checking the return value is a requirement, as written in the EIP-20 specification:

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_07_group

# no check on return value of approve

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-282
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/282
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-282.md

## Brief Summary

Detailed description of the impact of this finding. Here there is no check for the approve function.THere should be some check on that.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Potential for error when checking for nominee existence or removal

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-116
- **Submitter:** cheatc0d3
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/116
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-116.md

## Brief Summary

The current condition for checking if a nominee already exists or has been removed uses a logical AND (`&&`) operator. This logic fails to accurately determine the presence of a nominee in the active or removed lists, potentially allowing duplicate entries or blocking valid ones. This issue can lead to inconsistencies in the contract's state and mismanagement of nominees, which can affect the integrity and reliability of the voting system.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Inadequate Return Data Length Check in _checkRatioPass Function Compromises Service Activity Verification and Staking Integrity

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-210
- **Submitter:** cheatc0d3
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/210
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-210.md

## Brief Summary

The `_checkRatioPass` function is critical for maintaining the integrity and security of a staking mechanism by verifying the activity of a service through nonce comparison over time. This function checks the liveness of a multisig address by comparing the current nonces with the last recorded nonces over a given time span (`ts`). The current implementation uses the condition `returnData.length > 63`, which is incorrect and potentially harmful. This check is supposed to ensure that the returned data contains at least two nonces, which is necessary for a meaningful comparison. However, the condition `returnData.length > 63` is not sufficient to guarantee this. Impact - **Security and Integri...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), :robot:_primary, :robot:_63_group

# Incorrect Nominee and Total Weight Updates Due to Silent Underflows, Leading to Misrepresented Voting Power and Governance Manipulation

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-211
- **Submitter:** cheatc0d3
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/211
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-211.md

## Brief Summary

The current implementation of the `_maxAndSub` function in the `VoteWeighting` contract can lead to incorrect updates of nominee weights and the total weight sum. Specifically, when the `oldBias` value is greater than the sum of the current weight (`_getWeight(account, chainId)`) and the new bias (`newBias`), or greater than the sum of the current total weight (`_getSum()`) and the new bias, the `_maxAndSub` function silently returns 0. This behavior can result in the following impacts: 1. **Inaccurate Weight History**: Incorrect historical data for nominee weights, making it difficult to track and audit past voting behavior accurately. 2. **Misrepresentation of Voting Power**: Nominees who...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), :robot:_primary

# Cross-Domain Transaction Failures and Irrecoverable Fund Losses to Depositors Due to Unsynchronized Pausing Mechanism

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-213
- **Submitter:** cheatc0d3
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/213
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-213.md

## Brief Summary

Transactions initiated on Layer 1 to Layer 2 using the `OptimismDepositProcessorL1` contract will fail and will not be replayable when the `DefaultTargetDispenserL2` (and by extension `OptimismTargetDispenserL2`) contract is paused. This is due to the paused state preventing proper processing of messages on L2. Vulnerability Detail The `OptimismDepositProcessorL1` and `DefaultTargetDispenserL2` contracts interact in a cross-domain manner. The `DefaultTargetDispenserL2` contract has a pausing mechanism controlled by the `paused` state variable. When the `DefaultTargetDispenserL2` contract is paused, the functions that process messages and deposit tokens will not execute correctly. Specifical...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), edited-by-warden, :robot:_primary

# Delegatecall to Destructed Contract in StakingProxy Fallback Function Leads to Unexpected Behavior

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-261
- **Submitter:** cheatc0d3
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/261
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-261.md

## Brief Summary

The use of delegatecall to a destructed or non-existent contract can lead to severe consequences and potential security vulnerabilities in the StakingProxy contract. If the implementation contract has been destructed, the proxy contract will still execute the delegatecall without any error, resulting in unexpected behavior and potential exploitation. An attacker can intentionally destruct the implementation contract and then interact with the proxy contract, knowing that the delegatecall will still succeed. This can allow the attacker to bypass access control or validation checks in the proxy contract, execute unintended or malicious code, and potentially compromise the integrity and securi...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), :robot:_primary, :robot:_131_group

# Missing access controls in the governance VoteWeighting contract

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-13
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/13
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-13.md

## Brief Summary

Detailed description of the impact of this finding. The functions addNomineeEVM, addNomineeNonEVM, and checkpoint allow unauthourised addresses to call these functions. I have tested the functions in the

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), edited-by-warden, :robot:_primary, :robot:_79_group

# Incorrect Reward Allocation when Total Rewards Exceed Available Rewards and Only One Service is Eligible.

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-100
- **Submitter:** foxb868
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/100
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-100.md

## Brief Summary

checkpoint function can lead to incorrect reward allocation when the total allocated rewards exceed the available rewards, and there is only one eligible service (i.e., `numServices` is 1). In this case, the service at index 0 will receive the entire `lastAvailableRewards` instead of the proportionally adjusted reward. This can result in the service being overpaid and the available rewards being depleted more quickly than intended. In the loop, the rewards are adjusted for services starting from index 1 up to `numServices - 1`. However, the reward for the service at index 0 is processed separately after the loop. This can lead to incorrect reward allocation if `numServices` is 1. [@StakingB...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_78_group

# calculateStakingLastReward Function Incorrectly Returns Zero for Ineligible Service Instead of Indicating Ineligibility.

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-102
- **Submitter:** foxb868
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/102
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-102.md

## Brief Summary

When the `serviceId` is not found in the `eligibleServiceIds` array, In this case, the loop will complete without setting the reward variable, and the function will return the default value of `reward`, which is 0. This means that if the `serviceId` is not eligible for rewards, the function will still return 0 instead of indicating that no reward is available for that service. If a caller of the function provides a `serviceId` that is not eligible for rewards, the function will return 0, falsely indicating that the service has no reward available. This can cause confusion and incorrect assumptions about the staking rewards for that particular service. In a scenario where a user or another c...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# getStakingState Function Does Not Handle Case When Service tsStart is Zero and Inactivity is Within Limit

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-103
- **Submitter:** foxb868
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/103
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-103.md

## Brief Summary

The case when `sInfo.tsStart` is 0 and `sInfo.inactivity` is less than or equal to `maxInactivityDuration`. In this code, the `getStakingState` function retrieves the `ServiceInfo` for a given `serviceId` from the `mapServiceInfo` mapping. It then checks two conditions: 1. If `sInfo.inactivity` is greater than `maxInactivityDuration`, the service is considered evicted, and `stakingState` is set to `StakingState.Evicted`. 2. If `sInfo.tsStart` is greater than 0, the service is considered staked, and `stakingState` is set to `StakingState.Staked`. However, if both conditions are not met (i.e., `sInfo.tsStart` is 0 and `sInfo.inactivity` is less than or equal to `maxInactivityDuration`), the f...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_12_group

# Incorrect Parameter Order in getProxyAddressWithNonce for Contract Address Derivation.

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-104
- **Submitter:** foxb868
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/104
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-104.md

## Brief Summary

According to the Ethereum contract address generation scheme, the correct order of parameters should be: However, in the getProxyAddressWithNonce code, the order of parameters is incorrect: [@StakingFactory.sol:150-154](https://github.com/code-423n4/2024-05-olas/blob/e2a8bc31d2769bfb578a06cc64919ad369a82c08/registries/contracts/staking/StakingFactory.sol#L150-L154) The `salt` and `keccak256(deploymentData)` are in the wrong order. The correct order should be: This bug will result in incorrect proxy addresses being calculated, which can lead to unexpected behavior and potential vulnerabilities.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# verifyInstanceAndGetEmissionsAmount Function Accesses Unverified Instance Contract

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-105
- **Submitter:** foxb868
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/105
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-105.md

## Brief Summary

In the `verifyInstanceAndGetEmissionsAmount`, if the `verifyInstance` call returns `false`, indicating that the instance verification failed, the function still proceeds to retrieve the emissions amount using `IStaking(instance).emissionsAmount()`. This is problematic because if the instance is not verified, calling `emissionsAmount()` on an invalid instance address could lead to unexpected behavior or even revert the transaction.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# verifyInstance Function Incorrectly Returns True When stakingToken Check Fails

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-106
- **Submitter:** foxb868
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/106
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-106.md

## Brief Summary

The function attempts to retrieve the staking token address by making a `staticcall` to the `stakingToken` function of the `IStaking` interface. However, if the `stakingToken` is not implemented in the staking contract or if it reverts due to an error, the `success` variable will be set to `false`. In such cases, the code continues execution without returning `false`, and the function ultimately returns `true` at the end. This means that even if the staking token check fails or the function is not implemented, the `verifyInstance` function will still return `true` at the end, potentially allowing instances with invalid staking tokens to pass the verification. In the following code block: [@...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# _setL2TargetDispenser Function Permanently Locks Contract by Revoking Owner Role

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-107
- **Submitter:** foxb868
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/107
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-107.md

## Brief Summary

`_setL2TargetDispenser` has a significant impact on the flexibility and maintainability of the contract. By allowing the owner to set the l2TargetDispenser address and then immediately revoking the owner role, the contract becomes permanently `ownerless` after the first call to this function. This means that once the `l2TargetDispenser` address is set, it cannot be modified or updated in the future, even if there is a legitimate need to do so. Consider a scenario where the initially set `l2TargetDispenser` address becomes compromised, invalid, or needs to be updated due to changes in the system architecture. In such cases, the contract would be stuck with the outdated or problematic address...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Incorrect Condition for Staking: Contract Paused State Checked Inversely

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-108
- **Submitter:** foxb868
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/108
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-108.md

## Brief Summary

The condition for checking the OLAS balance and the contract being unpaused: [@DefaultTargetDispenserL2.sol:188-202](https://github.com/code-423n4/2024-05-olas/blob/e2a8bc31d2769bfb578a06cc64919ad369a82c08/tokenomics/contracts/staking/DefaultTargetDispenserL2.sol#L189-L202) The condition `localPaused == 1` suggests that the contract should only approve and transfer OLAS when the contract is paused. However, logically, it should be the opposite. The contract should only approve and transfer OLAS when it is not paused. **`Impact on Staking Functionality:`** If the contract is not paused (`localPaused == 0`), but the condition is checking for `localPaused == 1`, the OLAS tokens will not be app...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Permanent Disabling of redeem Function Due to Improper Reentrancy Guard Handling in DefaultTargetDispenserL2.sol

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-109
- **Submitter:** foxb868
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/109
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-109.md

## Brief Summary

The order of operations when updating the `_locked` variable for the reentrancy guard. Currently, the function sets `_locked = 2` at the beginning and then sets `_locked = 1` at the end of the function. However, if an exception occurs after `_locked` is set to 2 but before it is set back to 1, the contract will be left in a state where `_locked` is permanently set to 2, effectively disabling the `redeem` function for future calls. [@DefaultTargetDispenserL2.sol:266-303](https://github.com/code-423n4/2024-05-olas/blob/e2a8bc31d2769bfb578a06cc64919ad369a82c08/tokenomics/contracts/staking/DefaultTargetDispenserL2.sol#L266-L303) As you can see, `_locked` is set to 2 at the beginning of the func...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_40_group

# Uninitialized Cost Calculation in _sendMessage Function Allows Insufficient Payment When transferAmount is Zero

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-111
- **Submitter:** foxb868
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/111
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-111.md

## Brief Summary

In the _sendMessage function, the code, `totalCost` is calculated as follows: [@ArbitrumDepositProcessorL1.sol:165](https://github.com/code-423n4/2024-05-olas/blob/e2a8bc31d2769bfb578a06cc64919ad369a82c08/tokenomics/contracts/staking/ArbitrumDepositProcessorL1.sol#L165) However, `cost[0]` is only set if `transferAmount > 0`. If `transferAmount` is zero, `cost[0]` remains uninitialized and holds its default value of zero. The bug occurs when `transferAmount` is zero. In this case, `totalCost` will only include the value of cost[1], which represents the cost for the message transfer. The value of cost[0], which should represent the token transfer cost, is not taken into account. As a result,...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_100_group

# Improper Conversion of bytes32 to Address May Result in Loss of Staking Incentives

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-112
- **Submitter:** foxb868
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/112
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-112.md

## Brief Summary

[@Dispenser.sol:424](https://github.com/code-423n4/2024-05-olas/blob/e2a8bc31d2769bfb578a06cc64919ad369a82c08/tokenomics/contracts/Dispenser.sol#L424) In this line, `stakingTarget` is of type `bytes32`, and it is being converted to an `address` by first converting it to `uint256` and then to `uint160`. However, this conversion assumes that the `stakingTarget` always represents a valid Ethereum address. If the `stakingTarget` is not a valid Ethereum address or if it contains any data other than an address, this conversion will result in an incorrect address. This could lead to the staking incentives being sent to the wrong address or even to an invalid address.v

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Incorrect Comparison Due to Improper Conversion in _checkOrderAndValues

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-113
- **Submitter:** foxb868
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/113
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-113.md

## Brief Summary

In `_checkOrderAndValues`, an issue lies in comparing `stakingTargets[i][j]` with `lastTarget`. [@Dispenser.sol:553-556](https://github.com/code-423n4/2024-05-olas/blob/e2a8bc31d2769bfb578a06cc64919ad369a82c08/tokenomics/contracts/Dispenser.sol#L553-L556) The comparison is done by converting `lastTarget` and `stakingTargets[i][j]` to `uint256`. However, `stakingTargets` is defined as `bytes32[][]`, which means each element is of type `bytes32`. Converting `bytes32` to `uint256` can lead to unexpected behavior because `bytes32` is a fixed-size byte array, and the comparison will be based on the byte representation rather than the intended numeric value.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# Activity checker can lead to revert although it must not

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-118
- **Submitter:** fyamf
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/118
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-118.md

## Brief Summary

During checking the ratio pass, it is commented that it must not revert, that is why it is implemented using low-level call. But, even using low-level call does not guarantee it will not revert in certain conditions.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# VoteWeighting:voteForNomineeWeights rounding error on nextTime uint256 calcution

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-226
- **Submitter:** joaovwfreire
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/226
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-226.md

## Brief Summary

There is a rounding error in the `nextTime` calculation due to dividing before multiplying by `WEEK`: The calculation of `nextTime` using integer division rounds down the result. When `block.timestamp + WEEK` is divided by `WEEK` before multiplying by `WEEK`, it causes the intermediate division result to round down, leading to an incorrect `nextTime`. Impact - The incorrect `nextTime` value results in an inaccurate lock period. - Users may vote after their lock period has expired, compromising the integrity of the voting system.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_01_group

# tsCheckPoint not updated after a liveness period could lead to wrong eviction and unshared reward.

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-182
- **Submitter:** lanrebayode77
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/182
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-182.md

## Brief Summary

2. Undistributed reward for some services that should get reward. > /// @dev Checks if the service multisig liveness ratio passes the defined liveness threshold. According to the description above, the is based on liveness period. However, based on the check in the function, when there are no available awards for distribution, it returns an empty array for , and is only updated when is > 1. checkPoint solidity // If service Ids are returned, then the checkpoint takes place if (serviceIds.length > 0) { ... // Record the current timestamp such that next calculations start from this point of time --- tsCheckpoint = block.timestamp; } +++ tsCheckpoint = block.timestamp;

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect assumption of maximum CHAIN_ID value may break support for EVM networks

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-41
- **Submitter:** marchev
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/41
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-41.md

## Brief Summary

The OLAS protocol incorrectly assumes that the maximum `CHAIN_ID` value is `type(uint64).max / 2 - 36`, based on a misinterpretation of EVM specifications. The EVM specification, however, does not enforce such an upper bound limit for `CHAIN_ID`, which is defined as an unsigned 256-bit number. Although EIP-2294 proposed an upper bound limit of `type(uint64).max / 2 - 36`, it is not in a Final state and has been stagnant for over two and a half years. Consequently, there is no standardized maximum value for `CHAIN_ID`, and a network could theoretically use a `CHAIN_ID` greater than `type(uint64).max / 2 - 36`, making it incompatible with the OLAS protocol. While the likelihood of this issue...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Inconsistent Agent Instance Handling in service struct and stakingParams

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-181
- **Submitter:** nonn_ac
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/181
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-181.md

## Brief Summary

In stakingBase.sol, the service struct contains the following: This represents the total number of agencies and the actual number of agencies. However, in stakingParams, the struct only contains: while staking, `numAgentInstances` in the service struct is not used as a reference to `numAgentInstances` in stakingParams. Instead, it uses `maxNumAgentInstances`: leaving ``numAgentInstances`` uninitialized.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential Front-Running Vulnerability in Staking Incentives Claiming Process

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-238
- **Submitter:** obingo76
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/238
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-238.md

## Brief Summary

The identified vulnerability lies within the ``claimStakingIncentives`` function of the Dispenser contract. This function allows users to claim staking incentives based on certain parameters such as the number of claimed epochs, chain ID, staking target, and bridge payload. The vulnerability stems from the possibility of exploiting the function to perform actions that were not intended by the original design, potentially leading to unauthorized benefits or disruptions in the staking process.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_75_group

# `Unstaking` failure on some chains due to `WETH` transfer Compatibility issue on some Chains.

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-183
- **Submitter:** okolicodes
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/183
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-183.md

## Brief Summary

Because the `StakingBase` never `approves` itself to spend `WETH`, the `token` transfer will always revert making it impossible to `unstake`

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# migrate in DefaultTargetDispenserL2 should migrate the native tokens as well.

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-67
- **Submitter:** peanuts
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/67
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-67.md

## Brief Summary

Native tokens will be stuck in the contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_16_group

# There is no `onlyWormholeRelayer` modifier when receiving messages on L1 and L2 through Wormhole

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-68
- **Submitter:** peanuts
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/68
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-68.md

## Brief Summary

Anybody can call `receiveWormholeMessages()` and input arbitrary data to mess up the accounting system. For L1 -> L2, anyone can fabricate any amount of `targets` and `amounts` through the data parameter. For L2 -> L1, anyone can set an arbitrary withheldSyncAmount.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_89_group

# Messages from Optimism can be hijacked by calling receiveMessages() directly

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-71
- **Submitter:** peanuts
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/71
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-71.md

## Brief Summary

Users can send arbitrary data to L1 or L2 through `receiveMessage()`, breaking accounting.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# If totalRewards > lastAvailableRewards, the first staker will get twice the rewards instead of the leftover rewards only

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-83
- **Submitter:** peanuts
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/83
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-83.md

## Brief Summary

First service staker will get double the rewards if `totalRewards > lastAvailableRewards`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_44_group

# Lack of Secure and Transparent Upgrade Mechanism

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-142
- **Submitter:** sunnyboy95
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/142
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-142.md

## Brief Summary

Implement an onlyOwner modifier to ensure that only the owner can upgrade the implementation address. Ownership Transfer Mechanism: Providing a function to transfer ownership to a new address, allowing administrative flexibility. Event Emissions: Emit events for significant actions such as upgrading the implementation address and transferring ownership to ensure transparency and traceability.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# When voting, users need to be given a deadline to prevent unintended voting.

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-186
- **Submitter:** zraxx
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/186
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-186.md

## Brief Summary

A user's vote may take effect at an unexpected time. Detail The protocol voting interval is one week, which means that the current vote will take effect next week. One situation is that a user votes on a weekend to get a quick effect, but due to congestion or delays, the actual voting time may exceed the current week, causing the user's vote to take effect in the the week after next.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# When queueHash is the same, redeem will fail

- **Contest:** Olas
- **Slug:** 2024-05-olas
- **Submission:** V-188
- **Submitter:** zraxx
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-olas-validation/issues/188
- **Source snapshot:** competitions/2024-05-olas/submissions/raw/V-188.md

## Brief Summary

redeem will fail when there are the same hashes.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_19_group
