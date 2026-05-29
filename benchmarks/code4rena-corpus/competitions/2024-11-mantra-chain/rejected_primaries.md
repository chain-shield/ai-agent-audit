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
