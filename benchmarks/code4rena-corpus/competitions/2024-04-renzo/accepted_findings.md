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
