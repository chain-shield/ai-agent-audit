
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

#### PuppyRaffle._baseURI() [INTERNAL]
```slithir
 data:application/json;base64,
RETURN data:application/json;base64,
```
#### PuppyRaffle._isActivePlayer() [INTERNAL]
```slithir
players_10(address[]) := phi(['players_9', 'players_1', 'players_5', 'players_10', 'players_0', 'players_6'])
 i = 0
i_1(uint256) := 0(uint256)
 i < players.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_2337 -> LENGTH players_10
TMP_11167(bool) = i_2 < REF_2337
CONDITION TMP_11167
 players[i] == msg.sender
REF_2338(address) -> players_10[i_2]
TMP_11168(bool) = REF_2338 == msg.sender
CONDITION TMP_11168
 true
RETURN True
 i ++
TMP_11169(uint256) := i_2(uint256)
i_3(uint256) = i_2 + 1
 false
RETURN False
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
#### PuppyRaffle.constructor(uint256,address,uint256) [PUBLIC]
```slithir
commonImageUri_1(string) := phi(['commonImageUri_2', 'commonImageUri_0'])
COMMON_RARITY_1(uint256) := phi(['COMMON_RARITY_4', 'COMMON_RARITY_0', 'COMMON_RARITY_2'])
COMMON_1(string) := phi(['COMMON_0', 'COMMON_2'])
rareImageUri_1(string) := phi(['rareImageUri_2', 'rareImageUri_0'])
RARE_RARITY_1(uint256) := phi(['RARE_RARITY_2', 'RARE_RARITY_4', 'RARE_RARITY_0'])
RARE_1(string) := phi(['RARE_0', 'RARE_2'])
legendaryImageUri_1(string) := phi(['legendaryImageUri_0', 'legendaryImageUri_2'])
LEGENDARY_RARITY_1(uint256) := phi(['LEGENDARY_RARITY_0', 'LEGENDARY_RARITY_2', 'LEGENDARY_RARITY_4'])
LEGENDARY_1(string) := phi(['LEGENDARY_2', 'LEGENDARY_0'])
 entranceFee = _entranceFee
entranceFee_1(uint256) := _entranceFee_1(uint256)
 feeAddress = _feeAddress
feeAddress_1(address) := _feeAddress_1(address)
 raffleDuration = _raffleDuration
raffleDuration_1(uint256) := _raffleDuration_1(uint256)
 raffleStartTime = block.timestamp
raffleStartTime_1(uint256) := block.timestamp(uint256)
 rarityToUri[COMMON_RARITY] = commonImageUri
REF_2305(string) -> rarityToUri_0[COMMON_RARITY_2]
rarityToUri_1(mapping(uint256 => string)) := phi(['rarityToUri_0'])
REF_2305(string) (->rarityToUri_1) := commonImageUri_2(string)
 rarityToUri[RARE_RARITY] = rareImageUri
REF_2306(string) -> rarityToUri_1[RARE_RARITY_2]
rarityToUri_2(mapping(uint256 => string)) := phi(['rarityToUri_1'])
REF_2306(string) (->rarityToUri_2) := rareImageUri_2(string)
 rarityToUri[LEGENDARY_RARITY] = legendaryImageUri
REF_2307(string) -> rarityToUri_2[LEGENDARY_RARITY_2]
rarityToUri_3(mapping(uint256 => string)) := phi(['rarityToUri_2'])
REF_2307(string) (->rarityToUri_3) := legendaryImageUri_2(string)
 rarityToName[COMMON_RARITY] = COMMON
REF_2308(string) -> rarityToName_0[COMMON_RARITY_2]
rarityToName_1(mapping(uint256 => string)) := phi(['rarityToName_0'])
REF_2308(string) (->rarityToName_1) := COMMON_2(string)
 rarityToName[RARE_RARITY] = RARE
REF_2309(string) -> rarityToName_1[RARE_RARITY_2]
rarityToName_2(mapping(uint256 => string)) := phi(['rarityToName_1'])
REF_2309(string) (->rarityToName_2) := RARE_2(string)
 rarityToName[LEGENDARY_RARITY] = LEGENDARY
REF_2310(string) -> rarityToName_2[LEGENDARY_RARITY_2]
rarityToName_3(mapping(uint256 => string)) := phi(['rarityToName_2'])
REF_2310(string) (->rarityToName_3) := LEGENDARY_2(string)
 ERC721(Puppy Raffle,PR)
INTERNAL_CALL, ERC721.constructor(string,string)(Puppy Raffle,PR)
```
#### PuppyRaffle.enterRaffle(address[]) [PUBLIC]
```slithir
entranceFee_2(uint256) := phi(['entranceFee_1', 'entranceFee_0'])
players_1(address[]) := phi(['players_9', 'players_1', 'players_5', 'players_10', 'players_0', 'players_6'])
 require(bool,string)(msg.value == entranceFee * newPlayers.length,PuppyRaffle: Must send enough to enter raffle)
REF_2311 -> LENGTH newPlayers_1
TMP_11104(uint256) = entranceFee_2 * REF_2311
TMP_11105(bool) = msg.value == TMP_11104
TMP_11106(None) = SOLIDITY_CALL require(bool,string)(TMP_11105,PuppyRaffle: Must send enough to enter raffle)
 i = 0
i_1(uint256) := 0(uint256)
 i < newPlayers.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_2312 -> LENGTH newPlayers_1
TMP_11107(bool) = i_2 < REF_2312
CONDITION TMP_11107
 players.push(newPlayers[i])
REF_2314(address) -> newPlayers_1[i_2]
REF_2315 -> LENGTH players_1
TMP_11109(uint256) := REF_2315(uint256)
TMP_11110(uint256) = TMP_11109 + 1
players_2(address[]) := phi(['players_1'])
REF_2315(uint256) (->players_2) := TMP_11110(uint256)
REF_2316(address) -> players_2[TMP_11109]
players_3(address[]) := phi(['players_2'])
REF_2316(address) (->players_3) := REF_2314(address)
 i ++
TMP_11111(uint256) := i_2(uint256)
i_3(uint256) = i_2 + 1
 i_scope_0 = 0
i_scope_0_1(uint256) := 0(uint256)
 i_scope_0 < players.length - 1
i_scope_0_2(uint256) := phi(['i_scope_0_1', 'i_scope_0_3'])
REF_2317 -> LENGTH players_1
TMP_11112(uint256) = REF_2317 - 1
TMP_11113(bool) = i_scope_0_2 < TMP_11112
CONDITION TMP_11113
 j = i_scope_0 + 1
TMP_11114(uint256) = i_scope_0_2 + 1
j_1(uint256) := TMP_11114(uint256)
 j < players.length
j_2(uint256) := phi(['j_1', 'j_3'])
REF_2318 -> LENGTH players_1
TMP_11115(bool) = j_2 < REF_2318
CONDITION TMP_11115
 require(bool,string)(players[i_scope_0] != players[j],PuppyRaffle: Duplicate player)
REF_2319(address) -> players_1[i_scope_0_2]
REF_2320(address) -> players_1[j_2]
TMP_11116(bool) = REF_2319 != REF_2320
TMP_11117(None) = SOLIDITY_CALL require(bool,string)(TMP_11116,PuppyRaffle: Duplicate player)
 j ++
TMP_11118(uint256) := j_2(uint256)
j_3(uint256) = j_2 + 1
 i_scope_0 ++
TMP_11119(uint256) := i_scope_0_2(uint256)
i_scope_0_3(uint256) = i_scope_0_2 + 1
 RaffleEnter(newPlayers)
Emit RaffleEnter(newPlayers_1)
```
#### PuppyRaffle.getActivePlayerIndex(address) [EXTERNAL]
```slithir
players_6(address[]) := phi(['players_9', 'players_1', 'players_5', 'players_10', 'players_0', 'players_6'])
 i = 0
i_1(uint256) := 0(uint256)
 i < players.length
i_2(uint256) := phi(['i_3', 'i_1'])
REF_2324 -> LENGTH players_6
TMP_11130(bool) = i_2 < REF_2324
CONDITION TMP_11130
 players[i] == player
REF_2325(address) -> players_6[i_2]
TMP_11131(bool) = REF_2325 == player_1
CONDITION TMP_11131
 i
RETURN i_2
 i ++
TMP_11132(uint256) := i_2(uint256)
i_3(uint256) = i_2 + 1
 0
RETURN 0
```
#### PuppyRaffle.refund(uint256) [PUBLIC]
```slithir
entranceFee_3(uint256) := phi(['entranceFee_1', 'entranceFee_0'])
players_4(address[]) := phi(['players_9', 'players_1', 'players_5', 'players_10', 'players_0', 'players_6'])
 playerAddress = players[playerIndex]
REF_2321(address) -> players_4[playerIndex_1]
playerAddress_1(address) := REF_2321(address)
 require(bool,string)(playerAddress == msg.sender,PuppyRaffle: Only the player can refund)
TMP_11121(bool) = playerAddress_1 == msg.sender
TMP_11122(None) = SOLIDITY_CALL require(bool,string)(TMP_11121,PuppyRaffle: Only the player can refund)
 require(bool,string)(playerAddress != address(0),PuppyRaffle: Player already refunded, or is not active)
TMP_11123 = CONVERT 0 to address
TMP_11124(bool) = playerAddress_1 != TMP_11123
TMP_11125(None) = SOLIDITY_CALL require(bool,string)(TMP_11124,PuppyRaffle: Player already refunded, or is not active)
 address(msg.sender).sendValue(entranceFee)
TMP_11126 = CONVERT msg.sender to address
LIBRARY_CALL, dest:Address, function:Address.sendValue(address,uint256), arguments:['TMP_11126', 'entranceFee_3'] 
 players[playerIndex] = address(0)
REF_2323(address) -> players_4[playerIndex_1]
TMP_11128 = CONVERT 0 to address
players_5(address[]) := phi(['players_4'])
REF_2323(address) (->players_5) := TMP_11128(address)
 RaffleRefunded(playerAddress)
Emit RaffleRefunded(playerAddress_1)
```
#### PuppyRaffle.selectWinner() [EXTERNAL]
```slithir
entranceFee_4(uint256) := phi(['entranceFee_1', 'entranceFee_0'])
players_7(address[]) := phi(['players_9', 'players_1', 'players_5', 'players_10', 'players_0', 'players_6'])
raffleDuration_2(uint256) := phi(['raffleDuration_0', 'raffleDuration_1'])
raffleStartTime_2(uint256) := phi(['raffleStartTime_1', 'raffleStartTime_3', 'raffleStartTime_0'])
totalFees_1(uint64) := phi(['totalFees_4', 'totalFees_2', 'totalFees_0'])
COMMON_RARITY_3(uint256) := phi(['COMMON_RARITY_4', 'COMMON_RARITY_0', 'COMMON_RARITY_2'])
RARE_RARITY_3(uint256) := phi(['RARE_RARITY_2', 'RARE_RARITY_4', 'RARE_RARITY_0'])
LEGENDARY_RARITY_3(uint256) := phi(['LEGENDARY_RARITY_0', 'LEGENDARY_RARITY_2', 'LEGENDARY_RARITY_4'])
 require(bool,string)(block.timestamp >= raffleStartTime + raffleDuration,PuppyRaffle: Raffle not over)
TMP_11133(uint256) = raffleStartTime_2 + raffleDuration_2
TMP_11134(bool) = block.timestamp >= TMP_11133
TMP_11135(None) = SOLIDITY_CALL require(bool,string)(TMP_11134,PuppyRaffle: Raffle not over)
 require(bool,string)(players.length >= 4,PuppyRaffle: Need at least 4 players)
REF_2326 -> LENGTH players_7
TMP_11136(bool) = REF_2326 >= 4
TMP_11137(None) = SOLIDITY_CALL require(bool,string)(TMP_11136,PuppyRaffle: Need at least 4 players)
 winnerIndex = uint256(keccak256(bytes)(abi.encodePacked(msg.sender,block.timestamp,block.difficulty))) % players.length
TMP_11138(bytes) = SOLIDITY_CALL abi.encodePacked()(msg.sender,block.timestamp,block.difficulty)
TMP_11139(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_11138)
TMP_11140 = CONVERT TMP_11139 to uint256
REF_2328 -> LENGTH players_7
TMP_11141(uint256) = TMP_11140 % REF_2328
winnerIndex_1(uint256) := TMP_11141(uint256)
 winner = players[winnerIndex]
REF_2329(address) -> players_7[winnerIndex_1]
winner_1(address) := REF_2329(address)
 totalAmountCollected = players.length * entranceFee
REF_2330 -> LENGTH players_7
TMP_11142(uint256) = REF_2330 * entranceFee_4
totalAmountCollected_1(uint256) := TMP_11142(uint256)
 prizePool = (totalAmountCollected * 80) / 100
TMP_11143(uint256) = totalAmountCollected_1 * 80
TMP_11144(uint256) = TMP_11143 / 100
prizePool_1(uint256) := TMP_11144(uint256)
 fee = (totalAmountCollected * 20) / 100
TMP_11145(uint256) = totalAmountCollected_1 * 20
TMP_11146(uint256) = TMP_11145 / 100
fee_1(uint256) := TMP_11146(uint256)
 totalFees = totalFees + uint64(fee)
TMP_11147 = CONVERT fee_1 to uint64
TMP_11148(uint64) = totalFees_1 + TMP_11147
totalFees_2(uint64) := TMP_11148(uint64)
 tokenId = totalSupply()
TMP_11149(uint256) = INTERNAL_CALL, ERC721.totalSupply()()
tokenId_1(uint256) := TMP_11149(uint256)
 rarity = uint256(keccak256(bytes)(abi.encodePacked(msg.sender,block.difficulty))) % 100
TMP_11150(bytes) = SOLIDITY_CALL abi.encodePacked()(msg.sender,block.difficulty)
TMP_11151(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_11150)
TMP_11152 = CONVERT TMP_11151 to uint256
TMP_11153(uint256) = TMP_11152 % 100
rarity_1(uint256) := TMP_11153(uint256)
 rarity <= COMMON_RARITY
TMP_11154(bool) = rarity_1 <= COMMON_RARITY_4
CONDITION TMP_11154
 tokenIdToRarity[tokenId] = COMMON_RARITY
REF_2332(uint256) -> tokenIdToRarity_0[tokenId_1]
tokenIdToRarity_1(mapping(uint256 => uint256)) := phi(['tokenIdToRarity_0'])
REF_2332(uint256) (->tokenIdToRarity_1) := COMMON_RARITY_4(uint256)
 rarity <= COMMON_RARITY + RARE_RARITY
TMP_11155(uint256) = COMMON_RARITY_4 + RARE_RARITY_4
TMP_11156(bool) = rarity_1 <= TMP_11155
CONDITION TMP_11156
 tokenIdToRarity[tokenId] = RARE_RARITY
REF_2333(uint256) -> tokenIdToRarity_0[tokenId_1]
tokenIdToRarity_2(mapping(uint256 => uint256)) := phi(['tokenIdToRarity_0'])
REF_2333(uint256) (->tokenIdToRarity_2) := RARE_RARITY_4(uint256)
 tokenIdToRarity[tokenId] = LEGENDARY_RARITY
REF_2334(uint256) -> tokenIdToRarity_0[tokenId_1]
tokenIdToRarity_3(mapping(uint256 => uint256)) := phi(['tokenIdToRarity_0'])
REF_2334(uint256) (->tokenIdToRarity_3) := LEGENDARY_RARITY_4(uint256)
tokenIdToRarity_4(mapping(uint256 => uint256)) := phi(['tokenIdToRarity_2', 'tokenIdToRarity_3'])
 delete players
players_9 = delete players_8 
 raffleStartTime = block.timestamp
raffleStartTime_3(uint256) := block.timestamp(uint256)
 previousWinner = winner
previousWinner_1(address) := winner_1(address)
 (success,None) = winner.call{value: prizePool}()
TUPLE_54(bool,bytes) = LOW_LEVEL_CALL, dest:winner_1, function:call, arguments:[''] value:prizePool_1 
success_1(bool)= UNPACK TUPLE_54 index: 0 
 require(bool,string)(success,PuppyRaffle: Failed to send prize pool to winner)
TMP_11157(None) = SOLIDITY_CALL require(bool,string)(success_1,PuppyRaffle: Failed to send prize pool to winner)
 _safeMint(winner,tokenId)
INTERNAL_CALL, ERC721._safeMint(address,uint256)(winner_1,tokenId_1)
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
TMP_11179(address) = INTERNAL_CALL, Ownable.owner()()
TMP_11180(address) = INTERNAL_CALL, Context._msgSender()()
TMP_11181(bool) = TMP_11179 == TMP_11180
TMP_11182(None) = SOLIDITY_CALL require(bool,string)(TMP_11181,Ownable: caller is not the owner)
```
#### PuppyRaffle.slitherConstructorVariables() [INTERNAL]
```slithir
 totalFees = 0
 commonImageUri = ipfs://QmSsYRx3LpDAb1GZQm7zZ1AuHZjfbPkD6J7s9r41xu1mf8
 rareImageUri = ipfs://QmUPjADFGEKmfohdTaNcWhp7VGk26h5jXDA7v3VtTnTLcW
 legendaryImageUri = ipfs://QmYx6GsYAKnNzZ9A6NvEKV9nf1VaDzJrqDR23Y8YSkebLU
```
#### PuppyRaffle.tokenURI(uint256) [PUBLIC]
```slithir
tokenIdToRarity_5(mapping(uint256 => uint256)) := phi(['tokenIdToRarity_1', 'tokenIdToRarity_0', 'tokenIdToRarity_6', 'tokenIdToRarity_4'])
rarityToUri_4(mapping(uint256 => string)) := phi(['rarityToUri_3', 'rarityToUri_0', 'rarityToUri_5'])
rarityToName_4(mapping(uint256 => string)) := phi(['rarityToName_0', 'rarityToName_3', 'rarityToName_5'])
 require(bool,string)(_exists(tokenId),PuppyRaffle: URI query for nonexistent token)
TMP_11170(bool) = INTERNAL_CALL, ERC721._exists(uint256)(tokenId_1)
TMP_11171(None) = SOLIDITY_CALL require(bool,string)(TMP_11170,PuppyRaffle: URI query for nonexistent token)
 rarity = tokenIdToRarity[tokenId]
REF_2339(uint256) -> tokenIdToRarity_6[tokenId_1]
rarity_1(uint256) := REF_2339(uint256)
 imageURI = rarityToUri[rarity]
REF_2340(string) -> rarityToUri_5[rarity_1]
imageURI_1(string) := REF_2340(string)
 rareName = rarityToName[rarity]
REF_2341(string) -> rarityToName_5[rarity_1]
rareName_1(string) := REF_2341(string)
 string(abi.encodePacked(_baseURI(),Base64.encode(bytes(abi.encodePacked({"name":",name(),", "description":"An adorable puppy!", ,"attributes": [{"trait_type": "rarity", "value": ,rareName,}], "image":",imageURI,"})))))
TMP_11172(string) = INTERNAL_CALL, PuppyRaffle._baseURI()()
TMP_11173(string) = INTERNAL_CALL, ERC721.name()()
TMP_11174(bytes) = SOLIDITY_CALL abi.encodePacked()({"name":",TMP_11173,", "description":"An adorable puppy!", ,"attributes": [{"trait_type": "rarity", "value": ,rareName_1,}], "image":",imageURI_1,"})
TMP_11175 = CONVERT TMP_11174 to bytes
TMP_11176(string) = LIBRARY_CALL, dest:Base64, function:Base64.encode(bytes), arguments:['TMP_11175'] 
TMP_11177(bytes) = SOLIDITY_CALL abi.encodePacked()(TMP_11172,TMP_11176)
TMP_11178 = CONVERT TMP_11177 to string
RETURN TMP_11178
```
#### PuppyRaffle.withdrawFees() [EXTERNAL]
```slithir
feeAddress_2(address) := phi(['feeAddress_3', 'feeAddress_4', 'feeAddress_0', 'feeAddress_1'])
totalFees_3(uint64) := phi(['totalFees_4', 'totalFees_2', 'totalFees_0'])
 require(bool,string)(address(this).balance == uint256(totalFees),PuppyRaffle: There are currently players active!)
TMP_11159 = CONVERT this to address
TMP_11160(uint256) = SOLIDITY_CALL balance(address)(TMP_11159)
TMP_11161 = CONVERT totalFees_3 to uint256
TMP_11162(bool) = TMP_11160 == TMP_11161
TMP_11163(None) = SOLIDITY_CALL require(bool,string)(TMP_11162,PuppyRaffle: There are currently players active!)
 feesToWithdraw = totalFees
feesToWithdraw_1(uint256) := totalFees_3(uint64)
 totalFees = 0
totalFees_4(uint64) := 0(uint256)
 (success,None) = feeAddress.call{value: feesToWithdraw}()
TUPLE_55(bool,bytes) = LOW_LEVEL_CALL, dest:feeAddress_2, function:call, arguments:[''] value:feesToWithdraw_1 
feeAddress_3(address) := phi(['feeAddress_4', 'feeAddress_3', 'feeAddress_1', 'feeAddress_2'])
success_1(bool)= UNPACK TUPLE_55 index: 0 
 require(bool,string)(success,PuppyRaffle: Failed to withdraw fees)
TMP_11164(None) = SOLIDITY_CALL require(bool,string)(success_1,PuppyRaffle: Failed to withdraw fees)
```
#### Address.sendValue(address,uint256) [INTERNAL]
```slithir
 require(bool,string)(address(this).balance >= amount,Address: insufficient balance)
TMP_10329 = CONVERT this to address
TMP_10330(uint256) = SOLIDITY_CALL balance(address)(TMP_10329)
TMP_10331(bool) = TMP_10330 >= amount_1
TMP_10332(None) = SOLIDITY_CALL require(bool,string)(TMP_10331,Address: insufficient balance)
 (success,None) = recipient.call{value: amount}()
TUPLE_39(bool,bytes) = LOW_LEVEL_CALL, dest:recipient_1, function:call, arguments:[''] value:amount_1 
success_1(bool)= UNPACK TUPLE_39 index: 0 
 require(bool,string)(success,Address: unable to send value, recipient may have reverted)
TMP_10333(None) = SOLIDITY_CALL require(bool,string)(success_1,Address: unable to send value, recipient may have reverted)
```

