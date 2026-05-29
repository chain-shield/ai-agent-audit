# Benchmark Ground Truth: PoolTogether

## Accepted H/M Findings

# Accepted H/M Findings: PoolTogether

# [H-01] Any fee claim lesser than the total yieldFeeBalance as unit of shares is lost and locked in the PrizeVault contract

- **Contest:** PoolTogether
- **Slug:** 2024-03-pooltogether
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-03-pooltogether
- **Source snapshot:** competitions/2024-03-pooltogether/final_report.html

yieldFeeBalance as unit of shares is lost and locked in the PrizeVault contract Submitted by DarkTower, also found by 0xJoyBoy03, smbv-1923, d3e4, 0xlemon, trachev, aua_oo7, Afriauditor, Aymen0909, FastChecker, Dots, AgileJune, leegh, Tripathi, turvy_fuzz, GoSlang, McToady, zhaojie, radin100, yotov721, y4y, 0xkeesmark, 0xRiO, gesha17, iberry, 0xmystery, sammy, Fitro, Greed, 0xJaeger, wangxx2026, dd0x7e8, yvuchev, Abdessamed, Daniel526, kR1s, n1punp, AcT3R, SoosheeTheWise, valentin_s2304, btk, pa6kuda, Al-Qa-qa, asui, dvrkzy, crypticdefense, marqymarq10, DanielTan_MetaTrust, and Krace Any fee claim by the fee recipient lesser than the accrued internal accounting of the

yieldFeeBalance is lost and locked in the PrizeVault contract with no way to pull out the funds.

## Recommended Mitigation Steps

Adjust the claimYieldFeeShares to only deduct the amount claimed/minted:

function claimYieldFeeShares(uint256 _shares) external onlyYieldFeeRecipient { if (_shares == 0) revert MintZeroShares(); - uint256 _yieldFeeBalance = yieldFeeBalance; - if (_shares > _yieldFeeBalance) revert SharesExceedsYieldFeeBalance(_shares, _yieldFeeBalance); + if (_shares > yieldFeeBalance) revert SharesExceedsYieldFeeBalance(_shares, yieldFeeBalance); - yieldFeeBalance -= _yieldFeeBalance; + yieldFeeBalance -= _shares; _mint(msg.sender, _shares); emit ClaimYieldFeeShares(msg.sender, _shares); } trmid (PoolTogether) confirmed and commented:

Mitigated here.

Medium Risk Findings (8)

# [M-01] The winner can steal claimer fees, and force him to pay for the gas

- **Contest:** PoolTogether
- **Slug:** 2024-03-pooltogether
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-pooltogether
- **Source snapshot:** competitions/2024-03-pooltogether/final_report.html

Submitted by Al-Qa-qa, also found by Infect3d and souilos When the winner earns his reward he can either claim it himself, or he can let a claimer contract withdraw it on his behalf, and he will pay part of his reward for that. This is because the user will not pay for the gas fees; instead the claimer contract will pay it instead.

The problem here is that the winner can make the claimer pay for the gas of the transaction, without paying the fees that the claimer contract takes.

Claimer contracts are allowed for anyone to use them, transfer prizes to winners, and claim some fees; where the one who fired the transaction is the one who will pay for the fees, so he deserved those fees.

pt-v5-claimer/Claimer.sol#L120-L150 // @audit and one can call the function function claimPrizes (... ) external returns ( uint256 totalFees ) {...

if (!

feeRecipientZeroAddress ) {...

} return feePerClaim * _claim ( _vault, _tier, _winners, _prizeIndices, _feeRecipient, feePerClaim ); } As in the function, the function takes the winners and he passed the fee recipient and his fees (but it should not exceed the maxFees, which is initialized in the constructor).

Now we know that anyone can transfer winners’ prizes and claim some fees.

Before the prizes are claimed, the winner can initialize a hook before calling the PoolPrize::claimPrize. This is used if the winner wants to initialize another address as the receiver of the reward. The hook parameter is passed by parameters that are used to determine the correct winner (winner address, tier, prizeIndex ).

abstract/Claimable.sol#L85-L95 uint24 public constant HOOK_GAS = 150_000;...

function claimPrize (... ) external onlyClaimer returns ( uint256 ) { address recipient; if ( _hooks [ _winner ].

useBeforeClaimPrize ) { recipient = _hooks [ _winner ].

implementation.

beforeClaimPrize { gas:

HOOK_GAS }( _winner, _tier, _prizeIndex, _reward, _rewardRecipient ); } else { recipient = _winner; } if ( recipient == address ( 0 )) revert ClaimRecipientZeroAddress (); uint256 prizeTotal = prizePool.

claimPrize (... );...

} But to prevent OOG the gas is limited to 150K.

Now what the user can do to make the claimer pay for the transaction, and not pay any fees is:

He will make a beforeClaimPrize hook.

In this function, the user will simply claim his reward Claimer::claimPrizes(...params) but with settings no fees, and only passing his winning prize parameters (we got them from the hook).

The winner (attacker) will not do any further interaction to not make the tx go OOG (remember we have only 150k).

After the user claims his reward, he will simply return his address (the winner’s address).

The Claimer contract will go to claim this winner’s rewards, but it will return 0 as it is already claimed.

The Claimer will complete his process (claiming other prizes on behalf of winners).

The winner (attacker) will end up claiming his reward without paying for the transaction gas fees.

Note: The Claimer claiming function will not revert, as if the prize was already claimed the function will just emit an event and will not revert.

pt-v5-claimer/Claimer.sol#L194-L198 function _claim (... ) internal returns ( uint256 ) {...

try _vault.

claimPrize ( _winners [ w ], _tier, _prizeIndices [ w ][ p ], _feePerClaim, _feeRecipient ) returns ( uint256 prizeSize ) { if ( 0 != prizeSize ) { actualClaimCount ++; } else { // @audit Emit an event if the prize already claimed emit AlreadyClaimed ( _winners [ w ], _tier, _prizeIndices [ w ][ p ]); } catch ( bytes memory reason ) { emit ClaimError ( _vault, _tier, _winners [ w ], _prizeIndices [ w ][ p ], reason ); }...

} The only check that can prevent this attack is the gas cost of calling beforeClaimPrize hook.

We will call one function Claimer::claimPrizes() by only passing one winner, and without fees. We calculated the gas that can be used by installing protocol contracts (Claimer and PrizePool), then grab a test function that first the function we need, and we get these results:

Calling Claimer::claimPrize() costs 5292 gas if it did not claimed anything.

Calling PrizePool::claimePrize() costs 118124 gas.

So the total gas that can be used is $118,124 + 5292 = $123,416 which is smaller than HOOK_GAS by more than 25K, so the function will not revert because of OOG error, and the reentrancy will occur.

Another thing that may lead to a misunderstanding is that the Judger may say if this happens the function will go to beforeClaimPrize hook again leading to infinite loop and the transaction will go OOG. However, making the transaction beforeClaimPrize be fired to make a result and when called again do another logic is an easy task that can be made by implementing a counter or something. However, we did not implement this counter in our test. We just wanted to point out how the attack will work in our POC, but in real interactions, there should be some edge cases to take care of and further configurations to take care off.

## Recommended Mitigation Steps

We can check the prize state before and after the hook; if it changes from unclaimed to claimed, we can revert the transaction.

Claimable.sol:

function claimPrize(... ) external onlyClaimer returns (uint256) { address recipient; if (_hooks[_winner].useBeforeClaimPrize) { + bool isClaimedBefore = prizePool.wasClaimed(address(this), _winner, _tier, _prizeIndex); recipient = _hooks[_winner].implementation.beforeClaimPrize{ gas: HOOK_GAS }(... ); + bool isClaimedAfter = prizePool.wasClaimed(address(this), _winner, _tier, _prizeIndex); + if (isClaimedBefore == false && isClaimedAfter == true) { + revert("The Attack Occuared"); + } } else {... }...

} Note: We were writing this issue 30 minutes before ending of the audit - the mitigation review may not be the best, or may not work (we did not test it). Devs should keep this in mind when mitigating this issue.

## Assessed type

Reentrancy raymondfam (lookout) commented:

Claimable::claimPrize has the visibility of onlyClaimer denying the winner’s hook reentrancy. The winner can’t any prize unless he/she is the permitted claimer.

hansfriese (judge) decreased severity to Low Al-Qa-qa (warden) commented:

@hansfriese - There is a misunderstanding of this issue with its group ( 78 ), and I will illustrate how this occurs.

There are three contracts that we will deal with for making this exploit:

PrizeVault(Claimable): this is the claimable contract that has the function to give a winner his prize.

Claimer: The contract that has the authority to transfer the prize to the winner (from the Claimable contract).

PrizePool: The Contract that has the prize, which we will be called to claim the prize and give it to the winner.

The issue was rejected by replying that the onlyClaimer modifier prevents the hook from returning and calling the function again. So the judge thought that I said the beforeClaimHook will call Claimable::claimPrize(), and this is not what I said in my report.

In my report, I said that the hook will go to the Claimer contract itself, and call the claimPrizes function (the function that is in Claimer contract that fires PrizeVault(Claimable)::claimPrize.

beforeClaimHook will call Claimer::claimPrizes() which will call Claimable::claimPrize() He will make a beforeClaimPrize hook.

In this function, the user will simply claim his reward Claimer::claimPrizes(...params) but with settings no fees, and only passing his winning prize parameters (we got them from the hook).

I think the conflict occurs as Claimer and Claimable are two different contracts, but they have similar names as well as the function names are also similar claimPrize and claimPrizes.

I illustrated in my report that the way the Claimer contract design is not restricted and, anyone can use it to claim fees and send prizes to the winners.

Claimer.sol#L120-L150 // @audit and one can call the function function claimPrizes (... ) external returns ( uint256 totalFees ) {...

if (!

feeRecipientZeroAddress ) {...

} return feePerClaim * _claim ( _vault, _tier, _winners, _prizeIndices, _feeRecipient, feePerClaim ); } According to @raymondfam’s comment for rejecting this issue:

Claimable::claimPrize has the visibility of onlyClaimer denying the winner’s hook reentrancy. The winner can’t any prize unless he/she is the permitted claimer.

Anyone can be a permitted claimer as the function that interacts with the PrizeVault(Claimable) is accessible to anyone, as I showed here and in my report.

In my report I did not say that the hook will go to fire PrizeVault(Claimable)::claimPrize directly, Instead, I said that it will call Claimer::claimPrizes().

So the pass will be the following:

We will use (MEV searcher) to represent the one that claims winners’ prizes using Claimer contract.

MEV searcher call Claimer::claimPrizes(...params), providing more than one winner.

When the malicious winner prize is the next one on the queue, beforeClaimPrize will be used to call Claimer::claimPrizes() again, using the parameters passed to it, and the winner prize will get claimed using MEV searcher gas, but without paying the fees to the MEV searcher.

Since the Prize already claimed, the MEV searcher will not be able to reclaim it again, and will gain 0 fees (Knowing that he is the one who paid for the gas in the first place).

PASS:

MEV--Claimer::claimPrizes(...).

PrizeVault(Claimabe)::claimPrize().

PrizeVault(Claimabe):WinnerHook:beforeClaimPrize().

WinnerHook--Claimer::claimPrizes() (with winner (attacker) params, no fees).

PrizeVault(Claimabe)::claimPrize() (with winner (attacker) params, no fees).

PrizeVault(Claimabe):WinnerHook:beforeClaimPrize() (with winner (attacker) params, no fees).

6.1 Make beforeClaimPrize just return winner address at this time.

PrizeVault(Claimabe):PrizePool::claimPrize([using no fees]) (with winner (attacker) params, no fees).

7.1 After the winner (attacker) claimed his reward using beforeClaimPrize(), the function beforeClaimPrize() will return the winner address.

7.2 beforeClaimPrize returned the winner address (note:

4, 5, 6 and 7 happens inside beforeClaimPrize() when the MEV searcher called claimer at 1.

7.3 After this, the function will complete its execution (and the MEV will go to claim the prize of the winner and earn fees).

PrizePool::claimPrize([using fees]) (with MEV searcher params, with fees).

Function return 0 as it is already claimed.

I highly encourage setting up the PoC and running it, I made a simulation of the normal state, and when the hook is used to steal rewards. By providing -vvvv, the call path will be viewed clearly.

All what I said here exists in my report, and I provided a runnable PoC that can be used by the judge to simulate the attack, it can be viewed by expanding the collapsed content from the triangle, and following setting up PoC instructions.

One additional thing: this issue is not a duplicate of issue 18, my issue illustrates how the winner can steal the fees, and force the MEV searcher to pay for the gas.

As I illustrated in the title there are two Impacts:

The fees will get stolen from the caller of Claimer contract (MEV searcher), and the winner will get them himself.

The Winner will force the caller (MEV searcher) to pay for the gas, which can be considered griefing.

hansfriese (judge) increased severity to Medium and commented:

Nice report! After checking again, I agree it’s a valid concern and Medium is appropriate.

trmid (PoolTogether) confirmed and commented:

Although this issue can be mitigated in the PrizeVault as described, we chose to update the PrizePool and Claimer contracts with logic that ensures a double prize claim will revert so that the griefer will not receive their prize. This removes any incentivization for the malicious hook to be set. (PRs linked for context) By fixing the issue in the prize pool and claimer, we save gas by avoiding the two additional external calls added in the original mitigation.

PrizePool.sol

## mitigation

here and Claimer.sol

## mitigation

here.

# [M-02] _maxYieldVaultWithdraw() uses yieldVault.convertToAssets()

- **Contest:** PoolTogether
- **Slug:** 2024-03-pooltogether
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-pooltogether
- **Source snapshot:** competitions/2024-03-pooltogether/final_report.html

_maxYieldVaultWithdraw() uses yieldVault.convertToAssets() Submitted by d3e4, also found by d3e4

## Recommended Mitigation Steps

Use yieldVault.previewRedeem(yieldVault.maxRedeem(address(this))).

## Assessed type

ERC4626 trmid (PoolTogether) confirmed and commented:

## Mitigation

here. Also see here for more details.

# [M-03] maxDeposit() uses yieldVault.maxDeposit() but _depositAndMint() uses yieldVault.mint()

- **Contest:** PoolTogether
- **Slug:** 2024-03-pooltogether
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-pooltogether
- **Source snapshot:** competitions/2024-03-pooltogether/final_report.html

maxDeposit() uses yieldVault.maxDeposit() but _depositAndMint() uses yieldVault.mint() Submitted by d3e4 maxDeposit() might return a value greater than can be deposited, violating EIP-4626.

## Recommended Mitigation Steps

Use yieldVault.previewRedeem(yieldVault.maxMint()).

## Assessed type

ERC4626 hansfriese (judge) decreased severity to Low and commented:

It’s a valid concern and QA is more appropriate due to the low impact.

I will mark as grade-a with some unique issues.

d3e4 (warden) commented:

Isn’t a violation of EIP-4626, for a vault that claims to be compliant, at least a Medium severity because of the integration issues it implies?

Note that being EIP-4626 compliant is explicitly stated in the README and that adherence to this was listed as one of the Attack ideas.

hansfriese (judge) increased severity to Medium and commented:

After checking again, I agree Medium is more appropriate as it may violate ERC4626 compliance.

trmid (PoolTogether) acknowledged and commented:

After further evaluation, the suggested mitigation seems to cause issues in common ERC4626 yield vaults since maxMint commonly returns type(uint256).max and calling previewRedeem or previewMint with such a high value also commonly causes an overflow error on conversion.

As long as the yield vault maxDeposit function takes into account any internal supply limits, the current implementation is unlikely to have any compatibility issues and will be left as-is.

# [M-04] Lack of Slippage Protection in withdraw / redeem Functions of the Vault

- **Contest:** PoolTogether
- **Slug:** 2024-03-pooltogether
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-pooltogether
- **Source snapshot:** competitions/2024-03-pooltogether/final_report.html

withdraw / redeem Functions of the Vault Submitted by Aymen0909, also found by cheatc0d3, turvy_fuzz, trachev, FastChecker, Tripathi, Abdessamed, 0xmystery, btk, and Giorgio

- https://github.com/code-423n4/2024-03-pooltogether/blob/main/pt-v5-vault/src/PrizeVault.sol#L454-L472
- https://github.com/code-423n4/2024-03-pooltogether/blob/main/pt-v5-vault/src/PrizeVault.sol#L355-L366
- https://github.com/code-423n4/2024-03-pooltogether/blob/main/pt-v5-vault/src/PrizeVault.sol#L489-L508
Issue Description When a user who has deposited assets into the PrizeVault wishes to withdraw (or redeem) them, they can do so by calling either the withdraw or redeem functions.

Under normal conditions in the vault, users expect to receive their full deposited asset amount back, as there is a 1:1 exchange ratio between the asset amount and shares.

However, if the underlying yield vault experiences a loss, this exchange rate will decrease. This is highlighted in the previewWithdraw function (or previewRedeem / convertToAssets function):

function previewWithdraw ( uint256 _assets ) public view returns ( uint256 ) { uint256 _totalAssets = totalAssets (); // No withdrawals can occur if the vault controls no assets.

if ( _totalAssets == 0 ) revert ZeroTotalAssets (); uint256 totalDebt_ = totalDebt (); if ( _totalAssets >= totalDebt_ ) { return _assets; } else { // Follows the inverse conversion of `convertToAssets` return _assets.

mulDiv ( totalDebt_, _totalAssets, Math.

Rounding.

Up ); } function convertToAssets ( uint256 _shares ) public view returns ( uint256 ) { uint256 totalDebt_ = totalDebt (); uint256 _totalAssets = totalAssets (); if ( _totalAssets >= totalDebt_ ) { return _shares; } else { // If the vault controls fewer assets than what has been deposited, a share will be worth a // proportional amount of the total assets. This can happen due to fees, slippage, or loss // of funds in the underlying yield vault.

return _shares.

mulDiv ( _totalAssets, totalDebt_, Math.

Rounding.

Down ); } function totalAssets () public view returns ( uint256 ) { return yieldVault.

convertToAssets ( yieldVault.

balanceOf ( address ( this ))) + _asset.

balanceOf ( address ( this )); } As shown, if the underlying yield vault experiences a loss, the total assets of the vault given by totalAssets() will decrease and might go below the total shares value totalDebt().

This will trigger the second if statement block, in which the calculated asset (or shares) amount will be converted using the ratio totalAssets/totalDebt (or its inverse for shares).

When a user redeems (or withdraws), if the yield vault experiences a loss while their transaction is pending (waiting in the mempool) and the total assets drop below the total vault debt, they will receive fewer assets after calling either the withdraw / redeem functions than they expected. Instead of a 1:1 ratio, they will use the current totalAssets/totalDebt ratio.

To illustrate this issue, consider the following scenario:

In the vault, we have totalAssets() = 51000 and totalDebt() = 50000.

Bob previously deposited 1000 tokens and received 1000 shares in return.

Bob wants to redeem 500 shares and expects to get his 500 tokens back since currently totalAssets() > totalDebt() (so there’s a 1:1 ratio).

While Bob’s transaction is pending in the mempool, the yield vault experiences a loss, and totalAssets() = 46000 drops below totalDebt() = 50000.

When Bob’s transaction goes through, he will receive:

(500 * 46000) / 50000 = 460 < 500.

Thus, instead of receiving 500 tokens, he only gets 460 back, resulting in a loss of 40 tokens.

If Bob had known that the yield vault had experienced a loss, he would have waited until the exchange rate increased again to withdraw his full amount.

This issue is also present in the withdraw function, but in that case, the user will be burning more shares to get the same amount and still incurring a loss.

## Impact

Both withdraw / redeem functions lack slippage protection, which can lead to users losing funds in the event of a yield vault loss.

Tools Used VS Code

## Recommended Mitigation

Both withdraw / redeem functions should include slippage protection parameters provided by the users (either minimum amount out for redeem function or maximum shares in for withdraw function).

## Assessed type

Context trmid (PoolTogether) confirmed and commented:

Providing depositors with slippage protection is a nice improvement! The suggested mitigation would break the ERC4626 spec requirements on the PrizeVault, so an alternate strategy will likely be used that either adds additional deposit and withdraw functions that have slippage params, or provides an external router that can provide this protection for the depositor.

d3e4 (warden) commented:

Is this not a design suggestion rather than a bug, and should belong in Analysis? To remain EIP-4626 compliant a slippage protection would have to be implemented in entirely new functionality; not by amending already existing code. Therefore, it cannot be said that there is something wrong with the current code.

btk (warden) commented:

@d3e4 - EIP-4626 states that:

If implementors intend to support EOA account access directly, they should consider adding an additional function call for deposit/mint/withdraw/redeem with the means to accommodate slippage loss or unexpected deposit/withdrawal limits, since they have no other means to revert the transaction if the exact output amount is not achieved.

hansfriese (judge) commented:

I think it’s eligible to be a Medium for the specific withdraw/redeem logic of PrizeVault. Will maintain it as a Medium.

trmid (PoolTogether) commented:

## Mitigation

here.

# [M-05] yieldFeeBalance wouldn’t be claimed after calling transferTokensOut()

- **Contest:** PoolTogether
- **Slug:** 2024-03-pooltogether
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-pooltogether
- **Source snapshot:** competitions/2024-03-pooltogether/final_report.html

yieldFeeBalance wouldn’t be claimed after calling transferTokensOut() Submitted by 0xhunter20 yieldFeeBalance wouldn’t be claimed after calling transferTokensOut() due to the twab supply limit.

## Recommended Mitigation Steps

liquidatableBalanceOf() shouldn’t apply yieldFeePercentage to compare with _maxAmountOut when _tokenOut == address(this).

function liquidatableBalanceOf ( address _tokenOut ) public view returns ( uint256 ) { uint256 _totalSupply = totalSupply (); uint256 _maxAmountOut; if ( _tokenOut == address ( this )) { // Liquidation of vault shares is capped to the TWAB supply limit.

_maxAmountOut = _twabSupplyLimit ( _totalSupply ); } else if ( _tokenOut == address ( _asset )) { // Liquidation of yield assets is capped at the max yield vault withdraw plus any latent balance.

_maxAmountOut = _maxYieldVaultWithdraw () + _asset.

balanceOf ( address ( this )); } else { return 0; } // The liquid yield is computed by taking the available yield balance and multiplying it // by (1 - yieldFeePercentage), rounding down, to ensure that enough yield is left for the // yield fee.

uint256 _liquidYield = _availableYieldBalance ( totalAssets (), _totalDebt ( _totalSupply )); if ( _tokenOut == address ( this )) { if ( _liquidYield >= _maxAmountOut ) { //compare before applying yieldFeePercentage _liquidYield = _maxAmountOut; } _liquidYield = _liquidYield.

mulDiv ( FEE_PRECISION - yieldFeePercentage, FEE_PRECISION ); } else { _liquidYield = _liquidYield.

mulDiv ( FEE_PRECISION - yieldFeePercentage, FEE_PRECISION ); if ( _liquidYield >= _maxAmountOut ) { //same as before _liquidYield = _maxAmountOut; } return _liquidYield; }

## Assessed type

Invalid Validation hansfriese (judge) commented:

The impact is the same as #91 but the flaw still exists after mitigating #91 because liquidatableBalanceOf() doesn’t use the newly accumulated yield fees while checking _twabSupplyLimit.

trmid (PoolTogether) confirmed and commented:

Similar to #91, this issue outlines the need for the TWAB supply limit checks to account for the yield fee balance so that the entire yield fee balance is always available to be realized as shares.

## Mitigation

here.

# [M-06] Funds locked due to missing transfer check

- **Contest:** PoolTogether
- **Slug:** 2024-03-pooltogether
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-pooltogether
- **Source snapshot:** competitions/2024-03-pooltogether/final_report.html

Submitted by CodeWasp, also found by Al-Qa-qa, d3e4, 0xmystery, and Drynooo All of the user’s funds are unretrievably locked in the PrizeVault contract.

A combination of issues allows for the following scenario:

Alice invokes _withdraw(receiver, assets) (via burn() or withdraw() ).

The contract computes the number of shares to redeem, via previewWithdraw(assets).

The contract redeems as many shares, but the ERC 4626-compliant vault returns fewer shares than expected. At this point, the contract holds fewer than assets tokens.

The contract attempts to transfer assets to the receiver. This fails due to insufficient funds, but the ERC 20-compliant token does not revert (only returns false ).

At this point, Alice’s assets are locked in the PrizeVault contract. They cannot be withdrawn at a later point, because the corresponding prize vault and yield vault shares have been burned.

The exploit relies on insufficient handling of two corner cases of ERC-20 and ERC-4246:

ERC-20 does not stipulate that transfer must throw if the message sender holds insufficient balance. Instead, returning false is compliant with ERC-20 and implemented by many tokens, including BAT, cUSDC, EURS, HuobiToken, ZRX and many more.

ERC-4626 does not stipulate that redeem(previewWithdraw(assets)) transfers at least assets. In particular, redeem(shares,...) only guarantees that exactly shares are burned. The only guaranteed way to gain a certain amount of assets is by calling withdraw(assets,...).\ While this is the most standards-compliant scenario, a malicious vault could simply not transfer the required tokens on purpose, and still trigger the same effect as described above.

## Recommended Mitigation Steps

We recommend to fix both the ERC-20 transfer and ERC-4626 withdrawal.

For the first, it is easiest to rely on OpenZeppelin’s SafeERC20 safeTransfer function:

diff --git a/pt-v5-vault/src/PrizeVault.sol b/pt-v5-vault/src/PrizeVault.sol index fafcff3..de69915 100644 --- a/pt-v5-vault/src/PrizeVault.sol +++ b/pt-v5-vault/src/PrizeVault.sol @@ -936,7 +936,7 @@ contract PrizeVault is TwabERC20, Claimable, IERC4626, ILiquidationSource, Ownab yieldVault.redeem(_yieldVaultShares, address(this), address(this)); } if (_receiver != address(this)) { - _asset.transfer(_receiver, _assets); + _asset.safeTransfer(_receiver, _assets); } This already mitigates the erroneous locking of assets.

In addition, we recommend to ensure that at least the necessary amount of shares is withdrawn from the yield vault. In the simplest form, this can be ensured by invoking withdraw directly:

diff --git a/pt-v5-vault/src/PrizeVault.sol b/pt-v5-vault/src/PrizeVault.sol index fafcff3..9bb0653 100644 --- a/pt-v5-vault/src/PrizeVault.sol +++ b/pt-v5-vault/src/PrizeVault.sol @@ -930,10 +930,7 @@ contract PrizeVault is TwabERC20, Claimable, IERC4626, ILiquidationSource, Ownab // latent balance, we don't need to redeem any yield vault shares.

uint256 _latentAssets = _asset.balanceOf(address(this)); if (_assets > _latentAssets) { - // The latent balance is subtracted from the withdrawal so we don't withdraw more than we need.

- uint256 _yieldVaultShares = yieldVault.previewWithdraw(_assets - _latentAssets); - // Assets are sent to this contract so any leftover dust can be redeposited later.

- yieldVault.redeem(_yieldVaultShares, address(this), address(this)); + yieldVault.withdraw(_assets - _latentAssets, address(this), address(this)); } if (_receiver != address(this)) { _asset.transfer(_receiver, _assets); If a tighter bound on redeemed shares is desired, the call to previewWithdraw / redeem should be followed by a withdraw of the outstanding assets:

diff --git a/pt-v5-vault/src/PrizeVault.sol b/pt-v5-vault/src/PrizeVault.sol index fafcff3..622a7a6 100644 --- a/pt-v5-vault/src/PrizeVault.sol +++ b/pt-v5-vault/src/PrizeVault.sol @@ -934,6 +934,13 @@ contract PrizeVault is TwabERC20, Claimable, IERC4626, ILiquidationSource, Ownab uint256 _yieldVaultShares = yieldVault.previewWithdraw(_assets - _latentAssets); // Assets are sent to this contract so any leftover dust can be redeposited later.

yieldVault.redeem(_yieldVaultShares, address(this), address(this)); + + // Redeeming `_yieldVaultShares` may have transferred fewer than the required assets.

+ // Ask for the outstanding assets directly.

+ _latentAssets = _asset.balanceOf(address(this)); + if (_assets > _latentAssets) { + yieldVault.withdraw(_assets - _latentAssets); + } } if (_receiver != address(this)) { _asset.transfer(_receiver, _assets);

## Assessed type

ERC20 trmid (PoolTogether) confirmed and commented:

I would like to add that if a “compatible ERC4626 yield vault returns less assets than expected”, then it is not actually ERC4626 compatible as these behaviors are required in the spec. That being said, there are likely to be some yield vaults that have errors like this and it is a good thing if we can protect against it without inhibiting the default experience!

The safeTransfer addition seems sufficient, while the other recommended mitigations are unnecessary and would break the “dust collector” strategy that the prize vault employs.

## Mitigation

here.

hansfriese (judge) decreased severity to Medium and commented:

The impact is critical if _asset.transfer() fails silently and it will be mitigated from this known issue. So according to this criteria, this issue might be OOS if it’s fully mitigated by adding safeTransfer.

But another impact is withdraw() might revert when yieldVault.redeem() returns fewer assets than requested and Medium is appropriate.

# [M-07] PrizeVault.maxDeposit() doesn’t take into account produced fees

- **Contest:** PoolTogether
- **Slug:** 2024-03-pooltogether
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-pooltogether
- **Source snapshot:** competitions/2024-03-pooltogether/final_report.html

PrizeVault.maxDeposit() doesn’t take into account produced fees Submitted by pa6kuda, also found by 0xhunter20 and Afriauditor

- https://github.com/code-423n4/2024-03-pooltogether/blob/480d58b9e8611c13587f28811864aea138a0021a/pt-v5-vault/src/PrizeVault.sol#L368-L392
- https://github.com/code-423n4/2024-03-pooltogether/blob/480d58b9e8611c13587f28811864aea138a0021a/pt-v5-vault/src/PrizeVault.sol#L685
- https://github.com/code-423n4/2024-03-pooltogether/blob/480d58b9e8611c13587f28811864aea138a0021a/pt-v5-vault/src/PrizeVault.sol#L619
Summary Currently, PrizeVault.maxDeposit() calculates the maximum possible amount of deposit without taking into account produced fees. That means if there is already maxed deposited amount of asset that is calculated by the current implementation in PrizeVault.maxDeposit(), yieldFeeRecipient can’t withdraw shares with PrizeVault.claimYieldFeeShares() because in that case _mint() will revert because of overflow. A lot of low-price tokens can exceed the limit of type(uint96).max with ease. For example, to make a deposit with maxDeposit() value with LADYS token it’s needed only $13568 (as of 08-03-2024).

## Impact

If a user makes a maximum allowed deposit that is calculated by the current implementation of PrizeVault.maxDeposit(), yieldFeeRecipient can’t withdraw fees if they are available.

## Recommended Mitigation Steps

Add function to withdraw fees in asset or change function PrizeVault.maxDeposit() to calculate max deposit with taking into account produced fees:

function maxDeposit(address) public view returns (uint256) { uint256 _totalSupply = totalSupply(); uint256 totalDebt_ = _totalDebt(_totalSupply); if (totalAssets() < totalDebt_) return 0; // the vault will never mint more than 1 share per asset, so no need to convert supply limit to assets uint256 twabSupplyLimit_ = _twabSupplyLimit(_totalSupply); uint256 _maxDeposit; uint256 _latentBalance = _asset.balanceOf(address(this)); uint256 _maxYieldVaultDeposit = yieldVault.maxDeposit(address(this)); if (_latentBalance >= _maxYieldVaultDeposit) { return 0; } else { unchecked { _maxDeposit = _maxYieldVaultDeposit - _latentBalance; } - return twabSupplyLimit_ < _maxDeposit ? twabSupplyLimit_: _maxDeposit;

+ return twabSupplyLimit_ < _maxDeposit ? twabSupplyLimit_ - yieldFeeBalance: _maxDeposit - yieldFeeBalance; }

## Assessed type

Math trmid (PoolTogether) confirmed and commented:

The TWAB max supply limit is a known issue with the prize vault and the deployer is expected to evaluate the possibility of the limit being exceeded before deploying a new prize vault. This issue has demonstrated that any yield fees accrued in this state may end up locked in the prize vault until enough withdrawals occur to free up the TWAB supply limit. This is undesirable behaviour, since all funds that have entered the prize vault through deposits or yield should be able to be taken out in these unexpected circumstances.

## Mitigation

here.

# [M-08] Permit doesn’t work with DAI

- **Contest:** PoolTogether
- **Slug:** 2024-03-pooltogether
- **Finding ID:** M-08
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-pooltogether
- **Source snapshot:** competitions/2024-03-pooltogether/final_report.html

Submitted by carrotsmuggler, also found by 0xabhay, Timenov, and Omik The function depositWithPermit in the PrizeVault.sol contract is used with permit options so that users can submit a signed message. They then can use that to give allowance to the contract to extract the tokens required for the deposit.

IERC20Permit ( address ( _asset )).

permit ( _owner, address ( this ), _assets, _deadline, _v, _r, _s ); The issue is that the test suite shows that the protocol aims to use sDAI, the dai savings rate, but the DAI token’s permit signature is different. From the contract at address 0x6B175474E89094C44Da98b954EedeAC495271d0F, we see the permit function:

function permit ( address holder, address spender, uint256 nonce, uint256 expiry, bool allowed, uint8 v, bytes32 r, bytes32 s ) external Due to the missing nonce field, DAI, a token which allows permit based interactions, cannot be used with signed messages for depositing into sDAI vaults. Due to the wrong parameters, the permit transactions will revert.

## Recommended Mitigation Steps

For the special case of DAI token, allow a different implementation of the permit function which allows a nonce variable.

## Assessed type

Token-Transfer trmid (PoolTogether) acknowledged and commented:

This is indeed a valuable insight and issue to be acknowledged, but no mitigation will be applied since the depositWithPermit function is meant as a quality of life improvement for depositors and adding additional logic to it’s operation could introduce unexpected vulnerabilities.

Infect3d (warden) commented:

@hansfriese, I think this submission should be considered as QA. A deposit function is made available, meaning there is still a possibility for users to deposit DAI (requiring them to approve first manually, which can be managed by the front-end UX as in most dApps already).

Based on the C4 severity categorization, a medium severity require either a loss of funds, or to impact the availability of the protocol, which is not the case thanks to the simple deposit.

hansfriese (judge) commented:

2 — Med: Assets not at direct risk, but the function of the protocol or its availability could be impacted, or leak value with a hypothetical attack path with stated assumptions, but external requirements.

While deposit() can be used instead of depositWithPermit(), I still believe Medium is eligible due to the potential dysfunctionality of the primary function.

## Rejected Primary Findings

# Rejected Primary Findings: PoolTogether

No rejected primary findings were captured for this contest in the current corpus.

- **Rejected primary count:** 0
- **Primary submission rows captured:** 0
- **Submission status:** requires_authentication
