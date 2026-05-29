# Benchmark Ground Truth: Concrete

## Accepted H/M Findings

# Accepted H/M Findings: Concrete

No accepted High/Medium findings were parsed from a final report in the current corpus.

- **Accepted H/M count:** 0
- **Final report status:** http_404
- **Submission status:** captured_from_authenticated_browser

## Rejected Primary Findings

# Rejected Primary Findings: Concrete

# Strategies with certain tokens cannot be removed

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-53
- **Submitter:** inh3l
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-53
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-53.txt

## Brief Summary

Protocols plans to work with various tokens. This includes tokens that revert on zero value approvals. From the information provided in the readme, this is very plausible. However, strategies cannot be removed if these tokens are in use. This is because the removeStrategy attempts to clear approval by approving the strategy with zero value. But due to the token's behavior, this will revert.

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Unrestricted Token Cascade Configuration Leads To Incorrect Usage

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-326
- **Submitter:** 0x2517
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-326
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-326.txt

## Brief Summary

The ClaimRouter contract contains a critical flaw in its token conversion logic where the tokenCascade array, which should exclusively contain stablecoin addresses, is not properly validated. The _convertFromTokenToStable function is used to convert token amounts to stable coin values, but the contract fails to enforce that the destination tokens in tokenCascade are actually stablecoins. amount = _convertFromTokenToStable(tokenAddress, amount_); //We control both the length of the array and the external call // slither-disable-next-line calls-loop (protectionStrat, requiresFunds) = _getStrategy(tokenCascade[i], amount); However, the tokenCascade array can be configured with any token addres...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Strategies are un-pausable despite being intended to be otherwise

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-43
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-43
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-43.txt

## Brief Summary

Strategies are un-pausable despite being intended to be otherwise

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# No mechanism to be able to get reward from radiant V2

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-84
- **Submitter:** grearlake
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-84
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-84.txt

## Brief Summary

To be able to get reward from Radiant, user need to zap RDNT to be eligible, but there is no mechanism to do that in RadiantV2Strategy contract

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Missing Dust Share Validation in Redeem Allows Dust Share Redemptions

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-26
- **Submitter:** Prosperity
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-26
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-26.txt

## Brief Summary

The ConcreteMultiStrategyVault contract has inconsistent validation for dust shares in its redeem and withdraw functions. While the withdraw() function properly validates against dust shares with (shares <= DUST) revert ZeroAmount() the redeem() function only checks for zero amounts with (shares_ == 0) revert ZeroAmount() This inconsistency allows users to redeem dust shares that should be restricted and incure loss on users as they loss their shares without getting any assets from the vault. Impact Call redeem() directly with any shares > 0 but <= DUST Users can burn dust shares by directly calling redeem(), incurring loss for users Bypass paying fee on multiple calls when share amount is...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Strategies claim rewards during harvest even when `rewardsData` is empty

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-291
- **Submitter:** Maroutis
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-291
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-291.txt

## Brief Summary

In the ConcreteMultiStrategyVault contract, the harvestRewards function iterates over all strategies and calls their harvestRewards method, passing in rewardsData for each. However, except for the MorphoStrategy, other strategies proceed to claim rewards even when rewardsData is empty. This behavior can lead to unintended or premature claiming of rewards, which might not be optimal for maximizing returns. By claiming rewards when not necessary, the overall rewards for users can be reduced, especially if those rewards would have been more beneficial if left to accumulate or reinvested within the strategy. Impact Premature claiming of rewards can lead to suboptimal yield, as rewards might be...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Wrong check causing incorrect withdrawals leading to reverts

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-323
- **Submitter:** samuraii77
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-323
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-323.txt

## Brief Summary

Wrong check causing incorrect withdrawals leading to reverts due to an unitialized variable

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Fee-on-Transfer tokens will cause incorrect accounting and losses/reverts in protocol

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-48
- **Submitter:** Maroutis
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-48
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-48.txt

## Brief Summary

The protocol does not properly handle fee-on-transfer tokens, which are tokens that deduct a fee upon transfer. In various parts of the code, such as during asset deposits/mints and reward deposits/claims, the protocol assumes that the amount of tokens sent by the user or received from a strategy is equal to the amount intended or specified. The protocol does not calculate the exact amount of assets deposited or withdrawn by checking the actual balance changes before and after the transfer. This assumption leads to incorrect accounting when interacting with fee-on-transfer tokens, as the actual amount received is less than the amount sent due to the fee deduction. Impact The protocol overes...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Lack of slippage protection.

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-32
- **Submitter:** sl1
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-32
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-32.txt

## Brief Summary

deposit(), mint(), withdraw() and redeem() functions of the ConcreteMultiStrategyVault are missing slippage control mechanism. When depositing or withdrawing from a vault, the respective function is called on each strategy of the vault. ConcreteMultiStrategyVault.sol#L258-L261 strategies[i].strategy.deposit( assets_.mulDiv( strategies[i].allocation.amount, MAX_BASIS_POINTS, Math.Rounding.Floor ), address(this) Each strategy then either deposits or withdraws funds from its underlying lending pool. MorphoVaultStrategy.sol#L71-L74 function _protocolDeposit( uint256 assets_, uint256 ) internal virtual override { // slither-disable-next-line unused-return _morphoVault.deposit(assets_, address(th...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Radiant Strategy Allows Reward Claims Without Meeting dLP Eligibility Requirements

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-85
- **Submitter:** Agontuk
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-85
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-85.txt

## Brief Summary

The RadiantV2Strategy contract implements functionality to claim RDNT rewards from Radiant's lending protocol. The strategy includes a rewardsEnabled flag that can be toggled by the owner through setEnableRewards() to enable/disable reward claiming. When enabled, the strategy attempts to claim rewards via _getRewardsToStrategy() during withdrawals and reward harvesting. However, Radiant's protocol requires maintaining a minimum 5% ratio between total deposit value and dLP (Radiant Liquidity Provider tokens) value to be eligible for rewards. The strategy lacks any mechanism to verify this requirement is met before attempting claims: function _getRewardsToStrategy(bytes memory) internal overr...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Withdrawal request cannot be claimed or finished and withdrawal queue cannot be changed for tokens with balance changes outside of transfers, such as deflationary tokens

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-329
- **Submitter:** rbserver
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-329
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-329.txt

## Brief Summary

According to https://github.com/code-423n4/2024-11-concrete?tab=readme-ov-file#erc20-token-behaviors-in-scope, this protocol intends to support tokens with balance changes outside of transfers, such as deflationary tokens. Yet, due to the changes of the vault's token balance held by itself and the vault's token amounts held by the strategies caused by the rebasing events, the withdrawal request, which has a stored and fixed token amount, can fail to be claimed or finished, which also causes the withdrawal queue to be unable to be changed.

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Implement A Method To Get Rewards From Strategies

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-298
- **Submitter:** EPSec
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-298
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-298.txt

## Brief Summary

When a strategy is retired in the ConcreteMultiStrategyVault, there is no mechanism to ensure that rewards allocated to the strategy are claimed. For example, in the MorphoVault, the _getRewardsToStrategy function is not invoked as part of the retireStrategy process. This oversight can leave unclaimed rewards stuck in the contract, rendering them inaccessible and disrupting the protocol's efficiency. Rewards remain inaccessible, reducing the efficiency of the protocol.

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# `removeRewardToken()` fails to claim the pending rewards tokens before removing them from the vault

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-265
- **Submitter:** blutorque
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-265
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-265.txt

## Brief Summary

The ConcreateMultiStrategyVault contract do support multiple strategies. Rewards accrues to these strategies over time are claimable through a harvesRewards() call on the vault. The issue here is in the method removeRewardToken() which leaves pending reward unclaimed when being called. This method is defined in StrategyBase contract, and the different strategies inherit this method from the StrategyBase. Some strategy such as AaveV3Strategy can have more than one reward tokens, /** * @dev Retrieves the addresses of reward tokens available for this strategy. * @return An array of addresses of reward tokens. */ function getRewardTokenAddresses() public view override returns (address[] memory)...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Fee recipient will be charged a fee in a certain case

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-330
- **Submitter:** samuraii77
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-330
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-330.txt

## Brief Summary

Fee recipient will be charged a fee in a certain case

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# User loses share of accumulated rewards processed during every withdraw in Aave strategy

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-316
- **Submitter:** 0xrex
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-316
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-316.txt

## Brief Summary

Everytime a withdraw/redeem happens in a multi strategy vault that is connected to an Aave strategy, the Aave strategy pulls accumulated rewards in Aave third-party pool through the _handleRewardsOnWithdraw function. These rewards are sent into the Aave strategy contract and not the multi-strategy vault it is tied to. In this case: The user who redeemed/withdrew shares will not get their share of the accumulated rewards The multi-strategy vault will also not be sent the accumulated rewards and thus its rewardIndex[rewardToken] for the aave reward will not be updated to let users claim it based on their share %. Considering user rewards are relative to their shares, once the user' shares are...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# User with blueprint can add rewards on behalf of a different blueprint.

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-65
- **Submitter:** sl1
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-65
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-65.txt

## Brief Summary

The are two way of adding tokens to protection strategies: addRewards() and repay() functions. Both these functions can only be called by a sender with a blueprint role. However, currently it's possible for a user with a blueprint role to steal funds from another blueprint that has approved tokens to the ClaimRouter. If we look at the addRewards() function userBlueprint address is a user-supplied parameter. ClaimRouter.sol#L233-L239 function addRewards( address tokenAddress, uint256 amount_, address userBlueprint ) external onlyRole(BLUEPRINT_ROLE) { _addTokensToStrategy(tokenAddress, amount_, userBlueprint, true); } When _addTokensToStrategy() is called, it will try to transfer tokens from...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Retiring Aave strategy will lead to stuck rewards in the Aave strategy contract

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-324
- **Submitter:** 0xrex
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-324
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-324.txt

## Brief Summary

During retirement of the Aave strategy, the protocol calls the aave incentive controller to claim accrued rewards on behalf of the Aave strategy contract. However, since these rewards are indeed claimed and sent to the Aave strategy contract and not sent to the concrete multi strategy vault it is linked to, the rewards will remain in the Aave strategy contract. In this case too, the feeRecipient of the Aave strategy contract will not receive its percentage share of the claimed rewards as well as users not receiving rewards earned from the strategy.

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Protocol takes performance fee even if the share rate has dropped

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-76
- **Submitter:** deadrxsezzz
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-76
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-76.txt

## Brief Summary

The modifier takeFees is responsible to take two types of fees - time-based one and a performance based one. The problem is that the performance fee one does not take into account the time-based one and therefore results in possible take of a performance fee, even in situations where the share rate has dropped.

## Rejection Reason

Final severity/evaluation: Low; Valid Primary Sufficient

# Incompatible Clone Implementation with ZKSync Chain

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-218
- **Submitter:** 0x2517
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-218
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-218.txt

## Brief Summary

The issue is proposed since the contract will be deployed across chains(Ethereum,BSC,Other,ArbitrumCorn) Finding description and impact The Clones library from OpenZeppelin is incompatible with ZKSync Era. This incompatibility stems from fundamental differences in how contract deployment works on ZKSync(EVM chain) compared to traditional EVM chains. The current implementation uses runtime assembly to generate and deploy proxy contracts using create and create2 opcodes: newVault = Clones.cloneDeterministic(implementation_.implementationAddress, salt_); On ZKSync Era, contract deployments require the compiler must be aware of the bytecode of the deployed contract in advance. To guarantee that...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Broken Functionality in ConcreteMultiStrategie Vault on Zero Allocation Strategy

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-63
- **Submitter:** a2security
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-63
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-63.txt

## Brief Summary

There exists a denial of service (DoS) vulnerability in the ConcreteMultiStrategie Vault contract where, if the allocation amount for any of the underlying strategies is set to zero, the contract will revert on both deposit and withdrawal attempts. To summarize, the BaseStrategy.sol contract that all the strategies inherit from will revert if the deposit amount or the withdrawal amount is zero. src/strategies/StrategyBase.sol function _deposit( address caller_, address receiver_, uint256 assets_, uint256 shares_ ) internal virtual override(ERC4626Upgradeable) whenNotPaused onlyVault { //slither-disable-next-line incorrect-equality if (shares_ == 0 || assets_ == 0) revert ZeroAmount(); when...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Borrow flow vault selection algorithm leads to inefficent use of liquidity and unfair distribution of risk between LPs

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-90
- **Submitter:** muellerberndt
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-90
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-90.txt

## Brief Summary

While repays are distributed proportionally between protection strategies based on the borrow debt, individual borrow requests are served from a single vault. Prioritization is, to some degree, based on the order the vaults are listed. This leads to an uneven distribution of risk and rewards between LPs of different vaults, as well as an inefficient use of liquidity. In the borrow flow (function requestToken()), the ClaimRouter searches for a vault that holds a sufficient amount of the desired token as its base asset. The selected vault needs to have a protection strategy. The yield generated by the protection strategy is also considered (see also the decision chart in the docs). This cause...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# `_addTokensToStrategy` of the Claim Router opens up ways to game protect strategies

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-16
- **Submitter:** 0xrex
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-16
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-16.txt

## Brief Summary

Other strategies using the same token that should get reward tokens added to them would be skipped with more than enough reward tokens added to the strategy being tricked. Hence, reward token loss for the rest strategies.

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# executeBorrowClaim does not account for fee on transfer tokens

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-56
- **Submitter:** zzebra83
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-56
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-56.txt

## Brief Summary

executeBorrowClaim in the ProtectStrategy transfers the specified amount of assets to the recipient and updates the borrow debt. But it does not account for assets with fees on transfer( fee on transfer issues in scope for this audit), causing borrow debt of protect strategy to be more than it actually is. function executeBorrowClaim(uint256 amount, address recipient) external override onlyClaimRouter { if (amount == 0) revert ZeroAmount(); uint256 balance = getAvailableAssetsForWithdrawal(); if (balance < amount) { _requestFromVault(amount - balance); } // borrow debt will be higher than it should due to fee on transfer borrowDebt += amount; IERC20(asset()).safeTransfer(recipient, amount);...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# In `ConcreteMultiStrategyVault.harvestRewards` function, there could be multiple entries of the same `rewardToken` into the `rewardAddresses` array

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-244
- **Submitter:** Udsen
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-244
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-244.txt

## Brief Summary

The ConcreteMultiStrategyVault.harvestRewards function harvestRewards from each of the registered strategies and then adds the rewardTokens returned from the respective strategy to the rewardIndex mapping if the rewardIndex[rewardToken] == 0 as shown below : if (amount != 0) { if (rewardIndex[rewardToken] == 0) rewardAddresses.push(rewardToken); rewardIndex[rewardToken] += amount.mulDiv(PRECISION, totalSupply, Math.Rounding.Floor); } But the issue is the same rewardToken can be added multiple times into the rewardAddresses array, if the amount.mulDiv(PRECISION, totalSupply, Math.Rounding.Floor) calculation rounds down to 0. This can happen in the following scenario: The concrete protocol ca...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# removeStrategy need to Claim Rewards Before Removal

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-266
- **Submitter:** ayden
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-266
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-266.txt

## Brief Summary

removeStrategy function remove an specific Strategy from vault , however not claim the reward before remove it.

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# approvalRaceProtection tokens that are awarded as rewardTokens can never be added again after they have been removed from a Strategy

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-279
- **Submitter:** 0xStalin
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-279
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-279.txt

## Brief Summary

A removed rewardToken can never be added agains a rewardToken on the same strategy. Finding Description When a rewardToken is added for the first time to a strategy, the contract grants infinite allowance to self. The problem is that when rewardTokens are removed, the infinite allowance is not removed, meaning, the rewardToken is left with an approval != 0. There are some ERC20 tokens (USDT) that reverts because an Approval Race Protection, a.k.a when the asset already has a defined approval and a new approval wants to be added. As specified ont the Readme, Approval Race Protection tokens are in scope. StrategyBase.removeRewardToken() function removeRewardToken(RewardToken calldata rewardTo...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# loss of assets for Recipient due to rounding issues when redeeming

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-105
- **Submitter:** Sancybars
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-105
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-105.txt

## Brief Summary

) public override nonReentrant whenNotPaused returns (uint256 assets) { if (receiver_ == address(0)) revert InvalidRecipient(); if (shares_ == 0) revert ZeroAmount(); if (shares_ > maxRedeem(owner_)) revert MaxError(); uint256 feeShares = shares_.mulDiv(uint256(fees.withdrawalFee), MAX_BASIS_POINTS, Math.Rounding.Ceil); assets = _convertToAssets(shares_ - feeShares, Math.Rounding.Floor);//@audit-issue should round up _withdraw(assets, receiver_, owner_, shares_, feeShares); } The amount of assets to send to the recipient is rounded downassets = _convertToAssets(shares_ - feeShares, Math.Rounding.Floor); due to this users/recipients will lose assets if the asset amount is rounded down to zer...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Exploitation of Minimal Debt in Protection Strategies Leading to Unfair Reward Distribution

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-99
- **Submitter:** chaduke
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-99
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-99.txt

## Brief Summary

The finding reveals a critical vulnerability in the ClaimRouter contract, where a malicious user with the BLUEPRINT_ROLE can exploit the reward distribution mechanism by lending just 1 wei (an extremely low-cost action) to a protection strategy. This minimal debt manipulation allows the strategy to monopolize all rewards, effectively excluding others from receiving their fair share. The exploit undermines the fairness mechanism, as rewards intended to be evenly distributed in the absence of debt are instead disproportionately allocated, creating a significant risk with negligible cost to the attacker. Root cause: The root cause is the lack of a minimum debt threshold in the reward distribut...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Users will miss Morpho Strategy last epoch reward

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-198
- **Submitter:** ChainProof
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-198
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-198.txt

## Brief Summary

Concrete strategies include a function to claim rewards from specific strategies. However, the MorphoVaultStrategy handles reward claiming differently from the others, as evident in the implementation below. On the display side: The reward calculation logic is block-based and the displayed amount is updated at different periods. The displayed amount of MORPHO rewards corresponds to the amount earned up to 00:00:00 UTC. For other rewards, the displayed amounts correspond to the amounts earned up to 30 minutes ago. On the claim side: Rewards work with an epoch system. When an epoch ends, rewards accrued during the epoch become available to claim. Currently epochs last 2 weeks, current one sho...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# If user is blocklisted for any of the reward tokens they won't be able to withdraw assets from the vault.

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-14
- **Submitter:** sl1
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-14
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-14.txt

## Brief Summary

Holders of the vault shares not only receive yield through the increase in their share values but also are entitled to a portion of the rewards from third-party protocols integrated with vault's strategies. When a user wishes to withdraw his assets from the vault, the internal _withdraw() function burns his shares. ConcreteMultiStrategyVault.sol#L419-L423 function _withdraw(uint256 assets_, address receiver_, address owner_, uint256 shares, uint256 feeShares) private { if (msg.sender != owner_) { _approve(owner_, msg.sender, allowance(owner_, msg.sender) - shares); } _burn(owner_, shares); Before actually burning the shares, _update() function will first claim all of the accrued rewards for...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Withdrawal queue can be spammed, forcing the admin to huge gas costs to resolve legit withdraw requests

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-51
- **Submitter:** thekmj
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-51
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-51.txt

## Brief Summary

When the user wants to withdraw assets, but the vault doesn't have enough funds (for example, if the protect strategy has high utilization), the withdraw is then inserted into a queue. if (availableAssetsForWithdrawal >= assets_) { _withdrawStrategyFunds(assets_, receiver_); } else { if (address(withdrawalQueue) == address(0)) { revert InsufficientVaultFunds(address(this), assets_, availableAssetsForWithdrawal); } withdrawalQueue.requestWithdrawal(receiver_, assets_); // @audit inserted into a queue } The admin can later claim these withdrawals for the user, using batchClaimWithdrawal /** * @notice Claims multiple withdrawal requests starting from the lasFinalizedRequestId. * @dev This func...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Wrong calculation of protocol fees in `ConcreteMultiStrategyVault::takeFees`

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-318
- **Submitter:** 0xAlix2
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-318
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-318.txt

## Brief Summary

In ConcreteMultiStrategyVault, the "default" 4626 shares/assets conversion is manipulated to block inflation attacks: function _convertToShares(uint256 assets, Math.Rounding rounding) internal view override returns (uint256 shares) { shares = assets.mulDiv(totalSupply() + 10 ** decimalOffset, totalAssets() + 1, rounding); } function _convertToAssets(uint256 shares, Math.Rounding rounding) internal view virtual override returns (uint256) { return shares.mulDiv(totalAssets() + 1, totalSupply() + 10 ** decimalOffset, rounding); } These functions convert users' balances/shares, whenever they deposit, mint, redeem, or withdraw. On the other hand, the protocol uses the takeFees modifier to mint s...

## Rejection Reason

Final severity/evaluation: Low; Valid Primary Sufficient

# No zero check on prices from the Berachain Oracle

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-256
- **Submitter:** waydou
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-256
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-256.txt

## Brief Summary

The _convertFromTokenToStable function retrieves a price from an oracle via _getPriceFromOracle. However, it does not check if the returned price is zero. since we devide by the price in stableAmount_.mulDiv(10 ** tokenDecimals, 10 ** CONCRETE_USD_DECIMALS, Math.Rounding.Floor).mulDiv( 10 ** quoteDecimals, price, Math.Rounding.Floor ) the whole function will revert. Another issue is that the code assumes that berachain oracle works perfectly. If an asset experiences a huge drop in value (i.e. LUNA crash) the price of the oracle will continue to return the minPrice instead of the actual price of the asset. This would allow user to continue borrowing with the asset but at the wrong price. Thi...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Using block timestamp in salt passed to cloneDeterministic() breaks deterministic deployment

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-236
- **Submitter:** muellerberndt
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-236
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-236.txt

## Brief Summary

New vaults are deployed by that vault admin via the DeploymentManager. In deployNewVault(), a salt is created as follows: bytes32 salt = keccak256( abi.encode(address(this), implementationData.implementationAddress, vaultCount, block.timestamp) ); The salt is then passed on to the VaultFactory which eventually calls the OpenZeppelin library function cloneDeterministic((address implementation, bytes32 salt), which uses the create2 opcode and the salt to deterministically deploy the code. The problem here is that the timestamp is unknown before the block that includes the transaction that executes deployVault() has been mined, defeating the entire point of deterministic deployment.

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Multiple Rounding Operations in Price Calculations Cause Growing Token Value Loss

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-112
- **Submitter:** victortheoracle
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-112
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-112.txt

## Brief Summary

The OraclePlug contract performs token price conversions using multiple sequential Floor rounding operations, leading to cumulative precision loss. This affects both token-to-stable and stable-to-token conversions, potentially causing value loss for users and the protocol. Root cause: The conversion functions perform multiple independent rounding operations: return price.mulDiv(tokenAmount_, 10 ** quoteDecimals, Math.Rounding.Floor) .mulDiv(10 ** CONCRETE_USD_DECIMALS, 10 ** tokenDecimals, Math.Rounding.Floor); return stableAmount_.mulDiv(10 ** tokenDecimals, 10 ** CONCRETE_USD_DECIMALS, Math.Rounding.Floor) .mulDiv(10 ** quoteDecimals, price, Math.Rounding.Floor); Impact: Each conversion l...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# `withdrawalQueue` burns users' shares while still lending out the user's assets, causing loss of yield for users in the withdrawal queue

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-45
- **Submitter:** rscodes
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-45
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-45.txt

## Brief Summary

When an user trys to withdraw but there isnt enough liquidity, the order gets sent to the withdrawal queue. function _withdraw(uint256 assets_, address receiver_, address owner_, uint256 shares, uint256 feeShares) private { if (msg.sender != owner_) { _approve(owner_, msg.sender, allowance(owner_, msg.sender) - shares); } 1.)--> _burn(owner_, shares); if (feeShares > 0) _mint(feeRecipient, feeShares); uint256 availableAssetsForWithdrawal = getAvailableAssetsForWithdrawal(); if (availableAssetsForWithdrawal >= assets_) { _withdrawStrategyFunds(assets_, receiver_); } else { if (address(withdrawalQueue) == address(0)) { revert InsufficientVaultFunds(address(this), assets_, availableAssetsForWi...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# `takeFees` may mint less fee shares than intended

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-108
- **Submitter:** trachev
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-108
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-108.txt

## Brief Summary

In the takeFees modifier it is possible for fees to have been accumulated, whilst the totalSupply is 0. In that case feeInShare, which is the number of shares the fee recipient will be minted, is set to totalFee. This, however, is wrong as even if totalSupply is 0, share tokens should be scaled by 9 decimals. This will cause the fee recipient to be minted much less fee shares than intended.

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Call to `withdraw()` reverts due to attempt to burn more shares than issued

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-259
- **Submitter:** t0x1c
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-259
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-259.txt

## Brief Summary

Upon calling withdraw(), the protocol attempts to burn shares + feeShares of the user when they would really have only shares issued to them. The protocol incorrectly increases quantity of shares to burn instead of decreasing the amount of assets to return. This causes a revert. Description The withdrawal fee ought to be deducted from the existing shares and the assets corresponding to the remaining shares need to be returned to the withdrawer when withdraw() is called. That's how it's implemented inside redeem() too.<br> However, the protocol adds the fee shares on top of the shares corresponding to the asset amount requested. This results in a situation where the user would not have enoug...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Users can be forced into going into the withdrawal queue

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-325
- **Submitter:** samuraii77
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-325
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-325.txt

## Brief Summary

Users can be forced into going into the withdrawal queue

## Rejection Reason

Final severity/evaluation: Low; Valid Primary Sufficient

# Users can avoid paying `withdrawFee` by using the Swapper

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-188
- **Submitter:** deadrxsezzz
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-188
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-188.txt

## Brief Summary

Within the Vault, there's a withdraw fee which users have to pay upon withdrawing funds. However, users can easily bypass it, given the provided Swapper contract.

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# The `ConcreteMultiStrategyVault.withdraw` does not account for the `unfinalized withdraw amount` during withdrawals

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-320
- **Submitter:** Udsen
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-320
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-320.txt

## Brief Summary

The ConcreteMultiStrategyVault.withdraw function is called to withdraw a specified amount of assets for the caller. During its execution flow the ConcreteMultiStrategyVault._withdraw function is called. In this function there is a check performed to ensure if the available assets to withdraw is greater than or equal to the requested amount to withdraw. If it is the case then the _withdrawStrategyFunds function is called to withdraw the requested amount from the ConcreteMultiStrategyVault contract and registered strategies and if there is not enough withdrawable assets to fulfill requested withdraw amount, withdraw request is queued in the withdrawalQueue. The following

## Rejection Reason

Final severity/evaluation: Low; Valid Primary Sufficient

# Changing strategies allocations with locked funds in strategies will break allocation percentages

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-54
- **Submitter:** zzebra83
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-54
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-54.txt

## Brief Summary

The admin has the ability change allocations for strategies via the changeallocation function. /** * @notice Changes strategies allocations. * @dev Can only be called by the vault owner. Validates the total allocation does not exceed 100% and the length corresponds with the strategies array. * Emits a `StrategyAllocationsChanged` * @param allocations_ The array with the new allocations. * @param redistribute A boolean indicating whether to redistributes allocations. */ function changeAllocations( Allocation[] calldata allocations_, bool redistribute ) external nonReentrant onlyOwner takeFees { uint256 len = allocations_.length; if (len != strategies.length) revert InvalidLength(len, strateg...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Radiant Token Is Upgradeable And IncentiveController Could Be Changed In The Future

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-300
- **Submitter:** EPSec
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-300
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-300.txt

## Brief Summary

The RadiantV2Strategy contract interacts with the rToken (Radiant's AToken implementation), specifically using its getIncentivesController() function to fetch the incentives controller. However, the incentiveController is stored in memory during initialization instead of dynamically fetched whenever required. Since rToken is part of an upgradeable system, its incentives controller can be updated, leading to potential inconsistencies in reward distribution. The relevant code in AToken.sol shows the getIncentivesController() function, which is subject to change: function getIncentivesController() external view override returns (address) { return _incentivesController; } This design assumes th...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# `WithdrawalQueue` does not work with rebasing or airdrop tokens

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-57
- **Submitter:** serial-coder
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-57
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-57.txt

## Brief Summary

Note: per the contest's readme, any ERC20 tokens with balance modifications outside of the transfers (rebasing/airdrops) are in scope. When a user calls the ConcreteMultiStrategyVault contract's redeem() or withdraw() to burn their shares for withdrawing assets, if available assets are less than the expected amount to withdraw, the user's withdrawal request will be enqueued for later withdrawal through the WithdrawalQueue contract. The WithdrawalQueue will record the expected asset amount to withdraw. Once sufficient assets are available, the recipient will later receive the expected amount of assets when an admin executes the batchClaimWithdrawal(). Assume that the underlying asset is a re...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Attackers can steal rewards by frontrunning the harvesting of rewards and minting huge amount of vault's shares to reduce the rewards per share.

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-47
- **Submitter:** 0xStalin
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-47
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-47.txt

## Brief Summary

Attackers can steal rewards from legitimate users. Finding Description When harvesting rewards generated on the vault's strategies, only two things are taken into account when computing the value by which the rewardIndex will grow (rewards per share), which are, the total harvested rewards and the totalSupply of vault shares, nothing more. The problem with this implementation is that allows attackers to frontrun the harvesting of rewards, and, mint a huge amount of shares to inflate the totalSupply of shares, which will cause that all the harvested rewards to be also distributed for the attacker's freshly minted shares (assets that did not contribute to the generation of the rewards). There...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Lack of allocation validation during ConcreteMultiStrategyVault initialization can render vault non-functional

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-252
- **Submitter:** muellerberndt
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-252
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-252.txt

## Brief Summary

The function ValidateVaultParameters in MultiStrategyVaultHelper.sol, which is called during vault initialization, is supposed to check that the sum of basis points of all allocations doesn't exceed 10,000. It says so in the function description (line 112): * @custom:reverts AllotmentTotalTooHigh if the total strategy allocations exceed 100%. However, this check isn't actually performed. As a result, the vault can be initialized incorrectly, breaking the functionality of the vault unless the allocations are later repaired by an admin: If the sum of allocations is less than 100%, distributeAssetsToStrategies() will leave unallocated assets in the vault; If the sum of allocations is higher th...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Incorrect Zero-Deposit Utilization Rate Calculation Leading to False Protocol Health Metrics

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-315
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-315
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-315.txt

## Brief Summary

A bug exists in the calculateUtilization function of the EasyMathV2 library related to incorrect handling of the edge case where total deposits are zero but borrowing exists. The current implementation returns 0% utilization in this case, when it should return 100% utilization (full utilization). The root cause is in the initial validation: if (_totalDeposits == 0 || _totalBorrowAmount == 0) return 0; This logic error has serious implications for lending protocols using this library: Underreported utilization rates leading to incorrect interest rate calculations Potential economic exploits where high-risk states appear safe Misleading protocol health indicators Inaccurate risk metrics for p...

## Rejection Reason

Final severity/evaluation: Low; Valid Primary Sufficient

# Radiant Strategy's Lack of Fallback Mechanisms Creates Single Point of Failure Risk

- **Contest:** Concrete
- **Slug:** 2024-11-concrete
- **Submission:** F-113
- **Submitter:** Agontuk
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-11-concrete/submissions/F-113
- **Source snapshot:** competitions/2024-11-concrete/submissions/raw/F-113.txt

## Brief Summary

The Concrete Protocol implements a multi-strategy vault system where different strategies can be used to generate yield on user deposits. The RadiantV2Strategy contract is one such strategy that integrates with the Radiant lending protocol to generate yields by depositing user funds into Radiant's lending pools. The strategy interacts with Radiant's lending pools through the _protocolDeposit() and _protocolWithdraw() functions: function _protocolDeposit(uint256 amount_, uint256) internal virtual override { lendingPool.deposit(asset(), amount_, address(this), 0); } function _protocolWithdraw(uint256 amount_, uint256) internal virtual override { lendingPool.withdraw(asset(), amount_, address(...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.
