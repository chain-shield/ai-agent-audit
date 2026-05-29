# Accepted H/M Findings: MANTRA Chain

# [H-01] BlockBeforeSend hook can be exploited to perform a denial of service

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-chain
- **Source snapshot:** competitions/2024-11-mantra-chain/final_report.html

BlockBeforeSend hook can be exploited to perform a denial of service Submitted by OakSecurity, also found by gxh191, p0wd3r, and Egis_Security

## Finding description and impact

The MsgSetBeforeSendHook in the tokenfactory module allows the token creator to set a BeforeSendHook tied to a CosmWasm contract address, which implements a custom logic from the Sudo handler that determines whether the transfer should succeed.

The issue is that a malicious token creator can force transfers to fail by simply setting the contract address to an invalid address (e.g., an externally owned account). If these tokens are intended to be transferred during important operations (e.g., staking hooks or ABCI phases), they will fail, and depending on the calling context, a denial of service issue may occur.

We have identified two impacts that can be triggered by the attacker.

The attacker can force delegators’ staked funds and rewards to be stuck and cannot be withdrawn. This scenario is demonstrated in the POC section below.

The attacker creates a tokenfactory denom.

The attacker mints some tokens for themselves.

The attacker sends the minted tokens to a validator with MsgDepositValidatorRewardsPool. This will cause the validator and delegators’ pending rewards to include the tokens.

The attacker calls MsgSetBeforeSendHook and sets the hook address to an invalid address.

At this point, all staking actions will now fail. Delegators cannot unbond their tokens and cannot withdraw their rewards, causing their bonded funds to be stuck.

The logic behind this is that all staking actions must call the BeforeDelegationSharesModified hook to distribute the pending rewards before updating the bonded amount in order to compute the rewards correctly. However, since the rewards cannot be transferred due to the malicious hook, the BeforeDelegationSharesModified hook will fail, preventing the staking actions from succeeding.

The attacker can trigger a chain halt by performing a redelegation from a misbehaved validator. This attack is similar to the above, weaponizing the impact of BeforeDelegationSharesModified hook failure but achieving a higher impact by forcefully causing the BeginBlocker to panic, ultimately halting the chain.

The attacker creates a tokenfactory denom.

The attacker mints some tokens for themselves.

The attacker sends the minted tokens to a validator with MsgDepositValidatorRewardsPool. This will cause the validator and delegators’ pending rewards to include the tokens.

The validator misbehaves (e.g., downtime/double-signing) and will be slashed in the next few blocks.

One of the delegators performs a redelegation with MsgBeginRedelegate and redelegates their tokens to other validators. This is considered a normal action when delegators prefer to bond their tokens to other validators.

- https://github.com/MANTRA-Chain/cosmos-sdk/blob/cfc36838ae32a2772d86bafa0cb192125f0ffbaf/x/staking/keeper/msg_server.go#L316
The attacker calls MsgSetBeforeSendHook and sets the hook address to an invalid address.

The automatic slashing mechanism will be initiated from the BeginBlocker to slash the validator for a percentage amount of bonded funds (recall that they misbehaved previously).

Double signing slash:

- https://github.com/MANTRA-Chain/cosmos-sdk/blob/cfc36838ae32a2772d86bafa0cb192125f0ffbaf/x/evidence/keeper/abci.go#L32-L37
Downtime slash:

- https://github.com/MANTRA-Chain/cosmos-sdk/blob/cfc36838ae32a2772d86bafa0cb192125f0ffbaf/x/slashing/abci.go#L24
The slashing mechanism will also slash the delegator who redelegated their funds to another validator. This is intended because the delegation contributed to the validator’s bonded tokens when they misbehaved.

- https://github.com/MANTRA-Chain/cosmos-sdk/blob/cfc36838ae32a2772d86bafa0cb192125f0ffbaf/x/staking/keeper/slash.go#L130-L141
Slashing redelegations works by forcefully unbond some tokens from the destination validator. However, since the Unbond function calls the BeforeDelegationSharesModified hook and the reward tokens cannot be transferred, an error will be returned and propagated back to the BeginBlocker. This triggers a chain halt when the ABCI receives an error from the chain, causing a denial of service.

The flow works as BeginBlock -> x/evidence, x/slashing BeginBlocker -> handleEquivocationEvidence/HandleValidatorSignature -> SlashWithInfractionReason -> Slash -> SlashRedelegation -> Unbond -> BeforeDelegationSharesModified -> withdrawDelegationRewards -> error.

## Recommended mitigation steps

Consider introducing a whitelist or access control mechanism to limit who can register a BeforeSendHook. Ensure only trusted users with appropriate permissions should be allowed to set a hook for a specific denomination.

For example, Neutron only allows whitelisted users to use this feature:

- https://github.com/neutron-org/neutron/blob/5ff5272e0a920b6874036db292a0c8debba99102/x/tokenfactory/keeper/msg_server.go#L253
Note: this finding was redacted upon submission and was populated by C4 staff to proceed through the audit process.

3docSec (judge) commented:

Looks valid.

Lance Lan (MANTRA) disputed and commented:

This vulnerability is valid overall; however, most of the submissions about the vector of attack are not valid/possible.

S-124 is the one that is most serious and valid. It has been addressed with mainnet proposals to block related messages from being executed instead of a code change.

We may decide to fix it in the future, but for now we will leave it as it is.

# [H-02] Unspent gas fees are always refunded to the FeePayer() which leads to incorrect refunds if the FeeGranter() paid for the fees

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-chain
- **Source snapshot:** competitions/2024-11-mantra-chain/final_report.html

FeePayer() which leads to incorrect refunds if the FeeGranter() paid for the fees Submitted by berndartmueller, also found by ABAIKUNANBAEV, abdulsamijay, Bauchibred, Egis_Security, and OakSecurity

## Finding description and impact

FeeMarketDeductDecorator.PostHandle() is called after the transaction is executed to refund the paid but unspent gas.

The issue is that the refund is always sent to the FeePayer(), which is not the address that paid for the gas if a fee granter ( FeeGranter() ) paid for the tx fees instead.

In this case, the fee granter will not receive the refund, the fee payer (i.e., the sender) will receive the refund instead. This would slowly drain the fee granter’s account if they are paying for tx fees on behalf of others. Or, if the fee granter assumes to receive the refund, only paying the actual consumed gas fees, and thus only charging the user for the consumed gas fees (afterwards, settled off-chain, e.g., via credit card), they would make a loss.

## Recommended mitigation steps

Consider determining the actual fee payer, which is either the FeePayer() or the FeeGranter(), and refund the unused gas fees to the correct address.

For example, see Celestia’s solution:

- https://github.com/rootulp/celestia-app/blob/d10fdbd4e5507f8449747a2388c4657f593b691b/app/posthandler/refund_gas_remaining.go#L133-L138
Lance Lan (MANTRA) confirmed and commented:

Valid. Grantee can drain all granter’s granted funds; however, most of the granter grantee relationship are internal so the impact is relatively low.

Lance Lan (MANTRA) commented:

Regarding S-203: yes, we have found this bug prior to it being reported and will be patched in next upgrade. This should be medium-high as all feegranter feegrantee relationships are mostly internal now.

# [H-03] Refunding the full gas fee tip allows inflating the transaction priority and filling up the block gas limit, effectively DoSing the network

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-chain
- **Source snapshot:** competitions/2024-11-mantra-chain/final_report.html

Submitted by berndartmueller

## Finding description and impact

skip-mev’s feemarket ante handler escrows the full paid fee amount (which includes the base gas fee + tip). Any paid gas fee that is not consumed is fully refunded to the fee payer in Mantra’s xfeemarket post handler.

This is problematic in two ways:

According to the genesis.json configuration for Mantra, the consensus parameter for the maximum block gas max_gas is set to 75_000_000. This means that blocks will have a 75M gas limit and will only contain transactions that have a cumulative gas consumption of less than or equal to 75M. This is ensured by using a block gas meter, as seen when preparing a block and when finalizing a block. Therefore, knowing that unspent gas is fully refunded, an attacker can submit a tx with a gas limit of near 75M, consume only a small fraction of it, and get the rest refunded. This can be repeated to fill up the block with transactions that consume only a small fraction of the gas limit, effectively DoSing the network.

A transaction’s priority in the mempool is determined based on the paid fee. By purposefully overpaying, a transaction can always have a very high priority.

## Recommended mitigation steps

Consider only partially refunding the gas fee tip to the fee payer. This would disincentivize users from overpaying the gas fee.

This has also been suggested as one of the mitigations by Celestia.

Note: this finding was redacted upon submission and was populated by C4 staff to proceed through the audit process.

Lance Lan (MANTRA) confirmed 3docSec (judge) commented:

Looks valid.

# [H-04] Multiplier is calculated using denom and not coin.Denom

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Finding ID:** H-04
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-chain
- **Source snapshot:** competitions/2024-11-mantra-chain/final_report.html

denom and not coin.Denom Submitted by ABAIKUNANBAEV, also found by berndartmueller

## Finding description and impact

Currently, there is a resolver that helps to convert different coins to the corresponding amount of the base coin to ensure multiple fee coin market can be supported. However, it incorrectly fetches the multiplier using the base denom instead of taking denom of the denom in the function.

## Recommended mitigation steps

Change the current functionality on the following line:

- https://github.com/code-423n4/2024-11-mantra/blob/main/x/xfeemarket/keeper/resolver.go#L16
multiplier, err:= k.DenomMultipliers.Get(ctx, coin.Denom) 3docSec (judge) commented:

Plausible, can’t exclude a misunderstanding.

Lance Lan (MANTRA) confirmed and commented:

Indeed a error. But this resolver is not wired in and is not used. It is not the final implementation thus Low.

3docSec (judge) commented:

The rules we have on speculation on future code (or in this case future configuration, that is enabling support for multi-token transaction fees) are clear: the feature is in scope unless said otherwise in the audit README.

Medium Risk Findings (4)

# [M-01] Fee market post handler misses to account for gas consumed after taking the gas snapshot, leading to higher refunds and unpaid gas

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-chain
- **Source snapshot:** competitions/2024-11-mantra-chain/final_report.html

Submitted by berndartmueller

## Finding description and impact

Mantra’s xfeemarket post handler refunds unspent gas after the transaction is executed. For this, in line 99, a snapshot of the gas consumed is taken, which is then used to calculate the refund ( tip ) via skip-mev’s CheckTxFee() in line 112.

However, this gas snapshot does not account for any gas consumed after the snapshot is taken, such as the gas consumed by burning the gas fees and the bank transfer that refunds the tip. As a result, the user is not charged for the gas consumed by these operations, leading to unaccounted gas consumption and potential abuse.

## Recommended mitigation steps

Consider defining a gas constant for those operations (e.g., BankSendGasConsumption = 12490 + 37325, which is only used during simulations) and charge the user for the gas consumed by them. This will prevent potential abuse and ensure that the user is charged for all gas consumed during the transaction.

3docSec (judge) commented:

Maybe a misunderstanding. Looking at the code it seems to me that the refund is lower than it should (gas consumed in the burn/lock and refund operations is paid but not refunded).

Lance Lan (MANTRA) disputed and commented:

L99 is actually just the gas limit, the naming is confusing. L112 it calculates the payCoin and tip which the tip is refunded. I agree that operations the burn and refund is not paid gas; however, this is very low impact.

3docSec (judge) commented:

Marking as valid Low. The warden is welcome to provide

# [M-02] Block gas utilization is slightly lower than the actual gas utilization due to the gas snapshot being taken too early, resulting in an inaccurate base fee calculation

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-chain
- **Source snapshot:** competitions/2024-11-mantra-chain/final_report.html

Submitted by berndartmueller

## Finding description and impact

xfeemarket implements a custom PostHandler() for skip-mev’s feemarket. At almost the very end of the execution, the feemarket’s state is updated with the consumed gas amount so that the current block’s gas utilization and, thus, the base fee can be calculated.

However, the gas snapshot that is provided is not 100% accurate. It is taken in line 81. But any gas consumed by the subsequent actions, e.g., GetMinGasPrice() in line 101, or ante.CheckTxFee() in line 112, are not accounted for in the gas snapshot.

As a result, the block utilization for the current height is inaccurate, leading to a slightly lower base fee than it should be.

## Recommended mitigation steps

Consider taking the snapshot right before calling state.Update().

3docSec (judge) commented:

Looks valid, may be worth verifying if the excluded operations actually cost gas.

Lance Lan (MANTRA) confirmed and commented:

Is indeed valid as the excluded operations cost gas. We followed skip-mev implementation where they have the same issue. Low at best as this has no chance for exploits and would not come up in normal circumstance. Even if it does, it has limited impact as users just have to do the transaction with more slightly gas.

berndartmueller (warden) commented:

Here’s a simple PoC that shows how much gas is left out to account for the block utilization.

Add lines after x/xfeemarket/post/fee.go#L99 feeGas:= int64(feeTx.GetGas()) + snapshot:= ctx.GasMeter().GasConsumed() and after L142 + // figure out the diff of gas consumed since snapshot + gasDiff:= ctx.GasMeter().GasConsumed() - snapshot + fmt.Println("gasDiff", gasDiff) Copy paste the following test case to x/xfeemarket/post/fee_test.go func TestPostHandle_Audit (t *testing.T) { // Same data for every test case const ( baseDenom = "stake" resolvableDenom = "atom" expectedConsumedGas = 59122 gasLimit = 100000 ) validFeeAmount:= types.DefaultMinBaseGasPrice.

MulInt64 ( int64 (gasLimit)) validFee:= sdk.

NewCoins (sdk.

NewCoin (baseDenom, validFeeAmount.

TruncateInt ())) testCases:= []postsuite.TestCase{ { Name:

"signer has enough funds, should pass, with tip", Malleate:

func (s *postsuite.TestSuite) postsuite.TestCaseArgs { accs:= s.

CreateTestAccounts ( 1 ) balance:= postsuite.TestAccountBalance{ TestAccount: accs[ 0 ], Coins: validFee, } s.

SetAccountBalances ([]postsuite.TestAccountBalance{balance}) return postsuite.TestCaseArgs{ Msgs: []sdk.Msg{testdata.

NewTestMsg (accs[ 0 ].Account.

GetAddress ())}, GasLimit: gasLimit, FeeAmount: validFee, } }, RunAnte:

true, RunPost:

true, Simulate:

false, ExpPass:

true, ExpErr:

nil, ExpectConsumedGas:

34836, Mock:

false, }, } for _, tc:= range testCases { t.

Run (fmt.

Sprintf ( "Case %s", tc.Name), func (t *testing.T) { s:= postsuite.

SetupTestSuite (t, tc.Mock) s.TxBuilder = s.ClientCtx.TxConfig.

NewTxBuilder () args:= tc.

Malleate (s) s.

RunTestCase (t, tc, args) s.

Fail ( "fail" ) }) } Running the test logs shows the gas diff since taking a snapshot:

gasDiff 31368 To put this into context:

A simple bank send tx, consumed 151,517 gas, that’s 31,368 / 151,517 = ~21% of the gas of such a tx.

When Mantra chain is heavily utilized, this gas inaccuracy is not negligible and results in underestimated base fees, failing to properly increase gas fees to counteract high utilization (and e.g. spam). Therefore, I kindly ask the judge to reconsider the severity and consider Medium appropriate.

3docSec (judge) commented:

Agreed, 21% of the gas accounting being off for reasonably popular transactions is enough impact for Medium severity.

# [M-03] xfeemarket module is not wired up, resulting in non-working CLI commands, message server, genesis export

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-chain
- **Source snapshot:** competitions/2024-11-mantra-chain/final_report.html

xfeemarket module is not wired up, resulting in non-working CLI commands, message server, genesis export Submitted by berndartmueller, also found by abdulsamijay and Egis_Security

## Finding description and impact

A Cosmos SDK app is set up in app/app.go, where all modules are wired up. Some of the steps include:

module.NewManager(..) initializes the module manager. For example, it ensures that the module’s CLI commands are registered and available from the binary genesisModuleOrder specifies the order of the modules for exporting and initializing genesis state storetypes.NewKVStoreKeys(..) registers all the store keys for the various modules However, it has been observed that Mantra’s custom xfeemarket module is not properly registered. It is missing in the steps above. As a result, the module’s CLI commands are not available and do not work. The message server is not registered, which prevents, for example, using the MsgUpsertFeeDenom and MsgRemoveFeeDenom messages to update the fee multipliers. The module’s state is not exported or initialized in the genesis state.

This prevents the module from being fully functional, especially from configuring fee multipliers to be able to use various coin denoms as gas fees.

## Recommended mitigation steps

To ensure that the custom xfeemarket module is fully functional, consider the following diff to wire up the module in app/app.go:

diff --git a/app/app.go b/app/app.go index bfaa4a7..7f4d4bb 100644 --- a/app/app.go +++ b/app/app.go @@ -43,6 +43,8 @@ import ( "github.com/MANTRA-Chain/mantrachain/x/tokenfactory" tokenfactorykeeper "github.com/MANTRA-Chain/mantrachain/x/tokenfactory/keeper" tokenfactorytypes "github.com/MANTRA-Chain/mantrachain/x/tokenfactory/types" + xfeemarketkeeper "github.com/MANTRA-Chain/mantrachain/x/xfeemarket/keeper" + xfeemarket "github.com/MANTRA-Chain/mantrachain/x/xfeemarket/module" xfeemarkettypes "github.com/MANTRA-Chain/mantrachain/x/xfeemarket/types" abci "github.com/cometbft/cometbft/abci/types" tmproto "github.com/cometbft/cometbft/proto/tendermint/types" @@ -254,8 +256,9 @@ type App struct {

ScopedFeeabsKeeper capabilitykeeper.ScopedKeeper // MANTRAChain keepers - TokenFactoryKeeper tokenfactorykeeper.Keeper - TaxKeeper taxkeeper.Keeper + TokenFactoryKeeper tokenfactorykeeper.Keeper + TaxKeeper taxkeeper.Keeper + CustomFeeMarketKeeper xfeemarketkeeper.Keeper // the module manager ModuleManager *module.Manager @@ -321,6 +324,7 @@ func New( tokenfactorytypes.StoreKey, taxtypes.StoreKey, ibchookstypes.StoreKey, feemarkettypes.StoreKey, oracletypes.StoreKey, marketmaptypes.StoreKey, + xfeemarkettypes.StoreKey, ) tkeys:= storetypes.NewTransientStoreKeys(paramstypes.TStoreKey) @@ -564,6 +568,14 @@ func New( authtypes.FeeCollectorName, ) + app.CustomFeeMarketKeeper = xfeemarketkeeper.NewKeeper(

+ appCodec, + runtime.NewKVStoreService(keys[xfeemarkettypes.StoreKey]), + logger, + authtypes.NewModuleAddress(govtypes.ModuleName).String(), + app.BankKeeper, + ) + app.FeeMarketKeeper = feemarketkeeper.NewKeeper( appCodec, keys[feemarkettypes.StoreKey], app.AccountKeeper, @@ -759,6 +771,7 @@ func New( tokenfactory.NewAppModule(appCodec, app.TokenFactoryKeeper), tax.NewAppModule(appCodec, app.TaxKeeper), feemarket.NewAppModule(appCodec, *app.FeeMarketKeeper), + xfeemarket.NewAppModule(appCodec, app.CustomFeeMarketKeeper, app.AccountKeeper, app.BankKeeper), ) // BasicModuleManager defines the module BasicManager is in charge of setting up basic, @@ -811,6 +824,7 @@ func New( tokenfactorytypes.ModuleName,

oracletypes.ModuleName, marketmaptypes.ModuleName, + xfeemarkettypes.ModuleName, ) app.ModuleManager.SetOrderEndBlockers( @@ -833,6 +847,7 @@ func New( taxtypes.ModuleName, oracletypes.ModuleName, marketmaptypes.ModuleName, + xfeemarkettypes.ModuleName, ) // NOTE: The genutils module must occur after staking so that pools are @@ -881,6 +896,7 @@ func New( // market map genesis must be called AFTER all consuming modules (i.e. x/oracle, etc.) oracletypes.ModuleName, marketmaptypes.ModuleName, + xfeemarkettypes.ModuleName, } app.ModuleManager.SetOrderInitGenesis(genesisModuleOrder...) app.ModuleManager.SetOrderExportGenesis(genesisModuleOrder...) 3docSec (judge) commented:

Looks valid.

Lance Lan (MANTRA) acknowledged and commented:

Intended behavior. Will only be wired in future when we want to support multi fee denoms.

berndartmueller (warden) commented:

According to the audit readme, “Multi-token support for transaction fees” is one of the listed features of Mantra Chain. Thus, it can be assumed that any issues related to preventing the use of other fee coins for gas payments are valid and their severity should be evaluated accordingly. There is also no hint that this feature will only be used later in the future.

Therefore, I would like the judge to reconsider the severity of this issue and consider Medium severity.

3docSec (judge) commented:

The rules we have on speculation on future code (or in this case future configuration, that is enabling support for multi-token transaction fees) are clear: the feature is in scope unless said otherwise in the audit README.

# [M-04] Resolver is not initialized in the protocol’s keeper

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-chain
- **Source snapshot:** competitions/2024-11-mantra-chain/final_report.html

Submitted by ABAIKUNANBAEV, also found by ABAIKUNANBAEV and zhaojie

## Finding description and impact

At the moment, resolver is not initialized in the feemarket/keeper resulting in a situation where ante from the skip-mev would use its own ConvertToDenom() and not the one initialized in the Mantra protocol.

## Recommended mitigation steps

Initialize the resolver in the feemarket/keeper.go.

3docSec (judge) commented:

Looks valid.

Lance Lan (MANTRA) disputed and commented:

xfeemarket keeper and resolver is not wired in as we only plan on using it in the future once we move to support multi fee denom.

3docSec (judge) commented:

Closing as intended behavior / insufficient proof it shouldn’t be.

ABAIKUNANBAEV (warden) commented:

Similar to H-04, if that is considered a valid issue, this one should also be reconsidered and given a medium severity.

Per audit README, multi-token fee support is to be audited, meaning all the issues regarding it should be taken into account.

3docSec (judge) commented:

More than H-04, I consider this finding closer to M-03, so a similar sort seems reasonable.
