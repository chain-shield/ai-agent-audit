# Benchmark Ground Truth: Revert Lend

## Accepted H/M Findings

# Accepted H/M Findings: Revert Lend

# [H-01] V3Vault.sol permit signature does not check receiving token address is USDC

- **Contest:** Revert Lend
- **Slug:** 2024-03-revert-lend
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-03-revert-lend
- **Source snapshot:** competitions/2024-03-revert-lend/final_report.html

V3Vault.sol permit signature does not check receiving token address is USDC Submitted by VAD37, also found by thank_you ( 1, 2, 3 ), santiellena, ArsenLupin, jesusrod15, and ayden In V3Vault.sol there all 3 instances of permit2.permitTransferFrom(), all 3 does not check token transfered in is USDC token. Allowing user to craft permit signature from any ERC20 token and Vault will accept it as USDC.

## Impact

User can steal all USDC from vault using permit signature of any ERC20 token.

## Recommended Mitigation Steps

Fix missing user input validations in 3 all instances of permit2:

- https://github.com/code-423n4/2024-03-revert-lend/blob/main/src/V3Vault.sol#L717C1-L725C15
- https://github.com/code-423n4/2024-03-revert-lend/blob/main/src/V3Vault.sol#L893C1-L898C15
- https://github.com/code-423n4/2024-03-revert-lend/blob/main/src/V3Vault.sol#L877C1-L917C6
if ( params.

permitData.

length > 0 ) { ( ISignatureTransfer.

PermitTransferFrom memory permit, bytes memory signature ) = abi.

decode ( params.

permitData, ( ISignatureTransfer.

PermitTransferFrom, bytes )); require ( permit.

permitted.

token == asset, "V3Vault: invalid token" ); //@permitted amount is checked inside uniswap Permit2 permit2.

permitTransferFrom ( permit, ISignatureTransfer.

SignatureTransferDetails ( address ( this ), state.

liquidatorCost ), msg.

sender, signature ); } else { // take value from liquidator SafeERC20.

safeTransferFrom ( IERC20 ( asset ), msg.

sender, address ( this ), state.

liquidatorCost ); }

## Assessed type

ERC20 kalinbas (Revert) confirmed Revert mitigated:

PR here - checks token in permit.

Status:

Mitigation confirmed. Full details in reports from thank_you, b0g0 and ktg.

# [H-02] Risk of reentrancy onERC721Received function to manipulate collateral token configs shares

- **Contest:** Revert Lend
- **Slug:** 2024-03-revert-lend
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-03-revert-lend
- **Source snapshot:** competitions/2024-03-revert-lend/final_report.html

onERC721Received function to manipulate collateral token configs shares Submitted by Aymen0909, also found by b0g0

- https://github.com/code-423n4/2024-03-revert-lend/blob/main/src/V3Vault.sol#L454-L473
- https://github.com/code-423n4/2024-03-revert-lend/blob/main/src/V3Vault.sol#L1223-L1241
Issue Description The onERC721Received function is invoked whenever the vault contract receives a Uniswap V3 position ERC721 token. This can happen either when an owner creates a new position or when a transformation occurs.

For this issue, we’ll focus on the second case, specifically when a position is going through a transformation, which creates a new position token. In such a case, we have tokenId != oldTokenId, and the else block is run, as shown below:

function onERC721Received ( address, address from, uint256 tokenId, bytes calldata data ) external override returns ( bytes4 ) {...

if {...

} else { uint256 oldTokenId = transformedTokenId; // if in transform mode - and a new position is sent - current position is replaced and returned if ( tokenId != oldTokenId ) { address owner = tokenOwner [ oldTokenId ]; // set transformed token to new one transformedTokenId = tokenId; // copy debt to new token loans [ tokenId ] = Loan ( loans [ oldTokenId ].

debtShares ); _addTokenToOwner ( owner, tokenId ); emit Add ( tokenId, owner, oldTokenId ); // clears data of old loan _cleanupLoan ( oldTokenId, debtExchangeRateX96, lendExchangeRateX96, owner ); //@audit can reenter with onERC721Received and call repay or borrow to call _updateAndCheckCollateral twice and manipulate collateral token configs // sets data of new loan _updateAndCheckCollateral ( tokenId, debtExchangeRateX96, lendExchangeRateX96, 0, loans [ tokenId ].

debtShares ); } return IERC721Receiver.

onERC721Received.

selector; } We should note that the _cleanupLoan function does return the old position token to the owner:

function _cleanupLoan ( uint256 tokenId, uint256 debtExchangeRateX96, uint256 lendExchangeRateX96, address owner ) internal { _removeTokenFromOwner ( owner, tokenId ); _updateAndCheckCollateral ( tokenId, debtExchangeRateX96, lendExchangeRateX96, loans [ tokenId ].

debtShares, 0 ); delete loans [ tokenId ]; nonfungiblePositionManager.

safeTransferFrom ( address ( this ), owner, tokenId ); emit Remove ( tokenId, owner ); } The issue that can occur is that the _cleanupLoan is invoked before the _updateAndCheckCollateral call. So, a malicious owner can use the onERC721Received callback when receiving the old token to call the borrow function, which makes changes to loans[tokenId].debtShares and calls _updateAndCheckCollateral. When the call resumes, the V3Vault.onERC721Received function will call _updateAndCheckCollateral again, resulting in incorrect accounting of internal token configs debt shares ( tokenConfigs[token0].totalDebtShares & tokenConfigs[token1].totalDebtShares ) and potentially impacting the vault borrowing process negatively.

## Impact

A malicious attacker could use the AutoRange transformation process to manipulate the internal token configs debt shares, potentially resulting in:

Fewer loans being allowed by the vault than expected.

A complete denial-of-service (DOS) for all borrow operations.

Tools Used VS Code

## Recommended Mitigation

The simplest way to address this issue is to ensure that the onERC721Received function follows the Correctness by Construction (CEI) pattern, as follows:

function onERC721Received(address, address from, uint256 tokenId, bytes calldata data) external override returns (bytes4) {...

if {...

} else { uint256 oldTokenId = transformedTokenId; // if in transform mode - and a new position is sent - current position is replaced and returned if (tokenId != oldTokenId) { address owner = tokenOwner[oldTokenId]; // set transformed token to new one transformedTokenId = tokenId; // copy debt to new token loans[tokenId] = Loan(loans[oldTokenId].debtShares); _addTokenToOwner(owner, tokenId); emit Add(tokenId, owner, oldTokenId); -- // clears data of old loan -- _cleanupLoan(oldTokenId, debtExchangeRateX96, lendExchangeRateX96, owner); // sets data of new loan _updateAndCheckCollateral( tokenId, debtExchangeRateX96, lendExchangeRateX96, 0, loans[tokenId].debtShares ); ++ // clears data of old loan

++ _cleanupLoan(oldTokenId, debtExchangeRateX96, lendExchangeRateX96, owner); } return IERC721Receiver.onERC721Received.selector; }

## Assessed type

Context kalinbas (Revert) confirmed via duplicate Issue #309:

Revert mitigated:

PRs here and here - removed sending of NFT to avoid reentrancy.

Status:

Mitigation confirmed. Full details in reports from thank_you, ktg and b0g0.

# [H-03] V3Vault::transform does not validate the data input and allows a depositor to exploit any position approved on the transformer

- **Contest:** Revert Lend
- **Slug:** 2024-03-revert-lend
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-03-revert-lend
- **Source snapshot:** competitions/2024-03-revert-lend/final_report.html

V3Vault::transform does not validate the data input and allows a depositor to exploit any position approved on the transformer Submitted by b0g0 Any account holding a position inside V3Vault can transform any NFT position outside the vault that has been delegated to Revert operators for transformation ( AutoRange, AutoCompound and all other transformers that manage positions outside of the vault).

The exploiter can pass any params at any time, affecting positions they do not own and their funds critically.

## Vulnerability details

In order to borrow from V3Vault, an account must first create a collateralized position by sending his position NFT through the create() function Any account that has a position inside the vault can use the transform() function to manage the NFT, while it is owned by the vault:

- https://github.com/code-423n4/2024-03-revert-lend/blob/435b054f9ad2404173f36f0f74a5096c894b12b7/src/V3Vault.sol#L497
function transform(uint256 tokenId, address transformer, bytes calldata data) external override returns (uint256 newTokenId) {....

//@audit -> tokenId inside data not checked (uint256 newDebtExchangeRateX96,) = _updateGlobalInterest(); address loanOwner = tokenOwner[tokenId]; // only the owner of the loan, the vault itself or any approved caller can call this if (loanOwner != msg.sender && !transformApprovals[loanOwner][tokenId][msg.sender]) { revert Unauthorized(); } // give access to transformer nonfungiblePositionManager.approve(transformer, tokenId); (bool success,) = transformer.call(data); if (!success) { revert TransformFailed(); }....

// check owner not changed (NEEDED because token could have been moved somewhere else in the meantime) address owner = nonfungiblePositionManager.ownerOf(tokenId); if (owner != address(this)) { revert Unauthorized(); }....

return tokenId; } The user passes an approved transformer address and the calldata to execute on it. The problem here is that the function only validates the ownership of the uint256 tokenId input parameter. However, it never checks if the tokenId encoded inside bytes calldata data parameter belongs to msg.sender.

This allows any vault position holder to call an allowed transformer with arbitrary params encoded as calldata and change any position delegated to that transformer.

This will impact all current and future transformers that manage Vault positions. To prove the exploit, I’m providing
## Recommended Mitigation Steps

Consider adding a check inside transform() to make sure the provided tokenId and the one encoded as calldata are the same. This way the caller will not be able to manipulate other accounts positions.

## Assessed type

Invalid Validation kalinbas (Revert) confirmed Revert mitigated:

PR here - refactoring to make all transformers properly check caller permission.

Status:

Mitigation confirmed. Full details in reports from ktg, thank_you and b0g0.

# [H-04] V3Utils.execute() does not have caller validation, leading to stolen NFT positions from users

- **Contest:** Revert Lend
- **Slug:** 2024-03-revert-lend
- **Finding ID:** H-04
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-03-revert-lend
- **Source snapshot:** competitions/2024-03-revert-lend/final_report.html

V3Utils.execute() does not have caller validation, leading to stolen NFT positions from users Submitted by 0xjuan, also found by CaeraDenoir, santiellena, Tigerfrake, Timenov, and novamanbg When a user wants to use V3Utils, one of the flows stated by the protocol is as follows:

TX1: User calls NPM.approve(V3Utils, tokenId).

TX2: User calls V3Utils.execute() with specific instructions.

Note that this can’t be done in one transaction since in TX1, the NPM has to be called directly by the EOA which owns the NFT. Thus, the V3Utils.execute() would have to be called in a subsequent transaction.

Now this is usually a safe design pattern, but the issue is that V3Utils.execute() does not validate the owner of the UniV3 Position NFT that is being handled. This allows anybody to provide arbitrary instructions and call V3Utils.execute() once the NFT has been approved in TX1.

A malicious actor provide instructions that include the following:

WhatToDo = WITHDRAW_AND_COLLECT_AND_SWAP.

recipient = malicious_actor_address.

liquidity = total_position_liquidity.

This would collect all liquidity from the position that was approved, and send it to the malicious attacker who didn’t own the position.

## Impact

The entire liquidity of a specific UniswapV3 liquidity provision NFT can be stolen by a malicious actor, with zero cost.

## Recommended Mitigation Steps

Add a check to ensure that only the owner of the position can call V3Utils.execute.

Note the fix also checks for the case where a user may have transferred the token into the V3Utils. In that case it is fine that msg.sender != tokenOwner, since tokenOwner would then be the V3Utils contract itself.

function execute(uint256 tokenId, Instructions memory instructions) public returns (uint256 newTokenId) { + address tokenOwner = nonfungiblePositionManager.ownerOf(tokenId); + if (tokenOwner != msg.sender && tokenOwner != address(this)) { + revert Unauthorized(); + } /* REST OF CODE */ }

## Assessed type

Access Control kalinbas (Revert) confirmed Revert mitigated:

PR here - refactoring to make all transformers properly check caller permission.

Status:

Mitigation confirmed. Full details in reports from thank_you, ktg and b0g0.

# [H-05] _getReferencePoolPriceX96() will show incorrect price for negative tick deltas in current implementation cause it doesn’t round up for them

- **Contest:** Revert Lend
- **Slug:** 2024-03-revert-lend
- **Finding ID:** H-05
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-03-revert-lend
- **Source snapshot:** competitions/2024-03-revert-lend/final_report.html

_getReferencePoolPriceX96() will show incorrect price for negative tick deltas in current implementation cause it doesn’t round up for them Submitted by Bauchibred, also found by grearlake ( 1, 2 ), Giorgio, and kodyvim Take a look here.

function _getReferencePoolPriceX96 ( IUniswapV3Pool pool, uint32 twapSeconds ) internal view returns ( uint256 ) { uint160 sqrtPriceX96; // if twap seconds set to 0 just use pool price if ( twapSeconds == 0 ) { ( sqrtPriceX96,,,,,,) = pool.

slot0 (); } else { uint32 [] memory secondsAgos = new uint32 []( 2 ); secondsAgos [ 0 ] = 0; // from (before) secondsAgos [ 1 ] = twapSeconds; // from (before) ( int56 [] memory tickCumulatives,) = pool.

observe ( secondsAgos ); // pool observe may fail when there is not enough history available (only use pool with enough history!) //@audit int24 tick = int24 (( tickCumulatives [ 0 ] - tickCumulatives [ 1 ]) / int56 ( uint56 ( twapSeconds ))); sqrtPriceX96 = TickMath.

getSqrtRatioAtTick ( tick ); } return FullMath.

mulDiv ( sqrtPriceX96, sqrtPriceX96, Q96 ); } This function is used to calculate the reference pool price. It uses either the latest slot price or TWAP based on twapSeconds.

Now note that unlike the original uniswap implementation, here the delta of the tick cumulative is being calculated in a different manner, i.e protocol implements ( tickCumulatives [0] - tickCumulatives [1] instead of tickCumulatives[1] - (tickCumulatives[0] which is because here, secondsAgos[0] = 0; and secondsAgos[1] = twapSeconds;; unlike in Uniswap OracleLibrary where secondsAgos[0] = secondsAgo; and secondsAgos[1] = 0;, so everything checks out and the tick deltas are calculated accurately, i.e in our case tickCumulativesDelta = tickCumulatives[0] - tickCumulatives[1].

The problem now is that in the case if our tickCumulativesDelta is negative, i.e int24(tickCumulatives[0] - tickCumulatives[1] < 0), then the tick should be rounded down, as it’s done in the uniswap library.

But this is not being done and as a result, in the case if int24(tickCumulatives[0] - tickCumulatives[1]) is negative and (tickCumulatives[0] - tickCumulatives[1]) % secondsAgo != 0, then the returned tick will be bigger then it should be; which opens possibility for some price manipulations and arbitrage opportunities.

## Impact

In this case, if int24(tickCumulatives[0] - tickCumulatives[1]) is negative and ((tickCumulatives[0] - tickCumulatives[1]) % secondsAgo != 0, then returned tick will be bigger than it should be which places protocol wanting prices to be right not be able to achieve this goal. Note that whereas protocol in some cases relies on multiple sources of price, they still come down and end on weighing the differences between the prices and reverting if a certain limit is passed ( MIN_PRICE_DIFFERENCE ) between both the Chainlink price and Uniswap twap price.

Now in the case where the implemented pricing mode is only TWAP, then the protocol would work with a flawed price since the returned price would be different than it really is; potentially leading to say, for example, some positions that should be liquidatable not being liquidated. Before liquidation, there is a check to see if the loan is healthy. Now this check queries the value of this asset via getValue() and if returned price is wrong then unhealthy loans could be pronounced as healthy and vice versa.

Also, this indirectly curbs the access to functions like borrow(), transform() and decreaseLiquidityAndCollect(), since they all make a call to _requireLoanIsHealthy(), which would be unavailable due to it’s dependence on _checkLoanIsHealthy().

This bug case causes the Automator’s _getTWAPTick() function to also return a wrong tick, which then leads to _hasMaxTWAPTickDifference() returning false data, since the difference would now be bigger eventually leading to wrongly disabling/enabling of swaps in AutoCompound.sol, whereas, it should be otherwise.

Note that for the second/third case, the call route to get to _getReferencePoolPriceX96() is:

"_checkLoanIsHealthy() -> getValue() -> _getReferenceTokenPriceX96 -> _getTWAPPriceX96 -> _getReferencePoolPriceX96() " as can be seen here.

Tools Used Uniswap V3’s OracleLibrary.

And a similar finding on Code4rena from Q1 2024.

## Recommended Mitigation Steps

Add this line:

if (tickCumulatives[0] - tickCumulatives[1] < 0 && (tickCumulatives[0] - tickCumulatives[1]) % secondsAgo != 0) timeWeightedTick --;.

## Assessed type

Uniswap kalinbas (Revert) confirmed Revert mitigated:

PR here - fixed calculation.

Status:

Mitigation confirmed. Full details in reports from thank_you, b0g0 and ktg.

# [H-06] Owner of a position can prevent liquidation due to the onERC721Received callback

- **Contest:** Revert Lend
- **Slug:** 2024-03-revert-lend
- **Finding ID:** H-06
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-03-revert-lend
- **Source snapshot:** competitions/2024-03-revert-lend/final_report.html

onERC721Received callback Submitted by 0xjuan, also found by CaeraDenoir, kinda_very_good, falconhoof, 0x175, Arz, JohnSmith, alix40, stackachu, givn, wangxx2026, Ocean_Sky, 0xloscar01, SpicyMeatball, 0xAlix2, Ali-_-Y, 0rpse, iamandreiski, 0xBugSlayer, nmirchev8, nnez, ayden, and novamanbg When liquidating a position, _cleanUpLoan() is called on the loan. This attempts to send the uniswap LP position back to the user via the following line:

nonfungiblePositionManager.

safeTransferFrom ( address ( this ), owner, tokenId ); This safeTransferFrom function call invokes the onERC721Received function on the owner’s contract. The transaction will only succeed if the owner’s contract returns the function selector of the standard onERC721Received function. However, the owner can design the function to return an invalid value, and this would lead to the safeTransferFrom reverting, thus being unable to liquidate the user.

## Impact

This leads to bad debt accrual in the protocol which cannot be prevented, and eventually insolvency.

## Recommended Mitigation Steps

One solution would be to approve the NFT to the owner and provide a way (via the front-end or another contract) for them to redeem the NFT back later on. This is a “pull over push” approach and ensures that the liquidation will occur.

Example:

function _cleanupLoan(uint256 tokenId, uint256 debtExchangeRateX96, uint256 lendExchangeRateX96, address owner) internal { _removeTokenFromOwner(owner, tokenId); _updateAndCheckCollateral(tokenId, debtExchangeRateX96, lendExchangeRateX96, loans[tokenId].debtShares, 0); delete loans[tokenId]; - nonfungiblePositionManager.safeTransferFrom(address(this), owner, tokenId); + nonfungiblePositionManager.approve(owner, tokenId); emit Remove(tokenId, owner); }

## Assessed type

DoS kalinbas (Revert) confirmed Revert mitigated:

PRs here and here - removed sending of NFT to avoid reentrancy.

Status:

Mitigation confirmed. Full details in reports from thank_you, ktg and b0g0.

Medium Risk Findings (25)

# [M-01] An attacker can easily bypass the collateral value limit factor checks

- **Contest:** Revert Lend
- **Slug:** 2024-03-revert-lend
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-revert-lend
- **Source snapshot:** competitions/2024-03-revert-lend/final_report.html

Submitted by Arz, also found by lanrebayode77 The collateral value limit factor checks in _updateAndCheckCollateral() are used to check if the current value of used collateral is more than the allowed limit. The problem here is that these checks are not done when withdrawing lent tokens so an attacker can easily bypass these checks.

Example:

The collateral value limit factor is set to 90% for both collateral tokens.

The lent amount is 100 USDC and Alice has borrowed 90 USDC.

An attacker wants to borrow the remaining 10 USDC but he cant because of the checks.

He executes an attack in 1 transaction - he supplies 10 USDC, borrows 10 USDC and then withdraws the 10 USDC that he lent.

The amount borrowed is 100 USDC even though the collateral factor is 90%.

The attacker can do this to bypass the checks, this can also block calls from the AutoRange automator. Whenever a new position is sent, _updateAndCheckCollateral() is called, which will revert because the attacker surpassed the limits.

## Impact

The attacker can easily surpass the limits, this can also make calls from AutoRange revert because _updateAndCheckCollateral() is called when the position is replaced and it will revert because the limits were surpassed. The automator will fail to change the range of the positions.

## Recommended Mitigation Steps

Not sure how this should be fixed as the collateral tokens are configured separately and the collateral factors can differ. However, maybe preventing depositing and withdrawing in the same tx/small fee can help this.

## Assessed type

Invalid Validation kalinbas (Revert) acknowledged and commented:

The fact that AutoRange automator doesn’t work if the collateral limit is reached is no problem (this is as designed).

The fact that withdrawing lent assets can lead to collateral value > limit is no problem because it is limited to a certain percentage (it’s not possible to add a huge amount, borrow it, and withdraw it (because it is borrowed)).

# [M-02] Protocol can be repeatedly gas griefed in AutoRange external call

- **Contest:** Revert Lend
- **Slug:** 2024-03-revert-lend
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-revert-lend
- **Source snapshot:** competitions/2024-03-revert-lend/final_report.html

AutoRange external call Submitted by falconhoof, also found by ktg and novamanbg Revert controlled AutoRange bot can be gas griefed and execute() reverted by malicious onERC721Received implementation

## Vulnerability Details

The initiator of a transaction pays the transaction gas; in the case of AutoRange::execute() and AutoRange::executeWithVault(), this will be a Revert controlled bot which is set up as an operator. Newly minted NFTs are sent to users via NPM::safeTransferFrom() which uses the onERC721Received callback.

An attacker can implement a malicious implementation of this callback, by wasting all the transaction gas and reverting the function to grief the protocol. It is expected that the gas spent by bots initiating transactions will be covered by protocol fees; however, no protocol fees will be generated from the attacker’s position, as AutoRange::execute() will not complete; so the protocol will experience a loss.

Furthermore, once an attacker has set the token’s config from positionConfigs, the protocol has no way to stop the griefing occurring each time the bot detects that the tokenId meets the conditions for a Range Change. Token Config is only removed from positionConfigs at the end of execute(), which the gas grief will prevent from being reached making it a recurring attack. The only recourse to the protocol is shutting down the contract completely by removing the bot address as an operator and DOSing the contract.

All this makes the likelihood of this attack quite high as it is a very inexpensive attack; user does not even need an open position and loan in the vault. A determined attacker.

## Impact

Protocol fees can be completely drained; particularly if a determined attacker sets token configs for multiple NFTs in AutoRange, all linked to the same malicious contract. Lack of fees can DOS multiple functions like the bot initiated AutoRange functions and affect the protocol’s profitability by draining fees.

Tools Used Foundry Testing

## Recommended Mitigation Steps

Enact a pull mechanism by transferring the newly minted NFT to a protocol owned contract, such as the AutoRange contract itself, from where the user initiates the transaction to transfer the NFT to themselves.

kalinbas (Revert) acknowledged and commented:

All these cases are possible but we are monitoring these off-chain bots and also implement gas-limiting, and taking action where needed.

ronnyx2017 (judge) decreased severity to Medium and commented:

valid gas grief, but not a persistent dos, so Medium.

falconhoof (warden) commented:

@ronnyx2017 - When a user adds their position’s config details via configToken(), the positionConfigs mapping is updated accordingly. From what I can see, there is no way to remove that config apart from at the end of execute().

Everytime the parameters defined in positionConfigs are met, the bot will call execute(), get griefed and user’s token config will remain in state. A malicious user can set up multiple positionConfigs to grief, with many different parameter trigger points, and the only recourse would be the shutting down of the AutoRange contract.

Once a new contract is set up of course the exact same thing can be done again, so I think it’s a strong case for a full DOS of this part of the protocol’s functionality and loss of funds for the protocol, which would be more than dust.

kalinbas (Revert) commented:

The logic which positions to execute is the responsibility of the bot. If there are worthless tokens detected or the tx simulation is not what expected the bot doesn’t execute the operation. So this is a risk which is controlled off-chain.

falconhoof (warden) commented:

How would that work? It doesn’t seem possible to foresee getting griefed, at least the first time and after that would the tokenId be blacklisted to prevent further griefing, which necessitates the shutting down of the contract?

Also, the mitigation of bots monitoring the contract is not documented under the list of known issues of the Contest’s README. I think it’s fair to flag this issue and leave up to the judge to decide if mitigation can be applied retrospectively after the audit.

ronnyx2017 (judge) commented:

I cannot understand what the warden is referring to with the “shutting down of the AutoRange contract”. There is no reason for the operator to waste gas on a transaction that continuously fails. Gas griefing is valid, and it will indeed continue to deplete the resources of the off-chain operator, but this does not constitute a substantial DoS. For predictions on any future actions/deployments, please refer to this org issue.

# [M-03] No minLoanSize means liquidators will have no incentive to liquidate small positions

- **Contest:** Revert Lend
- **Slug:** 2024-03-revert-lend
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-revert-lend
- **Source snapshot:** competitions/2024-03-revert-lend/final_report.html

minLoanSize means liquidators will have no incentive to liquidate small positions Submitted by falconhoof, also found by grearlake No minLoanSize can destabilise the protocol.

## Vulnerability Details

According to protocol team, they plan to roll out the protocol with minLoanSize = 0 and adjust that number if needs be. This can be a big issue because there will be no incentive for liquidators to liquidate small underwater positions given the gas cost. To do so would not make economic sense based on the incentive they would receive.

It also opens up a cheap attack path for would be attackers where they can borrow many small loans, which will go underwater as they accrue interest, but will not be liquidated.

## Impact

Can push the entire protocol into an underwater state. Underwater debt would first be covered by Protocol reserves and where they aren’t sufficient, lenders will bear the responsibility of the uneconomical clean up of bad debt, so both the protocol and lenders stand to lose out.

## Recommended Mitigation Steps

Close the vulnerability by implementing a realistic minLoanSize, which will incentivise liquidators to clean up bad debt.

kalinbas (Revert) acknowledged and commented:

Will do the deployment with a reasonable minLoanSize.

ronnyx2017 (judge) commented:

Normally, I would mark such issues as Low. But given that this issue provides a substantial reminder to the sponsor, I am retaining it as Medium.

# [M-04] Due to interest rates update method, Interest-Free Loans are possible and the costs of DoS are reduced

- **Contest:** Revert Lend
- **Slug:** 2024-03-revert-lend
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-revert-lend
- **Source snapshot:** competitions/2024-03-revert-lend/final_report.html

Submitted by alix40, also found by 0xPhantom, Norah ( 1, 2 ), ktg, and lanrebayode77 Allowing in interest free debt in 1 block could have several unwanted results:

Allowing for in same block (not necessarily same transaction) interest-free loans, could be abused by wales for arbitrage operations, resulting in protocol users unable to borrow because of the daily limit.

DoS attacks, with the new updates on the main net resulting in way lower transaction fees, a whale account with a big position could borrow a big amount of money and then repay it in the same block; resulting in him not paying any interest, and users unable to borrow in this block. The attack will then be repeated in each block resulting in DoS.

The attack would result not only in DoS, but also the protocol Liquidity Providers (LPs), and the protocol would lose potential interest payments for their deposits.

Please also note, that in L2 blockchains, it is quite common for multiple L2 Blocks to have the same block.timestamp. So borrower could potentially have interest-free debt on the span of multiple blocks.

## Recommended Mitigation Steps

The most simple solution to this issue, is to add a small borrow fee (percentagewise for e.g 0.1% of borrowed debt). This way even if arbitrageurs try to do swaps, or attackers try to DoS the system, Liquidity Providers will receive their fair yield (potentially a lot more yield if an attacker tries the DoS Attack described in this report).

## Assessed type

Context kalinbas (Revert) acknowledged and commented:

“Allowing for in same block (not necessarily same transaction) interest-free loans, could be abused by wales for arbitrage operations, resulting in protocol users unable to borrow because of the daily limit.” If the whale borrow is repayed in the same block - the limit is reset. So other users can borrow in the next block.

About the DOS attack: There is a way to disable this attack by increasing the dailyLimitMinValue. The probability of someone attacking like this seems very low, so we are comfortable with this workaround.

ronnyx2017 (judge) decreased severity to Medium and commented:

I do not consider this to be a high issue; the first impact is false. Regarding the second, a DoS, the attacker would suffer huge losses without gaining anything.

But I still believe the issue is valid because MEV bots have enough incentive to hold debt from the vault over multiple blocks (one by one, borrow in the index 0 tx and repay in the last index tx), which could actually lead to a deterioration in the protocol’s reliability.

lanrebayode77 (warden) commented:

@ronnyx2017 - I think the severity of this should be reconsidered due to it impact.

Due to update mode, user can get loans without interest as long as repayment is done in the same transaction. This is the entire basis for Flashloan which also comes at a cost in popular lending platforms like AAVE and UNISWAP.

Since this action can be repeated overtime, protocol will be losing a lot as unclaimed interest fee, which would have made more funds to the LPs and protocol. Since loss of funds(fee) is evident, it’s valid as a high severity.

ronnyx2017 (judge) commented:

There is no technical disagreement, the attack you mentioned has already been described in my comments above, maintain Medium for cost and likelihood of attack.

# [M-05] setReserveFactor fails to update global interest before updating reserve factor

- **Contest:** Revert Lend
- **Slug:** 2024-03-revert-lend
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-revert-lend
- **Source snapshot:** competitions/2024-03-revert-lend/final_report.html

setReserveFactor fails to update global interest before updating reserve factor Submitted by thank_you

- https://github.com/code-423n4/2024-03-revert-lend/blob/main/src/V3Vault.sol?plain=1#L1167-L1195
- https://github.com/code-423n4/2024-03-revert-lend/blob/main/src/V3Vault.sol?plain=1#L837-L840

## Impact

When the vault owner calls V3Vault.setReserveFactor(), the function updates the reserve factor for the Vault. Unfortunately, if the global interest is not updated before the reserve factor is updated, the updated reserve factor will be retroactively applied in the exchange rate formula starting at the last update. This leads to unexpected lending rate changes causing lenders to receive unexpected more or less favorable lending exchange rate depending on what the updated reserve factor value is.

## Recommended Mitigation Steps

Add _updateGlobalInterest() to the V3Vault.setReserveFactor() function before the reserve factor is updated. This ensures that the lending rate will not be artificially impacted and the updated reserve factor is not retroactively applied to the past:

function setReserveFactor ( uint32 _reserveFactorX32 ) external onlyOwner { _updateGlobalInterest (); reserveFactorX32 = _reserveFactorX32; }

## Assessed type

Math kalinbas (Revert) confirmed ronnyx2017 (judge) commented:

The losses are negligible, but it indeed breaks the math.

Revert mitigated:

Fixed here.

Status:

Mitigation confirmed. Full details in reports from b0g0, thank_you and ktg.

# [M-06] Users can lend and borrow above allowed limitations

- **Contest:** Revert Lend
- **Slug:** 2024-03-revert-lend
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-revert-lend
- **Source snapshot:** competitions/2024-03-revert-lend/final_report.html

Submitted by JohnSmith, also found by Arz, BowTiedOriole, FastChecker, shaka, Aymen0909, deepplus, KupiaSec, kennedy1030, kfx, and DanielArmstrong

- https://github.com/code-423n4/2024-03-revert-lend/blob/435b054f9ad2404173f36f0f74a5096c894b12b7/src/V3Vault.sol#L1250-L1251
- https://github.com/code-423n4/2024-03-revert-lend/blob/435b054f9ad2404173f36f0f74a5096c894b12b7/src/V3Vault.sol#L1262-L1263

## Impact

Protocol design includes limitations on how much a user can deposit and borrow per day. So it is 10% of lent money or dailyLendIncreaseLimitMin / dailyDebtIncreaseLimitMin, whichever is greater. Current implementation is wrong and makes it 110% because of mistake in calculations; which means that users are able to deposit/borrow close to 110% amount of current assets.

Issue is in these calculations:

uint256 lendIncreaseLimit = _convertToAssets ( totalSupply (), newLendExchangeRateX96, Math.

Rounding.

Up ) * ( Q32 + MAX_DAILY_LEND_INCREASE_X32 ) / Q32; uint256 debtIncreaseLimit = _convertToAssets ( totalSupply (), newLendExchangeRateX96, Math.

Rounding.

Up ) * ( Q32 + MAX_DAILY_DEBT_INCREASE_X32 ) / Q32;

## Recommended Mitigation Steps

Fix is simple for _resetDailyLendIncreaseLimit():

uint256 lendIncreaseLimit = _convertToAssets(totalSupply(), newLendExchangeRateX96, Math.Rounding.Up) - * (Q32 + MAX_DAILY_LEND_INCREASE_X32) / Q32; + * MAX_DAILY_LEND_INCREASE_X32 / Q32; And for _resetDailyDebtIncreaseLimit():

uint256 debtIncreaseLimit = _convertToAssets(totalSupply(), newLendExchangeRateX96, Math.Rounding.Up) - * (Q32 + MAX_DAILY_DEBT_INCREASE_X32) / Q32; + * MAX_DAILY_DEBT_INCREASE_X32 / Q32;

## Assessed type

Math kalinbas (Revert) confirmed Revert mitigated:

Fixed here.

Status:

Mitigation confirmed. Full details in reports from thank_you, b0g0 and ktg.

# [M-07] Large decimal of referenceToken causes overflow at oracle price calculation

- **Contest:** Revert Lend
- **Slug:** 2024-03-revert-lend
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-revert-lend
- **Source snapshot:** competitions/2024-03-revert-lend/final_report.html

referenceToken causes overflow at oracle price calculation Submitted by JecikPo, also found by linmiaomiao, kfx, KupiaSec, SpicyMeatball, kennedy1030, and t4sk The price calculation at the V3Oracle.sol will revert upon reaching certain level when referenceToken is used with high decimal value (e.g. 18). The revert (specifically happening when calling getValue() ) would make the Chainlink price feed useless; yet the TWAP price source would still be available. The protocol team would have to disable Chainlink and rely exclusively on the TWAP source reducing security of the pricing. The issue could manifest itself after certain amount of time once the project is already live and only when price returned by the feed reaches certain point.

## Recommended Mitigation Steps

Instead of calculating the price this way:

chainlinkPriceX96 = (10 ** referenceTokenDecimals) * chainlinkPriceX96 * Q96 / chainlinkReferencePriceX96 / (10 ** feedConfig.tokenDecimals); It could be done the following way as per Chainlink’s recommendation:

if (referenceTokenDecimals > feedConfig.tokenDecimals) chainlinkPriceX96 = (10 ** referenceTokenDecimals - feedConfig.tokenDecimals) * chainlinkPriceX96 * Q96 / chainlinkReferencePriceX96; else if (referenceTokenDecimals < feedConfig.tokenDecimals) chainlinkPriceX96 = chainlinkPriceX96 * Q96 / chainlinkReferencePriceX96 / (10 ** feedConfig.tokenDecimals - referenceTokenDecimals); else chainlinkPriceX96 = chainlinkPriceX96 * Q96 / chainlinkReferencePriceX96; Reference here.

## Assessed type

Decimal kalinbas (Revert) confirmed Revert mitigated:

Fixed here.

Status:

Mitigation confirmed. Full details in reports from ktg, thank_you and b0g0.

# [M-08] DailyLendIncreaseLimitLeft and dailyDebtIncreaseLimitLeft are not adjusted accurately

- **Contest:** Revert Lend
- **Slug:** 2024-03-revert-lend
- **Finding ID:** M-08
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-revert-lend
- **Source snapshot:** competitions/2024-03-revert-lend/final_report.html

DailyLendIncreaseLimitLeft and dailyDebtIncreaseLimitLeft are not adjusted accurately Submitted by FastChecker, also found by DanielArmstrong

- https://github.com/code-423n4/2024-03-revert-lend/blob/main/src/V3Vault.sol#L807-L949
- https://github.com/code-423n4/2024-03-revert-lend/blob/main/src/V3Vault.sol#L807-L883
- https://github.com/code-423n4/2024-03-revert-lend/blob/main/src/V3Vault.sol#L807-L1249

## Vulnerability Details

When the V3Vault.sol#_withdraw and V3Vault.sol#_repay functions are called, dailyLendIncreaseLimitLeft and dailyDebtIncreaseLimitLeft are increased. However, if it is called before _withdraw and _repay are called, this increase becomes meaningless.

## Impact

Even if the V3Vault.sol#_withdraw and V3Vault.sol#_repay functions are called, dailyLendIncreaseLimitLeft and dailyDebtIncreaseLimitLeft do not increase, so the protocol does not work as intended.

## Recommended Mitigation Steps

VeVault.sol#_withdraw function is modified as follows:

function _withdraw ( address receiver, address owner, uint256 amount, bool isShare ) internal returns ( uint256 assets, uint256 shares ) { ( uint256 newDebtExchangeRateX96, uint256 newLendExchangeRateX96 ) = _updateGlobalInterest (); + _resetDailyLendIncreaseLimit ( newLendExchangeRateX96, false ); if ( isShare ) { shares = amount; assets = _convertToAssets ( amount, newLendExchangeRateX96, Math.

Rounding.

Down ); } else { assets = amount; shares = _convertToShares ( amount, newLendExchangeRateX96, Math.

Rounding.

Up ); } // if caller has allowance for owners shares - may call withdraw if ( msg.

sender != owner ) { _spendAllowance ( owner, msg.

sender, shares ); } (, uint256 available,) = _getAvailableBalance ( newDebtExchangeRateX96, newLendExchangeRateX96 ); if ( available < assets ) { revert InsufficientLiquidity (); } // fails if not enough shares _burn ( owner, shares ); SafeERC20.

safeTransfer ( IERC20 ( asset ), receiver, assets ); // when amounts are withdrawn - they may be deposited again dailyLendIncreaseLimitLeft += assets; emit Withdraw ( msg.

sender, receiver, owner, assets, shares ); } VeVault.sol#_repay function is modified as follows:

function _repay ( uint256 tokenId, uint256 amount, bool isShare, bytes memory permitData ) internal { ( uint256 newDebtExchangeRateX96, uint256 newLendExchangeRateX96 ) = _updateGlobalInterest (); + _resetDailyLendIncreaseLimit ( newLendExchangeRateX96, false ); Loan storage loan = loans [ tokenId ]; uint256 currentShares = loan.

debtShares; uint256 shares; uint256 assets; if ( isShare ) { shares = amount; assets = _convertToAssets ( amount, newDebtExchangeRateX96, Math.

Rounding.

Up ); } else { assets = amount; shares = _convertToShares ( amount, newDebtExchangeRateX96, Math.

Rounding.

Down ); } // fails if too much repayed if ( shares > currentShares ) { revert RepayExceedsDebt (); } if ( assets > 0 ) { if ( permitData.

length > 0 ) { ( ISignatureTransfer.

PermitTransferFrom memory permit, bytes memory signature ) = abi.

decode ( permitData, ( ISignatureTransfer.

PermitTransferFrom, bytes )); permit2.

permitTransferFrom ( permit, ISignatureTransfer.

SignatureTransferDetails ( address ( this ), assets ), msg.

sender, signature ); } else { // fails if not enough token approved SafeERC20.

safeTransferFrom ( IERC20 ( asset ), msg.

sender, address ( this ), assets ); } uint256 loanDebtShares = loan.

debtShares - shares; loan.

debtShares = loanDebtShares; debtSharesTotal -= shares; // when amounts are repayed - they maybe borrowed again dailyDebtIncreaseLimitLeft += assets; _updateAndCheckCollateral ( tokenId, newDebtExchangeRateX96, newLendExchangeRateX96, loanDebtShares + shares, loanDebtShares ); address owner = tokenOwner [ tokenId ]; // if fully repayed if ( currentShares == shares ) { _cleanupLoan ( tokenId, newDebtExchangeRateX96, newLendExchangeRateX96, owner ); } else { // if resulting loan is too small - revert if ( _convertToAssets ( loanDebtShares, newDebtExchangeRateX96, Math.

Rounding.

Up ) < minLoanSize ) { revert MinLoanSize (); } emit Repay ( tokenId, msg.

sender, owner, assets, shares ); } kalinbas (Revert) confirmed Revert mitigated:

Fixed here.

Status:

Mitigation confirmed. Full details in reports from thank_you, b0g0 and ktg.

# [M-09] Liquidation reward sent to msg.sender instead of recipient

- **Contest:** Revert Lend
- **Slug:** 2024-03-revert-lend
- **Finding ID:** M-09
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-revert-lend
- **Source snapshot:** competitions/2024-03-revert-lend/final_report.html

Submitted by Giorgio, also found by thank_you

- https://github.com/code-423n4/2024-03-revert-lend/blob/435b054f9ad2404173f36f0f74a5096c894b12b7/src/V3Vault.sol#L1078-L1080

## Vulnerability details

When performing liquidation the liquidator will fill the LiquidateParams containing the liquidation details. The issue here is that instead of sending the liquidation rewards to the LiquidateParams.recipient, the rewards will be sent to msg.sender.

## Impact

The liquidation rewards will be sent to msg.sender instead of the recipient, any external logic that relies on the fact that the liquidation rewards will be sent to recipient won’t hold; this will influence the protocol’s composability.

## Recommended Mitigation Steps

The mitigation is straight forward. Use params.recipient instead of msg.sender for that specific call.

(amount0, amount1) = _sendPositionValue(params.tokenId, state.liquidationValue, -- state.fullValue, state.feeValue, msg.sender); ++ state.fullValue, state.feeValue, params.recipient);

## Assessed type

Context kalinbas (Revert) confirmed Revert mitigated:

Fixed here.

Status:

Mitigation confirmed. Full details in reports from b0g0, thank_you and ktg.

# [M-10] Users’s tokens stuck in AutoCompound after Vault is deactivated

- **Contest:** Revert Lend
- **Slug:** 2024-03-revert-lend
- **Finding ID:** M-10
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-revert-lend
- **Source snapshot:** competitions/2024-03-revert-lend/final_report.html

AutoCompound after Vault is deactivated Submitted by ktg

- https://github.com/code-423n4/2024-03-revert-lend/blob/main/src/automators/Automator.sol#L79-L82
- https://github.com/code-423n4/2024-03-revert-lend/blob/main/src/transformers/AutoCompound.sol#L201-L207

## Recommended Mitigation Steps

I recommend checking the total user left tokens in variable positionBalances and only allow deactivating the current Vault if the number if zero.

## Assessed type

Invalid Validation kalinbas (Revert) confirmed and commented:

Agree this to be an issue. Probably will make whitelisting vaults in automators be a non-reversible action.

ronnyx2017 (judge) commented:

A temporary, mitigable DoS caused by normal administrative operations, so a valid Medium.

Revert mitigated:

Fixed here.

Status:

Mitigation confirmed. Full details in reports from thank_you, b0g0 and ktg.

# [M-11] Lack of safety buffer in _checkLoanIsHealthy could subject users who take out the max loan into a forced liquidation

- **Contest:** Revert Lend
- **Slug:** 2024-03-revert-lend
- **Finding ID:** M-11
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-revert-lend
- **Source snapshot:** competitions/2024-03-revert-lend/final_report.html

_checkLoanIsHealthy could subject users who take out the max loan into a forced liquidation Submitted by CRYP70, also found by alix40, shaka, and atoko The _checkLoanIsHealthy function is used in the V3Vault to assess a user’s given position and determine the health factor of the loan. As there is no safety buffer when checking the health factor of a given position, users could be subject to a negative health factor if there are minor movements in the market which could result in liquidation or in the worst case scenario, an attacker could force a liquidation on a user and profit by sinking their position in the Uniswap pool.

## Vulnerability Details

The _checkLoanIsHealthy function holds the implementation to check if a users position is healthy and will return false if the position not able to be liquidated by obtaining the full value of the collateral inclusive of fees through the oracle by the tokenId. The collateralValue is then calculated from _calculateTokenCollateralFactorX32. Finally, we return whether the collateralValue is greater than or equal to the debt requested:

function _checkLoanIsHealthy ( uint256 tokenId, uint256 debt ) internal view returns ( bool isHealthy, uint256 fullValue, uint256 collateralValue, uint256 feeValue ) { ( fullValue, feeValue,,) = oracle.

getValue ( tokenId, address ( asset )); uint256 collateralFactorX32 = _calculateTokenCollateralFactorX32 ( tokenId ); collateralValue = fullValue.

mulDiv ( collateralFactorX32, Q32 ); isHealthy = collateralValue >= debt; } However, the issue in the code is that the the start of the liquidation threshold (I.E. 85%) is supposed to be greater than the loan to value ratio (I.E. 80%) to create some breathing room for the user and reduce the risk of the protocol incurring bad debt.

## Impact

Borrowers of the protocol may be unfairly liquidated due to minor movements in the market when taking out the max loan. In the worst case scenario, a user could be subject to a forced liquidation by the attacker (a malicious user or a bot) for profit.

## Recommended Mitigation Steps

Consider implementing a safety buffer for the users position, which is considered when attempting to take out a loan so that they are not subject to liquidations due to minor changes in the market. For instance, if the liquidation threshold is at 80%, the borrower’s max loan is at 75% of that ratio. After some small changes in market conditions the position is now at a 75.00002% and is still safe from liquidations as it is still over collateralised. This can be done by implementing this as another state variable and checking that the requested debt is initially below this threshold. When attempting to liquidate, the health of the position is then checked against the liquidation threshold.

kalinbas (Revert) disputed and commented:

It’s a design choice and we probably will add a safety buffer on the frontend. But in contract it is not needed.

ronnyx2017 (judge) decreased severity to Medium and commented:

I think this is a valid Medium, as typically the safety measures added at the frontend are considered unreliable. I don’t quite understand the significant benefits of the current design; it only slightly increases capital efficiency but exposes users to liquidation risks.

kalinbas (Revert) confirmed and commented:

Agreed. Will add this safety buffer Revert mitigated:

PR here - added safety buffer for borrow and decreaseLiquidity (not for transformers).

Status:

Mitigation confirmed. Full details in reports from thank_you and b0g0.

# [M-12] Unmitigated [Medium Bot-Report Issue #12] Mitigation Error Lenders can drain the Vault when withdrawing An attacker can DOS AutoExit and AutoRange transformers and incur losses for position owners

- **Contest:** Revert Lend
- **Slug:** 2024-03-revert-lend
- **Finding ID:** M-12
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-revert-lend
- **Source snapshot:** competitions/2024-03-revert-lend/final_report.html

Submitted by thank_you, also found by b0g0 and ktg Original issue M-12: Wrong global lending limit check in _deposit function Comments Revert utilizes a global lend limit to ensure that lenders do not exceed a global lend limit. Unfortunately, the global lend limit is denominated in assets. The value it compares itself to is totalSupply(), which is denominated in shares. This comparison is invalid since both values are in different denominations.

Lines of code

- https://github.com/revert-finance/lend/blob/audit/src/V3Vault.sol?plain=1#L961-L963

## Vulnerability details

The _deposit() function incorrectly utilizes the wrong denomination when comparing the globalLendLimit to the number of shares. The globalLendLimit denomination is set in assets. However, totalSupply() + shares denomination is set in shares. This makes this comparison check incorrect and can lead to more assets being lent than anticipated.

## Impact

More assets may be deposited than expected.

## Recommended Mitigation Steps

Convert the totalSupply() + shares to the correct denomination in assets:

uint256 totalSharesDenominatedInAssets = _convertToAssets ( totalSupply () + shares, newLendExchangeRateX96, Math.

Rounding.

Up ); if ( totalSharesDenominatedInAssets > globalLendLimit ) { revert GlobalLendLimit (); }

## Assessed type

Math [Medium Bot-Report Issue #12] Mitigation Error Submitted by ktg, also found by b0g0 and thank_you

- https://github.com/revert-finance/lend/blob/audit/src/V3Oracle.sol#L360-L362
Original issue Missing L2 sequencer checks for Chainlink oracle

## Impact

Wrong logic in L2 sequencer check:

If sequencerUptimeFeed is set, then the function will revert most of the time and affect a lot of other functions in Revert Lend.

## Recommended Mitigation Steps

Change if (sequencerAnswer == 0) to if (sequencerAnswer == 1).

## Assessed type

Invalid Validation kalinbas (Revert) confirmed Lenders can drain the Vault when withdrawing Submitted by b0g0 Severity: High

- https://github.com/revert-finance/lend/blob/audit/src/V3Vault.sol#L1007-L1010

## Impact

V3Vault can be drained through the withdraw() function due to improper asset conversion.

Vulnerability PR-14 introduced a couple of updates to the V3Vault contract in response to this finding in order to prevent liquidations from getting DOSed.

A changes has also been introduced to _withdraw() so that instead of reverting when a lender tries to withdraw more shares than he owns, the amount is automatically reduced to the max withdrawable shares for that lender. This is how the change looks:

- https://github.com/revert-finance/lend/blob/audit/src/V3Vault.sol#L1007-L1010
function _withdraw ( address receiver, address owner, uint256 amount, bool isShare ) internal returns ( uint256 assets, uint256 shares ) {....

if ( isShare ) { shares = amount; assets = _convertToAssets ( amount, newLendExchangeRateX96, Math.

Rounding.

Down ); } else { assets = amount; shares = _convertToShares ( amount, newLendExchangeRateX96, Math.

Rounding.

Up ); } + uint256 ownerBalance = balanceOf ( owner ); + if ( shares > ownerBalance ) { + shares = ownerBalance; + assets = _convertToAssets ( amount, newLendExchangeRateX96, Math.

Rounding.

Down ); + }....

} The problem is that the newly added code does not use the proper variable to convert the owner shares to assets. If you look closely you will see that _convertToAssets() uses amount instead of shares.

In the case the function is called with isShare == true (e.g redeem() ) everything will be ok, since amount == shares. However, if _withdraw() is called with isShare == false (e.g withdraw() ) the conversion will be wrong, because amount == assets. This will inflate the assets variable and since there are no checks after that to prevent it, more tokens will be transferred to the owner than he owns.

## Recommended Mitigation

Refactor the newly added check inside _withdraw() to use shares instead of amount:

uint256 ownerBalance = balanceOf ( owner ); if ( shares > ownerBalance ) { shares = ownerBalance; - assets = _convertToAssets ( amount, newLendExchangeRateX96, Math.

Rounding.

Down ); + assets = _convertToAssets ( shares, newLendExchangeRateX96, Math.

Rounding.

Down ); }

## Assessed type

Invalid Validation kalinbas (Revert) confirmed An attacker can DOS AutoExit and AutoRange transformers and incur losses for position owners Submitted by b0g0 Severity: Medium An exploiter can block the execution of AutoExit and AutoRange transformers, which leads to the following consequences:

Limit orders and Stoploss orders - position owners won’t be able to exit a bad market and will suffer losses.

Autorange orders - positions that go out-of-range won’t be rebalanced leading to missed profits or direct losses.

## Vulnerability details

The AutoRange.sol and AutoExit.sol contracts serve the following functionality in Revert Lend:

AutoRange.sol contract Auto-Range automates the process of rebalancing your liquidity positions. When the token price moves and your position goes out-of-range by your selected percentage, the system then automatically rebalances your position` AutoExit contract Auto-Exit lets you pre-configure a position so that the liquidity is automatically withdrawn when the pool price reaches a predetermined value. Moreover, you can optionally configure the system to swap from one token to the other on withdrawal, providing a safety net for your investments akin to a stop-loss order.

Both of those contracts implement an execute() function that respectively transforms an NFT position based on the parameters provided to it. It can only be called by revert controlled bots (operators) which owners have approved for their position or by the V3Vault through it’s transform() function.

The problem in both of those contracts is that the execute() function includes a validation that allows malicious users to DOS transaction execution and thus compromise the safety and integrity of the managed positions.

AutoExit::execute():

- https://github.com/revert-finance/lend/blob/audit/src/automators/AutoExit.sol#L130
function execute ( ExecuteParams calldata params ) external {....

// get position info (,, state.

token0, state.

token1, state.

fee, state.

tickLower, state.

tickUpper, state.

liquidity,,,,) = nonfungiblePositionManager.

positions ( params.

tokenId );....

// @audit can be front-run and prevent execution if ( state.

liquidity != params.

liquidity ) { revert LiquidityChanged (); }....

} AutoRange::execute():

- https://github.com/revert-finance/lend/blob/audit/src/transformers/AutoRange.sol#L139
function execute ( ExecuteParams calldata params ) external {....

// get position info (,, state.

token0, state.

token1, state.

fee, state.

tickLower, state.

tickUpper, state.

liquidity,,,,) = nonfungiblePositionManager.

positions ( params.

tokenId ); // @audit can be front-run and prevent execution if ( state.

liquidity != params.

liquidity ) { revert LiquidityChanged (); }....

} The problematic validation shared in both function is this one:

// @audit can be front-run and prevent execution if ( state.

liquidity != params.

liquidity ) { revert LiquidityChanged (); } The check is meant to ensure that the execution parameters the transaction was initiated with, are executed under the same conditions (the same liquidity) that were present when revert bots calculated them off-chain.

The main issue here arises from the fact that liquidity of a position inside NonfungiblePositionManager can be manipulated by anyone. More specifically NonfungiblePositionManager::increaseLiquidity() can be called freely, which means that liquidity can be added to any NFT position without restriction.

This can be validated by looking at NonfungiblePositionManager::increaseLiquidity():

- https://github.com/Uniswap/v3-periphery/blob/697c2474757ea89fec12a4e6db16a574fe259610/contracts/NonfungiblePositionManager.sol#L198C14-L198C31
function increaseLiquidity ( IncreaseLiquidityParams calldata params ) external payable override //<---------- No `isAuthorizedForToken` modifier - anyone can call checkDeadline (params.deadline) returns ( uint128 liquidity, uint256 amount0, uint256 amount1 ) {... }....

function decreaseLiquidity ( DecreaseLiquidityParams calldata params ) external payable override isAuthorizedForToken (params.tokenId) // <------- Only position owner can call checkDeadline (params.deadline) returns ( uint256 amount0, uint256 amount1 ) {... } All of this allows any attacker to exploit the check at practically zero cost.

## Recommended mitigation steps

Consider removing the problematic check from both functions, since it can cause more harm than good in this particular scenario.

## Assessed type

DoS kalinbas (Revert) confirmed, but disagreed with severity and commented:

We agree with this finding and will remove the check. But this should be at max a medium risk as there is no direct loss of funds. It’s more of a DOS (which could be resolved by using flashbots for example).

b0g0 (warden) commented:

This finding includes the AutoExit.sol case where a DOS leads to a significantly more serious impact. Here are the arguments AutoExit.sol is documented to:

Lets a v3 position to be automatically removed (limit order) or swapped to the opposite token (stop loss order) when it reaches a certain tick.

Here are short definitions of the 2 operations from Investopedia:

Limit Order definition A limit order guarantees that an order is filled at or better than a specific price level. Limit orders can be used in conjunction with stop orders to prevent large downside losses.

Stop Loss definition A stop-loss is designed to limit an investor’s loss on a security position that makes an unfavorable move.

Both of those operations are very time-bound, especially the Stop Loss order, where the idea is that the position owner configures a threshold at which he should exit the market or else his position will sustain losses. In case the price (ticks) drop below that threshold, DOSing execution even for a shorter amount of time can seriously affect the position, especially if it is a big one and the market is very active and volatile (like in a bull run) - the longer the position does NOT exit the market, the greater the losses.

DOSing here will not cost much compared to how it can affect a position.

ronnyx2017 (judge) decreased severity to Medium and commented:

I think the attack in AutoExit is more convincing, although passing specific parameters in auto range might produce a similar effect. However, since this exploitation is based on MEV, we should assume that the parameters in the original tx are not edge. This report elaborates more thoroughly on the exploitation scenarios and impacts, giving me the confidence to mark it as Medium.

Note: For full discussion, see here.

V3Vault::maxWithdrawal incorrectly converts balance to assets Submitted by b0g0, also found by ktg and thank_you Severity: Medium The maxWithdrawal() function of V3Vault calculates the maximum amount of underlying tokens an account can withdraw based on the shares it owns.

The initial problem with maxWithdrawal() and V3Vault overall was that they were not implemented according to the specs of ERC-4626 standard as outlined in the original issue. In the case of maxWithdrawal() it did not consider the following part of the spec:

MUST factor in both global and user-specific limits, like if withdrawals are entirely disabled (even temporarily) it MUST return 0.

In order to remediate the issue and make the V3Vault ERC-4626 compliant, protocol devs prepared this PR, where maxWithdrawal() was refactored so that it includes the actual daily limit that is applied when withdrawing assets:

- https://github.com/revert-finance/lend/blob/audit/src/V3Vault.sol#L335-L347
function maxWithdraw ( address owner ) external view override returns ( uint256 ) { - (, uint256 lendExchangeRateX96 ) = _calculateGlobalInterest (); - return _convertToAssets ( balanceOf ( owner ), lendExchangeRateX96, Math.

Rounding.

Down ); + ( uint256 debtExchangeRateX96, uint256 lendExchangeRateX96 ) = _calculateGlobalInterest (); + uint256 ownerShareBalance = balanceOf ( owner ); + uint256 ownerAssetBalance = _convertToAssets ( ownerShareBalance, lendExchangeRateX96, Math.

Rounding.

Down ); + ( uint256 balance, ) = _getBalanceAndReserves ( debtExchangeRateX96, lendExchangeRateX96 ); + if ( balance > ownerAssetBalance ) { + return ownerAssetBalance; + } else { + return _convertToAssets ( balance, lendExchangeRateX96, Math.

Rounding.

Down ); + } } The problem with the new code is this part:

// @audit balance is already converted to assets ( uint256 balance, ) = _getBalanceAndReserves ( debtExchangeRateX96, lendExchangeRateX96 ); // @audit - converts to assets a second time } else { return _convertToAssets ( balance, lendExchangeRateX96, Math.

Rounding.

Down ); } If we take a look at _getBalanceAndReserves() we can see that the returned balance is already converted to assets:

- https://github.com/revert-finance/lend/blob/audit/src/V3Vault.sol#L1107-L1116
function _getBalanceAndReserves ( uint256 debtExchangeRateX96, uint256 lendExchangeRateX96 ) internal view returns ( uint256 balance, uint256 reserves ) { ---> balance = totalAssets (); uint256 debt = _convertToAssets ( debtSharesTotal, debtExchangeRateX96, Math.

Rounding.

Up ); uint256 lent = _convertToAssets ( totalSupply (), lendExchangeRateX96, Math.

Rounding.

Up ); reserves = balance + debt > lent ?

balance + debt - lent:

0; } This means that maxWithdraw() improperly converts balance a second time and will overinflate the result, especially when debtExchangeRateX96 is high.

## Impact

V3Vault::maxWithdraw() inflates the actual amount that can be withdrawn, which can impact badly protocols and contracts integrating with the vault. The possibility is quite real considering that maxWithdraw() is part of the official ERC-4626 which is very widely adopted.

## Recommended mitigation steps

Refactor V3Vault::maxWithdraw() so that it does not convert balance to assets a second time:

function maxWithdraw ( address owner ) external view override returns ( uint256 ) {....

if ( balance > ownerAssetBalance ) { return ownerAssetBalance; } else { - return _convertToAssets ( balance, lendExchangeRateX96, Math.

Rounding.

Down ); + return balance }

## Assessed type

Math kalinbas (Revert) confirmed Some functions don’t check if liquidity > 0 before calling decreaseLiquidity Submitted by ktg Severity: Medium

- https://github.com/revert-finance/lend/blob/audit/src/V3Vault.sol#L654-L658

## Impact

Users cannot just collect UniswapV3 fees alone.

Users cannot call leverageDown with fee alone.

## Recommended Mitigation

In function V3Vault.decreaseLiquidityAndCollect, the code should check if liquidity > 0, if not, decreaseLiquidity should not be called. This allow the user to collect fees.

## Assessed type

Invalid Validation kalinbas (Revert) confirmed, but disagreed with severity and commented:

There is no medium risk here, in my opinion. But yes, it is a good finding.

ktg (warden) commented:

In my opinion, this does qualify as medium risk because it forces the users to decrease their liquidity in order to collect their fees. In this issue, one of the main features of the protocol (that is allowing collecting fees alone, or allowing to use only fees for leverageDown ) is affected.

kalinbas (Revert) commented:

Yeah I agree, it is a main feature. But with V3Utils you can collect fees only when liquidity == 0. So it is actually possible to collect fees only. I keep my opinion this should not be a medium risk.

WITHDRAW_AND_COLLECT_AND_SWAP doesnt force them to swap.

ktg (warden) commented:

xYou’re totally right, I’m sorry I’m mistaken, WITHDRAW_AND_COLLECT_AND_SWAP doesnt force them to swap. Now, the only problem is leverageDown forces user to decrease liquidity but I understand if you keep your opinion.

ronnyx2017 (judge) commented:

I am more inclined to maintain the Medium severity. Although this issue does not cause any value leakage, the standalone fee collection is a key function, which meets the criteria for key functionality errors.

# [M-13] User might execute PositionToken of token set by previous token owner

- **Contest:** Revert Lend
- **Slug:** 2024-03-revert-lend
- **Finding ID:** M-13
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-revert-lend
- **Source snapshot:** competitions/2024-03-revert-lend/final_report.html

PositionToken of token set by previous token owner Submitted by cryptphi The AutoRange.configToken() function updates the state variable positionConfigs for a tokenId and callable by the owner of the tokenId. However, in the call to AutoRange.execute(), the state variable positionConfigs for the tokenId is used in setting the local variable PositionConfig memory config without any further check to ensure the config has been set by the current owner of the tokenId.

This in some way could affect the swapcall on the token position such that the protocol does not receive any incentive.

## Recommended Mitigation Steps

Using an enumerable set or additional mapping parameter to set the current owner in the positionConfigs state variable and an additional check in execute() to ensure the config was set by token owner.

kalinbas (Revert) acknowledged and commented:

Position config is not reset when transferring a position to someone else, but the operator approval / approvalforall is reset. So the position can’t be automated anymore, and if it is given approval, the revert UI will also let the user set the new config. So this is valid but not a problem.

ronnyx2017 (judge) commented:

Makes sense. I also believe that frontend security checks are unreliable, so I’m still maintaining it as an Medium.

# [M-14] V3Vault is not ERC-4626 compliant

- **Contest:** Revert Lend
- **Slug:** 2024-03-revert-lend
- **Finding ID:** M-14
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-revert-lend
- **Source snapshot:** competitions/2024-03-revert-lend/final_report.html

V3Vault is not ERC-4626 compliant Submitted by Limbooo, also found by falconhoof, btk, 14si2o_Flint, wangxx2026, Silvermist, Aymen0909, shaka, jnforja, crypticdefense, erosjohn, 0xspryon, y0ng0p3 ( 1, 2 ), 0xDemon, and alix40

- https://github.com/code-423n4/2024-03-revert-lend/blob/435b054f9ad2404173f36f0f74a5096c894b12b7/src/V3Vault.sol#L301-L309
- https://github.com/code-423n4/2024-03-revert-lend/blob/435b054f9ad2404173f36f0f74a5096c894b12b7/src/V3Vault.sol#L312-L320
- https://github.com/code-423n4/2024-03-revert-lend/blob/435b054f9ad2404173f36f0f74a5096c894b12b7/src/V3Vault.sol#L323-L326
- https://github.com/code-423n4/2024-03-revert-lend/blob/435b054f9ad2404173f36f0f74a5096c894b12b7/src/V3Vault.sol#L329-L331

## Impact

Protocols that try to integrate with Revert Lend, expecting V3Vault to be ERC-4626 compliant, will multiple issues that may damage Revert Lend’s brand and limit Revert Lend’s growth in the market.

## Recommended Mitigation Steps

To address the non-compliance issues, the following changes are recommended:

diff --git a/src/V3Vault.sol b/src/V3Vault.sol index 64141ec..a25cebd 100644 --- a/src/V3Vault.sol +++ b/src/V3Vault.sol @@ -304,7 +304,12 @@ contract V3Vault is ERC20, Multicall, Ownable, IVault, IERC721Receiver, IErrors if (value >= globalLendLimit) { return 0; } else { - return globalLendLimit - value; + uint256 maxGlobalDeposit = globalLendLimit - value; + if (maxGlobalDeposit > dailyLendIncreaseLimitLeft) { + return dailyLendIncreaseLimitLeft; + } else { + return maxGlobalDeposit; + } } @@ -315,19 +320,37 @@ contract V3Vault is ERC20, Multicall, Ownable, IVault, IERC721Receiver, IErrors if (value >= globalLendLimit) { return 0; } else { - return _convertToShares(globalLendLimit - value, lendExchangeRateX96, Math.Rounding.Down);

+ uint256 maxGlobalDeposit = globalLendLimit - value; + if (maxGlobalDeposit > dailyLendIncreaseLimitLeft) { + return _convertToShares(dailyLendIncreaseLimitLeft, lendExchangeRateX96, Math.Rounding.Down); + } else { + return _convertToShares(maxGlobalDeposit, lendExchangeRateX96, Math.Rounding.Down); + } } /// @inheritdoc IERC4626 function maxWithdraw(address owner) external view override returns (uint256) { - (, uint256 lendExchangeRateX96) = `_calculateGlobalInterest()`; - return _convertToAssets(balanceOf(owner), lendExchangeRateX96, Math.Rounding.Down); + uint256 ownerBalance = balanceOf(owner); + (uint256 debtExchangeRateX96, uint256 lendExchangeRateX96) = `_calculateGlobalInterest()`; + (, uint256 available, ) = _getAvailableBalance(debtExchangeRateX96, lendExchangeRateX96);

+ if (available > ownerBalance) { + return _convertToAssets(ownerBalance, lendExchangeRateX96, Math.Rounding.Down); + } else { + return _convertToAssets(available, lendExchangeRateX96, Math.Rounding.Down); + } } /// @inheritdoc IERC4626 function maxRedeem(address owner) external view override returns (uint256) { - return balanceOf(owner); + uint256 ownerBalance = balanceOf(owner); + (uint256 debtExchangeRateX96, uint256 lendExchangeRateX96) = `_calculateGlobalInterest()`; + (, uint256 available, ) = _getAvailableBalance(debtExchangeRateX96, lendExchangeRateX96); + if (available > ownerBalance) { + return ownerBalance; + } else { + return available; + } } The modified maxDeposit function now correctly calculates the maximum deposit amount by considering both the global lend limit and the daily lend increase limit. If the calculated maximum global deposit exceeds the daily lend increase limit, the function returns the daily lend increase limit to comply with ERC-4626 requirements.

Similarly, the modified maxMint ensures compliance with ERC-4626 by calculating the maximum mints amount for a given owner. It considers both the global lend limit and the daily lend increase limit as mentioned in maxDeposit.

The modified maxWithdraw function now correctly calculates the maximum withdrawal amount for a given owner. It ensures that the returned value does not exceed the available balance in the vault. If the available balance is greater than the owner’s balance, it returns the owner’s balance, otherwise it return the available balance to prevent potential reverts during withdrawal transactions. This adjustment aligns with ERC-4626 requirements by ensuring that withdrawals do not cause unexpected reverts and accurately reflect the available funds for withdrawal.

Similarly, the modified maxRedeem function ensures compliance with ERC-4626 by calculating the maximum redemption amount for a given owner. It considers both the owner’s balance and the available liquidity in the vault as mentioned in maxWithdraw.

kalinbas (Revert) confirmed Revert mitigated:

Fixed here.

Status:

Mitigation Confirmed. Full details in the report from thank_you.

# [M-15] Users’ newly created positions can be prematurely closed and removed from the vault directly after they are created

- **Contest:** Revert Lend
- **Slug:** 2024-03-revert-lend
- **Finding ID:** M-15
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-revert-lend
- **Source snapshot:** competitions/2024-03-revert-lend/final_report.html

Submitted by JCN, also found by 0xPhantom A user can create a position by calling the create function or by simply sending the NFT to the vault via safeTransferFrom. Both of those methods will trigger the V3Vault::onERC721Received function:

V3Vault::onERC721Received 446:

loans [ tokenId ] = Loan ( 0 ); 447:

448:

_addTokenToOwner ( owner, tokenId ); // @audit: NFT added to storage The loan for this position ( loans[tokenId] ) is instantiated with 0 debt and the NFT is added to storage:

V3Vault::_addTokenToOwner 1297:

function _addTokenToOwner ( address to, uint256 tokenId ) internal { 1298:

ownedTokensIndex [ tokenId ] = ownedTokens [ to ].

length; 1299:

ownedTokens [ to ].

push ( tokenId ); 1300:

tokenOwner [ tokenId ] = to; // @audit: to == user address 1301: } At this point, the user has only instantiated their position and has not borrowed against it, meaning the position’s debt is 0. Additionally, the V3Vault::_repay function does not require the amount to repay a position’s debt to be non-zero. Thus, the following will occur when a malicious actor repays the non-existent debt of the newly created position with 0 value:

V3Vault::_repay 954:

function _repay ( uint256 tokenId, uint256 amount, bool isShare, bytes memory permitData ) internal { 955: ( uint256 newDebtExchangeRateX96, uint256 newLendExchangeRateX96 ) = _updateGlobalInterest (); 956:

957:

Loan storage loan = loans [ tokenId ]; 958:

959:

uint256 currentShares = loan.

debtShares; // @audit: 0, newly instantiated 960:

961:

uint256 shares; 962:

uint256 assets; 963:

964:

if ( isShare ) { 965:

shares = amount; // @audit: amount == 0 966:

assets = _convertToAssets ( amount, newDebtExchangeRateX96, Math.

Rounding.

Up ); 967: } else { 968:

assets = amount; // @audit: amount == 0 969:

shares = _convertToShares ( amount, newDebtExchangeRateX96, Math.

Rounding.

Down ); 970: } 971:

972:

// fails if too much repayed 973:

if ( shares > currentShares ) { // @audit: 0 == 0 974:

revert RepayExceedsDebt (); 975: }...

990:

uint256 loanDebtShares = loan.

debtShares - shares; // @audit: null storage operations 991:

loan.

debtShares = loanDebtShares; 992:

debtSharesTotal -= shares;...

1001:

address owner = tokenOwner [ tokenId ]; // @audit: user's address 1002:

1003:

// if fully repayed 1004:

if ( currentShares == shares ) { // @audit: 0 == 0 1005:

_cleanupLoan ( tokenId, newDebtExchangeRateX96, newLendExchangeRateX96, owner ); // @audit: remove NFT from storage and send NFT back to user As we can see above, repaying an empty position with 0 amount will result in the protocol believing that the loan is being fully repaid (see line 1004). Therefore, the _cleanupLoan internal function will be invoked, which will remove the position from storage and send the NFT back to the user:

V3Vault::_cleanupLoan 1077:

function _cleanupLoan ( uint256 tokenId, uint256 debtExchangeRateX96, uint256 lendExchangeRateX96, address owner ) 1078:

internal 1080: { 1081:

_removeTokenFromOwner ( owner, tokenId ); // @audit: remove NFT from storage 1082:

_updateAndCheckCollateral ( tokenId, debtExchangeRateX96, lendExchangeRateX96, loans [ tokenId ].

debtShares, 0 ); // @audit: noop 1083:

delete loans [ tokenId ]; // @audit: noop 1084:

nonfungiblePositionManager.

safeTransferFrom ( address ( this ), owner, tokenId ); // @audit: transfer NFT back to user Since the user’s NFT is no longer in the vault, any attempts by the user to borrow against the prematurely removed position will result in a revert since the position is now non-existent.

## Impact

Users’ newly created positions can be prematurely removed from the vault before the users can borrow against that position. This would result in the user wasting gas as they attempt to borrow against a non-existence position and are then forced to re-create the position.

Note that sophisticated users are able to bypass this griefing attack by submitting the create and borrow calls in one transaction. However, seeing as there are explicit functions used to first create a position and then to borrow against it, average users can be consistently griefed when they follow this expected flow.

## Recommended Mitigation Steps

I would recommend validating the amount parameter in the repay function and reverting if amount == 0. Additionally, there can be an explicit reversion if the loan attempting to be repaid currently has 0 debt.

kalinbas (Revert) confirmed Revert mitigated:

Fixed here and here.

Status:

Mitigation Confirmed. Full details in reports from thank_you, b0g0 and ktg.

# [M-16] Repayments and liquidations can be forced to revert by an attacker that repays minuscule amount of shares

- **Contest:** Revert Lend
- **Slug:** 2024-03-revert-lend
- **Finding ID:** M-16
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-revert-lend
- **Source snapshot:** competitions/2024-03-revert-lend/final_report.html

Submitted by kfx, also found by kinda_very_good, zxriptor, CaeraDenoir, grearlake ( 1, 2 ), falconhoof, 0x175, JohnSmith, Giorgio, JecikPo, jnforja, SpicyMeatball, shaka, givn, Aymen0909, AMOW, atoko, Norah, alexander_orjustalex, JCN ( 1, 2 ), web3Tycoon, erosjohn, lanrebayode77, nmirchev8, 0xjuan, and 0xAlix2

- https://github.com/code-423n4/2024-03-revert-lend/blob/435b054f9ad2404173f36f0f74a5096c894b12b7/src/V3Vault.sol#L696-L698
- https://github.com/code-423n4/2024-03-revert-lend/blob/435b054f9ad2404173f36f0f74a5096c894b12b7/src/V3Vault.sol#L973-L975

## Impact

At the moment, both repay and liquidate calls will fail if the amount of shares that the transaction attempts to repay exceeds the outstanding debt shares of the position, with RepayExceedsDebt and DebtChanged errors respectively.

This enables an attacker to keep repaying very small amounts, such as 1 share, of the debt, causing user/liquidator transactions to fail.

The attack exposes risks for users who are close to the liquidation theshold from increasing their position’s health, and also from self-liquidating their positions once they’re already below the threshold.

## Recommended Mitigation Steps

Allow to attempt to repay an unlimited amount of shares. Send back to the user tokens that were not required for the full repayment.

## Assessed type

DoS kalinbas (Revert) confirmed ronnyx2017 (judge) commented:

The attack vector includes MEV as a necessary condition.

Revert mitigated:

Fixed here and here.

Status:

Mitigation Confirmed. Full details in reports from b0g0, thank_you and ktg.

# [M-17] AutoExit could receive a reward calculated from the entire position’s fund even if onlyFee is true in AutoExit.execute()

- **Contest:** Revert Lend
- **Slug:** 2024-03-revert-lend
- **Finding ID:** M-17
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-revert-lend
- **Source snapshot:** competitions/2024-03-revert-lend/final_report.html

AutoExit could receive a reward calculated from the entire position’s fund even if onlyFee is true in AutoExit.execute() Submitted by kennedy1030, also found by deepplus and KupiaSec

- https://github.com/code-423n4/2024-03-revert-lend/blob/main/src/automators/AutoExit.sol#L100-L214
- https://github.com/code-423n4/2024-03-revert-lend/blob/main/src/automators/Automator.sol#L193-L215

## Impact

The owner of the NFT could end up paying more rewards to AutoExit than anticipated when onlyFee is set to true.

## Recommended Mitigation Steps

In Automator._decreaseFullLiquidityAndCollect(), feeAmount0, feeAmount1 must only include the amount calculated from the feeGrowthInside of UniswapV3 position.

mariorz (Revert) acknowledged, but disagreed with severity and commented:

Don’t believe this should be a “high risk”.

Users calling decreaseLiquidity without calling collect is possible but non-standard and there is no real reason to do this.

If this ever happened by some edge case, the Operator is an approved role that would be incentivized to return the extra fees to the affected user.

There is no risk for other lenders or borrowers.

Risk for the affected LP is limited to <2% of the position value.

ronnyx2017 (judge) decreased severity to Medium and commented:

More like user error or a malicious operator.

# [M-18] Users cannot stop loss in AutoRange and AutoExit

- **Contest:** Revert Lend
- **Slug:** 2024-03-revert-lend
- **Finding ID:** M-18
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-revert-lend
- **Source snapshot:** competitions/2024-03-revert-lend/final_report.html

Submitted by ktg

- https://github.com/code-423n4/2024-03-revert-lend/blob/main/src/automators/Automator.sol#L151-L153
- https://github.com/code-423n4/2024-03-revert-lend/blob/main/src/automators/AutoExit.sol#L162-L169
- https://github.com/code-423n4/2024-03-revert-lend/blob/main/src/transformers/AutoRange.sol#L157-L164

## Impact

Users cannot stop loss in AutoRange and AutoExit, resulting in huge loss.

Users cannot withdraw their tokens and cut loss even when they choose no swap option.

## Recommended Mitigation Steps

I recommend skipping the swap operation if _hasMaxTWAPTickDifference returns false instead of reverting.

## Assessed type

Invalid Validation EV_om (lookout) commented:

Users have no control over the timing of Automator calls, operators do. If a user wants to quickly exit a position, they can repay.

This is an in-built protection mechanism.

ktg (warden) commented:

@EV_om, AutoExit will be called by a bot, users uses a bot to avoid manually exiting their UniswapV3 positions so I think the point If a user wants to quickly exit a position, they can repay.

is not related.

What I meant in this issue is that even if amountIn is set to 0 (which means no swap), the code still call _validateSwap and revert if the price fluctuation is too high. Therefore, if execute is called with amountIn = 0 (which clearly indicates no swap, just withdraw and exit) and the price fluctuation is too high, it will revert. In another word, the code check the condition and revert in situations where users/callers clearly want to neglect that condition.

About This is an in-built protection mechanism., I think this is indeed a built-in protection mechanism, but that mechanism is only for cases where swap is needed. In this issue, this mechanism is still activated even when users don’t want it and result in their losses.

EV_om (lookout) commented:

You’re right, repayment of positions is unrelated, my bad. What I should have said was “If a user wants to quickly exit a position, they can do so manually.” As for the no swap case, there is no loss being prevented as the position owner will remain exposed to the same price action.

Nevertheless, the unnecessary call to _validateSwap() and resulting reverting behavior is a good observation. The judge may want to consider this as QA despite the overinflated severity and invalid assumptions of the original submission.

ktg (warden) commented:

@EV_om, @ronnyx2017 - I think the statement If a user wants to quickly exit a position, they can do so manually.

is also unrelated here because the role of AutoExit is to help users automatically exit a position. You can’t expect users to look at market data and exit manually all the time; that’s why AutoExit/AutoRange exists.

The loss here is that AutoExit cannot help users to automatically exit a position because it reverts on a condition that clearly needed to be omitted due to amountIn =0 (no swap).

I think we all agree that the call to _validateSwap() is unnecessary here but I disagree that it’s just a QA because it can cost huge loss due to users (through AutoExit or AutoRange ) unable to exit or re arrange their position.

If users choose to set amountIn (or swapAmount ) = 0, then what they want is “I don’t want to swap, just exit/autorange my position”, but then AutoExit / AutoRange fails to do this.

ronnyx2017 (judge) decreased severity to Medium and commented:

Good catch! Unnecessary _validateSwap here has genuinely compromised certain functionalities of the protocol. Medium is justified, in my opinion. Looking forward to the sponsor’s perspective, @kalinbas.

kalinbas (Revert) confirmed and commented:

_validateSwap() is not necessary only if there is no swap, we are going to remove if in that case.

Revert mitigated:

Fixed here.

Status:

Mitigation Confirmed. Full details in reports from b0g0, thank_you and ktg.

# [M-19] V3Oracle susceptible to price manipulation

- **Contest:** Revert Lend
- **Slug:** 2024-03-revert-lend
- **Finding ID:** M-19
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-revert-lend
- **Source snapshot:** competitions/2024-03-revert-lend/final_report.html

V3Oracle susceptible to price manipulation Submitted by b0g0, also found by 0x175, 14si2o_Flint, kfx, Fitro, Giorgio, grearlake, 0xblackskull ( 1, 2 ), crypticdefense, Silvermist, 0xspryon, MohammedRizwan, y0ng0p3, 0xAlix2, MSaptarshi, boredpukar, and maxim371 V3Oracle::getValue() is used to calculate the value of a position. The value is the product of the oracle price * the amounts held in the position. Price manipulation is prevented by checking for differences between Chainlink oracle and Uniswap TWAP.

However, the amounts ( amount0 and amount1 ) of the tokens in the position are calculated based on the current pool price ( pool.spot0() ), which means they can be manipulated. Since the value of the total position is calculated from amount0 and amount1 it can be manipulated as well.

## Recommended Mitigation Steps

Consider calculating amount0 & amount1 based on the oracle price and not on the spot price taken from slot0(). This way the above exploit will be mitigated.

## Assessed type

Uniswap kalinbas (Revert) commented:

The check in _checkPoolPrice() does limit the price manipulation. The current pool price (it is NOT the TWAP price as you mention) is compared to the derived pool price of the Chainlink oracle prices.

The amounts may be slightly manipulated for wide range positions, and more heavily manipulated for tight range positions. But the protection with _checkPoolPrice() should be enough to protect this from being a problem. Also, repay() does not need any oracles.

ronnyx2017 (judge) decreased severity to Medium and commented:

I think it is reasonable to classify this issue as Medium.

Firstly, I think that the possibility of exploitation exists only within the _checkPoolPrice method.

Secondly, although it would require very fringe (and even incorrect) configuration parameters to really cause un-dusty losses, given that similar attacks have occurred on the mainnet with substantial losses (refer to the Gamma gDai TWAP verification range configuration error, even though it’s unrelated to slot0 ), I believe it’s worth adding necessary safeguards, especially for tight range positions and tokens.

kalinbas (Revert) confirmed and commented:

We agree to change it to use the oracle price for position token calculation.

Revert mitigated:

Fixed here.

Status:

Mitigation Confirmed. Full details in reports from thank_you, b0g0 and ktg.

# [M-20] Tokens can’t be removed as a collateral without breaking liquidations and other core functions

- **Contest:** Revert Lend
- **Slug:** 2024-03-revert-lend
- **Finding ID:** M-20
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-revert-lend
- **Source snapshot:** competitions/2024-03-revert-lend/final_report.html

Submitted by iamandreiski, also found by 0xAlix2

- https://github.com/code-423n4/2024-03-revert-lend/blob/435b054f9ad2404173f36f0f74a5096c894b12b7/src/V3Vault.sol#L856-L866
- https://github.com/code-423n4/2024-03-revert-lend/blob/435b054f9ad2404173f36f0f74a5096c894b12b7/src/V3Vault.sol#L1197-L1202
- https://github.com/code-423n4/2024-03-revert-lend/blob/435b054f9ad2404173f36f0f74a5096c894b12b7/src/V3Vault.sol#L1270-L1278
- https://github.com/code-423n4/2024-03-revert-lend/blob/435b054f9ad2404173f36f0f74a5096c894b12b7/src/V3Vault.sol#L702-L703
- https://github.com/code-423n4/2024-03-revert-lend/blob/435b054f9ad2404173f36f0f74a5096c894b12b7/src/V3Vault.sol#L1090-L1120

## Impact

The core mechanism that Revert utilizes to prevent arbitrary tokens to be utilized as collateral is by the default setting of the collateralFactor. Since the collateralFactor for all tokens (besides the ones approved for usage in the system, and set by admins) is 0, that means that no one can borrow against a collateral which hasn’t been approved.

The problem arises when admins would want a collateral removed from the system. There would be multiple reasons as to why this might be the case:

The underlying collateral has become too volatile.

The DAO in charge of Revert protocol decides to remove it as a collateral.

There has been some kind of a change in the mechanics of how that token operates and Revert wants it removed (e.g. upgradeable tokens, most-popular examples include USDC/USDT).

The only way in which this could be performed is to set the collateralFactor back to 0, but this would break core mechanics such as liquidations.

## Recommended Mitigation Steps

Don’t use the collateralFactor as the common denominator whether a token is accepted as collateral or not; use a method such as whitelisting tokens by address and performing necessary checks to see if the token address matches the whitelist.

## Assessed type

DoS kalinbas (Revert) confirmed, but disagreed with severity ronnyx2017 (judge) decreased severity to Medium and commented:

Valid DOS, but needs admin config upgrade. Since the modification of this config is reasonable and normal, it is classified as Medium.

Revert mitigated:

Fixed here.

Status:

Mitigation Confirmed. Full details in reports from b0g0, thank_you and ktg.

# [M-21] Dangerous use of deadline parameter

- **Contest:** Revert Lend
- **Slug:** 2024-03-revert-lend
- **Finding ID:** M-21
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-revert-lend
- **Source snapshot:** competitions/2024-03-revert-lend/final_report.html

Submitted by y0ng0p3, also found by 0xk3y, falconhoof, Mike_Bello90, Myd, th3l1ghtd3m0n, 0xspryon, and lightoasis

- https://github.com/code-423n4/2024-03-revert-lend/blob/435b054f9ad2404173f36f0f74a5096c894b12b7/src/transformers/AutoCompound.sol#L159-L172
- https://github.com/code-423n4/2024-03-revert-lend/blob/435b054f9ad2404173f36f0f74a5096c894b12b7/src/V3Vault.sol#L1032-L1074

## Vulnerability details

The protocol is using block.timestamp as the deadline argument while interacting with the Uniswap NFT Position Manager, which completely defeats the purpose of using a deadline.

Actions in the Uniswap NonfungiblePositionManager contract are protected by a deadline parameter to limit the execution of pending transactions. Functions that modify the liquidity of the pool check this parameter against the current block timestamp in order to discard expired actions.

These interactions with the Uniswap position are present throughout the code base, in particular and not only in the functions:

V3Utils::_swapAndMint, Automator::_decreaseFullLiquidityAndCollect, LeverageTransformer::leverageUp. Those functions call their corresponding functions in the Uniswap Position Manager, providing the deadline argument with their own deadline argument.

On the other hand, AutoCompound::execute and V3Vault::_sendPositionValue functions provide block.timestamp as the argument for the deadline parameter in their call to the corresponding underlying Uniswap NonfungiblePositionManager contract.

File:

src / transformers / AutoCompound.

sol // deposit liquidity into tokenId if ( state.

maxAddAmount0 > 0 || state.

maxAddAmount1 > 0 ) { _checkApprovals ( state.

token0, state.

token1 ); (, state.

compounded0, state.

compounded1 ) = nonfungiblePositionManager.

increaseLiquidity ( INonfungiblePositionManager.

IncreaseLiquidityParams ( @@-> params.

tokenId, state.

maxAddAmount0, state.

maxAddAmount1, 0, 0, block.

timestamp ) ); // fees are always calculated based on added amount (to incentivize optimal swap) state.

amount0Fees = state.

compounded0 * rewardX64 / Q64; state.

amount1Fees = state.

compounded1 * rewardX64 / Q64; } File:

src / V3Vault.

sol if ( liquidity > 0 ) { nonfungiblePositionManager.

decreaseLiquidity ( INonfungiblePositionManager.

DecreaseLiquidityParams ( tokenId, liquidity, 0, 0, block.

timestamp ) ); } Using block.timestamp as the deadline is effectively a no-operation that has no effect nor protection. Since block.timestamp will take the timestamp value when the transaction gets mined, the check will end up comparing block.timestamp against the same value (see here ).

## Impact

Failure to provide a proper deadline value enables pending transactions to be maliciously executed at a later point. Transactions that provide an insufficient amount of gas such that they are not mined within a reasonable amount of time, can be picked by malicious actors or MEV bots and executed later in detriment of the submitter. See this issue for an excellent reference on the topic (the author runs a MEV bot).

## Recommended Mitigation Steps

As done in the LeverageTransformer::leverageUp and V3Utils::_swapAndIncrease functions, add a deadline parameter to the AutoCompound::execute and V3Vault::_sendPositionValue functions and forward this parameter to the corresponding underlying call to the Uniswap NonfungiblePositionManager contract.

kalinbas (Revert) confirmed Revert mitigated:

PR here - added deadline where missing.

Status:

Mitigation Confirmed. Full details in reports from thank_you, b0g0 and ktg.

# [M-22] dailyDebtIncreaseLimitLeft is not updated in liquidate()

- **Contest:** Revert Lend
- **Slug:** 2024-03-revert-lend
- **Finding ID:** M-22
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-revert-lend
- **Source snapshot:** competitions/2024-03-revert-lend/final_report.html

dailyDebtIncreaseLimitLeft is not updated in liquidate() Submitted by lanrebayode77, also found by Aymen0909 and 0xAlix2 On days with a significant number of liquidated positions, particularly when the asset quantity is substantial, there will be an excess of assets available in the vault that cannot be borrowed; thereby, causing a drastic decrease in the utilization rate.

This also contradicts what was stated in the repay() function, which asserts that repaid amounts should be borrowed again. Liquidation is also a form of repayment:

// when amounts are repayed - they may be borrowed again dailyDebtIncreaseLimitLeft += assets;

## Recommended Mitigation Steps

Include dailyDebyIncreaseLimitLeft increment in liquidate().

dailyDebtIncreaseLimitLeft += state.

liquidatorCost;

## Assessed type

Context kalinbas (Revert) confirmed Revert mitigated:

Fixed here.

Status:

Mitigation Confirmed. Full details in reports from thank_you, b0g0 and ktg.

# [M-23] AutoRange execution can be front-ran to avoid protocol fee, causing loss for protocol

- **Contest:** Revert Lend
- **Slug:** 2024-03-revert-lend
- **Finding ID:** M-23
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-revert-lend
- **Source snapshot:** competitions/2024-03-revert-lend/final_report.html

AutoRange execution can be front-ran to avoid protocol fee, causing loss for protocol Submitted by 0xjuan When users configure their NFT within the AutoRange contract, they have 2 options for fee-handling:

Protocol takes 0.15% of the entire position size.

Protocol takes a higher fee of 2%, but only from the position’s collected fees.

The user sets PositionConfig.onlyFees=false for the first option, and onlyFees=true for the second option. When an operator calls the AutoRange.execute() function, they set the reward parameter rewardX64 based on the user’s PositionConfig.

However, the execution can be front-ran by the user. They can change the onlyFees boolean, which changes the fee handling logic, while the rewardX64 parameter set by the operator is unchanged.

The user can exploit this to their advantage by initially setting onlyFees to false, so that the operator will call the function with only 0.15% reward percentage. But when the operator sends their transaction, the user front-runs it by changing onlyFees to true. Now, the protocol only gets 0.15% of the fees collected when they initially intended to collect 0.15% of the entire position.

## Impact

The cost of executing the swap is likely to exceed the fees obtained (since expected fee is 0.15% of entire position, but only 0.15% of fees are obtained). This leads to loss of funds for the protocol.

Note: this has been submitted as only a medium severity issue since the protocol’s off-chain operator logic can simply blacklist such users once they have performed the exploit.

## Recommended Mitigation Steps

Let the operator pass in 2 different values for rewardX64, where each one corresponds to a different value of onlyFees. This way, the rewardX64 parameter passed in will not be inconsistent with the executed logic.

## Assessed type

MEV kalinbas (Revert) acknowledged and commented:

As you mentioned we are solving this with the bot off-chain; it is a valid finding.

# [M-24] Incorrect liquidation fee calculation during underwater liquidation, disincentivizing liquidators to participate

- **Contest:** Revert Lend
- **Slug:** 2024-03-revert-lend
- **Finding ID:** M-24
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-revert-lend
- **Source snapshot:** competitions/2024-03-revert-lend/final_report.html

Submitted by 0xjuan As stated in the Revert Lend Whitepaper, the liquidation fee for underwater positions is supposed to be 10% of the debt. However, the code within V3Vault::_calculateLiquidation (shown below) calculates the liquidation fee as 10% of the fullValue rather than 10% of the debt.

} else { // all position value liquidationValue = fullValue; uint256 penaltyValue = fullValue * ( Q32 - MAX_LIQUIDATION_PENALTY_X32 ) / Q32; liquidatorCost = penaltyValue; reserveCost = debt - penaltyValue; } Note:

fullValue * (Q32 - MAX_LIQUIDATION_PENALTY_X32) / Q32; is equivalent to fullValue * 90%.

The code snippet is here.

## Impact

As the fullValue decreases below debt (since the position is underwater), liquidators are less-and-less incentivised to liquidate the position. This is because as fullValue decreases, the liquidation fee (10% of fullValue ) also decreases.

This goes against the protocol’s intention (stated in the whitepaper) that the liquidation fee will be fixed at 10% of the debt for underwater positions, breaking core protocol functionality.

## Recommended Mitigation Steps

Ensure that the liquidation fee is equal to 10% of the debt. Make the following changes in V3Vault::_calculateLiquidation():

else { -// all position value -liquidationValue = fullValue; -uint256 penaltyValue = fullValue * (Q32 - MAX_LIQUIDATION_PENALTY_X32) / Q32; -liquidatorCost = penaltyValue; -reserveCost = debt - penaltyValue; +uint256 penalty = debt * (MAX_LIQUIDATION_PENALTY_X32) / Q32; //[10% of debt] +liquidatorCost = fullValue - penalty; +liquidationValue = fullValue; +reserveCost = debt - liquidatorCost; // Remaining to pay.

}

## Assessed type

Error kalinbas (Revert) confirmed, but disagreed with severity and commented:

Low severity.

ronnyx2017 (judge) decreased severity to Medium and commented:

According to the C4 rules, Medium is appropriate, as this disrupts certain designs in the economic model.

Revert mitigated:

PR here - fixed calculation.

Status:

Mitigation Confirmed. Full details in reports from thank_you, b0g0 and ktg.

# [M-25] Asymmetric calculation of price difference

- **Contest:** Revert Lend
- **Slug:** 2024-03-revert-lend
- **Finding ID:** M-25
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-revert-lend
- **Source snapshot:** competitions/2024-03-revert-lend/final_report.html

Submitted by t4sk, also found by Bauchibred, lanrebayode77, and hunter_w3b Price difference is calculated in 2 ways depending on whether price > verified price or not.

If price > verified price, this is the equation:

(price - verified price) / price Otherwise price is calculated with this equation:

(verified price - price) / verified price When the 2 equations above are graphed with price = horizontal axis, we get 2 different curves, see here.

The first equation produces a asymptotic curve (shown in red). The second equation produces a linear curve (shown in green). Therefore, the rate at which the price difference changes is different depending on whether price > verified price or not.

Example Price difference of +1 or -1 from verified price are not symmetric:

# p < v v = 2 p = 1 d = (v - p) / v print (d) # output is 0.5 # p > v v = 2 p = 3 d = (p - v) / p print (d) # output is 0.33333 Tools Used Desmos graphing calculator and python

## Recommended Mitigation Steps

Use a different equation to check price difference (shown in blue here ):

|price - verified price| / verified price <= max difference Assuming verifyPriceX96 > 0:

uint256 diff = priceX96 >= verifyPriceX96 ? ( priceX96 - verifyPriceX96 ) * 10000: ( verifyPriceX96 - priceX96 ) * 10000; require ( diff / verifyPriceX96 <= maxDifferenceX1000 )

## Assessed type

Math kalinbas (Revert) confirmed Revert mitigated:

PR here - fixed calculation.

Status:

Mitigation Confirmed. Full details in reports from ktg and b0g0.

# [M-26] Some ERC20 can revert on a zero value transfer

- **Contest:** Revert Lend
- **Slug:** 2024-03-revert-lend
- **Finding ID:** M-26
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-revert-lend
- **Source snapshot:** competitions/2024-03-revert-lend/final_report.html

Submitted by DadeKuma Note: This finding was reported via the winning

# [M-27] Missing L2 sequencer checks for Chainlink oracle

- **Contest:** Revert Lend
- **Slug:** 2024-03-revert-lend
- **Finding ID:** M-27
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-03-revert-lend
- **Source snapshot:** competitions/2024-03-revert-lend/final_report.html

Submitted by DadeKuma Note: This finding was reported via the winning

## Rejected Primary Findings

# Rejected Primary Findings: Revert Lend

No rejected primary findings were captured for this contest in the current corpus.

- **Rejected primary count:** 0
- **Primary submission rows captured:** 0
- **Submission status:** requires_authentication
