# Benchmark Ground Truth: MANTRA DEX

## Accepted H/M Findings

# Accepted H/M Findings: MANTRA DEX

# [H-01] Protocol allows creating broken tri-crypto CPMM pools

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-dex
- **Source snapshot:** competitions/2024-11-mantra-dex/final_report.html

Submitted by carrotsmuggler, also found by 0xAlix2, Abdessamed, DadeKuma, DadeKuma, DadeKuma, gegul, LonnyFlash, and Tigerfrake /contracts/pool-manager/src/manager/commands.rs#L75

## Finding description and impact

The protocol allows the creation of constant product pools and stableswap pools. Stable-swap pools, as established by curve, can have any number of tokens and so the protocol allows for the creation of pools with 2 or more tokens.

Constant product market-makers (CPMM), however, can have multiple tokens as well; however, the protocol here uses the uniswap formula, which only works for 2-token pools. For pools with more than 2 tokens, this model does not work anymore, and invariants need to be established with different formulas with products of all tokens quantities, like shown in the balancer protocol.

The issue is that the protocol here does not check if the constant product pool being created has more than 2 tokens. Surprisingly, it is perfectly possible to create a constant product pool with 3 tokens, add/remove liquidity and even do swaps in them, even though the protocol was never designed to handle this.

The POC below will show how we can set up a 3-token CPMM pool, add liquidity and even do swaps in it. The issue is that these pools are completely broken and should not be allowed.

The compute_swap function in the helpers.rs contract calculates the number of output tokens given the number of input tokens.

// ask_amount = (ask_pool * offer_amount / (offer_pool + offer_amount)) - swap_fee - protocol_fee - burn_fee let return_amount: Uint256 = Decimal256::

from_ratio (ask_pool.

mul (offer_amount), offer_pool + offer_amount).

to_uint_floor (); But these are only valid for 2-token uniswap-style pools. If there are more than 2 tokens involved, the invariant changes from being x * y = k to x * y * z = k, and the formula above does not work anymore. So for multi token pools, this formula should not be used, or x-y swaps can be arbitraged off of with y-z swaps and vice versa.

Furthermore, there is a check in the assert_slippage_tolerance function in the helpers contract:

if deposits.

len () != 2 || pools.

len () != 2 { return Err(ContractError::InvalidPoolAssetsLength { expected:

2, actual: deposits.

len (), }); } This explicitly shows that constant product pools are only allowed to have 2 tokens. However, if no slippage tolerance is specified, this check can be completely bypassed.

pub fn assert_slippage_tolerance ( slippage_tolerance: & Option <Decimal>, deposits: &[Coin], pools: &[Coin], pool_type: PoolType, amount: Uint128, pool_token_supply: Uint128, ) -> Result <(), ContractError> { if let Some(slippage_tolerance) = *slippage_tolerance { //@audit check for number of tokens } By never sending a slippage tolerance, users can create, add/remove liquidity and even do swaps in pools with more than 2 tokens following constant product algorithm. But these pools are completely broken and should not be allowed since the invariants are not functioning correctly

## Recommended mitigation steps

Add an explicit check during pool creation to make sure constant product pools cannot have more than 2 tokens.

jvr0x (MANTRA) confirmed

# [H-02] Logical error in validate_fees_are_paid can cause a DoS or allow users to bypass fees if denom_creation_fee includes multiple coins, including pool_creation_fee , and the user attempts to pay all fees using only pool_creation_fee

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-dex
- **Source snapshot:** competitions/2024-11-mantra-dex/final_report.html

validate_fees_are_paid can cause a DoS or allow users to bypass fees if denom_creation_fee includes multiple coins, including pool_creation_fee, and the user attempts to pay all fees using only pool_creation_fee Submitted by 0xRajkumar, also found by 0xAlix2, carrotsmuggler, Egis_Security, Egis_Security, jasonxiale, Lambda, oakcobalt, Tigerfrake, Tigerfrake, and Tigerfrake /contracts/pool-manager/src/helpers.rs#L561-L592

## Finding description and Impact

When a user creates a pool, they must pay both denom_creation_fee and pool_creation_fee.

The denom_creation_fee can be paid using multiple coins or a single coin and may also include the same coin as pool_creation_fee. If multiple denom_creation_fee coins options are available, and one of them matches the coin used for pool_creation_fee, it can lead to issues.

Problem Scenario The issue arises when the user attempts to pay both fees using the same coin.

Different Fee Amounts:

If the user pays both fees in the same coin, with different amounts for denom_creation_fee and pool_creation_fee, they might add both amounts and send the total. When validating the pool_creation_fee, the check paid_pool_fee_amount == pool_creation_fee.amount will fail, causing a DoS.

ensure!

( paid_pool_fee_amount == pool_creation_fee.amount, ContractError::InvalidPoolCreationFee { amount: paid_pool_fee_amount, expected: pool_creation_fee.amount, } ); Same Fee Amounts:

If both fees have the same amount and the user pays only once, they can bypass one of the fees entirely, resulting in a fee payment bypass.

ensure!

( paid_pool_fee_amount == pool_creation_fee.amount, //-> HERE It will pass ContractError::InvalidPoolCreationFee { amount: paid_pool_fee_amount, expected: pool_creation_fee.amount, } ); total_fees.

push (Coin { denom: pool_fee_denom.

clone (), amount: paid_pool_fee_amount, }); // Check if the user paid the token factory fee in any other of the allowed denoms let tf_fee_paid = denom_creation_fee.

iter ().

any (|fee| { let paid_fee_amount = info.funds.

iter ().

filter (|fund| fund.denom == fee.denom).

map (|fund| fund.amount).

try_fold (Uint128::

zero (), |acc, amount| acc.

checked_add (amount)).

unwrap_or (Uint128::

zero ()); total_fees.

push (Coin { denom: fee.denom.

clone (), amount: paid_fee_amount, }); paid_fee_amount == fee.amount //-> HERE It will pass }); As both are equal, that’s why both checks will pass. The impact is High as it can cause a DoS and allow the bypass of one of the fees.

## Recommended mitigation steps

We can verify whether the user is paying with one coin or multiple coins. If the user is paying with one coin, we can combine both amounts and perform the validation. Similarly, if the user is paying with multiple coins, we can apply the same approach. This will effectively mitigate the issue.

jvr0x (MANTRA) confirmed and commented:

It is valid. However, considering the chain only supports 1 token to pay for the token factory at the moment, I wouldn’t deem it as high, but low.

3docSec (judge) commented:

I see your point, and while I would agree if this were a bug bounty program (funds are not at risk in live contracts), I consider this a High, because what counts is the code in-scope and not the live config, unless the in-scope code is hardcoded to have only one token and can’t be changed by config.

# [H-03] Multi-token stableswap pools allow 0 liquidity for tokens, creating bricked pools

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-dex
- **Source snapshot:** competitions/2024-11-mantra-dex/final_report.html

0 liquidity for tokens, creating bricked pools Submitted by carrotsmuggler, also found by 0xAlix2, 0xRajkumar, Abdessamed, carrotsmuggler, and LonnyFlash /contracts/pool-manager/src/liquidity/commands.rs#L46-L74 /contracts/pool-manager/src/liquidity/commands.rs#L234-L239

## Finding description and impact

The stableswap pools allow anyone to create pools following the stableswap formula with any number of tokens. This can be higher than 2. The issue is that the initial provide_liquidity does not check if ALL tokens are provided.

The provide_liquidity function does a number of checks. For the ConstantProduct pools, the constant product part uses both deposits[0] and deposits[1] to calculate the initial number of shares, and uses a product of the two. So anyone being absent or 0 leads to reverts during the initial liquidity addition itself.

However, the stableswap pools do not check if the initial liquidity provided is non-zero for all the tokens. So if only 2 of the three tokens are provided, the transaction still goes through. The only check is that all the passed in tokens must be pool constituents.

ensure!

( deposits.

iter ().

all (|asset| pool_assets.

iter ().

any (|pool_asset| pool_asset.denom == asset.denom)), ContractError::AssetMismatch ); This leads to a broken pool, where further liquidity cannot be added anymore. This is because the pool is saved in a state where the pool has 0 liquidity for one of the tokens. Then in future liquidity additions, amount_times_coins value evaluates to 0 for those tokens, which eventually leads to a division by zero error in d_prod calculation.

let amount_times_coins:

Vec <Uint128> = deposits.

iter ().

map (|coin| coin.amount.

checked_mul (n_coins).

unwrap ()).

collect (); //...

for _ in 0..

256 { let mut d_prod = d; for amount in amount_times_coins.

clone ().

into_iter () { d_prod = d_prod.

checked_mul (d).

unwrap ().

checked_div (amount.

into ()) //@audit division by zero.

unwrap (); //...

Thus this leads to a broken pool and there is nothing in the contract preventing this.

## Recommended mitigation steps

Add a check to make sure if total supply=0, every token of the pool is provided as liquidity.

jvr0x (MANTRA) confirmed

# [H-04] Block gas limit can be hit due to loop depth

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Finding ID:** H-04
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-dex
- **Source snapshot:** competitions/2024-11-mantra-dex/final_report.html

Submitted by carrotsmuggler, also found by 0xAlix2, Evo, and Lambda /contracts/farm-manager/src/farm/commands.rs#L43-L94

## Finding description and impact

The claim function iterates over the user positions and calculates the rewards in nested loops. The issue is that every blockchain, to combat against gas attacks of infinite loops, has a block gas limit. If this limit is exceeded, that transaction cannot be included in the chain. The implementation of the claim function here is of the order of N^3 and is thus highly susceptible to an out of gas error.

The claim function iterates over all the user’s positions.

let lp_denoms = get_unique_lp_asset_denoms_from_positions (open_positions); for lp_denom in &lp_denoms { // calculate the rewards for the lp denom let rewards_response = calculate_rewards ( deps.

as_ref (), &env, lp_denom, &info.sender, current_epoch.id, true, )?; //...

} Lets say the user has P positions, all of different lp_deonm values. Thus this loop is of the order of P. The calculate_rewards function then loops over all the farms of each lp_denom.

let farms = get_farms_by_lp_denom ( deps.storage, lp_denom, None, Some(config.max_concurrent_farms), )?; //...

for farm in farms { // skip farms that have not started if farm.start_epoch > current_epoch_id { continue; } // compute where the user can start claiming rewards for the farm let start_from_epoch = compute_start_from_epoch_for_address ( deps.storage, &farm.lp_denom, last_claimed_epoch_for_user, receiver, )?; //...

} Say there are F farms, then this inner loop is of the order of F. Then for each farm, the reward is calculated by iterating over all the epochs from start_from_epoch up to the current_epoch.

for epoch_id in start_from_epoch..=until_epoch { if farm.start_epoch > epoch_id { continue; } //...

} The start_from_epoch can be the very first deposit of the user, far back in time, if this is the first time the user is claiming rewards. Thus, this loop can run very long if the position is years old. Say the epoch loop is of the order of E.

Since these 3 loops are nested, the claim function is of the order of P*F*E.

P and F are restricted by the config can can have maximum values of the order of 10. But E can be very large, and is actually the order of epoch number. So if epochs are only a few days long, the E can be of the order of 500 over a couple of years.

Thus the claim function can be of the order of 50_000. This is an issue since it requires a loop running 50_000 times along with reward calculations and even token transfers. This can be above the block gas limit and thus the transaction will fail.

There is no functionality to skip positions/farms/epochs. Thus users cannot claim rewards of only a few particular farms or epochs. This part of the code is also executed during the close_position function, which checks if rewards are 0. Thus, the close_position function can also fail due to the same issue, and users are thus forced to emergency withdraw and lose deposits as well as their rewards.

Thus users who join a bunch of different farms and keep their positions for a long time can hit the block gsa limit during the time of claiming rewards or closing positions.

The OOG issue due to large nesting depth is present in multiple instances in the code, this is only one example.

## Recommended mitigation steps

The order of the nested loops need to be decreased. This can be done in multiple ways.

Implement sushi-masterchef style reward accounting. This way the entire E number of epochs dont need to be looped over.

Implement a way to only process a given number of positions. This way P can also be restricted and users can claim in batches.

jvr0x (MANTRA) confirmed

# [H-05] Farms can be created to start in past epochs

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Finding ID:** H-05
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-dex
- **Source snapshot:** competitions/2024-11-mantra-dex/final_report.html

Submitted by Abdessamed, also found by 0xAlix2, carrotsmuggler, Lambda, and Tigerfrake /contracts/farm-manager/src/helpers.rs#L128-L175

## Finding description and impact

In the farming mechanism, users can claim rewards from active farms based on their locked LP token share. The rewards distribution must adhere to the following invariant:

At any given epoch, all users with locked LP tokens claim rewards from corresponding farms proportional to their share of the total LP tokens.

However, the current implementation allows the creation of farms with a start_epoch in the past. This breaks the invariant, as users who have already claimed rewards for past epochs will miss out on additional rewards assigned retroactively to those epochs. This issue arises because the validate_farm_epochs function does not enforce that the farm’s start epoch must be in the future relative to the current epoch:

/// Validates the farm epochs. Returns a tuple of (start_epoch, end_epoch) for the farm.

pub ( crate ) fn validate_farm_epochs ( params: &FarmParams, current_epoch:

u64, max_farm_epoch_buffer:

u64, ) -> Result <( u64, u64 ), ContractError> { let start_epoch = params.start_epoch.

unwrap_or (current_epoch + 1u64 ); ensure!

( start_epoch > 0u64, ContractError::InvalidEpoch { which:

"start".

to_string () } ); let preliminary_end_epoch = params.preliminary_end_epoch.

unwrap_or ( start_epoch.

checked_add (DEFAULT_FARM_DURATION).

ok_or (ContractError::InvalidEpoch { which:

"end".

to_string (), })?, ); // ensure that start date is before end date ensure!

( start_epoch < preliminary_end_epoch, ContractError::FarmStartTimeAfterEndTime ); // ensure the farm is set to end in a future epoch ensure!

( preliminary_end_epoch > current_epoch, ContractError::FarmEndsInPast ); // ensure that start date is set within buffer ensure!

( start_epoch <= current_epoch.

checked_add (max_farm_epoch_buffer).

ok_or ( ContractError::

OverflowError (OverflowError { operation: OverflowOperation::Add }) )?, ContractError::FarmStartTooFar ); Ok((start_epoch, preliminary_end_epoch)) } The function lacks a check to ensure that start_epoch is not earlier than current_epoch + 1, allowing farms to be created retroactively. This leads to unfair rewards distribution.

## Recommended mitigation steps

Ensure the start_epoch is always in the future relative to the current_epoch:

/// Validates the farm epochs. Returns a tuple of (start_epoch, end_epoch) for the farm.

pub(crate) fn validate_farm_epochs( params: &FarmParams, current_epoch: u64, max_farm_epoch_buffer: u64, ) -> Result<(u64, u64), ContractError> { let start_epoch = params.start_epoch.unwrap_or(current_epoch + 1u64); + assert!(start_epoch >= current_epoch + 1); // --SNIP } jvr0x (MANTRA) confirmed

# [H-06] Stable swap pools don’t properly handle assets with different decimals, forcing LPs to receive wrong shares

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Finding ID:** H-06
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-dex
- **Source snapshot:** competitions/2024-11-mantra-dex/final_report.html

Submitted by 0xAlix2, also found by 0x1982us, Abdessamed, carrotsmuggler, and oakcobalt Stable swap pools in Mantra implement Curve’s stable swap logic, this is mentioned in the docs. Curve normalizes the tokens in a stable swap pool, by having something called rate multipliers where they’re used to normalize the tokens’ decimals. This is critical as it is used in D computation here.

The reflection of this in Mantra is compute_d, where it does something similar, here:

// sum(x_i), a.k.a S let sum_x = deposits.

iter ().

fold (Uint128::

zero (), |acc, x| acc.

checked_add (x.amount).

unwrap ()); However, the issue is that amounts are not normalized from the caller, where this is called from compute_lp_mint_amount_for_stableswap_deposit:

#[allow(clippy::unwrap_used, clippy::too_many_arguments)] pub fn compute_lp_mint_amount_for_stableswap_deposit ( amp_factor: & u64, old_pool_assets: &[Coin], new_pool_assets: &[Coin], pool_lp_token_total_supply: Uint128, ) -> Result < Option <Uint128>, ContractError> { // Initial invariant @> let d_0 = compute_d (amp_factor, old_pool_assets).

ok_or (ContractError::StableInvariantError)?; // Invariant after change, i.e. after deposit // notice that new_pool_assets already added the new deposits to the pool @> let d_1 = compute_d (amp_factor, new_pool_assets).

ok_or (ContractError::StableInvariantError)?; // If the invariant didn't change, return None if d_1 <= d_0 { Ok(None) } else { let amount = Uint512::

from (pool_lp_token_total_supply).

checked_mul (d_1.

checked_sub (d_0)?)?.

checked_div (d_0)?; Ok(Some(Uint128::

try_from (amount)?)) } This messes up the whole shares calculation logic, as D would be way greater for LPs depositing tokens of higher decimals than other tokens in the same stable swap pool.

NB: This is handled for swaps, here.

## Recommended mitigation steps

Whenever computing D, make sure all the deposits/amounts are in the “non-decimal” value, i.e., without decimals. For example, 100e6 should just be sent as 100, just like how it’s done in compute_swap. This should be added in compute_d.

jvr0x (MANTRA) confirmed

# [H-07] User cannot claim rewards or close_position , due to vulnerable division by zero handling

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Finding ID:** H-07
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-dex
- **Source snapshot:** competitions/2024-11-mantra-dex/final_report.html

close_position, due to vulnerable division by zero handling Submitted by oakcobalt, also found by 0xAlix2, Daniel526, Lambda, and Tigerfrake A user cannot claim rewards or close_position, due to vulnerable division by zero handling in the claim -> calculate_rewards flow.

In calculate_rewards, a user’s reward per farm per epoch is based on the user_share ( user_weight / contract_weights ); contract_weights can be zero.

The main vulnerability is division by zero handling is not done at the site of division; i.e., no check on contract_weights is non-zero before using it as a denominator in checked_mul_floor. (Flows:

claim -> calculate_rewards ).

//contracts/farm-manager/src/farm/commands.rs pub ( crate ) fn calculate_rewards (...

) -> Result <RewardsResponse, ContractError> {...

for epoch_id in start_from_epoch..=until_epoch {...

let user_weight = user_weights[&epoch_id]; let total_lp_weight = contract_weights.

get (&epoch_id).

unwrap_or (&Uint128::

zero ()).

to_owned (); //@audit contract_weights or total_lp_weight can be zero, when used as a fraction with checked_mul_floor, this causes division by zero error.

|> let user_share = (user_weight, total_lp_weight); let reward = farm_emissions.

get (&epoch_id).

unwrap_or (&Uint128::

zero ()).

to_owned () |>.

checked_mul_floor (user_share)?;...

/contracts/farm-manager/src/farm/commands.rs#L205 Current contract attempts to handle this at the source; clear the users LAST_CLAIMED_EPOCH when a user closes a position. This is also vulnerable because when the user has active positions in other lp-denoms, LAST_CLAIMED_EPOCH cannot be cleared for the user. Back in calcualte_rewards, this means the epoch iteration will still start at ( LAST_CLAIMED_EPOCH + 1 ) which includes the epoch where contract_weights is zero. (Flows:

close_position -> reconcile_user_state ).

//contracts/farm-manager/src/position/helpers.rs pub fn reconcile_user_state ( deps: DepsMut, receiver: &Addr, position: &Position, ) -> Result <(), ContractError> { let receiver_open_positions = get_positions_by_receiver ( deps.storage, receiver.

as_ref (), Some( true ), None, Some(MAX_ITEMS_LIMIT), )?; // if the user has no more open positions, clear the last claimed epoch //@audit-info note: LAST_CLAIMED_EPOCH will not be cleared for the user when the user has open positions in other lp_denom if receiver_open_positions.

is_empty () { |> LAST_CLAIMED_EPOCH.

remove (deps.storage, receiver); }...

/contracts/farm-manager/src/position/helpers.rs#L215

## Impact

Users’ rewards will be locked and unclaimable. Since pending rewards have to be claimed before close_position, users cannot close any positions without penalty.

## Recommended mitigation steps

Consider handling division by zero in calculate_rewards directly by skip the epoch iteration when contract_weights is 0.

jvr0x (MANTRA) confirmed

# [H-08] Stableswap pool can be skewed free of fees

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Finding ID:** H-08
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-dex
- **Source snapshot:** competitions/2024-11-mantra-dex/final_report.html

Submitted by carrotsmuggler, also found by Abdessamed /contracts/pool-manager/src/liquidity/commands.rs#L257-L265

## Finding description and impact

Stableswap pools are designed to work around a set pricepoint. If the price of the pool deviates away from that point, the pool can incur large slippage.

Normally in constant product AMMs (CPMMs), slippage is observed at every point in the curve. However, for a user to change the price drastically, they need to do a large swap. This costs them swap fees. In CPMMs, adding liquidity does not change the price since they always have to be added at specific ratios.

For stableswaps, this is not true. In stableswap pools, liquidity can be added in at any ratio. This means that a user can add liquidity at a ratio far from the current price, which will change the ratio of funds in the pool, leading to large slippage for all users. Stableswap pools protect against this by using fees.

If we look at the curve protocol, we see that if liquidity is added at a ratio far from the current price, the difference between the liquidity addition price and ideal price is computed. The contract can be found here.

ideal_balance = D1 * old_balances[i] / D0 difference = 0 new_balance = new_balances[i] if ideal_balance > new_balance:

difference = unsafe_sub(ideal_balance, new_balance) else:

difference = unsafe_sub(new_balance, ideal_balance) This basically is a measure of how much the pool is being skewed due to this liquidity addition. The user is then made to pay swap fees for this skew they introduced.

_dynamic_fee_i = self._dynamic_fee(xs, ys, base_fee) fees.append(unsafe_div(_dynamic_fee_i * difference, FEE_DENOMINATOR)) self.admin_balances[i] += unsafe_div(fees[i] * admin_fee, FEE_DENOMINATOR) new_balances[i] -= fees[i] So, If a user adds liquidity at the current pool price, difference will be 0 and they wont be charged fees. But if they add liquidity at a skewed price, they will be charged a fee which is equal to the swap fee on the skew they introduced.

This basically makes them equivalent to CPMMs, where to change the price you need to pay swap fees. In stableswap pools like on curve, you pay swap fees if you change the price during liquidity addition.

The issue is that in the stableswap implementation in the codebase, this fee isn’t charged. So users skewing the stableswap pool can basically do it for free, pay no swap fees and only lose out on some slippage.

let d_0 = compute_d (amp_factor, old_pool_assets).

ok_or (ContractError::StableInvariantError)?; let d_1 = compute_d (amp_factor, new_pool_assets).

ok_or (ContractError::StableInvariantError)?; if d_1 <= d_0 { Ok(None) } else { let amount = Uint512::

from (pool_lp_token_total_supply).

checked_mul (d_1.

checked_sub (d_0)?)?.

checked_div (d_0)?; Ok(Some(Uint128::

try_from (amount)?)) } Here new_pool_assets is just old_pool_assets + deposits. So a user can add liquidity at any ratio, and not the penalty for it. This can be used by any user to manipulate the pool price or highly skew the pool composition.

Attached is a POC showing a user doing the same. This shows that the pools can be manipulated very easily at very low costs, and users are at risk of losing funds due to high slippage.

## Recommended mitigation steps

Similar to curve, add swap fees based on the skewness introduced in the stableswap pools during liquidity addition.

jvr0x (MANTRA) confirmed

# [H-09] Attackers can force the rewards to be stuck in the contract with malicious x/tokenfactory denoms

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Finding ID:** H-09
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-dex
- **Source snapshot:** competitions/2024-11-mantra-dex/final_report.html

x/tokenfactory denoms Submitted by peachtea, also found by Audinarey, carrotsmuggler, Egis_Security, and p0wd3r Attackers can fund rewards of LP tokens with tokens created from the x/tokenfactory module and abuse the MsgForceTransfer message to prevent the contract from successfully distributing rewards. This would also prevent the contract owner from closing the malicious farm. As a result, rewards that are accrued to the users will be stuck in the contract, causing a loss of rewards.

## Recommended mitigation steps

To mitigate this attack, consider modifying the close_farms function so the messages are dispatched as SubMsg::reply_on_error when refunding the rewards to the farm owner. Within the reply handler, simply return an Ok(Response::default()) if an error occurred during BankMsg::Send. This will prevent the attack because the contract owner will still have the power to close malicious farms even though the attacker reduced the contract’s balance.

- https://docs.rs/cosmwasm-std/latest/cosmwasm_std/struct.SubMsg.html#method.reply_on_error
jvr0x (MANTRA) confirmed 3docSec (judge) commented:

Marking this one as primary, because it highlights the two impacts in this group:

Malicious pools brick claiming of legitimate pools’ rewards.

Malicious pools can’t be closed.

It is, however, recommended to take into consideration also the S-377 mitigation of letting users opt-out from malicious pools without requiring admin intervention

# [H-10] Incorrect slippage_tolerance handling in stableswap provide_liquidty function

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Finding ID:** H-10
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-dex
- **Source snapshot:** competitions/2024-11-mantra-dex/final_report.html

slippage_tolerance handling in stableswap provide_liquidty function Submitted by carrotsmuggler, also found by oakcobalt /contracts/pool-manager/src/helpers.rs#L437-L438

## Finding description and impact

The provide_liquidity function is used to add liquidity to the dex pools. This function implements a slippage tolerance check via the assert_slippage_tolerance function.

helpers::

assert_slippage_tolerance ( &slippage_tolerance, &deposits, &pool_assets, pool.pool_type.

clone (), share, total_share, )?; This function implements slippage tolerance in two sub-functions, one for stableswap and one for constant product. This function basically compares the ratio of liquidity of the deposit to the ratio of pool liquidity.

For the constant product pool, the slippage tolerance checks both the 1/0 ratio and 0/1 ratios, where 0 and 1 represent the two tokens of the pool.

if Decimal256::

from_ratio (deposits[ 0 ], deposits[ 1 ]) * one_minus_slippage_tolerance > Decimal256::

from_ratio (pools[ 0 ], pools[ 1 ]) || Decimal256::

from_ratio (deposits[ 1 ], deposits[ 0 ]) * one_minus_slippage_tolerance > Decimal256::

from_ratio (pools[ 1 ], pools[ 0 ]) { return Err(ContractError::MaxSlippageAssertion); } But for stableswap, it only does a one-sided check.

if pool_ratio * one_minus_slippage_tolerance > deposit_ratio { return Err(ContractError::MaxSlippageAssertion); } The situation is best described for the scenario where slippage_tolerance is set to 0. This means the pool should ONLY accept liquidity in the ratio of the pool liquidity. This is enforced for constant product pools correctly. However, for stableswap pools, this is incorrect.

If slippage_tolerance is set to 0, then one_minus_slippage_tolerance is 1. Thus, the inequality check above makes sure that the pool_ratio is always less than or equal to the deposit_ratio for the transaction to go through. However, the deposit_ratio can be be either higher or lower than than the pool_ratio, depending on the components of the liquidity addition. The inequality above only checks for one case (less than equals) and misses the other check (greater than equals).

This means even with slippage_tolerance set to 0, the stableswap pool will accept liquidity that is not in the ratio of the pool liquidity.

Furthermore, for the case where the deposit_ratio is higher than the pool_ratio, there is no slippage restriction on the pool at all.

The entire reason slippage_tolerance exists, is so that the user can specify the exact amount of lp tokens they expect out of the pool. However, the protocol does not implement a minimum_amount_out like on curve, and instead uses this slippage_tolerance value. This means the slippage_tolerance value is crucial to ensure that the depositor is not leaking any value. However, below shown is a situation where if the depositor adds liquidity in certain compositions, they can leak any amount of value.

A POC is run to generate the numbers given here.

Lets say a pool is created and liquidity is provided with 1e6 whale and 2e6 luna tokens. It is quite common to have stableswap pools similarly imbalanced, so this is a usual scenario. Now a user deposits 1e4 whale and 1e5 luna tokens in this pool.

At the end, the pool composition becomes 1.01e6 whale and 2.1e6 luna tokens. The initial liquidity addition created 2997146 lp tokens and the second liquidity addition creates 109702 lp tokens, for a total of 3106848 lp tokens.

These numbers come from running the POC below, which has the output:

running 1 test ===Liq addition=== ==Balance deltas== uwhale delta: -1000000 uluna delta: -2000000 lp delta: 2997146 ==Balance deltas== ===Liq addition 2=== ==Balance deltas== uwhale delta: -10000 uluna delta: -100000 lp delta: 109702 ==Balance deltas== So during the slippage check on the second deposit, pool_sum = 1e6+2e6 + 1e5+1e4 = 3.11e6, and the deposit_sum = 1e5+1e4 = 1.1e5.

pool_ratio = 3.11e6/3106848 = 1.001014533 deposit_ratio = 1.1e5/109702 =1.00271645 Now, even if slippage tolerance is set to 0, since `pool ratio < deposit_ratio, the transaction goes through. However, the issue is that in the second liquidity addition, the user could have received less than 109702` lp tokens and the transaction would have still gone through.

Say the user receives only 108000 tokens. Then, total pool lp_tokens = 2997146+108000 = 3105146 pool_ratio = 3.11e6/3105146 = 1.001563212 deposit_ratio = 1.1e5/108000 = 1.018518519 This transaction will also pass, since deposit_ratio > pool_ratio. However, we can clearly see that the liquidity depositor has lost 1.55% of their deposit. So even with slippage_tolerance set to 0, the stableswap pool can accept liquidity that is not in the ratio of the pool liquidity, and depositors can eat large amounts of slippage.

## Recommended mitigation steps

For stableswap, the slippage_tolerance should be checked against the difference in the price ratios, so abs( pool_ratio - deposit_ratio ). This way both sides of the inequality are checked.

jvr0x (MANTRA) confirmed

# [H-11] Stableswap does disjoint swaps, breaking the underlying invariant

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Finding ID:** H-11
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-dex
- **Source snapshot:** competitions/2024-11-mantra-dex/final_report.html

Submitted by carrotsmuggler, also found by 0x1982us, Abdessamed, Abdessamed, and LonnyFlash /contracts/pool-manager/src/helpers.rs#L117-L124 /contracts/pool-manager/src/helpers.rs#L39-L88

## Finding description and impact

In stableswap pools, the invariant that is preserved is a combination of a CPMM and a constant price model. For pools with more than 2 tokens, every token balance is used to compute the invariant.

This is shown in the curve protocol, where the invariant is calculated correctly.

The invariant is made up of two parts, the stable part and the constant product part:

Note: please see scenario in warden’s original submission.

This lets every token in the pool stay in parity with the others, and reduces the slippage. The issue is that in the current implementation, instead of summing or taking the product of all the tokens of the pools, the protocol only takes the sum/product of the ask and offer tokens.

For example, in the compute_swap function, let new_pool = calculate_stableswap_y ( n_coins, offer_pool, ask_pool, offer_amount, amp, ask_precision, StableSwapDirection::Simulate, )?; Only the ask and offer amounts token amounts are sent in. In the internal calculate_stableswap_y function, the invariant is calculated using these two only.

let pool_sum = match direction { StableSwapDirection::Simulate => offer_pool.

checked_add (offer_amount)?, StableSwapDirection::ReverseSimulate => ask_pool.

checked_sub (offer_amount)?, Here’s the curve stableswap code for comparison, for _i in range(N_COINS):

if _i == i:

_x = x elif _i != j:

_x = xp_[_i] else:

continue S_ += _x The sum_invariant D is calculated only with the two tokens in question, ignoring the third or fourth tokens in the pool; while the actual invariant requires a sum of ALL the tokens in the pool. Similarly, calculating in calculate_stableswap_d also calculates the sum using only 2 token balances.

let sum_pools = offer_pool.

checked_add (ask_pool)?; The n_coins used in the calculations, however, is correct and equal to the number of tokens in the pool. This is enforced since the reserves length is used, which is set up correctly during pool creation.

n_coins: Uint256::

from (pool_info.assets.

len () as u128 ), Thus, the S and D calculated are incorrect. This also influences the outcome of the newton-raphson iterations, since both these quantities are used there.

The result of this is that if a pool has three tokens A, B, C then A-B swaps ignore the liquidity of C. This is because the S and D calculations will never touch the liquidity of C, since they only deal with the ask and offer tokens.

So for tricrypto pools, the invariant preserved in A-B swaps is different from the invariant preserved in B-C swaps.

The result are swaps with worse slippage profiles. In normal stableswap pools, the pool tries to maintain all the tokens in parity with each other, giving higher slippage if the pool as a whole is imbalanced. So A-B swaps will have lots of slippage if token C is available in a drastically different amount. However, in this case, the pool only cares about the ask and offer tokens, so the slippage will be lower than expected, leading to arbitrage opportunities. This allows the pools to be more manipulatable.

## Recommended mitigation steps

Implement the correct invariant for stableswap, by also including the third/other token amounts in the sum and D calculations.

jvr0x (MANTRA) confirmed

# [H-12] Pool creators can manipulate the slippage calculation for liquidity providers

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Finding ID:** H-12
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-dex
- **Source snapshot:** competitions/2024-11-mantra-dex/final_report.html

Submitted by DadeKuma, also found by 0x1982us Pool creation is permissionless, and users can create a pool by specifying asset denoms. The issue is that they can put a different order of the same token denoms, which should result in the same pool, but in fact, it does not.

This ultimately cause the slippage mechanism to use the inverse ratio instead of the correct one, as in other parts of the codebase these values are always ordered, which will cause a loss of funds for the users that provide liquidity as they use the inverted slippage.

## Recommended mitigation steps

In pool_manager::create_pool, consider sorting the asset denoms, similarly to other parts of the code, by introducing a new struct to encapsulate both asset_denoms and asset_decimals (as they are tied together) and reorder it before creating the pool.

jvr0x (MANTRA) confirmed Medium Risk Findings (19)

# [M-01] In edge cases, create_pool can either be reverted or allow user underpay fees

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-dex
- **Source snapshot:** competitions/2024-11-mantra-dex/final_report.html

create_pool can either be reverted or allow user underpay fees Submitted by oakcobalt, also found by 0x1982us, Abdessamed, and Lambda /contracts/pool-manager/src/helpers.rs#L563-L564

## Finding description and impact

create_pool is permissionless. User will need to pay both pool creation fee and token factory fee (fees charged for creating lp token) when creating a pool.

The vulnerabilities are:

In validate_fees_are_paid, a strict equality check is used between total paid fees in pool_creation token and required pool creation fee.

paid_pool_fee_amount == pool_creation_fee.amount.

denom_creation_fee.iter() uses.any instead of.all, which allows user to only pay one coin from denom_creation.

//contracts/pool-manager/src/helpers.rs pub fn validate_fees_are_paid ( pool_creation_fee: &Coin, denom_creation_fee:

Vec <Coin>, info: &MessageInfo, ) -> Result < Vec <Coin>, ContractError> {...

// Check if the pool fee denom is found in the vector of the token factory possible fee denoms if let Some(tf_fee) = denom_creation_fee.

iter ().

find (|fee| &fee.denom == pool_fee_denom) { // If the token factory fee has only one option, check if the user paid the sum of the fees if denom_creation_fee.

len () == 1usize {...

} else { // If the token factory fee has multiple options besides pool_fee_denom, check if the user paid the pool creation fee let paid_pool_fee_amount = get_paid_pool_fee_amount (info, pool_fee_denom)?; //@audit (1) strict equality check. When user is also required to pay denom_creation_fee in pool creation fee token, check will revert create_pool ensure!

( |> paid_pool_fee_amount == pool_creation_fee.amount, ContractError::InvalidPoolCreationFee { amount: paid_pool_fee_amount, expected: pool_creation_fee.amount, } );...

// Check if the user paid the token factory fee in any other of the allowed denoms //@audit (2) iter().any() only requires one of denom_creation_fee token to be paid.

|> let tf_fee_paid = denom_creation_fee.

iter ().

any (|fee| { let paid_fee_amount = info.funds.

iter ().

filter (|fund| fund.denom == fee.denom).

map (|fund| fund.amount).

try_fold (Uint128::

zero (), |acc, amount| acc.

checked_add (amount)).

unwrap_or (Uint128::

zero ()); total_fees.

push (Coin { denom: fee.denom.

clone (), amount: paid_fee_amount, }); paid_fee_amount == fee.amount });...

/contracts/pool-manager/src/helpers.rs#L577 Based on cosmwasm tokenfactory, denom creation fee ( std.coins ) can contain multiple coins and every coin needs to be paid.

//x/tokenfactory/simulation/operations.go func SimulateMsgCreateDenom (tfKeeper TokenfactoryKeeper, ak types.AccountKeeper, bk BankKeeper) simtypes.Operation {...

// Check if sims account enough create fee createFee:= tfKeeper.

GetParams (ctx).DenomCreationFee balances:= bk.

GetAllBalances (ctx, simAccount.Address) |> _, hasNeg:= balances.

SafeSub (createFee) //@audit-info all denom creation fee tokens have to be paid if hasNeg { return simtypes.

NoOpMsg (types.ModuleName, types.MsgCreateDenom{}.

Type (), "Creator not enough creation fee" ), nil, nil }...

- https://github.com/CosmWasm/token-factory/blob/47dc2d5ae36980bcc03cf746580f7cb3deabc39e/x/tokenfactory/simulation/operations.go#L359-L361
Flows:

contracts/pool-manager/src/manager/commands::create_pool -> validate_fees_are_paid()

## Impact

User can either underpay fees, or create_pool tx will revert.

## Recommended mitigation steps

Change.any() ->.all().

Because this branch denom_creation_fee contains the pool_creation token, needs to add a control flow to handle the iteration of pool_creation token to check paid_fee_amount == fee.amount + pool_creation_fee.amount.

3docSec (judge) commented:

Looks to be intended behavior, as per comment L576.

jvr0x (MANTRA) disputed and commented:

pool_creation_fee is the fee for creating the pool while the vec.

denom_creation_fee are the tokens that the user can pay for the token factory in. Only 1 is enough, no need to pay in all the denoms listed there.

3docSec (judge) commented:

Behavior is inconsistent with the MantraChain tokenfactory that collects all fees; okay for valid Medium.

# [M-02] Penalty fees can be shared among future farms or expired farms, risks of exploits

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-dex
- **Source snapshot:** competitions/2024-11-mantra-dex/final_report.html

Submitted by oakcobalt, also found by 0xAlix2, 0xlookman, Bauchibred, Egis_Security, gegul, jasonxiale, Lambda, and Tigerfrake The penalty fee is a percentage of the value of existing positions that are emergency withdrawn ( withdraw_position ). It is shared equally among farm owners whose farms have the same lp_denom as the position.

The vulnerability is that the penalty fee is divided amongst all farms regardless of whether the farms are current, in the future, or already expired, which allows malicious farm owners to exploit.

## Impact

Active farm owners get less penalty fee shares due to fee shared among expired farms and future farms. A malicious farm creator could also take penalty fee shares without having to contribute to rewarding.

## Recommended mitigation steps

In withdraw_position, before dividing the owner_penalty_fee_comission, filter out farms that starts in a future epoch or have expired.

jvr0x (MANTRA) confirmed and commented:

The farm creation fee is going to be set high enough to prevent unserious players to create farms. Additionally, the contract owner can at any point close farms deemed as spam, malicious or dishonest.

However, the recommendation is valid, will likely adopt it.

3docSec (judge) commented:

Shares in penalty fees can, in my understanding, still compensate for farm creation fees, even though with low likelihood; so Medium seems appropriate.

# [M-03] User is unable to claim their reward for the expanded epochs if farm is expanded

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-dex
- **Source snapshot:** competitions/2024-11-mantra-dex/final_report.html

Submitted by 0xRajkumar, also found by 0xlookman and carrotsmuggler /contracts/farm-manager/src/manager/commands.rs#L240-L243

## Finding description and Impact

We have a claim function in the Farm Manager, which is used to claim rewards if the user has any open positions. This function updates the LAST_CLAIMED_EPOCH to the user’s last claimed epoch.

Additionally, we have the expand_farm function, which is used to expand farms. Technically, the farm creator can expand the farm even after the farm’s last reward epoch has been completed.

Let’s explore why below:

pub ( crate ) fn is_farm_expired ( farm: &Farm, deps: Deps, env: &Env, config: &Config, ) -> Result < bool, ContractError> { let epoch_response: EpochResponse = deps.querier // query preliminary_end_epoch + 1 because the farm is preliminary ending at that epoch, including it..

query_wasm_smart ( config.epoch_manager_addr.

to_string (), &QueryMsg::Epoch { id: farm.preliminary_end_epoch + 1u64, }, )?; let farm_ending_at = epoch_response.epoch.start_time; Ok( farm.farm_asset.amount.

saturating_sub (farm.claimed_amount) == Uint128::

zero () || farm_ending_at.

plus_seconds (config.farm_expiration_time) < env.block.time, ) } Let’s say our starting epoch was 1 and the preliminary_end_epoch was 2, meaning the farm was intended only for epoch 1. However, due to the condition farm_ending_at.plus_seconds(config.farm_expiration_time) < env.block.time, the farm owner can expand the farm during epoch 2 + farm_expiration_time as well.

Now let’s say even if farm expanding the farm for 2 epoch, then users who have an open position for that epoch should be able to claim reward, but this is not the case. Let’s see how.

Let’s say a user has an opened position claims rewards during epoch 2, then he will be able to claim reward from farm for epoch 1 only because preliminary_end_epoch is 2 as you can see this in this function.

fn compute_farm_emissions ( farm: &Farm, start_from_epoch: &EpochId, current_epoch_id: &EpochId, ) -> Result <(HashMap<EpochId, Uint128>, EpochId), ContractError> { let mut farm_emissions = HashMap::

new (); let until_epoch = if farm.preliminary_end_epoch <= *current_epoch_id { // the preliminary_end_epoch is not inclusive, so we subtract 1 farm.preliminary_end_epoch - 1u64 } else { *current_epoch_id }; for epoch in *start_from_epoch..=until_epoch { farm_emissions.

insert (epoch, farm.emission_rate); } Ok((farm_emissions, until_epoch)) } Whenever a user claims a reward, we maintain the LAST_CLAIMED_EPOCH. If the user tries to claim again, they will only be able to claim rewards starting from LAST_CLAIMED_EPOCH + 1.

There is a possibility that if a user claims rewards during the preliminary_end_epoch, and immediately after, the farm owner expands the farm, the user will not be able to claim rewards for the expanded epoch. This issue can occur for up to a maximum of two epochs.

Let’s consider a scenario: the farm’s start_epoch is 1, and the preliminary_end_epoch is 2, meaning the user can currently claim rewards only for epoch 1.

Now, the user claims their reward during epoch 2. However, since the preliminary_end_epoch is still 2, the user can only claim up to epoch 1. Immediately after the user’s claim transaction, the farm owner expands the farm during the same epoch (epoch 2) for 1 additional epoch. This expansion updates the preliminary_end_epoch to 3.

Even though the user has an open position for epoch 2, they will not be able to claim the reward for epoch 2. This is because their LAST_CLAIMED_EPOCH is now set to 2, and they can only claim rewards starting from epoch 3. However, the user should still be able to claim rewards for epoch 2 as they had an active position during that time.

In the example above, we observed that the user is unable to claim rewards for one epoch, even though they had an open position for that epoch. This issue can extend to an additional epoch if the user claims rewards during the period between farm_ending_at and farm_ending_at.plus_seconds(config.farm_expiration_time), and the farm owner expands the farm within the same time frame but after the user’s claim transaction.

## Impact

The impact is High because the user will not be able to claim their full reward.

## Recommended mitigation steps

We can mitigate this issue by only allowing farm expansion before preliminary_epoch - 1 only.

jvr0x (MANTRA) confirmed 3docSec (judge) commented:

S-387 proposes an alternative solution.

# [M-04] withdraw_liquidity lacks slippage protection

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-dex
- **Source snapshot:** competitions/2024-11-mantra-dex/final_report.html

withdraw_liquidity lacks slippage protection Submitted by Abdessamed, also found by 0x1982us, 0xAlix2, 0xRajkumar, Bauchibred, carrotsmuggler, DadeKuma, Egis_Security, honey-k12, jasonxiale, Sparrow, and Usagi /contracts/pool-manager/src/liquidity/commands.rs#L412-L503

## Vulnerability Details

The withdraw_liquidity function allows users to withdraw assets from a pool in exchange for burning their LP tokens. However, the function does not provide a mechanism for users to specify the minimum amount of tokens they are willing to accept upon withdrawal. This omission exposes users to the risk of receiving fewer tokens than expected due to market conditions especially for ConstantProduct pools between the transaction initiation and execution.

pub fn withdraw_liquidity ( deps: DepsMut, env: Env, info: MessageInfo, pool_identifier:

String, ) -> Result <Response, ContractError> { // --SNIP let refund_assets:

Vec <Coin> = pool.assets.

iter ().

map (|pool_asset| { Ok(Coin { denom: pool_asset.denom.

clone (), amount: Uint128::

try_from ( Decimal256::

from_ratio (pool_asset.amount, Uint256::

one ()).

checked_mul (share_ratio)?.

to_uint_floor (), )?, }).

collect::< Result < Vec <Coin>, ContractError>>()?.

into_iter () // filter out assets with zero amount.

filter (|coin| coin.amount > Uint128::

zero ()).

collect (); let mut messages:

Vec <CosmosMsg> = vec!

[]; // Transfer the refund assets to the sender messages.

push (CosmosMsg::

Bank (BankMsg::

Send { to_address: info.sender.

to_string (), amount: refund_assets.

clone (), })); // --SNIP } As seen above, the function directly calculates the refund amounts and transfers them to the user without any check for slippage or allowing the user to specify a minimum acceptable amount.

## Impact

Users withdrawing their liquidity can receive less amount than they expected.

Mitigations Consider allowing users to provide minimum amount of tokens to receive.

jvr0x (MANTRA) disputed and commented:

Users, especially in xyk pools will face impermanent loss when providing liquidity into the pool. That’s why there are swap fees going to LPers, to compensate in a way for that potential loss. While having slippage protection in the withdrawal function can help, it would prevent users going out of the pool if the minimum received tokens don’t match their expectation.

Will consider it as potential improvement though. This is low, not a medium issue.

3docSec (judge) commented:

Impermanent loss is implicit, but it’s reasonable to expect a protection during high volatility - the industry standard of XYK pools (uniswap v2) does allow users to provide amountAMin and amountBMin; they are free to set them to 0 if they want the withdrawal to always succeed.

# [M-05] Insufficient check on asset decimals input in create_pool allows malicious pool to be created with invalid swap results

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-dex
- **Source snapshot:** competitions/2024-11-mantra-dex/final_report.html

create_pool allows malicious pool to be created with invalid swap results Submitted by oakcobalt create_pool is permissionless and asset decimals are inputs from pool creators.

The vulnerability is there are insufficient check on asset decimals are valid. If a pool is created with incorrect asset decimals, stableswap will use incorrect decimals to scale assets, resulting in invalid swap results.

## Recommended mitigation steps

I don’t see a direct way to query asset decimals during create_pool for verification. However, the protocol can implement a registry contract with trusted token metadata info, such that create_pool can query the registry contract to validate asset decimals are correct atomically.

jvr0x (MANTRA) acknowledged and commented:

Valid point, but that solution was already thought through. If there was such an asset registry contract, it would need to be gated, and then the pool manager wouldn’t really be permissionless.

a_kalout (warden) commented:

@3docSec - I agree that this is valid, but I respectfully believe it should be low/QA at best.

Ok, a malicious user can create a pool with messed-up decimals, but there’s nothing that obliges users to use that pool. If only one pool was allowed for X denoms, ok, that would be a medium severity issue as users who want to swap these denoms are obligated to use that pool. Users can provide any pool identifier they want when swapping or when providing/withdrawing liquidity. If a user used that “malicious” pool identifier then that’s a user error!

carrotsmuggler (warden) commented:

This should be marked low/QA. The reason is that the exploit hinges on users adding liquidity to misconfigured pools. The data is on-chain for all to see, so sophisticated users can just check if the decimals are configured correctly.

Also, if the pool decimals are configured differently, it will be obvious when simulating swaps or liquidity addition, which is mitigated by slippage. So if a user accepts a transaction with very high slippage where they swap 10 usdc for 1 usdt or something, that’s on them.

oakcobalt (warden) commented:

@carrotsmuggler - for a stableswap pool, liquidity addition and its slippage calculation rely on correct assets decimals for correct results. If a stableswap pool has incorrect asset decimals, the slippage check for liquidity is also incorrect. In this case, a user cannot rely on slippage protection to prevent loss.

oakcobalt (warden) commented:

Vulnerable case:

provide_liquidity in a stableswap pool with incorrect asset decimals will result in incorrect liquidity calculation and incorrect slippage implementation. A user cannot rely on slippage protection to prevent losses.

Attack:

Sophisticated users can sandwich less sophisticated users on a pool with incorrect asset decimals. This also gives incentives for malicious pool creators to profit from pool users.

Note that pools with incorrect asset decimals are not correctable. It’s different from a faulty initialization pool price that can be arbitraged to normal price.

Based on C4 guideline assets can be at risks out of users control, with attack path. I think it should be Medium.

2 — Med: Assets not at direct risk, but the function of the protocol or its availability could be impacted, or leak value with a hypothetical attack path with stated assumptions, but external requirements.

3docSec (judge) commented:

I agree with Medium. It’s a non-obvious attack path that can fall through users’ due diligence checks, and the ineffectiveness of slippage protection is key here.

# [M-06] Spread calculation does not account for swap fees

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-dex
- **Source snapshot:** competitions/2024-11-mantra-dex/final_report.html

Submitted by Abdessamed, also found by 0x1982us, 0xRajkumar, and DOWSERS /contracts/pool-manager/src/helpers.rs#L182-L185

## Vulnerability Details

The spread represents the difference between the expected trade amount and the actual trade amount received during a swap. It is calculated to verify that the slippage remains within the user-defined tolerance during the swap operation. The spread formula varies depending on the pool type:

pub fn compute_swap ( n_coins: Uint256, offer_pool: Uint128, ask_pool: Uint128, offer_amount: Uint128, pool_fees: PoolFee, swap_type: &PoolType, offer_precision:

u8, ask_precision:

u8, ) -> Result <SwapComputation, ContractError> { // --SNIP match swap_type { PoolType::ConstantProduct => { let return_amount: Uint256 = Decimal256::

from_ratio (ask_pool.

mul (offer_amount), offer_pool + offer_amount).

to_uint_floor (); let exchange_rate = Decimal256::

checked_from_ratio (ask_pool, offer_pool).

map_err (|_| ContractError::PoolHasNoAssets)?; let spread_amount: Uint256 = (Decimal256::

from_ratio (offer_amount, Uint256::

one ()).

checked_mul (exchange_rate)?.

to_uint_floor ()) @>>>.

checked_sub (return_amount)?; // --SNIP let fees_computation: FeesComputation = compute_fees (pool_fees, return_amount)?; Ok( get_swap_computation ( return_amount, spread_amount, fees_computation, )?) } PoolType::StableSwap { amp } => { // --SNIP let return_amount = ask_pool.

to_uint256_with_precision ( u32::

from (ask_precision))?.

checked_sub (Uint256::

from_uint128 (new_pool))?; // the spread is the loss from 1:1 conversion // thus is it the offer_amount - return_amount let spread_amount = offer_amount.

to_uint256_with_precision ( u32::

from (ask_precision))?

@>>>.

saturating_sub (return_amount); let fees_computation = compute_fees (pool_fees, return_amount)?; Ok( get_swap_computation ( return_amount, spread_amount, fees_computation, )?) } The issue, as highlighted above, is the fact that the spread is calculated based on the return_amount without excluding the fees. As a result, the calculated spread_amount is higher than intended in which the slippage tolerance check in assert_max_spread may pass, when it should not.

## Impact

The calculated spread_amount is higher than the actual slippage because it includes the swap fees.

## Recommended mitigation steps

Consider subtracting the fees from return_amount before calculating the spread_amount.

jvr0x (MANTRA) confirmed

# [M-07] query_reverse_simulation doesn’t account for extra fees when simulating stable reversed swaps

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-dex
- **Source snapshot:** competitions/2024-11-mantra-dex/final_report.html

query_reverse_simulation doesn’t account for extra fees when simulating stable reversed swaps Submitted by 0xAlix2 /contracts/pool-manager/src/queries.rs#L117-L120

## Finding description and impact

Mantra only allows exact-in swaps; however, it provides a way for traders to compute a required amount to get an exact amount out. This could be done using query_reverse_simulation, from the code comments:

/// Queries a swap reverse simulation. Used to derive the number of source tokens returned for /// the number of target tokens.

query_reverse_simulation considers all the variable factors that affect the amounts in and out, most importantly fees. However, the issue is that for stable swaps it doesn’t consider extra fees, it just account swap, protocol, and burn fees:

let before_fees = (Decimal256::

one ().

checked_sub (pool_fees.protocol_fee.

to_decimal_256 ())?.

checked_sub (pool_fees.swap_fee.

to_decimal_256 ())?.

checked_sub (pool_fees.burn_fee.

to_decimal_256 ())?).

inv ().

unwrap_or_else (Decimal256::one).

checked_mul (Decimal256::

decimal_with_precision ( ask_asset.amount, ask_decimal, )?)?; This leads to a wrong offer_amount returned, leading to a wrong ask return_amount, ultimately leading to unexpected results for traders and unexpected reverts to contracts built on top of this.

## Recommended mitigation steps

Make sure extra fees are accounted for in the before_fees calculation:

let before_fees = (Decimal256::

one ().

checked_sub (pool_fees.protocol_fee.

to_decimal_256 ())?.

checked_sub (pool_fees.swap_fee.

to_decimal_256 ())?.

checked_sub (pool_fees.burn_fee.

to_decimal_256 ())?.

checked_sub (pool_fees.extra_fees.

iter ().

fold ( Decimal256::

zero (), |acc, fee| { acc.

checked_add (fee.

to_decimal_256 ()).

unwrap_or (Decimal256::

zero ()) }, ))?).

inv ().

unwrap_or_else (Decimal256::one).

checked_mul (Decimal256::

decimal_with_precision ( ask_asset.amount, ask_decimal, )?)?; 3docSec (judge) commented:

Looks valid, can be intended behavior though.

jvr0x (MANTRA) disputed and commented:

This is a valid issue; however, the extra fees were quickly added in one of the subsequent commits after the v1.0.0 tag was done.

This is how the current code on chain looks like:

let mut extra_fees = Decimal256::

zero (); for extra_fee in pool_fees.extra_fees.

iter () { extra_fees = extra_fees.

checked_add (extra_fee.

to_decimal_256 ())?; } let before_fees = (Decimal256::

one ().

checked_sub (pool_fees.protocol_fee.

to_decimal_256 ())?.

checked_sub (pool_fees.swap_fee.

to_decimal_256 ())?.

checked_sub (pool_fees.burn_fee.

to_decimal_256 ())?).

checked_sub (extra_fees)?.

inv ().

unwrap_or_else (Decimal256::one).

checked_mul (Decimal256::

decimal_with_precision ( ask_asset.amount, ask_decimal, )?)?; 3docSec (judge) commented:

I consider this valid because we have to base on the commit where scope was frozen.

# [M-08] compute_offer_amount floors the offer_amount when simulating constant product reversed swaps, leading to unexpected results

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Finding ID:** M-08
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-dex
- **Source snapshot:** competitions/2024-11-mantra-dex/final_report.html

compute_offer_amount floors the offer_amount when simulating constant product reversed swaps, leading to unexpected results Submitted by 0xAlix2 /contracts/pool-manager/src/helpers.rs#L355-L364

## Finding description and impact

Mantra only allows exact-in swaps; however, it provides a way for traders to compute a required amount to get an exact amount out. This could be done using query_reverse_simulation, from the code comments:

/// Queries a swap reverse simulation. Used to derive the number of source tokens returned for /// the number of target tokens.

query_reverse_simulation calls compute_offer_amount, which computes offer_amount but floors the result:

let cp: Uint256 = offer_asset_in_pool * ask_asset_in_pool; let offer_amount: Uint256 = Uint256::

one ().

multiply_ratio ( cp, ask_asset_in_pool.

checked_sub ( Decimal256::

from_ratio (ask_amount, Uint256::

one ()).

checked_mul (inv_one_minus_commission)?.

to_uint_floor (), )?, ).

checked_sub (offer_asset_in_pool)?; The calculation here differs a bit from the calculation in compute_swap, this results in some dust differences. The main issue in the above is that it rounds the result down, which could result in underpayment, ultimately leading to unexpected results for traders and unexpected reverts to contracts built on top of this.

NB: It is better to pay some extra dust amount and get an expected amount out, rather than paying dust amount less and getting an amount less than expected.

## Recommended mitigation steps

Round the offer_amount up, to make sure that the user is receiving the expected amount:

pub fn compute_offer_amount( offer_asset_in_pool: Uint128, ask_asset_in_pool: Uint128, ask_amount: Uint128, pool_fees: PoolFee, ) -> StdResult<OfferAmountComputation> { //...

let offer_amount: Uint256 = Uint256::one().multiply_ratio( cp, ask_asset_in_pool.checked_sub( Decimal256::from_ratio(ask_amount, Uint256::one()).checked_mul(inv_one_minus_commission)?.to_uint_floor(), - )?, + )?.checked_sub(Uint256::one())?, ).checked_sub(offer_asset_in_pool)?; //...

} jvr0x (MANTRA) acknowledged and commented:

Known issue. However, to convert from Decimal to Uint there’s a sacrifice to pay, you can either floor or round up. Rounding up, what you suggest with paying some more dust, could lead to the contract leaking value and eventually leak value from pools which should not happen.

3docSec (judge) decreased severity to Low and commented:

Marked low because the impact is limited to dust.

a_kalout (warden) commented:

@jvr0x - I agree that there’s a tradeoff here; however, we have 2 options:

Round down, and return wrong results to the user. For example, I want X amount of T1 out, I simulate that swap by calling query_reverse_simulation, and I get an amount Y needed of T2. The catch is that swapping Y of T2 for T1 gives me X-Z, which could easily lead to unexpected reverts for swappers, but no dust left in the contract.

Roundup, and have the needed amount of T2 as Y+Z (where Z is dust), in that case when I swap Y+Z of T2 in return for T1, I get exactly what I want of T1 X (and maybe some dust), but some dust could end up on the contract.

These are the only 2 options we have here with their tradeoffs, the protocol is going with the first, which I disagree with, as it has worse consequences than the second. I believe the second option is what should be followed in this case.

jvr0x (MANTRA) commented:

Ah, I had misinterpreted your point. Probably better to round up on the reverse simulation to be on the safe side, you are right.

3docSec (judge) increased severity to Medium and commented:

I’d stick with valid Medium here, because the use-case for the very reasonable “exact output” flow is off-by-one.

# [M-09] Single sided liquidity can’t be used to lock LP tokens in the farm manager

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Finding ID:** M-09
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-dex
- **Source snapshot:** competitions/2024-11-mantra-dex/final_report.html

Submitted by 0xAlix2, also found by 0xRajkumar, Abdessamed, carrotsmuggler, Egis_Security, and Tigerfrake /contracts/pool-manager/src/liquidity/commands.rs#L282-L286

## Finding description and impact

When users provide liquidity into different pools, they call the provide_liquidity function, they are also allowed to pass unlocking_duration, if provided, the minted LP shares are locked in the farm manager for extra rewards, see here. The protocol doesn’t allow users to open positions in the farm manager on behalf of other users, from the docs:

Note: It’s only possible to lock an LP position in the farm for the same user providing the liquidity, and not do it on behalf of another user.

It gets validated using the following:

// check if receiver is the same as the sender of the tx ensure!

( receiver == info.sender.

to_string (), ContractError::Unauthorized ); On the other hand, when providing single-sided liquidity, half of the deposited amount is swapped to the other asset and then deposited, the process is as follows:

provide_liquidity -> swap -> rely -> provide_liquidity.

However, the issue here is that the second provide_liquidity gets called from the contract itself, i.e., info.sender is the contract address itself, which forces the above receiver validation to revert wrongly.

This blocks users from depositing single-sided liquidity and locks the LP tokens in the farm manager in a single action, breaking a core functionality.

## Recommended mitigation steps

Modify the receiver validation in the if let Some(unlocking_duration) = unlocking_duration block, to bypass the condition if the sender is the contract itself.

ensure!

( receiver == info.sender.

to_string () || info.sender == env.contract.address, ContractError::Unauthorized ); To ensure that this doesn’t cause any flaws, a receiver validation should be added to the if is_single_asset_provision block, to ensure that the sent receiver is valid, is it enough to do the following:

ensure!

( receiver == info.sender.

to_string (), ContractError::Unauthorized ); jvr0x (MANTRA) confirmed

# [M-10] Protocol fees are mistakenly configured by protocol pools rather than being imposed

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Finding ID:** M-10
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-dex
- **Source snapshot:** competitions/2024-11-mantra-dex/final_report.html

Submitted by Abdessamed /contracts/pool-manager/src/manager/commands.rs#L81

## Impact

For every swap operation, the protocol is entitled to a get fees from the tokens out, the sponsor has confirmed that fees should be taken for every swap. However, the current implementation incorrectly gives the protocol_fee control to pool creators when creating pool:

pub fn create_pool ( deps: DepsMut, env: Env, info: MessageInfo, asset_denoms:

Vec < String >, asset_decimals:

Vec < u8 >, pool_fees: PoolFee, pool_type: PoolType, pool_identifier:

Option < String >, ) -> Result <Response, ContractError> { // --SNIP POOLS.

save ( deps.storage, &identifier, &PoolInfo { pool_identifier: identifier.

clone (), asset_denoms, pool_type: pool_type.

clone (), lp_denom: lp_asset.

clone (), asset_decimals, pool_fees, assets, }, )?; } Pool creators can specify 0 fees for pool_fees.protocol_fee, causing the protocol to lose swap fees they are entitled to.

## Recommended mitigation steps

Consider moving the protocol_fee to the contract’s config, rather than being controlled by pool creators.

jvr0x (MANTRA) confirmed 3docSec (judge) commented:

Looks valid, probably Low though, as there is no impact on users.

DadeKuma (warden) commented:

This should be Low/info at most because pool_fees.protocol_fee is a “frontend” fee (i.e., a fee charged for using the user interface that facilitates interaction with these contracts). This is common in many protocols.

The main fees, which are also implemented correctly (and fetched from the config), are the pool creation fee and the token factory fee.

Abdessamed (warden) commented:

@DadeKuma - the fees you are talking about are the pool creation fees, which is implemented correctly and the report does not speak about these fees at all.

What the report is highlighting is that the swap fees are given control to the pool creators mistakenly rather than being imposed by the protocol’s configuration whereby pool creators can simply put zero fees for the protocol team to prevent them from taking swap fees for whatever reason. The sponsor intends to take fees from every swap operation to collect revenue, which is not implemented.

@3docSec - there is no impact for pool creators but at the expense of protocol losses. The protocol team intends to collect revenue from swap fees (like Uniswap and other AMMs do), and not receiving those fees is a clear loss for the protocol team, and thus, this report is eligible for Medium severity.

3docSec (judge) commented:

@Abdessamed - I agree. I do not think this is a likely attack vector because the protocol can:

Force through the UI a proper value if creators create pools through the UI.

Not show in the UI pools that don’t have an adequate protocol fee for swap users.

We are a bit borderline here because of likelihood, but Medium seems justified to me.

# [M-11] When a user single-side deposit into a pool, slippage protection is invalid

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Finding ID:** M-11
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-dex
- **Source snapshot:** competitions/2024-11-mantra-dex/final_report.html

Submitted by oakcobalt, also found by Egis_Security When a user single-side deposit into a pool, a swap is performed first before liquidity provision.

The vulnerability is belief_price is always set to NONE which causes slippage protection to be invalid in some cases.

//contracts/pool-manager/src/liquidity/commands.rs pub fn provide_liquidity (...

) -> Result <Response, ContractError> {...

let is_single_asset_provision = deposits.

len () == 1usize; if is_single_asset_provision {...

Ok(Response::

default ().

add_submessage (SubMsg::

reply_on_success ( wasm_execute ( env.contract.address.

into_string (), &ExecuteMsg::Swap { ask_asset_denom, |> belief_price: None, max_spread, receiver: None, pool_identifier, }, vec!

[swap_half], )?, SINGLE_SIDE_LIQUIDITY_PROVISION_REPLY_ID, )).

add_attributes ( vec!

[( "action", "single_side_liquidity_provision" )])) } else { /contracts/pool-manager/src/liquidity/commands.rs#L164

## Recommended mitigation steps

Consider revising provide_liquidity to allow user to pass a belief_price for single-sided swap flow.

jvr0x (MANTRA) confirmed and commented:

Valid, probably low.

a_kalout (warden) commented:

When I, as a user, am providing liquidity to a pool, why would I care about the underlying swap that is happening behind the scenes? I don’t think it would make sense to put “conditions” (slippage) on that swap. What I care about is the result of providing liquidity and the shares minted to me, which are validated using slippage_tolerance.

As a result, I believe slippage_tolerance is more than enough as slippage protection, even when providing single-side liquidity. I believe this could be a low-severity issue.

carrotsmuggler (warden) commented:

The issue states that slippage is not set, which is incorrect since max_spread is used. The issue then says max_spread is not good for stableswap and constant products.

max_spread not being good is already covered in F-37, so this is not a new path and is basically just triggering a flaw reported in another issue.

The max_spread implementation on constant product pool does work. It calculates ask_pool / offer_pool, the current spot price of the pool, which is a measure of the amount of price change the user expects. The submitter says that “Both constant product pool and stableswap pool’s slippage protection can be invalid.”, but does not prove that statement beyond what is already reported in F-37.

Furthermore, the issue here shows that belief_price is being ignored and max_spread is being used instead.

oakcobalt (warden) commented:

The max_spread implementation on constant product pool does work. It calculates ask_pool / offer_pool … max_spread deals with spread_amount /( return_amount + spread_amount ). it uses the spot price ( ask_pool / offer_pool ) when belief_price is not provided.

the current spot price of the pool, which is a measure of the amount of price change the user expects.

This is not entirely correct. The user cannot measure the price change based on spot price without the belief_price, which is the vulnerable case this report deals with.

max_spread not being good is already covered in F-37, so this is not a new path and is basically just triggering a flaw reported in another issue.

Not true. F-37 only deals with stableswap case, and it has to do with a comparison logic specific in the stableswap branch. It doesn’t consider constant product pool.

oakcobalt (warden) commented:

The point here is not that there is no slippage check for provide_liquidity, it’s that when belief_price is none when single side depositing, existing slippage calculation is invalid.

For a constant product pool:

When belief_price is none, user cannot control how much ask tokens to transfer out, which means the user cannot control the asset ratio ( ask_token / ask_pool, or offer_token / offer_pool ) which determines actual minted LPs.

max_spread doesn’t prevent slippage without belief_price. It doesn’t care how much price moved before the swap.

slippage_tolerance is intended to work when no swaps, i.e., user deposits pool reserve ratio → deposit ratio is fixed by user. It only cares about input deposits ratio’s deviation from the pool reserve ratio. ( ask_token / offer_token vs ask_pool / offer_pool ).

However, because of (1), both deposit ratio and pool reserve ratio are moving targets, resulting in invalid slippage_tolerance’s comparison.

Example:

User provides 10000 DAI to a constant product pool with 20% slippage for simplicity.

Pool’s total lp: 316 Case1:

Pool’s spot price before swap: 5000 DAI, 20 WETH swap: → 5000 DAI, 10 WETH two-sided provide_liquidity:

pool reserve: 10000 DAI, 10 WETH deposit[0]/deposit[1] = 500 pool[0]/pool[1] = 1000 pool[0]/pool[1] > deposit[0]/ (deposit[1] * 80%) → slippage check fails.

Theoretical Lp mints: 158 Case2:

Pool’s spot price before swap: 20000 DAI, 5 WETH swap: → 5000 DAI, 1 WETH two-sided provide_liquidity:

pool reserve: 25000 DAI, 4 WETH deposit[0]/deposit[1] = 5000 pool[0]/pool[1]= 6250 (deposit[0]/deposit[1])* 80% ≤ pool[0]/pool[1] ≤ deposit[0]/ (deposit[1] * 80%) → slippage check pass.

Lp mints: 63 As seen, Case2’s slippage check passed; however, actual minted share is much lower compared to Case1.

# [M-12] Insufficient intermediate value precision in StableSwap calculations

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Finding ID:** M-12
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-dex
- **Source snapshot:** competitions/2024-11-mantra-dex/final_report.html

Submitted by Lambda The calculate_stableswap_y function uses Uint256 for intermediate calculations which can cause arithmetic overflow errors when dealing with large token amounts, especially for tokens with high decimal places (e.g. 18 decimals). This significantly impacts the reliability of the StableSwap pool implementation.

## Recommended mitigation steps

Replace Uint256 with Uint512 for intermediate calculations in calculate_stableswap_y. This provides sufficient precision to handle large token amounts with high decimal places.

pub fn calculate_stableswap_y ( n_coins: Uint256, offer_pool: Decimal256, ask_pool: Decimal256, offer_amount: Decimal256, amp: & u64, ask_precision:

u8, direction: StableSwapDirection, ) -> Result <Uint128, ContractError> { // Convert inputs to Uint512 for intermediate calculations let ann = Uint512::

from (Uint256::

from_u128 ((*amp).

into ()).

checked_mul (n_coins)?); //... rest of calculations using Uint512 } The final result can still be converted back to Uint128 since the actual swap amounts will fit within that range. This change ensures the contract can handle realistic token amounts while maintaining precision in the StableSwap calculations.

A test suite should be added that verifies the contract handles large token amounts correctly across different pool configurations and swap scenarios.

jvr0x (MANTRA) confirmed

# [M-13] Wrong simulation function used in reverse operation path

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Finding ID:** M-13
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-dex
- **Source snapshot:** competitions/2024-11-mantra-dex/final_report.html

Submitted by Rhaydden, also found by Tigerfrake reverse_simulate_swap_operations function incorrectly uses query_simulation instead of query_reverse_simulation when calculating multi-hop trades in reverse. As a result, users will get incorrect price calculations when they attempt to determine how many input tokens they need for a desired output amount.

In summary:

Users receive incorrect price quotes for trades.

The error compounds in multi-hop trades, causing issues.

## Recommended mitigation steps

pub fn reverse_simulate_swap_operations( deps: Deps, ask_amount: Uint128, operations: Vec<SwapOperation>, ) -> Result<SimulateSwapOperationsResponse, ContractError> { let operations_len = operations.len(); if operations_len == 0 { return Err(ContractError::NoSwapOperationsProvided); } let mut amount = ask_amount; for operation in operations.into_iter().rev() { match operation { SwapOperation::MantraSwap { token_in_denom, token_out_denom, pool_identifier, } => { - let res = query_simulation( + let res = query_reverse_simulation( deps, coin(amount.u128(), token_out_denom), token_in_denom, pool_identifier, )?; - amount = res.return_amount; + amount = res.offer_amount; } Ok(SimulateSwapOperationsResponse { amount })

} jvr0x (MANTRA) disputed and commented:

This is something that was fixed in a subsequent commit after the v1.0.0 tag was created. The current (live) reverse query code is the following:

pub fn reverse_simulate_swap_operations ( deps: Deps, ask_amount: Uint128, operations:

Vec <SwapOperation>, ) -> Result <ReverseSimulateSwapOperationsResponse, ContractError> { let operations_len = operations.

len (); if operations_len == 0 { return Err(ContractError::NoSwapOperationsProvided); } let mut offer_in_needed = ask_amount; let mut spreads:

Vec <Coin> = vec!

[]; let mut swap_fees:

Vec <Coin> = vec!

[]; let mut protocol_fees:

Vec <Coin> = vec!

[]; let mut burn_fees:

Vec <Coin> = vec!

[]; let mut extra_fees:

Vec <Coin> = vec!

[]; for operation in operations.

into_iter ().

rev () { match operation { SwapOperation::MantraSwap { token_in_denom, token_out_denom, pool_identifier, } => { let res = query_reverse_simulation ( deps, coin (offer_in_needed.

u128 (), token_out_denom.

clone ()), token_in_denom, pool_identifier, )?; if res.spread_amount > Uint128::

zero () { spreads.

push ( coin (res.spread_amount.

u128 (), &token_out_denom)); } if res.swap_fee_amount > Uint128::

zero () { swap_fees.

push ( coin (res.swap_fee_amount.

u128 (), &token_out_denom)); } if res.protocol_fee_amount > Uint128::

zero () { protocol_fees.

push ( coin (res.protocol_fee_amount.

u128 (), &token_out_denom)); } if res.burn_fee_amount > Uint128::

zero () { burn_fees.

push ( coin (res.burn_fee_amount.

u128 (), &token_out_denom)); } if res.extra_fees_amount > Uint128::

zero () { extra_fees.

push ( coin (res.extra_fees_amount.

u128 (), &token_out_denom)); } offer_in_needed = res.offer_amount; } spreads = aggregate_coins (spreads)?; swap_fees = aggregate_coins (swap_fees)?; protocol_fees = aggregate_coins (protocol_fees)?; burn_fees = aggregate_coins (burn_fees)?; extra_fees = aggregate_coins (extra_fees)?; Ok(ReverseSimulateSwapOperationsResponse { offer_amount: offer_in_needed, spreads, swap_fees, protocol_fees, burn_fees, extra_fees, }) } 3docSec (judge) commented:

For judging, we base on the commit frozen for the audit scope; so while it’s good to see the team found and fixed the issue independently, it wouldn’t be fair to wardens to not reward a valid finding.

# [M-14] Amplifiers can’t be ramped allowing loss of funds from the pool

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Finding ID:** M-14
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-dex
- **Source snapshot:** competitions/2024-11-mantra-dex/final_report.html

Submitted by Bauchibred The Mantra DEX stableswap implementation lacks the ability to modify the amplification coefficient (A) after pool creation, which is a critical feature present even in the original Curve implementation. While the focus was on reducing Newton-Raphson iterations from 256 to 32, the absence of amplifier ramping breaks the logic.

In Mantra’s implementation, the amplification factor is static:

/packages/amm/src/pool_manager.rs#L85-L94 pub enum PoolType { StableSwap { /// The amount of amplification to perform on the constant product part of the swap formula.

amp:

u64, }, ConstantProduct, } Once set during pool creation, there is no mechanism to modify this value. In contrast, Curve’s implementation includes comprehensive amplifier management, this can be seen here:

def ramp_A(_future_A: uint256, _future_time: uint256):

assert msg.sender == self.owner # dev: only owner assert block.timestamp >= self.initial_A_time + MIN_RAMP_TIME assert _future_time >= block.timestamp + MIN_RAMP_TIME # dev: insufficient time initial_A: uint256 = self._A() future_A_p: uint256 = _future_A * A_PRECISION assert _future_A > 0 and _future_A < MAX_A if future_A_p < initial_A:

assert future_A_p * MAX_A_CHANGE >= initial_A else:

assert future_A_p <= initial_A * MAX_A_CHANGE self.initial_A = initial_A self.future_A = future_A_p self.initial_A_time = block.timestamp self.future_A_time = _future_time Note that the Amplification Coefficient is a crucial feature for managing the pool’s behavior and adapting to changing market conditions.

Before going further we understand that the stableswap invariant is enforced by the equation:

An∑xi + D = ADⁿ + (D^(n+1))/(n^n∏xi), this can be seen in compute_y_raw(), that gets called from compute_y().

Then, this directly translates to how we mint the amount of liquidity say when a user is making a stable swap deposit when providing liquidity:

/contracts/pool-manager/src/liquidity/commands.rs#L256-L267 pub fn provide_liquidity () { //..snip //@audit below we route the call when providing the liquidity to the stableswap implementation } else { compute_lp_mint_amount_for_stableswap_deposit ( amp_factor, // pool_assets hold the balances before the deposit was made &pool_assets, // add the deposit to the pool_assets to calculate the new balances & add_coins (pool_assets.

clone (), deposits.

clone ())?, total_share, )?.

ok_or (ContractError::StableLpMintError)?

} //..snip } /contracts/pool-manager/src/helpers.rs#L790-L812 pub fn compute_lp_mint_amount_for_stableswap_deposit ( amp_factor: & u64, old_pool_assets: &[Coin], new_pool_assets: &[Coin], pool_lp_token_total_supply: Uint128, ) -> Result < Option <Uint128>, ContractError> { // Initial invariant let d_0 = compute_d (amp_factor, old_pool_assets).

ok_or (ContractError::StableInvariantError)?; // Invariant after change, i.e. after deposit // notice that new_pool_assets already added the new deposits to the pool let d_1 = compute_d (amp_factor, new_pool_assets).

ok_or (ContractError::StableInvariantError)?; // If the invariant didn't change, return None if d_1 <= d_0 { Ok(None) } else { let amount = Uint512::

from (pool_lp_token_total_supply).

checked_mul (d_1.

checked_sub (d_0)?)?.

checked_div (d_0)?; Ok(Some(Uint128::

try_from (amount)?)) } That’s to say essentially when we are trying to calculate the swap amount, see here:

pub fn compute_y_raw ( n_coins:

u8, amp_factor: & u64, swap_in: Uint128, //swap_out: Uint128, no_swap: Uint128, d: Uint512, ) -> Option <Uint512> { let ann = amp_factor.

checked_mul (n_coins.

into ())?; // A * n ** n // sum' = prod' = x // c = D ** (n + 1) / (n ** (2 * n) * prod' * A) let mut c = d; c = c.

checked_mul (d).

unwrap ().

checked_div (swap_in.

checked_mul (n_coins.

into ()).

unwrap ().

into ()).

unwrap (); c = c.

checked_mul (d).

unwrap ().

checked_div (no_swap.

checked_mul (n_coins.

into ()).

unwrap ().

into ()).

unwrap (); c = c.

checked_mul (d).

unwrap ().

checked_div (ann.

checked_mul (n_coins.

into ()).

unwrap ().

into ()).

unwrap (); // b = sum(swap_in, no_swap) + D // Ann - D // not subtracting D here because that could result in a negative.

let b = d.

checked_div (ann.

into ()).

unwrap ().

checked_add (swap_in.

into ()).

unwrap ().

checked_add (no_swap.

into ()).

unwrap (); // Solve for y by approximating: y**2 + b*y = c let mut y_prev: Uint512; let mut y = d; for _ in 0..

1000 { y_prev = y; // y = (y * y + c) / (2 * y + b - d); let y_numerator = y.

checked_mul (y).

unwrap ().

checked_add (c).

unwrap (); let y_denominator = y.

checked_mul (Uint512::

from ( 2u8 )).

unwrap ().

checked_add (b).

unwrap ().

checked_sub (d).

unwrap (); y = y_numerator.

checked_div (y_denominator).

unwrap (); if y > y_prev { if y.

checked_sub (y_prev).

unwrap () <= Uint512::

one () { break; } else if y_prev.

checked_sub (y).

unwrap () <= Uint512::

one () { break; } Some(y) } To explain the bug case more, let’s examine how different A (Amplifier) values affect a 2-token stableswap pool with DAI and USDC ( n=2 ):

Initial Balanced State:

x₁ = 1000 DAI x₂ = 1000 USDC D = 2000 (invariant) With Optimal A = 85:, concluding this as optimal considering this was also hinted in the original Curve implementation:

Left side: An∑xi + D = 85 * 2 * (1000 + 1000) + 2000 = 85 * 2 * 2000 + 2000 = 340,000 + 2000 = 342,000 Right side: ADⁿ + (D^(n+1))/(n^n∏xi) = 85 * 2000² + 2000³/(2² * 1000 * 1000) = 340,000 + 2000 = 342,000 Price impact for 10% imbalance trade ≈ 0.3% With Too High A = 1000:

Left side: An∑xi + D = 1000 * 2 * (1000 + 1000) + 2000 = 1000 * 2 * 2000 + 2000 = 4,000,000 + 2000 = 4,002,000 Right side: ADⁿ + (D^(n+1))/(n^n∏xi) = 1000 * 2000² + 2000³/(2² * 1000 * 1000) = 4,000,000 + 2000 = 4,002,000 Price impact for 10% imbalance trade ≈ 0.025% With Too Low A = 10:

Left side: An∑xi + D = 10 * 2 * (1000 + 1000) + 2000 = 10 * 2 * 2000 + 2000 = 40,000 + 2000 = 42,000 Right side: ADⁿ + (D^(n+1))/(n^n∏xi) = 10 * 2000² + 2000³/(2² * 1000 * 1000) = 40,000 + 2000 = 42,000 Price impact for 10% imbalance ≈ 2.5% This demonstrates how:

Optimal A (85) balances stability with safety.

Too high A (1000) makes the pool vulnerable to manipulation due to minimal price impact.

Too low A (10) causes excessive slippage even for small trades.

## Impact

As already slightly hinted under

## Recommended Mitigation Steps

Implement amplifier ramping functionality similar to Curve:

pub enum ExecuteMsg { RampAmplifier { future_amp:

u64, future_time:

u64, }, StopRamp {}, } And add safety constraints:

const MAX_A:

u64 = 1_000_000; // 10^6 const MAX_A_CHANGE:

u64 = 10; const MIN_RAMP_TIME:

u64 = 3600; // 1 hour 3docSec (judge) commented:

Looks reasonable but more of a missing feature than a bug.

jvr0x (MANTRA) confirmed and commented:

This is a valid point, and indeed a feature that’s missing in the contract. Need to think about it. The reason pools are immutable after creation is to prevent bad actors from creating a pool with favorable conditions/fees to attract liquidity to then change the parameters and manipulate things on their favor, effectively harming the LPs.

Not a high issue, potentially medium.

3docSec (judge) decreased severity to Medium and commented:

I agree Medium makes sense, because pools can leak value with a hypothetical attack path with stated assumptions, but external requirements like market conditions.

# [M-15] Emergency unlocking penalty makes long duration positions economically advantageous

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Finding ID:** M-15
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-dex
- **Source snapshot:** competitions/2024-11-mantra-dex/final_report.html

Submitted by Lambda, also found by Evo The farm-manager contract has a static emergency unlock penalty (initialized to 2% in the deployment file deploy_mantra_dex.sh ) regardless of the position’s unlocking duration. However, longer unlocking durations provide significantly higher reward weight multipliers (up to 16x for 1 year lockups). This creates an economic imbalance where users are incentivized to:

Create positions with maximum unlocking duration to get the highest weight multiplier (up to 16x).

Emergency unlock when they want liquidity, only paying the fixed 2% penalty.

This undermines the intended lockup mechanism since users can get much higher rewards while maintaining effective liquidity through emergency unlocks. The impact is that the protocol’s liquidity stability guarantees are weakened, as users are economically incentivized to game the system rather than maintain their intended lock periods.

Moreover, it can be profitable to open a huge position 1 second before an epoch ends and withdraw immediately (in the new epoch), which hurts real users of the system.

## Recommended mitigation steps

The emergency unlock penalty should scale with:

Remaining lock duration.

Position’s weight multiplier.

Suggested formula:

emergency_penalty = base_penalty * (remaining_duration / total_duration) * (position_weight / base_weight) This would make emergency unlocks proportionally expensive for positions with higher weights and longer remaining durations, better aligning incentives with the protocol’s goals.

Alternative mitigations:

Cap maximum unlock duration to reduce exploitability.

Increase base emergency unlock penalty.

Add minimum hold period before emergency unlocks are allowed.

The key is ensuring the penalty properly counterbalances the increased rewards from longer lock periods.

jvr0x (MANTRA) confirmed and commented:

This is a valid concern and the team is aware of that. However, the way the team intends to instantiate the farm manager at the beginning is to set min and max unlock periods to 1 day, so in that case all positions would be the same.

Will probably address this issue once/if the team decides to increase the constraint.

# [M-16] Liquidity providers can lose tokens due to disproportionate deposits not being properly handled

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Finding ID:** M-16
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-dex
- **Source snapshot:** competitions/2024-11-mantra-dex/final_report.html

Submitted by honey-k12, also found by Bauchibred, jasonxiale, Lambda, Lambda, LonnyFlash, and oakcobalt When providing liquidity to a pool that already has liquidity, users may lose a portion of their deposited tokens if they provide tokens in different proportions relative to the current pool reserves. While the slippage_tolerance parameter protects against receiving too few LP tokens, it doesn’t protect against token loss due to disproportionate deposits.

The provide_liquidity function in the pool manager calculates LP tokens to mint based on the minimum share ratio of provided tokens. When tokens are provided in different proportions relative to the pool’s current reserves, the excess tokens from the higher proportion are effectively donated to the pool. This way, users can lose tokens when providing liquidity with disproportionate amounts.

## Recommended mitigation steps

In the provide_liquidity function, consider calculating optimal token amounts based on the amounts specified by user, current pool reserves, and the minimal LP tokens amount specified by user. As a reference, consider this piece from the Uniswap V2 Router:

UniswapV2Router02.sol#L45-L60.

jvr0x (MANTRA) acknowledged 3docSec (judge) decreased severity to Medium and commented:

I consider this group a valid medium, basing on the following facts:

As said here, frontrunning is difficult to automate.

There is slippage protection in place that is sufficient to avoid considerable losses.

Because extra tokens are not returned; however, the “accidental” frontrun case can lead to non-dust value leakage, which is a solid medium severity impact.

# [M-17] Slippage tolerance vulnerability in StableSwap

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Finding ID:** M-17
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-dex
- **Source snapshot:** competitions/2024-11-mantra-dex/final_report.html

Submitted by DOWSERS, also found by Lambda The assert_slippage_tolerance function does not properly account for the non-linear pricing curve of StableSwap pools, which are influenced by the amplification factor (A). Specifically, the function calculates slippage tolerance based solely on a simple ratio of total deposits and pool tokens without integrating the StableSwap invariant (D). This oversight can lead to the acceptance of transactions with higher actual slippage than allowed, exposing the protocol to user losses or economic inefficiencies.

Link to code /contracts/pool-manager/src/helpers.rs#L423 match pool_type { PoolType::StableSwap {.. } => { let pools_total: Uint256 = pools.

into_iter ().

fold (Uint256::

zero (), |acc, x| acc.

checked_add (x).

unwrap ()); let deposits_total: Uint256 = deposits.

into_iter ().

fold (Uint256::

zero (), |acc, x| acc.

checked_add (x).

unwrap ()); let pool_ratio = Decimal256::

from_ratio (pools_total, pool_token_supply); let deposit_ratio = Decimal256::

from_ratio (deposits_total, amount); if pool_ratio * one_minus_slippage_tolerance > deposit_ratio { return Err(ContractError::MaxSlippageAssertion); }

## Impact

Failure to account for the StableSwap invariant (D) and amplification factor (A) allows for:

Incorrect slippage validation:

Transactions with actual slippage exceeding the user-defined tolerance may be accepted.

User losses:

Users may incur unexpected losses due to high slippage.

Example: Incorrect slippage validation:

Initial pool: [1000, 1000].

Amplification factor (A): 100.

User deposit: [100, 50].

Slippage tolerance: 1% (0.01).

pool_token_supply: 2000.

Amount (pool token received): 150.

Steps:

Calculate total values:

pools_total = 1000 + 1000 = 2000.

deposits_total = 100 + 50 = 150.

Calculate ratios:

pool_ratio = pools_total / pool_token_supply = 2000 / 2000 = 1.0.

deposit_ratio = deposits_total / amount = 150 / 150 = 1.0.

Slippage check:

pool_ratio * (1 - slippage_tolerance) = 1.0 * 0.99 = 0.99.

deposit_ratio = 1.0.

Result:

0.99 > 1.0 is false, so the transaction is accepted.

Real slippage using StableSwap invariant Before deposit:

𝐷initial = calculation based on [1000,1000] and 𝐴 = 100.

Let’s say 𝐷initial = 2000 (perfect equilibrium) After deposit:

[1000 + 100, 1000 + 50] = [1100, 1050].

𝐷final = StableSwap invariant recalculated Let’s say 𝐷final = 2105.

Relative change:

Actual slippage = (𝐷final - 𝐷initial) / 𝐷initial = (2105 - 2000) / 2000 = 0.0525 (5.25%).

Comparison with Tolerance:

Specified tolerance: 1% (0.01).

Actual slippage = 5.25% > 1%, so the transaction should have been rejected.

Risk Likelihood:

Medium, the vulnerability depends on specific deposit patterns, such as highly imbalanced deposits.

Impact:

Medium, users face financial losses.

## Recommended mitigation steps

Implement a StableSwap-specific slippage calculation that incorporates the invariant (D) and amplification factor (A):

Calculate (D) before and after the deposit.

Derive the price impact based on (D) and compare it to the user-defined slippage tolerance.

jvr0x (MANTRA) confirmed

# [M-18] Stablepools return wrong price when they do not converge

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Finding ID:** M-18
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-dex
- **Source snapshot:** competitions/2024-11-mantra-dex/final_report.html

Submitted by DadeKuma /contracts/pool-manager/src/helpers.rs#L751

## Finding description and impact

A StableSwap pool calculates the price using Newton’s method to approximate the D value. This calculation might not converge in unbalanced pools, resulting in a wrong price, and in this case, it shouldn’t be possible to swap.

However, this scenario is not prevented as the transaction will not revert even when the function does not converge.

## Recommended mitigation steps

Consider the following fix:

d = compute_next_d(amp_factor, d, d_prod, sum_x, n_coins).unwrap(); // Equality with the precision of 1 if d > d_prev { if d.checked_sub(d_prev).unwrap() <= Uint512::one() { - break; + return Some(d); } } else if d_prev.checked_sub(d).unwrap() <= Uint512::one() { - break; + return Some(d); } - Some(d) + Err(ContractError::ConvergeError) Abdessamed (warden) commented:

In case a pool diverges, it shouldn’t be possible to swap, but only to withdraw liquidity.

This is incorrect, due to the StableSwap math, there is no straightforward algebraic formula to compute D and instead, Newton’s method is used to estimate the value. In case the pool is highly imbalanced and Newton’s method does not converge, the latest estimated value should be returned rather than reverting the transaction. The pool can easily be balanced again by adding balanced liquidity in subsequent transactions.

The StableSwap pool referenced by the warden is Zaps pool and it is a non-standard pool. You can see that in 3pool which is one of the most used pools in Curve, the get_D does NOT revert and instead returns the best-estimated value DadeKuma (warden) commented:

If Newton’s method does not converge, it will return the wrong price; this is simply how an iterative algorithm works. You shared a specific pool implementation: a 3-token pool designed for DAI/USDC/USDT, as seen in this comment.

What I shared is a normal 2-token stable pool that requires this check, which is definitely standard. For example EURS/sEUR or the pool template from the same repository you have shared.

jvr0x (MANTRA) confirmed and commented:

DadeKuma - does it mean there has to be two cases for when the pool has 2 vs 3 assets?

DadeKuma (warden) commented:

@jvr0x - it’s not related to the number of assets but rather to the expected stability of the pool. For example, with DAI/USDC/USDT, it’s basically impossible for the algorithm to diverge, making this check unnecessary (unlike UST/DAI/USDC/USDT which also has this check ).

However, since this function can be used with any asset in 2, 3, or 4-asset combinations, I recommend implementing this check in any case.

jvr0x (MANTRA) commented:

Seems reasonable.

# [M-19] Vulnerable liquidity slippage calculation doesn’t ensure slippage protection due to unscaled assets sum

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Finding ID:** M-19
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-11-mantra-dex
- **Source snapshot:** competitions/2024-11-mantra-dex/final_report.html

Submitted by oakcobalt /contracts/pool-manager/src/helpers.rs#L426 /contracts/pool-manager/src/helpers.rs#L429

## Finding description and impact

Current slippage calculation for providing liquidity to stableswap pool uses simple sum of unscaled asset balance. This is vulnerable because it doesn’t account for differences in asset decimals, which might cause slippage calculation to be invalid.

## Recommended mitigation steps

Scale the asset to the same decimal before performing sum.

jvr0x (MANTRA) confirmed via chat with C4 staff

## Rejected Primary Findings

# Rejected Primary Findings: MANTRA DEX

# Pending ownership transfer not canceled when the current owner renounces ownership.

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Submission:** F-101
- **Submitter:** thisvishalsingh
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-dex/submissions/F-101
- **Source snapshot:** competitions/2024-11-mantra-dex/submissions/raw/F-101.txt

## Brief Summary

Function: execute (for renounce_ownership in UpdateOwnership) Root Issue: The renounce of ownership action does not invalidate the pending ownership transfer, leaving it active despite the current owner renouncing ownership. The contract does not check for or cancel pending transfers during renouncement. ExecuteMsg::UpdateOwnership(action) => { // @audit RenounceOwnership would be implemented as part of this action } Impact: While this violates logical expectations (a pending transfer being valid after the owner renounces ownership), could cause functional inconsistencies and unexpected contract behavior.

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# `ConstantProduct` invariant is not enforced during liquidity provision

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Submission:** F-24
- **Submitter:** Abdessamed
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-dex/submissions/F-24
- **Source snapshot:** competitions/2024-11-mantra-dex/submissions/raw/F-24.txt

## Brief Summary

ConstantProduct pools rely on the fundamental invariant 𝑥 ∗ 𝑦 = 𝑘 x∗y=k, where 𝑥 x and 𝑦 y represent the reserves of the two assets, and 𝑘 k is a constant. This invariant ensures that the pool maintains a balanced spot price. The provide_liquidity function, however, does not enforce this invariant. When adding liquidity, the spot price after the addition should remain equal to the spot price before the addition ( 𝑑 𝑦 𝑑 𝑥 = 𝑦 𝑥 dx dy ​ = x y ​ ). Currently, the function accepts both asset deposits and directly mints LP tokens without verifying that the spot price is preserved: pub fn provide_liquidity( deps: DepsMut, env: Env, info: MessageInfo, slippage_tolerance: Option<Decimal>, max_sprea...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Farm expansion permits mismatched lp_denom parameters

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Submission:** F-60
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-dex/submissions/F-60
- **Source snapshot:** competitions/2024-11-mantra-dex/submissions/raw/F-60.txt

## Brief Summary

During the expand_farm() function execution, the farm owner can pass any params.lp_denom value, as long as it was created by the current pool_manager_addr. This results in a mismatch between the original lp_denom of the farm and the params.lp_denom used during expansion.

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Emergency‐unlock penalty is equally distributed which results in unfair loss for smaller farms

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Submission:** F-51
- **Submitter:** axelot
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-dex/submissions/F-51
- **Source snapshot:** competitions/2024-11-mantra-dex/submissions/raw/F-51.txt

## Brief Summary

When a user does an emergency unlock, the penalty is divided equally among all owners of farms associated with the lp_asset, ignoring differences in farm size (in terms of lp_asset) and unfairly impacting smaller farms owners.

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Users can extend their farms expiry time by expanding farm with 0 amount

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Submission:** F-80
- **Submitter:** 0xcb90f054
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-dex/submissions/F-80
- **Source snapshot:** competitions/2024-11-mantra-dex/submissions/raw/F-80.txt

## Brief Summary

expand_farm allows the user to expand the farm with given parameters. The function requires that the reward amount to be added to the total asset amount must be a multiple of the farm's emission rate upon which the preliminary_end_epoch which is used to get expiry time is set. But the issue is that the function doesn't check that the reward amount is 0, giving users the ability to extend their farm expiry without actually adding any reward amount.

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# preliminary_end_epoch can allow setting extensive durations that can result in zero emission rates

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Submission:** F-52
- **Submitter:** Evo
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-dex/submissions/F-52
- **Source snapshot:** competitions/2024-11-mantra-dex/submissions/raw/F-52.txt

## Brief Summary

Users who set large preliminary_end_epoch values may unintentionally create farms with zero emission rates, effectively freezing farm rewards, as the emission rate calculation divides the total farm amount by the duration (preliminary_end_epoch - start_epoch).

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# All swaps on a Stableswap pool can fail due to a mismatch in the expected minimum amp value and only enforcing it in the test scope

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Submission:** F-84
- **Submitter:** Bauchibred
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-dex/submissions/F-84
- **Source snapshot:** competitions/2024-11-mantra-dex/submissions/raw/F-84.txt

## Brief Summary

All swaps on a Stableswap pool can fail due to a mismatch in the expected minimum amp value and only enforcing it in the test scope

## Rejection Reason

Final severity/evaluation: Low; Valid Primary Sufficient

# DoS issue when creating a farm with a valid identifier

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Submission:** F-87
- **Submitter:** 0xRajkumar
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-dex/submissions/F-87
- **Source snapshot:** competitions/2024-11-mantra-dex/submissions/raw/F-87.txt

## Brief Summary

In our farm manager, we have a fill_farm feature, which is used for creating new farms and expanding existing ones. When calling fill_farm, the user can pass a farm_identifier to create a new farm. However, if a farm with the given identifier already exists, the function will attempt to expand it. If the user is not the owner of the farm, the operation will revert and return an error. The implementation now appears correct, but if we examine the create_farm function, we see that the identifier is derived by concatenating 'm-' to the provided identifier to generate the farm_identifier. Now, let's say a user wants to create a farm with the identifier m-m-raj. They will need to pass m-raj as t...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Improper penalty distribution logic caused farm owners to not receive their rightful share of profits.

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Submission:** F-50
- **Submitter:** Usagi
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-dex/submissions/F-50
- **Source snapshot:** competitions/2024-11-mantra-dex/submissions/raw/F-50.txt

## Brief Summary

When handling scenarios where users trigger an Emergency Unlock, the protocol charges a penalty fee (total_penalty_fee) to the user, which is expected to be distributed as follows: 50% (owner_penalty_fee_commission) allocated to the respective farm owners (farm_owners). The remaining 50% allocated to the contract's fee collection address (fee_collector). However, when calculating the profit share for each farm owner (penalty_fee_share_per_farm_owner), the program applies to_uint_floor(), truncating the fractional part of the share to an integer. If the penalty fee amount is small and the number of farm owners is large, this can result in each owner’s share being less than 1, which gets trun...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Lack of expiration check

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Submission:** F-78
- **Submitter:** levi_104
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-dex/submissions/F-78
- **Source snapshot:** competitions/2024-11-mantra-dex/submissions/raw/F-78.txt

## Brief Summary

According to the annotation, after the farm expires, anyone can close it and create a new farm. Only the farm creator or the owner of the contract can close a farm, except if the farm has expired, in which case anyone can close it while creating a new farm. But there is no check here to see if it has expired. It must be the owner of the farm or the owner of the contract ensure!( farm.owner == info.sender || cw_ownable::is_owner(deps.storage, &info.sender)?, ContractError::Unauthorized );

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Invalid D validation in `calculate_stableswap_d`, resulting in a wrong returned swap amount

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Submission:** F-13
- **Submitter:** 0xAlix2
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-dex/submissions/F-13
- **Source snapshot:** competitions/2024-11-mantra-dex/submissions/raw/F-13.txt

## Brief Summary

When swapping assets in a stable swap pool, calculate_stableswap_d is called in calculate_stableswap_y, which calculates D (invariant of the pool), this is similar to the compute_d that is called when depositing liquidity into stable swap pools. All the amounts that are provided to calculate_stableswap_d are normalized, i.e. their decimals are stripped, this is done in https://github.com/code-423n4/2024-11-mantra-dex/blob/main/contracts/pool-manager/src/helpers.rs#L196-L198. calculate_stableswap_d computes D and "old" D, and then their subtraction is compared to what is supposed to be 1: if current_d >= old_d { if current_d.checked_sub(old_d)? <= Decimal256::decimal_with_precision(1u8, prec...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# `PoolFees.extra_fees` are stucked in the contract

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Submission:** F-74
- **Submitter:** Egis_Security
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-dex/submissions/F-74
- **Source snapshot:** competitions/2024-11-mantra-dex/submissions/raw/F-74.txt

## Brief Summary

Pool creator can define extra_fees: /// A list of custom, additional fees that can be defined for specific use cases or additional /// functionalities. Later on the swapped they are decreased from the swapped amount in the pool. fn get_swap_computation( return_amount: Uint256, spread_amount: Uint256, fees_computation: FeesComputation, ) -> Result<SwapComputation, ContractError> { let return_amount = return_amount .checked_sub(fees_computation.swap_fee_amount)? .checked_sub(fees_computation.protocol_fee_amount)? .checked_sub(fees_computation.burn_fee_amount)? .checked_sub(fees_computation.extra_fees_amount)?; However, there is no logic to transfer them to any account, or to save them in the...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Spread calculation in `compute_swap` can lead to revert due to underflow

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Submission:** F-41
- **Submitter:** carrotsmuggler
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-dex/submissions/F-41
- **Source snapshot:** competitions/2024-11-mantra-dex/submissions/raw/F-41.txt

## Brief Summary

The compute_swap function in the helpers.rs file calculates the result of the swap. It also has an inbuilt spread calculation. We are interested only in the logic in the constant product part. let return_amount: Uint256 = Decimal256::from_ratio(ask_pool.mul(offer_amount), offer_pool + offer_amount) .to_uint_floor(); let exchange_rate = Decimal256::checked_from_ratio(ask_pool, offer_pool) .map_err(|_| ContractError::PoolHasNoAssets)?; let spread_amount: Uint256 = (Decimal256::from_ratio(offer_amount, Uint256::one()) .checked_mul(exchange_rate)? .to_uint_floor()) .checked_sub(return_amount)?; Point to note is that the last line uses checked_sub, which reverts if the result is negative. In thi...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Incorrect farm asset validation results in creation failures due to lack of coin aggregation

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Submission:** F-64
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-dex/submissions/F-64
- **Source snapshot:** competitions/2024-11-mantra-dex/submissions/raw/F-64.txt

## Brief Summary

The assert_farm_asset() function validates the number of assets sent by comparing the length of info.funds with expected values. However, info.funds may contain multiple entries of the same denomination due to how funds are handled in transactions. This leads to incorrect validation because only one entry is taken into account. If duplicate entries of the same denomination are sent, the validation erroneously rejects the transaction (due insufficient funds) when in reality there are enough funds to complete the transaction.

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Incorrect fee processing logic potentially prevents farm creation even with sufficient funds

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Submission:** F-63
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-dex/submissions/F-63
- **Source snapshot:** competitions/2024-11-mantra-dex/submissions/raw/F-63.txt

## Brief Summary

The current implementation of process_farm_creation_fee() uses .iter().find(...) to locate the first instance of a coin in the info.funds vector that matches the required fee's denomination (farm_creation_fee.denom). However, this approach does not account for cases where the same coin denomination appears multiple times in the info.funds vector. If the user provides the correct total fee spread across multiple instances of the same coin denomination, the function may incorrectly determine that the fee has not been paid in full. This can result in erroneous reversion.

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# to_unit256_with_precision cannot handle assets w/ greater than 18 decimal, might cause stableswap DOS

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Submission:** F-15
- **Submitter:** oakcobalt
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-dex/submissions/F-15
- **Source snapshot:** competitions/2024-11-mantra-dex/submissions/raw/F-15.txt

## Brief Summary

to_unit256_with_precision doesn’t compare self.decimal_places() and precision. When precision > self.decimal_places() (18 decimals), any flows that invoke to_unit256_with_precision will be reverted due to underflow. //contracts/pool-manager/src/math.rs fn to_uint256_with_precision(&self, precision: u32) -> Result<Uint256, ContractError> { let value = self.atomics(); |> Ok(value.checked_div(10u128.pow(self.decimal_places() - precision).into())?) } (https://github.com/code-423n4/2024-11-mantra-dex/blob/26714ea59dab7ecfafca9db1138d60adcf513588/contracts/pool-manager/src/math.rs#L43)

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Incorrect test logic for divide_by_zero edge case, causing invalid test results

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Submission:** F-32
- **Submitter:** oakcobalt
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-dex/submissions/F-32
- **Source snapshot:** competitions/2024-11-mantra-dex/submissions/raw/F-32.txt

## Brief Summary

Tests are in scope according to readme. In contracts/farm-manager/tests/integration.rs, unit test test_query_rewards_divide_by_zero is intended to test that division by zero revert in reward claiming cases have been mitigated by current contract implementation. However, the test result is not valid due to an incorrect test logic. And the impacts of division by zero causing reward claiming DOS persists. Vulnerability: test_query_rewards_divide_by_zero only creats one position, which doesn't validate the common cases where a user have multiple positions in various farms. The only reason the current test passes is because the user only opened one position in the test, so the last_claimed_epoch...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Incorrect Reward Distribution Due To Mutable Epoch Duration

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Submission:** F-56
- **Submitter:** Lambda
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-dex/submissions/F-56
- **Source snapshot:** competitions/2024-11-mantra-dex/submissions/raw/F-56.txt

## Brief Summary

The epoch manager contract allows updating the epoch duration via ExecuteMsg::UpdateConfig, but the farm manager implicitly relies on constant epoch duration for its reward calculations and position tracking. An epoch duration change causes reward distribution errors, breaks position weight calculations, and makes historical LP weight data unreliable. The issue stems from the farm manager storing only epoch IDs in maps like LP_WEIGHT_HISTORY and using these epoch IDs for period-based calculations, while the actual time duration these epochs represent can be changed by the admin. When epoch duration changes, all calculations that map epoch IDs to time periods become invalid. This impacts: Fa...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Wrong constant product pool fee calculation increases k=xy invariant

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Submission:** F-73
- **Submitter:** Lambda
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-dex/submissions/F-73
- **Source snapshot:** competitions/2024-11-mantra-dex/submissions/raw/F-73.txt

## Brief Summary

The constant product pool swap implementation deducts fees from the output amount instead of the input amount, which increases the fundamental k = x * y invariant property after every swap and means that the users get too little tokens. In Uniswap V2 and other major constant product AMMs, fees are charged on the input amount before calculating the output. This ensures that after every swap: The new reserves satisfy k = (x + amount_in * (1 - fee)) * (y - amount_out) = x * y The pool collects fees while maintaining its price discovery mechanism The current implementation instead: Calculates output without fees: amount_out = (y * amount_in) / (x + amount_in) Deducts fees from amount_out Result...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Unrestricted `amp` factor could lead to price manipulation

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Submission:** F-86
- **Submitter:** OxElliot
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-dex/submissions/F-86
- **Source snapshot:** competitions/2024-11-mantra-dex/submissions/raw/F-86.txt

## Brief Summary

The StableSwap pool creation mechanism in the DEX protocol allows users to specify an unbounded amplification factor (amp) during pool creation. If the PoolType is StableSwap, the user must specify the amp value (a u64) and there are no restrictions on the value of amp factor. This oversight enables the creation of pools with extreme amplification values, leading to significant price discrepancies and allowing arbitrage exploitation between pools. An attacker can leverage this vulnerability to manipulate the price dynamics of the pools, systematically draining liquidity and extracting value from liquidity providers. If the PoolType is StableSwap, the user must specify the amp value (a u64)....

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Epoch duration can be updated breaking a core invariant

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Submission:** F-53
- **Submitter:** Bauchibred
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-dex/submissions/F-53
- **Source snapshot:** competitions/2024-11-mantra-dex/submissions/raw/F-53.txt

## Brief Summary

Epoch duration can be updated breaking a core invariant

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# `is_farm_expired` returns wrong results, because of a wrong end time used

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Submission:** F-26
- **Submitter:** 0xAlix2
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-dex/submissions/F-26
- **Source snapshot:** competitions/2024-11-mantra-dex/submissions/raw/F-26.txt

## Brief Summary

When a farm is created, it is created with a start and a preliminary end epoch, that end epoch is not included in the farm rewards emission, this could be confirmed in https://github.com/code-423n4/2024-11-mantra-dex/blob/main/contracts/farm-manager/src/farm/commands.rs#L375-L376. On the other hand, a farm is considered expired if any of the following conditions are met, if all of the farm assets are claimed, or if some time (farm_expiration_time) passes after the farm's end epoch. This logic is handled in is_farm_expired: pub(crate) fn is_farm_expired( farm: &Farm, deps: Deps, env: &Env, config: &Config, ) -> Result<bool, ContractError> { let epoch_response: EpochResponse = deps .querier /...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Unclaimable remainder during farm creation gives some farm owners expansion-advantage over others

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Submission:** F-77
- **Submitter:** Tigerfrake
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-dex/submissions/F-77
- **Source snapshot:** competitions/2024-11-mantra-dex/submissions/raw/F-77.txt

## Brief Summary

The current implementation of the farm expiration logic leads to an imbalance in the ability to expand farms. Specifically, when a user contributes an amount that does not evenly divide by the number of epochs provided, they can maintain their farm's active status longer than other users who provide fully claimable assets. Now, notice that once a farm is expired, it can be closed by anyone during new farm creation. This means that users who cannot expand their farms will therefore have to pay new creation fees (at least 1_000 amount) to create new farms if they still want to participate while the advantaged users do not incur this new fee (they bypass it). This is akin to improper value ext...

## Rejection Reason

Final severity/evaluation: Low; Valid Primary Sufficient

# Creating of positions with explicit identifiers can be permanently DOS'd

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Submission:** F-82
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-dex/submissions/F-82
- **Source snapshot:** competitions/2024-11-mantra-dex/submissions/raw/F-82.txt

## Brief Summary

Creating of positions with explicit identifiers can be permanently DOS'd

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# DoS in farm expansion due to redundant validation of lp_asset creator incurs owners new creation fees

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Submission:** F-61
- **Submitter:** Tigerfrake
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-dex/submissions/F-61
- **Source snapshot:** competitions/2024-11-mantra-dex/submissions/raw/F-61.txt

## Brief Summary

When creating and expanding farms, the LP asset (params.lp_denom) is validated to ensure it was created by the current pool_manager_addr. However, a configuration update to the pool_manager_addr (through update_config()) can potentially invalidate the checks during farm expansion, as the pool manager address used at the time of the position creation may differ from the updated address. Now, if a farm owner is blocked from expansion, their farm will be considered expired the moment all rewards have been claimed from it. Once this happens, the farm owner entirely loses expansion ability and therefore if they still want to participate, will have to create a new farm incurring a new (farm creat...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Risk of Unclaimed Rewards Misappropriation on Farm Closure

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Submission:** F-109
- **Submitter:** won
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-dex/submissions/F-109
- **Source snapshot:** competitions/2024-11-mantra-dex/submissions/raw/F-109.txt

## Brief Summary

The close_farms function in the Mantra Dex farm manager contract contains a critical vulnerability in how it handles unclaimed rewards during the farm closure process. Specifically, the function deducts claimed rewards from the total farm assets and transfers the remaining unclaimed rewards directly to the farm owner. This behavior creates a structural flaw with significant security implications: Transfer of Unclaimed Rewards: Upon closure, all rewards that users have not claimed are transferred directly to the farm owner instead of being allocated for users. This leads to permanent loss of user rewards. Exploit Potential: A farm owner can close the farm prematurely and misappropriate user...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Farm Creation Fee Can Be Ineffective in Preventing LP Token Denial of Service

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Submission:** F-35
- **Submitter:** Lambda
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-dex/submissions/F-35
- **Source snapshot:** competitions/2024-11-mantra-dex/submissions/raw/F-35.txt

## Brief Summary

The current farm creation fee is a fixed value. According to the deployment script, it is a flat 10 OM, which is insufficient to prevent malicious actors from performing a denial of service attack on specific LP tokens. Due to the max_concurrent_farms limit of 7 per LP token, an attacker can create farms with minimal emission rates that run for very long periods, effectively blocking legitimate users from creating farms for that LP token. For only 70 OM total (7 farms * 10 OM fee), plus minimal amounts of farming tokens to ensure positive emission rates, an attacker can lock out farming capabilities for specific LP tokens for extended periods (years), causing significant disruption to the p...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# provide_liquidity() farm_manager_addr may be modified to very low rewards, resulting in rewards being far below user expectations.

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Submission:** F-99
- **Submitter:** 0x1982us
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-dex/submissions/F-99
- **Source snapshot:** competitions/2024-11-mantra-dex/submissions/raw/F-99.txt

## Brief Summary

When we provide liquidity, we can lock LP at the same time, to get rewards pub fn provide_liquidity( deps: DepsMut, env: Env, info: MessageInfo, slippage_tolerance: Option<Decimal>, //@info slippage share max_spread: Option<Decimal>, //@info for single side swap receiver: Option<String>, pool_identifier: String, unlocking_duration: Option<u64>, lock_position_identifier: Option<String>, ) -> Result<Response, ContractError> { ... if let Some(unlocking_duration) = unlocking_duration { ... messages.push( wasm_execute( @> config.farm_manager_addr, &amm::farm_manager::ExecuteMsg::ManagePosition { action: amm::farm_manager::PositionAction::Create { identifier: Some(position_identifier), unlocking_...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Inconsistent fee validation logic during pool creation leads to incorrect payment rejections

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Submission:** F-47
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-dex/submissions/F-47
- **Source snapshot:** competitions/2024-11-mantra-dex/submissions/raw/F-47.txt

## Brief Summary

The validate_fees_are_paid() function employs inconsistent methods for validating fee payments based on the length of denom_creation_fee. When the token factory fee has only one option, it uses cw_utils::must_pay() which strictly requires a single payment entry, while other cases use get_paid_pool_fee_amount() which can aggregate multiple payments of the same denomination. This inconsistency can cause valid fee payments to be incorrectly rejected.

## Rejection Reason

Final severity/evaluation: Low; Valid Primary Sufficient

# Funds stuck in FeeCollector contract due to missing withdrawal functionality

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Submission:** F-125
- **Submitter:** Lambda
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-dex/submissions/F-125
- **Source snapshot:** competitions/2024-11-mantra-dex/submissions/raw/F-125.txt

## Brief Summary

The FeeCollector contract is designed to collect various protocol fees including farm creation fees, emergency withdrawal penalties, etc. However, the contract lacks any functionality to withdraw or transfer the collected fees, effectively locking them in the contract forever.

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Operational issues due to inability to manage individual pools

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Submission:** F-72
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-dex/submissions/F-72
- **Source snapshot:** competitions/2024-11-mantra-dex/submissions/raw/F-72.txt

## Brief Summary

The current implementation of the liquidity pool management system utilizes a universal config.feature_toggle that governs the ability to enable or disable swaps, deposits, and withdrawals across all pools. This design choice introduces significant operational complexities, as it lacks the flexibility to manage individual pools based on their specific conditions or requirements. When a single toggle controls all pools, it can lead to unintended operation issues.

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Single-sided liquidity can't be deposited if swaps are disabled

- **Contest:** MANTRA DEX
- **Slug:** 2024-11-mantra-dex
- **Submission:** F-21
- **Submitter:** 0xAlix2
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-mantra-dex/submissions/F-21
- **Source snapshot:** competitions/2024-11-mantra-dex/submissions/raw/F-21.txt

## Brief Summary

When users deposit liquidity into pools, they call provide_liquidity. If they provide more than 1 asset, the process takes the normal scenario, and assets are deposited into the corresponding pool. However, if the user provides 1 asset, half of the deposited amount is swapped to the other asset and then deposited (single-sided liquidity deposit can only be done on pools with 2 assets). The process is as follows: provide_liquidity -> swap -> rely -> provide_liquidity. On the other hand, the protocol can independently pause deposits, withdrawals, and swaps. When calling swap the protocol checks if the swaps are paused before swapping assets without checking if the sender is the contract itsel...

## Rejection Reason

Final severity/evaluation: Low; Valid Primary Sufficient
