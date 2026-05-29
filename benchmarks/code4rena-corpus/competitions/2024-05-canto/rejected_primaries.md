# Rejected Primary Findings: Canto

# Lack of Validation for Pruning Options

- **Contest:** Canto
- **Slug:** 2024-05-canto
- **Submission:** V-37
- **Submitter:** 0xSergeantPepper
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-canto-validation/issues/37
- **Source snapshot:** competitions/2024-05-canto/submissions/raw/V-37.md

## Brief Summary

In the StartCmdWithOptions function, there is no explicit validation for the custom pruning options provided via flags. Incorrect pruning configurations can lead to unintended data retention or deletion, affecting node performance and data availability. Incorrect pruning configurations could result in excessive disk usage or the unintentional loss of important blockchain data, potentially compromising node operations and historical data integrity.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential Race Conditions in Concurrency Handling

- **Contest:** Canto
- **Slug:** 2024-05-canto
- **Submission:** V-38
- **Submitter:** 0xSergeantPepper
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-canto-validation/issues/38
- **Source snapshot:** competitions/2024-05-canto/submissions/raw/V-38.md

## Brief Summary

The function startStandAlone and startInProcess use goroutines and errgroup without sufficient synchronization mechanisms, which could lead to race conditions and undefined behavior. Race conditions can cause data corruption, unexpected behavior, and crashes, compromising the reliability and stability of the node.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Insufficient Genesis Validation in `govshuttle/module.go`

- **Contest:** Canto
- **Slug:** 2024-05-canto
- **Submission:** V-39
- **Submitter:** 0xSergeantPepper
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-canto-validation/issues/39
- **Source snapshot:** competitions/2024-05-canto/submissions/raw/V-39.md

## Brief Summary

In the ValidateGenesis method of AppModuleBasic, there is a basic check for JSON unmarshalling and validation of genesis state. However, it does not perform in-depth validation on the genesis parameters, such as checking for null or invalid values within the genesis state. Impact This can result in the application starting with an invalid genesis state, leading to unexpected behavior or crashes during runtime.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Lack of Access Control on Module Initialization in `govshuttle/module.go`

- **Contest:** Canto
- **Slug:** 2024-05-canto
- **Submission:** V-40
- **Submitter:** 0xSergeantPepper
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-canto-validation/issues/40
- **Source snapshot:** competitions/2024-05-canto/submissions/raw/V-40.md

## Brief Summary

The InitGenesis method initializes the module's state without performing any access control checks. This can potentially allow unauthorized actors to initialize or manipulate the module's state. Unauthorized initialization or modification of the module's state could lead to security breaches, manipulation of data, and loss of funds.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Lack of Rate Limiting for GRPC Gateway Routes

- **Contest:** Canto
- **Slug:** 2024-05-canto
- **Submission:** V-41
- **Submitter:** 0xSergeantPepper
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-canto-validation/issues/41
- **Source snapshot:** competitions/2024-05-canto/submissions/raw/V-41.md

## Brief Summary

The RegisterGRPCGatewayRoutes method does not implement any rate limiting or throttling mechanism for the gRPC Gateway routes, which can make the application vulnerable to Denial of Service (DoS) attacks. Without rate limiting, an attacker can overwhelm the server with a high volume of requests, leading to resource exhaustion and service unavailability.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Insufficient Validation on Address Setting `govshuttle/keeper/keeper.go`

- **Contest:** Canto
- **Slug:** 2024-05-canto
- **Submission:** V-42
- **Submitter:** 0xSergeantPepper
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-canto-validation/issues/42
- **Source snapshot:** competitions/2024-05-canto/submissions/raw/V-42.md

## Brief Summary

The SetPort function commits the address of the current govShuttle map contract to the state without validating the input address. This can lead to issues if an invalid or malicious address is set. An invalid or malicious address could disrupt the functionality of the govShuttle module, potentially leading to loss of funds or denial of service.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Insufficient Validation of Extension Options in Ante Handler

- **Contest:** Canto
- **Slug:** 2024-05-canto
- **Submission:** V-3
- **Submitter:** Daniel526
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-canto-validation/issues/3
- **Source snapshot:** competitions/2024-05-canto/submissions/raw/V-3.md

## Brief Summary

An attacker could exploit this insufficient validation by crafting transactions with malicious or unexpected extension options that the current ante handler logic does not adequately process. This could potentially lead to the execution of unauthorized actions or bypassing certain transaction validations, compromising the security of the blockchain network.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_06_group

# The NewKeeper function uses a panic statement when the authority address format is invalid. Panicking can crash the application

- **Contest:** Canto
- **Slug:** 2024-05-canto
- **Submission:** V-31
- **Submitter:** MrxSnowden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-canto-validation/issues/31
- **Source snapshot:** competitions/2024-05-canto/submissions/raw/V-31.md

## Brief Summary

The use of panic in the NewKeeper function for handling invalid authority addresses can lead to an application crash. This is particularly dangerous in a production environment, where an unexpected crash can disrupt service, lead to loss of data, and affect user trust. Using panic for error handling is considered a bad practice as it does not allow for graceful error recovery.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# The Sign method in the MsgEthereumTx structure does not verify if the sender address is registered on the keyring before signing the transaction. This omission could lead to unauthorized transactions being signed and executed.

- **Contest:** Canto
- **Slug:** 2024-05-canto
- **Submission:** V-33
- **Submitter:** MrxSnowden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-canto-validation/issues/33
- **Source snapshot:** competitions/2024-05-canto/submissions/raw/V-33.md

## Brief Summary

The Sign method in the MsgEthereumTx structure is responsible for signing Ethereum transactions using the EIP155 standard. However, it does not check if the sender's address is registered on the keyring before proceeding with the signing process. This vulnerability can allow unauthorized addresses to sign transactions, potentially leading to unauthorized access and execution of transactions. An attacker could exploit this vulnerability to sign transactions on behalf of a legitimate user, causing financial loss or unauthorized actions within the system.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Nonce Check in GetSender Function Allows Potential Replay Attacks

- **Contest:** Canto
- **Slug:** 2024-05-canto
- **Submission:** V-34
- **Submitter:** MrxSnowden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-canto-validation/issues/34
- **Source snapshot:** competitions/2024-05-canto/submissions/raw/V-34.md

## Brief Summary

The GetSender function does not include a mechanism to ensure that transactions are unique and have not been previously used. This oversight allows for replay attacks, where an attacker can resubmit the same transaction multiple times. As a result, unauthorized transactions could be executed repeatedly, leading to potential financial losses or other unauthorized actions.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# FeegrantKeeper is not checked if defined

- **Contest:** Canto
- **Slug:** 2024-05-canto
- **Submission:** V-12
- **Submitter:** Sabit
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-canto-validation/issues/12
- **Source snapshot:** competitions/2024-05-canto/submissions/raw/V-12.md

## Brief Summary

The Validate function will not revert when FeegrantKeeper is nil

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# newMsgEthereumTx function does not handle the case where the chainID, gasPrice, or amount parameters are nil

- **Contest:** Canto
- **Slug:** 2024-05-canto
- **Submission:** V-7
- **Submitter:** Sabit
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-canto-validation/issues/7
- **Source snapshot:** competitions/2024-05-canto/submissions/raw/V-7.md

## Brief Summary

newMsgEthereumTx function will not revert when chainID, gasPrice, or amount parameters are nil.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_01_group

# Initialization and Error Handling Issues in CalleeContract Initialization.

- **Contest:** Canto
- **Slug:** 2024-05-canto
- **Submission:** V-15
- **Submitter:** aariiif
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-canto-validation/issues/15
- **Source snapshot:** competitions/2024-05-canto/submissions/raw/V-15.md

## Brief Summary

The `init()` function is commented out, which means that the `CalleeContract` variable is not being initialized with the JSON data from `calleeJSON`. If you uncomment the code in the `init()` function, it will attempt to unmarshal the JSON data into the `CalleeContract` variable. However, if there is an error during the unmarshaling process, it will only print "ERROR HERE" and continue execution without properly handling the error.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Users can profit from `convertCoin` by converting 2 coins of different Values

- **Contest:** Canto
- **Slug:** 2024-05-canto
- **Submission:** V-45
- **Submitter:** forgebyola
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-canto-validation/issues/45
- **Source snapshot:** competitions/2024-05-canto/submissions/raw/V-45.md

## Brief Summary

Users can make profit by using convertCoins with tokenPairs of widely variant values

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_00_group

# Sender is not checked against blacklist

- **Contest:** Canto
- **Slug:** 2024-05-canto
- **Submission:** V-46
- **Submitter:** honeymewn
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-05-canto-validation/issues/46
- **Source snapshot:** competitions/2024-05-canto/submissions/raw/V-46.md

## Brief Summary

Sender account is not checked against blacklist making it possible for malicious users to convert tokens back and forth between cosmos and and erc20. Summary ERC20 module checks whether a token receiver is in the block list using `MintingEnabled` function. However it doesn't do that for senders making it possible for malicious users to transact.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_04_group
