### Storage layout (SimpleBondingCurve) 

```text
VIRTUAL_BASE uint256
VIRTUAL_QUOTE uint256
reserves mapping(address => SimpleBondingCurve.Reserves)
supply mapping(address => SimpleBondingCurve.Supply)

```
#### SimpleBondingCurve._getBaseAmount(uint256,uint256,uint256,bool) [INTERNAL]
```slithir
quoteAmount_1(uint256) := phi(['quoteAmount_1'])
quoteReserve_1(uint256) := phi(['REF_1015'])
baseReserve_1(uint256) := phi(['REF_1016'])
isBuy_1(bool) := phi(['isBuy_1'])
 (quoteAmount * baseReserve) / quoteReserveAfter
TMP_2131(uint256) = quoteAmount_1 (c)* baseReserve_1
TMP_2132(uint256) = TMP_2131 (c)/ quoteReserveAfter_3
RETURN TMP_2132
 isBuy
CONDITION isBuy_1
 quoteReserveAfter = quoteReserve + quoteAmount
TMP_2133(uint256) = quoteReserve_1 (c)+ quoteAmount_1
quoteReserveAfter_1(uint256) := TMP_2133(uint256)
 quoteReserveAfter = quoteReserve - quoteAmount
TMP_2134(uint256) = quoteReserve_1 (c)- quoteAmount_1
quoteReserveAfter_2(uint256) := TMP_2134(uint256)
quoteReserveAfter_3(uint256) := phi(['quoteReserveAfter_1', 'quoteReserveAfter_2'])
 baseAmount
```

#### SimpleBondingCurve._setReserves(address,uint256,uint256) [INTERNAL]
```slithir
token_1(address) := phi(['token_1', 'token_1'])
quoteReserve_1(uint256) := phi(['quoteReserve_1', 'VIRTUAL_QUOTE_2'])
baseReserve_1(uint256) := phi(['TMP_2098', 'baseReserve_1'])
reserves_14(mapping(address => SimpleBondingCurve.Reserves)) := phi(['reserves_0', 'reserves_16', 'reserves_10', 'reserves_12', 'reserves_13', 'reserves_8', 'reserves_9', 'reserves_4', 'reserves_11'])
 r = reserves[token]
REF_1022(SimpleBondingCurve.Reserves) -> reserves_14[token_1]
r_1 (-> ['reserves'])(SimpleBondingCurve.Reserves) := REF_1022(SimpleBondingCurve.Reserves)
 r.quoteReserve = quoteReserve
REF_1023(uint256) -> r_1 (-> ['reserves']).quoteReserve
r_2 (-> ['reserves'])(SimpleBondingCurve.Reserves) := phi(["r_1 (-> ['reserves'])"])
REF_1023(uint256) (->r_2 (-> ['reserves'])) := quoteReserve_1(uint256)
reserves_15(mapping(address => SimpleBondingCurve.Reserves)) := phi(["r_2 (-> ['reserves'])"])
 r.baseReserve = baseReserve
REF_1024(uint256) -> r_2 (-> ['reserves']).baseReserve
r_3 (-> ['reserves'])(SimpleBondingCurve.Reserves) := phi(["r_2 (-> ['reserves'])"])
REF_1024(uint256) (->r_3 (-> ['reserves'])) := baseReserve_1(uint256)
reserves_16(mapping(address => SimpleBondingCurve.Reserves)) := phi(["r_3 (-> ['reserves'])"])
 ReservesSet(token,quoteReserve,baseReserve)
Emit ReservesSet(token_1,quoteReserve_1,baseReserve_1)
```
#### SimpleBondingCurve._setSupply(address,uint256,uint256) [INTERNAL]
```slithir
token_1(address) := phi(['token_1'])
totalSupply__1(uint256) := phi(['totalSupply__1'])
bondingSupply__1(uint256) := phi(['bondingSupply__1'])
supply_4(mapping(address => SimpleBondingCurve.Supply)) := phi(['supply_0', 'supply_6', 'supply_3', 'supply_2', 'supply_1'])
 s = supply[token]
REF_1025(SimpleBondingCurve.Supply) -> supply_4[token_1]
s_1 (-> ['supply'])(SimpleBondingCurve.Supply) := REF_1025(SimpleBondingCurve.Supply)
 s.totalSupply = totalSupply_
REF_1026(uint256) -> s_1 (-> ['supply']).totalSupply
s_2 (-> ['supply'])(SimpleBondingCurve.Supply) := phi(["s_1 (-> ['supply'])"])
REF_1026(uint256) (->s_2 (-> ['supply'])) := totalSupply__1(uint256)
supply_5(mapping(address => SimpleBondingCurve.Supply)) := phi(["s_2 (-> ['supply'])"])
 s.bondingSupply = bondingSupply_
REF_1027(uint256) -> s_2 (-> ['supply']).bondingSupply
s_3 (-> ['supply'])(SimpleBondingCurve.Supply) := phi(["s_2 (-> ['supply'])"])
REF_1027(uint256) (->s_3 (-> ['supply'])) := bondingSupply__1(uint256)
supply_6(mapping(address => SimpleBondingCurve.Supply)) := phi(["s_3 (-> ['supply'])"])
```
#### SimpleBondingCurve._setVirtualReserves(uint256,uint256) [INTERNAL]
```slithir
virtualBase_1(uint256) := phi(['virtualBase_1', 'virtualBase_1'])
virtualQuote_1(uint256) := phi(['virtualQuote_1', 'virtualQuote_1'])
 virtualBase == 0
TMP_2126(bool) = virtualBase_1 == 0
CONDITION TMP_2126
 revert InvalidVirtualBase()()
TMP_2127(None) = SOLIDITY_CALL revert InvalidVirtualBase()()
 virtualQuote == 0
TMP_2128(bool) = virtualQuote_1 == 0
CONDITION TMP_2128
 revert InvalidVirtualQuote()()
TMP_2129(None) = SOLIDITY_CALL revert InvalidVirtualQuote()()
 VIRTUAL_BASE = virtualBase
VIRTUAL_BASE_6(uint256) := virtualBase_1(uint256)
 VIRTUAL_QUOTE = virtualQuote
VIRTUAL_QUOTE_6(uint256) := virtualQuote_1(uint256)
 VirtualReservesSet(virtualBase,virtualQuote)
Emit VirtualReservesSet(virtualBase_1,virtualQuote_1)
```
#### SimpleBondingCurve.baseSoldFromCurve(address) [EXTERNAL]
```slithir
VIRTUAL_BASE_5(uint256) := phi(['VIRTUAL_BASE_4', 'VIRTUAL_BASE_6', 'VIRTUAL_BASE_0'])
reserves_9(mapping(address => SimpleBondingCurve.Reserves)) := phi(['reserves_0', 'reserves_16', 'reserves_10', 'reserves_12', 'reserves_13', 'reserves_8', 'reserves_9', 'reserves_4', 'reserves_11'])
supply_3(mapping(address => SimpleBondingCurve.Supply)) := phi(['supply_0', 'supply_6', 'supply_3', 'supply_2', 'supply_1'])
 (supply[token].bondingSupply + VIRTUAL_BASE) - reserves[token].baseReserve
REF_1005(SimpleBondingCurve.Supply) -> supply_3[token_1]
REF_1006(uint256) -> REF_1005.bondingSupply
TMP_2115(uint256) = REF_1006 (c)+ VIRTUAL_BASE_5
REF_1007(SimpleBondingCurve.Reserves) -> reserves_9[token_1]
REF_1008(uint256) -> REF_1007.baseReserve
TMP_2116(uint256) = TMP_2115 (c)- REF_1008
RETURN TMP_2116
```
#### SimpleBondingCurve.bondingSupply(address) [EXTERNAL]
```slithir
supply_1(mapping(address => SimpleBondingCurve.Supply)) := phi(['supply_0', 'supply_6', 'supply_3', 'supply_2', 'supply_1'])
 supply[token].bondingSupply
REF_1001(SimpleBondingCurve.Supply) -> supply_1[token_1]
REF_1002(uint256) -> REF_1001.bondingSupply
RETURN REF_1002
```
#### SimpleBondingCurve.buy(address,uint256) [EXTERNAL]
```slithir
reserves_1(mapping(address => SimpleBondingCurve.Reserves)) := phi(['reserves_0', 'reserves_16', 'reserves_10', 'reserves_12', 'reserves_13', 'reserves_8', 'reserves_9', 'reserves_4', 'reserves_11'])
 r = reserves[token]
REF_991(SimpleBondingCurve.Reserves) -> reserves_2[token_1]
r_1 (-> ['reserves'])(SimpleBondingCurve.Reserves) := REF_991(SimpleBondingCurve.Reserves)
 quoteAmount = _getQuoteAmount(baseAmount,r.quoteReserve,r.baseReserve,true)
REF_992(uint256) -> r_1 (-> ['reserves']).quoteReserve
REF_993(uint256) -> r_1 (-> ['reserves']).baseReserve
TMP_2111(uint256) = INTERNAL_CALL, SimpleBondingCurve._getQuoteAmount(uint256,uint256,uint256,bool)(baseAmount_1,REF_992,REF_993,True)
quoteAmount_1(uint256) := TMP_2111(uint256)
 r.quoteReserve += quoteAmount
REF_994(uint256) -> r_1 (-> ['reserves']).quoteReserve
r_2 (-> ['reserves'])(SimpleBondingCurve.Reserves) := phi(["r_1 (-> ['reserves'])"])
REF_994(-> r_2 (-> ['reserves'])) = REF_994 (c)+ quoteAmount_1
reserves_3(mapping(address => SimpleBondingCurve.Reserves)) := phi(["r_2 (-> ['reserves'])"])
 r.baseReserve -= baseAmount
REF_995(uint256) -> r_2 (-> ['reserves']).baseReserve
r_3 (-> ['reserves'])(SimpleBondingCurve.Reserves) := phi(["r_2 (-> ['reserves'])"])
REF_995(-> r_3 (-> ['reserves'])) = REF_995 (c)- baseAmount_1
reserves_4(mapping(address => SimpleBondingCurve.Reserves)) := phi(["r_3 (-> ['reserves'])"])
 onlyLaunchpad()
MODIFIER_CALL, SimpleBondingCurve.onlyLaunchpad()()
 quoteAmount
RETURN quoteAmount_1
```
#### SimpleBondingCurve.constructor(address) [PUBLIC]
```slithir
 launchpad = launchpad_
launchpad_1(address) := launchpad__1(address)
```
#### SimpleBondingCurve.getReserves(address) [EXTERNAL]
```slithir
reserves_11(mapping(address => SimpleBondingCurve.Reserves)) := phi(['reserves_0', 'reserves_16', 'reserves_10', 'reserves_12', 'reserves_13', 'reserves_8', 'reserves_9', 'reserves_4', 'reserves_11'])
 r = reserves[token]
REF_1011(SimpleBondingCurve.Reserves) -> reserves_11[token_1]
r_1 (-> ['reserves'])(SimpleBondingCurve.Reserves) := REF_1011(SimpleBondingCurve.Reserves)
 quoteReserve = r.quoteReserve
REF_1012(uint256) -> r_1 (-> ['reserves']).quoteReserve
quoteReserve_1(uint256) := REF_1012(uint256)
 baseReserve = r.baseReserve
REF_1013(uint256) -> r_1 (-> ['reserves']).baseReserve
baseReserve_1(uint256) := REF_1013(uint256)
 (quoteReserve,baseReserve)
RETURN quoteReserve_1,baseReserve_1
```
#### SimpleBondingCurve.init(bytes) [EXTERNAL]
```slithir
 (virtualBase,virtualQuote) = abi.decode(data,(uint256,uint256))
TUPLE_15(uint256,uint256) = SOLIDITY_CALL abi.decode()(data_1(uint256,uint256))
virtualBase_1(uint256)= UNPACK TUPLE_15 index: 0 
virtualQuote_1(uint256)= UNPACK TUPLE_15 index: 1 
 _setVirtualReserves(virtualBase,virtualQuote)
INTERNAL_CALL, SimpleBondingCurve._setVirtualReserves(uint256,uint256)(virtualBase_1,virtualQuote_1)
 onlyLaunchpad()
MODIFIER_CALL, SimpleBondingCurve.onlyLaunchpad()()
```
#### SimpleBondingCurve.initializeCurve(address,uint256,uint256) [EXTERNAL]
```slithir
VIRTUAL_BASE_1(uint256) := phi(['VIRTUAL_BASE_4', 'VIRTUAL_BASE_6', 'VIRTUAL_BASE_0'])
VIRTUAL_QUOTE_1(uint256) := phi(['VIRTUAL_QUOTE_0', 'VIRTUAL_QUOTE_4', 'VIRTUAL_QUOTE_6'])
 _setReserves(token,VIRTUAL_QUOTE,bondingSupply_ + VIRTUAL_BASE)
TMP_2098(uint256) = bondingSupply__1 (c)+ VIRTUAL_BASE_2
INTERNAL_CALL, SimpleBondingCurve._setReserves(address,uint256,uint256)(token_1,VIRTUAL_QUOTE_2,TMP_2098)
 _setSupply(token,totalSupply_,bondingSupply_)
INTERNAL_CALL, SimpleBondingCurve._setSupply(address,uint256,uint256)(token_1,totalSupply__1,bondingSupply__1)
 NewTokenLaunched(token,VIRTUAL_BASE,VIRTUAL_QUOTE)
Emit NewTokenLaunched(token_1,VIRTUAL_BASE_4,VIRTUAL_QUOTE_4)
 onlyLaunchpad()
MODIFIER_CALL, SimpleBondingCurve.onlyLaunchpad()()
```
#### SimpleBondingCurve.quoteBaseForQuote(address,uint256,bool) [EXTERNAL]
```slithir
reserves_12(mapping(address => SimpleBondingCurve.Reserves)) := phi(['reserves_0', 'reserves_16', 'reserves_10', 'reserves_12', 'reserves_13', 'reserves_8', 'reserves_9', 'reserves_4', 'reserves_11'])
 r = reserves[token]
REF_1014(SimpleBondingCurve.Reserves) -> reserves_12[token_1]
r_1 (-> ['reserves'])(SimpleBondingCurve.Reserves) := REF_1014(SimpleBondingCurve.Reserves)
 baseAmount = _getBaseAmount(quoteAmount,r.quoteReserve,r.baseReserve,isBuy)
REF_1015(uint256) -> r_1 (-> ['reserves']).quoteReserve
REF_1016(uint256) -> r_1 (-> ['reserves']).baseReserve
TMP_2118(uint256) = INTERNAL_CALL, SimpleBondingCurve._getBaseAmount(uint256,uint256,uint256,bool)(quoteAmount_1,REF_1015,REF_1016,isBuy_1)
baseAmount_1(uint256) := TMP_2118(uint256)
 baseAmount
RETURN baseAmount_1
```
#### SimpleBondingCurve.quoteBoughtByCurve(address) [EXTERNAL]
```slithir
VIRTUAL_QUOTE_5(uint256) := phi(['VIRTUAL_QUOTE_0', 'VIRTUAL_QUOTE_4', 'VIRTUAL_QUOTE_6'])
reserves_10(mapping(address => SimpleBondingCurve.Reserves)) := phi(['reserves_0', 'reserves_16', 'reserves_10', 'reserves_12', 'reserves_13', 'reserves_8', 'reserves_9', 'reserves_4', 'reserves_11'])
 reserves[token].quoteReserve - VIRTUAL_QUOTE
REF_1009(SimpleBondingCurve.Reserves) -> reserves_10[token_1]
REF_1010(uint256) -> REF_1009.quoteReserve
TMP_2117(uint256) = REF_1010 (c)- VIRTUAL_QUOTE_5
RETURN TMP_2117
```
#### SimpleBondingCurve.quoteQuoteForBase(address,uint256,bool) [EXTERNAL]
```slithir
reserves_13(mapping(address => SimpleBondingCurve.Reserves)) := phi(['reserves_0', 'reserves_16', 'reserves_10', 'reserves_12', 'reserves_13', 'reserves_8', 'reserves_9', 'reserves_4', 'reserves_11'])
 r = reserves[token]
REF_1017(SimpleBondingCurve.Reserves) -> reserves_13[token_1]
r_1 (-> ['reserves'])(SimpleBondingCurve.Reserves) := REF_1017(SimpleBondingCurve.Reserves)
 quoteAmount = _getQuoteAmount(baseAmount,r.quoteReserve,r.baseReserve,isBuy)
REF_1018(uint256) -> r_1 (-> ['reserves']).quoteReserve
REF_1019(uint256) -> r_1 (-> ['reserves']).baseReserve
TMP_2119(uint256) = INTERNAL_CALL, SimpleBondingCurve._getQuoteAmount(uint256,uint256,uint256,bool)(baseAmount_1,REF_1018,REF_1019,isBuy_1)
quoteAmount_1(uint256) := TMP_2119(uint256)
 quoteAmount
RETURN quoteAmount_1
```
#### SimpleBondingCurve.sell(address,uint256) [EXTERNAL]
```slithir
reserves_5(mapping(address => SimpleBondingCurve.Reserves)) := phi(['reserves_0', 'reserves_16', 'reserves_10', 'reserves_12', 'reserves_13', 'reserves_8', 'reserves_9', 'reserves_4', 'reserves_11'])
 r = reserves[token]
REF_996(SimpleBondingCurve.Reserves) -> reserves_6[token_1]
r_1 (-> ['reserves'])(SimpleBondingCurve.Reserves) := REF_996(SimpleBondingCurve.Reserves)
 quoteAmount = _getQuoteAmount(baseAmount,r.quoteReserve,r.baseReserve,false)
REF_997(uint256) -> r_1 (-> ['reserves']).quoteReserve
REF_998(uint256) -> r_1 (-> ['reserves']).baseReserve
TMP_2113(uint256) = INTERNAL_CALL, SimpleBondingCurve._getQuoteAmount(uint256,uint256,uint256,bool)(baseAmount_1,REF_997,REF_998,False)
quoteAmount_1(uint256) := TMP_2113(uint256)
 r.quoteReserve -= quoteAmount
REF_999(uint256) -> r_1 (-> ['reserves']).quoteReserve
r_2 (-> ['reserves'])(SimpleBondingCurve.Reserves) := phi(["r_1 (-> ['reserves'])"])
REF_999(-> r_2 (-> ['reserves'])) = REF_999 (c)- quoteAmount_1
reserves_7(mapping(address => SimpleBondingCurve.Reserves)) := phi(["r_2 (-> ['reserves'])"])
 r.baseReserve += baseAmount
REF_1000(uint256) -> r_2 (-> ['reserves']).baseReserve
r_3 (-> ['reserves'])(SimpleBondingCurve.Reserves) := phi(["r_2 (-> ['reserves'])"])
REF_1000(-> r_3 (-> ['reserves'])) = REF_1000 (c)+ baseAmount_1
reserves_8(mapping(address => SimpleBondingCurve.Reserves)) := phi(["r_3 (-> ['reserves'])"])
 onlyLaunchpad()
MODIFIER_CALL, SimpleBondingCurve.onlyLaunchpad()()
 quoteAmount
RETURN quoteAmount_1
```
#### SimpleBondingCurve.setReserves(address,uint256,uint256) [EXTERNAL]
```slithir
 _setReserves(token,quoteReserve,baseReserve)
INTERNAL_CALL, SimpleBondingCurve._setReserves(address,uint256,uint256)(token_1,quoteReserve_1,baseReserve_1)
 onlyLaunchpadOwner()
MODIFIER_CALL, SimpleBondingCurve.onlyLaunchpadOwner()()
```
#### SimpleBondingCurve.setVirtualReserves(uint256,uint256) [EXTERNAL]
```slithir
 virtualBase == 0
TMP_2105(bool) = virtualBase_1 == 0
CONDITION TMP_2105
 revert InvalidVirtualBase()()
TMP_2106(None) = SOLIDITY_CALL revert InvalidVirtualBase()()
 virtualQuote == 0
TMP_2107(bool) = virtualQuote_1 == 0
CONDITION TMP_2107
 revert InvalidVirtualQuote()()
TMP_2108(None) = SOLIDITY_CALL revert InvalidVirtualQuote()()
 _setVirtualReserves(virtualBase,virtualQuote)
INTERNAL_CALL, SimpleBondingCurve._setVirtualReserves(uint256,uint256)(virtualBase_1,virtualQuote_1)
 onlyLaunchpadOwner()
MODIFIER_CALL, SimpleBondingCurve.onlyLaunchpadOwner()()
```
#### SimpleBondingCurve.supportsInterface(bytes4) [EXTERNAL]
```slithir
 interfaceId == type()(IERC165).interfaceId || interfaceId == type()(IBondingCurveMinimal).interfaceId
TMP_2120(type(IERC165)) = SOLIDITY_CALL type()(IERC165)
REF_1020(bytes4) (->None) := 33540519(bytes4)
TMP_2121(bool) = interfaceId_1 == REF_1020
TMP_2122(type(IBondingCurveMinimal)) = SOLIDITY_CALL type()(IBondingCurveMinimal)
REF_1021(bytes4) (->None) := 1992782646(bytes4)
TMP_2123(bool) = interfaceId_1 == REF_1021
TMP_2124(bool) = TMP_2121 || TMP_2123
RETURN TMP_2124
```
#### SimpleBondingCurve.totalSupply(address) [EXTERNAL]
```slithir
supply_2(mapping(address => SimpleBondingCurve.Supply)) := phi(['supply_0', 'supply_6', 'supply_3', 'supply_2', 'supply_1'])
 supply[token].totalSupply
REF_1003(SimpleBondingCurve.Supply) -> supply_2[token_1]
REF_1004(uint256) -> REF_1003.totalSupply
RETURN REF_1004
```
