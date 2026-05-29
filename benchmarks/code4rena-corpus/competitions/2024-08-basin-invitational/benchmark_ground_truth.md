# Benchmark Ground Truth: Basin Invitational

## Accepted H/M Findings

# Accepted H/M Findings: Basin Invitational

# [M-01] Stable2::calcLpTokenSupply() function cannot convert under certain circumstances, DoSing calcReserveAtRatioLiquidity

- **Contest:** Basin Invitational
- **Slug:** 2024-08-basin-invitational
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-basin-invitational
- **Source snapshot:** competitions/2024-08-basin-invitational/final_report.html

Stable2::calcLpTokenSupply() function cannot convert under certain circumstances, DoSing calcReserveAtRatioLiquidity Submitted by Egis_Security calcLpTokenSupply is used in both calcReserveAtRatioLiquidity and calcReserveAtRatioSwap, we’ll focus on calcReserveAtRatioLiquidity.

calcLpTokenSupply is called inside calcRate here:

function calcReserveAtRatioLiquidity ( uint256 [] calldata reserves, uint256 j, uint256 [] calldata ratios, bytes calldata data ) external view returns ( uint256 reserve ) {...

for ( uint256 k; k < 255; k ++) { scaledReserves [ j ] = updateReserve ( pd, scaledReserves [ j ]); // calculate new price from reserves:

pd.

newPrice = calcRate ( scaledReserves, i, j, abi.

encode ( 18, 18 ));...

Inside we call the function:

function calcRate ( uint256 [] memory reserves, uint256 i, uint256 j, bytes memory data ) public view returns ( uint256 rate ) { uint256 [] memory decimals = decodeWellData ( data ); uint256 [] memory scaledReserves = getScaledReserves ( reserves, decimals ); // calc lp token supply (note: `scaledReserves` is scaled up, and does not require bytes).

uint256 lpTokenSupply = calcLpTokenSupply ( scaledReserves, abi.

encode ( 18, 18 )); rate = _calcRate ( scaledReserves, i, j, lpTokenSupply ); } Note that the only change to calcLpTokenSupply is the added revert on the last line of the function.

function calcLpTokenSupply ( uint256 [] memory reserves, bytes memory data ) public view returns ( uint256 lpTokenSupply ) { if ( reserves [ 0 ] == 0 && reserves [ 1 ] == 0 ) return 0; uint256 [] memory decimals = decodeWellData ( data ); // scale reserves to 18 decimals.

uint256 [] memory scaledReserves = getScaledReserves ( reserves, decimals ); uint256 Ann = a * N * N; uint256 sumReserves = scaledReserves [ 0 ] + scaledReserves [ 1 ]; lpTokenSupply = sumReserves; for ( uint256 i = 0; i < 255; i ++) { uint256 dP = lpTokenSupply; // If division by 0, this will be borked: only withdrawal will work. And that is good dP = dP * lpTokenSupply / ( scaledReserves [ 0 ] * N ); dP = dP * lpTokenSupply / ( scaledReserves [ 1 ] * N ); uint256 prevReserves = lpTokenSupply; lpTokenSupply = ( Ann * sumReserves / A_PRECISION + ( dP * N )) * lpTokenSupply / ((( Ann - A_PRECISION ) * lpTokenSupply / A_PRECISION ) + (( N + 1 ) * dP )); // Equality with the precision of 1

if ( lpTokenSupply > prevReserves ) { if ( lpTokenSupply - prevReserves <= 1 ) return lpTokenSupply; } else { if ( prevReserves - lpTokenSupply <= 1 ) return lpTokenSupply; } revert ( "Non convergence: calcLpTokenSupply" ); } The issue here lies that calcLpTokenSupply reverts under certain circumstances since it cannot converge, which makes the whole call to calcReserveAtRatioLiquidity revert. We’ll investigate this further inside the PoC section.

## Recommended Mitigation Steps

It’s very hard to recommend a fix here, as many tweaks can fix the issue. We recommend:

Passing a flag to calcLpTokenSupply, if true then if the function doesn’t converge it won’t revert.

Tweaking the PriceData values for the specific case in the lookup table, narrowing down the range seems to fix the issue.

## Assessed type

DoS Brean0 (Basin) confirmed 0xsomeone (judge) commented:

The Warden has outlined how the system might fail to converge under normal PriceData configurations due to looping between the threshold by a deviancy of ±2. I believe the vulnerability is valid as it causes normal operation of the system to result in a revert due to remediations carried out for a submission in the previous audit of Basin.

To note, a percentage-based deviation threshold might be better appropriate than a fixed unit (i.e., a permitted deviancy of 1 ) to avoid instances whereby the permitted deviation itself is missed by a negligible amount.

## Rejected Primary Findings

# Rejected Primary Findings: Basin Invitational

No rejected primary findings were captured for this contest in the current corpus.

- **Rejected primary count:** 0
- **Primary submission rows captured:** 0
- **Submission status:** requires_authentication
