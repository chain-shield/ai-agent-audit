# Benchmark Ground Truth: MANTRA Chain

## Accepted H/M Findings

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

## Rejected Primary Findings

# Rejected Primary Findings: MANTRA Chain

# Fee market governance simulation returns nil messages, breaking proposal testing

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Submission:** F-18
- **Submitter:** Rhaydden
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-chain/submissions/F-18
- **Source snapshot:** competitions/2024-11-mantra-chain/submissions/raw/F-18.txt

## Brief Summary

Fee market governance simulation returns nil messages, breaking proposal testing

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Unhandled panics in Tax Module Genesis and GRPC Operations bricks Mantra

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Submission:** F-9
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-chain/submissions/F-9
- **Source snapshot:** competitions/2024-11-mantra-chain/submissions/raw/F-9.txt

## Brief Summary

The tax module's InitGenesis, ExportGenesis, and RegisterGRPCGatewayRoutes functions use unhandled panics for error handling. This is particularly dangerous as these panics cannot be recovered by standard error handling mechanisms, potentially leading to chain halts or service disruptions. The use of MustUnmarshalJSON and direct panics in critical chain operations like genesis initialization creates significant reliability risks, especially during chain upgrades or network restarts.

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# DOS Vulnerability in BeginBlocker

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Submission:** F-5
- **Submitter:** gxh191
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-chain/submissions/F-5
- **Source snapshot:** competitions/2024-11-mantra-chain/submissions/raw/F-5.txt

## Brief Summary

DOS Vulnerability in BeginBlocker

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Genesis Export Validation Failure Due to Incomplete Field Initialization in the TokenFactory Module

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Submission:** F-73
- **Submitter:** gxh191
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-chain/submissions/F-73
- **Source snapshot:** competitions/2024-11-mantra-chain/submissions/raw/F-73.txt

## Brief Summary

I have decided to escalate this vulnerability to High severity instead of Medium because the inability to restart the chain in other projects is considered Critical. The primary function of createDenomAfterValidation is to execute the logic for creating a denom after completing the fee and denom validation. This function is designed to be used as the second function during the genesis initialization process. However, while creating the Metadata, not all required fields are being set properly. func (k Keeper) createDenomAfterValidation(ctx sdk.Context, creatorAddr, denom string) (err error) { _, exists := k.bankKeeper.GetDenomMetaData(ctx, denom) if !exists { denomMetaData := banktypes.Metad...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Module Version Map Initialization Discrepancy

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Submission:** F-43
- **Submitter:** defsec
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-chain/submissions/F-43
- **Source snapshot:** competitions/2024-11-mantra-chain/submissions/raw/F-43.txt

## Brief Summary

The current implementation in app.go does not properly initialize the version map for all modules during the upgrade keeper initialization. Unlike the reference implementation which explicitly sets the module version map after dependency injection, the code miss modules that aren't wired through dependency injection.

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Valid transactions fail for some tx signers due to the hardcoded tip refund logic

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Submission:** F-79
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-chain/submissions/F-79
- **Source snapshot:** competitions/2024-11-mantra-chain/submissions/raw/F-79.txt

## Brief Summary

Valid transactions fail for some tx signers due to the hardcoded tip refund logic

## Rejection Reason

Final severity/evaluation: Low; Valid Insufficient Primary

# Post handler refund operation might run out of gas, resulting in failed transactions

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Submission:** F-23
- **Submitter:** berndartmueller
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-chain/submissions/F-23
- **Source snapshot:** competitions/2024-11-mantra-chain/submissions/raw/F-23.txt

## Brief Summary

Mantra's xfeemarket post handler runs after a transaction is executed and burns/locks the consumed gas fee, and refunds the unspent gas fee (tip). The issue is that this post handler itself might run out of gas if the user provided a tight gas limit. This is because it uses the same gas meter capped at the gas limit that the user provided. As a result, the transaction might be executed successfully, but the post handler runs out of gas, which causes the whole transaction to fail. This is not desirable as the user's transaction should always succeed if the gas is sufficient but insufficient to run the post handler. In this case, there is no need to refund gas anyway. A similar observation wa...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Missing `WasmKeeper` dependency in `TokenFactory` module breaks `before-send` hook functionality

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Submission:** F-33
- **Submitter:** Rhaydden
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-chain/submissions/F-33
- **Source snapshot:** competitions/2024-11-mantra-chain/submissions/raw/F-33.txt

## Brief Summary

In the Publicly know issues section in the contest readMe, the protocol hints: Issues directly related to cosmos-sdk, IBC, cosmwasm and their dependencies (comet-bft) will not be within the scope of this audit. Basically if it is a issue affecting all cosmos chains of the same versions then it should be out of scope. However, if the issue arise from the custom code we made on our cosmos-sdk, or if the issues are directly affecting behavior of our custom modules, or if our custom modules are causing issues in the base cosmos modules (includes cosmwasm and IBC) then they should be included in the scope. I'd like to point out that the issue in this report is not an issue that affects "all cosm...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Permanent freezing of escrowed fees due to post-handler execution failure

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Submission:** F-45
- **Submitter:** OakSecurity
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-chain/submissions/F-45
- **Source snapshot:** competitions/2024-11-mantra-chain/submissions/raw/F-45.txt

## Brief Summary

The post-handler is designed to refund excessive fees escrowed during the ante-handler phase. While this mechanism works under normal circumstances, an issue arises when the transaction’s gas consumption reaches the maxUtilization limit. In such cases, the post-handler returns an error, and the escrowed fees are not refunded. This issue arises due to how the Cosmos SDK’s BaseApp manages state commits across different transaction phases. Specifically: Fee deduction in ante-handler: During the ante-handler phase, fees are deducted and written to the cacheContext. After this phase, a new cacheContext is created for runTxMsg and the post-handler. State Commit behavior in post-handler: The BaseA...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# When trackBeforeSend is out-of-gas, the user pays less gas fee.

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Submission:** F-7
- **Submitter:** p0wd3r
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-chain/submissions/F-7
- **Source snapshot:** competitions/2024-11-mantra-chain/submissions/raw/F-7.txt

## Brief Summary

In send hook, GasMeter is not updated during trackBeforeSend out-of-gas, which causes the gas value used to calculate the fee to be smaller than the actual value, resulting in undercharging.

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Genesis could be initialized with invalid values because `InitGenesis` sets params without checking if they're valid

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Submission:** F-17
- **Submitter:** Rhaydden
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-chain/submissions/F-17
- **Source snapshot:** competitions/2024-11-mantra-chain/submissions/raw/F-17.txt

## Brief Summary

Genesis could be initialized with invalid values because `InitGenesis` sets params without checking if they're valid

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# DOS Issue Caused by Lack of Gas Limitation in callBeforeSendListener Function during blockBeforeSend

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Submission:** F-24
- **Submitter:** gxh191
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-chain/submissions/F-24
- **Source snapshot:** competitions/2024-11-mantra-chain/submissions/raw/F-24.txt

## Brief Summary

The callBeforeSendListener function is an on-chain contract hook handler in the Cosmos SDK, primarily used to invoke smart contracts under certain conditions. However, when the blockBeforeSend flag is set, it directly calls contractKeeper.Sudo to send a message without enforcing any gas limit. This is a standard contract call and can lead to a potential Denial of Service issue. I will demonstrate this issue in the

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Tokens Starting with IBC Not Checked During Genesis Validation

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Submission:** F-72
- **Submitter:** gxh191
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-chain/submissions/F-72
- **Source snapshot:** competitions/2024-11-mantra-chain/submissions/raw/F-72.txt

## Brief Summary

This function is responsible for performing basic validation of parameters and tokens (Denom) in the genesis state, ensuring that the system starts in a consistent and secure state. Below is a detailed, section-by-section analysis of the function. It calls the validateDenom function to validate each token individually. // Validate performs basic genesis state validation returning an error upon any // failure. func (gs GenesisState) Validate() error { err := gs.Params.Validate() if err != nil { return err } seenDenoms := make(map[string]bool) for _, denom := range gs.GetFactoryDenoms() { if err := validateDenom(denom, seenDenoms); err != nil { // <---------- return err } } return nil } The v...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# The tax module parameters can never be updated using governance proposals

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Submission:** F-3
- **Submitter:** 0xAlix2
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-chain/submissions/F-3
- **Source snapshot:** competitions/2024-11-mantra-chain/submissions/raw/F-3.txt

## Brief Summary

The tax module is responsible for managing and allocating taxes within the blockchain network. At the start of each block, AllocateMcaTax is called which transfers a part of the fee collector balance to a specified address, called McaAddress. func (k Keeper) AllocateMcaTax(ctx context.Context, mcaTax math.LegacyDec, mcaAddress sdk.AccAddress) error { // ... // transfer allocated mca tax to the specified account err := k.bankKeeper.SendCoinsFromModuleToAccount(ctx, k.feeCollectorName, mcaAddress, mcaTaxAllocation) if err != nil { return err } // ... } The protocol allows updating the parameters of the tax module through governance proposals, according to the docs: The tax module parameters c...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Unbounded query in TokenFactory's `DenomsFromCreator` endpoint creates DoS vector

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Submission:** F-26
- **Submitter:** Rhaydden
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-chain/submissions/F-26
- **Source snapshot:** competitions/2024-11-mantra-chain/submissions/raw/F-26.txt

## Brief Summary

The TokenFactory module has an unbounded query endpoint DenomsFromCreator that retrieves all denoms associated with a creator address without any pagination limits. This creates a DOS vector where an attacker could: Create many token denoms associatted with an address (within transaction/gas limits) When querying these denoms through the GRPC endpoint, the protocool will attempt to load and return all denoms at once Cause memory bloat and potential node crashes due to out-of-memory errors The root cause is in the getDenomsFromCreator keeper function which performs an unbounded iteration over all denoms, combined with its exposure through an unpaginated gRPC query endpoint. Impact Potential...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Mint and burn functions are unusable from the CLI

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Submission:** F-2
- **Submitter:** 0xAlix2
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-chain/submissions/F-2
- **Source snapshot:** competitions/2024-11-mantra-chain/submissions/raw/F-2.txt

## Brief Summary

Token factory allows users to create custom denoms. When a user makes a denom, he automatically becomes the admin of that denom, this will enable him to mint/burn/transfer tokens to/from/between users, this is mentioned in the docs: Once a denom is created, the original creator is given "admin" privileges over the asset. This allows them to: Mint their denom to any account Burn their denom from any account The mint message has the following type: type MsgMint struct { Sender string `protobuf:"bytes,1,opt,name=sender,proto3" json:"sender,omitempty" yaml:"sender"` Amount types.Coin `protobuf:"bytes,2,opt,name=amount,proto3" json:"amount" yaml:"amount"` MintToAddress string `protobuf:"bytes,3,...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Silently ignored config errors in root command initialization which could corrupt configurations

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Submission:** F-25
- **Submitter:** Rhaydden
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-chain/submissions/F-25
- **Source snapshot:** competitions/2024-11-mantra-chain/submissions/raw/F-25.txt

## Brief Summary

In the NewRootCmd function, configuration errors are silently ignored during the AutoCLI setup. The happens when calling config.ReadFromClientConfig without proper error handling: >>> initClientCtx, _ = config.ReadFromClientConfig(initClientCtx) autoCliOpts.ClientCtx = initClientCtx Now, this bug occurs in three layers: a) Configuration Layer: ReadFromClientConfig is responsible for loading client configuration settings from the config contract. Settings include params like chainID, node RPC endpoints, output format, keyring backend configuration, tther client-specific settings b) Error Handling Layer: The first usage properly handles errors by returning them to the caller(see below for

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Oracle will fail to post prices to Mantrachain due to missing vote extensions enable height

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Submission:** F-63
- **Submitter:** crypticdefense
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-chain/submissions/F-63
- **Source snapshot:** competitions/2024-11-mantra-chain/submissions/raw/F-63.txt

## Brief Summary

Mantrachain currently registers the skip-mev connect oracle within the application: app.go#L144-L146 oracle "github.com/skip-mev/connect/v2/x/oracle" oraclekeeper "github.com/skip-mev/connect/v2/x/oracle/keeper" oracletypes "github.com/skip-mev/connect/v2/x/oracle/types" app.go#L585-L590 oracleKeeper := oraclekeeper.NewKeeper(runtime.NewKVStoreService(keys[oracletypes.StoreKey]), appCodec, app.MarketMapKeeper, authtypes.NewModuleAddress(govtypes.ModuleName)) app.OracleKeeper = &oracleKeeper oracleModule := oracle.NewAppModule(appCodec, *app.OracleKeeper) Both Cosmos sdk and Connect docs explain that Vote Extension (enable height) must be enabled for the oracle to post prices to the chain, o...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Less than MCA tax would be charged cumulatively causing a leak of value when allocating rewards to the MCA address

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Submission:** F-13
- **Submitter:** Bauchibred
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** known_issue
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-chain/submissions/F-13
- **Source snapshot:** competitions/2024-11-mantra-chain/submissions/raw/F-13.txt

## Brief Summary

Less than MCA tax would be charged cumulatively causing a leak of value when allocating rewards to the MCA address

## Rejection Reason

Marked known issue in authenticated Code4rena submission detail/table.

# Creating Denoms would be impossible for valid subdenoms

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Submission:** F-58
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-chain/submissions/F-58
- **Source snapshot:** competitions/2024-11-mantra-chain/submissions/raw/F-58.txt

## Brief Summary

Creating Denoms would be impossible for valid subdenoms

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Bug in skip-mev Connect affecting correct prices on Mantra

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Submission:** F-36
- **Submitter:** 0x41
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-chain/submissions/F-36
- **Source snapshot:** competitions/2024-11-mantra-chain/submissions/raw/F-36.txt

## Brief Summary

Note: The bug is not in Mantra itself, but skip-mev. However, it can lead to wrong prices on Mantra. Therefore, the following from the README seems to apply: However, if the issue arise from the custom code we made on our cosmos-sdk, or if the issues are directly affecting behavior of our custom modules, or if our custom modules are causing issues in the base cosmos modules (includes cosmwasm and IBC) then they should be included in the scope. Finding description and impact oracle.go integrates the connect oracle from skip-mev. This oracle has various providers and integration options such as Invert (to invert the price before returning), which Mantra may use (and currently does for some fe...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Failing to check `iterator.Close()` return errors can leave DB locked and break modules

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Submission:** F-65
- **Submitter:** crypticdefense
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-chain/submissions/F-65
- **Source snapshot:** competitions/2024-11-mantra-chain/submissions/raw/F-65.txt

## Brief Summary

Cosmos sdk docs explain that when using an iterator, "Keys/Values/KeyValues fully consume and close the Iterator, here we need to explicitly do a defer iterator.Close() call." If we take a look at the iterator interface, the Close() method returns an error: types.go#L158 // Close closes the iterator, relasing any allocated resources. Close() error The error returned does not always have to be checked directly from the Close() call, as long as any errors returned are checked after, for example (direct example from the link above): // var itr Iterator = ... // defer itr.Close() // // for ; itr.Valid(); itr.Next() { // k, v := itr.Key(); itr.Value() // ... // } // // if err := itr.Error(); err...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Unsafe oracle client initialization with uncontrolled goroutine

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Submission:** F-29
- **Submitter:** Rhaydden
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-chain/submissions/F-29
- **Source snapshot:** competitions/2024-11-mantra-chain/submissions/raw/F-29.txt

## Brief Summary

The oracle client initialization in the client connection is established in an uncontrolled goroutine without proper synchronization. This causes issues like: Race Conditions because the function returns an oracle client that may not be fully initialized, as the connection process happens asynchronously. Connection failures result in a panic in a separate goroutine, which may not be properly caught by the application's error handling. The application might proceed with an unconnected oracle client, potentially causing failed price updates or invalid vote extensions. The impact is severe as the oracle client is used for vote extension, price data aggregation and to block proposal validation

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Custom DenomResolver leaves fee tokens stuck in escrow

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Submission:** F-75
- **Submitter:** OakSecurity
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-chain/submissions/F-75
- **Source snapshot:** competitions/2024-11-mantra-chain/submissions/raw/F-75.txt

## Brief Summary

The feemarket module’s flow begins in the anteHandler, where transaction fees are escrowed, and concludes in the postHandler, where fees are burned and tips refunded. However, if fees are paid in a denomination different from defaultFeeDenom, the postHandler does not burn these tokens, leaving them stuck in the escrow account. Consequently, in case the custom DenomResolver is used instead of the default one to allow users to pay fees in multiple denominations, it results in perpetual accumulation of tokens in the module escrow address, effectively making those fees inaccessible and rendering them unusable.

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Tokenfactory bank hooks will not be able to work in conjuction with depinject

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Submission:** F-66
- **Submitter:** crypticdefense
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** known_issue
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-chain/submissions/F-66
- **Source snapshot:** competitions/2024-11-mantra-chain/submissions/raw/F-66.txt

## Brief Summary

There is a known issue with the tokenfactory module used in Mantrachain, where bank hooks aren't wired to be depinject compatible due to a missing SetHooks() call. Mantrachain attempted to address this issue in pull #160 by discontinuing the use of depinject. However, it was not fully discontinued, as the tokenfactory module continues to use depinject, causing these bank hooks to fail.

## Rejection Reason

Marked known issue in authenticated Code4rena submission detail/table.

# BankSendGasConsumption is incorrectly calculated for the case when the denom is the default fee denom

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Submission:** F-20
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-chain/submissions/F-20
- **Source snapshot:** competitions/2024-11-mantra-chain/submissions/raw/F-20.txt

## Brief Summary

In the current implementation of the fee.go, the BankSendGasConsumption value is the gas consumption of the bank sends that occur during feemarket handler execution. It has the value of 12490 plus additional 37325 in the case when fees are burnt and not transferred. The problem is that it's not always the case and the fees are burnt only when it's the default fee denom. As the fee market supports different denoms, some of the fees will be just locked.

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# The `tokenfactory` administrator function `UpdateParams` cannot be called from the outside.

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Submission:** F-50
- **Submitter:** zhaojie
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-chain/submissions/F-50
- **Source snapshot:** competitions/2024-11-mantra-chain/submissions/raw/F-50.txt

## Brief Summary

The tokenfactory administrator function UpdateParams cannot be called from the outside, causing the administrator function to become invalid.

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Fee market's `GasPrices` query returns inflated gas price for default fee coin

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Submission:** F-69
- **Submitter:** berndartmueller
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-chain/submissions/F-69
- **Source snapshot:** competitions/2024-11-mantra-chain/submissions/raw/F-69.txt

## Brief Summary

ExtraDenoms() returns all coin denoms that have a multiplier configured, i.e., all allowed fee coins. It is used as part of skip-mev's feemarket GetMinGasPrices() in line 117 to query the minimum gas prices for all allowed fee coins. Note that the default fee coin is supposed to be separate from the extra denoms, as it is already added to minGasPrices: 114: minGasPrice := sdk.NewDecCoinFromDec(params.FeeDenom, baseGasPrice) 115: minGasPrices := sdk.NewDecCoins(minGasPrice) 116: 117: extraDenoms, err := k.resolver.ExtraDenoms(ctx) 118: // [...] Subsequently, the extra denoms are retrieved and the base gas fee converted for each of them before adding them to minGasPrices: 122: for _, denom :=...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Invariant of not allowing transfers from modules is broken for the tax module

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Submission:** F-15
- **Submitter:** Bauchibred
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-chain/submissions/F-15
- **Source snapshot:** competitions/2024-11-mantra-chain/submissions/raw/F-15.txt

## Brief Summary

Invariant of not allowing transfers from modules is broken for the tax module

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# `tokenfactory::Mint` msg emit wrong `mint_to_address`, which may seriously impact off-chain integrations

- **Contest:** MANTRA Chain
- **Slug:** 2024-11-mantra-chain
- **Submission:** F-37
- **Submitter:** Egis_Security
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-chain/submissions/F-37
- **Source snapshot:** competitions/2024-11-mantra-chain/submissions/raw/F-37.txt

## Brief Summary

Mint msg in tokenfactory module takes care of minting tokens to address specified by the caller sender of the transaction. The problem is inside the event emission in the end of the msgServer corresponding function: ctx.EventManager().EmitEvents(sdk.Events{ sdk.NewEvent( types.TypeMsgMint, sdk.NewAttribute(types.AttributeMintToAddress, msg.Sender), sdk.NewAttribute(types.AttributeAmount, msg.Amount.String()), ), }) You can see that we will set mint_to_address field of the event to the address of the sender of the transaction. The following will result in compromised data being emitted on each Mint msg, which is in the core logic of the project. Note that the same is also present in Burn For...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient
