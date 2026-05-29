# Accepted H/M Findings: HydraDX

# [H-01] An attacker possesses the capability to exhaust the entirety of liquidity within the stable swap pools by manipulating the buy function, specifically by setting the asset_in parameter equal to the asset_out parameter

- **Contest:** HydraDX
- **Slug:** 2024-02-hydradx
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-02-hydradx
- **Source snapshot:** competitions/2024-02-hydradx/final_report.html

asset_in parameter equal to the asset_out parameter Submitted by castle_chain, also found by bin2chen

- https://github.com/code-423n4/2024-02-hydradx/blob/603187123a20e0cb8a7ea85c6a6d718429caad8d/HydraDX-node/pallets/stableswap/src/lib.rs#L787-L842
- https://github.com/code-423n4/2024-02-hydradx/blob/603187123a20e0cb8a7ea85c6a6d718429caad8d/HydraDX-node/math/src/stableswap/math.rs#L40-L41

## Impact

This vulnerability has been identified in the Stableswap pallet that could potentially drain all liquidity from all pools without any permissions. This vulnerability can be exploited by malicious actors, resulting in significant financial losses for both the protocol and liquidity providers.

## Recommended Mitigation Steps

To mitigate this vulnerability, it is crucial to prevent the setting of asset_in equal to asset_out. This can be achieved by adding the following line to the buy() function:

pub fn buy( origin: OriginFor<T>, pool_id: T::AssetId, asset_out: T::AssetId, asset_in: T::AssetId, amount_out: Balance, max_sell_amount: Balance, ) -> DispatchResult { let who = ensure_signed(origin)?; + ensure!( + asset_out != asset_in, Error::<T>::Invalid + ); Integrating this check into the buy() function will effectively prevent attackers from draining liquidity from the pool.

## Assessed type

Invalid Validation enthusiastmartin (HydraDX) confirmed and commented:

Nice one!

Medium Risk Findings (10)

# [M-01] Users can MAKE EMA-Oracle price outdated with direct transfers to StableSwap

- **Contest:** HydraDX
- **Slug:** 2024-02-hydradx
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-hydradx
- **Source snapshot:** competitions/2024-02-hydradx/final_report.html

Submitted by J4X The EMA oracle, designed to utilize HydraDX’s Omnipools and StableSwap for exchange rate information, operates by monitoring activities within these liquidity pools. It looks for specific operations like exchanges, deposits, and withdrawals to adjust the assets’ exchange rates accordingly. This updating process is not continuous but occurs when the responsible hooks are called by the StableSwap/Omnipool.

The system, although thorough, does not account for price update triggers in the event of direct asset transfers to Stableswap, as these do not set off any hooks within the oracle. This lapse means that such direct transfers can alter asset prices within the liquidity pools without the oracle’s knowledge, potentially leading to misleading exchange rates.

Moreover, there’s a risk of manipulation by bad actors who might use direct transfers to StableSwap in an effort to sway the arbitrage process, especially during periods of network congestion. Such interference could unjustly prevent necessary liquidations within lending protocols.

## Impact

The issue allows a malicious user to change the price of the AMM without updating the oracle.

## Recommended Mitigation Steps

The issue can be mitigated by disabling transfers to the StableSwap pools, similar to how it is implemented for the Omnipool.

## Assessed type

Oracle enthusiastmartin (HydraDX) disputed and commented:

We believe this is not an issue, impact is not obvious. Oracle is not guaranteed to be always correct.

Lambda (judge) decreased severity to Low and commented:

The finding itself is valid, but only speculates about potential impacts (“potentially leading to misleading exchange rates.”). Because of that, it is a design recommendation. Downgrading to Low.

J4X (warden) commented:

@Lambda - Thank you very much for reviewing this audit. I am sorry that my issue was a bit inconclusive about the severity/results of this. The same impact (but for omnipool) has already been found in one of the earlier audits at Finding A1, this is why I kept my issue intentionally rather short, which was suboptimal in hindsight.

The oracle should always return the current price of the assets at stableswap/omnipool. As identified in the other audit this could be broken for the Omnipool by transferring assets directly to the Omnipool, making the price outdated. This was confirmed as a medium severity finding by the sponsor and fixed by implementing guards that blocked any transfers of tokens directly to the Omnipool. Unfortunately, the team has forgotten to implement the same safety measures when the StableSwap AMM was added to the protocol. As a result of this, the attack path is once again possible for all assets listed on StableSwap.

While I have not described an attack path that leads to an attacker profiting from this (which might be possible), the “attack” path of donating to make the oracle outdated, that I have described shows a way how the oracle becomes outdated which should never be the case. To keep it simple, this leads to one of the components of the protocol not functioning as intended (the oracle returning a wrong price) leading to the damage scenario of “Assets not at direct risk, but the function of the protocol impacted”.

Additionally, finding #73 leads to the exact same damage scenario, the oracle returning an incorrect price for an asset and has been confirmed as medium severity.

Lambda (judge) commented:

Keeping at QA because of missing impact/attack path. This might lead to problems in the protocol and be a valid medium or high then, but the issue does not demonstrate that.

Issue #73 mentions a potential impact (third-party protocols) that has external requirements, but is still valid nevertheless.

J4X (warden) commented:

@Lambda - I disagree with you differentiating between this issue and #73. They both result in the exact same state of the oracle returning an incorrect price for an asset.

Additionally, you mention that #73 offers an impact while this one does not. The impact that #73 describes is “So any protocol that uses this oracle as a price source would receive an incorrect price for the re-added asset for a short period of time” which can be shortened down to “external protocols relying on this oracle might break”. My issue describes “Such interference could unjustly prevent necessary liquidations within lending protocols” which can also be shortened to “external protocols relying on this oracle might break too”. It is just more focused on lending protocols, as this is the first thing that came to mind for me.

Lambda (judge) increased severity to Medium and commented:

That’s a good point, I previously missed the mention of integration with other protocols. Because a potential realistic impact with external requirements is mentioned and the finding itself is valid, I am upgrading it back to Medium.

# [M-02] Malicious liquidity provider can put pool into highly manipulatable state

- **Contest:** HydraDX
- **Slug:** 2024-02-hydradx
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-hydradx
- **Source snapshot:** competitions/2024-02-hydradx/final_report.html

Submitted by J4X, also found by carrotsmuggler and 3docSec The StableSwap AMM of the HydraDx protocol implements safeguards against low liquidity so that too high price fluctuations are prevented, and manipulating the price becomes harder. These safeguards are enforced based on the MinPoolLiquidity which is a constant that describes the minimum liquidity that should be in a pool. Additionally, a pool is allowed to have a liquidity of 0, which would occur in the case of the creation of the pool, or by users withdrawing all their liquidity. This could also be defined as an invariant.

totalPoolIssuance(poolId) >= MinPoolLiquidity || totalPoolIssuance(poolId) == 0.

When a user wants to withdraw his liquidity, he can use either the remove_liquidity_one_asset() function or the withdraw_asset_amount() function.

remove_liquidity_one_asset():

To ensure holding the invariant 2 checks are implemented in the remove_liquidity_one_asset() function. The first checks if the user either leaves more than MinPoolLiquidity shares in the pool or withdraws all his shares:

let current_share_balance = T::Currency::

free_balance (pool_id, &who); ensure!

( current_share_balance == share_amount || current_share_balance.

saturating_sub (share_amount) >= T::MinPoolLiquidity::

get (), Error::<T>::InsufficientShareBalance ); The second checks if the total liquidity in the pool would fall below the intended amount of shares:

let share_issuance = T::Currency::

total_issuance (pool_id); ensure!

( share_issuance == share_amount || share_issuance.

saturating_sub (share_amount) >= T::MinPoolLiquidity::

get (), Error::<T>::InsufficientLiquidityRemaining ); These two checks work perfectly at holding the invariant at all times.

withdraw_asset_amount():

Unfortunately, the second function for withdrawing liquidity withdraw_asset_amount() omits one of the checks. The function only checks if the user either withdraws all his shares or leaves more than the MinPoolLiquidity shares.

let current_share_balance = T::Currency::

free_balance (pool_id, &who); ensure!

( current_share_balance == shares || current_share_balance.

saturating_sub (shares) >= T::MinPoolLiquidity::

get (), Error::<T>::InsufficientShareBalance ); One might state now that this could never break the invariant, as if every user’s shares are either more than MinPoolLiquidity or zero, the total liquidity can never fall below MinPoolLiquidity without being 0. Unfortunately, this approach forgets that users can transfer their shares to other addresses. This allows a user to transfer an amount as low as 1 share to another address, and then withdraw all his shares. As the check would only ensure that he is withdrawing all his shares it would pass. If he was the only liquidity provider, there now would only be 1 share of liquidity left in the pool breaking the invariant of:

totalPoolIssuance(poolId) >= MinPoolLiquidity.

## Impact

The issue allows a user to break the invariant about the MinPoolLiquidity and either push the pool into a state where it can easily be manipulated, or prevent other users from withdrawing their shares.

## Recommended Mitigation Steps

The issue can be mitigated by also adding a check for the total pool liquidity to withdraw_asset_amount():

let share_issuance = T::

Currency::

total_issuance ( pool_id ); ensure !( share_issuance == share_amount || share_issuance.

saturating_sub ( share_amount ) >= T::

MinPoolLiquidity::

get (), Error::

< T >::InsufficientLiquidityRemaining ); enthusiastmartin (HydraDX) confirmed, but disagreed with severity and commented:

Although the check is missing,the issue is not high risk. Any limit that we have in our AMM are soft limits, meaning it is designed to protect mainly users, they don’t have to be always respected.

There is no evidence that the state of pool would be exploitable.

Lambda (judge) commented:

The warden identified how a security limit can be circumvented in some rare edge cases and how this could lead to a temporary DoS, Medium is appropriate here.

castle_chain (warden) commented:

@Lambda - I believe the severity of this issue should be reconsidered due to the impact it has:

This issue will not lead to a DoS or a lock of funds, as the liquidity provider can withdraw all their liquidity by calling the function withdraw_asset_amount() instead of remove_liquidity_one_asset, which encounters an issue with the limit of minimum liquidity. Thus, the user can simply withdraw all their liquidity in the same manner the attacker has, since the function withdraw_asset_amount() does not check for a minimum limit of shares remaining in the pool. Therefore, there is no risk of funds being locked or DoS for the liquidity providers.

The report mentioned that:

A malicious user withdraws all their liquidity using withdraw_asset_amount().

A normal user then tries to withdraw all of their liquidity using remove_liquidity_one_asset().

Here, the normal user can use the function withdraw_asset_amount() instead of remove_liquidity_one_asset(), and the entire liquidity removal will be completed.

Therefore, the only impact of this issue is allowing dust accounts to exist in the pool without any other impact, which should not be considered a medium severity issue.

J4X (warden) commented:

@castle chain - you are correct that the user could use `remove liquidity one asset()` to withdraw his shares, but this would require him to abuse the same issue as the malicious user.

1. DOS Regarding the DOS, this issue still leads to a DOS on one of the functions of the protocol, which suffices medium, as per the severity guidelines Med requires “Assets not at direct risk, but the function of the protocol or its availability could be impacted”. In this case, the function of remove_liquidity_one_asset() is clearly impacted and not usable. I mentioned in my issue that the user could be a contract, which is programmed to interact through the remove_liquidity_one_asset() function. As a lot of the interactions with an AMM are actually contracts and not EOA, this is a very usual case. The contract can’t be changed later on so it would never be able to withdraw its shares again, although being 100% correctly programmed.

2. Broken Invariant From the code, one can see that the intended invariant for the liquidity in a pool is sharesInPool == 0 || sharesInPool >= MinPoolLiquidty. This is done so that pools with very low liquidity can’t exist as they can easily be manipulated, which a lot of other AMMs do too. The sponsor described this as “to protect mainly the users” in the comment above. This issue shows in “1. Breaking the invariant and letting the pool Liquidity fall below MinPoolLiquidity” how this invariant can be broken so that the pool is easily manipulatable. How this should be graded can be seen in the Supreme Court decisions:

High - A core invariant of the protocol can be broken for an extended duration.

Medium - A non-core invariant of the protocol can be broken for an extended duration or at scale, or an otherwise high-severity issue is reduced due to hypotheticals or external factors affecting likelihood.

3. Conclusion So in total, the issue leads to 2 impacts, which both would be (at least) of medium severity:

DOS of the remove_liquidity_one_asset() function.

Breaking of the Pool liquidity invariant.

# [M-03] No slippage check in remove_liquidity function in omnipool can lead to slippage losses during liquidity withdrawal.

- **Contest:** HydraDX
- **Slug:** 2024-02-hydradx
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-hydradx
- **Source snapshot:** competitions/2024-02-hydradx/final_report.html

remove_liquidity function in omnipool can lead to slippage losses during liquidity withdrawal.

Submitted by carrotsmuggler, also found by carrotsmuggler ( 1, 2 ), erebus, QiuhaoLi, Aymen0909, zhaojie, oakcobalt ( 1, 2 ), emerald7017, DadeKuma, Franfran, J4X ( 1, 2, 3 ), 3docSec, and ZanyBonzy The liquidity removal function in the omnipool pallet lacks slippage control. There is no minimum_amount_out parameter to ensure that the user gets out at least a certain amount of tokens. This can lead to slippage losses for liquidity providers if malicious users frontrun the liquidity withdrawer.

During liquidity removal, since there are lots of different fees involved, the scenario gets complicated and a POC is used to study the effect further. A POC is presented in the next section, which has ALICE depositing LP of token_1000 to the pool, the actor LP3 carrying out a swap, and then ALICE removing liquidity immediately after. In case ALICE receives any LRNA tokens, she swaps them out to token_1000. We compare the amount of token_1000 ALICE would end up with in different scenarios.

In all scenarios, ALICE is assumed to remove liquidity at the same price she put in. However the bad actor LP3 frontruns her removal, and we want to study the effect of her losses. In scenario 1, there is no action by LP3, and ALICE deposits and withdraws, to get a baseline measurement.

Scenario 1 - Liq Add - Liq remove:

Here, there is no frontrunner in order to get a baseline measurement:

running 1 test lrna_init: 2000000000000000 token_init: 5000000000000000 lrna_add: 2000000000000000 token_add: 4000000000000000 lrna_remove: 2000000000000000 token_remove: 4990000000000000 We can see ALICE started with 5000*ONE token_1000 s, and ends up with 4990*ONE token_1000 s. This is due to withdrawal fees, and is the acceptable baseline. Any lower amounts due to frontrunning would be unacceptable. This is a 0.1% loss.

Scenario 2 - Liq Add - token->DAI swap - Liq remove:

Here, the frontrunner devalues token_1000 by selling a bunch of it for DAI. Since the price is now lower, some of Alice’s shares will be burnt:

running 1 test lrna_init: 2000000000000000 token_init: 5000000000000000 lrna_add: 2000000000000000 token_add: 4000000000000000 lrna_remove: 2000000000000000 token_remove: 4961892744479493 In this scenario, ALICE ends up with 4961.89*ONE token_1000 s. This is nearly a 1% loss. Since some of her share tokens are burnt, the other liquidity providers profit from this, since their liquidity positions are now worth more.

Scenario 3 - Liq Add - DAI->token swap - Liq remove:

Here, the frontrunner buys up token_1000 increasing its price. Alice gets minted LRNA tokens to compensate the increase in price, but she swaps them out to token_1000 immediately. We then check her token_1000 balance and compare it to the beginning:

running 1 test lrna_init: 2000000000000000 token_init: 5000000000000000 lrna_add: 2000000000000000 token_add: 4000000000000000 lrna_remove: 2187637667548876 token_remove: 4841500000000000 lrna_end: 2000000000000000 token_end: 4958579804661321 Here ALICE ends up with 4958.57*ONE token_1000 s. This is again a 1% loss. The frontrunner can even sandwich the LRNA->token_1000 swap and even profit in this scenario.

Thus in all frontrunning scenarios, ALICE realizes a slippage loss due to insufficient parameters. The losses will be capped to 2%, since the ensure_price check in the remove_liquidity function checks if the price of the asset has not changed by more than 1% from the oracle price. Thus, the maximum price deviation that can happen is 2% (if the spot price was changed from +1% to -1%). 2% slippage is already unacceptable for a number of cases, since the industry standard for swaps has been 0.5%, and even lower for liquidity removals.

## Recommended Mitigation Steps

Add a slippage limit for liquidity removal. The in-built limit of 2% is too large for most use cases.

## Assessed type

MEV enthusiastmartin (HydraDX) confirmed, but disagreed with severity via duplicate issue #158 Lambda (judge) commented via duplicate issue #158:

Valid medium because there is a hypothetical path to leak values and in-line with other audits where missing slippage checks were medium.

# [M-04] Complete liquidity removals fail from stableswap pools

- **Contest:** HydraDX
- **Slug:** 2024-02-hydradx
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-hydradx
- **Source snapshot:** competitions/2024-02-hydradx/final_report.html

Submitted by carrotsmuggler, also found by QiuhaoLi and J4X

- https://github.com/code-423n4/2024-02-hydradx/blob/603187123a20e0cb8a7ea85c6a6d718429caad8d/HydraDX-node/pallets/stableswap/src/lib.rs#L638
- https://github.com/code-423n4/2024-02-hydradx/blob/603187123a20e0cb8a7ea85c6a6d718429caad8d/HydraDX-node/pallets/stableswap/src/lib.rs#L551

## Impact

The contracts for stableswap has 2 functions dealing with removal of liquidity:

remove_liquidity_one_asset and withdraw_asset_amount. However, both these functions allow redeeming LP tokens and payout in only one token. Critically, this contract is missing Curve protocol’s remove_liquidity function, which allows redeeming LP tokens for all the different tokens in the pool.

The result of this decision is that when the complete liquidity of a pool is to be removed, the contract reverts with an arithmetic overflow. In curve protocol, when removing the complete liquidity, the composing tokens are removed from the pool. However, they also need to be converted to a single token, using a liquidity that won’t exist anymore. This leads to an issue somewhere in the mathematics of the curve liquidity calculation, and thus reverts.

## Recommended Mitigation Steps

Allow multi-token liquidity withdrawal, which would allow complete redeeming of all LP tokens.

## Assessed type

Under/Overflow enthusiastmartin (HydraDX) disputed and commented:

It is not issue and it is by design, as we don’t need the multi-token withdrawal functionality.

Lambda (judge) commented:

The warden demonstrated that the initial liquidity cannot be removed from the system because of an overflow. This can lead to (temporary) locked funds in edge cases, so Medium is appropriate here.

# [M-05] No safe_withdrawal option in withdraw_protocol_liquidity function in omnipool can be abused by frontrunners to cause losses to the admin when removing liquidity

- **Contest:** HydraDX
- **Slug:** 2024-02-hydradx
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-hydradx
- **Source snapshot:** competitions/2024-02-hydradx/final_report.html

safe_withdrawal option in withdraw_protocol_liquidity function in omnipool can be abused by frontrunners to cause losses to the admin when removing liquidity Submitted by carrotsmuggler, also found by QiuhaoLi The sacrifice_position function can be used by any liquidity provider to hand over their liquidity position to the protocol. The protocol can then choose to remove this liquidity via the withdraw_protocol_liquidity function. This is similar to the remove_liquidity function, but with one key difference. The remove_liquidity function has a safe_withdrawal option, where if trading is ongoing, the price difference is limited to 1% via the ensure_price function. This is not present in the withdraw_protocol_liquidity

function.

// remove_liquidity if !safe_withdrawal { T::PriceBarrier::

ensure_price ( &who, T::HubAssetId::

get (), asset_id, EmaPrice::

new (asset_state.hub_reserve, asset_state.reserve), ).

map_err (|_| Error::<T>::PriceDifferenceTooHigh)?; } Thus when the admin decides to call withdraw_protocol_liquidity to remove the liquidity, they can be frontrun to eat slippage loss. The admin has to pass in a price parameter, and if the frontrunner manipulates the spot price to be different from the price passed in, the admin will eat losses. A deeper dive and simulation of losses has been done in another issue titled No slippage check in remove_liquidity function in omnipool can lead to slippage losses during liquidity withdrawal where the losses are limited to 2% due to the ensure_price check. However, the losses here can be much higher due to the lack of this check altogether.

Since higher losses can be possible, this is a high severity issue.

## Recommended Mitigation Steps

Add a safe_withdrawal parameter, or add a minimum_out parameter to limit slippage losses.

## Assessed type

MEV enthusiastmartin (HydraDX) confirmed, but disagreed with severity and commented:

This action is usually performed when trading is paused, it is not permissionless call.

Definitely not high risk, not even medium.

Lambda (judge) decreased severity to Medium and commented:

Medium is more appropriate for missing slippage protection, even if the potential slippage can be larger here. According to the sponsor, the function will usually not be used when trading is enabled. However, this is not enforced, so the issue itself is still valid.

# [M-06] complete liquidity removal will result in permanent disable of the liquidity addition and prevent minting shares for the liquidity providers.

- **Contest:** HydraDX
- **Slug:** 2024-02-hydradx
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-hydradx
- **Source snapshot:** competitions/2024-02-hydradx/final_report.html

Submitted by castle_chain

- https://github.com/code-423n4/2024-02-hydradx/blob/603187123a20e0cb8a7ea85c6a6d718429caad8d/HydraDX-node/pallets/omnipool/src/lib.rs#L612-L621

## Impact

This vulnerability will lead to prevent any liquidity provider from adding liquidity and prevent them from minting new shares; so this is considered a huge loss of funds for the users and the protocol.

No New Liquidity - Users can no longer add liquidity to the pool, hindering its growth and potential.

Complete liquidity removal shuts down the pool, preventing any future activity.

Financial losses for the protocol - It loses the benefits of increased liquidity and potential fees from user activity.

## Recommended Mitigation Steps

Add a special behaviour to the function add_liquidity to handle the situation of no initial liquidity. The mitigation can be done by:

When the pool initially has no shares (total shares equal zero), newly added assets from a liquidity provider trigger the minting of shares in an amount equal to the added asset value As happening in the function add_token() here.

delta_shares: BalanceUpdate::

Increase (amount),

## Assessed type

Context Lambda (judge) decreased severity to Medium and commented:

The warden identified that the complete removal of liquidity can be problematic, although from a different angle and without mentioning the full impact. Giving partial credit for this.

castle_chain (warden) commented:

@Lambda - I am requesting that this issue be considered as a solo medium for the following reasons:

Firstly, this was marked as a duplicate of Issue #86. However, Issue #86 has nothing to do with this finding for these reasons:

Issue #86 refers to an issue in the stableswap pallet, not the omnipool.

This finding, on the other hand, refers to an issue in the omnipool pallet.

While removing all liquidity from a pool in the stableswap will always fail according to Issue #86, removing all liquidity from a pool in the omnipool pallet will succeed, but it will cause a permanent DoS (Denial-of-Service) attack on the pool by permanently disabling liquidity addition due to the division by zero which will throw overflow error mentioned in the submitted report and the PoC.

As demonstrated, these are two distinct issues. Issue #86 has an impact of (temporary) locked funds, according to the judge’s comment:

This can lead to (temporary) locked funds in edge cases, so Medium is appropriate here.

In contrast, my report highlights a permanent DoS impact to the function add_liquidity, sell,and buy.

So the two findings have two completely different locations, two different impacts, and two different affected functions:

Category Issue 86 Issue 75 (current) Location (pallet) stableswap omnipool

## Impact

temporary DoS permanent DoS Can complete liquidity removal be done ?

no ( this is the problem ) yes ( this is the cause of the problem ) Affected function (disabled function) remove liquidity one_asset (liquidity removal always failed) add_liquidity (liquidity addition always failed) Root cause overflow

## Mitigation

allow multi-asset withdrawal in the staple swap pallet handle the situation of no liquidity exists in the pool The judge mentioned that the report did not mention the full impact. While I described it in the impact section, let me clarify:

The full impact mentioned in the report: permanent disable of liquidity addition == permanent DoS of the function add_liquidity.

Since this is an edge case, which can simply happen, that causes a permanent DoS forever, it does not require an attacker or an attack to be triggered and cause damage to the protocol.

However, an attacker could trigger this edge case as the PoC test got performed, you can consider the LP2 as the attacker, who can withdraw all the liquidity that he possessed leaving the pool in DoS state.

The DoS can also happen without an attacker, simply by users removing all liquidity from the pool.

I mentioned complete liquidity removal to encompass all scenarios where this issue can cause a DoS and disable liquidity addition and trading. Therefore, I stated:

If all liquidity shares have been removed from any pool.

This includes:

A single malicious user possessing all the liquidity shares of the pool and removing them (attack or normal action), the user LP2

# [M-07] Re-adding assets to the omnipool can cause a problem with the oracle

- **Contest:** HydraDX
- **Slug:** 2024-02-hydradx
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-hydradx
- **Source snapshot:** competitions/2024-02-hydradx/final_report.html

Submitted by TheSchnilch

- https://github.com/code-423n4/2024-02-hydradx/blob/603187123a20e0cb8a7ea85c6a6d718429caad8d/HydraDX-node/pallets/omnipool/src/lib.rs#L1541-L1574
- https://github.com/code-423n4/2024-02-hydradx/blob/603187123a20e0cb8a7ea85c6a6d718429caad8d/HydraDX-node/pallets/omnipool/src/traits.rs#L164-L190
- https://github.com/code-423n4/2024-02-hydradx/blob/603187123a20e0cb8a7ea85c6a6d718429caad8d/HydraDX-node/pallets/ema-oracle/src/lib.rs#L558-L566

## Impact

If an asset is removed from the omnipool, it is ensured that all data records in the omnipool are deleted and also all positions from liquidity providers. However, the data records in the Oracle are not reset. This means that if the asset is to be added again after some time and it then has a different price, the price in the Oracle is falsified.

## Assessed type

Oracle enthusiastmartin (HydraDX) acknowledged, but disagreed with severity and commented:

It has no impact, and it is currently intended to keep it in oracle.

It might be an issue when we decided to add a token back; although, the price would correct itself anyway.

Lambda (judge) commented:

The warden identified an edge case (reading a token that was previously removed) where keeping the old values can lead to problems (short DoS or wrong prices used if deviation is not too large). Medium is appropriate here because a value leak with some external requirements is possible.

# [M-08] Storage can be bloated with low value liquidity positions

- **Contest:** HydraDX
- **Slug:** 2024-02-hydradx
- **Finding ID:** M-08
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-hydradx
- **Source snapshot:** competitions/2024-02-hydradx/final_report.html

Submitted by J4X When using the substrate framework, it is one of the main goals of developers to prevent storage bloat. If storage can easily be bloated by users, this can lead to high costs for the maintainers of the chain and a potential DOS. A more in detail explanation can be found here.

The Omnipool allows users to deposit liquidity to earn fees on swaps. Whenever a user deposits liquidity through add_liquidity(), he gets an NFT minted and the details of his deposit are stored in the Positions map:

let instance_id = Self::

create_and_mint_position_instance (&position_owner)?; <Positions<T>>::

insert (instance_id, lp_position); To ensure that this storage is only used for serious deposits, it is ensured to be above MinimumPoolLiquidity which is 1,000,000 tokens in the runtime configuration.

ensure!

( amount >= T::MinimumPoolLiquidity::

get () && amount > 0, Error::<T>::MissingBalance ); Additionally, whenever a deposit gets fully withdrawn, the storage entry is removed:

if updated_position.shares == Balance::

zero () { // All liquidity removed, remove position and burn NFT instance <Positions<T>>::

remove (position_id); T::NFTHandler::

burn (&T::NFTCollectionId::

get (), &position_id, Some(&who))?; Self::

deposit_event (Event::PositionDestroyed { position_id, owner: who.

clone (), }); } Unfortunately, this implementation does not take into account that a malicious user can add MinimumPoolLiquidity tokens, and then instantly withdraw all but 1. In that case, he has incurred almost no cost for bloating the storage (besides the 1 token and gas fees) and can keep on doing this countless times.

## Impact

The issue allows a malicious attacker to bloat the storage in a cheap way. If done often enough this allows him to DOS the functionality of the HydraDX protocol by bloating the storage significantly until it can’t be maintained anymore. If the attacker uses a very low-value token, he only incurs the gas fee for each new entry.

If we consider that the intended cost for adding a new position entry (to potentially DOS) as defined by the MinimumPoolLiquidity should be 1_000_000 tokens, this issue allows an attacker to get the same storage bloat for 1/1_000_000 or 0.0001% of the intended cost.

## Recommended Mitigation Steps

The issue can be mitigated by not letting the amount in an open position fall below MinimumPoolLiquidity. This can be enforced as follows in the remove_liquidity() function:

ensure !( updated_position.

amount >= T::

MinimumPoolLiquidity::

get () || updated_position.

amount == 0, Error::

< T >::InsufficientLiquidity );

## Assessed type

DoS enthusiastmartin (HydraDX) disputed and commented:

This is publicly known issue, raised by our team here.

Lambda (judge) commented:

While the sponsor was already aware of the issue, it was not ruled out as a known issue in the audit description and therefore, cannot be deemed out of scope.

QiuhaoLi (warden) commented:

@Lambda and @enthusiastmartin, thanks for the review. I have a question (not a dispute):

Haven’t we already limited the storage usage with gas fees (weight) in omnipool/src/weights.rs ?:

/// Storage: `Omnipool::Positions` (r:0 w:1) <=== /// Proof: `Omnipool::Positions` (`max_values`: None, `max_size`: Some(100), added: 2575, mode: `MaxEncodedLen`) fn add_liquidity () -> Weight { // Proof Size summary in bytes:

// Measured: `3919` // Estimated: `8739` // Minimum execution time: 220_969_000 picoseconds.

Weight::

from_parts ( 222_574_000, 8739 ).

saturating_add (T::DbWeight::

get ().

reads ( 20 )).

saturating_add (T::DbWeight::

get ().

writes ( 14 )) // <=== } As we can see, the user will be charged the fees of storage writes for minting new positions. So if an attack tries to bloat the storage, it will suffer from the corresponding fees.

J4X (warden) commented:

@QiuhaoLi - The costs for a protocol on Polkadot consist of 2 kinds of costs. The computation costs are forwarded to the user using the weights and the storage costs, which have to be handled by the protocol themselves.

The attacker is correctly charged for the storage instruction (computation cost) but is able to force the protocol to incur the constant cost of maintaining the positions (storage cost). This storage cost should only be incurred by the protocol for serious positions, which is why they have set a minimum of 1 million tokens. From positions of that size, they can recoup their storage cost through other fees. As one can see in the issue this can be circumvented and the protocol will not be able to recoup the storage costs through fees on dust positions leading to a potential DOS. This can happen if the storage is flooded with dust positions, leading to massive storage costs that the protocol can not recoup through fees due to the insufficient size of each position.

As the sponsor has acknowledged this is a valid issue that they are trying to fix internally, so I don’t see why this should be invalidated.

QiuhaoLi (warden) commented:

@J4X - thanks a lot for the explanation! I once thought about the cost of positions and decided it has been charged as fees just like Ethereum storage, which seems wrong. As I said this is not a dispute, just a question, nice finding!

# [M-09] Missing hook call will lead to incorrect oracle results

- **Contest:** HydraDX
- **Slug:** 2024-02-hydradx
- **Finding ID:** M-09
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-hydradx
- **Source snapshot:** competitions/2024-02-hydradx/final_report.html

Submitted by J4X, also found by tsvetanovv The HydraDx protocol includes an oracle. This oracle generates prices, based upon the information it receives from its sources (of which Omnipool is one). The Omnipool provides information to the oracle through the on_liquidity_changed and on_trade hooks. Whenever a trade happens or the liquidity in one of the pools changes the corresponding hooks need to be called with the updated values.

The Omnipool contract also includes the remove_token() function. This function can only be called by the authority and can be only called on an asset which is FROZEN and where all the liquidity shares are owned by the protocol.

ensure!

(asset_state.tradable == Tradability::FROZEN, Error::<T>::AssetNotFrozen); ensure!

( asset_state.shares == asset_state.protocol_shares, Error::<T>::SharesRemaining ); When the function gets called it transfers all remaining liquidity to the beneficiary and removes the token. This is a change in liquidity in the Omnipool. The functionality in terms of liquidity change is similar to the withdraw_protocol_liquidity() where the protocol also withdraws liquidity in the form of protocol_shares from the pool. When looking at the withdraw_protocol_liquidity() function, one can see that it calls the on_liquidity_changed hook at the end, so that the oracle receives the information about the liquidity change.

T::OmnipoolHooks::

on_liquidity_changed (origin, info)?; Unfortunately, the remove_token() function does not call this hook, keeping the oracle in an outdated state. As the token is removed later on, the oracle will calculate based on liquidity that does not exist anymore in the Omnipool.

## Impact

The issue results in the oracle receiving incorrect information and calculating new prices, based on an outdated state of the Omnipool.

## Recommended Mitigation Steps

The issue can be mitigated by forwarding the updated asset state to the oracle by calling the on_liquidity_changed hook.

## Assessed type

Oracle enthusiastmartin (HydraDX) disputed and commented via duplicate issue #141:

The calls is not needed in mentioned functions.

sacrifice_position does not change any liquidity and remove_token just removes token.

J4X (warden) commented:

@Lambda - This issue has been deemed as invalid due to a comment by the sponsor on Issue #141. Issue #141 describes that in the functions sacrifice_position() and remove_token(), a hook call to on_liquidity_changed is missing. The sponsor has disputed this with the claim that in none of those functions, the liquidity gets changed, which is true for sacrifice_position() but not for remove_token(). In sacrifice_position(), the sacrificed positions’ ownership is transferred to the protocol but the liquidity does not change.

The same is not the case for the remove_token() function. As one can see in the following code snippet, the function transfers out all liquidity that is owned by protocol shares to a beneficiary, changing the liquidity in the pool:

T::Currency::

transfer (asset_id, & Self::

protocol_account (), &beneficiary, asset_state.reserve)?; The function documentation also mentions the liquidity change.

So contrary to the comment of the sponsor, not only does the token get removed but also the liquidity changes, as the protocol-owned liquidity is sent to the beneficiary. This should result in a call to the hook so that the circuit breaker and the oracle get accordingly updated (and trigger at the right values). This could for example lead to an issue if we have a maximum liquidity change per block of 100 tokens chosen in our circuit breaker and a token gets removed with 90 tokens of protocol liquidity being withdrawn. A later call withdrawing 20 liquidity would incorrectly pass as the earlier withdrawn liquidity is not accounted for due to the missing hook call. This would undermine the security measure of the circuit breaker as the limits are not correctly enforced. Additionally, due to the missing liquidity update, the oracle will be outdated too.

I would like to mention that my issue is the only issue that fully and correctly documents the problem, as Issue #141 is reporting an invalid additional issue and also recommends an incorrect mitigation of increasing the liquidityInBlock in sacrifice_position().

Lambda (judge) commented:

Thanks for your comment. After looking at it again, remove_token indeed changes the liquidity like add_token does. While add_token calls on_liquidity_changed, remove_token does not, which can lead to inconsistencies.

# [M-10] A huge loss of funds for all the users who try to remove liquidity after swapping got disabled at manipulated price.

- **Contest:** HydraDX
- **Slug:** 2024-02-hydradx
- **Finding ID:** M-10
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-hydradx
- **Source snapshot:** competitions/2024-02-hydradx/final_report.html

Submitted by castle_chain, also found by QiuhaoLi, oakcobalt, and J4X

- https://github.com/code-423n4/2024-02-hydradx/blob/603187123a20e0cb8a7ea85c6a6d718429caad8d/HydraDX-node/pallets/omnipool/src/lib.rs#L1330-L1360
- https://github.com/code-423n4/2024-02-hydradx/blob/603187123a20e0cb8a7ea85c6a6d718429caad8d/HydraDX-node/pallets/omnipool/src/lib.rs#L759-L764

## Impact

This vulnerability will lead to huge loss of funds for liquidity providers that want to withdraw their liquidity if the safe withdrawal is enabled. The loss of funds can be 100% of the liquidity provider’s shares.

## Recommended Mitigation Steps

This vulnerability can be mitigated by only one step:

Check that the price is in the allowed Range before disabling the swapping and allow remove and add liquidity on any asset. This mitigation will make sure that the safe_withdrawal is set to true, except if the price in the Range so the price is actually stable and safe to withdraw liquidity on this price.

- https://github.com/code-423n4/2024-02-hydradx/blob/603187123a20e0cb8a7ea85c6a6d718429caad8d/HydraDX-node/pallets/omnipool/src/lib.rs#L1330-L1361
Consider modifying set_asset_tradable_state() function to ensure that if the state is set to preventing swapping, then ensure the price:

pub fn set_asset_tradable_state ( origin: OriginFor<T>, asset_id: T::AssetId, state: Tradability, ) -> DispatchResult { T::TechnicalOrigin::

ensure_origin (origin)?; if asset_id == T::HubAssetId::

get () { // At the moment, omnipool does not allow adding/removing liquidity of hub asset.

// Although BUY is not supported yet, we can allow the new state to be set to SELL/BUY.

ensure!

( !state.

contains (Tradability::ADD_LIQUIDITY) && !state.

contains (Tradability::REMOVE_LIQUIDITY), Error::<T>::InvalidHubAssetTradableState ); HubAssetTradability::<T>::

mutate (|value| -> DispatchResult { *value = state; Self::

deposit_event (Event::TradableStateUpdated { asset_id, state }); Ok(()) }) } else { Assets::<T>::

try_mutate (asset_id, |maybe_asset| -> DispatchResult { let asset_state = maybe_asset.

as_mut ().

ok_or (Error::<T>::AssetNotFound)?; + if (state == Tradability::ADD_LIQUIDITY | Tradability::REMOVE_LIQUIDITY || state == Tradability::REMOVE_LIQUIDITY){ + + T::PriceBarrier::

ensure_price ( + &who, + T::HubAssetId::

get (), + asset_id, + EmaPrice::

new (asset_state.hub_reserve, asset_state.reserve), + ) +.

map_err (|_| Error::<T>::PriceDifferenceTooHigh)?;} asset_state.tradable = state; Self::

deposit_event (Event::TradableStateUpdated { asset_id, state }); Ok(()) }) }

## Assessed type

Invalid Validation Lambda (judge) decreased severity to Low and commented:

Intended behaviour/design that this check is not performed in this state which can only be set by the AuthorityOrigin, downgrading to QA.

castle_chain (warden) commented:

@Lambda, This finding points to the vulnerable function set_asset_tradable_state, because it does not check that the price of the oracle is not too far from the spot price before activate the safe mode so it can be front-run by attackers.

The impact of the 100% of the liquidity withdrawn by the user will be taken as the withdrawal_fee, the impact of the Issue #93 is just a 1% due to the absence of the slippage parameter.

Lambda (judge) increased severity to Medium and commented:

The issue demonstrates that there can be edge cases where a very high fee is charged, therefore, upgrading it to a medium.

enthusiastmartin (HydraDX) acknowledged Note: For full discussion, see here.
