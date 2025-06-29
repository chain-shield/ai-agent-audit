
### Storage layout (PuppyRaffle) 

```text
players address[]
raffleDuration uint256
raffleStartTime uint256
previousWinner address
feeAddress address
totalFees uint64
tokenIdToRarity mapping(uint256 => uint256)
rarityToUri mapping(uint256 => string)
rarityToName mapping(uint256 => string)
commonImageUri string
rareImageUri string
legendaryImageUri string

```

#### PuppyRaffle.constructor(uint256,address,uint256) [INTERNAL]
```slithir
commonImageUri_1(string) := phi(['commonImageUri_0', 'commonImageUri_2'])
COMMON_RARITY_1(uint256) := phi(['COMMON_RARITY_4', 'COMMON_RARITY_0', 'COMMON_RARITY_2'])
COMMON_1(string) := phi(['COMMON_0', 'COMMON_2'])
rareImageUri_1(string) := phi(['rareImageUri_0', 'rareImageUri_2'])
RARE_RARITY_1(uint256) := phi(['RARE_RARITY_0', 'RARE_RARITY_2', 'RARE_RARITY_4'])
RARE_1(string) := phi(['RARE_0', 'RARE_2'])
legendaryImageUri_1(string) := phi(['legendaryImageUri_0', 'legendaryImageUri_2'])
LEGENDARY_RARITY_1(uint256) := phi(['LEGENDARY_RARITY_2', 'LEGENDARY_RARITY_4', 'LEGENDARY_RARITY_0'])
LEGENDARY_1(string) := phi(['LEGENDARY_0', 'LEGENDARY_2'])
 entranceFee = _entranceFee
entranceFee_1(uint256) := _entranceFee_1(uint256)
 feeAddress = _feeAddress
feeAddress_1(address) := _feeAddress_1(address)
 raffleDuration = _raffleDuration
raffleDuration_1(uint256) := _raffleDuration_1(uint256)
 raffleStartTime = block.timestamp
raffleStartTime_1(uint256) := block.timestamp(uint256)
 rarityToUri[COMMON_RARITY] = commonImageUri
REF_226(string) -> rarityToUri_0[COMMON_RARITY_2]
rarityToUri_1(mapping(uint256 => string)) := phi(['rarityToUri_0'])
REF_226(string) (->rarityToUri_1) := commonImageUri_2(string)
 rarityToUri[RARE_RARITY] = rareImageUri
REF_227(string) -> rarityToUri_1[RARE_RARITY_2]
rarityToUri_2(mapping(uint256 => string)) := phi(['rarityToUri_1'])
REF_227(string) (->rarityToUri_2) := rareImageUri_2(string)
 rarityToUri[LEGENDARY_RARITY] = legendaryImageUri
REF_228(string) -> rarityToUri_2[LEGENDARY_RARITY_2]
rarityToUri_3(mapping(uint256 => string)) := phi(['rarityToUri_2'])
REF_228(string) (->rarityToUri_3) := legendaryImageUri_2(string)
 rarityToName[COMMON_RARITY] = COMMON
REF_229(string) -> rarityToName_0[COMMON_RARITY_2]
rarityToName_1(mapping(uint256 => string)) := phi(['rarityToName_0'])
REF_229(string) (->rarityToName_1) := COMMON_2(string)
 rarityToName[RARE_RARITY] = RARE
REF_230(string) -> rarityToName_1[RARE_RARITY_2]
rarityToName_2(mapping(uint256 => string)) := phi(['rarityToName_1'])
REF_230(string) (->rarityToName_2) := RARE_2(string)
 rarityToName[LEGENDARY_RARITY] = LEGENDARY
REF_231(string) -> rarityToName_2[LEGENDARY_RARITY_2]
rarityToName_3(mapping(uint256 => string)) := phi(['rarityToName_2'])
REF_231(string) (->rarityToName_3) := LEGENDARY_2(string)
 ERC721(Puppy Raffle,PR)
INTERNAL_CALL, ERC721.constructor(string,string)(Puppy Raffle,PR)
```
#### PuppyRaffle.tokenURI(uint256) [PUBLIC]
```slithir
tokenIdToRarity_5(mapping(uint256 => uint256)) := phi(['tokenIdToRarity_4', 'tokenIdToRarity_3', 'tokenIdToRarity_6', 'tokenIdToRarity_0'])
rarityToUri_4(mapping(uint256 => string)) := phi(['rarityToUri_3', 'rarityToUri_5', 'rarityToUri_0'])
rarityToName_4(mapping(uint256 => string)) := phi(['rarityToName_5', 'rarityToName_0', 'rarityToName_3'])
 require(bool,string)(_exists(tokenId),PuppyRaffle: URI query for nonexistent token)
TMP_615(bool) = INTERNAL_CALL, ERC721._exists(uint256)(tokenId_1)
TMP_616(None) = SOLIDITY_CALL require(bool,string)(TMP_615,PuppyRaffle: URI query for nonexistent token)
 rarity = tokenIdToRarity[tokenId]
REF_260(uint256) -> tokenIdToRarity_6[tokenId_1]
rarity_1(uint256) := REF_260(uint256)
 imageURI = rarityToUri[rarity]
REF_261(string) -> rarityToUri_5[rarity_1]
imageURI_1(string) := REF_261(string)
 rareName = rarityToName[rarity]
REF_262(string) -> rarityToName_5[rarity_1]
rareName_1(string) := REF_262(string)
 string(abi.encodePacked(_baseURI(),Base64.encode(bytes(abi.encodePacked({"name":",name(),", "description":"An adorable puppy!", ,"attributes": [{"trait_type": "rarity", "value": ,rareName,}], "image":",imageURI,"})))))
TMP_617(string) = INTERNAL_CALL, PuppyRaffle._baseURI()()
TMP_618(string) = INTERNAL_CALL, ERC721.name()()
TMP_619(bytes) = SOLIDITY_CALL abi.encodePacked()({"name":",TMP_618,", "description":"An adorable puppy!", ,"attributes": [{"trait_type": "rarity", "value": ,rareName_1,}], "image":",imageURI_1,"})
TMP_620 = CONVERT TMP_619 to bytes
TMP_621(string) = LIBRARY_CALL, dest:Base64, function:Base64.encode(bytes), arguments:['TMP_620'] 
TMP_622(bytes) = SOLIDITY_CALL abi.encodePacked()(TMP_617,TMP_621)
TMP_623 = CONVERT TMP_622 to string
RETURN TMP_623
```
#### PuppyRaffle.enterRaffle(address[]) [PUBLIC]
```slithir
entranceFee_2(uint256) := phi(['entranceFee_0', 'entranceFee_1'])
players_1(address[]) := phi(['players_6', 'players_0', 'players_9', 'players_5', 'players_1', 'players_10'])
 require(bool,string)(msg.value == entranceFee * newPlayers.length,PuppyRaffle: Must send enough to enter raffle)
REF_232 -> LENGTH newPlayers_1
TMP_549(uint256) = entranceFee_2 * REF_232
TMP_550(bool) = msg.value == TMP_549
TMP_551(None) = SOLIDITY_CALL require(bool,string)(TMP_550,PuppyRaffle: Must send enough to enter raffle)
 i = 0
i_1(uint256) := 0(uint256)
 i < newPlayers.length
i_2(uint256) := phi(['i_3', 'i_1'])
REF_233 -> LENGTH newPlayers_1
TMP_552(bool) = i_2 < REF_233
CONDITION TMP_552
 players.push(newPlayers[i])
REF_235(address) -> newPlayers_1[i_2]
REF_236 -> LENGTH players_1
TMP_554(uint256) := REF_236(uint256)
TMP_555(uint256) = TMP_554 + 1
players_2(address[]) := phi(['players_1'])
REF_236(uint256) (->players_2) := TMP_555(uint256)
REF_237(address) -> players_2[TMP_554]
players_3(address[]) := phi(['players_2'])
REF_237(address) (->players_3) := REF_235(address)
 i ++
TMP_556(uint256) := i_2(uint256)
i_3(uint256) = i_2 + 1
 i_scope_0 = 0
i_scope_0_1(uint256) := 0(uint256)
 i_scope_0 < players.length - 1
i_scope_0_2(uint256) := phi(['i_scope_0_3', 'i_scope_0_1'])
REF_238 -> LENGTH players_1
TMP_557(uint256) = REF_238 - 1
TMP_558(bool) = i_scope_0_2 < TMP_557
CONDITION TMP_558
 j = i_scope_0 + 1
TMP_559(uint256) = i_scope_0_2 + 1
j_1(uint256) := TMP_559(uint256)
 j < players.length
j_2(uint256) := phi(['j_1', 'j_3'])
REF_239 -> LENGTH players_1
TMP_560(bool) = j_2 < REF_239
CONDITION TMP_560
 require(bool,string)(players[i_scope_0] != players[j],PuppyRaffle: Duplicate player)
REF_240(address) -> players_1[i_scope_0_2]
REF_241(address) -> players_1[j_2]
TMP_561(bool) = REF_240 != REF_241
TMP_562(None) = SOLIDITY_CALL require(bool,string)(TMP_561,PuppyRaffle: Duplicate player)
 j ++
TMP_563(uint256) := j_2(uint256)
j_3(uint256) = j_2 + 1
 i_scope_0 ++
TMP_564(uint256) := i_scope_0_2(uint256)
i_scope_0_3(uint256) = i_scope_0_2 + 1
 RaffleEnter(newPlayers)
Emit RaffleEnter(newPlayers_1)
```
#### PuppyRaffle.refund(uint256) [PUBLIC]
```slithir
entranceFee_3(uint256) := phi(['entranceFee_0', 'entranceFee_1'])
players_4(address[]) := phi(['players_6', 'players_0', 'players_9', 'players_5', 'players_1', 'players_10'])
 playerAddress = players[playerIndex]
REF_242(address) -> players_4[playerIndex_1]
playerAddress_1(address) := REF_242(address)
 require(bool,string)(playerAddress == msg.sender,PuppyRaffle: Only the player can refund)
TMP_566(bool) = playerAddress_1 == msg.sender
TMP_567(None) = SOLIDITY_CALL require(bool,string)(TMP_566,PuppyRaffle: Only the player can refund)
 require(bool,string)(playerAddress != address(0),PuppyRaffle: Player already refunded, or is not active)
TMP_568 = CONVERT 0 to address
TMP_569(bool) = playerAddress_1 != TMP_568
TMP_570(None) = SOLIDITY_CALL require(bool,string)(TMP_569,PuppyRaffle: Player already refunded, or is not active)
 address(msg.sender).sendValue(entranceFee)
TMP_571 = CONVERT msg.sender to address
LIBRARY_CALL, dest:Address, function:Address.sendValue(address,uint256), arguments:['TMP_571', 'entranceFee_3'] 
 players[playerIndex] = address(0)
REF_244(address) -> players_4[playerIndex_1]
TMP_573 = CONVERT 0 to address
players_5(address[]) := phi(['players_4'])
REF_244(address) (->players_5) := TMP_573(address)
 RaffleRefunded(playerAddress)
Emit RaffleRefunded(playerAddress_1)
```
#### PuppyRaffle.getActivePlayerIndex(address) [EXTERNAL]
```slithir
players_6(address[]) := phi(['players_6', 'players_0', 'players_9', 'players_5', 'players_1', 'players_10'])
 i = 0
i_1(uint256) := 0(uint256)
 i < players.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_245 -> LENGTH players_6
TMP_575(bool) = i_2 < REF_245
CONDITION TMP_575
 players[i] == player
REF_246(address) -> players_6[i_2]
TMP_576(bool) = REF_246 == player_1
CONDITION TMP_576
 i
RETURN i_2
 i ++
TMP_577(uint256) := i_2(uint256)
i_3(uint256) = i_2 + 1
 0
RETURN 0
```
#### PuppyRaffle.selectWinner() [EXTERNAL]
```slithir
entranceFee_4(uint256) := phi(['entranceFee_0', 'entranceFee_1'])
players_7(address[]) := phi(['players_6', 'players_0', 'players_9', 'players_5', 'players_1', 'players_10'])
raffleDuration_2(uint256) := phi(['raffleDuration_0', 'raffleDuration_1'])
raffleStartTime_2(uint256) := phi(['raffleStartTime_0', 'raffleStartTime_1', 'raffleStartTime_3'])
totalFees_1(uint64) := phi(['totalFees_2', 'totalFees_0', 'totalFees_4'])
COMMON_RARITY_3(uint256) := phi(['COMMON_RARITY_4', 'COMMON_RARITY_0', 'COMMON_RARITY_2'])
RARE_RARITY_3(uint256) := phi(['RARE_RARITY_0', 'RARE_RARITY_2', 'RARE_RARITY_4'])
LEGENDARY_RARITY_3(uint256) := phi(['LEGENDARY_RARITY_2', 'LEGENDARY_RARITY_4', 'LEGENDARY_RARITY_0'])
 require(bool,string)(block.timestamp >= raffleStartTime + raffleDuration,PuppyRaffle: Raffle not over)
TMP_578(uint256) = raffleStartTime_2 + raffleDuration_2
TMP_579(bool) = block.timestamp >= TMP_578
TMP_580(None) = SOLIDITY_CALL require(bool,string)(TMP_579,PuppyRaffle: Raffle not over)
 require(bool,string)(players.length >= 4,PuppyRaffle: Need at least 4 players)
REF_247 -> LENGTH players_7
TMP_581(bool) = REF_247 >= 4
TMP_582(None) = SOLIDITY_CALL require(bool,string)(TMP_581,PuppyRaffle: Need at least 4 players)
 winnerIndex = uint256(keccak256(bytes)(abi.encodePacked(msg.sender,block.timestamp,block.difficulty))) % players.length
TMP_583(bytes) = SOLIDITY_CALL abi.encodePacked()(msg.sender,block.timestamp,block.difficulty)
TMP_584(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_583)
TMP_585 = CONVERT TMP_584 to uint256
REF_249 -> LENGTH players_7
TMP_586(uint256) = TMP_585 % REF_249
winnerIndex_1(uint256) := TMP_586(uint256)
 winner = players[winnerIndex]
REF_250(address) -> players_7[winnerIndex_1]
winner_1(address) := REF_250(address)
 totalAmountCollected = players.length * entranceFee
REF_251 -> LENGTH players_7
TMP_587(uint256) = REF_251 * entranceFee_4
totalAmountCollected_1(uint256) := TMP_587(uint256)
 prizePool = (totalAmountCollected * 80) / 100
TMP_588(uint256) = totalAmountCollected_1 * 80
TMP_589(uint256) = TMP_588 / 100
prizePool_1(uint256) := TMP_589(uint256)
 fee = (totalAmountCollected * 20) / 100
TMP_590(uint256) = totalAmountCollected_1 * 20
TMP_591(uint256) = TMP_590 / 100
fee_1(uint256) := TMP_591(uint256)
 totalFees = totalFees + uint64(fee)
TMP_592 = CONVERT fee_1 to uint64
TMP_593(uint64) = totalFees_1 + TMP_592
totalFees_2(uint64) := TMP_593(uint64)
 tokenId = totalSupply()
TMP_594(uint256) = INTERNAL_CALL, ERC721.totalSupply()()
tokenId_1(uint256) := TMP_594(uint256)
 rarity = uint256(keccak256(bytes)(abi.encodePacked(msg.sender,block.difficulty))) % 100
TMP_595(bytes) = SOLIDITY_CALL abi.encodePacked()(msg.sender,block.difficulty)
TMP_596(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_595)
TMP_597 = CONVERT TMP_596 to uint256
TMP_598(uint256) = TMP_597 % 100
rarity_1(uint256) := TMP_598(uint256)
 rarity <= COMMON_RARITY
TMP_599(bool) = rarity_1 <= COMMON_RARITY_4
CONDITION TMP_599
 tokenIdToRarity[tokenId] = COMMON_RARITY
REF_253(uint256) -> tokenIdToRarity_0[tokenId_1]
tokenIdToRarity_4(mapping(uint256 => uint256)) := phi(['tokenIdToRarity_0'])
REF_253(uint256) (->tokenIdToRarity_4) := COMMON_RARITY_4(uint256)
 rarity <= COMMON_RARITY + RARE_RARITY
TMP_600(uint256) = COMMON_RARITY_4 + RARE_RARITY_4
TMP_601(bool) = rarity_1 <= TMP_600
CONDITION TMP_601
 tokenIdToRarity[tokenId] = RARE_RARITY
REF_254(uint256) -> tokenIdToRarity_0[tokenId_1]
tokenIdToRarity_2(mapping(uint256 => uint256)) := phi(['tokenIdToRarity_0'])
REF_254(uint256) (->tokenIdToRarity_2) := RARE_RARITY_4(uint256)
 tokenIdToRarity[tokenId] = LEGENDARY_RARITY
REF_255(uint256) -> tokenIdToRarity_0[tokenId_1]
tokenIdToRarity_1(mapping(uint256 => uint256)) := phi(['tokenIdToRarity_0'])
REF_255(uint256) (->tokenIdToRarity_1) := LEGENDARY_RARITY_4(uint256)
tokenIdToRarity_3(mapping(uint256 => uint256)) := phi(['tokenIdToRarity_1', 'tokenIdToRarity_2'])
 delete players
players_9 = delete players_8 
 raffleStartTime = block.timestamp
raffleStartTime_3(uint256) := block.timestamp(uint256)
 previousWinner = winner
previousWinner_1(address) := winner_1(address)
 (success,None) = winner.call{value: prizePool}()
TUPLE_8(bool,bytes) = LOW_LEVEL_CALL, dest:winner_1, function:call, arguments:[''] value:prizePool_1 
success_1(bool)= UNPACK TUPLE_8 index: 0 
 require(bool,string)(success,PuppyRaffle: Failed to send prize pool to winner)
TMP_602(None) = SOLIDITY_CALL require(bool,string)(success_1,PuppyRaffle: Failed to send prize pool to winner)
 _safeMint(winner,tokenId)
INTERNAL_CALL, ERC721._safeMint(address,uint256)(winner_1,tokenId_1)
```
#### PuppyRaffle.withdrawFees() [EXTERNAL]
```slithir
feeAddress_2(address) := phi(['feeAddress_3', 'feeAddress_4', 'feeAddress_1', 'feeAddress_0'])
totalFees_3(uint64) := phi(['totalFees_2', 'totalFees_0', 'totalFees_4'])
 require(bool,string)(address(this).balance == uint256(totalFees),PuppyRaffle: There are currently players active!)
TMP_604 = CONVERT this to address
TMP_605(uint256) = SOLIDITY_CALL balance(address)(TMP_604)
TMP_606 = CONVERT totalFees_3 to uint256
TMP_607(bool) = TMP_605 == TMP_606
TMP_608(None) = SOLIDITY_CALL require(bool,string)(TMP_607,PuppyRaffle: There are currently players active!)
 feesToWithdraw = totalFees
feesToWithdraw_1(uint256) := totalFees_3(uint64)
 totalFees = 0
totalFees_4(uint64) := 0(uint256)
 (success,None) = feeAddress.call{value: feesToWithdraw}()
TUPLE_9(bool,bytes) = LOW_LEVEL_CALL, dest:feeAddress_2, function:call, arguments:[''] value:feesToWithdraw_1 
feeAddress_3(address) := phi(['feeAddress_3', 'feeAddress_4', 'feeAddress_1', 'feeAddress_2'])
success_1(bool)= UNPACK TUPLE_9 index: 0 
 require(bool,string)(success,PuppyRaffle: Failed to withdraw fees)
TMP_609(None) = SOLIDITY_CALL require(bool,string)(success_1,PuppyRaffle: Failed to withdraw fees)
```
#### PuppyRaffle.changeFeeAddress(address) [EXTERNAL][OWNER]
```slithir
 feeAddress = newFeeAddress
feeAddress_4(address) := newFeeAddress_1(address)
 FeeAddressChanged(newFeeAddress)
Emit FeeAddressChanged(newFeeAddress_1)
 onlyOwner()
MODIFIER_CALL, Ownable.onlyOwner()()
```
#### PuppyRaffle._isActivePlayer() [INTERNAL]
```slithir
players_10(address[]) := phi(['players_6', 'players_0', 'players_9', 'players_5', 'players_1', 'players_10'])
 i = 0
i_1(uint256) := 0(uint256)
 i < players.length
i_2(uint256) := phi(['i_3', 'i_1'])
REF_258 -> LENGTH players_10
TMP_612(bool) = i_2 < REF_258
CONDITION TMP_612
 players[i] == msg.sender
REF_259(address) -> players_10[i_2]
TMP_613(bool) = REF_259 == msg.sender
CONDITION TMP_613
 true
RETURN True
 i ++
TMP_614(uint256) := i_2(uint256)
i_3(uint256) = i_2 + 1
 false
RETURN False
```
#### PuppyRaffle._baseURI() [INTERNAL]
```slithir
 data:application/json;base64,
RETURN data:application/json;base64,
```
#### PuppyRaffle.slitherConstructorVariables() [INTERNAL]
```slithir
 totalFees = 0
 commonImageUri = ipfs://QmSsYRx3LpDAb1GZQm7zZ1AuHZjfbPkD6J7s9r41xu1mf8
 rareImageUri = ipfs://QmUPjADFGEKmfohdTaNcWhp7VGk26h5jXDA7v3VtTnTLcW
 legendaryImageUri = ipfs://QmYx6GsYAKnNzZ9A6NvEKV9nf1VaDzJrqDR23Y8YSkebLU
```
#### PuppyRaffle.slitherConstructorConstantVariables() [INTERNAL]
```slithir
Expression: COMMON_RARITY = 70
Expression: COMMON = common
Expression: RARE_RARITY = 25
Expression: RARE = rare
Expression: LEGENDARY_RARITY = 5
Expression: LEGENDARY = legendary
Expression: require(bool,string)(owner() == _msgSender(),Ownable: caller is not the owner)
IRs:
TMP_624(address) = INTERNAL_CALL, Ownable.owner()()
TMP_625(address) = INTERNAL_CALL, Context._msgSender()()
TMP_626(bool) = TMP_624 == TMP_625
TMP_627(None) = SOLIDITY_CALL require(bool,string)(TMP_626,Ownable: caller is not the owner)
```

#### Address.sendValue(address,uint256) [INTERNAL]
```slithir
 require(bool,string)(address(this).balance >= amount,Address: insufficient balance)
TMP_281 = CONVERT this to address
TMP_282(uint256) = SOLIDITY_CALL balance(address)(TMP_281)
TMP_283(bool) = TMP_282 >= amount_1
TMP_284(None) = SOLIDITY_CALL require(bool,string)(TMP_283,Address: insufficient balance)
 (success,None) = recipient.call{value: amount}()
TUPLE_1(bool,bytes) = LOW_LEVEL_CALL, dest:recipient_1, function:call, arguments:[''] value:amount_1 
success_1(bool)= UNPACK TUPLE_1 index: 0 
 require(bool,string)(success,Address: unable to send value, recipient may have reverted)
TMP_285(None) = SOLIDITY_CALL require(bool,string)(success_1,Address: unable to send value, recipient may have reverted)
```
