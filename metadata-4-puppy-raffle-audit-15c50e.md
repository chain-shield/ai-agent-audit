
## List of Files in Src Folder
src/PuppyRaffle.sol

## README.md summary
The Puppy Raffle project aims to facilitate a raffle for winning a dog NFT. Participants can enter using the `enterRaffle` function that takes a list of participant addresses, excluding duplicates. Participants can refund their tickets and value through the `refund` function. The raffle system can draw a winner periodically, with the owner of the protocol setting a fee address to handle financial transactions. Funds are split between the fee address and the raffle winner.

The getting started section outlines the necessary tools like Git and Foundry. Basic steps include cloning the repository and executing make commands. Alternatively, Gitpod can be used for remote setup.

Testing can be done using `forge test` and `forge coverage` for test coverage analysis.

Audit scope details include a commit hash for tracking changes and a specification that the PuppyRaffle.sol file is included. Solc version 0.7.6 is recommended, and the contract is deployed on Ethereum.

The owner holds the role of setting fee wallet addresses, while players can enter raffles or refund their entries. Currently, there are no known issues with the protocol.


## src/PuppyRaffle.sol summary
The **PuppyRaffle** contract is an Ethereum smart contract that enables users to participate in a raffle to win an NFT of a cute dog, with features such as entering the raffle, refunds, winner selection, and fee management.

