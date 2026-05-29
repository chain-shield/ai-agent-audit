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
