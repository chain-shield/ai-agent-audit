# Benchmark Ground Truth: Vultisig

## Accepted H/M Findings

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

## Rejected Primary Findings

# Rejected Primary Findings: Vultisig

# Missing Percentage Checks on `ILOManager::setPlatformFee` & `ILOManager::setPerformanceFee`

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-314
- **Submitter:** 0xDarko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/314
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-314.md

## Brief Summary

These two functions set fee's that are taken globally across all project pool's. Without any type of upper number limit, `onlyOwner` could set these to 100% by accident. The effect of having 100% fee's would be detrimental to user's since all their money in that transaction would be taken.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_24_group

# Ownable contract not initialised

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-93
- **Submitter:** 0xE1
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/93
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-93.md

## Brief Summary

Ownable function is not initialised, hence functions that require it will not be functional.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_02_group

# Smart Contract Wallets Cannot Withdraw WETH & Funds get locked

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-296
- **Submitter:** 0xHarryBarz
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/296
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-296.md

## Brief Summary

The PeripheryPayments.pay uses [transfer](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/src/base/PeripheryPayments.sol#L29-L30) to wrap ETH to WETH when a contract tries to make payments. If users attempt to withdraw funds using a Smart Wallet that has any extra logic on the receive method, the transaction will run out of gas and fail. Users would then need to transfer the WETH to an EOA in order to unwrap their funds.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Anyone can call function Pay

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-298
- **Submitter:** 0xHarryBarz
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/298
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-298.md

## Brief Summary

The Pay function should only by called by the msg.sender. However, it does not have any auth checks, so that means anyone can call it with an arbitrary _payAddress address and transfer tokens belonging to another user by simply imputing the victim's address on the [payer](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/src/base/PeripheryPayments.sol#L23) parameter, and it passes once the payer has an amount >= value inputed. Doing this will drain user's funds.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# State-Effects issue in the ILOPool.launch function leading to an reentrancy exit for shareholders who bough lp using a contract that listens to erc721 tx's.

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-646
- **Submitter:** 0xMango
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/646
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-646.md

## Brief Summary

Share holders buying shares with smart contracts, can call ILOPool.claimRefund() during the ILOPool.launch() function call, invoked by the sale admin.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_73_group

# frontrunning every call to "initProject" will avoid any creator from using it

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-493
- **Submitter:** 0xR360
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/493
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-493.md

## Brief Summary

DoS on the ILOManager ILOPool creation process

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_34_group

# Tokens to deposit on launch is wrongly calculated for projects with already circulating supply (pools available for trading)

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-572
- **Submitter:** 0xR360
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/572
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-572.md

## Brief Summary

The liquidity amount to deposit from the ILOPool on launch is wrongly calculated. Assumes that the uniswap pool price is static, for projects with already circulating supply this is not the case. When the deposit amount of the payment token (the one given by investors) is less than the amount in the ILOPool, this leftover will be stuck in the contract (loss of funds)

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_30_group

# When minting a new positionn in active pools it will allow for frontrunning

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-585
- **Submitter:** 0xR360
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/585
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-585.md

## Brief Summary

When the ILOPool launches on an active uniswap pool (pool with liquidity), it will allow an attacker to frontrun the liquidity minting and benefit from the price change or making the launch fail completely.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_35_group

# Native currency can be transferred by mistake on the claim function

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-631
- **Submitter:** 0xR360
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/631
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-631.md

## Brief Summary

Anyone calling the claim function can send value to it. This is not recorded, and it cannot be redeemed from the contract. Locking native currency forever.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_09_group

# ILOPool buy - possible liquidity overflow

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-227
- **Submitter:** 0xSpacePirate
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/227
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-227.md

## Brief Summary

In `ILOPool::buy` the solidity version of the contract is =0.7.6 which does NOT support automatic overflow/underflow protection as all the versions >=0.8.x do support. The `_position.liquidity` field is of type uint128 and by providing large enough number when we add up `liquidityDelta` to `_position.liquidity` an overflow is imminent resulting in totally incorrect _position.liquidity.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_13_group

# An attacker can drain funds from the protocol by causing abi.decode function to revert and consume a very significant amount of gas.

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-479
- **Submitter:** 0xpetern
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/479
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-479.md

## Brief Summary

Although decoding the revert reason may appear to work as expected, there is a vulnerability that can be exploited. if an attacker can cause the abi.decode function in the catch block to revert or consume a significant amount of gas, it could lead to a vulnerability where the user controls when the execution succeeds or drains caller funds.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# A malicious user can use both the the old and the new allowance in Vaultsig.sol by unfortunate transaction ordering.

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-488
- **Submitter:** 0xpetern
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/488
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-488.md

## Brief Summary

Vaultsig.sol has an "approveAndCall" function which allocates an allowance for a spender. This uses the approve function in ERC20 which has a risk. The risk is that changing an allowance with this method can allow a malicious user to spend both the old and new allowance by unfortunate transaction ordering which is not intended. This can lead to serious loss of fund by the owner especially when the allowance is a large amount.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Wrong Implementation May Lead to Wrong Accounting for token totalSupply() in mint during launch

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-540
- **Submitter:** 0xweebad
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/540
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-540.md

## Brief Summary

During buy, the code first checks if specified recipient has a token or not. It mints new token to recipient if recipient donot already possess one. However, if recipient already possess a token, it just increment the position and liquidity associated with the recipient's already owned token. See below: Also, during launch too, the code mints a token to each of the recipients of a vesting configurations except the vesting at the 0 index. ` _mint(projectConfig.recipient, (tokenId = _nextId++));` However, in the launch, the code doesnot check whether vesting recipient already has a tokenId or a position, it just mints the recipient right away a new token. Now, the Uniswap NFT manager standard...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# use msg.sender instead of tx.origin in `ILOManager`

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-425
- **Submitter:** 4B
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/425
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-425.md

## Brief Summary

`tx.origin` can make your contract vulnerable to attacks

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_43_group

# Whitelisted Addresses Late to Purchase Receive Fewer VULTs for Same ETH Paid, Potentially Leading to Gas Race and Front-running Issues

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-96
- **Submitter:** 4rdiii
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/96
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-96.md

## Brief Summary

[M-01] Whitelisted Addresses Late to Purchase Receive Fewer VULTs for Same ETH Paid, Potentially Leading to Gas Race and Front-running Issues Impact The design of Uniswap liquidity pools inherently reduces the amount of VULT tokens allocated to users as more participants engage in purchasing. Although this reduction is balanced by the fluctuating price, it may inadvertently introduce a competitive dynamic known as a "gas race." Users eager to secure VULT tokens at the earliest opportunity might rush to complete their transactions, leading to higher transaction fees and potential front-running issues. This discrepancy is not negligible and can be substantiated through a

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# ERC-165 functionality is not implemented which is a deviation from the ERC-721 spec

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-191
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/191
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-191.md

## Brief Summary

The current smart contract `ILOPool` inherits ERC-721 token functionality but does not follow specification correctly which creates a deviation and therefore possible incompatibility.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_77_group

# `maxSaleAmount` can be bypassed

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-206
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/206
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-206.md

## Brief Summary

In the current implementation of `ILOPool` smart contract, there is a `maxSaleAmount` parameter that represents maximum amount of `SALE` token that can be sold during the period of a sale. However, this parameter can be bypassed after `buy()` function call as the function doesn't check if the newly added amount exceeds `maxSaleAmount`.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Vulnerability in Enforcing _maxAddressCap Due to Front-Running

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-680
- **Submitter:** Afriauditor
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/680
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-680.md

## Brief Summary

The _maxAddressCap limit can be circumvented by users front-running transactions to deposit more ETH than the cap allows. The issue arises from the ability of a user to monitor the contract for changes to _maxAddressCap and potentially front-run a transaction to deposit ETH just before the cap is enforced. This allows the user to exceed the cap, depositing more ETH than what the _maxAddressCap allows.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_100_group

# Multicalling is broken

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-308
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/308
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-308.md

## Brief Summary

Important functionality of protocol is broken, considering the batched payable multicall operations would not work.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_23_group

# Incorrect Fee Calculation when adding liquidity to a new and old Position : Failure to Update Fee Growth will cause a user to claim more fees than they should

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-319
- **Submitter:** Bigsam
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/319
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-319.md

## Brief Summary

The current implementation of the fee calculation for user positions does not update the `feeGrowthInside` values when a new position is minted or when additional tokens are added to an existing position. This leads to inaccurate fee calculations, allowing users to collect more fees than they are entitled to. This discrepancy arises because the `feeGrowthInside` values remain at zero or do not reflect the current fee growth, resulting in overpayment of fees when users claim their tokens. The critical impact of this issue includes: - Users receiving more fees than they should, leading to potential losses for the protocol. - Misalignment with the documented behavior, which states that `feeGro...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_51_group

# When whitelister calling claimRefund function its not update the totalRaised

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-245
- **Submitter:** BlockSafe
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/245
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-245.md

## Brief Summary

When Whitelisted calling buy function , its updated the [totalRaised](https://github.com/code-423n4/2024-06-vultisig/blob/main/src/ILOPool.sol#L137) amount here. But whitelisted call [claimRefund](https://github.com/code-423n4/2024-06-vultisig/blob/main/src/ILOPool.sol#L350C4-L360C6) he is able to unlock the token but still the totalRaised is same. So it caused several issues when launching the project and adding liquidity to Uniswap . Impact Due to this error , There are 2 issues arises. Issue 1 : When launching the project there should be minimum amount [staked](https://github.com/code-423n4/2024-06-vultisig/blob/main/src/ILOPool.sol#L274) (softcap) . But if some whitelisted claimRefund ,...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# `ILOVest`'s vest schedule is incorrectly validated which breaks key invariant of Vesting

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-546
- **Submitter:** Breeje
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/546
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-546.md

## Brief Summary

Stakeholders can bypass the vesting invariant and withdraw tokens early.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_86_group

# Rounding Down to Zero Allows Users to Bypass `_maxAddressCap` Limit

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-561
- **Submitter:** Breeje
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/561
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-561.md

## Brief Summary

Users can bypass the invariant that they cannot exceed the `_maxAddressCap`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_22_group

# Sequencer status is not checked on L2 Chains

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-650
- **Submitter:** Bube
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/650
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-650.md

## Brief Summary

There is no check if the sequencer is still active in `UniswapV3Oracle`. The sequencer is responsible for ordering transactions and ensuring they are processed in a timely manner.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Address blacklist is ineffective if self-whitelisting has been enabled

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-64
- **Submitter:** Chinmay
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/64
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-64.md

## Brief Summary

is used to block suspected addresses from buying VULT tokens during the whitelisted period. According to the readme, this is done to remove any suspicious addresses that the admin might detect. But the protection is ineffective when self-whitelisting is enabled because such a malicious actor can have many addresses with which they can whitelist themselves repeatedly, and interact with the pool to buy VULT tokens. Also, when admin calls to blacklist an address, the malicious actor can simply front-run the admin's call to utilize their whole and get the VULT tokens transferred to themselves, before the setBlacklisted call actually gets executed. In such a situation, there will be no effect of...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_32_group

# ILOManager owner can prevent refunds from any project's ILOPools indefinitely

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-652
- **Submitter:** Chinmay
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/652
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-652.md

## Brief Summary

The setRefundDeadlineForProject() function is used to set a project's refund deadline to an abrupt value, and is only callable by the ILOManager owner. It doesn't make sense to allow ILOManager owner to change a project's refund deadline. This has a range of impacts : 1. Refund deadline can abruptly be changed to any time without the project's will 2. Since it is not callable by project admin, projects are forced to use the default deadline and can never change the deadline 3. Should have a check that new refund deadline is still > project.launchTime, otherwise it will mess up the whole timeline 4. The most serious impact is this : It can be used by the ILOManager owner to prevent projects...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_168_group

# An attacker can use claimRefund function in ILOPool.sol to DoS the launching of a project

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-313
- **Submitter:** Dots
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/313
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-313.md

## Brief Summary

An attacker can use claimRefund function in ILOPool.sol to DoS the launching of a project.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_83_group

# In ChainId.sol the get function should not be pure

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-317
- **Submitter:** Dots
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/317
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-317.md

## Brief Summary

In ChainId.sol the `get()` function should not be pure.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# An attacker can call initILOPool first to become admin

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-152
- **Submitter:** Drynooo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/152
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-152.md

## Brief Summary

An attacker can preemptively call initILOPool to become admin, preventing users from creating such pools for ILO.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Overflow may result in users unlocking fewer funds

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-158
- **Submitter:** Drynooo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/158
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-158.md

## Brief Summary

The contract has not checked that the unlocked end must be greater than start. Therefore, using vest.end - vest.start may cause overflow. This will result in the user unlocking less liquidity.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# estimatedETHAmount can return incorrect value whenever L2's are paused

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-517
- **Submitter:** Fassi_Security
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/517
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-517.md

## Brief Summary

inside function `checkWhitelist.sol` the `estimatedETHAmount` is calculated by calling `peek`. function `peek` returns the price of 1 VULT for the last 30 mins: the `PERIOD` is set to 30 min, this is to ensure that the price does not become too stale. This function calls `consult` to retrieve the `timeWeightedAveragetick`. The issue arises when an L2 sequencer experiences downtime and then comes back up. In such cases, the `timeweightedAverageTick` that is retrieved will be the same as it was before the sequencer went down and the TWAP used will still look at the time before the L2 paused. This issue has also been mentioned [here](https://solodit.xyz/issues/h-01-univ3-oracle-unsafe-on-l2s-i...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Funds can get stuck inside the whitelist contract

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-520
- **Submitter:** Fassi_Security
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/520
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-520.md

## Brief Summary

The protocol introduces a `self whitelisting` mechanism, which enables users to send funds to the `whitelist` contract to become whitelisted when this feature is enabled. This is also noted in the [documentation](https://code4rena.com/audits/2024-06-vultisig). The `whitelist` contract has a `receive` function that handles the `self whitelist` logic: `receive()` triggers whenever a user sends ETH to the contract. Several checks are performed, such as ensuring that `self whitelisting` is enabled. If the user passes all these checks, they will be whitelisted and the funds will be sent back to them. However, there is a problem with using `transfer()` instead of `call()`. The `transfer` function...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_08_group

# Muldiv will fail if performancefee is set to 0

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-524
- **Submitter:** Fassi_Security
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/524
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-524.md

## Brief Summary

Upon initialization a user is able to set specific values, one of which is the `performanceFee`. The `perfomanceFee` is used to charge fees generated from a position, this can be set to 0 for a more fee-friendly environment. Inside function `claim`, `performanceFee` is used as a parameter for `_deductFees`: Inside `_deductFees` `performanceFee` is set to the `BPS` paramater: Inside the function `mulDiv` is called, however `mulDiv` will always fail if the `denominator = 0`. the `denominator` is set to the `performanceFee`. This function will always fail if `performanceFee` is set to 0. Ultimately, users will invest in an ILOPool expecting to be able to make claims, only to discover that it's...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_31_group

# NFT does not get burned after refund

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-527
- **Submitter:** Fassi_Security
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/527
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-527.md

## Brief Summary

Whenever a new user purchases into an ILOPool, they call the `buy` function, which automatically mints a NFT. If the `OnlyManager` decides to refund the project he can do so by calling `claimProjectRefund`which calls `_refundProject` Alternatively, a user can call `claimRefund` to get a refund: Inside the `claimRefund` flow the minted NFT gets burned. However, if we look at the flow of `claimProjectRefund` there is no `_burn` happening. Ultimately if the manager calls `claimProjectRefund` all NFTs will still exist and will not be burned.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Zero amount transfer may cause a denial of service.

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-288
- **Submitter:** Flare
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/288
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-288.md

## Brief Summary

The buy function in the ILOPool.sol contract is responsible for opening a position for the recipient by providing the raiseAmount, which is ultimately transferred into the contract. However, the function does not validate that raiseAmount should not be zero. TransferHelper.safeTransferFrom(RAISE_TOKEN, msg.sender, address(this), raiseAmount); Any (all possible ERC20s) that complies with ERC20 token behaviors in scope And casue the protocol intended to use all ERC20 tokens. So the problem is here that some ERC20 tokens (e.g. LEND) do not allow zero value transfers, reverting such attempts. Impact Some ERC20 tokens do not allow zero value transfers, reverting such attempts.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Reentrancy in the function claim(

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-677
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/677
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-677.md

## Brief Summary

The function `claim` calls `TransferHelper.safeTransfer` at the end. This function sends funds externally and there is still a lot of code yet to be executed. Due to function call statements present after `safeTransfer`, there's a potential of reentrancy attack via those external calls.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_125_group

# Improper Access Control

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-683
- **Submitter:** JC
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/683
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-683.md

## Brief Summary

The `initialize` function in the contract `ILOPool` can be called by any account, not necessarily just the contract owner or manager. This represents a security issue as this function sets the initial parameters of the pool which should be a privileged functionality usually limited to contract creators or administrators. This lack of access control mechanism could allow any attacker to set malicious values by calling the `initialize` method.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_50_group

# If the admin gets blacklisted by the SALE_TOKEN token (like if USDC/USDT tokens are used), the whole launch process for all ILOPools of a particular pool key will be DoS'ed

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-516
- **Submitter:** JanuaryPersimmon2024
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/516
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-516.md

## Brief Summary

The whole `ILOManager:launch(...)` and `ILOPool:launch()` flows can be (both intentionally and unintentionally) permanently DoS'ed if the `admin` gets blacklisted by the `SALE_TOKEN` contract. Which is **High** severity. The DoS lock can be resolved if the `project.admin` is a fair actor, e.g. by calling `function transferAdminProject(address admin, address uniV3Pool) external override onlyProjectAdmin(uniV3Pool) {` function. But note that if the `admin` is malicious (which is potentially possible!!), then the batch `ILOManager:launch(...)` and `ILOPool:launch()` functions will be permanently DoS'ed. That is because the `transferAdminProject` function can ***ONLY*** be called by the current...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_169_group

# Malicious project admin can scam legitimate vestors

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-643
- **Submitter:** Maroutis
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/643
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-643.md

## Brief Summary

The `ILOManager` contract allows project admins control over the parameters of an `ILOPool`, including the ability to specify recipient addresses and the distribution of shares in the `vestingConfigs` struct. A bad actor can choose parameters that only rewards the admin by creating scam projects.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# User wont be able claim his funds due to wrong block.timestamp implementation.

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-491
- **Submitter:** Maushish
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/491
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-491.md

## Brief Summary

If [_unlockedLiquidity](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/src/ILOPool.sol#L400C14-L400C32) function gets called at the same timestamp that was `vest.end` timestamp shares won't be unlocked. **The impacts would be:** 1. User won't be able to [claim](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/src/ILOPool.sol#L184) the rewards as it uses [_claimableLiquidity](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/src/ILOPool.sol#L457) under the hood which calls to _unlockedLiquidity. 2. Call to Vesting status will not show the right amount of unlock...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Current PERIOD implementation could lead to swapping scenario where user could swap at an inflation rate.

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-499
- **Submitter:** Maushish
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/499
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-499.md

## Brief Summary

In the readme, its mentioned: Anyone can launch the pool when all conditions met. Consider these possible scenarios: - A denial of service attack on the blockchain has prevented transactions occurring for a significant period of time. - An extreme spike in gas prices has prevented transactions occurring for a significant period of time. - Some unforeseen technical error takes the blockchain down, perhaps during an upgrade or fork. Now due to any of the above scenarios the [oldest observation](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/hardhat-vultisig/contracts/oracles/uniswap/UniswapV3Oracle.sol#L39) is more than 30 minutes due to which def...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Use unchecked in OracleLibrary.sol#consult

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-529
- **Submitter:** Maushish
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/529
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-529.md

## Brief Summary

As the current version of OracleLibrary.sol is [>=0.5.0](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/hardhat-vultisig/contracts/oracles/uniswap/uniswapv0.8/OracleLibrary.sol#L2) due to which any call that could lead to overflow and underflow will get reverted. This happens because in solidity 0.8.0 checked is by default as mentioned in their [docs](https://docs.soliditylang.org/en/latest/control-structures.html#checked-or-unchecked-arithmetic) This happens due to not implementing an unchecked block [OracleLibrary.sol#consult](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/hardhat-vultisig/contract...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Use unchecked in SqrtPriceMathPartial.sol

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-616
- **Submitter:** Maushish
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/616
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-616.md

## Brief Summary

Uniswap math libraries rely on wrapping behavior for conducting arithmetic operations. Solidity version 0.8.0 introduced checked arithmetic by default where operations that cause an overflow would revert. Since the code was adapted from Uniswap these arithmetic operations should be wrapped in an unchecked block. **Impact:** values calculated by [getAmount1Delta](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/src/libraries/SqrtPriceMathPartial.sol#L49) will be far off from what they should be due to which sale tokens value will be incorrect.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Incorrect use of msg.sender in function approveAndCall()

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-477
- **Submitter:** MrPotatoMagic
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/477
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-477.md

## Brief Summary

Function [approveAndCall()](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/hardhat-vultisig/contracts/Vultisig.sol#L16) is used to approve vultisig tokens to a spender and make an external call to function receiveApproval() on the spender contract, which uses transferFrom() to spend the allowance provided. **Root cause:** In the function approveAndCall() though, the internal function _approve() [here](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/hardhat-vultisig/contracts/Vultisig.sol#L18) and the external call [here](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Attacker can make 0 msg.value receive() function calls to spam the whitelist

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-498
- **Submitter:** MrPotatoMagic
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/498
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-498.md

## Brief Summary

The [receive()](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/hardhat-vultisig/contracts/Whitelist.sol#L65) function in the Whitelist.sol contract is called by users to self-whitelist themselves when _isSelfWhitelistDisabled is false and provided they are not blacklisted. **Issue:** Currently though, this mechanism is prone to spamming/griefing since an attacker can make 0 msg.value (no upfront cost other than gas) or dust amount of msg.value calls to the receive() function. **Impact:** Since there are expected to be 1k whitelist slots as per the README, the attacker can spam the whitelist with random addresses **at any point in time**. The imp...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_20_group

# Attacker can frontrun creator of project to deny admin ownership

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-555
- **Submitter:** MrPotatoMagic
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/555
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-555.md

## Brief Summary

Function [initProject()](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/src/ILOManager.sol#L57) allows anyone to create a project by passing InitProjectParams. **Issue:** The issue is that an attacker can frontrun the project creator by submitting the same InitProjectParams parameters. This would give him the admin ownership of the project as seen [here](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/src/ILOManager.sol#L138).

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_38_group

# Invalid validation prevents admin of project from initializing price

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-597
- **Submitter:** MrPotatoMagic
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/597
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-597.md

## Brief Summary

The [initProject()](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/src/ILOManager.sol#L57) function allows anyone to create a project. During this call, a function [initV3PoolIfNecessary()](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/src/ILOManager.sol#L109) is called. This checks if a pool exists or not. If it does not, it creates a pool and initializes it. If it does, it ensures a price is initialized or the current price meets the existing price retrieved from slot0. **Root cause:** The issue will be described in the

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Hardcoded 0 value in tokenOfOwnerByIndex disallows investor from using another tokenId during buy()

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-635
- **Submitter:** MrPotatoMagic
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/635
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-635.md

## Brief Summary

It is possible for an investor to have two NFT tokenIds if someone sold their position or whitelist spot to them. But the current implementation, hardcodes 0 when retrieving the tokenId for the investor through tokenOfOwnerByIndex [here](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/src/ILOPool.sol#L147). This forces the investor to only be able to use their first tokenId instead of the second one they received.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_40_group

# Pausable Tokens like USDC Block Critical Liquidity Management Functions in `ILOPool`

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-124
- **Submitter:** Rhaydden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/124
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-124.md

## Brief Summary

DOS. The pausing of tokens like USDC can block critical liquidity management functions causing failed transactions and disrupted operations. This can affect the `buy`, `claim`, and `launch` functionalities, potentially halting the entire workflow of the ILOPool.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_49_group

# `ILOPool` is Non-Compliant with ERC721 Metadata Standard Limits Token Interoperability and Usability

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-294
- **Submitter:** Rhaydden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/294
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-294.md

## Brief Summary

The absence of the `tokenURI` function in the ILOPool contract, which is a crucial part of the [ERC721 Metadata standard](https://eips.ethereum.org/EIPS/eip-721#specification), has significant implications for the interoperability and usability of the tokens managed by the contract. Without this function, the tokens lack standardized metadata accessibility, hindering their seamless integration with other ERC721-compliant applications, marketplaces, and wallets. The ERC721 Metadata standard provides a unified interface for retrieving token-specific metadata, such as `name`, `description`, `image`, and other `attributes`. By not implementing the `tokenURI` function, the `ILOPool` contract bre...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Inconsistent `totalRaised` State Variable in `claimRefund`

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-502
- **Submitter:** Rhaydden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/502
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-502.md

## Brief Summary

By failing to update the `totalRaised` state variable during refunds, the contract maintains an inaccurate representation of the funds currently held. This has the abiility to cause a cascade of issues: The contract will continue to report an inflated total of raised funds, even after refunds have been processed. This could mislead potential investors and project stakeholders about the true state of the ILO. Additionally, the `launch` function relies on the `totalRaised` value to determine if the soft cap has been reached. With an inaccurate `totalRaised`, the contract might erroneously allow or prevent the launch of the pool. Also, If new purchases are allowed after refunds, the actual tot...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_62_group

# High Volatility TWAP Exploit

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-394
- **Submitter:** Ryonen
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/394
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-394.md

## Brief Summary

Users can exploit periods of high volatility to increase their contributions in `Whitelist`. A sudden price drop can be used to buy at a low cost, and when the Uniswap TWAP oracle is used, it would return a value higher than the market price, recording higher contribution values for users.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_26_group

# FlashSwap to Farm Whitelist

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-401
- **Submitter:** Ryonen
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/401
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-401.md

## Brief Summary

It is possible to use FlashSwap functionality to farm the `Whitelist`, paying a small fee along the way but allowing users to reach the permitted user limit.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# ILOManager Initializer

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-415
- **Submitter:** Ryonen
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/415
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-415.md

## Brief Summary

There is no protection within `ILOManager.initialize`, allowing attackers to monitor the deployment and initialize the contract with contaminated variables, rendering the contract useless.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_27_group

# user cannot call buy() immediately sale starts and when the sale is ending

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-252
- **Submitter:** Sabit
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/252
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-252.md

## Brief Summary

Prevention of buyers from participating in the sale at the exact start and end times of the sale

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Platform and Performance Fees can be set to 100% or more denying investors of earned fees.

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-398
- **Submitter:** Stormreckson
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/398
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-398.md

## Brief Summary

The owner can set platform and performance fees to any value without any validation. Specifically, the functions `setPlatformFee` and `setPerformanceFee` can set `PLATFORM_FEE` and `PERFORMANCE_FEE` to any value, including unreasonably high values such as 100% or more. This lack of validation could lead to severe financial impacts on users. Impact If the fees are set to excessively high values: - Users could lose all their generated profits or even a significant portion of their principal. - High or unpredictable fees would deter users from using the platform, impacting its adoption and liquidity.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# `launch()` cannot be called at the exact `launchTime`

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-101
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/101
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-101.md

## Brief Summary

The current check (`>`) means the `launch()` function cannot be called at the exact `launchTime`, only after it. Users might expect to launch exactly at `launchTime` but this will always revert. Vulnerability details The [launch()](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/src/ILOManager.sol#L187-L198) function is designed to initiate the launch of a project associated with a Uniswap V3 pool. However, the current implementation of the `launch()` function in the `ILOManager` contract uses [the following check](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/src/ILOManager.sol#L188) to determine if...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_126_group

# Reduntant check in `_validateSharesAndVests()` may cause a revert

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-143
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/143
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-143.md

## Brief Summary

Since `totalShares` already equals `BPS` after `_validateVestSchedule()`, adding ` vestingConfigs[i].shares` can cause `totalShares` to exceed `BPS`, leading to a revert. Vulnerability Details The function [_validateSharesAndVests()](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/src/base/ILOVest.sol#L17) is designed to validate the vesting configurations for a given launch time. It calls [_validateVestSchedule()](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/src/base/ILOVest.sol#L35) here: This initally checks that `totalShares == BPS` at the end of its loop. So the `totalShares` is already equal t...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Approve race condition in `Vultisig`

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-17
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/17
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-17.md

## Brief Summary

The contract of `Vultisig` does not have any protection against the well-known “Multiple Withdrawal Attack” attack on the `approve` method of the `ERC20` standard. Although this attack poses a limited risk in specific situations, it is worth mentioning to consider it for possible future operations.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_54_group

# Missing Import for `LiquidityAmounts` Library

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-175
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/175
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-175.md

## Brief Summary

The `LiquidityAmounts` library is used in the `ILOPool` contract but it is not imported. This will lead to a compilation error because the compiler cannot find the definitions for the functions used from this library. Vulnerability Details The `LiquidityAmounts` library is used in the following functions within the `ILOPool` contract: [buy()](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/src/ILOPool.sol#L154-L159) [_saleAmountNeeded()](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/src/ILOPool.sol#L388-L396) The contract however lacks the import statement for `LiquidityAmounts` from `Uniswap V3 peri...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Immediate Sale Closure due to inadequate `saleEndTime` validation

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-240
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/240
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-240.md

## Brief Summary

The absence of a validation check to ensure that the `params.end` timestamp is in the `future` can lead to immediate sale expiration of the ILO pool upon initialization. This means that no sales will be possible as the `endTime` would have already passed. Vulnerability Details The [initILOPool()](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/src/ILOManager.sol#L76-L77) function is designed to initialize an (ILO) pool with specific parameters provided by the project admin. These parameters include the `start` and `end` times for the sale. The function performs several checks to ensure the validity of these parameters, such as verifying that the...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_80_group

# Incomplete Fee Deduction in `claim()` Function

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-407
- **Submitter:** Tigerfrake
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/407
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-407.md

## Brief Summary

In the `Project` struct, there are three fees associated with the project: `fee, platformFee, and performanceFee`. However, only `platformFee` and `performanceFee` are deducted during the `claim()` function. The `fee` field is not utilized, which may indicate an incomplete fee deduction process. Vulnerability Details The `claim()` function currently deducts `platformFee` and `performanceFee` as follows: 1. [platformFee deduction](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/src/ILOPool.sol#L207-L208) 2. [performanceFee deduction](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/src/ILOPool.sol#L212-L...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Approved spender may halt liquidity claiming by calling `claimRefund()` which may conflict with `tokenOwner`'s intentions

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-563
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/563
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-563.md

## Brief Summary

The `claimRefund()` function allows `tokenOwner` or approved `spender` to claim a refund. This deletes the position and burn the NFT, halting the continual claiming of liquidity through the `claim()` function. The `tokenOwner` may intend to claim liquidity but the approved `spender` may halt this by simply calling `claimRefund()`. Vulnerability Details The [claim()](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/src/ILOPool.sol#L251-L253) function allows the `tokenOwner` or approved `spender` to claim their liquidity: The [claimRefund()](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/src/ILOPool.sol#...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_91_group

# Incorrect refund mechanism in `claimRefund()`

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-576
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/576
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-576.md

## Brief Summary

When another user opens a buy position for a given `recipient`, the `raiseAmount` is transferred from the user's account to the contract. However, when `claimRefund()` is called, all raise tokens are sent to the `recipient` i.e `tokenOwner`, not the user who supplied them. Vulnerability Details In [buy()](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/src/ILOPool.sol#L172-L173) the `raiseAmount` is transferred from the user's account (`msg.sender`) to the contract. However, in [claimRefund()](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/src/ILOPool.sol#L358), the refund amount is sent to the token...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# `refundDeadline` is not checked when launching a pool

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-82
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/82
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-82.md

## Brief Summary

The `launch()` function is responsible for initiating the liquidity pool on `Uniswap V3` once certain conditions are met, such as reaching the soft cap. However, it does not check if `_project.refundDeadline` has passed before proceeding to transfer back leftover sale token to `project admin` via the `_refundProject()` call. Vulnerability details During pool launching, any leftover sale token gets [transferred](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/src/ILOPool.sol#L331-L332) to the `project admin` However, if the function is called after the `refundDeadline`, it contradicts the refund logic as the function does not verify this deadline.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Wrong Total Liquidity Locked for Vest is Updated When Investor buys ILO after Claim has been Initially Made in Contract

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-347
- **Submitter:** Topmark
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/347
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-347.md

## Brief Summary

- The Total Liquidity Locked for Vest will be lower than expected as Position vested total Liquidity Updated does not consider liquidity that has been claimed before re-updating total liquidity value - Based on the usage of Position Vested Total Liquidity at [L458](https://github.com/code-423n4/2024-06-vultisig/blob/main/src/ILOPool.sol#L458) & [L195](https://github.com/code-423n4/2024-06-vultisig/blob/main/src/ILOPool.sol#L195) of the contract, Liquidity that has been been previously claimed is wrongly calculated as zero allowing overinflated Claimable Liquidity value breaking Protocol functionality

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Missing virtual Keyword in VultisigWhitelisted::_beforeTokenTransfer Override as per the Rules of Hooks

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-575
- **Submitter:** YourGuyD3v
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/575
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-575.md

## Brief Summary

In the `VultisigWhitelisted` contract, the `VultisigWhitelisted::_beforeTokenTransfer` function is overridden but does not include the virtual keyword. According to the `OpenZeppelin` guidelines, whenever you override a parent’s hook, you should reapply the virtual attribute to the hook to allow child contracts to add more functionality to the hook.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Specific numbers of `_vestingConfigs.length` and `projectConfig.schedule.length` might make a project unlaunchable

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-657
- **Submitter:** amaron
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/657
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-657.md

## Brief Summary

excessive length for `_vestingConfigs.length` and `projectConfig.schedule.length` might make the whole project unlaunchable.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_71_group

# Missing `_contributed` Mapping Update

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-163
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/163
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-163.md

## Brief Summary

The current implementation of the `receive` function does not update the `_contributed` mapping to reflect the amount of ETH each address has contributed. This omission leads to several potential problems: The contract cannot track the total amount of ETH contributed by each address. This makes it impossible to enforce any contribution limits, such as a maximum cap. Any other parts of the contract relying on the `_contributed` mapping to manage or reference contribution amounts will not function correctly.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Gap for Upgradable Contracts

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-177
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/177
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-177.md

## Brief Summary

In the current implementation of the upgradable contract, There are no storage gaps for contract upgradability

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential Off-by-One Error in Time Comparison Using >= with block.timestamp

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-180
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/180
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-180.md

## Brief Summary

Using the >= operator for time comparisons against block.timestamp can introduce off-by-one errors due to the nature of how block.timestamp is updated only once per block. This can lead to unexpected behavior if the condition is met at the exact second when block.timestamp changes. This issue is especially critical in scenarios where time-sensitive operations are performed, potentially causing operations to revert unexpectedly or execute when they shouldn't.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Refund not possible due to blacklisting

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-184
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/184
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-184.md

## Brief Summary

Currently, our contract allows refunds (claimRefund and claimProjectRefund functions) directly to users or project admins. However, direct transfers may fail if the recipient addresses are blacklisted. This situation can lead to contract failures and user dissatisfaction.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_72_group

# MissMatched data types

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-186
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/186
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-186.md

## Brief Summary

mismatches in data types and improper handling of time-based calculations can lead to several risks: Arithmetic Overflows and Underflows: Operations involving block.timestamp (a uint256) and uint128 variables can result in arithmetic operations that exceed the maximum value representable by uint128. If time differences (block.timestamp - vest.start and similar calculations) are not carefully managed, they may lead to unintended overflows or underflows. Incorrect Calculation of Vesting or Time-based Rewards: Incorrect calculations due to overflow can result in inaccurate distributions of vested assets or rewards over time. This could potentially allow an attacker to exploit the contract by m...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Update mint function to include callback data

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-249
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/249
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-249.md

## Brief Summary

The absence of encoded callback data in the mint function call within the addLiquidity function can lead to unauthorized Access to Liquidity, It should contain very important values like the `poolKey`, omitting this data might allow liquidity to be added without proper payment and validation. This could result in unauthorized access to liquidity and potential financial losses for the protocol. Without the `callback` data, the mint function may not be able to verify that the correct payment has been made, allowing an attacker to add liquidity without transferring the necessary required tokens.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# "call()" should be used instead of "transfer()"

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-405
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/405
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-405.md

## Brief Summary

Detailed description of the impact of this finding. This is a classic Code4rena issue:

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Oracles are vulnerable to cross-chain replay attacks

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-428
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/428
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-428.md

## Brief Summary

Detailed description of the impact of this finding. If there's a hard fork (e.g. miner vs proof of stake), signatures/txns submitted on one chain can be replayed on the other chain. Keepers on the forked chain can use oracle prices from the original chain, even though they may not be correct for the current chain (e.g. USDC's price on the fork may be worth zero until Circle adds support on that fork), leading to invalid execution prices and or liquidations.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# "claimRefund" can be overflow

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-437
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/437
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-437.md

## Brief Summary

Detailed description of the impact of this finding. Here there is no limit on initializedPools.length.if there is a large number of initializedPools then totalRefundAmount += IILOPool(initializedPools[i]).claimProjectRefund(_cachedProject[uniV3PoolAddress].admin); can be overflowed as we are using solidity =0.7.6;,our claimRefund will be lost.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Use safeMint instead of mint for ERC721

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-674
- **Submitter:** bareli
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/674
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-674.md

## Brief Summary

Detailed description of the impact of this finding. The msg.sender will be minted as a proof of staking NFT when _stakeToken() is called. However, if msg.sender is a contract address that does not support ERC721, the NFT can be frozen in the contract. As per the documentation of EIP-721: A wallet/broker/auction application MUST implement the wallet interface if it will accept safe transfers. Ref: https://eips.ethereum.org/EIPS/eip-721 As per the documentation of ERC721.sol by Openzeppelin Ref: https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/token/ERC721/ERC721.sol#L274-L285

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_85_group

# Arbitrary logic can be executed in `approveAndCall()` function due to lack of `spender` address validation

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-19
- **Submitter:** bbl4de
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/19
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-19.md

## Brief Summary

The documentation provided in the contest repo states that the receiver contract implementing `IApproveAndCallReceiver` interface will be trusted: *And the actual receiver contract should implement `IApproveAndCallReceiver` interface, especially `receiveApproval` function. This will be only used for trusted receiver contracts btw.* However, there is no validation of the `spender` address - which is the address of the contract implementing the mentioned interface. It means that arbitrary logic can be performed in `receiveApproval()` function.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_00_group

# After launchSucceed, calling the buy function in ILOPool can lead to funds being locked in the contract

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-69
- **Submitter:** bigtone
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/69
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-69.md

## Brief Summary

There is no validation to prevent the buy function from being called after the `launchSucceed` or the `refundTriggered`. This can result in funds being locked in the contract if the buy function is called after the `launchSucceed`. Additionally, the liquidity may be incorrectly added to the investor's position.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_70_group

# Missing validation if softCap is not greater than hardCap in initILOPool function

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-87
- **Submitter:** bigtone
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/87
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-87.md

## Brief Summary

When the softCap is greater than hardCap, the project launch will always fail. The `initILOPool` function does not have a validation to ensure that softCap is less than hardCap in the initILOPool function.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Missing validation check to ensure that launchTime is greater than block.timestamp in the initProject function

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-88
- **Submitter:** bigtone
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/88
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-88.md

## Brief Summary

There's no check to ensure that the `launchTime` is greater than the `block.timestamp` in the `initProject` function. If launchTime is set to a value less than `block.timestamp`, the `refundDeadline` check becomes ineffective. This vulnerability allows anyone to disrupt the project launch by calling the `buy` and `claimRefund` functions.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_29_group

# External call (`transferFrom()`) made after Contract state has been updated in `Buy()`

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-447
- **Submitter:** c-note
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/447
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-447.md

## Brief Summary

If a malicious `ERC20` token is used as the `RAISE_TOKEN` such that `transferFrom()` doesn’t act the way a standard `ERC20` should behave a user calling the `buy()` can get a minted NFT and locked liquidity position without transferring `RAISE_TOKEN` and can further drain all `RAISE_TOKEN` in the contract depending on the malicious `ERC20` `transferFrom()` configurations.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# `Vultisig.approveAndCall(address,uint256,bytes)` is susceptible to gas griefing

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-564
- **Submitter:** carlitox477
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/564
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-564.md

## Brief Summary

Given that gas is not capped when calling the line `IApproveAndCallReceiver(spender).receiveApproval(msg.sender, amount, address(this), extraData)`, if the spender is an upgradable contract, it can be upgraded to consume all attached gas without giving the user the option to limit the gas. Impact * In the case of a batched transaction using this function, the whole batch would revert. * A malicious upgrade can take advantage of using `tx.origin` in other contracts without the user noticing.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_92_group

# Critical privilages are transferred in one step instead of two

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-86
- **Submitter:** chista0x
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/86
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-86.md

## Brief Summary

The `ProjectAdmin` role holds critical privileges for specific projects within the protocol. The `ILOManager::transferAdminProject` function permits the current project admin to transfer ownership and all associated critical privileges to another address in a single step. Relevant Code: [ILOManager.sol::transferAdminProject](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/src/ILOManager.sol#L169) [ILOManager.sol::claimRefund()](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/src/ILOManager.sol#L201) Impact Critical `ProjectAdmin` priviliges could be transferred to an incorrect address e.g. if: - mistak...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Malicious investor can steal rewards if `RAISE_TOKEN` is a token that doesn't revert on failure

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-330
- **Submitter:** crypticdefense
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/330
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-330.md

## Brief Summary

Project admins can create multiple `ILOPools` for their project, which contains mechanism such as vesting, sale management, ERC721 integration, etc. `ILOPool::buy()` allows users to invest into the project by transferring `RAISE_TOKEN`. In return, they receive a portion of liquidity that can earn them rewards. The contest [README](https://code4rena.com/audits/2024-06-vultisig#top:~:text=ERC20%20token%20behaviors%20in%20scope) specifies how tokens that don't revert on failure are in-scope. If a token that doesn't revert on failure is used as the `RAISE_TOKEN`, a malicious investor will have their liquidity amount increased despite no actual transfer, allowing them to later claim tokens that...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Attacker may be able to drain pool funds due to missing check if `liquidity2Claim > 0`

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-331
- **Submitter:** crypticdefense
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/331
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-331.md

## Brief Summary

Project admins can create multiple `ILOPools` for their project, which contains mechanism such as vesting, sale management, ERC721 integration, etc. Users can earn rewards for the amount they invested by calling `ILOPool::claim()`. These rewards are calculated based off the `liquidity2claim`, which is the `amount of unlocked liquidity for the position`. The problem is that despite `liquidity2claim = 0`, the `ILOPool::claim()` may still calculate rewards > 0 for the user. This is because `IUniswapV3PoolActions::burn()` is called with `amount = liquidity2claim`, which actually recalculates the fees owed to a position if `amount = 0`. Since the fees owed are recalculated, they may be greater t...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Once refund is triggered, no one else can receive refunds

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-334
- **Submitter:** crypticdefense
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/334
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-334.md

## Brief Summary

Project admins can create multiple `ILOPools` for their project, which contains mechanism such as vesting, sale management, ERC721 integration, etc. Users can invest in projects by calling `ILOPool::buy`, which transfers `RAISE_TOKEN` from the investor to the pool, in return for liquidity. The project admin can also transfer `SALE_TOKEN` to the pool, which will be used to mint liquidity when the project is launched. If the project has not launched yet and the refund deadline has passed, investors and the project admin can receive refunds for the amount of tokens they had transferred. The problem is that each refund function has a `refundable()` modifier which requires that a refund has not...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_75_group

# Transferring `Vultisig` tokens may experience DoS from overflow due to incorrect version of `FullMath.sol`

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-336
- **Submitter:** crypticdefense
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/336
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-336.md

## Brief Summary

The `Vultisig token` will initially have a whitelist period that allows only whitelisted addresses to transfer tokens, with a maximum transfer cap of `VULT` tokens worth `3 ETH`. To ensure that whitelisted addresses can only transfer `VULT` tokens worth `3 ETH`, the protocol utilizes `UniswapV3Oracle` that returns the `ETH` price for the amount of `VULT` currently being transferred. The `UniswapV3Oracle` utilizes `OracleLibrary.getOldestObservationSecondsAgo` and `OracleLibrary.getQuoteAtTick` for this calculation. `OracleLibrary.getQuoteAtTick` performs `FullMath.mulDiv` with `sqrtRatioX96` and `sqrtRatioX128` values, which is supposedly safe, as `FullMath` handles `phantom overflows`, whe...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_06_group

# Only whitelisted addresses can invest in projects, despite project admin's intention to allow anyone to invest

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-340
- **Submitter:** crypticdefense
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/340
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-340.md

## Brief Summary

Project admins can create multiple `ILOPools` for their project, which contains mechanism such as vesting, sale management, ERC721 integration, etc. Project admins have the authority to decide whether only whitelisted addresses can invest into the project (essentially private investors), or if anyone can invest. The problem is that when a `project admin` sets the ability for anyone to invest into the project, the function `ILOPool::buy`, which is the function called by investors to invest into the project, doesn't take this into account. Only `whitelisted addresses` can invest into projects permanently. The impact is that project creators must always know of potential investors ahead of tim...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Projects are incompatible with USDT

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-343
- **Submitter:** crypticdefense
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/343
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-343.md

## Brief Summary

In the contest [README](https://code4rena.com/audits/2024-06-vultisig#top:~:text=ERC20%20token%20behaviors%20in%20scope), the protocol mentions that `ERC20 Missing return values` are in-scope. One of these ERC20 tokens is USDT, which does not return type `bool` on mainnet. The protocol utilizes `TransferHelper` with openzeppelin's `IERC20` functions for `transfer` and `approval`. These functions return type `bool`, thus the function signature will not match with `USDT`, and will revert. Projects will not be compatible with `USDT`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Use safeTransfer instead of transfer

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-473
- **Submitter:** ctrus
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/473
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-473.md

## Brief Summary

Result of transfer is not checked in `pay()` function of PeripheryPayments contract. Some tokens do not revert on failure, but instead return false (e.g. ZRX, EURS). these tokens could return false from transfer function call to indicate the transfer fails, while the calling contract would not notice the failure if the return value is not checked. Checking the return value is a requirement to ensure the transfer really went successfully. >Callers MUST handle false from returns (bool success). Callers MUST NOT assume that false is never returned!

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_12_group

# Denial of Service (DOS) Attack on the Multicall contract

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-20
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/20
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-20.md

## Brief Summary

Detailed description of the impact of this finding. The Multicall contract allows multiple method calls within a single transaction using the multicall function. This functionality can be exploited to create a DOS attack by exhausting the gas limit. If an attacker manages to exhaust the gas limit, it could prevent legitimate users from successfully executing transactions. This could disrupt the availability and reliability of the contract’s services.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_76_group

# Incorrect calculation in the FullMath contract

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-54
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/54
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-54.md

## Brief Summary

Detailed description of the impact of this finding. Using the bitwise XOR operator instead of the exponential operator results in incorrect calculations, which can lead to severe financial discrepancies and security vulnerabilities within the smart contract. Incorrect values can propagate through the contract, leading to unintended behaviors, loss of funds, or manipulation by malicious actors. A vulnerability has been found in the FullMath.sol contract located at https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/hardhat-vultisig/contracts/oracles/uniswap/uniswapv0.8/FullMath.sol#L83. The vulnerability arises from the incorrect usage of the bitwise...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Rounding errors during division operator in the SqrtPriceMathPartial contract

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-60
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/60
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-60.md

## Brief Summary

Detailed description of the impact of this finding. Rounding errors can lead to incorrect calculations of price deltas, which may result in financial discrepancies. This can affect the accurate execution of trades and liquidity provisions, potentially leading to losses for users or incorrect token balances. The following code sections in SqrtPriceMathPartial.sol are identified as potentially vulnerable to rounding errors, specifically during division operations:

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_90_group

# Whitelist doesn't matter

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-159
- **Submitter:** djanerch
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/159
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-159.md

## Brief Summary

The current whitelisting system is ineffective because whitelisted users can approve or send NFTs to non-whitelisted addresses, allowing those addresses to interact with the protocol. This undermines the purpose of having a whitelist.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of zero value check

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-164
- **Submitter:** djanerch
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/164
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-164.md

## Brief Summary

In the `ILOPool:claim` function, there is no check to verify if `feeGrowthInside0LastX128` is zero. If it is zero, this subtraction can cause an underflow.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# `_beforeTokenTransfer` will not work as intended because of missing override

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-182
- **Submitter:** dvrkzy
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/182
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-182.md

## Brief Summary

`VultisigWhitelisted.sol` implements a `_beforeTokenTransfer()` hook: As you can see after the if statement it calls `super._beforeTokenTransfer()`. The issue is that this line will not actually do anything because of a missing override. Impact Because `_beforeTokenTransfer()` implementation does not do anything except some checks with `checkWhitelist()` it cannot be used as a hook. It does not do any token transfers and it is not called before any.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# `TickMath.sol` and `FullMath.sol` don't use unchecked.

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-196
- **Submitter:** dvrkzy
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/196
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-196.md

## Brief Summary

The `TickMath.sol` and `FullMath.sol` libraries were taken from Uniswap. However Uniswap implements them with solidity version `< 0.8.0`. This solidity version introduces overflow and underflow protection. Which means operations will revert if they lead to overflow/underflow. The issue is that the version upper cap has been removed in this protocol's implementation but the arithmetic operations are not be wrapped in an unchecked block. Impact `TickMath.sol:getSqrtRatioAtTick()` is used to get the upper/lower sqrtRatioX96 in `ILOManager.sol`: `FullMath.sol: mulDiv()` is used multiple times in `ILOPool.sol`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_84_group

# `ILOPool.sol` cannot be initialized

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-198
- **Submitter:** dvrkzy
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/198
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-198.md

## Brief Summary

The `ILOPool.sol` constructor calls `_disableInitialize()` which would prevent any calls to the `initialize()` function. Impact The contract cannot be initialized and thus the values for the state variables cannot be set. This makes the contract unusable because the uninitialized variables will make the functions in this contract revert.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_94_group

# `setOpenToAll` has no access control

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-213
- **Submitter:** dvrkzy
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/213
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-213.md

## Brief Summary

`ILOWhitelist:setOpenToAll` is intended to be used by a project admin to set `_openToAll` to either true or false. The issue is that the modifier it has doesn't do anything and thus this function has no access control. Impact Because of this missing access control anyone can set `_openToAll` to true which will make everyone whitelisted.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential out of gas error

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-455
- **Submitter:** dvrkzy
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/455
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-455.md

## Brief Summary

`ILOPool.sol: _unlockedLiquidity()` unnecessarily loops through a storage variable which could lead to an out of gas error. Impact If an out of gas error occurs then the transaction will just revert and users won't be able to use `claim()`

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# No deadline set when depositing into Uniswap pool

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-464
- **Submitter:** dvrkzy
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/464
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-464.md

## Brief Summary

Calling `ILOPool.sol: launch()` deploys liquidity to a Uniswap pool. However there is no deadline set which can be a problem in a specific edge case. Impact If the transaction for launching the project gets stuck in the mempool it might execute in an unfavorable market conditions which will be a loss of funds for both the project and the investors.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_48_group

# Wrong liquidity specified when adding it to a uniswap pool

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-474
- **Submitter:** dvrkzy
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/474
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-474.md

## Brief Summary

`ILOPool.sol: launch()` adds liquidity to a uniswap pool: The issue is that the `liquidity` that we add is calculated for only one of the two tokens. Impact Projects will deploy only a portion of the whole liquidity. This is a loss of funds for the investors since the positions they are given won't be providing as much liquidity as expected. It's also possible that this might make the function revert because 0 slippage is expected:

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_11_group

# Address Order Causing Discrepancies in Calculated Quote Amounts

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-589
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/589
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-589.md

## Brief Summary

In the following lines [OracleLibrary.sol#L47-L49](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/hardhat-vultisig/contracts/oracles/uniswap/uniswapv0.8/OracleLibrary.sol#L47-L49) when calculating the `quoteAmount`. and [OracleLibrary.sol#L52-L54](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/hardhat-vultisig/contracts/oracles/uniswap/uniswapv0.8/OracleLibrary.sol#L52-L54) The `quoteAmount` is calculated differently based on the order of `baseToken` and `quoteToken` addresses. However, this can lead to a precision loss in the resulting `quoteAmount`. OracleLibrary.sol

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# UniswapV3 Oracle Misidentifies Oldest Observation During Pool Expansion

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-594
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/594
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-594.md

## Brief Summary

The way the function handles the case when the observation cardinality is in the process of increasing. > Uniswap V3 stores observations in a circular buffer. `observationIndex` points to the most recent observation, and `observationCardinality` indicates the buffer's size. > When the buffer is complete, and a new observation needs to be stored, the cardinality is increased, and the oldest observation is overwritten. > The function tries to find the oldest observation by checking if the observation at `(observationIndex + 1) % observationCardinality` is initialized. If not, it assumes the oldest observation is at index 0. **The Problem** The assumption that the oldest observation is at inde...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_151_group

# getTickAtSqrtRatio Returns One Tick Too High Due to Incorrect Logic

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-596
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/596
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-596.md

## Brief Summary

The `getTickAtSqrtRatio` function in `TickMath.so`l aims to compute the tick corresponding to a given square root price. While the code implements a binary search-like approach to achieve this, there's a subtle yet significant bug. **The Bug:** The issue lies in the final comparison used to determine whether `tickHigh` or `tickLow` is the correct tick: [TickMath.sol#L203-L208](https://github.com/code-423n4/2024-06-vultisig/blob/cb72b1e9053c02a58d874ff376359a83dc3f0742/hardhat-vultisig/contracts/oracles/uniswap/uniswapv0.8/TickMath.sol#L203-L208) The code checks if `getSqrtRatioAtTick(tickHi)` is less than or equal to `sqrtPriceX96`. However, the logic should be reversed. If `getSqrtRatioAtT...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Using average tick price method leads to incorrect price quote

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-357
- **Submitter:** gesha17
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/357
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-357.md

## Brief Summary

The [vultisig oracle contracts](https://github.com/code-423n4/2024-06-vultisig/blob/main/hardhat-vultisig/contracts/oracles/uniswap/uniswapv0.8/OracleLibrary.sol#L14) are using an average tick method to provide a TWAP for the price of a token from uniswap. Using an average tick to compute average price leads to a discrepancy in the computed average price. This is because averaging the tick observations is not the same as averaging the resulting price observations, since linear tick growth results in exponential price growth. This largely depends on the market conditions, however if the market is volatile enough this can create opportunities for users to take advantage of lower than actual p...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_117_group

# User / fee taker blacklisted by token will temporarily prevent fees from being collected for the protocol

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-414
- **Submitter:** gesha17
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/414
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-414.md

## Brief Summary

If the token owner/fee taker gets blacklisted by a token, the claim() function will start reverting as it directly transfers tokens to the tokenOwner. If the fee taker is blacklisted users will be temporarily unable to use the claim function until the fee taker address is changed.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_46_group

# Non-EVM addresses not supported

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-225
- **Submitter:** hassan-truscova
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/225
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-225.md

## Brief Summary

As per the documentation of [Vultisig](https://docs.vultisig.com/#what-is-vultisig), it also supports non-evm chains. > Vultisig is a multi-chain multi-platform Threshold-Signature vault that does not need any specialised hardware. It supports most UTXO, EVM, BFT and EdDSA Chains. These chains can have a different address standards. These addresses can’t be whitelisted in [Whitelist.sol](https://github.com/code-423n4/2024-06-vultisig/blob/main/hardhat-vultisig/contracts/Whitelist.sol#L41-L45). Thus, implementation from non-EMV chain can’t interact with the EVM chain.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Wrong VULT price calculation.

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-376
- **Submitter:** ke1caM
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/376
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-376.md

## Brief Summary

TWAP price for 1 VULT for the last 30 mins is miscalculated. `getQuoteAtTick` given a tick and a token amount, calculates the amount of token received in exchange. If we want to exchange 100 tokenA for X tokenB the price will not be the same when we want to exchange 1 tokenA for y tokenB and multiply it by 100. Price per one token will be different in both of these scenarios. Another issue is that the price is returned by `getQuoteAtTick` accounts already for slippage so applying 5% slippage is just another cost for user. Impact Incorrect price calculation will expose users to losses.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Initial price can be set outside of the price range.

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-379
- **Submitter:** ke1caM
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/379
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-379.md

## Brief Summary

Project admin can set initial price outside of the tick range. As a result investors liquidity will not earn any fees as their tokens will not be used during trades outside thier price range. As we can see this check inside `ILOManager::initILOPool` check if the `sqrtRatioLowerX96` is lower than `_project.initialPoolPriceX96` and if `sqrtRatioLowerX96` is lower than `sqrtRatioUpperX96`. In fact it should check that `_project.initialPoolPriceX96` is lower than `sqrtRatioUpperX96`, meaning the price is in range. Impact Investors liquidity will not earn trading fees.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_146_group

# TWAP might return inaccurate price.

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-380
- **Submitter:** ke1caM
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/380
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-380.md

## Brief Summary

Uniswap v3 provides a Time-Weighted Average Price (TWAP) mechanism, which allows to get a relatively stable price over a specified period. This can be used to reduce the impact of short-term volatility and manipulation. To use this effectively a pool needs to have observations to calculate the price. If there are a only few observations the price can be unstable and volatile to price swings. In peek function the protocol uses `getOldestObservationSecondsAgo` which gets oldest observation. The issue is that at this point there can be a few observations which provide inaccurate prices. This observations will be used to calculate TWAP. Impact TWAP can return inaccurate price.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_42_group

# Incorrect vestingConfig initialisation when lauching pool

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-360
- **Submitter:** kodyvim
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/360
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-360.md

## Brief Summary

first element within vestingConfig would be skipped when lauching pool affecting claiming as recipients within these position in the config would not be minted a tokenId.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_16_group

# Non-whitelisted user may buy a position if whitelist is implemented after sale start

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-395
- **Submitter:** light
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/395
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-395.md

## Brief Summary

A user that is not whitelisted may buy a liquidity position, if the admin introduces the whitelist after the start of the sale. Imagine a scenario where there is initially no whitelist enabled. A user buys a liquidity position, and subsequently, the admin introduces a whitelist that does not include the buyer. There is no possibility for the admin to remove the buyer from the sale.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# # [H-1] CREATE2 deployments can have ILOPool drained by Attacker

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-538
- **Submitter:** mansa11
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/538
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-538.md

## Brief Summary

ILOPool are user(project admin) supplied pools, which gets deployed after every call to `initILOPool` which creates unique pools based on the projects requirements and settings to be used for sale and vest. These pools are susceptible to be frontrun and drained by malicious actors. **Relevant Context** These past reports details and shows how attackers can take advantage of the CREATE2 opcode in performing this exploit - https://github.com/sherlock-audit/2023-12-arcadia-judging/issues/59 - https://cantina.xyz/code/5240b7c7-6fec-4902-bec0-8cad12f14ec4/src/ScribeOptimistic.sol#L113-L119 - https://cantina.xyz/code/c90131b4-5c7c-4ebc-a1f3-8002d219bfe0/findings/1091?status=duplicate%2Cconfirmed%...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_25_group

# ## [M-2] Different sales token can be specified, which breaks the protocol invariant

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-621
- **Submitter:** mansa11
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/621
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-621.md

## Brief Summary

The `initProject` is used to initialize a new project including all the details contained in the `InitProjectParams` such as the `saleToken` and the `raiseToken`. The `saleToken` is an invariant which is meant to be always fixed as it is the core token of the protocol which is used to be used for vesting. <details><summary>Details</summary> </details> According to the documentation, the pool pairs are meant to be in the form of ERC20/Vaultsig which consist of a `raiseToken` and a `saleToken`. However, this invariant does'nt hold when a user specifies a totally different `saleToken` pair such as USDT/USDC etc. Impact Summary This breaks the protocol invariant and logic Severity Explanation T...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# # [M-3] Projects unable to deploy with certain `poolKey`

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-647
- **Submitter:** mansa11
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/647
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-647.md

## Brief Summary

`poolKey` is the combination of the `poolKey.token0, poolKey.token1, poolKey.fee` and by standard, no project should be able to deploy with the same poolKey. Meaning no project should be able to deploy with the same token0, token1 and fees. However, it should be made possible to deploy with a combination of both token0, token1 and a different fee according to how it is been done in uniswap, but this invariant is broken and when projects wants to deploy with same tokens but different fees, it fails. Impact Summary This breaks the protocol core invariant and as such is a denial of service. Severity Explanation This can be classified as a MEDIUM as it only breaks the protocol invariant

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Users can bypass the vesting schedule as claimableLiquidity does not follow the vesting schedule properly

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-138
- **Submitter:** peanuts
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/138
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-138.md

## Brief Summary

The recipient will not be able to withdraw the liquidity. Liquidity will be stuck.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_18_group

# ILOPool.buy does not incentivize investors at all by limiting the liquidity given to the investors

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-189
- **Submitter:** peanuts
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/189
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-189.md

## Brief Summary

Investors will not participate in the project because there is no incentive to do so.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_21_group

# Attacker can DOS attack by claiming refund for ILOPools

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-679
- **Submitter:** pranavgarg
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/679
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-679.md

## Brief Summary

If any user claims refund for any ILOPools, it is counted as refunded, and the pool cannot be launched. It is very easy for an attacker to buy and claimRefund for a pool in the same transaction with a very low cost, thus doing a DOS attack to the launch.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_118_group

# `ILOManager.launch` function is callable by anyone, which breaks invariant that only project admin should be allowed to launch project and all of its ilo pools

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-526
- **Submitter:** rbserver
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/526
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-526.md

## Brief Summary

`ILOManager.launch` function is callable by anyone, which breaks invariant that only project admin should be allowed to launch project and all of its ilo pools

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_47_group

# Donation will result in asset loss

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-352
- **Submitter:** steadyman
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/352
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-352.md

## Brief Summary

Donation will result in asset loss

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Modifier `onlyProjectAdmin()` implemented in `ILOWhitelist.sol` is empty which when used by the functions of other contracts makes the contract vulnerable.

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-671
- **Submitter:** tdey
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/671
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-671.md

## Brief Summary

The `onlyProjectAdmin()` modifier implemented in `ILOWhitelist.sol` is empty. The `onlyProjectAdmin()` modifier is declared in `IILOWhitelist.sol` which is inherited by `ILOWhitelist.sol`. The name of the modifier suggest access control by Admin, if the modifier does not functions as it is supposed to, then it can lead to access control vulnerability. All contracts importing or inheriting `ILOWhitelist.sol` will also be effected. In `ILOWhitelist.sol` contract, the function `isOpenToAll()` uses `onlyProjectAdmin()` modifier too, which is not declared properly. In turn the `_isWhitelisted()` function returns `isOpenToAll()` function. Any malicious user can modify `_isWhitelisted` function to...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Users can avoid paying protocol and performance fees

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-267
- **Submitter:** web3km
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/267
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-267.md

## Brief Summary

Users can avoid paying fees for the raise token.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# EIP-1167 is not supported on zkSync Era

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-269
- **Submitter:** web3km
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/269
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-269.md

## Brief Summary

Unexpected reverts when deploying to zkSync era due to a failure when cloning the implementation.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Hash Collision in `initILOPool` Function

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-592
- **Submitter:** yasmine
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/592
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-592.md

## Brief Summary

The use of `abi.encodePacked` in the `src/ILOManager::initILOPool` function can lead to hash collisions, which may allow a malicious actor to create different inputs that produce the same hash. This could potentially result in the creation of duplicate ILO pools or other unintended behaviors, compromising the integrity of the system. In the `initILOPool` function, the salt for the deterministic clone is generated using `abi.encodePacked`: The use of `abi.encodePacked` to generate the salt can result in hash collisions, as different inputs can produce the same hash, potentially leading to unintended behavior.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Anyone can become manager of a pool

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-50
- **Submitter:** youleeyan
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/50
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-50.md

## Brief Summary

The initialize function of ILOPools contract does not verify the message sender and directly assigns them the MANAGER role. If an attacker sees that the contract has been deployed but not initialized, they can initialize it and become the new manager. What is more, this is very likely to be detected by a bot and be instantly hijacked.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_50_group

# Some liquidity is locked due to division.

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-265
- **Submitter:** zraxx
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/265
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-265.md

## Brief Summary

Some liquidity is locked due to division.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_57_group

# The schedules can be overlapped due to improper check.

- **Contest:** Vultisig
- **Slug:** 2024-06-vultisig
- **Submission:** V-273
- **Submitter:** zraxx
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-vultisig-validation/issues/273
- **Source snapshot:** competitions/2024-06-vultisig/submissions/raw/V-273.md

## Brief Summary

The schedules can be overlapped due to improper check.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary
