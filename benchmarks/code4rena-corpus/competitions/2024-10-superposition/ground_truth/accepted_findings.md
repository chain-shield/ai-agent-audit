# Accepted H/M Findings: Superposition

# [H-01] createPoolD650E2D0 will not work due to mismatch in solidity and stylus function definitions

- **Contest:** Superposition
- **Slug:** 2024-10-superposition
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-10-superposition
- **Source snapshot:** competitions/2024-10-superposition/final_report.html

createPoolD650E2D0 will not work due to mismatch in solidity and stylus function definitions Submitted by ZanyBonzy

- https://github.com/code-423n4/2024-10-superposition/blob/7ad51104a8514d46e5c3d756264564426f2927fe/pkg/sol/SeawaterAMM.sol#L160-L168
- https://github.com/code-423n4/2024-10-superposition/blob/7ad51104a8514d46e5c3d756264564426f2927fe/pkg/seawater/src/lib.rs#L802

## Recommended Mitigation Steps

Remove the unneeded parameters.

function createPoolD650E2D0( //@audit address /* token */, uint256 /* sqrtPriceX96 */, uint32 /* fee */, - uint8 /* tickSpacing */, - uint128 /* maxLiquidityPerTick */ ) external { directDelegate(_getExecutorAdmin()); }

## Assessed type

Context af-afk (Superposition) confirmed 0xsomeone (judge) increased severity to High and commented:

The Warden has correctly identified that the function definitions of the Solidity and Stylus contracts differ, resulting in the relevant functionality of the system being inaccessible.

In line with the previous audit’s rulings, I believe a high-risk rating is appropriate for this submission as pool creations are rendered inaccessible via any other functions in contrast to the original audit’s submission which permitted circumvention of this error.

DadeKuma (warden) commented:

I believe some key pieces of information are missing to provide an accurate severity assessment, which I will address in this comment.

It is true that createPoolD650E2D0 will not work if directly called, as it has the wrong ABI, and this finding is technically valid. However, there is a fallback function that allows the creation of new pools by using the correct ABI.

The correct ABI, like this issue points, is the following:

createPoolD650E2D0(address,uint256,uint32) So the third byte is 0x80:

function testAbi () public pure returns ( bytes1 ) { return abi.

encodeWithSignature ( "createPoolD650E2D0(address,uint256,uint32)", address ( 0 ), 0, 0 )[ 2 ]; } decoded output { “0”: “bytes1: 0x80” } If we look at the fallback function, the execution will fall under the executor fallback:

fallback () external { // swaps if ( uint8 ( msg.

data [ 2 ]) == EXECUTOR_SWAP_DISPATCH ) directDelegate ( _getExecutorSwap ()); // update positions else if ( uint8 ( msg.

data [ 2 ]) == EXECUTOR_UPDATE_POSITION_DISPATCH ) directDelegate ( _getExecutorUpdatePosition ()); // positions else if ( uint8 ( msg.

data [ 2 ]) == EXECUTOR_POSITION_DISPATCH ) directDelegate ( _getExecutorPosition ()); // admin else if ( uint8 ( msg.

data [ 2 ]) == EXECUTOR_ADMIN_DISPATCH ) directDelegate ( _getExecutorAdmin ()); // swap permit 2 else if ( uint8 ( msg.

data [ 2 ]) == EXECUTOR_SWAP_PERMIT2_A_DISPATCH ) directDelegate ( _getExecutorSwapPermit2A ()); // quotes else if ( uint8 ( msg.

data [ 2 ]) == EXECUTOR_QUOTES_DISPATCH ) directDelegate ( _getExecutorQuote ()); else if ( uint8 ( msg.

data [ 2 ]) == EXECUTOR_ADJUST_POSITION_DISPATCH ) directDelegate ( _getExecutorAdjustPosition ()); else if ( uint8 ( msg.

data [ 2 ]) == EXECUTOR_SWAP_PERMIT2_B_DISPATCH ) directDelegate ( _getExecutorSwapPermit2B ()); -> else directDelegate ( _getExecutorFallback ()); }

- https://github.com/code-423n4/2024-10-superposition/blob/7ad51104a8514d46e5c3d756264564426f2927fe/pkg/sol/SeawaterAMM.sol#L505
Current values:

uint8 constant EXECUTOR_SWAP_DISPATCH = 0; uint8 constant EXECUTOR_UPDATE_POSITION_DISPATCH = 1; uint8 constant EXECUTOR_POSITION_DISPATCH = 2; uint8 constant EXECUTOR_ADMIN_DISPATCH = 3; uint8 constant EXECUTOR_SWAP_PERMIT2_A_DISPATCH = 4; uint8 constant EXECUTOR_QUOTES_DISPATCH = 5; uint8 constant EXECUTOR_ADJUST_POSITION_DISPATCH = 6; uint8 constant EXECUTOR_SWAP_PERMIT2_B_DISPATCH = 7; Moreover, creating a pool is permissionless and intended by the Sponsor, it doesn’t have to be called by the executor admin; the executor fallback would be able to create new pools.

Therefore, there is no loss of funds, and the functionality of the protocol is not impacted in this way; I don’t see how a High risk can be justified. I believe this issue falls under the QA umbrella, as a function does not work according to specifications.

0xsomeone (judge) commented:

@DadeKuma - This submission’s assessment is in line with the previous audit and the presence of a fallback mechanism is not sufficient to justify the finding’s invalidation. Otherwise, any inaccessible functionality of the system could be argued as being present in the fallback function and all findings pertaining to it would have to be invalidated in the previous audit as well.

Given that wardens were aware of the judgment style of this particular submission type, I do not believe that downgrading it in the follow-up round is a fair approach.

# [H-02] Users are incorrectly refunded when liquidity is insufficient

- **Contest:** Superposition
- **Slug:** 2024-10-superposition
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-10-superposition
- **Source snapshot:** competitions/2024-10-superposition/final_report.html

Submitted by ZanyBonzy, also found by Q7, Tigerfrake, and DadeKuma In swap_2_internal, if the first pool doesn’t have enough liquidity, amount_in could be less than original_amount, and as expected, amount_in is taken from swapper. But the function still refunds original_amount - amount_in to the user if original_amount is more than amount_in.

From the function, we can see than amount_in is taken from swapper. Then the function checks if original_amount is more than amount_in, before which the difference is transferred back to the sender.

>> erc20::

take (from, amount_in, permit2)?; erc20::

transfer_to_sender (to, amount_out)?; >> if original_amount > amount_in { erc20::

transfer_to_sender ( to, original_amount >>.

checked_sub (amount_in).

ok_or (Error::TransferToSenderSub)?, )?; } An unnecessary refund is processed leading to loss of funds for the protocol. Malicious users can take advantage of this to “rob” the protocol of funds through the refunds.

## Recommended Mitigation Steps

No need to process refunds since amount_in is already taken.

erc20::take(from, amount_in, permit2)?; erc20::transfer_to_sender(to, amount_out)?; - if original_amount > amount_in { - erc20::transfer_to_sender( - to, - original_amount -.checked_sub(amount_in) -.ok_or(Error::TransferToSenderSub)?, - )?; }

## Assessed type

Context af-afk (Superposition) confirmed 0xsomeone (judge) commented:

The submission and its duplicates have correctly identified that the refund process in the swap_2_internal_erc20 function is extraneous and thus results in excess funds being sent to the user.

I believe a high-risk severity rating is appropriate as the issue manifests itself in all cases and would result in direct fund loss for the AMM pair.

af-afk (Superposition) commented:

For Issue #12 @0xsomeone how does this compare to your findings here?

0xsomeone (judge) commented:

@af-afk - I am unsure what comparison is to be drawn here. None of the findings are mine as I am a judge, and I do not believe that the finding referenced has any relation to this one when it comes to impact.

af-afk (Superposition) commented:

Sorry, I should clarify, I mean your assessment that both are valid. It’s not possible for both of these to be correct, right? I’m of the opinion that this refund should not be implemented after consideration (and this submission) since the contract’s quoting functionality should indicate that this is taking place.

0xsomeone (judge) commented:

@af-afk - the original submission shared was submitted in a audit that relies on a different commit hash from this one. As we can observe in the highlighted code segment, the code originally transferred the original_amount from the from address.

In the remediated code that was part of this audit, the code was updated to simultaneously extract the amount_in from the user and perform a refund. The incorrect aspect is that two different solutions for the same problem were incorporated, rendering the refund to be extraneous. I hope this clears things up!

af-afk (Superposition) commented:

Fixed:

- https://github.com/fluidity-money/long.so/commit/9c7657e8336208e3397b30c32d557379f88a5b87

# [H-03] No slippage control when withdrawing a position leads to loss of funds

- **Contest:** Superposition
- **Slug:** 2024-10-superposition
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-10-superposition
- **Source snapshot:** competitions/2024-10-superposition/final_report.html

Submitted by DadeKuma An attacker can sandwich a user withdrawing funds as there is no way to put slippage protection, which will cause a large loss of funds for the victim.

## Recommended mitigation steps

Consider reintroducing a withdrawal function that offers slippage protection to users (they should be able to choose amount_0_min, amount_1_min, amount_0_desired, and amount_1_desired ).

af-afk (Superposition) acknowledged 0xsomeone (judge) commented:

The submission has demonstrated that liquidity withdrawals from the system are inherently insecure due to being open to arbitrage opportunities as no slippage is enforced.

I am unsure why the Sponsor has opted to acknowledge this submission as it is a tangible vulnerability and one that merits a high-risk rating. The protocol does not expose a secure way to natively extract funds from it whilst offering this functionality for other types of interactions.

af-afk (Superposition) commented:

@0xsomeone - we won’t fix this for now since Superposition has a centralised sequencer, and there’s no MEV that’s possible for a third-party to extract using the base interaction directly with our provider.

DadeKuma (warden) commented:

@af-afk - I highly suggest fixing this issue, as a centralized sequencer does not prevent MEV extraction. You can check this impact on Arbitrum, for example.

Medium Risk Findings (3)

# [M-01] It’s still not possible to set pool’s protocol fees

- **Contest:** Superposition
- **Slug:** 2024-10-superposition
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-10-superposition
- **Source snapshot:** competitions/2024-10-superposition/final_report.html

Submitted by DadeKuma The admin can’t set a pool’s protocol fee because the function has not been implemented.

## Recommended Mitigation Steps

Consider adding the following function to SeaWaterAMM.sol:

function setFeeProtocolCBD3EC35 ( address /* pool */, uint8 /* feeProtocol0 */, uint8 /* feeProtocol1 */ ) external { directDelegate ( _getExecutorAdmin ()); }

## Assessed type

Access Control af-afk (Superposition) commented:

0xsomeone - It’s possible to call this function since the signature resolves it to the admin facet in the fallback.

0xsomeone (judge) commented:

Per discussions in #8 this is a valid, albeit, Medium risk issue.

af-afk (Superposition) commented:

We felt this was technically inaccurate given that the function signature corresponded to the right fallback, triggering the correct dispatch, but we opted to fix this in principal with the similar issues. We weren’t responsive at the time to affect the ruling.

# [M-02] Tokens are pulled from users without verifying pool status contrary to requirement

- **Contest:** Superposition
- **Slug:** 2024-10-superposition
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-10-superposition
- **Source snapshot:** competitions/2024-10-superposition/final_report.html

Submitted by Tigerfrake, also found by DadeKuma Both the update_position_internal() and adjust_position_internal() functions are responsible for managing token positions, which involves taking tokens from users. However, there is a critical inconsistency in how each function verifies the operational status of the liquidity pool before performing token transfers from the user.

update_position_internal() - checks if the pool is enabled before taking tokens from the user.

// if we're TAKING, make sure that the pool is enabled.

assert_or !( pool.

enabled.

get (), Error::

PoolDisabled ); --- SNIP --- erc20::

take ( pool_addr, token_0.

abs_pos ()?, permit_0 )?; erc20::

take ( FUSDC_ADDR, token_1.

abs_pos ()?, permit_1 )?; adjust_position_internal() - does not explicitly check whether the pool is enabled before proceeding.

let ( amount_0, amount_1 ) = self.

pools.

setter ( pool ).

adjust_position ( id, amount_0_desired, amount_1_desired )?; --- SNIP --- erc20::

take ( pool, amount_0, permit_0 )?; erc20::

take ( FUSDC_ADDR, amount_1, permit_1 )?; First, it calls self.pools.setter(pool).adjust_position(...) which has the following comment:

// [update_position] should also ensure that we don't do this on a pool that's not currently running self.

update_position ( id, delta ) The comment in the adjust_position() function implies that a check for the pool’s operational state is necessary and should be enforced in update_position(). However, update_position() function does not make such enforcement as it does not check for pool status.

## Impact

Users could unintentionally have their tokens adjusted or transferred to a pool that is not operational which is not in accordance with protocol requirement. This also exposes users to risks in the event that there are potential issues with the pool.

## Recommended Mitigation Steps

Modify the adjust_position_internal() function to include a status check before executing the position adjustment:

// Ensure the pool is enabled before making any adjustments + assert_or!(pool.enabled.get(), Error::PoolDisabled); af-afk (Superposition) acknowledged and commented via duplicate Issue #4:

We made the decision that we were going to allow this functionality for now. The reason being that in a programmatic context, the pool can be controlled to be enabled and disabled depending on the broader environment. We use this for example with 9lives to prevent trading of a market that’s expired, in lieu of remembering ownership at different time points of the asset. We made the decision that allowing people to supply liquidity could be useful in the future, and for this reason we also allowed supplying liquidity to a frozen pool as well.

0xsomeone (judge) commented via duplicate Issue #4:

The submission claims that an issue submitted in the original audit was not resolved properly; however, the Sponsor’s choice to acknowledge the issue does not contradict the original issue’s acceptance. As this audit is a follow-up one, it would have been helpful to discuss with the Sponsor directly about their intentions on how to resolve issue #31 of the original audit.

I believe that the issue is invalid based on the Sponsor’s intentions.

DadeKuma (warden) commented via duplicate Issue #4:

@0xsomeone - The original issue was fixed, but it introduced another bug (this issue).

Code documentation clearly states that adding liquidity to disabled pools shouldn’t be possible:

Requires the pool to be enabled unless removing liquidity. Moreover, it’s actually not possible to add liquidity to disabled pools by using the following path, like the documentation suggests:

update_position_C_7_F_1_F_740 > update_position_internal But it is still possible by using the path described in this issue:

incr_position_E_2437399 > adjust_position_internal > adjust_position > update_position Even if the documentation states that this shouldn’t be possible here:

// [update_position] should also ensure that we don’t do this on a pool that’s not currently // running This is clearly a discrepancy, and I strongly believe this is a valid issue based on the information available during the audit.

0xsomeone (judge) commented via duplicate Issue #4:

@DadeKuma - I believe the discrepancies between the documentation and the implementation are adequate to merit a proper medium-risk vulnerability rating and have re-instated it so.

af-afk (Superposition) commented:

Fixed here and here.

# [M-03] Incorrect slippage handling in swap_internal()

- **Contest:** Superposition
- **Slug:** 2024-10-superposition
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-10-superposition
- **Source snapshot:** competitions/2024-10-superposition/final_report.html

swap_internal() Submitted by Tigerfrake In the swap_internal() function, the slippage check uses the || operator to validate the swap results. This can lead to a scenario where one of the amounts ( amount_0_abs or amount_1_abs ) is allowed to be zero, potentially resulting in unwanted slippage.

assert_or !( amount_0_abs > U256::

zero () || amount_1_abs > U256::

zero (), // Problematic operator Error::

SwapResultTooLow ); Using the || operator allows the swap to proceed even if one of the amounts is zero, which could lead to unacceptable slippage.

Scenario:

Consider a user swapping 100 units of token A ( amount_0 ) for token B ( amount_1 ).

Due to slippage, token B ’s output ( amount_1_abs ) becomes zero, while token A’s output ( amount_0_abs ) remains positive.

With the current || operator, the swap would still be considered valid since one amount is greater than zero, even though the user receives no token B ( amount_1_abs = 0 ), resulting in a poor outcome for the user.

## Impact

Using the || operator means that one token amount can be zero while the other passes the check, leading to an imbalanced swap that might not meet user expectations.

## Recommended Mitigation Steps

Replace the || operator with the && operator to ensure both token amounts are greater than zero.

assert_or!( - amount_0_abs > U256::zero() || amount_1_abs > U256::zero(), + amount_0_abs > U256::zero() && amount_1_abs > U256::zero(), Error::SwapResultTooLow );

## Assessed type

Invalid Validation af-afk (Superposition) confirmed 0xsomeone (judge) commented:

The Warden has identified an incorrect conditional clause that would permit either zero-input non-zero output swaps or non-zero input zero output ones, the latter of which may occur in a realistic scenario and would be unacceptable for the user.

I consider a medium-risk severity rating to be acceptable for this behavior as it would solely manifest in low transaction amounts and high value discrepancy AMM pairs.

af-afk (Superposition) commented:

Fixed:

- https://github.com/fluidity-money/long.so/commit/b0d39cf8d1be2096cba9c845e424b17c958847c5
