# Rejected Primary Findings: Superposition

# Positions can be re-initialized

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-196
- **Submitter:** 0xhashiman
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/196
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-196.md

## Brief Summary

In position.rs the function `pub fn new` is used the first time a position is created, users normally will call `mint_position_B_C5_B086_D` in lib.rs to handle the logic of position id so that we dont override an already initialized position. In the current design, functions like `new` in `position.rs` and `create_position` in `pool.rs` are marked as pub fn, meaning they are publicly accessible and can be called by any external source they are also without any initialization checks . Unlike private functions, which are restricted to internal use, public functions lack this protection and are exposed to potential misuse. Because these public functions don't include initialization they allow...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_75_group

# Silent Overflow in get_amount_0_delta and get_amount_1_delta

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-207
- **Submitter:** 0xhashiman
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/207
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-207.md

## Brief Summary

In both functions `get_amount_0_delta` and `get_amount_1_delta`, there are unchecked conversion from U256 (which the function _get_amount_0_delta or _get_amount_1_delta returns) to I256. The issue is that the value returned from _get_amount_0_delta can be greater than what I256 represents, which will result in a faulty conversion. As you can see from the function above if the overflow happens this wont revert and will silently returns 0. Impact The impact of this issue as we can see from the

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Delegatecall Vulnerability Leading to Reentrancy Risk in SeawaterAMM.sol

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-126
- **Submitter:** 14Kattel
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/126
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-126.md

## Brief Summary

The identified reentrancy vulnerability in the `swapIn32502CA71`, `swapInPermit2CEAAB576`, `swapOut5E08A399`, and `swapOutPermit23273373B` functions can have severe implications for the smart contract's security and overall integrity: 1. **Financial Losses**: Reentrancy attacks can lead to the unauthorized withdrawal or manipulation of funds. If an attacker can exploit this vulnerability, they could potentially drain significant amounts of tokens or other assets from the contract, causing financial loss to users and the contract owner. 2. **Unintended Behavior**: The vulnerability allows an attacker to recursively call into the affected functions, which may lead to unintended contract state...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_37_group

# transfer() and transferFrom() functions are incorrectly encoded in wasm_erc20

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-100
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/100
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-100.md

## Brief Summary

In the current implementation of `wasm_erc20` module, functions for encoding and calling ERC20 functions are provided. The problem is that when encoding `transfer()` and `transferFrom()`, address type is assumed to have 32 bytes when in reality it has 20 bytes. This can lead to consistently paying more gas when sending transactions as there will be more bytes allocated than needed.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# word_pos and bit_pos are incorrectly derived when flipping the tick

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-116
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/116
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-116.md

## Brief Summary

Currently the function `flip()` is used to toggle a tick on the bitmap. The problem is that, when getting `word_pos` and `bit_pos` values, `spaced_tick` is used instead of using `position()` function from `tick_bitmap()` library. This leads to incorrect values being obtained as the `spaced_tick` (the result of the division of tick and spacing).

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# The proxy contract is not actually upgradeable as it has to be

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-145
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/145
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-145.md

## Brief Summary

Currently the contract realizes Diamond-like faucet pattern for upgradeable contracts as stated by the spec: This pattern is used to update the logic for different facets if needed. However, in the current version of the `SeawaterAMM`, there is no any function to actually update the logic of the facets but the function to upgrade all the facets at once.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# safeTransferFrom() does not actually transfer the NFT

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-169
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/169
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-169.md

## Brief Summary

Currently the `OwnershipNFTs` smart contract is used to make approvals and transfers of the positions by the authorized entities. The problem is that the `transferFrom()` and `safeTransferFrom()` functions call internal `_transfer()` that only checks if the caller is authorized to transfer and then calls `transferPositionEEC7A3CD()` function that removes the current owner of the position and sets a new one. Therefore, the ownership over NFT is not actually transferred as the owner remained the same.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_00_group

# Position is not minted in the `OwnershipNFTs` upon mint in the Rust implementation

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-171
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/171
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-171.md

## Brief Summary

Currently the user can mint a new position using `mintPositionBC5B086D()` function in the proxy which will be delegated later in the implementation. The problem is that in the Rust code a new position is created but the actual ERC721 tokenId is not minted. Therefore, the owner will not be authorized to call any function in the `OwnershipNFTs` as he's not the owner of the tokenId.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# burnPositionAE401070() does not burn ERC721 token

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-179
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/179
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-179.md

## Brief Summary

In the current functionality of the `SeawaterAMM` smart contract, the users can call `burnPositionAE401070()` that will burn the position by delegating the call to the implementation. The problem is that the actual ERC721 token is burnt meaning the position is not fully burnt but just the owner is removed in the Rust implementation code.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# tokenURI does not contain information about position which should not be the case

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-204
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/204
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-204.md

## Brief Summary

The protocol follows UniswapV3 protocol and implements NFT position manager. The problem is that the tokenURI does not contain any information about the position meaning that the user will only have tokenId but the position information will not be stored in the tokenId but somewhere else.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_05_group

# setApprovalForAll() is incompatible with ERC721Metadata

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-74
- **Submitter:** ABAIKUNANBAEV
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/74
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-74.md

## Brief Summary

In the current implementation of `OwnershipNFTs`, `setApprovalForAll()` deviates from the standard IERC721Metadata implementation as it does not check for the operator to be != address(0).

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Liquidity Accounting in Position Updates Leading to Economic Imbalance

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-139
- **Submitter:** Agontuk
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/139
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-139.md

## Brief Summary

The SeawaterAMM , designed for Arbitrum's Stylus environment and based on Uniswap v3, contains an issue in its liquidity management system. The issue lies in the [`StoragePool::update_position()` function](https://github.com/code-423n4/2024-08-superposition/blob/4528c9d2dbe1550d2660dac903a8246076044905/pkg/seawater/src/pool.rs#L90-L251), which is responsible for updating the liquidity of a position by modifying the liquidity at the lower and upper ticks. The core of the AMM's functionality relies on accurate liquidity tracking across different price ranges. Each position in the pool is defined by a lower and upper tick, representing the price range in which the liquidity is concentrated. Wh...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_12_group

# Price incorrectly being considered to be in range and will output non-zero value for token 0 and token 1

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-256
- **Submitter:** Hawkeye
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/256
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-256.md

## Brief Summary

When price is within a range, liquidity is assumed to be active and thus both token 0 and token 1 amounts should be non-zero and when out of range token 0 or token 1 is returned and not both ,but based on the conditions checked for when liquidity is to be changed, both token 0 and token 1 will be returned as non-zero when current tick is equal to lower (it is assumed to be range based on an erroneous condition).

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_08_group

# Improper Memory Layout Handling for returnData

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-101
- **Submitter:** Mrxstrange
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/101
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-101.md

## Brief Summary

In the Solidity function `directDelegate`, there is a potential issue with the memory layout when handling `returnData` using inline assembly. Specifically, the function does not account for the possibility of empty or improperly structured `returnData`, which can lead to out-of-bounds memory access and unexpected reverts. Vulnerable Code: SeawaterAMM.sol#L132 Description: 1. **Memory Layout Issue**: The `returnData` is a `bytes` array that includes a length prefix in the first 32 bytes of memory. When using inline assembly to access the data, the code assumes that the data is always available and has a valid length. However, if `returnData` is empty or contains an invalid length, the assem...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# No Maximum Price Protection in swapIn32502CA71 Exposes Users to Significant Financial Risk

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-215
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/215
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-215.md

## Brief Summary

The `swapIn32502CA71` function sets no upper bound on the price of the swap, potentially allowing for price manipulation or extreme slippage.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Incorrect Swap Direction in swapOutPermit23273373B Function Causes Unintended Token Exchanges

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-227
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/227
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-227.md

## Brief Summary

The `swapOutPermit23273373B` function is intended to swap USDC for another token using Permit2 for approvals. However, the implementation incorrectly sets up the swap direction. In the contract implementation, the function calls `swapPermit2EE84AD91` with `zeroForOne` set to `false`. This setting indicates a swap from the target token to USDC, which is the opposite of the intended behavior as described in the interface.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Unprotected Proxy Routing Enables Arbitrary Function Execution in SeawaterAMM

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-228
- **Submitter:** NexusAudits
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/228
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-228.md

## Brief Summary

The SeawaterAMM contract uses a proxy pattern with a fallback function to route calls to different executor contracts. The routing is based on the third byte of the calldata. The fallback function lacks proper access control. Any external caller can potentially access admin functions or other sensitive operations if they craft the calldata correctly. The function routes calls based solely on the third byte of the calldata, without checking the caller's permissions. This could allow unauthorized access to admin functions or other sensitive operations.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_06_group

# Unnecessary Token Transfer in Quote Function

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-150
- **Submitter:** OMEN
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/150
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-150.md

## Brief Summary

High. The quote function performs an actual token transfer, potentially depleting contract funds or user allowances, and significantly increasing gas costs for what should be a gas-free, read-only operation.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Improper Address Packing in pack_details Function Leading to Potential Data Corruption

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-198
- **Submitter:** Purpledragon
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/198
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-198.md

## Brief Summary

The `pack_details` function in the smart contract code is designed to pack three fields—`tick_lower`, `tick_upper`, and `owner` (an Ethereum address)—into a single `U256` value. However, there is a bug in the way the `owner` address is handled. The improper packing of the 160-bit address (`U160`) into a 256-bit unsigned integer (`U256`) can lead to data corruption or misalignment, causing the contract to behave unexpectedly when reading or processing packed data. --- Vulnerability Detail: In Ethereum, addresses are typically 160-bit values. The function `pack_details` takes an address as input, converts it into a `U160`, and then attempts to pack it into the lower 160 bits of a `U256` varia...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Unsafe Use of unwrap() in Address Conversion Leading to Potential Runtime Panics

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-199
- **Submitter:** Purpledragon
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/199
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-199.md

## Brief Summary

The `emit_campaign_created` and `emit_campaign_updated` functions in the contract code use `unwrap()` to convert a `FixedBytes<8>` identifier into a byte array. This use of `unwrap()` is unsafe and can cause runtime panics if the conversion fails. This practice is risky and can lead to unexpected behavior or crashes, impacting the stability and reliability of the contract. --- **Vulnerability Detail:** The functions `emit_campaign_created` and `emit_campaign_updated` rely on the `unwrap()` method to convert a `FixedBytes<8>` value into a byte array suitable for logging: **Issue:** - **Unwrapping Risk**: The `unwrap()` function is used to extract the value from a `Result` or `Option`. If the...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Vulnerability in storage_load_bytes32 Function Due to Unsafe Pointer Handling

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-200
- **Submitter:** Purpledragon
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/200
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-200.md

## Brief Summary

The `storage_load_bytes32` function in the provided Rust code contains a vulnerability due to unsafe handling of raw pointers. This function reads a value from storage based on a provided key and writes the result to an output pointer. The lack of validation for pointer validity and alignment introduces the risk of undefined behavior, including memory corruption and potential crashes. Vulnerability Detail The `storage_load_bytes32` function assumes that the raw pointers `key` and `out` are valid and correctly sized, and that they point to appropriately allocated and aligned memory. The function's reliance on these assumptions without any validation exposes it to several risks: 1. **Invalid...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# token code length is not checked in wasm_erc20

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-239
- **Submitter:** SBSecurity
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/239
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-239.md

## Brief Summary

`wasm_erc20::call_optional_return` doesn’t check whether the token that is being called has non-zero length and can be used by pool deployed to honeypot the users directly.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Deadline is missing in update_position

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-250
- **Submitter:** SBSecurity
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/250
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-250.md

## Brief Summary

There is no deadline available when position is being modified, which can harm LPs due to the sharp price movements before their transaction gets included.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Missing refund of excess ether

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-13
- **Submitter:** Tigerfrake
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/13
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-13.md

## Brief Summary

Users do not get refunded any excess ether supplied during NFT transfer. As such, the stand to lose these funds.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_28_group

# Storage can be bloated with low liquidity positions

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-66
- **Submitter:** ZanyBonzy
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/66
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-66.md

## Brief Summary

By chance, or most likely in a coordinated effort, protocol storage can be completely bloated by users due to the fact that positions can be minted/created with as little as zero liquidity. This will drastically increase the onchain storage cost causing that the protocol has to pay more to handle it, high costs of the maintainance and potential DOS.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_34_group

# lib.rs contract even with Stylus preventing reentrancy consider adding reentrancy guards to swap_internal function

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-115
- **Submitter:** debo
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/115
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-115.md

## Brief Summary

Detailed description of the impact of this finding. While Stylus may provide some protection against reentrancy attacks, adding explicit reentrancy guards ensures that the contract is fully secure when interacting with external token contracts. Reentrancy could lead to unauthorized withdrawals or other unintended consequences.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_13_group

# StoragePool contract contract has Unencrypted Private Data On-Chain

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-118
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/118
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-118.md

## Brief Summary

Detailed description of the impact of this finding. Severity: Medium Issue Type: Privacy Risk In the StoragePool contract, liquidity and fee-related information, such as the positions (positions), liquidity (liquidity), and fee growth (fee_growth_global_0, fee_growth_global_1), are stored on-chain and accessible publicly. These details can reveal sensitive trading positions and strategies, which may be considered private by liquidity providers and traders. Since all data on-chain is public, competitors could analyze the positions and liquidity strategies of other users. This could lead to competitive disadvantages and reduce user trust if they feel their data is being exposed without encryp...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# wasm_erc20.rs contract has signature malleability vulnerability in take_permit2 function

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-123
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/123
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-123.md

## Brief Summary

Detailed description of the impact of this finding. Severity: Medium Issue Type: Signature Malleability If signature malleability is not addressed, an attacker could modify a valid signature (without invalidating it) to produce a different but valid signature. This would allow an attacker to potentially bypass signature validation and authorization mechanisms.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_76_group

# OwnershipNFTs contract has denial of service gas exhaustion in the ownerOf function

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-146
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/146
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-146.md

## Brief Summary

Detailed description of the impact of this finding. The contract makes external calls to the SEAWATER contract using the staticcall function within ownerOf and balanceOf. If the SEAWATER contract becomes complex or inefficient (e.g., grows to include more data or complex logic), these calls could potentially exceed the block gas limit. This would result in transactions reverting due to insufficient gas, which can deny service to legitimate users attempting to query ownership or balances, effectively leading to a Denial-of-Service (DoS) attack. The functions ownerOf and balanceOf call the external contract SEAWATER using staticcall to retrieve the owner of a position and the balance of posit...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary

# OwnershipNFTs contract has incorrect Authorization Logic in _requireAuthorised Function

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-9
- **Submitter:** debo
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/9
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-9.md

## Brief Summary

Detailed description of the impact of this finding. Title: OwnershipNFTs contract has incorrect Authorization Logic in _requireAuthorised Function • Severity: Medium • Impact: The function redundantly checks ownership after already ensuring the caller is authorized, leading to unnecessary computations. • Status: Unresolved • File: OwnershipNFTs.sol • Lines Affected: 98-107 The _requireAuthorised function in the OwnershipNFTs contract is designed to ensure that the caller is authorized to manage a given token. The function first checks if the caller is the owner of the token, followed by a check to ensure the caller is either the owner or an approved operator. However, the function redundant...

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, edited-by-warden, :robot:_primary, :robot:_15_group

# In some edgecases liquidity 0 will result in division by 0 error

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-45
- **Submitter:** nslavchev
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/45
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-45.md

## Brief Summary

Division by 0 error will cause the revert of the transaction

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_11_group

# Unchecked negation for liquidity in get_amount_0_delta and get_amount_1_delta in sqrt_price_maths

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-48
- **Submitter:** nslavchev
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/48
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-48.md

## Brief Summary

Negating i128::MIN would result in an overflow error

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_17_group

# remove_position in lib.rs does not check that the position id has an owner, which affects accounting when position is transferred

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-105
- **Submitter:** peanuts
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/105
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-105.md

## Brief Summary

Incorrect accounting which will lead to the owner still having a position count but not owning a position.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# burn_position_AE401070 will leave liquidity and fees stuck in the contract if uncollected

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-106
- **Submitter:** peanuts
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/106
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-106.md

## Brief Summary

Users that burns their position or have their position transferred without retrieving their liquidity and fees will result in funds stuck in contract.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_52_group

# Unbound loop enables denial of service

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-108
- **Submitter:** peanuts
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/108
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-108.md

## Brief Summary

**Similar issue as found in [TOB-UNI-006](https://solodit.xyz/issues/swapping-on-zero-liquidity-allows-for-control-of-the-pools-price-trailofbits-uniswap-v3-core-pdf), issue reproduced in context.** The swap function relies on an unbounded loop. An attacker could disrupt swap operations by forcing the loop to go through too many operations, potentially trapping the swap due to a lack of gas.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_29_group

# Permit2 can be frontrunned

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-109
- **Submitter:** peanuts
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/109
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-109.md

## Brief Summary

Attacker can DoS any functions that uses permit2 by extracting the signature parameters from the function call and frontrunning with a direct `permit()`.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Anybody can modify a position and mess with the accumulated position fees because of the lack of access control

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-226
- **Submitter:** peanuts
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/226
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-226.md

## Brief Summary

The position of a user can be manipulated by anyone.

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary, :robot:_26_group

# One token address cannot have different fee tiers

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-232
- **Submitter:** peanuts
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/232
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-232.md

## Brief Summary

Lesser liquidity in the pool for the particular token.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_19_group

# Premature Tick Removal/Deletion

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-85
- **Submitter:** rare_one
- **Claimed severity:** High
- **Final severity:** High
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/85
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-85.md

## Brief Summary

The clear() function in the contract deletes a tick from storage without validating whether it is safe to do so. This can lead to the loss of state or orphaned data tied to the tick, potentially causing unintended consequences in systems that rely on the tick's data for financial calculations or tracking liquidity. The clear() function removes a tick entry from the storage map The clear() function in the provided code does not explicitly check if the tick is fully cleared or safe to delete before removing it from storage. Here are the potential issues arising from this: Incomplete Clearing: The function might delete a tick while it still has associated liquidity or fee information. This cou...

## Rejection Reason

GitHub validation labels: bug, 3 (High Risk), insufficient quality report, :robot:_primary

# Centralization Risk in Proxy Pattern - Single Point of Failure in Proxy Admin Role

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-174
- **Submitter:** swapnaliss
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/174
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-174.md

## Brief Summary

The contract uses a proxy pattern where a single address (proxyAdmin) has the power to update all executor contracts. This creates a central point of failure. [Link To Code](https://github.com/code-423n4/2024-08-superposition/blob/4528c9d2dbe1550d2660dac903a8246076044905/pkg/sol/SeawaterAMM.sol#L115-L125) Impact If the `proxyAdmin` account is compromised, an attacker could replace all executor contracts with malicious versions, potentially leading to loss of all funds managed by the protocol.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary

# Lack of Existence Checks for Tokens

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-182
- **Submitter:** swapnaliss
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/182
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-182.md

## Brief Summary

The contract doesn't check if a token exists before performing operations on it. This could lead to unexpected behavior if non-existent tokens are transferred or approved. [Link To Code](https://github.com/code-423n4/2024-08-superposition/blob/4528c9d2dbe1550d2660dac903a8246076044905/pkg/sol/OwnershipNFTs.sol#L109-L116) Impact Operations on non-existent tokens could succeed when they should fail, potentially leading to inconsistent state or unexpected behavior.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_30_group

# Lack of slippage control for function updatePositionC7F1F740()

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-31
- **Submitter:** zhaojohnson
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/31
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-31.md

## Brief Summary

Lack of slippage control for function update_position_C_7_F_1_F_740(). LP holders may increase/decrease liquidity with one unexpected price.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_77_group

# Users may lose funds if they use permitTransferFrom

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Submission:** V-34
- **Submitter:** zhaojohnson
- **Claimed severity:** Medium
- **Final severity:** Medium
- **Status:** invalid
- **Rejection category:** other
- **Source URL:** https://github.com/code-423n4/2024-08-superposition-validation/issues/34
- **Source snapshot:** competitions/2024-08-superposition/submissions/raw/V-34.md

## Brief Summary

When users try to add liquidity via permit2 way, and the transaction is reverted because of some reasons, for example, slippage control, malicious users can get users' unused signature and transfer users' funds via permit2.

## Rejection Reason

GitHub validation labels: bug, 2 (Med Risk), insufficient quality report, :robot:_primary, :robot:_24_group
