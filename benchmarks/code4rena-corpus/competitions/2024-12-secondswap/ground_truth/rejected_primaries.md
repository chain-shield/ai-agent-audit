# Rejected Primary Findings: SecondSwap

# Changing minListingDuration after a listing Is created alters unlisting penalties for ongoing listings

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-125
- **Submitter:** 0xastronatey
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-125
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-125.txt

## Brief Summary

The marketplace contract uses the current value of minListingDuration from MarketplaceSetting contract at the time of unlisting, rather than the value that was in place when the listing was originally created. This means if minListingDuration is modified mid-way, sellers face new and unexpected conditions when they try to unlist. They might avoid or incur penalties they never anticipated, simply because the rules changed after they listed their tokens. if ((listing.listTime + IMarketplaceSetting(marketplaceSetting).minListingDuration()) > block.timestamp) { //.. Apply penalty fee } Here, minListingDuration() is fetched dynamically at unlisting time rather than using a stored value from the...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Potential Reentrancy Due to External Calls Before State Updates

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-80
- **Submitter:** TheFabled
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-80
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-80.txt

## Brief Summary

In the spotPurchase function, the contract makes external token transfers before updating critical state variables, such as listing.balance and listing.status: _handleTransfers(listing, _amount, discountedPrice, bfee, sfee, _referral); // Update listing status listing.balance -= _amount; listing.status = listing.balance == 0 ? Status.SOLDOUT : Status.LIST; Issue: If a malicious token contract is used, it could re-enter the spotPurchase function during the external token transfer calls (safeTransferFrom and safeTransfer). This could potentially allow an attacker to manipulate the state variables in unintended ways. Impact: Reentrancy Attack: An attacker might exploit this to execute function...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Lost of `SecondSwap::claim` rewards when Arbitrum L2 sequencer is down

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-67
- **Submitter:** KiteWeb3
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-67
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-67.txt

## Brief Summary

The chains the protocol will be deployed on is defined in the "Question" section in the readme file and are: Ethereum, Base, zkSync, Arbitrum. On the Arbitrum chain when the sequencer is down the state changes can still happen on L2 by passing them from L1 through the Delayed Inbox. During periods when the Arbitrum sequencer is unavailable, transactions are rerouted through the Delayed Inbox, which applies address aliasing to the sender's address. When the sequencer is down on Arbitrum, the msg.sender of a transaction from the Delayed Inbox is aliased in this way: L2_Alias = L1_Contract_Address + 0x1111000000000000000000000000000000001111 If the user that initiates a claim during the sequen...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Missing functionality in `SecondSwap_Marketplace` could harm seller leading in loss of funds and protocol leading in missing profit

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-61
- **Submitter:** mrMorningstar
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-61
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-61.txt

## Brief Summary

If some seller wants to sell their tokens they need first to be listed via listVesting that looks like this: function listVesting( address _vestingPlan, uint256 _amount, uint256 _price, uint256 _discountPct, ListingType _listingType, DiscountType _discountType, uint256 _maxWhitelist, address _currency, uint256 _minPurchaseAmt, bool _isPrivate ) external isFreeze { require( _listingType != ListingType.SINGLE || (_minPurchaseAmt > 0 && _minPurchaseAmt <= _amount), "SS_Marketplace: Minimum Purchase Amount cannot be more than listing amount" ); require(_price > 0, "SS_Marketplace: Price must be greater than 0"); require( (_discountType != DiscountType.NO && _discountPct > 0) || (_discountType =...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Inability to Sell Remaining Tokens Due to Base Amount Check

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-71
- **Submitter:** macart224
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-71
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-71.txt

## Brief Summary

The current implementation of the baseAmount calculation in the listVesting function can lead to a situation where a user is unable to sell their last portion of a listing when the listing type is set to partial. Specifically, if the remaining amount of tokens is too small, it may not satisfy the condition baseAmount > 0, causing the transaction to always revert. This can prevent buyers from purchasing the remaining tokens. The relevant

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# unlistVesting() is susceptible to frontrunning leading to loss of funds to the seller.

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-27
- **Submitter:** dhank
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-27
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-27.txt

## Brief Summary

When a user calls unlistVesting( _vestingPlan, _listingId) before a minListingDuration, he should also pay a fixed penaltyFee in USDT. That means user X calls this functions only if he is aware that reclaiming all the vesting tokens they listed for sale is worth more than the penaltyfee. But an attacker can frontrun the above execution by calling the spotPurchasing from the X's _listingId leaving only 1 token remaining in the _listingId. (if attacker takes the entire amount ,listingId is marked as soldOut and the attcker cannot explait since the unlistingexecution reverts) So when the X's unlistVesting() takes place he is actually paying the penaltyFee for 1 vesting token , which puts the X...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Missing CoinAdded event negates transparency and tracking of supported tokens

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-66
- **Submitter:** Mushow
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-66
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-66.txt

## Brief Summary

The addCoin function in the SS_Marketplace contract does not emit the CoinAdded event when a new token is added as a supported payment currency. While the CoinAdded event was previously included in the code as a comment, it was forgotten to be uncommented and properly implemented in the final version. As a result, there is no explicit on-chain record of newly supported tokens. Without the CoinAdded event, users and developers cannot track or verify which tokens have been added as supported currencies. This significantly reduces transparency and usability, forcing users to manually query the isTokenSupport mapping or analyze all past transactions to identify supported tokens. Impact Negates...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Lack of proper support for tokens with decimals that are less than 6

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-109
- **Submitter:** 056Security
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-109
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-109.txt

## Brief Summary

From the contest README, the protocol has stated that they will handle tokens with decimals that are less than 6. This is not properly handled, as such tokens could lead to the vesting plan's releaseRate to be 0 due to how Solidity rounds down.

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Incorrect Error Message in `setMaxWhitelist` Function

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-88
- **Submitter:** Brene
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-88
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-88.txt

## Brief Summary

The error message in the setMaxWhitelist function is misleading. It currently states, "SS_Whitelist: amount cannot be lesser that the current whitelist amount," which inaccurately describes the condition being checked. The condition actually compares _maxWhitelist to maxWhitelist, not totalWhitelist. Impact Incorrect error messages can lead to confusion for developers and users, making it difficult to understand the nature of the error and how to resolve it.

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Unbounded `maxSellPercent` Value May Cause Incorrect Sell Limit Calculations

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-112
- **Submitter:** Mrxstrange
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-112
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-112.txt

## Brief Summary

The vestingSettings[plan].maxSellPercent variable lacks validation to ensure it falls within a reasonable range, such as 0-100%. An unbounded or incorrectly set maxSellPercent could result in erroneous sell limit calculations, allowing users to bypass expected restrictions or unintentionally restrict valid transactions. Impact: If maxSellPercent exceeds the intended range, users may sell more than allowed, violating the vesting plan’s constraints. If maxSellPercent is negative or zero due to a misconfiguration, valid sales could be blocked. Steps to Reproduce: Deploy the contract and configure a maxSellPercent outside the expected range (e.g., 200%). Attempt to sell tokens exceeding userAll...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Sellers can not sell all his vesting tokens which are bought from marketplace

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-34
- **Submitter:** 0xc0ffEE
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-34
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-34.txt

## Brief Summary

Users who are allocated will have the sell limit according to the allocated amount. Besides, users who are not allocated, can be able to sell all the tokens bought from marketplace. However, in case an user's allocation is decreased, the users will be unable to sell the bought vesting tokens. Here, the root cause is the discrepancy between total allocation and sold/bought amount tracked by vesting manager, such that when reallocation happens, only the user's total allocation changed but the sold/bought amount stays unchanged. This can cause user's next sells apply the wrong sell limit. function listVesting(address seller, address plan, uint256 amount) external onlyMarketplace { require(vest...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Giving the `manager` access to `createVesting()` and `createVestings()` can lead to unintended consequences if the `VestingManger` contract is updated

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-142
- **Submitter:** BenRai
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-142
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-142.txt

## Brief Summary

The vestingManager currently has access to the functions createVesting() and createVestings() even though in the current implementation the vestingManger never calls those functions. But since the vestingManager contract is upgradable, there might be an upgrade which allows normal users to make arbitrary calls from the vestingManager. If this is the case, a normal user would be able to create his own vestings since the tokens for vestings are always taken from the tokenIssuer.

## Rejection Reason

Final severity/evaluation: Low; Valid Primary Sufficient

# Transfer vestings without fee payment through listing and purchasing with high discount and low price

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-4
- **Submitter:** fyamf
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-4
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-4.txt

## Brief Summary

Users are able to bypass fee payment when transferring vestings to another address by listing a vesting with maximum discount and minimum price.

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Inconsistent Fee Validation Logic

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-68
- **Submitter:** Mrxstrange
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-68
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-68.txt

## Brief Summary

The following require statement contains a logical inconsistency between the condition and the accompanying error message: require(_fee > -1 && _fee <= 5000, "SS_VestingManager: Seller fee cannot be less than 0"); The condition _fee > -1 is equivalent to _fee >= 0. However, the error message "Seller fee cannot be less than 0" suggests that _fee must indeed be non-negative (greater than or equal to 0). This creates confusion for both developers and auditors since _fee can technically be any value greater than -1, including negative values like -1 (if _fee is a decimal type). Impact This inconsistency can lead to: Misleading Error Messages: Developers may misinterpret the actual requirements...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Using the `currentAllocation` to calculate the `sellLimit` breaks the functionality of the ´maxSellPercent´ variable

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-147
- **Submitter:** BenRai
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-147
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-147.txt

## Brief Summary

Because the currentAllocation is used to calculate the amount of unvested tokens a user can sell, after a while all unvested tokens will be sellable breaking the purpose of the ´maxSellPercent´ variable. This will increase the potential supply of listed vestings over time which will result in a lower marked price (assuming the same demand but higher supply). This will lead to substantial financial loss for users wanting to sell their listing since they will need to sell for a lower price.

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Blockchain re-org may cause users to mistakenly purchase tokens at a higher price.

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-17
- **Submitter:** shaflow2
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-17
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-17.txt

## Brief Summary

The spotPurchase function uses the plan address and ID to distinguish different orders, but it lacks user protection against spending with the highest slippage control. If a blockchain reorganization occurs, it may swap the order IDs, causing users to mistakenly purchase tokens at a higher price.

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Lack of deadline checks in spotPurchase allows miners to manipulate execution timing for profit

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-129
- **Submitter:** bumbleb33
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-129
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-129.txt

## Brief Summary

Description: The spotPurchase function in SecondSwap_Marketplace.sol lacks a deadline parameter and slippage protection checks. This allows miners to intentionally delay transaction execution until market conditions are unfavorable for the buyer. When a user submits a purchase transaction, miners can: See the transaction in the mempool Hold the transaction without including it in a block Wait until the price moves unfavorably for the user Include the transaction when it will result in the worst possible execution price This is particularly dangerous because: The marketplace deals with vested tokens which may have volatile prices There's no way for users to specify maximum acceptable slippage

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Bypassing penalty fee by purchasing own listed vesting

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-3
- **Submitter:** fyamf
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-3
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-3.txt

## Brief Summary

When a user unlists a vesting early, they are required to pay a penalty fee. However, the user can bypass this penalty fee by purchasing their own vesting. In this scenario, instead of paying the penalty fee (which defaults to 10 ether in USDT), the user only needs to pay the buyer and seller fees (both set at a default rate of 2.5%).

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Excessive Constraints in `setMaxWhitelist` Function Result in Potential Loss of Functionality

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-89
- **Submitter:** y51r
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-89
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-89.txt

## Brief Summary

The setMaxWhitelist function in the contract is overly restrictive due to redundant require statements. Specifically, the condition _maxWhitelist > maxWhitelist imposes unnecessary constraints on reducing the maxWhitelist value. The issue arises because: The _maxWhitelist > totalWhitelist check is already sufficient to ensure the whitelist capacity cannot be set lower than the number of currently whitelisted users, preserving functionality. The additional _maxWhitelist > maxWhitelist constraint prevents the lotOwner from reducing the maxWhitelist value after it has been increased. If the lotOwner sets an excessively high value (e.g., uint256.max), they lose the ability to adjust the value d...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# `VestingTransferred` event can be griefed with duplicate / forged transaction Ids leading off-chain data corruption

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-133
- **Submitter:** YouCrossTheLineAlfie
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-133
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-133.txt

## Brief Summary

The SecondSwap_VestingDeployer::transferVesting function can be used by the token issuer for transferring vesting. This function uses a transactionId as an unique identifier, which is passed down in the emitted event as well function transferVesting( address _grantor, address _beneficiary, uint256 _amount, address _stepVesting, string memory _transactionId <@ - // Used by off-chain mechanism for reconciliation ) external { The issue lies with the way this function is being permissioned require( _tokenOwner[msg.sender] == address(SecondSwap_StepVesting(_stepVesting).token()), <@ - // This allows a malicious _stepVesting contract to be passed "SS_VestingDeployer: caller is not the token owner"

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Admin Can Prevent Users from Unlisting Their Vestings by Arbitrarily Increasing `penaltyFee`

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-52
- **Submitter:** y51r
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-52
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-52.txt

## Brief Summary

The setPenaltyFee function in the contract allows the admin to update the penaltyFee value. This fee is charged to users when they attempt to unlist their vestings before the minimum listing duration has elapsed. However, the function does not enforce an upper bound on the penaltyFee, allowing the admin to set it to an excessively high value. This issue creates a Denial of Service (DoS) scenario for users attempting to unlist their vestings before the minimum listing duration. Specifically: High Penalty Fee Enforcement: If the admin sets an unreasonably high penaltyFee, users will be unable to meet the requirement to pay the fee, preventing them from unlisting their vestings. Exploitation o...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Receiver can be same as sender when spot purchasing a Listing

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-111
- **Submitter:** AshishLach
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-111
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-111.txt

## Brief Summary

When placing listings using spotPurchase() function, protocol doesn't check whether receiver is not the same as the sender Need to keep this require statement in spotPurchase() function require(msg.sender != listing.seller, " seller cant be same as receiver") Write a detailed description of the root cause and impact(s) of this finding. While no major attack path was identified, this could potentially enable attacks based on learnings from previous protocols. Therefore, it is advisable to keep this check in place

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# deployVesting Allows Creation of Vesting Schedules with Past Start Times Enabling Immediate Token Claims

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-77
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-77
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-77.txt

## Brief Summary

A vulnerability has been identified in the vesting schedule deployment mechanism of SecondSwap. The deployVesting function in SecondSwap_VestingDeployer lacks proper temporal validation, allowing the creation of vesting schedules that begin in the past. While the function validates that the start time precedes the end time, it fails to validate against the current blockchain timestamp. The vulnerability stems from the interaction between SecondSwap_VestingDeployer and SecondSwap_StepVesting. When a vesting schedule is deployed with a past start time, the claimable() function in SecondSwap_StepVesting calculates token availability based on elapsed time since the start. This calculation doesn...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Admin of the marketplace can prevent users from claiming their allocations.

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-18
- **Submitter:** sl1
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-18
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-18.txt

## Brief Summary

As stated by the contest page, admin of the protocol is expected to only be able to pause and unpause the marketplace, but shouldn't be able to prevent users from claiming their vesting allocations. However, currently if user a has vesting that is listed on the marketplace and decides to unlist and claim it, they won't be able to do so while the market is paused due to unlistVesting() function having isFreeze modifier SecondSwap_Marketplace.sol#L339 function unlistVesting( address _vestingPlan, uint256 _listingId ) external isFreeze { Impact Admin of the marketplace can prevent users from claiming their vesting allocations.

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Wrongly Set Penalty Fee

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-14
- **Submitter:** EPSec
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-14
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-14.txt

## Brief Summary

In SecondSwap_MarketplaceSetting the penaltyFee is set to 10 ether. This is 10e18. However, the penalty fee is paid with the USDT token. USDT token on all supported chains is a token with 6 decimals. Setting the penaltyFee to 10 ether will set it to 10e18 which is 10000000000000 USDT ~ 10000000000000 $. The penaltyFee can be changed, however, this mistake renders a big part of the protocol useless until that is done by the owner. What is more, if a user theoretically has this amount of USDT and is using the given contract with an approval set to an equal or bigger number, they will lose funds thinking that the fee should be much less.

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Incomplete Function Existence Check

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-82
- **Submitter:** TheFabled
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-82
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-82.txt

## Brief Summary

The doesFunctionExist function aims to check if a target contract has a specific function. However, the implementation may not reliably determine the existence of the function. function doesFunctionExist(address target, string memory functionSignature) public view returns (bool) { bytes4 selector = bytes4(keccak256(bytes(functionSignature))); (bool success, ) = target.staticcall(abi.encodeWithSelector(selector)); return success; } Issue: This method may return true even if the function does not exist but a fallback function is present. Additionally, it does not account for function input parameters. Impact: False Positives: The contract might assume a function exists when it does not, leadi...

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient

# Linear discount is not calculated correctly

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-21
- **Submitter:** 0xrex
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-21
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-21.txt

## Brief Summary

Token sellers can set up discounts for tokens being sold such as 10%. This would mean that the users would pay 10% less overall for the token sale. For the case of the linear discount, using the current calculation, the actual discount linearly will be 5% max not 10.

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Creating a vesting for a user that is past the last claim step will lock the tokens

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-146
- **Submitter:** gesha17
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-146
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-146.txt

## Brief Summary

When creating a vesting, the amount that is being assigned to the benificiary is calculated based on how many steps are left. If the beneficiary has claimed past the last step, then the releaseRate is set to 0. This means that any new tokens that are assigned to this beneficiary will essentially become locked. This can happen by chance or by an honest mistake E.g. a token owner decides to grant some extra tokens to a benificiary at the last step, seeing the beneficiary has not yet claimed his tokens, but the beneficiary decides to claim his tokens just before the owners transaction executes, so the tokens become locked. This can be somewhat mitigated by the beneficiary - he can make a listi...

## Rejection Reason

Final severity/evaluation: Low; Valid Primary Sufficient

# Token issuer referral reward manipulation through combined direct and market Transfers

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-143
- **Submitter:** Sabit
- **Claimed severity:** High
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-143
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-143.txt

## Brief Summary

The tokenIssuer can be used as a referrer in marketplace purchases, even after directly transferring tokens to the same buyer. This creates an unintended profit opportunity through referral rewards that undermines the marketplace's referral incentive system. The vulnerability stems from insufficient referral validation in the spotPurchase function: function spotPurchase(address _vestingPlan, uint256 _listingId, uint256 _amount, address _referral) external isFreeze { // Current validation only checks if buyer is not referrer _validatePurchase(listing, _amount, _referral); } function _validatePurchase(Listing storage listing, uint256 _amount, address _referral) private view { require(msg.send...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# Rounding Issues Leading to Potential Exploits

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-101
- **Submitter:** TheFabled
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** duplicate
- **Rejection category:** duplicate
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-101
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-101.txt

## Brief Summary

There are calculations involving division that could result in integer rounding errors, potentially leading to zero values or inaccurate amounts. Example in _handleTransfers: uint256 baseAmount = (_amount * discountedPrice) / uint256(10 ** (IERC20Extended(...).decimals())); require(baseAmount > 0, "SS_Marketplace: Amount too little"); Issue: If discountedPrice is low or the token decimals are large, baseAmount could be calculated as zero due to integer division truncation. This could allow purchasers to buy tokens for zero payment or cause a denial of service. Impact: Economic Loss: Sellers might lose funds if tokens are sold for less than intended. Denial of Service: Transactions might fai...

## Rejection Reason

Marked duplicate in authenticated Code4rena submission detail/table.

# token like cUSDCv3 can be stolen from the SecondSwap_StepVesting contract

- **Contest:** SecondSwap
- **Slug:** 2024-12-secondswap
- **Submission:** F-118
- **Submitter:** Fon
- **Claimed severity:** Medium
- **Final severity:** Low
- **Status:** low_or_qa
- **Rejection category:** low_or_qa_severity
- **Source URL:** https://code4rena.com/audits/2024-12-secondswap/submissions/F-118
- **Source snapshot:** competitions/2024-12-secondswap/submissions/raw/F-118.txt

## Brief Summary

when creating vesting, the _totalAmount is transferred with token.safeTransferFrom(tokenIssuer, address(this), _totalAmount); however, no check confirms that the contract balance increases by that amount. for tokens like cUSDCv3 where transferring type(uint256).max just transfers the total balance of the account this can lead to accounting errors and stolen funds

## Rejection Reason

Final severity/evaluation: Low; Valid Sufficient
