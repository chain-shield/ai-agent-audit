# Accepted H/M Findings: Lavarage Invitational

# [H-01] Collateral can be claimed back without repaying its corresponding loan due to insufficient instruction validation

- **Contest:** Lavarage Invitational
- **Slug:** 2024-04-lavarage-invitational
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-lavarage-invitational
- **Source snapshot:** competitions/2024-04-lavarage-invitational/final_report.html

Submitted by Arabadzhiev, also found by Koolex Users can bypass the repayment of their loans when claiming their collateral, which can be abused in order to drain any trading pool.

## Recommended Mitigation Steps

Replace the current verification checks with a single one for the position_account value:

if ix_discriminator == crate::instruction::TradingCloseRepaySol::DISCRIMINATOR { - require_keys_eq!( - ix.accounts[2].pubkey, - ctx.accounts.trading_pool.key(), - FlashFillError::IncorrectProgramAuthority - ); - require_keys_eq!( - ix.accounts[1].pubkey, - ctx.accounts.trader.key(), - FlashFillError::IncorrectProgramAuthority - ); - require_keys_eq!( - ctx.accounts.position_account.trader.key(), - ctx.accounts.trader.key(), - FlashFillError::IncorrectProgramAuthority - ); - require_keys_eq!( - ctx.accounts.position_account.pool.key(), - ctx.accounts.trading_pool.key(), - FlashFillError::IncorrectProgramAuthority - ); + require_keys_eq!( + ix.accounts[0].pubkey, + ctx.accounts.position_account.key(),

+ FlashFillError::IncorrectProgramAuthority + );...

}

## Assessed type

Invalid Validation piske-alex (Lavarage) confirmed

# [H-02] A borrower can borrow SOL without backing it by a collateral

- **Contest:** Lavarage Invitational
- **Slug:** 2024-04-lavarage-invitational
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-lavarage-invitational
- **Source snapshot:** competitions/2024-04-lavarage-invitational/final_report.html

Submitted by Koolex, also found by Arabadzhiev and rvierdiiev The borrower can borrow SOL from the lender without backing it by a collateral. This is possible because the borrower can open two positions at the same time (same TX) but link both addCollateral to one position. Although borrow checks the existence of addCollateral, it doesn’t check if the positions match.

This can be done as follows:

The borrower opens two positions ( Pos#1 and Pos#2 ).

When opening the position, the borrower links both collateral to Pos#1.

The borrower repays Pos#1.borrowed, Thus, withdrawing both collaterals.

Now, the protocol has no collaterals.

The borrower got away with Pos#2.borrowed without adding a collateral.

Check the PoC below, It demonstrates how a thief could perform the scenario above.

## Recommended Mitigation Steps

On borrow validate that the TradingOpenAddCollateral has the relevant position account.

## Assessed type

Invalid Validation piske-alex (Lavarage) confirmed

# [H-03] Malicious borrowers will never repay loans with high interest

- **Contest:** Lavarage Invitational
- **Slug:** 2024-04-lavarage-invitational
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-lavarage-invitational
- **Source snapshot:** competitions/2024-04-lavarage-invitational/final_report.html

Submitted by DadeKuma, also found by DadeKuma and Arabadzhiev Borrowers have no incentives to repay the loan if the owed interest grows too much, as the liquidation check fails to take it into consideration when calculating the LTV. This will generate bad debt for the lenders.

## Recommended Mitigation Steps

Consider adding the owed interest to the total amount when performing the liquidation check.

## Assessed type

Invalid Validation piske-alex (Lavarage) confirmed alcueca (judge) decreased severity to Medium and commented:

Downgraded to Medium because even if borrowers can effectively steal the borrowed amount, to do so they need to keep an amount of collateral of higher value locked in the protocol.

DadeKuma (warden) commented:

@alcueca - I disagree, as this clearly warrants High severity if we follow the severity categorization. There are zero hypotheticals, any borrower can get a loan for an unlimited amount of time (and not pay ANY interest), without consequences.

It’s like saying that I go to the bank to get a loan, never pay the interest (without consequences), and they can’t liquidate me until the collateral I provided is worthless. The bank is experiencing a loss of funds because it is lending money for free.

alcueca (judge) commented:

The attacker is experiencing a larger loss of funds, which makes it a grieving attack, which is Medium.

DadeKuma (warden) commented:

@alcueca - Consider the following case. The lender lends 1000 SOL with a 10% monthly rate. Let’s say that the collateral is worth 1500 SOL.

Every month, the borrower should pay 100 SOL, but they can ignore the payments without consequences.

After 6 months, the lender has lost more funds than the borrower (they should have earned 600 SOL but they have earned 0 ), and they can’t do anything to claim the collateral, they are lending money for free. But they are forced to wait until the collateral is worthless.

Consequences:

Lenders miss interest payments, potentially forever.

Meanwhile, they can’t offer new loans, as their funds are already locked in this one.

The protocol is useless if no one repays their loans as lenders only lose money.

If accrued interest is higher than collateral, the borrower will never repay, because at this point the liquidation costs less than the repayment.

The lender is accruing bad debt, which is a clear loss of funds for the lender. Recouping the collateral is not enough if their funds stay locked for decades.

alcueca (judge) commented:

When measuring financial losses it is uncommon to consider loss of future income or opportunity cost. I see this vulnerability as the victim putting X assets into the protocol, and the attacker spending Y assets so that the victim loses theirs, with the value of Y greater than the value of X.

If the victim would have put their assets somewhere expecting 0% income, for example, in an escrow contract, and they would get locked in the same fashion, you could still make the reasoning that because the victim has been blocked from withdrawing and earning an income on their assets in perpetuity, the issue is a critical.

The reasoning can be extended to make all grieving attacks of critical severity, which isn’t fair to attacks where the attacker obtains an actual profit.

DadeKuma (warden) commented:

It’s not future income as interest is accrued daily. High risk means that funds can be compromised directly (it’s worth noting that this isn’t a dust amount, and any borrower can do it). The actual rules say nothing about griefing, and borrowing/lending is a core feature of this protocol.

There are multiple scenarios:

Scenario 1:

The borrower locks collateral but they recoup some immediately by taking the borrowed amount. The lender has no access to any funds until liquidation, which can’t be enforced as interest is not accrued. For a % cost of the total borrowed amount, the attacker can lock a sizeable capital forever.

Scenario 2:

There is no attacker, and a borrower has lost access to their wallet. The loan remains unliquidable forever, and the lender loses 100% of the capital as there is no time limit for a loan (like described in #9 which was duped to this issue).

Picodes (Appellate Court lead judge) commented:

Lavarage Appellate Court Decisions Summary Issue #10 describes how because loans have no fixed duration and interests are not taken into account into the liquidiation mechanism, lenders cannot count on interests to trigger a liquidation at some point in the future, and there is a point time after which it isn’t profitable anymore for the borrower to pay back its loan. It’s related to #9 which focuses on the business implications of having infinite duration loans.

Lavarage’s (sponsor) input:

The sponsor was asked for his input on the original design he had in mind and the value added by #9 and #10. Here is his answer:

“I can confirm that the design I am willing to implement is that loans do not have a fixed term. But I also agree that that could be a risk on the business logic side. However, we are looking into implementing interest payment collection through sales of collateral instead of setting a fixed time for the loans. As mentioned above I agree that #9 is a valid concern in regards of business logic design. I don’t think we have discussed it in our documentation.” Picodes’ (lead judge) view:

I think H is more appropriate as well. For sure there will be users leverage trading and losing their borrowed amount, losing their keys, etc, and the main backstop for lenders is that due to interests they will get their collateral back at some point. That’s the classic behavior for Aave, Compound, Morpho, etc. Without this they can only rely on price movements which isn’t the deal. As a proof I think Aave V2 or deprecated lending markets speaks for themselves where lenders are just waiting for liquidations to be able to withdraw and the amounts at stake are significant.

0xTheC0der’s (judge 2) view:

I am viewing this from the following perspectives:

Adversary:

Is never at a profit by not repaying the loan. Would have been better off just swapping the collateral for the borrowed asset. Therefore, a grieving attack and no theft of assets.

Protocol:

Missing out on interest (loss of yield), but no direct theft. Collateral is unusable/locked until price swing allows liquidation on LTV > 90%, could be pretty permanent in case of stable assets. Bad debt once interest accrual puts actual LTV > 100%, but only “in the books” i.e. no direct loss.

Leaning towards High severity due to the indefinite lockup of collateral assets even without malicious user intent which effectively translates into a loss of the value of the borrowed assets. While the warden deserves to have their finding upgraded to High severity, this is a borderline case and also the audit judge’s assessment of Medium severity seems to hold under C4 rules and therefore cannot be labeled a “clear mistake” as the appellate court rules currently suggest in case of a 3/3 agreement on High severity. Consequently, I want my final verdict to be interpreted in such a way that a 2/3 agreement on High severity is reached.

Hickuphh3’s (judge 3) view:

I think H severity is more appropriate, not because of the loss of unrealised yield (this would be M), but because of the collateral that the lender is entitled to can be indefinitely locked, as long as LTV is <= 90%.

Should interest accrual be accounted for, the position will eventually be liquidatable at some point, but excluding it means the condition may only be met from asset price changes Verdict By a 2/3 consensus, the conclusion from the appellate court is that the ruling should be overruled and the issue should be made of High severity.

Note, this finding was upgraded to High by C4 staff in reference to the Appellate Court decision.

Medium Risk Findings (4)

# [M-01] Lack of freeze authority check for collateral tokens on create trading pool

- **Contest:** Lavarage Invitational
- **Slug:** 2024-04-lavarage-invitational
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-lavarage-invitational
- **Source snapshot:** competitions/2024-04-lavarage-invitational/final_report.html

Submitted by Koolex SPL tokens are used as collateral in the protocol. On borrow, there is a transfer from the borrower into a PDA (position account). On repay, the other way around.

However, SPL token could have a freeze authority. Therefore, any account is vulnerable to be frozen. This could be harmful for both borrowers and lenders. I believe, The protocol should be resilient enough to not fall into such situations where the funds are locked and borrowing or repaying are DoSed.

## Recommended Mitigation Steps

Ensure the collateral token does not have an active freeze_authority. If the freeze_authority was set to None, then freezing feature can never work again.

## Assessed type

Access Control piske-alex (Lavarage) confirmed alcueca (judge) commented:

Even given that this will be an exceedingly rare event, there will be losses to innocent users if the account of a trading pool becomes frozen. Given that this is an avoidable issue, the severity stays as Medium.

# [M-02] Borrowers can avoid the payment of an interest share fee by setting themselves as a fee_receipient

- **Contest:** Lavarage Invitational
- **Slug:** 2024-04-lavarage-invitational
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-lavarage-invitational
- **Source snapshot:** competitions/2024-04-lavarage-invitational/final_report.html

fee_receipient Submitted by Arabadzhiev, also found by Arabadzhiev, DadeKuma, Koolex, and rvierdiiev src/processor/swapback.rs#L185 src/processor/swapback.rs#L192 src/context/repay_sol.rs#L23

## Impact

Borrowers can avoid the payment of the 20% interest share fee on their accumulated interest.

## Recommended Mitigation Steps

Apply some restrictions on the fee_receipient public key value. For example, you can make it be a property of the Pool struct that is set on the creation of each new trading pool by its operator.

## Assessed type

Invalid Validation piske-alex (Lavarage) confirmed

# [M-03] Innocent borrower could incur losses caused by a malicious lender

- **Contest:** Lavarage Invitational
- **Slug:** 2024-04-lavarage-invitational
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-lavarage-invitational
- **Source snapshot:** competitions/2024-04-lavarage-invitational/final_report.html

Submitted by Koolex The protocol allows the lender to change the interest rate anytime. However, since the new interest rate is stored on trading pool level, the lender could front-run a borrowing transaction that’s yet to be processed, updating the interest rate too high (up to 99). This is harmful to the borrower even if the borrower repays the SOL immediately. That’s because the minimum elapsed days on repay is set to be one let days_elapsed = ((current_timestamp - timestamp) as u64 / 86400 ) + 1; // +1 to ensure interest is charged from day 0 src/processor/swapback.rs#L145

## Recommended Mitigation Steps

Allow the borrower to pass maximum interest rate, this protects the borrower from any change of the interest rate that occur after they send their TX.

Another suggestion: store the interest rate on position level instead.

piske-alex (Lavarage) confirmed and commented:

Another suggestion: store the interest rate on position level instead.

Will implement max interest rate param. How do I store the interest rate on position before the position is created?

Koolex (warden) commented:

@piske-alex - That’s a very good point, as it can still be front-run.

However, if you still would like to avoid passing the interest rate as a param, interest rate should be stored in trading pool with updated_time, then on borrowing, check if there is not enough timespan between current timestamp and updated_time, revert accordingly. Otherwise, proceed and store the interest rate (for records only).

This should be a sufficient protection without requiring the user to pass max interest rate as a param due to the fact that, a Solana TX has an expiration time. So, if it is not processed within a certain time, it will never be.

During transaction processing, Solana Validators will check if each transaction’s recent blockhash is recorded within the most recent 151 stored hashes (aka “max processing age”). If the transaction’s recent blockhash is older than this max processing age, the transaction is not processed.

Check this for more info.

alcueca (judge) commented:

Front-running by validators is possible in Solana, and after a brief analysis of the current situation, concerning to some users. This issue can cause mild losses to users. Nothing major, but a headache for the protocol that will have to deal with the complaints and possibly refunds. Affected users would have to close their positions immediately if they notice the issue. All in all, a medium is a fair severity rating.

Note: For full discussion, see here.

# [M-04] Small loans will never be liquidated, generating bad debt for lenders

- **Contest:** Lavarage Invitational
- **Slug:** 2024-04-lavarage-invitational
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-lavarage-invitational
- **Source snapshot:** competitions/2024-04-lavarage-invitational/final_report.html

Submitted by DadeKuma, also found by adeolu There isn’t a minimum position requirement for borrowers when they start a loan. Malicious borrowers might abuse this to create a large amount of small loans that will be unprofitable to liquidate. This results in a total loss of funds for lenders as they will get only bad debt.

## Recommended Mitigation Steps

Consider implementing a minimum amount to start a loan.

## Assessed type

Invalid Validation piske-alex (Lavarage) confirmed alcueca (judge) decreased severity to Medium Note: For full discussion, see here.
