# Accepted H/M Findings: Thruster Invitational

# [M-01] Tickets can be entered after prizes for current round have partially been distributed

- **Contest:** Thruster Invitational
- **Slug:** 2024-02-thruster-invitational
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-thruster-invitational
- **Source snapshot:** competitions/2024-02-thruster-invitational/final_report.html

Submitted by EV_om, also found by 0xDING99YA ( 1, 2 ), oakcobalt, and rvierdiiev The ThrusterTreasure contract is designed to facilitate a lottery game where users can enter tickets to win prizes based on entropy. The contract includes mechanisms for entering tickets into rounds ( enterTickets() ), setting prizes for rounds ( setPrize() ), and claiming prizes ( claimPrizesForRound() ). A critical aspect of the game’s integrity is ensuring each ticket has an equal chance to win every prize.

However, there is a significant flaw in enterTickets(). The function checks if winning tickets for the prize index 0 have been set by verifying that winningTickets[currentRound_][0].length == 0. This check is intended to prevent users from entering tickets after prizes have begun to be distributed, but it does not account for prizes with higher indices that may already have been distributed. As a result, users can still enter tickets after some prizes have been distributed, but these late-entered tickets will not have a chance to win the already distributed prizes:

ThrusterTreasure.sol#L83-L96 function enterTickets ( uint256 _amount, bytes32 [] calldata _proof ) external {...

require ( winningTickets [ currentRound_ ][ 0 ].

length == 0, "ET" );...

}

## Recommended Mitigation Steps

Freeze ticket entry for the current round once any prize has been set.

jooleseth (Thruster) confirmed, but disagreed with severity and commented:

This is an issue reported in a few other Medium’s as well Rewards should be set atomically in the same transaction call by an admin script. The expectation is that if the 0 index is set, then all should be set for the round, hence the check for the zero index.

We also use this zero index check in the claimPrizesForRound call.

I would consider this a Quality Assurance to improve the require check, or Medium at most, as reported by issue 17.

0xleastwood (judge) decreased severity to Low/Non-Critical and commented:

It seems to me that users would have to intentionally be negligible and enter tickets into a round that they are not eligible for. Downgrading to QA.

EV_om (warden) commented:

@0xleastwood - all users are eligible for prizes as long as they have valid tickets. The user cannot know that prizes have already been set - even if they checked before entering their tickets, an owner call to setWinningTickets() may end up being included in a block before their transaction.

Setting rewards atomically via a script addresses the issue, but it is an OOS mitigation. Considering that setting them non-atomically in both ascending and descending order is problematic and that this was not documented, Medium severity seems reasonable.

0xleastwood (judge) increased severity to Medium and commented:

Understandably, this issue would not be mitigated by having all prizes set at the same time because any tickets entered prior would not be eligible for any reward. There needs to be some clear distinction at which a round ends and when prizes are distributed to prevent this from happening. Would typically class front-running issues as QA but this leads to users spending funds with no expected return.

# [M-02] claimPrizesForRound transfers the entire amount deposited for a prize regardless of the number of winners

- **Contest:** Thruster Invitational
- **Slug:** 2024-02-thruster-invitational
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-thruster-invitational
- **Source snapshot:** competitions/2024-02-thruster-invitational/final_report.html

claimPrizesForRound transfers the entire amount deposited for a prize regardless of the number of winners Submitted by EV_om, also found by 0xDING99YA and oakcobalt claimPrizesForRound() transfers the entire amount of a prize to a winner without considering the total number of winners for that prize.

The prize for a given round and prize index can be set by calling the setPrize() function, which pulls the amounts from the caller (the owner) and stores the prize data in the prizes array:

ThrusterTreasure.sol#L163-L184 function setPrize ( uint256 _round, uint64 _prizeIndex, uint256 _amountWETH, uint256 _amountUSDB, uint64 _numWinners ) external onlyOwner { require ( _round >= currentRound, "ICR" ); require ( _prizeIndex < maxPrizeCount, "IPC" ); depositPrize ( msg.

sender, _amountWETH, _amountUSDB ); prizes [ _round ][ _prizeIndex ] = Prize ( _amountWETH, _amountUSDB, _numWinners, _prizeIndex, uint64 ( _round )); } function depositPrize ( address _from, uint256 _amountWETH, uint256 _amountUSDB ) internal { WETH.

transferFrom ( _from, address ( this ), _amountWETH ); USDB.

transferFrom ( _from, address ( this ), _amountUSDB ); emit DepositedPrizes ( _amountWETH, _amountUSDB ); } However, the claimPrizesForRound() function always transfers the full prize amounts to the first caller, regardless of the number of winners for the prize. Once the prize for a specific index is claimed, other winners of that prize cannot claim their share (or winners of other prizes may end up not being able to claim theirs), effectively being denied their winnings:

ThrusterTreasure.sol#L102-L134 function claimPrizesForRound ( uint256 roundToClaim ) external {...

for ( uint256 i = 0; i < maxPrizeCount_; i ++) { Prize memory prize = prizes [ roundToClaim ][ i ]; uint256 [] memory winningTicketsRoundPrize = winningTickets [ roundToClaim ][ i ]; for ( uint256 j = 0; j < winningTicketsRoundPrize.

length; j ++) { uint256 winningTicket = winningTicketsRoundPrize [ j ]; if ( round.

ticketStart <= winningTicket && round.

ticketEnd > winningTicket ) { _claimPrize ( prize, msg.

sender, winningTicket ); }...

} function _claimPrize ( Prize memory _prize, address _receiver, uint256 _winningTicket ) internal { uint256 amountETH = _prize.

amountWETH; uint256 amountUSDB = _prize.

amountUSDB; WETH.

transfer ( _receiver, amountETH ); USDB.

transfer ( _receiver, amountUSDB ); emit ClaimedPrize ( _receiver, _prize.

round, _prize.

prizeIndex, amountETH, amountUSDB, _winningTicket ); } This approach can lead to scenarios where the amount available to be distributed among prize winners is less than that represented by the prizes stored in the prizes array.

This is considered medium severity because:

claimPrizesForRound() will revert if there aren’t enough funds to transfer the prize to the winner, altering the user the owner can mitigate this by simply “refilling” the prize as many times as needed

## Recommended Mitigation Steps

It is unclear whether the amounts passed to setPrize() are meant to be distributed among all winners of the given prize or to be paid out to each winner, but the cleaner approach would be the latter. In that case, the amount pulled from the owner can simply be scaled by the number of winners:

function depositPrize ( address _from, uint64 _numWinners, uint256 _amountWETH, uint256 _amountUSDB ) internal { WETH.

transferFrom ( _from, address ( this ), _amountWETH * _numWinners ); USDB.

transferFrom ( _from, address ( this ), _amountUSDB * _numWinners ); emit DepositedPrizes ( _numWinners, _amountWETH, _amountUSDB ); } jooleseth (Thruster) confirmed and commented:

I agree with the Warden’s evaluation.

# [M-03] Dynamic modification of maxPrizeCount affects prize claims

- **Contest:** Thruster Invitational
- **Slug:** 2024-02-thruster-invitational
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-thruster-invitational
- **Source snapshot:** competitions/2024-02-thruster-invitational/final_report.html

maxPrizeCount affects prize claims Submitted by EV_om, also found by 0xDING99YA and rvierdiiev The ThrusterTreasure contract is designed to manage rounds of a lottery game, where participants can enter tickets and claim prizes based on random draws. The contract includes a variable maxPrizeCount which dictates the maximum number of prizes that can be set for any given round. This variable can be modified by the contract owner at any time through the setMaxPrizeCount(uint256 _maxPrizeCount) function:

ThrusterTreasure.sol#L139-L142 function setMaxPrizeCount ( uint256 _maxPrizeCount ) external onlyOwner { maxPrizeCount = _maxPrizeCount; emit SetMaxPrizeCount ( _maxPrizeCount ); } The issue arises when maxPrizeCount is decreased after prizes for a round have been set but before they have been claimed. Since the claimPrizesForRound(uint256 roundToClaim) function iterates over prize indices up to maxPrizeCount, reducing this count means that winners of prizes with indices higher than the new maxPrizeCount will be unable to claim their winnings:

ThrusterTreasure.sol#L102-L120 function claimPrizesForRound ( uint256 roundToClaim ) external {...

uint256 maxPrizeCount_ = maxPrizeCount; for ( uint256 i = 0; i < maxPrizeCount_; i ++) { [ claim prize ] } entered [ msg.

sender ][ roundToClaim ] = Round ( 0, 0, roundToClaim ); // Clear user's tickets for the round emit CheckedPrizesForRound ( msg.

sender, roundToClaim ); } This could lead to a scenario where legitimate winners are denied their prizes due to a change in contract state that is unrelated to the rules of the game or their actions. Moreover, since calling claimPrizesForRound() clears the user’s entries for the round, reverting maxPrizeCount to its previous state does not allow them to claim the remaining tickets. This means they will effectively never be able to claim their prize.

## Recommended Mitigation Steps

To address this issue, implementing a checkpoint pattern for the maxPrizeCount variable is suggested. This method involves tracking changes to maxPrizeCount with checkpoints that record the value and the round number when the change occurs.

A possible implementation could look like this:

// Add a struct to store checkpoints for maxPrizeCount changes struct MaxPrizeCountCheckpoint { uint256 round; uint256 maxPrizeCount; } // Use an array to keep track of all checkpoints MaxPrizeCountCheckpoint [] public maxPrizeCountCheckpoints; constructor (...

) Ownable ( msg.

sender ) { maxPrizeCountCheckpoints.

push ( MaxPrizeCountCheckpoint ( 0, _maxPrizeCount ) );...

} // Modify setMaxPrizeCount to push a new checkpoint to the array function setMaxPrizeCount ( uint256 _maxPrizeCount ) external onlyOwner { require ( _maxPrizeCount != getMaxPrizeCountForRound ( currentRound ), "same value" ) maxPrizeCountCheckpoints.

push ( MaxPrizeCountCheckpoint ( currentRound, _maxPrizeCount ) ); emit SetMaxPrizeCount ( _maxPrizeCount ); } // Helper function to get the maxPrizeCount for a given round // Assumes more recent rounds will be queried more often function getMaxPrizeCountForRound ( uint256 _round ) public view returns ( uint256 ) { uint256 length = maxPrizeCountCheckpoints.

length; for ( uint256 i = length; i > 0; i --) { MaxPrizeCountCheckpoint storage checkpoint = maxPrizeCountCheckpoints [ i - 1 ]; if ( checkpoint.

round <= _round ) { return checkpoint.

maxPrizeCount; } return 0; } // Disallow setting prizes for future rounds since the maxPrizeCount could change function setPrize ( uint64 _prizeIndex, uint256 _amountWETH, uint256 _amountUSDB, uint64 _numWinners ) external onlyOwner { uint256 maxPrizeCount = getMaxPrizeCountForRound ( currentRound ); require ( _prizeIndex < maxPrizeCount, "IPC" );...

} function claimPrizesForRound ( uint256 roundToClaim ) external { uint256 maxPrizeCount = getMaxPrizeCountForRound ( roundToClaim );...

} This change ensures that each round’s prize structure is fixed upon the round’s creation, preventing post-hoc alterations that could negatively impact participants. Note that this implementation still requires attention is paid to not calling setMaxPrizeCount() for a given round if prizes have already been set for higher indices.

jooleseth (Thruster) commented:

I would consider this QA to ensure to add a require check that maxPrizeCount cannot be decreased as that is the intention.

EV_om (warden) commented:

That’s indeed a much simpler fix!

I would still say Medium severity is appropriate seeing as that could not have been inferred from the audit scope and given the potential impact, but either way I’ll accept the judge’s decision.

jooleseth (Thruster) confirmed

# [M-04] Incorrect gas claiming logic in ThrusterPoolDeployer

- **Contest:** Thruster Invitational
- **Slug:** 2024-02-thruster-invitational
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-thruster-invitational
- **Source snapshot:** competitions/2024-02-thruster-invitational/final_report.html

ThrusterPoolDeployer Submitted by EV_om, also found by oakcobalt and rvierdiiev From the audit documentation:

All contracts that use gas should comply with the Blast gas claim logic.

However, the ThrusterPoolDeployer contains a flaw in the implementation of claimGas() which will prevent it from ever claiming the gas it induces. The function attempts to claim gas for the zero address ( address(0) ) instead of the deployer’s own address ( address(this) ):

function claimGas ( address _recipient ) external override onlyFactory returns ( uint256 amount ) { amount = IBlast ( BLAST ).

claimMaxGas ( address ( 0 ), _recipient ); } This misconfiguration prevents the ThrusterPoolDeployer from reclaiming any gas, as the IBlast.claimMaxGas() call will always fail when provided with the zero address.

## Recommended Mitigation Steps

Use address(this) rather than 0.

jooleseth (Thruster) confirmed and commented:

I agree, we caught this issue a few days ago too. Blast had initially in their docs specified it should be address(0) instead of address(this) for gas claiming and we missed changing this in the audit commit freeze. Will agree to a Medium Risk bug for this, as it doesn’t affect any user funds.

# [M-05] ThrusterFactory.setYieldCut should claim fees for all pools before

- **Contest:** Thruster Invitational
- **Slug:** 2024-02-thruster-invitational
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-thruster-invitational
- **Source snapshot:** competitions/2024-02-thruster-invitational/final_report.html

ThrusterFactory.setYieldCut should claim fees for all pools before Submitted by rvierdiiev Thruster have introduced ability to change yield cut for their uniswap v2 like pools.

The yield is charged only, when some LP manages their position.

This means that in case if yield cut will be changed for a pool, then protocol fee should be minted for previous period using old yield cut, otherwise the yield cut will be incorrect, especially this is important for pools where liquidity is not added/removed often.

## Impact

Yield can be collected with wrong proportion.

Tools Used VsCode

## Recommended Mitigation Steps

I guess protocol just needs to acknowledge the issue as it will be not possible (not worthy) to implement such mechanism.

jooleseth (Thruster) acknowledged and commented:

We acknowledge that this situation is possible, but the effects and consequences of this are very minimal in reality as LPs update often and LPs are made aware of changes to fees with sufficient time.
