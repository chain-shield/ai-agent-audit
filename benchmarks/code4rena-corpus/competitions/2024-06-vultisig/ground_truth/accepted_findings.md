# Accepted H/M Findings: Vultisig

# [H-01] Most users won’t be able to claim their share of Uniswap fees

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-06-vultisig
- **Source snapshot:** competitions/2024-06-vultisig/final_report.html

Submitted by juancito, also found by Bigsam, crypticdefense, tobi0x18, rspadi, Chinmay, 0x04bytes, KupiaSec, Audinarey, h2134, HChang26, kennedy1030, Ryonen, rbserver, and shaflow2 Users should be able to claim Uniswap fees for their current liquidity position regardless of their pending vestings, or cliff. But most users won’t be able to claim those Uniswap fees.

It is also possible that they won’t be able to claim their vesting if they accumulate sufficient unclaimed Uniswap fees.

## Vulnerability Details

The root issue is that the claim() function collects ALL the owed tokens at once, including the ones from the burnt liquidity, but also the fees corresponding to ALL positions:

( uint128 amountCollected0, uint128 amountCollected1 ) = pool.

collect ( address ( this ), TICK_LOWER, TICK_UPPER, @> type ( uint128 ).

max, @> type ( uint128 ).

max );

- https://github.com/code-423n4/2024-06-vultisig/blob/main/src/ILOPool.sol#L246-L247
Then the platform fees are sent alongside the Uniswap fees from the users that still didn’t claim amountCollected - amount:

TransferHelper.

safeTransfer ( _cachedPoolKey.

token0, feeTaker, amountCollected0 - amount0 ); TransferHelper.

safeTransfer ( _cachedPoolKey.

token1, feeTaker, amountCollected1 - amount1 );

- https://github.com/code-423n4/2024-06-vultisig/blob/main/src/ILOPool.sol#L252-L260
The next time a user calls claim(), pool.collect() will not contain any Uniswap fees as all of them have already been claimed and sent to the first claimer and the rest to the fee taker. If the platform fees are enough to cover the owed fees for the claiming user, the transaction might succeed (this may be possible if the burnt liquidity is enough).

As time passes, more fees will be accumulated, and when Uniswap fees > platform fees, the transaction will also revert even for unclaimed vestings with liquidity to burn. In addition, in most cases after the initial vesting, users won’t be able to claim Uniswap fees, as no fees will be collected, and the contract doesn’t hold those assets (they have been sent to the fee taker).

## Recommended Mitigation Steps

Here’s an suggestion on how this could be solved. The idea is to only collect() the tokens corresponding to the liquidity of the tokenId position. So that the next user can also claim their share.

function claim(uint256 tokenId) external payable override isAuthorizedForToken(tokenId) returns (uint256 amount0, uint256 amount1) { + uint128 collect0; + uint128 collect1; uint128 liquidity2Claim = _claimableLiquidity(tokenId); IUniswapV3Pool pool = IUniswapV3Pool(_cachedUniV3PoolAddress); { IILOManager.Project memory _project = IILOManager(MANAGER).project(address(pool)); uint128 positionLiquidity = position.liquidity; // get amount of token0 and token1 that pool will return for us (amount0, amount1) = pool.burn(TICK_LOWER, TICK_UPPER, liquidity2Claim); + collect0 = amount0; + collect1 = amount1; // get amount of token0 and token1 after deduct platform fee (amount0, amount1) = _deductFees(amount0, amount1, _project.platformFee);...

uint256 fees0 = FullMath.mulDiv( feeGrowthInside0LastX128 - position.feeGrowthInside0LastX128, positionLiquidity, FixedPoint128.Q128 ); uint256 fees1 = FullMath.mulDiv( feeGrowthInside1LastX128 - position.feeGrowthInside1LastX128, positionLiquidity, FixedPoint128.Q128 ); + collect0 += fees0; + collect1 += fees1; // amount of fees after deduct performance fee (fees0, fees1) = _deductFees(fees0, fees1, _project.performanceFee);...

} (uint128 amountCollected0, uint128 amountCollected1) = pool.collect( address(this), TICK_LOWER, TICK_UPPER, - type(uint128).max, - type(uint128).max + collect0, + collect1 );...

}

## Assessed type

Uniswap 0xsomeone (judge) commented:

The Warden outlines an issue with the fee collection mechanism whenever a position is claimed that would result in the contract claiming more funds than the user is due and the fee taker acquiring this difference.

In turn, this will result in all consequent claim operations of other NFT IDs on the same tick range (i.e., the same pool) to fail potentially permanently due to being unable to capture the fee-related portion. I consider this to be a significant flaw and one that merits a high-risk severity rating.

Haupc (Vultisig) confirmed

# [H-02] Vultisig whitelisting can be bypassed by anyone

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-06-vultisig
- **Source snapshot:** competitions/2024-06-vultisig/final_report.html

Submitted by juancito, also found by h2134, bbl4de, robertodf99, DanielArmstrong, 4rdiii, Atharv, Mj0ln1r, dvrkzy, Bigsam, 0xrugpull_detector, 0xMAKEOUTHILL, Shahil_Hussain, 0x04bytes, deepkin, Utsav, Nikki, Maroutis, EPSec, kennedy1030, 0xMosh, lionleo, Bob, MrPotatoMagic, leegh, Hendobox, c-note, excalibor, 0xR360, araj, and KupiaSec Whitelist launch will be bricked. Anyone can buy tokens, and also bypass the 3 ETH limit by buying via other non-whitelisted accounts. This will have an impact on price and ruin the opportunities of legit whitelisted users.

Here’s a diagram on the timelines of the launch. “WL Launch” is the affected phase.

## Vulnerability Details

The checkWhitelist() function makes an erroneous check here:

if ( _allowedWhitelistIndex == 0 || _whitelistIndex [ to ] > _allowedWhitelistIndex ) { revert NotWhitelisted (); }

- https://github.com/code-423n4/2024-06-vultisig/blob/main/hardhat-vultisig/contracts/Whitelist.sol#L216
_allowedWhitelistIndex is the max index allowed, and works as a limit, not a whitelist flag. Once it is set (which must happen for all whitelists), any non-whitelisted user can bypass it. This is because _whitelistIndex[to] will be 0, and _whitelistIndex[to] > _allowedWhitelistIndex will never revert ( 0 > 1000, for example).

## Recommended Mitigation Steps

Prevent non-whitelisted users to bypass the whitelist:

- if (_allowedWhitelistIndex == 0 || _whitelistIndex[to] > _allowedWhitelistIndex) { + if (_whitelistIndex[to] == 0 || _whitelistIndex[to] > _allowedWhitelistIndex) { revert NotWhitelisted(); }

## Assessed type

Invalid Validation wewecalibrate (Vultisig) confirmed 0xsomeone (judge) commented:

The Warden and its duplicates outline how the whitelist mechanism in the Whitelist::checkWhitelist function is invalid and will treat every user as initialized by default.

I consider a high-risk rating to be appropriate given that this represents an egregious error that affects sensitive functionality of the system.

# [H-03] Adversary can prevent the launch of any ILO pool with enough raised capital at any moment by providing single-sided liquidity

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-06-vultisig
- **Source snapshot:** competitions/2024-06-vultisig/final_report.html

Submitted by juancito, also found by nnez, iam_emptyset, and 0xc0ffEE

- https://github.com/code-423n4/2024-06-vultisig/blob/main/src/ILOPool.sol#L296

## Impact

It is possible to prevent the launch of any ILO pool at any time, including pools that have reached their total raised amount. This can be done at any time and the cost for the attacker is negligible.

Not only this is a DOS of the whole protocol, but the attack can be performed at the very end of the sale, making users lose a lot on gas fees, considering it will be deployed on Ethereum Mainnet. Hundreds or thousands of users will participate in ILO pools via buy(), and will have to later call claimRefund() to get their “raise” tokens back.

Token launches that were deemed to be successful will be blocked after raising funds from many users, and this will most certainly affect the perception of the token, and its pricing on any attempt of a future launch/sale.

## Vulnerability Details

The ILOManager contract has a check to assert that the price at the time of the token launch is the same as the one initialized by the project. If they differ the transaction will revert, and the token launch will fail:

function launch ( address uniV3PoolAddress ) external override { require ( block.

timestamp > _cachedProject [ uniV3PoolAddress ].

launchTime, "LT" ); ( uint160 sqrtPriceX96,,,,,, ) = IUniswapV3Pool ( uniV3PoolAddress ).

slot0 (); @> require ( _cachedProject [ uniV3PoolAddress ].

initialPoolPriceX96 == sqrtPriceX96, "UV3P" ); address [] memory initializedPools = _initializedILOPools [ uniV3PoolAddress ]; require ( initializedPools.

length > 0, "NP" ); for ( uint256 i = 0; i < initializedPools.

length; i ++) { IILOPool ( initializedPools [ i ]).

launch (); } emit ProjectLaunch ( uniV3PoolAddress ); }

- https://github.com/code-423n4/2024-06-vultisig/blob/main/src/ILOManager.sol#L190
The problem is that sqrtPriceX96 can be easily manipulated in Uniswap v3 Pools when there is no liquidity in it via a swap with no cost. In theory, this could be mitigated by anyone by swapping back to get back to the original price. But there is an additional problem which makes the severity of the attack even higher. The attacker can add single-sided liquidity to the pool (just the Raise Token) after the price was manipulated.

When you select a range that is outside the current price range, you will only be able to supply one of the two tokens.

By adding liquidity in ticks greater than the manipulated price, but lower than the expected initial price, it would require the swapper to provide some SALE_TOKEN, which should not be available at this moment, since they should all be in the ILO pool.

Even if the project admin has some SALE_TOKEN, the attacker can mint a higher amount of liquidity by providing more single-sided RAISE_TOKEN liquidity, making the needed amount of SALE_TOKEN even higher.

## Recommended Mitigation Steps

Since the price can be manipulated, and single-sided liquidity can be minted, getting the price back to its initial price would require swapping and providing SALE_TOKEN. Since it’s an initial sale with vesting for other participants, it is expected that no parties hold the token. But, even if they do, the attack can be performed at some cost anyway as explained before.

So one possible solution could be to reserve some amount in the ILO pool in case it needs to be swapped back, and perform a swap before the liquidity is added to the Uniswap Pool, taking into account an amount that would make the attack very expensive to rollback. Another approach could involve having a wrapper token around the SALE_TOKEN that can be minted and swapped to reach the expected price.

This is a potential first step. Additional considerations shall be taken into account, like an attacker minting liquidity on various tick ranges, which may also affect calculations.

## Assessed type

Uniswap Haupc (Vultisig) confirmed Medium Risk Findings (3)

# [M-01] Vultisig should be burnable

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-06-vultisig
- **Source snapshot:** competitions/2024-06-vultisig/final_report.html

Submitted by EPSec, also found by h2134, chista0x, Drynooo, stacey, and MrPotatoMagic The Vultisig token, as described in its documentation, is expected to include a burnable feature. However, the current implementation of the Vultisig token contract lacks the necessary functions to support token burning. This report identifies the impact of this missing functionality and provides a recommended solution to implement the burn feature. The vultisig stated that they forgot to add this functionality.

## Impact

Non-compliance with Documentation:

Users and developers relying on the documentation will expect burn functionality, leading to confusion and potential loss of trust when they find it missing.

## Recommended Mitigation Steps

To address this issue, the following burn functions should be added to the Vultisig contract:

Burn Function:

Allows token holders to destroy a specified amount of their own tokens.

Burn From Function:

Allows an account to burn tokens from another account, given that the caller has sufficient allowance.

Here is the modified contract with the added burn functionality. Something like this can be added to the Vultisig code:

// SPDX-License-Identifier: MIT pragma solidity ^0.8.24; import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol"; import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol"; import {IApproveAndCallReceiver} from "./interfaces/IApproveAndCallReceiver.sol"; /** * @title ERC20 based Vultisig token contract */ contract Vultisig is ERC20, Ownable { constructor() ERC20("Vultisig Token", "VULT") { _mint(_msgSender(), 100_000_000 * 1e18); } function approveAndCall( address spender, uint256 amount, bytes calldata extraData ) external returns (bool) { // Approve the spender to spend the tokens _approve(msg.sender, spender, amount); // Call the receiveApproval function on the spender contract

IApproveAndCallReceiver(spender).receiveApproval( msg.sender, amount, address(this), extraData ); return true; } + function burn(uint256 amount) public { + _burn(msg.sender, amount); + } + function burnFrom(address account, uint256 amount) public { + uint256 currentAllowance = allowance(account, msg.sender); + require(currentAllowance >= amount, "ERC20: burn amount exceeds + allowance"); + _approve(account, msg.sender, currentAllowance - amount); + _burn(account, amount); + } }

## Assessed type

ERC20 0xsomeone (judge) decreased severity to Medium and commented:

Per the original discussions in the validation repository, this finding’s set was deemed as a valid medium-risk vulnerability due to being a feature described in the documentation that the Sponsor intends to introduce after the audit.

A medium severity was assessed because the functionality is not imperative to the way the protocol works (i.e. all contracts behave “as expected” without it), and burning functionality can be replicated by f.e. transferring funds to the 0xdeaD...DEaD address.

Vultisig confirmed

# [M-02] claim function lacks slippage controls for amount0 and amount1 returned by pool.burn function call

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-06-vultisig
- **Source snapshot:** competitions/2024-06-vultisig/final_report.html

claim function lacks slippage controls for amount0 and amount1 returned by pool.burn function call Submitted by rbserver, also found by bigtone, atoko, crypticdefense, Breeje, cheatc0d3, DanielArmstrong, juancito, jesjupyter, and zraxx Because the claim function does not have slippage controls for amount0 and amount1 returned by the pool.burn function call, the claim function call can suffer from price manipulation on the associated Uniswap v3 pool. If a price manipulation frontruns the claim transaction, the claimed token amounts can be much less than what they should be.

## Recommended Mitigation Steps

The claim function can be updated to include slippage controls for amount0 and amount1 returned by the pool.burn function call like what Uniswap’s decreaseLiquidity function does.

## Assessed type

Invalid Validation 0xsomeone (judge) commented:

The submission outlines how the ILOPool::claim function does not impose any slippage checks on the liquidity withdrawal operation it performs. This is a valid observation as evidenced by the Uniswap V3 router itself and the existence of impermanent loss in Uniswap V3 pairs. A malicious user is able to execute sandwich attacks on ILOPool::claim operations which may result in the claim operation withdrawing more of one asset than the other (in most cases the token the ILO occurred for which, in theory, will be worth less than its paired counterpart intrinsically).

I believe a medium risk severity rating is appropriate given that value can be impacted as impermanent loss is realized during the withdrawal operation.

jarvisnn (Vultisig) acknowledged and commented:

Acknowledged, however, if malicious user performs sandwich attack, they won’t earn anything. In return sandwich attack only benefits more the LP owner.

# [M-03] Transfer of ILOPool NFT token to different account allows for users to bypass the pool’s maxCapPerUser invariant

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-06-vultisig
- **Source snapshot:** competitions/2024-06-vultisig/final_report.html

ILOPool NFT token to different account allows for users to bypass the pool’s maxCapPerUser invariant Submitted by 0xb0k0, also found by hals, araj, hakunamatata, ke1caM, GEEKS, Nikki, Aymen0909, Ryonen, carlitox477, light, Chinmay, Spearmint, LuarSec, juancito, jesjupyter, dimulski, Ack, nnez, h2134, 0x04bytes, rbserver, and Utsav

- https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/src/ILOPool.sol#L143
- https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/src/ILOPool.sol#L151
Description The ILOPool smart contract enables investors to acquire a locked liquidity position represented as an NFT. When an investor invokes the buy() function, they transfer a specified amount of RAISE TOKENS into the pool, thereby opening a position and receiving an ILOPool NFT token that signifies their ownership. The protocol enforces certain invariants related to the minimum and maximum amounts of RAISE TOKENS required for the sale. Furthermore, there is a restriction on the maximum number of tokens each investor can contribute per sale, defined by maxCapPerUser.

struct InitPoolParams { address uniV3Pool; int24 tickLower; int24 tickUpper; uint160 sqrtRatioLowerX96; uint160 sqrtRatioUpperX96; uint256 hardCap; // total amount of raise tokens uint256 softCap; // minimum amount of raise token needed for launch pool uint256 maxCapPerUser; // TODO: user tiers uint64 start; uint64 end; // config for vests and shares.

// First element is always for investor // and will mint nft when investor buy ilo VestingConfig [] vestingConfigs; } Each subsequent call to buy() is intended to increase the investor’s raised amount for their position, ensuring that the user’s total raised amount does not surpass the sale’s maxCapPerUser. However, this restriction can be circumvented by transferring an existing ILOPool NFT token to another account and invoking buy() again. This action results in the protocol minting a new NFT (thus creating a new position) for the investor. Consequently, the maxCapPerUser check applies to the new position’s raised amount, rather than the total amount contributed by the investor.

// If the investor already has a position, increase the raise amount and liquidity // Otherwise, mint a new NFT for the investor and assign vesting schedules @> if ( balanceOf ( recipient ) == 0 ) { // The user can easily set their balance to 0 _mint ( recipient, ( tokenId = _nextId ++)); _positionVests [ tokenId ].

schedule = _vestingConfigs [ 0 ].

schedule; } else { tokenId = tokenOfOwnerByIndex ( recipient, 0 ); } Position storage _position = _positions [ tokenId ]; @> require ( raiseAmount <= saleInfo.

maxCapPerUser - _position.

raiseAmount, "UC" ); // User can open multiple positions bypassing the `maxCapPerUser` constraint _position.

raiseAmount += raiseAmount;

## Impact

This vulnerability allows an investor to:

Bypass the maxCapPerUser constraint by transferring their NFT to another account and purchasing additional tokens, thus minting new NFTs and opening new positions, which in turn breaks a core invariant.

Prevent other investors from participating in the pool by monopolizing the contributions and reaching the pool’s hardCap.

## Recommended Mitigation Steps

Implement an internal tracking mechanism to specify if the investor has bought an NFT, instead of using balanceOf(recipient) == 0. Another thing would be to implement a tracking mechanism to aggregate the total raised amount by an individual investor across all their positions.

## Assessed type

Token-Transfer Haupc (Vultisig) confirmed 0xsomeone (judge) commented:

The Warden and its duplicates have demonstrated how the raise limitation per user can be effectively bypassed by transferring the NFT that the raise amounts are attached with to a different user, permitting one to circumvent the check and deposit as many funds as they wish.

Normally, a QA (L) severity rating would be assigned if the function was permissionless due to the ability of a user to use a secondary account to participate anyway. However, coupled with the fact that a whitelist may be enforced for raising operations, the impact of this submission has been properly assessed as medium-risk.

A subset of this duplicate set has been awarded a 75% reward due to describing an incorrect alleviation, such as imposing a whitelist on the NFT transfers. This is insufficient, as a whitelisted user would be able to collude with another whitelisted user and transfer all NFTs to them (or even acquire whitelist access twice).
