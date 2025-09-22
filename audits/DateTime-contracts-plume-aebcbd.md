
#### DateTime.isLeapYear(uint16) [PUBLIC]
```slithir
year_1(uint16) := phi(['year_1', 'year_1', 'i_2', 'TMP_23326'])
 year % 4 != 0
TMP_23259(uint16) = year_1 % 4
TMP_23260(bool) = TMP_23259 != 0
CONDITION TMP_23260
 false
RETURN False
 year % 100 != 0
TMP_23261(uint16) = year_1 % 100
TMP_23262(bool) = TMP_23261 != 0
CONDITION TMP_23262
 true
RETURN True
 year % 400 != 0
TMP_23263(uint16) = year_1 % 400
TMP_23264(bool) = TMP_23263 != 0
CONDITION TMP_23264
 false
RETURN False
 true
RETURN True
```
#### DateTime.leapYearsBefore(uint256) [PUBLIC]
```slithir
year_1(uint256) := phi(['ORIGIN_YEAR_3', 'year_1', 'REF_9524', 'ORIGIN_YEAR_6'])
 year -= 1
year_2(uint256) = year_1 (c)- 1
 year / 4 - year / 100 + year / 400
TMP_23265(uint256) = year_2 (c)/ 4
TMP_23266(uint256) = year_2 (c)/ 100
TMP_23267(uint256) = TMP_23265 (c)- TMP_23266
TMP_23268(uint256) = year_2 (c)/ 400
TMP_23269(uint256) = TMP_23267 (c)+ TMP_23268
RETURN TMP_23269
```
#### DateTime.getDaysInMonth(uint8,uint16) [PUBLIC]
```slithir
month_1(uint8) := phi(['m_2', 'REF_9528', 'i_2'])
year_1(uint16) := phi(['REF_9526', 'year_1', 'REF_9529'])
 month == 1 || month == 3 || month == 5 || month == 7 || month == 8 || month == 10 || month == 12
TMP_23270(bool) = month_1 == 1
TMP_23271(bool) = month_1 == 3
TMP_23272(bool) = TMP_23270 || TMP_23271
TMP_23273(bool) = month_1 == 5
TMP_23274(bool) = TMP_23272 || TMP_23273
TMP_23275(bool) = month_1 == 7
TMP_23276(bool) = TMP_23274 || TMP_23275
TMP_23277(bool) = month_1 == 8
TMP_23278(bool) = TMP_23276 || TMP_23277
TMP_23279(bool) = month_1 == 10
TMP_23280(bool) = TMP_23278 || TMP_23279
TMP_23281(bool) = month_1 == 12
TMP_23282(bool) = TMP_23280 || TMP_23281
CONDITION TMP_23282
 31
RETURN 31
 month == 4 || month == 6 || month == 9 || month == 11
TMP_23283(bool) = month_1 == 4
TMP_23284(bool) = month_1 == 6
TMP_23285(bool) = TMP_23283 || TMP_23284
TMP_23286(bool) = month_1 == 9
TMP_23287(bool) = TMP_23285 || TMP_23286
TMP_23288(bool) = month_1 == 11
TMP_23289(bool) = TMP_23287 || TMP_23288
CONDITION TMP_23289
 30
RETURN 30
 isLeapYear(year)
TMP_23290(bool) = INTERNAL_CALL, DateTime.isLeapYear(uint16)(year_1)
CONDITION TMP_23290
 29
RETURN 29
 28
RETURN 28
```
#### DateTime.parseTimestamp(uint256) [INTERNAL]
```slithir
timestamp_1(uint256) := phi(['timestamp_1', 'timestamp_1', 'timestamp_1'])
DAY_IN_SECONDS_1(uint256) := phi(['DAY_IN_SECONDS_10', 'DAY_IN_SECONDS_0', 'DAY_IN_SECONDS_6'])
YEAR_IN_SECONDS_1(uint256) := phi(['YEAR_IN_SECONDS_0', 'YEAR_IN_SECONDS_4', 'YEAR_IN_SECONDS_10', 'YEAR_IN_SECONDS_7'])
LEAP_YEAR_IN_SECONDS_1(uint256) := phi(['LEAP_YEAR_IN_SECONDS_4', 'LEAP_YEAR_IN_SECONDS_10', 'LEAP_YEAR_IN_SECONDS_0', 'LEAP_YEAR_IN_SECONDS_7'])
ORIGIN_YEAR_1(uint16) := phi(['ORIGIN_YEAR_7', 'ORIGIN_YEAR_0', 'ORIGIN_YEAR_4'])
 secondsAccountedFor = 0
secondsAccountedFor_1(uint256) := 0(uint256)
 dt.year = getYear(timestamp)
REF_9523(uint16) -> dt_0.year
TMP_23291(uint16) = INTERNAL_CALL, DateTime.getYear(uint256)(timestamp_1)
YEAR_IN_SECONDS_2(uint256) := phi(['YEAR_IN_SECONDS_7'])
LEAP_YEAR_IN_SECONDS_2(uint256) := phi(['LEAP_YEAR_IN_SECONDS_7'])
ORIGIN_YEAR_2(uint16) := phi(['ORIGIN_YEAR_7'])
dt_1(DateTime._DateTime) := phi(['dt_0'])
REF_9523(uint16) (->dt_1) := TMP_23291(uint16)
 buf = leapYearsBefore(dt.year) - leapYearsBefore(ORIGIN_YEAR)
REF_9524(uint16) -> dt_1.year
TMP_23292(uint256) = INTERNAL_CALL, DateTime.leapYearsBefore(uint256)(REF_9524)
TMP_23293(uint256) = INTERNAL_CALL, DateTime.leapYearsBefore(uint256)(ORIGIN_YEAR_3)
TMP_23294(uint256) = TMP_23292 (c)- TMP_23293
buf_1(uint256) := TMP_23294(uint256)
 secondsAccountedFor += LEAP_YEAR_IN_SECONDS * buf
TMP_23295(uint256) = LEAP_YEAR_IN_SECONDS_4 (c)* buf_1
secondsAccountedFor_2(uint256) = secondsAccountedFor_1 (c)+ TMP_23295
 secondsAccountedFor += YEAR_IN_SECONDS * (dt.year - ORIGIN_YEAR - buf)
REF_9525(uint16) -> dt_1.year
TMP_23296(uint16) = REF_9525 (c)- ORIGIN_YEAR_4
TMP_23297(uint16) = TMP_23296 (c)- buf_1
TMP_23298(uint256) = YEAR_IN_SECONDS_4 (c)* TMP_23297
secondsAccountedFor_3(uint256) = secondsAccountedFor_2 (c)+ TMP_23298
dt_3(DateTime._DateTime) := phi(['dt_1', 'dt_2'])
 i = 1
i_1(uint8) := 1(uint256)
 i <= 12
secondsAccountedFor_4(uint256) := phi(['secondsAccountedFor_5', 'secondsAccountedFor_3'])
i_2(uint8) := phi(['i_1', 'i_3'])
TMP_23299(bool) = i_2 <= 12
CONDITION TMP_23299
 secondsInMonth = DAY_IN_SECONDS * getDaysInMonth(i,dt.year)
REF_9526(uint16) -> dt_1.year
TMP_23300(uint8) = INTERNAL_CALL, DateTime.getDaysInMonth(uint8,uint16)(i_2,REF_9526)
TMP_23301(uint256) = DAY_IN_SECONDS_5 (c)* TMP_23300
secondsInMonth_1(uint256) := TMP_23301(uint256)
 secondsInMonth + secondsAccountedFor > timestamp
TMP_23302(uint256) = secondsInMonth_1 (c)+ secondsAccountedFor_4
TMP_23303(bool) = TMP_23302 > timestamp_1
CONDITION TMP_23303
 dt.month = i
REF_9527(uint8) -> dt_1.month
dt_2(DateTime._DateTime) := phi(['dt_1'])
REF_9527(uint8) (->dt_2) := i_2(uint8)
 secondsAccountedFor += secondsInMonth
secondsAccountedFor_5(uint256) = secondsAccountedFor_4 (c)+ secondsInMonth_1
 i ++
TMP_23304(uint8) := i_2(uint8)
i_3(uint8) = i_2 (c)+ 1
dt_5(DateTime._DateTime) := phi(['dt_1', 'dt_4'])
 i = 1
i_4(uint8) := 1(uint256)
 i <= getDaysInMonth(dt.month,dt.year)
secondsAccountedFor_6(uint256) := phi(['secondsAccountedFor_3', 'secondsAccountedFor_7'])
i_5(uint8) := phi(['i_4', 'i_6'])
REF_9528(uint8) -> dt_3.month
REF_9529(uint16) -> dt_3.year
TMP_23305(uint8) = INTERNAL_CALL, DateTime.getDaysInMonth(uint8,uint16)(REF_9528,REF_9529)
TMP_23306(bool) = i_5 <= TMP_23305
CONDITION TMP_23306
 DAY_IN_SECONDS + secondsAccountedFor > timestamp
TMP_23307(uint256) = DAY_IN_SECONDS_6 (c)+ secondsAccountedFor_6
TMP_23308(bool) = TMP_23307 > timestamp_1
CONDITION TMP_23308
 dt.day = i
REF_9530(uint8) -> dt_3.day
dt_4(DateTime._DateTime) := phi(['dt_3'])
REF_9530(uint8) (->dt_4) := i_5(uint8)
 secondsAccountedFor += DAY_IN_SECONDS
secondsAccountedFor_7(uint256) = secondsAccountedFor_6 (c)+ DAY_IN_SECONDS_6
 i ++
TMP_23309(uint8) := i_5(uint8)
i_6(uint8) = i_5 (c)+ 1
 dt.hour = getHour(timestamp)
REF_9531(uint8) -> dt_5.hour
TMP_23310(uint8) = INTERNAL_CALL, DateTime.getHour(uint256)(timestamp_1)
dt_6(DateTime._DateTime) := phi(['dt_5'])
REF_9531(uint8) (->dt_6) := TMP_23310(uint8)
 dt.minute = getMinute(timestamp)
REF_9532(uint8) -> dt_6.minute
TMP_23311(uint8) = INTERNAL_CALL, DateTime.getMinute(uint256)(timestamp_1)
dt_7(DateTime._DateTime) := phi(['dt_6'])
REF_9532(uint8) (->dt_7) := TMP_23311(uint8)
 dt.second = getSecond(timestamp)
REF_9533(uint8) -> dt_7.second
TMP_23312(uint8) = INTERNAL_CALL, DateTime.getSecond(uint256)(timestamp_1)
dt_8(DateTime._DateTime) := phi(['dt_7'])
REF_9533(uint8) (->dt_8) := TMP_23312(uint8)
 dt.weekday = getWeekday(timestamp)
REF_9534(uint8) -> dt_8.weekday
TMP_23313(uint8) = INTERNAL_CALL, DateTime.getWeekday(uint256)(timestamp_1)
dt_9(DateTime._DateTime) := phi(['dt_8'])
REF_9534(uint8) (->dt_9) := TMP_23313(uint8)
 dt
RETURN dt_9
```
#### DateTime.getYear(uint256) [PUBLIC]
```slithir
timestamp_1(uint256) := phi(['timestamp_1'])
YEAR_IN_SECONDS_5(uint256) := phi(['YEAR_IN_SECONDS_0', 'YEAR_IN_SECONDS_4', 'YEAR_IN_SECONDS_10', 'YEAR_IN_SECONDS_7'])
LEAP_YEAR_IN_SECONDS_5(uint256) := phi(['LEAP_YEAR_IN_SECONDS_4', 'LEAP_YEAR_IN_SECONDS_10', 'LEAP_YEAR_IN_SECONDS_0', 'LEAP_YEAR_IN_SECONDS_7'])
ORIGIN_YEAR_5(uint16) := phi(['ORIGIN_YEAR_7', 'ORIGIN_YEAR_0', 'ORIGIN_YEAR_4'])
 secondsAccountedFor = 0
secondsAccountedFor_1(uint256) := 0(uint256)
 year = uint16(ORIGIN_YEAR + timestamp / YEAR_IN_SECONDS)
TMP_23314(uint256) = timestamp_1 (c)/ YEAR_IN_SECONDS_5
TMP_23315(uint16) = ORIGIN_YEAR_5 (c)+ TMP_23314
TMP_23316 = CONVERT TMP_23315 to uint16
year_1(uint16) := TMP_23316(uint16)
 numLeapYears = leapYearsBefore(year) - leapYearsBefore(ORIGIN_YEAR)
TMP_23317(uint256) = INTERNAL_CALL, DateTime.leapYearsBefore(uint256)(year_1)
TMP_23318(uint256) = INTERNAL_CALL, DateTime.leapYearsBefore(uint256)(ORIGIN_YEAR_6)
TMP_23319(uint256) = TMP_23317 (c)- TMP_23318
numLeapYears_1(uint256) := TMP_23319(uint256)
 secondsAccountedFor += LEAP_YEAR_IN_SECONDS * numLeapYears
TMP_23320(uint256) = LEAP_YEAR_IN_SECONDS_7 (c)* numLeapYears_1
secondsAccountedFor_2(uint256) = secondsAccountedFor_1 (c)+ TMP_23320
 secondsAccountedFor += YEAR_IN_SECONDS * (year - ORIGIN_YEAR - numLeapYears)
TMP_23321(uint16) = year_1 (c)- ORIGIN_YEAR_7
TMP_23322(uint16) = TMP_23321 (c)- numLeapYears_1
TMP_23323(uint256) = YEAR_IN_SECONDS_7 (c)* TMP_23322
secondsAccountedFor_3(uint256) = secondsAccountedFor_2 (c)+ TMP_23323
 secondsAccountedFor > timestamp
year_2(uint16) := phi(['year_1', 'year_3'])
TMP_23324(bool) = secondsAccountedFor_3 > timestamp_1
CONDITION TMP_23324
 isLeapYear(uint16(year - 1))
TMP_23325(uint16) = year_2 (c)- 1
TMP_23326 = CONVERT TMP_23325 to uint16
TMP_23327(bool) = INTERNAL_CALL, DateTime.isLeapYear(uint16)(TMP_23326)
CONDITION TMP_23327
 secondsAccountedFor -= LEAP_YEAR_IN_SECONDS
secondsAccountedFor_5(uint256) = secondsAccountedFor_3 (c)- LEAP_YEAR_IN_SECONDS_8
 secondsAccountedFor -= YEAR_IN_SECONDS
secondsAccountedFor_4(uint256) = secondsAccountedFor_3 (c)- YEAR_IN_SECONDS_8
secondsAccountedFor_6(uint256) := phi(['secondsAccountedFor_5', 'secondsAccountedFor_4'])
 year -= 1
year_3(uint16) = year_2 (c)- 1
 year
RETURN year_2
```
#### DateTime.getMonth(uint256) [PUBLIC]
```slithir
 parseTimestamp(timestamp).month
TMP_23328(DateTime._DateTime) = INTERNAL_CALL, DateTime.parseTimestamp(uint256)(timestamp_1)
REF_9535(uint8) -> TMP_23328.month
RETURN REF_9535
```
#### DateTime.getDay(uint256) [PUBLIC]
```slithir
 parseTimestamp(timestamp).day
TMP_23329(DateTime._DateTime) = INTERNAL_CALL, DateTime.parseTimestamp(uint256)(timestamp_1)
REF_9536(uint8) -> TMP_23329.day
RETURN REF_9536
```
#### DateTime.getHour(uint256) [PUBLIC]
```slithir
timestamp_1(uint256) := phi(['timestamp_1'])
 uint8((timestamp / 60 / 60) % 24)
TMP_23330(uint256) = timestamp_1 (c)/ 60
TMP_23331(uint256) = TMP_23330 (c)/ 60
TMP_23332(uint256) = TMP_23331 % 24
TMP_23333 = CONVERT TMP_23332 to uint8
RETURN TMP_23333
```
#### DateTime.getMinute(uint256) [PUBLIC]
```slithir
timestamp_1(uint256) := phi(['timestamp_1'])
 uint8((timestamp / 60) % 60)
TMP_23334(uint256) = timestamp_1 (c)/ 60
TMP_23335(uint256) = TMP_23334 % 60
TMP_23336 = CONVERT TMP_23335 to uint8
RETURN TMP_23336
```
#### DateTime.getSecond(uint256) [PUBLIC]
```slithir
timestamp_1(uint256) := phi(['timestamp_1'])
 uint8(timestamp % 60)
TMP_23337(uint256) = timestamp_1 % 60
TMP_23338 = CONVERT TMP_23337 to uint8
RETURN TMP_23338
```
#### DateTime.getWeekday(uint256) [PUBLIC]
```slithir
timestamp_1(uint256) := phi(['timestamp_1', 'timestamp_1'])
DAY_IN_SECONDS_7(uint256) := phi(['DAY_IN_SECONDS_10', 'DAY_IN_SECONDS_0', 'DAY_IN_SECONDS_6'])
 uint8((timestamp / DAY_IN_SECONDS + 4) % 7)
TMP_23339(uint256) = timestamp_1 (c)/ DAY_IN_SECONDS_7
TMP_23340(uint256) = TMP_23339 (c)+ 4
TMP_23341(uint256) = TMP_23340 % 7
TMP_23342 = CONVERT TMP_23341 to uint8
RETURN TMP_23342
```
#### DateTime.toTimestamp(uint16,uint8,uint8,uint8,uint8,uint8) [PUBLIC]
```slithir
year_1(uint16) := phi(['year_1', 'year_1', 'year_1'])
month_1(uint8) := phi(['month_1', 'month_1', 'month_1'])
day_1(uint8) := phi(['day_1', 'day_1', 'day_1'])
hour_1(uint8) := phi(['hour_1', 'hour_1'])
minute_1(uint8) := phi(['minute_1'])
DAY_IN_SECONDS_8(uint256) := phi(['DAY_IN_SECONDS_10', 'DAY_IN_SECONDS_0', 'DAY_IN_SECONDS_6'])
YEAR_IN_SECONDS_9(uint256) := phi(['YEAR_IN_SECONDS_0', 'YEAR_IN_SECONDS_4', 'YEAR_IN_SECONDS_10', 'YEAR_IN_SECONDS_7'])
LEAP_YEAR_IN_SECONDS_9(uint256) := phi(['LEAP_YEAR_IN_SECONDS_4', 'LEAP_YEAR_IN_SECONDS_10', 'LEAP_YEAR_IN_SECONDS_0', 'LEAP_YEAR_IN_SECONDS_7'])
HOUR_IN_SECONDS_1(uint256) := phi(['HOUR_IN_SECONDS_3', 'HOUR_IN_SECONDS_0'])
MINUTE_IN_SECONDS_1(uint256) := phi(['MINUTE_IN_SECONDS_0', 'MINUTE_IN_SECONDS_3'])
ORIGIN_YEAR_8(uint16) := phi(['ORIGIN_YEAR_7', 'ORIGIN_YEAR_0', 'ORIGIN_YEAR_4'])
 i = ORIGIN_YEAR
i_1(uint16) := ORIGIN_YEAR_8(uint16)
 i < year
i_2(uint16) := phi(['i_1', 'i_3'])
TMP_23346(bool) = i_2 < year_1
CONDITION TMP_23346
 isLeapYear(i)
TMP_23347(bool) = INTERNAL_CALL, DateTime.isLeapYear(uint16)(i_2)
CONDITION TMP_23347
 timestamp += LEAP_YEAR_IN_SECONDS
timestamp_1(uint256) = timestamp_0 (c)+ LEAP_YEAR_IN_SECONDS_10
 timestamp += YEAR_IN_SECONDS
timestamp_2(uint256) = timestamp_0 (c)+ YEAR_IN_SECONDS_10
timestamp_3(uint256) := phi(['timestamp_1', 'timestamp_2'])
 i ++
TMP_23348(uint16) := i_2(uint16)
i_3(uint16) = i_2 (c)+ 1
 monthDayCounts[0] = 31
REF_9537(uint8) -> monthDayCounts_0[0]
monthDayCounts_1(uint8[12]) := phi(['monthDayCounts_0'])
REF_9537(uint8) (->monthDayCounts_1) := 31(uint256)
 isLeapYear(year)
TMP_23349(bool) = INTERNAL_CALL, DateTime.isLeapYear(uint16)(year_1)
CONDITION TMP_23349
 monthDayCounts[1] = 29
REF_9538(uint8) -> monthDayCounts_1[1]
monthDayCounts_2(uint8[12]) := phi(['monthDayCounts_1'])
REF_9538(uint8) (->monthDayCounts_2) := 29(uint256)
 monthDayCounts[1] = 28
REF_9539(uint8) -> monthDayCounts_1[1]
monthDayCounts_3(uint8[12]) := phi(['monthDayCounts_1'])
REF_9539(uint8) (->monthDayCounts_3) := 28(uint256)
 monthDayCounts[2] = 31
REF_9540(uint8) -> monthDayCounts_3[2]
monthDayCounts_4(uint8[12]) := phi(['monthDayCounts_3'])
REF_9540(uint8) (->monthDayCounts_4) := 31(uint256)
 monthDayCounts[3] = 30
REF_9541(uint8) -> monthDayCounts_4[3]
monthDayCounts_5(uint8[12]) := phi(['monthDayCounts_4'])
REF_9541(uint8) (->monthDayCounts_5) := 30(uint256)
 monthDayCounts[4] = 31
REF_9542(uint8) -> monthDayCounts_5[4]
monthDayCounts_6(uint8[12]) := phi(['monthDayCounts_5'])
REF_9542(uint8) (->monthDayCounts_6) := 31(uint256)
 monthDayCounts[5] = 30
REF_9543(uint8) -> monthDayCounts_6[5]
monthDayCounts_7(uint8[12]) := phi(['monthDayCounts_6'])
REF_9543(uint8) (->monthDayCounts_7) := 30(uint256)
 monthDayCounts[6] = 31
REF_9544(uint8) -> monthDayCounts_7[6]
monthDayCounts_8(uint8[12]) := phi(['monthDayCounts_7'])
REF_9544(uint8) (->monthDayCounts_8) := 31(uint256)
 monthDayCounts[7] = 31
REF_9545(uint8) -> monthDayCounts_8[7]
monthDayCounts_9(uint8[12]) := phi(['monthDayCounts_8'])
REF_9545(uint8) (->monthDayCounts_9) := 31(uint256)
 monthDayCounts[8] = 30
REF_9546(uint8) -> monthDayCounts_9[8]
monthDayCounts_10(uint8[12]) := phi(['monthDayCounts_9'])
REF_9546(uint8) (->monthDayCounts_10) := 30(uint256)
 monthDayCounts[9] = 31
REF_9547(uint8) -> monthDayCounts_10[9]
monthDayCounts_11(uint8[12]) := phi(['monthDayCounts_10'])
REF_9547(uint8) (->monthDayCounts_11) := 31(uint256)
 monthDayCounts[10] = 30
REF_9548(uint8) -> monthDayCounts_11[10]
monthDayCounts_12(uint8[12]) := phi(['monthDayCounts_11'])
REF_9548(uint8) (->monthDayCounts_12) := 30(uint256)
 monthDayCounts[11] = 31
REF_9549(uint8) -> monthDayCounts_12[11]
monthDayCounts_13(uint8[12]) := phi(['monthDayCounts_12'])
REF_9549(uint8) (->monthDayCounts_13) := 31(uint256)
 i = 1
i_4(uint16) := 1(uint256)
 i < month
timestamp_4(uint256) := phi(['timestamp_5', 'timestamp_0'])
i_5(uint16) := phi(['i_4', 'i_6'])
TMP_23350(bool) = i_5 < month_1
CONDITION TMP_23350
 timestamp += DAY_IN_SECONDS * monthDayCounts[i - 1]
TMP_23351(uint16) = i_5 (c)- 1
REF_9550(uint8) -> monthDayCounts_13[TMP_23351]
TMP_23352(uint256) = DAY_IN_SECONDS_10 (c)* REF_9550
timestamp_5(uint256) = timestamp_4 (c)+ TMP_23352
 i ++
TMP_23353(uint16) := i_5(uint16)
i_6(uint16) = i_5 (c)+ 1
 timestamp += DAY_IN_SECONDS * (day - 1)
TMP_23354(uint8) = day_1 (c)- 1
TMP_23355(uint256) = DAY_IN_SECONDS_10 (c)* TMP_23354
timestamp_6(uint256) = timestamp_4 (c)+ TMP_23355
 timestamp += HOUR_IN_SECONDS * (hour)
TMP_23356(uint256) = HOUR_IN_SECONDS_3 (c)* hour_1
timestamp_7(uint256) = timestamp_6 (c)+ TMP_23356
 timestamp += MINUTE_IN_SECONDS * (minute)
TMP_23357(uint256) = MINUTE_IN_SECONDS_3 (c)* minute_1
timestamp_8(uint256) = timestamp_7 (c)+ TMP_23357
 timestamp += second
timestamp_9(uint256) = timestamp_8 (c)+ second_1
 timestamp
RETURN timestamp_9
 timestamp
```
#### DateTime.getWeekNumber(uint256) [PUBLIC]
```slithir
 dt = parseTimestamp(timestamp)
TMP_23358(DateTime._DateTime) = INTERNAL_CALL, DateTime.parseTimestamp(uint256)(timestamp_1)
dt_1(DateTime._DateTime) := TMP_23358(DateTime._DateTime)
 year = dt.year
REF_9551(uint16) -> dt_1.year
year_1(uint16) := REF_9551(uint16)
 month = dt.month
REF_9552(uint8) -> dt_1.month
month_1(uint8) := REF_9552(uint8)
 day = dt.day
REF_9553(uint8) -> dt_1.day
day_1(uint8) := REF_9553(uint8)
 weekday = getWeekday(timestamp)
TMP_23359(uint8) = INTERNAL_CALL, DateTime.getWeekday(uint256)(timestamp_1)
weekday_1(uint8) := TMP_23359(uint8)
 weekday == 0
TMP_23360(bool) = weekday_1 == 0
CONDITION TMP_23360
 weekday = 7
weekday_2(uint8) := 7(uint256)
weekday_3(uint8) := phi(['weekday_2', 'weekday_1'])
 daysSinceYearStart = getDaysSinceYearStart(year,month,day)
TMP_23361(uint256) = INTERNAL_CALL, DateTime.getDaysSinceYearStart(uint16,uint8,uint8)(year_1,month_1,day_1)
daysSinceYearStart_1(uint256) := TMP_23361(uint256)
 weekNumber = (daysSinceYearStart + 7 - weekday) / 7
TMP_23362(uint256) = daysSinceYearStart_1 (c)+ 7
TMP_23363(uint256) = TMP_23362 (c)- weekday_3
TMP_23364(uint256) = TMP_23363 (c)/ 7
weekNumber_1(uint256) := TMP_23364(uint256)
 uint8(weekNumber)
TMP_23365 = CONVERT weekNumber_1 to uint8
RETURN TMP_23365
```
#### DateTime.getDaysSinceYearStart(uint16,uint8,uint8) [INTERNAL]
```slithir
year_1(uint16) := phi(['year_1'])
month_1(uint8) := phi(['month_1'])
day_1(uint8) := phi(['day_1'])
 days_ = day
days__1(uint256) := day_1(uint8)
 m = 1
m_1(uint8) := 1(uint256)
 m < month
days__2(uint256) := phi(['days__1', 'days__3'])
m_2(uint8) := phi(['m_3', 'm_1'])
TMP_23366(bool) = m_2 < month_1
CONDITION TMP_23366
 days_ += getDaysInMonth(m,year)
TMP_23367(uint8) = INTERNAL_CALL, DateTime.getDaysInMonth(uint8,uint16)(m_2,year_1)
days__3(uint256) = days__2 (c)+ TMP_23367
 m ++
TMP_23368(uint8) := m_2(uint8)
m_3(uint8) = m_2 (c)+ 1
 days_
RETURN days__2
```
#### SpinProxy.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 PROXY_NAME = keccak256(bytes)(SpinProxy)
```
