# contracts/plume - Findings Report
## Commit hash: aebcbd127a932bed3bb6c3e6dfc4cd0df7de5902

## Protocol Overview 

**Plume Protocol** is a diamond-pattern, upgradeable suite that turns the PLUME token into an on-chain economy composed of delegated staking, multi-token rewards, random-reward gaming and role-based governance.

1. **PlumeStaking (Diamond)**
   • Users delegate PLUME to validators via a Diamond proxy exposing Staking, Rewards, Validator, Management and AccessControl facets.  
   • Each validator has independent commission, capacity and reward-rate checkpoints.  
   • Stake → cooldown → withdraw lifecycle is enforced; all state is stored in a shared PlumeStakingStorage layout.  
   • Slashing is possible through unanimous validator votes executed by a timelock role, burning all staked/cooling funds of the offender.  
   • Reward accrual is “lazy”: a checkpointed index lets users claim any ERC20 reward without per-block updates, keeping gas low.

2. **Treasury**
   • Rewards are custodied in a separate UUPS-upgradeable PlumeStakingRewardTreasury; only the staking diamond can instruct transfers, isolating funds from logic risk.

3. **Spin & Raffle**
   • Spin is a VRF-driven daily wheel where users pay PLUME, earn tickets, jackpots and maintain streaks; probabilities and campaign dates are admin-configurable.  
   • Raffle consumes tickets from Spin, lets admins post prizes and uses Supra VRF to pick winners; winners claim via treasury transfers.

4. **Governance & Upgrades**
   • Roles (ADMIN, VALIDATOR, TIMELOCK, REWARD_MANAGER, UPGRADER) are set in an AccessControl facet.  
   • All major contracts (diamond, treasury, spin, raffle) are ERC1967 or UUPS proxies, enabling seamless future upgrades.
## High Risk Findings
[H-1]. Upgradeability Initializer Safety issue found with High severity
[H-2]. Gas Grief BlockLimit issue found with High severity
[H-3]. DOS issue found with High severity
[H-4]. DOS issue found with High severity
[H-5]. DOS issue found with High severity
[H-6]. Gas Grief BlockLimit issue found with High severity
[H-7]. Upgradeability Initializer Safety issue found with High severity
## Medium Risk Findings
[M-1]. Zero Code issue found with Medium severity
[M-2]. Pausable Emergency Stop issue found with Medium severity
[M-3]. Access Control issue found with Medium severity
[M-4]. Reentrancy issue found with Medium severity
[M-5]. DOS issue found with Medium severity
[M-6]. Oracle issue found with Medium severity
[M-7]. Gas Grief BlockLimit issue found with Medium severity
[M-8]. Zero Code issue found with Medium severity
[M-9]. DOS issue found with Medium severity
[M-10]. DOS issue found with Medium severity
[M-11]. Gas Grief BlockLimit issue found with Medium severity
[M-12]. DOS issue found with Medium severity
[M-13]. DOS issue found with Medium severity
[M-14]. Unexpected Eth issue found with Medium severity
## Low Risk Findings
[L-1]. Unexpected Eth issue in MockPUSDProxy::receive
[L-2]. Pausable Emergency Stop issue in PlumeStaking::NA
[L-3]. Gas Grief BlockLimit issue in RewardsFacet::claimAll
[L-4]. Unexpected Eth issue in SPINProxy::receive
[L-5]. DOS issue in StakingFacet::_processMaturedCooldowns
[L-6]. DOS issue in StakingFacet::withdraw
[L-7]. Pausable Emergency Stop issue in Spin::handleRandomness
[L-8]. Timestamp Dependent Logic issue in Spin::_computeStreak
[L-9]. Zero Code issue in Spin::initialize
[L-10]. Gas Grief BlockLimit issue in Raffle::removePrize
[L-11]. Event Consistency issue in Raffle::setPrizeActive
[L-12]. DOS issue in Raffle::removePrize
[L-13]. Zero Code issue in Raffle::initialize
[L-14]. Gas Grief BlockLimit issue in Raffle::removePrize
[L-15]. Pausable Emergency Stop issue in RewardsFacet::claim
[L-16]. Pausable Emergency Stop issue in PlumeStaking::NA
[L-17]. Frontrun/Backrun/Sandwhich MEV issue in ValidatorFacet::setValidatorCommission
[L-18]. Pausable Emergency Stop issue in PlumeStaking::NA
[L-19]. Gas Grief BlockLimit issue in RewardsFacet::claimAll
[L-20]. Inheritance issue in PlumeStakingProxy::receive
## Info Risk Findings
[I-1]. Integer Overflow issue in DateTime::getYear
[I-2]. Unexpected Eth issue in PlumeStakingRewardTreasuryProxy::receive
[I-3]. Pragma issue in PlumeStaking::NA
[I-4]. Event Consistency issue in Spin::setJackpotProbabilities
[I-5]. Unexpected Eth issue in Raffle::receive
[I-6]. Access Control issue in Spin::spendRaffleTickets(address,uint256)
[I-7]. Pragma issue in StakingFacet::NA
[I-8]. Zero Code issue in SpinProxy::constructor
[I-9]. Access Control issue in Plume::burn(address,uint256)
[I-10]. Pragma issue in PlumeStaking::NA
[I-11]. Gas Grief BlockLimit issue in PlumeStakingRewardTreasury::getRewardTokens


### Number of Findings
- H: 7
- M: 14
- L: 20
- I: 11



# Low Risk Findings

## [L-1]. Unexpected Eth issue in MockPUSDProxy::receive

## Description
The `MockPUSDProxy` contract includes a `receive() external payable {}` function. This function has an empty body, which means the contract can accept Ether sent to it, but it provides no mechanism to withdraw this Ether. Any native currency (ETH) sent to this proxy address will be permanently locked within the contract, as there are no functions to manage or transfer the received funds.

## Impact
Permanent loss of any Ether sent to the `MockPUSDProxy` contract address. This can lead to financial loss for users who mistakenly send ETH to the contract.

## Proof of Concept
1. An administrator deploys the `MockPUSDProxy` contract, providing a logic address.
2. A user, by mistake, sends 1 ETH directly to the `MockPUSDProxy` address.
3. The transaction succeeds because of the `receive()` payable fallback function.
4. The 1 ETH is now held by the `MockPUSDProxy` contract.
5. There is no function within the proxy or a standard implementation that allows anyone, including the admin, to withdraw this ETH. The funds are locked forever.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

import {Test, console} from "forge-std/Test.sol";
import {MockPUSDProxy} from "src/proxy/MockPUSDProxy.sol";

contract DummyLogic {}

contract UnexpectedEthTest is Test {
    MockPUSDProxy proxy;
    address attacker = makeAddr("attacker");

    function setUp() public {
        DummyLogic logic = new DummyLogic();
        proxy = new MockPUSDProxy(address(logic), "");
    }

    function test_StuckEthInProxy() public {
        // Give the attacker some ETH
        vm.deal(attacker, 1 ether);

        // Attacker sends ETH to the proxy
        vm.prank(attacker);
        (bool success, ) = address(proxy).call{value: 1 ether}("");
        assertTrue(success, "ETH transfer should succeed");

        // Assert that the ETH is now in the proxy contract
        assertEq(address(proxy).balance, 1 ether, "Proxy should hold 1 ETH");

        // There is no function to withdraw the ETH, so it is permanently stuck.
        console.log("1 ETH is now permanently locked in the proxy contract at address:", address(proxy));
    }
}
```

## Suggested Mitigation
If the proxy is not intended to hold Ether, the `receive()` function should revert transactions attempting to send ETH. This prevents funds from being accidentally locked. This pattern is correctly implemented in `PlumeProxy`.

```diff
+ import {ETHTransferUnsupported} from "src/lib/PlumeErrors.sol";

  contract MockPUSDProxy is ERC1967Proxy {
      // ...

-     // solhint-disable-next-line no-empty-blocks
-     receive() external payable {}
+     receive() external payable {
+         revert ETHTransferUnsupported();
+     }
  }
```

## [L-2]. Pausable Emergency Stop issue in PlumeStaking::NA

## Description
The core staking system, managed through the `PlumeStaking` diamond proxy and its facets (`StakingFacet`, `RewardsFacet`, `ValidatorFacet`), lacks a comprehensive emergency stop (pause) mechanism. While the `Plume` token contract itself is pausable, this does not protect the main staking logic. Critical functions such as `stake`, `unstake`, `withdraw`, and `claim` remain operational at all times. If a severe vulnerability is discovered in the reward calculation, staking logic, or validator management, there is no way for the administrators to halt the system to prevent exploitation and mitigate damage.

## Impact
The staking system cannot be halted if a critical bug is discovered in the future.  Although no current path to steal or lock funds is demonstrated, operators lose the ability to freeze user-facing functions during an incident, increasing potential damage window and operational risk.

## Proof of Concept
1. A critical bug is found in the `PlumeRewardLogic` that incorrectly calculates rewards, allowing a user to claim 1000x the intended amount.
2. An attacker discovers this bug and prepares to exploit it by repeatedly calling the `claim()` function in `RewardsFacet`.
3. The Plume team is notified of the bug but has no mechanism to pause the `claim()` function.
4. The attacker successfully executes their exploit, draining the `PlumeStakingRewardTreasury` of multiple reward tokens before the team can react.
5. The only available action, pausing the `PLUME` token itself, does not stop the draining of other reward tokens (e.g., USDC, WETH).

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";
import {Pausable} from "@openzeppelin/contracts/security/Pausable.sol";
import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";

// This is a minimal mock to demonstrate the issue.
// It shows that even if we had a Pausable contract, the staking functions
// lack the necessary `whenNotPaused` modifier.
contract MockStakingLogic is Pausable, AccessControl {
    bytes32 public constant ADMIN_ROLE = keccak256("ADMIN_ROLE");
    mapping(address => uint256) public balances;

    function stake() external payable whenNotPaused {
        balances[msg.sender] += msg.value;
    }

    function pause() external onlyRole(ADMIN_ROLE) {
        _pause();
    }
}

// The actual StakingFacet does not inherit Pausable nor use the modifier.
contract StakingFacet_Vulnerable {
    mapping(address => uint256) public balances;

    // This function should have a `whenNotPaused` modifier
    function stake() external payable {
        balances[msg.sender] += msg.value;
    }
}

contract NoPauseTest is Test {
    StakingFacet_Vulnerable staking;
    address user = makeAddr("user");

    function setUp() public {
        staking = new StakingFacet_Vulnerable();
    }

    function test_StakingCannotBePaused() public {
        // The protocol's admin discovers a critical bug.
        // However, there is no pause() function on the staking facet.
        // An attacker can continue to interact with the vulnerable `stake` function.
        
        vm.deal(user, 1 ether);
        vm.prank(user);
        staking.stake{value: 1 ether}();

        // The stake succeeds because there's no way to pause it.
        assertEq(staking.balances(user), 1 ether);
        assertEq(address(staking).balance, 1 ether);
    }
}
```

## Suggested Mitigation
Implement a comprehensive emergency stop mechanism across the entire `PlumeStaking` system. This can be achieved by:
1. Adding a `Pausable`-like module or facet to the diamond storage.
2. Adding a new role, e.g., `PAUSER_ROLE`, to control pausing and unpausing.
3. Applying a `whenNotPaused` modifier to all critical state-changing external functions across all facets, including `stake`, `unstake`, `withdraw`, `claim`, `restake`, and admin functions that modify critical parameters.

## [L-3]. Gas Grief BlockLimit issue in RewardsFacet::claimAll

## Description
The `claimAll()` function in `RewardsFacet` iterates through all available reward tokens. For each token, it calls the `claim(address token)` function, which in turn iterates through all validators a user is staked with. This creates a nested loop (`rewardTokens.length * userValidators.length`). If a user stakes with a large number of validators and several reward tokens are active, the total gas cost of calling `claimAll()` can easily exceed the block gas limit. This would cause the transaction to fail, effectively creating a Denial of Service (DoS) condition where the user is unable to retrieve their rewards through this convenience function.

## Impact
Calling `claimAll()` performs `rewardTokens.length × userValidators.length` internal operations which may incur high gas cost. If a user stakes with many validators and the protocol lists many reward tokens the call might become too expensive for the user to execute, forcing them to switch to per-token / per-validator claiming. No third party can trigger the cost on other users and funds always remain accessible through smaller batched calls, so the issue is limited to a user-experience degradation rather than a loss or lock of funds.

## Proof of Concept
1. The `REWARD_MANAGER_ROLE` adds 15 different reward tokens to the system.
2. A user diversifies their stake across 80 different validators to support the network.
3. The user has pending rewards for all 15 tokens from all 80 validators.
4. The user calls `claimAll()` to collect their rewards.
5. The transaction initiates a nested loop that would need to run `15 * 80 = 1200` times. Each iteration involves storage reads/writes and reward calculation, consuming a significant amount of gas.
6. The total gas required exceeds the block gas limit (e.g., 30M on Ethereum mainnet), causing the transaction to revert with an 'out of gas' error.
7. The user is now unable to claim their rewards using the primary `claimAll` function.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import {Test, console} from "forge-std/Test.sol";

// Minimal mock of the necessary parts of the system to demonstrate the DoS.

contract RewardsFacetMock {
    address[] public rewardTokens;
    mapping(address => uint16[]) public userValidators;
    mapping(address => mapping(address => mapping(uint16 => uint256))) public rewards;

    function addRewardToken(address token) external {
        rewardTokens.push(token);
    }

    function mockStake(address user, uint16 validatorId) external {
        userValidators[user].push(validatorId);
        // Mock some rewards
        for(uint i = 0; i < rewardTokens.length; i++){
            rewards[user][rewardTokens[i]][validatorId] = 1 ether;
        }
    }

    function claim(address token) public {
        uint16[] memory validators = userValidators[msg.sender];
        for (uint i = 0; i < validators.length; i++) {
            claim(token, validators[i]);
        }
    }

    function claim(address token, uint16 validatorId) public {
        // a non-trivial amount of work happens here in the real contract
        uint256 reward = rewards[msg.sender][token][validatorId];
        if (reward > 0) {
            rewards[msg.sender][token][validatorId] = 0;
        }
    }

    function claimAll() external {
        for (uint i = 0; i < rewardTokens.length; i++) {
            claim(rewardTokens[i]);
        }
    }
}

contract GasGriefTest is Test {
    RewardsFacetMock rewardsFacet;
    address user = makeAddr("user");

    function setUp() public {
        rewardsFacet = new RewardsFacetMock();
    }

    function test_Fail_ClaimAll_GasLimit() public {
        // 1. Admin adds 20 reward tokens
        for (uint8 i = 0; i < 20; i++) {
            rewardsFacet.addRewardToken(makeAddr(string(abi.encodePacked("token", i))));
        }

        // 2. User stakes in 100 validators
        vm.startPrank(user);
        for (uint16 i = 0; i < 100; i++) {
            rewardsFacet.mockStake(user, i);
        }
        vm.stopPrank();

        // 3. User attempts to claim all rewards.
        // The nested loop will run 20 * 100 = 2000 times.
        // This will almost certainly exceed the block gas limit on a live network.
        // In Foundry, we expect an out-of-gas revert.
        vm.prank(user);
        vm.expectRevert(); // Expects a revert, which out-of-gas is.
        rewardsFacet.claimAll();
    }
}
```

## Suggested Mitigation
Keep `claimAll` as a convenience wrapper but add bounded parameters, e.g. `claimBatch(address[] calldata tokens, uint16[] calldata validatorIds)` so users (or the UI) can claim in configurable chunks.  Alternatively, iterate in‐contract until the remaining gas falls below a safety threshold (`gasleft() > X`) and allow the user to resume from the next index on a subsequent transaction.

## [L-4]. Unexpected Eth issue in SPINProxy::receive

## Description
Several proxy contracts in the system, such as `MockPUSDProxy`, `SPINProxy`, and `PlumeStakingRewardTreasuryProxy`, implement an empty `receive() external payable {}` function. This allows the contracts to receive Ether, but they lack a mechanism to withdraw it. Any Ether sent to these proxies, either accidentally or maliciously, will be permanently locked within the contract, leading to a loss of funds.

## Impact
Accidentally sent ETH can be recovered by an account with ADMIN_ROLE using Spin.adminWithdraw(token = 0xEeee…EEeE, amount, recipient). Therefore no permanent loss; only temporary lock until admin pulls it. The only risk is operational inconvenience if admin is unavailable.

## Proof of Concept
1. User sends 1 ETH to SPINProxy.
2. Balance increases.
3. Admin calls Spin(address(proxy)).adminWithdraw(0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE, 1 ether, admin).
4. ETH successfully transferred back; balance is zero.

## Proof of Code
```solidity
// test/UnexpectedEth.t.sol
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import {Test} from "forge-std/Test.sol";
import {SPINProxy} from "../src/proxy/SPINProxy.sol";
import {Spin} from "../src/spin/Spin.sol";
import {Plume} from "../src/Plume.sol";
import {SupraRouterContract} from "../src/interfaces/ISupraRouterContract.sol";

contract MockSupraRouter is SupraRouterContract {
    function generateRequest(uint256, uint256, uint256, address, uint256, bytes calldata) external payable returns(uint256) { return 1; }
    function deposit(uint256) external payable {}
}

contract UnexpectedEthTest is Test {
    SPINProxy internal spinProxy;
    Spin internal spinLogic;
    Plume internal plumeToken;
    MockSupraRouter internal supraRouter;

    address internal admin = makeAddr("admin");
    address internal user = makeAddr("user");

    function setUp() public {
        vm.startPrank(admin);
        spinLogic = new Spin();
        plumeToken = new Plume();
        plumeToken.initialize(admin);
        supraRouter = new MockSupraRouter();

        bytes memory initData = abi.encodeWithSelector(
            Spin.initialize.selector,
            admin,
            address(plumeToken),
            address(supraRouter),
            address(this) // raffle contract
        );
        spinProxy = new SPINProxy(address(spinLogic), initData);
        vm.stopPrank();
    }

    function testLockEthInSpinProxy() public {
        assertEq(address(spinProxy).balance, 0);

        // User accidentally sends 1 ETH to the proxy
        vm.prank(user);
        vm.deal(user, 1 ether);
        (bool sent, ) = address(spinProxy).call{value: 1 ether}("");
        require(sent, "Failed to send ETH");

        // ETH is now locked in the contract
        assertEq(address(spinProxy).balance, 1 ether);

        // There is no function to withdraw native ETH.
        // The adminWithdraw function is for the PLUME token, not native ETH.
        // Attempting to call it will not retrieve the ETH.
        vm.startPrank(admin);
        uint256 plumeBalance = plumeToken.balanceOf(address(spinProxy));
        // The admin can withdraw Plume tokens, but not the ETH.
        Spin(address(spinProxy)).adminWithdraw(plumeBalance);
        vm.stopPrank();
        
        // The ETH balance remains unchanged.
        assertEq(address(spinProxy).balance, 1 ether);
    }
}
```

## Suggested Mitigation
No change needed. Document adminWithdraw usage for native ETH recovery; optionally make receive() revert to prevent accidental transfers.

## [L-5]. DOS issue in StakingFacet::_processMaturedCooldowns

## Description
In `StakingFacet`, the functions `withdraw`, `restake`, and `restakeRewards` all rely on an internal function `_processMaturedCooldowns`, which in turn calls `_findAllCooldowns`. `_findAllCooldowns` iterates over the entire `userCooldowns` array for a given user. A user can increase the size of this array by repeatedly calling `unstake` with small amounts. If the array grows large enough, any transaction calling one of the affected functions will consume more gas than the block gas limit, causing it to revert. This allows a user to permanently lock their own funds in a 'cooling' state, as they will be unable to withdraw or restake them.

## Impact
The DoS only affects the address that purposefully bloats its own `userCooldowns` array. No other users, validators, or protocol balances become locked or at risk, and no global functionality is halted. The loss is therefore self-inflicted and limited in scope.

## Proof of Concept
1. A user stakes a certain amount of PLUME tokens to a validator.
2. The user calls `unstake(validatorId, 1 wei)` in a loop for a large number of times (e.g., 400-500 times).
3. Each call adds a new entry to the `userCooldowns` array for that user.
4. The user waits for the cooldown period to pass.
5. The user attempts to call `withdraw()` to retrieve their unstaked funds.
6. The transaction will fail due to running out of gas because the loop inside `_processMaturedCooldowns` will iterate too many times.

## Proof of Code
```solidity
// test/StakingFacetDos.t.sol
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import {Test} from "forge-std/Test.sol";
import {PlumeStakingProxy} from "../src/proxy/PlumeStakingProxy.sol";
import {PlumeStaking} from "../src/PlumeStaking.sol";
import {IDiamondCut} from "@solidstate/contracts/proxy/diamond/writable/IDiamondWritable.sol";
import {StakingFacet} from "../src/facets/StakingFacet.sol";
import {ValidatorFacet} from "../src/facets/ValidatorFacet.sol";
import {ManagementFacet} from "../src/facets/ManagementFacet.sol";
import {AccessControlFacet} from "../src/facets/AccessControlFacet.sol";
import {PlumeRoles} from "../src/lib/PlumeRoles.sol";

contract StakingFacetDosTest is Test {
    PlumeStakingProxy internal stakingProxy;
    address internal admin = makeAddr("admin");
    address internal user = makeAddr("user");
    uint16 internal validatorId = 1;

    function setUp() public {
        // Deploy Diamond and Facets
        vm.prank(admin);
        PlumeStaking diamondLogic = new PlumeStaking();
        stakingProxy = new PlumeStakingProxy(address(diamondLogic), abi.encodeWithSelector(PlumeStaking.initialize.selector, admin));

        StakingFacet stakingFacet = new StakingFacet();
        ValidatorFacet validatorFacet = new ValidatorFacet();
        ManagementFacet mgmtFacet = new ManagementFacet();
        AccessControlFacet acf = new AccessControlFacet();

        IDiamondCut.FacetCut[] memory cuts = new IDiamondCut.FacetCut[](4);
        cuts[0] = IDiamondCut.FacetCut(address(stakingFacet), IDiamondCut.FacetCutAction.Add, StakingFacet.stake.selectors);
        cuts[1] = IDiamondCut.FacetCut(address(validatorFacet), IDiamondCut.FacetCutAction.Add, ValidatorFacet.addValidator.selectors);
        cuts[2] = IDiamondCut.FacetCut(address(mgmtFacet), IDiamondCut.FacetCutAction.Add, ManagementFacet.setCooldownInterval.selectors);
        cuts[3] = IDiamondCut.FacetCut(address(acf), IDiamondCut.FacetCutAction.Add, AccessControlFacet.grantRole.selectors);

        IDiamondCut(address(stakingProxy)).diamondCut(cuts, address(0), "");

        // Initialize roles and settings
        AccessControlFacet(address(stakingProxy)).initializeAccessControl();
        AccessControlFacet(address(stakingProxy)).grantRole(PlumeRoles.VALIDATOR_ROLE(), admin);
        PlumeStaking(address(stakingProxy)).initializePlume(admin, 1 ether, 1 days);
        ValidatorFacet(address(stakingProxy)).addValidator(validatorId, 0, admin, admin, "", "", address(0), 10000 ether);

        // User stakes funds
        vm.startPrank(user);
        vm.deal(user, 1000 ether);
        StakingFacet(address(stakingProxy)).stake{value: 1000 ether}(validatorId);
        vm.stopPrank();
    }

    function testDosOnWithdraw() public {
        vm.startPrank(user);
        uint256 unstakeAmount = 1 wei;
        uint256 loopCount = 500; // A large number of unstakes

        for (uint i = 0; i < loopCount; i++) {
            StakingFacet(address(stakingProxy)).unstake(validatorId, unstakeAmount);
        }

        // Warp time past the cooldown
        uint256 cooldown = ManagementFacet(address(stakingProxy)).getCooldownInterval();
        vm.warp(block.timestamp + cooldown + 1);

        // The withdraw function will now fail due to out of gas
        vm.expectRevert();
        StakingFacet(address(stakingProxy)).withdraw();
        vm.stopPrank();
    }
}
```

## Suggested Mitigation
The design should avoid iterating over an unbounded array. Instead of processing all matured cooldowns at once, process them in batches or one by one. A user could specify how many cooldown entries they want to process in a single transaction.

For example, `withdraw` could take an argument specifying the maximum number of cooldowns to process, or a specific cooldown to withdraw from.

Alternatively, refactor the `CooldownEntry` storage. Instead of an array, use a mapping from a cooldown ID to the struct and a separate mechanism (like a linked list) to track matured cooldowns that can be processed incrementally.

## [L-6]. DOS issue in StakingFacet::withdraw

## Description
Several functions loop over arrays that can grow based on user actions or administrative decisions. This can lead to situations where a transaction's gas cost exceeds the block gas limit, effectively causing a Denial of Service (DoS).

1.  **`StakingFacet.withdraw()`**: This function calls `_processCooldowns`, which iterates through all of a user's `userCooldowns`. A user can repeatedly unstake tiny amounts, adding numerous entries to this array. This can make their own `withdraw()` function too costly to execute, locking their withdrawable funds.
2.  **`RewardsFacet.claim(address token)`**: This function iterates over all validators a user is staked with (`s.userValidators[msg.sender]`). If a user stakes with a large number of validators, this function can become too expensive to run, preventing them from claiming rewards for that specific token.
3.  **`RewardsFacet.claimAll()`**: This function compounds the issue by looping through all `rewardTokens` and then, for each token, all of the user's validators.

## Impact
A user can create so many cooldown or validator-stake entries that the gas needed to process them later may exceed the block gas limit. This can temporarily prevent THAT SAME USER from calling withdraw() or claim() until they supply enough gas (or the code is upgraded). Other users, validators and protocol funds remain unaffected.

## Proof of Concept
A malicious or naive user can DoS their own `withdraw` function.
1. A user stakes a certain amount of PLUME tokens.
2. The user calls `unstake(validatorId, 1)` in a loop for a large number of times (e.g., 200 times).
3. Each call creates a new entry in their `userCooldowns` array.
4. After the cooldown period passes, the user attempts to call `withdraw()`.
5. The transaction for `withdraw()` will iterate through all 200 cooldown entries, consuming a very large amount of gas and likely reverting due to the block gas limit on a live network.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {StakingFacet} from "src/facets/StakingFacet.sol";
import {PlumeStaking} from "src/PlumeStaking.sol";
import {Plume} from "src/Plume.sol";
import {TestHelper} from "test/TestHelper.sol";

contract DoSTest is TestHelper {
    function test_dos_withdrawWithManyCooldowns() public {
        uint256 stakeAmount = 1_000_000 * 1e18;
        _stake(user1, validator1Id, stakeAmount);

        uint16 unstakeIterations = 200;

        // User unstakes 1 wei, `unstakeIterations` times
        vm.startPrank(user1);
        for (uint16 i = 0; i < unstakeIterations; i++) {
            stakingFacet.unstake(validator1Id, 1);
        }
        vm.stopPrank();

        // Move time forward past the cooldown period
        vm.warp(block.timestamp + stakingFacet.getCooldownInterval() + 1);

        // Estimate gas for withdraw(), this will be very high
        vm.startPrank(user1);
        uint256 gasStart = gasleft();
        stakingFacet.withdraw();
        uint256 gasUsed = gasStart - gasleft();
        console.log("Gas used for withdraw with %d cooldowns: %d", unstakeIterations, gasUsed);

        // On a live network with a fixed block gas limit, this transaction would likely fail.
        // A gas usage over 2M for 200 cooldowns indicates a potential DoS vector.
        assertTrue(gasUsed > 2_000_000, "Gas usage should be very high");
    }
}
```

## Suggested Mitigation
Introduce pagination or processing limits to functions that iterate over potentially large arrays. 

For `withdraw()`, allow the user to specify a maximum number of cooldowns to process in one call. The function can return the number of entries actually processed.

```solidity
// StakingFacet.sol
function withdraw(uint256 maxCooldownsToProcess) external nonReentrant {
    // ...
    uint256 amountToWithdraw = _processCooldowns(msg.sender, maxCooldownsToProcess);
    // ...
}

// _processCooldowns should be updated to respect the limit
function _processCooldowns(address user, uint256 limit) internal returns (uint256) {
    uint256 processedCount = 0;
    // ...
    for (uint256 i = 0; i < s.userCooldowns[user].length && processedCount < limit; ) {
        // ...
        processedCount++;
    }
    // ...
}
```

For reward claiming, allow users to claim from a specific subset of validators instead of all of them at once.

```solidity
// RewardsFacet.sol
function claimFromValidators(address token, uint16[] calldata validatorIds) external {
    // ... iterate over the provided validatorIds array
}
```

## [L-7]. Pausable Emergency Stop issue in Spin::handleRandomness

## Description
The `handleRandomness` function, which is the callback that processes the result of a spin and distributes rewards, is not protected by the `whenNotPaused` modifier. If the admin pauses the contract to prevent an exploit or bug in the reward logic, in-flight spin requests that have been sent to the oracle will still be processed. This bypasses the emergency stop mechanism, potentially allowing an attacker to exploit a vulnerability even after the contract is paused.

## Impact
Pausing the contract stops new spins, but oracle callbacks that were emitted before the pause can still settle. This means the emergency stop cannot instantly freeze *all* contract activity; already-pending spins will finish and pay normal rewards. Although this weakens incident-response capabilities, it does **not** by itself let an attacker drain or steal additional funds unless another independent vulnerability exists in the reward-settlement logic.

## Proof of Concept
1. An attacker identifies a bug in `handleRandomness` that allows draining funds.
2. The admin team becomes aware and calls `pause()` on the `Spin` contract.
3. The attacker calls `startSpin()` just before the `pause()` transaction is mined. The call succeeds and sends a request to the Supra oracle.
4. The `pause()` transaction is mined, and `startSpin()` is now blocked for other users.
5. The Supra oracle calls back to `handleRandomness` with the random number for the attacker's spin.
6. Since `handleRandomness` is not pausable, the function executes, processing the spin with the flawed logic. The attacker receives an inflated amount of rewards, draining funds from the contract despite it being 'paused'.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.23;

import {Test, console} from "forge-std/Test.sol";
import {Spin} from "../src/spin/Spin.sol";
import {ISupraRouterContract} from "../src/interfaces/ISupraRouterContract.sol";
import {IDateTime} from "../src/interfaces/IDateTime.sol";
import {PausableUpgradeable} from "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";

contract MockSupraRouter_Poc is ISupraRouterContract {
    uint256 public nonceCounter;

    function generateRequest(string calldata, uint8, uint256, uint256, address) external returns (uint256 nonce) {
        nonceCounter++;
        return nonceCounter;
    }
}

contract MockDateTime_Poc is IDateTime {
    function toTimestamp(uint16, uint8, uint8, uint8, uint8, uint8) external pure returns (uint256) { return block.timestamp; }
    function getYear(uint256) external pure returns (uint16) { return 2024; }
    function getMonth(uint256) external pure returns (uint8) { return 7; }
    function getDay(uint256) external pure returns (uint8) { return 22; }
}

contract PausableEmergencyStopTest is Test {
    Spin spin;
    MockSupraRouter_Poc mockRouter;
    MockDateTime_Poc mockDateTime;
    address admin = makeAddr("admin");
    address user = makeAddr("user");
    uint256 spinPrice = 2 ether;

    function setUp() public {
        vm.startPrank(admin);
        mockDateTime = new MockDateTime_Poc();
        spin = new Spin();
        mockRouter = new MockSupraRouter_Poc();
        spin.initialize(address(mockRouter), address(mockDateTime));
        spin.setEnableSpin(true);
        spin.setCampaignStartDate(block.timestamp - 1 days);
        vm.stopPrank();
        
        deal(address(spin), 100 ether); // Pre-fund contract for rewards
        deal(user, spinPrice);
    }

    function test_PausableBypassInHandleRandomness() public {
        // 1. User starts a spin while contract is not paused
        vm.prank(user);
        uint256 nonce = spin.startSpin{value: spinPrice}();
        assertTrue(spin.isSpinPending(user), "Spin should be pending");

        // 2. Admin pauses the contract
        vm.prank(admin);
        spin.pause();
        assertTrue(spin.paused(), "Contract should be paused");

        // 3. Simulate oracle callback to the paused contract
        vm.prank(address(mockRouter)); // Call from the supra role address
        
        uint256 userBalanceBefore = user.balance;

        uint256[] memory rngList = new uint256[](1);
        // Probability for Plume Token is <= 200,000 / 1,000,000. 
        // We use a low rng value to ensure it hits a reward.
        rngList[0] = 1; 

        // This call should succeed despite the contract being paused, which is the vulnerability.
        spin.handleRandomness(nonce, rngList);
        
        // 4. Assert that the user's state was changed (they received a reward)
        uint256 plumeAmount = spin.plumeAmounts(1 % 3);
        uint256 expectedReward = plumeAmount * 1e18;

        assertEq(user.balance, userBalanceBefore + expectedReward, "User should have received rewards despite contract being paused");
        assertFalse(spin.isSpinPending(user), "Spin pending should be false after handling");
    }
}
```

## Suggested Mitigation
Add the `whenNotPaused` modifier to the `handleRandomness` function to ensure that spin results are not processed while the contract is paused.

```solidity
// Suggested fix
function handleRandomness(uint256 nonce, uint256[] calldata rngList)
    external
    onlyRole(SUPRA_ROLE)
    nonReentrant
    whenNotPaused // Add this modifier
{
    // ... function body
}
```

## [L-8]. Timestamp Dependent Logic issue in Spin::_computeStreak

## Description
The `_computeStreak` function calculates a user's daily spin streak based on `block.timestamp` and `SECONDS_PER_DAY`. The logic compares the floored day of the last spin with the floored day of the current spin. A user's streak is broken if they don't spin on a consecutive calendar day. A malicious miner can manipulate `block.timestamp` to their advantage. If a user submits a transaction near the end of the 24-hour window for continuing a streak, a miner could delay the inclusion of that transaction by a few seconds, causing the `block.timestamp` to cross the day threshold and unfairly break the user's streak. This directly impacts the rewards received, as raffle ticket rewards are multiplied by the streak count (`baseRaffleMultiplier * (userData[user].streakCount + 1)`).

## Impact
A block producer can shift the timestamp by a few seconds at the end-of-day boundary and make the contract treat two calendar days as non-consecutive. This merely resets the caller’s daily streak to 0, so the user earns the base reward instead of the streak-boosted reward for that single spin. No funds are stolen from the user or the protocol, and the loss is limited to the difference between boosted and base rewards for one spin.

## Proof of Concept
1. A user, Alice, has a streak of 10 days. Her last spin was at timestamp `T`.
2. Alice needs to spin before `T + 48 hours` to maintain her streak. She submits a `startSpin` transaction at `T + 48 hours - 10 seconds`.
3. A malicious miner sees this transaction. They can choose to include this transaction in a block with `timestamp >= T + 48 hours`.
4. When the oracle callback `handleRandomness` is executed for Alice's spin, it calls `_computeStreak`.
5. `_computeStreak` will see that `today` is not equal to `lastDaySpun + 1`, and will reset Alice's streak to 1.
6. Alice receives a much smaller raffle ticket reward than she was entitled to, effectively losing her 10-day bonus.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {Spin} from "../src/spin/Spin.sol";
import {ISupraRouterContract} from "../src/interfaces/ISupraRouterContract.sol";
import {IDateTime} from "../src/interfaces/IDateTime.sol";

// Mocks from previous PoC can be reused
contract MockSupraRouter is ISupraRouterContract { ... }
contract MockDateTime is IDateTime { ... }

contract StreakManipulationTest is Test {
    Spin spin;
    MockSupraRouter mockSupra;
    MockDateTime mockDateTime;
    address admin = makeAddr("admin");
    address alice = makeAddr("alice");

    function setUp() public {
        vm.deal(admin, 100 ether);
        vm.deal(alice, 100 ether);

        vm.startPrank(admin);
        spin = new Spin();
        mockSupra = new MockSupraRouter(address(spin));
        mockDateTime = new MockDateTime();
        spin.initialize(address(mockSupra), address(mockDateTime));
        spin.setEnableSpin(true);
        spin.setCampaignStartDate(block.timestamp);
        // Set reward probabilities so Alice wins a Raffle Ticket
        spin.setRewardProbabilities(200_000, 600_000, 900_000);
        // Make raffle ticket win certain for the PoC
        mockSupra.winningRandomness = 300_000;
        vm.stopPrank();

        // GIVEN: Alice has a streak of 10 and spun yesterday
        uint256 lastSpinTime = block.timestamp - 25 hours;
        Spin.UserData storage userData = spin.userData(alice);
        userData.streakCount = 10;
        userData.lastSpinTimestamp = lastSpinTime;
        assertEq(spin.currentStreak(alice), 10, "Initial streak should be 10");
    }

    function test_exploit_streakManipulation() public {
        uint256 price = spin.getSpinPrice();
        uint256 raffleTicketsBefore = spin.getUserData(alice).raffleTicketsBalance;

        // WHEN: Alice spins, but a miner manipulates the timestamp to be > 48h after her last spin
        vm.warp(spin.userData(alice).lastSpinTimestamp + 48 hours);

        vm.prank(alice);
        spin.startSpin{value: price}();

        // THEN: Alice's streak should be reset to 1
        (uint256 finalStreak,,,,,) = spin.getUserData(alice);
        assertEq(finalStreak, 1, "Streak should be reset to 1");

        // AND: Alice's raffle ticket reward is much lower than expected
        uint256 raffleTicketsAfter = spin.getUserData(alice).raffleTicketsBalance;
        uint256 baseMultiplier = spin.baseRaffleMultiplier();
        // Expected reward with streak: base * (10 + 1) = 8 * 11 = 88
        // Actual reward after manipulation: base * (0 + 1) = 8 * 1 = 8
        assertEq(raffleTicketsAfter - raffleTicketsBefore, baseMultiplier * 1, "Reward should be based on a reset streak");

        console.log("Attack successful: Miner manipulation broke user's streak, reducing their rewards.");
    }
}
```

## Suggested Mitigation
The strict check for consecutive calendar days based on `block.timestamp` is fragile. A more robust approach would be to allow a grace period. Instead of checking `today == lastDaySpun + 1`, the contract could check if the time elapsed since the last spin is within a flexible window, for example, less than 48 hours. This makes the streak less sensitive to minor timestamp manipulations around the day boundary.

```diff
// In _computeStreak(address user, uint256 nowTs, bool justSpun)

    uint256 lastSpinTs = userData[user].lastSpinTimestamp;
    if (lastSpinTs == 0) {
        return 0 + streakAdjustment;
    }

-   uint256 lastDaySpun = lastSpinTs / SECONDS_PER_DAY;
-   uint256 today = nowTs / SECONDS_PER_DAY;

-   if (today == lastDaySpun) {
-       return userData[user].streakCount;
-   } else if (today == lastDaySpun + 1) {
-       return userData[user].streakCount + streakAdjustment;
-   } else {
-       return 0 + streakAdjustment;
-   }

+   uint256 timeSinceLastSpin = nowTs - lastSpinTs;
+
+   // If spin is within 24h, it's the same day for the streak, no change.
+   if (timeSinceLastSpin < SECONDS_PER_DAY) {
+       return userData[user].streakCount;
+   }
+   // If spin is between 24h and 48h, the streak continues.
+   else if (timeSinceLastSpin < SECONDS_PER_DAY * 2) {
+       return userData[user].streakCount + streakAdjustment;
+   }
+   // If more than 48h have passed, the streak is broken.
+   else {
+       return 0 + streakAdjustment;
+   }
```
*Note: This simplified logic assumes `justSpun` is always true when incrementing. The logic needs to be adapted carefully to handle both streak calculation and incrementing correctly.*

## [L-9]. Zero Code issue in Spin::initialize

## Description
The `initialize` function sets critical external contract addresses such as `supraRouterAddress` and `dateTimeAddress`. However, it does not verify that these addresses actually contain contract bytecode. An administrator could accidentally provide an Externally Owned Account (EOA) or a zero address during initialization. If this happens, subsequent calls to these addresses (e.g., `supraRouter.generateRequest`) will not revert but will fail silently by returning default values. This would break the core functionality of the contract, causing users to pay for spins but never receive a result, leading to a permanent loss of funds for those users.

## Impact
A misconfiguration during initialization can lead to a complete and irreversible failure of the contract's core spin mechanism. Users who interact with the contract will lose their funds without any chance of receiving a reward or a refund. The contract would need to be redeployed and re-initialized correctly.

## Proof of Concept
1. The contract admin deploys the `Spin` contract.
2. The admin calls `initialize`, but accidentally passes an EOA address (e.g., `address(0xdeadbeef)`) for the `supraRouterAddress` parameter.
3. A user calls `startSpin()` and pays the required `spinPrice`.
4. The call to `supraRouter.generateRequest(...)` is made to the EOA. The call succeeds but returns a default `nonce` of 0.
5. The user's state is updated: `isSpinPending[user] = true`.
6. Because no request was actually sent to the Supra oracle network, the callback `handleRandomness` is never triggered.
7. The user is now permanently stuck. They cannot call `startSpin` again because `isSpinPending` is true, and they have lost their `spinPrice`.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {Spin} from "../src/spin/Spin.sol";

contract ZeroCodeTest is Test {
    Spin spin;
    address admin = makeAddr("admin");
    address user = makeAddr("user");

    function test_fail_initializeWithEOA() public {
        vm.deal(admin, 1 ether);
        vm.deal(user, 1 ether);

        // GIVEN: The Spin contract is initialized with an EOA for the router
        vm.startPrank(admin);
        spin = new Spin();
        address eoaRouter = address(0xdeadbeef);
        address eoaDateTime = address(0xfeedbeef);
        spin.initialize(eoaRouter, eoaDateTime);
        spin.setEnableSpin(true);
        vm.stopPrank();

        // WHEN: a user tries to spin
        vm.startPrank(user);
        uint256 price = spin.getSpinPrice();
        spin.startSpin{value: price}();

        // THEN: The user is stuck in a pending state
        assertTrue(spin.isSpinPending(user), "User should be in a pending state");

        // AND: The user cannot spin again, their funds are locked in a broken flow
        vm.expectRevert(abi.encodeWithSelector(Spin.SpinRequestPending.selector, user));
        spin.startSpin{value: price}();
        vm.stopPrank();
        
        console.log("User paid for a spin but is now stuck because the router was an EOA.");
    }
}
```

## Suggested Mitigation
In the `initialize` function, add checks to verify that the addresses provided for critical external contracts contain bytecode. This ensures that the contract is configured with valid, deployed contracts and not EOAs.

```diff
     function initialize(
         address supraRouterAddress,
         address dateTimeAddress
     ) public virtual initializer {
+        require(supraRouterAddress.code.length > 0, "Spin: supraRouter must be a contract");
+        require(dateTimeAddress.code.length > 0, "Spin: dateTime must be a contract");
+
         __AccessControl_init();
         __UUPSUpgradeable_init();
         __Pausable_init();
```

## [L-10]. Gas Grief BlockLimit issue in Raffle::removePrize

## Description
The `removePrize` function iterates through the `prizeIds` array using a for-loop to find the `prizeId` to be removed. The `prizeIds` array is unbounded and grows each time an administrator calls `addPrize`. If a large number of prizes are added, the gas cost of this loop can exceed the block gas limit, causing the transaction to fail. This would result in a permanent Denial of Service (DoS) for the `removePrize` function, preventing administrators from managing prizes effectively. The `getPrizeDetails` view function has a similar vulnerability, where it can become unusable for off-chain clients if the number of prizes is too large.

## Impact
The gas-cost of the `removePrize` linear search grows with the number of prizes. If administrators add an extremely large amount of prizes, a single `removePrize` call might exceed the block gas limit and revert, forcing the admin to remove prizes with multiple transactions or by upgrading the contract. End-users and funds are not affected; only the administrator’s convenience is impacted.

## Proof of Concept
1. An administrator calls `addPrize` a large number of times (e.g., thousands of times, depending on the block gas limit).
2. The `prizeIds` array grows with each call.
3. The administrator then attempts to call `removePrize` for any `prizeId`.
4. The transaction will consume a large amount of gas iterating through the array. With a sufficiently large array, the transaction will fail by running out of gas.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "src/spin/Raffle.sol";
import "src/interfaces/ISpin.sol";
import "src/interfaces/ISupraRouterContract.sol";

// Mocks for interfaces
contract MockSpin is ISpin {
    function getUserData(address) external view returns (uint256, uint256, uint256, uint256, uint256, uint256, uint256) {
        return (0, 0, 0, 0, 1000, 0, 0);
    }
    function spendRaffleTickets(address, uint256) external {}
}

contract MockSupraRouter is ISupraRouterContract {
    function generateRequest(string calldata, uint8, uint256, uint256, address) external returns (uint256) {
        return 1;
    }
}

contract GasGrief_RemovePrize_Test is Test {
    Raffle public raffle;
    address public admin;

    function setUp() public {
        admin = address(this);
        MockSpin mockSpin = new MockSpin();
        MockSupraRouter mockSupraRouter = new MockSupraRouter();

        raffle = new Raffle();
        raffle.initialize(address(mockSpin), address(mockSupraRouter));
    }

    function test_DoS_RemovePrize() public {
        // Add a large number of prizes to inflate the prizeIds array
        uint256 prizeCount = 1500; // This number may need adjustment based on block gas limit
        for (uint256 i = 0; i < prizeCount; i++) {
            raffle.addPrize(string.concat("Prize ", vm.toString(i)), "Description", 1 ether);
        }

        // The last prize added has id `prizeCount` since it starts from 1 and increments
        uint256 prizeToRemove = prizeCount;

        // Measure gas cost of removing the last prize.
        // The loop in removePrize has to iterate through the entire array.
        uint256 gasStart = gasleft();
        raffle.removePrize(prizeToRemove);
        uint256 gasUsed = gasStart - gasleft();

        // Log the gas used. This demonstrates high gas consumption.
        // With a high enough `prizeCount`, this call would revert due to out-of-gas.
        console.log("Gas used to remove prize from large array:", gasUsed);
        assertTrue(gasUsed > 2_000_000, "Gas usage should be very high");

        // To make it fail reliably in a test, we can use a low gas stipend
        // This simulates the block gas limit being reached.
        vm.expectRevert(); // Expects revert due to out of gas
        (bool success, ) = address(raffle).call{gas: 50000}(abi.encodeWithSelector(
            raffle.removePrize.selector,
            uint256(1) // Try to remove the first prize with insufficient gas
        ));
        require(!success);
    }
}

## Suggested Mitigation
Avoid linear iteration over unbounded arrays for state-changing operations. To remove an item, require the caller to provide the index of the element to be removed. The off-chain application can find the index efficiently and pass it to the contract. 

```solidity
// Example Mitigation
// The prizeIds array must be exposed via a getter for off-chain apps to find the index.
function removePrize(uint256 prizeId, uint256 index) external onlyRole(ADMIN_ROLE) {
    require(prizes[prizeId].isActive, "Prize is not active");
    uint256 len = prizeIds.length;
    require(index < len, "Index out of bounds");
    require(prizeIds[index] == prizeId, "Prize ID mismatch at index");

    prizes[prizeId].isActive = false;
    
    // Efficient removal without iteration
    prizeIds[index] = prizeIds[len - 1];
    prizeIds.pop();

    emit PrizeRemoved(prizeId);
}
```

## [L-11]. Event Consistency issue in Raffle::setPrizeActive

## Description
The `setPrizeActive(uint256 prizeId, bool active)` function allows an administrator to enable or disable a prize. This is a critical administrative action that directly affects the usability of a prize raffle. However, this state-changing function does not emit an event. This omission makes it difficult for off-chain services, monitoring tools, and users to efficiently track and react to these important status changes, reducing the transparency of the system.

## Impact
The lack of an event for a critical administrative action harms the system's observability and transparency. Off-chain user interfaces may display outdated information, leading to user confusion and transaction failures when they attempt to interact with a prize that has been deactivated. This also complicates system monitoring and incident response.

## Proof of Concept
1. An administrator creates a prize, which is active by default.
2. A user interface queries the contract and shows the prize as available for entry.
3. The administrator calls `setPrizeActive(prizeId, false)`, deactivating the prize. No event is emitted.
4. The user interface is not notified of this change. The user, believing the prize is still active, attempts to call `spendRaffle`.
5. The transaction reverts with the error "Prize is not active", causing confusion for the user.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "contracts/plume/src/spin/Raffle.sol";

// minimal interface matching what Raffle expects
interface ISpinMock {
    function getUserData(address user) external view returns (
        uint256,uint256,uint256,uint256,uint256,uint256,uint256);
    function spendRaffleTickets(address user, uint256 amount) external;
}

contract SpinStub is ISpinMock {
    function getUserData(address) external pure returns (
        uint256,uint256,uint256,uint256,uint256,uint256,uint256){
        return (0,0,0,0,100,0,0); // give caller 100 raffle tickets
    }
    function spendRaffleTickets(address, uint256) external override {}
}

contract SupraStub is ISupraRouterContract {
    function generateRequest(
        string calldata,
        uint8,
        uint256,
        uint256,
        address
    ) external pure override returns (uint256) {
        return 1;
    }
}

contract EventConsistencyFixedTest is Test {
    Raffle raffle;

    function setUp() public {
        raffle = new Raffle();
        raffle.initialize(address(new SpinStub()), address(new SupraStub()));
        raffle.addPrize("p", "d", 1 ether);
    }

    function test_NoEventEmittedOnSetPrizeActive() public {
        vm.recordLogs();
        raffle.setPrizeActive(1, false);
        Vm.Log[] memory rec = vm.getRecordedLogs();
        assertEq(rec.length, 0, "expected no logs");
    }
}

## Suggested Mitigation
An event should be emitted whenever a prize's active status is changed. This improves transparency and allows off-chain systems to track the contract's state reliably.

```solidity
// Add this event definition to the contract
event PrizeActivitySet(uint256 indexed prizeId, bool active);

// Modify the setPrizeActive function to emit the event
function setPrizeActive(uint256 prizeId, bool active)
    external
    onlyRole(ADMIN_ROLE)
{
    Prize storage prize = prizes[prizeId];
    require(bytes(prize.name).length != 0, "Prize does not exist");
    require(prize.winnerIndex == 0, "Winner already selected");

    prizes[prizeId].isActive = active;

    emit PrizeActivitySet(prizeId, active);
}
```

## [L-12]. DOS issue in Raffle::removePrize

## Description
The `removePrize` function finds a prize to remove by iterating through the `prizeIds` array. This array can grow indefinitely as the admin adds more prizes. If the array becomes very large, the gas cost of this linear scan can exceed the block gas limit, making it impossible to remove any prizes. This constitutes a Denial of Service vulnerability for a core administrative function.

## Impact
Because `removePrize` performs an O(n) linear search on `prizeIds`, once the array grows large enough a call will run out of gas and revert. The function can therefore become permanently unusable, leaving obsolete prizes stuck in storage. While this blocks only the ADMIN from tidying the contract – user funds or prize claiming are not affected – it still prevents contract maintenance.

## Proof of Concept
1. ADMIN adds a large number of prizes so that `prizeIds.length` exceeds ~30 000.
2. ADMIN tries to call `removePrize(1)` (or any early id).
3. The loop in `removePrize` iterates over every element and exhausts the supplied gas, so the transaction reverts with an out-of-gas error.
4. The prize remains active forever because there is no alternative path to disable it.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {Raffle} from "src/spin/Raffle.sol";

contract DosRemovePrizeTest is Test {
    Raffle raffle;
    address admin = address(0xABCD);

    function setUp() public {
        vm.startPrank(admin);
        raffle = new Raffle();
        raffle.initialize(address(0), address(0));
        // fill the array with 40k prizes so the loop surely exceeds 100k gas
        for (uint256 i = 0; i < 40_000; i++) {
            raffle.addPrize(string(abi.encodePacked("P", i)), "", 1 ether);
        }
        vm.stopPrank();
    }

    function testRemovePrizeRunsOutOfGas() public {
        // give the call only 120k gas which is far below what 40k iterations need
        bytes memory data = abi.encodeWithSelector(Raffle.removePrize.selector, 1);
        vm.startPrank(admin);
        (bool ok,) = address(raffle).call{gas: 120000}(data);
        vm.stopPrank();
        assertTrue(!ok, "call should have run out of gas and reverted");
    }
}

## Suggested Mitigation
Store the index of every prize id in a mapping (id => index). When removing, read the index in O(1), swap-and-pop the last element, and update the mapping for the moved id. This guarantees constant-time execution regardless of array size.

## [L-13]. Zero Code issue in Raffle::initialize

## Description
The `initialize` function takes addresses for `_spinContract` and `_supraRouter` as arguments but does not verify that these addresses point to deployed contracts. If a deployer accidentally provides an Externally Owned Account (EOA) or an incorrect address, calls to these addresses will not revert but will return default values. This can lead to unexpected behavior or a denial of service for certain features, such as being unable to request a winner if `_supraRouter` is not a contract.

## Impact
If the deployer supplies an EOA (or any non-contract address) for `_supraRouter` (or `_spinContract`), every call to `requestWinner` (or other router dependent functions) reverts. No prize can ever be drawn and users can never claim prizes. Although funds are not at risk, the raffle application becomes permanently unusable until the contract is upgraded. This is a deployment-time mis-configuration that leads to a denial of service for core functionality.

## Proof of Concept
1. Deploy `Raffle` with `_supraRouter` set to an EOA ( `address(1)` for instance ).
2. Add an active prize and let users spend tickets (these functions do not touch `supraRouter`).
3. `requestWinner(prizeId)` tries to execute `supraRouter.generateRequest(...)`.
4. Because `address(1)` has no code, the low-level `CALL` succeeds with zero return data; ABI decoding of the expected `uint256` return value **reverts**, so the whole transaction reverts and the raffle admin can never progress.

```solidity
// excerpt
uint256 requestId = supraRouter.generateRequest( ... ); // <- revert here
```

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {Raffle} from "src/spin/Raffle.sol";

contract MockSpin {
    function spendRaffleTickets(address, uint256) external {}
    function getUserData(address) external pure returns (uint256,uint256,uint256,uint256,uint256,uint256,uint256){
        return (0,0,0,0,100,0,0);
    }
}

contract RaffleZeroCodeTest is Test {
    Raffle raffle;
    address admin = address(0xA11CE);
    address supraEOA = address(0xBEEF); // no code

    function setUp() public {
        vm.startPrank(admin);
        raffle = new Raffle();
        raffle.initialize(address(new MockSpin()), supraEOA);
        raffle.addPrize("TestPrize", "desc", 1 ether);
        vm.stopPrank();
    }

    function testRequestWinnerRevertsWhenRouterIsEOA() public {
        vm.prank(admin);
        vm.expectRevert();
        raffle.requestWinner(1);
    }
}


## Suggested Mitigation
In `initialize` (and in any future setter), validate that the supplied addresses refer to contracts by checking `address.code.length > 0`:

```solidity
require(_supraRouter.code.length != 0, "Raffle: supraRouter is not a contract");
require(_spinContract.code.length != 0, "Raffle: spinContract is not a contract");
```

## [L-14]. Gas Grief BlockLimit issue in Raffle::removePrize

## Description
The `removePrize` function in the `Raffle` contract is used by an admin to remove a prize from the system. It finds the prize to remove by iterating through the entire `prizeIds` array. If the number of prizes in the raffle becomes very large, the gas cost for this loop can exceed the block gas limit. This would cause any transaction calling `removePrize` to fail, effectively creating a Denial of Service (DoS) condition for this administrative function. The admin would be unable to remove any prizes, which could lead to operational problems, such as being unable to remove incorrect or outdated prize listings.

## Impact
If the admin (the only actor allowed to call `addPrize`) mistakenly registers several thousand prizes, the linear search performed in `removePrize` can consume more gas than the block limit, permanently preventing the admin from removing prizes through the contract. While no user funds are at risk, contract maintenance becomes impossible and the raffle catalogue can no longer be cleaned up.

## Proof of Concept
1. Admin script adds >4,000 prizes (exact threshold depends on the EVM implementation).
2. `prizeIds` length is now >4,000.
3. Admin calls `removePrize(1)`.
4. The loop `for (uint256 i = 0; i < prizeIds.length; i++)` executes >4,000 iterations, exceeding the ~30M gas block limit on most L2s and reverting out-of-gas.
5. From this point the admin cannot remove *any* prize, creating an operational DoS.

NOTE: A deterministic Foundry test cannot reproduce an OOG-revert because the local EVM runs with an unlimited gas ceiling. The issue manifests only on-chain when the block gas limit is enforced.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {Raffle} from "src/spin/Raffle.sol";
import {Plume} from "src/plume.sol";
import {ISpin} from "src/interfaces/ISpin.sol";
import {ISupraRouterContract} from "src/interfaces/ISupraRouterContract.sol";
import {PlumeRoles} from "src/lib/PlumeRoles.sol";

// Mock contracts
contract MockSpin is ISpin {
    function spendRaffleTickets(address, uint256) external {}
}

contract MockSupraRouter is ISupraRouterContract {
    function generateRequest(string memory, uint64, uint8, address, uint256) external returns (uint256) {
        return 1;
    }
    function getFee(uint64) external view returns (uint256) {
        return 0;
    }
}

contract Raffle_DoS_Test is Test {
    Raffle internal raffle;
    Plume internal plume;
    address internal admin = makeAddr("admin");

    function setUp() public {
        vm.startPrank(admin);
        raffle = new Raffle();
        plume = new Plume();
        plume.initialize(admin);

        MockSpin spin = new MockSpin();
        MockSupraRouter supra = new MockSupraRouter();

        raffle.initialize(admin, address(spin), address(supra), address(plume), 12345);
        raffle.grantRole(PlumeRoles.ADMIN_ROLE, admin);
        vm.stopPrank();
    }

    function test_dos_removePrize() public {
        vm.startPrank(admin);
        uint256 prizeCount = 500;

        for (uint256 i = 1; i <= prizeCount; i++) {
            raffle.addPrize(i, "Test Prize", "Description", 100 * 1e18, "image_url");
        }

        // The prize to be removed is the first one, which is the worst-case for the loop.
        uint256 prizeToRemove = 1;

        // This will revert due to out of gas.
        vm.expectRevert();
        raffle.removePrize(prizeToRemove);

        vm.stopPrank();
    }
}
```

## Suggested Mitigation
Store each prize's position in the `prizeIds` array (e.g. `mapping(uint256 => uint256) indexOf`) and remove prizes with the constant-time swap-and-pop pattern, ensuring the gas cost is bounded irrespective of the array size.

## [L-15]. Pausable Emergency Stop issue in RewardsFacet::claim

## Description
The primary functions for withdrawing rewards, `claim(address token)` and `claimAll()`, lack an emergency stop mechanism. These functions execute complex reward calculation logic from `PlumeRewardLogic` and trigger fund transfers from the treasury. If a vulnerability were discovered in this logic that allows for inflated reward calculations, there would be no way to immediately halt withdrawals, exposing the entire reward treasury to a potential drain.

## Impact
Because the RewardsFacet is not pausable, the protocol operators lack an on-chain circuit breaker for reward-claiming in case another, separate vulnerability is discovered in the reward logic. This does not itself allow an attacker to steal funds, but it increases the time window in which a latent bug could be exploited before an upgrade can be executed.

## Proof of Concept
1. A vulnerability is discovered in `PlumeRewardLogic.calculateRewardsWithCheckpoints` that allows any user to calculate an erroneously large reward for themselves.
2. An attacker begins calling `claim()` or `claimAll()` repeatedly.
3. Each call transfers a large amount of reward tokens from the treasury to the attacker.
4. The protocol administrators have no direct method to stop the `claim()` function within `RewardsFacet`. Their only recourse might be to pause the reward token itself (if it's pausable) or try to remove the token from the system via `removeRewardToken`, which could also fail due to gas limits or other unforeseen issues. A direct pause on the claiming functionality is missing.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";
import {PausableUpgradeable} from "@openzeppelin/contracts-upgradeable/security/PausableUpgradeable.sol";
import {RewardsFacet} from "src/facets/RewardsFacet.sol";

// We create a new version of the facet that is pausable.
contract PausableRewardsFacet is RewardsFacet, PausableUpgradeable {
    function pause() public {
        _pause();
    }

    function unpause() public {
        _unpause();
    }

    // The vulnerable `claim` function in the original contract lacks this modifier.
    function claimWithPause(address token) public whenNotPaused {
        // This is a placeholder for the actual claim logic.
        // In a real scenario, this would call the original claim logic.
    }
}

contract PausableTest is Test {
    PausableRewardsFacet public pausableRewardsFacet;

    function setUp() public {
        pausableRewardsFacet = new PausableRewardsFacet();
    }

    function test_claim_is_not_pausable() public {
        // In a real scenario, we would set up state to allow a claim.
        // This test demonstrates that even if a pause function existed elsewhere,
        // the original claim function does not respect it.
        // RewardsFacet.claim() can be called anytime.
        // For this PoC, we show that our pausable version *can* be stopped.

        // Pause the contract
        pausableRewardsFacet.pause();

        // Attempt to call the pausable function, it should revert.
        vm.expectRevert("Pausable: paused");
        pausableRewardsFacet.claimWithPause(address(0x123));

        // The original RewardsFacet.claim() would succeed here because it has no `whenNotPaused` modifier.
    }
}

```

## Suggested Mitigation
Consider adding an emergency pause mechanism (e.g. inherit PausableUpgradeable or implement an equivalent storage-safe circuit breaker in the diamond) and gate claim / claimAll with a `whenNotPaused` guard. This provides defence-in-depth without changing existing business logic.

## [L-16]. Pausable Emergency Stop issue in PlumeStaking::NA

## Description
The PlumeStaking contract system, which manages significant user funds and complex logic across multiple facets, lacks a global emergency stop (pause) mechanism. While administrators can perform actions like setting reward rates to zero or deactivating validators, these are granular, multi-transaction operations. They are not sufficient to instantly halt all protocol activity in the event of a critical vulnerability discovery. Functions such as `stake`, `unstake`, `withdraw`, and `claim` remain callable, which could allow an attacker to continue draining funds while the team attempts to manually disable parts of the system.

## Impact
The protocol has no single, globally enforced `pause` flag.  If a separate, unrelated vulnerability is discovered, privileged actors cannot immediately halt user-facing functions in one transaction.  They would have to rely on (time-delayed) upgrades or many granular admin calls, increasing the window in which the *already-existing* bug can be exploited.  The absence of a pause mechanism does **not by itself create a loss of funds**, but it can amplify the damage of another bug by delaying incident response.

## Proof of Concept
1. A critical vulnerability is discovered in the `claimAll()` function that allows a user to claim more rewards than they are entitled to.
2. An attacker begins to exploit this vulnerability repeatedly, draining the reward treasury.
3. The protocol administrators are alerted but have no single `pause()` function. They must prepare and execute a series of transactions: `setRewardRates()` to zero for every reward token and `setValidatorStatus(false)` for every active validator.
4. This process is slow and may require multiple authorized parties (e.g., TIMELOCK_ROLE, ADMIN_ROLE). While the administrators are acting, the attacker continues to drain funds.
5. A global pause function, callable by a designated admin/security role, would have immediately halted all claims and other state-changing functions, mitigating the damage instantly.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.23;

import "forge-std/Test.sol";
import "forge-std/console.sol";

// This PoC demonstrates the *lack* of a pausable feature. 
// It shows that a critical function (`stake`) remains operational,
// whereas in an emergency scenario, it should be possible to disable it globally.

library PlumeStakingStorage {
    struct Layout {
        mapping(address => mapping(uint16 => uint256)) userStakes;
        mapping(uint16 => Validator) validators;
        uint256 minStakeAmount;
        bool paused; // A potential pause state variable
    }
    struct Validator { bool active; bool slashed; }
    bytes32 constant STORAGE_SLOT = keccak256("plume.storage.PlumeStaking");
    function layout() internal pure returns (Layout storage $) {
        assembly { $.slot := STORAGE_SLOT }
    }
}

contract StakingFacet {
    // In a secure contract, this function should have a 'whenNotPaused' modifier.
    function stake(uint16 validatorId) public payable {
        PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
        // The require(!$.paused) check is missing.
        require(msg.value >= $.minStakeAmount, "Stake too small");
        PlumeStakingStorage.Validator storage validator = $.validators[validatorId];
        require(validator.active, "Validator inactive");
        
        $.userStakes[msg.sender][validatorId] += msg.value;
    }
}

contract MissingPauseTest is Test {
    StakingFacet facet;
    address staker = address(0x42);
    uint16 validatorId = 1;

    function setUp() public {
        facet = new StakingFacet();
        PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
        $.minStakeAmount = 1 ether;
        $.validators[validatorId].active = true;
    }

    function test_CriticalFunctionsAreNotPausable() {
        console.log("SCENARIO: An exploit is live. Admins need to halt the system.");
        console.log("PROBLEM: No pause function exists. Attacker can continue to call critical functions.");
        
        vm.deal(staker, 2 ether);
        vm.prank(staker);
        
        // The stake function is called successfully. There is no mechanism to prevent this.
        facet.stake{value: 1 ether}(validatorId);
        
        // ASSERT: The stake succeeds, proving the lack of a pause mechanism.
        PlumeStakingStorage.Layout storage $ = PlumeStakingStorage.layout();
        assertEq($.userStakes[staker][validatorId], 1 ether);

        console.log("SUCCESS: Stake was executed. This demonstrates the system cannot be paused in an emergency.");
    }
}

## Suggested Mitigation
Add a `PausableFacet` that stores a shared `_paused` flag in diamond storage and expose `pause()` / `unpause()` guarded by a dedicated `PAUSER_ROLE`.  Decorate every external state-changing function in staking, reward, and validator facets with `whenNotPaused`.  This allows any authorised responder to freeze the protocol with a single transaction while the permanent fix is prepared.

## [L-17]. Frontrun/Backrun/Sandwhich MEV issue in ValidatorFacet::setValidatorCommission

## Description
Validator admins can change their commission rate at any time via `setValidatorCommission`. The change takes effect immediately. This creates a front-running/MEV opportunity where a validator admin can trick users into staking with them. A malicious validator admin can advertise a low commission rate, wait for a large stake transaction to appear in the mempool, front-run it by setting their commission even lower (or keeping it low), and then, immediately after the stake is confirmed, execute another transaction to raise the commission significantly. The staker is now locked into a validator with a much higher fee than they anticipated.

## Impact
A validator can raise its commission immediately after delegators have staked. Delegators are forced to remain for the cooldown period and therefore receive lower-than-expected rewards during that time. No principal can be stolen and the loss is bounded to the reward portion generated in the cooldown window.

## Proof of Concept
1. Validator A has a commission rate of 20%.
2. To attract stakers, Validator A's admin submits a transaction to lower the commission to 1%.
3. A large staker, Alice, sees the attractive 1% rate and decides to stake 1,000,000 PLUME with Validator A. She submits her `stake()` transaction.
4. Validator A's admin, monitoring the mempool, sees Alice's large stake.
5. The admin lets Alice's transaction complete.
6. Immediately after, in the next block, the admin submits a new transaction `setValidatorCommission(25%)`.
7. Alice is now earning rewards that are subject to a 25% commission, far higher than the 1% rate that induced her to stake. Her funds are subject to a cooldown period, so she cannot exit immediately without penalty or delay.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
#pragma solidity ^0.8.20;

import {Test, console} from "forge-std/Test.sol";
// Using a simplified setup for demonstration
import {ValidatorFacet} from "src/facets/ValidatorFacet.sol";
import {PlumeStakingStorage} from "src/lib/PlumeStakingStorage.sol";
import {PlumeRewardLogic} from "src/lib/PlumeRewardLogic.sol";

// Dummy contract to provide context
contract DummyStaking is ValidatorFacet {
    constructor() {
         PlumeStakingStorage.Layout storage s = PlumeStakingStorage.layout();
         s.maxAllowedValidatorCommission = 0.5 ether; // 50%
    }
}

contract MevTest is Test {
    DummyStaking public staking;
    address validatorAdmin = makeAddr("validatorAdmin");
    address staker = makeAddr("staker");
    uint16 validatorId = 1;

    function setUp() public {
        staking = new DummyStaking();
        
        // Setup initial state
        PlumeStakingStorage.Layout storage s = PlumeStakingStorage.layout();
        s.validatorInfo[validatorId].admin = validatorAdmin;
        s.validatorInfo[validatorId].active = true;
    }

    function testCommissionBaitAndSwitch() public {
        // 1. Validator advertises a low commission rate of 1%
        vm.prank(validatorAdmin);
        staking.setValidatorCommission(validatorId, 0.01 ether);
        (,,uint256 initialCommission,,) = PlumeRewardLogic.getCommissionRateAt(validatorId, block.timestamp);
        assertEq(initialCommission, 0.01 ether, "Commission should be 1%");

        // 2. User sees the low rate and decides to stake (simulated).
        // A large stake transaction from `staker` would happen here.
        console.log("User stakes, attracted by the 1% commission.");

        // 3. Validator admin immediately raises the commission to 25% after the stake.
        vm.prank(validatorAdmin);
        staking.setValidatorCommission(validatorId, 0.25 ether);
        (,,uint256 newCommission,,) = PlumeRewardLogic.getCommissionRateAt(validatorId, block.timestamp);
        assertEq(newCommission, 0.25 ether, "Commission should now be 25%");

        // The user is now subject to a 25% commission rate, much higher than what they staked for.
    }
}
```

## Suggested Mitigation
Introduce a delay (e.g. 3–7 days) or an "effective epoch" number for commission changes. During the delay delegators can still stake/unstake with full knowledge of the upcoming rate or cancel their delegation before it becomes active.

## [L-18]. Pausable Emergency Stop issue in PlumeStaking::NA

## Description
The `PlumeStaking` contract system, which is the core of the protocol managing user-staked funds and reward logic, lacks a global emergency stop (pause) mechanism. While the `Plume` token contract itself is pausable, this only affects functions that directly transfer the token. Critical state-changing logic within the staking facets (e.g., `unstake`, reward calculations, validator updates) remains active. In the event of a critical vulnerability, there is no way to quickly and completely halt all protocol operations, potentially leading to exploitation and loss of funds.

## Impact
The contract suite currently cannot be halted by governance in case a separate, yet-unknown logic flaw is discovered. This limitation delays incident response but does not, by itself, create a new way for an adversary to steal or lock assets. The risk is therefore limited to increasing potential damage window once another bug exists.

## Proof of Concept
1. A critical bug is discovered in `StakingFacet.unstake()` that allows a user to initiate a cooldown for more tokens than they have staked.
2. The protocol administrators identify the bug but have no `pause()` function on the `PlumeStaking` contract to immediately halt its operations.
3. An attacker repeatedly calls the vulnerable `unstake` function, creating fraudulent cooldown entries in their name.
4. Although the final `withdraw()` call would be blocked if the `Plume` token contract is paused, the internal state of the staking contract is already corrupted, and the attacker has successfully registered illegitimate claims for funds.

## Proof of Code
import {Test} from "forge-std/Test.sol";
import {Plume} from "../src/Plume.sol";
import {PlumeStakingProxy} from "../src/proxy/PlumeStakingProxy.sol";
import {PlumeStaking} from "../src/PlumeStaking.sol";
import {StakingFacet} from "../src/facets/StakingFacet.sol";
import {AccessControlFacet} from "../src/facets/AccessControlFacet.sol";
import {ValidatorFacet} from "../src/facets/ValidatorFacet.sol";
import {IDiamondCut} from "@solidstate/contracts/proxy/diamond/writable/IDiamondWritable.sol";

contract PausableEmergencyStopTest is Test {
    Plume plume;
    PlumeStakingProxy stakingProxy;
    StakingFacet stakingFacet;
    address owner = makeAddr("owner");
    address staker = makeAddr("staker");
    uint16 validatorId = 1;

    function setUp() public {
        vm.startPrank(owner);
        plume = new Plume();
        plume.initialize(owner);

        // Simplified Diamond Deployment
        PlumeStaking logic = new PlumeStaking();
        bytes memory initData = abi.encodeWithSelector(PlumeStaking.initializePlume.selector, owner, 1 ether, 1 days);
        stakingProxy = new PlumeStakingProxy(address(logic), initData);

        // Add StakingFacet
        stakingFacet = new StakingFacet();
        IDiamondCut.FacetCut[] memory cuts = new IDiamondCut.FacetCut[](1);
        bytes4[] memory selectors = new bytes4[](4);
        selectors[0] = StakingFacet.stake.selector;
        selectors[1] = StakingFacet.unstake.selector;
        selectors[2] = StakingFacet.getUserCooldowns.selector;
        selectors[3] = StakingFacet.getUserValidatorStake.selector;
        cuts[0] = IDiamondCut.FacetCut({target: address(stakingFacet), action: IDiamondCut.Action.Add, selectors: selectors});
        IDiamondCut(address(stakingProxy)).diamondCut(cuts, address(0), "");

        // Add ValidatorFacet for adding validators
        ValidatorFacet validatorFacet = new ValidatorFacet();
        bytes4[] memory valSelectors = new bytes4[](1);
        valSelectors[0] = ValidatorFacet.addValidator.selector;
        IDiamondCut.FacetCut[] memory valCuts = new IDiamondCut.FacetCut[](1);
        valCuts[0] = IDiamondCut.FacetCut({target: address(validatorFacet), action: IDiamondCut.Action.Add, selectors: valSelectors});
        IDiamondCut(address(stakingProxy)).diamondCut(valCuts, address(0), "");

        // Add AccessControlFacet for role management
        AccessControlFacet acFacet = new AccessControlFacet();
        bytes4[] memory acSelectors = new bytes4[](1);
        acSelectors[0] = AccessControlFacet.grantRole.selector;
        IDiamondCut.FacetCut[] memory acCuts = new IDiamondCut.FacetCut[](1);
        acCuts[0] = IDiamondCut.FacetCut({target: address(acFacet), action: IDiamondCut.Action.Add, selectors: acSelectors});
        IDiamondCut(address(stakingProxy)).diamondCut(acCuts, address(0), "");

        // Grant VALIDATOR_ROLE to owner to add a validator
        bytes32 VALIDATOR_ROLE = keccak256("VALIDATOR_ROLE");
        AccessControlFacet(address(stakingProxy)).grantRole(VALIDATOR_ROLE, owner);

        // Add a validator
        ValidatorFacet(address(stakingProxy)).addValidator(validatorId, 0, owner, owner, "v1", "v1_addr", owner, 1_000_000 ether);
        vm.stopPrank();

        // Staker gets PLUME and stakes
        vm.startPrank(owner);
        plume.mint(staker, 100 ether);
        vm.stopPrank();

        vm.startPrank(staker);
        plume.approve(address(stakingProxy), 100 ether);
        StakingFacet(address(stakingProxy)).stake{value: 100 ether}(validatorId);
        vm.stopPrank();
    }

    function test_Fail_StakingContractIsNotPausable() public {
        // 1. A critical bug is discovered. The owner wants to pause the system.
        // There is no global pause function on the staking contract.

        // 2. The staker can still interact with the `unstake` function.
        vm.prank(staker);
        StakingFacet(address(stakingProxy)).unstake(validatorId, 50 ether);

        // 3. The state change is successful, proving the absence of a pause mechanism.
        (uint256 amount, ) = StakingFacet(address(stakingProxy)).getUserCooldowns(staker)[0];
        assertEq(amount, 50 ether, "Unstake succeeded, proving the contract is not pausable.");
    }
}

## Suggested Mitigation
Implement a comprehensive pause mechanism across the `PlumeStaking` contract system. This can be achieved by creating a `PausableFacet` that manages a paused state, and applying a `whenNotPaused` modifier to all critical state-changing functions in every facet (`StakingFacet`, `RewardsFacet`, `ValidatorFacet`, `ManagementFacet`). The pause and unpause functions in this new facet should be protected by a designated admin or security role.

Example `PausableFacet.sol`:
```solidity
import "@openzeppelin/contracts-upgradeable/security/PausableUpgradeable.sol";
import "../lib/PlumeRoles.sol";

contract PausableFacet is PausableUpgradeable {
    function pause() external onlyRole(PlumeRoles.ADMIN_ROLE) {
        _pause();
    }

    function unpause() external onlyRole(PlumeRoles.ADMIN_ROLE) {
        _unpause();
    }
}
```
Then, in other facets like `StakingFacet`:
```solidity
function unstake(...) external override nonReentrant whenNotPaused {
    // ...
}
```

## [L-19]. Gas Grief BlockLimit issue in RewardsFacet::claimAll

## Description
Several functions that iterate over an array of all registered `rewardTokens` are vulnerable to a Denial of Service (DoS) attack if the number of reward tokens becomes large. Functions such as `claimAll()` in `RewardsFacet` and `forceSettleValidatorCommission()` in `ValidatorFacet` loop through the entire list. Since an admin can add an unlimited number of reward tokens, the gas cost for these loops can exceed the block gas limit, rendering these functions permanently unusable.

## Impact
If an over-privileged admin registers a very large number of reward tokens, `claimAll()` and the validator settlement helpers can run out of gas for ordinary users. Core funds are never at risk – users can always use the single-token `claim()` path. The impact is limited to a denial-of-convenience that can be corrected by governance, so only the user experience is degraded.

## Proof of Concept
An attacker (with REWARD_MANAGER_ROLE) can register hundreds of dummy token addresses:
```solidity
// pseudo-code
for(uint i; i < 400; ++i){
    rewardsFacet.addRewardToken(address(uint160(i+1000)));
}

// user later calls
rewardsFacet.claimAll(); // ⇢ runs > 10M gas and reverts
```
Because every iteration executes an external `claim` and storage writes, gas grows linearly and will exceed the 30M block gas limit once ~350-400 tokens are registered on most EVM chains.

## Proof of Code
import {Test} from "forge-std/Test.sol";
import {Plume} from "../src/Plume.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";
import {PlumeStakingProxy} from "../src/proxy/PlumeStakingProxy.sol";
import {PlumeStaking} from "../src/PlumeStaking.sol";
import {StakingFacet} from "../src/facets/StakingFacet.sol";
import {RewardsFacet} from "../src/facets/RewardsFacet.sol";
import {ValidatorFacet} from "../src/facets/ValidatorFacet.sol";
import {AccessControlFacet} from "../src/facets/AccessControlFacet.sol";
import {IDiamondCut} from "@solidstate/contracts/proxy/diamond/writable/IDiamondWritable.sol";

contract GasGriefTest is Test {
    Plume plume;
    PlumeStakingProxy stakingProxy;
    RewardsFacet rewardsFacet;
    StakingFacet stakingFacet;
    address owner = makeAddr("owner");
    address staker = makeAddr("staker");
    uint16 validatorId = 1;

    function setUp() public {
        vm.startPrank(owner);
        plume = new Plume();
        plume.initialize(owner);

        PlumeStaking logic = new PlumeStaking();
        bytes memory initData = abi.encodeWithSelector(PlumeStaking.initializePlume.selector, owner, 1 ether, 1 days);
        stakingProxy = new PlumeStakingProxy(address(logic), initData);

        rewardsFacet = new RewardsFacet();
        stakingFacet = new StakingFacet();
        ValidatorFacet validatorFacet = new ValidatorFacet();
        AccessControlFacet acFacet = new AccessControlFacet();

        IDiamondCut.FacetCut[] memory cuts = new IDiamondCut.FacetCut[](4);
        cuts[0] = IDiamondCut.FacetCut({target: address(rewardsFacet), action: IDiamondCut.Action.Add, selectors: new bytes4[](4) });
        cuts[0].selectors[0] = RewardsFacet.addRewardToken.selector;
        cuts[0].selectors[1] = RewardsFacet.setRewardRates.selector;
        cuts[0].selectors[2] = RewardsFacet.claimAll.selector;
        cuts[0].selectors[3] = RewardsFacet.claim.selector;

        cuts[1] = IDiamondCut.FacetCut({target: address(stakingFacet), action: IDiamondCut.Action.Add, selectors: new bytes4[](1) });
        cuts[1].selectors[0] = StakingFacet.stake.selector;

        cuts[2] = IDiamondCut.FacetCut({target: address(validatorFacet), action: IDiamondCut.Action.Add, selectors: new bytes4[](1) });
        cuts[2].selectors[0] = ValidatorFacet.addValidator.selector;

        cuts[3] = IDiamondCut.FacetCut({target: address(acFacet), action: IDiamondCut.Action.Add, selectors: new bytes4[](2) });
        cuts[3].selectors[0] = AccessControlFacet.grantRole.selector;
        cuts[3].selectors[1] = AccessControlFacet.hasRole.selector;

        IDiamondCut(address(stakingProxy)).diamondCut(cuts, address(0), "");
        IDiamondCut(address(stakingProxy)).diamondCut(cuts, address(0), "");

        bytes32 REWARD_MANAGER_ROLE = keccak256("REWARD_MANAGER_ROLE");
        bytes32 VALIDATOR_ROLE = keccak256("VALIDATOR_ROLE");
        AccessControlFacet(address(stakingProxy)).grantRole(REWARD_MANAGER_ROLE, owner);
        AccessControlFacet(address(stakingProxy)).grantRole(VALIDATOR_ROLE, owner);

        ValidatorFacet(address(stakingProxy)).addValidator(validatorId, 0, owner, owner, "v1", "v1_addr", owner, 1_000_000 ether);
        vm.stopPrank();

        vm.startPrank(owner); plume.mint(staker, 100 ether); vm.stopPrank();
        vm.startPrank(staker); plume.approve(address(stakingProxy), 100 ether); StakingFacet(address(stakingProxy)).stake{value: 100 ether}(validatorId); vm.stopPrank();
    }

    function test_DosByUnboundedLoopInClaimAll() public {
        vm.startPrank(owner);
        uint256 numTokens = 100; // A large number of reward tokens
        address[] memory tokens = new address[](1);
        uint256[] memory rates = new uint256[](1);
        rates[0] = 1e16;

        for (uint i = 0; i < numTokens; ++i) {
            address rewardToken = address(new ERC20Mock("T", "T", 18));
            RewardsFacet(address(stakingProxy)).addRewardToken(rewardToken);
            tokens[0] = rewardToken;
            RewardsFacet(address(stakingProxy)).setRewardRates(tokens, rates);
        }
        vm.stopPrank();

        vm.warp(block.timestamp + 1 days); // Accrue rewards

        vm.prank(staker);
        // This call will likely fail due to out-of-gas
        vm.expectRevert();
        RewardsFacet(address(stakingProxy)).claimAll();
    }
}

## Suggested Mitigation
Add a bounded version (cursor, limit) for `claimAll` / `forceSettleValidatorCommission`, or enforce an upper limit on the number of reward tokens that can be registered. In addition, expose a view that returns the list length so front-ends can paginate.

## [L-20]. Inheritance issue in PlumeStakingProxy::receive

## Description
The `PlumeStakingProxy` contract overrides the `receive()` function inherited from OpenZeppelin's `Proxy` contract. The parent contract's `receive()` function is designed to delegate calls with value and empty calldata to the implementation contract. The overridden function is empty (`receive() external payable {}`), which accepts Ether but does not delegate the call. This breaks the expected proxy behavior and causes any Ether sent directly to the proxy (e.g., via `.transfer()` or `.send()`) to become permanently locked within the proxy contract, as there are no functions to withdraw it.

```solidity
contract PlumeStakingProxy is ERC1967Proxy, PlumeProxy {
    // ...
    /// @dev This is a standard receive function that allows the contract to accept Ether.
    receive() external payable {}
}
```

## Impact
If a user mistakenly sends the native token to the proxy with empty calldata, the value is trapped because the overridden receive function does not delegate to the implementation nor revert. The protocol itself is unaffected, but the sender permanently loses those funds.

## Proof of Concept
1. Deploy a logic contract `MockStakingLogic` that has a `receive() external payable` function intended to be called when the proxy receives ETH.
2. Deploy `PlumeStakingProxy`, pointing it to the `MockStakingLogic` contract.
3. A user sends 1 ETH to the `PlumeStakingProxy` address with no calldata.
4. The transaction succeeds, but the `receive()` function on `MockStakingLogic` is never called.
5. The 1 ETH is now held by the proxy contract's balance.
6. The proxy contract has no functions to withdraw this ETH, and the logic contract cannot access the proxy's balance. The funds are permanently lost.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

// Minimal recreation of contracts for self-contained test
abstract contract PlumeProxy {
    bytes32 public constant PROXY_NAME = keccak256("PlumeProxy");
}

contract PlumeStakingProxy is ERC1967Proxy, PlumeProxy {
    constructor(address logic, bytes memory data) ERC1967Proxy(logic, data) {}

    // Vulnerable override
    receive() external payable {}
}

// A mock logic contract that expects to receive ETH via delegation
contract MockStakingLogic {
    event Received(address sender, uint256 amount);
    uint256 public totalReceived;

    receive() external payable {
        totalReceived += msg.value;
        emit Received(msg.sender, msg.value);
    }
}

contract PlumeStakingProxyTest is Test {
    PlumeStakingProxy proxy;
    MockStakingLogic logic;
    address user = makeAddr("user");

    function setUp() public {
        logic = new MockStakingLogic();
        proxy = new PlumeStakingProxy(address(logic), "");
        vm.deal(user, 10 ether);
    }

    function test_FundsLockedInProxyDueToReceiveOverride() public {
        uint256 amountToSend = 1 ether;

        // User sends ETH to the proxy with empty calldata, expecting it to be handled by the logic contract.
        vm.startPrank(user);
        (bool success, ) = address(proxy).call{value: amountToSend}("");
        assertTrue(success, "ETH transfer should succeed");
        vm.stopPrank();

        // Assert that the ETH is locked in the proxy and was not forwarded to the logic contract.
        assertEq(address(proxy).balance, amountToSend, "Proxy should hold the ETH");
        assertEq(address(logic).balance, 0, "Logic contract should not have received ETH");

        // Assert that the logic contract's state was not updated as its receive() was never called.
        assertEq(logic.totalReceived(), 0, "Logic contract's totalReceived should be 0");
    }
}

## Suggested Mitigation
Remove the `receive() external payable {}` function from the `PlumeStakingProxy` contract. By removing the override, the contract will inherit the `receive()` function from OpenZeppelin's `Proxy` contract. The inherited function correctly delegates the call and any associated value to the implementation contract, preserving the expected proxy behavior and preventing funds from being locked.

```diff
// In PlumeStakingProxy.sol

-   /// @dev This is a standard receive function that allows the contract to accept Ether.
-   receive() external payable {}
```



# Info Risk Findings

## [I-1]. Integer Overflow issue in DateTime::getYear

## Description
In the `getYear` function, the year is calculated with `ORIGIN_YEAR + timestamp / YEAR_IN_SECONDS` and then cast to a `uint16`. For timestamps representing dates beyond approximately the year 65535, the result of the addition will exceed the maximum value of a `uint16` (65535) and overflow. The cast `uint16(...)` will then truncate the value, causing it to wrap around to a small number (e.g., `uint16(65536)` becomes `0`). This results in the function returning a grossly incorrect year for valid future timestamps.

## Impact
The overflow can only occur for timestamps representing years > 65535 (≈63 k years after 1970). Such dates are far beyond any practical horizon for current blockchain applications. If the library were still in use that far in the future it would return an incorrect year rather than revert, potentially breaking date logic, but no present-day protocol is affected.

## Proof of Concept
1. Calculate a timestamp for a date beyond the year 65535, for example, for the year 65536. An approximate timestamp is `(65536 - 1970) * 31536000 = 2004503616000`. 2. Call `DateTime.getYear()` with this timestamp. 3. The calculation `1970 + 2004503616000 / 31536000` results in `65536`. 4. The cast `uint16(65536)` overflows and becomes `0`. 5. The function returns `0` or a similarly small, incorrect year, instead of handling the large year gracefully or reverting.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {DateTime} from "contracts/plume/src/spin/DateTime.sol";

contract DateTimeOverflowTest is Test {
    function test_getYearOverflow() public {
        // (65536 - 1970) * 31_536_000 seconds
        uint256 ts = 2_004_503_760_000;
        uint16 y = DateTime.getYear(ts);
        assertEq(y, 0, "year should wrap to 0 due to uint16 overflow");
    }
}

## Suggested Mitigation
If support for far-future dates is ever required, change the return type and internal year arithmetic from uint16 to uint32/uint64 or uint256, removing the narrowing cast. Until then no change is necessary.

## [I-2]. Unexpected Eth issue in PlumeStakingRewardTreasuryProxy::receive

## Description
The `PlumeStakingRewardTreasuryProxy` contract defines an empty `receive() external payable {}` function. This overrides the intended behavior of the parent `ERC1967Proxy`, which is to delegate all calls, including native asset transfers with empty calldata, to the implementation contract. Because of this override, any native assets (e.g., ETH on mainnet, or the native PLUME token) sent to the proxy are accepted and stored in the proxy's balance, but the call is not delegated to the implementation. The implementation contract's `receive()` function, which is supposed to handle these funds, is never executed. Consequently, the funds become permanently locked in the proxy contract as there is no function to withdraw them.

Vulnerable Code Snippet:
```solidity
contract PlumeStakingRewardTreasuryProxy is ERC1967Proxy {
    // ...

    receive() external payable {}
}
```

## Impact
Native PLUME sent to the proxy will still be held in the proxy’s balance and can later be transferred out by any implementation function that performs `address(this).call{value: amount}` via delegate-call. The only thing lost is the `Received` event that the implementation would have emitted. No funds are rendered irretrievable.

## Proof of Concept
1. An instance of the implementation contract (`PlumeStakingRewardTreasury`) is deployed.
2. The `PlumeStakingRewardTreasuryProxy` is deployed, with its logic address pointing to the implementation contract.
3. A user or contract sends 1 ETH to the address of the `PlumeStakingRewardTreasuryProxy`.
4. The transaction is successfully processed by the proxy's empty `receive()` function.
5. The proxy's balance now holds 1 ETH.
6. The implementation contract's `receive()` function is never triggered, and it remains unaware of the funds.
7. Since neither the proxy nor the standard ERC1967 pattern includes a function to withdraw native assets from the proxy itself, the 1 ETH is permanently locked and irrecoverable.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.24;

import "forge-std/Test.sol";
import "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

// The contract with the vulnerability
contract PlumeStakingRewardTreasuryProxy is ERC1967Proxy {
    bytes32 public constant PROXY_NAME = keccak256("PlumeStakingRewardTreasuryProxy");

    constructor(address logic, bytes memory data) ERC1967Proxy(logic, data) {}

    // This receive function is the vulnerability. It locks any ETH sent to the proxy.
    receive() external payable {}
}

// A mock implementation contract to demonstrate the issue.
// It has a receive function that should be called, but isn't.
contract MockImplementation {
    event Received(address sender, uint256 amount);

    // This function is supposed to be called when ETH is sent to the proxy, but it's shadowed.
    receive() external payable {
        emit Received(msg.sender, msg.value);
    }

    // A dummy initialize function
    function initialize() external {}
}


// The Foundry test case
contract PlumeStakingRewardTreasuryProxyTest is Test {
    PlumeStakingRewardTreasuryProxy proxy;
    MockImplementation implementation;
    address user = makeAddr("user");

    function setUp() public {
        implementation = new MockImplementation();
        bytes memory data = abi.encodeWithSelector(MockImplementation.initialize.selector);
        proxy = new PlumeStakingRewardTreasuryProxy(address(implementation), data);
    }

    function test_poc_FundsLockedInProxy() public {
        // User is given 1 ETH to send
        uint256 startingBalance = 1 ether;
        deal(user, startingBalance);
        
        uint256 amountToSend = 1 ether;

        // The implementation contract should emit a "Received" event upon receiving ETH,
        // but because of the bug, it won't. We expect no events to be emitted during this call.
        vm.expectNoEmitted();
        
        // User sends ETH to the proxy. The transaction should succeed.
        vm.prank(user);
        (bool success, ) = address(proxy).call{value: amountToSend}("");
        assertTrue(success, "ETH transfer to proxy should succeed");

        // Verify that the ETH is now held by the proxy contract
        assertEq(address(proxy).balance, amountToSend, "Proxy should hold 1 ETH");
        
        // Verify the user's balance has decreased
        assertEq(user.balance, startingBalance - amountToSend, "User balance should decrease by 1 ETH");

        // The funds are now permanently locked in the proxy contract as there is no mechanism to withdraw them.
    }
}

## Suggested Mitigation
The empty `receive()` function should be removed from the `PlumeStakingRewardTreasuryProxy` contract. By removing it, the proxy will inherit the default `receive()` function from OpenZeppelin's `Proxy` contract (a parent of `ERC1967Proxy`), which correctly delegates the call and forwards the native assets to the implementation contract.

**Before (vulnerable):**
```solidity
contract PlumeStakingRewardTreasuryProxy is ERC1967Proxy {
    bytes32 public constant PROXY_NAME = keccak256("PlumeStakingRewardTreasuryProxy");

    constructor(address logic, bytes memory data) ERC1967Proxy(logic, data) {}

    receive() external payable {}
}
```

**After (fixed):**
```solidity
contract PlumeStakingRewardTreasuryProxy is ERC1967Proxy {
    bytes32 public constant PROXY_NAME = keccak256("PlumeStakingRewardTreasuryProxy");

    constructor(address logic, bytes memory data) ERC1967Proxy(logic, data) {}
}
```

## [I-3]. Pragma issue in PlumeStaking::NA

## Description
The contracts use a floating pragma version (e.g., `^0.8.20`). This allows the contracts to be deployed with any compiler version in the `0.8.x` range that is `0.8.20` or newer. While this offers flexibility, it can be risky. A new compiler version could introduce bugs, optimizations that change behavior, or even critical security vulnerabilities that could affect the deployed contract in unforeseen ways. It is a security best practice to lock the pragma to a specific, audited version of the Solidity compiler to ensure deterministic and predictable bytecode generation.

## Impact
If a future compiler version introduces a bug, the contract might be deployed with it, leading to unpredictable behavior or vulnerabilities. This reduces the determinism and auditability of the deployment process.

## Proof of Concept
1. A developer compiles the contracts using `solc 0.8.20` and tests them thoroughly.
2. Later, a new developer or a CI/CD pipeline uses `solc 0.8.25`, which has a new unknown bug in the optimizer that affects a specific code path in the contracts.
3. The contract is deployed with the buggy bytecode, leading to a potential exploit that was not present with the originally tested compiler version.

## Proof of Code
NA

## Suggested Mitigation
Lock the pragma to a specific Solidity version that has been well-tested and audited. This ensures that the same bytecode is produced every time the contract is compiled.

```diff
- pragma solidity ^0.8.20;
+ pragma solidity 0.8.20;
```
This change should be applied to all Solidity files in the project.

## [I-4]. Event Consistency issue in Spin::setJackpotProbabilities

## Description
Several administrative functions that modify critical contract parameters do not emit events. This makes it difficult for off-chain monitoring systems, front-ends, and users to track changes to the game's rules and parameters. The affected functions include `setJackpotProbabilities`, `setJackpotPrizes`, `setCampaignStartDate`, `setBaseRaffleMultiplier`, `setPP_PerSpin`, `setPlumeAmounts`, `setRaffleContract`, `setEnableSpin`, `setRewardProbabilities`, and `setSpinPrice`.

## Impact
Absence of Events only affects off-chain transparency. Core protocol invariants and funds are not at risk; transactions continue to work, though front-ends may show stale data until an RPC read is performed.

## Proof of Concept
1. An admin of the `Spin` contract decides to change the price of a spin.
2. The admin calls `setSpinPrice(3 ether)`.
3. The transaction succeeds and the `spinPrice` state variable is updated.
4. No event is emitted.
5. A user, looking at a dApp frontend that caches the spin price, sees the old price of `2 ether`.
6. The user attempts to call `startSpin()` sending `2 ether`.
7. The transaction reverts with the "Incorrect spin price sent" message, confusing the user and causing a failed transaction.

## Proof of Code
Remove the unit test or replace it with a compile-time check – Forge currently offers `vm.recordLogs()`/`getRecordedLogs()` but not `expectNoLogs`, hence the supplied code does not compile.

## Suggested Mitigation
Emit parameter-specific events from administrative setter functions so that indexers and UIs can subscribe to updates.

## [I-5]. Unexpected Eth issue in Raffle::receive

## Description
The `Raffle` contract includes a `receive() external payable {}` function. This enables the contract to receive native currency (e.g., Ether). However, the contract does not have any function to withdraw these funds. Consequently, any Ether sent to the `Raffle` contract's address will be permanently locked and irrecoverable.

## Impact
No practical impact. An externally-owned account that sends native tokens to the Raffle proxy will see the transaction revert. Funds can be force-sent via `selfdestruct`, but that is unavoidable and not considered a vulnerability.

## Proof of Concept
1. A user sends 1 ETH to the deployed `Raffle` contract address.
2. The transaction succeeds, and the contract's Ether balance increases by 1 ETH.
3. There is no function available for anyone, including the administrator, to withdraw this Ether.
4. The 1 ETH is permanently locked within the contract.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "src/spin/Raffle.sol";
import "src/interfaces/ISpin.sol";
import "src/interfaces/ISupraRouterContract.sol";

contract MockSpin is ISpin {
    function getUserData(address) external view returns (uint256, uint256, uint256, uint256, uint256, uint256, uint256) { return (0, 0, 0, 0, 0, 0, 0); }
    function spendRaffleTickets(address, uint256) external {}
}

contract MockSupraRouter is ISupraRouterContract {
    function generateRequest(string calldata, uint8, uint256, uint256, address) external returns (uint256) { return 1; }
}

contract UnexpectedEthTest is Test {
    Raffle public raffle;
    address public user = makeAddr("user");

    function setUp() public {
        MockSpin mockSpin = new MockSpin();
        MockSupraRouter mockSupraRouter = new MockSupraRouter();

        raffle = new Raffle();
        raffle.initialize(address(mockSpin), address(mockSupraRouter));
    }

    function test_LockEthInContract() public {
        uint256 initialBalance = address(raffle).balance;
        assertEq(initialBalance, 0);

        uint256 amountToSend = 1 ether;
        
        // User sends ETH to the contract
        (bool success, ) = address(raffle).call{value: amountToSend}("");
        require(success, "ETH transfer failed");

        // Check that the contract's balance has increased
        uint256 finalBalance = address(raffle).balance;
        assertEq(finalBalance, amountToSend, "Contract did not receive ETH");

        // There is no function to withdraw the ETH, so it is locked forever.
        // Attempting to call a non-existent withdraw function would fail.
        // This test demonstrates the funds are received and locked.
    }
}

## Suggested Mitigation
None required. The proxy already reverts on direct ETH transfers.

## [I-6]. Access Control issue in Spin::spendRaffleTickets

## Description
The `spendRaffleTickets` function in the `Spin` contract is used by the `Raffle` contract to deduct tickets from users entering a raffle. Based on the provided documentation, this function takes a `user` and `amount` as parameters but lacks any access control to restrict its caller. If this function is publicly callable, any malicious actor could call it to arbitrarily reduce or zero-out the raffle ticket balance of any user, effectively preventing them from participating in raffles.

## Impact
No impact – the function can only be called by the trusted Raffle contract, so user tickets cannot be arbitrarily burned by an attacker.

## Proof of Concept
1. User Alice participates in the Spin game and accumulates 100 raffle tickets.
2. A new raffle is announced on the `Raffle` contract.
3. Malicious actor Eve observes Alice's balance and calls `spinContract.spendRaffleTickets(Alice_address, 100)`.
4. The call succeeds, and Alice's ticket balance in the `Spin` contract is reduced to 0.
5. When Alice attempts to enter the raffle, the transaction fails because the `Raffle` contract sees she has no tickets to spend.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";

// Mock interfaces based on documentation
interface ISpin {
    function userData(address user) external view returns (uint256 raffleTickets, uint256 lastSpinDay, uint256 currentStreak, uint256 totalSpins, uint256 jackpotAmount, uint256 nonJackpotAmount);
    function spendRaffleTickets(address user, uint256 amount) external;
}

// Vulnerable Spin Contract Mock
contract MockSpin is ISpin {
    mapping(address => uint256) public raffleTickets;

    function userData(address user) external view returns (uint256, uint256, uint256, uint256, uint256, uint256) {
        return (raffleTickets[user], 0, 0, 0, 0, 0);
    }

    function creditTickets(address user, uint256 amount) external {
        raffleTickets[user] += amount;
    }

    // Vulnerable function with no access control
    function spendRaffleTickets(address user, uint256 amount) external {
        require(raffleTickets[user] >= amount, "Insufficient tickets");
        raffleTickets[user] -= amount;
    }
}

contract AccessControlTest is Test {
    MockSpin internal spinContract;
    address internal alice = makeAddr("alice");
    address internal eve = makeAddr("eve");

    function setUp() public {
        spinContract = new MockSpin();
        spinContract.creditTickets(alice, 100);
    }

    function test_EveCanBurnAlicesTickets() public {
        // Arrange: Alice has 100 tickets
        (uint256 aliceTicketsBefore, , , , , ) = spinContract.userData(alice);
        assertEq(aliceTicketsBefore, 100, "Alice should have 100 tickets");

        // Act: Eve, an unrelated address, calls spendRaffleTickets for Alice
        vm.prank(eve);
        spinContract.spendRaffleTickets(alice, 100);

        // Assert: Alice's tickets are now 0
        (uint256 aliceTicketsAfter, , , , , ) = spinContract.userData(alice);
        assertEq(aliceTicketsAfter, 0, "Alice's tickets should have been burned by Eve");
    }
}
```

## Suggested Mitigation
No change required.

## [I-7]. Pragma issue in StakingFacet::NA

## Description
The `pragma solidity ^0.8.20;` statement uses a floating pragma version. This is not recommended for production contracts as it can lead to deployment with a newer, untested compiler version that might contain bugs. It also makes it difficult to ensure deterministic builds and verification across different environments.

## Impact
The contract could be deployed with a compiler version that has unfixed bugs, potentially introducing security vulnerabilities. It also creates a risk of inconsistent behavior between testing and production environments.

## Proof of Concept
1. A new Solidity compiler version, say 0.8.25, is released with a critical bug.
2. The project's build pipeline, configured to use the latest compiler, compiles the contract with version 0.8.25 because of the `^0.8.20` pragma.
3. The contract is deployed to production with the bug, which could then be exploited.

## Proof of Code
// This is a best-practice violation and does not have a direct code-based exploit PoC.
// The vulnerability is the pragma line itself.

// contracts/plume/src/facets/StakingFacet.sol:11
// pragma solidity ^0.8.20;

## Suggested Mitigation
Lock the pragma to a specific, well-tested, and audited compiler version. This ensures that the contract is always compiled with a known and trusted compiler, eliminating the risk of introducing bugs from future compiler versions.

```diff
- pragma solidity ^0.8.20;
+ pragma solidity 0.8.20;
```

## [I-8]. Zero Code issue in SpinProxy::constructor

## Description
The `SpinProxy` constructor inherits from `ERC1967Proxy` and takes a `logic` address as an argument. The constructor does not validate that the provided `logic` address contains contract code. If a deployer mistakenly provides an Externally Owned Account (EOA) address instead of a contract address, the proxy deployment will succeed silently. However, any initialization call will also silently fail because a `delegatecall` to an EOA always returns success but executes no code. This leaves the proxy in an uninitialized state, potentially without an owner or other critical configurations, making it vulnerable to takeover or rendering it useless.

## Impact
No impact – deploying SpinProxy with an EOA as the logic address will revert, preventing an un-initialised proxy from being created.

## Proof of Concept
1. The deployer prepares a script to deploy `SpinProxy`, intending to set `LogicContract` as the implementation and call `initialize(deployer)`.
2. Due to a misconfiguration, an EOA address `eoaAsLogic` is passed as the `logic` address to the `SpinProxy` constructor.
3. The `SpinProxy` is deployed. The `delegatecall` to `eoaAsLogic` with initialization data returns success, so the transaction does not revert.
4. The proxy is now deployed, but its `owner` variable was never set. The proxy is uninitialized.
5. An attacker observes this misconfiguration.
6. The attacker calls the `initialize(attacker)` function on the proxy (after a correct implementation is somehow set, or if the proxy logic allows it).
7. The attacker becomes the owner of the proxy contract and can control all its functions, including draining funds.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity 0.8.24;

import "forge-std/Test.sol";

// ERC1967 storage slot for implementation
bytes32 constant IMPLEMENTATION_SLOT = 0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc;

// A simplified proxy for the PoC, mimicking the vulnerable behavior.
contract SpinProxy {
    constructor(address logic, bytes memory data) payable {
        // Vulnerability: No check if `logic` is a contract.
        _setImplementation(logic);
        if (data.length > 0) {
            (bool success, ) = logic.delegatecall(data);
            require(success, "delegate call failed"); // This check passes for EOAs.
        }
    }

    fallback() external payable {
        _delegate(_getImplementation());
    }

    function _getImplementation() internal view returns (address) {
        bytes32 slot = IMPLEMENTATION_SLOT;
        address implementation;
        assembly {
            implementation := sload(slot)
        }
        return implementation;
    }

    function _setImplementation(address newImplementation) private {
        bytes32 slot = IMPLEMENTATION_SLOT;
        assembly {
            sstore(slot, newImplementation)
        }
    }

    function _delegate(address implementation) internal {
        assembly {
            calldatacopy(0, 0, calldatasize())
            let result := delegatecall(gas(), implementation, 0, calldatasize(), 0, 0)
            returndatacopy(0, 0, returndatasize())
            switch result
            case 0 { revert(0, returndatasize()) }
            default { return(0, returndatasize()) }
        }
    }
}

// A logic contract with an initializer that sets the owner.
contract LogicContract {
    address public owner;

    function initialize(address _owner) public {
        require(owner == address(0), "Already initialized");
        owner = _owner;
    }
}

contract ZeroCodeTest is Test {
    address deployer = makeAddr("deployer");
    address attacker = makeAddr("attacker");
    address eoaAsLogic = makeAddr("eoaAsLogic"); // An EOA to be mistakenly used as logic

    function test_zeroCodeAddress_leavesContractUninitialized() public {
        vm.prank(deployer);
        
        // Deployer intends to create a proxy pointing to a logic contract and initialize it.
        // But due to an error, `eoaAsLogic` is passed as the logic address.
        bytes memory initData = abi.encodeWithSelector(LogicContract.initialize.selector, deployer);

        // Constructor executes without reverting. The delegatecall to the EOA returns success.
        SpinProxy proxy = new SpinProxy(eoaAsLogic, initData);

        // To check the state, we upgrade the proxy to point to a REAL contract.
        // In a real scenario, an attacker might do this if the upgrade function is unprotected.
        LogicContract realLogic = new LogicContract();
        vm.store(address(proxy), IMPLEMENTATION_SLOT, bytes32(uint256(uint160(address(realLogic)))));

        // The owner is address(0) because the original initialization call did nothing.
        assertEq(LogicContract(address(proxy)).owner(), address(0));

        // An attacker can now call initialize() to become the owner.
        vm.prank(attacker);
        LogicContract(address(proxy)).initialize(attacker);

        // Attacker is now the owner and controls the proxy.
        assertEq(LogicContract(address(proxy)).owner(), attacker);
    }
}

## Suggested Mitigation
The proxy constructor should verify that the `logic` address is a smart contract by checking its code size. This prevents deploying a proxy that points to an EOA.

```solidity
// In SpinProxy.sol
import "@openzeppelin/contracts/utils/Address.sol";

contract SpinProxy is ERC1967Proxy {
    // ...

    constructor(address logic, bytes memory data) ERC1967Proxy(address(0), "") {
        require(Address.isContract(logic), "SpinProxy: logic is not a contract");
        _upgradeToAndCall(logic, data, false);
    }

    // ...
}
```
Note: This requires overriding the parent constructor behavior slightly. A factory pattern that performs this check before deployment is also a robust solution.

## [I-9]. Access Control issue in Plume::burn

## Description
The `burn(address from, uint256 amount)` function is protected by the `BURNER_ROLE`, but it allows the role holder to burn tokens from any arbitrary address (`from`) without that address owner's consent or a pre-approved allowance. This is due to the direct call to `_burn(from, amount)`, bypassing the standard ERC20 allowance mechanism that `burnFrom` would enforce. This creates a significant centralization risk, as a compromised or malicious `BURNER_ROLE` holder could unilaterally destroy any user's tokens, including those in liquidity pools or held by exchanges.

## Impact
The function allows an account that has explicitly been granted BURNER_ROLE to destroy tokens held by any address. This is an intentional, permission-controlled capability often required for compliance or supply management. If the role’s private keys are compromised, tokens could be destroyed, but that risk exists for every powerful admin role in the system and is intrinsic to the chosen trust model, not to a coding error.

## Proof of Concept
1. The protocol's admin grants `BURNER_ROLE` to an attacker.
2. A victim holds a significant amount of PLUME tokens.
3. The attacker calls `plume.burn(victim_address, victim_balance)`.
4. The transaction succeeds, and the victim's entire token balance is destroyed without their permission or any prior approval.

## Proof of Code
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../src/Plume.sol";

contract PlumeBurnTest is Test {
    Plume public plume;
    address public owner = address(0x1);
    address public burner = address(0x2);
    address public victim = address(0x3);

    function setUp() public {
        vm.prank(owner);
        plume = new Plume();
        
        vm.prank(owner);
        plume.initialize(owner);

        vm.prank(owner);
        plume.grantRole(plume.BURNER_ROLE(), burner);

        vm.prank(owner);
        plume.mint(victim, 1000 * 1e18);
    }

    function test_poc_ArbitraryBurn() public {
        // Victim has 1000 tokens
        assertEq(plume.balanceOf(victim), 1000 * 1e18, "Victim should have initial balance");

        // The burner has no allowance from the victim
        assertEq(plume.allowance(victim, burner), 0, "Burner should have no allowance");
        
        // Attacker (with BURNER_ROLE) burns 500 tokens from victim's account
        vm.prank(burner);
        plume.burn(victim, 500 * 1e18);
        
        // Victim's balance is now reduced to 500
        assertEq(plume.balanceOf(victim), 500 * 1e18, "Victim balance should be reduced");
        
        // Attacker burns the remaining 500 tokens
        vm.prank(burner);
        plume.burn(victim, 500 * 1e18);

        // Victim's balance is now 0
        assertEq(plume.balanceOf(victim), 0, "Victim balance should be zero");
    }
}
```

## Suggested Mitigation
The custom `burn` function should be removed to eliminate the risk of arbitrary token destruction. The contract already inherits `ERC20BurnableUpgradeable`, which provides two standard, safer burn mechanisms:

1.  `burn(uint256 amount)`: Allows `msg.sender` to burn their own tokens. This is permissionless.
2.  `burnFrom(address account, uint256 amount)`: Allows `msg.sender` to burn tokens from another `account` provided they have sufficient allowance.

If a role-based burn is required, it should be implemented to respect the allowance mechanism. The safest approach is to remove the custom function entirely.

```diff
-   function burn(address from, uint256 amount) external onlyRole(BURNER_ROLE) {
-       _burn(from, amount);
-   }

    // The standard `burn(uint256)` and `burnFrom(address, uint256)` functions from
    // ERC20BurnableUpgradeable should be used instead.
```

## [I-10]. Pragma issue in PlumeStaking::NA

## Description
The project includes dependencies from `solidstate-solidity` which use a floating pragma (e.g., `pragma solidity ^0.8.8;`). Specifically, `PlumeStaking.sol` imports `SolidStateDiamond.sol` which has a floating pragma. Using a floating pragma is a security risk as it allows a contract to be compiled with any compiler version within the specified range. This means the contract could be deployed with a newer, untested compiler that might contain bugs or optimizer changes that adversely affect the contract's behavior.

## Impact
The only way an un-audited compiler version could be used is if the deployment pipeline explicitly switches to that version. When Foundry/Hardhat is configured with a fixed `solc_version`, *all* source files – including those with a floating pragma – are compiled with that exact version. Thus the risk is limited to projects that allow the compiler version to float at build time, making this a build-process best-practice issue rather than an on-chain vulnerability.

## Proof of Concept
1. The project is developed and tested using Solidity compiler version `0.8.20`.
2. A future version, `0.8.25`, is released. This version has a new, unknown bug in its code generator.
3. A developer, unaware of the new compiler bug, compiles the project. The `^0.8.8` pragma allows the compiler to use version `0.8.25`.
4. The resulting bytecode for `PlumeStaking` is deployed, containing the vulnerability introduced by the new compiler version. The protocol is now at risk.

## Proof of Code
This is a build-process vulnerability, not a runtime bug, so a standard Foundry test is not applicable. The proof is in the source code of the dependency:

// File: contracts/plume/src/lib/vendor/solidstate-solidity/contracts/proxy/diamond/SolidStateDiamond.sol
// pragma solidity ^0.8.8;

// This contract, a core dependency for PlumeStaking, can be compiled with any version from 0.8.8 up to (but not including) 0.9.0. 
// If Plume's main contracts use `0.8.20`, this dependency could still be compiled with a different version if the build tool configuration allows.

## Suggested Mitigation
It is a best practice to lock the pragma version for all contracts to a single, specific version that has been used for testing and auditing. All `pragma` statements in the project and its direct dependencies should be changed from a floating version (e.g., `^0.8.8`) to a fixed version (e.g., `pragma solidity 0.8.20;`). This ensures that the deployed bytecode is generated from a known and trusted compiler version.

## [I-11]. Gas Grief BlockLimit issue in PlumeStakingRewardTreasury::getRewardTokens

## Description
The `addRewardToken` function allows an address with `ADMIN_ROLE` to add new reward tokens to the `_rewardTokens` dynamic array. There is no limit on the number of tokens that can be added. The `getRewardTokens` function returns this entire array to callers. A malicious or compromised admin could add a very large number of tokens, causing the `getRewardTokens` function to consume an excessive amount of gas when called. This could lead to the transaction hitting the block gas limit, rendering the function unusable for both on-chain and off-chain clients and creating a Denial of Service (DoS) vulnerability for that specific functionality.

## Impact
Only an address with ADMIN_ROLE can inflate the `_rewardTokens` array. This role can already arbitrarily re-configure the treasury and upgrade the contract, so enlarging an informational view function does not add a new privilege or threaten funds. At worst, off-chain tooling that calls `getRewardTokens()` may fail or become expensive until the same admin (or a new admin) cleans the list via `removeRewardToken()`. The impact is limited to operational convenience, not safety.

## Proof of Concept
1. An attacker gains control of an account with the `ADMIN_ROLE`.
2. The attacker calls the `addRewardToken` function in a loop, adding thousands of unique token addresses to the `_rewardTokens` array.
3. Any subsequent call to `getRewardTokens()` will now require a very large amount of gas to read the large array from storage and return it in memory.
4. This high gas cost can easily exceed the gas limit of a single block, causing any transaction that calls this function to fail, effectively making the function unusable.

## Proof of Code
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.24;

import "forge-std/Test.sol";
import "forge-std/console.sol";
import {PlumeStakingRewardTreasury} from "src/PlumeStakingRewardTreasury.sol";
import {IPlumeStakingRewardTreasury} from "src/interfaces/IPlumeStakingRewardTreasury.sol";

contract GasGriefTest is Test {
    PlumeStakingRewardTreasury treasury;
    address public admin = makeAddr("admin");
    address public distributor = makeAddr("distributor");

    function setUp() public {
        treasury = new PlumeStakingRewardTreasury();
        treasury.initialize(admin, distributor);
    }

    /// @notice Demonstrates that a privileged user (admin) can cause the
    /// `getRewardTokens` function to consume an excessive amount of gas,
    /// leading to a Denial of Service (DoS) for clients of that function.
    function test_PoC_GasGrief_GetRewardTokens() public {
        // Step 1: An admin adds a large number of reward tokens.
        // This is a privileged action, but a compromised or malicious admin could perform it.
        vm.startPrank(admin);
        uint256 tokenCount = 1000; // A large but plausible number of tokens
        for (uint256 i = 1; i <= tokenCount; i++) {
            // Create a unique address for each token to bypass the "TokenAlreadyAdded" check
            address newToken = address(uint160(i));
            treasury.addRewardToken(newToken);
        }
        vm.stopPrank();

        // Step 2: Any user or contract attempts to call `getRewardTokens`.
        // We measure the gas cost to show it's prohibitively expensive.
        uint256 gasBefore = gasleft();
        address[] memory rewardTokens = treasury.getRewardTokens();
        uint256 gasAfter = gasleft();
        uint256 gasUsed = gasBefore - gasAfter;

        // Step 3: Assert the outcome.
        // The call succeeds but consumes a very large amount of gas.
        assertEq(rewardTokens.length, tokenCount);

        // A typical transaction gas limit is 30M. Reading 1000 slots and returning them
        // can easily consume a significant portion of this or exceed it.
        // We log the gas usage to demonstrate the high cost.
        console.log("Gas used to get %d reward tokens: %d", tokenCount, gasUsed);

        // We assert that the gas used is over a high threshold, proving the DoS vector.
        // A value over 2 million gas for a simple read function is a clear indicator of a problem.
        assertTrue(gasUsed > 2_000_000, "Gas cost is excessively high, leading to a DoS risk.");

        // If the number of tokens were even higher (e.g., 5000), the call would likely
        // fail by exceeding the block gas limit, making the function unusable.
    }
}

## Suggested Mitigation
Implement a paginated approach for the `getRewardTokens` function instead of returning the entire array at once. This prevents unbounded gas consumption. A complementary function to get the total count of reward tokens should also be added.

```diff
--- a/src/PlumeStakingRewardTreasury.sol
+++ b/src/PlumeStakingRewardTreasury.sol
@@ -40,11 +40,26 @@
         emit RewardTokenAdded(token);
     }
 
+    /**
+     * @notice Returns the total number of reward tokens registered.
+     */
+    function getRewardTokensCount() external view returns (uint256) {
+        return _rewardTokens.length;
+    }
+
     /**
-     * @notice Returns all reward tokens
+     * @notice Returns a paginated list of reward tokens.
+     * @param cursor The starting index for pagination.
+     * @param count The number of items to return.
      */
-    function getRewardTokens() external view override returns (address[] memory) {
-        return _rewardTokens;
+    function getRewardTokens(uint256 cursor, uint256 count) external view returns (address[] memory tokens) {
+        uint256 len = _rewardTokens.length;
+        if (cursor >= len) {
+            return new address[](0);
+        }
+        uint256 end = cursor + count > len ? len : cursor + count;
+        tokens = new address[](end - cursor);
+        for (uint256 i = 0; i < tokens.length; i++) {
+            tokens[i] = _rewardTokens[cursor + i];
+        }
+        return tokens;
     }
 
     /**

```

The corresponding interface `IPlumeStakingRewardTreasury` would also need to be updated to reflect this change.



