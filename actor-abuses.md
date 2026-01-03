Phase 2 complete: 24 Verified actor exploit!
[2025-11-23T20:09:49Z INFO  ai_agent_audit::llm_review::analysis::code_review_v2] 
    ### Actor Name: Attested DApp using implicit sessions
    
    ### Actor Capability: Trigger `acceptImplicitRequest` callback to validate session context
    
    ### Abuse Title: ERC-1271 Non-Compliance Leading to Integration Denial of Service
    
    ### Scenario: 1. DApp contract calls `isValidSignature` on the Sequence Wallet to verify a signature/permit. 2. The signature is invalid (e.g., expired static signature). 3. `BaseAuth` reverts the transaction instead of returning the ERC-1271 failure magic value or `0x00000000`. 4. The DApp's transaction reverts entirely, preventing it from handling the invalid signature gracefully (e.g., fallback to other logic). 5. User is unable to interact with the DApp using the wallet.
    
    ### Likely Category: GriefableCallbacks
    
    ### Assets at Risk: 
    
    ### Victim: User
    
    
    ### Actor Name: Restricted session key delegate
    
    ### Actor Capability: execute
    
    ### Abuse Title: Session Limit Griefing via Dummy Targets
    
    ### Scenario: A session delegate has a cumulative permission to transfer tokens (e.g., USDC), restricted by an 'Any Target' or a user-controlled target rule. The delegate constructs a payload calling `incrementUsageLimit` for the full allowance, paired with a call to a dummy contract they control (matching the method signature/params). The wallet counts this as valid usage, burning the user's session allowance without actually transferring value to the intended recipient.
    
    ### Likely Category: AccountingInvariantViolation
    
    ### Assets at Risk: 
    
    ### Victim: user
    
    
    ### Actor Name: Checkpointer service syncing cross-chain state
    
    ### Actor Capability: Disable the checkpointing mechanism by publishing a zero imageHash
    
    ### Abuse Title: Compromised Checkpointer Can Enable Replay of Revoked Configurations
    
    ### Scenario: 1. Wallet Owner updates configuration from compromised Config A (Checkpoint 10) to secure Config B (Checkpoint 11).
    2. Owner relies on the Checkpointer to publish a snapshot at Checkpoint 11, ensuring Config A cannot be reused (as `10 < 11`).
    3. An attacker compromises the Checkpointer service and publishes a snapshot with `imageHash = bytes32(0)`.
    4. This specific `imageHash` value acts as an escape hatch in `BaseSig.recover`, bypassing the monotonicity check (`checkpoint <= snapshot.checkpoint`).
    5. The attacker, possessing a signature chain for the compromised Config A, submits a transaction.
    6. The wallet validates the signature despite the lower checkpoint, allowing the attacker to regain control or drain funds using the revoked configuration.
    
    ### Likely Category: AccessControlOrAuthByPass
    
    ### Assets at Risk: User funds, Wallet control
    
    ### Victim: Wallet Owner
    
    
    ### Actor Name: Restricted session key delegate defined in SessionManager
    
    ### Actor Capability: Execute specific function calls on allowed target contracts via `SessionManager`
    
    ### Abuse Title: Privilege Escalation via Missing Function Selector Checks in Explicit Sessions
    
    ### Scenario: 1. Wallet owner creates an Explicit Session for a delegate to interact with a specific Token contract. 2. The permission is configured with `target = TokenAddress` but no `ParameterRule` is added to restrict the function selector (e.g., to only allow `transfer`). 3. The delegate is intended to only transfer tokens, but the configuration effectively grants 'Open' access to the target. 4. The delegate calls `transferFrom` (stealing tokens from the wallet) or `approve` (granting access to an attacker). 5. The `SessionManager` validates the target matches and approves the call.
    
    ### Likely Category: ConfigFootgun
    
    ### Assets at Risk: Wallet ERC20 holdings
    
    ### Victim: Wallet Owner (User/DAO)
    
    
    ### Actor Name: Primary wallet owner with full configuration rights
    
    ### Actor Capability: Execute arbitrary batched calls via `execute` with full access to wallet funds
    
    ### Abuse Title: Wallet Takeover via Malicious Delegate Extension
    
    ### Scenario: 1. Attacker deploys a malicious contract implementing `IDelegatedExtension` that writes to storage slots `0` (implementation) or `IMAGE_HASH_KEY`.
    2. Attacker convinces the wallet owner to sign a payload containing a `delegateCall` to this contract.
    3. The wallet executes the call, and the malicious extension overwrites the wallet's configuration or implementation address.
    4. Attacker gains full control over the wallet.
    
    ### Likely Category: UntrustedDelegateCall
    
    ### Assets at Risk: all wallet funds
    
    ### Victim: User
    
    
    ### Actor Name: ERC-4337 Bundler submitting UserOperations
    
    ### Actor Capability: Trigger `validateUserOp` to ensure gas fees are covered by the wallet
    
    ### Abuse Title: Bundler Griefing via Reverting validateUserOp on Expired Static Signatures
    
    ### Scenario: 1. Attacker configures wallet with a static signature expiring at block T.
    2. Attacker submits a UserOp using this signature at block T-1.
    3. Bundler simulates successfully.
    4. Transaction is mined at block T or later.
    5. `signatureValidation` reverts with `InvalidStaticSignatureExpired`.
    6. `validateUserOp` reverts instead of returning failure code.
    7. Bundler pays gas for the failed transaction.
    
    ### Likely Category: GriefableCallbacks
    
    ### Assets at Risk: MEV
    
    ### Victim: MEV
    
    
    ### Actor Name: Restricted session key delegate
    
    ### Actor Capability: Bypass specific parameter validation if permission rules are configured as 'open'
    
    ### Abuse Title: Session Permission Bypass via Dynamic ABI Encoding Manipulation
    
    ### Scenario: 1. Wallet Owner grants a session key permission to call a function with dynamic parameters (e.g., `multicall(bytes[])` or `execute(target,val,data)`). 2. Owner configures rules to validate the dynamic data (e.g. check first 4 bytes of `data`), calculating offsets based on standard ABI encoding. 3. Delegate constructs a payload calling the function but manipulates the ABI pointer for the dynamic parameter to point to the end of the calldata. 4. Delegate places compliant data at the Owner's expected fixed offset and malicious data at the manipulated pointer location. 5. `PermissionValidator` checks the compliant data at the fixed offset and passes. 6. Target contract decodes and executes the malicious data from the manipulated pointer.
    
    ### Likely Category: AccessControlOrAuthByPass
    
    ### Assets at Risk: vault deposits, protocol treasury
    
    ### Victim: Wallet Owner
    
    
    ### Actor Name: ERC-4337 Bundler
    
    ### Actor Capability: Submit UserOperation bundles triggering executeUserOp
    
    ### Abuse Title: Estimator and Simulator contracts execute arbitrary calls without reverting, allowing theft of held assets
    
    ### Scenario: 1. The `Estimator` and `Simulator` contracts are deployed as public helper contracts to facilitate gas estimation and simulation.
    2. Both contracts inherit `Stage2Module` and override `_isValidImage` to always return `true`, allowing any signature to pass authorization.
    3. Both contracts expose external execution functions (`estimate` and `simulate`) that execute payloads via `_execute` but, unlike standard simulation patterns, do not revert the state changes at the end of the transaction.
    4. If a user accidentally sends assets (ETH/tokens) to these contract addresses, an attacker (or Bundler) can submit a UserOperation (or direct call) with a payload transferring the assets to themselves.
    5. The contract validates the operation (always true) and executes the transfer, permanently draining the funds.
    
    ### Likely Category: AccessControlOrAuthByPass
    
    ### Assets at Risk: funds held by Estimator contract, funds held by Simulator contract
    
    ### Victim: User (who accidentally sends funds to helpers)
    
    
    ### Actor Name: Checkpointer service syncing cross-chain state
    
    ### Actor Capability: Invalidate old wallet configurations on lagging chains by publishing a higher checkpoint
    
    ### Abuse Title: Revoked Signers Exploit Stale State via Checkpointer Inactivity
    
    ### Scenario: 1. Wallet is at Config A. 2. A signer key (S) in Config A is compromised. 3. Owner signs a configuration update A->B (removing S) off-chain but does not execute an on-chain update (relying on chained signatures). 4. The Checkpointer Service fails to publish a snapshot for Config B (due to downtime or collusion). 5. The wallet storage remains at Config A. 6. Attacker uses compromised key S to sign a transaction under Config A. 7. Since the Checkpointer has not invalidated A, the wallet accepts the transaction. 8. Funds are drained despite the owner's attempt to revoke the key.
    
    ### Likely Category: AccessControlOrAuthByPass
    
    ### Assets at Risk: Wallet funds
    
    ### Victim: Wallet Owner (User/DAO)
    
    
    ### Actor Name: Attested DApp using implicit sessions
    
    ### Actor Capability: Execute calls on behalf of wallet using off-chain attestations
    
    ### Abuse Title: Implicit Sessions May Bypass Value Transfer Restrictions
    
    ### Scenario: 1. Protocol invariants state implicit sessions cannot send value.
    2. `SessionManager` executes implicit calls via `_validateImplicitCall`.
    3. Explicit session logic restricts value, but implicit logic branch does not visibly check `call.value`.
    4. If `_validateImplicitCall` lacks this check, a DApp with an implicit session can transfer ETH from the wallet.
    5. `Calls` contract executes the transfer via `LibOptim.call` with value.
    
    ### Likely Category: AccessControlOrAuthByPass
    
    ### Assets at Risk: vault diposits
    
    ### Victim: user
    
    
    ### Actor Name: ERC-4337 Bundler submitting UserOperations
    
    ### Actor Capability: Trigger validateUserOp to ensure gas fees are covered by the wallet
    
    ### Abuse Title: validateUserOp reverts instead of returning failure code on signature validation
    
    ### Scenario: 1. A UserOperation is submitted with a signature that triggers a revert in `BaseAuth` (e.g., expired static signature, insufficient weight).
    2. The EntryPoint calls `validateUserOp` on the wallet.
    3. `validateUserOp` calls `isValidSignature`, which calls `signatureValidation`.
    4. `signatureValidation` reverts due to the invalid signature check.
    5. The revert bubbles up to the EntryPoint, causing the validation transaction to fail entirely instead of returning the `SIG_VALIDATION_FAILED` code as required by ERC-4337.
    6. This can cause the bundler to pay for gas for invalid operations if they passed simulation but fail on-chain (e.g. timestamp expiry).
    
    ### Likely Category: ConfigFootgun
    
    ### Assets at Risk: MEV
    
    ### Victim: ERC-4337 Bundler
    
    
    ### Actor Name: ERC-4337 Bundler
    
    ### Actor Capability: validateUserOp
    
    ### Abuse Title: Bundler Gas Griefing via Reverting Validation
    
    ### Scenario: A malicious user creates a UserOperation with a signature that validates off-chain but reverts on-chain (e.g., a static signature expiring at the exact block time). The wallet's `signatureValidation` reverts with `InvalidStaticSignatureExpired` instead of returning a failure code. The Bundler, having simulated success, includes the op, which reverts on-chain, causing the Bundler to pay gas without compensation.
    
    ### Likely Category: GriefableCallbacks
    
    ### Assets at Risk: 
    
    ### Victim: MEV
    
    
    ### Actor Name: Primary wallet owner with full configuration rights
    
    ### Actor Capability: Upgrade wallet implementation to arbitrary contract address via `updateImplementation`
    
    ### Abuse Title: Exposure of Unprotected Execution via Simulator Implementation Upgrade
    
    ### Scenario: 1. Wallet owner mistakenly calls `updateImplementation` targeting the `Simulator` contract address instead of the standard `Stage2Module`.
    2. The `Simulator` contract inherits wallet logic but exposes a public `simulate` function.
    3. `simulate` accepts arbitrary calls and executes them in the wallet's context without signature verification.
    4. Attacker calls `simulate` on the wallet address to transfer all assets to themselves.
    
    ### Likely Category: ConfigFootgun
    
    ### Assets at Risk: all wallet funds
    
    ### Victim: User
    
    
    ### Actor Name: Social recovery guardian authorized via Recovery extension
    
    ### Actor Capability: Execute a queued payload after the mandatory time delay via `recoverSapientSignatureCompact`
    
    ### Abuse Title: Inconsistent Merkle Root Calculation in Recovery Module causes Verification Failure
    
    ### Scenario: 1. Wallet owner configures the Recovery module with a nested Merkle tree structure (Branch at root).
    2. Off-chain tooling calculates the expected root using standard Merkle logic (`Root = BranchRoot`).
    3. Owner commits this root to the wallet configuration.
    4. Guardian attempts recovery with a valid signature matching the structure.
    5. `Recovery._recoverBranch` incorrectly calculates the root as `Keccak256(0, BranchRoot)` because it fails to handle the empty root case for `FLAG_BRANCH`.
    6. The calculated root mismatches the committed root, causing the recovery transaction to revert and locking the user out.
    
    ### Likely Category: AccountingInvariantViolation
    
    ### Assets at Risk: vault diposits
    
    ### Victim: User (Wallet Owner)
    
    
    ### Actor Name: Restricted session key delegate defined in SessionManager
    
    ### Actor Capability: Execute specific function calls on allowed target contracts via SessionManager
    
    ### Abuse Title: Unhandled behaviorOnError flag allows masking call failures and bypassing fallback logic
    
    ### Scenario: 1. The delegate constructs a batch payload containing a call (e.g., a prerequisite check) and a subsequent dependent call.
    2. The delegate sets the `behaviorOnError` bits in the call flags to `3` (binary `11`). `SessionManager` only forbids `ABORT` (2), allowing 3.
    3. The prerequisite call executes and fails (returns false).
    4. In `Calls.sol`, the error handling logic checks for flags 0, 1, and 2. Flag 3 matches none, so it falls through.
    5. `errorFlag` is not set to true, and `CallSucceeded` is incorrectly emitted.
    6. The subsequent call executes despite the prerequisite failure, or an `onlyFallback` recovery call is skipped because `errorFlag` remains false, violating batch atomicity.
    
    ### Likely Category: ConfigFootgun
    
    ### Assets at Risk: user funds, protocol integrity
    
    ### Victim: Wallet Owner
    
    
    ### Actor Name: Primary wallet owner with full configuration rights
    
    ### Actor Capability: Execute arbitrary batched calls via `execute` with full access to wallet funds
    
    ### Abuse Title: Spoofing successful execution via undefined behaviorOnError flag
    
    ### Scenario: 1. Actor constructs a payload with a call guaranteed to fail (e.g. transfer with insufficient funds).
    2. Actor sets the `behaviorOnError` flag for that call to `3` (binary `11`).
    3. Actor signs and executes the payload.
    4. In `Calls._execute`, the call fails (`success` is false).
    5. The error handling logic checks for defined behaviors (0, 1, 2) but skips `3`.
    6. Execution falls through to `emit CallSucceeded`.
    7. Off-chain indexers and DApps erroneously record the failed transaction as successful.
    
    ### Likely Category: GriefableCallbacks
    
    ### Assets at Risk: 
    
    ### Victim: DApps/Indexers relying on event logs
    
    
    ### Actor Name: Checkpointer service
    
    ### Actor Capability: snapshotFor
    
    ### Abuse Title: Permanent Wallet Lockout via Checkpointer Downtime
    
    ### Scenario: A wallet is configured with a specific Checkpointer address. All valid signatures for this wallet must subsequently include checkpointer data to correctly derive the on-chain `imageHash`. If the Checkpointer service goes offline or refuses to sign, no valid signature can be generated. The user cannot even execute a `ConfigUpdate` to remove the Checkpointer because that update itself requires a valid signature (dependent on the dead Checkpointer), permanently freezing the wallet.
    
    ### Likely Category: UnprotectedPauseOrStop
    
    ### Assets at Risk: vault diposits
    
    ### Victim: user
    
    
    ### Actor Name: Restricted session key delegate
    
    ### Actor Capability: Execute specific function calls on allowed target contracts via `SessionManager`
    
    ### Abuse Title: Indefinite Persistence of Compromised Implicit Session Keys
    
    ### Scenario: 1. User authenticates with an Identity Provider, generating an attestation for a session key. 2. `ImplicitSessionManager` validates the attestation signature but does not enforce an expiration time relative to the attestation's `issuedAt` timestamp. 3. User's device/key is compromised. 4. Attacker uses the stolen session key and original attestation to execute transactions indefinitely, even after the original Identity Provider session has expired. 5. Access persists until the user manually updates the wallet configuration to blacklist the signer.
    
    ### Likely Category: AccessControlOrAuthByPass
    
    ### Assets at Risk: vault deposits
    
    ### Victim: Wallet Owner
    
    
    ### Actor Name: Restricted session key delegate defined in SessionManager
    
    ### Actor Capability: Increment cumulative usage limits for value transfer permissions via `incrementUsageLimit`
    
    ### Abuse Title: Session Usage Limits Persist Across Sessions Leading to DoS
    
    ### Scenario: 1. Wallet Owner creates a session for Delegate D with a cumulative value limit of 1000 USDC.
    2. Delegate D spends 1000 USDC. The `ExplicitSessionManager` records this usage on-chain against a hash of `(Delegate, USDC)`.
    3. The session expires. The Owner issues a *new* session to Delegate D with the same 1000 USDC limit, intending to grant a fresh allowance.
    4. Delegate D attempts to spend 1 USDC.
    5. `PermissionValidator` retrieves the persistent usage (1000) from the previous session.
    6. It calculates `1000 + 1 > 1000`, and the transaction reverts.
    7. The Delegate is unexpectedly locked out of the new session, resulting in a Denial of Service unless the Owner manually increases the limit to account for historical usage.
    
    ### Likely Category: AccountingInvariantViolation
    
    ### Assets at Risk: Operational availability
    
    ### Victim: Session Key Delegate
    
    
    ### Actor Name: Checkpointer service syncing cross-chain state
    
    ### Actor Capability: Disable the checkpointing mechanism by publishing a zero imageHash
    
    ### Abuse Title: Checkpointer Service Can Enable Replay of Stale Configurations on Lagging Chains
    
    ### Scenario: 1. Wallet owner rotates keys from Config A to Config B. L2 wallet remains at Config A (lagging).
    2. Malicious checkpointer publishes a snapshot for L2 with `imageHash = bytes32(0)`.
    3. This value disables the `UnusedSnapshot` verification in `BaseSig.recover`.
    4. Attacker uses a valid signature from Config A (now compromised/old) to execute transactions on L2.
    5. The wallet accepts the signature because it matches the on-chain state (Config A) and the external checkpointer guard is bypassed.
    
    ### Likely Category: BeaconOrFactoryAuthorityDrift
    
    ### Assets at Risk: wallet funds on lagging chains
    
    ### Victim: User
    
    
    ### Actor Name: ERC-4337 Bundler submitting UserOperations
    
    ### Actor Capability: Trigger `validateUserOp` to ensure gas fees are covered by the wallet
    
    ### Abuse Title: ERC-4337 Validation Reverts Cause Bundler DoS and Reputation Loss
    
    ### Scenario: 1. Bundler simulates `validateUserOp` for a Sequence Wallet UserOperation. 2. `validateUserOp` calls `isValidSignature`, which calls `BaseAuth.signatureValidation`. 3. `signatureValidation` reverts (instead of returning a failure code) upon common errors like expired static signatures or insufficient weight. 4. The revert propagates out of `validateUserOp`. 5. The Bundler, receiving a revert instead of the expected `SIG_VALIDATION_FAILED` code, treats the operation as malformed/spam and bans or down-scores the wallet's sender address, denying service to the user.
    
    ### Likely Category: UnprotectedPauseOrStop
    
    ### Assets at Risk: 
    
    ### Victim: User
    
    
    ### Actor Name: Primary wallet owner with full configuration rights
    
    ### Actor Capability: Sign chained configuration updates to change signers, thresholds, or implementation off-chain
    
    ### Abuse Title: Cross-Chain Replay Vulnerability via `noChainId` Signature Flag
    
    ### Scenario: 1. Owner signs a transaction payload with the `noChainId` flag (bit 1 of signature flag) set to true, intended for a specific chain.
    2. The EIP-712 domain separator implies a chainId of 0.
    3. Attacker captures the signature and submits it to the same wallet address on a different chain (address is consistent via CREATE2).
    4. The signature validates successfully on the target chain, executing the transaction and potentially draining funds.
    
    ### Likely Category: ReplayAcrossForksOrL2s
    
    ### Assets at Risk: wallet funds on other chains
    
    ### Victim: User
    
    
    ### Actor Name: Primary wallet owner with full configuration rights
    
    ### Actor Capability: Execute arbitrary batched calls via `execute` with full access to wallet funds
    
    ### Abuse Title: Undefined `behaviorOnError` Value Allows Silent Call Failures and Atomicity Violation
    
    ### Scenario: 1. Attacker (via social engineering) or malicious actor creates a payload where a critical call has `behaviorOnError` set to 3 (binary `11`).
    2. The wallet owner signs and submits this payload.
    3. In `Calls._execute`, the call fails. The error handling logic checks for values 0, 1, and 2, but falls through for 3.
    4. `errorFlag` is not set, and `CallSucceeded` is emitted despite failure.
    5. Subsequent calls execute in an invalid state (e.g., swapping funds that failed to bridge), breaking transaction atomicity.
    
    ### Likely Category: AccountingInvariantViolation
    
    ### Assets at Risk: user funds, protocol consistency
    
    ### Victim: User
    
    
    ### Actor Name: Checkpointer service syncing cross-chain state
    
    ### Actor Capability: Provide state snapshots via `snapshotFor` to enforce checkpoint monotonicity
    
    ### Abuse Title: Checkpointer Service Denial of Service via Future Checkpoints
    
    ### Scenario: 1. Malicious or compromised Checkpointer Service publishes a snapshot with a `checkpoint` value set to `type(uint256).max` and a random `imageHash`. 2. Wallet owner attempts to execute a transaction using their current valid configuration (which has a lower checkpoint). 3. The `BaseSig.recover` logic compares the transaction's checkpoint against the snapshot. 4. Since the transaction checkpoint is lower and the image hashes do not match, the contract reverts with `UnusedSnapshot`. 5. The owner cannot produce a valid signature chain that ends at the random future hash, effectively permanently bricking the wallet.
    
    ### Likely Category: UnprotectedPauseOrStop
    
    ### Assets at Risk: Wallet funds (permanently frozen)
    
    ### Victim: Wallet Owner (User/DAO)
    