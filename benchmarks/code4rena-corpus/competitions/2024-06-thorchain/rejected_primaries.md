# Rejected Primary Findings: Thorchain

# Insufficient Event Validation in `smartcontract_log_parser::unpackVaultLog`

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Submission:** V-175
- **Submitter:** 0xsi
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-thorchain-validation/issues/175
- **Source snapshot:** competitions/2024-06-thorchain/submissions/raw/V-175.md

## Brief Summary

In the `unpackVaultLog` function, the code assumes that the log.Topics array has enough elements to parse the required indexed arguments. This assumption can be risky because if the log.Topics array is shorter than expected, the function will encounter an error when trying to access elements that do not exist. - Event Signature Check: The function starts by checking if log.Topics is not empty and if the first topic matches the expected event signature. Data Unpacking: If log.Data is present, it attempts to unpack it into the provided output structure. - Indexed Arguments Parsing: It iterates over the event's inputs to find indexed arguments and then attempts to parse the topics into these i...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_06_group

# Missing Event Emission in `transferAllowance` Function When Using External Router

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Submission:** V-244
- **Submitter:** Afriauditor
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-thorchain-validation/issues/244
- **Source snapshot:** competitions/2024-06-thorchain/submissions/raw/V-244.md

## Brief Summary

The transferAllowance function does not emit an event when router != address(this). This omission leads to a lack of transparency and tracking for allowance deductions and transfers made from the vault in this function call. The absence of an event contradicts also the code comment that states `events are emitted for all outgoing transfers`, as the network will not be alerted to these critical transactions.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# leading to a denial of service (DoS) situation

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Submission:** V-84
- **Submitter:** Ali55
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-thorchain-validation/issues/84
- **Source snapshot:** competitions/2024-06-thorchain/submissions/raw/V-84.md

## Brief Summary

The vulnerability discovered in the `thorchain/chain/handler.go` file could allow an attacker to potentially cause a panic in the Go program, leading to a denial of service (DoS) situation. This is due to insufficient input validation when processing transactions involving the THOR token.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Depositing via any router through `THORChain_Router_routerDeposit()` could be easily broken for some supported tokens

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Submission:** V-134
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-thorchain-validation/issues/134
- **Source snapshot:** competitions/2024-06-thorchain/submissions/raw/V-134.md

## Brief Summary

Core functionality of protocol would be DOS'd considering access to `THORChain_Router_routerDeposit()` would be broken.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_00_group

# Zero value transfers are not ignored in log-parser.go when they should be ignored since Zero value transfers and approvals should revert according to the README

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Submission:** V-242
- **Submitter:** Bigsam
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-thorchain-validation/issues/242
- **Source snapshot:** competitions/2024-06-thorchain/submissions/raw/V-242.md

## Brief Summary

Transfer and Approve function according to the readme should revert on zero value transfer and approvals but the THORChain_Router fails to check this and relies on the Smartcontract_log_parser.go to ignore zero value transactions. The GetTXInItem fails to remove some transaction that should be ignored when we are swapping and when we are transferring. According to the README > Revert on zero value transfers Yes, AND > Revert on zero value approvals Yes

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# DoS can occur in THORChain_Router.sol

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Submission:** V-127
- **Submitter:** EaglesSecurity
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-thorchain-validation/issues/127
- **Source snapshot:** competitions/2024-06-thorchain/submissions/raw/V-127.md

## Brief Summary

In THORChain_Router.sol functions batchTransferOutV5() and batch transferOutAndCallV5() based on documentation are called by vaults. We can see that they use loops to proceed data. The problem is that external calls can fail accidentally or deliberately, which can cause a DoS condition in the contract. To minimize the damage caused by such failures, it is better to isolate each external call into its own transaction as it is in transferOutV5() and transferOutAndCallV5() that can be initiated by the recipient of the call.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_21_group

# Migrating vault is not putting a proper deadline

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Submission:** V-66
- **Submitter:** Jorgect
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-thorchain-validation/issues/66
- **Source snapshot:** competitions/2024-06-thorchain/submissions/raw/V-66.md

## Brief Summary

The `depositWithExpiry` has a expiration input to ensure that transactions are executed within a specified time frame this is made for several reason as Price protection, Transaction finality, security between other, it's a common practices implement deadline in router, dex, lending protocols: [[Link]](https://github.com/code-423n4/2024-06-thorchain/blob/e3fd3c75ff994dce50d6eb66eb290d467bd494f5/chain/ethereum/contracts/THORChain_Router.sol#L138) The problem is that when a vault is migrating to other router this is been doing through the depositWithExpiry function the `_routerDeposit` function (see

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_05_group

# The smartcontract_log_parser.go client is not checking is the `depositWithExpired` function is used with a correct vault

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Submission:** V-71
- **Submitter:** Jorgect
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-thorchain-validation/issues/71
- **Source snapshot:** competitions/2024-06-thorchain/submissions/raw/V-71.md

## Brief Summary

The `GetTxInItem` function is doing some important validations in each emitted event, in this case we going to analyze the `depositEvent`: [[Link]](https://github.com/code-423n4/2024-06-thorchain/blob/e3fd3c75ff994dce50d6eb66eb290d467bd494f5/bifrost/pkg/chainclients/shared/evm/smartcontract_log_parser.go#L182C8-L182C20) We can see that the contract is validating some importan values as the to input in the deposit event is equal to the txInItem.To. Another intersted check is that the asset is been validate in a assetResolver function we can see how this functions is gonna look in the test: The problem is that this functions is not been validate the `to `value which is the vault to deposit. I...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_04_group

# Malicius user can spam the chain sending depositing just 1 wei

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Submission:** V-72
- **Submitter:** Jorgect
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-thorchain-validation/issues/72
- **Source snapshot:** competitions/2024-06-thorchain/submissions/raw/V-72.md

## Brief Summary

The `GetTxInItem` function is doing some important validations in each emitted event, in this case we going to analyze the `depositEvent`: [[Link]](https://github.com/code-423n4/2024-06-thorchain/blob/e3fd3c75ff994dce50d6eb66eb290d467bd494f5/bifrost/pkg/chainclients/shared/evm/smartcontract_log_parser.go#L182C8-L182C20) The important check here in this report is the `depositEvt.Amount.Bits()) == 0` The problem is that this is not enough to prevent spam. An attacker can send thousand of transaction with just 1 wei deposit doing the node do to much work. Impact The network can be spammed depositing 1 wei thousand of time.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Some tokens approval and transactions will get reverted.

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Submission:** V-128
- **Submitter:** Maushish
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-thorchain-validation/issues/128
- **Source snapshot:** competitions/2024-06-thorchain/submissions/raw/V-128.md

## Brief Summary

In the whitelist of tokens its already mentioned that THORChain supports [UNI](https://gitlab.com/thorchain/thornode/-/blob/develop/common/tokenlist/ethtokens/eth_mainnet_latest.json?ref_type=heads#L2179), [COMP](https://gitlab.com/thorchain/thornode/-/blob/develop/common/tokenlist/ethtokens/eth_mainnet_latest.json?ref_type=heads#L1139). These tokens have some weird behaviors when it comes to approvals and sending transactions: - If the amt is more than `type(unit96).max` in case of UNI approval gets reverted. - If the amt is more than `type(unit96).max` in the case of COMP, both transactions and approvals get reverted.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Potential resource leaks or deadlocks

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Submission:** V-67
- **Submitter:** MrxSnowden
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-thorchain-validation/issues/67
- **Source snapshot:** competitions/2024-06-thorchain/submissions/raw/V-67.md

## Brief Summary

The vulnerability in question pertains to improper handling of semaphore acquisition errors in the extractTxs method. Semaphores are used to control the number of concurrent operations. In the current implementation, if sem.Acquire fails, the semaphore is not released, leading to potential resource leaks or deadlocks. Impact The impact of this vulnerability can be significant in a long-running system. If the semaphore is not released properly: 1- Resource Exhaustion: Over time, the available slots in the semaphore will be exhausted, preventing any further transactions from being processed. 2- System Hang or Crash: As resources become exhausted, the system may hang or crash, leading to downt...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# [M-04] Missing deadline check in transfer out functions

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Submission:** V-151
- **Submitter:** Svetoslavb
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-thorchain-validation/issues/151
- **Source snapshot:** competitions/2024-06-thorchain/submissions/raw/V-151.md

## Brief Summary

All the transfer out and call functions inside `THORChain_Router` are missing the deadline parameter. Deadline provides users the option to limit the execution of their transaction. Without a deadline parameter, users can execute their transactions at unexpected times when market conditions are unfavourable. Refer to [this issue](https://solodit.xyz/issues/m-03-missing-deadline-check-in-swap-functions-pashov-none-kekotron-markdown) Users will miss positive slippage

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_44_group

# No check for "chainid"

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Submission:** V-173
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-thorchain-validation/issues/173
- **Source snapshot:** competitions/2024-06-thorchain/submissions/raw/V-173.md

## Brief Summary

Detailed description of the impact of this finding. No check for chainID in NewETHScanner.There is no check for whether we are using the correct chainID.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# calculation of gasPriceWei can get precision loss.

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Submission:** V-174
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-thorchain-validation/issues/174
- **Source snapshot:** competitions/2024-06-thorchain/submissions/raw/V-174.md

## Brief Summary

Detailed description of the impact of this finding. There is a division before multiplication is happening in calculating gasPriceWei.This will cause a precision loss.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Infinite Allowance in THORChain_Router Contract Exposes Users to Unauthorized Token Draining

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Submission:** V-205
- **Submitter:** cheatc0d3
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-thorchain-validation/issues/205
- **Source snapshot:** competitions/2024-06-thorchain/submissions/raw/V-205.md

## Brief Summary

Allowing an authorized party to set or use max/infinite allowances poses a significant security risk. A malicious authorized party could exploit these allowances to drain the user's tokens without their explicit consent, leading to potential loss of funds.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Network Delays Can Cause Transactions to Revert in depositWithExpiry Function Due to Close Expiration Time

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Submission:** V-210
- **Submitter:** cheatc0d3
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-thorchain-validation/issues/210
- **Source snapshot:** competitions/2024-06-thorchain/submissions/raw/V-210.md

## Brief Summary

Network delays can cause transactions to take longer to be included in a block. If the `expiration` parameter in the `depositWithExpiry` function is too close to the current time, these delays can lead to the transaction being considered expired before it is mined. This results in the function reverting, causing inconvenience to users and potentially leading to the loss of transaction fees.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_17_group

# The code assumes the genesis state is properly formatted. An attacker could provide malformed genesis state data, potentially leading to crashes or unexpected behavior.

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Submission:** V-248
- **Submitter:** cmishra
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-thorchain-validation/issues/248
- **Source snapshot:** competitions/2024-06-thorchain/submissions/raw/V-248.md

## Brief Summary

Detailed description of the impact of this finding.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# `Removed` field from event log is ignored which can lead to false event being processed

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Submission:** V-79
- **Submitter:** dontonka
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-thorchain-validation/issues/79
- **Source snapshot:** competitions/2024-06-thorchain/submissions/raw/V-79.md

## Brief Summary

[Removed field](https://github.com/ethereum/go-ethereum/blob/release/1.14/core/types/log.go#L53) is being `ignored` in the current `SmartContractLogParser::GetTxInItem` implementation, which could potentially lead THORChain to process `fake event` as those have been removed due to reorg on the operating chain, which seem to warrant `Medium` severity. As noted in the `go-ethereum` codebase: The current report `might not` be a problem in the moment, as THORChain node is fecthing `transaction manually` and not throught a filter query. Nevertheless, as explained in my other report (Bifrost `risk of DoS` due to the increase in transactions and events to process), the team will most likely needs...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# `_transferOutV5` implementation allows a discrepancy to happen which can cause loss of funds.

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Submission:** V-37
- **Submitter:** dvrkzy
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-thorchain-validation/issues/37
- **Source snapshot:** competitions/2024-06-thorchain/submissions/raw/V-37.md

## Brief Summary

In `THORChain_Router.sol` we have `transferOutV5()` which allows any vault to transfer any asset to any recipient (including ETH). It is a public payable function which calls the internal `_transferOutV5()`. The issues is that the internal function doesn't check if the `amount` of `asset` a vault has specified to transfer to the recipient is equal to the `msg.value` that was sent (if we are sending ETH). This allows ETH to be stuck in the contract that an attacker can claim. Impact Loss of funds. Attacker can claim stuck ETH.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_09_group

# Vaults can transfer their allowance to stale vaults

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Submission:** V-41
- **Submitter:** dvrkzy
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-thorchain-validation/issues/41
- **Source snapshot:** competitions/2024-06-thorchain/submissions/raw/V-41.md

## Brief Summary

`THORChain_Router.sol` has a `transferAllowance()` function which has the following purpose according to the documentation: >transferAllowance is used during “churns” (i.e. vault rotation). Retiring vaults transfer their allowance to spend ERC20 tokens on the router to the new vaults. If the ERC20s are stored on the router contract on which this function is called then it only adjusts the allowances of the vaults. If another router is specified then it approves it to spend tokens on the behalf of the old vault and calls the `depositWithExpiry()` function on the new router to deposit into the new vault. The issues is that when calling `depositWithExpiry()` on the other router it does not spe...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_20_group

# The router will not work with USDT and KNC tokens

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Submission:** V-43
- **Submitter:** dvrkzy
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-thorchain-validation/issues/43
- **Source snapshot:** competitions/2024-06-thorchain/submissions/raw/V-43.md

## Brief Summary

Thorchain has a whitelist of ERC20 tokens that includes USDT and KNC: They both don't allow approving an amount bigger than 0 when an existing amount which is bigger than 0 is already approved. Instead you have to first reduce the address' allowance to zero. `THORChain_Router.sol` doesn't do that so it will have issues with these tokens. Impact During vault rotations a revert can happen which will prevent a vault from transferring USDT or KNC to a new router. If the retiring vault has to transfer its allowances to a vault that uses the new router it will not be able to.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_02_group

# In `transferOutAndCall` function, ether can be locked forever if the recipient is a contract.

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Submission:** V-176
- **Submitter:** hunter_w3b
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-thorchain-validation/issues/176
- **Source snapshot:** competitions/2024-06-thorchain/submissions/raw/V-176.md

## Brief Summary

The `transferOutAndCall` function is designed to allow a user to send ETH to an aggregator for a token swap. If the swap fails, the function attempts to send the ETH directly to the recipient or back to the sender. However, when the asset is ETH, the function using the low-level `transfer()` function: However, `transfer()` only forwards 2300 gas, which is not enough for the recipient to execute any non-trivial logic in a `receive()` or `fallback` function. For instance, it is not enough for Safes (such as [this one](https://etherscan.io/address/0xd1e6626310fd54eceb5b9a51da2ec329d6d4b68a) in use by the protocol) to receive funds, which require > 6k gas for the call to reach the implementatio...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_01_group

# Inappropriate Use of `strings.EqualFold` for Hash Comparison

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Submission:** V-225
- **Submitter:** hunter_w3b
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-thorchain-validation/issues/225
- **Source snapshot:** competitions/2024-06-thorchain/submissions/raw/V-225.md

## Brief Summary

Using `strings.EqualFold` for comparing hash values can lead to incorrect comparisons. Cryptographic hashes are case-sensitive and should be treated as binary data, not as strings. Since `strings.EqualFold` performs a case-insensitive comparison, it may produce false positives or negatives when comparing hash values. Vulnerability Details The `processReorg` function in the `ethereum_block_scanner` is responsible for handling reorgs. In the event of a reorg, this function checks whether the previous block hash in the block metadata matches the parent hash of the current block. If they match, the function returns without taking any further action. However, the hash comparison is performed usi...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Rely on the token address in the accounting, this can introduce vulnerabilities and complications in the accounting and handling of these tokens, as the same logical token could have different addresses. This situation can lead to inconsistencies, incorrect balances, and potential exploits if the system incorrectly assumes that each address represents a distinct token.

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Submission:** V-65
- **Submitter:** jeremie
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-thorchain-validation/issues/65
- **Source snapshot:** competitions/2024-06-thorchain/submissions/raw/V-65.md

## Brief Summary

When handling ERC20 tokens, it is crucial not to rely solely on the token address for accounting because certain situations can lead to complications: Tokens with Multiple Addresses: Some tokens may exist under multiple addresses due to upgrades, forks, or deployments on different networks or testnets. For example, the same token might have a different address on the mainnet and a testnet, or after a contract upgrade. Potential Issues: Inconsistencies in Balances: Systems that rely only on the token address might not aggregate balances correctly if they consider each address as a distinct token. Potential Exploits: Attackers could exploit these inconsistencies to create situations of double...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# no check if amountOutMin is zero

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Submission:** V-163
- **Submitter:** mgf15
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-thorchain-validation/issues/163
- **Source snapshot:** competitions/2024-06-thorchain/submissions/raw/V-163.md

## Brief Summary

the function `transferOutAndCall` doesn't validated `amountOutMin` , set `amountOutMin` to zero user can lost his tokens .

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# _routerDeposit will fail for USDT

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Submission:** V-172
- **Submitter:** mgf15
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-thorchain-validation/issues/172
- **Source snapshot:** competitions/2024-06-thorchain/submissions/raw/V-172.md

## Brief Summary

When using the approval mechanism in USDT, the approval must be set to 0 before it is updated.the paired asset's approval is not set to 0 before it is updated.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_08_group

# vault.send will return success if sending ETH to vault address that is non-existed contract

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Submission:** V-55
- **Submitter:** mrudenko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-thorchain-validation/issues/55
- **Source snapshot:** competitions/2024-06-thorchain/submissions/raw/V-55.md

## Brief Summary

Disclaimer: this report is succinct, because I am not using ChatGPT to write it. When ACTOR calling function `depositWithExpiry`, if `vault` parameter is passed and this is non-existed address, when ACTOR tries to deposit ETH, `bool success = vault.send(safeAmount);` will return true to success flag, which is incorrect. Actor will loose their money. Issue is happening 2 times in code. The same idea goes with all places where `.send` method is used

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_10_group

# Using memo param as string can cause DoS

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Submission:** V-73
- **Submitter:** mrudenko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-thorchain-validation/issues/73
- **Source snapshot:** competitions/2024-06-thorchain/submissions/raw/V-73.md

## Brief Summary

Bifrost actor can call `transferOutAndCallV5`, `transferOut` and other functions which have `memo` string param. However this param can cause DoS for specific values. If there will be changes in memo notation in future it can cause DoS

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Front-Running Vulnerability in EvilERC20Token Contract

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Submission:** V-220
- **Submitter:** obingo76
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-thorchain-validation/issues/220
- **Source snapshot:** competitions/2024-06-thorchain/submissions/raw/V-220.md

## Brief Summary

This finding indicates a potential front-running vulnerability in the THORChain_Router contract, specifically in the transferOutV5 method. Front-running vulnerabilities can allow attackers to manipulate transaction ordering to their advantage, potentially causing financial losses or other adverse effects. Exploiting this vulnerability could lead to unauthorized transfers of assets or manipulation of contract states.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_40_group

# Failure to Revert on Critical Failures in THORChain_Router Contract

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Submission:** V-252
- **Submitter:** obingo76
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-thorchain-validation/issues/252
- **Source snapshot:** competitions/2024-06-thorchain/submissions/raw/V-252.md

## Brief Summary

The "noRevert" issue indicates that certain functions within the THORChain_Router contract do not revert on failure, potentially allowing unintended behaviours or vulnerabilities to be exploited. This can lead to inconsistent state changes, unexpected financial losses, or unauthorized operations within the contract. Attackers could exploit this behaviour to manipulate balances, allowances, or interact with external contracts without proper validation.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Some functions doesn't have a check for msg.value being 0 in case of ERC20 transfer, which could lead to loss of funds

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Submission:** V-70
- **Submitter:** typicalHuman
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-thorchain-validation/issues/70
- **Source snapshot:** competitions/2024-06-thorchain/submissions/raw/V-70.md

## Brief Summary

Some of the functions can accept both ERC20 and ETH at the same time, but you can send ETH even in the case of an ERC20 transfer, and there's no check to ensure that `msg.value == 0` - this could lead to losing ETH if the user accidentally sends it when trying to send ERC20.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary
