# Rejected Primary Findings: Chainlink

# OnRamp and OffRamp does not configure gas yield on blast

- **Contest:** Chainlink
- **Slug:** 2024-11-chainlink
- **Submission:** F-45
- **Submitter:** Oblivionis
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-11-chainlink/submissions/F-45
- **Source snapshot:** competitions/2024-11-chainlink/submissions/raw/F-45.txt

## Brief Summary

Existing L2s like Optimism and Arbitrum keep sequencer fees for themselves. Blast redirects sequencer fees to the dapps that induced them, allowing smart contract developers to have an additional source of revenue. Contracts have two options for their Gas Mode: Void (DEFAULT) and Claimable. However in current CCIP implementation, all contracts do not setting Gas Mode to Claimable. Considering CCIP is an active cross-chain protocol with a large number of transactions and comsume huge amount of gas, there will be huge value-leak. In other words, for all messages to Blast, the sequencer fee prepaid by users on any source chain will permanently get locked.

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Malicious relayer can execute messages OOO or grief the user by purposefully forcing the message to fail due to insufficient gas

- **Contest:** Chainlink
- **Slug:** 2024-11-chainlink
- **Submission:** F-5
- **Submitter:** haxatron
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-chainlink/submissions/F-5
- **Source snapshot:** competitions/2024-11-chainlink/submissions/raw/F-5.txt

## Brief Summary

The OffRamp now attempts to catch all errors and sets the message status to FAILURE if it occurs. This was one of the major changes after the CodeHawks CCIP audit. OffRamp.sol#L532-L545 function _trialExecute( Internal.Any2EVMRampMessage memory message, bytes[] memory offchainTokenData, uint32[] memory tokenGasOverrides ) internal returns (Internal.MessageExecutionState executionState, bytes memory) { try this.executeSingleMessage(message, offchainTokenData, tokenGasOverrides) {} catch (bytes memory err) { // return the message execution state as FAILURE and the revert data. // Max length of revert data is Router.MAX_RET_BYTES, max length of err is 4 + Router.MAX_RET_BYTES. return (Internal...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Double Spending Due to Optimism Commitment Challenge

- **Contest:** Chainlink
- **Slug:** 2024-11-chainlink
- **Submission:** F-57
- **Submitter:** joaovwfreire
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-chainlink/submissions/F-57
- **Source snapshot:** competitions/2024-11-chainlink/submissions/raw/F-57.txt

## Brief Summary

Messages sent from Optimism to Ethereum Mainnet rely on state commitments from the Optimism rollup. These commitments are subject to fault proof games during a 7-day challenge window. If a commitment is successfully challenged and the Optimism state is reverted, a user may reclaim their tokens on Optimism while the corresponding message is still processed on Mainnet. This creates a double-spending vulnerability. Critically, the system does not account for the possibility of a state rollback during the challenge window, allowing messages to be processed on Mainnet even though the corresponding state on Optimism may later be invalidated. Relying solely on block finality for readiness ignores...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Gas Manipulation in `CallWithExactGas` leads to Transaction Reverts

- **Contest:** Chainlink
- **Slug:** 2024-11-chainlink
- **Submission:** F-79
- **Submitter:** DaidalosInitiative
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-chainlink/submissions/F-79
- **Source snapshot:** competitions/2024-11-chainlink/submissions/raw/F-79.txt

## Brief Summary

The CallWithExactGas contract forwards the entire gasLimit to target contracts during execution, allowing malicious receivers to precisely consume gas and force transaction reverts by triggering EIP-150's 1/64th rule. Details The CallWithExactGas contract performs upfront validation to ensure sufficient gas is available for execution including the EIP-150 1/64th reserve. However, during the actual low-level call, it forwards the entire gasLimit to the target contract: success := call(gasLimit, target, 0, add(payload, 0x20), mload(payload), 0x0, 0x0) This gives target contracts complete control over gas consumption during their execution window. A malicious contract can: Calculate the forwar...

## Rejection Reason

Final severity/evaluation: Low; Valid Primary Sufficient

# dont directly use `ecrecover()` in code to be deployed on zksync era.

- **Contest:** Chainlink
- **Slug:** 2024-11-chainlink
- **Submission:** F-30
- **Submitter:** adeolu
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-chainlink/submissions/F-30
- **Source snapshot:** competitions/2024-11-chainlink/submissions/raw/F-30.txt

## Brief Summary

In the codebase, ecrecover() is used here and here. This codebase is to be deployed on zksync era as it is listed as a supported chain in the contesst readme. ecrecover() should not be directly used for signature verification on zksync era as zkSync EOA accounts are quite different from the ethereum EOA accounts which ecrecover() is made to work for. On ethereum, There are two account types: Contract account, accounts with code and Externally Onwed Accounts(EOA) accounts which are controlled by anyone with the private keys, as such only accounts that are controlled by a private key can produce an ESCDA signature of which the signer can be recoverable via ecrecover(). Contract accounts canno...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# MultiOCR3Base MAX_NUM_ORACLES value can lead to DOS on certain EVM-compatible chains

- **Contest:** Chainlink
- **Slug:** 2024-11-chainlink
- **Submission:** F-32
- **Submitter:** ayden
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-chainlink/submissions/F-32
- **Source snapshot:** competitions/2024-11-chainlink/submissions/raw/F-32.txt

## Brief Summary

The Maximum number of oracles the offchain reporting protocol is designed for 256. However, setting 256 oracles consumes a substantial amount of gas, potentially exceeding the current block's gasLimit and resulting in a denial of service (DoS) for the setOCR3Configs function.

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Integer Overflow in _setOCR3Config Function

- **Contest:** Chainlink
- **Slug:** 2024-11-chainlink
- **Submission:** F-34
- **Submitter:** giodem
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-chainlink/submissions/F-34
- **Source snapshot:** competitions/2024-11-chainlink/submissions/raw/F-34.txt

## Brief Summary

The _setOCR3Config function allows the signers array to have up to MAX_NUM_ORACLES (256) entries. However, it assigns configInfo.n using a uint8, which can only represent values from 0 to 255. When signers.length is exactly 256, uint8(signers.length) wraps around to 0 due to integer overflow. This results in configInfo.n being incorrectly set to 0 instead of 256. Such a mismatch can lead to improper validation and enforcement of the number of signers, potentially undermining the security guarantees of the protocol by allowing configurations that should be invalid or improperly managed.

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Token price update tx can be arbitrage by MEV

- **Contest:** Chainlink
- **Slug:** 2024-11-chainlink
- **Submission:** F-23
- **Submitter:** kutugu
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-chainlink/submissions/F-23
- **Source snapshot:** competitions/2024-11-chainlink/submissions/raw/F-23.txt

## Brief Summary

Each token price update will set it to the latest token price. When the token price fluctuates violently, there may be multiple updated txs in the same period; even if the token price is stable, there may be multiple updated txs due to the presence of multiple updaters. When the prices of these txs are inconsistent, it creates an opportunity for MEV arbitrage. The tx sent later can be overwritten by the tx sent earlier. They can choose to execute the tx with the highest token price to save fees. Even more extreme, if the updated tx stays in the mempool due to low gasPrice, it may also be arbitraged by the MEV.

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Underbilling due to lack of inclusion of token transfer associated bytes in the execution cost

- **Contest:** Chainlink
- **Slug:** 2024-11-chainlink
- **Submission:** F-16
- **Submitter:** hash
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-11-chainlink/submissions/F-16
- **Source snapshot:** competitions/2024-11-chainlink/submissions/raw/F-16.txt

## Brief Summary

Lower calculation of execution cost hence underbilling

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Keystone updates will expose rounding errors for +18 decimals tokens and can truncate to `0` for `36` decimals tokens.

- **Contest:** Chainlink
- **Slug:** 2024-11-chainlink
- **Submission:** F-58
- **Submitter:** Al-Qa-qa
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-11-chainlink/submissions/F-58
- **Source snapshot:** competitions/2024-11-chainlink/submissions/raw/F-58.txt

## Brief Summary

There's an increased risk of rounding errors when working with tokens with >36 decimals in the FeeQuoter. The statement tills that rounding errors will increase for tokens greater than 36 decimals. Still, our issue will prove that even +18 decimals tokens (24, 30, 36) token decimals will subjected to large rounding. Even a token with 36 decimals (NOTE: do not exceed 36 decimals as the README said), can get truncated to make the value zero. Description When receiving Keystone price updates, the price fee decimals have a constant value equals 18, we send the token decimals and price we want to update and calculate rebasedValue value. FeeQuoter.sol#L515-L516 function onReport(bytes calldata me...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Legacy curse can cause all messages to be marked failed without attempting execution

- **Contest:** Chainlink
- **Slug:** 2024-11-chainlink
- **Submission:** F-65
- **Submitter:** hash
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-11-chainlink/submissions/F-65
- **Source snapshot:** competitions/2024-11-chainlink/submissions/raw/F-65.txt

## Brief Summary

When LEGACY_CURSE_SUBJECT is set, users can loose gas fees and have their nonce dishonoured

## Rejection Reason

Final severity/evaluation: Low; Valid Primary Sufficient

# Execution will get reverted for all reports if one of the chains is not-enabled

- **Contest:** Chainlink
- **Slug:** 2024-11-chainlink
- **Submission:** F-60
- **Submitter:** Al-Qa-qa
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-11-chainlink/submissions/F-60
- **Source snapshot:** competitions/2024-11-chainlink/submissions/raw/F-60.txt

## Brief Summary

When transmitters execute reports, they execute more than one report in a single execution. for each single report execution, we call _getEnabledSourceChainConfig() which reverts in case the chain is not-enabled. offRamp/OffRamp.sol#L391 function _executeSingleReport( ... ) internal { ... { bytes32 metaDataHash = keccak256( abi.encode( Internal.ANY_2_EVM_MESSAGE_HASH, sourceChainSelector, i_chainSelector, >> keccak256(_getEnabledSourceChainConfig(sourceChainSelector).onRamp) ) ); } // ------------------ function _getEnabledSourceChainConfig( ... ) internal view returns (SourceChainConfig storage) { SourceChainConfig storage sourceChainConfig = s_sourceChainConfigs[sourceChainSelector]; if (...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Attacker can exploit executeable configuration updates to pay less

- **Contest:** Chainlink
- **Slug:** 2024-11-chainlink
- **Submission:** F-2
- **Submitter:** hash
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-chainlink/submissions/F-2
- **Source snapshot:** competitions/2024-11-chainlink/submissions/raw/F-2.txt

## Brief Summary

Attacker can pay less fees when there are pending configuration updates

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Initial sender nonce for users that used a previous on/off ramp will be incremented twice

- **Contest:** Chainlink
- **Slug:** 2024-11-chainlink
- **Submission:** F-61
- **Submitter:** gesha17
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-chainlink/submissions/F-61
- **Source snapshot:** competitions/2024-11-chainlink/submissions/raw/F-61.txt

## Brief Summary

When a user first uses the new on/off ramps the nonce manager will fetch the last nonce from the previous ramps. This is done to keep the nonces going from where they stopped at in the previous architecture. However, the nonce will be incremented twice for users that have used a previous on/off ramp. This happens because the call to fetch the sender nonce to the previos ramp will return the next nonce instead of the last one and also that returned value is then incremented again. This will result in messages that are not sent to the destination chain because the offchain code does checks on the sender nonce, so the message will be marked as invalid. This will lead to loss of funds as funds...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# GAS_FOR_CALL_EXACT_CHECK does not account for different opcode call costs on Blast

- **Contest:** Chainlink
- **Slug:** 2024-11-chainlink
- **Submission:** F-14
- **Submitter:** haxatron
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-11-chainlink/submissions/F-14
- **Source snapshot:** competitions/2024-11-chainlink/submissions/raw/F-14.txt

## Brief Summary

GAS_FOR_CALL_EXACT_CHECK does not account for different opcode call costs on Blast. On Blast, there is a slight difference on how gas is charged when an external CALL is made. Specifically, if a transaction has made calls to five different contracts in the same transaction, the Blast EVM will charge an additional 7100 gas in the caller context for any additional CALL made. func makeCallVariantGasCallEIP2929(oldCalculator gasFunc, isProxiedCall bool) gasFunc { return func(evm *EVM, contract *Contract, stack *Stack, mem *Memory, memorySize uint64) (uint64, error) { ... if evm.frameCount > params.BlastMaxFrameCount && isNewFrame { blastGasCost = params.BlastGasParamStorageGas if !contract.UseG...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Malicious transmitters can force messages to fail by executing reports with a low gas limit

- **Contest:** Chainlink
- **Slug:** 2024-11-chainlink
- **Submission:** F-44
- **Submitter:** spuriousdragon
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-chainlink/submissions/F-44
- **Source snapshot:** competitions/2024-11-chainlink/submissions/raw/F-44.txt

## Brief Summary

In order to avoid situations where one message failing affects the whole batch execution logic and makes the whole transaction revert, CCIP implements a safety mechanism where messages that fail will be directly set to a FAILURE state instead of reverting. This can be seen in the _trialExecute function: // File: OffRamp.sol function _trialExecute( Internal.Any2EVMRampMessage memory message, bytes[] memory offchainTokenData, uint32[] memory tokenGasOverrides ) internal returns (Internal.MessageExecutionState executionState, bytes memory) { try this.executeSingleMessage(message, offchainTokenData, tokenGasOverrides) {} catch (bytes memory err) { // return the message execution state as FAILUR...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Double Message Execution via Sequence Number Race Condition During Chain Reorgs

- **Contest:** Chainlink
- **Slug:** 2024-11-chainlink
- **Submission:** F-84
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-chainlink/submissions/F-84
- **Source snapshot:** competitions/2024-11-chainlink/submissions/raw/F-84.txt

## Brief Summary

The CCIP cross-chain messaging system is vulnerable to blockchain reorganizations in its message sequencing and execution logic. The core issue exists in the OnRamp's message sequencing and OffRamp's execution where blockchain reorgs could cause message duplications and inconsistent states across chains. When the OnRamp processes a message, it increments a sequence number and emits a CCIPMessageSent event: function forwardFromRouter(...) external returns (bytes32) { // Sequence number is incremented without reorg protection newMessage.header.sequenceNumber = ++destChainConfig.sequenceNumber; // This locks tokens and emits an event with the sequence emit CCIPMessageSent(destChainSelector, ne...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# gasPerPubdataByte not taken into account in Chainlink gas logic, logic not compatible with zkSync's zkEVM environment

- **Contest:** Chainlink
- **Slug:** 2024-11-chainlink
- **Submission:** F-31
- **Submitter:** Topmark
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-chainlink/submissions/F-31
- **Source snapshot:** competitions/2024-11-chainlink/submissions/raw/F-31.txt

## Brief Summary

It is noted in Audit Doc that Chainlink is to be deployed on zkSync era however gasPerPubdataByte value is not taken into account in gas logic, the code will revert/tx will fail if the call is made in connection to zksync

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Execution Cost Miscalculation Due to Missing Calldata Cost

- **Contest:** Chainlink
- **Slug:** 2024-11-chainlink
- **Submission:** F-52
- **Submitter:** 0xRobocop
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-chainlink/submissions/F-52
- **Source snapshot:** competitions/2024-11-chainlink/submissions/raw/F-52.txt

## Brief Summary

The FeeQuoter contract is responsible for calculating the fee associated with message transmission in the CCIP system via the getValidatedFee function. This function computes the final fee by considering three components: premiumFee, executionCost, and dataAvailabilityCost. return ((premiumFee * s_premiumMultiplierWeiPerEth[message.feeToken]) + executionCost + dataAvailabilityCost) / feeTokenPrice; The executionCost is intended to account for the cost associated with executing the message on the destination chain. This cost is computed by adding all the amount of gas it will take to execute the message successfully. The components of the total amount of gas is divided in 4: A destGasOverhea...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# user is able to frontrun DON thereby breaking a core invariant

- **Contest:** Chainlink
- **Slug:** 2024-11-chainlink
- **Submission:** F-80
- **Submitter:** mxuse
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-11-chainlink/submissions/F-80
- **Source snapshot:** competitions/2024-11-chainlink/submissions/raw/F-80.txt

## Brief Summary

inside the execute flow the following check gets made: if (manualExecution) { tokenGasOverrides = manualExecGasExecOverrides[i].tokenGasOverrides; bool isOldCommitReport = (block.timestamp - timestampCommitted) > s_dynamicConfig.permissionLessExecutionThresholdSeconds; // Manually execution is fine if we previously failed or if the commit report is just too old. // Acceptable state transitions: UNTOUCHED->SUCCESS, UNTOUCHED->FAILURE, FAILURE->SUCCESS. if (!(isOldCommitReport || originalState == Internal.MessageExecutionState.FAILURE)) { revert ManualExecutionNotYetEnabled(sourceChainSelector); } This check simply ensures that the DON has permissionLessExecutionThresholdSeconds to perform ex...

## Rejection Reason

Final severity/evaluation: Low; Valid Primary Sufficient

# Nonce Reordering may happen during L1 Block Re-orgs and lead to out-of-order message execution

- **Contest:** Chainlink
- **Slug:** 2024-11-chainlink
- **Submission:** F-49
- **Submitter:** joaovwfreire
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-chainlink/submissions/F-49
- **Source snapshot:** competitions/2024-11-chainlink/submissions/raw/F-49.txt

## Brief Summary

In the OnRamp contract, the nonce is incremented whenever a message is sent that must not be executed out of order. However, in the event of an L1 block reorganization (re-org), messages sent in one block might be included in a different order in the final chain. This disrupts the intended execution order enforced by nonces, potentially leading to the execution of transactions out of order. Impact It can lead to incorrect execution of cross-chain messages. For instance, if a user sends multiple sequential messages that are intended to be executed in a specific order, a block re-org can cause these messages to be processed out of sequence. This can lead to various issues, including failures...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# CCIPHome doesn’t implement the getCapabilityConfiguration(), which results in the capability registry not being able to fetch configs

- **Contest:** Chainlink
- **Slug:** 2024-11-chainlink
- **Submission:** F-62
- **Submitter:** EastHats
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-chainlink/submissions/F-62
- **Source snapshot:** competitions/2024-11-chainlink/submissions/raw/F-62.txt

## Brief Summary

The Capability Registry’s is used to manage Nodes (including their links to Node Operators), Capabilities, and DONs (Decentralized Oracle Networks) which are sets of nodes that support those Capabilities. When a capability’s data is being set the logic also sets a configuration contract as well which is the CCIPHome contract. Its purpose is to store the configuration for the CCIP capability. There are two classes of configuration: chain configuration and DON (in the CapabilitiesRegistry sense) configuration. Each chain has a single configuration which includes information like the router address. The problem is that there are multiple requirements for a “configuration contract”, including:...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# `_transmit()` will be reverted with an out-of-bound error when the number of signatures exceeds `32`

- **Contest:** Chainlink
- **Slug:** 2024-11-chainlink
- **Submission:** F-12
- **Submitter:** stuart_the_minion
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-chainlink/submissions/F-12
- **Source snapshot:** competitions/2024-11-chainlink/submissions/raw/F-12.txt

## Brief Summary

In CCIP v1.6, the protocol increases maximum number of oracles from 32 to 256. abstract contract MultiOCR3Base is ITypeAndVersion, Ownable2StepMsgSender { uint256 internal constant MAX_NUM_ORACLES = 256; ... ... } And, the MultiOCR3Base::_verifySignatures() function takes rs, ss and rawVs to verify the hashed report with the signatures. function _verifySignatures( uint8 ocrPluginType, bytes32 hashedReport, bytes32[] memory rs, bytes32[] memory ss, bytes32 rawVs ) internal view { // Verify signatures attached to report. Using a uint256 means we can only verify up to 256 oracles. uint256 signed = 0; uint256 numberOfSignatures = rs.length; for (uint256 i; i < numberOfSignatures; ++i) { // Safe...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# A chain reorganization may cause one source to block the price update of another source.

- **Contest:** Chainlink
- **Slug:** 2024-11-chainlink
- **Submission:** F-13
- **Submitter:** shaflow2
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-chainlink/submissions/F-13
- **Source snapshot:** competitions/2024-11-chainlink/submissions/raw/F-13.txt

## Brief Summary

Using a global s_latestPriceSequenceNumber for price updates can lead to a situation where, after a chain reorganization, one source blocks another source's price updates. This results in subsequent cross-chain message fees being calculated using outdated prices.

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Token registration with the multi aggregate rate limiter should function independently for both onRamp and offRamp

- **Contest:** Chainlink
- **Slug:** 2024-11-chainlink
- **Submission:** F-9
- **Submitter:** ether_sky
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-chainlink/submissions/F-9
- **Source snapshot:** competitions/2024-11-chainlink/submissions/raw/F-9.txt

## Brief Summary

There are two types of rate limiters in the system: Token Pool Rate Limiters: These limit individual token transfers. Multi-Aggregate Rate Limiters: These limit the total transfer of tokens to and from a chain. In the previous version, adding a token to the aggregate rate limiter was independent for onRamp (outgoing tokens from a chain) and offRamp (ingoing tokens to a chain). This design allowed for flexibility in how token transfers were managed. For example: Selective Limiting: Token owners could limit outgoing transfers from a chain without affecting incoming transfers. This was useful for specific purposes where restricting one direction was sufficient. There is no requirement for all...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Malicious `offChainTokenData` can be passed to mark a message FAILED and dishonour nonce ordering

- **Contest:** Chainlink
- **Slug:** 2024-11-chainlink
- **Submission:** F-18
- **Submitter:** hash
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-chainlink/submissions/F-18
- **Source snapshot:** competitions/2024-11-chainlink/submissions/raw/F-18.txt

## Brief Summary

An attacker can intentionally mark a user's message as FAILED and dishonour the nonce ordering

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient
