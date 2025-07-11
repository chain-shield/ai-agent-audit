
## Slither Contract Summary
--ignore-compile used, if something goes wrong, consider removing the ignore compile flag
INFO:Printers:
+ Contract Base64 (Most derived contract)
  - From Base64
    - decode(string) (internal)
    - encode(bytes) (internal)

+ Contract Ownable
  - From Context
    - _msgData() (internal)
    - _msgSender() (internal)
  - From Ownable
    - constructor() (internal)
    - owner() (public)
    - renounceOwnership() (public)
    - transferOwnership(address) (public)

+ Contract ERC165
  - From ERC165
    - _registerInterface(bytes4) (internal)
    - constructor() (internal)
    - supportsInterface(bytes4) (public)

+ Contract IERC165
  - From IERC165
    - supportsInterface(bytes4) (external)

+ Contract SafeMath (Most derived contract)
  - From SafeMath
    - add(uint256,uint256) (internal)
    - div(uint256,uint256) (internal)
    - div(uint256,uint256,string) (internal)
    - mod(uint256,uint256) (internal)
    - mod(uint256,uint256,string) (internal)
    - mul(uint256,uint256) (internal)
    - sub(uint256,uint256) (internal)
    - sub(uint256,uint256,string) (internal)
    - tryAdd(uint256,uint256) (internal)
    - tryDiv(uint256,uint256) (internal)
    - tryMod(uint256,uint256) (internal)
    - tryMul(uint256,uint256) (internal)
    - trySub(uint256,uint256) (internal)

+ Contract ERC721
  - From ERC165
    - _registerInterface(bytes4) (internal)
    - constructor() (internal)
    - supportsInterface(bytes4) (public)
  - From Context
    - _msgData() (internal)
    - _msgSender() (internal)
  - From ERC721
    - _approve(address,uint256) (private)
    - _beforeTokenTransfer(address,address,uint256) (internal)
    - _burn(uint256) (internal)
    - _checkOnERC721Received(address,address,uint256,bytes) (private)
    - _exists(uint256) (internal)
    - _isApprovedOrOwner(address,uint256) (internal)
    - _mint(address,uint256) (internal)
    - _safeMint(address,uint256) (internal)
    - _safeMint(address,uint256,bytes) (internal)
    - _safeTransfer(address,address,uint256,bytes) (internal)
    - _setBaseURI(string) (internal)
    - _setTokenURI(uint256,string) (internal)
    - _transfer(address,address,uint256) (internal)
    - approve(address,uint256) (public)
    - balanceOf(address) (public)
    - baseURI() (public)
    - constructor(string,string) (public)
    - getApproved(uint256) (public)
    - isApprovedForAll(address,address) (public)
    - name() (public)
    - ownerOf(uint256) (public)
    - safeTransferFrom(address,address,uint256) (public)
    - safeTransferFrom(address,address,uint256,bytes) (public)
    - setApprovalForAll(address,bool) (public)
    - symbol() (public)
    - tokenByIndex(uint256) (public)
    - tokenOfOwnerByIndex(address,uint256) (public)
    - tokenURI(uint256) (public)
    - totalSupply() (public)
    - transferFrom(address,address,uint256) (public)

+ Contract IERC721
  - From IERC165
    - supportsInterface(bytes4) (external)
  - From IERC721
    - approve(address,uint256) (external)
    - balanceOf(address) (external)
    - getApproved(uint256) (external)
    - isApprovedForAll(address,address) (external)
    - ownerOf(uint256) (external)
    - safeTransferFrom(address,address,uint256) (external)
    - safeTransferFrom(address,address,uint256,bytes) (external)
    - setApprovalForAll(address,bool) (external)
    - transferFrom(address,address,uint256) (external)

+ Contract IERC721Enumerable
  - From IERC721
    - approve(address,uint256) (external)
    - balanceOf(address) (external)
    - getApproved(uint256) (external)
    - isApprovedForAll(address,address) (external)
    - ownerOf(uint256) (external)
    - safeTransferFrom(address,address,uint256) (external)
    - safeTransferFrom(address,address,uint256,bytes) (external)
    - setApprovalForAll(address,bool) (external)
    - transferFrom(address,address,uint256) (external)
  - From IERC165
    - supportsInterface(bytes4) (external)
  - From IERC721Enumerable
    - tokenByIndex(uint256) (external)
    - tokenOfOwnerByIndex(address,uint256) (external)
    - totalSupply() (external)

+ Contract IERC721Metadata
  - From IERC721
    - approve(address,uint256) (external)
    - balanceOf(address) (external)
    - getApproved(uint256) (external)
    - isApprovedForAll(address,address) (external)
    - ownerOf(uint256) (external)
    - safeTransferFrom(address,address,uint256) (external)
    - safeTransferFrom(address,address,uint256,bytes) (external)
    - setApprovalForAll(address,bool) (external)
    - transferFrom(address,address,uint256) (external)
  - From IERC165
    - supportsInterface(bytes4) (external)
  - From IERC721Metadata
    - name() (external)
    - symbol() (external)
    - tokenURI(uint256) (external)

+ Contract IERC721Receiver (Most derived contract)
  - From IERC721Receiver
    - onERC721Received(address,address,uint256,bytes) (external)

+ Contract Address (Most derived contract)
  - From Address
    - _verifyCallResult(bool,bytes,string) (private)
    - functionCall(address,bytes) (internal)
    - functionCall(address,bytes,string) (internal)
    - functionCallWithValue(address,bytes,uint256) (internal)
    - functionCallWithValue(address,bytes,uint256,string) (internal)
    - functionDelegateCall(address,bytes) (internal)
    - functionDelegateCall(address,bytes,string) (internal)
    - functionStaticCall(address,bytes) (internal)
    - functionStaticCall(address,bytes,string) (internal)
    - isContract(address) (internal)
    - sendValue(address,uint256) (internal)

+ Contract Context
  - From Context
    - _msgData() (internal)
    - _msgSender() (internal)

+ Contract EnumerableMap (Most derived contract)
  - From EnumerableMap
    - _at(EnumerableMap.Map,uint256) (private)
    - _contains(EnumerableMap.Map,bytes32) (private)
    - _get(EnumerableMap.Map,bytes32) (private)
    - _get(EnumerableMap.Map,bytes32,string) (private)
    - _length(EnumerableMap.Map) (private)
    - _remove(EnumerableMap.Map,bytes32) (private)
    - _set(EnumerableMap.Map,bytes32,bytes32) (private)
    - _tryGet(EnumerableMap.Map,bytes32) (private)
    - at(EnumerableMap.UintToAddressMap,uint256) (internal)
    - contains(EnumerableMap.UintToAddressMap,uint256) (internal)
    - get(EnumerableMap.UintToAddressMap,uint256) (internal)
    - get(EnumerableMap.UintToAddressMap,uint256,string) (internal)
    - length(EnumerableMap.UintToAddressMap) (internal)
    - remove(EnumerableMap.UintToAddressMap,uint256) (internal)
    - set(EnumerableMap.UintToAddressMap,uint256,address) (internal)
    - tryGet(EnumerableMap.UintToAddressMap,uint256) (internal)

+ Contract EnumerableSet (Most derived contract)
  - From EnumerableSet
    - _add(EnumerableSet.Set,bytes32) (private)
    - _at(EnumerableSet.Set,uint256) (private)
    - _contains(EnumerableSet.Set,bytes32) (private)
    - _length(EnumerableSet.Set) (private)
    - _remove(EnumerableSet.Set,bytes32) (private)
    - add(EnumerableSet.AddressSet,address) (internal)
    - add(EnumerableSet.Bytes32Set,bytes32) (internal)
    - add(EnumerableSet.UintSet,uint256) (internal)
    - at(EnumerableSet.AddressSet,uint256) (internal)
    - at(EnumerableSet.Bytes32Set,uint256) (internal)
    - at(EnumerableSet.UintSet,uint256) (internal)
    - contains(EnumerableSet.AddressSet,address) (internal)
    - contains(EnumerableSet.Bytes32Set,bytes32) (internal)
    - contains(EnumerableSet.UintSet,uint256) (internal)
    - length(EnumerableSet.AddressSet) (internal)
    - length(EnumerableSet.Bytes32Set) (internal)
    - length(EnumerableSet.UintSet) (internal)
    - remove(EnumerableSet.AddressSet,address) (internal)
    - remove(EnumerableSet.Bytes32Set,bytes32) (internal)
    - remove(EnumerableSet.UintSet,uint256) (internal)

+ Contract Strings (Most derived contract)
  - From Strings
    - toString(uint256) (internal)

+ Contract PuppyRaffle (Most derived contract)
  - From Ownable
    - constructor() (internal)
    - owner() (public)
    - renounceOwnership() (public)
    - transferOwnership(address) (public)
  - From Context
    - _msgData() (internal)
    - _msgSender() (internal)
  - From ERC721
    - _approve(address,uint256) (private)
    - _beforeTokenTransfer(address,address,uint256) (internal)
    - _burn(uint256) (internal)
    - _checkOnERC721Received(address,address,uint256,bytes) (private)
    - _exists(uint256) (internal)
    - _isApprovedOrOwner(address,uint256) (internal)
    - _mint(address,uint256) (internal)
    - _safeMint(address,uint256) (internal)
    - _safeMint(address,uint256,bytes) (internal)
    - _safeTransfer(address,address,uint256,bytes) (internal)
    - _setBaseURI(string) (internal)
    - _setTokenURI(uint256,string) (internal)
    - _transfer(address,address,uint256) (internal)
    - approve(address,uint256) (public)
    - balanceOf(address) (public)
    - baseURI() (public)
    - constructor(string,string) (public)
    - getApproved(uint256) (public)
    - isApprovedForAll(address,address) (public)
    - name() (public)
    - ownerOf(uint256) (public)
    - safeTransferFrom(address,address,uint256) (public)
    - safeTransferFrom(address,address,uint256,bytes) (public)
    - setApprovalForAll(address,bool) (public)
    - symbol() (public)
    - tokenByIndex(uint256) (public)
    - tokenOfOwnerByIndex(address,uint256) (public)
    - totalSupply() (public)
    - transferFrom(address,address,uint256) (public)
  - From ERC165
    - _registerInterface(bytes4) (internal)
    - supportsInterface(bytes4) (public)
  - From PuppyRaffle
    - _baseURI() (internal)
    - _isActivePlayer() (internal)
    - changeFeeAddress(address) (external)
    - constructor(uint256,address,uint256) (public)
    - enterRaffle(address[]) (public)
    - getActivePlayerIndex(address) (external)
    - refund(uint256) (public)
    - selectWinner() (external)
    - tokenURI(uint256) (public)
    - withdrawFees() (external)

INFO:Slither:4-puppy-raffle-audit analyzed (16 contracts)

## List of Files in Src Folder
4-puppy-raffle-audit/src/PuppyRaffle.sol
## 4-puppy-raffle-audit/README.md summary
The Puppy Raffle smart contract allows users to enter a raffle to win a dog NFT. The protocol includes functions to enter the raffle, ensure unique participants, and facilitate refunds. Additionally, winners are drawn periodically, potentially earning a puppy NFT. The protocol involves a fee structure where the owner sets a feeAddress, and remaining funds go to the winner. 

## Getting Started
To begin, ensure you have git and Foundry installed. Clone the repository, navigate into the directory, and execute the provided make command for setup. Users can optionally work with the repo in Gitpod without local installations.

## Usage
Testing can be executed using Foundry commands like `forge test` and `forge coverage` for coverage testing.

## Audit Scope Details
There is an explicit audit scope focusing on the `PuppyRaffle.sol` file with Solc version 0.7.6, meant for deployment on Ethereum.

## Roles
Key roles include the owner, who deploys the protocol and manages fee addresses, and players, who enter raffles and manage refunds.

## Known Issues
No reported issues.


## 4-puppy-raffle-audit/src/PuppyRaffle.sol summary
The `PuppyRaffle` contract facilitates a raffle for winning a puppy NFT. Participants can enter the raffle by paying an entrance fee. Duplicate entries are not permitted. Refunds are available for participants. Once the raffle duration elapses and there are at least four players, a winner is selected, and a random puppy is minted to the winner. The contract owner receives a percentage cut of the collected entry fees, while the remainder is sent to the winner.

