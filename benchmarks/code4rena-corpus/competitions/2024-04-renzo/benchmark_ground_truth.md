# Benchmark Ground Truth: Renzo

## Accepted H/M Findings

# Accepted H/M Findings: Renzo

# [H-01] Withdrawals can be locked forever if recipient is a contract

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-renzo
- **Source snapshot:** competitions/2024-04-renzo/final_report.html

Submitted by LessDupes, also found by Bauchibred and grearlake The WithdrawQueue contract allows users to request withdrawals of their ezETH tokens in exchange for a selected asset, such as ETH or an ERC20 token. After a cooldown period, users can call the claim() function to receive their withdrawn assets.

When the selected asset is ETH, the claim() function sends the ETH using the low-level transfer() function:

payable ( msg.

sender ).

transfer ( _withdrawRequest.

amountToRedeem ); However, transfer() only forwards 2300 gas, which is not enough for the recipient to execute any non-trivial logic in a receive() or fallback function. For instance, it is not enough for Safes (such as this one in use by the protocol) to receive funds, which require > 6k gas for the call to reach the implementation contract and emit an event:

Note: to view the provided image, please see the original submission here.

In this case, the impact is higher than that reported by 4naly3er because claim() requires the caller to be the same address that initiated the original withdrawal request via withdraw().

If a user calls withdraw() from a contract account like a multisig or smart contract wallet that has a receive() function requiring >2300 gas, their subsequent claim() call will fail permanently. The withdrawn ETH will be locked in the WithdrawQueue contract forever, leading to loss of funds.

## Recommended Mitigation Steps

Use call() instead of transfer() to send ETH in claim():

(bool success, ) = payable(msg.sender).call{value: _withdrawRequest.amountToRedeem}(""); require(success, "ETH transfer failed"); This forwards all available gas and allows contract recipients to execute arbitrary logic.

## Assessed type

ETH-Transfer jatinj615 (Renzo) confirmed alcueca (judge) commented:

The ruling from the Supreme Court is only consultative.

From my point of view, a bot report that can be reasonably upgraded in severity due to the specific context of the code is a valid finding. Other judges might see this differently, so this ruling shouldn’t be seen as authoritative by itself in future audits. Instead, jurisprudence should arise from a broader consensus.

The 4naly3er report states that:

The use of the deprecated transfer() function for an address may make the transaction fail That description of impact merits a Medium severity; however, in this case the severity is higher due to the two-step withdrawal process. The withdrawal address is locked in the withdraw step, which will work fine for smart contract wallets. However, upon calling claim, the transaction will revert.

The actual impact for the sponsor would be severe. The first few users trying this would have their funds locked. Even after efforts of communication by the team, this would be an ongoing issue that would bring considerable trouble.

I’m ruling this as a valid High, and all the duplicates that mention the two-step withdrawal process as valid duplicates.

Note: For full discussion, see here.

Renzo mitigated:

The PR allows contracts like multisigs to be able to claim the withdraw request in Native ETH by sending it through call instead of transfer.

Status:

Mitigation confirmed. Full details in reports from 0xCiphky, grearlake, Fassi_Security, Bauchibred, and LessDupes.

# [H-02] Incorrect calculation of queued withdrawals can deflate TVL and increase ezETH mint rate

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-renzo
- **Source snapshot:** competitions/2024-04-renzo/final_report.html

Submitted by LessDupes, also found by adam-idarrha, araj, zigtur, jokr, SBSecurity, fyamf, 0xCiphky, Tendency, p0wd3r, bigtone, maxim371, NentoR, kennedy1030, mussucal, 0xnightfall, FastChecker, baz1ka, aman, 0xAadi, 0xhacksmithh, 0rpse, and KupiaSec The function OperatorDelegator.getTokenBalanceFromStrategy() is used by the RestakeManager to calculate the protocol TVL, which in turn is used to calculate the amount of ezETH to mint against a given value in collateral tokens.

This function, however, incorrectly checks for the queued amount of address(this) instead of address(token); therefore, consistently failing to consider collaterals in the withdrawal process for calculation:

File:

OperatorDelegator.

sol 326:

/// @dev Gets the underlying token amount from the amount of shares + queued withdrawal shares 327:

function getTokenBalanceFromStrategy ( IERC20 token ) external view returns ( uint256 ) { 328:

return 329:

queuedShares [ address ( this )] == 0 330: ?

tokenStrategyMapping [ token ].

userUnderlyingView ( address ( this )) 331::

tokenStrategyMapping [ token ].

userUnderlyingView ( address ( this )) + 332:

tokenStrategyMapping [ token ].

sharesToUnderlyingView ( 333:

queuedShares [ address ( token )] 334: ); 335: } Within this code, queuedShares[address(this)] will always return 0; therefore, missing the opportunity to count the contribution of queuedShares[address(token)].

## Impact

Any amount of collateral in the OperatorDelegator withdrawal process will not be counted for TVL calculation. This causes the TVL to be low, so more ezETH will be minted for the same amount of collateral, unfairly favoring people who mint ezETH during an OperatorDelegator withdrawal, penalizing holders, and those who initiate a RestakeManager withdraw.

## Recommended Mitigation Steps

Consider changing the address used for the mapping lookup:

/// @dev Gets the underlying token amount from the amount of shares + queued withdrawal shares function getTokenBalanceFromStrategy(IERC20 token) external view returns (uint256) { return - queuedShares[address(this)] == 0 + queuedShares[address(token)] == 0 ? tokenStrategyMapping[token].userUnderlyingView(address(this)): tokenStrategyMapping[token].userUnderlyingView(address(this)) + tokenStrategyMapping[token].sharesToUnderlyingView( queuedShares[address(token)] ); } jatinj615 (Renzo) confirmed Renzo mitigated Status:

Mitigation confirmed. Full details in reports from 0xCiphky, grearlake, Fassi_Security, Bauchibred, and LessDupes.

# [H-03] ETH withdrawals from EigenLayer always fail due to OperatorDelegator ’s nonReentrant receive()

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-renzo
- **Source snapshot:** competitions/2024-04-renzo/final_report.html

OperatorDelegator ’s nonReentrant receive() Submitted by LessDupes, also found by blutorque, ilchovski, 0x73696d616f, zzykxx, kennedy1030, and KupiaSec

- https://github.com/code-423n4/2024-04-renzo/blob/519e518f2d8dec9acf6482b84a181e403070d22d/contracts/Delegation/OperatorDelegator.sol#L269
- https://github.com/code-423n4/2024-04-renzo/blob/519e518f2d8dec9acf6482b84a181e403070d22d/contracts/Delegation/OperatorDelegator.sol#L501

## Vulnerability details

The OperatorDelegator.completeQueuedWithdrawal() function is used by admins to finalize previously initiated withdraws of shares from EigenLayer.

We note that both this and the OperatorDelegator’s receive() functions are nonReentrant:

File:

OperatorDelegator.

sol 265:

function completeQueuedWithdrawal ( 266:

IDelegationManager.

Withdrawal calldata withdrawal, 267:

IERC20 [] calldata tokens, 268:

uint256 middlewareTimesIndex 269: ) external nonReentrant onlyNativeEthRestakeAdmin { 270:

uint256 gasBefore = gasleft (); 271:

if ( tokens.

length != withdrawal.

strategies.

length ) revert MismatchedArrayLengths (); 272:

273:

// complete the queued withdrawal from EigenLayer with receiveAsToken set to true 274:

delegationManager.

completeQueuedWithdrawal ( withdrawal, tokens, middlewareTimesIndex, true ); --- 501:

receive () external payable nonReentrant { 502:

// check if sender contract is EigenPod. forward full withdrawal eth received 503:

if ( msg.

sender == address ( eigenPod )) { 504:

restakeManager.

depositQueue ().

forwardFullWithdrawalETH { value:

msg.

value }(); However, the receive() function is normally called by the EigenPod in the call stack originated by the L274 completeQueuedWithdrawal() when receiveAsTokens == true like in this case. This particular instance of reentrancy is not only acceptable but also required to allow ETH redemptions from EigenLayer. However, the nonReentrant modifier prevents it.

## Impact

All withdrawals that include any amount of ETH will be permanently stuck in EigenLayer and won’t be redeemable. Only amounts coming from new deposits can be redeemed and the team will have no way to fill the withdrawal queues. To unblock them, the team will necessarily have to upgrade OperatorDelegator.

## Recommended Mitigation Steps

Consider removing nonReentrant from OperatorDelegator’s receive, or applying the modifier only in case msg.sender != eigenPod.

## Assessed type

Reentrancy jatinj615 (Renzo) confirmed via duplicate Issue #571 Renzo mitigated Status:

Mitigation confirmed. Full details in reports from 0xCiphky, grearlake, Fassi_Security, Bauchibred, and LessDupes.

# [H-04] Withdrawals logic allows MEV exploits of TVL changes and zero-slippage zero-fee swaps

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Finding ID:** H-04
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-renzo
- **Source snapshot:** competitions/2024-04-renzo/final_report.html

Submitted by guhu95, also found by cu5t0mpeo, bill, t0x1c, 0xCiphky, gjaldon ( 1, 2 ), 0xabhay ( 1, 2 ), WildSniper ( 1, 2, 3 ), 0rpse, GoatedAudits ( 1, 2, 3 ), honey-k12 ( 1, 2 ), Bauchibred ( 1, 2 ), jokr, blutorque ( 1, 2 ), Tendency ( 1, 2 ), crypticdefense ( 1, 2 ), Fassi_Security ( 1, 2 ), SBSecurity ( 1, 2, 3 ), peanuts ( 1, 2 ), tapir ( 1, 2, 3 ), MSaptarshi, kennedy1030 ( 1, 2 ), OMEN, LessDupes ( 1, 2 ), 0x007, ilchovski, zzykxx ( 1, 2, 3 ), gumgumzum, stonejiajia, Audinarey, RamenPeople, Ocean_Sky, 0x73696d616f, underdog, josephdara, p0wd3r, aslanbek, d3e4, KupiaSec, grearlake ( 1, 2 ), and GalloDaSballo Deposit and withdrawal requests can be done immediately with no costs or fees, and both use the current oracle prices and TVL calculation (

deposit, and withdraw ). Crucially, the withdrawal amount is calculated at withdrawal request submission time instead of at withdrawal claim time. Any small change in either ezETH value or in the price of a collateral token can be exploited at no cost by MEV. Specifically, if the price increases, a deposit is made before the increase, and a withdrawal request immediately after.

Additionally, in case of a supported LST’s sudden change in price (for example, due to price manipulation, an exploit of that LST, due to consensus layer penalties (slashing), or liquidity issues), external holders of that LST may frontrun the change, deposit the LST into Renzo, and immediately request a withdrawal of another asset (e.g., native ETH). In such situations, Renzo functions as a zero-slippage zero-fees oracle-price-based DEX for LSTs and ETH up to the TVL cap for the affected LST. Zero-slippage zero-fee oracle-price-based designs are notoriously vulnerable to both oracle manipulation and oracle latency attacks if not carefully prevented.

## Impact

The newly introduced frontrunning vector, due to incurring only gas fees, and no fee that is proportional to the size of the “trade”, allows profitably exploiting most TVL and oracle price changes, and exploiting previously exploitable updates (via Balancer’s usage of getRate() ) and even more profitably via the new vector.

The impact is that value, that otherwise should be distributed to ezETH holders, is constantly lost to MEV.

Additionally, ezETH holders lose value due to facilitating asset swaps with no slippage and no fees based on outdated oracle prices.

## Recommended Mitigation Steps

The redemption conversion should be performed at both request and claim time. If it results in a lower redeem value, that value should be used for the claim instead of the initial redeem amount. Additionally, a rate limit or a short delay on deposits with similar protection can be added as well.

function claim(uint256 withdrawRequestIndex) external nonReentrant {...

+ // All the code converting from ezETH amount to amountToRedeem as is done in withdraw() + if (amountToRedeem < _withdrawRequest.amountToRedeem) { + _withdrawRequest.amountToRedeem = amountToRedeem; + } }

## Assessed type

MEV alcueca (judge) commented:

The sponsor’s comment in #259 is relevant here, on why withdrawals are priced on withdraw, and not claim. The resulting implementation might have to take a trade-off between being arbitraged one way or another, or opt for a different implementation altogether.

jatinj615 (Renzo) confirmed Renzo mitigated:

The PR reduces the risk of arbitrage at withdraw by calculating the amount of withdrawing asset at time of withdraw as well as claim and returns the min of both amount to user.

Status:

Mitigation confirmed. Full details in reports from 0xCiphky, grearlake, and Bauchibred.

# [H-05] Withdrawals of rebasing tokens can lead to insolvency and unfair distribution of protocol reserves

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Finding ID:** H-05
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-renzo
- **Source snapshot:** competitions/2024-04-renzo/final_report.html

Submitted by LessDupes, also found by SBSecurity, peanuts, guhu95 ( 1, 2 ), bill, ilchovski, and RamenPeople The WithdrawQueue contract allows users to withdraw their funds in various tokens, including liquid staking derivatives (LSDs) such as stETH. The withdraw() function calculates the amount of the specified _assetOut token equivalent to the ezETH being withdrawn using the renzoOracle.lookupTokenAmountFromValue() function. This amount is then stored in the amountToRedeem field of a new WithdrawRequest struct, which is added to the user’s withdrawRequests array and the token’s claimReserve.

When the user later calls claim(), the contract transfers the amountToRedeem to the user via the IERC20.transfer() function.

However, this implementation does not properly handle rebasing tokens like stETH. The stETH balance of the WithdrawQueue can change between the time a withdrawal is recorded and when it is claimed, even though the contract’s stETH shares remain constant.

If the stETH balance decreases during this period due to a rebasing event (e.g., a slashing of the staked ETH), the amountToRedeem stored in the WithdrawRequest may exceed the contract’s actual stETH balance at the time of claiming. As a result, the withdrawal can fail or result in the user receiving a larger share of the total protocol reserves than intended.

The issue can be illustrated by comparing the behavior of withdrawals for non-rebasing and rebasing LSDs:

Non-rebasing LSD (e.g., wBETH):

User A requests a withdrawal of 10 wBETH (worth 10 ETH) from the protocol.

While the withdrawal is pending, wBETH’s underlying staked ETH suffers a 50% slashing event.

The price of wBETH drops to 0.5 ETH per token due to the slashing.

When User A claims their withdrawal, they receive 10 wBETH, which is now worth only 5 ETH.

User A bears the loss from the slashing event.

Rebasing LSD (e.g., stETH):

User B requests a withdrawal of 10 stETH (worth 10 ETH) from the protocol.

While the withdrawal is pending, the underlying staked ETH suffers a 50% slashing event.

Everyone’s stETH balances are rebased to maintain the ETH peg, so the protocol’s stETH balance is halved.

When User B claims their withdrawal, they receive the original 10 stETH (still worth 10 ETH) as recorded in the withdrawal request.

The protocol bears the loss from the slashing event, as it has sent out more than its fair share of the rebased stETH balance.

## Impact

The current withdrawal mechanism for rebasing tokens like stETH can lead to:

Unfair distribution of funds:

Users who claim their withdrawals after a rebasing event that decreases the contract’s balance will receive a larger share of the reserves than intended, at the expense of other users.

Withdrawal failures:

If the contract’s balance falls below the total amountToRedeem of all pending withdrawals due to rebasing, users will face transaction failures when attempting to claim their withdrawals.

## Recommended Mitigation Steps

To address the issue of unfair distribution of funds when withdrawing rebasing tokens like stETH, the WithdrawQueue contract should store and transfer the user’s withdrawal as stETH shares instead of a fixed stETH amount.

When a user initiates a withdrawal with stETH as the _assetOut, the contract should convert the calculated amountToRedeem to stETH shares using the stETH.getSharesByPooledEth() function:

uint256 sharesAmount = IStETH(stETHAddress).getSharesByPooledEth(amountToRedeem); The resulting sharesAmount should be stored in the WithdrawRequest struct instead of the amountToRedeem.

When the user calls claim(), the contract should transfer the stETH shares directly to the user using the stETH.transferShares() function:

IStETH(stETHAddress).transferShares(msg.sender, sharesAmount); By storing and transferring stETH shares instead of a fixed stETH amount, the contract ensures that each user receives their fair share of the stETH balance, regardless of any rebasing events that occur between the time of the withdrawal request and the claim.

To implement this mitigation, the contract should:

Check if the _assetOut is stETH when processing a withdrawal request.

If so, convert the amountToRedeem to stETH shares using stETH.getSharesByPooledEth() and store the shares amount in the WithdrawRequest struct.

Update the claim() function to check if the withdrawal is in stETH and, if so, transfer the shares directly using stETH.transferShares() instead of using the standard IERC20.transfer() function.

Note that this mitigation is specific to stETH and may need to be adapted for other rebasing tokens that use a similar shares-based system.

Furthermore, the claimReserve and withdrawalBufferTarget for stETH would also need to be stored in shares and converted to underlying in TVL and withdraw buffer calculations, respectively.

alcueca (judge) commented:

I’m going to sustain the high severity on the grounds that:

If the stEth balance increases, as it normally does, users lose value in comparison to non-rebasing LSTs.

If the stEth balance decreases, the protocol loses value in comparison to non-rebasing LSTs.

If the stEth balance decreases, the protocol might DoS.

The users that win in a slashing event are not the same users that lose during normal operation.

jatinj615 (Renzo) acknowledged

# [H-06] The amount of xezETH in circulation will not represent the amount of ezETH tokens 1:1

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Finding ID:** H-06
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-renzo
- **Source snapshot:** competitions/2024-04-renzo/final_report.html

xezETH in circulation will not represent the amount of ezETH tokens 1:1 Submitted by zzykxx, also found by 0x007, GoatedAudits, 0xCiphky, jokr, mt030d, fyamf, LessDupes, and GalloDaSballo The protocol allows to deposit ETH / WETH (or the specific chain native currency) on a supported L2 in order to mint ezETH tokens, this is the process:

User mints xezETH on L2s via xRenzoDeposit::deposit() in exchange for either ETH or WETH. The xezETH are minted based on the current ezETH valuation.

After some time the bridge sweepers transfer the ETH / WETH collected to the L1, via xRenzoDeposit::sweep(). The funds are transferred to the xRenzoBridge contract on L1 via Connext.

Connext calls xRenzoBridge::xReceive() on L1 which will receive the ETH / WETH and deposit them in the protocol via RestakeManager::depositETH(). This will mint ezETH tokens based on the current ezETH valuation, the ezETH tokens will then be locked in the lockbox in exchange for xezETH, which are then immediately burned because an equivalent amount should have already been minted on the L2 during step 1.

Theoretically, the amount of xezETH tokens minted during step 1 should be the same as the amount of ezETH tokens minted during step 3, but because on both steps the tokens are minted at the current valuation, and the valuation changes over time, there will be a discrepancy between the amount of xezETH and ezETH tokens in circulation.

This is an issue because XERC20Lockbox::withdraw() always exchanges xezETH for ezETH 1:1.

## Impact

The price of ezETH is expected to increase over time. This will create a situation where there will be more xezETH in circulation than ezETH, rendering some xezETH worthless and impossible to redeem for ezETH.

A situation in which the ezETH valuation decreases instead of increasing is also problematic, because the protocol will mint less xezETH than it should.

The discrepancy will become bigger and bigger with time.

## Recommended Mitigation Steps

The protocol should track the amount of xezETH tokens minted via xRenzoDeposit::deposit() on the L2 and mint the equivalent amount of ezETH tokens on L1 when xRenzoBridge::xReceive() is executed by passing the necessary data on the xcall() to Connext.

If this is implemented it’s possible for the valuation of ezETH tokens to change instantly after xRenzoBridge::xReceive() is executed, this is because the amount of ezETH tokens is not minted based on the current valuation anymore. As explained in other reports, the protocol will be subject to instant ezETH valuation changes (increase/decrease) no matter what (rewards, slashing, penalties), and should gracefully handle these situations via appropriate deposit and withdrawals queues.

jatinj615 (Renzo) acknowledged and commented:

Yes, theoretically it is correct. But the protocol tackles this by sending the updated mint rate of ezETH frequently to L2s and also sweeps funds every hour if above batch size keeping the collateralization in place for ezETH. Also the bridgeRouterFee (5 bps) deducted on L2 if the transactions goes through slow path (during which mint rate of ezETH can rise) the extra 5 bps routerFee deducted on L2 is accumulated in minting ezETH on L1 in xRenzoBridge::xReceive instead of getting deducted by connext routers.

alcueca (judge) commented:

The long term effect of this is akin to bad debt in lending protocols. Eventually, the protocol or the (last) users need to assume the losses.

0xTenma (warden) commented:

I think severity of this issue should be Medium instead of High because there is no direct loss of funds possible here. If the ratio of xezETH to ezETH is not equal 1:1 in any case then it is only possible and the protocol sends the updated mint rate of ezETH frequently to L2s to avoid this situation. Furthermore, According to C4 docs:

2 — Med: Assets not at direct risk, but the function of the protocol or its availability could be impacted, or leak value with a hypothetical attack path with stated assumptions, but external requirements.

Stated assumptions and external requirement such as oracle delay are required to face to this issue.

jatinj615 (Renzo) commented:

@alcueca - to add more to it as I have explained in the comment above and discussion in #70. The ezETH in prod is over collateralised due to the fact that in some cases (when connext routers don’t have enough liquidity to process fast path) the 5bps deducted on L2 is used to collateralise ezETH on L1 which is why lockbox currently has more ezETH against the xezETH minted on L1. Team has been monitoring and keeping a track on it.

Considering the above argument, please confirm on the severity.

0xCiphky (warden) commented:

Respectfully, I believe this should remain as High severity:

The finding clearly demonstrates a realistic scenario where the system could become insolvent, resulting in a loss of funds for users. Despite the current over-collateralization and precautions, there remains a long-term risk.

The audited implementation contains blockages that will affect efforts to maintain collateralization for ezETH. For example, if the MaxTVL is reached, the protocol cannot deposit in L1, causing an accumulation of minted L2 tokens until it becomes possible to deposit again, which could take a significant amount of time.

This issue can be intentionally exploited by users whenever the price on the L1 side is higher than on the L2 side.

# [H-07] DOS of completeQueuedWithdrawal when ERC20 buffer is filled

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Finding ID:** H-07
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-renzo
- **Source snapshot:** competitions/2024-04-renzo/final_report.html

completeQueuedWithdrawal when ERC20 buffer is filled Submitted by Aymen0909, also found by tapir ( 1, 2, 3 ), crypticdefense, gjaldon, eeshenggoh, gumgumzum, 0x73696d616f, LessDupes, GoatedAudits, and pauliax

- https://github.com/code-423n4/2024-04-renzo/blob/main/contracts/Delegation/OperatorDelegator.sol#L299-L303
- https://github.com/code-423n4/2024-04-renzo/blob/main/contracts/Deposits/DepositQueue.sol#L134-L137
Issue Description When the OperatorDelegator::completeQueuedWithdrawal function is invoked to finalize a withdrawal from EL, it attempts to utilize the accumulated ERC20 tokens to fill the ERC20 withdrawal buffer, as demonstrated in the code snippet below:

function completeQueuedWithdrawal ( IDelegationManager.Withdrawal calldata withdrawal, IERC20 [] calldata tokens, uint256 middlewareTimesIndex ) external nonReentrant onlyNativeEthRestakeAdmin { uint256 gasBefore = gasleft (); if ( tokens.

length != withdrawal.

strategies.

length ) revert MismatchedArrayLengths (); // Complete the queued withdrawal from EigenLayer with receiveAsToken set to true delegationManager.

completeQueuedWithdrawal ( withdrawal, tokens, middlewareTimesIndex, true ); IWithdrawQueue withdrawQueue = restakeManager.

depositQueue ().

withdrawQueue (); for ( uint256 i; i < tokens.

length; ) { if ( address ( tokens [ i ]) == address ( 0 )) revert InvalidZeroInput (); // Deduct queued shares for tracking TVL queuedShares [ address ( tokens [ i ])] -= withdrawal.

shares [ i ]; // Check if the token is not Native ETH if ( address ( tokens [ i ]) != IS_NATIVE ) { // Check the withdrawal buffer and fill if below buffer target uint256 bufferToFill = withdrawQueue.

getBufferDeficit ( address ( tokens [ i ])); // Get the balance of this contract uint256 balanceOfToken = tokens [ i ].

balanceOf ( address ( this )); if ( bufferToFill > 0 ) { bufferToFill = ( balanceOfToken <= bufferToFill ) ?

balanceOfToken:

bufferToFill; // Update the amount to send to the operator Delegator balanceOfToken -= bufferToFill; // Safely approve for depositQueue tokens [ i ].

safeApprove ( address ( restakeManager.

depositQueue ()), bufferToFill ); // Fill the Withdraw Buffer via depositQueue restakeManager.

depositQueue ().

fillERC20withdrawBuffer ( address ( tokens [ i ]), bufferToFill ); } // Deposit remaining tokens back to EigenLayer if ( balanceOfToken > 0 ) { _deposit ( tokens [ i ], balanceOfToken ); } unchecked { ++ i; } // Emit the Withdraw Completed event with withdrawalRoot emit WithdrawCompleted ( delegationManager.

calculateWithdrawalRoot ( withdrawal ), withdrawal.

strategies, withdrawal.

shares ); // Record the current spent gas _recordGas ( gasBefore ); } The function iterates over the withdrawn tokens array and, for each token, checks if the withdrawal buffer needs filling. If required, the function attempts to call the depositQueue::fillERC20withdrawBuffer function, which is responsible for directing the ERC20 to the withdrawal queue contract to fill the buffer.

The issue arises because the depositQueue::fillERC20withdrawBuffer function can only be accessed by the RestakeManager contract, as it enforces the onlyRestakeManager modifier, as depicted below:

/// @dev Allows only the RestakeManager address to call functions modifier onlyRestakeManager () { if ( msg.

sender != address ( restakeManager )) revert NotRestakeManager (); _; } function fillERC20withdrawBuffer ( address _asset, uint256 _amount ) external nonReentrant onlyRestakeManager {...

} Consequently, when the completeQueuedWithdrawal function attempts this call, it reverts because OperatorDelegator lacks access to the depositQueue::fillERC20withdrawBuffer function. This results in the entire withdrawal completion call reverting, rendering it impossible for the admin to retrieve funds from EL.

In summary, this issue triggers a persistent DOS of the OperatorDelegator::completeQueuedWithdrawal function, preventing the protocol and users from withdrawing funds from EL and resulting in a loss of funds.

## Impact

Persistent DOS of the OperatorDelegator::completeQueuedWithdrawal function, preventing the protocol from withdrawing funds from EL and leading to fund losses for the protocol and users.

Tools Used VS Code

## Recommended Mitigation

The simplest resolution is to grant access to the depositQueue::fillERC20withdrawBuffer function to everyone by removing the onlyRestakeManager modifier. This adjustment introduces no vulnerabilities to the protocol since any user calling it effectively donates funds to the protocol (to the withdrawal queue).

## Assessed type

DoS jatinj615 (Renzo) confirmed Renzo mitigated Status:

Mitigation confirmed. Full details in reports from 0xCiphky, grearlake, Fassi_Security, Bauchibred, and LessDupes.

# [H-08] Incorrect withdraw queue balance in TVL calculation

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Finding ID:** H-08
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-04-renzo
- **Source snapshot:** competitions/2024-04-renzo/final_report.html

Submitted by pauliax, also found by BiasedMerc, NentoR, gjaldon, crypticdefense, zhaojohnson, twcctop, bigtone, b0g0, DanielArmstrong, fyamf, GoatedAudits, 0xCiphky, zigtur, xg, SBSecurity, lanrebayode77, blutorque, aslanbek, Aamir, araj, TheFabled, t0x1c, tapir, eeshenggoh, p0wd3r, peanuts, Greed, 0xordersol, 14si2o_Flint, guhu95, m_Rassska ( 1, 2 ), ustazz, maxim371, Fassi_Security, shui, mt030d, aman, rbserver, mussucal, josephdara, zzykxx, honey-k12, 0xnightfall, Maroutis, Aymen0909, OMEN, Stefanov, FastChecker, hunter_w3b, gesha17, baz1ka, kinda_very_good, carlitox477, 0xAadi, 0rpse, ak1, 0x73696d616f, 0xhacksmithh, ilchovski, LessDupes, adam-idarrha, siguint, 0xnev, 0xPwned, carrotsmuggler, KupiaSec, grearlake, and oakcobalt ( 1, 2 ) When calculating TVL it iterates over all the operator delegators and inside it iterates over all the collateral tokens.

for ( uint256 i = 0; i < odLength; ) {...

// Iterate through the tokens and get the value of each uint256 tokenLength = collateralTokens.

length; for ( uint256 j = 0; j < tokenLength; ) {...

// record token value of withdraw queue if (!

withdrawQueueTokenBalanceRecorded ) { totalWithdrawalQueueValue += renzoOracle.

lookupTokenValue ( collateralTokens [ i ], collateralTokens [ j ].

balanceOf ( withdrawQueue ) ); } unchecked { ++ j; }...

unchecked { ++ i; } However, the balance of withdrawQueue is incorrectly fetched, specifically this line:

totalWithdrawalQueueValue += renzoOracle.

lookupTokenValue ( collateralTokens [ i ], collateralTokens [ j ].

balanceOf ( withdrawQueue ) ); It uses an incorrect index of the outer loop i to access the collateralTokens.

i belongs to the operator delegator index, thus the returned value will not represent the real value of the token. For instance, if there is 1 OD and 3 collateral tokens, it will add the balance of the first token 3 times and neglect the other 2 tokens. If there are more ODs than collateral tokens, the the execution will revert (index out of bounds).

This calculation impacts the TVL which is the essential data when calculating mint/redeem and other critical values. A miscalculation in TVL could have devastating results.

## Recommended Mitigation Steps

Change to collateralTokens[j].

## Assessed type

Math jatinj615 (Renzo) confirmed and commented:

Yeah, the index should be j not i.

Renzo mitigated Status:

Mitigation confirmed. Full details in reports from 0xCiphky, grearlake, Fassi_Security, Bauchibred, and LessDupes.

Medium Risk Findings (14)

# [M-01] Withdrawals can fail due to deposits reverting in completeQueuedWithdrawal()

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-renzo
- **Source snapshot:** competitions/2024-04-renzo/final_report.html

completeQueuedWithdrawal() Submitted by LessDupes, also found by 0x73696d616f ( 1, 2 ) The OperatorDelegator.completeQueuedWithdrawal() function serves to finalize a queued withdrawal consisting of different tokens, sending the withdrawn tokens to the WithdrawQueue contract up to the buffer amount, and depositing any excess tokens back into the corresponding EigenLayer strategy.

The issue is that the call to strategyManager.depositIntoStrategy() used to deposit any amount of excess tokens back into EigenLayer may revert, which would cause the entire completeQueuedWithdrawal() transaction to revert.

There are a couple reasons why depositIntoStrategy() may revert:

In StrategyManager.depositIntoStrategy(), the number of shares returned by the strategy is verified to be > 0. If the strategy mints shares at less than a 1:1 ratio, this can cause the transaction to revert, as the amount being deposited back may be as small as 1 wei.

EigenLayer’s stETH and wBETH strategies both implement per-transaction and total deposit limits ( stETH, wBETH ):

If the deposit limit of a strategy has been reached, attempting to complete a queued withdrawal containing any amount over the WithdrawQueue ’s buffer of the strategy’s token will fail.

If the amount being redeposited is large enough, it may surpass the per-deposit limit.

This issue forces the withdrawQueueAdmin (which is a separate entity from the nativeEthRestakeAdmin affected) to set the buffer for the affected token high enough for the full amount being withdrawn to be transferred to the WithdrawQueue. This will require a third party to intervene and the protocol to handle in an unintended way by increasing the withdraw buffer for one or more tokens.

Furthermore, another entry point that calls strategyManager.depositIntoStrategy() and will revert under the same conditions is RestakeManager.deposit(), the main deposit function for collateral tokens. Also here, the collateral token being deposited is first used to fill the WithdrawQueue ’s buffer, so the transaction can revert for any of the reasons outlined above.

## Recommended Mitigation Steps

Consider catching any reverts when depositing excess tokens into strategies. In order to ensure that the excess tokens remain in the system and are accounted for in the TVL, the best option may be to send them to the WithdrawQueue in the catch clause, regardless of whether the buffer for the given token is already full.

jatinj615 (Renzo) acknowledged and commented:

There is no max deposit limit on any LSTs in the EigenLayer anymore. All the MaxCaps have been lifted.

With 1 wei issue, yes the transaction will revert but it doesn’t affect the funds and will be retried later on. Acknowledging for 1 wei issue.

alcueca (judge) commented:

In general, accepting. Because in any case that EigenLayer puts conditions on deposits, it might make withdrawals fail as well.

# [M-02] Withdrawals and Claims are meant to be pausable, but it is not possible in practice

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-renzo
- **Source snapshot:** competitions/2024-04-renzo/final_report.html

Submitted by zigtur, also found by Sathish9098, LessDupes, tapir, xg, t0x1c, rbserver ( 1, 2 ), guhu95, eeshenggoh, TECHFUND, TheFabled, cu5t0mpeo, 0xCiphky, ladboy233, NentoR, 0xBeastBoy, ilchovski, bigtone, josephdara ( 1, 2 ), FastChecker, mt030d, ak1, 0x73696d616f, Aymen0909, and oakcobalt

- https://github.com/code-423n4/2024-04-renzo/blob/main/contracts/Withdraw/WithdrawQueue.sol#L13
- https://github.com/code-423n4/2024-04-renzo/blob/main/contracts/Withdraw/WithdrawQueue.sol#L206
- https://github.com/code-423n4/2024-04-renzo/blob/main/contracts/Withdraw/WithdrawQueue.sol#L279

## Impact

Administrator is not able to pause users’ withdrawals and claims as expected.

## Recommended Mitigation Steps

Consider implementing whenNotPaused modifier on claim and withdraw functions. The following patch implements such a fix.

diff --git a/contracts/Withdraw/WithdrawQueue.sol b/contracts/Withdraw/WithdrawQueue.sol index 786238c..91ec77b 100644 --- a/contracts/Withdraw/WithdrawQueue.sol +++ b/contracts/Withdraw/WithdrawQueue.sol @@ -203,7 +203,7 @@ contract WithdrawQueue is * @param _amount amount of ezETH to withdraw * @param _assetOut output token to receive on claim */ - function withdraw(uint256 _amount, address _assetOut) external nonReentrant { + function withdraw(uint256 _amount, address _assetOut) whenNotPaused external nonReentrant { // check for 0 values if (_amount == 0 || _assetOut == address(0)) revert InvalidZeroInput(); @@ -276,7 +276,7 @@ contract WithdrawQueue is * @dev revert on claim before cooldown period

* @param withdrawRequestIndex Index of the Withdraw Request user wants to claim */ - function claim(uint256 withdrawRequestIndex) external nonReentrant { + function claim(uint256 withdrawRequestIndex) whenNotPaused external nonReentrant { // check if provided withdrawRequest Index is valid if (withdrawRequestIndex >= withdrawRequests[msg.sender].length) revert InvalidWithdrawIndex(); Note: The patch can be applied with git apply.

## Assessed type

Context jatinj615 (Renzo) confirmed Renzo mitigated Status:

Mitigation confirmed. Full details in reports from 0xCiphky, grearlake, Fassi_Security, LessDupes, and Bauchibred.

# [M-03] Fixed hearbeat used for price validation is too stale for some tokens

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-renzo
- **Source snapshot:** competitions/2024-04-renzo/final_report.html

Submitted by Maroutis, also found by ZanyBonzy, ilchovski, and NentoR

- https://github.com/code-423n4/2024-04-renzo/blob/519e518f2d8dec9acf6482b84a181e403070d22d/contracts/Bridge/L2/Oracle/RenzoOracleL2.sol#L13
- https://github.com/code-423n4/2024-04-renzo/blob/519e518f2d8dec9acf6482b84a181e403070d22d/contracts/Bridge/L2/Oracle/RenzoOracleL2.sol#L52

## Impact

The stale period 86400 + 60 seconds used for the oracle price validation is too short for some tokens like ezETH for example on Arbitrum. This could lead to the protocol consuming stale prices on Arbitrum.

## Recommended Mitigation Steps

It is recommended to store a mapping that would record the hearbeat parameter for the stale period of each token and for every different chain.

## Assessed type

Oracle jatinj615 (Renzo) acknowledged Note: For full discussion, see here.

# [M-04] Price updating mechanism can break

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-renzo
- **Source snapshot:** competitions/2024-04-renzo/final_report.html

Submitted by Fassi_Security Renzo uses CCIP to send price updates from L1 -> L2 using:

function sendPrice ( CCIPDestinationParam [] calldata _destinationParam, ConnextDestinationParam [] calldata _connextDestinationParam ) external payable onlyPriceFeedSender nonReentrant { // omitted code for ( uint256 i = 0; i < _destinationParam.

length; ) { Client.

EVM2AnyMessage memory evm2AnyMessage = Client.

EVM2AnyMessage ({ receiver:

abi.

encode ( _destinationParam [ i ].

_renzoReceiver ), // ABI-encoded xRenzoDepsot contract address data:

_callData, // ABI-encoded ezETH exchange rate with Timestamp tokenAmounts:

new Client.

EVMTokenAmount []( 0 ), // Empty array indicating no tokens are being sent extraArgs:

Client.

_argsToBytes ( // Additional arguments, setting gas limit Client.

EVMExtraArgsV1 ({ gasLimit:

200_000 }) ), // Set the feeToken address, indicating LINK will be used for fees feeToken:

address ( linkToken ) }); // Get the fee required to send the message uint256 fees = linkRouterClient.

getFee ( _destinationParam [ i ].

destinationChainSelector, evm2AnyMessage ); if ( fees > linkToken.

balanceOf ( address ( this ))) revert NotEnoughBalance ( linkToken.

balanceOf ( address ( this )), fees ); // approve the Router to transfer LINK tokens on contract's behalf. It will spend the fees in LINK linkToken.

approve ( address ( linkRouterClient ), fees ); // Send the message through the router and store the returned message ID -> bytes32 messageId = linkRouterClient.

ccipSend ( _destinationParam [ i ].

destinationChainSelector, evm2AnyMessage ); // omitted code } The CCIP architecture allows for simultaneous CCIP calls to be made from one address.

However, there is a caveat. As per Chainlink Docs, under the second header:

If a user sends multiple messages and the first message isn’t successfully delivered and goes into a manual execution mode, does that mean all subsequent messages from the user will also be stuck?

It depends. If a message goes into manual execution mode due to receiver errors (unhandled exceptions or gas limit issues), subsequent messages don’t get automatically blocked, unless they would encounter the same error.

However, suppose a message goes into manual execution mode after the Smart Execution time window expires (currently 8 hours). In that case, subsequent messages must wait for the first message to be processed to maintain the default sequence.

If a message fails, it can be manually executed, but after the Smart Execution time window expires, which is 8 hours at the moment, all subsequent messages will fail until the failing message will succeed.

The problem is that on the L2 side of things, in CCIPReceiver.sol, the reverts are not handled gracefully. This can easily lead to a CCIP call that can never be executed, thus resulting in a Denial of Service.

These are the failure points during the _ccipReceive() call, that can lead to a CCIP call not being able to be delivered, called by a honest party, marked by an arrow inside _updatePrice:

function _ccipReceive ( Client.Any2EVMMessage memory any2EvmMessage ) internal override whenNotPaused { address _ccipSender = abi.

decode ( any2EvmMessage.

sender, ( address )); uint64 _ccipSourceChainSelector = any2EvmMessage.

sourceChainSelector; // Verify origin on the price feed if ( _ccipSender != xRenzoBridgeL1 ) revert InvalidSender ( xRenzoBridgeL1, _ccipSender ); // Verify Source chain of the message if ( _ccipSourceChainSelector != ccipEthChainSelector ) revert InvalidSourceChain ( ccipEthChainSelector, _ccipSourceChainSelector ); ( uint256 _price, uint256 _timestamp ) = abi.

decode ( any2EvmMessage.

data, ( uint256, uint256 )); xRenzoDeposit.

updatePrice ( _price, _timestamp ); emit MessageReceived ( any2EvmMessage.

messageId, _ccipSourceChainSelector, _ccipSender, _price, _timestamp ); } function updatePrice ( uint256 _price, uint256 _timestamp ) external override { if ( msg.

sender != receiver ) revert InvalidSender ( receiver, msg.

sender ); _updatePrice ( _price, _timestamp ); } function _updatePrice ( uint256 _price, uint256 _timestamp ) internal { // Check for 0 if ( _price == 0 ) { -> revert InvalidZeroInput (); } // Check for price divergence - more than 10% if ( _price > lastPrice && ( _price - lastPrice ) > ( lastPrice / 10 )) || ( _price < lastPrice && ( lastPrice - _price ) > ( lastPrice / 10 )) ) { -> revert InvalidOraclePrice (); } // Do not allow older price timestamps if ( _timestamp <= lastPriceTimestamp ) { -> revert InvalidTimestamp ( _timestamp ); } // Do not allow future timestamps if ( _timestamp > block.

timestamp ) { -> revert InvalidTimestamp ( _timestamp ); } // Update values and emit event lastPrice = _price; lastPriceTimestamp = _timestamp; emit PriceUpdated ( _price, _timestamp ); } For example, if the price deviates more than 10%, the delivery will fail. If this delivery keeps failing, even after the 8 hours Smart Execution window has elapsed, the CCIP pathway will be DoS’ed until this CCIP call successfully delivers.

The impact is broad - price data will be corrupted, minting prices will not be calculated correctly and updates from L1 will be DoS’ed.

## Recommended Mitigation Steps

As per the Chainlink docs linked in the report:

Test thoroughly to ensure logical conditions for all paths are gracefully handled in your receiver contract.

Handle the reverts gracefully.

## Assessed type

Timing bronze_pickaxe (warden) commented:

Our issue is not a duplicate of this invalidated finding. Our issue describes multiple ways in which the CCIP path can be DoS’ed, without the need of a malicious entity. Just regular usage can lead to DoS because failures are not handled gracefully.

The Sponsor has confirmed that one of the two path ways of updating a price will be used, either CCIP or Oracle here, which means that the CCIP pathway being DoS’ed would break the L2 side of things + new data will not be able to go through.

To reiterate, there is no malicious entity needed for this pathway to be DoS’ed, the reverts marked with an -> in our issue’s are all point of failures.

EV_om (warden) commented:

I believe this finding is also invalid.

The interpretation of the Chainlink docs is incorrect. The quoted excerpt specifies that if a message goes into manual execution mode due to unhandled exceptions, subsequent messages don’t get automatically blocked. The only case in which a message blocks subsequent messages is if it “goes into manual execution mode after the Smart Execution time window expires”, which can only happen:

If the issue was due to extreme gas spikes or network conditions, and CCIP was not able to successfully transfer the message despite gas bumping for the entire duration of the Smart Execution time window This is:

Extremely unlikely to happen.

Unrelated to the unhandled reverts.

An inherent risk of CCIP which will affect all protocols integrating it equally.

Avoidable by the admin bumping the gas fee during such period.

bronze_pickaxe (warden) commented:

You skipped the following bullet point when copying the Chainlink docs:

manual-execution Unhandled exception (logical error) in the receiver contract: If the receiver contract is upgradeable, developers must correct the logic, re-deploy the logic contract, and then manually execute the same transaction. If the receiver contract is not upgradeable, developers must deploy a new receiver contract, and then users can send a new CCIP message If the CCIP path is stuck due to unhandled exceptions, the project would need to re-deploy said contracts.

For example, if the price would currently deviate more than 10%, every CCIP update would fail for length = Smart Execution Window due to this check:

// Check for price divergence - more than 10% if ( _price > lastPrice && ( _price - lastPrice ) > ( lastPrice / 10 )) || ( _price < lastPrice && ( lastPrice - _price ) > ( lastPrice / 10 )) ) { -> revert InvalidOraclePrice (); } Bumping the gas here makes no sense.

In Chainlink docs, under CCIP-Execution #6.2:

If the message does not involve token transfers, only arbitrary messaging, and the receiver execution fails due to gas limits or unhandled exceptions, the transaction becomes eligible for manual execution.

This means that a CCIP call can become eligible for the manual execution -> smart contract window expiry flow by either having a too low gas limit, which can be fixed by upping the gas, or by unhandled exceptions.

Lastly, what you are quoting is when a call becomes eligible for manual execution. In manual execution mode, you are allowed to bump the gas. Bumping the gas here will not make the transaction succeed, which will result in a DoS and to re-iterate, if it’s the case of an unhandled exception.

New deployment, not bumping the gas fees.

To summarize:

A CCIP call is made at timestamp k.

This CCIP call will fail due to one of the many unhandled exceptions, lets go with price deviation.

Subsequent calls will still fail due to the price deviation.

Timestamp is now k + eligble_for_manual_execution.

Bumping the gas price would not make it succeed.

Timestamp is now k + smart_execution_window_expired.

DoS.

jatinj615 (Renzo) acknowledged and commented:

From the above argument, if there is any issue with the checks, for example, highlighted 10% deviation. As, protocol updates the price feeds on L2 every 6 hours if the price deviation recorded on L2 is more than that, which means the feed is corrupted. Then, protocol would want to halt the deposit services on L2 until the new receiver is deployed or manually handled by owner through updatePriceByOwner.

alcueca (judge) commented:

I’m going to accept this as Medium, quite borderline. Burden of proof falls on the reporting warden, which has done a considerable effort taking into account the lack of facilities provided with the code. While not conclusively proven, the risk of malfunctioning is high enough with the evidence presented. The sponsor is recommended to apply a fix.

Note: For full discussion, see here.

# [M-05] calculateTVL may run out of gas for modest number of operators and tokens breaking deposits, withdrawals, and trades

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-renzo
- **Source snapshot:** competitions/2024-04-renzo/final_report.html

calculateTVL may run out of gas for modest number of operators and tokens breaking deposits, withdrawals, and trades Submitted by guhu95, also found by zzykxx, LessDupes, ilchovski, and Bauchibred The calculateTVLs function in the RestakeManager contains two nested loops: one loop for each operator delegator (OD), and an internal loop for each token. In each internal loop:

The OD’s getTokenBalanceFromStrategy function is called for the token, which in turn calls the Eigenlayer strategy for the shares balance.

The renzoOracle is called for the token value, which in turns calls the respective oracle.

Additionally, memory is allocated for all the results and intermediate call outputs, incurring high quadratic memory expansion costs.

Due to the the nested loop structure, the gas consumption in the case of several operator delegators, and several tokens used is quadratic:

n-OD x n-Tokens. Crucially, this method is called in most user flows:

deposits, withdrawals, and Balancer trades.

## Impact

An even small number of operators and supported tokens will render the protocol unusable, up to running out of block gas limit.

Currently, with 1 OD and 2 tokens, calculateTVLs consumes approximately 450K gas (see the example transaction gas profiling section).

For reasonable values, such as 12 ODs and 12 tokens ( Eigenlayer supports 12 tokens ), there would be 144 nested loop iterations instead of just 2 iterations in the current version and configuration. A simple extrapolation, without adding the cost of operations added in the new version, and without estimating the additional quadratic memory expansion costs, would suggest that 144 iterations, instead of the current 2, would cost 32M gas ( 450K * 144 / 2 ), exceeding the block gas limit.

However, an earlier limiting factor will be encountered because calculateTVL is called in most user interactions with Renzo:

deposit, withdrawal, and Balancer trade. Thus, exceeding even 1M-2M gas on L1 will render most protocol methods practically unusable for users.

## Recommended Mitigation Steps

Instead of “pulling” operator delegator token balances, a “pushing” pattern can be used:

Instead of “pulling” token TVLs from each component, the aggregated TVL sums should be maintained and “pushed” by each OD when its TVL changes (on OD deposits and withdrawals). This way, during admin’s OD operations, the RM’s TVL cache and each token’s total protocol balance is updated.

Then, total token balances can be used to calculate TVL from RenzoOracle.lookupTokenValues once, which will result in a single loop over the tokens only.

For enabling chooseOperatorDelegatorForDeposit a Set of ODs that can accept a deposit (are below allocation) can be maintained via “push” updates.

This will remove all loops during the calculation, except for the loop needed to iterate over the token oracles (once).

Alternatively, an off-chain custom TVL oracle can be used, for example, via a custom Chainlink oracle.

## Assessed type

DoS jatinj615 (Renzo) acknowledged

# [M-06] L1::xRenzoBridge and L2::xRenzoBridge uses the block.timestamp as dependency, which can cause issues

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-renzo
- **Source snapshot:** competitions/2024-04-renzo/final_report.html

L1::xRenzoBridge and L2::xRenzoBridge uses the block.timestamp as dependency, which can cause issues Submitted by ladboy233, also found by oakcobalt In L1::xRenzoBridge the block.timestamp from L1 is encoded and sent to L2. When the message is delivered from L1 to L2 with xRenzoBridge::_updatePrice(), the function checks the block.timestamp like this:

if ( _timestamp > block.

timestamp ) { revert InvalidTimestamp ( _timestamp ); } This check is done to not allow future timestamps for updating the price But the timestamps between two chains L1 and L2 are different for chain like Arbitrum as there’s a possibility that the sequencer fails to post batches on the parent chain (for example, Ethereum) for a period of time.

According to the Arbitrum docs:

Timestamp boundaries of the sequencer As mentioned, block timestamps are usually set based on the sequencer’s clock. Because there’s a possibility that the sequencer fails to post batches on the parent chain (for example, Ethereum) for a period of time, it should have the ability to slightly adjust the timestamp of the block to account for those delays and prevent any potential reorganisations of the chain. To limit the degree to which the sequencer can adjust timestamps, some boundaries are set, currently to 24 hours earlier than the current time, and 1 hour in the future.

So the issue is that timestamp validation for _updatePrice() won’t be effective and can reject validation both l2 tiimestamp is not related to l1 timestamp Block timestamps on Arbitrum are not linked to the timestamp of the L1 block. They are updated every L2 block based on the sequencer’s clock. These timestamps must follow these two rules:

Must be always equal or greater than the previous L2 block timestamp.

Must fall within the established boundaries (24 hours earlier than the current time or 1 hour in the future). More on this below.

Furthermore, for transactions that are force-included from L1 (bypassing the sequencer), the block timestamp will be equal to either the L1 timestamp when the transaction was put in the delayed inbox on L1 (not when it was force-included), or the L2 timestamp of the previous L2 block, whichever of the two timestamps is greater.

## Recommended Mitigation Steps

Remove the timestamp check in in L2 update rate.

## Assessed type

Timing jatinj615 (Renzo) acknowledged EV_om (warden) commented:

@alcueca - I believe this finding must be invalid.

While it is true that the sequencer can adjust the timestamp of a delayed block to up to 24 hours in the past, I believe this block will only ever include transactions that were received by the sequencer prior to that timestamp.

I have not been able to find a source to back up this claim, but this seems evident as the opposite would allow exploiting a sequencer downtime to include transactions in blocks with a timestamp in the past, which would have grave consequences for any protocol relying on block timestamps for their operations and allow manipulation of liquidations, governance votes, orders with expiration timestamps, and so on.

If this was indeed the case, the warden would have found a much more severe vulnerability than the trivial revert here.

On Optimism, for instance, this check is implemented here.

ladboy233 (warden) commented:

On Optimism, for instance, this check is implemented here.

The original report does not really mention optimism. The original report’s concern is that the L1 / L2 timestamp is not related and use L1 timestamp to validate against L2 timestamp can revert valid price update.

if ( _timestamp > block.

timestamp ) { revert InvalidTimestamp ( _timestamp ); } The L2 timestamp does not always bigger than L1 timestamp. For example, please free feel to run this POC:

from web3 import Web3 import time # Dictionary mapping blockchain names to RPC URLs rpc_urls = { ' Ethereum Mainnet ': ' https:

//eth.llamarpc.com', " blast ": " https:

//rpc.blastblockchain.com", } def get_block_timestamp ( name, url ):

try:

web3 = Web3 ( Web3.

HTTPProvider ( url )) latest_block = web3.

eth.

get_block ( 'latest' ) return f "{name} block timestamp: {latest_block.timestamp}, block number {latest_block.number}" except Exception as e:

return f "{name} Error: {e}" # Query each blockchain and print the latest block 's timestam p for name, url in rpc_urls.

items ():

print ( get_block_timestamp ( name, url )) The L1 block.timestamp is greater than blast (optimism fork) block.timestamp at the same time.

Example output:

Ethereum Mainnet block timestamp:

1716757859, block number 19956651 blast block timestamp:

1707500043, block number 2699354 From Arbitrum docs:

Block timestamps on Arbitrum are not linked to the timestamp of the L1 block.

alcueca (judge) commented:

Timestamps in L1 and L2 are not related and it can’t be demanded that one or the other is always greater.

# [M-07] Lack of slippage and deadline during withdraw and deposit

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-renzo
- **Source snapshot:** competitions/2024-04-renzo/final_report.html

Submitted by t0x1c, also found by ZanyBonzy, hunter_w3b, carlitox477 ( 1, 2 ), umarkhatab_465 ( 1, 2 ), btk, ladboy233, ilchovski, jokr, Shaheen, PNS, FastChecker, MSaptarshi, atoko, Tigerfrake, Maroutis, honey-k12, 0xDemon, DanielArmstrong, rbserver, NentoR, 0xCiphky, crypticdefense, twcctop, SBSecurity, Ocean_Sky, Rhaydden, and Bauchibred When users call withdraw() to burn their ezETH and receive redemption amount in return, there is no provision to provide any slippage & deadline params. This is necessary because the withdraw() function uses values from the oracle and the users may get a worse rate than they planned for.

Additionally, the withdraw() function also makes use of calls to calculateTVLs() to fetch the current totalTVL. The calculateTVLs() function makes use of oracle prices too. Note that though there is a MAX_TIME_WINDOW inside these oracle lookup functions, the users are forced to rely on this hardcoded value & can’t provide a deadline from their side. These facts are apart from the consideration that users’ call to withdraw() could very well be unintentionally/intentionally front-run which causes a drop in totalTVL.

In all of these situations, users receive less than they bargained for and, hence, a slippage and deadline parameter is necessary.

Similar issue can be seen inside deposit() and depositETH().

## Recommended Mitigation Steps

Allow users to pass a slippage tolerance value and a deadline parameter while calling these functions.

jatinj615 (Renzo) disputed and commented:

Oracle updates the value every 24 hours and technically, it creates an arbitrage opportunity which will not be beneficial to users as they will arbitraging 1 days reward share and losing on 7 days rewards due to coolDownPeriod.

We need the warden to provide a POC of delta around deposit and withdraw considering we will be implementing slashing pricing mechanism at the time of claim specified in #326.

sin1st3r__ (warden) commented:

@alcueca - I believe this should be a QA rather than a Med. You need slippage and deadline when the price can be manipulated like in an AMM. Here, since the price of the exchange is always fair because it can’t be moved by however large user operation, but only by changes in oracle price (that is fair by definition), there is no strong case for the same of level of protection as in AMMs.

Staking on Lido for example doesn’t have slippage or deadline either. See here.

alcueca (judge) commented:

The thing is that Renzo is using market oracles, as stated in #13. These oracles can fluctuate more than the actual exchange rate, due to market forces. In some situations the users might be displeased that their withdrawals rended less value because of a temporary spike in the oracle.

It is quite borderline, because it needs the oracles to have jumps large enough to bother users. However, I do see value to add slippage controls, and let users disable them in volatile conditions when they just want to dump.

# [M-08] Not handling the failure of cross chain messaging

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Finding ID:** M-08
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-renzo
- **Source snapshot:** competitions/2024-04-renzo/final_report.html

Submitted by fyamf, also found by tapir, 0xCiphky ( 1, 2 ), LessDupes, ladboy233, guhu95, t0x1c, and grearlake If the xReceive function fails to handle reverts properly, it could result in funds getting stuck. This can happen for example when:

Depositing into L2 when protocol on L1 is paused: because pausing is not defined on L2.

Depositing large amount on L2: because it is not enforced to deposit less than a limit on L2 Depositing too small amount on L2: because the ezETH rate on L1 and L2 could be different.

## Recommended Mitigation Steps

To address these potential failures, it is suggested to:

Implement error handling in the xReceive function. Specifically, placing the depositETH function call within a try/catch block could help manage these failures. Additionally, only authenticated addresses should be allowed to handle these errors.

Enforce the deposited amount on L2 to be less than maxDepositTVL.

Define the pausing mechanism on L2.

Allow the BrdigeSweepr to define the amount of token to be bridged to L1. By doing so, it can handle the situations where a large amount is deposited on L2.

function sweep ( uint256 _amount ) public payable nonReentrant { //...

}

- https://github.com/code-423n4/2024-04-renzo/blob/main/contracts/Bridge/L2/xRenzoDeposit.sol#L414

## Assessed type

Context jatinj615 (Renzo) acknowledged Blckhv (warden) commented:

@alcueca - I think this issue should be of low severity because we can see that xRenzoBridge has functions to retrieve the funds that are “stuck”, which is exactly what the first paragraph of the provided Connext documentation advises here. In all matters, manually distributing failed transactions funds is not an optimal solution, but in the end, funds are not really stuck, since the admin can still process them from the Connext directly.

sin1st3r__ (warden) commented:

@Blckhv - I’m afraid this is factually incorrect. Connext allows you to re-submit/retry a failed TX only if it failed due to insufficient relayer fee.

In the case it failed due to the xReceive() on the destination chain reverting, then the funds will be stuck. Not to mention that this failure is silent. Meaning that admins may be notified that this call failed on the destination chain very late.

That’s why the Connext docs says that the IXReceiver contract should be implemented defensively.

If the call on the receiver contract (also referred to as “target” contract) reverts, funds sent in with the call will end up on the receiver contract. To avoid situations where user funds get stuck on the receivers, developers should build any contract implementing IXReceive defensively.

Ultimately, the goal should be to handle any revert-susceptible code and ensure that the logical owner of funds always maintains agency over them.

Bridging silently failing and admins having to manually recover funds and manually redistribute to other parts of the system is surely Medium severity worthy.

Note: for full discussion, see here.

# [M-09] Deposits will always revert if the amount being deposited is less than the bufferToFill value

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Finding ID:** M-09
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-renzo
- **Source snapshot:** competitions/2024-04-renzo/final_report.html

bufferToFill value Submitted by 0xCiphky, also found by kinda_very_good, LessDupes, adam-idarrha, 0x007, bill, inzinko, underdog, baz1ka, bigtone, MaslarovK, 0xAadi, RamenPeople, Neon2835, hunter_w3b, FastChecker, Shaheen, gesha17, Aymen0909, m_Rassska, zzykxx, mussucal, josephdara, kennedy1030, Fassi_Security, DanielArmstrong, 14si2o_Flint, 0rpse, mt030d, ADM, cu5t0mpeo, Tendency, araj, BiasedMerc, tapir, Aamir, blutorque, ZanyBonzy, SBSecurity, jokr, xg, lanrebayode77, b0g0, gumgumzum, fyamf, carrotsmuggler, and KupiaSec The deposit function in the RestakeManager contract enables users to deposit ERC20 whitelisted collateral tokens into the protocol. It first checks the withdrawal buffer and fills it up using some or all of the deposited amount if it is below the buffer target. The remaining amount is then transferred to the operator delegator and deposited into EigenLayer.

The current issue with this implementation is that if the amount deposited is less than bufferToFill, the full amount will be used to fill the withdrawal buffer, leaving the amount value as zero.

function deposit ( IERC20 _collateralToken, uint256 _amount, uint256 _referralId ) public nonReentrant notPaused { // Verify collateral token is in the list - call will revert if not found uint256 tokenIndex = getCollateralTokenIndex ( _collateralToken );...

// Check the withdraw buffer and fill if below buffer target uint256 bufferToFill = depositQueue.

withdrawQueue ().

getBufferDeficit ( address ( _collateralToken )); if ( bufferToFill > 0 ) { bufferToFill = ( _amount <= bufferToFill ) ?

_amount:

bufferToFill; // update amount to send to the operator Delegator _amount -= bufferToFill; // safe Approve for depositQueue _collateralToken.

safeApprove ( address ( depositQueue ), bufferToFill ); // fill Withdraw Buffer via depositQueue depositQueue.

fillERC20withdrawBuffer ( address ( _collateralToken ), bufferToFill ); } // Approve the tokens to the operator delegator _collateralToken.

safeApprove ( address ( operatorDelegator ), _amount ); // Call deposit on the operator delegator operatorDelegator.

deposit ( _collateralToken, _amount );...

} Subsequently, the function will approve the zero amount to the operator delegator and call deposit on the operator delegator. However, as seen in the OperatorDelegator contract’s deposit function below, a zero deposit will be reverted.

function deposit ( IERC20 token, uint256 tokenAmount ) external nonReentrant onlyRestakeManager returns ( uint256 shares ) { if ( address ( tokenStrategyMapping [ token ]) == address ( 0x0 ) || tokenAmount == 0 ) { revert InvalidZeroInput (); } // Move the tokens into this contract token.

safeTransferFrom ( msg.

sender, address ( this ), tokenAmount ); return _deposit ( token, tokenAmount ); }

## Impact

Severity: Medium. User deposits will always revert if the amount being deposited is less than the bufferToFill value.

Likelihood: High. Depending on the set amount for the withdrawal buffer, this could be a common occurrence.

Recommendation To address this issue, the deposit function can be modified to only approve the amount to the operator delegator and call deposit on the operator delegator if the amount is greater than zero.

function deposit ( IERC20 _collateralToken, uint256 _amount, uint256 _referralId ) public nonReentrant notPaused { // Verify collateral token is in the list - call will revert if not found uint256 tokenIndex = getCollateralTokenIndex ( _collateralToken );...

// Check the withdraw buffer and fill if below buffer target uint256 bufferToFill = depositQueue.

withdrawQueue ().

getBufferDeficit ( address ( _collateralToken )); if ( bufferToFill > 0 ) { bufferToFill = ( _amount <= bufferToFill ) ?

_amount:

bufferToFill; // update amount to send to the operator Delegator _amount -= bufferToFill; // safe Approve for depositQueue _collateralToken.

safeApprove ( address ( depositQueue ), bufferToFill ); // fill Withdraw Buffer via depositQueue depositQueue.

fillERC20withdrawBuffer ( address ( _collateralToken ), bufferToFill ); } if ( _amount > 0 ) { // ADD HERE // Transfer the tokens to the operator delegator _collateralToken.

safeApprove ( address ( operatorDelegator ), _amount ); // Call deposit on the operator delegator operatorDelegator.

deposit ( _collateralToken, _amount ); }...

} jatinj615 (Renzo) confirmed Renzo mitigated Status:

Mitigation confirmed. Full details in reports from 0xCiphky, grearlake, Fassi_Security, LessDupes, and Bauchibred.

# [M-10] Potential arbitrage opportunity in the xRenzoDeposit L2 contract

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Finding ID:** M-10
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-renzo
- **Source snapshot:** competitions/2024-04-renzo/final_report.html

xRenzoDeposit L2 contract Submitted by 0xCiphky, also found by LessDupes The sendPrice function in the xRenzoBridge contract calls the getRate function to retrieve the current price of ezETH to ETH and broadcasts it to Layer 2 networks. Subsequently, the price is received by either the ConnextReceiver or CCIPReceiver and invokes the updatePrice function in the xRenzoDeposit contract on L2. However, a potential opportunity exists where a user can monitor the L1 mempool for the sendPrice function call, observe the new price, and sandwich it if profitable.

/** * @notice Exposes the price via getRate() * @dev This is required for a balancer pool to get the price of ezETH * @return uint256.

*/ function getRate () external view override returns ( uint256 ) { return lastPrice; } Upon detecting a favourable price change, the user could mint xezETH on L2 before the price adjustment takes effect. Subsequently, when the price change is finalized, the user can sell the xezETH on a protocol that reads the price from the getRate function in the xRenzoDeposit contract, thus profiting from the price discrepancy.

## Impact

Severity: Medium. This will allow users to exploit price changes by minting xezETH at a favourable rate before the price update is reflected.

Likelihood: High. Given the visibility of transactions in the mempool and the potential for arbitrage opportunities, it is likely that users will attempt to exploit this.

Recommendation Since there are two fees associated with L2 deposits, this should help minimize this problem. Additionally, as ezETH is more on the stable side, it is less prone to significant price fluctuations. However, continuous monitoring and adjustment of the update frequency may be necessary to prevent potential exploitation.

jatinj615 (Renzo) acknowledged and commented:

Expected behaviour.

# [M-11] Fetched price from the oracle is not stored in xRenzoDeposit

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Finding ID:** M-11
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-renzo
- **Source snapshot:** competitions/2024-04-renzo/final_report.html

xRenzoDeposit Submitted by fyamf, also found by Fassi_Security Failure to store the fetched price from the oracle in the storage variables of the xRenzoDeposit contract can result in several important issues.

## Recommended Mitigation Steps

To address these issues, it’s recommended to modify the getMintRate function to store the fetched price from the oracle in the storage variables. This ensures that the most recent price is used for calculations.

function getMintRate () public view returns ( uint256, uint256 ) { // revert if PriceFeedNotAvailable if ( receiver == address ( 0 ) && address ( oracle ) == address ( 0 )) revert PriceFeedNotAvailable (); if ( address ( oracle ) != address ( 0 )) { ( uint256 oraclePrice, uint256 oracleTimestamp ) = oracle.

getMintRate (); if ( oracleTimestamp > lastPriceTimestamp ){ lastPrice = oraclePrice; lastPriceTimestamp = oracleTimestamp; return ( oraclePrice, oracleTimestamp ); } else { return ( lastPrice, lastPriceTimestamp ); } else { return ( lastPrice, lastPriceTimestamp ); }

## Assessed type

Oracle jatinj615 (Renzo) acknowledged and commented:

In practical, we are not using both configurations together. But yeah, can be acknowledged. In my opinion, this is a low severity not a high.

alcueca (judge) decreased severity to Medium and commented:

The first issue described by the warden can be described as a governance error.

EV_om (warden) commented:

@alcueca - I agree with @jatinj615 here that this is a QA issue.

The first issue can be described as a governance issue as you said.

The second issue would be an issue in case of concurrent use of both the pull and push oracle configurations together as @jatinj615 pointed out, but that was never the intention here. The contract is working as designed, which is by only performing the divergence check on the push mechanism, against previous updates via the same mechanism.

The third issue is an inherent tradeoff of the divergence check - there is no way to prevent this issue while keeping the check, and it can be easily mitigated by the owner calling updatePriceByOwner() in incremental steps of 10% as pointed out by the warden.

alcueca (judge) commented:

The first issue is a governance issue, as agreed.

There is no documentation on the intended use of the code. The assumption from the original warden that both oracles would be used simultaneously is not an outrageous one, I assumed the same. Under that assumption, the second issue pointed out is valid.

On the third issue, I can see how the code is expected to halt due to large price changes, with an admin calling updatePriceByOwner to validate that the changes are real.

Note: For full discussion, see here.

# [M-12] Incorrect exchange rate provided to Balancer pools

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Finding ID:** M-12
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-renzo
- **Source snapshot:** competitions/2024-04-renzo/final_report.html

Submitted by LessDupes, also found by Bauchibred The xRenzoDeposit contract, apart from being the entry point for deposits on L2s, also acts as a rate provider for Balancer pools on L2s, allowing them to determine the exchange rate between xezWETH and WETH tokens. Rate providers are crucial for Balancer pools to calculate token prices and determine yield protocol fees during joins and exits, as explained in the Balancer documentation.

However, the getRate() function in xRenzoDeposit does not provide the correct exchange rate. It simply returns the lastPrice state variable, which:

May be stale if updatePrice() or updatePriceByOwner() have not been called recently.

May be older than the rate provided by oracle.getMintRate().

Can be different from the rate at which xezETH are minted in _deposit(), which uses the newest of the oracle and internal lastPrice values, opening up arbitrage opportunities.

## Impact

By providing an incorrect and potentially outdated exchange rate, xRenzoDeposit can cause Balancer pools to misprice xezWETH relative to WETH. This can lead to incorrect yield calculations and enable manipulation of pool joins and exits.

## Recommended Mitigation Steps

Update the getRate() function to provide the same exchange rate used when minting xezETH. This can be achieved by calling getMintRate() instead of returning lastPrice directly:

- return lastPrice; + (uint256 rate, uint256 timestamp) = getMintRate(); + require(block.timestamp <= timestamp + 1 days, "Price is stale"); + return rate; This ensures that the rate provided to Balancer pools is consistent with the actual minting rate and is not stale.

jatinj615 (Renzo) confirmed Renzo mitigated:

The PR adds staleness check in getRate function for balancerPools on L2.

Status:

Mitigation confirmed. Full details in reports from 0xCiphky, grearlake, Fassi_Security, LessDupes, and Bauchibred.

# [M-13] Pending withdrawals prevent safe removal of collateral assets

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Finding ID:** M-13
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-renzo
- **Source snapshot:** competitions/2024-04-renzo/final_report.html

Submitted by LessDupes, also found by kennedy1030, Bauchibred ( 1, 2 ), ZanyBonzy, inzinko, KupiaSec, OMEN, t0x1c, TECHFUND, 14si2o_Flint, Hajime ( 1, 2 ), oxwhite ( 1, 2, 3, 4 ), golu, Bigsam, CodeWasp, and 0xAadi

- https://github.com/code-423n4/2024-04-renzo/blob/main/contracts/RestakeManager.sol#L316-L321
- https://github.com/code-423n4/2024-04-renzo/blob/main/contracts/Withdraw/WithdrawQueue.sol#L279

## Vulnerability details

The RestakeManager contract allows the admin to add and remove collateral tokens that are accepted for deposits into the protocol via the addCollateralToken() and removeCollateralToken() functions.

From the protocol team:

(In order to) remove a collateral token we can withdraw full and increase the withdraw buffer to let users withdraw it.

However, since users cannot be forced to claim their withdrawals, it is always possible that a significant amount of the collateral token remains in the WithdrawQueue indefinitely in outstanding withdrawals.

In that case, calling removeCollateralToken() for that token would break accounting in the protocol. The ezETH total supply would still reflect the amounts that were burned to initiate those withdrawals, but the token balance in the WithdrawQueue would no longer be counted towards the TVL.

## Impact

The RestakeManager admin is unable to safely remove a collateral token. Removing the token anyway would inflate the ezETH mint and redeem rate compared to the actual backing collateral value.

## Recommended Mitigation Steps

Consider forcing pending withdrawals of a token to be claimed before that token can be removed as collateral. This could be done by only allowing removeCollateralToken() to be called if claimReserve[token] == 0.

Alternatively, include the balance of the WithdrawQueue in the TVL calculation even for tokens that have been removed, as long as there are still pending withdrawals of that token. This would ensure mint and redeem rates remain correct.

jatinj615 (Renzo) acknowledged and commented:

Will be partially mitigating this by having a sanity check on removeCollateralToken to revert if withdrawQueue Balance is non zero for the collateral asset getting removed.

The force kickOff will be implemented later on.

# [M-14] stETH/ETH feed being used opens up to 2 way deposit<->withdrawal arbitrage

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Finding ID:** M-14
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-04-renzo
- **Source snapshot:** competitions/2024-04-renzo/final_report.html

deposit<->withdrawal arbitrage Submitted by GalloDaSballo, also found by 0xabhay, SBSecurity, zhaojohnson, jokr, p0wd3r, peanuts, GoatedAudits, and d3e4 The stETH/ETH oracle is not a exchange rate feed, it’s a Market Rate Feed, while other feeds are exchange rate feeds.

This opens up ezETH to be vulnerable to:

Market Rate Manipulations.

Sentiment based Price Action.

Duration based discounts.

## Mitigation

I believe the withdrawal logic needs to be rethought to be denominated in ETH. The suggested architecture would look like the following:

Deposit of ETH or LSTs, estimated via a pessimistic exchange rate.

Withdraw exclusively ETH, while pricing in slashing, discounts and operative costs.

## Assessed type

Oracle jatinj615 (Renzo) acknowledged and commented:

Expected Behaviour.

alcueca (judge) commented:

It is debatable whether the market or exchange rate is the real price. The market price is the price for an instant trade, while the exchange rate is the price for a trade in the terms of the stETH contract. Renzo is not even using a real market price, which would be retrieved from a DEX. Whatever the choice of oracle, there will be some arbitrage opportunities.

alcueca (judge) commented:

Regarding PJQA here, I do actually know of other protocols working on similar topics that have recognized this as a problem and taken significant steps to avoid it.

Mitigation from sponsor on #424, along with explanation on why both groups should be merged. Namely, that using the market rate for stETH/ETH is what enables arbitraging between different collaterals, and that the fix from this finding will also fix the duplicates.

## Rejected Primary Findings

# Rejected Primary Findings: Renzo

# xRenzoDeposit::_recoverBridgeFee() always assumes the native has a wrapper, which may not be the case

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-1050
- **Submitter:** 0x73696d616f
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/1050
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-1050.md

## Brief Summary

Admin is not able to withdraw fees from `xRenzoDeposit` deposits, losing these funds forever.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, unknown, :robot:_primary, :robot:_25_group

# `ezETH` Transfer Restrictions Bypassed for Zero Address Transfers

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-855
- **Submitter:** 0xAadi
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/855
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-855.md

## Brief Summary

The `EzEthToken` contract's `_beforeTokenTransfer` function does not prevent transfers to the zero address when the contract is paused, potentially allowing for unintended token burns and bypassing transfer restrictions. Impact This issue could lead to the permanent loss of tokens if users inadvertently transfer to the zero address while the contract is paused. The impact is heightened by the expectation that all non-authorized transfers should be halted during a pause, which is not currently enforced for transfers to the zero address.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_142_group

# Missing validation in __XERC20_init function of XERC20 contract

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-566
- **Submitter:** 0xBeastBoy
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/566
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-566.md

## Brief Summary

The `__XERC20_init` function in the `XERC20` contract is responsible for initializing the contract configuration, including setting the token name, symbol, and factory address. However, there is lack of validation for the `_factory` address parameter and the use of an unsafe function `_transferOwnership` instead of transferOwnership. As `_transferOwnership` is called instead of `transferOwnership`, it doesn't validate the address sent to it. So it becomes `__XERC20_init` responsibility to do the verification before sending the address. Because there is no function for changing or setting the `FACTORY` once it is set. If it is set to zero address or incorrect one then protocol can either go...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_51_group

# Potential Risk of Exceeding Maximum Limits in XERC20:_calculateNewCurrentLimit

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-576
- **Submitter:** 0xBeastBoy
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/576
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-576.md

## Brief Summary

The `XERC20:_calculateNewCurrentLimit` function is responsible for determining the new current limit `_newCurrentLimit` based on the updated maximum limit and current limit values. However, in the else condition if the calculation `_currentLimit + _difference` results in a value more than max limit? If the calculated new current limit exceeds the maximum limit defined by the contract, it may lead to inconsistencies in limit management, contract state, or token operations, resulting in protocol limits breaching. Allowing the current limit to exceed the maximum limit poses security risks, such as potential exploitation by attackers to bypass limit restrictions, manipulate token balances, or p...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_15_group

# OperatorDelegator::stakeEth may be front-run by malicious operator to steal ETH

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-838
- **Submitter:** 0xblackskull
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/838
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-838.md

## Brief Summary

Delegated staking protocols may be exposed to a [known vulnerability](https://ethresear.ch/t/deposit-contract-exploit/6528), where a malicious operator front-runs a staker’s deposit call to the chain deposit contract and provides a different withdrawal credentials. The front-running vulnerability exposes the contract to the risk of unauthorized Ether transfers and manipulation of contract state by malicious actors. This could result in financial loss and undermine the integrity of the staking process.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_38_group

# `xRenzoBridge#sendPrice()` does not correctly query Connext's `xcall()`

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-107
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/107
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-107.md

## Brief Summary

Protocol's core functionality is flawed when considering it's integration with Connext, cause when the `PRICE_FEED_SENDER` is calling `Connext#xcall()` it instead passes in the `relayerFee` as is if it's a native value instead of passing it as an argument to the function, which then causes the context in Connext to not know that the `xcall` with the `relayerFee` integration is actually the one being queried breaking the integration.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_74_group

# xRenzoDeposit::getBridgeFeeShare() doesn't calculate correct fee on amounts greater than 32ETH

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-100
- **Submitter:** BiasedMerc
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/100
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-100.md

## Brief Summary

`xRenzoDeposit::getBridgeFeeShare()` only calculates the bridge fee up to a deposit of `sweepBatchSize` which will be some value above `32 ETH`, this means that any large deposits will not be charged the full bridging fee, losing funds for the protocol.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_24_group

# XERC20Lockbox::withdraw() can withdraw ERC20 or Native, however dev comment state otherwise

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-223
- **Submitter:** BiasedMerc
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/223
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-223.md

## Brief Summary

`XERC20Lockbox::withdraw()` states that the function allows for the withdrawal of `ERC20` tokens, however the function also allows for the withdrawal of native currency if `IS_NATIVE` is `true`. Meaning the code has been incorrectly implemented, leading to incorrect behaviour of allowing natitve withdrawals.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_32_group

# OperatorDelegator::completeQueuedWithdrawal() checks wrong withdrawal length

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-24
- **Submitter:** BiasedMerc
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/24
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-24.md

## Brief Summary

`OperatorDelegator::completeQueuedWithdrawal()` checks the incorrect `withdrawal` array length. Currently it checks `withdrawal.strategies.length` but it should be checking `withdrawal.shares.length` as this is the array that is accessed within the function's loop. This can cause an out-of-bounds revert when accessing `queuedShares[address(tokens[i])] -= withdrawal.shares[i];` as it is possible for `withdrawal.strategies` and `withdrawal.shares` to be of different lengths.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_54_group

# XERC20::mint() is callable by anyone, however comments state otherwise

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-90
- **Submitter:** BiasedMerc
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/90
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-90.md

## Brief Summary

`XERC20::mint()` dev comments for the function state that the function >Can only be called by a bridge However `XERC20::mint()` has no access control, and when following to `XERC20::_mintWithCaller()` it can be seen that anyone with minting allowance can call `mint()` successfully. This deviates from the expected behaviour from the comments on the function. It can also be seen within the repo that `RestakeManager::deposit()` also calls this function for staking, which is not a bridge.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_117_group

# RenzoOracleL2::getMintRate() reverts if price is less than 1 ETH, which can happen due to market forces

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-96
- **Submitter:** BiasedMerc
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/96
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-96.md

## Brief Summary

`RenzoOracleL2::getMintRate()` retrieves the price of `ezETH` in `ETH`, however if the price of `ezETH` is less than `1 ETH` the function will revert. The price of `1 ezETH` can be less than `1 ETH` due to market forces. At the time of writing, the [chainlink ezeth-eth](https://data.chain.link/feeds/ethereum/mainnet/ezeth-eth) oracle states that `1 ezETH` is worth `0.9868 Ether`. Meaning in the current code, the function would revert incorrectly.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_43_group

# MEV opportunity when refunding gas through `_refundGas()`

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-792
- **Submitter:** BlockSails
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/792
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-792.md

## Brief Summary

The function `_refundGas` calculates the gas refund based on `tx.gasprice`, which can be manipulated by validators or in cooperation with a malicious user. A validator or a user in collaboration with a validator could set an arbitrarily high `gasprice` for their transaction, leading to an inflated gas refund from the contract's balance. This could drain a decent amount out of the contract's funds if the balance is sufficient.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_18_group

# Chainlink oracle lookup division using hardcoded value instead of `pricefeed.decimals` may cause incorrect token look-up

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-801
- **Submitter:** BlockSails
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/801
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-801.md

## Brief Summary

The `lookupTokenValue` function divides the price by a hard-coded scale-factor of `10e18` instead of getting the actual scale factor through Chainlink's `pricefeed.decimals` function. If any token is introduced that deviates from this scaling factor, it will cause incorrect price look-ups.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_55_group

# Did Not Approve To Zero First

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-760
- **Submitter:** FastChecker
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/760
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-760.md

## Brief Summary

A number of features within the protocol will not work if the approve function reverts.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_16_group

# Lack of Proper Input Validation in recoverNative() and recoverERC20() can lead to lost of funds for users

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-1020
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/1020
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-1020.md

## Brief Summary

In the contraxt xRenzoBridge.sol, the `recoverNative` and `recoverERC20` functions do not perform proper validation of the receiver address (`_to`). If `_to` is set to a zero address (`0x0`), the tokens or Ether will be lost because they are sent to an address from which recovery is impossible (burning funds) Impact: Lost of funds for users

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_63_group

# Usage of depreciated safeApprove() function

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-1030
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/1030
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-1030.md

## Brief Summary

In the contract xRenzoDeposit.sol, the function _trade() and sweep() are using OpenZeppelin’s safeApprove() which has been documented as (1) Deprecated because of approve-like race condition and (2) To be used only for initial setting of allowance (current allowance == 0) or resetting to 0 because it reverts otherwise.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_88_group

# Inadequate Decimals Check

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-1032
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/1032
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-1032.md

## Brief Summary

The contract assumes that all tokens interfaced with have 18 decimal places. However, it does not enforce this expectation thoroughly. While the contract does check that the tokens involved in the initialization process (`initialize` function) comply with this assumption, it doesn't enforce this check elsewhere (e.g., when dealing with tokens received in other functions). Although this assumption might be considered reasonable within the Ethereum ecosystem since most tokens use 18 decimal places, there are execptions such as USDT that only have 6. If a token with mismatched decimal places is used with this contract, unexpected behavior such as incorrect token balances and transactions could...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_80_group

# Potential overflow issue in RenzoOracleL2.sol::getMintRate()

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-1041
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/1041
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-1041.md

## Brief Summary

The function `getMintRate` reads the price from an oracle and scales it to a value with 18 decimal places. This operation to adjust the decimals involves a multiplication of the price with `10 ** (18 - oracle.decimals())`, which could cause numerical overflow if not properly safeguarded. An overflow occurs when an operation tries to create a number that is outside the maximum limit that can be held by the data type. In Solidity, a `uint256` has a maximum limit of 2^256 - 1, and any calculation that exceeds this value will result in an overflow and the value will loop around to zero.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_21_group

# Unrestricted Ownership Transfer

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-1051
- **Submitter:** JC
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/1051
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-1051.md

## Brief Summary

The contract `OptimismMintableXERC20Factory` is the factory that deploys a new `OptimismMintableXERC20` token. However, it contains a dangerous and unrestricted function that allows the caller to transfer the ownership of the newly created contract. In this function Ownership of the newly created `OptimismMintableXERC20` token contract is transferred to the caller of the `_deployOptimismMintableXERC20` function, which can be any account. If an attacker can control the owner of the token contract, they can manipulate its behavior to their advantage and potentially do malicious activities such as pulling all the tokens to their account, regulating transactions, and other harmful actions.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_84_group

# DepositQueue.sol:: Lack of Input Sanitization in Stake Function

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-1056
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/1056
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-1056.md

## Brief Summary

The contract DepositQueue.sol lacks proper input sanitization in the function `stakeEthFromQueueMulti`, where arrays of input values are used. The contract trusts the caller to provide appropriate inputs, which can lead to unintended behavior if incorrect data is supplied. In the function `stakeEthFromQueueMulti`, the contract processes multiple calls to `stakeEthInOperatorDelegator` by iterating through the arrays of `operatorDelegators`, `pubkeys`, `signatures`, and `depositDataRoots` and passing each index's values to the stake function. However, it contains no validation to confirm whether the supplied data is appropriate, leading to potential risks if the data is incorrect or malicious.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_04_group

# DepositQueue.sol:: Arbitrary Spending of Tokens

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-1057
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/1057
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-1057.md

## Brief Summary

The contract DepositQueue.sol does not sufficiently validate the `_asset` address in the `fillERC20withdrawBuffer` function leading to a potential vulnerability where any troublesome token (e.g a token having re-entrancy in its `transferFrom` function) could be spent arbitrarily.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_14_group

# Use address.call() instead of address.send() to avoid denial of service

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-675
- **Submitter:** Kaysoft
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/675
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-675.md

## Brief Summary

Denial of service for receipients with fallback/receive functions that consumes more than 2300 gas.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_53_group

# Centralization risk in `DepositQueue`

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-826
- **Submitter:** MaslarovK
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/826
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-826.md

## Brief Summary

The `DepositQueue` allows the trusted role to set the fee up to 100%, potentially increasing the centralization risk Centralization risk should be a big concern and prevented at all costs.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_119_group

# Failure to initialize after disabling the initializer

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-1027
- **Submitter:** Mylifechangefast_eth
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/1027
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-1027.md

## Brief Summary

In the optimism contract, they were planning to use the initialize imported contract but they didn't

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_48_group

# Deposit and DepositETH needs some validations for protection from slippage attack

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-424
- **Submitter:** Mylifechangefast_eth
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/424
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-424.md

## Brief Summary

When making swaps, some validation needs to be checked to protect the contract from being sandwiched or prone to slippage attacks. Every deposit in both deposit and depositETH made will be prone to slippage attacks because of no check for that.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_90_group

# Initialization functions can be front-run

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-952
- **Submitter:** Mylifechangefast_eth
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/952
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-952.md

## Brief Summary

Several implementation contracts have initialize functions that can be front-run, allowing an attacker to incorrectly initialize the contracts. If the front-running of one of these functions is not detected immediately, an attacker may be able to steal funds at a later time. Attacker Eve has studied the next version of the renzo protocol and identified several parameters of initialization functions that, if set to certain values, will allow her to steal funds from the protocol. She sets up a script to automatically watch the mempool and front-run the initialize functions of the next renzo protocol deployment. Bob, a developer, deploys the next version of the 88mph protocol. Eve’s script fro...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_116_group

# `RestakeManager::depositETH()` always assumes 1:1 peg with ETH

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-589
- **Submitter:** NentoR
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/589
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-589.md

## Brief Summary

`RestakeManager::deposit()` and `RestakeManager::depositETH()` calculate amounts to be minted differently. The first one uses the price of the deposited asset whereas the second one the amount of ether sent. This can lead to incorrect accounting when the price of `ezETH` is not aligned with `ETH`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_101_group

# Ambiguous abi encoding in OptimismMintableXERC20 factory leading to salt collision and deployment DOS for same user deployment

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-800
- **Submitter:** ReadyPlayer2
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/800
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-800.md

## Brief Summary

Ambiguous abi encoding of the salt in the _deployOptimismMintableXERC20 function leads to a hash calculation collision, which in turn leads to a collision in create3 contract deployment for the same user. This will be a major problem for users that wish to use the factory to deploy multiple tokens that could most likely have ambiguous names and symbols as a hash collision will also mean contract deployment collision.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_223_group

# TimelockController's `_minDelay` can be set to an arbitrarily high value, potentially rendering the timelock unusable.

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-105
- **Submitter:** Rhaydden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/105
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-105.md

## Brief Summary

If the `_minDelay` is set to a very high value, it could make the timelock unusable, as any new operation would require an extremely long delay before it can be executed. This could lead to a situation where the controlled contract becomes stuck and unable to perform important administrative actions, such as updating critical parameters or addresses.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_89_group

# TimelockController could be used to allow anyone execute proposals

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-547
- **Submitter:** Rhaydden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/547
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-547.md

## Brief Summary

The `TimelockController` contract allows the zero address (`address(0)`) to be granted the `EXECUTOR_ROLE`. If this occurs, either intentionally or by mistake, any address can execute operations without explicitly having the `EXECUTOR_ROLE` assigned to them. This could lead to unauthorized execution of sensitive operations, potentially compromising the security of the contract and any dependent systems.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_61_group

# CCIP router cannot be updated

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-220
- **Submitter:** RootKit0xCE
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/220
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-220.md

## Brief Summary

CCIP Router addresses cannot be updated in [Receiver](https://github.com/code-423n4/2024-04-renzo/blob/main/contracts/Bridge/L2/PriceFeed/CCIPReceiver.sol#L14) On contracts that inherit from CCIPReceiver, router addresses need to be updateable. Chainlink may update the router addresses as they did before. This issue introduces a single point of failure that is outside of the protocol's control. [an example from Chainlink](https://github.com/smartcontractkit/ccip-tic-tac-toe/blob/main/contracts/TTTDemo.sol#L81-L83) this example is created by Chainlink [CCIP Tic Tac Toa Example](https://docs.chain.link/ccip/examples#ccip-tic-tac-toe) [Chainlink documents noticing users about router address up...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_150_group

# WithdrawQueue's lack of expiration can be used to force the withdrawal more from EigenLayer

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-226
- **Submitter:** SBSecurity
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/226
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-226.md

## Brief Summary

Withdraw requests lack expiration and can stay forever if the user doesn’t call `claim()`, causing the withdraw buffer to be lower which forces withdraws from EigenLayer.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_92_group

# setFeeConfig allows setting feeAddress to zero address when feeBasisPoints is 0

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-326
- **Submitter:** Sabit
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/326
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-326.md

## Brief Summary

setFeeConfig allows setting feeAddress to zero address when feeBasisPoints is 0

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_208_group

# [H-06] `xRenzoDeposit::deposit` - Lack of Replay Attack Protection could lead to Draining of Funds

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-179
- **Submitter:** Squilliam
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/179
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-179.md

## Brief Summary

The `deposit` function in `xRenzoDeposit.sol` does not have any mechanisms in place to prevent the replay of user signatures across different chains. This vulnerability could allow an attacker to steal funds by replaying a victim's signature on a different chain. This vulnerability allows an attacker to directly steal user funds by replaying the victim's failed transactions. The financial impact can be significant, as an attacker can potentially steal large amounts of user funds by exploiting this vulnerability across multiple victims.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_94_group

# [H-10] `xRenzoBridge::recoverERC20` allows admins to Rug-Pull an Arbitrary Amount of ERC20 tokens from Users, leaving the protocol insolvent

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-183
- **Submitter:** Squilliam
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/183
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-183.md

## Brief Summary

The `recoverERC20` function in `xRenzoBridge` provides admins with the ability to rugpull as much ERC20 tokens as they want. If they decide to rug-pull, they can, leaving the protocol totally insolvent.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_164_group

# [M-5] `OperatorDelegator::queueWithdrawals` - Unbounded Loop can lead to Denial of Service (reuploaded)

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-273
- **Submitter:** Squilliam
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/273
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-273.md

## Brief Summary

(I am reuploading this vulnerability because I accidently withdrew the previous one when I went to update it.) The `queueWithdrawals` function in the `OperatorDelegator` contract contains an unbounded loop that iterates through the entire array of tokens and token amounts. If the array is large, it can consume a significant amount of gas, which can lead to a Denial-of-Service (DoS) condition. An Admin can provide a large array of tokens and/or token amounts when calling the `queueWithdrawals` function, causing the transaction to consume an excessive amount of gas. This can result in the transaction exceeding the block gas limit and reverting, making the contract unusable or blocking other t...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_01_group

# [H-14] `ConnextReceiver::xReceive` is vulnerable to cross-chain replay attacks, allowing attackers to steal funds

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-282
- **Submitter:** Squilliam
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/282
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-282.md

## Brief Summary

the `xReceive` function in the `ConnextReceiver.sol` contract does not have any mechanism to validate the origin of the Connext message, leaving it vulnerable to replay attacks across different chains. An attacker could intercept a valid Connext message on one chain and replay it on a different chain, resulting in unauthorized actions, such as updating the price feed for the attackers benefit or triggering other sensitive operations. This could lead to the loss of user funds and compromise the integrity of the cross-chain functionality.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_151_group

# [M-17] `xRenzoDeposit::sweep()`: Ignoring Return Values can lead to loss of user funds.

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-318
- **Submitter:** Squilliam
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/318
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-318.md

## Brief Summary

The `xRenzoDeposit::sweep()` function ignores the return value of the `connext.xcall()` call, which sends a cross-chain message to the destination chain. Ignoring the return value of the `connext.xcall()` call means that the function will continue executing even if the message sending fails. This could lead to the following issues: Inconsistent Protocol State: If the cross-chain message fails to be delivered, the protocol's state may become inconsistent, as the `xRenzoDeposit` contract would have processed the sweep operation locally, but the corresponding action may not have been executed on the destination chain. Loss of User Funds: If the cross-chain message fails, the user's deposited f...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_132_group

# [M-24] `withdrawQueue::withdraw` - Lack of Slippage Protection creates an opportunity for attackers to Sandwich Attack users, leading to financial losses for the users.

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-542
- **Submitter:** Squilliam
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/542
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-542.md

## Brief Summary

The `withdraw` function in the `withdrawQueue` contract allows users to request a withdrawal of their ezETH tokens. However, the function lacks any form of slippage protection, exposing users to potential sandwich attacks. An attacker can monitor the mempool for pending withdraw transactions, execute their own transactions before and after the victim's transaction, and manipulate the redemption amount in their favor. The absence of slippage protection in the `withdraw` function can lead to users receiving less tokens than expected during the withdrawal process. Attackers can exploit this vulnerability to extract value from users' withdrawals by strategically placing their own transactions b...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_113_group

# [M-23] `RenzoOracle::lookupTokenValue` - Lack of validation for rollup sequencer leading to stale prices and indirect fund risk

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-567
- **Submitter:** Squilliam
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/567
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-567.md

## Brief Summary

The `lookupTokenValue` function in the `RenzoOracle` contract does not include any explicit validation to check if the rollup sequencer is running. If the rollup sequencer goes offline, it could lead to stale prices being used by the protocol, potentially resulting in mispricing of assets and indirect risk to funds. If the rollup sequencer is offline and stale prices are used, it can lead to mispricing of assets and indirect risk to funds. The impact of this vulnerability can manifest in several ways: Mispricing of assets: If the protocol relies on stale prices, it may lead to incorrect valuation of assets, causing users to make suboptimal trading or investment decisions. Indirect financial...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_112_group

# The Invariant That a Token's TVL in the Protocol Be Within the Set Limit Can Be Broken by an Admin Action

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-236
- **Submitter:** Tendency
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/236
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-236.md

## Brief Summary

[DepositQueue::SweepERC20](https://github.com/code-423n4/2024-04-renzo/blob/519e518f2d8dec9acf6482b84a181e403070d22d/contracts/Deposits/DepositQueue.sol#L254-L277) function is to be called by an admin to sweep stuck tokens in the `DepositQueue` to the set token's Eigen layer strategy manager. Here is the call path: ` RestakeManager::depositTokenRewardsFromProtocol --> OperatorDelegator::deposit --> StrategyManager::depositIntoStrategy ` The problem here is that, the system currently uses a limit system that intends to limit the total value locked, and each collateral token's total value locked to an admin set value. + To Illustrate: If for example the tvl limit for stETH has been set to 500...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_115_group

# Fee-loss is incurred during `xezETH` minting

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-103
- **Submitter:** Tigerfrake
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/103
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-103.md

## Brief Summary

`Fee-loss` arises from the incorrect handling of the `bridgeFee` deduction in the `_deposit()` function. Impact The protocol gets no `bridgeFee` at all as the whole `_amountIn` is traded for `nextWETH` for the user. In other words, the user doesn't pay any `bridgeFee` for their deposited tokens.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_41_group

# RewardHandler::forwardRewards() is not payable

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-99
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/99
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-99.md

## Brief Summary

`RewardHandler::forwardRewards()` lacks `payable` modifier. If `value` is not zero, it could always revert. Impact `RewardHandler::forwardRewards()` cannot work if `value != 0`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_118_group

# Potential Front-Running Vulnerability in Lockbox Deployment

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-383
- **Submitter:** Topmark
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/383
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-383.md

## Brief Summary

The predictability of the _salt used in the _deployLockbox function could potentially expose the XERC20Factory contract to front-running attacks. In this attack, a malicious actor could anticipate the outcome of the function call, pre-compute the address of the lockbox, and interact with it before the legitimate transaction is confirmed. This could lead to unauthorized access or manipulation of the deployed lockbox contract, compromising the integrity and security of the ptotocol.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_195_group

# in RenzoOracleL2 , wrong check will suscept the contract to wrong calculations

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-966
- **Submitter:** WildSniper
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/966
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-966.md

## Brief Summary

in RenzoOracleL2 , wrong check will suscept the contract to wrong calculations

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_140_group

# `burn` function in OptimismMintableXERC20 contract should not burn token from `_from` but from `msg.sender`

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-190
- **Submitter:** ZanyBonzy
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/190
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-190.md

## Brief Summary

The `burn` function in OptimismMintableXERC20.sol burns from `_from` and not `msg.sender` which causes that malicious users can burn any tokens in the contract or from other users that have any unspent allowance in the contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_12_group

# RestakeManager - calculateTVLs() returns the empty `operatorDelegatorTokenTVLs` array

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-892
- **Submitter:** ak1
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/892
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-892.md

## Brief Summary

Since the `calculateTVLs()` returns the empty `operatorDelegatorTokenTVLs`, it affects the RestakeManager's deposit logic. Especially, the following check would be bypassed since the value is zero always. [RestakeManager.sol#L528-L530](https://github.com/code-423n4/2024-04-renzo/blob/519e518f2d8dec9acf6482b84a181e403070d22d/contracts/RestakeManager.sol#L528-L530)

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_222_group

# LockboxAdapterBlast : bridgeTo could be used to drain the contract balance by sending the large _extraData

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-925
- **Submitter:** ak1
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/925
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-925.md

## Brief Summary

The balance of [LockboxAdapterBlast](https://github.com/code-423n4/2024-04-renzo/blob/519e518f2d8dec9acf6482b84a181e403070d22d/contracts/Bridge/Connext/integration/LockboxAdapterBlast.sol#L56-L95) could be drained by specifying large extra data.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_85_group

# Griefing attack against the `depositIntoStrategy` function

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-723
- **Submitter:** alphacipher
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/723
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-723.md

## Brief Summary

The griefing attack delays the staking process, potentially causing the node to miss out on rewards or incur additional transaction costs. It also disrupts the smooth operation of the staking system.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_35_group

# RewardHandler's ETH balance is not accounted in TVL, enabling sandwich attacks on pending rewards

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-168
- **Submitter:** aslanbek
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/168
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-168.md

## Brief Summary

`RestakeManager#calculateTVLs` is used to retrieve total value of assets in the system, denominated in ETH. However, its logic does not include RewardHandler's balance, making the ezETH price smaller than it should be (until rewards are forwarded), and enabling sandwich attacks.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_216_group

# Inconsistent use of upgradable versions in oppenzeppelin Imports

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-141
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/141
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-141.md

## Brief Summary

The `XERC20Lockbox` contract imports non-upgradeable contracts from `openzeppelin`, which may limit the contract's upgradability and compatibility with upgradeable systems. While the contract itself inherits from `Initializable`, ensuring proper initialization, the non-upgradeable imports could hinder seamless upgrades and integration with other upgradeable contracts.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_188_group

# Missing Import of Initializable in Upgradeable Contract

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-142
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/142
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-142.md

## Brief Summary

The `OperatorDelegator` contract upgradeability is hindered due to the absence of the `Initializable` contract import. This prevents the proper use of the initializer function, which is essential for initializing upgradeable contracts. Without proper initialization, the contract may exhibit unexpected behavior during deployment or upgrade, potentially leading to security vulnerabilities and instability.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_237_group

# Timelock Controller does not add all supported interfaces in supportsInterface()

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-412
- **Submitter:** b0g0
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/412
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-412.md

## Brief Summary

Timelock Controller contract implements both the `onERC721Received` & `onERC1155Received` methods to handle safeTransfer calls to it. However the `supportsInterface()` function looks like this: Only the `IERC1155Receiver` interface is defined, while the `IERC721Receiver` is missing. This breaks the contract composability and prevents other contract calling the function from verifying that the contract supports the interface for receiving ERC721 tokens.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_214_group

# Storage collision can brick token minting on Optimism

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-574
- **Submitter:** b0g0
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/574
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-574.md

## Brief Summary

`OptimismMintableXERC20` storage variables will be overridden in case its parent contract `XERC20` is upgraded. Vulnerability details In order to better handle tokens bridging to different L2s, the protocol employs [the XERC20 standard, designed to make the process easier and more reliable](https://hackmd.io/@arjunbhuptani/xerc20-bridge-spec). [The `XERC20.sol` contract](https://github.com/code-423n4/2024-04-renzo/blob/519e518f2d8dec9acf6482b84a181e403070d22d/contracts/Bridge/xERC20/contracts/XERC20.sol#L16) has been modified to use the upgradeability pattern of OpenZeppelin. Additionally a separate version has been created to be used on Optimism L2, called `OptimismMintableXERC20`. It inhe...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_158_group

# "_deposit" will not work for all tokens in "XERC20Lockbox"

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-414
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/414
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-414.md

## Brief Summary

Detailed description of the impact of this finding. here we are not tracking the native token in XERC20Lockbox.sol. in deposit function when our token is "NATIVE" then we do not know whether we are depositing token or not.There are no way of tracking the NATIVE token.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_73_group

# In WithdrawQueue, an address could not claim its assets if it got blacklisted during the cooldown period.

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-154
- **Submitter:** blutorque
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/154
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-154.md

## Brief Summary

Renzo does support a blacklist token, e.g., wBETH. If a user is added to the blacklist during the cooldown period, they will not be able to claim wBETH, as it is transferred back to the same blacklisted address.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_30_group

# `RenzoOracleL2.getMinRate()`: Lack of sequencer check

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-847
- **Submitter:** carlitox477
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/847
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-847.md

## Brief Summary

Chainlink recommends that all Optimistic L2 oracles consult the Sequencer Uptime Feed to ensure that the sequencer is live before trusting the data returned by the oracle. This check is not implemented in `RenzoOracleL2.getMinRate()` When utilizing Chainlink in L2 chains like Arbitrum, it's important to ensure that the prices provided are not falsely perceived as fresh, even when the sequencer is down. Impact Use of stale price in case of sequencer down in L2 chains like arbitrum

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_69_group

# Return values of `approve()` not checked

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-288
- **Submitter:** codeslide
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/288
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-288.md

## Brief Summary

Not all IERC20 implementations `revert()` when there is a failure in `approve()`. The function signature has a `boolean` return value and the function indicates an error by returning `false` instead of reverting. By not checking the return value, operations that should have failed may potentially go through without actually approving anything.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_152_group

# `xRenzoDeposit::deposit` calculation for converting `nextWETH` to `xezETH` is incorrect

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-276
- **Submitter:** crypticdefense
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/276
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-276.md

## Brief Summary

`xRenzoDeposit::deposit` allows users to deposit tokens in exchange for `xezETH`. Due to an error regarding conversion from `nextWETH` to `xezETH` during deposit, users will be minted an incorrect amount of `xezETH`, likely much less than actually owed.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_52_group

# `WithdrawQueue::claim` can be sandwich attacked

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-418
- **Submitter:** crypticdefense
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/418
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-418.md

## Brief Summary

`WithdrawQueue::claim` allows users to claim their withdrawal request after `coolDownPeriod` has passed. The user's `ezETH` is burned and they are sent `collateralToken` asset. An attacker can front-run this transaction and call `RestakeManager::deposit` to mint them `ezETH` for collateral. When the user's `claim` is executed, the attacker can call `WithdrawQueue::withdraw` to burn their `ezETH` for the collateral, effectively sandwich attacking the call and profiting.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_79_group

# Users can control the price of ezETH through donation attack

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-431
- **Submitter:** cu5t0mpeo
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/431
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-431.md

## Brief Summary

The price of ezETH is susceptible to manipulation, which can result in user losses or prevent deposits to L2.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_86_group

# When the feeBasisPoints value is 10000, the sweepERC20 function cannot be called.

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-435
- **Submitter:** cu5t0mpeo
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/435
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-435.md

## Brief Summary

The sweepERC20 function will not run properly.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_98_group

# Missing Access control to destination chain, which may cause lost of funds

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-366
- **Submitter:** eeshenggoh
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/366
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-366.md

## Brief Summary

The `sendPrice` & `xReceive` functions is used to send the price feed and take all collateral and deposit it into Renzo to the L1. Both functions however are NOT protected by sending funds to wrong destination whitelisted chain. Impact Funds will be lost when calling the function with unauthorized destination chain

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_47_group

# Potential for draining the contract's funds by repeatedly calling the `_execute` function with different addresses.

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-891
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/891
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-891.md

## Brief Summary

[target.call{value: value}(data)](https://github.com/code-423n4/2024-04-renzo/blob/519e518f2d8dec9acf6482b84a181e403070d22d/contracts/TimelockController.sol#L369) line is sending value amount of Ether to the address specified by the target variable, which can be any address. This is considered a security risk because an attacker could potentially drain the contract's funds by calling this function repeatedly with different addresses. Vulnerability Details In the line [(bool success, ) = target.call{ value: value }(data);](https://github.com/code-423n4/2024-04-renzo/blob/519e518f2d8dec9acf6482b84a181e403070d22d/contracts/TimelockController.sol#L369). This line sends value amount of Ether to...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_46_group

# RewardHandler's receive function reduces MEV yield

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-783
- **Submitter:** guhu95
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/783
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-783.md

## Brief Summary

MEV bribes (direct `block.coinbase` payments) that execute [the `RewardHandler`'s `receive`](https://github.com/code-423n4/2024-04-renzo/blob/main/contracts/Rewards/RewardHandler.sol#L52-L54) function [call the `DepositQueue`](https://github.com/code-423n4/2024-04-renzo/blob/main/contracts/Rewards/RewardHandler.sol#L12-L13), and in its [`receive` function](https://github.com/code-423n4/2024-04-renzo/blob/main/contracts/Deposits/DepositQueue.sol#L158-L183), it calls `feeAddress`, and [`WithdrawQueue`'s `getBufferDeficit` and `fillEthWithdrawBuffer`](https://github.com/code-423n4/2024-04-renzo/blob/main/contracts/Deposits/DepositQueue.sol#L296-L302), and [increments `totalEarned`](https://git...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_65_group

# Missing Initialization of ReentrancyGuardUpgradeable In WIthdrawQueue Would Brick All The Reentrancy Protection And Break The Code Consistency

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-974
- **Submitter:** ihtishamsudo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/974
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-974.md

## Brief Summary

[WithdrawQueue](https://github.com/code-423n4/2024-04-renzo/blob/519e518f2d8dec9acf6482b84a181e403070d22d/contracts/Withdraw/WithdrawQueue.sol#L11) contract inherits [PausableUpgradeable](https://github.com/code-423n4/2024-04-renzo/blob/519e518f2d8dec9acf6482b84a181e403070d22d/contracts/Withdraw/WithdrawQueue.sol#L13) & [ReentrancyGuardUpgradeable](https://github.com/code-423n4/2024-04-renzo/blob/519e518f2d8dec9acf6482b84a181e403070d22d/contracts/Withdraw/WithdrawQueue.sol#L14) upgradable contracts and it's invoking only [PausableUpgradeable](https://github.com/code-423n4/2024-04-renzo/blob/519e518f2d8dec9acf6482b84a181e403070d22d/contracts/Withdraw/WithdrawQueue.sol#L13) initializer in its...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_233_group

# Collateral Token not included in List for Minting ezETH can be used for staking in the Operator Delegator

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-1019
- **Submitter:** inzinko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/1019
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-1019.md

## Brief Summary

When The Protocol receives rewards for staking it is sent to the `DepositQueue`, and it is used to stake back in the operator delegator, but the problem here is that the rewards sent to the contract are different ERC20 tokens that may have strategies on the eigen layer, but may not have being added to the collateral token list in the `RestakeManager`, which means Temporary DOS for any process that involves those tokens on the `RestakeManager`

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_169_group

# Lack of Input Validation in sendPrice Function

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-304
- **Submitter:** kaveyjoe
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/304
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-304.md

## Brief Summary

The sendPrice function takes two parameters: _destinationParam and _connextDestinationParam. These are arrays containing the details of the destination chains and the addresses of the corresponding receivers on those chains. The function does not validate the contents of these arrays before processing them, which means that any data passed into the function is used as-is. Impact If the arrays contain malformed or malicious data, this could result in failed transactions, incorrect exchange rates being sent, or exploitation of the contract's functionality. This could undermine the integrity of the price feed and potentially lead to financial losses or reputational damage.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_131_group

# Malicious proxyAdmin can upgrade xerc20 token implementation and steal fund from user.

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-770
- **Submitter:** ladboy233
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/770
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-770.md

## Brief Summary

Malicious proxyAdmin can upgrade xerc20 token implementation and steal fund from user.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_174_group

# Withdrawal request is not handled correctly upon claiming

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-945
- **Submitter:** m_Rassska
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/945
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-945.md

## Brief Summary

* In order to keep track of the user's withdrawal requests, the mapping is used, where the key is a user, and the value - `withdrawRequestIndex`, which itself holds a request. After claiming the request, it should be removed from the mapping to avoid a double claim. Currently, it's done in the following way: * However, the system assumes that the `withdrawRequests[msg.sender][withdrawRequests[msg.sender].length - 1]` will retrieve the last requested withdrawal, which is not true. In fact, it might be an empty slot, since the `withdrawRequestIndex` is not tied to a specific user. Impact * There is a possibility to accidentally remove two separate requests by only claiming one of them.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_58_group

# OperatorDelegator._refundGas() does not refund to the admin i.e. `onlyNativeEthRestakeAdmin`

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-703
- **Submitter:** mussucal
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/703
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-703.md

## Brief Summary

All protocol ETH rewards go back to `depositQueue` to be restaked without paying back the gas costs to admin.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_03_group

# Failed ERC20 transfer inside `claim()` results in permanent loss of funds

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-54
- **Submitter:** t0x1c
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/54
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-54.md

## Brief Summary

Although this issue has been mentioned in the [automated findings](https://github.com/code-423n4/2024-04-renzo/blob/main/4naly3er-report.md#m-9-return-values-of-transfertransferfrom-not-checked), I believe it warrants a clear mention here due to the impact being loss of funds with no way for the user to retry the transaction. <br> The `claim()` function [uses the transfer()](https://github.com/code-423n4/2024-04-renzo/blob/main/contracts/Withdraw/WithdrawQueue.sol#L305) function from the [OZ IERC20 interface](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/token/ERC20/IERC20.sol#L34-L41) but never checks it's return value to see if it executed successfully or no...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_91_group

# It is not possible to completely remove an operator delegator from restaking manager without changing ezETH exchange rate

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-137
- **Submitter:** tapir
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/137
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-137.md

## Brief Summary

Completely removing an operator delegator can be impossible because if the completed withdrawal amount is bigger than the deficit then the excess will be redeposited which in case of a migration or off boarding an operator delegator this behaviour would not be correct. The excess being redeposited to the operator delegator makes the restaking manager admin removing the operator not possible. If the admin does that regardless, the excess TVL will also be scraped hence, the TVL will change and exchange rate will drop significantly.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_50_group

# Access control implemented in WithdrawQueue's `fillEthWithdrawBuffer method is Inconsistent with Natspec

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-161
- **Submitter:** umarkhatab_465
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/161
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-161.md

## Brief Summary

The DocString/Natspec of the method `fillEthWithdrawBuffer` states that it is access controlled by the Restake manager - only restake manager is able to call it with access control imposed by `` but the modifier used is `onlyDepositQueue` which ensures only the deposit Queue contract will be able to call this method . But this check given in Natspec is not really enforced

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_67_group

# Missing token vaules in withdrawqueue when checking `collateralTokenTvlLimits`

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-292
- **Submitter:** zhaojohnson
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/292
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-292.md

## Brief Summary

Collateral tokens' value might exceed the collateral's limit `collateralTokenTvlLimits`

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_78_group

# Improper share price calculation in calculateRedeemAmount()

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-320
- **Submitter:** zhaojohnson
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/320
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-320.md

## Brief Summary

Depositors may earn more or less profit than expected. Even some profits will be locked in contract and nobody can claim them.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_22_group

# Possible claim() failure because of out of gas.

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-321
- **Submitter:** zhaojohnson
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/321
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-321.md

## Brief Summary

Users may claim failure because of out of gas and users' funds are locked in the contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_23_group

# Deposit fees are taken two times in `xRenzoDeposit` even without trade

- **Contest:** Renzo
- **Slug:** 2024-04-renzo
- **Submission:** V-322
- **Submitter:** zigtur
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-04-renzo-validation/issues/322
- **Source snapshot:** competitions/2024-04-renzo/submissions/raw/V-322.md

## Brief Summary

Fees are taken twice from users, bypassing the 1% limit for fees.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_177_group
