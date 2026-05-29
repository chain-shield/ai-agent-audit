# Accepted H/M Findings: zkSync Era

# [H-01] paymaster will refund spentOnPubdata to user

- **Contest:** zkSync Era
- **Slug:** 2024-03-zksync-era
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-03-zksync-era
- **Source snapshot:** competitions/2024-03-zksync-era/final_report.html

paymaster will refund spentOnPubdata to user Submitted by bin2chen A very important modification of this update is that the GAS spent by pubdata is collected at the final step of the transaction.

But if there is a paymaster, when executing paymaster.postTransaction(_maxRefundedGas), _maxRefundedGas does not subtract the spentOnPubdata.

bootloader.yul the code is as follow： function refundCurrentL2Transaction( txDataOffset, transactionIndex, success, gasLeft, gasPrice, reservedGas, basePubdataSpent, gasPerPubdata ) -> finalRefund { setTxOrigin(BOOTLOADER_FORMAL_ADDR()) finalRefund:= 0 let innerTxDataOffset:= add(txDataOffset, 32) let paymaster:= getPaymaster(innerTxDataOffset) let refundRecipient:= 0 switch paymaster case 0 { // No paymaster means that the sender should receive the refund refundRecipient:= getFrom(innerTxDataOffset) } default { refundRecipient:= paymaster if gt(gasLeft, 0) { checkEnoughGas(gasLeft) let nearCallAbi:= getNearCallABI(gasLeft) let gasBeforePostOp:= gas() pop(ZKSYNC_NEAR_CALL_callPostOp( // Maximum number of gas that the postOp could spend

nearCallAbi, paymaster, txDataOffset, success, // Since the paymaster will be refunded with reservedGas, // it should know about it @> safeAdd(gasLeft, reservedGas, "jkl"), basePubdataSpent, reservedGas, gasPerPubdata )) let gasSpentByPostOp:= sub(gasBeforePostOp, gas()) gasLeft:= saturatingSub(gasLeft, gasSpentByPostOp) } // It was expected that before this point various `isNotEnoughGasForPubdata` methods would ensure that the user // has enough funds for pubdata. Now, we just subtract the leftovers from the user.

@> let spentOnPubdata:= getErgsSpentForPubdata( basePubdataSpent, gasPerPubdata ) let totalRefund:= saturatingSub(add(reservedGas, gasLeft), spentOnPubdata) askOperatorForRefund( totalRefund, spentOnPubdata, gasPerPubdata ) let operatorProvidedRefund:= getOperatorRefundForTx(transactionIndex) // If the operator provides the value that is lower than the one suggested for // the bootloader, we will use the one calculated by the bootloader.

let refundInGas:= max(operatorProvidedRefund, totalRefund) // The operator cannot refund more than the gasLimit for the transaction if gt(refundInGas, getGasLimit(innerTxDataOffset)) { assertionError("refundInGas > gasLimit") } if iszero(validateUint32(refundInGas)) { assertionError("refundInGas is not uint32") } let ethToRefund:= safeMul( refundInGas, gasPrice, "fdf" ) directETHTransfer(ethToRefund, refundRecipient) finalRefund:= refundInGas } paymaster's _maxRefundedGas = gasLeft + reservedGas, without subtracting spentOnPubdata.

This way _maxRefundedGas will be much larger than the correct value.

paymaster will refund the used spentOnPubdata to the user.

## Impact

paymaster will refund the spentOnPubdata already used by the user.

## Recommended Mitigation

function refundCurrentL2Transaction( txDataOffset, transactionIndex, success, gasLeft, gasPrice, reservedGas, basePubdataSpent, gasPerPubdata ) -> finalRefund { setTxOrigin(BOOTLOADER_FORMAL_ADDR()) finalRefund:= 0 let innerTxDataOffset:= add(txDataOffset, 32) let paymaster:= getPaymaster(innerTxDataOffset) let refundRecipient:= 0 switch paymaster case 0 { // No paymaster means that the sender should receive the refund refundRecipient:= getFrom(innerTxDataOffset) } default { refundRecipient:= paymaster + let expectSpentOnPubdata:= getErgsSpentForPubdata( + basePubdataSpent, + gasPerPubdata + ) if gt(gasLeft, 0) { checkEnoughGas(gasLeft) let nearCallAbi:= getNearCallABI(gasLeft) let gasBeforePostOp:= gas()

pop(ZKSYNC_NEAR_CALL_callPostOp( // Maximum number of gas that the postOp could spend nearCallAbi, paymaster, txDataOffset, success, // Since the paymaster will be refunded with reservedGas, // it should know about it - safeAdd(gasLeft, reservedGas, "jkl"), + saturatingSub(add(reservedGas, gasLeft), expectSpentOnPubdata), basePubdataSpent, reservedGas, gasPerPubdata )) let gasSpentByPostOp:= sub(gasBeforePostOp, gas()) gasLeft:= saturatingSub(gasLeft, gasSpentByPostOp) } // It was expected that before this point various `isNotEnoughGasForPubdata` methods would ensure that the user // has enough funds for pubdata. Now, we just subtract the leftovers from the user.

let spentOnPubdata:= getErgsSpentForPubdata( basePubdataSpent, gasPerPubdata ) let totalRefund:= saturatingSub(add(reservedGas, gasLeft), spentOnPubdata) askOperatorForRefund( totalRefund, spentOnPubdata, gasPerPubdata ) let operatorProvidedRefund:= getOperatorRefundForTx(transactionIndex) // If the operator provides the value that is lower than the one suggested for // the bootloader, we will use the one calculated by the bootloader.

let refundInGas:= max(operatorProvidedRefund, totalRefund) // The operator cannot refund more than the gasLimit for the transaction if gt(refundInGas, getGasLimit(innerTxDataOffset)) { assertionError("refundInGas > gasLimit") } if iszero(validateUint32(refundInGas)) { assertionError("refundInGas is not uint32") } let ethToRefund:= safeMul( refundInGas, gasPrice, "fdf" ) directETHTransfer(ethToRefund, refundRecipient) finalRefund:= refundInGas }

## Assessed type

Context saxenism (zkSync) confirmed, but disagreed with severity and commented:

We confirm the finding. It is good.

We however believe that this is a medium severity issue since this is a rarely used functionality.

0xsomeone (judge) commented:

The Warden has identified a discrepancy in the way paymaster refunds are processed for L2 transactions, resulting in an over-compensation that overlaps with the gas spent on public data.

The exhibit is correct, and I am not in complete agreement with the Sponsor’s assessment in relation to the submission’s severity. The referenced code will trigger if a paymaster has been defined, and I do not believe there is any constraint that permits a malicious user from always triggering the surplus refund and thus from slowly siphoning funds in the form of gas from the system.

As the flaw is always present and its impact is properly considered medium, I consider the combination of those two factors to merit a high severity rating.

Medium Risk Findings (4)

# [M-01] Freezed Chain will never be unfreeze since StateTransitionManager::unfreezeChain is calling freezeDiamond instead of unfreezeDiamond

- **Contest:** zkSync Era
- **Slug:** 2024-03-zksync-era
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-zksync-era
- **Source snapshot:** competitions/2024-03-zksync-era/final_report.html

StateTransitionManager::unfreezeChain is calling freezeDiamond instead of unfreezeDiamond Submitted by 0x11singh99, also found by oakcobalt, Bauchibred, erebus, XDZIBECX, Dup1337, yashar, bin2chen, forgebyola, Topmark, rvierdiiev, bctester, and zhanmingjing StateTransitionManager::unfreezeChain function is meant for unfreeze the freezed chain of passed _chainId param. While freezeChain function is meant for freeze the chain according to passed _chainId. But freezeChain and unfreezeChain both functions are calling same function freezeDiamond by same line IZkSyncStateTransition(stateTransition[_chainId]).freezeDiamond() by mistake. So both these function will only freeze the chain.

Also there is no other function inside StateTransitionManager.sol contract which is calling unfreezeDiamond.

unfreezeDiamond is function defined in Admin.sol where the call is going since IZkSyncStateTransition also inherits IAdmin which have freezeDiamond and unfreezeDiamond both functions. But unfreezeDiamond is not called from unfreezeChain function. So freezed chain will never be unfreeze.

unfreezeChain also have wrong comment instead of writing unfreezes it writes freezes. It seems like dev just copy pasted without doing required changes.

Vulnerable Code code/contracts/ethereum/contracts/state-transition/StateTransitionManager.sol#L165-L167 159:

/// @dev freezes the specified chain 160:

function freezeChain ( uint256 _chainId ) external onlyOwner { 161:

IZkSyncStateTransition ( stateTransition [ _chainId ]).

freezeDiamond (); } 164:

/// @dev freezes the specified chain 165:

function unfreezeChain ( uint256 _chainId ) external onlyOwner { 166:

IZkSyncStateTransition ( stateTransition [ _chainId ]).

freezeDiamond (); //@audit `freezeDiamond` called instead of `unfreezeDiamond` } (

- https://github.com/code-423n4/2024-03-zksync/blob/main/code/contracts/ethereum/contracts/state-transition/chain-interfaces/IZkSyncStateTransition.sol#L15
) IZkSyncStateTransition is inheriting IAdmin and by IZkSyncStateTransition wrapping instance is prepared to call freezeDiamond.

15:

interface IZkSyncStateTransition is IAdmin, IExecutor, IGetters, IMailbox { IAdmin interfaces have both functions 13:

interface IAdmin is IZkSyncStateTransitionBase {....

52:

/// @notice Instantly pause the functionality of all freezable facets & their selectors /// @dev Only the governance mechanism may freeze Diamond Proxy 54:

function freezeDiamond () external; /// @notice Unpause the functionality of all freezable facets & their selectors /// @dev Both the admin and the STM can unfreeze Diamond Proxy 58:

function unfreezeDiamond () external; It shows that by mistake unfreezeChain is calling freezeDiamond instaed of unfreezeDiamond which should be used to unfreeze the chain.

## Impact

Freezed chain will never be unfreeze. Since freezeChain and unfreezeChain both functions are calling same function freezeDiamond which is used to freeze the chain. And unfreezeDiamond no where called which should is made for unfreeze the freezed chain.

## Recommended Mitigation

In StateTransitionManager::unfreezeChain function call unfreezeDiamond instead of freezeDiamond on IZkSyncStateTransition(stateTransition[_chainId]) instance.

File: code/contracts/ethereum/contracts/state-transition/StateTransitionManager.sol - 164: /// @dev freezes the specified chain + 164: /// @dev unfreezes the specified chain 165: function unfreezeChain(uint256 _chainId) external onlyOwner { - 166: IZkSyncStateTransition(stateTransition[_chainId]).freezeDiamond(); + 166: IZkSyncStateTransition(stateTransition[_chainId]).unfreezeDiamond(); } 0xsomeone (judge) increased severity to High saxenism (zkSync) confirmed, but disagreed with severity and commented:

This is a good finding, but we consider this a medium severity issue because in the current codebase admin could also unfreeze (so no permanent freeze & so not high), but in the future we might wanna change this mechanism.

0xsomeone (judge) decreased severity to Medium and commented:

The submission and its relevant duplicates have identified a mistype in the codebase that causes certain functionality that is expected to be accessible to behave oppositely.

The exhibit represents an actual error in the code resulting in functionality missing, however, the functionality does contain an alternative access path per the Sponsor’s statement and as such I consider this exhibit to be of medium risk.

# [M-02] L2SharedBridge l1LegacyBridge is not set

- **Contest:** zkSync Era
- **Slug:** 2024-03-zksync-era
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-zksync-era
- **Source snapshot:** competitions/2024-03-zksync-era/final_report.html

Submitted by bin2chen, also found by rvierdiiev The migration steps for L1ERC20Bridge/L2ERC20Bridge are as follows:

- https://github.com/code-423n4/2024-03-zksync/blob/main/docs/Protocol%20Section/Migration%20process.md
II. Upgrade L1ERC20Bridge contract Upgrade L2 bridge The new L2ERC20Bridge will upgraded to become the L2SharedBridge, and it will be backwards compatible with all messages from the old L1ERC20Bridge, so we upgrade that first as L1->L2 messages are much faster, and in the meantime we can upgrade the L1ERC20Bridge. The new L2SharedBridge can receive deposits from both the old L1ERC20Bridge and the new L1SharedBridge.

Upgrade L1ERC20Bridge We upgrade the L1ERC20Bridge, and move all ERC20 tokens to the L1SharedBridge.

Since L2ERC20Bridge will be updated first, and then L1ERC20Bridge will be updated, L2SharedBridge needs to be compatible with the old L1ERC20Bridge before L1ERC20Bridge is updated.

So in L2SharedBridge.initialize() we need to set l1LegacyBridge = L1ERC20Bridge and finalizeDeposit() to allow l1LegacyBridge to execute.

But the current implementation doesn’t set l1LegacyBridge, it’s always address(0).

function initialize ( address _l1Bridge, address _l1LegecyBridge, bytes32 _l2TokenProxyBytecodeHash, address _aliasedOwner ) external reinitializer (2) { require ( _l1Bridge != address ( 0 ), "bf" ); require ( _l2TokenProxyBytecodeHash != bytes32 ( 0 ), "df" ); require ( _aliasedOwner != address ( 0 ), "sf" ); require ( _l2TokenProxyBytecodeHash != bytes32 ( 0 ), "df" ); l1Bridge = _l1Bridge; l2TokenProxyBytecodeHash = _l2TokenProxyBytecodeHash; if ( block.

chainid != ERA_CHAIN_ID ) { address l2StandardToken = address ( new L2StandardERC20 { salt:

bytes32 ( 0 )}()); l2TokenBeacon = new UpgradeableBeacon { salt:

bytes32 ( 0 )}( l2StandardToken ); l2TokenBeacon.

transferOwnership ( _aliasedOwner ); } else { require ( _l1LegecyBridge != address ( 0 ), "bf2" ); @> // Missing set l1LegecyBridge？ // l2StandardToken and l2TokenBeacon are already deployed on ERA, and stored in the proxy } The above method just checks _l1LegecyBridge ! = address(0), and does not assign a value, l1LegacyBridge is always adddress(0).

This way any messages sent by the user before update L1ERC20Bridge will fail because finalizeDeposit() will not pass the validation

## Impact

Until L1ERC20Bridge is updated, messages sent by the user will fail.

## Recommended Mitigation

function initialize( address _l1Bridge, address _l1LegecyBridge, bytes32 _l2TokenProxyBytecodeHash, address _aliasedOwner ) external reinitializer(2) {...

if (block.chainid != ERA_CHAIN_ID) { address l2StandardToken = address(new L2StandardERC20{salt: bytes32(0)}()); l2TokenBeacon = new UpgradeableBeacon{salt: bytes32(0)}(l2StandardToken); l2TokenBeacon.transferOwnership(_aliasedOwner); } else { require(_l1LegecyBridge != address(0), "bf2"); + l1LegacyBridge = _l1LegecyBridge // l2StandardToken and l2TokenBeacon are already deployed on ERA, and stored in the proxy }

## Assessed type

Context saxenism (zkSync) confirmed and commented:

We confirm this finding. Thank you:) Just adding a little more context here:

The deposits will fail but user can call claimFailedDeposit to get funds back. We have failed to assign the l1LegacyBridge, but that does not pose a security risk since the l1Bridge should work as expected and therefore, the finalizeDeposit function still has a way to work. However, yes, this is an issue because this breaks our intended behaviour.

0xsomeone (judge) commented:

The Warden has demonstrated how a missing assignment will result in legacy transactions failing to finalize. The impact is constrained to legacy transactions, and as it is possible for users to recover their failed deposits the impact is impermanent resulting in a severity of medium being appropriate.

# [M-03] State transition manager is unable to force upgrade a deployed ST, which invalidates the designed safeguard for ‘urgent high risk situation’

- **Contest:** zkSync Era
- **Slug:** 2024-03-zksync-era
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-zksync-era
- **Source snapshot:** competitions/2024-03-zksync-era/final_report.html

Submitted by oakcobalt

## Impact

State transition manager (STM) will be unable to force upgrade a deployed ST against intended design for ‘urgent high risk situation’.

This invalidates the designed safeguard mechanism of an STM force upgrade and ST.

## Recommended Mitigation Steps

In StateTransitionManager.sol, add a method that can call executeUpgrade() or upgradeChainFromVersion() on a local chain.

saxenism (zkSync) confirmed and commented:

Agree with the finding. Thank you:) 0xsomeone (judge) commented:

The Warden has demonstrated how, in an urgent high-risk situation, a forced upgrade cannot be performed in contradiction with the project’s documentation.

Based on the fact that the vulnerability is correct and would surface in a low-likelihood scenario, a medium-risk assessment is appropriate.

# [M-04] User might be able to double withdraw during migration

- **Contest:** zkSync Era
- **Slug:** 2024-03-zksync-era
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-zksync-era
- **Source snapshot:** competitions/2024-03-zksync-era/final_report.html

Submitted by oakcobalt

## Impact

A user might be able to double withdraw during migration in some edge conditions: (1) if their withdrawal tx is included in a batch number the same or after eraFirstPostUpgradeBatch; (2)And if the user finalizeWithdrawal on the old L1ERC20Bridge.sol before L1ERC20Bridge.sol is upgraded.

## Recommended Mitigation Steps

Consider adding a grace period during and following an upgrade, during which time legacyWithdrawal status will always be checked.

razzorsec (zkSync) confirmed, but disagreed with severity and commented:

Realistically invalid, since we will not finalize the upgrade batch. But an interesting thing to remember, so we would consider this as Low, as it is mostly about managing the server.

0xsomeone (judge) commented:

The Warden specifies that under certain circumstances, it is possible to perform a duplicate withdrawal when the system is in the midst of an upgrade.

The Sponsor specifies that this issue is realistically invalid, however, the Sponsor’s statement relies on being aware of the flaw and acting actively against it (i.e. not finalizing the upgrade batch) which I do not consider adequate justification to lower the severity of this exhibit.

I believe a medium-risk grade is appropriate based on the fact that it illustrates a code flaw that will arise under operations permitted by the smart contracts which we cannot presume the Sponsor was aware of.
