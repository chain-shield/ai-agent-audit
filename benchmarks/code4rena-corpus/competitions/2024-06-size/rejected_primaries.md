# Rejected Primary Findings: Size

# Liquidated user is favored instead of liquidator when calculating the debt positions assigned collateral

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-190
- **Submitter:** 0xAlix2
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/190
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-190.md

## Brief Summary

Upon liquidation, the protocol calculates the assigned collateral amount for the debt position that is being liquidated, this is done in `LoanLibrary::getDebtPositionAssignedCollateral`. That amount is being used to calculate the rewards of the liquidator and the protocol. This is being calculated by the following: Which is rounding down the amount. The protocol here favors the liquidated user instead of the liquidator and the protocol. This affects the resulting liquidator and protocol liquidation rewards.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_230_group

# `isDebtPositionLiquidatable` doesn't account for fees

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-630
- **Submitter:** 0xMilenov
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/630
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-630.md

## Brief Summary

The current implementation of the `isDebtPositionLiquidatable` function in the `Size` protocol does not account for fees when determining if a debt position is liquidatable. This oversight could lead to scenarios where positions that should be liquidated (due to accumulated fees making the borrower effectively underwater) are not liquidated, resulting in potential financial losses for the protocol and its participants. Description The function `isDebtPositionLiquidatable` is responsible for checking if a debt position can be liquidated based on its status and whether the user is underwater. However, the function does not take into account any fees that might have accumulated on the debt pos...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_207_group

# Rounding on profitable liquidations beneffits the Liquidator at the expense of the Borrower and the Protocol

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-341
- **Submitter:** 0xStalin
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/341
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-341.md

## Brief Summary

The liquidator is benefitted at the expense of the Borrower and the Protocol because the `liquidatorReward` is roundedUp, and the `protocolProfitCollateralToken` is rounded down.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_47_group

# Incorrect validation to determine if the liquidator receives the requested minimum profit for executing the validation.

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-344
- **Submitter:** 0xStalin
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/344
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-344.md

## Brief Summary

Liquidators could receive less than the expected minimum profit, or even worse, liquidate at a loss when they would not be willing to do it.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_26_group

# Multicall Failure Due to Concurrent Collateral and Borrow Token Deposits in `Size::deposit()

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-307
- **Submitter:** 0xarno
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/307
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-307.md

## Brief Summary

The issue in the `Multicall` functionality within the `Size` contract, specifically impacting concurrent token deposits of collateral and borrow tokens. When using the multicall feature, discrepancies arise between `msg.value` and `params.token` validations. For instance, during a multicall transaction, if the first call deposits ETH as collateral and the subsequent call deposits a borrow token, `msg.value` remains same while `params.token` changes, leading to validation failures. Impact The issue disrupts the functionality of multicall transactions within the `Size` contract. Attempting to deposit collateral tokens and borrow tokens concurrently. This inconsistency will result in failed mu...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Inability to Update Aave Variable Pool Address in Configuration Update Function

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-515
- **Submitter:** 0xarno
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/515
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-515.md

## Brief Summary

The contract initializes the Aave variable pool address during its setup phase and subsequently does not offer an option to update this address. According to Aave’s [documentation](https://docs.aave.com/developers/core-contracts/pooladdressesprovider), the pool address from their `PoolAddressesProvider` should not be hardcoded because it is subject to change [proof](https://github.com/aave/aave-v3-core/blob/6070e82d962d9b12835c88e68210d0e63f08d035/contracts/protocol/configuration/PoolAddressesProvider.sol#L57). Instead, it is recommended to query the `PoolAddressesProvider` every time the pool address is needed to ensure the address used is current. This configuration is a critical design o...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_74_group

# Users Can Steal ETH Balance with Minimum Investment

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-590
- **Submitter:** 0xarno
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/590
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-590.md

## Brief Summary

The `executeDeposit` function in the contract contains a vulnerability where a user can deposit a minimal amount of ETH (e.g., 1 wei) to trigger the conversion of the entire contract's ETH balance to WETH. This occurs because the function uses the contract's total ETH balance (`address(this).balance`) instead of the `msg.value` provided in the transaction, allowing the user to deposit significantly more than they actually send. Since contract balance is used to prevent `msg.value` in loop vulnerability while `multicall` is used to deposit, this opens up a way for a user to steal contract ETH balance by depositing the minimum amount. Impact This vulnerability allows users to drain the contra...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_02_group

# Potential APR Manipulation and Variability Issues in `BuyCreditMarket` function

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-617
- **Submitter:** 0xarno
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/617
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-617.md

## Brief Summary

The `BuyCreditMarket` function enables lenders to lend with either a fixed APR or a dynamic APR that adjusts based on market conditions and borrower-specified multipliers. In dynamic APR setups, borrowers can opt for potentially lower rates by specifying a negative APR during the yield curve construction, which inversely adjusts the APR based on the market rate. The APR is adjusted according to the formula `apr + marketRateMultiplier * marketRate`. This mechanism is designed to reflect current market conditions when a borrower sells credit in the market. However, this flexibility also introduces significant risks due to the absence of a maximum APR cap and the possibility for lenders to man...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_164_group

# Inflexibility in Adjusting `crLiquidation` Value in Smart Contract Settings

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-693
- **Submitter:** 0xarno
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/693
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-693.md

## Brief Summary

The `UpdateConfig.sol` contract for managing protocol configurations exhibits a significant limitation in the adjustment of the `crLiquidation` (Collateral Ratio for Liquidation) parameter. As per the current implementation, once the `crLiquidation` value is set, it can only be decreased and not increased. This restriction is enforced by a validation check that reverts any transaction where the new `crLiquidation` value is greater than the existing one. While this might initially help in maintaining a conservative approach to risk management by not allowing abrupt increases in the liquidation threshold, it also restricts the protocol's ability to respond adaptively to favorable market condi...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Violation of SOLVENCY_02 Invariant in createDebtAndCreditPositions() Function

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-401
- **Submitter:** 0xbrett8571
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/401
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-401.md

## Brief Summary

The [createDebtAndCreditPositions()](https://github.com/code-423n4/2024-06-size/blob/8850e25fb088898e9cf86f9be1c401ad155bea86/src/libraries/AccountingLibrary.sol#L62-L92) function in the `AccountingLibrary.sol` violates the `SOLVENCY_02: SUM(credit) <= SUM(debt)` invariant. This function is responsible for creating both debt and credit positions simultaneously, which is a critical operation in the protocol. Impact The fact that the credit position is created with `credit: debtPosition.futureValue`. This means that the credit amount is exactly equal to the debt amount. While this doesn't directly violate `SOLVENCY_02`, it creates a situation where any subsequent fee application or credit red...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# `liquidatorProfitBorrowToken` send to wrong address

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-569
- **Submitter:** 0xhacksmithh
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/569
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-569.md

## Brief Summary

Liquidator `KEEPER_ROLE` will lost profit as Profit send to wrong address

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Insufficient access control on Default admin which can compromise the protocol

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-717
- **Submitter:** 0xpetern
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/717
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-717.md

## Brief Summary

The protection mechanism for the default admin role is insufficient. As seen in line 101 in size.sol, "_grantRole(DEFAULT_ADMIN_ROLE, owner);". When you go to the initailize.sol where the validation for owner takes place, it only checks that the address passed is not a zero address. It goes against openzeppelin recoomendation which says in their code base "WARNING: The `DEFAULT_ADMIN_ROLE` is also its own admin: it has permission to grant and revoke this role. Extra precautions should be taken to secure accounts that have been granted it. We recommend using {AccessControlDefaultAdminRules} to enforce additional security measures for this role.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Users can not compensate with `creditPositionToCompensateId = RESERVED_ID` at the end of the loan

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-439
- **Submitter:** 3n0ch
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/439
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-439.md

## Brief Summary

When users compensate with `creditPositionToCompensateId = RESERVED_ID`, the contract makes a call to `createDebtAndCreditPositions` In this function, there is a check for the `tenor` Because of this check, the function will revert when then `tenor` less than `state.riskConfig.minTenor`. As a result, users can not compensate with `creditPositionToCompensateId = RESERVED_ID` at the end of the loan, which also breaks the DOS invariant > DOS: Functions should not revert if preconditions are met (Denial of Service)

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# This function is marked as an initializer but is also declared as public

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-3
- **Submitter:** Ali55
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/3
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-3.md

## Brief Summary

After analyzing the provided Solidity code, I found a potential vulnerability related to the `initialize` function. This function is marked as an `initializer` but is also declared as public, which could potentially allow an attacker to repeatedly call this function and initialize the contract with different parameters. This could lead to unintended behavior and potential security issues. To mitigate this vulnerability, I recommend changing the `initialize` function's visibility to `internal` or adding a modifier to restrict its execution to a single call. Here's an example of how you can modify the function: This modification ensures that the `initialize` function can only be called once,...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_33_group

# gasPrice

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-6
- **Submitter:** Ali55
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/6
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-6.md

## Brief Summary

The smart contract's `withdraw` function allows users to withdraw Ether from the contract. However, the current implementation does not enforce a minimum withdrawal amount. This could lead to potential issues when handling small gas costs for transactions. If a user sends a transaction with a `gasPrice` lower than the gas cost of the `withdraw` function, the contract will still process the transaction, but the user could end up with a debt instead of withdrawing the expected amount. This debt will accumulate in the user's account and could potentially lead to unintended behavior or security breaches.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_192_group

# APR Calculation is Favoring the Borrower Instead of the Lender

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-532
- **Submitter:** BaldHeads
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/532
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-532.md

## Brief Summary

The loan APR affects how much the borrower is expected to pay to the lender. In the following examples, the APR is calculated using the `mulDivDown` function. This favors the borrower instead of the lender. [getAPR](https://github.com/code-423n4/2024-06-size/blob/8850e25fb088898e9cf86f9be1c401ad155bea86/src/libraries/YieldCurveLibrary.sol#L115-L143) will return this value if `y0 > y1`: * `y0 + Math.mulDivDown(y1 - y0, tenor - x0, x1 - x0);` However, as this is an APR calculation, a lower APR will be beneficial for the borrower.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Loan and Borrow Offers Are Not Being Reset After the User Deposit Has Been Used, Leaving Stale Orders on the Order Book.

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-535
- **Submitter:** BaldHeads
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/535
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-535.md

## Brief Summary

Once a legitimate user adds a loan or borrow offer to the order book, other users can use the market order functions to fulfill that order. Once a user's deposit has been consumed to fulfill other orders, the offer still remains in the order book, creating stale orders which cannot be fulfilled.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_105_group

# `Deposit#executeDeposit()` expects msg.value but is not marked as payable

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-239
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/239
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-239.md

## Brief Summary

Core functionality is broken and as such direct ether deposits which are to be accepted would never work, since [this block](https://github.com/code-423n4/2024-06-size/blob/8850e25fb088898e9cf86f9be1c401ad155bea86/src/libraries/actions/Deposit.sol#L66-L73) never gets triggered.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_86_group

# Chainlink addresses/intervals should not be immutable

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-240
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/240
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-240.md

## Brief Summary

As hinted under _Proof of Concept_ this could lead to multiple issues. - Ingestion of stale if the heartbeat of the feed gets changed to a lower one (How stale the price would be depends on the change). - Consistent partial DOS if the heartbeat of the feed gets changed to a higher one (Duration of DOS periodically depends on the change). - And even worse a permanent DOS if any of the underlying feeds get changed. Would be key to note that this DOS aren't just applicable to the pricing functionalities, but would also affect all other functionalities that directly query this, i.e liquidations et all, for liquidations, this would mean thats bad position would be able to accumulate more debt, c...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_116_group

# Grace period time is too much for after the sequencer comes back up and allows for accumulation of bad debt

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-241
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/241
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-241.md

## Brief Summary

Protocol would be easily put in an unwanted state since it allows for bad debt positions to be accumulating for a long duration even after the sequencer comes up (and these positions could be more than one in this case, exarcebating the issue). > NB: Even if transactions are passed via the delayed inbox, the sequencer only includes the message from the inbox after a delay of ~ 10 minutes when it comes back up which is so as to ensure that the transaction/message’s arrival will not be affected by a reorganisation of the base layer chain... which would mean that the current implementation of 1 hour just puts protocol in a position of heavy risk, where 10/15 minutes would suffice.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_64_group

# Users can front/backrun the updates to the variable rates and still have a relatively lower position for their borrows on the yield curve

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-242
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/242
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-242.md

## Brief Summary

A user can still time their entries to have a lower position in the yield curve all they need to do is front/back run the attempt of the owner of the `BORROW_RATE_UPDATER_ROLE` to update the rates

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# No slippage checks when withdrawing tokens from the variable pool

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-243
- **Submitter:** Bauchibred
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/243
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-243.md

## Brief Summary

When withdrawing no slippage is applied which could lead to loss of user funds.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_25_group

# Impact of Loan Repayment Front-Running on Protocol Fees and Incentives (Overdue charges are not deduct when an overdue loan is fine)

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-300
- **Submitter:** Bigsam
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/300
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-300.md

## Brief Summary

A 1% fee removal from the user with an overdue loan should be enforced in repayment function, the protocol believes that once a loan is liquidatable it would be liquidated but the time during between liquidation and execution a user can repay and since the Overdue fee is not enforced in the repayment logic thus the protocol loses revenue. A user has all the time in the world to repay his loan not doing so on time attracts a fee BASED ON THE PROTOCOL IMPLEMENTATION AND DOCS if his/her position is not underwater and it is overdue. The current loan repayment logic allows users to bypass the overdue fee by front-running the liquidation process. This issue arises because the protocol does not en...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_178_group

# A Lenders address can be matched against his borrow offer in the Liquidated with Replacement Function thereby leading to loss of funds by the lender.

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-583
- **Submitter:** Bigsam
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/583
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-583.md

## Brief Summary

The `liquidateWithReplacement` function fails to check if the new borrower selected is the same as the lender who holds the debt position being liquidated. This oversight can lead to a scenario where a lender incurs a loss on their position if they end up being the new borrower for their own position, despite lenders being allowed to hold collateral tokens and borrow tokens like any other user. > A lender is allowed to hold collateral tokens and borrowAtookens like every other user. A lender is allowed to create a borrow offer where he can put up his future value up for sale and get the present worth of the loan. The issue arises when a lender’s position is not yet due but the borrower’s Co...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_20_group

# The `YEAR` is constant in `Math` contract and its value is 365 days which is not always correct

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-549
- **Submitter:** Bube
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/549
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-549.md

## Brief Summary

The constant `Math::Year` is used in the calculation of swap fee percent in the `AccountingLibrary::getSwapFeePercent` function. Also, the `getSwapFeePercent` function is called in the `getSwapFee` function to calculate the swap fee. This swap fee is used in different functions of the protocol (some of them: `SizeView::getSwapFee`, `AccountingLibrary::getCreditAmountOut`, `AccountingLibrary::getCashAmountIn`, `AccountingLibrary::getCashAmountOut`). The problem is that the `YEAR` is set to be `365 days`. But that is not correct for the leap years that have `366 days`. The protocol may lose fees due to the incorrect duration of the year when the year is leap, leading to inaccuracies in intere...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# The input parameter `_data` in `Size::multicall` is not checked and can be malicious

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-614
- **Submitter:** Bube
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/614
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-614.md

## Brief Summary

The `Size::multicall` function allows multiple function calls to be executed in a single transaction. While this provides efficiency and convenience, it also introduces potential security risks if the `_data` passed to multicall is malicious. Malicious data could exploit reentrancy vulnerabilities within the protocol, unauthorized access to contract funds, state manipulation, and other unintended behaviors. Also, malicious data could bypass input validation checks within individual functions, leading to execution of unintended or harmful operations for the protocol. Malicious data could cause `multicall` to exceed gas limits, leading to incomplete transactions or denial of service for the p...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_36_group

# Anyone can call `DepositTokenLibrary::withdrawUnderlyingCollateralToken` and `DepositTokenLibrary::withdrawUnderlyingTokenFromVariablePool` functions with arbitrary input parameters

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-618
- **Submitter:** Bube
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/618
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-618.md

## Brief Summary

The `DepositTokenLibrary::withdrawUnderlyingCollateralToken` and `DepositTokenLibrary::withdrawUnderlyingTokenFromVariablePool` functions are external and anyone can call them with arbitrary input arguments. That means a malicious user can set arbitrary address for `from` parameter and his/her address for `to` address. In that way a malicious user can withdraw funds that are not intended to withdraw. The `Withdraw::executeWithdraw` function calls these two functions but the `from` address is set to `msg.sender`. The problem in the `DepositTokenLibrary` is that the functions are defined as external and anyone can call it.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Missing a gap as the buffer zone between crOpening and crLiquidation

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-165
- **Submitter:** Chad0
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/165
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-165.md

## Brief Summary

The protocol does not implement a proper validation to ensure there should be a gap between the `crOpening` at which borrowers are allowed to open a loan and the `crLiquidation` at which liquidation can be executed. This means that the protocol will accept the two values passed in even if there is no gap between them. In the worst scenario it allows the user to borrow at the very edge of liquidation and be liquidated with a slight price change. This is a super risky setup for regular protocol users. This is a typical issue of inadequate safety parameters. The top tier lending protocols have widely adopted the standard mitigation of introducing a gap as the buffer zone between the two parame...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# The `crLiquidation` can only be updated downwards but not upwards which is a bad design

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-166
- **Submitter:** Chad0
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/166
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-166.md

## Brief Summary

The protocol only allows the `crLiquidation` to be updated downwards but not upwards, basically making it a one-way door with no turning back. I understand the sponsor's idea behind such a design is to avoid the risk of a sudden upwards change on the `crLiquidation` may cause many users facing immediate liquidation. However, this one-way door design is by no means an adequate design, and it will backfire someday when the market condition has changed and the protocol needs to change the `crLiquidation` upwards but do not have the flexibility to do so. When this situation happens, the worst impact is that this product has to be deprecated. Actually, there is a feasible design to make the `crL...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# YieldCurve accepts negative apr inputs, which can lead to sell/buyCreditLimit orders being able to get listed but possibly never gonna get fulfilled

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-230
- **Submitter:** Chad0
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/230
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-230.md

## Brief Summary

First and foremost, the `YieldCurve` accepting `int256[]` for `aprs` is directly contradicting the code in the interface of `ISize` like [here](https://github.com/code-423n4/2024-06-size/blob/8850e25fb088898e9cf86f9be1c401ad155bea86/src/interfaces/ISize.sol#L66) and [here](https://github.com/code-423n4/2024-06-size/blob/8850e25fb088898e9cf86f9be1c401ad155bea86/src/interfaces/ISize.sol#L74). If accepting `int256[]`, that means some part of the yield curve can have negative values, which does not make sense in the lending business at all. Not any part of the `YieldCurve` should be negative no matter what the associated `tenor` is. That is counter-intuitive. Moreover, if a user used some negat...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Malicious actor can enforce liquidation to a user

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-729
- **Submitter:** Evo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/729
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-729.md

## Brief Summary

Malicious actor can enforce liquidation to a user

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_05_group

# Reentrancy Attack is possible.

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-362
- **Submitter:** Filip
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/362
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-362.md

## Brief Summary

There is a classic vulnerability where `Reentrancy attack` can occur in the `withdrawUnderlyingTokenFromVariablePool` function. Please refer to the code below. function withdrawUnderlyingTokenFromVariablePool(State storage state, address from, address to, uint256 amount) external { IAToken aToken = IAToken(state.data.variablePool.getReserveData(address(state.data.underlyingBorrowToken)).aTokenAddress); uint256 scaledBalanceBefore = aToken.scaledBalanceOf(address(this)); // slither-disable-next-line unused-return >>> state.data.variablePool.withdraw(address(state.data.underlyingBorrowToken), amount, to); >>> uint256 scaledAmount = scaledBalanceBefore - aToken.scaledBalanceOf(address(this));...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_160_group

# When both `low` and `high` are large enough value, overflow will occur

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-536
- **Submitter:** Filip
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/536
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-536.md

## Brief Summary

`binarySearch` function can occur overflow error. It seems to be the right if it is common to calculate the `mid` value of `low` and `high`. However, if both `low` and `high` are large enough (close to Max values), overflow will occur in `low + high` calculations.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Debt Token Repayment Mechanism Allows Users to Exploit USDC Depreciation

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-656
- **Submitter:** Fortis_audits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/656
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-656.md

## Brief Summary

When the value of USDC depreciates, users can repay their debt at a reduced cost in USD, while still holding debt tokens that were minted at the original, higher value of USDC. This discrepancy can result in significant losses for the platform and undermine the stability of the borrowing and lending system. Impact: Users who have borrowed USDC can take advantage of a market depreciation in its value to repay their debts at a reduced cost in USD, effectively underpaying their debt. This discrepancy arises because the debt tokens are minted based on the initial value of USDC at the time of borrowing, but repayment is based on the current, depreciated value of USDC. As a result, the platform c...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# An attacker can force a sellCreditMarket/buyCreditMarket call to match with a non-optimal offer by front-running the victim

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-481
- **Submitter:** Infect3d
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/481
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-481.md

## Brief Summary

A malicious lender that has put a limit-buy offer in the orderbook can wait for a borrower to match its offer with a market-sell, and front-run him by swapping its intial offer with one that matches the borrower's `maxAPR`. The issue with this is that the borrower will not have the optimal offer available on the orderbook, and other lenders that had limit-buy orders with better APR than the malicious lender will not have their order matched while they should have been. See

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_137_group

# Size can overestimate liquidity on borrowing and lending actions after Aave upgrades to v3.1

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-197
- **Submitter:** JCN
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/197
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-197.md

## Brief Summary

Size can overestimate liquidity on borrowing and lending actions after Aave upgrades to v3.1

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_89_group

# The `collateralRemainderCap` is not the correct cap

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-155
- **Submitter:** Jorgect
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/155
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-155.md

## Brief Summary

The `collateralRemainder` of a liquidation taking already the liquidator profit in consideration is been capped (see `collateralRemainderCap` arrow below) to let some collateral to the borrower. that can be see it in the `executeLiquidate` function: [[Link]](https://github.com/code-423n4/2024-06-size/blob/8850e25fb088898e9cf86f9be1c401ad155bea86/src/libraries/actions/Liquidate.sol#L107) The problem is that this `collateralRemainderCap` is been capped with the wrong variable, the line is capping `debtInCollateralToken` insted of `collateralRemainder `. Impact The most clearly impact is that the protocolProfitCollateralToken is returning the wrong value. this is bad for the user and for the p...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Ability to change the apr of the curve any time allow lenders and borrower to front run the actions.

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-369
- **Submitter:** Jorgect
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/369
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-369.md

## Brief Summary

In each action `buyCreditMarket`, `sellCreditMarket`, `liquidateWithReplacement`, there is a input parameter that protect lender or borrower (depend of the function) of "slippage" this value is the `minAPR`, in other function `maxAPR`. The problem is that all this functions can be front run sending a transaction modification the offer with a apr that make revert the transaction. Impact Let's be more specific here, in `buyCreditMarket` a lender can be trying to open a position, and the borrower can front run the tx changing the apr making revert the transaction. in `sellCreditMarket` a borrower can be trying to open a position, and the lender can front run the tx changing the apr making reve...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# validateSelfLiquidate Uses Borrower's Overall Collateral Ratio

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-171
- **Submitter:** LinKenji
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/171
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-171.md

## Brief Summary

* **Intended Behavior:** Self-liquidation should only be allowed when the debt value (regarding collateral token) exceeds the total collateral assigned to the specific credit position being self-liquidated. * **Code Behavior:** The code checks if the borrower is underwater (`state.collateralRatio(debtPosition.borrower)`), meaning it considers the borrower's overall collateralization, not just the collateral tied to the specific credit position. **Why this is a bug:** * **Profitable Self-Liquidation:** If a borrower has multiple credit positions, a lender could self-liquidate a credit position even if that specific position is overcollateralized. This could happen if the borrower's other pos...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_38_group

# executeSelfLiquidate Function: Pass assignedCollateral as Argument to transferFrom

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-174
- **Submitter:** LinKenji
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/174
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-174.md

## Brief Summary

The `assignedCollateral` variable is not being used correctly. It is being assigned to the `state.data.collateralToken.transferFrom()`, but it should be passed as an argument instead.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Certain configurations of liquidationRewardPercent could disincentivize liquidators

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-528
- **Submitter:** MidgarAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/528
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-528.md

## Brief Summary

Liquidators are at the core of the health of the system because they guard it from bad debt. However, if `liquidationRewardPercent` is misconfigured, liquidator could be disincentivized to liquidate. `liquidationRewardPercent` must be set at a level that incentivizes liquidators to liquidate positions below the `crLiquidation` threshold. This level can be adjusted by the admin according to current market risks. The lower this level, the less motivated the liquidators will be to liquidate. `liquidatorReward` is calculated in `executeLiquidate()` function: Liquidators are supposed to get up to `liquidationRewardPercent` as a reward. However, liquidators will never receive this level of reward...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# getDebtPositionAssignedCollateral Reverts on Zero Debt with Non-Zero Collateral.

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-205
- **Submitter:** Myd
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/205
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-205.md

## Brief Summary

[LoanLibrary.sol#156-160](https://github.com/code-423n4/2024-06-size/blob/8850e25fb088898e9cf86f9be1c401ad155bea86/src/libraries/LoanLibrary.sol#L156-L160) If the `debt` variable is zero, the code correctly handles the case by returning zero as the assigned collateral. However, if `debt` is zero and the code execution reaches the `Math.mulDivDown` function, it will attempt to perform a division by zero, which can cause the transaction to revert. Impact The `getDebtPositionAssignedCollateral` function calculates the amount of collateral assigned to a specific debt position. It does this by proportionally distributing the borrower's total collateral based on the debt position's future value a...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unsuccessful Aave Withdrawal Leads to BorrowAToken Burn

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-207
- **Submitter:** Myd
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/207
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-207.md

## Brief Summary

`withdrawUnderlyingTokenFromVariablePool` calculates the `scaledAmount` by subtracting the `scaledBalanceBefore` from the current `scaledBalanceOf(address(this))`. However, if the withdrawal from the Aave pool fails for any reason (e.g., insufficient liquidity, Aave pool being paused, etc.), the `scaledAmount` will still be calculated and the corresponding `burnScaled` operation will be performed on the `borrowAToken`. This means that even if the withdrawal from Aave fails, the user's `borrowAToken` balance will still be burned, leading to a loss of funds for the user. The contract should ensure that the burnScaled operation is only performed if the withdrawal from Aave is successful. Impact

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_42_group

# Lender Mismatch When Creating New Credit Positions

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-210
- **Submitter:** Myd
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/210
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-210.md

## Brief Summary

When `params.creditPositionId` is equal to RESERVED_ID, indicating that a new credit position should be created, the function calls `state.createDebtAndCreditPositions` with both the lender and borrower set to `msg.sender.` This means the lender and borrower of the newly created debt and credit positions will be at the same address. However, when selling credit as a market order, the lender should be `params.lender`, not `msg.sender`. The current implementation incorrectly sets the lender to the function caller (`msg.sender`) instead of the lender specified in the function parameters. [SellCreditMarket.sol#184-191](https://github.com/code-423n4/2024-06-size/blob/8850e25fb088898e9cf86f9be1c4...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_174_group

# Mistakenly sent eth could be locked

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-194
- **Submitter:** Naresh
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/194
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-194.md

## Brief Summary

If ERC20 tokens and ETH are transferred at the same time, the mistakenly sent ETH will be locked. Several functions could be affected and cause user funds to be locked: - `buyCreditMarket()` - `sellCreditMarket()` - `repay()` - `claim()` - `liquidate()` - `selfLiquidate()` - `liquidateWithReplacement()` - `compensate()`

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unchecked Maximum Slippage in Liquidation Exposes Protocol to Market Manipulation and Disproportionate Gains for Whales

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-668
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/668
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-668.md

## Brief Summary

The lack of a global maximum profit percentage in the liquidation mechanism exposes the protocol to potential market manipulation and unfair advantages for wealthy participants. This vulnerability could lead to excessive profits during liquidations, destabilizing the system. The current implementation in Liquidate.sol only enforces a minimum profit for liquidators. Code: The executeLiquidate function calculates the liquidator's profit without an upper bound: This setup allows a malicious actor with significant resources to manipulate market prices briefly, triggering liquidations at artificially favorable rates. They could then profit excessively when prices return to normal, potentially dr...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Insufficient Handling of Deviation Threshold for ETH/USD Price Feed on BASE L2

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-674
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/674
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-674.md

## Brief Summary

The PriceFeed smart contract, when deployed on BASE L2, does not account for the different deviation threshold of the ETH/USD price feed compared to Ethereum Mainnet. This discrepancy can lead to inaccurate price reporting and potential economic vulnerabilities in the system. Details: **Current Implementation:** - File: PriceFeed.sol - The contract uses Chainlink's ETH/USD price feed without any specific deviation threshold checks. - The contract assumes a uniform behavior of price feeds across different networks. **Network-Specific Behavior:** - Ethereum Mainnet ETH/USD feed deviation threshold: 0.5% (0.005 in decimal form) - BASE L2 ETH/USD feed deviation threshold: 0.15% (0.0015 in decim...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect APR Calculation for Exact Tenor Matches in Yield Curve Interpolation

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-676
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/676
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-676.md

## Brief Summary

A bug has been identified in the getAPR function of the YieldCurveLibrary contract. The function incorrectly handles cases where the input tenor exactly matches a value in the yield curve's tenor array. This can lead to inaccurate APR calculations for these specific tenors. Details: The getAPR function uses a binary search algorithm to locate the position of the input tenor within the yield curve's tenor array. When an exact match is found, the function should return the APR specifically defined for that tenor. However, the current implementation fails to do so. The problematic code segment is as follows: When low == high (indicating an exact match), the function returns y0, which is calcul...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_134_group

# Precision Loss in Scaled Token Burns Due to Incorrect Rounding

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-680
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/680
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-680.md

## Brief Summary

In the NonTransferrableScaledToken contract, the transferFrom function uses Math.mulDivDown when calculating the scaledAmount for token burns. This downward rounding can lead to precision loss and potential undercollateralization of the protocol over time. Code: Impact: The use of mulDivDown for calculating scaledAmount can result in burning slightly fewer tokens than necessary. While the discrepancy is minimal per transaction due to RAY's high precision (1e27), the cumulative effect across multiple transactions and users may lead to: 1. Gradual protocol undercollateralization 2. Unfair advantage to active users over passive ones 3. Potential system instability in high-volume scenarios 4. I...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Reorg Vulnerability in Contract Initialization Process

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-682
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/682
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-682.md

## Brief Summary

The contract initialization process implemented in the `executeInitializeData` function of the Initialize library is vulnerable to blockchain reorganization events. This vulnerability could lead to inconsistent contract states, potentially compromising the entire protocol's functionality and security. Code: The vulnerability is present in the `executeInitializeData` function: Vulnerability Details: - The function uses the CREATE opcode (via Solidity's `new` keyword) to deploy three new contracts. - The addresses of these contracts are determined by the deployer's address and nonce at the time of deployment. - In a blockchain reorg, if this transaction is re-executed in a different block or...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of slippage control in market operations

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-683
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/683
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-683.md

## Brief Summary

Lack of slippage control in some market operations could lead to unexpected results in volatile market conditions. The issue here is that while these functions do check for a minimum APR in `validateBuyCreditMarket` or maximum APR in `validateSellCreditMarket`, they don't include a mechanism for the user to specify a minimum amount out or maximum amount in. This lack of slippage control could lead to some problems. The issue primarily affects the `BuyCreditMarket` and `SellCreditMarket` functions. Code In BuyCreditMarket.sol: Similarly, in SellCreditMarket.sol: Impact: 1. Front-running: In volatile market conditions, an attacker could front-run a transaction, causing the market rate to chan...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_35_group

# Lack of Loan Status Check in `createCreditPosition` Function Allows Modification of Non-Active Loans

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-184
- **Submitter:** NoOne
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/184
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-184.md

## Brief Summary

The `createCreditPosition` function in the smart contract does not explicitly verify that the loan associated with a credit position is in the `ACTIVE` status before allowing an exit process. This could lead to unintended behavior where credit positions linked to `OVERDUE` or `REPAID` loans are modified, potentially violating the contract’s intended logic and leading to financial discrepancies.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lenders are not getting interest from AAVE when claiming

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-541
- **Submitter:** Nyx
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/541
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-541.md

## Brief Summary

When a debt position is repaid, lenders can claim their borrow tokens with interest using the claim function. The claimAmount is calculated as follows. (From docs : https://docs.size.credit/technical-docs/contracts/3.6-claim) It uses a token's current liquidity index and the liquidity index when repayment has been made. The problem is that if a lender wants to claim his tokens after repayment, the current liquidity index and the liquidityIndexAtRepayment will be the same, and thus, the lender won't get any interest. If the protocol wants to give interest that comes from AAVE to lenders, it should at least use position creation time instead of the liquidityIndexAtRepayment to give interest t...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Credit positions can be buyable instantly, which can cause lenders to lose funds

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-579
- **Submitter:** Nyx
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/579
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-579.md

## Brief Summary

When a borrower borrows with a market order using the sellCreditMarket function, a debt position is created for the borrower and a credit position is created for the lender. And the credit position is created with the forSale being true. This may cause problems in some conditions. Users can be both lenders and borrowers simultaneously. However, a problem arises when a credit position is opened with "forSale" set to true. Anyone can purchase the lender's credit position, causing the lender to lose funds. When a credit position is bought, fees are taken from the borrower, who in this case is the original lender. As a result, the lender can lose funds instantly, equal to fees.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_99_group

# limit order might be filled with undesired apr

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-526
- **Submitter:** OMEN
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/526
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-526.md

## Brief Summary

limit order would be executed at unintended time with undesired yield curve .

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# ETH collateral depositers will lose funds in the depegged event of weth

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-529
- **Submitter:** OMEN
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/529
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-529.md

## Brief Summary

Borrowers who deposit ETH as collateral will lose funds in the depegged event of WETH .

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# A malicious borrower user can cause loss of funds for lender

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-446
- **Submitter:** PASCAL
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/446
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-446.md

## Brief Summary

Lenders are being charged fees when borrowers fill up a lending order, thus malicious borrowers can spam borrow transactions to lenders causing fees being charged multple times from the lenders when they're basically not doing anything thus lenders incurring losses.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Incorrect fee calculation in `getCashAmountOut()` results in overcharging users

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-193
- **Submitter:** Rhaydden
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/193
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-193.md

## Brief Summary

The `getCashAmountOut()` function calculates fees based on the maximum cash amount before subtracting those fees, resulting in users paying fees on money they don't actually receive. This leads to unfair fee calculations and potential loss of funds for users.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Unhandled Chainlink latestRoundData() revert can lock price oracle access in PriceFeed.sol

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-256
- **Submitter:** Rhaydden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/256
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-256.md

## Brief Summary

If the Chainlink multisig blocks access (which they can at will) to the price feed, causing the `latestRoundData()` function to revert, the `PriceFeed` contract will also revert. Therefore, to prevent DOS scenarios, it is recommended to query Chainlink price feeds using a defensive approach with Solidity’s try/catch structure. In this way, if the call to the price feed fails, the caller contract is still in control and can handle any errors safely and explicitly. Refer to https://blog.openzeppelin.com/secure-smart-contract-guidelines-the-dangers-of-price-oracles/ for more information regarding potential risks to account for when relying on external price feed providers. Also refer to this p...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_68_group

# `maxDueDate` is not checked against the `block.timestamp + state.riskConfig.maxTenor` value

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-648
- **Submitter:** Shield
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/648
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-648.md

## Brief Summary

The `BuyCreditLimit.validateBuyCreditLimit` function is used to validate the input parameters for buying credit as a limit order. The following check is in place to ensure that `maxDueDate` is not less than the `block.timestamp + minTenor`. But there is no check to ensure that `maxDueDate <= block.timestamp + state.riskConfig.maxTenor`. As a result a `maxDueDate` can be set to a very large value in the `loanOffer` because it does not have to complement the `state.riskConfig.maxTenor` value. This is against the purpose of introducing the `maxDueDate` which is to protect the lenders by setting a maximum timestamp for a loan to be matched. By allowing it to be set to any large value effectivel...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_95_group

# borrowers face a loss when selling or buying credit by market

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-679
- **Submitter:** Shield
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/679
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-679.md

## Brief Summary

During buying/selling credit through market fragmentation fee is an important aspect of the protocol when existing credit positions are involved but the issue is this fee is applied twice leading to invalid accounting & a loss for the borrowers

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_28_group

# The Market Maker does not pay the `swapFee` and it is the taker who pays the `swapFee` in the `executeBuyCreditMarket` function

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-712
- **Submitter:** Shield
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/712
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-712.md

## Brief Summary

In the `SellCreditMarket.executeSellCreditMarket` transaction the `seller of the credit position (msg.sender in this instance)` pays for the `fee` amount when selling the credit position to a new lender. The `seller` pays for both the `swapFee and the fragmentationFee` in this instance. In the `BuyCreditMarket.executeBuyCreditMarket` still the `seller of the credit position` who is the lender of the existing credit position pays for the `swapFee` amount as shown below: Here the cash amount sent to the `borrower` by the `buyer` is deducted by the `fees` amount. The buyer of the credit position (who is the msg.sender) in this instance the market maker does not pay for the `swapFee` and he onl...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_96_group

# DoS when trying to access the newly created debit & credit position

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-545
- **Submitter:** Shubham
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/545
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-545.md

## Brief Summary

Whenever someone wants to interact with a user's credit or debt position to buy/sell credit, compensate or liquidate either `getDebtPosition()`, `getCreditPosition()` or `getDebtPositionByCreditPositionId()` is called in most cases to get that user's position. If the id lies within the correct range, the position is returned else the call reverts. The issue is that the function does not check for the most recently created debt and credit position & whenever anyone tries to access it, revert occurs. *Setting the severity to High because both the debt & credit positions of a user are called in almost every function.* Impact The last/newly created debt and credit position of a user is inaccess...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_43_group

# Non-functional Borrowing Mechanism Leads to Debt Accrual Without Fund Transfer

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-356
- **Submitter:** Squilliam
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/356
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-356.md

## Brief Summary

The Size protocol's borrowing mechanism, implemented in the `SellCreditMarket` library, allows users to accrue debt without actually receiving the borrowed funds. This discrepancy occurs due to an incorrect implementation of the token transfer logic in the `executeSellCreditMarket` function. This vulnerability can lead to several severe issues: Users accrue debt without receiving borrowed funds, potentially leading to unfair liquidations. The protocol's internal accounting becomes inconsistent with actual fund distribution. Malicious actors could exploit this to manipulate their positions or the protocol's overall state. It may lead to insolvency of the protocol if exploited at scale. Loss...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# `SellCreditLimit` function allows blacklisted addresses to create borrow offers, bypassing USDC restrictions

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-548
- **Submitter:** Squilliam
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/548
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-548.md

## Brief Summary

The `sellCreditLimit` function in the Size protocol allows addresses blacklisted by USDC to create borrow offers, effectively initiating the process of borrowing USDC. This vulnerability stems from the function's lack of blacklist checks and the protocol's use of internal accounting and wrapper tokens, which do not enforce the blacklist restrictions present in the underlying USDC token contract. This vulnerability undermines USDC's blacklist feature, which is crucial for regulatory compliance and anti-money laundering efforts. By allowing blacklisted addresses to create borrow offers, the Size protocol provides a pathway for these addresses to potentially access and use USDC, circumventing...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_59_group

# Approvals can be revoked by the token owner, leading to liquidations being reverted

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-146
- **Submitter:** TECHFUND-inc
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/146
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-146.md

## Brief Summary

`executeLiquidate()` when triggered by a caller and transfer funds into the size contract for borrowed tokens worth `futureValue`, the logic transfers the collateral from borrower's account to the caller's account. Like wise, a fee applicable is also transfer from Borrow's account to fee fee Recipient account. The vulnerability is that the transfers are performed using `transferFrom` function which needs prior approval from the token owner to spend the tokens to be processed by the transaction. The issue is that liquidation will happen in future time period when circumstances arise. The token owner has the power to revoke the approvals granted at any time by passing the approved amount as 0...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Liquidators can be front-runned

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-718
- **Submitter:** Takarez
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/718
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-718.md

## Brief Summary

Borrower can avoid liquidation and also discourage `liquidators` from doing their job.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_21_group

# Unauthorized Token Transfer Risk in SellCreditMarket.sol

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-424
- **Submitter:** ThomasHeim
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/424
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-424.md

## Brief Summary

Allowing an arbitrary address to be used as the from parameter in the transferFrom function poses a significant security risk. It means that anyone could potentially transfer tokens from someone else's address without proper authorization, leading to unauthorized token transfers and potential loss of funds for the token owner. This vulnerability exposes users to financial risks and undermines the security of the protocol.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_62_group

# Inability to Withdraw ETH in Size.sol

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-428
- **Submitter:** ThomasHeim
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/428
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-428.md

## Brief Summary

Detailed description of the impact of this finding. The Size contract does not provide a mechanism to withdraw ETH, making it impossible to recover funds sent to it. This can lead to a permanent loss of any ETH accidentally or intentionally sent to the contract. The lack of a withdrawal function in the Size contract poses a critical risk as it leads to the irreversible loss of any ETH sent to the contract. Users and developers may accidentally send ETH to the contract, which would then be irretrievable due to the absence of a withdrawal mechanism. This can result in significant financial loss and erode trust in the contract's safety and usability.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_205_group

# Missing Return Statement in approve Function of NonTransferrableToken.sol

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-433
- **Submitter:** ThomasHeim
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/433
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-433.md

## Brief Summary

The approve function in the NonTransferrableToken contract does not contain a return statement, even though the ERC-20 standard expects one. This could potentially lead to unexpected behaviors when interacting with other contracts that rely on the ERC-20 standard. The absence of an explicit return statement in the approve function can cause interoperability issues with other smart contracts that expect a boolean return value upon calling this function. This may lead to unwanted behaviors, such as transaction failures or incorrect state assumptions, which can affect the usability and reliability of the token in a broader ecosystem.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Centralization Risk for Trusted Owners in Size.sol

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-435
- **Submitter:** ThomasHeim
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/435
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-435.md

## Brief Summary

The contracts identified in src/Size.sol utilize role-based access control (RBAC) through OpenZeppelin's AccessControlUpgradeable library. Roles such as DEFAULT_ADMIN_ROLE, BORROW_RATE_UPDATER_ROLE, PAUSER_ROLE, and KEEPER_ROLE are assigned specific privileged functionalities. While RBAC enhances security by restricting access to critical functions, it introduces centralization risks as these roles are entrusted with significant control over contract operations. Centralization Risk: Owners assigned roles (DEFAULT_ADMIN_ROLE, BORROW_RATE_UPDATER_ROLE, PAUSER_ROLE, KEEPER_ROLE) have the authority to perform critical administrative tasks. Malicious actions or errors by these privileged account...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_158_group

# Centralization Risk for trusted owners in NonTransferrableToken.sol

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-437
- **Submitter:** ThomasHeim
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/437
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-437.md

## Brief Summary

Detailed description of the impact of this finding.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_108_group

# Using ERC721::\_mint() Can Be Dangerous in NonTransferrableToken.sol

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-444
- **Submitter:** ThomasHeim
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/444
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-444.md

## Brief Summary

Using _mint() without verifying the recipient can handle ERC721 tokens may lead to scenarios where tokens are minted to addresses that do not support ERC721, rendering the tokens inaccessible and potentially causing loss of value.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_217_group

# Clash of MultiCall Calls Due to Incomplete Validation

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-624
- **Submitter:** Topmark
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/624
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-624.md

## Brief Summary

Clash of MultiCall Calls Due to Incomplete Validation

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_06_group

# Credit Amount Can be Reduced Below The Minimum Credit Value

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-632
- **Submitter:** Topmark
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/632
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-632.md

## Brief Summary

The credit amount can be reduced below the minimum credit due to error in validation breaking Protocol functionality

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_49_group

# Missing check for `RoundID`

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-180
- **Submitter:** Velislav4o
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/180
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-180.md

## Brief Summary

When sequencer goes down instead of reverting it will give stale price

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Missing check on loan repayment resulting in free credits

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-271
- **Submitter:** Velislav4o
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/271
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-271.md

## Brief Summary

User can repay loans without actually having the amount

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_15_group

# Insufficient Repayment Handling in repayDebt Function

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-486
- **Submitter:** XDZIBECX
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/486
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-486.md

## Brief Summary

The repayDebt function is use for processing repayments and updating the debt position, and the current implementation only deducts the repayment amount from the future value of the debt without considering any accrued interest or fees. and This is can result in a discrepancy between the recorded debt and the actual amount owed by the borrower here : the bug is from the reduction of the debt's future value by the repayment amount, without accounting for interest and fees that have accumulated over the life of the debt. and The absence of calculations for these additional amounts leads to an incomplete and incorrect update of the debt position and a malicious borrower can exploit this vulner...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_117_group

# Lack of Slippage Protection in Limit Orders Allows for Unfavorable Deals via MEV

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-251
- **Submitter:** alix40
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/251
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-251.md

## Brief Summary

Size protocol allows user to open limit orders either using `LoanOffer` or `BorrowOffer` and in the same way the protocol allows users to dynamically calculate apy using the interest rates from AAVE and a variable `marketMultiplier`that the user can set in the limit order. Knowing that the interest rates on AAVE are highly volatile for USDC most of the time the rate is around 6% but sometimes the interest rates peaks shortly to 70 or 80% for a couple of hours or days. Shortly said the rates on aave on the short term are highly unpredictable and this could open up the oppurtinity for MEV BOTS to to backrun the import of high rates (or really low 1%) to lock borrowers or lenders in unfavourab...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Absense of a minCredit amount in limit orders, could expose users to unfavourable or unprofitable matches

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-330
- **Submitter:** alix40
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/330
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-330.md

## Brief Summary

The size protocol is expected to be deployed on mainnet where the gas fees could range from 5 usd to 40 USDC, it might be important especially for lenders to decide the minamount of their Liquidity provided to be lent out in a CreditPosition, as the cost to claim those Positions could eat out at their yield. Please also note, as it is the case now, lenders are required to call the `claim()` function in order for their lent out assets to be accredited to their account in order to be lent out. > N.B please also note that the protocol has put in a place a `fragmentationFee` to disentivize credit traders from fragmenting creditPositions (later streamlined to be claimed by the bot), However this...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# The decimals precision of the `priceFeed` is not handled in `collateralRatio()`, leading to potential wrong valuation

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-627
- **Submitter:** alix40
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/627
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-627.md

## Brief Summary

The price returned from chainlink oracle in the `collateralRatio()` function, is not handeld and the precision of the value is not checked or converted to WAD. Knowing that the chainlink oracle will return values with 8 decimals for non-eth pairs (eth/usd is not an eth pair and will return 8 decimals), but the value recieved is treated as if it is in WAD.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_04_group

# Report on `CreateInitCodeSizeLimit` Issue and Potential Optimizations for the Size Protocol

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-374
- **Submitter:** arabgodx
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/374
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-374.md

## Brief Summary

**Finding Category**: Gas Optimization / Library Issues **Description**: During the deployment of contracts for the Size protocol, an error `Invalid transaction: CreateInitCodeSizeLimit` was encountered. This error indicates that the size of the initialization code for the contract exceeds the block gas limit allowed by the Ethereum network. Here are the detailed observations and potential solutions: Problem The error `Invalid transaction: CreateInitCodeSizeLimit` occurs when the combined size of the contract's bytecode and the initialization code exceeds the maximum allowed size. This can result from large contracts with complex logic or multiple levels of inheritance. Observations 1. **Co...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Incorrect Validation of BuyCreditMarket

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-645
- **Submitter:** arabgodx
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/645
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-645.md

## Brief Summary

Vulnerability Report for Size Protocol: Incorrect Validation of BuyCreditMarket Summary The Size protocol has a potential vulnerability in the `validateBuyCreditMarket` function that allows borrowers to act as lenders in the same transaction. This creates a scenario where only the protocols benefits. Vulnerability Description The function `validateBuyCreditMarket` does not properly enforce the distinction between borrowers and lenders. This allows a borrower to simultaneously find themselves as a lender in the same transaction, leading to potential mismanagement. Affected Code The vulnerability is present in the following function: Impact Allowing borrowers to act as lenders in the same tra...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_52_group

# Credit seller(lender early exit) could be made to borrow unexpectedly due to offer being the same as borrower's offer.

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-413
- **Submitter:** asui
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/413
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-413.md

## Brief Summary

A trader(credit seller) who lends with a higher interest rate and early exit using with lower interest rate to earn profit could be forced to borrow incase he liquidate a position. The trader pays interest + swap fees and also got exposed to being underwater and liquidated, when his intention was to never borrow but instead sell his credit.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_00_group

# Risk can be transferred forcefully to unwilling risk takers(borrowers who wants USDC).

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-626
- **Submitter:** asui
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/626
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-626.md

## Brief Summary

Unwilling users can be forced to borrow unwithdrawable USDC, which defeats the whole purpose of borrowing.And made to pay interests on it plus exposing him to borrow risks like overDue or underwater. Useless as in not convertible to USDC, for borrowers who wants USDC.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_51_group

# Borrower can set openingLimitBorrowCR to max to make BuyCreditMarket order always revert

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-292
- **Submitter:** ayden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/292
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-292.md

## Brief Summary

According to the documentation, when a borrower places a competitive limit order with a long duration and high APY, this order remains in a favorable position at the front of the page. However, if this competitive order consistently reverts, it can disrupt the system's functionality. A borrower could intentionally set an `openingLimitBorrowCR` to cause the order to revert due to CR checks.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Minimum credit check can lead to sell credit market dos

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-361
- **Submitter:** ayden
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/361
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-361.md

## Brief Summary

Credit lower than minimum credit check can lead to sell credit market dos.User is unable to sell the remaining credit in a creditPostion.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_166_group

# `Size.sellCreditMarket(SellCreditMarketParams)` race condition can force many transaction to revert in normal circumstances

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-714
- **Submitter:** carlitox477
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/714
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-714.md

## Brief Summary

The way borrowers sell credit on market is specifying the amount of credit to sell, the tenor and the interest rate. However, this function can be frontrunned by a any other borrower. If the sum of both borrower amount is greater than the credit bid, the second borrower will revert the transaction. This is an issue because the amount of the first borrower may not fulfill the credit bid, forcing the second borrower transaction to revert. This could happen again in normal cirmustances, by a third borrower, and so on, leading to many honest borrowers transaction to revert as well. Impact Race condition of `Size.sellCreditMarket(SellCreditMarketParams)` can force many honest transactions to rev...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_39_group

# Unrestricted Proxy Initialisation in the Size contract

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-106
- **Submitter:** debo
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/106
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-106.md

## Brief Summary

Detailed description of the impact of this finding. The initialise function can be called by any address, which means that any user can initialise the proxy contract with arbitrary values. This can lead to unauthorised control over the contract and potential loss of funds or misconfiguration.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_53_group

# Extremely High Gas Consumption on the Multicall library

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-82
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/82
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-82.md

## Brief Summary

Detailed description of the impact of this finding. High gas consumption for certain functions can lead to inefficiencies and increased costs for users interacting with the smart contract. Functions consuming above 1 million gas units are typically considered expensive and may deter users due to the high transaction fees. While this issue does not directly result in a Denial of Service (DoS), it indicates a need for optimisation to improve the overall performance and cost-effectiveness of the contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_16_group

# Lender/SelfLiquidator receives less collateral from borrower than expected.

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-452
- **Submitter:** dhank
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/452
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-452.md

## Brief Summary

Lender/SelfLiquidator receives less collateral from borrower than expected due to extra flooring done to calculate the `ProRataAssignedCollateral`.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Lender is not allowed to take calculated Risk to avoid a huge selfLiquidation Loss - Buy/SellCreditMarketOrder

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-455
- **Submitter:** dhank
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/455
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-455.md

## Brief Summary

Lender is not allowed to take calculated Risk to avoid a huge selfLiquidation loss.Buyer/seller can buy/sell his own creditPosition to himself because there is no msg.sendr validation. This allows user to avoid huge self Liquidation Loss by dividing the current credit and self Liquidating only one of them.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Inconsistent State and Loss of Funds Due to Missing Token Transfer Logic in `buyCreditMarket` Function

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-50
- **Submitter:** emerald7017
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/50
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-50.md

## Brief Summary

Currently, the `buyCreditMarket` validates the input parameters, executes the buy credit market action, and validates that the user is not below the opening limit borrow CR and that the variable pool has enough liquidity. However, it does not transfer the tokens from the caller to the contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_14_group

# Lenders can steal money from borrowers when market conditions are very volatile.

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-488
- **Submitter:** evmboi32
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/488
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-488.md

## Brief Summary

Lenders can steal money from borrowers when market conditions are very volatile.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Incorrect BorrowAToken Mint in Deposit Underlying

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-274
- **Submitter:** foxb868
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/274
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-274.md

## Brief Summary

`depositUnderlyingBorrowTokenToVariablePool` function is responsible for depositing the underlying borrow token (e.g., USDC) into the Aave variable pool and minting the corresponding `borrowAToken` (e.g., aUSDC) to the specified `to` address. However, bug in the function is that it directly mints the `borrowAToken` using the `scaledAmount` obtained from the Aave pool, without considering the exchange rate between the `underlyingBorrowToken` and the `borrowAToken`. 1. The function first transfers the `underlyingBorrowToken` from the `from` address to the contract: [DepositTokenLibrary.sol#L52](https://github.com/code-423n4/2024-06-size/blob/8850e25fb088898e9cf86f9be1c401ad155bea86/src/librar...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Binary Search Function Returns Incorrect Indices

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-275
- **Submitter:** foxb868
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/275
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-275.md

## Brief Summary

When the searched value is not found in the array, the function should return the indices of the largest element less than or equal to the value and the smallest element greater than or equal to the value. However, due to the bug in the return statement, the function incorrectly returns `(high, low)` instead of `(low, high)`, can lead to incorrect results when using the `binarySearch` function in other parts of the codebase. [Math.sol#L51-L68](https://github.com/code-423n4/2024-06-size/blob/8850e25fb088898e9cf86f9be1c401ad155bea86/src/libraries/Math.sol#L51-L68)

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of function `claimReward()` to claim reward

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-571
- **Submitter:** grearlake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/571
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-571.md

## Brief Summary

Function `claimRewards()` [link](https://docs.aave.com/developers/periphery-contracts/rewardscontroller#claimrewards) is used to claim reward from aave pool: function claimRewards( address[] calldata assets, uint256 amount, address to, address reward ) external override returns (uint256) { require(to != address(0), 'INVALID_TO_ADDRESS'); return _claimRewards(assets, amount, msg.sender, msg.sender, to, reward); } But it does not appear in protocol, which lead to additional reward cant be claimed. Impact Additional reward cant be claimed and stuck forever in aave.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_187_group

# Oracle will return wrong price if price goes out of chainlink's minanswer/maxanswer

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-651
- **Submitter:** grearlake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/651
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-651.md

## Brief Summary

Chainlink oracles have a minAnswer-maxAnswer price range in which they report prices. If an asset's price goes out of this range, it will continue to report the price which the asset has crossed. Which will lead to incorrect price returned. In the `PriceFeed` contract, there is no checking condition about minAnswer and maxAnswer. Impact Incorrect price returned when price is outside of range [minAnswer, maxAnswer].

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_13_group

# Ordinary liquidations can become unprofitable as expiration time cannot be set

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-503
- **Submitter:** hyh
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/503
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-503.md

## Brief Summary

Liquidate deals with exchange of the borrow token (debt position's future value) and collateral token (`liquidatorProfitCollateralToken` amount), so can turn unprofitable with a passage of time (e.g. being stuck in txpool for a while), but doesn't have the deadline parameter. Impact Liquidation orders can become unprofitable if were executed with some delay (e.g. due to locally raising gas costs). Because of that liquidators will add an additional premium to threshold conditions of their strategies, i.e. will be slower to act, so protocol health will be more loosely maintained, the probability of bad debt appearance will be higher, so will be overall protocol risk, which will diminish its t...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_44_group

# Lenders might capitalize on more borrow token that they're entitled to during significant liquidity index changes

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-405
- **Submitter:** iamandreiski
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/405
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-405.md

## Brief Summary

There are a few ways in which lenders can capitalize and receive significantly more borrow token than they're entitled to after a loan was paid off: - `claim()` wasn't called for a certain amount of time after it became available and allowed for a significant index change OR it wasn't sanitized frequently enough by bots. - In a situation where the borrower and lender are the same person, a significant liquidity index change was observed, and the change was "sandwiched" in order to capitalize on it.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_70_group

# legitimate selfLiquidate can be stopped by users looking to earn liquidation profit

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-425
- **Submitter:** inzinko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/425
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-425.md

## Brief Summary

Users looking to earn liquidation profit can stop selfLiquidation by minting collateral tokens to the borrower that is underwater increasing the collateral ratio, blocking the check that prevents selfLiquidation, if it is greater than 100% , leading to DOS of legitimate function

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_75_group

# Every borrower in Size can block the keeper from choosing them as a replacement borrower

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-477
- **Submitter:** inzinko
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/477
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-477.md

## Brief Summary

When the keepers come to liquidate and replace with another borrow with preferable better borrow offer, this process of picking borrowers can be blocked by the borrowers, because after performing liquidations in the `Size::liquidateWithReplacement` it tries to check the replacement is not under opening limitborrow cr, but the protocol allows each borrower to set their own borrow limit cr, this allows borrowers even though they have a borrowOffer set their cr so high that it fails this check and reverts the call.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# borrowers can't use some credit positions to compensate their debt positions

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-527
- **Submitter:** inzinko
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/527
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-527.md

## Brief Summary

The protocol specifies that users can use all credit positions, to compensate for their debt positions, but this is not the case with some credit positions, where the credit position to compensate is greater than the credit position with debt to repay, and the difference between them is less than 50 USDC(which is the minimum credit allowed),preventing the compensation process

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_233_group

# Impact of Variable Pool (Aave v3) Failures on Size Protocol's Liquidation Process

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-613
- **Submitter:** jo13
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/613
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-613.md

## Brief Summary

If the **Variable Pool (Aave v3)** fails to supply or withdraw for any reason, such as **supply caps**, it can have a **significant impact** on the **liquidation process** within the protocol. The **Variable Pool** is essential for providing the necessary **liquidity** to facilitate **liquidations**. Its failure can lead to a **liquidity crunch**, **increased insolvency risk** for borrowers, **financial losses** for the protocol, and overall **operational inefficiencies**.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Potential Reentrancy Vulnerability in Deposit and Withdraw Functions

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-97
- **Submitter:** kerdakov
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/97
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-97.md

## Brief Summary

The presence of potential reentrancy vulnerabilities in the `DepositTokenLibrary` contract's deposit and withdraw functions can lead to significant security risks. Specifically, an attacker can exploit these vulnerabilities to perform recursive calls to the deposit and withdraw functions, allowing them to manipulate the state in ways that could result in unauthorized minting or withdrawal of tokens. This can lead to a draining of funds from the protocol, financial losses for users, and a compromise of the protocol's integrity.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_97_group

# A malicious lender can demand extremely high interest rate on loans.

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-572
- **Submitter:** lightoasis
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/572
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-572.md

## Brief Summary

Lenders can set interest rate on loans to be extremely high. This is because there is no cap on the amount of interest that lenders can set. Malicious lenders can abuse this to get high interest payments from borrowers. As seen below, there is no cap on the interest rate that lenders can demand. The apr set by the lender is not properly validated. This allows malicious lenders to demand extremely high interest payments from borrowers. [BuyCreditLimit.sol#L59](https://github.com/code-423n4/2024-06-size/blob/8850e25fb088898e9cf86f9be1c401ad155bea86/src/libraries/actions/BuyCreditLimit.sol#L59)

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Malicious lenders can create buy credit limit orders without depositing any tokens into the protocol.

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-610
- **Submitter:** lightoasis
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/610
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-610.md

## Brief Summary

Lending in size protocol is as an exchange of cash for credit, or "buying credit". Lenders deposit USDC, which earns a variable rate via Aave until matched with a borrower. See: [lending-buying-credit](https://docs.size.credit/non-technical/lending-buying-credit). The issue is that lenders can buy credit limits without depositing any tokens into the protocol. This is because there are no checks in place to verify if the lenders actually deposited any tokens into the protocol before executing their buy credit limit order. Malicious lenders can exploit this to execute buy credit limit orders without depositing any tokens into the protocol. As seen below, there is no check in place to verify i...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Incorrect Compensate Event Emission

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-496
- **Submitter:** m4k2
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/496
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-496.md

## Brief Summary

The `Compensate` event, emitted at the beginning of the `executeCompensate` function, might return an incorrect value. The event currently emits `params.amount` as `amount`, but the actual amount used for compensation is `amountToCompensate`, which is the lesser of `params.amount` and `creditPositionWithDebtToRepay.credit`. Impact Events are crucial for monitoring blockchain actions. This discrepancy could mislead front-end applications and external technologies reliant on these events, resulting in user errors and potential operational issues.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Whales can force overDue liquidation via block stuffing

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-685
- **Submitter:** m4k2
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/685
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-685.md

## Brief Summary

Whales, or accounts with substantial cryptocurrency holdings, can exploit block stuffing to force positions into overdue status, triggering liquidation and benefiting at the expense of the borrower. This risk increased with the current low gas fees on the mainnet. Description Whales can manipulate the `dueDate` mechanism by submitting a high volume of transactions, known as block stuffing. The `dueDate` is a timestamp that marks when a position becomes overdue and eligible for liquidation. Once overdue, the protocol allows liquidators to purchase the collateral at a discount. By block stuffing, whales can ensure positions become overdue, thereby triggering liquidation and allowing them to b...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_31_group

# sellCreditMarket only checks borrow limit when opening a new position

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-245
- **Submitter:** max10afternoon
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/245
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-245.md

## Brief Summary

The [sellCreditMarket](https://github.com/code-423n4/2024-06-size/blob/8850e25fb088898e9cf86f9be1c401ad155bea86/src/Size.sol#L188C14-L188C30) function checks that the collateral ratio is above the opening rate, only when creating a new position. This means that a lender can sell very dangerous debt to other "passive" lenders that are offering to buy credit, event if the position is only 1 wei above the liquidation threshold. Enabling lenders to dump their soon to be liquidated position onto other traders (if they fear that the liquidation won't be exercised).

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_102_group

# Early repay may be front ran, by MEV for profit

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-246
- **Submitter:** max10afternoon
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/246
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-246.md

## Brief Summary

An early repay by the borrower can be front ran, by buying the credit position from the lender, then letting the repay execute, and than withdraw right away. This will be profitable for the attacker and will cause losses to the lender (which should be entitled to the full future value of the trade, since they were exposed to the risk of the debt up to the repay time). Note: although there are fees applied on every credit swap, this fees are applied on the receiver of the tokens and not on the caller of the function. Meaning that, not only the fee won't represent a cost for the MEV, but they will instead appear as losses to the original lender, making things worst if anything.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# It's possible to swap credit without payng fees

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-289
- **Submitter:** max10afternoon
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/289
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-289.md

## Brief Summary

Due to a division, in which de divisor is user controllable, it is possible to swap credit without paying the related fees.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unauthorized access via `delegatecall` in `setUserConfiguration` allows malicious user configuration alterations and manipulation of credit position IDs on behalf of other users

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-379
- **Submitter:** moneyversed
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/379
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-379.md

## Brief Summary

The `setUserConfiguration` function in the Size contract includes checks that should prevent unauthorized access in typical scenarios. Specifically, the `validateSetUserConfiguration` function verifies that the caller (i.e., `msg.sender`) is the legitimate owner of the credit positions being modified. However, using a crafted exploit that leverages the `delegatecall` mechanism, a malicious actor can bypass these checks, impersonating another user and altering their configuration. This exploit demonstrates the vulnerability and underscores the importance of robust access control mechanisms. Vulnerability Detail The `setUserConfiguration` function is intended to allow users to update their cr...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_24_group

# Inadequate liquidation logic in `multicall` facilitates strategic abuse throughout bypass of liquidation checks in batched operations

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-641
- **Submitter:** moneyversed
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/641
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-641.md

## Brief Summary

The Size credit marketplace has a critical vulnerability in its multicall function implementation that allows for manipulation of the liquidation logic, basically leading to substantial financial losses. This vulnerability arises due to improper handling of post-condition checks within multicalls, enabling attackers to manipulate the protocol's state and exploit liquidations in a malicious manner. Vulnerability Detail The vulnerability lies in the `multicall` function of the Size protocol, within the `Multicall` library. The `multicall` function allows users to batch multiple operations into a single transaction, which is particularly useful for complex interactions such as liquidations. Ho...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_79_group

# In `getAdjustedAPR` the APR will not be adjusted and it will revert

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-183
- **Submitter:** mrMorningstar
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/183
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-183.md

## Brief Summary

The function `getAdjustedAPR` will revert always when `marketRateMultiplier` is not 0, making the function unusable and impacting the `getAPR` function which will then impact following functions of the protocol: - `getAPRByTenor` (both of them for loan and for borrow offer) - `getBorrowOfferAPR`, `getLoanOfferAPR`, `validateBuyCreditMarket`, `validateSellCreditMarket` and `validateLiquidateWithReplacement` (they all call `getAPRByTenor` which is already impacted)

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary

# Malicious actors can do block stuffing and liquidate a debt to profit when repayment is close to the dueDate timestamp.

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-389
- **Submitter:** mt030d
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/389
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-389.md

## Brief Summary

The `borrowAToken` is a yield-bearing token that continuously accrues interest. It is in a borrower's best interest to repay the debt close to the `dueDate` timestamp. However, if a borrower submits a `repay()` transaction too close to the `dueDate`, malicious actors can stuff a few blocks to prevent the transaction from being executed before the deadline. Once the `dueDate` passes, an attacker can front-run the `repay()` with a `liquidate()` transaction to profit. Consequently, the borrower loses funds due to liquidation.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_46_group

# Usage of an incorrect version of Ownable library

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-472
- **Submitter:** nikhilx0111
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/472
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-472.md

## Brief Summary

the contract is designed to be deployed as an upgradeable proxy contract However, the current implementation is using an non-upgradeable version of the Ownable library: @openzeppelin/contracts/access/Ownable.sol instead of the upgradeable version: @openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol. impact When a regular, non-upgradeable `Ownable` library is used in an upgradeable proxy contract, the ownership control might not be managed correctly across different versions of the contract. This could lead to unintended changes in ownership or unexpected behaviors during upgrades.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# repay does not transfer excess

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-495
- **Submitter:** nikhilx0111
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/495
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-495.md

## Brief Summary

users can repay their debt by calling repay users can only repay in full however if the user accidently specified an amount larger than their debt balance, the excess will be stuck in the contract as the contract does not refund excess repaid amount impact innocent users can lose their funds

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_01_group

# reentrancy in accountinglibrary.sol

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-716
- **Submitter:** nikhilx0111
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/716
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-716.md

## Brief Summary

the contract is burning the burning the debt token and updating the debtposition after executing the repay impact debtPosition.futureValue could be modified multiple times in unexpected ways, leading to potential loss of funds or incorrect state

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_206_group

# `borrowOffer` can be replaced by calling `sellCreditLimit`, so that `buyCreditMarket` and `LiquidateWithReplacement` can be bypassed.

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-658
- **Submitter:** petro_1912
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/658
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-658.md

## Brief Summary

Malicious users can grief liquidator by replacing `borrowOffer` to null by calling `sellCreditLimit` again.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_32_group

# `NonTransferrableScaledToken` uses wrong unscaled calculation.

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-662
- **Submitter:** petro_1912
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/662
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-662.md

## Brief Summary

`NonTransferrableScaledToken` (borrowAToken) is very important in the protocol. But unscaling is not so correct, so overall behavior of protocol will not work properly.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_57_group

# No Storage Gap for Upgradeable Contract Might Lead to Storage Slot Collision

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-678
- **Submitter:** petro_1912
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/678
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-678.md

## Brief Summary

This contract is supposed to be upgradeable which means storage gap is required. Storage of Size contracts might be corrupted during an upgrade.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_110_group

# lender can not liquidate self even though loan is overdue unless borrower's debt position is not underwater.

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-691
- **Submitter:** petro_1912
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/691
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-691.md

## Brief Summary

Lenders can't liquidate their credit position even though loan is overdue.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_69_group

# Rounding issue in `getCreditPositionProRataAssignedCollateral` calculation.

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-705
- **Submitter:** petro_1912
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/705
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-705.md

## Brief Summary

The proportional collateral assigned to CreditPosition may be slightly lower than expected.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Wrong unscale could lead to stuck amount

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-147
- **Submitter:** radin100
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/147
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-147.md

## Brief Summary

User loss of funds due to stuck amount

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Bad stale borrowRate check could lead to borrow/lend at unexpected rate

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-181
- **Submitter:** radin100
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/181
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-181.md

## Brief Summary

Cash borrowed/lent at stale AAVE's variablePoolBorrowRate

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# The use of `block.timestamp` in calculating tenors and deadlines can lead to MEV exploitation

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-282
- **Submitter:** sunnyStefi
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/282
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-282.md

## Brief Summary

`block.timestamp`s are used to calculate *tenor* values, *deadlines* and *stale prices*. This value can be known in advance and manipulated by bots and malicious validators. The `tenor` values are generated by subtracting the `block.timestamp` from the debt position due date, while `deadline`s are calculated by adding a `block.timestamp` to a `tenor`. Impact A MEV bot or a miner can manipulate the `block.timestamp` to slightly increase or decrease tenor and deadline values. This can lead to the following issues: 1. **Validation manipulations** - The error `TENOR_OUT_OF_RANGE` can be easily induced during a *compensation validation* [L53](https://github.com/code-423n4/2024-06-size/blob/8850e...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing Access Control

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-696
- **Submitter:** sunnyboy95
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/696
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-696.md

## Brief Summary

Unauthorized users may access or modify critical data, leading to potential security vulnerabilities.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# manipulate rates of aave based rates

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-394
- **Submitter:** tomerganor
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/394
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-394.md

## Brief Summary

it is very cheap to manipulate the rate that is offchain by pumping the aave pool. for example: lets say the utilisation is 80% 8M out of 10M with 0.3M Attacker can loop deposit borrow to (given LTV of 97% in emode) up to 33 times leverage. 10M collateral 9.7M debt for the Attacker which give us 17.7M total debt 20M total deposit 88.5% with only 300K of funds even if the rates are 100% apr the cost is 0.19% per day (without the gains from the interest in this high utilisation is about 80% of paid interest so it is only 0.04% per day in that case). that means Attacker can manipulate the apr parameter and accept possitions that users didnt intend to.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Invalid validation in `UpdateConfig.sol` prevents the swap fee apr from being more than the minimum

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-635
- **Submitter:** trachev
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/635
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-635.md

## Brief Summary

The admin is able to update the swap fee apr through a call to `updateConfig`. The issue is that the function validates whether the value is above the maximum incorrectly. As a result the `swapFeeApr` cannot be above the minimum.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# `YieldCurveLibrary.getAPR` rounds in different directions, returning unexpected results

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-706
- **Submitter:** trachev
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/706
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-706.md

## Brief Summary

The `getAPR` function of `YieldCurveLibrary` is intended to always round in the same direction according to the README.md file: `In some generic situations, such as in yield curve calculations, the rounding is always in one direction.`. The issue is that due to incorrect calculations, the rounding is inconsistent and differs in some cases.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of maxDueDate in borrow offers/limit-orders

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-313
- **Submitter:** ubl4nk
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/313
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-313.md

## Brief Summary

> maxDueDate is a protection for passive lenders, they can specify the timestamp at which their limit orders become invalid. When the lenders submit a yield-curve/limit-order, they should define a maxDueDate, it means they should specify how long these rates/configurations/yield-curve are valid (because the market condition might change and the lenders don't want to issue unaffordable loans). However this feature is not available in borrower's offers/limit-orders which can cause some significant issues for passive borrowers.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Solidity version >= 0.8.20 will not work on Base due to PUSH0

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-314
- **Submitter:** ubl4nk
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/314
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-314.md

## Brief Summary

Solidity version >= 0.8.20 will not work on Base due to PUSH0.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lenders can be front-runned while disabling forSale flag for a credit position

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-359
- **Submitter:** ubl4nk
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/359
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-359.md

## Brief Summary

Attacker can front-run a lender and buy the credit position before `forSale` flag is disabled. This will lead to loss of credit positions that the lenders are not willing to sell.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_66_group

# Incomplete validation check in the [Claim::executeClaim], a lender can claim a token before its duedate of repayment without the borrower acknowledgement/approval

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-595
- **Submitter:** willycode20
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/595
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-595.md

## Brief Summary

Loan Repayment Status: The function checks if the loan associated with the provided creditPositionId is marked as REPAID using `state.getLoanStatus(params.creditPositionId)`. If the loan is not yet repaid, the function reverts with an error indicating that the loan is not repaid. Credit Position Already Claimed: It also checks if the credit amount in the creditPosition is zero, which would indicate that the credit position has already been fully claimed. If so, the function reverts with an error stating that the credit position has already been claimed. However, neither of these checks directly addresses the maturity or tenor of the loan. To ensure that a claim is only processed if the loan...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing check for zero value of msg.value can cause a DoS attack on the deposit function

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-600
- **Submitter:** willycode20
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/600
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-600.md

## Brief Summary

A malicious actor exploiting a contract that doesn't properly handle transactions with zero msg.value could leverage this oversight to manipulate the contract's state or drain funds. One common exploitation technique is known as "Denial of Service" (DoS) attack. Exploitation Technique: DoS Attack A malicious actor could exploit this vulnerability by flooding the contract with transactions that consume gas but do not contribute to the intended functionality (i.e., depositing tokens). Since the contract does not check msg.value, it cannot differentiate between legitimate and malicious transactions.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Mishandling of ETH in the [Liquidate::executeLiquidateWithReplacement] can lead to accounting error and siphoning of fund by malicious user

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-616
- **Submitter:** willycode20
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/616
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-616.md

## Brief Summary

The `Liquidate:executeLiquidateWithReplacement` mint new tokens(USDC) and transfer to a new borrower without a provided step for burning out the token of the previous user(borrower), this can cause incorrect accounting within the system and possibly stealing of funds by a malicious operator

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Loan limit check logic loophole leads to loan limit exceeding problem

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-692
- **Submitter:** xiao
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/692
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-692.md

## Brief Summary

This check uses a strict "greater than" condition. However, the vulnerability is exploitable when `borrowAToken.totalSupply()` is equal to `borrowATokenCap`. In this case, even though the borrow limit has been reached and no more tokens should be borrowed (because we have reached the maximum limit), the conditional statement will cause the function to return `false`, allowing additional borrowing transactions to exceed the borrow limit.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Insufficient validation before state change

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-700
- **Submitter:** xiao
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/700
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-700.md

## Brief Summary

This code is intended to allow users to buy credit on the market. However, there is a logic vulnerability in the code. The main problem of the vulnerability lies in the order of checking the validation conditions and executing the state changes. Specifically, the state change operation `executeBuyCreditMarket` is executed before the last two validation operations `validateUserIsNotBelowOpeningLimitBorrowCR` and `validateVariablePoolHasEnoughLiquidity`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_94_group

# Liquidate doesn't always increase the sender collaterals as per LIQUIDATE_01 invariant

- **Contest:** Size
- **Slug:** 2024-06-size
- **Submission:** V-609
- **Submitter:** zarkk01
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-06-size-validation/issues/609
- **Source snapshot:** competitions/2024-06-size/submissions/raw/V-609.md

## Brief Summary

As per [main invariants](https://github.com/code-423n4/2024-06-size/blob/8850e25fb088898e9cf86f9be1c401ad155bea86/README.md?plain=1#L289) of Size protocol, during a liquidation process the collaterals of the sender (liquidator) must increase. We can see it here at [LIQUIDATE_01](https://github.com/code-423n4/2024-06-size/blob/8850e25fb088898e9cf86f9be1c401ad155bea86/README.md?plain=1#L303C1-L303C58) : However, this is not always the case since the protocol let a user to liquidate his own if this position is liquidatable meaning he is underwater. So, in the case the liquidator is the same address with the borrower of the position, this will result to the collaterals of the sender to be, actu...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary
