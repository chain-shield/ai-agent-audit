# Benchmark Ground Truth: AI Arena

## Accepted H/M Findings

# Accepted H/M Findings: AI Arena

# [H-01] A locked fighter can be transferred; leads to game server unable to commit transactions, and unstoppable fighters

- **Contest:** AI Arena
- **Slug:** 2024-02-ai-arena
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-02-ai-arena
- **Source snapshot:** competitions/2024-02-ai-arena/final_report.html

Submitted by CodeWasp, also found by adam-idarrha, d3e4, vnavascues, c0pp3rscr3w3r, shaka, GhK3Ndf, alexxander, Fulum, thank_you, denzi_, Limbooo, korok, evmboi32, juancito, grearlake, ktg, radev_sw, rouhsamad, KmanOfficial, MidgarAudits, KupiaSec, 0xaghas, Bauchibred, jaydhales, immeas, MrPotatoMagic, soliditywala, sashik_eth, 0xLogos, Tendency, 0x13, PedroZurdo, al88nsk, ladboy233 ( 1, 2 ), merlinboii, DanielArmstrong, hulkvision, israeladelaja, Draiakoo, ADM, petro_1912, nuthan2x, 0xCiphky, peanuts, cats, 0xAsen, stackachu, deadrxsezzz, Krace, jnforja, 0xvj, _eperezok, pkqs90, Josh4324, web3pwn, Aamir, btk, pynschon ( 1, 2 ), 0xE1,

ubl4nk, fnanni, devblixt, cartlex_, erosjohn, BARW, dimulski, zhaojohnson, blutorque, Kalogerone, DMoore, jesjupyter, 0xlemon, klau5, 0xAlix2, alexzoid, novamanbg, sobieski, xchen1130, matejdb, tallo, Pelz, oualidpro, aslanbek, and 0xlyov FighterFarm contract implements restrictions on the transferability of fighters in functions transferFrom() and safeTransferFrom(), via the call to function _ableToTransfer(). Unfortunately this approach doesn’t cover all possible ways to transfer a fighter: The FighterFarm contract inherits from OpenZeppelin’s ERC721 contract, which includes the public function safeTransferFrom(…, data), i.e. the same as safeTransferFrom() but with the additional

data parameter. This inherited function becomes available in the GameItems contract, and calling it allows us to circumvent the transferability restriction. As a result, a player will be able to transfer any of their fighters, irrespective of whether they are locked or not. Violation of such a basic system invariant leads to various kinds of impacts, including:

The game server won’t be able to commit some transactions; The transferred fighter becomes unstoppable (a transaction in which it loses can’t be committed); The transferred fighter may be used as a “poison pill” to spoil another player, and prevent it from leaving the losing zone (a transaction in which it wins can’t be committed).

Both of the last two impacts include the inability of the game server to commit certain transactions, so we illustrate both of the last two with PoCs, thus illustrating the first one as well.

Impact 1: A fighter becomes unstoppable, game server unable to commit If a fighter wins a battle, points are added to accumulatedPointsPerAddress mapping. When a fighter loses a battle, the reverse happens:

points are subtracted. If a fighter is transferred after it wins the battle to another address, accumulatedPointsPerAddress for the new address is empty, and thus the points can’t be subtracted:

the game server transaction will be reverted. By transferring the fighter to a new address after each battle, the fighter becomes unstoppable, as its accumulated points will only grow, and will never decrease.

Impact 2: Another fighter can’t win, game server unable to commit If a fighter loses a battle, funds are transferred from the amount at stake, to the stake-at risk, which is reflected in the amountLost mapping of StakeAtRisk contract. If the fighter with stake-at-risk is transferred to another player, the invariant that amountLost reflects the lost amount per address is violated: after the transfer the second player has more stake-at-risk than before. A particular way to exploit this violation is demonstrated below: the transferred fighter may win a battle, which leads to reducing amountLost by the corresponding amount. Upon subsequent wins of the second player own fighters, this operation will underflow, leading to

the game server unable to commit transactions, and the player unable to exit the losing zone. This effectively makes a fighter with the stake-at-risk a “poison pill”.

Impact 1: A fighter becomes unstoppable, game server unable to commit diff --git a/test/RankedBattle.t.sol b/test/RankedBattle.t.sol index 6c5a1d7..dfaaad4 100644 --- a/test/RankedBattle.t.sol +++ b/test/RankedBattle.t.sol @@ -465,6 +465,31 @@ contract RankedBattleTest is Test { assertEq(unclaimedNRN, 5000 * 10 ** 18); } + /// @notice An exploit demonstrating that it's possible to transfer a staked fighter, and make it immortal!

+ function testExploitTransferStakedFighterAndPlay() public { + address player = vm.addr(3); + address otherPlayer = vm.addr(4); + _mintFromMergingPool(player); + uint8 tokenId = 0; + _fundUserWith4kNeuronByTreasury(player); + vm.prank(player); + _rankedBattleContract.stakeNRN(1 * 10 ** 18, tokenId); + // The fighter wins one battle + vm.prank(address(_GAME_SERVER_ADDRESS)); + _rankedBattleContract.updateBattleRecord(tokenId, 0, 0, 1500, true); + // The player transfers the fighter to other player + vm.prank(address(player)); + _fighterFarmContract.safeTransferFrom(player, otherPlayer, tokenId, ""); + assertEq(_fighterFarmContract.ownerOf(tokenId), otherPlayer); + // The fighter can't lose + vm.prank(address(_GAME_SERVER_ADDRESS));

+ vm.expectRevert(); + _rankedBattleContract.updateBattleRecord(tokenId, 0, 2, 1500, true); + // The fighter can only win: it's unstoppable!

+ vm.prank(address(_GAME_SERVER_ADDRESS)); + _rankedBattleContract.updateBattleRecord(tokenId, 0, 0, 1500, true); + } + /*////////////////////////////////////////////////////////////// HELPERS //////////////////////////////////////////////////////////////*/ Impact 2: Another fighter can’t win, game server unable to commit diff --git a/test/RankedBattle.t.sol b/test/RankedBattle.t.sol index 6c5a1d7..196e3a0 100644 --- a/test/RankedBattle.t.sol +++ b/test/RankedBattle.t.sol @@ -465,6 +465,62 @@ contract RankedBattleTest is Test { assertEq(unclaimedNRN, 5000 * 10 ** 18); } +/// @notice Prepare two players and two fighters +function preparePlayersAndFighters() public returns (address, address, uint8, uint8) {

+ address player1 = vm.addr(3); + _mintFromMergingPool(player1); + uint8 fighter1 = 0; + _fundUserWith4kNeuronByTreasury(player1); + address player2 = vm.addr(4); + _mintFromMergingPool(player2); + uint8 fighter2 = 1; + _fundUserWith4kNeuronByTreasury(player2); + return (player1, player2, fighter1, fighter2); +} + +/// @notice An exploit demonstrating that it's possible to transfer a fighter with funds at stake +/// @notice After transferring the fighter, it wins the battle, +/// @notice and the second player can't exit from the stake-at-risk zone anymore.

+function testExploitTransferStakeAtRiskFighterAndSpoilOtherPlayer() public { + (address player1, address player2, uint8 fighter1, uint8 fighter2) = + preparePlayersAndFighters(); + vm.prank(player1); + _rankedBattleContract.stakeNRN(1_000 * 10 **18, fighter1); + vm.prank(player2); + _rankedBattleContract.stakeNRN(1_000 * 10 **18, fighter2); + // Fighter1 loses a battle + vm.prank(address(_GAME_SERVER_ADDRESS)); + _rankedBattleContract.updateBattleRecord(fighter1, 0, 2, 1500, true); + assertEq(_rankedBattleContract.amountStaked(fighter1), 999 * 10 ** 18); + // Fighter2 loses a battle + vm.prank(address(_GAME_SERVER_ADDRESS)); + _rankedBattleContract.updateBattleRecord(fighter2, 0, 2, 1500, true);

+ assertEq(_rankedBattleContract.amountStaked(fighter2), 999 * 10 ** 18); + + // On the game server, player1 initiates a battle with fighter1, + // then unstakes all remaining stake from fighter1, and transfers it + vm.prank(address(player1)); + _rankedBattleContract.unstakeNRN(999 * 10 ** 18, fighter1); + vm.prank(address(player1)); + _fighterFarmContract.safeTransferFrom(player1, player2, fighter1, ""); + assertEq(_fighterFarmContract.ownerOf(fighter1), player2); + // Fighter1 wins a battle, and part of its stake-at-risk is derisked.

+ vm.prank(address(_GAME_SERVER_ADDRESS)); + _rankedBattleContract.updateBattleRecord(fighter1, 0, 0, 1500, true); + assertEq(_rankedBattleContract.amountStaked(fighter1), 1 * 10 ** 15); + // Fighter2 wins a battle, but the records can't be updated, due to underflow!

+ vm.expectRevert(); + vm.prank(address(_GAME_SERVER_ADDRESS)); + _rankedBattleContract.updateBattleRecord(fighter2, 0, 0, 1500, true); + // Fighter2 can't ever exit from the losing zone in this round, but can lose battles + vm.prank(address(_GAME_SERVER_ADDRESS)); + _rankedBattleContract.updateBattleRecord(fighter2, 0, 2, 1500, true); + (uint32 wins, uint32 ties, uint32 losses) = _rankedBattleContract.getBattleRecord(fighter2); + assertEq(wins, 0); + assertEq(ties, 0); + assertEq(losses, 2); +} + /*////////////////////////////////////////////////////////////// HELPERS //////////////////////////////////////////////////////////////*/

## Recommended Mitigation Steps

Remove the incomplete checks in the inherited functions transferFrom() and safeTransferFrom() of FighterFarm contract, and instead to enforce the transferability restriction via the _beforeTokenTransfer() hook, which applies equally to all token transfers, as illustrated below.

diff --git a/src/FighterFarm.sol b/src/FighterFarm.sol index 06ee3e6..9f9ac54 100644 --- a/src/FighterFarm.sol +++ b/src/FighterFarm.sol @@ -330,40 +330,6 @@ contract FighterFarm is ERC721, ERC721Enumerable { ); } - /// @notice Transfer NFT ownership from one address to another.

- /// @dev Add a custom check for an ability to transfer the fighter.

- /// @param from Address of the current owner.

- /// @param to Address of the new owner.

- /// @param tokenId ID of the fighter being transferred.

- function transferFrom( - address from, - address to, - uint256 tokenId - ) - public - override(ERC721, IERC721) - { - require(_ableToTransfer(tokenId, to)); - _transfer(from, to, tokenId); - } - - /// @notice Safely transfers an NFT from one address to another.

- /// @dev Add a custom check for an ability to transfer the fighter.

- /// @param from Address of the current owner.

- /// @param to Address of the new owner.

- /// @param tokenId ID of the fighter being transferred.

- function safeTransferFrom( - address from, - address to, - uint256 tokenId - ) - public - override(ERC721, IERC721) - { - require(_ableToTransfer(tokenId, to)); - _safeTransfer(from, to, tokenId, ""); - } - /// @notice Rolls a new fighter with random traits.

/// @param tokenId ID of the fighter being re-rolled.

/// @param fighterType The fighter type.

@@ -448,7 +414,9 @@ contract FighterFarm is ERC721, ERC721Enumerable { internal override(ERC721, ERC721Enumerable) { - super._beforeTokenTransfer(from, to, tokenId); + if(from != address(0) && to != address(0)) + require(_ableToTransfer(tokenId, to)); + super._beforeTokenTransfer(from, to, tokenId); } /*////////////////////////////////////////////////////////////// brandinho (AI Arena) confirmed AI Arena mitigated:

Fixed safeTransferFrom override with data.

- https://github.com/ArenaX-Labs/2024-02-ai-arena-mitigation/pull/2
Status:

Mitigation confirmed. Full details in reports from niser93, d3e4, and fnanni.

# [H-02] Non-transferable GameItems can be transferred with GameItems::safeBatchTransferFrom(...)

- **Contest:** AI Arena
- **Slug:** 2024-02-ai-arena
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-02-ai-arena
- **Source snapshot:** competitions/2024-02-ai-arena/final_report.html

GameItems can be transferred with GameItems::safeBatchTransferFrom(...) Submitted by Aamir, also found by m4ttm, vnavascues, visualbits, Fulum, peter, GhK3Ndf, shaka, sandy, alexxander, thank_you, evmboi32, 0xprinc, Limbooo, denzi_, grearlake, Greed, korok, josephdara, juancito, ZanyBonzy, Breeje, hulkvision, KmanOfficial, CodeWasp, DeFiHackLabs, 0xaghas, MidgarAudits, Ryonen, alexzoid, n0kto, immeas, Aymen0909, jaydhales, sashik_eth, soliditywala, 0xLogos, 0x13, Tendency, 0xpoor4ever, PedroZurdo, merlinboii, ni8mare, BARW, israeladelaja, 0xlyov, Draiakoo, petro_1912, ADM, Krace, 0x11singh99, McToady, ktg, 0xCiphky, MrPotatoMagic, 0xWallSecurity, cats, nuthan2x, ladboy233, 0xAsen, stackachu, deadrxsezzz, 0xvj, _eperezok, pkqs90, Josh4324, web3pwn, jnforja, Jorgect, SovaSlava, btk, pynschon, blutorque, lil_eth, 0xE1, fnanni, Bauchibred, devblixt, tpiliposian, erosjohn, Pelz, ubl4nk, cartlex_, krikolkk, Kalogerone, kutugu, zhaojohnson, shaflow2, pa6kuda, djxploit, solmaxis69, 0rpse, 0xlemon, 0xKowalski, SpicyMeatball, novamanbg, DMoore, jesjupyter, csanuragjain, 0xBinChook, matejdb, tallo, aslanbek, 0xAlix2, sobieski, kiqo, dimulski, xchen1130, 0xbranded, oualidpro, Timenov, haxatron, klau5, and al88nsk The GamesItems contract fails to appropriately override and include essential checks within the

safeBatchTransferFrom function, enabling the transfer of non-transferrable Game Items.

## Impact

While the GamesItems contract allows for the designation of Game Items as either transferrable or non-transferrable through different states and overrides the ERC1155::safeTransferFrom(...) function accordingly, it neglects to override the ERC1155::safeBatchTransferFrom(...) function. This oversight permits users to transfer Game Items that were intended to be non-transferrable.

## Recommended Mitigation Steps

It is recommended to override the safeBatchTransferFrom(...) function and include the necessary checks to prevent the transfer of non-transferrable Game Items.

+ function safeBatchTransferFrom( + address from, + address to, + uint256[] memory ids, + uint256[] memory amounts, + bytes memory data + ) public override(ERC1155) { + for(uint256 i; i < ids.length; i++{ + require(allGameItemAttributes[ids[i]].transferable); + } + super.safeBatchTransferFrom(from, to, ids, amounts, data); + } Or, consider overriding the _safeBatchTransferFrom(...) function as follows:

+ function _safeBatchTransferFrom( + address from, + address to, + uint256[] memory ids, + uint256[] memory amounts, + bytes memory data + ) internal override(1155) { + require(ids.length == amounts.length, "ERC1155: ids and amounts length mismatch"); + require(to != address(0), "ERC1155: transfer to the zero address"); + + address operator = _msgSender(); + + _beforeTokenTransfer(operator, from, to, ids, amounts, data); + + for (uint256 i = 0; i < ids.length; ++i) { + require( + uint256 id = ids[i]; + uint256 amount = amounts[i]; + require(allGameItemAttributes[id].transferable); + uint256 fromBalance = _balances[id][from]; + require(fromBalance >= amount, "ERC1155: insufficient balance for transfer");

+ unchecked { + _balances[id][from] = fromBalance - amount; + } + _balances[id][to] += amount; + } + + emit TransferBatch(operator, from, to, ids, amounts); + + _afterTokenTransfer(operator, from, to, ids, amounts, data); + + _doSafeBatchTransferAcceptanceCheck(operator, from, to, ids, amounts, data); + } brandinho (AI Arena) confirmed hickuphh3 (judge) increased severity to High AI Arena mitigated:

Fixed Non-transferable GameItems being transferred with GameItems::safeBatchTransferFrom.

- https://github.com/ArenaX-Labs/2024-02-ai-arena-mitigation/pull/4
Status:

Mitigation confirmed. Full details in reports from niser93, d3e4, and fnanni.

# [H-03] Players have complete freedom to customize the fighter NFT when calling redeemMintPass and can redeem fighters of types Dendroid and with rare attributes

- **Contest:** AI Arena
- **Slug:** 2024-02-ai-arena
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-02-ai-arena
- **Source snapshot:** competitions/2024-02-ai-arena/final_report.html

redeemMintPass and can redeem fighters of types Dendroid and with rare attributes Submitted by Abdessamed, also found by juancito ( 1, 2 ), vnavascues, d3e4, DarkTower, shaka, alexxander, 0xmystery, evmboi32 ( 1, 2, 3 ), soliditywala, givn, korok, dimulski, denzi_, VAD37, sl1, OMEN, stakog, ahmedaghadi, Ryonen, FloatingPragma, n0kto ( 1, 2 ), adamn000, 0rpse, bhilare_, 0xAsen, Tendency, ke1caM, Draiakoo ( 1, 2 ), Archime, ADM, McToady, MrPotatoMagic, 0xCiphky ( 1, 2 ), VrONTg, Velislav4o, btk ( 1, 2 ), stackachu, zhaojohnson, 0xvj ( 1, 2 ), pkqs90, devblixt, niser93, cats, peter, BARW ( 1, 2, 3 ), Aamir, jesjupyter, fnanni, alexzoid, krikolkk, t0x1c, yotov721, klau5, 0xlemon, radin100, 0xAlix2, SpicyMeatball, matejdb, haxatron, JCN, immeas, PetarTolev, and Zac The function redeemMintPass allows burning multiple mint passes in exchange for fighters’ NFTs. It is mentioned by the sponsor that the player should not have a choice of customizing the fighters’ properties and their type. However, nothing prevents a player from:

Providing uint8[] fighterTypes of values 1 to mint fighters of types Dendroid.

Checking previous transactions in which the dna provided led to minting fighters with rare physical attributes, copying those Dnas and passing them to the redeemMinPass to mint fighters with low rarity attributes. That is because creating physical attributes is deterministic, so providing the same inputs leads to generating a fighter with the same attributes.

## Impact

This issue has two major impacts:

Players with valid mint passes can mint fighters of type Dendroid easily.

Players with valid mint passes can mint easily fighters with low rarity attributes which breaks the pseudo-randomness attributes generation aspect

## Recommended Mitigation Steps

The main issue is that the mint pass token is not tied to the fighter properties that the player should claim and the player has complete freedom of the inputs. Consider implementing a signature mechanism that prevents the player from changing the fighter’s properties like implemented in claimFighters brandinho (AI Arena) confirmed AI Arena mitigated:

- https://github.com/ArenaX-Labs/2024-02-ai-arena-mitigation/pull/10
Status:

Mitigation confirmed. Full details in reports from niser93 and fnanni.

# [H-04] Since you can reroll with a different fighterType than the NFT you own, you can reroll bypassing maxRerollsAllowed and reroll attributes based on a different fighterType

- **Contest:** AI Arena
- **Slug:** 2024-02-ai-arena
- **Finding ID:** H-04
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-02-ai-arena
- **Source snapshot:** competitions/2024-02-ai-arena/final_report.html

Submitted by klau5, also found by McToady, Tychai0s ( 1, 2 ), vnavascues, d3e4, shaka, alexxander, grearlake, AlexCzm, lanrebayode77, evmboi32, givn, juancito, VAD37, Varun_05, DanielArmstrong, linmiaomiao, 14si2o_Flint, Ryonen, n0kto, alexzoid, ktg, denzi_, Aymen0909, Davide, soliditywala, Aamir, sashik_eth, sl1, nuthan2x, 0xAsen, merlinboii, ke1caM, Draiakoo, petro_1912, MrPotatoMagic, PoeAudits, 0xCiphky, aslanbek, 0xvj, cats ( 1, 2 ), yotov721, btk, pynschon, solmaxis69, fnanni, 0xAlix2, 0xKowalski, blutorque, t0x1c, ubl4nk, BARW, radin100, Giorgio, zhaojohnson, 0xlemon, jesjupyter, SpicyMeatball, novamanbg, xchen1130, matejdb, haxatron, 0xAleko, Blank_Space, and Silvermist Can reroll attributes based on a different fighterType, and can bypass maxRerollsAllowed.

## Recommended Mitigation Steps

Check fighterType at reRoll function.

function reRoll(uint8 tokenId, uint8 fighterType) public { require(msg.sender == ownerOf(tokenId)); require(numRerolls[tokenId] < maxRerollsAllowed[fighterType]); require(_neuronInstance.balanceOf(msg.sender) >= rerollCost, "Not enough NRN for reroll"); + require((fighterType == 1 && fighters[tokenId].dendroidBool) || (fighterType == 0 && !fighters[tokenId].dendroidBool), "Wrong fighterType"); _neuronInstance.approveSpender(msg.sender, rerollCost); bool success = _neuronInstance.transferFrom(msg.sender, treasuryAddress, rerollCost); if (success) { numRerolls[tokenId] += 1; uint256 dna = uint256(keccak256(abi.encode(msg.sender, tokenId, numRerolls[tokenId]))); (uint256 element, uint256 weight, uint256 newDna) = _createFighterBase(dna, fighterType);

fighters[tokenId].element = element; fighters[tokenId].weight = weight; fighters[tokenId].physicalAttributes = _aiArenaHelperInstance.createPhysicalAttributes( newDna, generation[fighterType], fighters[tokenId].iconsType, fighters[tokenId].dendroidBool ); _tokenURIs[tokenId] = ""; } raymondfam (lookout) commented:

This report covers three consequences from the same root cause of fighter type validation: 1. more re-rolls, 2. rarer attribute switch, 3. generation attribute switch, with

# [H-05] Malicious user can stake an amount which causes zero curStakeAtRisk on a loss but equal rewardPoints to a fair user on a win

- **Contest:** AI Arena
- **Slug:** 2024-02-ai-arena
- **Finding ID:** H-05
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-02-ai-arena
- **Source snapshot:** competitions/2024-02-ai-arena/final_report.html

Submitted by t0x1c, also found by Aamir, peanuts ( 1, 2 ), YouCrossTheLineAlfie, ubermensch, d3e4, shaka, alexxander, CodeWasp, 0xDetermination, evmboi32, Honour, DarkTower, ni8mare, Blank_Space, McToady ( 1, 2 ), neocrao, PedroZurdo, ZanyBonzy, VAD37, DanielArmstrong, 14si2o_Flint ( 1, 2 ), forkforkdog, MidgarAudits, n0kto, Krace, MrPotatoMagic, 0xAadi, 0xBinChook, immeas, btk ( 1, 2 ), forgebyola, aslanbek, Draiakoo ( 1, 2 ), petro_1912, ktg, israeladelaja, 0xCiphky, Merulez99, swizz, 0rpse, Silvermist, ubl4nk ( 1, 2 ), dimulski, erosjohn ( 1, 2 ), fnanni, yotov721, Abdessamed, djxploit, csanuragjain ( 1, 2 ), Kalogerone, shaflow2, WoolCentaur, Tychai0s, handsomegiraffe, AC000123, VrONTg ( 1, 2, 3 ), okolicodes, Velislav4o, and juancito The _getStakingFactor() function rounds-up the stakingFactor_ to one if its zero. Additionally, the _addResultPoints() function rounds-down the curStakeAtRisk.

Whenever a player loses and has no accumulated reward points, 0.1% of his staked amount is considered “at risk” and transferred to the _stakeAtRiskAddress. However due to the above calculation styles, he can stake just 1 wei of NRN to have zero curStakeAtRisk in case of a loss and in case of a win, still gain the same amount of reward points as a player staking 4e18-1 wei of NRN.

Let’s look at three cases of a player with ELO as 1500:

Case Staked NRN stakingFactor calculated by the protocol Reward Points accumulated in case of a WIN NRNs lost (stake at risk) in case of a LOSS 1 4 2 1500 0.004 2 3.999999999999999999 1 750 0.003999999999999999 3 0.000000000000000001 1 750 0 As can be seen in Case2 vs Case3, a player staking 0.000000000000000001 NRN (1 wei) has the same upside in case of a win as a player staking 3.999999999999999999 NRN (4e18-1 wei) while their downside is 0.

## Recommended Mitigation Steps

Protocol can choose to set a minimum stake amount of 4 NRN (4e18 wei). One needs to take care that even after a partial unstake, this amount is not allowed to go below 4 NRN.

Also, do not round up stakingFactor_ i.e. remove L530-L532. An additional check can be added too which ensures that stakingFactor_ is greater than zero:

File: src/RankedBattle.sol 519: function _getStakingFactor( 520: uint256 tokenId, 521: uint256 stakeAtRisk 522: ) 523: private 524: view 525: returns (uint256) 526: { 527: uint256 stakingFactor_ = FixedPointMathLib.sqrt( 528: (amountStaked[tokenId] + stakeAtRisk) / 10**18 529: ); - 530: if (stakingFactor_ == 0) { - 531: stakingFactor_ = 1; - 532: } + 532: require(stakingFactor_ > 0, "stakingFactor_ = 0"); 533: return stakingFactor_; 534: } The above fixes would ensure that curStakeAtRisk can never be gamed to 0 while still having a positive reward potential.

It’s may also be a good idea to have a provision to return any “extra” staked amount. For example, if only 4 NRN is required to achieve a stakingFactor of 1 and the player stakes 4.5 NRN, then the extra 0.5 NRN could be returned. This however is up to the protocol to consider.

raymondfam (lookout) commented:

Strategic dodging to avoid penalty. Might as well fully unstake to make curStakeAtRisk 0. However, points would be zero if at risk penalty were to kick in.

hickuphh3 (judge) increased severity to High

# [H-06] FighterFarm::reRoll won’t work for nft id greater than 255 due to input limited to uint8

- **Contest:** AI Arena
- **Slug:** 2024-02-ai-arena
- **Finding ID:** H-06
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-02-ai-arena
- **Source snapshot:** competitions/2024-02-ai-arena/final_report.html

FighterFarm::reRoll won’t work for nft id greater than 255 due to input limited to uint8 Submitted by abhishek_thaku_r, also found by pontifex, Fulum, 0xDetermination, Greed, givn, stakog, offside0011, maxim371, ktg, alexzoid, immeas, sashik_eth, korok, Draiakoo, MrPotatoMagic, PoeAudits, Tychai0s, ahmedaghadi, kartik_giri_47538, iamandreiski, fnanni, 0xAlix2, klau5, dimulski, 0xShitgem, yotov721, kiqo, and swizz FighterFarm::reRoll uses uint8 for nft id as input, which will stop people calling this function who owns id greater than 255. It will lead to not being able to use the reRoll to get random traits, which could have been better for there game performance.

## Recommended Mitigation Steps

Use uint256 for nft id input to fix the issue.

- function reRoll(uint8 tokenId, uint8 fighterType) public { + function reRoll(uint256 tokenId, uint8 fighterType) public { require(msg.sender == ownerOf(tokenId)); require(numRerolls[tokenId] < maxRerollsAllowed[fighterType]); require(_neuronInstance.balanceOf(msg.sender) >= rerollCost, "Not enough NRN for reroll"); _neuronInstance.approveSpender(msg.sender, rerollCost); bool success = _neuronInstance.transferFrom(msg.sender, treasuryAddress, rerollCost); if (success) { numRerolls[tokenId] += 1; uint256 dna = uint256(keccak256(abi.encode(msg.sender, tokenId, numRerolls[tokenId]))); (uint256 element, uint256 weight, uint256 newDna) = _createFighterBase(dna, fighterType); fighters[tokenId].element = element;

fighters[tokenId].weight = weight; fighters[tokenId].physicalAttributes = _aiArenaHelperInstance.createPhysicalAttributes( newDna, generation[fighterType], fighters[tokenId].iconsType, fighters[tokenId].dendroidBool ); _tokenURIs[tokenId] = ""; } raymondfam (lookout) commented:

Unsigned integer type limitation indeed.

brandinho (AI Arena) confirmed AI Arena mitigated:

Fixed reRoll for fighters with tokenIds greater than 255.

- https://github.com/ArenaX-Labs/2024-02-ai-arena-mitigation/pull/1
Status:

Mitigation confirmed. Full details in reports from niser93, d3e4, and fnanni.

# [H-07] Fighters cannot be minted after the initial generation due to uninitialized numElements mapping

- **Contest:** AI Arena
- **Slug:** 2024-02-ai-arena
- **Finding ID:** H-07
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-02-ai-arena
- **Source snapshot:** competitions/2024-02-ai-arena/final_report.html

numElements mapping Submitted by haxatron, also found by visualbits, vnavascues, sandy, shaka, alexxander, evmboi32, DarkTower, VAD37, 0xStriker, DanielArmstrong ( 1, 2 ), 14si2o_Flint, MidgarAudits, Ryonen, KupiaSec, Topmark, 0xmystery, AgileJune, immeas, MrPotatoMagic, sashik_eth, soliditywala, nuthan2x, 0xaghas, merlinboii, VrONTg, Krace, ke1caM, Draiakoo, petro_1912, PoeAudits, ktg, 0xCiphky, Tychai0s, EagleSecurity, lil_eth, 0xvj, _eperezok, pkqs90, pynschon, peter, Aamir, sl1, 0xAlix2, fnanni, alexzoid, blutorque, cartlex_, Giorgio, radin100, klau5, t0x1c, WoolCentaur, jesjupyter, aslanbek, SpicyMeatball, 0xbranded, Varun_05, d3e4, juancito, 0xlamide, Aymen0909, btk, devblixt, and ubl4nk In FighterFarm.sol there is a mapping numElements which stores the number of possible types of elements a fighter can have in a generation:

FighterFarm.sol#L84-L85 /// @notice Mapping of number elements by generation.

mapping ( uint8 => uint8 ) public numElements; But the problem here is that only the initial generation, Generation 0, is initialized to 3, in the numElements mapping during the constructor of FighterFarm.sol.

FighterFarm.sol#L100-L111 /// @notice Sets the owner address, the delegated address.

/// @param ownerAddress Address of contract deployer.

/// @param delegatedAddress Address of delegated signer for messages.

/// @param treasuryAddress_ Community treasury address.

constructor ( address ownerAddress, address delegatedAddress, address treasuryAddress_ ) ERC721 ( "AI Arena Fighter", "FTR" ) { _ownerAddress = ownerAddress; _delegatedAddress = delegatedAddress; treasuryAddress = treasuryAddress_; numElements [ 0 ] = 3; } It is therefore not possible to write to the numElements mapping for any other generations. As they are uninitialized, numElements[i] = 0 when i != 0 Moreover, this numElements mapping is read from when creating a fighter.

FighterFarm.sol#L458-L474 /// @notice Creates the base attributes for the fighter.

/// @param dna The dna of the fighter.

/// @param fighterType The type of the fighter.

/// @return Attributes of the new fighter: element, weight, and dna.

function _createFighterBase ( uint256 dna, uint8 fighterType ) private view returns ( uint256, uint256, uint256 ) { => uint256 element = dna % numElements [ generation [ fighterType ]]; // numElements is 0 when generation[fighterType] != 0.

uint256 weight = dna % 31 + 65; uint256 newDna = fighterType == 0 ?

dna:

uint256 ( fighterType ); return ( element, weight, newDna ); } Therefore if the protocol updates to a new generation of fighters, it will not be able to create anymore new fighters as numElements[generation[fighterType]] will be uninitialized and therefore equal 0. This will cause the transaction to always revert as any modulo by 0 will cause a panic according to Solidity Docs Modulo with zero causes a Panic error. This check can not be disabled through unchecked { … }.

## Recommended Mitigation Steps

Allow the admin to update the numElements mapping when a new generation is created.

raymondfam (lookout) commented:

Missing numElements[generation[fighterType]] setter.

brandinho (AI Arena) confirmed AI Arena mitigated:

- https://github.com/ArenaX-Labs/2024-02-ai-arena-mitigation/pull/7
Status:

Mitigation confirmed. Full details in reports from niser93, d3e4, and fnanni.

# [H-08] Player can mint more fighter NFTs during claim of rewards by leveraging reentrancy on the claimRewards() function

- **Contest:** AI Arena
- **Slug:** 2024-02-ai-arena
- **Finding ID:** H-08
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-02-ai-arena
- **Source snapshot:** competitions/2024-02-ai-arena/final_report.html

claimRewards() function Submitted by DarkTower, also found by alexxander, 0brxce, BoRonGod, 0xDetermination, evmboi32, grearlake, PedroZurdo, BARW, sl1, Krace, Zac, Tricko, Aymen0909, immeas, rouhsamad, sashik_eth, ke1caM, 0xCiphky, MrPotatoMagic, jnforja, 0xBinChook, web3pwn, 0xLogos, bhilare_, Kow, ubl4nk, zxriptor, djxploit, solmaxis69, klau5, haxatron, and ZanyBonzy When a fighting round ends, winners for the current round get picked and allocated respective rewards. These rewards are fighter NFTs that can be claimed by such winners. When you claim your rewards for a round or several rounds, the numRoundsClaimed state variable which stores the number of rounds you’ve claimed for gets updated to reflect your claim and each winner can only ever claim up to the amounts they win for each given round. That means if you try to batch-claim for two given rounds for which you won 2 fighter NFTs, your NFT count after the claim should be whatever your current balance of NFT is plus 2 fighter NFTs.

The issue here is that there’s a way to mint additional fighter NFTs on top of the fighter NFTs you’re owed for winning even though the claimRewards function has implemented a decent system to prevent over-claims. For one, it’s relatively complex to spoof a call pretending to be the _mergingPoolAddress to mint but a malicious user doesn’t need to worry too much about that to mint more fighters; they just need to leverage using a smart contract for engineering a simple reentrancy.

## Recommended Mitigation Steps

Use a nonReentrant modifier for the claimRewards function.

brandinho (AI Arena) confirmed AI Arena mitigated:

- https://github.com/ArenaX-Labs/2024-02-ai-arena-mitigation/pull/6
Status:

Mitigation confirmed. Full details in reports from niser93, d3e4, and fnanni.

Medium Risk Findings (9)

# [M-01] Almost all rarity rank combinations cannot be, and are not uniformly, generated

- **Contest:** AI Arena
- **Slug:** 2024-02-ai-arena
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-ai-arena
- **Source snapshot:** competitions/2024-02-ai-arena/final_report.html

Submitted by d3e4, also found by niser93 and fnanni Only a tiny fraction, 0.0002427%, of all rarity rank combinations are possible. The probability distribution of the possible combinations (assuming the DNA is uniformly random) is not uniform: 24% of combinations are twice as likely as the rest.

## Recommended Mitigation Steps

Extract multiple small random numbers by repeatedly taking the modulo and dividing. I.e.

- uint256 rarityRank = (dna / attributeToDnaDivisor[attributes[i]]) % 100; + uint256 rarityRank = dna % 100; + dna /= 100; hickuphh3 (judge) commented:

Agree that rarityRank determination isn’t uniformly random.

# [M-02] Minter / Staker / Spender roles can never be revoked

- **Contest:** AI Arena
- **Slug:** 2024-02-ai-arena
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-ai-arena
- **Source snapshot:** competitions/2024-02-ai-arena/final_report.html

Submitted by nuthan2x, also found by 0xvj, klau5, McToady, btk, juancito, merlinboii, alexxander, sandy, favelanky, josephdara, MidgarAudits ( 1, 2 ), _eperezok, SovaSlava, lil_eth, kutugu, zaevlad, jesjupyter, shaflow2, pynschon, SpicyMeatball, 0xE1, PetarTolev, c0pp3rscr3w3r, 0xblackskull, Greed, 0xgrbr, Sabit ( 1, 2, 3 ), Tychai0s, and Timenov From Neuron::addMinter and AccessControl::setupRole function addMinter ( address newMinterAddress ) external { require ( msg.

sender == _ownerAddress ); _setupRole ( MINTER_ROLE, newMinterAddress ); } function _setupRole ( bytes32 role, address account ) internal virtual { _grantRole ( role, account ); } function _grantRole ( bytes32 role, address account ) internal virtual { if (!

hasRole ( role, account )) { _roles [ role ].

members [ account ] = true; emit RoleGranted ( role, account, _msgSender ()); } Roles of minter, staker, spender can never be revoked due to missing default admin implementation. The DEFAULT_ADMIN_ROLE = 0x00 which is default role which is admin to all the roles, and the real contract owner should own this role, since it is not granted, the owner cannot govern the roles.

Another wrong implemnatation is using _setupRole to grant roles intead of using _grantRole, because of the warnings in the library contract.

From Openzeppelin::AccessControl and AccessControl::setupRole * [ WARNING ] * ==== * This function should only be called from the constructor when setting * up the initial roles for the system.

* Using this function in any other way is effectively circumventing the admin * system imposed by { AccessControl }.

* ==== * * NOTE:

This function is deprecated in favor of { _grantRole }.

*/ function _setupRole ( bytes32 role, address account ) internal virtual { _grantRole ( role, account ); } * Roles can be granted and revoked dynamically via the { grantRole } and * { revokeRole } functions.

Each role has an associated admin role, and only * accounts that have a role 's admin role can call {grantRole} and {revokeRole}.

* By default, the admin role for all roles is `DEFAULT_ADMIN_ROLE`, which means * that only accounts with this role will be able to grant or revoke other * roles.

More complex role relationships can be created by using * { _setRoleAdmin }.

* * WARNING:

The `DEFAULT_ADMIN_ROLE` is also its own admin:

it has permission to * grant and revoke this role.

Extra precautions should be taken to secure * accounts that have been granted it.

*/

## Recommended Mitigation Steps

Modify the constructor on Neuron.sol to grant default admin role to owner.

constructor(address ownerAddress, address treasuryAddress_, address contributorAddress) ERC20("Neuron", "NRN") { _ownerAddress = ownerAddress; treasuryAddress = treasuryAddress_; isAdmin[_ownerAddress] = true; _mint(treasuryAddress, INITIAL_TREASURY_MINT); _mint(contributorAddress, INITIAL_CONTRIBUTOR_MINT); + _grantRole(DEFAULT_ADMIN_ROLE, _ownerAddress); } hickuphh3 (judge) commented:

Selecting as best report because it also mentions that _grantRole should be used instead of _setupRole.

I’m excluding admin error in this. Simply about functionality: not being able to revoke roles might be problematic for deprecation / migration purposes.

# [M-03] Fighter created by mintFromMergingPool can have arbitrary weight and element

- **Contest:** AI Arena
- **Slug:** 2024-02-ai-arena
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-ai-arena
- **Source snapshot:** competitions/2024-02-ai-arena/final_report.html

mintFromMergingPool can have arbitrary weight and element Submitted by ktg, also found by visualbits, d3e4, vnavascues, dutra, niser93, alexxander, denzi_, Silvermist, evmboi32, Tumelo_Crypto, givn, Blank_Space, yotov721, juancito, Matue, stakog, 0xWallSecurity, FloatingPragma, ahmedaghadi, aslanbek, agadzhalov, Topmark, immeas, _eperezok, AlexCzm, klau5, petro_1912, Tendency, BARW, peter, ke1caM, Draiakoo, McToady, 0xCiphky, 0xvj, pkqs90, cats, fnanni, rspadi, krikolkk, handsomegiraffe, Giorgio, 0xlemon, kiqo, SpicyMeatball, 0xRiO, haxatron, MrPotatoMagic, 0xDetermination, and sl1

- https://github.com/code-423n4/2024-02-ai-arena/blob/main/src/FighterFarm.sol#L313-L331
- https://github.com/code-423n4/2024-02-ai-arena/blob/main/src/MergingPool.sol#L139-L167
Fighter created by mintFromMergingPool can have arbitrary weight and element like 0 or 2**256 - 1 Invalid weight and element could greatly affect AI Arena battles.

## Recommended Mitigation Steps

I recommend checking customAttributes in function mintFromMergingPool and only restrict weight and element in predefined ranges. For example, weight must be in range [65, 95], element must be in range [0,2].

brandinho (AI Arena) confirmed via duplicate issue #1670 AI Arena mitigated:

- https://github.com/ArenaX-Labs/2024-02-ai-arena-mitigation/pull/16
Status:

Not fully mitigated. Full details in reports from fnanni and niser93 ( 1, 2 ), and also included in the

# [M-04] DoS in MergingPool::claimRewards function and potential DoS in RankedBattle::claimNRN function if called after a significant amount of rounds passed

- **Contest:** AI Arena
- **Slug:** 2024-02-ai-arena
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-ai-arena
- **Source snapshot:** competitions/2024-02-ai-arena/final_report.html

MergingPool::claimRewards function and potential DoS in RankedBattle::claimNRN function if called after a significant amount of rounds passed Submitted by ahmedaghadi, also found by SpicyMeatball ( 1, 2 ), btk, 0xKowalski, ktg ( 1, 2 ), 0xDetermination ( 1, 2 ), Ryonen, sl1 ( 1, 2 ), MrPotatoMagic, klau5 ( 1, 2, 3 ), vnavascues, pontifex, Nyxaris, alexzoid, inzinko, grearlake ( 1, 2 ), Honour, cu5t0mpeo, evmboi32, Avci, Greed, josephdara, VAD37, DeFiHackLabs, 0xPluto, radev_sw ( 1, 2 ), MidgarAudits, KmanOfficial, soliditywala, ladboy233, AlexCzm, jesusrod15, erosjohn, merlinboii, taner2344, Krace, ke1caM ( 1, 2 ), BigVeezus, Draiakoo, peanuts ( 1, 2 ), McToady (

1, 2 ), Giorgio, 0x13, deadrxsezzz, SovaSlava, almurhasan ( 1, 2 ), 0xvj, _eperezok ( 1, 2 ), emrekocak, fnanni, pipidu83, y4y, jesjupyter, BARW ( 1, 2 ), djxploit, sobieski, ReadyPlayer2 ( 1, 2 ), Fitro, 0xAleko, zaevlad, GoSlang, 0xRiO, Cryptor, Kalogerone, t0x1c, nuthan2x, yovchev_yoan, and dvrkzy The MergingPool::claimRewards function loop through last claimed round ID to the latest round ID (

- https://github.com/code-423n4/2024-02-ai-arena/blob/main/src/MergingPool.sol#L139
):

function claimRewards ( string [] calldata modelURIs, string [] calldata modelTypes, uint256 [ 2 ][] calldata customAttributes ) external { uint256 winnersLength; uint32 claimIndex = 0; @-> uint32 lowerBound = numRoundsClaimed [ msg.

sender ]; @-> for ( uint32 currentRound = lowerBound; currentRound < roundId; currentRound ++) { numRoundsClaimed [ msg.

sender ] += 1; winnersLength = winnerAddresses [ currentRound ].

length; for ( uint32 j = 0; j < winnersLength; j ++) { if ( msg.

sender == winnerAddresses [ currentRound ][ j ]) { _fighterFarmInstance.

mintFromMergingPool ( msg.

sender, modelURIs [ claimIndex ], modelTypes [ claimIndex ], customAttributes [ claimIndex ] ); claimIndex += 1; } if ( claimIndex > 0 ) { emit Claimed ( msg.

sender, claimIndex ); } Also there’s another nested loop which loops through all the winners each round. Thus, it will become very expensive to claim rewards and eventually leads to block gas limit. Due to which some users may never be able to claim their rewards.

Therefore, If user try to claim their rewards after many rounds has passed then due to the above mentioned loops, it will consume a lot of gas and eventually leads to block gas limit.

Similarly, the RankedBattle::claimNRN function loop through last claimed round ID to the latest round ID (

- https://github.com/code-423n4/2024-02-ai-arena/blob/main/src/RankedBattle.sol#L294
):

function claimNRN () external { require ( numRoundsClaimed [ msg.

sender ] < roundId, "Already claimed NRNs for this period" ); uint256 claimableNRN = 0; uint256 nrnDistribution; @-> uint32 lowerBound = numRoundsClaimed [ msg.

sender ]; @-> for ( uint32 currentRound = lowerBound; currentRound < roundId; currentRound ++) { nrnDistribution = getNrnDistribution ( currentRound ); claimableNRN += ( accumulatedPointsPerAddress [ msg.

sender ][ currentRound ] * nrnDistribution ) / totalAccumulatedPoints [ currentRound ]; numRoundsClaimed [ msg.

sender ] += 1; } if ( claimableNRN > 0 ) { amountClaimed [ msg.

sender ] += claimableNRN; _neuronInstance.

mint ( msg.

sender, claimableNRN ); emit Claimed ( msg.

sender, claimableNRN ); } Although, it’s relatively difficult to reach the block gas limit in claimNRN function as compared to claimRewards function, but still it’s possible.

## Recommended Mitigation Steps

It can be fixed by adding a parameter for the number of rounds to consider.

For claimRewards function, so the changes would look like:

function claimRewards( string[] calldata modelURIs, string[] calldata modelTypes, - uint256[2][] calldata customAttributes + uint256[2][] calldata customAttributes, + uint32 totalRoundsToConsider ) external { uint256 winnersLength; uint32 claimIndex = 0; uint32 lowerBound = numRoundsClaimed[msg.sender]; + require(lowerBound + totalRoundsToConsider < roundId, "MergingPool: totalRoundsToConsider exceeds the limit"); - for (uint32 currentRound = lowerBound; currentRound < roundId; currentRound++) { + for (uint32 currentRound = lowerBound; currentRound < lowerBound + totalRoundsToConsider; currentRound++) { numRoundsClaimed[msg.sender] += 1; winnersLength = winnerAddresses[currentRound].length; for (uint32 j = 0; j < winnersLength; j++) {

if (msg.sender == winnerAddresses[currentRound][j]) { _fighterFarmInstance.mintFromMergingPool( msg.sender, modelURIs[claimIndex], modelTypes[claimIndex], customAttributes[claimIndex] ); claimIndex += 1; } if (claimIndex > 0) { emit Claimed(msg.sender, claimIndex); } For claimNRN function, so the changes would look like:

- function claimNRN() external { + function claimNRN(uint32 totalRoundsToConsider) external { require(numRoundsClaimed[msg.sender] < roundId, "Already claimed NRNs for this period"); uint256 claimableNRN = 0; uint256 nrnDistribution; uint32 lowerBound = numRoundsClaimed[msg.sender]; + require(lowerBound + totalRoundsToConsider < roundId, "RankedBattle: totalRoundsToConsider exceeds the limit"); - for (uint32 currentRound = lowerBound; currentRound < roundId; currentRound++) { + for (uint32 currentRound = lowerBound; currentRound < lowerBound + totalRoundsToConsider; currentRound++) { nrnDistribution = getNrnDistribution(currentRound); claimableNRN += ( accumulatedPointsPerAddress[msg.sender][currentRound] * nrnDistribution

) / totalAccumulatedPoints[currentRound]; numRoundsClaimed[msg.sender] += 1; } if (claimableNRN > 0) { amountClaimed[msg.sender] += claimableNRN; _neuronInstance.mint(msg.sender, claimableNRN); emit Claimed(msg.sender, claimableNRN); } brandinho (AI Arena) confirmed AI Arena mitigated:

- https://github.com/ArenaX-Labs/2024-02-ai-arena-mitigation/pull/12
Status:

Mitigation confirmed. Full details in reports from niser93, fnanni, and d3e4.

# [M-05] Can mint NFT with the desired attributes by reverting transaction

- **Contest:** AI Arena
- **Slug:** 2024-02-ai-arena
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-ai-arena
- **Source snapshot:** competitions/2024-02-ai-arena/final_report.html

Submitted by klau5, also found by klau5, visualbits, thank_you, dutra, d3e4 ( 1, 2 ), andywer, 0xkaju, PedroZurdo, shaka, honey-k12, Matue, favelanky, 0xDetermination ( 1, 2 ), BoRonGod, bgsmallerbear, 0xGreyWolf ( 1, 2 ), soliditywala, PUSH0, Tumelo_Crypto, Blank_Space, korok, Greed, givn ( 1, 2 ), grearlake, evmboi32, juancito, VAD37, dimulski, DanielArmstrong, gesha17, 0xgrbr, FloatingPragma ( 1, 2 ), n0kto, 0xaghas, MidgarAudits, 0xBinChook, Zac, Tricko, alexzoid ( 1, 2 ), immeas, AlexCzm, 0xLogos, ni8mare, peter, 0xWallSecurity, WoolCentaur, 0xlyov, forkforkdog, ke1caM, Draiakoo, McToady, web3pwn, peanuts, kaveyjoe, cats, lsaudit, tallo, Tekken, Silvermist, lil_eth, desaperh, Jorgect, niser93, erosjohn ( 1, 2, 3 ), fnanni, BARW ( 1, 2, 3 ), pa6kuda ( 1, 2, 3, 4 ), t0x1c, solmaxis69, yotov721, SpicyMeatball, PoeAudits, Daniel526, iamandreiski ( 1, 2 ), tpiliposian, kiqo, aslanbek, haxatron, 0rpse, sl1, Giorgio, vnavascues, Nyxaris, and Pelz All the attributes of the NFT that should be randomly determined are set in the same transaction which they’re claimed. Therefore, if a user uses a contract wallet, they can intentionally revert the transaction and retry minting if they do not get the desired attribute.

function _createNewFighter ( address to, uint256 dna, string memory modelHash, string memory modelType, uint8 fighterType, uint8 iconsType, uint256 [ 2 ] memory customAttributes ) private { require ( balanceOf ( to ) < MAX_FIGHTERS_ALLOWED ); uint256 element; uint256 weight; uint256 newDna; if ( customAttributes [ 0 ] == 100 ) { @> ( element, weight, newDna ) = _createFighterBase ( dna, fighterType ); } else { element = customAttributes [ 0 ]; weight = customAttributes [ 1 ]; newDna = dna; } uint256 newId = fighters.

length; bool dendroidBool = fighterType == 1; @> FighterOps.

FighterPhysicalAttributes memory attrs = _aiArenaHelperInstance.

createPhysicalAttributes ( newDna, generation [ fighterType ], iconsType, dendroidBool ); fighters.

push ( FighterOps.

Fighter ( weight, element, attrs, newId, modelHash, modelType, generation [ fighterType ], iconsType, dendroidBool ) ); @> _safeMint ( to, newId ); FighterOps.

fighterCreatedEmitter ( newId, weight, element, generation [ fighterType ]); } function _createFighterBase ( uint256 dna, uint8 fighterType ) private view returns ( uint256, uint256, uint256 ) { uint256 element = dna % numElements [ generation [ fighterType ]]; uint256 weight = dna % 31 + 65; uint256 newDna = fighterType == 0 ?

dna:

uint256 ( fighterType ); return ( element, weight, newDna ); }

## Recommended Mitigation Steps

Users should only request minting, and attributes values should not be determined in the transaction called by the user. When a user claims an NFT, it should only mint the NFT and end. The attribute should be set by the admin or third-party like chainlink VRF after minting so that the user cannot manipulate it.

It’s not about lack of randomless problem, this is about setting attributes at same transaction when minting NFT. Even if you use chainlink, the same bug can happen if you set the attribute and mint NFTs in the same transaction.

brandinho (AI Arena) confirmed Note: For full original discussion, see here.

AI Arena mitigated:

Updated dna generation in reRoll and updated dna generation in claimFighters.

- https://github.com/ArenaX-Labs/2024-02-ai-arena-mitigation/pull/11
Fixed dna generation in mintFromMergingPool.

- https://github.com/ArenaX-Labs/2024-02-ai-arena-mitigation/pull/3
Status:

Not fully mitigated. Full details in reports from fnanni, niser93 ( 1, 2 ), and d3e4 ( 1, 2, 3 ), and also included in the

# [M-06] NFTs can be transferred even if StakeAtRisk remains, so the user’s win cannot be recorded on the chain due to underflow, and can recover past losses that can’t be recovered (steal protocol’s token)

- **Contest:** AI Arena
- **Slug:** 2024-02-ai-arena
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-ai-arena
- **Source snapshot:** competitions/2024-02-ai-arena/final_report.html

Submitted by klau5, also found by klau5, peanuts ( 1, 2 ), ubermensch ( 1, 2, 3 ), denzi_, alexxander, yotov721, 0xMosh, evmboi32, 0xDetermination ( 1, 2 ), alexzoid, AlexCzm ( 1, 2 ), nuthan2x, ZanyBonzy ( 1, 2 ), CodeWasp, DanielArmstrong ( 1, 2 ), stakog, 14si2o_Flint, 0xabhay, KmanOfficial, sl1, 0xG0P1, McToady ( 1, 2 ), immeas, Aymen0909, merlin, merlinboii, ke1caM, blutorque, 0xCiphky ( 1, 2 ), Kalogerone, lanrebayode77, swizz, JCN, handsomegiraffe ( 1, 2 ), Kow, WoolCentaur, t0x1c, shaflow2, Jorgect, Giorgio ( 1, 2 ), jesjupyter, solmaxis69 ( 1, 2 ), 0xAlix2, SpicyMeatball, csanuragjain, haxatron, VAD37, FloatingPragma, dipp, vnavascues, shaka, KupiaSec, tallo, almurhasan, lil_eth, and djxploit

- https://github.com/code-423n4/2024-02-ai-arena/blob/1d18d1298729e443e14fea08149c77182a65da32/src/RankedBattle.sol#L285-L286
- https://github.com/code-423n4/2024-02-ai-arena/blob/1d18d1298729e443e14fea08149c77182a65da32/src/RankedBattle.sol#L495-L496
Cannot record a user’s victory on-chain, and it may be possible to recover past losses (which should impossible).

## Recommended Mitigation Steps

Tokens with a remaining StakeAtRisk should not be allowed to be exchanged.

function unstakeNRN(uint256 amount, uint256 tokenId) external { require(_fighterFarmInstance.ownerOf(tokenId) == msg.sender, "Caller does not own fighter"); if (amount > amountStaked[tokenId]) { amount = amountStaked[tokenId]; } amountStaked[tokenId] -= amount; globalStakedAmount -= amount; stakingFactor[tokenId] = _getStakingFactor( tokenId, _stakeAtRiskInstance.getStakeAtRisk(tokenId) ); _calculatedStakingFactor[tokenId][roundId] = true; hasUnstaked[tokenId][roundId] = true; bool success = _neuronInstance.transfer(msg.sender, amount); if (success) { - if (amountStaked[tokenId] == 0) { + if (amountStaked[tokenId] == 0 && _stakeAtRiskInstance.getStakeAtRisk(tokenId) == 0) { _fighterFarmInstance.updateFighterStaking(tokenId, false);

} emit Unstaked(msg.sender, amount); } hickuphh3 (judge) decreased severity to Medium and commented:

See comment on this vulnerability in the initial primary issue #1641.

brandinho (AI Arena) confirmed AI Arena mitigated:

- https://github.com/ArenaX-Labs/2024-02-ai-arena-mitigation/pull/9
Status:

Not fully mitigated. Full details in report from d3e4, and also included in the

# [M-07] Erroneous probability calculation in physical attributes can lead to significant issues

- **Contest:** AI Arena
- **Slug:** 2024-02-ai-arena
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-ai-arena
- **Source snapshot:** competitions/2024-02-ai-arena/final_report.html

Submitted by haxatron, also found by juancito, BARW, fnanni, and DanielArmstrong To determine what physical attributes a user gets first we obtain a rarityRank which is computed from the DNA.

AiArenaHelper.sol#L106-L110 } else { uint256 rarityRank = ( dna / attributeToDnaDivisor [ attributes [ i ]]) % 100; uint256 attributeIndex = dnaToIndex ( generation, rarityRank, attributes [ i ]); finalAttributeProbabilityIndexes [ i ] = attributeIndex; } Here since we use % 100 operation is used, the range of rarityRank would be [0,99].

This rarityRank is used in the dnaToIndex to determine the final attribute of the part.

AiArenaHelper.sol#L165-L186 /// @dev Convert DNA and rarity rank into an attribute probability index.

/// @param attribute The attribute name.

/// @param rarityRank The rarity rank.

/// @return attributeProbabilityIndex attribute probability index.

function dnaToIndex ( uint256 generation, uint256 rarityRank, string memory attribute ) public view returns ( uint256 attributeProbabilityIndex ) { uint8 [] memory attrProbabilities = getAttributeProbabilities ( generation, attribute ); uint256 cumProb = 0; uint256 attrProbabilitiesLength = attrProbabilities.

length; for ( uint8 i = 0; i < attrProbabilitiesLength; i ++) { cumProb += attrProbabilities [ i ]; if ( cumProb >= rarityRank ) { attributeProbabilityIndex = i + 1; break; } return attributeProbabilityIndex; } There is however, a very subtle bug in the calculation above due to the use of cumProb >= rarityRank as opposed to cumProb > rarityRank.

To explain the above, I will perform a calculation using a simple example. Let’s say we only have 2 possible attributes and the attrProbabilities is [50, 50].

First iteration, when i = 0, we have cumProb = 50, for the if (cumProb >= rarityRank) to be entered the range of values rarityRank can be is [0, 50]. Therefore there is a 51% chance of obtaining this attribute Next iteration, when i = 1, we have cumProb = 100, for the if (cumProb >= rarityRank) to be entered the range of values rarityRank can be is [51, 99]. Therefore there is a 49% chance of obtaining this attribute.

This means that for values in the first index, the probability is 1% greater than intended and for values in the last index the probability is 1% lesser than intended. This can be significant in certain cases, let us run through two of them.

Case 1: The first value in attrProbabilities is 1.

If the first value in attrProbabilities is 1. Let’s say [1, 99].

Then in reality if we perform the calculation above we get the following results:

First iteration, when i = 0, we have cumProb = 1, for the if (cumProb >= rarityRank) to be entered the range of values rarityRank can be is [0, 1]. Therefore there is a 2% chance of obtaining this attribute Next iteration, when i = 1, we have cumProb = 100, for the if (cumProb >= rarityRank) to be entered the range of values rarityRank can be is [2, 99]. Therefore there is a 98% chance of obtaining this attribute.

Then we have twice the chance of getting the rarer item, which would make it twice as common, thus devaluing it.

Case 2: The last value in attrProbabilities is 1.

If the last value in attrProbabilities is 1. Let’s say [99, 1].

Then in reality if we perform the calculation above we get the following results:

First iteration, when i = 0, we have cumProb = 99, for the if (cumProb >= rarityRank) to be entered the range of values rarityRank can be is [0, 99]. Therefore we will always enter the if (cumProb >= rarityRank) block.

Then it would be impossible (0% chance) to obtain the 1% item.

## Recommended Mitigation Steps

It should be cumProb > rarityRank. Going back to our example of [50, 50], if it were cumProb > rarityRank. Then we will get the following results:

First iteration, when i = 0, we have cumProb = 50, for the if (cumProb > rarityRank) to be entered the range of values rarityRank can be is [0, 49]. Therefore there is a 50% chance of obtaining this attribute Next iteration, when i = 1, we have cumProb = 100, for the if (cumProb > rarityRank) to be entered the range of values rarityRank can be is [50, 99]. Therefore there is a 50% chance of obtaining this attribute.

Thus the above recommended mitigation is correct.

hickuphh3 (judge) commented:

Valid issue: wrong inclusion of boundary skews the probability to one-side by 1%.

The shift in probabilities will affect trait generation, which affects fighter valuations.

Note: For full discussion, see here.

# [M-08] Burner role cannot be revoked

- **Contest:** AI Arena
- **Slug:** 2024-02-ai-arena
- **Finding ID:** M-08
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-ai-arena
- **Source snapshot:** competitions/2024-02-ai-arena/final_report.html

Submitted by Timenov, also found by 0x11singh99, vnavascues, MrPotatoMagic, Rolezn, merlinboii ( 1, 2 ), btk, andywer, 0xblackskull, josephdara, CodeWasp, MidgarAudits, Sabit, SovaSlava, lil_eth, and sobieski

- https://github.com/code-423n4/2024-02-ai-arena/blob/f2952187a8afc44ee6adc28769657717b498b7d4/src/GameItems.sol#L185-L188
- https://github.com/code-423n4/2024-02-ai-arena/blob/f2952187a8afc44ee6adc28769657717b498b7d4/src/GameItems.sol#L242-L245
In GameItems::setAllowedBurningAddresses the admin can update the allowedBurningAddresses mapping.

function setAllowedBurningAddresses ( address newBurningAddress ) public { require ( isAdmin [ msg.

sender ]); allowedBurningAddresses [ newBurningAddress ] = true; } This mapping is used to check if the msg.sender has permission to burn specific amount of game items from an account. This can happen through the burn function.

function burn ( address account, uint256 tokenId, uint256 amount ) public { require ( allowedBurningAddresses [ msg.

sender ]); _burn ( account, tokenId, amount ); } However setAllowedBurningAddresses() works only one way. I mean that the admin can give permission, but not revoke it.

## Recommended Mitigation Steps

Use adjustAdminAccess function as an example:

function adjustAdminAccess ( address adminAddress, bool access ) external { require ( msg.

sender == _ownerAddress ); isAdmin [ adminAddress ] = access; } So the new setAllowedBurningAddresses function should look something like this:

function adjustBurningAccess ( address burningAddress, bool access ) public { require ( isAdmin [ msg.

sender ]); allowedBurningAddresses [ burningAddress ] = access; } Now even if the admin sets the wrong address, he can immediately change it.

hickuphh3 (judge) decreased severity to Medium and commented:

Similar to M-02, but the root causes are different even though the effect is the same.

AI Arena mitigated:

- https://github.com/ArenaX-Labs/2024-02-ai-arena-mitigation/pull/18
Status:

Not fully mitigated. Full details in report from d3e4, and also included in the

# [M-09] Constraints of dailyAllowanceReplenishTime and allowanceRemaining during mint() can be bypassed by using alias accounts & safeTransferFrom()

- **Contest:** AI Arena
- **Slug:** 2024-02-ai-arena
- **Finding ID:** M-09
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-02-ai-arena
- **Source snapshot:** competitions/2024-02-ai-arena/final_report.html

dailyAllowanceReplenishTime and allowanceRemaining during mint() can be bypassed by using alias accounts & safeTransferFrom() Submitted by t0x1c, also found by visualbits, MrPotatoMagic, 0xDetermination, MatricksDeCoder, Greed, givn, VAD37, MidgarAudits, Shubham, PedroZurdo, forkforkdog, Draiakoo, 0xCiphky, lil_eth, Velislav4o, ladboy233, cats ( 1, 2 ), lanrebayode77, djxploit, Kalogerone, zaevlad, btk, ZanyBonzy, and SpicyMeatball The mint() function in GameItems.sol constraints a user from minting more than 10 game items in 1 day. This constraint can easily be bypassed since a similar check is missing inside the safeTransferFrom() function:

File:

src / GameItems.

sol 147:

function mint ( uint256 tokenId, uint256 quantity ) external { 148:

require ( tokenId < _itemCount ); 149:

uint256 price = allGameItemAttributes [ tokenId ].

itemPrice * quantity; 150:

require ( _neuronInstance.

balanceOf ( msg.

sender ) >= price, "Not enough NRN for purchase" ); 151:

require ( 152:

allGameItemAttributes [ tokenId ].

finiteSupply == false || 153: ( 154:

allGameItemAttributes [ tokenId ].

finiteSupply == true && 155:

quantity <= allGameItemAttributes [ tokenId ].

itemsRemaining 156: ) 157: ); 158: @---> require ( 159: @---> dailyAllowanceReplenishTime [ msg.

sender ][ tokenId ] <= block.

timestamp || 160: @---> quantity <= allowanceRemaining [ msg.

sender ][ tokenId ] 161: ); 162:

163:

_neuronInstance.

approveSpender ( msg.

sender, price ); 164:

bool success = _neuronInstance.

transferFrom ( msg.

sender, treasuryAddress, price ); 165:

if ( success ) { 166:

if ( dailyAllowanceReplenishTime [ msg.

sender ][ tokenId ] <= block.

timestamp ) { 167:

_replenishDailyAllowance ( tokenId ); 168: } 169:

allowanceRemaining [ msg.

sender ][ tokenId ] -= quantity; 170:

if ( allGameItemAttributes [ tokenId ].

finiteSupply ) { 171:

allGameItemAttributes [ tokenId ].

itemsRemaining -= quantity; 172: } 173:

_mint ( msg.

sender, tokenId, quantity, bytes ( "random" )); 174:

emit BoughtItem ( msg.

sender, tokenId, quantity ); 175: } 176: } Missing check:

File:

src / GameItems.

sol 291:

function safeTransferFrom ( 292:

address from, 293:

address to, 294:

uint256 tokenId, 295:

uint256 amount, 296:

bytes memory data 297: ) 298:

public 299:

override ( ERC1155 ) 300: { 301:

require ( allGameItemAttributes [ tokenId ].

transferable ); 302:

super.

safeTransferFrom ( from, to, tokenId, amount, data ); 303: } Note:

This could also lead to a situation where a NRN whale having enough funds can buy the complete supply of the game items within minutes by using his multiple alias accounts.

## Recommended Mitigation Steps

Add the same check inside safeTransferFrom() too:

File: src/GameItems.sol 291: function safeTransferFrom( 292: address from, 293: address to, 294: uint256 tokenId, 295: uint256 amount, 296: bytes memory data 297: ) 298: public 299: override(ERC1155) 300: { 301: require(allGameItemAttributes[tokenId].transferable); + require(dailyAllowanceReplenishTime[to][tokenId] <= block.timestamp || amount <= allowanceRemaining[to][tokenId], "Cannot bypass constraints via alias accounts"); 302: super.safeTransferFrom(from, to, tokenId, amount, data); 303: } hickuphh3 (judge) commented:

Agree with the issue. The main impact is about bypassing replenishing / minting limits for a specific account by minting batteries / redeeming passes with multiple accounts & transferring all to that 1 account for consumption || transferring fighter NFTs back and forth across multiple wallets.

## Rejected Primary Findings

# Rejected Primary Findings: AI Arena

No rejected primary findings were captured for this contest in the current corpus.

- **Rejected primary count:** 0
- **Primary submission rows captured:** 0
- **Submission status:** requires_authentication
