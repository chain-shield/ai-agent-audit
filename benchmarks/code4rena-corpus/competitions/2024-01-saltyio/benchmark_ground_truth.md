# Benchmark Ground Truth: Salty.IO

## Accepted H/M Findings

# Accepted H/M Findings: Salty.IO

# [H-01] Development Team might receive less SALT because there is no access control on VestingWallet#release()

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

VestingWallet#release() Submitted by 0xpiken The Development Team could potentially incur a loss on their SALT distribution reward due to the absence of access control on VestingWallet#release().

## Recommended Mitigation Steps

Since exchangeConfig.managedTeamWallet is immutable, it is reasonable to config managedTeamWallet as the beneficiary when deploying teamVestingWallet:

- teamVestingWallet = new VestingWallet( address(upkeep), uint64(block.timestamp), 60 * 60 * 24 * 365 * 10 ); + teamVestingWallet = new VestingWallet( address(managedTeamWallet), uint64(block.timestamp), 60 * 60 * 24 * 365 * 10 ); Introduce a new function in managedTeamWallet to transfer all SALT balance to mainWallet:

function release ( address token ) external { uint balance = IERC20 ( token ).

balanceOf ( address ( this )); if ( balance != 0 ) { IERC20 ( token ).

safeTransfer ( mainWallet, balance ); } Call managedTeamWallet#release() in Upkeep#performUpkeep():

function step11() public onlySameContract { - uint256 releaseableAmount = VestingWallet(payable(exchangeConfig.teamVestingWallet())).releasable(address(salt)); - // teamVestingWallet actually sends the vested SALT to this contract - which will then need to be sent to the active teamWallet VestingWallet(payable(exchangeConfig.teamVestingWallet())).release(address(salt)); - salt.safeTransfer( exchangeConfig.managedTeamWallet().mainWallet(), releaseableAmount ); + exchangeConfig.managedTeamWallet().release(address(salt)); } othernet-global (Salty.IO) confirmed and commented:

The ManagedWallet now the recipient of teamVestingWalletRewards to prevent the issue of DOS of the team rewards.

- https://github.com/othernet-global/salty-io/commit/534d04a40c9b5821ad4e196095df70c0021d15ab
ManagedWallet has been removed.

- https://github.com/othernet-global/salty-io/commit/5766592880737a5e682bb694a3a79e12926d48a5
Picodes (Judge) commented:

My initial view on this is that the issue is within Upkeep as it integrates poorly with the vesting wallet. It forgets that there is no access control, so I tend to see this as in scope.

The issue is not strictly in the deployment scripts, not strictly in the vesting wallet either because it makes sense to have no access control on release, so it must be in Upkeep.

Note: For full discussion, see here.

Status:

Mitigation confirmed. Full details in reports from 0xpiken, zzebra83, and t0x1c.

# [H-02] First Liquidity provider can claim all initial pool rewards

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

Submitted by 0xCiphky, also found by 0xCiphky ( 1, 2 ), J4X, Toshii, stackachu, Silvermist, DedOhWale, OMEN, zhaojie, 0x3b ( 1, 2, 3 ), ether_sky, Evo, israeladelaja, RootKit0xCE ( 1, 2 ), a3yip6, Stormreckson, and twcctop

- https://github.com/code-423n4/2024-01-salty/blob/53516c2cdfdfacb662cdea6417c52f23c94d5b5b/src/staking/StakingRewards.sol#L57
- https://github.com/code-423n4/2024-01-salty/blob/53516c2cdfdfacb662cdea6417c52f23c94d5b5b/src/staking/StakingRewards.sol#L147
- https://github.com/code-423n4/2024-01-salty/blob/53516c2cdfdfacb662cdea6417c52f23c94d5b5b/src/staking/StakingRewards.sol#L232
Liquidity providers can add liquidity to the protocol using the depositCollateralAndIncreaseShare or depositLiquidityAndIncreaseShare functions, both functions call the _increaseUserShare function to stake the users liquidity and account for the positions rewards. The current implementation has an issue, particularly in how it deals with the virtualRewards calculation for the first user. Since there is no current shares in the pool, then the virtualRewards calculation is skipped.

// Increase a user's share for the given whitelisted pool.

function _increaseUserShare ( address wallet, bytes32 poolID, uint256 increaseShareAmount, bool useCooldown ) internal {...

uint256 existingTotalShares = totalShares [ poolID ]; if ( existingTotalShares != 0 // prevent / 0 ) { // Round up in favor of the protocol.

uint256 virtualRewardsToAdd = Math.

ceilDiv ( totalRewards [ poolID ] * increaseShareAmount, existingTotalShares ); user.

virtualRewards += uint128 ( virtualRewardsToAdd ); totalRewards [ poolID ] += uint128 ( virtualRewardsToAdd ); } // Update the deposit balances user.

userShare += uint128 ( increaseShareAmount ); totalShares [ poolID ] = existingTotalShares + increaseShareAmount;...

} To understand the implications of this, we need to look at how a user’s rewards are calculated. The formula used in the userRewardForPool function is:

uint256 rewardsShare = (totalRewards[poolID] * user.userShare) / totalShares[poolID]; From this calculated rewardsShare, virtualRewards are then deducted:

return rewardsShare - user.virtualRewards; In the case where the first user stakes in an empty pool, they end up having the same number of shares as the totalShares in the pool, but with zero virtualRewards. This means that the first user can claim all the pools rewards in the staking contract, as their share of rewards would not have the necessary deduction of virtualRewards.

// Returns the user's pending rewards for a specified pool.

function userRewardForPool ( address wallet, bytes32 poolID ) public view returns ( uint256 ) {...

// Determine the share of the rewards for the user based on their deposited share uint256 rewardsShare = ( totalRewards [ poolID ] * user.

userShare ) / totalShares [ poolID ];...

return rewardsShare - user.

virtualRewards; } A potential issue is the lack of Initial Rewards in the Contract. Initially, the staking contract does not contain any rewards, meaning that if a user were to claim rewards immediately, they would receive nothing. To overcome this, the user needs to trigger the upkeep function. This function is responsible for transferring up to the maximum allowable daily rewards to the staking contract.

The upkeep contract employs a timer to regulate the frequency and quantity of rewards distribution. However, since this timer begins counting from the moment of the contract’s deployment (in the constructor), and considering the initial voting period for starting up the exchange spans several days, it becomes feasible to distribute the maximum daily reward amount by invoking upkeep.

## Impact

A LP can exploit this vulnerability to claim all the current staking rewards in the contract. Initially, there are 555k SALT bootstrap rewards per pool in the stakingRewardsEmitter, which are emitted at a rate of 1% per day. As a result, the first LP could claim up to 5.5k SALT.

Mitigation confirmed. Full details in reports from zzebra83, 0xpiken, and t0x1c.

# [H-03] The use of spot price by CoreSaltyFeed can lead to price manipulation and undesired liquidations

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

Submitted by 00xSEV, also found by OMEN, J4X, miaowu, Myrault, Banditx0x, linmiaomiao, CongZhang-CertiK, n1punp, and jesjupyter When the price moves, Chainlink instantly reports the spot price, while the TWAP slowly changes the price. The spot price of CoreSaltyFeed can be manipulated, allowing an attacker to move the price in a desired direction.

## Vulnerability Details

The spot price of CoreSaltyFeed can be manipulated, even when considering automatic arbitrage. The cost of moving the price depends on the liquidity of the pools. While the protocol is small, it will be cheap to manipulate, but even as it grows, the cost won’t become prohibitively expensive. If all the pools have 2*1_000 ETH of value each, the attack will cost only ~0.0036 ETH to move a price by 3%, and ~0.0363 ETH to move it by 10%. Refer to the PoCs for the estimated cost of the attack.

Assume the WBTC/USD price moves 3%, from $40,000 to$38,800. Chainlink updates instantly, but the TWAP takes some time. You can see my calculations of the TWAP price change here.

CoreSaltyFeed WBTC/USDS price will be adjusted to match Chainlink’s price by arbitrageurs.

CoreSaltyFeed returns $38,800, Chainlink returns$38,800, TWAP returns $40,000.

The attacker moves the CoreSaltyFeed price ~3%, but less than the difference between TWAP and Chainlink, to $38,000.0035 ETH if the pools have 1000 ETH of liquidity, but if they have 100 ETH, it will require only ~0.0004 ETH.

The difference between CoreSaltyFeed and Chainlink is $800, and from TWAP and Chainlink it's$1,200.

The average price is set to ($38,000 +$38,800) / 2 = $38,400.

Now the attacker can liquidate pools that should not be liquidatable because the price from PriceAggregator is lower than the real price. The attacker can do it first and get the rewards (5%, up to $500 by default). See the relevant code here.

// Reward the caller wbtc.

safeTransfer ( msg.

sender, rewardedWBTC ); weth.

safeTransfer ( msg.

sender, rewardedWETH ); maxRewardValueForCallingLiquidation is set to $500. Depending on Salty’s pool liquidity, ETH price, and how many positions an attacker can liquidate, profitability will vary. I argue that before the protocol gains traction, liquidity will be low for some time, making the attack profitable.

We should also consider that sometimes it will be profitable for the attacker to move the price slightly and be the first to call liquidate in order to receive the rewards.

Other liquidators, who don’t use this attack, will not be able to liquidate, which is unfair.

Note: WBTC and WETH movements of 3% are common and will happen often. For example, about a month ago, there was a 6.5% drop in 20 minutes as reported by Business Insider.

Variations If the Chainlink oracle fails to update prices on time (due to block stuffing before the heartbeat or Chainlink DAO turning it off, as described here and here ), the attack becomes easier as a 3% price change in the market will not be necessary.

In the event of a sudden crash in BTC and/or ETH, an attacker could mint undercollateralized USDS. The 200% collateral requirement, set in StableConfig.initialCollateralRatioPercent and calculated using the outdated TWAP price along with the manipulated CoreSaltyFeed, would be ineffective as protection against this attack when the real price has already dropped below 100%.

During a sudden crash of BTC and/or ETH, the oracle price feed may continue to report the incorrect minimum price. This can again lead to the minting of undercollateralized USDS.

## Impact

Positions that should not be liquidated are liquidated => unexpected liquidation and loss of part of collateral for a borrower (on fees) Honest liquidators who don’t move the price won’t be able to liquidate because an attacker will move the price and liquidate in the same transaction

## Recommended Mitigation Steps

Consider replacing CoreSaltyFeed with a different oracle that provides better protection against manipulation, like Band Protocol.

othernet-global (Salty.IO) confirmed and commented:

Note: the overcollateralized stablecoin mechanism has been removed from the DEX.

- https://github.com/othernet-global/salty-io/commit/f3ff64a21449feb60a60c0d60721cfe2c24151c1
The stablecoin framework: /stablecoin, /price_feed, WBTC/WETH collateral, PriceAggregator, price feeds and USDS have been removed:

- https://github.com/othernet-global/salty-io/commit/88b7fd1f3f5e037a155424a85275efd79f3e9bf9
Status:

Mitigation confirmed. Full details in reports from t0x1c, 0xpiken, and zzebra83.

# [H-04] First depositor can break staking-rewards accounting

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** H-04
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

Submitted by 0xRobocop, also found by stackachu, Toshii, Arz, DedOhWale, peanuts, Draiakoo, zhaojie, and ether_sky Staking in SALTY pools happens automatically when adding liquidity. In order to track the accrued rewards, the code “simulates” the amount of virtual rewards that need to be added given the increase of shares and lend this amount to the user. So, when computing the real rewards for a given user, the code will compute its rewards based on the totalRewards of the given pool minus the virtual rewards. The following code computes the virtual rewards for a user:

- https://github.com/code-423n4/2024-01-salty/blob/53516c2cdfdfacb662cdea6417c52f23c94d5b5b/src/staking/StakingRewards.sol#L81
uint256 virtualRewardsToAdd = Math.

ceilDiv ( totalRewards [ poolID ] * increaseShareAmount, existingTotalShares ); Basically, it aims to maintain the current ratio of totalRewards and existingTotalShares. The issue with this is that allows the first depositor to set the ratio too high by donating some SALT tokens to the contract. For example, consider the following values:

uint256 virtualRewardsToAdd = Math.

ceilDiv ( 1000e18 * 200e18, 202 ); The returned value is in order of 39-40 digits. Which is beyond what 128 bits can represent:

- https://github.com/code-423n4/2024-01-salty/blob/53516c2cdfdfacb662cdea6417c52f23c94d5b5b/src/staking/StakingRewards.sol#L83-L84
user.

virtualRewards += uint128 ( virtualRewardsToAdd ); totalRewards [ poolID ] += uint128 ( virtualRewardsToAdd ); This will broke the reward computations. For a more concrete example look the PoC.

## Recommended Mitigation Steps

Some options:

Make the function addRewards in the StakingRewards contract permissioned. In this way, all rewards will need to go through the emitter first.

Do not let the first depositor to manipulate the initial ratio of rewards / share. It is possible for every pool to burn the initial 10000 shares and starts with an initial small amount of rewards, kind of simulating being the first depositor.

othernet-global (Salty.IO) confirmed and commented:

virtualRewards and userShare are now uint256 rather than uint128.

Fixed in:

- https://github.com/othernet-global/salty-io/commit/5f79dc4f0db978202ab7da464b09bf08374ec618
Picodes (Judge) commented:

Considering that you could time this to break in the future and that it seems easily doable by an attacker on a new pool, High severity seems justified under “Loss of matured yield”.

Status:

Mitigation confirmed. Full details in reports from t0x1c, 0xpiken, and zzebra83.

# [H-05] User can evade liquidation by depositing the minimum of tokens and gain time to not be liquidated

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** H-05
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

liquidation by depositing the minimum of tokens and gain time to not be liquidated Submitted by 0xbepresent, also found by Arz, Audinarey ( 1, 2 ), c0pp3rscr3w3r, stackachu, memforvik, HALITUS, Infect3d, Udsen, Toshii, J4X, Aymen0909, Kalyan-Singh, 0xlemon, novodelta, mussucal, Draiakoo, 0xpiken, zhaojie, zhaojohnson, 00xSEV, juancito, CaeraDenoir, n0kto, DanielArmstrong, Auditwolf, Krace, israeladelaja, 0xAsen, pkqs90, PENGUN, 0xBinChook, lanrebayode77, twcctop, KingNFT, Jorgect, b0g0, 0xRobocop, 0xCiphky, djxploit, erosjohn, holydevoti0n, Banditx0x, iamandreiski, ayden, 0xanmol, klau5, solmaxis69, developerjordy, and 0xAlix2

- https://github.com/code-423n4/2024-01-salty/blob/53516c2cdfdfacb662cdea6417c52f23c94d5b5b/src/stable/CollateralAndLiquidity.sol#L140
- https://github.com/code-423n4/2024-01-salty/blob/53516c2cdfdfacb662cdea6417c52f23c94d5b5b/src/stable/CollateralAndLiquidity.sol#L70
The CollateralAndLiquidity contract contains a critical vulnerability that allows a user undergoing liquidation to evade the process by manipulating the user.cooldownExpiration variable. This manipulation is achieved through the CollateralAndLiquidity::depositCollateralAndIncreaseShare function, specifically within the StakingRewards::_increaseUserShare function (code line #70 ):

File:

StakingRewards.

sol 57:

function _increaseUserShare ( address wallet, bytes32 poolID, uint256 increaseShareAmount, bool useCooldown ) internal 58: { 59:

require ( poolsConfig.

isWhitelisted ( poolID ), "Invalid pool" ); 60:

require ( increaseShareAmount != 0, "Cannot increase zero share" ); 61:

62:

UserShareInfo storage user = _userShareInfo [ wallet ][ poolID ]; 63:

64:

if ( useCooldown ) 65:

if ( msg.

sender != address ( exchangeConfig.

dao ()) ) // DAO doesn't use the cooldown 66: { 67:

require ( block.

timestamp >= user.

cooldownExpiration, "Must wait for the cooldown to expire" ); 68:

69:

// Update the cooldown expiration for future transactions 70:

user.

cooldownExpiration = block.

timestamp + stakingConfig.

modificationCooldown (); 71: } 72:

73:

uint256 existingTotalShares = totalShares [ poolID ]; 74:

75:

// Determine the amount of virtualRewards to add based on the current ratio of rewards/shares.

76:

// The ratio of virtualRewards/increaseShareAmount is the same as totalRewards/totalShares for the pool.

77:

// The virtual rewards will be deducted later when calculating the user's owed rewards.

78:

if ( existingTotalShares != 0 ) // prevent / 0 79: { 80:

// Round up in favor of the protocol.

81:

uint256 virtualRewardsToAdd = Math.

ceilDiv ( totalRewards [ poolID ] * increaseShareAmount, existingTotalShares ); 82:

83:

user.

virtualRewards += uint128 ( virtualRewardsToAdd ); 84:

totalRewards [ poolID ] += uint128 ( virtualRewardsToAdd ); 85: } 86:

87:

// Update the deposit balances 88:

user.

userShare += uint128 ( increaseShareAmount ); 89:

totalShares [ poolID ] = existingTotalShares + increaseShareAmount; 90:

91:

emit UserShareIncreased ( wallet, poolID, increaseShareAmount ); 92: } Malicious user can perform front-running of the liquidation function by depositing small amounts of tokens to his position, incrementing the user.cooldownExpiration variable. Consequently, the execution of the liquidation function will be reverted with the error message Must wait for the cooldown to expire.

This vulnerability could lead to attackers evading liquidation, potentially causing the system to enter into debt as liquidations are avoided.

## Recommended Mitigation Steps

Consider modifying the liquidation function as follows:

function liquidateUser( address wallet ) external nonReentrant { require( wallet != msg.sender, "Cannot liquidate self" ); // First, make sure that the user's collateral ratio is below the required level require( canUserBeLiquidated(wallet), "User cannot be liquidated" ); uint256 userCollateralAmount = userShareForPool( wallet, collateralPoolID ); // Withdraw the liquidated collateral from the liquidity pool.

// The liquidity is owned by this contract so when it is withdrawn it will be reclaimed by this contract.

(uint256 reclaimedWBTC, uint256 reclaimedWETH) = pools.removeLiquidity(wbtc, weth, userCollateralAmount, 0, 0, totalShares[collateralPoolID] ); // Decrease the user's share of collateral as it has been liquidated and they no longer have it.

-- _decreaseUserShare( wallet, collateralPoolID, userCollateralAmount, true ); ++ _decreaseUserShare( wallet, collateralPoolID, userCollateralAmount, false ); // The caller receives a default 5% of the value of the liquidated collateral.

uint256 rewardPercent = stableConfig.rewardPercentForCallingLiquidation(); uint256 rewardedWBTC = (reclaimedWBTC * rewardPercent) / 100; uint256 rewardedWETH = (reclaimedWETH * rewardPercent) / 100; // Make sure the value of the rewardAmount is not excessive uint256 rewardValue = underlyingTokenValueInUSD( rewardedWBTC, rewardedWETH ); // in 18 decimals uint256 maxRewardValue = stableConfig.maxRewardValueForCallingLiquidation(); // 18 decimals if ( rewardValue > maxRewardValue ) { rewardedWBTC = (rewardedWBTC * maxRewardValue) / rewardValue; rewardedWETH = (rewardedWETH * maxRewardValue) / rewardValue; } // Reward the caller wbtc.safeTransfer( msg.sender, rewardedWBTC ); weth.safeTransfer( msg.sender, rewardedWETH );

// Send the remaining WBTC and WETH to the Liquidizer contract so that the tokens can be converted to USDS and burned (on Liquidizer.performUpkeep) wbtc.safeTransfer( address(liquidizer), reclaimedWBTC - rewardedWBTC ); weth.safeTransfer( address(liquidizer), reclaimedWETH - rewardedWETH ); // Have the Liquidizer contract remember the amount of USDS that will need to be burned.

uint256 originallyBorrowedUSDS = usdsBorrowedByUsers[wallet]; liquidizer.incrementBurnableUSDS(originallyBorrowedUSDS); // Clear the borrowedUSDS for the user who was liquidated so that they can simply keep the USDS they previously borrowed.

usdsBorrowedByUsers[wallet] = 0; _walletsWithBorrowedUSDS.remove(wallet); emit Liquidation(msg.sender, wallet, reclaimedWBTC, reclaimedWETH, originallyBorrowedUSDS); } This modification ensures that the user.cooldownExpiration expiration check does not interfere with the liquidation process, mitigating the identified security risk.

othernet-global (Salty.IO) confirmed and commented:

The stablecoin framework: /stablecoin, /price_feed, WBTC/WETH collateral, PriceAggregator, price feeds and USDS have been removed:

- https://github.com/othernet-global/salty-io/commit/88b7fd1f3f5e037a155424a85275efd79f3e9bf9
Status:

Mitigation confirmed. Full details in reports from zzebra83, 0xpiken, and t0x1c.

# [H-06] When borrowers repay USDS, it is sent to the wrong address, allowing anyone to burn Protocol Owned Liquidity and build bad debt for USDS

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** H-06
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

Submitted by nonseodion, also found by Toshii, lanrebayode77, Aymen0909, KingNFT, juancito, 00xSEV, fnanni, oakcobalt, chaduke, israeladelaja, Ephraim, zach ( 1, 2 ), Drynooo, solmaxis69, pkqs90, wangxx2026, ether_sky, 0x3b, LeoGold, Jorgect, 0xAlix2, 0xRobocop, 0xanmol, djxploit, ayden, and klau5 When a user repays the USDS he has borrowed, it is taken from him and kept for burning. The Liquidizer contract is updated with the new amount repaid. The USDS is burnt whenever the performUpkeep function is called on Liquidizer by the Upkeep contract during upkeep.

The USDS collected is sent to the USDS contract which can be burned whenever burnTokensInContract is called. The amount of USDS to be burnt in the Liquidizer contract is also increased by the incrementBurnableUSDS call. This increases the usdsThatShouldBeBurned variable on the Liquidizer.

function repayUSDS ( uint256 amountRepaid ) external nonReentrant {...

usds.

safeTransferFrom ( msg.

sender, address ( usds ), amountRepaid ); // Have USDS remember that the USDS should be burned liquidizer.

incrementBurnableUSDS ( amountRepaid );...

} During upkeep, the Liquidizer first checks if it has enough USDS balance to burn i.e usdsBalance >= usdsThatShouldBeBurned. If it does it burns them else it converts Protocol Owned Liquidity (POL) to USDS and burns it to cover the deficit. Burning POL allows the protocol to cover bad debt from liquidation.

function _possiblyBurnUSDS () internal {...

uint256 usdsBalance = usds.

balanceOf ( address ( this )); if ( usdsBalance >= usdsThatShouldBeBurned ) { // Burn only up to usdsThatShouldBeBurned.

// Leftover USDS will be kept in this contract in case it needs to be burned later.

_burnUSDS ( usdsThatShouldBeBurned ); usdsThatShouldBeBurned = 0; } else { // The entire usdsBalance will be burned - but there will still be an outstanding balance to burn later _burnUSDS ( usdsBalance ); usdsThatShouldBeBurned -= usdsBalance; // As there is a shortfall in the amount of USDS that can be burned, liquidate some Protocol Owned Liquidity and // send the underlying tokens here to be swapped to USDS dao.

withdrawPOL ( salt, usds, PERCENT_POL_TO_WITHDRAW ); dao.

withdrawPOL ( dai, usds, PERCENT_POL_TO_WITHDRAW ); } Since the usdsThatShouldBeBurned variable will always be increased without increasing the Liquidizer balance, it will always sell POL to cover the increase.

If the POL is exhausted, the protocol cannot cover bad debt generated from liquidations. This will affect the price of USDS negatively.

An attacker can borrow and repay multiple times to exhaust POL and create bad debt or it could just be done over time as users repay their USDS.

## Impact

This will affect the price of USDS negatively.

## Recommended Mitigation Steps

Send the repaid USDS to the Liquidizer.

othernet-global (Salty.IO) confirmed and commented:

The stablecoin framework: /stablecoin, /price_feed, WBTC/WETH collateral, PriceAggregator, price feeds and USDS have been removed:

- https://github.com/othernet-global/salty-io/commit/88b7fd1f3f5e037a155424a85275efd79f3e9bf9
Status:

Mitigation confirmed. Full details in reports from 0xpiken, zzebra83, and t0x1c.

Medium Risk Findings (31)

# [M-01] The user who withdraws liquidity from a particular pool is able to claim more rewards than they should by carefully selecting a decreaseShareAmount value such that the virtualRewardsToRemove is rounded down to zero

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

decreaseShareAmount value such that the virtualRewardsToRemove is rounded down to zero Submitted by Udsen, also found by stackachu, ether_sky, Jorgect, Banditx0x, J4X, DanielArmstrong, santiellena, Draiakoo, and 0xfave

- https://github.com/code-423n4/2024-01-salty/blob/main/src/staking/StakingRewards.sol#L113-L118
- https://github.com/code-423n4/2024-01-salty/blob/main/src/staking/StakingRewards.sol#L132-L133
- https://github.com/code-423n4/2024-01-salty/blob/main/src/staking/StakingRewards.sol#L99
The StakingRewards._decreaseUserShare function is used to decrease a user’s share for the pool and have any pending rewards sent to them. When the amount of pending rewards are calculated, initially the virtualRewardsToRemove are calculated as follows:

uint256 virtualRewardsToRemove = (user.virtualRewards * decreaseShareAmount) / user.userShare; Then the virtualRewardsToRemove is substracted from the rewardsForAmount value to calculate the claimableRewards amount as shown below:

if ( virtualRewardsToRemove < rewardsForAmount ) claimableRewards = rewardsForAmount - virtualRewardsToRemove; But the issue here is that the virtualRewardsToRemove calculation is rounded down in favor of the user and not in the favor of the protocol. Since the virtualRewardsToRemove is rounded down there is an opportunity to the user to call the StakingRewards._decreaseUserShare function with a very small decreaseShareAmount value such that the virtualRewardsToRemove will be rounded down to 0. Providing a very small decreaseShareAmount value is possible since only input validation on decreaseShareAmount is ! = 0 as shown below:

require( decreaseShareAmount != 0, "Cannot decrease zero share" ); When the claimableRewards is calculated it will be equal to the rewardsForAmount value since the virtualRewardsToRemove will be 0. This way the user can keep on removing his liquidity from a particular pool by withdrawing small decreaseShareAmount at a time such that keeping virtualRewardsToRemove at 0 due to rounding down.

Furthermore the decreaseShareAmount value should be selected in such a way rewardsForAmount is calculated to a considerable amount after round down (not zero) and the virtualRewardsToRemove should round down to zero.

Hence as a result the user can withdraw all the rewardsForAmount as the claimableRewards even though some of those rewards are virtual rewards which should not be claimable as clearly stated by the following natspec comment:

// Some of the rewardsForAmount are actually virtualRewards and can't be claimed.

Hence as a result the user is able to get an undue advantage and claim more rewards for his liquidity during liquidity withdrawable. This happens because the user can bypass the virtual reward subtraction by making it round down to 0. As a result the virtualReward amount of the rewardsForAmount, which should not be claimable is also claimed by the user unfairly.

## Recommended Mitigation Steps

Hence it is recommended to round up the virtualRewardsToRemove value during its calculation such that it will not be rounded down to zero for a very small decreaseShareAmount. This way user is unable to claim the rewards which he is not eligible for and the rewards will be claimed after accounting for the virtual rewards.

othernet-global (Salty.IO) confirmed and commented:

virtualRewards now rounded up on _decreaseUserShare

- https://github.com/othernet-global/salty-io/commit/b3b8cb955db2b9f0e47a4964e1e4f833a447a72d
Status:

Mitigated with an Error. Full details in report from t0x1c, and also included in the

# [M-02] Persistent Contract Call revert prevents finalizing a ballot

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

Submitted by vnavascues, also found by ether_sky, 0xRobocop, and haxatron

- https://github.com/code-423n4/2024-01-salty/blob/main/src/dao/DAO.sol#L180
- https://github.com/code-423n4/2024-01-salty/blob/main/src/dao/DAO.sol#L219
The DAO._executeApproval function does not handle an external contract call error:

else if ( ballot.

ballotType == BallotType.

CALL_CONTRACT ) { // @audit-issue unhandled revert ICalledContract ( ballot.

address1 ).

callFromDAO ( ballot.

number1 ); emit ContractCalled ( ballot.

address1, ballot.

number1 ); } Given an approved CALL_CONTRACT ballot that can be finalized, the ballot won’t be marked as finalized if the external contract call (from above) reverts.

function _finalizeApprovalBallot ( uint256 ballotID ) internal { if ( proposals.

ballotIsApproved ( ballotID )) { Ballot memory ballot = proposals.

ballotForID ( ballotID ); _executeApproval ( ballot ); } // @audit-issue the line below won't be executed if `_executeApproval` reverts proposals.

markBallotAsFinalized ( ballotID ); }

## Impact

A permanent revert leaves the ballot unfinalized, and the user that posted it with an active proposal (in the _userHasActiveProposal mapping); a state that prevents the user account from creating a new proposal. At this point the user has two options to sort out the situation:

A. Unstake and transfer its SALT into a new account. B. Convince the other users to reach quorum on NO and finalize the ballot without calling the external contract.

## Recommended Mitigation Steps

A trivial solution is to handle the reverted external contract call with a try..catch and allow to always mark the approved ballot as finalized. A new ballot can always be created if the desired effects of the call were not applied on the first call.

Amend the CALL_CONTRACT case in the DAO._executeApproval function:

else if ( ballot.

ballotType == BallotType.

CALL_CONTRACT ) { try ICalledContract ( ballot.

address1 ).

callFromDAO ( ballot.

number1 ) { // NB: place the emission outside if it must be emitted no matter the external call outcome emit ContractCalled ( ballot.

address1, ballot.

number1 ); } catch ( bytes memory ) {} } Add the following test in DAO.t.sol:

function testCallContractApproveRevertHandled () public { // Arrange vm.

startPrank ( alice ); staking.

stakeSALT ( 1000000 ether ); TestCallReceiverFaulty testReceiver = new TestCallReceiverFaulty (); uint256 ballotID = proposals.

proposeCallContract ( address ( testReceiver ), 123, "description" ); // Act _voteForAndFinalizeBallot ( ballotID, Vote.

YES ); // Assert assertTrue ( testReceiver.

value () != 123, "Receiver shouldn't receive the call" ); assertEq ( proposals.

userHasActiveProposal ( alice ), false, "Alice proposal is not active" ); } othernet-global (Salty.IO) confirmed and commented:

callFromDAO now wrapped in a try/catch

- https://github.com/othernet-global/salty-io/commit/5f1a5206a04b0f3fe45ad88a311370ce12fb0135
Note: For full discussion, see here.

Status:

Mitigation confirmed. Full details in reports from t0x1c, 0xpiken, and [zzebra83].

# [M-03] Creation of token whitelisting proposals can be DOS’d

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

Submitted by falconhoof, also found by josephdara, inzinko ( 1, 2 ), zhaojie, forgebyola, Rhaydden, J4X, BiasedMerc, jesjupyter, and cats

- https://github.com/code-423n4/2024-01-salty/blob/53516c2cdfdfacb662cdea6417c52f23c94d5b5b/src/dao/Proposals.sol#L162-L177
- https://github.com/code-423n4/2024-01-salty/blob/53516c2cdfdfacb662cdea6417c52f23c94d5b5b/src/dao/Proposals.sol#L81-L118
The creation of token whitelisting proposals isn limited to 5 proposals; after which it is DOS’d until proposals are voted on and finalized.

## Vulnerability Details

In proposeTokenWhitelisting() when a proposal is created to whitelist a new token; the ballotId is added to _openBallotsForTokenWhitelisting. There is then a check to ensure that the length of _openBallotsForTokenWhitelisting does not exceed daoConfig.maxPendingTokensForWhitelisting().

Where maxPendingTokensForWhitelisting has been reached, new proposals will not be created for whitelisting tokens. This could happen by accident or by malicious actions on the part of users manipulating the system.

By default maxPendingTokensForWhitelisting is set to 5 but it could be decreased via vote to 3 which would make the issue even more common place.

function proposeTokenWhitelisting( IERC20 token, string calldata tokenIconURL, string calldata description ) external nonReentrant returns (uint256 _ballotID) { // SOME CODE >>> require( _openBallotsForTokenWhitelisting.length() < daoConfig.maxPendingTokensForWhitelisting(), "The maximum number of token whitelisting proposals are already pending" ); // SOME CODE uint256 ballotID = _possiblyCreateProposal( ballotName, BallotType.WHITELIST_TOKEN, address(token), 0, tokenIconURL, description ); _openBallotsForTokenWhitelisting.add( ballotID ); return ballotID; }

## Impact

Malicious users can clog the whitelisting queue with fake token proposals, blocking the addition of genuine tokens and DOSing core functionality of the protocol. This can restrict the ability of the protocol to operate and it’s popularity with users. Although a user can only create one proposal per address at a time; a coordinated group of just five could block the functionality indefinitely.

Tools Used Foundry Recommendations Allow a trusted authority to remove proposals which they deem to be malicious such as proposals for fake tokens.

othernet-global (Salty.IO) confirmed, but disagreed with severity and commented:

Make proposals require a percent of the staking SALT which is set by the DAO. Each user can only make one proposal at a time. Additionally, the default unstaking period for xSALT is 52 weeks and xSALT is non-transferrable.

Picodes (Judge) decreased severity to Medium and commented:

Medium severity seems appropriate here under “function of the protocol or its availability could be impacted”.

othernet-global (Salty.IO) commented:

There is now no limit to the number of tokens that can be proposed for whitelisting.

Also, any whitelisting proposal that has reached quorum with sufficient approval votes can be executed.

- https://github.com/othernet-global/salty-io/commit/ccf4368fcf1777894417fccd2771456f3eeaa81c
Status:

Mitigation confirmed. Full details in reports from zzebra83, 0xpiken, and [t0x1c].

# [M-04] If there is only one USDS borrower, he can never be liquidated

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

Submitted by J4X, also found by 0xHelium, Toshii, 0xpiken, 0xCiphky, israeladelaja, aman ( 1, 2 ), b0g0, 0xRobocop, djxploit, ayden, and 0xWaitress The Salty protocol offers the USDS stablecoin, collateralized by WETH/WBTC. Users can deposit collateral and borrow USDS against it. A key safeguard mechanism is liquidation, which is activated when the collateral value drops below a certain threshold (typically 110%). However, a significant flaw exists in the liquidation process, particularly when there is only one user borrowing USDS.

The process to liquidate a user involves calling the liquidateUser() function, which in turn calls pools.removeLiquidity() to withdraw the user’s collateral. However, the pools.removeLiquidity() function checks if the remaining reserves after withdrawal are below the DUST threshold and reverts if they are.

require (( reserves.

reserve0 >= PoolUtils.

DUST ) && ( reserves.

reserve0 >= PoolUtils.

DUST ), "Insufficient reserves after liquidity removal" ); This check creates a situation where, if all the collateral for USDS is held by a single user, it becomes impossible to liquidate this user. Any attempt to liquidate would reduce the reserves below DUST, causing the transaction to revert

## Impact

This flaw means the only holder of USDS can evade liquidation, potentially leading to bad debt in the protocol that cannot be recouped or mitigated.

## Recommended Mitigation Steps

To resolve this issue, the logic in pools.removeLiquidity() needs to be adjusted. The function should allow the withdrawal of the last collateral, even if it reduces the reserves to zero. This adjustment can be implemented with a conditional check that permits reserves to be either above DUST or exactly zero:

require ((( reserves.

reserve0 >= PoolUtils.

DUST && reserves.

reserve0 >= PoolUtils.

DUST ) || ( reserves.

reserve0 == 0 && reserves.

reserve0 == 0 )), "Insufficient reserves after liquidity removal" ); With this change, the protocol will maintain its ability to liquidate the sole holder of USDS, ensuring the safeguard against bad debt remains effective.

othernet-global (Salty.IO) acknowledged and commented:

The stablecoin framework: /stablecoin, /price_feed, WBTC/WETH collateral, PriceAggregator, price feeds and USDS have been removed:

- https://github.com/othernet-global/salty-io/commit/88b7fd1f3f5e037a155424a85275efd79f3e9bf9
Status:

Mitigation confirmed. Full details in reports from zzebra83, and t0x1c.

# [M-05] Absence of autonomous mechanism for selling collateral assets in the external market in exchange for USDS will cause undercollateralization during market crashes and will cause USDS to depeg

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

selling collateral assets in the external market in exchange for USDS will cause undercollateralization during market crashes and will cause USDS to depeg Submitted by 0xGreyWolf, also found by Toshii, 0x3b, BiasedMerc, and klau5

- https://github.com/code-423n4/2024-01-salty/blob/53516c2cdfdfacb662cdea6417c52f23c94d5b5b/src/Upkeep.sol#L244
- https://github.com/code-423n4/2024-01-salty/blob/53516c2cdfdfacb662cdea6417c52f23c94d5b5b/src/stable/CollateralAndLiquidity.sol#L140
The stablecoin USDS retains its US dollar peg by being overcollateralized. That is true on a bull market. That is also true on a bear market if the liquidation process is faster than the falling prices of the collateral assets.

However during a bear market, there is a scenario where the the price of collateral assets may tank faster than the liquidation process. In this scenario, the total value of the collateral assets of the protocol may end up being lower than the minted / circulating USDS. This will cause undercollateralization and will cause the USDS to depeg.

The depegging will cause the holders of USDS to lose financially. It may cause panic and that will be an existential threat to the protocol.

## Recommended Mitigation Steps

Create a function to sell assets and acquire USDS on external market and just like liquidateUser() and performUpkeep(), reward the users for doing it (calling the function).

If USDS is not available, buy stablecoins USDC and store it for a while to serve as an emergency collateral backing until the market goes back to normal.

othernet-global (Salty.IO) acknowledged and commented:

Note: the overcollateralized stablecoin mechanism has been removed from the DEX.

- https://github.com/othernet-global/salty-io/commit/f3ff64a21449feb60a60c0d60721cfe2c24151c1
Note: the overcollateralized stablecoin mechanism has been removed from the DEX.

- https://github.com/othernet-global/salty-io/commit/f3ff64a21449feb60a60c0d60721cfe2c24151c1
Picodes (Judge) decreased severity to Medium and commented:

Regrouping as duplicates of this issue reports about the fact that the swaps are not atomic so the protocol holds a temporary change risk.

othernet-global (Salty.IO) commented:

The stablecoin framework: /stablecoin, /price_feed, WBTC/WETH collateral, PriceAggregator, price feeds and USDS have been removed:

- https://github.com/othernet-global/salty-io/commit/88b7fd1f3f5e037a155424a85275efd79f3e9bf9
Status:

Mitigation confirmed. Full details in reports from zzebra83, 0xpiken, and [t0x1c].

# [M-06] Reusing a SALT that has already been used for voting can allow a malicious proposal to pass and compromise the protocol

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

Submitted by PENGUN, also found by dutra, vnavascues, falconhoof, grearlake, ReadyPlayer2, Draiakoo, peanuts, zhaojie, Matue, cu5t0mpeo, piyushshukla, 0xanmol, zhanmingjing, and J4X

- https://github.com/code-423n4/2024-01-salty/blob/main/src/dao/Proposals.sol#L259-L293
- https://github.com/code-423n4/2024-01-salty/blob/main/src/dao/Proposals.sol#L385-L400
Reuse of SALT that has already been used for voting could allow a malicious proposal to pass and compromise the protocol.

Details castVote is a function that votes as much SALT as is being staked on the proposal.

function castVote ( uint256 ballotID, Vote vote ) external nonReentrant { Ballot memory ballot = ballots [ ballotID ];...

uint256 userVotingPower = staking.

userShareForPool ( msg.

sender, PoolUtils.

STAKED_SALT ); require ( userVotingPower > 0, "Staked SALT required to vote" ); // Remove any previous votes made by the user on the ballot UserVote memory lastVote = _lastUserVoteForBallot [ ballotID ][ msg.

sender ]; // Undo the last vote?

@> if ( lastVote.

votingPower > 0 ) @> _votesCastForBallot [ ballotID ][ lastVote.

vote ] -= lastVote.

votingPower; // Update the votes cast for the ballot with the user's current voting power _votesCastForBallot [ ballotID ][ vote ] += userVotingPower; // Remember how the user voted in case they change their vote later _lastUserVoteForBallot [ ballotID ][ msg.

sender ] = UserVote ( vote, userVotingPower ); emit VoteCast ( msg.

sender, ballotID, vote, userVotingPower ); } Calling it again on a proposal that has already been voted on will revert the existing vote.

Therefore, the same account cannot vote multiple times on the same proposal. However, it is possible to re-vote by unstaking SALT and transferring it to another account.

// Checks that ballot is live, and minimumEndTime and quorum have both been reached.

function canFinalizeBallot ( uint256 ballotID ) external view returns ( bool ) { Ballot memory ballot = ballots [ ballotID ]; if ( !

ballot.

ballotIsLive ) return false; // Check that the minimum duration has passed @> if ( block.

timestamp < ballot.

ballotMinimumEndTime ) return false; // Check that the required quorum has been reached if ( totalVotesCastForBallot ( ballotID ) < requiredQuorumForBallotType ( ballot.

ballotType )) return false; return true; } The reason this is a viable attack is because the conditions under which a proposal can be finalized are unusual.

In a typical voting system, there is a period of time during which a proposal can be voted on, and if it does not meet the quorum, it is dropped, and if it does, the ratio of upvotes to downvotes determines whether it should be executed.

However, Salty’s voting system allows the voting period to be infinitely long if the quorum is not met. In other words, if the voting lasts longer than the period required to unstake SALT, it can be unstaked and transferred to another account to vote again.

Salty will be launched on the Ethereum mainnet, which means that voting will be quite expensive. With no delegates, the current specification requires many individual votes to achieve a quorum. This means that users may not actively vote for outrageous votes, which could lead to a longer proposal voting period.

The scenario for the attack is as follows Staking SALT Create a malicious proposal Vote YES Unstake SALT Wait for unstake and recover Send SALT to other account Staking SALT Vote YES

## Recommended Mitigation Steps

A short-term solution is to use ballotMaximumEndTime to prevent votes from lasting too long.

A more fundamental solution would be to take a snapshot of the staked SALT and make it available for voting, like ERC20Votes, to prevent re-voting after a transfer.

othernet-global (Salty.IO) confirmed and commented:

ballotMaximumDuration added. There is now a default 30 day period after which ballots can be removed by any user.

- https://github.com/othernet-global/salty-io/commit/758349850a994c305a0ab9a151d00e738a5a45a0
Status:

Mitigation confirmed. Full details in report from zzebra83.

# [M-07] Impossible to change managed wallets with proposeWallets after first rejection

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

proposeWallets after first rejection Submitted by Aymen0909, also found by juancito ( 1, 2 ), J4X, jasonxiale, niroh, neocrao, pina, erosjohn, 0xpiken, lilizhu, oakcobalt, n0kto, ZanyBonzy, pkqs90, Drynooo, zxriptor, Ephraim, LeoGold, 0xRobocop, 0xOmer, klau5, jesjupyter, and Beepidibop

- https://github.com/code-423n4/2024-01-salty/blob/main/src/ManagedWallet.sol#L67-L69
- https://github.com/code-423n4/2024-01-salty/blob/main/src/ManagedWallet.sol#L48-L49
The receive function in ManagedWallet fails to reset the proposedMainWallet address to the zero address ( address(0) ) when the confirmation wallet rejects the wallet change:

receive () external payable { require ( msg.

sender == confirmationWallet, "Invalid sender" ); // Confirm if.05 or more ether is sent and otherwise reject.

// Done this way in case custodial wallets are used as the confirmationWallet - which sometimes won't allow for smart contract calls.

if ( msg.

value >=.05 ether ) activeTimelock = block.

timestamp + TIMELOCK_DURATION; // establish the timelock else activeTimelock = type ( uint256 ).

max; // effectively never //@audit doesn't reset proposedMainWallet to address(0) } This omission renders it impossible to submit new proposals for changing wallets in the future. After the confirmation wallet rejects for the first time, the proposedMainWallet remains different from address(0), causing the proposeWallets function to revert due to the check on the proposedMainWallet address:

function proposeWallets ( address _proposedMainWallet, address _proposedConfirmationWallet ) external {...

//@audit revert if proposedMainWallet != address(0) // Make sure we're not overwriting a previous proposal (as only the confirmationWallet can reject proposals) require ( proposedMainWallet == address ( 0 ), "Cannot overwrite non-zero proposed mainWallet." ); proposedMainWallet = _proposedMainWallet; proposedConfirmationWallet = _proposedConfirmationWallet; emit WalletProposal ( proposedMainWallet, proposedConfirmationWallet ); }

## Impact

After the first rejection of wallet changes, the proposeWallets function will consistently revert, making it impossible to change the main and confirmation wallets indefinitely.

## Recommended Mitigation

In the receive function, reset the proposedMainWallet variable to address(0) after the confirmation process has been rejected (similar to the reset done in changeWallets() ).

if ( msg.

value >=.05 ether ) activeTimelock = block.

timestamp + TIMELOCK_DURATION; // establish the timelock else { //@audit Reset activeTimelock = type ( uint256 ).

max; proposedMainWallet = address ( 0 ); proposedConfirmationWallet = address ( 0 ); } othernet-global (Salty.IO) confirmed and commented:

ManagedWallet has been removed.

- https://github.com/othernet-global/salty-io/commit/5766592880737a5e682bb694a3a79e12926d48a5
Picodes (Judge) decreased severity to Medium Status:

Mitigation confirmed. Full details in reports from t0x1c, 0xpiken, and [zzebra83].

# [M-08] PriceFeed is likely to be disabled in times of volatility, causing liquidations and borrows to freeze

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** M-08
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

Submitted by niroh, also found by oakcobalt

- https://github.com/code-423n4/2024-01-salty/blob/53516c2cdfdfacb662cdea6417c52f23c94d5b5b/src/price_feed/PriceAggregator.sol#L142
- https://github.com/code-423n4/2024-01-salty/blob/53516c2cdfdfacb662cdea6417c52f23c94d5b5b/src/price_feed/PriceAggregator.sol#L183
- https://github.com/code-423n4/2024-01-salty/blob/53516c2cdfdfacb662cdea6417c52f23c94d5b5b/src/price_feed/PriceAggregator.sol#L195
The price feed aggregator relies on three feeds (Chainlink feeds, Uniswap 30 minute TWAP and Salty spot price) to report pricing. The aggregation logic is to average the closest two feed prices, and if the price difference between them exceeds maximumPriceFeedPercentDifferenceTimes1000 (default value: 3%), the aggregator reverts. Since liquidation and borrowing operations rely on price feed reporting, these will revert as well when the minimum price disparity between the three oracles exceeds maximumPriceFeedPercentDifferenceTimes1000.

In times of price volatility for the reported pairs (ETH/USD, BTC/USD) the three oracles used are likely to diverge by more than the 3% limit. To understand why, consider the following traits of each of the three feed sources:

Uniswap 30 minutes TWAP reports the time weighed average over the last 30 minutes.

Chainlink uses multiple sources (onchain Dexes, Cexes) and reports immediately on price changes larger than 0.5% (the Chainlink update trigger setting for the two feeds used.) Salty reports the immediate price taken from the relevant pools on Salty at the time (block) of price aggragation.

When volatility is high, the 30 minutes TWAP is likely to diverge from spot oracles such as Chainlink feeds and Salty’s pools. If for example the market price shifts drastically within 5 minutes, Uniswap’s TWAP will only weigh the change at 20% and the previous price at 80%, while Chainlink and Salty will report the most recent price. In such conditions, a 3% price difference is likely to happen.

Since Salty’s liquidity is likely to be much smaller than the Uniswap pools used (Weth/Wbtc and Weth/Usdc) its reported price is likely to diverge from Uniswap’s price, especially when sharp price shifts occur. This is because large pools such as Uniswaps are likely to be arbitraged sooner (to match centralized exchanges where price discovery typically happens) and smaller pools take longer to catch up to the new price.

The result is that is times of high volatility, Salty’s price feed is likely to enter a state of >3% disparity between the feeds, effetively blocking liquidations at the time they are most crucial.

The maximumPriceFeedPercentDifferenceTimes1000 parameter can be extended up to 7% through a DAO vote, but given that every 0.5% increase requires a vote duration of 10 days at least, the DAO is not likely to react it time to adjust for market volatility.

Tools Used Foundry

## Recommended Mitigation Steps

The common behavior for DeFi price feed aggregators is to fallback to the most trustworthy oracle rather than abort. Typically, Chainlink is selected as the preferred fallback due to the diversity of its sources and reputation of stability. It is recommended to follow this principle and fallback to Chainlink’s price instead of reverting when prices diverge by more than the max.

othernet-global (Salty.IO) acknowledged and commented:

The stablecoin framework: /stablecoin, /price_feed, WBTC/WETH collateral, PriceAggregator, price feeds and USDS have been removed:

- https://github.com/othernet-global/salty-io/commit/88b7fd1f3f5e037a155424a85275efd79f3e9bf9
Status:

Mitigation confirmed. Full details in reports from zzebra83, 0xpiken, and t0x1c.

# [M-09] Remove Liquidity has missing reserve1 DUST check, which can make reserve1 to be less than DUST

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** M-09
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

Submitted by neocrao, also found by jasonxiale, J4X, 0x11singh99, okolicodes, HALITUS, memforvik, Kalyan-Singh, The-Seraphs, wangxx2026, zhaojohnson, 00xSEV, juancito, cu5t0mpeo, parrotAudits0, AgileJune, agadzhalov, aman, Drynooo, ewah, RootKit0xCE, MSaptarshi, Imp, 0xRobocop, 0xAlix2, erosjohn, jesjupyter, 0x3b, rudolph, 0xanmol, ayden, KHOROAMU, 0xSmartContractSamurai, t0x1c, and klau5 The function Pools::removeLiquidity() checks that the reserve0 and reserve1 amounts for a pool are always above or at DUST amount. If after the liquidity is removed, and the reserve0 or reserve1 goes below DUST level, then the transaction should revert.

But, this check is implemented incorrectly. The code that performs this check is as below in the code:

require (( reserves.

reserve0 >= PoolUtils.

DUST ) && ( reserves.

reserve0 >= PoolUtils.

DUST ), "Insufficient reserves after liquidity removal" ); If you notice, the reserve0 is checked twice, instead of the checks being for each reserve0 and reserve1.

## Impact

The documentation states the following reason for the DUST check:

// Make sure that removing liquidity doesn't drive either of the reserves below DUST.

// This is to ensure that ratios remain relatively constant even after a maximum withdrawal.

So, if the reserve1 goes below the DUST amount, then it can imbalance the ratios. Also, once a pool has been established with the reserves, functions like and related to swap() rely on the amounts to be above the DUST amounts. If either of the reserves go below the DUST levels, then these functions will start to revert.

## Recommended Mitigation Steps

The check should be updated to check for reserve1 as well.

- require((reserves.reserve0 >= PoolUtils.DUST) && (reserves.reserve0 >= PoolUtils.DUST), "Insufficient reserves after liquidity removal"); + require((reserves.reserve0 >= PoolUtils.DUST) && (reserves.reserve1 >= PoolUtils.DUST), "Insufficient reserves after liquidity removal"); Picodes (Judge) commented:

Note: this is a typo following a fix of the Trail Of Bits audit. It has a large number of duplicates, with the main impact being DoS of the swaps and related functionalities and manipulating the initial ratio. I haven’t found a report proving that manipulating the initial ratio is of high severity except these 2 that copy-pasted ToB’s report:

- https://github.com/code-423n4/2024-01-salty-findings/issues/197
and

- https://github.com/code-423n4/2024-01-salty-findings/issues/592.

othernet-global (Salty.IO) confirmed and commented:

Fixes reserves DUST check:

- https://github.com/othernet-global/salty-io/commit/b01f6e5cb360e89f9e4cdae41d609ea747bcaa86
Status:

Mitigation confirmed. Full details in reports from t0x1c, 0xpiken, and zzebra83.

# [M-10] Unwhitelisting does not clear _arbitrageProfits, so re-whitelisting may result in an unfair distribution of liquidity rewards

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** M-10
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

Submitted by klau5, also found by falconhoof, Toshii, jasonxiale, 0xCiphky, 0xbepresent ( 1, 2 ), jesjupyter, KingNFT, pkqs90, Topmark, 0xAsen, and nonseodion

- https://github.com/code-423n4/2024-01-salty/blob/aab6bbc6fe49d4dd37becc7bbd0c847ec4a7c1e6/src/dao/DAO.sol#L157-L164
- https://github.com/code-423n4/2024-01-salty/blob/aab6bbc6fe49d4dd37becc7bbd0c847ec4a7c1e6/src/pools/PoolStats.sol#L51-L55
When a pool that has been excluded from the whitelist is added again, it can receive liquidity rewards based on the previous _arbitrageProfits.

## Recommended Mitigation Steps

Before finalizing the unwhitelist ballot, user should call performUpkeep first to force the rewards to be settled. Check if _arbitrageProfits is cleared.

function _executeApproval( Ballot memory ballot ) internal { if ( ballot.ballotType == BallotType.UNWHITELIST_TOKEN ) { + require(pools._arbitrageProfits(poolId_wbtc) == 0, "not cleared yet"); + require(pools._arbitrageProfits(poolId_weth) == 0, "not cleared yet"); // All tokens are paired with both WBTC and WETH so unwhitelist those pools poolsConfig.unwhitelistPool( pools, IERC20(ballot.address1), exchangeConfig.wbtc() ); poolsConfig.unwhitelistPool( pools, IERC20(ballot.address1), exchangeConfig.weth() ); emit UnwhitelistToken(IERC20(ballot.address1)); }...

} othernet-global (Salty.IO) disputed and commented:

It is acceptable that an unwhitelisted pool will retain some rewards after being whitelisted again - as the period in which they were unwhitelisted they did not receive rewards that were owed to them. As the frequency of performUpkeep is sufficiently high, this behavior is acceptable.

# [M-11] SALT staker can get extra voting power by simply unstaking their xSALT

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** M-11
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

Submitted by 0xpiken, also found by Infect3d, Toshii, Aymen0909 ( 1, 2 ), jasonxiale, Draiakoo, zhaojie, solmaxis69, t0x1c, 0xRobocop, 0xBinChook, J4X ( 1, 2 ), klau5, 0xWaitress, cats, and haxatron SALT staker might have chance to finalize the vote by manipulating the required quorum.

## Recommended Mitigation Steps

There are several ways to fix this problem. The simplest one is calculating and storing the required quorum when the proposal is created. Once the required quorum is fixed, no one can change it by unstaking their xSALT:

struct Ballot { uint256 ballotID; bool ballotIsLive; BallotType ballotType; string ballotName; address address1; uint256 number1; string string1; string description; // The earliest timestamp at which a ballot can end. Can be open longer if the quorum has not yet been reached for instance.

uint256 ballotMinimumEndTime; + uint256 requiredQuorum; } function _possiblyCreateProposal( string memory ballotName, BallotType ballotType, address address1, uint256 number1, string memory string1, string memory string2 ) internal returns (uint256 ballotID) { require( block.timestamp >= firstPossibleProposalTimestamp, "Cannot propose ballots within the first 45 days of deployment" ); // The DAO can create confirmation proposals which won't have the below requirements if ( msg.sender != address(exchangeConfig.dao() ) ) { // Make sure that the sender has the minimum amount of xSALT required to make the proposal uint256 totalStaked = staking.totalShares(PoolUtils.STAKED_SALT); uint256 requiredXSalt = ( totalStaked * daoConfig.requiredProposalPercentStakeTimes1000() ) / ( 100 * 1000 );

require( requiredXSalt > 0, "requiredXSalt cannot be zero" ); uint256 userXSalt = staking.userShareForPool( msg.sender, PoolUtils.STAKED_SALT ); require( userXSalt >= requiredXSalt, "Sender does not have enough xSALT to make the proposal" ); // Make sure that the user doesn't already have an active proposal require( ! _userHasActiveProposal[msg.sender], "Users can only have one active proposal at a time" ); } // Make sure that a proposal of the same name is not already open for the ballot require( openBallotsByName[ballotName] == 0, "Cannot create a proposal similar to a ballot that is still open" ); require( openBallotsByName[ string.concat(ballotName, "_confirm")] == 0, "Cannot create a proposal for a ballot with a secondary confirmation" );

uint256 ballotMinimumEndTime = block.timestamp + daoConfig.ballotMinimumDuration(); // Add the new Ballot to storage ballotID = nextBallotID++; - ballots[ballotID] = Ballot( ballotID, true, ballotType, ballotName, address1, number1, string1, string2, ballotMinimumEndTime ); + uint requiredQuorum = requiredQuorumForBallotType(ballotType); + ballots[ballotID] = Ballot( ballotID, true, ballotType, ballotName, address1, number1, string1, string2, ballotMinimumEndTime, requiredQuorum ); openBallotsByName[ballotName] = ballotID; _allOpenBallots.add( ballotID ); // Remember that the user made a proposal _userHasActiveProposal[msg.sender] = true; _usersThatProposedBallots[ballotID] = msg.sender; emit ProposalCreated(ballotID, ballotType, ballotName);

} function canFinalizeBallot( uint256 ballotID ) external view returns (bool) { Ballot memory ballot = ballots[ballotID]; if ( ! ballot.ballotIsLive ) return false; // Check that the minimum duration has passed if (block.timestamp < ballot.ballotMinimumEndTime ) return false; // Check that the required quorum has been reached + if ( totalVotesCastForBallot(ballotID) < requiredQuorumForBallotType( ballot.ballotType )) - if ( totalVotesCastForBallot(ballotID) < ballot.requiredQuorum) return false; return true; } Picodes (Judge) commented:

This shows how without economic penalty, the actual quorum can be quorum / (1 + quorum) if all voters collude to do this.

othernet-global (Salty.IO) confirmed and commented:

Ballots now keep track of their own requiredQuorum at the time they were created.

- https://github.com/othernet-global/salty-io/commit/c46069644739885fa36e84e27e1dd6362b854663
Status:

Mitigation confirmed. Full details in reports from Reports from zzebra83, 0xpiken, and t0x1c.

# [M-12] DOS of proposals by abusing ballot names without important parameters

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** M-12
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

Submitted by juancito, also found by 0xRobocop, pina, DanielArmstrong, zhaojie, 0xCiphky, erosjohn, PENGUN, twcctop, haxatron, J4X, klau5, 0xWaitress, and lanrebayode77

- https://github.com/code-423n4/2024-01-salty/blob/53516c2cdfdfacb662cdea6417c52f23c94d5b5b/src/dao/Proposals.sol#L101-L102
- https://github.com/code-423n4/2024-01-salty/blob/53516c2cdfdfacb662cdea6417c52f23c94d5b5b/src/dao/Proposals.sol#L196
An adversary can prevent legit proposals from being created by using the same ballot name.

Proposals with the same name can’t be created, leading to a DOS for some days until the voting phase ends. This can be done repeatedly, after finalizing the previous malicious proposal and creating a new one.

Impacts for each proposal function:

proposeSendSALT(): DOS of all proposals proposeSetContractAddress(): DOS of specific contract setting by proposing a malicious address proposeCallContract(): DOS of specific contract call by providing a wrong number proposeTokenWhitelisting(): DOS of token whitelisting by providing a fake tokenIconURL All: Prevent the creation of any legit proposal, by providing a fake/malicious description to discourage positive voting Note: This Impact fits into the Attack Ideas: “Any issue that would prevent the DAO from functioning correctly.”

## Vulnerability Details

The main issue is that ballots with the same name revert, and the name doesn’t contain all the important parameters to create the proposal:

// Make sure that a proposal of the same name is not already open for the ballot require ( openBallotsByName [ ballotName ] == 0, "Cannot create a proposal similar to a ballot that is still open" ); Proposals.sol#L101-L102 proposeSendSALT()

## Vulnerability Details

Let’s see for example the “Send SALT” proposal. It always has the same name sendSALT. Despite this appears to be an expected behaviour, it can be exploited by an adversary.

The minimum ballot duration is 3 days, with a default value of 10 days. Given that ballots can’t be finalized before that, an adversary can consistently create malicious proposals to send themselves the SALT token. The proposal will enter the voting period for some days, and when the phase ends, the adversary can finalize it, and immediately create the same proposal.

This will prevent any other legit “Send SALT” proposal from being created.

There are no mechanisms to remove these malicious proposals, or to prevent malicious actors from creating them, nor removing their stake. The cost for the adversary is meaningless, as it requires to execute a tx every few days, and they can still claim rewards from the staked assets needed for the proposals.

proposeSetContractAddress()

## Vulnerability Details

Only the contractName is considered for the ballot name, but not the newAddress.

This means that an attack can be performed to consistently create proposals for a specific contract with a malicious address. This prevents updating the price feeds and the access manager.

proposeCallContract()

## Vulnerability Details

Only the contractName is considered for the ballot name, but not the number with which it will be called.

Same as with the previous attack, an adversary can target a specific contract, and consistently create proposals with a wrong calling number.

proposeTokenWhitelisting()

## Vulnerability Details

The tokenIconURL is missing in the ballot name, so whitelisting proposals can be maliciously created for a specific token with a wrong token icon.

description()

## Vulnerability Details

No proposal includes the description (or its hash) in its ballot name. So an adversary can prevent the creation of the legit proposal, by frontrunning it for example, and change the description to something that users would not vote for.

## Vulnerability Details

section. That test could be extended to all the other mentioned functions with their corresponding impacts.

## Recommended Mitigation Steps

In order to prevent the DOS, ballot names (or some new id variable) should include ALL the attributes of the proposal:

ballotType, address1, number1, string1, and string2. Strings could be hashed, and the whole pack could be hashed as well.

So, if an adversary creates the proposal, it would look exactly the same as the legit one.

In the particular case of proposeSendSALT(), strictly preventing simultaneous proposals as they are right now will lead to the explained DOS. Some other mechanism should be implemented to mitigate risks. One way could be to set a long enough cooldown for each user, so that they can’t repeatedly send these type of proposals (take into account unstake time).

othernet-global (Salty.IO) confirmed and commented:

ballotNames now include all provided proposal arguments.

- https://github.com/othernet-global/salty-io/commit/39921b4a25041c7ac4e9b5279e12bb2ec518140b
Picodes (Judge) commented:

Flagging as duplicate of #621 all issues about the fact that identifying proposals by names that are not sender-specific and do not include all arguments opens the door to being front-run and could lead to a DoS. Not all reports found all the different cases where this could happen but I gave full credit to the one where the impact was Medium.

Status:

Mitigated with an Error. Full details in report from t0x1c, and also included in the

# [M-13] Adversary can prevent updating price feed addresses by creating poisonous proposals ending in _confirm

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** M-13
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

_confirm Submitted by juancito, also found by falconhoof, gkrastenov, memforvik, 0xAsen, jasonxiale, 0x3b, 0xpiken, Ward, Limbooo, israeladelaja, 0xPluto, t0x1c, a3yip6, PENGUN, haxatron, miaowu, Myrault, b0g0, linmiaomiao, nonseodion, 0xAlix2 ( 1, 2 ), and y4y An adversary can prevent the creation of setContract and websiteUpdate proposals by creating a poisonous proposal with the same ballot name + _confirm. This attack can be performed repeatedly every few days (when the voting period ends).

Affected proposals:

setContract:priceFeed1 -> setContract:priceFeed1_confirm setContract:priceFeed2 -> setContract:priceFeed2_confirm setContract:priceFeed3 -> setContract:priceFeed3_confirm setContract:accessManager -> setContract:accessManager_confirm setURL:{{newWebsiteURL}} -> setURL:{{newWebsiteURL}}_confirm This is especially worrisome for the setContract proposals, as it prevents changing the price feed contracts, which are used for USDS borrowing and liquidations.

Note: This Impact fits into the Attack Ideas: “Any issue that would prevent the DAO from functioning correctly.”

## Vulnerability Details

setContract and websiteUpdate proposals are executed in multiple steps.

Here’s the process for changing the Price Feed 1, as an example:

A new proposal is created with a ballot name setContract:priceFeed1 The proposal is voted, and when it wins, a confirmation proposal is created, appending _confirm to the ballot name, resulting in setContract:priceFeed1_confirm.

When the confirmation proposal wins, it checks its ballot name and then it sets the new price feed.

The problem is that there is a check in _possiblyCreateProposal() that can be exploited:

require ( openBallotsByName [ string.

concat ( ballotName, "_confirm" )] == 0, "Cannot create a proposal for a ballot with a secondary confirmation" ); Proposals.sol#L103 This check is intended to prevent creating new proposals if there is a pending confirmation proposal being voted for the same change, but an adversary can use it to create a proposal with a ballot name setContract:priceFeed1_confirm, by setting priceFeed1_confirm as the contract name.

If someone tries to create a legit priceFeed1 proposal later, it will revert, leading to a DOS.

This holds for all proposals mentioned in the

## Impact

section.

## Recommended Mitigation Steps

Given the current implementation, the most straightforward fix would be to prevent the creation of proposals with ballot names ending in _confirm for proposals that need a confirmation.

This would mean checking the contractName in proposeSetContractAddress(), and the newWebsiteURL in proposeWebsiteUpdate().

But, as a recommendation, I would suggest refactoring createConfirmationProposal() to pass a “confirmation” parameter to _possiblyCreateProposal(), so that confirmation proposals don’t rely on the ballot name.

othernet-global (Salty.IO) confirmed and commented:

confirm_ is now prepended to automatic confirmation ballots form setWebsiteURL and setContract proposals.

- https://github.com/othernet-global/salty-io/commit/5aa1bc1ddadd67cd875de932633948af25ff8957
The stablecoin framework: /stablecoin, /price_feed, WBTC/WETH collateral, PriceAggregator, price feeds and USDS have been removed:

- https://github.com/othernet-global/salty-io/commit/88b7fd1f3f5e037a155424a85275efd79f3e9bf9
Picodes (Judge) commented:

Here assets are not at direct risk but a “function of the protocol or its availability could be impacted” Note: For full discussion, see here.

Status:

Mitigation confirmed. Full details in reports from 0xpiken, zzebra83, and t0x1c.

# [M-14] Ballots not yet past their deadline are incorrectly looped too by tokenWhitelistingBallotWithTheMostVotes()

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** M-14
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

Submitted by t0x1c Inside DAO.sol, _finalizeTokenWhitelisting() before finalizing a token whitelisting proposal makes a check that it’s the ballot with most votes:

249 // Fail to whitelist for now if this isn't the whitelisting proposal with the most votes - can try again later.

250 uint256 bestWhitelistingBallotID = proposals.

tokenWhitelistingBallotWithTheMostVotes (); 251 require ( bestWhitelistingBallotID == ballotID, "Only the token whitelisting ballot with the most votes can be finalized" ); The tokenWhitelistingBallotWithTheMostVotes() function however does not care if the other ballots being looped through are yet not past their deadline. This is an incorrect approach because a ballot which has more Yes votes at this point of time may well turn into a rejected ballot by its deadline timestamp if additional No votes are cast in coming days. Hence using its current state to deny whitelisting of a proposal past its deadline is unfair & only contributes to delaying the timelines, since this may happen again & again which would result in the protocol repeatedly pushing the finalization time further & further away into the future.

Either only ballots past their deadline should be considered, OR Only compare ballots which were created within a few hours of each other Explanation through an example scenario (also provided in the form of
## Recommended Mitigation Steps

Either only ballots past their deadline should be considered, OR Only compare ballots which were created within a few hours of each other.

The following diff uses the first option:

// Returns the ballotID of the whitelisting ballot that currently has the most yes votes // Requires that the quorum has been reached and that the number of yes votes is greater than the number no votes function tokenWhitelistingBallotWithTheMostVotes() external view returns (uint256) { uint256 quorum = requiredQuorumForBallotType( BallotType.WHITELIST_TOKEN); uint256 bestID = 0; uint256 mostYes = 0; for( uint256 i = 0; i < _openBallotsForTokenWhitelisting.length(); i++ ) { uint256 ballotID = _openBallotsForTokenWhitelisting.at(i); + if (block.timestamp < ballots[ballotID].ballotMinimumEndTime) + continue; uint256 yesTotal = _votesCastForBallot[ballotID][Vote.YES]; uint256 noTotal = _votesCastForBallot[ballotID][Vote.NO];

if ( (yesTotal + noTotal) >= quorum ) // Make sure that quorum has been reached if ( yesTotal > noTotal ) // Make sure the token vote is favorable if ( yesTotal > mostYes ) // Make sure these are the most yes votes seen { bestID = ballotID; mostYes = yesTotal; } return bestID; } othernet-global (Salty.IO) confirmed and commented:

Removed maxPendingTokensForWhitelisting.

There is now no limit to the number of tokens that can be proposed for whitelisting.

Also, any whitelisting proposal that has reached quorum with sufficient approval votes can be executed.

- https://github.com/othernet-global/salty-io/commit/ccf4368
Status:

Mitigation confirmed. Full details in reports from 0xpiken, zzebra83, and t0x1c.

# [M-15] Attacker can take advantage of Chainlink price not occuring within it’s 60 minute heartbeat to make PriceAggregator calls fail

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** M-15
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

Submitted by israeladelaja, also found by jasonxiale, VAD37, and PENGUN

- https://github.com/code-423n4/2024-01-salty/blob/main/src/price_feed/CoreChainlinkFeed.sol#L45
- https://github.com/code-423n4/2024-01-salty/blob/main/src/price_feed/CoreSaltyFeed.sol#L34
- https://github.com/code-423n4/2024-01-salty/blob/main/src/price_feed/CoreSaltyFeed.sol#L47
Salty.IO relies on three default price feeds to get the price of BTC and ETH for the price of the collateral backing USDS. In the CoreChainlinkFeed contract, a price of 0 is returned when the Chainlink price update has not occurred within it’s 60 minute heartbeat. When this happens, the other two price feeds being the Uniswap V3 TWAP and the Salty.IO Reserves would provide the necessary price feed data. However, using the reserves of a liquidity pool directly for price data is dangerous as a user can skew the ratio of the tokens in the pool by simply swapping one for the other. This issue becomes more concerning when flash loans are involved, giving anyone a large amount of tokens temporarily to significantly skew the ratio of the pool. This means that when the Chainlink price update has not occurred within it’s 60 minute heartbeat, an attacker can skew the ratio of the Salty.IO Reserves and artificially change the price of BTC or ETH in their respective pools paired with USDS. This will inevitably make the

PriceAggregator contract revert when price data is needed (because there are two non zero price feeds and the difference between these prices is too large).

## Recommended Mitigation Steps

A TWAP for the Salty.IO Reserves would be recommended to smooth off any significant price movement and decrease the chance of there being a significant deviation from the real world price.

Picodes (Judge) decreased severity to Medium and commented:

Medium severity is appropriate as a protocol’s functionality is broken but the reports doesn’t show how to extract funds using this.

othernet-global (Salty.IO) confirmed and commented:

Chainlink timeout now set to 65 minutes:

- https://github.com/othernet-global/salty-io/commit/f9a830c61e77a22722a8e674a8affabe2a0cf04a
The stablecoin framework: /stablecoin, /price_feed, WBTC/WETH collateral, PriceAggregator, price feeds and USDS have been removed:

- https://github.com/othernet-global/salty-io/commit/88b7fd1f3f5e037a155424a85275efd79f3e9bf9
Status:

Mitigation confirmed. Full details in reports from t0x1c, zzebra83, and 0xpiken.

# [M-16] Suboptimal arbitrage implementation

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** M-16
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

Submitted by fnanni, also found by t0x1c The bestArbAmountIn estimated in _bisectionSearch() can be calculated with a simple formula. Roughly estimating bestArbAmountIn instead of deriving its exact value has the following consequences:

In some scenarios, arbitrage profits are missed completely.

In most cases, arbitrage profits are not optimal.

It could be profitable to sandwich-attack certain swaps. The pre-transaction would push the pools into scenario 1., then the post-transaction would recover the investment + arbitrage, capturing the arbitrage profits that were meant for the DAO.

This issue may look like a mere optimization. However, if the math presented next is correct, I’d argue that this is a medium issue. Built-in arbitrage is the main feature and competitive advantage of Salty.IO. Missing arbitrage profits due to a flawed implementation should not happen.

## Recommended Mitigation Steps

Consider replacing _bisectionSearch() with something similar to computeBestArbitrage(). Beware that computeBestArbitrage() is not overflow-proof.

othernet-global (Salty.IO) confirmed and commented:

Works great! Thank you!

I modified the calculations to reduce overflow risk:

uint256 n0 = A0 * B0 * C0; uint256 n1 = A1 * B1 * C1; if (n1 <= n0) return 0; uint256 m = A1 * ( B1 + C0 ) + C0 * B0; uint256 z = PoolMath._sqrt( (n0 / m) * (n1 / m) ); bestArbAmountIn = z - n0 / m; Added an MSB shift to prevent overflow:

- https://github.com/othernet-global/salty-io/commit/a54656dd18135ca57eef7c4bf615b7cdff2613a7
- https://github.com/othernet-global/salty-io/commit/53feaeb0d335bd33803f98db022871b48b3f2454
uint256 maximumMSB = _maximumReservesMSB( A0, A1, B0, B1, C0, C1 ); // Assumes the largest number should use no more than 80 bits.

// Multiplying three 80 bit numbers will yield 240 bits - within the 256 bit limit.

uint256 shift = 0; if ( maximumMSB > 80 ) { shift = maximumMSB - 80; A0 = A0 >> shift; A1 = A1 >> shift; B0 = B0 >> shift; B1 = B1 >> shift; C0 = C0 >> shift; C1 = C1 >> shift; } // Each variable will use less than 80 bits uint256 n0 = A0 * B0 * C0; uint256 n1 = A1 * B1 * C1; if (n1 <= n0) return 0; uint256 m = A1 * B1 + C0 * ( B0 + A1 ); // Calculating n0 * n1 directly would overflow under some situations.

// Multiply the sqrt's instead - effectively keeping the max size the same uint256 z = Math.sqrt(n0) * Math.sqrt(n1); bestArbAmountIn = ( z - n0 ) / m; Picodes (Judge) commented:

Considering the value-added for the sponsor and the fact that this report:

Is actually saving funds for the protocol Shows that a functionality is in some specific case broken (which we know as the bisection search isn’t exact) could be fixed relatively easily I think Med severity is appropriate.

Status:

Mitigated with an Error. Full details in report from zzebra83, and also included in the

# [M-17] Caller of Upkeep may skip step 11 to save gas

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** M-17
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

Submitted by handsomegiraffe In Upkeep.sol, performUpkeep() is expected to be called by anyone who wishes to earn the 5% incentive. The function runs through steps 1 to 11, each wrapped in a try/catch block to prevent reversions from blocking the entire function.

A dishonest user may call performUpkeep but provide only sufficient gas for steps 1 to 10. Step 11 would revert and the user still receives the incentive for performing the upkeep.

This attack is possible due to the EIP150 rule where 63/64 of gas is forwarded to an external call. If insufficient gas is sent to complete step 11, the remaining 1/64 gas could still be sufficient to emit the error and continue without reverting.

try this.

step11 () {} catch ( bytes memory error ) { emit UpkeepError ( "Step 11", error ); }

## Impact

The user benefits from gas saved by not having to run step 11. Step 11 sends SALT from the team vesting wallet to the team; skipping this step this could cause issues for the team by not receiving expected SALT at each upkeep.

## Recommended Mitigation Steps

Check that sufficient gas is sent at the start of the function call.

othernet-global (Salty.IO) acknowledged Picodes (Judge) commented:

This report shows how a user could potentially be rewarded and not execute the tasks correctly, so would essentially steal funds from the DAO.

# [M-18] _getUniswapTwapWei() will show incorrect price for negative ticks cause it doesn’t round up for negative ticks

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** M-18
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

_getUniswapTwapWei() will show incorrect price for negative ticks cause it doesn’t round up for negative ticks Submitted by Bauchibred, also found by grearlake Take a look at

- https://github.com/code-423n4/2024-01-salty/blob/f742b554e18ae1a07cb8d4617ec8aa50db037c1c/src/price_feed/CoreUniswapFeed.sol#L49-L75
// Returns amount of token0 * (10**18) given token1 function _getUniswapTwapWei ( IUniswapV3Pool pool, uint256 twapInterval ) public view returns ( uint256 ) { uint32 [] memory secondsAgo = new uint32 []( 2 ); secondsAgo [ 0 ] = uint32 ( twapInterval ); // from (before) secondsAgo [ 1 ] = 0; // to (now) // Get the historical tick data using the observe() function ( int56 [] memory tickCumulatives, ) = pool.

observe ( secondsAgo ); //@audit int24 tick = int24 (( tickCumulatives [ 1 ] - tickCumulatives [ 0 ]) / int56 ( uint56 ( twapInterval ))); uint160 sqrtPriceX96 = TickMath.

getSqrtRatioAtTick ( tick ); uint256 p = FullMath.

mulDiv ( sqrtPriceX96, sqrtPriceX96, FixedPoint96.

Q96 ); uint8 decimals0 = ( ERC20 ( pool.

token0 () ) ).

decimals (); uint8 decimals1 = ( ERC20 ( pool.

token1 () ) ).

decimals (); if ( decimals1 > decimals0 ) return FullMath.

mulDiv ( 10 ** ( 18 + decimals1 - decimals0 ), FixedPoint96.

Q96, p ); if ( decimals0 > decimals1 ) return ( FixedPoint96.

Q96 * ( 10 ** 18 ) ) / ( p * ( 10 ** ( decimals0 - decimals1 ) ) ); return ( FixedPoint96.

Q96 * ( 10 ** 18 ) ) / p; } This function is used to get twap price tick using uniswap oracle. it uses pool.observe() to get tickCumulatives array which is then used to calculate int24 tick.

The problem is that in case if int24(tickCumulatives[1] - tickCumulatives[0]) is negative, then the tick should be rounded down as it’s done in the uniswap library.

As result, in case if int24(tickCumulatives[1] - tickCumulatives[0]) is negative and (tickCumulatives[1] - tickCumulatives[0]) % secondsAgo != 0, then returned tick will be bigger then it should be, which opens possibility for some price manipulations and arbitrage opportunities.

## Impact

In case if int24(tickCumulatives[1] - tickCumulatives[0]) is negative and ((tickCumulatives[1] - tickCumulatives[0]) % secondsAgo != 0, then returned tick will be bigger than it should be which places protocol wanting prices to be right not be able to achieve this goal, note that where as protocol still relies on multiple sources of price, they still come down and end on weighing the differences between the prices and reverting if a certain limit is passed, effectively causing the pricing logic to be unavailable and also reverting on important functions like CollateralAndLiquidity::liquidate() cause a call to underlyingTokenValueInUSD() is made which would not be available.

## Recommended Mitigation Steps

Add this line:

if (tickCumulatives[1] - tickCumulatives[0] < 0 && (tickCumulatives[1] - tickCumulatives[0]) % secondsAgo != 0) timeWeightedTick --; othernet-global (Salty.IO) confirmed and commented:

Now rounds down for negative ticks as suggested.

- https://github.com/othernet-global/salty-io/commit/4625393e9bd010778003a1424201513885068800
The stablecoin framework: /stablecoin, /price_feed, WBTC/WETH collateral, PriceAggregator, price feeds and USDS have been removed:

- https://github.com/othernet-global/salty-io/commit/88b7fd1f3f5e037a155424a85275efd79f3e9bf9
Status:

Mitigation confirmed. Full details in reports from t0x1c, zzebra83, and 0xpiken.

# [M-19] No proposal time limit traps sponsors of unpopular proposals

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** M-19
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

Submitted by 0xBinChook, also found by 0x3b, 0xRobocop, ether_sky, pina, Tripathi, 0xpiken, juancito, SpicyMeatball, erosjohn, fnanni, and cats

- https://github.com/code-423n4/2024-01-salty/blob/53516c2cdfdfacb662cdea6417c52f23c94d5b5b/src/dao/Proposals.sol#L396-L397
- https://github.com/code-423n4/2024-01-salty/blob/53516c2cdfdfacb662cdea6417c52f23c94d5b5b/src/dao/DAO.sol#L281-L281
A staker is restricted to sponsoring a single proposal at any given time. This proposal remains active until a predetermined duration has elapsed, and it has received the sufficient number of votes to reach a quorum, after which it can be finalized.

However, if a staker sponsors a proposal that fails to attract the necessary quorum of votes, it remains indefinitely in an active state. This situation effectively locks the staker in a position where they cannot propose any new initiatives, as they are stuck with an unresolved proposal.

There is no mechanism for sponsors to withdraw or cancel their proposals. So when a proposal is unable to achieve quorum, the sponsor is left in a predicament where they are unable to further participate in the governance through the initiation of new proposals.

Furthermore, there is a noticeable lack of motivation for stakers to vote against proposals that are not on track to meet the quorum. As these proposals cannot pass without achieving the required quorum, resulting in a situation where voting against such proposals does not offer any tangible benefit.

## Recommended Mitigation Steps

Allow the proposals to be closed (equivalent to finalized as NO or NO_CHANGE ), which would allow the sponsor to afterward make a different proposal.

(This feature would also generally allow removing dead proposals) Add a time field to Ballot in IProposals:

// The earliest timestamp at which a ballot can end. Can be open longer if the quorum has not yet been reached for instance.

uint256 ballotMinimumEndTime; + // The earliest timestamp at which a ballot can be closed without quorum being reached.

+ uint256 ballotCloseTime; } Populate the ballotCloseTime in Proposal::_possiblyCreateProposal, using a constant in this example, it could always another DAO configuration option:

// Make sure that a proposal of the same name is not already open for the ballot require( openBallotsByName[ballotName] == 0, "Cannot create a proposal similar to a ballot that is still open" ); require( openBallotsByName[ string.concat(ballotName, "_confirm")] == 0, "Cannot create a proposal for a ballot with a secondary confirmation" ); uint256 ballotMinimumEndTime = block.timestamp + daoConfig.ballotMinimumDuration(); + uint256 ballotCloseTime = ballotMinimumEndTime + 1 weeks; // Add the new Ballot to storage ballotID = nextBallotID++; + ballots[ballotID] = Ballot( ballotID, true, ballotType, ballotName, address1, number1, string1, string2, ballotMinimumEndTime ); + ballots[ballotID] = Ballot( ballotID, true, ballotType, ballotName, address1, number1, string1, string2, ballotMinimumEndTime, ballotCloseTime );

openBallotsByName[ballotName] = ballotID; _allOpenBallots.add( ballotID ); Add a function to return whether a proposal can be closed to Proposal:

+ function canCloseBallot( uint256 ballotID ) external view returns (bool) + { + Ballot memory ballot = ballots[ballotID]; + if ( ! ballot.ballotIsLive ) + return false; + + // Check that the minimum duration has passed + if (block.timestamp < ballot.ballotCloseTime ) + return false; + + return true; + } Add a function to close a ballot without any side effect to DAO + function closeBallot( uint256 ballotID ) external nonReentrant + { + // Checks that ballot is live and closeTime has passed + require( proposals.canCloseBallot(ballotID), "The ballot is not yet able to be closed" ); + + // No mutation from the propsal + _finalizeApprovalBallot(ballotID); + } othernet-global (Salty.IO) confirmed and commented:

There is now a default 30 day period after which ballots can be removed by any user.

- https://github.com/othernet-global/salty-io/commit/758349850a994c305a0ab9a151d00e738a5a45a0
Status:

Mitigated with an Error. Full details in report from 0xpiken, and also included in the

# [M-20] Some rewards from POL will not be send to team wallet nor burned

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** M-20
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

Submitted by 0xRobocop, also found by klau5 The rewards earned from the DAOs POL are distributed among the team wallet, then part of the remaining rewards are burned and the rest are kept as DAOs balance.

The issue, is that is possible for the DAO to claim SALT rewards without sending the team’s share to the team wallet and without burning the amount that should be burned.

## Recommended Mitigation Steps

Two options:

Save the amount of rewards received when withdrawing some POL, so they can be distributed and burned.

Make the call to claim all the rewards at the beginning of upkeep.

Picodes (Judge) commented:

This report shows how in some cases some rewards may end up being stuck when withdrawing PoL.

othernet-global (Salty.IO) acknowledged and commented:

POL has been removed from the protocol:

eaf40ef0fa27314c6e674db6830990df68e5d70e

- https://github.com/othernet-global/salty-io/commit/8e3231d3f444e9851881d642d6dd03021fade5ed
Status:

Mitigation confirmed. Full details in reports from t0x1c, zzebra83, and 0xpiken.

# [M-21] When forming POL the DAO will end up stucked with DAI and USDS tokens that cannot handle

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** M-21
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

Submitted by 0xRobocop, also found by oakcobalt, deepplus, DanielArmstrong, and KupiaSec The DAO contract cannot handle any token beside SALT tokens. So, if tokens like USDS or DAI were in its balance, they will be lost forever.

This can happen during the upkeep calls. Basically, during upkeep the contract takes some percentage from the arbitrage profits and use them to form POL for the DAO (usds/dai and salt/usds). The DAO swaps the ETH for both of the needed tokens and then adds the liquidity using the zapping flag to true.

- https://github.com/code-423n4/2024-01-salty/blob/53516c2cdfdfacb662cdea6417c52f23c94d5b5b/src/dao/DAO.sol#L316-L324
Zapping will compute the amount of either tokenA or tokenB to swap in order to add liquidity at the final ratio of reserves after the swap. But, it is important to note that the zap computations do no take into account that the same pool may get arbitraged atomically, changing the ratio of reserves a little.

As a consequence, some of the USDS and DAI tokens will be send back to the DAO contract:

- https://github.com/code-423n4/2024-01-salty/blob/53516c2cdfdfacb662cdea6417c52f23c94d5b5b/src/staking/Liquidity.sol#L110C3-L115C1

## Recommended Mitigation Steps

The leftovers of USDS or DAI should be send to liquidizer so they can be handled.

othernet-global (Salty.IO) commented:

The DAO contract uses the available token balances to form POL, ensuring no extra tokens left in the contract.

- https://github.com/othernet-global/salty-io/commit/5364426aaf97e646fa3990f148e364167adcd0a5
POL has been removed from the protocol:

eaf40ef0fa27314c6e674db6830990df68e5d70e

- https://github.com/othernet-global/salty-io/commit/8e3231d3f444e9851881d642d6dd03021fade5ed
Status:

Mitigation confirmed. Full details in reports from 0xpiken, zzebra83, and t0x1c.

# [M-22] Minimium Collateral Check Can Be Bypassed

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** M-22
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

Submitted by Banditx0x, also found by Audinarey, Giorgio, and t0x1c The minimum collateral is enforced to prevent a well known vulnerability - there is no incentive to liquidate small loans. Therefore the impact of this check being bypassed is the same - small loans will not be liquidated which can lead to bad debt in the protocol.

## Recommended Mitigation Steps

The minimum collateral should be enforced on withdrawals whenever a user has an active USDS loan.

Picodes (Judge) decreased severity to Medium and commented:

To me medium severity is more appropriate here under “leak value with a hypothetical attack path with stated assumptions, but external requirements” and “broken functionality”.

Indeed this could lead to bad debt but the attack is hardly profitable at any point in time as the gas costs to setup small loans is expensive (you need to add collateral, borrow, repay, withdraw) and unless the oracle is flawed you can’t have a guarantee that the attack will profitable.

othernet-global (Salty.IO) acknowledged and commented:

The stablecoin framework: /stablecoin, /price_feed, WBTC/WETH collateral, PriceAggregator, price feeds and USDS have been removed:

- https://github.com/othernet-global/salty-io/commit/88b7fd1f3f5e037a155424a85275efd79f3e9bf9
Status:

Mitigation confirmed. Full details in reports from t0x1c, zzebra83, and 0xpiken.

# [M-23] StakingRewards pools are not given their promised share of rewards due to incorrect calculation

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** M-23
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

Submitted by t0x1c, also found by Banditx0x and 0xAsen RewardsEmitter::performUpkeep() distributes the added rewards to the eligible staking pools at the rate of X% per day (default value set to 1% by the protocol). To ensure that once disbursed, it is not sent again, the code reduces the pendingRewards[poolID] variable on L126:

120 // Each pool will send a percentage of the pending rewards based on the time elapsed since the last send 121 uint256 amountToAddForPool = ( pendingRewards [ poolID ] * numeratorMult ) / denominatorMult; 122 123 // Reduce the pending rewards so they are not sent again 124 if ( amountToAddForPool != 0 ) 125 { 126 pendingRewards [ poolID ] -= amountToAddForPool; 127 128 sum += amountToAddForPool; 129 } The impact of this is:

higher the number of times performUpkeep() is called, lesser the rewards per day is distributed to the pools.

That is, calling it 100 times a day is worse than calling it once at the end of the day. This is at odds with how the protocol wants to achieve a higher frequency of upkeep-ing.

Reasoning:

This above code logic is incorrect maths as doing this will result in the following scenario:

Suppose that on Day0, the addedRewards = 10 ether and rewardsEmitterDailyPercentTimes1000 = 2500 i.e. 2.5% per day. One would expect all rewards to be distributed to the pool after 40 days (since 2.5% * 40 = 100%).

On Day1, performUpkeep() gets called.

amountToAddForPool and pendingRewards[poolID] are calculated on L121 & L126 respectively as:

amountToAddForPool = 0.025 * 10 ether = 0.25 ether pendingRewards[poolID] = 10 ether - 0.25 ether = 9.75 ether On Day2, performUpkeep() gets called again.

amountToAddForPool and pendingRewards[poolID] are calculated now as:

amountToAddForPool = 0.025 * 9.75 ether = 0.24375 ether pendingRewards[poolID] = 9.75 ether - 0.24375 ether = 9.50625 ether So on and so forth for each new day. The actual formula being followed is totalRewardsStillRemainingAfterXdays = 10 ether * (1 - 2.5%)**X which would be 3632324398878806621 or 3.63232 ether after 40 days. In fact, even after another 40 days, it’s still not all paid out. Please refer the

## Recommended Mitigation Steps

Store the eligible pool reward & the already paid out reward in separate variables and compare them to make sure extra rewards are not being paid out to a pool. Irrespective of the performUpkeep() calling frequency, at the end of the day, 1% should be disbursed.

Picodes (Judge) decreased severity to Medium and commented:

Medium severity seems more appropriate considering this only concerns rewards and is subject to external conditions.

othernet-global (Salty.IO) acknowledged

# [M-24] Salt Rewards - Rewards related to Arbitrage profits for pools can be lost

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** M-24
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

Submitted by zzebra83 Arbitrage profits are distributed to pools that played a part in generating them. This is distributed via calling the performUpkeep function with upkeep.sol.

The process of upkeep is multi step, and any failure in any step does not disable next steps to trigger because each step is wrapped in a try catch. lets delve into steps 5 and 7.

// 5. Convert remaining WETH to SALT and sends it to SaltRewards.

function step5() public onlySameContract { uint256 wethBalance = weth.balanceOf( address(this) ); if ( wethBalance == 0 ) return; // Convert remaining WETH to SALT and send it to SaltRewards // @audit arbitrage profits sent to saltrewards here, pools contract // has allowance to withdrawm unlimited WETH from the upkeep contract // but this swap operation can fail, if arbitrage profits are too large // and not enough reserves of salt are there uint256 amountSALT = pools.depositSwapWithdraw( weth, salt, wethBalance, 0, block.timestamp ); salt.safeTransfer(address(saltRewards), amountSALT); } step 5 converts the arbitrage profits from WETH to Salt. Next, after which they are distributed to SaltRewards contract. If the operation to swap WETH to Salt fails due to reserves going below dust for example, then no Salt will be transferred to the salt rewards contract. this will impact step 7.

// 7. Distribute SALT from SaltRewards to the stakingRewardsEmitter and liquidityRewardsEmitter.

function step7() public onlySameContract { // @audit line below can return a list with arbitrage profits assigned for a pool uint256[] memory profitsForPools = pools.profitsForWhitelistedPools(); bytes32[] memory poolIDs = poolsConfig.whitelistedPools(); // @audit if more than 1 week passed, less rewards passed to the emitters in step 8.

saltRewards.performUpkeep(poolIDs, profitsForPools ); // @audit can arbitrage profits be cleared without distributing?

pools.clearProfitsForPools(); } Step 6 will distribute the salt emissions to the saltrewards contract. so the saltrewards contract will have only salt related to emissions but not arbitrage profits.

Step 7 will distribute all salt rewards(including those arbitrage profits in Salt to all the pools) via calling performUpkeep function in salt rewards contrat, but this assumes the SaltRewards contract will have a salt balance containing the arbitrage profits (from step 5), which might not be the case.

uint256 saltRewardsToDistribute = salt.balanceOf(address(this)); if ( saltRewardsToDistribute == 0 ){ // @audit function simply returns, this could be problematic return; } // Determine the total profits so we can calculate proportional share for the liquidity rewards uint256 totalProfits = 0; for( uint256 i = 0; i < poolIDs.length; i++ ) { totalProfits += profitsForPools[i]; } // Make sure that there are some profits to determine the proportional liquidity rewards.

// Otherwise just handle the SALT balance later so it can be divided between stakingRewardsEmitter and liquidityRewardsEmitter without further accounting.

if ( totalProfits == 0 ) { return; } // Determine how much of the SALT rewards will be directly awarded to the SALT/USDS pool.

// This is because SALT/USDS is important, but not included in other arbitrage trades - which would normally yield additional rewards for the pool by being part of arbitrage swaps.

uint256 directRewardsForSaltUSDS = ( saltRewardsToDistribute * rewardsConfig.percentRewardsSaltUSDS() ) / 100; uint256 remainingRewards = saltRewardsToDistribute - directRewardsForSaltUSDS; // Divide up the remaining rewards between SALT stakers and liquidity providers uint256 stakingRewardsAmount = ( remainingRewards * rewardsConfig.stakingRewardsPercent() ) / 100; uint256 liquidityRewardsAmount = remainingRewards - stakingRewardsAmount; _sendStakingRewards(stakingRewardsAmount); _sendLiquidityRewards(liquidityRewardsAmount, directRewardsForSaltUSDS, poolIDs, profitsForPools, totalProfits); // @audit salt rewards for liquidity will not include amounts related to pool arbitrate profits As you can see above, the performupkeep checks the contracts salt balance and based off that the liquidity rewards amounts are determined. but given that actual balance does not include those profits, these figures will be inaccurate.

It would then return back to step 7 and ‘Clear’ the profits assigned to each pool via calling pools.clearProfitsForPools().

This essentially means that profits assigned to pools are cleared from storage without actually being distributed fairly as Salt rewards to pool liquidity providers.

The likelihood of this bug occuring is low to medium, because it is dependant on failure in the WETH to Salt swap in step 5 which might occur if the pool was maliciously targeted or in extreme market conditions and volatility. However the impact is medium to high since pool liquidity providers will most certainly lose out on arbitrage profits(dependant the frequency of calling upkeep and trading dynamics, this could potentially be a large arbitrage profit), so given the potential for lost rewards, a rating of atleast high here is appropriate.

## Recommended Mitigation Steps

Given the interdependencies between step 5 and step 7, a failure in step 5 due to a failed swap implies that step 7 should also not proceed because its calculations are dependent on the success of step 5. Appropriate logic should be added to handle this.

othernet-global (Salty.IO) disputed and commented:

It is acceptable for step 7 to not distribute rewards if step 5 has not functioned correctly. Assuming step 5 functions correctly later, then step 7 will function correctly later as well.

Picodes (Judge) commented:

It seems to me that rewards are just delayed here as the step 5 uses the contract’s balance so there is no issue.

genesiscrew (Warden) commented:

There is no guarantee that the rewards that were delayed will be later distributed fairly to the pools that generated them. This is due to the fact that in the case of distribution failure, profits assigned to pools are cleared from storage as is shown in the POC.

Imagine pool A accumulated most of the rewards in interval 1, reward distribution fails, in interval two pool B accumulated most of the rewards, they will be rewarded more than they should be because the balance will include rewards from interval 1 and interval 2.

- https://github.com/code-423n4/2024-01-salty/blob/53516c2cdfdfacb662cdea6417c52f23c94d5b5b/src/rewards/SaltRewards.sol#L68
The line above shows how rewards are distributed for each pool. in this formula profitsForPools[i] and totalProfits will not factor in delayed rewards. while liquidityRewardsAmount will because its based off contract balance. so this means pool B in interval 2 will be effectively earning more rewards than it should, hence lost rewards for pool A.

Picodes (Judge) commented:

Thanks @genesiscrew. On second read it seems you are right. We could imagine a scenario where an attacker forces step 5 to fail to clear the storage and then distribute profits to a pool he is in. But the scenario wouldn’t be simple because you need to take into account the fact that automatic arbitrages increases the cost of manipulating pool reserves.

# [M-25] Incorrect assumption in PoolMath.sol can cause underflow when zapping is used

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** M-25
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

Submitted by t0x1c, also found by Draiakoo and AgileJune _zapSwapAmount() assumes that:

191:

// r1 * z0 guaranteed to be greater than r0 * z1 per the conditional check in _determineZapSwapAmount 192:

uint256 C = r0 * ( r1 * z0 - r0 * z1 ) / ( r1 + z1 ); The protocol’s assumption of r1 * z0 guaranteed to be greater than r0 * z1 is based on the following check in _determineZapSwapAmount():

214:

// zapAmountA / zapAmountB exceeds the ratio of reserveA / reserveB? - meaning too much zapAmountA 215:

if ( zapAmountA * reserveB > reserveA * zapAmountB ) 216: ( swapAmountA, swapAmountB ) = ( _zapSwapAmount ( reserveA, reserveB, zapAmountA, zapAmountB ), 0 ); 217:

218:

// zapAmountA / zapAmountB is less than the ratio of reserveA / reserveB? - meaning too much zapAmountB 219:

if ( zapAmountA * reserveB < reserveA * zapAmountB ) 220: ( swapAmountA, swapAmountB ) = ( 0, _zapSwapAmount ( reserveB, reserveA, zapAmountB, zapAmountA )); The assumption would had been true for all cases if not for this piece of logic inside _zapSwapAmount()#L158-L168 which right shifts the arguments if the maximumMSB is greater than 80:

// Assumes the largest number has more than 80 bits - but if not then shifts zero effectively as a straight assignment.

// C will be calculated as: C = r0 * ( r1 * z0 - r0 * z1 ) / ( r1 + z1 ); // Multiplying three 80 bit numbers will yield 240 bits - within the 256 bit limit.

if ( maximumMSB > 80 ) shift = maximumMSB - 80; // Normalize the inputs to 80 bits.

uint256 r0 = reserve0 >> shift; uint256 r1 = reserve1 >> shift; uint256 z0 = zapAmount0 >> shift; uint256 z1 = zapAmount1 >> shift; This can lead to z0 being reduced to 0 and hence causing underflow on L192.

Since _determineZapSwapAmount() is internally called whenever depositLiquidityAndIncreaseShare() is called with useZapping = true, it will cause a revert when a situation like the following exists:

uint256 reserveA = 1500000000; uint256 reserveB = 2000000000 ether; uint256 zapAmountA = 150; uint256 zapAmountB = 100 ether; maximumMSB in this case is 90 (for reserveB ) and also an excess of zapAmountA is being provided as compared to the existing reserve ratio. Hence, right shift by 90 - 80 = 10 bits will occur resulting z0 to be 0.

## Recommended Mitigation Steps

Make sure to check the assertion again:

function _zapSwapAmount( uint256 reserve0, uint256 reserve1, uint256 zapAmount0, uint256 zapAmount1 ) internal pure returns (uint256 swapAmount) { uint256 maximumMSB = _maximumMSB( reserve0, reserve1, zapAmount0, zapAmount1); uint256 shift = 0; // Assumes the largest number has more than 80 bits - but if not then shifts zero effectively as a straight assignment.

// C will be calculated as: C = r0 * ( r1 * z0 - r0 * z1 ) / ( r1 + z1 ); // Multiplying three 80 bit numbers will yield 240 bits - within the 256 bit limit.

if ( maximumMSB > 80 ) shift = maximumMSB - 80; // Normalize the inputs to 80 bits.

uint256 r0 = reserve0 >> shift; uint256 r1 = reserve1 >> shift; uint256 z0 = zapAmount0 >> shift; uint256 z1 = zapAmount1 >> shift; // In order to swap and zap, require that the reduced precision reserves and one of the zapAmounts exceed DUST.

// Otherwise their value was too small and was crushed by the above precision reduction and we should just return swapAmounts of zero so that default addLiquidity will be attempted without a preceding swap.

if ( r0 < PoolUtils.DUST) return 0; if ( r1 < PoolUtils.DUST) return 0; if ( z0 < PoolUtils.DUST) if ( z1 < PoolUtils.DUST) return 0; // Components of the quadratic formula mentioned in the initial comment block: x = [-B + sqrt(B^2 - 4AC)] / 2A uint256 A = 1; uint256 B = 2 * r0; // Here for reference // uint256 C = r0 * ( r0 * z1 - r1 * z0 ) / ( r1 + z1 ); // uint256 discriminant = B * B - 4 * A * C; - // Negate C (from above) and add instead of subtract.

- // r1 * z0 guaranteed to be greater than r0 * z1 per the conditional check in _determineZapSwapAmount - uint256 C = r0 * ( r1 * z0 - r0 * z1 ) / ( r1 + z1 ); - uint256 discriminant = B * B + 4 * A * C; + uint256 C; + uint256 discriminant; + if ((r1 * z0) >= (r0 * z1)) { + C = r0 * ( r1 * z0 - r0 * z1 ) / ( r1 + z1 ); + discriminant = B * B + 4 * A * C; + } else { + C = r0 * ( r0 * z1 - r1 * z0 ) / ( r1 + z1 ); + discriminant = B * B - 4 * A * C; + } // Compute the square root of the discriminant.

uint256 sqrtDiscriminant = Math.sqrt(discriminant); // Safety check: make sure B is not greater than sqrtDiscriminant if ( B > sqrtDiscriminant ) return 0; // Only use the positive sqrt of the discriminant from: x = (-B +/- sqrtDiscriminant) / 2A swapAmount = ( sqrtDiscriminant - B ) / ( 2 * A ); // Denormalize from the 80 bit representation swapAmount <<= shift; } othernet-global (Salty.IO) confirmed and commented:

Zapping no longer uses scaling:

- https://github.com/othernet-global/salty-io/commit/44320a8cc9b94de433e437e025f072aa850b995a
Status:

Mitigation confirmed. Full details in reports from zzebra83, 0xpiken, and t0x1c.

# [M-26] formPOL lacks slippage and deadline protection

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** M-26
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

Submitted by Banditx0x, also found by Hajime, oakcobalt, 0xGreyWolf, Tripathi, PENGUN, Krace, 00xSEV ( 1, 2 ), 0xmuxyz, b0g0 ( 1, 2 ), Jorgect, Kaysoft, and djxploit

- https://github.com/code-423n4/2024-01-salty/blob/53516c2cdfdfacb662cdea6417c52f23c94d5b5b/src/dao/DAO.sol#L316
- https://github.com/code-423n4/2024-01-salty/blob/53516c2cdfdfacb662cdea6417c52f23c94d5b5b/src/dao/DAO.sol#L360
formPOL can be vulnerable to a well known liquidity deposit sandwich attack causing loss of funds for the protocol.

## Recommended Mitigation Steps

formPOL should have some form of slippage protection. This could be based off a maximum deviation off some other price feed such a Uniswap v3 TWAP or Chainlink pricefeed. If the pool reserves deviates too much from the expected ratio from the price feed, formPOL should revert.

othernet-global (Salty.IO) acknowledged and commented:

The automatic arbitrage on the DEX provides some built in protection against front running as the attacker is not able to move in and out without experiencing friction from the arbitrage itself.

Simulations (see Sandwich.t.sol) show that when sandwich attacks are used on Salty, the arbitrage earned by the protocol sometimes exceeds any amount lost due to the sandwich attack itself. The actual swap loss (taking arbitrage profits generated by the sandwich swaps into account) is dependent on the multiple pool reserves involved in the arbitrage (which are encouraged by rewards distribution to create more reasonable arbitrage opportunities).

Picodes (Judge) decreased severity to Medium and commented:

Regrouping issues about missing slippage checks here as the root cause is the same - the assumption that AAA is enough to prevent MEV bots from sandwiching maintenance transactions doesn’t always hold.

othernet-global (Salty.IO) commented:

POL has been removed from the protocol:

eaf40ef0fa27314c6e674db6830990df68e5d70e

- https://github.com/othernet-global/salty-io/commit/8e3231d3f444e9851881d642d6dd03021fade5ed
Status:

Mitigation confirmed. Full details in reports from 0xpiken, zzebra83, and t0x1c.

# [M-27] Attacker Can Inflate LP Position Value To Create a Bad Debt Loan

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** M-27
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

Submitted by Banditx0x, also found by Arz, Infect3d, Toshii, Kalyan-Singh, jasonxiale, israeladelaja, PENGUN, a3yip6, linmiaomiao, and zhaojohnson An attacker can inflate the value of liquidity through reserve ratio manipulation (even without manipulating the aggregated oracle) and take out a undercollateralized USDS loan.

## Recommended Mitigation Steps

Valuing LP positions is tricky. Salty’s current method is to “trust” the untrustworthy pool reserves. Instead, you could value the liquidity as if the ratio was at the correct ratio, rather than the current pool ratio. Here is an example of a protocol implementing this solution for Uniswap v3 positions:

- https://github.com/arcadia-finance/accounts-v2/blob/main/src/asset-modules/UniswapV3/UniswapV3AM.sol
othernet-global (Salty.IO) confirmed, but disagreed with severity and commented:

USDS has been removed from the exchange.

- https://github.com/othernet-global/salty-io/commit/f3ff64a21449feb60a60c0d60721cfe2c24151c1
The stablecoin framework: /stablecoin, /price_feed, WBTC/WETH collateral, PriceAggregator, price feeds and USDS have been removed:

- https://github.com/othernet-global/salty-io/commit/88b7fd1f3f5e037a155424a85275efd79f3e9bf9
Picodes (Judge) decreased severity to Medium and commented:

As shown by the report above, this attack seems valid but if arbitrage paths are properly configured the cost of attack is greater than usual. You’d need to either increase the size of the pool or manipulate the price of all the pools in the arbitrage paths at once to prevent arbitrages from happening. As it’s still possible, the correct severity seems to be Medium under ” leak value with a hypothetical attack path with stated assumptions, but external requirements.”, the external requirements being that the cost of attack is smaller than the value extractable by borrowing USDS.

Status:

Mitigation confirmed. Full details in reports from zzebra83, 0xpiken, and t0x1c.

# [M-28] MinShares Slippage Parameters Are Ineffective For Initial Deposit

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** M-28
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

Submitted by Banditx0x, also found by 0xMango and jasonxiale The first depositor in the AMM can lose the majority of their tokens to a frontrunning attack even if they set the correct minShares slippage parameters.

## Recommended Mitigation Steps

Consider using minAmount0 and minAmount1 deposited into the AMM. This is how Uniswap V3 implements their slippage parameters.

Alternatively, using the same liquidity formula as Uniswap v2 - $L = sqrt(x \* y)$ will prevent this attack.

othernet-global (Salty.IO) confirmed and commented:

minAddedAmountA and minAddedAmountB are now used.

Fixed in:

- https://github.com/othernet-global/salty-io/commit/0bb763cc67e6a30a97d8b157f7e5954692b3dd68
Picodes (Judge) decreased severity to Medium Note: For full discussion, see here.

Status:

Mitigation confirmed. Full details in reports from 0xpiken, zzebra83, and t0x1c.

# [M-29] Incorrect calculation to check remaining ratio after reward in StableConfig.sol

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** M-29
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

Submitted by t0x1c, also found by 0xpiken, klau5, and haxatron

- https://github.com/code-423n4/2024-01-salty/blob/main/src/stable/StableConfig.sol#L51-L54
- https://github.com/code-423n4/2024-01-salty/blob/main/src/stable/StableConfig.sol#L127-L130
changeMinimumCollateralRatioPercent() allows the owner to change the minimumCollateralRatioPercent while the changeRewardPercentForCallingLiquidation() function allows to change the rewardPercentForCallingLiquidation.

The protocol aims to maintain a remaining ratio of 105% as is evident by the comments in these 2 functions:

// Don't decrease the minimumCollateralRatioPercent if the remainingRatio after the rewards would be less than 105% - to ensure that the position will be liquidatable for more than the originally borrowed USDS amount (assume reasonable market volatility) and // Don't increase rewardPercentForCallingLiquidation if the remainingRatio after the rewards would be less than 105% - to ensure that the position will be liquidatable for more than the originally borrowed USDS amount (assume reasonable market volatility) That is, if borrow amount is 100, after the calls to any of these two functions, there should be at least 105 collateral remaining which will act as buffer against market volatility.

The current calculation however is incorrect and there is really no direct relationship ( in the way the developer assumes ) between the rewardPercentForCallingLiquidation and minimumCollateralRatioPercent. Consider this:

minimumCollateralRatioPercent of 110% means that for a borrow amount of 200, collateral should not go below 220. This is 110% of 200.

However, rewardPercentForCallingLiquidation of 5% is calculated on the collateral amount and NOT the borrowed amount as is evident in the comments too here and here. So the liquidator will receive 5% of 220 ( let’s assume boundary values for rounded calculations ) which is 11.

The remaining amount would be 220 - 11 = 209 which is 104.5% of the borrowed amount. This is less than the 105% the protocol was aiming for. In fact, this figure of 104.5% goes down further to 103.5% when rewardPercentForCallingLiquidation = 5% and minimumCollateralRatioPercent = 115% which is much lower than protocol’s buffer target. Not being aware of this risk can cause unexpected loss of funds.

A table outlining the real buffer upper limit values is provided below. Another table showing the actual desirable gap in values is also provided so that the buffer always is above 105%.

Straightaway, it can be seen that the current default protocol values of 5% and 110% give a buffer of less than 105% and hence either the minimumCollateralRatioPercent needs to have a lower limit of 111 instead of 110, or there should be agreement to the fact that 103.5% is an acceptable remainingRatio figure under the current scheme of things.

## Recommended Mitigation Steps

Assuming that the protocol wants to calculate an actual 105% remainingRatio, changes along these lines need to be made. Please note that you may have to additionally make sure rounding errors & precision loss do not creep in. These suggestions point towards a general direction:

Update the two functions:

function changeRewardPercentForCallingLiquidation(bool increase) external onlyOwner { if (increase) { // Don't increase rewardPercentForCallingLiquidation if the remainingRatio after the rewards would be less than 105% - to ensure that the position will be liquidatable for more than the originally borrowed USDS amount (assume reasonable market volatility) + uint256 afterIncrease = rewardPercentForCallingLiquidation + 1; + uint256 remainingRatio = minimumCollateralRatioPercent - minimumCollateralRatioPercent * afterIncrease / 100; - uint256 remainingRatioAfterReward = minimumCollateralRatioPercent - rewardPercentForCallingLiquidation - 1; - if (remainingRatioAfterReward >= 105 && rewardPercentForCallingLiquidation < 10)

+ if (remainingRatio >= 105 && rewardPercentForCallingLiquidation < 10) rewardPercentForCallingLiquidation += 1; } else { if (rewardPercentForCallingLiquidation > 5) rewardPercentForCallingLiquidation -= 1; } emit RewardPercentForCallingLiquidationChanged(rewardPercentForCallingLiquidation); } and function changeMinimumCollateralRatioPercent(bool increase) external onlyOwner { if (increase) { if (minimumCollateralRatioPercent < 120) minimumCollateralRatioPercent += 1; } else { // Don't decrease the minimumCollateralRatioPercent if the remainingRatio after the rewards would be less than 105% - to ensure that the position will be liquidatable for more than the originally borrowed USDS amount (assume reasonable market volatility)

+ uint256 afterDecrease = minimumCollateralRatioPercent - 1; + uint256 remainingRatio = afterDecrease - afterDecrease * rewardPercentForCallingLiquidation / 100; - uint256 remainingRatioAfterReward = minimumCollateralRatioPercent - 1 - rewardPercentForCallingLiquidation; - if (remainingRatioAfterReward >= 105 && minimumCollateralRatioPercent > 110) + if (remainingRatio >= 105 && minimumCollateralRatioPercent > 111) minimumCollateralRatioPercent -= 1; } emit MinimumCollateralRatioPercentChanged(minimumCollateralRatioPercent); } Also L39:

- 39: uint256 public minimumCollateralRatioPercent = 110; + 39: uint256 public minimumCollateralRatioPercent = 111; Picodes (Judge) commented:

This report shows how an invariant of the protocol is broken so Medium severity seems appropriate.

othernet-global (Salty.IO) acknowledged and commented:

The stablecoin framework: /stablecoin, /price_feed, WBTC/WETH collateral, PriceAggregator, price feeds and USDS have been removed:

- https://github.com/othernet-global/salty-io/commit/88b7fd1f3f5e037a155424a85275efd79f3e9bf9
Status:

Mitigation confirmed. Full details in reports from zzebra83, 0xpiken, and t0x1c.

# [M-30] Chainlink price feed uses BTC, not WBTC. In case of depegging, oracles will become easier to manipulate

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** M-30
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

Submitted by thekmj, also found by peanuts, Tripathi, Ward, OMEN, Toshii, J4X, grearlake, 00xSEV, juancito, Lalanda, and eeshenggoh

- https://github.com/code-423n4/2024-01-salty/blob/main/src/price_feed/CoreChainlinkFeed.sol#L15
- https://github.com/code-423n4/2024-01-salty/blob/main/src/price_feed/CoreSaltyFeed.sol#L32-L41
- https://github.com/code-423n4/2024-01-salty/blob/main/src/price_feed/PriceAggregator.sol#L108
Chainlink BTC price feed is BTC/USD, not WBTC/USD. In the event of WBTC depegging, the oracle’s return price will deviate from its actual value. We also provide a real-life WBTC depegging event as evidence.

This alone is not enough for the price aggregator to return the incorrect price, as an adversary needs to manipulate two of three price feeds to manipulate the price. However, due to the aggregator design, we also make an argument that in case of actual depegging, the price will indeed be easier to manipulate.

## Vulnerability details

According to the official Chainlink docs, there are four price feeds for BTC on Ethereum Mainnet:

BTC / ETH BTC / USD ETH / BTC WBTC / BTC Based on the following observations, we believe Salty will use BTC/USD on the Chainlink price feed, instead of WBTC:

All test cases use BTC/USD feed, and nowhere in the code repo is the WBTC feed used.

There is no WBTC/USD feed on Ethereum Mainnet (they are available on some other networks).

Salty’s Chainlink price fetcher uses only one feed for the WBTC price. At least two feeds are needed (WBTC/BTC and BTC/USD) to fetch the WBTC/USD price.

Historically, WBTC has depegged down to 0.98 before, in the event of wild market swing, specifically during the LUNA crash.

Even as of time of report writing, the Chainlink feed for WBTC/BTC does not return a price of 1.

Screenshot.

This article explains some of the reasons of why WBTC can depeg.

## Impact

In the event of WBTC/BTC depeg, such as rapid market swing, the price oracle will become easier to manipulate.

Given the price of BTC, even a 2% deviation can be considered large.

## Recommended mitigation steps

Collect the WBTC price from two Chainlink price feeds, the BTC/USD feed and the WBTC/BTC feed, as the source of truth.

othernet-global (Salty.IO) acknowledged and commented:

The stablecoin framework: /stablecoin, /price_feed, WBTC/WETH collateral, PriceAggregator, price feeds and USDS have been removed:

- https://github.com/othernet-global/salty-io/commit/88b7fd1f3f5e037a155424a85275efd79f3e9bf9
Status:

Mitigation confirmed. Full details in reports from 0xpiken, zzebra83, and t0x1c.

# [M-31] changeWallets() can be confirmed immediately after proposalWallets() by manipulating activeTimelock beforehand

- **Contest:** Salty.IO
- **Slug:** 2024-01-saltyio
- **Finding ID:** M-31
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-saltyio
- **Source snapshot:** competitions/2024-01-saltyio/final_report.html

Submitted by t0x1c, also found by ether_sky, peanuts, oakcobalt, IceBear, wangxx2026, 0xpiken, and 0xCiphky ManagedWallet.sol states that:

// A smart contract which provides two wallet addresses (a main and confirmation wallet) which can be changed using the following mechanism:

// 1. Main wallet can propose a new main wallet and confirmation wallet.

// 2. Confirmation wallet confirms or rejects.

// 3. There is a timelock of 30 days before the proposed mainWallet can confirm the change.

However, the current 30-day wait period can be bypassed.

The expected order by the protocol is:

proposeWallets() is called by mainWallet.

To confirm the proposal, confirmationWallet sends at least 0.05 ether and causes the receive() function to trigger. This sets the activeTimelock to block.timestamp + TIMELOCK_DURATION i.e. 30 days into the future.

proposedMainWallet calls changeWallets() after 30 days and the new wallet addresses are set.

To bypass the 30-day limitation, the following flow can be used:

Even with no propsal for a change existing, confirmationWallet sends at least 0.05 ether and causes the receive() function to trigger. This sets the activeTimelock to block.timestamp + TIMELOCK_DURATION i.e. 30 days into the future.

Just as 30 days pass, proposeWallets() is called by mainWallet Immediately, with no delay whatsoever, proposedMainWallet calls changeWallets() The new wallet addresses are successfully assigned.

Impact:

Although the users believe that any changes to wallet address is going to have a timelock of 30 days as promised by the protocol, it really can be bypassed by the current admins/wallet addresses. This breaks the intended functionality implmentation.

## Recommended Mitigation Steps

Inside receive() make sure an active proposal exists:

receive() external payable { require( msg.sender == confirmationWallet, "Invalid sender" ); + require( proposedMainWallet != address(0), "Cannot manipulate activeTimelock without active proposal" ); // Confirm if.05 or more ether is sent and otherwise reject.

// Done this way in case custodial wallets are used as the confirmationWallet - which sometimes won't allow for smart contract calls.

if ( msg.value >=.05 ether ) activeTimelock = block.timestamp + TIMELOCK_DURATION; // establish the timelock else activeTimelock = type(uint256).max; // effectively never } othernet-global (Salty.IO) confirmed and commented:

Managed wallet has been removed:

- https://github.com/othernet-global/salty-io/commit/5766592880737a5e682bb694a3a79e12926d48a5
Status:

Mitigation confirmed. Full details in reports from t0x1c, zzebra83, and 0xpiken.

## Rejected Primary Findings

# Rejected Primary Findings: Salty.IO

No rejected primary findings were captured for this contest in the current corpus.

- **Rejected primary count:** 0
- **Primary submission rows captured:** 0
- **Submission status:** requires_authentication
