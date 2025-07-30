
#### DateTime.isLeapYear(uint16) [PUBLIC]
```slithir
year_1(uint16) := phi(['year_1', 'year_1', 'i_2', 'TMP_23548'])
 year % 4 != 0
TMP_23481(uint16) = year_1 % 4
TMP_23482(bool) = TMP_23481 != 0
CONDITION TMP_23482
 false
RETURN False
 year % 100 != 0
TMP_23483(uint16) = year_1 % 100
TMP_23484(bool) = TMP_23483 != 0
CONDITION TMP_23484
 true
RETURN True
 year % 400 != 0
TMP_23485(uint16) = year_1 % 400
TMP_23486(bool) = TMP_23485 != 0
CONDITION TMP_23486
 false
RETURN False
 true
RETURN True
```
#### DateTime.leapYearsBefore(uint256) [PUBLIC]
```slithir
year_1(uint256) := phi(['ORIGIN_YEAR_3', 'ORIGIN_YEAR_6', 'REF_9669', 'year_1'])
 year -= 1
year_2(uint256) = year_1 (c)- 1
 year / 4 - year / 100 + year / 400
TMP_23487(uint256) = year_2 (c)/ 4
TMP_23488(uint256) = year_2 (c)/ 100
TMP_23489(uint256) = TMP_23487 (c)- TMP_23488
TMP_23490(uint256) = year_2 (c)/ 400
TMP_23491(uint256) = TMP_23489 (c)+ TMP_23490
RETURN TMP_23491
```
#### DateTime.getDaysInMonth(uint8,uint16) [PUBLIC]
```slithir
month_1(uint8) := phi(['m_2', 'REF_9673', 'i_2'])
year_1(uint16) := phi(['REF_9671', 'REF_9674', 'year_1'])
 month == 1 || month == 3 || month == 5 || month == 7 || month == 8 || month == 10 || month == 12
TMP_23492(bool) = month_1 == 1
TMP_23493(bool) = month_1 == 3
TMP_23494(bool) = TMP_23492 || TMP_23493
TMP_23495(bool) = month_1 == 5
TMP_23496(bool) = TMP_23494 || TMP_23495
TMP_23497(bool) = month_1 == 7
TMP_23498(bool) = TMP_23496 || TMP_23497
TMP_23499(bool) = month_1 == 8
TMP_23500(bool) = TMP_23498 || TMP_23499
TMP_23501(bool) = month_1 == 10
TMP_23502(bool) = TMP_23500 || TMP_23501
TMP_23503(bool) = month_1 == 12
TMP_23504(bool) = TMP_23502 || TMP_23503
CONDITION TMP_23504
 31
RETURN 31
 month == 4 || month == 6 || month == 9 || month == 11
TMP_23505(bool) = month_1 == 4
TMP_23506(bool) = month_1 == 6
TMP_23507(bool) = TMP_23505 || TMP_23506
TMP_23508(bool) = month_1 == 9
TMP_23509(bool) = TMP_23507 || TMP_23508
TMP_23510(bool) = month_1 == 11
TMP_23511(bool) = TMP_23509 || TMP_23510
CONDITION TMP_23511
 30
RETURN 30
 isLeapYear(year)
TMP_23512(bool) = INTERNAL_CALL, DateTime.isLeapYear(uint16)(year_1)
CONDITION TMP_23512
 29
RETURN 29
 28
RETURN 28
```
#### DateTime.parseTimestamp(uint256) [INTERNAL]
```slithir
timestamp_1(uint256) := phi(['timestamp_1', 'timestamp_1', 'timestamp_1'])
DAY_IN_SECONDS_1(uint256) := phi(['DAY_IN_SECONDS_6', 'DAY_IN_SECONDS_0', 'DAY_IN_SECONDS_10'])
YEAR_IN_SECONDS_1(uint256) := phi(['YEAR_IN_SECONDS_0', 'YEAR_IN_SECONDS_7', 'YEAR_IN_SECONDS_4', 'YEAR_IN_SECONDS_10'])
LEAP_YEAR_IN_SECONDS_1(uint256) := phi(['LEAP_YEAR_IN_SECONDS_0', 'LEAP_YEAR_IN_SECONDS_7', 'LEAP_YEAR_IN_SECONDS_10', 'LEAP_YEAR_IN_SECONDS_4'])
ORIGIN_YEAR_1(uint16) := phi(['ORIGIN_YEAR_7', 'ORIGIN_YEAR_0', 'ORIGIN_YEAR_4'])
 secondsAccountedFor = 0
secondsAccountedFor_1(uint256) := 0(uint256)
 dt.year = getYear(timestamp)
REF_9668(uint16) -> dt_0.year
TMP_23513(uint16) = INTERNAL_CALL, DateTime.getYear(uint256)(timestamp_1)
YEAR_IN_SECONDS_2(uint256) := phi(['YEAR_IN_SECONDS_7'])
LEAP_YEAR_IN_SECONDS_2(uint256) := phi(['LEAP_YEAR_IN_SECONDS_7'])
ORIGIN_YEAR_2(uint16) := phi(['ORIGIN_YEAR_7'])
dt_1(DateTime._DateTime) := phi(['dt_0'])
REF_9668(uint16) (->dt_1) := TMP_23513(uint16)
 buf = leapYearsBefore(dt.year) - leapYearsBefore(ORIGIN_YEAR)
REF_9669(uint16) -> dt_1.year
TMP_23514(uint256) = INTERNAL_CALL, DateTime.leapYearsBefore(uint256)(REF_9669)
TMP_23515(uint256) = INTERNAL_CALL, DateTime.leapYearsBefore(uint256)(ORIGIN_YEAR_3)
TMP_23516(uint256) = TMP_23514 (c)- TMP_23515
buf_1(uint256) := TMP_23516(uint256)
 secondsAccountedFor += LEAP_YEAR_IN_SECONDS * buf
TMP_23517(uint256) = LEAP_YEAR_IN_SECONDS_4 (c)* buf_1
secondsAccountedFor_2(uint256) = secondsAccountedFor_1 (c)+ TMP_23517
 secondsAccountedFor += YEAR_IN_SECONDS * (dt.year - ORIGIN_YEAR - buf)
REF_9670(uint16) -> dt_1.year
TMP_23518(uint16) = REF_9670 (c)- ORIGIN_YEAR_4
TMP_23519(uint16) = TMP_23518 (c)- buf_1
TMP_23520(uint256) = YEAR_IN_SECONDS_4 (c)* TMP_23519
secondsAccountedFor_3(uint256) = secondsAccountedFor_2 (c)+ TMP_23520
dt_3(DateTime._DateTime) := phi(['dt_2', 'dt_1'])
 i = 1
i_1(uint8) := 1(uint256)
 i <= 12
secondsAccountedFor_4(uint256) := phi(['secondsAccountedFor_3', 'secondsAccountedFor_5'])
i_2(uint8) := phi(['i_3', 'i_1'])
TMP_23521(bool) = i_2 <= 12
CONDITION TMP_23521
 secondsInMonth = DAY_IN_SECONDS * getDaysInMonth(i,dt.year)
REF_9671(uint16) -> dt_1.year
TMP_23522(uint8) = INTERNAL_CALL, DateTime.getDaysInMonth(uint8,uint16)(i_2,REF_9671)
TMP_23523(uint256) = DAY_IN_SECONDS_5 (c)* TMP_23522
secondsInMonth_1(uint256) := TMP_23523(uint256)
 secondsInMonth + secondsAccountedFor > timestamp
TMP_23524(uint256) = secondsInMonth_1 (c)+ secondsAccountedFor_4
TMP_23525(bool) = TMP_23524 > timestamp_1
CONDITION TMP_23525
 dt.month = i
REF_9672(uint8) -> dt_1.month
dt_2(DateTime._DateTime) := phi(['dt_1'])
REF_9672(uint8) (->dt_2) := i_2(uint8)
 secondsAccountedFor += secondsInMonth
secondsAccountedFor_5(uint256) = secondsAccountedFor_4 (c)+ secondsInMonth_1
 i ++
TMP_23526(uint8) := i_2(uint8)
i_3(uint8) = i_2 (c)+ 1
dt_5(DateTime._DateTime) := phi(['dt_1', 'dt_4'])
 i = 1
i_4(uint8) := 1(uint256)
 i <= getDaysInMonth(dt.month,dt.year)
secondsAccountedFor_6(uint256) := phi(['secondsAccountedFor_3', 'secondsAccountedFor_7'])
i_5(uint8) := phi(['i_6', 'i_4'])
REF_9673(uint8) -> dt_3.month
REF_9674(uint16) -> dt_3.year
TMP_23527(uint8) = INTERNAL_CALL, DateTime.getDaysInMonth(uint8,uint16)(REF_9673,REF_9674)
TMP_23528(bool) = i_5 <= TMP_23527
CONDITION TMP_23528
 DAY_IN_SECONDS + secondsAccountedFor > timestamp
TMP_23529(uint256) = DAY_IN_SECONDS_6 (c)+ secondsAccountedFor_6
TMP_23530(bool) = TMP_23529 > timestamp_1
CONDITION TMP_23530
 dt.day = i
REF_9675(uint8) -> dt_3.day
dt_4(DateTime._DateTime) := phi(['dt_3'])
REF_9675(uint8) (->dt_4) := i_5(uint8)
 secondsAccountedFor += DAY_IN_SECONDS
secondsAccountedFor_7(uint256) = secondsAccountedFor_6 (c)+ DAY_IN_SECONDS_6
 i ++
TMP_23531(uint8) := i_5(uint8)
i_6(uint8) = i_5 (c)+ 1
 dt.hour = getHour(timestamp)
REF_9676(uint8) -> dt_5.hour
TMP_23532(uint8) = INTERNAL_CALL, DateTime.getHour(uint256)(timestamp_1)
dt_6(DateTime._DateTime) := phi(['dt_5'])
REF_9676(uint8) (->dt_6) := TMP_23532(uint8)
 dt.minute = getMinute(timestamp)
REF_9677(uint8) -> dt_6.minute
TMP_23533(uint8) = INTERNAL_CALL, DateTime.getMinute(uint256)(timestamp_1)
dt_7(DateTime._DateTime) := phi(['dt_6'])
REF_9677(uint8) (->dt_7) := TMP_23533(uint8)
 dt.second = getSecond(timestamp)
REF_9678(uint8) -> dt_7.second
TMP_23534(uint8) = INTERNAL_CALL, DateTime.getSecond(uint256)(timestamp_1)
dt_8(DateTime._DateTime) := phi(['dt_7'])
REF_9678(uint8) (->dt_8) := TMP_23534(uint8)
 dt.weekday = getWeekday(timestamp)
REF_9679(uint8) -> dt_8.weekday
TMP_23535(uint8) = INTERNAL_CALL, DateTime.getWeekday(uint256)(timestamp_1)
dt_9(DateTime._DateTime) := phi(['dt_8'])
REF_9679(uint8) (->dt_9) := TMP_23535(uint8)
 dt
RETURN dt_9
```
#### DateTime.getYear(uint256) [PUBLIC]
```slithir
timestamp_1(uint256) := phi(['timestamp_1'])
YEAR_IN_SECONDS_5(uint256) := phi(['YEAR_IN_SECONDS_0', 'YEAR_IN_SECONDS_7', 'YEAR_IN_SECONDS_4', 'YEAR_IN_SECONDS_10'])
LEAP_YEAR_IN_SECONDS_5(uint256) := phi(['LEAP_YEAR_IN_SECONDS_0', 'LEAP_YEAR_IN_SECONDS_7', 'LEAP_YEAR_IN_SECONDS_10', 'LEAP_YEAR_IN_SECONDS_4'])
ORIGIN_YEAR_5(uint16) := phi(['ORIGIN_YEAR_7', 'ORIGIN_YEAR_0', 'ORIGIN_YEAR_4'])
 secondsAccountedFor = 0
secondsAccountedFor_1(uint256) := 0(uint256)
 year = uint16(ORIGIN_YEAR + timestamp / YEAR_IN_SECONDS)
TMP_23536(uint256) = timestamp_1 (c)/ YEAR_IN_SECONDS_5
TMP_23537(uint16) = ORIGIN_YEAR_5 (c)+ TMP_23536
TMP_23538 = CONVERT TMP_23537 to uint16
year_1(uint16) := TMP_23538(uint16)
 numLeapYears = leapYearsBefore(year) - leapYearsBefore(ORIGIN_YEAR)
TMP_23539(uint256) = INTERNAL_CALL, DateTime.leapYearsBefore(uint256)(year_1)
TMP_23540(uint256) = INTERNAL_CALL, DateTime.leapYearsBefore(uint256)(ORIGIN_YEAR_6)
TMP_23541(uint256) = TMP_23539 (c)- TMP_23540
numLeapYears_1(uint256) := TMP_23541(uint256)
 secondsAccountedFor += LEAP_YEAR_IN_SECONDS * numLeapYears
TMP_23542(uint256) = LEAP_YEAR_IN_SECONDS_7 (c)* numLeapYears_1
secondsAccountedFor_2(uint256) = secondsAccountedFor_1 (c)+ TMP_23542
 secondsAccountedFor += YEAR_IN_SECONDS * (year - ORIGIN_YEAR - numLeapYears)
TMP_23543(uint16) = year_1 (c)- ORIGIN_YEAR_7
TMP_23544(uint16) = TMP_23543 (c)- numLeapYears_1
TMP_23545(uint256) = YEAR_IN_SECONDS_7 (c)* TMP_23544
secondsAccountedFor_3(uint256) = secondsAccountedFor_2 (c)+ TMP_23545
 secondsAccountedFor > timestamp
year_2(uint16) := phi(['year_3', 'year_1'])
TMP_23546(bool) = secondsAccountedFor_3 > timestamp_1
CONDITION TMP_23546
 isLeapYear(uint16(year - 1))
TMP_23547(uint16) = year_2 (c)- 1
TMP_23548 = CONVERT TMP_23547 to uint16
TMP_23549(bool) = INTERNAL_CALL, DateTime.isLeapYear(uint16)(TMP_23548)
CONDITION TMP_23549
 secondsAccountedFor -= LEAP_YEAR_IN_SECONDS
secondsAccountedFor_4(uint256) = secondsAccountedFor_3 (c)- LEAP_YEAR_IN_SECONDS_8
 secondsAccountedFor -= YEAR_IN_SECONDS
secondsAccountedFor_5(uint256) = secondsAccountedFor_3 (c)- YEAR_IN_SECONDS_8
secondsAccountedFor_6(uint256) := phi(['secondsAccountedFor_4', 'secondsAccountedFor_5'])
 year -= 1
year_3(uint16) = year_2 (c)- 1
 year
RETURN year_2
```
#### DateTime.getMonth(uint256) [PUBLIC]
```slithir
 parseTimestamp(timestamp).month
TMP_23550(DateTime._DateTime) = INTERNAL_CALL, DateTime.parseTimestamp(uint256)(timestamp_1)
REF_9680(uint8) -> TMP_23550.month
RETURN REF_9680
```
#### DateTime.getDay(uint256) [PUBLIC]
```slithir
 parseTimestamp(timestamp).day
TMP_23551(DateTime._DateTime) = INTERNAL_CALL, DateTime.parseTimestamp(uint256)(timestamp_1)
REF_9681(uint8) -> TMP_23551.day
RETURN REF_9681
```
#### DateTime.getHour(uint256) [PUBLIC]
```slithir
timestamp_1(uint256) := phi(['timestamp_1'])
 uint8((timestamp / 60 / 60) % 24)
TMP_23552(uint256) = timestamp_1 (c)/ 60
TMP_23553(uint256) = TMP_23552 (c)/ 60
TMP_23554(uint256) = TMP_23553 % 24
TMP_23555 = CONVERT TMP_23554 to uint8
RETURN TMP_23555
```
#### DateTime.getMinute(uint256) [PUBLIC]
```slithir
timestamp_1(uint256) := phi(['timestamp_1'])
 uint8((timestamp / 60) % 60)
TMP_23556(uint256) = timestamp_1 (c)/ 60
TMP_23557(uint256) = TMP_23556 % 60
TMP_23558 = CONVERT TMP_23557 to uint8
RETURN TMP_23558
```
#### DateTime.getSecond(uint256) [PUBLIC]
```slithir
timestamp_1(uint256) := phi(['timestamp_1'])
 uint8(timestamp % 60)
TMP_23559(uint256) = timestamp_1 % 60
TMP_23560 = CONVERT TMP_23559 to uint8
RETURN TMP_23560
```
#### DateTime.getWeekday(uint256) [PUBLIC]
```slithir
timestamp_1(uint256) := phi(['timestamp_1', 'timestamp_1'])
DAY_IN_SECONDS_7(uint256) := phi(['DAY_IN_SECONDS_6', 'DAY_IN_SECONDS_0', 'DAY_IN_SECONDS_10'])
 uint8((timestamp / DAY_IN_SECONDS + 4) % 7)
TMP_23561(uint256) = timestamp_1 (c)/ DAY_IN_SECONDS_7
TMP_23562(uint256) = TMP_23561 (c)+ 4
TMP_23563(uint256) = TMP_23562 % 7
TMP_23564 = CONVERT TMP_23563 to uint8
RETURN TMP_23564
```
#### DateTime.toTimestamp(uint16,uint8,uint8,uint8,uint8,uint8) [PUBLIC]
```slithir
year_1(uint16) := phi(['year_1', 'year_1', 'year_1'])
month_1(uint8) := phi(['month_1', 'month_1', 'month_1'])
day_1(uint8) := phi(['day_1', 'day_1', 'day_1'])
hour_1(uint8) := phi(['hour_1', 'hour_1'])
minute_1(uint8) := phi(['minute_1'])
DAY_IN_SECONDS_8(uint256) := phi(['DAY_IN_SECONDS_6', 'DAY_IN_SECONDS_0', 'DAY_IN_SECONDS_10'])
YEAR_IN_SECONDS_9(uint256) := phi(['YEAR_IN_SECONDS_0', 'YEAR_IN_SECONDS_7', 'YEAR_IN_SECONDS_4', 'YEAR_IN_SECONDS_10'])
LEAP_YEAR_IN_SECONDS_9(uint256) := phi(['LEAP_YEAR_IN_SECONDS_0', 'LEAP_YEAR_IN_SECONDS_7', 'LEAP_YEAR_IN_SECONDS_10', 'LEAP_YEAR_IN_SECONDS_4'])
HOUR_IN_SECONDS_1(uint256) := phi(['HOUR_IN_SECONDS_0', 'HOUR_IN_SECONDS_3'])
MINUTE_IN_SECONDS_1(uint256) := phi(['MINUTE_IN_SECONDS_3', 'MINUTE_IN_SECONDS_0'])
ORIGIN_YEAR_8(uint16) := phi(['ORIGIN_YEAR_7', 'ORIGIN_YEAR_0', 'ORIGIN_YEAR_4'])
 i = ORIGIN_YEAR
i_1(uint16) := ORIGIN_YEAR_8(uint16)
 i < year
i_2(uint16) := phi(['i_1', 'i_3'])
TMP_23568(bool) = i_2 < year_1
CONDITION TMP_23568
 isLeapYear(i)
TMP_23569(bool) = INTERNAL_CALL, DateTime.isLeapYear(uint16)(i_2)
CONDITION TMP_23569
 timestamp += LEAP_YEAR_IN_SECONDS
timestamp_2(uint256) = timestamp_0 (c)+ LEAP_YEAR_IN_SECONDS_10
 timestamp += YEAR_IN_SECONDS
timestamp_1(uint256) = timestamp_0 (c)+ YEAR_IN_SECONDS_10
timestamp_3(uint256) := phi(['timestamp_1', 'timestamp_2'])
 i ++
TMP_23570(uint16) := i_2(uint16)
i_3(uint16) = i_2 (c)+ 1
 monthDayCounts[0] = 31
REF_9682(uint8) -> monthDayCounts_0[0]
monthDayCounts_1(uint8[12]) := phi(['monthDayCounts_0'])
REF_9682(uint8) (->monthDayCounts_1) := 31(uint256)
 isLeapYear(year)
TMP_23571(bool) = INTERNAL_CALL, DateTime.isLeapYear(uint16)(year_1)
CONDITION TMP_23571
 monthDayCounts[1] = 29
REF_9683(uint8) -> monthDayCounts_1[1]
monthDayCounts_3(uint8[12]) := phi(['monthDayCounts_1'])
REF_9683(uint8) (->monthDayCounts_3) := 29(uint256)
 monthDayCounts[1] = 28
REF_9684(uint8) -> monthDayCounts_1[1]
monthDayCounts_2(uint8[12]) := phi(['monthDayCounts_1'])
REF_9684(uint8) (->monthDayCounts_2) := 28(uint256)
 monthDayCounts[2] = 31
REF_9685(uint8) -> monthDayCounts_3[2]
monthDayCounts_4(uint8[12]) := phi(['monthDayCounts_3'])
REF_9685(uint8) (->monthDayCounts_4) := 31(uint256)
 monthDayCounts[3] = 30
REF_9686(uint8) -> monthDayCounts_4[3]
monthDayCounts_5(uint8[12]) := phi(['monthDayCounts_4'])
REF_9686(uint8) (->monthDayCounts_5) := 30(uint256)
 monthDayCounts[4] = 31
REF_9687(uint8) -> monthDayCounts_5[4]
monthDayCounts_6(uint8[12]) := phi(['monthDayCounts_5'])
REF_9687(uint8) (->monthDayCounts_6) := 31(uint256)
 monthDayCounts[5] = 30
REF_9688(uint8) -> monthDayCounts_6[5]
monthDayCounts_7(uint8[12]) := phi(['monthDayCounts_6'])
REF_9688(uint8) (->monthDayCounts_7) := 30(uint256)
 monthDayCounts[6] = 31
REF_9689(uint8) -> monthDayCounts_7[6]
monthDayCounts_8(uint8[12]) := phi(['monthDayCounts_7'])
REF_9689(uint8) (->monthDayCounts_8) := 31(uint256)
 monthDayCounts[7] = 31
REF_9690(uint8) -> monthDayCounts_8[7]
monthDayCounts_9(uint8[12]) := phi(['monthDayCounts_8'])
REF_9690(uint8) (->monthDayCounts_9) := 31(uint256)
 monthDayCounts[8] = 30
REF_9691(uint8) -> monthDayCounts_9[8]
monthDayCounts_10(uint8[12]) := phi(['monthDayCounts_9'])
REF_9691(uint8) (->monthDayCounts_10) := 30(uint256)
 monthDayCounts[9] = 31
REF_9692(uint8) -> monthDayCounts_10[9]
monthDayCounts_11(uint8[12]) := phi(['monthDayCounts_10'])
REF_9692(uint8) (->monthDayCounts_11) := 31(uint256)
 monthDayCounts[10] = 30
REF_9693(uint8) -> monthDayCounts_11[10]
monthDayCounts_12(uint8[12]) := phi(['monthDayCounts_11'])
REF_9693(uint8) (->monthDayCounts_12) := 30(uint256)
 monthDayCounts[11] = 31
REF_9694(uint8) -> monthDayCounts_12[11]
monthDayCounts_13(uint8[12]) := phi(['monthDayCounts_12'])
REF_9694(uint8) (->monthDayCounts_13) := 31(uint256)
 i = 1
i_4(uint16) := 1(uint256)
 i < month
timestamp_4(uint256) := phi(['timestamp_0', 'timestamp_5'])
i_5(uint16) := phi(['i_4', 'i_6'])
TMP_23572(bool) = i_5 < month_1
CONDITION TMP_23572
 timestamp += DAY_IN_SECONDS * monthDayCounts[i - 1]
TMP_23573(uint16) = i_5 (c)- 1
REF_9695(uint8) -> monthDayCounts_13[TMP_23573]
TMP_23574(uint256) = DAY_IN_SECONDS_10 (c)* REF_9695
timestamp_5(uint256) = timestamp_4 (c)+ TMP_23574
 i ++
TMP_23575(uint16) := i_5(uint16)
i_6(uint16) = i_5 (c)+ 1
 timestamp += DAY_IN_SECONDS * (day - 1)
TMP_23576(uint8) = day_1 (c)- 1
TMP_23577(uint256) = DAY_IN_SECONDS_10 (c)* TMP_23576
timestamp_6(uint256) = timestamp_4 (c)+ TMP_23577
 timestamp += HOUR_IN_SECONDS * (hour)
TMP_23578(uint256) = HOUR_IN_SECONDS_3 (c)* hour_1
timestamp_7(uint256) = timestamp_6 (c)+ TMP_23578
 timestamp += MINUTE_IN_SECONDS * (minute)
TMP_23579(uint256) = MINUTE_IN_SECONDS_3 (c)* minute_1
timestamp_8(uint256) = timestamp_7 (c)+ TMP_23579
 timestamp += second
timestamp_9(uint256) = timestamp_8 (c)+ second_1
 timestamp
RETURN timestamp_9
 timestamp
```
#### DateTime.getWeekNumber(uint256) [PUBLIC]
```slithir
 dt = parseTimestamp(timestamp)
TMP_23580(DateTime._DateTime) = INTERNAL_CALL, DateTime.parseTimestamp(uint256)(timestamp_1)
dt_1(DateTime._DateTime) := TMP_23580(DateTime._DateTime)
 year = dt.year
REF_9696(uint16) -> dt_1.year
year_1(uint16) := REF_9696(uint16)
 month = dt.month
REF_9697(uint8) -> dt_1.month
month_1(uint8) := REF_9697(uint8)
 day = dt.day
REF_9698(uint8) -> dt_1.day
day_1(uint8) := REF_9698(uint8)
 weekday = getWeekday(timestamp)
TMP_23581(uint8) = INTERNAL_CALL, DateTime.getWeekday(uint256)(timestamp_1)
weekday_1(uint8) := TMP_23581(uint8)
 weekday == 0
TMP_23582(bool) = weekday_1 == 0
CONDITION TMP_23582
 weekday = 7
weekday_2(uint8) := 7(uint256)
weekday_3(uint8) := phi(['weekday_2', 'weekday_1'])
 daysSinceYearStart = getDaysSinceYearStart(year,month,day)
TMP_23583(uint256) = INTERNAL_CALL, DateTime.getDaysSinceYearStart(uint16,uint8,uint8)(year_1,month_1,day_1)
daysSinceYearStart_1(uint256) := TMP_23583(uint256)
 weekNumber = (daysSinceYearStart + 7 - weekday) / 7
TMP_23584(uint256) = daysSinceYearStart_1 (c)+ 7
TMP_23585(uint256) = TMP_23584 (c)- weekday_3
TMP_23586(uint256) = TMP_23585 (c)/ 7
weekNumber_1(uint256) := TMP_23586(uint256)
 uint8(weekNumber)
TMP_23587 = CONVERT weekNumber_1 to uint8
RETURN TMP_23587
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
days__2(uint256) := phi(['days__3', 'days__1'])
m_2(uint8) := phi(['m_3', 'm_1'])
TMP_23588(bool) = m_2 < month_1
CONDITION TMP_23588
 days_ += getDaysInMonth(m,year)
TMP_23589(uint8) = INTERNAL_CALL, DateTime.getDaysInMonth(uint8,uint16)(m_2,year_1)
days__3(uint256) = days__2 (c)+ TMP_23589
 m ++
TMP_23590(uint8) := m_2(uint8)
m_3(uint8) = m_2 (c)+ 1
 days_
RETURN days__2
```
#### SpinProxy.slitherConstructorConstantVariables() [INTERNAL]
```slithir
 PROXY_NAME = keccak256(bytes)(SpinProxy)
```
