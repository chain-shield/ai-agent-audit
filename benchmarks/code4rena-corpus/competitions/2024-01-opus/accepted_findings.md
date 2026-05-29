# Accepted H/M Findings: Opus 

# [H-01] Neglect of exceptional redistribution amounts in withdraw_helper function

- **Contest:** Opus 
- **Slug:** 2024-01-opus
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-01-opus
- **Source snapshot:** competitions/2024-01-opus/final_report.html

withdraw_helper function Submitted by Aymen0909, also found by etherhood, bin2chen, jasonxiale, minhquanym, and kodyvim Lines of code

- https://github.com/code-423n4/2024-01-opus/blob/main/src/core/shrine.cairo#L1382-L1392
- https://github.com/code-423n4/2024-01-opus/blob/main/src/core/shrine.cairo#L1421-L1431
Description The withdraw_helper function in the shrine contract handles withdrawal logic for both the withdraw and seize functions. It is responsible for updating trove balances, total yang balances, and charging interest for the trove via the charge function. However, there is an oversight in the current implementation:

fn withdraw_helper(ref self: ContractState, yang: ContractAddress, trove_id: u64, amount: Wad) {...

let new_trove_balance: Wad = trove_balance - amount; let new_total: Wad = self.yang_total.read(yang_id) - amount; self.charge(trove_id); //@audit will not account for exceptional redistribution added to deposits balance in `charge` call self.yang_total.write(yang_id, new_total); self.deposits.write((yang_id, trove_id), new_trove_balance); // Emit events self.emit(YangTotalUpdated { yang, total: new_total }); self.emit(DepositUpdated { yang, trove_id, amount: new_trove_balance }); } The issue in the code above is that the withdraw_helper function proceeds to update the storage variables yang_total and deposits using the previously calculated new_total and new_trove_balance values, without accounting for any new yang balance added to the trove after an exceptional redistribution. This results in neglecting any exceptional redistributions added to the

deposits balance during the charge call:

fn charge(ref self: ContractState, trove_id: u64) {...

// If there was any exceptional redistribution, write updated yang amounts to trove if updated_trove_yang_balances.is_some() { let mut updated_trove_yang_balances = updated_trove_yang_balances.unwrap(); loop { match updated_trove_yang_balances.pop_front() { Option::Some(yang_balance) => { //@audit will updated the trove yang balance self.deposits.write((*yang_balance.yang_id, trove_id), *yang_balance.amount); }, Option::None => { break; }, }; }...

} Because the trove deposits map is changed in the charge function but withdraw_helper uses directly the value new_trove_balance, which was calculated before the charge call, the exceptional redistribution added to deposits will be overridden and will be neglected in the trove yang balance.

This oversight could result in financial losses for all protocol users. When users withdraw yang amounts, any exceptional redistributions that should have been added to their trove balances will be neglected and lost.

## Impact

Users are at risk of losing all yang exceptional redistribution amounts due to an error in the withdraw_helper function, which causes it to neglect any yang-added redistribution to the trove deposits map.

## Recommended Mitigation

To address this issue, the charge function should be called before calculating the new trove yang balance ( new_trove_balance ). This ensures that any exceptional redistributions are accounted for before updating the trove balance and total yang balance:

fn withdraw_helper(ref self: ContractState, yang: ContractAddress, trove_id: u64, amount: Wad) { let yang_id: u32 = self.get_valid_yang_id(yang); //@audit add exceptional redistribution before calculating `new_trove_balance` ++ self.charge(trove_id); // Fails if amount > amount of yang deposited in the given trove let trove_balance: Wad = self.deposits.read((yang_id, trove_id)); assert(trove_balance >= amount, 'SH: Insufficient yang balance'); let new_trove_balance: Wad = trove_balance - amount; let new_total: Wad = self.yang_total.read(yang_id) - amount; -- self.charge(trove_id); self.yang_total.write(yang_id, new_total); self.deposits.write((yang_id, trove_id), new_trove_balance); // Emit events

self.emit(YangTotalUpdated { yang, total: new_total }); self.emit(DepositUpdated { yang, trove_id, amount: new_trove_balance }); }

## Assessed type

Context tserg (Opus) confirmed and commented via duplicate issue #211:

This is valid - potentially fixed.

0xsomeone (judge) commented:

The warden has demonstrated how an exception trove redistribution will not be properly tracked by the withdrawal helper, resulting in an unsynchronized accounting state for the Opus system whereby the user will lose the collateral they acquired in the redistribution.

I believe a high-risk severity is appropriate as it details a scenario in which the collateral balances of users will potentially lose the full redistributed collateral.

# [H-02] convert_to_yang_helper() loss precision

- **Contest:** Opus 
- **Slug:** 2024-01-opus
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-01-opus
- **Source snapshot:** competitions/2024-01-opus/final_report.html

convert_to_yang_helper() loss precision Submitted by bin2chen In gate.cairo, when the user calls deposit(), it calculates the corresponding shares through convert_to_yang_helper(). The code is as follows:

fn convert_to_yang_helper(self: @ContractState, asset_amt: u128) -> Wad { let asset: IERC20Dispatcher = self.asset.read(); let total_yang: Wad = self.get_total_yang_helper(asset.contract_address); if total_yang.is_zero() { let decimals: u8 = asset.decimals(); // Otherwise, scale `asset_amt` up by the difference to match `Wad` // precision of yang. If asset is of `Wad` precision, then the same // value is returned fixed_point_to_wad(asset_amt, decimals) } else { @> (asset_amt.into() * total_yang) / get_total_assets_helper(asset).into() } The calculation formula is:

(asset_amt.into() * total_yang) / get_total_assets_helper(asset).into().

The actual calculation of converting Wad to pure numbers is:

(asset_amt * total_yang / 1e18) * 1e18 / total_assets.

The above formula (asset_amt * total_yang / 1e18) will lose precision, especially when the asset’s decimals are less than 18.

Assume btc as an example, decimals = 8 after add_yang(btc) INITIAL_DEPOSIT_AMT = 1000 so:

total_assets = 1000 total_yang = 1000e10 = 1e13 If the user deposits 0.0009e8 BTC, according to the formula = (asset_amt * total_yang / 1e18):

= 0.0009e8 * 1e13 /1e18 = 0.9e5 * 1e13 /1e18 = 0 With BTC’s price at 40,000 USD, 0.0009e8 = 36 USD. The user will lose 36 USD.

We should cancel dividing by 1e18 and then multiplying by 1e18, and calculate directly: shares = asset_amt.into() * total_yang.into() / total_assets.into().

shares = 0.0009e8 * 1e13 / 1000 = 0.0009e18 = 900000000000000 Note: In order to successfully deposit should be > 0.0009e8 such as 0.0019e8, which is simplified and convenient to explain.

## Impact

Due to the premature division by 1e18, precision is lost, and the user loses a portion of their funds.

## Recommended Mitigation

fn convert_to_yang_helper(self: @ContractState, asset_amt: u128) -> Wad { let asset: IERC20Dispatcher = self.asset.read(); let total_yang: Wad = self.get_total_yang_helper(asset.contract_address); if total_yang.is_zero() { let decimals: u8 = asset.decimals(); // Otherwise, scale `asset_amt` up by the difference to match `Wad` // precision of yang. If asset is of `Wad` precision, then the same // value is returned fixed_point_to_wad(asset_amt, decimals) } else { - (asset_amt.into() * total_yang) / get_total_assets_helper(asset).into() + let result:u256 = asset_amt.into() * total_yang.into() / total_assets.into(); + Wad { val:result.try_into().expect('u128')}; }

## Assessed type

Decimal tserg (Opus) confirmed 0xsomeone (judge) commented:

The warden has demonstrated how the “hidden” operations of multiplication and division that are performed as part of the overloaded Wad data type primitive operators can result in loss of precision for assets with less than 18 decimals; which are explicitly meant to be supported by the Opus system per the onboarding guidelines.

I consider a high-risk rating appropriate given that the truncation will be greater the lower the decimals of the token and the higher the value per unit of the token is.

# [H-03] A user can steal from the shrine by forcing redistribution of their trove; due to incorrect logic trove debt will be reset but yangs kept

- **Contest:** Opus 
- **Slug:** 2024-01-opus
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-01-opus
- **Source snapshot:** competitions/2024-01-opus/final_report.html

Submitted by kfx, also found by bin2chen, minhquanym, and TrungOre Let’s assume two yangs in the system, yang A and yang B, and two users:

User U1 with trove #1 with zero A units, 1000 B units, and 500 yin debt; User U2 with trove #2 10000 A unit, 1000 B units, and 500 yin debt.

If the user U1 can force redistribution of their position, then they can steal from the shrine due to a bug in the code. The function redistribute_helper loops through all yangs in order, including those not in the trove #1. Since trove_yang_amt.is_zero() returns true for yang A, the updated_trove_yang_balances array is updated early and then continue statement is executed.

However, since the new_yang_totals array is not updated in the iteration of the loop, some values of updated_trove_yang_balances end up never being used.

Let’s assume 100% redistribution. After the all loop is fully executed, the two arrays contain:

updated_trove_yang_balances = [(A, 0), (B, 0)]; new_yang_totals = [(B, 1000)]; The final loop of the function is executed just once. Its first and only iteration writes the new total B value. However, it does not update the amount of B in the trove #1, since (B, 0) is the second element of the first array. The final state is that trove #1 still has 1000 units of B, but no more debt. The user U1 can now withdraw all 1000 units from the trove #1.

This bug violates the shrine invariant “The total amount of a yang is equal to the sum of all troves’ deposits of that yang (this includes any exceptionally redistributed yangs and their accompanying errors) and the initial amount seeded at the time of add_yang.”

## Recommended Mitigation Steps

Do not update the array updated_trove_yang_balances before the continue statement.

## Assessed type

Loop tserg (Opus) confirmed and commented via duplicate issue #199:

This is valid - potentially fixed.

0xsomeone (judge) commented:

The warden has demonstrated how a debt redistribution will maintain incorrect entries in the updated Yang balances and total Yang balances when skipping over one, weaponizing this behavior to acquire collateral from a shrine.

I believe a high-risk evaluation is apt as collateral of other users is directly impacted.

# [H-04] Shrine’s recovery mode can be weaponized as leverage to liquidate healthy troves

- **Contest:** Opus 
- **Slug:** 2024-01-opus
- **Finding ID:** H-04
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-01-opus
- **Source snapshot:** competitions/2024-01-opus/final_report.html

Submitted by 3docSec, also found by etherhood, nmirchev8, and kfx Lines of code

- https://github.com/code-423n4/2024-01-opus/blob/4720e9481a4fb20f4ab4140f9cc391a23ede3817/src/core/shrine.cairo#L1046
Description In the Shrine implementation, the loan (trove) health is calculated by having its LTV compared to the shrine threshold:

File: shrine.cairo 1133: fn is_healthy_helper(self: @ContractState, health: Health) -> bool { 1134: health.ltv <= health.threshold 1135: } --- 1140: fn assert_valid_trove_action(self: @ContractState, trove_id: u64) { 1141: let health: Health = self.get_trove_health(trove_id); 1142: assert(self.is_healthy_helper(health), 'SH: Trove LTV is too high'); The shrine threshold is in turn calculated from the weighted thresholds of the yang deposits, scaled down by a variable factor, in case the shrine is in recovery mode:

File: shrine.cairo 1040: fn get_trove_health(self: @ContractState, trove_id: u64) -> Health { --- 1045: let (mut threshold, mut value) = self.get_threshold_and_value(trove_yang_balances, interval); 1046: threshold = self.scale_threshold_for_recovery_mode(threshold); --- 1202: fn scale_threshold_for_recovery_mode(self: @ContractState, mut threshold: Ray) -> Ray { 1203: let shrine_health: Health = self.get_shrine_health(); 1204:

1205: if self.is_recovery_mode_helper(shrine_health) { 1206: let recovery_mode_threshold: Ray = shrine_health.threshold * RECOVERY_MODE_THRESHOLD_MULTIPLIER.into(); 1207: return max( 1208: threshold * THRESHOLD_DECREASE_FACTOR.into() * (recovery_mode_threshold / shrine_health.ltv), 1209: (threshold.val / 2_u128).into() 1210: ); 1211: } 1212:

1213: threshold 1214: } We can see from the above code that triggering recovery mode lowers the threshold, exposing the more under-collateralized loans (troves) to liquidation. This is expected behavior when the LTV fluctuations are coming from collateral price swings.

If we look at how recovery mode is triggered:

File: shrine.cairo 0079: const RECOVERY_MODE_THRESHOLD_MULTIPLIER: u128 = 700000000000000000000000000; // 0.7 (ray) --- 1165: fn is_recovery_mode_helper(self: @ContractState, health: Health) -> bool { 1166: let recovery_mode_threshold: Ray = health.threshold * RECOVERY_MODE_THRESHOLD_MULTIPLIER.into(); 1167: health.ltv >= recovery_mode_threshold 1168: } We can see that all it takes to trigger recovery mode is to bring the shrine LTV to 70% of its nominal threshold, or higher. This can be achieved by a malicious (or naive) user, provided they have enough collateral to take large borrows close to the collateralization threshold, and the shrine debt_ceiling provides enough headroom.

## Impact

Loans can be forced into liquidation territory, and be liquidated, whenever a new loan is opened large enough to trigger recovery mode. This can also happen as a deliberate attack, and within a single transaction, without exposing the attacker’s funds to liquidation. It is consequently a solid candidate for a flash loan attack, but can also be executed with a large amount of pre-deposited collateral.

## Recommended Mitigation Steps

It is not entirely clear how the recovery mechanism, intended as is, can be modified to fix this issue. Introducing a form of limitation to liquidations happening in the same block of a recovery trigger can mitigate exposure to flash-loans, but large loans against pre-owned collateral left dormant on the shrine would still be a viable attack path.

What we can tell, however, is that the recovery mechanism appears to have the intent of increasing the difficulty of opening new loans as the shrine health approaches the liquidation threshold.

Popular DeFi protocols like Compound solved this very issue by having two different LTV references: one for accepting liquidations and one lower for accepting new loans.

More in detail, the protocol is vulnerable only because one can borrow at LTV values above the recovery threshold (70% of the nominal threshold) but still below the liquidation threshold. Therefore, is able to raise the global LTV above that recovery threshold. If users were not allowed to borrow above that 70%, they wouldn’t be able to raise the global LTV above it, even with infinite collateral.

## Assessed type

MEV tserg (Opus) confirmed and commented via duplicate issue #205:

This is valid - potentially fixed.

0xsomeone (judge) commented:

The warden has demonstrated how the automatic recovery mode mechanism of the Opus system can be exploited to force the system into recovery mode, enabling the liquidation of previously healthy troves.

A high-risk vulnerability rating for this issue is valid as the automatic recovery mode can be exploited within a single transaction to force the system into recovery mode by opening a bad position, liquidating whichever troves are lucrative, and closing the previously bad position with zero risk.

Medium Risk Findings (9)

# [M-01] Loss of liquidation compensation assets in absorb

- **Contest:** Opus 
- **Slug:** 2024-01-opus
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-opus
- **Source snapshot:** competitions/2024-01-opus/final_report.html

Submitted by etherhood In absorb, since shrine.melt is being called after free, caller will end up receiving less compensation than actually intended. It is because in free, while checking user deposits, there are two cases when user has pending redistribution with exception: 1: User’s yang 1 has zero balance 2: User’s yang 1 has non zero balance In both cases user’s balances are due for an update, if first n yang balances are zero, caller will miss out on compensation from unaccounted yang updates from all yangs due for redistribution until it encounters one with non zero yang balance. When it encounters first non-zero yang balance, it will still miss out on unaccounted fund from that yang as well, but after that it will be okay, since

charge() would be called inside seize function which will then update balances for all yangs.

## Recommended Mitigation Steps

The absorb function should update order of melt and free:

shrine.melt(absorber.contract_address, trove_id, purge_amt); let compensation_assets: Span<AssetBalance> = self.free(shrine, trove_id, pct_value_to_compensate, caller); tserg (Opus) commented:

This is valid and mitigated.

tserg (Opus) confirmed, but disagreed with severity and commented:

Caller still gets compensation, plus it is capped to 50 USD, so the impact of this is capped to 50 USD. User funds are also not at risk.

0xsomeone (judge) decreased severity to Medium and commented:

The warden has showcased a way in which the compensation for the caller of the absorption will be lower than intended.

While the issue is valid, I agree with the sponsor in that the severity of this cannot be considered high-risk. User funds are indeed affected, however, so I believe a medium-risk rating is more apt.

# [M-02] after shut, no pulled redistribution yang will be locked

- **Contest:** Opus 
- **Slug:** 2024-01-opus
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-opus
- **Source snapshot:** competitions/2024-01-opus/final_report.html

Submitted by bin2chen, also found by mahdikarimi and etherhood In caretaker.release(), we can release the remaining yang:

fn release (ref self: ContractState, trove_id:

u64 ) -> Span<AssetBalance> { let shrine: IShrineDispatcher = self.shrine.

read ();...

loop { match yangs_copy.

pop_front () { Option::Some(yang) => { @> let deposited_yang: Wad = shrine.

get_deposit (*yang, trove_id); let asset_amt:

u128 = if deposited_yang.

is_zero () { 0 } else { let exit_amt:

u128 = sentinel.

exit (*yang, trove_owner, trove_id, deposited_yang); // Seize the collateral only after assets have been // transferred so that the asset amount per yang in Gate // does not change and user receives the correct amount shrine.

seize (*yang, trove_id, deposited_yang); exit_amt }; released_assets.

append (AssetBalance { address: *yang, amount: asset_amt }); }, Option::None => { break; }, }; As above, we can only release the yang that already exists in the trove:

shrine.get_deposit(*yang, trove_id);.

When shire.get_live() == false, we can no longer perform shire.pull_redistributed_debt_and_yangs(). So if there is any yang that hasn’t been pulled, it can’t be retrieved through caretaker.release(). This part of the yang will be locked in the contract.

## Impact

After shut(), the redistributed yang that hasn’t been pulled will be locked in the contract.

## Recommended Mitigation

Add pull_when_shut():

fn pull_when_shut (ref self: ContractState, trove_id:

u64 ) { if self.is_live.

read () { return; } let trove: Trove = self.troves.

read (trove_id); let current_interval:

u64 = now (); let trove_yang_balances: Span<YangBalance> = self.

get_trove_deposits (trove_id); let (updated_trove_yang_balances, _) = self.

pull_redistributed_debt_and_yangs (trove_id, trove_yang_balances,Wad { val:

0 }); // If there was any exceptional redistribution, write updated yang amounts to trove if updated_trove_yang_balances.

is_some () { let mut updated_trove_yang_balances = updated_trove_yang_balances.

unwrap (); loop { match updated_trove_yang_balances.

pop_front () { Option::Some(yang_balance) => { self.deposits.

write ((*yang_balance.yang_id, trove_id), *yang_balance.amount); }, Option::None => { break; }, }; } self.trove_redistribution_id.

write (trove_id, self.redistributions_count.

read ()); } fn release(ref self: ContractState, trove_id: u64) -> Span<AssetBalance> { let shrine: IShrineDispatcher = self.shrine.read();...

+ shrine.pull_when_shut(trove_id); loop { match yangs_copy.pop_front() { Option::Some(yang) => { let deposited_yang: Wad = shrine.get_deposit(*yang, trove_id);

## Assessed type

Error tserg (Opus) confirmed 0xsomeone (judge) commented:

The warden has demonstrated how the graceful shutdown of a shrine can result in loss of funds if an exceptional trove redistribution occurred, resulting in loss of the redistributed collateral.

I believe a medium-risk grade is apt for this exhibit as it relates to a low-likelihood scenario and a loss that is contained.

# [M-03] ERC4626 inflate issue mitigation is not sufficient

- **Contest:** Opus 
- **Slug:** 2024-01-opus
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-opus
- **Source snapshot:** competitions/2024-01-opus/final_report.html

Submitted by jasonxiale, also found by bin2chen Lines of code

- https://github.com/code-423n4/2024-01-opus/blob/4720e9481a4fb20f4ab4140f9cc391a23ede3817/src/core/gate.cairo#L191-L223
- https://github.com/code-423n4/2024-01-opus/blob/4720e9481a4fb20f4ab4140f9cc391a23ede3817/src/core/absorber.cairo#L667-L705

## Impact

Both absorber and gate use the same mitigation for ERC4626 first depositor front-running vulnerability, but current implementation is not sufficient. By abusing the flaw, even though malicious attacker can’t benefit from the mitigation, he can cause other normal users to lose assets.

## Recommended Mitigation Steps

In gate.get_total_assets_helper, don’t use balance_of to calculate the amount; instead, define a new variables and record the deposited asset amount by the variables

## Assessed type

ERC4626 tserg (Opus) confirmed via duplicate issue #196 0xsomeone (judge) commented:

This submission details the vulnerability with a greater impact than #196, and has thus been selected as primary.

# [M-04] The provide() function does not reset withdrawal requests, allowing an attacker to bypass risk-free yield tactics protection

- **Contest:** Opus 
- **Slug:** 2024-01-opus
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-opus
- **Source snapshot:** competitions/2024-01-opus/final_report.html

provide() function does not reset withdrawal requests, allowing an attacker to bypass risk-free yield tactics protection Submitted by minhquanym In the Opus protocol, the absorber is a stability pool that permits yin holders to contribute their yin and participate in liquidations (also known as absorptions) as a consolidated pool. Provided yin from users could be absorbed during an absorption. In return, users receive yang, which usually has a higher value than the absorbed yin. However, in the event of bad debt, it could be less. This situation could lead to risk-free yield frontrunning tactics, where an attacker only provides yin when the absorption brings profit then removing all yin right after that.

The Opus team is aware of this attack vector and has, therefore, implemented a request-withdraw procedure. Users must first request a withdrawal and wait for a certain period before executing the withdrawal.

However, the provide() method does not reset these request timestamps, which allows an attacker to bypass this safeguard.

## Recommended Mitigation Steps

Reset the withdrawal request in the provide() function.

## Assessed type

MEV tserg (Opus) confirmed, but disagreed with severity and commented:

The economic feasibility of the exploit is questionable, and it requires extensive infrastructure to be set up and maintained. Ultimately, no user funds are at risk.

0xsomeone (judge) decreased severity to Medium and commented:

The warden has demonstrated how a malicious user can game the queued withdrawal system of Opus and cycle withdrawal authorizations between multiple accounts to always retain one that can immediately withdraw at any given point in time. As a provision will not reset those requests, the account whose withdrawal authorization is valid at a point a lucrative absorption occurs can be exploited to acquire a bigger share of the collateral.

I believe a medium-risk grade is better suited for this finding as only the “reward” distribution is manipulated and it requires extensive money-translated effort (i.e. gas for maintaining positions active) that will further reduce the economic viability of this exploit.

Note: For full discussion, see here.

# [M-05] An attacker could manipulate debt exceptional redistribution because it is allowed to deposit into any trove

- **Contest:** Opus 
- **Slug:** 2024-01-opus
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-opus
- **Source snapshot:** competitions/2024-01-opus/final_report.html

Submitted by minhquanym In a redistribution, the unhealthy trove’s collateral and debt is distributed among troves proportionally to its collateral composition. If no other troves have deposited a yang that is to be redistributed, then the debt and value attributed to that yang will be redistributed among all other yangs in the system, according to their value proportionally to the total value of all remaining yangs in the system.

However, the Opus protocol allows anyone to deposit into any trove, even if they are not the trove’s owner, as seen in the function abbot.deposit(). This feature could potentially be exploited by an attacker. For instance, if a yang is only deposited in one trove that’s being redistributed, the attacker could deposit a small amount into a victim’s trove. This could result in all bad debt being redistributed to the victim’s trove instead of exceptional redistribution, even if the victim didn’t want this (i.e., they didn’t deposit this yang into their trove).

fn deposit (ref self: ContractState, trove_id:

u64, yang_asset: AssetBalance) { // There is no need to check the yang address is non-zero because the // Sentinel does not allow a zero address yang to be added.

assert (trove_id != 0, 'ABB: Trove ID cannot be 0 '); assert (trove_id <= self.troves_count.

read (), 'ABB: Non-existent trove'); // note that caller does not need to be the trove's owner to deposit // @audit Attacker could deposit for any trove to make it the only trove deposited this yang and being redistributed when bad debt happen self.

deposit_helper (trove_id, get_caller_address (), yang_asset); }

## Recommended Mitigation Steps

Limit deposits to only the trove’s owner.

tserg (Opus) acknowledged, but disagreed with severity and commented:

The likelihood of this happening is low.

0xsomeone (judge) decreased severity to Medium and commented:

The warden has demonstrated how bad debt re-allocation can be “gamed” due to the permissionless deposit system of troves.

This particular attack vector is quite interesting and I commend the warden for their out-of-the-box thinking. However, I believe a medium-risk grade is better suited, given that the likelihood of a liquidation of a trove with an asset that is not held by any other trove in the system is low.

# [M-06] Multiplier is incorrectly calculated in Controller

- **Contest:** Opus 
- **Slug:** 2024-01-opus
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-opus
- **Source snapshot:** competitions/2024-01-opus/final_report.html

Controller Submitted by ABAIKUNANBAEV In Controller smart contract, due to mistake in multiplication of i_gain, it’s miscalculated and the error occurs. This can lead to a multiplier reflecting an incorrect value and therefore affect yin borrowing rate.

## Recommended Mitigation Steps

- https://github.com/code-423n4/2024-01-opus/blob/main/src/core/controller.cairo#L256-258
- old_i_term + nonlinear_transform(self.get_prev_error(), - self.alpha_i.read(), self.beta_i.read()) * - time_since_last_update_scaled + old_i_term + i_gain * nonlinear_transform(self.get_prev_error(), + self.alpha_i.read(), self.beta_i.read()) * + time_since_last_update_scaled

- https://github.com/code-423n4/2024-01-opus/blob/main/src/core/controller.cairo#L167
- multiplier += i_gain * new_i_term; + multiplier += new_i_term; 0xsomeone (judge) commented:

The warden has demonstrated how the formula that is in use by the Controller contradicts the documentation of the project, and namely the specification of the aforementioned formula.

In detail, the specification states:

Note: Please see provided formula in the judge’s original comment.

We are interested in the latter part of the formula, specifically:

Note: Please see provided formula in the judge’s original comment.

The documentation states that k_i stands for the:

gain that is applied to the integral term The problem highlighted by the warden is that the implementation will calculate the following:

Note: Please see provided formula in the judge’s original comment.

The above corresponds to:

let new_i_term: SignedRay = self.get_i_term_internal(); multiplier += i_gain * new_i_term; Whereby the get_i_term_internal function will ultimately yield:

old_i_term + nonlinear_transform(self.get_prev_error(), self.alpha_i.read(), self.beta_i.read()) * time_since_last_update_scaled In the above:

Note: please review provided terms in the judge’s original comment.

Based on the above analysis, the documentation indeed contradicts what the code calculates. The Opus team is invited to evaluate this exhibit; however, regardless of the correct behaviour of the system, this submission will be accepted as valid due to the documentation being the “source of truth” during the audit’s duration.

milancermak (Opus) confirmed

# [M-07] Collateral cannot be withdrawn from trove once yang is suspended

- **Contest:** Opus 
- **Slug:** 2024-01-opus
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-opus
- **Source snapshot:** competitions/2024-01-opus/final_report.html

Submitted by 0xTheC0der Once a yang (collateral asset) is suspended on Opus, the following holds true according to the documentation:

No further deposits can be made. This is enforced by the Sentinel.

Its threshold will decrease to zero linearly over the SUSPENSION_GRACE_PERIOD.

Therefore, a user will naturally deposit healthier collateral to their trove to maintain its threshold, while withdrawing the unhealthy/suspended collateral as it loses value on Opus, effectively replacing the collateral.

However, this is not possible because the underlying assets of a yang are immediately frozen once suspended. It’s clear from the documentation that no more deposits of suspended collateral can be made, but this accidentally also affects the withdrawals.

The abbot::withdraw (…) method calls sentinel::convert_to_yang (…) which in turn calls sentinel::assert_can_enter (…):

fn assert_can_enter(self: @ContractState, yang: ContractAddress, gate: IGateDispatcher, enter_amt: u128) {...

let suspension_status: YangSuspensionStatus = self.shrine.read().get_yang_suspension_status(yang); assert(suspension_status == YangSuspensionStatus::None, 'SE: Yang suspended');...

} One can see that the assertion will be triggered once the suspension status is Temporary or Permanent.

As a consequence, a yang’s underlying assets are frozen once suspended:

If the suspension status is still Temporary, the admin can unsuspend the yang to make it withdrawable again. However, this also stops its threshold decrease and makes it depositable again, which eliminates the incentives to withdraw in the first place. This essentially breaks the suspension mechanism.

If the suspension status reaches Permanent, the assets are permanently locked from withdrawal. Nevertheless, there is a workaround by closing the trove, which requires all the yin (debt) to be repaid and unnecessarily withdraws all other yangs (collateral assets) of the trove too. Therefore, this is not a viable solution for the present issue.

## Recommended Mitigation Steps

Replace the accidental assert_can_enter(..) check with those that are really necessary at this point:

diff --git a/src/core/sentinel.cairo b/src/core/sentinel.cairo index b18edde..9671ca2 100644 --- a/src/core/sentinel.cairo +++ b/src/core/sentinel.cairo @@ -156,7 +156,8 @@ mod sentinel { // This can be used to simulate the effects of `enter`.

fn convert_to_yang(self: @ContractState, yang: ContractAddress, asset_amt: u128) -> Wad { let gate: IGateDispatcher = self.yang_to_gate.read(yang); - self.assert_can_enter(yang, gate, asset_amt); + assert(gate.contract_address.is_non_zero(), 'SE: Yang not added'); // alike to sentinel::convert_to_assets(...) + assert(self.yang_is_live.read(yang), 'SE: Gate is not live'); // to satisfy test_sentinel::test_kill_gate_and_preview_enter() gate.convert_to_yang(asset_amt) }

## Assessed type

Invalid Validation tserg (Opus) confirmed 0xsomeone (judge) commented:

The warden has demonstrated how a Yang’s suspension (either temporary or permanent) will cause the overall system to deviate from its specification and disallow withdrawals of the Yang (collateral) instead of permitting them, thereby never letting the system gradually recover.

I consider a medium-risk severity to be appropriate for this exhibit as it relates to misbehavior that will arise during an emergency scenario.

# [M-08] Attacker can lock every trove withdrawals

- **Contest:** Opus 
- **Slug:** 2024-01-opus
- **Finding ID:** M-08
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-opus
- **Source snapshot:** competitions/2024-01-opus/final_report.html

Submitted by zigtur Lines of code

- https://github.com/code-423n4/2024-01-opus/blob/main/src/core/abbot.cairo#L210
- https://github.com/code-423n4/2024-01-opus/blob/main/src/core/sentinel.cairo#L159
- https://github.com/code-423n4/2024-01-opus/blob/main/src/core/sentinel.cairo#L288
Description During withdraw, the convert_to_yang function of sentinel is called. This function converts an asset amount to a Yang amount.

convert_to_yang then calls assert_can_enter to ensure that current_total + enter_amt <= max_amt. This check is incorrect in the case of a withdrawal, as a subtraction should be made instead of an addition.

An attacker can deposit an asset amount such that current_total == max_amt. In such condition, withdrawing assets will be impossible because of the incorrect check.

## Impact

In every Gate for which a maximum amount of asset is set in Sentinel, an attacker can lock asset withdrawals for every users.

## Recommended Mitigation Steps

During withdrawals, a withdraw_to_yang() function could be used instead of convert_to_yang(). This new function would not call assert_can_enter.

The following patch fixes this issue by implementing a withdraw_to_yang function, which calculates the amount of Yang without reverting:

diff --git a/src/core/abbot.cairo b/src/core/abbot.cairo index 1f0a589..1ededa4 100644 --- a/src/core/abbot.cairo +++ b/src/core/abbot.cairo @@ -207,7 +207,7 @@ mod abbot { let user = get_caller_address(); self.assert_trove_owner(user, trove_id); - let yang_amt: Wad = self.sentinel.read().convert_to_yang(yang_asset.address, yang_asset.amount); + let yang_amt: Wad = self.sentinel.read().withdraw_to_yang(yang_asset.address, yang_asset.amount); self.withdraw_helper(trove_id, user, yang_asset.address, yang_amt); } diff --git a/src/core/sentinel.cairo b/src/core/sentinel.cairo index b18edde..644bb85 100644 --- a/src/core/sentinel.cairo +++ b/src/core/sentinel.cairo @@ -160,6 +160,15 @@ mod sentinel {

gate.convert_to_yang(asset_amt) } + fn withdraw_to_yang(self: @ContractState, yang: ContractAddress, asset_amt: u128) -> Wad { + let gate: IGateDispatcher = self.yang_to_gate.read(yang); + assert(gate.contract_address.is_non_zero(), 'SE: Yang not added'); + assert(self.yang_is_live.read(yang), 'SE: Gate is not live'); + let suspension_status: YangSuspensionStatus = self.shrine.read().get_yang_suspension_status(yang); + assert(suspension_status == YangSuspensionStatus::None, 'SE: Yang suspended'); + gate.convert_to_yang(asset_amt) + } + // This can be used to simulate the effects of `exit`.

fn convert_to_assets(self: @ContractState, yang: ContractAddress, yang_amt: Wad) -> u128 { let gate: IGateDispatcher = self.yang_to_gate.read(yang); diff --git a/src/interfaces/ISentinel.cairo b/src/interfaces/ISentinel.cairo index 4149a38..04d0d04 100644 --- a/src/interfaces/ISentinel.cairo +++ b/src/interfaces/ISentinel.cairo @@ -33,5 +33,6 @@ trait ISentinel<TContractState> { fn unsuspend_yang(ref self: TContractState, yang: ContractAddress); // view fn convert_to_yang(self: @TContractState, yang: ContractAddress, asset_amt: u128) -> Wad; + fn withdraw_to_yang(self: @TContractState, yang: ContractAddress, asset_amt: u128) -> Wad; fn convert_to_assets(self: @TContractState, yang: ContractAddress, yang_amt: Wad) -> u128;

} Note: Unit tests are all passing with the fix. To apply the patch, import the content in a fix.patch file, then execute git apply fix.patch.

## Assessed type

DoS 0xsomeone (judge) decreased severity to Medium and commented:

The warden has demonstrated how the withdrawal flow in the Abbot contract an incorrect conversion will occur that will enforce a deposit limitation assuming the withdrawn amount is newly deposited.

This behavior will result in the withdrawal not going through and its equality case can be exploited to force all positions of the Yang to not be withdrawable until the limit is updated. I consider a medium-risk better suited for this submission as:

The funds will not be permanently lost and a reconfiguration can recover them.

The attacker would have sacrificed their funds (as they wouldn’t be able to withdraw either), and those funds would be substantial.

It is, however, a flaw that needs to be remediated by the Opus team.

milancermak (Opus) confirmed and commented:

We mitigated this issue based on this report. Thanks!

# [M-09] Unhealthy troves with LTV > 90% cannot always be absorbed as intended

- **Contest:** Opus 
- **Slug:** 2024-01-opus
- **Finding ID:** M-09
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-opus
- **Source snapshot:** competitions/2024-01-opus/final_report.html

Submitted by 0xTheC0der Unhealthy troves with ltv > 90% and threshold < 90% cannot always be absorbed due to a wrong if-condition. According to Priority of liquidation methods it should always be possible to absorb unhealthy troves with ltv > 90%:

Absorption can happen only after an unhealthy trove’s LTV has exceeded the LTV at which the maximum possible penalty is reached, or if it has exceeded 90% LTV. The liquidation penalty in this case will similarly be capped to the maximum of 12.5% or the maximum possible penalty.

However, the purger::get_absorption_penalty_internal (…) method mistakenly checks the threshold instead of the ltv against the ABSORPTION_THRESHOLD (90%) in L467:

fn get_absorption_penalty_internal( self: @ContractState, threshold: Ray, ltv: Ray, ltv_after_compensation: Ray ) -> Option<Ray> { if ltv <= threshold { return Option::None; }...

let mut max_possible_penalty: Ray = min( (RAY_ONE.into() - ltv_after_compensation) / ltv_after_compensation, MAX_PENALTY.into() ); if threshold > ABSORPTION_THRESHOLD.into() { // @audit ltv instead let s = self.penalty_scalar.read(); let penalty = min(MIN_PENALTY.into() + s * ltv / threshold - RAY_ONE.into(), max_possible_penalty); return Option::Some(penalty); } let penalty = min(MIN_PENALTY.into() + ltv / threshold - RAY_ONE.into(), max_possible_penalty); if penalty == max_possible_penalty { Option::Some(penalty) } else { Option::None } As a consequence, unhealthy troves can only be absorbed if they reach the maximum possible penalty although the condition ltv > 90% is already satisfied. This is against the protocol’s intended liquidation/absorption incentives and therefore, endangers the solvency of the protocol.

By observing the sponsor’s graph for liquidation penalty it becomes evident that the MAX_PENALTY can only be achieved for ltv up to 89%. For even higher ltv up to 100%, the penalty approaches 0% due to max_possible_penalty (see code above), which lowers the incentives for liquidation and makes absorption a necessity.

In case of threshold > 83% there is a window where 90% < ltv < ltv@max_possible_penalty causing absorptions to be impossible due to the present bug. This linked graph visualizes the present issue.

## Recommended Mitigation Steps

Make sure the absorption threshold is checked against the ltv as intended:

diff --git a/src/core/purger.cairo b/src/core/purger.cairo index 6a36bbc..820aff5 100644 --- a/src/core/purger.cairo +++ b/src/core/purger.cairo @@ -464,7 +464,7 @@ mod purger { (RAY_ONE.into() - ltv_after_compensation) / ltv_after_compensation, MAX_PENALTY.into() ); - if threshold > ABSORPTION_THRESHOLD.into() { + if ltv > ABSORPTION_THRESHOLD.into() { let s = self.penalty_scalar.read(); let penalty = min(MIN_PENALTY.into() + s * ltv / threshold - RAY_ONE.into(), max_possible_penalty);

## Assessed type

Math 0xsomeone (judge) commented:

The warden has demonstrated how a contradiction between the documentation and the implementation of the project will cause certain troves to not be liquidate-able temporarily.

I confirmed this submission as the documentation of the project states in the priority of liquidation methods chapter that a trove should be liquidate-able if its TVL exceeds 90% (i.e. the ABSORPTION_THRESHOLD ). The code incorrectly validates the trove’s threshold rather than LTV, rendering the submission to be valid.

I consider a medium-risk severity apt for this finding as the DoS is temporary.

tserg (Opus) commented:

This is an error in the documentation.

The correct wording should be:

Absorption can happen only after an unhealthy trove's LTV has exceeded the LTV at which the maximum possible penalty is reached, or if its threshold exceeds 90% LTV and its LTV has exceeded the threshold 0xsomeone (judge) commented:

The sponsor has clarified that the documentation was incorrect and that the code behaves as expected; however, per C4 standards I will accept this submission as valid given that the documentation serves as the source of truth for the wardens to validate.

0xTheC0der (warden) commented:

First of all, thanks for keeping the issue valid due to the source of truth consideration. I can confirm that’s how we handle such cases on C4 and the present case serves as good exmaple of fair judging.

Anyways, I still want to provide further insights about this since it might be relevant for the sponsor.

According to the sponsor’s update:

Absorption can happen only after an unhealthy trove’s LTV has exceeded the LTV at which the maximum possible penalty is reached, or if its threshold exceeds 90% LTV and its LTV has exceeded the threshold The following would be true:

A trove with 88% threshold could only be absorbed > 92.5% LTV (due to max. penalty).

A trove with > 90% threshold could already be absorbed > 90% LTV (due to 90% threshold).

This is contradictory and the discrepancy starts arising for thresholds > 83% effectively creating an “absorption gap”, see main report and graph.

As far as I understood the mechanics of the protocol, the initially documented LTV criteria of absorption seemed to be the most reasonable while the code is subject to the above discrepancy. Therefore, I still recommend to go with the implementation according to the main report’s mitigation measures.

For anyone wanting to play around with threshold vs. LTV vs. max. penalty, I’ve created this graph which is based on the sponsor’s initial graph from the docs.

I hope I could provide further insights and value!

tserg (Opus) commented:

To give some context, the alternative condition of “if its threshold exceeds 90% LTV and its LTV has exceeded the threshold” for absorption was added because:

As a matter of convenience, because at thresholds greater than 90%, there is relatively less room for the LTV to increase before the maximum penalty is reached; and At these high thresholds, the balance lies in favour of securing the protocol by liquidating these positions however the method (searcher or absorber) before they become underwater.

In my opinion, the “absorption gap” for thresholds > 83%, while conceptually contradictory, is acceptable because searcher liquidations is already available once LTV > threshold. Of course, the contradiction is more jarring the closer we get to 90% (e.g. 88% threshold as you pointed), but a line has to be drawn somewhere and 90% is a convenient round figure.

milancermak (Opus) confirmed
