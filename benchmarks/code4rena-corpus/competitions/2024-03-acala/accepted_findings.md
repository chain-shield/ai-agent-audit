# Accepted H/M Findings: Acala

# [H-01] transfer_share_and_rewards allows for self transfer

- **Contest:** Acala
- **Slug:** 2024-03-acala
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-03-acala
- **Source snapshot:** competitions/2024-03-acala/final_report.html

transfer_share_and_rewards allows for self transfer Submitted by ZanyBonzy, also found by ihtishamsudo The rewards library holds the transfer_share_and_rewards allows for self transfer which can be used to double shares and rewards. Important to note that the function, for now is not in use by the in-scope contracts. However, I still believe it’s worth pointing out.

## Recommended Mitigation Steps

Include a check in the function that returns if who == other.

Lambda (judge) increased severity to High xlc (Acala) confirmed and commented:

Fixed by this PR. Just want to highlight that transfer_share_and_rewards is not currently used.

# [H-02] Early user can break pool via inflation attack due to no minimum liquidity check in the incentive contract

- **Contest:** Acala
- **Slug:** 2024-03-acala
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-03-acala
- **Source snapshot:** competitions/2024-03-acala/final_report.html

Submitted by carrotsmuggler, also found by zhaojie The incentive contract does not enforce a minimum liquidity limit. This means users can have as little as 1 share in the pool. This can lead to inflation attacks as described below.

Let’s imagine the state of the pool is as follows:

There is a single depositor, with 1000 shares deposited. Rewards have been accumulated up to 500 tokens. The user can then withdraw 998 shares, leaving 2 shares. They will also claim the rewards, and leave 1 reward tokens in the pool. This is the setup for the inflation attack. The user can then deposit 1 share.

The inflation is calculated as shown below:

U256::

from (add_amount.

to_owned ().

saturated_into::< u128 >()).

saturating_mul (total_reward.

to_owned ().

saturated_into::< u128 >().

into ()).

checked_div (initial_total_shares.

to_owned ().

saturated_into::< u128 >().

into ()).

unwrap_or_default ().

as_u128 ().

saturated_into () Here total_reward=1, add_amount=1 and initial_total_shares=2. So the result is calculated to 0; so inflation is 0.

After this step, the initial_total_shares is updated to 3. Now the user can deposit 2 wei of shares without changing the inflation amount. Next iteration, they can deposit 4 shares. This way, the user can deposit 2**n shares each iteration, and inflate the initial_total_shares without affecting the reward inflation. This leads to the situation where the total_shares keeps growing according to the deposit, but the entire reward inflation mechanism is broken. This lets users steal reward tokens from other users, and is a high severity issue.

In fact, whenever the total_reward value is less than the total_shares, this issue can be triggered. This is because in those conditions, users can create deposits and have the reward_inflation evaluate to 0.

0 reward_inflation basically means later users can steal rewards of earlier users, as is outlined in the docs. However, this donation attack is more effective the lower the total_shares in the system.

## Recommended Mitigation Steps

Add a minimum liquidity limit. This will ensure the pool never reaches a liquidity amount so low that rounding errors become significant.

xlc (Acala) confirmed and commented:

It is actually almost impossible to trigger this in production because anyone can deposit into the incentives pool at any time. I.E. before rewards starts accumulates.

Fixed by this PR.

# [H-03] transfer_share_and_rewards can be used to transfer out shares without transferring reward debt due to rounding

- **Contest:** Acala
- **Slug:** 2024-03-acala
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-03-acala
- **Source snapshot:** competitions/2024-03-acala/final_report.html

transfer_share_and_rewards can be used to transfer out shares without transferring reward debt due to rounding Submitted by carrotsmuggler, also found by AM The function transfer_share_and_rewards can be used to split up the position in a single account into multiple accounts. The contract sends some of the shares to be held by the second account, and similarly also updates the reward debt of the receiving account so that the receiver cannot take out more rewards than they deserve.

This is calculated in the following snippet.

move_balance is the amount of the reward debt that is to be transferred to the receiver.

let move_balance = U256::

from (balance.

to_owned ().

saturated_into::< u128 >()) * U256::

from (move_share.

to_owned ().

saturated_into::< u128 >()) / U256::

from (share.

to_owned ().

saturated_into::< u128 >()); Here we see the calculation is simple and by default is rounded down. So if balance*move_share is lower than share, move_balance evaluates to 0. So the receiving account’s reward debt is not increased at all!

increased_rewards.

entry (*reward_currency).

and_modify (|increased_reward| { *increased_reward = increased_reward.

saturating_add (move_balance); Since move_balance is 0, the increased_reward is not updated. This means the new account now has shares, but no reward debt. So the receiving account can claim rewards that were already claimed.

This can be done multiple times to drain the reward pool.

The criteria is that balance*move_share has to be lower than share. This can be achieved by sending a small fraction of the funds to the receiving account, such that move_share is much lower than share. Also, if balance, the reward debt of the sender is low, this facilitates the attack more.

## Recommended Mitigation Steps

The calculation of move_balance should be changed to saturated round up instead of rounding down. This will ensure that the receiving account’s reward debt is updated correctly. The saturated rounding up is important since the reward debt should never be larger than the reward pool, or it will cause underflow errors when subtracting.

Another option is to revert transfer_share_and_rewards operations if the reward debt of the receiving account is calculated to be 0, unless the sending account ALSO has a reward debt of 0.

## Assessed type

Math xlc (Acala) confirmed and commented:

Just want to highlight that transfer_share_and_rewards is not currently used.

We will choose to not fix this issue as the impact are relatively small and a complete fix is non-trivial. I don’t think it is possible to make profit that is more than transaction fee anyway.

Medium Risk Findings (4)

# [M-01] Claiming rewards while the deduction rate is != 0 , allows for repeated withdrawal of redistributed rewards

- **Contest:** Acala
- **Slug:** 2024-03-acala
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-acala
- **Source snapshot:** competitions/2024-03-acala/final_report.html

!= 0, allows for repeated withdrawal of redistributed rewards Submitted by n4nika, also found by ABAIKUNANBAEV and djxploit If a participant in a pool claims rewards while the deduction rate is != 0, the deducted rewards are redistributed between all the participants.

fn payout_reward_and_reaccumulate_reward ( pool_id: PoolId, who: &T::AccountId, reward_currency_id: CurrencyId, payout_amount: Balance, reaccumulate_amount: Balance, ) -> DispatchResult { if !reaccumulate_amount.

is_zero () { <orml_rewards::Pallet<T>>::

accumulate_reward (&pool_id, reward_currency_id, reaccumulate_amount)?; } T::Currency::

transfer (reward_currency_id, & Self::

account_id (), who, payout_amount)?; Ok(()) } Since the deduction is redistributed between all participants (since accumulate_reward distributes to all participating users) including the one claiming the reward, they can repeatedly claim rewards and receive more of the rewards pool than they probably should.

## Recommended Mitigation Steps

Add the claiming user to accumulate_rewards and implement reaccumulating excluding the calling user.

xlc (Acala) disputed and commented:

This is intended behaviour and non issue.

# [M-02] Incentive accumulation can be sandwiched with additional shares to gain advantage over long-term depositors

- **Contest:** Acala
- **Slug:** 2024-03-acala
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-acala
- **Source snapshot:** competitions/2024-03-acala/final_report.html

Submitted by 0xTheC0der, also found by zhaojie, djxploit, and carrotsmuggler Incentives are accumulated periodically in intervals of T::AccumulatePeriod blocks. Thereby, a fixed incentive reward amount, which is set via update_incentive_rewards (…), is accumulated among all deposited shares of a respective pool. This means if only 1 share is deposited, it is entitled for the all the rewards of this period, if N shares are deposited, the rewards are split among them, etc. (

Furthermore, to be eligible for rewards, it is sufficient to deposit (DEX) shares before accumulate_incentives (…) is called via the on_initialize hook. This is also demonstrated in the transfer_reward_and_update_rewards_storage_atomically_when_accumulate_incentives_work()

## Recommended Mitigation Steps

The present issue scales with the size of T::AccumulatePeriod in terms of blocks. Therefore, it’s recommended (for fairness) to also track deposited shares in between the accumulation intervals and scale the incentive rewards according to the actual deposit duration.

## Assessed type

MEV xlc (Acala) disputed and commented:

Furthermore, to be eligible for rewards, it is sufficient to deposit (DEX) shares before accumulate_incentives(…) is called via the on_initialize hook It is not possible to do perform user triggered action before on_initialize. To exploit this, attacker will need to acquire a large number of dex share somehow, deposit it, what for a block, withdraw it, ????, and repeat the same thing on next minute.

Firstly, it is not possible to borrow such amount of share without some payments, because the lender have the full incentives to deposit the shares and getting the rewards. It is also not a lost free action to mint such amount of dex share and redeem them due to potential price exposure and sandwich risk.

Therefore, it is economically impossible to exploit this behavior and make a profit.

Lambda (judge) commented:

The described scenario has some stated assumptions with external requirements (availability of liquidity, possibility to front- and back-run the transactions risk free, etc…) that may not always hold in practice. But these are not very unreasonable assumptions and under these assumptions, a value leak is possible. This, therefore, fulfills the criteria of a valid medium according to the severity categorization.

# [M-03] Unbond_instant removes incorrect amount of shares

- **Contest:** Acala
- **Slug:** 2024-03-acala
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-acala
- **Source snapshot:** competitions/2024-03-acala/final_report.html

Unbond_instant removes incorrect amount of shares Submitted by TheSchnilch, also found by Aymen0909 With unbond_instant, a user can unbond a bonded amount directly without having to wait. However, they must pay a fee for this:

let amount = change.change; let fee = fee_ratio.

mul_ceil (amount); let final_amount = amount.

saturating_sub (fee); let unbalance = T::Currency::

withdraw (&who, fee, WithdrawReasons::TRANSFER, ExistenceRequirement::KeepAlive)?; T::OnUnstakeFee::

on_unbalanced (unbalance); T::OnUnbonded::

happened (&(who.

clone (), final_amount)); Here, the mistake is that T::OnUnbonded::happened is called with final_amount, meaning without a fee. As a result, a portion of the shares that were added as shares during bonding can no longer be removed. This, in turn, leads to these shares still receiving rewards that other users cannot receive.

If you insert a println!

statement into the functions bond and unbond_instant to display the amounts with which OnBonded::happened and OnUnbonded::happened are called, you will see when you execute the following code that not all shares are removed when the bonded amount of a user is removed with unbond_instant:

For bond:

+ println!("change.change: {:?}", change.change); 144: T::OnBonded::happened(&(who.clone(), change.change)); 145: Self::deposit_event(Event::Bonded { 146: who, 147: amount: change.change, 148: }); For unbond_instant:

+ println!("final_amount: {:?}", final_amount); 196: T::OnUnbonded::happened(&(who.clone(), final_amount)); 197: Self::deposit_event(Event::InstantUnbonded { 198: who, 199: amount: final_amount, 200: fee, 201: }); Code that can be inserted into the file modules/earning/src/tests.rs to test both functions:

#[test] fn

# [M-04] Storage can be bloated with low liquidity positions

- **Contest:** Acala
- **Slug:** 2024-03-acala
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-acala
- **Source snapshot:** competitions/2024-03-acala/final_report.html

Submitted by ZanyBonzy, also found by Bauchibred and carrotsmuggler The deposit_dex_share function enforce no minimum amount that can be deposited into the pool allows for creating multiple pool positions. This causes that in a coordinated effort, for a pretty cheap cost, users/attackers can create multiple low liquidity positions to bloat the runtime storage. This is very important as substrate framework requires optimization of storage to prevent bloat which can lead to high maintenance costs for the chain and a potential DOS. A more in detail explanation can be found here.

## Recommended Mitigation Steps

Introduce a minimum deposit amount.

xlc (Acala) confirmed and commented:

Fixed here.
