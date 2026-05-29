# Accepted H/M Findings: Canto

# [M-01] An attacker can DoS a coinswap pool

- **Contest:** Canto
- **Slug:** 2024-05-canto
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-05-canto
- **Source snapshot:** competitions/2024-05-canto/final_report.html

Submitted by 0x1771, also found by 0xSergeantPepper and zhaojie The balance calculations are initiated by calling k.GetPoolBalances(ctx, pool.EscrowAddress), which internally calls the k.bk.GetAllBalances function. This function iterates through all token balances in a loop. If the array of tokens is excessively large, the function may fail due to insufficient gas.

In essence, if an attacker introduces a large number of tokens, for instance through the AddLiquidity process, and subsequently transfers these tokens to a target pool, it can lead to an exploit. The attacker can strategically overload the array, causing significant gas consumption and ultimately causing the function to fail.

The process is as follows:

When the AddLiquidity or RemoveLiquidity functions are called within the coinswap module, the k.GetPoolBalances function retrieves the balance of all tokens in the pool. This function, k.GetPoolBalances, calls k.bk.GetAllBalances, which iterates through and aggregates all token balances before sorting them into an array.

Specifically, k.bk.GetAllBalances utilizes the following approach:

func (k BaseViewKeeper) GetAllBalances (ctx context.Context, addr sdk.AccAddress) sdk.Coins { balances:= sdk.

NewCoins () k.

IterateAccountBalances (ctx, addr, func (balance sdk.Coin) bool { balances = balances.

Add (balance) return false }) return balances.

Sort () } Here, sdk.NewCoins() returns an array of type Coins.

When an attacker exploits the AddLiquidity function in the coinswap module, they can create a pool using k.CreatePool(ctx, msg.MaxToken.Denom) if the pool does not already exist. By generating a large number of tokens and sending them to the target pool, the attacker causes the array of balances returned by GetPoolBalances to become excessively large. This leads to high gas consumption and potential transaction failure due to insufficient gas, thus disrupting the functionality of the coinswap module.

## Recommended Mitigation Steps

Get only 1 token balance instead of all.

poorphd (Canto) confirmed and commented via duplicate Issue #20:

Reasoning:

As raised in the issue, if an attacker sends tokens of various denoms to the reserved pool address, k.GetPoolBalances(ctx, pool.EscrowAddress) could invoke k.bk.GetAllBalances that internally uses iteration, leading to a situation where the operation could fail if the array becomes very large.

However, pool creation is only allowed for whitelisted denoms, so it is impossible to obtain new tokens through AddLiquidity as raised in the issue. (See here and here ).

Severity:

Mid → Low.

In the worst-case scenario, the swap or RemoveLiquidity in coinswap might fail, but this only affects the auto swap during onboarding and does not impact the essential functions of the chain.

Patch:

We will patch this before v0.50 upgrade.

Change k.GetPoolBalances(ctx, pool.EscrowAddress) so that it does not use k.bk.GetAllBalances and only queries and returns the balance of standard coin, counter party coin, and pool coin.

Make appropriate changes for GetPoolBalances callers.

// GetPoolBalances return the liquidity pool by the specified anotherCoinDenom func (k Keeper) GetPoolBalances (ctx sdk.Context, pool types.Pool) (coins sdk.Coins, err error ) { address, err:= sdk.

AccAddressFromBech32 (pool.EscrowAddress) if err != nil { return coins, err } acc:= k.ak.

GetAccount (ctx, address) if acc == nil { return nil, errorsmod.

Wrap (types.ErrReservePoolNotExists, pool.EscrowAddress) } balances:= sdk.

NewCoins () balances.

Add (k.bk.

GetBalance (ctx, acc.

GetAddress (), pool.StandardDenom)) balances.

Add (k.bk.

GetBalance (ctx, acc.

GetAddress (), pool.CounterpartyDenom)) balances.

Add (k.bk.

GetBalance (ctx, acc.

GetAddress (), pool.LptDenom)) return balances, nil } 3docSec (judge) decreased severity to Medium 3docSec (judge) commented via duplicate Issue #20:

I find Medium to be appropriate for this group.

Because Canto is connected to other Cosmos networks via IBC, an arbitrary number of token denominations can coexist (and be donated) to an existing pool to DoS its liquidity operations, without any privilege required for an attacker.

# [M-02] MsgSwapOrder will never work for Canto nodes

- **Contest:** Canto
- **Slug:** 2024-05-canto
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-05-canto
- **Source snapshot:** competitions/2024-05-canto/final_report.html

MsgSwapOrder will never work for Canto nodes Submitted by 0x1771, also found by 3docSec An oversight in the MsgSwapOrder where the tag is directed to nested. The input message lacks the necessary cosmos.msg.v1.signer to indirectly identify the signer.

message Input { string address = 1; cosmos.base.v1beta1.Coin coin = 2 [ (gogoproto.nullable) = false ]; }

## Recommended Mitigation Steps

Add DefineCustomGetSigners call in app.go for the coinswap Input message like you did for MsgConvertERC20.

- https://github.com/code-423n4/2024-05-canto/blob/d1d51b2293d4689f467b8b1c82bba84f8f7ea008/canto-main/app/app.go#L316
signingOptions.

DefineCustomGetSigners (protov2.

MessageName (&erc20v1.MsgConvertERC20{}), erc20types.GetSignersFromMsgConvertERC20V2) poorphd (Canto) confirmed and commented:

Reasoning:

The liquidity pools used for onboarding are directly called by the keeper method in the IBC middleware, so there is no problem with the onboarding function because the swap occurs. However, since only the swap from the ibc voucher to canto takes place, if there is a price discrepancy, a mechanism is needed for the arbitrager to return to the appropriate price through MsgSwapOrder.

Severity:

Mid → QA.

In order to abuse this for price manipulation, it is necessary to repeatedly make auto-swaps through IBC transfers, but the auto-swap function only works when the balance of canto is less than 4, making it difficult to manipulate prices. This issue is valid, but since it is not an issue that opposes the real risk of assets, the severity should be adjusted from Mid to QA.

Patch:

We will patch this before the v0.50 production release.

3docSec (judge) commented:

As the sponsor said, the effect of this vulnerability is that the pools’ price drifts won’t be balanced by a necessary arbitraging force which is required for the swap to meet the slippage/ maxSwapAmount check; hence, impacting the availability of the Onboarding functionality. For this reason, I find Medium an appropriate severity for this finding.

# [M-03] Govshuttle module does not register its transaction MsgServer

- **Contest:** Canto
- **Slug:** 2024-05-canto
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-05-canto
- **Source snapshot:** competitions/2024-05-canto/final_report.html

Govshuttle module does not register its transaction MsgServer Submitted by 3docSec The x/govshuttle module in canto-main defines and handles two messages that can be emitted by a governance proposal:

MsgLendingMarketProposal MsgTreasuryProposal However, because the module only registers the QueryServer (and not its MsgServer ) in its RegisterServices function, causing no message to be routed to its message server:

func (am AppModule) RegisterServices (cfg module.Configurator) { types.

RegisterQueryServer (cfg.

QueryServer (), am.keeper) } If we compare this with another module that can handle messages, for example, CSR, we see that this is the place for registering the MsgServer where transactional messages are routed to:

func (am AppModule) RegisterServices (cfg module.Configurator) { types.

RegisterMsgServer (cfg.

MsgServer (), am.keeper) types.

RegisterQueryServer (cfg.

QueryServer (), am.keeper) }

## Impact

Successful governance actions that include a LendingMarketProposal or TreasuryProposal will fail to execute because no handler is provided for them.

## Recommended Mitigation Steps

Consider adding a RegisterMsgServer call in the x/govshuttle RegisterService callback.

dudong2 (Canto) confirmed and commented:

Reasoning:

As your description, the MsgServer isn’t not registered about govshuttle module. Even if gov proposal that include a LendingMarketProposal or TreasuryProposal is passed, the msgs are not executed because there is no handler registered.

Severity:

Mid.

Patch:

We will patch this before the v0.50 production release.

# [M-04] Incorrect names provided in RegisterConcrete calls break LegacyAmino signing method

- **Contest:** Canto
- **Slug:** 2024-05-canto
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-05-canto
- **Source snapshot:** competitions/2024-05-canto/final_report.html

RegisterConcrete calls break LegacyAmino signing method Submitted by 3docSec One of the breaking changes introduced with the Cosmos SDK v0.50.x upgrade is a change in the codec used for Amino JSON (de)serialization. To ensure the new codec behaves as the abandoned one did, the team added amino.name tags to the message types defined in the Canto modules’ “.proto” files.

There are however many instances where these tags are inconsistent with the RegisterConcrete calls made by the in-scope modules’ func (AppModuleBasic) RegisterInterfaces functions, all summarized below:

Module coinswap:

MsgAddLiquidity’s tag in protobuf ( canto/MsgAddLiquidity ) does not match the name registered in code ( coinswap/coinswap/MsgSwapOrder ) MsgRemoveLiquidity’s tag in protobuf ( canto/MsgRemoveLiquidity ) does not match the name registered in code ( coinswap/coinswap/MsgAddLiquidity ) MsgSwapOrder’s tag in protobuf ( canto/MsgSwapOrder ) does not match the name registered in code ( coinswap/coinswap/MsgRemoveLiquidity ) MsgUpdateParams’s tag in protobuf ( canto/MsgUpdateParams ) does not match the name registered in code ( coinswap/coinswap/MsgUpdateParams ) Param’s tag in protobuf (not set) does not match the name registered in code ( coinswap/coinswap/Params ) Module csr:

MsgUpdateParams’s tag in protobuf ( "canto/MsgUpdateParams" ) does not match the name registered in code ( canto/x/csr/MsgUpdateParams ) Param’s tag in protobuf (not set) does not match the name registered in code ( canto/x/csr/Params ) Module erc20:

MsgRegisterCoin’s tag in protobuf ( canto/MsgRegisterCoin ) does not match the name registered in code ( "canto/RegisterCoinProposal" ) MsgRegisterERC20’s tag in protobuf ( canto/MsgRegisterERC20 ) does not match the name registered in code ( "canto/RegisterERC20Proposal" ) Params’s tag in protobuf (not set) does not match the name registered in code ( "canto/Params" ) Module govshuttle:

Module govshuttle has no discrepancy thanks to the fact that the RegisterConcrete call was not made with the Msg types Module inflation:

MsgUpdateParams’s tag in protobuf ( canto/MsgUpdateParams ) does not match the name registered in code ( canto/x/inflation/MsgUpdateParams ) Params’s tag in protobuf (not set) does not match the name registered in code ( canto/x/inflation/Params ) Module onboarding:

MsgUpdateParams’s tag in protobuf ( canto/MsgUpdateParams ) does not match the name registered in code ( canto/x/onboarding/MsgUpdateParams ) Params’s tag in protobuf (not set) does not match the name registered in code ( canto/x/onboarding/Params ) Module evm:

MsgUpdateParams’s tag in protobuf (not set) does not match the name registered in code ( ethermint/MsgUpdateParams ) Module feemarket:

MsgUpdateParams’s tag in protobuf (not set) does not match the name registered in code ( ethermint/feemarket/MsgUpdateParams )

## Impact

All the messages with inconsistent settings listed above, when signed with the LegacyAmino method on a v7 or compatible client, will not be recognized (and consequently rejected) by the Canto app v8 message routing.

## Recommended Mitigation Steps

Consider fixing the RegisterConcrete calls to match the amino.name flags of all the messages enumerated above, which fail the test provided as PoC.

## Assessed type

en/de-code dudong2 (Canto) confirmed and commented:

Reasoning:

Through your test code, we checked that several LegacyAmino has wrong type name. And it can cause AminoJson signing failing.

Severity:

Mid.

Patch:

We will patch this before the v0.50 production release.
