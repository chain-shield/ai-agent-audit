# Benchmark Ground Truth: Nibiru

## Accepted H/M Findings

# Accepted H/M Findings: Nibiru

# [H-01] Vesting account preemption attack preventing future contract deployment

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-11-nibiru
- **Source snapshot:** competitions/2024-11-nibiru/final_report.html

Submitted by gh8eo This vulnerability allows an attacker to preemptively set a target address as a vesting account, permanently blocking contract deployments by Factory contracts or other users to that address. Once the address is marked as a vesting account, any deployment attempt stores the contract bytecode in the state without creating a codeHash, rendering the contract permanently inaccessible.

For example, an attacker could target critical ecosystem addresses, such as those planned for LayerZero or Uniswap, and preemptively mark them as vesting accounts. This would effectively “orphan” the contract bytecode at these addresses, with no way to interact with or access it. The severity is compounded if funds are deployed with the contract, as these would also be irretrievable.

If exploited, this vulnerability allows an attacker to lock up critical addresses by setting them as vesting accounts, resulting in “lost” contracts with unreachable bytecode and permanently inaccessible funds. For ecosystem-critical contracts or high-value deployments, this could disrupt functionality and lead to substantial, irreversible losses.

Impact 1: Although the bytecode has been stored in the state, no codeHash is mapped to the account. This is due to the internal logic in SetAccount, where the codeHash is only set if the account implements EthAccountI. Because the account here is a PermanentLockedAccount, the codeHash is not set. With no mapping between the contract address and the codeHash, the deployed bytecode is permanently inaccessible.

Impact 2: Since the bytecode is permanently inaccessible, it wastes storage space in the state. In this PoC, the Child contract bytecode occupies 1,401 bytes, leading to 1,401 bytes of wasted storage.

Impact 3: When the factory contract deploys a child, it transfers 999 wei to the new contract. To retrieve these funds, the withdraw function on the child contract should be called. However, because no codeHash exists in the state for the child contract’s address, it is inaccessible. As a result, these funds are permanently locked. If this attack targeted large-scale contracts that deploy children with substantial amounts of funds, the potential loss could be significant.

## Recommended mitigation steps

There are two potential approaches for patching this vulnerability: a fundamental patch and a more practical one.

The fundamental approach involves completely separating the Cosmos address system from the EVM address system. Currently, the bytes of an EVM address are directly used as Cosmos addresses, allowing them to share state, which enables this vulnerability. By fully decoupling these address systems, this issue could be prevented entirely. However, this would require a major design overhaul and is thus not a realistic solution.

The more practical approach is to disable the Vesting Account feature at the ante handler level. While this would prevent the use of vesting features, it is likely a necessary trade-off for security reasons.

Unique-Divine (Nibiru) confirmed and commented:

I think this one’s a nice finding. Impact 3 is not so much a factor since you can only do this attack prior to the deployment of a contract. It’s a bit of an edge case because it assumes the deployer’s opting for a deterministic address.

Mitigation for us would be a simple removal of each of the “auth/vesting” transaction messages, because we don’t even use that module and all vesting is managed by Wasm contracts Nibiru mitigated:

PR-2127 - Disabled built in auth/vesting module functionality.

Status:

Mitigation confirmed.

# [H-02] Non-deterministic gas consumption due to shared StateDB pointer in bank keeper affecting consensus

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-11-nibiru
- **Source snapshot:** competitions/2024-11-nibiru/final_report.html

StateDB pointer in bank keeper affecting consensus Submitted by 0x41 An issue exists in Nibiru’s implementation of the bank keeper and its interaction with the EVM’s StateDB. The NibiruBankKeeper maintains a pointer field to StateDB that gets updated during read-only EVM operations (like eth_estimateGas ), which then affects the gas computation of subsequent bank transactions.

The issue arises because the StateDB pointer in NibiruBankKeeper is modified during read-only operations, and the presence or absence of this pointer affects program flow in bank operations through nil checks:

func (bk *NibiruBankKeeper) SyncStateDBWithAccount (ctx sdk.Context, acc sdk.AccAddress) { // If there's no StateDB set, it means we're not in an EthereumTx.

if bk.StateDB == nil { return } //... state updates } This can lead to consensus failures as different nodes may compute different gas amounts for the same transaction (depending on if they previously executed a read only query via RPC), which should never happen.

## Recommended mitigation steps

There are several ways to fix this issue:

Clone the StateDB for read-only operations:

func (k Keeper) EstimateGas (ctx sdk.Context, msg core.Message) ( uint64, error ) { originalStateDB:= k.Bank.StateDB k.Bank.StateDB = originalStateDB.

Copy () defer func () { k.Bank.StateDB = originalStateDB }() //... estimation logic } Use context to pass StateDB instead of keeping it as a field:

type NibiruBankKeeper struct { bankkeeper.BaseKeeper } func (bk *NibiruBankKeeper) SyncStateDBWithAccount ( ctx sdk.Context, stateDB *statedb.StateDB, acc sdk.AccAddress, ) { if stateDB == nil { return } //... state updates } Implement a proper snapshot/restore mechanism:

type BankKeeperState struct { stateDB *statedb.StateDB } func (bk *NibiruBankKeeper) Snapshot () *BankKeeperState { return &BankKeeperState{stateDB: bk.StateDB} } func (bk *NibiruBankKeeper) Restore (state *BankKeeperState) { bk.StateDB = state.stateDB } The solution must ensure:

Deterministic gas computation across all nodes.

Proper isolation between read-only and state-modifying operations.

Lambda (warden) commented:

It is possible that I am missing something obvious and this is indeed invalid, but I still think that this is (was) a valid critical issue that could have caused consensus failure because of non-deterministic gas usage across nodes. While looking at the current Nibiri code, I even noticed that it was fixed in the meantime here. However, this was in a later version than the frozen code, i.e., the frozen commit still had the issue.

berndartmueller (judge) commented:

@Lambda - After a more comprehensive second look, I agree that this is an issue. This finding was initially dismissed as invalid due to the lack of a comprehensive write-up and PoC.

While this issue has been separately discovered by the sponsor, the code in scope of the audit still contained the flaw. Therefore, it’s still a valid submission, with High severity justified, given that it can cause consensus failures.

k-yang (Nibiru) confirmed and commented:

Addressed by

- https://github.com/NibiruChain/nibiru/pull/2173.

Nibiru mitigated:

PR-2165 - Ensure only one copy of StateDB when executing Ethereum txts.

Status:

Mitigation confirmed.

# [H-03] Unlimited Nibi could be minted because evm and bank balance are not synced when staking

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-11-nibiru
- **Source snapshot:** competitions/2024-11-nibiru/final_report.html

Submitted by 0x007 NibiruBankKeeper.SyncStateDBWithAccount function in bank_extension.go is responsible for synchronizing the EVM state database ( StateDB ) with the corresponding bank account balance whenever the balance is updated. However, this function is not invoked by all operations that modify bank balances.

func (bk *NibiruBankKeeper) SyncStateDBWithAccount ( ctx sdk.Context, acc sdk.AccAddress, ) { // If there's no StateDB set, it means we're not in an EthereumTx.

if bk.StateDB == nil { return } balanceWei:= evm.

NativeToWei ( bk.

GetBalance (ctx, acc, evm.EVMBankDenom).Amount.

BigInt (), ) bk.StateDB.

SetBalanceWei (eth.

NibiruAddrToEthAddr (acc), balanceWei) } The following functions call bankKeeper.setBalance, but some do not trigger SyncStateDBWithAccount:

* bankKeeper.addCoins * bankKeeper.SendCoins (✅ Synced) * bankMsgServer.Send * bankKeeper.SendCoinsFromModuleToAccount (✅ Synced) * bankKeeper.SendCoinsFromModuleToModule (✅ Synced) * bankKeeper.SendCoinsFromAccountToModule (✅ Synced) * bankKeeper.InputOutputCoins (❌ Not Synced) * bankMsgServer.MultiSend * bankKeeper.DelegateCoins (❌ Not Synced) * bankKeeper.UndelegateCoins (❌ Not Synced) * bankKeeper.MintCoins (✅ Synced) * bankKeeper.subUnlockedCoins * bankKeeper.SendCoins (✅ Synced) * bankKeeper.InputOutputCoins (❌ Not Synced) * bankKeeper.UndelegateCoins (❌ Not Synced) * bankKeeper.BurnCoins (✅ Synced) * bankKeeper.DelegateCoins (❌ Not Synced) * bankKeeper.DelegateCoinsFromAccountToModule * stakingKeeper.Delegate

* stakingMsgServer.CreateValidator * stakingMsgServer.Delegate * stakingMsgServer.CancelUnbondingDelegation * stakingKeeper.BeginRedelegation * stakingMsgServer.BeginRedelegate The EVM can mint or burn an arbitrary amount of Nibi tokens when the obj.Account.BalanceWei in the StateDB is out of sync. Specifically, the EVM’s SetAccBalance function allows this discrepancy to occur if balances are updated outside of the SyncStateDBWithAccount mechanism.

## Recommended mitigation steps

Make sure EVM statedb is synced for every action that changes bank balances, preferably from setBalance.

k-yang (Nibiru) confirmed and commented:

Agree this is a high risk vulnerability.

Nibiru mitigated:

PR-2142 - Add additional missing bank keeper method overrides to sync with StateDB.

Status:

Mitigation confirmed.

# [H-04] Gas is not consumed when precompile method fail, allowing resource consumption related DOS

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Finding ID:** H-04
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-11-nibiru
- **Source snapshot:** competitions/2024-11-nibiru/final_report.html

Submitted by 0x007

- https://github.com/code-423n4/2024-11-nibiru/blob/main/x/evm/precompile/funtoken.go#L79-L84
- https://github.com/code-423n4/2024-11-nibiru/blob/main/x/evm/precompile/wasm.go#L71-L79
- https://github.com/code-423n4/2024-11-nibiru/blob/main/x/evm/precompile/oracle.go#L60-L64

## Finding description and impact

When a precompile method fails (e.g., due to an error), gas is not consumed as the method returns early before invoking the gas consumption logic. This issue affects all three precompiles in the system:

FunToken Wasm Oracle The lack of gas consumption on failure allows attackers to perform denial-of-service (DoS) attacks by exploiting the failure conditions to consume excessive resources without paying for them. The code snippet below demonstrates the issue:

if err != nil { return nil, err } // Gas consumed by a local gas meter contract.

UseGas (startResult.CacheCtx.

GasMeter ().

GasConsumed ()) Impact:

Resource Consumption without Gas Payment:

Since gas is not consumed on failure, an attacker can repeatedly trigger precompile failures, consuming large amounts of resources without the associated cost.

Potential DoS Attack:

This can lead to a DoS attack, where an attacker fills the block with failed precompile executions, causing network slowdowns, failures, or even halting the chain.

Block Gas Limit Exploitation:

Before the precompile.Run method is called, a small amount of requiredGas is consumed. However, once this is consumed, attackers can continue to use gas at no cost, potentially exhausting the block gas limit.

## Recommended mitigation steps

Use gas before returning the err:

// Gas consumed by a local gas meter contract.

UseGas (startResult.CacheCtx.

GasMeter ().

GasConsumed ()) if err != nil { return nil, err } onikonychev (Nibiru) commented:

Moving contract.UseGas() before the error handling does not make sense. See the reasons below:

User must specify a gas limit when sending an Ethereum tx.

Before precompile execution, an isolated gas meter is created with the specified gas limit.

If, during the execution (could be in the beginning or in the middle), gas consumption exceeds the user-specified gas limit, the gas meter throws an OutOfGas exception and reverts the transaction.

In case of transaction failure (as well as success), the user pays almost 100% of the gas specified in the transaction. This is done to encourage users to be more accurate and to call EstimateGas() before execution. There is a potential 20% refund (see here ), but this is not a significant factor.

Therefore, a potential attacker WILL still pay gas regardless of whether the transaction succeeds or reverts.

berndartmueller (judge) commented:

@onikonychev - due to not consuming the precompile gas in the case of an error, a user can repeatedly call the precompile, have it purposefully error and thus consume more computational resources than the user would be eligible for with the paid gas.

k-yang (Nibiru) confirmed and commented:

Agree it’s a valid issue. Here’s a wasm contract that demonstrates it:

use cosmwasm_std::{ entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, }; use cw2::set_contract_version; use crate::error::ContractError; use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg}; // version info for migration info const CONTRACT_NAME: & str = "crates.io:counter"; const CONTRACT_VERSION: & str = env!

( "CARGO_PKG_VERSION" ); #[cfg_attr(not(feature = "library" ), entry_point)] pub fn instantiate ( deps: DepsMut, _env: Env, info: MessageInfo, _msg: InstantiateMsg, ) -> Result <Response, ContractError> { set_contract_version (deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?; Ok(Response::

new ().

add_attribute ( "method", "instantiate" ).

add_attribute ( "owner", info.sender)) } #[cfg_attr(not(feature = "library" ), entry_point)] pub fn execute ( _deps: DepsMut, _env: Env, _info: MessageInfo, msg: ExecuteMsg, ) -> Result <Response, ContractError> { match msg { ExecuteMsg::WasteGas {} => Err(ContractError::

Std (StdError::

generic_err ( "arbitrary revert".

to_string (), ))), ExecuteMsg::NoGas {} => Ok(Response::

new ().

add_attribute ( "method", "no_gas" )), } //./msg.rs use cosmwasm_schema::{cw_serde}; #[cw_serde] pub struct InstantiateMsg {} #[cw_serde] pub enum ExecuteMsg { WasteGas {}, NoGas {}, } And here’s a script that executes the wasteful gas scenario:

import { HDNodeWallet, JsonRpcProvider, toUtf8Bytes } from "ethers"; import { WastefulGas__factory } from "../../typechain-types"; // connects to local node const jsonRpcProvider = new JsonRpcProvider ( "http://localhost:8545" ); // mnemonic for the HD wallet const mnemonic = "..." const owner = HDNodeWallet.

fromPhrase ( mnemonic, "", "m/44'/118'/0'/0/0" ).

connect ( jsonRpcProvider ) const WASM_CONTRACT_ADDR = process.

argv [ 2 ]; async function main () { // deploy contract const factory = new WastefulGas__factory ( owner ); const attackContract = await factory.

deploy (); await attackContract.

waitForDeployment (); console.

log ( "contract address: ", await attackContract.

getAddress ()) const msgBzNoGas = toUtf8Bytes ( JSON.

stringify ({ "no_gas":

{}, })); // call attack const txNoGas = await attackContract.

attack ( WASM_CONTRACT_ADDR, msgBzNoGas, { gasLimit:

"200000" }); const receiptNoGas = await txNoGas.

wait (); console.

log ( "receiptNoGas: ", receiptNoGas ); const msgBzWasteGas = toUtf8Bytes ( JSON.

stringify ({ "waste_gas":

{ "gas_limit":

0, }, })); // call attack const txWasteGas = await attackContract.

attack ( WASM_CONTRACT_ADDR, msgBzWasteGas, { gasLimit:

"200000" }); const receiptWasteGas = await txWasteGas.

wait (); console.

log ( "receiptWasteGas: ", receiptWasteGas ); } main (); The two attacks should consume the same amount of gas, but the one that errors consumes considerably less gas.

Nibiru mitigated:

PR-2152 - Consume gas before returning error.

Status:

Mitigation confirmed.

# [H-05] Inconsistent state management: ethereumTx StateDB overriding CallContract results

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Finding ID:** H-05
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-11-nibiru
- **Source snapshot:** competitions/2024-11-nibiru/final_report.html

ethereumTx StateDB overriding CallContract results Submitted by 0x007 When a precompile is invoked, the context ( ctx ) is cached, and the state database ( statedb ) commits to this cache, ensuring that precompiles operate with the most up-to-date context and data. During the execution of the precompile, the context can be modified, but these changes are not fully reflected in the state database, except for bank-related modifications.

func (s *StateDB) Commit () error { if s.writeToCommitCtxFromCacheCtx != nil { s.

writeToCommitCtxFromCacheCtx () } return s.

commitCtx (s.

GetEvmTxContext ()) } The snippet above shows that at the end of the transaction, the evmTxCtx is updated to the cached context ( cachedCtx ) before the state changes are committed by the statedb.commitCtx. However, the issue arises because EvmState and Account modifications made within cachedCtx can be overwritten when statedb.commitCtx commits the state changes. This creates a situation where certain state changes, particularly those made by precompiles like FunToken, can be lost or corrupted.

For example, the FunToken precompile may call CallContract and modify EvmState ’s AccState after the account state object has been added to the statedb and dirtied.

## Impact

Unlimited token minting: The state inconsistencies could allow the minting of unlimited FunToken ’s, as state changes made during precompile execution may be overwritten.

State corruption: The precompile could corrupt the state of any contract by exploiting the statedb ’s lack of awareness of the modifications made during precompile execution.

Malicious contract exploits: An attacker could create a malicious ERC20 token, which, when added to FunToken, could leverage the MaliciousERC20.transfer method as a callback to perform arbitrary operations on any contract, including state manipulation.

Locking factories: A lot of factories use create which depends on their nonce being incremented in sequence. If a nonce is reused, the transaction would fail because there’s already a contract where they want to deploy.

## Recommended mitigation steps

Make sure EthereumTx.statedb knows what CallContracts in precompile have done, and it has to work well when reverts occur.

k-yang (Nibiru) disputed and commented:

I don’t agree with the statement:

However, the issue arises because EvmState and Account modifications made within cachedCtx can be overwritten when statedb.commitCtx commits the state changes.

writeToCommitCtxFromCacheCtx() writes the changes from cacheCtx to evmTxCtx, so cacheCtx changes are never lost. And furthermore, s.commitCtx(s.GetEvmTxContext()) writes the stateObject changes from StateDB to evmTxCtx, so evmTxCtx accrues all pending changes.

I firmly believe this is a false positive, but I’m happy to dive deeper into it if the warden can provide a more thorough attack scenario, with actual working code and transactions.

0x007 (warden) commented:

Mitigation confirmed.

# [H-06] Hardcoded gas used in ERC20 queries allows for block production halt from infinite recursion

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Finding ID:** H-06
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-11-nibiru
- **Source snapshot:** competitions/2024-11-nibiru/final_report.html

Submitted by 3docSec, also found by 0x007

- https://github.com/code-423n4/2024-11-nibiru/blob/8ed91a036f664b421182e183f19f6cef1a4e28ea/x/evm/precompile/funtoken.go#L149
- https://github.com/code-423n4/2024-11-nibiru/blob/8ed91a036f664b421182e183f19f6cef1a4e28ea/x/evm/precompile/funtoken.go#L285

## Vulnerability details

The funtoken precompile allows an EVM caller to access information about tokens that coexist in the Cosmos (“coin”) and EVM (“ERC20”) spaces.

Some operations performed by this precompile consist of EVM calls; for example, if we look at the balance method:

File: funtoken.

go 265:

func (p precompileFunToken) balance ( 266: start OnRunStartResult, 267: contract *vm.Contract, 268: ) (bz [] byte, err error ) { --- 285:

erc20Bal, err:= p.evmKeeper.

ERC20 ().

BalanceOf (funtoken.Erc20Addr.Address, addrEth, ctx) 286:

if err != nil { 287:

return 288: } We see that for fetching the EVM info, it calls the evmKeeper.ERC20().BalanceOf function:

File: erc20.

go 125:

func (e erc20Calls) BalanceOf ( 126: contract, account gethcommon.Address, 127: ctx sdk.Context, 128: ) (out *big.Int, err error ) { 129:

return e.

LoadERC20BigInt (ctx, e.ABI, contract, "balanceOf", account) 130: } Which in turn calls LoadERC20BigInt:

File: erc20.

go 222:

func (k Keeper) LoadERC20BigInt ( 223: ctx sdk.Context, 224: abi *gethabi.ABI, 225: contract gethcommon.Address, 226: methodName string, 227: args...any, 228: ) (out *big.Int, err error ) { 229:

res, err:= k.

CallContract ( 230: ctx, 231: abi, 232: evm.EVM_MODULE_ADDRESS, // @audit from 233: &contract, 234:

false, // @audit commit = false 235: Erc20GasLimitQuery, // @audit 100_000 236: methodName, 237: args..., 238: ) 239:

if err != nil { 240:

return nil, err 241: } If we look closely to how this callback to the EVM is done, we see that the gas allowed for this call is hardcoded to 100_000 and is charged only after the call returned.

This is problematic because 100_000 is allocated regardless of the gas limit used to call the funtoken precompile, and this breaks the core invariant of the 63/64 gas allocation that ultimately secures EVM implementation from infinite recursions, which can halt block production and cause the validator to be slashed.

While the balance example was described in detail, the same applies to the Transfer call in sendToBank:

File: funtoken.

go 109:

func (p precompileFunToken) sendToBank ( 110: startResult OnRunStartResult, 111: caller gethcommon.Address, 112: readOnly bool, 113: ) (bz [] byte, err error ) { --- 149:

gotAmount, transferResp, err:= p.evmKeeper.

ERC20 ().

Transfer (erc20, caller, transferTo, amount, ctx) 150:

if err != nil { 151:

return nil, fmt.

Errorf ( "error in ERC20.transfer from caller to EVM account: %w", err) 152: } The Burn call in sendToBank is instead secure because it only applies to ERC20 tokens deployed from Coins whose EVM contract is safe.

## Recommended Mitigation Steps

Consider refactoring the evmKeeper.ERC20().BalanceOf and evmKeeper.ERC20().Transfer calls to accept as argument, and use at most, 63/64 of the EVM gas available.

Unique-Divine (Nibiru) commented:

Please label this as sponsor confirmed. I’ve not yet run the steps to reproduce the error case, but it seems legit from reading the description.

k-yang (Nibiru) confirmed and commented:

Agreed it’s a valid issue.

onikonychev (Nibiru) commented:

This is a great catch, indeed! I was able to crash my localnet with either recursive balanceOf() or transfer().

Nibiru mitigated:

PR-2129 - Resolved an infinite recursion issue in ERC20 FunToken contracts.

Status:

Mitigation confirmed.

Medium Risk Findings (10)

# [M-01] ERC20 transfer fails with non-compliant tokens missing return values

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-nibiru
- **Source snapshot:** competitions/2024-11-nibiru/final_report.html

Submitted by 0x41, also found by ifex445, 0x007, Bauchibred, and 0xaltego The erc20.go file assumes all ERC20 tokens return a boolean value from transfer operations, but some popular tokens like USDT, BNB, and OMG do not (

- https://github.com/d-xo/weird-erc20
). These tokens are valid ERC20s but do not conform to the current standard of returning a boolean.

When attempting to transfer such tokens, UnpackIntoInterface will call Unpack which fails when there is no return data but a return value is expected in the ABI. This causes the transfer to fail with an error, preventing users from transferring otherwise valid tokens through the system.

As noted in the audit scope, handling of “Missing return values” is explicitly in scope for this audit.

## Recommended mitigation steps

Modify the transfer function to consider a transfer successful if:

The token returns true, OR The token returns no value and the transfer didn’t revert.

func (e erc20Calls) Transfer ( contract, from, to gethcommon.Address, amount *big.Int, ctx sdk.Context, ) (balanceIncrease *big.Int, resp *evm.MsgEthereumTxResponse, err error ) { recipientBalanceBefore, err:= e.

BalanceOf (contract, to, ctx) if err != nil { return balanceIncrease, nil, errors.

Wrap (err, "failed to retrieve recipient balance" ) } resp, err = e.

CallContract (ctx, e.ABI, from, &contract, true, Erc20GasLimitExecute, "transfer", to, amount) if err != nil { return balanceIncrease, nil, err } // If there's return data, try to unpack it if len (resp.Ret) > 0 { var erc20Bool ERC20Bool if err:= e.ABI.

UnpackIntoInterface (&erc20Bool, "transfer", resp.Ret); err != nil { return balanceIncrease, nil, err } if !erc20Bool.Value { return balanceIncrease, nil, fmt.

Errorf ( "transfer executed but returned success=false" ) } // No return data = transfer didn't revert, consider it successful recipientBalanceAfter, err:= e.

BalanceOf (contract, to, ctx) if err != nil { return balanceIncrease, nil, errors.

Wrap (err, "failed to retrieve recipient balance" ) } balanceIncrease = new (big.Int).

Sub (recipientBalanceAfter, recipientBalanceBefore) if balanceIncrease.

Sign () <= 0 { return balanceIncrease, nil, fmt.

Errorf ( "amount of ERC20 tokens received MUST be positive: the balance of recipient %s would've changed by %v for token %s", to.

Hex (), balanceIncrease.

String (), contract.

Hex (), ) } return balanceIncrease, resp, nil } This makes the system compatible with both standard ERC20 tokens that return a boolean and non-standard tokens that don’t return a value. The approach is similar to that used by established DeFi protocols that need to handle both types of tokens.

berndartmueller (judge) decreased severity to Low and commented:

As seen in the ERC-20 standard, the transfer function is expected to return a bool. Thus, a token like USDT is, strictly speaking, not an ERC-20 token.

Moreover, USDT in the same form as on ETH Mainnet will likely not exist on Nibiru. The token contract will have to be deployed there and is likely to be deployed with an ERC-20 compliant version that returns a bool from transfer().

For example, USDT on Base does not have this flaw, it has a return value.

Therefore, I’m downgrading this to QA (Low).

Lambda (warden) commented:

The transfer function is expected to return a bool. Thus, a token like USDT is, strictly speaking, not an ERC-20 token.

@berndartmueller - while this is true, there are unfortunately many tokens that do not completely adhere to the standard. While some may change their code for a deployment on Nibiru, others may not in order to have a consistent behaviour across chains and to not introduce any code changes (if they used something like xdeployer or another CREATE2 based factory for consistent addresses across chains, they could not even without changing the address). Of course, it is ultimately up to a project to decide if these tokens should be supported or not, which is why C4 started to ask the sponsors if they want to.

On the audit page, “missing return values” were explicitly marked as in scope, which was my motivation to mark it as Medium.

Issue 14 was also the reason this was kept as Medium; although, the total value of all rebasing tokens across all chains is probably much lower than the one of tokens with missing return values.

berndartmueller (judge) increased severity to Medium and commented:

For consistency across severities, especially with regards to Issue 14, I’m upgrading this issue to Medium severity.

k-yang (Nibiru) acknowledged and commented outside of Github with C4 staff:

Will address it on an as needed basis (i.e., when dApps tell us they need support for non-ERC20 compliant tokens).

# [M-02] Double fee application breaks supply invariant for fee-on-transfer ERC20s

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-nibiru
- **Source snapshot:** competitions/2024-11-nibiru/final_report.html

Submitted by 0x41 The EVM module incorrectly handles fee-on-transfer tokens when converting bank coins back to ERC20s, resulting in unbacked bank coins remaining in circulation. This breaks the intended 1:1 supply tracking invariant between ERC20 tokens and their bank coin representations.

When converting from ERC20 to bank coins via sendToBank, the code correctly accounts for transfer fees by only minting bank coins equal to the amount actually received. However, when converting these bank coins back to ERC20s via convertCoinToEvmBornERC20, the code:

Takes in X bank coins from the user.

Tries to transfer X ERC20 tokens.

Due to fees, only Y tokens are received ( Y < X ).

Only burns Y bank coins.

This creates a discrepancy since the original conversion already accounted for fees. The transfer fees are effectively applied twice:

First fee:

100 ERC20 -> 95 bank coins (correct).

Second fee:

95 bank coins -> ~90.25 ERC20 (and only burn 90.25 bank coins).

This leaves 4.75 unbacked bank coins in circulation (95 - 90.25), as the code only burns what was actually transferred in the second conversion.

The impact is monetary - it creates unbacked bank coins that can be used in the rest of the system but don’t have corresponding ERC20 tokens backing them in the EVM module’s account. Over time, this could lead to significant supply inflation of the bank coin representation.

## Recommended mitigation steps

When converting bank coins back to ERC20s in convertCoinToEvmBornERC20, the code should burn the full input amount of bank coins, not just the amount after fees. This ensures fees are only applied once in the entire conversion cycle.

// In msg_server.go convertCoinToEvmBornERC20:

actualSentAmount, _, err:= k.

ERC20 ().

Transfer ( erc20Addr, evm.EVM_MODULE_ADDRESS, recipient, coin.Amount.

BigInt (), ctx, ) if err != nil { return nil, errors.

Wrap (err, "failed to transfer ERC-20 tokens" ) } // Burn the full input amount, not the amount after fees err = k.Bank.

BurnCoins (ctx, evm.ModuleName, sdk.

NewCoins (coin)) This maintains the supply invariant since:

First conversion:

100 ERC20 -> 95 bank coins (after 5% fee).

Second conversion:

95 bank coins -> ~90.25 ERC20 (after another 5% fee), burn full 95 bank coins.

The documentation should also be updated to explicitly describe how fee-on-transfer tokens are handled and that fees will apply on both conversions.

k-yang (Nibiru) confirmed and commented:

I don’t think it has a monetary impact on the financial ecosystem though because those unburned coins that accumulate at the EVM module address are inaccessible. It’s the logical equivalent of being burned, except that it still shows up in total supply calculations. We’ll fix it for accounting purposes, but it’s not an infinite mint bug where the attacker can actually use the funds.

Nibiru mitigated:

PR-2139 - Ensure bank coins are properly burned after converting back to ERC20.

Status:

Mitigation confirmed.

# [M-03] Gas used mismatch in failed contract calls can lead to wrong gas deductions

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-nibiru
- **Source snapshot:** competitions/2024-11-nibiru/final_report.html

Submitted by 0x41 In call_contract.go, when a contract call fails, the code only consumes gas for the failed transaction but does not account for previously accumulated block gas usage. This creates a mismatch between the actual gas used in the block and what gets consumed in the gas meter.

The issue occurs in the error handling path of CallContractWithInput where after a failed call, only the gas of the failed transaction is consumed:

if evmResp.

Failed () { k.

ResetGasMeterAndConsumeGas (ctx, evmResp.GasUsed) // Only consumes gas for this tx //... error handling } However, in the success path, the code correctly adds the gas used to the block total:

blockGasUsed, err:= k.

AddToBlockGasUsed (ctx, evmResp.GasUsed) //...

k.

ResetGasMeterAndConsumeGas (ctx, blockGasUsed) This inconsistency means that failed transactions do not properly contribute to the block’s gas tracking. This is especially bad when the failed transaction is the last one in a block, as it will decrease the gas counter heavily (only the last transaction vs. the sum of all).

## Recommended mitigation steps

To fix this issue, the gas accounting should be consistent between success and failure paths. Add the block gas tracking before handling the failure case:

// Add to block gas used regardless of success/failure blockGasUsed, err:= k.

AddToBlockGasUsed (ctx, evmResp.GasUsed) if err != nil { k.

ResetGasMeterAndConsumeGas (ctx, ctx.

GasMeter ().

Limit ()) return nil, nil, errors.

Wrap (err, "error adding transient gas used to block" ) } k.

ResetGasMeterAndConsumeGas (ctx, blockGasUsed) if evmResp.

Failed () { if strings.

Contains (evmResp.VmError, vm.ErrOutOfGas.

Error ()) { err = fmt.

Errorf ( "gas required exceeds allowance (%d)", gasLimit) return } //... rest of error handling } This ensures that all gas used, whether from successful or failed transactions, is properly accounted for in the block total.

berndartmueller (judge) commented:

Valid issue! For comparison, Ethermint consumes the cumulative gas for all EVM messages contained in the Cosmos tx (transient gas) -> here.

k-yang (Nibiru) confirmed and commented:

It’s a valid issue but inaccurate.

k.ResetGasMeterAndConsumeGas() sets the gas on the current transaction’s gas meter, so setting it to blockGasUsed is incorrect and might lead to out of gas panics in later txs in a block.

Specifically, the mistake is that we’re not adding to the block gas meter on evm tx failure, and k.ResetGasMeterAndConsumeGas(ctx, N) should only be called with the evm tx gas used.

Lambda (warden) commented:

For audit severity consistency, I’d like to ask for a reconsideration of the severity here. The impact is the same as Issue 25, so I think they should have the same severity.

berndartmueller (judge) commented:

I had a closer look again.

CallContractWithInput() is relevant for precompile calls. Those precompile calls have their own gas meter with a specific gas limit and are executed from within the EVM, which meters the gas and returns the gas used. This evmResp.GasUsed is then added to the transient gas (block gas used, keeping track of the cumulative gas used across all EVM tx’s bundled within a single Cosmos tx).

I conclude from this that contract calls from within precompile calls are always added to the block gas used. Rendering this issue invalid as it does not explain the real issue.

I even think that in the success case, the used gas should not be added to the block gas (in call_contract.go ) and k.ResetGasMeterAndConsumeGas(..) should be called by providing evmResp.GasUsed. Otherwise, in my opinion, it would consider the gas used twice, the second time in EthereumTx(). This matches the sponsor’s comment above.

Inviting @Lambda for comment.

Lambda (warden) commented:

I first want to note that CallContractWithInput is not only used within EthereumTx, but also within CallContract or deployERC20ForBankCoin. Therefore, we would have to do the gas consumption in all consumers (which was not done) based on the evmResp and then we could drop it completely within CallContractWithInput. In the fix, the sponsor now actually did this. In other callers, the return value is now also added to the block gas meter and consumed as well, see example here. However, this was previously not done and it was the responsibility of this function to add to the block gas meter in both cases.

But yes, after looking at it again, I agree that the success path was previously also wrong (depending on the caller), which would have been an issue on its own. This lead to a wrong recommended mitigation from me because I assumed this path was correct. Nevertheless, the issue that was pointed out in the finding description (“This inconsistency means that failed transactions do not properly contribute to the block’s gas tracking”) was actually present and would have lead to wrong gas tracking (for instance, for deployERC20ForBankCoin with failed calls, where the evmResp.gasUsed was not read and added to the block gas).

berndartmueller (judge) commented:

@Lambda - let’s clarify what “block gas” i.e., AddToBlockGasUsed() is used for. It is not used to track/meter cumulative gas used for the Cosmos block. Instead, it accumulates the gas that is used by EVM messages that are bundled within a single Cosmos SDK tx. The meter is reset in the ante handler for each Cosmos tx. “Block gas” is a rather lousy name for this. “Transient gas” meter would be more appropriate. Overall, this meter is used to make sure that the total gas used by a Cosmos tx that includes one or multiple EVM transactions is correctly accounted and added to the actual block gas meter. Also important to know that to ensure EVM gas equivalence, an infinite gas meter is used for the overall Cosmos tx. The gas limit is then enforced per EVM tx.

We have to consider this potential issue within two different contexts:

Within a precompile called from within an EVM tx (potentially multiple EVM transactions bundled in a Cosmos tx).

Within a regular Cosmos tx.

For 1, the precompile call has its own gas meter. Gas usage is bubbled up and handled appropriately in EthereumTx(). Therefore, it’s not necessary that within the precompile, i.e., within CallContractWithInput(), gas is added to the transient gas meter by calling AddToBlockGasUsed().

Can we agree on that?

For 2, the transient gas meter is not relevant. A regular Cosmos tx has a gas meter with the limit set to the limit provided by the tx sender. This gas meter is used for all messages contained in that tx.

Basically, this acts as a “transient” gas meter. Therefore, it’s also not an issue. It even seems as if the EVM’s transient gas meter is counterproductive and even problematic. It’s not reset for each Cosmos SDK tx in the ante handler, it accumulates across them, causing issues at some point. @k-yang, did you consider this?

That’s why I think the reported issue is not an actual issue. Please let me know what you think, happy to hear your thoughts!

Lambda (warden) commented:

Regarding 1, agree, this was apparently another issue that the gas was added twice in this codepath.

Regarding 2, for regular Cosmos TX that contains some EVM calls (such as deployERC20ForBankCoin ), we still need to add the gas used by the EVM execution to the Cosmos gas meter, right? Here, you have two approaches: Either you bubble it up and add it there (this is the approach that the sponsor seems to take in the new PR for all consumers of CallContractWithInput ) or you do it directly in CallContractWithInput (this was done in the in-scope code).

Moreover, when multiple messages are contained in a TX, you could add up the gas for the EVM calls and add it in the end or add it directly. In the in-scope code, both were done*, depending on if the call was successful or not. This still seems wrong to me, I do not see any reason why the gas accounting logic should be handled differently in the success or failure path (which EthereumTx does not as well), i.e., the underlying issue of this report.

* Note:

It seems like it was indeed done incorrectly after looking at it in more detail, as the transient gas meter is not reset in the ante handler for Cosmos SDK txs. But I think this is a different problem with a different underlying issue (the underlying issue being that it was not reset) and it seems natural to assume that the code was written with the intended functioning that it should add to the transient gas meter there; otherwise, this code would not have been in the success path.

berndartmueller (judge) commented:

I now agree that within regular Cosmos TXs, it must track the cumulative EVM gas used across all batched Cosmos messages (which invoke an EVM call) and use it with ResetGasMeterAndConsumeGas(..).

Otherwise, as it is currently the case, ResetGasMeterAndConsumeGas(..) will incorrectly reset the Cosmos TX’s gas meter to the currently failed msg’s consumed gas ( evmResp.Failed() ) or gas limit ( err != nil ), ignoring the gas consumed by the preceding messages (in the same TX). As a result, the block gas meter will be inaccurate and increased by less gas than actually consumed.

k-yang (Nibiru) commented:

Sorry for the late reply, but our code doesn’t allow for mixing EVM msgs and Cosmos msgs in the same Cosmos tx, see:

- https://github.com/code-423n4/2024-11-nibiru/blob/main/app/ante.go#L32-L47
- https://github.com/code-423n4/2024-11-nibiru/blob/main/app/evmante/evmante_validate_basic.go#L95-L101
If a user wants to submit an EVM tx, it will contain the /eth.evm.v1.ExtensionOptionsEthereumTx extension option and EthValidateBasicDecorator will ensure that every bundled msg in the Tx is of type evm.MsgEthereumTx. So that eliminates the case where there are Cosmos and EVM msgs bundled in the same Cosmos Tx.

I agree that BlockGasUsed is a lousy name for the field. It should be CumulativeGasUsedInTransaction or something along those lines.

I see the warden’s point for how, in a bundle of EVM msgs in a tx, if the last EVM msg fails, then we reset the entire tx’s gas meter and only consume the gas used of the last EVM msg. Now, we have removed the BlockGasUsed gas meter and switched to a model where each and every EVM msg adds to the gas meter (think vector addition), instead of always resetting and adding the cumulative gas used to the gas meter (which seemed redundant and more complicated than it needs to be).

See this fix for the removal of the BlockGasUsed gas meter.

Nibiru mitigated:

PR-2132 - Proper tx gas refund outside of CallContract.

Status:

Mitigation confirmed.

# [M-04] Gas refunds use block gas instead of transaction gas, leading to incorrect refund amounts

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-nibiru
- **Source snapshot:** competitions/2024-11-nibiru/final_report.html

Submitted by 0x41, also found by ABAIKUNANBAEV and Sentryx There is a mismatch between how gas fees are deducted and refunded in the EVM implementation:

In evmante_gas_consume.go, gas fees are deducted upfront based on each transaction’s individual gas limit.

However, the refund calculation in msg_server.go uses the cumulative block gas usage to determine refunds for individual transactions.

This mismatch means users will receive incorrect (lower) refunds than they should. The gas refund should be based on the difference between a transaction’s gas limit (what was charged) and its actual gas usage (what was consumed), not the block’s total gas usage.

The impact is that users will lose funds as they receive smaller refunds than they should. This becomes especially problematic when multiple transactions are included in a block, as the cumulative block gas increases with each transaction, reducing refunds for subsequent transactions.

## Recommended mitigation steps

The refund calculation should be based on each transaction’s individual gas usage rather than the block gas. Modify the refund logic in msg_server.go:

// Before blockGasUsed, err:= k.

AddToBlockGasUsed (ctx, evmResp.GasUsed) refundGas:= uint64 ( 0 ) if evmMsg.

Gas () > blockGasUsed { refundGas = evmMsg.

Gas () - blockGasUsed } // After refundGas:= uint64 ( 0 ) if evmMsg.

Gas () > evmResp.GasUsed { refundGas = evmMsg.

Gas () - evmResp.GasUsed } blockGasUsed, err:= k.

AddToBlockGasUsed (ctx, evmResp.GasUsed) This ensures that each transaction’s refund is calculated based on its own gas limit and usage, independent of other transactions in the block.

ABAIKUNANBAEV (warden) commented:

@berndartmueller, I believe this should be of high severity as the issue deals with refunds and therefore, losing of funds. There are several DOS-related issues marked as high but this one with the actual funds losing is not.

berndartmueller (judge) commented:

@ABAIKUNANBAEV, sticking with Medium as this affects individual users, users who batch multiple EVM tx’s within a single Cosmos SDK tx. And batch EVM messages are not that common currently. Thus, I think Medium is justified.

k-yang (Nibiru) disputed and commented:

Please see my comment on Issue 46.

BlockGasUsed was actually a terrible name for the gas tracker variable. It’s actually the cumulative amount of gas used in the TX when multiple EVM msgs are bundled in it. It gets reset between TXs by the evm ante handler.

Since that’s the case, the gas refunded here is correct. The blockGasUsed variable is actually the total amount of gas used by the entire TXs, summing up all the individually bundled EVM msgs.

Nibiru mitigated:

PR-2132 - Proper tx gas refund outside of CallContract.

Status:

Mitigation confirmed.

# [M-05] Inconsistent fee denomination handling in transaction validation and building

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-nibiru
- **Source snapshot:** competitions/2024-11-nibiru/final_report.html

Submitted by 0x41 The Nibiru EVM module incorrectly handles fee denominations during transaction validation and building, failing to convert wei amounts to the native unibi denomination. This can lead to significant discrepancies in fee calculations and potentially allow users to pay far fewer fees than intended.

The issue occurs in two locations:

Transaction validation in evmante_validate_basic.go.

Transaction building in msg.go.

In both cases, fees that are calculated in wei (1e18 units) are directly used with evm.EVMBankDenom (unibi), without converting from wei to unibi (1e6 units). This causes an undervaluation of fees by a factor of 10^12, as the system expects fees in unibi but receives them in wei.

## Recommended mitigation steps

In evmante_validate_basic.go, modify the fee validation to convert from wei to unibi:

txFee = txFee.

Add ( sdk.Coin{ Denom: evm.EVMBankDenom, Amount: sdkmath.

NewIntFromBigInt (evm.

WeiToNative (txData.

Fee ())), }, ) In msg.go, update the fee conversion in BuildTx:

feeAmt:= sdkmath.

NewIntFromBigInt (evm.

WeiToNative (txData.

Fee ())) if feeAmt.

Sign () > 0 { fees = append (fees, sdk.

NewCoin (evmDenom, feeAmt)) } k-yang (Nibiru) confirmed Unique-Divine (Nibiru) commented:

Note that a gas price of 1 wei isn’t possible. The smallest unit of funds that can be transferred is 10^{12} wei, or 1 unibi, meaning the 21000unibi value for fees paid what’s wanted here.

Digging into this now to see which parts are correct or incorrect between msg.go and evmante_validate_basic.go.

Nibiru mitigated:

PR-2157 - Fixed unit inconsistency related to AuthInfo.Fee and txData.Fee.

Status:

Mitigation confirmed.

# [M-06] RPC DOS via TraceTx

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-nibiru
- **Source snapshot:** competitions/2024-11-nibiru/final_report.html

TraceTx Submitted by gxh191 The TraceTx method in x/evm/keeper/grpc_query.go implements a gRPC query interface that allows simulation and tracing of specific transactions based on provided configurations. This method enables users to perform detailed execution simulations for transactions in a block.

However, a DOS issue arises during the simulation of predecessor transactions.

# [M-07] Nonce can be manipulated by inserting a contract creation EthereumTx message first in an SDK TX with multiple EthereumTX messages

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-nibiru
- **Source snapshot:** competitions/2024-11-nibiru/final_report.html

EthereumTx message first in an SDK TX with multiple EthereumTX messages Submitted by Sentryx The Ante handler for MsgEthereumTx transactions is responsible for ensuring messages are coming with correct nonces. After doing so, it’ll increment the account’s sequences for each message that the account has signed and broadcasted.

The problem is that when the EthereumTx() message server method calls ApplyEvmMsg() it’ll override the account nonce when the currently processed EVM transaction message is a contract creation and will set it to the nonce of the message. When a non-contract creation EVM transaction message is processed, however, the ApplyEvmMsg() method does not touch the account’s nonce.

This opens up an exploit window where a malicious user can replay a TX multiple times and reuse their nonces. Users can manipulate their Sequence (nonce) by submitting a contract creation EVM transaction message and multiple call/transfer EVM transaction messages in a single SDK transaction.

The code relies on evmObj.Call() and later stateDB.commitCtx() to persist a correct nonce in state but the Call() method on evmObj does not handle account nonces, it just executes the transaction. As we can see the method in geth that’s normally used to transition the state increments the sender’s nonce by 1 in either case:

- https://github.com/NibiruChain/go-ethereum/blob/nibiru/geth/core/state_transition.go#L331-L337
if contractCreation { ret, _, st.gas, vmerr = st.evm.

Create (sender, st.data, st.gas, st.value) } else { // Increment the nonce for the next transaction st.state.

SetNonce (msg.

From (), st.state.

GetNonce (sender.

Address ())+ 1 ) ret, st.gas, vmerr = st.evm.

Call (sender, st.

to (), st.data, st.gas, st.value) }

- https://github.com/NibiruChain/go-ethereum/blob/nibiru/geth/core/vm/evm.go#L498-L501
func (evm *EVM) Create (caller ContractRef, code [] byte, gas uint64, value *big.Int) (ret [] byte, contractAddr common.Address, leftOverGas uint64, err error ) { contractAddr = crypto.

CreateAddress (caller.

Address (), evm.StateDB.

GetNonce (caller.

Address ())) return evm.

create (caller, &codeAndHash{code: code}, gas, value, contractAddr, CREATE) }

- https://github.com/NibiruChain/go-ethereum/blob/7fb652f186b09b81cce9977408e1aff744f4e3ef/core/vm/evm.go#L405-L418
func (evm *EVM) create (caller ContractRef, codeAndHash *codeAndHash, gas uint64, value *big.Int, address common.Address, typ OpCode) ([] byte, common.Address, uint64, error ) { // Depth check execution. Fail if we're trying to execute above the // limit.

if evm.depth > int (params.CallCreateDepth) { return nil, common.Address{}, gas, ErrDepth } if !evm.Context.

CanTransfer (evm.StateDB, caller.

Address (), value) { return nil, common.Address{}, gas, ErrInsufficientBalance } nonce:= evm.StateDB.

GetNonce (caller.

Address ()) if nonce+ 1 < nonce { return nil, common.Address{}, gas, ErrNonceUintOverflow } evm.StateDB.

SetNonce (caller.

Address (), nonce+ 1 ) But ApplyEvmMsg() calls evmObj.Call() ( st.evm.Call() in the above code snippet) directly and does not increment sender’s nonce:

if contractCreation { // take over the nonce management from evm:

// - reset sender's nonce to msg.Nonce() before calling evm.

// - increase sender's nonce by one no matter the result.

stateDB.

SetNonce (sender.

Address (), msg.

Nonce ()) ret, _, leftoverGas, vmErr = evmObj.

Create ( sender, msg.

Data (), leftoverGas, msgWei, ) stateDB.

SetNonce (sender.

Address (), msg.

Nonce ()+ 1 ) } else { ret, leftoverGas, vmErr = evmObj.

Call ( sender, *msg.

To (), msg.

Data (), leftoverGas, msgWei, ) }

## Recommended mitigation steps

Set the sender’s nonce to msg.Nonce() + 1 when contractCreation is false.

berndartmueller (judge) commented:

Initially, I assumed that only a single EVM message is supported, due to MsgEthereumTx.GetMsgs() returning only a single message.

- https://github.com/code-423n4/2024-11-nibiru/blob/8ed91a036f664b421182e183f19f6cef1a4e28ea/x/evm/msg.go#L190-L193
190:

// GetMsgs returns a single MsgEthereumTx as sdk.Msg.

191:

func (msg *MsgEthereumTx) GetMsgs () []sdk.Msg { 192:

return []sdk.Msg{msg} 193: } However, that’s not the correct GetMsgs(). In fact, multiple EVM messages within a single Cosmos tx are supported. As a result, the demonstrated issue is valid, allowing replaying a user’s EVM messages in this specific scenario. That’s a great catch!

k-yang (Nibiru) confirmed and commented:

Agree it’s a valid issue.

onikonychev (Nibiru) commented:

@berndartmueller - and this is true so far. That’s why we have a separate Ante handler which does not allow MsgEthereumTx within Cosmos TXs:

- https://github.com/NibiruChain/nibiru/blob/main/app/ante/reject_ethereum_tx_msgs.go
I don’t think there is a backdoor for sending a bulk of MsgEthereumTx.

Unique-Divine (Nibiru) disputed and commented:

Rebuttal to the Proposed Vulnerability I’d argue the side of sponsor-disputed here, similar to @onikonychev. When every transaction goes through execution via the DeliverTx ABCI (application blockchain interface) method. The implementation details of this function can be seen in BaseApp.runTx from the Cosmos-SDK baseapp/bapp.go code that, as it implements much of the non-consensus side of the ABCI.

In BaseApp.runTx, it first gets each message and calls ValidateBasic on them, then it enters the BaseApp.anteHandler before beginning any other logic with the tx msgs.

Note: to view the provided image, please see the original comment here.

The Nibiru ante handler, instantiated with NewAnteHandler in Nibiru/app/ante.go splits a tx down different paths of potential ante handlers.

You can see below that, if a tx ( sdk.Tx ) comes in and has the properties indicating it’s an sdk.Tx that is also a MsgEthereumTx, then it passes through an EVM ante handler ( evmante.NewAnteHandlerEVM ).

If however, the sdk.Tx is anything else, it goes through a non-EVM ante handler ( NewAnteHandlerNonEVM ), which you’ll see has the very first sdk.ChainAnteDecorator hook as a blocker that disallows the execution of the sdk.Tx if any of the contained messages are evm.MsgEthereumTx instances.

Note:

Thus, it is not possible to insert a contract creation EthereumTx message in a non-EVM sdk.Tx with multiple EthereumTx messages because that type of tx would be rejected in the ante handler, even if there was only one MsgEthereumTx contained in the sdk.Tx // NewAnteHandler returns and AnteHandler that checks and increments sequence // numbers, checks signatures and account numbers, and deducts fees from the // first signer.

func NewAnteHandler ( keepers AppKeepers, options ante.AnteHandlerOptions, ) sdk.AnteHandler { return func ( ctx sdk.Context, tx sdk.Tx, sim bool, ) (newCtx sdk.Context, err error ) { if err:= options.

ValidateAndClean (); err != nil { return ctx, err } var anteHandler sdk.AnteHandler txWithExtensions, ok:= tx.(authante.HasExtensionOptionsTx) if ok { opts:= txWithExtensions.

GetExtensionOptions () if len (opts) > 0 { switch typeURL:= opts[ 0 ].

GetTypeUrl (); typeURL { case "/eth.evm.v1.ExtensionOptionsEthereumTx":

// handle as *evmtypes.MsgEthereumTx anteHandler = evmante.

NewAnteHandlerEVM (options) default:

return ctx, fmt.

Errorf ( "rejecting tx with unsupported extension option: %s", typeURL) } return anteHandler (ctx, tx, sim) } switch tx.(type) { case sdk.Tx:

anteHandler = NewAnteHandlerNonEVM (options) default:

return ctx, fmt.

Errorf ( "invalid tx type (%T) in AnteHandler", tx) } return anteHandler (ctx, tx, sim) } Why is this confusing at first glance?

The term transaction is overused in Web3 and means different things in the EVM context than in the ABCI/Cosmos-SDK context. An Ethereum tx can contain several Ethereum txs inside it. In other words, what’s called a tx msg or simply a “message” in the ABCI is what’s analogous to the idea of an Ethereum Tx, not the sdk.Tx.

MsgEthereumTx is meant to represent a true “Ethereum tx” like on the Eth L1. That means, a MsgEthereumTx may induce one or more inner Ethereum txs; however, they’ll only be still only occur within that single MsgEthereumTx.

berndartmueller (judge) commented:

@Unique-Divine - this issue is about multiple EVM messages within a single Cosmos TX. It’s not mixing EVM and regular msg’s; therefore, the EVM ante handler is used. It can, however, be argued on the severity.

flacko (warden) commented:

Mitigation confirmed.

# [M-08] Nibiru’s bank coin to EVM balance tracking logic is completely broken for rebasing tokens and would lead to leakage/loss of funds when converting

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Finding ID:** M-08
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-nibiru
- **Source snapshot:** competitions/2024-11-nibiru/final_report.html

Submitted by Bauchibred, also found by Sentryx and 0x007 The Nibiru EVM module’s token conversion mechanism contains a critical vulnerability when handling rebasing tokens. The issue stems from an incorrect assumption about the 1:1 relationship between escrowed ERC20 tokens and their bank coin representations, which can be violated when token balances change outside of transfers (e.g., through rebasing) and these type of tokens are supported by Nibiru.

Context The Nibiru EVM module supports converting ERC20 tokens to bank coins and vice versa. When converting from ERC20 to bank coins, the tokens are escrowed in the EVM module, and when converting back, these escrowed tokens are used to fulfill the conversion.

This logic can be seen in the ConvertCoinToEvm, convertCoinToEvmBornERC20 and convertEvmToCoin functions in the msg_server.go file, see x/evm/keeper/msg_server.go#L486-L561.

Problem The conversion mechanism assumes a static 1:1 relationship between escrowed ERC20 tokens and bank coins, as evidenced in the convertCoinToEvmBornERC20 function that is used when converting the bank coins back to its ERC20 representation:

See here:

func (k Keeper) convertCoinToEvmBornERC20 ( ctx sdk.Context, sender sdk.AccAddress, recipient gethcommon.Address, coin sdk.Coin, funTokenMapping evm.FunToken, ) (*evm.MsgConvertCoinToEvmResponse, error ) { erc20Addr:= funTokenMapping.Erc20Addr.Address // 1 | Caller transfers Bank Coins to be converted to ERC20 tokens.

if err:= k.Bank.

SendCoinsFromAccountToModule ( ctx, sender, evm.ModuleName, sdk.

NewCoins (coin), ); err != nil { return nil, errors.

Wrap (err, "error sending Bank Coins to the EVM" ) } // 2 | EVM sends ERC20 tokens to the "to" account.

// This should never fail due to the EVM account lacking ERc20 fund because // the an account must have sent the EVM module ERC20 tokens in the mapping // in order to create the coins originally.

// // Said another way, if an asset is created as an ERC20 and some amount is // converted to its Bank Coin representation, a balance of the ERC20 is left // inside the EVM module account in order to convert the coins back to // ERC20s.

actualSentAmount, _, err:= k.

ERC20 ().

Transfer ( erc20Addr, evm.EVM_MODULE_ADDRESS, recipient, coin.Amount.

BigInt (), ctx, ) if err != nil { return nil, errors.

Wrap (err, "failed to transfer ERC-20 tokens" ) } // 3 | In the FunToken ERC20 → BC conversion process that preceded this // TxMsg, the Bank Coins were minted. Consequently, to preserve an invariant // on the sum of the FunToken's bank and ERC20 supply, we burn the coins here // in the BC → ERC20 conversion.

burnCoin:= sdk.

NewCoin (coin.Denom, sdk.

NewIntFromBigInt (actualSentAmount)) err = k.Bank.

BurnCoins (ctx, evm.ModuleName, sdk.

NewCoins (burnCoin)) if err != nil { return nil, errors.

Wrap (err, "failed to burn coins" ) } // Emit event with the actual amount received _ = ctx.

EventManager ().

EmitTypedEvent (&evm.EventConvertCoinToEvm{ Sender: sender.

String (), Erc20ContractAddress: funTokenMapping.Erc20Addr.

String (), ToEthAddr: recipient.

String (), BankCoin: burnCoin, }) return &evm.MsgConvertCoinToEvmResponse{}, nil } Evidently, Nibiru makes a critical assumption (invariant) about token availability as shown in the snippet above.

// This should never fail due to the EVM account lacking ERc20 fund because // the an account must have sent the EVM module ERC20 tokens in the mapping // in order to create the coins originally.

// // Said another way, if an asset is created as an ERC20 and some amount is // converted to its Bank Coin representation, a balance of the ERC20 is left // inside the EVM module account in order to convert the coins back to // ERC20s.

However, this assumption would be incorrect for some supported tokens like rebasing tokens, which have been hinted to be used by Nibiru as shown in the README:

README.md#L129-L137 This is because for tokens that have their balance changes not necessarily through transfers, and are rebasing in nature there would be multiple rebases while the tokens are escrowed after the initial conversion from ERC20 to Bank Coin; which would then mean that by the time there is an attempt to convert the tokens back to ERC20, the balance of the tokens in the escrow would have changed (positively/negatively) completely sidestepping the invariant of 1:1 relationship between escrowed ERC20 tokens and their bank coin representations.

## Impact

This bug case completely breaks the subtle invariant of 1:1 relationship between escrowed ERC20 tokens and their bank coin representations. In our case, the issue manifests itself in the following two scenarios:

If cumulatively, the rebases that occur since the initial conversion from ERC20 to Bank Coin are positive, then the difference between the amount of escrowed tokens and that of bank coins would be stuck in the escrow.

Alternatively, if cumulatively, the rebases that occur since the initial conversion from ERC20 to Bank Coin are negative, then the escrow balance would be insufficient to fulfill the conversion, causing the transaction to revert with insufficient balance errors.

Tools Used Similar issue from the Q2 Thorchain Contest on Code4rena here and here.

Nibiru’s documentation on intended to-be integrated tokens Weird ERC20 tokens documentation

## Recommended Mitigation Steps

Consider not supporting these type of tokens at all or instead provide a mechanism to handle balance changes in the escrow/EVM module.

berndartmueller (judge) decreased severity to Medium:

Unique-Divine (Nibiru) disputed and commented:

This function uses the actually transferred amount by querying the ERC20 balance before and after the transfer ( actualSentAmount ). There is not a 1 to 1 assumption like the issue says.

We already addressed this potential issue in the Zenith audit.

berndartmueller (judge) commented:

Balance changes outside of transfers The “Balance changes outside of transfers” behavior is explicitly marked as in-scope in the audit readme. Therefore, I consider this to be a valid issue.

Bauchibred (warden) commented:

Wouldn’t this be categorised as 3 rating since there is a direct impact on assets?

berndartmueller (judge) commented:

@Bauchibred - no, I don’t see how high severity would be justified when this is a compatibility issue with rebasing tokens. “Assets” at risk in this case would be only the rebasing token itself, so the risk is contained.

# [M-09] The bankBalance function failed to handle errors correctly

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Finding ID:** M-09
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-nibiru
- **Source snapshot:** competitions/2024-11-nibiru/final_report.html

bankBalance function failed to handle errors correctly Submitted by shaflow2, also found by Rhaydden The bankBalance function does not handle errors after decoding the call parameters. As a result, p.evmKeeper.Bank.GetBalance may throw a panic, and this erroneous panic cannot be recovered by HandleOutOfGasPanic, leading to the erroneous panic being propagated further up the program.

## Recommended mitigation steps

func (p precompileFunToken) bankBalance( start OnRunStartResult, contract *vm.Contract, ) (bz []byte, err error) { method, args, ctx:= start.Method, start.Args, start.CacheCtx defer func() { if err != nil { err = ErrMethodCalled(method, err) } }() if err:= assertContractQuery(contract); err != nil { return bz, err } addrEth, addrBech32, bankDenom, err:= p.parseArgsBankBalance(args) + if err != nil { + err = ErrInvalidArgs(err) + return + } bankBal:= p.evmKeeper.Bank.GetBalance(ctx, addrBech32, bankDenom).Amount.BigInt() return method.Outputs.Pack([]any{ bankBal, struct { EthAddr gethcommon.Address `json:"ethAddr"` Bech32Addr string `json:"bech32Addr"` }{ EthAddr: addrEth, Bech32Addr: addrBech32.String(),

}, }...) } Unique-Divine (Nibiru) confirmed Nibiru mitigated:

PR-2116 - Fixed bug where the err != nil check is missing in the bankBalance precompile method.

Status:

Mitigation confirmed.

# [M-10] IOracle.queryExchangeRate returns incorrect blockTimeMs

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Finding ID:** M-10
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-nibiru
- **Source snapshot:** competitions/2024-11-nibiru/final_report.html

IOracle.queryExchangeRate returns incorrect blockTimeMs Submitted by 3docSec The IOracle.queryExchangeRate offers the following lookup functionality, in analogy to other oracles like Chainlinks’:

/// @notice Queries the dated exchange rate for a given pair /// @param pair The asset pair to query. For example, "ubtc:uusd" is the /// USD price of BTC and "unibi:uusd" is the USD price of NIBI.

/// @return price The exchange rate for the given pair /// @return blockTimeMs The block time in milliseconds when the price was /// last updated /// @return blockHeight The block height when the price was last updated /// @dev This function is view-only and does not modify state.

function queryExchangeRate ( string memory pair ) external view returns ( uint256 price, uint64 blockTimeMs, uint64 blockHeight ); If we focus on the blockTimeMs returned value, this is meant to correspond to the time when the price was last updated and, in analogy to how Chainlink oracles are typically used, is most likely to be used in staleness checks.

If we see how this is implemented, we see that values are passed through from OracleKeeper (L92):

File: oracle.

go 78:

func (p precompileOracle) queryExchangeRate ( 79: ctx sdk.Context, 80: method *gethabi.Method, 81: args []any, 82: ) (bz [] byte, err error ) { 83:

pair, err:= p.

parseQueryExchangeRateArgs (args) 84:

if err != nil { 85:

return nil, err 86: } 87:

assetPair, err:= asset.

TryNewPair (pair) 88:

if err != nil { 89:

return nil, err 90: } 91:

92:

price, blockTime, blockHeight, err:= p.oracleKeeper.

GetDatedExchangeRate (ctx, assetPair) 93:

if err != nil { 94:

return nil, err 95: } 96:

97:

return method.Outputs.

Pack (price.

BigInt (), uint64 (blockTime), blockHeight) 98: } However, the blockTime returned by oracleKeeper.GetDatedExchangeRate corresponds to the “current” block time and, therefore, does not correspond to what should be returned by the precompile, that is the block time of the last price update.

The impact of this misalignment is quite severe because since the returned value is the timestamp returned is always that of the current block, the price will always look as freshly set when validated against blockTime, which is the most common validation. Consequently, downstream contracts will likely always pass staleness checks on potentially extremely old prices.

## Recommended Mitigation Steps

Consider maintaining and querying a lookup table (height to time) for returning the right blockTime instead of relying on the value passed by GetDatedExchangeRate.

Unique-Divine (Nibiru) commented:

Running the steps to reproduce the issue and see if the direct context modifier works or if a CometBFT query is needed.

Unique-Divine (Nibiru) confirmed and commented:

This ticket has been addressed here. This change adds the block timestamp as a field on the data structure where we store the price and block number. I’ve also added a test case to prevent regressions and show correctness Since the exchange rate’s timestamp is not actually used to signal an expiry in the protocol (it’s only stored to make this query for the EVM), I’d maybe argue that this isn’t high risk but more so a medium when compared with other tickets.

berndartmueller (judge) decreased severity to Medium and commented:

After more consideration, I consider this issue to be Medium severity.

The chosen High severity is really borderline and has likely been determined on the assumption that the oracle prices are outdated/stale. However, this represents an external requirement, justifying Medium severity. This also aligns with issues in other audits that highlight the incorrect use of Chainlink’s latestAnswer(), judged as Medium severity.

Nibiru mitigated:

PR-2117 - Added timestamps for exchange rates.

Status:

Mitigation confirmed.

## Rejected Primary Findings

# Rejected Primary Findings: Nibiru

# Potential Chain Split Due to CometBFT State Sync Vulnerability (ASA-2024-009)

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-14
- **Submitter:** 0x41
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/14
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-14.md

## Brief Summary

The Nibiru project uses CometBFT 0.37.5 as specified in its `go.mod`, which is affected by a medium severity vulnerability ([ASA-2024-009](https://github.com/cometbft/cometbft/security/advisories/GHSA-g5xx-c4hv-9ccc)). The vulnerability allows a malicious state sync node to provide an invalid state of the proposer selection algorithm, which could lead to a chain split. When validators state sync from RPC endpoints with invalid proposer priority states, they will: 1. Disagree with other validators about block proposer selection 2. Reject valid blocks due to proposer mismatch 3. Have their own proposed blocks rejected by the network 4. Eventually contribute to network halt if enough validator...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Hardcoded gas limits may lead to failed execution when opcode costs change

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-62
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/62
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-62.md

## Brief Summary

Currently, the gas limits for several operations are hardcoded in the `erc20` file. This can create a situation where the future calls / deployments will fail due to the opcodes cost changes.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Coin validation in fee handling would lead to broken accounting for fees

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-164
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/164
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-164.md

## Brief Summary

Borderline, considering the direct construction of `sdk.Coin` without proper validation could lead to not only invalid fee calculations and potential panics in production, but also the fact that the `Add` function would never return coins i.e when we have the amount to be non-negative for one of the coins, see documentation from `Add()` above.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_31_group

# Nibiru lacks support for TSTORE

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-167
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/167
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-167.md

## Brief Summary

Smart contracts relying on transient storage will fail, i.e DApps designed with transient storage optimizations won't work (incompatibility with Ethereum mainnet behavior). Which also showcases how the first window for attack ideas requested by Nibiru for the audit is not followed: Considering users that expect the same implementation of `tstore` even in Nibiru would not get this as they are used to it on Chains like Mainnet/Arbitrum.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# One could still provide gas for tx lower than base fee

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-168
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/168
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-168.md

## Brief Summary

Inconsistent fee market behavior, also seems to portray how network could be congested since we now provide lower gas fees than the base fee.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lower decimal tokens would be non-transferrable

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-193
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/193
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-193.md

## Brief Summary

> 2 — Med: Assets not at direct risk, but the function of the protocol or its availability could be impacted, or leak value with a hypothetical attack path with stated assumptions, but external requirements. MEDIUM. Considering the strict minimum transfer requirement of 10^12 wei causes: - Inability to transfer small amounts of low-decimal tokens... \_this ends up as not being able to integrate these set of tokens, contrary to what's been stated in the docs.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unsafe Integer Conversion in GetTransactionLogs

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-100
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/100
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-100.md

## Brief Summary

The res.MsgIndex field is cast to an integer without verifying bounds or type. Impact: High May cause incorrect indexing of logs or runtime panics.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Lack of Authorization for Critical Methods in function SendRawTransaction, GetTransactionByHash, and GetTransactionReceipt

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-102
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/102
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-102.md

## Brief Summary

Many methods such as SendRawTransaction, GetTransactionByHash, and GetTransactionReceipt do not appear to have any access control mechanisms Impact: Malicious actors could potentially send arbitrary transactions to the blockchain if no checks are in place to verify the source or authenticity of the requests. This can result in unauthorized transactions being broadcasted to the network. Risk: High. If there are no access controls in place for sensitive functions, this could lead to unauthorized users interacting with the blockchain.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Potential Memory Exhaustion in Gas Estimation

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-106
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/106
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-106.md

## Brief Summary

- May allow DoS attacks via large transaction inputs. - Affects service availability. Risk High – Memory exhaustion vulnerabilities are a common vector for attacks.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Insufficient Validation of hexutil.Bytes Input

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-107
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/107
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-107.md

## Brief Summary

In SendRawTransaction, raw transaction bytes are decoded without additional validation. Impact: Denial of service through malformed payloads. Risk: High

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Insufficient Granularity in Multi-Execution Error Handling in executeMulti function

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-108
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/108
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-108.md

## Brief Summary

The executeMulti method stops execution for all subsequent calls if any individual execution fails. Impact: A single failure could prevent legitimate operations, leading to poor user experience. Risk: Medium

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Access Control on instantiate

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-109
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/109
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-109.md

## Brief Summary

The instantiate method doesn't enforce strict access control, allowing any caller to instantiate contracts. Impact: Malicious actors could deploy unverified contracts, leading to trust issues. Risk: High

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Lack of Input Validation in queryRaw

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-111
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/111
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-111.md

## Brief Summary

The function queryRaw assumes args[argIdx] is always of type []byte. If an invalid argument type is passed, it will panic instead of returning an error. Impact: - Unexpected panics can disrupt EVM execution. - May be exploitable if raw inputs are not sanitized.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Validation of Contract Addresses in execute, executeMulti, instantiate, and query function

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-112
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/112
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-112.md

## Brief Summary

Addresses passed in execute, executeMulti, instantiate, and query are not validated to ensure they are correctly formatted Bech32 addresses.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Unchecked Type Assertions in Multiple Functions like retrieveEVMTxFeesFromBlock and AllTxLogsFromEvents

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-113
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/113
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-113.md

## Brief Summary

Several functions, such as retrieveEVMTxFeesFromBlock and AllTxLogsFromEvents, use unchecked type assertions that may cause runtime panics. Impact: - Runtime panics due to invalid type assertions. - Inconsistent behavior and potential Denial-of-Service (DoS) vectors. Risk: High

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Potential Out-of-Bounds Error in TxLogsFromEvents

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-114
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/114
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-114.md

## Brief Summary

The function decrements msgIndex without bounds checking, leading to a potential negative index. Impact: - The function might access unintended or undefined memory. - Could result in unexpected application behavior. Risk: Medium

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Gas Limit Validation in retrieveEVMTxFeesFromBlock

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-115
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/115
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-115.md

## Brief Summary

The function does not check for invalid or zero values for blockGasUsed, which can cause division-by-zero errors or invalid calculations. Impact: - Division-by-zero errors if gasLimitUint64 is invalid. - Incorrect gas usage ratio calculations. Risk: High

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_13_group

# Potential Index Out of Range in retrieveEVMTxFeesFromBlock

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-116
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/116
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-116.md

## Brief Summary

The code uses txIndex to iterate through the sorter array, but there is no check to ensure that txIndex does not exceed the length of the array. Impact: - Index Out of Range: If the array sorter is empty or if txIndex exceeds ethTxCount-1, the code will panic due to an "index out of range" error. Risk: High - This can cause the service to crash, making it unreliable.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Missing Input Validation for Trace Call

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-119
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/119
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-119.md

## Brief Summary

In the TraceCall function, there is no input validation to ensure that the parameters passed, especially the txArgs and contextBlock, are valid and not nil. User-provided inputs like hash, txArgs, or config are not properly validated for correctness. Malformed inputs could lead to panics or unexpected errors. Impact: - Malformed inputs can cause system instability. - Exposes backend to DoS attacks. - If txArgs is nil or improperly formatted, this could result in a runtime error, crashing the application. It may also lead to incorrect behavior when invoking trace functionality. Risk: High This missing validation can lead to unexpected crashes, incorrect results, or failures in tracing due to...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Unnecessary Access to NetworkClient

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-120
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/120
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-120.md

## Brief Summary

In TraceTransaction, TraceCall and TraceBlock, the code accesses NetworkClient using the following pattern: nc, ok := b.clientCtx.Client.(tmrpcclient.NetworkClient) if !ok { return nil, errors.New("invalid rpc client") } This cast is performed multiple times and is unnecessary when the client context (b.clientCtx.Client) can be validated once at the start. Impact: Repeated casting of the client context might impact performance if the client is accessed multiple times within a block or transaction trace. Additionally, it introduces redundancy in the code. Risk: While this doesn't create a significant security risk, it contributes to code inefficiency and redundancy, which could make maintena...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Validation in parseArgsWasmInstantiate

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-123
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/123
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-123.md

## Brief Summary

- The admin string and msgArgs byte array are not validated for length, potentially leading to oversized or malformed payloads. - This oversight introduces potential vulnerabilities related to oversized or malformed input, which can have downstream effects. The parseArgsWasmInstantiate function processes arguments for contract instantiation but lacks constraints on: 1. admin: A string that should represent an optional administrator address. There is no check for: - Maximum permissible length. - Whether the string conforms to Bech32 encoding when provided. 2. msgArgs: A byte array that represents the contract initialization message payload. There is no restriction on: - The maximum allowable...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_40_group

# Missing Synchronization in Concurrent Environments

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-128
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/128
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-128.md

## Brief Summary

The journal struct's methods (append, Revert, etc.) modify shared state (e.g., entries, dirties). There is no synchronization mechanism to prevent data races in concurrent environments. Impact: - Data races leading to unpredictable behavior. - Corrupted state data in entries and dirties. Risk Level: High

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Inconsistent Snapshot Handling in Revert Function

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-130
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/130
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-130.md

## Brief Summary

The Revert function depends on a snapshot index without validating its range. Impact: Improper handling can lead to crashes or undefined behavior during state rollbacks. Risk: High — as state rollback is critical to blockchain execution integrity.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_08_group

# Gas Inefficiency and Redundant Operations in sortedDirties

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-131
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/131
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-131.md

## Brief Summary

The sortedDirties function creates a new slice and sorts it each time it is called, even if the dirties map has not changed. Sorting dirty addresses may lead to performance issues when the number of entries is large. Impact: - Increased gas costs in cases with frequent dirty address accesses. - Inefficiency when used in a loop or multiple times. - Performance degradation in scenarios with a high number of dirty addresses. Risk Level: Medium

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Integer Overflow/Underflow Handling in AddBalance and SubBalance

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-132
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/132
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-132.md

## Brief Summary

Functions AddBalance and SubBalance directly modify balances without validating that the operations do not result in overflow/underflow. - In the AddBalance and SubBalance methods, there is no explicit check for overflow or underflow when performing arithmetic operations on the account balance. - Although Go supports arbitrary-precision integers with *big.Int, overflow or underflow could occur with operations involving external inputs (such as user-provided amounts). AddBalance: s.SetBalance(new(big.Int).Add(s.Balance(), amount)) SubBalance: s.SetBalance(new(big.Int).Sub(s.Balance(), amount)) Even though *big.Int handles large values, operations that might result in invalid values (such as...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_20_group

# Improper Handling of Code Hash & Code Modifications in SetCode

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-133
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/133
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-133.md

## Brief Summary

The SetCode function assumes the provided codeHash is valid and directly updates the state, making it susceptible to invalid or tampered hashes. The method SetCode allows changing the contract code by updating the CodeHash and the associated code. However, there is no validation that checks if the new code is different from the previous one before applying changes. Impact: Invalid code hashes could lead to execution errors or contract code inconsistencies. 1. Inefficiency: Setting the code unnecessarily adds redundant operations to the journal, which may increase computational and storage costs. 2. Possible security risks: If the new code is not validated correctly, an attacker could exploi...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Access Control for Sensitive Functions like Mint

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-135
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/135
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-135.md

## Brief Summary

The Mint function allows minting tokens, which is a highly sensitive operation. While the Solidity function restricts access using the onlyOwner modifier, there is no explicit check in the Go implementation to ensure only authorized users can invoke this function. The Mint method does not explicitly verify if the caller is the contract owner. While the documentation specifies onlyOwner, the implementation does not enforce this constraint programmatically. Impact - If the from address is not properly validated, unauthorized parties could mint tokens, inflating the total supply and potentially compromising the token's value and ecosystem. - An attacker could exploit this by calling the Mint f...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Insufficient Error Handling in Transfer

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-136
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/136
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-136.md

## Brief Summary

The Transfer function assumes that the recipient's balance always increases when the transfer succeeds. This assumption may not hold for fee-on-transfer tokens, which deduct fees from the transferred amount. The Transfer function only checks the success boolean but does not revert when success == false. Impact For tokens with transfer fees, this validation will falsely flag legitimate transfers as failed, disrupting the functionality for fee-on-transfer tokens. Users may perceive the system as unreliable due to ambiguous failure handling Risk High

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Invalid Balance Parsing GetBalance and GetProof

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-140
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/140
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-140.md

## Brief Summary

Balance conversion from string in GetBalance and GetProof lacks proper validation and error handling. Impact Malformed balance strings can crash the node or cause undefined behavior. Risk High

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Inefficient Storage Proof Handling in GetProof

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-141
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/141
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-141.md

## Brief Summary

GetProof queries storage proofs in a loop, potentially causing performance bottlenecks for large storageKeys arrays. Impact Nodes can experience significant delays when processing multiple storage keys due to sequential requests. Risk Moderate

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Panic on Error in GetCode function

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-142
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/142
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-142.md

## Brief Summary

The GetCode function calls panic(err) when an error occurs while fetching contract bytecode. Panicking in production code is generally not recommended, as it can lead to unhandled exceptions, breaking the flow of the system. Impact: The use of panic can cause the application to terminate unexpectedly, leading to a poor user experience and potential loss of functionality. System downtime or disruption in contract processing could occur. Risk: High: The system can crash, causing interruptions in services.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_22_group

# Inefficient Use of big.Int for Balance Comparison in SetAccBalance

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-143
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/143
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-143.md

## Brief Summary

The balance comparison in the SetAccBalance function uses big.Int for each coin calculation and comparison, which can be inefficient. The logic creates new big.Int objects multiple times in each case. Impact: Medium: The frequent creation of new big.Int objects can cause unnecessary memory allocation, impacting the performance of the contract especially in a high-frequency environment. Risk: Medium: It can cause higher gas costs and reduced performance due to inefficient memory handling.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential for nil Dereference in GetAccount

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-144
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/144
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-144.md

## Brief Summary

In the GetAccount method, if the GetAccountWithoutBalance returns nil (i.e., no account is found), the function continues to access the acct object, leading to a potential nil dereference. Impact: High: A nil dereference can cause a runtime panic, leading to a crash of the application. The system may not recover and could lead to data inconsistencies. Risk: High: A crash could interrupt processing and affect multiple contracts and accounts.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Potential Unchecked Error in GetContractBytecode and GetState

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-145
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/145
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-145.md

## Brief Summary

In the GetContractBytecode and GetState methods, there's an implicit handling of default values for missing keys. However, when using the GetOr function, the code does not handle potential errors explicitly. While missing data is defaulted to an empty byte array (for GetContractBytecode) or an empty Hash (for GetState), there is no check or logging for the errors that might occur, which could lead to silent failures. Impact: Silent Failures: The code will silently return default values if an error occurs, making it difficult to diagnose issues. Risk: This could lead to faulty contract behavior or misleading results, especially if an actual data retrieval error occurs. Risk: Critical: Lack o...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Potential Security Risk with SetAccState (Inconsistent State Deletion)

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-146
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/146
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-146.md

## Brief Summary

In the SetAccState function, if the provided stateValue is empty, the corresponding entry in AccState is deleted. This might inadvertently erase important state information that could be necessary, especially for non-empty state values in other contexts. Impact: Data Loss: Deleting state without any checks may lead to accidental removal of data that could be needed for contract execution, leading to potential data corruption. Risk: Moderate to High: Deletion logic should include safeguards, as unintended state deletions could cause the smart contract to behave unpredictably or lead to vulnerabilities in the contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_23_group

# No Validation of CreateFuntokenFee in SetParams

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-147
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/147
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-147.md

## Brief Summary

The SetParams function checks if CreateFuntokenFee is negative, but no validation occurs on other parameters of params. There might be other parameters that should be validated for correct ranges, types, or values. Impact: Incomplete Validation: Missing validation for other parameters could allow unintended values to be set, potentially causing vulnerabilities or improper contract execution. Risk: Moderate: Improper parameter values could introduce logic flaws or security issues, especially if the contract deals with funds or important configurations.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_01_group

# Incorrect Handling of Zero refundQuotient in GasToRefund

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-148
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/148
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-148.md

## Brief Summary

The GasToRefund function assumes refundQuotient is always non-zero. A zero value will cause a division by zero error, leading to a runtime panic. Impact: This issue could cause the application to crash if the refundQuotient value is accidentally set to zero, disrupting the state machine. Risk: High

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Lack of Validation for leftoverGas in RefundGas

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-149
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/149
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-149.md

## Brief Summary

The RefundGas function assumes leftoverGas is valid. While unlikely, a bug elsewhere in the system could pass an invalid or excessively large leftoverGas value. Impact: Could lead to an overflow when calculating leftoverWei, resulting in incorrect refunds or system instability. Risk: Medium

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Checks in DeductTxCostsFromUserBalance

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-150
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/150
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-150.md

## Brief Summary

The function does not verify if fees is non-zero. A zero fee could bypass deductions. Impact: This could allow transactions to proceed without proper fee deductions, leading to economic inconsistencies. Risk: High

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# PendingTransactions Missing Check for TxDecoder

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-153
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/153
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-153.md

## Brief Summary

The PendingTransactions method assumes TxDecoder is always non-nil, leading to a potential panic if it's not initialized. Impact: High. Critical functionality relies on TxDecoder. Risk: High. Can crash the application during runtime.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# FeeHistory Block Calculation Logic

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-154
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/154
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-154.md

## Brief Summary

Description: In the FeeHistory function, blockEnd+1 < blocks can result in an underflow if blockEnd is negative or small compared to blocks. Impact: High. Could cause incorrect fee history retrieval or unexpected behavior. Risk: High. Affects core functionality.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_58_group

# Missing Input Validation methodById and decomposeInput

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-156
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/156
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-156.md

## Brief Summary

The methodById and decomposeInput functions do not adequately validate the input data passed to them. For example, while there is a length check (len(sigdata) != 4), there is no safeguard against malformed or malicious inputs. Impact Can result in panics or undefined behavior. Might expose the EVM to denial-of-service (DoS) attacks by exploiting gas consumption for erroneous inputs. Risk High – Input validation is a critical component of contract security.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Gas Cost Calculation Vulnerability in requiredGas

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-157
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/157
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-157.md

## Brief Summary

In requiredGas, the isMutation map is accessed without checking for the presence of the method. While tests may ensure valid methods, introducing new methods without updating isMutation can lead to panics. Impact - Contracts could break if unexpected methods are introduced. - Potential for deployment downtime or unintended halts. Risk Medium – A well-maintained codebase reduces the risk, but unhandled edge cases can still cause issues.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# epetitive StateDB Synchronization Logic

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-174
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/174
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-174.md

## Brief Summary

The logic to synchronize the StateDB is repeated across multiple functions such as MintCoins, BurnCoins, SendCoins, etc. This duplication leads to a larger and harder-to-maintain codebase. The SyncStateDBWithAccount function silently fails if StateDB is nil. This could lead to inconsistent state if StateDB is not properly initialized. Impact: - Silent failure may lead to debugging difficulties. - Could cause desynchronization between Cosmos SDK state and Ethereum VM state.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Issue: Missing Input Validation for Coin Denomination

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-175
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/175
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-175.md

## Brief Summary

The contract does not validate the denomination (Denom) of the coins provided in the findEtherBalanceChangeFromCoins function. If Denom contains invalid or unexpected values (e.g., empty string or incorrect format), it could cause logical errors or unintended behavior. Impact: Risk of processing invalid or malicious coin denominations. Potential inconsistencies in state updates. Risk Level: Medium

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_28_group

# Missing Checks for Negative Coin Balances

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-176
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/176
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-176.md

## Brief Summary

The MintCoins, BurnCoins, and SendCoins functions assume that balances are non-negative. There are no checks to prevent negative balances due to logic errors. Impact: - Negative balances can lead to state inconsistencies. - Could be exploited to create invalid state transitions. Risk Level: Critical

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Missing Replay Protection

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-178
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/178
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-178.md

## Brief Summary

There is no mechanism in the code to prevent replay attacks. A malicious user could reuse a signed transaction multiple times, especially if signatures are not properly handled. Impact: - Potential double-spending or unauthorized state modifications. - Risks to user funds and overall chain integrity. Risk Level: High

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_04_group

# Metadata Overlap in ToBankMetadata

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-180
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/180
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-180.md

## Brief Summary

In the ToBankMetadata function, there's a potential overlap in the denomUnits and symbol fields. If a token has the same name as another, it might overwrite existing metadata or cause conflicts. Impact: Conflicts in DenomUnits can lead to users seeing incorrect balances or misidentification of assets. Risk: Medium

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unchecked User Input for Gas Limits

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-181
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/181
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-181.md

## Brief Summary

The txData.GetGas() method is used directly to determine gas limits without robust validation to ensure it aligns with the block gas limit. Impact: Can lead to denial of service (DoS) by exceeding gas thresholds or resource exhaustion. Risk: High

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_24_group

# Function AddToBlockGasUsed: Integer Overflow

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-183
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/183
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-183.md

## Brief Summary

The function calculates blockGasUsed as k.EvmState.BlockGasUsed.GetOr(ctx, 0) + gasUsed. An integer overflow may occur if gasUsed is large enough, causing blockGasUsed to wrap around and become smaller than gasUsed. Impact: High — Incorrect gas accounting can destabilize the EVM module, potentially allowing malicious actors to bypass gas limits. Risk: Severe.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_16_group

# Function BaseFeeWeiPerGas: Invalid Context Instantiation

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-184
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/184
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-184.md

## Brief Summary

The function calls k.BaseFeeMicronibiPerGas(sdk.Context{}). Passing an empty context (sdk.Context{}) may lead to undefined behavior or incorrect calculations if BaseFeeMicronibiPerGas depends on contextual parameters. Impact: Medium — Incorrect fee calculations may lead to under- or over-charging users, reducing system trust. Risk: Moderate.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Panic Handling During Out of Gas

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-185
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/185
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-185.md

## Brief Summary

The HandleOutOfGasPanic mechanism used with defer assumes all panics are gas-related, which may not always hold true. Other critical issues causing panics could be masked. Impact: - Non-gas-related errors are incorrectly categorized, potentially hiding critical bugs or malicious attempts. - Leads to poor debugging experience and reduced reliability. Risk Level: High

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_18_group

# Gas Limit Mismanagement

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-186
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/186
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-186.md

## Brief Summary

Gas is consumed unconditionally in some scenarios, such as when EVM execution fails due to an error other than ErrOutOfGas. This could lead to over-consumption of gas. Impact: - Unnecessary depletion of gas for transactions that fail due to logical errors or external constraints. - Potential Denial of Service (DoS) if this behavior is exploited. Risk Level: High

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_26_group

# Nil Pointer Dereference on baseFeeWeiPerGas

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-188
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/188
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-188.md

## Brief Summary

The contract checks whether baseFeeWeiPerGas is nil, but the subsequent call to msgEthTx.EffectiveGasCapWei(baseFeeWeiPerGas) does not handle the scenario where it is nil. This creates the potential for a runtime panic. Impact High: This could cause a denial of service (DoS) by panicking the system when processing specific transactions. Risk Level High

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Potential Integer Overflows in TxConfig

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-189
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/189
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-189.md

## Brief Summary

The TxIndex and LogIndex are retrieved using uint conversion, which may result in undefined behavior if the source value exceeds the bounds of unit. Impact: Integer overflow could lead to incorrect configuration or crashing nodes. Risk: Medium

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Inconsistent Index Management in DeleteSlot

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-190
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/190
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-190.md

## Brief Summary

In the DeleteSlot function, the deletion of the slot may result in incorrect management of the slots array. The logic to handle "last slot deletion" (al.slots = al.slots[:idx]) truncates the array incorrectly. This can lead to removal of unrelated slots and address mismanagement. Impact: This issue can lead to the loss of critical access list data, which might cause unintended access or denial of access to slots. Risk: High Incorrect state management can result in unpredictable behavior, impacting transaction correctness.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Lack of Validation for idx in Contains

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-191
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/191
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-191.md

## Brief Summary

The Contains function does not validate the idx before accessing al.slots[idx]. If idx is out of bounds due to an invalid state, it will panic. Impact: A panic could halt execution unexpectedly, causing disruptions in smart contract processing. Risk: Medium Although this requires an invalid state, improper initialization or corrupted state can trigger it.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# No Check for Nonce Exhaustion in deployERC20ForBankCoin

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-194
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/194
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-194.md

## Brief Summary

The deployERC20ForBankCoin function creates a deterministic address using the nonce of the EVM_MODULE_ADDRESS. If the nonce reaches its limit, the contract creation will fail. Impact: Failure to deploy new ERC20 contracts once the nonce is exhausted. Risk Level: High.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Missing Reentrancy Protection in CallContractWithInput function

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-195
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/195
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-195.md

## Brief Summary

If the CallContractWithInput function interacts with external contracts, there is a risk of reentrancy attacks. Impact: Unauthorized modifications or data leakage during contract calls. Risk Level: High.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_39_group

# Absence of Rate-Limiting or Validation in RPCFilterCap

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-196
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/196
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-196.md

## Brief Summary

The RPCFilterCap function lacks mechanisms to enforce or validate the filter cap dynamically, which might lead to resource exhaustion. Impact: Resource exhaustion from excessive filter creation. Risk Level: High

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Handling for Duplicate Insertions

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-198
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/198
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-198.md

## Brief Summary

The SafeInsert function directly inserts the token without checking if a token with the same ID already exists in the state. This could overwrite existing tokens without notice. Impact: Overwriting tokens may cause data loss and inconsistencies, especially if the same erc20 address maps to multiple bankDenom values. Risk: Medium - While recoverable, this could lead to business logic errors or unintended overwrites.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_07_group

# Infinite Loop Risk and Incorrect Iteration Terminationin ForEachStorage

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-74
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/74
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-74.md

## Brief Summary

The stopIter callback is expected to stop iteration when it returns false, but if it always returns true, the loop can iterate indefinitely. The iterator loop in ForEachStorage uses stopIter incorrectly. When stopIter returns false, the iteration stops prematurely, but it is intended to continue iterating. Impact: Medium. May lead to node resource exhaustion. Important storage keys may be skipped, leading to incomplete storage traversal. Risk: Dependent on usage patterns and callback implementations.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unchecked Input in SetState

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-75
- **Submitter:** SmartAuditPro
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/75
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-75.md

## Brief Summary

The SetState function allows updating contract storage by setting the value for a given key. However, there is no validation of the stateKey or stateValue inputs, leaving the system vulnerable to issues such as: Risk: 1. State Corruption: Invalid keys or values can introduce inconsistencies or unexpected behavior in the contract state. 2. Denial of Service (DoS): Excessively large or malformed inputs can strain node resources, leading to degraded performance or system crashes. 3. Unauthorized Access: Lack of validation may enable attackers to overwrite critical data unintentionally or maliciously. Impact: - Data integrity is compromised if invalid keys or values corrupt the storage. - Perfo...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Inefficient Error Handling in Transaction Decoding in function EthMsgsFromTendermintBlock

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-91
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/91
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-91.md

## Brief Summary

The function EthMsgsFromTendermintBlock decodes transactions as follows: tx, err := b.clientCtx.TxConfig.TxDecoder()(tx) if err != nil { b.logger.Debug("failed to decode transaction in block", "height", block.Height, "error", err.Error()) continue } The error handling logs the error and proceeds to the next iteration of the loop, ignoring the failure silently. While logging errors is useful, it doesn't properly handle the error scenario and continues processing without addressing the issue. Impact 1. Silent failure: The system might continue processing invalid or incomplete data, leading to inaccurate or incomplete information being returned to the client. 2. Data integrity risk: Invalid tr...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Pruning Configurations Leading to Missing Data in Functions like RPCBlockFromTendermintBlock, EthBlockFromTendermintBlock

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-93
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/93
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-93.md

## Brief Summary

The smart contract backend contains several references to handling pruned nodes, such as fetching data from pruned blocks and failing gracefully when the block data is unavailable due to node pruning. Specifically, the issue arises in methods like RPCBlockFromTendermintBlock, EthBlockFromTendermintBlock, and others when attempting to fetch historical data that has been pruned. Bug Locations 1. RPCBlockFromTendermintBlock: - Error handling for BaseFeeWei fails when querying pruned blocks. The log output explicitly states, "failed to fetch Base Fee from prunned block". 2. EthBlockFromTendermintBlock: - Similar error occurs while attempting to retrieve BaseFeeWei. The error is logged but does...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Input Validation Issues in sendToBank function

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-94
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/94
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-94.md

## Brief Summary

The contract's sendToBank method does not validate inputs comprehensively. The argument parsing (parseArgsSendToBank) assumes that inputs will always be of the expected type and format. If malformed input is passed, the contract will crash or behave unpredictably. - The amount parameter in sendToBank is validated only as amount.Cmp(big.NewInt(0)) != 1, which excludes nil but doesn't ensure sufficient precision (e.g., very small amounts). - The to parameter validation relies solely on sdk.AccAddressFromBech32, which doesn’t verify if the address is active or capable of receiving tokens. Impact: - Untrusted or invalid inputs could lead to unexpected behavior, contract failures, or vulnerabili...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_00_group

# Inefficient Transaction Search in GetTransactionByHash and getTransactionByHashPending methods

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-98
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/98
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-98.md

## Brief Summary

In both methods, the logic includes redundant or inefficient checks such as iterating over the entire list of transactions in a block or mempool in order to find a specific transaction. This can be particularly problematic when dealing with large blocks or high transaction volumes. Impact: Increased computational overhead and slower response times, which could degrade user experience and application performance. Potential scalability issues as the blockchain grows.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Input Validation for GetTransactionByHash

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-99
- **Submitter:** SmartAuditPro
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/99
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-99.md

## Brief Summary

The method directly processes hash without ensuring that it is valid or follows the expected format. Impact: Medium. This can lead to application crashes or potential Denial-of-Service (DoS) if exploited repeatedly.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_33_group

# Overflow risk due to improper block number casting in `GetTransactionCount()`

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-26
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/26
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-26.md

## Brief Summary

In the `GetTransactionCount()` function, a potential `integer overflow` issue exists due to the absence of a boundary check when converting a block number (`bn`) `int64`. While the `GetProof()` function implements a safeguard by checking if bn exceeds `math.MaxInt64` before casting, `GetTransactionCount()` directly casts bn to `int64` without this validation. This inconsistency could lead to `overflow`, resulting in erroneous or negative block heights, which may cause logic errors or unintended behavior.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_06_group

# GetEthIntrinsicGas Function Should Read Homestead and Istanbul Flags from Config Instead of Using Fixed true Values

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-41
- **Submitter:** ayden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/41
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-41.md

## Brief Summary

The GetEthIntrinsicGas function in the Cosmos SDK-based EVM module (likely in x/evm/keeper/gas.go) is responsible for calculating the intrinsic gas cost for Ethereum transactions. The issue arises from the fact that the function currently uses fixed true values for the homestead and istanbul parameters when calling core.IntrinsicGas. These values should instead be derived from the configuration (cfg) based on the current block height, as Ethereum's network conditions change with different network upgrades (such as Homestead and Istanbul).

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential Negative Value Transfer Vulnerability

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-43
- **Submitter:** ayden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/43
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-43.md

## Brief Summary

The current implementation only checks for positive value transfers in the CanTransferDecorator, potentially allowing negative value transfers to bypass balance checks. - Theoretical possibility of negative value transfers - Potential balance manipulation - Bypass of transfer validation checks

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_03_group

# wrong implement of AddPrecompiles

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-53
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/53
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-53.md

## Brief Summary

There is a wrong implementation of AddPrecompiles as the is an extra comma.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Validation for Exchange Rate Bounds in `precompileOracle` Smart Contract

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-89
- **Submitter:** enami_el
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/89
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-89.md

## Brief Summary

**Missing Validation for Exchange Rate Bounds in `precompileOracle` Smart Contract** --- Summary The lack of validation for minimum and maximum exchange rate bounds in the `queryExchangeRate` method will cause a mispricing vulnerability for the protocol. This allows stale or manipulated exchange rates to propagate through the system, potentially causing incorrect calculations or economic losses for protocol users and stakeholders. --- Root Cause In the `precompileOracle` smart contract: - In `queryExchangeRate` , the contract fetches the exchange rate using `p.oracleKeeper.GetDatedExchangeRate`. - The returned `price` is directly packed and returned without checking whether it falls within...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Weak permissions and potential for permission escalation in "ModuleAccPerms"

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-5
- **Submitter:** felon
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/5
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-5.md

## Brief Summary

The ModuleAccPerms function returns permissions that allow minting and burning tokens without fine-grained control. Allowing multiple modules to mint and burn tokens presents a high-risk for token manipulation or supply inflation, especially if exploited by modules with loose or insufficiently controlled access.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential Error wrapping with errors.Wrapf

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-6
- **Submitter:** felon
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/6
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-6.md

## Brief Summary

The use of "errors.Wrapf" provides contextual error information, but improper handling of wrapped errors could expose internal details, potentially leading to information leakage.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Validity Check for Parameters in the balance Function in x/evm/precompile/funtoken.go

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-28
- **Submitter:** gxh191
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/28
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-28.md

## Brief Summary

The `balance` function in Go is used to implement the `IFunToken.balance` contract method. Its purpose is to return the ERC20 balance of a user address, the bank balance, FunToken-related information, and the Nibiru base account address. solidity // function balance( // address who, // address funtoken // ) // external // returns ( // uint256 erc20Balance, // uint256 bankBalance, // FunToken memory token, // NibiruAccount memory whoAddrs // ); // In this code, the `balance` function calls `parseArgsBalance` to parse the arguments and obtain `addrEth`, `addrBech32`, and `funtoken`: However, after this, there is no validation of these parameters for their correctness and safety before they ar...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# QA Report

- **Contest:** Nibiru
- **Slug:** 2024-11-nibiru
- **Submission:** V-7
- **Submitter:** klmldl
- **Claimed severity:** QA
- **Final severity:** QA
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-11-nibiru-validation/issues/7
- **Source snapshot:** competitions/2024-11-nibiru/submissions/raw/V-7.md

## Brief Summary

See the markdown file with the details of this report [here](https://github.com/code-423n4/2024-11-nibiru-validation/blob/main/data/klmldl-Q.md).

## Rejection Reason

GitHub validation labels: bug, insufficient quality report, QA (Quality Assurance), :robot:_primary
