# Benchmark Ground Truth: Abracadabra Mimswap

## Accepted H/M Findings

# Accepted H/M Findings: Abracadabra Mimswap

# [H-01] Anyone making use of the MagicLP’s TWAP to determine token prices will be exploitable.

- **Contest:** Abracadabra Mimswap
- **Slug:** 2024-03-abracadabra-mimswap
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-03-abracadabra-mimswap
- **Source snapshot:** competitions/2024-03-abracadabra-mimswap/final_report.html

Submitted by Trust, also found by ether_sky, blutorque, and ZanyBonzy MagicLP provides a TWAP value which can be accessed via _BASE_PRICE_CUMULATIVE_LAST_ It is updated in the function below:

function _twapUpdate() internal { uint32 blockTimestamp = uint32(block.timestamp % 2 ** 32); uint32 timeElapsed = blockTimestamp - _BLOCK_TIMESTAMP_LAST_; if (timeElapsed > 0 && _BASE_RESERVE_ != 0 && _QUOTE_RESERVE_ != 0) { /// @dev It is desired and expected for this value to /// overflow once it has hit the max of `type.uint256`.

unchecked { _BASE_PRICE_CUMULATIVE_LAST_ += getMidPrice() * timeElapsed; } _BLOCK_TIMESTAMP_LAST_ = blockTimestamp; } It is updated by any function that changes the reserves, for example:

function _resetTargetAndReserve() internal returns (uint256 baseBalance, uint256 quoteBalance) { baseBalance = _BASE_TOKEN_.balanceOf(address(this)); quoteBalance = _QUOTE_TOKEN_.balanceOf(address(this)); if (baseBalance > type(uint112).max || quoteBalance > type(uint112).max) { revert ErrOverflow(); } _BASE_RESERVE_ = uint112(baseBalance); _QUOTE_RESERVE_ = uint112(quoteBalance); _BASE_TARGET_ = uint112(baseBalance); _QUOTE_TARGET_ = uint112(quoteBalance); _RState_ = uint32(PMMPricing.RState.ONE); _twapUpdate(); } function _setReserve(uint256 baseReserve, uint256 quoteReserve) internal { _BASE_RESERVE_ = baseReserve.toUint112(); _QUOTE_RESERVE_ = quoteReserve.toUint112(); _twapUpdate(); } The root cause of the issue is that the TWAP is updated

after reserve changes. Since the TWAP multiplies the duration of time since the last update with the new reserves, an attacker has control over the registered price for the entire passed duration.

For reference, the Uniswap and Beanstalk TWAPs are provided below:

UniswapV2 - priceXCumulativeLast is written and then reserves are changed Beanstalk - pumps are updated before the swap operation.

RareSkills details how TWAP operation works here.

## Impact

Any application making use of the MagicLP’s TWAP to determine token prices will be exploitable.

## Recommended Mitigation Steps

_twapUpdate() needs to be called before reserves are updated.

0xCalibur (Abracadabra) disputed and commented:

It’s as designed. Integrating protocol should always check for min output to avoid frontrunning.

cccz (Judge) commented:

I think this is valid, the problem is that the TWAP algorithm is wrong, TWAP:

- https://en.wikipedia.org/wiki/Time-weighted_average_price
By the way, the code here is consistent with DODOV2.

0xCalibur (Abracadabra) commented:

We decided to removed the TWAP functionality at the end. But during the time of this review, this was in the code.

# [H-02] Attacker can amplify a rounding error in MagicLP to break the I invariant and cause malicious pricing

- **Contest:** Abracadabra Mimswap
- **Slug:** 2024-03-abracadabra-mimswap
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-03-abracadabra-mimswap
- **Source snapshot:** competitions/2024-03-abracadabra-mimswap/final_report.html

Submitted by Trust One of the two key parameters in MagicLP pools is I, which is defined to be the ideal ratio between the two reserves. It is set during MagicLP initialization:

_I_ = i; It is used when performing the initial LP deposit, in buyShares():

if (totalSupply() == 0) { // case 1. initial supply if (quoteBalance == 0) { revert ErrZeroQuoteAmount(); } shares = quoteBalance < DecimalMath.mulFloor(baseBalance, _I_) ? DecimalMath.divFloor(quoteBalance, _I_): baseBalance; _BASE_TARGET_ = shares.toUint112(); _QUOTE_TARGET_ = DecimalMath.mulFloor(shares, _I_).toUint112(); if (_QUOTE_TARGET_ == 0) { revert ErrZeroQuoteTarget(); } if (shares <= 2001) { revert ErrMintAmountNotEnough(); } _mint(address(0), 1001); shares -= 1001; The QUOTE_TARGET is determined by multiplying the BASE_TARGET with I.

The flaw is in the check below:

shares = quoteBalance < DecimalMath.mulFloor(baseBalance, _I_) ? DecimalMath.divFloor(quoteBalance, _I_): baseBalance; Essentially there needs to be enough quoteBalance at the I ratio to mint baseBalance shares, if there’s not enough then shares are instead determined by dividing the quoteBalance with I. An attacker can abuse the mulFloor() to create a major inconsistency. Suppose quoteBalance = 1, baseBalance = 19999, I = 1e14. Then we have:

1 < 19999 * 1e14 / 1e18 => 1 < 1 => False Therefore shares = 19999. It sets the targets:

_BASE_TARGET_ = 19999 _QUOTE_TARGET_ = 19999 * 1e14 / 1e18 = 1 The result is the ratio 1:19999, when the intended ratio from I is 1:1000.

Essentially a small rounding error is magnified. The rounding direction should instead be:

quoteBalance < DecimalMath.mulCeil(baseBalance, _I_) This would ensure that DecimalMath.divFloor(quoteBalance, _I_) is executed. At this point, when calculating QUOTE_TARGET there will not be a precision error as above (it performs the opposing actions to the divFloor).

An attacker can abuse it by making users perform trades under the assumption I is the effective ratio, however the ratio is actually 2I. The pool’s pricing mechanics will be wrong. Note that users will legitimately trust any MagicLP pool created by the Factory as it is supposed to enforce that ratio.

The attack can be performed on another entity’s pool right after the init() call, or on a self-created pool. The initial cost for the attack is very small due to the small numbers involved.

## Impact

Attacker can initialize a Pool with malicious pricing mechanics that break the assumed invariants of the pool, leading to incorrect pool interactions.

## Recommended Mitigation Steps

Use DecimalMaths.mulCeil() to protect against the rounding error.

0xCalibur (Abracadabra) acknowledged, but disagreed with severity and commented:

Based on our previous audit, it was discussed that using mulCeil here would not be the right answer since that would just create imprecision in the ratio in the other direction.

It would be good if the submitter could provide a PoC showing a veritable exploit case for this one.

Acknowledged, but will not fix it on contract level but filtering pool quality and legitimacy on our main frontend.

trust1995 commented:

Hi, The impact demonstrated is doubling the ratio of the pool, which is a core invariant of the MIMswap platform. It means pricing will be incorrect, which is the core functionality of an AMM. A user who will make a trade assuming they will follow the price set out by the I parameter will make incorrect trades, losing their funds inappropriately. I believe the direct risk of loss of funds by innocent traders who are not making a mistake, warrants the severity of High.

cccz (Judge) commented:

The attacker can compromise QUOTE TARGET_ in buyShares() after the pool is created.

function createPool ( address baseToken, address quoteToken, uint256 lpFeeRate, uint256 i, uint256 k, address to, uint256 baseInAmount, uint256 quoteInAmount ) external returns ( address clone, uint256 shares ) { _validateDecimals ( IERC20Metadata ( baseToken ).

decimals (), IERC20Metadata ( quoteToken ).

decimals ()); clone = IFactory ( factory ).

create ( baseToken, quoteToken, lpFeeRate, i, k ); baseToken.

safeTransferFrom ( msg.

sender, clone, baseInAmount ); quoteToken.

safeTransferFrom ( msg.

sender, clone, quoteInAmount ); ( shares,, ) = IMagicLP ( clone ).

buyShares ( to ); <======== } The victim calls sellBase, and the call chain is as follows. In adjustedTarget, state.Q0 will not be adjusted since state.R == RState.ONE.

function sellBase ( address to ) external nonReentrant returns ( uint256 receiveQuoteAmount ) { uint256 baseBalance = _BASE_TOKEN_.

balanceOf ( address ( this )); uint256 baseInput = baseBalance - uint256 ( _BASE_RESERVE_ ); uint256 mtFee; uint256 newBaseTarget; PMMPricing.

RState newRState; ( receiveQuoteAmount, mtFee, newRState, newBaseTarget ) = querySellBase ( tx.

origin, baseInput ); <==============...

function querySellBase ( address trader, uint256 payBaseAmount ) public view returns ( uint256 receiveQuoteAmount, uint256 mtFee, PMMPricing.RState newRState, uint256 newBaseTarget ) { PMMPricing.

PMMState memory state = getPMMState (); <======== ( receiveQuoteAmount, newRState ) = PMMPricing.

sellBaseToken ( state, payBaseAmount ); <======...

function getPMMState () public view returns (PMMPricing.PMMState memory state ) { state.

i = _I_; state.

K = _K_; state.

B = _BASE_RESERVE_; state.

Q = _QUOTE_RESERVE_; state.

B0 = _BASE_TARGET_; // will be calculated in adjustedTarget state.

Q0 = _QUOTE_TARGET_; state.

R = PMMPricing.

RState ( _RState_ ); PMMPricing.

adjustedTarget ( state ); <====== }...

function adjustedTarget ( PMMState memory state ) internal pure { if ( state.

R == RState.

BELOW_ONE ) { state.

Q0 = Math.

_SolveQuadraticFunctionForTarget ( state.

Q, state.

B - state.

B0, state.

i, state.

K ); } else if ( state.

R == RState.

ABOVE_ONE ) { state.

B0 = Math.

_SolveQuadraticFunctionForTarget ( state.

B, state.

Q - state.

Q0, DecimalMath.

reciprocalFloor ( state.

i ), state.

K ); } sellBaseToken calls ROneSellBaseToken, and _ROneSellBaseToken calls _SolveQuadraticFunctionForTrade to use compromised _QUOTE TARGET_ for calculation. It’ll compromise the victim.

function sellBaseToken ( PMMState memory state, uint256 payBaseAmount ) internal pure returns ( uint256 receiveQuoteAmount, RState newR ) { if ( state.

R == RState.

ONE ) { // case 1: R=1 // R falls below one receiveQuoteAmount = _ROneSellBaseToken ( state, payBaseAmount ); <====== newR = RState.

BELOW_ONE;...

function _ROneSellBaseToken ( PMMState memory state, uint256 payBaseAmount ) internal pure returns ( uint256 // receiveQuoteToken ) { // in theory Q2 <= targetQuoteTokenAmount // however when amount is close to 0, precision problems may cause Q2 > targetQuoteTokenAmount return Math.

_SolveQuadraticFunctionForTrade ( state.

Q0, state.

Q0, payBaseAmount, state.

i, state.

K ); <====== } Therefore, High Severity is warranted.

Note: For full discussion, see here.

# [H-03] Users who deposited MIM and USDB tokens into BlastOnboarding may incur losses when the pool is created via bootstrap

- **Contest:** Abracadabra Mimswap
- **Slug:** 2024-03-abracadabra-mimswap
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-03-abracadabra-mimswap
- **Source snapshot:** competitions/2024-03-abracadabra-mimswap/final_report.html

Submitted by ether_sky

- https://github.com/code-423n4/2024-03-abracadabra-money/blob/1f4693fdbf33e9ad28132643e2d6f7635834c6c6/src/blast/BlastOnboarding.sol#L104-L106
- https://github.com/code-423n4/2024-03-abracadabra-money/blob/1f4693fdbf33e9ad28132643e2d6f7635834c6c6/src/blast/BlastOnboardingBoot.sol#L101-L106
- https://github.com/code-423n4/2024-03-abracadabra-money/blob/1f4693fdbf33e9ad28132643e2d6f7635834c6c6/src/mimswap/periphery/Router.sol#L68-L70
- https://github.com/code-423n4/2024-03-abracadabra-money/blob/1f4693fdbf33e9ad28132643e2d6f7635834c6c6/src/mimswap/MagicLP.sol#L381-L383
- https://github.com/code-423n4/2024-03-abracadabra-money/blob/1f4693fdbf33e9ad28132643e2d6f7635834c6c6/src/mimswap/MagicLP.sol#L171
- https://github.com/code-423n4/2024-03-abracadabra-money/blob/1f4693fdbf33e9ad28132643e2d6f7635834c6c6/src/mimswap/libraries/PMMPricing.sol#L194-L199
- https://github.com/code-423n4/2024-03-abracadabra-money/blob/1f4693fdbf33e9ad28132643e2d6f7635834c6c6/src/mimswap/libraries/PMMPricing.sol#L59-L63
Users can deposit MIM and USDB tokens into BlastOnboarding. Once locked, these tokens cannot be unlocked. The locked tokens will be utilized to establish a MagicLP MIM/USDB pool through bootstrap. Although the prices of MIM and USDB tokens are nearly identical, there is no assurance regarding the locked amounts of MIM and USDB tokens. Significant differences between the locked amounts may exist. Depending on this difference, the K value, and the chosen funds by the attacker, substantial funds can be stolen.

The vulnerability stems from the createPool function in the Router. Consequently, any user who creates a MagicLP pool using this function is susceptible to fund loss.

## Recommended Mitigation Steps

We should ensure that the targets and reserves are the same for the first depositor. This can be directly changed in the buyShares function. Alternatively, we can implement small swaps twice in the createPool function. You could verify this in the above test file by setting isForTesting to false.

After performing 2 small swaps, the targets and reserves become as follows:

base reserve ==> 1,000.000009998331296597 base target ==> 1,000.000000000000000000 quote reserve ==> 3,000.000010004999999500 quote target ==> 3,000.000010003331129210 0xCalibur (Abracadabra) acknowledged and commented:

We mitigated the issue by pausable the LP when we are bootstrapping and making sure it’s all good before launching it. So we bootstrap it and balance it out ourself and once it’s in a good state, we enable trading on the pool.

We added notion of protocol own pool and MIM/USDB will be one. The others will be community pools.

See changes here for the fix:

- https://github.com/Abracadabra-money/abracadabra-money-contracts/blob/main/src/mimswap/MagicLP.sol

# [H-04] Oracle price can be manipulated

- **Contest:** Abracadabra Mimswap
- **Slug:** 2024-03-abracadabra-mimswap
- **Finding ID:** H-04
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-03-abracadabra-mimswap
- **Source snapshot:** competitions/2024-03-abracadabra-mimswap/final_report.html

Submitted by SpicyMeatball, also found by Breeje Oracle price can be manipulated.

## Recommended Mitigation Steps

Consider adding a sanity check, where base and quote token prices are compared with the chainlink price feed function latestAnswer() public view override returns (int256) { uint256 baseAnswerNomalized = uint256(baseOracle.latestAnswer()) * (10 ** (WAD - baseOracle.decimals())); uint256 quoteAnswerNormalized = uint256(quoteOracle.latestAnswer()) * (10 ** (WAD - quoteOracle.decimals())); uint256 minAnswer = baseAnswerNomalized < quoteAnswerNormalized ? baseAnswerNomalized: quoteAnswerNormalized; + uint256 midPrice = pair.getMidPrice() * (10 ** (WAD - 6); + uint256 feedPrice = baseAnswerNormalized * WAD / quoteAnswerNormalized; + uint256 difference = midPrice > feedPrice + ? (midPrice - feedPrice) * 10000 / midPrice

+: (feedPrice - midPrice) * 10000 / feedPrice; + // if too big difference - revert + if (difference >= MAX_DIFFERENCE) { + revert PriceDifferenceExceeded(); + } (uint256 baseReserve, uint256 quoteReserve) = _getReserves(); baseReserve = baseReserve * (10 ** (WAD - baseDecimals)); quoteReserve = quoteReserve * (10 ** (WAD - quoteDecimals)); return int256(minAnswer * (baseReserve + quoteReserve) / pair.totalSupply()); } 0xmDreamy (Abracadabra) acknowledged Medium Risk Findings (16)

# [M-01] Pool Creation Failure Due to WETH Transfer Compatibility Issue on Some Chains

- **Contest:** Abracadabra Mimswap
- **Slug:** 2024-03-abracadabra-mimswap
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-abracadabra-mimswap
- **Source snapshot:** competitions/2024-03-abracadabra-mimswap/final_report.html

Submitted by Limbooo In the Router.sol file of the mimswap, there’s a method to create a pool for native tokens by wrapping them to their “wrapped” counterpart before sending them to the newly created pool.

src / mimswap / periphery / Router.

sol:

73:

function createPoolETH ( 74:

address token, 75:

bool useTokenAsQuote, 76:

uint256 lpFeeRate, 77:

uint256 i, 78:

uint256 k, 79:

address to, 80:

uint256 tokenInAmount 81: ) external payable returns ( address clone, uint256 shares ) { 82:

if ( useTokenAsQuote ) { 83:

_validateDecimals ( 18, IERC20Metadata ( token ).

decimals ()); 84: } else { 85:

_validateDecimals ( IERC20Metadata ( token ).

decimals (), 18 ); 86: } 87:

88:

clone = IFactory ( factory ).

create ( useTokenAsQuote ?

address ( weth ):

token, useTokenAsQuote ?

token:

address ( weth ), lpFeeRate, i, k ); 89:

90:

weth.

deposit {value:

msg.

value }(); 91:

token.

safeTransferFrom ( msg.

sender, clone, tokenInAmount ); 92:

address ( weth ).

safeTransferFrom ( address ( this ), clone, msg.

value ); 93: ( shares,, ) = IMagicLP ( clone ).

buyShares ( to ); 94: } However, the transfer done using address(weth).safeTransferFrom (see line 92). This works fine on most chains (Ethereum, Optimism, Polygon, BSC) which uses the standard WETH9 contract that handles the case when src == msg.sender:

WETH9.

sol if ( src != msg.

sender && allowance [ src ][ msg.

sender ] != uint (- 1 )) { require ( allowance [ src ][ msg.

sender ] >= wad ); allowance [ src ][ msg.

sender ] -= wad; } The problem is that the WETH implementation on Blast uses a different contract, and does not have this src == msg.sender handling.

Also, the issue is presented in Wrapped Arbitrum and Wrapped Fantom.

## Impact

The failure to approve the Router contract to spend WETH tokens will prevent the protocol from creating native tokens pools on multiple chains like Blast.

## Recommended Mitigation Steps

To address this issue, it’s recommended to modify the Router.sol file as follows:

src/mimswap/periphery/Router.sol:

-92: address(weth).safeTransferFrom(address(this), clone, msg.value); +92: address(weth).safeTransfer(clone, msg.value); 0xCalibur (Abracadabra) acknowledged, and commented:

Nice catch.

Fix is here:

- https://github.com/Abracadabra-money/abracadabra-money-contracts/pull/150

# [M-02] Tokens yeild can not be set to claimable.

- **Contest:** Abracadabra Mimswap
- **Slug:** 2024-03-abracadabra-mimswap
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-abracadabra-mimswap
- **Source snapshot:** competitions/2024-03-abracadabra-mimswap/final_report.html

Submitted by hassanshakeel13 The enabling of yeild on rebasing tokens can be permanently DOS-ed.

## Recommended Mitigation Steps

Use the correct signature used in blast codebase.

0xCalibur (Abracadabra) confirmed and commented:

This is a valid report. We already fixed it before the audit ended.

The fix is here in IBlast.sol:

- https://github.com/Abracadabra-money/abracadabra-money-contracts/commit/12f07da03c0adaff123d7e6c684b757855521d61#diff-8f7d8246c1e6d7928012209792b0b3f0a9684a94a40bd802e2b22e5032db04bc
Also fixed in the BlastMock.sol file.

cccz (Judge) decreased severity to Medium and commented:

Downgrade to M because the problematic function will be called when deployed. An attacker can just exploit it to prevent the contract from being deployed. And if the attacker wants to exploit it after the contract is deployed, since the mode is fixed to YieldMode.CLAIMABLE, it will just return even if it is called again by the Owner.

function enableTokenClaimable ( address token ) internal { if ( IERC20Rebasing ( token ).

getConfiguration ( address ( this )) == YieldMode.

CLAIMABLE ) { return; } This is the deployment script for blastOnboarding.sol, which shows that setTokenSupported will be called immediately after deployment.

contract BlastOnboardingScript is BaseScript { function deploy () public returns ( BlastOnboarding onboarding ) { address owner = toolkit.

getAddress ( block.

chainid, "safe.ops" ); address feeTo = toolkit.

getAddress ( block.

chainid, "safe.ops" ); address blastGovernor = toolkit.

getAddress ( block.

chainid, "blastGovernor" ); address blastTokenRegistry = toolkit.

getAddress ( block.

chainid, "blastTokenRegistry" ); vm.

startBroadcast (); onboarding = BlastOnboarding ( payable ( deploy ( "Onboarding", "BlastOnboarding.sol:BlastOnboarding", abi.

encode ( blastTokenRegistry, feeTo, tx.

origin ))) ); if (!

testing ()) { address usdb = toolkit.

getAddress ( block.

chainid, "usdb" ); address mim = toolkit.

getAddress ( block.

chainid, "mim" ); if (!

onboarding.

supportedTokens ( usdb )) { onboarding.

setTokenSupported ( usdb, true ); } if (!

onboarding.

supportedTokens ( mim )) { onboarding.

setTokenSupported ( mim, true ); } if ( onboarding.

owner () != owner ) { onboarding.

transferOwnership ( owner ); } vm.

stopBroadcast (); } Also, even without the deployment script, I think this is a DOS of M severity and there is no loss of funds because the user cannot deposit tokens until setTokenSupported is called.

Note: For full discussion, see here.

# [M-03] Miscalculation in addLiquidity of Router results in unauthorized spending of tokens

- **Contest:** Abracadabra Mimswap
- **Slug:** 2024-03-abracadabra-mimswap
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-abracadabra-mimswap
- **Source snapshot:** competitions/2024-03-abracadabra-mimswap/final_report.html

Submitted by Trust, also found by ether_sky Description Users are expected to add liquidity through addLiquidity() of the Router. It receives base and quote amounts which are adjusted through _adjustAddLiquidity():

function addLiquidity( address lp, address to, uint256 baseInAmount, uint256 quoteInAmount, uint256 minimumShares, uint256 deadline ) external ensureDeadline(deadline) returns (uint256 baseAdjustedInAmount, uint256 quoteAdjustedInAmount, uint256 shares) { (baseAdjustedInAmount, quoteAdjustedInAmount) = _adjustAddLiquidity(lp, baseInAmount, quoteInAmount); IMagicLP(lp)._BASE_TOKEN_().safeTransferFrom(msg.sender, lp, baseAdjustedInAmount); IMagicLP(lp)._QUOTE_TOKEN_().safeTransferFrom(msg.sender, lp, quoteAdjustedInAmount); shares = _addLiquidity(lp, to, minimumShares); } The adjustment function has DODOv2 code which determines the effective base:quote ratio and adjusts the higher of the two input values so that no input is wasted. Additionally Abrakadbra inserted the following logic at the start of the function:

(uint256 baseReserve, uint256 quoteReserve) = IMagicLP(lp).getReserves(); uint256 baseBalance = IMagicLP(lp)._BASE_TOKEN_().balanceOf(address(lp)) + baseInAmount; uint256 quoteBalance = IMagicLP(lp)._QUOTE_TOKEN_().balanceOf(address(lp)) + quoteInAmount; baseInAmount = baseBalance - baseReserve; quoteInAmount = quoteBalance - quoteReserve; The intention is that if the current balance of token X is higher than reserve, the input should be reduced so that only the delta needs to be sent. However the logic is miswritten. The balanceOf() and getReserve() calls have been reversed. The correct logic should be:

requested amount = reserves + requested amount - balance Instead it is:

requested amount = balance + requested amount - reserves. The higher the current balanceOf(), the higher the final requested amount becomes. This is a critical issue because tokens can be donated to the MagicLP by an attacker, making the victim send more tokens than expected. The POC gives an example of such a sequence.

Essentially this makes addLiquidity(amountX, amountY, minShares) become: (amountX2, amountY2, minShares) where amountX2 > amountX, amountY2 > amountY. By definition this is unauthorized use of the victim’s funds.

## Impact

Unauthorized spending of victim’s tokens, leading to monetary losses.

## Recommended Mitigation Steps

Change the usage of getReserves() and balanceOf() in the lines below as described:

(uint256 baseReserve, uint256 quoteReserve) = IMagicLP(lp).getReserves(); uint256 baseBalance = IMagicLP(lp)._BASE_TOKEN_().balanceOf(address(lp)) + baseInAmount; uint256 quoteBalance = IMagicLP(lp)._QUOTE_TOKEN_().balanceOf(address(lp)) + quoteInAmount; A great extra invariant check would be to require in addLiquidity() that the results baseAdjustedInAmount, quoteAdjustedInAmount are smaller than the original input amounts respectively.

Note that the issue also occurs in the preview function:

function previewAddLiquidity( address lp, uint256 baseInAmount, uint256 quoteInAmount ) external view returns (uint256 baseAdjustedInAmount, uint256 quoteAdjustedInAmount, uint256 shares) { (uint256 baseReserve, uint256 quoteReserve) = IMagicLP(lp).getReserves(); uint256 baseBalance = IMagicLP(lp)._BASE_TOKEN_().balanceOf(address(lp)) + baseInAmount; uint256 quoteBalance = IMagicLP(lp)._QUOTE_TOKEN_().balanceOf(address(lp)) + quoteInAmount; baseInAmount = baseBalance - baseReserve; quoteInAmount = quoteBalance - quoteReserve; It should also be fixed here. We view it as the same root cause, but of a lesser end impact.

0xCalibur (Abracadabra) confirmed and commented:

Fix here:

- https://github.com/Abracadabra-money/abracadabra-money-contracts/pull/141
cccz (Judge) decreased severity to Medium and commented:

This is more of a griefing attack, and it seems the victim’s 200 share would be worth 2000 USDC, 1666 DAI?

Trust (Warden) commented:

Unauthorized spending of tokens is breaking of a core invariant (do not use money user did not intend to spend), which by itself achieves High impact. The submission also explains the risks affecting the unapproved funds.

Note that sponsor has confirmed the issue at High severity and views it as such.

cccz (Judge) commented:

Hey @trust1995, in POC, if I understand correctly, while the victim spends more tokens, the shares the victim receives will be worth more due to the donations of the griefer (3000 -> 3666), and there doesn’t seem to be any locks here, so I don’t think this as a higher impact than DOS.

Trust (Warden) commented:

I can confirm that the user spends more tokens and receives more shares. In my opinion that suffices for H severity as it is not blocking a victim interaction, it is taking unapproved funds (until the user figures out how to redeem them back). Again, touching user’s assets without their permission is very severe.

cccz (Judge) commented:

In terms of this attack scenario The attacker needs to donate funds the victim spends more funds than expected The victim receives a portion of the attacker’s donated funds the user who added the liquidity should know how to get them back victims can get more money back immediately So I don’t think this is high severity.

# [M-04] Loss of assumed functionality of the Onboarding contract in a highly-sensitive area

- **Contest:** Abracadabra Mimswap
- **Slug:** 2024-03-abracadabra-mimswap
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-abracadabra-mimswap
- **Source snapshot:** competitions/2024-03-abracadabra-mimswap/final_report.html

Submitted by Trust, also found by hals and SpicyMeatball The BlastOnboardingBoot contract contains the bootstraping funds. After launch it will start a reward staking pool and allow users to claim tokens into it. This is in the launch sequence:

staking = new LockingMultiRewards(pool, 30_000, 7 days, 13 weeks, address(this)); staking.setOperator(address(this), true); staking.transferOwnership(owner); // Approve staking contract pool.safeApprove(address(staking), totalPoolShares); Note that the code approves the staking pool towithdraw from the created LP. The code also provisions setting of the staking contract to another one.

// Just in case we need to change the staking contract after // the automatic bootstrapping process function setStaking(LockingMultiRewards _staking) external onlyOwner { if (ready) { revert ErrCannotChangeOnceReady(); } staking = _staking; emit LogStakingChanged(address(_staking)); } The issue is that this function lacks two actions:

Resetting the previous staking pool’s approval to zero More importantly, approving the new staking pool all the bootstrap pool shares As a result, owner which assumes changing to a new staking pool is perfectly safe, will in fact cause reverts when users try claiming in the new pool. It is still possible to revert back to the old staking pool, but in fact it will never be possible to migrate to a new pool as no other approve() call exists in the contract.

## Impact

Loss of assumed functionality of the Onboarding contract in a highly-sensitive area.

## Recommended Mitigation Steps

Approve the new contract Revoke approval from the previous contract for good sanitization 141345 (Lookout) commented:

Change staking contract, miss approve update and reset.

0xCalibur (Abracadabra) acknowledged and commented:

we can also upgrade the bootstrapper if we were to use the function. I agree with the finding but we just decided to remove the function setStaking.

# [M-05] A user’s tokens could be locked for an extended duration beyond their intention and without their control

- **Contest:** Abracadabra Mimswap
- **Slug:** 2024-03-abracadabra-mimswap
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-abracadabra-mimswap
- **Source snapshot:** competitions/2024-03-abracadabra-mimswap/final_report.html

Submitted by Trust Description The LockingMultiRewards allows anyone to stake MagicLP tokens in return for rewards. LP tokens are sent with either functions below:

function stake(uint256 amount, bool lock_) public whenNotPaused { _stakeFor(msg.sender, amount, lock_); } /// @notice Locks an existing unlocked balance.

function lock(uint256 amount) public whenNotPaused { if (amount == 0) { revert ErrZeroAmount(); } _updateRewardsForUser(msg.sender); _balances[msg.sender].unlocked -= amount; unlockedSupply -= amount; _createLock(msg.sender, amount); } This will lead to _createLock(), which calculates the nextUnlockTime():

function _createLock(address user, uint256 amount) internal { Balances storage bal = _balances[user]; uint256 _nextUnlockTime = nextUnlockTime(); function nextUnlockTime() public view returns (uint256) { return nextEpoch() + lockDuration; } Note that nextEpoch() would always return the next 1-week (or reward duration) slot.

function epoch() public view returns (uint256) { return (block.timestamp / rewardsDuration) * rewardsDuration; } function nextEpoch() public view returns (uint256) { return epoch() + rewardsDuration; } An issue arises because the user cannot specify the latest desired unlock time. This opens the path for the tokens to be locked for longer than expected, which may have significant impact for users if they need the funds. Consider the following case, where x is week number:

It is day 7x + 6 (one day from nextEpoch) Assumed unlock time is 7x + 7 + lockDuration The TX does not execute in next 1 day for any reason (gas price went up, validator does not include TX, etc) It is now day 7x+7, another epoch starts. Now nextEpoch is 7x + 14 Executed unlock time is 7x + 14 + lockDuration This means user’s funds are locked for an additional 7 days more than expected.

Note that delayed execution of a user’s TX has always been considered in scope, certainly for Med severity impacts: -

- https://solodit.xyz/issues/m-13-interactions-with-amms-do-not-use-deadlines-for-operations-code4rena-paraspace-paraspace-contest-git
-

- https://solodit.xyz/issues/m-04-lack-of-deadline-for-uniswap-amm-code4rena-asymmetry-finance-asymmetry-contest-git
-

- https://solodit.xyz/issues/m-03-missing-deadline-param-in-swapexactamountout-allowing-outdated-slippage-and-allow-pending-transaction-to-be-executed-unexpectedly-code4rena-pooltogether-pooltogether-git
Essentially the locking function lacks a deadline parameter similar to swapping functions, and the impact is temporary freeze of funds.

## Impact

A user’s tokens could be locked for an extended duration beyond their intention and without their control.

## Recommended Mitigation Steps

Consider adding a latestUnlockTime deadline parameter for the locking functions.

0xCalibur (Abracadabra) acknowledged, but disagreed with severity and commented:

- https://github.com/Abracadabra-money/abracadabra-money-contracts/pull/138
Should be low.

cccz (Judge) commented:

Here the latest discussion on transaction delays.

- https://github.com/code-423n4/2024-02-uniswap-foundation-findings/issues/331#issuecomment-2021292618
And I would consider it QA which is due to low likelihood and medium impact (temporary freeze of funds).

Trust (Warden) commented:

Hi, I would like to argue that likelihood is medium and impact is high. likelihood - Assuming there will be many stakers on the platform, at any moment there could be a request close to the end of epoch. The closer the request is to the end of an epoch, the more likely it is a TX will not be executed until the next one. Additionally, the users of the stake() function are completely different in understanding/sophistication from the ones in the Unistaker audit. They cannot be assumed to understand TX delays, requirement of invalidating one’s TX if it is undesirable, etc. Additionally one needs to take into consideration that gas prices are fluctuating, so censorship of a TX is absolutely not required for the FoF to occur. It just needs to be on a local low-point for the duration until the next interval for the impact to be achieved. Again, consider that this is

not a targetted attack, the issue is a constant probabilistic leak that given enough time will materialize.

For impact, temporary FoF is a very serious issue, on the low end of the High impact (imo). Consider for example that a user assumes their tokens will be available at the unlock period in order to pay off a loan, and because of the FoF they will now default. This is just one example of hundred of different very serious scenarios that would occur to a user that assumes funds will be available in the future.

Considering all the arguments above, I believe the finding to be between M and H severity, rather than between L and M severity.

cccz (Judge) commented:

Agreed, the lock duration is jumping, which may compromise user.

# [M-06] MagicLpAggregator always returns lower than correct answer, leading to arbitrage loss

- **Contest:** Abracadabra Mimswap
- **Slug:** 2024-03-abracadabra-mimswap
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-abracadabra-mimswap
- **Source snapshot:** competitions/2024-03-abracadabra-mimswap/final_report.html

Submitted by Trust MagicLpAggregator is used to price LP tokens for “closely-tied” underlying tokens. It calculates the price below:

function latestAnswer() public view override returns (int256) { uint256 baseAnswerNomalized = uint256(baseOracle.latestAnswer()) * (10 ** (WAD - baseOracle.decimals())); uint256 quoteAnswerNormalized = uint256(quoteOracle.latestAnswer()) * (10 ** (WAD - quoteOracle.decimals())); uint256 minAnswer = baseAnswerNomalized < quoteAnswerNormalized ? baseAnswerNomalized: quoteAnswerNormalized; (uint256 baseReserve, uint256 quoteReserve) = _getReserves(); baseReserve = baseReserve * (10 ** (WAD - baseDecimals)); quoteReserve = quoteReserve * (10 ** (WAD - quoteDecimals)); return int256(minAnswer * (baseReserve + quoteReserve) / pair.totalSupply()); } The code takes the minimal answer between the underlying oracles and considers all reserves to be worth that amount:

return int256(minAnswer * (baseReserve + quoteReserve) / pair.totalSupply()); The issue is that any difference in price between the assets represents an easy arbitrage opportunity. Suppose we have tokens (A,B), where real oracle shows:

A = $0.99 B = $1 The Pool has 1000000 LP tokens and contains:

1000000 A 1000000 B The LP value would calculate as:

0.99 * 2000000 / 1000000 = $1.98 The actual value is:

(0.99 * 1000000 + 1 * 1000000) / 1000000 = $1.99 Suppose a platform trades LPs using the aggregator pricing. An attacker could:

Buy 100,000 LP tokens at $198000 Withdraw from the pool the underlying shares Sell 100,000 A, 100,000 B at $199000 Profit $1000 from the exchange, when the difference is just$0.01 (this is very common fluctuation even with pegged assets).

The delta comes at the expense of LP holders whose position gets minimized.

## Impact

Loss of value due to arbitrage of any platform using MagicLpAggregator pricing.

## Recommended Mitigation Steps

Always calculate the value based on the real underlying token value multiplied by amount.

Consider creating two separate oracles for lower-bound and upper-bound results. Then a lending protocol could indeed use the lower-bound for determining collateral value.

0xm3rlin (Abracadabra) disputed and commented:

Intended behavior.

141345 (Lookout) commented:

Rounding error could accumulate in MagicLpAggregator.

# [M-07] Permanent loss of yield for stakers in reward pools due to precision loss.

- **Contest:** Abracadabra Mimswap
- **Slug:** 2024-03-abracadabra-mimswap
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-abracadabra-mimswap
- **Source snapshot:** competitions/2024-03-abracadabra-mimswap/final_report.html

Submitted by Trust, also found by AgileJune, Bigsam, and grearlake The LockingMultiRewards contract facilitates distribution of rewards across the remaining epoch duration. The calculation for reward rate is provided below:

reward.rewardRate = amount / _remainingRewardTime; The rate is later multiplied by the duration to get the total rewards for elapsed time, demonstrated below:

function rewardsForDuration(address rewardToken) external view returns (uint256) { return _rewardData[rewardToken].rewardRate * rewardsDuration; } An issue occurs because there is no sufficient wrapping of the amount before dividing by _remainingRewardTime. The number is divided and later multiplied by elapsed time, causing a loss of precision of the amount modulo remaining time. For the provided period of 1 week by the sponsor, the maximum amount lost can be calculated:

7 * 24 * 3600 - 1 = 604799. Note that the average case loss is half of the worst case assuming even distribution across time. However since rewards are usually not sent at the low end of remaining time (see the minRemainingTime variable, the actual average would be higher).

The effect of this size of loss depends on the decimals and value of the reward token. For USDC, this would be $0.6. For WBTC, it would be$423 (at time of writing). The loss is shared between all stakers relative to their stake amount. The loss occurs for every notification, so it is clear losses will be severe.

Sponsor remarked in the channel that reward tokens would be real, popular tokens. We believe USDC / WBTC on ARB are extremely popular tokens and therefore remain fully in scope as reward tokens.

Severity Rationalization Impact - high Likelihood - medium -> Severity - high

## Impact

Permanent loss of yield for stakers in reward pools due to precision loss.

## Recommended Mitigation Steps

Store the rewardRate scaled by 1e18, so loss of precision will be lower by magnitude of 1e18.

0xCalibur (Abracadabra) disputed and commented via duplicate #166:

No factor.

Trust (Warden) commented:

Hi, The duplicate has been closed due to loss of “dust amounts”. However I show in my submission that it is far from dust, I believe any submissions that have identified material loss of funds should be awarded appropriately.

# [M-08] Factory::create() is vulnerable to reorg attacks

- **Contest:** Abracadabra Mimswap
- **Slug:** 2024-03-abracadabra-mimswap
- **Finding ID:** M-08
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-abracadabra-mimswap
- **Source snapshot:** competitions/2024-03-abracadabra-mimswap/final_report.html

Submitted by Bauchibred Take a look at

- https://github.com/code-423n4/2024-03-abracadabra-money/blob/1f4693fdbf33e9ad28132643e2d6f7635834c6c6/src/mimswap/periphery/Factory.sol#L81-L90
function create ( address baseToken_, address quoteToken_, uint256 lpFeeRate_, uint256 i_, uint256 k_ ) external returns ( address clone ) { address creator = tx.

origin; bytes32 salt = _computeSalt ( creator, baseToken_, quoteToken_, lpFeeRate_, i_, k_ ); clone = LibClone.

cloneDeterministic ( address ( implementation ), salt ); IMagicLP ( clone ).

init ( address ( baseToken_ ), address ( quoteToken_ ), lpFeeRate_, address ( maintainerFeeRateModel ), i_, k_ ); emit LogCreated ( clone, baseToken_, quoteToken_, creator, lpFeeRate_, maintainerFeeRateModel, i_, k_ ); _addPool ( creator, baseToken_, quoteToken_, clone ); } We can see that this function uses the create() logic and depends on the tx.origin, now Blast is suspicious of a reorg attack and protocol would be deployed on here.

Where as one can assume that the tx.origin should be different for different calls this is not really the case as going to the Blast Explorer for transactions that are currently enqueued we can see that only two addresses are rampant as the tx.origin, i.e 0x4b16E5d33D7ab3864d53aAec93c8301C1FA4a226 and 0x6e5572f31bd9385709ec61305Afc749F0fa8fae1 what this leads is the fact that another user can just wait and due to a re-org take control of the magic lp deployment, since the tx.origin in this case would be the same with original deployer’s own.

## Impact

Now, if users rely on the address derivation in advance or try to deploy the magiclp with the same address, any funds/tokens sent to it could potentially be withdrawn by anyone else. All in all, it could lead to the theft of user funds.

## Recommended Mitigation Steps

Try the deployment using create2 with salt that includes real msg.sender.

0xCalibur (Abracadabra) acknowledged and commented:

This was fixed during the audit. We now use the creator address as part of the salt.

See here:

- https://github.com/Abracadabra-money/abracadabra-money-contracts/blob/main/src/mimswap/periphery/Factory.sol
cccz (Judge) commented:

- https://docs.code4rena.com/roles/judges/how-to-judge-a-contest#notes-on-judging
Unless there is something uniquely novel created by combining vectors, most submissions regarding vulnerabilities that are inherent to a particular system or the Ethereum network as a whole should be considered QA. Examples of such vulnerabilities include front running, sandwich attacks, and MEV. In such events, leave a comment on the issue:

As per the c4 docs, will downgrade to QA.

Bauchibred (Warden) commented:

Hi @cccz (Judge), thanks for judging, in regards to this:

- https://docs.code4rena.com/roles/judges/how-to-judge-a-contest#notes-on-judging
Unless there is something uniquely novel created by combining vectors, most submissions regarding vulnerabilities that are inherent to a particular system or the Ethereum network as a whole should be considered QA. Examples of such vulnerabilities include front running, sandwich attacks, and MEV. In such events, leave a comment on the issue:

As per the c4 docs, will downgrade to QA.

The idea of using this rule as the grounds for downgrading this issue seems to be flawed, cause considering this rule, do we now say all front running, MEV bug ideas are invalid on Code4rena? We beg to differ as context really matters for bug cases like this.

In fact a similar discussion around this bug case (very similar instance) was held a short while ago, can be seen here, and the deciding lines of the validity of the report not being a medium severity was the fact that in that instance the protocol was to be deployed on Canto, and Canto is a fork of Evmos/Ethermint, so it uses Tendermint Core BFT consensus, which provides a 1-block finality and not probabilisitic finality like other chains:

- https://docs.ethermint.zone/core/pending_state.html
But that’s not the case we have here, in this case protocol is to be deployed on Blast Blast is an optimistic rollup, just like Arbritrum, Base, Optimism, etc.

Optimistic rollups are known for having re-org issues.

Not all current satisfactory H/M issues have been fixed by the protocol, but this was not only confirmed by the protocol but also addressed in this commit, proving that this issue is of great value to them and accepted by them to be worthy a fix.

Considering all the above arguments, we’d request a reassessment of this finding being validated as medium severity, thank you.

cccz (Judge) commented:

These facts raise the likelihood of this issue, will reconsider it as M:

Blast is an optimistic rollup, just like Arbritrum, Base, Optimism, etc.

Optimistic rollups are known for having re-org issues.

# [M-09] Adjusting ” I ” will create a sandwich opportunity because of price changes

- **Contest:** Abracadabra Mimswap
- **Slug:** 2024-03-abracadabra-mimswap
- **Finding ID:** M-09
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-abracadabra-mimswap
- **Source snapshot:** competitions/2024-03-abracadabra-mimswap/final_report.html

I ” will create a sandwich opportunity because of price changes Submitted by grearlake, also found by roguereggiant, Breeje, hals, and blutorque

- https://github.com/code-423n4/2024-03-abracadabra-money/blob/main/src/mimswap/MagicLP.sol#L470-#L510
- https://github.com/code-423n4/2024-03-abracadabra-money/blob/main/src/mimswap/MagicLP.sol#L244-#L265
Because MagicLP is forked from DODO, we will check DODO documentation to understand this contract. From DODO documentation, the ” I ” is the “i” value in here and it is directly related with the output amount a trader will receive when selling a quote/base token:

Adjusting the value of ” I ” directly by calling setParameters() function will influence the price. This can be exploited by a MEV bot, simply by trading just before the setParameters() function and exiting right after the price change. The profit gained from this operation essentially represents potential losses for the liquidity providers who supplied liquidity to the pool.

## Impact

Since the price will change, the MEV bot can simply sandwich the tx and get profit.

Another note on this is that even though the adjustPrice called by onlyImplementationOwner without getting frontrunned, it still creates a big price difference which requires immediate arbitrages. Usually these type of parameter changes that impacts the trades are setted by time via ramping to mitigate the unfair advantages that it can occur during the price update.

## Recommended Mitigation Steps

Acknowledge the issue and use private RPC’s to eliminate front-running or slowly ramp up the ” I ” so that the arbitrage opportunity is fair 0xCalibur (Abracadabra) acknowledged and commented:

Fixed in main:

- https://github.com/Abracadabra-money/abracadabra-money-contracts/
We now have protocol own pool and can disable trading for our own pools, changing the parameters and make sure the pool is in good state before enabling it back.

# [M-10] Missing Return Statement in _getReserves Function in MagicLpAggregator Contract

- **Contest:** Abracadabra Mimswap
- **Slug:** 2024-03-abracadabra-mimswap
- **Finding ID:** M-10
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-abracadabra-mimswap
- **Source snapshot:** competitions/2024-03-abracadabra-mimswap/final_report.html

_getReserves Function in MagicLpAggregator Contract Submitted by 0xAadi, also found by bareli, 0x11singh99, and hihen The MagicLpAggregator contract contains a flaw within the _getReserves function where it fails to return the reserve values fetched from the pair contract. This oversight results in the latestAnswer function always returning zero, which can have severe implications for any systems that depend on this contract for accurate liquidity pool pricing data.

## Impact

The missing return statement in the _getReserves function leads to the latestAnswer function always returning zero. This affects any dependent systems or contracts that rely on accurate price data from the MagicLpAggregator contract, as they will receive incorrect information, potentially leading to financial loss or system failure.

## Recommended Mitigation Steps

Update _getReserves() - function _getReserves() internal view virtual returns (uint256, uint256) { + function _getReserves() internal view virtual returns (uint256 baseReserve, uint256 quoteReserve) { - (uint256 baseReserve, uint256 quoteReserve) = pair.getReserves(); + (baseReserve, quoteReserve) = pair.getReserves(); } 0xCalibur (Abracadabra) confirmed and commented:

Fixed in main during the audit:

- https://github.com/Abracadabra-money/abracadabra-money-contracts
Note: For full discussion, see here.

# [M-11] MagicLpAggregator can be incompatible with potential integrators due to incorrect latestRoundData function

- **Contest:** Abracadabra Mimswap
- **Slug:** 2024-03-abracadabra-mimswap
- **Finding ID:** M-11
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-abracadabra-mimswap
- **Source snapshot:** competitions/2024-03-abracadabra-mimswap/final_report.html

MagicLpAggregator can be incompatible with potential integrators due to incorrect latestRoundData function Submitted by DarkTower MagicLpAggregator does not update the roundId, startAt, updatedAt and answeredInRound to correct values.

MagicLpAggregator.sol#L48-L50 function latestRoundData () external view returns ( uint80, int256, uint256, uint256, uint80 ) { return ( 0, latestAnswer (), 0, 0, 0 ); } A common code is to check updatedAt for staleness issue (although it isn’t required to do so anymore.

(, int256 price,, uint256 updatedAt, ) = priceFeed.

latestRoundData (); if ( updatedAt < block.

timestamp - 60 * 60 /* 1 hour */ ) { revert ( "stale price feed" ); } Therefore any integrator that uses the above code will not be able to integrate MagicLpAggregator oracles as it will always revert due to the incorrect updatedAt being provided.

## Recommended Mitigation Steps

Use the values roundId, startAt, updatedAt and answeredInRound from whichever oracle, baseOracle or quoteOracle was used.

rexjoseph (Warden) commented:

I think we should reiterate the issue here as the submission has tried its best to point out:

The MagicLpAggregator gets the price from Chainlink’s Feed Integrators (for example protocol A) query the MagicLPAggregator Oracle for price specifically the latestRoundData() function it exposes in their implementation They make sure the prices returned are fresh and so do well to check the time specifically updatedAt so they can be sure the feed is fresh to proceed with utilizing the returned data Since the updatedAt as well as other returned data are hardcoded to 0 in MagicLpAggregator oracle implementation, the call reverts. 0 will always be less than block.timestamp - 60 * 60 The step by step description of the issue above I believe is sufficient

# [M-12] MagicLpAggregator doesn’t consider the dcimal of MagicLP

- **Contest:** Abracadabra Mimswap
- **Slug:** 2024-03-abracadabra-mimswap
- **Finding ID:** M-12
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-abracadabra-mimswap
- **Source snapshot:** competitions/2024-03-abracadabra-mimswap/final_report.html

Submitted by ether_sky, also found by hals, DarkTower, and SpicyMeatball

- https://github.com/code-423n4/2024-03-abracadabra-money/blob/1f4693fdbf33e9ad28132643e2d6f7635834c6c6/src/mimswap/MagicLP.sol#L163-L165
- https://github.com/code-423n4/2024-03-abracadabra-money/blob/1f4693fdbf33e9ad28132643e2d6f7635834c6c6/src/oracles/aggregators/MagicLpAggregator.sol#L37-L46
We can know that the MagicLpAggregator can serve as the price source for MagicLP shares when the prices of base and quote tokens are similar.

MagicLpAggregator would be used to price MagicLP collateral for Cauldrons.

Something to note is that the MagicLP Oracle is only meant for closed-together price pool.

It's just that the oracle is not meant to be used for any kind of MagicLP, just for closely priced tokens like MIM/USDB.

There’s no assurance that we can use MagicLpAggregator for base and quote tokens with only 18 decimals. The MagicLP token has the same decimal as the base token. However, the MagicLpAggregator does not account for this decimal, resulting in incorrect prices when the decimal of the base token is not 18.

## Recommended Mitigation Steps

function latestAnswer() public view override returns (int256) { uint256 baseAnswerNomalized = uint256(baseOracle.latestAnswer()) * (10 ** (WAD - baseOracle.decimals())); uint256 quoteAnswerNormalized = uint256(quoteOracle.latestAnswer()) * (10 ** (WAD - quoteOracle.decimals())); uint256 minAnswer = baseAnswerNomalized < quoteAnswerNormalized ? baseAnswerNomalized: quoteAnswerNormalized; (uint256 baseReserve, uint256 quoteReserve) = _getReserves(); baseReserve = baseReserve * (10 ** (WAD - baseDecimals)); quoteReserve = quoteReserve * (10 ** (WAD - quoteDecimals)); + uint256 totalSupply = pair.totalSupply() * (10 ** (WAD - baseDecimals)); - return int256(minAnswer * (baseReserve + quoteReserve) /

pair.totalSupply()); + return int256(minAnswer * (baseReserve + quoteReserve) / totalSupply); } 0xCalibur (Abracadabra) disputed

# [M-13] Less base tokens are transferred when selling quote tokens due to the precision loss that occurred in _GeneralIntegrate()

- **Contest:** Abracadabra Mimswap
- **Slug:** 2024-03-abracadabra-mimswap
- **Finding ID:** M-13
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-abracadabra-mimswap
- **Source snapshot:** competitions/2024-03-abracadabra-mimswap/final_report.html

_GeneralIntegrate() Submitted by Matin

- https://github.com/code-423n4/2024-03-abracadabra-money/blob/main/src/mimswap/libraries/Math.sol#L51-L65
- https://github.com/code-423n4/2024-03-abracadabra-money/blob/main/src/mimswap/MagicLP.sol#L180-L191
- https://github.com/code-423n4/2024-03-abracadabra-money/blob/main/src/mimswap/libraries/PMMPricing.sol#L76-L100
Solidity rounds down the result of an integer division, and because of that, it is always recommended to multiply before dividing to avoid that precision loss. In the case of a prior division over multiplication, the final result may face serious precision loss as the first answer would face truncated precision and then multiplied to another integer.

The problem lies in the magicLP’s quote token querying part. The function querySellQuote() is responsible for querying the price of the quote token. This function calls the sellQuoteToken() function of the contract PMMPricing which uses a curve integral at its heart. To be precise, for the cases where the parameter R is not equal to 1, the function sellQuoteToken() calls the two functions _RAboveSellBaseToken(), _RBelowSellQuoteToken() for the cases R > 1 and R < 1 respectively. These two functions calculate the base token amounts using the _GeneralIntegrate() function.

If we look deeply at the function _GeneralIntegrate() we can see the numerical integration procedure is presented as:

/* Integrate dodo curve from V1 to V2 require V0>=V1>=V2>0 res = (1-k)i(V1-V2)+ikV0*V0(1/V2-1/V1) let V1-V2=delta res = i*delta*(1-k+k(V0^2/V1/V2)) i is the price of V-res trading pair support k=1 & k=0 case [round down] */ function _GeneralIntegrate ( uint256 V0, uint256 V1, uint256 V2, uint256 i, uint256 k ) internal pure returns ( uint256 ) { if ( V0 == 0 ) { revert ErrIsZero (); } uint256 fairAmount = i * ( V1 - V2 ); // i*delta if ( k == 0 ) { return fairAmount / DecimalMath.

ONE; } uint256 V0V0V1V2 = DecimalMath.

divFloor (( V0 * V0 ) / V1, V2 ); uint256 penalty = DecimalMath.

mulFloor ( k, V0V0V1V2 ); // k(V0^2/V1/V2) return ((( DecimalMath.

ONE - k ) + penalty ) * fairAmount ) / DecimalMath.

ONE2; } we can see there is a hidden division before a multiplication that makes round down the whole expression. The parameter V0V0V1V2 is calculated in such a way that the V0 * V0 is divided by V1, then the answer is divided by V2. After these divisions, the penalty variable is defined by the multiplication of V0V0V1V2 by the k variable. This writing method is bad as the precision loss can be significant, leading to the magic pool selling fewer base tokens than actual.

At the Proof of Concept part, we can check this behavior precisely.

## Recommended Mitigation Steps

Consider modifying the numerical integral calculation to prevent such precision loss and prioritize multiplication over division:

function _GeneralIntegrate ( uint256 V0, uint256 V1, uint256 V2, uint256 i, uint256 k ) public pure returns ( uint256 ) { if ( V0 == 0 ) { revert ErrIsZero (); } uint256 fairAmount = i * ( V1 - V2 ); // i*delta if ( k == 0 ) { return fairAmount / 1e18; } uint256 V0V0V1V2 = ( V0 * V0 ); uint256 penalty = ( k * V0V0V1V2 ) / ( V1 * V2 ); // k(V0^2/V1/V2) return ((( 1e18 - k ) + penalty ) * fairAmount ) / 1e36; } 0xCalibur (Abracadabra) acknowledged

# [M-14] Staking contract is not able to support native USDB/WETH

- **Contest:** Abracadabra Mimswap
- **Slug:** 2024-03-abracadabra-mimswap
- **Finding ID:** M-14
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-abracadabra-mimswap
- **Source snapshot:** competitions/2024-03-abracadabra-mimswap/final_report.html

Submitted by yixxas Loss of yield if USDB/WETH is used as reward token.

## Recommended Mitigation Steps

Add the ability to set native RebasingERC20 token to CLAIMABLE and implement a way to claim the yields to the staking contract.

0xCalibur (Abracadabra) confirmed and commented:

It’s fixed here:

- https://github.com/Abracadabra-money/abracadabra-money-contracts/blob/main/src/blast/BlastLockingMultiRewards.sol

# [M-15] LockingMultiRewards contract on Blast does not configure gas yield nor token yield mode.

- **Contest:** Abracadabra Mimswap
- **Slug:** 2024-03-abracadabra-mimswap
- **Finding ID:** M-15
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-abracadabra-mimswap
- **Source snapshot:** competitions/2024-03-abracadabra-mimswap/final_report.html

LockingMultiRewards contract on Blast does not configure gas yield nor token yield mode.

Submitted by DarkTower The Abracadabra team configures gas yield for all contracts in the Blast L2 except for the LockingMultiRewards contract. Therefore, gas yields accrued on the LockingMultiRewards staking contract since deployment will be lost as it was never configured using BlastYields.configureDefaultClaimables. If the contract holds native yield reward tokens in ERC20 such as WETHRebasing or USDB then these rewards will also be lost.

## Recommended Mitigation Steps

Configure the Blast gas yield mode for the LockingMultiRewards staking contract in the constructor and expose a function for admin to collect the yields. Additionally, also consider whether Blast points need to be configured here.

cccz (Judge) commented:

I’ll consider this an M about value leak.

Note: For full discussion, see here.

# [M-16] User can grief bootstrap process by sending the cap amount of unlocked tokens to it.

- **Contest:** Abracadabra Mimswap
- **Slug:** 2024-03-abracadabra-mimswap
- **Finding ID:** M-16
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-abracadabra-mimswap
- **Source snapshot:** competitions/2024-03-abracadabra-mimswap/final_report.html

Submitted by DarkTower, also found by ether_sky The bootstrap process relies on users locking up all their tokens as it will only use tokens that are marked locked in totals[MIM].locked and totals[USDB].locked.

BlastOnboardingBoot.sol#L96-L127 function bootstrap ( uint256 minAmountOut ) external onlyOwner onlyState (State.Closed) returns ( address, address, uint256 ) { if ( pool != address ( 0 )) { revert ErrAlreadyBootstrapped (); } => uint256 baseAmount = totals [ MIM ].

locked; => uint256 quoteAmount = totals [ USDB ].

locked; MIM.

safeApprove ( address ( router ), type ( uint256 ).

max ); USDB.

safeApprove ( address ( router ), type ( uint256 ).

max ); ( pool, totalPoolShares ) = router.

createPool ( MIM, USDB, FEE_RATE, I, K, address ( this ), baseAmount, quoteAmount ); if ( totalPoolShares < minAmountOut ) { revert ErrInsufficientAmountOut (); } // Create staking contract // 3x boosting for locker, 7 days reward duration, 13 weeks lp locking // make this contract temporary the owner the set it as an operator // for permissionned `stakeFor` during the claiming process and then // transfer the ownership to the onboarding owner.

staking = new LockingMultiRewards ( pool, 30_000, 7 days, 13 weeks, address ( this )); staking.

setOperator ( address ( this ), true ); staking.

transferOwnership ( owner ); // Approve staking contract pool.

safeApprove ( address ( staking ), totalPoolShares ); emit LogLiquidityBootstrapped ( pool, address ( staking ), totalPoolShares ); return ( pool, address ( staking ), totalPoolShares ); } However, it is possible that these values can be zero because there is a cap on the amount of tokens that can be stored in the contract.

BlastOnboarding.sol#L101-L121 function deposit ( address token, uint256 amount, bool lock_ ) external whenNotPaused onlyState (State.Opened) onlySupportedTokens ( token ) { token.

safeTransferFrom ( msg.

sender, address ( this ), amount ); if ( lock_ ) { totals [ token ].

locked += amount; balances [ msg.

sender ][ token ].

locked += amount; } else { totals [ token ].

unlocked += amount; balances [ msg.

sender ][ token ].

unlocked += amount; } totals [ token ].

total += amount; if ( caps [ token ] > 0 && totals [ token ].

total > caps [ token ]) { revert ErrCapReached (); } balances [ msg.

sender ][ token ].

total += amount; emit LogDeposit ( msg.

sender, token, amount, lock_ ); } In the code above, notice that there is a cap on the amount of tokens that can be sent to the contract caps[token]. The problem here is that it is checked against the total token amount totals[token].total rather than the locked amount totals[token].locked.

Therefore a griefer can deposit tokens up to the caps[token] with lock_ = false. The result, is that no one else can deposit tokens into the contract.

During bootstrap, since these tokens are still considered unlocked, then totals[MIM].locked = 0 and totals[USDB].locked = 0, therefore there won’t be any locked tokens available for the bootstrapping process.

## Recommended Mitigation Steps

Instead of checking caps[token] against totals[token].total, it should be checked against totals[token].locked.

0xCalibur (Abracadabra) confirmed, but disagreed with severity and and commented:

- https://github.com/Abracadabra-money/abracadabra-money-contracts/pull/139
cccz (Judge) decreased severity to Medium and commented:

Incorrect cap check, consider M.

Note: For full discussion, see here.

## Rejected Primary Findings

# Rejected Primary Findings: Abracadabra Mimswap

No rejected primary findings were captured for this contest in the current corpus.

- **Rejected primary count:** 0
- **Primary submission rows captured:** 0
- **Submission status:** requires_authentication
