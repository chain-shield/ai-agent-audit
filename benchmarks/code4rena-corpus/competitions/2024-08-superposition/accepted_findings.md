# Accepted H/M Findings: Superposition

# [H-01] update_emergency_council_7_D_0_C_1_C_58() updates nft manager instead of emergency council

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-08-superposition
- **Source snapshot:** competitions/2024-08-superposition/final_report.html

update_emergency_council_7_D_0_C_1_C_58() updates nft manager instead of emergency council Submitted by ABAIKUNANBAEV, also found by Q7, DadeKuma, ZanyBonzy, Rhaydden, Nikki, nslavchev, Testerbot, eta, d4r3d3v1l, prapandey031, zhaojohnson, oakcobalt, wasm_it, and shaflow2 Inside of lib.rs, there is a function update_emergency_council_7_D_0_C_1_C_58() that is needed to update the emergency council that can disable the pools. However, in the current implementation, nft_manager is updated instead.

## Recommended Mitigation Steps

Change update_emergency_council_7_D_0_C_1_C_58() to update emergency_council.

af-afk (Superposition) confirmed via duplicate issue #64 0xsomeone (judge) commented:

The Warden and its duplicates have correctly identified that the mechanism exposed for updating the emergency_council will incorrectly update the nft_manager instead.

I initially wished to retain a medium risk severity rating for this vulnerability due to how the emergency_council is configured during the contract’s initialization and its value changing being considered a rare event; however, a different highly sensitive variable is altered instead incorrectly ( nft_manager ) which would have significant consequences to the system temporarily.

Based on the above, I believe that a high-risk rating is appropriate due to the unexpected effects invocation of the function would result in.

# [H-02] Unrevoked approvals allow NFT recovery by previous owner

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-08-superposition
- **Source snapshot:** competitions/2024-08-superposition/final_report.html

Submitted by Japy69, also found by zhaojohnson, ZanyBonzy, Testerbot, IzuMan, Shubham, SBSecurity, Nikki, SpicyMeatball, and oakcobalt The vulnerability arises from the fact that after a token transfer, the approval status for the token is not revoked. Specifically, the getApproved[_tokenId] is not updated on transfer. This allows the previous owner (or any approved address) to reclaim the NFT by using the approval mechanism to re-transfer the token back to themselves. This is critical because the new owner of the NFT may lose their asset without realizing it, leading to potential exploitation, loss of assets, and decreased trust in the platform.

Details In the provided approve function, any user can approve themselves or another address for a specific token ID:

/// @inheritdoc IERC721Metadata function approve ( address _approved, uint256 _tokenId ) external payable { _requireAuthorised ( msg.

sender, _tokenId ); getApproved [ _tokenId ] = _approved; } Since the approval is not revoked upon transfer, the previous owner retains the ability to re-transfer the NFT. The _requireAuthorised function is the only check on transfer permission:

function _requireAuthorised ( address _from, uint256 _tokenId ) internal view { // revert if the sender is not authorised or the owner bool isAllowed = msg.

sender == _from || isApprovedForAll [ _from ][ msg.

sender ] || msg.

sender == getApproved [ _tokenId ]; require ( isAllowed, "not allowed" ); require ( ownerOf ( _tokenId ) == _from, "_from is not the owner!" ); }

## Recommended Mitigation Steps

To prevent this vulnerability, any existing approvals should be revoked when a token is transferred. This can be achieved by adding a line in the transfer function to clear the approval:

getApproved [ _tokenId ] = address ( 0 ); This line should be added to the token transfer function to ensure that any previously approved addresses cannot transfer the NFT after it has been sold or transferred to a new owner.

## Assessed type

Token-Transfer af-afk (Superposition) confirmed via duplicate issue #56 0xsomeone (judge) commented:

The submission details how approvals are not cleared whenever an NFT transfer occurs, permitting the NFT to be recovered after it has been transferred which breaks a crucial invariant of NFTs and would affect any and all integrations of the NFT (i.e., staking systems, marketplaces, etc.). As such, I believe a high-risk severity rating is appropriate.

# [H-03] Missing lower<upper check in mint_position

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-08-superposition
- **Source snapshot:** competitions/2024-08-superposition/final_report.html

lower<upper check in mint_position Submitted by Q7, also found by aldarion, Nikki, prapandey031, SBSecurity, DadeKuma, nnez, Silvermist, oakcobalt, shaflow2, ABAIKUNANBAEV, nslavchev, rare_one, OMEN, Testerbot, wasm_it, and devival The current implementation does not perform a check for lower < upper during mint_position, while many other functions assume lower < upper. This discrepancy allows malicious actors to exploit inconsistencies in handling undefined behavior in other parts of the code for their benefit.

Case when lower = upper:

In the StorageTicks::update function, liquidity can be added without issue because only one boundary and the current tick need to be passed, meaning it should function correctly even if the current tick equals the boundary. However, when calculating the amount of tokens required from the user, due to lower = upper, the calculation can only fall into the first and third branches. Here, sqrt_ratio_a_x_96 = sqrt_ratio_a_x_96, and the difference between these two values multiplies the result, leading to a token amount of 0 regardless of liquidity. A malicious user can exploit this implementation discrepancy to open a position with arbitrary liquidity value without consuming any tokens.

/* pkg/seawater/src/pool.rs */ 170 | // calculate liquidity change and the amount of each token we need 171 | if delta != 0 { 172 | let (amount_0, amount_1) = if self.cur_tick.

get ().

sys () < lower {...

183 | 184 | // we're below the range, we need to move right, we'll need more token0 185 | ( 186 | sqrt_price_math::

get_amount_0_delta ( 187 | tick_math::

get_sqrt_ratio_at_tick (lower)?, 188 | tick_math::

get_sqrt_ratio_at_tick (upper)?, 189 | delta, 190 | )?, 191 | I256::

zero (), 192 | ) 193 | } else if self.cur_tick.

get ().

sys () < upper { 194 | // we're inside the range, the liquidity is active and we need both tokens...

224 | } else {...

236 | // we're above the range, we need to move left, we'll need token1 237 | ( 238 | I256::

zero (), 239 | sqrt_price_math::

get_amount_1_delta ( 240 | tick_math::

get_sqrt_ratio_at_tick (lower)?, 241 | tick_math::

get_sqrt_ratio_at_tick (upper)?, 242 | delta, 243 | )?, 244 | ) 245 | }; While the attacker cannot directly profit by closing the position, these positions affect fee calculations. For example, in the
## Recommended Mitigation Steps

Add assert_or!(lower < upper, Error::InvalidTick) check.

## Assessed type

Invalid Validation af-afk (Superposition) confirmed via duplicate issue #59 0xsomeone (judge) commented:

The Warden has properly identified that the creation of a position does not sufficiently validate its lower and upper tick values, permitting positions whereby the ticks are inverted or equal to be created with significant consequences.

I consider a high-risk severity rating appropriate and have penalized submissions that simply identified the vulnerability (i.e., as a medium, or simply noted it with no impact justification) by 25% (i.e., rewarded with a partial reward of 75%).

# [H-04] Position’s owed fees should allow underflow but it reverts instead, resulting in locked funds

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Finding ID:** H-04
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-08-superposition
- **Source snapshot:** competitions/2024-08-superposition/final_report.html

Submitted by DadeKuma, also found by Tricko and SBSecurity The math used to calculate how many fees are owed to the user does not allow underflows, but this is a necessary due to how the Uniswap logic works as fees can be negative. This impact adding/removing funds to a position, resulting in permanently locked funds due to a revert.

Context Similar to Position's fee growth can revert resulting in funds permanently locked but with a different function/root cause, both issues must be fixed separately.

## Recommended Mitigation Steps

Consider using - to let Rust underflow without errors in release mode:

let owed_fees_0 = full_math::mul_div( fee_growth_inside_0 -.checked_sub(info.fee_growth_inside_0.get()) -.ok_or(Error::FeeGrowthSubPos)?, + - info.fee_growth_inside_0.get() U256::from(info.liquidity.get()), full_math::Q128, )?; let owed_fees_1 = full_math::mul_div( fee_growth_inside_1 -.checked_sub(info.fee_growth_inside_1.get()) -.ok_or(Error::FeeGrowthSubPos)?, + - info.fee_growth_inside_1.get() U256::from(info.liquidity.get()), full_math::Q128, )?; Alternatively, consider using wrapping_sub.

## Assessed type

Uniswap 0xsomeone (judge) commented:

The submission and its duplicates properly identify that the position.rs file fails to permit underflows to occur when calculating fees which might result in inaccessible funds. I have personally observed this vulnerability in production, and I am inclined to agree with a high-risk severity rating as positions resulting in the underflow would effectively become permanently inaccessible.

af-afk (Superposition) confirmed

# [H-05] Parameter misordering in fee collection function causes denial of service and fee loss

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Finding ID:** H-05
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-08-superposition
- **Source snapshot:** competitions/2024-08-superposition/final_report.html

Submitted by mashbust, also found by Q7, eta, d4r3d3v1l, DadeKuma, Tricko, and shaflow2

- https://github.com/code-423n4/2024-08-superposition/blob/4528c9d2dbe1550d2660dac903a8246076044905/pkg/seawater/src/lib.rs#L1149
- https://github.com/code-423n4/2024-08-superposition/blob/4528c9d2dbe1550d2660dac903a8246076044905/pkg/seawater/src/lib.rs#L1150

## Impact

A critical bug has been identified in the collect_protocol_7540_F_A_9_F function of the Seawater Automated Market Maker (AMM) protocol. This vulnerability affects the core functionality of fee collection, rendering the system unable to transfer protocol fees from liquidity pools to the Seawater Admin or any designated recipient. The bug stems from incorrect parameter handling during the invocation of the transfer_to_addr function, which results in a complete failure of the fee withdrawal process.

The impact of this issue is severe:

Denial of Service (DoS): The system’s inability to transfer protocol fees halts the entire fee collection process, effectively disabling this core feature. Without this function, the Seawater Admin cannot retrieve fees collected from liquidity pools, meaning the protocol cannot generate or distribute earnings from the token swapping activity in these pools.

Financial Loss: Since the protocol relies on fees for its economic model, this bug causes a direct financial loss. The protocol’s earnings from fees generated by liquidity pools are locked and inaccessible, leading to loss of funds. Over time, the fees accumulate in the pools, but they cannot be withdrawn by the Seawater Admin or any authorized recipient.

Erosion of User Trust: Protocol participants expect proper fee collection and distribution. If this feature fails, users may lose confidence in the protocol’s reliability, leading to reduced engagement or participation. For liquidity providers and investors, this issue directly affects expected returns, as fees cannot be withdrawn.

Long-term Protocol Viability: If not addressed, this bug could harm the protocol’s long-term viability. Since the protocol is unable to collect revenue from its own pools, its ability to cover operational costs or distribute rewards is compromised. This could lead to a shutdown or significant degradation of the protocol’s functionality.

In summary, the failure to collect protocol fees undermines the core value proposition of the AMM protocol, resulting in a loss of funds, DoS of essential functionality, and the potential collapse of the protocol’s economic model if left unresolved.

## Recommended Mitigation Steps

To fix this issue, the arguments passed to the erc20::transfer_to_addr function need to be correctly ordered, ensuring that the token address is provided first, followed by the recipient address. This correction will allow the transfer of protocol fees from the liquidity pools to the designated recipient.

Corrected Code:

erc20::

transfer_to_addr (pool, recipient, U256::

from (token_0))?; erc20::

transfer_to_addr (FUSDC_ADDR, recipient, U256::

from (token_1))?; This update reverses the order of parameters, passing the token address as the first parameter and the recipient address as the second parameter, as expected by the transfer_to_addr function. By implementing this fix, the protocol will correctly transfer collected fees, ensuring that the Seawater Admin or any authorized recipient can successfully withdraw fees from the AMM pools.

Additional Steps to Consider Add Unit Tests: To prevent this issue from recurring, add specific unit tests to validate the correct behavior of the collect_protocol_7540_F_A_9_F function. These tests should confirm that fees can be collected and transferred without errors.

Error Handling Improvements: Consider adding more explicit error messages to the transfer_to_addr function to help identify issues during fee transfers. This could include checking whether the token address and recipient address are valid before proceeding with the transfer.

Gas Efficiency: After resolving this issue, it’s advisable to audit the overall gas usage of the fee collection process to ensure that no unnecessary operations are being performed during the transfer of protocol fees.

## Assessed type

DoS af-afk (Superposition) confirmed via duplicate issue #60 0xsomeone (judge) commented:

The Warden has identified that an incorrect order of arguments in the EIP-20 transfer calls within the lib::collect_protocol_7540_F_A_9_F function will cause a token transfer to be attempted with the recipient address as the “token” thereby failing on each invocation.

I believe a high severity rating is appropriate given that funds are directly lost and are irrecoverable due to an improper implementation of the protocol fee claim mechanism.

# [H-06] get_fee_growth_inside in tick.rs should allow for underflow / overflow but doesn’t

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Finding ID:** H-06
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-08-superposition
- **Source snapshot:** competitions/2024-08-superposition/final_report.html

get_fee_growth_inside in tick.rs should allow for underflow / overflow but doesn’t Submitted by SBSecurity, also found by DadeKuma, ZanyBonzy, aldarion, peanuts, 0xhashiman, Tricko, and Q7 When operations need to calculate the Superposition position’s fee growth, it uses a similar function implemented by uniswap v3.

However, according to this known issue, Uniswap/v3-core#573, the contract implicitly relies on underflow/overflow when calculating the fee growth if underflow is prevented, some operations that depend on fee growth will revert.

## Recommended Mitigation Steps

Use unsafe / unchecked math when calculating the fee growths.

## Assessed type

Under/Overflow af-afk (Superposition) confirmed 0xsomeone (judge) commented:

The submission and its duplicates properly identify that the tick.rs file fails to permit underflows to occur when calculating fees which might result in inaccessible funds. I have personally observed this vulnerability in production, and I am inclined to agree with a high-risk severity rating as positions resulting in the underflow would effectively become permanently inaccessible.

# [H-07] swapOut functions have invalid slippage check, causing user loss of funds

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Finding ID:** H-07
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-08-superposition
- **Source snapshot:** competitions/2024-08-superposition/final_report.html

swapOut functions have invalid slippage check, causing user loss of funds Submitted by oakcobalt, also found by Testerbot In SeawaterAMM.sol, swapOut5E08A399 and swapOutPermit23273373B are intended to allow usdc(token1) -> pool(token0) swap with slippage check. See ISeawaterAMM’s doc.

However, both functions have incorrect slippage checks. We see in swapOut5E08A399 swapAmountOut is used to check with minOut. But swapAmountOut is actually usdc(token1), not the output token(token0). This uses an incorrect variable to check slippage.

function swapOut5E08A399 ( address token, uint256 amountIn, //@audit-info note: this is usdc(token1) uint256 minOut ) external returns ( int256, int256 ) { ( bool success, bytes memory data ) = _getExecutorSwap ().

delegatecall ( abi.

encodeCall ( ISeawaterExecutorSwap.

swap904369BE, ( token, false, int256 ( amountIn ), type ( uint256 ).

max ) ); require ( success, string ( data )); //@audit-info note: swapAmountIn <=> token0, swapAmountOut <=> token1 |> ( int256 swapAmountIn, int256 swapAmountOut ) = abi.

decode ( data, ( int256, int256 ) ); //@audit This should use token0 value, not token1 |> require ( swapAmountOut >= int256 ( minOut ), "min out not reached!" ); return ( swapAmountIn, swapAmountOut ); }

- https://github.com/code-423n4/2024-08-superposition/blob/4528c9d2dbe1550d2660dac903a8246076044905/pkg/sol/SeawaterAMM.sol#L317
For reference, in the swap facet, swap_internal called in the flow returns Ok((amount_0, amount_1)). This means swapAmountOut refers to token1, the input token amount.

//pkg/seawater/src/lib.rs pub fn swap_internal ( pools: & mut Pools, pool: Address, zero_for_one:

bool, amount: I256, price_limit_x96: U256, permit2:

Option <Permit2Args>, ) -> Result <(I256, I256), Revert> { let (amount_0, amount_1, _ending_tick) = pools.pools.

setter (pool).

swap (zero_for_one, amount, price_limit_x96)?;...

Ok((amount_0, amount_1))

- https://github.com/code-423n4/2024-08-superposition/blob/4528c9d2dbe1550d2660dac903a8246076044905/pkg/seawater/src/lib.rs#L194
swapOutPermit23273373B also has the same erroneous slippage check:

//pkg/sol/SeawaterAMM.sol function swapOutPermit23273373B ( address token, uint256 amountIn, uint256 minOut, uint256 nonce, uint256 deadline, uint256 maxAmount, bytes memory sig ) external returns ( int256, int256 ) {...

|> ( int256 swapAmountIn, int256 swapAmountOut ) = abi.

decode ( data, ( int256, int256 )); |> require ( swapAmountOut >= int256 ( minOut ), "min out not reached!" ); return ( swapAmountIn, swapAmountOut ); }

- https://github.com/code-423n4/2024-08-superposition/blob/4528c9d2dbe1550d2660dac903a8246076044905/pkg/sol/SeawaterAMM.sol#L339
Invalid slippage checks will cause users to lose funds during swaps.

## Recommended Mitigation Steps

Consider changing to:...

( int256 swapAmountOut, int256 swapAmountIn ) = abi.

decode ( data, ( int256, int256 ) ); require ( uint256 (- swapAmountOut ) >= minOut, "min out not reached!" ); return ( swapAmountOut, swapAmountIn );

## Assessed type

Error af-afk (Superposition) confirmed via duplicate issue #53 0xsomeone (judge) commented:

The Warden has identified that the slippage protections of the swap-out functions are incorrect and thus ineffective due to validating an incorrect amount, resulting in either incorrect successful executions or incorrect failed executions depending on the price relation between the two assets.

A medium-risk severity rating is appropriate given that a subset of the system’s functionality is affected with slippage being the component affected.

0xsomeone (judge) increased severity to High and commented:

After revisiting this submission in light of the comments shared in Issue #53, I am inclined to upgrade it to a high-risk severity rating as user funds are directly impacted.

Medium Risk Findings (12)

# [M-01] _onTransferReceived() does not work as intended

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-superposition
- **Source snapshot:** competitions/2024-08-superposition/final_report.html

_onTransferReceived() does not work as intended Submitted by adeolu, also found by debo ( 1, 2 ), Tigerfrake, 0xAleko, 0xastronatey, zhaojohnson, nslavchev, ZanyBonzy, Rhaydden, Nikki, ABAIKUNANBAEV, Agontuk, OMEN, Testerbot, mashbust, swapnaliss, Sparrow, NexusAudits, SBSecurity, Bauchibred, Decap, wasm_it, oakcobalt, SpicyMeatball, and dreamcoder _onTransferReceived does not work as intended or described in the function natspec.

It will not revert when the recipient does not implement onERC721Received() function correctly (does not return onERC721Received().selector ).

It will revert when the recipient implements onERC721Received() function correctly and as described/specified by the EIP 712 (returns onERC721Received().selector ).

This will prevent transfers to contracts that have correctly implemented the ERC721TokenReceiver interface to accept safe token transfers via safeTransferFrom().

## Recommended Mitigation Steps

Change the != in the require statement to ==:

require ( data == IERC721TokenReceiver.

onERC721Received.

selector, "bad nft transfer received data" );

## Assessed type

Context af-afk (Superposition) confirmed and commented via duplicate issue #55:

We don’t believe this is a high risk submission since assets cannot be stolen. Maybe transfers to up to spec contracts could not be used, but that’s just frustrating for users.

0xsomeone (judge) decreased severity to Medium and commented:

The submission details how the EIP-721 safe transfers will fail to validate the return data properly, causing EIP-721 callback integrations to fail for the token.

I believe a medium-risk severity rating is better suited for this submission given that assets are not at risk and functionality becomes inaccessible that should normally be circumventable.

# [M-02] bytes data param is not passed to ERC721 recipient as expected by EIP-721

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-superposition
- **Source snapshot:** competitions/2024-08-superposition/final_report.html

bytes data param is not passed to ERC721 recipient as expected by EIP-721 Submitted by adeolu ERC-721 standard has two variations of the safeTransferFrom(), one with no bytes data param and the other with a bytes data param. The second variation that accepts a bytes data param is meant to pass that data param to the token recipient during the IERC721TokenReceiver. onERC721Received() call. This bytes data param is usually an encoding of extra variables which is used by the recipient contract for the extra logic it will perform as it receives a token.

OwnershipNFTs.sol is ERC721 compliant and so it has two safeTransferFrom() functions but the second safeTransferFrom() has a bytes data param which is not passed into the recipient during IERC721TokenReceiver. onERC721Received() call to recipient. It accepts the bytes data param but doesn’t use it.

/// @inheritdoc IERC721Metadata function safeTransferFrom ( address _from, address _to, uint256 _tokenId, bytes calldata /* _data */ ) external payable { _transfer ( _from, _to, _tokenId ); _onTransferReceived ( msg.

sender, _from, _to, _tokenId ); As we can see above, the bytes param is declared as an arbitrary param, perhaps just to conform to IERC721 spec but the bytes param is not used in the logic at all, and it ought to be used as defined here in the EIP.

/// @param data Additional data with no specified format, sent in call to _to For contracts that integrate OwnershipNFTs or receive transfers, if they have logic which require the additional bytes passed in via the second safeTransferFrom(), they will be unable to execute this logic and may reject the transfers; or revert as this bytes data supplied by the caller of the safeTransferFrom() is not passed into them via the IERC721TokenReceiver.onERC721Received() call. Instead, empty bytes is passed into IERC721TokenReceiver.onERC721Received(), as seen here.

Below is a POC which shows how passing empty bytes by default instead of passing the bytes specified by the sender into a tokenReceiver may cause the transfer to fail.

Run with forge test:

// SPDX-License-Identifier: UNLICENSED pragma solidity ^ 0.8.

13; import { Test, console } from "forge-std/Test.sol"; /// @dev Note: the ERC-165 identifier for this interface is 0x150b7a02.

interface IERC721TokenReceiver { /// @notice Handle the receipt of an NFT /// @dev The ERC721 smart contract calls this function on the recipient /// after a `transfer`. This function MAY throw to revert and reject the /// transfer. Return of other than the magic value MUST result in the /// transaction being reverted.

/// Note: the contract address is always the message sender.

/// @param _operator The address which called `safeTransferFrom` function /// @param _from The address which previously owned the token /// @param _tokenId The NFT identifier which is being transferred /// @param _data Additional data with no specified format /// @return `bytes4(keccak256("onERC721Received(address,address,uint256,bytes)"))` /// unless throwing function onERC721Received ( address _operator, address _from, uint256 _tokenId, bytes memory _data ) external returns ( bytes4 ); } /* * OwnershipNFTs is a simple interface for tracking ownership of * positions in the Seawater Stylus contract.

*/ contract OwnershipNFTs { /** * @notice _onTransferReceived by calling the callback `onERC721Received` * in the recipient if they have codesize > 0. if the callback * doesn't return the selector, revert!

* @param _sender that did the transfer * @param _from owner of the NFT that the sender is transferring * @param _to recipient of the NFT that we're calling the function on * @param _tokenId that we're transferring from our internal storage */ // _onTransferReceived() is exact same function that is in the codebase, with no changes to logic whatsoever function _onTransferReceived ( address _sender, address _from, address _to, uint256 _tokenId ) internal { // only call the callback if the receiver is a contract if ( _to.

code.

length == 0 ) return; bytes4 data = IERC721TokenReceiver ( _to ).

onERC721Received ( _sender, _from, _tokenId, // this is empty byte data that can be optionally passed to // the contract we're confirming is able to receive NFTs "" ); //@audit we don't expect execution to get here, revert should happen in the call to _to because of decoding of empty bytes in _to logic // require( // data != IERC721TokenReceiver.onERC721Received.selector, // "bad nft transfer received data" // ); } function safeTransferFrom ( address _from, address _to, uint256 _tokenId, bytes calldata /** _data */ ) external { //_transfer(_from, _to, _tokenId); _onTransferReceived ( msg.

sender, _from, _to, _tokenId ); } contract MockCompliantReceiver is IERC721TokenReceiver { function onERC721Received ( address _operator, address _from, uint256 _tokenId, bytes memory _data ) external returns ( bytes4 ) { //get values in bytes memory _data ( uint timestamp, uint commandID ) = abi.

decode ( _data, ( uint, uint )); return IERC721TokenReceiver.

onERC721Received.

selector; } contract receiverTest is Test { MockCompliantReceiver compliant_receiver; OwnershipNFTs ownershipNft; function setUp () public { //deploy MockCompliantReceiver compliant_receiver = new MockCompliantReceiver (); //deploy ownershipNft contract ownershipNft = new OwnershipNFTs (); } function test_revertForCompliantReceiver () public { //data to be encoded and expected passed into the recipient contract by a sender/user bytes memory data = abi.

encode ( block.

timestamp, 10 ); /* we expect the call to revert because of the decoding of the empty bytes data passed into _to.OnERC721Received() */ vm.

expectRevert (); ownershipNft.

safeTransferFrom ( msg.

sender, payable ( address ( compliant_receiver )), //param `to` is set to be the compliant receiver here 1, data ); }

## Impact

The implementation of the safeTransferFrom() function that accepts additional bytes is not sufficient and not as defined by the EIP-721 spec. The bytes param passed in by a caller is not sent to the recipient contract. Receiving contracts may need to decode this bytes param, but since empty bytes are passed in to recipient by default in OwnershipNfts this will cause reverts/inaccessibility to this function/feature of the ERC721.

## Recommended Mitigation Steps

The issue stems from the _onTransferReceived() not accepting a bytes data param. Modify it to accept it and pass that param into the IERC721TokenReceiver.onERC721Received() call. Then in each safeTransferFrom(), pass the bytes param into the modified _onTransferReceived(). If the safeTransferFrom() accepts no bytes param, pass empty bytes into _onTransferReceived().

function _onTransferReceived ( address _sender, address _from, address _to, uint256 _tokenId, bytes calldata data ) internal {...

bytes4 data = IERC721TokenReceiver ( _to ).

onERC721Received ( _sender, _from, _tokenId, data );...

} /// @inheritdoc IERC721Metadata function safeTransferFrom ( address _from, address _to, uint256 _tokenId ) external payable { _transfer ( _from, _to, _tokenId ); _onTransferReceived ( msg.

sender, _from, _to, _tokenId, "" ); } /// @inheritdoc IERC721Metadata function safeTransferFrom ( address _from, address _to, uint256 _tokenId, bytes calldata _data ) external payable { _transfer ( _from, _to, _tokenId ); _onTransferReceived ( msg.

sender, _from, _to, _tokenId, _data ); }

## Assessed type

ERC721 0xsomeone (judge) decreased severity to Low adeolu (warden) commented:

@0xsomeone - I would like implore you to review your decision on this finding as it is quite different from the findings it was grouped with.

This finding does not just talk about EIP-721 compliance but spotlights an implementation flaw in the callback to the token receiver. The hardcoding of the empty bytes passed to the token receiver when the receiver is a contract obstructs the use case of the ERC721Receiver callback/hook. The use cases of this hooks are Ensure that a contract is ready to receive the erc721 token.

Allow for execution of additional custom logic on receipt of the token.

Allow for sender to communicate with receiving contract via the bytes data that should be passed into it via the callback. This bytes data will then be used in the additional logic to be executed upon the contract receiving the token.

The OwnershipNFT as it is prevents use cases 2 and 3 and in some cases will may prevent 1 because if a contract that has custom logic which requires bytes to be passed in, it will never be ready to receive the token.

Bytes data passed in by a token sender should be propagated to the token receiver contracts. This is the correct, expected and complete implementation of safeTransferFrom() -> receiver.OnERC721Recevied() callback flow.

0xsomeone (judge) increased severity to Medium and commented:

@adeolu - Indeed, this finding seems to have been grouped incorrectly during the validation phase. Per the rationale laid out in Issue #55, the EIP-721 callback feature was deliberately implemented and does not forward the _data payload as described.

A medium severity rating for this submission, similar to #55, is appropriate.

af-afk (Superposition) confirmed

# [M-03] Wrong liquidity formula used

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-superposition
- **Source snapshot:** competitions/2024-08-superposition/final_report.html

Submitted by prapandey031 The protocol provides a function to increment a position’s liquidity:

function incrPositionC3AC7CAA ( address pool, uint256 id, uint256 amount0Min, uint256 amount1Min, uint256 amount0Desired, uint256 amount1Desired ) external returns ( uint256, uint256 ); It is present in the SeawaterAmm.sol:

- https://github.com/code-423n4/2024-08-superposition/blob/main/pkg/sol/SeawaterAMM.sol#L464
function incrPositionC3AC7CAA ( address /* pool */, uint256 /* id */, uint256 /* amount0Min */, uint256 /* amount1Min */, uint256 /* amount0Desired */, uint256 /* amount1Desired */ ) external returns ( uint256, uint256 ) { directDelegate ( _getExecutorUpdatePosition ()); } It calls a function in lib.rs:

- https://github.com/code-423n4/2024-08-superposition/blob/main/pkg/seawater/src/lib.rs#L914
#[allow(non_snake_case)] pub fn incr_position_C_3_A_C_7_C_A_A ( & mut self, pool: Address, id: U256, amount_0_min: U256, amount_1_min: U256, amount_0_desired: U256, amount_1_desired: U256, ) -> Result <(U256, U256), Revert> { self.

adjust_position_internal ( pool, id, amount_0_min, amount_1_min, amount_0_desired, amount_1_desired, false, None, ) } This calls an internal function in lib.rs:

- https://github.com/code-423n4/2024-08-superposition/blob/main/pkg/seawater/src/lib.rs#L730
#[ allow ( clippy::

too_many_arguments )] pub fn adjust_position_internal ( & mut self, pool:

Address, id:

U256, amount_0_min:

U256, amount_1_min:

U256, amount_0_desired:

U256, amount_1_desired:

U256, giving:

bool, permit2:

Option <( Permit2Args, Permit2Args )>, ) -> Result <( U256, U256 ), Revert > { assert_eq_or!( msg::

sender (), self.position_owners.get(id), Error::

PositionOwnerOnly ); let ( amount_0, amount_1 ) = self.

pools.

setter ( pool ).

adjust_position ( id, amount_0_desired, amount_1_desired, giving, )?;.

} This function calls a function in pool.rs:

- https://github.com/code-423n4/2024-08-superposition/blob/main/pkg/seawater/src/pool.rs#L253
pub fn adjust_position ( & mut self, id: U256, amount_0: U256, amount_1: U256, giving:

bool, ) -> Result <(I256, I256), Revert> { // calculate the delta using the amounts that we have here, guaranteeing // that we don't dip below the amount that's supplied as the minimum.

let position = self.positions.positions.

get (id); let sqrt_ratio_x_96 = tick_math::

get_sqrt_ratio_at_tick ( self.

get_cur_tick ().

as_i32 ())?; let sqrt_ratio_a_x_96 = tick_math::

get_sqrt_ratio_at_tick (position.lower.

get ().

as_i32 ())?; let sqrt_ratio_b_x_96 = tick_math::

get_sqrt_ratio_at_tick (position.upper.

get ().

as_i32 ())?; let mut delta = sqrt_price_math::

get_liquidity_for_amounts ( sqrt_ratio_x_96, // cur_tick sqrt_ratio_a_x_96, // lower_tick sqrt_ratio_b_x_96, // upper_tick amount_0, // amount_0 amount_1, // amount_1 )?.

to_i128 ().

map_or_else (|| Err(Error::LiquidityAmountTooWide), Ok)?; if giving { // If we're giving, then we need to take from the delta.

delta = -delta; } #[cfg(feature = "testing-dbg" )] dbg!

(( "inside adjust_position", current_test!

(), sqrt_ratio_x_96.

to_string (), sqrt_ratio_a_x_96.

to_string (), sqrt_ratio_b_x_96.

to_string (), amount_0.

to_string (), amount_1.

to_string (), delta )); // [update_position] should also ensure that we don't do this on a pool that's not currently // running self.

update_position (id, delta) } In the above function at LOC-265, we have:

- https://github.com/code-423n4/2024-08-superposition/blob/main/pkg/seawater/src/pool.rs#L265
let sqrt_ratio_x_96 = tick_math::

get_sqrt_ratio_at_tick ( self.

get_cur_tick ().

as_i32 ())?; Instead of:

let sqrt_price_x_96 = self.sqrt_price.

get (); This passes not the current price but the price of the current tick to the get_liquidity_for_amounts() function.

## Recommended Mitigation Steps

Replace the following in the adjust_position() function below:

pub fn adjust_position( &mut self, id: U256, amount_0: U256, amount_1: U256, giving: bool, ) -> Result<(I256, I256), Revert> { // calculate the delta using the amounts that we have here, guaranteeing // that we don't dip below the amount that's supplied as the minimum.

let position = self.positions.positions.get(id); - let sqrt_ratio_x_96 = tick_math::get_sqrt_ratio_at_tick(self.get_cur_tick().as_i32())?; + let sqrt_ratio_x_96 = self.sqrt_price.get(); let sqrt_ratio_a_x_96 = tick_math::get_sqrt_ratio_at_tick(position.lower.get().as_i32())?; let sqrt_ratio_b_x_96 = tick_math::get_sqrt_ratio_at_tick(position.upper.get().as_i32())?; let mut delta = sqrt_price_math::get_liquidity_for_amounts( sqrt_ratio_x_96, // cur_tick sqrt_ratio_a_x_96, // lower_tick sqrt_ratio_b_x_96, // upper_tick amount_0, // amount_0 amount_1, // amount_1 )?.to_i128().map_or_else(|| Err(Error::LiquidityAmountTooWide), Ok)?; if giving { // If we're giving, then we need to take from the delta.

delta = -delta; } #[cfg(feature = "testing-dbg")] dbg!(( "inside adjust_position", current_test!(), sqrt_ratio_x_96.to_string(), sqrt_ratio_a_x_96.to_string(), sqrt_ratio_b_x_96.to_string(), amount_0.to_string(), amount_1.to_string(), delta )); // [update_position] should also ensure that we don't do this on a pool that's not currently // running self.update_position(id, delta) }

## Assessed type

Math 0xsomeone (judge) commented:

While the submission is elaborate and goes into great detail, I do not believe that a tangible issue can manifest from calculating the square root price from the current tick as it should match the current price whenever this particular function is invoked. As such, I would advise a PoC to be introduced for this particular submission.

prapandey031 (warden) commented:

@0xsomeone - The issue is not a mere use of the price of current_tick instead of the current_sqrt_price. Rather, this report describes how the Superposition protocol uses the wrong Uniswap v3 liquidity math formula.

Difference between current_sqrt_price and price of the current_tick Uniswap v3 wrote a blog describing the relationship between current_tick, current_tick_price, and current_sqrt_price. The blog states that:

This explains why using ticks can be less precise than sqrtPriceX96 in Uniswap v3.

Just like ticks can be in between initialized ticks, ticks can also be in between integers as well! The protocol itself reports the floor ⌊ic⌋ of the current tick, which the sqrtPriceX96 retains.

To simplify, getting the price of the current_tick would not be equal to getting the current_sqrt_price, as Uniswap v3 stores the current_tick as the floor of the tick derived through the current_sqrt_price:

Note: please see scenario in warden’s original comment.

The above is equation 6.8 from Uniswap v3 whitepaper. The example in the blog makes it clear:

current_sqrt_price = 649004842.70137 price using current_tick = 648962487.5642413

# [M-04] Lp’s liquidity may be lost if re-org happens

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-superposition
- **Source snapshot:** competitions/2024-08-superposition/final_report.html

Submitted by zhaojohnson LP holders will remove liquidity and then burn their liquidity position. If re-org happens, malicious users can manipulate the pool’s price to cause user’s removing liquidity revert. Then LP holders’ position will be burned and liquidity will be locked in contract.

## Recommended Mitigation Steps

Add some more check on burn_position_AE401070(), revert if there are some left liquidity.

## Assessed type

Context af-afk (Superposition) confirmed and commented:

We’re going to resolve this by deleting the burn position feature.

0xsomeone (judge) commented:

The Warden has identified a mechanism in the Seawater system that can become harmful if a re-organization occurs, leading to a total loss of the funds associated with the position.

I believe a medium risk severity rating is appropriate given the likelihood of a re-organization simultaneously occurring with the sequence of events outlined (i.e., different blocks that ended up being re-ordered executing the actions outlined) is low.

Silvermist (warden) commented:

Arbitrum’s docs states that a transaction can’t be re-orged, so this should be invalid.

Blckhv (warden) commented:

Yeah, there are no re-orgs in Arbitrum -

- https://abarbatei.xyz/blockchain-reorgs-for-managers-and-auditors#heading-metrics-for-protocols-and-auditors.

0xsomeone (judge) commented:

@Silvermist and @Blckhv - The documentation referenced states that if the L1 re-orgs, then the Arbitrum transactions will re-org as well. This is also referenced in the glossary itself.

The link referenced by @Blckhv also does not say that “re-orgs are not possible”, simply that they have not been observed yet. As such, the original ruling of the submission stands.

Note: For full discussion, see here.

# [M-05] When performing swap and the swap position does not cover swap amount , the base price of sqrt_price is set incorrectly

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-superposition
- **Source snapshot:** competitions/2024-08-superposition/final_report.html

swap and the swap position does not cover swap amount, the base price of sqrt_price is set incorrectly Submitted by 13u9 When performing a swap, a position is searched and if a position with an appropriate price exists, the swap is performed using the token amount of that position. If the positions in the current pool do not cover the swap amount requested by the user, the value of sqrt_price will not be set accurately, which may cause confusion when the next user swaps.

## Recommended Mitigation Steps

Modify to allow setting the value to the price of the last swapped position even if the position amount is insufficient.

@@ -375,9 +375,10 @@ impl StoragePool { // continue swapping while there's tokens left to swap // and we haven't reached the price limit let mut iters = 0; + let mut last_valid_price = state.price; while !state.amount_remaining.is_zero() && state.price != price_limit { iters += 1; - debug_assert!(iters < 500); + // debug_assert!(iters < 500); let step_initial_price = state.price; @@ -479,6 +480,7 @@ impl StoragePool { }; state.liquidity = liquidity_math::add_delta(state.liquidity, liquidity_net)?; + last_valid_price = state.price; } state.tick = match zero_for_one { @@ -493,7 +495,8 @@ impl StoragePool { // write state // update price and tick - self.sqrt_price.set(state.price); + self.sqrt_price.set(if state.price == price_limit { last_valid_price } else { state.price });

+ if state.tick != self.cur_tick.get().sys() { self.cur_tick.set(I32::unchecked_from(state.tick)); } Mitigation Test Result:

Test Code is same above Test Code Running tests/lib.rs (target/debug/deps/lib-28fa4ebf2403ec3f) running 1 test self.sqrt_price: 792281625142643375935439503360 self.sqrt_price: 560222498985353939371108591955 test test_poc1... ok test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 16 filtered out; finished in 0.14s Running tests/pools.rs (target/debug/deps/pools-a3343d45185ff606) Unlike before, it is set to the tick where the last swap occurred.

## Assessed type

Loop af-afk (Superposition) confirmed 0xsomeone (judge) commented:

The Warden has correctly identified that the configuration of the square root price of the pool will be incorrect in case the price limit configured by the user has been achieved as the state’s price is updated on each search iteration rather than being updated on the last valid swapped price.

I believe a medium-risk severity rating is appropriate given that a user can simply swap the price back down to the actual price of the AMM and no active harm may occur to the liquidity within the pool or its assets.

# [M-06] decrPosition09293696 will not work due to incorrect function signature

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Finding ID:** M-06
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-superposition
- **Source snapshot:** competitions/2024-08-superposition/final_report.html

decrPosition09293696 will not work due to incorrect function signature Submitted by ZanyBonzy, also found by shaflow2 and oakcobalt

- https://github.com/code-423n4/2024-08-superposition/blob/4528c9d2dbe1550d2660dac903a8246076044905/pkg/sol/SeawaterAMM.sol#L476-L485
- https://github.com/code-423n4/2024-08-superposition/blob/4528c9d2dbe1550d2660dac903a8246076044905/pkg/seawater/src/lib.rs#L938

## Impact

decrPosition09293696 will not work, breaking SeawaterAMM.sol’s functionality to decrease position.

## Recommended Mitigation Steps

Introduce the pool address parameter:

function decrPosition09293696( + address /* pool */, uint256 /* id */, uint256 /* amount0Min */, uint256 /* amount1Min */, uint256 /* amount0Max */, uint256 /* amount1Max */ ) external returns (uint256, uint256) { directDelegate(_getExecutorUpdatePosition()); }

## Assessed type

Context af-afk (Superposition) confirmed 0xsomeone (judge) commented:

The Warden has correctly identified that the function definitions of the Solidity and Stylus contracts differ, resulting in the relevant functionality of the system being inaccessible.

I believe a medium-risk rating is appropriate for this vulnerability given that it is possible to interact with positions via the adjust position function and thus the functionality is simply inaccessible via the decrease position pathway rather than being inaccessible completely.

# [M-07] Unintended under/overflow of the amount already swapped in/out due to unmatching logic

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Finding ID:** M-07
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-superposition
- **Source snapshot:** competitions/2024-08-superposition/final_report.html

Submitted by DadeKuma A possible wrong amount of the amount already swapped in/out of the input/output asset of a swap due to an unintended under/overflow.

## Recommended Mitigation Steps

Consider using checked_sub instead of - (and checked_add instead of + ):

match exact_in { true => { state.amount_remaining -= I256::unchecked_from(step_amount_in + step_fee_amount); - state.amount_calculated -= I256::unchecked_from(step_amount_out); + state.amount_calculated = state.amount_calculated.checked_sub(I256::unchecked_from(step_amount_out)); } false => { state.amount_remaining += I256::unchecked_from(step_amount_out); - state.amount_calculated += - I256::unchecked_from(step_amount_in + step_fee_amount); + state.amount_calculated = state.amount_calculated.checked_add( + I256::unchecked_from(step_amount_in + step_fee_amount)); }

## Assessed type

Uniswap af-afk (Superposition) confirmed 0xsomeone (judge) commented:

The submission details that the swap step calculations will perform an insecure subtraction or addition which might result in uncaught overflows/underflows occurring that should normally be prevented.

While I am inclined to retain a medium-risk severity rating for this submission for now, I would advise the Warden to substantiate their claim via a PoC that demonstrates the vulnerability in practice. If no PoC is provided, I will re-evaluate after PJQA and potentially downgrade to QA (L) due to insufficient proof.

# [M-08] Users can’t remove liquidity while a pool is disabled

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Finding ID:** M-08
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-superposition
- **Source snapshot:** competitions/2024-08-superposition/final_report.html

Submitted by Silvermist, also found by zhaojohnson, nslavchev, Testerbot, devival, and prapandey031 There is a functionality for disabling a pool in case of unforeseen circumstances or if a problem occurs. While the pool is disabled the users should not be able to do certain actions but removing their liquidity should not be one of them.

When a problem occurs in the pool, users should be able to remove their liquidity, as it may be at risk and they may lose their money. The problem is there is a check that does not allow them to do it.

## Recommended Mitigation Steps

If the passed delta is negative, it means the user is removing liquidity, so check the pool’s status only when the delta is positive.

## Assessed type

Access Control af-afk (Superposition) confirmed 0xsomeone (judge) commented:

The submission and its duplicates have correctly identified that liquidity withdrawals are disallowed when the pool is paused despite what the code’s documentation indicates.

I believe a medium-risk severity rating is appropriate given that important functionality of the protocol is inaccessible in an emergency scenario when it should be accessible.

# [M-09] swap_2 implementation will randomly revert due to improper check, root cause for failed test ethers_suite_uniswap_orchestrated_uniswap_two

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Finding ID:** M-09
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-superposition
- **Source snapshot:** competitions/2024-08-superposition/final_report.html

swap_2 implementation will randomly revert due to improper check, root cause for failed test ethers_suite_uniswap_orchestrated_uniswap_two Submitted by oakcobalt, also found by pipidu83, prapandey031, and Silvermist swap_2 implementation will randomly revert due to improper check, root cause for failed test ethers_suite_uniswap_orchestrated_uniswap_two.

## Recommended Mitigation Steps

Consider removing/relaxing the strict equality check.

af-afk (Superposition) confirmed 0xsomeone (judge) commented:

The submission and its duplicates have correctly identified that a restriction imposed in lib::swap_2_internal will cause proper swaps to fail due to mandating strict equality for the funds consumed during a swap.

I believe a medium risk rating is appropriate given that it will cause a subset of transactions to fail with no harm beyond a Denial-of-Service.

# [M-10] If liquidity is insufficient, users may need to pay more tokens in swap2

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Finding ID:** M-10
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-superposition
- **Source snapshot:** competitions/2024-08-superposition/final_report.html

swap2 Submitted by shaflow2, also found by ZanyBonzy, OMEN, nnez, and oakcobalt

- https://github.com/code-423n4/2024-08-superposition/blob/4528c9d2dbe1550d2660dac903a8246076044905/pkg/seawater/src/lib.rs#L210
- https://github.com/code-423n4/2024-08-superposition/blob/4528c9d2dbe1550d2660dac903a8246076044905/pkg/seawater/src/lib.rs#L290

## Impact

In the swap_2_internal function, if the first pool experiences a liquidity depletion, it could result in amount_in being less than original_amount. However, the contract might still require the user to transfer original_amount of tokens, causing the user to pay more tokens.

## Recommended Mitigation Steps

Since compatibility with Permit2 is required, it is recommended to refund the excess tokens back to the user.

pub fn swap_2_internal_erc20( pools: &mut Pools, from: Address, to: Address, amount: U256, min_out: U256, permit2: Option<Permit2Args>, ) -> Result<(U256, U256), Revert> { let ( original_amount, amount_in, amount_out, _interim_usdc_out, _final_tick_in, _final_tick_out, ) = Self::swap_2_internal(pools, from, to, amount, min_out)?; // transfer tokens erc20::take(from, original_amount, permit2)?; erc20::transfer_to_sender(to, amount_out)?; + if(original_amount > amount_in){ + erc20::transfer_to_sender(to, original_amount - amount_in)?; + } Ok((amount_in, amount_out)) } af-afk (Superposition) confirmed and commented:

See commit.

0xsomeone (judge) commented:

The submission and its duplicates have correctly identified that a swap_2_internal_erc20 operation will result in an overpayment of funds if the input amount is not consumed in full.

I believe a medium risk severity rating to be appropriate for this submission as it can lead to a minor fund loss under certain circumstances for the user.

# [M-11] Volatile pools with higher fee structure cannot be created because of tick_spacing

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Finding ID:** M-11
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-superposition
- **Source snapshot:** competitions/2024-08-superposition/final_report.html

Submitted by wasm_it

- https://github.com/code-423n4/2024-08-superposition/blob/main/pkg/seawater/src/pool.rs#L53
- https://github.com/code-423n4/2024-08-superposition/blob/main/pkg/seawater/src/pool.rs#L67

## Impact

Provision of liquidity for pools with high volatility will not be possible as such pools cannot be initiated with a tick spacing basis point greater than 256 because of the tick_spacing datatype constrained to a u8.

## Recommended Mitigation Steps

Uniswap uses a bigger datatype for this such as a 24-bit integer type. A bigger datatype would be sufficient for this such as u16.

## Assessed type

Context af-afk (Superposition) confirmed and commented:

See commit.

We wound up enforcing a limit on the fee after some discussion that we don’t intend to set fees that high.

0xsomeone (judge) commented:

The submission has correctly identified that the data type used for the tick spacing of AMM pools is insufficient for volatile pools, causing them to become unusable due to their prices moving across ticks too swiftly.

I believe a medium-risk severity rating is appropriate given that the current AMM model is curated for volatile rather than stable pairs, rendering this scenario to have a non-negligible likelihood of manifesting in practice.

# [M-12] No related function to set fee_protocol

- **Contest:** Superposition
- **Slug:** 2024-08-superposition
- **Finding ID:** M-12
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-08-superposition
- **Source snapshot:** competitions/2024-08-superposition/final_report.html

fee_protocol Submitted by shaflow2, also found by prapandey031, zhaojohnson, DadeKuma, peanuts, oakcobalt, and wasm_it There are no related functions to set fee_protocol, which prevents the protocol from accumulating protocol fees.

## Recommended Mitigation Steps

Add the relevant functions to enable protocol fees.

pkg/seawater/src/pool.rs:

+ pub fn set_fee_protocol(&mut self, new_fee_protocol: U256) { + self.fee_protocol.set(new_fee_protocol); + } pkg/seawater/src/lib.rs:

+ #[allow(non_snake_case)] + pub fn set_fee_protocol( + &mut self, + pool: Address, + new_fee_protocol: U256, + ) -> Result<(), Revert> { + assert_eq_or!( + msg::sender(), + self.seawater_admin.get(), + Error::SeawaterAdminOnly + ); + + self.pools.setter(pool).set_fee_protocol(new_fee_protocol); + + Ok(()) + } af-afk (Superposition) confirmed and commented:

See commit.

0xsomeone (judge) commented:

The submission and its duplicates have correctly identified that there is no mechanism to set the protocol fee in the system, causing fees to never accumulate in the current implementation.

I believe a severity of medium is appropriate given that its only harm is prospective earnings for the protocol itself.
