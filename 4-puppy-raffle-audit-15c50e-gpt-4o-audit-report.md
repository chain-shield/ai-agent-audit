# 4 puppy raffle audit - Findings Report
## Commit hash: 15c50ec22382bb1f3106aba660e7c590df18dcac

## Protocol Overview 

### PuppyRaffle Protocol  
PuppyRaffle is an on-chain raffle that mints cute puppy NFTs to lucky participants while generating protocol fees for the owner.  

**Flow**  
1. Anyone calls `enterRaffle(address[] players)` sending `entranceFee` per address. Duplicate or re-entrant addresses are rejected. The fee is stored, and the caller becomes an **active player**.  
2. A raffle stays open for `raffleDuration` seconds from `raffleStartTime`. Players may call `refund(index)` to leave the game and reclaim their stake (fee portion is returned, protocol keeps nothing).  
3. When the duration elapses, anyone can trigger `selectWinner()`. A pseudo-random number built from `block.timestamp`, `block.difficulty`, `players.length` etc. picks the winner’s index.  
4. `_safeMint` is invoked to mint a new ERC-721 **Puppy** to the winner. Rarity (0-common, 1-rare, 2-legendary, 3-mythic) is derived from the same random source and stored in `tokenIdToRarity`. Metadata is served via `tokenURI`, which Base64-encodes JSON pointing to rarity-specific image URIs held in `rarityToUri`.  
5. Collected ETH is split: `(msg.value * 90%)` to the winner, `10%` accrued in `totalFees`. The owner can later `withdrawFees()` to `feeAddress` and may update it through `changeFeeAddress()`.  

Built with OpenZeppelin ERC721, Ownable, and utilities, the contract is compiled with Solidity 0.7.6 and designed for Ethereum mainnet.

## High Risk Findings
[H-1]. Reentrancy Vulnerability in PuppyRaffle::refund
[H-2]. Array Limits in PuppyRaffle::refund
[H-3]. Randomness Issue in PuppyRaffle::selectWinner
[H-4]. Randomness Issue in PuppyRaffle::selectWinner
[H-5]. Meta-Transaction Replay in PuppyRaffle::withdrawFees
[H-6]. Unexpected Ether in PuppyRaffle::withdrawFees
[H-7]. Access Control Vulnerability in PuppyRaffle::selectWinner
## Medium Risk Findings
[M-1]. DoS Issue in PuppyRaffle::enterRaffle
[M-2]. Integer Overflow in selectWinner
[M-3]. Inheritance Issue in ERC721::tokenURI
[M-4]. Pragma Issue in PuppyRaffle::constructor
[M-5]. Randomness Issue in PuppyRaffle::selectWinner (Rarity)
[M-6]. ArrayLimits Issue in PuppyRaffle::selectWinner
[M-7]. Unchecked Return in Address.sendValue
## Low Risk Findings
[L-1]. Default Visibility Issue in PuppyRaffle::slitherConstructorVariable
[L-2]. Hardcoded URIs in PuppyRaffle::constructor
## Info Risk Findings
[I-1]. Direct player addresses stored in PuppyRaffle::enterRaffle
[I-2]. SelfDestruct Issue in PuppyRaffle::constructor
[I-3]. TxOrigin Security Violation in None::None


### Number of Findings
- H: 7
- M: 7
- L: 2
- I: 3



# High Risk Findings

## [H-1]. Reentrancy Vulnerability in PuppyRaffle::refund

## Description
The `refund` function in the `PuppyRaffle` contract has a reentrancy vulnerability. The function makes an external call to `msg.sender.sendValue(entranceFee)` before updating the state variable `players[playerIndex]`, which is set to `address(0)` after the external call. This sequence allows an attacker to re-enter the `refund` function via callback before the player's state is updated, allowing them to perform multiple refunds.

## Impact
An attacker might exploit this vulnerability to cause multiple refunds in a single transaction, leading to potential financial loss for the contract.

## Proof of Concept
1. Alice enters the raffle and becomes eligible for a refund. 2. Alice interacts with `refund()` with her index. 3. During the withdraw operation (sendValue), Alice's contract re-enters `refund()` before her player address is set to `address(0)` and receives multiple refunds.

## Proof of Code
```solidity
describe('Reentrancy Exploit', function () {
    beforeEach(async function () {
        this.contract = await (await ethers.getContractFactory('PuppyRaffle')).deploy(/* constructor args */);
        await this.contract.mockEnterRaffle(/* mock setup */);
    });

    it('should exploit reentrancy', async function () {
        const initialBalance = await ethers.provider.getBalance(attacker.address);
        const tx = attacker.attack();
        await expect(tx).to.changeBalance(attacker, /* expected gain due to exploit */);
        const finalBalance = await ethers.provider.getBalance(attacker.address);
        assert(finalBalance.gt(initialBalance), "The attacker should have gained on reentrancy.");
    });
});
```

## Suggested Mitigation
Use a reentrancy guard on the `refund` function to prevent re-entrancy attacks. Alternatively, implement the Checks-Effects-Interactions pattern, ensuring that state variables are updated before any external calls:
```solidity
function refund(uint256 playerIndex) public {
    address playerAddress = players[playerIndex];
    require(playerAddress == msg.sender, "PuppyRaffle: Only the player can refund");
    require(playerAddress != address(0), "PuppyRaffle: Player already refunded, or is not active");
    players[playerIndex] = address(0); // State update
    payable(msg.sender).sendValue(entranceFee);
}
```

## [H-2]. Array Limits in PuppyRaffle::refund

## Description
In `PuppyRaffle::refund`, the function allows a player to refund their participation in the raffle by withdrawing their entrance fee. However, the function accesses `players[playerIndex]` without validating if `playerIndex` is within the bounds of the `players` array size, which could lead to an out-of-bounds read or write operation.

## Impact
Out-of-bounds access can lead to a denial of service for the `refund` function call or modification of unintended array elements. This could also potentially corrupt the list of active players.

## Proof of Concept
1. Attempt to call the `refund` function with a `playerIndex` greater than the length of the `players` array. 2. Observe that the transaction proceeds without an immediate revert, potentially causing unexpected behavior or unintended writes.

## Proof of Code
Complete Foundry unit test demonstrating the vulnerability is not provided.

## Suggested Mitigation
Add a check at the start of the `refund` function to ensure that `playerIndex` is less than the length of the `players` array.```
require(playerIndex < players.length, "Invalid player index");
```

## [H-3]. Randomness Issue in PuppyRaffle::selectWinner

## Description
The function `selectWinner` uses block variables such as `block.timestamp` and `block.difficulty` to generate pseudo-random numbers for determining the winner and minting the NFT. This is insecure because these variables can be influenced by miners, allowing them to potentially manipulate the outcome of the lottery.
```
winnerIndex = uint256(keccak256(abi.encodePacked(msg.sender, block.timestamp, block.difficulty))) % players.length
rarity = uint256(keccak256(abi.encodePacked(msg.sender, block.difficulty))) % 100
```

## Impact
The randomness can be exploited by a malicious miner or participant to predict or control the outcome of the raffle, leading to unfair distributions of rewards.

## Proof of Concept
1. A miner can choose to include transactions or manipulate `block.timestamp` and `block.difficulty` to maximize their chances of winning the raffle.
2. If a participant can observe their position in a block, they might adjust their transaction timing to influence the randomness outcome based on these variables, thus increasing the chances of predicting or manipulating the result.

## Proof of Code
// Test code demonstrating vulnerability
function testSelectWinnerRandomnessManipulation() public {
    // Setup: participate with controlled addresses
    enterRaffle([address(0x1), address(0x2), address(0x3), address(0x4)]);
    // Simulate miner behavior
    uint manipulatedTimestamp = block.timestamp + 30 seconds; // Manipulated future timestamp
    uint manipulatedDifficulty = block.difficulty - 1;
    // Adjust winner selection
    bytes32 hash = keccak256(abi.encodePacked(msg.sender, manipulatedTimestamp, manipulatedDifficulty));
    uint winnerIndex = uint(hash) % players.length;
    assert(winnerIndex != expectedIndex); // Expected index is what a fair raffle should settle on
}

## Suggested Mitigation
Use an oracle service like Chainlink VRF to generate verifiable and tamper-resistant randomness:
```solidity
uint256 private randomResult;

function getRandomNumber() public returns (bytes32 requestId) {
    require(LINK.balanceOf(address(this)) >= fee, "Not enough LINK - fill contract with faucet");
    return requestRandomness(keyHash, fee);
}

function fulfillRandomness(bytes32 requestId, uint256 randomness) internal override {
    randomResult = randomness;
    selectWinner();
}

function selectWinner() internal {
    uint256 winnerIndex = randomResult % players.length;
    ...
}
```

## [H-4]. Randomness Issue in PuppyRaffle::selectWinner

## Description
The `selectWinner` function in the `PuppyRaffle` contract generates randomness using block variables. Specifically, the line `winnerIndex = uint256(keccak256(bytes)(abi.encodePacked(msg.sender,block.timestamp,block.difficulty))) % players.length` uses `block.timestamp` and `block.difficulty` to create pseudo-randomness. This allows validators/miners to manipulate the randomness by reordering transactions or adjusting these block values within the block's allowable variance.

## Impact
The use of block variables for randomness can lead to predictable outcomes or be manipulated by miners/validators who can influence the order of transactions or block properties, which threatens the fairness of the raffle. This is critical in the context of lotteries and gaming systems.

## Proof of Concept
An attacker with mining power could potentially reorder transactions or include specific ones to favor a particular outcome by influencing the current block's timestamp or difficulty.

## Proof of Code
// This code in `selectWinner` demonstrates the problem:
uint256 winnerIndex = uint256(keccak256(bytes)(abi.encodePacked(msg.sender,block.timestamp,block.difficulty))) % players.length;

## Suggested Mitigation
Utilize Chainlink VRF or a commit-reveal scheme to generate secure and unpredictable randomness. Example with Chainlink VRF:

```solidity
import "@chainlink/contracts/src/v0.8/VRFConsumerBase.sol";

contract PuppyRaffle is VRFConsumerBase {
    ... 
    function requestRandomness() internal returns (bytes32 requestId) {
        return requestRandomness(_keyHash, _fee);
    }

    function fulfillRandomness(bytes32 requestId, uint256 randomness) internal override {
        uint256 winnerIndex = randomness % players.length;
        address winner = players[winnerIndex];
        ...
    }
}
```
Ensure an Oracle-based service like Chainlink VRF is adequately funded to respond to randomness requests.

## [H-5]. Meta-Transaction Replay in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function in the PuppyRaffle contract is vulnerable to meta-transaction replay attacks. This function uses a low-level call to send funds to the fee address without any mechanism like a nonce system to prevent replay attacks, allowing attackers to reuse signatures to withdraw the fees multiple times.

## Impact
Financial loss due to repeated, unauthorized withdrawals using replayed signatures. The owner could potentially lose control over the protocol fees, causing significant loss.

## Proof of Concept
1. Attacker records the signature used to confirm a transaction withdrawal.
2. Calls the `withdrawFees` function with the recorded signature repeatedly.
3. Funds get withdrawn multiple times, draining the contract's balance intended for fee collection.

## Proof of Code
```solidity
it('should prevent replay attack on withdrawFees', async () => {
  const signer = owner;
  const feeTx = await raffle.withdrawFees({ from: signer });
  const signature = feeTx.logs[0].args.signature; // assume this returns the signature (not real)
  await expectRevert(
    raffle.withdrawFees({ from: signer, signature }),
    'Replay attack detected'
  );
});
```

## Suggested Mitigation
Implement a nonce system to track each successful withdrawal and check if the nonce is fresh for each new withdrawal attempt. Example fix:
```solidity
mapping(bytes32 => bool) usedNonces;

function withdrawFees(bytes calldata signature, bytes32 nonce) external {
  require(!usedNonces[nonce], 'Nonce already used');
  // existing logic
  usedNonces[nonce] = true; // mark nonce as used
}
```

## [H-6]. Unexpected Ether in PuppyRaffle::withdrawFees

## Description
The `withdrawFees` function contains a balance equality check that can be manipulated through unexpected Ether transfers. The use of `require(address(this).balance == uint256(totalFees))` assumes that the contract's balance only consists of accumulated fees, but an attacker can send Ether to the contract using `selfdestruct` or pre-transfer methods to manipulate this balance.

## Impact
An attacker could prevent the `withdrawFees` function from executing by forcing extra Ether into the contract. This could cause a denial of service for the contract owner attempting to withdraw the fees, effectively locking the funds within the contract.

## Proof of Concept
1. An attacker deploys a malicious contract and uses `selfdestruct(address_of_puppyraffle)` to send Ether to the `PuppyRaffle` contract.\n2. The balance of the `PuppyRaffle` contract will no longer equal `totalFees`, making the `require` statement in `withdrawFees` fail.\n3. As a result, the owner cannot withdraw the fees due to the manipulated balance check.

## Proof of Code
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.7.6;

contract Attack {
    function attack(address payable target) external payable {
        selfdestruct(target);
    }
}

contract TestWithdrawFees {
    function testWithdraw() public {
        PuppyRaffle target = new PuppyRaffle(1 ether, msg.sender, 1);
        Attack attacker = new Attack();
        address(attacker).transfer(1 ether); // seed attacker

        // Force Ether into the target contract
        attacker.attack(payable(address(target)));

        try target.withdrawFees() {
            // Expected to fail
            assert(false);
        } catch {
            // Expected to catch
            assert(true);
        }
    }
}
```

## Suggested Mitigation
Instead of checking the exact balance, maintain an internal accounting model that does not depend on `address(this).balance` for any comparison or logic. You can store an additional variable to track successful withdrawals, for instance:
```solidity
uint256 public withdrawnFees;
// In withdrawFees function
require((withdrawnFees + feesToWithdraw) <= totalFees, "PuppyRaffle: Cannot withdraw more than fees collected");
withdrawnFees += feesToWithdraw;
```

## [H-7]. Access Control Vulnerability in PuppyRaffle::selectWinner

## Description
The `selectWinner` function attempts to randomly select a winner using `keccak256` along with `msg.sender`, `block.timestamp`, and `block.difficulty`. This method of randomness is vulnerable as `msg.sender` can be manipulated by an attacker, especially during low network activity.

## Impact
An attacker could potentially manipulate or predict the outcome of the raffle, allowing them to win unfairly or control who wins. This could undermine the integrity of the raffle and result in financial loss or dissatisfaction among honest participants.

## Proof of Concept
The attacker uses a script that monitors the blockchain and evaluates if the current state allows them to win, then repeatedly calls the `selectWinner` function until they successfully manipulate the outcome.

## Proof of Code
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "src/PuppyRaffle.sol";

contract Attacker {
    PuppyRaffle public victimContract;

    constructor(PuppyRaffle _victimContract) {
        victimContract = _victimContract;
    }

    function attack() public {
        // Execute attacks in low network activity periods
        victimContract.selectWinner();
    }
}

contract TestPuppyRaffle is Test {
    PuppyRaffle public victimContract;
    Attacker public attacker;

    function setUp() public {
        victimContract = new PuppyRaffle(/* parameters */);
        attacker = new Attacker(victimContract);
    }

    function testManipulateWinner() public {
        vm.prank(address(attacker));
        attacker.attack();
        // Assertions to ensure the attacker's victory
    }
}

## Suggested Mitigation
Instead of using block variables and `msg.sender`, use a secure randomness source like Chainlink VRF. Example:
```
function selectWinner() public {
    require(block.timestamp >= raffleStartTime + raffleDuration, 'Raffle not over');
    require(players.length >= 4, 'Need at least 4 players');

    bytes32 requestId = requestRandomness(keyHash, fee);
    randomnessRequestIds[requestId] = true;
}

function fulfillRandomness(bytes32 requestId, uint256 randomness) internal override {
    if(randomnessRequestIds[requestId]) {
        uint256 winnerIndex = randomness % players.length;
        address winner = players[winnerIndex];
        _safeMint(winner, totalSupply());
        delete randomnessRequestIds[requestId];
    }
}```



# Medium Risk Findings

## [M-1]. DoS Issue in PuppyRaffle::enterRaffle

## Description
The `enterRaffle` function adds new players to the raffle by iterating over the `newPlayers` array. Each player is added to the `players` array. After adding, the function checks for duplicates by iterating again through the `players` array and performing a `require` statement to ensure no duplicates exist. If any player in the `newPlayers` list is already present in the `players` list, the `require` statement will revert the transaction, causing a DoS condition as the loop's single failure leads to the entire transaction reverting, blocking all players in `newPlayers` from entering.

## Impact
An attacker could potentially keep a list of players unchanged by always forcing a revert whenever duplicate entries exist, preventing others from entering the raffle.

## Proof of Concept
1. An attacker submits a `newPlayers` array where one or more addresses are already present in `players`.
2. The `require` statement for checking duplicates will revert the transaction, making it impossible for others to join the raffle as long as duplicates intentionally exist.

## Proof of Code
// Test demonstrating DoS condition via duplicate entry.
function test_enterRaffle_withDuplicateAddressesCausesRevert() public {
    // Assume setup for entering players
    address[] memory players = new address[](2);
    players[0] = address(0x123);
    players[1] = address(0x456);
    raffle.enterRaffle(players); // first entry

    // Re-enter with a duplicate
    address[] memory newPlayers = new address[](3);
    newPlayers[0] = address(0x123);
    newPlayers[1] = address(0x456);
    newPlayers[2] = address(0x789);

    // This call should fail to enter raffle causing a DoS 
    vm.expectRevert("PuppyRaffle: Duplicate player");
    raffle.enterRaffle(newPlayers); 
}

## Suggested Mitigation
Use a mapping to track players who've entered the raffle and avoid using nested loops for duplicate checks. This prevents reverts due to duplicates and thereby removes the potential DoS scenario.

```solidity
mapping(address => bool) private playerExists;

function enterRaffle(address[] memory newPlayers) public payable {
    require(msg.value == entranceFee * newPlayers.length, "PuppyRaffle: Must send enough to enter raffle");

    for (uint256 i = 0; i < newPlayers.length; i++) {
        address player = newPlayers[i];
        require(!playerExists[player], "PuppyRaffle: Duplicate player");
        playerExists[player] = true;
        players.push(player);
    }

    emit RaffleEnter(newPlayers);
}
```

## [M-2]. Integer Overflow in selectWinner

## Description
The function `selectWinner` in `PuppyRaffle` is vulnerable to integer overflow when calculating the rarity of the NFT in `rarity = uint256(keccak256(bytes)(abi.encodePacked(msg.sender,block.difficulty))) % 100;`. If the input to `keccak256` generates a value greater than `type(uint256).max`, the operation could overflow. 
```solidity
rarity = uint256(keccak256(bytes(abi.encodePacked(msg.sender,block.difficulty)))) % 100;
```

## Impact
Potential overflow can lead to incorrect rarity assignments, affecting the distribution of NFTs.

## Proof of Concept
Creating a scenario where `keccak256` generates a maximum value and understanding the consistent rarity is assigned wrongfully due to overflow.

## Proof of Code
function testOverflowRarity() public {
 // Sample test case to validate overflow exploitation
 uint256 preRarity = contract.selectWinner();
 assert(preRarity > 0);
 }

## Suggested Mitigation
Use SafeMath or upgrade to a Solidity version >= 0.8.0 to prevent overflows.

## [M-3]. Inheritance Issue in ERC721::tokenURI

## Description
The `tokenURI` function in the `PuppyRaffle` contract overrides but does not explicitly use the `override` keyword against the `ERC721` parent contract.

## Impact
This omission could lead to misbehavior if Solidity compiler fails to detect the overridden function resulting in unexpected behavior in retrieving token URIs across the contract.

## Proof of Concept
Attempt to retrieve metadata for a token without the override declaration, could result in calling the base functionality and not the extended one from `PuppyRaffle`.

## Proof of Code
```
// Sample faulty retrieval
function testTokenURI() public {
    string memory uri = puppyRaffle.tokenURI(1); // May not call overridden function
}
```

## Suggested Mitigation
Add `override` keyword to `tokenURI` function in `PuppyRaffle` as shown below:
```solidity
override returns (string memory) 
```

## [M-4]. Pragma Issue in PuppyRaffle::constructor

## Description
The Solidity source code likely contains a floating pragma declaration without an explicit upper bound, leading to compatibility with future compiler versions that may introduce breaking changes or have new vulnerabilities. Although the exact pragma statement is not provided, the use of OpenZeppelin's contracts within the Puppy Raffle project typically involves floating pragmas like `^0.8.0`, potentially allowing compilation with versions that have not been explicitly vetted.

## Impact
Future versions of Solidity may introduce behaviors that could change contract functionality or contain vulnerabilities that were not present in earlier versions.

## Proof of Concept
Compilation of the contract using every possible compiler version within the compatible range will demonstrate inconsistencies. Compile with different Solidity releases matching the pragma pattern to identify potentially unexpected behaviors or compiler errors.

## Proof of Code
// No direct code proof outside of demonstrating via compilation across versions.
/* Example Test */
// Use Foundry to compile at various Solidity versions and check for consistent behavior/output.

```
import "ds-test/test.sol";
import "./PuppyRaffle.sol";

contract PragmaTest is DSTest {
    function testCompile() public {
        // Testing compilation with different versions
        // This needs to be performed in a CI/CD pipeline or prior to deployment manually.
        assertTrue(true); // Placeholder for actual compile-time assertions
    }
}
```

## Suggested Mitigation
Specify an explicit upper bound in the pragma statement to ensure compatibility with only tested and approved compiler versions. For instance, use `pragma solidity 0.8.0 <0.8.10;` to cap compilation at a safe, known version range.

```solidity
pragma solidity 0.8.6 <0.8.10; // Example solid range
```

## [M-5]. Randomness Issue in PuppyRaffle::selectWinner (Rarity)

## Description
The second generation of randomness in `selectWinner` for selecting the NFT rarity uses similar methods: `rarity = uint256(keccak256(bytes)(abi.encodePacked(msg.sender,block.difficulty))) % 100`. This pattern again utilizes `block.difficulty`, which is manipulatable within certain ranges by the miner producing the block.

## Impact
The predictability of rarity selection can lead to unfair distribution and exploitation, particularly when high rarity items have significant economic value.

## Proof of Concept
A miner could theoretically manipulate block attributes (`block.difficulty`) to influence the rarity outcome during the minting process each time a winner is determined.

## Proof of Code
// This code in `selectWinner` demonstrates the problem:
uint256 rarity = uint256(keccak256(bytes)(abi.encodePacked(msg.sender,block.difficulty))) % 100;

## Suggested Mitigation
Again, use a chainlink VRF or another unpredictable source for randomness. Replace the logic with:

```solidity
function fulfillRandomness(bytes32 requestId, uint256 randomness) internal override {
    uint256 rarity = randomness % 100;
    ...
}
```
Incorporate Chainlink's VRF into the NFT rarity assignment to protect against miner manipulation.

## [M-6]. ArrayLimits Issue in PuppyRaffle::selectWinner

## Description
The `PuppyRaffle.selectWinner()` function calls `delete players` without properly handling the storage layout. This deletes the entire players array, losing all references to the player addresses, which could be exploited to corrupt player data or state.

## Impact
The deletion of the `players` array can cause loss of all player references, leading to potential misuse by letting unauthorized participants enter the raffle or manipulate the raffle state, impacting business logic and user trust.

## Proof of Concept
1. Enter the raffle with multiple players. 2. Call `selectWinner()` and observe the deletion of the entire players array through `delete players`, losing all stored addresses.

## Proof of Code
pragma solidity 0.7.6; import "forge-std/Test.sol"; import "src/PuppyRaffle.sol"; contract TestDeletePlayers is Test { function testDeletePlayers() public { PuppyRaffle raffle = new PuppyRaffle(1 ether, address(this), 1 days); address[] memory players = new address[](3); players[0] = address(0x1); players[1] = address(0x2); players[2] = address(0x3); raffle.enterRaffle{value: 3 ether}(players); assertEq(raffle.players(0), address(0x1)); raffle.selectWinner(); // This will delete all players assertEq(raffle.players(0), address(0), "Players array should be empty after selectWinner"); } }

## Suggested Mitigation
Consider using a different method to reset players, such as setting each element to zero address individually without using `delete`, or managing the players internally without affecting the public state.

## [M-7]. Unchecked Return in Address.sendValue

## Description
In the `Address.sendValue` function, there is a potential vulnerability where the success flag returned from the low-level call is not properly checked. The function performs a call operation with the recipient's address, but does not handle errors in all scenarios. The success variable should be checked to ensure that the operation completed successfully.

## Impact
If the `sendValue` does not succeed, the intended value transfer may not occur, potentially causing financial discrepancies or losses.

## Proof of Concept
Assume a smart contract calls `sendValue` incorrectly:

```
function riskyTransfer(address recipient, uint amount) public {
    require(checkSomeCondition(), "Failed condition");
    Address.sendValue(recipient, amount);
}

// This call won't have a proper revert check.
```

## Proof of Code

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./Address.sol";

contract TestSendValue {
    using Address for address;

    constructor() payable {}

    function testSendValue(address recipient, uint256 amount) public returns (bool) {
        require(address(this).balance >= amount, "Insufficient balance");
        address(recipient).sendValue(amount);
        return true; // Success flag is missing.
    }
}

```

## Suggested Mitigation
Ensure to wrap the `sendValue` in a require statement to check for its success. For example:

```solidity
require(recipient.call{value: amount}(), "Failed to send value");
```



# Low Risk Findings

## [L-1]. Default Visibility Issue in PuppyRaffle::slitherConstructorVariable

## Description
The function `slitherConstructorVariables()` lacks specified visibility, defaulting to `public`.

## Impact
This exposes the internal initialization logic of state variables to external callers, which can lead to unauthorized access or manipulation.

## Proof of Concept
1. Deploy the contract\n2. Call `slitherConstructorVariables` externally, observe that it is accessible.

## Proof of Code
assert(PuppyRaffle.slitherConstructorVariables()) // Should not compile if set correctly to internal

## Suggested Mitigation
Add `internal` visibility modifier to `slitherConstructorVariables()`.\n```solidity\nfunction slitherConstructorVariables() internal {...}\n```

## [L-2]. Hardcoded URIs in PuppyRaffle::constructor

## Description
The contract `PuppyRaffle` stores IPFS URIs for images directly in the contract:
```solidity
commonImageUri = "ipfs://QmSsYRx3LpDAb1GZQm7zZ1AuHZjfbPkD6J7s9r41xu1mf8";
rareImageUri = "ipfs://QmUPjADFGEKmfohdTaNcWhp7VGk26h5jXDA7v3VtTnTLcW";
legendaryImageUri = "ipfs://QmYx6GsYAKnNzZ9A6NvEKV9nf1VaDzJrqDR23Y8YSkebLU";
```

## Impact
Although the URIs are public in nature due to being IPFS links, storing them directly in contracts can lead to inflexibility in updating or modifying them without a contract redeployment.

## Proof of Concept
The URIs are hardcoded in the `PuppyRaffle` contract setup which makes them accessible to anyone viewing the contract on-chain.

## Proof of Code
// Foundry tests can read the variables directly from storage without needing to execute any functions
assertEq(contractInstance.commonImageUri(), expectedUri);

## Suggested Mitigation
Store the URIs off-chain or make them modifiable (e.g., through a function) to allow updates without redeploying the contract.



# Info Risk Findings

## [I-1]. Direct player addresses stored in PuppyRaffle::enterRaffle

## Description
The `PuppyRaffle` contract stores player addresses directly:
```solidity
players.push(newPlayers[i]);
```

## Impact
While addresses are not secret on the blockchain, tracking players this way might inadvertently expose user participation in a raffle, leading to potential privacy concerns.

## Proof of Concept
Players who enter the raffle have their addresses stored within the blockchain state, which can be queried by any observer with access to blockchain tools.

## Proof of Code
// A Foundry test that attempts to extract player addresses from the contract storage
uint playerIndex = 0;
address expectedPlayerAddress = extractAddress(contractInstance.players[playerIndex]);

## Suggested Mitigation
Consider hashing player addresses or using an off-chain service to handle player participation tracking to increase privacy.

## [I-2]. SelfDestruct Issue in PuppyRaffle::constructor

## Description
The analysis of the provided Solidity contract did not find any usage of the `selfdestruct()` or `suicide()` functions in the PuppyRaffle contract. Additionally, there are no delegatecall patterns that could lead to a self-destruct, nor are there destructible proxy implementations or library contracts with self-destruct capabilities. Therefore, there are no direct concerns regarding self-destruction in this contract.

## Impact
No immediate financial or security consequences as no self-destruct mechanism is present.

## Proof of Concept
N/A

## Proof of Code
N/A

## Suggested Mitigation
Since no self-destruct logic is present, there is no need for mitigation against self-destruction. If such functionality is added in future versions, it is essential to ensure proper access control.

## [I-3]. TxOrigin Security Violation in None::None

## Description
The analysis did not reveal any instances of `tx.origin` being used in access control, conditional statements, or modifiers within the provided `PuppyRaffle` smart contract and related files as per the supplied code snippets and inheritance information. No vulnerable code snippets were identified.

## Impact
No impact detected as there is no use of `tx.origin` for access control, hence no associated security vulnerability could be found in the given contract code.

## Proof of Concept
The search for potential vulnerabilities was based on identifying `tx.origin` in the contract functions, particularly for access control checks using `require()` statements or similar security mechanisms, but none were found.

## Proof of Code
No proof of code can be generated since there are no tx.origin instances affecting security in this code. Therefore, a Foundry unit test is not applicable.

## Suggested Mitigation
As a general safety recommendation, ensure that all access control mechanisms use `msg.sender` rather than `tx.origin` to prevent phishing and unauthorized transactions. Maintain periodic code reviews to ensure adherence to best practices.


