
## Issue => Proof Steps

Follow these steps, please

1. is the bug IN SCOPE 
## PROJECT SCOPE



-----------------------
if its in scope then...
2. honest assessment: is bug legit? does protocol have safeguards against it? is it realistic for an attacker to exploit?
If yes to above then...

3. is the severity level accurately stated? where severity levels are: Critical | High | Medium | Low | informational ?
when in doubt between 2 severity levels opt for more serious one (say it could be high or medium, please choose high).
this severity scale is for a competitive audit. Refer to the below:

| Severity     | Typical impact examples                                                                                                                                                                                                            | What it signals to the team               |
| ------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------- |
| **Critical** | - Direct theft of any funds - Permanent, **total** loss or control of all user or protocol funds - Arbitrary code execution                                                                                                  | “Drop everything—patch immediately.”      |
| **High**     | - **Permanent freezing** or bricking of user or protocol funds (can’t be reversed without privileged migration) - Loss of governance control - Logic that lets an attacker mint/ burn / drain but under specific constraints | “Must fix before next release / upgrade.” |
| **Medium**   | - Temporary loss (funds stuck until admin action) - Convincing grief / DoS that makes the protocol unusable - Oracle or math bugs that skew accounting but don’t directly drain value                                        | “Important, schedule a patch.”            |
| **Low**      | - Minor economic grief (extra gas, incorrect event data) - Edge-case DoS that requires unusual conditions - Best-practice deviations with limited real-world impact                                                          | “Fix in regular development cycle.”       |
| **Insight**  | Code cleanliness, documentation issues, minor style or test suggestions                                                                                                                                                            | “Nice-to-have, no security impact.”       |


4. ADD test inside of test/PoC.t.sol (there are other tests there, KEEP THOSE and do NOT touch setup(), then use `just <test_name>` to run.  setup() generated everything you need to create test, look at other tests in PoC.t.sol as example.
If PoC tests confirmes issue then...
8. ->
please write up full Poc report following poc-proof/readme.md guidelines 
for this report please do NOT include simple local test. please INCLUDE testnet PoC (PoC.t.sol) 
- full OUTPUT for test 

NOTE:  please remember we are making testnet fork test inside of Poc.t.sol ( do NOT touch setup()  or current test, just make additional test)

------------------------------------------------------------------
## REPORT VERIFICATION

1. honest assessment: does this bug report look correct? 
Does foundry test look right? is it correctly setup to demonstrate bug exists?
Do the foundry test results look right?  Does it prove bug is real?

2. is the severity level accurately stated? where severity levels are: Critical | High | Medium | Low | informational ?
when in doubt between 2 severity levels opt for more serious one (say it could be high or medium, please choose high).
this severity scale is for a competitive audit. Refer to the below:

| Severity     | Typical impact examples                                                                                                                                                                                                            | What it signals to the team               |
| ------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------- |
| **Critical** | - Direct theft of any funds - Permanent, **total** loss or control of all user or protocol funds - Arbitrary code execution                                                                                                  | “Drop everything—patch immediately.”      |
| **High**     | - **Permanent freezing** or bricking of user or protocol funds (can’t be reversed without privileged migration) - Loss of governance control - Logic that lets an attacker mint/ burn / drain but under specific constraints | “Must fix before next release / upgrade.” |
| **Medium**   | - Temporary loss (funds stuck until admin action) - Convincing grief / DoS that makes the protocol unusable - Oracle or math bugs that skew accounting but don’t directly drain value                                        | “Important, schedule a patch.”            |
| **Low**      | - Minor economic grief (extra gas, incorrect event data) - Edge-case DoS that requires unusual conditions - Best-practice deviations with limited real-world impact                                                          | “Fix in regular development cycle.”       |
| **Insight**  | Code cleanliness, documentation issues, minor style or test suggestions                                                                                                                                                            | “Nice-to-have, no security impact.”       |
-----------------------------------------------------------------

3. what would this security bug (detailed in below report)  result in if exploited? list all that apply: 

- Direct theft of any user funds, whether at-rest or in-motion, other than unclaimed yield
- Permanent freezing of funds
- Protocol insolvency
- Theft of unclaimed yield
- Temporary freezing of funds for at least 24 hours
- Theft of gas
- Smart contract unable to operate due to lack of token funds
- Contract fails to deliver promised returns, but doesn't lose value
- Unbounded gas consumption
- Temporary freezing of funds for at least 1 hour

## REPORT CODE VERIFICATION

Please review the security bug report below.

## REPORT


## END OF REPORT

Does this security bug report contain the below test contract?
Are they identical?

## TEST CONTRACT 

