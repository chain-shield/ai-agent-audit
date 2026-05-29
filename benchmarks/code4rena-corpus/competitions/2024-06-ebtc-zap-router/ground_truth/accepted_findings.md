# Accepted H/M Findings: eBTC Zap Router

# [M-01] Incorrect comparison logic in post-operation checks

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-06-ebtc-zap-router
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/final_report.html

Submitted by Nexarion, also found by chaduke, SBSecurity, lian886, Drynooo, stacey, jesjupyter, and Rhaydden The _doCheckValueType function, which is crucial for post-operation validation, contains reversed comparison logic for the gte (greater than or equal to) and lte (less than or equal to) operators. This reversal causes the function to perform the opposite checks than intended, potentially allowing operations to pass validation when they should fail, or fail when they should pass.

This issue could lead to:

Incorrect validation of CDP (Collateralized Debt Position) states after operations.

Potential manipulation of the system by exploiting these misaligned checks.

## Recommended Mitigation Steps

To fix this issue:

Reverse the comparisons in the _doCheckValueType function for the gte and lte operators.

Update the function as follows:

function _doCheckValueType ( CheckValueAndType memory check, uint256 valueToCheck ) internal { if ( check.

operator == Operator.

skip ) { // Early return return; } else if ( check.

operator == Operator.

gte ) { require ( valueToCheck >= check.

value, "!LeverageMacroReference: gte post check" ); } else if ( check.

operator == Operator.

lte ) { require ( valueToCheck <= check.

value, "!LeverageMacroReference: lte post check" ); } else if ( check.

operator == Operator.

equal ) { require ( check.

value == valueToCheck, "!LeverageMacroReference: equal post check" ); } else { revert ( "Operator not found" ); }

## Assessed type

Context wtj2021 (Badger) acknowledged via duplicate Issue #18 0xsomeone (judge) increased severity to High GalloDaSballo (Badger) commented:

We disagree that the finding is of High Severity and believe it’s logically equivalent to a lack of a slippage check, more commonly classified as Medium Severity.

Additionally a slippage check is implicitly present when using 0x for swaps.

The check is a important check to have but a lack of it working well doesn’t open up to meaningful losses due to other safeguards.

0xsomeone (judge) commented:

@GalloDaSballo - My initial judgment of this submission as high was based on the fact that the code performs an invalid operation and is a sensitive dependency with a high likelihood of vulnerability re-surfacing if left unpatched due to its presence in a utility library. In detail, the impact would be considered medium but the incidence itself merits a “high” rating as it is incorrectly executed every time the function is invoked.

Given that the function is present in a generic library, its impact would vary depending on the library’s usage and it is not unreasonable to assume that, if left unpatched, it would lead to a higher severity vulnerability surfacing.

I believe a high-severity rating is justified based on the above analysis, however, I am keen to hear more feedback especially if you believe that any of my statements are incorrect.

GalloDaSballo (Badger) commented:

From our perspective the check is tightly coupled with these parameters.

And was meant to be a generic way to perform checks on those values not to be re-used in other contracts.

0xsomeone (judge) decreased severity to Medium and commented:

After closely reviewing the codebase, the function does not appear to be utilized anywhere else except for the LeverageMacroBase contract itself. Based on the fact that the demonstratable vulnerability would be capped at medium severity and the prospective vulnerabilities that can arise from this code flaw are not readily apparent or guaranteed, I will proceed with downgrading this submission to medium risk.

This decision is final and no further feedback is expected in relation to this submission.

# [M-02] Staking ETH incorrectly assumes revert bubbling

- **Contest:** eBTC Zap Router
- **Slug:** 2024-06-ebtc-zap-router
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-06-ebtc-zap-router
- **Source snapshot:** competitions/2024-06-ebtc-zap-router/final_report.html

Submitted by 0xBinChook, also found by debo, pep7siup, DemoreX, Greed, alix40, and Chad0 When EbtcLeverageZapRouter::openCdpWithEth and EbtcLeverageZapRouter::adjustCdpWithEth stake ETH with Lido, the underlying function performing the call assumes that any exceptions (from Lido) will be bubbled up, but with call being a low level rather than a Solidity function, it is not the case.

Lido stETH has two cases where a revert on depositing ETH for staking may be encountered, when deposits are paused or when staking limits are enabled and it is exceeded.

If Lido returns a revert it is ignored and the entire CDP flow continues (permits, flash loans, account synching, CDP initialization, eBTC minting), with a revert only being cause only by one of the last step of the flow, moving the collateral to the active pool.

## Recommended Mitigation Steps

Use submit instead, as that will propagate any revert.

In ZapRouterBase::\_depositRawEthIntoLido():

function _depositRawEthIntoLido(uint256 _initialETH) internal returns (uint256) { // check before-after balances for 1-wei corner case uint256 _balBefore = stEth.balanceOf(address(this)); // TODO call submit() with a referral?

- payable(address(stEth)).call{value: _initialETH}(""); + stETH.submit{value: _initialETH}(address(0)); uint256 _deposit = stEth.balanceOf(address(this)) - _balBefore; return _deposit; } The msg.sender could also be included for the referral, as that will be included by Lido their emitted event.

function _depositRawEthIntoLido(uint256 _initialETH, address _referral) internal returns (uint256) { // check before-after balances for 1-wei corner case uint256 _balBefore = stEth.balanceOf(address(this)); // TODO call submit() with a referral?

- payable(address(stEth)).call{value: _initialETH}(""); + stETH.submit{value: _initialETH}(_referral); uint256 _deposit = stEth.balanceOf(address(this)) - _balBefore; return _deposit; } GalloDaSballo (Badger) confirmed and commented:

We agree that the finding is valid, but believe it should be downgraded to QA. The check is missing and will cause the delta returned to be 0.

The stETH amount is checked a few lines after, and a 0 amount, or a 0 change will cause every operation to revert due to insufficient change amount:

Open Cdp would revert here:

_requireAtLeastMinNetStEthBalance ( _stEthDepositAmount - LIQUIDATOR_REWARD ); On adjustCDP:

_requireZeroOrMinAdjustment ( _params.

debtChange ); _requireZeroOrMinAdjustment ( _params.

stEthBalanceChange ); _requireZeroOrMinAdjustment ( _params.

stEthMarginBalance ); The operation will also revert later once it tries to send an insufficient amount of tokens to the protocol.

For these reasons, we will fix the bug, but believe it’s QA.

0xsomeone (judge) increased severity to High and commented:

The Warden has demonstrated how a low-level interaction with the stETH system has not adequately handled the bool flag yielded which would indicate whether the call was successful or not, causing revert errors to be ignored.

I am not sure I fully agree with the Sponsor, as I believe certain code paths and CDP operations would result in the vulnerability regardless of the additional security checks imposed in the routers.

Specifically, a healthy CDP that has its debt increased and collateral added to it simultaneously via the ETH or WETH to stETH deposit flows would satisfy all conditions and be executed. Here’s a quick PoC that can be added to the LeverageZaps.t.sol file:

function test_marginErrorSilent () public { seedActivePool (); ( address user, bytes32 cdpId ) = createLeveragedPosition ( MarginType.

stETH ); IEbtcZapRouter.

PositionManagerPermit memory pmPermit = createPermit ( user ); uint256 debtChange = 1e18; uint256 marginIncrease = 0.5e18; uint256 collValue = _debtToCollateral ( debtChange ) * COLLATERAL_BUFFER / 10000; uint256 flAmount = _debtToCollateral ( debtChange ); _before (); vm.

startPrank ( user ); leverageZapRouter.

adjustCdpWithEth {value:

marginIncrease }( cdpId, _getAdjustCdpParams ( flAmount, int256 ( debtChange ), int256 ( collValue ), int256 ( marginIncrease ), false ), abi.

encode ( pmPermit ), _getExactInDebtToCollateralTradeData ( debtChange ) ); vm.

stopPrank (); _after (); // Test zap fee assertEq ( eBTCToken.

balanceOf ( testFeeReceiver ), ( 1e18 + debtChange ) * defaultZapFee / 10000 ); // Demonstrate a non-zero balance remains assertEq ( address ( leverageZapRouter ).

balance, 0 ); _checkZapStatusAfterOperation ( user ); } I believe that the above PoC sufficiently justifies a high severity risk rating as the funds transmitted alongside the call are lost and a CDP adjustment can leave the position in a worse-off state than the caller expected due to the unprocessed margin. To note, more PoCs can be observed in #37 and #22 that demonstrate the flaw and the possibility that funds will indeed be locked in the router.

A subset of submissions has been marked as no-reward due to lacking sufficient justification as to why the low-level interaction must be validated and how it can be exploited. Simply pointing out that the low-level call remains unchecked is insufficient for this particular vulnerability as it is identical to re-hashing static analysis output without properly understanding its ramifications.

To note, the primary submission of this duplicate set might change after PJQA as this submission details the vulnerability nicely but fails to demonstrate a valid exploitation path in its PoC.

Slavcheww (warden) commented:

@0xsomeone - I believe that the above PoC sufficiently justifies a high severity risk rating as the funds transmitted alongside the call are lost and a CDP adjustment can leave the position in a worse-off state than the caller expected due to the unprocessed margin.

It’s not valid, as like @GalloDaSballo stated and it’s just visible, if you adjust the position by 0 nothing worse can happen. In addition, there is a slippage ensuring that the position is as the user wants it, and a worse condition as you stated will be impossible.

What should the provided test show? This perhaps:

demonstrate the flaw and the possibility that funds will indeed be locked in the router.

This is also an invalid assumption because the test will fail if the balance is not 0, since the assertion is for equality.

assertEq(address(leverageZapRouter).balance, 0); Note: to view the provided image, please see the original comment here.

Also, the probability of a low-level stETH call revert due to a pause or deposit limit is very unlikely. The current staking limit is set at 150,000 ETH, which makes it even less likely to happen. Reference this doc.

0xsomeone (judge) commented:

@Slavcheww, thank you for your feedback! The screenshot shared does not properly apply the PoC described as you did not perform the necessary adjustments in the CollateralTokenTester (i.e., stETH ) contract for it to revert. As it is misleading, I strongly advise revising your latest feedback.
