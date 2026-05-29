# Benchmark Ground Truth: Spectra

## Accepted H/M Findings

# Accepted H/M Findings: Spectra

# [M-01] PrincipalToken is not ERC-5095 compliant

- **Contest:** Spectra
- **Slug:** 2024-02-spectra
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-spectra
- **Source snapshot:** competitions/2024-02-spectra/final_report.html

Submitted by jnforja, also found by sl1, dimulski, wangxx2026 ( 1, 2 ), JohnSmith, 0xLogos, 14si2o_Flint, erosjohn, Aymen0909, Limbooo, Giorgio ( 1, 2 ), smaul, ZanyBonzy, 0xhacksmithh, Brenzee, btk, 0xDemon, lsaudit ( 1, 2 ), mrudenko, memforvik, Franklin, Shubham, and nmirchev8

- https://github.com/code-423n4/2024-02-spectra/blob/main/src/tokens/PrincipalToken.sol#L483-L485
- https://github.com/code-423n4/2024-02-spectra/blob/main/src/tokens/PrincipalToken.sol#L460-L462
- https://github.com/code-423n4/2024-02-spectra/blob/main/src/tokens/PrincipalToken.sol#L278-L287
- https://github.com/code-423n4/2024-02-spectra/blob/main/src/tokens/PrincipalToken.sol#L229-L237
Protocols that try to integrate with Spectra, expecting PrincipalToken to be ERC-5095 compliant, will face an array of issues that may damage Spectra’s brand and limit Spectra’s growth in the market.

## Recommended Mitigation Steps

PrincipalToken::redeem and PrincipalToken::withdraw should be changed to support a flow where msg.sender has EPI-20 approval over the owner’s principal tokens.

PrincipalToken::maxRedeem and PrincipalToken::maxWithdraw should be changed to return 0 when PrincipalToken is paused.

Dravee (Judge) decreased severity to Medium yanisepfl (sponsor) confirmed and commented:

Mitigated here.

# [M-02] All yield generated in the IBT vault can be drained by performing a vault deflation attack using the flash loan functionality of the Principal Token contract

- **Contest:** Spectra
- **Slug:** 2024-02-spectra
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-spectra
- **Source snapshot:** competitions/2024-02-spectra/final_report.html

Submitted by Arabadzhiev, also found by ArmedGoose and blutorque The current implementation of the PrincipalToken has a flash lending functionality:

function flashLoan ( IERC3156FlashBorrower _receiver, address _token, uint256 _amount, bytes calldata _data ) external override returns ( bool ) { if ( _amount > maxFlashLoan ( _token )) revert FlashLoanExceedsMaxAmount (); uint256 fee = flashFee ( _token, _amount ); _updateFees ( fee ); // Initiate the flash loan by lending the requested IBT amount IERC20 ( ibt ).

safeTransfer ( address ( _receiver ), _amount ); // Execute the flash loan if ( _receiver.

onFlashLoan ( msg.

sender, _token, _amount, fee, _data ) != ON_FLASH_LOAN ) revert FlashLoanCallbackFailed (); // Repay the debt + fee IERC20 ( ibt ).

safeTransferFrom ( address ( _receiver ), address ( this ), _amount + fee ); return true; } And as of now, this functionality is implemented in such a way, that it allows users to borrow the whole IBT balance of the PrincipalToken permissionlessly:

function maxFlashLoan ( address _token ) public view override returns ( uint256 ) { if ( _token != ibt ) { return 0; } // Entire IBT balance of the contract can be borrowed return IERC4626 ( ibt ).

balanceOf ( address ( this )); } This is fine on it’s own and it works as it should. However there is a specific case where it can be abused. If the IBT vault prices its shares using the following formula:

$sharePrice = {totalAssets \over totalShares}$ Then, it will fall-back to some default price value when its totalAssets and totalShares values are equal to zero. Most usually that is the value of 1. Such is the case with the OpenZeppelin ERC4626 vault implementation, which is the most commonly used ERC4626 base implementation:

function _convertToAssets ( uint256 shares, Math.Rounding rounding ) internal view virtual returns ( uint256 ) { return shares.

mulDiv ( totalAssets () + 1, totalSupply () + 10 ** _decimalsOffset (), rounding ); } In that case, if the PerincipalToken contract happens to hold all of the IBT supply, a malicious lender can come in and perform the following exploit:

1. Take a flash loan from the PrincipalToken contract that is exactly equal to its IBT balance 2. Redeem all of the borrowed shares in the IBT vault for their underlying asset value 3. Mint back the borrowed IBT shares + the required flash loan fee shares from the vault 4. Pay back the flash loan + the flash loan fee What has just happened in the above described scenario is that the malicious lender has successfully reset the IBT vault share price to its default value, by redeeming all of the vault’s shares for all of its underlying assets. Then, they have minted back the previously redeemed shares plus the required flash loan fee shares at the default price and finally paid back the flash loan with those. Ultimately, what the attacker managed to accomplish is that they managed to get

totalIBTSupply * (initialIBTPrice - defaultIBTPrice) of underlying IBT assets at the expense of a single flash loan fee, while leaving the PerincipalToken contract’s users with a massive loss. More specifically, the users of the contract will lose all of their accumulated yield and potentially even more than that, depending on the IBT price at which they deposited into the PT and how far down it will be able to be deflated.

## Recommended Mitigation Steps

In the PrincipalToken::flashLoan function, verify that the IBT rate/price has not decreased once the flash loan has been repaid:

function flashLoan( IERC3156FlashBorrower _receiver, address _token, uint256 _amount, bytes calldata _data ) external override returns (bool) { if (_amount > maxFlashLoan(_token)) revert FlashLoanExceedsMaxAmount(); uint256 fee = flashFee(_token, _amount); _updateFees(fee); + uint256 initialIBTRate = IERC4626(ibt).convertToAssets(ibtUnit); // Initiate the flash loan by lending the requested IBT amount IERC20(ibt).safeTransfer(address(_receiver), _amount); // Execute the flash loan if (_receiver.onFlashLoan(msg.sender, _token, _amount, fee, _data) != ON_FLASH_LOAN) revert FlashLoanCallbackFailed(); // Repay the debt + fee IERC20(ibt).safeTransferFrom(address(_receiver), address(this), _amount + fee);

+ uint256 postLoanRepaymentIBTRate = IERC4626(ibt).convertToAssets(ibtUnit); + if (postLoanRepaymentIBTRate < initialIBTRate) revert FlashLoanDecreasedIBTRate(); return true; } Dravee (Judge) decreased severity to Medium and commented:

As per the conversation with the sponsor under 240 and given that the sponsor agreed that the finding could either be low or medium, I’ll acknowledge this bug as being more than a low. Although the edge case was mentioned to be unlikely, there’s still value in the mitigation (we never know how this could turn out to be further exploited).

Selecting the current report as it’s the most complete (although the remediation is too restrictive).

yanisepfl (Sponsor) acknowledged via duplicate #240 and commented:

Hello all, Thanks for the interesting conversation.

After discussing this issue and #111 with the team, we came to the conclusion that:

Having all the IBTs concentrated in our PTs is a very uncommon scenario. In particular, the usefulness of Spectra also relies on markets and if most of the IBTs are in our vaults then that would imply none or few are used as liquidity in the markets.

As it was rightly mentioned by @kazantseff, it is mostly an issue on the IBT 4626 not to be protected against vault price resets. Our protocol is neutral, notices the rate change and act accordingly (as per our design).

We therefore consider it a low or medium severity issue.

Concerning the mitigation, we believe there is no better solution than to have dead shares. The mitigation proposed in #111 is too restrictive (e.g. imprecisions) and the one proposed here wouldn’t work depending on the attacker’s PTs/YTs/IBTs ownership.

I hope this helps!

Edit: We acknowledge this issue. It will be clearly specified in our UI and documentations that users should be careful where they invest their funds. In particular, they are expected to make sure the IBTs follow the ERC 4626 and that their rate cannot be easily controlled by someone (e.g. price reset attack, see

- https://github.com/OpenZeppelin/openzeppelin-contracts/issues/3800
&

- https://github.com/OpenZeppelin/openzeppelin-contracts/pull/3979
). In particular, the PoC provided here does not work on Open Zeppelin’s 4626.

For full discussion, see duplicate issue #240.

## Rejected Primary Findings

# Rejected Primary Findings: Spectra

No rejected primary findings were captured for this contest in the current corpus.

- **Rejected primary count:** 0
- **Primary submission rows captured:** 0
- **Submission status:** requires_authentication
