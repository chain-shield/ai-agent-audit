# Benchmark Ground Truth: Munchables

## Accepted H/M Findings

# Accepted H/M Findings: Munchables

# [H-01] Malicious User can call lockOnBehalf repeatedly extend a users unlockTime , removing their ability to withdraw previously locked tokens

- **Contest:** Munchables
- **Slug:** 2024-05-munchables
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-05-munchables
- **Source snapshot:** competitions/2024-05-munchables/final_report.html

lockOnBehalf repeatedly extend a users unlockTime, removing their ability to withdraw previously locked tokens Submitted by Circolors, also found by 0xHash, nnez, pamprikrumplikas, niser93, bctester, Kaysoft, araj, Dudex_2004, 0rpse, bigtone, MrPotatoMagic, Audinarey, joaovwfreire, Dots, PENGUN, ayden, brgltd, octeezy, 0xMosh, turvy_fuzz, oxchsyston, adam-idarrha, rouhsamad, 0xfox, King_, ilchovski, TheFabled, iamandreiski, AvantGard, m4ttm, Utsav, fandonov, SovaSlava, 0xloscar01, trachev ( 1, 2 ), zhaojohnson, cats ( 1, 2 ), 0x175 ( 1, 2, 3 ), aslanbek, Limbooo, yotov721, grearlake, carrotsmuggler ( 1, 2, 3 ), SpicyMeatball, 0xMax1mus, lanrebayode77, Walter, 0xdice91, Bigsam, dhank, 0xhacksmithh, DPS, jasonxiale, fyamf, biakia, Evo, crypticdefense ( 1, 2 ), twcctop, dd0x7e8, 4rdiii, 0xblack_bird, 0xAadi, tedox, Sabit, Drynooo, Varun_05, 0xrex, and merlinboii

## Impact

The protocol allows users to donate ether and/or tokens to another user via a lockOnBehalf function. This function lets the caller specify which address should be the recipient of these funds. However issues arise because the lockOnBehalf deposit resets the receivers lockedToken.unlockTime pushing the users unlock time for that token further back.

The code in question:

lockedToken.

unlockTime = uint32 ( block.

timestamp ) + uint32 ( _lockDuration ); Therefore if a user has already locked tokens in the protocol, a malicious user can repeatedly call lockOnBehalf shortly before the current unlockTime and keep delaying the users ability to withdraw their tokens. This is compounded by the fact that the lockOnBehalf function has no minimum _quantity therefore the attacker doesn’t have to give up any of their own tokens to acheive this.

Alternatively, this attack vector can also be used to deny a users ability to decrease their minimum lock duration. In this instance, the attacker would have to call lockOnBehalf with a non zero amount, meaning the following check in setLockDuration would cause a revert and stop the user from decreasing their lock duration:

if ( uint32 ( block.

timestamp ) + uint32 ( _duration ) < lockedTokens [ msg.

sender ][ tokenContract ].

unlockTime ) { revert LockDurationReducedError (); }

## Recommended Mitigation

Other users having the power to extend a user’s lock duration is inherently dangerous. The protocol should seriously consider whether the lockOnBehalf functionality is necessary outside of being used by the MigrationManager contract.

If it is decided that the team wishes for the project to retain the ability for users to “donate” tokens to other users there are a number approaches they can consider:

Have tokens locked via the lockOnBehalf function not alter the users token’s lockDuration, but consider how bypassing lockduration may be abused (such as lockdrop NFT minting period) Make locking on behalf of a user a two step process where after a user attempts to lockOnBehalf the recipient has the choice to accept/deny the lock before any changes are made to the state of the user’s currently locked tokens.

0xinsanity (Munchables) confirmed and commented via duplicate issue #115:

Fixed We locked the lockOnBehalf function to only be called by the MigrationManager.

0xsomeone (judge) commented:

The submission and its duplicates have demonstrated how a lack of access control in the LockManager::lockOnBehalfOf function can lead to infinite locks and can sabotage lock duration changes.

This submission has been selected as the best given that it outlines both risks inherent to the lack of access control clearly and concisely while providing recommendations that align with best practices.

Submissions awarded with a 75% reward (i.e. penalized by 25%) either propose a mitigation that is incorrect or outline the lock duration adjustment problem which is of lower severity.

Submissions awarded with a 25% reward (i.e. penalized by 75%) are QA-reported issues that were improperly submitted as low-risk.

# [H-02] Invalid validation allows users to unlock early

- **Contest:** Munchables
- **Slug:** 2024-05-munchables
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-05-munchables
- **Source snapshot:** competitions/2024-05-munchables/final_report.html

Submitted by SpicyMeatball, also found by n4nika, Varun_05, yashgoel72, dhank, Janio, EPSec, adam-idarrha, gavfu, th3l1ghtd3m0n, rouhsamad, 0xdice91, stakog, bigtone, Walter, Bigsam, LinKenji, Stefanov, MrPotatoMagic, swizz, joaovwfreire, steadyman, itsabinashb, gajiknownnothing, tedox, Eeyore, araj, Tychai0s, trachev, merlinboii, pfapostol, Limbooo, 0xMosh, Sabit, Utsav, 0rpse, mitko1111, Oxsadeeq, jasonxiale, crypticdefense, 0xhacksmithh, ayden, AvantGard, ahmedaghadi, prapandey031, snakeeaterr, fyamf, Audinarey, sandy, 0xmystery, xyz, Mahmud, 0xblack_bird, ke1caM, leegh, Dots, Myd, 0xleadwizard, turvy_fuzz, SovaSlava, c0pp3rscr3w3r, zhaojohnson, aslanbek, and carrotsmuggler Lines of code

- https://github.com/code-423n4/2024-05-munchables/blob/main/src/managers/LockManager.sol#L257-L258
- https://github.com/code-423n4/2024-05-munchables/blob/main/src/managers/LockManager.sol#L265-L267

## Impact

Invalid validation in the setLockDuration function allows users to greatly reduce their unlock time or even unlock instantly.

## Recommended Mitigation Steps

+ uint32 lastLockTime = lockedTokens[msg.sender][tokenContract] if ( + lastLockTime + uint32(_duration) < lockedTokens[msg.sender][tokenContract].unlockTime ) { revert LockDurationReducedError(); }

## Assessed type

Invalid Validation 0xinsanity (Munchables) confirmed and commented via duplicate issue #236:

Fixed 0xsomeone (judge) increased severity to High and commented:

The Warden outlines a misbehavior in the adjustment of a user’s default lock duration that will retroactively apply to all their pre-existing locks and may incorrectly reduce them due to an invalid security check.

The present mechanism effectively permits the lock time of an entry to be halved which is considered a significant vulnerability and one that would merit an H-risk rating.

This submission has been selected as the best as it properly clarifies that each re-adjustment of the lock time can halve the remaining lock time and that it can be repeatedly invoked to continuously reduce the lock time and exploit the system to a significant degree.

Duplicates that have been penalized by 25% (i.e. awarded 75%) have been done so due to recommending an improper mitigation that would reset locks and render the lastLockTime variable useless.

Note: For full discussion, see here.

Medium Risk Findings (4)

# [M-01] Missing disapproval check in LockManager.sol::approveUSDPrice allows simultaneous approval and disapproval of a price proposal

- **Contest:** Munchables
- **Slug:** 2024-05-munchables
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-05-munchables
- **Source snapshot:** competitions/2024-05-munchables/final_report.html

LockManager.sol::approveUSDPrice allows simultaneous approval and disapproval of a price proposal Submitted by robertodf99, also found by Bauchibred ( 1, 2 ), Rushkov_Boyan, EaglesSecurity, twcctop, araj, Mahmud, typicalHuman, Topmark, Bbash, adam-idarrha, Bigsam, crypticdefense, 0xAadi, 0xdice91, Dots, xyz, iamandreiski, leegh, John_Femi, dhank, joaovwfreire, Eeyore, RotiTelur, Tychai0s, unique, avoloder, 0xleadwizard, AgileJune, swizz, djanerch, Sabit, Utsav, Evo, prapandey031, Stormreckson, mitko1111, Beosin, brevis, dd0x7e8, trachev, pfapostol, aslanbek, Sentryx, falconhoof, carrotsmuggler, ZdravkoHr, ZanyBonzy, bigtone, Walter, EPSec, 0xhacksmithh, MrPotatoMagic, pamprikrumplikas, brgltd, 0xAkira, and merlinboii

## Impact

Due to the missing disapproval check, a price feed can both disapprove and subsequently approve a newly proposed price. Price feeds are intended to vote either for approval or disapproval, not both. Hence, this can be considered an unintended functionality.

## Recommended Mitigation Steps

Add a check to see if the price feed has already disapproved the price proposal. If so, revert with a custom error.

function approveUSDPrice( uint256 _price ) external onlyOneOfRoles( [ Role.PriceFeed_1, Role.PriceFeed_2, Role.PriceFeed_3, Role.PriceFeed_4, Role.PriceFeed_5 ] ) { if (usdUpdateProposal.proposer == address(0)) revert NoProposalError(); if (usdUpdateProposal.proposer == msg.sender) revert ProposerCannotApproveError(); if (usdUpdateProposal.approvals[msg.sender] == _usdProposalId) revert ProposalAlreadyApprovedError(); + if (usdUpdateProposal.disapprovals[msg.sender] == _usdProposalId) + revert ProposalAlreadyDisapprovedError(); if (usdUpdateProposal.proposedPrice != _price) revert ProposalPriceNotMatchedError(); usdUpdateProposal.approvals[msg.sender] = _usdProposalId; usdUpdateProposal.approvalsCount++;

if (usdUpdateProposal.approvalsCount >= APPROVE_THRESHOLD) { _execUSDPriceUpdate(); } emit ApprovedUSDPrice(msg.sender); } 0xinsanity (Munchables) confirmed and commented via duplicate issue #83:

Should be low-risk.

Fixed 0xsomeone (judge) confirmed and commented:

The Warden outlines a misbehavior in the LockManager code that permits an oracle to disapprove and then approve a particular price measurement. The current approval and disapproval thresholds are meant to indicate that no stalemate should be possible, as they are configured at 3 with the total price feed roles being 5.

In reality, this will not lead to a quorum discrepancy as the only action permitted is disapproval and then approval. As such, a state with both approval and disapproval being enabled is impossible as either function reaching a quorum will cause the price measurement to be processed.

There is still an interesting edge case whereby 2 for and 2 against votes will, under normal operations, result in the final role who has not cast their vote yet being the tie-breaker. In the current system, any of the individuals who voted against can place a for vote incorrectly regardless of what the final voter believes.

Even though the price voters are privileged roles, their multitude does permit them to exploit this issue before they are removed if they are deemed malicious by the administrator team of the Munchables system, and in such a scenario the damage will already have been done. As such, I consider this to be a medium-risk rating due to the combination of a privileged action albeit not entirely trusted with an observable but not significant impact on the voting process.

Selecting the best submission out of this duplicate group was very hard as multiple submissions clearly demonstrated the vulnerability and went into depth as to its ramifications. This submission was selected as the best due to being concise, offering a very short and sweet PoC, and outlining the full details needed to grasp the vulnerability.

To note, any submission awarded with a 25% pie (i.e. a 75% reduction) was submitted via a QA report and thus cannot be eligible for the full reward.

Note: For full discussion, see here.

# [M-02] When LockManager.lockOnBehalf is called from MigrationManager , the user’s reminder will be set to 0, resulting in fewer received MunchableNFTs

- **Contest:** Munchables
- **Slug:** 2024-05-munchables
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-05-munchables
- **Source snapshot:** competitions/2024-05-munchables/final_report.html

LockManager.lockOnBehalf is called from MigrationManager, the user’s reminder will be set to 0, resulting in fewer received MunchableNFTs Submitted by PetarTolev, also found by joaovwfreire, bearonbike, Drynooo, and MrPotatoMagic Lines of code

- https://github.com/code-423n4/2024-05-munchables/blob/57dff486c3cd905f21b330c2157fe23da2a4807d/src/managers/MigrationManager.sol#L264
- https://github.com/code-423n4/2024-05-munchables/blob/57dff486c3cd905f21b330c2157fe23da2a4807d/src/managers/MigrationManager.sol#L285
- https://github.com/code-423n4/2024-05-munchables/blob/57dff486c3cd905f21b330c2157fe23da2a4807d/src/managers/MigrationManager.sol#L335-L347
- https://github.com/code-423n4/2024-05-munchables/blob/57dff486c3cd905f21b330c2157fe23da2a4807d/src/managers/LockManager.sol#L293
- https://github.com/code-423n4/2024-05-munchables/blob/57dff486c3cd905f21b330c2157fe23da2a4807d/src/managers/LockManager.sol#L345-L379
LockManager.lockOnBehalf is called when the user has MunchableNFT which should be migrated.

The LockManager._lock function called by the lockOnBehalf handles the actual locking process. This function does several important steps:

It verifies that the account is registered and that there is sufficient allowance for the tokens to be locked.

It calculates the total amount to lock, including any remainder from previous locks.

It calculates the number of NFTs the user will receive based on the locked amount, only if msg.sender != MigrationManager It transfers the tokens to the contract.

It updates the locked amount and the remainder ( here is the issue, when called from the MigrationManager ) It sets the unlock time based on the lock duration.

The remainder is an important part of this process. It represents any leftover amount that wasn’t enough to mint an additional NFT. Normally, this remainder is added to the next lock amount, allowing users to efficiently utilize all their tokens over time.

However, when _lock is called from MigrationManager, it improperly resets the remainder to 0 (step 5). This happens because the function doesn’t differentiate whether it’s being called from the migration process or a standard lock request. As a result, users end up receiving fewer MunchableNFTs than they should because the small leftover amounts are discarded instead of being carried over to the next lock.

## Impact

The user’s funds won’t be used efficiently, resulting in fewer received MunchableNFTs. Also, the user won’t be able to unlock these unused funds and relock them to be used. This happens because the MigrationManager resets the unlock time, forcing the user to wait an extra lockDuration.

## Recommended Mitigation Steps

Do not reset the remainder storage variable when the _lock function is called from the MigrationManager. Instead, ensure the remainder is correctly accumulated and carried over, allowing users to receive the correct amount of MunchableNFTs based on their total locked amount. This can be done by adding a condition to check the caller and handle the remainder accordingly.

## Assessed type

Context 0xsomeone (judge) commented:

The Warden and its duplicates outline a behavior whereby the migration manager will overwrite the remainder of a lock entry even if it is within the lock drop period. This is invalid behavior and will result in unaccounted remainders that will not carry over to the next lock operation properly.

I believe a medium-severity rating is acceptable given that it affects how NFTs are awarded in the lock drop system and I have selected this submission as the best due to describing the vulnerability in great and accurate detail.

0xinsanity (Munchables) commented:

If its coming from the migration manager, we don’t want to count that quantity towards the remainder calculations since we are choosing to migrate over old NFTs, not mint new ones.

0xsomeone (judge) commented:

Thanks to everyone who contributed to this submission’s discussion including the Sponsor @0xinsanity, and the Wardens @Tomiwasa0, @PetarTolev, @niser93, @0jovi0, and @Ys-Prakash!

I will address multiple concerns raised in as much of a concise manner as possible.

Addressing the Sponsor @0xinsanity and discussing from a security auditor perspective, I would like to note that the vulnerability does not relate to the remainder that should have been assigned for a migrated NFT, but rather the remainder a user has accumulated with previous locks. If they have accumulated a non-zero remainder with previous locks and migrated afterward, they would lose that remainder. As such, I implore you to revisit this submission.

Addressing the Wardens’ concerns and discussing from a C4 perspective:

# [M-03] Players can gain more NFTs benefiting from that past remainder in subsequent locks

- **Contest:** Munchables
- **Slug:** 2024-05-munchables
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-05-munchables
- **Source snapshot:** competitions/2024-05-munchables/final_report.html

Submitted by merlinboii, also found by 0xhacksmithh, dhank, Utsav, Stormreckson, rouhsamad, sandy, AvantGard, araj, SovaSlava, Holipot, DPS, Audinarey, Haipls, Varun_05, shaflow2, and Limbooo Lines of code

- https://github.com/code-423n4/2024-05-munchables/blob/main/src/managers/LockManager.sol#L311-L398
- https://github.com/code-423n4/2024-05-munchables/blob/main/src/managers/LockManager.sol#L401-L427
Description The remaining locked tokens continue to accumulate after the player unlocks their tokens, whether partially or fully.

As a result, the player benefits by retrieving more NFTs when they lock their tokens again during the lock drop period or in the next lock drop period.

Location:

LockManager::_lock()L344 - L380 uint256 quantity = _quantity + lockedToken.

remainder; uint256 remainder; uint256 numberNFTs; uint32 _lockDuration = playerSettings [ _lockRecipient ].

lockDuration; // SNIPPED if ( lockdrop.

start <= uint32 ( block.

timestamp ) && lockdrop.

end >= uint32 ( block.

timestamp ) ) { if ( _lockDuration < lockdrop.

minLockDuration || _lockDuration > uint32 ( configStorage.

getUint ( StorageKey.

MaxLockDuration )) ) revert InvalidLockDurationError (); if ( msg.

sender != address ( migrationManager )) { // calculate number of nfts --> remainder = quantity % configuredToken.

nftCost; numberNFTs = ( quantity - remainder ) / configuredToken.

nftCost; if ( numberNFTs > type ( uint16 ).

max ) revert TooManyNFTsError (); // Tell nftOverlord that the player has new unopened Munchables nftOverlord.

addReveal ( _lockRecipient, uint16 ( numberNFTs )); } // SNIPPED -> lockedToken.

remainder = remainder; lockedToken.

quantity += _quantity; Location:

LockManager::unlock()L401 - L427 function unlock ( address _tokenContract, uint256 _quantity ) external notPaused nonReentrant { LockedToken storage lockedToken = lockedTokens [ msg.

sender ][ _tokenContract ]; if ( lockedToken.

quantity < _quantity ) revert InsufficientLockAmountError (); if ( lockedToken.

unlockTime > uint32 ( block.

timestamp )) revert TokenStillLockedError (); // force harvest to make sure that they get the schnibbles that they are entitled to accountManager.

forceHarvest ( msg.

sender ); lockedToken.

quantity -= _quantity; // send token if ( _tokenContract == address ( 0 )) { payable ( msg.

sender ).

transfer ( _quantity ); } else { IERC20 token = IERC20 ( _tokenContract ); token.

transfer ( msg.

sender, _quantity ); } emit Unlocked ( msg.

sender, _tokenContract, _quantity ); }

## Impact

During the lock drop period or in the subsequent lock drop period, the player retrieves more NFTs in proportion to the quantity locked. The remaining tokens from previous locks enhance the new locking quantity when relocked until the end of the lock period.

Moreover, it probably assumes that the lockDuration can either be equal or greater than the lock drop duration ( lockDuration >= lockdrop.start - lockdrop.end ), allowing players to reenter by each lock drop, or significantly less than the lock drop duration ( lockDuration < lockdrop.start - lockdrop.end ), enabling players to reenter during the same lockdrop.

Therefore, the likelihood of the issue is based on how offset players can reenter and take advantage of the remainders.

The following core functions take an effect of this issue:

_lock() lockOnBehalf() lock() unlock()

## Recommended Mitigation Steps

Manage the remaining tokens when the player unlocks locked tokens. Alternatively, use other methods to handling the total amount of NFTs and locked quantity during the lock drop.

## Assessed type

Math 0xinsanity (Munchables) confirmed and commented:

Fixed 0xsomeone (judge) commented:

The Warden outlines a situation whereby the remainder of an account will not be properly maintained when unlocking a lock, permitting them to effectively carry over remainders that should otherwise not be accounted for.

While the vulnerability is present, its actual risk level is medium as the amounts involved would be insignificant and it would also imply that a user fully vested a minimum lock, and took advantage of the carried-over remainder in a second lock in the same “lockdrop”.

I selected this submission as the best given that it outlines the issue using a consumable step-by-step case. I would like to note that the mitigation cited by the Sponsor above is insufficient and the mitigation followed in #41 is the correct one.

Note: For full discussion, see here.

# [M-04] User’s schnibbles rewards are not harvested in the setLockDuration function

- **Contest:** Munchables
- **Slug:** 2024-05-munchables
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-05-munchables
- **Source snapshot:** competitions/2024-05-munchables/final_report.html

setLockDuration function Submitted by SpicyMeatball Lines of code

- https://github.com/code-423n4/2024-05-munchables/blob/main/src/managers/BonusManager.sol#L136
- https://github.com/code-423n4/2024-05-munchables/blob/main/src/managers/AccountManager.sol#L363-L387
- https://github.com/code-423n4/2024-05-munchables/blob/main/src/managers/LockManager.sol#L249

## Impact

User rewards are not harvested during lock duration updates, leading to inconsistencies in accrued rewards between users.

## Recommended Mitigation Steps

Harvest user rewards before updating lock duration:

function setLockDuration(uint256 _duration) external notPaused { if (_duration > configStorage.getUint(StorageKey.MaxLockDuration)) revert MaximumLockDurationError(); + accountManager.forceHarvest(msg.sender); playerSettings[msg.sender].lockDuration = uint32(_duration); 0xinsanity (Munchables) commented:

Fixed 0xsomeone (judge) confirmed and commented:

The submission outlines a discrepancy in the rewards a user will acquire due to the update of an account’s lock duration, and thereby the update of all their existing locks, not claiming Schnibble rewards and thus not updating the relevant entries in the AccountManager.

The vulnerability effectively permits users to change their lock duration and acquire more rewards than they would normally receive as the update would retroactively apply to existingly-vested rewards. I believe a medium risk rating is fair as rewards would be distributed at a higher rate than expected and the problem can be capitalized maliciously.

0xinsanity (Munchables) confirmed

## Rejected Primary Findings

# Rejected Primary Findings: Munchables

# Use call instead of transfer

- **Contest:** Munchables
- **Slug:** 2024-05-munchables
- **Submission:** V-282
- **Submitter:** 0xleadwizard
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-munchables-validation/issues/282
- **Source snapshot:** competitions/2024-05-munchables/submissions/raw/V-282.md

## Brief Summary

Using call instead of transfer is better and less error-prone as there is gas limit of 2300 with transfer and no-check for success.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary

# PriceFeed can continually launch the wrong proposals, preventing correct proposals from being launched

- **Contest:** Munchables
- **Slug:** 2024-05-munchables
- **Submission:** V-49
- **Submitter:** Drynooo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-munchables-validation/issues/49
- **Source snapshot:** competitions/2024-05-munchables/submissions/raw/V-49.md

## Brief Summary

Token prices may be maliciously delayed, resulting in erroneous prices.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), edited-by-warden, :robot:_primary, :robot:_22_group

# Setting `DISAPPROVE_THRESHOLD` to 0 results in the inability to pass proposals even when `APPROVE_THRESHOLD` is reached.

- **Contest:** Munchables
- **Slug:** 2024-05-munchables
- **Submission:** V-163
- **Submitter:** Eeyore
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-munchables-validation/issues/163
- **Source snapshot:** competitions/2024-05-munchables/submissions/raw/V-163.md

## Brief Summary

In the `setUSDThresholds()` function, an Admin can update the `APPROVE_THRESHOLD` and `DISAPPROVE_THRESHOLD` storage values that determine the thresholds after which a proposal is approved or disapproved. Unfortunately, setting the `DISAPPROVE_THRESHOLD` to 0 has the effect that none of the proposals will pass, leading to a temporary denial of service (DoS) situation if all current PriceFeed roles vote in favor of the proposal by calling the `approveUSDPrice()` function. There is a check in the `_execUSDPriceUpdate()` internal function: This condition will always be false when `DISAPPROVE_THRESHOLD` is 0, preventing any proposal from being passed no matter the number of approval votes. The...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_24_group

# Inaccurate Calculation of Weighted Value When Lock Duration Expires but Tokens Remain Unlocked

- **Contest:** Munchables
- **Slug:** 2024-05-munchables
- **Submission:** V-126
- **Submitter:** atoko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-munchables-validation/issues/126
- **Source snapshot:** competitions/2024-05-munchables/submissions/raw/V-126.md

## Brief Summary

The current implementation inaccurately calculates the weighted value of locked tokens when the lock duration has expired but the user has not yet unlocked them. This discrepancy can lead to misleading representations of the user's locked token holdings, affecting decision-making processes based on the reported values. the `getLockedWeightedValue` function checks to ensure that the player quantity is > 0 as below the issue with that is before unlocking the tokens users who locked have a quantity > 0 even if the lock duration is passed.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_18_group

# Lack of Quantity Check in lock and lockOnBehalf Functions.

- **Contest:** Munchables
- **Slug:** 2024-05-munchables
- **Submission:** V-575
- **Submitter:** aua_oo7
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-munchables-validation/issues/575
- **Source snapshot:** competitions/2024-05-munchables/submissions/raw/V-575.md

## Brief Summary

Description:In the `_lock` function which is call by `lock` and `lockOnBehalf`, there's no explicit check to ensure that the `_quantity` parameter passed to the function is greater than zero all `lock` , `lockOnBehalf` and `_lock` doesn't check it . This means that if `_quantity` is mistakenly set to zero, it would still execute the lock operation without actually locking any tokens. If an attacker were to exploit this vulnerability by calling `lockOnBehalf` or `lock` with a `_quantity` of zero, they could effectively bypass the intended token locking mechanism. This could lead to loss of funds if users rely on this function to securely lock their tokens. **Mitigation Steps:** Add a check a...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), :robot:_primary, :robot:_44_group

# Token Owner tokens will be lost if the owner uses wrong OnBehalf Address

- **Contest:** Munchables
- **Slug:** 2024-05-munchables
- **Submission:** V-614
- **Submitter:** d4r3d3v1l
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-munchables-validation/issues/614
- **Source snapshot:** competitions/2024-05-munchables/submissions/raw/V-614.md

## Brief Summary

Loss of tokens for the Owner who called the `lockOnBehalf` function.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_16_group

# Incomplete mapping deletion in the disapproveUSDPrice function

- **Contest:** Munchables
- **Slug:** 2024-05-munchables
- **Submission:** V-33
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-munchables-validation/issues/33
- **Source snapshot:** competitions/2024-05-munchables/submissions/raw/V-33.md

## Brief Summary

Detailed description of the impact of this finding. Location: The function `LockManager.disapproveUSDPrice(uint256)` in `src/managers/LockManager.sol` (lines 210-242) deletes the `ILockManager.USDUpdateProposal` structure in `src/interfaces/ILockManager.sol` (lines 60-69), which contains a mapping: - Deletion occurs at `src/managers/LockManager.sol` line 238. Description: Deleting a structure that contains a mapping does not remove the mapping itself, as per the [Solidity documentation](https://solidity.readthedocs.io/en/latest/types.html##delete). Residual data from the mapping may pose a security risk and could potentially be exploited to compromise the contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), edited-by-warden, :robot:_primary, :robot:_41_group

# Use of payable.transfer() Instead of .call for sending ether

- **Contest:** Munchables
- **Slug:** 2024-05-munchables
- **Submission:** V-225
- **Submitter:** kartik_giri_47538
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-munchables-validation/issues/225
- **Source snapshot:** competitions/2024-05-munchables/submissions/raw/V-225.md

## Brief Summary

The contract uses payable.transfer() to send Ether, which has a hard gas limit of 2300 gas. This can cause the transaction to fail if the recipient's fallback or receive function requires more than 2300 gas, or if there is a change in gas costs in the Ethereum network. **Impact:** If the recipient has a complex fallback or receive function that requires more than 2300 gas than tx can revert.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_28_group

# Locking native tokens using a non-EOA account makes funds unrecoverable

- **Contest:** Munchables
- **Slug:** 2024-05-munchables
- **Submission:** V-527
- **Submitter:** niser93
- **Claimed severity:** High
- **Final severity:** High
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-munchables-validation/issues/527
- **Source snapshot:** competitions/2024-05-munchables/submissions/raw/V-527.md

## Brief Summary

The [lock() method](https://github.com/code-423n4/2024-05-munchables/blob/main/src/managers/LockManager.sol#L296-L309) allows to lock funds and mint Munchables NFT. The [lockOnBehalf() method](https://github.com/code-423n4/2024-05-munchables/blob/main/src/managers/LockManager.sol#L274-L294) allows to lock funds on behalf of a player. If a player uses a smart contract without the `receive/fallback` methods instead of an EOA to lock his/her native tokens or decides to lock on behalf of a smart contract address, instead of an EOA, he/she will not be able to unlock his/her funds after the unlockTime period. This means that his/her funds will be stuck indefinitely inside the LockManager contract...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), edited-by-warden, :robot:_primary, :robot:_01_group

# setLockDuration doesn't have unlockTimeLatest controls

- **Contest:** Munchables
- **Slug:** 2024-05-munchables
- **Submission:** V-425
- **Submitter:** tonisives
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** unknown
- **Rejection category:** unknown
- **Source URL:** https://github.com/code-423n4/2024-05-munchables-validation/issues/425
- **Source snapshot:** competitions/2024-05-munchables/submissions/raw/V-425.md

## Brief Summary

User's tokens can are frozen for extended periods of time without their possible knowledge or control.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), :robot:_primary, :robot:_53_group
