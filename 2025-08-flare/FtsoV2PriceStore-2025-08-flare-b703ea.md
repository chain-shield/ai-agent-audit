

### Storage layout (FtsoV2PriceStore) 

```text
firstVotingRoundStartTs uint64
votingEpochDurationSeconds uint64
submitTrustedPricesWindowSeconds uint64
ftsoProtocolId uint8
feedIds bytes21[]
symbolToFeedId mapping(string => bytes21)
feedIdToSymbol mapping(bytes21 => string)
latestPrices mapping(bytes21 => FtsoV2PriceStore.PriceStore)
submittedTrustedPrices mapping(bytes21 => mapping(uint32 => bytes))
lastVotingEpochIdByProvider mapping(address => uint256)
trustedProviders address[]
trustedProvidersMap mapping(address => bool)
trustedProvidersThreshold uint8
maxSpreadBIPS uint16
relay IRelay
lastPublishedVotingRoundId uint32

```
#### FtsoV2PriceStore._calculateMedian(bytes) [INTERNAL]
```slithir
_prices_1(bytes) := phi(['trustedPrices_1'])
MAX_BIPS_3(uint256) := phi(['MAX_BIPS_2', 'MAX_BIPS_0'])
maxSpreadBIPS_2(uint16) := phi(['maxSpreadBIPS_1', 'maxSpreadBIPS_0'])
 length = _prices.length
REF_5524 -> LENGTH _prices_1
length_1(uint256) := REF_5524(uint256)
 assert(bool)(length > 0 && length % 4 == 0)
TMP_8845(bool) = length_1 > 0
TMP_8846(uint256) = length_1 % 4
TMP_8847(bool) = TMP_8846 == 0
TMP_8848(bool) = TMP_8845 && TMP_8847
TMP_8849(None) = SOLIDITY_CALL assert(bool)(TMP_8848)
 length /= 4
length_2(uint256) = length_1 (c)/ 4
 prices = new uint256[](length)
TMP_8851(uint256[])  = new uint256[](length_2)
prices_1(uint256[]) = ['TMP_8851(uint256[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < length
prices_2(uint256[]) := phi(['prices_3', 'prices_1'])
i_2(uint256) := phi(['i_1', 'i_3'])
TMP_8852(bool) = i_2 < length_2
CONDITION TMP_8852
 price = new bytes(4)
TMP_8854 = new bytes(4)
price_1(bytes) := TMP_8854(bytes)
 j = 0
j_1(uint256) := 0(uint256)
 j < 4
j_2(uint256) := phi(['j_1', 'j_3'])
TMP_8855(bool) = j_2 < 4
CONDITION TMP_8855
 price[j] = _prices[i * 4 + j]
REF_5525(None) -> price_1[j_2]
TMP_8856(uint256) = i_2 (c)* 4
TMP_8857(uint256) = TMP_8856 (c)+ j_2
REF_5526(None) -> _prices_1[TMP_8857]
price_2(bytes) := phi(['price_1'])
REF_5525(None) (->price_2) := REF_5526(None)
 j ++
TMP_8858(uint256) := j_2(uint256)
j_3(uint256) = j_2 (c)+ 1
 prices[i] = uint32(bytes4(price))
REF_5527(uint256) -> prices_2[i_2]
TMP_8859 = CONVERT price_1 to bytes4
TMP_8860 = CONVERT TMP_8859 to uint32
prices_3(uint256[]) := phi(['prices_2'])
REF_5527(uint256) (->prices_3) := TMP_8860(uint32)
 i ++
TMP_8861(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 i_scope_0 = 1
i_scope_0_1(uint256) := 1(uint256)
 i_scope_0 < length
prices_4(uint256[]) := phi(['prices_6', 'prices_1'])
i_scope_0_2(uint256) := phi(['i_scope_0_1', 'i_scope_0_3'])
TMP_8862(bool) = i_scope_0_2 < length_2
CONDITION TMP_8862
 currentPrice = prices[i_scope_0]
REF_5528(uint256) -> prices_4[i_scope_0_2]
currentPrice_1(uint256) := REF_5528(uint256)
 j_scope_1 = i_scope_0
j_scope_1_1(uint256) := i_scope_0_2(uint256)
 j_scope_1 > 0 && prices[j_scope_1 - 1] > currentPrice
prices_5(uint256[]) := phi(['prices_7', 'prices_1'])
j_scope_1_2(uint256) := phi(['j_scope_1_3', 'j_scope_1_1'])
TMP_8863(bool) = j_scope_1_2 > 0
TMP_8864(uint256) = j_scope_1_2 (c)- 1
REF_5529(uint256) -> prices_5[TMP_8864]
TMP_8865(bool) = REF_5529 > currentPrice_1
TMP_8866(bool) = TMP_8863 && TMP_8865
CONDITION TMP_8866
 prices[j_scope_1] = prices[j_scope_1 - 1]
REF_5530(uint256) -> prices_5[j_scope_1_2]
TMP_8867(uint256) = j_scope_1_2 (c)- 1
REF_5531(uint256) -> prices_5[TMP_8867]
prices_7(uint256[]) := phi(['prices_5'])
REF_5530(uint256) (->prices_7) := REF_5531(uint256)
 j_scope_1 --
TMP_8868(uint256) := j_scope_1_2(uint256)
j_scope_1_3(uint256) = j_scope_1_2 (c)- 1
 prices[j_scope_1] = currentPrice
REF_5532(uint256) -> prices_5[j_scope_1_2]
prices_6(uint256[]) := phi(['prices_5'])
REF_5532(uint256) (->prices_6) := currentPrice_1(uint256)
 i_scope_0 ++
TMP_8869(uint256) := i_scope_0_2(uint256)
i_scope_0_3(uint256) = i_scope_0_2 (c)+ 1
 spread = 0
spread_1(uint256) := 0(uint256)
 middleIndex = length / 2
TMP_8870(uint256) = length_2 (c)/ 2
middleIndex_1(uint256) := TMP_8870(uint256)
 length % 2 == 1
TMP_8871(uint256) = length_2 % 2
TMP_8872(bool) = TMP_8871 == 1
CONDITION TMP_8872
 _medianPrice = prices[middleIndex]
REF_5533(uint256) -> prices_4[middleIndex_1]
_medianPrice_2(uint256) := REF_5533(uint256)
 length >= 3
TMP_8873(bool) = length_2 >= 3
CONDITION TMP_8873
 spread = (prices[middleIndex + 1] - prices[middleIndex - 1]) / 2
TMP_8874(uint256) = middleIndex_1 (c)+ 1
REF_5534(uint256) -> prices_4[TMP_8874]
TMP_8875(uint256) = middleIndex_1 (c)- 1
REF_5535(uint256) -> prices_4[TMP_8875]
TMP_8876(uint256) = REF_5534 (c)- REF_5535
TMP_8877(uint256) = TMP_8876 (c)/ 2
spread_3(uint256) := TMP_8877(uint256)
spread_4(uint256) := phi(['spread_3', 'spread_1'])
 _medianPrice = (prices[middleIndex - 1] + prices[middleIndex]) / 2
TMP_8878(uint256) = middleIndex_1 (c)- 1
REF_5536(uint256) -> prices_4[TMP_8878]
REF_5537(uint256) -> prices_4[middleIndex_1]
TMP_8879(uint256) = REF_5536 (c)+ REF_5537
TMP_8880(uint256) = TMP_8879 (c)/ 2
_medianPrice_1(uint256) := TMP_8880(uint256)
 spread = prices[middleIndex] - prices[middleIndex - 1]
REF_5538(uint256) -> prices_4[middleIndex_1]
TMP_8881(uint256) = middleIndex_1 (c)- 1
REF_5539(uint256) -> prices_4[TMP_8881]
TMP_8882(uint256) = REF_5538 (c)- REF_5539
spread_2(uint256) := TMP_8882(uint256)
_medianPrice_3(uint256) := phi(['_medianPrice_1', '_medianPrice_2'])
spread_5(uint256) := phi(['spread_1', 'spread_2'])
 _priceOk = spread <= maxSpreadBIPS * _medianPrice / MAX_BIPS
TMP_8883(uint16) = maxSpreadBIPS_2 (c)* _medianPrice_3
TMP_8884(uint16) = TMP_8883 (c)/ MAX_BIPS_3
TMP_8885(bool) = spread_5 <= TMP_8884
_priceOk_1(bool) := TMP_8885(bool)
 (_medianPrice,_priceOk)
RETURN _medianPrice_3,_priceOk_1
```
#### FtsoV2PriceStore._getEndTimestamp(uint256) [INTERNAL]
```slithir
_votingEpochId_1(uint256) := phi(['REF_5502', 'REF_5522', 'previousVotingEpochId_1', 'votingRoundId_2'])
firstVotingRoundStartTs_3(uint64) := phi(['firstVotingRoundStartTs_1', 'firstVotingRoundStartTs_0'])
votingEpochDurationSeconds_3(uint64) := phi(['votingEpochDurationSeconds_0', 'votingEpochDurationSeconds_1'])
 firstVotingRoundStartTs + (_votingEpochId + 1) * votingEpochDurationSeconds
TMP_8836(uint256) = _votingEpochId_1 (c)+ 1
TMP_8837(uint256) = TMP_8836 (c)* votingEpochDurationSeconds_3
TMP_8838(uint64) = firstVotingRoundStartTs_3 (c)+ TMP_8837
RETURN TMP_8838
```
#### FtsoV2PriceStore._getPreviousVotingEpochId() [INTERNAL]
```slithir
firstVotingRoundStartTs_2(uint64) := phi(['firstVotingRoundStartTs_1', 'firstVotingRoundStartTs_0'])
votingEpochDurationSeconds_2(uint64) := phi(['votingEpochDurationSeconds_0', 'votingEpochDurationSeconds_1'])
 uint32((block.timestamp - firstVotingRoundStartTs) / votingEpochDurationSeconds) - 1
TMP_8832(uint256) = block.timestamp (c)- firstVotingRoundStartTs_2
TMP_8833(uint256) = TMP_8832 (c)/ votingEpochDurationSeconds_2
TMP_8834 = CONVERT TMP_8833 to uint32
TMP_8835(uint32) = TMP_8834 (c)- 1
RETURN TMP_8835
```
#### FtsoV2PriceStore._getPriceFromTrustedProviders(FtsoV2PriceStore.PriceStore) [INTERNAL]
```slithir
_feed_1 (-> ['latestPrices', 'latestPrices'])(FtsoV2PriceStore.PriceStore) := phi(["feed_1 (-> ['latestPrices'])", "feed_1 (-> ['latestPrices'])"])
 _price = _feed.trustedValue
REF_5521(uint32) -> _feed_1 (-> ['latestPrices', 'latestPrices']).trustedValue
_price_1(uint256) := REF_5521(uint32)
 _timestamp = _getEndTimestamp(_feed.trustedVotingRoundId)
REF_5522(uint32) -> _feed_1 (-> ['latestPrices', 'latestPrices']).trustedVotingRoundId
TMP_8839(uint256) = INTERNAL_CALL, FtsoV2PriceStore._getEndTimestamp(uint256)(REF_5522)
_timestamp_1(uint256) := TMP_8839(uint256)
 decimals = _feed.trustedDecimals
REF_5523(int8) -> _feed_1 (-> ['latestPrices', 'latestPrices']).trustedDecimals
decimals_1(int256) := REF_5523(int8)
 decimals < 0
TMP_8840(bool) = decimals_1 < 0
CONDITION TMP_8840
 _priceDecimals = 0
_priceDecimals_2(uint256) := 0(uint256)
 _price *= 10 ** uint256(- decimals)
TMP_8841(int256) = 0 (c)- decimals_1
TMP_8842 = CONVERT TMP_8841 to uint256
TMP_8843(uint256) = 10 (c)** TMP_8842
_price_2(uint256) = _price_1 (c)* TMP_8843
 _priceDecimals = uint256(decimals)
TMP_8844 = CONVERT decimals_1 to uint256
_priceDecimals_1(uint256) := TMP_8844(uint256)
_price_3(uint256) := phi(['_price_2', '_price_1'])
_priceDecimals_3(uint256) := phi(['_priceDecimals_1', '_priceDecimals_2'])
 (_price,_timestamp,_priceDecimals)
RETURN _price_3,_timestamp_1,_priceDecimals_3
```
#### FtsoV2PriceStore._updateContractAddresses(bytes32[],address[]) [INTERNAL]
```slithir
_contractNameHashes_1(bytes32[]) := phi(['_contractNameHashes_1'])
_contractAddresses_1(address[]) := phi(['_contractAddresses_1'])
 relay = IRelay(_getContractAddress(_contractNameHashes,_contractAddresses,Relay))
TMP_8830(address) = INTERNAL_CALL, AddressUpdatable._getContractAddress(bytes32[],address[],string)(_contractNameHashes_1,_contractAddresses_1,Relay)
TMP_8831 = CONVERT TMP_8830 to IRelay
relay_5(IRelay) := TMP_8831(IRelay)
```
#### FtsoV2PriceStore.constructor() [PUBLIC]
```slithir
 GovernedUUPSProxyImplementation()
INTERNAL_CALL, GovernedUUPSProxyImplementation.constructor()()
 AddressUpdatable(address(0))
TMP_8696 = CONVERT 0 to address
INTERNAL_CALL, AddressUpdatable.constructor(address)(TMP_8696)
```
#### FtsoV2PriceStore.getFeedId(string) [EXTERNAL]
```slithir
symbolToFeedId_5(mapping(string => bytes21)) := phi(['symbolToFeedId_5', 'symbolToFeedId_4', 'symbolToFeedId_0', 'symbolToFeedId_3', 'symbolToFeedId_1', 'symbolToFeedId_2'])
 symbolToFeedId[_symbol]
REF_5520(bytes21) -> symbolToFeedId_5[_symbol_1]
RETURN REF_5520
```
#### FtsoV2PriceStore.getFeedIds() [EXTERNAL]
```slithir
feedIds_9(bytes21[]) := phi(['feedIds_0', 'feedIds_7', 'feedIds_2', 'feedIds_8', 'feedIds_11', 'feedIds_3', 'feedIds_1'])
 feedIds
RETURN feedIds_9
```
#### FtsoV2PriceStore.getFeedIdsWithDecimals() [EXTERNAL]
```slithir
feedIds_10(bytes21[]) := phi(['feedIds_0', 'feedIds_7', 'feedIds_2', 'feedIds_8', 'feedIds_11', 'feedIds_3', 'feedIds_1'])
latestPrices_23(mapping(bytes21 => FtsoV2PriceStore.PriceStore)) := phi(['latestPrices_21', 'latestPrices_20', 'latestPrices_23', 'latestPrices_15', 'latestPrices_0', 'latestPrices_13', 'latestPrices_22', 'latestPrices_7'])
 _feedIds = feedIds
_feedIds_1(bytes21[]) := feedIds_10(bytes21[])
 _decimals = new int8[](_feedIds.length)
REF_5509 -> LENGTH _feedIds_1
TMP_8823(int8[])  = new int8[](REF_5509)
_decimals_1(int8[]) = ['TMP_8823(int8[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < _feedIds.length
i_2(uint256) := phi(['i_3', 'i_1'])
REF_5510 -> LENGTH _feedIds_1
TMP_8824(bool) = i_2 < REF_5510
CONDITION TMP_8824
 _decimals[i] = latestPrices[_feedIds[i]].trustedDecimals
REF_5511(int8) -> _decimals_1[i_2]
REF_5512(bytes21) -> _feedIds_1[i_2]
REF_5513(FtsoV2PriceStore.PriceStore) -> latestPrices_23[REF_5512]
REF_5514(int8) -> REF_5513.trustedDecimals
_decimals_2(int8[]) := phi(['_decimals_1'])
REF_5511(int8) (->_decimals_2) := REF_5514(int8)
 i ++
TMP_8825(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 (_feedIds,_decimals)
RETURN _feedIds_1,_decimals_1
```
#### FtsoV2PriceStore.getPrice(string) [EXTERNAL]
```slithir
symbolToFeedId_2(mapping(string => bytes21)) := phi(['symbolToFeedId_5', 'symbolToFeedId_4', 'symbolToFeedId_0', 'symbolToFeedId_3', 'symbolToFeedId_1', 'symbolToFeedId_2'])
latestPrices_20(mapping(bytes21 => FtsoV2PriceStore.PriceStore)) := phi(['latestPrices_21', 'latestPrices_20', 'latestPrices_23', 'latestPrices_15', 'latestPrices_0', 'latestPrices_13', 'latestPrices_22', 'latestPrices_7'])
 feedId = symbolToFeedId[_symbol]
REF_5499(bytes21) -> symbolToFeedId_2[_symbol_1]
feedId_1(bytes21) := REF_5499(bytes21)
 require(bool,error)(feedId != bytes21(0),revert SymbolNotSupported()())
TMP_8804 = CONVERT 0 to bytes21
TMP_8805(bool) = feedId_1 != TMP_8804
TMP_8806(None) = SOLIDITY_CALL revert SymbolNotSupported()()
TMP_8807(None) = SOLIDITY_CALL require(bool,error)(TMP_8805,TMP_8806)
 feed = latestPrices[feedId]
REF_5500(FtsoV2PriceStore.PriceStore) -> latestPrices_20[feedId_1]
feed_1 (-> ['latestPrices'])(FtsoV2PriceStore.PriceStore) := REF_5500(FtsoV2PriceStore.PriceStore)
 _price = feed.value
REF_5501(uint32) -> feed_1 (-> ['latestPrices']).value
_price_1(uint256) := REF_5501(uint32)
 _timestamp = _getEndTimestamp(feed.votingRoundId)
REF_5502(uint32) -> feed_1 (-> ['latestPrices']).votingRoundId
TMP_8808(uint256) = INTERNAL_CALL, FtsoV2PriceStore._getEndTimestamp(uint256)(REF_5502)
_timestamp_1(uint256) := TMP_8808(uint256)
 decimals = feed.decimals
REF_5503(int8) -> feed_1 (-> ['latestPrices']).decimals
decimals_1(int256) := REF_5503(int8)
 decimals < 0
TMP_8809(bool) = decimals_1 < 0
CONDITION TMP_8809
 _priceDecimals = 0
_priceDecimals_2(uint256) := 0(uint256)
 _price *= 10 ** uint256(- decimals)
TMP_8810(int256) = 0 (c)- decimals_1
TMP_8811 = CONVERT TMP_8810 to uint256
TMP_8812(uint256) = 10 (c)** TMP_8811
_price_2(uint256) = _price_1 (c)* TMP_8812
 _priceDecimals = uint256(decimals)
TMP_8813 = CONVERT decimals_1 to uint256
_priceDecimals_1(uint256) := TMP_8813(uint256)
_price_3(uint256) := phi(['_price_2', '_price_1'])
_priceDecimals_3(uint256) := phi(['_priceDecimals_1', '_priceDecimals_2'])
 (_price,_timestamp,_priceDecimals)
RETURN _price_3,_timestamp_1,_priceDecimals_3
```
#### FtsoV2PriceStore.getPriceFromTrustedProviders(string) [EXTERNAL]
```slithir
symbolToFeedId_3(mapping(string => bytes21)) := phi(['symbolToFeedId_5', 'symbolToFeedId_4', 'symbolToFeedId_0', 'symbolToFeedId_3', 'symbolToFeedId_1', 'symbolToFeedId_2'])
latestPrices_21(mapping(bytes21 => FtsoV2PriceStore.PriceStore)) := phi(['latestPrices_21', 'latestPrices_20', 'latestPrices_23', 'latestPrices_15', 'latestPrices_0', 'latestPrices_13', 'latestPrices_22', 'latestPrices_7'])
 feedId = symbolToFeedId[_symbol]
REF_5504(bytes21) -> symbolToFeedId_3[_symbol_1]
feedId_1(bytes21) := REF_5504(bytes21)
 require(bool,error)(feedId != bytes21(0),revert SymbolNotSupported()())
TMP_8814 = CONVERT 0 to bytes21
TMP_8815(bool) = feedId_1 != TMP_8814
TMP_8816(None) = SOLIDITY_CALL revert SymbolNotSupported()()
TMP_8817(None) = SOLIDITY_CALL require(bool,error)(TMP_8815,TMP_8816)
 feed = latestPrices[feedId]
REF_5505(FtsoV2PriceStore.PriceStore) -> latestPrices_21[feedId_1]
feed_1 (-> ['latestPrices'])(FtsoV2PriceStore.PriceStore) := REF_5505(FtsoV2PriceStore.PriceStore)
 (_price,_timestamp,_priceDecimals) = _getPriceFromTrustedProviders(feed)
TUPLE_77(uint256,uint256,uint256) = INTERNAL_CALL, FtsoV2PriceStore._getPriceFromTrustedProviders(FtsoV2PriceStore.PriceStore)(feed_1 (-> ['latestPrices']))
_price_1(uint256)= UNPACK TUPLE_77 index: 0 
_timestamp_1(uint256)= UNPACK TUPLE_77 index: 1 
_priceDecimals_1(uint256)= UNPACK TUPLE_77 index: 2 
 (_price,_timestamp,_priceDecimals)
RETURN _price_1,_timestamp_1,_priceDecimals_1
```
#### FtsoV2PriceStore.getPriceFromTrustedProvidersWithQuality(string) [EXTERNAL]
```slithir
symbolToFeedId_4(mapping(string => bytes21)) := phi(['symbolToFeedId_5', 'symbolToFeedId_4', 'symbolToFeedId_0', 'symbolToFeedId_3', 'symbolToFeedId_1', 'symbolToFeedId_2'])
latestPrices_22(mapping(bytes21 => FtsoV2PriceStore.PriceStore)) := phi(['latestPrices_21', 'latestPrices_20', 'latestPrices_23', 'latestPrices_15', 'latestPrices_0', 'latestPrices_13', 'latestPrices_22', 'latestPrices_7'])
 feedId = symbolToFeedId[_symbol]
REF_5506(bytes21) -> symbolToFeedId_4[_symbol_1]
feedId_1(bytes21) := REF_5506(bytes21)
 require(bool,error)(feedId != bytes21(0),revert SymbolNotSupported()())
TMP_8818 = CONVERT 0 to bytes21
TMP_8819(bool) = feedId_1 != TMP_8818
TMP_8820(None) = SOLIDITY_CALL revert SymbolNotSupported()()
TMP_8821(None) = SOLIDITY_CALL require(bool,error)(TMP_8819,TMP_8820)
 feed = latestPrices[feedId]
REF_5507(FtsoV2PriceStore.PriceStore) -> latestPrices_22[feedId_1]
feed_1 (-> ['latestPrices'])(FtsoV2PriceStore.PriceStore) := REF_5507(FtsoV2PriceStore.PriceStore)
 (_price,_timestamp,_priceDecimals) = _getPriceFromTrustedProviders(feed)
TUPLE_78(uint256,uint256,uint256) = INTERNAL_CALL, FtsoV2PriceStore._getPriceFromTrustedProviders(FtsoV2PriceStore.PriceStore)(feed_1 (-> ['latestPrices']))
_price_1(uint256)= UNPACK TUPLE_78 index: 0 
_timestamp_1(uint256)= UNPACK TUPLE_78 index: 1 
_priceDecimals_1(uint256)= UNPACK TUPLE_78 index: 2 
 _numberOfSubmits = feed.numberOfSubmits
REF_5508(uint8) -> feed_1 (-> ['latestPrices']).numberOfSubmits
_numberOfSubmits_1(uint8) := REF_5508(uint8)
 (_price,_timestamp,_priceDecimals,_numberOfSubmits)
RETURN _price_1,_timestamp_1,_priceDecimals_1,_numberOfSubmits_1
```
#### FtsoV2PriceStore.getSymbols() [EXTERNAL]
```slithir
feedIds_11(bytes21[]) := phi(['feedIds_0', 'feedIds_7', 'feedIds_2', 'feedIds_8', 'feedIds_11', 'feedIds_3', 'feedIds_1'])
feedIdToSymbol_2(mapping(bytes21 => string)) := phi(['feedIdToSymbol_0', 'feedIdToSymbol_1', 'feedIdToSymbol_2'])
 _symbols = new string[](feedIds.length)
REF_5515 -> LENGTH feedIds_11
TMP_8827(string[])  = new string[](REF_5515)
_symbols_1(string[]) = ['TMP_8827(string[])']
 i = 0
i_1(uint256) := 0(uint256)
 i < feedIds.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_5516 -> LENGTH feedIds_11
TMP_8828(bool) = i_2 < REF_5516
CONDITION TMP_8828
 _symbols[i] = feedIdToSymbol[feedIds[i]]
REF_5517(string) -> _symbols_1[i_2]
REF_5518(bytes21) -> feedIds_11[i_2]
REF_5519(string) -> feedIdToSymbol_2[REF_5518]
_symbols_2(string[]) := phi(['_symbols_1'])
REF_5517(string) (->_symbols_2) := REF_5519(string)
 i ++
TMP_8829(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 _symbols
RETURN _symbols_1
```
#### FtsoV2PriceStore.getTrustedProviders() [EXTERNAL]
```slithir
trustedProviders_4(address[]) := phi(['trustedProviders_3', 'trustedProviders_0'])
 trustedProviders
RETURN trustedProviders_4
```
#### FtsoV2PriceStore.initialize(IGovernanceSettings,address,address,uint64,uint8,uint8) [EXTERNAL]
```slithir
 require(bool,error)(_firstVotingRoundStartTs + _votingEpochDurationSeconds <= block.timestamp,revert InvalidStartTime()())
TMP_8698(uint64) = _firstVotingRoundStartTs_1 (c)+ _votingEpochDurationSeconds_1
TMP_8699(bool) = TMP_8698 <= block.timestamp
TMP_8700(None) = SOLIDITY_CALL revert InvalidStartTime()()
TMP_8701(None) = SOLIDITY_CALL require(bool,error)(TMP_8699,TMP_8700)
 require(bool,error)(_votingEpochDurationSeconds > 1,revert VotingEpochDurationTooShort()())
TMP_8702(bool) = _votingEpochDurationSeconds_1 > 1
TMP_8703(None) = SOLIDITY_CALL revert VotingEpochDurationTooShort()()
TMP_8704(None) = SOLIDITY_CALL require(bool,error)(TMP_8702,TMP_8703)
 initialise(_governanceSettings,_initialGovernance)
INTERNAL_CALL, GovernedBase.initialise(IGovernanceSettings,address)(_governanceSettings_1,_initialGovernance_1)
 setAddressUpdaterValue(_addressUpdater)
INTERNAL_CALL, AddressUpdatable.setAddressUpdaterValue(address)(_addressUpdater_1)
 firstVotingRoundStartTs = _firstVotingRoundStartTs
firstVotingRoundStartTs_1(uint64) := _firstVotingRoundStartTs_1(uint64)
 votingEpochDurationSeconds = _votingEpochDurationSeconds
votingEpochDurationSeconds_1(uint64) := _votingEpochDurationSeconds_1(uint8)
 submitTrustedPricesWindowSeconds = _votingEpochDurationSeconds / 2
TMP_8707(uint8) = _votingEpochDurationSeconds_1 (c)/ 2
submitTrustedPricesWindowSeconds_1(uint64) := TMP_8707(uint8)
 ftsoProtocolId = _ftsoProtocolId
ftsoProtocolId_1(uint8) := _ftsoProtocolId_1(uint8)
 lastPublishedVotingRoundId = _getPreviousVotingEpochId()
TMP_8708(uint32) = INTERNAL_CALL, FtsoV2PriceStore._getPreviousVotingEpochId()()
lastPublishedVotingRoundId_1(uint32) := TMP_8708(uint32)
```
#### FtsoV2PriceStore.publishPrices(IPricePublisher.FeedWithProof[]) [EXTERNAL]
```slithir
submitTrustedPricesWindowSeconds_2(uint64) := phi(['submitTrustedPricesWindowSeconds_8', 'submitTrustedPricesWindowSeconds_4', 'submitTrustedPricesWindowSeconds_1', 'submitTrustedPricesWindowSeconds_0'])
ftsoProtocolId_2(uint8) := phi(['ftsoProtocolId_0', 'ftsoProtocolId_1', 'ftsoProtocolId_4'])
feedIds_1(bytes21[]) := phi(['feedIds_0', 'feedIds_7', 'feedIds_2', 'feedIds_8', 'feedIds_11', 'feedIds_3', 'feedIds_1'])
latestPrices_1(mapping(bytes21 => FtsoV2PriceStore.PriceStore)) := phi(['latestPrices_21', 'latestPrices_20', 'latestPrices_23', 'latestPrices_15', 'latestPrices_0', 'latestPrices_13', 'latestPrices_22', 'latestPrices_7'])
submittedTrustedPrices_1(mapping(bytes21 => mapping(uint32 => bytes))) := phi(['submittedTrustedPrices_3', 'submittedTrustedPrices_7', 'submittedTrustedPrices_10', 'submittedTrustedPrices_0'])
trustedProvidersThreshold_1(uint8) := phi(['trustedProvidersThreshold_0', 'trustedProvidersThreshold_5', 'trustedProvidersThreshold_3'])
relay_1(IRelay) := phi(['relay_5', 'relay_0', 'relay_3'])
lastPublishedVotingRoundId_2(uint32) := phi(['lastPublishedVotingRoundId_1', 'lastPublishedVotingRoundId_4', 'lastPublishedVotingRoundId_0', 'lastPublishedVotingRoundId_7'])
 votingRoundId = 0
votingRoundId_1(uint32) := 0(uint256)
 require(bool,error)(_proofs.length == feedIds.length,revert WrongNumberOfProofs()())
REF_5423 -> LENGTH _proofs_1
REF_5424 -> LENGTH feedIds_1
TMP_8709(bool) = REF_5423 == REF_5424
TMP_8710(None) = SOLIDITY_CALL revert WrongNumberOfProofs()()
TMP_8711(None) = SOLIDITY_CALL require(bool,error)(TMP_8709,TMP_8710)
 i = 0
i_1(uint256) := 0(uint256)
 i < _proofs.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_5425 -> LENGTH _proofs_1
TMP_8712(bool) = i_2 < REF_5425
CONDITION TMP_8712
 proof = _proofs[i]
REF_5426(IPricePublisher.FeedWithProof) -> _proofs_1[i_2]
proof_1(IPricePublisher.FeedWithProof) := REF_5426(IPricePublisher.FeedWithProof)
 feed = proof.body
REF_5427(IPricePublisher.Feed) -> proof_1.body
feed_1(IPricePublisher.Feed) := REF_5427(IPricePublisher.Feed)
 i == 0
TMP_8713(bool) = i_2 == 0
CONDITION TMP_8713
 votingRoundId = feed.votingRoundId
REF_5428(uint32) -> feed_1.votingRoundId
votingRoundId_2(uint32) := REF_5428(uint32)
 require(bool,error)(votingRoundId > lastPublishedVotingRoundId,revert PricesAlreadyPublished()())
TMP_8714(bool) = votingRoundId_2 > lastPublishedVotingRoundId_2
TMP_8715(None) = SOLIDITY_CALL revert PricesAlreadyPublished()()
TMP_8716(None) = SOLIDITY_CALL require(bool,error)(TMP_8714,TMP_8715)
 require(bool,error)(_getEndTimestamp(votingRoundId) + submitTrustedPricesWindowSeconds <= block.timestamp,revert SubmissionWindowNotClosed()())
TMP_8717(uint256) = INTERNAL_CALL, FtsoV2PriceStore._getEndTimestamp(uint256)(votingRoundId_2)
TMP_8718(uint256) = TMP_8717 (c)+ submitTrustedPricesWindowSeconds_3
TMP_8719(bool) = TMP_8718 <= block.timestamp
TMP_8720(None) = SOLIDITY_CALL revert SubmissionWindowNotClosed()()
TMP_8721(None) = SOLIDITY_CALL require(bool,error)(TMP_8719,TMP_8720)
 lastPublishedVotingRoundId = votingRoundId
lastPublishedVotingRoundId_3(uint32) := votingRoundId_2(uint32)
 PricesPublished(votingRoundId)
Emit PricesPublished(votingRoundId_2)
 require(bool,error)(feed.votingRoundId == votingRoundId,revert VotingRoundIdMismatch()())
REF_5429(uint32) -> feed_1.votingRoundId
TMP_8723(bool) = REF_5429 == votingRoundId_1
TMP_8724(None) = SOLIDITY_CALL revert VotingRoundIdMismatch()()
TMP_8725(None) = SOLIDITY_CALL require(bool,error)(TMP_8723,TMP_8724)
votingRoundId_3(uint32) := phi(['votingRoundId_1', 'votingRoundId_2'])
 feedId = feedIds[i]
REF_5430(bytes21) -> feedIds_2[i_2]
feedId_1(bytes21) := REF_5430(bytes21)
 require(bool,error)(feed.id == feedId,revert FeedIdMismatch()())
REF_5431(bytes21) -> feed_1.id
TMP_8726(bool) = REF_5431 == feedId_1
TMP_8727(None) = SOLIDITY_CALL revert FeedIdMismatch()()
TMP_8728(None) = SOLIDITY_CALL require(bool,error)(TMP_8726,TMP_8727)
 require(bool,error)(feed.value >= 0,revert ValueMustBeNonNegative()())
REF_5432(int32) -> feed_1.value
TMP_8729(bool) = REF_5432 >= 0
TMP_8730(None) = SOLIDITY_CALL revert ValueMustBeNonNegative()()
TMP_8731(None) = SOLIDITY_CALL require(bool,error)(TMP_8729,TMP_8730)
 feedHash = keccak256(bytes)(abi.encode(feed))
TMP_8732(bytes) = SOLIDITY_CALL abi.encode()(feed_1)
TMP_8733(bytes32) = SOLIDITY_CALL keccak256(bytes)(TMP_8732)
feedHash_1(bytes32) := TMP_8733(bytes32)
 merkleRoot = relay.merkleRoots(ftsoProtocolId,votingRoundId)
TMP_8734(bytes32) = HIGH_LEVEL_CALL, dest:relay_2(IRelay), function:merkleRoots, arguments:['ftsoProtocolId_3', 'votingRoundId_3']  
submitTrustedPricesWindowSeconds_4(uint64) := phi(['submitTrustedPricesWindowSeconds_8', 'submitTrustedPricesWindowSeconds_4', 'submitTrustedPricesWindowSeconds_1', 'submitTrustedPricesWindowSeconds_3'])
ftsoProtocolId_4(uint8) := phi(['ftsoProtocolId_1', 'ftsoProtocolId_3', 'ftsoProtocolId_4'])
feedIds_3(bytes21[]) := phi(['feedIds_7', 'feedIds_2', 'feedIds_8', 'feedIds_11', 'feedIds_3', 'feedIds_1'])
latestPrices_3(mapping(bytes21 => FtsoV2PriceStore.PriceStore)) := phi(['latestPrices_21', 'latestPrices_20', 'latestPrices_23', 'latestPrices_15', 'latestPrices_13', 'latestPrices_2', 'latestPrices_22', 'latestPrices_7'])
submittedTrustedPrices_3(mapping(bytes21 => mapping(uint32 => bytes))) := phi(['submittedTrustedPrices_3', 'submittedTrustedPrices_7', 'submittedTrustedPrices_10', 'submittedTrustedPrices_2'])
trustedProvidersThreshold_3(uint8) := phi(['trustedProvidersThreshold_2', 'trustedProvidersThreshold_5', 'trustedProvidersThreshold_3'])
relay_3(IRelay) := phi(['relay_5', 'relay_2', 'relay_3'])
lastPublishedVotingRoundId_4(uint32) := phi(['lastPublishedVotingRoundId_1', 'lastPublishedVotingRoundId_4', 'lastPublishedVotingRoundId_3', 'lastPublishedVotingRoundId_7'])
merkleRoot_1(bytes32) := TMP_8734(bytes32)
 require(bool,error)(proof.proof.verifyCalldata(merkleRoot,feedHash),revert MerkleProofInvalid()())
REF_5435(bytes32[]) -> proof_1.proof
TMP_8735(bool) = LIBRARY_CALL, dest:MerkleProof, function:MerkleProof.verifyCalldata(bytes32[],bytes32,bytes32), arguments:['REF_5435', 'merkleRoot_1', 'feedHash_1'] 
TMP_8736(None) = SOLIDITY_CALL revert MerkleProofInvalid()()
TMP_8737(None) = SOLIDITY_CALL require(bool,error)(TMP_8735,TMP_8736)
 priceStore = latestPrices[feedId]
REF_5437(FtsoV2PriceStore.PriceStore) -> latestPrices_3[feedId_1]
priceStore_1 (-> ['latestPrices'])(FtsoV2PriceStore.PriceStore) := REF_5437(FtsoV2PriceStore.PriceStore)
 priceStore.votingRoundId = feed.votingRoundId
REF_5438(uint32) -> priceStore_1 (-> ['latestPrices']).votingRoundId
REF_5439(uint32) -> feed_1.votingRoundId
priceStore_2 (-> ['latestPrices'])(FtsoV2PriceStore.PriceStore) := phi(["priceStore_1 (-> ['latestPrices'])"])
REF_5438(uint32) (->priceStore_2 (-> ['latestPrices'])) := REF_5439(uint32)
latestPrices_5(mapping(bytes21 => FtsoV2PriceStore.PriceStore)) := phi(["priceStore_2 (-> ['latestPrices'])"])
 priceStore.value = uint32(feed.value)
REF_5440(uint32) -> priceStore_2 (-> ['latestPrices']).value
REF_5441(int32) -> feed_1.value
TMP_8738 = CONVERT REF_5441 to uint32
priceStore_3 (-> ['latestPrices'])(FtsoV2PriceStore.PriceStore) := phi(["priceStore_2 (-> ['latestPrices'])"])
REF_5440(uint32) (->priceStore_3 (-> ['latestPrices'])) := TMP_8738(uint32)
latestPrices_6(mapping(bytes21 => FtsoV2PriceStore.PriceStore)) := phi(["priceStore_3 (-> ['latestPrices'])"])
 priceStore.decimals = feed.decimals
REF_5442(int8) -> priceStore_3 (-> ['latestPrices']).decimals
REF_5443(int8) -> feed_1.decimals
priceStore_4 (-> ['latestPrices'])(FtsoV2PriceStore.PriceStore) := phi(["priceStore_3 (-> ['latestPrices'])"])
REF_5442(int8) (->priceStore_4 (-> ['latestPrices'])) := REF_5443(int8)
latestPrices_7(mapping(bytes21 => FtsoV2PriceStore.PriceStore)) := phi(["priceStore_4 (-> ['latestPrices'])"])
 trustedPrices = submittedTrustedPrices[feedId][votingRoundId]
REF_5444(mapping(uint32 => bytes)) -> submittedTrustedPrices_3[feedId_1]
REF_5445(bytes) -> REF_5444[votingRoundId_3]
trustedPrices_1(bytes) := REF_5445(bytes)
 trustedPrices.length > 0 && trustedPrices.length >= 4 * trustedProvidersThreshold
REF_5446 -> LENGTH trustedPrices_1
TMP_8739(bool) = REF_5446 > 0
REF_5447 -> LENGTH trustedPrices_1
TMP_8740(uint256) = 4 (c)* trustedProvidersThreshold_3
TMP_8741(bool) = REF_5447 >= TMP_8740
TMP_8742(bool) = TMP_8739 && TMP_8741
CONDITION TMP_8742
 (medianPrice,priceOk) = _calculateMedian(trustedPrices)
TUPLE_76(uint256,bool) = INTERNAL_CALL, FtsoV2PriceStore._calculateMedian(bytes)(trustedPrices_1)
medianPrice_1(uint256)= UNPACK TUPLE_76 index: 0 
priceOk_1(bool)= UNPACK TUPLE_76 index: 1 
 priceOk
CONDITION priceOk_1
 priceStore.trustedVotingRoundId = votingRoundId
REF_5448(uint32) -> priceStore_4 (-> ['latestPrices']).trustedVotingRoundId
priceStore_5 (-> ['latestPrices'])(FtsoV2PriceStore.PriceStore) := phi(["priceStore_4 (-> ['latestPrices'])"])
REF_5448(uint32) (->priceStore_5 (-> ['latestPrices'])) := votingRoundId_3(uint32)
latestPrices_8(mapping(bytes21 => FtsoV2PriceStore.PriceStore)) := phi(["priceStore_5 (-> ['latestPrices'])"])
 priceStore.trustedValue = uint32(medianPrice)
REF_5449(uint32) -> priceStore_5 (-> ['latestPrices']).trustedValue
TMP_8743 = CONVERT medianPrice_1 to uint32
priceStore_6 (-> ['latestPrices'])(FtsoV2PriceStore.PriceStore) := phi(["priceStore_5 (-> ['latestPrices'])"])
REF_5449(uint32) (->priceStore_6 (-> ['latestPrices'])) := TMP_8743(uint32)
latestPrices_9(mapping(bytes21 => FtsoV2PriceStore.PriceStore)) := phi(["priceStore_6 (-> ['latestPrices'])"])
 priceStore.numberOfSubmits = uint8(trustedPrices.length / 4)
REF_5450(uint8) -> priceStore_6 (-> ['latestPrices']).numberOfSubmits
REF_5451 -> LENGTH trustedPrices_1
TMP_8744(uint256) = REF_5451 (c)/ 4
TMP_8745 = CONVERT TMP_8744 to uint8
priceStore_7 (-> ['latestPrices'])(FtsoV2PriceStore.PriceStore) := phi(["priceStore_6 (-> ['latestPrices'])"])
REF_5450(uint8) (->priceStore_7 (-> ['latestPrices'])) := TMP_8745(uint8)
latestPrices_10(mapping(bytes21 => FtsoV2PriceStore.PriceStore)) := phi(["priceStore_7 (-> ['latestPrices'])"])
 delete submittedTrustedPrices[feedId][votingRoundId]
REF_5452(mapping(uint32 => bytes)) -> submittedTrustedPrices_4[feedId_1]
REF_5453(bytes) -> REF_5452[votingRoundId_3]
REF_5452 = delete REF_5453 
 i ++
TMP_8746(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
```
#### FtsoV2PriceStore.setTrustedProviders(address[],uint8) [EXTERNAL]
```slithir
trustedProviders_1(address[]) := phi(['trustedProviders_3', 'trustedProviders_0'])
 require(bool,error)(_trustedProviders.length < 2 ** 8,revert TooManyTrustedProviders()())
REF_5491 -> LENGTH _trustedProviders_1
TMP_8792(uint256) = 2 (c)** 8
TMP_8793(bool) = REF_5491 < TMP_8792
TMP_8794(None) = SOLIDITY_CALL revert TooManyTrustedProviders()()
TMP_8795(None) = SOLIDITY_CALL require(bool,error)(TMP_8793,TMP_8794)
 require(bool,error)(_trustedProviders.length >= _trustedProvidersThreshold,revert ThresholdTooHigh()())
REF_5492 -> LENGTH _trustedProviders_1
TMP_8796(bool) = REF_5492 >= _trustedProvidersThreshold_1
TMP_8797(None) = SOLIDITY_CALL revert ThresholdTooHigh()()
TMP_8798(None) = SOLIDITY_CALL require(bool,error)(TMP_8796,TMP_8797)
 trustedProvidersThreshold = _trustedProvidersThreshold
trustedProvidersThreshold_5(uint8) := _trustedProvidersThreshold_1(uint8)
 i = 0
i_1(uint256) := 0(uint256)
 i < trustedProviders.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_5493 -> LENGTH trustedProviders_2
TMP_8799(bool) = i_2 < REF_5493
CONDITION TMP_8799
 trustedProvidersMap[trustedProviders[i]] = false
REF_5494(address) -> trustedProviders_2[i_2]
REF_5495(bool) -> trustedProvidersMap_1[REF_5494]
trustedProvidersMap_3(mapping(address => bool)) := phi(['trustedProvidersMap_1'])
REF_5495(bool) (->trustedProvidersMap_3) := False(bool)
 i ++
TMP_8800(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 trustedProviders = _trustedProviders
trustedProviders_3(address[]) := _trustedProviders_1(address[])
 i_scope_0 = 0
i_scope_0_1(uint256) := 0(uint256)
 i_scope_0 < _trustedProviders.length
i_scope_0_2(uint256) := phi(['i_scope_0_1', 'i_scope_0_3'])
REF_5496 -> LENGTH _trustedProviders_1
TMP_8801(bool) = i_scope_0_2 < REF_5496
CONDITION TMP_8801
 trustedProvidersMap[_trustedProviders[i_scope_0]] = true
REF_5497(address) -> _trustedProviders_1[i_scope_0_2]
REF_5498(bool) -> trustedProvidersMap_1[REF_5497]
trustedProvidersMap_2(mapping(address => bool)) := phi(['trustedProvidersMap_1'])
REF_5498(bool) (->trustedProvidersMap_2) := True(bool)
 i_scope_0 ++
TMP_8802(uint256) := i_scope_0_2(uint256)
i_scope_0_3(uint256) = i_scope_0_2 (c)+ 1
 onlyGovernance()
MODIFIER_CALL, GovernedBase.onlyGovernance()()
```
#### FtsoV2PriceStore.submitTrustedPrices(uint32,IPricePublisher.TrustedProviderFeed[]) [EXTERNAL]
```slithir
submitTrustedPricesWindowSeconds_6(uint64) := phi(['submitTrustedPricesWindowSeconds_8', 'submitTrustedPricesWindowSeconds_4', 'submitTrustedPricesWindowSeconds_1', 'submitTrustedPricesWindowSeconds_0'])
feedIds_5(bytes21[]) := phi(['feedIds_0', 'feedIds_7', 'feedIds_2', 'feedIds_8', 'feedIds_11', 'feedIds_3', 'feedIds_1'])
latestPrices_11(mapping(bytes21 => FtsoV2PriceStore.PriceStore)) := phi(['latestPrices_21', 'latestPrices_20', 'latestPrices_23', 'latestPrices_15', 'latestPrices_0', 'latestPrices_13', 'latestPrices_22', 'latestPrices_7'])
submittedTrustedPrices_5(mapping(bytes21 => mapping(uint32 => bytes))) := phi(['submittedTrustedPrices_3', 'submittedTrustedPrices_7', 'submittedTrustedPrices_10', 'submittedTrustedPrices_0'])
lastVotingEpochIdByProvider_1(mapping(address => uint256)) := phi(['lastVotingEpochIdByProvider_0', 'lastVotingEpochIdByProvider_4'])
trustedProvidersMap_1(mapping(address => bool)) := phi(['trustedProvidersMap_0', 'trustedProvidersMap_1', 'trustedProvidersMap_3', 'trustedProvidersMap_2'])
 require(bool,error)(trustedProvidersMap[msg.sender],revert OnlyTrustedProvider()())
REF_5454(bool) -> trustedProvidersMap_1[msg.sender]
TMP_8747(None) = SOLIDITY_CALL revert OnlyTrustedProvider()()
TMP_8748(None) = SOLIDITY_CALL require(bool,error)(REF_5454,TMP_8747)
 require(bool,error)(_feeds.length == feedIds.length,revert AllPricesMustBeProvided()())
REF_5455 -> LENGTH _feeds_1
REF_5456 -> LENGTH feedIds_5
TMP_8749(bool) = REF_5455 == REF_5456
TMP_8750(None) = SOLIDITY_CALL revert AllPricesMustBeProvided()()
TMP_8751(None) = SOLIDITY_CALL require(bool,error)(TMP_8749,TMP_8750)
 previousVotingEpochId = _getPreviousVotingEpochId()
TMP_8752(uint32) = INTERNAL_CALL, FtsoV2PriceStore._getPreviousVotingEpochId()()
previousVotingEpochId_1(uint32) := TMP_8752(uint32)
 require(bool,error)(_votingRoundId == previousVotingEpochId,revert VotingRoundIdMismatch()())
TMP_8753(bool) = _votingRoundId_1 == previousVotingEpochId_1
TMP_8754(None) = SOLIDITY_CALL revert VotingRoundIdMismatch()()
TMP_8755(None) = SOLIDITY_CALL require(bool,error)(TMP_8753,TMP_8754)
 startTimestamp = _getEndTimestamp(previousVotingEpochId)
TMP_8756(uint256) = INTERNAL_CALL, FtsoV2PriceStore._getEndTimestamp(uint256)(previousVotingEpochId_1)
startTimestamp_1(uint256) := TMP_8756(uint256)
 endTimestamp = startTimestamp + submitTrustedPricesWindowSeconds
TMP_8757(uint256) = startTimestamp_1 (c)+ submitTrustedPricesWindowSeconds_8
endTimestamp_1(uint256) := TMP_8757(uint256)
 require(bool,error)(block.timestamp >= startTimestamp && block.timestamp < endTimestamp,revert SubmissionWindowClosed()())
TMP_8758(bool) = block.timestamp >= startTimestamp_1
TMP_8759(bool) = block.timestamp < endTimestamp_1
TMP_8760(bool) = TMP_8758 && TMP_8759
TMP_8761(None) = SOLIDITY_CALL revert SubmissionWindowClosed()()
TMP_8762(None) = SOLIDITY_CALL require(bool,error)(TMP_8760,TMP_8761)
 require(bool,error)(lastVotingEpochIdByProvider[msg.sender] < previousVotingEpochId,revert AlreadySubmitted()())
REF_5457(uint256) -> lastVotingEpochIdByProvider_3[msg.sender]
TMP_8763(bool) = REF_5457 < previousVotingEpochId_1
TMP_8764(None) = SOLIDITY_CALL revert AlreadySubmitted()()
TMP_8765(None) = SOLIDITY_CALL require(bool,error)(TMP_8763,TMP_8764)
 lastVotingEpochIdByProvider[msg.sender] = previousVotingEpochId
REF_5458(uint256) -> lastVotingEpochIdByProvider_3[msg.sender]
lastVotingEpochIdByProvider_4(mapping(address => uint256)) := phi(['lastVotingEpochIdByProvider_3'])
REF_5458(uint256) (->lastVotingEpochIdByProvider_4) := previousVotingEpochId_1(uint32)
 i = 0
i_1(uint256) := 0(uint256)
 i < _feeds.length
i_2(uint256) := phi(['i_3', 'i_1'])
REF_5459 -> LENGTH _feeds_1
TMP_8766(bool) = i_2 < REF_5459
CONDITION TMP_8766
 feed = _feeds[i]
REF_5460(IPricePublisher.TrustedProviderFeed) -> _feeds_1[i_2]
feed_1(IPricePublisher.TrustedProviderFeed) := REF_5460(IPricePublisher.TrustedProviderFeed)
 feedId = feedIds[i]
REF_5461(bytes21) -> feedIds_7[i_2]
feedId_1(bytes21) := REF_5461(bytes21)
 require(bool,error)(feed.id == feedId,revert FeedIdMismatch()())
REF_5462(bytes21) -> feed_1.id
TMP_8767(bool) = REF_5462 == feedId_1
TMP_8768(None) = SOLIDITY_CALL revert FeedIdMismatch()()
TMP_8769(None) = SOLIDITY_CALL require(bool,error)(TMP_8767,TMP_8768)
 require(bool,error)(feed.decimals == latestPrices[feedId].trustedDecimals,revert DecimalsMismatch()())
REF_5463(int8) -> feed_1.decimals
REF_5464(FtsoV2PriceStore.PriceStore) -> latestPrices_13[feedId_1]
REF_5465(int8) -> REF_5464.trustedDecimals
TMP_8770(bool) = REF_5463 == REF_5465
TMP_8771(None) = SOLIDITY_CALL revert DecimalsMismatch()()
TMP_8772(None) = SOLIDITY_CALL require(bool,error)(TMP_8770,TMP_8771)
 submittedTrustedPrices[feedId][previousVotingEpochId] = bytes.concat(submittedTrustedPrices[feedId][previousVotingEpochId],bytes4(feed.value))
REF_5466(mapping(uint32 => bytes)) -> submittedTrustedPrices_7[feedId_1]
REF_5467(bytes) -> REF_5466[previousVotingEpochId_1]
REF_5469(mapping(uint32 => bytes)) -> submittedTrustedPrices_7[feedId_1]
REF_5470(bytes) -> REF_5469[previousVotingEpochId_1]
REF_5471(uint32) -> feed_1.value
TMP_8773 = CONVERT REF_5471 to bytes4
TMP_8774(bytes) = SOLIDITY_CALL bytes.concat()(REF_5470,TMP_8773)
submittedTrustedPrices_8(mapping(bytes21 => mapping(uint32 => bytes))) := phi(['submittedTrustedPrices_7'])
REF_5467(bytes) (->submittedTrustedPrices_8) := TMP_8774(bytes)
 i ++
TMP_8775(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
```
#### FtsoV2PriceStore.supportsInterface(bytes4) [EXTERNAL]
```slithir
 _interfaceId == type()(IERC165).interfaceId || _interfaceId == type()(IPriceReader).interfaceId || _interfaceId == type()(IPricePublisher).interfaceId
TMP_8886(type(IERC165)) = SOLIDITY_CALL type()(IERC165)
REF_5540(bytes4) (->None) := 33540519(bytes4)
TMP_8887(bool) = _interfaceId_1 == REF_5540
TMP_8888(type(IPriceReader)) = SOLIDITY_CALL type()(IPriceReader)
REF_5541(bytes4) (->None) := 1373518958(bytes4)
TMP_8889(bool) = _interfaceId_1 == REF_5541
TMP_8890(bool) = TMP_8887 || TMP_8889
TMP_8891(type(IPricePublisher)) = SOLIDITY_CALL type()(IPricePublisher)
REF_5542(bytes4) (->None) := 3395489546(bytes4)
TMP_8892(bool) = _interfaceId_1 == REF_5542
TMP_8893(bool) = TMP_8890 || TMP_8892
RETURN TMP_8893
```
#### FtsoV2PriceStore.updateSettings(bytes21[],string[],int8[],uint16) [EXTERNAL]
```slithir
MAX_BIPS_1(uint256) := phi(['MAX_BIPS_2', 'MAX_BIPS_0'])
latestPrices_14(mapping(bytes21 => FtsoV2PriceStore.PriceStore)) := phi(['latestPrices_21', 'latestPrices_20', 'latestPrices_23', 'latestPrices_15', 'latestPrices_0', 'latestPrices_13', 'latestPrices_22', 'latestPrices_7'])
submittedTrustedPrices_9(mapping(bytes21 => mapping(uint32 => bytes))) := phi(['submittedTrustedPrices_3', 'submittedTrustedPrices_7', 'submittedTrustedPrices_10', 'submittedTrustedPrices_0'])
lastPublishedVotingRoundId_6(uint32) := phi(['lastPublishedVotingRoundId_1', 'lastPublishedVotingRoundId_4', 'lastPublishedVotingRoundId_0', 'lastPublishedVotingRoundId_7'])
 require(bool,error)(_feedIds.length == _symbols.length && _feedIds.length == _trustedDecimals.length,revert LengthMismatch()())
REF_5472 -> LENGTH _feedIds_1
REF_5473 -> LENGTH _symbols_1
TMP_8776(bool) = REF_5472 == REF_5473
REF_5474 -> LENGTH _feedIds_1
REF_5475 -> LENGTH _trustedDecimals_1
TMP_8777(bool) = REF_5474 == REF_5475
TMP_8778(bool) = TMP_8776 && TMP_8777
TMP_8779(None) = SOLIDITY_CALL revert LengthMismatch()()
TMP_8780(None) = SOLIDITY_CALL require(bool,error)(TMP_8778,TMP_8779)
 require(bool,error)(_maxSpreadBIPS <= MAX_BIPS,revert MaxSpreadTooBig()())
TMP_8781(bool) = _maxSpreadBIPS_1 <= MAX_BIPS_2
TMP_8782(None) = SOLIDITY_CALL revert MaxSpreadTooBig()()
TMP_8783(None) = SOLIDITY_CALL require(bool,error)(TMP_8781,TMP_8782)
 maxSpreadBIPS = _maxSpreadBIPS
maxSpreadBIPS_1(uint16) := _maxSpreadBIPS_1(uint16)
 feedIds = _feedIds
feedIds_8(bytes21[]) := _feedIds_1(bytes21[])
 i = 0
i_1(uint256) := 0(uint256)
 i < _feedIds.length
i_2(uint256) := phi(['i_1', 'i_3'])
REF_5476 -> LENGTH _feedIds_1
TMP_8784(bool) = i_2 < REF_5476
CONDITION TMP_8784
 feedId = _feedIds[i]
REF_5477(bytes21) -> _feedIds_1[i_2]
feedId_1(bytes21) := REF_5477(bytes21)
 symbolToFeedId[_symbols[i]] = feedId
REF_5478(string) -> _symbols_1[i_2]
REF_5479(bytes21) -> symbolToFeedId_0[REF_5478]
symbolToFeedId_1(mapping(string => bytes21)) := phi(['symbolToFeedId_0'])
REF_5479(bytes21) (->symbolToFeedId_1) := feedId_1(bytes21)
 feedIdToSymbol[feedId] = _symbols[i]
REF_5480(string) -> feedIdToSymbol_0[feedId_1]
REF_5481(string) -> _symbols_1[i_2]
feedIdToSymbol_1(mapping(bytes21 => string)) := phi(['feedIdToSymbol_0'])
REF_5480(string) (->feedIdToSymbol_1) := REF_5481(string)
 latestPrice = latestPrices[feedId]
REF_5482(FtsoV2PriceStore.PriceStore) -> latestPrices_15[feedId_1]
latestPrice_1 (-> ['latestPrices'])(FtsoV2PriceStore.PriceStore) := REF_5482(FtsoV2PriceStore.PriceStore)
 latestPrice.trustedDecimals != _trustedDecimals[i]
REF_5483(int8) -> latestPrice_1 (-> ['latestPrices']).trustedDecimals
REF_5484(int8) -> _trustedDecimals_1[i_2]
TMP_8785(bool) = REF_5483 != REF_5484
CONDITION TMP_8785
 latestPrice.trustedDecimals = _trustedDecimals[i]
REF_5485(int8) -> latestPrice_1 (-> ['latestPrices']).trustedDecimals
REF_5486(int8) -> _trustedDecimals_1[i_2]
latestPrice_2 (-> ['latestPrices'])(FtsoV2PriceStore.PriceStore) := phi(["latestPrice_1 (-> ['latestPrices'])"])
REF_5485(int8) (->latestPrice_2 (-> ['latestPrices'])) := REF_5486(int8)
latestPrices_17(mapping(bytes21 => FtsoV2PriceStore.PriceStore)) := phi(["latestPrice_2 (-> ['latestPrices'])"])
 latestPrice.trustedValue = 0
REF_5487(uint32) -> latestPrice_2 (-> ['latestPrices']).trustedValue
latestPrice_3 (-> ['latestPrices'])(FtsoV2PriceStore.PriceStore) := phi(["latestPrice_2 (-> ['latestPrices'])"])
REF_5487(uint32) (->latestPrice_3 (-> ['latestPrices'])) := 0(uint256)
latestPrices_18(mapping(bytes21 => FtsoV2PriceStore.PriceStore)) := phi(["latestPrice_3 (-> ['latestPrices'])"])
 latestPrice.trustedVotingRoundId = 0
REF_5488(uint32) -> latestPrice_3 (-> ['latestPrices']).trustedVotingRoundId
latestPrice_4 (-> ['latestPrices'])(FtsoV2PriceStore.PriceStore) := phi(["latestPrice_3 (-> ['latestPrices'])"])
REF_5488(uint32) (->latestPrice_4 (-> ['latestPrices'])) := 0(uint256)
latestPrices_19(mapping(bytes21 => FtsoV2PriceStore.PriceStore)) := phi(["latestPrice_4 (-> ['latestPrices'])"])
 j = lastPublishedVotingRoundId + 1
TMP_8786(uint32) = lastPublishedVotingRoundId_7 (c)+ 1
j_1(uint32) := TMP_8786(uint32)
 j <= _getPreviousVotingEpochId()
j_2(uint32) := phi(['j_1', 'j_3'])
TMP_8787(uint32) = INTERNAL_CALL, FtsoV2PriceStore._getPreviousVotingEpochId()()
TMP_8788(bool) = j_2 <= TMP_8787
CONDITION TMP_8788
 delete submittedTrustedPrices[feedId][j]
REF_5489(mapping(uint32 => bytes)) -> submittedTrustedPrices_11[feedId_1]
REF_5490(bytes) -> REF_5489[j_2]
REF_5489 = delete REF_5490 
 j ++
TMP_8789(uint32) := j_2(uint32)
j_3(uint32) = j_2 (c)+ 1
 i ++
TMP_8790(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 onlyGovernance()
MODIFIER_CALL, GovernedBase.onlyGovernance()()
```
#### MerkleProof.verifyCalldata(bytes32[],bytes32,bytes32) [INTERNAL]
```slithir
 processProofCalldata(proof,leaf) == root
TMP_416(bytes32) = INTERNAL_CALL, MerkleProof.processProofCalldata(bytes32[],bytes32)(proof_1,leaf_1)
TMP_417(bool) = TMP_416 == root_1
RETURN TMP_417
```
#### IRelay.merkleRoots(uint256,uint256) [EXTERNAL]
```slithir

```
#### MerkleProof.processProofCalldata(bytes32[],bytes32) [INTERNAL]
```slithir
proof_1(bytes32[]) := phi(['proof_1'])
leaf_1(bytes32) := phi(['leaf_1'])
 computedHash = leaf
computedHash_1(bytes32) := leaf_1(bytes32)
 i = 0
i_1(uint256) := 0(uint256)
 i < proof.length
computedHash_2(bytes32) := phi(['computedHash_1', 'computedHash_3'])
i_2(uint256) := phi(['i_1', 'i_3'])
REF_155 -> LENGTH proof_1
TMP_421(bool) = i_2 < REF_155
CONDITION TMP_421
 computedHash = _hashPair(computedHash,proof[i])
REF_156(bytes32) -> proof_1[i_2]
TMP_422(bytes32) = INTERNAL_CALL, MerkleProof._hashPair(bytes32,bytes32)(computedHash_2,REF_156)
computedHash_3(bytes32) := TMP_422(bytes32)
 i ++
TMP_423(uint256) := i_2(uint256)
i_3(uint256) = i_2 (c)+ 1
 computedHash
RETURN computedHash_2
```
#### MerkleProof._hashPair(bytes32,bytes32) [PRIVATE]
```slithir
a_1(bytes32) := phi(['a_3', 'a_3', 'computedHash_2', 'computedHash_2'])
b_1(bytes32) := phi(['b_5', 'REF_156', 'REF_154', 'b_5'])
 a < b
TMP_470(bool) = a_1 < b_1
CONDITION TMP_470
 _efficientHash(a,b)
TMP_471(bytes32) = INTERNAL_CALL, MerkleProof._efficientHash(bytes32,bytes32)(a_1,b_1)
RETURN TMP_471
 _efficientHash(b,a)
TMP_472(bytes32) = INTERNAL_CALL, MerkleProof._efficientHash(bytes32,bytes32)(b_1,a_1)
RETURN TMP_472
```
