
### Storage layout (FakePriceReader) 

```text
provider address
pricingData mapping(string => FakePriceReader.PricingData)

```

#### FakePriceReader.constructor(address) [PUBLIC]
```slithir
 provider = _provider
provider_1(address) := _provider_1(address)
```
#### FakePriceReader.finalizePrices() [EXTERNAL]
```slithir
 PricesPublished(0)
Emit PricesPublished(0)
 onlyDataProvider()
MODIFIER_CALL, FakePriceReader.onlyDataProvider()()
```
#### FakePriceReader.getPrice(string) [EXTERNAL]
```slithir
 data = _getPricingData(_symbol)
TMP_8991(FakePriceReader.PricingData) = INTERNAL_CALL, FakePriceReader._getPricingData(string)(_symbol_1)
data_1 (-> ['TMP_8991'])(FakePriceReader.PricingData) := TMP_8991(FakePriceReader.PricingData)
 (data.price,data.timestamp,data.decimals)
REF_5583(uint128) -> data_1 (-> ['TMP_8991']).price
REF_5584(uint64) -> data_1 (-> ['TMP_8991']).timestamp
REF_5585(uint8) -> data_1 (-> ['TMP_8991']).decimals
RETURN REF_5583,REF_5584,REF_5585
 (_price,_timestamp,_priceDecimals)
```
#### FakePriceReader.getPriceFromTrustedProviders(string) [EXTERNAL]
```slithir
 data = _getPricingData(_symbol)
TMP_8992(FakePriceReader.PricingData) = INTERNAL_CALL, FakePriceReader._getPricingData(string)(_symbol_1)
data_1 (-> ['TMP_8992'])(FakePriceReader.PricingData) := TMP_8992(FakePriceReader.PricingData)
 (data.trustedPrice,data.trustedTimestamp,data.decimals)
REF_5586(uint128) -> data_1 (-> ['TMP_8992']).trustedPrice
REF_5587(uint64) -> data_1 (-> ['TMP_8992']).trustedTimestamp
REF_5588(uint8) -> data_1 (-> ['TMP_8992']).decimals
RETURN REF_5586,REF_5587,REF_5588
 (_price,_timestamp,_priceDecimals)
```
#### FakePriceReader.getPriceFromTrustedProvidersWithQuality(string) [EXTERNAL]
```slithir
 data = _getPricingData(_symbol)
TMP_8993(FakePriceReader.PricingData) = INTERNAL_CALL, FakePriceReader._getPricingData(string)(_symbol_1)
data_1 (-> ['TMP_8993'])(FakePriceReader.PricingData) := TMP_8993(FakePriceReader.PricingData)
 (data.trustedPrice,data.trustedTimestamp,data.decimals,0)
REF_5589(uint128) -> data_1 (-> ['TMP_8993']).trustedPrice
REF_5590(uint64) -> data_1 (-> ['TMP_8993']).trustedTimestamp
REF_5591(uint8) -> data_1 (-> ['TMP_8993']).decimals
RETURN REF_5589,REF_5590,REF_5591,0
 (_price,_timestamp,_priceDecimals,_numberOfSubmits)
```
#### FakePriceReader.setDecimals(string,uint256) [EXTERNAL]
```slithir
pricingData_1(mapping(string => FakePriceReader.PricingData)) := phi(['pricingData_0', 'pricingData_3', 'pricingData_4'])
 pricingData[_symbol].decimals = _decimals.toUint8()
REF_5572(FakePriceReader.PricingData) -> pricingData_2[_symbol_1]
REF_5573(uint8) -> REF_5572.decimals
TMP_8979(uint8) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint8(uint256), arguments:['_decimals_1'] 
pricingData_3(mapping(string => FakePriceReader.PricingData)) := phi(['pricingData_2'])
REF_5573(uint8) (->pricingData_3) := TMP_8979(uint8)
 onlyDataProvider()
MODIFIER_CALL, FakePriceReader.onlyDataProvider()()
```
#### FakePriceReader.setPrice(string,uint256) [EXTERNAL]
```slithir
 data = _getPricingData(_symbol)
TMP_8981(FakePriceReader.PricingData) = INTERNAL_CALL, FakePriceReader._getPricingData(string)(_symbol_1)
data_1 (-> ['TMP_8981'])(FakePriceReader.PricingData) := TMP_8981(FakePriceReader.PricingData)
 data.price = _price.toUint128()
REF_5575(uint128) -> data_1 (-> ['TMP_8981']).price
TMP_8982(uint128) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint128(uint256), arguments:['_price_1'] 
data_2 (-> ['TMP_8981'])(FakePriceReader.PricingData) := phi(["data_1 (-> ['TMP_8981'])"])
REF_5575(uint128) (->data_2 (-> ['TMP_8981'])) := TMP_8982(uint128)
TMP_8981(FakePriceReader.PricingData) := phi(["data_2 (-> ['TMP_8981'])"])
 data.timestamp = block.timestamp.toUint64()
REF_5577(uint64) -> data_2 (-> ['TMP_8981']).timestamp
TMP_8983(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['block.timestamp'] 
data_3 (-> ['TMP_8981'])(FakePriceReader.PricingData) := phi(["data_2 (-> ['TMP_8981'])"])
REF_5577(uint64) (->data_3 (-> ['TMP_8981'])) := TMP_8983(uint64)
TMP_8981(FakePriceReader.PricingData) := phi(["data_3 (-> ['TMP_8981'])"])
 onlyDataProvider()
MODIFIER_CALL, FakePriceReader.onlyDataProvider()()
```
#### FakePriceReader.setPriceFromTrustedProviders(string,uint256) [EXTERNAL]
```slithir
 data = _getPricingData(_symbol)
TMP_8985(FakePriceReader.PricingData) = INTERNAL_CALL, FakePriceReader._getPricingData(string)(_symbol_1)
data_1 (-> ['TMP_8985'])(FakePriceReader.PricingData) := TMP_8985(FakePriceReader.PricingData)
 data.trustedPrice = _price.toUint128()
REF_5579(uint128) -> data_1 (-> ['TMP_8985']).trustedPrice
TMP_8986(uint128) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint128(uint256), arguments:['_price_1'] 
data_2 (-> ['TMP_8985'])(FakePriceReader.PricingData) := phi(["data_1 (-> ['TMP_8985'])"])
REF_5579(uint128) (->data_2 (-> ['TMP_8985'])) := TMP_8986(uint128)
TMP_8985(FakePriceReader.PricingData) := phi(["data_2 (-> ['TMP_8985'])"])
 data.trustedTimestamp = block.timestamp.toUint64()
REF_5581(uint64) -> data_2 (-> ['TMP_8985']).trustedTimestamp
TMP_8987(uint64) = LIBRARY_CALL, dest:SafeCast, function:SafeCast.toUint64(uint256), arguments:['block.timestamp'] 
data_3 (-> ['TMP_8985'])(FakePriceReader.PricingData) := phi(["data_2 (-> ['TMP_8985'])"])
REF_5581(uint64) (->data_3 (-> ['TMP_8985'])) := TMP_8987(uint64)
TMP_8985(FakePriceReader.PricingData) := phi(["data_3 (-> ['TMP_8985'])"])
 onlyDataProvider()
MODIFIER_CALL, FakePriceReader.onlyDataProvider()()
```
#### FakePriceReader.supportsInterface(bytes4) [EXTERNAL]
```slithir
 _interfaceId == type()(IERC165).interfaceId || _interfaceId == type()(IPriceReader).interfaceId || _interfaceId == type()(IPriceChangeEmitter).interfaceId
TMP_8994(type(IERC165)) = SOLIDITY_CALL type()(IERC165)
REF_5592(bytes4) (->None) := 33540519(bytes4)
TMP_8995(bool) = _interfaceId_1 == REF_5592
TMP_8996(type(IPriceReader)) = SOLIDITY_CALL type()(IPriceReader)
REF_5593(bytes4) (->None) := 1373518958(bytes4)
TMP_8997(bool) = _interfaceId_1 == REF_5593
TMP_8998(bool) = TMP_8995 || TMP_8997
TMP_8999(type(IPriceChangeEmitter)) = SOLIDITY_CALL type()(IPriceChangeEmitter)
REF_5594(bytes4) (->None) := 0(bytes4)
TMP_9000(bool) = _interfaceId_1 == REF_5594
TMP_9001(bool) = TMP_8998 || TMP_9000
RETURN TMP_9001
```
#### SafeCast.toUint8(uint256) [INTERNAL]
```slithir
 require(bool,string)(value <= type()(uint8).max,SafeCast: value doesn't fit in 8 bits)
TMP_782(uint8) := 255(uint8)
TMP_783(bool) = value_1 <= TMP_782
TMP_784(None) = SOLIDITY_CALL require(bool,string)(TMP_783,SafeCast: value doesn't fit in 8 bits)
 uint8(value)
TMP_785 = CONVERT value_1 to uint8
RETURN TMP_785
```
#### SafeCast.toUint128(uint256) [INTERNAL]
```slithir
 require(bool,string)(value <= type()(uint128).max,SafeCast: value doesn't fit in 128 bits)
TMP_707(uint128) := 340282366920938463463374607431768211455(uint128)
TMP_708(bool) = value_1 <= TMP_707
TMP_709(None) = SOLIDITY_CALL require(bool,string)(TMP_708,SafeCast: value doesn't fit in 128 bits)
 uint128(value)
TMP_710 = CONVERT value_1 to uint128
RETURN TMP_710
```
#### SafeCast.toUint64(uint256) [INTERNAL]
```slithir
 require(bool,string)(value <= type()(uint64).max,SafeCast: value doesn't fit in 64 bits)
TMP_747(uint64) := 18446744073709551615(uint64)
TMP_748(bool) = value_1 <= TMP_747
TMP_749(None) = SOLIDITY_CALL require(bool,string)(TMP_748,SafeCast: value doesn't fit in 64 bits)
 uint64(value)
TMP_750 = CONVERT value_1 to uint64
RETURN TMP_750
```
