# Accepted H/M Findings: Decent

# [H-01] Anyone can update the address of the Router in the DcntEth contract to any address they would like to set.

- **Contest:** Decent
- **Slug:** 2024-01-decent
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-01-decent
- **Source snapshot:** competitions/2024-01-decent/final_report.html

Submitted by NPCsCorp, also found by seraviz, dutra, 0xSimeon, EV_om, azanux, Aymen0909, 0xprinc, ZdravkoHr, 0x11singh99, CDSecurity, nuthan2x, GhK3Ndf, 0xAadi, Eeyore, ZanyBonzy ( 1, 2 ), DadeKuma, Matue, Timeless, Giorgio, slylandro_star, 0xdice91, Nikki, ke1caM, cu5t0mpeo, Greed, nobody2018, Tendency, Inference, al88nsk, DarkTower, th13vn, Soliditors, Timenov, wangxx2026, NentoR, ether_sky, peanuts, MrPotatoMagic, ravikiranweb3, mrudenko, Kaysoft, deth, 0xBugSlayer, nmirchev8, GeekyLumberjack, Aamir, adeolu, stealth, simplor, PUSH0, 0xabhay, darksnow, haxatron, m4ttm, 0xE1, boredpukar, abiih, 0xSmartContract, bareli, mgf15 ( 1, 2, 3, 4 ), vnavascues, d4r3d3v1l, zaevlad, 0xPluto, rouhsamad, Krace, kodyvim, Tigerfrake, JanuaryPersimmon2024, and piyushshukla By allowing anybody to set the address of the Router contract to any address they want to set it allows malicious users to get access to the mint and burn functions of the DcntEth contract.

## Recommended Mitigation Steps

Make sure to add an Acess Control mechanism to limit who can set the address of the Router in the DcnEth contract.

0xsomeone (Judge) commented:

This and all relevant submissions correctly specify that the lack of access control in the DcntEth::setRouter function can be exploited maliciously and effectively compromise the entire TVL of the Decent ETH token.

A high-risk severity is appropriate, and this submission was selected as the best due to detailing all possible impacts:

Arbitrary mints of the token to withdraw funds provided as liquidity to UTB Arbitrary burns to sabotage liquidity pools and other escrow-based contracts Sabotage of liquidity provision function invocations wkantaros (Decent) confirmed

# [H-02] Due to missing checks on minimum gas passed through LayerZero, executions can fail on the destination chain

- **Contest:** Decent
- **Slug:** 2024-01-decent
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-01-decent
- **Source snapshot:** competitions/2024-01-decent/final_report.html

Submitted by iamandreiski, also found by NPCsCorp, EV_om, windhustler ( 1, 2 ), and nuthan2x

- https://github.com/decentxyz/decent-bridge/blob/7f90fd4489551b69c20d11eeecb17a3f564afb18/src/DecentEthRouter.sol#L148-L194
- https://github.com/decentxyz/decent-bridge/blob/7f90fd4489551b69c20d11eeecb17a3f564afb18/src/DecentEthRouter.sol#L80-L111
In LayerZero, the destination chain’s function call requires a specific gas amount; otherwise, it will revert with an out-of-gas exception. It falls under the responsibility of the User Application to ensure that appropriate limits are established. These limits guide relayers in specifying the correct gas amount on the source chain, preventing users from inputting insufficient values for gas.

The contract logic in DecentEthRouter, assumes that a user will first get their estimated fees through estimateSendAndCallFee() and pass it as an argument in either bridge() or bridgeWithPayload() to be added to the calculation together with the hardcoded GAS_FOR_RELAY so that it can be passed as the adapter params when CommonOFT.LzCallParams is called, although this is not enforced and is left on the user’s responsibility.

A user can pass an arbitrary value as the _dstGasForCall argument to be added to the hardcoded GAS_FOR_RELAY fee, thus sending less gas than required which can lead to out-of-gas exceptions.

Once the message is received by destination, the message is considered delivered (transitioning from INFLIGHT to either SUCCESS or STORED), even though it threw an out-of-gas error.

Any uncaught errors/exceptions (including out-of-gas) will cause the message to transition into STORED. A STORED message will block the delivery of any future message from source to all destination on the same destination chain and can be retried until the message becomes SUCCESS.

As per:

- https://layerzero.gitbook.io/docs/faq/messaging-properties

## Recommended Mitigation Steps

Validate/require that the _dstGasForCall parameter is greater than nativeFee + zroFee or re-engineer the architecture to make the estimateSendAndCallFee() function a mandatory step of the process.

0xsomeone (Judge) increased severity to High and commented:

This and all duplicate exhibits highlight that the GAS_FOR_RELAY is a hard-coded value and that the overall gas supplied for a cross-chain call can be controlled by a user.

A severity of high is appropriate given that the cross-chain LayerZero channel will be permanently blocked.

None of the submissions have correctly proposed a solution as a mere adjustment of the GAS_FOR_RELAY is insufficient. The DecentBridgeExecutor permits arbitrary calls to be made that can force the transaction to run out-of-gas regardless of the gas limit imposed. This is properly defined in #697.

A valid solution for this problem would be a combination of a minimum enforced at the transaction level and a maximum gas consumed enforced at the executor level, ensuring that the gas remainder after the executor performs the arbitrary call is enough to store the failed message. This can be achieved by performing a subtraction from the gasleft value (hard to implement as it would need to take into account the cost of keccak256 encoding the data payload) or by enforcing a fixed value that should be much less than the minimum imposed on the source chain.

This submission was selected as the best given that it illustrates in-depth knowledge of the LayerZero system states and correctly highlights that a user can also maliciously block the channel.

wkantaros (Decent) acknowledged via duplicate #212, but disagreed with severity and commented:

This vulnerability is not a concern in Layer Zero v2. Decent designed the contracts expecting to use LZ v2 and have since implemented this upgrade.

# [H-03] When DecentBridgeExecutor.execute fails, funds will be sent to a random address

- **Contest:** Decent
- **Slug:** 2024-01-decent
- **Finding ID:** H-03
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-01-decent
- **Source snapshot:** competitions/2024-01-decent/final_report.html

DecentBridgeExecutor.execute fails, funds will be sent to a random address Submitted by DadeKuma, also found by NPCsCorp, MrPotatoMagic, SBSecurity, deth, nmirchev8, Tendency, ether_sky, Kow, haxatron, EV_om, 0xJaeger, ZdravkoHr, Giorgio, Soliditors, Aamir, Eeyore, Inference, and kutugu

- https://github.com/decentxyz/decent-bridge/blob/7f90fd4489551b69c20d11eeecb17a3f564afb18/src/DecentEthRouter.sol#L101-L105
- https://github.com/decentxyz/decent-bridge/blob/7f90fd4489551b69c20d11eeecb17a3f564afb18/src/DecentBridgeExecutor.sol#L63
When the DecentBridgeExecutor._executeWeth/_executeEth target call fails, a refund is issued to the from address.

However, this address is wrongly set, so those refunds will be permanently lost.

## Recommended Mitigation Steps

The executor.execute call in DecentEthRouter.onOFTReceived should be changed to an appropriate address ( e.g. the user refund address ) instead of using _from:

} else { weth.

approve ( address ( executor ), _amount ); executor.

execute ( _from, _to, deliverEth, _amount, callPayload ); } 0xsomeone (Judge) commented:

The Warden has detailed how the encoding of the cross-chain payload will use an incorrect _from parameter under normal operating conditions, leading to failed transfers at the target chain refunding the wrong address.

This submission was selected as the best given that it precisely details that the _from address is known to be incorrect at all times when the protocol is used normally.

A high-risk rating is appropriate as any failed call will lead to full fund loss for the cross-chain call.

wkantaros (Decent) confirmed

# [H-04] Users will lose their cross-chain transaction if the destination router do not have enough WETH reserves.

- **Contest:** Decent
- **Slug:** 2024-01-decent
- **Finding ID:** H-04
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-01-decent
- **Source snapshot:** competitions/2024-01-decent/final_report.html

Submitted by haxatron, also found by EV_om, MrPotatoMagic, deth, rouhsamad, Aamir, Topmark, and bart1e When the DecentEthRouter receives the dcntEth OFT token from a cross-chain transaction, if the WETH balance of the destination router is less than amount of dcntEth received (this could be due to the router receiving more cross-chain transactions than than sending cross-chain transactions which depletes its WETH reserves), then the dcntEth will get transferred to the address specified by _to.

DecentEthRouter.sol#L266-L281 function onOFTReceived ( uint16 _srcChainId, bytes calldata, uint64, bytes32, uint _amount, bytes memory _payload ) external override onlyLzApp {...

if ( weth.

balanceOf ( address ( this )) < _amount ) { => dcntEth.

transfer ( _to, _amount ); return; } if ( msgType == MT_ETH_TRANSFER ) { if (!

gasCurrencyIsEth || !

deliverEth ) { weth.

transfer ( _to, _amount ); } else { weth.

withdraw ( _amount ); payable ( _to ).

transfer ( _amount ); } else { weth.

approve ( address ( executor ), _amount ); executor.

execute ( _from, _to, deliverEth, _amount, callPayload ); } This dcntEth is sent to the user so that they can either redeem the WETH / ETH from the router once the WETH balance is refilled or send it back to the source chain to redeem back the WETH.

The problem is that if the msgType != MT ETH TRANSFER, then the _to address is not the user, it is instead the target meant to be called by the destination chain’s bridge executor (if the source chain uses a decent bridge adapter, the target is always the destination chain’s bridge adapter which does not have a way to withdraw the dcntEth).

The following snippet shows what occurs in the bridge executor ( _executeEth omitted as it does largely the same thing as _executeWeth ):

DecentBridgeExecutor.sol#L24-L82 function _executeWeth ( address from, address target, uint256 amount, bytes memory callPayload ) private { uint256 balanceBefore = weth.

balanceOf ( address ( this )); weth.

approve ( target, amount ); ( bool success, ) = target.

call ( callPayload ); if (!

success ) { weth.

transfer ( from, amount ); return; } uint256 remainingAfterCall = amount - ( balanceBefore - weth.

balanceOf ( address ( this ))); // refund the sender with excess WETH weth.

transfer ( from, remainingAfterCall ); }...

function execute ( address from, address target, bool deliverEth, uint256 amount, bytes memory callPayload ) public onlyOwner { weth.

transferFrom ( msg.

sender, address ( this ), amount ); if (!

gasCurrencyIsEth || !

deliverEth ) { _executeWeth ( from, target, amount, callPayload ); } else { _executeEth ( from, target, amount, callPayload ); } Therefore, once the dcntEth is transferred to the execution target (which is almost always the destination chain bridge adapter, see Appendix for the code walkthrough). The user cannot do anything to retrieve the dcntEth out of the execution target, so the cross-chain transaction is lost.

## Recommended Mitigation Steps

Pass a destination chain refund address into the payload sent cross-chain and replace the _to address used in DecentEthRouter.sol#L267:

if ( weth.

balanceOf ( address ( this )) < _amount ) { // REPLACE '_to' with the destination chain refund address => dcntEth.

transfer ( _to, _amount ); return; }

# [M-01] Permanent loss of tokens if swap data gets outdated

- **Contest:** Decent
- **Slug:** 2024-01-decent
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-decent
- **Source snapshot:** competitions/2024-01-decent/final_report.html

Submitted by windhustler, also found by monrel, nuthan2x, and imare While sending funds through StargateBridgeAdapter, the user passes the swap data as a parameter. The flow is stargate sending tokens on the receiving chain into the StargateBridgeAdapter and executing sgReceive.

The issue here is that sgReceive will fail if the swap data gets outdated, but this is not going to make the whole transaction revert.

Stargate will still send the tokens to the StargateBridgeAdapter, and if the sgReceive fails, the swap will be cached for later execution. See StargateComposer logic here:

- https://stargateprotocol.gitbook.io/stargate/stargate-composability/stargatecomposer.sol#sgreceive.

Now, tokens will be left sitting in the StargateBridgeAdapter contract, and since the user can only retry the transaction with the same swap data, the tokens will be stuck forever.

The impact is loss of transferred tokens for the user.

## Recommended Mitigation Steps

Wrap the whole StargateBridgeAdapter:receiveFromBridge() call into a try/catch and if it reverts send the transferred tokens back to the user.

wkantaros (Decent) confirmed 0xsomeone (Judge) decreased severity to Medium and commented:

The Warden has demonstrated that it is possible for a Stargate-based cross-chain interaction to fail at the swap level perpetually.

While the StargateComposer::clearCachedSwap function exists to retry the same payload, as the Warden states, a sharp market event (i.e. token launch that leads to an upward trend or market crash that leads to a downward event) can cause the swap to fail.

Theoretically, the tokens can be rescued using a flash-loan if they are substantial, however, this would be very unconventional and not a real mitigation to the issue. The proposed solution by the Warden is adequate.

I believe a severity of Medium is more appropriate as the vulnerability relies on external requirements (i.e. a sharp market event), the maximum impact is the user’s own funds, and when users specify slippage (i.e. minimum output) they usually factor in the time it takes for a transaction to execute. Network congestion, market events, and cross-chain relay delays all play a role in whether the vulnerability manifests but present external requirements.

Note: For full discussion, see here.

# [M-02] Users can use the protocol freely without paying any fees by calling the DecentEthRouter::bridgeWithPayload() function directly.

- **Contest:** Decent
- **Slug:** 2024-01-decent
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-decent
- **Source snapshot:** competitions/2024-01-decent/final_report.html

DecentEthRouter::bridgeWithPayload() function directly.

Submitted by NPCsCorp, also found by Soliditors, peanuts, nmirchev8, and haxatron The execution flow of bridgeAndExecute function To understand the vulnerability, we need to understand the execution flow of the bridgeAndExecute() function, at least a small portion of it.

When the user wants to bridge tokens of him and execute an action on another chain, he will need to execute the UTB::bridgeAndExecute() function.

Suppose the user exists in Polygon, he has USDC and he wants to mint an NFT in Optimism which costs 1000 DAI. What will happen is that the protocol will first, in polygon, swap the user’s USDC with WETH, then bridge the WETH to Optimism, then swap the WETH with DAI and then execute the arbitrary call the user wants to execute, which will be to mint the NFT in exchange for the resulting 1000 DAI from the post-bridge swap operation.

When this function is called, the following will happen:

Step 1: When the user calls the UTB::bridgeAndExecute() function, it will do three things: first, it will collect the fees by calling the UTBFeeCollector:collectFees() function, secondly, it will conduct the pre-bridge swap operation (occurs in the source destination), it will swap the user’s USDC to WETH. thirdly, it will modify the swapInstructions which the user supplied to prepare for the post-bridge swap. Then after all of the 3 operations take place, it will invoke the UTB::callBridge() function.

Step 2: In the UTB::callBridge() function, some approvals are granted to the DecentBridgeAdapter contract, and then the it will invoke the function DecentBridgeAdapter::bridge() in the DecentBridgeAdapter contract.

Step 3: In the DecentBridgeAdapter::bridge() function, some data like the post-bridge swap payload and bridge payload (what to execute when the TX reaches destination) will be encoded, then it will reach out to the DecentEthRouter contract and invoke the function DecentEthRouter::bridgeWithPayload Step 4: When the execution reaches the DecentEthRouter::bridgeWithPayload function, an internal function containing the actual logic, with the same name will also be called:

DecentEthRouter::_bridgeWithPayload Note: Notice that the `DecentEthRouter::bridgeWithPayload() function isn’t protected by any modifiers, any body can call it directly Step 5: When the execution gets inside the DecentEthRouter::_bridgeWithPayload function, the function will prepare the LzCallParams for the layerzero call and the actual bridging will happen when the dcntEth::sendAndCall function is actually invoked.

Step 6: The bridging process kickstarts and the execution flow is continued in the destination chain.

Here is a graph of the execution flow Note: to view the provided image, please see the original submission here.

## Impact

Users can use the protocol without paying any fees.

## Recommended Mitigation Steps

Tighten up the access control on the DecentEthRouter::bridgeWithPayload function. Allow only the DecentBridgeAdapter to call the bridgeWithPayload() function.

Found & reported by: sin1st3r__ Team: NPCsCorp 0xsomeone (Judge) decreased severity to Medium and commented:

The Warden has demonstrated that it is possible to bypass Decent fees when performing bridging operations by directly interacting with the DecentEthRouter. This is indeed a valid concern and has been confirmed by the Sponsor.

I believe a severity of medium is more appropriate given that uncaptured profit is solely affected.

wkantaros (Decent) confirmed Note: For full discussion, see here.

# [M-03] Missing access control on UTB:receiveFromBridge allows UTB swaps to be executed without spending bridge fees while bypassing fee/swap instruction signature verification

- **Contest:** Decent
- **Slug:** 2024-01-decent
- **Finding ID:** M-03
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-decent
- **Source snapshot:** competitions/2024-01-decent/final_report.html

Submitted by GhK3Ndf, also found by dutra, CDSecurity, Aymen0909, 0xAadi, Eeyore, DadeKuma, pkqs90, th13vn, Soliditors, bart1e, NentoR, Kow, 0xDING99YA, antonttc, haxatron, Matue, 0xdedo93, SovaSlava, peanuts, and MrPotatoMagic Users can abuse the public UTB:receiveFromBridge function’s lack of access control to directly call the internal UTB:_swapAndExecute function.

This bypasses the UTB:retrieveAndCollectFees modifier, which is used to collect fees from UTB users and validate fee and swap instructions via UTBFeeCollector:collectFees in all other public swap/bridge functions (namely UTB:swapAndExecute and UTB:bridgeAndExecute ).

This allows users to execute inter-chain swaps without spending bridge fees, using instructions that have not been signed by a validator key. Unsigned additional payloads can also be included in the swap instruction’s payload element, which would then be executed by the UTBExecutor:execute function.

Root Cause Missing access control in UTB:receiveFromBridge is likely missing an access control modifier.

Note that the missing modifier may not be the [ UTB:retrieveAndCollectFees ] modifier, as this modifier is understood to be intended for checking the UTB swap/bridge instructions on the source chain. Refer to the remedial suggestions for an alternate means of access control.

UTB:receiveFromBridge //File:src/UTB.sol contract UTB is Owned {...

function receiveFromBridge ( SwapInstructions memory postBridge, address target, address paymentOperator, bytes memory payload, address payable refund ) public { // missing retrieveAndCollect modifier _swapAndExecute ( postBridge, target, paymentOperator, payload, refund ); }...

// if a feeCollector has been set, then this modifier validates the fee struct against the signer:

modifier retrieveAndCollectFees ( FeeStructure calldata fees, bytes memory packedInfo, bytes calldata signature ) { if ( address ( feeCollector ) != address ( 0 )) { uint value = 0; if ( fees.

feeToken != address ( 0 )) { IERC20 ( fees.

feeToken ).

transferFrom ( msg.

sender, address ( this ), fees.

feeAmount ); IERC20 ( fees.

feeToken ).

approve ( address ( feeCollector ), fees.

feeAmount ); } else { value = fees.

feeAmount; } feeCollector.

collectFees {value:

value }( fees, packedInfo, signature ); } _; }...

// swapAndExecute/bridgeAndExecute functions both validate fees/instructions signatures via retrieveAndCollectFees modifier function swapAndExecute ( SwapAndExecuteInstructions calldata instructions, FeeStructure calldata fees, bytes calldata signature ) public payable retrieveAndCollectFees ( fees, abi.encode( instructions, fees ), signature ) { _swapAndExecute ( instructions.

swapInstructions, instructions.

target, instructions.

paymentOperator, instructions.

payload, instructions.

refund ); } function bridgeAndExecute ( BridgeInstructions calldata instructions, FeeStructure calldata fees, bytes calldata signature ) public payable retrieveAndCollectFees ( fees, abi.encode( instructions, fees ), signature ) returns ( bytes memory ) { ( uint256 amt2Bridge, BridgeInstructions memory updatedInstructions ) = swapAndModifyPostBridge ( instructions ); return callBridge ( amt2Bridge, fees.

bridgeFee, updatedInstructions ); }...

// _swapAndExecute directly bypasses any signature/fee checks before the swap/payload is executed.

function _swapAndExecute ( SwapInstructions memory swapInstructions, address target, address paymentOperator, bytes memory payload, address payable refund ) private { ( address tokenOut, uint256 amountOut ) = performSwap ( swapInstructions ); if ( tokenOut == address ( 0 )) { executor.

execute {value:

amountOut }( target, paymentOperator, payload, tokenOut, amountOut, refund ); } else { IERC20 ( tokenOut ).

approve ( address ( executor ), amountOut ); executor.

execute ( target, paymentOperator, payload, tokenOut, amountOut, refund ); } UTBFeeCollector:collectFees //File:src/UTBFeeCollector.sol function collectFees ( FeeStructure calldata fees, bytes memory packedInfo, bytes memory signature ) public payable onlyUtb { bytes32 constructedHash = keccak256 ( abi.

encodePacked ( BANNER, keccak256 ( packedInfo )) ); ( bytes32 r, bytes32 s, uint8 v ) = splitSignature ( signature ); address recovered = ecrecover ( constructedHash, v, r, s ); require ( recovered == signer, "Wrong signature" ); if ( fees.

feeToken != address ( 0 )) { IERC20 ( fees.

feeToken ).

transferFrom ( utb, address ( this ), fees.

feeAmount ); } UTBExecutor:execute //File:src/UTBExecutor.sol:

contract UTBExecutor is Owned {...

function execute ( address target, address paymentOperator, bytes memory payload, address token, uint amount, address payable refund ) public payable onlyOwner { return execute ( target, paymentOperator, payload, token, amount, refund, 0 ); }...

function execute ( address target, address paymentOperator, bytes memory payload, address token, uint amount, address payable refund, uint extraNative ) public onlyOwner { bool success; if ( token == address ( 0 )) { ( success, ) = target.

call {value:

amount }( payload ); if (!

success ) { ( refund.

call { value:

amount }( "" )); } return; } uint initBalance = IERC20 ( token ).

balanceOf ( address ( this )); IERC20 ( token ).

transferFrom ( msg.

sender, address ( this ), amount ); IERC20 ( token ).

approve ( paymentOperator, amount ); if ( extraNative > 0 ) { ( success, ) = target.

call {value:

extraNative }( payload ); if (!

success ) { ( refund.

call { value:

extraNative }( "" )); } else { ( success, ) = target.

call ( payload ); } uint remainingBalance = IERC20 ( token ).

balanceOf ( address ( this )) - initBalance; if ( remainingBalance == 0 ) { return; } IERC20 ( token ).

transfer ( refund, remainingBalance ); }

## Recommended Mitigation Steps

Primary Recommendation: Implement a robust access control modifier on the UTB:receiveFromBridge function to restrict access to known bridge adaptor addresses The UTB:receiveFromBridge function appears to be intended to be called by the DecentBridgeAdapter:receiveFromBridge and StargateBridgeAdapter:sgReceive functions. These functions are protected by the BaseAdapter:onlyExecutor modifier. No other calls to [ UTB:receiveFromBridge ] are made in the audit codebase.

It therefore may be suitable to introduce a similar onlyBridgeAdapter modifier to UTB, using the already present UTB:bridgeAdapters mapping to filter calls from only allowlisted bridge adaptors:

//File:src/UTB.sol...

modifier onlyBridgeAdapter (){ require ( bridgeAdapters [ IBridgeAdapter ( msg.

sender ).

getId ()] != address ( 0 ), "invalid bridge adaptor" ); _; } function receiveFromBridge ( SwapInstructions memory postBridge, address target, address paymentOperator, bytes memory payload, address payable refund ) public onlyBridgeAdapter () { _swapAndExecute ( postBridge, target, paymentOperator, payload, refund ); }...

Best practices: Review off-chain validator signature generation and update UTBFeeCollector:collectFees to allow for on-chain validation of signatures if UTB:feeCollector has not been set.

Note that in the receiveFromBridge modifier, fee and swap instructions are only validated if a feeCollector is set in UTB.

UTB:retrieveAndCollectFees //File:src/UTB.col modifier retrieveAndCollectFees ( FeeStructure calldata fees, bytes memory packedInfo, bytes calldata signature ) { if ( address ( feeCollector ) != address ( 0 )) { // if feeCollector has not been set, then signature verifcation does not occur...

feeCollector.

collectFees {value:

value }( fees, packedInfo, signature ); } _; } UTBFeeCollector:collectFees //File:src/UTBFeeCollector.sol function collectFees ( FeeStructure calldata fees, bytes memory packedInfo, bytes memory signature ) public payable onlyUtb { bytes32 constructedHash = keccak256 ( abi.

encodePacked ( BANNER, keccak256 ( packedInfo )) ); ( bytes32 r, bytes32 s, uint8 v ) = splitSignature ( signature ); // validating both fees and swap instructions with signature from signer address recovered = ecrecover ( constructedHash, v, r, s ); require ( recovered == signer, "Wrong signature" );...

} It is not known whether this is an intended design choice, possibly stemming from the way any off-chain swap/bridge/fee instruction validator APIs generate signatures and the fact that one signature is expected for both fee content and swap/bridge instructions.

However, if swaps/bridge operations are intended to ever be called without incurring a fee via UTBFeeCollector, it is recommended for off-chain APIs to generate swap/bridge operation signatures with a fee amount of 0 in the case where fees are not intended to be collected for signed swap/bridge operations.

This would allow the feecollector.collectFees line in UTB to be placed outside of the if check on the feeCollector address.

Alternatively, it is recommended to separate signature verification of fee and swap/bridge instructions in both the API and the UTBFeeCollector contract, to allow their associated signatures to be validated independently.

This would ensure that signed swap/bridge instructions can be verified before execution, in the event that a UTBFeeCollector contract is not set for a particular chain/UTB deployment.

0xsomeone (Judge) decreased severity to Medium and commented:

This and its duplicate submissions have illustrated that the UTB::receiveFromBridge will behave identically to the UTB::swapAndExecute function albeit with no fees applied or signatures validated, permitting users to bypass these measures.

The maximum impact of this and relevant submissions is the loss of fees that are meant to be imposed when the UTB::swapAndExecute functionality is utilized. As a subset of submissions has recognized, the signature validation is also bypassed permitting arbitrary payloads to be executed which is also considered unwanted with an unquantifiable impact.

Given that the UTB::swapAndExecute is one of the main features of the protocol, I consider a medium-risk severity for this exhibit to be more appropriate.

This submission has been selected as the best due to going in great detail in relation to the submission, however, it should be noted that the recommended alleviation suffers from impersonation attacks (i.e. results from getter functions on the msg.sender can be spoofed) and a mapping based whitelist mechanism should be enforced instead.

wkantaros (Decent) confirmed

# [M-04] Potential loss of capital due to fixed fee calculations

- **Contest:** Decent
- **Slug:** 2024-01-decent
- **Finding ID:** M-04
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-decent
- **Source snapshot:** competitions/2024-01-decent/final_report.html

Submitted by Soliditors, also found by Soliditors, windhustler ( 1, 2 ), gesha17, wangxx2026, NentoR, and peanuts The StargateBridgeAdapter relies on a fixed fee calculation (0.06% of the current Stargate fee), but as explained in the Stargate documentation, fees can be automatically adjusted to meet demand. ( here ) This reward can be adjusted ( StargateFeeLibraryV02.sol#L68 ) to “To incentivize users to conduct swaps that ‘refill’ native asset balances”. A problem arises because the StargateBridgeAdapter doesn’t account for this variable fee.

Then the callback function (triggered on the target chain) will receive a token amount greater than amountIn.

StargateBridgeAdapter.sol#L207 IERC20 ( swapParams.

tokenIn ).

approve ( utb, swapParams.

amountIn ); As you can see, here the difference between the received amount StargateBridgeAdapter.sol#L188 and swapParams.amountIn gets lost in the adapter.

## Recommended Mitigation Steps

It’s recommended approve the amountLD instead of the swapParams.amountIn. This way, all token received during the callback will be transfered.

function sgReceive ( uint16, // _srcChainid bytes memory, // _srcAddress uint256, // _nonce - address, // _token - uint256, // amountLD + address _token, + uint256 amountLD, bytes memory payload ) external override onlyExecutor { ( SwapInstructions memory postBridge, address target, address paymentOperator, bytes memory utbPayload, address payable refund ) = abi.

decode ( payload, ( SwapInstructions, address, address, bytes, address ) ); SwapParams memory swapParams = abi.

decode ( postBridge.

swapPayload, ( SwapParams ) ); - IERC20 ( swapParams.

tokenIn ).

approve ( utb, swapParams.

amountIn ); + IERC20 ( _token ).

approve ( utb, amountLD ); // _token == swapParams.tokenIn + swapParams.

amountIn = amountLD // swapParams also needs to be updated to swap the correct amount + postBridge.

swapPayload = abi.

encode ( swapParams ); IUTB ( utb ).

receiveFromBridge ( postBridge, target, paymentOperator, utbPayload, refund ); }

# [M-05] DecentEthRouter.sol#_bridgeWithPayload() - Any refunded ETH (native token) will be refunded to the DecentBridgeAdapter, making them stuck

- **Contest:** Decent
- **Slug:** 2024-01-decent
- **Finding ID:** M-05
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-01-decent
- **Source snapshot:** competitions/2024-01-decent/final_report.html

Submitted by deth, also found by NPCsCorp, MrPotatoMagic, Shaheen, ZdravkoHr, DadeKuma, gesha17, cu5t0mpeo, wangxx2026, zaevlad, Tendency, haxatron, bronze_pickaxe, nmirchev8, kutugu, Timepunk, and ptsanev

## Impact

The current flow of swapping and bridging tokens using the DecentBridgeAdapter looks like so:

bridgeAndExecute inside UTB is called, passing in the bridgeId of the DecentBridgeAdapter.

function bridgeAndExecute ( BridgeInstructions calldata instructions, FeeStructure calldata fees, bytes calldata signature ) public payable retrieveAndCollectFees ( fees, abi.encode( instructions, fees ), signature ) returns ( bytes memory ) { ( uint256 amt2Bridge, BridgeInstructions memory updatedInstructions ) = swapAndModifyPostBridge ( instructions ); return callBridge ( amt2Bridge, fees.

bridgeFee, updatedInstructions ); } This then makes a call to callBridge, which will call bridge on the DecentBridgeAdapter.

function callBridge ( uint256 amt2Bridge, uint bridgeFee, BridgeInstructions memory instructions ) private returns ( bytes memory ) { bool native = approveAndCheckIfNative ( instructions, amt2Bridge ); return IBridgeAdapter ( bridgeAdapters [ instructions.

bridgeId ]).

bridge { value:

bridgeFee + ( native ?

amt2Bridge:

0 ) }( amt2Bridge, instructions.

postBridge, instructions.

dstChainId, instructions.

target, instructions.

paymentOperator, instructions.

payload, instructions.

additionalArgs, instructions.

refund ); } DecentBridgeAdapter then makes a call to the bridgeWithPayload inside DecentEthRouter.

function bridge ( uint256 amt2Bridge, SwapInstructions memory postBridge, uint256 dstChainId, address target, address paymentOperator, bytes memory payload, bytes calldata additionalArgs, address payable refund ) public payable onlyUtb returns ( bytes memory bridgePayload ) { require ( destinationBridgeAdapter [ dstChainId ] != address ( 0 ), string.

concat ( "dst chain address not set " ) ); uint64 dstGas = abi.

decode ( additionalArgs, ( uint64 )); bridgePayload = abi.

encodeCall ( this.

receiveFromBridge, ( postBridge, target, paymentOperator, payload, refund ) ); SwapParams memory swapParams = abi.

decode ( postBridge.

swapPayload, ( SwapParams ) ); if (!

gasIsEth ) { IERC20 ( bridgeToken ).

transferFrom ( msg.

sender, address ( this ), amt2Bridge ); IERC20 ( bridgeToken ).

approve ( address ( router ), amt2Bridge ); } router.

bridgeWithPayload {value:

msg.

value }( lzIdLookup [ dstChainId ], destinationBridgeAdapter [ dstChainId ], swapParams.

amountIn, false, dstGas, bridgePayload ); } bridgeWithPayoad calls the internal function _bridgeWithPayload which starts the call to LZ and the bridging process itself.

function _bridgeWithPayload ( uint8 msgType, uint16 _dstChainId, address _toAddress, uint _amount, uint64 _dstGasForCall, bytes memory additionalPayload, bool deliverEth ) internal { ( bytes32 destinationBridge, bytes memory adapterParams, bytes memory payload ) = _getCallParams ( msgType, _toAddress, _dstChainId, _dstGasForCall, deliverEth, additionalPayload ); ICommonOFT.

LzCallParams memory callParams = ICommonOFT.

LzCallParams ({ refundAddress:

payable ( msg.

sender ), //@audit-issue all refunded tokens will be sent to the DecentBridgeAdapter zroPaymentAddress:

address ( 0x0 ), adapterParams:

adapterParams }); uint gasValue; if ( gasCurrencyIsEth ) { weth.

deposit {value:

_amount }(); gasValue = msg.

value - _amount; } else { weth.

transferFrom ( msg.

sender, address ( this ), _amount ); gasValue = msg.

value; } dcntEth.

sendAndCall {value:

gasValue }( address ( this ), // from address that has dcntEth (so DecentRouter) _dstChainId, destinationBridge, // toAddress _amount, // amount payload, //payload (will have recipients address) _dstGasForCall, // dstGasForCall callParams // refundAddress, zroPaymentAddress, adapterParams ); } When we are using LZ, we have to specify LzCallParams. The struct holds several things, but importantly it holds the refundAddress ICommonOFT.

LzCallParams memory callParams = ICommonOFT.

LzCallParams ({ refundAddress:

payable ( msg.

sender ), zroPaymentAddress:

address ( 0x0 ), adapterParams:

adapterParams }); You’ll notice that the refundAddress is specified as msg.sender, in this case msg.sender is the DecentBridgeAdapter since that’s the address that made the call to DecentEthRouter.

The refundAddress is used for refunding any excess native tokens (in our case) that are sent to LZ in order to pay for the gas. The excess will be refunded on the source chain.

Basically if you send 0.5 ETH to LZ for gas and LZ only needs 0.1ETH, then 0.4ETH will be sent to the refundAddress.

The problem here is, that the DecentBridgeAdapter has no way of retrieving the funds, as it doesn’t implement any withdraw functionality whatsoever.

The protocol team even stated in the README.

Fund Accumulation: Other than the UTBFeeCollector, and DcntEth, the contracts are not intended to hold on to any funds or unnecessary approvals. Any native value or erc20 flowing through the protocol should either get delivered or refunded.

This bug clearly violates what the protocol team expects.

## Recommended Mitigation Steps

The user specifies a refund when calling bridgeAndExecute inside UTB. Use the address that the user specifies instead of msg.sender.

wkantaros (Decent) confirmed 0xsomeone (Judge) decreased severity to Medium and commented:

The Warden has clearly demonstrated that the refund configuration of the LayerZero relayed call payload is incorrect, causing native fund gas refunds to be sent to the wrong address. I appreciate that the Warden has referenced all code snippets necessary for the elaborate cross-chain call.

In reality, the flaw will result in relatively small amounts of the native asset to be lost. As a result, I believe a medium-risk category is better suited for this vulnerability.

ihtishamsudo (Warden) commented:

Thank you, Alex, for judging. I strongly believe that this is a high-severity issue. Although the individual loss per user may be minimal at present, it has the potential to accumulate over time, becoming a persistent problem and frozen funds forever. The existing refund mechanisms contribute to a lack of concern among users when sending gas. Consequently, when substantial gas amounts are sent, significant funds are at risk due to this vulnerability. I kindly request you to revisit and reconsider assigning a high severity rating to this issue. Appreciate your attention to this matter. Thank you again!

0xsomeone (Judge) commented:

Hey @ihtisham-sudo, thank you for contributing to this discussion! There has been a long-standing discussion about capping gas-impacting findings at a QA (L) level among the C4 judge community but I have made an exception for this finding.

In this particular case, I consider it a medium-risk issue as it is likely those transactions would have a substantial over-allocation of gas due to their cross-chain nature. In any other circumstance, this would be considered a QA issue.
