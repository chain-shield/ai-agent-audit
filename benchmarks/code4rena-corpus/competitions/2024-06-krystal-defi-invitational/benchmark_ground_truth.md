# Benchmark Ground Truth: Krystal DeFi Invitational

## Accepted H/M Findings

# Accepted H/M Findings: Krystal DeFi Invitational

# [M-01] Wrong logic in AUTO_COMPOUND doesn’t allow for swap to token1

- **Contest:** Krystal DeFi Invitational
- **Slug:** 2024-06-krystal-defi-invitational
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-06-krystal-defi-invitational
- **Source snapshot:** competitions/2024-06-krystal-defi-invitational/final_report.html

AUTO_COMPOUND doesn’t allow for swap to token1 Submitted by Dup1337 Protocol functionality broken

## Recommended Mitigation Steps

} else if (params.action == Action.AUTO_COMPOUND) { if (params.targetToken == state.token0) { _swapAndIncrease(SwapAndIncreaseLiquidityParams(params.protocol, params.nfpm, params.tokenId, state.amount0, state.amount1, 0, positionOwner, params.deadline, IERC20(state.token1), params.amountIn1, params.amountOut1Min, params.swapData1, 0, 0, bytes(""), params.amountAddMin0, params.amountAddMin1, 0), IERC20(state.token0), IERC20(state.token1), false); - } else if (state.token0 == state.token1) { + } else if (params.targetToken == state.token1) { _swapAndIncrease(SwapAndIncreaseLiquidityParams(params.protocol, params.nfpm, params.tokenId, state.amount0, state.amount1, 0, positionOwner, params.deadline, IERC20(state.token0), 0, 0, bytes(""), params.amountIn0, params.amountOut0Min, params.swapData0, params.amountAddMin0, params.amountAddMin1, 0), IERC20(state.token0), IERC20(state.token1), false);

3docSec (judge) decreased severity to Medium and commented:

The finding completely misses a justification for High severity. A swap not happening as it should is better categorized as Medium.

Haupc (Krystal DeFi) confirmed

# [M-02] The signatures are replayable

- **Contest:** Krystal DeFi Invitational
- **Slug:** 2024-06-krystal-defi-invitational
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-06-krystal-defi-invitational
- **Source snapshot:** competitions/2024-06-krystal-defi-invitational/final_report.html

Submitted by Dup1337, also found by SpicyMeatball

- https://github.com/code-423n4/2024-06-krystal-defi/blob/f65b381b258290653fa638019a5a134c4ef90ba8/src/V3Automation.sol#L79-L81
- https://github.com/code-423n4/2024-06-krystal-defi/blob/f65b381b258290653fa638019a5a134c4ef90ba8/src/StructHash.sol#L274-L296

## Impact

User signed orders can be replayed

## Recommended Mitigation Steps

Introduce nonce and verification that operator parameters are the same that the user signed.

3docSec (judge) commented:

Confirming as Medium. There is the concrete possibility of fund loss, however, the onlyRole(OPERATOR_ROLE) privilege required to exploit it mitigates the risk.

namnm1991 (Krystal DeFi) acknowledged

# [M-03] _deductFees() is incompatible with tokens that revert on zero value transfers

- **Contest:** Krystal DeFi Invitational
- **Slug:** 2024-06-krystal-defi-invitational
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-06-krystal-defi-invitational
- **Source snapshot:** competitions/2024-06-krystal-defi-invitational/final_report.html

_deductFees() is incompatible with tokens that revert on zero value transfers Submitted by d3e4 All main functionality risks reverting with tokens that revert on zero value transfers, via a transfer in Common._deductFees().

## Recommended Mitigation Steps

if ( feeAmount0 > 0 ) { SafeERC20.

safeTransfer ( IERC20 ( params.

token0 ), FEE_TAKER, feeAmount0 ); } etc.

## Assessed type

ERC20 Haupc (Krystal DeFi) confirmed

# [M-04] The Protocol breaks the Allowance Mechanism of the NFTs

- **Contest:** Krystal DeFi Invitational
- **Slug:** 2024-06-krystal-defi-invitational
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-06-krystal-defi-invitational
- **Source snapshot:** competitions/2024-06-krystal-defi-invitational/final_report.html

Submitted by Dup1337

- https://github.com/code-423n4/2024-06-krystal-defi/blob/f65b381b258290653fa638019a5a134c4ef90ba8/src/Common.sol#L392
- https://github.com/code-423n4/2024-06-krystal-defi/blob/f65b381b258290653fa638019a5a134c4ef90ba8/src/V3Utils.sol#L76-L85
- https://github.com/code-423n4/2024-06-krystal-defi/blob/f65b381b258290653fa638019a5a134c4ef90ba8/src/V3Utils.sol#L171
- https://github.com/code-423n4/2024-06-krystal-defi/blob/f65b381b258290653fa638019a5a134c4ef90ba8/src/V3Automation.sol#L92

## Impact

The user loses their approved entities

## Recommended Mitigation Steps

NFPM uses isAuthorizedForToken modifier as below:

Contract:

NonfungiblePositionManager.

sol 183:

modifier isAuthorizedForToken ( uint256 tokenId ) { 184:

require ( _isApprovedOrOwner ( msg.

sender, tokenId ), 'Not approved' ); 185:

_; 186: } Take approval of token owners rather than using onERC721Received hook.

Haupc (Krystal DeFi) acknowledged and commented:

There are 2 ways to take approval from user via approve function => this function also discards previous approval. So we can not keep other’s approval.

via setApprovalForAll function => this function seems too risky for user to use quanghuy219 (Krystal DeFi) commented:

ERC721 implementation allows only one spender on a token at a time, therefore taking user’s approval for our contract will also clear other approval on the same token mapping ( uint256 tokenId => address ) private _tokenApprovals; Another approach is to use setApprovalForAll function in ERC721, which poses another concern of allowing our contract to use all user’s positions.

With that in mind, we think that using onERC721Received hook is still the safest option for our users and Krystal is able to provide convenient user experience.

# [M-05] Swapping logic would be broken for some supported tokens

- **Contest:** Krystal DeFi Invitational
- **Slug:** 2024-06-krystal-defi-invitational
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-06-krystal-defi-invitational
- **Source snapshot:** competitions/2024-06-krystal-defi-invitational/final_report.html

Submitted by Bauchibred, also found by SpicyMeatball and d3e4 First take a look at this excerpt from the README:

- https://github.com/code-423n4/2024-06-krystal-defi/blob/f65b381b258290653fa638019a5a134c4ef90ba8/README.md#L103-L106
| Question | Answer | | ---------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------- | | Chains the protocol will be deployed on | Ethereum,Arbitrum,Base,BSC,Optimism,Polygon | | ---------------------------------------------------------------------------------------------------------------------------------------------------------- | ------ | | [ Revert on zero value approvals ](

- https://github.com/d-xo/weird-erc20?tab=readme-ov-file#revert-on-zero-value-approvals
) | Yes | From the above we can conclude that tokens like BNB that revert on zero value approvals are to be integrated within protocol.

Now take a look at

- https://github.com/code-423n4/2024-06-krystal-defi/blob/f65b381b258290653fa638019a5a134c4ef90ba8/src/Common.sol#L537-L567
function _swap ( IERC20 tokenIn, IERC20 tokenOut, uint256 amountIn, uint256 amountOutMin, bytes memory swapData ) internal returns ( uint256 amountInDelta, uint256 amountOutDelta ) { if ( amountIn != 0 && swapData.

length != 0 && address ( tokenOut ) != address ( 0 )) { uint256 balanceInBefore = tokenIn.

balanceOf ( address ( this )); uint256 balanceOutBefore = tokenOut.

balanceOf ( address ( this )); // approve needed amount _safeApprove ( tokenIn, swapRouter, amountIn ); // execute swap ( bool success,) = swapRouter.

call ( swapData ); if (!

success ) { revert ( "swap failed!" ); } // reset approval //@audit resetting the approval would never work for these tokens _safeApprove ( tokenIn, swapRouter, 0 ); uint256 balanceInAfter = tokenIn.

balanceOf ( address ( this )); uint256 balanceOutAfter = tokenOut.

balanceOf ( address ( this )); amountInDelta = balanceInBefore - balanceInAfter; amountOutDelta = balanceOutAfter - balanceOutBefore; // amountMin slippage check if ( amountOutDelta < amountOutMin ) { revert SlippageError (); } // event for any swap with exact swapped value emit Swap ( address ( tokenIn ), address ( tokenOut ), amountInDelta, amountOutDelta ); } This is the general swap function that eventually gets called which uses the external router with off-chain calculated swap instruction. The issue, however, is that after approving the initial amount needed to the router, there is a need to reset these approvals, however resetting these approvals whereas would work for most tokens would not work for a token like BNB that’s to be supported,

## Impact

As hinted above, swaps would be completely broken for tokens that revert on zero value approvals, due to a reversion that always occurs on this line.

## Recommended Mitigation Steps

Consider try/catching the attempt to safeApprove in _swap() and in the case it reverts, query _safeResetAndApprove() instead. Alternatively, do not support tokens that revert on zero value approvals.

## Assessed type

Context Haupc (Krystal DeFi) confirmed

## Rejected Primary Findings

# Rejected Primary Findings: Krystal DeFi Invitational

No rejected primary findings were captured for this contest in the current corpus.

- **Rejected primary count:** 0
- **Primary submission rows captured:** 0
- **Submission status:** requires_authentication
