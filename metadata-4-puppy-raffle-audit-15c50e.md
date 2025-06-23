
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

INFO:Slither:. analyzed (16 contracts)

## List of Files in Src Folder
src/PuppyRaffle.sol

## Slither Call Graph


### Functions

id: 2079_functionDelegateCall, contract: Address, name: functionDelegateCall [INTERNAL]
id: 2662_length, contract: EnumerableMap, name: length [INTERNAL]
id: 1591_balanceOf, contract: ERC721, name: balanceOf [EXTERNAL]
id: 1591__beforeTokenTransfer, contract: ERC721, name: _beforeTokenTransfer [INTERNAL]
id: 648_sub, contract: SafeMath, name: sub [INTERNAL]
id: 1591__approve, contract: ERC721, name: _approve [PRIVATE]
id: 281_constructor, contract: ERC165, name: constructor [INTERNAL]
id: 1707_transferFrom, contract: IERC721, name: transferFrom [EXTERNAL]
id: 2079_sendValue, contract: Address, name: sendValue [INTERNAL]
id: 2662__at, contract: EnumerableMap, name: _at [PRIVATE]
id: 1591__setTokenURI, contract: ERC721, name: _setTokenURI [INTERNAL]
id: 1765_symbol, contract: IERC721Metadata, name: symbol [EXTERNAL]
id: 3895_refund, contract: PuppyRaffle, name: refund [PUBLIC]
id: 1591_constructor, contract: ERC721, name: constructor [INTERNAL]
id: 2079_functionCall, contract: Address, name: functionCall [INTERNAL]
id: 3154_remove, contract: EnumerableSet, name: remove [INTERNAL]
id: 2079_functionStaticCall, contract: Address, name: functionStaticCall [INTERNAL]
id: 1707_balanceOf, contract: IERC721, name: balanceOf [EXTERNAL]
id: 648_tryMod, contract: SafeMath, name: tryMod [INTERNAL]
id: 2662_at, contract: EnumerableMap, name: at [INTERNAL]
id: 648_tryMul, contract: SafeMath, name: tryMul [INTERNAL]
id: 1738_totalSupply, contract: IERC721Enumerable, name: totalSupply [EXTERNAL]
id: 2662__get, contract: EnumerableMap, name: _get [PRIVATE]
id: 2662_remove, contract: EnumerableMap, name: remove [INTERNAL]
id: 293_supportsInterface, contract: IERC165, name: supportsInterface [EXTERNAL]
id: 3895__isActivePlayer, contract: PuppyRaffle, name: _isActivePlayer [INTERNAL]
id: 1591_symbol, contract: ERC721, name: symbol [EXTERNAL]
id: 1591__mint, contract: ERC721, name: _mint [INTERNAL]
id: 1591__safeTransfer, contract: ERC721, name: _safeTransfer [INTERNAL]
id: 3895_slitherConstructorVariables, contract: PuppyRaffle, name: slitherConstructorVariables [INTERNAL]
id: 224_constructor, contract: Ownable, name: constructor [INTERNAL]
id: 1707_getApproved, contract: IERC721, name: getApproved [EXTERNAL]
id: 3895_selectWinner, contract: PuppyRaffle, name: selectWinner [EXTERNAL]
id: 2079_isContract, contract: Address, name: isContract [INTERNAL]
id: 114_slitherConstructorConstantVariables, contract: Base64, name: slitherConstructorConstantVariables [INTERNAL]
id: 3895_changeFeeAddress, contract: PuppyRaffle, name: changeFeeAddress [EXTERNAL][OWNER]
id: 3895_constructor, contract: PuppyRaffle, name: constructor [INTERNAL]
id: 2102__msgSender, contract: Context, name: _msgSender [INTERNAL]
id: 1765_tokenURI, contract: IERC721Metadata, name: tokenURI [EXTERNAL]
id: 3895_withdrawFees, contract: PuppyRaffle, name: withdrawFees [EXTERNAL]
id: 3154_at, contract: EnumerableSet, name: at [INTERNAL]
id: 3154__contains, contract: EnumerableSet, name: _contains [PRIVATE]
id: 1591__transfer, contract: ERC721, name: _transfer [INTERNAL]
id: 2102__msgData, contract: Context, name: _msgData [INTERNAL]
id: 1707_isApprovedForAll, contract: IERC721, name: isApprovedForAll [EXTERNAL]
id: 1591__burn, contract: ERC721, name: _burn [INTERNAL]
id: 1591_safeTransferFrom, contract: ERC721, name: safeTransferFrom [EXTERNAL]
id: 1707_safeTransferFrom, contract: IERC721, name: safeTransferFrom [EXTERNAL]
id: 3241_toString, contract: Strings, name: toString [INTERNAL]
id: 2662__length, contract: EnumerableMap, name: _length [PRIVATE]
id: 114_decode, contract: Base64, name: decode [INTERNAL]
id: 2662_get, contract: EnumerableMap, name: get [INTERNAL]
id: 224_owner, contract: Ownable, name: owner [PUBLIC]
id: 1591_ownerOf, contract: ERC721, name: ownerOf [EXTERNAL]
id: 224_transferOwnership, contract: Ownable, name: transferOwnership [PUBLIC][OWNER]
id: 224_renounceOwnership, contract: Ownable, name: renounceOwnership [PUBLIC][OWNER]
id: 2662__tryGet, contract: EnumerableMap, name: _tryGet [PRIVATE]
id: 3154__remove, contract: EnumerableSet, name: _remove [PRIVATE]
id: 281__registerInterface, contract: ERC165, name: _registerInterface [INTERNAL]
id: 3895__baseURI, contract: PuppyRaffle, name: _baseURI [INTERNAL]
id: 1591__exists, contract: ERC721, name: _exists [INTERNAL]
id: 648_add, contract: SafeMath, name: add [INTERNAL]
id: 1707_approve, contract: IERC721, name: approve [EXTERNAL]
id: 3154_length, contract: EnumerableSet, name: length [INTERNAL]
id: 3895_slitherConstructorConstantVariables, contract: PuppyRaffle, name: slitherConstructorConstantVariables [INTERNAL]
id: 1783_onERC721Received, contract: IERC721Receiver, name: onERC721Received [EXTERNAL]
id: 1591_tokenURI, contract: ERC721, name: tokenURI [EXTERNAL]
id: 3154__add, contract: EnumerableSet, name: _add [PRIVATE]
id: 3154__length, contract: EnumerableSet, name: _length [PRIVATE]
id: 1591__isApprovedOrOwner, contract: ERC721, name: _isApprovedOrOwner [INTERNAL]
id: 1591_setApprovalForAll, contract: ERC721, name: setApprovalForAll [EXTERNAL]
id: 648_mul, contract: SafeMath, name: mul [INTERNAL]
id: 2079__verifyCallResult, contract: Address, name: _verifyCallResult [PRIVATE]
id: 1591_tokenByIndex, contract: ERC721, name: tokenByIndex [EXTERNAL]
id: 1738_tokenOfOwnerByIndex, contract: IERC721Enumerable, name: tokenOfOwnerByIndex [EXTERNAL]
id: 2662__remove, contract: EnumerableMap, name: _remove [PRIVATE]
id: 648_tryDiv, contract: SafeMath, name: tryDiv [INTERNAL]
id: 3154_contains, contract: EnumerableSet, name: contains [INTERNAL]
id: 3895_tokenURI, contract: PuppyRaffle, name: tokenURI [PUBLIC]
id: 1591_tokenOfOwnerByIndex, contract: ERC721, name: tokenOfOwnerByIndex [EXTERNAL]
id: 1591_baseURI, contract: ERC721, name: baseURI [PUBLIC]
id: 1591__setBaseURI, contract: ERC721, name: _setBaseURI [INTERNAL]
id: 2662_contains, contract: EnumerableMap, name: contains [INTERNAL]
id: 1707_ownerOf, contract: IERC721, name: ownerOf [EXTERNAL]
id: 1707_setApprovalForAll, contract: IERC721, name: setApprovalForAll [EXTERNAL]
id: 3895_enterRaffle, contract: PuppyRaffle, name: enterRaffle [PUBLIC]
id: 1591_transferFrom, contract: ERC721, name: transferFrom [EXTERNAL]
id: 1591__safeMint, contract: ERC721, name: _safeMint [INTERNAL]
id: 2079_functionCallWithValue, contract: Address, name: functionCallWithValue [INTERNAL]
id: 3154__at, contract: EnumerableSet, name: _at [PRIVATE]
id: 2662_tryGet, contract: EnumerableMap, name: tryGet [INTERNAL]
id: 648_div, contract: SafeMath, name: div [INTERNAL]
id: 1738_tokenByIndex, contract: IERC721Enumerable, name: tokenByIndex [EXTERNAL]
id: 3154_add, contract: EnumerableSet, name: add [INTERNAL]
id: 2662__set, contract: EnumerableMap, name: _set [PRIVATE]
id: 648_tryAdd, contract: SafeMath, name: tryAdd [INTERNAL]
id: 114_encode, contract: Base64, name: encode [INTERNAL]
id: 648_trySub, contract: SafeMath, name: trySub [INTERNAL]
id: 2662__contains, contract: EnumerableMap, name: _contains [PRIVATE]
id: 648_mod, contract: SafeMath, name: mod [INTERNAL]
id: 1591_isApprovedForAll, contract: ERC721, name: isApprovedForAll [EXTERNAL]
id: 1591_name, contract: ERC721, name: name [EXTERNAL]
id: 1591_totalSupply, contract: ERC721, name: totalSupply [EXTERNAL]
id: 1591__checkOnERC721Received, contract: ERC721, name: _checkOnERC721Received [PRIVATE]
id: 3895_getActivePlayerIndex, contract: PuppyRaffle, name: getActivePlayerIndex [EXTERNAL]
id: 1591_getApproved, contract: ERC721, name: getApproved [EXTERNAL]
id: 2662_set, contract: EnumerableMap, name: set [INTERNAL]
id: 1591_approve, contract: ERC721, name: approve [EXTERNAL]
id: 1765_name, contract: IERC721Metadata, name: name [EXTERNAL]
id: 281_supportsInterface, contract: ERC165, name: supportsInterface [EXTERNAL]

### Dot Edges (Caller -> Callee)

224_constructor [INTERNAL] -> 224__msgSender
224_transferOwnership [PUBLIC][OWNER] -> 224_onlyOwner
224_renounceOwnership [PUBLIC][OWNER] -> 224_onlyOwner
2662_tryGet [INTERNAL] -> 2662__tryGet [PRIVATE]
2662_contains [INTERNAL] -> 2662__contains [PRIVATE]
2662_remove [INTERNAL] -> 2662__remove [PRIVATE]
2662_at [INTERNAL] -> 2662__at [PRIVATE]
2662_set [INTERNAL] -> 2662__set [PRIVATE]
2662_length [INTERNAL] -> 2662__length [PRIVATE]
2662_get [INTERNAL] -> 2662__get [PRIVATE]
3895_constructor [INTERNAL] -> 3895_constructor [INTERNAL]
3895_changeFeeAddress [EXTERNAL][OWNER] -> 3895_onlyOwner
3895_tokenURI [PUBLIC] -> 3895__baseURI [INTERNAL]
3895_tokenURI [PUBLIC] -> 3895_name
3895_selectWinner [EXTERNAL] -> 3895_totalSupply
3895_selectWinner [EXTERNAL] -> 3895__safeMint
3895_tokenURI [PUBLIC] -> 3895__exists
3154_remove [INTERNAL] -> 3154__remove [PRIVATE]
3154_add [INTERNAL] -> 3154__add [PRIVATE]
3154_length [INTERNAL] -> 3154__length [PRIVATE]
3154_contains [INTERNAL] -> 3154__contains [PRIVATE]
3154_at [INTERNAL] -> 3154__at [PRIVATE]
3154__add [PRIVATE] -> 3154__contains [PRIVATE]
1591__transfer [INTERNAL] -> 1591_ownerOf [EXTERNAL]
1591_tokenURI [EXTERNAL] -> 1591_baseURI [PUBLIC]
1591__burn [INTERNAL] -> 1591__approve [PRIVATE]
1591_approve [EXTERNAL] -> 1591_ownerOf [EXTERNAL]
1591_approve [EXTERNAL] -> 1591_isApprovedForAll [EXTERNAL]
1591__isApprovedOrOwner [INTERNAL] -> 1591_isApprovedForAll [EXTERNAL]
1591_safeTransferFrom [EXTERNAL] -> 1591__safeTransfer [INTERNAL]
1591_tokenURI [EXTERNAL] -> 1591__exists [INTERNAL]
1591__transfer [INTERNAL] -> 1591__approve [PRIVATE]
1591__checkOnERC721Received [PRIVATE] -> 1591__msgSender
1591_approve [EXTERNAL] -> 1591__approve [PRIVATE]
1591__safeMint [INTERNAL] -> 1591__mint [INTERNAL]
1591__burn [INTERNAL] -> 1591_ownerOf [EXTERNAL]
1591_setApprovalForAll [EXTERNAL] -> 1591__msgSender
1591_safeTransferFrom [EXTERNAL] -> 1591__msgSender
1591_transferFrom [EXTERNAL] -> 1591__isApprovedOrOwner [INTERNAL]
1591__safeMint [INTERNAL] -> 1591__safeMint [INTERNAL]
1591__safeMint [INTERNAL] -> 1591__checkOnERC721Received [PRIVATE]
1591_transferFrom [EXTERNAL] -> 1591__transfer [INTERNAL]
1591__burn [INTERNAL] -> 1591__beforeTokenTransfer [INTERNAL]
1591__isApprovedOrOwner [INTERNAL] -> 1591__exists [INTERNAL]
1591_constructor [INTERNAL] -> 1591__registerInterface
1591_getApproved [EXTERNAL] -> 1591__exists [INTERNAL]
1591__mint [INTERNAL] -> 1591__exists [INTERNAL]
1591__transfer [INTERNAL] -> 1591__beforeTokenTransfer [INTERNAL]
1591__mint [INTERNAL] -> 1591__beforeTokenTransfer [INTERNAL]
1591__setTokenURI [INTERNAL] -> 1591__exists [INTERNAL]
1591__approve [PRIVATE] -> 1591_ownerOf [EXTERNAL]
1591_approve [EXTERNAL] -> 1591__msgSender
1591_safeTransferFrom [EXTERNAL] -> 1591__isApprovedOrOwner [INTERNAL]
1591_transferFrom [EXTERNAL] -> 1591__msgSender
1591__safeTransfer [INTERNAL] -> 1591__transfer [INTERNAL]
1591_safeTransferFrom [EXTERNAL] -> 1591_safeTransferFrom [EXTERNAL]
1591__safeTransfer [INTERNAL] -> 1591__checkOnERC721Received [PRIVATE]
1591__isApprovedOrOwner [INTERNAL] -> 1591_getApproved [EXTERNAL]
1591__isApprovedOrOwner [INTERNAL] -> 1591_ownerOf [EXTERNAL]
281_constructor [INTERNAL] -> 281__registerInterface [INTERNAL]
2079_functionCallWithValue [INTERNAL] -> 2079_isContract [INTERNAL]
2079_functionCallWithValue [INTERNAL] -> 2079__verifyCallResult [PRIVATE]
2079_functionCall [INTERNAL] -> 2079_functionCall [INTERNAL]
2079_functionDelegateCall [INTERNAL] -> 2079_isContract [INTERNAL]
2079_functionCall [INTERNAL] -> 2079_functionCallWithValue [INTERNAL]
2079_functionStaticCall [INTERNAL] -> 2079__verifyCallResult [PRIVATE]
2079_functionCallWithValue [INTERNAL] -> 2079_functionCallWithValue [INTERNAL]
2079_functionStaticCall [INTERNAL] -> 2079_functionStaticCall [INTERNAL]
2079_functionDelegateCall [INTERNAL] -> 2079_functionDelegateCall [INTERNAL]
2079_functionDelegateCall [INTERNAL] -> 2079__verifyCallResult [PRIVATE]
2079_functionStaticCall [INTERNAL] -> 2079_isContract [INTERNAL]
1591__transfer [INTERNAL] -> 2662_set [INTERNAL]
1591_ownerOf [EXTERNAL] -> 2662_get [INTERNAL]
1591__burn [INTERNAL] -> 2662_remove [INTERNAL]
1591__transfer [INTERNAL] -> 3154_remove [INTERNAL]
1591_balanceOf [EXTERNAL] -> 3154_length [INTERNAL]
1591__checkOnERC721Received [PRIVATE] -> 2079_functionCall [INTERNAL]
3895_refund [PUBLIC] -> 2079_sendValue [INTERNAL]
3895_tokenURI [PUBLIC] -> 114_encode [INTERNAL]
1591_totalSupply [EXTERNAL] -> 2662_length [INTERNAL]
1591__exists [INTERNAL] -> 2662_contains [INTERNAL]
1591__burn [INTERNAL] -> 3154_remove [INTERNAL]
1591__mint [INTERNAL] -> 2662_set [INTERNAL]
1591__mint [INTERNAL] -> 3154_add [INTERNAL]
1591_tokenByIndex [EXTERNAL] -> 2662_at [INTERNAL]
1591__transfer [INTERNAL] -> 3154_add [INTERNAL]
1591__checkOnERC721Received [PRIVATE] -> 2079_isContract [INTERNAL]
1591_tokenURI [EXTERNAL] -> 3241_toString [INTERNAL]
1591_tokenOfOwnerByIndex [EXTERNAL] -> 3154_at [INTERNAL]
2079_functionCallWithValue [INTERNAL] -> 2079_isContract [INTERNAL]
2079_functionCallWithValue [INTERNAL] -> 2079__verifyCallResult [PRIVATE]
2079_functionCall [INTERNAL] -> 2079_functionCall [INTERNAL]
2079_functionDelegateCall [INTERNAL] -> 2079_isContract [INTERNAL]
2079_functionCall [INTERNAL] -> 2079_functionCallWithValue [INTERNAL]
2079_functionStaticCall [INTERNAL] -> 2079__verifyCallResult [PRIVATE]
2079_functionCallWithValue [INTERNAL] -> 2079_functionCallWithValue [INTERNAL]
2079_functionStaticCall [INTERNAL] -> 2079_functionStaticCall [INTERNAL]
2079_functionDelegateCall [INTERNAL] -> 2079_functionDelegateCall [INTERNAL]
2079_functionDelegateCall [INTERNAL] -> 2079__verifyCallResult [PRIVATE]
2079_functionStaticCall [INTERNAL] -> 2079_isContract [INTERNAL]
2662_tryGet [INTERNAL] -> 2662__tryGet [PRIVATE]
2662_contains [INTERNAL] -> 2662__contains [PRIVATE]
2662_remove [INTERNAL] -> 2662__remove [PRIVATE]
2662_at [INTERNAL] -> 2662__at [PRIVATE]
2662_set [INTERNAL] -> 2662__set [PRIVATE]
2662_length [INTERNAL] -> 2662__length [PRIVATE]
2662_get [INTERNAL] -> 2662__get [PRIVATE]
3154_remove [INTERNAL] -> 3154__remove [PRIVATE]
3154_add [INTERNAL] -> 3154__add [PRIVATE]
3154_length [INTERNAL] -> 3154__length [PRIVATE]
3154_contains [INTERNAL] -> 3154__contains [PRIVATE]
3154_at [INTERNAL] -> 3154__at [PRIVATE]
3154__add [PRIVATE] -> 3154__contains [PRIVATE]
224_constructor [INTERNAL] -> 224__msgSender
224_transferOwnership [PUBLIC][OWNER] -> 224_onlyOwner
224_renounceOwnership [PUBLIC][OWNER] -> 224_onlyOwner
3895_constructor [INTERNAL] -> 3895_constructor [INTERNAL]
3895_changeFeeAddress [EXTERNAL][OWNER] -> 3895_onlyOwner
3895_tokenURI [PUBLIC] -> 3895__baseURI [INTERNAL]
3895_tokenURI [PUBLIC] -> 3895_name
3895_selectWinner [EXTERNAL] -> 3895_totalSupply
3895_selectWinner [EXTERNAL] -> 3895__safeMint
3895_tokenURI [PUBLIC] -> 3895__exists
1591__transfer [INTERNAL] -> 1591_ownerOf [EXTERNAL]
1591_tokenURI [EXTERNAL] -> 1591_baseURI [PUBLIC]
1591__burn [INTERNAL] -> 1591__approve [PRIVATE]
1591_approve [EXTERNAL] -> 1591_ownerOf [EXTERNAL]
1591_approve [EXTERNAL] -> 1591_isApprovedForAll [EXTERNAL]
1591__isApprovedOrOwner [INTERNAL] -> 1591_isApprovedForAll [EXTERNAL]
1591_safeTransferFrom [EXTERNAL] -> 1591__safeTransfer [INTERNAL]
1591_tokenURI [EXTERNAL] -> 1591__exists [INTERNAL]
1591_approve [EXTERNAL] -> 1591__approve [PRIVATE]
1591__transfer [INTERNAL] -> 1591__approve [PRIVATE]
1591__safeMint [INTERNAL] -> 1591__mint [INTERNAL]
1591__checkOnERC721Received [PRIVATE] -> 1591__msgSender
1591_setApprovalForAll [EXTERNAL] -> 1591__msgSender
1591__burn [INTERNAL] -> 1591_ownerOf [EXTERNAL]
1591_safeTransferFrom [EXTERNAL] -> 1591__msgSender
1591_transferFrom [EXTERNAL] -> 1591__isApprovedOrOwner [INTERNAL]
1591__safeMint [INTERNAL] -> 1591__safeMint [INTERNAL]
1591__safeMint [INTERNAL] -> 1591__checkOnERC721Received [PRIVATE]
1591_transferFrom [EXTERNAL] -> 1591__transfer [INTERNAL]
1591__burn [INTERNAL] -> 1591__beforeTokenTransfer [INTERNAL]
1591__isApprovedOrOwner [INTERNAL] -> 1591__exists [INTERNAL]
1591_constructor [INTERNAL] -> 1591__registerInterface
1591_getApproved [EXTERNAL] -> 1591__exists [INTERNAL]
1591__mint [INTERNAL] -> 1591__exists [INTERNAL]
1591__mint [INTERNAL] -> 1591__beforeTokenTransfer [INTERNAL]
1591__transfer [INTERNAL] -> 1591__beforeTokenTransfer [INTERNAL]
1591__setTokenURI [INTERNAL] -> 1591__exists [INTERNAL]
1591__approve [PRIVATE] -> 1591_ownerOf [EXTERNAL]
1591_approve [EXTERNAL] -> 1591__msgSender
1591_safeTransferFrom [EXTERNAL] -> 1591__isApprovedOrOwner [INTERNAL]
1591_transferFrom [EXTERNAL] -> 1591__msgSender
1591__safeTransfer [INTERNAL] -> 1591__transfer [INTERNAL]
1591_safeTransferFrom [EXTERNAL] -> 1591_safeTransferFrom [EXTERNAL]
1591__safeTransfer [INTERNAL] -> 1591__checkOnERC721Received [PRIVATE]
1591__isApprovedOrOwner [INTERNAL] -> 1591_getApproved [EXTERNAL]
1591__isApprovedOrOwner [INTERNAL] -> 1591_ownerOf [EXTERNAL]
281_constructor [INTERNAL] -> 281__registerInterface [INTERNAL]
## Slither Inheritance Json
{"success": true, "error": null, "results": {"printers": [{"elements": [], "description": "Inheritance\nChild_Contract -> Immediate_Base_Contracts [Not_Immediate_Base_Contracts]\n+ Base64\n\n+ Ownable\n -> Context\n\n+ ERC165\n -> IERC165\n\n+ IERC165\n\n+ SafeMath\n\n+ ERC721\n -> Context, ERC165, IERC721, IERC721Metadata, IERC721Enumerable\n, [IERC165]\n\n+ IERC721\n -> IERC165\n\n+ IERC721Enumerable\n -> IERC721\n, [IERC165]\n\n+ IERC721Metadata\n -> IERC721\n, [IERC165]\n\n+ IERC721Receiver\n\n+ Address\n\n+ Context\n\n+ EnumerableMap\n\n+ EnumerableSet\n\n+ Strings\n\n+ PuppyRaffle\n -> ERC721, Ownable\n, [IERC721Enumerable, IERC721Metadata, IERC721, ERC165, IERC165, Context]\n\n\nBase_Contract -> Immediate_Child_Contracts\n [Not_Immediate_Child_Contracts]\n\n+ Base64\n\n+ Ownable\n -> PuppyRaffle\n\n+ ERC165\n -> ERC721\n, [PuppyRaffle]\n\n+ IERC165\n -> ERC165, IERC721\n, [ERC721, IERC721Enumerable, IERC721Metadata, PuppyRaffle]\n\n+ SafeMath\n\n+ ERC721\n -> PuppyRaffle\n\n+ IERC721\n -> ERC721, IERC721Enumerable, IERC721Metadata\n, [PuppyRaffle]\n\n+ IERC721Enumerable\n -> ERC721\n, [PuppyRaffle]\n\n+ IERC721Metadata\n -> ERC721\n, [PuppyRaffle]\n\n+ IERC721Receiver\n\n+ Address\n\n+ Context\n -> Ownable, ERC721\n, [PuppyRaffle]\n\n+ EnumerableMap\n\n+ EnumerableSet\n\n+ Strings\n\n+ PuppyRaffle\n", "markdown": "Inheritance\nChild_Contract -> Immediate_Base_Contracts [Not_Immediate_Base_Contracts]\n+ Base64\n\n+ Ownable\n -> Context\n\n+ ERC165\n -> IERC165\n\n+ IERC165\n\n+ SafeMath\n\n+ ERC721\n -> Context, ERC165, IERC721, IERC721Metadata, IERC721Enumerable\n, [IERC165]\n\n+ IERC721\n -> IERC165\n\n+ IERC721Enumerable\n -> IERC721\n, [IERC165]\n\n+ IERC721Metadata\n -> IERC721\n, [IERC165]\n\n+ IERC721Receiver\n\n+ Address\n\n+ Context\n\n+ EnumerableMap\n\n+ EnumerableSet\n\n+ Strings\n\n+ PuppyRaffle\n -> ERC721, Ownable\n, [IERC721Enumerable, IERC721Metadata, IERC721, ERC165, IERC165, Context]\n\n\nBase_Contract -> Immediate_Child_Contracts\n [Not_Immediate_Child_Contracts]\n\n+ Base64\n\n+ Ownable\n -> PuppyRaffle\n\n+ ERC165\n -> ERC721\n, [PuppyRaffle]\n\n+ IERC165\n -> ERC165, IERC721\n, [ERC721, IERC721Enumerable, IERC721Metadata, PuppyRaffle]\n\n+ SafeMath\n\n+ ERC721\n -> PuppyRaffle\n\n+ IERC721\n -> ERC721, IERC721Enumerable, IERC721Metadata\n, [PuppyRaffle]\n\n+ IERC721Enumerable\n -> ERC721\n, [PuppyRaffle]\n\n+ IERC721Metadata\n -> ERC721\n, [PuppyRaffle]\n\n+ IERC721Receiver\n\n+ Address\n\n+ Context\n -> Ownable, ERC721\n, [PuppyRaffle]\n\n+ EnumerableMap\n\n+ EnumerableSet\n\n+ Strings\n\n+ PuppyRaffle\n", "first_markdown_element": "", "id": "a9bf5482188546bc97936983490d3a6a09db994ce1f14217420f872735b57aff", "additional_fields": {"child_to_base": {"Base64": {"immediate": [], "not_immediate": []}, "Ownable": {"immediate": ["Context"], "not_immediate": []}, "ERC165": {"immediate": ["IERC165"], "not_immediate": []}, "IERC165": {"immediate": [], "not_immediate": []}, "SafeMath": {"immediate": [], "not_immediate": []}, "ERC721": {"immediate": ["Context", "ERC165", "IERC721", "IERC721Metadata", "IERC721Enumerable"], "not_immediate": ["IERC165"]}, "IERC721": {"immediate": ["IERC165"], "not_immediate": []}, "IERC721Enumerable": {"immediate": ["IERC721"], "not_immediate": ["IERC165"]}, "IERC721Metadata": {"immediate": ["IERC721"], "not_immediate": ["IERC165"]}, "IERC721Receiver": {"immediate": [], "not_immediate": []}, "Address": {"immediate": [], "not_immediate": []}, "Context": {"immediate": [], "not_immediate": []}, "EnumerableMap": {"immediate": [], "not_immediate": []}, "EnumerableSet": {"immediate": [], "not_immediate": []}, "Strings": {"immediate": [], "not_immediate": []}, "PuppyRaffle": {"immediate": ["ERC721", "Ownable"], "not_immediate": ["IERC721Enumerable", "IERC721Metadata", "IERC721", "ERC165", "IERC165", "Context"]}}, "base_to_child": {"Base64": {"immediate": [], "not_immediate": []}, "Ownable": {"immediate": ["PuppyRaffle"], "not_immediate": []}, "ERC165": {"immediate": ["ERC721"], "not_immediate": ["ERC721"]}, "IERC165": {"immediate": ["ERC165", "IERC721"], "not_immediate": ["ERC165", "IERC721"]}, "SafeMath": {"immediate": [], "not_immediate": []}, "ERC721": {"immediate": ["PuppyRaffle"], "not_immediate": []}, "IERC721": {"immediate": ["ERC721", "IERC721Enumerable", "IERC721Metadata"], "not_immediate": ["ERC721", "IERC721Enumerable", "IERC721Metadata"]}, "IERC721Enumerable": {"immediate": ["ERC721"], "not_immediate": ["ERC721"]}, "IERC721Metadata": {"immediate": ["ERC721"], "not_immediate": ["ERC721"]}, "IERC721Receiver": {"immediate": [], "not_immediate": []}, "Address": {"immediate": [], "not_immediate": []}, "Context": {"immediate": ["Ownable", "ERC721"], "not_immediate": ["Ownable", "ERC721"]}, "EnumerableMap": {"immediate": [], "not_immediate": []}, "EnumerableSet": {"immediate": [], "not_immediate": []}, "Strings": {"immediate": [], "not_immediate": []}, "PuppyRaffle": {"immediate": [], "not_immediate": []}}}, "printer": "inheritance"}]}}

## README.md summary
The Puppy Raffle project allows participants to enter a raffle to win a dog NFT by calling the `enterRaffle` function with a list of participant addresses. Duplicate addresses cannot enter, and refunds are available through the `refund` function. Periodically, a winner is drawn who receives a puppy NFT, while the owner sets a fee address to collect a portion of the entry value. 

### Getting Started
The project requires git and foundry. It can be started by cloning the repository and running `make` to compile the code.

### Usage
Testing is performed with `forge test`, and coverage is checked using `forge coverage`. The audit is scoped to the `PuppyRaffle.sol` file, compatible with Solc version 0.7.6, and intended for deployment on Ethereum.

### Roles
The Owner, or protocol deployer, can change the fee address, while Players enter the raffle and can request refunds.

### Known Issues
Currently, there are no known issues with the Puppy Raffle protocol.


## src/PuppyRaffle.sol summary
### Contract: `PuppyRaffle`

The `PuppyRaffle` contract is a raffle system that allows users to enter for a chance to win a puppy NFT. Users pay an entrance fee to participate, and a random winner is selected after a specified raffle duration. Notably, the owner can set a fee address to receive part of the entrance fees as commission.

#### Key Components:
- **`entranceFee`**: Entry cost in wei to join the raffle.
- **`players`**: List of participants.
- **`raffleDuration`**: Duration of the raffle in seconds.
- **`raffleStartTime`**: Timestamp when the raffle started.
- **`previousWinner`**: Winner of the last raffle.
- **`feeAddress`**: Address where the fees are sent.
- **`totalFees`**: Total gathered fees.

### Functions Summary:

#### `constructor(uint256 _entranceFee, address _feeAddress, uint256 _raffleDuration)`

**Purpose**: Initializes the contract with entry fee, fee address, and raffle duration.

#### `enterRaffle(address[] memory newPlayers)`

**Purpose**: Allows players to join the raffle by paying the entrance fee and ensures no duplicate entries.

#### `refund(uint256 playerIndex)`

**Purpose**: Enables players to reclaim their entrance fee by providing their index in the player list.

#### `getActivePlayerIndex(address player)`

**Purpose**: Retrieves the index of a specific player in the active player list.

#### `selectWinner()`

**Purpose**: Selects a random winner from the participants, mints the NFT, and rewards the winner after the raffle duration.

#### `withdrawFees()`

**Purpose**: Transfers the collected fees to the designated feeAddress.

#### `changeFeeAddress(address newFeeAddress)`

**Purpose**: Allows the contract owner to modify the fee address.

#### `tokenURI(uint256 tokenId)`

**Purpose**: Provides the metadata URI for a given token based on its rarity.

