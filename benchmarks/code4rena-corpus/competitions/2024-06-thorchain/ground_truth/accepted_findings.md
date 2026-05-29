# Accepted H/M Findings: Thorchain

# [H-01] A malicious user can steal money out of the vault and other users

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Finding ID:** H-01
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-06-thorchain
- **Source snapshot:** competitions/2024-06-thorchain/final_report.html

Submitted by samuraii77, also found by Bauchibred, ilchovski, rbserver ( 1, 2 ), mt030d, and cheatc0d3 A malicious user can steal money out of other users and the vault when a rebasing token like AMPL is used. That particular token is whitelisted as seen here.

## Recommended Mitigation Steps

I don’t think the fix is trivial, I will give some ideas but they have to be carefully examined. First, one of the root causes for the issue is the approval given to the router that is not actually equal to the amount the user is supposed to receive as this is a rebasing token. If that is taken care of and the correct approval is given, I think this vulnerability shouldn’t be possible anymore.

Trust (judge) decreased severity to Medium samuraii77 (warden) commented:

I believe this issue is wrongly duplicated. The root cause of the issue it is duplicated to is improper setup for fee-on-transfer tokens and the impact is a DoS. The root cause of this issue is an improper setup for rebasing tokens and the impact is a direct theft of funds. This issue should be its own separate issue and should be a high.

Trust (judge) commented:

Since the contracts don’t have code that handles both FoT/rebasing tokens, it is fair to combine the two issues together. If there was an attempt to handle one of the types, but it is bugged, that would indeed merit two different issues.

samuraii77 (warden) commented:

FoT tokens and rebasing tokens are not the same thing; they have different behaviors and different things that could go wrong when implementing them.

Root Cause:

FoT case:

_transferOutAndCallV5() assumes that the amount parameter is equal to the amount received.

Rebasing case: The contract assumes that the amount that was received time ago through a deposit will stay the same across that time which would not be the case.

Impact:

FoT case: User will be DoSed and funds will be locked.

Rebasing case: Direct theft of funds.

Fix:

FoT case: Use the actual amount received instead of the amount parameter.

Rebasing case: Do not assume that the balances will stay constant and implement a mechanism to handle that.

None of these match up and they should not be duplicated. Furthermore, as I mentioned, FoT tokens and rebasing tokens are not the same thing, it is like duplicating an issue regarding FoT tokens with an issue that used transferFrom on tokens that don’t revert on failure.

Trust (judge) commented:

We don’t duplicate by impact or fixes, that’s irrelevant. The root cause is the same - devs did not have the presence of mind to deal with tokens with non standard balance mechanisms.

samuraii77 (warden) commented:

Respectfully, what makes this issue a Medium? Why is it not a high when this issue clearly explains an issue with a high severity? Here is a rule regarding that:

Given the above, when similar exploits would demonstrate different impacts, the highest, most irreversible would be the one used for scoring the finding. Duplicates of the finding will be graded based on the achieved impact relative to the Submission Chosen for Report.

More specifically, if fixing the Root Cause (in a reasonable manner) would cause the finding to no longer be exploitable, then the findings are duplicates.

Fixing one of the issues does not fix the other one, so your argument that you do not duplicate by fixes is not entirely correct.

Also, I don’t agree that the root cause is the same even ignoring the above rule. You are pulling the root cause up until it is similar but the actual root causes are different, incorrect handling of FoT tokens and incorrect handling of rebasing tokens. The way you generalize the root cause of the issue is similar to saying that all issues are duplicate because the developers wrote code that is wrong.

Furthermore, they did have the presence of mind to deal with non-standard balance mechanisms. Take a look at this code in deposit():

safeAmount = safeTransferFrom ( asset, amount ); Which then calls:

// Safe transferFrom in case asset charges transfer fees function safeTransferFrom ( address _asset, uint _amount ) internal returns ( uint amount ) { uint _startBal = iERC20 ( _asset ).

balanceOf ( address ( this )); ( bool success, bytes memory data ) = _asset.

call ( abi.

encodeWithSignature ( "transferFrom(address,address,uint256)", msg.

sender, address ( this ), _amount ) ); require ( success && ( data.

length == 0 || abi.

decode ( data, ( bool )))); return ( iERC20 ( _asset ).

balanceOf ( address ( this )) - _startBal ); } Trust (judge) increased severity to High and commented:

I assumed there is no differentiation at the code level between these. As it is clear they aimed to support FoT tokens, there are now clearly two separate root causes. Upgrading to High since it is clear these tokens are whitelisted and intended to be used.

the-eridanus (Thorchain) confirmed

# [H-02] ThorChain will be informed wrongly about the unsuccessful ETH transfers due to the incorrect events emissions

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Finding ID:** H-02
- **Severity:** High
- **Source URL:** https://code4rena.com/reports/2024-06-thorchain
- **Source snapshot:** competitions/2024-06-thorchain/final_report.html

Submitted by Shaheen, also found by mt030d, _karanel ( 1, 2 ), bigtone, ilchovski, Fortis_audits, Team_RockSolid, Svetoslavb, Heba-Elhasan, Greed, 0xAadi, and shaflow2

- https://github.com/code-423n4/2024-06-thorchain/blob/e3fd3c75ff994dce50d6eb66eb290d467bd494f5/ethereum/contracts/THORChain_Router.sol#L196
- https://github.com/code-423n4/2024-06-thorchain/blob/e3fd3c75ff994dce50d6eb66eb290d467bd494f5/ethereum/contracts/THORChain_Router.sol#L206

## Vulnerability details

One of the main invariant of the protocol is:

Only valid events emitted from the Router contract itself should result in the txInItem parameter being populated in the GetTxInItem function of the smartcontract_log_parser.

In short, this means that all the events ThorChain_Router emits, should be correct.

This invariants breaks in the edge cases of the transferOut(), _transferOutV5(), transferOutAndCall() and _transferOutAndCallV5() For the sake of simplicity, we will only gonna take a look at the transferOut() function.

transferOut() function is used by the vaults to transfer Native Tokens (ethers) or ERC20 Tokens to any address to. It first transfers the funds to the specified to address and then emits the TransferOut event for ThorChain. In case the Native Tokens transfer to the to address fails, it just refunds or bounce back the ethers to the vault address ( msg.sender ). Transfer to to address can fail often, as the function uses solidity’s.send to transfer the funds. If the to address is a contract which takes more than 2300 gas to complete the execution, then.send will return false and the ethers will be bounced back to the vault address.

The problem is, in the case when the.send will fail and the ethers will bounce back to the vault address, the event TransferOut will be wrong. As we can see, when the ethers receiver will be in vault, not the input to address, the to doesn’t get updated to the vault’s address and the function in the end emits the same to, ThorChain is getting informed that the ether receiver is still input to:

function transferOut ( address payable to, address asset, uint amount, string memory memo ) public payable nonReentrant { uint safeAmount; if ( asset == address ( 0 )) { safeAmount = msg.

value; bool success = to.

send ( safeAmount ); // Send ETH.

if (!

success ) { payable ( address ( msg.

sender )).

transfer ( safeAmount ); // For failure, bounce back to vault & continue.

} else {.....

} ///@audit-issue H worng event `to` incase of the bounce back - PoC: `Should bounce back ethers but emits wrong event` emit TransferOut ( msg.

sender, to, asset, safeAmount, memo ); } Technically, the ETH transfer is unsuccessful, but the ThorChain is informed that its successful and the funds are successfully transferred to the specified to address. Also, the smartcontract_log_parser ’s GetTxInItem() function doesn’t ignore these trxs at all, as it doesn’t check if txInItem.To is equal to the calling vault or not.

## Impact

The network believes the outbound was successful and updates the vaults accordingly, but the outbound was not successful; resulting in loss of funds for the users.

## Recommended Mitigation Steps

There are multiple solutions to this issue:

Only emit event when transfer to the target is successful (highly recommended):

function transferOut ( address payable to, address asset, uint amount, string memory memo ) public payable nonReentrant { uint safeAmount; if ( asset == address ( 0 )) { safeAmount = msg.

value; bool success = to.

send ( safeAmount ); // Send ETH.

emit TransferOut ( msg.

sender, to, asset, safeAmount, memo ); if (!

success ) { payable ( address ( msg.

sender )).

transfer ( safeAmount ); // For failure, bounce back to vault & continue.

} else { _vaultAllowance [ msg.

sender ][ asset ] -= amount; // Reduce allowance ( bool success, bytes memory data ) = asset.

call ( abi.

encodeWithSignature ( "transfer(address,uint256)", to, amount ) ); require ( success && ( data.

length == 0 || abi.

decode ( data, ( bool )))); safeAmount = amount; emit TransferOut ( msg.

sender, to, asset, safeAmount, memo ); } Simply Revert the trx upon.send failure.

Set to address to the vault when bounce back happens.

Ignore these trxs in the smartcontract_log_parser ’s GetTxInItem().

Use.call which will potentially lower the chance of failure while transferring the ethers (least recommended).

## Assessed type

Context the-eridanus (Thorchain) confirmed and commented:

Seems like a good fix to make.

Medium Risk Findings (2)

# [M-01] Incorrect call argument in THORChain_Router::_transferOutAndCallV5 , leading to grief/steal of THORChain_Aggregator ’s funds or DoS

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Finding ID:** M-01
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-06-thorchain
- **Source snapshot:** competitions/2024-06-thorchain/final_report.html

THORChain_Router::_transferOutAndCallV5, leading to grief/steal of THORChain_Aggregator ’s funds or DoS Submitted by Svetoslavb, also found by dvrkzy, iam_emptyset, samuraii77, nfmelendez, Team_RockSolid, dhank, PetarTolev, Shaheen, Greed, Gosho, and rbserver When transferring a token, which is of type fee-on-transfer, in THORChain_Router::_transferOutAndCallV5, the token is first deposited to the THORChain_Aggregator and then THORChain_Aggregator::swapOutV5 is called with the same amount. The call will always revert if the THORChain_Aggregator does not have tokens or grief/steal (depending on the token) of THORChain_Aggregator ’s tokens. An example of a fee-on-transfer token that is in the whitelist is

PAXG ( see here ) to view the loss of funds that are locked and waiting to be rescued in the THORChain_Aggregator.

## Recommended Mitigation Steps

Consider creating a safeTransfer function, similar to the safeTransferFrom.

Add this below THORChain_Router::safeTransferFrom:

+ // Safe transfer in case asset charges transfer fees + function safeTransfer(address _asset, address _to, uint256 _amount) internal returns (uint256 amount) { + uint256 _startBal = iERC20(_asset).balanceOf(_to); + (bool success, bytes memory data) = + _asset.call(abi.encodeWithSignature("transfer(address,address,uint256)", msg.sender, _to, + _amount)); + require(success && (data.length == 0 || abi.decode(data, (bool))), "Failed to transfer token"); + return (iERC20(_asset).balanceOf(_to) - _startBal); + } In THORChain_Router::_transferOutAndCallV5:

- (bool transferSuccess, bytes memory data) = aggregationPayload.fromAsset.call( - abi.encodeWithSignature( - "transfer(address,uint256)", aggregationPayload.target, aggregationPayload.fromAmount - ) - ); - - require( - transferSuccess && (data.length == 0 || abi.decode(data, (bool))), - "Failed to transfer token before dex agg call" - ); + uint256 safeAmount = + safeTransfer(aggregationPayload.fromAsset, aggregationPayload.target, aggregationPayload.fromAmount);

## Assessed type

Token-Transfer the-eridanus (Thorchain) confirmed via duplicate Issue #15

# [M-02] Due to the use of msg.value in for loop, anyone can drain all the funds from the THORChain_Router contract

- **Contest:** Thorchain
- **Slug:** 2024-06-thorchain
- **Finding ID:** M-02
- **Severity:** Medium
- **Source URL:** https://code4rena.com/reports/2024-06-thorchain
- **Source snapshot:** competitions/2024-06-thorchain/final_report.html

msg.value in for loop, anyone can drain all the funds from the THORChain_Router contract Submitted by PetarTolev, also found by mt030d, samuraii77, bigtone, 0xfox, TECHFUND-inc, 0xAadi, benoitonchain, Team_RockSolid, ilchovski, Svetoslavb, Timenov, EPSec, hunter_w3b, Shaheen, inh3l, Limbooo, LuarSec, shaflow2, Gosho, and 0x1771

- https://github.com/code-423n4/2024-06-thorchain/blob/e3fd3c75ff994dce50d6eb66eb290d467bd494f5/chain/ethereum/contracts/THORChain_Router.sol#L309-L311
- https://github.com/code-423n4/2024-06-thorchain/blob/e3fd3c75ff994dce50d6eb66eb290d467bd494f5/chain/ethereum/contracts/THORChain_Router.sol#L324

## Vulnerability details

The functions transferOutAndCallV5() and batchTransferOutAndCallV5() both internally invoke _transferOutAndCallV5(). This function, in turn, calls the swapOutV5() function on the aggregationPayload.target, which should be the THORChain_Aggregator contract.

However, the aggregationPayload is a struct passed as a parameter through either transferOutAndCallV5() or batchTransferOutAndCallV5(). This allows anyone to set the aggregationPayload.target address to their preference without any validation.

When a low-level call to aggregationPayload.target is made, the return value swapOutSuccess is checked. If it’s false, a fallback logic attempts to send the msg.value to the target. If this also fails, the msg.value is refunded to the msg.sender.

The issue arises when batchTransferOutAndCallV5() is called. It loops through the aggregationPayloads array and passes each element to _transferOutAndCallV5(), which then sends the msg.value multiple times to either the target address or msg.sender.

function batchTransferOutAndCallV5 ( TransferOutAndCallData [] calldata aggregationPayloads ) external payable nonReentrant { for ( uint i = 0; i < aggregationPayloads.

length; ++ i ) { @> _transferOutAndCallV5 ( aggregationPayloads [ i ]); } function _transferOutAndCallV5 ( TransferOutAndCallData calldata aggregationPayload ) private { if ( aggregationPayload.

fromAsset == address ( 0 )) { // call swapOutV5 with ether @> ( bool swapOutSuccess, ) = aggregationPayload.

target.

call { value:

msg.

value }( abi.

encodeWithSignature ( "swapOutV5(address,uint256,address,address,uint256,bytes,string)", aggregationPayload.

fromAsset, aggregationPayload.

fromAmount, aggregationPayload.

toAsset, aggregationPayload.

recipient, aggregationPayload.

amountOutMin, aggregationPayload.

payload, aggregationPayload.

originAddress ) ); @> if (!

swapOutSuccess ) { @> bool sendSuccess = payable ( aggregationPayload.

target ).

send ( msg.

value ); // If can't swap, just send the recipient the gas asset if (!

sendSuccess ) { @> payable ( address ( msg.

sender )).

transfer ( msg.

value ); // For failure, bounce back to vault & continue.

} emit TransferOutAndCallV5 ( msg.

sender, aggregationPayload.

target, msg.

value, aggregationPayload.

toAsset, aggregationPayload.

recipient, aggregationPayload.

amountOutMin, aggregationPayload.

memo, aggregationPayload.

payload, aggregationPayload.

originAddress ); } else {...

}

## Impact

There are two issues associated with the use of msg.value in _transferOutAndCallV5:

The first one, described in the Proof of Concept section below, is of high severity. A malicious user could potentially drain all funds from the THORChain_Router.

The second issue is of medium severity. When a trusted actor invokes the batchTransferOutAndCallV5 function and if the length of the aggregationPayloads array exceeds 1, it will constantly revert. This happens because the entire msg.value is sent in the first iteration, causing the second iteration to revert with OutOfFunds when the Router’s balance is reduced to zero.

## Recommended Mitigation Steps

It is recommended to avoid the use of msg.value in for loops. To mitigate the current issue, the cumulative value sent to the aggregationPayload.target should not exceed the msg.value. This can be achieved by adding a parameter etherAmount in the TransferOutAndCallData struct, which will be used instead of msg.value in the _transferOutAndCallV5. Then, add a require statement in the batchTransferOutAndCallV5 which checks if msg.value == cumulativeValueSent.

struct TransferOutAndCallData { address payable target; address fromAsset; uint256 fromAmount; address toAsset; address recipient; uint256 amountOutMin; string memo; bytes payload; string originAddress; + uint256 etherAmount; } function batchTransferOutAndCallV5( TransferOutAndCallData[] calldata aggregationPayloads ) external payable nonReentrant { + uint cumulativeValueSent; for (uint i = 0; i < aggregationPayloads.length; ++i) { + cumulativeValueSent += aggregationPayloads[i].etherAmount; _transferOutAndCallV5(aggregationPayloads[i]); } + require(msg.value == cumulativeValueSent); } function _transferOutAndCallV5( TransferOutAndCallData calldata aggregationPayload ) private { if (aggregationPayload.fromAsset == address(0)) {

// call swapOutV5 with ether (bool swapOutSuccess, ) = aggregationPayload.target.call{ - value: msg.value + value: aggregationPayload.etherAmount }( abi.encodeWithSignature( "swapOutV5(address,uint256,address,address,uint256,bytes,string)", aggregationPayload.fromAsset, aggregationPayload.fromAmount, aggregationPayload.toAsset, aggregationPayload.recipient, aggregationPayload.amountOutMin, aggregationPayload.payload, aggregationPayload.originAddress ) ); if (!swapOutSuccess) { - bool sendSuccess = payable(aggregationPayload.target).send(msg.value); // If can't swap, just send the recipient the gas asset + bool sendSuccess = payable(aggregationPayload.target).send(aggregationPayload.etherAmount); // If can't swap, just send the recipient the gas asset

if (!sendSuccess) { - payable(address(msg.sender)).transfer(msg.value); // For failure, bounce back to vault & continue.

+ payable(address(msg.sender)).transfer(aggregationPayload.etherAmount); // For failure, bounce back to vault & continue.

} emit TransferOutAndCallV5( msg.sender, aggregationPayload.target, - msg.value, + aggregationPayload.etherAmount, aggregationPayload.toAsset, aggregationPayload.recipient, aggregationPayload.amountOutMin, aggregationPayload.memo, aggregationPayload.payload, aggregationPayload.originAddress ); } else {...

}

## Assessed type

ETH-Transfer the-eridanus (Thorchain) confirmed via duplicate Issue #7 Trust (judge) decreased severity to Medium
